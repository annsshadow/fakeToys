## 运行性能点（OPP）库


(C) 2009-2010 Nishanth Menon <nm@ti.com>, Texas Instruments Incorporated


  1. 简
  2. OPP 列表的初始注
  3. OPP 搜索函数
  4. OPP 可用性控制函
  5. OPP 数据检索函
  6. 数据结构

## 1. 简


### 1.1 什么是运行性能点（OPP）？


如今的复SoC 由多个协同工作的子模块组成。在一个运行多种用例的操作系统中，并非 SoC 中的所有模块都需要始终以最高性能频率工作。为便于实现这一点，SoC 中的子模块被划分为多个域，允许某些域以较低电压和频率运行，而其他域以较高的电压/频率对运行

设备在每个域所支持的、由频率和电压对组成的离散元组集合，称为运行性能点（Operating Performance Points，简OPP）

例如

Let us consider an MPU device which supports the following:
{300MHz at minimum voltage of 1V}, {800MHz at minimum voltage of 1.2V},
{1GHz at minimum voltage of 1.3V}

我们可以将这些表示为如下三个 OPP，即 {Hz, uV} 元组

- {300000000, 1000000}
- {800000000, 1200000}
- {1000000000, 1300000}

### 1.2 运行性能点库


OPP 库提供了一组辅助函数，用于组织与查OPP 信息。该库位drivers/opp/ 目录，头文件位于 include/linux/pm_opp.h。可以通过电源管理 menuconfig 菜单中的 CONFIG_PM_OPP 来启OPP 库。某SoC（如德州仪器（Texas Instruments）的 OMAP 框架）允许在不需cpufreq 的情况下以某个特OPP 启动

```

 (users)	-> registers a set of default OPPs		-> (library)
 SoC framework	-> modifies on required cases certain OPPs	-> OPP layer
		-> queries to search/retrieve information	->

```
OPP 层期望每个域由一个唯一的设备指针表示。SoC 框架OPP 层注册每个设备的一组初OPP。该列表预期为最优的小数目，通常每个设备5 个。这个初始列表包含框架期望在系统中默认安全启用的一OPP

##### 关于 OPP 可用性的说明


随着系统开始运行，SoC 框架可以基于各种外部因素选择使某OPP 在每个设备上可用或不可用。示例用法：热管理或其他异常情况，此SoC 框架可能选择禁用一个较高频率的 OPP，以安全地继续运行，直到OPP 在可能时重新启用

OPP 库在其实现中支持这一概念。以下操作函数仅对可用的 OPP 起作用：dev_pm_opp_find_freq_{ceil, floor}、dev_pm_opp_get_voltage、dev_pm_opp_get_freq、dev_pm_opp_get_opp_count

dev_pm_opp_find_freq_exact 用于查找 opp 指针，该指针随后可用dev_pm_opp_enable/disable 函数，以按需使某opp 可用

警告：OPP 库的用户如果为某个设备调用了 dev_pm_opp_enable/disable 函数，应当使get_opp_count 刷新其可用计数；触发这些操作的精确机制，或向 cpufreq 等其他依赖子系统发出通知的机制，由使OPP 库的 SoC 专用框架自行决定。在执行这些操作时，同样需要注意刷cpufreq 表

## 2. OPP 列表的初始注

SoC 实现会迭代调dev_pm_opp_add 函数，为每个设备添加 OPP。预SoC 框架会最优地注册 OPP 条目——典型数量少5 个。注OPP 所生成的列表由 OPP 库在设备运行的整个过程中维护。SoC 框架随后可以使用 dev_pm_opp_enable/disable 函数动态控OPP 的可用性

dev_pm_opp_add
	为设备指针所表示的特定域添加一个新OPP
	OPP 由频率和电压定义。一旦添加，OPP 即被视为可用，并可使dev_pm_opp_enable/disable 函数控制其可用性。OPP 库在 dev_pm_opp struct 内部存储并管理此信息
	SoC 框架可以使用此函数，根据 SoC 使用环境的需求定义一个最优列表

	警告
		不要在中断上下文中使用此函数

```

	 soc_pm_init()
	 {
		/* Do things */
		r = dev_pm_opp_add(mpu_dev, 1000000, 900000);
		if (!r) {
			pr_err("%s: unable to register mpu opp(%d)\n", r);
			goto no_cpufreq;
		}
		/* Do cpufreq things */
	 no_cpufreq:
		/* Do remaining things */
	 }

```
## 3. OPP 搜索函数

