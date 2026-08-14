## ChipIdea 楂橀€熷弻瑙掕壊鎺у埗鍣ㄩ┍鍔?

### 1. 濡備綍娴嬭瘯 OTG FSM锛圚NP 涓?SRP锛?

灞曠ず濡備綍閫氳繃 sys 杈撳叆鏂囦欢锛岀敤 2 鍧?Freescale i.MX6Q sabre SD 鏉挎紨绀?OTG HNP 涓?SRP 鍔熻兘銆?
### 1.1 濡備綍鍚敤 OTG FSM


##### 1.1.1 鍦?menuconfig 涓€夋嫨 CONFIG_USB_OTG_FSM锛岄噸鏂版瀯寤哄唴鏍?

鏄犲儚涓庢ā鍧椼€傚鏋滀綘鎯虫鏌?otg fsm 鐨勪竴浜涘唴閮ㄥ彉閲忥紝鎸傝浇 debugfs锛屾湁浠ヤ笅 2 涓枃浠?```

	cat /sys/kernel/debug/ci_hdrc.0/otg
	cat /sys/kernel/debug/ci_hdrc.0/registers

```
##### 1.1.2 鍦ㄤ綘鐨?dts 鏂囦欢涓负浣犵殑鎺у埗鍣ㄨ妭鐐规坊鍔犱互涓嬫潯鐩?

```

	otg-rev = <0x0200>;
	adp-disable;

```
### 1.2 娴嬭瘯鎿嶄綔


1) 鐢ㄥ凡鍔犺浇 gadget 绫婚┍鍔紙渚嬪 g_mass_storage锛夌殑 2 鍧?Freescale i.MX6Q sabre SD 鏉夸笂鐢点€?
2) 鐢?usb 绾跨紗杩炴帴 2 鍧楁澘锛氫竴绔槸 micro A 鎻掑ご锛屽彟涓€绔槸 micro B 鎻掑ご銆?
   A 璁惧锛堟彃鍏?micro A 鎻掑ご锛夊簲褰撴灇涓?B 璁惧銆?
3) 瑙掕壊鍒囨崲

```

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/b_bus_req

   B 璁惧搴斿綋鎷呭綋涓绘満瑙掕壊骞舵灇涓?A 璁惧銆?
```
4) A 璁惧鍒囧洖涓绘満銆?
```

	echo 0 > /sys/bus/platform/devices/ci_hdrc.0/inputs/b_bus_req

   鎴栬€咃紝閫氳繃寮曞叆 HNP 杞锛孊-Host 鍙互鐭ラ亾 A-peripheral 浣曟椂甯屾湜澶勪簬涓绘満瑙掕壊锛屽洜姝ゆ瑙掕壊鍒囨崲涔熷彲浠?   鍦?A-peripheral 绔€氳繃搴旂瓟鏉ヨ嚜 B-Host 鐨勮疆璇㈡潵瑙﹀彂銆傝繖鍙互鍦?A 璁惧涓婂畬鎴?:

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/a_bus_req

   A 璁惧搴斿綋鍒囧洖涓绘満骞舵灇涓?B 璁惧銆?
```
5) 绉婚櫎 B 璁惧锛堟嫈涓?micro B 鎻掑ご锛夊苟鍦?10 绉掑唴閲嶆柊鎻掑叆锛汚 璁惧搴斿綋鍐嶆鏋氫妇 B 璁惧銆?
6) 绉婚櫎 B 璁惧锛堟嫈涓?micro B 鎻掑ご锛夊苟鍦?10 绉掑悗閲嶆柊鎻掑叆锛汚 璁惧搴斿綋**涓?*鏋氫妇 B 璁惧銆?
   濡傛灉 A 璁惧鎯宠浣跨敤鎬荤嚎锛?
```

	echo 0 > /sys/bus/platform/devices/ci_hdrc.0/inputs/a_bus_drop
	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/a_bus_req

   濡傛灉 B 璁惧鎯宠浣跨敤鎬荤嚎锛?
   鍦?B 璁惧涓?:

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/b_bus_req

```
7) A 璁惧鏂數鎬荤嚎銆?
```

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/a_bus_drop

   A 璁惧搴斿綋涓?B 璁惧鏂紑骞舵柇鐢垫€荤嚎銆?
```
8) B 璁惧涓?SRP 鍋氭暟鎹剦鍐层€?
```

	echo 1 > /sys/bus/platform/devices/ci_hdrc.0/inputs/b_bus_req

   A 璁惧搴斿綋鎭㈠ usb 鎬荤嚎骞舵灇涓?B 璁惧銆?
```
### 1.3 鍙傝€冩枃妗?

"On-The-Go and Embedded Host Supplement to the USB Revision 2.0 Specification
July 27, 2012 Revision 2.0 version 1.1a"

### 2. 濡備綍灏?USB 鍚敤涓虹郴缁熷敜閱掓簮


浠ヤ笅鏄浣曞湪 imx6 骞冲彴涓婂皢 USB 鍚敤涓虹郴缁熷敜閱掓簮鐨勭ず渚嬨€?
```

	echo enabled > /sys/bus/platform/devices/ci_hdrc.0/power/wakeup

```
```

	echo enabled > /sys/bus/platform/devices/2184000.usb/power/wakeup

```
```

	echo enabled > /sys/bus/platform/devices/20c9000.usbphy/power/wakeup

```
```

	echo enabled > /sys/bus/usb/devices/usb1/power/wakeup

```
```

	echo enabled > /sys/bus/usb/devices/1-1/power/wakeup

```
濡傛灉绯荤粺鍙湁涓€涓?usb 绔彛锛屽苟涓斾綘鎯冲湪璇ョ鍙ｅ惎鐢?usb 鍞ら啋锛屼綘
```

	for i in $(find /sys -name wakeup | grep usb);do echo enabled > $i;done;

```
