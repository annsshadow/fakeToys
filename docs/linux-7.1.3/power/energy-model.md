
## 设备的能量模

### 1. 概述


能量模型（Energy Model，EM）框架充当这样一种接口：一侧是了解设备在各种性能等级功耗的驱动，另一侧是希望利用这些信息做出能量感知决策的內核子系统
关于设备功耗的信息来源在不同平台之间差异很大。在某些情况下，这些功耗开销可以使用
devicetree 数据进行估算。在其他情况下，固件会更清楚。或者，用户空间可能处于最有利位置。为了避免每个客户端子系统各自重新实现对每一种可能信息源的支持，EM 框架作为一抽象层介入，它标准化了内核中功耗表的格式，从而避免重复工作
功耗值可以用微瓦表示，也可以用“抽象刻度”表示。多个子系统可能使用 EM，由系统集成负责检查功耗值刻度类型的各项要求是否得到满足。一个例子可以在能量感知调度（Energy-Aware Scheduler）文Documentation/scheduler/sched-energy.rst 中找到。对thermal powercap 等某些子系统，以“抽象刻度”表示的功耗值可能会引发问题。这些子系统
更关注对过去所用功耗的估算，因此可能需要真实的微瓦值。这些要求的一个例子可以在
Documentation/driver-api/thermal/power_allocator.rst 中的智能功耗分配（Intelligent Power
Allocation）部分找到。内核子系统可能实现自动检测，以检查已注册EM 的设备是否具不一致的刻度（基EM 内部标志）。需要牢记的一点是，当功耗值以“抽象刻度”表示时，无推导出以微焦耳为单位的真实能量
下图展示了一个驱动（此处Arm 专用，但该方法适用于任何架构）EM 提供功耗开销示例
```
       +---------------+  +-----------------+  +---------------+
       | Thermal (IPA) |  | Scheduler (EAS) |  |     Other     |
       +---------------+  +-----------------+  +---------------+
               |                   | em_cpu_energy()   |
               |                   | em_cpu_get()      |
               +---------+         |         +---------+
                         |         |         |
                         v         v         v
                        +---------------------+
                        |    Energy Model     |
                        |     Framework       |
                        +---------------------+
                           ^       ^       ^
                           |       |       | em_dev_register_perf_domain()
                +----------+       |       +---------+
                |                  |                 |
        +---------------+  +---------------+  +--------------+
        |  cpufreq-dt   |  |   arm_scmi    |  |    Other     |
        +---------------+  +---------------+  +--------------+
                ^                  ^                 ^
                |                  |                 |
        +--------------+   +---------------+  +--------------+
        | Device Tree  |   |   Firmware    |  |      ?       |
        +--------------+   +---------------+  +--------------+

```
对于 CPU 设备，EM 框架管理系统中的每个“性能域”（performance domain）的功耗表。性能是一组性能被一起缩放的 CPU。性能域通常CPUFreq 策略具有一一映射关系。性能域中的所CPU 必须具有相同的微架构。不同性能域中CPU 可以具有不同的微架构
为了更好地反映由于静态功耗（泄漏）引起的功耗变化，EM 支持在运行时修改功耗值。该机制
依赖 RCU 来释放可修改EM perf_state 表内存。其用户——任务调度器——也使用 RCU 来访该内存。EM 框架提供用于分配/释放可修EM 表新内存API。当给定EM 运行时表实例不再
有拥有者时，旧内存会通过 RCU 回调机制自动释放。这通过 kref 机制进行跟踪。在运行时提EM 的设备驱动应在不再需要时调用 EM API 安全地释放它。EM 框架会在可能时负责清理工作
希望修改 EM 值的内核代码受互斥体保护，以免并发访问。因此，设备驱动代码在尝试修EM 必须在可睡眠上下文（sleeping context）中运行
借助运行时可修改EM，我们将设计从“在整个运行期间单一且静态的 EM”（系统属性）转变“可在运行期间根据例如工作负载而改变的单一 EM”（系统与工作负载属性）
还可以修改每EM 性能状态的 CPU 性能值。因此，完整的功耗与性能曲线（呈指数曲线）可根据例如工作负载或系统属性而改变

### 2. 核心 API


##### 2.1 配置选项


必须使用 CONFIG_ENERGY_MODEL 才能使用 EM 框架

##### 2.2 性能域的注册


#### 'advanced' EM 的注

“advancedEM 之所以得名，是因为允许驱动提供更精确的功耗模型。它不局限于框架中实现的
某些数学公式（如同“simpleEM 的情况）。它可以更好地反映为每个性能状态执行的真实功测量。因此，在考虑 EM 静态功耗（泄漏）很重要的情况下，应优先使用这种注册方法
驱动应通过以下方式将性能域注册到 EM 框架
```

  int em_dev_register_perf_domain(struct device *dev, unsigned int nr_states,
		struct em_data_callback *cb, cpumask_t *cpus, bool microwatts);

```
驱动必须提供一个回调函数，为每个性能状态返<频率, 功 元组。驱动提供的回调函数自由地从任何相关位置（DT、固件……）并以任何必要的方式获取数据。仅对于 CPU 设备，驱必须使用 cpumask 指定性能域的 CPU。对于非 CPU 的其他设备，最后一个参数必须设NULL最后一个参'microwatts' 必须以正确的值设置，这一点很重要。使EM 的内核子系统可能
依赖此标志来检查所EM 设备是否使用相同的刻度。如果存在不同的刻度，这些子系统可能返回警告/错误、停止工作甚panic。有关实现此回调的驱动示例，请参见第 3 节；有关API
的更多文档，请参见第 2.4 节
#### 使用 DT 注册 EM


