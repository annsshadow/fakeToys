## 内存资源控制

      本文档已经严重过时，需要完全重写。它仍然包含一些有用的信息，因此暂时保      在这里，但如果你需要更深入的理解，请确保查看当前的代码
      在本文档中，内存资源控制器通常被笼统地称为内存控制器。不要将这里使用的内存控制器
      与硬件中使用的内存控制器混淆
      当我们提到一个带有内存控制器cgroup（cgroupfs 的目录）时，我们称之      “memory cgroup”。当你查git 日志和源代码时，会看到补丁标题和函数名倾向      使用 “memcg”。在本文档中，我们避免使用它
## 内存控制器的益处与目

内存控制器将一组任务的內存行为与系统中其余部分隔离开来。LWN 上的文章 [^12^]_ 提到内存控制器的一些可能用途。内存控制器可用
a. 隔离一个应用程序或一组应用程   占用大量内存的应用程序可以被隔离并限制为较小的内存用量b. 创建一个内存用量受限的 cgroup；这可以很好地替代使mem=XXXX 启动内核c. 虚拟化方案可以控制它们想分配给某个虚拟机实例的内存量d. CD/DVD 刻录机可以控制系统中其余部分使用的内存量，以确保刻录不会   可用内存不足而失败e. 还有其他一些用例；找个用例，或者仅仅为了好玩（学习hack VM 子系统）而使用该控制器
当前状态：linux-2.6.34-mmotm010 4 月的开发版本）

特性：

 - 统计匿名页、文件缓存、交换缓存的使用情况并对其进行限制 - 页被独占地链接到每个 memcg LRU，不存在全局 LRU - 可选地，可以对内存+交换的使用情况进行统计和限制 - 层级统计
 - 软限 - 移动任务时转移（充值）记账是可选择的 - 使用量阈值通知 - 内存压力通知 - oom-killer 禁用开关和 oom 通知 - cgroup 没有限制控制
 内核内存支持仍是进行中的工作，当前版本基本提供该功能。（参见 :ref:`2.7  <cgroup-v1-memory-kernel-extension>`
控制文件简要汇总
==================================== ==========================================
 tasks				     附加一个任务（线程）并展示线程列表
 cgroup.procs			     展示进程列表
 cgroup.event_control		     用于 event_fd() 的接				     该开关在 CONFIG_PREEMPT_RT 系统上不可用 memory.usage_in_bytes		     展示内存的当前使用量
				     （详5.5 memory.memsw.usage_in_bytes	     展示内存+交换的当前使用量
				     （详5.5 memory.limit_in_bytes		     设置/展示内存使用量的限制
 memory.memsw.limit_in_bytes	     设置/展示内存+交换使用量的限制
 memory.failcnt			     展示内存使用量超出限制的次数
 memory.memsw.failcnt		     展示内存+交换超出限制的次 memory.max_usage_in_bytes	     展示记录到的最大内存使用量
 memory.memsw.max_usage_in_bytes     展示记录到的最大内交换使用 memory.soft_limit_in_bytes	     设置/展示内存使用量的软限				     该开关在 CONFIG_PREEMPT_RT 系统上不可用                                     该开关已废弃，不应再使用 memory.stat			     展示各种统计信息
 memory.use_hierarchy		     设置/展示层级记账是否启用
                                     该开关已废弃，不应再使用 memory.force_empty		     触发强制页回 memory.pressure_level		     设置内存压力通知
                                     该开关已废弃，不应再使用 memory.swappiness		     设置/展示 vmscan swappiness 参数
				     （参sysctl vm.swappiness				     每个 memcg 的开关在 cgroup v2 中不存在 memory.move_charge_at_immigrate     该开关已废弃 memory.oom_control		     设置/展示 oom 控制                                     该开关已废弃，不应再使用 memory.numa_stat		     展示每个 numa 节点的内存使用量
 memory.kmem.limit_in_bytes         已废弃的开关，用于设置和读取内核内存硬限制                                     5.16 起不再支持内核硬限制。向该文件写入任何值都
                                     不会生效，如同指定了 nokmem 内核参数一样。内核内                                     仍由 memory.kmem.usage_in_bytes 记账和报告 memory.kmem.usage_in_bytes         展示当前内核内存分配 memory.kmem.failcnt                展示内核内存使用量超出限制的次数
 memory.kmem.max_usage_in_bytes     展示记录到的最大内核内存使用量

 memory.kmem.tcp.limit_in_bytes     设置/展示 tcp 缓冲区内存的硬限                                     该开关已废弃，不应再使用 memory.kmem.tcp.usage_in_bytes     展示当前 tcp 缓冲区内存分配量
                                     该开关已废弃，不应再使用 memory.kmem.tcp.failcnt            展示 tcp 缓冲区内存使用量超出限制的次				     该开关已废弃，不应再使用 memory.kmem.tcp.max_usage_in_bytes 展示记录到的最tcp 缓冲区内存使用量
                                     该开关已废弃，不应再使用==================================== ==========================================

