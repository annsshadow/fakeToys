## /proc/sys/vm/ 文档说明


Linux 内核版本 2.6.29

Copyright (c) 1998, 1999,  Rik van Riel <riel@nl.linux.org>

Copyright (c) 2008         Peter W. Morreale <pmorreale@novell.com>

有关一般信息与法律声明，请参阅 index.rst
------------------------------------------------------------------------------

本文件包/proc/sys/vm sysctl 文件的文档，适用Linux 内核版本 2.6.29
该目录下的文件可用于调优 Linux 内核的虚拟内存（VM）子系统的运行，以及脏数据写回磁盘的行为
这些文件的大多数默认值和初始化例程可mm/swap.c 中找到
目前，这些文件位/proc/sys/vm
- admin_reserve_kbytes
- compact_memory
- compaction_proactiveness
- compact_unevictable_allowed
- defrag_mode
- dirty_background_bytes
- dirty_background_ratio
- dirty_bytes
- dirty_expire_centisecs
- dirty_ratio
- dirtytime_expire_seconds
- dirty_writeback_centisecs
- drop_caches
- enable_soft_offline
- extfrag_threshold
- highmem_is_dirtyable
- hugetlb_shm_group
- legacy_va_layout
- lowmem_reserve_ratio
- max_map_count
- mem_profiling         (only if CONFIG_MEM_ALLOC_PROFILING=y)
- memory_failure_early_kill
- memory_failure_recovery
- min_free_kbytes
- min_slab_ratio
- min_unmapped_ratio
- mmap_min_addr
- mmap_rnd_bits
- mmap_rnd_compat_bits
- movable_gigantic_pages
- nr_hugepages
- nr_hugepages_mempolicy
- nr_overcommit_hugepages
- nr_trim_pages         (only if CONFIG_MMU=n)
- numa_zonelist_order
- oom_dump_tasks
- oom_kill_allocating_task
- overcommit_kbytes
- overcommit_memory
- overcommit_ratio
- page-cluster
- page_lock_unfairness
- panic_on_oom
- percpu_pagelist_high_fraction
- stat_interval
- stat_refresh
- numa_stat
- swappiness
- unprivileged_userfaultfd
- user_reserve_kbytes
- vfs_cache_pressure
- vfs_cache_pressure_denom
- watermark_boost_factor
- watermark_scale_factor
- zone_reclaim_mode


## admin_reserve_kbytes


系统中应预留给具cap_sys_admin 能力的用户的空闲内存大小
admin_reserve_kbytes 默认值为 min(空闲页的 3%, 8MB)

这应当足以让管理员在默认overcommit "guess"（猜测）模式下登录并杀死进程（如有必要）
运行overcommit "never"（从不）模式下的系统应增大该值，以覆盖用于恢复的程序的完整虚拟内存大小（Virtual Memory Size）。否则，root 可能无法登录以恢复系统
如何计算一个最小有用预留量
sshd login + bash（或某种其他 shell top（或 ps、kill 等）

对于 overcommit "guess"，我们可以对常驻集大小（RSS）求和。在 x86_64 上约8MB
对于 overcommit "never"，我们可以取它们的虚拟大小（VSZ）的最大值，再加上它们的 RSS 之和。在 x86_64 上约128MB
更改此值会在应用程序请求内存时立即生效

## compact_memory


仅当设置CONFIG_COMPACTION 时可用。向该文件写1 时，所zone 都会被压缩，以便尽可能以连续块的形式提供空闲内存。例如，这对大页（huge pages）的分配可能很重要，不过进程在需要时也会直接压缩内存
## compaction_proactiveness


该可调参数取值在 [0, 100] 范围内，默认值为 20。该参数决定后台压缩的激进程度。向该参数写入非零值会立即触发主动压缩（proactive compaction）。将其设0 则禁用主动压缩
注意，压缩会产生非平凡的系统级影响，因为属于不同进程的页会被移动，这也可能导致无辜应用程序出现延迟尖峰。内核采用多种启发式方法，如果检测到主动压缩没有效果，则避免浪费 CPU 周期
将值设80 以上，除了降低可接受的碎片水平外，还会使压缩代码对碎片增加更加敏感，即压缩会触发得更频繁，但每次减少的碎片量更小。这使得碎片水平随时间更加稳定
将其设为 100 之类的极端值时要小心，因为这可能导致过多的后台压缩活动
## compact_unevictable_allowed


