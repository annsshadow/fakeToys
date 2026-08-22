
## 使用 RCU CPU 停顿检测器


本文档首先讨RCU CPU 停顿检测器能够定位哪些类型的问题，然后讨论可用于微调该检测器运行kernel 参数Kconfig 选项。最后，本文档解释停顿检测器splat"格式

## 什么会导致 RCU CPU 停顿警告

你的内核打印了一RCU CPU 停顿警告。下一个问题是"是什么导致的以下问题可能导致 RCU CPU 停顿警告
- 在某CPU 上的 RCU 读侧临界区中循环
- 在某个中断被禁用CPU 上循环
- 在某个抢占被禁用CPU 上循环
- 在某个底半部被禁用的 CPU 上循环
- 对于 !CONFIG_PREEMPTION 内核，某CPU 在内核中任何位置循环，而没有潜在地调用 schedule()。如果内核中的循环确实是预期且期望的行为，你可能需要添加一些对 cond_resched() 的调用
- 使用过慢而无法跟上启动时控制台消息速率console 连接来引Linux。例如，115Kbaud 串口控制台可*非常**慢，无法跟上启动时的消息速率，并会频繁产RCU CPU 停顿警告消息。特别是如果你还添加了调试用 printk()
- 任何阻止 RCU 宽限kthread 运行的情况。这会导All QSes seen"控制台日志消息。该消息将包kthread 上次运行的时间以及它应被期望运行的频率信息。它还可能导`rcu_.*kthread starved for` 控制台日志消息，其中将包含额外的调试信息
- CONFIG_PREEMPTION 内核中，一个绑定到 CPU 的实时任务，可能恰好抢占了处RCU 读侧临界区中间的低优先级任务。如果该低优先级任务不允许在任何其他 CPU 上运行，则下一RCU 宽限期将永远无法完成，最终会导致系统内存耗尽并挂起。在系统耗尽内存的过程中，你可能会看到停顿警告消息
- CONFIG_PREEMPT_RT 内核中，一个绑定到 CPU 的实时任务以高于 RCU softirq 线程的优先级运行。这将阻RCU 回调被调用，并且CONFIG_PREEMPT_RCU 内核中还会进一步阻RCU 宽限期完成。无论哪种情况，系统最终都会内存耗尽并挂起。在 CONFIG_PREEMPT_RCU 情况下，你可能会看到停顿警告消息
  你可以使rcutree.kthread_prio kernel 引导参数来提RCU kthread 的调度优先级，这有助于避免此问题。但请注意，这样做会增加系统的上下文切换频率，从而降低性能