## 1. 历史


内存控制器有着悠久的历史。关于内存控制器的征求意见（RFC）由 Balbir Singh [^1^]_ 发布RFC 发布时，已有若干内存控制的实现。RFC 的目标是就内存控制所需的最小特建立共识。第一RSS 控制器由 Balbir Singh [^2^]_ 2007 2 月发布Pavel Emelianov [^3^]_ [^4^]_ [^5^]_ 此后发布了三个版本的 RSS 控制器。在 OLS 资源管理 BoF 上，每个人都建议我们同时处理页缓存和 RSS。另一个请求是允许
用户空间处理 OOM。当前的内存控制器是6 版；它同时结合了已映射（RSS）和
未映射的页缓存控[^11^]_
## 2. 内存控制


内存是一种独特的资源，因为它以有限的数量存在。如果一个任务需要大CPU 处理该任务可以将其处理分散在数小时、数天、数月或数年的时间里，但对于内存，同样的
物理内存需要被复用以完成任务
内存控制器的实现被划分为几个阶段。它们是
1. 内存控制2. mlock(2) 控制3. 内核用户内存记账slab 控制
4. 用户映射长度控制
内存控制器是第一个被开发的控制器
### 2.1. 设计


设计的核心是一个称page_counter 的计数器。page_counter 跟踪与控制器关联那组进程的当前内存使用量和限制。每cgroup 都有一个与之关联的内存控制特定数据结构（mem_cgroup）
### 2.2. 记账


   :caption: 1：记账的层级结构

		+--------------------+
		|  mem_cgroup        |
		|  (page_counter)    |
		+--------------------+
		 /            ^      \
		/             |       \
           +---------------+  |        +---------------+
           | mm_struct     |  |....    | mm_struct     |
           |               |  |        |               |
           +---------------+  |        +---------------+
                              |
                              - --------------+
                                              |
           +---------------+           +------+--------+
           | page          +---------->  page_cgroup|
           |               |           |               |
           +---------------+           +---------------+



1 展示了控制器的重要方
1. 记账按每cgroup 进行
2. 每个 mm_struct 知道它属于哪cgroup
3. 每个页都有一个指page_cgroup 的指针，page_cgroup 又知道它所属的
   cgroup

记账的过程如下：调用 mem_cgroup_charge_common() 来建立必要的数据结构，并检正在被记账的 cgroup 是否超过了它的限制。如果是，则在该 cgroup 上调用回收更多细节可以在本文档的回收（reclaim）一节找到。如果一切顺利，则更新一个名page_cgroup 的页元数据结构。page_cgroup cgroup 上拥有自己的 LRU(*) page_cgroup 结构在启内存热插拔时分配
### 2.2.1 记账细节


所有已映射的匿名页（RSS）和缓存页（Page Cache）都会被记账一些永远不可回收且不会出现LRU 上的页不会被记账。我们只对处于通常 VM 管理之下页进行记账
RSS 页在 page_fault 时被记账，除非它们之前已经被记账过。文件页在被插入 inode
（xarray）时会作Page Cache 被记账。当它被映射到进程的页表中时，会小心地避重复记账
RSS 页在完全解除映射时被取消记账。PageCache 页在xarray 移除时被取消记账即使 RSS 页已被完全解除映射（kswapd），在它们真正被释放之前，它们可能以
SwapCache 的形式存在于系统中。这样的 SwapCache 也会被记账。换入的页会在被加入
swapcache 后被记账
注意：内核会进行 swapin-readahead 并一次性读取多个交换页。由于页memcg 在交换时
就被记录swap 中（无论是否启用 memsw），该页会在 swapin 之后被记账
在页迁移时，记账信息会被保留
注意：我们只对处LRU 上的页进行记账，因为我们的目的是控制已用页的数量；从 VM
的角度看，不LRU 上的页往往处于失控状态
### 2.3 共享页记

