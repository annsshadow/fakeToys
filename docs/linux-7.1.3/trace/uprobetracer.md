## Uprobe-tracer：基于 Uprobe 的事件跟踪


:Author: Srikar Dronamraju


### 概述


基于 uprobe 的跟踪事件与基于 kprobe 的跟踪事件类似。要启用此功能，请用
CONFIG_UPROBE_EVENTS=y 构建你的内核。

与 kprobe-event 跟踪器类似，这不需要通过 current_tracer 激活。取而代之的是，通过
/sys/kernel/tracing/uprobe_events 添加探测点，并通过
/sys/kernel/tracing/events/uprobes/<EVENT>/enable 启用它。

但与 kprobe-event 跟踪器不同，uprobe 事件接口期望用户计算探测点在对象中的偏移量。

你也可以使用 /sys/kernel/tracing/dynamic_events 代替 uprobe_events。该接口还将为
其他动态事件提供统一访问。

### uprobe_tracer 语法


```

  p[:[GRP/][EVENT]] PATH:OFFSET [FETCHARGS] : Set a uprobe
  r[:[GRP/][EVENT]] PATH:OFFSET [FETCHARGS] : Set a return uprobe (uretprobe)
  p[:[GRP/][EVENT]] PATH:OFFSET%return [FETCHARGS] : Set a return uprobe (uretprobe)
  -:[GRP/][EVENT]                           : Clear uprobe or uretprobe event

  GRP           : Group name. If omitted, "uprobes" is the default value.
  EVENT         : Event name. If omitted, the event name is generated based
                  on PATH+OFFSET.
  PATH          : Path to an executable or a library.
  OFFSET        : Offset where the probe is inserted.
  OFFSET%return : Offset where the return probe is inserted.

  FETCHARGS     : Arguments. Each probe can have up to 128 args.
   %REG         : Fetch register REG
   @ADDR	: Fetch memory at ADDR (ADDR should be in userspace)
   @+OFFSET	: Fetch memory at OFFSET (OFFSET from same file as PATH)
   $stackN	: Fetch Nth entry of stack (N >= 0)
   $stack	: Fetch stack address.
   $retval	: Fetch return value.(\*1)
   $comm	: Fetch current task comm.
   +|-[u]OFFS(FETCHARG) : Fetch memory at FETCHARG +|- OFFS address.(\*2)(\*3)
   \IMM		: Store an immediate value to the argument.
   NAME=FETCHARG     : Set NAME as the argument name of FETCHARG.
   FETCHARG:TYPE     : Set TYPE as the type of FETCHARG. Currently, basic types
		       (u8/u16/u32/u64/s8/s16/s32/s64), hexadecimal types
		       (x8/x16/x32/x64), "string" and bitfield are supported.

  (\*1) only for return probe.
  (\*2) this is useful for fetching a field of data structures.
  (\*3) Unlike kprobe event, "u" prefix will just be ignored, because uprobe
        events can access only user-space memory.

```
### 类型


fetch-args 支持多种类型。Uprobe 跟踪器将按给定类型访问内存。前缀 's' 和 'u' 分别
表示这些类型是有符号和无符号的。'x' 前缀表示它是无符号的。被跟踪的参数以十进制
（'s' 和 'u'）或十六进制（'x'）显示。在没有类型转换的情况下，根据架构使用 'x32'
或 'x64'（例如 x86-32 使用 x32，x86-64 使用 x64）。
字符串类型是一种特殊类型，它从用户空间获取一个"以 null 结尾"的字符串。
位域是另一种特殊类型，它接受 3 个参数：位宽、位
```

 b<bit-width>@<bit-offset>/<container-size>

```
对于 $comm，默认类型是 "string"；任何其他类型都无效。


### 事件统计


你可以通过 /sys/kernel/tracing/uprobe_profile 检查每个事件的探测命中总数。第一列
是文件名，第二列是事件名，第三列是探测命中次数。