EM 也可以使OPP 框架以及 DT 中的 "operating-points-v2" 信息来注册。DT 中的每个 OPP
条目都可以用包含微瓦功耗值的属"opp-microwatt" 进行扩展。这OPP DT 属性允许平台注反映总功耗（静+ 动态）EM 功耗值。这些功耗值可能直接来自实验和测量
#### 'artificial' EM 的注

对于缺少每个性能状态功耗值详细信息的驱动，可以选择提供一个自定义回调。回.get_cost()
是可选的，提EAS 使用的“cost”值。这对于仅提CPU 类型之间相对效率信息的平台很有用利用这些信息可以创建抽象功耗模型。但即使抽象功耗模型，考虑到输入功耗值的大小限制，有也难以适配get_cost() 允许提供反映 CPU 效率的“cost”值。这样可以提供与 EM 内部计算
“cost”值的公式所强制的关系不同的 EAS 信息。要为这样的平台注册 EM，驱动必须将标志
'microwatts' 设为 0，提.get_power() 回调并提.get_cost() 回调。EM 框架会在注册
期间正确处理此类平台。对此类平台会设EM_PERF_DOMAIN_ARTIFICIAL 标志。使EM 的其框架应格外注意，正确测试和处理此标志
#### 'simple' EM 的注

“simpleEM 使用框架辅助函数 cpufreq_register_em_with_opp() 注册。它实现的功耗模型与
以下式子相关
```

	Power = C * V^2 * f

```
使用此方法注册的 EM 可能无法正确反映真实设备的物理特性，例如当静态功耗（泄漏）很重要时

##### 2.3 访问性能

有两API 函数提供对能量模型的访问：em_cpu_get() CPU id 作为参数，em_pd_get() 设备指针作为参数。使用哪个接口取决于子系统，但对CPU 设备，这两个函数返回相同的性能
域
CPU 能量模型感兴趣的子系统可以使em_cpu_get() API 获取它。能量模型表在性能域创建时
分配一次，并原样保留在内存中
性能域消耗的能耗可以使em_cpu_energy() API 估算。该估算假设CPU 设备的情况下使用
schedutil CPUfreq 调度器。目前未针对其他类型的设备提供此计算
有关上述 API 的更多详情可`<linux/energy_model.h>` 或第 2.5 节中找到

##### 2.4 运行时修

希望在运行时更新 EM 的驱动应使用以下专用函数来分配已修改 EM 的新实例。该 API 如下
```

  struct em_perf_table __rcu *em_table_alloc(struct em_perf_domain *pd);

```
这允许分配一个结构，其中包含新的 EM 表，以及 EM 框架所需RCU krefstruct
em_perf_table' 包含数组 'struct em_perf_state state[]'，即按升序排列的性能状态列表。该
列表必须由希望更EM 的设备驱动填充。频率列表可以从现有EM（在启动时创建）获取'struct em_perf_state' 中的内容也必须由驱动填充
```

  int em_dev_update_perf_domain(struct device *dev,
			struct em_perf_table __rcu *new_table);

```
驱动必须提供指向已分配并初始化的EM 'struct em_perf_table' 的指针。该EM 将在 EM
框架内被安全使用，并对内核中的其他子系统（thermal、powercap）可见。此 API 的主要设目标是快速，并避免在运行时进行额外的计算或内存分配。当设备驱动中已有预计算EM 时，
应当可以简单地复用它们，且性能开销很低
为了释放驱动先前提供EM（例如当模块

```

  void em_table_free(struct em_perf_table __rcu *table);

```
当没有其他子系统（例EAS）使用它时，这将允许 EM 框架安全地移除该内存
要在其他子系统（thermal、powercap）中使用功耗值，需要调用能够保护读取者并保证 EM
一致性的 API
```

  struct em_perf_state *em_perf_state_from_pd(struct em_perf_domain *pd);

```
它返'struct em_perf_state' 指针，即按升序排列的性能状态数组。此函数必须RCU 读锁
区间（rcu_read_lock() 之后）调用。当不再需EM 表时，需要调rcu_read_unlock()。这EM 可以安全地使RCU 读区间并保护用户。它也允EM 框架管理内存并释放它。有关如何使它的更多详情，请参见3.2 节中的示例驱动
**提供了专API 供设备驱动计em_perf_state** : cost

