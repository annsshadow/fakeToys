## 事件跟踪


:Author: Theodore Ts'o
:Updated: Li Zefan and Tom Zanussi

## 1. 简介


跟踪点（tracepoints，见 Documentation/trace/tracepoints.rst）可以在不创建
自定义内核模块的情况下使用，通过事件跟踪（event tracing）基础设施来注册
探测（probe）函数。

并非所有跟踪点都能通过事件跟踪系统进行跟踪；内核开发者必须提供代码片段，
定义跟踪信息如何保存到跟踪缓冲区（tracing buffer），以及如何打印这些
跟踪信息。

## 2. 使用事件跟踪


### 2.1 通过 'set_event' 接口


可用于跟踪的事件可在 /sys/kernel/tracing/available_events 文件中找到。

要启用某个特定事件（例如 'sched_wakeup'），只需将其 echo 写入：

```
	# echo sched_wakeup >> /sys/kernel/tracing/set_event
```

要禁用某个事件，将事件名加上前缀后 echo 写入 set_event 文件：

```
	# echo '!sched_wakeup' >> /sys/kernel/tracing/set_event
```

```
	# echo > /sys/kernel/tracing/set_event
```

```
	# echo *:* > /sys/kernel/tracing/set_event
```

这些事件被组织到各个子系统中，例如 ext4、irq、sched 等，一个完整的事件名
形如：<subsystem>:<event>。子系统名是可选的，但会显示在 available_events
文件中。某个子系统中的所有事件可以通过 `<subsystem>:*` 语法来指定；例如，
要启用所有 irq 事件，可以使用：

```
	# echo 'irq:*' > /sys/kernel/tracing/set_event
```

set_event 文件也可用于启用仅与某个模块关联的事件：

```
	# echo ':mod:<module>' > /sys/kernel/tracing/set_event
```

这将启用模块 `<module>` 中的所有事件。如果该模块尚未加载，这个字符串会被
保存下来，当加载到与之匹配的模块 `<module>` 时，就会应用事件的启用设置。

`:mod:` 之前的文本会被解析，用以指定该模块中需要启用的具体事件：

```
	# echo '<match>:mod:<module>' > /sys/kernel/tracing/set_event
```

上述命令将启用任何与 `<match>` 匹配的系统或事件。如果 `<match>` 为 `"*"`，
则会匹配所有事件。

```
	# echo '<system>:<event>:mod:<module>' > /sys/kernel/tracing/set_event
```

如果 `<event>` 为 `"*"`，则会匹配该模块给定系统中的所有事件。

### 2.2 通过 'enable' 开关


可用的事件也以目录层级的形式列在 /sys/kernel/tracing/events/ 下。

```
	# echo 1 > /sys/kernel/tracing/events/sched/sched_wakeup/enable
```

```
	# echo 0 > /sys/kernel/tracing/events/sched/sched_wakeup/enable
```

```
	# echo 1 > /sys/kernel/tracing/events/sched/enable
```

```
	# echo 1 > /sys/kernel/tracing/events/enable
```

读取这些 enable 文件时，会有四种结果：

 - 0 - 该文件影响的所有事件都已禁用
 - 1 - 该文件影响的所有事件都已启用
 - X - 启用和禁用的事件混合存在
 - ? - 该文件不影响任何事件

### 2.3 启动参数


```
	trace_event=[event-list]
```

event-list 是一个以逗号分隔的事件列表。事件格式见第 2.1 节。

## 3. 定义一个启用事件跟踪的跟踪点


参见 samples/trace_events 中提供的示例。

## 4. 事件格式


每个跟踪事件都有一个关联的 'format' 文件，其中包含对日志记录事件中每个字段
的描述。这些信息可用于解析二进制的跟踪流，同时也是查找可用于事件过滤器
（见第 5 节）的字段名的地方。

它还显示了用于以文本模式打印事件的格式字符串，以及用于性能分析的
事件名和 ID。

每个事件都有一组与之关联的 `common` 字段；这些是以 `common_` 为前缀的字段。
其他字段在不同的事件间有所不同，对应于该事件在 TRACE_EVENT 定义中声明的字段。

```
     field:field-type field-name; offset:N; size:N;
```

其中 offset 是字段在跟踪记录中的偏移，size 是数据项的大小（以字节为单位）。

例如，下面是 'sched_wakeup' 事件所显示的信息：

