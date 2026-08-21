
## 设计


## 执行模型与数据结
与监控相关的信息，包括监控请求规格说明以及基DAMON 的操作方案（operation schemes），都存储在一个名DAMON `context` 的数据结构中。DAMON 通过一个名`kdamond` 的内核线程来执行每个 context。多kdamond 可以并行运行，用于不同类型的监控
要了解用户空间如何进行配置以及启停止 DAMON，请参阅 DAMON sysfs 接口 <sysfs_interface> 文档

## 总体架构

DAMON 子系统由三层构成，包
- Operations Set <damon_operations_set>：实DAMON 的基础操作，这些操作依赖于给定的监控目标地址空间以及可用的软硬件原语集合- Core <damon_core_logic>：在 operations set 层之上实现核心逻辑，包括监控开销/精度控制以及基于访问的系统操作，以及
- Modules <damon_modules>：在 core 层之上实现用于各种目的的内核模块，并为用户空间提供接口

## Operations Set 灞。
为了进行数据访问监控以及额外的底层工作，DAMON 需要一组针对特定操作、且依赖于给定目标地址空间并为之优化的实现。例如，下面两个用于访问监控的操作就依赖于地址空间
1. 标识该地址空间的监控目标地址区间2. 检查目标空间中特定地址区间的访问情况
DAMON 将这些实现收敛到一个称DAMON Operations Set 的层中，并定义了该层与上层之间的接口。上层专用于 DAMON 的核心逻辑，包括监控精度与开销的控制机制
因此，DAMON 通过配置核心逻辑以使用合适的 operations set，就可以轻松地扩展到任意地址空间或可用的硬件特性。如果没有可用于特定目的operations set，可以遵循层间接口实现一个新operations set
例如，物理内存、虚拟内存、交换空间、特定进程的内存、NUMA 节点、文件以及后备内存设备等都可以提供支持。此外，如果某些架构或设备支持特殊的优化访问检查特性，它们也可以轻松配置
DAMON 目前提供以下三个 operation set。下面三个小节描述了它们的工作方式
 - vaddr：监控特定进程的虚拟地址空间
 - fvaddr：监控固定的虚拟地址区间
 - paddr：监控系统物理地址空间

要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 进行配置，请参阅文档中的 operations <sysfs_context> 部分

 .. _damon_design_vaddr_target_regions_construction:

### 基于 VMA 的目标地址区间构建

`vaddr` DAMON operations set 的一种机制，用于自动初始化并更新监控目标地址区域，从而覆盖目标进程的整个内存映射
该机制仅适用`vaddr` operations set。对`fvaddr` `paddr` operation set，需要用户手动设置监控目标地址区间
进程的超大虚拟地址空间中，只有很小一部分被映射到物理内存并被访问。因此，跟踪未映射的地址区域纯属浪费。然而，由于 DAMON 可以利用自适应区域调整机制处理一定程度的噪声，跟踪每一次映射并非严格必需，但在某些情况下反而会产生很高的开销。话虽如此，监控目标内部过大的未映射区域仍应被移除，以免占用自适应机制的时间
出于这个原因，该实现将复杂的映射转换为三个不同的区域，覆盖地址空间中每一个已映射的区域。这三个区域之间的两个空隙，就是给定地址空间中两个最大的未映射区域。在大多数情况下，这两个最大的未映射区域分别是堆（heap）与最上方 mmap()-ed 区域之间的空隙，以及最下方 mmap()-ed 区域与栈（stack）之间的空隙。由于这些空隙在通常的地址空间中异常巨大，排除它们就足够了
```
    <heap>
    <BIG UNMAPPED REGION 1>
    <uppermost mmap()-ed region>
    (small mmap()-ed regions and munmap()-ed regions)
    <lowermost mmap()-ed region>
    <BIG UNMAPPED REGION 2>
    <stack>


```
### 基于 PTE Accessed-bit 的访问检
物理和虚拟地址空间的两种实现都使用 PTE Accessed-bit 进行基础访问检查。唯一的区别在于从地址找到相关 PTE Accessed bit 的方式。虚拟地址的实现会为目标任务的地址遍历页表，而物理地址的实现会遍历所有映射到该地址的页表。通过这种方式，实现找到并清除下一次采样目标地址bit(s)，并检查该 bit(s) 在一个采样周期后是否被再次置位。这可能会干扰其他使Accessed bit 的内核子系统，即 Idle page tracking 和回收（reclaim）逻辑。DAMON 不会为避免干Idle page tracking 做任何处理，因此处理这种干扰是系统管理员的责任。不过，它像 Idle page tracking 那样，使`PG_idle` `PG_young` 页标志解决了与回收逻辑的冲突

