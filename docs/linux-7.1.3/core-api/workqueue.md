## 工作队列


:Date: September, 2010
:Author: Tejun Heo <tj@kernel.org>
:Author: Florian Mickler <florian@mickler.org>


## 简介


在许多情况下都需要一个异步的进程执行上下文，而工作队列（wq）API 是此类情况下最常用的机制。

当需要这样的异步执行上下文时，描述要执行哪个函数的工作项（work item）会被放入一个队列。一个独立的线程充当异步执行上下文。该队列称为工作队列，该线程称为工作者（worker）。

只要工作队列上还有工作项，工作者就会依次执行这些工作项所对应的函数。当工作队列上没有剩余工作项时，工作者变为空闲。当一个新的工作项被入队时，工作者再次开始执行。


## 为什么需要并发管理工作队列（Concurrency Managed Workqueue）？


在最初的 wq 实现中，多线程（MT）wq 每个 CPU 有一个工作者线程，而单线程（ST）wq 整个系统只有一个工作者线程。一个 MT wq 需要维持与 CPU 数量相同的工作者数量。多年来内核增加了大量 MT wq 用户，随着 CPU 核心数量持续增长，某些系统仅在启动时就会耗尽默认的 32k PID 空间。

虽然 MT wq 浪费了大量资源，但所提供的并发程度却不尽如人意。这个限制对 ST 和 MT wq 都是共通的，尽管在 MT 上没那么严重。每个 wq 维护自己独立的 worker 池。一个 MT wq 每个 CPU 只能提供一个执行上下文，而 ST wq 只能为整个系统提供一个。工作项不得不争抢这些非常有限的执行上下文，导致各种问题，包括单一执行上下文周围容易死锁。

所提供的并发程度与资源使用之间的张力也迫使其用户做出不必要的权衡，例如 libata 选择使用 ST wq 来轮询 PIO，并接受了“两个轮询 PIO 不能同时推进”这一不必要的限制。由于 MT wq 并不能提供好得多的并发性，需要更高并发级别的用户（如 async 或 fscache）不得不实现自己的线程池。

并发管理工作队列（cmwq）是对 wq 的重新实现，重点关注以下目标。

- 保持与原始工作队列 API 的兼容性。

- 使用由所有 wq 共享的每 CPU 统一 worker 池，按需提供灵活的并发级别，而不浪费大量资源。

- 自动调节 worker 池和并发级别，使 API 用户无需操心这些细节。


## 设计


为了简化函数的异步执行，引入了一个新的抽象：工作项（work item）。

工作项是一个简单的结构体，其中保存了指向要异步执行的函数的指针。每当某个驱动或子系统希望某个函数被异步执行时，它必须建立一个指向该函数的工作项，并将该工作项排入工作队列。

工作项可以在线程或 BH（softirq）上下文中执行。

对于线程化工作队列，称为 [k]workers 的专用线程依次从队列中取函数执行。如果没有工作入队，工作者线程变为空闲。这些工作者线程在 worker 池中管理。

cmwq 设计区分了面向用户的工作队列（子系统和驱动将工作项排入其中）与后端机制（管理 worker 池并处理排队的工作项）。

对于每个可能的 CPU，有两个 worker 池，一个用于普通工作项，另一个用于高优先级工作项，另外还有一些额外的 worker 池用于服务排入未绑定（unbound）工作队列的工作项 —— 这些后端池的数量是动态的。

BH 工作队列使用相同的框架。然而，由于只能有一个并发执行上下文，因此无需担心并发问题。每个每 CPU 的 BH worker 池只包含一个代表 BH 执行上下文的伪工作者（pseudo worker）。BH 工作队列可以看作 softirq 的便捷接口。

子系统和驱动可以根据需要，通过特殊的工作队列 API 函数创建并排队工作项。它们可以通过在所排入的工作队列上设置标志，来影响工作项执行方式的一些方面。这些标志包括 CPU 局部性、并发限制、优先级等。详细概览请参阅下面 `alloc_workqueue()` 的 API 描述。

