## Notes 鍦?Analysing Behaviour 浣跨敤 浜嬩欢 鍜?Tracepoints

:Author: Mel Gorman (PCL information heavily 鍩轰簬 email 鏉ヨ嚜 Ingo Molnar)

## 1. Introduction


Tracepoints (鍙傝 Documentation/trace/tracepoints.rst) 鍙?涓?浣跨敤 鏃?
creating custom 鍐呮牳 妯″潡 鍒?娉ㄥ唽 probe 鍑芥暟 浣跨敤 the 浜嬩欢
tracing infrastructure.

Simplistically, tracepoints represent 閲嶈 浜嬩欢 璇?鍙?涓?
taken 鍦?conjunction 涓?鍏朵粬 tracepoints 鍒?build 涓€涓?"Big Picture" 鐨?
浠€涔?鏄?going 鍦?涔嬪唴 the 绯荤粺. 瀛樺湪 涓€涓?large 鏁板瓧 鐨?鏂规硶 鐢ㄤ簬
gathering 鍜?interpreting 杩欎簺 浜嬩欢. Lacking 浠讳綍 鐢垫祦 Best Practises,
姝?document describes 涓€浜?鐨?the 鏂规硶 璇?鍙?涓?浣跨敤.

姝?document assumes 璇?debugfs 鏄?mounted 鍦?/sys/鍐呮牳/debug 鍜?璇?
the appropriate tracing 閫夐」 鍏锋湁 宸茬粡 configured 杩涘叆 the 鍐呮牳. 瀹冩槸
assumed 璇?the PCL tool tools/perf 鍏锋湁 宸茬粡 installed 鍜?鏄?鍦?鎮ㄧ殑 path.

## 2. Listing 鍙敤 浜嬩欢


### 2.1 鏍囧噯 Utilities


鍏ㄩ儴 鍙兘 浜嬩欢 鏄?visible 鏉ヨ嚜 /sys/鍐呮牳/tracing/浜嬩欢. Simply
```

  $ find /sys/kernel/tracing/events -type d

```
灏?give 涓€涓?fair indication 鐨?the 鏁板瓧 鐨?浜嬩欢 鍙敤.

### 2.2 PCL (鎬ц兘 Counters 鐢ㄤ簬 Linux)


Discovery 鍜?enumeration 鐨?鍏ㄩ儴 counters 鍜?浜嬩欢, including tracepoints,
鏄?鍙敤 涓?the perf tool. Getting 涓€涓?鍒楀嚭 鐨?鍙敤 浜嬩欢 鏄?涓€涓?
```

  $ perf list 2>&1 | grep Tracepoint
  ext4:ext4_free_inode                     [Tracepoint event]
  ext4:ext4_request_inode                  [Tracepoint event]
  ext4:ext4_allocate_inode                 [Tracepoint event]
  ext4:ext4_write_begin                    [Tracepoint event]
  ext4:ext4_ordered_write_end              [Tracepoint event]
  [ .... remaining output snipped .... ]


```
## 3. Enabling 浜嬩欢


### 3.1 System-Wide 浜嬩欢 Enabling


鍙傝 Documentation/trace/浜嬩欢.rst 鐢ㄤ簬 涓€涓?proper description 鍦?濡備綍 浜嬩欢
鍙?涓?宸插惎鐢?system-wide. 涓€涓?short 绀轰緥 鐨?enabling 鍏ㄩ儴 浜嬩欢 related
```

  $ for i in `find /sys/kernel/tracing/events -name "enable" | grep mm_`; do echo 1 > $i; done

```
### 3.2 System-Wide 浜嬩欢 Enabling 涓?SystemTap


鍦?SystemTap, tracepoints 鏄?accessible 浣跨敤 the 鍐呮牳.trace() 鍑芥暟
call. The 浠ヤ笅 鏄?涓€涓?绀轰緥 璇?reports every 5 seconds 浠€涔?杩涚▼
鏇炬槸 allocating the 椤?
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
### 3.3 System-Wide 浜嬩欢 Enabling 涓?PCL


鐢?specifying the -涓€涓?switch 鍜?analysing sleep, the system-wide 浜嬩欢
鐢ㄤ簬 涓€涓?duration 鐨?time 鍙?涓?examined.
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
Similarly, one 鍙互 execute 涓€涓?shell 鍜?exit 瀹?浣滀负 desired 鍒?get 涓€涓?report
鍦?璇?point.

### 3.4 鏈湴 浜嬩欢 Enabling


Documentation/trace/ftrace.rst describes 濡備綍 鍒?鍚敤 浜嬩欢 鍦?涓€涓?per-thread
basis 浣跨敤 set_ftrace_pid.

### 3.5 鏈湴 浜嬩欢 Enablement 涓?PCL


浜嬩欢 鍙?涓?activated 鍜?tracked 鐢ㄤ簬 the duration 鐨?涓€涓?杩涚▼ 鍦?涓€涓?鏈湴
basis 浣跨敤 PCL 渚嬪 follows.
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
## 4. 浜嬩欢 Filtering


Documentation/trace/ftrace.rst covers in-depth 濡備綍 鍒?filter 浜嬩欢 鍦?
ftrace.  Obviously 浣跨敤 grep 鍜?awk 鐨?trace_pipe 鏄?涓€涓?閫夐」 浣滀负 well
浣滀负 浠讳綍 script reading trace_pipe.

## 5. Analysing 浜嬩欢 Variances 涓?PCL


浠讳綍 workload 鍙?exhibit variances 涔嬮棿 runs 鍜?瀹?鍙?涓?閲嶈
鍒?know 浠€涔?the 鏍囧噯 deviation 鏄? 鐢?鍜?large, 杩欐槸 left 鍒?the
鎬ц兘 analyst 鍒?鎵ц 瀹?鐢?hand. 鍦?the 浜嬩欢 璇?the discrete 浜嬩欢
occurrences 鏄?useful 鍒?the 鎬ц兘 analyst, 鐒跺悗 perf 鍙?涓?浣跨敤.
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
鍦?the 浜嬩欢 璇?涓€浜?higher-level 浜嬩欢 鏄?蹇呴渶 璇?depends 鍦?涓€浜?
aggregation 鐨?discrete 浜嬩欢, 鐒跺悗 涓€涓?script 灏嗕細 闇€瑕?鍒?涓?developed.

浣跨敤 --repeat, 瀹冩槸 涔?鍙兘 鍒?view 濡備綍 浜嬩欢 鏄?fluctuating 鍦ㄢ€︿笂
time 鍦?涓€涓?system-wide basis 浣跨敤 -涓€涓?鍜?sleep.
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


褰?浜嬩欢 鏄?宸插惎鐢?the 浜嬩欢 璇?鏄?triggering 鍙?涓?璇诲彇 鏉ヨ嚜
/sys/鍐呮牳/tracing/trace_pipe 鍦?human-readable 鏍煎紡 灏界 binary
閫夐」 exist 浣滀负 well. 鐢?post-processing the 杈撳嚭, further information 鍙?
涓?gathered on-line 浣滀负 appropriate. 绀轰緥 鐨?post-processing 鍙兘 鍖呭惈

  - Reading information 鏉ヨ嚜 /proc 鐢ㄤ簬 the PID 璇?triggered the 浜嬩欢
  - Deriving 涓€涓?higher-level 浜嬩欢 鏉ヨ嚜 涓€涓?绯诲垪 鐨?lower-level 浜嬩欢.
  - Calculating latencies 涔嬮棿 two 浜嬩欢

Documentation/trace/postprocess/trace-pagealloc-postprocess.pl 鏄?涓€涓?绀轰緥
script 璇?鍙?璇诲彇 trace_pipe 鏉ヨ嚜 STDIN 鎴?涓€涓?copy 鐨?涓€涓?trace. 褰?浣跨敤
on-line, 瀹?鍙?涓?interrupted 涓€鏃?鍒?generate 涓€涓?report 鏃?exiting
鍜?twice 鍒?exit.

