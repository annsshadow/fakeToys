## 内核驱动 exynos_tmu


Supported chips:

- ARM Samsung Exynos4, Exynos5 series of SoC

  Datasheet: Not publicly available

Authors: Donggeun Kim <dg77.kim@samsung.com>
Authors: Amit Daniel <amit.daniel@samsung.com>

### TMU 控制器描述：


本驱动允许读取 Samsung Exynos4/5 系列 SoC 内部的温度。

该芯片仅通过一个寄存器暴露测量得到的 8 位温度代码值。
温度可由温度代码换算得出。
共有三个从温度换算为温度代码的公式。

这三个公式如下：
```

	Tc = (T - 25) * (TI2 - TI1) / (85 - 25) + TI1

  2. 单点修整（One point trimming）::

	Tc = T + TI1 - 25

  3. 无修整（No trimming）::

	Tc = T + 50

  Tc:
       温度代码，T：温度，
  TI1:
       25 摄氏度对应的修整信息（存储在 TRIMINFO 寄存器）
       在 25 摄氏度下测得的、保持不变的温度代码
  TI2:
       85 摄氏度对应的修整信息（存储在 TRIMINFO 寄存器）
       在 85 摄氏度下测得的、保持不变的温度代码

```
Exynos4/5 中的 TMU（热管理单元，Thermal Management Unit）在温度超过预定义级别时产生中断。
可配置的阈值最大数量为五个。
```

  Level_0: current temperature > trigger_level_0 + threshold
  Level_1: current temperature > trigger_level_1 + threshold
  Level_2: current temperature > trigger_level_2 + threshold
  Level_3: current temperature > trigger_level_3 + threshold

```
阈值与各个 trigger_level 通过相应的寄存器设置。

当中断发生时，本驱动通过 exynos_report_trigger 函数通知内核热框架。
虽然可以为 level_0 设置中断条件，但它可用于同步降温动作。

### TMU 驱动描述：


```

					Kernel Core thermal framework
				(thermal_core.c, step_wise.c, cpufreq_cooling.c)
								^
								|
								|
  TMU configuration data -----> TMU Driver  <----> Exynos Core thermal wrapper
  (exynos_tmu_data.c)	      (exynos_tmu.c)	   (exynos_thermal_common.c)
  (exynos_tmu_data.h)	      (exynos_tmu.h)	   (exynos_thermal_common.h)

```
a) TMU 配置数据：
		它由通过结构体 exynos_tmu_registers 描述的 TMU 寄存器偏移/位域组成。此外还使用若干其他平台数据（struct exynos_tmu_platform_data）成员来配置 TMU。
b) TMU 驱动：
		该组件初始化 TMU 控制器并设置不同的阈值。它通过调用 exynos_report_trigger 来触发核心热实现。
c) Exynos 核心热封装层（Exynos Core thermal wrapper）：
		它提供 3 个封装函数以使用内核核心热框架，分别是 exynos_unregister_thermal、exynos_register_thermal 和 exynos_report_trigger。