仅当设置CONFIG_COMPACTION 时可用。设1 时，允许压缩检查不可回收的 lru（mlocked 页）以寻找可压缩的页。这应当用于那些愿意用次要缺页（minor page fault）停顿来换取大量连续空闲内存的系统。设0 可阻止压缩移动不可回收的页。默认值为 1。在 CONFIG_PREEMPT_RT 上默认值为 0，以避免因压缩导致的缺页而阻塞任务，直到缺页被解决，任务才能变为活动状态
## defrag_mode


设为 1 时，页分配器会更加努力地避免碎片，并保持生成大页/高阶页的能力
建议在启动后立即启用，因为碎片一旦发生，可能会长期存在甚至永久存在
## dirty_background_bytes


包含后台内核回写线程（flusher threads）将开始写回脏内存的量
注意  dirty_background_bytes dirty_background_ratio 的对应项。两者一次只能指定其一。写入某sysctl 时会立即计入以评估脏内存限制，而另一个在被读取时显示0

## dirty_background_ratio


以包含空闲页和可回收页的总可用内存的百分比形式，包含后台内核回写线程将开始写出脏数据的页数
总可用内存不等于系统总内存

## dirty_bytes


包含进程生成磁盘写入时将自行开始写回的脏内存量
注意：dirty_bytes dirty_ratio 的对应项。一次只能指定其一。写入某sysctl 时会立即计入以评估脏内存限制，而另一个在读时被显示为 0
注意：dirty_bytes 允许的最小值为两页（以字节计）；任何低于此限制的值都将被忽略，并保留旧的配置

## dirty_expire_centisecs


该可调参数用于定义脏数据足够"陈旧"、有资格被内核回写线程写出的时间。它以百分之一秒表示。在内存中变脏时间超过此间隔的数据将在回写线程下次唤醒时被写出

## dirty_ratio


以包含空闲页和可回收页的总可用内存的百分比形式，包含生成磁盘写入的进程将自行开始写出脏数据的页数
总可用内存不等于系统总内存

## dirtytime_expire_seconds


当一lazytime inode 持续不断地被弄脏时，带有更新时间戳的 inode 将永远不会有机会被写出。而且，如果文件系统上唯一发生的事情是atime 更新引起dirtytime inode，则会调度一worker 来确保该 inode 最终被推送到磁盘。该可调参数用于定义inode 足够陈旧、有资格被内核回写线程写回的时间。并且，它也用作唤醒 dirtytime_writeback 线程的间隔
将其设为 0 会禁用周期dirtytime 写回

## dirty_writeback_centisecs


内核回写线程会周期性唤醒并`old` 数据写出到磁盘。该可调参数以百分之一秒表示这些唤醒之间的间隔
将其设为 0 将完全禁用周期性写回

## drop_caches


写入该文件将导致内核丢弃干净的缓存，以及可回收的 slab 对象（如 dentries inodes）。一旦丢弃，它们的内存即变为空闲
```

	echo 1 > /proc/sys/vm/drop_caches

```
```
	echo 2 > /proc/sys/vm/drop_caches

```
```
	echo 3 > /proc/sys/vm/drop_caches

```
这是非破坏性操作，不会释放任何脏对象。为了增加此操作释放的对象数量，用户可以在写/proc/sys/vm/drop_caches 之前运行 `sync`。这将尽量减少系统上的脏对象数量，并产生更多可被丢弃的候选对象
该文件并不是用于控制各种内核缓存（inodes、dentries、pagecache 等）增长的方法。当系统其他地方需要内存时，这些对象会被内核自动回收
使用此文件可能导致性能问题。由于它丢弃了缓存对象，重新创建被丢弃的对象可能需要消耗大量的 I/O CPU，尤其是在它们被大量使用的情况下。因此，不建议在测试或调试环境之外使用
当该文件被使用时，您可能会在内核日志中看到如下信息性消息：

```
	cat (1234): drop_caches: 3

```
这些仅作为信息提示。它们并不意味着您的系统有任何问题。要禁用它们，向 drop_caches 写入 4（第 2 位）
## enable_soft_offline

可纠正的内存错误在服务器上非常常见。软离线（Soft-offline）是内核针对存在（过多）已纠正内存错误的内存页提供的解决方案
对于不同类型的页，软离线有不同的行为/开销
- 对于原始错误页，软离线将正在使用的页内容迁移到一个新的原始页
- 对于作为透明大页（transparent hugepage）一部分的页，软离线先将透明大页拆分为原始页，然后仅迁移原始错误页。结果，用户透明地少1 个大页支持，影响内存访问性能
- 对于作为 HugeTLB 大页一部分的页，软离线首先迁移整个 HugeTLB 大页，期间会消耗一个空闲大页作为迁移目标。然后原始大页被无补偿地溶解为原始页，使 HugeTLB 池容量减1
在透明HugeTLB 情况下，用户需要权衡可靠性（远离脆弱的物理内存）与性能/容量影响
对于所有架构，enable_soft_offline 控制是否对内存页进行软离线。设1 时，内核在认为需要时尝试对页进行软离线。设0 时，内核对软离线页的请求返回 EOPNOTSUPP。其默认值为 1
值得一提的是，在将 enable_soft_offline 设为 0 之后，以下对页软离线的请求将不会被执行：

- 来自 RAS 可纠正错误收集器（Correctable Errors Collector）的软离线页请求
- ARM 上，来自 GHES 驱动的软离线页请求
- PARISC 上，来自页回收表（Page Deallocation Table）的软离线页请求
## extfrag_threshold


该参数影响内核是压缩内存还是直接回收（direct reclaim）以满足高阶分配。debugfs 中的 extfrag/extfrag_index 文件显示了系统中每个 zone 每个 order 的碎片指数。趋向于 0 的值意味着分配将因内存不足而失败；趋向1000 的值意味着失败是由碎片造成的；-1 意味着只要满足水位线（watermarks），分配就会成功
如果碎片指数 <= extfrag_threshold，内核将不会压缩zone 的内存。默认值为 500

## highmem_is_dirtyable


仅适用于启用了 CONFIG_HIGHMEM2 位系统）的系统
该参数控制是否将高端内存（high memory）计入脏写者限流（dirty writers throttling）。默认情况下并非如此，这意味着只有内核可直接看使用的内存量才能被弄脏。结果，在具有大量内存且 lowmem 基本耗尽的系统上，写者可能被过早限流，流式写入会变得非常慢
将值改为非零将允许更多内存被弄脏，从而允许写者写入更多可被更有效地刷到存储的数据。注意这也带来过早触OOM killer 的风险，因为某些写者（例如直接块设备写入）只能使用低内存，它们可能在没有限流的情况下用脏数据填满低内存

## hugetlb_shm_group


hugetlb_shm_group 包含允许使用 hugetlb 页创SysV 共享内存段的id

## legacy_va_layout


如果非零，该 sysctl 禁用新的 32 mmap 布局——内核将对所有进程使用旧的（2.4）布局

## lowmem_reserve_ratio


对于高端内存（highmem）机器上某些专门的工作负载，内核允许"lowmem" zone 分配进程内存是危险的。这是因为该内存随后可能mlock() 系统调用固定（pinned），或由于缺少交换空间而不可用
在大型高端内存机器上，这种缺少可回收低内存的情况可能是致命的
因此 Linux 页分配器有一种机制，防止本可使用高端内存的分配占用过多低内存。这意味着一定数量的低内存被保护起来，避免被捕获为固定用户内存的可能性
（同样的论点也适用于旧16 兆字ISA DMA 区域。该机制也会保护该区域免受本可使用高端内存或低内存的分配占用。）

`lowmem_reserve_ratio` 可调参数决定内核在保护这些较zone 时的激进程度
如果您有一台使用高端内存或 ISA DMA 的机器，且您的应用程序正在使mlock()，或者您在没有交换的情况下运行，您可能应该更lowmem_reserve_ratio 设置
```

	% cat /proc/sys/vm/lowmem_reserve_ratio
	256     256     32

```
但是，这些值并非直接使用。内核从中为每个 zone 计算保护页数。它们作为保护页数组显示/proc/zoneinfo 中，如下所示（这是 x86-64 机器的示例）```

  Node 0, zone      DMA
    pages free     1355
          min      3
          low      3
          high     4
	:
	:
      numa_other   0
          protection: (0, 2004, 2004, 2004)
	^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    pagesets
      cpu: 0 pcp: 0
          :

```
这些保护值被加到评分中，以判断该 zone 是否应用于页分配，还是应被回收
在此示例中，如果DMA zone 需要普通页（index=2）且使用 watermark[WMARK_HIGH] 作为水位线，内核判断不应使用zone，因pages_free(1355) 小于 watermark + protection[^2^] + 2004 = 2008）。如果此保护值为 0，该 zone 将用于普通页需求。如果需求是 DMA zone（index=0），则使protection[^0^]0）
```

  (i < j):
    zone[i]->protection[j]
    = (node 上从 zone[i+1] zone[j] managed_pages 总和)
      / lowmem_reserve_ratio[i];
  (i = j):
     (不应被保护= 0;
  (i > j):
     (不必要，但看起来0)

```
lowmem_reserve_ratio[i] 的默认值为

    === ====================================
    256 (如果 zone[i] 表示 DMA DMA32 zone)
    32  (其他)
    === ====================================

