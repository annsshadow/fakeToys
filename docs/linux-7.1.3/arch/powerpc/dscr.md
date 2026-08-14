## DSCR（数据流控制寄存器，Data Stream Control Register）


powerpc 上的 DSCR 寄存器允许用户对处理器中数据流的预取进行一定的控制。关于如何使用此 DSCR
来获得对预取的控制，请参阅 ISA 文档或相关手册以获取更详细的信息。本文档概述了内核
对 DSCR 的支持、相关的内核对象、其功能以及导出的用户接口。

(A) 数据结构：

```

		dscr		/* 线程 DSCR 值 */
		dscr_inherit	/* 线程已更改默认 DSCR */

	(2) PACA::

		dscr_default	/* 每 CPU 的 DSCR 默认值 */

	(3) sysfs.c::

		dscr_default	/* 系统 DSCR 默认值 */

```
(B) 调度器改动：

	如果线程的 dscr_inherit 值为清零状态（意味着它到目前为止还没有更改过默认 DSCR），
	调度器会将存储在 CPU 的 PACA 值中的每 CPU DSCR 默认值写入寄存器。如果设定了
	dscr_inherit 值（意味着它已经更改了默认 DSCR 值），调度器将写入更改后的值，该值
	现在包含在 thread 结构体的 dscr 中，而不是基于每 CPU 默认 PACA 的 DSCR 值。

	注意：请注意，系统范围的全局 DSCR 值在调度器的进程上下文切换中根本不会被直接使用。

(C) SYSFS 接口：

 - 全局 DSCR 默认值：		/sys/devices/system/cpu/dscr_default
 - CPU 特定的 DSCR 默认值：	/sys/devices/system/cpu/cpuN/dscr

	在 sysfs 中更改全局 DSCR 默认值会立即更改其 PACA 结构中的所有 CPU 特定 DSCR 默认值。
	同样，如果当前进程的 dscr_inherit 是清零的，它也会立即将新值写入每个 CPU 的 DSCR 寄存器，
	并更新当前线程的 DSCR 值。

	在 sysfs 中更改 CPU 特定的 DSCR 默认值所做的事情与上述完全相同，但与上面的全局值不同，
	它只更改该特定 CPU 的内容，而不是系统中所有 CPU 的内容。

(D) 用户空间指令：

	DSCR 寄存器可以在用户空间中使用为此目的提供的任意一个 SPR 编号来访问。

	(1) 问题态 SPR：		0x03	（非特权，仅 POWER8）
	(2) 特权态 SPR：		0x11	（特权）

	从用户空间通过特权 SPR 编号（0x11）访问 DSCR 是可行的，因为它在内核内的非法指令
	异常之后被模拟。mfspr 与 mtspr 指令都会被模拟。

	从用户空间通过用户级 SPR（0x03）访问 DSCR 首先会创建一个 facility unavailable 异常。
	在此异常处理程序内部，所有基于 mfspr 指令的读取尝试都会被模拟并返回，而基于 mtspr 指令
	的首次写入尝试会通过设置 FSCR 寄存器中的 DSCR facility 来为下一次（读和写）启用 DSCR facility。

(E) 关于 'dscr_inherit' 的细节：

	线程结构体元素 'dscr_inherit' 表示相关线程是否曾尝试并使用以下任一方法自行更改过 DSCR。
	该元素表示线程是想要在内核中使用 CPU 默认的 DSCR 值，还是它自己更改过的 DSCR 值。

		(1) mtspr 指令	（SPR 编号 0x03）
		(2) mtspr 指令	（SPR 编号 0x11）
		(3) ptrace 接口	（显式设置用户 DSCR 值）

	在此事件之后由该进程创建的任何子进程也会继承相同的行为。
