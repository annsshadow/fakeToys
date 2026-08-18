## 浣跨敤 the tracer 鐢ㄤ簬 debugging


Copyright 2024 Google LLC.

:Author:   Steven Rostedt <rostedt@goodmis.org>
:License:  The GNU Free Documentation License, 鐗堟湰 1.2
          (dual licensed 鍦ㄢ€︿笅 the GPL v2)

- Written 鐢ㄤ簬: 6.12

### Introduction

The tracing infrastructure 鍙?涓?very useful 鐢ㄤ簬 debugging the Linux
鍐呮牳. 姝?document 鏄?涓€涓?place 鍒?add 鍚勭 鏂规硶 鐨?浣跨敤 the tracer
鐢ㄤ簬 debugging.

```

 $ sudo mount -t tracefs tracefs /sys/kernel/tracing


```
### 浣跨敤 trace_printk()


trace_printk() 鏄?涓€涓?very lightweight utility 璇?鍙?涓?浣跨敤 鍦?浠讳綍 涓婁笅鏂?
inside the 鍐呮牳, 涓?the 寮傚父 鐨?"noinstr" sections. 瀹?鍙?涓?浣跨敤
鍦?姝ｅ父, softirq, 涓柇 鍜?even NMI 涓婁笅鏂? The trace 鏁版嵁 鏄?
written 鍒?the tracing ring 缂撳啿鍖?鍦?涓€涓?lockless way. 鍒?make 瀹?even
lighter weight, 褰?鍙兘, 瀹?灏?浠?record the 鎸囬拡 鍒?the 鏍煎紡
瀛楃涓? 鍜?save the raw arguments 杩涘叆 the 缂撳啿鍖? The 鏍煎紡 鍜?the
arguments 灏?涓?post processed 褰?the ring 缂撳啿鍖?鏄?璇诲彇. 姝?way the
trace_printk() 鏍煎紡 conversions 鏄?涓?宸插畬鎴?鏈熼棿 the hot path, 浣曞
the trace 鏄?姝ｅ湪 recorded.

trace_printk() 鏄?meant 浠?鐢ㄤ簬 debugging, 鍜?搴斿綋 浠庝笉 涓?added 杩涘叆
涓€涓?瀛愮郴缁?鐨?the 鍐呮牳. 鑻?鎮?闇€瑕?debugging traces, add trace 浜嬩欢
鏀逛负. 鑻?涓€涓?trace_printk() 鏄?found 鍦?the 鍐呮牳, the 浠ヤ笅 灏?
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
### Debugging 鍐呮牳 crashes

瀛樺湪 鍚勭 鏂规硶 鐨?acquiring the 鐘舵€?鐨?the 绯荤粺 褰?涓€涓?鍐呮牳
crash occurs. 姝?鍙互 涓?鏉ヨ嚜 the oops message 鍦?printk, 鎴?one 鍙互
浣跨敤 kexec/kdump. 浣?杩欎簺 just 鏄剧ず 浠€涔?happened 鍦?the time 鐨?the crash.
瀹?鍙?涓?very useful 鍦?knowing 浠€涔?happened up 鍒?the point 鐨?the crash.
The tracing ring 缂撳啿鍖? 榛樿鎯呭喌涓? 鏄?涓€涓?circular 缂撳啿鍖?璇?灏?
overwrite older 浜嬩欢 涓?newer ones. 褰?涓€涓?crash happens, the content 鐨?
the ring 缂撳啿鍖?灏?涓?鍏ㄩ儴 the 浜嬩欢 璇?lead up 鍒?the crash.

瀛樺湪 鑻ュ共 鍐呮牳 鍛戒护 line 鍙傛暟 璇?鍙?涓?浣跨敤 鍒?help 鍦?
姝? The 绗竴 鏄?"ftrace_dump_鍦╛oops". 姝?灏?dump the tracing ring
缂撳啿鍖?褰?涓€涓?oops occurs 鍒?the console. 姝?鍙?涓?useful 鑻?the console
鏄?姝ｅ湪 logged somewhere. 鑻?涓€涓?涓茶 console 鏄?浣跨敤, 瀹?鍙?涓?prudent 鍒?
纭繚 the ring 缂撳啿鍖?鏄?relatively small, 鍚﹀垯 the dumping 鐨?the
ring 缂撳啿鍖?鍙?take 鑻ュ共 minutes 鍒?hours 鍒?finish. 姝ゅ's 涓€涓?绀轰緥
```

  ftrace_dump_on_oops trace_buf_size=50K

```
娉ㄦ剰, the tracing 缂撳啿鍖?鏄?made up 鐨?姣?CPU 缂撳啿鍖?浣曞 姣忎釜 鐨?杩欎簺
缂撳啿鍖?鏄?broken up 杩涘叆 sub-buffers 璇?鏄?榛樿鎯呭喌涓?椤礯澶у皬. The
涓婃枃 trace_buf_澶у皬 閫夐」 涓婃枃 sets 姣忎釜 鐨?the 姣?CPU 缂撳啿鍖?鍒?50K,
鍥犳, 鍦?涓€涓?machine 涓?8 CPUs, 璇?s actually 400K 鎬昏.

