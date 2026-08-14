## CPU 特性（CPU Features）


Hollis Blanchard <hollis@austin.ibm.com>
2002 年 6 月 5 日

本文档描述了 PPC Linux 内核中使用的系统（包括自修改代码），用于支持多种
PowerPC CPU，而无需在编译期进行选择。

在启动过程的早期，ppc32 内核会检测当前的 CPU 类型并相应地选择一组特性。
一些例子包括 Altivec 支持、指令与数据分离的缓存，以及 CPU 是否支持 DOZE 与
NAP 睡眠模式。

特性集合的检测很简单。处理器列表可在 arch/powerpc/kernel/cputable.c 中找到。
PVR 寄存器被掩码处理并与列表中的每个值进行比较。如果找到匹配，cur_cpu_spec
的 cpu_features 会被赋值为该处理器的特性位掩码，并调用一个 __setup_cpu 函数。

C 代码可以测试 'cur_cpu_spec[smp_processor_id()]->cpu_features' 来获取某个
特定的特性位。这一操作在很多地方都会进行，例如在 ppc_setup_l2cr() 中。

在汇编中实现 cpufeatures 要稍微复杂一些。有若干性能关键路径，如果加入数组
索引、结构体解引用和条件分支就会受影响。为了避免性能损失，同时仍允许运行时
（而非编译期）CPU 选择，未使用的代码会被替换为 'nop' 指令。这种 nop 替换
基于 CPU 0 的能力，因此由非相同处理器组成的多处理器系统将无法工作（不过这样
的系统本来也可能会有其它问题）。

在检测到处理器类型之后，内核会通过写入 nop 来修补掉不应被使用的代码段。使用
cpufeatures 只需要 2 个宏（位于 arch/powerpc/include/asm/cputable.h 中），
如 head.S 中所示：

```

	#ifdef CONFIG_ALTIVEC
	BEGIN_FTR_SECTION
		mfspr	r22,SPRN_VRSAVE		/* if G4, save vrsave register value */
		stw	r22,THREAD_VRSAVE(r23)
	END_FTR_SECTION_IFSET(CPU_FTR_ALTIVEC)
	#endif /* CONFIG_ALTIVEC */

```
如果 CPU 0 支持 Altivec，则代码保持不变。如果不支持，两条指令都会被替换为
nop。

END_FTR_SECTION 宏有两个更简单的变体：END_FTR_SECTION_IFSET 与
END_FTR_SECTION_IFCLR。它们分别用于测试某个标志（在
cur_cpu_spec[^0^]->cpu_features 中）是否被置位或清除。在大多数情况下应使用
这两个宏。

END_FTR_SECTION 宏的实现方式是将有关这段代码的信息存储在 '__ftr_fixup' ELF
段中。当 do_cpu_ftr_fixups（arch/powerpc/kernel/misc.S）被调用时，它会遍历
__ftr_fixup 中的记录，如果所需特性不存在，就会从每个 BEGIN_FTR_SECTION 到
END_FTR_SECTION 循环写入 nop。
