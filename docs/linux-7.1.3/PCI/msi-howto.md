
## MSI 驱动指南 HOWTO

:Authors: Tom L Nguyen; Martine Silbermann; Matthew Wilcox

:Copyright: 2003, 2008 Intel Corporation

## 关于本指
本指南描述了消息信号中断（Message Signaled Interrupts，MSI）的基础知识、使MSI 相比
传统中断机制的优势、如何将你的驱动改为使用 MSI MSI-X，以及当设备不支MSI 时可尝试
的一些基本诊断方法
## 什么是 MSI
消息信号中断是设备向一个特殊地址发起的一次写操作，从而使 CPU 收到一个中断
MSI 能力最早在 PCI 2.2 中规范定义，后来PCI 3.0 中得到增强，允许对每个中断单独进屏蔽。MSI-X 能力也是PCI 3.0 中引入的。它支持每个设备MSI 更多的中断，并允许中断被
独立配置
设备可能同时支持 MSI MSI-X，但同一时间只能启用其中之一
## 为什么要使用 MSI
使用 MSI 相比传统的基于引脚（pin-based）的中断有三个优势
基于引脚PCI 中断通常在多个设备之间共享。为了支持这一点，内核必须调用与该中断相关每一个中断处理程序，这会导致整个系统的性能下降。MSI 从不共享，因此不会出现这个问题
当设备向内存写入数据，然后再拉高基于引脚的中断时，有可能中断在全部数据到达内存之前就
已经到达（在位于 PCI-PCI 桥后面的设备上，这种情况更可能发生）。为了确保所有数据都到达内存，中断处理程序必须读取引发该中断的设备上的一个寄存器。PCI 事务排序规则要求所数据在寄存器返回值之前到达内存。使MSI 可以避免这个问题，因为产生中断的那次写操无法越过数据写操作，因此在中断被引发时，驱动已经知道所有数据都已到达内存
PCI 设备每个功能只能支持一个基于引脚的中断。驱动常常需要查询设备才能知道发生了什么事件，
这拖慢了常见情况下的中断处理。借助 MSI，设备可以支持更多中断，从而使每个中断可以专门用于
不同的目的。一种可能的设计将不常见的条件（如错误）分配给它们自己的中断，从而使驱动能够
更高效地处理正常的中断处理路径。其他可能的设计包括为网卡中的每个数据包队列或存储控制器
中的每个端口分配一个中断
## 如何使用 MSI

PCI 设备在初始化时被设置为使用基于引脚的中断。设备驱动必须设置设备以使用 MSI MSI-X并非所有机器都正确支持 MSI，对于那些机器，下面描述API 会简单地失败，设备将继续使用
基于引脚的中断
### 在内核中启用MSI 的支
要支MSI MSI-X，内核必须使CONFIG_PCI_MSI 选项构建。该选项仅在某些架构上可用，
并且可能还依赖于其他一些选项也被设置。例如，x86 上，你还必须启用 X86_UP_APIC SMP
才能看到 CONFIG_PCI_MSI 选项
### 使用 MSI

大部分繁重的工作已由 PCI 层为驱动完成。驱动只需请求 PCI 层为该设备设MSI 能力
要自动使MSI MSI-X 中断向量，请使用以下函数
```
  int pci_alloc_irq_vectors(struct pci_dev *dev, unsigned int min_vecs,
		unsigned int max_vecs, unsigned int flags);

```
该函数为 PCI 设备分配最max_vecs 个中断向量。它返回所分配的向量数量或负的错误码。如设备对向量数量有最小值要求，驱动可以传入一个设为该下限min_vecs 参数；如PCI 核心
无法满足最小向量数量，将返-ENOSPC
flags 参数用于指定设备和驱动可以使用哪种类型的中断（PCI_IRQ_INTX、PCI_IRQ_MSIPCI_IRQ_MSIX）。还有一个方便的简写（PCI_IRQ_ALL_TYPES）可用于请求任何可能的中断类型如果设置PCI_IRQ_AFFINITY 标志，pci_alloc_irq_vectors() 会在可用CPU 之间分散这些
中断
要获取传request_irq() free_irq() Linux IRQ 号，请使用：

```
  int pci_irq_vector(struct pci_dev *dev, unsigned int nr);

```
如果驱动使用 pcim_enable_device() 启用设备，则不应调用 pci_free_irq_vectors()，因pcim_enable_device() 会激活对 IRQ 向量的自动管理。否则，驱动应在使用以下函数移除设备之前
释放任何已分配的 IRQ 向量
```
  void pci_free_irq_vectors(struct pci_dev *dev);

```
如果设备同时支持 MSI-X MSI 能力，该 API 会优先使MSI-X 而非 MSI。MSI-X 支持 1 2048 之间的任意数量中断。相比之下，MSI 最多限制为 32 个中断（且必须是 2 的幂）。此外，MSI
中断向量必须连续分配，因此系统可能无法为 MSI 分配MSI-X 那样多的向量。在某些平台上，MSI
中断必须全部指向同一CPU，MSI-X 中断则可以全部指向不同的 CPU
如果设备既不支持 MSI-X 也不支持 MSI，它将回退到单个传统的 IRQ 向量
使用 MSI MSI-X 中断的典型做法是尽可能多地分配向量，很可能分配至设备支持的上限。如nvec 大于设备支持的数量，它会自动被限制到支持的上限，因此无需查询支持数量，例如：

```
	nvec = pci_alloc_irq_vectors(pdev, 1, nvec, PCI_IRQ_ALL_TYPES)
	if (nvec < 0)
		goto out_err;

```
如果驱动无法或不愿处理可变数量的 MSI 中断，它可以通过将同一个数量同时作'min_vecs' 'max_vecs' 传给 pci_alloc_irq_vectors() 来请求特定数量的中断，例如：

