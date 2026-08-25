
## LoongArch 半虚拟化接口


KVM hypercall 使用 HVCL 指令，操作码0x100，hypercall 编号放在 a0 中。最可将五个参数放入寄存a1 - a5。返回值放v0（即 a0 的别名）中
该接口的相关源代码位arch/loongarch/kvm*
## 存在性查

要判断宿主机是否运行KVM 之上，可以利cpucfg() 函数在索CPUCFG_KVM_BASE (0x40000000) 处进行查询
CPUCFG_KVM_BASE 范围0x40000000 0x400000FFx40000000 - 0x400000FF 之间
CPUCFG_KVM_BASE 范围被标记为保留。因此，当前及未来的所有处理器都不会在范围内实现任何特性
KVM 虚拟化的 Linux 系统上，对索CPUCFG_KVM_BASE (0x40000000) 处的 cpucfg()
执行读操作会返回魔数字符'KVM\0'
一旦确定你的宿主机运行在支持半虚拟化的 KVM 之上，便可以使用如下所述的
hypercall銆。
## KVM hypercall ABI


KVM hypercall ABI 很简单，使用一个临时寄存器 a0（即 v0），以及最多五个通用
寄存器（a1 - a5）作为输入参数。FP（浮点）与向量寄存器不作为输入寄存器使用且在 hypercall 期间必须保持不被修改
Hypercall 函数可以内联，因为它只使用一个临时寄存器
参数如下
	========	=================	================
	Register	IN			OUT
	========	=================	================
	a0		function number		返回	a1		1 个参	-
	a2		2 个参	-
	a3		3 个参	-
	a4		4 个参	-
	a5		5 个参	-
	========	=================	================

返回码可能是下列之一
	====		=========================
	Code		含义
	====		=========================
	0		成功
	-1		Hypercall 未实	-2		错误Hypercall 参数
	====		=========================

## KVM Hypercall 文档


每个 hypercall 的模板如下：

1. Hypercall 名称
2. 用
### 1. KVM_HCALL_FUNC_IPI


:用 向多vCPU 发IPI
- a0: KVM_HCALL_FUNC_IPI
- a1: 目标物理 CPUID 位图32 - a2: 目标物理 CPUID 位图32 - a3: 位图中最小的物理 CPUID

hypercall 允许客户机在单次调用中向最128 个目标发送多IPI（处理器中断）。目标由前两个输入寄存器（a1 a2）中的位图表示
a1 的第 0 位对应第三个输入寄存器（a3）中的物CPUID，第 1 位对a3+1 中的
物理 CPUID，依此类推
LoongArch 上的 PV IPI 同时包含 PV IPI 多播发送与 PV IPI 接收，由于访SWI
寄存器不会产VM-exit，因此使SWI 来注PV IPI