### 地址单元

DAMON core 层使`unsinged long` 类型表示监控目标地址区间。在某些情况下，给定 operations set 的地址空间可能太大而无法用该类型处理。带有大物理地址扩展ARM2 位）就是一个例子。对于这种情况，提供了一个称`address unit` per-operations set 参数。它表示需要乘core 层地址以计算给定地址空间上真实地址的比例因子。`address unit` 参数的支持取决于operations set 的实现。`paddr` 是唯一支持该参数的 operations set 实现
如果该值小`PAGE_SIZE`，则只能使用 2 的幂

## Core Logics


### 监控

下面四个小节描述DAMON 的各个核心机制以及五个监控属性：`sampling interval`、`aggregation interval`、`update interval`、`minimum number of regions` ``maximum number of regions``
注意，`minimum number of regions` 必须3 或更高。这是因为虚拟地址空间监控被设计为至少处理三个区域，以适应普通虚拟地址空间中常见的两个大型未映射区域。虽然对于像 `paddr` 这样的其operation set 来说，这一限制可能并非严格必要，但目前为了一致性，在所DAMON operation 中都强制执行
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 设置这些属性，请参monitoring_attrs <sysfs_monitoring_attrs> 部分

#### 访问频率监控

DAMON 的输出说明了在给定时长内，哪些页以何种频率被访问。访问频率的分辨率由设置 `sampling interval` `aggregation interval` 来控制。具体来说，DAMON 每个 `sampling interval` 检查一次每个页的访问情况并聚合结果。换句话说，统计每个页的访问次数。每`aggregation interval` 过去后，DAMON 调用用户先前注册的回调函数，以便用户读取聚合结果，然```
    while monitoring_on:
        for page in monitoring_target:
            if accessed(page):
                nr_accesses[page] += 1
        if time() % aggregation_interval == 0:
            for callback in user_registered_callbacks:
                callback(monitoring_target, nr_accesses)
            for page in monitoring_target:
                nr_accesses[page] = 0
        sleep(sampling interval)

```
该机制的监控开销会随着目标工作负载规模的增长而任意增加

#### 基于区域的采
为了避免开销无界增长，DAMON 将假定具有相同访问频率的相邻页归并为一组区域。只要这一假设（同一区域内的页具有相同的访问频率）成立，就只需检查该区域中的一页。因此，对于每个 ``sampling interval``，DAMON 在每个区域中随机选取一页，等待一`sampling interval`，检查该页在此期间是否被访问，如果是，则增加该区域的访问频率计数器。该计数器被称为区域`nr_accesses`。因此，通过设置区域的数量即可控制监控开销。DAMON 允许用户为这种权衡设置区域的最小和最大数量
然而，如果假设无法保证，该方案就无法保持输出的质量

#### 自适应区域调整

即使初始监控目标区域构建得很好地满足了假设（同一区域内的页具有相似的访问频率），数据访问模式也可能动态变化。这将导致监控质量下降。为了尽可能维持该假设，DAMON 根据访问频率自适应地合并和拆分每个区域
对于每个 `aggregation interval`，它比较相邻区域的访问频率（`nr_accesses`）。如果差异较小，且两个区域大小之和小于总区域大小除`minimum number of regions` 的结果，DAMON 就合并这两个区域。如果合并后总区域数仍然高于 ``maximum number of regions``，它会以不断增大的访问频率差阈值重复合并，直到达到区域数量的上限，或者阈值变得高于可能的最大值（`aggregation interval` 除以 `sampling interval`）。然后，在它报告并清除每个区域的聚合访问频率之后，如果拆分后总区域数不会超过用户指定的最大区域数，它就把每个区域拆分成两个或三个区域
通过这种方式，DAMON 在保持用户设置的权衡边界的同时，提供尽力而为（best-effort）的质量与最小的开销

