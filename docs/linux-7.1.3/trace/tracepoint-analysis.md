## Notes 在 Analysing Behaviour 使用 事件 和 Tracepoints

:Author: Mel Gorman (PCL information heavily 基于 email 来自 Ingo Molnar)

## 1. Introduction


Tracepoints (参见 Documentation/trace/tracepoints.rst) 可 为 使用 无
creating custom 内核 模块 到 注册 probe 函数 使用 the 事件
tracing infrastructure.

Simplistically, tracepoints represent 重要 事件 该 可 为
taken 在 conjunction 与 其他 tracepoints 到 build 一个 "Big Picture" 的
什么 是 going 在 之内 the 系统. 存在 一个 large 数字 的 方法 用于
gathering 和 interpreting 这些 事件. Lacking 任何 电流 Best Practises,
此 document describes 一些 的 the 方法 该 可 为 使用.

此 document assumes 该 debugfs 是 mounted 在 /sys/内核/debug 和 该
the appropriate tracing 选项 具有 已经 configured 进入 the 内核. 它是
assumed 该 the PCL tool tools/perf 具有 已经 installed 和 是 在 您的 path.

## 2. Listing 可用 事件


### 2.1 标准 Utilities


全部 可能 事件 是 visible 来自 /sys/内核/tracing/事件. Simply
```

  $ find /sys/kernel/tracing/events -type d

```
将 give 一个 fair indication 的 the 数字 的 事件 可用.

### 2.2 PCL (性能 Counters 用于 Linux)


Discovery 和 enumeration 的 全部 counters 和 事件, including tracepoints,
是 可用 与 the perf tool. Getting 一个 列出 的 可用 事件 是 一个
```

  $ perf list 2>&1 | grep Tracepoint
  ext4:ext4_free_inode                     [Tracepoint event]
  ext4:ext4_request_inode                  [Tracepoint event]
  ext4:ext4_allocate_inode                 [Tracepoint event]
  ext4:ext4_write_begin                    [Tracepoint event]
  ext4:ext4_ordered_write_end              [Tracepoint event]
  [ .... remaining output snipped .... ]


```
## 3. Enabling 事件


### 3.1 System-Wide 事件 Enabling


参见 Documentation/trace/事件.rst 用于 一个 proper description 在 如何 事件
可 为 已启用 system-wide. 一个 short 示例 的 enabling 全部 事件 related
```

  $ for i in `find /sys/kernel/tracing/events -name "enable" | grep mm_`; do echo 1 > $i; done

```
### 3.2 System-Wide 事件 Enabling 与 SystemTap


在 SystemTap, tracepoints 是 accessible 使用 the 内核.trace() 函数
call. The 以下 是 一个 示例 该 reports every 5 seconds 什么 进程
曾是 allocating the 页.
```

  global page_allocs

  probe kernel.trace("mm_page_alloc") {
  	page_allocs[execname()]++
  }

  function print_count() {
  	printf ("%-25s %-s\n", "#Pages Allocated", "Process Name")
  	foreach (proc in page_allocs-)
  		printf("%-25d %s\n", page_allocs[proc], proc)
  	printf ("\n")
  	delete page_allocs
  }

  probe timer.s(5) {
          print_count()
  }

```
### 3.3 System-Wide 事件 Enabling 与 PCL


由 specifying the -一个 switch 和 analysing sleep, the system-wide 事件
用于 一个 duration 的 time 可 为 examined.
```

 $ perf stat -a \
	-e kmem:mm_page_alloc -e kmem:mm_page_free \
	-e kmem:mm_page_free_batched \
	sleep 10
 Performance counter stats for 'sleep 10':

           9630  kmem:mm_page_alloc
           2143  kmem:mm_page_free
           7424  kmem:mm_page_free_batched

   10.002577764  seconds time elapsed

```
Similarly, one 可以 execute 一个 shell 和 exit 它 作为 desired 到 get 一个 report
在 该 point.

### 3.4 本地 事件 Enabling


Documentation/trace/ftrace.rst describes 如何 到 启用 事件 在 一个 per-thread
basis 使用 set_ftrace_pid.

### 3.5 本地 事件 Enablement 与 PCL


事件 可 为 activated 和 tracked 用于 the duration 的 一个 进程 在 一个 本地
basis 使用 PCL 例如 follows.
```

  $ perf stat -e kmem:mm_page_alloc -e kmem:mm_page_free \
		 -e kmem:mm_page_free_batched ./hackbench 10
  Time: 0.909

    Performance counter stats for './hackbench 10':

          17803  kmem:mm_page_alloc
          12398  kmem:mm_page_free
           4827  kmem:mm_page_free_batched

    0.973913387  seconds time elapsed

```
## 4. 事件 Filtering