共享页基于“首次访问”原则进行记账。首次访问某个页cgroup 会被计入该页。这种方背后的原理是，积极使用共享页cgroup 最终会被计入该页（一旦它从将其引入的 cgroup
处被解除记账——这会在内存压力下发生）
### 2.4 交换扩展


每个 cgroup 的交换使用量总是被记录。交换扩展（Swap Extension）允许你读取并限制它
当启CONFIG_SWAP 时，会添加以下文件
 - memory.memsw.usage_in_bytes銆? - memory.memsw.limit_in_bytes銆。
memsw 表示内存+交换。内交换的使用量memsw.limit_in_bytes 限制
示例：假设一个拥4G 交换的系统。一个在 2G 内存限制下分配了 6G 内存（由于失误）任务将用完所有交换。在这种情况下，设置 memsw.limit_in_bytes=3G 将防止对交换不当使用。通过使用 memsw 限制，你可以避免由交换短缺引起的系统 OOM
#### 2.4.1 为什么是 “memory+swap而不swap


全局 LRU（kswapd）可以换出任意页。换出意味着把记账从内存移动到交换……内交换使用量没有变化。换句话说，当我们想要在不影响全局 LRU 的情况下限制交换的使用量时，
从操作系统的角度看，内存+交换限制比仅仅限制交换更好
#### 2.4.2. cgroup 达到 memory.memsw.limit_in_bytes 时会发生什

cgroup 达到 memory.memsw.limit_in_bytes 时，在该 cgroup 内进行换出是没有意义的那么，cgroup 例程将不会进行换出，并且文件缓存会被丢弃。但如上所述，全局 LRU 可以
为了系统内存管理状态的健康而从中换出内存。你无法通过 cgroup 禁止它
### 2.5 回收


每个 cgroup 维护一个与全局 VM 结构相同的每 cgroup LRU。当一cgroup 超过它的限制时，
我们首先尝试从该 cgroup 回收内存，以便为 cgroup 已接触的那些新页腾出空间。如果回不成功，则调OOM 例程来选择并杀死该 cgroup 中体积最大的任务。（参见下文
10. OOM 控制 <cgroup-v1-memory-oom-control>。）

回收算法并未针对 cgroup 进行修改，只是被选作回收目标的页来自cgroup LRU 列表
   回收对根 cgroup 不起作用，因为我们无法对cgroup 设置任何限制
   panic_on_oom 被设置为 时，整个系统将会 panic
当注册了 oom 事件通知器时，事件会被传递（参oom_control <cgroup-v1-memory-oom-control> 一节）

### 2.6 加锁


```

  folio_lock
    mm->page_table_lock or split pte_lock
      mapping->i_pages lock
        lruvec->lru_lock.

```
每节点每 memcgroup LRU（cgroup 的私LRU）由 lruvec->lru_lock 保护；在
lruvec->lru_lock 下从 LRU 中隔离一个页之前，会先清除该 folio LRU 标志

### 2.7 内核内存扩展


通过内核内存扩展，内存控制器能够限制系统所使用的内核内存量。内核内存从根本不同于用户内存，因为它无法被换出，这使得通过消耗过多这种宝贵资源来对系统发DoS 成为可能
默认情况下，内核内存记账对所有内cgroup 启用。但它可以通过在内核启动时向内传cgroup.memory=nokmem 来在系统范围内禁用。在这种情况下，内核内存将完全不记账
cgroup 不施加内核内存限制。根 cgroup 的使用量可能会被记账，也可能不会。所使用内存被累加到 memory.kmem.usage_in_bytes，或者在有意义时累加到一个单独的计数器中
（目前仅针对 tcp）
“kmem计数器被馈入主计数器，因kmem 记账也会从用户计数器中可见
目前尚未针对内核内存实现软限制。在这些限制达到时触slab 回收是未来的工作
### 2.7.1 当前已记账的内核内存资源


