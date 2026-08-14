## 截止期任务调度


    0. 警告（WARNING）
    1. 概述（Overview）
    2. 调度算法（Scheduling algorithm）
      2.1 主算法（Main algorithm）
      2.2 带宽回收（Bandwidth reclaiming）
    3. 调度实时任务（Scheduling Real-Time Tasks）
      3.1 定义（Definitions）
      3.2 单处理器系统的可调度性分析（Schedulability Analysis for Uniprocessor Systems）
      3.3 多处理器系统的可调度性分析（Schedulability Analysis for Multiprocessor Systems）
      3.4 与 SCHED_DEADLINE 参数的关系（Relationship with SCHED_DEADLINE Parameters）
    4. 带宽管理（Bandwidth management）
      4.1 系统级设置（System-wide settings）
      4.2 任务接口（Task interface）
      4.3 默认行为（Default behavior）
      4.4 sched_yield() 的行为（Behavior of sched_yield()）
    5. 任务的 CPU 亲和性（Tasks CPU affinity）
      5.1 使用 cgroup v1 cpuset 控制器（Using cgroup v1 cpuset controller）
      5.2 使用 cgroup v2 cpuset 控制器（Using cgroup v2 cpuset controller）
    6. 未来计划（Future plans）
    A. 测试套件（Test suite）
    B. 最小 main()（Minimal main()）


## 0. 警告


 随意改动这些设置可能导致系统行为不可预测甚至不稳定。对于 -rt（组）调度，假定 root 用户清楚自己在做什么。


## 1. 概述


 sched_dl 调度类中的 SCHED_DEADLINE 策略本质上是 Earliest Deadline First（EDF，最早截止期优先）调度算法的实现，并辅以一种机制（称为 Constant Bandwidth Server，CBS，恒定带宽服务器），使得任务之间的行为能够相互隔离。


## 2. 调度算法


### 2.1 主算法


 SCHED_DEADLINE [^18^] 使用三个参数，分别为 "runtime"（运行时间）、"period"（周期）和 "deadline"（截止期），对任务进行调度。一个 SCHED_DEADLINE 任务应当每 "period" 微秒获得 "runtime" 微秒的执行时间，且这些 "runtime" 微秒在周期开始后的 "deadline" 微秒之内可用。为了实现这一行为，每次任务被唤醒时，调度器都会依据保证（使用 CBS[2,3] 算法）计算一个 "调度截止期"（scheduling deadline）。随后任务依据这些调度截止期采用 EDF[^1^] 进行调度（选择调度截止期最早的任务执行）。请注意，只有在使用了恰当的 "准入控制"（admission control，参见第 "4. 带宽管理" 节）策略时，任务才能在实际的 "deadline" 内获得 "runtime" 时间单位（显然，若系统过载，这一保证无法被遵守）。

 总而言之，CBS[2,3] 算法为任务分配调度截止期，使得每个任务在每个周期内最多运行其 runtime，从而避免不同任务之间的相互干扰（带宽隔离）；而 EDF[^1^] 算法则选择调度截止期最早的任务作为下一个要执行的任务。得益于这一特性，那些并不完全符合 "传统" 实时任务模型（参见第 3 节）的任务也能有效地使用这一新策略。

 更详细地说，CBS 算法按照以下方式为任务分配调度截止期：

  - 每个 SCHED_DEADLINE 任务由 "runtime"、"deadline" 和 "period" 参数所刻画；

  - 任务的状态由一个 "调度截止期" 和一个 "剩余运行时间" 描述。这两个参数初始被设为 0；

  - 当一个 SCHED_DEADLINE 任务被唤醒（变为可执行状态）时，

```
                 remaining runtime                  runtime
        ----------------------------------    >    ---------
        scheduling deadline - current time           period

    then, if the scheduling deadline is smaller than the current time, or
    this condition is verified, the scheduling deadline and the
    remaining runtime are re-initialized as

         scheduling deadline = current time + deadline
         remaining runtime = runtime

    otherwise, the scheduling deadline and the remaining runtime are
    left unchanged;

  - When a SCHED_DEADLINE task executes for an amount of time t, its
    remaining runtime is decreased as::

         remaining runtime = remaining runtime - t

    (technically, the runtime is decreased at every tick, or when the
    task is descheduled / preempted);

  - When the remaining runtime becomes less or equal than 0, the task is
    said to be "throttled" (also known as "depleted" in real-time literature)
    and cannot be scheduled until its scheduling deadline. The "replenishment
    time" for this task (see next item) is set to be equal to the current
    value of the scheduling deadline;

  - When the current time is equal to the replenishment time of a
    throttled task, the scheduling deadline and the remaining runtime are
    updated as::

         scheduling deadline = scheduling deadline + period
         remaining runtime = remaining runtime + runtime

 The SCHED_FLAG_DL_OVERRUN flag in sched_attr's sched_flags field allows a task
 to get informed about runtime overruns through the delivery of SIGXCPU
 signals.

```

