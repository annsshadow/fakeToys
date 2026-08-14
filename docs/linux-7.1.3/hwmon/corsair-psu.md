
## 内核驱动 corsair-psu


支持的设备：

- Corsair 电源

  Corsair HX550i

  Corsair HX650i

  Corsair HX750i

  Corsair HX850i

  Corsair HX1000i (Legacy 与 Series 2023)

  Corsair HX1200i (Legacy、Series 2023 与 Series 2025)

  Corsair HX1500i (Legacy 与 Series 2023)

  Corsair RM550i

  Corsair RM650i

  Corsair RM750i

  Corsair RM850i

  Corsair RM1000i

作者：Wilken Gottwalt

### 描述


该驱动为采用 HID 协议接口的 HXi 与 RMi 系列 Corsair 电源实现了 sysfs 接口。
这些电源可访问一个微控制器，该控制器带有 2 个附接的温度传感器、1 个风扇转速传感器、4 个电压电平传感器、4 个功耗传感器与 4 个电流电平传感器，以及运行时间等额外的非传感器信息。

### Sysfs 条目


=======================	========================================================
curr1_input		总电流用量
curr2_input		12v 电源轨上的电流
curr2_crit		12v 电源轨上的电流最大临界值
curr3_input		5v 电源轨上的电流
curr3_crit		5v 电源轨上的电流最大临界值
curr4_input		3.3v 电源轨上的电流
curr4_crit		3.3v 电源轨上的电流最大临界值
fan1_input		电源风扇的转速（RPM）
in0_input		电源交流输入的电压
in1_input		12v 电源轨上的电压
in1_crit		12v 电源轨上的电压最大临界值
in1_lcrit		12v 电源轨上的电压最小临界值
in2_input		5v 电源轨上的电压
in2_crit		5v 电源轨上的电压最大临界值
in2_lcrit		5v 电源轨上的电压最小临界值
in3_input		3.3v 电源轨上的电压
in3_crit		3.3v 电源轨上的电压最大临界值
in3_lcrit		3.3v 电源轨上的电压最小临界值
power1_input		总功耗
power2_input		12v 电源轨上的功耗
power3_input		5v 电源轨上的功耗
power4_input		3.3v 电源轨上的功耗
pwm1			PWM 值，只读
pwm1_enable		PWM 模式，只读
temp1_input		电源 VRM 组件的温度
temp1_crit		电源 VRM 组件温度的最大临界值
temp2_input		电源外壳的温度
temp2_crit		电源外壳温度的最大临界值
=======================	========================================================

### 使用说明


它是一个 USB HID 设备，因此支持自动检测、热插拔以及同时使用多个设备。

电源轨电压电平的闪烁可能是电源故障的迹象。根据默认的自动风扇转速策略，风扇在功率达到额定值的约 30% 时启动。如果未发生，则很可能是风扇故障。该驱动还通过 debugfs 提供一些额外的有用值，这些值不属于 hwmon 类的范畴。

### Debugfs 条目


=======================	========================================================
ocpmode                 PCIe 电源连接器的单轨或多轨模式
product                 电源的产品名称
uptime			电源的会话运行时间
uptime_total		电源的总运行时间
vendor			电源的厂商名称
=======================	========================================================
