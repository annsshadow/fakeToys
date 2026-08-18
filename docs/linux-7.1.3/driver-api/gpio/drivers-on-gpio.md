## 浣跨敤 GPIO 鐨勫瓙绯荤粺椹卞姩


娉ㄦ剰锛屾爣鍑嗗唴鏍搁┍鍔ㄥ凡瀛樺湪浜庡父瑙佺殑 GPIO 浠诲姟锛屽苟涓斾細涓鸿宸ヤ綔鎻愪緵姝ｇ‘鐨勫唴鏍告€佷笌鐢ㄦ埛鎬?API/ABI锛岃€屼笖杩欎簺椹卞姩鍙互鍊熷姪璁惧鏍戞垨 ACPI 绛夌‖浠舵弿杩帮紝闈炲父瀹规槗鍦颁笌鍏朵粬鍐呮牳瀛愮郴缁?浜掕仈锛?
- leds-gpio锛歞rivers/leds/leds-gpio.c 灏嗗鐞嗚繛鎺ュ埌 GPIO 绾跨殑 LED锛屼负浣犳彁渚?LED 鐨?  sysfs 鎺ュ彛

- ledtrig-gpio锛歞rivers/leds/trigger/ledtrig-gpio.c 灏嗘彁渚涗竴涓?LED 瑙﹀彂鍣紝
  鍗充竴涓?LED 浼氭牴鎹?GPIO 绾垮彉涓洪珮鐢靛钩鎴栦綆鐢靛钩鑰屼寒/鐏?  锛堣€岃 LED 杩涜€屽張鍙兘濡備笂鎵€杩颁娇鐢?leds-gpio锛夈€?
- gpio-keys锛歞rivers/input/keyboard/gpio_keys.c 鐢ㄤ簬褰撲綘鐨?GPIO 绾胯兘
  鍦ㄦ寜閿寜涓嬫椂浜х敓涓柇鐨勬儏鍐点€備篃鏀寔鍘绘姈銆?
- gpio-keys-polled锛歞rivers/input/keyboard/gpio_keys_polled.c 鐢ㄤ簬褰撲綘鐨?  GPIO 绾挎棤娉曚骇鐢熶腑鏂€佷粠鑰岄渶瑕佺敱瀹氭椂鍣ㄥ懆鏈熸€ц疆璇㈢殑鎯呭喌銆?
- gpio_mouse锛歞rivers/input/mouse/gpio_mouse.c 鐢ㄤ簬閫氳繃浠呬娇鐢?GPIO 鑰屾棤闇€榧犳爣绔彛
  鏉ユ彁渚涗竴涓渶澶氫笁閿殑榧犳爣銆備綘鍙互鍓柇榧犳爣绾跨紗骞跺皢瀵肩嚎杩炲埌 GPIO 绾匡紝鎴栧皢榧犳爣杩炴帴鍣?  鐒婃帴鍒拌繖浜涚嚎涓婁互鑾峰緱鏇存寔涔呯殑姝ょ被鏂规銆?
- gpio-beeper锛歞rivers/input/misc/gpio-beeper.c 鐢ㄤ簬閫氳繃杩炴帴鍒?GPIO 绾跨殑澶栭儴鎵０鍣?  鍙戝嚭铚傞福澹般€傦紙濡傛灉铚傞福鐢卞紑/鍏虫帶鍒讹紝鑰岃浜х敓鐪熸鐨?PWM 娉㈠舰锛岃瑙佷笅鏂?pwm-gpio銆傦級

- pwm-gpio锛歞rivers/pwm/pwm-gpio.c 鐢ㄤ簬浠ラ珮鍒嗚鲸鐜囧畾鏃跺櫒缈昏浆 GPIO锛屽湪 GPIO 绾夸笂浜х敓
  PWM 娉㈠舰锛屾濡?Linux 楂樺垎杈ㄧ巼瀹氭椂鍣ㄦ墍鑳藉仛鍒扮殑閭ｆ牱銆?
- extcon-gpio锛歞rivers/extcon/extcon-gpio.c 鐢ㄤ簬褰撲綘闇€瑕佽鍙栧閮ㄨ繛鎺ュ櫒鐘舵€侊紙渚嬪闊抽
  椹卞姩鍣ㄧ殑鑰虫満绾挎垨 HDMI 杩炴帴鍣級鏃躲€傚畠浼氭彁渚涙瘮 GPIO 鏇村ソ鐨勭敤鎴锋€?sysfs 鎺ュ彛銆?
- restart-gpio锛歞rivers/power/reset/gpio-restart.c 鐢ㄤ簬閫氳繃鎷変綆涓€鏉?GPIO 绾挎潵閲嶅惎/
  閲嶆柊寮曞绯荤粺锛屽苟灏嗘敞鍐屼竴涓噸鍚鐞嗗櫒锛屼互渚跨敤鎴锋€佸彲浠ュ彂鍑烘纭殑绯荤粺璋冪敤鏉ラ噸鍚郴缁熴€?
- poweroff-gpio锛歞rivers/power/reset/gpio-poweroff.c 鐢ㄤ簬閫氳繃鎷変綆涓€鏉?GPIO 绾挎潵鍏抽棴
  绯荤粺鐢垫簮锛屽苟灏嗘敞鍐屼竴涓?pm_power_off() 鍥炶皟锛屼互渚跨敤鎴锋€佸彲浠ュ彂鍑烘纭殑绯荤粺璋冪敤鏉ュ叧闂?  绯荤粺鐢垫簮銆?
- gpio-gate-clock锛歞rivers/clk/clk-gpio.c 鐢ㄤ簬鎺у埗涓€涓娇鐢?GPIO 鐨勫彈鎺ф椂閽?  锛堝紑/鍏筹級锛屽苟涓庢椂閽熷瓙绯荤粺闆嗘垚銆?
- i2c-gpio锛歞rivers/i2c/busses/i2c-gpio.c 鐢ㄤ簬閫氳繃缈昏浆锛坆itbang锛変袱鏉?GPIO 绾挎潵椹卞姩
  涓€涓?I2C 鎬荤嚎锛堜袱鏉＄嚎锛孲DA 涓?SCL 绾匡級銆傚畠瀵圭郴缁熻€岃█灏嗗鍚屼换浣曞叾瀹?I2C 鎬荤嚎涓€鏍峰嚭鐜帮紝
  骞朵娇寰楀彲浠ュ儚杩炴帴浠讳綍鍏跺畠 I2C 鎬荤嚎椹卞姩閭ｆ牱锛岃繛鎺ユ€荤嚎涓?I2C 璁惧鐨勯┍鍔ㄣ€?
- spi_gpio锛歞rivers/spi/spi-gpio.c 鐢ㄤ簬閫氳繃 GPIO 缈昏浆锛坆itbang锛夋潵椹卞姩涓€涓?SPI 鎬荤嚎
  锛堝彲鍙樻暟閲忕殑绾匡紝鑷冲皯 SCK锛屼互鍙婂彲閫夌殑 MISO銆丮OSI 涓庣墖閫夌嚎锛夈€傚畠瀵圭郴缁熻€岃█灏嗗鍚屼换浣?  鍏跺畠 SPI 鎬荤嚎涓€鏍峰嚭鐜帮紝骞朵娇寰楀彲浠ュ儚杩炴帴浠讳綍鍏跺畠 SPI 鎬荤嚎椹卞姩閭ｆ牱锛岃繛鎺ユ€荤嚎涓?SPI 璁惧
  鐨勯┍鍔ㄣ€備緥濡傦紝浠讳綍 MMC/SD 鍗￠殢鍚庨兘鍙互閫氳繃鏉ヨ嚜 MMC/SD 鍗″瓙绯荤粺鐨?mmc_spi 涓绘満杩炴帴鍒?  姝?SPI銆?
- w1-gpio锛歞rivers/w1/masters/w1-gpio.c 鐢ㄤ簬閫氳繃涓€鏉?GPIO 绾块┍鍔ㄥ崟鎬荤嚎锛坥ne-wire锛夛紝
  涓?W1 瀛愮郴缁熼泦鎴愶紝骞跺儚澶勭悊浠讳綍鍏跺畠 W1 璁惧閭ｆ牱澶勭悊鎬荤嚎涓婄殑璁惧銆?
- gpio-fan锛歞rivers/hwmon/gpio-fan.c 鐢ㄤ簬鎺у埗杩炴帴鍒颁竴鏉?GPIO 绾匡紙浠ュ強鍙€夊湴涓€鏉?GPIO
  鍛婅绾匡級鐨勯鎵囨潵涓虹郴缁熸暎鐑紝鎻愪緵鎵€鏈夋纭殑鍐呮牳鎬佷笌 sysfs 鎺ュ彛锛屼娇浣犵殑绯荤粺涓嶄細杩囩儹銆?
- gpio-regulator锛歞rivers/regulator/gpio-regulator.c 鐢ㄤ簬閫氳繃鎷変綆涓€鏉?GPIO 绾挎潵鎺у埗
  鎻愪緵鏌愪竴鐢靛帇鐨勭ǔ鍘嬪櫒锛坮egulator锛夛紝涓庣ǔ鍘嬪櫒瀛愮郴缁熼泦鎴愶紝骞朵负浣犳彁渚涙墍鏈夋纭殑鎺ュ彛銆?
- gpio-wdt锛歞rivers/watchdog/gpio_wdt.c 鐢ㄤ簬鎻愪緵涓€涓湅闂ㄧ嫍瀹氭椂鍣紝瀹冨皢鍛ㄦ湡鎬у湴閫氳繃
  浠?1 鍒?0 鍐嶅埌 1 鍦扮炕杞繛鎺ュ埌 GPIO 绾跨殑纭欢鏉モ€減ing鈥濆畠銆傚鏋滆纭欢娌℃湁鍛ㄦ湡鎬у湴鏀跺埌
  瀹冪殑鈥減ing鈥濓紝瀹冨氨浼氶噸缃郴缁熴€?
- gpio-nand锛歞rivers/mtd/nand/raw/gpio.c 鐢ㄤ簬灏?NAND 闂瓨鑺墖杩炴帴鍒颁竴缁勭畝鍗曠殑 GPIO 绾匡細
  RDY銆丯CE銆丄LE銆丆LE銆丯WP銆傚畠涓?NAND 闂瓨 MTD 瀛愮郴缁熶氦浜掞紝骞舵彁渚涗笌鍏跺畠浠讳綍 NAND 椹卞姩
  纭欢涓€鏍风殑鑺墖璁块棶涓庡垎鍖鸿В鏋愩€?
- ps2-gpio锛歞rivers/input/serio/ps2-gpio.c 鐢ㄤ簬閫氳繃缈昏浆涓ゆ潯 GPIO 绾挎潵椹卞姩 PS/2锛圛BM锛?  serio 鎬荤嚎銆佹暟鎹笌鏃堕挓绾裤€傚畠瀵圭郴缁熻€岃█灏嗗鍚屼换浣曞叾瀹?serio 鎬荤嚎涓€鏍峰嚭鐜帮紝骞朵娇寰楀彲浠?  杩炴帴渚嬪閿洏浠ュ強鍏跺畠鍩轰簬 PS/2 鍗忚鐨勮澶囩殑椹卞姩銆?
- cec-gpio锛歞rivers/media/platform/cec-gpio/ 鐢ㄤ簬浠呬娇鐢?GPIO 鏉ヤ笌 CEC锛堟秷璐圭數瀛愭帶鍒讹級
  鎬荤嚎浜や簰銆傚畠鐢ㄤ簬涓?HDMI 鎬荤嚎涓婄殑璁惧閫氫俊銆?
- gpio-charger锛歞rivers/power/supply/gpio-charger.c 鐢ㄤ簬褰撲綘闇€瑕佸仛鐢垫睜鍏呯數锛岃€屾墍鏈?  鍙互鐢ㄦ潵妫€鏌ヤ氦娴佸厖鐢靛櫒瀛樺湪涓庡惁銆佹垨璇稿浣跨敤 GPIO 绾挎寚绀哄厖鐢电姸鎬佺瓑鏇村鏉備换鍔＄殑渚濇嵁
  鍙湁 GPIO 绾挎椂锛岃椹卞姩鎻愪緵杩欎簺鍔熻兘锛屽苟涓旇繕鎻愪緵浜嗕竴绉嶆竻鏅板畾涔夌殑鏂瑰紡锛岀敤浜庝粠璁惧鏍戠瓑
  纭欢鎻忚堪浼犻€掑厖鐢靛弬鏁般€?
- gpio-mux锛歞rivers/mux/gpio.c 鐢ㄤ簬閫氳繃 n 鏉?GPIO 绾挎帶鍒朵竴涓璺鐢ㄥ櫒锛屼粠鑰屼綘鍙互閫氳繃
  婵€娲讳笉鍚岀殑 GPIO 绾挎潵澶氳矾澶嶇敤杩?2^n 涓笉鍚岀殑璁惧銆侴PIO 閫氬父浣嶄簬 SoC 涓婏紝鑰岃澶囨槸
  鏌愪簺 SoC 澶栭儴鐨勫疄浣擄紝渚嬪 PCB 涓婂彲浠ユ湁閫夋嫨鍦板惎鐢ㄧ殑涓嶅悓缁勪欢銆?
闄や簡杩欎簺涔嬪锛岃繕鏈変竴浜涚壒娈?GPIO 椹卞姩浣嶄簬 MMC/SD 绛夊瓙绯荤粺涓紝鐢ㄤ簬璇诲彇鍗℃娴嬩笌鍐欎繚鎶?GPIO 绾匡紱浠ュ強浣嶄簬 TTY 涓茶瀛愮郴缁熶腑锛岀敤浜庨€氳繃浣跨敤涓ゆ潯 GPIO 绾挎潵妯℃嫙 MCTRL锛堣皟鍒惰В璋冨櫒
鎺у埗锛変俊鍙?CTS/RTS銆侻TD NOR 闂瓨涔熸湁鐢ㄤ簬棰濆 GPIO 绾跨殑闄勫姞浠讹紝灏界鍦板潃鎬荤嚎閫氬父鐩存帴
杩炲埌闂瓨銆?
璇蜂娇鐢ㄨ繖浜涢┍鍔紝鑰屼笉瑕佷粠鐢ㄦ埛鎬佺洿鎺ユ搷浣?GPIO锛涘畠浠瘮浣犵殑鐢ㄦ埛鎬佷唬鐮佽兘鏇村ソ鍦颁笌鍐呮牳
妗嗘灦闆嗘垚銆備笉鐢ㄨ锛屼粎浠呬娇鐢ㄩ€傚綋鐨勫唴鏍搁┍鍔紝灏辫兘閫氳繃鎻愪緵鐜版垚鐨勭粍浠讹紝鐗瑰埆鏄畝鍖栧苟鍔犻€?浣犵殑宓屽叆寮忓紑鍙戙€?