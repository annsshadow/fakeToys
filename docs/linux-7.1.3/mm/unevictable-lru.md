## 不可回收 LRU 基础设施




## 简介


本文档描述 Linux 内存管理器的"不可回收 LRU"（Unevictable LRU）基础设施，以及利用它来管理几种类型的"不可回收"页帧（folio）。

本文档试图提供这一机制背后的整体设计依据，以及驱动实现的一些设计决策背后的依据。后者的设计依据在实现的上下文中加以讨论。无可否认，通过阅读代码即可获得实现细节——即"它做了什么"。作者希望下面的描述能够通过回答"它为何那样做？"而提供额外的价值。



## 不可回收 LRU


不可回收 LRU 机制新增了一个额外的 LRU 链表，用于跟踪不可回收的 folio，并将这些 folio 对 vmscan 隐藏。该机制基于 Red Hat 的 Larry Woodman 的一个补丁，旨在解决 Linux 中 folio 回收的若干可扩展性问题。这些问题已在客户的大型内存 x86_64 系统上观察到。

举例说明，一台拥有 128GB 主存的非 NUMA x86_64 平台在单个节点上会有超过 3200 万个 4k 页。当这些页中有很大一部分因任何原因而不可回收[见下文]时，vmscan 将花费大量时间扫描 LRU 链表以寻找那小部分可回收的页。这可能导致这样一种情况：所有 CPU 连续数小时或数天将 100% 的时间耗费在 vmscan 中，系统完全无响应。

不可回收链表处理了以下几类不可回收页：

 - 由 ramfs 拥有的页。

 - 由带有 noswap 挂载选项的 tmpfs 拥有的页。

 - 映射到 SHM_LOCK 的共享内存区域的页。

 - 映射到 VM_LOCKED [mlock()ed] VMA 的页。

该基础设施未来或许还能处理因定义或环境而使得页不可回收的其他情况。


### 不可回收 LRU 的 folio 链表


不可回收 LRU folio 链表是个"谎言"。它从来就不是一个按 LRU 排序的链表，而是与按 LRU 排序的匿名和文件、活跃和非活跃 folio 链表相伴而生的；而如今它甚至不再是 folio 链表。但遵循惯用约定，在本文档和源码中，我们经常把它想象成第五个 LRU folio 链表。

不可回收 LRU 基础设施包含一个额外的、每节点的 LRU 链表，称为"unevictable"链表，以及一个相关的 folio 标志 PG_unevictable，用于表明该 folio 正由不可回收链表管理。

PG_unevictable 标志类似于 PG_active 标志且与之互斥，因为当 PG_lru 被置位时，它表示 folio 位于哪个 LRU 链表上。

不可回收 LRU 基础设施将不可回收 folio 当作它们在额外的 LRU 链表上一样来维护，原因有几点：

 (1) 我们可以"像对待系统中其他 folio 一样对待不可回收 folio——这意味着我们可以使用相同的代码来操作它们、使用相同的代码来隔离它们（用于迁移等）、使用相同的代码来跟踪统计信息等……" [Rik van Riel]

 (2) 我们希望能够在节点之间迁移不可回收 folio，以进行内存碎片整理、工作负载管理和内存热插拔。Linux 内核只能迁移那些能够成功从 LRU 链表隔离出来的 folio（或"可移动"的 folio：这里不在考虑范围内）。如果我们把这些 folio 维护在 LRU 类链表之外（即 folio_isolate_lru() 无法检测到的地方），就会阻止它们的迁移。

不可回收链表不区分文件后备与匿名、swap 后备的 folio。这种区分仅在 folio 确实可回收时才有意义。

不可回收链表受益于 Christoph Lameter 最初提出并发布的每节点 LRU 链表与统计的"数组化"（arrayification）。


### 内存控制组交互


不可回收 LRU 机制通过扩展 lru_list 枚举，与内存控制组[即 memory controller；见 Documentation/admin-guide/cgroup-v1/memory.rst]交互。

由于每节点 LRU 链表的"数组化"（每个 lru_list 枚举元素对应一个），内存控制器数据结构会自动获得一个每节点的不可回收链表。内存控制器跟踪页进出不可回收链表的移动。

