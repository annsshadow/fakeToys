## HugeTLB 椤。

## 概述


本文档旨在简要概Linux 内核中对 hugetlbpage（大页）的支持。该支持构建于大多数
现代架构所提供的多页面大小支持之上。例如，x86 CPU 通常支持 4K 2M（如果架支持则为 1G）页面大小，ia64 架构支持多种页面大小 4KK4K56KMM6M256M，ppc64 支持 4K 16M。TLB 是虚拟到物理地址转换的缓存。通常这是处理器上
非常稀缺的资源。操作系统试图最佳地利用数量有限TLB 资源。随着越来越大（数GB）的物理内存变得更容易获得，这种优化现在显得更为关键
用户可以通过使用 mmap 系统调用或标准的 SYSV 共享内存系统调用（shmget、shmat）来
使用 Linux 内核中的大页支持
首先，Linux 内核需要使CONFIG_HUGETLBFS（位“File systems项下）和
CONFIG_HUGETLB_PAGE（选择 CONFIG_HUGETLBFS 时自动选中）配置选项来构建
`/proc/meminfo` 文件提供有关内核大页池中持久 hugetlb 页总数的信息。它还显示默大页大小，以及与默认大小大页池中空闲、保留和盈余大页数量相关的信息生成映射大页区域的系统调用参数的正确对齐方式和大小需要用到该大页大小
```

	HugePages_Total: uuu
	HugePages_Free:  vvv
	HugePages_Rsvd:  www
	HugePages_Surp:  xxx
	Hugepagesize:    yyy kB
	Hugetlb:         zzz kB

```
其中
HugePages_Total
	是大页池的大小HugePages_Free
	是池中尚未分配的大页数量HugePages_Rsvd
	“reserved”（保留）的缩写，表示已经承诺从池中分配但尚未实际分配的
	大页数量。保留的大页保证了应用程序在缺页时能够从大页池中分配到一	大页HugePages_Surp
	“surplus”（盈余）的缩写，表示池中超`/proc/sys/vm/nr_hugepages`
	值的大页数量。盈余大页的最大数量由 `/proc/sys/vm/nr_overcommit_hugepages`
	控制	注意：当启用释放与每hugetlb 页关联的未使vmemmap 页的特性时，在
	系统内存紧张的情况下，盈余大页的数量可能会暂时大于盈余大页的最大数量Hugepagesize
	是默认的大页大小（以 kB 为单位）Hugetlb
	是所有大小的大页所消耗的内存总量（以 kB 为单位）	如果使用了不同大小的大页，该数值将超过 HugePages_Total \* Hugepagesize	要获取更详细的信息，请参`/sys/kernel/mm/hugepages`（如下所述）

`/proc/filesystems` 也应显示一个在内核中配置好“hugetlbfs类型文件系统
`/proc/sys/vm/nr_hugepages` 表示内核大页池中当前 “persistent”（持久）大页的数量“持久大页在被任务释放时会返回到大页池。具root 权限的用户可以通过增大或减`nr_hugepages` 的值来动态分配更多或释放一些持久大页
注意：当启用释放与每hugetlb 页关联的未使vmemmap 页的特性时，在系统内存紧张
的情况下，我们可能无法释放由用户触发释放的大页。请稍后重试
用作大页的页面在内核中保留，不能用于其他目的。在内存压力下，大页不能被换出
一旦一定数量的预分配大页被放入内核大页池，具有相应权限的用户就可以使用 mmap 系统
调用或共享内存系统调用来使用这些大页。请参阅下面关于使用大页 <using_huge_pages>
的讨论
管理员可以通过在内核启动命令行上指“hugepages=N参数来分配持久大页，其中 'N' =
所请求的大页数量。这是分配大页最可靠的方法，因为此时内存尚未变得碎片化
某些平台支持多种大页大小。要分配特定大小的大页，必须在用于大页的启动命令行参之前加上大页大小选择参数 “hugepagesz=<size>”size> 必须以字节为单位指定，并
可带可选的比例后缀 [kKmMgG]。可以使“default_hugepagesz=<size>启动参数选择
默认大页大小
Hugetlb 启动命令行参数语
hugepagesz
	指定一个大的页大小。与 hugepages 参数配合使用以预分配指定大小	若干大页。因此，hugepagesz hugepages 通常成对指定```

		hugepagesz=2M hugepages=512

	hugepagesz 在命令行上只能针对特定大页大小指定一次。有效的大页大小
	取决于架构```
