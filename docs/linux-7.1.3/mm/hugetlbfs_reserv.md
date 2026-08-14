## Hugetlbfs 预留


## 概述


Documentation/admin-guide/mm/hugetlbpage.rst 中描述的巨页（huge pages）通常
为应用程序使用而预分配。如果这些巨页对应的 VMA 指示要使用巨页，则它们会在
缺页（page fault）时实例化到任务（task）的地址空间中。如果在缺页时不存在
巨页，任务会收到 SIGBUS 信号并常常以不愉快的方式终止。在加入巨页支持后不久，
人们确定最好在 mmap() 时就检测出巨页的短缺。其思路是，如果没有足够的巨页
来覆盖该映射，mmap() 就会失败。这最初是通过在 mmap() 时做一次简单检查，
判断是否有足够的空闲巨页来覆盖该映射来完成的。与内核中大多数事物一样，
代码随着时间不断演进。不过，其基本思路是在 mmap() 时“预留”（reserve）巨页，
以确保该映射中的缺页能够获取到巨页。下面的描述试图说明在 v4.10 内核中
巨页预留处理是如何进行的。


## 读者对象


本描述主要面向正在修改 hugetlbfs 代码的核开发人员（kernel developers）。


## 数据结构


resv_huge_pages
	这是一个全局的（每个 hstate）已预留巨页计数。已预留的巨页
	仅对预留它们的任务可用。因此，通常可用的巨页数量计算为
	(`free_huge_pages - resv_huge_pages`)。
Reserve Map
```

		struct resv_map {
			struct kref refs;
			spinlock_t lock;
			struct list_head regions;
			long adds_in_progress;
			struct list_head region_cache;
			long region_cache_count;
		};

	There is one reserve map for each huge page mapping in the system.
	The regions list within the resv_map describes the regions within
	the mapping.  A region is described as::

		struct file_region {
			struct list_head link;
			long from;
			long to;
		};

	The 'from' and 'to' fields of the file region structure are huge page
	indices into the mapping.  Depending on the type of mapping, a
	region in the reserv_map may indicate reservations exist for the
	range, or reservations do not exist.
```
Flags for MAP_PRIVATE Reservations
	这些标志存储在预留映射指针的低比特位中。

	`#define HPAGE_RESV_OWNER    (1UL << 0)`
		指示此任务是与映射关联的预留的所有者。
	`#define HPAGE_RESV_UNMAPPED (1UL << 1)`
		指示最初映射此范围（并创建预留）的任务由于一次失败的
		写时复制（COW）而将此页从该任务（子进程）解除映射。
Page Flags
	PagePrivate 页标志用于指示在巨页被释放时必须恢复巨页预留。
	更多细节将在“释放巨页（Freeing huge pages）”一节中讨论。


## 预留映射的位置（私有或共享）


一个巨页映射或段（segment）要么是私有的，要么是共享的。如果是私有的，
它通常仅对单个地址空间（任务）可用。如果是共享的，它可以映射到多个
地址空间（任务）。预留映射的位置和语义对于这两类映射有显著不同。
位置上的差异为：

- 对于私有映射，预留映射挂在 VMA 结构上。具体来说，即 vma->vm_private_data。
  该预留映射在映射（mmap(MAP_PRIVATE)）创建时建立。
- 对于共享映射，预留映射挂在 inode 上。具体来说，即 inode->i_mapping->private_data。
  由于共享映射总是由 hugetlbfs 文件系统中的文件作后备，hugetlbfs 代码会确保
  每个 inode 都包含一个预留映射。因此，预留映射在 inode 创建时分配。


## 创建预留


当创建一个由巨页作后备的共享内存段（shmget(SHM_HUGETLB)）或通过
mmap(MAP_HUGETLB) 创建映射时，会创建预留。
```

	int hugetlb_reserve_pages(struct inode *inode,
				  long from, long to,
				  struct vm_area_struct *vma,
				  vm_flags_t vm_flags)

```
hugetlb_reserve_pages() 首先做的是检查 NORESERVE 标志是否在
shmget() 或 mmap() 调用中被指定。如果指定了 NORESERVE，则该例程立即返回，
因为不需要任何预留。

