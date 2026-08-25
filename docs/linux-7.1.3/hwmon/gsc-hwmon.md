## gsc-hwmon 内核驱动


支持的芯片：Gateworks GSC
Datasheet: http://trac.gateworks.com/wiki/gsc
Author: Tim Harvey <tharvey@gateworks.com>

### 描述

该驱动支持对温度传感器、连接到 GSC 的各ADC，以及某些板上可选的 FAN 控制器的
硬件监控

### 电压监控


电压输入根据 GSC 版本和固件在内部或由驱动进行缩放。驱动返回的值不需要进一缩放。电压输入标签提供电压轨名称
inX_input                  测量的电压（mV）inX_label                  电压轨名称

### 温度监控


温度12 位或 10 位分辨率测量，并根据 GSC 版本和固件在内部或由驱动进行缩放驱动返回的值反映毫摄氏度：

tempX_input                测量的温度tempX_label                温度输入名称

### PWM 输出控制


GSC 具有 1 PWM 输出，工作在自动模式，其PWM 值根6 个温度边界进行缩放温度边界为读写，单位为毫摄氏度，只读PWM 值范围为 0（关闭）255（全速）当温度传感器读数低于 pwm1_auto_point1_temp 时，风扇速度将设为最小（关闭）；温度传感器读数等于或超过 pwm1_auto_point6_temp 时，设为最大
pwm1_auto_point[1-6]_pwm       PWM 值pwm1_auto_point[1-6]_temp      温度边界