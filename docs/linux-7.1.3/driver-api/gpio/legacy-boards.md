## Supporting Legacy Boards


内核中的许多驱动，例`leds-gpio` `gpio-keys`，正逐渐从使用板特定`platform_data` 迁移
统一的设备属性（device properties）接口。该接口让驱动更简单、更通用，因为它们可以以标准化的
方式查询属性

在现代系统上，这些属性通过设备树提供。然而，一些较旧的平台尚未转换为设备树，而是依赖板文
来描述其硬件配置。为了弥合这一差距，并让这些传统板能够配合现代的通用驱动工作，内核提供了一
称为**软件节点**（software node）的机制

本文档提供了如何将传统板文件从使`platform_data` `gpiod_lookup_table` 转换为现代的软件
节点方法来描GPIO 连接设备的指南

### The Core Idea: Software Nodes


软件节点允许板特定代码使struct software_node struct property_entry 构建内存中的、类
设备树的结构。该结构随后可以与平台设备关联，使驱动能够使用标准的设备属API（例
device_property_read_u32()、device_property_read_string()）查询配置，就像ACPI 或设备树系统
一样

gpiolib 代码支持处理软件节点，因此如GPIO 被正确描述（如下节详述），那么常规的 gpiolib API
gpiod_get()、gpiod_get_optional() 等，都能正常工作

#### Requirements for GPIO Properties


使用软件节点描述 GPIO 连接时，必须满足以下要求，GPIO 核心才能正确解析引用

1. **GPIO 控制器的软件节点必须已注册，并作为主固件节点或次固件节点挂载到控制器`struct
    device` 上* gpiolib 核心使用固件节点的地址在运行时查找对应`struct gpio_chip`

2. **GPIO 属性必须是一个引用* `PROPERTY_ENTRY_GPIO()` 宏处理了这一点，因为它是
   `PROPERTY_ENTRY_REF()` 的别名

3. **该引用必须恰好有两个参数*

    - 第一个参数是控制器内GPIO 偏移量
    - 第二个参数是GPIO 线的标志（例GPIO_ACTIVE_HIGH、GPIO_ACTIVE_LOW）

`PROPERTY_ENTRY_GPIO()` 宏是在软件节点中定义 GPIO 属性的首选方式

### Conversion Example


让我们通过一个将定义 GPIO 连接LED 和按钮的板文件进行转换的示例来逐步说明

#### Before: Using Platform Data


一个典型的传统板文件可能如下所示：


  #include <linux/platform_device.h>
  #include <linux/leds.h>
  #include <linux/gpio_keys.h>
  #include <linux/gpio/machine.h>

  #define MYBOARD_GPIO_CONTROLLER "gpio-foo"

  /** LED 设置 **/
  static const struct gpio_led myboard_leds[] = {
  	{
  		.name = "myboard:green:status",
  		.default_trigger = "heartbeat",
  	},
  };

  static const struct gpio_led_platform_data myboard_leds_pdata = {
  	.num_leds = ARRAY_SIZE(myboard_leds),
  	.leds = myboard_leds,
  };

  static struct gpiod_lookup_table myboard_leds_gpios = {
  	.dev_id = "leds-gpio",
  	.table = {
  		GPIO_LOOKUP_IDX(MYBOARD_GPIO_CONTROLLER, 42, NULL, 0, GPIO_ACTIVE_HIGH),
  		{ },
  	},
  };

  /** 按钮设置 **/
  static struct gpio_keys_button myboard_buttons[] = {
  	{
  		.code = KEY_WPS_BUTTON,
  		.desc = "WPS Button",
  		.active_low = 1,
  	},
  };

  static const struct gpio_keys_platform_data myboard_buttons_pdata = {
  	.buttons = myboard_buttons,
  	.nbuttons = ARRAY_SIZE(myboard_buttons),
  };

  static struct gpiod_lookup_table myboard_buttons_gpios = {
  	.dev_id = "gpio-keys",
  	.table = {
  		GPIO_LOOKUP_IDX(MYBOARD_GPIO_CONTROLLER, 15, NULL, 0, GPIO_ACTIVE_LOW),
  		{ },
  	},
  };

  /** 设备注册 **/
  static int __init myboard_init(void)
  {
  	struct platform_device_info pdev_info = {
  		.name = MYBOARD_GPIO_CONTROLLER,
  		.id = PLATFORM_DEVID_NONE,
  		.swnode = &gpio_controller_node
  	};

  	gpiod_add_lookup_table(&myboard_leds_gpios);
  	gpiod_add_lookup_table(&myboard_buttons_gpios);

  	platform_device_register_full(&pdev_info);
  	platform_device_register_data(NULL, "leds-gpio", -1,
  				      &myboard_leds_pdata, sizeof(myboard_leds_pdata));
  	platform_device_register_data(NULL, "gpio-keys", -1,
  				      &myboard_buttons_pdata,
  				      sizeof(myboard_buttons_pdata));

  	return 0;
  }

#### After: Using Software Nodes


以下是如何使用软件节点表达相同的配置

######## 步骤 1：定GPIO 控制器节


