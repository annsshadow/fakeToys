## 基于 Kprobe 的事件跟踪（Kprobe-based Event Tracing
:Author: Masami Hiramatsu

### 概述（Overview
这些事件与基tracepoint 的事件类似。与 tracepoint 不同，它基于 kprobe（kprobe kretprobe）因此它可以探kprobe 能够探测的任何地方（这意味着，除了带`__kprobes`/`nokprobe_inline` 注解以及
标记NOKPROBE_SYMBOL 的函数之外的所有函数）。与基于 tracepoint 的事件不同，它可以动态地、在运行添加和移除
要启用此功能，请以内CONFIG_KPROBE_EVENTS=y 构建你的内核
与事件跟踪器（event tracer）类似，它不需要通过 current_tracer 激活。取而代之的是，通过
/sys/kernel/tracing/kprobe_events 添加探测点，并通过
/sys/kernel/tracing/events/kprobes/<EVENT>/enable 启用它
你也可以使用 /sys/kernel/tracing/dynamic_events 代替 kprobe_events。该接口也将为其它动态事件提统一的访问方式
### kprobe_events 语法（Synopsis of kprobe_events
```

  p[:[GRP/][EVENT]] [MOD:]SYM[+offs]|MEMADDR [FETCHARGS]	: Set a probe
  r[MAXACTIVE][:[GRP/][EVENT]] [MOD:]SYM[+0] [FETCHARGS]	: Set a return probe
  p[:[GRP/][EVENT]] [MOD:]SYM[+0]%return [FETCHARGS]	: Set a return probe
  -:[GRP/][EVENT]						: Clear a probe

 GRP		: Group name. If omitted, use "kprobes" for it.
 EVENT		: Event name. If omitted, the event name is generated
		  based on SYM+offs or MEMADDR.
 MOD		: Module name which has given SYM.
 SYM[+offs]	: Symbol+offset where the probe is inserted.
 SYM%return	: Return address of the symbol
 MEMADDR	: Address where the probe is inserted.
 MAXACTIVE	: Maximum number of instances of the specified function that
		  can be probed simultaneously, or 0 for the default value
		  as defined in Documentation/trace/kprobes.rst section 1.3.1.

 FETCHARGS	: Arguments. Each probe can have up to 128 args.
  %REG		: Fetch register REG
  @ADDR		: Fetch memory at ADDR (ADDR should be in kernel)
  @SYM[+|-offs]	: Fetch memory at SYM +|- offs (SYM should be a data symbol)
  $stackN	: Fetch Nth entry of stack (N >= 0)
  $stack	: Fetch stack address.
  $argN		: Fetch the Nth function argument. (N >= 1) (\*1)
  $retval	: Fetch return value.(\*2)
  $comm		: Fetch current task comm.
  +|-[u]OFFS(FETCHARG) : Fetch memory at FETCHARG +|- OFFS address.(\*3)(\*4)
  \IMM		: Store an immediate value to the argument.
  NAME=FETCHARG : Set NAME as the argument name of FETCHARG.
  FETCHARG:TYPE : Set TYPE as the type of FETCHARG. Currently, basic types
		  (u8/u16/u32/u64/s8/s16/s32/s64), hexadecimal types
		  (x8/x16/x32/x64), VFS layer common type(%pd/%pD), "char",
                  "string", "ustring", "symbol", "symstr" and bitfield are
                  supported.

  (\*1) only for the probe on function entry (offs == 0). Note, this argument access
        is best effort, because depending on the argument type, it may be passed on
        the stack. But this only support the arguments via registers.
  (\*2) only for return probe. Note that this is also best effort. Depending on the
        return value type, it might be passed via a pair of registers. But this only
        accesses one register.
  (\*3) this is useful for fetching a field of data structures.
  (\*4) "u" means user-space dereference. See :ref:`user_mem_access`.

```
### kretprobe 处的函数参数（Function arguments at kretprobe
函数参数可以kretprobe 处使$arg<N> fetch 参数来访问。这对于一次性记录函数参数和返回值，并跟结构体字段的差异（用于调试某个函数是否正确更新了给定的数据结构）很有用。关于其工作原理，请参见
fprobe 事件中的示例<fprobetrace_exit_args_sample>
### 类型（Types
fetcharg 支持多种类型。Kprobe 跟踪器会按给定类型访问内存。前缀 's' 'u' 分别表示这些类型是有符号
和无符号的x' 前缀表示它是无符号的。被跟踪的参数以十进制（'s' 'u'）或十六进制x'）显示。不
进行类型转换时，根据架构使用 'x32' 'x64'（例x86-32 使用 x32，x86-64 使用 x64）
这些值类型可以是数组。要记录数组数据，你可以给基类型添加 '[N]'（其N 是一个小64 的固定数字）例如 'x16[^4^]' 表示4 个元素的 x16 字节十六进制）数组。注意，数组可以应用于内存类型的
fetcharg，但不能应用于寄存器/栈条目等（例'$stack1:x8[^8^]' 是错误的，但 '+8($stack):x8[^8^]'
是正确的）
Char 类型可用于显示被跟踪参数的字符值
String 类型是一种特殊类型，它从内核空间获取一null 结尾"的字符串。这意味着如果该字符串所在的
容器已被换出，它将失败并存储 NULLustring" 类型string 面向用户空间的替代类型。更多信息参user_mem_access
字符串数组类型与其它类型略有不同。对于其它基类型base-type>[^1^] 等于 <base-type>（例+0(%di):x32[^1^] +0(%di):x32 相同）。但 string[^1^] 不等string。string 类型本身表示"字符数组"而字符串数组类型表示"char * 数组"。因此，例如 +0(%di):string[^1^] 等于 +0(+0(%di)):stringBitfield 是另一种特殊类型，它接3 个参数：位宽、位偏移和容器大小：
```

 b<bit-width>@<bit-offset>/<container-size>

```
Symbol 类型symbol'）是 u32 u64 类型（取决于 BITS_PER_LONG）的别名，以 "symbol+offset" 样式
显示给定的指针。另一方面，symbol-string 类型symstr'）把给定的地址转换"symbol+offset/symbolsize"
样式，并将其作为null 结尾的字符串存储。使'symstr' 类型，你可以用符号的通配符模式过滤事件，无需自己解析符号名。对$comm，默认类型是 "string"；任何其它类型都是无效的
VFS 层通用类型pd/%pD）是一种特殊类型，它从 struct dentry 的地址struct file 的地址获取 dentry
或文件名
### 用户内存访问（User Memory Access
Kprobe 事件支持用户空间内存访问。为此，你可以使用用户空间解引用语法'ustring' 类型
用户空间解引用语法允许你访问用户空间中某个数据结构的字段。这是通过给解引用语法添加 "u" 前缀来实现的例如u4(%si) 表示它将从寄存器 %si 中地址偏移 4 的位置读取内存，并且该内存预期位于用户空间。你也可把它用于字符串，例如 +u0(%si):string 将从寄存%si 中预期位于用户空间的地址读取一个字符串ustring'
是执行相同任务的快捷方式。也就是说，+0(%si):ustring 等价+u0(%si):string
注意，kprobe-event 提供了用户内存访问语法，但它并不会透明地使用它。这意味着如果你对用户内存使用普通的
解引用或 string 类型，它可能会失败，并且在某些架构上可能总是失败。用户必须仔细检查目标数据是在内空间还是用户空间
### 每探测事件过滤（Per-Probe Event Filtering
每探测事件过滤功能允许你在每个探测上设置不同的过滤器，并决定哪些参数会显示在跟踪缓冲区中。如果在
kprobe_events 'p:' 'r:' 之后指定了事件名，它会在 tracing/events/kprobes/<EVENT> 下添加一个事件，
在该目录中你可以看到 'id'enable'format'filter' 'trigger'
enable:
  你可以通过向其写入 1 0 来启禁用该探测
