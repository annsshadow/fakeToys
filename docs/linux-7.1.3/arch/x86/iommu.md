## x86 IOMMU 支持


架构规范可以从厂商网站获取。搜索以下文档以获取最新版本：

- Intel：Intel Virtualization Technology for Directed I/O Architecture Specification（ID: D51397）
- AMD：AMD I/O Virtualization Technology (IOMMU) Specification（ID: 48882）

本指南为一些基本理解提供快速备忘单。

### 基础内容


ACPI 枚举并列出平台上不同的 IOMMU，以及设备与哪个 IOMMU 控制它们之间的
设备作用域（device scope）关系。

一些 ACPI 关键字：

- DMAR - Intel DMA 重映射表（DMA Remapping table）
- DRHD - Intel DMA 重映射硬件单元定义（DMA Remapping Hardware Unit Definition）
- RMRR - Intel 保留内存区域报告结构（Reserved Memory Region Reporting Structure）
- IVRS - AMD I/O 虚拟化报告结构（I/O Virtualization Reporting Structure）
- IVDB - AMD I/O 虚拟化定义块（I/O Virtualization Definition Block）
- IVHD - AMD I/O 虚拟化硬件定义（I/O Virtualization Hardware Definition）

##### 什么是 Intel RMRR？


有一些设备由 BIOS 控制，例如 USB 设备用于执行 PS2 仿真。用于这些设备的内存
区域在 e820 映射中被标记为保留。当我们开启 DMA 转换时，对这些区域的 DMA 将
失败。因此 BIOS 使用 RMRR 来指定这些区域以及需要访问这些区域的设备。OS 应当
为这些区域设置统一映射（unity mapping），以便这些设备访问这些区域。

##### 什么是 AMD IVRS？


该架构定义了一个称为 I/O 虚拟化报告结构（IVRS）的 ACPI 兼容数据结构，用于向
系统软件传达与 I/O 虚拟化相关的信息。IVRS 描述了平台中包含的 IOMMU 的配置与
能力，以及每个 IOMMU 虚拟化的设备的信息。

IVRS 提供以下关于以下方面的信息：

- 平台中存在的 IOMMU，包括它们的能力与正确配置
- 与每个 IOMMU 相关的系统 I/O 拓扑
- 无法以其他方式枚举的外设
- SMI/SMM、平台固件与平台硬件使用的内存区域。这些通常是需要由系统软件配置的
  排除范围。

### 如何生成 I/O 虚拟地址（IOVA）？


行为良好的驱动在发送需要执行 DMA 的命令到设备之前调用 dma_map_*() 调用。一旦
DMA 完成且不再需要映射，驱动执行 dma_unmap_*() 调用以取消映射该区域。

### Intel 特定说明


##### 图形问题？


如果你遇到图形设备的问题，可以尝试添加选项 intel_iommu=igfx_off 来关闭集成
图形引擎。如果这修复了任何问题，请确保你提交一个 bug 报告该问题。

##### IOVA 的一些例外


中断范围不被地址转换（0xfee00000 - 0xfeefffff）。对等（peer to peer）事务也
同样如此。因此我们保留来自 PCI MMIO 范围的地址，使它们不被分配给 IOVA 地址。

### AMD 特定说明


##### 图形问题？


如果你遇到集成图形设备的问题，可以尝试在内核命令行上添加选项 iommu=pt，对
IOMMU 使用 1:1 映射。如果这修复了任何问题，请确保你提交一个 bug 报告该问题。

### 故障报告


当报告错误时，IOMMU 通过中断发出信号。导致故障的原因和設備会打印在控制台上。

### 内核日志样例


##### Intel 启动消息


会打印类似以下内容，指示 ACPI 中存在 DMAR 表：

```

	ACPI: DMAR (v001 A M I  OEMDMAR  0x00000001 MSFT 0x00000097) @ 0x000000007f5b5ef0

```
当 DMAR 被 ACPI 处理并初始化时，打印 DMAR 位置以及任何已处理的 RMRR：

```

	ACPI DMAR:Host address width 36
	ACPI DMAR:DRHD (flags: 0x00000000)base: 0x00000000fed90000
	ACPI DMAR:DRHD (flags: 0x00000000)base: 0x00000000fed91000
	ACPI DMAR:DRHD (flags: 0x00000001)base: 0x00000000fed93000
	ACPI DMAR:RMRR base: 0x00000000000ed000 end: 0x00000000000effff
	ACPI DMAR:RMRR base: 0x000000007f600000 end: 0x000000007fffffff

```
当 DMAR 被启用使用时，你会注意到：

```

	PCI-DMA: Using DMAR IOMMU

```
##### Intel 故障报告

```

	DMAR:[DMA Write] Request device [00:02.0] fault addr 6df084000
	DMAR:[fault reason 05] PTE Write access is not set
	DMAR:[DMA Write] Request device [00:02.0] fault addr 6df084000
	DMAR:[fault reason 05] PTE Write access is not set

```
##### AMD 启动消息


会打印类似以下内容，指示 IOMMU 的存在：

```

	iommu: Default domain type: Translated
	iommu: DMA domain TLB invalidation policy: lazy mode

```
##### AMD 故障报告

```

	AMD-Vi: Event logged [IO_PAGE_FAULT domain=0x0007 address=0xffffc02000 flags=0x0000]
	AMD-Vi: Event logged [IO_PAGE_FAULT device=07:00.0 domain=0x0007 address=0xffffc02000 flags=0x0000]

```
