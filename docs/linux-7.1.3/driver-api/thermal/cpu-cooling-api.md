## CPU 散热 API 使用说明


作者：Amit Daniel Kachhap <amit.kachhap@linaro.org>

更新015 1 6 
Copyright (c)  2012 Samsung Electronics Co., Ltd(http://www.samsung.com)

## 0. 简

通用CPU 散热（频率限制，freq clipping）向调用者提供注注销 API。将散热设备绑定触发点（trip point）留给用户完成。注API 返回散热设备指针
## 1. CPU 散热 API


### 1.1 cpufreq 注册/注销 API

```

	struct thermal_cooling_device
	*cpufreq_cooling_register(struct cpumask *clip_cpus)

    该接口函数以名称 "thermal-cpufreq-%x" 注册 cpufreq 散热设备。该 API 可支持多    cpufreq 散热设备实例
   clip_cpus:
	将施加频率约束的 CPU cpumask
    ::

	struct thermal_cooling_device
	*of_cpufreq_cooling_register(struct cpufreq_policy *policy)

    该接口函数以名称 "thermal-cpufreq-%x" 注册 cpufreq 散热设备，并将其与一个设备树
    节点关联，以便通过 thermal DT 代码进行绑定。该 API 可支持多cpufreq 散热设备实例
    policy:
	CPUFreq policy銆?

    ::

	void cpufreq_cooling_unregister(struct thermal_cooling_device *cdev)

    该接口函数注销 "thermal-cpufreq-%x" 散热设备
    cdev: 需要注销的散热设备指针
```
## 2. 功耗模

功API 注册函数CPU 提供了一个简单的功耗模型。当前功耗按动态功耗计算（静态功当前不支持）。该功耗模型要CPU 的工作点（operating-points）已使用内核OPP 库注册，
`cpufreq_frequency_table` 已赋CPU `struct device`。如果你使用
CONFIG_CPUFREQ_DT，那`cpufreq_frequency_table` 应该已经赋给CPU 设备
处理器的动态功耗消耗取决于许多因素。对于给定的处理器实现，主要因素有：

- 处理器花费在运行、消耗动态功耗的时间，与处于空闲状态、动态消耗可忽略的时间之比  这里我们称其为“利用率（utilisation）”- DVFS 产生的电压与频率水平。DVFS 水平是支配功耗的主导因素- 在运行时间内，“执行”行为（指令类型、内存访问模式等）在多数情况下造成二阶变化  在极端情况下这种变化可能很显著，但通常其影响远小于上述因素
```

	Pdyn = f(run) * Voltage^2 * Frequency * Utilisation

```
这里f(run) 表示上述执行行为，其结果单位Watts/Hz/Volt^2（常表示mW/MHz/uVolt^2）
f(run) 的详细行为可以建模为在线（on-line）模型。然而实际上，这样的在线模型依赖于若实现特定的处理器支持与特性刻画因素。因此，在初始实现中该项贡献用一个常数系数表示这是一个与整体功耗变化相对贡献相一致的简化
```

	Pdyn = Capacitance * Voltage^2 * Frequency * Utilisation

```
其中 `capacitance` 是一个常数，以基础单位 mW/MHz/uVolt^2 表示指示性的运行时动态功系数。移CPU 的典型值可能在 100 500 之间。作为参考，ARM Juno 开发平台中 SoC 近似值对Cortex-A57 簇为 530，对Cortex-A53 簇为 140