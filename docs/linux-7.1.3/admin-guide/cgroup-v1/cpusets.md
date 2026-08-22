
## CPUSETS（CPU 集合

Copyright (C) 2004 BULL SA.

Written by Simon.Derr@bull.net

- Portions Copyright (c) 2004-2006 Silicon Graphics, Inc.
- Modified by Paul Jackson <pj@sgi.com>
- Modified by Christoph Lameter <cl@gentwo.org>
- Modified by Paul Menage <menage@google.com>
- Modified by Hidetoshi Seto <seto.hidetoshi@jp.fujitsu.com>


   1. Cpusets（CPU 集合     1.1 什么是 cpusets     1.2 为什么需cpusets     1.3 cpusets 是如何实现的     1.4 什么是独占 cpusets     1.5 什么是 memory_pressure     1.6 什么是 memory spread     1.7 什么是 sched_load_balance     1.8 什么是 sched_relax_domain_level     1.9 我如何使cpusets   2. 使用示例与语     2.1 基本用法
     2.2 添加/移除 cpus
     2.3 设置标志
     2.4 附加进程
   3. 问题
   4. 联系方式

## 1. Cpusets（CPU 集合

### 1.1 什么是 cpusets

Cpusets 提供了一种机制，用于将一CPU 和内存节点（Memory Nodes）分配给一组任务。在本文档中内存节点（Memory Node指的是包含内存的在线节点
Cpusets 将任务的 CPU 和内存放置限制为仅在其当cpuset 内的资源。它们形成了一个嵌套的、在虚拟文件系统中可见的层次结构。这些是除了现有机制之外、管理大型系统上动态作业放置所需的必要钩子
Cpusets 使用 Documentation/admin-guide/cgroup-v1/cgroups.rst 中描述的通用 cgroup 子系统
任务使用 sched_setaffinity(2) 系统调用将其 CPU 包含CPU 亲和性掩码，并使mbind(2) set_mempolicy(2) 系统调用将内存节点包含进其内存策略，这些请求都会经过该任务的 cpuset 过滤，过滤掉不在cpuset 中的任何 CPU 或内存节点。调度器不会在任务的 cpus_allowed 向量所不允许的 CPU 上调度任务，并且内核页分配器不会在请求任mems_allowed 向量所不允许的节点上分配页
用户级代码可以在 cgroup 虚拟文件系统中按名称创建和销cpusets，管理这cpusets 的属性与权限，以及分配给每个 cpuset CPU 和内存节点，指定并查询任务被分配到哪cpuset，并列出分配给某cpuset 的任pid

### 1.2 为什么需cpusets

管理具有许多处理器（CPU）、复杂内存缓存层次结构以及具有非均匀访问时间（NUMA）的多个内存节点的大型计算机系统，对进程的高效调度和内存放置提出了额外的挑战
通常，更适中规模的系统可以通过让操作系统在请求的任务之间自动共享可用的 CPU 和内存资源，以足够的效率运行
但更大的系统从仔细的处理器和内存放置中获益更多，以减少内存访问时间和争用，并且通常代表了客户更大的投资，可以从将作业显式放置在适当大小的系统子集上获益
这在以下场景尤其有价值：

    - 运行同一 Web 应用多个实例Web 服务器，
    - 运行不同应用的服务器（例如，一Web 服务器和一个数据库），    - 运行具有苛刻性能特征的的大型 HPC 应用NUMA 系统
这些子集，或"软分区（soft partitions必须能够随着作业组合的变化而动态调整，而不影响其他并发执行的作业。运行作业页面的位置也可能在内存位置改变时被移动
内核 cpuset 补丁提供了高效实现此类子集所需的最小必要内核机制。它利用 Linux 内核中现有的 CPU 和内存放置设施，以避免对关键的调度器或内存分配器代码产生任何额外影响

### 1.3 cpusets 是如何实现的

Cpusets 提供了一Linux 内核机制，用于约束进程或一组进程所使用CPU 和内存节点
Linux 内核已经有一对机制来指定任务可以在哪CPU 上被调度（sched_setaffinity）以及可以从哪些内存节点获取内存（mbind、set_mempolicy）
Cpusets 对这两种机制扩展如下
 - Cpusets 是内核所知的、允许的 CPU 和内存节点的集合 - 系统中的每个任务都通过一个指向引用计cgroup 结构的指针附加到一cpuset - sched_setaffinity 的调用被过滤为仅限该任务 cpuset 中允许的 CPU - mbind set_mempolicy 的调用被过滤为仅限该任务 cpuset 中允许的内存节点 - cpuset 包含系统的所CPU 和内存节点 - 对于任何 cpuset，可以定义包含父CPU 和内存节点资源子集的cpusets - cpusets 的层次结构可以挂载在 /dev/cpuset，以便从用户空间浏览和操作 - 一cpuset 可以被标记为独占（exclusive），这确保没有其cpuset（直接祖先和后代除外）可以包含任何重叠的 CPU 或内存节点 - 你可以列出附加到任何 cpuset 的所有任务（pid）
cpusets 的实现需要少量、简单的钩子插入内核其余部分，且都不在性能关键路径上：

 - init/main.c 中，在系统启动时初始化根 cpuset - fork exit 中，将任务附加到和从cpuset 分离 - sched_setaffinity 中，用该任务 cpuset 中允许的内容屏蔽请求CPU - sched.c migrate_live_tasks() 中，尽可能将任务保留在其 cpuset 允许CPU 内迁移 - mbind set_mempolicy 系统调用中，用该任务 cpuset 中允许的内容屏蔽请求的内存节点 - page_alloc.c 中，将内存限制为允许的节点 - vmscan.c 中，将页回收限制在当cpuset 内
你应该挂"cgroup" 文件系统类型，以启用浏览和修改内核当前已知的 cpusets。没有为 cpusets 添加新的系统调用——所有查询和修改 cpusets 的支持都通过cpuset 文件系统
每个任务/proc/<pid>/status 文件有新增的四行，显示任务的 cpus_allowed（可在其上被调度CPU）和 mems_allowed（可从中获取内存的内存节点）```

  Cpus_allowed:   ffffffff,ffffffff,ffffffff,ffffffff
  Cpus_allowed_list:      0-127
  Mems_allowed:   ffffffff,ffffffff
  Mems_allowed_list:      0-63

```
每个 cpuset cgroup 文件系统中的目录表示，该目录包含（在标准 cgroup 文件之上）描述该 cpuset 的以下文件：

 - cpuset.cpus：该 cpuset 中的 CPU 列表
 - cpuset.mems：该 cpuset 中的内存节点列表
 - cpuset.memory_migrate 标志：若设置，将页移动到 cpusets 节点
 - cpuset.cpu_exclusive 标志：CPU 放置是否独占 - cpuset.mem_exclusive 标志：内存放置是否独占？
 - cpuset.mem_hardwall 标志：内存分配是否硬墙隔 - cpuset.memory_pressure：cpuset 中分页压力大小的度量
 - cpuset.memory_spread_page 标志：若设置，在允许的节点上均匀分散页缓 - cpuset.memory_spread_slab 标志：已废弃。没有任何功能 - cpuset.sched_load_balance 标志：若设置，在cpuset 内的 CPU 上做负载均衡
 - cpuset.sched_relax_domain_level：迁移任务时的搜索范
此外，只有根 cpuset 具有以下文件
 - cpuset.memory_pressure_enabled 标志：是否计memory_pressure
新的 cpusets 是使mkdir 系统调用shell 命令创建的。cpuset 的属性，例如其标志、允许的 CPU 和内存节点，以及附加的任务，通过写入cpuset 目录中的相应文件来修改，如上所列
嵌套 cpusets 的命名层次结构允许将大型系统划分为嵌套的、可动态变更的"软分
每个任务的附加（在该任务 fork 时由其子任务自动继承）到一cpuset，使得可以将系统上的工作负载组织成相关的任务集合，每个集合被约束为使用特cpuset CPU 和内存节点。如果必cpuset 文件系统目录上的权限允许，任务可以重新附加到任何其他 cpuset
这种"大范的系统管理与使用 sched_setaffinity、mbind set_mempolicy 系统调用在单个任务和内存区域上完成的详细放置相集成
以下规则适用于每cpuset
 - 它的 CPU 和内存节点必须是其父级的子集 - 除非其父级是独占的，否则它不能被标记为独占 - 如果它的 CPU 或内存是独占的，它们不得与任何兄弟节点重叠
