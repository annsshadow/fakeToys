# Timerlat 跟踪器


timerlat 跟踪器旨在帮助抢占式内核开发者找到实时线程唤醒延迟的来源。与 cyclictest 类似，该跟踪器设置一个周期性定时器来唤醒一个线程。然后该线程计算一个**唤醒延迟**值，即*当前时间**与定时器被设置为到期的**绝对时间*之间的差值。timerlat 的主要目标是以帮助内核开发者的方式进行跟踪。

### 用法


将 ASCII 文本 “timerlat” 写入跟踪系统的 current_tracer 文件（通常挂载在 /sys/kernel/tracing）。

```

        [root@f32 ~]# cd /sys/kernel/tracing/
        [root@f32 tracing]# echo timerlat > current_tracer

```
```

  [root@f32 tracing]# cat trace
  # tracer: timerlat
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            || /
  #                            ||||             ACTIVATION
  #         TASK-PID      CPU# ||||   TIMESTAMP    ID            CONTEXT                LATENCY
  #            | |         |   ||||      |         |                  |                       |
          <idle>-0       [000] d.h1    54.029328: #1     context    irq timer_latency       932 ns
           <...>-867     [000] ....    54.029339: #1     context thread timer_latency     11700 ns
          <idle>-0       [001] dNh1    54.029346: #1     context    irq timer_latency      2833 ns
           <...>-868     [001] ....    54.029353: #1     context thread timer_latency      9820 ns
          <idle>-0       [000] d.h1    54.030328: #2     context    irq timer_latency       769 ns
           <...>-867     [000] ....    54.030330: #2     context thread timer_latency      3070 ns
          <idle>-0       [001] d.h1    54.030344: #2     context    irq timer_latency       935 ns
           <...>-868     [001] ....    54.030347: #2     context thread timer_latency      4351 ns


```
该跟踪器创建一个具有实时优先级 SCHED_FIFO:95 的每 CPU 内核线程，在每次激活时打印两行。第一行是在线程激活**之前**、在**硬中断**上下文观察到的**定时器延迟**。第二行是该线程观察到的**定时器延迟**。ACTIVATION ID 字段用于将**irq**执行与其相应的**线程**执行关联起来。

