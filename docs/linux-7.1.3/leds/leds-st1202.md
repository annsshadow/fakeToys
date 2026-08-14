## STMicroelectronics LED1202 内核驱动


### /sys/class/leds/<led>/hw_pattern


为 ST1202 LED 指定硬件模式。该 LED 控制器实现 12 个具有独立调光控制的低边电流源。内部易失性存储器允许用户存储最多 8 种不同的模式。每个模式是一组特定的输出配置，由 PWM 占空比与持续时间（ms）定义。

为兼容硬件模式格式，最多需向 hw_pattern 写入 8 组亮度（PWM）与持续时间元组。

- 最小模式持续时间：22 ms
- 最大模式持续时间：5660 ms

硬件模式值的格式应为：
"brightness duration brightness duration ..."

### /sys/class/leds/<led>/repeat


指定模式重复次数，该值对所有通道通用。默认值为 1；负数和 0 均无效。

该文件始终返回最初写入的重复次数。

向其写入 255 时，所有模式将无限重复。