stack pages（栈页）  每个进程都消耗一些栈页。通过计入内核内存，我们防止在内核内存使用量过高时
  创建新进程
slab pages（slab 页）  SLAB SLUB 分配器分配的页会被跟踪。每当缓存首次从 memcg 内部被接触时  都会创建一个每kmem_cache 的副本。创建是惰性进行的，因此在缓存创建期间
  仍可能跳过一些对象。slab 页中的所有对象都应属于同一memcg。只有当一个任务在
  缓存分配页期间被迁移到不同的 memcg 时，这一点才会失效
sockets memory pressure（套接字内存压力）：
  一些套接字协议具有内存压力阈值。内存控制器允许按每cgroup 单独控制它们  而不是全局控制
tcp memory pressure（tcp 内存压力）：
  tcp 协议的套接字内存压力
### 2.7.2 常见用例


由于 “kmem计数器被馈入主用户计数器，内核内存永远无法完全独立于用户内存被限制“U为用户限制，“K为内核限制。限制有以下几种可能的设置方式：

U != 0, K = 无限    这是kmem 记账出现之前就已有的标准 memcg 限制机制。内核内存被完全忽略
U != 0, K < U    内核内存是用户内存的一个子集。这种设置在对每cgroup 的总内存量进行过量承诺
    （overcommit）的部署中很有用。过量承诺内核内存限制绝对不被推荐，因为机器仍可    耗尽不可回收的内存    在这种情况下，管理员可以设置 K，使得所有组的总和永远不会大于总内存，并以牺牲
    QoS 为代价自由地设置 U
```
       In the current implementation, memory reclaim will NOT be triggered for
       a cgroup when it hits K while staying below U, which makes this setup
       impractical.

```
U != 0, K >= U    由于 kmem 记账也会被馈入用户计数器，并且回收会针对这两种内存为cgroup 触发    这种设置为管理员提供了对内存的统一视图，对于只想跟踪内核内存使用量的人也很有用
## 3. 用户接口


要使用用户接口：

1. 启用 CONFIG_CGROUPS CONFIG_MEMCG 选项
2. 准备 cgroup（参:ref:`为什么需cgroup```
	# mount -t tmpfs none /sys/fs/cgroup
	# mkdir /sys/fs/cgroup/memory
	# mount -t cgroup none /sys/fs/cgroup/memory -o memory

```
```

	# mkdir /sys/fs/cgroup/memory/0
	# echo $$ > /sys/fs/cgroup/memory/0/tasks

```
```

	# echo 4M > /sys/fs/cgroup/memory/0/memory.limit_in_bytes

   The limit can now be queried::

	# cat /sys/fs/cgroup/memory/0/memory.limit_in_bytes
	4194304

```
   我们可以使用后缀（k、K、m、M、g G）来表示千、兆或吉字节   （这里，Kilo、Mega、Giga 分别Kibibytes、Mebibytes、Gibibytes。）

   我们可以写入 1来重`*.limit_in_bytes（无限制）`
   我们不再能对cgroup 设置限制

```

  # cat /sys/fs/cgroup/memory/0/memory.usage_in_bytes
  1216512

```
向该文件写入成功并不能保证文件中写入的值被成功设置为该限制。这可能是由于多因素造成的，例如向上取整到页边界或系统上内存的总可用量。用户需要重新读```

  # echo 1 > memory.limit_in_bytes
  # cat memory.limit_in_bytes
  4096

