## 透明大页（Transparent Hugepage）支

本文档描述了透明大页（Transparent Hugepage，THP）支持的设计原则及其与内存管系统其他部分的交互
## 设计原则


- “优雅回退（graceful fallback）”：不了解透明大页mm 组件会回退到将大页 pmd 映射拆分pte 表，并在必要时拆分一个透明大页。因此这些组  可以继续在常规页或常pte 映射上工作
- 如果由于内存碎片导致大页分配失败  应优雅地分配常规页，并在同一 vma 中混合，而没有任何失败或显著延迟，且用户空间不会察觉到
- 如果某个任务退出并有更多大页变得可用（无论是在 buddy 中立即获得，还是通过 VM 获得），由常规页支撑  客户机物理内存应自动（通过 khugepaged）重定位到大页上
- 它不需要内存预留，进而在可能时随时使用大页（此处唯一可能的预留是 kernelcore=
  以避免不可移动页碎片化了所有内存，但此类调整并非透明大页支持所特有，它是适用于内核中所有动态高阶分配的通用特性）
## get_user_pages 涓?pin_user_pages


get_user_pages pin_user_pages 如果在大页上运行，将照常返回头页（head page）或尾页（tail page）（正如它们hugetlbfs 上所做的那样）。大多数 GUP 用户只关心页的实际物理地址及其临时固定（pinning），以便I/O
完成后释放，因此他们永远不会注意到该页是大页这一事实。但如果任何驱动打算对尾页的页结构动手脚（例如检page->mapping 与头页相关而非尾页相关的其他位），则应更新为跳过去检查头页。获取任何头/尾页上的引用将阻止该页被任何人拆分
   这些GUP API 而言并非新的约束，它们与适用hugetlbfs 的约束相同，因此任何能够处理 hugetlbfs GUP 的驱   也能在透明大页支撑的映射上正常工作
## 优雅回退


遍历页表但不了解大页 pmd 的代码，只需pmd pmd_offset 返回pmd 时调split_huge_pmd(vma, pmd, addr)。仅通过 grep 查找 "pmd_offset" 并在 pmd_offset 返回 pmd 缺失处添split_huge_pmd，就能轻松让代码变得透明大页感知。多亏了优雅回退设计，只需一行改动，你就能避免编数百乃至数千行复杂代码来让你的代码变得大页感知
如果你不是遍历页表，而是遇到了你的代码无法原生处理的物理大页，你可以通过调用 split_huge_page(page) 来拆分它例如，这正是 Linux VM 在尝试换出（swapout）该大页之前所做的。如果页被固定（pinned），split_huge_page() 可能失败你必须正确处理这一点
mremap.c 变得透明大页感知的单行示
```
	diff --git a/mm/mremap.c b/mm/mremap.c
	--- a/mm/mremap.c
	+++ b/mm/mremap.c
	@@ -41,6 +41,7 @@ static pmd_t *get_old_pmd(struct mm_stru
			return NULL;

		pmd = pmd_offset(pud, addr);
	+	split_huge_pmd(vma, pmd, addr);
		if (pmd_none_or_clear_bad(pmd))
			return NULL;
```
## 大页感知代码中的加锁


我们希望尽可能多的代码变得大页感知，因为调用 split_huge_page() split_huge_pmd() 是有代价的
要让页表遍历变得大页 pmd 感知，你只需pmd_offset 返回pmd 调用 pmd_trans_huge()。你必须以读（或写）模式持有
mmap_lock，以确保大页 pmd 不会khugepaged 从你脚下创建出来（khugepaged collapse_huge_page 除了 anon_vma 锁外，还会以写模式获mmap_lock）如果 pmd_trans_huge 返回 false，你就回退到旧代码路径。如pmd_trans_huge 返回 true，你必须获取页表锁（pmd_lock()）并重新运行 pmd_trans_huge。获取页表锁将阻止大pmd 在你脚下被转换为常规 pmd（split_huge_pmd 可以与页表遍历并行运行）。如果第二次 pmd_trans_huge 返回 false，你只需释放页表锁并像之前一样回退到旧代码。否则，你可以继续原生处理大pmd 和大页。完成后，释放页表锁
## 引用计数与透明大页


