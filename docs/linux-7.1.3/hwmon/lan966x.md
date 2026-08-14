## 内核驱动 lan966x-hwmon


支持的芯片：

  - Microchip LAN9668（SoC 内置传感器）

    Prefix: 'lan9668-hwmon'

    Datasheet: https://microchip-ung.github.io/lan9668_reginfo

Authors:

	Michael Walle <michael@walle.cc>

### 说明


该驱动为 Microchip LAN9668 片上温度传感器及其风扇控制器提供支持。它提供一个温度传感器和一个风扇控制器。传感器的温度范围为 -40 到 +125 摄氏度，精度为 +/- 5 摄氏度。风扇控制器具有一个测速（tacho）输入和一个 PWM 输出，PWM 输出频率可在约 20Hz 到约 650kHz 之间定制。

该 SoC 不支持告警。

驱动通过以下 sysfs 文件导出温度值、风扇测速输入和 PWM 设置：

**temp1_input**

**fan1_input**

**pwm1**

**pwm1_freq**