Documentation/trace/ftrace.rst covers in-depth 如何 到 filter 事件 在
ftrace.  Obviously 使用 grep 和 awk 的 trace_pipe 是 一个 选项 作为 well
作为 任何 script reading trace_pipe.

## 5. Analysing 事件 Variances 与 PCL


任何 workload 可 exhibit variances 之间 runs 和 它 可 为 重要
到 know 什么 the 标准 deviation 是. 由 和 large, 这是 left 到 the
性能 analyst 到 执行 它 由 hand. 在 the 事件 该 the discrete 事件
occurrences 是 useful 到 the 性能 analyst, 然后 perf 可 为 使用.
```

  $ perf stat --repeat 5 -e kmem:mm_page_alloc -e kmem:mm_page_free
			-e kmem:mm_page_free_batched ./hackbench 10
  Time: 0.890
  Time: 0.895
  Time: 0.915
  Time: 1.001
  Time: 0.899

   Performance counter stats for './hackbench 10' (5 runs):

          16630  kmem:mm_page_alloc         ( +-   3.542% )
          11486  kmem:mm_page_free	    ( +-   4.771% )
           4730  kmem:mm_page_free_batched  ( +-   2.325% )

    0.982653002  seconds time elapsed   ( +-   1.448% )

```
在 the 事件 该 一些 higher-level 事件 是 必需 该 depends 在 一些
aggregation 的 discrete 事件, 然后 一个 script 将会 需要 到 为 developed.

使用 --repeat, 它是 也 可能 到 view 如何 事件 是 fluctuating 在…上
time 在 一个 system-wide basis 使用 -一个 和 sleep.
```

  $ perf stat -e kmem:mm_page_alloc -e kmem:mm_page_free \
		-e kmem:mm_page_free_batched \
		-a --repeat 10 \
		sleep 1
  Performance counter stats for 'sleep 1' (10 runs):

           1066  kmem:mm_page_alloc         ( +-  26.148% )
            182  kmem:mm_page_free          ( +-   5.464% )
            890  kmem:mm_page_free_batched  ( +-  30.079% )

    1.002251757  seconds time elapsed   ( +-   0.005% )

```
## 6. Higher-Level Analysis 与 Helper Scripts


当 事件 是 已启用 the 事件 该 是 triggering 可 为 读取 来自
/sys/内核/tracing/trace_pipe 在 human-readable 格式 尽管 binary
选项 exist 作为 well. 由 post-processing the 输出, further information 可
为 gathered on-line 作为 appropriate. 示例 的 post-processing 可能 包含

  - Reading information 来自 /proc 用于 the PID 该 triggered the 事件
  - Deriving 一个 higher-level 事件 来自 一个 系列 的 lower-level 事件.
  - Calculating latencies 之间 two 事件

Documentation/trace/postprocess/trace-pagealloc-postprocess.pl 是 一个 示例
script 该 可 读取 trace_pipe 来自 STDIN 或 一个 copy 的 一个 trace. 当 使用
on-line, 它 可 为 interrupted 一旦 到 generate 一个 report 无 exiting
和 twice 到 exit.

Simplistically, the script just reads STDIN 和 counts up 事件 但 它
也 可 执行 更多 例如

  - Derive high-level 事件 来自 许多 low-level 事件. 若 一个 数字 的 页
    是 freed 到 the 主要 allocator 来自 the per-CPU 列表, 它 recognises
    该 作为 one per-CPU drain even though 存在 无 特定 tracepoint
    用于 该 事件
  - 它 可 aggregate 基于 PID 或 各个 进程 数字
  - 在 the 事件 内存 是 getting externally fragmented, 它 reports
    在 是否 the fragmentation 事件 曾是 severe 或 moderate.
  - 当 receiving 一个 事件 关于 一个 PID, 它 可 record 谁 the parent 曾是 因此
    该 若 large numbers 的 事件 是 coming 来自 very short-lived
    进程, the parent 进程 responsible 用于 creating 全部 the helpers
    可 为 identified

## 7. Lower-Level Analysis 与 PCL


那里 可 也 为 一个 requirement 到 identify 什么 函数 之内 一个 program
曾是 generating 事件 之内 the 内核. 到 begin 此 sort 的 analysis, the
数据 必须 为 recorded. 在 the time 的 writing, 此 必需 root:
```

  $ perf record -c 1 \
	-e kmem:mm_page_alloc -e kmem:mm_page_free \
	-e kmem:mm_page_free_batched \
	./hackbench 10
  Time: 0.894
  [ perf record: Captured and wrote 0.733 MB perf.data (~32010 samples) ]

```
注意 the 使用 的 '-c 1' 到 set the 事件 period 到 sample. The 默认 sample
period 是 quite high 到 minimise overhead 但 the information collected 可 为
very coarse 因此.

