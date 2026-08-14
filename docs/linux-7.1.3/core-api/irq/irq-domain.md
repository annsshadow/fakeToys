## irq_domain 中断号映射库

Linux 内核的当前设计使用一个单一的大号段，其中每个独立的中断源都被分配一个唯一的编号。当系统中只有一个中断控制器时，这很简单。但在拥有多个中断控制器的系统中，内核必须确保每个控制器都被分配到互不重叠的 Linux IRQ 编号。

被注册为独立 irqchip 的中断控制器的数量呈上升趋势。例如，GPIO 控制器等各类子驱动通过将其中断处理程序建模为 irqchip，从而避免重新实现与 IRQ 核心系统相同的回调机制。也就是说，实际上形成了级联的中断控制器。

因此在过去，IRQ 编号可以选择为与进入根中断控制器的硬件 IRQ 线相匹配（即实际向 CPU 触发中断线的组件）。而如今，这个编号仅仅是一个编号，与硬件中断编号没有任何关系。

出于这个原因，我们需要一种机制，将控制器本地的中断编号（称为硬件 IRQ，即 hwirq）与 Linux IRQ 编号区分开来。

`irq_alloc_desc*()` 和 `irq_free_desc*()` 系列 API 提供 IRQ 编号的分配，但它们并不提供将控制器本地 IRQ（hwirq）编号反向映射到 Linux IRQ 编号空间的支持。

irq_domain 库在 `irq_alloc_desc*()` API 之上增加了 hwirq 与 IRQ 编号之间的映射。相比于中断控制器驱动自行硬编码它们自己的反向映射方案，更推荐使用一个 irq_domain 来管理映射。

irq_domain 还实现了从抽象的 `struct irq_fwspec` 到 hwirq 编号的转换（目前支持 Device Tree、非 DT 固件节点、ACPI GSI 以及软件节点），并且可以轻松扩展以支持其他 IRQ 拓扑数据源。该实现无需任何额外的平台支持代码即可完成。

## irq_domain 的使用

`struct irq_domain` 可以定义为一个中断域控制器。也就是说，它负责处理给定中断域内硬件中断编号与虚拟中断编号之间的映射。该域结构通常由 PIC 代码针对某个 PIC 实例创建（不过如果一个域采用扁平编号模型，它也可以覆盖多个 PIC）。负责对已映射的 irq_desc 设置 irq_chip 的，正是域回调。

主机代码和数据结构使用 `fwnode_handle` 指针来标识域。在某些情况下，并且为了保持源代码兼容性，这个 fwnode 指针会被“升级”为一个 DT `device_node`。对于那些无法为中断控制器提供唯一标识符的固件基础设施，irq_domain 代码提供了一个 fwnode 分配器。

中断控制器驱动通过调用其中一个 `irq_domain_create_*()` 函数来创建并注册一个 `struct irq_domain`（每种映射方法都有不同的分配函数，稍后详述）。该函数在成功时会返回一个指向 `struct irq_domain` 的指针。调用者必须向分配函数提供一个 `struct irq_domain_ops` 指针。

在大多数情况下，irq_domain 初始为空，hwirq 与 IRQ 编号之间没有任何映射。通过调用 `irq_create_mapping()` 可以向 irq_domain 添加映射，该函数接受 irq_domain 和一个 hwirq 编号作为参数。如果某个 hwirq 的映射尚不存在，`irq_create_mapping()` 会分配一个新的 Linux irq_desc，将其与 hwirq 关联，并调用 `:c`irq_domain_ops.map()`` 回调。驱动可以在该回调中执行任何所需的硬件设置。

一旦建立映射，就可以通过多种方法检索或使用它：

- `irq_resolve_mapping()` 返回给定域和 hwirq 编号所对应的 irq_desc 结构指针，如果没有映射则返回 NULL。
- `irq_find_mapping()` 返回给定域和 hwirq 编号对应的 Linux IRQ 编号，如果没有映射则返回 0。
- `generic_handle_domain_irq()` 处理由某个域和 hwirq 编号描述的中断。

注意，irq_domain 查找必须发生在与 RCU 读侧临界区相兼容的上下文中。