这些规则，以cpusets 的自然层次结构，使得能够高效实施独占保证，而无需在它们中的任何一个发生变化时扫描所cpusets 以确保没有东西与独占 cpuset 重叠。此外，使用 Linux 虚拟文件系统（vfs）来表示 cpuset 层次结构，为 cpusets 提供了一个熟悉的权限和命名空间，且只需最少量的额外内核代码
根（top_cpuset）cpuset 中的 cpus mems 文件是只读的。cpus 文件使用 CPU 热插拔通知器自动跟cpu_online_mask 的值，mems 文件使用 cpuset_track_online_nodes() 钩子自动跟踪 node_states[N_MEMORY] 的值（即带有内存的节点）
cpuset.effective_cpus cpuset.effective_mems 文件通常cpuset.cpus cpuset.mems 文件的只读副本。如cpuset cgroup 文件系统使用特殊"cpuset_v2_mode" 选项挂载，这些文件的行为将变得类似于 cpuset v2 中的相应文件。换句话说，热插拔事件不会改cpuset.cpus cpuset.mems。这些事件只会影cpuset.effective_cpus cpuset.effective_mems，它们显示此 cpuset 当前实际使用CPU 和内存节点。有cpuset v2 行为的更多信息，请参Documentation/admin-guide/cgroup-v2.rst

### 1.4 什么是独占 cpusets

如果一cpuset CPU 或内存独占的，那么没有其cpuset（直接祖先或后代除外）可以共享任何相同的 CPU 或内存节点
一cpuset.mem_exclusive *** cpuset.mem_hardwall cpuset 硬墙隔离（hardwalled的，即它限制内核为页、缓冲区和其他通常被内核在多个用户间共享的数据进行的分配。所cpusets，无论是否硬墙隔离，都限制用户空间的内存分配。这使得可以配置一个系统，使几个独立的作业可以共享公共的内核数据，例如文件系统页，同时将每个作业的用户分配隔离在它自己cpuset 中。为此，构造一个大mem_exclusive cpuset 来容纳所有作业，并为每个单独作业构造子级的、非 mem_exclusive cpusets。只有少量的典型内核内存，例如来自中断处理程序的请求，才被允许在 mem_exclusive cpuset 之外获取

### 1.5 什么是 memory_pressure

cpuset memory_pressure 提供了一个简单的、每 cpuset 的度量指标，表示 cpuset 中的任务试图释放 cpuset 节点上正在使用的内存以满足额外内存请求的速度
这使得在专用 cpusets 中监控作业的批处理管理器能够高效地检测该作业引起的内存压力水平
这对这两类情况都很有用：在运行各种提交作业的紧密管理系统中，可能会选择终止或重新优先排序那些试图使用超过分配给它们的节点上允许内存的作业；以及在紧密耦合、长时间运行、大规模并行的科学计算作业中，如果它们开始使用超过允许的内存，将剧烈地无法达到所需的性能目标
此机制为批处理管理器提供了一种非常经济的方式来监cpuset 的内存压力迹象。由批处理管理器或其他用户代码来决定如何处理它并采取行动
==>
    除非通过"1" 写入特殊文件 /dev/cpuset/memory_pressure_enabled 启用此特性，否则 __alloc_pages() rebalance 代码中用于此度量的钩子会简化为简单地注意cpuset_memory_pressure_enabled 标志为零。因此，只有启用此特性的系统才会计算该度量
为何使用cpuset 的运行平均值：

    因为此仪表是cpuset 的，而非每任务或mm，所以批处理调度器监控此度量对系统施加的负载在大型系统上急剧降低，因为可以避免在每组查询时扫描任务列表
    因为此仪表是运行平均值，而非累积计数器，批处理调度器可以通过单次读取检测内存压力，而不必在一段时间内读取和累积结果
    因为此仪表是cpuset 的，而非每任务或mm，批处理调度器可以通过单次读取获得关键信息（cpuset 中的内存压力），而不必查询并累积 cpuset 中（动态变化的）所有任务集合的结果
一个每 cpuset 的简单数字滤波器（每cpuset 需要一个自旋锁3 个字的数据）被维护，并在任何附加到该 cpuset 的任务进入同步（直接）页回收代码时更新
一个每 cpuset 的文件提供一个整数，表示近期（半衰期10 秒）cpuset 中任务引起的直接页回收速率，单位为每秒尝试回收次数，乘1000

### 1.6 什么是 memory spread

