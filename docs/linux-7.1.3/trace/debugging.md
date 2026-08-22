## 使用 the tracer 用于 debugging


Copyright 2024 Google LLC.

:Author:   Steven Rostedt <rostedt@goodmis.org>
:License:  The GNU Free Documentation License, 版本 1.2
          (dual licensed 在…下 the GPL v2)

- Written 用于: 6.12

### Introduction

The tracing infrastructure very useful 用于 debugging the Linux
内核. document 一place add 各种 方法 使用 the tracer
用于 debugging.

```

 $ sudo mount -t tracefs tracefs /sys/kernel/tracing


```
### 使用 trace_printk()


trace_printk() 一very lightweight utility 使用 任何 上下
inside the 内核, the 异常 "noinstr" sections. 使用
正常, softirq, 中断 even NMI 上下 The trace 数据 
written the tracing ring 缓冲一lockless way. make even
lighter weight, 可能, record the 指针 the 格式
字符 save the raw arguments 进入 the 缓冲 The 格式 the
arguments post processed the ring 缓冲读取. way the
trace_printk() 格式 conversions 已完期间 the hot path, 何处
the trace 正在 recorded.

trace_printk() meant 用于 debugging, 应当 从不 added 进入
一子系the 内核. 需debugging traces, add trace 事件
改为. 一trace_printk() found the 内核, the 以下 
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

存在 各种 方法 acquiring the 状the 系统 一内核
crash occurs. 可以 来自 the oops message printk, one 可以
使用 kexec/kdump. 这些 just 显示 什happened the time the crash.
very useful knowing 什happened up the point the crash.
The tracing ring 缓冲 默认情况 一circular 缓冲
overwrite older 事件 newer ones. 一crash happens, the content 
the ring 缓冲全部 the 事件 lead up the crash.

存在 若干 内核 命令 line 参数 使用 help 
 The 第一 "ftrace_dump_在_oops". dump the tracing ring
缓冲一oops occurs the console. useful the console
正在 logged somewhere. 一串行 console 使用, prudent 
确保 the ring 缓冲relatively small, 否则 the dumping the
ring 缓冲take 若干 minutes hours finish. 此处's 一示例
```

  ftrace_dump_on_oops trace_buf_size=50K

```
注意, the tracing 缓冲made up CPU 缓冲何处 每个 这些
缓冲broken up 进入 sub-buffers 默认情况页_大小. The
上文 trace_buf_大小 选项 上文 sets 每个 the CPU 缓冲50K,
因此, 一machine 8 CPUs, s actually 400K 总计.

### Persistent 缓冲across boots

the 系统 内存 allows  the tracing ring 缓冲specified 
一特定 location 内存. the location the 相同 across boots 
the 内存 modified, the tracing 缓冲retrieved 来自 the
以下 boot. 那里's two ways reserve 内存 用于 the 使用 the ring
缓冲

The 更多 reliable way (x86) reserve 内存 the "memmap" 内核
命令 line 选项 然后 使用 内存 用于 the trace_实例. 
需一knowledge the 物理 内存 layout the 系统. The
advantage 使用 方法, the 内存 用于 the ring 缓冲
```

  memmap==12M$0x284500000 trace_instance=boot_map@0x284500000:12M

```
The memmap 上文 reserves 12 megabytes 内存 the 物理 内存
location 0x284500000. 然后 the trace_实例 选项 创建 一trace
实例 "boot_map" 相同 location the 相同 amount 内存
reserved. 作为 the ring 缓冲broke up 进入 CPU 缓冲 the 12
megabytes broken up evenly 之间 那些 CPUs. 具有 8 CPUs,
每个 CPU ring 缓冲1.5 megabytes 大小. 注意, 
包含 meta 数据, 因此 the amount 内存 actually 使用 the ring 缓冲
灏，涓?slightly smaller.

Another 更多 generic less robust way allocate 一ring 缓冲映射
```

  reserve_mem=12M:4096:trace trace_instance=boot_map@trace

```
The reserve_mem 选项 上文 find 12 megabytes 可用 
boot up, align 4096 bytes. label 内存 作为 "trace"
使用 稍后 命令 line 选项.

The trace_实例 选项 creates 一"boot_map" 实例 使用 the
内存 reserved reserve_mem 曾是 labeled 作为 "trace". 方法 
更多 generic 作为 reliable. 由于 KASLR, the 内存 reserved
reserve_mem located the 相同 location. happens,
然后 the ring 缓冲来自 the 前一boot reset.

有时, 使用 一larger alignment, keep KASLR 来自 moving things
around 此类 一way move the location the reserve_mem. 
使用 一larger alignment, find better the 缓冲更多
```

  reserve_mem=12M:0x2000000:trace trace_instance=boot_map@trace

```
boot up, the 内存 reserved 用于 the ring 缓冲validated. go
through 一系列 tests 确保 the ring 缓冲包含 valid
数据. 它是, 然后 set up 可用 读取 来自 the
实例. fails 任何 the tests, clear the entire ring 缓冲
initialize 作为 

The layout mapped 内存 consistent 来自 内核 
内核, 因此 the 相同 内核 guaranteed work the 映射 
preserved. Switching 一不同 内核 版本 find 一不同
layout mark the 缓冲作为 invalid.

NB: 两the mapped 地址 大小 必须 aligned 用于 the architecture.

### 使用 trace_printk() the boot 实例

默认情况 the content trace_printk() goes 进入 the top level tracing
实例. 实例 从不 preserved across boots. 具有 the
trace_printk() content, 一其他 内部 tracing go the preserved
缓冲(类似 dump stacks), 任一set the 实例 the trace_printk()
destination 来自 the 内核 命令 line, set 之后 boot up 通过 the
trace_printk_dest 选项.

```

  echo 1 > /sys/kernel/tracing/instances/boot_map/options/trace_printk_dest

```
```

  reserve_mem=12M:4096:trace trace_instance=boot_map^traceprintk^traceoff@trace

```
设置 来自 the 内核 命令 line, 它是 recommended 
禁用 tracing the "traceoff" 标志, 启用 tracing 之后 boot up.
否则 the trace 来自 the 大多recent boot mixed the trace
来自 the 前一boot, make confusing 读取.

### 使用 一backup 实例 用于 keeping 前一boot 数据


它是 可能 record trace 数据 系统 boot time specifying
事件 the persistent ring 缓冲 case the 数据 之前 the
reboot lost 之前 读取. problem solved 一
```

  reserve_mem=12M:4096:trace trace_instance=boot_map@trace,sched,irq trace_instance=backup=boot_map

```
boot up, the 前一数据 the "boot_map" copied the "backup"
实例, the "sched:**" "irq:**" 事件 用于 the 电流 boot traced
the "boot_map". 从the 用户 读取 the 前一boot 数据 来自 the "backup"
瀹炰緥 鏃?stopping the trace.

注意 "backup" 实例 readonly, removed automatically
clear the trace 数据 读取 out 全部 trace 数据 来自 the "trace_pipe"
the "trace_pipe_raw" 文件.
