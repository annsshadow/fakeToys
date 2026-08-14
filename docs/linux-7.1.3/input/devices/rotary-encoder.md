## rotary-encoder - 一个用于 GPIO 连接设备的通用驱动


:Author: Daniel Mack <daniel@caiaq.de>, 2009 年 2 月

### 功能


旋转编码器是通过两根线与 CPU 或其他外设连接的设备。输出相位差为 90 度，通过在下降沿和上升沿触发，可以确定转动方向。

有些编码器在稳定状态下两个输出都为低电平，另一些在稳定状态下两个输出都为高电平（半周期模式），还有一些在每个步进都有稳定状态（四分之一周期模式）。

```

                  _____       _____       _____
                 |     |     |     |     |     |
  Channel A  ____|     |_____|     |_____|     |____

                 :  :  :  :  :  :  :  :  :  :  :  :
            __       _____       _____       _____
              |     |     |     |     |     |     |
  Channel B   |_____|     |_____|     |_____|     |__

                 :  :  :  :  :  :  :  :  :  :  :  :
  Event          a  b  c  d  a  b  c  d  a  b  c  d

                |<-------->|
	          one step

                |<-->|
	          one step (half-period mode)

                |<>|
	          one step (quarter-period mode)

```
更多信息，请参见
	https://en.wikipedia.org/wiki/Rotary_encoder


### 事件 / 状态机


在半周期模式下，使用上述状态 a) 和 c) 基于最后一个稳定状态来确定旋转方向。事件在状态 b) 和 d) 中上报，前提是新的稳定状态与上一个不同（即旋转没有在中途反转）。

此外，以下情况适用：

a) 通道 A 上的上升沿，通道 B 处于低电平
	此状态用于识别顺时针转动

b) 通道 B 上的上升沿，通道 A 处于高电平
	进入此状态时，编码器被置于“armed”状态，意味着它已经看到了一个单步转换的一半路程。

c) 通道 A 上的下降沿，通道 B 处于高电平
	此状态用于识别逆时针转动

d) 通道 B 上的下降沿，通道 A 处于低电平
	停车位置。如果编码器进入此状态，应当已经发生了一个完整的转换，除非它在中途翻转回来。“armed”状态告诉我们这一点。

### 平台要求


由于此驱动中没有任何与硬件相关的调用，使用它的平台必须支持 gpiolib。另一个要求是 IRQ 必须能够在两个边沿上触发。


### 板级集成


要在你的系统中使用此驱动，需注册一个名为 'rotary-encoder' 的 platform_device，并将 IRQ 和某些特定平台数据与之关联。由于该驱动使用通用设备属性，这可以通过设备树、ACPI 或使用静态板文件来完成，如下例所示：

```

	/* board support file example */

	#include <linux/input.h>
	#include <linux/gpio/machine.h>
	#include <linux/property.h>

	#define GPIO_ROTARY_A 1
	#define GPIO_ROTARY_B 2

	static struct gpiod_lookup_table rotary_encoder_gpios = {
		.dev_id = "rotary-encoder.0",
		.table = {
			GPIO_LOOKUP_IDX("gpio-0",
					GPIO_ROTARY_A, NULL, 0, GPIO_ACTIVE_LOW),
			GPIO_LOOKUP_IDX("gpio-0",
					GPIO_ROTARY_B, NULL, 1, GPIO_ACTIVE_HIGH),
			{ },
		},
	};

	static const struct property_entry rotary_encoder_properties[] = {
		PROPERTY_ENTRY_U32("rotary-encoder,steps-per-period", 24),
		PROPERTY_ENTRY_U32("linux,axis",		      ABS_X),
		PROPERTY_ENTRY_U32("rotary-encoder,relative_axis",    0),
		{ },
	};

	static const struct software_node rotary_encoder_node = {
		.properties = rotary_encoder_properties,
	};

	static struct platform_device rotary_encoder_device = {
		.name		= "rotary-encoder",
		.id		= 0,
	};

	...

	gpiod_add_lookup_table(&rotary_encoder_gpios);
	device_add_software_node(&rotary_encoder_device.dev, &rotary_encoder_node);
	platform_device_register(&rotary_encoder_device);

	...

```
请参阅设备树绑定文档以了解该驱动支持的所有属性。
