## lp5562 鍐呮牳椹卞姩


- TI LP5562 LED 椹卞姩

浣滆€咃細Milo(Woogyom) Kim <milo.kim@ti.com>

## 鎻忚堪


  LP5562 鏈€澶氬彲椹卞姩 4 涓€氶亾锛歊/G/B 涓?White銆?  LED 鍙互閫氳繃 led class 鎺у埗鎺ュ彛鐩存帴鎺у埗銆?
  鎵€鏈夊洓涓€氶亾涔熷彲浠ヤ娇鐢ㄥ紩鎿庡井绋嬪簭鎺у埗銆侺P5562 鎷ユ湁鍐呴儴绋嬪簭瀛樺偍鍣紝鍙繍琛屽悇绉?LED 鍥炬銆?  璇︽儏璇峰弬鑰?leds-lp55xx.txt 涓殑 'firmware' 绔犺妭銆?
## 璁惧灞炴€?

engine_mux
   LP5562 涓垎閰嶄簡 3 涓紩鎿庯紝浣嗛€氶亾鏁颁负 4銆?   鍥犳姣忎釜閫氶亾閮藉簲鏄犲皠鍒板紩鎿庣紪鍙枫€?
   鍙栧€硷細RGB 鎴?W

   姝ゅ睘鎬х敤浜庨€氳繃鍥轰欢鎺ュ彛瀵?LED 鏁版嵁杩涜缂栫▼銆備笌 LP5521/LP5523/55231 涓嶅悓锛孡P5562 瀵?   寮曟搸 mux 鍏锋湁鐙壒鐗规€э紝鍥犳闇€瑕侀澶栫殑 sysfs銆?
   LED 鏄犲皠

   ===== === ===============================
   Red   ... 寮曟搸 1锛堝浐瀹氾級
   Green ... 寮曟搸 2锛堝浐瀹氾級
   Blue  ... 寮曟搸 3锛堝浐瀹氾級
   White ... 寮曟搸 1 鎴?2 鎴?3锛堝彲閫夛級
   ===== === ===============================

## 濡備綍浣跨敤 engine_mux 鍔犺浇绋嬪簭鏁版嵁


  鍦ㄥ姞杞?LP5562 绋嬪簭鏁版嵁涔嬪墠锛屽簲鍦ㄥ紩鎿庨€夋嫨涓庡姞杞藉浐浠朵箣闂村啓鍏?engine_mux銆?  寮曟搸 mux 鏈変袱绉嶄笉鍚屾ā寮忥細RGB 涓?W銆?   RGB 鐢ㄤ簬鍔犺浇 RGB 绋嬪簭鏁版嵁锛學 鐢ㄤ簬鍔犺浇 W 绋嬪簭鏁版嵁銆?
```

    echo 2 > /sys/bus/i2c/devices/xxxx/select_engine     # 2 琛ㄧず缁胯壊閫氶亾
    echo "RGB" > /sys/bus/i2c/devices/xxxx/engine_mux    # 鐢ㄤ簬 RGB 鐨勫紩鎿?mux
    echo 1 > /sys/class/firmware/lp5562/loading
    echo "4000600040FF6000" > /sys/class/firmware/lp5562/data
    echo 0 > /sys/class/firmware/lp5562/loading
    echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

  瑕佽繍琛岄棯鐑佺殑鐧借壊鍥炬::

    echo 1 or 2 or 3 > /sys/bus/i2c/devices/xxxx/select_engine
    echo "W" > /sys/bus/i2c/devices/xxxx/engine_mux
    echo 1 > /sys/class/firmware/lp5562/loading
    echo "4000600040FF6000" > /sys/class/firmware/lp5562/data
    echo 0 > /sys/class/firmware/lp5562/loading
    echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
## 濡備綍鍔犺浇棰勫畾涔夊浘妗?

   璇峰弬鑰?'leds-lp55xx.txt'

## 璁剧疆姣忎釜閫氶亾鐨勭數娴?

   涓?LP5521 鍜?LP5523/55231 绫讳技锛孡P5562 鎻愪緵 LED 鐢垫祦璁剧疆銆?   浣跨敤 'led_current' 涓?'max_current'銆?
## 骞冲彴鏁版嵁绀轰緥


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
		/* 閰嶇疆纭欢璧勬簮 */
	}

	static void lp5562_release(void)
	{
		/* 閲婃斁纭欢璧勬簮 */
	}

	static void lp5562_enable(bool state)
	{
		/* 鎺у埗鑺墖浣胯兘淇″彿 */
	}

	static struct lp55xx_platform_data lp5562_platform_data = {
		.led_config     = lp5562_led_config,
		.num_channels   = ARRAY_SIZE(lp5562_led_config),
		.setup_resources   = lp5562_setup,
		.release_resources = lp5562_release,
		.enable            = lp5562_enable,
	};

```
瑕侀厤缃钩鍙扮浉鍏虫暟鎹椂锛屼娇鐢?lp55xx_platform_data 缁撴瀯


濡傛灉鍦ㄥ钩鍙版暟鎹腑鐢垫祦琚涓?0锛屽垯璇ラ€氶亾琚鐢紝骞朵笖涓嶄細鍑虹幇鍦?sysfs 涓€?