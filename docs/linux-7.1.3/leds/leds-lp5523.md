## 面向 lp5523 的内核驱动


- National Semiconductor LP5523 LED 驱动芯片
- Datasheet: http://www.national.com/pf/LP/LP5523.html

Authors: Mathias Nyman, Yuri Zaporozhets, Samu Onkalo
Contact: Samu Onkalo (samu.p.onkalo-at-nokia.com)

### 描述


LP5523 可驱动多达 9 个通道。LED 可通过 LED 类控制接口直接控制。每个通道的名称
可在平台数据中配置——name 与 label。有三种方式来生成通道名称。

a) 在平台数据中定义 ‘name’


要生成特定的通道名称，可使用 ‘name’ 平台数据。

- /sys/class/leds/R1               (name: 'R1')
- /sys/class/leds/B1               (name: 'B1')

b) 使用 ‘label’ 且不带 ‘name’ 字段


对于一个带通道编号的设备名，可使用 ‘label’。
- /sys/class/leds/RGB:channelN     (label: 'RGB', N: 0 ~ 8)

c) 默认


若两个字段均为 NULL，则默认使用 ‘lp5523’。
- /sys/class/leds/lp5523:channelN  (N: 0 ~ 8)

LP5523 具有用于运行各种 LED 图案的内部程序存储器。有两种方式运行 LED 图案。

1) sysfs 接口 - enginex_mode、enginex_load 和 enginex_leds


  引擎的控制接口：

  x 为 1 .. 3

  enginex_mode:
	disabled, load, run
  enginex_load:
	microcode load
  enginex_leds:
	led mux control

```
cd /sys/class/leds/lp5523:channel2/device
echo "load" > engine3_mode
echo "9d80400004ff05ff437f0000" > engine3_load
echo "111111111" > engine3_leds
echo "run" > engine3_mode

  要停止引擎：

echo "disabled" > engine3_mode

```
2) 固件接口 - LP55xx 通用接口


有关细节，请参考 leds-lp55xx.txt 中的 ‘firmware’ 一节。

LP5523 有三个主调光器（master fader）。若一个通道被映射到其中一个主调光器，
其输出将基于主调光器的值变暗。

```
echo "123000123" > master_fader_leds

```
```
  channel 0,6 映射到 master_fader1
  channel 1,7 映射到 master_fader2
  channel 2,8 映射到 master_fader3

```
```
echo 64 > master_fader1

```
```
echo 0 > master_fader2

```
```
echo 255 > master_fader3

```
```
echo "000000000" > master_fader_leds

```
自检始终使用平台数据中的电流。

每个通道都包含 LED 电流设置。
- /sys/class/leds/lp5523:channel2/led_current - RW
- /sys/class/leds/lp5523:channel2/max_current - RO

格式：10x mA，即 10 表示 1.0 mA

```
static struct lp55xx_led_config lp5523_led_config[] = {
	{
		.name		= "D1",
		.chan_nr        = 0,
		.led_current    = 50,
		.max_current    = 130,
	},
	...
	{
		.chan_nr        = 8,
		.led_current    = 50,
		.max_current    = 130,
	}
};

static int lp5523_setup(void)
{
	/* 设置硬件资源 */
}

static void lp5523_release(void)
{
	/* 释放硬件资源 */
}

static void lp5523_enable(bool state)
{
	/* 控制芯片使能信号 */
}

static struct lp55xx_platform_data lp5523_platform_data = {
	.led_config     = lp5523_led_config,
	.num_channels   = ARRAY_SIZE(lp5523_led_config),
	.clock_mode     = LP55XX_CLOCK_EXT,
	.setup_resources   = lp5523_setup,
	.release_resources = lp5523_release,
	.enable            = lp5523_enable,
};

```
注意
  chan_nr 的取值可在 0 到 8 之间。
