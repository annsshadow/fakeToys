## ftrace - 函数跟踪


版权所2008 Red Hat 公司

:作   Steven Rostedt <srostedt@redhat.com>
:许可  The GNU Free Documentation License, Version 1.2
          (GPL v2 下双重许

:原始审阅  Elias Oltmanns, Randy Dunlap, Andrew Morton,
		      John Kacur, and David Teigland.

- 编写针对: 2.6.28-rc2
- 更新针对: 3.10
- 更新针对: 4.13 - 版权所2017 VMware 公司 Steven Rostedt
- 转换rst 格式 - Changbin Du <changbin.du@intel.com>

### 简


ftrace 是一个内部跟踪器，旨在帮助开发者与系统设计者了解内核内
正在发生什么。它可用于调试或分析发生在用户空间之外的延迟和性能问题

虽然 ftrace 通常被认为是一个函数跟踪器，但它实际上是由多个不同
跟踪工具组成的框架。其中包括延迟跟踪，用于检查从中断禁用到启用之间
以及抢占之间、从任务被唤醒到该任务真正被调度进来之间所发生的情况

ftrace 最常见的用途之一是事件跟踪。内核中遍布数百个静态事件点
可以通过 tracefs 文件系统启用，以查看内核某些部分内部正在发生什么

更多信息请参events.rst


### 实现细节


架构移植者等相关细节请参Documentation/trace/ftrace-design.rst


### 文件系统


ftrace 使用 tracefs 文件系统来保存控制文件以及用于显示输出的文件

tracefs 被配置进内核（选择任意 ftrace 选项都会如此）时，会创建
目录 /sys/kernel/tracing。挂载方式如下：

```

 tracefs       /sys/kernel/tracing       tracefs defaults        0       0

```

```

 mount -t tracefs nodev /sys/kernel/tracing

```
为了方便访问该目录，你可能想要建立一个软链接

```

 ln -s /sys/kernel/tracing /tracing

```

  4.1 之前，所ftrace 跟踪控制文件都位debugfs 文件系统中，
  通常位于 /sys/kernel/debug/tracing。为了向后兼容，当挂debugfs
  文件系统、tracefs 文件系统会自动挂载到

  /sys/kernel/debug/tracing

  位于 tracefs 文件系统中的全部文件也会出现在该 debugfs 文件系统
  目录中

  任何被选中ftrace 选项也会创建 tracefs 文件系统。文档的其余部分
  将假设你处于 ftrace 目录中（cd /sys/kernel/tracing），并且只会
  关注该目录内的文件，而不会用冗长"/sys/kernel/tracing" 路径
  来分散对内容的注意力

就这样！（假设你已经ftrace 配置进了内核

挂载 tracefs 之后，你将可以访ftrace 的控制和输出文件。以下是其中
一些关键文件的列表


 注意：所有时间值均以微秒为单位

  current_tracer:

	该文件用于设置或显示当前已配置的跟踪器。更改当前跟踪器会同
	清除环形缓冲区的内容以及 "snapshot"（快照）缓冲区

  available_tracers:

	该文件保存已编译进内核的各种不同类型的跟踪器。这里列出的
	跟踪器可以通过将它们的名字 echo current_tracer 来配置

  tracing_on:

	该文件设置或显示是否启用了向跟踪环形缓冲区的写入。向该文
	echo 0 可禁用跟踪器，echo 1 可启用它。注意，这只会禁用向环形
	缓冲区的写入，跟踪开销仍有可能在继续发生

	内核函数 tracing_off() 可用于在内核内部禁用向环形缓冲区的写入，
	这会把该文件设置"0"。用户空间可以通过向该文件 echo "1" 
	重新启用跟踪

	注意，函数和事件"traceoff" 触发器也会将该文件清零并停止跟踪
	它同样可以由用户空间使用该文件来重新启用

  trace:

	该文件以人类可读的格式（如下所述）保存跟踪的输出。以 O_TRUNC
	标志打开该文件进行写入会清除环形缓冲区的内容。注意，该文
	不是一个消费型文件。如果跟踪已关闭（没有跟踪器在运行，
	tracing_on 为零），每次读取它都会产生相同的输出。当跟踪开启时
	由于它试图在不消费的情况下读取整个缓冲区，可能会产生不一致的
	结果

  trace_pipe:

	其输出与 "trace" 文件相同，但该文件用于配合实时跟踪进行流式读取
	从该文件读取会阻塞，直到获取到新数据。与 "trace" 文件不同，该文件
	是一个消费型文件。这意味着从该文件读取会导致顺序读取显示更新的
	数据。一旦数据从该文件读出，它就被消费掉了，顺序读取将不会再读到
	它trace" 文件是静态的，如果跟踪器没有添加更多数据，每次读取它
	都会显示相同的信息

  trace_options:

	该文件让用户可以控制上述某个输出文件中所显示的数据量。也有一
	选项用于修改跟踪器或事件的工作方式（栈回溯、时间戳等）

  options:

	这是一个目录，其中包含每个可用跟踪选项（同样存在于 trace_options
	中）对应的一个文件。也可以向对应选项名的文件写入 "1" "0" 
	设置或清除该选项

  tracing_max_latency:

	部分跟踪器会记录最大延迟。例如，中断被禁用的最长时间。最大时
	保存在该文件中。最大跟踪记录也会被保存，并"trace" 显示。只有当
	延迟大于该文件中的值时，才会记录一条新的最大跟踪（以微秒为单位）

	向该文件 echo 一个时间值后，除非延迟大于该文件中的时间，否则不
	记录任何延迟

  tracing_thresh:

	当延迟大于该文件中的数值时，部分延迟跟踪器会记录一条跟踪。仅
	该文件中的数值大0 时才生效。（以微秒为单位

  buffer_percent:

	这是环形缓冲区在被唤醒之前需要填充多少的水位线。也就是说，如果一
	应用程序在一per_cpu trace_pipe_raw 文件上调用阻塞读取系统调用，
	它会一直阻塞，直到 buffer_percent 指定的给定数量的数据进入环形缓冲区，
	才会唤醒读取者。这

```
	  0   - 表示一旦环形缓冲区中有任何数据就唤醒
	  50  - 表示当大约一半的环形缓冲区子缓冲区填满时唤醒
	  100 - 表示一直阻塞直到环形缓冲区完全填满，即将开始覆盖旧数据
```

  buffer_size_kb:

	该文件设置或显示每个 CPU 缓冲区所持有的千字节数。默认情况下，每
	CPU 的跟踪缓冲区大小相同。显示的数值是 CPU 缓冲区的大小，而不是所
	缓冲区的总大小。跟踪缓冲区以页（内核用于分配的内存块，通常4 KB
	为单位分配。可能会额外分配少许几页以容纳缓冲区的管理元数据。如果最
	分配的页中还有多于请求字节的空间，该页的剩余部分也会被使用，使得
	实际分配量大于所请求或显示的
	（注意，由于缓冲区管理元数据的原因，该大小可能不是页大小的整数倍。）

	单个 CPU 的缓冲区大小可能会不同（见下面的 "per_cpu/cpu0/buffer_size_kb"），
	如果不同，该文件将显"X"

  buffer_total_size_kb:

	该文件显示所有跟踪缓冲区合并后的总大小

  buffer_subbuf_size_kb:

	该文件设置或显示子缓冲区的大小。环形缓冲区被划分为若干个相同大小的
	"子缓冲区"。一个事件不能大于子缓冲区的大小。通常，子缓冲区的大小
	等于架构的页大小（x86 上为 4K）。子缓冲区开头还包含元数据，这同
	限制了事件的大小。这意味着当子缓冲区为一个页大小时，没有任何事件
	大于页大小减去子缓冲区元数据

	注意，buffer_subbuf_size_kb 是用户指定子缓冲区最小大小的一种方式
	由于实现细节，内核可能会把它变大，或者如果内核无法处理该请求，则直接
	使操作失败

	更改子缓冲区大小可以让事件大于页大小

	注意：更改子缓冲区大小时，跟踪会停止，环形缓冲区和快照缓冲区中的任何
	数据都会被丢弃

  free_buffer:

	如果一个进程正在执行跟踪，并且该进程的环形缓冲区应在其结束时（即使
	它被信号杀死）被收"释放"，则可以使用该文件来实现此目的。在该文
	关闭时，环形缓冲区会被重置为最小大小。让正在跟踪的进程同时打开该文件，
	当该进程退出时，其对应此文件的文件描述符会被关闭，在此过程中环形缓冲区
	会被"释放"

	如果设置disable_on_free 选项，它也可能停止跟踪

  tracing_cpumask:

	这是一个掩码，让用户只能在特定 CPU 上进行跟踪。格式为表示 CPU 
	十六进制字符串

  set_ftrace_filter:

	当配置了动ftrace 时（见下面的 "dynamic ftrace" 一节），代码会
	动态修改（代码文本重写）以禁用对函数性能分析器（mcount）的调用。这使得
	配置跟踪带来的性能开销几乎可以忽略不计。这还有一个副作用，即能够启用
	或禁用对特定函数的跟踪。向该文echo 函数名，将把跟踪限制为仅这些函数
	这会影响 "function" "function_graph" 跟踪器，因此也影响函数性能分析
	（见 "function_profile_enabled"）

	可以写入该文件的函数列在 "available_filter_functions" 中

	该接口也允许使用命令。更多细节请参阅 "Filter commands"（过滤命令）一节

	作为一种加速手段，由于处理字符串可能相当昂贵，并且需要检查所有注册到
	跟踪中的函数，因此可以改为向该文件写入一个索引。写入一个数字（"1"
	开头）将改为选择 "available_filter_functions" 文件中对应行位置的相
	函数

  set_ftrace_notrace:

	该文件的作用set_ftrace_filter 相反。添加到此处的任何函数都不会
	跟踪。如果一个函数同时存在于 set_ftrace_filter set_ftrace_notrace
	中，则该函数在_不_会被跟踪

  set_ftrace_pid:

	让函数跟踪器只跟PID 列在该文件中的线程

	如果设置"function-fork" 选项，那么当 PID 列在该文件中的任fork 时，
	子进程的 PID 会自动添加到该文件中，子进程也将被函数跟踪器跟踪。该选项
	还会导致退出的任务PID 从该文件中被移除

  set_ftrace_notrace_pid:

        让函数跟踪器忽略 PID 列在该文件中的线程

        如果设置"function-fork" 选项，那么当 PID 列在该文件中的任fork 时，
	子进程的 PID 会自动添加到该文件中，子进程也不会被函数跟踪器跟踪。该选项
	同样会导致退出的任务PID 从该文件中被移除

        如果一PID 同时存在于该文件"set_ftrace_pid" 中，则该文件优先
	该线程不会被跟踪

  set_event_pid:

	让事件只跟踪 PID 列在该文件中的任务。注意，sched_switch sched_wake_up
	也会跟踪列在该文件中的事件

	要让列在该文件中的任务的子进PID fork 时被添加进来，请启用
	"event-fork" 选项。该选项还会导致任务PID 在任务退出时从该文件中被移除

  set_event_notrace_pid:

	让事件不跟踪 PID 列在该文件中的任务。注意，sched_switch sched_wakeup
	会跟踪未列在该文件中的线程，即使某个线程PID 在该文件中，如果
	sched_switch sched_wakeup 事件同时也跟踪某个应当被跟踪的线程

	要让列在该文件中的任务的子进PID fork 时被添加进来，请启用
	"event-fork" 选项。该选项还会导致任务PID 在任务退出时从该文件中被移除

  set_graph_function:

	列在该文件中的函数会使函数图跟踪器只跟踪这些函数以及它们所调用的函数
	（更多细节见 "dynamic ftrace" 一节。）注意，set_ftrace_filter 
	set_ftrace_notrace 仍然会影响哪些函数被跟踪

  set_graph_notrace:

	类似set_graph_function，但在命中该函数时禁用函数图跟踪，直到它退
	该函数为止。这样可以忽略对某个特定函数所调用函数的跟踪

  available_filter_functions:

	该文件列ftrace 已处理并且可以跟踪的函数。这些就是你可以传递给
	"set_ftrace_filter"銆?set_ftrace_notrace"銆?set_graph_function" 鎴。
	"set_graph_notrace" 的函数名
	（更多细节见下面"dynamic ftrace" 一节。）

  available_filter_functions_addrs:

	类似available_filter_functions，但为每个函数显示地址。显示的地址
	补丁站点地址，可能与 /proc/kallsyms 中的地址不同

  syscall_user_buf_size:

	部分系统调用跟踪事件会记录某个参数所指向的用户空间地址中的数据。每
	事件的数据量是受限的。该文件保存将被记录进环形缓冲区以保存这些数据的最
	字节数。当前最大值为 165

  dyn_ftrace_total_info:

	该文件用于调试目的。显示已被转换成 nop 并且可用于跟踪的函数数量

  enabled_functions:

	该文件更多用于调ftrace，但在查看是否有任何函数挂接了回调时也很有用
	不仅跟踪基础设施会用ftrace 的函数跟踪功能，其他子系统也可能用到。该文件
	显示所有挂接了回调的函数，以及已挂接的回调数量。注意，一个回调也可能调用
	多个函数，这些不会被计入此计数

	如果注册的回调是"save regs" 属性（因此开销更大）被跟踪的函数，则在
	返回寄存器的函数同一行会显示一'R'

	如果注册的回调是"ip modify" 属性（因此 regs->ip 可以被修改）被跟踪的
	函数，则在同一行会显示一'I'

	如果挂接了一个非 ftrace 的蹦床（BPF），则会显示一'D'。注意，普通的
	ftrace 蹦床也可以挂接，但一个给定的函数一次只能挂接一直接"蹦床

	某些架构无法调用直接蹦床，而是ftrace ops 函数放置在函数入口点之上
	在这种情况下会显示一'O'

	如果一个函数过去曾挂接"ip modify" 或直接调用，则会显示一'M'。该标志
	永远不会被清除。它用于了解某个函数是否曾被 ftrace 基础设施修改过，可用
	调试

	如果架构支持，它还会显示该函数正在直接调用的回调。如果计数大1，则很可
	鏄?ftrace_ops_list_func()銆。

	如果一个函数的回调跳转到一个特定于该回调而非标准蹦床的蹦床，则会打印其地址
	以及该蹦床所调用的函数

  touched_functions:

	该文件包含曾通过 ftrace 基础设施挂接过函数回调的所有函数。它的格式与
	enabled_functions 相同，但显示的是所有曾经被跟踪过的函数

	要查看任何曾"ip modify" 或直接蹦床修改过的函数，可以执行以下命令

	grep ' M ' /sys/kernel/tracing/touched_functions

  function_profile_enabled:

	设置该文件时，它会启用所有函数的 function 跟踪器，如果已配置，则启用函数图
	跟踪器。它会保存被调用函数数量的直方图，如果配置了函数图跟踪器，它还会记录
	这些函数所花费的时间。直方图内容可以显示在以下文件中

	trace_stat/function<cpu>（function0、function1 等）

  trace_stat:

	一个保存不同跟踪统计信息的目录

  kprobe_events:

	启用动态跟踪点。请参阅 kprobetrace.rst

  kprobe_profile:

	动态跟踪点统计信息。请参阅 kprobetrace.rst

  max_graph_depth:

	与函数图跟踪器配合使用。这是它将跟踪进入函数的深度。将其设置为 1 将只显示
	从用户空间调用的第一个内核函数

  printk_formats:

	该文件供读取原始格式文件的工具使用。如果环形缓冲区中的一个事件引用了一
	字符串，则只把指向该字符串的指针记录进缓冲区，而不是字符串本身。这导致工具
	无法知道那个字符串是什么。该文件显示字符串及其地址，使工具能够将指针映
	到对应的字符串

  saved_cmdlines:

	除非事件特别保存了任务的 comm，否则在跟踪事件中只记录任务pid。ftrace
	建立一pid comm 的映射缓存，以尝试为事件显示 comm。如果某comm pid
	未列出，则输出中会显"<...>"

	如果 "record-cmd" 选项被设置为 "0"，则在记录期间不会保存任务的 comm。默
	情况下它是启用的

  saved_cmdlines_size:

	默认情况下保128 comm（见上面"saved_cmdlines"）。要增加或减少被缓存
	comm 数量，向该文echo 要缓存的 comm 数量

  saved_tgids:

	如果设置"record-tgid" 选项，则每次调度上下文切换时，任务的线程ID 会被
	保存到一个映PID 到其 TGID 的表中。默认情况下record-tgid" 选项是禁用的

  snapshot:

	该文件显快照"缓冲区，并允许用户对当前正在运行的跟踪拍摄快照。更多细
	请参阅下面的 "Snapshot"（快照）一节

  stack_max_size:

	当启用栈跟踪器时，该文件会显示它所遇到的最大栈大小。请参阅下面"Stack Trace"
	（栈跟踪）一节

  stack_trace:

	该文件显示启用栈跟踪器时所遇到的最大栈的栈回溯。请参阅下面"Stack Trace"
	（栈跟踪）一节

  stack_trace_filter:

	该文件类似于 "set_ftrace_filter"，但它限制栈跟踪器所检查的函数

  trace_clock:

	每当一个事件被记录进环形缓冲区时，都会添加一时间。该时间戳来自某
	指定的时钟。默认情况下，ftrace 使用 "local" 时钟。该时钟非常快并且严格按
	每个 CPU 独立，但在某些系统上它相对于其他 CPU 可能不是单调的。换句话说，
	本地时钟可能与其CPU 上的本地时钟不同步

	跟踪常用的时钟：

```
	  # cat trace_clock
	  [local] global counter x86-tsc
```

	带有方括号的时钟是正在生效的时钟

	local:
		默认时钟，但可能在不CPU 之间不同

	global:
		该时钟与所CPU 同步，但可能比本地时钟稍慢

	counter:
		这根本不是时钟，而是一个字面上的原子计数器。它逐个递增，但与所
		CPU 同步。当你需要确切知道不CPU 上事件相互之间的发生顺序时，
		很有用

	uptime:
		该时钟使jiffies 计数器，时间戳相对于系统启动后的时间

	perf:
		该时钟使 ftrace 使用perf 相同的时钟。最perf 将能够读ftrace
		缓冲区，这将有助于交织合并数据

	x86-tsc:
		架构可以定义自己的时钟。例如，x86 在这里使用自己的 TSC 周期时钟

	ppc-tb:
		该时钟使powerpc timebase 寄存器值。它与所CPU 同步，并且如
		已知 tb_offset，还可用于关联管理程客户机之间的事件

	mono:
		该时钟使用快速单调时钟（CLOCK_MONOTONIC），它是单调的，并受 NTP 速率
		调整影响

	mono_raw:
		该时钟是原始单调时钟（CLOCK_MONOTONIC_RAW），它是单调的，但不受任
		速率调整影响，并以与硬件时钟源相同的速率走时

	boot:
		该时钟是启动时钟（CLOCK_BOOTTIME），基于快速单调时钟，但也会计入在
		挂起状态中所花费的时间。由于时钟访问是为在挂起路径中的跟踪使用而设计，
		如果在快mono 时钟更新之前、在挂起时间被计入之后访问该时钟，可能会
		产生一些副作用。在这种情况下，时钟更新看起来会比正常情况下稍早发生
		此外32 位系统上4 位启动偏移量可能会看到部分更新。这些效应很罕见
		后处理应当能够处理它们。更多信息请参阅 ktime_get_boot_fast_ns() 函数
		中的注释

	tai:
		该时钟是 tai 时钟（CLOCK_TAI），派生自墙上时钟时间。但是，该时钟不
		经历NTP 插入闰秒所导致的间断和回跳。由于时钟访问是为跟踪使用而设计，
		可能会产生副作用。如果内TAI 偏移量被更新（例如由设置系统时间或使
		带偏移量adjtimex() 引起），时钟访问可能会产生错误的读数。这些效
		很罕见，后处理应当能够处理它们。更多信息请参阅 ktime_get_tai_fast_ns()
		函数中的注释

	要设置一个时钟，只需将时钟名 echo 到该文件

```
	  # echo global > trace_clock
```

	设置一个时钟会清除环形缓冲区的内容以及 "snapshot"（快照）缓冲区

  trace_marker:

	该文件对于将用户空间与内核中发生的事件同步非常有用。向该文件写入字符串
	被写ftrace 缓冲区

	在应用程序中，在应用程序启动时打开该文件并仅引用其文件描述符是很有用的

```
		void trace_write(const char *fmt, ...)
		{
			va_list ap;
			char buf[256];
			int n;

			if (trace_fd < 0)
				return;

			va_start(ap, fmt);
			n = vsnprintf(buf, 256, fmt, ap);
			va_end(ap);

			write(trace_fd, buf, n);
		}
```

	启动

```
		trace_fd = open("trace_marker", O_WRONLY);
```

	注意：写trace_marker 文件也可以触发写入到
	/sys/kernel/tracing/events/ftrace/print/trigger 的触发器。请参阅
	Documentation/trace/events.rst 中的 "Event triggers"（事件触发器）以
	Documentation/trace/histogram.rst（第 3 节）中的示例

  trace_marker_raw:

	该文件类似于上面trace_marker，但用于向其写入二进制数据，可以用工具从
	trace_pipe_raw 解析这些数据

  uprobe_events:

	在程序中添加动态跟踪点。请参阅 uprobetracer.rst

  uprobe_profile:

	Uprobe 统计信息。请参阅 uprobetrace.txt

  instances:

	这是一种创建多个跟踪缓冲区的方式，不同的事件可以记录在不同的缓冲区中
	请参阅下面的 "Instances"（实例）一节

  events:

	该文件是跟踪事件目录。它保存已编译进内核的事件跟踪点（也称为静态跟踪点）
	它显示存在哪些事件跟踪点，以及它们如何按系统分组。在不同层级"enable"
	文件，向它们写入 "1" 即可启用这些跟踪点

	更多信息请参events.rst

  set_event:

	通过向该文件 echo 事件名，将启用该事件

	更多信息请参events.rst

  show_event_filters:

	带有过滤器的事件列表。它显示系统/事件对以及挂接在该事件上的过滤器

	更多信息请参events.rst

  show_event_triggers:

	带有触发器的事件列表。它显示系统/事件对以及挂接在该事件上的触发器

	更多信息请参events.rst

  available_events:

	可以进行跟踪的可用事件列表

	更多信息请参events.rst

  timestamp_mode:

	某些跟踪器可能会改变将跟踪事件记录进事件缓冲区时所使用的时间戳模式。具
	不同模式的事件可以在同一缓冲区中共存，但在记录某个事件时生效的模式决定了
	该事件使用哪种时间戳模式。默认的时间戳模式是 'delta'

	跟踪常用的时间戳模式

```
	  # cat timestamp_mode
	  [delta] absolute
```

	  带有方括号的时间戳模式是正在生效的模式

	  delta: 默认时间戳模- 时间戳是相对于每个缓冲区时间戳的增量

	  absolute: 时间戳是完整时间戳，而不是相对于其他某个值的增量。因此它会占
	                更多空间，效率也较低

  hwlat_detector:

	硬件延迟探测器的目录。请参阅下面"Hardware Latency Detector"（硬件延
	探测器）一节

  per_cpu:

	该目录包per_cpu 的跟踪信息

  per_cpu/cpu0/buffer_size_kb:

	ftrace 缓冲区是per_cpu 定义的。也就是说，每个 CPU 都有一个独立的缓冲区，
	以便写入可以原子方式进行，并避免缓存抖动。这些缓冲区可能有不同的大小。该文件
	类似buffer_size_kb 文件，但它只显示或设置特CPU（此处为 cpu0）的缓冲区大小

  per_cpu/cpu0/trace:

	该文件类似于 "trace" 文件，但它只显示特定于该 CPU 的数据。如果向其写入，
	只清除特CPU 的缓冲区

  per_cpu/cpu0/trace_pipe

	该文件类似于 "trace_pipe" 文件，并且是一个消费型读取，但它只显示（并消费
	特定于该 CPU 的数据

  per_cpu/cpu0/trace_pipe_raw

	对于能够解析 ftrace 环形缓冲区二进制格式的工具，可以使用 trace_pipe_raw 文件
	直接从环形缓冲区提取数据。借助 splice() 系统调用，缓冲区数据可以快速传输到
	文件或网络，由服务器收集这些数据

	trace_pipe 一样，这是一个消费型读取器，多次读取总是会产生不同的数据

  per_cpu/cpu0/snapshot:

	该文件类似于"snapshot" 文件，但只会对当CPU 拍摄快照（如果支持）。它
	显示给定 CPU 的快照内容，如果向其写入，则只清除该 CPU 的缓冲区

  per_cpu/cpu0/snapshot_raw:

	类似trace_pipe_raw，但会从给定 CPU 的快照缓冲区读取二进制格式

  per_cpu/cpu0/stats:

	该文件显示有关环形缓冲区的某些统计信息：

	entries:
		缓冲区中仍然存在的事件数量

	overrun:
		由于缓冲区满而被覆盖所丢失的事件数量

	commit overrun:
		应始终为零。如果在嵌套事件（环形缓冲区是可重入的）中发生了太多事件
		导致缓冲区填满并开始丢弃事件，该值会被设置

	bytes:
		实际读取的字节数（未被覆盖）

	oldest event ts:
		缓冲区中最旧的时间

	now ts:
		当前时间

	dropped events:
		由于 overwrite 选项关闭而丢失的事件

	read events:
		已读取的事件数量

```
### 跟踪


以下是当前可以配置的跟踪器列表

  "function"

	用于跟踪所有内核函数的函数调用跟踪器

  "function_graph"

	类似于函数跟踪器，区别在于函数跟踪器在函数入口处探测，而函数图跟踪器在
	函数的入口和出口都进行跟踪。它还提供了绘制类似C 代码源码的函数调
	图的能力

	注意，函数图在内部为每个实例分别计算函数开始和返回时的时间。如果有两个
	实例运行函数图跟踪器并跟踪相同的函数，由于各自分别读取时间戳而非同时读取
	计时长度可能会有轻微偏差

  "blk"

	块设备跟踪器。blktrace 用户态应用程序所使用的跟踪器

  "hwlat"

	硬件延迟跟踪器，用于检测硬件是否产生任何延迟。请参阅下面"Hardware Latency
	Detector"（硬件延迟探测器）一节

  "irqsoff"

	跟踪禁用中断的区域，并保存具有最长最大延迟的跟踪。请参阅 tracing_max_latency
	当记录到新的最大值时，它会替换旧的跟踪。最好配latency-format 选项启用
	查看此跟踪，选择该跟踪器时会自动启用该选项

  "preemptoff"

	类似irqsoff，但跟踪并记录抢占被禁用的时间长度

  "preemptirqsoff"

	类似irqsoff preemptoff，但跟踪并记irq 或抢占被禁用的最长时间

  "wakeup"

	跟踪并记录最高优先级任务被唤醒后到它被调度所花费的最大延迟。按照普通开发者的
	预期跟踪所有任务

  "wakeup_rt"

	跟踪并记录仅RT 任务（如同当前的 "wakeup" 那样）被唤醒所花费的最大延迟。这
	对关RT 任务唤醒时间的人很有用

  "wakeup_dl"

	跟踪并记SCHED_DEADLINE 任务被唤醒（如同 "wakeup" "wakeup_rt" 那样）所
	花费的最大延迟

  "mmiotrace"

	一种用于跟踪二进制模块的特殊跟踪器。它会跟踪一个模块对硬件进行的所有调用，
	以及它从 I/O 读写的所有内容

  "branch"

	该跟踪器可以在跟踪内核中likely/unlikely 调用时配置。它会跟踪命中一
	likely unlikely 分支的时机，以及它对该分支预测的是否正确

  "nop"

	这是"什么都不跟的跟踪器。要移除所有跟踪器，只需current_tracer echo
	"nop" 即可

### 错误情形


  对于大多ftrace 命令，失败模式是显而易见的，并且使用标准返回码进行反馈

  对于其他更复杂的命令，可通过 tracing/error_log 文件获取扩展错误信息。对于支
  它的命令，在出错后读tracing/error_log 文件会显示有关出错原因的更详细信
  （如果有信息可用）。tracing/error_log 文件是一个循环错误日志，显示少量（当前为
  8 条）最近的 条）失败命令ftrace 错误

  扩展错误信息及用法采用以下形

```

    # echo xxx > /sys/kernel/tracing/events/sched/sched_wakeup/trigger
    echo: write error: Invalid argument

    # cat /sys/kernel/tracing/error_log
    [ 5348.887237] location: error: Couldn't yyy: zzz
      Command: xxx
               ^
    [ 7517.023364] location: error: Bad rrr: sss
      Command: ppp qqq
                   ^

  要清除错误日志，向它 echo 空字符串

```

    # echo > /sys/kernel/tracing/error_log

```
### 使用跟踪器的示例


以下是在仅使tracefs 接口（不使用任何用户态工具）控制跟踪器时的典型示例

### 输出格式


```

  # tracer: function
  #
  # entries-in-buffer/entries-written: 140080/250280   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
              bash-1977  [000] .... 17284.993652: sys_close <-system_call_fastpath
              bash-1977  [000] .... 17284.993653: __close_fd <-sys_close
              bash-1977  [000] .... 17284.993653: _raw_spin_lock <-__close_fd
              sshd-1974  [003] .... 17284.993653: __srcu_read_unlock <-fsnotify
              bash-1977  [000] .... 17284.993654: add_preempt_count <-_raw_spin_lock
              bash-1977  [000] ...1 17284.993655: _raw_spin_unlock <-__close_fd
              bash-1977  [000] ...1 17284.993656: sub_preempt_count <-_raw_spin_unlock
              bash-1977  [000] .... 17284.993657: filp_close <-__close_fd
              bash-1977  [000] .... 17284.993657: dnotify_flush <-filp_close
              sshd-1974  [003] .... 17284.993658: sys_select <-system_call_fastpath
              ....

```

会打印一个头部，其中包含由跟踪表示的跟踪器名。在本例中跟踪器"function"。然后它
显示缓冲区中的事件数量以及已写入的条目总数。两者的差值就是由于缓冲区填满而丢失的
条目数（250280 - 140080 = 110200 个事件丢失）

头部解释了事件的内容。任务名 "bash"、任PID "1977"、运行所在的 CPU "000"、延迟格
（如下解释）secs>.<usecs> 格式的时间戳、被跟踪的函数名 "sys_close" 以及调用该函数的
父函"system_call_fastpath"。时间戳是函数被进入的时间

### 延迟跟踪格式


当启用了 latency-format 选项，或者设置了某个延迟跟踪器时，trace 文件会提供更多信
以便查看

```

  # tracer: irqsoff
  #
  # irqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 259 us, #4/4, CPU#2 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: ps-6143 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: __lock_task_sighand
  #  => ended at:   _raw_spin_unlock_irqrestore
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
        ps-6143    2d...    0us!: trace_hardirqs_off <-__lock_task_sighand
        ps-6143    2d..1  259us+: trace_hardirqs_on <-_raw_spin_unlock_irqrestore
        ps-6143    2d..1  263us+: time_hardirqs_on <-_raw_spin_unlock_irqrestore
        ps-6143    2d..1  306us : <stack trace>
   => trace_hardirqs_on_caller
   => trace_hardirqs_on
   => _raw_spin_unlock_irqrestore
   => do_task_stat
   => proc_tgid_stat
   => proc_single_show
   => seq_read
   => vfs_read
   => sys_read
   => system_call_fastpath


```
这表明当前跟踪器"irqsoff"，正在跟踪中断被禁用的时间。它给出跟踪版本（从不改变）
以及其上执行的内核版本（3.8）。然后它显示最大延迟（以微秒为单位59 us）。显示的
跟踪条目数以及总数（两者都是四4/4）。VP、KP、SP HP 始终为零，保留供以后使用
#P 是在CPU 的数量（#P:4）

任务是延迟发生时正在运行的进程。（ps pid143）

导致延迟的开始和停止（分别是禁用和启用中断的函数）：

  - __lock_task_sighand 是禁用中断的位置
  - _raw_spin_unlock_irqrestore 是重新启用中断的位置

头部之后的几行是跟踪本身。头部解释了哪个是哪个

  cmd: 跟踪中进程的名字

  pid: 该进程的 PID

  CPU#: 该进程运行的 CPU

  irqs-off: 'd' 表示中断被禁用，否则'.'

  need-resched:
 - 'B' 表示 TIF_NEED_RESCHED、PREEMPT_NEED_RESCHED TIF_RESCHED_LAZY 都已设置
 - 'N' 表示 TIF_NEED_RESCHED PREEMPT_NEED_RESCHED 都已设置
 - 'n' TIF_NEED_RESCHED 被设置，
 - 'p' PREEMPT_NEED_RESCHED 被设置，
 - 'L' 表示 PREEMPT_NEED_RESCHED TIF_RESCHED_LAZY 都已设置
 - 'b' 表示 TIF_NEED_RESCHED TIF_RESCHED_LAZY 都已设置
 - 'l' TIF_RESCHED_LAZY 被设
 - '.' 否则

  hardirq/softirq:
 - 'Z' - 在硬中断内部发生NMI
 - 'z' - NMI 正在运行
 - 'H' - 在软中断内部发生了硬中断
 - 'h' - 硬中断正在运
 - 's' - 软中断正在运
 - '.' - 普通上下文

  preempt-depth: preempt_disabled 的层

  以上内容主要对内核开发者有意义

  time:
	当启用了 latency-format 选项时，trace 文件输出包含相对于跟踪开始的时间戳
	这与禁用 latency-format 时输出绝对时间戳不同

  delay:
	这仅为了更好地吸引你的眼球。它需要被修正为只相对于同一 CPU。这些标记由当前
	这条跟踪与下一条跟踪之间的差值决定

   - '$' - 澶т簬 1 绉。
   - '@' - 大于 100 毫秒
   - '*' - 大于 10 毫秒
   - '#' - 大于 1000 微秒
   - '!' - 大于 100 微秒
   - '+' - 大于 10 微秒
   - ' ' - 小于或等10 微秒

  其余部分'trace' 文件相同

  注意，延迟跟踪器通常以一个栈回溯结束，以便轻松找到延迟发生的位置

### trace_options


trace_options 文件（或 options 目录）用于控制跟踪输出中打印什么，或者操纵跟踪器

```

  cat trace_options
	print-parent
	nosym-offset
	nosym-addr
	noverbose
	noraw
	nohex
	nobin
	noblock
	nofields
	trace_printk
	annotate
	nouserstacktrace
	nosym-userobj
	noprintk-msg-only
	context-info
	nolatency-format
	record-cmd
	norecord-tgid
	overwrite
	nodisable_on_free
	irq-info
	markers
	noevent-fork
	function-trace
	nofunction-fork
	nodisplay-graph
	nostacktrace
	nobranch

```
要禁用其中一个选项，向其中 echo 带有前缀的选项

```

  echo noprint-parent > trace_options

```

```

  echo sym-offset > trace_options

```
以下是可用选项

  print-parent
	在函数跟踪中，显示被调用（父）函数以及正在被跟踪的函数

```

	  print-parent:
	   bash-4000  [01]  1477.606694: simple_strtoul <-kstrtoul

	  noprint-parent:
	   bash-4000  [01]  1477.606694: simple_strtoul


  sym-offset
	不仅显示函数名，还显示函数内的偏移量。例如，你看到的不再"ktime_get"
	而是 "ktime_get+0xb/0x20"

```

	  sym-offset:
	   bash-4000  [01]  1477.606694: simple_strtoul+0x6/0xa0

  sym-addr
	这还会显示函数地址以及函数名

```

	  sym-addr:
	   bash-4000  [01]  1477.606694: simple_strtoul <c0339346>

  verbose
	该选项处理

```
        latency-format 选项启用时的 trace 文件

```

	    bash  4000 1 0 00000000 00010a95 [58127d26] 1720.415ms \
	    (+0.000ms): simple_strtoul (kstrtoul)

  raw
	该选项显示原始数字。此选项最适合与能够更好地翻译原始数字的用户态应用程
	配合使用，而不是由内核来完成

  hex
	类似raw，但数字采用十六进制格式

  bin
	该选项以原始二进制格式打印输出

  block
	设置后，轮询读取 trace_pipe 时不会阻塞

  fields
	按其类型所述打印字段。这比使hex、bin raw 更好，因为它能更好地解析
	事件的内容

  trace_printk
	可以禁止 trace_printk() 写入缓冲区

  trace_printk_dest
	设置后让 trace_printk() 及类似的内部跟踪函数写入此实例。注意，只有一个跟
	实例可以设置此标志。设置此标志会清除之前设置了该标志的实例trace_printk_dest
	标志。默认情况下，顶层跟踪具有此设置，如果另一个实例设置了它然后又清除它，
	顶层跟踪会重新获得该设置

	此标志不能被顶层实例清除，因为它是默认实例。顶层实例清除此标志的唯一方式
	是由另一个实例设置它

  copy_trace_marker
	如果有应用程序硬编码写入顶层 trace_marker 文件sys/kernel/tracing/trace_marker
	trace_marker_raw），而工具希望将其转到某个实例，则可以使用此选项。创建一
	实例并设置此选项，之后所有对顶层 trace_marker 文件的写入也都会被重定向到该
	实例

	注意，默认情况下顶层实例设置了此选项。如果它被禁用，那么trace_marker 
	trace_marker_raw 文件的写入将不会被写入顶层文件。如果没有任何实例设置此选项
	则写入将ENODEV 错误码失败

  annotate
	CPU 缓冲区已满，且某CPU 缓冲区最近有大量事件（因而时间窗口较短），
	另一CPU 可能只有少量事件（从而可以保留较旧的事件）时，情况有时会令人困惑
	当报告跟踪时，它先显示最旧的事件，并且可能看起来好像只有运行时间最长的那个
	CPU（拥有最旧事件的那个）在运行。当设置annotate 选项时，它会显示一个新
	CPU 缓冲区何时开始：

```
			  <idle>-0     [001] dNs4 21169.031481: wake_up_idle_cpu <-add_timer_on
			  <idle>-0     [001] dNs4 21169.031482: _raw_spin_unlock_irqrestore <-add_timer_on
			  <idle>-0     [001] .Ns4 21169.031484: sub_preempt_count <-_raw_spin_unlock_irqrestore
		##### CPU 2 buffer started ####
			  <idle>-0     [002] .N.1 21169.031484: rcu_idle_exit <-cpu_idle
			  <idle>-0     [001] .Ns3 21169.031484: _raw_spin_unlock <-clocksource_watchdog
			  <idle>-0     [001] .Ns3 21169.031485: sub_preempt_count <-_raw_spin_unlock
```

  userstacktrace
	该选项会改变跟踪。它在每次跟踪事件之后记录当前用户空间线程的栈回溯

  sym-userobj
	当启用了用户栈回溯时，查找该地址属于哪个对象，并打印相对地址。当开启了 ASLR
	时这尤其有用，否则在应用不再运行后，你无法将地址解析为对文件/行

	查找在你读取 trace、trace_pipe 时执行。示例：

```

		  a.out-1623  [000] 40874.465068: /root/a.out[+0x480] <-/root/a.out[+0
		  x494] <- /root/a.out[+0x4a8] <- /lib/libc-2.7.so[+0x1e1a6]


  printk-msg-only
	设置后，trace_printk() 将只显示格式而不显示其参数（如果使用trace_bprintk()
	trace_bputs() 来保trace_printk()）

  context-info
	只显示事件数据。隐comm、PID、时间戳、CPU 和其他有用数据

  latency-format
	该选项会改变跟踪输出。启用时，跟踪会显示有关延迟的附加信息，"Latency trace
	format"（延迟跟踪格式）中所述

  pause-on-trace
	设置后，为读取而打开 trace 文件会暂停向环形缓冲区的写入（如tracing_on 
	设置0）。这模拟trace 文件最初的行为。当文件关闭时，跟踪会重新启用

  hash-ptr
        设置后，事件 printk 格式中的 "%p" 显示哈希后的指针值而不是真实地址。如果你
	想查明跟踪日志中哪个哈希值对应于真实值，这会很有用

  bitmask-list
        启用后，位掩码使printk "%*pbl" 格式说明符显示为可读的范围列表（例如
	0,2-5,7）。禁用时（默认），位掩码以传统的十六进制位图表示形式显示。列表格
	对于跟踪 CPU 掩码和其他大型位掩码特别有用，其中单个位的位置比其十六进制编
	更有意义

  record-cmd
	当启用任何事件或跟踪器时，会sched_switch 跟踪点中启用一个钩子，以填充映射了
	pid comm comm 缓存。但这可能会带来一些开销，如果你只关pid 而不关心任务
	名，禁用此选项可以降低跟踪的影响。请参阅 "saved_cmdlines"

  record-tgid
	当启用任何事件或跟踪器时，会sched_switch 跟踪点中启用一个钩子，以填充映射了
	线程ID（TGID）到 pid 的缓存。请参阅 "saved_tgids"

  overwrite
	该选项控制当跟踪缓冲区已满时发生的情况。如果为 "1"（默认），最旧的事件会被
	丢弃并覆盖。如果为 "0"，则丢弃最新的事件
	（见 per_cpu/cpu0/stats 中的 overrun dropped

  disable_on_free
	free_buffer 关闭时，跟踪会停止（tracing_on 被设置为 0）

  irq-info
	显示中断、抢占计数、need resched 数据。禁用时，跟踪看起来如下

```

		# tracer: function
		#
		# entries-in-buffer/entries-written: 144405/9452052   #P:4
		#
		#           TASK-PID   CPU#      TIMESTAMP  FUNCTION
		#              | |       |          |         |
			  <idle>-0     [002]  23636.756054: ttwu_do_activate.constprop.89 <-try_to_wake_up
			  <idle>-0     [002]  23636.756054: activate_task <-ttwu_do_activate.constprop.89
			  <idle>-0     [002]  23636.756055: enqueue_task <-activate_task


  markers
	设置后，trace_marker 可写（仅 root）。禁用时，对 trace_marker 的写入将EINVAL
	错误

  event-fork
	设置后，列在 set_event_pid 中的 PID 的任务在 fork 时，其子进程PID 会被添加
	set_event_pid。同样，当列set_event_pid 中的 PID 的任务退出时，其 PID 会从
	文件中移除

        这同样会影响列在 set_event_notrace_pid 中的 PID

  function-trace
	如果启用了此选项（默认启用），延迟跟踪器将启用函数跟踪。禁用时，延迟跟踪器
	不会跟踪函数。这在进行延迟测试时降低了跟踪器的开销

  function-fork
	设置后，列在 set_ftrace_pid 中的 PID 的任务在 fork 时，其子进程PID 会被添加
	set_ftrace_pid。同样，当列set_ftrace_pid 中的 PID 的任务退出时，其 PID 
	从该文件中移除

        这同样会影响列在 set_ftrace_notrace_pid 中的 PID

  display-graph
	设置后，延迟跟踪器（irqsoff、wakeup 等）将使用函数图跟踪而不是函数跟踪

  stacktrace
	设置后，在记录任何跟踪事件后会记录一条栈回溯

  branch
	用跟踪器启用分支跟踪。这会启用分支跟踪器以及当前设置的跟踪器。用 "nop" 跟踪
	启用此选项等同于仅启用 "branch" 跟踪器

```

       file when the tracer is active. They always appear in the
       options directory.


以下是各跟踪器的选项

函数跟踪器的选项

  func_stack_trace
	设置后，在每次记录的函数之后都会记录一条栈回溯。注意！在启用此选项之前，先
	"set_ftrace_filter" 限制被记录的函数，否则系统性能会严重下降。记得在清除函数
	过滤器之前禁用此选项

函数图跟踪器的选项

  由于函数图跟踪器的输出略有不同，它有自己的一组选项来控制显示内容

  funcgraph-overrun
	设置后，在每次被跟踪的函数之后会显示图栈溢出"。溢出是指调用栈深度大于为每
	任务保留的深度。每个任务都有一个固定大小的函数数组用于调用图跟踪。如果调
	深度超过该数组，该函数就不会被跟踪。溢出就是由于超出该数组而错过的函数数量

  funcgraph-cpu
	设置后，显示发生跟踪CPU CPU 编号

  funcgraph-overhead
	设置后，如果函数花费的时间超过一定量，则会显示一个延迟标记。见上面头部描述
	下的 "delay"

  funcgraph-proc
	与其他跟踪器不同，进程的命令行默认不显示，而是仅在上下文切换期间任务被跟踪
	进入和退出时才显示。启用此选项会让每个进程的命令显示在每个行上

  funcgraph-duration
	在每个函数结束时（返回时），显示该函数中花费的时间长度（以微秒为单位）

  funcgraph-abstime
	设置后，每行都会显示时间戳

  funcgraph-irqs
	禁用后，发生在中断内部的函数不会被跟踪

  funcgraph-tail
	设置后，返回事件会包含它所代表的函数。默认情况下关闭，只为函数返回显示一
	闭合花括"}"

  funcgraph-retval
	设置后，每个被跟踪函数的返回值会打印在等"=" 之后。默认情况下关闭

  funcgraph-retval-hex
	设置后，返回值将始终以十六进制格式打印。如果未设置该选项且返回值是错误码，
	会以有符号十进制格式打印；否则也会以十六进制格式打印。默认情况下该选项关闭

  sleep-time
	运行函数图跟踪器时，将任务调出（schedule out）的时间包含进其函数中。启用时
	它会将任务被调出的时间计入函数调用的一部分

  graph-time
	配合函数图跟踪器运行函数性能分析器时，将调用嵌套函数的时间包含在内。未设置
	时，报告的该函数时间只包含该函数自身执行的时间，而不包含它调用的函数的时间

块设备跟踪器的选项

  blk_classic
	显示更精简的输出


### irqsoff


当中断被禁用时，CPU 无法对任何其他外部事件（除了 NMI SMI）做出反应。这会阻
定时器中断触发，或阻止鼠标中断告知内核有新的鼠标事件。结果是反应时间上的延迟

irqsoff 跟踪器跟踪中断被禁用的时间。当达到一个新的最大延迟时，跟踪器会保存导致该
延迟点的跟踪，这样每当达到一个新的最大值，旧的已保存跟踪就会被丢弃，新的跟踪被保存

要重置最大值，tracing_max_latency echo 0。以下是

```

  # echo 0 > options/function-trace
  # echo irqsoff > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # ls -ltr
  [...]
  # echo 0 > tracing_on
  # cat trace
  # tracer: irqsoff
  #
  # irqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 16 us, #4/4, CPU#0 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: swapper/0-0 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: run_timer_softirq
  #  => ended at:   run_timer_softirq
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       0d.s2    0us+: _raw_spin_lock_irq <-run_timer_softirq
    <idle>-0       0dNs3   17us : _raw_spin_unlock_irq <-run_timer_softirq
    <idle>-0       0dNs3   17us+: trace_hardirqs_on <-run_timer_softirq
    <idle>-0       0dNs3   25us : <stack trace>
   => _raw_spin_unlock_irq
   => run_timer_softirq
   => __do_softirq
   => call_softirq
   => do_softirq
   => irq_exit
   => smp_apic_timer_interrupt
   => apic_timer_interrupt
   => rcu_idle_exit
   => cpu_idle
   => rest_init
   => start_kernel
   => x86_64_start_reservations
   => x86_64_start_kernel

```
这里我们看到延迟16 微秒（非常好）。run_timer_softirq 中的 _raw_spin_lock_irq 禁用
中断。显示的 16 与显示的时间25us 之间的差异，是因为记录最大延迟的时间与记录具
该延迟的函数的时间之间，时钟被增加了

注意上面的示例未设置 function-trace。如果我们设

```

 with echo 1 > options/function-trace

  # tracer: irqsoff
  #
  # irqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 71 us, #168/168, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: bash-2042 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: ata_scsi_queuecmd
  #  => ended at:   ata_scsi_queuecmd
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
      bash-2042    3d...    0us : _raw_spin_lock_irqsave <-ata_scsi_queuecmd
      bash-2042    3d...    0us : add_preempt_count <-_raw_spin_lock_irqsave
      bash-2042    3d..1    1us : ata_scsi_find_dev <-ata_scsi_queuecmd
      bash-2042    3d..1    1us : __ata_scsi_find_dev <-ata_scsi_find_dev
      bash-2042    3d..1    2us : ata_find_dev.part.14 <-__ata_scsi_find_dev
      bash-2042    3d..1    2us : ata_qc_new_init <-__ata_scsi_queuecmd
      bash-2042    3d..1    3us : ata_sg_init <-__ata_scsi_queuecmd
      bash-2042    3d..1    4us : ata_scsi_rw_xlat <-__ata_scsi_queuecmd
      bash-2042    3d..1    4us : ata_build_rw_tf <-ata_scsi_rw_xlat
  [...]
      bash-2042    3d..1   67us : delay_tsc <-__delay
      bash-2042    3d..1   67us : add_preempt_count <-delay_tsc
      bash-2042    3d..2   67us : sub_preempt_count <-delay_tsc
      bash-2042    3d..1   67us : add_preempt_count <-delay_tsc
      bash-2042    3d..2   68us : sub_preempt_count <-delay_tsc
      bash-2042    3d..1   68us+: ata_bmdma_start <-ata_bmdma_qc_issue
      bash-2042    3d..1   71us : _raw_spin_unlock_irqrestore <-ata_scsi_queuecmd
      bash-2042    3d..1   71us : _raw_spin_unlock_irqrestore <-ata_scsi_queuecmd
      bash-2042    3d..1   72us+: trace_hardirqs_on <-ata_scsi_queuecmd
      bash-2042    3d..1  120us : <stack trace>
   => _raw_spin_unlock_irqrestore
   => ata_scsi_queuecmd
   => scsi_dispatch_cmd
   => scsi_request_fn
   => __blk_run_queue_uncond
   => __blk_run_queue
   => blk_queue_bio
   => submit_bio_noacct
   => submit_bio
   => submit_bh
   => __ext3_get_inode_loc
   => ext3_iget
   => ext3_lookup
   => lookup_real
   => __lookup_hash
   => walk_component
   => lookup_last
   => path_lookupat
   => filename_lookup
   => user_path_at_empty
   => user_path_at
   => vfs_fstatat
   => vfs_stat
   => sys_newstat
   => system_call_fastpath


```
这里我们跟踪了一71 微秒的延迟。但我们也看到了在此期间被调用的所有函数。注意，通过
启用函数跟踪，我们带来了额外的开销。这个开销可能会延长延迟时间。但尽管如此，此跟踪
提供了一些非常有帮助的调试信息

如果我们偏好函数图输出而非函数输出，可以设

```

 with echo 1 > options/display-graph

  # tracer: irqsoff
  #
  # irqsoff latency trace v1.1.5 on 4.20.0-rc6+
  # --------------------------------------------------------------------
  # latency: 3751 us, #274/274, CPU#0 | (M:desktop VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: bash-1507 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: free_debug_processing
  #  => ended at:   return_to_handler
  #
  #
  #                                       _-----=> irqs-off
  #                                      / _----=> need-resched
  #                                     | / _---=> hardirq/softirq
  #                                     || / _--=> preempt-depth
  #                                     ||| /
  #   REL TIME      CPU  TASK/PID       ||||     DURATION                  FUNCTION CALLS
  #      |          |     |    |        ||||      |   |                     |   |   |   |
          0 us |   0)   bash-1507    |  d... |   0.000 us    |  _raw_spin_lock_irqsave();
          0 us |   0)   bash-1507    |  d..1 |   0.378 us    |    do_raw_spin_trylock();
          1 us |   0)   bash-1507    |  d..2 |               |    set_track() {
          2 us |   0)   bash-1507    |  d..2 |               |      save_stack_trace() {
          2 us |   0)   bash-1507    |  d..2 |               |        __save_stack_trace() {
          3 us |   0)   bash-1507    |  d..2 |               |          __unwind_start() {
          3 us |   0)   bash-1507    |  d..2 |               |            get_stack_info() {
          3 us |   0)   bash-1507    |  d..2 |   0.351 us    |              in_task_stack();
          4 us |   0)   bash-1507    |  d..2 |   1.107 us    |            }
  [...]
       3750 us |   0)   bash-1507    |  d..1 |   0.516 us    |      do_raw_spin_unlock();
       3750 us |   0)   bash-1507    |  d..1 |   0.000 us    |  _raw_spin_unlock_irqrestore();
       3764 us |   0)   bash-1507    |  d..1 |   0.000 us    |  tracer_hardirqs_on();
      bash-1507    0d..1 3792us : <stack trace>
   => free_debug_processing
   => __slab_free
   => kmem_cache_free
   => vm_area_free
   => remove_vma
   => exit_mmap
   => mmput
   => begin_new_exec
   => load_elf_binary
   => search_binary_handler
   => __do_execve_file.isra.32
   => __x64_sys_execve
   => do_syscall_64
   => entry_SYSCALL_64_after_hwframe


```
### preemptoff


当抢占被禁用时，我们可能能接收中断，但任务无法被抢占，更高优先级的任务必须等待抢
被重新启用后才能抢占较低优先级的任务

preemptoff 跟踪器跟踪禁用抢占的位置。与 irqsoff 跟踪器类似，它记录抢占被禁用的最
延迟。preemptoff 跟踪器的控制方式irqsoff 跟踪器非常相似

```

  # echo 0 > options/function-trace
  # echo preemptoff > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # ls -ltr
  [...]
  # echo 0 > tracing_on
  # cat trace
  # tracer: preemptoff
  #
  # preemptoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 46 us, #4/4, CPU#1 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: sshd-1991 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: do_IRQ
  #  => ended at:   do_IRQ
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
      sshd-1991    1d.h.    0us+: irq_enter <-do_IRQ
      sshd-1991    1d..1   46us : irq_exit <-do_IRQ
      sshd-1991    1d..1   47us+: trace_preempt_on <-do_IRQ
      sshd-1991    1d..1   52us : <stack trace>
   => sub_preempt_count
   => irq_exit
   => do_IRQ
   => ret_from_intr


```
这有一些更多的变化。当进入中断时（注意 'h'），抢占被禁用，并在退出时启用。但我们
看到在进入抢占禁用段和离开它时中断已被禁用d'）。我们不知道在此期间或此后不久中
是否被重新启用

```

  # tracer: preemptoff
  #
  # preemptoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 83 us, #241/241, CPU#1 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: bash-1994 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: wake_up_new_task
  #  => ended at:   task_rq_unlock
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
      bash-1994    1d..1    0us : _raw_spin_lock_irqsave <-wake_up_new_task
      bash-1994    1d..1    0us : select_task_rq_fair <-select_task_rq
      bash-1994    1d..1    1us : __rcu_read_lock <-select_task_rq_fair
      bash-1994    1d..1    1us : source_load <-select_task_rq_fair
      bash-1994    1d..1    1us : source_load <-select_task_rq_fair
  [...]
      bash-1994    1d..1   12us : irq_enter <-smp_apic_timer_interrupt
      bash-1994    1d..1   12us : rcu_irq_enter <-irq_enter
      bash-1994    1d..1   13us : add_preempt_count <-irq_enter
      bash-1994    1d.h1   13us : exit_idle <-smp_apic_timer_interrupt
      bash-1994    1d.h1   13us : hrtimer_interrupt <-smp_apic_timer_interrupt
      bash-1994    1d.h1   14us : _raw_spin_lock <-hrtimer_interrupt
      bash-1994    1d.h1   14us : add_preempt_count <-_raw_spin_lock
      bash-1994    1d.h2   14us : ktime_get_update_offsets <-hrtimer_interrupt
  [...]
      bash-1994    1d.h1   35us : lapic_next_event <-clockevents_program_event
      bash-1994    1d.h1   35us : irq_exit <-smp_apic_timer_interrupt
      bash-1994    1d.h1   36us : sub_preempt_count <-irq_exit
      bash-1994    1d..2   36us : do_softirq <-irq_exit
      bash-1994    1d..2   36us : __do_softirq <-call_softirq
      bash-1994    1d..2   36us : __local_bh_disable <-__do_softirq
      bash-1994    1d.s2   37us : add_preempt_count <-_raw_spin_lock_irq
      bash-1994    1d.s3   38us : _raw_spin_unlock <-run_timer_softirq
      bash-1994    1d.s3   39us : sub_preempt_count <-_raw_spin_unlock
      bash-1994    1d.s2   39us : call_timer_fn <-run_timer_softirq
  [...]
      bash-1994    1dNs2   81us : cpu_needs_another_gp <-rcu_process_callbacks
      bash-1994    1dNs2   82us : __local_bh_enable <-__do_softirq
      bash-1994    1dNs2   82us : sub_preempt_count <-__local_bh_enable
      bash-1994    1dN.2   82us : idle_cpu <-irq_exit
      bash-1994    1dN.2   83us : rcu_irq_exit <-irq_exit
      bash-1994    1dN.2   83us : sub_preempt_count <-irq_exit
      bash-1994    1.N.1   84us : _raw_spin_unlock_irqrestore <-task_rq_unlock
      bash-1994    1.N.1   84us+: trace_preempt_on <-task_rq_unlock
      bash-1994    1.N.1  104us : <stack trace>
   => sub_preempt_count
   => _raw_spin_unlock_irqrestore
   => task_rq_unlock
   => wake_up_new_task
   => do_fork
   => sys_clone
   => stub_clone


```
上面是设置了 function-trace preemptoff 跟踪示例。这里我们看到中断并非在整个期间
被禁用。irq_enter 代码让我们知道我们进入了一个中'h'。在此之前，被跟踪的函数仍然
显示它不在中断中，但我们从函数本身可以看出情况并非如此

### preemptirqsoff


了解中断被禁用或抢占被禁用时间最长的位置很有帮助。但有时我们想知道抢占和/或中
何时被禁用

```

    local_irq_disable();
    call_function_with_irqs_off();
    preempt_disable();
    call_function_with_irqs_and_preemption_off();
    local_irq_enable();
    call_function_with_preemption_off();
    preempt_enable();

```
irqsoff 跟踪器会记录 call_function_with_irqs_off() 
call_function_with_irqs_and_preemption_off() 的总长度

preemptoff 跟踪器会记录 call_function_with_irqs_and_preemption_off() 
call_function_with_preemption_off() 的总长度

但二者都不会跟踪中断或抢占被禁用的时间。这个总时间是我们无法调度的时长。要记录
此时长，请使preemptirqsoff 跟踪器

同样，使用此跟踪irqsoff preemptoff 跟踪器非常相似

```

  # echo 0 > options/function-trace
  # echo preemptirqsoff > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # ls -ltr
  [...]
  # echo 0 > tracing_on
  # cat trace
  # tracer: preemptirqsoff
  #
  # preemptirqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 100 us, #4/4, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: ls-2230 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: ata_scsi_queuecmd
  #  => ended at:   ata_scsi_queuecmd
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
        ls-2230    3d...    0us+: _raw_spin_lock_irqsave <-ata_scsi_queuecmd
        ls-2230    3...1  100us : _raw_spin_unlock_irqrestore <-ata_scsi_queuecmd
        ls-2230    3...1  101us+: trace_preempt_on <-ata_scsi_queuecmd
        ls-2230    3...1  111us : <stack trace>
   => sub_preempt_count
   => _raw_spin_unlock_irqrestore
   => ata_scsi_queuecmd
   => scsi_dispatch_cmd
   => scsi_request_fn
   => __blk_run_queue_uncond
   => __blk_run_queue
   => blk_queue_bio
   => submit_bio_noacct
   => submit_bio
   => submit_bh
   => ext3_bread
   => ext3_dir_bread
   => htree_dirblock_to_tree
   => ext3_htree_fill_tree
   => ext3_readdir
   => vfs_readdir
   => sys_getdents
   => system_call_fastpath


```
trace_hardirqs_off_thunk x86 上从中断被禁用时由汇编代码调用。没有函数跟踪，我们
无法知道在抢占点内部中断是否被启用。我们确实看到它从启用抢占开始

```

  # tracer: preemptirqsoff
  #
  # preemptirqsoff latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 161 us, #339/339, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: ls-2269 (uid:0 nice:0 policy:0 rt_prio:0)
  #    -----------------
  #  => started at: schedule
  #  => ended at:   mutex_unlock
  #
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
  kworker/-59      3...1    0us : __schedule <-schedule
  kworker/-59      3d..1    0us : rcu_preempt_qs <-rcu_note_context_switch
  kworker/-59      3d..1    1us : add_preempt_count <-_raw_spin_lock_irq
  kworker/-59      3d..2    1us : deactivate_task <-__schedule
  kworker/-59      3d..2    1us : dequeue_task <-deactivate_task
  kworker/-59      3d..2    2us : update_rq_clock <-dequeue_task
  kworker/-59      3d..2    2us : dequeue_task_fair <-dequeue_task
  kworker/-59      3d..2    2us : update_curr <-dequeue_task_fair
  kworker/-59      3d..2    2us : update_min_vruntime <-update_curr
  kworker/-59      3d..2    3us : cpuacct_charge <-update_curr
  kworker/-59      3d..2    3us : __rcu_read_lock <-cpuacct_charge
  kworker/-59      3d..2    3us : __rcu_read_unlock <-cpuacct_charge
  kworker/-59      3d..2    3us : update_cfs_rq_blocked_load <-dequeue_task_fair
  kworker/-59      3d..2    4us : clear_buddies <-dequeue_task_fair
  kworker/-59      3d..2    4us : account_entity_dequeue <-dequeue_task_fair
  kworker/-59      3d..2    4us : update_min_vruntime <-dequeue_task_fair
  kworker/-59      3d..2    4us : update_cfs_shares <-dequeue_task_fair
  kworker/-59      3d..2    5us : hrtick_update <-dequeue_task_fair
  kworker/-59      3d..2    5us : wq_worker_sleeping <-__schedule
  kworker/-59      3d..2    5us : kthread_data <-wq_worker_sleeping
  kworker/-59      3d..2    5us : put_prev_task_fair <-__schedule
  kworker/-59      3d..2    6us : pick_next_task_fair <-pick_next_task
  kworker/-59      3d..2    6us : clear_buddies <-pick_next_task_fair
  kworker/-59      3d..2    6us : set_next_entity <-pick_next_task_fair
  kworker/-59      3d..2    6us : update_stats_wait_end <-set_next_entity
        ls-2269    3d..2    7us : finish_task_switch <-__schedule
        ls-2269    3d..2    7us : _raw_spin_unlock_irq <-finish_task_switch
        ls-2269    3d..2    8us : do_IRQ <-ret_from_intr
        ls-2269    3d..2    8us : irq_enter <-do_IRQ
        ls-2269    3d..2    8us : rcu_irq_enter <-irq_enter
        ls-2269    3d..2    9us : add_preempt_count <-irq_enter
        ls-2269    3d.h2    9us : exit_idle <-do_IRQ
  [...]
        ls-2269    3d.h3   20us : sub_preempt_count <-_raw_spin_unlock
        ls-2269    3d.h2   20us : irq_exit <-do_IRQ
        ls-2269    3d.h2   21us : sub_preempt_count <-irq_exit
        ls-2269    3d..3   21us : do_softirq <-irq_exit
        ls-2269    3d..3   21us : __do_softirq <-call_softirq
        ls-2269    3d..3   21us+: __local_bh_disable <-__do_softirq
        ls-2269    3d.s4   29us : sub_preempt_count <-_local_bh_enable_ip
        ls-2269    3d.s5   29us : sub_preempt_count <-_local_bh_enable_ip
        ls-2269    3d.s5   31us : do_IRQ <-ret_from_intr
        ls-2269    3d.s5   31us : irq_enter <-do_IRQ
        ls-2269    3d.s5   31us : rcu_irq_enter <-irq_enter
  [...]
        ls-2269    3d.s5   31us : rcu_irq_enter <-irq_enter
        ls-2269    3d.s5   32us : add_preempt_count <-irq_enter
        ls-2269    3d.H5   32us : exit_idle <-do_IRQ
        ls-2269    3d.H5   32us : handle_irq <-do_IRQ
        ls-2269    3d.H5   32us : irq_to_desc <-handle_irq
        ls-2269    3d.H5   33us : handle_fasteoi_irq <-handle_irq
  [...]
        ls-2269    3d.s5  158us : _raw_spin_unlock_irqrestore <-rtl8139_poll
        ls-2269    3d.s3  158us : net_rps_action_and_irq_enable.isra.65 <-net_rx_action
        ls-2269    3d.s3  159us : __local_bh_enable <-__do_softirq
        ls-2269    3d.s3  159us : sub_preempt_count <-__local_bh_enable
        ls-2269    3d..3  159us : idle_cpu <-irq_exit
        ls-2269    3d..3  159us : rcu_irq_exit <-irq_exit
        ls-2269    3d..3  160us : sub_preempt_count <-irq_exit
        ls-2269    3d...  161us : __mutex_unlock_slowpath <-mutex_unlock
        ls-2269    3d...  162us+: trace_hardirqs_on <-mutex_unlock
        ls-2269    3d...  186us : <stack trace>
   => __mutex_unlock_slowpath
   => mutex_unlock
   => process_output
   => n_tty_write
   => tty_write
   => vfs_write
   => sys_write
   => system_call_fastpath


```
这是一次有趣的跟踪。它kworker 运行并调度出去、ls 接管开始。但一ls 释放rq 
并启用了中断（但未启用抢占），一个中断就被触发了。当中断结束时，它开始运行软中断
但在软中断运行期间，另一个中断被触发了。当中断在软中断内部运行时，标记'H'


### wakeup


人们感兴趣的一种常见情况是，一个被唤醒的任务真正被唤醒所花费的时间。对于非实时任务
这可能是任意的。但无论如何跟踪它都很有趣

```

  # echo 0 > options/function-trace
  # echo wakeup > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # chrt -f 5 sleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: wakeup
  #
  # wakeup latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 15 us, #4/4, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: kworker/3:1H-312 (uid:0 nice:-20 policy:0 rt_prio:0)
  #    -----------------
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       3dNs7    0us :      0:120:R   + [003]   312:100:R kworker/3:1H
    <idle>-0       3dNs7    1us+: ttwu_do_activate.constprop.87 <-try_to_wake_up
    <idle>-0       3d..3   15us : __schedule <-schedule
    <idle>-0       3d..3   15us :      0:120:R ==> [003]   312:100:R kworker/3:1H


```
跟踪器只跟踪系统中最高优先级的任务，以避免跟踪正常情况。这里我们看nice 优先级为
-20（非常不友好）的 kworker，从它被唤醒到它运行，只花了 15 微秒

非实时任务没那么有趣。更有趣的跟踪是只关注实时任务

### wakeup_rt


在实时环境中，了解被唤醒的最高优先级任务从被唤醒到它执行所花费的唤醒时间非
重要。这也被称为"调度延迟"。我强调一点，这是关于 RT 任务的。了解非 RT 任务的调
延迟也很重要，但对于RT 任务，平均调度延迟更合适。像 LatencyTop 这样的工具更适合
此类测量

实时环境关注最坏情况延迟。也就是某件事发生所需的最长时间，而不是平均时间。我们可
有一个非常快的调度器，它可能只是偶尔才出现一次大延迟，但这对实时任务来说并不合适
wakeup_rt 跟踪器就是为记录 RT 任务的最坏情况唤醒而设计的。非 RT 任务不会被记录，因为
该跟踪器只记录一个最坏情况，跟踪不可预测的非 RT 任务会覆盖掉 RT 任务的最坏情况延
（只需运行一段时间的普wakeup 跟踪器就能看到这种效果）

由于该跟踪器只处RT 任务，我们将以与之前跟踪器略有不同的方式运行它。不再执'ls'
而是'chrt' 下运'sleep 1'，这会更改任务的优先级

```

  # echo 0 > options/function-trace
  # echo wakeup_rt > current_tracer
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # chrt -f 5 sleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: wakeup
  #
  # tracer: wakeup_rt
  #
  # wakeup_rt latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 5 us, #4/4, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: sleep-2389 (uid:0 nice:0 policy:1 rt_prio:5)
  #    -----------------
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       3d.h4    0us :      0:120:R   + [003]  2389: 94:R sleep
    <idle>-0       3d.h4    1us+: ttwu_do_activate.constprop.87 <-try_to_wake_up
    <idle>-0       3d..3    5us : __schedule <-schedule
    <idle>-0       3d..3    5us :      0:120:R ==> [003]  2389: 94:R sleep


```
在一个空闲系统上运行，我们看到执行任务切换只花了 5 微秒。注意，由于 schedule 中的
跟踪点位于实切换"之前，我们在被记录的任务即将调度进来时停止跟踪。如果我们在一
调度器末尾添加一个新的标记，这可能会改变

注意记录的任'sleep' PID 2389，它rt_prio 5。该优先级是用户空间优先级，
而不是内核内部优先级。policy 1 表示 SCHED_FIFO 表示 SCHED_RR

注意，跟踪数据显示的是内部优先级9 - rtprio）

```

  <idle>-0       3d..3    5us :      0:120:R ==> [003]  2389: 94:R sleep

```
0:120:R 表示 idle nice 优先020 - 120）运行，并处于运行'R'。sleep 任务
2389: 94:R 被调度进来。也就是说优先级是内rtprio9 - 5 = 94），它也处于运行态

chrt -r 5 并设function-trace 做同样的事

```

  echo 1 > options/function-trace

  # tracer: wakeup_rt
  #
  # wakeup_rt latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 29 us, #85/85, CPU#3 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: sleep-2448 (uid:0 nice:0 policy:1 rt_prio:5)
  #    -----------------
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       3d.h4    1us+:      0:120:R   + [003]  2448: 94:R sleep
    <idle>-0       3d.h4    2us : ttwu_do_activate.constprop.87 <-try_to_wake_up
    <idle>-0       3d.h3    3us : check_preempt_curr <-ttwu_do_wakeup
    <idle>-0       3d.h3    3us : resched_curr <-check_preempt_curr
    <idle>-0       3dNh3    4us : task_woken_rt <-ttwu_do_wakeup
    <idle>-0       3dNh3    4us : _raw_spin_unlock <-try_to_wake_up
    <idle>-0       3dNh3    4us : sub_preempt_count <-_raw_spin_unlock
    <idle>-0       3dNh2    5us : ttwu_stat <-try_to_wake_up
    <idle>-0       3dNh2    5us : _raw_spin_unlock_irqrestore <-try_to_wake_up
    <idle>-0       3dNh2    6us : sub_preempt_count <-_raw_spin_unlock_irqrestore
    <idle>-0       3dNh1    6us : _raw_spin_lock <-__run_hrtimer
    <idle>-0       3dNh1    6us : add_preempt_count <-_raw_spin_lock
    <idle>-0       3dNh2    7us : _raw_spin_unlock <-hrtimer_interrupt
    <idle>-0       3dNh2    7us : sub_preempt_count <-_raw_spin_unlock
    <idle>-0       3dNh1    7us : tick_program_event <-hrtimer_interrupt
    <idle>-0       3dNh1    7us : clockevents_program_event <-tick_program_event
    <idle>-0       3dNh1    8us : ktime_get <-clockevents_program_event
    <idle>-0       3dNh1    8us : lapic_next_event <-clockevents_program_event
    <idle>-0       3dNh1    9us : irq_exit <-smp_apic_timer_interrupt
    <idle>-0       3dNh1    9us : sub_preempt_count <-irq_exit
    <idle>-0       3dN.2    9us : idle_cpu <-irq_exit
    <idle>-0       3dN.2    9us : rcu_irq_exit <-irq_exit
    <idle>-0       3dN.2   10us : rcu_eqs_enter_common.isra.45 <-rcu_irq_exit
    <idle>-0       3dN.2   10us : sub_preempt_count <-irq_exit
    <idle>-0       3.N.1   11us : rcu_idle_exit <-cpu_idle
    <idle>-0       3dN.1   11us : rcu_eqs_exit_common.isra.43 <-rcu_idle_exit
    <idle>-0       3.N.1   11us : tick_nohz_idle_exit <-cpu_idle
    <idle>-0       3dN.1   12us : menu_hrtimer_cancel <-tick_nohz_idle_exit
    <idle>-0       3dN.1   12us : ktime_get <-tick_nohz_idle_exit
    <idle>-0       3dN.1   12us : tick_do_update_jiffies64 <-tick_nohz_idle_exit
    <idle>-0       3dN.1   13us : cpu_load_update_nohz <-tick_nohz_idle_exit
    <idle>-0       3dN.1   13us : _raw_spin_lock <-cpu_load_update_nohz
    <idle>-0       3dN.1   13us : add_preempt_count <-_raw_spin_lock
    <idle>-0       3dN.2   13us : __cpu_load_update <-cpu_load_update_nohz
    <idle>-0       3dN.2   14us : sched_avg_update <-__cpu_load_update
    <idle>-0       3dN.2   14us : _raw_spin_unlock <-cpu_load_update_nohz
    <idle>-0       3dN.2   14us : sub_preempt_count <-_raw_spin_unlock
    <idle>-0       3dN.1   15us : calc_load_nohz_stop <-tick_nohz_idle_exit
    <idle>-0       3dN.1   15us : touch_softlockup_watchdog <-tick_nohz_idle_exit
    <idle>-0       3dN.1   15us : hrtimer_cancel <-tick_nohz_idle_exit
    <idle>-0       3dN.1   15us : hrtimer_try_to_cancel <-hrtimer_cancel
    <idle>-0       3dN.1   16us : lock_hrtimer_base.isra.18 <-hrtimer_try_to_cancel
    <idle>-0       3dN.1   16us : _raw_spin_lock_irqsave <-lock_hrtimer_base.isra.18
    <idle>-0       3dN.1   16us : add_preempt_count <-_raw_spin_lock_irqsave
    <idle>-0       3dN.2   17us : __remove_hrtimer <-remove_hrtimer.part.16
    <idle>-0       3dN.2   17us : hrtimer_force_reprogram <-__remove_hrtimer
    <idle>-0       3dN.2   17us : tick_program_event <-hrtimer_force_reprogram
    <idle>-0       3dN.2   18us : clockevents_program_event <-tick_program_event
    <idle>-0       3dN.2   18us : ktime_get <-clockevents_program_event
    <idle>-0       3dN.2   18us : lapic_next_event <-clockevents_program_event
    <idle>-0       3dN.2   19us : _raw_spin_unlock_irqrestore <-hrtimer_try_to_cancel
    <idle>-0       3dN.2   19us : sub_preempt_count <-_raw_spin_unlock_irqrestore
    <idle>-0       3dN.1   19us : hrtimer_forward <-tick_nohz_idle_exit
    <idle>-0       3dN.1   20us : ktime_add_safe <-hrtimer_forward
    <idle>-0       3dN.1   20us : ktime_add_safe <-hrtimer_forward
    <idle>-0       3dN.1   20us : hrtimer_start_range_ns <-hrtimer_start_expires.constprop.11
    <idle>-0       3dN.1   21us : __hrtimer_start_range_ns <-hrtimer_start_range_ns
    <idle>-0       3dN.1   21us : lock_hrtimer_base.isra.18 <-__hrtimer_start_range_ns
    <idle>-0       3dN.1   21us : _raw_spin_lock_irqsave <-lock_hrtimer_base.isra.18
    <idle>-0       3dN.1   21us : add_preempt_count <-_raw_spin_lock_irqsave
    <idle>-0       3dN.2   22us : ktime_add_safe <-__hrtimer_start_range_ns
    <idle>-0       3dN.2   22us : enqueue_hrtimer <-__hrtimer_start_range_ns
    <idle>-0       3dN.2   22us : tick_program_event <-__hrtimer_start_range_ns
    <idle>-0       3dN.2   23us : clockevents_program_event <-tick_program_event
    <idle>-0       3dN.2   23us : ktime_get <-clockevents_program_event
    <idle>-0       3dN.2   23us : lapic_next_event <-clockevents_program_event
    <idle>-0       3dN.2   24us : _raw_spin_unlock_irqrestore <-__hrtimer_start_range_ns
    <idle>-0       3dN.2   24us : sub_preempt_count <-_raw_spin_unlock_irqrestore
    <idle>-0       3dN.1   24us : account_idle_ticks <-tick_nohz_idle_exit
    <idle>-0       3dN.1   24us : account_idle_time <-account_idle_ticks
    <idle>-0       3.N.1   25us : sub_preempt_count <-cpu_idle
    <idle>-0       3.N..   25us : schedule <-cpu_idle
    <idle>-0       3.N..   25us : __schedule <-preempt_schedule
    <idle>-0       3.N..   26us : add_preempt_count <-__schedule
    <idle>-0       3.N.1   26us : rcu_note_context_switch <-__schedule
    <idle>-0       3.N.1   26us : rcu_sched_qs <-rcu_note_context_switch
    <idle>-0       3dN.1   27us : rcu_preempt_qs <-rcu_note_context_switch
    <idle>-0       3.N.1   27us : _raw_spin_lock_irq <-__schedule
    <idle>-0       3dN.1   27us : add_preempt_count <-_raw_spin_lock_irq
    <idle>-0       3dN.2   28us : put_prev_task_idle <-__schedule
    <idle>-0       3dN.2   28us : pick_next_task_stop <-pick_next_task
    <idle>-0       3dN.2   28us : pick_next_task_rt <-pick_next_task
    <idle>-0       3dN.2   29us : dequeue_pushable_task <-pick_next_task_rt
    <idle>-0       3d..3   29us : __schedule <-preempt_schedule
    <idle>-0       3d..3   30us :      0:120:R ==> [003]  2448: 94:R sleep


```
即便启用了函数跟踪，这也不是很大的跟踪，所以我把整个跟踪都包含了进来

中断在系统空闲时触发。在 task_woken_rt() 被调用之前的某处，NEED_RESCHED 标志被设置，
这由第一次出'N' 标志指示

### 延迟跟踪与事


由于函数跟踪会带来大得多的延迟，但如果不看到延迟期间发生了什么，就很难知道是什
导致了它。有一个折中方案，那就是启用事件

```

  # echo 0 > options/function-trace
  # echo wakeup_rt > current_tracer
  # echo 1 > events/enable
  # echo 1 > tracing_on
  # echo 0 > tracing_max_latency
  # chrt -f 5 sleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: wakeup_rt
  #
  # wakeup_rt latency trace v1.1.5 on 3.8.0-test+
  # --------------------------------------------------------------------
  # latency: 6 us, #12/12, CPU#2 | (M:preempt VP:0, KP:0, SP:0 HP:0 #P:4)
  #    -----------------
  #    | task: sleep-5882 (uid:0 nice:0 policy:1 rt_prio:5)
  #    -----------------
  #
  #                  _------=> CPU#
  #                 / _-----=> irqs-off
  #                | / _----=> need-resched
  #                || / _---=> hardirq/softirq
  #                ||| / _--=> preempt-depth
  #                |||| /     delay
  #  cmd     pid   ||||| time  |   caller
  #     \   /      |||||  \    |   /
    <idle>-0       2d.h4    0us :      0:120:R   + [002]  5882: 94:R sleep
    <idle>-0       2d.h4    0us : ttwu_do_activate.constprop.87 <-try_to_wake_up
    <idle>-0       2d.h4    1us : sched_wakeup: comm=sleep pid=5882 prio=94 success=1 target_cpu=002
    <idle>-0       2dNh2    1us : hrtimer_expire_exit: hrtimer=ffff88007796feb8
    <idle>-0       2.N.2    2us : power_end: cpu_id=2
    <idle>-0       2.N.2    3us : cpu_idle: state=4294967295 cpu_id=2
    <idle>-0       2dN.3    4us : hrtimer_cancel: hrtimer=ffff88007d50d5e0
    <idle>-0       2dN.3    4us : hrtimer_start: hrtimer=ffff88007d50d5e0 function=tick_sched_timer expires=34311211000000 softexpires=34311211000000
    <idle>-0       2.N.2    5us : rcu_utilization: Start context switch
    <idle>-0       2.N.2    5us : rcu_utilization: End context switch
    <idle>-0       2d..3    6us : __schedule <-schedule
    <idle>-0       2d..3    6us :      0:120:R ==> [002]  5882: 94:R sleep


```
### 硬件延迟探测


硬件延迟探测器通过启用 "hwlat" 跟踪器来运行

注意，该跟踪器会影响系统性能，因为它会周期性地让一CPU 在中断禁用的情况下持续忙等

```

  # echo hwlat > current_tracer
  # sleep 100
  # cat trace
  # tracer: hwlat
  #
  # entries-in-buffer/entries-written: 13/13   #P:8
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
             <...>-1729  [001] d...   678.473449: #1     inner/outer(us):   11/12    ts:1581527483.343962693 count:6
             <...>-1729  [004] d...   689.556542: #2     inner/outer(us):   16/9     ts:1581527494.889008092 count:1
             <...>-1729  [005] d...   714.756290: #3     inner/outer(us):   16/16    ts:1581527519.678961629 count:5
             <...>-1729  [001] d...   718.788247: #4     inner/outer(us):    9/17    ts:1581527523.889012713 count:1
             <...>-1729  [002] d...   719.796341: #5     inner/outer(us):   13/9     ts:1581527524.912872606 count:1
             <...>-1729  [006] d...   844.787091: #6     inner/outer(us):    9/12    ts:1581527649.889048502 count:2
             <...>-1729  [003] d...   849.827033: #7     inner/outer(us):   18/9     ts:1581527654.889013793 count:1
             <...>-1729  [007] d...   853.859002: #8     inner/outer(us):    9/12    ts:1581527658.889065736 count:1
             <...>-1729  [001] d...   855.874978: #9     inner/outer(us):   9/11    ts:1581527660.861991877 count:1
             <...>-1729  [001] d...   863.938932: #10    inner/outer(us):    9/11    ts:1581527668.970010500 count:1 nmi-total:7 nmi-count:1
             <...>-1729  [007] d...   878.050780: #11    inner/outer(us):    9/12    ts:1581527683.385002600 count:1 nmi-total:5 nmi-count:1
             <...>-1729  [007] d...   886.114702: #12    inner/outer(us):    9/12    ts:1581527691.385001600 count:1


```
上面的输出在头部上大致相同。所有事件都会有中断禁用标志 'd'。在 FUNCTION 标题下方是：

 #1
	这是记录的、大tracing_threshold（见下文）的事件计数

 inner/outer(us):   11/11

      这显示两个数字："内部延迟"外部延迟"。测试在一个循环中运行，检查两次时间戳
      在两个时间戳之间检测到的延迟就内部延迟"，而在前一个时间戳和循环中下一
      时间戳之间检测到的延迟就外部延迟"

 ts:1581527483.343962693

      在该窗口中记录第一个延迟时的绝对时间戳

 count:6

      在该窗口期间检测到延迟的次数

 nmi-total:7 nmi-count:1

      在支持它的架构上，如果测试期间来NMI，则 NMI 中花费的时间会报告在 "nmi-total"
      中（以微秒为单位）

      所有具NMI 的架构都会在测试期间来了 NMI 时显"nmi-count"

hwlat 文件

  tracing_threshold
	该文件会自动设置"10"，表10 微秒。这是需要被检测到才会记录跟踪的延迟阈值

	注意，当 hwlat 跟踪器结束（"current_tracer" 写入另一个跟踪器）时，tracing_threshold
	的原始值会被放回该文件

  hwlat_detector/width
	测试在中断禁用状态下运行的时长

  hwlat_detector/window
	测试运行的窗口的时长。也就是说，测试会在每个 "window" 微秒内运"width"
	微秒

  tracing_cpumask
	测试启动时，会创建一个内核线程来运行测试。该线程会在每个周期（一"window"
	之间tracing_cpumask 中列出的 CPU 之间交替。要将测试限制在特定 CPU 上，
	将该文件中的掩码设置为测试应当运行的那些 CPU

### function


该跟踪器就是函数跟踪器。可以通过调试文件系统启用函数跟踪器。确ftrace_enabled 
设置；否则该跟踪器就是一nop。请参阅下面"ftrace_enabled" 一节

```

  # sysctl kernel.ftrace_enabled=1
  # echo function > current_tracer
  # echo 1 > tracing_on
  # usleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: function
  #
  # entries-in-buffer/entries-written: 24799/24799   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
              bash-1994  [002] ....  3082.063030: mutex_unlock <-rb_simple_write
              bash-1994  [002] ....  3082.063031: __mutex_unlock_slowpath <-mutex_unlock
              bash-1994  [002] ....  3082.063031: __fsnotify_parent <-fsnotify_modify
              bash-1994  [002] ....  3082.063032: fsnotify <-fsnotify_modify
              bash-1994  [002] ....  3082.063032: __srcu_read_lock <-fsnotify
              bash-1994  [002] ....  3082.063032: add_preempt_count <-__srcu_read_lock
              bash-1994  [002] ...1  3082.063032: sub_preempt_count <-__srcu_read_lock
              bash-1994  [002] ....  3082.063033: __srcu_read_unlock <-fsnotify
  [...]


```
注意：函数跟踪器使用环形缓冲区来存储上述条目。最新的数据可能会覆盖最旧的数据。有
使用 echo 来停止跟踪并不够，因为跟踪可能已经覆盖了你想记录的数据。因此，有时最好直
从程序中禁用跟踪。这让你能够在命中你感兴趣的部分时停止跟踪。要C 程序直接禁用跟踪

```

	int trace_fd;
	[...]
	int main(int argc, char *argv[]) {
		[...]
		trace_fd = open(tracing_file("tracing_on"), O_WRONLY);
		[...]
		if (condition_hit()) {
			write(trace_fd, "0", 1);
		}
		[...]
	}

```
### 鍗曠嚎绋嬭窡韪。


通过set_ftrace_pid 写入，你可以跟踪一

```

  # cat set_ftrace_pid
  no pid
  # echo 3111 > set_ftrace_pid
  # cat set_ftrace_pid
  3111
  # echo function > current_tracer
  # cat trace | head
  # tracer: function
  #
  #           TASK-PID    CPU#    TIMESTAMP  FUNCTION
  #              | |       |          |         |
      yum-updatesd-3111  [003]  1637.254676: finish_task_switch <-thread_return
      yum-updatesd-3111  [003]  1637.254681: hrtimer_cancel <-schedule_hrtimeout_range
      yum-updatesd-3111  [003]  1637.254682: hrtimer_try_to_cancel <-hrtimer_cancel
      yum-updatesd-3111  [003]  1637.254683: lock_hrtimer_base <-hrtimer_try_to_cancel
      yum-updatesd-3111  [003]  1637.254685: fget_light <-do_sys_poll
      yum-updatesd-3111  [003]  1637.254686: pipe_poll <-do_sys_poll
  # echo > set_ftrace_pid
  # cat trace |head
  # tracer: function
  #
  #           TASK-PID    CPU#    TIMESTAMP  FUNCTION
  #              | |       |          |         |
  ##### CPU 3 buffer started ####
      yum-updatesd-3111  [003]  1701.957688: free_poll_entry <-poll_freewait
      yum-updatesd-3111  [003]  1701.957689: remove_wait_queue <-free_poll_entry
      yum-updatesd-3111  [003]  1701.957691: fput <-free_poll_entry
      yum-updatesd-3111  [003]  1701.957692: audit_syscall_exit <-sysret_audit
      yum-updatesd-3111  [003]  1701.957693: path_put <-audit_syscall_exit

```
如果你想在执行时跟踪一个函数，可以使用类似这样一个简单程序

```

	#include <stdio.h>
	#include <stdlib.h>
	#include <sys/types.h>
	#include <sys/stat.h>
	#include <fcntl.h>
	#include <unistd.h>
	#include <string.h>

	#define _STR(x) #x
	#define STR(x) _STR(x)
	#define MAX_PATH 256

	const char *find_tracefs(void)
	{
	       static char tracefs[MAX_PATH+1];
	       static int tracefs_found;
	       char type[100];
	       FILE *fp;

	       if (tracefs_found)
		       return tracefs;

	       if ((fp = fopen("/proc/mounts","r")) == NULL) {
		       perror("/proc/mounts");
		       return NULL;
	       }

	       while (fscanf(fp, "%*s %"
		             STR(MAX_PATH)
		             "s %99s %*s %*d %*d\n",
		             tracefs, type) == 2) {
		       if (strcmp(type, "tracefs") == 0)
		               break;
	       }
	       fclose(fp);

	       if (strcmp(type, "tracefs") != 0) {
		       fprintf(stderr, "tracefs not mounted");
		       return NULL;
	       }

	       strcat(tracefs, "/tracing/");
	       tracefs_found = 1;

	       return tracefs;
	}

	const char *tracing_file(const char *file_name)
	{
	       static char trace_file[MAX_PATH+1];
	       snprintf(trace_file, MAX_PATH, "%s/%s", find_tracefs(), file_name);
	       return trace_file;
	}

	int main (int argc, char **argv)
	{
		if (argc < 1)
		        exit(-1);

		if (fork() > 0) {
		        int fd, ffd;
		        char line[64];
		        int s;

		        ffd = open(tracing_file("current_tracer"), O_WRONLY);
		        if (ffd < 0)
		                exit(-1);
		        write(ffd, "nop", 3);

		        fd = open(tracing_file("set_ftrace_pid"), O_WRONLY);
		        s = sprintf(line, "%d\n", getpid());
		        write(fd, line, s);

		        write(ffd, "function", 8);

		        close(fd);
		        close(ffd);

		        execvp(argv[1], argv+1);
		}

		return 0;
	}

```
或者这个简单的脚本

```

  #!/bin/bash

  tracefs=`sed -ne 's/^tracefs \(.*\) tracefs.*/\1/p' /proc/mounts`
  echo 0 > $tracefs/tracing_on
  echo $$ > $tracefs/set_ftrace_pid
  echo function > $tracefs/current_tracer
  echo 1 > $tracefs/tracing_on
  exec "$@"


```
### 函数图跟踪器


该跟踪器与函数跟踪器类似，区别在于它在函数的入口和出口处都进行探测。这是通过在每
task_struct 中使用一个动态分配的返回地址栈来实现的。在函数入口处，跟踪器会覆盖每个
被跟踪函数的返回地址，以设置一个自定义探针。因此原始的返回地址被存储在 task_struct
的返回地址栈上

在函数两端都进行探测会带来特殊功能，例如

- 测量函数的执行时
- 拥有可靠的调用栈以绘制函数调用图

该跟踪器在以下几种情况下很有用：

- 你想找出某个奇怪的内核行为的原因，并且需要详细查看任何区域（或特定区域）内部发生
  什么

- 你正经历奇怪的延迟，但很难找到其根源

- 你想快速找到某个特定函数所采取的路

- 你只是想窥探一个正在运行的内核内部，看看那里发生了什么

```

  # tracer: function_graph
  #
  # CPU  DURATION                  FUNCTION CALLS
  # |     |   |                     |   |   |   |

   0)               |  sys_open() {
   0)               |    do_sys_open() {
   0)               |      getname() {
   0)               |        kmem_cache_alloc() {
   0)   1.382 us    |          __might_sleep();
   0)   2.478 us    |        }
   0)               |        strncpy_from_user() {
   0)               |          might_fault() {
   0)   1.389 us    |            __might_sleep();
   0)   2.553 us    |          }
   0)   3.807 us    |        }
   0)   7.876 us    |      }
   0)               |      alloc_fd() {
   0)   0.668 us    |        _spin_lock();
   0)   0.570 us    |        expand_files();
   0)   0.586 us    |        _spin_unlock();


```
有几个列可以动态启禁用。你可以根据需要使用任意选项组合

- 函数执行所在的 CPU 编号默认启用。有时最好只跟踪一CPU（见 tracing_cpumask 文件），
  或者你可能CPU 跟踪切换时看到乱序的函数调用

 - 隐藏：echo nofuncgraph-cpu > trace_options
 - 显示：echo funcgraph-cpu > trace_options

- 持续时间（函数的执行时间）显示在函数的闭合花括号行上，或者在叶子函数的情况下显示在与
  当前函数同一行上。默认启用

 - 隐藏：echo nofuncgraph-duration > trace_options
 - 显示：echo funcgraph-duration > trace_options

- overhead 字段在达到持续时间阈值时位于 duration 字段之前

 - 隐藏：echo nofuncgraph-overhead > trace_options
 - 显示：echo funcgraph-overhead > trace_options
 - 依赖于：funcgraph-duration

```

    3) # 1837.709 us |          } /* __switch_to */
    3)               |          finish_task_switch() {
    3)   0.313 us    |            _raw_spin_unlock_irq();
    3)   3.177 us    |          }
    3) # 1889.063 us |        } /* __schedule */
    3) ! 140.417 us  |      } /* __schedule */
    3) # 2034.948 us |    } /* schedule */
    3) * 33998.59 us |  } /* schedule_preempt_disabled */

    [...]

    1)   0.260 us    |              msecs_to_jiffies();
    1)   0.313 us    |              __rcu_read_unlock();
    1) + 61.770 us   |            }
    1) + 64.479 us   |          }
    1)   0.313 us    |          rcu_bh_qs();
    1)   0.313 us    |          __local_bh_enable();
    1) ! 217.240 us  |        }
    1)   0.365 us    |        idle_cpu();
    1)               |        rcu_irq_exit() {
    1)   0.417 us    |          rcu_eqs_enter_common.isra.47();
    1)   3.125 us    |        }
    1) ! 227.812 us  |      }
    1) ! 457.395 us  |    }
    1) @ 119760.2 us |  }

    [...]

    2)               |    handle_IPI() {
    1)   6.979 us    |                  }
    2)   0.417 us    |      scheduler_ipi();
    1)   9.791 us    |                }
    1) + 12.917 us   |              }
    2)   3.490 us    |    }
    1) + 15.729 us   |            }
    1) + 18.542 us   |          }
    2) $ 3594274 us  |  }

```

```

  + 表示该函数超10 微秒
  ! 表示该函数超100 微秒
  # 表示该函数超1000 微秒
  * 表示该函数超10 毫秒
  @ 表示该函数超100 毫秒
  $ 表示该函数超1 秒


```
- 任务/pid 字段显示执行该函数的线程命令行和 pid。默认禁用

 - 隐藏：echo nofuncgraph-proc > trace_options
 - 显示：echo funcgraph-proc > trace_options

```

    # tracer: function_graph
    #
    # CPU  TASK/PID        DURATION                  FUNCTION CALLS
    # |    |    |           |   |                     |   |   |   |
    0)    sh-4802     |               |                  d_free() {
    0)    sh-4802     |               |                    call_rcu() {
    0)    sh-4802     |               |                      __call_rcu() {
    0)    sh-4802     |   0.616 us    |                        rcu_process_gp_end();
    0)    sh-4802     |   0.586 us    |                        check_for_new_grace_period();
    0)    sh-4802     |   2.899 us    |                      }
    0)    sh-4802     |   4.040 us    |                    }
    0)    sh-4802     |   5.151 us    |                  }
    0)    sh-4802     | + 49.370 us   |                }

```
- 绝对时间字段是系统时钟自启动以来给出的绝对时间戳。在函数的每次进退出时给出此时间的
  快照

 - 隐藏：echo nofuncgraph-abstime > trace_options
 - 显示：echo funcgraph-abstime > trace_options

```

    #
    #      TIME       CPU  DURATION                  FUNCTION CALLS
    #       |         |     |   |                     |   |   |   |
    360.774522 |   1)   0.541 us    |                                          }
    360.774522 |   1)   4.663 us    |                                        }
    360.774523 |   1)   0.541 us    |                                        __wake_up_bit();
    360.774524 |   1)   6.796 us    |                                      }
    360.774524 |   1)   7.952 us    |                                    }
    360.774525 |   1)   9.063 us    |                                  }
    360.774525 |   1)   0.615 us    |                                  journal_mark_dirty();
    360.774527 |   1)   0.578 us    |                                  __brelse();
    360.774528 |   1)               |                                  reiserfs_prepare_for_journal() {
    360.774528 |   1)               |                                    unlock_buffer() {
    360.774529 |   1)               |                                      wake_up_bit() {
    360.774529 |   1)               |                                        bit_waitqueue() {
    360.774530 |   1)   0.594 us    |                                          __phys_addr();


```
函数名总是在函数的闭合花括号之后显示，如果该函数开头不在跟踪缓冲区中

对于开头在跟踪缓冲区中的函数，也可以启用闭合花括号之后显示函数名，以便grep 
容易地搜索函数持续时间。默认禁用

 - 隐藏：echo nofuncgraph-tail > trace_options
 - 显示：echo funcgraph-tail > trace_options

```

    0)               |      putname() {
    0)               |        kmem_cache_free() {
    0)   0.518 us    |          __phys_addr();
    0)   1.757 us    |        }
    0)   2.861 us    |      }

  使用 funcgraph-tail 的示例：

    0)               |      putname() {
    0)               |        kmem_cache_free() {
    0)   0.518 us    |          __phys_addr();
    0)   1.757 us    |        } /* kmem_cache_free() */
    0)   2.861 us    |      } /* putname() */

```
每个被跟踪函数的返回值可以显示在等号 "=" 之后。当遇到系统调用失败时，它能非常有帮助地
快速定位第一个返回错误码的函数

 - 隐藏：echo nofuncgraph-retval > trace_options
 - 显示：echo funcgraph-retval > trace_options

```

    1)               |    cgroup_migrate() {
    1)   0.651 us    |      cgroup_migrate_add_task(); /* = 0xffff93fcfd346c00 */
    1)               |      cgroup_migrate_execute() {
    1)               |        cpu_cgroup_can_attach() {
    1)               |          cgroup_taskset_first() {
    1)   0.732 us    |            cgroup_taskset_next(); /* = 0xffff93fc8fb20000 */
    1)   1.232 us    |          } /* cgroup_taskset_first = 0xffff93fc8fb20000 */
    1)   0.380 us    |          sched_rt_can_attach(); /* = 0x0 */
    1)   2.335 us    |        } /* cpu_cgroup_can_attach = -22 */
    1)   4.369 us    |      } /* cgroup_migrate_execute = -22 */
    1)   7.143 us    |    } /* cgroup_migrate = -22 */

```
上面的示例显示函cpu_cgroup_can_attach 首先返回了错误码 -22，然后我们可以阅读该
函数的代码来找到根本原因

当未设置 funcgraph-retval-hex 选项时，返回值可以以智能方式显示。具体来说，如果它是
错误码，则会以有符号十进制格式打印，否则以十六进制格式打印

 - 智能：echo nofuncgraph-retval-hex > trace_options
 - 十六进制：echo funcgraph-retval-hex > trace_options

```

    1)               |      cgroup_migrate() {
    1)   0.651 us    |        cgroup_migrate_add_task(); /* = 0xffff93fcfd346c00 */
    1)               |        cgroup_migrate_execute() {
    1)               |          cpu_cgroup_can_attach() {
    1)               |            cgroup_taskset_first() {
    1)   0.732 us    |              cgroup_taskset_next(); /* = 0xffff93fc8fb20000 */
    1)   1.232 us    |            } /* cgroup_taskset_first = 0xffff93fc8fb20000 */
    1)   0.380 us    |            sched_rt_can_attach(); /* = 0x0 */
    1)   2.335 us    |          } /* cpu_cgroup_can_attach = 0xffffffea */
    1)   4.369 us    |        } /* cgroup_migrate_execute = 0xffffffea */
    1)   7.143 us    |      } /* cgroup_migrate = 0xffffffea */

```
目前，使funcgraph-retval 选项有一些限制，这些限制将在未来被消除：

- 即使函数的返回类型是 void，仍然会打印一个返回值，你可以直接忽略它

- 即使返回值存储在多个寄存器中，也只有第一个寄存器中的值会被记录和打印。举例来说，
  x86 架构中，eax edx 用于存储一64 位的返回值，32 位保存在 eax 中，
  32 位保存在 edx 中。但是，只有保存eax 中的值会被记录和打印

- 在某些过程调用标准中，例arm64 AAPCS64，当类型小于一GPR 时，由调用者负
  执行窄化操作，高位可能包UNKNOWN 值。因此，对于此类情况检查代码是明智的。例如，
  当在 64 GPR 中使u8 时，[63:8] 可能包含任意值，尤其是在较大类型被截断时
  （无论是显式还是隐式）。以下是一些具体案例来说明这一点：

  **案例一**

```

	u8 narrow_to_u8(u64 val)
	{
		// 隐式截断
		return val;
	}

  它可能被编译为：

	narrow_to_u8:
		< ... ftrace 插桩 ... >
		RET

  如果你向该函数传0x123456789abcdef 并想将其窄化，它可能被记录为 0x123456789abcdef
  而不0xef

  **案例*

  函数 error_if_not_4g_aligned 定义如下

	int error_if_not_4g_aligned(u64 val)
	{
		if (val & GENMASK(31, 0))
			return -EINVAL;

		return 0;
	}

  它可能被编译为：

	error_if_not_4g_aligned:
		CBNZ    w0, .Lnot_aligned
		RET			// [31:0] 为零，位
					// [63:32] 涓?UNKNOWN
	.Lnot_aligned:
		MOV    x0, #-EINVAL
		RET

  当传0x2_0000_0000 时，返回值可能被记录0x2_0000_0000 而不0

```
你可以使trace_printk() 在特定函数上添加一些注释。例如，如果你想__might_sleep()
函数内部添加注释，只需包含

```

	trace_printk("I'm a comment!\n")

```

```

   1)               |             __might_sleep() {
   1)               |                /* I'm a comment! */
   1)   1.449 us    |             }


```
你可能会在该跟踪器的以下 "dynamic ftrace"（动ftrace）一节中发现其他有用功能，例
只跟踪特定函数或任务

### 动ftrace


如果设置CONFIG_DYNAMIC_FTRACE，在函数跟踪被禁用时，系统运行的开销几乎为零。其
工作原理是，mcount 函数调用（位于每个内核函数的开头，gcc -pg 开关生成）一开
指向一个简单的返回。（启用 FTRACE 会在内核编译中包-pg 开关。）

在编译时，每C 文件目标都会经过 recordmcount 程序（位scripts 目录）。该程序
解析 C 目标中的 ELF 头，以找.text 段中所有调mcount 的位置。从 gcc 4.6 版本开始，
x86 增加-mfentry，它调用 "__fentry__" 而不"mcount"。它在栈帧创建之前调用

注意，并非所有段都会被跟踪。它们可能会notrace 阻止，或以其他方式被阻止，并且所
内联函数都不会被跟踪。查"available_filter_functions" 文件以了解哪些函数可以被跟踪

会创建一个名"__mcount_loc" 的段，其中包含对 .text 段中所mcount/fentry 调用站点
的引用。recordmcount 程序将这个段重新链接回原始目标。内核的最终链接阶段会将所
这些引用添加到一个单独的表中

在启动时，在 SMP 初始化之前，动ftrace 代码会扫描此表并将所有位置更新为 nop。它
还会记录这些位置，并将它们添加到 available_filter_functions 列表中。模块在加载时
执行前被处理。当卸载模块时，它也会将其函数从 ftrace 函数列表中移除。这在模块卸载代
中是自动完成的，模块作者无需为此担心

启用跟踪时，修改函数跟踪点的过程依赖于架构。旧方法是使kstop_machine 来防止与正在
执行被修改代码的 CPU 发生竞争（这可能导致 CPU 做出不希望的事情，特别是如果修改后的代码
跨越了缓存（或页）边界），并nop 打补丁回调用。但这一次，它们调用的不再是 mcount
（那只是一个函数桩）。它们现在调用进ftrace 基础设施

修改函数跟踪点的新方法是：在要修改的位置放置一个断点，同步所CPU，修改断点未覆盖
指令其余部分。再次同步所CPU，然后用完成的版本（指向 ftrace 调用站点）移除断点

某些架构甚至不需要折腾同步，可以直接将新代码覆盖在旧代码之上，而不会出现其CPU
同时执行它的问题

记录被跟踪函数的一个特殊副作用是，我们现在可以有选择地选择要跟踪哪些函数，以及希望
mcount 调用保持nop 的哪些函数

使用两个文件，一个用于启用，一个用于禁用指定函数的跟踪。它们是

  set_ftrace_filter

鍜。

  set_ftrace_notrace

你可以添加到这些文件中的可用函数列表列在

   available_filter_functions

```

  # cat available_filter_functions
  put_prev_task_idle
  kmem_cache_create
  pick_next_task_rt
  cpus_read_lock
  pick_next_task_fair
  mutex_lock
  [...]

```

```

  # echo sys_nanosleep hrtimer_interrupt > set_ftrace_filter
  # echo function > current_tracer
  # echo 1 > tracing_on
  # usleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: function
  #
  # entries-in-buffer/entries-written: 5/5   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
            usleep-2665  [001] ....  4186.475355: sys_nanosleep <-system_call_fastpath
            <idle>-0     [001] d.h1  4186.475409: hrtimer_interrupt <-smp_apic_timer_interrupt
            usleep-2665  [001] d.h1  4186.475426: hrtimer_interrupt <-smp_apic_timer_interrupt
            <idle>-0     [003] d.h1  4186.475426: hrtimer_interrupt <-smp_apic_timer_interrupt
            <idle>-0     [002] d.h1  4186.475427: hrtimer_interrupt <-smp_apic_timer_interrupt

```
要查看哪些函数正在被跟踪，你可以 cat 该文件：

```

  # cat set_ftrace_filter
  hrtimer_interrupt
  sys_nanosleep


```
也许这还不够。过滤器还允glob(7) 匹配

  `<match>*`
	匹配<match> 开头的函数
  `*<match>`
	匹配<match> 结尾的函
  `**<match>**`
	匹配包含 <match> 的函
  `<match1>*<match2>`
	匹配<match1> 开头并<match2> 结尾的函

      最好使用引号将通配符括起来，否shell 可能会将参数展开为本地目录中的文件名

```

  # echo 'hrtimer_*' > set_ftrace_filter

```

```

  # tracer: function
  #
  # entries-in-buffer/entries-written: 897/897   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
            <idle>-0     [003] dN.1  4228.547803: hrtimer_cancel <-tick_nohz_idle_exit
            <idle>-0     [003] dN.1  4228.547804: hrtimer_try_to_cancel <-hrtimer_cancel
            <idle>-0     [003] dN.2  4228.547805: hrtimer_force_reprogram <-__remove_hrtimer
            <idle>-0     [003] dN.1  4228.547805: hrtimer_forward <-tick_nohz_idle_exit
            <idle>-0     [003] dN.1  4228.547805: hrtimer_start_range_ns <-hrtimer_start_expires.constprop.11
            <idle>-0     [003] d..1  4228.547858: hrtimer_get_next_event <-get_next_timer_interrupt
            <idle>-0     [003] d..1  4228.547859: hrtimer_start <-__tick_nohz_idle_enter
            <idle>-0     [003] d..2  4228.547860: hrtimer_force_reprogram <-__rem

```
注意我们丢失sys_nanosleep

```

  # cat set_ftrace_filter
  hrtimer_run_queues
  hrtimer_run_pending
  hrtimer_setup
  hrtimer_cancel
  hrtimer_try_to_cancel
  hrtimer_forward
  hrtimer_start
  hrtimer_reprogram
  hrtimer_force_reprogram
  hrtimer_get_next_event
  hrtimer_interrupt
  hrtimer_nanosleep
  hrtimer_wakeup
  hrtimer_get_remaining
  hrtimer_get_res
  hrtimer_init_sleeper


```
这是因为 '>' '>>' 的行为与bash 中完全一样。要重写过滤器，使用 '>'；要追加
过滤器，使用 '>>'

要清除过滤器以便记录所有函

```

 # echo > set_ftrace_filter
 # cat set_ftrace_filter
 #

```
再次，现在我们想追加

```

  # echo sys_nanosleep > set_ftrace_filter
  # cat set_ftrace_filter
  sys_nanosleep
  # echo 'hrtimer_*' >> set_ftrace_filter
  # cat set_ftrace_filter
  hrtimer_run_queues
  hrtimer_run_pending
  hrtimer_setup
  hrtimer_cancel
  hrtimer_try_to_cancel
  hrtimer_forward
  hrtimer_start
  hrtimer_reprogram
  hrtimer_force_reprogram
  hrtimer_get_next_event
  hrtimer_interrupt
  sys_nanosleep
  hrtimer_nanosleep
  hrtimer_wakeup
  hrtimer_get_remaining
  hrtimer_get_res
  hrtimer_init_sleeper


```
set_ftrace_notrace 阻止这些函数被跟踪

```

  # echo '*preempt*' '*lock*' > set_ftrace_notrace

```

```

  # tracer: function
  #
  # entries-in-buffer/entries-written: 39608/39608   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
              bash-1994  [000] ....  4342.324896: file_ra_state_init <-do_dentry_open
              bash-1994  [000] ....  4342.324897: open_check_o_direct <-do_last
              bash-1994  [000] ....  4342.324897: ima_file_check <-do_last
              bash-1994  [000] ....  4342.324898: process_measurement <-ima_file_check
              bash-1994  [000] ....  4342.324898: ima_get_action <-process_measurement
              bash-1994  [000] ....  4342.324898: ima_match_policy <-ima_get_action
              bash-1994  [000] ....  4342.324899: do_truncate <-do_last
              bash-1994  [000] ....  4342.324899: setattr_should_drop_suidgid <-do_truncate
              bash-1994  [000] ....  4342.324899: notify_change <-do_truncate
              bash-1994  [000] ....  4342.324900: current_fs_time <-notify_change
              bash-1994  [000] ....  4342.324900: current_kernel_time <-current_fs_time
              bash-1994  [000] ....  4342.324900: timespec_trunc <-current_fs_time

```
我们可以看到不再lock preempt 跟踪

### 通过索引选择函数过滤


由于字符串处理代价高昂（在将传入的字符串与函数地址比较之前，需要先查找函数的地址），
也可以使用一个索引来启用函数。这在一次性设置数千个特定函数时很有用。通过传入一个数
列表，不会发生任何字符串处理。相反，会选择内部数组（对应于 "available_filter_functions"
文件中的函数）中特定位置处的函数

```

  # echo 1 > set_ftrace_filter

```
将选择 "available_filter_functions" 中列出的第一个函

```

  # head -1 available_filter_functions
  trace_initcall_finish_cb

  # cat set_ftrace_filter
  trace_initcall_finish_cb

  # head -50 available_filter_functions | tail -1
  x86_pmu_commit_txn

  # echo 1 50 > set_ftrace_filter
  # cat set_ftrace_filter
  trace_initcall_finish_cb
  x86_pmu_commit_txn

```
### 函数图跟踪器的动ftrace


虽然上面解释的内容同时涉及函数跟踪器和函数图跟踪器，但函数图跟踪器中只有一些特
功能可用

如果你只想跟踪一个函数及其所有子函数

```

 echo __do_fault > set_graph_function

```
将产生如__do_fault() 展开"跟踪

```

   0)               |  __do_fault() {
   0)               |    filemap_fault() {
   0)               |      find_lock_page() {
   0)   0.804 us    |        find_get_page();
   0)               |        __might_sleep() {
   0)   1.329 us    |        }
   0)   3.904 us    |      }
   0)   4.979 us    |    }
   0)   0.653 us    |    _spin_lock();
   0)   0.578 us    |    page_add_file_rmap();
   0)   0.525 us    |    native_set_pte_at();
   0)   0.585 us    |    _spin_unlock();
   0)               |    unlock_page() {
   0)   0.541 us    |      page_waitqueue();
   0)   0.639 us    |      __wake_up_bit();
   0)   2.786 us    |    }
   0) + 14.237 us   |  }
   0)               |  __do_fault() {
   0)               |    filemap_fault() {
   0)               |      find_lock_page() {
   0)   0.698 us    |        find_get_page();
   0)               |        __might_sleep() {
   0)   1.412 us    |        }
   0)   3.950 us    |      }
   0)   5.098 us    |    }
   0)   0.631 us    |    _spin_lock();
   0)   0.571 us    |    page_add_file_rmap();
   0)   0.526 us    |    native_set_pte_at();
   0)   0.586 us    |    _spin_unlock();
   0)               |    unlock_page() {
   0)   0.533 us    |      page_waitqueue();
   0)   0.638 us    |      __wake_up_bit();
   0)   2.793 us    |    }
   0) + 14.012 us   |  }

```

```

 echo sys_open > set_graph_function
 echo sys_close >> set_graph_function

```
现在如果你想回到跟踪所有函数，可以清除

```

 echo > set_graph_function


```
### ftrace_enabled


注意，proc sysctl ftrace_enable 是函数跟踪器的一个总开/关开关。默认情况下它是启用
（当内核中启用了函数跟踪时）。如果它被禁用，所有函数跟踪都会被禁用。这不仅包括 ftrace
的函数跟踪器，也包括任何其他用途（perf、kprobes、栈跟踪、性能分析等）。如果注册了带有
FTRACE_OPS_FL_PERMANENT 标志设置的回调，则无法禁用它

请谨慎禁用此开关

```

  sysctl kernel.ftrace_enabled=0
  sysctl kernel.ftrace_enabled=1

 鎴?

  echo 0 > /proc/sys/kernel/ftrace_enabled
  echo 1 > /proc/sys/kernel/ftrace_enabled


```
### 过滤命令


set_ftrace_filter 接口支持一些命令

```

  <function>:<command>:<parameter>

```
支持的命令如下：

- mod:
  该命令按模块启用函数过滤。参数定义模块。例如，如果只想ext3 模块中的 write*
  函数，运行：

   echo 'write*:mod:ext3' > set_ftrace_filter

  该命令以与基于函数名过滤相同的方式与过滤器交互。因此，通过在过滤器文件中追加（>>
  来添加不同模块中的更多函数。通过加前缀来移除特定模块的函数

   echo '!writeback*:mod:ext3' >> set_ftrace_filter

  mod 命令支持模块 glob 匹配。禁用除特定模块外的所有函数跟踪：

   echo '!*:mod:!ext3' >> set_ftrace_filter

  禁用所有模块的跟踪，但仍跟踪内核：

   echo '!*:mod:*' >> set_ftrace_filter

  仅启用内核过滤：

   echo '*write*:mod:!*' >> set_ftrace_filter

  启用模块 glob 匹配的过滤：

   echo '*write*:mod:*snd*' >> set_ftrace_filter

```
- traceon/traceoff:
  这些命令在命中指定函数时打开和关闭跟踪。参数决定跟踪系统被打开和关闭的次数。如
  未指定，则没有限制。例如，要在出现 schedule bug 时禁用跟

   echo '__schedule_bug:traceoff:5' > set_ftrace_filter

  要在每次命中 __schedule_bug 时始终禁用跟踪：

   echo '__schedule_bug:traceoff' > set_ftrace_filter

  无论是否追加set_ftrace_filter，这些命令都是累积的。要移除一个命令，在其前加 '!'
  并去掉参数：

   echo '!__schedule_bug:traceoff:0' > set_ftrace_filter

  上面移除了带有计数器__schedule_bug traceoff 命令。要移除不带计数器的命令

   echo '!__schedule_bug:traceoff' > set_ftrace_filter

```
- snapshot:
  在命中该函数时会触发一次快照

   echo 'native_flush_tlb_others:snapshot' > set_ftrace_filter

  只快照一次：

   echo 'native_flush_tlb_others:snapshot:1' > set_ftrace_filter

  要移除上述命令：

   echo '!native_flush_tlb_others:snapshot' > set_ftrace_filter
   echo '!native_flush_tlb_others:snapshot:0' > set_ftrace_filter

```
- enable_event/disable_event:
  这些命令可以启用或禁用一个跟踪事件。注意，由于函数跟踪回调非常敏感，当注册这些命令
  时，跟踪点会被激活，但以"模式禁用。也就是说，跟踪点会被调用，但只是不会被跟踪
  只要有一个触发它的命令存在，事件跟踪点就保持此模式

   echo 'try_to_wake_up:enable_event:sched:sched_switch:2' > \
   	 set_ftrace_filter

  格式为：

    <function>:enable_event:<system>:<event>[:count]
    <function>:disable_event:<system>:<event>[:count]

  要移除事件命令：

   echo '!try_to_wake_up:enable_event:sched:sched_switch:0' > \
   	 set_ftrace_filter
   echo '!schedule:disable_event:sched:sched_switch' > \
   	 set_ftrace_filter

```
- dump:
  命中该函数时，它会将 ftrace 环形缓冲区的内容转储到控制台。如果你需要调试某些东西，
  并想在命中某个函数时转储跟踪，这会很有用。也许它是一个在三重故障发生之前被调用
  且不允许你获取常规转储的函数

- cpudump:
  命中该函数时，它会将当前 CPU ftrace 环形缓冲区内容转储到控制台。与 "dump" 命令
  不同，它只打印执行了触发转储的函数的那个 CPU 的环形缓冲区内容

- stacktrace:
  命中该函数时，会记录一条栈回溯

### trace_pipe


trace_pipe 输出trace 文件相同的内容，但对跟踪的影响不同。每次从 trace_pipe 读取
都会被消费。这意味着后续读取会不同。跟踪是实时的

```

  # echo function > current_tracer
  # cat trace_pipe > /tmp/trace.out &
  [1] 4153
  # echo 1 > tracing_on
  # usleep 1
  # echo 0 > tracing_on
  # cat trace
  # tracer: function
  #
  # entries-in-buffer/entries-written: 0/0   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |

  #
  # cat /tmp/trace.out
             bash-1994  [000] ....  5281.568961: mutex_unlock <-rb_simple_write
             bash-1994  [000] ....  5281.568963: __mutex_unlock_slowpath <-mutex_unlock
             bash-1994  [000] ....  5281.568963: __fsnotify_parent <-fsnotify_modify
             bash-1994  [000] ....  5281.568964: fsnotify <-fsnotify_modify
             bash-1994  [000] ....  5281.568964: __srcu_read_lock <-fsnotify
             bash-1994  [000] ....  5281.568964: add_preempt_count <-__srcu_read_lock
             bash-1994  [000] ...1  5281.568965: sub_preempt_count <-__srcu_read_lock
             bash-1994  [000] ....  5281.568965: __srcu_read_unlock <-fsnotify
             bash-1994  [000] ....  5281.568967: sys_dup2 <-system_call_fastpath


```
注意，读trace_pipe 文件会阻塞，直到有更多输入加入。这trace 文件相反。如果有
任何进程打开trace 文件进行读取，它实际上会禁用跟踪并阻止添加新条目。trace_pipe 文件
没有此限制

### 跟踪条目


在内核中诊断问题时，数据过多或过少都会令人困扰。buffer_size_kb 文件用于修改内部跟踪
缓冲区的大小。列出的数字是每CPU 可以记录的条目数。要知道完整大小，将可能CPU 数量
乘以条目数

```

  # cat buffer_size_kb
  1408 (units kilobytes)

```
或者简单地读取 buffer_total_size_kb

```

  # cat buffer_total_size_kb
  5632

```
要修改缓冲区，只需 echo 一个数字（1024 字节为单位）

```

  # echo 10000 > buffer_size_kb
  # cat buffer_size_kb
  10000 (units kilobytes)

```
它会尝试尽可能多地分配。如果你分配过多，可能会触发内存不足（Out-Of-Memory）

```

  # echo 1000000000000 > buffer_size_kb
  -bash: echo: write error: Cannot allocate memory
  # cat buffer_size_kb
  85

```
per_cpu 缓冲区也可以单独更改

```

  # echo 10000 > per_cpu/cpu0/buffer_size_kb
  # echo 100 > per_cpu/cpu1/buffer_size_kb

```
per_cpu 缓冲区不相同时，顶层buffer_size_kb 只会显示一X

```

  # cat buffer_size_kb
  X

```
这就buffer_total_size_kb 有用之处

```

  # cat buffer_total_size_kb
  12916

```
写入顶层buffer_size_kb 会将所有缓冲区重置为相同大小

### 快照


CONFIG_TRACER_SNAPSHOT 为所有非延迟跟踪器提供一个通用的快照功能。（记录最大延迟的
延迟跟踪器，例如 "irqsoff" "wakeup"，不能使用此功能，因为它们已经在内部使用
快照机制。）

快照在某一时刻保留当前的跟踪缓冲区，而不停止跟踪。ftrace 将当前缓冲区与一个备用缓
区交换，跟踪在新的当前（=之前的备用）缓冲区中继续

"tracing" 目录中以下与 tracefs 相关的文件与此功能有关：

  snapshot:

	该文件用于拍摄快照并读取快照的输出。向该文echo 1 以分配一个备用缓冲区
	拍摄快照（交换），然后以"trace" 相同的格式（在上"文件系统" 一节中描述
	从该文件读取快照。快照的读取和跟踪可以并行执行。当备用缓冲区被分配时，echo 0
	会释放它，echo 其他（正数）值会清除快照内容。更多细节如下表所示

	+--------------+------------+------------+------------+
	|状态\输入     |     0      |     1      |    else    |
	+==============+============+============+============+
	|未分       |(不执行任何操| 分配+交换 |(不执行任何操|
	+--------------+------------+------------+------------+
	|已分       |    释放    |    交换    |    清除    |
	+--------------+------------+------------+------------+

以下是使用快照功能的示例

```

  # echo 1 > events/sched/enable
  # echo 1 > snapshot
  # cat snapshot
  # tracer: nop
  #
  # entries-in-buffer/entries-written: 71/71   #P:8
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
            <idle>-0     [005] d...  2440.603828: sched_switch: prev_comm=swapper/5 prev_pid=0 prev_prio=120   prev_state=R ==> next_comm=snapshot-test-2 next_pid=2242 next_prio=120
             sleep-2242  [005] d...  2440.603846: sched_switch: prev_comm=snapshot-test-2 prev_pid=2242 prev_prio=120   prev_state=R ==> next_comm=kworker/5:1 next_pid=60 next_prio=120
  [...]
          <idle>-0     [002] d...  2440.707230: sched_switch: prev_comm=swapper/2 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=snapshot-test-2 next_pid=2229 next_prio=120

  # cat trace
  # tracer: nop
  #
  # entries-in-buffer/entries-written: 77/77   #P:8
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
            <idle>-0     [007] d...  2440.707395: sched_switch: prev_comm=swapper/7 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=snapshot-test-2 next_pid=2243 next_prio=120
   snapshot-test-2-2229  [002] d...  2440.707438: sched_switch: prev_comm=snapshot-test-2 prev_pid=2229 prev_prio=120 prev_state=S ==> next_comm=swapper/2 next_pid=0 next_prio=120
  [...]


```
如果你尝试在当前的跟踪器是某个延迟跟踪器时使用此快照功能，你会得到以下结果

```

  # echo wakeup > current_tracer
  # echo 1 > snapshot
  bash: echo: write error: Device or resource busy
  # cat snapshot
  cat: snapshot: Device or resource busy


```
### 实例


tracefs tracing 目录中，有一个名"instances" 的目录。可以使mkdir 在该目录
内创建新目录，并使用 rmdir 移除目录。用 mkdir 在此目录中创建的目录在创建后已经包含
文件和子目录

```

  # mkdir instances/foo
  # ls instances/foo
  buffer_size_kb  buffer_total_size_kb  events  free_buffer  per_cpu
  set_event  snapshot  trace  trace_clock  trace_marker  trace_options
  trace_pipe  tracing_on

```
如你所见，新目录看起来tracing 目录本身相似。实际上它非常相似，只是缓冲区与事件与主
目录或创建的任何其他实例无关

新目录中的文件与 tracing 目录中同名的文件工作方式相同，只是使用的缓冲区是一个独立的
缓冲区。这些文件影响该缓冲区，但不会影响主缓冲区（trace_options 除外）。当前，
trace_options 对所有实例和顶层缓冲区的影响相同，但这在未来版本中可能会改变。也就是说，
选项可能会变成特定于它们所在的实例

注意，那里没有任何函数跟踪器文件，也没有 current_tracer available_tracers。这
因为缓冲区目前只能为它们启用事件

```

  # mkdir instances/foo
  # mkdir instances/bar
  # mkdir instances/zoot
  # echo 100000 > buffer_size_kb
  # echo 1000 > instances/foo/buffer_size_kb
  # echo 5000 > instances/bar/per_cpu/cpu1/buffer_size_kb
  # echo function > current_trace
  # echo 1 > instances/foo/events/sched/sched_wakeup/enable
  # echo 1 > instances/foo/events/sched/sched_wakeup_new/enable
  # echo 1 > instances/foo/events/sched/sched_switch/enable
  # echo 1 > instances/bar/events/irq/enable
  # echo 1 > instances/zoot/events/syscalls/enable
  # cat trace_pipe
  CPU:2 [LOST 11745 EVENTS]
              bash-2044  [002] .... 10594.481032: _raw_spin_lock_irqsave <-get_page_from_freelist
              bash-2044  [002] d... 10594.481032: add_preempt_count <-_raw_spin_lock_irqsave
              bash-2044  [002] d..1 10594.481032: __rmqueue <-get_page_from_freelist
              bash-2044  [002] d..1 10594.481033: _raw_spin_unlock <-get_page_from_freelist
              bash-2044  [002] d..1 10594.481033: sub_preempt_count <-_raw_spin_unlock
              bash-2044  [002] d... 10594.481033: get_pageblock_flags_group <-get_pageblock_migratetype
              bash-2044  [002] d... 10594.481034: __mod_zone_page_state <-get_page_from_freelist
              bash-2044  [002] d... 10594.481034: zone_statistics <-get_page_from_freelist
              bash-2044  [002] d... 10594.481034: __inc_zone_state <-zone_statistics
              bash-2044  [002] d... 10594.481034: __inc_zone_state <-zone_statistics
              bash-2044  [002] .... 10594.481035: arch_dup_task_struct <-copy_process
  [...]

  # cat instances/foo/trace_pipe
              bash-1998  [000] d..4   136.676759: sched_wakeup: comm=kworker/0:1 pid=59 prio=120 success=1 target_cpu=000
              bash-1998  [000] dN.4   136.676760: sched_wakeup: comm=bash pid=1998 prio=120 success=1 target_cpu=000
            <idle>-0     [003] d.h3   136.676906: sched_wakeup: comm=rcu_preempt pid=9 prio=120 success=1 target_cpu=003
            <idle>-0     [003] d..3   136.676909: sched_switch: prev_comm=swapper/3 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=rcu_preempt next_pid=9 next_prio=120
       rcu_preempt-9     [003] d..3   136.676916: sched_switch: prev_comm=rcu_preempt prev_pid=9 prev_prio=120 prev_state=S ==> next_comm=swapper/3 next_pid=0 next_prio=120
              bash-1998  [000] d..4   136.677014: sched_wakeup: comm=kworker/0:1 pid=59 prio=120 success=1 target_cpu=000
              bash-1998  [000] dN.4   136.677016: sched_wakeup: comm=bash pid=1998 prio=120 success=1 target_cpu=000
              bash-1998  [000] d..3   136.677018: sched_switch: prev_comm=bash prev_pid=1998 prev_prio=120 prev_state=R+ ==> next_comm=kworker/0:1 next_pid=59 prio=120
       kworker/0:1-59    [000] d..4   136.677022: sched_wakeup: comm=sshd pid=1995 prio=120 success=1 target_cpu=001
       kworker/0:1-59    [000] d..3   136.677025: sched_switch: prev_comm=kworker/0:1 prev_pid=59 prev_prio=120 prev_state=S ==> next_comm=bash next_pid=1998 prio=120
  [...]

  # cat instances/bar/trace_pipe
       migration/1-14    [001] d.h3   138.732674: softirq_raise: vec=3 [action=NET_RX]
            <idle>-0     [001] dNh3   138.732725: softirq_raise: vec=3 [action=NET_RX]
              bash-1998  [000] d.h1   138.733101: softirq_raise: vec=1 [action=TIMER]
              bash-1998  [000] d.h1   138.733102: softirq_raise: vec=9 [action=RCU]
              bash-1998  [000] ..s2   138.733105: softirq_entry: vec=1 [action=TIMER]
              bash-1998  [000] ..s2   138.733106: softirq_exit: vec=1 [action=TIMER]
              bash-1998  [000] ..s2   138.733106: softirq_entry: vec=9 [action=RCU]
              bash-1998  [000] ..s2   138.733109: softirq_exit: vec=9 [action=RCU]
              sshd-1995  [001] d.h1   138.733278: irq_handler_entry: irq=21 name=uhci_hcd:usb4
              sshd-1995  [001] d.h1   138.733280: irq_handler_exit: irq=21 ret=unhandled
              sshd-1995  [001] d.h1   138.733281: irq_handler_entry: irq=21 name=eth0
              sshd-1995  [001] d.h1   138.733283: irq_handler_exit: irq=21 ret=handled
  [...]

  # cat instances/zoot/trace
  # tracer: nop
  #
  # entries-in-buffer/entries-written: 18996/18996   #P:4
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
              bash-1998  [000] d...   140.733501: sys_write -> 0x2
              bash-1998  [000] d...   140.733504: sys_dup2(oldfd: a, newfd: 1)
              bash-1998  [000] d...   140.733506: sys_dup2 -> 0x1
              bash-1998  [000] d...   140.733508: sys_close(fd: a)
              bash-1998  [000] d...   140.733510: sys_close -> 0x0
              bash-1998  [000] d...   140.733514: sys_rt_sigprocmask(how: 0, nset: 0, oset: 6e2768, sigsetsize: 8)
              bash-1998  [000] d...   140.733515: sys_rt_sigprocmask -> 0x0
              bash-1998  [000] d...   140.733516: sys_rt_sigaction(sig: 2, act: 7fff718846f0, oact: 7fff71884650, sigsetsize: 8)
              bash-1998  [000] d...   140.733516: sys_rt_sigaction -> 0x0

```
你可以看到，最顶层的跟踪缓冲区只显示了函数跟踪。foo 实例显示了唤醒和任务切换

要移除实例，只需删除它们的目录：

```

  # rmdir instances/foo
  # rmdir instances/bar
  # rmdir instances/zoot

```
注意，如果有进程在某个实例目录中打开了跟踪文件，rmdir 将以 EBUSY 失败


### 鏍堣窡韪。


由于内核拥有固定大小的栈，在函数上浪费栈空间是很重要的。内核开发者必须注意他们在栈上
分配了什么。如果他们分配过多，系统就有栈溢出的危险，并会发生损坏，通常导致系统恐慌

有一些工具会检查这一点，通常是通过中断定期检查使用情况。但如果你能在每次函数调用时
执行检查，那将非常有用。由ftrace 提供了函数跟踪器，使得在每次函数调用时检查栈大小
变得方便。这通过栈跟踪器启用

CONFIG_STACK_TRACER 启用 ftrace 的栈跟踪功能。要启用它，/proc/sys/kernel/stack_tracer_enabled 写入 '1'

```

 # echo 1 > /proc/sys/kernel/stack_tracer_enabled

```
你也可以在内核命令行上启用它，以跟踪内核在启动期间的栈大小，方法是向内核命令行参
添加 "stacktrace"

运行几分钟后，输出如下：

```

  # cat stack_max_size
  2928

  # cat stack_trace
          Depth    Size   Location    (18 entries)
          -----    ----   --------
    0)     2928     224   update_sd_lb_stats+0xbc/0x4ac
    1)     2704     160   find_busiest_group+0x31/0x1f1
    2)     2544     256   load_balance+0xd9/0x662
    3)     2288      80   idle_balance+0xbb/0x130
    4)     2208     128   __schedule+0x26e/0x5b9
    5)     2080      16   schedule+0x64/0x66
    6)     2064     128   schedule_timeout+0x34/0xe0
    7)     1936     112   wait_for_common+0x97/0xf1
    8)     1824      16   wait_for_completion+0x1d/0x1f
    9)     1808     128   flush_work+0xfe/0x119
   10)     1680      16   tty_flush_to_ldisc+0x1e/0x20
   11)     1664      48   input_available_p+0x1d/0x5c
   12)     1616      48   n_tty_poll+0x6d/0x134
   13)     1568      64   tty_poll+0x64/0x7f
   14)     1504     880   do_select+0x31e/0x511
   15)      624     400   core_sys_select+0x177/0x216
   16)      224      96   sys_select+0x91/0xb9
   17)      128     128   system_call_fastpath+0x16/0x1b

```
注意，如gcc 使用-mfentry，函数会在设置栈帧之前被跟踪。这意味着当使-mfentry 时，
叶子级函数不会被栈跟踪器测试

目前mfentry 仅由 x86 gcc 4.6.0 及以上版本使用

### 更多


更多细节可以在源代码、kernel/trace/*.c 文件中找到
