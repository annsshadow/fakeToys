## AArch64 Linux 上的内存布局


作者：Catalin Marinas <catalin.marinas@arm.com>

本文档描述 AArch64 Linux 内核所使用的虚拟内存布局。该架构在 4KB 页大小下支持最多 4 级页表，在 64KB 页大小下支持最多 3 级页表。

AArch64 Linux 在 4KB 页配置下使用 3 级或 4 级页表，分别允许用户和内核使用 39 位（512GB）或 48 位（256TB）的虚拟地址。在 64KB 页下，仅使用 2 级页表，允许 42 位（4TB）虚拟地址，但内存布局相同。

ARMv8.2 增加了对大虚拟地址空间的可选支持。这仅在使用 64KB 页大小时可用，并扩展了第一级页表中的描述符数量。

TTBRx 由虚拟地址的第 55 位选择。swapper_pg_dir 仅包含内核（全局）映射，而用户 pgd 仅包含用户（非全局）映射。swapper_pg_dir 地址被写入 TTBR1，而永远不会写入 TTBR0。

在未使用虚拟化主机扩展（Virtualization Host Extensions）而使用 KVM 时，hypervisor 在 EL2 中将内核页映射到与线性映射相距一个固定（且可能随机）的偏移处。更多细节请参见 kern_hyp_va 宏和 kvm_update_va_mask 函数。诸如 GICv2 之类的 MMIO 设备被映射到 HYP idmap 页旁边，当为特定 CPU 启用 ARM64_SPECTRE_V3A 时，向量（vectors）也是如此。

在使用带虚拟化主机扩展的 KVM 时，不会创建额外的映射，因为宿主内核直接在 EL2 中运行。

### 内核中的 52 位 VA 支持

如果存在 ARMv8.2-LVA 可选特性，并且我们使用 64KB 页大小；那么就可以对用户空间和内核地址都使用 52 位地址空间。然而，任何支持 52 位的 kernel 二进制文件也必须能够在硬件特性不存在时，在早期启动阶段回退到 48 位。

该回退机制要求内核的 .text 位于较高地址，使其对 48/52 位 VA 保持不变。由于 kasan 影子区是整个内核 VA 空间的一部分，kasan 影子的末端对于 48 位和 52 位也必须位于内核 VA 空间的上半部分。（从 48 位切换到 52 位时，kasan 影子的末端保持不变并依赖于 ~0UL，而起始地址会向较低地址“增长”。）

为了优化 phys_to_virt 和 virt_to_phys，PAGE_OFFSET 保持在常量 0xFFF0000000000000（对应 52 位），这避免了对额外变量的读取。physvirt 偏移和 vmemmap 偏移在早期启动时计算，以启用该逻辑。

由于单个二进制需要同时支持 48 位和 52 位 VA 空间，VMEMMAP 必须足够大以容纳 52 位 VA，同时也必须足够大以容纳固定的 PAGE_OFFSET。

内核中的大多数代码不需要考虑 VA_BITS，对于确实需要知道 VA 大小的情况，变量定义如下：

VA_BITS		const	**最大** VA 空间大小

VA_BITS_MIN	const	**最小** VA 空间大小

vabits_actual	variable	**实际** VA 空间大小


最大和最小大小有助于确保缓冲区被调整得足够大，或地址被定位得足够近，以应对“最坏”情况。

### 52 位用户空间 VA

为了保持对依赖 ARMv8.0 VA 空间最大大小为 48 位的软件的兼容性，默认情况下，内核将向用户空间返回来自 48 位范围的虚拟地址。

软件可以通过指定一个大于 48 位的 mmap 提示参数来“选择加入”接收来自 52 位空间的 VA。

例如：


   maybe_high_address = mmap(~0UL, size, prot, flags,...);

也可以构建在启用以下内核配置选项时返回来自 52 位空间地址的调试内核：


   CONFIG_EXPERT=y && CONFIG_ARM64_FORCE_52BIT=y

请注意，该选项仅用于调试应用程序，不应在生产环境中使用。
