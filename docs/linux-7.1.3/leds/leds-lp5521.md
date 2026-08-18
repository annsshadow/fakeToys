## lp5521 鍐呮牳椹卞姩


- National Semiconductor LP5521 LED 椹卞姩鑺墖
- Datasheet: http://www.national.com/pf/LP/LP5521.html

Authors: Mathias Nyman, Yuri Zaporozhets, Samu Onkalo

Contact: Samu Onkalo (samu.p.onkalo-at-nokia.com)

### 鎻忚堪


LP5521 鏈€澶氬彲椹卞姩 3 涓€氶亾銆侺ED 鍙互閫氳繃 LED 绫绘帶鍒舵帴鍙ｇ洿鎺ユ帶鍒躲€傞€氶亾鍏锋湁閫氱敤鍚嶇О锛歭p5521:channelx锛屽叾涓?x 涓?0 .. 2銆?
鎵€鏈変笁涓€氶亾涔熷彲浠ヤ娇鐢ㄥ紩鎿庡井绋嬪簭锛坋ngine micro program锛夋潵鎺у埗銆傛湁鍏虫寚浠ょ殑鏇村缁嗚妭鍙湪鍏紑鐨勬暟鎹墜鍐屼腑鎵惧埌銆?
LP5521 鍏锋湁鍐呴儴绋嬪簭瀛樺偍鍣紝鐢ㄤ簬杩愯鍚勭 LED 妯″紡銆傛湁涓ょ杩愯 LED 妯″紡鐨勬柟寮忋€?
1) sysfs 鎺ュ彛 - enginex_mode 鍜?enginex_load
   寮曟搸鐨勬帶鍒舵帴鍙ｏ細

   x 涓?1 .. 3

   enginex_mode:
	disabled锛堢鐢級銆乴oad锛堝姞杞斤級銆乺un锛堣繍琛岋級
   enginex_load:
	瀛樺偍绋嬪簭锛堜粎鍦?engine 鍔犺浇妯″紡涓嬪彲瑙侊級

```

	cd   /sys/class/leds/lp5521:channel2/device
	echo "load" > engine3_mode
	echo "037f4d0003ff6000" > engine3_load
	echo "run" > engine3_mode

  瑕佸仠姝㈠紩鎿?:

	echo "disabled" > engine3_mode

```

2) 鍥轰欢鎺ュ彛 - LP55xx 閫氱敤鎺ュ彛

鏈夊叧缁嗚妭锛岃鍙傞槄 leds-lp55xx.txt 涓殑 'firmware' 绔犺妭銆?
sysfs 鍖呭惈涓€涓嚜妫€锛坰elftest锛夋潯鐩€?
璇ユ祴璇曚笌鑺墖閫氫俊锛屽苟妫€鏌ユ椂閽熸ā寮忔槸鍚﹀凡鑷姩璁剧疆涓烘墍璇锋眰鐨勬ā寮忋€?
姣忎釜閫氶亾閮芥湁鍚勮嚜鐨?LED 鐢垫祦璁剧疆銆?
- /sys/class/leds/lp5521:channel0/led_current - RW锛堣鍐欙級
- /sys/class/leds/lp5521:channel0/max_current - RO锛堝彧璇伙級

鏍煎紡锛?0x mA锛屽嵆 10 琛ㄧず 1.0 mA

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
	/* 璁剧疆纭欢璧勬簮 */
  }

  static void lp5521_release(void)
  {
	/* 閲婃斁纭欢璧勬簮 */
  }

  static void lp5521_enable(bool state)
  {
	/* 鎺у埗鑺墖浣胯兘淇″彿 */
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

娉ㄦ剰锛?  chan_nr 鍙彇 0 鍒?2 涔嬮棿鐨勫€笺€?  姣忎釜閫氶亾鐨勫悕绉板彲閰嶇疆銆?  濡傛灉鏈畾涔?name 瀛楁锛屽垯榛樿鍚嶇О灏嗚璁句负 'xxxx:channelN'
  锛圶XXX : pdata->label 鎴?i2c 瀹㈡埛绔悕绉帮紝N : 閫氶亾鍙凤級


濡傛灉骞冲彴鏁版嵁涓數娴佽璁句负 0锛屽垯璇ラ€氶亾琚鐢紝骞朵笖涓嶄細鍦?sysfs 涓嚭鐜般€?