### 2.2 带宽回收


 截止期任务的带宽回收基于 GRUB（Greedy Reclamation of Unused Bandwidth，未使用带宽的贪婪回收）算法 [15, 16, 17]，并在设置 SCHED_FLAG_RECLAIM 标志时启用。

```

                             ------------
                 (d)        |   Active   |
              ------------->|            |
              |             | Contending |
              |              ------------
              |                A      |
          ----------           |      |
         |          |          |      |
         | Inactive |          |(b)   | (a)
         |          |          |      |
          ----------           |      |
              A                |      V
              |              ------------
              |             |   Active   |
              --------------|     Non    |
                 (c)        | Contending |
                             ------------

 A task can be in one of the following states:

  - ActiveContending: if it is ready for execution (or executing);

  - ActiveNonContending: if it just blocked and has not yet surpassed the 0-lag
    time;

  - Inactive: if it is blocked and has surpassed the 0-lag time.

 State transitions:

  (a) When a task blocks, it does not become immediately inactive since its
      bandwidth cannot be immediately reclaimed without breaking the
      real-time guarantees. It therefore enters a transitional state called
      ActiveNonContending. The scheduler arms the "inactive timer" to fire at
      the 0-lag time, when the task's bandwidth can be reclaimed without
      breaking the real-time guarantees.

      The 0-lag time for a task entering the ActiveNonContending state is
      computed as::

                        (runtime * dl_period)
             deadline - ---------------------
                             dl_runtime

      where runtime is the remaining runtime, while dl_runtime and dl_period
      are the reservation parameters.

  (b) If the task wakes up before the inactive timer fires, the task re-enters
      the ActiveContending state and the "inactive timer" is canceled.
      In addition, if the task wakes up on a different runqueue, then
      the task's utilization must be removed from the previous runqueue's active
      utilization and must be added to the new runqueue's active utilization.
      In order to avoid races between a task waking up on a runqueue while the
      "inactive timer" is running on a different CPU, the "dl_non_contending"
      flag is used to indicate that a task is not on a runqueue but is active
      (so, the flag is set when the task blocks and is cleared when the
      "inactive timer" fires or when the task  wakes up).

  (c) When the "inactive timer" fires, the task enters the Inactive state and
      its utilization is removed from the runqueue's active utilization.

  (d) When an inactive task wakes up, it enters the ActiveContending state and
      its utilization is added to the active utilization of the runqueue where
      it has been enqueued.

 For each runqueue, the algorithm GRUB keeps track of two different bandwidths:

  - Active bandwidth (running_bw): this is the sum of the bandwidths of all
    tasks in active state (i.e., ActiveContending or ActiveNonContending);

  - Total bandwidth (this_bw): this is the sum of all tasks "belonging" to the
    runqueue, including the tasks in Inactive state.

  - Maximum usable bandwidth (max_bw): This is the maximum bandwidth usable by
    deadline tasks and is currently set to the RT capacity.


 The algorithm reclaims the bandwidth of the tasks in Inactive state.
 It does so by decrementing the runtime of the executing task Ti at a pace equal
 to

           dq = -(max{ Ui, (Umax - Uinact - Uextra) } / Umax) dt

 where:

  - Ui is the bandwidth of task Ti;
  - Umax is the maximum reclaimable utilization (subjected to RT throttling
    limits);
  - Uinact is the (per runqueue) inactive utilization, computed as
    (this_bq - running_bw);
  - Uextra is the (per runqueue) extra reclaimable utilization
    (subjected to RT throttling limits).


 Let's now see a trivial example of two deadline tasks with runtime equal
 to 4 and period equal to 8 (i.e., bandwidth equal to 0.5)::

         A            Task T1
         |
         |                               |
         |                               |
         |--------                       |----
         |       |                       V
         |---|---|---|---|---|---|---|---|--------->t
         0   1   2   3   4   5   6   7   8


         A            Task T2
         |
         |                               |
         |                               |
         |       ------------------------|
         |       |                       V
         |---|---|---|---|---|---|---|---|--------->t
         0   1   2   3   4   5   6   7   8


         A            running_bw
         |
       1 -----------------               ------
         |               |               |
      0.5-               -----------------
         |                               |
         |---|---|---|---|---|---|---|---|--------->t
         0   1   2   3   4   5   6   7   8


  - Time t = 0:

    Both tasks are ready for execution and therefore in ActiveContending state.
    Suppose Task T1 is the first task to start execution.
    Since there are no inactive tasks, its runtime is decreased as dq = -1 dt.

  - Time t = 2:

    Suppose that task T1 blocks
    Task T1 therefore enters the ActiveNonContending state. Since its remaining
    runtime is equal to 2, its 0-lag time is equal to t = 4.
    Task T2 start execution, with runtime still decreased as dq = -1 dt since
    there are no inactive tasks.

  - Time t = 4:

    This is the 0-lag time for Task T1. Since it didn't woken up in the
    meantime, it enters the Inactive state. Its bandwidth is removed from
    running_bw.
    Task T2 continues its execution. However, its runtime is now decreased as
    dq = - 0.5 dt because Uinact = 0.5.
    Task T2 therefore reclaims the bandwidth unused by Task T1.

  - Time t = 8:

    Task T1 wakes up. It enters the ActiveContending state again, and the
    running_bw is incremented.


```

