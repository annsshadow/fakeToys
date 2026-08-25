
## 用户空间调试建议


本文档简要概述了从用户空间调Linux 内核的常用工具面向驱动开发者的调试建议请见 :doc:`此处
</process/debugging/driver_development_debugging_guide>`关于一般性调试建议，:doc:`通用建议文档
</process/debugging/index>`銆。
    :depth: 3

以下各节向你展示可用的工具
### Dynamic debug（动态调试）


通过启用/禁用日志消息来过滤最终进入内核日志的内容的机制
前置条件：`CONFIG_DYNAMIC_DEBUG`

动态调试只能针对以下目标：

- pr_debug()
- dev_dbg()
- print_hex_dump_debug()
- print_hex_dump_bytes()

因此，就目前而言，此工具的可用性相当有限，因为向代码库添加调试打印并没统一的规则，导致这些打印的实现方式五花八门
另外请注意，大多数调试语句都实现dprintk() 的某种变体，必须通过相应模块
中的参数来激活，动态调试无法替你完成这一步
```

  $ alias ddcmd='echo $* > /proc/dynamic_debug/control'
  $ ddcmd '-p; file v4l2-h264.c +p'
  $ grep =p /proc/dynamic_debug/control
   drivers/media/v4l2-core/v4l2-h264.c:372 [v4l2_h264]print_ref_list_b =p
   "ref_pic_list_b%u (cur_poc %u%c) %s"
   drivers/media/v4l2-core/v4l2-h264.c:333 [v4l2_h264]print_ref_list_p =p
   "ref_pic_list_p (cur_poc %u%c) %s\n"

```
**何时应该优先使用它而不Ftrace*

- 当代码中包含有效的打印语句之一（见上文）时，或者当你在开发过程中添加  多个 pr_debug() 语句- 当时序不成问题时，即代码中的多个 pr_debug() 语句不会引起延迟- 当你更关心接收特定的日志消息，而不是追踪函数被调用的模式时

完整文档[/admin-guide/dynamic-debug-howto](/admin-guide/dynamic-debug-howto)

### Ftrace


前置条件：`CONFIG_DYNAMIC_FTRACE`

此工具使tracefs 文件系统来存放控制文件和输出文件。该文件系统会被挂载一`tracing` 目录，可以在 `/sys/kernel/` `/sys/debug/kernel/` 中找到
一些最重要的调试操作为
- 你可以通过将函数名添加`set_ftrace_filter` 文件（它接受
  `available_filter_functions` 文件中出现的任何函数名）来执行函数跟踪；或  你也可以将特定函数的名称添加`set_ftrace_notrace` 文件来禁用它们（更多
  信息见：trace/ftrace:dynamic ftrace）- 为了找出调用的来源，你可以激`options/func_stack_trace` 下的
  `func_stack_trace` 选项- 通过把期望的函数添加`set_graph_function` 文件中（需要配  `FUNCTION_GRAPH_RETVAL`），可以跟踪函数调用的子函数并显示返回值；更多信息  trace/ftrace:dynamic ftrace with the function graph tracer
完整Ftrace 文档[/trace/ftrace](/trace/ftrace)

或者，你也可以通过 :ref:`使用事件跟踪
<trace/events:2. using event tracing>` 来跟踪特定事件，其定义方式见此处:ref:`创建一个自定义Ftrace 跟踪<process/debugging/driver_development_debugging_guide:ftrace>`
完整Ftrace 事件跟踪文档[/trace/events](/trace/events)


#### Reading the ftrace log（读ftrace 日志

`trace` 文件可以像任何其他文件一样读取（`cat`、`tail`、`head`、`vim` 等）文件的大小受 `buffer_size_kb` 限制（`echo 1000 > buffer_size_kb`）trace/ftrace:trace_pipe 的行为与 `trace` 文件类似，但每当你从该文件读取时内容会被消费掉
#### Kernelshark


一GUI 界面，用于将 `trace-cmd
<https://git.kernel.org/pub/scm/utils/trace-cmd/trace-cmd.git/>`__ 应用程序输出可视化为图形和列表视图
完整文档`<https://kernelshark.org/Documentation.html>`__

### Perf 及替代工

上面提到的工具提供了检查内核代码、结果、变量值等的方法。有时你首先得弄清楚
从哪里入手去看，对于这些情况，一套性能跟踪工具可以帮助你框定问题
#### 为什么应该做性能分析

在以下原因之一等情况下，性能分析是一个很好的第一步：

- 你无法界定问- 你不知道它发生在哪里
- 运行中的系统不应被打断，或者它是一个远程系统，你无法在其中安装新的
  模块/内核

#### 如何linux 工具做一个简单的分析

在性能分析的开头，你可以从常用工具开始，例如
- `top` / `htop` / `atop`获取系统负载概览，查看特定进程上的尖- `mpstat -P ALL`*查看 CPU 之间的负载分*- `iostat -x`*观察输入输出设备的利用率和性能**- `vmstat`*系统内存使用概览**- `pidstat`*类似* `vmstat` *但按进程，以便聚焦到目标*- `strace -tp $PID`一旦你知道了进程，就可以弄清楚它如何与内核通信*
这些应该有助于充分缩小要查看的范围
#### Diving deeper with perf（用 perf 深入挖掘

