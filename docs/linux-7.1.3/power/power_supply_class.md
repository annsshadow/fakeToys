## Linux 电源（power supply）类


#### 概要


电源类用于向用户空间表示电池、UPS、交流（AC）或直流（DC）电源的属性
它定义了一组核心属性，这些属性应当适用于（几乎）所有的电源。属性可以通过
sysfs uevent 接口获取
每个属性都有定义良好的含义，包括所用的度量单位。尽管所提供的属性被认为普遍
适用于任何电源，但具体的监测硬件可能无法提供全部属性，因此其中任何一个都可以
被省略
电源类是可扩展的，并允许驱动定义它们自己的属性。核心属性集会遵循标准的 Linux
演进规则（即，如果发现某个属性适用于许多电源类型或其驱动，就可以把它加入核属性集）
它还LED 框架集成，用于提供通常所期望的电池充电中/已充满状态以AC/USB 电源在线状态的反馈。（注意，指示的具体细节——包括是否使用它——完全由用户
或具体机器的默认值控制，这符LED 框架的设计原则。）


#### 属特

电源类预定义了一组属性。这消除了驱动之间的代码重复。电源类坚持要求复用预定义的属*以及**它们的单位
因此，用户空间对于任何种类的电源都能得到一组可预测的属性和单位，并能够以一致的
方式对它们进行处呈现给用户。不同电源和机器上的结果也可以直接比较
参见 drivers/power/supply/ds2760_battery.c 以了解如何声明和处理属性的示例

#### 单位


引自 include/linux/power_supply.h
  除另有说明外，所有的电压、电流、电荷、能量、时间和温度，单位分别为
  µV、µA、µAh、µWh、秒和十分之一摄氏度。把原始值转换为本类所使用单位的工  由驱动负责

#### 属特性详

+--------------------------------------------------------------------------+
|               **Charge/Energy/Capacity —如何不混*                  |
+--------------------------------------------------------------------------+
| **由于“charge”（电荷量，µAh）和“energy”（能量，µWh）都表示电池“capacity”|
| （容量），本类对这两个术语作了区分。不要把它们混为一谈！**              |
|                                                                          |
| - `CHARGE_*`                                                             |
|	属性仅µAh 表示容量                                           |
| - `ENERGY_*`                                                             |
|	属性仅µWh 表示容量                                           |
| - `CAPACITY`                                                             |
|	属性以**百分*表示容量，范围从 0 100                        |
+--------------------------------------------------------------------------+

后缀
_AVG
  **硬件**平均值，只有在你的硬件确实能够报告平均值时才使用它_NOW
  瞬时/即时值
STATUS
  该属性表示运行状态（充电中、已充满、放电中（即正在为负载供电）等）。它
  对应battery.h 中定义的 `BATTERY_STATUS_*` 值
CHARGE_TYPE
  电池通常可以以不同的速率充电。这里定义了涓流充电和快速充电。对于已  充满或正在放电的电池，可以显'n/a'（或者，如果状态未知，则显'unknown'）
AUTHENTIC
  表示连接到平台的电源（电池或充电器）是正1)还是非正0)
HEALTH
  表示电池的健康状况。值对应于 battery.h 中定义的 POWER_SUPPLY_HEALTH_*
VOLTAGE_OCV
  电池的开路电压
VOLTAGE_MAX_DESIGN, VOLTAGE_MIN_DESIGN
  电源最大和最小电压的设计值。最最小指的是电池在正常条件下被认为“满
  “空”时的电压值。是的，电压与电池容量之间没有直接关系，但某些简易电  利用电压来粗略估算容量。电池驱动也可以用这个属性来告知用户空间某块给定
  电池的最大和最小电压阈值
VOLTAGE_MAX, VOLTAGE_MIN
  _DESIGN 电压值相同，只是当硬件只能猜测（测量并保留）给定电源的阈值时  应当使用这两个值
VOLTAGE_BOOT
  报告在启动期间测量到的电
CURRENT_BOOT
  报告在启动期间测量到的电
CHARGE_FULL_DESIGN, CHARGE_EMPTY_DESIGN
  设计电荷值，即电池被认为空时的电荷
ENERGY_FULL_DESIGN, ENERGY_EMPTY_DESIGN
  同上，但针对能量
CHARGE_FULL, CHARGE_EMPTY
  这些属性意为“电池变变空时最后记住的电荷值”。它们也可以表示“在给定
  条件（温度、老化）下电池被认为满/空时的电荷值”。也就是说，这些属性表  真实的阈值，而非设计值
ENERGY_FULL, ENERGY_EMPTY
  同上，但针对能量
CHARGE_COUNTER
  当前的电荷计数器（单µAh）。它很容易为负；不存在空或满的值。它仅对
  基于时间的相对测量有用
