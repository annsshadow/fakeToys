## 页迁移（Page migration）


页迁移允许在进程运行时，在 NUMA 系统的节点之间移动页的物理位置。这意味着进程看到的虚拟地址不会改变。然而，系统会重新排列这些页的物理位置。

另请参阅 Documentation/mm/hmm.rst，了解向设备私有内存迁移页或从设备私有内存迁移页的内容。

页迁移的主要意图是通过将页移动到正在访问该内存的进程所在处理器附近，来降低内存访问的延迟。

页迁移允许进程通过 MF_MOVE 和 MF_MOVE_ALL 选项，在通过 mbind() 设置新的内存策略时，手动重定位其页所在的节点。进程的页也可以通过 sys_migrate_pages() 函数调用从另一个进程进行重定位。migrate_pages() 函数调用接受两组节点，并将位于 from 节点上的进程页移动到目标节点。页迁移函数由 Andi Kleen 的 numactl 包提供（需要 0.9.3 之后的版本。可从 https://github.com/numactl/numactl.git 获取）。numactl 提供 libnuma，它为页迁移提供了类似于其他 NUMA 功能的接口。cat `/proc/<pid>/numa_maps` 可以方便地查看进程的页位于何处。另请参阅 proc(5) 手册页中的 numa_maps 文档。

例如，如果调度器已将进程重定位到远端节点上的处理器，手动迁移就很有用。批处理调度器或管理员可以检测到这种情况，并将进程的页移动到更靠近新处理器的位置。内核本身只提供手动页迁移支持。自动页迁移可以通过移动页的用户空间进程来实现。一个特殊的函数调用 “move_pages” 允许在进程内移动单个页。例如，NUMA 性能分析器可能获得一个显示频繁的非本地节点访问的日志，并可能使用结果将页移动到更有利的位置。

较大的部署通常使用 cpusets 将系统划分为若干节点区段。Paul Jackson 为 cpusets 配备了在任务被移动到另一个 cpuset 时移动页的能力（参见 CPUSETS <cpusets>）。Cpusets 允许自动化进程的位置局部性。如果一个任务被移动到一个新的 cpuset，那么它的所有页也会随之移动，这样进程的性能就不会急剧下降。此外，如果某个 cpuset 允许的节点发生变化，该 cpuset 中进程的页也会被移动。

页迁移保留了所有迁移技术中一组节点内页的相对位置，即使在迁移进程后也会保留特定的内存分配模式。这是为了保持内存延迟所必需的。迁移后进程将以类似的性能运行。

页迁移分几个步骤进行。首先是为那些试图从内核使用 migrate_pages() 的人提供的高层描述（用户空间用法请参见上面提到的 Andi Kleen 的 numactl 包），然后是对底层细节如何运作的底层描述。

## 内核中对 migrate_pages() 的使用


1. 从 LRU 中移除 folios。

   要迁移的 folios 列表是通过扫描 folios 并将它们移入列表来生成的。这是通过调用 folio_isolate_lru() 完成的。
   调用 folio_isolate_lru() 会增加对 folio 的引用，以便在 folio 迁移发生时它不会消失。
   它还防止 swapper 或其他扫描遇到该 folio。

2. 我们需要一个 new_folio_t 类型的函数，可以传递给 migrate_pages()。这个函数应该弄清楚如何在给定旧 folio 的情况下分配正确的新 folio。

3. 调用 migrate_pages() 函数尝试执行迁移。它会为每个被考虑移动的 folio 调用该函数来分配新的 folio。

## migrate_pages() 的工作原理


migrate_pages() 会对它的 folios 列表进行多次遍历。如果一个 folio 在当时的所有引用都是可移除的，则该 folio 被移动。该 folio 已经通过 folio_isolate_lru() 从 LRU 中移除，并且引用计数已增加，以便在 folio 迁移发生时不会被释放。

步骤：

1. 锁定要迁移的页。

2. 确保回写（writeback）已完成。

