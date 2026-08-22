
## RCU lockdep 检


所有类型的 RCU 都具有可用的 lockdep 检查，因此 lockdep 知道每个任务何时进入和离开任意
类型RCU 读侧临界区。每种类型的 RCU 被单独跟踪（但请注意2.6.32 及更早版本中并非
如此）。这使得 lockdep 的跟踪可以包RCU 状态，在调试死锁等情况时有时会有所帮助

此外，RCU 提供以下检lockdep 的原语：
```

	rcu_read_lock_held() for normal RCU.
	rcu_read_lock_bh_held() for RCU-bh.
	rcu_read_lock_sched_held() for RCU-sched.
	rcu_read_lock_any_held() for any of normal RCU, RCU-bh, and RCU-sched.
	srcu_read_lock_held() for SRCU.
	rcu_read_lock_trace_held() for RCU Tasks Trace.

```
这些函数是保守的，因此如果它们不确定就会返回 1（例如，如果未设CONFIG_DEBUG_LOCK_ALLOC）
这可以防止诸WARN_ON(!rcu_read_lock_held()) lockdep 被禁用时给出误报

此外，一个独立的内核配置参数 CONFIG_PROVE_RCU 启用rcu_dereference() 原语的检查：

	rcu_dereference(p):
		检RCU 读侧临界区
	rcu_dereference_bh(p):
		检RCU-bh 读侧临界区
	rcu_dereference_sched(p):
		检RCU-sched 读侧临界区
	srcu_dereference(p, sp):
		检SRCU 读侧临界区
	rcu_dereference_check(p, c):
		使用显式检查表达式 "c" 以及 rcu_read_lock_held()。这在既
		RCU 读者又被更新者调用的代码中很有用
	rcu_dereference_bh_check(p, c):
		使用显式检查表达式 "c" 以及 rcu_read_lock_bh_held()。这在既
		RCU-bh 读者又被更新者调用的代码中很有用
	rcu_dereference_sched_check(p, c):
		使用显式检查表达式 "c" 以及 rcu_read_lock_sched_held()。这在既
		RCU-sched 读者又被更新者调用的代码中很有用
	srcu_dereference_check(p, c):
		使用显式检查表达式 "c" 以及 srcu_read_lock_held()。这在既
		SRCU 读者又被更新者调用的代码中很有用
	rcu_dereference_raw(p):
		不检查。（尽量慎用，最好不用。）
	rcu_dereference_raw_check(p):
		完全不做 lockdep。（尽量慎用，最好不用。）
	rcu_dereference_protected(p, c):
		使用显式检查表达式 "c"，并省略所有屏障和编译器约束。这在数
		结构不能改变时很有用，例如在被仅由更新者调用的代码中
	rcu_access_pointer(p):
		返回指针的值并省略所有屏障，但保留防止重复或合并的编译器约束
		这在测试指针本身的值时（例如与 NULL 比较）很有用

rcu_dereference_check() 的检查表达式可以是任何布尔表达式，但通常会包含一lockdep 表达式
对于一
```

	file = rcu_dereference_check(fdt->fd[fd],
				     lockdep_is_held(&files->file_lock) ||
				     atomic_read(&files->count) == 1);

```
该表达式以一RCU 安全的方式获取指"fdt->fd[fd]"，并且，如果配置CONFIG_PROVE_RCU
则验证该表达式被用于

1. 一RCU 读侧临界区（隐式），
2. 持有 files->file_lock，或
3. 一个未共享files_struct

在情(1) 中，指针RCU 安全的方式获取，适用于普通的 RCU 读侧临界区；在情(2) 中，
->file_lock 阻止任何改变发生；最后，在情(3) 中，当前任务是唯一访问file_struct 
任务，同样阻止任何改变发生。如果上述语句仅由更新者调
```

	file = rcu_dereference_protected(fdt->fd[fd],
					 lockdep_is_held(&files->file_lock) ||
					 atomic_read(&files->count) == 1);

```
这将验证上面#2 #3 情况，而且 lockdep 还会抱怨，即使它被用于 RCU 读侧临界区，除非
这两种情况之一成立。因rcu_dereference_protected() 省略了所有屏障和编译器约束，它生成的
代码比其rcu_dereference() 变体更好。另一方面，如果受 RCU 保护的指针或它指向的RCU
保护的数据可能并发改变，则使rcu_dereference_protected() 是非法的

rcu_dereference() 类似，当启用 lockdep 时，RCU list hlist 遍历原语会检查是否从
RCU 读侧临界区内调用。然而，可以把一lockdep 表达式作为额外的可选参数传给它们。有
这个 lockdep 表达式，这些遍历原语仅当 lockdep 表达式为假且它们从任RCU 读侧临界区之
调用时才会抱怨

例如，workqueue for_each_pwq() 宏旨在既RCU 读侧临界区内使用，也在持wq->mutex
时使用
```

	#define for_each_pwq(pwq, wq)
		list_for_each_entry_rcu((pwq), &(wq)->pwqs, pwqs_node,
					lock_is_held(&(wq->mutex).dep_map))

```