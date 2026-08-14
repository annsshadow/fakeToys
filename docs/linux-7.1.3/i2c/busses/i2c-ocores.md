## 鍐呮牳椹卞姩 i2c-ocores


鏀寔鐨勯€傞厤鍣細
  - OpenCores.org 鐢?Richard Herveille 璁捐鐨?I2C 鎺у埗鍣紙鍙傝鏁版嵁鎵嬪唽閾炬帴锛?    https://opencores.org/project/i2c/overview

浣滆€咃細Peter Korsgaard <peter@korsgaard.com>

### 鎻忚堪


i2c-ocores 鏄拡瀵?Richard Herveille 璁捐鐨?OpenCores.org I2C 鎺у埗鍣?IP 鏍哥殑 i2c 鎬荤嚎椹卞姩銆?
### 鐢ㄦ硶


i2c-ocores 浣跨敤 platform 鎬荤嚎锛屽洜姝や綘闇€瑕佹彁渚涗竴涓甫鏈夊熀鍦板潃鍜屼腑鏂彿鐨?struct platform_device銆傝澶囩殑 dev.platform_data 杩樺簲鎸囧悜涓€涓?struct ocores_i2c_platform_data锛堝弬瑙?linux/platform_data/i2c-ocores.h锛夛紝鐢ㄤ簬鎻忚堪瀵勫瓨鍣ㄤ箣闂寸殑闂撮殧浠ュ強杈撳叆鏃堕挓閫熷害銆?涔熷彲浠ラ檮鍔犱竴涓?i2c_board_info 鍒楄〃锛宨2c-ocores 椹卞姩鍦ㄥ垱寤烘椂灏嗗叾娣诲姞鍒版€荤嚎涓娿€?
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