**perf** 工具提供了一系列指标和事件，以进一步聚焦问题
前置条件：在你的系统上构建或安装 perf

```

  # perf stat -d find /usr -name 'gcc*' | wc -l

   Performance counter stats for 'find /usr -name gcc*':

     1277.81 msec    task-clock             #    0.997 CPUs utilized
     9               context-switches       #    7.043 /sec
     1               cpu-migrations         #    0.783 /sec
     704             page-faults            #  550.943 /sec
     766548897       cycles                 #    0.600 GHz                         (97.15%)
     798285467       instructions           #    1.04  insn per cycle              (97.15%)
     57582731        branches               #   45.064 M/sec                       (2.85%)
     3842573         branch-misses          #    6.67% of all branches             (97.15%)
     281616097       L1-dcache-loads        #  220.390 M/sec                       (97.15%)
     4220975         L1-dcache-load-misses  #    1.50% of all L1-dcache accesses   (97.15%)
     <not supported> LLC-loads
     <not supported> LLC-load-misses

   1.281746009 seconds time elapsed

   0.508796000 seconds user
   0.773209000 seconds sys


  52

```
事件和指标的可用性取决于你运行的系统
完整文档`<https://perf.wiki.kernel.org/index.php/Main_Page>`__

#### Perfetto


一套用于测量和分析应用程序与系统表现如何的工具。你可以借助它来
- 识别瓶颈
- 优化代码
- 让软件运行得更快、更高效
**perfetto perf 有什么区别？**

- perf 是作Linux 内核一部分、并专门针对 Linux 内核的工具，具有 CLI 用户
  界面- perfetto 是跨平台的性能分析技术栈，将功能扩展到用户空间，并提WEB
  用户界面
完整文档`<https://perfetto.dev/docs/>`__

### Kernel panic analysis tools（内核崩溃分析工具）


  要捕获崩溃转储请使用 `Kdump` `Kexec`。下面你可以找到一些分析数据的建议
  完整文档[/admin-guide/kdump/kdump](/admin-guide/kdump/kdump)

  为了找出代码中对应的行，你可以使`faddr2line
  <https://elixir.bootlin.com/linux/v6.11.6/source/scripts/faddr2line>`__；注  要使它工作，你需要启`CONFIG_DEBUG_INFO`
  使用 `faddr2line` 的替代方案是使用 `objdump`（以及针对不同平台的衍生工具  `aarch64-linux-gnu-objdump`）。以这一行为例：

  `[  +0.000240]  rkvdec_device_run+0x50/0x138 [rockchip_vdec]`銆。
```

    aarch64-linux-gnu-objdump -dS drivers/staging/media/rkvdec/rockchip-vdec.ko | grep rkvdec_device_run\>: -A 40
    0000000000000ac8 <rkvdec_device_run>:
     ac8:	d503201f 	nop
     acc:	d503201f 	nop
    {
     ad0:	d503233f 	paciasp
     ad4:	a9bd7bfd 	stp	x29, x30, [sp, #-48]!
     ad8:	910003fd 	mov	x29, sp
     adc:	a90153f3 	stp	x19, x20, [sp, #16]
     ae0:	a9025bf5 	stp	x21, x22, [sp, #32]
        const struct rkvdec_coded_fmt_desc *desc = ctx->coded_fmt_desc;
     ae4:	f9411814 	ldr	x20, [x0, #560]
        struct rkvdec_dev *rkvdec = ctx->dev;
     ae8:	f9418015 	ldr	x21, [x0, #768]
        if (WARN_ON(!desc))
     aec:	b4000654 	cbz	x20, bb4 <rkvdec_device_run+0xec>
        ret = pm_runtime_resume_and_get(rkvdec->dev);
     af0:	f943d2b6 	ldr	x22, [x21, #1952]
        ret = __pm_runtime_resume(dev, RPM_GET_PUT);
     af4:	aa0003f3 	mov	x19, x0
     af8:	52800081 	mov	w1, #0x4                   	// #4
     afc:	aa1603e0 	mov	x0, x22
     b00:	94000000 	bl	0 <__pm_runtime_resume>
        if (ret < 0) {
     b04:	37f80340 	tbnz	w0, #31, b6c <rkvdec_device_run+0xa4>
        dev_warn(rkvdec->dev, "Not good\n");
     b08:	f943d2a0 	ldr	x0, [x21, #1952]
     b0c:	90000001 	adrp	x1, 0 <rkvdec_try_ctrl-0x8>
     b10:	91000021 	add	x1, x1, #0x0
     b14:	94000000 	bl	0 <_dev_warn>
        *bad = 1;
     b18:	d2800001 	mov	x1, #0x0                   	// #0
     ...
```

**Copyright** 漏2024 : Collabora