### Persistent 缂撳啿鍖?across boots

鑻?the 绯荤粺 鍐呭瓨 allows 瀹? the tracing ring 缂撳啿鍖?鍙?涓?specified 鍦?
涓€涓?鐗瑰畾 location 鍦?鍐呭瓨. 鑻?the location 鏄?the 鐩稿悓 across boots 鍜?
the 鍐呭瓨 鏄?涓?modified, the tracing 缂撳啿鍖?鍙?涓?retrieved 鏉ヨ嚜 the
浠ヤ笅 boot. 閭ｉ噷's two ways 鍒?reserve 鍐呭瓨 鐢ㄤ簬 the 浣跨敤 鐨?the ring
缂撳啿鍖?

The 鏇村 reliable way (鍦?x86) 鏄?鍒?reserve 鍐呭瓨 涓?the "memmap" 鍐呮牳
鍛戒护 line 閫夐」 鍜?鐒跺悗 浣跨敤 璇?鍐呭瓨 鐢ㄤ簬 the trace_瀹炰緥. 姝?
闇€瑕?涓€涓?浣?鐨?knowledge 鐨?the 鐗╃悊 鍐呭瓨 layout 鐨?the 绯荤粺. The
advantage 鐨?浣跨敤 姝?鏂规硶, 鏄?璇?the 鍐呭瓨 鐢ㄤ簬 the ring 缂撳啿鍖?灏?
```

  memmap==12M$0x284500000 trace_instance=boot_map@0x284500000:12M

```
The memmap 涓婃枃 reserves 12 megabytes 鐨?鍐呭瓨 鍦?the 鐗╃悊 鍐呭瓨
location 0x284500000. 鐒跺悗 the trace_瀹炰緥 閫夐」 灏?鍒涘缓 涓€涓?trace
瀹炰緥 "boot_map" 鍦?璇?鐩稿悓 location 涓?the 鐩稿悓 amount 鐨?鍐呭瓨
reserved. 浣滀负 the ring 缂撳啿鍖?鏄?broke up 杩涘叆 姣?CPU 缂撳啿鍖? the 12
megabytes 灏?涓?broken up evenly 涔嬮棿 閭ｄ簺 CPUs. 鑻?鎮?鍏锋湁 8 CPUs,
姣忎釜 姣?CPU ring 缂撳啿鍖?灏?涓?1.5 megabytes 鍦?澶у皬. 娉ㄦ剰, 璇?涔?
鍖呭惈 meta 鏁版嵁, 鍥犳 the amount 鐨?鍐呭瓨 actually 浣跨敤 鐢?the ring 缂撳啿鍖?
灏?涓?slightly smaller.

Another 鏇村 generic 浣?less robust way 鍒?allocate 涓€涓?ring 缂撳啿鍖?鏄犲皠
```

  reserve_mem=12M:4096:trace trace_instance=boot_map@trace

```
The reserve_mem 閫夐」 涓婃枃 灏?find 12 megabytes 璇?鏄?鍙敤 鍦?
boot up, 鍜?align 瀹?鐢?4096 bytes. 瀹?灏?label 姝?鍐呭瓨 浣滀负 "trace"
璇?鍙?涓?浣跨敤 鐢?绋嶅悗 鍛戒护 line 閫夐」.

The trace_瀹炰緥 閫夐」 creates 涓€涓?"boot_map" 瀹炰緥 鍜?灏?浣跨敤 the
鍐呭瓨 reserved 鐢?reserve_mem 璇?鏇炬槸 labeled 浣滀负 "trace". 姝?鏂规硶 鏄?
鏇村 generic 浣?鍙?涓?涓?浣滀负 reliable. 鐢变簬 KASLR, the 鍐呭瓨 reserved
鐢?reserve_mem 鍙?涓?涓?located 鍦?the 鐩稿悓 location. 鑻?姝?happens,
鐒跺悗 the ring 缂撳啿鍖?灏?涓?涓?鏉ヨ嚜 the 鍓嶄竴涓?boot 鍜?灏?涓?reset.