#### 年龄追踪

通过分析监控结果，用户还可以发现某个区域的当前访问模式已经维持了多久。这有助于更好地理解访问模式。例如，可以利用频率和近期性（recency）来实现页面放置算法。为了让这种访问模式维持期的分析更容易，DAMON 在每个区域中维护另一个名`age` 的计数器。对于每``aggregation interval``，DAMON 检查该区域的大小和访问频率（`nr_accesses`）是否发生了显著变化。如果是，则将计数器重置为零；否则，计数器加一

#### 动态目标空间更新处
监控目标地址区间可能会动态变化。例如，虚拟内存可以被动态地映射和取消映射。物理内存可以热插拔（hot-plugged）
由于在某些情况下变化可能相当频繁，DAMON 允许监控操作检查动态变化（包括内存映射变化），并仅在每个用户指定的时间间隔（`update interval`）将其应用到与监控操作相关的数据结构中，例如抽象出来的监控目标内存区域
用户空间可以通过 DAMON sysfs 接口tracepoint 获取监控结果。更多细节，请分别参DAMOS tried regions <sysfs_schemes_tried_regions> tracepoint 的文档

#### 监控参数调优指南

简而言之，根据目的设置 `aggregation interval` 以捕获有意义数量的访问。访问的数量可以使用聚合监控结果快照中区域的 `nr_accesses` `age` 来衡量。该间隔的默认`100ms` 在许多情况下都太短了。将 `sampling interval` 设置为与 `aggregation interval` 成比例。默认情况下，推荐比例为 `1/20`
`Aggregation interval` 应设置为：在该间隔内，工作负载能够产生满足监控目的的访问数量。如果间隔太短，只能捕获到很少的访问。结果就是监控结果看起来所有内容都只是偶尔被访问。对于许多目的而言，那将毫无用处。然而，如果间隔太长，使:ref:`区域调整机制 <damon_design_adaptive_regions_adjustment>` 收敛区域所需的时间可能会过长，这取决于给定目的的时间尺度。如果工作负载实际上只产生很少的访问，但用户认为满足监控目的的访问数量过高，就可能出现这种情况。对于此类情况，应当仔细重新考虑每个 `aggregation interval` 要捕获的目标访问数量。另外请注意，捕获的访问数量不仅`nr_accesses` 表示，也`age` 表示。例如，即使监控结果中每个区域的 `nr_accesses` 都为零，仍然可以使用 `age` 值作为近期性信息来区分区域
因此，`aggregation interval` 的最优值取决于工作负载的访问密集程度。用户应根据监控结果每个聚合快照上捕获的访问数量来调整该间隔
注意，该间隔的默认值是 100 毫秒，在许多情况下都太短，尤其是在大型系统上
`Sampling interval` 定义了每次聚合的分辨率。如果设置得太大，监控结果将看起来像每个区域都只是偶尔被访问，或者都只是频繁被访问。也就是说，区域将基于访问模式变得无法区分，因此在许多用例中结果将毫无用处。如`sampling interval` 太小，不会降低分辨率，但会增加监控开销。如果它足以提供满足给定目的所需的监控结果分辨率，就不应该不必要地进一步降低它。建议将其设置为`aggregation interval` 成比例。默认情况下，比例设`1/20`，目前仍推荐使用
基于该手动调优指南，DAMON 提供了更直观的、基于旋钮（knob）的间隔自动调优机制。详情请参阅 :ref:`该特性的设计文档 <damon_design_monitoring_intervals_autotuning>`
基于上述指南的示例调优，请参阅以下文档
- [monitoring_intervals_tuning_example](monitoring_intervals_tuning_example)


#### 监控间隔自动调优