- 一个周期性中断，其处理程序耗时超过连续两对中断之间的时间间隔。这会阻RCU kthread softirq 处理程序运行。请注意，某些高开销的调试选项（例function_graph tracer）会导致中断处理程序耗时明显长于正常情况，进而可能导RCU CPU 停顿警告
- 在快速系统上测试工作负载，将停顿警告超时调低到刚好避RCU CPU 停顿警告，然后以相同的停顿警告超时在慢速系统上运行相同的工作负载。请注意，散热节流和按需调节（on-demand governors）可能导致同一系统有时快有时慢
- 硬件或软件问题关闭了未处dyntick-idle 模式CPU 上的调度时钟中断。这个问题确实发生过，并且似乎最可能导致 CONFIG_NO_HZ_COMMON=n 内核出现 RCU CPU 停顿警告
- 硬件或软件问题阻止了基于时间的唤醒发生。这些问题范围很广，从配置错误或存在 bug 的定时器硬件，到中断或异常路径中bug（无论是硬件、固件还是软件），再Linux 定时器子系统中的 bug，以及调度器中的 bug，甚至包RCU 自身bug。它还可能导`rcu_.*timer wakeup didn't happen for` 控制台日志消息，其中包含额外的调试信息
- 定时器问题导致时间看起来向前跳跃，使RCU 认为 RCU CPU 停顿警告超时已超出，而实际上只过去了很少的时间。这可能是由于定时器硬件 bug、定时器驱动 bug，甚jiffies"全局变量的损坏。在测试新硬件时，这类定时器硬件和驱bug 并不少见
- 底层内核问题，一方面未能调用 rcu_eqs_enter(true)、rcu_eqs_exit(true)、ct_idle_enter()、ct_idle_exit()、ct_irq_enter() ct_irq_exit() 中的某一个变体，另一方面又过多调用了其中之一。历史上，最常见的问题是遗漏irq_enter() irq_exit()，而它们又分别调用 ct_irq_enter() ct_irq_exit()。使CONFIG_RCU_EQS_DEBUG=y 构建内核有助于追踪这类问题，它们有时出现在体系结构相关代码中
- RCU 实现中的 bug
- 硬件故障。这不太可能，但在大型数据中心中绝不少见。在几十年前一个令人难忘的案例中，一个运行中的系统中CPU 发生故障，变得无响应，但没有立即导致崩溃。这导致了一系列 RCU CPU 停顿警告，最终使人们意识到该 CPU 已发生故障
RCU、RCU-sched、RCU-tasks RCU-tasks-trace 实现都具CPU 停顿警告。请注意，SRCU ***具有 CPU 停顿警告。请注意，RCU 仅在存在正在进行的宽限期时才会检CPU 停顿。没有宽限期，就没有 CPU 停顿警告
要诊断停顿的原因，请检查栈回溯。出问题的函数通常位于栈的顶部附近。如果你有一系列来自单个长时间停顿的停顿警告，比较栈回溯通常有助于确定停顿发生的位置，该位置通常位于从一次回溯到下一次回溯中保持不变的那部分栈顶部最接近的函数中。如果你能可靠地触发停顿，ftrace 会很有帮助
RCU bug 通常可在 CONFIG_RCU_TRACE RCU 事件追踪的帮助下调试。有RCU 事件追踪的信息，请参include/trace/events/rcu.h

## 微调 RCU CPU 停顿检测器


rcuupdate.rcu_cpu_stall_suppress 模块参数会禁RCU CPU 停顿检测器，该检测器用于检测不当延RCU 宽限期的情况。此模块参数默认启用 CPU 停顿检测，但可通过引导时参数或运行时通过 sysfs 覆盖。停顿检测器不当延迟"的定义由一kernel 配置变量cpp 宏控制：

### CONFIG_RCU_CPU_STALL_TIMEOUT


	kernel 配置参数定义RCU 从宽限期开始到发出 RCU CPU 停顿警告所等待的时间段。该时间段通常21 秒
	此配置参数可在运行时通过 /sys/module/rcupdate/parameters/rcu_cpu_stall_timeout 修改，但仅在周期开始时检查该参数。因此，如果你正处于一40 秒停顿的10 秒，将此 sysfs 参数设置为（比如 会缩*下一*停顿的超时，或当前停顿的后续警告超时（假设停顿持续时间足够长）。它不会影响当前停顿下一次警告的计时
	停顿警告消息可通过 /sys/module/rcupdate/parameters/rcu_cpu_stall_suppress 完全启用或禁用
### CONFIG_RCU_EXP_CPU_STALL_TIMEOUT


	CONFIG_RCU_CPU_STALL_TIMEOUT 参数相同，但仅适用于加速宽限期。此参数定义 RCU 从加速宽限期开始到发出 RCU CPU 停顿警告所等待的时间段。在 Android 设备上，该时间段通常20 毫秒。零值会导致在转换为毫秒后使CONFIG_RCU_CPU_STALL_TIMEOUT 的值
	此配置参数可在运行时通过 /sys/module/rcupdate/parameters/rcu_exp_cpu_stall_timeout 修改，但仅在周期开始时检查该参数。如果你处于当前停顿周期中，将其设置为新值将改变 -next- 停顿的超时
	停顿警告消息可通过 /sys/module/rcupdate/parameters/rcu_cpu_stall_suppress 完全启用或禁用