参数 'from' 和 'to' 是映射或底层文件中的巨页索引。对于 shmget()，
'from' 始终为 0，'to' 对应于段/映射的长度。对于 mmap()，offset 参数
可用于指定底层文件中的偏移量。在这种情况下，'from' 和 'to' 参数已经
根据该偏移量进行了调整。

PRIVATE 与 SHARED 映射之间的一个重大区别在于预留在预留映射中的表示方式。

- 对于共享映射，预留映射中的一个条目指示对应页存在或曾经存在预留。
  随着预留被消费，预留映射不会被修改。
- 对于私有映射，预留映射中缺少条目指示对应页存在预留。随着预留被消费，
  会向预留映射中添加条目。因此，预留映射也可用于确定哪些预留已经被消费。

对于私有映射，hugetlb_reserve_pages() 创建预留映射并将其挂在 VMA 结构上。
此外，会设置 HPAGE_RESV_OWNER 标志以指示此 VMA 拥有这些预留。

查询预留映射以确定当前映射/段需要多少巨页预留。对于私有映射，这始终是
值 (to - from)。然而，对于共享映射，有可能该范围 (to - from) 内已经存在
某些预留。有关这是如何完成的细节，请参见预留映射的修改一节
<resv_map_modifications>。

映射可能与一个子池（subpool）关联。如果是这样，会查询子池以确保映射有
足够的空间。子池有可能预留了一些可供该映射使用的预留。更多细节请参见
子池预留一节 <sub_pool_resv>。

在查询预留映射和子池之后，所需的新预留数量便已知。调用例程
hugetlb_acct_memory() 来检查并取得所请求的预留数量。hugetlb_acct_memory()
会调用一些例程，这些例程可能会分配并调整盈余（surplus）页计数。
然而，在这些例程内部，代码只是简单地检查以确保有足够的空闲巨页来满足
该预留。如果有，全局预留计数 resv_huge_pages 会被类似如下方式调整：
```

	if (resv_needed <= (free_huge_pages - resv_huge_pages)
		resv_huge_pages += resv_needed;

```
注意，在检查和调整这些计数器时，持有全局锁 hugetlb_lock。

如果当时有足够的空闲巨页且全局计数 resv_huge_pages 已被调整，则会修改
与该映射关联的预留映射以反映这些预留。对于共享映射，会存在一个包含
范围 'from' - 'to' 的 file_region。对于私有映射，不会对预留映射做任何
修改，因为缺少条目即表示存在预留。

如果 hugetlb_reserve_pages() 成功，则会根据需要修改与该映射关联的全局
预留计数和预留映射，以确保范围 'from' - 'to' 内存在预留。


## 消费预留/分配巨页


当与预留关联的巨页被分配并实例化到相应映射中时，预留即被消费。分配
```

	struct folio *alloc_hugetlb_folio(struct vm_area_struct *vma,
				     unsigned long addr, int avoid_reserve)

```
alloc_hugetlb_folio 被传入一个 VMA 指针和一个虚拟地址，因此它可以
查询预留映射以确定是否存在预留。此外，alloc_hugetlb_folio 接受参数
avoid_reserve，该参数指示即使看似已为指定地址预留了资源，也不应使用
这些预留。avoid_reserve 参数最常用于写时复制（Copy on Write）和页迁移
（Page Migration）的场景，此时正在分配现有页的额外副本。

辅助例程 vma_needs_reservation() 被调用来确定映射（vma）中该地址是否
存在预留。有关该例程行为的详细信息，请参见预留映射辅助例程一节
<resv_map_helpers>。
vma_needs_reservation() 的返回值通常为 0 或 1。如果该地址存在预留则为 0，
如果不存在预留则为 1。如果不存在预留，且映射关联了一个子池，则查询该子池
以确定它是否包含预留。如果子池包含预留，则其中一份可用于此次分配。
然而，在任何情况下，avoid_reserve 参数都会覆盖对该预留的使用。在确定了
是否存在预留且可用于此次分配之后，会调用例程 dequeue_huge_page_vma()。
该例程接受两个与预留相关的参数：

