## 能量感知调度


### 1. 引言


能量感知调度（Energy Aware Scheduling，简EAS）使调度器能够预测其决策CPU 能耗的影响。EAS 依赖CPU 的能量模型（Energy Model，简EM），为每项任务选择一个高能效CPU，同时将对吞吐量的影响降到最低。本文旨在介EAS 的工作原理、其背后的主要设计决策，并说明使其运行所需的要素
```

   /!\ EAS 不支持具有对CPU 拓扑的平/!\

```
EAS 仅在异构 CPU 拓扑（例Arm big.LITTLE）上运行，因为这类系统通过调度节省能耗的潜力最大
EAS 实际使用EM 并非由调度器维护，而由一个专门的框架维护。有关该框架及其提供的功能，请参阅其文档（见 Documentation/power/energy-model.rst）

### 2. 背景与术

首先明确一点：
 - energy = [joule]（资源，例如供电设备上的电池 - power = energy/time = [joule/second] = [watt]

EAS 的目标是在完成任务的同时尽量减少能耗。也就是```

	performance [inst/s]
	--------------------
	    power [W]

```
```

	energy [J]
	-----------
	instruction

```
同时还要获得“良好”的性能。它本质上是当前调度器“仅性能”目标的另一种优化目标。该替代目标考虑两个方面的目标：能效与性能
引入 EM 的理念在于让调度器能够评估其决策的影响，而不是盲目地套用那些可能仅在某些平台上才产生积极效果的节能技术。同时，EM 必须尽可能简单，以尽量减小对调度器延迟的影响
简而言之，EAS 改变CFS 任务被分配到 CPU 的方式。当调度器需要决定任务在哪里运行（唤醒期间）时，EM 用于在多个优秀CPU 候选者之间打破平局，并选择那个预计能在不损害系统吞吐量的前提下产生最佳能耗的 CPU。EAS 的预测依赖于关于平台拓扑的特定知识要素，包括 CPU 的“capacity”（算力）及其各自的能耗成本

### 3. 拓扑信息


EAS（以及调度器的其余部分）使用“capacity”（算力）的概念来区分具有不同计算吞吐量CPU。某CPU 的“capacity”表示其在最高频率下运行时相对于系统中算力最CPU 所能吸收的工作量。capacity 值在 1024 的范围内归一化，并可Per-Entity Load Tracking（PELT，每实体负载跟踪）机制计算得出的任务CPU 利用率信号进行比较。借助 capacity 和利用率数值，EAS 能够估计任务/CPU 的“大繁忙程度”，并在评估性能与能耗的权衡时将其考虑在内。CPU capacity 通过架构相关代码中的 arch_scale_cpu_capacity() 回调提供
EAS 所使用的其余平台知识直接读取自能量模型（EM）框架。一个平台的 EM 由系统中每个“性能域”（performance domain）的功耗成本表组成（有关性能域的更多细节，请参阅 Documentation/power/energy-model.rst）
调度器在构建或重建调度域时，在拓扑代码中管理EM 对象的引用。对于每个根域（root domain，rd），调度器维护一个单向链表，包含与当rd->span 相交的所有性能域。链表中的每个节点都包含一个指EM 框架提供struct em_perf_domain 的指针
这些链表挂载到根域上，以应对独占（exclusive）cpuset 配置。由于独cpuset 的边界不一定与性能域的边界一致，不同根域的链表可能包含重复的元素
示例 1.
    考虑一个具12 CPU 的平台，分为 3 个性能```

	          CPUs:   0 1 2 3 4 5 6 7 8 9 10 11
	          PDs:   |--pd0--|--pd4--|---pd8---|
	          RDs:   |----rd1----|-----rd2-----|

    现在，考虑用户空间决定用两个独cpuset 来切分系统，从而创建了两个独立的根域，每个包含 6 CPU。上图中这两个根域记rd1 rd2。由pd4 rd1 rd2 都相交，它将出现在挂载到各自根域的链'->pd' 中：

       * rd1->pd: pd0 -> pd4
       * rd2->pd: pd4 -> pd8

    请注意，调度器将pd4 创建两个重复的链表节点（每个链表各一个）。不过，两者都只持有指EM 框架同一个共享数据结构的指针
```
由于对这些链表的访问可能与热插拔（hotplug）等操作并发进行，它们和调度器操作的其他拓扑结构一样，RCU 保护
EAS 还维护一个静态键（sched_energy_present），当至少有一个根域满EAS 启动的全部条件时该键被启用。这些条件在6 节中总结

### 4. 能量感知的任务放