cpufreq 这样的高层框架以频率为单位工作。为了将频率映射回对应的 OPP，OPP 库提供了便捷函数来搜OPP 库内部管理的 OPP 列表。这些搜索函数在找到匹配时返回代表该 opp 的匹配指针，否则返回错误。这些错误应通过 IS_ERR() 等标准错误检查来处理，并由调用者采取适当措施

这些函数的调用者在使用OPP 后，应当调用 dev_pm_opp_put()。否OPP 的内存将永远不会被释放，从而导致内存泄漏（memleak）

dev_pm_opp_find_freq_exact
	基于**精确**频率和可用性搜OPP。此函数在启用一个默认不可用OPP 时尤其有用
	示例：当 SoC 框架检测到可以使某个更高频率可用的情况时，它可以先使用此函数找到该 OPP，然后再调用 dev_pm_opp_enable 实际使其
```

	 opp = dev_pm_opp_find_freq_exact(dev, 1000000000, false);
	 dev_pm_opp_put(opp);
	 /* dont operate on the pointer.. just do a sanity check.. */
	 if (IS_ERR(opp)) {
		pr_err("frequency not disabled!\n");
		/* trigger appropriate actions.. */
	 } else {
		dev_pm_opp_enable(dev,1000000000);
	 }

	NOTE:
	  This is the only search function that operates on OPPs which are
	  not available.

```
dev_pm_opp_find_freq_floor
	搜索一个可用且频率**至多**等于所提供频率OPP。此函数在搜索较小匹配，或按频率递减顺序处理 OPP 信息时很有用
```

	 freq = ULONG_MAX;
	 opp = dev_pm_opp_find_freq_floor(dev, &freq);
	 dev_pm_opp_put(opp);

```
dev_pm_opp_find_freq_ceil
	搜索一个可用且频率**至少**等于所提供频率OPP。此函数在搜索较大匹配，或按频率递增顺序处理 OPP 信息时很有用
```

	 freq = 0;
	 opp = dev_pm_opp_find_freq_ceil(dev, &freq);
	 dev_pm_opp_put(opp);

	Example 2: A simplified implementation of a SoC cpufreq_driver->target::

	 soc_cpufreq_target(..)
	 {
		/* Do stuff like policy checks etc. */
		/* Find the best frequency match for the req */
		opp = dev_pm_opp_find_freq_ceil(dev, &freq);
		dev_pm_opp_put(opp);
		if (!IS_ERR(opp))
			soc_switch_to_freq_voltage(freq);
		else
			/* do something when we can't satisfy the req */
		/* do other stuff */
	 }

```
## 4. OPP 可用性控制函

OPP 库注册的默认 OPP 列表可能无法满足所有可能的情形。OPP 库提供了一组函数来修改 OPP 列表中某OPP 的可用性。这SoC 框架能够精细地动态控制哪OPP 集合在运行上可用。这些函数用于在某些条件（例如热考虑（如：在温度下降前不要使OPPx））*临时**移除一OPP

警告
	不要在中断上下文中使用这些函数

dev_pm_opp_enable
	使一OPP 可用于运行
	示例：假1GHz OPP 仅在 SoC 温度低于某个阈值时才可用。SoC 框架
```

	 if (cur_temp < temp_low_thresh) {
		/* Enable 1GHz if it was disabled */
		opp = dev_pm_opp_find_freq_exact(dev, 1000000000, false);
		dev_pm_opp_put(opp);
		/* just error check */
		if (!IS_ERR(opp))
			ret = dev_pm_opp_enable(dev, 1000000000);
		else
			goto try_something_else;
	 }

```
dev_pm_opp_disable
	使一OPP 不可用于运行
	示例：假设当温度超过阈值时GHz OPP 将被禁用。SoC 框架的实现可
```

	 if (cur_temp > temp_high_thresh) {
		/* Disable 1GHz if it was enabled */
		opp = dev_pm_opp_find_freq_exact(dev, 1000000000, true);
		dev_pm_opp_put(opp);
		/* just error check */
		if (!IS_ERR(opp))
			ret = dev_pm_opp_disable(dev, 1000000000);
		else
			goto try_something_else;
	 }

```
## 5. OPP 数据检索函

由于 OPP 库对 OPP 信息进行了抽象，因此需要从 dev_pm_opp 结构中提取信息的一组函数。一旦使用搜索函数获取了 OPP 指针，SoC 框架就可以使用以下函数检OPP 层内部所表示的信息

