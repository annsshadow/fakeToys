
## Linux Init (Early Boot)


Linux 的配置分为两个主要步骤：Early-Boot（早期启动）以及其余部分。

在早期启动期间，Linux 设置不可变资源（例如 numa 节点），而后续的操作包括驱动探测和内存热插拔等。在整个过程中，Linux 可能会读取 EFI 和 ACPI 信息，以配置设备的逻辑表示。

在 Linux 早期启动阶段（内核中带有 __init 修饰符的函数），系统会获取由 EFI/BIOS 创建的、资源（[ACPI tables <../platform/acpi>](ACPI tables <../platform/acpi>)），并将它们转换为内核可以消费的资源。


## BIOS, Build and Boot Options


在内核构建时有 4 个需要预先考虑的启动前选项，它们决定了 Linux 在早期启动期间如何管理内存。

- EFI_MEMORY_SP

  - BIOS/EFI 选项，决定内存是 SystemRAM 还是 Specific Purpose。Specific Purpose 内存将被推迟交给驱动管理——而不会立即作为系统 RAM 暴露。

- CONFIG_EFI_SOFT_RESERVE

  - Linux 构建配置选项，决定内核是否支持 Specific Purpose 内存。

- CONFIG_MHP_DEFAULT_ONLINE_TYPE

  - Linux 构建配置，决定转换为 dax 设备的 Specific Purpose 内存是否以及如何被管理（保留为 DAX，或作为 ZONE_NORMAL 或 ZONE_MOVABLE 中的 SystemRAM 上线）。

- nosoftreserve

  - Linux 内核启动选项，决定是否支持 Soft Reserve。与 CONFIG_EFI_SOFT_RESERVE 类似。

## Memory Map Creation


当内核解析 EFI 内存映射时，如果支持并检测到了 `Specific Purpose` 内存，它会将该区域单独划出为 `SOFT_RESERVED`。

如果 `EFI_MEMORY_SP=0`、`CONFIG_EFI_SOFT_RESERVE=n` 或 `nosoftreserve=y`，Linux 会将 CXL 设备内存区域默认作为 SystemRAM。这会把该内存暴露给内核页分配器中的 `ZONE_NORMAL`，使其可用于大多数分配（包括 `struct page` 和页表）。

如果设置了 `Specific Purpose` 且受支持，`CONFIG_MHP_DEFAULT_ONLINE_TYPE_*` 决定该内存是否默认上线（`_OFFLINE` 或 `_ONLINE_*`），以及如果上线，默认将其上线到哪个 zone（`_NORMAL` 或 `_MOVABLE`）。

如果放置在 `ZONE_MOVABLE`，该内存将不可用于大多数内核分配（例如 `struct page` 或页表）。根据系统的内存容量，这可能会对性能产生显著影响。


## NUMA Node Reservation


Linux 引用 :doc:`SRAT <../platform/acpi/srat>` 中定义的 proximity 域（`PXM`）来在 `acpi_numa_init` 中创建 NUMA 节点。通常，`PXM` 与 NUMA 节点 ID 之间存在 1:1 的关系。

SRAT 是定义 Proximity 域的唯一 ACPI 定义方式。Linux 最多选择将它们与 NUMA 节点 1:1 映射。[CEDT <../platform/acpi/cedt>](CEDT <../platform/acpi/cedt>) 增加了对 SPA 范围的描述，Linux 可能会将其映射到一个或多个 NUMA 节点。

如果 CFMWS 中存在但 SRAT 中没有的 CXL 范围，则会创建一个伪 `PXM`（自 v6.15 起）。未来，由于 proximity 域关联的模糊性，Linux 可能会拒绝 SRAT 未描述的 CFMWS。

需要注意的是，NUMA 节点的创建不能在运行时进行。所有可能的 NUMA 节点都在 `__init` 时（更具体地说，在 `mm_init` 期间）被识别。CEDT 和 SRAT 必须包含足够的 `PXM` 数据，以便 Linux 识别 NUMA 节点及其关联的内存区域。

相关代码位于：`linux/drivers/acpi/numa/srat.c`。

更多信息请参阅 [Example Platform Configurations <../platform/example-configs>](Example Platform Configurations <../platform/example-configs>)。

## Memory Tiers Creation


内存分层（memory tier）是按性能特征分组的 NUMA 节点集合。在 `__init` 期间，Linux 会使用包含标记为 `N_MEMORY` 的所有节点的默认内存分层来初始化系统。

默认情况下，`memory_tier_init` 在启动时对所有已上线内存的节点调用。`memory_tier_late_init` 在 late-init 期间对驱动配置阶段设置的节点调用。

节点只有在拥有**在线**内存时才会被标记为 `N_MEMORY`。

```

  /sys/devices/virtual/memory_tiering/memory_tierN/nodelist
  0-1

```
如果分组的节点在性能上存在明显差异，请检查 CXL 节点的 [HMAT <../platform/acpi/hmat>](HMAT <../platform/acpi/hmat>) 和 CDAT 信息。除非通过 `access_coordinates` 向 memory_tier 组件报告了 HMAT/CDAT 信息，否则所有节点默认都属于 DRAM 分层。

更多内容请参阅 :doc:`CXL access coordinates 文档 <../linux/access-coordinates>`。

## Contiguous Memory Allocation


连续内存分配器（CMA）能够在早期启动期间在 NUMA 节点上预留连续的物理内存区域。然而，CMA 无法预留内存：

```

  void __init hugetlb_cma_reserve(void) {
    if (!node_online(nid))
      /* 不允许预留 */
  }

```
这意味着，如果用户打算将 CXL 内存的管理推迟到驱动，则 CMA 不能用于保证大页分配。如果在早期启动期间将 CXL 内存作为 `ZONE_NORMAL` 中的 SystemRAM 启用，则可以使用 `cma_pernuma` 或 `numa_cma` 内核命令行参数为每个节点进行 CMA 预留。