```
memory.failcnt 字段给出cgroup 限制被超出次数的值
memory.stat 文件给出了记账信息。现在，会展示缓存、RSS 以及活跃非活跃页的数量
## 4. 测试


有关测试特性和实现，请参见 memcg_test.txt
性能测试也很重要。要查看纯粹的内存控制器开销，在 tmpfs 上测试将给出较小开销数值。示例：tmpfs 上执行内make
页错误（page-fault）的可扩展性也很重要。在测量并行页错误测试时，多进程测试可能
优于多线程测试，因为它具有共享对状态的噪声
但上述两种测试都在测试极端情况在内存控制器下进行常规测试总是有帮助的

### 4.1 故障排查


有时用户可能会发cgroup 下的应用程序OOM killer 终止。造成这种情况的原因有
几个
1. cgroup 限制太低（低到无法做任何有用的事情）
2. 用户正在使用匿名内存，且交换被关闭或太低

执行 sync，然echo 1 > /proc/sys/vm/drop_caches 将有助于清除 cgroup 中缓存的
一些页（页缓存页）
要了解发生了什么，按照 :ref:`0. OOM 控制<cgroup-v1-memory-oom-control>`（下文）禁用 OOM_Kill 并观察发生的情况会很有帮助

### 4.2 任务迁移


当一个任务从一cgroup 迁移到另一cgroup 时，默认情况下它的记账不会被结转从原 cgroup 分配的页仍然计入cgroup，记账会在页被释放或回收时丢弃
你可以让任务的记账随任务迁移一起移动参见 8. “任务迁移时转移记账<cgroup-v1-memory-move-charges>

### 4.3 移除一cgroup


可以通过 rmdir 移除一cgroup，但正如 :ref:`4.1 <cgroup-v1-memory-test-troubleshoot>` :ref:`4.2 <cgroup-v1-memory-test-task-migration>` 中所讨论的，一cgroup 可能仍关联着一记账，即使所有任务都已迁移走。（因为我们是针对页记账，而不是针对任务记账。）

我们将统计信息移动到父节点，除了从子节点解除记账外，记账没有变化
在移cgroup 时，交换信息中记录的记账不会被更新。记录的信息会被丢弃，而使用交（swapcache）的 cgroup 将被作为它的新所有者记账
## 5. 杂项接口


### 5.1 force_empty


  memory.force_empty 接口用于cgroup 的内存使用量变为空```

    # echo 0 > memory.force_empty

  cgroup 将被回收，尽可能多地回收页
  该接口的典型用例是在调用 rmdir() 之前。虽rmdir() 会使 memcg 离线，但 memcg
  可能由于被记账的文件缓存而仍然存在。一些不再使用的页缓存可能一直保持被记账状态，
  直到发生内存压力。如果你想避免这种情况，force_empty 会很有用
