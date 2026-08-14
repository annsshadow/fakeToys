## 硬件延迟检测器


### 简介


跟踪器 hwlat_detector 是一个特殊用途的跟踪器，用于检测由某些底层硬件或固件的行为所引起的、与 Linux 本身无关的大型系统延迟。该代码最初开发用于检测 x86 系统上的 SMI（系统管理中断，System Management Interrupts），但此补丁集并没有任何 x86 特有的内容。它最初是为“RT”补丁编写的，因为实时（Real Time）内核对延迟高度敏感。

SMI 不由 Linux 内核服务，这意味着内核甚至不知道它们正在发生。SMI 由 BIOS 代码设置，并由 BIOS 代码服务，通常用于“关键”事件，例如对热传感器和风扇的管理。但有时，SMI 被用于其他任务，而这些任务可能在处理程序中花费过多的时间（有时以毫秒计）。显然，如果你试图将事件服务延迟保持在微秒级别，这就是一个问题。

硬件延迟检测器的工作原理是：在可配置的一段时间内独占（禁用中断）其中一个 CPU，在一段时间内轮询 CPU 时间戳计数器（Time Stamp Counter），然后查找 TSC 数据中的间隙。任何间隙都表明轮询被中断了，而由于中断已被禁用，唯一能做到这一点的就是 SMI 或其他硬件故障（或 NMI，但 NMI 可以被跟踪）。

请注意，hwlat 检测器**绝不**应在生产环境中使用。它旨在手动运行，以确定硬件平台是否存在长系统固件服务例程的问题。

### 用法


将 ASCII 文本 "hwlat" 写入跟踪系统的 current_tracer 文件（挂载在 /sys/kernel/tracing 或 /sys/kernel/tracing）。可以重新定义阈值（以微秒 us 为单位），高于该阈值的延迟峰值将被纳入考虑。

```

	# echo hwlat > /sys/kernel/tracing/current_tracer
	# echo 100 > /sys/kernel/tracing/tracing_thresh

```
/sys/kernel/tracing/hwlat_detector 接口包含以下文件：

  - width - 持有 CPU 时采样的时间周期（usecs）
            必须小于总的窗口大小（强制）
  - window - 采样的总周期，width 位于其中（usecs）

默认情况下，width 设置为 500,000，window 设置为 1,000,000，意味着每 1,000,000 微秒（1 秒）hwlat 检测器将自旋 500,000 微秒（0.5 秒）。如果在启用 hwlat 跟踪器时 tracing_thresh 为零，它将更改为默认的 10 微秒。如果观察到任何超过阈值的延迟，数据将被写入跟踪环形缓冲区。

周期之间的最小睡眠时间为 1 毫秒。即使 width 与 window 相距不到 1 毫秒，也会如此，以使系统不被完全饿死。

如果 hwlat 检测器启动时 tracing_thresh 为零，当加载另一个跟踪器时，它将被设回零。注意，hwlat 检测器在 tracing_thresh 中的最后一个值会被保存，如果该值在 hwlat 检测器再次启动时仍为零，则此值将被恢复到 tracing_thresh 中。

hwlat_detector 使用以下跟踪目录文件：

在 /sys/kernel/tracing 中：

 - tracing_threshold	- 被视为最小延迟的值（usecs）
 - tracing_max_latency	- 实际观测到的最大硬件延迟（usecs）
 - tracing_cpumask	- hwlat 线程要迁移经过的 CPU
 - hwlat_detector/width	- 在窗口内自旋的指定时间量（usecs）
 - hwlat_detector/window	- （width）运行之间的时间间隔（usecs）
 - hwlat_detector/mode	- 线程模式

默认情况下，一个 hwlat 检测器的内核线程会在新窗口开始时以轮询（round-robin）方式迁移经过 cpumask 中指定的每个 CPU。该行为可以通过更改线程模式来改变，可用选项有：

 - none:        不强制迁移
 - round-robin: 迁移经过 cpumask 中指定的每个 CPU [默认]
 - per-cpu:     为 tracing_cpumask 中的每个 cpu 创建一个线程
