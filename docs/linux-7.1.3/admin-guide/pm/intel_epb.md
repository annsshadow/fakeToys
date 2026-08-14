
## Intel 性能与能耗偏置提示


:Copyright: |copy| 2019 Intel Corporation

:Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>


   :doc: overview

## Intel 性能与能耗偏置（EPB）在 ``sysfs`` 中的属性


给定（逻辑）CPU 的 Intel 性能与能耗偏置提示（EPB）值可以通过 ``sysfs`` 下的一个属性（文件）来查看或更新，该属性位于 `/sys/devices/system/cpu/cpu<N>/power/`，其中 CPU 编号 `<N>` 在系统初始化时分配：

`energy_perf_bias`
	以 0 - 15 的滑动刻度显示该 CPU 当前的 EPB 值，其中
	值 0 对应最高性能的偏好，值 15 对应最大节能。

	为了更新该 CPU 的 EPB 值，可以向该属性写入，既可以写入上述 0 - 15 滑动刻度中的一个数字，也可以写入以下代表其含义的字符串之一："performance"、"balance-performance"、"normal"、"balance-power"、"power"。

	该属性存在于所有支持 EPB 特性的在线 CPU 上。

注意，虽然到处理器的 EPB 接口定义在逻辑 CPU 级别，但支持它的物理寄存器可能被多个 CPU 共享（例如，同一封装中的 SMT 兄弟核心或核心）。因此，更新一个 CPU 的 EPB 值可能导致其它 CPU 的 EPB 值发生变化。
