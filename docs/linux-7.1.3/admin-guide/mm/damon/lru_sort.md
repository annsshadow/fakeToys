
## 基于 DAMON 的 LRU 链表排序


基于 DAMON 的 LRU 链表排序（DAMON_LRU_SORT）是一个静态内核模块，旨在用于主动且轻量地基于数据访问
模式对 LRU 链表上的页面进行（取消）优先级排序，以使 LRU 链表成为更可信赖的数据访问模式来源。

## 何处需要主动的 LRU 链表排序？


由于在庞大的系统上页粒度访问检查的开销可能很大，LRU 链表通常不被主动排序，而是针对特定用户请求、
系统调用和内存压力等特殊事件进行部分且被动的排序。因此，在某些情形下，LRU 链表有时没有被完美地
准备好用作可信赖的访问模式来源，例如突然内存压力下的回收目标页面选择。

由于 DAMON 可以在只引入用户指定范围的开销的情况下，以尽力而为的精度识别访问模式，主动运行
DAMON_LRU_SORT 有助于以低且受控的开销使 LRU 链表成为更可信赖的访问模式来源。

## 它如何工作？


DAMON_LRU_SORT 使用 DAMON 查找热页（访问速率高于用户指定阈值的页面）和冷页（在长于用户指定阈值的
时间内没有任何访问的页面），并在它们的 LRU 链表上对热页进行优先级排序，同时对冷页进行取消优先级排序。
为了避免其消耗过多 CPU 用于优先级排序，可以配置一个 CPU 时间使用上限。在该上限下，它分别对更热的
和更冷的页面优先进行优先级排序和取消优先级排序。系统管理员还可以配置在何种情况下该方案应使用三个
内存压力水位线自动激活和停用。

其关于热/冷阈值和 CPU 配额限制的默认参数选择得比较保守。也就是说，该模块在其默认参数下可以在常见
情况下被广泛使用而不会造成损害，同时为在内存压力下具有明确热/冷访问模式的系统提供一定程度的收益，
同时仅消耗有限的少量 CPU 时间。

## 接口：模块参数


要使用此功能，你首先应确保你的系统运行在内置了 `CONFIG_DAMON_LRU_SORT=y` 的内核上。

为了让系统管理员能够启用或禁用它，并为给定系统调优，DAMON_LRU_SORT 利用了模块参数。也就是说，你
可以在内核启动命令行上放置 `damon_lru_sort.<parameter>=<value>`，或者向
`/sys/module/damon_lru_sort/parameters/<parameter>` 文件写入适当的值。

以下是每个参数的说明。

### enabled

启用或禁用 DAMON_LRU_SORT。

你可以通过将此项参数的值设置为 `Y` 来启用 DAMON_LRU_SORT。将其设置为 `N` 将禁用 DAMON_LRU_SORT。
注意，由于基于水位线的激活条件，DAMON_LRU_SORT 可能无法进行真正的监控和 LRU 链表排序。有关此点
请参阅下文对 watermarks 参数的说明。

### commit_inputs

使 DAMON_LRU_SORT 重新读取输入参数（除 `enabled` 外）。

DAMON_LRU_SORT 运行时更新的输入参数默认不会被应用。一旦将此参数设置为 `Y`，DAMON_LRU_SORT 会再次
读取除 `enabled` 外的参数值。一旦重新读取完成，此参数被设置为 `N`。如果在重新读取期间发现无效参数，
DAMON_LRU_SORT 将被禁用。

一旦向此参数写入 `Y`，用户必须在该参数再次读回 `N` 之前，不得向任何参数写入。如果用户违反此规则，
内核可能会表现出未定义的行为。

### active_mem_bp

期望的活跃与[非]活跃内存比率，单位为 bp（1/10,000）。

在保持其他配额设定的上限的同时，DAMON_LRU_SORT 自动增加和减少配额的有效级别，目标是热内存和冷内存
的 LRU [取消]优先级排序，从而产生此活跃与[非]活跃内存比率。值为零表示禁用此自动调优功能。

默认禁用。

### autotune_monitoring_intervals

