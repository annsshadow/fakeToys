## Spreadtrum SC27XX 内核驱动


### /sys/class/leds/<led>/hw_pattern


为 SC27XX LED 指定硬件模式。对于 SC27XX LED 控制器，它仅支持 4 个阶段来构成单个硬件模式，用于配置呼吸模式的上升时间、高电平时间、下降时间和低电平时间。

对于呼吸模式，SC27XX LED 仅在高电平阶段期望一个亮度值。为了兼容硬件模式格式，应将上升阶段、下降阶段和低电平阶段的亮度设为 0。

- 最小阶段时长：125 ms
- 最大阶段时长：31875 ms

由于阶段时长的步进为 125 ms，时长应为 125 的倍数，如 125ms、250ms、375ms、500ms …… 31875ms。

因此硬件模式值的格式应为：
"0 rise_duration brightness high_duration 0 fall_duration 0 low_duration"。
