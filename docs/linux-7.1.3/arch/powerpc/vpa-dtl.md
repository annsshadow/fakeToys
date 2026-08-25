## DTL（Dispatch Trace Log，调度跟踪日志）


Athira Rajeev锛?025 骞?4 鏈?19 鏃。
    :depth: 3


## 基本概述（Basic overview

pseries 的共享处理器逻辑分区（SPLPAR）机器可以使用来自调度跟踪日志（DTL）缓冲区的数据，hypervisor 检索调度（dispatch）和抢占（preempt）事件的日志。利用这些信息，用户可以检索每次调度和抢占发生的时间与原因vpa-dtl PMU 通过 perf 暴露虚拟处理器区（VPA）的 DTL 计数器
## 使用的基础设施（Infrastructure used

VPA DTL PMU 计数器在溢出时不会中断，也不会产生任PMI 中断。因此，使用 hrtimer 来轮DTL 数据。该定时器间隔可由用户通过 sample_period 字段以纳秒为单位提供vpa dtl pmu 为每vpa-dtl pmu 线程添加一hrtimer。DTL（调度跟踪日志）包含关于调度/抢占、入队时间等信息我们直接DTL 缓冲区数据作为辅助缓冲区（auxiliary buffer）的一部分复制，稍后再处理。这将避免在内核空间中创建采样所花费的时间收集调度跟踪日志（DTL）条目的 PMU 驱动利用perf 基础设施中的 AUX 支持。在工具侧，这些数据PERF_RECORD_AUXTRACE 记录的形式提供
为了将每DTL 条目与跨 CPU 的其他事件关联起来，为每CPU 创建一auxtrace_queue。每auxtrace 队列都有一auxtrace 缓冲区数列表所auxtrace 队列都维护在 auxtrace 堆（heap）中。队列根据时间戳排序。在处理不同PERF_RECORD_XX 记录时，perf 记录的时间戳auxtrace 堆中栈顶元素的时间戳进行比较，从而可以将 DTL 事件与其他事件关联起来如果堆中元素的时间戳低于 perf 记录中条目的时间戳，则处auxtrace 队列，以DTL 事件可以与其他事件关联有时一个缓冲区可能只被部分处理。如果另一个事件发生的时间戳大于队列中当前已处理的元素，它将转到下一perf 记录。因此要记录缓冲区的位置，以便下次继续处理。用 auxtrace 缓冲区中最后处理的条目的时间戳更新 auxtrace 堆的时间戳
该基础设施确保调度跟踪日志条目能够与其他事件（sched）关联并一起呈现
## vpa-dtl PMU 使用示例（vpa-dtl PMU example usage

  # ls /sys/devices/vpa_dtl/
  events  format  perf_event_mux_interval_ms  power  subsystem  type  uevent


要使perf record 捕获 DTL 数据
  # ./perf record -a -e sched:\*,vpa_dtl/dtl_all/ -c 1000000000 sleep 1

结果可以使用 perf record 解释。下面是 perf report -D 的片

  # ./perf report -D

存在不同PERF_RECORD_XX 记录。其中与 auxtrace 缓冲区对应的记录包括
1. PERF_RECORD_AUX
   表示 AUX 区域中有新数据可
2. PERF_RECORD_AUXTRACE_INFO
   描述缓冲区中 auxtrace 数据的偏移和大小

3. PERF_RECORD_AUXTRACE
   这是定义 auxtrace 数据的记录，vpa-dtl pmu 的情况下，这里就是调度跟踪日志数据
下面是由 perf report -D 显示PERF_RECORD_AUXTRACE dump 片段