如上表达式所示，它们是比率的倒数56 表示 1/256。保护页数变为该 node 上较zone 的托管页（managed pages）总数的约 "0.39%"
如果您想保护更多页，较小的值更有效。最小值为 1/1 -> 100%）。小1 的值会完全禁用页保护

## max_map_count


该文件包含一个进程可能拥有的内存映射区域的最大数量。内存映射区域是调用 malloc、通过 mmap、mprotect madvise 直接调用，以及加载共享库的副作用
虽然大多数应用程序需要少于一千个映射，但某些程序（特别是 malloc 调试器）可能会消耗大量映射，例如每次分配最多一两个映射
默认值为 65530

## mem_profiling


启用内存分析（当 CONFIG_MEM_ALLOC_PROFILING=y 时）

1: 启用内存分析
0: 禁用内存分析
启用内存分析会给所有内存分配带来较小的性能开销
默认值取决于 CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT
CONFIG_MEM_ALLOC_PROFILING_DEBUG=y 时，此控件为只读，以避免在禁用分析时进行的分配并在启用时释放所产生的警告

## memory_failure_early_kill


控制在后台由硬件检测到无法被内核处理的未纠正内存错误（通常是内存模块中2 位错误）时如何杀死进程。在某些情况下（例如页在磁盘上仍有有效副本），内核将透明地处理该故障，而不影响任何应用程序。但如果没有其他最新的数据副本，它将杀死进程以防止任何数据损坏传播
1: 一旦检测到损坏，就杀死所有映射了已损坏且不可重载页的进程。注意这对于少数几种类型的页（如内核内部分配的数据或交换缓存）不支持，但对绝大多数用户页有效
0: 仅从所有进程中取消映射损坏的页，并且只杀死试图访问它的进程
杀死动作使用可捕获SIGBUS（带 BUS_MCEERR_AO）完成，因此进程如果愿意可以处理此信号
这仅在具有高级机器检查（machine check）处理能力的架构/平台上激活，并取决于硬件能力
应用程序可以使用 PR_MCE_KILL prctl 单独覆盖此设置

## memory_failure_recovery


启用内存故障恢复（当平台支持时）

1: 尝试恢复
0: 内存故障时总是 panic

## min_free_kbytes


这用于强Linux VM 保留最小数量的千字节空闲。VM 使用此数字为每个系统中的 lowmem zone 计算一watermark[WMARK_MIN] 值。每lowmem zone 根据大小按比例获得一定数量的保留空闲页
需要最少量内存来满PF_MEMALLOC 分配；如果将其设为低1024KB，您的系统将变得微妙地损坏，并在高负载下容易出现死锁
将其设得太高会立即导致您的机OOM

## min_slab_ratio


这仅NUMA 内核上可用
每个 zone 中总页数的一个百分比。在 zone 回收（发生从本地 zone 回退）时，如果某 zone 中可回收 slab 页超过此百分比，则会回收 slab。这确保即使在很少进行全局回收NUMA 系统中，slab 增长也保持受控
默认值为 5%
注意 slab 回收是按 zone/node 方式触发的。回slab 内存的过程目前不node 特定的，且可能不快

## min_unmapped_ratio


这仅NUMA 内核上可用
这是每个 zone 中总页数的一个百分比。仅当某 zone 中可回收状态的页数超过此百分比时，才会发生 zone 回收（具体由 zone_reclaim_mode 允许）
如果 zone_reclaim_mode 的值为 4（按位或），则该百分比与包括交换缓存页和 tmpfs 文件在内的所有文件后备未映射页进行比较。否则，仅考虑由普通文件（而非 tmpfs 文件及类似文件）后备的未映射页
默认值为 1%

## mmap_min_addr


该文件表示用户进程将被限制不mmap 的地址空间大小。由于内核空指针解引用（null dereference）漏洞可能基于内存前几页中的信息意外运行，不应允许用户进程写入它们。默认此值设0，安全模块将不强制执行任何保护。将此值设为诸64k 之类的值将允许绝大多数应用程序正常工作，并提供纵深防御以抵御未来潜在的内核漏洞

## mmap_rnd_bits


该值可用于选择在支持调整地址空间随机化的架构上，用于确定 mmap 分配产生vma 区域基址随机偏移的位数。该值受架构支持的最小值和最大值约束
该值可在启动后使用 /proc/sys/vm/mmap_rnd_bits 可调参数更改

## mmap_rnd_compat_bits


