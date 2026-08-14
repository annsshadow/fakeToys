## CFS 带宽控制

   本文档只讨论针对 SCHED_NORMAL 的 CPU 带宽控制。
   SCHED_RT 的情况在 Documentation/scheduler/sched-rt-group.rst 中介绍。

CFS 带宽控制是 CONFIG_FAIR_GROUP_SCHED 的一个扩展，它允许指定一个组或层级（hierarchy）所能使用的最大 CPU 带宽。

一个组所允许的带宽通过 quota（配额）和 period（周期）来指定。在每个给定的“period”（微秒）内，一个任务组会被分配最多“quota”微秒的 CPU 时间。该 quota 会在线程于 cgroup 中变为可运行（runnable）时，以片段（slice）的形式分配到各 CPU 的运行队列上。一旦 quota 全部分配完毕，任何额外的 quota 请求都将导致这些线程被节流（throttled）。被节流的线程在下一个 period 中 quota 得到补充之前将无法再次运行。

一个组未分配出去的 quota 会在全局范围内被跟踪，并在每个 period 边界处刷新回 cfs_quota 单位。随着线程消费这一带宽，它会按需被转移到 CPU 本地的“silos”中。每次更新中所转移的数量是可调的，被称为“slice”。

### 突发（Burst）特性

这一特性相当于用我们未来的 underrun（未用尽）来透支当前的时间，代价是增加对系统其他用户的干扰。一切都得到了良好的限制。

传统的（UP-EDF）带宽控制大致如下：

  (U = \Sum u_i) <= 1

这既保证了每个截止期限（deadline）都能被满足，也保证了系统的稳定性。毕竟，如果 U 大于 1，那么每过一秒真实时间（walltime），我们就得运行超过一秒的程序时间，显然会错过截止期限，而下一个截止期限只会更远，永远没有时间追回来，这是无界的失败。

突发特性注意到，一个工作负载并不总是执行完整的 quota；这使得人们可以将 u_i 描述为一个统计分布。

例如，令 u_i = {x,e}_i，其中 x 是 p(95)，而 x+e 是 p(100)（即传统的 WCET）。这实际上允许 u 更小，从而提高效率（我们可以在系统中装入更多任务），但代价是当所有概率都对准时错过截止期限。不过，它确实维持了稳定性，因为只要我们的 x 高于平均值，每一次 over（超出）都必须与一次 under（未用尽）相配对。

也就是说，假设我们有两个任务，都指定了 p(95) 值，那么我们就有 p(95)*p(95) = 90.25% 的概率两个任务都在其 quota 之内，一切正常。同时我们有 p(5)*p(5) = 0.25% 的概率两个任务会同时超出其 quota（必然导致截止期限失败）。在两者之间存在着一个阈值，使得其中一个超出而其他不足以用 underrun 来补偿；这取决于具体的 CDF。

同时，我们可以说最坏情况下的截止期限错过，将是 \Sum e_i；也就是说，存在着有界的迟到（tardiness）（在假设 x+e 确实是 WCET 的前提下）。

使用突发时的干扰程度，由错过截止期限的可能性以及平均 WCET 来度量。测试结果表明，当存在许多 cgroup 或 CPU 利用率不足时，干扰是有限的。更多细节见：
https://lore.kernel.org/lkml/5371BD36-55AE-4F71-B9D7-B86DC32E3D2B@linux.alibaba.com/

### 管理

quota、period 和 burst 通过 cgroupfs 在 cpu 子系统内进行管理。

   本节所描述的 cgroupfs 文件仅适用于 cgroup v1。对于 cgroup v2，参见
   Documentation/admin-guide/cgroup-v2.rst <cgroup-v2-cpu>。

- cpu.cfs_quota_us: 在单个 period 内补充的运行时间（以微秒为单位）
- cpu.cfs_period_us: 一个 period 的长度（以微秒为单位）
- cpu.stat: 导出节流统计信息 [详见下文]
- cpu.cfs_burst_us: 所能累积的最大运行时间（以微秒为单位）

```

	cpu.cfs_period_us=100ms
	cpu.cfs_quota_us=-1
	cpu.cfs_burst_us=0

```
cpu.cfs_quota_us 的值为 -1 表示组没有任何带宽限制，这样的组被称为无约束（unconstrained）带宽组。这代表了 CFS 传统的“工作保持”（work-conserving）行为。

写入任何不小于 cpu.cfs_burst_us 的（有效）正值将实施所指定的带宽限制。quota 或 period 所允许的最小值为 1ms。period 的长度也有 1s 的上限。在以层级方式使用带宽限制时还存在额外的限制，下文将更详细地说明。

向 cpu.cfs_quota_us 写入任何负值将移除带宽限制，并使该组重新回到无约束状态。

cpu.cfs_burst_us 的值为 0 表示组不能累积任何未使用的带宽。它使 CFS 传统的带宽控制行为保持不变。向 cpu.cfs_burst_us 写入任何不大于 cpu.cfs_quota_us 的（有效）正值，将实施对未使用带宽累积的上限。

对某个组的带宽规格的任何更新，如果它处于受约束状态，都将使其变为未节流状态。

### 系统级设置

为了提高效率，运行时间在全局池与 CPU 本地“silos”之间以批处理方式转移。这大大减少了大型系统上的全局记账（accounting）压力。每次需要此类更新时所转移的数量被称为“slice”。

```

	/proc/sys/kernel/sched_cfs_bandwidth_slice_us (default=5ms)

```
较大的 slice 值会降低转移开销，而较小的值则允许更细粒度的消费。

