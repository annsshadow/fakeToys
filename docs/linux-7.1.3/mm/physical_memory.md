
## 物理内存


Linux 可用于范围广泛的架构，因此需要一种与架构无关的抽象来表示物理内存。
本章描述了在运行中的系统中用于管理物理内存的结构。

内存管理中最主要的概念是 `Non-Uniform Memory Access (NUMA)
<https://en.wikipedia.org/wiki/Non-uniform_memory_access>`_。
在多核与多路插槽的机器上，内存可能被划分成多个存储块（bank），根据与
处理器的“距离”远近，访问它们所需的代价各不相同。例如，可能为每个 CPU
分配一个内存块，或者在外围设备附近有一块非常适合 DMA 的内存。

每个存储块被称为一个节点（node），即便架构是 UMA，该概念在 Linux 下也由
`struct pglist_data` 表示。该结构总是以其 typedef `pg_data_t` 被引用。
特定节点的 `pg_data_t` 结构可通过 `NODE_DATA(nid)` 宏来引用，其中 `nid`
是该节点的 ID。

对于 NUMA 架构，节点结构由架构相关的代码在启动早期分配。通常，这些结构
分配在它们所表示的那个内存块本地。对于 UMA 架构，只会使用一个名为
`contig_page_data` 的静态 `pg_data_t` 结构。节点将在 Section Nodes <nodes>
中进一步讨论。

整个物理地址空间被划分成一个或多个称为 zone（区）的块，它们表示内存中的
范围。这些范围通常由访问物理内存的架构约束决定。一个节点内对应于某个特定
zone 的内存范围由 `struct zone` 描述。每个 zone 具有下面描述的类型之一。

- `ZONE_DMA` 与 `ZONE_DMA32` 历史上表示适合由无法访问全部可寻址内存的
  外围设备进行 DMA 的内存。多年来已经有了更好、更健壮的接口来获取满足
  DMA 特定要求的内存（Documentation/core-api/dma-api.rst），但 `ZONE_DMA`
  和 `ZONE_DMA32` 仍然表示在如何被访问上受限的内存范围。依据架构的不同，
  这两种 zone 类型之一、甚至两者都可以在构建时通过 `CONFIG_ZONE_DMA` 和
  `CONFIG_ZONE_DMA32` 配置选项禁用。某些 64 位平台可能需要两个 zone，因为
  它们支持具有不同 DMA 寻址限制的外围设备。

- `ZONE_NORMAL` 用于内核始终可以访问的普通内存。如果 DMA 设备支持传输到
  所有可寻址内存，则可在该 zone 的页上执行 DMA 操作。`ZONE_NORMAL` 始终
  启用。

- `ZONE_HIGHMEM` 是物理内存中未被内核页表永久映射所覆盖的部分。该 zone 中
  的内存只能通过临时映射被内核访问。该 zone 仅在某些 32 位架构上可用，并
  通过 `CONFIG_HIGHMEM` 启用。

- `ZONE_MOVABLE` 用于可正常访问的内存，就像 `ZONE_NORMAL` 一样。不同之处在于
  `ZONE_MOVABLE` 中大多数页的内容是可移动的。这意味着虽然这些页的虚拟地址
  不变，但其内容可能在不同物理页之间移动。通常 `ZONE_MOVABLE` 是在内存热插拔
  期间填充的，但也可以在启动时通过 `kernelcore`、`movablecore` 和
  `movable_node` 这几个内核命令行参数之一来填充。更多细节参见
  Documentation/mm/page_migration.rst 与
  Documentation/admin-guide/mm/memory-hotplug.rst。

- `ZONE_DEVICE` 表示驻留在设备（如 PMEM 和 GPU）上的内存。它与 RAM zone 类型
  具有不同的特性，其存在是为了给设备驱动所标识的物理地址范围提供 struct page
  <Pages> 与内存映射（memory map）服务。`ZONE_DEVICE` 由配置选项
  `CONFIG_ZONE_DEVICE` 启用。

需要注意，许多内核操作只能使用 `ZONE_NORMAL` 进行，因此它是性能最关键的
zone。zone 将在 Section Zones <zones> 中进一步讨论。

节点与 zone 范围之间的关系由固件报告的物理内存映射、内存寻址的架构约束以及
内核命令行中的某些参数决定。

