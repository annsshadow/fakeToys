
## PPC KVM 半虚拟化（paravirtual）接口


KVM on PowerPC 的基本运行原理是：内核态代码以 PR=1（用户态）方式运行于客户机中。特权指令由此以相应的方式陷入（trap）并被模拟。

但不幸的是，这也有其缺陷。相当一部分特权指令即便本可以不同方式处理，也会不必要地返回到我们的 hypervisor。

PPC PV 接口正是用于解决这一问题。它将特权指令转换为非特权指令来辅助 hypervisor，从而将虚拟化开销在我的基准测试中降低了约 50%。

该接口的代码位于 `arch/powerpc/kernel/kvm*`。


## 查询存在性（Querying existence）

要判断自己是否运行于 KVM 之上，可利用设备树（device tree）。在运行于 KVM 的 Linux 中，会存在 `/hypervisor` 节点。该节点包含一个值为 "linux,kvm" 的 `compatible` 属性。

一旦确定自己运行于支持 PV 的 KVM 之上，即可使用下文描述的 hypercall。


## KVM hypercalls

在设备树的 `/hypervisor` 节点中，有一个名为 `hypercall-instructions` 的属性。该属性包含构成一次 hypercall 的最多 4 条 opcode。要发起 hypercall，只需执行这些指令即可。

参数约定如下：

========	================	================
寄存器	IN OUT
========	================	================
r0 - volatile
r3 1st 参数 返回 code
r4 2nd 参数 1st output 值
r5 3rd 参数 2nd output 值
r6 4th 参数 3rd output 值
r7 5th 参数 4th output 值
r8 6th 参数 5th output 值
r9 7th 参数 6th output 值
r10 8th 参数 7th output 值
r11 hypercall 编号	8th output 值
r12 - volatile
========	================	================

hypercall 的定义在通用代码中共享，x86 与 powerpc 使用相同的 hypercall 编号；异常情况是，KVM hypercall 需要与 KVM vendor code（42 << 16）做按位或。

返回码约定如下：

==== =========================
Code Meaning
==== =========================
0 Success
12 Hypercall implemented
<0 错误
==== =========================


## magic 页

为启用 guest 与 hypervisor 之间的通信，引入了一页新的共享内存，其中包含部分仅 supervisor 可见的寄存器状态。guest 可通过 KVM hypercall `KVM_HC_PPC_MAP_MAGIC_PAGE` 映射该共享页。

该 hypercall 由 guest 发起后，总是会把 magic 页映射到期望的位置。第一个参数表示启用 MMU 时的有效地址（effective address）；第二个参数表示实模式（real mode）下的地址，适用于相应目标。目前，magic 页总是映射到 -4096 处。这样便可使用绝对加载/存储函数来访问。例如：

```
	ld	rX, -4096(0)
```

该接口被设计为可扩展的，以便日后向 magic 页添加更多寄存器。向 magic 页添加字段时，应定义新的 hypercall 特性位来指示 host 提供了更多寄存器。若 host 支持该附加特性，即可加以利用。

magic 页的布局由 `arch/powerpc/include/uapi/asm/kvm_para.h` 中的结构体 `kvm_vcpu_arch_shared` 描述。


## Magic 页 特性

映射 magic 页使用 KVM hypercall `KVM_HC_PPC_MAP_MAGIC_PAGE`，其第二个返回值会传给 guest。第二个返回值包含一个位图，指示 magic 页内可用的特性。

目前 magic 页可用的增强特性如下：

============================ =======================================
KVM_MAGIC_FEAT_SR Maps SR 寄存器 r/w magic 页
KVM_MAGIC_FEAT_MAS0_TO_SPRG7	Maps MASn, ESR, PIR high SPRGs
============================ =======================================

要启用 magic 页的增强特性，请先检查该特性是否存在（使用相应的特性位）！


## Magic 页 标志

