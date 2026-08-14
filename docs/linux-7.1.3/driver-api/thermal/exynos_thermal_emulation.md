## Exynos 仿真模式（Emulation Mode）


Copyright (C) 2012 Samsung Electronics

Written by Jonghwa Lee <jonghwa3.lee@samsung.com>

### 描述


Exynos 4x12（4212、4412）与 5 系列为热管理单元（TMU）提供仿真模式。热仿真模式支持对 TMU 操作的软件调试。用户可以借助软件代码手动设置温度，TMU 将从用户值而非传感器值读取当前温度。

启用 CONFIG_THERMAL_EMULATION 选项将使该支持可用。启用后，将创建 sysfs 节点 /sys/devices/virtual/thermal/thermal_zone'zone id'/emul_temp。

sysfs 节点 'emul_node' 的初始状态包含值 0。当你向 sysfs 节点输入任何想要更新的温度时，它会自动启用仿真模式，当前温度将被改变为该值。

（Exynos 还支持用户可变的延迟时间，用于延迟温度的变化。然而，该节点仅使用与真实感测时间相同的延迟，即 938us。）

Exynos 仿真模式要求值变更与启用同步执行。这意味着，当你想要更新任何值（例如延迟或下一个温度）时，必须同时启用仿真模式（或保持该模式处于启用状态）。否则，值将无法更新，并会继续沿用上一次成功的值。因此，该节点仅允许用户更改温度。提供单一接口使其更易于使用。

禁用仿真模式只需向 sysfs 节点写入值 0。

```



  TEMP	120 |
	    |
	100 |
	    |
	 80 |
	    |				 +-----------
	 60 |      			 |	    |
	    |		   +-------------|          |
	 40 |              |         	 |          |
	    |		   |		 |          |
	 20 |		   |		 |          +----------
	    |		   |		 |          |          |
	  0 |______________|_____________|__________|__________|_________
		   A		 A	    A		       A     TIME
		   |<----->|	 |<----->|  |<----->|	       |
		   | 938us |  	 |	 |  |       |          |
  emulation   : 0  50	   |  	 70      |  20      |          0
  current temp:   sensor   50		 70         20	      sensor

```