该值可用于选择在支持调整地址空间随机化的架构上，用于确定在兼容模式下运行的应用程mmap 分配产生vma 区域基址随机偏移的位数。该值受架构支持的最小值和最大值约束
该值可在启动后使用 /proc/sys/vm/mmap_rnd_compat_bits 可调参数更改

## movable_gigantic_pages


该参数控制是否可ZONE_MOVABLE 分配巨型页（gigantic pages）。如果设为非零，则可ZONE_MOVABLE 分配巨型页。ZONE_MOVABLE 内存可通过内核启动参数 `kernelcore` 创建，或通过内存热插拔创建，Documentation/admin-guide/mm/memory-hotplug.rst 中所述
支持可能取决于特定架构
注意，使ZONE_MOVABLE 巨型页会使内存热移除（memory hotremove）不可靠
内存热移除操作将无限期阻塞，直到管理员预留足够的巨型页来服务与内存下线过程相关的迁移请求。由HugeTLB 巨型页预留是一个手动过程（通过 `nodeN/hugepages/.../nr_hugepages` 接口），在仅尝试下线一个内存块时这可能并不明显
此外，由于单个块上可能预留多个巨型页，似乎巨型页可用于迁移，而实际上它们正在被移除的过程中。例如，如果 `memoryN` 包含两个巨型页，一个已预留、一个已分配，而管理员尝试下线该块，除非另一`memoryM` 上有另一个已预留的巨型页可用，否则该操作可能无限期挂起

## nr_hugepages


更改大页池的最小大小
请参Documentation/admin-guide/mm/hugetlbpage.rst


## hugetlb_optimize_vmemmap


该旋钮在 'struct page'（include/linux/mm_types.h 中定义的结构）的大小不是 2 的幂时不可用（不寻常的系统配置可能导致此情况）
启用（设1）或禁用（设0）HugeTLB Vmemmap 优化（HVO）
启用后，随后从伙伴分配器（buddy allocator）分配的 HugeTLB 页的 vmemmap 页将被优化（每个 2MB HugeTLB 7 页，每个 1GB HugeTLB 4095 页），而已经分配的 HugeTLB 页不会被优化。当那些被优化的 HugeTLB 页从 HugeTLB 池释放到伙伴分配器时，表示该范围vmemmap 页需要重新映射，并且之前丢弃vmemmap 页需要重新分配。如果您的用例是 HugeTLB 页是"按需"（on the fly）分配的（例如，从不显式使用 'nr_hugepages' 分配 HugeTLB 页，而只设置 'nr_overcommit_hugepages'，那些超额提交的 HugeTLB 页是"按需"分配的），而不是从 HugeTLB 池中取出，您应该权衡内存节省的好处与 HugeTLB 页在 HugeTLB 池和伙伴分配器之间分配或释放的更多开销（比以前慢约 2 倍）。另一个需要注意的行为是，如果系统处于严重的内存压力下，它可能阻止用户HugeTLB 页从 HugeTLB 池释放到伙伴分配器，因为 vmemmap 页的分配可能失败，如果您的系统遇到这种情况，您必须稍后重试
一旦禁用，随后从伙伴分配器分配HugeTLB 页的 vmemmap 页将不被优化，意味着来自伙伴分配器分配时的额外开销消失，而已经优化的 HugeTLB 页不受影响。如果您想确保没有优化的 HugeTLB 页，可以先将 "nr_hugepages" 设为 0，然后再禁用此功能。注意向 nr_hugepages 写入 0 会使任何"使用HugeTLB 页变为盈余页（surplus pages）。因此，这些盈余页在被使用之前仍然被优化。您需要等待这些盈余页被释放，系统中才会没有优化的页

## nr_hugepages_mempolicy


在运行时更改特定一NUMA 节点上的大页池大小
请参Documentation/admin-guide/mm/hugetlbpage.rst


## nr_overcommit_hugepages


更改大页池的最大大小。最大值为 nr_hugepages + nr_overcommit_hugepages
请参Documentation/admin-guide/mm/hugetlbpage.rst


## nr_trim_pages


这仅NOMMU 内核上可用
该值调2 的幂对齐NOMMU mmap 分配的超额页裁剪行为
0 完全禁用分配的裁剪，而1 会激进地裁剪超额页。任>= 1 的值都作为启动分配裁剪的水位线
默认值为 1
有关更多信息，请参阅 Documentation/admin-guide/mm/nommu-mmap.rst

## numa_zonelist_order


sysctl 仅用NUMA，且已弃用。除 Node 顺序之外的任何值都将失败！