hugepages
	指定要预分配的大页数量。它通常跟在一个有效的 hugepagesz 	default_hugepagesz 参数之后。但是，如果 hugepages 是第一个或唯一	hugetlb 命令行参数，它会隐式指定要分配的默认大小大页的数量。如	隐式指定了默认大小大页的数量，则它不能被针对默认大小	hugepagesz,hugepages 参数对覆盖。该参数还有一个节点格式。节点格	指定要在特定节点上分配的大页数量
```

		hugepages=256 hugepagesz=2M hugepages=512

	将导致分256 2M 大页，并发出警告信息，指hugepages=512 参数
	被忽略。如hugepages 参数前面是一个无效的 hugepagesz 参数，它将被
	忽略
	Node format example::

		hugepagesz=2M hugepages=0:1,1:2

	它将node0 上分1 2M 大页，在 node1 上分2 2M 大页	如果节点号无效，该参数将被忽略```
hugepage_alloc_threads
	指定在启动期间应该用于分配大页的线程数量。当分配大量大页时，可以使用
	此参数来改善系统启动时间
	默认值是可用硬件线程数的 25%```

		hugepage_alloc_threads=8

	注意，此参数仅适用于非巨型（non-gigantic）大页```
default_hugepagesz
	指定默认的大页大小。此参数在命令行上只能指定一次。default_hugepagesz
	可选地后跟 hugepages 参数，以预分配指定数量的默认大小大页。默认大	大页的预分配数量也可以如上在 hugepages 一节中提到的那样隐式指定。因此，```

		hugepages=256
		default_hugepagesz=2M hugepages=256
		hugepages=256 default_hugepagesz=2M

	都会导致分配 256 2M 大页。有效的默认大页大小取决于架构```
hugetlb_free_vmemmap
	当设置了 CONFIG_HUGETLB_PAGE_OPTIMIZE_VMEMMAP 时，这会启用 HugeTLB
	Vmemmap 优化（HVO）
当支持多种大页大小时，`/proc/sys/vm/nr_hugepages` 表示当前预分配的默认大小大页数量。因此，可以使用以下命令动态分释放
```

	echo 20 > /proc/sys/vm/nr_hugepages

```
此命令将尝试把默认大小大页池中的大页数量调整20，根据需要分配或释放大页
NUMA 平台上，内核会尝试将大页池分布到修改 `nr_hugepages` 的任务的 NUMA 内存策略
所指定的所有允许节点集合上。当任务使用默认内存策略时，允许节点的默认值是所有带内存的在线节点。在为某个大页分配时，可用连续内存不足的允许节点将被静默跳过。请参阅
下面 <mem_policy_and_hp_alloc> 关于任务内存策略、cpusets 和每节点属性与大页的分和释放之间交互的讨论
大页分配的成功或失败取决于分配尝试时刻系统中存在的物理连续内存的数量。如果内核无NUMA 系统中的某些节点分配大页，它将尝试通过在具有足够可用连续内存的其他节点分配额外的页来弥补差额（如果有的话）
系统管理员可能希望将此命令放在某个本rc 初始化文件中。这将使内核能够在启动过程的
早期分配大页，此时获得物理连续页的可能性仍然很高。管理员可以通过检sysctl meminfo 来验证实际分配的大页数量。要检查每节点的情```

	cat /sys/devices/system/node/node*/meminfo | fgrep Huge

```
`/proc/sys/vm/nr_overcommit_hugepages` 指定了大页池可以增长到多大，如果应用程序请求大页数量超过`/proc/sys/vm/nr_hugepages`。向此文件写入任何非零值表示，当持久大页池
耗尽时，允许 hugetlb 子系统尝试从内核的普通页池中获取该数量的 “surplus”（盈余）大页当这些盈余大页变为未使用时，它们会被释放回内核的普通页池
通过 `nr_hugepages` 增大大页池大小时，任何现有的盈余页将首先被提升为持久大页。然后，
如有必要且可能，将分配额外的大页，以满足新的持久大页池大小
管理员可以通过`nr_hugepages` sysctl 设置为较小的值来缩减默认大页大小的持久大页池内核将尝试在修改 `nr_hugepages` 的任务的内存策略中的所有节点之间平衡大页的释放。所节点上的任何空闲大页都将被释放回内核的普通页池
注意：通过 `nr_hugepages` 缩减持久大页池，使其小于正在使用的大页数量，会将正在使用
的大页的余额转换为盈余大页。即使盈余页的数量会超过 overcommit 值，这种情况也会发生只要此条件成立——即直到 `nr_hugepages+nr_overcommit_hugepages` 被充分增大，或者盈大页不再使用并被释放——就不再允许分配更多的盈余大页
随着运行时支持多个大页池，位`/proc/sys/vm` 中的大量大页用户空间接口已被复制sysfs 中。上面讨论的 `/proc` 接口已被保留以向后兼```

	/sys/kernel/mm/hugepages

```
对于运行内核支持的每种大页大小，都有一个子目录
```

	hugepages-${size}kB

```
在这些目录中的每一个内部，都会存在 `/proc` 中包含的文件集合。此外，还有两个用于
降级（demote）大页的额外接口
```

        demote
        demote_size
	nr_hugepages
	nr_hugepages_mempolicy
	nr_overcommit_hugepages
	free_hugepages
	resv_hugepages
	surplus_hugepages

```
demote 接口提供了将一个大页拆分为更小大页的能力。例如，x86 架构同时支持 1GB 2MB
大页大小。一1GB 大页可以被拆分为 512 2MB 大页。对于最小的大页大小，demote
接口不可用。demote 接口如下
demote_size
	是被降级页的大小。当一个页被降级时，将创建相应数量demote_size 大页	默认情况下，demote_size 被设置为次小的大页大小。如果存在多个更小的大页大小	demote_size 可以设置为这些更小大小中的任何一个。只允许小于当前大页大小	大页大小