```
### 5.2 stat 文件


memory.stat 文件包含以下统计信息
  - 每个内存 cgroup 的本地状
    =============== ===============================================================
    cache           页缓存内存的字节数    rss             匿名和交换缓存内存（包含透明大页）的字节数    rss_huge        匿名透明大页的字节数    mapped_file     已映射文件（包含 tmpfs/shmem）的字节数    pgpgin          向内cgroup 记账的事件数。每次一个页被记账为映射到该 cgroup                     匿名页（RSS）或缓存页（Page Cache）时，就会发生记账事件    pgpgout         从内cgroup 解除记账的事件数。每次一个页cgroup 解除记账时，
                    就会发生解除记账事件    swap            交换使用量的字节数    swapcached      内存中缓存的交换的字节数    dirty           等待写回磁盘的字节数    writeback       已排队等待同步到磁盘的文匿名缓存的字节数    inactive_anon   处于非活LRU 列表上的匿名和交换缓存内存的字节数    active_anon     处于活跃 LRU 列表上的匿名和交换缓存内存的字节数    inactive_file   处于非活LRU 列表上的文件支撑内存MADV_FREE 匿名
                    内存（LazyFree 页）的字节数    active_file     处于活跃 LRU 列表上的文件支撑内存的字节数    unevictable     无法被回收的内存（mlocked 等）的字节数    =============== ===============================================================

  - 考虑层级的状态（参见 memory.use_hierarchy 设置）：

    ========================= ===================================================
    hierarchical_memory_limit 与内cgroup 所处层级相关的内存限制的字节数
    hierarchical_memsw_limit  与内cgroup 所处层级相关的内存+交换限制的字节数

    total_<counter>           层级版本<counter>，除cgroup 自身的值外，还包含
                              所有层级子节点 <counter> 值的总和，即 total_cache
    ========================= ===================================================

  - 额外vm 参数（取决于 CONFIG_DEBUG_VM）：

    ========================= ========================================
    recent_rotated_anon       VM 内部参数。（参见 mm/vmscan.c    recent_rotated_file       VM 内部参数。（参见 mm/vmscan.c    recent_scanned_anon       VM 内部参数。（参见 mm/vmscan.c    recent_scanned_file       VM 内部参数。（参见 mm/vmscan.c    ========================= ========================================

	recent_rotated 表示 LRU 旋转的近期频率	recent_scanned 表示近期LRU 的扫描次数	为便于调试，请参见代码以了解其含义
	只有匿名和交换缓存内存被列为 “rss统计的一部分	不应将其与真正的 “resident set sizecgroup 使用的物理内存量混淆
	“rss + mapped_file会给cgroup 的常驻集大小
	注意，某些内核配置可能会将完整较大的分配（例THP）计“rss	“mapped_file”，即使只有部分（而非全部）该内存被映射
	（注意：文件shmem 可能在其cgroup 之间共享。在这种情况下，
	仅当内存 cgroup 是页缓存的所有者时才对 mapped_file 记账。）

### 5.3 swappiness


覆盖特定组的 /proc/sys/vm/swappiness。根 cgroup 中的可调参数对应于全局 swappiness
设置
请注意，与全局回收不同，限制回收强制要0 swappiness 确实会阻止任何交换，即使
有可用的交换存储。如果不存在可回收的文件页，这可能导memcg OOM killer 触发
### 5.4 failcnt


内存 cgroup 提供 memory.failcnt memory.memsw.failcnt 文件。这failcnt=
失败计数）展示了使用量计数器达到其限制的次数。当内存 cgroup 达到限制时，failcnt
增加，其下的内存将被回收
```

	# echo 0 > .../memory.failcnt

```
### 5.5 usage_in_bytes


为了提高效率，与其他内核组件一样，内存 cgroup 使用一些优化来避免不必要的缓存伪共享。usage_in_bytes 会受该方法影响，不会显示内存（和交换）使用量的“精确”值，
它是一个用于高效访问的模糊值。（当然，在必要时它会同步。）如果你想了解更精确的
内存使用量，应该使用 memory.stat 中的 RSS+CACHESWAP）值（参见 5.2）
### 5.6 numa_stat


这类似于 numa_maps，但作用于每memcg 的层面。这在提memcg 内部numa 局部信息可见性方面很有用，因为页被允许从任何物理节点分配。一个用例是通过将该信息应用程序CPU 分配相结合来评估应用程序性能
每个 memcg numa_stat 文件包含 “total”、“file”、“anon“unevictable每节点页计数，包“hierarchical_<counter>”，它除memcg 自身的值外，还累加
所有层级子节点的值
```

  total=<total pages> N0=<node 0 pages> N1=<node 1 pages> ...
  file=<total file pages> N0=<node 0 pages> N1=<node 1 pages> ...
  anon=<total anon pages> N0=<node 0 pages> N1=<node 1 pages> ...
  unevictable=<total anon pages> N0=<node 0 pages> N1=<node 1 pages> ...
  hierarchical_<counter>=<counter pages> N0=<node 0 pages> N1=<node 1 pages> ...

```
“total计数file + anon + unevictable 之和
## 6. 层级支持


内存控制器支持深层层级和层级记账。层级是通过cgroup 文件系统中创建适当cgroup
来创建的。例如，考虑以下 cgroup 文件系统
```

	       root
	     /  |   \
            /	|    \
	   a	b     c
		      | \
		      |  \
		      d   e

```
在上图中，在启用了层级记账的情况下，e 的所有内存使用量都会被记账到它的祖先，直根（c 和根）。如果某个祖先超过了它的限制，回收算法会从该祖先及其子节点中的任进行回收
### 6.1 层级记账与回

层级记账默认启用。禁用层级记账已被废弃。尝试这样做将导致失败，并向 dmesg 打印
警告
```

	# echo 1 > memory.use_hierarchy