```

  int em_dev_compute_costs(struct device *dev, struct em_perf_state *table,
                           int nr_states);

```
EM 中的这些“cost”值用EAS。新EM 表应与条目数量和设备指针一起传入。当 cost 值的计算
正确完成时，函数返回值为 0。该函数还负责为每个性能**状*正确设置低效值，并相应地更新
em_perf_state : flags銆。
随后，这样准备好的新 EM 可以传递给 em_dev_update_perf_domain() 函数，从而使其可用
有关上述 API 的更多详情可`<linux/energy_model.h>` 或第 3.2 节中找到，其中包含一示例，展示了设备驱动中更新机制的简单实现

##### 2.5 API 的详细描
   :internal:

   :export:


### 3. 示例


##### 3.1 注册 EM 的示例驱

CPUFreq 框架支持专用回调，用于注*给定 CPU 'policy' 对象EM：cpufreq_driver**
: register_em()。必须针对特定驱动正确实现该回调，因为框架会在设置期间的适当时机调用它本节提供了一个简单示例，展示一CPUFreq 驱动使用（虚构的foo' 协议在能量模型框架中
注册性能域。该驱动实现est_power() 函数，提供给

```

  -> drivers/cpufreq/foo_cpufreq.c

  01	static int est_power(struct device *dev, unsigned long *mW,
  02			unsigned long *KHz)
  03	{
  04		long freq, power;
  05
  06		/* Use the 'foo' protocol to ceil the frequency */
  07		freq = foo_get_freq_ceil(dev, *KHz);
  08		if (freq < 0)
  09			return freq;
  10
  11		/* Estimate the power cost for the dev at the relevant freq. */
  12		power = foo_estimate_power(dev, freq);
  13		if (power < 0)
  14			return power;
  15
  16		/* Return the values to the EM framework */
  17		*mW = power;
  18		*KHz = freq;
  19
  20		return 0;
  21	}
  22
  23	static void foo_cpufreq_register_em(struct cpufreq_policy *policy)
  24	{
  25		struct em_data_callback em_cb = EM_DATA_CB(est_power);
  26		struct device *cpu_dev;
  27		int nr_opp;
  28
  29		cpu_dev = get_cpu_device(cpumask_first(policy->cpus));
  30
  31     	/* Find the number of OPPs for this policy */
  32     	nr_opp = foo_get_nr_opp(policy);
  33
  34     	/* And register the new performance domain */
  35     	em_dev_register_perf_domain(cpu_dev, nr_opp, &em_cb, policy->cpus,
  36					    true);
  37	}
  38
  39	static struct cpufreq_driver foo_cpufreq_driver = {
  40		.register_em = foo_cpufreq_register_em,
  41	};


```
##### 3.2 修改 EM 的示例驱

本节提供了一个简单的热管理驱动修EM 的示例。该驱动实现foo_thermal_em_update()
函数。驱动被唤醒

```

  -> drivers/soc/example/example_em_mod.c

  01	static void foo_get_new_em(struct foo_context *ctx)
  02	{
  03		struct em_perf_table __rcu *em_table;
  04		struct em_perf_state *table, *new_table;
  05		struct device *dev = ctx->dev;
  06		struct em_perf_domain *pd;
  07		unsigned long freq;
  08		int i, ret;
  09
  10		pd = em_pd_get(dev);
  11		if (!pd)
  12			return;
  13
  14		em_table = em_table_alloc(pd);
  15		if (!em_table)
  16			return;
  17
  18		new_table = em_table->state;
  19
  20		rcu_read_lock();
  21		table = em_perf_state_from_pd(pd);
  22		for (i = 0; i < pd->nr_perf_states; i++) {
  23			freq = table[i].frequency;
  24			foo_get_power_perf_values(dev, freq, &new_table[i]);
  25		}
  26		rcu_read_unlock();
  27
  28		/* Calculate 'cost' values for EAS */
  29		ret = em_dev_compute_costs(dev, new_table, pd->nr_perf_states);
  30		if (ret) {
  31			dev_warn(dev, "EM: compute costs failed %d\n", ret);
  32			em_table_free(em_table);
  33			return;
  34		}
  35
  36		ret = em_dev_update_perf_domain(dev, em_table);
  37		if (ret) {
  38			dev_warn(dev, "EM: update failed %d\n", ret);
  39			em_table_free(em_table);
  40			return;
  41		}
  42
  43		/*
  44		 * Since it's one-time-update drop the usage counter.
  45		 * The EM framework will later free the table when needed.
  46		 */
  47		em_table_free(em_table);
  48	}
  49
  50	/*
  51	 * Function called periodically to check the temperature and
  52	 * update the EM if needed
  53	 */
  54	static void foo_thermal_em_update(struct foo_context *ctx)
  55	{
  56		struct device *dev = ctx->dev;
  57		int cpu;
  58
  59		ctx->temperature = foo_get_temp(dev, ctx);
  60		if (ctx->temperature < FOO_EM_UPDATE_TEMP_THRESHOLD)
  61			return;
  62
  63		foo_get_new_em(ctx);
  64	}


```