```
	# cat /sys/kernel/tracing/events/sched/sched_wakeup/format

	name: sched_wakeup
	ID: 60
	format:
		field:unsigned short common_type;	offset:0;	size:2;
		field:unsigned char common_flags;	offset:2;	size:1;
		field:unsigned char common_preempt_count;	offset:3;	size:1;
		field:int common_pid;	offset:4;	size:4;
		field:int common_tgid;	offset:8;	size:4;

		field:char comm[TASK_COMM_LEN];	offset:12;	size:16;
		field:pid_t pid;	offset:28;	size:4;
		field:int prio;	offset:32;	size:4;
		field:int success;	offset:36;	size:4;
		field:int cpu;	offset:40;	size:4;

	print fmt: "task %s:%d [%d] success=%d [%03d]", REC->comm, REC->pid,
		   REC->prio, REC->success, REC->cpu
```

该事件包含 10 个字段，前 5 个是通用字段，其余 5 个是事件特有的字段。除 'comm'
（一个字符串）外，该事件的所有字段都是数字类型，这一区别在进行事件过滤时
很重要。

## 5. 事件过滤


跟踪事件可以通过在内核中为其关联布尔型“过滤表达式”来进行过滤。一旦某个事件
被记录到跟踪缓冲区，就会用该事件类型关联的过滤表达式检查其字段。字段值
“匹配”过滤器的事件会显示在跟踪输出中，而不匹配的事件将被丢弃。没有关联
过滤器的事件会匹配所有内容，这也是某个事件尚未设置过滤器时的默认行为。

### 5.1 表达式语法


一个过滤表达式由一个或多个“谓词”（predicate）组成，可以使用逻辑运算符
'&&' 和 '||' 进行组合。谓词就是一个简单的子句，它将日志记录事件中包含的
某个字段的值与常量值进行比较，并根据结果返回 0 或 1：

```
	  field-name relational-operator value
```

可以使用括号来提供任意的逻辑分组，双引号可用于防止 shell 将运算符解释为
shell 元字符。

可用于过滤的字段名可以在跟踪事件的 'format' 文件中找到（见第 4 节）。

关系运算符取决于被测试字段的类型：

数字字段可用的运算符为：

==, !=, <, <=, >, >=, &

而字符串字段可用的运算符为：

==, !=, ~

glob（~）接受通配符（\*,?）和字符类：

```
  prev_comm ~ "*sh"
  prev_comm ~ "sh*"
  prev_comm ~ "*sh*"
  prev_comm ~ "ba*sh"
```

如果该字段是一个指向用户空间（user space）的指针（例如来自 sys_enter_openat
的 "filename"），则必须在其后附加 ".ustring"：

```
  filename.ustring ~ "password"
```

因为内核需要知道如何从该指针所指向的用户空间内存中获取数据。

```
  call_site.function == security_prepare_creds
```

上述过滤会在字段 "call_site" 落在 "security_prepare_creds" 函数地址范围内时
生效。也就是说，它会比较 "call_site" 的值，如果它大于或等于该函数起始地址
且小于该函数结束地址，则过滤返回真。

".function" 后缀只能附加到大小为 long 的值上，并且只能与 "==" 或 "!=" 进行比较。

Cpumask 字段或编码了 CPU 编号的标量字段可以使用以下方式进行过滤：

```
  CPUS{$cpulist}
```

用于 cpumask 过滤的运算符有：

&（交集）, ==, !=

例如，这将过滤掉 .target_cpu 字段存在于以下列表中的事件：

```
  target_cpu & CPUS{17-42}
```

### 5.2 设置过滤器


单个事件的过滤器是通过将过滤表达式写入该事件的 'filter' 文件来设置的。

```
	# cd /sys/kernel/tracing/events/sched/sched_wakeup
	# echo "common_preempt_count > 4" > filter
```

```
	# cd /sys/kernel/tracing/events/signal/signal_generate
	# echo "((sig >= 10 && sig < 15) || sig == 17) && comm != bash" > filter
```

如果表达式中存在错误，在设置时会得到“Invalid argument”错误，并且错误的
字符串会连同：

```
	# cd /sys/kernel/tracing/events/signal/signal_generate
	# echo "((sig >= 10 && sig < 15) || dsig == 17) && comm != bash" > filter
	-bash: echo: write error: Invalid argument
	# cat filter
	((sig >= 10 && sig < 15) || dsig == 17) && comm != bash
	^
	parse_error: Field not found
```

目前错误位置的脱字符（'^'）总是出现在过滤字符串的开头；不过即便没有更精确的
位置信息，错误消息仍应具有参考价值。

### 5.2.1 过滤器限制


如果过滤器被放置在字符串指针 `(char *)` 上，而该指针并不指向环形缓冲区
（ring buffer）中的字符串，而是指向内核或用户空间内存，那么出于安全原因，
最多会将 1024 字节的内容复制到临时缓冲区中进行比较。如果内存复制时发生
缺页（指针指向不应被访问的内存），则该字符串比较将被视为不匹配。

### 5.3 清除过滤器