当某个内存控制组面临内存压力时，控制器不会尝试回收不可回收链表上的页。这有几个效果：

 (1) 因为这些页在不可回收链表上对回收"隐藏"，回收过程可以更高效，只处理那些有可能被回收的页。

 (2) 另一方面，如果计入该控制组的页中有太多不可回收，该控制组任务的 working set 中可回收的部分可能无法放入可用内存。这可能导致控制组发生颠簸（thrash）或对任务执行 OOM-kill。


### 将地址空间标记为不可回收


对于像 ramfs 这样的设施，附加到地址空间的页都不可被回收。为了防止任何此类页被回收，提供了 AS_UNEVICTABLE 地址空间标志，文件系统可以使用若干包装函数来操作它：

 - `void mapping_set_unevictable(struct address_space *mapping);`

	将该地址空间标记为完全不可回收。

 - `void mapping_clear_unevictable(struct address_space *mapping);`

	将该地址空间标记为可回收。

 - `int mapping_unevictable(struct address_space *mapping);`

	查询该地址空间，如果它完全不可回收则返回 true。

这些函数目前在内核的三个地方使用：

 (1) 由 ramfs 在其 inode 创建时标记其地址空间，该标记在 inode 的整个生命周期内保持。

 (2) 由 SYSV SHM 标记 SHM_LOCK 的地址空间，直到调用 SHM_UNLOCK。注意，如果锁定的页已被换出，SHM_LOCK 并不要求将它们调页入内存；应用程序若想确保它们在内存中，必须手动访问这些页。

 (3) 由 i915 驱动标记被固定的地址空间，直到其被解除固定。i915 驱动标记的不可回收内存量大约对应于 debugfs/dri/0/i915_gem_objects 中的有界对象大小。


### 检测不可回收页


mm/internal.h 中的函数 folio_evictable() 使用上文概述的查询函数[见 Marking address spaces unevictable <mark_addr_space_unevict> 一节]来检查 AS_UNEVICTABLE 标志，从而判断一个 folio 是否可回收。

对于在填充之后才被如此标记的地址空间（SHM 区域可能如此），加锁动作（例如 SHM_LOCK）可以是惰性的，无需像 mlock() 那样填充该区域的页表，也无需特意将 SHM_LOCK 区域内的任何页推入不可回收链表。相反，vmscan 会在回收扫描中遇到这些 folio 时再做这件事。

在解锁动作（如 SHM_UNLOCK）时，解锁者（如 shmctl()）必须扫描该区域的页，并在没有其他条件使其保持不可回收的情况下，将它们从不可回收链表中"解救"出来。如果一个不可回收区域被销毁，这些页也会在释放过程中从不可回收链表中被"解救"出来。

folio_evictable() 还会通过调用 folio_test_mlocked() 来检查 mlocked 的 folio，后者在 folio 被缺页映射进 VM_LOCKED VMA、或在正被 VM_LOCKED 的 VMA 中被发现时置位。


### Vmscan 对不可回收 folio 的处理


如果不可回收 folio 在缺页路径中被剔除（cull），或在 mlock()/mmap() 时被移到不可回收链表，那么 vmscan 在它们重新变为可回收（例如通过 munlock()）并从不可回收链表被"解救"之前，不会遇到这些 folio。然而，出于便利，我们可能会决定把一个不可回收 folio 留在某个常规的活跃/非活跃 LRU 链表上，交由 vmscan 处理。vmscan 在所有 shrink_{active|inactive|folio}_list() 函数中都会检查此类 folio，并会"剔除"遇到的这类 folio：即把那些 folio 转向正在扫描的内存 cgroup 和节点的不可回收链表。

在某些情况下，一个 folio 被映射到 VM_LOCKED VMA，但该 folio 没有设置 mlocked 标志。这样的 folio 会一路到达 shrink_active_list() 或 shrink_folio_list()，在 vmscan 通过 folio_referenced() 或 try_to_unmap() 遍历反向映射时被检测到。当该 folio 被 shrinker 释放时，它会被剔除到不可回收链表。

要"剔除"一个不可回收 folio，vmscan 在释放 folio 锁后，简单地通过 folio_putback_lru()（folio_isolate_lru() 的逆操作）把 folio 放回 LRU 链表。因为使 folio 不可回收的条件在 folio 解锁后可能改变，__pagevec_lru_add_fn() 会在把它放到不可回收链表之前重新检查其不可回收状态。


## MLOCKED 页


除了 ramfs 和 SYSV SHM 之外，不可回收 folio 链表对 mlock() 也很有用。注意，mlock() 仅在 CONFIG_MMU=y 的情况下可用；在 NOMMU 情况下，所有映射实际上都是 mlocked 的。


