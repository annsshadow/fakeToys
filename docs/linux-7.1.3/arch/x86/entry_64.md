
## 内核入口

本文档记录了 arch/x86/entry/entry_64.S 中的一些内核入口。其中大量解释改编自
Ingo Molnar 的一封邮件：

https://lore.kernel.org/r/20110529191055.GC9835%40elte.hu

x86 架构有相当多种不同的方式可以跳入内核代码。这些入口点大多注册在
arch/x86/kernel/traps.c 中，并分别在 arch/x86/entry/entry_64.S（64 位）、
arch/x86/entry/entry_32.S（32 位）以及 arch/x86/entry/entry_64_compat.S（它实现
32 位兼容的 syscall 入口点，从而为 32 位进程提供了在 64 位内核上执行 syscall 的
能力）中实现。

IDT 向量分配列在 arch/x86/include/asm/irq_vectors.h 中。

其中一些入口点包括：

 - system_call：来自 64 位代码的 syscall 指令。

 - entry_INT80_compat：来自 32 位或 64 位代码的 int 0x80；无论哪种方式都是兼容
   syscall。

 - entry_INT80_compat、ia32_sysenter：来自 32 位代码的 syscall 和 sysenter。

 - interrupt：一组入口点构成的数组。每个没有显式指向别处的 IDT 向量都会被设为
   interrupts 中的对应值。这些指向一整个由魔法般生成的函数的数组，这些函数以
   中断号作为参数，最终到达 common_interrupt()。

 - APIC 中断：用于诸如 TLB shootdown 等各种特殊用途的中断。

 - 架构定义的异常，如 divide_error。

这里有一些复杂性。不同的 x86-64 入口点有不同的调用约定。syscall 和 sysenter 指令
有它们自己特殊的调用约定。某些 IDT 项会将错误码压入栈中；其它的则不会。使用 IST
替代栈机制的 IDT 项需要它们自己的魔法来正确设置栈帧。（你可以在 AMD APM 第 2 卷
第 8 章以及 Intel SDM 第 3 卷第 6 章中找到一些文档。）

处理 swapgs 指令尤其棘手。Swapgs 切换 gs 是内核 gs 还是用户 gs。swapgs 指令相当
脆弱：它必须完美嵌套且只能是单层的；它只应在从用户模式进入内核模式、以及随后返回
用户空间时使用，并且必须精确如此。哪怕我们稍微搞错了它，系统就会崩溃。

因此，当我们有一个已经进入内核模式的二级入口时，我们*绝不能*盲目地使用 SWAPGS——
也不能在尚未切换/交换时忘记执行 SWAPGS。

现在，有第二个复杂之处：有一种廉价的方法和一种昂贵的方法来测试 CPU 处于哪种模式。

廉价的方法是从内核栈上的入口帧中取出这一信息
```

	xorl %ebx,%ebx
	testl $3,CS+8(%rsp)
	je error_kernelspace
	SWAPGS

```
昂贵（paranoid，多疑）的方法是读回 MSR_GS_BASE 的值
```

	movl $1,%ebx
	movl $MSR_GS_BASE,%ecx
	rdmsr
	testl %edx,%edx
	js 1f   /* negative -> in kernel */
	SWAPGS
	xorl %ebx,%ebx
  1:	ret

```
如果我们处于中断或用户陷阱/门类似的边界，就可以使用更快的检查：栈会是一个可靠的
指标，表明 SWAPGS 是否已经执行过：如果我们看到这是一个打断了内核模式执行的二级入口，
那么我们就知道 GS 基址已经被切换过了。如果它表明我们打断了用户空间执行，那么我们就
必须执行 SWAPGS。

但是，如果我们处于 NMI/MCE/DEBUG/whatever 的超原子（super-atomic）入口上下文中，
它可能在普通入口将 CS 写入栈之后、但在我们执行 SWAPGS 之前就触发了，那么检查 GS 的
唯一安全方式就是较慢的方法：RDMSR。

因此，超原子入口（NMI 除外，它单独处理）必须使用 idtentry 并设置 paranoid=1，以正确
处理 gsbase。这会触发三个主要的行为变化：

 - 中断入口将使用较慢的 gsbase 检查。
 - 来自用户模式的中断入口将关闭 IST 栈。
 - 到内核模式的中断退出将不会尝试重新调度。

我们尽量只对那些确实需要进行更昂贵的 GS 基址检查的向量使用 IST 入口和 paranoid 入口
代码——而所有“普通”入口点我们都用常规的（更快的）paranoid=0 变体来生成。