"从何处分配内zonelists 控制
（为简单说明，本文档忽ZONE_HIGHMEM/ZONE_DMA32。您可以ZONE_DMA 读作 ZONE_DMA32……）

在非 NUMA 情况下，GFP_KERNEL zonelist 排序如下ZONE_NORMAL -> ZONE_DMA
这意味着GFP_KERNEL 的内存分配请求仅ZONE_NORMAL 不可用时才会ZONE_DMA 获得内存
NUMA 情况下，您可以考虑以下 2 种顺序类型```

  (A) Node(0) ZONE_NORMAL -> Node(0) ZONE_DMA -> Node(1) ZONE_NORMAL
  (B) Node(0) ZONE_NORMAL -> Node(1) ZONE_NORMAL -> Node(0) ZONE_DMA.

```
类型 (A) Node(0) 上的进程提供最佳局部性，ZONE_DMA 会在 ZONE_NORMAL 耗尽之前被使用。这增加ZONE_DMA ZONE_DMA 往往较小而发生内存耗尽（OOM）的可能性
类型 (B) 不能提供最佳局部性，但对 DMA zone OOM 更鲁棒
类型 (A) 称为 "Node" 顺序。类(B) "Zone" 顺序
"Node order" node 排序 zonelists，然后按每个 node 内的 zone 排序。指"[Nn]ode" 表示 node 顺序
"Zone Order" zone 类型排序 zonelists，然后按每个 zone 内的 node 排序。指"[Zz]one" 表示 zone 顺序
指定 "[Dd]efault" 以请求自动配置
32 位上，Normal zone 需要为内核可访问的分配保留，因此将选择 "zone" 顺序
64 位上，需DMA32/DMA 的设备相对较少，因此将选择 "node" 顺序
除非这给您的系统/应用程序带来问题，否则建议使用默认顺序

## oom_dump_tasks


启用在系统范围内生成一个任务转储（不包括内核线程），当内核执行 OOM-killing 时，并包含诸pid、uid、tgid、vm size、rss、pgtables_bytes、swapents、oom_score_adj 分数和名称等信息。这有助于确定为何调用了 OOM killer、识别导致它的恶意任务，以及确定 OOM killer 为何选择它杀死的任务
如果将其设为 0，则抑制此信息。在具有数千个任务的超大型系统上，转储每个任务的内存状态信息可能不可行。此类系统不应被迫在可能不需要该信息时承OOM 条件下的性能损失
如果将其设为非零，则每当 OOM killer 实际杀死一个内存消耗过大的任务时，就会显示此信息
默认值为 1（已启用）

## oom_kill_allocating_task


这启用或禁用在内存不足（out-of-memory）情况下杀死触OOM 的任务
如果将其设为 0，OOM killer 将扫描整个任务列表并根据启发式方法选择一个任务杀死。这通常选择一个消耗大量内存的恶意任务，杀死时会释放大量内存
如果将其设为非零，OOM killer 直接杀死触发内存不足条件的任务。这避免了昂贵的任务列表扫描
如果选择panic_on_oom，它优先oom_kill_allocating_task 中使用的任何值
默认值为 0

## overcommit_kbytes


overcommit_memory 设为 2 时，提交的地址空间不允许超过交换空间加上此量的物理 RAM。见下文
注意：overcommit_kbytes overcommit_ratio 的对应项。一次只能指定其一。设置一个会禁用另一个（另一个在被读时显示为 0）

## overcommit_memory


该值包含一个启用内存超额提交（overcommitment）的标志
当该标志0 时，内核将用户空间内存请求大小与总内存加交换空间进行比较，并拒绝明显的超额提交
当该标志1 时，内核假装总有足够的内存，直到它实际耗尽
当该标志2 时，内核使用"从不超额提交"never overcommit"）策略，试图阻止任何内存超额提交。注user_reserve_kbytes 影响此策略
此特性可能非常有用，因为有很多程malloc() 大量内存"以防万一"却很少使用
默认值为 0
请参Documentation/mm/overcommit-accounting.rst **mm/util.c**
: __vm_enough_memory() 获取更多信息

## overcommit_ratio


overcommit_memory 设为 2 时，提交的地址空间不允许超过交换空间加上此百分比的物理 RAM。见上文

## page-cluster