```
## 7. 软限制（已废弃）


此功能已废弃
软限制允许更大程度地共享内存。软限制背后的想法是允许控制组在需要时尽可能多地使内存，前提是

a. 不存在内存争b. 它们不超过各自的硬限
当系统检测到内存争用或内存不足时，控制组会被压回到它们的软限制。如果每个控制组软限制都很高，它们会被尽可能压回，以确保一个控制组不会让其他控制组饿死内存
请注意，软限制是一个尽力而为的特性；它不提供任何保证，但会尽最大努力确保当内存
被激烈争用时，内存是基于软限制提设置来分配的。目前基于软限制的回收被设置balance_pgdat（kswapd）调用
### 7.1 接口


可以使用以下命令设置软限制（在此示例中我```

	# echo 256M > memory.soft_limit_in_bytes

```
```

	# echo 1G > memory.soft_limit_in_bytes

```
       软限制需要较长一段时间才能生效，因为它们涉及为平衡内cgroup 之间的内       而进行回收
       建议始终将软限制设置在硬限制之下，否则硬限制将优先

## 8. 任务迁移时转移记账（已废弃！

此功能已废弃
读取 memory.move_charge_at_immigrate 将总是返回 0，向它写入将总是返回 -EINVAL
## 9. 内存阈

内存 cgroup 使用 cgroup 的通知 API（参cgroups.txt）实现内存阈值。它允许注册
多个内存memsw 阈值，并在越过阈值时获得通知
要注册一个阈值，应用程序必须
- 使用 eventfd(2) 创建一eventfd- 打开 memory.usage_in_bytes memory.memsw.usage_in_bytes- 将形event_fd> <fd of memory.usage_in_bytes> <threshold>的字符串写入
  cgroup.event_control銆。
当内存使用量在任何方向上越过阈值时，应用程序将通过 eventfd 收到通知
它适用于根 cgroup 和非cgroup

## 10. OOM 控制（已废弃

此功能已废弃
memory.oom_control 文件用于 OOM 通知和其他控制
内存 cgroup 使用 cgroup 通知 API（参cgroups.txt）实OOM 通知器。它允许注册
多个 OOM 通知投递，并在发生 OOM 时获得通知
要注册一个通知器，应用程序必须
 - 使用 eventfd(2) 创建 eventfd
 - 打开 memory.oom_control 文件
 - 将形event_fd> <fd of memory.oom_control>的字符串写入
   cgroup.event_control

应用程序将在发生 OOM 时通过 eventfd 收到通知。OOM 通知对根 cgroup 不起作用
你可以通过写入 memory.oom_control 文件来禁OOM-killer，如下所示：

	#echo 1 > memory.oom_control

如果 OOM-killer 被禁用，cgroup 下的任务在请求可记账内存时，将挂休眠在内cgroup
OOM 等待队列中
要让它们运行，你必须通过以下方式放宽内存 cgroup OOM 状态：

 - 增大限制或减小使用量
要减小使用量
 - 杀死一些任务 - 通过记账迁移将一些任务移动到其他组 - 移除一些文件（tmpfs 上？
然后，被停止的任务将再次工作
读取时，会展OOM 的当前状态
 - oom_kill_disable 0 鎴?1
	  （若1，则 oom-killer 被禁用）
 - under_oom	   0 鎴?1
	  （若1，则内存 cgroup 处于 OOM 之下，任务可能被停止。）
        - oom_kill         整数计数          属于cgroup 被任何类型的 OOM killer 杀死的进程数量
## 11. 内存压力（已废弃

此功能已废弃
压力级别通知可用于监控内存分配代价；基于压力，应用程序可以实现不同的策略来管它们的内存资源。压力级别定义如下：

“low级别表示系统正在为新的分配回收内存。监控这种回收活动可能有助于维持缓存
级别。收到通知时，程序（通常是“Activity Manager”）可能会分vmstat 并提前采取行（即提前关闭不重要的服务）
“medium级别表示系统正在经历中等内存压力，系统可能正在进行交换、换出活跃文件缓等。发生此事件时，应用程序可能会决定进一步分vmstat/zoneinfo/memcg 或内部内存使统计，并释放任何可以轻松重建或从磁盘重新读取的资源
“critical级别表示系统正在 actively thrashing（剧烈颠簸），它即将内存耗尽（OOM），
甚至内核内的 OOM killer 也即将触发。应用程序应尽其所能帮助系统。此时再咨询 vmstat
或任何其他统计信息可能为时已晚，因此建议立即采取行动
默认情况下，事件向上传播直到事件被处理，即事件不是透传的。例如，你有三个 cgroupA->B->C。现在你cgroup A、B C 上设置了事件监听器，并假设组 C 经历了某些压力在这种情况下，只有组 C 会收到通知，即A B 不会收到通知。这样做是为了避免过“广播”消息，这会扰乱系统，并且在内存不足或颠簸时尤其糟糕。只有当C 没有事件
监听器时，组 B 才会收到通知
有三种可选模式指定不同的传播行为
 - “default”：这是上面指定的默认行为。该模式与省略可选模式参数相同，出于向后兼容
   而保留
 - “hierarchy”：事件总是向上传播到根，类似于默认行为，不同的是在 “hierarchy模式   无论每一层是否有事件监听器，传播都会继续。在上述示例中，A、B C 都将收到
   内存压力通知
 - “local”：事件是透传的，即它们只有在为其注册通知memcg 经历内存压力时才会收   通知。在上述示例中，如果“local通知注册，组 C 会在经历内存压力时收到通知   然而，无论C 是否有事件监听器，组 B 永远不会收到通知，如果组 B 注册的是 local
   通知
级别和事件通知模式（必要时“hierarchy“local”）由逗号分隔的字符串指定，即
“low,hierarchy指定层级、透传、对所有祖memcg 的通知。默认的非透传行为不指模式。“medium,local指定 medium 级别的透传通知
memory.pressure_level 文件仅用于设eventfd。要注册通知，应用程序必须：

- 使用 eventfd(2) 创建 eventfd- 打开 memory.pressure_level- 将形event_fd> <fd of memory.pressure_level> <level[,mode]>的字符串写入
  cgroup.event_control銆。
当内存压力处于特定级别（或更高）时，应用程序将通过 eventfd 收到通知。对
memory.pressure_level 的读/写操作未实现
测试
   这里有一个小脚本示例，它创建一个新cgroup，设置内存限制，在该 cgroup 中设   通知，然后创建子 cgroup
```

	# cd /sys/fs/cgroup/memory/
	# mkdir foo
	# cd foo
	# cgroup_event_listener memory.pressure_level low,hierarchy &
	# echo 8000000 > memory.limit_in_bytes
	# echo 8000000 > memory.memsw.limit_in_bytes
	# echo $$ > tasks
	# dd if=/dev/zero | read x

   （预期会收到一堆通知，最oom-killer 会被触发。）