每个 cpuset 有两个布尔标志文件，控制内核为文件系统缓冲区和相关的内核数据结构分配页的位置。它们称'cpuset.memory_spread_page' 'cpuset.memory_spread_slab'
如果设置了每 cpuset 布尔标志文件 'cpuset.memory_spread_page'，那么内核将把文件系统缓冲区（页缓存）均匀地分散在故障任务被允许使用的所有节点上，而不是倾向于将这些页放在任务运行的节点上
如果设置了每 cpuset 布尔标志文件 'cpuset.memory_spread_slab'，那么内核将把一些文件系统相关的 slab 缓存（例如用inode dentry）均匀地分散在故障任务被允许使用的所有节点上，而不是倾向于将这些页放在任务运行的节点上
这些标志的设置不影响任务的匿名数据段或栈段页
默认情况下，两种内存分散都是关闭的，内存页分配在任务运行的本地节点上，除非可能被任务NUMA 内存策略cpuset 配置修改，只要有足够的空闲内存页可用
当创建新cpusets 时，它们继承其父级的 memory spread 设置
设置内存分散会导致受影响页或 slab 缓存的分配忽略任务的 NUMA 内存策略，而被分散。使mbind() set_mempolicy() 调用设置 NUMA 内存策略的任务不会注意到这些调用因其包含任务memory spread 设置而发生任何改变。如果关闭内存分散，则当前指定的 NUMA 内存策略再次适用于内存页分配
'cpuset.memory_spread_page' 'cpuset.memory_spread_slab' 都是布尔标志文件。默认情况下它们包含 "0"，意味着cpuset 的特性是关闭的。如果向该文件写"1"，则打开命名的特性
实现很简单
设置标志 'cpuset.memory_spread_page' 会为cpuset 中或随后加入cpuset 的每个任务打开一个每进程标志 PFA_SPREAD_PAGE。为页缓存进行的页分配调用被修改为对PFA_SPREAD_PAGE 任务标志执行内联检查，如果设置了，则调用新例程 cpuset_mem_spread_node() 返回用于分配的偏好节点
类似地，设置 'cpuset.memory_spread_slab' 会打开标志 PFA_SPREAD_SLAB，并且适当标记slab 缓存将从 cpuset_mem_spread_node() 返回的节点分配页
cpuset_mem_spread_node() 例程也很简单。它使用每任务转cpuset_mem_spread_rotor 的值来选择当前任务 mems_allowed 中的下一个节点作为分配偏好
这种内存放置策略在其他上下文中也称为轮询（round-robin）或交错（interleave）
此策略可以为需要将线程本地数据放在相应节点上、但需要访问大型文件系统数据集（这些数据集需要分散在作业 cpuset 的几个节点上才能放下）的作业带来实质性改进。如果没有此策略，特别是对于可能有一个线程读取数据集的作业，作业 cpuset 中节点间的内存分配会变得非常不均匀
### 1.7 什么是 sched_load_balance

内核调度器（kernel/sched/core.c）自动对任务进行负载均衡。如果一CPU 利用率不足，运行在该 CPU 上的内核代码将寻找其他更过载CPU 上的任务，并将其移动到自己这里，受诸cpusets sched_setaffinity 等放置机制的约束
负载均衡的算法成本及其对任务列表等关键共享内核数据结构的影响，会随被均衡 CPU 数量的增加而超线性增长。因此调度器支持将系统的 CPU 划分为若干调度域（sched domains），使得它只在每个调度域内做负载均衡。每个调度域覆盖系统中某CPU 子集；两个调度域不重叠；某些 CPU 可能不在任何调度域中，因此不会被负载均衡
简而言之，在两个较小的调度域之间做均衡比在一个大的调度域上做均衡成本更低，但这样做意味着一个域中的过载不会被负载均衡到另一个域
默认情况下，有一个覆盖所CPU 的调度域，包括那些使用内核启动参"isolcpus=" 标记为隔离的 CPU。但是，被隔离的 CPU 不会参与负载均衡，也不会有任务运行在它们上面，除非被显式分配
这种跨所CPU 的默认负载均衡不适合以下两种情况
 1) 在大型系统上，跨许多 CPU 的负载均衡代价高昂。如果系统使cpusets 管理以将独立作业放在独立CPU 集合上，则完全负载均衡是不必要的 2) 在某CPU 上支持实时（realtime）的系统需要最小化这些 CPU 上的系统开销，包括避免任务负载均衡（如果不需要的话）