例如，在具有 2 Gbytes RAM 的 x86 UMA 机器上运行 32 位内核时，整个内存将位于
节点 0，并会有三个 zone：`ZONE_DMA`、
```

  0                                                            2G
  +-------------------------------------------------------------+
  |                            node 0                           |
  +-------------------------------------------------------------+

  0         16M                    896M                        2G
  +----------+-----------------------+--------------------------+
  | ZONE_DMA |      ZONE_NORMAL      |       ZONE_HIGHMEM       |
  +----------+-----------------------+--------------------------+


```
使用禁用了 `ZONE_DMA`、启用了 `ZONE_DMA32` 的内核，并在具有 16 Gbytes RAM、
均匀分布于两个节点的 arm64 机器上以 `movablecore=80%` 参数引导时，节点 0 上
将会有 `ZONE_DMA32`、`ZONE_NORMAL` 和 `ZONE_MOVABLE`，而节点 1 上会有
`ZONE_NORMAL` 和
```


  1G                                9G                         17G
  +--------------------------------+ +--------------------------+
  |              node 0            | |          node 1          |
  +--------------------------------+ +--------------------------+

  1G       4G        4200M          9G          9320M          17G
  +---------+----------+-----------+ +------------+-------------+
  |  DMA32  |  NORMAL  |  MOVABLE  | |   NORMAL   |   MOVABLE   |
  +---------+----------+-----------+ +------------+-------------+


```
内存块可能属于交错（interleaving）的节点。在下面这个例子中，一台 x86 机器有
16 Gbytes RAM，分布在 4 个内存块中，偶数块属于节点 0
```


  0              4G              8G             12G            16G
  +-------------+ +-------------+ +-------------+ +-------------+
  |    node 0   | |    node 1   | |    node 0   | |    node 1   |
  +-------------+ +-------------+ +-------------+ +-------------+

  0   16M      4G
  +-----+-------+ +-------------+ +-------------+ +-------------+
  | DMA | DMA32 | |    NORMAL   | |    NORMAL   | |    NORMAL   |
  +-----+-------+ +-------------+ +-------------+ +-------------+

```
在这种情况下，节点 0 将从 0 跨到 12 Gbytes，节点 1 将从 4 跨到 16 Gbytes。


## 节点


如前所述，内存中的每个节点由一个 `pg_data_t` 描述，它是 `struct pglist_data`
的 typedef。在分配一个页时，默认情况下 Linux 使用节点本地（node-local）分配
策略，从距离正在运行的 CPU 最近的节点分配内存。由于进程往往运行在相同的 CPU
上，当前节点的内存很可能被使用。分配策略可由用户控制，如
Documentation/admin-guide/mm/numa_memory_policy.rst 中所述。

大多数 NUMA 架构维护一个指向节点结构的指针数组。实际的结构在启动早期由
架构相关代码解析固件报告的物理内存映射时分配。节点初始化的主体部分稍后在
启动流程中由 free_area_init() 函数完成，稍后将在 Section Initialization
<initialization> 中描述。

除了节点结构，内核还维护一个称为 `node_states` 的 `nodemask_t` 位掩码数组。
该数组中的每个位掩码表示一组具有 `enum node_states` 所定义特定属性的节点：

`N_POSSIBLE`
  该节点可能在某个时刻上线（online）。
`N_ONLINE`
  该节点已上线。
`N_NORMAL_MEMORY`
  该节点具有常规内存。
`N_HIGH_MEMORY`
  该节点具有常规或高端内存。当 `CONFIG_HIGHMEM` 被禁用时，与
  `N_NORMAL_MEMORY` 别名为同一含义。
`N_MEMORY`
  该节点具有内存（常规、高端、可移动）。
`N_CPU`
  该节点具有一个或多个 CPU。
`N_GENERIC_INITIATOR`
  该节点具有一个或多个 Generic Initiator。

对于具有上述属性的每个节点，会在 `node_states[<property>]` 位掩码中设置对应于
该节点 ID 的位。

```

  node_states[N_POSSIBLE]
  node_states[N_ONLINE]
  node_states[N_NORMAL_MEMORY]
  node_states[N_HIGH_MEMORY]
  node_states[N_MEMORY]
  node_states[N_CPU]

```
关于 nodemask 可执行的各种操作，请参考 `include/linux/nodemask.h`。

除此之外，nodemask 还用于提供节点遍历的宏，即 `for_each_node()` 与
`for_each_online_node()`。

```

	for_each_online_node(nid) {
		pg_data_t *pgdat = NODE_DATA(nid);

		foo(pgdat);
	}

```
### 节点结构


节点结构 `struct pglist_data` 声明于 `include/linux/mmzone.h`。这里我们简要
描述该结构的字段：