### 2.3 能效感知调度


 当选择 cpufreq 的 schedutil 调控器（governor）时，SCHED_DEADLINE 会实现 GRUB-PA [^19^] 算法，将 CPU 工作频率降低到仍能满足截止期的最小值。该行为目前仅针对 ARM 架构实现。

 若改变频率所需的时间与预留周期处于同一数量级，则需格外注意。在这种情况下，设置固定的 CPU 频率反而会带来更少的截止期错失。


## 3. 调度实时任务



 ..  BIG FAT WARNING ******************************************************

```

   This section contains a (not-thorough) summary on classical deadline
   scheduling theory, and how it applies to SCHED_DEADLINE.
   The reader can "safely" skip to Section 4 if only interested in seeing
   how the scheduling policy can be used. Anyway, we strongly recommend
   to come back here and continue reading (once the urge for testing is
   satisfied :P) to be sure of fully understanding all technical details.

 .. ************************************************************************

```

 任何类型的任务都可以利用这一新的调度机制，尽管应该说它特别适合那些需要对时序行为提供保证的周期性或零星（sporadic）实时任务，例如多媒体、流媒体、控制应用等。


### 3.1 定义


 一个典型的实时任务由一系列计算阶段（任务实例，或称作业，jobs）的重复组成，这些阶段以周期性或零星（sporadic）的方式被激活。每个作业 J_j（其中 J_j 是任务的第 j 个作业）由到达时间 r_j（作业开始的时间）、完成作业所需的计算时间 c_j，以及作业的绝对截止期 d_j（作业应当在该时间之前完成）所刻画。最大执行时间 max{c_j} 被称为该任务的 "最坏情况执行时间"（Worst Case Execution Time，WCET）。如果 r_{j+1} = r_j + P，则实时任务可以是周期为 P 的周期任务；或者以最小到达间隔 P 满足 r_{j+1} >= r_j + P 的零星任务。最后，d_j = r_j + D，其中 D 是任务的相对截止期。总而言之，一个实时任务可以描述为

	Task = (WCET, D, P)

 实时任务的利用率（utilization）定义为其 WCET 与周期（或最小到达间隔）之比，表示执行该任务所需的 CPU 时间比例。

 如果总利用率 U=sum(WCET_i/P_i) 大于 M（其中 M 等于 CPU 数量），那么调度器将无法遵守所有截止期。请注意，总利用率定义为系统中所有实时任务的利用率 WCET_i/P_i 之和。当考虑多个实时任务时，第 i 个任务的参数用 "_i" 后缀表示。此外，如果总利用率大于 M，那么我们就有让实时任务饿死非实时任务的风险。如果，反之，总利用率小于 M，那么非实时任务将不会被饿死，系统或许能够遵守所有截止期。事实上，在这种情况下可以为 tardiness（迟到时间，定义为 0 与作业的完成时间及其绝对截止期之差之间的最大值）提供一个上界。更精确地说，可以证明在使用全局 EDF 调度器时，每个任务的最大 tardiness 小于等于

	((M − 1) · WCET_max − WCET_min)/(M − (M − 2) · U_max) + WCET_max

 其中 WCET_max = max{WCET_i} 为最大 WCET，WCET_min=min{WCET_i} 为最小 WCET，U_max = max{WCET_i/P_i} 为最大利用率[^12^]。

