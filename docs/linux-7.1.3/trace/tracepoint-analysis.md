## Notes Analysing Behaviour 使用 事件 Tracepoints

:Author: Mel Gorman (PCL information heavily 基于 email 来自 Ingo Molnar)

## 1. Introduction


Tracepoints (参见 Documentation/trace/tracepoints.rst) 使用 
creating custom 内核 模块 注册 probe 函数 使用 the 事件
tracing infrastructure.

Simplistically, tracepoints represent 重要 事件 
taken conjunction 其他 tracepoints build 一"Big Picture" 
什going 之内 the 系统. 存在 一large 数字 方法 用于
gathering interpreting 这些 事件. Lacking 任何 电流 Best Practises,
document describes 一the 方法 使用.

document assumes debugfs mounted /sys/内核/debug 
the appropriate tracing 选项 具有 已经 configured 进入 the 内核. 它是
assumed the PCL tool tools/perf 具有 已经 installed 您的 path.

## 2. Listing 可用 事件


### 2.1 标准 Utilities


全部 可能 事件 visible 来自 /sys/内核/tracing/事件. Simply
```

  $ find /sys/kernel/tracing/events -type d

```
give 一fair indication the 数字 事件 可用.

### 2.2 PCL (性能 Counters 用于 Linux)


Discovery enumeration 全部 counters 事件, including tracepoints,
可用 the perf tool. Getting 一列出 可用 事件 一
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


参见 Documentation/trace/事件.rst 用于 一proper description 如何 事件
已启system-wide. 一short 示例 enabling 全部 事件 related
```

  $ for i in `find /sys/kernel/tracing/events -name "enable" | grep mm_`; do echo 1 > $i; done

```
### 3.2 System-Wide 事件 Enabling SystemTap


SystemTap, tracepoints accessible 使用 the 内核.trace() 函数
call. The 以下 一示例 reports every 5 seconds 什进程
鏇炬槸 allocating the 椤。
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
### 3.3 System-Wide 事件 Enabling PCL


specifying the -一switch analysing sleep, the system-wide 事件
用于 一duration time examined.
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
Similarly, one 可以 execute 一shell exit 作为 desired get 一report
鍦，璇?point.

### 3.4 本地 事件 Enabling


Documentation/trace/ftrace.rst describes 如何 启用 事件 一per-thread
basis 使用 set_ftrace_pid.

### 3.5 本地 事件 Enablement PCL


事件 activated tracked 用于 the duration 一进程 一本地
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


Documentation/trace/ftrace.rst covers in-depth 如何 filter 事件 
ftrace.  Obviously 使用 grep awk trace_pipe 一选项 作为 well
作为 任何 script reading trace_pipe.

## 5. Analysing 事件 Variances PCL


任何 workload exhibit variances 之间 runs 重要
know 什the 标准 deviation  large, 这是 left the
性能 analyst 执行 hand. the 事件 the discrete 事件
occurrences useful the 性能 analyst, 然后 perf 使用.
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
the 事件 一higher-level 事件 必需 depends 一
aggregation discrete 事件, 然后 一script 将会 需developed.

使用 --repeat, 它是 可能 view 如何 事件 fluctuating 在…上
time 一system-wide basis 使用 -一sleep.
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
## 6. Higher-Level Analysis 涓?Helper Scripts


事件 已启the 事件 triggering 读取 来自
/sys/内核/tracing/trace_pipe human-readable 格式 尽管 binary
选项 exist 作为 well. post-processing the 输出, further information 
gathered on-line 作为 appropriate. 示例 post-processing 可能 包含

  - Reading information 来自 /proc 用于 the PID triggered the 事件
  - Deriving 一higher-level 事件 来自 一系列 lower-level 事件.
  - Calculating latencies 之间 two 事件

Documentation/trace/postprocess/trace-pagealloc-postprocess.pl 一示例
script 读取 trace_pipe 来自 STDIN 一copy 一trace. 使用
on-line, interrupted 一generate 一report exiting
鍜?twice 鍒?exit.

Simplistically, the script just reads STDIN counts up 事件 
执行 更多 例如

  - Derive high-level 事件 来自 许多 low-level 事件. 一数字 
    freed the 主要 allocator 来自 the per-CPU 列表, recognises
    作为 one per-CPU drain even though 存在 特定 tracepoint
    用于 事件
  - aggregate 基于 PID 各个 进程 数字
  - the 事件 内存 getting externally fragmented, reports
    是否 the fragmentation 事件 曾是 severe moderate.
  - receiving 一事件 关于 一PID, record the parent 曾是 因此
    large numbers 事件 coming 来自 very short-lived
    进程, the parent 进程 responsible 用于 creating 全部 the helpers
    鍙，涓?identified

## 7. Lower-Level Analysis 涓?PCL


那里 一requirement identify 什函数 之内 一program
曾是 generating 事件 之内 the 内核. begin sort analysis, the
数据 必须 recorded. the time writing, 必需 root:
```

  $ perf record -c 1 \
	-e kmem:mm_page_alloc -e kmem:mm_page_free \
	-e kmem:mm_page_free_batched \
	./hackbench 10
  Time: 0.894
  [ perf record: Captured and wrote 0.733 MB perf.data (~32010 samples) ]

```
注意 the 使用 '-c 1' set the 事件 period sample. The 默认 sample
period 鏄?quite high 鍒?minimise overhead 浣?the information collected 鍙，涓。
very coarse 因此.

record outputted 一文件 called perf.数据 analysed 使用
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
根据  the vast majority 事件 triggered 事件
之内 the VDSO. 简binaries, 通常 the case 因此 let's
take 一slightly 不同 示例. the course writing  曾是
noticed X 曾是 generating 一insane amount allocations 因此 let's look
鍦，瀹。
```

  $ perf record -c 1 -f \
		-e kmem:mm_page_alloc -e kmem:mm_page_free \
		-e kmem:mm_page_free_batched \
		-p `pidof X`

```
曾是 interrupted 之后 一少量 seconds 
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
因此, almost half the 事件 occurring 一 get 一idea 
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
参见 何处 之内 the 函数 pixmanFillsse2 things going wrong:
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
一glance, looks 类似 the time 正在 spent copying pixmaps 
the   Further investigation 将会 needed determine 为何 pixmaps
正在 copied around 因此 much 一starting point 将会 take 一
ancient build libpixmap 超出 the path 何处 曾是 totally
forgotten 关于 来自 months ago!
