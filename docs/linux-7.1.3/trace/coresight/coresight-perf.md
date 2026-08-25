
## CoreSight - Perf


    :Author:   Carsten Haitzler <carsten.haitzler@arm.com>
    :Date:     June 29th, 2022

Perf 能够在本地访CoreSight 跟踪数据并将其存储到输出perf 数据文件中。这些数据随后可以被解码，以给出被跟踪的指令，用于调试或性能分析。你
```

   perf record -e cs_etm//u testbinary

```
这会运行某个测试二进制文件（testbinary）直到它退出，并记录一perf.data 跟踪文件。如CoreSight 工作正常，该文件会包AUX 段。你可以这样转储该文件的内容```

   perf report --stdio --dump -i perf.data

```
```

   0x1e78 [0x30]: PERF_RECORD_AUXTRACE size: 0x11dd0  offset: 0  ref: 0x1b614fc1061b0ad1  idx: 0  tid: 531230  cpu: -1

   . ... CoreSight ETM Trace data: size 73168 bytes
           Idx:0; ID:10;   I_ASYNC : Alignment Synchronisation.
             Idx:12; ID:10;  I_TRACE_INFO : Trace Info.; INFO=0x0 { CC.0 }
             Idx:17; ID:10;  I_ADDR_L_64IS0 : Address, Long, 64 bit, IS0.; Addr=0x0000000000000000;
             Idx:26; ID:10;  I_TRACE_ON : Trace On.
             Idx:27; ID:10;  I_ADDR_CTXT_L_64IS0 : Address & Context, Long, 64 bit, IS0.; Addr=0x0000FFFFB6069140; Ctxt: AArch64,EL0, NS;
             Idx:38; ID:10;  I_ATOM_F6 : Atom format 6.; EEEEEEEEEEEEEEEEEEEEEEEE
             Idx:39; ID:10;  I_ATOM_F6 : Atom format 6.; EEEEEEEEEEEEEEEEEEEEEEEE
             Idx:40; ID:10;  I_ATOM_F6 : Atom format 6.; EEEEEEEEEEEEEEEEEEEEEEEE
             Idx:41; ID:10;  I_ATOM_F6 : Atom format 6.; EEEEEEEEEEEEN
             ...

```
如果你看到上述内容，说明你的系统正在正确地跟CoreSight 数据```

   make CORESIGHT=1

```
构建它（指带 CoreSight 支持perf）需OpenCSD。你可以安装发行版提供的支持包，libopencsd libopencsd-dev，或者下载源码自行构建。OpenCSD 上游代码位于
  https://github.com/Linaro/OpenCSD

有关构建CoreSight 支持perf 以及更详尽用法的完整信息，请参见
  https://github.com/Linaro/OpenCSD/blob/master/HOWTO.md


### 内核 CoreSight 支持


你还应该在你的内核配置中启用 CoreSight 支持```

   CONFIG_CORESIGHT=y

```
你可能还想要启用其他各种 CoreSight 选项
```

   CONFIG_CORESIGHT_LINKS_AND_SINKS=y
   CONFIG_CORESIGHT_LINK_AND_SINK_TMC=y
   CONFIG_CORESIGHT_CATU=y
   CONFIG_CORESIGHT_SINK_TPIU=y
   CONFIG_CORESIGHT_SINK_ETBV10=y
   CONFIG_CORESIGHT_SOURCE_ETM4X=y
   CONFIG_CORESIGHT_CTI=y
   CONFIG_CORESIGHT_CTI_INTEGRATION_REGS=y

```
更多信息请参阅内核配置帮助
### 使用 AUX 暂停与恢复进行细粒度跟踪


Arm CoreSight 可能产生大量的硬件跟踪数据，这会带来记录开销，并在查看性能分析结果时分散用户注意力。为了缓解过多跟踪数据的问题，Perf 提供AUX 暂停（pause）和恢复（resume）功能，以实现细粒度跟踪
AUX 暂停和恢复可以由关联的事件触发。这些事件可以是 ftrace 跟踪点（包括静态和动态跟踪点）或 PMU 事件（例CPU PMU 周期事件）。为了创建一个带AUX 暂停/恢复perf 会话，引入了三个配置项：

