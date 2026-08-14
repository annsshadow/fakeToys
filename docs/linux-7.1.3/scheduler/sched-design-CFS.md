
## CFS Scheduler



## 1.  OVERVIEW


CFS 是“Completely Fair Scheduler”（完全公平调度器）的缩写，是由 Ingo Molnar 实现并
于 Linux 2.6.23 合并的“桌面”进程调度器。最初合并时，它是此前标准调度器中
SCHED_OTHER 交互性代码的替代品。如今，CFS 正在为 EEVDF 让路，相关文档可在
Documentation/scheduler/sched-eevdf.rst 找到。

CFS 设计中的 80% 可以用一句话概括：CFS 本质上是在真实硬件上建模了一个“理想的、精确的
多任务 CPU”。

“理想的多任务 CPU”是一个（不存在的 :-））CPU，它拥有 100% 的物理算力，并能以精确的
相等速度并行运行每个任务，每个任务以 1/nr_running 的速度运行。例如：如果有 2 个任务在
运行，它就以 50% 的物理算力运行每个任务——即实际上是并行运行。

在真实硬件上，我们一次只能运行一个任务，因此我们必须引入“虚拟运行时间”（virtual
runtime）的概念。一个任务的虚拟运行时间指明了它在上述理想多任务 CPU 上其下一个时间片
将于何时开始执行。实际上，一个任务的虚拟运行时间是它的实际运行时间，按运行中任务的总数
归一化后的值。



## 2.  FEW IMPLEMENTATION DETAILS


在 CFS 中，虚拟运行时间通过每个任务的 p->se.vruntime（纳秒单位）值来表达和跟踪。这样，
就可以精确地时间戳记录并度量一个任务“本应得到的 CPU 时间”。

   小细节：在“理想”硬件上，任何时刻所有任务都具有相同的 p->se.vruntime 值——即任务会同时
   执行，没有任何任务会偏离“理想”的 CPU 时间份额而“失衡”。

CFS 的任务选择逻辑基于这个 p->se.vruntime 值，因此非常简单：它总是尝试运行具有最小
p->se.vruntime 值的任务（即到目前为止执行最少的任务）。CFS 总是试图在可运行任务之间分配
CPU 时间，尽可能接近“理想的多任务硬件”。

CFS 设计的其余大部分内容都从这个非常简单的概念自然推导出来，加上一些附加的修饰，如
nice 级别、多处理，以及各种用于识别休眠者（sleeper）的算法变体。



## 3.  THE RBTREE


CFS 的设计相当激进：它不使用旧的运行队列（runqueue）数据结构，而是使用一棵按时间排序的
rbtree 来构建未来任务执行的“时间线”，因此没有“数组切换”造成的伪影（之前的标淮调度器和
RSDL/SD 都受此影响）。

CFS 还维护 rq->cfs.min_vruntime 值，这是一个单调递增的计数器，跟踪运行队列中所有任务中
最小的 vruntime。系统所做的总工作量通过 min_vruntime 跟踪；该值用于将新激活的实体尽可能
放置在树的左侧。

运行队列中正在运行的任务总数通过 rq->cfs.load 值统计，它是排队在运行队列上的任务权重
之和。

CFS 维护一棵按时间排序的 rbtree，所有可运行任务按 p->se.vruntime 键排序。CFS 从这个树中
选取“最左”的任务并坚持运行它。随着系统向前推进，已执行的任务被越来越向右放入树中——
缓慢但确定地给每个任务成为“最左任务”的机会，从而在一个确定的时间内获得 CPU。

总结一下，CFS 的工作方式如下：它运行一个任务一小段时间，当该任务调度（或发生一次调度器
tick）时，该任务的 CPU 使用被“记账”：它刚刚使用物理 CPU 的（一小段）时间被加到
p->se.vruntime 上。一旦 p->se.vruntime 高到足以使另一个任务成为它所维护的按时间排序的
rbtree 的“最左任务”（再加上相对于最左任务的一小段“粒度”距离，这样我们就不会过度调度任务
并破坏缓存），那么新的最左任务被选中，当前任务被抢占。



## 4.  SOME FEATURES OF CFS


CFS 使用纳秒粒度的记账，不依赖任何 jiffies 或其他 HZ 细节。因此 CFS 调度器没有之前调度器
那样的“时间片”概念，也完全没有任何启发式规则。它只有一个核心可调参数：

   /sys/kernel/debug/sched/base_slice_ns

它可以用来将调度器从“桌面”（即低延迟）调整到“服务器”（即良好的批处理）工作负载。它默认
为适合桌面工作负载的设置。SCHED_BATCH 也由 CFS 调度器模块处理。

如果 CONFIG_HZ 导致 base_slice_ns < TICK_NSEC，则 base_slice_ns 的值对工作量几乎
没有影响。

