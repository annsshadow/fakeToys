
## HTM（硬件跟踪宏，Hardware Trace Macro）


Athira Rajeev, 2 Mar 2025

    :depth: 3


## 基本概述


H_HTM 用作执行硬件跟踪宏（HTM）功能的接口，包括 HTM 数据的设置、配置、控制和转储。使用 HTM
需要设置 HTM 缓冲区，并且 HTM 操作可以使用 H_HTM hcall 进行控制。该 hcall 可以从分区内部针对
系统的任何核心/芯片调用。要使用此特性，在 /sys/kernel/debug/powerpc 下存在一个名为 “htmdump”
的 debugfs 文件夹。


## HTM debugfs 使用示例


  #  ls /sys/kernel/debug/powerpc/htmdump/
  coreindexonchip  htmcaps  htmconfigure  htmflags  htminfo  htmsetup
  htmstart  htmstatus  htmtype  nodalchipindex  nodeindex  trace

每个文件的详细信息：

- nodeindex、nodalchipindex、coreindexonchip 指定要为哪个分区配置 HTM。
- htmtype：指定 HTM 的类型。支持的目标是 hardwareTarget。
- trace：用于读取 HTM 数据。
- htmconfigure：配置/取消配置 HTM。向文件写入 1 将配置跟踪，写入 0 将取消配置。
- htmstart：启动/停止 HTM。向文件写入 1 将启动跟踪，写入 0 将停止跟踪。
- htmstatus：获取 HTM 的状态。这用于了解每次操作后的 HTM 状态。
- htmsetup：设置 HTM 缓冲区大小。HTM 缓冲区大小为 2 的幂
- htminfo：提供系统处理器配置详细信息。这用于了解 nodeindex、nodalchipindex、coreindexonchip
  的适当值。
- htmcaps：提供 HTM 的能力，如最小/最大缓冲区大小、HTM 支持何种跟踪等。
- htmflags：允许向 hcall 传递标志。目前支持控制 HTM 缓冲区的回绕。

要查看系统处理器配置详细信息：


  # cat /sys/kernel/debug/powerpc/htmdump/htminfo > htminfo_file

结果可以使用 hexdump 进行解析。

要为 nodeindex 为 0、nodalchipindex 为 1、coreindexonchip 为 12 的分区收集 HTM 跟踪


  # cd /sys/kernel/debug/powerpc/htmdump/
  # echo 2 > htmtype
  # echo 33 > htmsetup ( 设置 8GB 内存用于 HTM 缓冲区，数字为 2 的幂大小 )

这需要重启 CEC 以分配 HTM 缓冲区。


  # cd /sys/kernel/debug/powerpc/htmdump/
  # echo 2 > htmtype
  # echo 0 > nodeindex
  # echo 1 > nodalchipindex
  # echo 12 > coreindexonchip
  # echo 1 > htmflags     # 为 HTM 缓冲区设置 noWrap
  # echo 1 > htmconfigure # 配置 HTM
  # echo 1 > htmstart     # 启动 HTM
  # echo 0 > htmstart     # 停止 HTM
  # echo 0 > htmconfigure # 取消配置 HTM
  # cat htmstatus         # 将 HTM 条目状态作为数据转储

上述操作将设置 htmtype 和核心详细信息，然后执行相应的 HTM 操作。

## 读取 HTM 跟踪数据


开始跟踪收集后，运行你感兴趣的工作负载。在所需时间后停止跟踪收集，并读取跟踪文件。


  # cat /sys/kernel/debug/powerpc/htmdump/trace > trace_file

此跟踪文件将包含在工作负载执行期间收集的相关指令跟踪。它可作为跟踪解码器的输入文件来理解数据。

## 使用 HTM debugfs 接口的好处


现在可以从系统的任何分区内部为特定核心/芯片收集跟踪并解码。通过此功能，一个小的分区可以被
专门用于收集跟踪数据并进行分析，从而为性能分析、软件调优或硬件调试提供重要信息。