### 3.2 单处理器系统的可调度性分析


 如果 M=1（单处理器系统），或者在采用分区调度（每个实时任务被静态地分配到唯一一个 CPU）的情况下，可以形式化地检查是否所有截止期都被遵守。如果对所有任务都有 D_i = P_i，那么当且仅当运行于该 CPU 上的任务总利用率小于等于 1 时，EDF 才能遵守运行于该 CPU 上所有任务的全部截止期。如果某些任务的 D_i != P_i，则可以将任务的密度定义为 WCET_i/min{D_i,P_i}；当运行于该 CPU 上的任务密度之和小于等于 1 时，EDF 能够遵守运行于该 CPU 上所有任务的全部截止期：

	sum(WCET_i / min{D_i, P_i}) <= 1

 需要注意的是，这一条件只是充分的，而非必要的：存在一些任务集是可调度的，却不满足该条件。例如，考虑任务集 {Task_1,Task_2}，其中 Task_1=(50ms,50ms,100ms)，Task_2=(10ms,100ms,100ms)。显然 EDF 能够在不错失任何截止期的情况下调度这两个任务（Task_1 一旦释放即被调度，并刚好在截止期前完成；Task_2 在 Task_1 之后立即被调度，因此其响应时间不会大于 50ms + 10ms = 60ms），即使

	50 / min{50,100} + 10 / min{100, 100} = 50 / 50 + 10 / 100 = 1.1

 当然，也可以检验 D_i != P_i 任务的精确可调度性（即同时满足充分且必要的条件），但这无法通过把总利用率或密度与某个常数比较来完成。取而代之，可以使用所谓的 "处理器需求"（processor demand）方法：计算在时间长度为 t 的区间内，所有任务为遵守其全部截止期所需的总 CPU 时间 h(t)，并将该时间与区间长度 t 进行比较。如果对所有可能的 t 值都有 h(t) 小于 t（即在长度为 t 的时间区间内任务所需的时间小于区间长度），那么 EDF 能够调度这些任务并遵守其全部截止期。由于对所有可能的 t 值执行此检查是不可能的，文献[4,5,6]已证明只需对 0 到最大值 L 之间的 t 值执行测试即可。所引用的论文包含了全部数学细节，并解释了如何计算 h(t) 和 L。无论如何，这类分析过于复杂且耗时，无法在线执行。因此，如第 4 节所述，Linux 使用一个基于任务利用率的准入测试。

### 3.3 多处理器系统的可调度性分析


 在采用全局 EDF 调度（非分区系统）的多处理器系统上，可调度性的充分性测试不能基于利用率或密度：可以证明，即便 D_i = P_i，利用率略大于 1 的任务集也有可能错失截止期，而与 CPU 数量无关。

 考虑一个包含 M+1 个任务的集合 {Task_1,...Task_{M+1}}，运行在具有 M 个 CPU 的系统上。其中第一个任务 Task_1=(P,P,P) 的周期、相对截止期和 WCET 都等于 P。其余 M 个任务 Task_i=(e,P-1,P-1) 具有任意小的最坏情况执行时间（此处记为 "e"）以及比第一个任务更小的周期。因此，如果所有任务都在同一时刻 t 被激活，全局 EDF 会先调度这 M 个任务（因为它们的绝对截止期等于 t + P - 1，比 Task_1 的绝对截止期 t + P 更小）。结果，Task_1 只能在时刻 t + e 被调度，并将在时刻 t + e + P 完成，即在其绝对截止期之后。该任务集的总利用率为 U = M · e / (P - 1) + P / P = M · e / (P - 1) + 1，当 e 取很小的值时，该值可以非常接近 1。这被称为 "Dhall 效应"[^7^]（Dhall's effect）。注：Dhall 原始论文中的例子在此被略微简化（例如，Dhall 更正确地计算了 lim_{e->0}U）。

 实时文献[8,9]中已发展了更复杂的全局 EDF 可调度性测试，但它们同样不是基于总利用率（或密度）与固定常数的简单比较。如果所有任务都有 D_i = P_i，则一个充分的可调度性条件可以简单地表达为：

	sum(WCET_i / P_i) <= M - (M - 1) · U_max

 其中 U_max = max{WCET_i / P_i}[^10^]。注意当 U_max = 1 时，M - (M - 1) · U_max 变为 M - M + 1 = 1，这一可调度性条件恰好印证了 Dhall 效应。关于多处理器实时调度可调度性测试的更完整文献综述可参见 [^11^]。

 如上所述，强制总利用率小于 M 并不能保证全局 EDF 调度任务而不错失任何截止期（换言之，全局 EDF 并非最优调度算法）。然而，总利用率小于 M 足以保证非实时任务不会被饿死，且实时任务的 tardiness 具有上界[^12^]（如前所述）。各种论文[13,14]中已提出了实时任务最大 tardiness 的不同上界，但对 SCHED_DEADLINE 而言重要的理论结论是：如果总利用率小于等于 M，那么任务的响应时间就是有界的。