Simplistically, the script just reads STDIN 鍜?counts up 浜嬩欢 浣?瀹?
涔?鍙?鎵ц 鏇村 渚嬪

  - Derive high-level 浜嬩欢 鏉ヨ嚜 璁稿 low-level 浜嬩欢. 鑻?涓€涓?鏁板瓧 鐨?椤?
    鏄?freed 鍒?the 涓昏 allocator 鏉ヨ嚜 the per-CPU 鍒楄〃, 瀹?recognises
    璇?浣滀负 one per-CPU drain even though 瀛樺湪 鏃?鐗瑰畾 tracepoint
    鐢ㄤ簬 璇?浜嬩欢
  - 瀹?鍙?aggregate 鍩轰簬 PID 鎴?鍚勪釜 杩涚▼ 鏁板瓧
  - 鍦?the 浜嬩欢 鍐呭瓨 鏄?getting externally fragmented, 瀹?reports
    鍦?鏄惁 the fragmentation 浜嬩欢 鏇炬槸 severe 鎴?moderate.
  - 褰?receiving 涓€涓?浜嬩欢 鍏充簬 涓€涓?PID, 瀹?鍙?record 璋?the parent 鏇炬槸 鍥犳
    璇?鑻?large numbers 鐨?浜嬩欢 鏄?coming 鏉ヨ嚜 very short-lived
    杩涚▼, the parent 杩涚▼ responsible 鐢ㄤ簬 creating 鍏ㄩ儴 the helpers
    鍙?涓?identified

## 7. Lower-Level Analysis 涓?PCL


閭ｉ噷 鍙?涔?涓?涓€涓?requirement 鍒?identify 浠€涔?鍑芥暟 涔嬪唴 涓€涓?program
鏇炬槸 generating 浜嬩欢 涔嬪唴 the 鍐呮牳. 鍒?begin 姝?sort 鐨?analysis, the
鏁版嵁 蹇呴』 涓?recorded. 鍦?the time 鐨?writing, 姝?蹇呴渶 root:
```

  $ perf record -c 1 \
	-e kmem:mm_page_alloc -e kmem:mm_page_free \
	-e kmem:mm_page_free_batched \
	./hackbench 10
  Time: 0.894
  [ perf record: Captured and wrote 0.733 MB perf.data (~32010 samples) ]

```
娉ㄦ剰 the 浣跨敤 鐨?'-c 1' 鍒?set the 浜嬩欢 period 鍒?sample. The 榛樿 sample
period 鏄?quite high 鍒?minimise overhead 浣?the information collected 鍙?涓?
very coarse 鍥犳.

姝?record outputted 涓€涓?鏂囦欢 called perf.鏁版嵁 鍏?鍙?涓?analysed 浣跨敤
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
鏍规嵁 姝? the vast majority 鐨?浜嬩欢 triggered 鍦?浜嬩欢
涔嬪唴 the VDSO. 涓?绠€鍗?binaries, 姝?灏?閫氬父 涓?the case 鍥犳 let's
take 涓€涓?slightly 涓嶅悓 绀轰緥. 鍦?the course 鐨?writing 姝? 瀹?鏇炬槸
noticed 璇?X 鏇炬槸 generating 涓€涓?insane amount 鐨?椤?allocations 鍥犳 let's look
鍦?瀹?
```

  $ perf record -c 1 -f \
		-e kmem:mm_page_alloc -e kmem:mm_page_free \
		-e kmem:mm_page_free_batched \
		-p `pidof X`

```
姝?鏇炬槸 interrupted 涔嬪悗 涓€涓?灏戦噺 seconds 鍜?
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
鍥犳, almost half 鐨?the 浜嬩欢 鏄?occurring 鍦?涓€涓?搴? 鍒?get 涓€涓?idea 鍏?
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
鍒?鍙傝 浣曞 涔嬪唴 the 鍑芥暟 pixmanFillsse2 things 鏄?going wrong:
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
鍦?涓€涓?glance, 瀹?looks 绫讳技 the time 鏄?姝ｅ湪 spent copying pixmaps 鍒?
the 鍗?  Further investigation 灏嗕細 涓?needed 鍒?determine 涓轰綍 pixmaps
鏄?姝ｅ湪 copied around 鍥犳 much 浣?涓€涓?starting point 灏嗕細 涓?鍒?take 涓€涓?
ancient build 鐨?libpixmap 瓒呭嚭 the 搴?path 浣曞 瀹?鏇炬槸 totally
forgotten 鍏充簬 鏉ヨ嚜 months ago!
