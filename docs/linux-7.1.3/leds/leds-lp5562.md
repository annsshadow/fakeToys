## lp5562 内核驱动


- TI LP5562 LED 驱动

作者：Milo(Woogyom) Kim <milo.kim@ti.com>

## 描述


  LP5562 最多可驱动 4 个通道：R/G/B White  LED 可以通过 led class 控制接口直接控制
  所有四个通道也可以使用引擎微程序控制。LP5562 拥有内部程序存储器，可运行各LED 图案  详情请参leds-lp55xx.txt 中的 'firmware' 章节
## 设备属

engine_mux
   LP5562 中分配了 3 个引擎，但通道数为 4   因此每个通道都应映射到引擎编号
   取值：RGB W

   此属性用于通过固件接口LED 数据进行编程。与 LP5521/LP5523/55231 不同，LP5562    引擎 mux 具有独特特性，因此需要额外的 sysfs
   LED 映射

   ===== === ===============================
   Red   ... 引擎 1（固定）
   Green ... 引擎 2（固定）
   Blue  ... 引擎 3（固定）
   White ... 引擎 1 2 3（可选）
   ===== === ===============================

## 如何使用 engine_mux 加载程序数据


  在加LP5562 程序数据之前，应在引擎选择与加载固件之间写engine_mux  引擎 mux 有两种不同模式：RGB W   RGB 用于加载 RGB 程序数据，W 用于加载 W 程序数据
```

    echo 2 > /sys/bus/i2c/devices/xxxx/select_engine     # 2 表示绿色通道
    echo "RGB" > /sys/bus/i2c/devices/xxxx/engine_mux    # 用于 RGB 的引mux
    echo 1 > /sys/class/firmware/lp5562/loading
    echo "4000600040FF6000" > /sys/class/firmware/lp5562/data
    echo 0 > /sys/class/firmware/lp5562/loading
    echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

  要运行闪烁的白色图案::

    echo 1 or 2 or 3 > /sys/bus/i2c/devices/xxxx/select_engine
    echo "W" > /sys/bus/i2c/devices/xxxx/engine_mux
    echo 1 > /sys/class/firmware/lp5562/loading
    echo "4000600040FF6000" > /sys/class/firmware/lp5562/data
    echo 0 > /sys/class/firmware/lp5562/loading
    echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
## 如何加载预定义图

   请参'leds-lp55xx.txt'

## 设置每个通道的电

   LP5521 LP5523/55231 类似，LP5562 提供 LED 电流设置   使用 'led_current' 'max_current'
## 平台数据示例


```

	static struct lp55xx_led_config lp5562_led_config[] = {
		{
			.name 		= "R",
			.chan_nr	= 0,
			.led_current	= 20,
			.max_current	= 40,
		},
		{
			.name 		= "G",
			.chan_nr	= 1,
			.led_current	= 20,
			.max_current	= 40,
		},
		{
			.name 		= "B",
			.chan_nr	= 2,
			.led_current	= 20,
			.max_current	= 40,
		},
		{
			.name 		= "W",
			.chan_nr	= 3,
			.led_current	= 20,
			.max_current	= 40,
		},
	};

	static int lp5562_setup(void)
	{
		/* 配置硬件资源 */
	}

	static void lp5562_release(void)
	{
		/* 释放硬件资源 */
	}

	static void lp5562_enable(bool state)
	{
		/* 控制芯片使能信号 */
	}

	static struct lp55xx_platform_data lp5562_platform_data = {
		.led_config     = lp5562_led_config,
		.num_channels   = ARRAY_SIZE(lp5562_led_config),
		.setup_resources   = lp5562_setup,
		.release_resources = lp5562_release,
		.enable            = lp5562_enable,
	};

```
要配置平台相关数据时，使lp55xx_platform_data 结构


如果在平台数据中电流被设0，则该通道被禁用，并且不会出现sysfs 中