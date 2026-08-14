## 软锁死检测器与硬锁死检测器（即 nmi_watchdog）


Linux 内核可以充当看门狗（watchdog），以检测软锁死和硬锁死。

“软锁死”（softlockup）被定义为一种导致内核在内核模式下循环超过 20 秒（详情见
下文“实现”）、而不给其它任务运行机会的缺陷。检测到时会显示当前的栈回溯，并且
默认情况下系统会保持锁定状态。或者，内核可以被配置为恐慌（panic）；为此提供了
一个 sysctl “kernel.softlockup_panic”、一个内核参数 “softlockup_panic”（详见
“Documentation/admin-guide/kernel-parameters.rst”）以及一个编译选项
“BOOTPARAM_SOFTLOCKUP_PANIC”。

“硬锁死”（hardlockup）被定义为一种导致 CPU 在内核模式下循环数秒（详见下文
“实现”）、而不让其它中断有运行机会的缺陷。与软锁死的情况类似，检测到时会显示
当前的栈回溯，并且系统会保持锁定状态，除非更改默认行为，这可通过一个 sysctl
“hardlockup_panic”、一个编译期开关 “BOOTPARAM_HARDLOCKUP_PANIC” 以及一个内核
参数 “nmi_watchdog” 来完成（详见
“Documentation/admin-guide/kernel-parameters.rst”）。

panic 选项可以与 panic_timeout 组合使用（这个超时通过命名令人困惑的
“kernel.panic” sysctl 设置），以使系统在指定的时间量后自动重启。

## 配置


提供了一个内核开关，允许管理员配置这个周期。“watchdog_thresh” 参数（默认 10
秒）控制阈值。对特定环境而言，合适的值是在对锁死的快速响应与检测开销之间的权衡。

## 实现


软锁死和硬锁死检测器都是围绕一个 hrtimer 构建的。此外，软锁死检测器会定期调度
一个任务，而硬锁死检测器可能在支持的架构上使用 Perf/NMI 事件。

### 频率与心跳


检测器的核心是一个 hrtimer。它服务于多个目的：

- 为软锁死检测器调度看门狗任务
- 为硬锁死检测器递增中断计数器（心跳）
- 检测软锁死
- 在 Buddy 模式下检测硬锁死

该 hrtimer 的周期为 2*watchdog_thresh/5，默认即 4 秒。在硬锁死检测器介入之前，
该 hrtimer 有两到三次机会产生中断（心跳）。

### 软锁死检测器


看门狗任务由 hrtimer 调度，并运行在一个停止（stop）调度线程中。它每次被调度时
都会更新一个时间戳。如果该时间戳在 2*watchdog_thresh 秒（软锁死阈值）内没有被
更新，那么“软锁死检测器”（编码在 hrtimer 回调函数中）会向系统日志转储有用的
调试信息，之后如果收到指示则调用 panic，否则恢复执行其它内核代码。

### 硬锁死检测器（NMI/Perf）


在支持 NMI（非屏蔽中断）perf 事件的架构上，会每 “watchdog_thresh” 秒产生一个
周期性 NMI。

如果系统中的任何 CPU 在 “watchdog_thresh” 窗口期间没有收到任何 hrtimer 中断
（心跳），那么“硬锁死检测器”（NMI perf 事件的处理程序）会生成一条内核警告或
调用 panic。

**检测开销（NMI）：**

检测到锁死所需的时间会因锁死相对于 NMI 检查窗口发生的时间而不同。以下示例假设
watchdog_thresh 为 10。

- **最佳情况：** 锁死发生在第一次心跳即将到期之前。检测器几乎会在下一次检查时
  立即注意到缺失的 hrtimer 中断。

```

    Time 100.0: cpu 1 heartbeat
    Time 100.1: hardlockup_check, cpu1 stores its state
    Time 103.9: Hard Lockup on cpu1
    Time 104.0: cpu 1 heartbeat never comes
    Time 110.1: hardlockup_check, cpu1 checks the state again, should be the same, declares lockup

    Time to detection: ~6 seconds

```
- **最坏情况：** 锁死发生在一次有效中断（心跳）之后不久，而该心跳本身发生在 NMI
  检查之后不久。下一次 NMI 检查发现中断计数已经改变（由于那一次心跳），便假设 CPU
  是健康的，并重置基线。锁死只在随后的检查中才被检测到。

```

    Time 100.0: hardlockup_check, cpu1 stores its state
    Time 100.1: cpu 1 heartbeat
    Time 100.2: Hard Lockup on cpu1
    Time 110.0: hardlockup_check, cpu1 stores its state (misses lockup as state changed)
    Time 120.0: hardlockup_check, cpu1 checks the state again, should be the same, declares lockup

    Time to detection: ~20 seconds

```
### 硬锁死检测器（Buddy）


在 NMI perf 事件不可用（或被禁用）的架构或配置上，内核可能使用 “buddy” 硬锁死
检测器。该机制需要 SMP（对称多处理）。

在此模式下，每个 CPU 被分配一个 “buddy” CPU 来监控。监控 CPU 运行它自己的 hrtimer
（与用于软锁死检测的相同），并检查 buddy CPU 的 hrtimer 中断计数是否增加。

为确保及时并避免误报，buddy 系统在每次 hrtimer 间隔（2*watchdog_thresh/5，默认
为 4 秒）进行检查。它使用 3 的缺失中断阈值。如果 buddy 的中断计数在连续 3 次检查
中都未改变，就假定该 buddy CPU 已硬锁死（中断被禁用）。监控 CPU 随后会触发硬锁死
响应（警告或 panic）。

**检测开销（Buddy）：**

在默认检查间隔为 4 秒（watchdog_thresh = 10）的情况下：

- **最佳情况：** 锁死发生在一次检查之前不久。
    在约 8 秒内检测到（距第 1 次检查 0 秒 + 距第 2 次 4 秒 + 距第 3 次 4 秒）。
- **最坏情况：** 锁死发生在一次检查之后不久。
    在约 12 秒内检测到（距第 1 次检查 4 秒 + 距第 2 次 4 秒 + 距第 3 次 4 秒）。

**Buddy 检测器的局限性：**

1. **所有 CPU 锁死：** 如果所有 CPU 同时锁死，buddy 检测器无法检测到该状况，因为
    监控 CPU 也被冻结了。
2. **栈回溯：** 与 NMI 检测器不同，buddy 检测器无法直接中断被锁定的 CPU 来获取
    栈回溯。它依赖架构特定的机制（如 NMI backtrace 支持）来尝试获取被锁定 CPU 的状态。
    如果缺少此类支持，日志可能只显示发生了锁死，而无法提供被锁定 CPU 的栈。

## 看门狗核心排除


默认情况下，看门狗在所有在线核心上运行。然而，在内核配置了 NO_HZ_FULL 的情况下，
默认看门狗只运行在 housekeeping（管家）核心上，而非 “nohz_full” 启动参数指定的
核心上。如果我们默认允许看门狗在 “nohz_full” 核心上运行，我们就必须运行定时器
滴答来激活调度器，这将阻止 “nohz_full” 功能保护这些核心上的用户代码免受内核影响。

当然，默认在 nohz_full 核心上禁用它意味着，当这些核心确实进入内核时，默认我们将
无法检测它们是否锁死。然而，允许看门狗继续在 housekeeping（非无滴答）核心上运行，
意味着我们将继续正确地检测这些核心上的锁死。

无论哪种情况，被排除运行看门狗的核心集合都可以通过 kernel.watchdog_cpumask
sysctl 调整。对于 nohz_full 核心，这在调试内核似乎在 nohz_full 核心上挂起的情况时
可能有用。