当每 cpuset 标志 "cpuset.sched_load_balance" 被启用（默认设置）时，它请求cpuset 允许'cpuset.cpus' 中的所CPU 包含在一个单一调度域中，确保负载均衡可以将任务（未被其他方式固定的，如 sched_setaffinity）从cpuset 中的任何 CPU 移动到任何其CPU
当每 cpuset 标志 "cpuset.sched_load_balance" 被禁用时，调度器将避免在cpuset 内的 CPU 之间做负载均衡，——除非——因为某个重叠的 cpuset 启用"sched_load_balance" 而必须这样做
因此，例如，如果顶层 cpuset 启用了标"cpuset.sched_load_balance"，那么调度器将有一个覆盖所CPU 的单一调度域，任何其他 cpusets "cpuset.sched_load_balance" 标志的设置都无关紧要，因为我们已经在做完全负载均衡了
因此，在上述两种情况下，应禁用顶cpuset 标志 "cpuset.sched_load_balance"，并且只有一些较小的cpusets 启用此标志
这样做时，你通常不想在顶cpuset 中留下任何可能被固定（pinned）的任务，这些任务可能会使用不可忽略量的 CPU，因为此类任务可能被人为约束到某CPU 子集，取决于后代 cpusets 中此标志设置的具体情况。即使此类任务可以使用其他一CPU 中的空闲 CPU 周期，内核调度器也可能不会考虑将该任务负载均衡到那个未充分利用CPU 上的可能性
当然，被固定到特CPU 的任务可以留在一个禁"cpuset.sched_load_balance" cpuset 中，因为这些任务本来也不会去任何其他地方
这里cpusets 和调度域之间存在阻抗失配（impedance mismatch）。Cpusets 是层次化且嵌套的。调度域是扁平的；它们不重叠，每CPU 最多在一个调度域中
调度域必须是扁平的，因为对部分重叠的 CPU 集合做负载均衡会带来超出我们理解的、不稳定的动态。因此，如果两个部分重叠cpusets 都启用了标志 'cpuset.sched_load_balance'，那么我们会形成一个包含两个的超集的单一调度域。我们不会将任务移动到其 cpuset 之外CPU，但调度器负载均衡代码可能会浪费一些计算周期来考虑这种可能性
这种失配就是为什么哪cpusets 启用了标"cpuset.sched_load_balance" 与调度域配置之间没有简单的一一对应关系的原因。如果一cpuset 启用了该标志，它将获得其所CPU 上的均衡，但如果它禁用了该标志，则只有在没有其他重叠 cpuset 启用该标志时，才能保证没有负载均衡
如果两个 cpusets 'cpuset.cpus' 允许集合部分重叠，且只有其中一个启用了该标志，那么另一个可能会发现其任务仅在重叠的 CPU 上被部分负载均衡。这只是前面几段给出的顶cpuset 示例的一般情况。在一般情况下，如同顶cpuset 情况一样，不要将可能使用不可忽略量 CPU 的任务留在这样的部分负载均衡 cpusets 中，因为它们可能被人为约束到允许给它们的某些 CPU 子集，因为缺乏到其他 CPU 的负载均衡
"cpuset.isolcpus" 中的 CPU isolcpus= 内核启动选项排除在负载均衡之外，并且无论任何 cpuset "cpuset.sched_load_balance" 的值如何，都永远不会被负载均衡
### 1.7.1 sched_load_balance 实现细节

cpuset 标志 'cpuset.sched_load_balance' 默认启用（与大多cpuset 标志相反）。当为某cpuset 启用时，内核将确保可以在cpuset 的所CPU 上做负载均衡（确保该 cpuset cpus_allowed 中的所CPU 都在同一个调度域中）
如果两个重叠cpusets 都启用了 'cpuset.sched_load_balance'，那么它们将（必须）都在同一个调度域中
如果如默认情况，顶层 cpuset 启用'cpuset.sched_load_balance'，那么根据上述，意味着存在一个覆盖整个系统的单一调度域，无论任何其他 cpuset 设置如何
内核向用户空间承诺它将尽可能避免负载均衡。它会选择尽可能细粒度的调度域划分，同时仍为任何允许了 'cpuset.sched_load_balance' cpuset CPU 集合提供负载均衡
内核内部 cpuset 到调度器的接口，cpuset 代码向调度器代码传递系统中负载均衡 CPU 的一个划分（partition）。此划分是一组子集（表示struct cpumask 数组），两两不相交，覆盖所有必须被负载均衡CPU
cpuset 代码构建一个新的此类划分并传递给调度器调度域建立代码，以在必要时重建调度域，只要发生以下情况
 - 具有非空 CPU cpuset 'cpuset.sched_load_balance' 标志发生变化 - CPU 从启用了此标志的 cpuset 中加入或移除 - 或具有非CPU 且启用了此标志的 cpuset 'cpuset.sched_relax_domain_level' 值发生变化，
 - 或移除了一个具有非CPU 且启用了此标志的 cpuset - 或某CPU 被下上线
