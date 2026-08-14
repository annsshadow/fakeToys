## 内核驱动 i2c-mux-gpio


Author: Peter Korsgaard <peter.korsgaard@barco.com>

### 描述


i2c-mux-gpio 是一个 i2c mux 驱动，通过 GPIO 引脚控制的硬件 MUX，从主 I2C 总线
提供对 I2C 总线段的访问。

```

  ----------              ----------  Bus segment 1   - - - - -
 |          | SCL/SDA    |          |-------------- |           |
 |          |------------|          |
 |          |            |          | Bus segment 2 |           |
 |  Linux   | GPIO 1..N  |   MUX    |---------------   Devices
 |          |------------|          |               |           |
 |          |            |          | Bus segment M
 |          |            |          |---------------|           |
  ----------              ----------                  - - - - -

```
主 I2C 总线的 SCL/SDA 根据 GPIO 引脚 1..N 的设置被多路复用到总线段 1..M。

### 用法


i2c-mux-gpio 使用 platform 总线，因此你需要提供一个 struct platform_device，
其 platform_data 指向一个 struct i2c_mux_gpio_platform_data，其中包含主总线的
I2C adapter 编号、要创建的总线段数量，以及用于控制它的 GPIO 引脚。详情请参见
include/linux/platform_data/i2c-mux-gpio.h。

例如，为一个提供 4 个总线段的 MUX，可以这样写：

```

  #include <linux/platform_data/i2c-mux-gpio.h>
  #include <linux/platform_device.h>

  static const unsigned myboard_gpiomux_gpios[] = {
	AT91_PIN_PC26, AT91_PIN_PC25, AT91_PIN_PC24
  };

  static const unsigned myboard_gpiomux_values[] = {
	0, 1, 2, 3
  };

  static struct i2c_mux_gpio_platform_data myboard_i2cmux_data = {
	.parent		= 1,
	.base_nr	= 2, /* optional */
	.values		= myboard_gpiomux_values,
	.n_values	= ARRAY_SIZE(myboard_gpiomux_values),
	.gpios		= myboard_gpiomux_gpios,
	.n_gpios	= ARRAY_SIZE(myboard_gpiomux_gpios),
	.idle		= 4, /* optional */
  };

  static struct platform_device myboard_i2cmux = {
	.name		= "i2c-mux-gpio",
	.id		= 0,
	.dev		= {
		.platform_data	= &myboard_i2cmux_data,
	},
  };

```
如果你在注册时不知道绝对的 GPIO 引脚编号，可以改为提供一个芯片名
（.chip_name）和相对的 GPIO 引脚编号，i2c-mux-gpio 驱动会替你完成相关工作，
包括在该 GPIO 芯片不能立即可用时进行延迟探测（deferred probing）。

### 设备注册


注册你的 i2c-mux-gpio 设备时，你应当将其使用的任一 GPIO 引脚的编号作为设备
ID 传入。这保证了每个实例都有不同的 ID。

或者，如果你不需要稳定的设备名，可以直接传入 PLATFORM_DEVID_AUTO 作为设备
ID，platform 核心会为你的设备分配一个动态 ID。如果你在注册时不知道绝对的
GPIO 引脚编号，这甚至是唯一的选择。
