## Sony Notebook Control Driver (SNC) Readme


 - Copyright (C) 2004- 2005 Stelian Pop <stelian@popies.net>
 - Copyright (C) 2007 Mattia Dongili <malattia@linux.it>

杩欎釜杩蜂綘椹卞姩椹卞姩 Sony Vaio 绗旇鏈數鑴?ACPI BIOS 涓瓨鍦ㄧ殑 SNC 鍜?SPIC 璁惧銆傝椹卞姩鍦紙灏介噺涓€鑷寸殑锛夊悓涓€鎺ュ彛涓嬫贩鍚堜簡杩欎袱绉嶈澶囩殑鍔熻兘銆傝繖涔熸剰鍛崇潃 sonypi 椹卞姩鐜板湪宸茶 sony-laptop 鍙栦唬銆?
### Fn keys (hotkeys):


涓€浜涘瀷鍙烽€氳繃 SNC 鎴?SPIC 璁惧鎶ュ憡鐑敭锛屾绫讳簨浠舵棦閫氳繃 ACPI 瀛愮郴缁熶綔涓?acpi 浜嬩欢鎶ュ憡锛屼篃閫氳繃 INPUT 瀛愮郴缁熸姤鍛娿€傛煡鐪?/proc/bus/input/devices 鐨勬棩蹇楀彲浠ユ壘鍑鸿繖浜涗簨浠舵槸浠€涔堬紝浠ュ強椹卞姩鍒涘缓浜嗗摢浜涜緭鍏ヨ澶囥€傛澶栵紝浣跨敤 debug 閫夐」鍔犺浇椹卞姩浼氬湪鍐呮牳鏃ュ織涓姤鍛婃墍鏈変簨浠躲€?
浼犻€掔粰杈撳叆绯荤粺锛堝彲浠ョ敤 udev 閲嶆柊鏄犲皠锛夌殑鈥滄壂鎻忕爜鈥濇槸 sony-laptop.c 妯″潡涓〃 "sony_laptop_input_keycode_map" 鐨勭储寮曘€備緥濡?鈥淔N/E鈥?缁勫悎閿紙鍦ㄦ煇浜涘瀷鍙蜂笂鏄?EJECTCD锛夌敓鎴愭壂鎻忕爜 20锛?x14锛夈€?
### Backlight control:


濡傛灉浣犵殑绗旇鏈瀷鍙锋敮鎸侊紝浣犱細鍦?/sys/class/backlight/sony/
鐩綍涓嬫壘鍒?sysfs 鏂囦欢銆備綘灏嗚兘澶熸煡璇㈠拰璁剧疆褰撳墠灞忓箷浜害锛?
	======================	=========================================
	brightness		get/set screen brightness (an integer
				between 0 and 7)
	actual_brightness	reading from this file will query the HW
				to get real brightness value
	max_brightness		the maximum brightness value
	======================	=========================================


### Platform specific:


鍔犺浇 sony-laptop 妯″潡浼氬垱寤?/sys/devices/platform/sony-laptop/
鐩綍锛屽叾涓～鍏呬簡涓€浜涙枃浠躲€?
浣犲彲浠ラ€氳繃鏍囧噯 UNIX 宸ュ叿瀵硅繖浜涙枃浠惰繘琛屾暣鏁板€肩殑璇?鍐欍€?
杩欎簺鏂囦欢鏄細

	======================	==========================================
	brightness_default	screen brightness which will be set
				when the laptop will be rebooted
	cdpower			power on/off the internal CD drive
	audiopower		power on/off the internal sound card
	lanpower		power on/off the internal ethernet card
				(only in debug mode)
	bluetoothpower		power on/off the internal bluetooth device
	fanspeed		get/set the fan speed
	======================	==========================================

娉ㄦ剰锛屽鏋滄煇涓枃浠朵笉琚綘鐨勭壒瀹氱瑪璁版湰鍨嬪彿鏀寔锛屽畠鍙兘涓嶅瓨鍦ㄣ€?
```

	# echo "1" > /sys/devices/platform/sony-laptop/brightness_default

```
涓轰笅娆″強浠ュ悗鐨勯噸鍚缃渶浣庡睆骞曚寒搴?
```

	# echo "8" > /sys/devices/platform/sony-laptop/brightness_default

```
涓轰笅娆″強浠ュ悗鐨勯噸鍚缃渶楂樺睆骞曚寒搴?
```

	# cat /sys/devices/platform/sony-laptop/brightness_default

```
鑾峰彇璇ュ€?
```

	# echo "0" > /sys/devices/platform/sony-laptop/audiopower

```
鍏抽棴澹板崱

```

	# echo "1" > /sys/devices/platform/sony-laptop/audiopower

```
鎵撳紑澹板崱銆?

### RFkill control:


杈冩柊鐨?Vaio 鍨嬪彿鏆撮湶浜嗕竴缁勪竴鑷寸殑 ACPI 鏂规硶鏉ユ帶鍒跺皠棰戝彂灏勮澶囥€傚鏋滀綘鏈夊垢鎷ユ湁杩欐牱鐨勭瑪璁版湰锛屼綘浼氬湪
```

	# grep . /sys/class/rfkill/*/{state,name}


```
涓嬫壘鍒版墍闇€鐨?rfkill 璁惧

### Development:


濡傛灉浣犳兂甯姪寮€鍙戣繖涓┍鍔紙骞朵笖浣犱笉鎬曞浣犵殑 ACPI BIOS 鍋氬鎬殑浜嬫儏鍙兘缁欎綘鐨勭瑪璁版湰甯︽潵鐨勪换浣曞壇浣滅敤锛夛紝鍔犺浇椹卞姩骞朵紶鍏ラ€夐」 'debug=1'銆?
REPEAT:
	**濡傛灉浣犱笉鍠滄鍐掗櫓琛屼负锛屽氨涓嶈杩欐牱鍋氥€?*

鍦ㄥ唴鏍告棩蹇椾腑浣犱細鎵惧埌浣犵殑绗旇鏈笂 SNC 璁惧鎷ユ湁鐨勬墍鏈?ACPI 鏂规硶鍒楄〃銆?
- 瀵逛簬鏂板瀷鍙凤紝浣犱細鐪嬪埌涓€涓暱闀跨殑鏃犳剰涔夋柟娉曞悕鍒楄〃锛岄槄璇?DSDT 琛ㄦ簮鐮佸簲璇ヨ兘鎻ず锛?
(1) SNC 璁惧浣跨敤鍐呴儴鑳藉姏鏌ユ壘琛?(2) SN00 鐢ㄤ簬鍦ㄦ煡鎵捐〃涓煡鎵惧€?(3) SN06 鍜?SN07 鐢ㄤ簬鏍规嵁浣犲彲浠ラ€氳繃 SN00 杩唬琛ㄨ幏寰楃殑鍋忕Щ閲忚皟鐢ㄧ湡瀹炴柟娉?(4) SN02 鐢ㄤ簬鍚敤浜嬩欢銆?
鑳藉姏鏌ユ壘琛ㄤ腑鐨勪竴浜涘€兼垨澶氭垨灏戞槸宸茬煡鐨勶紝鍙傝鎵€鏈?sony_call_snc_handle 璋冪敤鐨勪唬鐮侊紝鍏朵粬鍒欐洿鏅︽订銆?
- 瀵逛簬鏃у瀷鍙凤紝浣犲彲浠ョ湅鍒扮敤浜庢墦寮€/鍏抽棴 CD 椹卞姩鐨?GCDP/GCDP 鏂规硶锛屼絾杩樻湁鍏朵粬鏂规硶锛屽苟涓斿畠浠€氬父鍥犲瀷鍙疯€屽紓銆?
**鎴戝畬鍏ㄤ笉鐭ラ亾閭ｄ簺鏂规硶鏄仛浠€涔堢殑銆?*

sony-laptop 椹卞姩涓哄叾涓竴浜涙柟娉曪紙鍦ㄥ涓?Vaio 鍨嬪彿涓婃壘鍒扮殑鏈€鏂版柟娉曪級鍦?/sys/devices/platform/sony-laptop 涓嬪垱寤轰簡涓€涓潯鐩紝灏卞儚 'cdpower' 閭ｆ牱銆備綘鍙互閫氳繃杩涗竴姝ョ紪杈戞簮鐮侊紙鍙傝 'sony_nc_values' 琛紝骞朵娇鐢?SNC_HANDLE_NAMES 瀹忔妸浣犵殑 get/set 鏂规硶鍚嶄綔涓烘柊鏉＄洰鍔犲叆璇ヨ〃锛夋潵鍒涘缓瀵瑰簲浜庝綘鑷繁绗旇鏈柟娉曠殑鍏朵粬鏉＄洰銆?
浣犵殑浠诲姟锛堝鏋滀綘鎺ュ彈鐨勮瘽锛夋槸灏濊瘯閫氳繃浠庤繖浜涙枃浠惰/鍐欓殢鏈哄€兼潵鎵惧嚭杩欎簺鏉＄洰鏄仛浠€涔堢敤鐨勶紝浠ュ強瀹冧滑瀵逛綘鐨勭瑪璁版湰鏈変粈涔堝奖鍝嶃€?
濡傛灉浣犲彂鐜颁簡浠讳綍鏈夎叮鐨勪笢瑗匡紝璇峰洖鎶ョ粰鎴戯紝鎴戜笉浼氬惁璁ゅ浣犺涓虹殑鍏ㄩ儴浜嗚В :)

鍙﹁ http://www.linux.it/~malattia/wiki/index.php/Sony_drivers 鑾峰彇鍏朵粬鏈夌敤淇℃伅銆?
### Bugs/Limitations:


- 璇ラ┍鍔ㄥ苟闈炲熀浜?Sony 鐨勫畼鏂规枃妗ｏ紙鍥犱负鏍规湰娌℃湁锛夛紝鍥犳涓嶄繚璇佽椹卞姩鑳藉伐浣滐紝鎴栧仛姝ｇ‘鐨勪簨銆傚敖绠¤繖娌℃湁鍙戠敓鍦ㄦ垜韬笂锛屼絾璇ラ┍鍔ㄥ彲鑳藉浣犵殑绗旇鏈仛寰堢碂绯曠殑浜嬶紝鍖呮嫭姘镐箙鎬ф崯鍧忋€?
- sony-laptop 鍜?sonypi 椹卞姩涔嬮棿瀹屽叏涓嶄氦浜掋€傚皢鏉ワ紝sonypi 灏嗚绉婚櫎骞剁敱 sony-laptop 鍙栦唬銆?
- spicctrl 鏄敤浜庝笌 sonypi 椹卞姩锛堥€氳繃 /dev/sonypi锛夐€氫俊鐨勭敤鎴风┖闂村伐鍏凤紝涔熷凡琚純鐢紝鍥犱负瀹冪殑鎵€鏈夌壒鎬х幇鍦ㄩ兘鍙互閫氳繃 sony-laptop 鍦?sysfs 鏍戜笅浣跨敤銆?