
## Intel 热节流（thermal throttle）事件报告


:Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>

### 简介（Introduction）


Intel 处理器内置自动且自适应的热监控机制，强制处理器降低其功耗，以便在预定的温度限制内运行。

更多细节请参阅"Intel® 64 and IA-32 Architectures Software Developer's Manual Volume 3 (3A, 3B, 3C, & 3D): System Programming Guide"中的"THERMAL MONITORING AND PROTECTION"一节。

一般而言，有两种机制用于控制处理器的核心温度。它们称为"Thermal Monitor 1（TM1，热监控器 1）与 Thermal Monitor 2（TM2，热监控器 2）"。

触发热监控（TM1/TM2）的温度传感器的状态，通过核心级的 MSR_IA32_THERM_STATUS 与封装（package）级的 MSR_IA32_PACKAGE_THERM_STATUS 中的"热状态标志"（thermal status flag）与"热状态日志标志"（thermal status log flag）来指示。

热状态标志（Thermal Status flag），第 0 位 — 置位时，表示处理器核心温度当前处于热监控器的触发（trip）温度，且处理器功耗正通过 TM1 或 TM2（取决于哪个被启用）被降低。清零时，该标志表示核心温度低于热监控器触发温度。该标志为只读。

热状态日志标志（Thermal Status Log flag），第 1 位 — 置位时，表示自上次上电或复位以来，或者自软件上次清除该标志以来，热传感器已触发过。该标志是"黏滞位"（sticky bit）；一旦置位，它会保持置位，直到被软件清除或直到处理器上电或复位。默认状态为清零。

有可能当用户读取 MSR_IA32_THERM_STATUS 或 MSR_IA32_PACKAGE_THERM_STATUS 时，TM1/TM2 并未处于活动状态。此时，"热状态标志"将读为"0"，而"热状态日志标志"会被置位以显示任何先前的"TM1/TM2"激活。但由于它需要被软件清除，因此无法显示"TM1/TM2"激活的发生次数。

因此，Linux 提供了"热状态标志"被置位的次数计数，同时呈现"热状态标志"处于活动状态的毫秒时长。利用这些计数器，用户可以检查性能是否因热事件而受到限制。建议从 sysfs 读取，而非直接读取 MSR，因为"热状态日志标志"会被驱动重置以实现速率控制（rate control）。

### Sysfs 接口（Sysfs Interface）


热节流事件在每个 CPU 下通过 "/sys/devices/system/cpu/cpuX/thermal_throttle/" 呈现，其中 "X" 为 CPU 编号。

所有这些计数器都是只读的。它们不能被重置为 0。因此，它们在达到 64 位无符号整数的最大值后可能会溢出。

`core_throttle_count`
	显示自操作系统启动且热向量（thermal vector）初始化以来，该 CPU 的"热状态标志"从 0 变为 1 的次数。这是一个 64 位计数器。

`package_throttle_count`
	显示自操作系统启动且热向量初始化以来，包含该 CPU 的封装（package）的"热状态标志"从 0 变为 1 的次数。封装状态会被广播到所有 CPU；封装内所有 CPU 都递增该计数。这是一个 64 位计数器。

`core_throttle_max_time_ms`
	显示自操作系统启动且热向量初始化以来，该 CPU 在核心级"热状态标志"被置为 1 的最大总时长。

`package_throttle_max_time_ms`
	显示自操作系统启动且热向量初始化以来，包含该 CPU 的封装的"热状态标志"被置为 1 的最大总时长。

`core_throttle_total_time_ms`
	显示自操作系统启动且热向量初始化以来，该 CPU 在核心级"热状态标志"被置为 1 的累计时长。

`package_throttle_total_time_ms`
	显示自操作系统启动且热向量初始化以来，包含该 CPU 的封装的"热状态标志"被置为 1 的累计时长。
