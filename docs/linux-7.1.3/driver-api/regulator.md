


## Voltage and current regulator API


:Author: Liam Girdwood
:Author: Mark Brown

## Introduction


本框架旨在提供一个标准的内核接口，以控制电压和电流调节器（regulator）。

其意图是让系统能够动态控制调节器的功率输出，从而节省功耗、延长电池寿命。这既适用于电压调节器（其输出电压可控），也适用于电流汇（其电流上限可控）。

请注意，在 Linux 内核源码的 `Documentation/power/regulator` 下提供了额外的（目前更完整的）文档。

### Glossary


Regulator API 使用一些可能不常见的术语：

Regulator（调节器）

    向其他设备供电的电子器件。大多数调节器可以启用和禁用其输出，有些还能控制其输出电压或电流。

Consumer（消费方）

    消耗调节器所提供电力的电子器件。这些器件可以是静态的（只需要一个固定的电源），也可以是动态的（需要在运行时对调节器进行主动管理）。

Power Domain（电源域）

    由给定调节器供电的电子电路，包括调节器和所有消费设备。调节器的配置在电路中的所有组件之间共享。

Power Management Integrated Circuit (PMIC)（电源管理集成电路）

    包含众多调节器、通常还包含其他子系统的 IC。在嵌入式系统中，主 PMIC 通常相当于桌面系统中 PSU（电源）和南桥的组合。

## Consumer driver interface


这提供了与内核时钟框架类似的 API。消费方驱动使用 `get <#API-regulator-get>`__ 和 `put <#API-regulator-put>`__ 操作来获取和释放调节器。提供了用于 `enable <#API-regulator-enable>`__ 和 `disable <#API-regulator-disable>`__ 调节器的函数，以及用于获取和设置调节器运行时参数的函数。

消费方在请求调节器时，使用其电源的符号名称，例如 "Vcc"，这些名称由机器接口映射到实际的调节器设备。

当未使用调节器框架时，会提供一个该 API 的桩（stub）版本，以尽量减少对 ifdef 的使用需求。

### Enabling and disabling


调节器 API 提供了对调节器的引用计数式的启用和禁用。消费方设备使用 `regulator_enable()` 和 `regulator_disable()` 函数来启用和禁用调节器。对这两个函数的调用必须成对平衡。

请注意，由于多个消费方可能正在使用一个调节器，且机器约束可能不允许禁用该调节器，因此不能保证调用 `regulator_disable()` 就一定能导致调节器提供的电源被禁用。消费方驱动应当假设调节器可能一直处于启用状态。

### Configuration


某些消费方设备可能需要能够动态配置其电源。例如，MMC 驱动可能需要为它们的卡选择正确的工作电压。这可以在调节器启用或禁用时进行。

`regulator_set_voltage()` 和 `regulator_set_current_limit()` 函数为此提供了主要接口。两者都接受电压和电流的范围，以支持那些不需要特定取值的驱动（例如，CPU 频率调整通常允许 CPU 在较低频率下使用更宽的供电电压范围，但并不要求必须降低供电电压）。如果需要精确的取值，则最小值和最大值应当相同。

### Callbacks


也可以注册回调以响应诸如调节失败之类的事件。

## Regulator driver interface


调节器芯片的驱动会向调节器核心注册调节器，并向核心提供操作结构（operations structures）。一个通知器（notifier）接口允许将错误情况报告给核心。

注册应当由平台显式完成的设置来触发，该设置提供一个包含约束和电源信息的 struct regulator_init_data。

## Machine interface


本接口提供了一种方式来定义给定系统上调节器如何连接到消费方，以及该系统的有效运行参数是什么。

### Supplies


调节器电源使用 struct `regulator_consumer_supply` 指定。这是在驱动注册时作为机器约束的一部分完成的。

### Constraints


除了定义连接关系外，机器接口还提供约束，用于定义允许客户端执行的操作以及可以设置的参数。这是必需的，因为通常调节器设备提供的灵活性会超过在给定系统上安全使用的范围，例如支持高于消费方额定值的供电电压。

这是通过提供 struct regulation_constraints 在驱动注册时完成的。

约束还可以在约束中指定调节器的初始配置，这对于静态消费方尤其有用。

## API reference


由于内核文档框架的局限性以及源码现有的布局，整个调节器 API 都在此处记录。

   :internal:

   :internal:

   :internal:

   :export:
