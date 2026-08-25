## 调节器消费者驱动接

本文描述了面向消费者设备驱动的调节器（regulator）接口。术语说明请参阅 overview.txt

## 1. 消费者调节器访问（静态与动态驱动）


```
	regulator = regulator_get(dev, "Vcc");

```
消费者传入其 struct device 指针和电ID。核心随后通过查询机器特定的查找表来找到正确的调节器。如果查找成功，该调用将返回一个指向为该消费者供电的 struct regulator 的指针
```
	regulator_put(regulator);

```
消费者可能由多个调节器供电，例如带有如下代码的编解码器消费者：
```
	struct regulator_bulk_data supplies[2];

	supplies[0].supply = "Vcc"; /* digital core */
	supplies[1].supply = "Avdd"; /* analog */

	ret = regulator_bulk_get(dev, ARRAY_SIZE(supplies), supplies);

	// convenience helper to call regulator_put() on multiple regulators
	regulator_bulk_free(ARRAY_SIZE(supplies), supplies);


```
调节器访问函regulator_get() regulator_put() 通常会分别在你的设备驱动probe() remove() 中调用

## 2. 调节器输出启用与禁用（静态与动态驱动）


```
	int regulator_enable(regulator);

```
注意  在调regulator_enable() 之前，电源可能已经被启用。如果消费者共享该调节器，或者该调节器之前已被引导加载程序或内核板级初始化代码启用，就会发生这种情况
```
	int regulator_is_enabled(regulator);

```
当调节器被启用时，这将返回大0 的值
```
	int regulator_bulk_enable(int num_consumers,
				  struct regulator_bulk_data *consumers);


```
```
	int regulator_disable(regulator);

```
```
	int regulator_bulk_disable(int num_consumers,
			 	   struct regulator_bulk_data *consumers);

```
注意  如果与其他消费者共享，这可能不会禁用电源。只有当启用引用计数为零时，调节器才会被禁用
```
	int regulator_force_disable(regulator);

```
```
	int regulator_bulk_force_disable(int num_consumers,
			 		 struct regulator_bulk_data *consumers);

```
注意  这将立即且强制关闭调节器输出。所有消费者都会被断电
## 3. 调节器电压控制与状态（动态驱动）


一些消费者驱动需要能够动态地改变其供电电压以匹配系统工作点。例如，CPUfreq 驱动可以随频率一起调节电压以节省功耗，SD 驱动可能需要选择正确的卡电压等
```
	int regulator_set_voltage(regulator, min_uV, max_uV);

```
其中 min_uV max_uV 是以微伏为单位的、可接受的最小和最大电压
注意：这可以在调节器启用或禁用时调用。如果在启用时调用，电压会立即改变；否则电压配置会发生变化，并在调节器下次启用时实际设置电压
```
	int regulator_get_voltage(regulator);

```
注意  get_voltage() 无论调节器启用还是禁用都会返回配置的输出电压，不应使用它来判断调节器的输出状态。不过它可以is_enabled() 配合使用，以确定调节器的实际输出电压

## 4. 调节器电流限制控制与状态（动态驱动）


一些消费者驱动需要能够动态地改变其供电电流限制以匹配系统工作点。例如，LCD 背光驱动可以改变电流限制来调节背光亮度，USB 驱动在供电时可能想把限制设为 500mA
```
	int regulator_set_current_limit(regulator, min_uA, max_uA);

```
其中 min_uA max_uA 是以微安为单位的、可接受的最小和最大电流限制
注意  这可以在调节器启用或禁用时调用。如果在启用时调用，电流限制会立即改变；否则电流限制配置会发生变化，并在调节器下次启用时实际设置电流限制
```
	int regulator_get_current_limit(regulator);

```
注意  get_current_limit() 无论调节器启用还是禁用都会返回电流限制，不应使用它来判断调节器的电流负载

## 5. 调节器工作模式控制与状态（动态驱动）


当消费者的工作状态改变时，一些消费者可以通过将为其供电的调节器的工作模式改为更高效来进一步节省系统功耗。例如，消费者驱动空闲后随之消耗更少的电流
调节器的工作模式可以间接或直接地改变
### 间接工作模式控制
消费者驱动可以请求改变为其供电的调节器的工作模式
```
	int regulator_set_load(struct regulator *regulator, int load_uA);

```
这将使核心重新计算调节器上的总负载（基于其所有消费者），并在必要时及允许的情况下改变工作模式，以最佳匹配当前工作负载
load_uA 值可以从消费者的数据手册中确定。例如，大多数数据手册都有表格显示在某些情况下的电流消耗最大值
大多数消费者会使用间接工作模式控制，因为它们不了解调节器，也不知道调节器是否与其他消费者共享
### 直接工作模式控制

定制的或紧密耦合的驱动可能希望根据其工作点直接控制调节器的工作模式。这可以通过以下方式实现```
	int regulator_set_mode(struct regulator *regulator, unsigned int mode);
	unsigned int regulator_get_mode(struct regulator *regulator);

```
直接模式只会被那*了解**该调节器、且未与其他消费者共享该调节器的消费者使用

## 6. 调节器事

调节器可以向消费者通知外部事件。消费者可能在调节器处于压力或故障条件下时收到事件
```
	int regulator_register_notifier(struct regulator *regulator,
					struct notifier_block *nb);

```
```
	int regulator_unregister_notifier(struct regulator *regulator,
					  struct notifier_block *nb);

```
调节器使用内核通知（notifier）框架向感兴趣它们的消费者发送事件
## 7. 调节器直接寄存器访问


某些电源管理硬件或固件被设计成需要对调节器进行底层硬件访问，且不涉及内核。这类设备的例子有：

- 带有压控振荡器和通过 I2C 改变供电电压以实现所需输出时钟频率的控制逻辑的时钟源
- 能够在过热条件下发出任意 I2C 事务来执行系统断电的热管理固
要配置这样的设备/固件，需要将调节器的 I2C 地址、各种调节器寄存器地址等参数配置给它。调节器框架提供以下辅助函数来查询这些细节
总线相关的细节（I2C 地址或传输速率）由
```
	struct regmap *regulator_get_regmap(struct regulator *regulator);

```
要获取调节器电压的硬件寄存器偏移和位掩码
```
	int regulator_get_hardware_vsel_register(struct regulator *regulator,
						 unsigned *vsel_reg,
						 unsigned *vsel_mask);

```
要将调节器框架的电压选择器代码（regulator_list_voltage 使用）转换为可以
```
	int regulator_list_hardware_vsel(struct regulator *regulator,
					 unsigned selector);

```
要访问硬件以启用/禁用调节器，消费者必须使regulator_get_exclusive()，因为如果存在多```
	int regulator_hardware_enable(struct regulator *regulator, bool enable);

```