0 0 0x39b10 [0x30]: PERF_RECORD_AUXTRACE size: 0x690  offset: 0  ref: 0  idx: 0  tid: -1  cpu: 0
.
. ... VPA DTL PMU data: size 1680 bytes, entries is 35
.  00000000: boot_tb: 21349649546353231, tb_freq: 512000000
.  00000030: dispatch_reason:decrementer interrupt, preempt_reason:H_CEDE, enqueue_to_dispatch_time:7064, ready_to_enqueue_time:187, waiting_to_ready_time:6611773
.  00000060: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:146, ready_to_enqueue_time:0, waiting_to_ready_time:15359437
.  00000090: dispatch_reason:decrementer interrupt, preempt_reason:H_CEDE, enqueue_to_dispatch_time:4868, ready_to_enqueue_time:232, waiting_to_ready_time:5100709
.  000000c0: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:179, ready_to_enqueue_time:0, waiting_to_ready_time:30714243
.  000000f0: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:197, ready_to_enqueue_time:0, waiting_to_ready_time:15350648
.  00000120: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:213, ready_to_enqueue_time:0, waiting_to_ready_time:15353446
.  00000150: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:212, ready_to_enqueue_time:0, waiting_to_ready_time:15355126
.  00000180: dispatch_reason:decrementer interrupt, preempt_reason:H_CEDE, enqueue_to_dispatch_time:6368, ready_to_enqueue_time:164, waiting_to_ready_time:5104665

以上是如下格式的 dtl 条目的表示：

struct dtl_entry {
        u8      dispatch_reason;
        u8      preempt_reason;
        u16     processor_id;
        u32     enqueue_to_dispatch_time;
        u32     ready_to_enqueue_time;
        u32     waiting_to_ready_time;
        u64     timebase;
        u64     fault_addr;
        u64     srr0;
        u64     srr1;

};

前两个字段表示调度原因和抢占原因。PERF_RECORD_AUXTRACE 记录的后期处理将转换为对用户有意义的数据
## 使用 perf report 可视化调度跟踪日志条目（Visualize the dispatch trace log entries with perf report

  # ./perf record -a -e sched:\*,vpa_dtl/dtl_all/ -c 1000000000 sleep 1
  [ perf record: Woken up 1 times to write data ]
  [ perf record: Captured and wrote 0.300 MB perf.data ]

  # ./perf report
  # Samples: 321  of event 'vpa-dtl'
  # Event count (approx.): 321
  #
  # Children      Self  Command  Shared Object      Symbol
  # ........  ........  .......  .................  ..............................
  #
     100.00%   100.00%  swapper  [kernel.kallsyms]  [k] plpar_hcall_norets_notrace

## 使用 perf script 可视化调度跟踪日志条目（Visualize the dispatch trace log entries with perf script

   # ./perf script
     migration/9      67 [^009^] 105373.359903:                     sched:sched_waking: comm=perf pid=13418 prio=120 target_cpu=009
     migration/9      67 [^009^] 105373.359904:               sched:sched_migrate_task: comm=perf pid=13418 prio=120 orig_cpu=9 dest_cpu=10
     migration/9      67 [^009^] 105373.359907:               sched:sched_stat_runtime: comm=migration/9 pid=67 runtime=4050 [ns]
     migration/9      67 [^009^] 105373.359908:                     sched:sched_switch: prev_comm=migration/9 prev_pid=67 prev_prio=0 prev_state=S ==> next_comm=swapper/9 next_pid=0 next_prio=120
            :256     256 [^016^] 105373.359913:                                vpa-dtl: timebase: 21403600706628832 dispatch_reason:decrementer interrupt, preempt_reason:H_CEDE, enqueue_to_dispatch_time:4854,                        ready_to_enqueue_time:139, waiting_to_ready_time:511842115 c0000000000fcd28 plpar_hcall_norets_notrace+0x18 ([kernel.kallsyms])
            :256     256 [^017^] 105373.360012:                                vpa-dtl: timebase: 21403600706679454 dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:236,                         ready_to_enqueue_time:0, waiting_to_ready_time:133864583 c0000000000fcd28 plpar_hcall_norets_notrace+0x18 ([kernel.kallsyms])
            perf   13418 [^010^] 105373.360048:               sched:sched_stat_runtime: comm=perf pid=13418 runtime=139748 [ns]
            perf   13418 [^010^] 105373.360052:                     sched:sched_waking: comm=migration/10 pid=72 prio=0 target_cpu=010
