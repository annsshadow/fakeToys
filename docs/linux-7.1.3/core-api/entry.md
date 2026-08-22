## 异常、中断、系统调用与 KVM 的进退出处


各执行域之间的切换都需要进行状态更新，这些更新
受严格的顺序约束。以下情形需要执行状态更新：


  - Lockdep
  - RCU / 上下文跟
  - 抢占计数
  - 跟踪
  - 时间记账

更新顺序取决于切换类型，并在下文的切换类型章节中说明：`Syscalls`_、`KVM`_、`Interrupts and regular
exceptions`_, `NMI and NMI-like exceptions`_銆。
exceptions`_, `NMI and NMI-like exceptions`_.

### 不可插桩代码 - noinstr


大多数插桩机制依RCU，因此在 RCU 开始监视之前的进入代码，以RCU 停止监视之后的退出代码中，禁止进行插桩。此外，许多体系结构必须保存和恢复寄存器状态，这意味着（例如）在断点进入代码中放置一个断点会覆盖初始断点的调试寄存器





此类代码必须使用 'noinstr' 属性标记，将其放入插桩与调试工具都无法访问的特殊段中。部分函数可部分插桩，其处理方式是将函数标记noinstr，并使用 instrumentation_begin() instrumentation_end() 来标记可插桩的代码范围：






  noinstr void entry(void)
  {
  	handle_entry();     // <-- must be 'noinstr' or '__always_inline'
	...

	instrumentation_begin();
	handle_context();   // <-- instrumentable code
	instrumentation_end();

	...
	handle_exit();      // <-- must be 'noinstr' or '__always_inline'
  }

这样便可在受支持的体系结构上通过 objtool 验证 'noinstr' 限制


从可插桩上下文中调用不可插桩函数没有任何限制，并且有助于保护那些一旦被插桩就会出错的代码，例如状态切换



RCU 状态切换之前与之后的所有不可插桩进退出代码段都必须在中断被禁止的情况下运行


### 系统调用


系统调用进入代码始于汇编代码，在建立底层与体系结构相关的状态和栈帧之后，调用底C 代码。这段底C 代码不能被插桩。一个从底层汇编代码调用的典型系统调用处理函数如下所示：





  noinstr void syscall(struct pt_regs *regs, int nr)
  {
	arch_syscall_enter(regs);
	nr = syscall_enter_from_user_mode(regs, nr);

	instrumentation_begin();
	if (!invoke_syscall(regs, nr) && nr != -1)
	 	result_reg(regs) = __sys_ni_syscall(regs);
	instrumentation_end();

	syscall_exit_to_user_mode(regs);
  }

syscall_enter_from_user_mode() 首先调用 enter_from_user_mode()，该函数按以下顺序建立状态：


  - Lockdep
  - RCU / 上下文跟
  - 跟踪

随后调用各种进入阶段的工作函数，ptrace、seccomp、audit、syscall tracing 等。完成这些工作后，方可调用可插桩invoke_syscall 函数。可插桩代码段到此结束，之后调用 syscall_exit_to_user_mode()




syscall_exit_to_user_mode() 处理返回用户空间之前需要完成的所有工作，例如跟踪、audit、信号、task work 等。之后它调用 exit_to_user_mode()，该函数以相反的顺序再次处理状态切换：




  - 跟踪
  - RCU / 上下文跟
  - Lockdep

syscall_enter_from_user_mode() syscall_exit_to_user_mode() 也可作为细粒度的子函数使用，适用于体系结构代码需要在各步骤之间完成额外工作的场景。这种情况下，必须确保在进入时首先调enter_from_user_mode()，并在退出时最后调exit_to_user_mode()





不要嵌套系统调用。嵌套系统调用会导致 RCU 或上下文跟踪打印警告


### KVM


进入或退出客户机模式与系统调用非常相似。从宿主内核的角度看，CPU 在进入客户机时仿佛进入了用户空间，而在退出时返回内核



guest_state_enter_irqoff() exit_to_user_mode() KVM 专用变体，guest_state_exit_irqoff() enter_from_user_mode() KVM 变体。其状态操作的顺序相同



对于客户机，任务工作的处理在 vcpu_run() 循环的边界处通过 xfer_to_guest_mode_handle_work() 单独进行，该函数处理的是返回用户空间时处理工作的一部分子集



不要嵌套 KVM 的进退出切换，因为这样做毫无意义

### 中断与常规异


中断的进入与退出处理比系统调用KVM 切换要稍稍复杂一些


如果中断是在 CPU 执行用户空间代码时触发的，其进入与退出处理与系统调用完全相同


如果中断是在 CPU 执行内核空间代码时触发的，其进入与退出处理则略有不同。只有当中断是在 CPU 空闲任务的上下文中触发时，才会更RCU 状态；否则 RCU 已经在监视中。Lockdep tracing 必须无条件更新




irqentry_enter() irqentry_exit() 提供了此功能的实现

与体系结构相关的部分与系统调用处理类似：


  noinstr void interrupt(struct pt_regs *regs, int nr)
  {
	arch_interrupt_enter(regs);
	state = irqentry_enter(regs);

	instrumentation_begin();

	irq_enter_rcu();
	invoke_irq_handler(regs, nr);
	irq_exit_rcu();

	instrumentation_end();

	irqentry_exit(regs, state);
  }

请注意，实际中断处理程序的调用位irq_enter_rcu() irq_exit_rcu() 这一对调用之间


irq_enter_rcu() 会更新抢占计数，in_hardirq() 返回 true，并处理 NOHZ tick 状态与中断时间记账。这意味着在调irq_enter_rcu() 之前，in_hardirq() 一直返false




irq_exit_rcu() 处理中断时间记账，撤销抢占计数的更新，并最终处理软中断NOHZ tick 状态


理论上，抢占计数可以irqentry_enter() 中更新。但实际上，将此更新推迟irq_enter_rcu() 可以让抢占计数相关代码被跟踪，同时保持与 irq_exit_rcu() irqentry_exit()（在下一段中描述）的对称性。唯一的缺点是，在调用 irq_enter_rcu() 之前的早期进入代码必须意识到抢占计数尚未更新HARDIRQ_OFFSET 状态






注意，irq_exit_rcu() 在处理软中断之前，必须从抢占计数中移HARDIRQ_OFFSET，因为软中断的处理程序必须在 BH 上下文中运行，而不是在中断被禁止的上下文中。此外，irqentry_exit() 可能会进行调度，这同样要求从抢占计数中移HARDIRQ_OFFSET




尽管中断处理程序应当在本地中断被禁止的情况下运行，但从进退出的角度看，中断嵌套是很常见的。例如，软中断的处理就发生在本地中断处于开启状态的 irqentry_{enter,exit}() 代码块内。此外，虽然不常见，但没有任何机制阻止中断处理程序重新开启中断





中断的进退出代码并不严格要求处理可重入性，因为它是在本地中断被禁止的情况下运行的。但 NMI 可能在任何时候发生，而且两者共享大量进入代码



### NMI 与类 NMI 异常


NMI 与类 NMI 异常（machine checks、double faults、debug 中断等）可以发生在任何上下文中，必须格外谨慎地对待状态



调试异常machine-check 异常的状态变化取决于这些异常发生在用户空间（断点或观察点）还是内核模式（代码补丁）。在用户空间中，它们被当作中断处理；在内核模式中，它们被当作 NMI 处理




NMI 与其他类 NMI 异常处理状态切换时，不区分其来源是用户模式还是内核模式


进入时的状态更新由 irqentry_nmi_enter() 处理，该函数按以下顺序更新状态：


  - 抢占计数
  - Lockdep
  - RCU / 上下文跟
  - 跟踪

其退出对应函irqentry_nmi_exit() 以相反的顺序执行反向操作


注意，抢占计数的更新在进入时必须是第一个操作，在退出时必须是最后一个操作。原因是 lockdep RCU 都依赖于 in_nmi() 在这种情况下返回 true。NMI 进入/退出场景中的抢占计数修改不能被跟踪





与体系结构相关的代码如下所示：


  noinstr void nmi(struct pt_regs *regs)
  {
	arch_nmi_enter(regs);
	state = irqentry_nmi_enter(regs);

	instrumentation_begin();
	nmi_handler(regs);
	instrumentation_end();

	irqentry_nmi_exit(regs);
  }

例如，对于调试异常，代码可能如下所示：


  noinstr void debug(struct pt_regs *regs)
  {
	arch_nmi_enter(regs);

	debug_regs = save_debug_regs();

	if (user_mode(regs)) {
		state = irqentry_enter(regs);

		instrumentation_begin();
		user_mode_debug_handler(regs, debug_regs);
		instrumentation_end();

		irqentry_exit(regs, state);
  	} else {
  		state = irqentry_nmi_enter(regs);

		instrumentation_begin();
		kernel_mode_debug_handler(regs, debug_regs);
		instrumentation_end();

		irqentry_nmi_exit(regs, state);
	}
  }

没有可用的组合函irqentry_nmi_if_kernel()，因为上述情况无法以与异常无关的方式处理


NMI 可以发生在任何上下文中。例如，在处NMI 时可能触发一个类 NMI 异常。因此，NMI 的进入代码必须是可重入的，并且状态更新需要处理嵌套