DAMON 基于 :ref:`调优指南思路 <damon_design_monitoring_params_tuning_guide>` 提供 `sampling interval` ``aggregation interval`` 的自动调优。该调优机制允许用户设置在给定时间间隔内希望通过 DAMON 观察到的目标访问事件数量。该目标可以由用户指定为一个比率，DAMON 观察到的访问事件与在给定数量的聚合（`aggrs`）内测量得到的理论最大事件数量（`access_bp`）之比
DAMON 观察到的访问事件以字节粒度计算，基于 DAMON 区域假设 <damon_design_region_based_sampling>。例如，如果发现一个大小为 `X` 字节、具`Y` `nr_accesses` 的区域，则意味着 DAMON 观察`X * Y` 个访问事件。该区域的理论最大访问事件以同样的方式计算，但将 `Y` 替换为理论最大`nr_accesses`，而它可计算为 `aggregation interval / sampling interval`
该机制计`aggrs` 次聚合的访问事件比率，如果观察到的访问比率低于或高于目标，则按相同比率增大或减小 `sampleing interval` ``aggregation interval``。间隔变化的比率根据当前样本比率与目标比率之间的距离按比例决定
用户可以通过两个参数（`min_sample_us` `max_sample_us`）进一步设置调优机制可以设置的 `sampling interval` 的最小值和最大值。由于调优机制始终以相同比率改变 `sampling interval` `aggregation interval`，每次调优变化后的最小和最`aggregation interval` 可以自动一起设置
默认情况下调优是关闭的，需要由用户显式设置。作为经验法则和帕累托（Parreto）原理，推荐 4% 的访问样本比率目标。注意帕累托原理0/20 规则）被应用了两次。也就是说，假设 4%0% 20%）的 DAMON 观察到的访问事件比率（来源）来捕64%0% 乘以 80%）的真实访问事件（结果）
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 使用该特性，请参阅文档中:ref:`intervals_goal <damon_usage_sysfs_monitoring_intervals_goal>` 部分

### 操作方案

数据访问监控的一个常见目的是基于访问感知的系统效率优化。例如，

    paging out memory regions that are not accessed for more than two minutes

鎴?
    using THP for memory regions that are larger than 2 MiB and showing a high
    access frequency for more than one minute.

此类方案的一种直接方法是 profile-guided（基于性能剖析）优化。也就是说，使用 DAMON 获取工作负载或系统的数据访问监控结果，通过对监控结果进行剖析找到具有特殊特征的内存区域，并针对这些区域进行系统操作变更。变更可以通过修改软件（应用和/或内核）或向其提供建议，或者重新配置硬件来完成。离线和在线方式都可用
其中，在运行时向内核提供建议将是灵活且有效的，因此会被广泛使用。然而，实现此类方案可能带来不必要的冗余和效率低下。如果感兴趣的类型很常见，剖析可能是多余的。在内核与用户空间之间交换包括监控结果和操作建议在内的信息可能效率低下
为了让用户通过卸载（offloading）这些工作来减少此类冗余和效率低下，DAMON 提供了一个称为基于数据访问监控的操作方案（Data Access Monitoring-based Operation Schemes，DAMOS）的特性。它让用户在较高层次指定他们期望的方案。对于此类规格说明，DAMON 启动监控，找到具有感兴趣访问模式的区域，并在每个用户指定的时间间隔（称为 `apply_interval`）内，对这些区域应用用户期望的操作动作
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 设置 `apply_interval`，请参阅 apply_interval_us <sysfs_scheme> 部分

#### 操作动作