#### 通用


`node_zones`
  该节点的各个 zone。并非所有 zone 都可能被填充，但这是完整的列表。它被该节点
  的 node_zonelists 以及其他节点的 node_zonelists 引用。

`node_zonelists`
  所有节点中所有 zone 的列表。该列表定义了优先从中分配的 zone 顺序。
  `node_zonelists` 由 `mm/page_alloc.c` 中的 `build_zonelists()` 在核心内存
  管理结构初始化期间建立。

`nr_zones`
  该节点中已填充 zone 的数量。

`node_mem_map`
  对于使用 FLATMEM 内存模型的 UMA 系统，0 号节点的 `node_mem_map` 是表示每个
  物理页帧（frame）的 struct page 数组。

`node_page_ext`
  对于使用 FLATMEM 内存模型的 UMA 系统，0 号节点的 `node_page_ext` 是 struct
  page 扩展项的数组。仅在启用了 `CONFIG_PAGE_EXTENSION` 的内核中可用。

`node_start_pfn`
  该节点中起始页帧的页帧号（page frame number）。

`node_present_pages`
  该节点中存在的物理页总数。

`node_spanned_pages`
  物理页范围的总大小，包含空洞（hole）。

`node_size_lock`
  保护用于定义节点范围（extent）字段的锁。仅当至少启用了 `CONFIG_MEMORY_HOTPLUG`
  或 `CONFIG_DEFERRED_STRUCT_PAGE_INIT` 其中之一时定义。`pgdat_resize_lock()`
  与 `pgdat_resize_unlock()` 被提供用于操作 `node_size_lock`，而无需检查
  `CONFIG_MEMORY_HOTPLUG` 或 `CONFIG_DEFERRED_STRUCT_PAGE_INIT`。

`node_id`
  节点的节点 ID（NID），从 0 开始。

`totalreserve_pages`
  这是每节点保留的、对用户空间分配不可用的页。

`first_deferred_pfn`
  如果在大型机器上内存初始化被延迟，则这是需要被初始化的第一个 PFN。仅当
  启用了 `CONFIG_DEFERRED_STRUCT_PAGE_INIT` 时定义。

`deferred_split_queue`
  每节点的巨页（huge page）队列，这些巨页的拆分被延迟。仅当启用了
  `CONFIG_TRANSPARENT_HUGEPAGE` 时定义。

`__lruvec`
  每节点的 lruvec，持有 LRU 链表及相关的参数。仅在内存 cgroup 被禁用时使用。
  不应直接访问它，而应改用 `mem_cgroup_lruvec()` 来查找 lruvec。

#### 回收控制


另见 Documentation/mm/page_reclaim.rst。

`kswapd`
  每节点的 kswapd 内核线程实例。

`kswapd_wait`、`pfmemalloc_wait`、`reclaim_wait`
  用于同步内存回收任务的 workqueue。

`nr_writeback_throttled`
  因等待脏页回写而被节流（throttled）的任务数量。

`nr_reclaim_start`
  回收被节流以等待回写期间写入的页数。

`kswapd_order`
  控制 kswapd 尝试回收的阶（order）。

`kswapd_highest_zoneidx`
  由 kswapd 回收的最高 zone 索引。

`kswapd_failures`
  kswapd 无法回收任何页的运行次数。

`min_unmapped_pages`
  不可被回收的未映射文件支撑（file backed）页的最小数量。由
  `vm.min_unmapped_ratio` sysctl 决定。仅当启用了 `CONFIG_NUMA` 时定义。

`min_slab_pages`
  不可被回收的 SLAB 页的最小数量。由 `vm.min_slab_ratio` sysctl 决定。仅当
  启用了 `CONFIG_NUMA` 时定义。

`flags`
  控制回收行为的标志。

#### 规整（Compaction）控制


`kcompactd_max_order`
  kcompactd 应尝试达到的页阶（page order）。

`kcompactd_highest_zoneidx`
  由 kcompactd 进行规整的最高 zone 索引。

`kcompactd_wait`
  用于同步内存规整任务的 workqueue。

`kcompactd`
  每节点的 kcompactd 内核线程实例。

`proactive_compact_trigger`
  决定是否启用主动规整（proactive compaction）。由 `vm.compaction_proactiveness`
  sysctl 控制。

#### 统计


`per_cpu_nodestats`
  该节点的每 CPU VM 统计信息。

`vm_stat`
  该节点的 VM 统计信息。


## 区（Zones）


