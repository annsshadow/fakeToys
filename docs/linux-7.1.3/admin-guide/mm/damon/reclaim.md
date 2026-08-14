
## 基于 DAMON 的回收

DAMON-based Reclamation（DAMON_RECLAIM）是一个静态内核模块，旨在用于
在较轻的内存压力下主动、轻量地进行回收。它并非要取代基于 LRU 链表、
以页为粒度的回收，而是用于针对不同级别的内存压力和需求进行选择性使用。

## 何处需要主动回收？


在一般的过度申请（over-committed）内存系统上，主动回收冷页有助于节省
内存，并减少由进程的直接回收或 kswapd 的 CPU 消耗所引发的延迟尖峰，
同时只带来最小的性能下降 [^1^]_ [^2^]_ 。

基于空闲页报告 [^3^]_ 的内存过度申请虚拟化系统就是这类情况的好例子。
在这类系统中，客户机 VM 将其空闲内存报告给宿主机，宿主机再将报告的
内存重新分配给其他客户机。结果是系统的内存得到了充分利用。然而，客户机
可能并不那么节省内存，主要是因为某些内核子系统和用户空间应用程序被设计
为尽可能多地使用可用内存。于是，客户机可能只向宿主机报告少量空闲内存，
导致系统的内存利用率下降。在客户机中运行主动回收可以缓解此问题。

## 它如何工作？


DAMON_RECLAIM 找出在特定期限内未被访问的内存区域并将其换出（page out）。
为了避免换出操作消耗过多 CPU，可以配置一个速度限制。在该速度限制下，
它会优先换出更长时间未被访问的内存区域。系统管理员还可以通过三个内存
压力水位线，配置在什么情况下该方案应自动激活和停用。

## 接口：模块参数


要使用此特性，应首先确保系统运行的内核已启用 `CONFIG_DAMON_RECLAIM=y`。

为了让系统管理员能够启用、禁用它并针对给定系统进行调优，DAMON_RECLAIM
利用了模块参数。也就是说，你可以在内核引导命令行上放置
`damon_reclaim.<parameter>=<value>`，或者向
`/sys/module/damon_reclaim/parameters/<parameter>` 文件写入适当的值。

以下是每个参数的说明。

### enabled


启用或禁用 DAMON_RECLAIM。

你可以将该参数的值设为 `Y` 来启用 DAMON_RECLAIM。设为 `N` 则禁用
DAMON_RECLAIM。注意，由于基于水位线的激活条件，DAMON_RECLAIM 可能
不会进行真正的监控和回收。有关水位线参数，请参考下面的说明。

### commit_inputs


使 DAMON_RECLAIM 重新读取输入参数（除 `enabled` 外）。

DAMON_RECLAIM 运行时更新的输入参数默认不会被应用。一旦将此参数设为
`Y`，DAMON_RECLAIM 会重新读取除 `enabled` 以外各参数的值。重新读取
完成后，此参数会被设为 `N`。如果在重新读取时发现无效参数，
DAMON_RECLAIM 将被禁用。

一旦向此参数写入 `Y`，用户在该参数再次读回 `N` 之前，不得写入任何
参数。如果用户违反此规则，内核可能表现出未定义的行为。

### min_age


冷内存区域判定的时间阈值，单位为微秒。

如果一个内存区域在此时间或更长时间内未被访问，DAMON_RECLAIM 会将该
区域识别为冷区域并予以回收。

默认 120 秒。

### quota_ms


回收的时间限制，单位为毫秒。

DAMON_RECLAIM 试图在一个时间窗口（quota_reset_interval_ms）内最多只使用
这段时间来尝试回收冷页。这可用于限制 DAMON_RECLAIM 的 CPU 消耗。如果
该值为零，则禁用此限制。

默认 10 毫秒。

### quota_sz


回收的内存大小限制，单位为字节。

DAMON_RECLAIM 会在一个时间窗口（quota_reset_interval_ms）内对尝试回收的
内存量进行记账，并使得尝试回收的量不超过此限制。这可用于限制 CPU 和 IO
的消耗。如果该值为零，则禁用此限制。

默认 128 MiB。

### quota_reset_interval_ms


时间/大小配额的记账重置间隔，单位为毫秒。

用于时间配额（quota_ms）和大小配额（quota_sz）的记账重置间隔。也就是说，
在 quota_reset_interval_ms 毫秒内，DAMON_RECLAIM 尝试回收的时间不超过
quota_ms 毫秒，或字节数不超过 quota_sz 字节。

默认 1 秒。

### quota_mem_pressure_us


期望的内存压力停顿时间水平，单位为微秒。

在保持其他配额所设上限的同时，DAMON_RECLAIM 会自动增加或降低配额的有效
水平，以达成引发此级别内存压力的目标。系统范围内每次配额重置间隔
（`quota_reset_interval_ms`）内以微秒计的 `some` 内存 PSI 会被收集，
并与该值比较，以判断是否达成目标。值为零表示禁用此自动调优特性。

默认禁用。

### quota_autotune_feedback


用户可指定的用于有效配额自动调优的反馈。

在保持其他配额所设上限的同时，DAMON_RECLAIM 会自动增加或降低配额的
有效水平，以期望收到来自用户的、值为 `10,000` 的此反馈。DAMON_RECLAIM
假定反馈值与配额呈正比关系。值为零表示禁用此自动调优特性。

默认禁用。

### wmarks_interval


当 DAMON_RECLAIM 已启用但因水位线规则处于非活动状态时，检查水位线前
最少要等待的时间。