demote
	用于降级一定数量的大页。具root 权限的用户可以写入此文件。可能无法降级所
	请求的那么大数量的大页。要确定实际降级了多少页，请比较写入 demote 接口之前
	和之后的 nr_hugepages 值。demote 是一个只写接口
`/proc` 中相同的接口（除 demote demote_size 之外的所有接口）的功能与上述
默认大页大小情况下的描述相同

## 任务内存策略与大页分释放的交

无论大页是通过 `/proc` 接口还是通过 `/sysfs` 接口并使`nr_hugepages_mempolicy`
属性分配和释放，从中分配或释放大页NUMA 节点都由修改 `nr_hugepages_mempolicy`
sysctl 或属性的任务的内存策略控制。当使用 `nr_hugepages` 属性时，会忽略 mempolicy
推荐用于从内核分配或释放大页的方```

    numactl --interleave <node-list> echo 20 \
				>/proc/sys/vm/nr_hugepages_mempolicy

```
```

    numactl -m <node-list> echo 20 >/proc/sys/vm/nr_hugepages_mempolicy

```
这将根据持久大页数量最初是小于还是大于 20，向 <node-list> 中指定的节点分配或释`abs(20 - nr_hugepages)` 个大页。在所指定<node-list> 之外的任何节点上都不会分或释放大页
通过 `nr_hugepages_mempolicy` 调整持久大页计数时，可以使用任何内存策略模式——bindpreferred、local interleave。对持久大页分配的相应影响如下：

#. 无论 mempolicy 模式如何 [参见
   Documentation/admin-guide/mm/numa_memory_policy.rst]，持久大页都会分布到
   mempolicy 中指定的一个或多个节点上，就好像指定了 “interleave一样。但是，如果
   策略中的某个节点不包含用于一个大页的足够连续内存，分配不“fallback”（回退）到
   具有足够连续内存的相邻节点。这样做会导致大页池分布出现不良的不平衡，或者可能在   被任务内存策略允许的节点上分配持久大页
#. 可以使用 bind interleave 策略指定一个或多个节点。如果使preferred 策略指定
   了多个节点，则只会使用数值最小的 id。local 策略将选择构nodes_allowed 掩码   任务正在运行的节点。为了使 local 策略具有确定性，任务必须绑定到单个节点上cpu    一cpu。否则，任务可能在启动后的任何时刻被迁移到其它某个节点，而得到的节点将是
   不确定的。因此，local 策略用于此目的并不是很有用。可以使用任何其mempolicy 模式
   来指定单个节点
#. 允许的节点掩码将从任何非默认的任mempolicy 派生，无论此策略是由任务自身显式设置
   的还是由其某个祖先（例如 numactl）设置的。这意味着，如果任务是从具有非默认策略   shell 中调用的，则将使用该策略。可以使numactl --interleave --membind [-m]
   指定 “all的节点列表，以在系统cpuset 中的所有节点上实现交错
#. 指定的任何任mempolicy——例如使numactl——都将受到任务运行的任何 cpuset 的资   限制约束。因此，在包含系统节点子集的 cpuset 中运行的非默认策略任务将无法在该 cpuset
   之外分配大页，除非先移动到包含全部所需节点cpuset
#. 启动时的巨大页分配尝试将请求的大页数量分布到所有带有内存的在线节点上
## 姣忚妭鐐瑰ぇ椤靛睘鎬。

