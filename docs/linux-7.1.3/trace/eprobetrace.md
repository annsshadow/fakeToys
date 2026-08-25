
## Eprobe - 基于事件的探针追
:Author: Steven Rostedt <rostedt@goodmis.org>

- v6.17 撰写

## 概述

Eprobes 是放置在现有事件之上的动态事件，用于解引用作为指针的字段，或只是限制记录到追踪事件中的字段
Eprobes 依赖kprobe 事件，因此要启用此功能，请用 `CONFIG_EPROBE_EVENTS=y` 构建你的内核
Eprobes 通过 /sys/kernel/tracing/dynamic_events 文件创建
### eprobe_events 概要

```

  e[:[EGRP/][EEVENT]] GRP.EVENT [FETCHARGS]	: Set a probe
  -:[EGRP/][EEVENT]				: Clear a probe

 EGRP		: Group name of the new event. If omitted, use "eprobes" for it.
 EEVENT		: Event name. If omitted, the event name is generated and will
		  be the same event name as the event it attached to.
 GRP		: Group name of the event to attach to.
 EVENT		: Event name of the event to attach to.

 FETCHARGS	: Arguments. Each probe can have up to 128 args.
  $FIELD	: Fetch the value of the event field called FIELD.
  @ADDR		: Fetch memory at ADDR (ADDR should be in kernel)
  @SYM[+|-offs]	: Fetch memory at SYM +|- offs (SYM should be a data symbol)
  $comm		: Fetch current task comm.
  +|-[u]OFFS(FETCHARG) : Fetch memory at FETCHARG +|- OFFS address.(\*3)(\*4)
  \IMM		: Store an immediate value to the argument.
  NAME=FETCHARG : Set NAME as the argument name of FETCHARG.
  FETCHARG:TYPE : Set TYPE as the type of FETCHARG. Currently, basic types
		  (u8/u16/u32/u64/s8/s16/s32/s64), hexadecimal types
		  (x8/x16/x32/x64), VFS layer common type(%pd/%pD), "char",
                  "string", "ustring", "symbol", "symstr" and "bitfield" are
                  supported.

```
### 类型

上面FETCHARGS Documentation/trace/kprobetrace.rst 中描述的 kprobe 事件非常相似
eprobes kprobes FETCHARGS 之间的区别在于，eprobes 有一`$FIELD` 命令，用于返回所附加事件字段的内容。Eprobes 无法访问 kprobes 所拥有的寄存器、栈和函数参数
如果一个字段参数是一个指针，它可以像内存地址一样使FETCHARGS 语法进行解引用
### 附加到动态事
Eprobes 可以附加到动态事件，也可以附加到普通事件。它可以附加kprobe 事件、synthetic 事件fprobe 事件。如果一个字段的类型需要改变，这会很有用。请参阅下面的示2
## 用法示例

### 示例 1

