
## 利用率钳制（Utilization Clamping）


## 1. 简介


利用率钳制（Utilization clamping），也称为 util clamp 或 uclamp，是一项调度器特性，允许用户空间协助管理任务的性能需求。它于 v5.3 版本引入，CGroup 支持在 v5.4 合入。

Uclamp 是一种提示（hinting）机制，让调度器理解任务的性能需求和限制，从而帮助调度器做出更好的决策。当使用 schedutil cpufreq 调速器时，util clamp 还会影响 CPU 频率的选择。

由于调度器和 schedutil 都由 PELT（util_avg）信号驱动，util clamp 通过对该信号进行钳制（clamping）来达到其目的，即将信号钳制到某个点；由此得名。也就是说，通过钳制利用率，我们让系统运行在某个特定的性能点上。

看待 util clamp 的正确方式是将其视为一种对性能约束发出请求或提示的机制。它由以下两个可调参数组成：

        - UCLAMP_MIN，用于设置下限。
        - UCLAMP_MAX，用于设置上限。

这两个边界将确保一个任务运行在系统的此性能范围之中。UCLAMP_MIN 意味着提升（boosting）一个任务，而 UCLAMP_MAX 意味着限制（capping）一个任务。

我们可以告诉系统（调度器），某些任务需要运行在某个最低性能点之上才能交付所期望的用户体验。或者，我们可以告诉系统，某些任务应当被限制、不得消耗过多资源、且不应超过某个特定性能点。从用户空间的角度看，将 uclamp 值视为性能点而非利用率，是更好的抽象。

例如，一个游戏可以利用 util clamp 与其感知到的每秒帧数（FPS）形成反馈回路。它可以动态地提高其显示流水线所需的最低性能点，以确保不丢帧。如果它知道在接下来的几百毫秒内即将出现计算密集的场景，它也可以动态地“预先”提升这些任务。

在设备能力差异很大的移动硬件上，这种动态反馈回路提供了极大的灵活性，能在任何系统的能力范围内确保最佳用户体验。

当然，静态配置也是可行的。具体的使用方式将取决于系统、应用以及期望的结果。

另一个例子是在 Android 中，任务被分类为后台（background）、前台（foreground）、顶层应用（top-app）等。Util clamp 可用于限制后台任务能消耗的资源量，方法是限制它们所能运行的性能点。这种约束有助于为重要任务（例如属于当前活动应用的任务，即 top-app 组）保留资源。此外，这也有助于限制它们消耗的功耗。在异构系统（例如 Arm big.LITTLE）中这一点更为明显；该约束将有助于让后台任务偏向停留在 little 核心上，从而确保：

        1. big 核心空闲，可以立即运行 top-app 任务。top-app 任务是用户当前正在交互的任务，因此是系统中最重要的任务。
        2. 它们不会运行在耗电的核心上并耗尽电池，即使它们是 CPU 密集型的任务。

  **little 核心**：
    capacity < 1024 的 CPU

  **big 核心**：
    capacity = 1024 的 CPU

通过发出这些 uclamp 性能请求（或更确切地说，提示），用户空间可以确保系统资源被最优地使用，以交付尽可能好的用户体验。

另一个用例是帮助**克服调度器利用率信号计算方式本身固有的上升延迟**。

另一方面，例如一个繁忙的任务需要运行在最高性能点，调度器要花约 200ms（PELT 半衰期 HALFLIFE = 32ms）才能意识到这一点。众所周知，这会影响移动设备上的游戏等工作负载，由于选择任务及时完成工作所需更高频率的响应时间过慢，会导致掉帧。将 UCLAMP_MIN=1024 将确保此类任务在开始运行时始终看到最高性能级别。

整体的可见效果不仅仅在于更好的感知用户体验/性能，若使用得当，还能延伸到帮助实现更好的整体性能/功耗（performance/watt）。

用户空间也可以与散热子系统形成反馈回路，确保设备不会热到触发降频（throttle）的程度。

SCHED_NORMAL/OTHER 和 SCHED_FIFO/RR 都遵循 uclamp 请求/提示。

在 SCHED_FIFO/RR 的情况下，uclamp 提供了让 RT 任务运行在任意性能点的选项，而不必始终被绑定到最高频率。这对于运行在电池供电设备上的通用系统很有用。

注意，按设计 RT 任务没有每任务的 PELT 信号，并且必须始终以恒定频率运行，以应对不确定的 DVFS 上升延迟。

