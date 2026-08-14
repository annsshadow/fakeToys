## 内存管理文档


这是一份帮助你理解 Linux 内存管理子系统的指南。如果你只是想了解如何简单地分配内存，请参阅 memory_allocation。关于控制与调优的指南，请参阅 [管理员指南 <../admin-guide/mm/index>](admin guide <../admin-guide/mm/index>)。

- [physical_memory](physical_memory)
- [page_tables](page_tables)
- [process_addrs](process_addrs)
- [bootmem](bootmem)
- [page_allocation](page_allocation)
- [vmalloc](vmalloc)
- [slab](slab)
- [highmem](highmem)
- [page_reclaim](page_reclaim)
- [swap](swap)
- [swap-table](swap-table)
- [page_cache](page_cache)
- [shmfs](shmfs)
- [oom](oom)

## 未分类文档


这是一组关于 Linux 内存管理（MM）子系统内部机制的未分类文档，详尽程度不一，既有简单的笔记和邮件列表回复，也有对数据结构和算法的详尽描述。理想情况下，这些内容应当被很好地整合到上述结构化文档中，或者在完成其使命后被删除。

- [active_mm](active_mm)
- [allocation-profiling](allocation-profiling)
- [arch_pgtable_helpers](arch_pgtable_helpers)
- [balance](balance)
- [damon/index](damon/index)
- [free_page_reporting](free_page_reporting)
- [hmm](hmm)
- [hwpoison](hwpoison)
- [hugetlbfs_reserv](hugetlbfs_reserv)
- [ksm](ksm)
- [memory-model](memory-model)
- [memfd_preservation](memfd_preservation)
- [mmu_notifier](mmu_notifier)
- [multigen_lru](multigen_lru)
- [numa](numa)
- [overcommit-accounting](overcommit-accounting)
- [page_migration](page_migration)
- [page_frags](page_frags)
- [page_owner](page_owner)
- [page_table_check](page_table_check)
- [remap_file_pages](remap_file_pages)
- [split_page_table_lock](split_page_table_lock)
- [transhuge](transhuge)
- [unevictable-lru](unevictable-lru)
- [vmalloced-kernel-stacks](vmalloced-kernel-stacks)
- [vmemmap_dedup](vmemmap_dedup)
- [zsmalloc](zsmalloc)