### wmarks_high


高水位线的空闲内存率（每千分之一）。

如果系统每千字节中的空闲内存字节数高于此值，DAMON_RECLAIM 变为非活动
状态，因此除了定期检查水位线外不做任何事情。

### wmarks_mid


中水位线的空闲内存率（每千分之一）。

如果系统每千字节中的空闲内存字节数介于此值与低水位线之间，DAMON_RECLAIM
变为活动状态，于是开始监控和回收。

### wmarks_low


低水位线的空闲内存率（每千分之一）。

如果系统每千字节中的空闲内存字节数低于此值，DAMON_RECLAIM 变为非活动
状态，因此除了定期检查水位线外不做任何事情。在这种情况下，系统回退到
基于 LRU 链表、以页为粒度的回收逻辑。

### sample_interval


监控的采样间隔，单位为微秒。

DAMON 用于冷内存监控的采样间隔。更多细节请参考 DAMON 文档
（[usage](usage)）。

### aggr_interval


监控的聚合间隔，单位为微秒。

DAMON 用于冷内存监控的聚合间隔。更多细节请参考 DAMON 文档
（[usage](usage)）。

### min_nr_regions


监控区域的最小数量。

DAMON 用于冷内存监控的监控区域最小数量。这可用于设置监控质量的下界。
但设置过高可能导致监控开销增加。更多细节请参考 DAMON 文档
（[usage](usage)）。

注意该值必须为 3 或更高。其下界背后的原理，请参考设计文档的
:ref:`监控 <damon_design_monitoring>` 章节。

### max_nr_regions


监控区域的最大数量。

DAMON 用于冷内存监控的监控区域最大数量。这可用于设置监控开销的上界。
然而，设置过低可能导致监控质量变差。更多细节请参考 DAMON 文档
（[usage](usage)）。

### monitor_region_start


目标内存区域的起始物理地址。

DAMON_RECLAIM 将对其工作的内存区域起始物理地址。也就是说，DAMON_RECLAIM
会在此区域内找出冷内存区域并予以回收。默认使用最大的 System RAM 作为
该区域。

### monitor_region_end


目标内存区域的结束物理地址。

DAMON_RECLAIM 将对其工作的内存区域结束物理地址。也就是说，DAMON_RECLAIM
会在此区域内找出冷内存区域并予以回收。默认使用最大的 System RAM 作为
该区域。

### addr_unit


内存地址与字节的缩放因子。

该参数用于设置和获取 DAMON_RECLAIM 的 DAMON 实例的 :ref:`地址单位
<damon_design_addr_unit>` 参数。

`monitor_region_start` 和 `monitor_region_end` 应以该单位提供。例如，
假设 `addr_unit`、`monitor_region_start` 和 `monitor_region_end` 分别设置为
`1024`、`0` 和 `10`。那么 DAMON_RECLAIM 将对从地址 0 开始、长度为 10 KiB
的物理地址范围工作（`[0 ** 1024, 10 ** 1024)`，以字节计）。

`bytes_reclaim_tried_regions` 和 `bytes_reclaimed_regions` 也以该单位计。
例如，假设 `addr_unit`、`bytes_reclaim_tried_regions` 和
`bytes_reclaimed_regions` 的值分别为 `1024`、`42` 和 `32`。这意味着
DAMON_RECLAIM 尝试回收了共 42 KiB 内存，并成功回收了 32 KiB 内存。

如果不确定，只需使用默认值（`1`）并忽略此项。

### skip_anon


跳过匿名页的回收。

如果此参数设为 `Y`，DAMON_RECLAIM 不会回收匿名页。默认 `N`。


### kdamond_pid


DAMON 线程的 PID。

如果 DAMON_RECLAIM 已启用，该值即为工作线程的 PID。否则为 -1。

### nr_reclaim_tried_regions


DAMON_RECLAIM 尝试回收的内存区域数量。

### bytes_reclaim_tried_regions


DAMON_RECLAIM 尝试回收的内存区域的总字节数。

### nr_reclaimed_regions


DAMON_RECLAIM 成功回收的内存区域数量。

### bytes_reclaimed_regions


DAMON_RECLAIM 成功回收的内存区域的总字节数。

### nr_quota_exceeds


时间/空间配额限制被超出（exceeded）的次数。

## 示例


以下运行时示例命令让 DAMON_RECLAIM 找出 30 秒或更长时间未被访问的内存
区域并将其换出（page out）。回收被限制为每秒最多 1 GiB，以避免 DAMON_RECLAIM
因换出操作消耗过多 CPU 时间。它还要求 DAMON_RECLAIM 在系统的空闲内存率
超过 50% 时什么都不做，但在该比率低于 40% 时开始实际工作。如果
DAMON_RECLAIM 没有进展，从而空闲内存率低于 20%，则要求 DAMON_RECLAIM 再次
什么都不做，以便回退到基于 LRU 链表、以页为粒度的
```

    # cd /sys/module/damon_reclaim/parameters
    # echo 30000000 > min_age
    # echo $((1 * 1024 * 1024 * 1024)) > quota_sz
    # echo 1000 > quota_reset_interval_ms
    # echo 500 > wmarks_high
    # echo 400 > wmarks_mid
    # echo 200 > wmarks_low
    # echo Y > enabled

```
请注意，此模块（damon_reclaim）不能与其他基于 DAMON 的专用模块同时运行。
更多细节请参考 :ref:`DAMON 设计专用模块互斥性
<damon_design_special_purpose_modules_exclusivity>`。