注意，使用 schedutil 总是意味着在 RT 任务唤醒时需要一次修改频率的延迟。这个代价不会因使用 uclamp 而改变。Uclamp 只是帮助选择要请求的频率，而不是让 schedutil 始终为所有 RT 任务请求 MAX。

关于默认值，请参阅第 3.4 节 <uclamp-default-values>，以及关于如何更改 RT 任务默认值的 3.4.1 节 <sched-util-clamp-min-rt-default>。

## 2. 设计


Util clamp 是系统中每个任务的属性。它设定了其利用率信号的边界；作为一种偏置（bias）机制，影响调度器内部的某些决策。

任务的真实利用率信号在现实中从未被钳制。如果你在任何时刻检查 PELT 信号，应当继续看到它们完好无损。钳制只在需要时发生，例如当一个任务唤醒、且调度器需要为其选择一个合适的 CPU 来运行时。

由于 util clamp 的目标是允许请求任务运行所需的最小和最大性能点，它必须能够影响频率选择以及任务放置（task placement），才能最为有效。这两者都会对 CPU 运行队列（rq，简称）级别的利用率值产生影响，这也引出了主要的设计挑战。

当一个任务在某个 rq 上唤醒时，该 rq 的利用率信号将受到其上所有已入队任务的 uclamp 设置的影响。例如，如果一个任务请求以 UTIL_MIN = 512 运行，那么该 rq 的 util 信号需要尊重这个请求，以及来自所有已入队任务的所有其他请求。

为了能够聚合附加到 rq 的所有任务的 util clamp 值，uclamp 必须在每次入队/出队时做一些簿记工作，而这正是调度器的热路径（hot path）。因此必须小心，因为任何减速都会对大量用例产生显著影响，并可能在实践中妨碍其可用性。

处理方式是把利用率范围划分为若干个桶（struct uclamp_bucket），从而让我们能够将搜索空间从 rq 上的每个任务缩减到仅最顶层桶中的任务子集。

当一个任务入队时，对应桶中的计数器递增；出队时则递减。这使得跟踪 rq 级别的有效 uclamp 值变得容易得多。

随着任务的入队和出队，我们跟踪 rq 当前的有效 uclamp 值。关于其工作原理的细节，请参阅第 2.1 节 <uclamp-buckets>。

随后，在任何需要确定 rq 有效 uclamp 值的代码路径中，它只需要在做出决策的精确时刻读取该 rq 的有效 uclamp 值即可。

对于任务放置的情况，目前只有能量感知调度和容量感知调度（EAS/CAS）会利用 uclamp，这意味着它仅应用于异构系统。当一个任务唤醒时，调度器将查看每个 rq 当前的有效 uclamp 值，并将其与该任务若入队到此 rq 时潜在的新值进行比较。倾向于选择最终能效组合最佳的 rq。

类似地，在 schedutil 中，当它需要更新频率时，会查看 rq 当前的有效 uclamp 值（该值受当前入队于此的任务集合影响），并选择能满足这些请求约束的合适频率。

其他路径，例如设置过度利用（overutilization）状态（这实际上会禁用 EAS），也会利用 uclamp。此类情况被视为允许上述两个主要用例所必需的簿记工作，此处不会详述，因为它们可能随实现细节而改变。


### 2.1. 桶（Buckets）


```

                           [struct rq]

  (bottom)                                                    (top)

    0                                                          1024
    |                                                           |
    +-----------+-----------+-----------+----   ----+-----------+
    |  Bucket 0 |  Bucket 1 |  Bucket 2 |    ...    |  Bucket N |
    +-----------+-----------+-----------+----   ----+-----------+
       :           :                                   :
       +- p0       +- p3                               +- p4
       :                                               :
       +- p1                                           +- p5
       :
       +- p2


```
  上图只是示意，而非对内部数据结构的真实描绘。

为了在任务入队/出队时决定 rq 的有效 uclamp 值而减少搜索空间，整个利用率范围被划分为 N 个桶，其中 N 在编译时通过 CONFIG_UCLAMP_BUCKETS_COUNT 配置。默认值为 5。

rq 对每个 uclamp_id 可调参数都有对应的桶：[UCLAMP_MIN, UCLAMP_MAX]。

每个桶的范围为 1024/N。例如，对于默认值 5，将有 5 个桶，每个桶覆盖以下范围：

```

        DELTA = round_closest(1024/5) = 204.8 = 205

        Bucket 0: [0:204]
        Bucket 1: [205:409]
        Bucket 2: [410:614]
        Bucket 3: [615:819]
        Bucket 4: [820:1024]

```
当具有如下可调参数的任务 p

```

        p->uclamp[UCLAMP_MIN] = 300
        p->uclamp[UCLAMP_MAX] = 1024

```
被入队到 rq 时，UCLAMP_MIN 的桶 1 会递增，UCLAMP_MAX 的桶 4 会递增，以反映该 rq 在此范围内有一个任务这一事实。

rq 随后跟踪其每个 uclamp_id 当前的有效 uclamp 值。

当任务 p 入队时，rq 的值变为：

```

        // update bucket logic goes here
        rq->uclamp[UCLAMP_MIN] = max(rq->uclamp[UCLAMP_MIN], p->uclamp[UCLAMP_MIN])
        // repeat for UCLAMP_MAX

```
类似地，当 p 出队时，rq 的值变为：

```

        // update bucket logic goes here
        rq->uclamp[UCLAMP_MIN] = search_top_bucket_for_highest_value()
        // repeat for UCLAMP_MAX

```
当所有桶都为空时，rq 的 uclamp 值会重置为系统默认值。关于默认值的细节，请参阅第 3.4 节 <uclamp-default-values>。


### 2.2. 最大聚合（Max aggregation）


Util clamp 经过调优，以尊重需要最高性能点的任务的请求。

当多个任务附加到同一个 rq 时，util clamp 必须确保需要最高性能点的任务能够获得它，即使存在另一个不需要它、或被禁止达到该点的任务。

例如，如果有多个任务附加到某个 rq，其值如下：

```

        p0->uclamp[UCLAMP_MIN] = 300
        p0->uclamp[UCLAMP_MAX] = 900

        p1->uclamp[UCLAMP_MIN] = 500
        p1->uclamp[UCLAMP_MAX] = 500

```
那么假设 p0 和 p1 都被入队到同一个 rq，则 UCLAMP_MIN 和 UCLAMP_MAX 都变为：

```

        rq->uclamp[UCLAMP_MIN] = max(300, 500) = 500
        rq->uclamp[UCLAMP_MAX] = max(900, 500) = 900

```
正如我们将在第 5.1 节 <uclamp-capping-fail> 中看到的，这种最大聚合是使用 util clamp 时某些限制的根源之一，特别是当用户空间希望节省功耗时的 UCLAMP_MAX 提示。

### 2.3. 层级聚合（Hierarchical aggregation）


如前所述，util clamp 是系统中每个任务的属性。但实际应用的（有效的）值可能不仅受任务自身或其代理（中间件库）所发出请求的影响。

任何任务的有效 util clamp 值受到如下限制：

  1. 受其附加到的 cgroup CPU 控制器所定义的 uclamp 设置的限制（如果有）。
  2. （1）中被限制后的值，进一步受系统范围的 uclamp 设置的限制。

第 3 节 <uclamp-interfaces> 将讨论这些接口，并进一步展开。

目前只需说明，如果一个任务发出请求，其实际有效值必须遵循 cgroup 和系统范围设置施加的某些限制。

即使有效值最终会超出约束，系统仍会接受该请求，但一旦任务移动到不同的 cgroup，或系统管理员修改了系统设置，该请求只有在新的约束范围内才会被满足。

换句话说，这种聚合在任务更改其 uclamp 值时不会导致错误，而是系统可能因这些因素而无法满足请求。

### 2.4. 范围


Uclamp 性能请求的范围是 0 到 1024（含端点）。

对于 cgroup 接口，使用百分比（即 0 到 100，含端点）。就像其他 cgroup 接口一样，你可以用 'max' 代替 100。


## 3. 接口


### 3.1. 每任务接口


sched_setattr() 系统调用被扩展以接受两个新字段：

- sched_util_min：请求该任务运行时系统应运行的最低性能点。即性能下限。
- sched_util_max：请求该任务运行时系统应运行的最高性能点。即性能上限。

例如，以下场景具有 40% 到 80% 的利用率约束：

```

        attr->sched_util_min = 40% * 1024;
        attr->sched_util_max = 80% * 1024;

```
当任务 @p 运行时，**调度器应尽力确保它从 40% 性能级别开始**。如果该任务运行时间足够长，使其实际利用率超过 80%，则该利用率（或性能级别）将被限制在 80%。

特殊值 -1 用于将 uclamp 设置重置为系统默认值。

注意，使用 -1 将 uclamp 值重置为系统默认值，与手动将 uclamp 值设置为系统默认值是不同的。这一区别很重要，因为正如我们在系统接口中将要看到的，RT 的默认值可能被更改。SCHED_NORMAL/OTHER 未来也可能获得类似的旋钮。

### 3.2. cgroup 接口


CPU cgroup 控制器中有两个与 uclamp 相关的值：

- cpu.uclamp.min
- cpu.uclamp.max

当一个任务附加到某个 CPU 控制器时，其 uclamp 值将受到如下影响：

- cpu.uclamp.min 是一个保护值，如 :ref:`cgroup v2 文档的第 3-3 节 <cgroupv2-protections-distributor>` 所述。

  如果一个任务的 uclamp_min 值低于 cpu.uclamp.min，则该任务将继承 cgroup 的 cpu.uclamp.min 值。

  在 cgroup 层级中，有效的 cpu.uclamp.min 是（子，父）中的最大值。

- cpu.uclamp.max 是一个限制值，如 :ref:`cgroup v2 文档的第 3-2 节 <cgroupv2-limits-distributor>` 所述。

  如果一个任务的 uclamp_max 值高于 cpu.uclamp.max，则该任务将继承 cgroup 的 cpu.uclamp.max 值。

  在 cgroup 层级中，有效的 cpu.uclamp.max 是（子，父）中的最小值。

例如，给定以下参数：

```

        p0->uclamp[UCLAMP_MIN] = // system default;
        p0->uclamp[UCLAMP_MAX] = // system default;

        p1->uclamp[UCLAMP_MIN] = 40% * 1024;
        p1->uclamp[UCLAMP_MAX] = 50% * 1024;

        cgroup0->cpu.uclamp.min = 20% * 1024;
        cgroup0->cpu.uclamp.max = 60% * 1024;

        cgroup1->cpu.uclamp.min = 60% * 1024;
        cgroup1->cpu.uclamp.max = 100% * 1024;

```
当 p0 和 p1 附加到 cgroup0 时，这些值变为：

```

        p0->uclamp[UCLAMP_MIN] = cgroup0->cpu.uclamp.min = 20% * 1024;
        p0->uclamp[UCLAMP_MAX] = cgroup0->cpu.uclamp.max = 60% * 1024;

        p1->uclamp[UCLAMP_MIN] = 40% * 1024; // intact
        p1->uclamp[UCLAMP_MAX] = 50% * 1024; // intact

```
当 p0 和 p1 附加到 cgroup1 时，这些值则变为：

```

        p0->uclamp[UCLAMP_MIN] = cgroup1->cpu.uclamp.min = 60% * 1024;
        p0->uclamp[UCLAMP_MAX] = cgroup1->cpu.uclamp.max = 100% * 1024;

        p1->uclamp[UCLAMP_MIN] = cgroup1->cpu.uclamp.min = 60% * 1024;
        p1->uclamp[UCLAMP_MAX] = 50% * 1024; // intact

```
注意，cgroup 接口允许 cpu.uclamp.max 的值低于 cpu.uclamp.min。其他接口不允许这样。

### 3.3. 系统接口


### 3.3.1 sched_util_clamp_min


系统范围内允许的 UCLAMP_MIN 范围限制。默认设为 1024，这意味着任务允许的有效 UCLAMP_MIN 范围是 [0:1024]。例如将其改为 512，范围就缩小到 [0:512]。这对于限制任务所能获得的提升（boosting）程度很有用。

任务发出的、超过此旋钮值的请求仍会成功，但它们不会被满足，直到该值大于 p->uclamp[UCLAMP_MIN]。

该值必须小于或等于 sched_util_clamp_max。

### 3.3.2 sched_util_clamp_max


系统范围内允许的 UCLAMP_MAX 范围限制。默认设为 1024，这意味着任务允许的有效 UCLAMP_MAX 范围是 [0:1024]。

例如将其改为 512，有效允许范围就缩小到 [0:512]。这意味着没有任务能运行在 512 以上，这也意味着所有 rq 都受到限制。换言之，整个系统被限制在了一半的性能容量上。

这对于限制系统的整体最高性能点很有用。例如，在电池电量低时，或当系统处于空闲状态、屏幕关闭、希望限制访问更耗能的性能级别时，限制性能会很方便。

任务发出的、超过此旋钮值的请求仍会成功，但它们不会被满足，直到该值大于 p->uclamp[UCLAMP_MAX]。

该值必须大于或等于 sched_util_clamp_min。


### 3.4. 默认值


默认情况下，所有 SCHED_NORMAL/SCHED_OTHER 任务被初始化为：

```

        p_fair->uclamp[UCLAMP_MIN] = 0
        p_fair->uclamp[UCLAMP_MAX] = 1024

```
也就是说，默认情况下它们被提升以运行在系统的此外部性能点上（原本此处文本有误，意为最大性能点）。关于能否在启动时或运行时更改，目前尚无相关论证说明为何应提供此项，但未来可能会加入。

对于 SCHED_FIFO/SCHED_RR 任务：

```

        p_rt->uclamp[UCLAMP_MIN] = 1024
        p_rt->uclamp[UCLAMP_MAX] = 1024

```
也就是说，默认情况下它们被提升以运行在系统的最大性能点上，这保留了 RT 任务的历史行为。

RT 任务的默认 uclamp_min 值可以在启动时或运行时通过 sysctl 修改。请参阅下面的小节。


### 3.4.1 sched_util_clamp_min_rt_default


让 RT 任务运行在最大性能点上，对于电池供电的设备来说代价高昂且没有必要。为了让系统开发者在不把这些任务一路推到最大性能点的前提下，为这些任务提供良好的性能保证，这个 sysctl 旋钮允许调优最佳的提升值，以满足系统需求，而无需一直以最大性能点运行来消耗功耗。

鼓励应用开发者使用每任务的 util clamp 接口，以确保他们兼顾性能与功耗。理想情况下，系统设计师应将此旋钮设为 0，并将管理性能需求的工作留给应用。

## 4. 如何使用 util clamp


Util clamp 倡导用户空间辅助的功耗与性能管理的理念。在调度器层面，无需任何信息即可做出最佳决策。然而，借助 util clamp，用户空间可以向调度器提示，从而对任务放置和频率选择做出更好的决策。

最佳结果源于不对应用所运行的系统做任何假设，并将其与反馈回路结合使用，以动态监控和调整。最终这将带来更好的用户体验和更好的性能/功耗比。

对于某些系统和用例，静态设置将有助于取得良好结果。但在这种情况下可移植性会成为问题。在 100、200 或 1024 下能完成多少工作，对每个系统都不同。除非有特定的目标系统，否则应避免静态设置。

基于 util clamp 构建一整套框架，或开发直接利用它的自包含应用，存在足够多的可能性。

### 4.1. 提升重要且对 DVFS 延迟敏感的任务


一个 GUI 任务在唤醒时可能并不繁忙，不足以驱动频率升高。然而，它需要在特定的时间窗口内完成其工作，以交付期望的用户体验。它在唤醒时所需的正确频率将取决于系统。在一些性能不足的系统上它会较高，而在另一些性能过剩的系统上它会较低或为 0。

这个任务可以在每次错过截止期限时提高其 UCLAMP_MIN 值，以确保下次唤醒时它运行在更高的性能点。它应当尽量逼近允许其在任何特定系统上满足截止期限的最低 UCLAMP_MIN 值，从而为该系统的性能/功耗比达到最佳。

在异构系统上，让这个任务运行在更快的 CPU 上可能很重要。

** generally 建议将输入视为性能级别或性能点，这将同时暗示任务放置和频率选择**。（建议将输入理解为性能级别或性能点，它会同时影响任务放置与频率选择。）

### 4.2. 限制后台任务


如简介中针对 Android 情况所解释的。任何应用都可以为某些不关心性能、但可能最终变得繁忙并消耗不必要系统资源的后台任务降低 UCLAMP_MAX。

### 4.3. 省电模式


sched_util_clamp_max 系统范围接口可用于限制所有任务运行在通常能效较低的高性能点上。

这并非 uclamp 独有的能力，因为也可以通过降低 cpufreq 调速器的最大频率来实现同样的效果。它可以被视为一个更便利的替代接口。

### 4.4. 每应用性能限制


中间件/工具可以为用户提供选项，在每次执行某个应用时设置其 UCLAMP_MIN/MAX，以保证最低性能点，和/或限制其消耗系统功耗（代价是这些应用的性能下降）。

如果你想在外出编译内核时避免笔记本电脑过热，并且乐意牺牲性能来省电，但仍希望保持浏览器性能不受影响，uclamp 让这成为可能。

## 5. 限制


### 5.1. 在某些条件下用 uclamp_max 限制频率会失败


如果任务 p0 被限制为运行在 512：

```

        p0->uclamp[UCLAMP_MAX] = 512

```
并且它与 p1 共享 rq，而 p1 可以自由运行在任意性能点：

```

        p1->uclamp[UCLAMP_MAX] = 1024

```
那么由于最大聚合，rq 将被允许达到最大性能点：

```

        rq->uclamp[UCLAMP_MAX] = max(512, 1024) = 1024

```
假设 p0 和 p1 都有 UCLAMP_MIN = 0，那么 rq 的频率选择将取决于任务的实际利用率值。

如果 p1 是一个小任务，而 p0 是一个 CPU 密集型任务，那么由于两者运行在同一个 rq 上，尽管 p1 被允许运行在任意性能点、实际上并不需要运行在该频率，p1 仍会导致频率限制从 rq 上被解除。

### 5.2. UCLAMP_MAX 可能破坏 PELT（util_avg）信号


PELT 假设频率会随着信号增长而始终提升，以确保 CPU 上始终有一些空闲时间。但有了 UCLAMP_MAX，这种频率提升会被阻止，从而在某些情况下导致没有空闲时间。当没有空闲时间时，一个任务会卡在忙循环中，这将导致 util_avg 变为 1024。

结合下文描述的问题，当被严重限制的任务与小而未受限的任务共享 rq 时，这可能导致不期望的频率尖峰。

例如，如果任务 p 具有：

```

        p0->util_avg = 300
        p0->uclamp[UCLAMP_MAX] = 0

```
在一个空闲 CPU 上唤醒，那么它将运行在该 CPU 所能达到的最低频率（Fmin）。最大 CPU 频率（Fmax）在这里也很重要，因为它标定了在该 CPU 上完成任务工作所需的最短计算时间。

```

        rq->uclamp[UCLAMP_MAX] = 0

```
如果 Fmax/Fmin 的比值为 3，那么最大值为：

```

        300 * (Fmax/Fmin) = 900

```
这表示 CPU 仍会看到空闲时间，因为 900 < 1024。不过_实际的_ util_avg 不会是 900，而是介于 300 和 900 之间。只要有空闲时间，p->util_avg 的更新就会有一定的偏差，但不是与 Fmax/Fmin 成比例。

```

        p0->util_avg = 300 + small_error

```
现在如果 Fmax/Fmin 的比值为 4，最大值变为：

```

        300 * (Fmax/Fmin) = 1200

```
它高于 1024，表示 CPU 没有空闲时间。当这种情况发生时，_实际的_ util_avg 将变为：

```

        p0->util_avg = 1024

```
如果任务 p1 在这个 CPU 上唤醒，它具有：

```

        p1->util_avg = 200
        p1->uclamp[UCLAMP_MAX] = 1024

```
那么根据最大聚合规则，该 CPU 的有效 UCLAMP_MAX 将为 1024。但由于被限制的 p0 任务一直在运行并被严重节流，那么 rq->util_avg 将为：

```

        p0->util_avg = 1024
        p1->util_avg = 200

        rq->util_avg = 1024
        rq->uclamp[UCLAMP_MAX] = 1024

```
从而导致频率尖峰，因为如果不对 p0 进行节流，我们应当得到：

```

        p0->util_avg = 300
        p1->util_avg = 200

        rq->util_avg = 500

```
并运行在该 CPU 接近中段的性能点，而不是我们得到的 Fmax。

### 5.3. Schedutil 响应时间问题


schedutil 有三个限制：

        1. 硬件响应任何频率更改请求需要非零的时间。在某些平台上可达几毫秒的量级。
        2. 非快速切换（non fast-switch）系统需要一个 worker 截止期限线程唤醒并执行频率更改，这增加了可观的开销。
        3. schedutil 的 rate_limit_us 会丢弃在这个 rate_limit_us 时间窗口内的任何请求。

如果一个相对较小的任务正在执行关键工作，并在唤醒和开始运行时需要某个性能点，那么所有这些限制都会使其无法在期望的时间尺度内获得它想要的结果。

这个限制不仅在使用 uclamp 时产生影响，而且随着我们不再逐步上升或下降，它会变得更加普遍。我们很容易根据任务唤醒的顺序及其各自的 uclamp 值，在不同频率之间跳跃。

我们将其视为底层系统自身能力的限制。

schedutil 的 rate_limit_us 的行为有改进空间，但对于 1 和 2 则无能为力。它们被视为系统的硬性限制。