此划分精确地定义了调度器应建立哪些调度域——划分中的每个元素（struct cpumask）对应一个调度域
调度器记住当前活动的调度域划分。当调度器例partition_sched_domains() cpuset 代码被调用来更新这些调度域时，它会将请求的新划分与当前划分进行比较，并为每个变更更新其调度域，移除旧的并添加新的

### 1.8 什么是 sched_relax_domain_level

在调度域中，调度器以两种方式迁移任务：tick 上的周期性负载均衡，以及在某些调度事件发生时
当任务被唤醒时，调度器尝试将任务移动到空CPU 上。例如，如果运行CPU X 上的任务 A 激活了同一 CPU X 上的另一个任B，并且如CPU Y X 的兄弟且处于空闲，那么调度器将任B 迁移CPU Y，以便任B 可以CPU Y 上启动而无需等待 CPU X 上的任务 A
而如果一CPU runqueue 中没有任务了，该 CPU 会在自己即将空闲之前，尝试从其他繁忙 CPU 拉取额外任务来帮助它们
当然，查找可移动任务或空CPU 需要一定的搜索成本，调度器可能不会每次都搜索域中的所CPU。实际上，在某些架构上，事件时的搜索范围被限制在CPU 所在的同一插槽或节点内，tick 上的负载均衡搜索所CPU
例如，假CPU Z CPU X 相对较远。即CPU Z 空闲CPU X 及其兄弟都繁忙，调度器也无法将唤醒的任务 B X 迁移Z，因为它超出了搜索范围。结果，CPU X 上的任务 B 需要等待任A 或等待下一tick 上的负载均衡。对于某些特殊情况的某些应用，等1 tick 可能太长
'cpuset.sched_relax_domain_level' 文件允许你按需请求更改此搜索范围。此文件接受一int 值，大致按如下级别指示搜索范围的大小，否则初始值为 -1，表示该 cpuset 没有请求
====== ===========================================================
  -1   无请求。使用系统默认或遵循其他人的请求   0   不搜索   1   搜索兄弟（核心中的超线程）   2   搜索封装（package）中的核心   3   搜索节点中的 cpu [在非 NUMA 系统= 系统范围]
   4   搜索节点块（chunk of node）中的节[NUMA 系统上]
   5   搜索系统范围 [NUMA 系统上]
====== ===========================================================

并非所有级别都可能存在，并且值可能根据系统架构和内核配置而变化。请查看 /sys/kernel/debug/sched/domains/cpu**/domain**/ 了解系统特定的详细信息
系统默认值依赖于架构。系统默认值可以使relax_domain_level= 启动参数更改
此文件是cpuset 的，并影响其所cpuset 的调度域。因此，如果一cpuset 的标'cpuset.sched_load_balance' 被禁用，那么 'cpuset.sched_relax_domain_level' 没有效果，因为不存在属于cpuset 的调度域
如果多个 cpusets 重叠并因此形成单一调度域，则使用其中的最大值。注意，如果一个请0 而其他为 -1，则使用 0
注意修改此文件会有好有坏的影响，是否可接受取决于你的情况。如果你不确定，不要修改此文件
如果你的情况是：

 - 由于你的特殊应用行为CPU 缓存等特殊的硬件支持，每cpu 之间的迁移成本对你可以假定相当小 - 搜索成本对你没有影响，或者你可以通过管理 cpuset 使其紧凑等来使搜索成本足够小 - 即使牺牲缓存命中率等也需要低延迟   那么增加 'sched_relax_domain_level' 会对你有益

### 1.9 我如何使cpusets

