## 检查进程页表


pagemap 是内核中一组较新的接口（自 2.6.25 起引入），它允许
用户空间程序通过读取 `/proc` 下的文件来检查页表以及相关信息。

pagemap 包含四个组成部分：

 - `/proc/pid/pagemap`。该文件让一个用户空间进程查找出每个虚拟页映射到
   哪个物理页帧（physical frame）。它为每个虚拟页包含一个 64 位的值，其中
   含有如下数据（来自 `fs/proc/task_mmu.c`，pagemap_read 之上）：

    - Bit 0-54  页帧号（PFN，page frame number），若存在
    - Bit 0-4   交换类型（swap type），若已换出
    - Bit 5-54  交换偏移（swap offset），若已换出
    - Bit 55    pte 为软脏（soft-dirty）（参见
      Documentation/admin-guide/mm/soft-dirty.rst）
    - Bit 56    页被独占映射（自 4.2 起）
    - Bit 57    pte 被 uffd-wp 写保护（自 5.13 起）（参见
      Documentation/admin-guide/mm/userfaultfd.rst）
    - Bit 58    pte 是一个保护区域（guard region）（自 6.15 起）（参见 madvise (2) 手册页）
    - Bit 59-60 全零
    - Bit 61    页为文件页或共享匿名页（自 3.5 起）
    - Bit 62    页已换出
    - Bit 63    页存在

   自 Linux 4.0 起，只有拥有 CAP_SYS_ADMIN 能力的用户才能获取 PFN。在 4.0 和
   4.1 中，无特权用户的打开操作会以 -EPERM 失败。从 4.2 开始，如果用户没有
   CAP_SYS_ADMIN，PFN 字段会被清零。原因：关于 PFN 的信息有助于利用 Rowhammer
   漏洞。

   如果页不存在但位于交换区中，则 PFN 包含交换文件编号以及该页在交换区中偏移量的
   编码。未映射的页返回空 PFN。这样可以精确地判断哪些页被映射（或位于交换区中），
   并在进程之间比较被映射的页。

   传统上，bit 56 表示一页恰好被映射一次，而当一页被多次映射时（即使在同一进程中被
   多次映射），bit 56 会被清除。在某些内核配置中，对于属于较大分配（如 THP）的页，
   其语义可能不同：如果对应大分配的所有页**确实**都映射在同一进程中，即使该页在该
   进程中被映射多次，也会设置 bit 56。当大分配中的任一页**可能**映射在另一个进程时，
   bit 56 会被清除。在某些情况下，一个大分配可能会被视为"可能被多个进程映射"，即使
   实际情况已不再如此。

   该接口的高效使用者会利用 `/proc/pid/maps` 来确定内存中实际被映射的区域，并使用
   llseek 跳过未映射的区域。

 - `/proc/kpagecount`。该文件包含一个 64 位的计数，表示每个页被映射的次数，以 PFN
   为索引。某些内核配置不会精确跟踪属于较大分配（如 THP）的页被映射的次数。在这些
   配置中，会返回该大分配中每页映射次数的平均值；但只要大分配中的任一页被映射，返回
   值就至少为 1。

tools/mm 目录下的 page-types 工具可用于查询一个页被映射的次数。

 - `/proc/kpageflags`。该文件为每个页包含一个 64 位的标志集合，以 PFN 为索引。

   这些标志如下（来自 `fs/proc/page.c`，kpageflags_read 之上）：

    0. LOCKED
    1. ERROR
    2. REFERENCED
    3. UPTODATE
    4. DIRTY
    5. LRU
    6. ACTIVE
    7. SLAB
    8. WRITEBACK
    9. RECLAIM
    10. BUDDY
    11. MMAP
    12. ANON
    13. SWAPCACHE
    14. SWAPBACKED
    15. COMPOUND_HEAD
    16. COMPOUND_TAIL
    17. HUGE
    18. UNEVICTABLE
    19. HWPOISON
    20. NOPAGE
    21. KSM
    22. THP
    23. OFFLINE
    24. ZERO_PAGE
    25. IDLE
    26. PGTABLE

 - `/proc/kpagecgroup`。该文件包含每个页所计入（charged）的内存 cgroup 的 64 位
   inode 编号，以 PFN 为索引。仅当设置了 CONFIG_MEMCG 时才可用。

## 页标志简述


0 - LOCKED
   该页正被锁定以进行独占访问，例如正在进行读/写 IO。

7 - SLAB
   该页由 SLAB/SLUB 内核内存分配器管理。
   使用复合页（compound page）时，二者都只会在头页上设置该标志。