- "aux-action=start-paused"：为 cs_etm PMU 事件指定，使其以暂停状态启动- "aux-action=pause"：用此配置项指定一个关联事件以暂停 AUX 跟踪- "aux-action=resume"：用此配置项指定一个关联事件以恢复 AUX 跟踪```

  perf record -e cs_etm/aux-action=start-paused/k,syscalls:sys_enter_openat/aux-action=resume/,syscalls:sys_exit_openat/aux-action=pause/ ls

```
```

  perf record -a -e cs_etm/aux-action=start-paused/k \
        -e cycles/aux-action=pause,period=10000000/ \
        -e cycles/aux-action=resume,period=1050000/ -- sleep 1

```
### Perf 测试 - 验证内核与用户空perf CoreSight 是否工作


当你运行 perf test 时，它会进行大量自测试。其中一些测试会覆盖 CoreSight（仅在启用且位于 ARM64 上时）。通常你会在内核树tools/perf 目录下运perf test。一些测试会检查某些内perf 支持，例如：

   Check Arm CoreSight trace data recording and synthesized samples
   检Arm CoreSight 跟踪数据记录与合成采   Check Arm SPE trace data recording and synthesized samples
   检Arm SPE 跟踪数据记录与合成采
另一些测试会实际使用 perf record 以及 tests/shell/coresight 中的一些测试二进制文件，并收集跟踪以确保达到最低的功能水平。启动这些测试的脚本位于同一目录中。它们看起来都像
   CoreSight / ASM 纯循   CoreSight / Memcpy 16k 10 线程
   CoreSight / 线程循环 10 线程 - 检TID
   etc.

如果工具二进制文件不存在tests/shell/coresight\*/ 中，这些 perf record 测试将不会运行，而是被跳过。如果你的硬件不支持 CoreSight，那么要么不构建CoreSight 支持perf，要么移除这些二进制文件，以免这些测试失败，让它们改为被跳过
这些测试会在当前工作目录（例tools/perf）中记录历史结果，并stats-\*.csv 这样的名称命名，例如
   stats-asm_pure_loop-out.csv
   stats-memcpy_thread-16k_10.csv
   ...

这些统计文件记录 perf 数据输出AUX 数据段的某些方面，统计某些特定编码的数量（一种以非常简单的方式确认其是否正常工作的方法）。CoreSight 的一个问题是，当需要记录的待记录数据量足够大时，其中一部分可能会由于处理器未能及时唤醒以从缓冲区读出所有数据等原因而丢失。你会注意到每次运行 perf test 收集到的数据量可能有很大差异。如果你想观察它随时间如何变化，只需多次运行 perf test，所有这csv 文件都会不断追加更多数据，供你之后检查、绘图或以其他方式使用，来判断情况变好还是变坏
这意味着有时这些测试会失败，因为它们没有捕获到所需的全部数据。这关乎随时间跟踪所产生的数据质量和数量，以及观察对 Linux 内核的更改何时改善了跟踪质量
请注意，其中一些测试运行时间相当长，特别是在处perf 数据文件并转储内容以检查其内部时
你可以通过在运perf 之前设置 PERF_TEST_CORESIGHT_STATDIR 环境变量来改变这csv 日志的存储位```

   export PERF_TEST_CORESIGHT_STATDIR=/var/tmp
   perf test

```
它们还会将生成的 perf 输出数据存储在当```

   perf-asm_pure_loop-out.data
   perf-memcpy_thread-16k_10.data
   ...

```
你可以通过设置
```

   PERF_TEST_CORESIGHT_DATADIR=/var/tmp
   perf test

```
来改perf 数据文件的存储位置。如果你希望将测试输出保存在当前工作目录之外以进行长期存储和检查，可以设置上述环境变量