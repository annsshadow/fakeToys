## I2C SMBus 简

I²C（读作：I 平方 C，在内核文档中写I2C）是Philips 开发的一种协议。它是一种两线制
协议，速度可变（通常最400 kHz，高速模式最5 MHz）。它为连接许多具有不频繁或低带宽
通信需求的设备类型提供了一种廉价的总线。I2C 广泛用于嵌入式系统。某些系统使用不满足品牌
要求的变体，因此不宣传为 I2C，而以不同的名称出现，例如 TWI（Two Wire Interface）、IIC
最新的官方 I2C 规范是由 NXP Semiconductors 发布`"I²C-bus specification and user
manual" (UM10204) <https://www.nxp.com/docs/en/user-guide/UM10204.pdf>`_，撰写本文时版本7
SMBus（System Management Bus）基I2C 协议，且大多I2C 协议与信号的子集。许I2C 设备
可以SMBus 上工作，但某SMBus 协议增加了超出实I2C 品牌所需语义的内容。现PC 主板
依赖 SMBus。通过 SMBus 连接的最常见设备是使I2C EEPROM 配置RAM 模块，以及硬件监视芯片
由于 SMBus 大多是通用 I2C 总线的子集，我们可以在许I2C 系统上使用其协议。然而，也存同时不满SMBus I2C 电气约束的系统，以及其他无法实现所有常SMBus 协议语义或消息的
系统

## 术语


I2C 总线连接一个或多个控制器（controller）芯片与一个或目标（target）芯片
   :alt:    一个控制器3 个目标的简I2C 总线

   简单的 I2C 总线

**控制器（controller* 芯片是启动与目标通信的节点。在 Linux 内核实现中也被称为“适配（adapter）”或“总线（bus）”。控制器驱动通常位于 `drivers/i2c/busses/` 子目录
**算法（algorithm* 包含可用于实现一整类 I2C 控制器的通用代码。每个具体的控制器驱动要依赖 `drivers/i2c/algos/` 子目录中的算法驱动，要么包含其自有实现
**目标（target* 芯片是在被控制器寻址时响应通信的节点。在 Linux 内核实现中也被称为“客户端
（client）”。虽然目标通常是独立的外部芯片，Linux 也可以充当目标（需要硬件支持）并响应总线
上的另一个控制器。这被称*本地目标（local target*。相比之下，外部芯片被称*远程目标
（remote target*
目标驱动保存在与其提供的特性相关的特定目录中，例如 GPIO 扩展器在 `drivers/gpio/`，与视频
相关的芯片在 `drivers/media/i2c/`
对于上图中的示例配置，你需要一I2C 控制器驱动，以及你的 I2C 目标驱动。通常每个目标一驱动
### 同义

如上所述，Linux I2C 实现历史上对控制器使用术语“adapter”，对目标使用术语“client”。一些数结构在其名称中包含这些同义词。因此，在讨论实现细节时，你也应该了解这些术语。不过，官方
措辞更受青睐
### 过时的术

在较早的 I2C 规范中，控制器被称为“master”，目标被称为“slave”。这些术语已在规范第 7 版中
被废弃，Linux 内核行为准则也不鼓励使用它们。你可能仍会在尚未更新的文档引用中发现它们。不过，
通用的态度是使用包容性的术语：controller target。在 Linux 内核中替换旧术语的工作正在进行中