
## 使用 RCU 保护动NMI 处理程序

虽然 RCU 通常用于保护以读为主的数据结构，但也可以使用 RCU 来提供动态的
不可屏蔽中断（NMI）处理程序，以及动态的 irq 处理程序。本文档描述了如何做这一点，它大致借鉴Zwane Mwaikambo 在旧版本 "arch/x86/kernel/traps.c" 中的
NMI-timer 工作
相关的代码片段列在下面，每一段之后都附有
```

	static int dummy_nmi_callback(struct pt_regs *regs, int cpu)
	{
		return 0;
	}

```
dummy_nmi_callback() 函数是一个“dummy”（空）NMI 处理程序，它什么也不做，但返回
零，从而表示它什么也没做，并允许
```

	static nmi_callback_t nmi_callback = dummy_nmi_callback;

```
这个 nmi_callback 变量是指向当NMI 处理程序的全局函数指针
```

	void do_nmi(struct pt_regs * regs, long error_code)
	{
		int cpu;

		nmi_enter();

		cpu = smp_processor_id();
		++nmi_count(cpu);

		if (!rcu_dereference_sched(nmi_callback)(regs, cpu))
			default_do_nmi(regs);

		nmi_exit();
	}

```
do_nmi() 函数处理每个 NMI。它首先以与硬件 irq 相同的方式禁用抢占，然后递增 per-CPU
NMI 计数。接着它调用存储在 nmi_callback 函数指针中的 NMI 处理程序。如果该处理程序
返回零，do_nmi() 就调default_do_nmi() 函数来处理机器特定的 NMI。最后，恢复抢占
理论上，rcu_dereference_sched() 并不是必需的，因为这段代码只运行在 i386 上，i386
理论上本来也不需rcu_dereference_sched()。然而，在实践中它是一个很好的文档辅助特别是对于那些试图在 Alpha 或使用了激进优化编译器的系统上做类似事情的人
快速测验：
		考虑到指针所引用的代码是只读的，为什么在 Alpha 上可能仍然需		rcu_dereference_sched()
快速测验的答案 <answer_quick_quiz_NMI>

```

	void set_nmi_callback(nmi_callback_t callback)
	{
		rcu_assign_pointer(nmi_callback, callback);
	}

```
set_nmi_callback() 函数注册一NMI 处理程序。注意，任何要被回调使用的数据都必须调用 set_nmi_callback() *之前* 完成初始化。在不对写入进行排序的架构上rcu_assign_pointer() 确保 NMI 处理程序能看```

	void unset_nmi_callback(void)
	{
		rcu_assign_pointer(nmi_callback, dummy_nmi_callback);
	}

```
这个函数注销一NMI 处理程序，恢复原始的 dummy_nmi_handler()。但是，可能恰好有某其它 CPU 上正在执行一NMI 处理程序。因此，在所有其CPU 上该处理程序执行完毕之前我们不能释放旧的 NMI 处理程序所使用的任何数据结构
一种实现方式是借助 synchronize_rcu()，例```

	unset_nmi_callback();
	synchronize_rcu();
	kfree(my_nmi_data);

```
这是可行的，因为（从 v4.20 起）synchronize_rcu() 会一直阻塞，直到所CPU 完成它们
正在执行的任何禁用抢占的代码段。由NMI 处理程序会禁用抢占，synchronize_rcu() 保证
在所有正在进行的 NMI 处理程序退出之前不会返回。因此，一synchronize_rcu() 返回就可以安全地释放该处理程序的数据
重要提示：要让上述机制工作，相关架构必须NMI 进入和退出时分别调用 nmi_enter() nmi_exit()

快速测验的答案		考虑到指针所引用的代码是只读的，为什么在 Alpha 上可能仍然需		rcu_dereference_sched()
		调用 set_nmi_callback() 的人很可能已经初始化了某些要被新NMI
		处理程序使用的数据。在这种情况下，就需rcu_dereference_sched()		因为否则，在设置了新处理程序之后立即收到 NMI CPU，可能会看到
		指向NMI 处理程序的指针，但看到的是处理程序数据的旧（预初始化		版本
		当使用一个带有激进指针值推测（pointer-value speculation）优化的
		编译器时，同样的糟糕情况也会发生在其CPU 上。（但请别这样！
		更重要的是，rcu_dereference_sched() 让阅读代码的人清楚地知道，该
		指针正受RCU-sched 的保护