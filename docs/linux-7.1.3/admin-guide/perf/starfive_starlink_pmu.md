## StarFive StarLink 性能监控单元（PMU）


StarFive StarLink 性能监控单元（PMU）位于 StarLink 一致性片上网络（CNoC）中，
该网络将多个 CPU 集群与 L3 内存系统连接起来。

该 uncore PMU 支持溢出中断、最多 16 个可编程 64bit 事件计数器，以及一个
独立的 64bit 周期计数器。PMU 只能通过内存映射 I/O（MMIO）访问，并且对
连接到同一 PMU 的核心来说是共享的。

```

  /sys/bus/event_source/devices/starfive_starlink_pmu/events/

```
驱动在 sysfs 的“cpumask”目录中暴露用于处理 PMU 事件的 cpu

```

  /sys/bus/event_source/devices/starfive_starlink_pmu/cpumask/

```
驱动在 sysfs 的“format”目录中描述 config（事件 ID）的格式

```

  /sys/bus/event_source/devices/starfive_starlink_pmu/format/

```
```

	$ perf list

	starfive_starlink_pmu/cycles/                      [Kernel PMU event]
	starfive_starlink_pmu/read_hit/                    [Kernel PMU event]
	starfive_starlink_pmu/read_miss/                   [Kernel PMU event]
	starfive_starlink_pmu/read_request/                [Kernel PMU event]
	starfive_starlink_pmu/release_request/             [Kernel PMU event]
	starfive_starlink_pmu/write_hit/                   [Kernel PMU event]
	starfive_starlink_pmu/write_miss/                  [Kernel PMU event]
	starfive_starlink_pmu/write_request/               [Kernel PMU event]
	starfive_starlink_pmu/writeback/                   [Kernel PMU event]


	$ perf stat -a -e /starfive_starlink_pmu/cycles/ sleep 1

```
不支持采样。因此不支持“perf record”。不支持附加到任务，仅支持系统范围的计数。