当工作项被排入工作队列时，目标 worker 池会根据队列参数和工作队列属性来确定，并追加到该 worker 池的共享工作列表（worklist）末尾。例如，除非特别覆盖，否则绑定工作队列的工作项会被排入与发起者所在 CPU 相关联的普通或高优先级 worker 池的工作列表中。

对于任何线程池实现，管理并发级别（有多少个执行上下文处于活动状态）都是一个重要问题。cmwq 试图将并发保持在最小但充足的水平。最小是为了节省资源，充足是指系统被充分利用。

每个绑定到实际 CPU 的 worker 池通过挂接到调度器来实现并发管理。每当一个活动工作者唤醒或休眠时，worker 池都会收到通知，并跟踪当前可运行工作者的数量。一般来说，工作项不应霸占 CPU 并消耗大量周期。这意味着只要维持刚好足够的并发以防止工作处理停滞就是最优的。只要 CPU 上还有一个或多个可运行的工作者，worker 池就不会启动新工作的执行；但是，当最后一个正在运行的工作者进入休眠时，它会立即调度一个新的工作者，以免在还有待处理工作项时 CPU 空闲。这使得可以用最少的工作者数量而不损失执行带宽。

保留空闲工作者除了 kthread 占用的内存空间外没有别的代价，因此 cmwq 在销毁它们之前会保留它们一段时间。

对于未绑定的工作队列，后端池的数量是动态的。未绑定工作队列可以使用 `apply_workqueue_attrs()` 分配自定义属性，工作队列会自动创建匹配这些属性的后端 worker 池。调节并发级别的责任在于用户。还有一个标志可以将绑定 wq 标记为忽略并发管理。详情请参阅 API 小节。

前进进度保证（forward progress）依赖于在需要更多执行上下文时能够创建工作者，而这又通过使用应急工作者（rescue workers）来保证。所有可能用于内存回收代码路径上的工作项都必须排入为在内存压力下执行而保留了应急工作者的 wq 上。否则 worker 池可能会死锁，等待执行上下文释放。


## 应用程序编程接口（API）


`alloc_workqueue()` 分配一个 wq。原始的 `create_*workqueue()` 函数已被废弃，并计划移除。`alloc_workqueue()` 接受三个参数 —— `@name`、`@flags` 和 `@max_active`。`@name` 是 wq 的名称，如果有应急线程的话也用作其名称。

wq 不再管理执行资源，而是作为前进进度保证、flush 和工作项属性的一个域（domain）。`@flags` 和 `@max_active` 控制工作项如何被分配执行资源、如何被调度和执行。


### ``flags``


`WQ_BH`
  BH 工作队列可以看作 softirq 的便捷接口。BH 工作队列始终是每 CPU 的，所有 BH 工作项都按照入队顺序在入队 CPU 的 softirq 上下文中执行。

  所有 BH 工作队列的 `max_active` 必须为 0，且 `WQ_HIGHPRI` 是唯一允许的附加标志。

  BH 工作项不能睡眠。所有其他特性（如延迟入队、flush 和取消）都受支持。

`WQ_PERCPU`
  排入每 CPU wq 的工作项被绑定到特定 CPU。当 CPU 局部性很重要时，此标志是正确的选择。

  此标志是 `WQ_UNBOUND` 的补集。

`WQ_UNBOUND`
  排入未绑定 wq 的工作项由特殊的 worker 池提供服务，这些 worker 池中的工作者不绑定到任何特定 CPU。这使得 wq 表现为一个简单的执行上下文提供者，没有并发管理。未绑定 worker 池会尽快开始执行工作项。未绑定 wq 牺牲了局部性，但在以下情况下有用。

  - 预期并发级别需求大幅波动，且使用绑定 wq 可能因发起者在不同 CPU 间跳转而在不同 CPU 上创建大量大多闲置的工作者。

  - 长时间运行的 CPU 密集型工作负载，可以由系统调度器更好地管理。

`WQ_FREEZABLE`
  可冻结的 wq 参与系统挂起操作的冻结阶段。wq 上的工作项会被排空，在解冻之前不会启动新的工作项执行。

`WQ_MEM_RECLAIM`
  所有可能用于内存回收路径的 wq **必须**设置此标志。无论内存压力如何，该 wq 都保证至少拥有一个执行上下文。