```
	ret = pci_alloc_irq_vectors(pdev, nvec, nvec, PCI_IRQ_ALL_TYPES);
	if (ret < 0)
		goto out_err;

```
上述请求类型最著名的例子是为设备启用单 MSI 模式。可以通过传入两个 1 来实现：

```
	ret = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_ALL_TYPES);
	if (ret < 0)
		goto out_err;

```
某些设备可能不支持使用传统线路中断，此时可使用：

```
	nvec = pci_alloc_irq_vectors(pdev, 1, nvec, PCI_IRQ_MSI | PCI_IRQ_MSIX);
	if (nvec < 0)
		goto out_err;

```
### 旧版 API

以下用于启用和禁MSI MSI-X 中断的旧 API 应被避免使用
```
  pci_enable_msi()		/* deprecated */
  pci_disable_msi()		/* deprecated */
  pci_enable_msix_range()	/* deprecated */
  pci_enable_msix_exact()	/* deprecated */
  pci_disable_msix()		/* deprecated */

```
此外还有一些用于提供所支持MSI MSI-X 向量数量API：pci_msi_vec_count() pci_msix_vec_count()。一般来说应避免使用它们，而让 pci_alloc_irq_vectors() 自行限制向量
数量。如果你有使用向量数量的合理特殊用例，我们可能需要重新考虑这一决定，并添加一个能透明
处理 MSI MSI-X pci_nr_irq_vectors() 辅助函数
### 使用 MSI 时的注意事项

#### 自旋
大多数设备驱动都有一个每设备自旋锁，在中断处理程序中会被持有。对于基于引脚的中断或单MSI，不需要禁用中断（Linux 保证同一中断不会被重入）。如果设备使用多个中断，驱动必须持有锁的同时禁用中断。如果设备发送了不同的中断，驱动会试图递归获取自旋锁而陷入死锁。这死锁可以通过使用 spin_lock_irqsave() spin_lock_irq() 来避免，它们会禁用本地中断并获取
锁（参见 Documentation/kernel-hacking/locking.rst）
### 如何判断设备是否启用MSI/MSI-X

使用 'lspci -v'（以 root 身份）可能会显示一些带有“MSI”、“Message Signaled Interrupts”或
“MSI-X”能力的设备。这些能力中的每一个都有一'Enable' 标志，其后跟随”（已启用）”（已禁用）
## MSI 怪癖（quirks
已知有若PCI 芯片组或设备不支MSI。PCI 协议栈提供了三种禁用 MSI 的方式：

1. 全局禁用
2. 在某个特定桥后面的所有设备上禁用
3. 在单个设备上禁用

### 全局禁用 MSI

有些主机芯片组根本不能正确地支持 MSI。如果我们幸运的话，制造商知道这一点并ACPI FADT
表中做了标记。在这种情况下，Linux 会自动禁MSI。有些主板没有在表中包含这一信息，因我们必须自行检测它们。这些设备的完整列表可以drivers/pci/quirks.c 中的 quirk_disable_all_msi()
函数附近找到
如果你有一块在使用 MSI 时有问题的主板，可以在内核命令行上传pci=nomsi 来在所有设备上
禁用 MSI。为了你自己的利益，最好将问题报告linux-pci@vger.kernel.org，并附上完整'lspci -v'，以便我们将相应的怪癖加入内核
### 禁用桥下方的 MSI

有些 PCI 桥不能在总线之间正确地路MSI。在这种情况下，必须在该桥后面的所有设备上禁用
MSI銆。
有些桥允许你通过更改PCI 配置空间中的某些位来启用 MSI（尤其是 Hypertransport 芯片组，
nVidia nForce Serverworks HT2000）。与主机芯片组一样，Linux 大体上了解它们，并在
可能时自动启MSI。如果你有一Linux 未知的桥，可以用你所知的任何有效方法在配置空间中
启用 MSI，然后执行：

```
       echo 1 > /sys/bus/pci/devices/$bridge/msi_bus

```
其中 $bridge 是你已启用的桥的 PCI 地址（例0000:00:0e.0）
要禁MSI，则回显 0 而非 1。更改此值应谨慎进行，因为它可能破坏该桥下面所有设备的中断
处理
同样，请将任何需要特殊处理的桥通知 linux-pci@vger.kernel.org
### 在单个设备上禁用 MSI

已知某些设备MSI 实现有缺陷。通常这由各个设备驱动处理，但偶尔有必要通过怪癖来处理。有驱动提供了一个禁MSI 使用的选项。虽然这对驱动作者来说是一个方便的权宜之计，但这并非良实践，不应被效仿
### 查找设备MSI 被禁用的原因

从以上三节可以看出，有许多原因可能导致某个给定设备上未启MSI。你的第一步应该是仔细检你的 dmesg，以确定 MSI 是否为你的机器启用。你还应该检查你.config 以确认已启用
CONFIG_PCI_MSI銆。
然后lspci -t' 会给出设备之上的桥列表。读`/sys/bus/pci/devices/*/msi_bus` 会告诉你
MSI 是启用（1）还是禁用（0）。如果在属于 PCI 根与设备之间任意桥的 msi_bus 文件中发现了 0则说MSI 被禁用了
同样值得检查设备驱动是否支MSI。例如，它可能包含带PCI_IRQ_MSI PCI_IRQ_MSIX 标志pci_alloc_irq_vectors() 的调用
## 设备驱动 MSI(-X) API 列表

PCI/MSI 子系统为其导出的设备驱动 API 提供了一个专用的 C 文件——`drivers/pci/msi/api.c`导出的函数如下：

   :export:
