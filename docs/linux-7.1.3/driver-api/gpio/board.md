## GPIO 映射


本文档说明如何将 GPIO 分配给指定的设备和功能
所有平台都可以启用 GPIO 库，但如果某个平台严格要求必须提GPIO 功能，则需要在Kconfig 中选择 GPIOLIB。之后，GPIO 如何映射取决于该平台使用什么方式来描述其硬布局。目前，映射可以通过设备树（device tree）、ACPI 和平台数据（platform data）来定义
### 设备
在设备树中，GPIO 可以很方便地映射到设备和功能上。具体的做法取决于提供这GPIO GPIO 控制器，请参考你的控制器对应的设备树绑定（device tree bindings）
GPIO 映射定义在消费设备（consumer device）的节点中，位于一个名<function>-gpios 的属性里，其<function> 是驱动将请求的那个功```


	foo_device {
		compatible = "acme,foo";
		...
		led-gpios = <&gpio 15 GPIO_ACTIVE_HIGH>, /* red */
			    <&gpio 16 GPIO_ACTIVE_HIGH>, /* green */
			    <&gpio 17 GPIO_ACTIVE_HIGH>; /* blue */

		power-gpios = <&gpio 1 GPIO_ACTIVE_LOW>;
	};


```
名为 <function>-gpio 的属性也被认为有效，旧的绑定中使用了它，但仅为兼容性而保留，
由于已被弃用，新的绑定中不应再使用
该属性将GPIO 156 17 通过以下方式对驱动可```
	struct gpio_desc *red, *green, *blue, *power;

	red = gpiod_get_index(dev, "led", 0, GPIOD_OUT_HIGH);
	green = gpiod_get_index(dev, "led", 1, GPIOD_OUT_HIGH);
	blue = gpiod_get_index(dev, "led", 2, GPIOD_OUT_HIGH);

	power = gpiod_get(dev, "power", GPIOD_OUT_HIGH);

```
led GPIO 将为高电平有效（active high），power GPIO 为低电平有效（active low（即 gpiod_is_active_low(power) 将返true）
gpiod_get() 系列函数的第二个参数，即 con_id 字符串，必须GPIO 后缀
gpios" "gpio"，由 gpiod 函数在内部自动查找）所对应<function>- 前缀该后缀在设备树中使用。以上面"led-gpios" 为例，作con_id 参数应使用不"-"
的前缀led"
在内部，GPIO 子系统会把传con_id 的字符串GPIO 后缀gpios" "gpio"）拼接，
得到最终的字符串（`snprintf(... "%s-%s", con_id, gpio_suffixes[]`）
### ACPI

ACPI 也以类似DT 的方式支GPIO 的功能名。上面的 DT 示例可以转换为等价的 ACPI 描述
```

	Device (FOO) {
		Name (_CRS, ResourceTemplate () {
			GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
				"\\_SB.GPI0", 0, ResourceConsumer) { 15 } // red
			GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
				"\\_SB.GPI0", 0, ResourceConsumer) { 16 } // green
			GpioIo (Exclusive, PullUp, 0, 0, IoRestrictionOutputOnly,
				"\\_SB.GPI0", 0, ResourceConsumer) { 17 } // blue
			GpioIo (Exclusive, PullNone, 0, 0, IoRestrictionOutputOnly,
				"\\_SB.GPI0", 0, ResourceConsumer) { 1 } // power
		})

		Name (_DSD, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () {
					"led-gpios",
					Package () {
						^FOO, 0, 0, 1,
						^FOO, 1, 0, 1,
						^FOO, 2, 0, 1,
					}
				},
				Package () { "power-gpios", Package () { ^FOO, 3, 0, 0 } },
			}
		})
	}

```
有关 ACPI GPIO 绑定的更多信息，请参Documentation/firmware-guide/acpi/gpio-properties.rst
### 软件节点


软件节点（software nodes）允许板级特定的代码使用 struct software_node struct
property_entry 构造一个内存中、类设备树的结构。随后该结构可以与平台设备关联，使驱能够使用标准的设备属性（device properties）API 来查询配置，就像ACPI 或设备树系统
上一样
由软件节点支持的 GPIO 使用 `PROPERTY_ENTRY_GPIO()` 宏来描述，该宏将代表 GPIO 控制器的
软件节点与消费设备关联起来。它允许消费方使用常规的 gpiolib API，例gpiod_get()、gpiod_get_optional()
代表 GPIO 控制器的软件节点必须挂接GPIO 控制器设—既可以作为主固件节点，也可以作为
次级固件节点
例如，下面是如何描述一个由单个 GPIO 连接LED。这是在旧系统上使用 platform_data 替代方案

	#include <linux/property.h>
	#include <linux/gpio/machine.h>
	#include <linux/gpio/property.h>

	/*
  - 1. 定义 GPIO 控制器的节点	 */
	static const struct software_node gpio_controller_node = {
		.name = "gpio-foo",
	};

	/** 2. 定义 LED 设备的属性**/
	static const struct property_entry led_device_props[] = {
		PROPERTY_ENTRY_STRING("label", "myboard:green:status"),
		PROPERTY_ENTRY_STRING("linux,default-trigger", "heartbeat"),
		PROPERTY_ENTRY_GPIO("gpios", &gpio_controller_node, 42, GPIO_ACTIVE_HIGH),
		{ }
	};

	/** 3. 定义 LED 设备的软件节点**/
	static const struct software_node led_device_swnode = {
		.name = "status-led",
		.properties = led_device_props,
	};

	/*
  - 4. 注册软件节点和平台设备	 */
	const struct software_node *swnodes[] = {
		&gpio_controller_node,
		&led_device_swnode,
		NULL
	};
	software_node_register_node_group(swnodes);

	/*
  - 5. GPIO 控制器的软件节点挂接到设备并注册它	 */
	 static void gpio_foo_register(void)
	 {
		struct platform_device_info pdev_info = {
			.name = "gpio-foo",
			.id = PLATFORM_DEVID_NONE,
			.swnode = &gpio_controller_node
		};

		platform_device_register_full(&pdev_info);
	 }

	// 然后"leds-gpio" 注册一platform_device，并通过 .fwnode
	// 将其&led_device_swnode 关联
关于如何将板文件转换为使用软件节点的完整指南，请参见
Documentation/driver-api/gpio/legacy-boards.rst銆。
### 平台数据

最后，GPIO 还可以通过平台数据绑定到设备和功能。板级代```

	#include <linux/gpio/machine.h>


```
GPIO 通过查找表（tables of lookups）来映射，表中包含如下实```

	GPIO_LOOKUP(key, chip_hwnum, con_id, flags)
	GPIO_LOOKUP_IDX(key, chip_hwnum, con_id, idx, flags)


```
其中

  - key 是提供该 GPIO gpiod_chip 实例的标签，或者是 GPIO 线名  - chip_hwnum GPIO 在芯片内的硬件编号，或U16_MAX 表示 key 是一GPIO 线名  - con_id 是从设备视角看到GPIO 功能名称。它可以NULL，此时将匹配任意功能  - idx GPIO 在功能内的索引  - flags 用于指定以下属性：
 - GPIO_ACTIVE_HIGH	- GPIO 线为高电平有 - GPIO_ACTIVE_LOW	- GPIO 线为低电平有 - GPIO_OPEN_DRAIN	- GPIO 线被配置为开漏（open drain - GPIO_OPEN_SOURCE	- GPIO 线被配置为开源（open source - GPIO_PERSISTENT	- GPIO 线在挂起/恢复（suspend/resume）期间保				  其取值不 - GPIO_TRANSITORY	- GPIO 线是暂时性的，在挂起/恢复期间可能丢失
				  鍏剁數姘旂姸鎬。
将来，这些标志可能会扩展以支持更多属性
注意  1. GPIO 线名称不保证全局唯一，因此会采用找到的第一个匹配项  2. GPIO_LOOKUP() 只是 GPIO_LOOKUP_IDX() idx = 0 时的简便写法
然后可以按如下方式定义查找表，以一个空条目表示表的结束。表中的 'dev_id' 字段是将使用
这些 GPIO 的设备的标识符。它可以NULL，此时将匹配NULL 设备调用 gpiod_get() 的情况

        struct gpiod_lookup_table gpios_table = {
                .dev_id = "foo.0",
                .table = {
                        GPIO_LOOKUP_IDX("gpio.0", 15, "led", 0, GPIO_ACTIVE_HIGH),
                        GPIO_LOOKUP_IDX("gpio.0", 16, "led", 1, GPIO_ACTIVE_HIGH),
                        GPIO_LOOKUP_IDX("gpio.0", 17, "led", 2, GPIO_ACTIVE_HIGH),
                        GPIO_LOOKUP("gpio.0", 1, "power", GPIO_ACTIVE_LOW),
                        { },
                },
        };


```

	gpiod_add_lookup_table(&gpios_table);


```
```
	struct gpio_desc *red, *green, *blue, *power;

	red = gpiod_get_index(dev, "led", 0, GPIOD_OUT_HIGH);
	green = gpiod_get_index(dev, "led", 1, GPIOD_OUT_HIGH);
	blue = gpiod_get_index(dev, "led", 2, GPIOD_OUT_HIGH);

	power = gpiod_get(dev, "power", GPIOD_OUT_HIGH);


```
由于 "led" GPIO 被映射为高电平有效，本示例将把它们信号置1，即点亮 LED。而对于被
映射为低电平有效"power" GPIO，这段代码执行后其实际信号将0。与旧的整型 GPIO 接口
不同，低电平有效（active-low）属性是在映射过程中处理的，因此GPIO 消费方是透明的
一组诸gpiod_set_value() 之类的函数可用于操作这个新的、以描述符为导向的接口
### 引脚数组

除了逐个请求属于某个功能的引脚外，设备也可以请求分配给该功能的一组引脚。这些引如何映射到设备，决定了该数组是否有资格进行快速的位图处理。如果可以，位图将通过
get/set 数组函数在调用方GPIO 芯片相应.get/set_multiple() 回调之间直接传递
为了符合快速位图处理的条件，数组必须满足以下要求：

- 数组成员 0 的引脚硬件编号也必须0- 与成0 属于同一芯片的连续数组成员的引脚硬件编号，也必须与其数组索引相匹配
否则不会使用快速位图处理路径，以避免属于同一芯片但硬件顺序不连续的引脚被分开处理
如果数组符合快速位图处理路径，那么与成0 不同芯片的引脚，以及索引与其硬件引脚编号
不同的引脚，都会被排除在快速路径之外，无论输入还是输出。此外，开漏和开源引脚会排除在快速位图输出处理之外