## 内核驱动 thmc50


支持的芯片：

  - Analog Devices ADM1022

    Prefix: 'adm1022'

    扫描地址：I2C 0x2c - 0x2e

    Datasheet: http://www.analog.com/en/prod/0,2877,ADM1022,00.html

  - Texas Instruments THMC50

    Prefix: 'thmc50'

    扫描地址：I2C 0x2c - 0x2e

    Datasheet: https://www.ti.com/


Author: Krzysztof Helt <krzysztof.h1@wp.pl>

本驱动派生自 2.4 内核的 thmc50.c 源文件。

Credits:

  thmc50.c (2.4 内核):

 - Frodo Looijaard <frodol@dds.nl>
 - Philip Edelbrock <phil@netroedge.com>

### 模块参数


- adm1022_temp3: short 数组
    列出 adapter,address 对，用于将芯片强制置为带第二个远程温度的 ADM1022 模式。
    这对于原始的 THMC50 芯片不起作用。

### 描述


THMC50 实现了：一个内部温度传感器、对一个外部二极管型温度传感器的支持（与许多
处理器内部的二极管传感器兼容），以及一个可控的风扇/模拟输出 DAC。对于温度
传感器，可以通过相应的 Overtemperature Shutdown 寄存器与 Hysteresis 寄存器设置
限制。每个值都可以以半度的精度进行设置和读取。当温度高于 Overtemperature
Shutdown 值时，会发出一个告警（通常发给相连的 LM78）；该告警会一直保持，直到
温度降到 Hysteresis 值以下。所有温度都以摄氏度为单位，并保证在 -55 到 +125 度的
范围内。

THMC50 每 1.5 秒才更新一次其值；更频繁地读取不会造成损害，但会返回“旧”值。

THMC50 通常与类似 LM78 的芯片配合使用，用于测量处理器温度。

ADM1022 的工作方式与 THMC50 相同，但它更快（5 Hz，而 THMC50 为 1 Hz）。它也可以
被置于一种新模式以处理额外的远程温度传感器。默认情况下，驱动使用由 BIOS 设置的
模式。

如果 BIOS 有问题且模式设置不正确，你可以使用 adm1022_temp3 参数强制设置为带
额外远程温度的模式。错误设置的典型症状是被强制全速运转的风扇。

### 驱动特性


本驱动最多提供三个温度：

temp1
 - 内部
temp2
 - 远程
temp3
 - 仅 ADM1022 的第二个远程

pwm1
 - 风扇转速（0 = 停止，255 = 全速）
pwm1_mode
 - 始终为 0（DC 模式）

pwm1 设为 0 同时也会强制芯片发出 FAN_OFF 信号，因此即使向 ANALOG_OUT 寄存器写入
0 值不会停转风扇，它也会让风扇停止。

本驱动在 Compaq AP550 上进行了测试，该机器带有两块 ADM1022 芯片（其中一块工作于
temp3 模式），五个温度读数与两个风扇。