THP 上的引用计数与其他复合（compound）页的引用计数基本一致：

  - get_page()/put_page() GUP 操作 folio->_refcount
  - 尾页中的 ->_refcount 始终为零：get_page_unless_zero() 在尾页上永远不会成功
  - 整个 THP PMD 表项的映解除映射会递增/递减 folio->_entire_mapcount folio->_large_mapcount
    我们还维护两个用于跟MM 拥有者（MM ID 与对应的 mapcount）以及当前状态（“maybe mapped shared    “mapped exclusively”）的槽位
    CONFIG_PAGE_MAPCOUNT 下，_entire_mapcount -1 变到 0 或从 0 变到 -1 时，我们还通过 ENTIRELY_MAPPED 递增/递减 folio->_nr_pages_mapped
  - 使用 PTE 表项对单个页的映解除映射会递增/递减 folio->_large_mapcount
    我们还维护两个用于跟MM 拥有者（MM ID 与对应的 mapcount）以及当前状态（“maybe mapped shared    “mapped exclusively”）的槽位
    CONFIG_PAGE_MAPCOUNT 下，page->_mapcount -1 变到 0 或从 0 变到 -1 时，我们还递增/递减 page->_mapcount 以及 folio->_nr_pages_mapped，因为这计算了由 PTE 映射的页数量
split_huge_page 内部必须在从页结构中清除所PG_head/tail 位之前，将头页中的引用计数分配到尾页。对于由页表
表项获取的引用计数这很容易做到，但我们对如何分配任何其他固定（即来自 get_user_pages 的）缺乏足够信息。split_huge_page() 会拒绝任何拆分已被固定大页的请求：它期望页计数等于所有子mapcount 之和加一（split_huge_page 调用者必须持有对头页的引用）
split_huge_page 使用迁移（migration）表项来稳定匿名页的 page->_refcount page->_mapcount。文件页只是被解除映射
我们对物理内存扫描器也是安全的：扫描器获取页引用的唯一合法方式get_page_unless_zero()
所有尾页在 atomic_add() 之前都具有零->_refcount。这阻止了扫描器在此之前获取对尾页的引用。在 atomic_add() 之后，我们不再关->_refcount 的值。我们已经知道应该从头页中释放多少引用
对于头页，get_page_unless_zero() 会成功，我们并不介意。拆分后引用应去向何处是明确的：它将留在头页上
注意 split_huge_pmd() 在引用计数方面没有任何限制：pmd 可以在任何点拆分且永远不会失败
## 部分解除映射deferred_split_folio()（仅匿名 THP

解除 THP 的一部分映射（通过 munmap() 或其他方式）不会立即释放内存。相反，我们folio_remove_rmap_*() 中检测到 THP 的某个子页未被使用，
并在出现内存压力时将THP 排队等待拆分。拆分将释放未使用的子页
由于在可以检测部分解除映射的位置，锁上下文不允许立即拆分该页。由于在很多情况下，THP 跨越 VMA 边界时，部分解除映射发生exit(2) 期间，这也可能适得其反
deferred_split_folio() 函数用于folio 排队等待拆分。拆分本身将在我们通过 shrinker 接口遇到内存压力时发生
CONFIG_PAGE_MAPCOUNT 下，我们基于 folio->_nr_pages_mapped 可靠地检测部分映射
CONFIG_NO_PAGE_MAPCOUNT 下，我们基于 THP 中每页平mapcount 来检测部分映射：如果平均< 1，则一个匿THP 肯定是部分映射的。只要只有一个进程映射一THP，此检测就是可靠的。对于长时间运行的子进程，可能存在当前无法检测到部分映射的场景，未来可能需要在内存回收期间进行异步检测