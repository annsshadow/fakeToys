## 内核驱动 qnap-mcu-hwmon


本驱动启用部QNAP 网络附加存储（network attached storage）设备上所MCU 
硬件监控与风扇控制功能

Author: Heiko Stuebner <heiko@sntech.de>

### 描述


本驱动实现了一个简单接口，通过设定 PWM 输出值来驱动风扇，并通过 hwmon 
sysfs 接口向用户空间暴露风扇转速（rpm）与机箱温度

通过可选的 'fan1_input' 返回的转速在 MCU 设备内部计算

本驱动在 sysfs 中提供以下传感器访问

=============== ======= =======================================================
fan1_input	ro	风扇转速计速度，单RPM
pwm1		rw	相对速度-255），255=最大速度
temp1_input	ro	测得的温度，单位毫摄氏度（millicelsius
=============== ======= =======================================================

