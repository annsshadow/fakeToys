## Cavium ThunderX2 SoC 性能监控单元（PMU UNCORE）


ThunderX2 SoC PMU 由独立的、系统范围的、每插槽 PMU 组成，例如三级缓存
（L3C）、DDR4 内存控制器（DMC）以及 Cavium 一致性处理器互连（CCPI2）。

DMC 有 8 个交错通道，L3C 有 16 个交错分块。事件针对默认通道（即通道 0）
计数，并按通道/分块总数按比例分摊。

DMC 与 L3C 最多支持 4 个计数器，而 CCPI2 最多支持 8 个计数器。计数器可独立
编程为不同事件，并可单独启动和停止。没有任何计数器支持溢出中断。DMC 与
L3C 计数器为 32 位，每 2 秒读取一次。CCPI2 计数器为 64 位，在正常操作中
假定不会溢出。

PMU UNCORE（perf）驱动：

thunderx2_pmu 驱动为 DMC 与 L3C 设备注册每插槽的 perf PMU。每个 PMU 可用于
同时计数最多 4 个（DMC/L3C）或最多 8 个（CCPI2）事件。这些 PMU 在 sysfs 下
提供其可用事件与配置选项的描述，见
/sys/bus/event_source/devices/uncore_<l3c_S/dmc_S/ccpi2_S/>；S 为插槽 id。

该驱动不支持采样，因此“perf record”无法工作。也不支持每任务 perf 会话。

```

  # perf stat -a -e uncore_dmc_0/cnt_cycles/ sleep 1

  # perf stat -a -e \
  uncore_dmc_0/cnt_cycles/,\
  uncore_dmc_0/data_transfers/,\
  uncore_dmc_0/read_txns/,\
  uncore_dmc_0/write_txns/ sleep 1

  # perf stat -a -e \
  uncore_l3c_0/read_request/,\
  uncore_l3c_0/read_hit/,\
  uncore_l3c_0/inv_request/,\
  uncore_l3c_0/inv_hit/ sleep 1

```
