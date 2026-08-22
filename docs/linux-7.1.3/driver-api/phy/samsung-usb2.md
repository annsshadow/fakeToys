## Samsung USB 2.0 PHY 閫傞厤灞。

### 1. 描述


在许Samsung SoC 中，USB 2.0 PHY 模块的架构是相似的。尽管有这些相似之处，但创建一个能适配所有这PHY 控制器的单一驱动被证明是困难的。差异往往很小，存在于 PHY 寄存器的特定位中。在少数罕见情况下，必须
改变寄存器写入的顺序PHY 上电过程。此适配层是在拥有独立驱动和拥有为许多特殊情况增加支持的单一驱动之间一种折衷
### 2. 文件描述


- phy-samsung-usb2.c
   这是适配层的主文件。该文件包含 probe 函数，并向通用 PHY 框架提供两个回调。这两个回调用于phy 上电
   和下电。它们执行所有版PHY 模块都必须完成的公共工作。根据所选择SoC，它们执SoC 特定的回调   特定SoC 版本通过选择适当compatible 字符串来确定。此外，该文件包含针对特SoC    struct of_device_id 定义
- phy-samsung-usb2.h
   这是头文件。它声明此驱动使用的结构体。此外，它应包含描述特定 SoC 的结构体extern 声明
### 3. 支持SoC


要支持一个新SoC，应drivers/phy 目录添加一个新文件。每SoC 的配置存储在一```

  struct samsung_usb2_phy_config {
	const struct samsung_usb2_common_phy *phys;
	int (*rate_to_clk)(unsigned long, u32 *);
	unsigned int num_phys;
	bool has_mode_switch;
  };

```
num_phys 是驱动处理的 phy 数量。`*phys` 是一个数组，包含每个 phy 的配置。has_mode_switch 属性是一布尔标志，决SoC 是否在一对引脚上同时具有 USB 主机和设备。如果是，则必须修改一个特殊寄存器来在这些引脚内部路由之间切换，以连到 USB 设备或主机模块
```

  const struct samsung_usb2_phy_config exynos4210_usb2_phy_config = {
	.has_mode_switch        = 0,
	.num_phys		= EXYNOS4210_NUM_PHYS,
	.phys			= exynos4210_phys,
	.rate_to_clk		= exynos4210_rate_to_clk,
  }

```
- `int (**rate_to_clk)(unsigned long, u32 **)`

	rate_to_clk 回调用于将用PHY 模块参考时钟的时钟速率转换为应写入硬件寄存器的值
```

  static const struct samsung_usb2_common_phy exynos4210_phys[] = {
	{
		.label		= "device",
		.id		= EXYNOS4210_DEVICE,
		.power_on	= exynos4210_power_on,
		.power_off	= exynos4210_power_off,
	},
	{
		.label		= "host",
		.id		= EXYNOS4210_HOST,
		.power_on	= exynos4210_power_on,
		.power_off	= exynos4210_power_off,
	},
	{
		.label		= "hsic0",
		.id		= EXYNOS4210_HSIC0,
		.power_on	= exynos4210_power_on,
		.power_off	= exynos4210_power_off,
	},
	{
		.label		= "hsic1",
		.id		= EXYNOS4210_HSIC1,
		.power_on	= exynos4210_power_on,
		.power_off	= exynos4210_power_off,
	},
	{},
  };

```
- `int (**power_on)(struct samsung_usb2_phy_instance **);`
  `int (**power_off)(struct samsung_usb2_phy_instance **);`

	这两个回调用于通过修改适当的寄存器来给 phy 上电和下电
对驱动的最后改动是phy-samsung-usb2.c 文件添加适当compatible 值。对Exynos 4210，以下行
```

  #ifdef CONFIG_PHY_EXYNOS4210_USB2
	{
		.compatible = "samsung,exynos4210-usb2-phy",
		.data = &exynos4210_usb2_phy_config,
	},
  #endif

```
为了给驱动增加进一步的灵活性，Kconfig 文件使能在编译的驱动中包含对所SoC 的支持。Kconfig
```

  config PHY_EXYNOS4210_USB2
	bool "Support for Exynos 4210"
	depends on PHY_SAMSUNG_USB2
	depends on CPU_EXYNOS4210
	help
	  Enable USB PHY support for Exynos 4210. This option requires that
	  Samsung USB 2.0 PHY driver is enabled and means that support for this
	  particular SoC is compiled in the driver. In case of Exynos 4210 four
	  phys are available - device, host, HSCI0 and HSCI1.

```
新创建的支持SoC 的文件也必须添加```

  obj-$(CONFIG_PHY_EXYNOS4210_USB2)       += phy-exynos4210-usb2.o

```
完成这些步骤后，对新 SoC 的支持就应该就绪了