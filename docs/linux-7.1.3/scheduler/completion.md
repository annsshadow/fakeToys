## Completions - “等待完成”屏API

### 简介：

如果你有一个或多个线程必须等待某些内核活动到达某个点或特定状态，completions（完成量）可以为这一问题提供一个无竞态（race-free）的解决方案。从语义上讲，它们有些类`pthread_barrier()`，并且具有相似的用例
Completions 是一种代码同步机制，比任何对信号量的误用以及忙循环都要可取。任何时候当你想使用 `yield()` 或某种古怪的 `msleep(1)` 循环来让其他事情继续推进时，你可能应当考虑改用其中一`wait_for_completion*()` 调用`complete()`
使用 completions 的好处在于它们具有定义良好、目标专一的用途，这使得代码的意图非常容易看清，而且它们还能产生更高效的代码，因为所有线程都可以一直执行下去，直到真正需要结果时为止，并且等待与信号通知都通过底层的调度器睡眠/唤醒设施实现了极高的效率
Completions 构建Linux 调度器的等待队列（waitqueue）和唤醒基础设施之上。等待队列上线程们所等待的事件被化简`struct completion` 中的一个简单标志，恰当地称之为“done”
由于 completions 与调度相关，相关代码可以kernel/sched/completion.c 中找到
### 用法
使用 completions 有三个主要部分：

 - `struct completion` 同步对象的初始化
 - 通过调用 `wait_for_completion()` 的某个变体进行的等待部分
 - 通过调用 `complete()` `complete_all()` 进行的信号通知部分

