## OMAP PM 接口


本文档描述了临时的 OMAP PM 接口。驱动开发者使用这些函数，向内核电源管理代码传达最低延迟或吞吐量约束。长远目标是把 OMAP PM 接口的特性合并到 Linux PM QoS 代码之中。

驱动需要表达满足以下条件的电源管理参数：

- 支持 TI SRF 中存在的电源管理参数范围；

- 将驱动与底层的 PM 参数实现相分离，无论其是 TI SRF、Linux PM QoS、Linux 延迟框架，还是其他实现；

- 以基本单位（例如延迟和吞吐量），而非 OMAP 专有或特定 OMAP 变体专有的单位来指定 PM 参数；

- 允许与其他架构（例如 DaVinci）共享的驱动，以不影响非 OMAP 系统的方式添加这些约束；

- 能够立即实现，且对其他架构的干扰最小。


本文档提出 OMAP PM 接口，包含供驱动代码使用的以下五个电源管理函数：

```
   (*pdata->set_max_mpu_wakeup_lat)(struct device *dev, unsigned long t)
```
```
   (*pdata->set_max_dev_wakeup_lat)(struct device *dev, unsigned long t)
```
```
   (*pdata->set_max_sdma_lat)(struct device *dev, long t)
```
```
   (*pdata->set_min_bus_tput)(struct device *dev, u8 agent_id, unsigned long r)
```
```
   (*pdata->get_dev_context_loss_count)(struct device *dev)
```

所有 OMAP PM 接口函数的进一步文档可在 arch/arm/plat-omap/include/mach/omap-pm.h 中找到。


### OMAP PM 层设计为临时方案


目标是最终由 Linux PM QoS 层支持 OMAP3 中存在的电源管理特性范围。随着这一目标实现，使用 OMAP PM 接口的既有驱动可以修改为使用 Linux PM QoS 代码；届时 OMAP PM 接口便可被移除。


### 驱动对 OMAP PM 函数的使用


正如上述示例中的 'pdata' 所示，这些函数通过驱动 `.platform_data` 结构中的函数指针暴露给驱动。这些函数指针由 `board-*.c` 文件初始化，指向相应的 OMAP PM 函数：

- set_max_dev_wakeup_lat 将指向 omap_pm_set_max_dev_wakeup_lat() 等。不支持这些函数的其他架构应将这类函数指针保持为 NULL。

```
        if (pdata->set_max_dev_wakeup_lat)
            (*pdata->set_max_dev_wakeup_lat)(dev, t);
```

这些函数最常见的用法大概是：指定从中断发生到设备变为可访问之间的最大时间。为此，驱动编写者应使 set_max_mpu_wakeup_lat() 函数约束 MPU 唤醒延迟，并使用 set_max_dev_wakeup_lat() 函数约束设备唤醒延迟（从 clk_enable() 到可访问）。例如：

```
        /* Limit MPU wakeup latency */
        if (pdata->set_max_mpu_wakeup_lat)
            (*pdata->set_max_mpu_wakeup_lat)(dev, tc);

        /* Limit device powerdomain wakeup latency */
        if (pdata->set_max_dev_wakeup_lat)
            (*pdata->set_max_dev_wakeup_lat)(dev, td);

        /* total wakeup latency in this example: (tc + td) */
```

可以通过再次调用该函数并传入新值来覆盖 PM 参数。可以通过将 t 参数设为 -1 来移除设置（set_max_bus_tput() 除外，它应以 r 参数设为 0 来调用）。

上述第五个函数 omap_pm_get_dev_context_loss_count()，旨在作为一种优化，使驱动能够判断设备是否已丢失其内部上下文。如果上下文已丢失，驱动必须在继续之前恢复其内部上下文。


### 其他专用接口函数


上面列出的五个函数旨在供任何设备驱动使用。DSPBridge 和 CPUFreq 有一些特殊需求。DSPBridge 以 OPP ID 的形式表达目标 DSP 性能级别。CPUFreq 以 MPU 频率的形式表达目标 MPU 性能级别。OMAP PM 接口为这些专用场景提供了函数，用于将该输入信息（OPP/MPU 频率）转换为底层电源管理实现所需的形式：

6. `(*pdata->dsp_get_opp_table)(void)`

7. `(*pdata->dsp_set_min_opp)(u8 opp_id)`

8. `(*pdata->dsp_get_opp)(void)`

9. `(*pdata->cpu_get_freq_table)(void)`

10. `(*pdata->cpu_set_freq)(unsigned long f)`

11. `(*pdata->cpu_get_freq)(void)`

## 为平台定制 OPP

定义 CONFIG_PM 应当会为硅片启用 OPP 层，并且 OPP 表的注册应当自动进行。然而在特殊情况下，默认的 OPP 表可能需要调整，例如：

 - 启用默认被禁用、但在某平台上可以启用的默认 OPP
 - 在该平台上禁用一个不受支持的 OPP
 - 定义并添加自定义的 OPP 表项

在这些情况下，板级文件需要执行如下额外步骤：

```
	#include "pm.h"
	....
	static void __init omap_xyz_init_irq(void)
	{
		....
		/* Initialize the default table */
		omapx_opp_init();
		/* Do customization to the defaults */
		....
	}
```

注意：
  omapx_opp_init 将依据 omap 系列成为 omap3_opp_init 或相应名称。
