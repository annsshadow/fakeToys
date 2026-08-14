
## rtla-timerlat-top

### Measures the operating system timer latency


:Manual section: 1

## SYNOPSIS

**rtla timerlat top** [**OPTIONS**] ...

## DESCRIPTION


**rtla timerlat top** 显示来自 **timerlat** tracer 的周期性输出的摘要。它还通过 **osnoise:** tracepoints 提供每个操作系统噪声的信息，可通过选项 **-T** 查看。

## OPTIONS




**--aa-only** **us**

        设置停止追踪条件并运行，但不收集和显示统计信息。
        如果系统命中停止追踪条件，则打印自动分析。该选项有助于降低 rtla timerlat 的 CPU 占用，
        在不收集统计信息开销的情况下启用调试。


## EXAMPLE


在下面的例子中，timerlat tracer 在 cpu **1-23** 上以自动追踪模式启动，并指示 tracer 在出现 **40 us** 延迟或
```

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
在这种情况下，主要因素是处理 **timerlat** 唤醒的 **IRQ 处理程序**所遭受的延迟：**65.52%**。这可能由当前线程屏蔽中断引起，可以在阻塞线程栈跟踪中看到：当前线程（**objtool:49256**）在 btrfs 文件系统中进行 write 系统调用时，通过 mem cgroup 内的 **raw spin lock** 操作禁用了中断。

原始 trace 被保存在 **timerlat_trace.txt** 文件中以备进一步分析。

注意，**rtla timerlat** 是在不改变 **timerlat** tracer 线程优先级的情况下启动的。这通常不需要，因为这些线程默认优先级为 **FIFO:95**，这是实时内核开发者用于分析调度延迟的常用优先级。

### SEE ALSO

**rtla-timerlat**\(1), **rtla-timerlat-hist**\(1)

`Timerlat tracer <https://docs.kernel.org/trace/timerlat-tracer.html>`__

### AUTHOR

Written by Daniel Bristot de Oliveira <bristot@kernel.org>