EAS 重写CFS 任务的唤醒均衡代码。它利用平台EM PELT 信号，在唤醒均衡期间选择一个高能效的目CPU。启EAS 时，select_task_rq_fair() 会调find_energy_efficient_cpu() 来做出放置决策。该函数在每个性能域中寻找空闲 capacity 最大（CPU capacity - CPU utilization）的 CPU，因为这将使我们能把频率保持在最低。接着，该函数检查将任务放置在该处相比将其留prev_cpu（即任务上一次被激活时运行CPU）是否能节省能耗
find_energy_efficient_cpu() 使用 compute_energy() 来估计若唤醒的任务被迁移，系统将消耗多少能量。compute_energy() 查看 CPU 当前的利用率分布，并调整它以“模拟”任务迁移。EM 框架提供 em_pd_energy() API，用于计算给定利用率分布下每个性能域的预期能耗
下面详细描述一个能效优化的任务放置决策示例
示例 2.
    考虑一个（虚构的）平台，具2 个独立的性能域，每个性能域由 2 CPU 组成。CPU0 CPU1 little（小核）CPU；CPU2 CPU3 big（大核）
    调度器必须决定把任务 P 放在何处，其 util_avg = 200，prev_cpu = 0
    CPU 当前的利用率分布如下图所示。CPU 0-3 util_avg 分别4000000 500。每个性能域有三个运行性能点（Operating Performance Point，OPP）。与每个 OPP 关联CPU capacity 和功耗成本列于能量模型表中。P util_avg 在图中标```

     CPU util.
      1024                 - - - - - - -              Energy Model
                                               +-----------+-------------+
                                               |  Little   |     Big     |
       768                 =============       +-----+-----+------+------+
                                               | Cap | Pwr | Cap  | Pwr  |
                                               +-----+-----+------+------+
       512  ===========    - ##- - - - -       | 170 | 50  | 512  | 400  |
                             ##     ##         | 341 | 150 | 768  | 800  |
       341  -PP - - - -      ##     ##         | 512 | 300 | 1024 | 1700 |
             PP              ##     ##         +-----+-----+------+------+
       170  -## - - - -      ##     ##
             ##     ##       ##     ##
           ------------    -------------
            CPU0   CPU1     CPU2   CPU3

      Current OPP: =====       Other OPP: - - -     util_avg (100 each): ##


    find_energy_efficient_cpu() will first look for the CPUs with the
    maximum spare capacity in the two performance domains. In this example,
    CPU1 and CPU3. Then it will estimate the energy of the system if P was
    placed on either of them, and check if that would save some energy
    compared to leaving P on CPU0. EAS assumes that OPPs follow utilization
    (which is coherent with the behaviour of the schedutil CPUFreq
    governor, see Section 6. for more details on this topic).

    **Case 1. P is migrated to CPU1**::

      1024                 - - - - - - -

                                            Energy calculation:
       768                 =============     * CPU0: 200 / 341 * 150 = 88
                                             * CPU1: 300 / 341 * 150 = 131
                                             * CPU2: 600 / 768 * 800 = 625
       512  - - - - - -    - ##- - - - -     * CPU3: 500 / 768 * 800 = 520
                             ##     ##          => total_energy = 1364
       341  ===========      ##     ##
                    PP       ##     ##
       170  -## - - PP-      ##     ##
             ##     ##       ##     ##
           ------------    -------------
            CPU0   CPU1     CPU2   CPU3


    **Case 2. P is migrated to CPU3**::

      1024                 - - - - - - -

                                            Energy calculation:
       768                 =============     * CPU0: 200 / 341 * 150 = 88
                                             * CPU1: 100 / 341 * 150 = 43
                                    PP       * CPU2: 600 / 768 * 800 = 625
       512  - - - - - -    - ##- - -PP -     * CPU3: 700 / 768 * 800 = 729
                             ##     ##          => total_energy = 1485
       341  ===========      ##     ##
                             ##     ##
       170  -## - - - -      ##     ##
             ##     ##       ##     ##
           ------------    -------------
            CPU0   CPU1     CPU2   CPU3


    **Case 3. P stays on prev_cpu / CPU 0**::

      1024                 - - - - - - -

                                            Energy calculation:
       768                 =============     * CPU0: 400 / 512 * 300 = 234
                                             * CPU1: 100 / 512 * 300 = 58
                                             * CPU2: 600 / 768 * 800 = 625
       512  ===========    - ##- - - - -     * CPU3: 500 / 768 * 800 = 520
                             ##     ##          => total_energy = 1437
       341  -PP - - - -      ##     ##
             PP              ##     ##
       170  -## - - - -      ##     ##
             ##     ##       ##     ##
           ------------    -------------
            CPU0   CPU1     CPU2   CPU3


```
根据计算，情1 的总能耗最低。因此从能效角度看，CPU 1 是最佳候选
大核 CPU 通常比小核更耗电，因此主要在任务无法放入小核时使用。然而，小核并不一定始终比大核更节能。例如，对于某些系统，小核的OPP 可能比大核的最OPP 能效更低。因此，如果小核在某个时刻恰好利用率足够高，那么此刻唤醒的一个小任务即便能放入小核，也可能更适合在大核上执行以节省能耗
即便在大核的所OPP 都比小核低效的情况下，在特定条件下用大核运行小任务仍可能节省能耗。实际上，将任务放在小核上可能导致整个性能域的 OPP 被抬高，从而增加已在其中运行的任务的能耗成本。如果唤醒的任务被放在大核上，其自身的执行成本可能高于在小核上运行，但它不会影响小核上的其他任务，那些任务会继续以较低的 OPP 运行。因此，CPU 消耗的总能耗来看，在一个大核上运行该任务的额外成本，可能小于为所有其他任务抬高小OPP 所带来的成本
若不掌握系统所CPU 在不OPP 下运行的成本，上述示例几乎不可能以通用方式、对全部平台都处理正确。得益于其基EM 的设计，EAS 应当能够较为正确地应对这些情况。然而，为了在高利用率场景下将对吞吐量的影响降到最低，EAS 还实现了另一种称为“over-utilization”（过度利用）的机制