### RCU_STALL_DELAY_DELTA


	尽管 lockdep 工具极其有用，但它确实会增加一些开销。因此，CONFIG_PROVE_RCU 下，RCU_STALL_DELAY_DELTA 宏会在发RCU CPU 停顿警告消息之前额外允许 5 秒。（这是一cpp 宏，而非 kernel 配置参数。）

### RCU_STALL_RAT_DELAY


	CPU 停顿检测器会尽量让出问题的 CPU 打印自己的警告，因为这通常会产生更高质量的栈回溯。但是，如果出问题的 CPU RCU_STALL_RAT_DELAY 指定jiffies 数量内未检测到自身的停顿，则其他某CPU 会发出警告。此延迟通常设置为两jiffies。（这是一cpp 宏，而非 kernel 配置参数。）

### rcupdate.rcu_task_stall_timeout


	此引sysfs 参数控制 RCU-tasks RCU-tasks-trace 的停顿警告间隔。零或更小的值会抑制 RCU-tasks 停顿警告。正值以秒为单位设置停顿警告间隔。RCU-tasks 停顿警告以如下行开始：

		INFO: rcu_tasks detected stalls on tasks:

	然后继续输出每个阻塞当前 RCU-tasks 宽限期的任务sched_show_task()
	RCU-tasks-trace 停顿警告类似地开始（并继续）
		INFO: rcu_tasks_trace detected stalls on tasks


## 解读 RCU CPU 停顿检测器"Splats"


对于RCU-tasks 类型RCU，当某个 CPU 检测到其他
```
	INFO: rcu_sched detected stalls on CPUs/tasks:
	2-...: (3 GPs behind) idle=06c/0/0 softirq=1453/1455 fqs=0
	16-...: (0 ticks this GP) idle=81c/0/0 softirq=764/764 fqs=0
	(detected by 32, t=2603 jiffies, g=7075, q=625)

```
此消息表CPU 32 检测到 CPU 2 16 都在造成停顿，并且该停顿正在影响 RCU-sched。此消息之后通常会跟随每CPU 的栈转储。请注意，PREEMPT_RCU 构建可能被任务以CPU 所停顿，任务将PID 表示，例P3421"。甚至可能出rcu_state 停顿CPU ***任务共同导致的情况，此时出问题的 CPU 和任务都会在列表中明确指出。在某些情况下，CPU 会检测到自身停顿，这将导致自检测停顿
CPU 2 (3 GPs behind)"表示CPU 在过去三个宽限期内未RCU 核心交互。相比之下，CPU 16 (0 ticks this GP)"表示CPU 在当前停顿的宽限期内未接收任何调度时钟中断
消息中的"idle="部分打印 dyntick-idle 状态。第一/"之前的十六进制数dynticks 计数器的16 位，如果 CPU 处于 dyntick-idle 模式则为偶数值，否则为奇数值。两/"之间的十六进制数是嵌套值，如果在空闲循环中（如上所示）则为较小的非负数，否则为非常大的正数。最后一/"之后的数字是 NMI 嵌套值，将为较小的非负数
消息中的"softirq="部分跟踪被停CPU 已执行的 RCU softirq 处理程序数量/"之前的数字是CPU 上次记录宽限期开始时（自启动以来）已执行的数量，可能是当前（停顿的）宽限期，也可能是某个更早的宽限期（例如，如果CPU 可能处于 dyntick-idle 模式很长一段时间）/"之后的数字是自启动到当前时刻已执行的数量。如果后一个数字在重复的停顿警告消息中保持不变，则 RCU softirq 处理程序可能已无法在CPU 上执行。如果停顿的 CPU 在中断被禁用的情况下自旋，或者在 -rt 内核中高优先级进程正在饿RCU softirq 处理程序，就可能出现这种情况
"fqs="显示自上次该 CPU 记录宽限期开始以来，宽限kthread 跨此 CPU 进行的强制静止状态（force-quiescent-state）空离线检测次数
"detected by"行指示哪CPU 检测到了停顿（本例中为 CPU 32）、自宽限期开始以来已过去多少 jiffies（本例中2603）、宽限期序列号（7075），以及跨所CPU 排队RCU 回调总数估计值（本例中为 625）
如果宽限期恰好在停顿警告开始打印时结束，则会出现一条虚假的停顿警告消息，其中包```
	INFO: Stall ended before state dump start

```
这很少见，但在实际中确实时有发生。根据停顿警告与宽限期初始化的交互方式，也有可能在这种情况下标记一个零 jiffy 停顿。请注意，若不借助 stop_machine() 之类的方式，就不可能完全消除这类误报，而这对于此类问题而言属于过度杀伤
如果所CPU 和任务都已通过静止状态，但宽限期仍然未能结束，则停顿警告 splat
```
	All QSes seen, last rcu_preempt kthread activity 23807 (4297905177-4297881370), jiffies_till_next_fqs=3, root ->qsmask 0x0

```
"23807"表示自宽限期 kthread 运行以来已过去超23000 jiffiesjiffies_till_next_fqs"指示kthread 应运行的频率，给出强制静止状态扫描之间的 jiffies 数，本例中为 3，远小于 23807。最后，打印rcu_node 结构->qsmask 字段，通常为零
如果相关的宽限期 kthread 在停顿警告之前无法运行，如上All QSes seen"行的情况```
	rcu_sched kthread starved for 23807 jiffies! g7075 f0x0 RCU_GP_WAIT_FQS(3) ->state=0x1 ->cpu=5
	Unless rcu_sched kthread gets sufficient CPU time, OOM is now expected behavior.