要清除某个事件的过滤器，向该事件的 filter 文件写入 '0'。

要清除某个子系统中所有事件的过滤器，向该子系统的 filter 文件写入 '0'。

### 5.4 子系统过滤器


为方便起见，可以通过向子系统根目录下的 filter 文件写入过滤表达式，将子系统中
每个事件的过滤器作为一个整体进行设置或清除。但要注意，如果子系统中任何
事件的过滤器缺少子系统过滤器中指定的字段，或者由于任何其他原因无法应用该
过滤器，该事件的过滤器将保留其之前的设置。这可能导致出现意料之外的过滤器
混合，进而产生令人困惑（对可能以为应用了不同过滤器的用户而言）的跟踪输出。
只有引用了仅通用（common）字段的过滤器，才能保证成功传播到所有事件。

以下是对上述几点进行说明的几个子系统过滤器示例：

```
	# cd /sys/kernel/tracing/events/sched
	# echo 0 > filter
	# cat sched_switch/filter
	none
	# cat sched_wakeup/filter
	none
```

使用仅包含通用字段的过滤器来设置 sched 子系统中所有事件的过滤器：

```
	# cd /sys/kernel/tracing/events/sched
	# echo common_pid == 0 > filter
	# cat sched_switch/filter
	common_pid == 0
	# cat sched_wakeup/filter
	common_pid == 0
```

尝试使用非通用字段为 sched 子系统中所有事件设置过滤器（除具有 prev_pid
字段的事件外，其余事件都保留了：

```
	# cd /sys/kernel/tracing/events/sched
	# echo prev_pid == 0 > filter
	# cat sched_switch/filter
	prev_pid == 0
	# cat sched_wakeup/filter
	common_pid == 0
```

### 5.5 PID 过滤


在与顶级 events 目录同级的目录下，存在一个 set_event_pid 文件，它会过滤掉
所有 PID 未列在 set_event_pid 文件中的任务的事件：

```
	# cd /sys/kernel/tracing
	# echo $$ > set_event_pid
	# echo 1 > events/enable
```

这将只跟踪当前任务的事件。

要在不丢失已包含 PID 的情况下添加更多 PID，使用 '>>'：

```
	# echo 123 244 1 >> set_event_pid
```

## 6. 事件触发器


跟踪事件可以被设置为有条件地调用触发器“命令”（trigger 'commands'），这些命令
有多种形式，下文将详细描述；例如可以是每当命中该跟踪事件时，启用或禁用
其他跟踪事件，或者调用栈回溯（stack trace）。每当调用带有附加触发器的
跟踪事件时，就会调用与该事件关联的那组触发器命令。任何给定的触发器还可以
有一个与第 5 节（事件过滤）描述形式相同的事件过滤器与之关联——只有当被调用
的事件通过了关联的过滤器时，该命令才会被调用。如果没有与触发器关联的过滤器，
则总是通过。

触发器是通过将触发器表达式写入给定事件的 'trigger' 文件来添加和移除的。

一个给定的事件可以关联任意数量的触发器，但需遵守各个命令在这方面可能有的
任何限制。

事件触发器建立在“软”（soft）模式之上，这意味着每当某个跟踪事件关联了一个
或多个触发器时，即使该事件实际上并未被启用，它也会被激活，但处于“软”模式下
被禁用。也就是说，跟踪点会被调用，但不会实际被跟踪，除非它确实被启用了。
这一机制使得即使对于未启用的事件也能调用触发器，同时也使得当前的事件过滤器
实现可用于有条件地调用触发器。

事件触发器的语法大致基于 set_ftrace_filter 的“ftrace 过滤器命令”语法（见
Documentation/trace/ftrace.rst 的“过滤器命令”一节），但两者存在重大差异，
且目前的实现并未以任何方式与之绑定，因此不要对二者妄加类比。

     写入 trace_marker（见 Documentation/trace/ftrace.rst）
     也可以启用写入
     /sys/kernel/tracing/events/ftrace/print/trigger 的触发器

### 6.1 表达式语法


```
  # echo 'command[:count] [if filter]' > trigger
```

触发器通过 echo 相同的命令但以前导的 '!' 开头来移除：

```
  # echo '!command[:count] [if filter]' > trigger
```

在移除时，[if filter] 部分不参与命令匹配，因此在使用 '!' 命令时省略它
与包含它的效果相同。

过滤器语法与上述“事件过滤”一节中描述的相同。

为方便起见，目前使用 '>' 写入 trigger 文件只是添加或移除单个触发器，并不
显式支持 '>>'（'>' 实际上表现得像 '>>'），也不支持通过截断来移除所有触发器
（你必须对每个添加的触发器使用 '!'）。

### 6.2 支持的触发器命令


以下命令受支持：