用户希望应用到其感兴趣区域的管理的动作。例如，换出页面（paging out）、为下一次回收受害者选择设置优先级、建`khugepaged` 折叠或拆分，或者什么都不做只收集区域的统计信息
受支持的动作列表DAMOS 中定义，但每个动作的实现位于 DAMON operations set 层，因为实现通常依赖于监控目标地址空间。例如，将特定虚拟地址区间换出的代码就不同于物理地址区间的代码。并monitoring operations 实现不强制要求支持列表中的所有动作。因此，特定 DAMOS 动作的可用性取决于选择了哪operations set 一起使用
受支持的动作列表、其含义以及支持每个动作DAMON operations set 如下
 - `willneed`：对该区域调用带 `MADV_WILLNEED` `madvise()`   `vaddr` `fvaddr` operations set 支持 - `cold`：对该区域调用带 `MADV_COLD` `madvise()`   `vaddr` `fvaddr` operations set 支持 - `pageout`：回收该区域   `vaddr`、`fvaddr` `paddr` operations set 支持 - `hugepage`：对该区域调用带 `MADV_HUGEPAGE` `madvise()`   `vaddr` `fvaddr` operations set 支持。当
   TRANSPARENT_HUGEPAGE 被禁用时，该动作的应用将直接失败 - `nohugepage`：对该区域调用带 `MADV_NOHUGEPAGE` `madvise()`   `vaddr` `fvaddr` operations set 支持。当
   TRANSPARENT_HUGEPAGE 被禁用时，该动作的应用将直接失败 - `lru_prio`：在该区域的 LRU 链表上提升其优先级   `paddr` operations set 支持 - `lru_deprio`：在该区域的 LRU 链表上降低其优先级   `paddr` operations set 支持 - `migrate_hot`：迁移区域时优先迁移较热的区域   `vaddr`、`fvaddr` `paddr` operations set 支持 - `migrate_cold`：迁移区域时优先迁移较冷的区域   `vaddr`、`fvaddr` `paddr` operations set 支持 - `stat`：什么都不做，只统计计数   受所operations set 支持
对区域应用除 `stat` 之外的动作被视为改变了区域的特性。因此，DAMOS 在对这些区域应用任何此类动作时会重置区域age
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 设置该动作，请参action <sysfs_scheme> 部分

#### 目标访问模式

方案感兴趣的访问模式。这些模式由 DAMON 监控结果提供的属性构建，具体是大小、访问频率和 age。用户可以通过设置这三个属性的最小值和最大值来描述他们感兴趣的访问模式。如果一个区域的三个属性都在这些范围内，DAMOS 就将其归类为方案感兴趣的区域之一
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 设置访问模式，请参阅 :ref:`access_pattern <sysfs_access_pattern>` 部分

#### 配额

DAMOS 的上限开销控制特性。如果目标访问模式没有被恰当地调优，DAMOS 可能产生很高的开销。例如，如果找到了一个具有感兴趣访问模式的巨大内存区域，对该巨大区域的所有页应用方案的动作可能会消耗不可接受的大量系统资源。通过调优访问模式来防止此类问题可能很有挑战性，特别是如果工作负载的访问模式高度动态
为了缓解这种情况，DAMOS 提供了一个称为配额（quotas）的上限开销控制特性。它让用户指DAMON 可用于应用动作的时间上限，和/或在用户指定的时间持续内可以应用该动作的内存区域的最大字节数
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 设置基本配额，请参阅 quotas <sysfs_quotas> 部分

##### 浼樺厛绾ф帓搴?
在配额限制下做出良好决策的机制。当由于配额限制而无法对所有感兴趣区域应用动作时，DAMOS 对区域进行优先级排序，并只对优先级足够高的区域应用动作，从而不会超过配额
优先级排序机制对于每个动作应当不同。例如，对于换出（page-out）方案动作，会优先处理很少被访问（较冷）的内存区域。相反，对于大页折叠方案动作，较冷的区域会被降优先级。因此，每个动作的优先级排序机制与动作一起实现在每个 DAMON operations set 中
虽然实现取决DAMON operations set，但通常使用区域的访问模式属性来计算优先级是常见的做法。一些用户希望这些机制针对他们的特定情况进行个性化。例如，一些用户希望机制更看重近期性（`age`）而非访问频率（`nr_accesses`）。DAMOS 允许用户指定每个访问模式属性的权重，并将该信息传递给底层机制。尽管如此，权重是否以及如何在多大程度上被尊重，取决于底层的优先级排序机制实现
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 设置优先级权重，请参weights <sysfs_quotas> 部分