### 使用示例


 - Add a probe as a new uprobe event, write a new definition to uprobe_events
```

    echo 'p /bin/bash:0x4245c0' > /sys/kernel/tracing/uprobe_events

 * Add a probe as a new uretprobe event::

    echo 'r /bin/bash:0x4245c0' > /sys/kernel/tracing/uprobe_events

 * Unset registered event::

    echo '-:p_bash_0x4245c0' >> /sys/kernel/tracing/uprobe_events

 * Print out the events that are registered::

    cat /sys/kernel/tracing/uprobe_events

 * Clear all events::

    echo > /sys/kernel/tracing/uprobe_events

```
以下示例展示了如何转储指令指针和 %ax 寄存器
```

    # cd /sys/kernel/tracing/
    # cat /proc/`pgrep zsh`/maps | grep /bin/zsh | grep r-xp
    00400000-0048a000 r-xp 00000000 08:03 130904 /bin/zsh
    # objdump -T /bin/zsh | grep -w zfree
    0000000000446420 g    DF .text  0000000000000012  Base        zfree

```
0x46420 是对象 /bin/zsh 中 zfree 的偏移量，该对象被加载到
```

    # echo 'p:zfree_entry /bin/zsh:0x46420 %ip %ax' > uprobe_events

```
```

    # echo 'r:zfree_exit /bin/zsh:0x46420 %ip %ax' >> uprobe_events

```
	中。

我们可以通过查看 uprobe_events 文件来查看已注册的事件。
```

    # cat uprobe_events
    p:uprobes/zfree_entry /bin/zsh:0x00046420 arg1=%ip arg2=%ax
    r:uprobes/zfree_exit /bin/zsh:0x00046420 arg1=%ip arg2=%ax

```
事件的格式可以通过查看文件 events/uprobes/zfree_entry/format 来查看。
```

    # cat events/uprobes/zfree_entry/format
    name: zfree_entry
    ID: 922
    format:
         field:unsigned short common_type;         offset:0;  size:2; signed:0;
         field:unsigned char common_flags;         offset:2;  size:1; signed:0;
         field:unsigned char common_preempt_count; offset:3;  size:1; signed:0;
         field:int common_pid;                     offset:4;  size:4; signed:1;
         field:int common_padding;                 offset:8;  size:4; signed:1;

         field:unsigned long __probe_ip;           offset:12; size:4; signed:0;
         field:u32 arg1;                           offset:16; size:4; signed:0;
         field:u32 arg2;                           offset:20; size:4; signed:0;

    print fmt: "(%lx) arg1=%lx arg2=%lx", REC->__probe_ip, REC->arg1, REC->arg2

```
定义之后，每个事件默认是禁用的。为了跟踪这些事件
```

    # echo 1 > events/uprobes/enable

```
让我们开始跟踪，睡眠一段时间然后停止跟踪。
```

    # echo 1 > tracing_on
    # sleep 20
    # echo 0 > tracing_on

```
```

    # echo 0 > events/uprobes/enable

```
你可以通过 /sys/kernel/tracing/trace 查看跟踪信息。
```

    # cat trace
    # tracer: nop
    #
    #           TASK-PID    CPU#    TIMESTAMP  FUNCTION
    #              | |       |          |         |
                 zsh-24842 [006] 258544.995456: zfree_entry: (0x446420) arg1=446420 arg2=79
                 zsh-24842 [007] 258545.000270: zfree_exit:  (0x446540 <- 0x446420) arg1=446540 arg2=0
                 zsh-24842 [002] 258545.043929: zfree_entry: (0x446420) arg1=446420 arg2=79
                 zsh-24842 [004] 258547.046129: zfree_exit:  (0x446540 <- 0x446420) arg1=446540 arg2=0

```
输出显示，uprobe 被 pid 24842 触发，ip 为 0x446420，ax 寄存器内容为 79。而 uretprobe
被触发时 ip 在 0x446540，对应的函数入口在 0x446420。