```
## 12. TODO


1. 让每 cgroup 的扫描器优先回收非共享页
2. 教会控制器对共享页记3. 在尚未达到限制但使用量逐渐接近时，在后台开始回
## 总结


总体而言，内存控制器一直是一个稳定的控制器，并且在社区中得到了相当广泛的评论讨论
## 参考资

   http://lwn.net/Articles/222762/
   https://lore.kernel.org/r/45ED7DEC.7010403@sw.ru
   https://lore.kernel.org/r/461A3010.90403@sw.ru
   https://lore.kernel.org/r/465D9739.8070209@openvz.org

6. Menage, Paul. Control Groups v10, http://lwn.net/Articles/236032/
7. Vaidyanathan, Srinivasan, Control Groups: Pagecache accounting and control
   subsystem (v3), http://lwn.net/Articles/235534/
8. Singh, Balbir. RSS controller v2 test results (lmbench),
   https://lore.kernel.org/r/464C95D4.7070806@linux.vnet.ibm.com
9. Singh, Balbir. RSS controller v2 AIM9 results
   https://lore.kernel.org/r/464D267A.50107@linux.vnet.ibm.com
10. Singh, Balbir. Memory controller v6 test results,
    https://lore.kernel.org/r/20070819094658.654.84837.sendpatchset@balbir-laptop

   https://lore.kernel.org/r/20070817084228.26003.12568.sendpatchset@balbir-laptop
   http://lwn.net/Articles/243795/