##### 面向目标的反馈驱动自动调
自动的反馈驱动配额调优。用户可以不设置绝对的配额值，而是指定他们感兴趣的指标，以及他们希望该指标值达到的目标值。然DAMOS 自动调优相应方案的激进程度（配额）。例如，如果 DAMOS 未达成目标，DAMOS 自动增加配额。如DAMOS 超额达成目标，则减少配额
用户可以按需选择两种此类调优算法
- `consist`：基于比例反馈回路的算法。尝试找到一个应被持续保持的最优配额，以持续达成目标。适用于动态、长时间运行环境下的纯内核操作。这是默认选择。如果不确定，使用这个- `temporal`：更直接的算法。尝试尽快达成目标，使用最大允许的配额，但仅在一个临时的短时间内。当配额未达成时，该算法持续将配额调优到最大允许值。一旦配额[超额]达成，它就将配额设为零。适用于需要确定性控制的环境
目标可以用五个参数指定，`target_metric`、`target_value`、`current_value`、`nid` `path`。自动调优机制试图使 `target_metric` `current_value` `target_value` 相同
- `user_input`：用户提供的值。用户可以使用他们感兴趣的任何指标作为该值。使用空间主工作负载的延迟或吞吐量、系统指标如空闲内存比率或内存压力停滞时间（PSI）都可能是例子。注意在这种情况下用户应显式自行设置 `current_value`。换句话说，用户应重复提供反馈- `some_mem_psi_us`：从上次配额重置到下次配额重置期间测量得到的、系统范围的 `some` 内存压力停滞信息，以微秒计。DAMOS 自行完成测量，因此用户只需要在初始时设`target_value`。换句话说，DAMOS 进行自我反馈- `node_mem_used_bp`：特NUMA 节点的已用内存比率，bp/10,000）计- `node_mem_free_bp`：特NUMA 节点的空闲内存比率，bp/10,000）计- `node_memcg_used_bp`：针对特NUMA 节点的、特cgroup 的节点已用内存比率，bp/10,000）计- `node_memcg_free_bp`：针对特NUMA 节点的、特cgroup 的节点未用内存比率，bp/10,000）计- `active_mem_bp`：active 相对 active + inactive（LRU）的内存大小比率，以 bp/10,000）计- `inactive_mem_bp`：inactive 相对 active + inactive（LRU）的内存大小比率，以 bp/10,000）计
`nid` 仅为 `node_mem_used_bp`、`node_mem_free_bp`、`node_memcg_used_bp` `node_memcg_free_bp` 可选需要，用于指向特定NUMA 节点
`path` 仅为 `node_memcg_used_bp` `node_memcg_free_bp` 可选需要，用于指向 cgroup 的路径。该值应为从 cgroups 挂载点开始的 memory cgroup 的路径
要了解用户空间如何通过 DAMON sysfs 接口 <sysfs_interface> 设置调优目标指标、目标值和/或当前值，请参quota goals <sysfs_schemes_quota_goals> 部分

#### 水印

条件性的 DAMOS 激停用自动化。用户可能希DAMOS 只在特定情况下运行。例如，当保证有充足空闲内存时，运行主动回收（proactive reclamation）方案只会消耗不必要的系统资源。为避免此类消耗，用户需要手动监控某些指标（如空闲内存比率），并打开或关DAMON/DAMOS
DAMOS 允许用户使用三个水印（watermark）来卸载此类工作。它允许用户配置他们感兴趣的指标，以及三个水印值，high、middle low。如果指标值高于高水印或低于低水印，方案被停用。如果指标值低于中水印但高于低水印，方案被激活。如果所有方案都被水印停用，则监控也被停用。在这种情况下，DAMON 工作线程只定期检查水印，因此产生几乎为零的开销
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 设置水印，请参阅 watermarks <sysfs_watermarks> 部分

#### 杩囨护鍣?
非基于访问模式的目标内存区域过滤。如果用户运行自己编写的程序或拥有良好的剖析工具，他们可能知道比内核更多的东西，例如未来的访问模式或某些特殊类型内存的特殊需求。例如，一些用户可能只知道匿名页（anonymous pages）会影响他们程序的性能。他们也可能拥有一份延迟关键（latency-critical）进程列表
为了让用户利用此类特殊知识优DAMOS 方案，DAMOS 提供了一个称DAMOS filters 的特性。该特性允许用户为每个方案设置任意数量的过滤器。每个过滤器指定