`irq_create_mapping()` 函数必须在任何对 `irq_find_mapping()` 的调用之前**至少调用一次**，否则描述符将不会被分配。

如果驱动拥有 Linux IRQ 编号或 `irq_data` 指针，并且需要获知关联的 hwirq 编号（例如在 irq_chip 回调中），则可以直接从 `:c`irq_data.hwirq`` 获取。

## irq_domain 映射的类型

从 hwirq 到 Linux IRQ 的反向映射有若干可用机制，每种机制使用不同的分配函数。应当使用哪种反向映射类型取决于具体用例。下面描述了每种反向映射类型：

### 线性映射（Linear）

```
	irq_domain_create_linear()

```
线性反向映射维护一个由 hwirq 编号索引的固定大小表。当一个 hwirq 被映射时，会为该 hwirq 分配一个 irq_desc，并将 IRQ 编号存入表中。

当 hwirq 的最大数量固定且相对较小时（约 < 256），线性映射是一个不错的选择。这种映射的优点是对 IRQ 编号的查找时间为常数，并且 irq_desc 仅为正在使用的 IRQ 分配。缺点是该表必须大到足以容纳可能的最大 hwirq 编号。

大多数驱动都应该使用线性映射。

### 树映射（Tree）

```
	irq_domain_create_tree()

```
irq_domain 维护一个从 hwirq 编号到 Linux IRQ 的基数树（radix tree）映射。当一个 hwirq 被映射时，会分配一个 irq_desc，并以 hwirq 作为基数树的查找键。

如果 hwirq 编号可能非常大，树映射是一个不错的选择，因为它不需要分配一个与最大 hwirq 编号一样大的表。缺点是从 hwirq 到 IRQ 编号的查找依赖于表中条目的数量。

只有极少数驱动需要使用这种映射。

### 无映射（No Map）

```
	irq_domain_create_nomap()

```
无映射用于 hwirq 编号在硬件中可编程的情况。在这种情况下，最好将 Linux IRQ 编号直接编程到硬件本身，从而不需要映射。调用 `irq_create_direct_mapping()` 会分配一个 Linux IRQ 编号并调用 `.map()` 回调，以便驱动能够将 Linux IRQ 编号编程到硬件中。

大多数驱动无法使用这种映射，它现在受 `CONFIG_IRQ_DOMAIN_NOMAP` 选项限制。请避免引入该 API 的新使用者。

### 传统映射（Legacy）

```
	irq_domain_create_simple()
	irq_domain_create_legacy()

```
传统映射是针对已经为 hwirq 分配了一系列 irq_desc 的驱动的一种特例。当驱动无法立即转换为使用线性映射时使用它。例如，许多嵌入式系统板级支持文件使用一组 `#define` 来定义传递给 `struct device` 注册的 IRQ 编号。在这种情况下，Linux IRQ 编号无法动态分配，因此应当使用传统映射。

顾名思义，`*_legacy()` 函数已被弃用，仅为了便于支持古老的平台而存在。不应再新增使用者。当 `*_simple()` 函数的使用会导致传统行为时，同样不应再新增使用者。

传统映射假设已经为该控制器分配了一段连续的 IRQ 编号范围，并且 IRQ 编号可以通过向 hwirq 编号加上固定偏移量来计算，反之亦然。缺点是它要求中断控制器管理 IRQ 分配，并且要求为每个 hwirq 分配一个 irq_desc，即使该 hwirq 未被使用。

传统映射应当仅在必须支持固定 IRQ 映射时使用。例如，ISA 控制器会使用传统映射来映射 Linux IRQ 0-15，以便现有的 ISA 驱动获得正确的 IRQ 编号。

传统映射的大多数使用者应当使用 `irq_domain_create_simple()`，它仅在系统提供了 IRQ 范围时才使用传统域，否则使用线性域映射。该调用的语义是：如果指定了 IRQ 范围，则会按需为其分配描述符；如果没有指定范围，则回退到 `irq_domain_create_linear()`，这意味着**不会**分配任何 IRQ 描述符。

简单域的一个典型用例是：某个 irqchip 提供者同时支持动态和静态的 IRQ 分配。

