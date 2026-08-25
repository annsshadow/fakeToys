## Fujitsu Uncore 性能监控单元（PMU


本驱动支Fujitsu 芯片中的 Uncore MAC PMU Uncore PCI PMU
这些芯片上的每个 MAC PMU 都作为一uncore perf PMU 暴露出来，设备名
mac_iod<iod>_mac<mac>_ch<ch>銆。
每个 PCI PMU 作为 uncore perf PMU 暴露，设备名pci_iod<iod>_pci<pci>

驱动sysfs 中提供其可用事件与配置选项的描述，参见
/sys/bus/event_sources/devices/mac_iod<iod>_mac<mac>_ch<ch>/
涓?/sys/bus/event_sources/devices/pci_iod<iod>_pci<pci>/銆。
本驱动导出：

- formats：供 perf 用户空间及其他工具配置事件使
- events：供 perf 用户空间及其他工具创建事件使
```

    perf stat -a -e mac_iod0_mac0_ch0/event=0x21/ ls
    perf stat -a -e pci_iod0_pci0/event=0x24/ ls

```
- cpumask：供 perf 用户空间及其他工具了解应在哪CPU 上打开事件

本驱动为 MAC 支持以下事件

- cycles
  此事件统MAC MAC 频率下的周期数
- read-count
  此事件统计发往 MAC 的读请求数量
- read-count-request
  此事件统计包含重试的、发往 MAC 的读请求数量
- read-count-return
  此事件统计对发往 MAC 的读请求的响应数量
- read-count-request-pftgt
  此事件统计带PFTGT 标志、包含重试的读请求数量
- read-count-request-normal
  此事件统计不PFTGT 标志、包含重试的读请求数量
- read-count-return-pftgt-hit
  此事件统计命PFTGT 缓冲区的读请求响应数量
- read-count-return-pftgt-miss
  此事件统计未命中 PFTGT 缓冲区的读请求响应数量
- read-wait
  此事件统计每个周期由 DDR 内存控制器发出的未完成读请求数量
- write-count
  此事件统计发往 MAC 的写请求数量（包括零写、全写、部分写、写取消）
- write-count-write
  此事件统计发往 MAC 的全写请求数量（不包括零写）
- write-count-pwrite
  此事件统计发往 MAC 的部分写请求数量
- memory-read-count
  此事件统MAC 发往内存的读请求数量
- memory-write-count
  此事件统MAC 发往内存的全写请求数量
- memory-pwrite-count
  此事件统MAC 发往内存的部分写请求数量
- ea-mac
  此事件统MAC 的能耗
- ea-memory
  此事件统计内存的能耗
- ea-memory-mac-write
  此事件统MAC 发往内存的写请求数量
- ea-ha
  此事件统HA 的能耗

  'ea' 'Energy Analyzer'（能耗分析器）的缩写

```

  perf stat -e mac_iod0_mac0_ch0/ea-mac/ ls

```
此外，本驱动PCI 支持以下事件

- pci-port0-cycles
  此事件统port0 PCI PCI 频率下的周期数
- pci-port0-read-count
  此事件统port0 中用于数据传输的读事务数量
- pci-port0-read-count-bus
  此事件统port0 中用于总线占用的读事务数量
- pci-port0-write-count
  此事件统port0 中用于数据传输的写事务数量
- pci-port0-write-count-bus
  此事件统port0 中用于总线占用的写事务数量
- pci-port1-cycles
  此事件统port1 PCI PCI 频率下的周期数
- pci-port1-read-count
  此事件统port1 中用于数据传输的读事务数量
- pci-port1-read-count-bus
  此事件统port1 中用于总线占用的读事务数量
- pci-port1-write-count
  此事件统port1 中用于数据传输的写事务数量
- pci-port1-write-count-bus
  此事件统port1 中用于总线占用的写事务数量
- ea-pci
  此事件统PCI 的能耗

  'ea' 'Energy Analyzer'（能耗分析器）的缩写

```

  perf stat -e pci_iod0_pci0/ea-pci/ ls

```
由于这些uncore PMU，驱动不支持采样，因"perf record" 无法使用。也不支持按任务perf 会话
