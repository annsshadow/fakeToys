
## Intel Uncore 频率调节


:Copyright: |copy| 2022-2023 Intel Corporation

:Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>

### 简

在基Intel 的工作负载特征下，uncore（非核）部分会消耗相当可观的功耗为了优化总功耗并提升整体性能，SoC 内部提供了用于调uncore 频率的算法这些算法监控 uncore 的工作负载使用情况，并设置一个合适的频率
用户可能uncore 性能有不同的期望，并希望对其加以控制。其目标类似于允用户通过 cpufreq sysfs 接口设置缩放的最最大频率来提升 CPU 性能用户可能有一些对延迟敏感的工作负载，不希uncore 频率发生任何变化。此外，
用户也可能有在分阶段阶段需要不core uncore 性能的工作负载，并可能希同时使用 cpufreq uncore 缩放接口来分配功耗、提升整体性能
### Sysfs 接口


为了控制 uncore 频率，在以下目录提供sysfs 接口`/sys/devices/system/cpu/intel_uncore_frequency/`
对于每一package（封装）die 的组合都有一个目录，因为 uncore 缩放控制作用范围，在die/package SoC 上是die 划分，而在package die SoC 上是package 划分。目录名称代表了控制的作用范围。例如：
'package_00_die_00' 对应 package id 0 die 0
每个 package_**_die_** 包含以下属性：

`initial_max_freq_khz`
	复位后，该属性代表可能的最大频率	这是一个只读属性。如果用户调整了 max_freq_khz	他们随时可以使用该属性的值回到最大频率
`initial_min_freq_khz`
	复位后，该属性代表可能的最小频率	这是一个只读属性。如果用户调整了 min_freq_khz	他们随时可以使用该属性的值回到最小频率
`max_freq_khz`
	该属性用于设uncore 的最大频率
`min_freq_khz`
	该属性用于设uncore 的最小频率
`current_freq_khz`
	该属性用于获取当uncore 频率
### 带有 TPMI（Topology Aware Register and PM Capsule Interface，拓扑感知寄存器与功耗管理胶囊接口）SoC


一SoC 可以包含多个功耗域，其中含有独立或成组mesh（网状）分区。这分区被称fabric cluster（互连簇）
某些类型mesh 需要以相同频率运行，它们会被放在同一fabric cluster 中fabric cluster 的好处在于，它提供了一种可扩展的机制来处理 SoC 中分区化互连结构
当前sysfs 接口支持package die 级别进行控制。该接口不足以支持在
fabric cluster 级别进行更精细的控制
支持 TPMI（Topology Aware Register and PM Capsule Interface，拓扑感知寄存器功耗管理胶囊接口）SoC 可以拥有多个功耗域。每个功耗域可包含一个或多个
fabric cluster銆。
为了package die 级别控制（如同不支持 TPMI 的系统）之外，还能表fabric cluster 级别的控制，sysfs 得到了增强。这种更精细的接口在 sysfs 以名"uncore" 前缀的目录呈现。例如：uncore00、uncore01 等
控制的作用范围由目录中的 "package_id"domain_id" "fabric_cluster_id"
属性指定
每个目录中的属性：

`domain_id`
	该属性用于获取该实例的功耗域 id
`die_id`
	该属性用于获取该实例Linux die id	该属性仅在使core agent 的域中存在，
	且当 CPUID leaf 0x1f 提供 die ID 时才出现
`fabric_cluster_id`
	该属性用于获取该实例fabric cluster id
`package_id`
	该属性用于获取该实例package id
`agent_types`
	该属性显示域内存在的所有硬agent（代理）。每agent 能够控制
	一个或多个硬件子系统，包括：core、cache、memory（内存）I/O
其余属性与 package_**_die_** 级别所呈现的相同
在当前大多数用例中，"max_freq_khz" "min_freq_khz" 是在 "package_**_die_**"
级别更新的。以下方式仍将支持该模式
当用户使"package_**_die_**" 级别的控制时，该 package die 中的每一fabric cluster 都会受到影响。例如：用户package_00_die_00 中修改了
"max_freq_khz"，那么具有相package id uncore* 目录中的 "max_freq_khz"
也会被更新。在这种情况下，用户仍可以在每个 uncore* 级别更新 "max_freq_khz"这会更严格。类似地，用户可以在 "package_**_die_**" 级别更新 "min_freq_khz"
以应用到每个 uncore* 级别
"current_freq_khz" 的支持仅存在于每fabric cluster 级别（即uncore* 目录中）
### 效率与延迟的权衡


Efficiency Latency Control（ELC，效率延迟控制）特性可提升每瓦性能。借助该特性，
硬件功耗管理算法会在延迟和功耗之间进行优化权衡。对于一些对延迟敏感的工作负载，
可以通过软件进行进一步调优，以获得期望的性能
硬件以固定间隔监控一个功耗域内所core 的平CPU 利用率，并决定一uncore
频率。虽然这可能带来最佳的每瓦性能，但工作负载可能期望以更高的功耗为代价获得
更高的性能。考虑一个在空闲系统上间歇性唤醒以执行内存读取的应用程序。在这种
情况下，如果硬件降低uncore 频率，那么频率爬升到满足目标性能可能存在延迟
ELC 控制定义了一些可由软件更改的参数。如果平CPU 利用率低于用户定义的阈（下面的 elc_low_threshold_percent 属性），将使用用户定义uncore 下限频率
（下面的 elc_floor_freq_khz 属性），而不是硬件计算出的最小值
类似地，在高负载场景下，CPU 利用率超过高阈值（下面elc_high_threshold_percent
属性）时，频率会以 100MHz 的步长递增，而不是直接跳到最uncore 频率。这避免CPU 利用率突增而立即消耗不必要的高功耗
效率延迟控制的属性：

`elc_floor_freq_khz`
	该属性用于获设置效率延迟下限频率	如果该值低'min_freq_khz'，固件将忽略它
`elc_low_threshold_percent`
	该属性用于获设置效率延迟控制的低阈值。该属性以 CPU 利用率的百分比表示
`elc_high_threshold_percent`
	该属性用于获设置效率延迟控制的高阈值。该属性以 CPU 利用率的百分比表示
`elc_high_threshold_enable`
	该属性用于启禁用效率延迟控制的高阈值。写 '1' 启用0' 禁用
下面的示例系统配置做了如下事情：
  - CPU 利用率低10% 时：uncore 频率设置800MHz
  - CPU 利用率高95% 时：100MHz 步长递增 uncore 频率，直到达到功耗上
  elc_floor_freq_khz:800000
  elc_high_threshold_percent:95
  elc_high_threshold_enable:1
  elc_low_threshold_percent:10