为了最小化 cpusets 对关键内核代码（如调度器）的影响，并且由于内核不支持一个任务直接更新另一个任务的内存放置，更改任务的 cpuset CPU 或内存节点放置，或更改任务附加到cpuset，对任务的影响是微妙的
如果某个 cpuset 的内存节点被修改，那么对于附加到cpuset 的每个任务，下一次内核尝试为该任务分配内存页时，内核会注意到任务 cpuset 的变化，并更新其每任务内存放置以保持在新cpuset 内存放置范围内。如果任务正在使mempolicy MPOL_BIND，并且它所绑定的节点与其新cpuset 重叠，那么任务将继续使用新的 cpuset 中仍然允许的 MPOL_BIND 节点的任何子集。如果任务正在使MPOL_BIND，而现在其 MPOL_BIND 节点都不在新cpuset 中被允许，那么任务将基本上被视为绑定到新cpuset MPOL_BIND（即使其通过 get_mempolicy() 查询NUMA 放置没有改变）。如果一个任务从一cpuset 移动到另一cpuset，那么内核会在下一次尝试为该任务分配内存页时，如上调整该任务的内存放置
如果某个 cpuset 'cpuset.cpus' 被修改，那么cpuset 中的每个任务的允CPU 放置将立即改变。类似地，如果一个任务的 pid 被写入另一cpuset 'tasks' 文件，那么它的允CPU 放置也会立即改变。如果这样的任务之前使用 sched_setaffinity() 调用被绑定到cpuset 的某个子集，那么该任务将被允许在其新 cpuset 中允许的任何 CPU 上运行，从而抵消先sched_setaffinity() 调用的效果
总之，cpuset 被更改的任务的内存放置由内核在该任务下一次分配页时更新，而处理器放置会立即更新
通常，一旦分配了一个页（获得了主存的一个物理页），那么该页就会留在它被分配的节点上，只要它保持分配状态，即使 cpusets 内存放置策略 'cpuset.mems' 随后改变。如cpuset 标志文件 'cpuset.memory_migrate' 被设true，那么当任务附加到该 cpuset 时，该任务在其先cpuset 的节点上分配给它的任何页都会被迁移到任务的新 cpuset。在这些迁移操作中会尽可能保留页cpuset 内的相对放置。例如，如果页在先前 cpuset 的第二个有效节点上，那么页将被放在新 cpuset 的第二个有效节点上
同样，如'cpuset.memory_migrate' 被设true，那么如果该 cpuset 'cpuset.mems' 文件被修改，分配给该 cpuset 中任务的、位于先'cpuset.mems' 设置节点上的页，将被移动'mems' 新设置中的节点上。不在任务先cpuset 中、或不在 cpuset 先前 'cpuset.mems' 设置中的页不会被移动
上述有一个例外。如果使用了热插拔功能来移除当前分配给某cpuset 的所CPU，那么该 cpuset 中的所有任务将被移动到具有非空 CPU 的最近祖先。但是，如果 cpuset 与另一个具有某些任务附加限制的 cgroup 子系统绑定，某些（或全部）任务的移动可能会失败。在这种失败情况下，那些任务将留在原cpuset 中，内核会自动更新它们的 cpus_allowed 以允许所有在CPU。当用于移除内存节点的内存热插拔功能可用时，预期那里也适用类似的例外。一般来说，内核倾向于违cpuset 放置，而不是让一个任务的所有允CPU 或内存节点都离线而导致其饿死
上述还有第二个例外。GFP_ATOMIC 请求是必须立即满足的内核内部分配。如GFP_ATOMIC 分配失败，内核可能会丢弃某些请求，在极少数情况下甚至会崩溃。如果请求无法在当前任务cpuset 内满足，那么我们会放cpuset，并在任何能找到的地方寻找内存。违cpuset 也比给内核施加压力要好
要启动一个包含在cpuset 中的新作业，步骤如下
 1) mkdir /sys/fs/cgroup/cpuset
 2) mount -t cgroup -ocpuset cpuset /sys/fs/cgroup/cpuset
 3) 通过/sys/fs/cgroup/cpuset 虚拟文件系统中执mkdir write（或 echo）来创建新的 cpuset 4) 启动一个将成为新作创始父进程（founding father的任务 5) 通过将其 pid 写入cpuset /sys/fs/cgroup/cpuset tasks 文件，将该任务附加到cpuset 6) 从此创始父任fork、exec clone 作业任务
