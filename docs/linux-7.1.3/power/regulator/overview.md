## Linux 电压与电流调节器框架


## 概述


该框架旨在提供一个标准的内核接口来控制电压和电流调节器
其意图是让系统能够动态控制调节器的功率输出来节省功耗、延长电池寿命。这既适用电压调节器（电压输出可控），也适用于电流阱（电流限制可控）
(C) 2008  Wolfson Microelectronics PLC.

Author: Liam Girdwood <lrg@slimlogic.co.uk>


## 术语


本文档使用的一些术语：

  - Regulator（调节器                 - 为其他设备供电的电子设备。大多数调节器可以启用和禁用                   输出，有些可以控制其输出电压或电流
                   输入电压 -> 调节-> 输出电压


  - PMIC
                 - 电源管理 IC（Power Management IC）。一种包含众多调节器                   通常还包含其他子系统IC

  - Consumer（消费者）
                 - 由调节器供电的电子设备。消费者可分为两类

                   静态（Static）：消费者不改变其供电电压或电流限制。它只需
                   启用或禁用其电源。其供电电压由硬件、bootloader、固件或内核
                   板级初始化代码设置
                   动态（Dynamic）：消费者需要改变其供电电压或电流限制以满足
                   运行需求

  - Power Domain（电源域                 - 其输入功率由调节器、开关或另一个电源域的输出功率提供的
                   电子电路
```
                     Regulator -+-> Switch-1 -+-> Switch-2 --> [Consumer A]
                                |             |
                                |             +-> [Consumer B], [Consumer C]
                                |
                                +-> [Consumer D], [Consumer E]

                   That is one regulator and three power domains:

                   - Domain 1: Switch-1, Consumers D & E.
                   - Domain 2: Switch-2, Consumers B & C.
                   - Domain 3: Consumer A.

                   and this represents a "supplies" relationship:

                   Domain-1 --> Domain-2 --> Domain-3.

                   A power domain may have regulators that are supplied power
                   by other regulators. i.e.::

                     Regulator-1 -+-> Regulator-2 -+-> [Consumer A]
                                  |
                                  +-> [Consumer B]

                   This gives us two regulators and two power domains:

                   - Domain 1: Regulator-2, Consumer B.
                   - Domain 2: Consumer A.

                   and a "supplies" relationship:

                   Domain-1 --> Domain-2


  - Constraints（约束）
                 - 约束用于定义性能与硬件保护的功率级别。约束存在于三个层级
                   Regulator Level（调节器层级）：由调节器硬件工作参数定义                   并在调节器数据手册中指定。即

                     - 电压输出范围800mV -> 3500mV                     - 调节器电流输出限制在 5V 时为 20mA，但10V 时为 10mA
                   Power Domain Level（电源域层级）：由内核级板级初始化代码在
                   软件中定义。它用于将一个电源域约束到特定的功率范围。即

                     - Domain-1 电压3300mV
                     - Domain-2 电压1400mV -> 1600mV
                     - Domain-3 电流限制0mA -> 20mA
                   Consumer Level（消费者层级）：由消费者驱动动态设置电压或
                   电流限制级别
                   例如，一个消费者背光驱动请求将电流5mA 增加10mA 以提                   LCD 亮度。这会按如下方式穿过各层级：-

                   消费者（Consumer）：需要提LCD 亮度。查找并请求亮度                   中的下一个电mA 值（同一参考设备可以有不同personality                   消费者驱动可据此复用）
                   电源域（Power Domain）：新的电流限制是否在该域及系统状                   （例如电池供电、USB 供电）的运行限制内
                   调节器域（Regulator Domains）：新的电流限制是否在输输出
                   电压的调节器工作参数内
                   如果该调节器请求通过了所有约束测试，则应用新的调节器值

```
## 设计


该框架针对基SoC 的设备设计和打造，但也可能与非 SoC 设备相关，并被拆分为以下
四个接口

   1. 消费者驱动接口（Consumer driver interface）
      它使用的 API 与内核时钟接口类似，消费者驱动可以获取和释放一个调节器
      （就像现在对时钟所做的那样），并获设置电压、电流限制、模式、启用和
      禁用。这应能让消费者完全控制其供电电压和电流限制。如果未使用，它也会      编译掉，以便驱动可以在没有基于调节器的电源控制的系统中复用
        See Documentation/power/regulator/consumer.rst

   2. 调节器驱动接口（Regulator driver interface）
      这允许调节器驱动注册其调节器并向核心提供操作。它还有一个通知调用链，
      用于将调节器事件传播给客户端
        See Documentation/power/regulator/regulator.rst

   3. 机器接口（Machine interface）
      该接口用于机器特定的代码，允许为每个调节器创建电电流域（带约束）      它可以提供调节器约束，防止有缺陷的客户端驱动通过过压或过流损坏设备      它还允许创建调节器树，其中某些调节器由其他调节器供电（类似于时钟树）
        See Documentation/power/regulator/machine.rst

   4. 用户空间 ABI（Userspace ABI）
      该框架还通过 sysfs 向用户空间导出大量有用的电压/电流/操作模式数据      这可用于帮助监控设备功耗和状态
        See Documentation/ABI/testing/sysfs-class-regulator