上面描述sysfs 中根大页控制目录内容的一个子集，将被复制到每个系统设备下
```

	/sys/devices/system/node/node[0-9]*/hugepages/

```
在此目录下，每种受支持大页大小的子目```

	nr_hugepages
	free_hugepages
	surplus_hugepages

```
free\_' surplus\_' 属性文件是只读的。它们分别返回父节点上空闲和盈余 [overcommitted]
大页的数量
`nr_hugepages` 属性返回指定节点上的大页总数。写入此属性时，如果资源充足，无论任务mempolicy cpuset 约束如何，父节点上的持久大页数量都将被调整到指定值
请注意，overcommit reserve 页的数量仍然是全局量，因为直到缺页时应用缺页任务的
mempolicy，我们才知道将从哪个节点尝试分配大页
hugetlb 可能在以下场景中的每节点大页池之间迁移：内存下线（memory offline）、内存故（memory failure）、长期固定（longterm pinning）、系统调用（mbind、migrate_pages move_pages）、alloc_contig_range() alloc_contig_pages()。目前只有内存下线、内存故和系统调用允许在当前节点无法hugetlb 迁移期间分配时回退到在不同节点上分配一个新hugetlb，这意味着3 种情况会破坏每节点大页池

## 使用大页


如果用户应用程序打算使用 mmap 系统调用请求大页，则要求系统管理员挂载一个文件系```

  mount -t hugetlbfs \
	-o uid=<value>,gid=<value>,mode=<value>,pagesize=<value>,size=<value>,\
	min_size=<value>,nr_inodes=<value> none /mnt/huge

```
此命令在 `/mnt/huge` 目录上挂载一hugetlbfs 类型的（伪）文件系统。在 `/mnt/huge`
上创建的任何文件都使用大页
`uid` `gid` 选项设置文件系统根的所有者和组。默认情况下采用当前进程`uid` `gid`
`mode` 选项将文件系统根的模式设置为 value & 01777。此值以八进制给出。默认选取 0755
如果平台支持多种大页大小，可以使`pagesize` 选项指定大页大小及关联的池。`pagesize`
以字节为单位指定。如果未指定 `pagesize`，则将使用平台的默认大页大小及关联的池
`size` 选项设置允许该文件系统（`/mnt/huge`）使用的最大内存值（大页）。`size` 选项可以
以字节为单位指定，或者指定为指定大页池（`nr_hugepages`）的百分比。size 向下取整HPAGE_SIZE 边界
`min_size` 选项设置允许该文件系统使用的最小内存值（大页）。`min_size` 的指定方式与
`size` 相同，可以是字节或大页池的百分比。在挂载时，`min_size` 指定的大页数量被
保留供该文件系统使用。如果没有足够的空闲大页可用，挂载将失败。随着大页被分配和释放文件系统，保留计数会被调整，以便已分配和已保留大页的总和始终至少`min_size`
选项 `nr_inodes` 设置 `/mnt/huge` 可以使用的最inode 数量
如果命令行上未提`size`、`min_size` `nr_inodes` 选项，则不会设置任何限制
对于 `pagesize`、`size`、`min_size` `nr_inodes` 选项，你可以使用 [G|g]/[M|m]/[K|k]
来表giga/mega/kilo。例如，size=2K size=2048 含义相同
虽然 hugetlb 文件系统上的文件支持读系统调用，但不支持写系统调用
具有适当权限的常chown、chgrp chmod 命令可用于更hugetlbfs 上的文件属性
此外，值得注意的是，如果应用程序只打算使用 shmat/shmget 系统调用或带 MAP_HUGETLB mmap，则不需要这样的挂载命令。有关如何将 mmap MAP_HUGETLB 配合使用的示例，请参下面map_hugetlb <map_hugetlb>
希望通过共享内存段使hugetlb 内存的用户必须是某个补充组的成员，并且系统管理员需要将
gid 配置`/proc/sys/vm/hugetlb_shm_group` 中。相同或不同的应用程序可以使mmaps shm* 调用的任意组合，尽管对于不带 MAP_HUGETLB mmap 调用，将需要挂载文系统
操作hugetlb 页支持的内存的系统调用，其长度仅对齐到处理器的原生页面大小；如果按大页对齐，它们通常会以 errno 设置EINVAL 失败，或者排除超出该长度hugetlb 页例如，如果内存由 hugetlb 页支持且长度小于大页大小，munmap(2) 将失败

## 示例



`map_hugetlb`
	参见 tools/testing/selftests/mm/map_hugetlb.c

`hugepage-shm`
	参见 tools/testing/selftests/mm/hugepage-shm.c

`hugepage-mmap`
	参见 tools/testing/selftests/mm/hugepage-mmap.c

`libhugetlbfs`_ 库提供了广泛的用户空间工具，以帮助提高大页的可用性、环境设置和控制