format:
  这显示该探测事件的格式
filter:
  你可以写入该事件的过滤规则
id:
  这显示该探测事件id
trigger:
  这允许安装当事件命中时执行的触发命令（详情参Documentation/trace/events.rst 6 节）
### 事件统计（Event Profiling
你可以通过 /sys/kernel/tracing/kprobe_profile 查看探测命中和未命中的总次数。第一列是事件名，第二是探测命中次数，第三列是探测未命中次数
### 内核启动参数（Kernel Boot Parameter
你可以通过 "kprobe_event=" 参数在内核启动时添加并启用新kprobe 事件。该参数接受以分号分隔的 kprobe
事件，其格式kprobe_events 类似。区别在于探测定义参数是以逗号分隔的：
```

  p:myprobe do_sys_open dfd=%ax filename=%dx flags=%cx mode=+4($stack)

```
```

  p:myprobe,do_sys_open,dfd=%ax,filename=%dx,flags=%cx,mode=+4($stack)


```
### 使用示例（Usage examples
要添加一个新的事件作为探测，kprobe_events 写入一个新的定义：
```

  echo 'p:myprobe do_sys_open dfd=%ax filename=%dx flags=%cx mode=+4($stack)' > /sys/kernel/tracing/kprobe_events

```
这会do_sys_open() 函数顶部设置了一kprobe，把1 到第 4 个参数记录为 "myprobe" 事件。注意，每个
函数参数被分配到哪个寄存栈条目取决于架构相关ABI。如果你不确ABI，请尝试使用 perf-tools probe
子命令（你可以在 tools/perf/ 下找到它）。正如这个示例所示，用户可以为每个参数选择更熟悉的名称```

  echo 'r:myretprobe do_sys_open $retval' >> /sys/kernel/tracing/kprobe_events

```
这会do_sys_open() 函数的返回点设置了一kretprobe，把返回值记录为 "myretprobe" 事件你可以通过 /sys/kernel/tracing/events/kprobes/<EVENT>/format 查看这些事件的格式```

  cat /sys/kernel/tracing/events/kprobes/myprobe/format
  name: myprobe
  ID: 780
  format:
          field:unsigned short common_type;       offset:0;       size:2; signed:0;
          field:unsigned char common_flags;       offset:2;       size:1; signed:0;
          field:unsigned char common_preempt_count;       offset:3; size:1;signed:0;
          field:int common_pid;   offset:4;       size:4; signed:1;

          field:unsigned long __probe_ip; offset:12;      size:4; signed:0;
          field:int __probe_nargs;        offset:16;      size:4; signed:1;
          field:unsigned long dfd;        offset:20;      size:4; signed:0;
          field:unsigned long filename;   offset:24;      size:4; signed:0;
          field:unsigned long flags;      offset:28;      size:4; signed:0;
          field:unsigned long mode;       offset:32;      size:4; signed:0;


  print fmt: "(%lx) dfd=%lx filename=%lx flags=%lx mode=%lx", REC->__probe_ip,
  REC->dfd, REC->filename, REC->flags, REC->mode

```
你可以看到，该事件拥4 个参数，正如你所指定的表达式那样```

  echo > /sys/kernel/tracing/kprobe_events

```
这会清除所有探测点
或者，
```

  echo -:myprobe >> kprobe_events

```
这会选择性地清除探测点
在定义之后，每个事件默认是禁用的。要跟踪这些事件，你需要启用它```

  echo 1 > /sys/kernel/tracing/events/kprobes/myprobe/enable
  echo 1 > /sys/kernel/tracing/events/kprobes/myretprobe/enable

```
使用以下命令在一段区间内开始跟踪```

    # echo 1 > tracing_on
    Open something...
    # echo 0 > tracing_on

```
你可以通过 /sys/kernel/tracing/trace 查看跟踪到的信息```

  cat /sys/kernel/tracing/trace
  # tracer: nop
  #
  #           TASK-PID    CPU#    TIMESTAMP  FUNCTION
  #              | |       |          |         |
             <...>-1447  [001] 1038282.286875: myprobe: (do_sys_open+0x0/0xd6) dfd=3 filename=7fffd1ec4440 flags=8000 mode=0
             <...>-1447  [001] 1038282.286878: myretprobe: (sys_openat+0xc/0xe <- do_sys_open) $retval=fffffffffffffffe
             <...>-1447  [001] 1038282.286885: myprobe: (do_sys_open+0x0/0xd6) dfd=ffffff9c filename=40413c flags=8000 mode=1b6
             <...>-1447  [001] 1038282.286915: myretprobe: (sys_open+0x1b/0x1d <- do_sys_open) $retval=3
             <...>-1447  [001] 1038282.286969: myprobe: (do_sys_open+0x0/0xd6) dfd=ffffff9c filename=4041c6 flags=98800 mode=10
             <...>-1447  [001] 1038282.286976: myretprobe: (sys_open+0x1b/0x1d <- do_sys_open) $retval=3


```
每行显示内核命中一个事件的时刻，<- SYMBOL 表示内核SYMBOL 返回（例"sys_open+0x1b/0x1d <- do_sys_open"
表示内核do_sys_open 返回sys_open+0x1b）