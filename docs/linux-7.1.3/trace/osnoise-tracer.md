## OSNOISE 跟踪器


在高性能计算（HPC）的语境中，操作系统噪声（**osnoise**）指的是应用程序由于操作系统内部活动而经历的干扰。在 Linux 的语境下，NMI、IRQ、SoftIRQ 以及任何其它系统线程都可能给系统带来噪声。此外，与硬件相关的工作也可能引起噪声，例如通过 SMI。

hwlat_detector 是用来识别最复杂噪声源——**硬件噪声**——的工具之一。

简而言之，hwlat_detector 创建一个线程，该线程以给定周期周期性地运行。在一个周期开始时，该线程禁用中断并开始采样。运行中，hwlatd 线程在一个循环中读取时间。由于中断被禁用，线程、IRQ 和 SoftIRQ 都无法干扰 hwlatd 线程。因此，两次不同时间读取之间出现间隔的原因，要么在 NMI 中，要么在硬件本身。在周期结束时，hwlatd 重新启用中断，并报告读取之间观测到的最大间隔。它还会打印一个 NMI 发生计数器。如果输出中没有报告 NMI 执行，用户就可以断定硬件是该延迟的罪魁祸首。hwlat 通过观察 NMI 的进入与退出检测 NMI 执行。

osnoise 跟踪器利用 hwlat_detector，运行一个类似的循环，但允许抢占、SoftIRQ 和 IRQ，从而允许在其执行期间出现所有来源的 **osnoise**。采用与 hwlat 相同的方法，osnoise 记录任何干扰源的进入与退出点，并递增一个 per-cpu 干扰计数器。osnoise 跟踪器还会为每一种干扰源保存一个干扰计数器。每当工具观察到 NMI、IRQ、SoftIRQ 和线程这些干扰的进入事件时，相应的干扰计数器就会递增。当发生噪声而没有来自操作系统层面的任何干扰时，硬件噪声计数器递增，指向一个与硬件相关的噪声。通过这种方式，osnoise 可以统计任何来源的干扰。在周期结束时，osnoise 跟踪器打印所有噪声之和、最大单次噪声、线程可用的 CPU 百分比，以及各噪声源的计数器。

### 用法


将 ASCII 文本 "osnoise" 写入 tracing 系统（通常挂载于 /sys/kernel/tracing）的 current_tracer 文件。

```

        [root@f32 ~]# cd /sys/kernel/tracing/
        [root@f32 tracing]# echo osnoise > current_tracer

```
```

        [root@f32 tracing]# cat trace
        # tracer: osnoise
        #
        #                                _-----=> irqs-off
        #                               / _----=> need-resched
        #                              | / _---=> hardirq/softirq
        #                              || / _--=> preempt-depth                            MAX
        #                              || /                                             SINGLE     Interference counters:
        #                              ||||               RUNTIME      NOISE   % OF CPU  NOISE    +-----------------------------+
        #           TASK-PID      CPU# ||||   TIMESTAMP    IN US       IN US  AVAILABLE  IN US     HW    NMI    IRQ   SIRQ THREAD
        #              | |         |   ||||      |           |             |    |            |      |      |      |      |      |
                   <...>-859     [000] ....    81.637220: 1000000        190  99.98100       9     18      0   1007     18      1
                   <...>-860     [001] ....    81.638154: 1000000        656  99.93440      74     23      0   1006     16      3
                   <...>-861     [002] ....    81.638193: 1000000       5675  99.43250     202      6      0   1013     25     21
                   <...>-862     [003] ....    81.638242: 1000000        125  99.98750      45      1      0   1011     23      0
                   <...>-863     [004] ....    81.638260: 1000000       1721  99.82790     168      7      0   1002     49     41
                   <...>-864     [005] ....    81.638286: 1000000        263  99.97370      57      6      0   1006     26      2
                   <...>-865     [006] ....    81.638302: 1000000        109  99.98910      21      3      0   1006     18      1
                   <...>-866     [007] ....    81.638326: 1000000       7816  99.21840     107      8      0   1016     39     19

```
除了常规的 trace 字段（从 TASK-PID 到 TIMESTAMP）外，跟踪器会在每个周期结束时，为正在运行 osnoise/ 线程的每个 CPU 打印一条消息。osnoise 特有的字段报告如下：

 - RUNTIME IN US（以微秒计的运行时）报告 osnoise 线程持续循环读取时间所花费的时间量。
 - NOISE IN US（以微秒计的噪声）报告 osnoise 跟踪器在相应运行时间内观测到的噪声总和。
 - % OF CPU AVAILABLE（可用 CPU 百分比）报告运行时间窗口内 osnoise 线程可用的 CPU 百分比。
 - MAX SINGLE NOISE IN US（最大单次噪声，以微秒计）报告运行时间窗口内观测到的最大单次噪声。
 - 干扰计数器显示各对应干扰在运行时间窗口内发生的次数。

