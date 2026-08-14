
## 物理内存模型


系统中的物理内存可以通过不同方式寻址。最简单的情况是物理内存从地址 0 开始，
并跨越一段连续的地址范围直到最大地址。然而，这段范围中可能包含 CPU 无法访问
的小空洞。此外，也可能在完全不同的地址上存在若干段连续的范围。而且，别忘了
NUMA，在 NUMA 中不同的内存库挂载在不同的 CPU 上。

Linux 使用两种内存模型之一来抽象这种多样性：FLATMEM 和 SPARSEMEM。每个架构
定义它支持哪些内存模型、默认内存模型是什么，以及是否可能手动覆盖该默认值。

所有内存模型都使用排列在一个或多个数组中的 struct page 来跟踪物理页帧的状态。

无论选择哪种内存模型，物理页帧号（PFN）与对应的 `struct page` 之间都存在
一一映射。

每个内存模型都定义了 `pfn_to_page` 和 `page_to_pfn` 辅助函数，用于在 PFN 与
`struct page` 之间相互转换。

## FLATMEM


最简单的内存模型是 FLATMEM。该模型适用于具有连续或近乎连续的物理内存的非 NUMA
系统。

在 FLATMEM 内存模型中，有一个全局的 `mem_map` 数组来映射整个物理内存。对于
大多数架构，空洞在 `mem_map` 数组中也有对应的表项。对应于空洞的 `struct page`
对象从未被完全初始化。

为了分配 `mem_map` 数组，架构相关的 setup 代码应当调用 `free_area_init` 函数。
然而，在调用 `memblock_free_all` 将全部内存交给页分配器之前，该映射数组是
不可用的。

架构可以释放 `mem_map` 数组中未覆盖实际物理页的部分。在这种情况下，架构相关的
`pfn_valid` 实现应当把 `mem_map` 中的空洞考虑在内。

在 FLATMEM 下，PFN 与 `struct page` 之间的转换很直接：`PFN - ARCH_PFN_OFFSET`
是 `mem_map` 数组的索引。

`ARCH_PFN_OFFSET` 定义了物理内存起始地址不为 0 的系统的第一个页帧号。

## SPARSEMEM


SPARSEMEM 是 Linux 中通用性最强内存模型，也是唯一支持若干高级特性的内存模型，
例如物理内存的热插拔与热移除、非易失性内存设备的替代内存映射，以及大型系统的
内存映射延迟初始化。

SPARSEMEM 模型将物理内存呈现为一组 section 的集合。一个 section 由 struct
mem_section 表示，其中包含 `section_mem_map`，从逻辑上讲，它是指向 struct page
数组的指针。然而，它还存储了一些其它的“魔法”信息，以辅助 section 的管理。section
的大小和 section 的最大数量由每个支持 SPARSEMEM 的架构定义的 `SECTION_SIZE_BITS`
和 `MAX_PHYSMEM_BITS` 常量指定。虽然 `MAX_PHYSMEM_BITS` 是一个架构支持的物理
地址的实际宽度，但 `SECTION_SIZE_BITS` 是一个任意值。

section 的最大数量记为 `NR_MEM_SECTIONS`，定义为


   NR\_MEM\_SECTIONS = 2 ^ {(MAX\_PHYSMEM\_BITS - SECTION\_SIZE\_BITS)}

`mem_section` 对象排列在一个称为 `mem_sections` 的二维数组中。该数组的大小和
位置取决于 `CONFIG_SPARSEMEM_EXTREME` 以及 section 的最大可能数量：

- 当 `CONFIG_SPARSEMEM_EXTREME` 被禁用时，`mem_sections` 数组是静态的，并且
  具有 `NR_MEM_SECTIONS` 行。每行保存一个 `mem_section` 对象。
- 当 `CONFIG_SPARSEMEM_EXTREME` 被启用时，`mem_sections` 数组是动态分配的。
  每行包含 PAGE_SIZE 大小的 `mem_section` 对象，行数经过计算以容纳所有内存
  section。

在 SPARSEMEM 下，将 PFN 转换为对应的 `struct page` 有两种可能的方式——“classic
sparse”和“sparse vmemmap”。选择是在构建时做出的，由 `CONFIG_SPARSEMEM_VMEMMAP`
的值决定。

classic sparse 将页的 section 编号编码在 page->flags 中，并使用 PFN 的高位来
访问映射该页帧的 section。在一个 section 内部，PFN 是页数组的索引。

sparse vmemmap 使用虚拟映射的内存映射来优化 pfn_to_page 和 page_to_pfn 操作。
有一个全局的 `struct page *vmemmap` 指针，指向一个虚拟连续的 `struct page`
对象数组。PFN 是该数组的索引，`struct page` 相对 `vmemmap` 的偏移量就是该页的
PFN。

要使用 vmemmap，架构必须保留一段虚拟地址范围，用于映射包含内存映射的物理页，
并确保 `vmemmap` 指向该范围。此外，架构应实现 `vmemmap_populate` 方法，以
分配物理内存并为虚拟内存映射创建页表。如果架构对 vmemmap 映射没有任何特殊要求，
它可以使用通用内存管理提供的默认 `vmemmap_populate_basepages`。

虚拟映射的内存映射允许将持久内存设备的 `struct page` 对象存储在那些设备上预分配的
存储中。该存储由 struct vmem_altmap 表示，最终通过一长串函数调用传递给
vmemmap_populate()。vmemmap_populate() 的实现可以使用 `vmem_altmap` 以及
`vmemmap_alloc_block_buf` 辅助函数在持久内存设备上分配内存映射。

## ZONE_DEVICE


`ZONE_DEVICE` 机制建立在 `SPARSEMEM_VMEMMAP` 之上，为设备驱动识别的物理地址
范围提供 `struct page` 的 `mem_map` 服务。`ZONE_DEVICE` 的“设备”方面与以下
事实相关：这些地址范围的页对象永远不会被标记为在线，并且必须对该设备（而不仅仅是
页）持有引用，才能将内存保持为被固定使用状态。`ZONE_DEVICE` 通过
`devm_memremap_pages` 执行了足够的内存热插拔，以开启给定 PFN 范围的
`pfn_to_page`、`page_to_pfn` 和 `get_user_pages` 服务。由于页引用计数永远不会
降到 1 以下，该页永远不会被作为空闲内存跟踪，并且该页的 `struct list_head lru`
空间被重新利用，用于反向引用映射该内存的主机设备/驱动。

虽然 `SPARSEMEM` 将内存呈现为一组 section（可选地收集为内存块），但
`ZONE_DEVICE` 的用户需要更细的粒度来填充 `mem_map`。鉴于 `ZONE_DEVICE` 内存
永远不会被标记为在线，因此它随后永远不会通过 sysfs 内存热插拔 API 在内存块边界
暴露其内存范围。该实现依赖这种缺乏用户空间 API 约束的特性，允许向
`arch_add_memory`（内存热插拔的上半部分）指定子 section 大小的内存范围。子
section 支持允许以 2MB 作为 `devm_memremap_pages` 的跨架构通用对齐粒度。

`ZONE_DEVICE` 的用户有：

- pmem：将平台持久内存映射为通过 DAX 映射用作直接 I/O 目标。

- hmm：用 `->page_fault()` 和 `->folio_free()` 事件回调扩展 `ZONE_DEVICE`，
  以允许设备驱动协调与设备内存（通常是 GPU 内存）相关的内存管理事件。参见
  Documentation/mm/hmm.rst。

- p2pdma：创建 `struct page` 对象，以允许 PCI/-E 拓扑中的对等设备在彼此之间
  协调直接 DMA 操作，即绕过主机内存。