`WQ_HIGHPRI`
  高优先级 wq 的工作项会被排入目标 CPU 的高优先级 worker 池。高优先级 worker 池由具有提升的 nice 级别的工作者线程提供服务。

  注意普通和高优先级 worker 池彼此不交互。各自维护独立的工作者池，并在其工作者之间实现并发管理。

`WQ_CPU_INTENSIVE`
  CPU 密集型 wq 的工作项不计入并发级别。换句话说，可运行的 CPU 密集型工作项不会阻止同一 worker 池中其他工作项的执行启动。这对于预期会霸占 CPU 周期、从而需要由系统调度器调节其执行的绑定工作项很有用。

  虽然 CPU 密集型工作项不计入并发级别，但其执行的启动仍受并发管理调节，可运行的非 CPU 密集型工作项会延迟 CPU 密集型工作项的执行。

  此标志对未绑定 wq 没有意义。


### ``max_active``


`@max_active` 决定了每个 CPU 可以分配给 wq 工作项的最大执行上下文数量。例如，当 `@max_active` 为 16 时，该 wq 在每个 CPU 上最多可以有 16 个工作项同时执行。这始终是每 CPU 属性，即使是未绑定工作队列也是如此。

`@max_active` 的最大限制是 2048，指定 0 时使用的默认值是 1024。这些值选得足够高，使得它们不会成为限制因素，同时在失控情况下提供保护。

wq 的活动工作项数量通常由 wq 的用户调节，更具体地说，由用户可能同时排队多少工作项来调节。除非有特定需要来限制活动工作项的数量，否则建议指定 '0'。

一些用户依赖严格的执行顺序，即任何时刻只有一个工作项在飞行中（in flight），且工作项按入队顺序处理。虽然过去曾使用 `@max_active` 为 1 加上 `WQ_UNBOUND` 来实现这种行为，但现在已不是这样。请改用 alloc_ordered_workqueue()。


## 示例执行场景


以下示例执行场景试图说明 cmwq 在不同配置下的行为。

 工作项 w0、w1、w2 被排入同一 CPU 上的绑定 wq q0。
 w0 占用 CPU 5ms，然后睡眠 10ms，然后再次占用 CPU 5ms 后完成。w1 和 w2 占用 CPU 5ms 然后睡眠 10ms。

忽略所有其他任务、工作和处理开销，并假设简单的 FIFO 调度，以下是高度简化的一个版本
```

 TIME IN MSECS	EVENT
 0		w0 starts and burns CPU
 5		w0 sleeps
 15		w0 wakes up and burns CPU
 20		w0 finishes
 20		w1 starts and burns CPU
 25		w1 sleeps
 35		w1 wakes up and finishes
 35		w2 starts and burns CPU
 40		w2 sleeps
 50		w2 wakes up and finishes

```
```

 TIME IN MSECS	EVENT
 0		w0 starts and burns CPU
 5		w0 sleeps
 5		w1 starts and burns CPU
 10		w1 sleeps
 10		w2 starts and burns CPU
 15		w2 sleeps
 15		w0 wakes up and burns CPU
 20		w0 finishes
 20		w1 wakes up and finishes
 25		w2 wakes up and finishes

```
```

 TIME IN MSECS	EVENT
 0		w0 starts and burns CPU
 5		w0 sleeps
 5		w1 starts and burns CPU
 10		w1 sleeps
 15		w0 wakes up and burns CPU
 20		w0 finishes
 20		w1 wakes up and finishes
 20		w2 starts and burns CPU
 25		w2 sleeps
 35		w2 wakes up and finishes

```
现在，假设 w1 和 w2 被排入一个不同的 wq q1，它具有
```

 TIME IN MSECS	EVENT
 0		w0 starts and burns CPU
 5		w0 sleeps
 5		w1 and w2 start and burn CPU
 10		w1 sleeps
 15		w2 sleeps
 15		w0 wakes up and burns CPU
 20		w0 finishes
 20		w1 wakes up and finishes
 25		w2 wakes up and finishes


```
## 指南


- 如果 wq 可能处理在内存回收期间使用的工作项，不要忘记使用 `WQ_MEM_RECLAIM`。每个设置了 `WQ_MEM_RECLAIM` 的 wq 都为其保留了一个执行上下文。如果内存回收期间使用的多个工作项之间存在依赖关系，它们应分别排入各自带有 `WQ_MEM_RECLAIM` 的独立 wq。

- 除非需要严格排序，否则无需使用 ST wq。

- 除非有特定需要，否则建议对 @max_active 使用 0。在大多数用例中，并发级别通常远低于默认限制。

- wq 作为前进进度保证（`WQ_MEM_RECLAIM`、flush 和工作项属性）的一个域。不涉及内存回收、不需要作为一组工作项的一部分被 flush、并且不需要任何特殊属性的工作项，可以使用某个系统 wq。使用专用 wq 与系统 wq 在执行特性上没有区别。

  注意：如果某些东西可能生成超过 @max_active 的未完成工作项（请对你的生产者做压力测试），它可能会耗尽系统 wq 并可能导致死锁。它应使用自己的专用工作队列，而不是系统 wq。

- 除非预期工作项会消耗大量 CPU 周期，否则由于 wq 操作和工作项执行中局部性水平的提高，使用绑定 wq 通常是有益的。


## 亲和性范围（Affinity Scopes）


未绑定工作队列根据其亲和性范围（affinity scope）对 CPU 进行分组，以改善缓存局部性。例如，如果工作队列使用默认的 "cache_shard" 亲和性范围，它会将 CPU 分组为子 LLC 分片（sub-LLC shard）。排入该工作队列的工作项会被分配给与发起 CPU 处于同一分片内的某个 CPU 上的工作者。一旦启动，该工作者是否允许移动到该范围之外取决于该范围的 `affinity_strict` 设置。

工作队列目前支持以下亲和性范围。

`default`
  使用模块参数 `workqueue.default_affinity_scope` 中设置的范围，它总是被设为下面范围之一。

`cpu`
  CPU 不被分组。在某个 CPU 上发起的工作项由同一 CPU 上的工作者处理。这使得未绑定工作队列表现得像没有并发管理的每 CPU 工作队列。

`smt`
  CPU 根据 SMT 边界分组。这通常意味着每个物理 CPU 核心的逻辑线程被分组在一起。

`cache`
  CPU 根据缓存边界分组。具体使用哪个缓存边界由架构代码决定。很多情况下使用 L3。

`cache_shard`
  CPU 被分组为最多 `wq_cache_shard_size` 个核心的子 LLC 分片（默认 8，可通过 `workqueue.cache_shard_size` 启动参数调整）。分片总是按核心（SMT 组）边界切分。这是默认的亲和性范围。

`numa`
  CPU 根据 NUMA 边界分组。

`system`
  所有 CPU 被放入同一组。工作队列不努力在与发起 CPU 接近的 CPU 上处理工作项。

默认的亲和性范围可以通过模块参数 `workqueue.default_affinity_scope` 更改，特定工作队列的亲和性范围可以使用 `apply_workqueue_attrs()` 更改。

如果设置了 `WQ_SYSFS`，该工作队列将在其 `/sys/devices/virtual/workqueue/WQ_NAME/` 目录下拥有以下与亲和性范围相关的接口文件。

`affinity_scope`
  读取以查看当前的亲和性范围。写入以更改。

  当当前范围为 default 时，读取此文件还会在括号中显示当前生效的范围，例如 `default (cache)`。

`affinity_strict`
  默认为 0，表示亲和性范围不严格。当工作项开始执行时，工作队列会尽最大努力确保工作者位于其亲和性范围内，这称为 repatriation（遣返）。一旦启动，调度器可以自由地将工作者移动到系统中的任何位置。这使得既能从范围局部性中受益，又能在必要且可用时利用其他 CPU。

  如果设置为 1，则保证该范围内的所有工作者始终处于该范围内。当跨越亲和性范围有其他影响时（例如就功耗或工作负载隔离而言），这可能很有用。严格的 NUMA 范围也可用于匹配旧内核的工作队列行为。


## 亲和性范围与性能


如果未绑定工作队列的行为无需进一步调优就对绝大多数用例都是最优的，那将是理想的。不幸的是，在当前内核中，局部性与利用率之间存在显著的权衡，在工作队列被大量使用时需要进行显式配置。

更高的局部性带来更高的效率，即在消耗相同数量的 CPU 周期时完成更多工作。然而，如果发起者没有将工作项充分分散到各个亲和性范围，更高的局部性也可能导致整体系统利用率降低。下面使用 dm-crypt 的性能测试清楚地说明了这种权衡。

测试运行在一个具有 12 核/24 线程、分布于四个 L3 缓存（AMD Ryzen 9 3900x）的 CPU 上。为保持一致，关闭了 CPU 时钟加速。`/dev/dm-0` 是在 NVME SSD（Samsung 990 PRO）上创建的 dm-crypt 设备，使用 `cryptsetup` 以默认设置打开。


### 场景 1：足够的发起者且工作分散到整个机器


```

  $ fio --filename=/dev/dm-0 --direct=1 --rw=randrw --bs=32k --ioengine=libaio \
    --iodepth=64 --runtime=60 --numjobs=24 --time_based --group_reporting \
    --name=iops-test-job --verify=sha512

```
有 24 个发起者，每个并发发出 64 个 IO。`--verify=sha512` 使 `fio` 每次生成并读回内容，这使得发起者与 `kcryptd` 之间的执行局部性变得重要。以下是根据在 `kcryptd` 上针对不同亲和性范围设置测得的五轮运行的读取带宽和 CPU 利用率。带宽单位为 MiBps，CPU 利用率为百分比。

   :widths: 16 20 20
   :header-rows: 1

   - - 亲和性
     - 带宽 (MiBps)
     - CPU 利用率 (%)

   - - system
     - 1159.40 ±1.34
     - 99.31 ±0.02

   - - cache
     - 1166.40 ±0.89
     - 99.34 ±0.01

   - - cache (strict)
     - 1166.00 ±0.71
     - 99.35 ±0.01

有足够且分散到整个系统的发起者时，使用 "cache"（无论严格与否）都没有坏处。这三种配置都使整台机器饱和，但缓存亲和的配置凭借改善的局部性领先 0.6%。


### 场景 2：较少的发起者，但工作足以饱和


```

  $ fio --filename=/dev/dm-0 --direct=1 --rw=randrw --bs=32k \
    --ioengine=libaio --iodepth=64 --runtime=60 --numjobs=8 \
    --time_based --group_reporting --name=iops-test-job --verify=sha512

```
与上一场景唯一的区别是 `--numjobs=8`。发起者数量是其三分之一，但总工作量仍足以使系统饱和。

   :widths: 16 20 20
   :header-rows: 1

   - - 亲和性
     - 带宽 (MiBps)
     - CPU 利用率 (%)

   - - system
     - 1155.40 ±0.89
     - 97.41 ±0.05

   - - cache
     - 1154.40 ±1.14
     - 96.15 ±0.09

   - - cache (strict)
     - 1112.00 ±4.64
     - 93.26 ±0.35

这足以使系统饱和。无论是 "system" 还是 "cache" 都几乎使机器饱和但并未完全饱和。"cache" 使用的 CPU 更少，但更高的效率使其带宽与 "system" 相同。

八个发起者在四个 L3 缓存范围间移动，仍能让 "cache (strict)" 基本使机器饱和，但工作守恒（work conservation）的丧失现在开始造成 3.7% 的带宽损失。


### 场景 3：更少的发起者，工作不足以饱和


```

  $ fio --filename=/dev/dm-0 --direct=1 --rw=randrw --bs=32k \
    --ioengine=libaio --iodepth=64 --runtime=60 --numjobs=4 \
    --time_based --group_reporting --name=iops-test-job --verify=sha512

```
同样，唯一的区别是 `--numjobs=4`。发起者数量减少到四个后，现在没有足够的工作使整个系统饱和，带宽变得依赖于完成延迟。

   :widths: 16 20 20
   :header-rows: 1

   - - 亲和性
     - 带宽 (MiBps)
     - CPU 利用率 (%)

   - - system
     - 993.60 ±1.82
     - 75.49 ±0.06

   - - cache
     - 973.40 ±1.52
     - 74.90 ±0.07

   - - cache (strict)
     - 828.20 ±4.49
     - 66.84 ±0.29

现在，局部性与利用率之间的权衡更加明显。"cache" 相比 "system" 显示 2% 的带宽损失，而 "cache (strict)" 则高达 20%。


### 结论与建议


在上述实验中，"cache" 亲和性范围相对于 "system" 的效率优势虽然一致且明显，但很小。然而，其影响取决于各范围之间的距离，在拓扑更复杂的处理器上可能更显著。

虽然在某些情况下工作守恒的丧失会造成损害，但它比 "cache (strict)" 好得多，而且最大化工作队列利用率无论如何不太可能是常见情况。因此，"cache" 是未绑定池的默认亲和性范围。

- 由于没有一种选项对大多数情况都很好，可能消耗大量 CPU 的工作队列用法建议使用 `apply_workqueue_attrs()` 和/或启用 `WQ_SYSFS` 来配置工作队列。

- 具有严格 "cpu" 亲和性范围的未绑定工作队列，其行为与 `WQ_CPU_INTENSIVE` 每 CPU 工作队列相同。后者没有真正的优势，而未绑定工作队列提供了更大的灵活性。

- 亲和性范围在 Linux v6.5 中引入。要模拟之前的行为，请使用严格的 "numa" 亲和性范围。

- 非严格亲和性范围中工作守恒的丧失可能源自调度器。没有理论上的理由说明内核无法做正确的事并在大多数情况下保持工作守恒。因此，未来的调度器改进可能会使大多数此类可调参数变得不必要。


## 检查配置


使用 tools/workqueue/wq_dump.py 来检查未绑定的 CPU 亲和性
```

  $ tools/workqueue/wq_dump.py
  Affinity Scopes
  ===============
  wq_unbound_cpumask=0000000f

  CPU
    nr_pods  4
    pod_cpus [0]=00000001 [1]=00000002 [2]=00000004 [3]=00000008
    pod_node [0]=0 [1]=0 [2]=1 [3]=1
    cpu_pod  [0]=0 [1]=1 [2]=2 [3]=3

  SMT
    nr_pods  4
    pod_cpus [0]=00000001 [1]=00000002 [2]=00000004 [3]=00000008
    pod_node [0]=0 [1]=0 [2]=1 [3]=1
    cpu_pod  [0]=0 [1]=1 [2]=2 [3]=3

  CACHE (default)
    nr_pods  2
    pod_cpus [0]=00000003 [1]=0000000c
    pod_node [0]=0 [1]=1
    cpu_pod  [0]=0 [1]=0 [2]=1 [3]=1

  NUMA
    nr_pods  2
    pod_cpus [0]=00000003 [1]=0000000c
    pod_node [0]=0 [1]=1
    cpu_pod  [0]=0 [1]=0 [2]=1 [3]=1

  SYSTEM
    nr_pods  1
    pod_cpus [0]=0000000f
    pod_node [0]=-1
    cpu_pod  [0]=0 [1]=0 [2]=0 [3]=0

  Worker Pools
  ============
  pool[00] ref= 1 nice=  0 idle/workers=  4/  4 cpu=  0
  pool[01] ref= 1 nice=-20 idle/workers=  2/  2 cpu=  0
  pool[02] ref= 1 nice=  0 idle/workers=  4/  4 cpu=  1
  pool[03] ref= 1 nice=-20 idle/workers=  2/  2 cpu=  1
  pool[04] ref= 1 nice=  0 idle/workers=  4/  4 cpu=  2
  pool[05] ref= 1 nice=-20 idle/workers=  2/  2 cpu=  2
  pool[06] ref= 1 nice=  0 idle/workers=  3/  3 cpu=  3
  pool[07] ref= 1 nice=-20 idle/workers=  2/  2 cpu=  3
  pool[08] ref=42 nice=  0 idle/workers=  6/  6 cpus=0000000f
  pool[09] ref=28 nice=  0 idle/workers=  3/  3 cpus=00000003
  pool[10] ref=28 nice=  0 idle/workers= 17/ 17 cpus=0000000c
  pool[11] ref= 1 nice=-20 idle/workers=  1/  1 cpus=0000000f
  pool[12] ref= 2 nice=-20 idle/workers=  1/  1 cpus=00000003
  pool[13] ref= 2 nice=-20 idle/workers=  1/  1 cpus=0000000c

  Workqueue CPU -> pool
  =====================
  [    workqueue \ CPU              0  1  2  3 dfl]
  events                   percpu   0  2  4  6
  events_highpri           percpu   1  3  5  7
  events_long              percpu   0  2  4  6
  events_unbound           unbound  9  9 10 10  8
  events_freezable         percpu   0  2  4  6
  events_power_efficient   percpu   0  2  4  6
  events_freezable_pwr_ef  percpu   0  2  4  6
  rcu_gp                   percpu   0  2  4  6
  rcu_par_gp               percpu   0  2  4  6
  slub_flushwq             percpu   0  2  4  6
  netns                    ordered  8  8  8  8  8
  ...

```
更多信息请参阅该命令的帮助信息。


## 监控


```

  $ tools/workqueue/wq_monitor.py events
                              total  infl  CPUtime  CPUhog CMW/RPR  mayday rescued
  events                      18545     0      6.1       0       5       -       -
  events_highpri                  8     0      0.0       0       0       -       -
  events_long                     3     0      0.0       0       0       -       -
  events_unbound              38306     0      0.1       -       7       -       -
  events_freezable                0     0      0.0       0       0       -       -
  events_power_efficient      29598     0      0.2       0       0       -       -
  events_freezable_pwr_ef        10     0      0.0       0       0       -       -
  sock_diag_events                0     0      0.0       0       0       -       -

                              total  infl  CPUtime  CPUhog CMW/RPR  mayday rescued
  events                      18548     0      6.1       0       5       -       -
  events_highpri                  8     0      0.0       0       0       -       -
  events_long                     3     0      0.0       0       0       -       -
  events_unbound              38322     0      0.1       -       7       -       -
  events_freezable                0     0      0.0       0       0       -       -
  events_power_efficient      29603     0      0.2       0       0       -       -
  events_freezable_pwr_ef        10     0      0.0       0       0       -       -
  sock_diag_events                0     0      0.0       0       0       -       -

  ...

```
更多信息请参阅该命令的帮助信息。


## 调试


由于工作函数由通用工作者线程执行，需要一些技巧来揭示行为异常的工作队列用户。
```

  root      5671  0.0  0.0      0     0 ?        S    12:07   0:00 [kworker/0:1]
  root      5672  0.0  0.0      0     0 ?        S    12:07   0:00 [kworker/1:2]
  root      5673  0.0  0.0      0     0 ?        S    12:12   0:00 [kworker/0:0]
  root      5674  0.0  0.0      0     0 ?        S    12:13   0:00 [kworker/1:0]

```
如果 kworker 失控（占用过多 CPU），有两类可能的问题：

 1. 某物被快速连续地调度
 2. 消耗大量 CPU 周期的单个工作项
```

	$ echo workqueue:workqueue_queue_work > /sys/kernel/tracing/set_event
	$ cat /sys/kernel/tracing/trace_pipe > out.txt
	(wait a few secs)
	^C

```
如果某物在入队工作上忙循环，它将在输出中占主导地位，可以通过工作项函数确定罪魁祸首。

对于第二类问题，应该只需检查
```

	$ cat /proc/THE_OFFENDING_KWORKER/stack

```
工作项的函数应能在栈回溯中直接看到。


## 不可重入条件


如果工作项入队后满足以下条件，工作队列保证该工作项不可重入：

        1. 工作函数未被更改。
        2. 没有人将该工作项排入另一个工作队列。
        3. 该工作项未被重新初始化。

换句话说，如果上述条件成立，则保证在任何时刻整个系统范围内最多只有一个工作者在执行该工作项。

注意，在工作函数（self function）中将工作项重新入队（到同一队列）不会破坏这些条件，因此可以安全进行。否则，在工作函数内部破坏这些条件时需要谨慎。


## 内核内联文档参考