为了避免出现“使用了线性域却没有分配任何描述符”的情况，非常重要的是要确保使用简单域的驱动在调用任何 `irq_find_mapping()` 之前先调用 `irq_create_mapping()`，因为后者在静态 IRQ 分配的场景下实际上也能工作。

### 层级 IRQ 域（Hierarchy IRQ Domain）

在某些架构上，从设备将中断投递到目标 CPU 可能涉及多个中断控制器。

```
  Device --> IOAPIC -> Interrupt remapping Controller -> Local APIC -> CPU

```
其中涉及三个中断控制器：

1) IOAPIC 控制器
2) 中断重映射控制器（Interrupt remapping controller）
3) Local APIC 控制器

为了支持这种硬件拓扑并使软件架构与硬件架构相匹配，会为每个中断控制器构建一个 irq_domain 数据结构，并将这些 irq_domain 组织成层级结构。在构建 irq_domain 层级时，最靠近设备的 irq_domain 为子节点，最靠近 CPU 的 irq_domain 为父节点。因此层级结构如下：

```
	CPU Vector irq_domain (root irq_domain to manage CPU vectors)
		^
		|
	Interrupt Remapping irq_domain (manage irq_remapping entries)
		^
		|
	IOAPIC irq_domain (manage IOAPIC delivery entries/pins)

```
使用层级 irq_domain 有四个主要接口：

1) `irq_domain_alloc_irqs()`：分配 IRQ 描述符以及用于投递这些中断的中断控制器相关资源。
2) `irq_domain_free_irqs()`：释放与这些中断关联的 IRQ 描述符以及中断控制器相关资源。
3) `irq_domain_activate_irq()`：激活中断控制器硬件以投递中断。
4) `irq_domain_deactivate_irq()`：停用中断控制器硬件以停止投递中断。

支持层级 irq_domain 需要以下条件：

1) `struct irq_domain` 中的 `:c`parent`` 字段用于维护 irq_domain 层级信息。
2) `struct irq_data` 中的 `:c`parent_data`` 字段用于构建与层级 irq_domain 相匹配的层级 irq_data。`irq_data` 用于存储 irq_domain 指针和硬件 irq 编号。
3) `struct irq_domain_ops` 中的 `:c`alloc()`、`:c`free()` 等回调，用于支持层级 irq_domain 操作。

在层级 irq_domain 和层级 irq_data 准备就绪后，会为每个中断控制器构建一个 irq_domain 结构，并为每个与某个 IRQ 关联的 irq_domain 分配一个 irq_data 结构。

要让中断控制器驱动支持层级 irq_domain，它需要：

1) 实现 `irq_domain_ops.alloc()` 和 `irq_domain_ops.free()`。
2) 可选地，实现 `irq_domain_ops.activate()` 和 `irq_domain_ops.deactivate()`。
3) 可选地，实现一个 irq_chip 来管理中断控制器硬件。
4) 不需要实现 `irq_domain_ops.map()` 和 `irq_domain_ops.unmap()`。它们在层级 irq_domain 中未被使用。

请注意，层级 irq_domain 绝非 x86 特有，它也被大量用于支持其他架构，例如 ARM、ARM64 等。

#### 堆叠 irq_chip（Stacked irq_chip）

现在，我们可以更进一步以支持堆叠的（层级的）irq_chip。也就是说，沿层级结构的每个 irq_data 都关联一个 irq_chip。子 irq_chip 可以通过自身实现所需的操作，也可以与其父 irq_chip 协作完成。

有了堆叠的 irq_chip，中断控制器驱动只需处理由自身管理的硬件，并在需要时向其父 irq_chip 请求服务。这样我们就能获得一个更加清晰的软件架构。

## 调试

IRQ 子系统的大部分内部信息都可以通过打开 `CONFIG_GENERIC_IRQ_DEBUGFS` 在 debugfs 中暴露。

## 提供的结构与公共函数

本章包含用于 IRQ 域的结构和导出的内核 API 函数的自动生成文档。

   :export:

## 提供的内部函数

本章包含内部函数的自动生成文档。

   :internal:
