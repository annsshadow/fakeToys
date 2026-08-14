## LP5521/LP5523/LP55231/LP5562/LP8501 鍏叡椹卞姩


浣滆€咃細Milo(Woogyom) Kim <milo.kim@ti.com>

### 鎻忚堪


LP5521銆丩P5523/55231銆丩P5562 鍜?LP8501 鍏锋湁浠ヤ笅鍏卞悓鐗规€э細

  閫氳繃 I2C 璁块棶瀵勫瓨鍣?  璁惧鐨勫垵濮嬪寲/鍙嶅垵濮嬪寲
  涓哄涓緭鍑洪€氶亾鍒涘缓 LED 绫昏澶?  鐢ㄤ簬鐢ㄦ埛绌洪棿鎺ュ彛鐨勮澶囧睘鎬?  鐢ㄤ簬杩愯 LED 妯″紡鐨勭▼搴忓瓨鍌ㄥ櫒

LP55xx 鍏叡椹卞姩浣跨敤瀵煎嚭鐨勫嚱鏁版彁渚涜繖浜涚壒鎬с€?
  lp55xx_init_device() / lp55xx_deinit_device()
  lp55xx_register_leds() / lp55xx_unregister_leds()
  lp55xx_regsister_sysfs() / lp55xx_unregister_sysfs()

锛堥┍鍔ㄧ粨鏋勬暟鎹級

鍦?lp55xx 鍏叡椹卞姩涓紝浣跨敤浜嗕袱绉嶄笉鍚岀殑鏁版嵁缁撴瀯銆?
- lp55xx_led
    鎺у埗澶氳緭鍑?LED 閫氶亾锛屼緥濡?LED 鐢垫祦銆侀€氶亾绱㈠紩銆?- lp55xx_chip
    閫氱敤鑺墖鎺у埗锛屼緥濡?I2C 鍜屽钩鍙版暟鎹€?
渚嬪锛孡P5521 鏈€澶氭湁 3 涓?LED 閫氶亾銆?```

  lp55xx_chip for LP5521 ... lp55xx_led #1
			     lp55xx_led #2
			     lp55xx_led #3

  lp55xx_chip for LP5523 ... lp55xx_led #1
			     lp55xx_led #2
				   .
				   .
			     lp55xx_led #9

```
锛堜緷璧栬姱鐗囩殑浠ｇ爜锛?
涓轰簡鏀寔璁惧鐗瑰畾鐨勯厤缃紝浣跨敤浜嗕竴涓壒娈婄粨鏋?鈥榣pxx_device_config鈥欍€?
  - 鏈€澶ч€氶亾鏁?  - 澶嶄綅鍛戒护銆佽姱鐗囦娇鑳藉懡浠?  - 鑺墖鐗瑰畾鍒濆鍖?  - 浜害鎺у埗瀵勫瓨鍣ㄨ闂?  - 璁剧疆 LED 杈撳嚭鐢垫祦
  - 鐢ㄤ簬杩愯妯″紡鐨勭▼搴忓瓨鍌ㄥ櫒鍦板潃璁块棶
  - 闄勫姞鐨勮澶囩壒瀹氬睘鎬?
锛堝浐浠舵帴鍙ｏ級

LP55xx 绯诲垪璁惧鎷ユ湁鐢ㄤ簬杩愯鍚勭 LED 妯″紡鐨勫唴閮ㄧ▼搴忓瓨鍌ㄥ櫒銆?
姝ゆā寮忔暟鎹綔涓烘枃浠朵繚瀛樺湪鐢ㄦ埛绌洪棿锛屾垨鑰呴€氳繃 I2C 灏嗗崄鍏繘鍒跺瓧鑺備覆鍐欏叆瀛樺偍鍣ㄣ€?
LP55xx 鍏叡椹卞姩鏀寔鍥轰欢鎺ュ彛銆?
LP55xx 鑺墖鏈変笁涓▼搴忓紩鎿庛€?
瑕佸姞杞藉苟杩愯妯″紡锛岀紪绋嬮『搴忓涓嬨€?
  (1) 閫夋嫨涓€涓紩鎿庡彿锛?/2/3锛?  (2) 妯″紡鍒囨崲涓?load锛堝姞杞斤級
  (3) 灏嗘ā寮忔暟鎹啓鍏ユ墍閫夊尯鍩?  (4) 妯″紡鍒囨崲涓?run锛堣繍琛岋級

LP55xx 鍏叡椹卞姩鎻愪緵濡備笅绠€鍗曟帴鍙ｃ€?
select_engine:
	閫夋嫨鐢ㄤ簬杩愯绋嬪簭鐨勫紩鎿?run_engine:
	鍚姩閫氳繃鍥轰欢鎺ュ彛鍔犺浇鐨勭▼搴?firmware:
	鍔犺浇绋嬪簭鏁版嵁

鍦?LP5523 鐨勬儏鍐典笅锛岃繕闇€瑕佷竴涓懡浠?鈥榚nginex_leds鈥欍€傚畠鐢ㄤ簬鍦ㄦ瘡涓紩鎿庡彿涓婇€夋嫨 LED
杈撳嚭銆傛洿澶氱粏鑺傝鍙傞槄 鈥榣eds-lp5523.txt鈥欍€?
```

	echo 1 > /sys/bus/i2c/devices/xxxx/select_engine
	echo 1 > /sys/class/firmware/lp5521/loading
	echo "4000600040FF6000" > /sys/class/firmware/lp5521/data
	echo 0 > /sys/class/firmware/lp5521/loading
	echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
渚嬪锛屽湪 LP55231 鐨勫紩鎿?#3 涓繍琛岄棯鐑佹ā寮?
```

	echo 3 > /sys/bus/i2c/devices/xxxx/select_engine
	echo 1 > /sys/class/firmware/lp55231/loading
	echo "9d0740ff7e0040007e00a0010000" > /sys/class/firmware/lp55231/data
	echo 0 > /sys/class/firmware/lp55231/loading
	echo "000001100" > /sys/bus/i2c/devices/xxxx/engine3_leds
	echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
```

	for idx in 2 3
	do
	echo $idx > /sys/class/leds/red/device/select_engine
	sleep 0.1
	echo 1 > /sys/class/firmware/lp5521/loading
	echo "4000600040FF6000" > /sys/class/firmware/lp5521/data
	echo 0 > /sys/class/firmware/lp5521/loading
	done
	echo 1 > /sys/class/leds/red/device/run_engine

```
杩欐槸 LP5523 鐨勫彟涓€涓ず渚嬨€?
```

	echo 2 > /sys/bus/i2c/devices/xxxx/select_engine
	echo 1 > /sys/class/firmware/lp5523/loading
	echo "9d80400004ff05ff437f0000" > /sys/class/firmware/lp5523/data
	echo 0 > /sys/class/firmware/lp5523/loading
	echo "111111111" > /sys/bus/i2c/devices/xxxx/engine2_leds
	echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
涓€鏃?鈥榣oading鈥?琚涓?0锛屾敞鍐岀殑鍥炶皟灏变細琚皟鐢ㄣ€傚湪鍥炶皟鍐呴儴锛屾墍閫夊紩鎿庤鍔犺浇锛屽瓨鍌ㄥ櫒
琚洿鏂般€傝杩愯宸茬紪绋嬬殑妯″紡锛屽簲鍚敤 鈥榬un_engine鈥?灞炴€с€?
LP8501 鐨勬ā寮忛『搴忎笌 LP5523 绫讳技銆?
涓嶈繃妯″紡鏁版嵁鏄壒瀹氱殑銆?
```

	echo 1 > /sys/bus/i2c/devices/xxxx/select_engine
	echo 1 > /sys/class/firmware/lp8501/loading
	echo "9d0140ff7e0040007e00a001c000" > /sys/class/firmware/lp8501/data
	echo 0 > /sys/class/firmware/lp8501/loading
	echo 1 > /sys/bus/i2c/devices/xxxx/run_engine