- avoid_reserve，这是传入 alloc_hugetlb_folio() 的同一个值/参数。
- chg，尽管该参数的类型为 long，但只传入值 0 或 1 给 dequeue_huge_page_vma。
  如果值为 0，表示存在预留（可能存在的问题请参见“内存策略与预留”一节）。
  如果值为 1，表示不存在预留，且如果可能的话，该页必须从全局空闲池中获取。

搜索与 VMA 内存策略关联的空闲链表以寻找空闲页。如果找到一页，当该页从
空闲链表中移除时，free_huge_pages 的值会递减。如果存在预留
```

	SetPagePrivate(page);	/* 指示分配此页消费了一个预留，并且
				 * 如果遇到错误必须释放该页，
				 * 预留将被恢复。 */
	resv_huge_pages--;	/* 递减全局预留计数 */

```
注意，如果找不到满足 VMA 内存策略的巨页，将尝试使用伙伴分配器（buddy allocator）
分配一页。这就引出了盈余巨页和过量提交（overcommit）的问题，这超出了预留的
讨论范围。即使分配了盈余页，也会进行与上述相同的基于预留的调整：
SetPagePrivate(page) 和 resv_huge_pages--。

在获得一个新的 hugetlb folio 之后，如果该页关联的子池存在，则 (folio)->_hugetlb_subpool
会被设为该子池的值。这将在 folio 被释放时用于子池记账。

然后调用例程 vma_commit_reservation() 以根据预留的消费情况调整预留映射。
一般来说，这涉及确保在区域映射的 file_region 结构中表示该页。对于预留已
存在的共享映射，预留映射中已存在条目，因此不做更改。然而，如果是共享映射中
没有预留，或者这是私有映射，则必须创建一个新条目。

在 alloc_hugetlb_folio() 开头调用 vma_needs_reservation() 与 folio 分配后调用
vma_commit_reservation() 之间，预留映射有可能已被更改。如果在共享映射中对
同一页调用了 hugetlb_reserve_pages，就会发生这种情况。在这种情况下，预留计数
和子池空闲页计数会相差一。这种罕见的情况可以通过比较 vma_needs_reservation
和 vma_commit_reservation 的返回值来识别。如果检测到这种竞争，会调整子池和
全局预留计数以进行补偿。有关这些例程的更多信息，请参见预留映射辅助例程一节
<resv_map_helpers>。


## 实例化巨页


在分配巨页之后，该页通常会被加入分配任务的页表中。在此之前，共享映射中的
页会被加入页缓存（page cache），私有映射中的页会被加入匿名反向映射
（anonymous reverse mapping）。在这两种情况下，PagePrivate 标志都会被清除。
因此，当已实例化的巨页被释放时，不会对全局预留计数（resv_huge_pages）做调整。


## 释放巨页


巨页由 free_huge_folio() 释放。由于它是从通用 MM 代码调用的，因此只传入
一个指向 folio 的指针。当释放一个巨页时，可能需要执行预留记账。如果
该页关联了一个包含预留的子池，或者该页正在错误路径上被释放且必须恢复
全局预留计数，就属于这种情况。

page->private 字段指向任何与该页关联的子池。如果设置了 PagePrivate 标志，
则指示应调整全局预留计数（有关这些标志如何设置的信息，请参见消费预留/
分配巨页一节 <consume_resv>）。

该例程首先为该页调用 hugepage_subpool_put_pages()。如果此例程返回值为 0
（不等于传入的值 1），则指示有预留与该子池关联，并且这个新释放的页必须
用于保持子池预留数量不低于最小大小。因此，在这种情况下全局 resv_huge_pages
计数器会递增。

如果页中设置了 PagePrivate 标志，全局 resv_huge_pages 计数器将总是被递增。


## 子池（Subpool）预留


每个巨页大小都关联一个 struct hstate。hstate 跟踪指定大小的所有巨页。
子池表示一个 hstate 中与管理挂载的 hugetlbfs 文件系统关联的一部分页。