如前所述，内存中的每个 zone 由 `struct zone` 描述，它是其所属节点的
`node_zones` 数组的一个元素。`struct zone` 是页分配器（page allocator）的核心
数据结构。一个 zone 表示一段物理内存范围，并可能包含空洞。

页分配器使用由内存分配指定的 GFP 标志（参见 mm-api-gfp-flags）来确定该内存
分配可从节点中的哪个最高 zone 分配内存。页分配器首先从该 zone 分配内存；如果
页分配器无法从该 zone 分配出所请求数量的内存，它将从节点中下一个较低的 zone
分配，此过程一直向上进行，直到并包括最低的 zone。例如，如果一个节点包含
`ZONE_DMA32`、`ZONE_NORMAL` 和 `ZONE_MOVABLE`，且某次内存分配的最高 zone 是
`ZONE_MOVABLE`，则页分配器从中分配内存的 zone 顺序为 `ZONE_MOVABLE` >
`ZONE_NORMAL` > `ZONE_DMA32`。

在运行时，zone 中的空闲页位于每 CPU 页集（Per-CPU Pagesets，PCP）或该 zone 的
空闲区域（free areas）中。每 CPU 页集是内核内存管理系统中的一项关键机制。通过
在每个 CPU 上本地处理最频繁的分配与释放，每 CPU 页集提升了性能与可扩展性，
尤其是在拥有众多核心的系统上。内核中的页分配器采用两步策略进行内存分配，先
从每 CPU 页集开始，再回退（fall back）到伙伴分配器（buddy allocator）。页在每
CPU 页集与全局空闲区域（由伙伴分配器管理）之间以批处理方式转移。这最小化了对
全局伙伴分配器频繁交互的开销。

架构相关代码调用 free_area_init() 来初始化 zone。

### 区结构


zone 结构 `struct zone` 定义于 `include/linux/mmzone.h`。这里我们简要描述该
结构的字段：

#### 通用


`_watermark`
  该 zone 的水位线（watermark）。当一个 zone 中空闲页的数量低于 min 水位线时，
  会忽略 boosting，一次分配可能触发直接回收（direct reclaim）与直接规整
  （direct compaction），它也用于限制直接回收。当一个 zone 中空闲页的数量低于
  low 水位线时，会唤醒 kswapd。当一个 zone 中空闲页的数量高于 high 水位线时，
  若 `sysctl_numa_balancing_mode` 的 `NUMA_BALANCING_MEMORY_TIERING` 位未设置，
  kswapd 会停止回收（一个 zone 达到平衡）。promo 水位线用于内存分层（memory
  tiering）与 NUMA 平衡。当一个 zone 中空闲页的数量高于 promo 水位线时，若
  `sysctl_numa_balancing_mode` 的 `NUMA_BALANCING_MEMORY_TIERING` 位已设置，
  kswapd 会停止回收。水位线由 `__setup_per_zone_wmarks()` 设置。min 水位线根据
  `vm.min_free_kbytes` sysctl 计算。另外三个水位线根据两个水位线之间的距离设置。
  距离本身的计算会考虑 `vm.watermark_scale_factor` sysctl。

`watermark_boost`
  用于提升水位线的页数，以增加回收压力来降低将来发生回退（fallback）的可能性，
  并立即唤醒 kswapd，因为该节点整体上可能已平衡，kswapd 不会自然唤醒。

`nr_reserved_highatomic`
  为高阶原子（high-order atomic）分配所保留的页数。

`nr_free_highatomic`
  已保留的 highatomic pageblock 中的空闲页数。

`lowmem_reserve`
  该 zone 中为内存分配所保留的内存数量数组。例如，如果某次内存分配可从中分配
  内存的最高 zone 是 `ZONE_MOVABLE`，则在尝试从该 zone 分配内存时，为该分配所
  保留的内存量即为 `lowmem_reserve[ZONE_MOVABLE]`。这是页分配器用来防止本可使用
  `highmem` 的分配占用过多 `lowmem` 的一种机制。对于 `highmem` 机器上某些专门的
  工作负载，内核允许进程内存从 `lowmem` zone 分配是危险的。这是因为那样的内存随后
  可能被 `mlock()` 系统调用钉住（pin），或者因交换空间不可用而无法回收。
  `vm.lowmem_reserve_ratio` sysctl 决定了内核在捍卫这些较低 zone 时有多激进。该
  数组在运行时由 `setup_per_zone_lowmem_reserve()` 重新计算，前提是
  `vm.lowmem_reserve_ratio` sysctl 发生变化。