由于其设计，CFS 调度器不易受到当今针对标准调度器启发式规则的那些“攻击”的影响：fiftyp.c、
thud.c、chew.c、ring-test.c、massive_intr.c 都能正常工作，不会影响交互性，并产生预期的
行为。

与之前的标准调度器相比，CFS 调度器对 nice 级别和 SCHED_BATCH 的处理要强得多：这两类工作
负载被更加激进地隔离。

SMP 负载均衡已被重新设计/净化：负载均衡代码中不再有运行队列遍历的假设，而是使用调度模块
的迭代器。结果均衡代码变得相当简洁。



## 5. Scheduling policies


CFS 实现了三种调度策略：

  - SCHED_NORMAL（传统上称为 SCHED_OTHER）：用于常规任务的调度策略。

  - SCHED_BATCH：抢占频率远低于常规任务，从而让任务运行更长时间并更好地利用缓存，
    代价是交互性下降。这很适合批处理作业。

  - SCHED_IDLE：这甚至比 nice 19 还要弱，但它不是真正的空闲定时器调度器，以避免陷入
    优先级反转问题而导致机器死锁。

SCHED_FIFO/_RR 在 sched/rt.c 中实现，并符合 POSIX 规范。

util-linux-ng 2.13.1.1 中的 chrt 命令可以设置除 SCHED_IDLE 以外的所有这些策略。



## 6.  SCHEDULING CLASSES


新的 CFS 调度器被设计成引入“调度类”（Scheduling Classes），一个可扩展的调度器模块层级。
这些模块封装了调度策略的细节，并由调度器核心处理，而核心代码不会对它们做过多假设。

sched/fair.c 实现了上述 CFS 调度器。

sched/rt.c 以比之前标准调度器更简单的方式实现了 SCHED_FIFO 和 SCHED_RR 语义。它使用 100
个运行队列（对应全部 100 个 RT 优先级级别，而非之前调度器的 140 个），并且不需要 expired
数组。

调度类通过 sched_class 结构实现，其中包含必须在有趣事件发生时被调用的函数钩子（hook）。

以下是（部分）钩子列表：

 - enqueue_task(...)

   当一个任务进入可运行状态时调用。它将调度实体（任务）放入红黑树，并递增 nr_running
   变量。

 - dequeue_task(...)

   当一个任务不再可运行时，调用此函数将对应的调度实体移出红黑树。它递减 nr_running 变量。

 - yield_task(...)

   此函数通过将当前运行任务的位置向后移动来让出 CPU，以便其他可运行任务先被调度。

 - wakeup_preempt(...)

   此函数检查进入可运行状态的任务是否应当抢占当前运行的任务。

 - pick_next_task(...)

   此函数选择下一个最适合运行的合格任务。

 - set_next_task(...)

   当一个任务改变其调度类、改变其任务组或被调度时，调用此函数。

 - task_tick(...)

   此函数主要从时间 tick 函数调用；它可能导致进程切换。它驱动运行时的抢占。




## 7.  GROUP SCHEDULER EXTENSIONS TO CFS


通常，调度器以单个任务为单位运行，并力求为每个任务提供公平的 CPU 时间。有时，可能需要
将任务分组，并为每个这样的任务组提供公平的 CPU 时间。例如，可能希望先为系统上的每个用户
提供公平的 CPU 时间，然后为每个属于该用户的任务提供公平的 CPU 时间。

CONFIG_CGROUP_SCHED 正是力求实现这一点。它允许将任务分组，并在这些组之间公平地划分 CPU
时间。

CONFIG_RT_GROUP_SCHED 允许对实时（即 SCHED_FIFO 和 SCHED_RR）任务进行分组。

CONFIG_FAIR_GROUP_SCHED 允许对 CFS（即 SCHED_NORMAL 和 SCHED_BATCH）任务进行分组。

   这些选项需要定义 CONFIG_CGROUPS，并让管理员使用“cgroup”伪文件系统创建任意的任务组。
   有关此文件系统的更多信息，请参阅 Documentation/admin-guide/cgroup-v1/cgroups.rst。

当定义了 CONFIG_FAIR_GROUP_SCHED 时，会为使用伪文件系统创建的每个组创建一个“cpu.shares”
文件。请参阅下面的示例步骤以创建
```

	# mount -t tmpfs cgroup_root /sys/fs/cgroup
	# mkdir /sys/fs/cgroup/cpu
	# mount -t cgroup -ocpu none /sys/fs/cgroup/cpu
	# cd /sys/fs/cgroup/cpu

	# mkdir multimedia	# create "multimedia" group of tasks
	# mkdir browser		# create "browser" group of tasks

	# #Configure the multimedia group to receive twice the CPU bandwidth
	# #that of browser group

	# echo 2048 > multimedia/cpu.shares
	# echo 1024 > browser/cpu.shares

	# firefox &	# Launch firefox and move it to "browser" group
	# echo <firefox_pid> > browser/tasks

	# #Launch gmplayer (or your favourite movie player)
	# echo <movie_player_pid> > multimedia/tasks

```