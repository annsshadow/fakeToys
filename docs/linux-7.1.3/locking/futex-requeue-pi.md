## Futex Requeue PI


将任务从非 PI futex 重新排队（requeue）到 PI futex 需要特殊处理，以确保底层的
rt_mutex 在有等待者时永远不会没有拥有者；否则会破坏 PI 提升逻辑 [see rt-mutex-design.rst]。
为简洁起见，本文档中将该操作统一称为 "requeue_pi"。优先级继承在全文缩写为 "PI"。

### Motivation


如果没有 requeue_pi，pthread_cond_broadcast() 的 glibc 实现就必须唤醒所有等待在某个
pthread_condvar 上的任务，再让它们自行争抢谁先运行，形成经典的惊群（thundering-herd）
局面。理想的实现应当唤醒优先级最高的等待者，而其余的则交由与 condvar 相关联的互斥体
解锁时固有的自然唤醒来处理。

```

	/* caller must lock mutex */
	pthread_cond_wait(cond, mutex)
	{
		lock(cond->__data.__lock);
		unlock(mutex);
		do {
		unlock(cond->__data.__lock);
		futex_wait(cond->__data.__futex);
		lock(cond->__data.__lock);
		} while(...)
		unlock(cond->__data.__lock);
		lock(mutex);
	}

	pthread_cond_broadcast(cond)
	{
		lock(cond->__data.__lock);
		unlock(cond->__data.__lock);
		futex_requeue(cond->data.__futex, cond->mutex);
	}

```
一旦 pthread_cond_broadcast() 重新排队了这些任务，cond->mutex 就有了等待者。注意
pthread_cond_wait() 只有在返回到用户空间之后才会尝试锁定该互斥体。这将使底层的 rt_mutex
处于有等待者却没有拥有者的状态，从而破坏了前面提到的 PI 提升算法。

为了支持感知 PI 的 pthread_condvar，内核需要能够把任务重新排队到 PI futex。这种支持
意味着，在一次成功的 futex_wait 系统调用之后，调用者返回用户空间时已经持有了该 PI futex。
glibc 的实现
```


	/* caller must lock mutex */
	pthread_cond_wait_pi(cond, mutex)
	{
		lock(cond->__data.__lock);
		unlock(mutex);
		do {
		unlock(cond->__data.__lock);
		futex_wait_requeue_pi(cond->__data.__futex);
		lock(cond->__data.__lock);
		} while(...)
		unlock(cond->__data.__lock);
		/* the kernel acquired the mutex for us */
	}

	pthread_cond_broadcast_pi(cond)
	{
		lock(cond->__data.__lock);
		unlock(cond->__data.__lock);
		futex_requeue_pi(cond->data.__futex, cond->mutex);
	}

```
实际的 glibc 实现很可能会对 PI 进行测试，并在现有调用内部做必要的修改，而不是为 PI 场景
新建调用。pthread_cond_timedwait() 和 pthread_cond_signal() 也需要类似的修改。

### Implementation


为了确保 rt_mutex 在有等待者时拥有拥有者，重新排队代码以及等待代码都必须能够在返回用户
空间之前获取该 rt_mutex。重新排队代码不能简单地唤醒等待者，然后任由其去获取 rt_mutex，
因为那样会在重新排队调用返回用户空间与等待者被唤醒并开始运行之间打开一个竞态窗口。在
无竞争的情况下尤其如此。

解决方案引入了两个新的 rt_mutex 辅助例程，rt_mutex_start_proxy_lock() 和
rt_mutex_finish_proxy_lock()，它们允许重新排队代码代表等待者获取一个无竞争的 rt_mutex，
并把等待者排入一个有竞争的 rt_mutex 的等待队列。两个新的系统调用提供了内核与用户空间之间
用于 requeue_pi 的接口：FUTEX_WAIT_REQUEUE_PI 和 FUTEX_CMP_REQUEUE_PI。

FUTEX_WAIT_REQUEUE_PI 由等待者（pthread_cond_wait() 和 pthread_cond_timedwait()）调用，
用于阻塞在初始 futex 上并等待被重新排队到一个感知 PI 的 futex。其实现是 futex_wait() 与
futex_lock_pi() 高速碰撞的结果，并加入了一些额外的逻辑来处理那些额外的唤醒场景。

FUTEX_CMP_REQUEUE_PI 由唤醒者（pthread_cond_broadcast() 和 pthread_cond_signal()）调用，
用于重新排队并可能唤醒等待的任务。在内部，该系统调用仍然由 futex_requeue 处理（通过传入
requeue_pi=1）。在重新排队之前，futex_requeue() 会代表最顶端的等待者尝试获取重新排队目标的
PI futex。如果成功，该等待者被唤醒。随后 futex_requeue() 继续把剩下的 nr_wake+nr_requeue
个任务重新排队到 PI futex，在每次重新排队前调用 rt_mutex_start_proxy_lock() 以将该任务准备
为底层 rt_mutex 上的一个等待者。在这一阶段也有可能获取到锁，如果是这样，下一个等待者会被
唤醒以完成锁的获取。

FUTEX_CMP_REQUEUE_PI 接受 nr_wake 和 nr_requeue 作为参数，但真正重要的只是它们的和。
futex_requeue() 会唤醒或重新排队最多 nr_wake + nr_requeue 个任务。它只会唤醒它能够为其获取锁
的那么多任务，而在大多数情况下，这个数字应当是 0，因为良好的编程实践要求 pthread_cond_broadcast()
或 pthread_cond_signal() 的调用者在发起调用之前先获取互斥体。FUTEX_CMP_REQUEUE_PI 要求 nr_wake=1。
对于 broadcast，nr_requeue 应为 INT_MAX；对于 signal，应为 0。