```
```

	echo 2 > /sys/bus/i2c/devices/xxxx/select_engine
	sleep 1
	echo 1 > /sys/class/firmware/lp8501/loading
	echo "9d0140ff7e0040007e00a001c000" > /sys/class/firmware/lp8501/data
	echo 0 > /sys/class/firmware/lp8501/loading
	sleep 1
	echo 3 > /sys/bus/i2c/devices/xxxx/select_engine
	sleep 1
	echo 1 > /sys/class/firmware/lp8501/loading
	echo "9d0340ff7e0040007e00a001c000" > /sys/class/firmware/lp8501/data
	echo 0 > /sys/class/firmware/lp8501/loading
	sleep 1
	echo 1 > /sys/class/leds/d1/device/run_engine

```
锛堚€榬un_engine鈥?涓?鈥榝irmware_cb鈥欙級

杩愯绋嬪簭鏁版嵁鐨勯『搴忔槸閫氱敤鐨勩€?
浣嗘瘡涓澶囬兘鏈夎嚜宸辩壒瀹氱殑鍛戒护瀵勫瓨鍣ㄥ湴鍧€銆?
涓烘锛屸€榬un_engine鈥?鍜?鈥榝irmware_cb鈥?鍦ㄦ瘡涓┍鍔ㄤ腑鏄彲閰嶇疆鐨勩€?
run_engine:
	鎺у埗鎵€閫夊紩鎿?firmware_cb:
	鍥轰欢鍔犺浇瀹屾垚鍚庣殑鍥炶皟鍑芥暟銆?
	鐢ㄤ簬鍔犺浇鍜屾洿鏂扮▼搴忓瓨鍌ㄥ櫒鐨勮姱鐗囩壒瀹氬懡浠ゃ€?
锛堥瀹氫箟妯″紡鏁版嵁锛?
濡傛灉娌℃湁鍥轰欢鎺ュ彛锛孡P55xx 椹卞姩鎻愪緵鍙︿竴绉嶅姞杞?LED 妯″紡鐨勬柟娉曘€傞偅灏辨槸鈥滈瀹氫箟鈥濇ā寮忋€?
棰勫畾涔夋ā寮忓畾涔夊湪骞冲彴鏁版嵁涓紝骞跺湪闇€瑕佹椂閫氳繃 sysfs 鍔犺浇瀹冿紙鎴栧畠浠級銆?
瑕佷娇鐢ㄩ瀹氫箟妯″紡鐨勬蹇碉紝搴旈厤缃?鈥榩atterns鈥?鍜?鈥榥um_patterns鈥欍€?
```

  /* mode_1: blinking data */
  static const u8 mode_1[] = {
		0x40, 0x00, 0x60, 0x00, 0x40, 0xFF, 0x60, 0x00,
		};

  /* mode_2: always on */
  static const u8 mode_2[] = { 0x40, 0xFF, };

  struct lp55xx_predef_pattern board_led_patterns[] = {
	{
		.r = mode_1,
		.size_r = ARRAY_SIZE(mode_1),
	},
	{
		.b = mode_2,
		.size_b = ARRAY_SIZE(mode_2),
	},
  }

  struct lp55xx_platform_data lp5562_pdata = {
  ...
	.patterns      = board_led_patterns,
	.num_patterns  = ARRAY_SIZE(board_led_patterns),
  };

```
```

  echo 1 > /sys/bus/i2c/devices/xxxx/led_pattern    # 绾㈣壊 LED 闂儊妯″紡
  echo 2 > /sys/bus/i2c/devices/xxxx/led_pattern    # 钃濊壊 LED 甯镐寒

```
```

  echo 0 > /sys/bus/i2c/devices/xxxx/led_pattern

```
