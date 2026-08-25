## 拆分页表锁（Split page table lock
最初，`mm->page_table_lock` 自旋锁保`mm_struct` 的所有页表。但这种方法由于锁竞争激烈，导致多线程应用的缺页异常可扩展性较差。为改善可扩展性，引入了拆分页表锁
采用拆分页表锁后，每个表都拥有独立的 per-table 锁来串行化对该表的访问。目前我们对 PTE PMD 表使用拆分锁。对更高级别表的访问`mm->page_table_lock` 保护
提供了一组用于锁解锁表以及其他访问器函数的辅助函数：

 - pte_offset_map_lock()
	映射 PTE 并获PTE 表锁，返回指PTE 的指针及PTE 表锁的指针，若没PTE 表则返回 NULL - pte_offset_map_ro_nolock()
	映射 PTE，返回指PTE 的指针及PTE 表锁的指针（未获取），若没有 PTE 表则返回 NULL - pte_offset_map_rw_nolock()
	映射 PTE，返回指PTE 的指针及PTE 表锁的指针（未获取）以及pmd 项的值，若没PTE 表则返回 NULL - pte_offset_map()
	映射 PTE，返回指PTE 的指针，若没PTE 表则返回 NULL - pte_unmap()
	取消映射 PTE 表；
 - pte_unmap_unlock()
	解锁并取消映PTE 表；
 - pte_alloc_map_lock()
	必要时分PTE 表并获取其锁，返回指PTE 的指针及其锁的指针，若分配失败则返回 NULL - pmd_lock()
	获取 PMD 表锁，返回指向已获取锁的指针 - pmd_lockptr()
	返回指向 PMD 表锁的指针；

PTE 表的拆分页表锁在编译期启用，条件`CONFIG_SPLIT_PTLOCK_CPUS`（通常4）小于或等于 `NR_CPUS`。如果拆分锁被禁用，所有表都由 `mm->page_table_lock` 保护
PMD 表的拆分页表锁在 PTE 表启用且架构支持时启用（见下文）
## Hugetlb 与拆分页表锁

Hugetlb 可以支持多种页大小。我们仅PMD 级别使用拆分锁，而不PUD 使用
Hugetlb 专用的辅助函数：

 - huge_pte_lock()
	PMD_SIZE 页获pmd 拆分锁，否则获取 `mm->page_table_lock` - huge_pte_lockptr()
	返回指向表锁的指针；

## 架构对拆分页表锁的支
不需要对 PTE 拆分页表锁做特殊启用：所需的全部工作由 `pagetable_pte_ctor()` `pagetable_dtor()` 完成，它们必须在 PTE 表分释放时调用
请确保架构没有使slab 分配器来分配页表：slab 使用 `page->slab_cache` 作为其页的字段。该字段`page->ptl` 共享存储空间
PMD 拆分锁只有在拥有多于两级页表时才有意义
PMD 拆分锁的启用需要在 PMD 表分配时调用 `pagetable_pmd_ctor()`，在释放时调`pagetable_dtor()`
分配通常发生`pmd_alloc_one()` 中，释放`pmd_free()` `pmd_free_tlb()` 中，但请确保覆盖所PMD 表分释放路径：例X86_PAE 会在 `pgd_alloc()` 中预分配若干 PMD
一切就绪后，可以设`CONFIG_ARCH_ENABLE_SPLIT_PMD_PTLOCK`
注意：`pagetable_pte_ctor()` `pagetable_pmd_ctor()` 可能失败——必须妥善处理
## page->ptl

`page->ptl` 用于访问拆分页表锁，其中 `page` 是包含该表的页对应的 `struct page`。它`page->private`（以union 中的其他若干字段）共享存储空间
为避免增`struct page` 的尺寸并获得最佳性能，我们使用了一个技巧：

 - 如果 `spinlock_t` 能放`long`，我们将 `page->ptr` 用作自旋锁，从而避免间接访问并节省一个缓存行 - 如果 `spinlock_t` 的大小大`long` 的大小，我们`page->ptl` 用作指向 `spinlock_t` 的指针并动态分配它。这允许在启`DEBUG_SPINLOCK` `DEBUG_LOCK_ALLOC` 时使用拆分锁，但间接访问会多消耗一个缓存行
`spinlock_t` PTE 表的 `pagetable_pte_ctor()` 中分配，PMD 表的 `pagetable_pmd_ctor()` 中分配
请绝直接访问 `page->ptl`——请使用相应的辅助函数