此 record outputted 一个 文件 called perf.数据 其 可 为 analysed 使用
perf report.
```

  $ perf report
  # Samples: 30922
  #
  # Overhead    Command                     Shared Object
  # ........  .........  ................................
  #
      87.27%  hackbench  [vdso]
       6.85%  hackbench  /lib/i686/cmov/libc-2.9.so
       2.62%  hackbench  /lib/ld-2.9.so
       1.52%       perf  [vdso]
       1.22%  hackbench  ./hackbench
       0.48%  hackbench  [kernel]
       0.02%       perf  /lib/i686/cmov/libc-2.9.so
       0.01%       perf  /usr/bin/perf
       0.01%       perf  /lib/ld-2.9.so
       0.00%  hackbench  /lib/i686/cmov/libpthread-2.9.so
  #
  # (For more details, try: perf report --sort comm,dso,symbol)
  #

```
根据 此, the vast majority 的 事件 triggered 在 事件
之内 the VDSO. 与 简单 binaries, 此 将 通常 为 the case 因此 let's
take 一个 slightly 不同 示例. 在 the course 的 writing 此, 它 曾是
noticed 该 X 曾是 generating 一个 insane amount 的 页 allocations 因此 let's look
在 它:
```

  $ perf record -c 1 -f \
		-e kmem:mm_page_alloc -e kmem:mm_page_free \
		-e kmem:mm_page_free_batched \
		-p `pidof X`

```
此 曾是 interrupted 之后 一个 少量 seconds 和
```

  $ perf report
  # Samples: 27666
  #
  # Overhead  Command                            Shared Object
  # ........  .......  .......................................
  #
      51.95%     Xorg  [vdso]
      47.95%     Xorg  /opt/gfx-test/lib/libpixman-1.so.0.13.1
       0.09%     Xorg  /lib/i686/cmov/libc-2.9.so
       0.01%     Xorg  [kernel]
  #
  # (For more details, try: perf report --sort comm,dso,symbol)
  #

```
因此, almost half 的 the 事件 是 occurring 在 一个 库. 到 get 一个 idea 其
symbol:
```

  $ perf report --sort comm,dso,symbol
  # Samples: 27666
  #
  # Overhead  Command                            Shared Object  Symbol
  # ........  .......  .......................................  ......
  #
      51.95%     Xorg  [vdso]                                   [.] 0x000000ffffe424
      47.93%     Xorg  /opt/gfx-test/lib/libpixman-1.so.0.13.1  [.] pixmanFillsse2
       0.09%     Xorg  /lib/i686/cmov/libc-2.9.so               [.] _int_malloc
       0.01%     Xorg  /opt/gfx-test/lib/libpixman-1.so.0.13.1  [.] pixman_region32_copy_f
       0.01%     Xorg  [kernel]                                 [k] read_hpet
       0.01%     Xorg  /opt/gfx-test/lib/libpixman-1.so.0.13.1  [.] get_fast_path
       0.00%     Xorg  [kernel]                                 [k] ftrace_trace_userstack

```
到 参见 何处 之内 the 函数 pixmanFillsse2 things 是 going wrong:
```

  $ perf annotate pixmanFillsse2
  [ ... ]
    0.00 :         34eeb:       0f 18 08                prefetcht0 (%eax)
         :      }
         :
         :      extern __inline void __attribute__((__gnu_inline__, __always_inline__, _
         :      _mm_store_si128 (__m128i *__P, __m128i __B) :      {
         :        *__P = __B;
   12.40 :         34eee:       66 0f 7f 80 40 ff ff    movdqa %xmm0,-0xc0(%eax)
    0.00 :         34ef5:       ff
   12.40 :         34ef6:       66 0f 7f 80 50 ff ff    movdqa %xmm0,-0xb0(%eax)
    0.00 :         34efd:       ff
   12.39 :         34efe:       66 0f 7f 80 60 ff ff    movdqa %xmm0,-0xa0(%eax)
    0.00 :         34f05:       ff
   12.67 :         34f06:       66 0f 7f 80 70 ff ff    movdqa %xmm0,-0x90(%eax)
    0.00 :         34f0d:       ff
   12.58 :         34f0e:       66 0f 7f 40 80          movdqa %xmm0,-0x80(%eax)
   12.31 :         34f13:       66 0f 7f 40 90          movdqa %xmm0,-0x70(%eax)
   12.40 :         34f18:       66 0f 7f 40 a0          movdqa %xmm0,-0x60(%eax)
   12.31 :         34f1d:       66 0f 7f 40 b0          movdqa %xmm0,-0x50(%eax)

```
在 一个 glance, 它 looks 类似 the time 是 正在 spent copying pixmaps 到
the 卡.  Further investigation 将会 为 needed 到 determine 为何 pixmaps
是 正在 copied around 因此 much 但 一个 starting point 将会 为 到 take 一个
ancient build 的 libpixmap 超出 the 库 path 何处 它 曾是 totally
forgotten 关于 来自 months ago!
