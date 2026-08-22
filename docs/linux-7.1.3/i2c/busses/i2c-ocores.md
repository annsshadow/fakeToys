## 内核驱动 i2c-ocores


支持的适配器：
  - OpenCores.org Richard Herveille 设计I2C 控制器（参见数据手册链接    https://opencores.org/project/i2c/overview

作者：Peter Korsgaard <peter@korsgaard.com>

### 描述


i2c-ocores 是针Richard Herveille 设计OpenCores.org I2C 控制IP 核的 i2c 总线驱动
### 用法


i2c-ocores 使用 platform 总线，因此你需要提供一个带有基地址和中断号struct platform_device。设备的 dev.platform_data 还应指向一struct ocores_i2c_platform_data（参linux/platform_data/i2c-ocores.h），用于描述寄存器之间的间隔以及输入时钟速度也可以附加一i2c_board_info 列表，i2c-ocores 驱动在创建时将其添加到总线上
```

  static struct resource ocores_resources[] = {
	[0] = {
		.start	= MYI2C_BASEADDR,
		.end	= MYI2C_BASEADDR + 8,
		.flags	= IORESOURCE_MEM,
	},
	[1] = {
		.start	= MYI2C_IRQ,
		.end	= MYI2C_IRQ,
		.flags	= IORESOURCE_IRQ,
	},
  };

  /* optional board info */
  struct i2c_board_info ocores_i2c_board_info[] = {
	{
		I2C_BOARD_INFO("tsc2003", 0x48),
		.platform_data = &tsc2003_platform_data,
		.irq = TSC_IRQ
	},
	{
		I2C_BOARD_INFO("adv7180", 0x42 >> 1),
		.irq = ADV_IRQ
	}
  };

  static struct ocores_i2c_platform_data myi2c_data = {
	.regstep	= 2,		/* two bytes between registers */
	.clock_khz	= 50000,	/* input clock of 50MHz */
	.devices	= ocores_i2c_board_info, /* optional table of devices */
	.num_devices	= ARRAY_SIZE(ocores_i2c_board_info), /* table size */
  };

  static struct platform_device myi2c = {
	.name			= "ocores-i2c",
	.dev = {
		.platform_data	= &myi2c_data,
	},
	.num_resources		= ARRAY_SIZE(ocores_resources),
	.resource		= ocores_resources,
  };

```