```
即使所CPU 和任务都已通过所需的静止状态，饿死宽限kthread CPU 时间当然也会导致 RCU CPU 停顿警告g"数字显示当前宽限期序列号f"位于发送给宽限kthread ->gp_flags 命令之前RCU_GP_WAIT_FQS"表示kthread 正在等待一个短暂的超时state"位于 task_struct ->state 字段的值之前，"cpu"表示该宽限期 kthread 最近在 CPU 5 上运行
如果相关的宽限期 kthread 未能FQS 等待中唤醒，```
	kthread timer wakeup didn't happen for 23804 jiffies! g7076 f0x0 RCU_GP_WAIT_FQS(5) ->state=0x402

```
"23804"表示kthread 的定时器在超23000 jiffies 之前就已到期。该行的其余部分含义kthread 饿死情况类似
```
	Possible timer handling issue on cpu=4 timer-softirq=11142

```
这里cpu"表示宽限kthread 最近在 CPU 4 上运行，它在该处排队fqs 定时器timer-softirq"之后的数字是 CPU 4 上当前的 `TIMER_SOFTIRQ` 计数。如果此值在连续RCU CPU 停顿警告中不变，则有进一步理由怀疑定时器问题
这些消息之后通常会跟随参与停顿的 CPU 和任务的栈转储。这些栈回溯可以帮助你定位停顿的原因，但要记住，检测停顿的 CPU 会有一个主要致力于检测停顿的中断帧

## 一次停顿产生的多条警告


如果一个停顿持续时间足够长，将会为它打印多条停顿警告消息。第二条及后续消息以更长的间隔打印，因此（例如）第一条和第二条消息之间的时间约为停顿开始到第一条消息之间间隔的三倍。比较同一停顿宽限期的不同消息的栈转储可能会有帮助

## 加速宽限期的停顿警

如果加速宽限期检测到停顿，它会放置一条消```
	INFO: rcu_sched detected expedited stalls on CPUs/tasks: { 7-... } 21119 jiffies s: 73 root: 0x2/.

