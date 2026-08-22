## 减少per-cpu kthread 引起的操作系统抖动（Reducing OS jitter due to per-cpu kthreads
本文档列出了 Linux 内核中的 per-CPU kthread，并给出了控制其操作系统抖动（OS jitter）的
若干选项。注意，这里不列出非 per-CPU kthread。要减少来自per-CPU kthread 的操作系抖动，请将其绑定到一个专用于此类工作管家"（housekeeping）CPU 上
## 参考资料（References
- Documentation/core-api/irq/irq-affinity.rst：将中断绑定到一CPU
- Documentation/admin-guide/cgroup-v1：使cgroup 将任务绑定到一CPU
- man taskset：使taskset 命令将任务绑定到一	CPU
- man sched_setaffinity：使sched_setaffinity() 系统调用
	将任务绑定到一CPU
- /sys/devices/system/cpu/cpuN/online：控CPU N 的热插拔状态，
	写入 "0" 表示下线，写"1" 表示上线
- 为了定位 CPU N 上由内核产生的操作系统抖动：

		cd /sys/kernel/tracing
		echo 1 > max_graph_depth # 增大 "1" 以获得更多细		echo function_graph > current_tracer
		# 运行工作负载
		cat per_cpu/cpuN/trace

## kthread（kthreads
Name:
  ehca_comp/%u

Purpose:
  周期性地处理 Infiniband 相关的任务
要减少其操作系统抖动，可执行以下任意一项：

1. 不要使用 eHCA Infiniband 硬件，而是选择不需per-CPU kthread 的硬件。这样会从一开始就
	阻止这些 kthread 被创建。（这对大多数人有效，因为这种硬件虽然重要，但相对较旧，且产	相对较低。）
2. 在其CPU 上完成所eHCA-Infiniband 相关的工作，包括中断3. 重新改eHCA 驱动，使per-CPU kthread 仅配置在选定CPU 上

Name:
  irq/%d-%s

Purpose:
  处理线程化中断（threaded interrupt）
要减少其操作系统抖动，请执行以下操作
1. 使用中断亲和性（irq affinity）强irq 线程在其CPU 上执行
Name:
  kcmtpd_ctr_%d

Purpose:
  处理蓝牙（Bluetooth）相关的工作
要减少其操作系统抖动，可执行以下任意一项：

1. 不要使用蓝牙，这样这kthread 从一开始就不会被创建2. 使用中断亲和性强制蓝牙相关的中断发生在其CPU 上，并进一步在其它 CPU 上发起所	蓝牙活动
Name:
  ksoftirqd/%u

Purpose:
  在采用线程化或处于高负载时执softirq 处理函数
要减少其操作系统抖动，必须分别处理每softirq 向量，如下所示：

### TIMER_SOFTIRQ

请执行以下全部操作：

1. CPU 非空闲时，尽可能让其脱离内核态，例如，避免系统调用，并强制让内核线程与中	都在别处执行2. CONFIG_HOTPLUG_CPU=y 构建。启动完成后，强制将CPU 下线，再将其重新上线。这	强制周期性定时器迁移到别处。如果你关注多个 CPU，请在把第一个重新上线之前，先把它们全部
	强制下线。一旦你把这CPU 上线后，不要再让其它 CPU 下线，因为那样做可能会把定时	重新强制放回这些 CPU 中的某一个上
### NET_TX_SOFTIRQ 涓?NET_RX_SOFTIRQ

请执行以下全部操作：

1. 将网络中断强制迁移到其它 CPU 上2. 在其CPU 上发起任何网I/O3. 一旦你的应用程序启动，就要阻止可能运行在待去抖CPU 上的任务发起 CPU 热插拔操作	（在CPU 上强制下线然后再重新上线是可以的，前提是在你启动应用程序之前完成。）

### BLOCK_SOFTIRQ

请执行以下全部操作：

1. 将块设备中断强制迁移到其CPU 上2. 在其CPU 上发起任何块 I/O3. 一旦你的应用程序启动，就要阻止可能运行在待去抖CPU 上的任务发起 CPU 热插拔操作	（在CPU 上强制下线然后再重新上线是可以的，前提是在你启动应用程序之前完成。）

### IRQ_POLL_SOFTIRQ

请执行以下全部操作：

