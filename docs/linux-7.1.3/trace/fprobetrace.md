
## 基于 Fprobe 的事件跟踪



### 概述


Fprobe 事件与 kprobe 事件类似，但仅限于在
函数的入口和出口处进行探测。对于许多只跟踪某些
特定函数的用例来说，这已经足够了。

本文档也涵盖 tracepoint 探测事件（tprobe），因为它
同样只在 tracepoint 入口处工作。用户可以跟踪
tracepoint 的一部分参数，或者没有 trace-event 的 tracepoint，
后者不会在 tracefs 上暴露。

与其他动态事件一样，fprobe 事件和 tracepoint 探测
事件通过 tracefs 上的 `dynamic_events` 接口文件定义。

### fprobe 事件的语法

```

  f[:[GRP1/][EVENT1]] SYM [FETCHARGS]                       : Probe on function entry
  f[MAXACTIVE][:[GRP1/][EVENT1]] SYM%return [FETCHARGS]     : Probe on function exit
  t[:[GRP2/][EVENT2]] TRACEPOINT [FETCHARGS]                : Probe on tracepoint

 GRP1           : Group name for fprobe. If omitted, use "fprobes" for it.
 GRP2           : Group name for tprobe. If omitted, use "tracepoints" for it.
 EVENT1         : Event name for fprobe. If omitted, the event name is
                  "SYM__entry" or "SYM__exit".
 EVENT2         : Event name for tprobe. If omitted, the event name is
                  the same as "TRACEPOINT", but if the "TRACEPOINT" starts
                  with a digit character, "_TRACEPOINT" is used.
 MAXACTIVE      : Maximum number of instances of the specified function that
                  can be probed simultaneously, or 0 for the default value
                  as defined in Documentation/trace/fprobe.rst

 FETCHARGS      : Arguments. Each probe can have up to 128 args.
  ARG           : Fetch "ARG" function argument using BTF (only for function
                  entry or tracepoint.) (\*1)
  @ADDR         : Fetch memory at ADDR (ADDR should be in kernel)
  @SYM[+|-offs] : Fetch memory at SYM +|- offs (SYM should be a data symbol)
  $stackN       : Fetch Nth entry of stack (N >= 0)
  $stack        : Fetch stack address.
  $argN         : Fetch the Nth function argument. (N >= 1) (\*2)
  $retval       : Fetch return value.(\*3)
  $comm         : Fetch current task comm.
  +|-[u]OFFS(FETCHARG) : Fetch memory at FETCHARG +|- OFFS address.(\*4)(\*5)
  \IMM          : Store an immediate value to the argument.
  NAME=FETCHARG : Set NAME as the argument name of FETCHARG.
  FETCHARG:TYPE : Set TYPE as the type of FETCHARG. Currently, basic types
                  (u8/u16/u32/u64/s8/s16/s32/s64), hexadecimal types
                  (x8/x16/x32/x64), "char", "string", "ustring", "symbol", "symstr"
                  and bitfield are supported.

  (\*1) This is available only when BTF is enabled.
  (\*2) only for the probe on function entry (offs == 0). Note, this argument access
        is best effort, because depending on the argument type, it may be passed on
        the stack. But this only support the arguments via registers.
  (\*3) only for return probe. Note that this is also best effort. Depending on the
        return value type, it might be passed via a pair of registers. But this only
        accesses one register.
  (\*4) this is useful for fetching a field of data structures.
  (\*5) "u" means user-space dereference.

```
有关 TYPE 的详细信息，请参见 kprobetrace 文档 <kprobetrace_types>。

### 退出时的函数参数

在退出探测中可以使用 $arg<N> fetcharg 访问函数参数。这
有助于一次性记录函数参数和返回值，并
跟踪结构体字段的差异（用于调试函数是否正确
更新了给定的数据结构）
其工作方式请见下面的示例 <fprobetrace_exit_args_sample>。

### BTF 参数

BTF（BPF Type Format）参数允许用户按照名称而非 `$argN` 来跟踪函数和 tracepoint 的
参数。该特性在内核配置了 CONFIG_BPF_SYSCALL 和 CONFIG_DEBUG_INFO_BTF 时可用。
如果用户只指定 BTF 参数，事件的参数名也会
随之确定。
```

 # echo 'f:myprobe vfs_read count pos' >> dynamic_events
 # cat dynamic_events
 f:fprobes/myprobe vfs_read count=count pos=pos

```
它还会根据 BTF 信息选择获取类型。例如，在上面
的示例中，`count` 是 unsigned long，而 `pos` 是一个指针。因此，
两者都被转换为 64 位无符号 long，但只有 `pos` 带有 "%Lx"
```

 # cat events/fprobes/myprobe/format
 name: myprobe
 ID: 1313
 format:
	field:unsigned short common_type;	offset:0;	size:2;	signed:0;
	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;
	field:unsigned char common_preempt_count;	offset:3;	size:1;	signed:0;
	field:int common_pid;	offset:4;	size:4;	signed:1;

	field:unsigned long __probe_ip;	offset:8;	size:8;	signed:0;
	field:u64 count;	offset:16;	size:8;	signed:0;
	field:u64 pos;	offset:24;	size:8;	signed:0;

 print fmt: "(%lx) count=%Lu pos=0x%Lx", REC->__probe_ip, REC->count, REC->pos

```
如果用户不确定参数的名称，`$arg**` 会很有帮助。`$arg**`
```

 # echo 'f:myprobe vfs_read $arg*' >> dynamic_events
 # cat dynamic_events
 f:fprobes/myprobe vfs_read file=file buf=buf count=count pos=pos

```
BTF 也会影响 `$retval`。如果用户没有设置任何类型，返回值
类型会自动从 BTF 中选取。如果函数返回 `void`，
则 `$retval` 会被拒绝。

可以使用访问运算符 `->` 来访问数据结构的数据字段
```

```
# echo 't sched_switch preempt prev_pid=prev->pid next_pid=next->pid' >> dynamic_events

字段访问运算符 `->` 和 `.` 可以组合使用，以访问更深层的
成员以及由该成员指向的其他结构体成员。例如 `foo->bar.baz->qux`
如果存在没有名称的 union 成员，可以像 C 代码那样直接访问它。
```

 struct {
	union {
	int a;
	int b;
	};
 } *foo;

```
要访问 `a` 和 `b`，在这种情况下请使用 `foo->a` 和 `foo->b`。

这种数据字段访问也可通过 `$retval` 用于返回值，
例如 `$retval->name`。

对于这些 BTF 参数和字段，`:string` 和 `:ustring` 会改变
行为。如果它们用于 BTF 参数或字段，会检查该参数或数据字段的 BTF 类型
是否为 `char *` 或 `char []`，
若不是，则拒绝应用字符串类型。此外，借助 BTF
支持，在访问由 `PTR` 指向的字符串时，
您不再需要内存解引用运算符（`+0(PTR)`）。它会自动添加内存
```

```
# echo 't sched_switch prev->comm:string' >> dynamic_events
# echo 'f getname_flags%return $retval->name:string' >> dynamic_events

`prev->comm` 是数据结构中的内嵌字符数组，而
`$retval->name` 是数据结构中的字符指针。但在两种
情况下，都可以使用 `:string` 类型来获取字符串。


### 使用示例

下面是一个在 `vfs_read()` 函数的入口和
出口处添加带 BTF 参数的 fprobe 事件的示例。
```

  # echo 'f vfs_read $arg*' >> dynamic_events
  # echo 'f vfs_read%return $retval' >> dynamic_events
  # cat dynamic_events
 f:fprobes/vfs_read__entry vfs_read file=file buf=buf count=count pos=pos
 f:fprobes/vfs_read__exit vfs_read%return arg1=$retval
  # echo 1 > events/fprobes/enable
  # head -n 20 trace | tail
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
               sh-70      [000] ...1.   335.883195: vfs_read__entry: (vfs_read+0x4/0x340) file=0xffff888005cf9a80 buf=0x7ffef36c6879 count=1 pos=0xffffc900005aff08
               sh-70      [000] .....   335.883208: vfs_read__exit: (ksys_read+0x75/0x100 <- vfs_read) arg1=1
               sh-70      [000] ...1.   335.883220: vfs_read__entry: (vfs_read+0x4/0x340) file=0xffff888005cf9a80 buf=0x7ffef36c6879 count=1 pos=0xffffc900005aff08
               sh-70      [000] .....   335.883224: vfs_read__exit: (ksys_read+0x75/0x100 <- vfs_read) arg1=1
               sh-70      [000] ...1.   335.883232: vfs_read__entry: (vfs_read+0x4/0x340) file=0xffff888005cf9a80 buf=0x7ffef36c687a count=1 pos=0xffffc900005aff08
               sh-70      [000] .....   335.883237: vfs_read__exit: (ksys_read+0x75/0x100 <- vfs_read) arg1=1
               sh-70      [000] ...1.   336.050329: vfs_read__entry: (vfs_read+0x4/0x340) file=0xffff888005cf9a80 buf=0x7ffef36c6879 count=1 pos=0xffffc900005aff08
               sh-70      [000] .....   336.050343: vfs_read__exit: (ksys_read+0x75/0x100 <- vfs_read) arg1=1

```
可以看到所有函数参数和返回值都被记录为带符号整数。

此外，下面是一个在 `sched_switch` tracepoint 上的 tracepoint 事件示例。
为了对比结果，这里也启用了 `sched_switch` traceevent。
```

  # echo 't sched_switch $arg*' >> dynamic_events
  # echo 1 > events/sched/sched_switch/enable
  # echo 1 > events/tracepoints/sched_switch/enable
  # echo > trace
  # head -n 20 trace | tail
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
               sh-70      [000] d..2.  3912.083993: sched_switch: prev_comm=sh prev_pid=70 prev_prio=120 prev_state=S ==> next_comm=swapper/0 next_pid=0 next_prio=120
               sh-70      [000] d..3.  3912.083995: sched_switch: (__probestub_sched_switch+0x4/0x10) preempt=0 prev=0xffff88800664e100 next=0xffffffff828229c0 prev_state=1
           <idle>-0       [000] d..2.  3912.084183: sched_switch: prev_comm=swapper/0 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=rcu_preempt next_pid=16 next_prio=120
           <idle>-0       [000] d..3.  3912.084184: sched_switch: (__probestub_sched_switch+0x4/0x10) preempt=0 prev=0xffffffff828229c0 next=0xffff888004208000 prev_state=0
      rcu_preempt-16      [000] d..2.  3912.084196: sched_switch: prev_comm=rcu_preempt prev_pid=16 prev_prio=120 prev_state=I ==> next_comm=swapper/0 next_pid=0 next_prio=120
      rcu_preempt-16      [000] d..3.  3912.084196: sched_switch: (__probestub_sched_switch+0x4/0x10) preempt=0 prev=0xffff888004208000 next=0xffffffff828229c0 prev_state=1026
           <idle>-0       [000] d..2.  3912.085191: sched_switch: prev_comm=swapper/0 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=rcu_preempt next_pid=16 next_prio=120
           <idle>-0       [000] d..3.  3912.085191: sched_switch: (__probestub_sched_switch+0x4/0x10) preempt=0 prev=0xffffffff828229c0 next=0xffff888004208000 prev_state=0

```
如您所见，`sched_switch` trace-event 显示的是 **cooked**（已处理）参数，而
另一方面，`sched_switch` tracepoint 探测事件显示的是 **raw**（原始）
参数。这意味着您可以访问 task
结构体中由 `prev` 和 `next` 参数指向的任何字段值。

**例如，通常 ``task_struct**
的 start_time`` 默认不会被跟踪，但借助这个
traceprobe 事件，您可以像下面这样跟踪该字段。
```

  # echo 't sched_switch comm=next->comm:string next->start_time' > dynamic_events
  # head -n 20 trace | tail
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
               sh-70      [000] d..3.  5606.686577: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="rcu_preempt" usage=1 start_time=245000000
      rcu_preempt-16      [000] d..3.  5606.686602: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="sh" usage=1 start_time=1596095526
               sh-70      [000] d..3.  5606.686637: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="swapper/0" usage=2 start_time=0
           <idle>-0       [000] d..3.  5606.687190: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="rcu_preempt" usage=1 start_time=245000000
      rcu_preempt-16      [000] d..3.  5606.687202: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="swapper/0" usage=2 start_time=0
           <idle>-0       [000] d..3.  5606.690317: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="kworker/0:1" usage=1 start_time=137000000
      kworker/0:1-14      [000] d..3.  5606.690339: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="swapper/0" usage=2 start_time=0
           <idle>-0       [000] d..3.  5606.692368: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="kworker/0:1" usage=1 start_time=137000000

```

返回探测允许我们访问某些函数的返回结果，这些函数返回
错误码，且其结果通过函数参数传递，例如一个
结构体初始化函数。

例如，vfs_open() 会将文件结构体链接到 inode 并更新
模式。您可以使用返回探测来跟踪这些变更。
```

 # echo 'f vfs_open mode=file->f_mode:x32 inode=file->f_inode:x64' >> dynamic_events
 # echo 'f vfs_open%%return mode=file->f_mode:x32 inode=file->f_inode:x64' >> dynamic_events
 # echo 1 > events/fprobes/enable
 # cat trace
              sh-131     [006] ...1.  1945.714346: vfs_open__entry: (vfs_open+0x4/0x40) mode=0x2 inode=0x0
              sh-131     [006] ...1.  1945.714358: vfs_open__exit: (do_open+0x274/0x3d0 <- vfs_open) mode=0x4d801e inode=0xffff888008470168
             cat-143     [007] ...1.  1945.717949: vfs_open__entry: (vfs_open+0x4/0x40) mode=0x1 inode=0x0
             cat-143     [007] ...1.  1945.717956: vfs_open__exit: (do_open+0x274/0x3d0 <- vfs_open) mode=0x4a801d inode=0xffff888005f78d28
             cat-143     [007] ...1.  1945.720616: vfs_open__entry: (vfs_open+0x4/0x40) mode=0x1 inode=0x0
             cat-143     [007] ...1.  1945.728263: vfs_open__exit: (do_open+0x274/0x3d0 <- vfs_open) mode=0xa800d inode=0xffff888004ada8d8

```
**您可以看到 `file**
: f_mode` and `file::f_inode` are updated in `vfs_open()。