### 3.4 与 SCHED_DEADLINE 参数的关系


 最后，理解第 2 节描述的 SCHED_DEADLINE 调度参数（runtime、deadline 和 period）与本节描述的实时任务参数（WCET、D、P）之间的关系非常重要。请注意，任务的时间约束由上面描述的绝对截止期 d_j = r_j + D 表示，而 SCHED_DEADLINE 是依据调度截止期对任务进行调度的（参见第 2 节）。如果使用准入测试来保证调度截止期被遵守，那么 SCHED_DEADLINE 就可以用来调度实时任务，并保证一个任务的所有作业截止期都被遵守。为此，必须按如下方式设置任务：

  - runtime >= WCET
  - deadline = D
  - period <= P

 换言之（IOW），如果 runtime >= WCET 且 period <= P，那么调度截止期与绝对截止期（d_j）重合，因此恰当的准入控制可以保证遵守该任务各作业的绝对截止期（这被称为 "硬可调度性属性"，hard schedulability property，是 [^2^] 中引理 1 的扩展）。请注意，如果 runtime > deadline，准入控制一定会拒绝该任务，因为其时间约束无法被遵守。


 参考文献：

  1 - C. L. Liu and J. W. Layland. Scheduling algorithms for multiprogram-
      ming in a hard-real-time environment. Journal of the Association for
      Computing Machinery, 20(1), 1973.
  2 - L. Abeni , G. Buttazzo. Integrating Multimedia Applications in Hard
      Real-Time Systems. Proceedings of the 19th IEEE Real-time Systems
      Symposium, 1998. http://retis.sssup.it/~giorgio/paps/1998/rtss98-cbs.pdf
  3 - L. Abeni. Server Mechanisms for Multimedia Applications. ReTiS Lab
      Technical Report. http://disi.unitn.it/~abeni/tr-98-01.pdf
  4 - J. Y. Leung and M.L. Merril. A Note on Preemptive Scheduling of
      Periodic, Real-Time Tasks. Information Processing Letters, vol. 11,
      no. 3, pp. 115-118, 1980.
  5 - S. K. Baruah, A. K. Mok and L. E. Rosier. Preemptively Scheduling
      Hard-Real-Time Sporadic Tasks on One Processor. Proceedings of the
      11th IEEE Real-time Systems Symposium, 1990.
  6 - S. K. Baruah, L. E. Rosier and R. R. Howell. Algorithms and Complexity
      Concerning the Preemptive Scheduling of Periodic Real-Time tasks on
      One Processor. Real-Time Systems Journal, vol. 4, no. 2, pp 301-324,
      1990.
  7 - S. J. Dhall and C. L. Liu. On a real-time scheduling problem. Operations
      research, vol. 26, no. 1, pp 127-140, 1978.
  8 - T. Baker. Multiprocessor EDF and Deadline Monotonic Schedulability
      Analysis. Proceedings of the 24th IEEE Real-Time Systems Symposium, 2003.
  9 - T. Baker. An Analysis of EDF Schedulability on a Multiprocessor.
      IEEE Transactions on Parallel and Distributed Systems, vol. 16, no. 8,
      pp 760-768, 2005.
  10 - J. Goossens, S. Funk and S. Baruah, Priority-Driven Scheduling of
       Periodic Task Systems on Multiprocessors. Real-Time Systems Journal,
       vol. 25, no. 2–3, pp. 187–205, 2003.
  11 - R. Davis and A. Burns. A Survey of Hard Real-Time Scheduling for
       Multiprocessor Systems. ACM Computing Surveys, vol. 43, no. 4, 2011.
       http://www-users.cs.york.ac.uk/~robdavis/papers/MPSurveyv5.0.pdf
  12 - U. C. Devi and J. H. Anderson. Tardiness Bounds under Global EDF
       Scheduling on a Multiprocessor. Real-Time Systems Journal, vol. 32,
       no. 2, pp 133-189, 2008.
  13 - P. Valente and G. Lipari. An Upper Bound to the Lateness of Soft
       Real-Time Tasks Scheduled by EDF on Multiprocessors. Proceedings of
       the 26th IEEE Real-Time Systems Symposium, 2005.
  14 - J. Erickson, U. Devi and S. Baruah. Improved tardiness bounds for
       Global EDF. Proceedings of the 22nd Euromicro Conference on
       Real-Time Systems, 2010.
  15 - G. Lipari, S. Baruah, Greedy reclamation of unused bandwidth in
       constant-bandwidth servers, 12th IEEE Euromicro Conference on Real-Time
       Systems, 2000.
  16 - L. Abeni, J. Lelli, C. Scordino, L. Palopoli, Greedy CPU reclaiming for
       SCHED DEADLINE. In Proceedings of the Real-Time Linux Workshop (RTLWS),
       Dusseldorf, Germany, 2014.
  17 - L. Abeni, G. Lipari, A. Parri, Y. Sun, Multicore CPU reclaiming: parallel
       or sequential?. In Proceedings of the 31st Annual ACM Symposium on Applied
       Computing, 2016.
  18 - J. Lelli, C. Scordino, L. Abeni, D. Faggioli, Deadline scheduling in the
       Linux kernel, Software: Practice and Experience, 46(6): 821-839, June
       2016.
  19 - C. Scordino, L. Abeni, J. Lelli, Energy-Aware Real-Time Scheduling in
       the Linux Kernel, 33rd ACM/SIGAPP Symposium On Applied Computing (SAC
       2018), Pau, France, April 2018.


