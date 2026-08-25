## lp5521 内核驱动


- National Semiconductor LP5521 LED 驱动芯片
- Datasheet: http://www.national.com/pf/LP/LP5521.html

Authors: Mathias Nyman, Yuri Zaporozhets, Samu Onkalo

Contact: Samu Onkalo (samu.p.onkalo-at-nokia.com)

### 描述


LP5521 最多可驱动 3 个通道。LED 可以通过 LED 类控制接口直接控制。通道具有通用名称：lp5521:channelx，其x 0 .. 2
所有三个通道也可以使用引擎微程序（engine micro program）来控制。有关指令的更多细节可在公开的数据手册中找到
LP5521 具有内部程序存储器，用于运行各种 LED 模式。有两种运行 LED 模式的方式
1) sysfs 接口 - enginex_mode enginex_load
   引擎的控制接口：

   x 涓?1 .. 3

   enginex_mode:
	disabled（禁用）、load（加载）、run（运行）
   enginex_load:
	存储程序（仅engine 加载模式下可见）

```

	cd   /sys/class/leds/lp5521:channel2/device
	echo "load" > engine3_mode
	echo "037f4d0003ff6000" > engine3_load
	echo "run" > engine3_mode

  瑕佸仠姝㈠紩鎿?:

	echo "disabled" > engine3_mode

```

2) 固件接口 - LP55xx 通用接口

有关细节，请参阅 leds-lp55xx.txt 中的 'firmware' 章节
sysfs 包含一个自检（selftest）条目
该测试与芯片通信，并检查时钟模式是否已自动设置为所请求的模式
每个通道都有各自LED 电流设置
- /sys/class/leds/lp5521:channel0/led_current - RW（读写）
- /sys/class/leds/lp5521:channel0/max_current - RO（只读）

格式0x mA，即 10 表示 1.0 mA

```

  static struct lp55xx_led_config lp5521_led_config[] = {
	  {
		.name = "red",
		  .chan_nr        = 0,
		  .led_current    = 50,
		.max_current    = 130,
	  }, {
		.name = "green",
		  .chan_nr        = 1,
		  .led_current    = 0,
		.max_current    = 130,
	  }, {
		.name = "blue",
		  .chan_nr        = 2,
		  .led_current    = 0,
		.max_current    = 130,
	  }
  };

  static int lp5521_setup(void)
  {
	/* 设置硬件资源 */
  }

  static void lp5521_release(void)
  {
	/* 释放硬件资源 */
  }

  static void lp5521_enable(bool state)
  {
	/* 控制芯片使能信号 */
  }

  static struct lp55xx_platform_data lp5521_platform_data = {
	  .led_config     = lp5521_led_config,
	  .num_channels   = ARRAY_SIZE(lp5521_led_config),
	  .clock_mode     = LP55XX_CLOCK_EXT,
	  .setup_resources   = lp5521_setup,
	  .release_resources = lp5521_release,
	  .enable            = lp5521_enable,
  };

```

注意  chan_nr 可取 0 2 之间的值  每个通道的名称可配置  如果未定name 字段，则默认名称将被设为 'xxxx:channelN'
  （XXXX : pdata->label i2c 客户端名称，N : 通道号）


如果平台数据中电流被设为 0，则该通道被禁用，并且不会sysfs 中出现