另外还有一些辅助函数用于检completions 的状态。注意，虽然初始化必须最先发生，但等待和信号通知部分可以以任意顺序发生。即，在另一个线程检查它是否需要等待之前，某个线程就已经将一completion 标记为“done”是完全正常的
要使completions，你需`#include <linux/completion.h>` 并创建一`struct completion` 类型的静态或动态变量，
```

	struct completion {
		unsigned int done;
		struct swait_queue_head wait;
	};

```
这为等待（如有）的任务提供了 ->wait 等待队列，以及用于指示是否完成的 ->done 完成标志
Completions 的命名应当指向正在被同步的事件```

	wait_for_completion(&early_console_added);

	complete(&early_console_added);

```
好的、直观的命名（一如既往）有助于代码可读性。将一completion 命名'complete' 是没有帮助的，除非其用途极其明显…
### 初始completions
动态分配的 completion 对象最好内嵌在能够保证在其函数/驱动生命周期内存活的数据结构中，以防止与异步`complete()` 调用发生竞态
在使`wait_for_completion()` `_timeout()` `_killable()`/`_interruptible()` 变体时应当特别小心，因为必须保证内存释放不会发生在所有相关活动（`complete()` `reinit_completion()`）完成之前，即使这些等待函数由于超时或信号触发而提前返回
动态分配的 completion 对象的初始化通过调用以下函数完成
```

	init_completion(&dynamic_object->done);

```
在此调用中，我们初始化等待队列并->done 设置0，即“未完成”或“未完成”
重新初始化函`reinit_completion()` 只是->done 字段重置0（“未完成”），而不触碰等待队列。此函数的调用者必须确保没有并发的 `wait_for_completion()` 调用在并行进行
对同一completion 对象调用 `init_completion()` 两次极有可能是一bug，因为它将队列重新初始化为空队列，而已入队的任务可能会“丢失”——在这种情况下应使用 `reinit_completion()`，但也要注意其他竞态
对于静态声明和初始化，提供了宏
对于文件作用域中的静态（或全局）声明，你可以使```

	static DECLARE_COMPLETION(setup_done);
	DECLARE_COMPLETION(setup_done);

```
注意，在这种情况下，completion 在启动时间（或模块加载时间）被初始化为“未完成”，不需`init_completion()` 调用
当一completion 被声明为函数内的局部变量时，那么初始化应当始终显式使用 `DECLARE_COMPLETION_ONSTACK()`，这不仅是为了让 lockdep 满意，也是为了明```

	DECLARE_COMPLETION_ONSTACK(setup_done)

```
注意，当completion 对象用作局部变量时，你必须敏锐地意识到函数栈的短暂生命周期：在所有活动（例如等待线程）停止且 completion 对象完全不再被使用之前，函数不得返回到调用上下文
再次强调这一点：特别是当使用一些具有更复杂结果的等API 变体时，例如超时或信号通知（`_timeout()`、`_killable()` `_interruptible()`）变体，等待可能会提前完成，而该对象可能仍被另一个线程使用——如果某个其他线程中执行`complete()`，那`wait_on_completion*()` 调用方函数的返回将释放函数栈并导致难以察觉的数据损坏。简单的测试可能无法触发这类竞态
如果不确定，请使用动态分配的 completion 对象，最好内嵌在某个其他长生命周期的对象中，其生命周期之长超过任何使completion 对象的辅助线程，或者具有锁或其他同步机制以确保不会对已释放的对象调`complete()`
在栈上朴素的 `DECLARE_COMPLETION()` 会触lockdep 警告
### 等待 completions
对于一个线程要等待某些并发活动完成，它
```

	void wait_for_completion(struct completion *done)

```
```

	CPU#1					CPU#2

	struct completion setup_done;

	init_completion(&setup_done);
	initialize_work(...,&setup_done,...);

	/* run non-dependent code */		/* do setup */

	wait_for_completion(&setup_done);	complete(&setup_done);

```
这并不暗`wait_for_completion()` 与对 `complete()` 的调用之间存在任何特定顺序——如果对 `complete()` 的调用发生在`wait_for_completion()` 的调用之前，那么等待方将立即继续，因为所有依赖都已满足；否则它将阻塞，直到被 `complete()` 信号通知完成
注意 `wait_for_completion()` 调用`spin_lock_irq()`/`spin_unlock_irq()`，因此只有在你确定中断已使能时才能安全调用。在中断关闭（IRQs-off）的原子上下文中调用它会导致难以检测的伪中断使能
默认行为是不带超时地等待，并将任务标记为不可中断（uninterruptible）。`wait_for_completion()` 及其变体仅在进程上下文（因为它们会睡眠）中安全，而在原子上下文、中断上下文、中断被禁用或抢占被禁用时则不安全——关于在原子/中断上下文中处理 completion，另请参阅下面的 `try_wait_for_completion()`
由于 `wait_for_completion()` 的所有变体都可能（显然）根据所等待活动的性质阻塞很长时间，因此在大多数情况下你可能不希望持有着互斥体（mutex）时调用它
### 可用wait_for_completion*() 变体
以下变体都返回状态，并且在大多数所有）情况下都应检查该状态——在故意不检查状态的情况下，你可能想要写一条注释来解释原因（例如参arch/arm/kernel/smp.c:__cpu_up()）
一个常见的出现的问题是返回类型赋值不干净，因此要小心将返回值赋给适当类型的变量
检查返回值的特定含义也曾被发```

	if (!wait_for_completion_interruptible_timeout(...))

```
……会对成功完成和以下情况执行相同的代码路```

	int wait_for_completion_interruptible(struct completion *done)

```
此函数在等待时将任务标记TASK_INTERRUPTIBLE```

	unsigned long wait_for_completion_timeout(struct completion *done, unsigned long timeout)

```
任务被标记为 TASK_UNINTERRUPTIBLE，并且最多等待“timeout”个 jiffies。如果发生超时，它返0，否则返回剩余的 jiffies（但至少1）
超时最好使`msecs_to_jiffies()` `usecs_to_jiffies()` 计算，以使代码在很大程度上与 HZ 无关
如果故意忽略返回的超时值，大概应当写一条注释来解释
```

	long wait_for_completion_interruptible_timeout(struct completion *done, unsigned long timeout)

```
此函数传入以 jiffies 为单位的超时，并将任务标记为 TASK_INTERRUPTIBLE。如果收到信号，它将返回 -ERESTARTSYS；否则，如果 completion 超时则返0，如果发completion 则返回剩余的 jiffies
进一步的变体包括 `_killable`，它使用 TASK_KILLABLE 作为指定的任务状态，如果被中断将返回 -ERESTARTSYS```

	long wait_for_completion_killable(struct completion *done)
	long wait_for_completion_killable_timeout(struct completion *done, unsigned long timeout)

```
`_io` 变体 `wait_for_completion_io()` 的行为与`_io` 变体相同，只是将等待时间计入“等IO”，这有
```

	void wait_for_completion_io(struct completion *done)
	unsigned long wait_for_completion_io_timeout(struct completion *done, unsigned long timeout)


```
### 信号通知 completions
想要发出“继续条件已达成”信号的一个线程调`complete()` 来精确地通知其中一个等待者它可以
```

	void complete(struct completion *done)

```
```

	void complete_all(struct completion *done)

```
即使 completions 在某一线程开始等待之前就被信号通知，信号通知也会如预期般工作。这是通过等待者“消费”（递减）`struct completion` done 字段实现的。等待线程的唤醒顺序与它们入队顺序相同（FIFO 顺序）
如果多次调用 `complete()`，那么这将允许相应数量的等待者继续——每次对 `complete()` 的调用只是将 done 字段加一。不过多次调`complete_all()` 是一bug。无论是 `complete()` 还是 `complete_all()` 都可以在 IRQ/原子上下文中安全调用
任何时刻只能有一个线程调`complete()` `complete_all()` 作用于特定的 `struct completion`——通过等待队列自旋锁串行化。任何此类对 `complete()` `complete_all()` 的并发调用都可能是设计上bug
IRQ 上下文信号通知 completion 是没问题的，因为它会适当地使`spin_lock_irqsave()`/`spin_unlock_irqrestore()` 加锁，并且永远不会睡眠
### try_wait_for_completion()/completion_done()锛?
`try_wait_for_completion()` 函数不会将线程放到等待队列上，而是如果需要将线程入队（阻塞）则返false```

	bool try_wait_for_completion(struct completion *done)

```
最后，要在不改completion 任何状态的情况下检查其状态，调用 `completion_done()`，如果没有被等待者消费的已提completion（意味着
```

	bool completion_done(struct completion *done)

```
`try_wait_for_completion()` `completion_done()` 都可以在 IRQ 或原子上下文中安全调用