- 内存类型（`type`），
- 是适用于该类型的内存还是除该类型之外的所有内存（`matching`），以及
- 是允许（include）还是拒绝（exclude）对内存应用方案的动作（`allow`）
为了高效处理过滤器，某些类型的过滤器core 层处理，而另一些由 operations set 处理。在后一种情况下，过滤器类型的支持取决于 DAMON operations set。对core 层处理的过滤器，被过滤器排除的内存区域不计入方案已尝试作用于该区域。相反，如果一个内存区域被 operations set 层处理的过滤器过滤掉，则计入方案已尝试。这一差异影响统计信息
当安装了多个过滤器时，由 core 层处理的过滤器组先被评估。之后，operations 层处理的过滤器组被评估。每个组内的过滤器按其安装顺序评估。如果一部分内存与某个过滤器匹配，则忽略后续过滤器。如果该部分因为没有匹配任何过滤器而通过过滤器评估阶段，对其应用方案的动作取决于最后一个过滤器的允许类型。如果最后一个过滤器是允许性的，则该部分内存将被拒绝，反之亦然
例如，假设按此顺序安装了 1) 一个允许匿名页的过滤器2) 另一个拒绝年轻（young）页的过滤器。如果某个有资格应用方案动作的区域中的一页是匿名页，无论它是否年轻，方案的动作都将被应用到该页，因为它与第一个允许过滤器匹配。如果该页不是匿名页但年轻，则方案的动作不会被应用，因为第二个拒绝过滤器阻止了它。如果该页既不是匿名页也不是年轻页，由于没有匹配的过滤器，该页将通过过滤器评估阶段，并且该动作将被应用到该页
目前支持以下 `type` 的过滤器
- Core 层处    - addr
        - 应用于属于给定地址区间的页    - target
        - 应用于属于给DAMON 监控目标的页- Operations 层处理，仅受 `paddr` operations set 支持    - anon
        - 应用于包含未存入文件的数据的页    - active
        - 应用active 页    - memcg
        - 应用于属于给cgroup 的页    - young
        - 应用于在方案上次访问检查之后被访问过的页    - hugepage_size
        - 应用于以给定大小范围管理的页    - unmapped
        - 应用于未映射的页
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 设置过滤器，请参filters <sysfs_filters> 部分

#### 统计

为帮助监控、调优和调试 DAMOS 而设计的 DAMOS 行为统计
DAMOS 从方案开始执行起，为每个方案累计以下统计信息
- `nr_tried`：方案尝试应用的区域总数- `sz_tried`：方案尝试应用的区域总大小- `sz_ops_filter_passed`：通过 operations set 层处理的 DAMOS 过滤器的字节总数- `nr_applied`：方案已应用的区域总数- `sz_applied`：方案已应用的区域总大小- `qt_exceeds`：方案的配额被超过的总次数- `nr_snapshots`：方案尝试应用的 DAMON 快照总数- `max_nr_snapshots`：`nr_snapshots` 的上限
“一个方案尝试应用到某个区域”意味着 DAMOS 核心逻辑确定该区域有资格应用方案:ref:`动作 <damon_design_damos_action>`。在核心逻辑上处理的 :ref:`访问模式 <damon_design_damos_access_pattern>`ref:`配额 <damon_design_damos_quotas>`ref:`水印 <damon_design_damos_watermarks>` :ref:`过滤<damon_design_damos_filters>` 都可能影响这一点。核心逻辑只会要求底层:ref:`operation set <damon_operations_set>` 对该区域应用该动作，因此该动作是否真的被应用并不明确。这就是它被称为“tried（尝试）”的原因
“一个方案被应用到某个区域”意味着 :ref:`operation set <damon_operations_set>` 已对该区域至少一部分应用了该动作。由 operation set 处理的过滤器 <damon_design_damos_filters>，以及动<damon_design_damos_action> 的类型和该区域的页都可能影响这一点。例如，如果设置了排除匿名页的过滤器，而该区域只有匿名页，或者动作是 `pageout` 而该区域的所有页都不可回收，则对该区域应用该动作将失败
与普通统计不同，`max_nr_snapshots` 由用户设置。如果它被设为非零，`nr_snapshots` 等于或大`nr_snapshots`，则该方案被停用
要了解用户空间如何通过 :ref:`DAMON sysfs 接口 <sysfs_interface>` 读取这些统计，请参阅 :ref:s`stats <sysfs_stats>` 部分