例如，以下命令序列将建立一个名"Charlie" cpuset，仅包含 CPU 2 3，以及内存节1```

  mount -t cgroup -ocpuset cpuset /sys/fs/cgroup/cpuset
  cd /sys/fs/cgroup/cpuset
  mkdir Charlie
  cd Charlie
  /bin/echo 2-3 > cpuset.cpus
  /bin/echo 1 > cpuset.mems
  /bin/echo $$ > tasks
  sh
  # shell 'sh' 现在运行cpuset Charlie   # 下一行应显示 '/Charlie'
  cat /proc/self/cpuset

```
有几种查询或修改 cpusets 的方式：

 - 直接通过 cpuset 文件系统，使shell 中的各种 cd、mkdir、echo、cat、rmdir 命令，或它们C 中的等价物 - 通过 C libcpuset - 通过 C libcgroup   (https://github.com/libcgroup/libcgroup/)
 - 通过 python 应用 cset   (http://code.google.com/p/cpuset/)

sched_setaffinity 调用也可以在 shell 提示符下使用 SGI runon Robert Love taskset 完成。mbind set_mempolicy 调用可以shell 提示符下使用 numactl 命令（Andi Kleen numa 包的一部分）完成
## 2. 使用示例与语

### 2.1 基本用法


创建、修改、使cpusets 可以通过 cpuset 虚拟文件系统完成
要挂载它，输入：
# mount -t cgroup -o cpuset cpuset /sys/fs/cgroup/cpuset

然后/sys/fs/cgroup/cpuset 下你可以找到一个对应于系统cpusets 树的树。例如，/sys/fs/cgroup/cpuset 是持有整个系统的 cpuset
```

  # cd /sys/fs/cgroup/cpuset
  # mkdir my_cpuset

```
```

  # cd my_cpuset

```
```

  # ls
  cgroup.clone_children  cpuset.memory_pressure
  cgroup.event_control   cpuset.memory_spread_page
  cgroup.procs           cpuset.memory_spread_slab
  cpuset.cpu_exclusive   cpuset.mems
  cpuset.cpus            cpuset.sched_load_balance
  cpuset.mem_exclusive   cpuset.sched_relax_domain_level
  cpuset.mem_hardwall    notify_on_release
  cpuset.memory_migrate  tasks

```
读取它们会给你关于此 cpuset 状态的信息：它可以使用CPU 和内存节点、正在使用它的进程、它的属性。通过写入这些文件你可以操纵该 cpuset
```

  # /bin/echo 1 > cpuset.cpu_exclusive

```
```

  # /bin/echo 0-7 > cpuset.cpus

```
```

  # /bin/echo 0-7 > cpuset.mems

```
```

  # /bin/echo $$ > tasks

```
你还可以通过在此处使mkdir 在你cpuset 内创cpusets
```

  # mkdir my_sub_cs

```
```

  # rmdir my_sub_cs

```
如果 cpuset 正在使用中（内部cpusets，或附加了进程），这将失败
注意，出于遗留原因，"cpuset" 文件系统作为 cgroup 文件系统的封装存在```

  mount -t cpuset X /sys/fs/cgroup/cpuset

```
```

  mount -t cgroup -ocpuset,noprefix X /sys/fs/cgroup/cpuset
  echo "/sbin/cpuset_release_agent" > /sys/fs/cgroup/cpuset/release_agent

```
### 2.2 添加/移除 cpus


这是cpus mems 文件中写入时使用的语```

  # /bin/echo 1-4 > cpuset.cpus		-> cpus 列表设置cpus 1,2,3,4
  # /bin/echo 1,2,3,4 > cpuset.cpus	-> cpus 列表设置cpus 1,2,3,4

```
要添CPU cpuset，写入包含该 CPU 的新 CPU 列表
```

  # /bin/echo 1-4,6 > cpuset.cpus	-> cpus 列表设置cpus 1,2,3,4,6

```
类似地，要从 cpuset 中移CPU，写入不包含要移CPU 的新 CPU 列表```

  # /bin/echo "" > cpuset.cpus		-> 清空 cpus 列表

```
### 2.3 设置标志


```

  # /bin/echo 1 > cpuset.cpu_exclusive 	-> 设置标志 'cpuset.cpu_exclusive'
  # /bin/echo 0 > cpuset.cpu_exclusive 	-> 取消设置标志 'cpuset.cpu_exclusive'

```
### 2.4 附加进程


```

  # /bin/echo PID > tasks

```
注意这是 PID，而不PIDs。你一次只能附加一个任务```

  # /bin/echo PID1 > tasks
  # /bin/echo PID2 > tasks
	...
  # /bin/echo PIDn > tasks


```
## 3. 问题


Q:
   '/bin/echo' 是怎么回事
A:
   bash 内建'echo' 命令不会检查对 write() 的调用是否有错误。如果你cpuset 文件系统中使用它，你将无法判断命令是成功还是失败
Q:
   当我附加进程时，只有行中的第一个真正被附加了！

A:
   我们每次write() 的调用只能返回一个错误码。所以你应该也只放一pid
## 4. 联系方式


Web: http://www.bullopensource.org/cpuset