除了指示 host 是否支持某个特定特性的"特性"位之外，还存在一种 guest 告知 host "自己也支持某能力"的通道，称为"标志"。

标志通过有效地址（Effective address）的低 12 位传给 host。

目前 guest 可暴露的标志如下：

MAGIC_PAGE_FLAG_NOT_MAPPED_NX Guest 能正确处理 magic 页的 NX 位


## MSR 位

MSR 中包含一些需要 hypervisor 介入的位，以及一些需要直接由 hypervisor 解释、在进入 guest 时不影响 hypervisor 行为的位。

以下位可在 guest 内安全设置：

- MSR_EE
- MSR_RI

对 MSR 的位进行修改时，仍请使用 `mtmsr(d)`。


## Patched instructions（补丁化指令）

"ld" 与 "std" 指令分别被转换为 "lwz" 与 "stw" 指令（在 32 位系统上，并加上偏移量 4 以适应大端序）。

以下是 Linux 内核在 guest 运行时所执行的映射。实现这些映射是可选的——若指令陷入，仍会按共享页方式处理；调用特权指令同样可行。

======================= ================================
mfmsr	rX ld	rX, magic_page->msr
mfsprg	rX, 0 ld	rX, magic_page->sprg0
mfsprg	rX, 1 ld	rX, magic_page->sprg1
mfsprg	rX, 2 ld	rX, magic_page->sprg2
mfsprg	rX, 3 ld	rX, magic_page->sprg3
mfsrr0	rX ld	rX, magic_page->srr0
mfsrr1	rX ld	rX, magic_page->srr1
mfdar	rX ld	rX, magic_page->dar
mfdsisr	rX lwz	rX, magic_page->dsisr
mtmsr	rX std	rX, magic_page->msr
mtsprg	0, rX std	rX, magic_page->sprg0
mtsprg	1, rX std	rX, magic_page->sprg1
mtsprg	2, rX std	rX, magic_page->sprg2
mtsprg	3, rX std	rX, magic_page->sprg3
mtsrr0	rX std	rX, magic_page->srr0
mtsrr1	rX std	rX, magic_page->srr1
mtdar	rX std	rX, magic_page->dar
mtdsisr	rX stw	rX, magic_page->dsisr
tlbsync nop
mtmsrd	rX, 0 b	<special mtmsr 章节>
mtmsr	rX b	<special mtmsr 章节>
mtmsrd	rX, 1 b	<special mtmsrd 章节>
[Book3S ]
mtsrin	rX, rY b	<special mtsrin 章节>
[BookE ]
wrteei	[0|1] b	<special wrteei 章节>
======================= ================================

对于那些需要更多逻辑来判断是加载还是存储指令被交付的指令，启用补丁（patching）后，会在实时翻译指令的 RAM 周围保留空间。其过程如下：

1) 将模拟代码复制到内存
2) 补丁化代码以适配被模拟的指令
3) 补丁化代码使其返回原始 pc + 4
4) 将被补丁化的原始指令分支到新代码

由此，可用任意数量的代码替换单条指令。例如，这允许我们通过设置 EE=1 来检查挂起的中断。


## Hypercall ABIs（KVM PowerPC）

1) KVM hypercalls (ePAPR)

符合 ePAPR 的 hypercall 实现（如前所述）。即便通用 hypercall 已实现（如 ePAPR idle hcall），也可用。适用于相应 targets。

2) PAPR hypercalls

运行 server PowerPC PAPR guest（`-M pseries` QEMU）需要 PAPR hypercall。这些 hypercall 与 pHyp（POWER hypervisor）实现的相同。一部分由内核处理，一部分由用户空间处理。可用于 book3s_64。

3) OSI hypercalls

Mac-on-Linux 用户为 KVM PowerPC 提供了自己的 hypercall（沿用自早期的 KVM）。为保持兼容性而支持这些 hypercall。它们会被转发到用户空间。对 book3s_32 有用，同样适用于 book3s_64。