#### 区域遍历

DAMOS 特性，允许用户访问某个 DAMOS 动作刚刚应用到的每个区域。使用该特性，DAMON API <damon_design_api> 允许用户访问区域的完整属性，包括访问监控结果以及通过 DAMOS 过滤器的该区域内部内存的数量。DAMON sysfs 接口 <sysfs_interface> 也允许用户通过特殊文件 <sysfs_schemes_tried_regions> 读取这些数据

### 应用程序编程接口

面向内核空间数据访问感知应用程序的编程接口。DAMON 是一个框架，因此它本身不做任何事情。相反，它只帮助其他内核组件（如子系统和模块）使DAMON 的核心特性构建它们的数据访问感知应用程序。为此，DAMON 通过名为 `include/linux/damon.h` 的应用程序编程接口，将其所有特性暴露给其他内核组件。接口详情请参阅 API [文档 </mm/damon/api>](文档 </mm/damon/api>)

## 模块

由于 DAMON 的核心是面向内核组件的框架，它不提供任何直接的用户空间接口。此类接口应该由每个 DAMON API 用户内核组件来实现。DAMON 子系统本身实现了此类 DAMON API 用户模块，这些模块用于通用 DAMON 控制和特殊目的的数据访问感知系统操作，并为用户空间提供稳定的应用程序二进制接口（ABI）。用户空间可以使用这些接口构建其高效的数据访问感知应用程序

### 通用用户接口模块

在运行时为通用 DAMON 使用提供用户空间 ABI DAMON 模块
像许多其ABI 一样，这些模块在伪文件系统（如 'sysfs'）上创建文件，允许用户通过写入和读取这些文件向 DAMON 指定请求并从 DAMON 获取答案。作为对此类 I/O 的响应，DAMON 用户接口模块按照用户通过 DAMON API 的请求控DAMON 并检索结果，然后将结果返回给用户空间
这些 ABI 是为用户空间应用程序开发设计的，而非为人手操作。建议人类用户使用此类用户空间工具。其中一个用 Python 编写的用户空间工具可Github（https://github.com/damonitor/damo）、Pypi（https://pypistats.org/packages/damo）以及多个发行版（https://repology.org/project/damo/versions）获取
目前，此类型有一个模块，'DAMON sysfs 接口' 可用。接口详情请参阅 ABI 文档 <sysfs_interface>

### 特殊目的访问感知内核模块

为特定目DAMON 使用提供用户空间 ABI DAMON 模块
DAMON 用户接口模块用于在运行时完全控制所DAMON 特性。对于每个特殊目的的系统范围数据访问感知系统操作（如主动回收LRU 链表平衡），可以通过移除针对该特定目的的不必要的旋钮来简化接口，并扩展到启动时刻甚至编译时刻的控制。用于该用途的 DAMON 控制参数的默认值也需要针对该目的进行优化
为支持此类情况，还提供了更多提供更简单、更优化的用户空间接口的 DAMON API 用户内核模块。目前，提供了两个用于主动回收和 LRU 链表操作的模块。更多细节，请阅读这些用途文档（[/admin-guide/mm/damon/stat](/admin-guide/mm/damon/stat)、[/admin-guide/mm/damon/reclaim](/admin-guide/mm/damon/reclaim) [/admin-guide/mm/damon/lru_sort](/admin-guide/mm/damon/lru_sort)）

注意，这些模块目前以独占方式运行。如果其中某一个已经在运行，其他模块在启动请求时将返回 `-EBUSY`

### 示例 DAMON 模块

提供示例 DAMON 内核 API 用法DAMON 模块
内核程序员可以使DAMON 内核 API 构建他们自己的特殊或通用目的 DAMON 模块。为了帮助他们轻松理解如何使DAMON 内核 API，在 linux 源码树的 `samples/damon/` 下提供了一些示例模块。请注意，这些模块并非为在真实产品中使用而开发，而只是为了展示如何以简单方式使DAMON 内核 API