- enable_event/disable_event

  这些命令可以在触发事件命中时启用或禁用另一个跟踪事件。当注册这些命令时，
  另一个跟踪事件会被激活，但处于“软”模式下被禁用。也就是说，跟踪点会被调用，
  但不会实际被跟踪。只要存在能够触发它的生效触发器，该事件跟踪点就保持这种模式。

  例如，当进入 read 系统调用时，以下触发器会导致 kmalloc 事件被跟踪，末尾的
  :1 表示仅触发一次：

```
	  # echo 'enable_event:kmem:kmalloc:1' > \
	      /sys/kernel/tracing/events/syscalls/sys_enter_read/trigger
```

  当 read 系统调用退出时，以下触发器会导致 kmalloc 事件停止被跟踪。这种禁用
  在每次 read 系统调用退出时都会发生：

```
	  # echo 'disable_event:kmem:kmalloc' > \
	      /sys/kernel/tracing/events/syscalls/sys_exit_read/trigger
```

  格式为：

```
      enable_event:<system>:<event>[:count]
      disable_event:<system>:<event>[:count]
```

  要移除上述命令：

```
	  # echo '!enable_event:kmem:kmalloc:1' > \
	      /sys/kernel/tracing/events/syscalls/sys_enter_read/trigger

	  # echo '!disable_event:kmem:kmalloc' > \
	      /sys/kernel/tracing/events/syscalls/sys_exit_read/trigger
```

  注意，每个触发事件可以有任意数量的 enable/disable_event 触发器，但每个被
  触发的事件只能有一个触发器。例如，sys_enter_read 可以有两个触发器分别启用
  kmem:kmalloc 和 sched:sched_switch，但不能有两个 kmem:kmalloc 版本，例如
  kmem:kmalloc 和 kmem:kmalloc:1，或者 'kmem:kmalloc if bytes_req == 256' 和
  'kmem:kmalloc if bytes_alloc == 256'（不过它们可以合并为 kmem:kmalloc 上的
  单个过滤器）。

- stacktrace

  该命令在触发事件发生时将栈回溯（stacktrace）转储到跟踪缓冲区中。

  例如，以下触发器每次命中时都会转储一次栈回溯：

```
	  # echo 'stacktrace' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger
```

  以下触发器在 kmalloc 请求大小 >= 64K 时，前 5 次命中的每一次都转储栈回溯：

```
	  # echo 'stacktrace:5 if bytes_req >= 65536' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger
```

  格式为：

```
      stacktrace[:count]
```

  要移除上述命令：

```
	  # echo '!stacktrace' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger

	  # echo '!stacktrace:5 if bytes_req >= 65536' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger
```

  后者也可以更简单地通过以下方式（不带过滤器）移除：

```
	  # echo '!stacktrace:5' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger
```

  注意，每个触发事件只能有一个 stacktrace 触发器。

- snapshot

  该命令在触发事件发生时触发一次快照（snapshot）。

  以下命令在块请求队列以深度 > 1 拔出（unplug）时创建一次快照。如果你当时
  正在跟踪一组事件或函数，快照跟踪缓冲区将捕获触发那一刻的跟踪内容：

```
	  # echo 'snapshot if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  只快照一次：

```
	  # echo 'snapshot:1 if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  要移除上述命令：

```
	  # echo '!snapshot if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger

	  # echo '!snapshot:1 if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  注意，每个触发事件只能有一个 snapshot 触发器。

- traceon/traceoff

  这些命令在指定事件命中时打开或关闭跟踪。参数决定了跟踪系统被打开和关闭
  的次数。如果未指定，则没有次数限制。

  以下命令在块请求队列以深度 > 1 拔出时第一次关闭跟踪。如果你当时正在跟踪
  一组事件或函数，就可以检查跟踪缓冲区，查看导致该事件发生的事件序列：

```
	  # echo 'traceoff:1 if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  当 nr_rq > 1 时始终禁用跟踪：

```
	  # echo 'traceoff if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  要移除上述命令：

```
	  # echo '!traceoff:1 if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger

	  # echo '!traceoff if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  注意，每个触发事件只能有一个 traceon 或 traceoff 触发器。

- hist

  该命令将命中的事件聚合到一个哈希表中，哈希表的键基于一个或多个跟踪事件
  格式字段（或栈回溯），以及从一或多个跟踪事件格式字段和/或事件计数
  （hitcount）派生出来的一组累计总值。

  详见 Documentation/trace/histogram.rst 获取详细信息和示例。

## 7. 内核态跟踪事件 API