**irq**/**线程**的拆分对于澄清异常高的值来自哪个上下文很重要。**irq** 上下文可能被与硬件相关的动作延迟，例如 SMI、NMI、IRQ，或者被线程屏蔽中断所延迟。一旦定时器触发，延迟也可能受到线程引起的阻塞的影响。例如，通过 preempt_disable()、调度器执行或屏蔽中断来推迟调度器执行。线程也可能被其他线程和 IRQ 的干扰所延迟。

### 跟踪器选项


timerlat 跟踪器建立在 osnoise 跟踪器之上。因此它的配置也在 osnoise/ 配置目录中完成。timerlat 的配置有：

 - cpus：timerlat 线程将在其上执行的 CPU。
 - timerlat_period_us：timerlat 线程的周期。
 - stop_tracing_us：如果**irq**上下文中的定时器延迟高于配置的值，则停止系统跟踪。写入 0 会禁用此选项。
 - stop_tracing_total_us：如果**线程**上下文中的定时器延迟高于配置的值，则停止系统跟踪。写入 0 会禁用此选项。
 - print_stack：保存 IRQ 发生的栈。该栈在**线程上下文**事件之后打印，或者在命中 **stop_tracing_us** 时在 IRQ 处理程序中打印。

### timerlat 与 osnoise


timerlat 也可以利用 osnoise: traceevents。
```

        [root@f32 ~]# cd /sys/kernel/tracing/
        [root@f32 tracing]# echo timerlat > current_tracer
        [root@f32 tracing]# echo 1 > events/osnoise/enable
        [root@f32 tracing]# echo 25 > osnoise/stop_tracing_total_us
        [root@f32 tracing]# tail -10 trace
             cc1-87882   [005] d..h...   548.771078: #402268 context    irq timer_latency     13585 ns
             cc1-87882   [005] dNLh1..   548.771082: irq_noise: local_timer:236 start 548.771077442 duration 7597 ns
             cc1-87882   [005] dNLh2..   548.771099: irq_noise: qxl:21 start 548.771085017 duration 7139 ns
             cc1-87882   [005] d...3..   548.771102: thread_noise:      cc1:87882 start 548.771078243 duration 9909 ns
      timerlat/5-1035    [005] .......   548.771104: #402268 context thread timer_latency     39960 ns

```
在这种情况下，定时器延迟的根本原因并不指向单一原因，而是指向多个原因。首先，定时器 IRQ 被延迟了 13 us，这可能指向一个较长的禁用中断区段（见 IRQ 栈跟踪一节）。然后，唤醒 timerlat 线程的定时器中断花了 7597 ns，而 qxl:21 设备 IRQ 花了 7139 ns。最后，在上下文切换之前，cc1 线程噪声占用了 9909 ns 的时间。这些证据对开发者使用其他跟踪方法来弄清如何调试和优化系统很有帮助。

值得一提的是，osnoise: 事件报告的**duration**值是**净**值。例如，thread_noise 不包括由 IRQ 执行引起的开销持续时间（其确实占用了 12736 ns）。但 timerlat 跟踪器报告的值（timerlat_latency）是**毛**值。

下面的示意图展示了一条 CPU 时间线，以及 timerlat 跟踪器在顶部、osnoise: 事件在底部如何观察它。每个 “-”
```

      External     timer irq                   thread
       clock        latency                    latency
       event        13585 ns                   39960 ns
         |             ^                         ^
         v             |                         |
         |-------------|                         |
         |-------------+-------------------------|
                       ^                         ^
  ========================================================================
                    [tmr irq]  [dev irq]
  [another thread...^       v..^       v.......][timerlat/ thread]  <-- CPU timeline
  =========================================================================
                    |-------|  |-------|
                            |--^       v-------|
                            |          |       |
                            |          |       + thread_noise: 9909 ns
                            |          +-> irq_noise: 6139 ns
                            +-> irq_noise: 7597 ns

```
### IRQ 栈跟踪


osnoise/print_stack 选项对于那些由于抢占或
```

        [root@f32 tracing]# echo 500 > osnoise/stop_tracing_total_us
        [root@f32 tracing]# echo 500 > osnoise/print_stack
        [root@f32 tracing]# echo timerlat > current_tracer
        [root@f32 tracing]# tail -21 per_cpu/cpu7/trace
          insmod-1026    [007] dN.h1..   200.201948: irq_noise: local_timer:236 start 200.201939376 duration 7872 ns
          insmod-1026    [007] d..h1..   200.202587: #29800 context    irq timer_latency      1616 ns
          insmod-1026    [007] dN.h2..   200.202598: irq_noise: local_timer:236 start 200.202586162 duration 11855 ns
          insmod-1026    [007] dN.h3..   200.202947: irq_noise: local_timer:236 start 200.202939174 duration 7318 ns
          insmod-1026    [007] d...3..   200.203444: thread_noise:   insmod:1026 start 200.202586933 duration 838681 ns
      timerlat/7-1001    [007] .......   200.203445: #29800 context thread timer_latency    859978 ns
      timerlat/7-1001    [007] ....1..   200.203446: <stack trace>
  => timerlat_irq
  => __hrtimer_run_queues
  => hrtimer_interrupt
  => __sysvec_apic_timer_interrupt
  => asm_call_irq_on_stack
  => sysvec_apic_timer_interrupt
  => asm_sysvec_apic_timer_interrupt
  => delay_tsc
  => dummy_load_1ms_pd_init
  => do_one_initcall
  => do_init_module
  => __do_sys_finit_module
  => do_syscall_64
  => entry_SYSCALL_64_after_hwframe

```
线程噪声成为导致定时器延迟的主要因素的情况很有帮助，因为在 timerlat IRQ 处理程序期间保存的栈跟踪指向了一个名为
```

	static int __init dummy_load_1ms_pd_init(void)
	{
		preempt_disable();
		mdelay(1);
		preempt_enable();
		return 0;

	}

```
的函数。

### 用户空间接口


timerlat 允许用户空间线程使用 timerlat 基础设施来测量调度延迟。此接口可通过 $tracing_dir/osnoise/per_cpu/cpu$ID/timerlat_fd 内的每 CPU 文件描述符访问。

此接口在以下条件下可访问：

 - timerlat 跟踪器已启用
 - osnoise workload 选项设为 NO_OSNOISE_WORKLOAD
 - 用户空间线程被绑定到单一处理器
 - 线程打开了与其单一处理器相关联的文件
 - 一次只能有一个线程访问该文件

如果不满足上述任何条件，open() 系统调用将失败。打开文件描述符后，用户空间可以从中读取。

read() 系统调用将运行一段 timerlat 代码，它会像常规内核线程那样在未来设置定时器并等待它。

当定时器 IRQ 触发时，timerlat IRQ 将执行，报告 IRQ 延迟并唤醒在 read 中等待的线程。该线程将被调度，并像内核线程一样通过跟踪器报告线程延迟。

与内核内 timerlat 的不同之处在于，timerlat 不会重新设置定时器，而是返回到 read() 系统调用。此时，用户可以运行任何代码。

如果用户重新读取 timerlat 文件描述符，跟踪器将报告从用户空间返回的延迟，即总延迟。如果这是工作的结束，它可以解释为请求的响应时间。

在报告总延迟之后，timerlat 将重启循环，设置定时器，并为下一次激活进入睡眠。

如果任何时候某个条件被破坏，例如线程在用户空间中迁移，或者 timerlat 跟踪器被禁用，则会向用户空间线程发送 SIG_KILL 信号。

```

 int main(void)
 {
	char buffer[1024];
	int timerlat_fd;
	int retval;
	long cpu = 0;   /* 放置于 CPU 0 */
	cpu_set_t set;

	CPU_ZERO(&set);
	CPU_SET(cpu, &set);

	if (sched_setaffinity(gettid(), sizeof(set), &set) == -1)
		return 1;

	snprintf(buffer, sizeof(buffer),
		"/sys/kernel/tracing/osnoise/per_cpu/cpu%ld/timerlat_fd",
		cpu);

	timerlat_fd = open(buffer, O_RDONLY);
	if (timerlat_fd < 0) {
		printf("error opening %s: %s\n", buffer, strerror(errno));
		exit(1);
	}

	for (;;) {
		retval = read(timerlat_fd, buffer, 1024);
		if (retval < 0)
			break;
	}

	close(timerlat_fd);
	exit(0);
 }

```