`node`
  该 zone 所属节点的索引。仅当启用了 `CONFIG_NUMA` 时可用，因为 UMA 系统中只有
  一个 zone。

`zone_pgdat`
  指向该 zone 所属节点的 `struct pglist_data` 的指针。

`per_cpu_pageset`
  指向由 `setup_zone_pageset()` 分配并初始化的每 CPU 页集（PCP）的指针。通过
  在每个 CPU 上本地处理最频繁的分配与释放，PCP 在拥有众多核心的系统上提升了性能
  与可扩展性。

`pageset_high_min`
  复制到每 CPU 页集的 `high_min`，以便更快访问。

`pageset_high_max`
  复制到每 CPU 页集的 `high_max`，以便更快访问。

`pageset_batch`
  复制到每 CPU 页集的 `batch`，以便更快访问。每 CPU 页集的 `batch`、`high_min`
  和 `high_max` 用于在单次持有锁的情况下计算每 CPU 页集从伙伴分配器获取的元素
  数量，以提升效率。它们还用于在页释放过程中决定是否将页返回给伙伴分配器。

`pageblock_flags`
  指向该 zone 中 pageblock 标志的指针（标志列表见
  `include/linux/pageblock-flags.h`）。内存在 `setup_usemap()` 中分配。每个
  pageblock 占用 `NR_PAGEBLOCK_BITS` 位。仅当启用了 `CONFIG_FLATMEM` 时定义。
  当启用了 `CONFIG_SPARSEMEM` 时，标志存储在 `mem_section` 中。

`zone_start_pfn`
  zone 的起始 pfn。由 `calculate_node_totalpages()` 初始化。

`managed_pages`
  由伙伴系统管理的存在页（present pages），计算方式为：`managed_pages` =
  `present_pages` - `reserved_pages`，其中 `reserved_pages` 包含由 memblock
  分配器分配的页。它应由页分配器与 vm 扫描器用来计算各种水位线与阈值。它使用
  `atomic_long_xxx()` 函数访问。它在 `free_area_init_core()` 中初始化，然后在
  memblock 分配器将页释放回伙伴系统时重新初始化。

`spanned_pages`
  该 zone 所跨越的总页数，包含空洞，计算方式为：`spanned_pages` = `zone_end_pfn`
  - `zone_start_pfn`。由 `calculate_node_totalpages()` 初始化。

`present_pages`
  该 zone 中存在的物理页，计算方式为：`present_pages` = `spanned_pages` -
  `absent_pages`（空洞中的页）。它可被内存热插拔或内存电源管理逻辑通过检查
  （`present_pages` - `managed_pages`）来推算未受管理的页。运行时对
  `present_pages` 的写访问应由 `mem_hotplug_begin/done()` 保护。任何无法容忍
  `present_pages` 漂移的读取者应使用 `get_online_mems()` 获取稳定的值。它由
  `calculate_node_totalpages()` 初始化。

`present_early_pages`
  该 zone 中位于自早期启动即可用内存上的存在页，不包括热插拔内存。仅当启用了
  `CONFIG_MEMORY_HOTPLUG` 时定义，并由 `calculate_node_totalpages()` 初始化。

`cma_pages`
  为 CMA 使用所保留的页。当这些页未被用于 CMA 时，它们的行为类似于
  `ZONE_MOVABLE`。仅当启用了 `CONFIG_CMA` 时定义。

`name`
  zone 的名称。它是指向 `zone_names` 数组对应元素的指针。

`nr_isolate_pageblock`
  已隔离的 pageblock 数量。它用于解决由于竞争地获取 pageblock 的迁移类型
  （migratetype）而导致的空闲页计数不正确的问题。由 `zone->lock` 保护。仅当
  启用了 `CONFIG_MEMORY_ISOLATION` 时定义。

`span_seqlock`
  保护 `zone_start_pfn` 与 `spanned_pages` 的顺序锁（seqlock）。它是顺序锁，因为
  必须在 `zone->lock` 之外读取，并且是在主分配器路径中进行的。不过，该顺序锁的
  写入相当不频繁。仅当启用了 `CONFIG_MEMORY_HOTPLUG` 时定义。

`initialized`
  指示该 zone 是否已初始化的标志。在启动期间由 `init_currently_empty_zone()`
  设置。