1. 将块设备中断强制迁移到其CPU 上2. 在其CPU 上发起任何块 I/O 以及I/O 轮询3. 一旦你的应用程序启动，就要阻止可能运行在待去抖CPU 上的任务发起 CPU 热插拔操作	（在CPU 上强制下线然后再重新上线是可以的，前提是在你启动应用程序之前完成。）

### TASKLET_SOFTIRQ

请执行以下一项或多项
1. 避免使用使用 tasklet 的驱动。（这类驱动中会包含tasklet_schedule() 之类的调用。）
2. 把你必须使用的所有驱动从 tasklet 改造为 workqueue3. 将使tasklet 的驱动的中断强制迁移到其CPU 上，并且这些驱动涉及I/O 也都	其它 CPU 上进行
### SCHED_SOFTIRQ

请执行以下全部操作：

1. 避免向待去抖动的 CPU 发送调度器 IPI，例如，确保CPU 上最多只存在一个可运行kthread	如果某个期望在去抖动 CPU 上运行的线程被唤醒，调度器会发送一个可能导致后SCHED_SOFTIRQ
	IPI2. 设置 CONFIG_NO_HZ_FULL=y，并使用 "nohz_full=" 启动参数将待去抖动的 CPU 标记	adaptive-ticks（自适应时钟）CPU。这会减少去抖动 CPU 收到的调度器时钟中断数量，从	降低其被选中执行运行SCHED_SOFTIRQ 上下文中的负载均衡工作的概率3. CPU 非空闲时，尽可能让其脱离内核态，例如，避免系统调用，并强制让内核线程与中	都在别处执行。这进一步减少了去抖CPU 收到的调度器时钟中断数量
### HRTIMER_SOFTIRQ

请执行以下全部操作：

1. CPU 非空闲时，尽可能让其脱离内核态。例如，避免系统调用，并强制让内核线程与中断
	都在别处执行2. CONFIG_HOTPLUG_CPU=y 构建。启动完成后，强制将CPU 下线，再将其重新上线。这	强制周期性定时器迁移到别处。如果你关注多个 CPU，请在把第一个重新上线之前，先把它们全部
	强制下线。一旦你把这CPU 上线后，不要再让其它 CPU 下线，因为那样做可能会把定时	重新强制放回这些 CPU 中的某一个上
### RCU_SOFTIRQ

请至少执行以下一项：

1. 卸载回调（offload callbacks），并让CPU 保持 dyntick-idle adaptive-ticks 状态，
	具体做法如下
	a.	设置 CONFIG_NO_HZ_FULL=y，并使用 "nohz_full=" 启动参数将待去抖动的 CPU 标记		adaptive-ticks CPU。将 rcuo kthread 绑定到能够容忍操作系统抖动的管家 CPU 上	b.	CPU 非空闲时，尽可能让其脱离内核态，例如，避免系统调用，并强制让内核线程		中断都在别处执行
2. 通过 dyntick-idle RCU 远程完成其处理，具体做法如下
	a.	CONFIG_NO_HZ=y 构建	b.	确保CPU 频繁进入空闲状态，让其CPU 能够检测到它已经经过了一RCU 静止
		（quiescent）状态。如果内核以 CONFIG_NO_HZ_FULL=y 构建，用户空间执行也能让其它
		CPU 检测到CPU 已经经过了静止状态	c.	CPU 非空闲时，尽可能让其脱离内核态，例如，避免系统调用，并强制让内核线程		中断都在别处执行
Name:
  kworker/%u:%d%s (cpu, id, priority)

Purpose:
  执行 workqueue 请求
要减少其操作系统抖动，可执行以下任意一项：