page-cluster 控制单次尝试中从交换空间连续读入的页数上限。这是交换空间对应页缓存预读（page cache readahead）的对应项。所提到的连续性不是就虚拟/物理地址而言，而是在交换空间上连续——意味着它们是一起被换出的
它是一个对数值——设0 表示 "1 ，设1 表示 "2 ，设2 表示 "4 ，依此类推 完全禁用交换预读
默认值为 3（一8 页）。如果您的工作负载是交换密集型的，将其调为不同的值可能会有一些小的好处
较低的值意味着初始缺页的延迟更低，但同时，如果后续缺页本可以是该连续页预读带入的一部分，则会产生额外的缺页I/O 延迟

## page_lock_unfairness


该值决定页锁可以从等待者手中被窃取的次数。在锁被窃取指定次数（默认为 5）后，将应用"公平锁交（fair lock handoff）语义，并且只有在可以获取锁时才会唤醒等待者
## panic_on_oom


这启用或禁用内存不足panic 的功能
如果将其设为 0，内核将杀死某个称oom_killer 的恶意进程。通常，oom_killer 可以杀死恶意进程，系统将存活
如果将其设为 1，内存不足发生时内核 panic。但是，如果某进程通过 mempolicy/cpusets 限制使用节点，且那些节点变为内存耗尽状态，一个进程可能被 oom-killer 杀死。这种情况下不发panic。因为其他节点的内存可能是空闲的。这意味着系统总体状态可能尚未致命
如果将其设为 2，内核甚至在如上所述情况下也强panic。即使在内存 cgroup 下发oom，整个系统也panic
默认值为 0
1 2 用于集群的故障转移（failover）。请根据您的故障转移策略选择其中之一
panic_on_oom=2+kdump 为您提供了非常强大的工具来调oom 发生的原因。您可以获取快照

## percpu_pagelist_high_fraction


这是可存储到CPU 页列表（per-cpu page lists）的每个 zone 中页数的比例。它是根据在CPU 数量划分的上限。此值的最小值为 8，意味着我们不允许每zone 中超1/8 的页存储在每 CPU 页列表上。该条目仅更改热CPU 页列表的值。用户可以指定诸100 之类的数字，以在每个 zone 1/100 分配给每 CPU 列表之间分配
每个CPU 页列表的批处理值（batch value）无high fraction 的值如何都保持不变，因此分配延迟不受影响
初始值为零。内核使用此值基zone 的低水位线和本地在线 CPU 数量为该 zone 设置 high pcp->high 标记。如果用户向sysctl 写入 '0'，它将恢复为默认行为

## stat_interval


更新 vm 统计数据的时间间隔。默认为 1 秒

## stat_refresh


任何读取或写入（root）都会将所有每 CPU vm 统计数据刷新到它们的全局总计，以便在测试时（例如 cat /proc/sys/vm/stat_refresh /proc/meminfo）获得更准确的报告
作为副作用，它还会检查负总计（在其他地方报告0），如果发现任何负总计，则EINVAL "失败"，并dmesg 中发出警告。（在撰写时，已知少数统计有时会变为负数，没有任何不良影响：这些统计上的错误和警告被抑制。）


## numa_stat


该接口允numa 统计的运行时配置
当页分配性能成为瓶颈，且您可以容忍某些可能的工具损坏和降低的 numa 计数器精度时，您可以
```

	echo 0 > /proc/sys/vm/numa_stat

```
当页分配性能不是瓶颈且您想要所```

	echo 1 > /proc/sys/vm/numa_stat


```
## swappiness


该控件用于定义交换和文件系统分页的粗略相IO 成本，取值介0 200 之间。在 100 时，VM 假设相等IO 成本，因此将内存压力平等地施加于页缓存和交换后备页；较低的值表示交IO 更昂贵，较高的值表示更便宜
请记住，内存压力下的文件系统 IO 模式往往比交换的随机 IO 更高效。最优值需要实验，并且也取决于工作负载
默认值为 60
对于内存中交换（zram zswap），以及交换设备比文件系统更快的混合设置，可以考虑超过 100 的值。例如，如果针对交换设备的随IO 平均比来自文件系统的 IO 2 倍，swappiness 应为 133（x + 2x = 200x = 133.33）
0 时，内核将不会启动交换，直到空闲页和文件后备页的数量小于zone 中的高水位线

## unprivileged_userfaultfd


该标志控制无特权用户可以使用 userfaultfd 系统调用的模式。将其设0 以限制无特权用户仅在用户模式下处理缺页。在这种情况下，没有 SYS_CAP_PTRACE 的用户必须传UFFD_USER_MODE_ONLY，userfaultfd 才能成功。禁止将 userfaultfd 用于处理来自内核模式的缺页可能使某些漏洞更难利用
将其设为 1 以允许无特权用户在没有任何限制的情况下使userfaultfd 系统调用
默认值为 0
另一种控userfaultfd 权限的方法是使用 /dev/userfaultfd 而非 userfaultfd(2)。请参阅 Documentation/admin-guide/mm/userfaultfd.rst
## user_reserve_kbytes