注意，上面的示例显示了大量的 HW 噪声样本。其原因是该样本取自一台虚拟机，而主机的干扰被当作硬件干扰检测到了。

### 跟踪器配置


跟踪器在 osnoise 目录下有一组选项，它们是：

 - osnoise/cpus：将运行 osnoise 线程的 CPU。
 - osnoise/period_us：osnoise 线程的周期。
 - osnoise/runtime_us：osnoise 线程寻找噪声的时长。
 - osnoise/stop_tracing_us：如果发生的单次噪声高于配置值，停止系统 tracing。写入 0 会禁用该选项。
 - osnoise/stop_tracing_total_us：如果总噪声高于配置值，停止系统 tracing。写入 0 会禁用该选项。
 - tracing_threshold：两次 time() 读取之间被视为噪声的最小差值（以 us 计）。设为 0 时使用默认值，目前为 1 us。
 - osnoise/options：一组开/关选项，可通过将选项名写入该文件来启用，或通过写入带有 'NO\_' 前缀的选项名来禁用。例如，写入 NO_OSNOISE_WORKLOAD 会禁用 OSNOISE_WORKLOAD 选项。特殊的 DEFAULTS 选项会将所有选项重置为默认值。

### 跟踪器选项


osnoise/options 文件暴露了一组用于 osnoise 跟踪器的开/关配置选项。这些选项是：

 - DEFAULTS：将选项重置为默认值。
 - OSNOISE_WORKLOAD：不调度 osnoise 工作负载（见下文专门小节）。
 - PANIC_ON_STOP：如果跟踪器停止，调用 panic()。该选项用于捕获 vmcore。
 - OSNOISE_PREEMPT_DISABLE：运行 osnoise 工作负载时禁用抢占，只允许 IRQ 和与硬件相关的噪声。
 - OSNOISE_IRQ_DISABLE：运行 osnoise 工作负载时禁用 IRQ，只允许 NMI 和与硬件相关的噪声，类似 hwlat 跟踪器。

### 额外的 Tracing


除了跟踪器之外，还添加了一组 tracepoint，以方便识别 osnoise 的来源。

 - osnoise:sample_threshold：任何时候噪声高于可配置的 tolerance_ns 时打印。
 - osnoise:nmi_noise：来自 NMI 的噪声，包含持续时间。
 - osnoise:irq_noise：来自 IRQ 的噪声，包含持续时间。
 - osnoise:softirq_noise：来自 SoftIRQ 的噪声，包含持续时间。
 - osnoise:thread_noise：来自线程的噪声，包含持续时间。

注意，所有的值都是**净值**。例如，如果在 osnoise 运行期间，另一个线程抢占了 osnoise 线程，它会在开始时启动一个 thread_noise 持续时间。随后发生一次 IRQ，抢占该 thread_noise，启动一个 irq_noise。当 IRQ 执行结束时，它会计算自身的持续时间，并从 thread_noise 中减去该持续时间，以避免对 IRQ 执行的重复记账。这一逻辑对所有噪声源都成立。

```

       osnoise/8-961     [008] d.h.  5789.857532: irq_noise: local_timer:236 start 5789.857529929 duration 1845 ns
       osnoise/8-961     [008] dNh.  5789.858408: irq_noise: local_timer:236 start 5789.858404871 duration 2848 ns
     migration/8-54      [008] d...  5789.858413: thread_noise: migration/8:54 start 5789.858409300 duration 3068 ns
       osnoise/8-961     [008] ....  5789.858413: sample_threshold: start 5789.858404555 duration 8812 ns interferences 2

```
在此示例中，最后一行报告了一个 8 微秒的噪声样本，指向两次干扰。向后查看 trace，前两个条目是关于在一次定时器 IRQ 执行之后运行的迁移线程。第一个事件不属于该噪声的一部分，因为它发生在一毫秒之前。

值得注意的是，tracepoint 中报告的持续时间之和小于 sample_threshold 中报告的 8 us。其根源在于任何干扰执行前后进入与退出代码的开销。这正是采用双重方法——测量线程与 tracing——的理由所在。

### 在没有工作负载的情况下运行 osnoise 跟踪器


通过启用设置了 NO_OSNOISE_WORKLOAD 选项的 osnoise 跟踪器，osnoise: tracepoint 可用于测量任何类型 Linux 任务的执行时间，而不受其它任务干扰。
