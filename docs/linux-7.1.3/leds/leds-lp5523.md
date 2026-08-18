## 闈㈠悜 lp5523 鐨勫唴鏍搁┍鍔?

- National Semiconductor LP5523 LED 椹卞姩鑺墖
- Datasheet: http://www.national.com/pf/LP/LP5523.html

Authors: Mathias Nyman, Yuri Zaporozhets, Samu Onkalo
Contact: Samu Onkalo (samu.p.onkalo-at-nokia.com)

### 鎻忚堪


LP5523 鍙┍鍔ㄥ杈?9 涓€氶亾銆侺ED 鍙€氳繃 LED 绫绘帶鍒舵帴鍙ｇ洿鎺ユ帶鍒躲€傛瘡涓€氶亾鐨勫悕绉?鍙湪骞冲彴鏁版嵁涓厤缃€斺€攏ame 涓?label銆傛湁涓夌鏂瑰紡鏉ョ敓鎴愰€氶亾鍚嶇О銆?
a) 鍦ㄥ钩鍙版暟鎹腑瀹氫箟 鈥榥ame鈥?

瑕佺敓鎴愮壒瀹氱殑閫氶亾鍚嶇О锛屽彲浣跨敤 鈥榥ame鈥?骞冲彴鏁版嵁銆?
- /sys/class/leds/R1               (name: 'R1')
- /sys/class/leds/B1               (name: 'B1')

b) 浣跨敤 鈥榣abel鈥?涓斾笉甯?鈥榥ame鈥?瀛楁


瀵逛簬涓€涓甫閫氶亾缂栧彿鐨勮澶囧悕锛屽彲浣跨敤 鈥榣abel鈥欍€?- /sys/class/leds/RGB:channelN     (label: 'RGB', N: 0 ~ 8)

c) 榛樿


鑻ヤ袱涓瓧娈靛潎涓?NULL锛屽垯榛樿浣跨敤 鈥榣p5523鈥欍€?- /sys/class/leds/lp5523:channelN  (N: 0 ~ 8)

LP5523 鍏锋湁鐢ㄤ簬杩愯鍚勭 LED 鍥炬鐨勫唴閮ㄧ▼搴忓瓨鍌ㄥ櫒銆傛湁涓ょ鏂瑰紡杩愯 LED 鍥炬銆?
1) sysfs 鎺ュ彛 - enginex_mode銆乪nginex_load 鍜?enginex_leds


  寮曟搸鐨勬帶鍒舵帴鍙ｏ細

  x 涓?1 .. 3

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

  瑕佸仠姝㈠紩鎿庯細

echo "disabled" > engine3_mode

```
2) 鍥轰欢鎺ュ彛 - LP55xx 閫氱敤鎺ュ彛


鏈夊叧缁嗚妭锛岃鍙傝€?leds-lp55xx.txt 涓殑 鈥榝irmware鈥?涓€鑺傘€?
LP5523 鏈変笁涓富璋冨厜鍣紙master fader锛夈€傝嫢涓€涓€氶亾琚槧灏勫埌鍏朵腑涓€涓富璋冨厜鍣紝
鍏惰緭鍑哄皢鍩轰簬涓昏皟鍏夊櫒鐨勫€煎彉鏆椼€?
```
echo "123000123" > master_fader_leds

```
```
  channel 0,6 鏄犲皠鍒?master_fader1
  channel 1,7 鏄犲皠鍒?master_fader2
  channel 2,8 鏄犲皠鍒?master_fader3

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
鑷濮嬬粓浣跨敤骞冲彴鏁版嵁涓殑鐢垫祦銆?
姣忎釜閫氶亾閮藉寘鍚?LED 鐢垫祦璁剧疆銆?- /sys/class/leds/lp5523:channel2/led_current - RW
- /sys/class/leds/lp5523:channel2/max_current - RO

鏍煎紡锛?0x mA锛屽嵆 10 琛ㄧず 1.0 mA

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
	/* 璁剧疆纭欢璧勬簮 */
}

static void lp5523_release(void)
{
	/* 閲婃斁纭欢璧勬簮 */
}

static void lp5523_enable(bool state)
{
	/* 鎺у埗鑺墖浣胯兘淇″彿 */
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
娉ㄦ剰
  chan_nr 鐨勫彇鍊煎彲鍦?0 鍒?8 涔嬮棿銆?