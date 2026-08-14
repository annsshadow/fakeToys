
## 如何实现一个新的 CPUFreq 处理器驱动


Authors:

 - Dominik Brodowski  <linux@brodo.de>
 - Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 - Viresh Kumar <viresh.kumar@linaro.org>


   1. 该做什么？
   1.1  初始化
   1.2  每 CPU 初始化
   1.3  verify
   1.4  target/target_index 还是 setpolicy？
   1.5  target/target_index
   1.6  setpolicy
   1.7  get_intermediate 与 target_intermediate
   2. 频率表辅助工具



## 1. 该做什么？


那么，你刚拿到一块全新的 CPU / 芯片组以及它的数据手册，并想为这颗 CPU /
芯片组添加 cpufreq 支持？太好了。下面是一些关于必要工作的提示：


### 1.1 初始化


首先，在一个 __initcall 第 7 级（module_init()）或更晚的函数中，检查当前
内核是否运行在正确的 CPU 和正确的芯片组上。如果是，则使用
cpufreq_register_driver() 向 CPUfreq 核心注册一个 struct cpufreq_driver。

这个 struct cpufreq_driver 应当包含什么？

 .name - 该驱动的名称。

 .init - 指向每策略（per-policy）初始化函数的指针。

 .verify - 指向一个“验证”函数的指针。

 .setpolicy _或_ .fast_switch _或_ .target _或_ .target_index - 关于差异
 见下文。

以及可选的

 .flags - 提供给 cpufreq 核心的提示。

 .driver_data - cpufreq 驱动特有的数据。

 .get_intermediate 和 target_intermediate - 用于在改变 CPU 频率时切换到
 稳定频率。

 .get - 返回 CPU 的当前频率。

 .bios_limit - 返回 CPU 的硬件/BIOS 最大频率限制。

 .exit - 指向一个每策略清理函数的指针，在 CPU 热插拔过程的 CPU_POST_DEAD
 阶段被调用。

 .suspend - 指向一个每策略挂起函数的指针，在关闭中断、且调节器（governor）
 为该策略停止_之后_被调用。

 .resume - 指向一个每策略恢复函数的指针，在关闭中断、且调节器为该策略
 重新启动_之前_被调用。

 .ready - 指向一个每策略就绪函数的指针，在策略完全初始化之后被调用。

 .attr - 指向一个 NULL 结尾的 "struct freq_attr" 列表的指针，用于将值
 导出到 sysfs。

 .boost_enabled - 若置位，则启用 boost 频率。

 .set_boost - 指向一个每策略函数的指针，用于启用/禁用 boost 频率。


### 1.2 每 CPU 初始化


每当一个新的 CPU 被注册到设备模型，或者在 cpufreq 驱动注册自身之后，如果
该 CPU 还不存在 cpufreq 策略，就会调用每策略初始化函数 cpufreq_driver.init。
注意，.init() 和 .exit() 例程只针对策略被调用一次，而不是针对该策略管理的
每个 CPU 调用。它接受一个 ``struct cpufreq_policy *policy`` 作为参数。现在该
做什么？

如果有必要，在你的 CPU 上激活 CPUfreq 支持。

接着，驱动必须填入以下值：

+-----------------------------------+--------------------------------------+
|policy->cpuinfo.min_freq _以及_    |					   |
|policy->cpuinfo.max_freq	    | 该 CPU 支持的最小和最大频率	   |
|				    | （单位 kHz）			   |
+-----------------------------------+--------------------------------------+
|policy->cpuinfo.transition_latency | 该 CPU 在两种频率之间切换所需的	   |
|				    | 时间，单位纳秒			   |
+-----------------------------------+--------------------------------------+
|policy->cur			    | 该 CPU 的当前运行频率		   |
|				    | （如适用）			   |
+-----------------------------------+--------------------------------------+
|policy->min,			    |					   |
|policy->max,			    |					   |
|policy->policy 以及必要时	    |					   |
|policy->governor		    | 必须包含该 CPU 的“默认策略”。稍后   |
|				    | cpufreq_driver.verify 以及二者之一  |
|				    | cpufreq_driver.setpolicy 或	   |
|				    | cpufreq_driver.target/target_index  |
|				    | 会以这些值被调用。		   |
+-----------------------------------+--------------------------------------+
|policy->cpus			    | 用（在线 + 离线）CPU 的掩码更新它，|
|				    | 这些 CPU 与该 CPU 一起进行 DVFS	   |
|				    | （即与它在同一时钟/电压轨上）。	   |
+-----------------------------------+--------------------------------------+

对于设置其中某些值（cpuinfo.min[max]_freq、policy->min[max]），频率表辅助工具
可能会有帮助。关于它们的更多信息，请参阅第 2 节。


### 1.3 verify


当用户决定设置一个新的策略（由 "policy、governor、min、max" 组成）时，必须
对这个策略进行校验，以便把不兼容的值纠正过来。用于校验这些值，函数
cpufreq_verify_within_limits(`struct cpufreq_policy *policy`,
`unsigned int min_freq`, `unsigned int max_freq`) 可能会有帮助。关于频率表
辅助工具的细节，请参阅第 2 节。

你需要确保至少有一个有效的频率（或工作范围）落在 policy->min 和 policy->max
之间。如有必要，先增大 policy->max，只有在这也无法解决时，才降低 policy->min。


### 1.4 target 还是 target_index 还是 setpolicy 还是 fast_switch？


大多数 cpufreq 驱动，甚至大多数 CPU 频率调节算法，只允许将 CPU 频率设置为
预定义的固定值。对于这些，你使用 ->target()、->target_index() 或
->fast_switch() 回调。

一些支持 cpufreq 的处理器会在某些限制之间自行切换频率。这些应当使用
->setpolicy() 回调。


### 1.5. target/target_index


target_index 调用有两个参数：`struct cpufreq_policy *policy` 和 `unsigned
int` index（索引到所暴露的频率表中）。

CPUfreq 驱动必须在这里被调用时设置新的频率。实际频率必须由
freq_table[index].frequency 确定。

即使在之前切换到了中间频率，也应当在出错时恢复到更早的频率（即
policy->restore_freq）。

### 已废弃


target 调用有三个参数：`struct cpufreq_policy *policy`、unsigned int
target_frequency、unsigned int relation。

CPUfreq 驱动必须在这里被调用时设置新的频率。实际频率必须依据以下规则确定：

- 尽量接近 "target_freq"
- policy->min <= new_freq <= policy->max（这必须成立！！！）
- 若 relation==CPUFREQ_REL_L，尝试选择一个大于等于 target_freq 的 new_freq。
  （“L 表示 lowest，但不低于”）
- 若 relation==CPUFREQ_REL_H，尝试选择一个小于等于 target_freq 的 new_freq。
  （“H 表示 highest，但不高于”）

这里频率表辅助工具同样可以帮到你 —— 详情见第 2 节。


### 1.6. fast_switch


这个函数用于从调度器上下文中进行频率切换。并非所有驱动都要求实现它，因为
在这个回调内部不允许睡眠。这个回调必须被高度优化，以尽快完成切换。

这个函数有两个参数：`struct cpufreq_policy *policy` 和 `unsigned int
target_frequency`。


### 1.7 setpolicy


setpolicy 调用只接受一个 `struct cpufreq_policy *policy` 作为参数。你需要把
处理器内或芯片组内动态频率切换的下限设为 policy->min，上限设为 policy->max，
并且——如果支持的话——在 policy->policy 为 CPUFREQ_POLICY_PERFORMANCE 时选择
面向性能的设置，在 CPUFREQ_POLICY_POWERSAVE 时选择面向节能的设置。同时请参考
drivers/cpufreq/longrun.c 中的参考实现。


### 1.8 get_intermediate 与 target_intermediate


仅适用于未设置 target_index() 和 CPUFREQ_ASYNC_NOTIFICATION 的驱动。

get_intermediate 应当返回一个平台想切换到的稳定中间频率，而 target_intermediate()
应当在跳转到与 'index' 对应的频率之前，把 CPU 设置到那个频率。核心会负责发送
通知，驱动不必在 target_intermediate() 或 target_index() 中处理它们。

如果驱动不希望为某个目标频率切换到中间频率，可以从 get_intermediate() 返回
'0'。这种情况下，核心会直接调用 ->target_index()。

注意：->target_index() 在失败时应当恢复到 policy->restore_freq，因为核心会
为它发送通知。


## 2. 频率表辅助工具


由于大多数 cpufreq 处理器只允许被设置为少数几个特定频率，带有一些函数的
“频率表”可以在处理器驱动的某些工作中提供帮助。这样一个“频率表”由一个
struct cpufreq_frequency_table 条目数组组成，其中在 "driver_data" 中保存驱动
特定的值，在 "frequency" 中保存对应的频率，并设置 flags。在表的末尾，你需要
添加一个 frequency 设为 CPUFREQ_TABLE_END 的 cpufreq_frequency_table 条目。而
如果你想跳过表中的某个条目，就把频率设为 CPUFREQ_ENTRY_INVALID。条目不需要按
任何特定顺序排列，但如果排了序，cpufreq 核心对它们做 DVFS 会快一些，因为查找
最佳匹配更快。

如果策略在其 policy->freq_table 字段中包含有效指针，cpufreq 表会由核心自动
校验。

cpufreq_frequency_table_verify() 确保至少有一个有效频率落在 policy->min 和
policy->max 之间，并且满足所有其他标准。这对 ->verify 调用很有帮助。

cpufreq_frequency_table_target() 是对应于 ->target 阶段的频率表辅助工具。
只需把值传递给这个函数，它就会返回包含 CPU 应被设置到的频率的频率表条目。

以下宏可用作遍历 cpufreq_frequency_table 的迭代器：

cpufreq_for_each_entry(pos, table) - 遍历频率表的所有条目。

cpufreq_for_each_valid_entry(pos, table) - 遍历所有条目，但排除
CPUFREQ_ENTRY_INVALID 频率。
使用参数 "pos" —— 作为循环游标的 `cpufreq_frequency_table *`，以及 "table" ——
你想要遍历的 `cpufreq_frequency_table *`。

```
	struct cpufreq_frequency_table *pos, *driver_freq_table;

	cpufreq_for_each_entry(pos, driver_freq_table) {
		/* Do something with pos */
		pos->frequency = ...
	}
```
如果你需要使用 pos 在 driver_freq_table 中的位置，不要对指针做相减，因为这
相当耗费资源。相反，请使用宏 cpufreq_for_each_entry_idx() 和
cpufreq_for_each_valid_entry_idx()。