### 统计信息

一个组的带宽统计信息通过 cpu.stat 中的 5 个字段导出。

cpu.stat:

- nr_periods: 已经过的强制执行间隔（enforcement interval）数量。
- nr_throttled: 该组被节流/限制的次数。
- throttled_time: 该组的实体被节流的累计时长（以纳秒为单位）。
- nr_bursts: 发生突发的 period 数量。
- burst_time: 任意 CPU 在相应 period 中超出 quota 所使用的累计真实时间（以纳秒为单位）。

此接口是只读的。

### 层级相关考量

该接口强制要求单个实体的带宽始终是可达到的，即：max(c_i) <= C。然而，在聚合情况下，显式允许过度订阅（over-subscription），以在层级内实现工作保持语义：

  e.g. \Sum (c_i) may exceed C

[ 其中 C 是父级的带宽，c_i 是其子级 ]

一个组可能通过两种方式被节流：

	a. 它在某个 period 内完全消耗了自身的 quota
	b. 某个父级的 quota 在其 period 内被完全消耗

在上述 b) 的情况下，即便子级可能仍有剩余的运行时间，在父级的运行时间被刷新之前它也不会被允许运行。

### CFS 带宽配额注意事项

一旦某个 slice 被分配给一个 CPU，它就不会过期。不过，如果那个 CPU 上的所有线程都变为不可运行，则除 1ms 之外的全部 slice 都可能会被归还给全局池。这由编译期变量 min_cfs_rq_runtime 配置。这是一项性能微调，有助于减少对全局锁的额外争用。

CPU 本地 slice 不会过期这一事实，会导致一些应当被理解的有趣的边界情况。

对于受 CPU 约束（cpu limited）的 cgroup CPU 受限应用程序而言，这是一个相对无关紧要的问题，因为它们会自然地消费掉其全部 quota 以及每个 CPU 本地 slice 的全部内容。因此，预期 nr_periods 大致等于 nr_throttled，并且 cpuacct.usage 在每个 period 中的增长大致等于 cfs_quota_us。

对于高度多线程、非 CPU 绑定的应用程序，这一不过期的细节允许应用程序短暂地突破其 quota 限制，突破量等于该任务组所运行的每个 CPU 上未使用的 slice（通常每个 CPU 最多 1ms，或由 min_cfs_rq_runtime 定义）。这一轻微的突发仅适用于此前 period 中 quota 已被分配给某个 CPU、随后未被完全使用或归还的情况。这一突变量不会在核心之间转移。因此，该机制仍然严格地将任务组限制为 quota 的平均用量，只不过是在比单个 period 更长的时间窗口内。这还将突发能力限制为每个 CPU 最多 1ms。对于在高核心数机器上拥有较小 quota 限制的高度多线程应用程序，这提供了更好、更可预测的用户体验。它还消除了在这些应用程序使用少于 quota 数量的 CPU 的同时对其节流的倾向。换句话说，通过允许 slice 的未使用部分在 period 之间保持有效，我们减少了在那些不需要一整个 slice 的 CPU 时间的 CPU 本地 silos 上浪费地使 quota 过期的可能性。

还应考虑 CPU 绑定与非 CPU 绑定的交互式应用程序之间的相互影响，尤其是当单核使用率达到 100% 时。如果您给这些应用程序各分配半个 CPU 核心，而它们又被调度到同一个 CPU 上，那么理论上非 CPU 绑定的应用程序在某些 period 中会多用最多 1ms 的 quota，从而阻止 CPU 绑定的应用程序用足同等数量的 quota。在这些情况下，将由 CFS 算法（参见 sched-design-CFS.rst）来决定选择哪个应用程序运行，因为它们都将是可运行的且都还有剩余 quota。这一运行时间上的差异将在交互式应用程序空闲下来的后续 period 中得到弥补。

### 示例

```

	If period is 250ms and quota is also 250ms, the group will get
	1 CPU worth of runtime every 250ms.

	# echo 250000 > cpu.cfs_quota_us /* quota = 250ms */
	# echo 250000 > cpu.cfs_period_us /* period = 250ms */

```
2. 在多 CPU 机器上将某个组限制为相当于 2 个 CPU 的运行时间

   采用 500ms 的 period 和 1000ms 的 quota，该组可以获得相当于 2 个 CPU 的运行时间
```

	# echo 1000000 > cpu.cfs_quota_us /* quota = 1000ms */
	# echo 500000 > cpu.cfs_period_us /* period = 500ms */

	The larger period here allows for increased burst capacity.

```
3. 将某个组限制为 1 个 CPU 的 20%。

```

	# echo 10000 > cpu.cfs_quota_us /* quota = 10ms */
	# echo 50000 > cpu.cfs_period_us /* period = 50ms */

   By using a small period here we are ensuring a consistent latency
   response at the expense of burst capacity.

```
4. 将某个组限制为 1 个 CPU 的 40%，并允许在已累积的情况下额外累积最多 1 个 CPU 的 20%。

   采用 50ms 的 period，20ms 的 quota 相当于 1 个 CPU 的 40%。
```

	# echo 20000 > cpu.cfs_quota_us /* quota = 20ms */
	# echo 50000 > cpu.cfs_period_us /* period = 50ms */
	# echo 10000 > cpu.cfs_burst_us /* burst = 10ms */

   Larger buffer setting (no larger than quota) allows greater burst capacity.

```
