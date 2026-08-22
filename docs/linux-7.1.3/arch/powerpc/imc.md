
## IMC（In-Memory Collection Counters，内存内采集计数器）


Anju T Sudhakar锛?019 骞?5 鏈?10 鏃。

    :depth: 3


## 基本概述


IMC（In-Memory collection counters，内存内采集计数器）是一种硬件监控设施，它在 Nest 级别（片内但核外）、Core 级别Thread 级别收集大量的硬件性能事件

Nest PMU 计数器由一个运行在 OCC（On-Chip Controller，片上控制器）复合体中的 Nest IMC 微码处理。该微码收集计数器数据，并将 nest IMC 计数器数据搬移到内存中

Core Thread IMC PMU 计数器在核内处理。Core PMU 计数器为我们提供每个核的 IMC 计数器数据，thread PMU 计数器为我们提供每个 CPU 线程IMC 计数器数据

OPAL IMC Catalog 获取 IMC PMU 及所支持事件的信息，并通过设备树传递给内核。事件的信息包含

- 事件名称（Event name
- 事件偏移（Event Offset
- 事件描述（Event description

并且可能还包含：

- 事件缩放（Event scale
- 事件单位（Event unit

某些 PMU 可能对其所有受支持的事件具有共同的 scale unit 值。对于这些情况，这些事件scale unit 属性必须从 PMU 继承

内存中的事件偏移处就是计数器数据被累加的地方

IMC catalog 位于
	https://github.com/open-power/ima-catalog

内核在设备树`imc-counters` 设备节点中发IMC 计数器信息，该节点具compatible 字段 `ibm,opal-in-memory-counters`。内核从设备树中解析 PMU 及其事件信息，并在内核中注册 PMU 及其属性

## IMC 使用示例



  # perf list
  [...]
  nest_mcs01/PM_MCS01_64B_RD_DISP_PORT01/            [Kernel PMU event]
  nest_mcs01/PM_MCS01_64B_RD_DISP_PORT23/            [Kernel PMU event]
  [...]
  core_imc/CPM_0THRD_NON_IDLE_PCYC/                  [Kernel PMU event]
  core_imc/CPM_1THRD_NON_IDLE_INST/                  [Kernel PMU event]
  [...]
  thread_imc/CPM_0THRD_NON_IDLE_PCYC/                [Kernel PMU event]
  thread_imc/CPM_1THRD_NON_IDLE_INST/                [Kernel PMU event]

要查nest_mcs0/PM_MCS_DOWN_128B_DATA_XFER_MC0/ 的每个芯片数据：


  # ./perf stat -e "nest_mcs01/PM_MCS01_64B_WR_DISP_PORT01/" -a --per-socket

要查core 0 的非空闲指令


  # ./perf stat -e "core_imc/CPM_NON_IDLE_INST/" -C 0 -I 1000

要查"make" 的非空闲指令


  # ./perf stat -e "thread_imc/CPM_NON_IDLE_PCYC/" make


## IMC 跟踪模式（Trace-mode


POWER9 支持 IMC 的两种模式：累加（Accumulation）模式和跟踪（Trace）模式。在累加模式下，事件计数在系统内存中累加。然Hypervisor 会周期性地或在被请求时读取这些已提交的计数。在 IMC 跟踪模式下，64 位的 trace SCOM 值被初始化为事件信息。trace SCOM 中的 CPMCxSEL CPMC_LOAD 指定了要监控的事件以及采样时长。在 CPMCxSEL 每次溢出时，硬件会快照程序计数器以及事件计数，并写入LDBAR 指向的内存

LDBAR 是一64 位的每线程特殊用途寄存器，它的位用于指示硬件是配置为累加模式还是跟踪模式

### LDBAR 寄存器布局


  +-------+----------------------+
  | 0     | Enable/Disable       |
  +-------+----------------------+
  | 1     | 0: Accumulation Mode |
  |       +----------------------+
  |       | 1: Trace Mode        |
  +-------+----------------------+
  | 2:3   | Reserved             |
  +-------+----------------------+
  | 4-6   | PB scope             |
  +-------+----------------------+
  | 7     | Reserved             |
  +-------+----------------------+
  | 8:50  | Counter Address      |
  +-------+----------------------+
  | 51:63 | Reserved             |
  +-------+----------------------+

### TRACE_IMC_SCOM 位表


  +-------+------------+
  | 0:1   | SAMPSEL    |
  +-------+------------+
  | 2:33  | CPMC_LOAD  |
  +-------+------------+
  | 34:40 | CPMC1SEL   |
  +-------+------------+
  | 41:47 | CPMC2SEL   |
  +-------+------------+
  | 48:50 | BUFFERSIZE |
  +-------+------------+
  | 51:63 | RESERVED   |
  +-------+------------+

CPMC_LOAD 包含采样时长。SAMPSEL CPMCxSEL 决定要计数的事件。BUFFERSIZE 指示内存范围。每次溢出时，硬件会快照程序计数器以及事件计数，并更新内存并重新加载 CMPC_LOAD 值以进行下一次采样。IMC 硬件不支持异常，因此如果内存缓冲区到达末尾，它会静默地回绕

**目前，跟踪模式下监控的事件固定为 cycle*

## 跟踪 IMC 使用示例



  # perf list
  [....]
  trace_imc/trace_cycles/                            [Kernel PMU event]

要记录一个使trace-imc 事件的应用程进程


  # perf record -e trace_imc/trace_cycles/ yes > /dev/null
  [ perf record: Woken up 1 times to write data ]
  [ perf record: Captured and wrote 0.012 MB perf.data (21 samples) ]

生成`perf.data` 可以使用 perf report 读取

## 使用 IMC 跟踪模式的好


避免PMI（Performance Monitoring Interrupts，性能监控中断）中断处理，因为 IMC 跟踪模式会快照程序计数器并更新到内存。这也提供了一种方式，让操作系统在不产PMI 处理开销的情况下实时进行指令采样

使用 `perf top` 带与不带 trace-imc 事件时的性能数据

执行 `perf top` 命令但不trace-imc 事件时，会统PMI 中断计数


  # grep PMI /proc/interrupts
  PMI:          0          0          0          0   Performance monitoring interrupts
  # ./perf top
  ...
  # grep PMI /proc/interrupts
  PMI:      39735       8710      17338      17801   Performance monitoring interrupts
  # ./perf top -e trace_imc/trace_cycles/
  ...
  # grep PMI /proc/interrupts
  PMI:      39735       8710      17338      17801   Performance monitoring interrupts


也就是说，使`trace_imc` 事件时，PMI 中断计数不会增加