PRECHARGE_CURRENT
  充电周期预充阶段的最大充电电流（通常为电池容量的 20%）
CHARGE_TERM_CURRENT
  充电终止电流。当电池电压高于再充电阈值，且充电电流低于该设置值（通常  电池容量10%）时，充电周期终止
CONSTANT_CHARGE_CURRENT
  由充电器设定的恒流充电电流
CONSTANT_CHARGE_CURRENT_MAX
  电源对象支持的最大充电电流
CONSTANT_CHARGE_VOLTAGE
  由充电器设定的恒压充电电压
CONSTANT_CHARGE_VOLTAGE_MAX
  电源对象支持的最大充电电压
INPUT_CURRENT_LIMIT
  由充电器设定的输入电流限制。表示从一个充电源汲取的电流INPUT_VOLTAGE_LIMIT
  由充电器设定的输入电压限制。表示来自充电源的电压限制INPUT_POWER_LIMIT
  由充电器设定的输入功率限制。表示来自充电源的功率限制
CHARGE_CONTROL_LIMIT
  当前的充电控制限制设置CHARGE_CONTROL_LIMIT_MAX
  最大充电控制限制设置
CALIBRATE
  电池或库仑计数器的校准状
CAPACITY
  以百分数表示的容量CAPACITY_ALERT_MIN
  最小容量告警值（百分数）CAPACITY_ALERT_MAX
  最大容量告警值（百分数）CAPACITY_LEVEL
  容量等级。对应于 POWER_SUPPLY_CAPACITY_LEVEL_*
TEMP
  电源的温度TEMP_ALERT_MIN
  最小电池温度告警TEMP_ALERT_MAX
  最大电池温度告警TEMP_AMBIENT
  环境温度TEMP_AMBIENT_ALERT_MIN
  最小环境温度告警TEMP_AMBIENT_ALERT_MAX
  最大环境温度告警TEMP_MIN
  可工作的最低温TEMP_MAX
  可工作的最高温
TIME_TO_EMPTY
  电池被认为变空（即电池为负载供电时）前剩余的秒数
TIME_TO_FULL
  电池被认为变满（即电池正在充电时）前剩余的秒

#### 电池 <-> 外部电源交互


电源常常同时充当供电方和被供电方。电池就是一个很好的例子。所以，电池通常
关心自己是否被外部供电
针对这种情况，电源类为电池实现了一套通知机制
一个外部电源（AC）在 "supplied_to" 结构体成员中列出被供电方（电池）的名称，
而外部电源发出的每一power_supply_changed() 调用都会通过
external_power_changed 回调来通知被供电方

#### Devicetree 电池特

驱动应当调用 power_supply_get_battery_info() 来从 devicetree 电池节点获取
电池特性，该节点定义于 Documentation/devicetree/bindings/power/supply/
battery.yaml。这drivers/power/supply/bq27xxx_battery.c 中有实现
struct power_supply_battery_info 中的属性，以及它们在电池节点中的对应项其名称对应于 enum power_supply_property 中的元素，以保持 sysfs 属性与电池
节点属性在命名上的一致性

#### 问答（Q&A

Q:
    POWER_SUPPLY_PROP_XYZ 属性在哪里A:
    如果你找不到适合你的驱动需求的属性，欢迎添加它，并连同你的驱动一起提    补丁
    当前可用的属性就是已编写的驱动当前所提供的那些
    未来适合添加的候选属性有：型部件号、循环时间、制造商等

Q:
    我有一些非常特殊的属性（例如电池颜色）。我应该把它加入标准属性吗A:
    很可能不该加。如果这样的属性有用，可以放在驱动自身之中。当然，如果所
    讨论的属性适用于由许多驱动提供的大量电池，或来自某个通用的电池规
    标准，那么它或许有资格被加入核心属性集

Q:
    假设我的电池监测芯片/固件不提供以百分数表示的容量，但提供    charge_{now,full,empty}。我应该在驱动内部手动计算百分比容量，并注册
    CAPACITY 属性吗？关time_to_empty/time_to_full 也是同样的问题A:
    很可能不该。本类的设计目标是导出可由当前可用具体硬件直接测量的属性
    使用某些启发式或数学模型去推断不可得的属性，不属于电池驱动的工作范畴    此类功能应当被抽离出去；事实上，apm_power——用于在电源类之上为传统 APM
    API 提供服务的驱动——使用了一种简单的启发式方法，基于电荷、电流、电压等
    来近似剩余电池容量。但一个完整的电池模型很可能根本不属于内核的范畴，因为
    它将需要浮点运算来处理微分方程和卡尔曼滤波之类的事情。这部分最好由
    batteryd/libbattery 处理，不过它们还有待编写