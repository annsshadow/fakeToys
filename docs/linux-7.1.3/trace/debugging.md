## 使用 the tracer 用于 debugging


Copyright 2024 Google LLC.

:Author:   Steven Rostedt <rostedt@goodmis.org>
:License:  The GNU Free Documentation License, 版本 1.2
          (dual licensed 在…下 the GPL v2)

- Written 用于: 6.12

### Introduction

The tracing infrastructure 可 为 very useful 用于 debugging the Linux
内核. 此 document 是 一个 place 到 add 各种 方法 的 使用 the tracer
用于 debugging.

```

 $ sudo mount -t tracefs tracefs /sys/kernel/tracing


```
### 使用 trace_printk()


trace_printk() 是 一个 very lightweight utility 该 可 为 使用 在 任何 上下文
inside the 内核, 与 the 异常 的 "noinstr" sections. 它 可 为 使用
在 正常, softirq, 中断 和 even NMI 上下文. The trace 数据 是
written 到 the tracing ring 缓冲区 在 一个 lockless way. 到 make 它 even
lighter weight, 当 可能, 它 将 仅 record the 指针 到 the 格式
字符串, 和 save the raw arguments 进入 the 缓冲区. The 格式 和 the
arguments 将 为 post processed 当 the ring 缓冲区 是 读取. 此 way the
trace_printk() 格式 conversions 是 不 已完成 期间 the hot path, 何处
the trace 是 正在 recorded.

trace_printk() 是 meant 仅 用于 debugging, 和 应当 从不 为 added 进入
一个 子系统 的 the 内核. 若 您 需要 debugging traces, add trace 事件
改为. 若 一个 trace_printk() 是 found 在 the 内核, the 以下 将
```

  **********************************************************
  **   NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE   **
  **                                                      **
  ** trace_printk() being used. Allocating extra memory.  **
  **                                                      **
  ** This means that this is a DEBUG kernel and it is     **
  ** unsafe for production use.                           **
  **                                                      **
  ** If you see this message and you are not debugging    **
  ** the kernel, report this immediately to your vendor!  **
  **                                                      **
  **   NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE   **
  **********************************************************

```
### Debugging 内核 crashes

存在 各种 方法 的 acquiring the 状态 的 the 系统 当 一个 内核
crash occurs. 此 可以 为 来自 the oops message 在 printk, 或 one 可以
使用 kexec/kdump. 但 这些 just 显示 什么 happened 在 the time 的 the crash.
它 可 为 very useful 在 knowing 什么 happened up 到 the point 的 the crash.
The tracing ring 缓冲区, 默认情况下, 是 一个 circular 缓冲区 该 将
overwrite older 事件 与 newer ones. 当 一个 crash happens, the content 的
the ring 缓冲区 将 为 全部 the 事件 该 lead up 到 the crash.

存在 若干 内核 命令 line 参数 该 可 为 使用 到 help 在
此. The 第一 是 "ftrace_dump_在_oops". 此 将 dump the tracing ring
缓冲区 当 一个 oops occurs 到 the console. 此 可 为 useful 若 the console
是 正在 logged somewhere. 若 一个 串行 console 是 使用, 它 可 为 prudent 到
确保 the ring 缓冲区 是 relatively small, 否则 the dumping 的 the
ring 缓冲区 可 take 若干 minutes 到 hours 到 finish. 此处's 一个 示例
```

  ftrace_dump_on_oops trace_buf_size=50K

```
注意, the tracing 缓冲区 是 made up 的 每 CPU 缓冲区 何处 每个 的 这些
缓冲区 是 broken up 进入 sub-buffers 该 是 默认情况下 页_大小. The
上文 trace_buf_大小 选项 上文 sets 每个 的 the 每 CPU 缓冲区 到 50K,
因此, 在 一个 machine 与 8 CPUs, 该's actually 400K 总计.

### Persistent 缓冲区 across boots

若 the 系统 内存 allows 它, the tracing ring 缓冲区 可 为 specified 在
一个 特定 location 在 内存. 若 the location 是 the 相同 across boots 和
the 内存 是 不 modified, the tracing 缓冲区 可 为 retrieved 来自 the
以下 boot. 那里's two ways 到 reserve 内存 用于 the 使用 的 the ring
缓冲区.

The 更多 reliable way (在 x86) 是 到 reserve 内存 与 the "memmap" 内核
命令 line 选项 和 然后 使用 该 内存 用于 the trace_实例. 此
需要 一个 位 的 knowledge 的 the 物理 内存 layout 的 the 系统. The
advantage 的 使用 此 方法, 是 该 the 内存 用于 the ring 缓冲区 将
```

  memmap==12M$0x284500000 trace_instance=boot_map@0x284500000:12M

```
The memmap 上文 reserves 12 megabytes 的 内存 在 the 物理 内存
location 0x284500000. 然后 the trace_实例 选项 将 创建 一个 trace
实例 "boot_map" 在 该 相同 location 与 the 相同 amount 的 内存
reserved. 作为 the ring 缓冲区 是 broke up 进入 每 CPU 缓冲区, the 12
megabytes 将 为 broken up evenly 之间 那些 CPUs. 若 您 具有 8 CPUs,
每个 每 CPU ring 缓冲区 将 为 1.5 megabytes 在 大小. 注意, 该 也
包含 meta 数据, 因此 the amount 的 内存 actually 使用 由 the ring 缓冲区
将 为 slightly smaller.

Another 更多 generic 但 less robust way 到 allocate 一个 ring 缓冲区 映射
```

  reserve_mem=12M:4096:trace trace_instance=boot_map@trace

```
The reserve_mem 选项 上文 将 find 12 megabytes 该 是 可用 在
boot up, 和 align 它 由 4096 bytes. 它 将 label 此 内存 作为 "trace"
该 可 为 使用 由 稍后 命令 line 选项.

The trace_实例 选项 creates 一个 "boot_map" 实例 和 将 使用 the
内存 reserved 由 reserve_mem 该 曾是 labeled 作为 "trace". 此 方法 是
更多 generic 但 可 不 为 作为 reliable. 由于 KASLR, the 内存 reserved
由 reserve_mem 可 不 为 located 在 the 相同 location. 若 此 happens,
然后 the ring 缓冲区 将 不 为 来自 the 前一个 boot 和 将 为 reset.

有时, 由 使用 一个 larger alignment, 它 可 keep KASLR 来自 moving things
around 在 此类 一个 way 该 它 将 move the location 的 the reserve_mem. 由
使用 一个 larger alignment, 您 可 find better 该 the 缓冲区 是 更多
```

  reserve_mem=12M:0x2000000:trace trace_instance=boot_map@trace

```
在 boot up, the 内存 reserved 用于 the ring 缓冲区 是 validated. 它 将 go
through 一个 系列 的 tests 到 确保 该 the ring 缓冲区 包含 valid
数据. 若 它是, 它 将 然后 set 它 up 到 为 可用 到 读取 来自 the
实例. 若 它 fails 任何 的 the tests, 它 将 clear the entire ring 缓冲区
和 initialize 它 作为 新.

The layout 的 此 mapped 内存 可 不 为 consistent 来自 内核 到
内核, 因此 仅 the 相同 内核 是 guaranteed 到 work 若 the 映射 是
preserved. Switching 到 一个 不同 内核 版本 可 find 一个 不同
layout 和 mark the 缓冲区 作为 invalid.

NB: 两者 the mapped 地址 和 大小 必须 为 页 aligned 用于 the architecture.

### 使用 trace_printk() 在 the boot 实例

默认情况下, the content 的 trace_printk() goes 进入 the top level tracing
实例. 但 此 实例 是 从不 preserved across boots. 到 具有 the
trace_printk() content, 和 一些 其他 内部 tracing go 到 the preserved
缓冲区 (类似 dump stacks), 任一个 set the 实例 到 为 the trace_printk()
destination 来自 the 内核 命令 line, 或 set 它 之后 boot up 通过 the
trace_printk_dest 选项.

```

  echo 1 > /sys/kernel/tracing/instances/boot_map/options/trace_printk_dest

```
```

  reserve_mem=12M:4096:trace trace_instance=boot_map^traceprintk^traceoff@trace

```
若 设置 它 来自 the 内核 命令 line, 它是 recommended 到 也
禁用 tracing 与 the "traceoff" 标志, 和 启用 tracing 之后 boot up.
否则 the trace 来自 the 大多数 recent boot 将 为 mixed 与 the trace
来自 the 前一个 boot, 和 可 make 它 confusing 到 读取.

### 使用 一个 backup 实例 用于 keeping 前一个 boot 数据


它是 也 可能 到 record trace 数据 在 系统 boot time 由 specifying
事件 与 the persistent ring 缓冲区, 但 在 此 case the 数据 之前 the
reboot 将 为 lost 之前 它 可 为 读取. 此 problem 可 为 solved 由 一个
```

  reserve_mem=12M:4096:trace trace_instance=boot_map@trace,sched,irq trace_instance=backup=boot_map

```
在 boot up, the 前一个 数据 在 the "boot_map" 是 copied 到 the "backup"
实例, 和 the "sched:**" 和 "irq:**" 事件 用于 the 电流 boot 是 traced
在 the "boot_map". 从而 the 用户 可 读取 the 前一个 boot 数据 来自 the "backup"
实例 无 stopping the trace.

注意 该 此 "backup" 实例 是 readonly, 和 将 为 removed automatically
若 您 clear the trace 数据 或 读取 out 全部 trace 数据 来自 the "trace_pipe"
或 the "trace_pipe_raw" 文件.
