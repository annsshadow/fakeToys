## Linux 下的闪光 LED 处理


某些 LED 设备提供两种模式——手电筒（torch）与闪光（flash）。在 LED 子系统中，这两种模式分别LED 类（Documentation/leds/leds-class.rst）与 LED 闪光类支持。手电筒模式相关特性默认启用，而闪光模式特性仅在驱动通过设置 `LED_DEV_CAP_FLASH` 标志声明支持时才启用

为了启用对闪LED 的支持，必须在内核配置中定义 `CONFIG_LEDS_CLASS_FLASH` 符号。LED 闪光类驱动必须使led_classdev_flash_register 函数LED 子系统中注册

为控制闪LED 设备，暴露了以下 sysfs 属性：
（见 Documentation/ABI/testing/sysfs-class-led-flash

 - flash_brightness
 - max_flash_brightness
 - flash_timeout
 - max_flash_timeout
 - flash_strobe
 - flash_fault


## 闪光 LED V4L2 封装


LED 子系统驱动也可以VideoForLinux2 子系统的层面进行控制。为了启用此功能，必须在内核配置中定`CONFIG_V4L2_FLASH_LED_CLASS` 符号

驱动必须调用 v4l2_flash_init 函数以在 V4L2 子系统中注册。该函数接受六个参数

- dev:
	闪光设备，例如一I2C 设备
- of_node:
	LED of_node，若与设备的相同则可NULL
- fled_cdev:
	要封装的 LED 闪光类设
- iled_cdev:
	代表fled_cdev 关联的指LED LED 闪光类设备，可为 NULL
- ops:
	V4L2 特定的操

 - external_strobe_set
		定义闪光 LED strobe 的来源—
		V4L2_CID_FLASH_STROBE 控制或外部来源，通常
		一个传感器，这样可以使闪光 strobe 的启动与曝光
		启动保持同步
 - intensity_to_led_brightness 涓?led_brightness_to_intensity
		以设备特定的方式执行
		enum led_brightness <-> V4L2 亮度值的转换——它们可用于
		具有非线LED 电流刻度的设备
- config:
	V4L2 闪光子设备的配置

 - dev_name
		媒体实体的名称，在系统中唯一
 - flash_faults
		LED 闪光类设备可报告的闪光故障的位掩码；
		相应LED_FAULT* 位定义可<linux/led-class-flash.h> 中找到，
 - torch_intensity
		闪光模式LED 的约束，以微安为单位
 - indicator_intensity
		指示 LED 的约束，以微安为单位
 - has_external_strobe
		决定闪光 strobe 来源是否可切换到
		外部

在移除时，必须调v4l2_flash_release 函数，它接受一个参数——即先前v4l2_flash_init 返回struct v4l2_flash 指针。该函数可以安全地以 NULL 或错误指针作为参数调用

有关 v4l2 闪光封装的示例用法，请参drivers/leds/leds-max77693.c

一旦由创建Media controller 设备的驱动注册了 V4L2 子设备，该子设备节点就表现得如同原生 V4L2 闪光 API 设备的节点一样。调用只是被路由LED 闪光 API

打开 V4L2 闪光子设备会LED 子系统的 sysfs 接口不可用。在V4L2 闪光子设备关闭后，接口会重新启用