10 - BUDDY
   由伙伴系统分配器管理的空闲内存块。
   伙伴系统将空闲内存组织为不同阶（order）的块。
   一个阶为 N 的块包含 2^N 个物理连续的页，所有页都设置了 BUDDY 标志。
   在 4.6 之前，只有该块的第一个页设置了该标志。

15 - COMPOUND_HEAD
   一个阶为 N 的复合页由 2^N 个物理连续的页组成。
   一个阶为 2 的复合页形如 "HTTT"，其中 H 表示其头页，T 表示其尾页。
   复合页的主要使用者有 hugeTLB 页（Documentation/admin-guide/mm/hugetlbpage.rst）、
   SLUB 等内存分配器以及各种设备驱动。
   但在该接口中，只有 huge/giga 页对最终用户可见。

16 - COMPOUND_TAIL
   复合页的尾页（参见上面的描述）。

17 - HUGE
   这是 HugeTLB 页的组成部分。

19 - HWPOISON
   该页上被硬件检测到内存损坏：不要碰这些数据！

20 - NOPAGE
   在请求地址处不存在页帧。

21 - KSM
   在一个或多个进程之间动态共享的相同内存页。

22 - THP
   构成任意大小的 THP 并以任意粒度映射的连续页。

23 - OFFLINE
   该页在逻辑上已离线。

24 - ZERO_PAGE
   用于 pfn_zero 或 huge_zero 的零页。

25 - IDLE
   该页自被标记为 idle 以来尚未被访问（参见
   Documentation/admin-guide/mm/idle_page_tracking.rst）。
   注意，如果该页是通过 PTE 访问的，该标志可能会过时。为确保标志是最新的，需要先读取
   `/sys/kernel/mm/page_idle/bitmap`。

26 - PGTABLE
   该页正被用作页表。

### 与 IO 相关的页标志


1 - ERROR
   发生了 IO 错误。

3 - UPTODATE
   该页含有最新的数据。
   即，对于文件后备页：（内存中数据版本 >= 磁盘上版本）

4 - DIRTY
   该页已被写入，因而含有新数据。
   即，对于文件后备页：（内存中数据版本 > 磁盘上版本）

8 - WRITEBACK
   该页正被同步到磁盘。

### 与 LRU 相关的页标志


5 - LRU
   该页位于某个 LRU 链表中。

6 - ACTIVE
   该页位于活跃的 LRU 链表中。

18 - UNEVICTABLE
   该页位于不可回收（非）LRU 链表中。它以某种方式被钉住，不是 LRU 页回收的候选对象，
   例如 ramfs 页、shmctl(SHM_LOCK) 以及 mlock() 内存段。

2 - REFERENCED
   自上次入队/重新入队 LRU 链表以来，该页已被引用。

9 - RECLAIM
   在其页换出 IO 完成后，该页将很快被回收。

11 - MMAP
   一个内存映射页。

12 - ANON
   一个不属于文件的内存映射页。

13 - SWAPCACHE
   该页被映射到交换空间，即拥有一个关联的交换项（swap entry）。

14 - SWAPBACKED
   该页由交换空间/RAM 支持。

tools/mm 目录下的 page-types 工具可用于查询上述标志。

## 共享内存的例外情况


当共享页被 zap 或换出时，其页表项会被清除。这使得被换出的页与从未分配的页无法区分。

在内核空间，仍然可以从页缓存（page cache）中取回交换位置。但是，仅存储在普通 PTE
上的值在页被换出时会永久丢失（即 SOFT_DIRTY）。

在用户空间，该页是存在、已换出还是无，可以借助 lseek 和/或 mincore 系统调用来推断。

lseek() 可以通过在页所后备的文件上指定 SEEK_DATA 标志，来区分被访问过的页
（存在或已换出）与空洞（无/未分配）。对于匿名共享页，该文件可以在
`/proc/pid/map_files/` 中找到。

mincore() 可以区分位于内存中的页（存在，包括交换缓存）和不在内存中的页
（已换出或无/未分配）。

## 其他注意事项


如果读取不是从 8 字节边界处开始（例如，你 seek 到文件中奇数个字节处），或者读取的
大小不是 8 字节的整数倍，那么从任何这些文件中读取都会返回 -EINVAL。

在 Linux 3.11 之前，pagemap 的 bit 55-60 用于 "page-shift"（在大多数架构上始终为 12）。
自 Linux 3.11 起，其含义在首次清除软脏位后发生变化。自 Linux 4.2 起，它们被无条件地
用于标志。

## Pagemap Scan IOCTL


pagemap 文件上的 `PAGEMAP_SCAN` IOCTL 可用于获取或（可选地）清除关于页表项的
信息。该 IOCTL 支持以下操作：

