
## rtla-timerlat-top

### Measures the operating system timer latency


:Manual section: 1

## SYNOPSIS

**rtla timerlat top** [**OPTIONS**] ...

## DESCRIPTION


**rtla timerlat top** 鏄剧ず鏉ヨ嚜 **timerlat** tracer 鐨勫懆鏈熸€ц緭鍑虹殑鎽樿銆傚畠杩橀€氳繃 **osnoise:** tracepoints 鎻愪緵姣忎釜鎿嶄綔绯荤粺鍣０鐨勪俊鎭紝鍙€氳繃閫夐」 **-T** 鏌ョ湅銆?
## OPTIONS




**--aa-only** **us**

        璁剧疆鍋滄杩借釜鏉′欢骞惰繍琛岋紝浣嗕笉鏀堕泦鍜屾樉绀虹粺璁′俊鎭€?        濡傛灉绯荤粺鍛戒腑鍋滄杩借釜鏉′欢锛屽垯鎵撳嵃鑷姩鍒嗘瀽銆傝閫夐」鏈夊姪浜庨檷浣?rtla timerlat 鐨?CPU 鍗犵敤锛?        鍦ㄤ笉鏀堕泦缁熻淇℃伅寮€閿€鐨勬儏鍐典笅鍚敤璋冭瘯銆?

## EXAMPLE


鍦ㄤ笅闈㈢殑渚嬪瓙涓紝timerlat tracer 鍦?cpu **1-23** 涓婁互鑷姩杩借釜妯″紡鍚姩锛屽苟鎸囩ず tracer 鍦ㄥ嚭鐜?**40 us** 寤惰繜鎴?```

  # timerlat -a 40 -c 1-23 -q
                                     Timer Latency
    0 00:00:12   |          IRQ Timer Latency (us)        |         Thread Timer Latency (us)
  CPU COUNT      |      cur       min       avg       max |      cur       min       avg       max
    1 #12322     |        0         0         1        15 |       10         3         9        31
    2 #12322     |        3         0         1        12 |       10         3         9        23
    3 #12322     |        1         0         1        21 |        8         2         8        34
    4 #12322     |        1         0         1        17 |       10         2        11        33
    5 #12322     |        0         0         1        12 |        8         3         8        25
    6 #12322     |        1         0         1        14 |       16         3        11        35
    7 #12322     |        0         0         1        14 |        9         2         8        29
    8 #12322     |        1         0         1        22 |        9         3         9        34
    9 #12322     |        0         0         1        14 |        8         2         8        24
   10 #12322     |        1         0         0        12 |        9         3         8        24
   11 #12322     |        0         0         0        15 |        6         2         7        29
   12 #12321     |        1         0         0        13 |        5         3         8        23
   13 #12319     |        0         0         1        14 |        9         3         9        26
   14 #12321     |        1         0         0        13 |        6         2         8        24
   15 #12321     |        1         0         1        15 |       12         3        11        27
   16 #12318     |        0         0         1        13 |        7         3        10        24
   17 #12319     |        0         0         1        13 |       11         3         9        25
   18 #12318     |        0         0         0        12 |        8         2         8        20
   19 #12319     |        0         0         1        18 |       10         2         9        28
   20 #12317     |        0         0         0        20 |        9         3         8        34
   21 #12318     |        0         0         0        13 |        8         3         8        28
   22 #12319     |        0         0         1        11 |        8         3        10        22
   23 #12320     |       28         0         1        28 |       41         3        11        41
  rtla timerlat hit stop tracing
  ## CPU 23 hit stop tracing, analyzing it ##
  IRQ handler delay:                                        27.49 us (65.52 %)
  IRQ latency:                                              28.13 us
  Timerlat IRQ duration:                                     9.59 us (22.85 %)
  Blocking thread:                                           3.79 us (9.03 %)
                         objtool:49256                       3.79 us
    Blocking thread stacktrace
                -> timerlat_irq
                -> __hrtimer_run_queues
                -> hrtimer_interrupt
                -> __sysvec_apic_timer_interrupt
                -> sysvec_apic_timer_interrupt
                -> asm_sysvec_apic_timer_interrupt
                -> _raw_spin_unlock_irqrestore
                -> cgroup_rstat_flush_locked
                -> cgroup_rstat_flush_irqsafe
                -> mem_cgroup_flush_stats
                -> mem_cgroup_wb_stats
                -> balance_dirty_pages
                -> balance_dirty_pages_ratelimited_flags
                -> btrfs_buffered_write
                -> btrfs_do_write_iter
                -> vfs_write
                -> __x64_sys_pwrite64
                -> do_syscall_64
                -> entry_SYSCALL_64_after_hwframe
  ------------------------------------------------------------------------
    Thread latency:                                          41.96 us (100%)

  The system has exit from idle latency!
    Max timerlat IRQ latency from idle: 17.48 us in cpu 4
  Saving trace to timerlat_trace.txt

```
鍦ㄨ繖绉嶆儏鍐典笅锛屼富瑕佸洜绱犳槸澶勭悊 **timerlat** 鍞ら啋鐨?**IRQ 澶勭悊绋嬪簭**鎵€閬彈鐨勫欢杩燂細**65.52%**銆傝繖鍙兘鐢卞綋鍓嶇嚎绋嬪睆钄戒腑鏂紩璧凤紝鍙互鍦ㄩ樆濉炵嚎绋嬫爤璺熻釜涓湅鍒帮細褰撳墠绾跨▼锛?*objtool:49256**锛夊湪 btrfs 鏂囦欢绯荤粺涓繘琛?write 绯荤粺璋冪敤鏃讹紝閫氳繃 mem cgroup 鍐呯殑 **raw spin lock** 鎿嶄綔绂佺敤浜嗕腑鏂€?
鍘熷 trace 琚繚瀛樺湪 **timerlat_trace.txt** 鏂囦欢涓互澶囪繘涓€姝ュ垎鏋愩€?
娉ㄦ剰锛?*rtla timerlat** 鏄湪涓嶆敼鍙?**timerlat** tracer 绾跨▼浼樺厛绾х殑鎯呭喌涓嬪惎鍔ㄧ殑銆傝繖閫氬父涓嶉渶瑕侊紝鍥犱负杩欎簺绾跨▼榛樿浼樺厛绾т负 **FIFO:95**锛岃繖鏄疄鏃跺唴鏍稿紑鍙戣€呯敤浜庡垎鏋愯皟搴﹀欢杩熺殑甯哥敤浼樺厛绾с€?
### SEE ALSO

**rtla-timerlat**\(1), **rtla-timerlat-hist**\(1)

`Timerlat tracer <https://docs.kernel.org/trace/timerlat-tracer.html>`__

### AUTHOR

Written by Daniel Bristot de Oliveira <bristot@kernel.org>