dev_pm_opp_get_voltage
	检opp 指针所表示的电压
	示例：在 cpufreq 切换到不同频率时，SoC 框架需要使regulator 框架OPP 所表示的电压设置到提供该电压的电源管理芯片
```

	 soc_switch_to_freq_voltage(freq)
	 {
		/* do things */
		opp = dev_pm_opp_find_freq_ceil(dev, &freq);
		v = dev_pm_opp_get_voltage(opp);
		dev_pm_opp_put(opp);
		if (v)
			regulator_set_voltage(.., v);
		/* do other things */
	 }

```
dev_pm_opp_get_freq
	检opp 指针所表示的频率
	示例：假SoC 框架使用了几个辅助函数，我们可以传opp 指针，而无需额外传参
```

	 soc_cpufreq_target(..)
	 {
		/* do things.. */
		 max_freq = ULONG_MAX;
		 max_opp = dev_pm_opp_find_freq_floor(dev,&max_freq);
		 requested_opp = dev_pm_opp_find_freq_ceil(dev,&freq);
		 if (!IS_ERR(max_opp) && !IS_ERR(requested_opp))
			r = soc_test_validity(max_opp, requested_opp);
		 dev_pm_opp_put(max_opp);
		 dev_pm_opp_put(requested_opp);
		/* do other things */
	 }
	 soc_test_validity(..)
	 {
		 if(dev_pm_opp_get_voltage(max_opp) < dev_pm_opp_get_voltage(requested_opp))
			 return -EINVAL;
		 if(dev_pm_opp_get_freq(max_opp) < dev_pm_opp_get_freq(requested_opp))
			 return -EINVAL;
		/* do things.. */
	 }

```
dev_pm_opp_get_opp_count
	检索某个设备可用的 opp 数量
	示例：假SoC 中的协处理器需要了解可用的
```

	 soc_notify_coproc_available_frequencies()
	 {
		/* Do things */
		num_available = dev_pm_opp_get_opp_count(dev);
		speeds = kcalloc(num_available, sizeof(u32), GFP_KERNEL);
		/* populate the table in increasing order */
		freq = 0;
		while (!IS_ERR(opp = dev_pm_opp_find_freq_ceil(dev, &freq))) {
			speeds[i] = freq;
			freq++;
			i++;
			dev_pm_opp_put(opp);
		}

		soc_notify_coproc(AVAILABLE_FREQs, speeds, num_available);
		/* Do other things */
	 }

```
## 6. 数据结构

通常，一SoC 包含多个可变的电压域。每个域由一个设备指针表示。其OPP 的关系可
```

  SoC
   |- device 1
   |	|- opp 1 (availability, freq, voltage)
   |	|- opp 2 ..
   ...	...
   |	`- opp n ..
   |- device 2
   ...
   `- device m

```
OPP 库维护一个由 SoC 框架填充、并由上述各类函数访问的内部列表。然而，表示实际 OPP 和域的结构对 OPP 库自身是内部的，以实现跨系统可复用的恰当抽象

struct dev_pm_opp
	OPP 库用于表示单OPP 的内部数据结构。除频率、电压、可用性信息外，它还包OPP 库运行所需的内部记账信息。该结构的指针会被返回给 SoC 框架等用户，用作OPP 层交互时标识某个 OPP 的标识符

	警告
	  用户不应解析或修struct dev_pm_opp 指针。某个实例的默认值由 dev_pm_opp_add 填充，但OPP 的可用性可dev_pm_opp_enable/disable 函数修改

struct device
	这用于向 OPP 层标识一个域。设备的性质及其实现留给 OPP 库的用户（如 SoC 框架）决定

总体而言，从简化的角度看，数据结构操作表示如下
```

  Initialization / modification:
              +-----+        /- dev_pm_opp_enable
  dev_pm_opp_add --> | opp | <-------
    |         +-----+        \- dev_pm_opp_disable
    \-------> domain_info(device)

  Search functions:
               /-- dev_pm_opp_find_freq_ceil  ---\   +-----+
  domain_info<---- dev_pm_opp_find_freq_exact -----> | opp |
               \-- dev_pm_opp_find_freq_floor ---/   +-----+

  Retrieval functions:
  +-----+     /- dev_pm_opp_get_voltage
  | opp | <---
  +-----+     \- dev_pm_opp_get_freq

  domain_info <- dev_pm_opp_get_opp_count

```