```
这表CPU 7 未能响应重调IPI。CPU 编号后的三个句点."）表示该 CPU 在线（否则第一个句点会O"），CPU 在加速宽限期开始时在线（否则第二个句点会为"o"），并且CPU 自启动以来至少在线过一次（否则第三个句点会N"）jiffies"之前的数字表示加速宽限期已持21119 jiffiess:"之后的数字表示加速宽限期序列计数器为 73。最后一个值为奇数这一事实表明有一个加速宽限期正在进行中root:"之后的数字是一个位掩码，指示根 rcu_node 结构的哪些子节点对应于阻塞当前加速宽限期CPU 或任务。如果树有多个层级，则会为树中其rcu_node 结构的状态打印额外的十六进制数字
与正常宽限期一样，PREEMPT_RCU 构建可能被任务以CPU 所停顿，任务将PID 表示，例P3421"
完全有可能在同一运行期间几乎同时看到来自正常宽限期和加速宽限期的停顿警告
## RCU_CPU_STALL_CPUTIME


在使CONFIG_RCU_CPU_STALL_CPUTIME=y 构建或引导时设置 rcupdate.rcu_cpu_stall_cputime=1 的内核中，会显示以下附加信息
```
  rcu:          hardirqs   softirqs   csw/system
  rcu:  number:      624         45            0
  rcu: cputime:       69          1         2425   ==> 2500(ms)

```
这些统计信息在采样期间收集。行"number:"中的值是被停CPU 上的硬中断数、软中断数和上下文切换数。行"cputime:"中的前三个值表示被停顿 CPU 上由硬中断、软中断和任务消耗的 CPU 时间（毫秒）。最后一个数字是测量间隔，同样以毫秒为单位。因为用户模式任务通常不会导致 RCU CPU 停顿，这些任务通常是内核任务，因此只考虑了系CPU 时间
```
  |<------------first timeout---------->|<-----second timeout----->|
  |<--half timeout-->|<--half timeout-->|                          |
  |                  |<--first period-->|                          |
  |                  |<-----------second sampling period---------->|
  |                  |                  |                          |
             snapshot time point    1st-stall                  2nd-stall

```
以下描述四种典型场景
1. 某个 CPU 在中断被禁用的情况下循环
```
     rcu:          hardirqs   softirqs   csw/system
     rcu:  number:        0          0            0
     rcu: cputime:        0          0            0   ==> 2500(ms)

   因为中断在整个测量间隔期间都被禁用，所以没有中断也没有上下文切换。此外，由于 CPU 时间消耗是通过中断处理程序测量的，系统 CPU 消耗会被误导性地测量为零。此场景通常还会在该 CPU 的摘要行上打(0 ticks this GP)"
```
2. 某个 CPU 在底半部被禁用的情况下循环
   这与前一个例子类似，但硬中断的数量和消耗的 CPU 时间非零，同时软中断CPU 时间也呈非零特征
```
     rcu:          hardirqs   softirqs   csw/system
     rcu:  number:      624          0            0
     rcu: cputime:       49          0         2446   ==> 2500(ms)

   软中断为零这一事实提示它们被禁用了，可能是通过 local_bh_disable()。当然也有可能根本没有软中断，也许是因为所有会导致软中断执行的事件都局限于其他 CPU。在这种情况下，诊断应继续如下一个例子所示
```
3. 某个 CPU 在抢占被禁用的情况下循环
```
     rcu:          hardirqs   softirqs   csw/system
     rcu:  number:      624         45            0
     rcu: cputime:       69          1         2425   ==> 2500(ms)

   这种情况提示被停顿的 CPU 在抢占被禁用的情况下循环
```
4. 无循环，但大量硬中断和软中断
```
     rcu:          hardirqs   softirqs   csw/system
     rcu:  number:       xx         xx            0
     rcu: cputime:       xx         xx            0   ==> 2500(ms)

   这里，硬中断的数量和 CPU 时间都非零，但上下文切换的次数和消耗的内核 CPU 时间为零。软中断的数量和 cputime 通常非零，但也可能为零，例如，如CPU 在单个硬中断处理程序内自旋
   如果可以复现此类 RCU CPU 停顿警告，你可以通过查看 /proc/interrupts 或编写代码追踪每个中断（例如参show_interrupts()）来缩小范围
```