overcommit_memory 设为 2从不超额提交"模式时，预留 min(当前进程大小3%, user_reserve_kbytes) 的空闲内存。这旨在防止用户启动单个消耗大量内存的进程，从而无法恢复（杀死该消耗者）
user_reserve_kbytes 默认min(当前进程大小3%, 128MB)
如果将其减为零，则用户将允许用单个进程分配所有空闲内存，减去 admin_reserve_kbytes。任何后续执行命令的尝试都将导致 "fork: Cannot allocate memory"
更改此值会在应用程序请求内存时生效

## vfs_cache_pressure


该百分比值控制内核回收用于缓存目录和 inode 对象的内存方面的倾向
在默认vfs_cache_pressure=vfs_cache_pressure_denom 时，内核将尝试以相对于页缓存和交换缓存回收的"公平"速率回收 dentries inodes。减vfs_cache_pressure 导致内核倾向于保dentry inode 缓存。当 vfs_cache_pressure=0 时，内核永远不会因内存压力而回dentries inodes，这很容易导致内存不足状况。增vfs_cache_pressure 超过 vfs_cache_pressure_denom 导致内核倾向于回dentries inodes
vfs_cache_pressure 显著增大超过 vfs_cache_pressure_denom 可能有负面性能影响。回收代码需要获取各种锁来找到可释放的目录和 inode 对象。当 vfs_cache_pressure 等于 (10 * vfs_cache_pressure_denom) 时，它将寻找十倍于现有数量的可释放对象
注意：此设置应始终与 vfs_cache_pressure_denom 配合使用
## vfs_cache_pressure_denom


默认100（允许的最小值）。需要相应的 vfs_cache_pressure 设置才能生效
## watermark_boost_factor


该因子控制在内存碎片化时的回收级别。它定义了当不同迁移性的页在页块（pageblocks）内混合时，将回收某 zone 高水位线（high watermark）的百分比。意图是使未来的压缩工作量减少，并提高未来高阶分配（SLUB 分配、THP hugetlbfs 页）的成功率
为了watermark_scale_factor 参数保持一致，单位10,000 分之几。默认15,000 表示在因碎片导致页块混合的事件中，最多回收高水位线的 150%。回收级别由最近发生的碎片事件数量决定。如果此值小于一个页块，则将回收一个页块大小的页（例如 64 x86 上为 2MB）。提升因子为 0 将禁用该特性

## watermark_scale_factor


该因子控kswapd 的激进度。它定义了在节点/系统中保留的内存量，在该量之下唤kswapd，以及需要有多少空闲内存 kswapd 才会重新睡眠
单位10,000 分之几。默认10 表示水位线之间的距离为节系统中可用内存的 0.1%。最大值为 3000，即内存30%
大量线程进入直接回收（allocstall）或 kswapd 过早进入睡眠（kswapd_low_wmark_hit_quickly）可能表kswapd 为延迟原因维护的空闲页数对于系统中发生的分配突发来说太小了。然后可以使用此旋钮相应地调kswapd 激进度

## zone_reclaim_mode


Zone_reclaim_mode 允许某人设置或多或少的激进方法来zone 耗尽内存时回收内存。如果将其设0，则不发zone 回收。分配将由系统中的其zone/node 满足
该值是以下各项的按位或（OR）：

=	===================================
1	Zone reclaim on
2	Zone reclaim writes dirty pages out
4	Zone reclaim swaps pages
=	===================================

zone_reclaim_mode 默认是禁用的。对于文件服务器或受益于数据缓存的工作负载，应保zone_reclaim_mode 禁用状态，因为缓存效果可能比数据局部性更重要
如果已知工作负载是分区的，使得每个分区适合一NUMA 节点内，且访问远程内存会导致可测量的性能下降，则考虑启用一个或多个 zone_reclaim 模式位。页分配器将在分配非本地节点页之前采取额外操作
允许 zone 回收写出页会阻止写入大量数据的进程弄脏其他节点上的页。如果一zone 填满，zone 回收将写出脏页，从而有效地限制该进程。这可能会降低单个进程的性能，因为它不能再使用所有系统内存来缓冲传出写入，但它保留了其他节点上的内存，从而使运行在其他节点上的其他进程的性能不会受到影响
允许常规交换（regular swap）有效地将分配限制到本地节点，除非被内存策略cpuset 配置显式覆盖