### 历史


"不可回收的 mlocked 页"基础设施基于 Nick Piggin 最初在一个题为 "mm: mlocked pages off LRU" 的 RFC 补丁中发表的工作。Nick 发布他的补丁，作为 Christoph Lameter 发布的、达成同一目标（将 mlocked 页对 vmscan 隐藏）的补丁的替代方案。

在 Nick 的补丁中，他用 struct page 的 LRU 链表链接字段之一，作为映射该页的 VM_LOCKED VMA 的计数（Rik van Riel 三年前有过同样的想法）。但这种将该链接字段用于计数的做法，妨碍了在 LRU 链表上管理这些页，因此 mlocked 页不可迁移，因为 folio_isolate_lru() 无法检测到它们，并且 LRU 链表链接字段也无法供迁移子系统使用。

Nick 通过在进行隔离之前把 mlocked 页放回 LRU 链表解决了这个问题，从而放弃了 VM_LOCKED VMA 的计数。当 Nick 的补丁与不可回收 LRU 工作整合时，该计数被替换为在 munlock 时遍历反向映射，以判断是否还有其他 VM_LOCKED VMA 仍映射着该页。

然而，在 munlock 时为每个页遍历反向映射既丑陋又低效，并且当许多已 mlock 它的进程试图退出时，会导致文件 rmap 锁上灾难性的争用。在 5.18 中，将 mlock_count 保存在不可回收 LRU 链表链接字段中的想法被重新启用并付诸实践，同时不妨碍 mlocked 页的迁移。这就是为什么"不可回收 LRU 链表"现在不能是一个页的链表；不过那个链表本来也没什么用处——尽管其大小仍被维护以用于 meminfo。


### 基本管理


mlocked 页——即映射到 VM_LOCKED VMA 的页——是一类不可回收页。当内存管理子系统"注意到"这样的页时，该 folio 会被标记上 PG_mlocked 标志。这可以用 folio_set_mlocked() 和 folio_clear_mlocked() 函数操作。

一个 PG_mlocked 页在加入 LRU 时会被放到不可回收链表上。这样的页可以在多个地方被内存管理"注意到"：

 (1) 在 mlock()/mlock2()/mlockall() 系统调用处理程序中；

 (2) 在 mmap() 系统调用处理程序中，当以 MAP_LOCKED 标志 mmap 一个区域时；

 (3) 在一个曾以 MCL_FUTURE 标志调用 mlockall() 的任务中 mmap 一个区域时；

 (4) 在缺页路径中，以及当 VM_LOCKED 栈段被扩展时；或

 (5) 如上所述，在 vmscan:shrink_folio_list() 中，当试图通过 folio_referenced() 或 try_to_unmap() 回收某个 VM_LOCKED VMA 中的页时。

mlocked 页在以下情况被解锁并从不可回收链表解救出来：

 (1) 处于通过 munlock()/munlockall() 系统调用解锁的范围内的映射；

 (2) 通过 munmap() 从映射该页的最后一个 VM_LOCKED VMA 中移除，包括在任务退出时的解除映射；

 (3) 当该页从某个 mmapped 文件的最后一个 VM_LOCKED VMA 中被截断时；或

 (4) 在 VM_LOCKED VMA 中对页执行 COW（写时复制）之前。


### mlock()/mlock2()/mlockall() 系统调用处理


mlock()、mlock2() 和 mlockall() 系统调用处理程序会对调用所指定范围内的每个 VMA 调用 mlock_fixup()。在 mlockall() 的情况下，这就是任务的整个活动地址空间。注意，mlock_fixup() 既用于 mlock 也用于 munlock 一段内存。对已经是 VM_LOCKED 的 VMA 调用 mlock()，或对不是 VM_LOCKED 的 VMA 调用 munlock()，都被视为空操作，mlock_fixup() 直接返回。

如果 VMA 通过了下文"Filtering Special VMAs"中描述的某些过滤，mlock_fixup() 会尝试将 VMA 与其相邻者合并，或者在范围未覆盖整个 VMA 时切分出 VMA 的一个子集。VMA 中已有的任何页随后会通过 mlock_vma_pages_range() → walk_page_range() → mlock_pte_range() → mlock_folio() 被标记为 mlocked。