挂载 hugetlbfs 文件系统时，可以指定 min_size 选项，指示该文件系统所需的最小
巨页数量。如果指定了此选项，则对应 min_size 的巨页数量会被预留供该文件系统
使用。此数量记录在 struct hugepage_subpool 的 min_hpages 字段中。在挂载时，
会调用 hugetlb_acct_memory(min_hpages) 来预留指定数量的巨页。如果无法预留，
挂载失败。

当从子池获取页或将其释放回子池时，会调用例程 hugepage_subpool_get/put_pages()。
它们执行所有子池记账，并跟踪与子池关联的任何预留。hugepage_subpool_get/put_pages
被传入用于调整子池“已用页”计数的巨页数量（get 时递减，put 时递增）。通常，
它们返回传入的同一值，或者如果子池中没有足够的页则返回错误。

然而，如果子池关联了预留，则可能返回小于传入值的值。该返回值指示必须
进行的额外全局池调整的数量。例如，假设一个子池包含 3 个已预留的巨页，而
有人请求 5 个。与该子池关联的 3 个预留页可用于满足部分请求。但是，还必须有
2 页从全局池中获取。为了将此信息传递给调用者，会返回值 2。然后调用者负责
尝试从全局池中获取额外的 2 页。


## 写时复制（COW）与预留


由于共享映射都指向并使用相同的底层页，COW 最大的预留问题在于私有映射。
在这种情况下，两个任务可能指向同一个先前已分配的页。一个任务尝试写入该页，
因此必须分配一个新页，使每个任务都指向自己的页。

当该页最初被分配时，该页的预留已被消费。当由于 COW 而尝试分配新页时，
有可能没有空闲巨页，分配将失败。

当私有映射最初被创建时，通过设置拥有者预留映射指针中的 HPAGE_RESV_OWNER
位来记录该映射的拥有者。由于拥有者创建了映射，拥有者就拥有与该映射关联的
所有预留。因此，当发生写缺页（write fault）且没有可用页时，对预留的拥有者
和非拥有者会采取不同的动作。

在缺页任务不是拥有者的情况下，缺页将失败，该任务通常会收到 SIGBUS。

如果缺页任务就是拥有者，我们希望它成功，因为它拥有原始的预留。为了实现这一点，
将该页从非拥有者任务解除映射。这样，唯一的引用来自拥有者任务。此外，会
在非拥有者任务的预留映射指针中设置 HPAGE_RESV_UNMAPPED 位。如果非拥有者任务
稍后对不存在的页发生缺页，它可能会收到 SIGBUS。但是，映射/预留的原始拥有者
会按预期行为。



## 预留映射的修改


以下底层例程用于对预留映射进行修改。通常，不会直接调用这些例程。相反，
会调用一个预留映射辅助例程，由它再去调用其中一个底层例程。这些底层例程在
源码中有相当完善的文档说明
```

	long region_chg(struct resv_map *resv, long f, long t);
	long region_add(struct resv_map *resv, long f, long t);
	void region_abort(struct resv_map *resv, long f, long t);
	long region_count(struct resv_map *resv, long f, long t);

```
对预留映射的操作通常涉及两个步骤：

1) 调用 region_chg() 来检查预留映射，确定指定范围 [f, t) 中有多少页当前
   未被表示。

   调用代码执行全局检查和分配，以确定是否有足够的巨页使操作成功。

2)
  a) 如果操作可以成功，调用 region_add() 来实际修改之前传给 region_chg()
     的同一范围 [f, t) 的预留映射。
  b) 如果操作无法成功，对同一个范围 [f, t) 调用 region_abort 来中止操作。

注意，这是一个两步过程，在对同一范围先调用 region_chg() 之后，region_add()
和 region_abort() 保证会成功。region_chg() 负责预分配任何必要的数据结构，
以确保后续操作（特别是 region_add()）会成功。