- 扫描地址范围并获取与所提供条件匹配的内存范围。这在进行指定输出缓冲区时执行。

- 写保护这些页。使用 `PM_SCAN_WP_MATCHING` 来写保护感兴趣的页。`PM_SCAN_CHECK_WPASYNC`
  在发现非异步写保护的页时中止操作。`PM_SCAN_WP_MATCHING` 可以配合或不配合
  `PM_SCAN_CHECK_WPASYNC` 使用。

- 这两个操作可以组合成一个原子操作，在该操作中可以同时获取并写保护这些页。

当前支持以下关于页的标志：

- `PAGE_IS_WPALLOWED` - 页已启用异步写保护
- `PAGE_IS_WRITTEN` - 页自被写保护以来已被写入
- `PAGE_IS_FILE` - 页由文件后备
- `PAGE_IS_PRESENT` - 页存在于内存中
- `PAGE_IS_SWAPPED` - 页已换出
- `PAGE_IS_PFNZERO` - 页的 PFN 为零
- `PAGE_IS_HUGE` - 页由 PMD 映射的 THP 或 Hugetlb 后备
- `PAGE_IS_SOFT_DIRTY` - 页为软脏
- `PAGE_IS_GUARD` - 页是保护区域的一部分

`struct pm_scan_arg` 被用作该 IOCTL 的参数。

 1. `struct pm_scan_arg` 的大小必须在 `size` 字段中指定。如果以后进行了扩展，该字段
    有助于识别结构体。

 2. 标志可以在 `flags` 字段中指定。目前仅添加了 `PM_SCAN_WP_MATCHING` 和
    `PM_SCAN_CHECK_WPASYNC` 两个标志。是否执行获取操作取决于是否提供了输出缓冲区。

 3. 范围通过 `start` 和 `end` 指定。

 4. 遍历可能在该完整范围被访问之前中止，例如用户缓冲区可能已满等情况。遍历结束地址
    在 `end_walk` 中指定。

 5. `struct page_region` 数组的输出缓冲区及其大小在 `vec` 和 `vec_len` 中指定。

 6. 可选的最大请求页数在 `max_pages` 中指定。

 7. 掩码在 `category_mask`、`category_anyof_mask`、`category_inverted` 和
    `return_mask` 中指定。

```

   struct pm_scan_arg arg = {
   .size = sizeof(arg),
   .flags = PM_SCAN_CHECK_WPASYNC | PM_SCAN_CHECK_WPASYNC,
   ..
   .category_mask = PAGE_IS_WRITTEN,
   .return_mask = PAGE_IS_WRITTEN,
   };

```
查找已被写入、由文件后备、未换出且（满足下列任一条件）的页：
```

   struct pm_scan_arg arg = {
   .size = sizeof(arg),
   .flags = 0,
   ..
   .category_mask = PAGE_IS_WRITTEN | PAGE_IS_SWAPPED,
   .category_inverted = PAGE_IS_SWAPPED,
   .category_anyof_mask = PAGE_IS_PRESENT | PAGE_IS_HUGE,
   .return_mask = PAGE_IS_WRITTEN | PAGE_IS_SWAPPED |
                  PAGE_IS_PRESENT | PAGE_IS_HUGE,
   };

```
`PAGE_IS_WRITTEN` 标志可被视为比软脏标志性能更好的替代方案。它不受内核 VMA 合并
的影响，因此在普通页的情况下，用户可以找到真正的软脏页。（对于 THP 或 Hugetlb 页
仍可能报告额外的脏页。）

"PAGE_IS_WRITTEN" 类别与启用了 uffd 写保护的范围配合使用，以在用户空间实现内存脏
跟踪：

 1. 通过 `userfaultfd` 系统调用创建 userfaultfd 文件描述符。

 2. 通过 `UFFDIO_API` IOCTL 设置 `UFFD_FEATURE_WP_UNPOPULATED` 和
    `UFFD_FEATURE_WP_ASYNC` 特性。

 3. 通过 `UFFDIO_REGISTER` IOCTL 以 `UFFDIO_REGISTER_MODE_WP` 模式注册内存范围。

 4. 然后，必须使用带 `PM_SCAN_WP_MATCHING` 标志的 `PAGEMAP_SCAN` IOCTL，或者可以使用
    `UFFDIO_WRITEPROTECT` IOCTL，来写保护已注册内存的任何部分或整个内存区域。两者执行
    相同的操作，前者在性能上更优。

 5. 现在可以使用 `PAGEMAP_SCAN` IOCTL 来仅查找自上次标记以来已被写入的页，和/或可选地
    同时写保护这些页。