在从系统调用返回之前，do_mlock() 或 mlockall() 会调用 __mm_populate()，通过 get_user_pages() 将剩余页缺页调入，并在它们被缺页时标记为 mlocked。

注意，被 mlock 的 VMA 可能以 PROT_NONE 映射。在这种情况下，get_user_pages() 将无法将这些页缺页调入。这没关系。如果页最终被缺页映射进这个 VM_LOCKED VMA，它们会在缺页路径中处理——mlock2() 的 MLOCK_ONFAULT 区域也是这样处理的。

对于被缺页映射进 VMA 的每个 PTE（或 PMD），页的 rmap 添加函数会调用 mlock_vma_folio()，当 VMA 为 VM_LOCKED 时它会调用 mlock_folio()（除非它是透明大页一部分的 PTE 映射）。或者，当它是一个新分配的匿名页时，folio_add_lru_vma() 会改为调用 mlock_new_folio()：与 mlock_folio() 类似，但能做出更好的判断，因为该页被独占持有且已知尚未在 LRU 上。

mlock_folio() 立即设置 PG_mlocked，然后把页放到 CPU 的 mlock folio 批处理中，以将剩余工作批处理、在 lru_lock 下由 __mlock_folio() 完成。__mlock_folio() 设置 PG_unevictable，初始化 mlock_count，并将页转移到不可回收状态（"不可回收 LRU"，但以 mlock_count 代替 LRU 链接）。或者，如果页已经是 PG_lru、PG_unevictable 和 PG_mlocked，则只是递增 mlock_count。

但在实践中这未必理想：页可能尚未在 LRU 上，或者可能已被临时从 LRU 隔离。在这种情况下不能接触 mlock_count 字段，但它会在 __munlock_folio() 将页归还"LRU"时被设为 0。竞态禁止此时将 mlock_count 设为 1：与其冒着将页永久孤立为不可回收的风险，不如总是让 mlock_count 偏向低值，这样在 munlock 时该页会被解救到可回收 LRU，若之后 vmscan 在 VM_LOCKED VMA 中发现它，可能再次被 mlock。


### 过滤特殊 VMA


mlock_fixup() 过滤几类"特殊" VMA：

1) 设置了 VM_IO 或 VM_PFNMAP 的 VMA 被完全跳过。这些映射背后的页本质上是被固定的，因此我们不需要将它们标记为 mlocked。无论如何，这些页大多没有可供标记的 struct page。因此，get_user_pages() 对这些 VMA 会失败，所以尝试访问它们没有意义。

2) 映射 hugetlbfs 页的 VMA 实际上已经被固定到内存中。我们既不需要也不想对这些页做 mlock()。但 __mm_populate() 会包含 hugetlbfs 范围，分配大页并填充 PTE。

3) 带有 VM_DONTEXPAND 的 VMA 通常是内核页的用户态映射，例如 VDSO 页、relay 通道页等。这些页本质上不可回收，且不在 LRU 链表上管理。__mm_populate() 会包含这些范围，在尚未填充时填充 PTE。

4) 设置了 VM_MIXEDMAP 的 VMA 不会被标记为 VM_LOCKED，但 __mm_populate() 会包含这些范围，在尚未填充时填充 PTE。

注意，对于所有这些特殊 VMA，mlock_fixup() 不会设置 VM_LOCKED 标志。因此，我们之后在 munlock()、munmap() 或任务退出时不必处理它们。mlock_fixup() 也不会将这些 VMA 计入任务的 "locked_vm"。


### munlock()/munlockall() 系统调用处理


munlock() 和 munlockall() 系统调用由与 mlock()、mlock2() 和 mlockall() 相同的 mlock_fixup() 函数处理。如果调用 munlock 一个已经 munlock 的 VMA，mlock_fixup() 直接返回。由于上述 VMA 过滤，任何"特殊" VMA 中都不会设置 VM_LOCKED。因此，那些 VMA 在 munlock 时会被忽略。

如果 VMA 是 VM_LOCKED，mlock_fixup() 会再次尝试合并或切分出指定的范围。然后 VMA 中的所有页通过 mlock_vma_pages_range() → walk_page_range() → mlock_pte_range() → munlock_folio() 被 munlock——这与 mlock 一个 VMA 范围时使用的函数相同，只是 VMA 上带有标明正在执行 munlock() 的新标志。