如上所述，region_chg() 确定映射中当前未被表示的范围内的页数。该数字返回给
调用者。region_add() 返回添加到映射中的范围内的页数。在大多数情况下，
region_add() 的返回值与 region_chg() 的返回值相同。然而，对于共享映射，在
对 region_chg() 和 region_add() 的调用之间有可能对预留映射做了修改。在这种
情况下，region_add() 的返回值将与 region_chg() 的返回值不匹配。很可能在
这种情况下全局计数和子池记账会不正确，需要做出调整。调用者有责任检查这种
情况并进行适当的调整。

调用 region_del() 来从预留映射中移除区域。通常在以下情况下调用它：

- 当 hugetlbfs 文件系统中的文件被移除时，inode 将被释放，预留映射被释放。
  在释放预留映射之前，必须释放所有单独的 file_region 结构。在这种情况下，
  region_del 被传入范围 [0, LONG_MAX)。
- 当 hugetlbfs 文件被截断时。在这种情况下，新文件大小之后的所有已分配页
  必须被释放。此外，预留映射中超出文件新末尾的任何 file_region 条目必须
  被删除。在这种情况下，region_del 被传入范围 [new_end_of_file, LONG_MAX)。
- 当在 hugetlbfs 文件中打孔（punch a hole）时。在这种情况下，巨页会从文件
  中间逐个移除。随着页被移除，调用 region_del() 来从预留映射中移除相应的
  条目。在这种情况下，region_del 被传入范围 [page_idx, page_idx + 1)。

在每种情况下，region_del() 都会返回从预留映射中移除的页数。在非常罕见的
情况下，region_del() 可能失败。这只可能发生在打孔的情形中，此时它必须拆分
一个现有的 file_region 条目却又无法分配一个新的结构。在这种错误情况下，
region_del() 将返回 -ENOMEM。这里的问题是，预留映射会指示该页存在一个
预留。然而，子池和全局预留计数不会反映该预留。为了处理这种情况，会调用
例程 hugetlb_fix_reserve_counts() 来调整计数器，使其与无法删除的那个预留
映射条目相对应。

在取消映射私有巨页映射时，会调用 region_count()。在私有映射中，预留映射中
缺少条目指示存在预留。因此，通过统计预留映射中的条目数，我们就知道已经
消费了多少预留，以及还有多少未决（outstanding = (end - start) - region_count(resv, start, end)）。
由于映射正在消失，子池和全局预留计数会按未决预留的数量递减。


## 预留映射辅助例程


存在若干辅助例程用于查询和修改预留映射。这些例程只关注特定巨页的预留，
因此它们只传入一个地址而非一个范围。此外，它们传入关联的 VMA。从 VMA 中
可以确定映射的类型（私有或共享）以及预留映射的位置（inode 或 VMA）。这些
例程只是简单地调用“预留映射的修改”一节中描述的底层例程。然而，它们确实
考虑了私有和共享映射中预留映射条目“相反”的含义
```

	long vma_needs_reservation(struct hstate *h,
				   struct vm_area_struct *vma,
				   unsigned long addr)

```
此例程对指定页调用 region_chg()。如果没有预留
```

	long vma_commit_reservation(struct hstate *h,
				    struct vm_area_struct *vma,
				    unsigned long addr)

```
这将对指定页调用 region_add()。与 region_chg 和 region_add 的情况一样，
应在之前调用过 vma_needs_reservation 之后再调用此例程。它会为该页添加一个
预留条目。如果添加了预留则返回 1，否则返回 0。返回值应与之前对
vma_needs_reservation 的调用返回值进行比较。一个意外的差异指示预留
```

	void vma_end_reservation(struct hstate *h,
				 struct vm_area_struct *vma,
				 unsigned long addr)

```
这将对指定页调用 region_abort()。与 region_chg 和 region_abort 的情况一样，
应在之前调用过 vma_needs_reservation 之后再调用此例程。它将中止/结束正在
进行的预留添加
```

	long vma_add_reservation(struct hstate *h,
				 struct vm_area_struct *vma,
				 unsigned long addr)

```
这是一个特殊的包装例程，用于帮助在错误路径上进行预留清理。它只从例程
restore_reserve_on_error() 中调用。此例程与 vma_needs_reservation 配合
使用，以尝试向预留映射添加预留。它考虑了私有和共享映射不同的预留映射
语义。因此，对于共享映射调用 region_add（因为映射中存在条目即表示有预留），
对于私有映射调用 region_del（因为映射中缺少条目即表示有预留）。有关错误路径
上需要做什么的更多信息，请参见“错误路径中的预留清理”一节。