如果此参数设置为 `Y`，DAMON_LRU_SORT 会自动调优 DAMON 的采样和聚合间隔。自动调优旨在在每个 DAMON
快照中捕获有意义数量的访问事件，同时保持采样间隔最小 5 毫秒、最大 10 秒。将其设置为 `N` 将禁用自动
调优。

默认禁用。

### filter_young_pages

为 LRU [取消]优先级排序相应地过滤[非]年轻页面。

如果设置了此项，则每次 LRU [取消]优先级排序操作之前再次检查页级访问（年轻度）。如果自上次检查以来
页面未被访问（不年轻），则跳过 LRU 优先级排序操作。如果自上次检查以来页面已被访问（年轻），则跳过
LRU 取消优先级排序操作。如果分别将此参数设置为 `Y` 或 `N`，则启用或禁用该功能。

默认禁用。

### hot_thres_access_freq

用于识别热内存区域的访问频率阈值，单位为千分之一（permil）。

如果一个内存区域的访问频率达到或超过此值，DAMON_LRU_SORT 将该区域识别为热，并在 LRU 链表上将其标记
为已访问，从而使其在内存压力下不会被回收。默认为 50%。

### cold_min_age

用于识别冷内存区域的时间阈值，单位为微秒。

如果一个内存区域在此时间或更长时间内未被访问，DAMON_LRU_SORT 将该区域识别为冷，并在 LRU 链表上将其
标记为未访问，从而使其在内存压力下被首先回收。默认为 120 秒。

### quota_ms

尝试进行 LRU 链表排序的时间上限，单位为毫秒。

DAMON_LRU_SORT 尝试在一段时间窗口（quota_reset_interval_ms）内最多使用此时间来尝试 LRU 链表排序。
这可用于限制 DAMON_LRU_SORT 的 CPU 消耗。如果该值为零，则禁用此限制。

默认 10 毫秒。

### quota_reset_interval_ms

时间配额的计费重置间隔，单位为毫秒。

时间配额（quota_ms）的计费重置间隔。也就是说，DAMON_LRU_SORT 不会在 quota_reset_interval_ms 毫秒
内尝试超过 quota_ms 毫秒或 quota_sz 字节的 LRU 链表排序。

默认 1 秒。

### wmarks_interval

水位线检查的时间间隔，单位为微秒。

当 DAMON_LRU_SORT 已启用但由于其水位线规则而不活动时，检查水位线之前的最短等待时间。默认 5 秒。

### wmarks_high

高水位线的空闲内存比率（每千）。

如果系统每千字节中的空闲内存字节数高于此值，DAMON_LRU_SORT 变为不活动，因此它什么也不做，只是定期
检查水位线。默认 200（20%）。

### wmarks_mid

中水位线的空闲内存比率（每千）。

如果系统每千字节中的空闲内存字节数介于此值与低水位线之间，DAMON_LRU_SORT 变为活动，于是开始监控和
LRU 链表排序。默认 150（15%）。

### wmarks_low

低水位线的空闲内存比率（每千）。

如果系统每千字节中的空闲内存字节数低于此值，DAMON_LRU_SORT 变为不活动，因此它什么也不做，只是定期
检查水位线。默认 50（5%）。

### sample_interval

监控的采样间隔，单位为微秒。

DAMON 用于冷内存监控的采样间隔。更多细节请参阅 DAMON 文档（[usage](usage)）。默认 5ms。

### aggr_interval

监控的聚合间隔，单位为微秒。

DAMON 用于冷内存监控的聚合间隔。更多细节请参阅 DAMON 文档（[usage](usage)）。默认 100ms。

### min_nr_regions

监控区域的最小数量。

DAMON 用于冷内存监控的最小监控区域数。这可用于设置监控质量的下限。但是，设置过高可能导致监控开销
增加。更多细节请参阅 DAMON 文档（[usage](usage)）。默认 10。

注意，此值必须为 3 或更高。有关此下限背后的原理，请参阅设计文档的 :ref:`监控 <damon_design_monitoring>`
一节。

### max_nr_regions

监控区域的最大数量。

DAMON 用于冷内存监控的最大监控区域数。这可用于设置监控开销的上限。但是，设置过低可能导致监控质量
变差。更多细节请参阅 DAMON 文档（[usage](usage)）。默认 1000。