eprobes 的基本用途是限制记录到追踪缓冲区中的数据。例如，一个常见的要追踪的事件sched_switch
```

	field:unsigned short common_type;	offset:0;	size:2;	signed:0;
	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;
	field:unsigned char common_preempt_count;	offset:3;	size:1;	signed:0;
	field:int common_pid;	offset:4;	size:4;	signed:1;

	field:char prev_comm[16];	offset:8;	size:16;	signed:0;
	field:pid_t prev_pid;	offset:24;	size:4;	signed:1;
	field:int prev_prio;	offset:28;	size:4;	signed:1;
	field:long prev_state;	offset:32;	size:8;	signed:1;
	field:char next_comm[16];	offset:40;	size:16;	signed:0;
	field:pid_t next_pid;	offset:56;	size:4;	signed:1;
	field:int next_prio;	offset:60;	size:4;	signed:1;

```
前四个字段是所有事件共有的，无法被限制。但该事件的其余部分60 字节的信息。它记录了被调度出和调入的前后任务的名称，以及它们的 pid 和优先级。它还记录了前一任务的状态。如果只关心任务pid，为什么要浪费环形缓冲区来记录所有其他字段呢
Eprobe 可以限制记录的内容。注意，这对性能没有帮助，因为所有字段都会记录在一个临时缓冲区中以处理 eprobe```

 # echo 'e:sched/switch sched.sched_switch prev=$prev_pid:u32 next=$next_pid:u32' >> /sys/kernel/tracing/dynamic_events
 # echo 1 > /sys/kernel/tracing/events/sched/switch/enable
 # cat /sys/kernel/tracing/trace

 # tracer: nop
 #
 # entries-in-buffer/entries-written: 2721/2721   #P:8
 #
 #                                _-----=> irqs-off/BH-disabled
 #                               / _----=> need-resched
 #                              | / _---=> hardirq/softirq
 #                              || / _--=> preempt-depth
 #                              ||| / _-=> migrate-disable
 #                              |||| /     delay
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
     sshd-session-1082    [004] d..4.  5041.239906: switch: (sched.sched_switch) prev=1082 next=0
             bash-1085    [001] d..4.  5041.240198: switch: (sched.sched_switch) prev=1085 next=141
    kworker/u34:5-141     [001] d..4.  5041.240259: switch: (sched.sched_switch) prev=141 next=1085
           <idle>-0       [004] d..4.  5041.240354: switch: (sched.sched_switch) prev=0 next=1082
             bash-1085    [001] d..4.  5041.240385: switch: (sched.sched_switch) prev=1085 next=141
    kworker/u34:5-141     [001] d..4.  5041.240410: switch: (sched.sched_switch) prev=141 next=1085
             bash-1085    [001] d..4.  5041.240478: switch: (sched.sched_switch) prev=1085 next=0
     sshd-session-1082    [004] d..4.  5041.240526: switch: (sched.sched_switch) prev=1082 next=0
           <idle>-0       [001] d..4.  5041.247524: switch: (sched.sched_switch) prev=0 next=90
           <idle>-0       [002] d..4.  5041.247545: switch: (sched.sched_switch) prev=0 next=16
      kworker/1:1-90      [001] d..4.  5041.247580: switch: (sched.sched_switch) prev=90 next=0
        rcu_sched-16      [002] d..4.  5041.247591: switch: (sched.sched_switch) prev=16 next=0
           <idle>-0       [002] d..4.  5041.257536: switch: (sched.sched_switch) prev=0 next=16
        rcu_sched-16      [002] d..4.  5041.257573: switch: (sched.sched_switch) prev=16 next=0

```
注意，如果在 prev_pid next_pid 之后不加上“u32”，这些值默认会以十六进制显示
### 示例 2