在大多数情况下，跟踪事件的命令行接口已经绰绰有余。不过有时应用程序可能需要
表达比简单的一系列链接命令行表达式更复杂的关联关系，或者将一组命令组合
起来本身就过于繁琐。例如，某个应用程序可能需要“监听”跟踪流，以便维护一个
内核态状态机，检测（比如说）调度器中何时出现了非法的内核状态。

跟踪事件子系统提供了一个内核态 API，允许模块或其他内核代码按需生成用户定义的
“合成”（synthetic）事件，这些事件既可用于扩充现有的跟踪流，也可用于发出
某个特定重要状态已发生的信号。

类似的内核态 API 也可用于创建 kprobe 和 kretprobe 事件。

合成事件 API 与 k/ret/probe 事件 API 都建立在更低层的 "dynevent_cmd" 事件
命令 API 之上，该 API 也可用于更专门的应用，或作为其他更高级跟踪事件 API
的基础。

为此提供的 API 如下所述，并允许：

  - 动态创建合成事件定义
  - 动态创建 kprobe 和 kretprobe 事件定义
  - 从内核态代码跟踪合成事件
  - 低层级的 "dynevent_cmd" API

### 7.1 动态创建合成事件定义


有几种方法可以从内核模块或其他内核代码创建新的合成事件。

第一种方法使用 synth_event_create() 一步创建事件。在这种方法中，要创建的
事件名以及一个定义字段的数组被提供给 synth_event_create()。如果成功，就会
在调用之后存在具有该名称和字段的合成事件：

```
  ret = synth_event_create("schedtest", sched_fields,
                           ARRAY_SIZE(sched_fields), THIS_MODULE);
```

此示例中的 sched_fields 参数指向一个 struct synth_field_desc 数组，其中
每一项通过类型和名称描述一个事件字段：

```
  static struct synth_field_desc sched_fields[] = {
        { .type = "pid_t",              .name = "next_pid_field" },
        { .type = "char[16]",           .name = "next_comm_field" },
        { .type = "u64",                .name = "ts_ns" },
        { .type = "u64",                .name = "ts_ms" },
        { .type = "unsigned int",       .name = "cpu" },
        { .type = "char[64]",           .name = "my_string_field" },
        { .type = "int",                .name = "my_int_field" },
  };
```

可用类型参见 synth_field_size()。

如果 field_name 包含 [n]，则该字段被视为静态数组。

如果 field_names 包含 []（无下标），则该字段被视为动态数组，它只会占用
在事件中保存该数组所需的空间。

由于事件的空间是在为字段赋值之前就预留好的，因此使用动态数组意味着下面
描述的逐段（piecewise）内核态 API 不能与动态数组一起使用。不过，其他非逐段的
内核态 API 可以与动态数组一起使用。

如果该事件是从模块内部创建的，则必须向 synth_event_create() 传递一个指向
该模块的指针。这将确保在该模块被移除时，跟踪缓冲区不会包含不可读的事件。

此时，事件对象已准备好用于生成新的事件。

在第二种方法中，事件是分若干步创建的。这允许动态创建事件，而无需事先创建
并填充一个字段数组。

要使用这种方法，应首先使用 synth_event_gen_cmd_start() 或
synth_event_gen_cmd_array_start() 创建空或部分填充的合成事件。对于
synth_event_gen_cmd_start()，应提供事件名以及一个或多个参数对（每对参数
表示一个 'type field_name;' 字段规格）。对于
synth_event_gen_cmd_array_start()，应提供事件名以及一个 struct
synth_field_desc 数组。在调用 synth_event_gen_cmd_start() 或
synth_event_gen_cmd_array_start() 之前，用户应使用 synth_event_cmd_init()
创建并初始化一个 dynevent_cmd 对象。

例如，要创建一个带两个字段的名为 "schedtest" 的合成事件：

```
  struct dynevent_cmd cmd;
  char *buf;

  /* Create a buffer to hold the generated command */
  buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);

  /* Before generating the command, initialize the cmd object */
  synth_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);

  ret = synth_event_gen_cmd_start(&cmd, "schedtest", THIS_MODULE,
                                  "pid_t", "next_pid_field",
                                  "u64", "ts_ns");
```

或者，使用 struct synth_field_desc 字段数组：

```
  ret = synth_event_gen_cmd_array_start(&cmd, "schedtest", THIS_MODULE,
                                        fields, n_fields);
```

一旦合成事件对象被创建，就可以用更多字段填充它。字段通过
synth_event_add_field() 逐个添加，提供 dynevent_cmd 对象、字段类型和字段名。
例如，要添加一个名为 intfield 的新 int 字段：

```
  ret = synth_event_add_field(&cmd, "int", "intfield");
```

可用类型参见 synth_field_size()。如果 field_name 包含 [n]，则该字段被视为
数组。

也可以使用 synth_field_desc 数组，通过 add_synth_fields() 一次性添加一组字段。
例如，这将添加：

