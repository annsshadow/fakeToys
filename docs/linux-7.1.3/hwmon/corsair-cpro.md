
## 内核驱动 corsair-cpro


支持的设备：

  - Corsair Commander Pro
  - Corsair Commander Pro (1000D)

Author: Marius Zachmann

### 描述


该驱动为 Corsair Commander Pro 实现 sysfs 接口。Corsair Commander Pro 是一个带有 6 个风扇接口、4 个温度传感器接口和 2 个 Corsair LED 接口的 USB 设备。它可以读取 SATA 电源接口上的电压电平。

### 使用说明


由于它是 USB 设备，支持热插拔。设备会被自动检测。

### Sysfs 条目


======================= =====================================================================
in0_input		 SATA 12v 上的电压
in1_input		 SATA 5v 上的电压
in2_input		 SATA 3.3v 上的电压
temp[1-4]_input		 所连接温度传感器上的温度
fan[1-6]_input		 所连接风扇的转速（rpm）。
fan[1-6]_label		 显示设备检测到的风扇类型。
fan[1-6]_target		 设置风扇转速目标 rpm。
			 读取时，如果在驱动设置过该值则报告上次的值。
			 否则返回错误。
pwm[1-6]		 设置风扇转速。取值 0-255。只有在直接设置过 pwm 时才能读取。
======================= =====================================================================

### Debugfs 条目


======================= ===================
firmware_version	 固件版本
bootloader_version	 引导加载程序版本
======================= ===================
