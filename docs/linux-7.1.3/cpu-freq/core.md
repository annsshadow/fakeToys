
## CPUFreq 核心与 CPUFreq 通知器的一般性描述


作者：
 - Dominik Brodowski  <linux@brodo.de>
 - David Kimdon <dwhedon@debian.org>
 - Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 - Viresh Kumar <viresh.kumar@linaro.org>


   1. CPUFreq 核心与接口
   2. CPUFreq 通知器
   3. 使用工作性能点（OPP）生成 CPUFreq 表

## 1. 一般信息


CPUFreq 核心代码位于 drivers/cpufreq/cpufreq.c。该 cpufreq 代码为 CPUFreq
架构驱动（即真正执行频率切换的那部分代码）以及“通知器（notifier）”提供
标准化的接口。这些是设备驱动或内核的其他部分，它们需要在策略改变时（例如
像 ACPI 这样的热模块）或所有频率改变时（例如计时代码）得到通知，甚至需要
强制某些速度限制（例如 ARM 架构上的 LCD 驱动）。此外，内核“常量”
loops_per_jiffy 会在频率改变时在此处更新。

cpufreq 策略的引用计数由 cpufreq_cpu_get 和 cpufreq_cpu_put 完成，它们
确保 cpufreq 驱动已正确向核心注册，并且在调用 cpufreq_put_cpu 之前不会被
卸载。这也确保了相应的 cpufreq 策略在被使用时不会被释放。

## 2. CPUFreq 通知器


CPUFreq 通知器遵循标准的内核通知器接口。关于通知器的细节参见
linux/include/linux/notifier.h。

有两种不同的 CPUFreq 通知器——策略通知器和切换通知器。


### 2.1 CPUFreq 策略通知器


当创建或移除一个新策略时，会通知这些通知器。

阶段（phase）由传给通知器的第二个参数指定。当策略首次创建时阶段为
CPUFREQ_CREATE_POLICY，移除策略时为 CPUFREQ_REMOVE_POLICY。

第三个参数是一个 `void *pointer`，指向一个 struct cpufreq_policy，
其中包含若干值，包括 min、max（新策略的上下限频率，单位 kHz）。


### 2.2 CPUFreq 切换通知器


对于策略中的每个在线 CPU，当 CPUfreq 驱动切换 CPU 核心频率且该改变没有
任何外部影响时，会通知这些通知器两次。

第二个参数指定阶段——CPUFREQ_PRECHANGE 或 CPUFREQ_POSTCHANGE。

第三个参数是一个 struct cpufreq_freqs，包含以下值：

======	======================================
policy	指向 struct cpufreq_policy 的指针
old	旧频率
new	新频率
flags	cpufreq 驱动的标志
======	======================================

## 3. 使用工作性能点（OPP）生成 CPUFreq 表

关于 OPP 的细节，参见 Documentation/power/opp.rst

dev_pm_opp_init_cpufreq_table -
	该函数提供一个即取即用的转换例程，把 OPP 层内部关于可用频率的信息
	翻译成一种可以方便地提供给 cpufreq 的格式。

```

	   Do not use this function in interrupt context.

	Example::

	 soc_pm_init()
	 {
		/* Do things */
		r = dev_pm_opp_init_cpufreq_table(dev, &freq_table);
		if (!r)
			policy->freq_table = freq_table;
		/* Do other things */
	 }

	.. note::

	   This function is available only if CONFIG_CPU_FREQ is enabled in
	   addition to CONFIG_PM_OPP.

```
dev_pm_opp_free_cpufreq_table
	释放由 dev_pm_opp_init_cpufreq_table 分配的表