`free_area`
  空闲区域数组，其中每个元素对应于一个特定的阶（order），即 2 的幂。伙伴分配器
  使用该结构来高效管理空闲内存。分配时，它尝试寻找最小的足够块；如果最小的足够
  块大于请求的大小，它将被递归地拆分成更小的下一级块，直到达到所需大小。当释放
  一页时，它可能与其伙伴（buddy）合并形成一个更大的块。它由 `zone_init_free_lists()`
  初始化。

`unaccepted_pages`
  待接受（accept）的页列表。列表中的所有页都是 `MAX_PAGE_ORDER`。仅当启用了
  `CONFIG_UNACCEPTED_MEMORY` 时定义。

`flags`
  zone 的标志。最低三位被使用，由 `enum zone_flags` 定义。`ZONE_BOOSTED_WATERMARK`
  （位 0）：zone 最近提升了水位线。在唤醒 kswapd 时清除。`ZONE_RECLAIM_ACTIVE`
  （位 1）：kswapd 可能正在扫描该 zone。`ZONE_BELOW_HIGH`（位 2）：zone 低于 high
  水位线。

`lock`
  保护页分配器特定于该 zone 的内部数据结构的主要锁，尤其保护 `free_area`。

`percpu_drift_mark`
  当空闲页数低于此点时，在读取空闲页数量时会采取额外步骤，以避免每 CPU 计数器
  漂移导致水位线被突破。它在 `refresh_zone_stat_thresholds()` 中更新。

#### 规整控制


`compact_cached_free_pfn`
  规整空闲扫描器（free scanner）在下一次扫描中应开始的位置 PFN。

`compact_cached_migrate_pfn`
  规整迁移扫描器（migration scanner）在下一次扫描中应开始的位置 PFN。该数组有两个
  元素：第一个用于 `MIGRATE_ASYNC` 模式，另一个用于 `MIGRATE_SYNC` 模式。

`compact_init_migrate_pfn`
  初始迁移 PFN，在启动时被初始化为 0，在完整规整结束后被初始化为该 zone 中具有
  可迁移页的第一个 pageblock。它用于检查一次扫描是否为整个 zone 的扫描。

`compact_init_free_pfn`
  初始空闲 PFN，在启动时被初始化为 0，并被初始化为该 zone 中具有空闲
  `MIGRATE_MOVABLE` 页的最后一个 pageblock。它用于检查是否为一次扫描的起点。

`compact_considered`
  自上次失败以来已尝试的规整次数。当一次规整未能成功分配出页时，它在
  `defer_compaction()` 中被重置。当一次规整应被跳过时，它在 `compaction_deferred()`
  中加 1。`compaction_deferred()` 在 `compact_zone()` 被调用之前调用，
  `compaction_defer_reset()` 在 `compact_zone()` 返回 `COMPACT_SUCCESS` 时调用，
  `defer_compaction()` 在 `compact_zone()` 返回 `COMPACT_PARTIAL_SKIPPED` 或
  `COMPACT_COMPLETE` 时调用。

`compact_defer_shift`
  在再次尝试之前被跳过的规整次数为 `1<<compact_defer_shift`。它在 `defer_compaction()`
  中加 1。它在 `compaction_defer_reset()` 中，当一次直接规整成功分配出页时被重置。
  其最大值为 `COMPACT_MAX_DEFER_SHIFT`。

`compact_order_failed`
  最小的规整失败阶。它在一次规整成功时于 `compaction_defer_reset()` 中设置，并在
  一次规整未能成功分配出页时于 `defer_compaction()` 中设置。

`compact_blockskip_flush`
  当规整迁移扫描器与空闲扫描器相遇时设为 true，这意味着 `PB_compact_skip` 位应被
  清除。

`contiguous`
  当该 zone 是连续的（换言之，无空洞）时设为 true。

#### 统计


`vm_stat`
  该 zone 的 VM 统计信息。所追踪的项目由 `enum zone_stat_item` 定义。

`vm_numa_event`
  该 zone 的 VM NUMA 事件统计信息。所追踪的项目由 `enum numa_stat_item` 定义。

`per_cpu_zonestats`
  该 zone 的每 CPU VM 统计信息。它按每 CPU 方式记录 VM 统计信息与 VM NUMA 事件
  统计信息。它减少了对该 zone 全局 `vm_stat` 与 `vm_numa_event` 字段的更新，以提升
  性能。


## 页（Pages）



   本节尚未完成。请列出并描述相应的字段。


## 大页（Folios）



   本节尚未完成。请列出并描述相应的字段。


## 初始化（Initialization）



   本节尚未完成。请列出并描述相应的字段。