首先，定义一个代LED 和按钮所连接 GPIO 控制器的软件节点。该节点`name` 是可选的


  #include <linux/property.h>
  #include <linux/gpio/property.h>

  #define MYBOARD_GPIO_CONTROLLER "gpio-foo"

  static const struct software_node myboard_gpio_controller_node = {
  	.name = MYBOARD_GPIO_CONTROLLER,
  };

######## 步骤 2：定义消费设备节点与属


接下来，定义消费设备（LED 和按钮）的软件节点。这涉及为每个设备类型创建一个父节点，并为每
单独LED 或按钮创建子节点


  /** LED 设置 **/
  static const struct software_node myboard_leds_node = {
  	.name = "myboard-leds",
  };

  static const struct property_entry myboard_status_led_props[] = {
  	PROPERTY_ENTRY_STRING("label", "myboard:green:status"),
  	PROPERTY_ENTRY_STRING("linux,default-trigger", "heartbeat"),
  	PROPERTY_ENTRY_GPIO("gpios", &myboard_gpio_controller_node, 42, GPIO_ACTIVE_HIGH),
  	{ }
  };

  static const struct software_node myboard_status_led_swnode = {
  	.name = "status-led",
  	.parent = &myboard_leds_node,
  	.properties = myboard_status_led_props,
  };

  /** 按钮设置 **/
  static const struct software_node myboard_keys_node = {
  	.name = "myboard-keys",
  };

  static const struct property_entry myboard_wps_button_props[] = {
  	PROPERTY_ENTRY_STRING("label", "WPS Button"),
  	PROPERTY_ENTRY_U32("linux,code", KEY_WPS_BUTTON),
  	PROPERTY_ENTRY_GPIO("gpios", &myboard_gpio_controller_node, 15, GPIO_ACTIVE_LOW),
  	{ }
  };

  static const struct software_node myboard_wps_button_swnode = {
  	.name = "wps-button",
  	.parent = &myboard_keys_node,
  	.properties = myboard_wps_button_props,
  };



######## 步骤 3：分组并注册节点


为了可维护性，通常将所有软件节点分组到一个数组中并用一次调用注册它们是有益的


  static const struct software_node * const myboard_swnodes[] = {
  	&myboard_gpio_controller_node,
  	&myboard_leds_node,
  	&myboard_status_led_swnode,
  	&myboard_keys_node,
  	&myboard_wps_button_swnode,
  	NULL
  };

  static int __init myboard_init(void)
  {
  	int error;

  	error = software_node_register_node_group(myboard_swnodes);
  	if (error) {
  		pr_err("Failed to register software nodes: %d\n", error);
  		return error;
  	}

  	// ... 随后是平台设备注
  }

  当按所代表的设备拆分节点注册时，必须先注册代表 GPIO 控制器本身的软件节点，然后才能注册任
  引用它的节点

######## 步骤 4：使用软件节点注册平台设


最后，注册平台设备，并使用 struct platform_device_info 中的 `fwnode` 字段将它们与各自的软
节点关联


  static struct platform_device *leds_pdev;
  static struct platform_device *keys_pdev;

  static int __init myboard_init(void)
  {
  	struct platform_device_info pdev_info;
  	int error;

  	error = software_node_register_node_group(myboard_swnodes);
  	if (error)
  		return error;

  	memset(&pdev_info, 0, sizeof(pdev_info));
  	pdev_info.name = MYBOARD_GPIO_CONTROLLER;
  	pdev_info.id = PLATFORM_DEVID_NONE;
  	pdev_info.swnode = &myboard_gpio_controller_node;
  	gpio_pdev = platform_device_register_full(&pdev_info);
  	if (IS_ERR(gpio_pdev)) {
  		error = PTR_ERR(gpio_pdev);
  		goto err_unregister_nodes;
  	}

  	memset(&pdev_info, 0, sizeof(pdev_info));
  	pdev_info.name = "leds-gpio";
  	pdev_info.id = PLATFORM_DEVID_NONE;
  	pdev_info.fwnode = software_node_fwnode(&myboard_leds_node);
  	leds_pdev = platform_device_register_full(&pdev_info);
  	if (IS_ERR(leds_pdev)) {
  		error = PTR_ERR(leds_pdev);
  		platform_device_unregister(gpio_pdev);
  		goto err_unregister_nodes;
  	}

  	memset(&pdev_info, 0, sizeof(pdev_info));
  	pdev_info.name = "gpio-keys";
  	pdev_info.id = PLATFORM_DEVID_NONE;
  	pdev_info.fwnode = software_node_fwnode(&myboard_keys_node);
  	keys_pdev = platform_device_register_full(&pdev_info);
  	if (IS_ERR(keys_pdev)) {
  		error = PTR_ERR(keys_pdev);
  		platform_device_unregister(gpio_pdev);
  		platform_device_unregister(leds_pdev);
  		goto err_unregister_nodes;
  	}

  	return 0;

  err_unregister_nodes:
  	software_node_unregister_node_group(myboard_swnodes);
  	return error;
  }

  static void __exit myboard_exit(void)
  {
  	platform_device_unregister(keys_pdev);
  	platform_device_unregister(leds_pdev);
  	platform_device_unregister(gpio_pdev);
  	software_node_unregister_node_group(myboard_swnodes);
  }

通过这些更改，通用`leds-gpio` `gpio-keys` 驱动将能够成功探测，并从软件节点中定义的属
获取其配置，从而不再需要板特定platform data