3. 锁定我们要移动到的那个新页。它被锁定，以便在移动进行期间对该页（尚未是最新的）的访问会立即被阻塞。

4. 页表中对该页的所有引用都被转换为迁移条目。这会减少页的 mapcount。如果结果 mapcount 不为零，那么我们就不迁移该页。所有试图访问该页的用户空间进程现在将等待页锁，或等待迁移页表条目被移除。

5. 获取 i_pages 锁。这将导致所有试图通过映射访问该页的进程在自旋锁上阻塞。

6. 检查页的引用计数，如果仍有引用则退出。否则，我们就知道自己是唯一引用此页的。

7. 检查基数树（radix tree），如果它不包含指向此页的指针，则退出，因为其他某个进程修改了基数树。

8. 用旧页的一些设置为新页做准备，以便对新页的访问会发现一个具有正确设置的页。

9. 将基数树改为指向新页。

10. 旧页的引用计数被丢弃，因为地址空间引用已经消失。建立对新页的引用，因为新页被地址空间引用。

11. 释放 i_pages 锁。这样映射中的查找再次变为可能。进程将从自旋锁上的自旋转为在锁定的新页上睡眠。

12. 将页内容复制到新页。

13. 剩余的页标志被复制到新页。

14. 清除旧页标志，以表明该页不再提供任何信息。

15. 触发新页上排队的回写。

16. 如果迁移条目被插入了页表，则用真实的 pte 替换它们。这样做将允许尚未等待页锁的用户空间进程进行访问。

17. 从旧页和新页上释放页锁。等待页锁的进程将重做它们的页错误，并到达新页。

18. 将新页移动到 LRU，并且可以再次被 swapper 等扫描。

## movable_ops 页迁移


选定的有类型、非 folio 的页（例如，在内存气球中膨胀的页、zsmalloc 页）可以使用 movable_ops 迁移框架进行迁移。

“struct movable_operations” 提供特定于页类型的回调，用于隔离、迁移和取消隔离（putback）这些页。

一旦一个页被指示具有 movable_ops，在该页被释放回 buddy 之前，该条件不得改变。这包括不更改/清除页类型，也不更改/清除 PG_movable_ops 页标志。

任意驱动目前无法使用此框架，因为它要求：

(a) 一个页类型
(b) 基于页类型在 page_has_movable_ops() 中指示它们可能具有 movable_ops
(c) 基于页类型从 page_movable_ops() 返回 movable_ops
(d) 不为其他目的重用 PG_movable_ops 和 PG_movable_ops_isolated 页标志

例如，气球驱动可以通过位于核心内核中的 balloon-compaction 基础设施使用此框架。

## 监控迁移


可以使用以下事件（计数器）来监控页迁移。

1. PGMIGRATE_SUCCESS：普通页迁移成功。每次计数表示迁移了一个页。如果该页是非 THP 且非 hugetlb 页，则该计数器加一。如果该页是 THP 或 hugetlb，则该计数器按 THP 或 hugetlb 子页数增加。例如，迁移一个具有 4KB 大小基页（子页）的单个 2MB THP 会使该计数器增加 512。

2. PGMIGRATE_FAIL：普通页迁移失败。与上述 PGMIGRATE_SUCCESS 相同的计数规则：如果它是 THP 或 hugetlb，则按子页数增加。

3. THP_MIGRATION_SUCCESS：一个 THP 在未拆分的情况下被迁移。

4. THP_MIGRATION_FAIL：一个 THP 既无法迁移也无法拆分。

5. THP_MIGRATION_SPLIT：一个 THP 被迁移了，但不是作为整体迁移的：首先，该 THP 必须被拆分。拆分后，对其子页使用了迁移重试。

THP_MIGRATION_* 事件也会更新相应的 PGMIGRATE_SUCCESS 或 PGMIGRATE_FAIL 事件。例如，THP 迁移失败将导致 THP_MIGRATION_FAIL 和 PGMIGRATE_FAIL 都增加。

Christoph Lameter，2006 年 5 月 8 日。
Minchan Kim，2016 年 3 月 28 日。