## 4. 带宽管理


 如前所述，为了使 -deadline 调度有效且有用（即能够在 "deadline" 内提供 "runtime" 时间单位），必须有一些方法来控制将可用 CPU 时间份额分配给各个任务的方式。这通常被称为 "准入控制"（admission control）；如果不执行它，就无法对 -deadline 任务的实际调度提供任何保证。

 如第 3 节已经说明，正确调度一组实时任务所需遵守的一个必要条件是总利用率小于 M。对于 -deadline 任务而言，这要求所有任务的 runtime 与 period 之比的和小于 M。注意，runtime/period 之比等价于 "传统" 实时任务的利用率，也常被称为 "带宽"（bandwidth）。用于控制可分配给 -deadline 任务的 CPU 带宽的接口，与已用于 -rt 任务的实时组调度（即 RT-throttling，参见 Documentation/scheduler/sched-rt-group.rst）的接口类似，并基于位于 procfs 中、可读可写的控制文件（用于系统级设置）。请注意，针对 -deadline 任务的每个组（per-group）设置（通过 cgroupfs 控制）目前尚未定义，因为还需要更多讨论来确定我们想在任务组层面如何管理 SCHED_DEADLINE 带宽。

 截止期带宽管理与 RT-throttling 的一个主要区别在于：-deadline 任务自身拥有带宽（而 -rt 任务没有！），因此我们无需更高层的限流机制来强制实施期望的带宽。换言之，这意味着接口参数仅在准入控制时（即用户调用 sched_setattr() 时）使用。随后调度会依据任务的实际参数执行，从而以符合其粒度需求的方式将 CPU 带宽分配给 SCHED_DEADLINE 任务。因此，利用这一简单接口，我们可以对 -deadline 任务的总利用率设置上限（即 \Sum (runtime_i / period_i) < global_dl_utilization_cap）。

### 4.1 系统级设置


 系统级设置在 /proc 虚拟文件系统下配置。

 目前 -rt 的旋钮（knobs）被用于 -deadline 的准入控制；在启用 CONFIG_RT_GROUP_SCHED 时，-deadline 的运行时间计入（根）-rt 运行时间。在不启用 CONFIG_RT_GROUP_SCHED 时，该旋钮仅用于 -dl 的准入控制。我们意识到这并非完全理想；不过，暂时拥有一个较小的接口、且便于日后修改，是更好的选择。理想的情况（见第 5 节）是从一个 -deadline 服务器运行 -rt 任务；在这种情况下，-rt 带宽就是 dl_bw 的直接子集。

 这意味着，对于一个包含 M 个 CPU 的 root_domain，只要其带宽之和保持在以下值之下，就可以创建 -deadline 任务：

   M * (sched_rt_runtime_us / sched_rt_period_us)

 也可以禁用这一带宽管理逻辑，从而可以任意地超额订阅系统。这是通过向 /proc/sys/kernel/sched_rt_runtime_us 写入 -1 来实现的。


### 4.2 任务接口


 指定一个周期/零星任务（在每次实例中执行给定的运行时间，并根据自身时序约束的紧迫性进行调度），通常需要一种方式来声明：

  - （最大/典型）实例执行时间，
  - 连续实例之间的最小间隔，
  - 每个实例必须完成的时间约束。

 因此：

  - 提供了一个新的 struct sched_attr，包含全部必要字段；
  - 实现了操作它的新的调度相关系统调用，即 sched_setattr() 和 sched_getattr()。

 SCHED_DEADLINE 任务的剩余运行时间和绝对截止期可以通过 sched_getattr() 系统调用读取，只需将该系统调用的最后一个参数 flags 设为 SCHED_GETATTR_FLAG_DL_DYNAMIC=1。这会更新剩余运行时间，将绝对截止期转换为 CLOCK_MONOTONIC 参考系，然后将这些参数返回给用户空间。绝对截止期以自 CLOCK_MONOTONIC 时间参考系（启动时刻）以来的纳秒数形式返回，作为 sched_attr 的 sched_deadline 字段中的一个 u64，其可表示自启动以来近 585 年（而以 flags=0 调用 sched_getattr() 则返回静态参数）。

 出于调试目的，这些参数也可以通过 /proc/<pid>/sched 获取（条目 dl.runtime 和 dl.deadline，两者单位均为 ns），但是：这种方式效率极低；返回的剩余运行时间不像 sched_getattr() 那样被更新；截止期是以内核 rq_clock 时间参考系提供的，无法直接从用户空间使用。