1. 以实时（real-time）优先级运行你的工作负载，这将允许抢kworker 守护进程2. 通过在某workqueue alloc_workqueue() 中传WQ_SYSFS，可以让workqueue 	sysfs 文件系统中可见。这样的 workqueue 可以使用
	`/sys/devices/virtual/workqueue/*/cpumask` sysfs 文件被限制在给定的一CPU 上	可以使用 "ls /sys/devices/virtual/workqueue" 显示 WQ_SYSFS workqueue 的集合。话虽如此，
	workqueue 的维护者想提醒大家不要随意地把 WQ_SYSFS 到处乱用。之所以要谨慎，是因为添加
	WQ_SYSFS 很容易，但由sysfs 是正式的用户/内核 API 的一部分，即使添加是个错误，也几	不可能再将其移除3. 执行以下任意所需的操作，以避免你的应用程序无法容忍的抖动
	a.	避免使用 oprofile，从而避免来wq_sync_buffer() 的操作系统抖动	b.	限制你的 CPU 频率，从而不需CPU 频率调节（governor），可能还需要借助特殊		散热片或其它散热技术。如果做得正确，并且你的 CPU 架构允许，你应该能够		CONFIG_CPU_FREQ=n 构建内核，以避免 CPU 频率调节器（包括 cs_dbs_timer() 		od_dbs_timer()）周期性地在每CPU 上运行
		WARNING：请查阅你的 CPU 规格，确保这在你的特定系统上是安全的	c.	v3.18 起，Christoph Lameter 的按需 vmstat 工作（on-demand vmstat workers		提交防止了在 CONFIG_SMP=y 系统上由 vmstat_update() 引起的操作系统抖动。在 v3.18
		之前，无法完全消除操作系统抖动，但你可以通过/proc/sys/vm/stat_interval 写入
		一个较大的值来降低其频率。默认值是 HZ，对应一秒的间隔。当然，较大的值会让你		虚拟内存统计信息更新得更慢。当然，你也可以以实时优先级运行你的工作负载，从而抢		vmstat_update()，但如果你的工作负载CPU 密集型的，这就不是个好主意。不过，
		Christoph Lameter 有一个（基于 Gilad Ben-Yossef 早先提交的）RFC 补丁，能够为某些
		工作负载减轻甚至消除 vmstat 开销，地址		https://lore.kernel.org/r/00000140e9dfd6bd-40db3d4f-c1be-434f-8132-7820f81bb586-000000@email.amazonses.com	d.	如果运行在高powerpc 服务器上，请CONFIG_PPC_RTAS_DAEMON=n 构建。这会阻		RTAS 守护进程每隔一秒左右在每个 CPU 上运行。（这需要编Kconfig 文件，并且会破坏
		该平台的 RAS 功能。）这能避免rtas_event_scan() 函数引起的抖动		WARNING：请查阅你的 CPU 规格，确保这在你的特定系统上是安全的	e.	如果运行PowerMAC 上，请以 CONFIG_PMAC_RACKMETER=n 构建内核以禁CPU 计表		（CPU-meter），从而避免来rackmeter_do_timer() 的操作系统抖动
Name:
  rcuc/%u

Purpose:
  在以 CONFIG_RCU_BOOST=y 构建的内核中执行 RCU 回调
要减少其操作系统抖动，请至少执行以下一项：

1. CONFIG_PREEMPT=n 构建内核。这会从一开始就阻止这些 kthread 被创建，同时也消除了	RCU 优先级提升（priority boosting）的需求。这种方法对于不需要高度响应性的工作负载	可行的2. CONFIG_RCU_BOOST=n 构建内核。这会从一开始就阻止这些 kthread 被创建。这种方法仅当你	工作负载永远不需RCU 优先级提升时才可行，例如，如果你能确保所有可能在内核中执行的 CPU
	都有频繁空闲时间3. CONFIG_RCU_NOCB_CPU=y 构建，并使用 rcu_nocbs= 启动参数，从所有容易发生操作系统抖动的
	CPU 上卸RCU 回调。这种方法会rcuc/%u kthread 没有任何工作可做，从而永远不会被唤醒4. 确保CPU 永不进入内核态，尤其是避免在CPU 上发起任CPU 热插拔操作。这是防止任	回调被排队到CPU 上的另一种方式，同样能让 rcuc/%u kthread 没有任何工作可做
Name:
  rcuop/%d, rcuos/%d, and rcuog/%d

Purpose:
  从相应的 CPU 上卸RCU 回调
要减少其操作系统抖动，请至少执行以下一项：

1. 使用亲和性、cgroup 或其它机制，强制这些 kthread 在其CPU 上执行2. CONFIG_RCU_NOCB_CPU=n 构建，这将从一开始就阻止这些 kthread 被创建。不过请注意，这
	并不会消除操作系统抖动，而是会把它转移到 RCU_SOFTIRQ 上