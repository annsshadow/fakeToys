## Proper Locking Under a Preemptible Kernel: Keeping Kernel Code Preempt-Safe


:Author: Robert Love <rml@tech9.net>


## Introduction


可抢占内核带来了新的锁问题。这些问题与 SMP 下的问题相同：并发和重入。值得庆幸的是，Linux 可抢占内核模型利用了现有SMP 锁机制。因此，内核仅在极少数额外情况下才需要显式的额外加锁
本文档面向所有内核黑客。在内核中开发代码需要保护这些情况

##### RULE #1: Per-CPU data structures need explicit protection


```

	struct this_needs_locking tux[NR_CPUS];
	tux[smp_processor_id()] = some_value;
	/* task is preempted here... */
	something = tux[smp_processor_id()];

```
首先，由于数据是 per-CPU 的，它可能没有明确使SMP 锁，但在其他方面需要它。其次，当一个被抢占的任务最终被重新调度时，smp_processor_id 之前的值可能不等于当前值。你必须通过在这些情况周围禁用抢占来保护它们
你也可以使用 put_cpu() get_cpu()，它们会禁用抢占

##### RULE #2: CPU state must be protected.


在抢占下，CPU 的状态必须被保护。这与体系架构相关，但包括上下文切换时不被保存的 CPU 结构和状态。例如，x86 上，进入和退FPU 模式现在是一个临界区，必须在禁用抢占的情况下进行。试想一下，如果内核正在执行一条浮点指令，然后被抢占，会发生什么。请记住，内核不会保FPU 状态，只有用户任务才会。因此，一旦被抢占，FPU 寄存器就会卖给（sold to）出价最低者。因此，必须在这些区域周围禁用抢占
请注意，某些 FPU 函数已经明确是抢占安全的。例如，kernel_fpu_begin kernel_fpu_end 会禁用和启用抢占

##### RULE #3: Lock acquire and release must be performed by same task


在一个任务中获取的锁必须由同一个任务释放。这意味着你不能做诸如获取一个锁然后去干别的事、而由另一个任务来释放它这样的怪事。如果你想做类似的事情，应在同一代码路径中获取并释放任务，并让调用者等待另一个任务发出的事件

## Solution


在抢占下保护数据是通过在临界区持续期间禁用抢占来实现的
```

  preempt_enable()		decrement the preempt counter
  preempt_disable()		increment the preempt counter
  preempt_enable_no_resched()	decrement, but do not immediately preempt
  preempt_check_resched()	if needed, reschedule
  preempt_count()		return the preempt counter

```
这些函数是可嵌套的。换句话说，你可以在一条代码路径中调用 preempt_disable n 次，而在n 次调preempt_enable 之前，抢占不会被重新启用。如果未启用抢占，这preempt 语句定义为空
请注意，如果你持有任何锁或中断被禁用，则不需要显式防止抢占，因为在这些情况下抢占是隐式禁用的
但要记住irqs disabled' 是一种从根本上看不安全的禁用抢占方式——任cond_resched() cond_resched_lock() 都可能在抢占计数0 时触发重新调度。一个简单的 printk() 就可能触发重新调度。因此，只有在你知道受影响的代码路径不会做任何这类事情时，才使用这种隐式禁用抢占的特性。最佳策略是仅将其用于你编写的、小而原子的、且不调用复杂函数的代码
```

	cpucache_t *cc; /* this is per-CPU */
	preempt_disable();
	cc = cc_data(searchp);
	if (cc && cc->avail) {
		__free_block(searchp, cc_entry(cc), cc->avail);
		cc->avail = 0;
	}
	preempt_enable();
	return 0;

```
注意，抢占语句必须涵盖对以下内容的每一次引
```

	int buf[NR_CPUS];
	set_cpu_val(buf);
	if (buf[smp_processor_id()] == -1) printf(KERN_INFO "wee!\n");
	spin_lock(&buf_lock);
	/* ... */

```
这段代码不是抢占安全的，但看看我们只需spin_lock 上移两行就能多容易地修复它

## Preventing preemption using interrupt disabling


可以使用 local_irq_disable local_irq_save 来防止抢占事件。请注意，这样做时，你必须非常小心，不要引发会设need_resched 并导致抢占检查的事件。当有疑问时，依靠锁或显式的抢占禁用
请注意，2.5 中，禁用中断现在只是 per-CPU 的（即本地的）
另一个关注点是对 local_irq_disable local_irq_save 的正确使用。它们可用于保护免受抢占，但是，在退出时，如果可能启用抢占，则应当做一次检查，看是否需要抢占。如果它们是spin_lock read/write lock 宏中调用的，就会做正确的事情。它们也可以在自旋锁保护的区域内调用，但是，如果它们在该上下文之外被调用，则应当做一次抢占检查。请注意，来自中断上下文或底半部/tasklet 的调用也受抢占锁保护，因此可以使用不检查抢占的版本