munlock_folio() 使用 mlock pagevec 来批处理将在 lru_lock 下由 __munlock_folio() 完成的工作。__munlock_folio() 递减 folio 的 mlock_count，当减到 0 时清除 mlocked 标志和 unevictable 标志，将 folio 从不可回收状态转移到非活跃 LRU。

但在实践中这未必理想：folio 可能尚未到达"不可回收 LRU"，或者可能已被临时从中隔离。在这些情况下它的 mlock_count 字段不可用，必须假定为 0：这样 folio 会被解救到可回收 LRU，若之后 vmscan 在 VM_LOCKED VMA 中发现它，可能再次被 mlock。


### 迁移 MLOCKED 页


正在迁移的页已被从 LRU 链表隔离，并在该页的解映射、更新页的地址空间项、复制内容和状态期间保持锁定，直到页表项被替换为指向新页的项。Linux 支持迁移 mlocked 页和其他不可回收页。当旧页从最后一个 VM_LOCKED VMA 解映射时，PG_mlocked 从旧页清除；当新页被映射到 VM_LOCKED VMA 中取代迁移项时，PG_mlocked 被设置。如果页因 mlocked 而不可回收，PG_unevictable 跟随 PG_mlocked；但如果页因其他原因而不可回收，则显式复制 PG_unevictable。

注意，页迁移可能与同一页的 mlock 或 munlock 发生竞态。这基本没有问题，因为页迁移需要解映射旧页的所有 PTE（包括 VM_LOCKED 时的 munlock），然后映射新页（包括 VM_LOCKED 时的 mlock）。页表锁提供了充分的同步。

然而，由于 mlock_vma_pages_range() 从在 VMA 上设置 VM_LOCKED 开始，之后才 mlock 任何已存在的页，如果其中某个页在 mlock_pte_range() 到达它之前被迁移了，它会在 mlock_count 中被计数两次。为防止这种情况，mlock_vma_pages_range() 临时将 VMA 标记为 VM_IO，使 mlock_vma_folio() 跳过它。

为完成页迁移，我们在之后将旧页和新页放回 LRU。那个"不需要的"页——成功时是旧页，失败时是新页——在迁移过程持有的引用计数被释放时被释放。


### 压缩 MLOCKED 页


可以扫描内存映射以寻找可压缩区域，默认行为是允许移动不可回收页。/proc/sys/vm/compact_unevictable_allowed 控制这一行为（见 Documentation/admin-guide/sysctl/vm.rst）。压缩工作主要由页迁移代码处理，并套用 Migrating MLOCKED Pages 中描述的工作流程。


### 对透明大页执行 MLOCK


透明大页由 LRU 链表上的一个单独项表示。因此，我们只能使整个复合页不可回收，而不能使单个子页不可回收。

如果用户尝试 mlock() 大页的一部分，并且没有其他用户 mlock() 整个大页，我们希望大页的其余部分可回收。

我们不能在部分 mlock() 时直接拆分该页，因为 split_huge_page() 可能失败，而且我们不希望系统调用出现新的间歇性失败模式。

我们的处理方式是：将 PTE-mlocked 的大页保留在可回收 LRU 链表上：VM_LOCKED VMA 边界处的 PMD 会被拆分为 PTE 表。

这样大页对 vmscan 是可访问的。在内存压力下，该页会被拆分，属于 VM_LOCKED VMA 的子页会被移到不可回收 LRU，其余部分可被回收。

/proc/meminfo 的 Unevictable 和 Mlocked 数值不包含那些仅由 VM_LOCKED VMA 中 PTE 映射的透明大页部分。


### mmap(MAP_LOCKED) 系统调用处理


除了 mlock()、mlock2() 和 mlockall() 系统调用之外，应用程序还可以通过向 mmap() 调用提供 MAP_LOCKED 标志，请求将一段内存区域 mlock。不过这里有一个重要且微妙的区别。mmap() + mlock() 在范围无法被缺页调入时（例如因为 mm_populate 失败）会失败并返回 ENOMEM，而 mmap(MAP_LOCKED) 不会失败。被 mmap 的区域仍将具有锁定区域的属性——页不会被换出——但将内存缺页调入的重大缺页异常仍可能发生。

此外，任何曾以 MCL_FUTURE 标志调用 mlockall() 的任务所做的、扩展堆的 mmap() 调用或 brk() 调用，都会导致新映射的内存被 mlock。在不可回收/mlock 改动之前，内核只是调用 make_pages_present() 来分配页并填充页表。

