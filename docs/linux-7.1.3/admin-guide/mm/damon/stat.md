
## 数据访问监控结果统计 (DAMON_STAT)


数据访问监控结果统计（DAMON_STAT）是一个静态内核模块，旨在用于简单的访问模式监控。它使用
DAMON 监控整个系统物理内存上的访问，并提供简化的访问监控结果统计，即空闲时间百分位和估计
内存带宽。


## 监控精度与开销


DAMON_STAT 使用监控间隔的 :ref:`自动调优 <damon_design_monitoring_intervals_autotuning>`
来实现高精度和低开销。它自动调优间隔，目标是每次快照捕获 4% 的可观察访问事件，同时将得到的
采样间隔限制在最小 5 毫秒、最大 10 秒。在一些生产服务器系统上，它仅消耗 0.x% 的单 CPU 时间，
同时捕获了合理质量的访问模式。调优得到的间隔可以通过 `aggr_interval_us` :ref:`参数
<damon_stat_aggr_interval_us>` 获取。

## 接口：模块参数


要使用此特性，你应首先确保系统运行在内置了 `CONFIG_DAMON_STAT=y` 的内核上。通过在构建时设置
`CONFIG_DAMON_STAT_ENABLED_DEFAULT` 为 true，可以在构建时默认启用该特性。

为了让系统管理员在启动和/或运行时启用或禁用它，并读取监控结果，DAMON_STAT 提供了模块参数。
以下各节是这些参数的描述。

### enabled


启用或禁用 DAMON_STAT。

你可以通过将此参数的值设为 `Y` 来启用 DAMON_STAT。设为 `N` 则禁用 DAMON_STAT。默认值由
`CONFIG_DAMON_STAT_ENABLED_DEFAULT` 构建配置选项设置。

注意此模块（damon_stat）不能与其他基于 DAMON 的专用模块同时运行。更多详情请参阅 :ref:`DAMON
设计专用模块互斥性 <damon_design_special_purpose_modules_exclusivity>`。


### aggr_interval_us


自动调优的聚合时间间隔（微秒）。

用户可以读取 DAMON_STAT 的 DAMON 实例正在使用的 DAMON 聚合间隔。它是 :ref:`自动调优的
<damon_stat_monitoring_accuracy_overhead>`，因此该值会动态变化。

### estimated_memory_bandwidth


系统的估计内存带宽消耗（字节/秒）。

DAMON_STAT 读取当前 DAMON 结果快照上观察到的访问事件，并将其转换为以字节/秒为单位的内存带宽
消耗估计。得到的指标通过此只读参数暴露给用户。由于 DAMON 使用采样，这只是对访问强度的估计，
而非准确的内存带宽。

### memory_idle_ms_percentiles


系统每字节空闲时间（毫秒）的百分位。

DAMON_STAT 根据当前 DAMON 结果快照，计算内存中每个字节到目前为止未被访问的时长（空闲时间）。
对于访问频率（nr_accesses）大于零的区域，当前访问频率级别保持的时长乘以 `-1` 成为该区域每个
字节的空闲时间。如果一个区域的访问频率为零（nr_accesses），则该区域保持零访问频率的时长（age）
成为该区域每个字节的空闲时间。然后，DAMON_STAT 通过此只读参数暴露空闲时间值的百分位。读取该
参数会返回 101 个以毫秒为单位的空闲时间值，以逗号分隔。每个值分别代表第 0、1、2、3、...、99
和 100 百分位的空闲时间。