鏈夋椂, 鐢?浣跨敤 涓€涓?larger alignment, 瀹?鍙?keep KASLR 鏉ヨ嚜 moving things
around 鍦?姝ょ被 涓€涓?way 璇?瀹?灏?move the location 鐨?the reserve_mem. 鐢?
浣跨敤 涓€涓?larger alignment, 鎮?鍙?find better 璇?the 缂撳啿鍖?鏄?鏇村
```

  reserve_mem=12M:0x2000000:trace trace_instance=boot_map@trace

```
鍦?boot up, the 鍐呭瓨 reserved 鐢ㄤ簬 the ring 缂撳啿鍖?鏄?validated. 瀹?灏?go
through 涓€涓?绯诲垪 鐨?tests 鍒?纭繚 璇?the ring 缂撳啿鍖?鍖呭惈 valid
鏁版嵁. 鑻?瀹冩槸, 瀹?灏?鐒跺悗 set 瀹?up 鍒?涓?鍙敤 鍒?璇诲彇 鏉ヨ嚜 the
瀹炰緥. 鑻?瀹?fails 浠讳綍 鐨?the tests, 瀹?灏?clear the entire ring 缂撳啿鍖?
鍜?initialize 瀹?浣滀负 鏂?

The layout 鐨?姝?mapped 鍐呭瓨 鍙?涓?涓?consistent 鏉ヨ嚜 鍐呮牳 鍒?
鍐呮牳, 鍥犳 浠?the 鐩稿悓 鍐呮牳 鏄?guaranteed 鍒?work 鑻?the 鏄犲皠 鏄?
preserved. Switching 鍒?涓€涓?涓嶅悓 鍐呮牳 鐗堟湰 鍙?find 涓€涓?涓嶅悓
layout 鍜?mark the 缂撳啿鍖?浣滀负 invalid.

NB: 涓よ€?the mapped 鍦板潃 鍜?澶у皬 蹇呴』 涓?椤?aligned 鐢ㄤ簬 the architecture.

### 浣跨敤 trace_printk() 鍦?the boot 瀹炰緥

榛樿鎯呭喌涓? the content 鐨?trace_printk() goes 杩涘叆 the top level tracing
瀹炰緥. 浣?姝?瀹炰緥 鏄?浠庝笉 preserved across boots. 鍒?鍏锋湁 the
trace_printk() content, 鍜?涓€浜?鍏朵粬 鍐呴儴 tracing go 鍒?the preserved
缂撳啿鍖?(绫讳技 dump stacks), 浠讳竴涓?set the 瀹炰緥 鍒?涓?the trace_printk()
destination 鏉ヨ嚜 the 鍐呮牳 鍛戒护 line, 鎴?set 瀹?涔嬪悗 boot up 閫氳繃 the
trace_printk_dest 閫夐」.

```

  echo 1 > /sys/kernel/tracing/instances/boot_map/options/trace_printk_dest

```
```

  reserve_mem=12M:4096:trace trace_instance=boot_map^traceprintk^traceoff@trace

```
鑻?璁剧疆 瀹?鏉ヨ嚜 the 鍐呮牳 鍛戒护 line, 瀹冩槸 recommended 鍒?涔?
绂佺敤 tracing 涓?the "traceoff" 鏍囧織, 鍜?鍚敤 tracing 涔嬪悗 boot up.
鍚﹀垯 the trace 鏉ヨ嚜 the 澶у鏁?recent boot 灏?涓?mixed 涓?the trace
鏉ヨ嚜 the 鍓嶄竴涓?boot, 鍜?鍙?make 瀹?confusing 鍒?璇诲彇.

### 浣跨敤 涓€涓?backup 瀹炰緥 鐢ㄤ簬 keeping 鍓嶄竴涓?boot 鏁版嵁


瀹冩槸 涔?鍙兘 鍒?record trace 鏁版嵁 鍦?绯荤粺 boot time 鐢?specifying
浜嬩欢 涓?the persistent ring 缂撳啿鍖? 浣?鍦?姝?case the 鏁版嵁 涔嬪墠 the
reboot 灏?涓?lost 涔嬪墠 瀹?鍙?涓?璇诲彇. 姝?problem 鍙?涓?solved 鐢?涓€涓?
```

  reserve_mem=12M:4096:trace trace_instance=boot_map@trace,sched,irq trace_instance=backup=boot_map

```
鍦?boot up, the 鍓嶄竴涓?鏁版嵁 鍦?the "boot_map" 鏄?copied 鍒?the "backup"
瀹炰緥, 鍜?the "sched:**" 鍜?"irq:**" 浜嬩欢 鐢ㄤ簬 the 鐢垫祦 boot 鏄?traced
鍦?the "boot_map". 浠庤€?the 鐢ㄦ埛 鍙?璇诲彇 the 鍓嶄竴涓?boot 鏁版嵁 鏉ヨ嚜 the "backup"
瀹炰緥 鏃?stopping the trace.

娉ㄦ剰 璇?姝?"backup" 瀹炰緥 鏄?readonly, 鍜?灏?涓?removed automatically
鑻?鎮?clear the trace 鏁版嵁 鎴?璇诲彇 out 鍏ㄩ儴 trace 鏁版嵁 鏉ヨ嚜 the "trace_pipe"
鎴?the "trace_pipe_raw" 鏂囦欢.