```
  ret = synth_event_add_fields(&cmd, sched_fields, 4);
```

如果你已经有一个形如 'type field_name' 的字符串，可以使用
synth_event_add_field_str() 原样添加它；它还会自动在字符串后追加一个 ';'。

一旦所有字段都已添加，事件应被终结化并：

```
  ret = synth_event_gen_cmd_end(&cmd);
```

此时，事件对象已准备好用于跟踪新事件。

### 7.2 从内核态代码跟踪合成事件


要跟踪合成事件，有几种选择。第一种选择是使用 synth_event_trace()（接受
数量可变的多个值）或 synth_event_trace_array()（接受要设置的值数组）一次性
跟踪该事件。第二种选择可以避免预先构造值数组或参数列表的需要，通过
synth_event_trace_start() 和 synth_event_trace_end()，配合
synth_event_add_next_val() 或 synth_event_add_val() 来逐段添加值。

### 7.2.1 一次性跟踪合成事件


要一次性跟踪合成事件，可以使用 synth_event_trace() 或
synth_event_trace_array() 函数。

synth_event_trace() 函数传入表示合成事件的 trace_event_file（可通过
trace_get_event_file() 使用合成事件名、"synthetic" 作为系统名，以及跟踪
实例名（若使用全局跟踪数组则为 NULL）获取），以及数量可变的多个 u64 参数
（每个合成事件字段一个）和传入的值的个数。

因此，要跟踪对应于如下合成事件定义的事件：

```
  ret = synth_event_trace(create_synth_test, 7, /* number of values */
                          444,             /* next_pid_field */
                          (u64)"clackers", /* next_comm_field */
                          1000000,         /* ts_ns */
                          1000,            /* ts_ms */
                          smp_processor_id(),/* cpu */
                          (u64)"Thneed",   /* my_string_field */
                          999);            /* my_int_field */
```

所有值都应转换为 u64，字符串值只是指向字符串的指针，转换为 u64。字符串将
通过这些指针复制到事件中为该字符串预留的空间。

或者，可以使用 synth_event_trace_array() 函数完成同样的事情。它传入表示
合成事件的 trace_event_file（可通过 trace_get_event_file() 使用合成事件名、
"synthetic" 作为系统名，以及跟踪实例名（若使用全局跟踪数组则为 NULL）获取），
以及一个 u64 数组，每个合成事件字段一个。

要跟踪对应于如下合成事件定义的事件：

```
  u64 vals[7];

  vals[0] = 777;                  /* next_pid_field */
  vals[1] = (u64)"tiddlywinks";   /* next_comm_field */
  vals[2] = 1000000;              /* ts_ns */
  vals[3] = 1000;                 /* ts_ms */
  vals[4] = smp_processor_id();   /* cpu */
  vals[5] = (u64)"thneed";        /* my_string_field */
  vals[6] = 398;                  /* my_int_field */
```

'vals' 数组只是一个 u64 数组，其个数必须与合成事件中的字段数匹配，并且
必须与合成事件字段的顺序相同。

所有值都应转换为 u64，字符串值只是指向字符串的指针，转换为 u64。字符串将
通过这些指针复制到事件中为该字符串预留的空间。

为了跟踪合成事件，需要一个指向跟踪事件文件的指针。可以使用
trace_get_event_file() 函数获取它——它会在给定的跟踪实例（此处为 NULL，因为
使用的是顶层跟踪数组）中查找该文件，同时：

```
       schedtest_event_file = trace_get_event_file(NULL, "synthetic",
                                                   "schedtest");
```

在跟踪事件之前，应以某种方式启用它，否则合成事件实际上不会出现在跟踪缓冲区中。

要从内核启用合成事件，可以使用 trace_array_set_clr_event()（它并非合成事件
专用，因此需要显式指定 "synthetic" 系统名）。

```
       trace_array_set_clr_event(schedtest_event_file->tr,
                                 "synthetic", "schedtest", true);
```

```
       trace_array_set_clr_event(schedtest_event_file->tr,
                                 "synthetic", "schedtest", false);
```

最后，可以使用 synth_event_trace_array() 实际跟踪：

```
       ret = synth_event_trace_array(schedtest_event_file, vals,
                                     ARRAY_SIZE(vals));
```

要移除合成事件，应先禁用该事件，并：

```
       trace_array_set_clr_event(schedtest_event_file->tr,
                                 "synthetic", "schedtest", false);
       trace_put_event_file(schedtest_event_file);
```

如果这些都成功，就可以调用 synth_event_delete()：

```
       ret = synth_event_delete("schedtest");
```

### 7.2.2 逐段跟踪合成事件


要使用上文描述的逐段方法跟踪合成事件，使用 synth_event_trace_start() 函数
来“打开”合成事件：

