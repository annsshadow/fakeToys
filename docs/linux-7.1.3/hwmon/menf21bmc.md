## 内核驱动 menf21bmc_hwmon


支持的芯片：

 - MEN 14F021P00

	 前缀menf21bmc_hwmon'

	 扫描的地址

作者：Andreas Werner <andreas.werner@men.de>

### 描述


menf21bmc 是一个板管理控制器（BMC），它提I2C 接口供主机访BMC 实现的功能
该驱动提供对板主电压监控功能的访问电压传感器连接到 BMC ADC 输入，BMC 是一PIC16F917 微控制器
### 使用说明


该驱动是名为 "menf21bmc" MFD 驱动的一部分，不会自动探测设备你必须显式地实例MFD 驱动详见 Documentation/i2c/instantiating-devices.rst
### Sysfs 条目


支持以下属性。所有属性均为只读限值由驱动一次性读取
=============== ==========================
in0_input	+3.3V 输入电压
in1_input	+5.0V 输入电压
in2_input	+12.0V 输入电压
in3_input	+5V 待机输入电压
in4_input	VBAT（板载电池）

in[0-4]_min	最小电压限in[0-4]_max	最大电压限
in0_label	"MON_3_3V"
in1_label	"MON_5V"
in2_label	"MON_12V"
in3_label	"5V_STANDBY"
in4_label	"VBAT"
=============== ==========================