如果要记录某个特定的系统调用，但 syscalls 事件未启用，仍然可以使用 raw_syscalls（系统调用事件不是普通事件，而是在内核中raw_syscalls 事件创建）。为了追openat 系统调用，可以在 raw_syscalls 事件之上创建一个事件探针：
```

 # cd /sys/kernel/tracing
 # cat events/raw_syscalls/sys_enter/format
 name: sys_enter
 ID: 395
 format:
	field:unsigned short common_type;	offset:0;	size:2;	signed:0;
	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;
	field:unsigned char common_preempt_count;	offset:3;	size:1;	signed:0;
	field:int common_pid;	offset:4;	size:4;	signed:1;

	field:long id;	offset:8;	size:8;	signed:1;
	field:unsigned long args[6];	offset:16;	size:48;	signed:0;

 print fmt: "NR %ld (%lx, %lx, %lx, %lx, %lx, %lx)", REC->id, REC->args[0], REC->args[1], REC->args[2], REC->args[3], REC->args[4], REC->args[5]

```
从源代码看，sys_openat() 具有```

 int sys_openat(int dirfd, const char *path, int flags, mode_t mode)
 {
	return my_syscall4(__NR_openat, dirfd, path, flags, mode);
 }

```
path 是第二个参数，而这正是想要的```

 # echo 'e:openat raw_syscalls.sys_enter nr=$id filename=+8($args):ustring' >> dynamic_events

```
这是x86_64 上运行的，其中字大小8 字节，openat 系统调用 __NR_openat 设置257```

 # echo 'nr == 257' > events/eprobes/openat/filter

```
现在启用该事件并查看追踪记录```

 # echo 1 > events/eprobes/openat/enable
 # cat trace

 # tracer: nop
 #
 # entries-in-buffer/entries-written: 4/4   #P:8
 #
 #                                _-----=> irqs-off/BH-disabled
 #                               / _----=> need-resched
 #                              | / _---=> hardirq/softirq
 #                              || / _--=> preempt-depth
 #                              ||| / _-=> migrate-disable
 #                              |||| /     delay
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
              cat-1298    [003] ...2.  2060.875970: openat: (raw_syscalls.sys_enter) nr=0x101 filename=(fault)
              cat-1298    [003] ...2.  2060.876197: openat: (raw_syscalls.sys_enter) nr=0x101 filename=(fault)
              cat-1298    [003] ...2.  2060.879126: openat: (raw_syscalls.sys_enter) nr=0x101 filename=(fault)
              cat-1298    [003] ...2.  2060.879639: openat: (raw_syscalls.sys_enter) nr=0x101 filename=(fault)

```
filename 显示fault)”。这很可能是因为 filename 尚未被拉入内存，而当前的 trace 事件无法 fault in（按需调入）尚未出现的内存。当 eprobe 尝试读取尚未fault in 的内存时，它会显示fault)”文本
为了绕过这一点，由于内核很可能将这个 filename 拉入并使其存在，将其附加到一synthetic 事件上，该事件可以将 filename 的地址从事件的入口传递到事件的末尾，这可用于在系统调用返回时显示 filename
```

 # echo 1 > events/eprobes/openat/enable
 # echo '-:openat' >> dynamic_events

```
```

 # echo 'e:openat_start raw_syscalls.sys_enter nr=$id filename=+8($args):x64' >> dynamic_events

```
创建一synthetic 事件，将 filename 的地址传递到
```

 # echo 's:filename u64 file' >> dynamic_events
 # echo 'hist:keys=common_pid:f=filename if nr == 257' > events/eprobes/openat_start/trigger
 # echo 'hist:keys=common_pid:file=$f:onmatch(eprobes.openat_start).trace(filename,$file) if id == 257' > events/raw_syscalls/sys_exit/trigger

```
既然 filename 的地址已被传递到系统调用的末尾，创建另一eprobe 附加到退出事件以显示
```

 # echo 'e:openat synthetic.filename filename=+0($file):ustring' >> dynamic_events
 # echo 1 > events/eprobes/openat/enable
 # cat trace

 # tracer: nop
 #
 # entries-in-buffer/entries-written: 4/4   #P:8
 #
 #                                _-----=> irqs-off/BH-disabled
 #                               / _----=> need-resched
 #                              | / _---=> hardirq/softirq
 #                              || / _--=> preempt-depth
 #                              ||| / _-=> migrate-disable
 #                              |||| /     delay
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
              cat-1331    [001] ...5.  2944.787977: openat: (synthetic.filename) filename="/etc/ld.so.cache"
              cat-1331    [001] ...5.  2944.788480: openat: (synthetic.filename) filename="/lib/x86_64-linux-gnu/libc.so.6"
              cat-1331    [001] ...5.  2944.793426: openat: (synthetic.filename) filename="/usr/lib/locale/locale-archive"
              cat-1331    [001] ...5.  2944.831362: openat: (synthetic.filename) filename="trace"

```
### 示例 3

如果有可用的 syscall trace 事件，上述做法就不需要第一```

 # echo 's:filename u64 file' >> dynamic_events
 # echo 'hist:keys=common_pid:f=filename' > events/syscalls/sys_enter_openat/trigger
 # echo 'hist:keys=common_pid:file=$f:onmatch(syscalls.sys_enter_openat).trace(filename,$file)' > events/syscalls/sys_exit_openat/trigger
 # echo 'e:openat synthetic.filename filename=+0($file):ustring' >> dynamic_events
 # echo 1 > events/eprobes/openat/enable

```
而这会产生与示例 2 相同的结果