### monitor_region_start

目标内存区域的起始物理地址。

DAMON_LRU_SORT 将针对其工作的内存区域的起始物理地址。默认使用最大的 System RAM 作为区域。

### monitor_region_end

目标内存区域的结束物理地址。

DAMON_LRU_SORT 将针对其工作的内存区域的结束物理地址。默认使用最大的 System RAM 作为区域。

### addr_unit

内存地址和字节的缩放因子。

此参数用于设置和获取 DAMON_RECLAIM 的 DAMON 实例的 :ref:`地址单元 <damon_design_addr_unit>`
参数。

`monitor_region_start` 和 `monitor_region_end` 应以此单位提供。例如，假设 `addr_unit`、
`monitor_region_start` 和 `monitor_region_end` 分别设置为 `1024`、`0` 和 `10`。那么 DAMON_LRU_SORT
将针对从地址零开始、长度为 10 KiB 的物理地址范围工作（`[0 ** 1024, 10 ** 1024)`，以字节计）。

带有 `bytes_` 前缀的统计参数也以此单位计。例如，假设 `addr_unit`、`bytes_lru_sort_tried_hot_regions`
和 `bytes_lru_sorted_hot_regions` 的值分别为 `1024`、`42` 和 `32`。则意味着 DAMON_LRU_SORT 尝试对
42 KiB 的热内存进行 LRU 排序，并成功地对其中 32 KiB 的内存进行了 LRU 排序。

如果不确定，请仅使用默认值（`1`）并忽略此项。

### kdamond_pid

DAMON 线程的 PID。

如果 DAMON_LRU_SORT 已启用，此值即为工作线程的 PID。否则为 -1。

### nr_lru_sort_tried_hot_regions

尝试进行 LRU 排序的热内存区域数量。

### bytes_lru_sort_tried_hot_regions

尝试进行 LRU 排序的热内存区域的总字节数。

### nr_lru_sorted_hot_regions

成功进行 LRU 排序的热内存区域数量。

### bytes_lru_sorted_hot_regions

成功进行 LRU 排序的热内存区域的总字节数。

### nr_hot_quota_exceeds

热区域的时间配额限制被超出次数。

### nr_lru_sort_tried_cold_regions

尝试进行 LRU 排序的冷内存区域数量。

### bytes_lru_sort_tried_cold_regions

尝试进行 LRU 排序的冷内存区域的总字节数。

### nr_lru_sorted_cold_regions

成功进行 LRU 排序的冷内存区域数量。

### bytes_lru_sorted_cold_regions

成功进行 LRU 排序的冷内存区域的总字节数。

### nr_cold_quota_exceeds

冷区域的时间配额限制被超出次数。

## 示例


以下运行时示例命令使 DAMON_LRU_SORT 查找访问频率 >=50% 的内存区域并进行 LRU 优先级排序，同时对 120
秒内未被访问的内存区域进行 LRU 取消优先级排序。优先级排序和取消优先级排序被限制为最多使用 1% 的 CPU
时间，以避免 DAMON_LRU_SORT 消耗过多 CPU 时间用于（取消）优先级排序。它还在系统空闲内存比率超过 50%
时要求 DAMON_LRU_SORT 不做任何事，但在其低于 40% 时开始真正的工作。如果 DAMON_RECLAIM 没有取得进展，
因此空闲内存比率低于 20%，它会再次要求 DAMON_LRU_SORT 不做任何事，以便我们可以回退到

```

    # cd /sys/module/damon_lru_sort/parameters
    # echo 500 > hot_thres_access_freq
    # echo 120000000 > cold_min_age
    # echo 10 > quota_ms
    # echo 1000 > quota_reset_interval_ms
    # echo 500 > wmarks_high
    # echo 400 > wmarks_mid
    # echo 200 > wmarks_low
    # echo Y > enabled

```

注意，此模块（damon_lru_sort）不能与其他基于 DAMON 的专用模块同时运行。更多细节请参阅 :ref:`DAMON
设计专用模块排他性 <damon_design_special_purpose_modules_exclusivity>`。