```
       struct synth_event_trace_state trace_state;

       ret = synth_event_trace_start(schedtest_event_file, &trace_state);
```

它传入表示合成事件的 trace_event_file（使用与上述相同的方法），以及一个指向
struct synth_event_trace_state 对象的指针，该对象在使用前会被清零，并用于在
本次调用与后续调用之间维护状态。

一旦事件被打开（即已在跟踪缓冲区中为其预留了空间），就可以设置各个字段。
有两种方式：一种是按事件中的每个字段依次设置（无需查找），另一种是按名称设置
（需要查找）。两者的权衡在于赋值的灵活性与每个字段查找的开销之间。

要无需查找地依次赋值，应使用 synth_event_add_next_val()。每次调用传入与
synth_event_trace_start() 中相同的 synth_event_trace_state 对象，以及要设置
事件下一个字段的值。每设置一个字段后，“游标”（cursor）会指向下一个字段，
该字段将由后续调用设置，依次进行直到所有字段都按顺序设置完毕。与上述示例
相同的调用序列使用：

```
       /* next_pid_field */
       ret = synth_event_add_next_val(777, &trace_state);

       /* next_comm_field */
       ret = synth_event_add_next_val((u64)"slinky", &trace_state);

       /* ts_ns */
       ret = synth_event_add_next_val(1000000, &trace_state);

       /* ts_ms */
       ret = synth_event_add_next_val(1000, &trace_state);

       /* cpu */
       ret = synth_event_add_next_val(smp_processor_id(), &trace_state);

       /* my_string_field */
       ret = synth_event_add_next_val((u64)"thneed_2.01", &trace_state);

       /* my_int_field */
       ret = synth_event_add_next_val(395, &trace_state);
```

要按任意顺序赋值，应使用 synth_event_add_val()。每次调用传入与
synth_event_trace_start() 中相同的 synth_event_trace_state 对象，以及要设置的
字段的字段名和它的值。与上述示例相同的调用序列使用此方法（省略了错误处理）：

```
       ret = synth_event_add_val("next_pid_field", 777, &trace_state);
       ret = synth_event_add_val("next_comm_field", (u64)"silly putty",
                                 &trace_state);
       ret = synth_event_add_val("ts_ns", 1000000, &trace_state);
       ret = synth_event_add_val("ts_ms", 1000, &trace_state);
       ret = synth_event_add_val("cpu", smp_processor_id(), &trace_state);
       ret = synth_event_add_val("my_string_field", (u64)"thneed_9",
                                 &trace_state);
       ret = synth_event_add_val("my_int_field", 3999, &trace_state);
```

注意，synth_event_add_next_val() 和 synth_event_add_val() 在同一个事件的
跟踪过程中是不兼容的——可以使用其中任意一个，但不能同时使用两者。

最后，在事件被“关闭”之前，它实际上不会被跟踪，这一步通过
synth_event_trace_end() 完成，它只接受：

```
       ret = synth_event_trace_end(&trace_state);
```

注意，无论任何 add 调用是否失败（例如传入了错误的字段名），都必须在最后调用
synth_event_trace_end()。

### 7.3 动态创建 kprobe 和 kretprobe 事件定义


要从内核代码创建 kprobe 或 kretprobe 跟踪事件，可以使用
kprobe_event_gen_cmd_start() 或 kretprobe_event_gen_cmd_start() 函数。

要创建 kprobe 事件，应首先使用 kprobe_event_gen_cmd_start() 创建一个空或
部分填充的 kprobe 事件。应指定事件名和探测位置，以及一个表示探测字段的
参数列表提供给该函数。在调用 kprobe_event_gen_cmd_start() 之前，用户应使用
kprobe_event_cmd_init() 创建并初始化一个 dynevent_cmd 对象。

```
  struct dynevent_cmd cmd;
  char *buf;

  /* Create a buffer to hold the generated command */
  buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);

  /* Before generating the command, initialize the cmd object */
  kprobe_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);

  /*
   * Define the gen_kprobe_test event with the first 2 kprobe
   * fields.
   */
  ret = kprobe_event_gen_cmd_start(&cmd, "gen_kprobe_test", "do_sys_open",
                                   "dfd=%ax", "filename=%dx");
```

一旦 kprobe 事件对象被创建，就可以用更多字段填充它。可以使用
kprobe_event_add_fields() 添加字段，提供 dynevent_cmd 对象以及一个可变参数
列表的探测字段。例如，要添加：

```
  ret = kprobe_event_add_fields(&cmd, "flags=%cx", "mode=+4($stack)");
```

一旦所有字段都已添加，就应通过调用 kprobe_event_gen_cmd_end() 或
kretprobe_event_gen_cmd_end() 函数（取决于创建的是 kprobe 还是 kretprobe）
来终结化并注册该事件：

```
  ret = kprobe_event_gen_cmd_end(&cmd);
```

```
  ret = kretprobe_event_gen_cmd_end(&cmd);
```

此时，事件对象已准备好用于跟踪新事件。

类似地，可以使用 kretprobe_event_gen_cmd_start() 配合探针名、位置以及
来创建 kretprobe 事件：

```
  ret = kretprobe_event_gen_cmd_start(&cmd, "gen_kretprobe_test",
                                      "do_sys_open", "$retval");
```

与合成事件的情况类似，如下代码可以：

```
  gen_kprobe_test = trace_get_event_file(NULL, "kprobes", "gen_kprobe_test");

  ret = trace_array_set_clr_event(gen_kprobe_test->tr,
                                  "kprobes", "gen_kprobe_test", true);
```

最后，同样与合成事件类似，如下代码可以：

```
  trace_put_event_file(gen_kprobe_test);

  ret = kprobe_event_delete("gen_kprobe_test");
```

### 7.4 "dynevent_cmd" 低层 API


内核态的合成事件接口和 kprobe 接口都建立在更低层的 "dynevent_cmd" 接口之上。
该接口旨在为更高级的接口（例如合成事件接口和 kprobe 接口，它们可作为示例）
提供基础。

基本思想很简单，就是提供一个可用于生成跟踪事件命令的通用层。生成的命令字符串
随后可以被传递给跟踪事件子系统中已经存在的命令解析和事件创建代码，用于创建
相应的跟踪事件。

简而言之，它的工作方式是：更高级的接口代码创建一个 struct dynevent_cmd 对象，
然后使用 dynevent_arg_add() 和 dynevent_arg_pair_add() 这两个函数来构建命令
字符串，最后通过 dynevent_create() 函数执行该命令。该接口的细节如下所述。

构建新命令字符串的第一步是创建并初始化一个 dynevent_cmd 实例。例如，我们：

```
  struct dynevent_cmd cmd;
  char *buf;
  int ret;

  buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);

  dynevent_cmd_init(cmd, buf, maxlen, DYNEVENT_TYPE_FOO,
                    foo_event_run_command);
```

dynevent_cmd 初始化需要给定一个用户指定的缓冲区和缓冲区长度（可以使用
MAX_DYNEVENT_CMD_LEN——它大小为 2k，通常太大而不适合放在栈上，因此会动态分配）、
一个 dynevent 类型 id（用于检查后续 API 调用是否属于正确的命令类型），以及一个
指向特定事件的 run_command() 回调的指针，该回调将被调用以实际执行该特定事件的
命令函数。

完成之后，就可以通过连续调用添加参数的函数来构建命令字符串。

要添加单个参数，定义并初始化一个 struct dynevent_arg 或 struct
dynevent_arg_pair 对象。下面是一个最简单的参数添加示例，它只是将给定的字符串
作为附加到命令末尾：

```
  struct dynevent_arg arg;

  dynevent_arg_init(&arg, NULL, 0);

  arg.str = name;

  ret = dynevent_arg_add(cmd, &arg);
```

arg 对象首先使用 dynevent_arg_init() 初始化，在这种情况下的参数为 NULL 或 0，
意味着末尾没有附加可选的健全性检查函数或分隔符。

下面是另一个更复杂的、使用“参数对”（arg pair）的示例，它用于创建一个由两部分
组合为一个单元的参数，例如一个 'type field_name;' 参数或一个简单：

```
  struct dynevent_arg_pair arg_pair;

  dynevent_arg_pair_init(&arg_pair, dynevent_foo_check_arg_fn, 0, ';');

  arg_pair.lhs = type;
  arg_pair.rhs = name;

  ret = dynevent_arg_pair_add(cmd, &arg_pair);
```

同样，arg_pair 首先被初始化，在这种情况下带有一个用于检查参数健全性的回调
函数（例如，检查该对的两部分都不为 NULL），以及一个用于在两部分之间添加运算符
的字符（此处没有）和一个追加到参数对末尾的分隔符（此处为 ';'）。

还有一个 dynevent_str_add() 函数，可用于简单地原样添加一个字符串，不带空格、
分隔符或参数检查。

可以调用任意数量的 dynevent_*_add() 来构建字符串（直到其长度超过 cmd->maxlen）。
当所有参数都已添加且命令字符串完成时，剩下的唯一事情就是运行命令，这只需
简单地调用：

```
  ret = dynevent_create(&cmd);
```

此时，如果返回值为 0，则动态事件已被创建并可以使用。

有关该 API 的详细信息，请参见 dynevent_cmd 函数定义本身。