要在不可回收/mlock 基础设施下 mlock 一段内存范围，mmap() 处理程序和任务地址空间扩展函数会调用 populate_vma_page_range()，指定要 mlock 的 vma 和地址范围。


### munmap()/exit()/exec() 系统调用处理


当解除映射一段 mlocked 的内存区域时，无论是通过显式调用 munmap()，还是经由 exit() 或 exec() 处理中的内部解映射，如果我们正在移除映射这些页的最后一个 VM_LOCKED VMA，就必须 munlock 这些页。在不可回收/mlock 改动之前，mlock 不会以任何方式标记这些页，因此解除它们的映射无需任何处理。

对于正从 VMA 解除映射的每个 PTE（或 PMD），folio_remove_rmap_*() 会调用 munlock_vma_folio()，当 VMA 为 VM_LOCKED 时它会调用 munlock_folio()（除非它是透明大页一部分的 PTE 映射）。

munlock_folio() 使用 mlock pagevec 来批处理将在 lru_lock 下由 __munlock_folio() 完成的工作。__munlock_folio() 递减 folio 的 mlock_count，当减到 0 时清除 mlocked 标志和 unevictable 标志，将 folio 从不可回收状态转移到非活跃 LRU。

但在实践中这未必理想：folio 可能尚未到达"不可回收 LRU"，或者可能已被临时从中隔离。在这些情况下它的 mlock_count 字段不可用，必须假定为 0：这样 folio 会被解救到可回收 LRU，若之后 vmscan 在 VM_LOCKED VMA 中发现它，可能再次被 mlock。


### 截断 MLOCKED 页


文件截断或打洞会强制将已删除的页从用户空间解映射；截断甚至会解映射并删除任何从正被截断的文件页 Copy-On-Write（写时复制）而来的私有匿名页。

Mlocked 页可以以这种方式被 munlock 并删除：与 munmap() 类似，对于正从 VMA 解除映射的每个 PTE（或 PMD），folio_remove_rmap_*() 会调用 munlock_vma_folio()，当 VMA 为 VM_LOCKED 时它会调用 munlock_folio()（除非它是透明大页一部分的 PTE 映射）。

然而，如果存在竞争的 munlock()，由于 mlock_vma_pages_range() 通过从 VMA 清除 VM_LOCKED 来开始 munlock，在 munlock 所有已存在的页之前，如果其中某个页在 mlock_pte_range() 到达它之前就被截断或打洞解映射了，那么本 VMA 就不会将其识别为 mlocked，也不会从 mlock_count 中减除。在这种罕见情况下，一个页在完全解映射后可能仍显示为 PG_mlocked：此时交由 release_pages()（或 __page_cache_release()）在释放前清除它并更新统计（此事件计入 /proc/vmstat 的 unevictable_pgs_cleared，该值通常为 0）。


### shrink_*_list() 中的页回收


vmscan 的 shrink_active_list() 会剔除任何明显不可回收的页——即 !page_evictable(page) 的页——将它们转向不可回收链表。然而，shrink_active_list() 只能看到那些进入了活跃/非活跃 LRU 链表的不可回收页。注意，这些页没有设置 PG_unevictable——否则它们会在不可回收链表上，而 shrink_active_list() 永远不会看到它们。

LRU 链表上这类不可回收页的一些例子是：

 (1) 首次分配时就被放到 LRU 链表上的 ramfs 页。

 (2) SHM_LOCK 的共享内存页。shmctl(SHM_LOCK) 不会尝试分配或调入共享内存区域中的页。这发生在应用程序在 SHM_LOCK 该段之后第一次访问该页时。

 (3) 仍映射到 VM_LOCKED VMA 的页，本应被标记为 mlocked，但事件导致 mlock_count 过低，因此它们被过早 munlock 了。

vmscan 的 shrink_inactive_list() 和 shrink_folio_list() 也会将非活跃链表上发现的明显不可回收页，转向适当的 memory cgroup 和节点的不可回收链表。

rmap 的 folio_referenced_one()（经由 vmscan 的 shrink_active_list() 或 shrink_folio_list() 调用）以及 rmap 的 try_to_unmap_one()（经由 shrink_folio_list() 调用）会检查仍然映射到 VM_LOCKED VMA 的 (3) 类页，并调用 mlock_vma_folio() 来纠正它们。这类页在被 shrinker 释放时会被剔除到不可回收链表。