### 4.3 默认行为


 SCHED_DEADLINE 带宽的默认值将 rt_runtime 设为 950000。由于 rt_period 等于 1000000，默认情况下这意味着对于每个 root_domain，-deadline 任务最多可使用 95% 乘以组成该 root_domain 的 CPU 数量。这意味着非 -deadline 任务将至少获得 5% 的 CPU 时间，并且 -deadline 任务将以保证的最坏情况延迟获得其运行时间（相对于 "deadline" 参数）。如果 "deadline" = "period"，并且使用 cpuset 机制来实现分区调度（参见第 5 节），那么这一简单的带宽管理设置就能够确定性地保证 -deadline 任务在一个周期内获得其运行时间。

 最后请注意，为了不破坏准入控制，-deadline 任务不能 fork（创建子进程）。


### 4.4 sched_yield() 的行为


 当一个 SCHED_DEADLINE 任务调用 sched_yield() 时，它会放弃其剩余运行时间并被立即限流，直到下一个周期其运行时间被补充为止（会设置一个特殊标志 dl_yielded，用于正确处理调用 sched_yield() 之后的限流与运行时间补充）。

 sched_yield() 的这一行为使得任务能在下一个周期开始时恰好被唤醒。此外，这在未来与带宽回收机制结合时可能有用，届时 sched_yield() 会使剩余的运行时间可供其他 SCHED_DEADLINE 任务回收。


## 5. 任务的 CPU 亲和性


 截止期任务的 CPU 亲和性掩码不能小于其创建所在的 root domain。因此，使用 `sched_setaffinity(2)` 不会生效。相反，截止期任务应当创建在一个受限的 root domain 中。这可以通过使用 cgroup v1（已弃用）或 cgroup v2 的 cpuset 控制器来实现。更多信息请参见 Documentation/admin-guide/cgroup-v1/cpusets.rst <cpusets> 和 Documentation/admin-guide/cgroup-v2.rst <cgroup-v2>。

### 5.1 使用 cgroup v1 cpuset 控制器


```

   mkdir /dev/cpuset
   mount -t cgroup -o cpuset cpuset /dev/cpuset
   cd /dev/cpuset
   mkdir cpu0
   echo 0 > cpu0/cpuset.cpus
   echo 0 > cpu0/cpuset.mems
   echo 1 > cpuset.cpu_exclusive
   echo 0 > cpuset.sched_load_balance
   echo 1 > cpu0/cpuset.cpu_exclusive
   echo 1 > cpu0/cpuset.mem_exclusive
   echo $$ > cpu0/tasks
   chrt --sched-runtime 100000 --sched-period 200000 --deadline 0 yes > /dev/null

```

### 5.2 使用 cgroup v2 cpuset 控制器


 假定 cgroup v2 根挂载在 `/sys/fs/cgroup`，下面是一个

```

   cd /sys/fs/cgroup
   echo '+cpuset' > cgroup.subtree_control
   mkdir deadline_group
   echo 0 > deadline_group/cpuset.cpus
   echo 'root' > deadline_group/cpuset.cpus.partition
   echo $$ > deadline_group/cgroup.procs
   chrt --sched-runtime 100000 --sched-period 200000 --deadline 0 yes > /dev/null

```