### 5. 过度利用


总体而言，EAS 最能发挥作用的场景是涉及轻/中等 CPU 利用率的用例。只要运行长时间占用 CPU 的任务，它们就需要所有可用的 CPU 算力，调度器在没有严重损害吞吐量的前提下几乎无法节省能耗。为了避EAS 损害性能，一CPU 的使用超过其计算算力80%，就会被标记为“over-utilized”（过度利用）。只要根域中没有 CPU 被过度利用，负载均衡就会被禁用，并由 EAS 接管唤醒均衡代码。如果能在不损害吞吐量的前提下，EAS 可能会比其他 CPU 更多地加载系统中能效最高的 CPU。因此，负载均衡器被禁用，以防止它破EAS 找到的节能任务放置方案。当系统未被过度利用时这样做是安全的，因为低80% 这个临界值意味着
    a. 所CPU 上都有一些空闲时间，因此 EAS 使用的利用率信号很可能准确代表系统中各种任务的“大小”；
    b. 所有任务都已被提供足够CPU 算力，无论其 nice 值如何；
    c. 既然存在空闲算力，所有任务必然定期阻休眠，因此仅需在唤醒时均衡即可
一旦有某个 CPU 越过 80% 临界值，上述三个假设中至少有一个就不再成立。在这种情况下，整个根域会被置上“overutilized”标志，EAS 被禁用，负载均衡器重新启用。这样一来，CPU 受限的条件下，调度器回退到基于负载的算法来进行唤醒与负载均衡，从而更好地尊重任务nice 值
由于过度利用的概念在很大程度上依赖于检测系统中是否存在空闲时间，因此必须考虑被更高（CFS）调度类（以IRQ）所“窃取”的 CPU 算力。因此，过度利用的检测不仅会计入 CFS 任务所使用的算力，也会计入其他调度类和 IRQ 所使用的算力

### 6. EAS 的依赖与要求


能量感知调度依赖于系CPU 具备特定的硬件属性，并依赖于内核其他特性的启用。本节列出这些依赖项，并给出如何满足它们的提示

##### 6.1 - 非对CPU 拓扑


如引言所述，目前 EAS 仅在具有非对CPU 拓扑的平台上受支持。该要求在运行时通过在构建调度域时查SD_ASYM_CPUCAPACITY_FULL 标志的存在来校验
有关sched_domain 层级中设置该标志所需满足的条件，请参Documentation/scheduler/sched-capacity.rst
请注意，EAS 在根本上SMP 并不冲突，但目前尚未观察SMP 平台上有显著的节能效果。如果未来被证明相反，这一限制可能会被修正

##### 6.2 - 能量模型的存

EAS 使用平台EM 来估计调度决策对能耗的影响。因此，要使 EAS 启动，你的平台必须向 EM 框架提供功耗成本表。为此，请参Documentation/power/energy-model.rst 中独EM 框架的文档
另请注意，EM 注册后需要重新构建调度域，才能启EAS
EAS 使用 EM 来对能耗做出预测性决策，因此在检查任务放置的可能选项时更关注差异。对EAS 而言，EM 功耗值是以毫瓦（milli-Watts）还是以“抽象标度”表示都无关紧要

##### 6.3 - 能量模型的复杂度


EAS PD/OPP/CPU 的数量不施加任何复杂度限制，但将 CPU 数量限制EM_MAX_NUM_CPUS，以防止在能量估计期间发生溢出

##### 6.4 - Schedutil 璋冭妭鍣?

EAS 试图预测 CPU 在不久的将来会以哪个 OPP 运行，以估计其能耗。为此，假设 CPU OPP 跟随其利用率变化
尽管在实践中很难对这一假设的准确性提供硬性保证（例如，因为硬件可能并不按所告知的方式运行），但与其CPUFreq 调节器不同，schedutil 至少会使用利用率信号计算出的频率发起“请求”（_requests_）。因此，EAS 配合使用的唯一合理调节器是 schedutil，因为它是唯一一个在频率请求与能耗预测之间提供某种程度一致性的调节器
不支持将 EAS schedutil 以外的任何调节器一起使用

##### 6.5 与规模无关的利用率信

为了CPU 和所有性能状态做出准确预测，EAS 需要频率无关（frequency-invariant）和 CPU 无关（CPU-invariant）的 PELT 信号。这些可以通过架构定义arch_scale{cpu,freq}_capacity() 回调获得
不支持在未实现这两个回调的平台上EAS 中使用

##### 6.6 多线程（SMT

目前形式EAS SMT 无感知，无法利用多线程硬件来节省能耗。EAS 将线程视为独立的 CPU，这实际上可能对性能和能耗都产生反效果
不支持在 SMT 上使EAS