## 错误路径中的预留清理


如预留映射辅助例程一节 <resv_map_helpers> 所述，预留映射的修改分两步进行。
首先在分配页之前调用 vma_needs_reservation。如果分配成功，则调用
vma_commit_reservation。如果不成功，则调用 vma_end_reservation。根据操作
的成功或失败来调整全局和子池预留计数，一切正常。

此外，在巨页被实例化后，PagePrivate 标志会被清除，以便在该页最终被释放时
记账正确。

然而，存在若干在巨页被分配之后、但在其被实例化之前遇到错误的情形。在这种
情况下，页分配已经消费了预留，并做了相应的子池、预留映射和全局计数调整。
如果此时（在实例化和清除 PagePrivate 之前）释放该页，则 free_huge_folio
会递增全局预留计数。但是，预留映射指示预留已被消费。这种不一致的状态将
导致一个已预留巨页的“泄漏”（leak）。全局预留计数将比应有的更高，并阻止
分配一个预分配的页。

例程 restore_reserve_on_error() 试图处理这种情况。它有相当完善的文档说明。
该例程的意图是将预留映射恢复到页分配之前的状态。这样，在页被释放后，预留
映射的状态将与全局预留计数相对应。

restore_reserve_on_error 例程本身在尝试恢复预留映射条目时也可能遇到错误。
在这种情况下，它会简单地清除该页的 PagePrivate 标志。这样，在该页被释放时
全局预留计数不会被递增。但是，预留映射会继续看起来像是预留已被消费。仍然
可以为该地址分配一个页，但它不会像最初预期的那样使用一个已预留的页。

有一些代码（最显著的是 userfaultfd）无法调用 restore_reserve_on_error。
在这种情况下，它只是修改 PagePrivate，以便在释放巨页时不会泄漏预留。


## 预留与内存策略


当 git 最初被用于管理 Linux 代码时，per-node 巨页链表就已经存在于 struct hstate
中。预留的概念是在一段时间之后才加入的。加入预留时，并没有尝试将内存策略
考虑在内。虽然 cpusets 与内存策略并不完全相同，但 hugetlb_acct_memory 中的
这段注释概括了预留与它们之间的相互作用
```

	/*
	 * When cpuset is configured, it breaks the strict hugetlb page
	 * reservation as the accounting is done on a global variable. Such
	 * reservation is completely rubbish in the presence of cpuset because
	 * the reservation is not checked against page availability for the
	 * current cpuset. Application can still potentially OOM'ed by kernel
	 * with lack of free htlb page in cpuset that the task is in.
	 * Attempt to enforce strict accounting with cpuset is almost
	 * impossible (or too ugly) because cpuset is too fluid that
	 * task or memory node can be dynamically moved between cpusets.
	 *
	 * The change of semantics for shared hugetlb mapping with cpuset is
	 * undesirable. However, in order to preserve some of the semantics,
	 * we fall back to check against current free page availability as
	 * a best attempt and hopefully to minimize the impact of changing
	 * semantics that cpuset has.
	 */

```
加入巨页预留是为了防止在缺页时出现意外的页分配失败（OOM）。然而，如果应用
程序使用了 cpusets 或内存策略，则无法保证在所需节点上有巨页可用。即使有
足够数量的全局预留，也是如此。

## Hugetlbfs 回归测试


最完整的 hugetlb 测试集位于 libhugetlbfs 仓库中。如果你修改了任何 hugetlb
相关的代码，请使用 libhugetlbfs 测试套件来检查是否出现回归。此外，如果你
添加了任何新的 hugetlb 功能，请向 libhugetlbfs 添加适当的测试。

--
Mike Kravetz, 2017 年 4 月 7 日