## 6. 未来计划


 尚缺：

  - 以编程方式获取当前运行时间和绝对截止期的方法；
  - 对截止期继承（deadline inheritance）的改进，特别是关于在非交互任务之间保持带宽隔离的可能性。这正从理论和实践两个角度进行研究，希望我们很快能够产出一些演示性代码；
  - 基于 (c)group 的带宽管理，甚至调度；
  - 针对非 root 用户的访问控制（以及相关安全问题），这是允许非特权用户使用这些机制的最佳方式，以及如何防止非 root 用户 "欺骗" 系统？

 如前所述，我们也计划将这项工作与 EDF 限流补丁 [https://lore.kernel.org/r/cover.1266931410.git.fabio@helm.retis] 合并，但合并仍处于初步阶段，我们非常希望获得反馈，以帮助我们决定其发展方向。

## 附录 A. 测试套件


 SCHED_DEADLINE 策略可以使用两个应用程序轻松测试，它们是更大的 Linux 调度器验证套件的一部分。该套件以 GitHub 仓库形式提供：https://github.com/scheduler-tools。

 第一个测试应用程序名为 rt-app，可用于以特定参数启动多个线程。rt-app 支持 SCHED_{OTHER,FIFO,RR,DEADLINE} 调度策略及其相关参数（例如 niceness、priority、runtime/deadline/period）。rt-app 是一个有价值的工具，因为它可用于合成地重建某些工作负载（或许能模拟真实用例），并评估调度器在此类负载下的行为。这样，结果很容易复现。rt-app 可在以下地址获取：https://github.com/scheduler-tools/rt-app。

 rt-app 不接受命令行参数，而是从一个 JSON 配置文件中读取配置。下面是一个 `config.json` 示例：

 .. code-block:: json

  {
    "tasks": {
      "dl_task": {
        "policy": "SCHED_DEADLINE",
        "priority": 0,
        "dl-runtime": 10000,
        "dl-period": 100000,
        "dl-deadline": 100000
      },
      "fifo_task": {
        "policy": "SCHED_FIFO",
        "priority": 10,
        "runtime": 20000,
        "sleep": 130000
      }
    },
    "global": {
      "duration": 5
    }
  }

 运行 `rt-app config.json` 时，它会创建 2 个线程。第一个由 SCHED_DEADLINE 调度，每 100ms 执行 10ms。第二个以 SCHED_FIFO 优先级 10 调度，每 150ms 执行 20ms。测试总共运行 5 秒。

 有关 JSON 模式及更多示例，请参阅 rt-app 文档。

 第二个测试应用程序使用 chrt 实现，它支持 SCHED_DEADLINE。

```

  # chrt -d -T 10000000 -D 100000000 0 ./my_cpuhog_app

 With this, my_cpuhog_app is put to run inside a SCHED_DEADLINE reservation
 of 10ms every 100ms (note that parameters are expressed in nanoseconds).
 You can also use chrt to create a reservation for an already running
 application, given that you know its pid::

  # chrt -d -T 10000000 -D 100000000 -p 0 my_app_pid

```

## 附录 B. 最小 main()


 下面我们提供一个简单（丑陋）的自包含代码片段，展示实时任务如何创建 SCHED_DEADLINE 预留（reservation）：

```

   #define _GNU_SOURCE
   #include <unistd.h>
   #include <stdio.h>
   #include <stdlib.h>
   #include <string.h>
   #include <time.h>
   #include <linux/unistd.h>
   #include <linux/kernel.h>
   #include <linux/types.h>
   #include <sys/syscall.h>
   #include <pthread.h>

   #define gettid() syscall(__NR_gettid)

   #define SCHED_DEADLINE	6

   /* XXX use the proper syscall numbers */
   #ifdef __x86_64__
   #define __NR_sched_setattr		314
   #define __NR_sched_getattr		315
   #endif

   #ifdef __i386__
   #define __NR_sched_setattr		351
   #define __NR_sched_getattr		352
   #endif

   #ifdef __arm__
   #define __NR_sched_setattr		380
   #define __NR_sched_getattr		381
   #endif

   static volatile int done;

   struct sched_attr {
	__u32 size;

	__u32 sched_policy;
	__u64 sched_flags;

	/* SCHED_NORMAL, SCHED_BATCH */
	__s32 sched_nice;

	/* SCHED_FIFO, SCHED_RR */
	__u32 sched_priority;

	/* SCHED_DEADLINE (nsec) */
	__u64 sched_runtime;
	__u64 sched_deadline;
	__u64 sched_period;
   };

   int sched_setattr(pid_t pid,
		  const struct sched_attr *attr,
		  unsigned int flags)
   {
	return syscall(__NR_sched_setattr, pid, attr, flags);
   }

   int sched_getattr(pid_t pid,
		  struct sched_attr *attr,
		  unsigned int size,
		  unsigned int flags)
   {
	return syscall(__NR_sched_getattr, pid, attr, size, flags);
   }

   void *run_deadline(void *data)
   {
	struct sched_attr attr;
	int x = 0;
	int ret;
	unsigned int flags = 0;

	printf("deadline thread started [%ld]\n", gettid());

	attr.size = sizeof(attr);
	attr.sched_flags = 0;
	attr.sched_nice = 0;
	attr.sched_priority = 0;

	/* This creates a 10ms/30ms reservation */
	attr.sched_policy = SCHED_DEADLINE;
	attr.sched_runtime = 10 * 1000 * 1000;
	attr.sched_period = attr.sched_deadline = 30 * 1000 * 1000;

	ret = sched_setattr(0, &attr, flags);
	if (ret < 0) {
		done = 0;
		perror("sched_setattr");
		exit(-1);
	}

	while (!done) {
		x++;
	}

	printf("deadline thread dies [%ld]\n", gettid());
	return NULL;
   }

   int main (int argc, char **argv)
   {
	pthread_t thread;

	printf("main thread [%ld]\n", gettid());

	pthread_create(&thread, NULL, run_deadline, NULL);

	sleep(10);

	done = 1;
	pthread_join(thread, NULL);

	printf("main dies [%ld]\n", gettid());
	return 0;
   }

```
