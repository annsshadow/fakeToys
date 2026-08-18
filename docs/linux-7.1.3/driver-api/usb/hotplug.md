#### USB 鐑彃鎷?


## Linux 鐑彃鎷?



鍦?USB锛堜互鍙?Cardbus PCI锛夎繖绫诲彲鐑彃鎷旂殑鎬荤嚎涓婏紝缁堢鐢ㄦ埛鍦ㄧ郴缁熼€氱數鐘舵€佷笅灏嗚澶囨彃鍏ユ€荤嚎銆傚湪澶у鏁版儏鍐典笅锛岀敤鎴峰笇鏈涜澶囪兘澶熺珛鍗冲彲鐢ㄣ€傝繖鎰忓懗鐫€绯荤粺蹇呴』瀹屾垚璁稿宸ヤ綔锛屽寘鎷細

    - 鎵惧埌涓€涓兘澶熷鐞嗚璁惧鐨勯┍鍔ㄣ€傝繖鍙兘娑夊強鍔犺浇涓€涓唴鏍告ā鍧楋紱杈冩柊鐨勯┍鍔ㄥ彲浠ヤ娇鐢?module-init-tools
      灏嗗叾璁惧锛堝強绫诲埆锛夋敮鎸佷俊鎭彂甯冪粰瀹炵敤宸ュ叿銆?

    - 灏嗕竴涓┍鍔ㄧ粦瀹氬埌璇ヨ澶囥€傛€荤嚎妗嗘灦閫氳繃鍏惰澶囬┍鍔ㄧ殑 probe() 渚嬬▼鏉ュ畬鎴愯繖涓€宸ヤ綔銆?

    - 閫氱煡鍏朵粬瀛愮郴缁熼厤缃柊璁惧銆傛墦鍗伴槦鍒楀彲鑳介渶瑕佸惎鐢紝缃戠粶闇€瑕佸惎鍔紝纾佺洏鍒嗗尯闇€瑕佹寕杞斤紝绛夌瓑銆?
      鍦ㄦ煇浜涙儏鍐典笅锛岃繖浜涘皢鏄┍鍔ㄧ壒瀹氱殑鎿嶄綔銆?

杩欐秹鍙婂唴鏍告€佷笌鐢ㄦ埛鎬佹搷浣滅殑娣峰悎銆備娇璁惧绔嬪嵆鍙敤鎰忓懗鐫€浠讳綍鐢ㄦ埛鎬佹搷浣滈兘涓嶈兘绛夊緟绠＄悊鍛樺幓鎵ц锛?
鍐呮牳蹇呴』瑙﹀彂瀹冧滑锛屾棤璁烘槸琚姩鍦帮紙瑙﹀彂鏌愪釜鐩戞帶瀹堟姢杩涚▼璋冪敤杈呭姪绋嬪簭锛夎繕鏄富鍔ㄥ湴锛堢洿鎺ヨ皟鐢ㄨ繖鏍风殑
鐢ㄦ埛鎬佽緟鍔╃▼搴忥級銆?

閭ｄ簺琚Е鍙戠殑鎿嶄綔蹇呴』鏀寔绯荤粺鐨勭鐞嗙瓥鐣ワ紱姝ょ被绋嬪簭鍦ㄦ琚О涓衡€滅瓥鐣ヤ唬鐞嗏€濓紙policy agents锛夈€?
瀹冧滑閫氬父娑夊強 shell 鑴氭湰锛岃繖浜涜剼鏈垎娲剧粰鏇翠负鐔熸倝鐨勭鐞嗗伐鍏枫€?

鐢变簬鍏朵腑涓€浜涙搷浣滀緷璧栦簬鍏充簬椹卞姩锛堝厓鏁版嵁锛夌殑淇℃伅锛岃€岃繖浜涗俊鎭洰鍓嶄粎鍦ㄩ┍鍔ㄨ鍔ㄦ€侀摼鎺ユ椂鎵嶅彲鐢紝
鍥犳褰撲綘閰嶇疆涓€涓珮搴︽ā鍧楀寲鐨勭郴缁熸椂锛屽彲浠ヨ幏寰楁渶浣崇殑鐑彃鎷旀晥鏋溿€?

## 鍐呮牳鐑彃鎷旇緟鍔╃▼搴?(``/sbin/hotplug``)


瀛樺湪涓€涓唴鏍稿弬鏁帮細`/proc/sys/kernel/hotplug`锛屽畠閫氬父淇濆瓨璺緞鍚?`/sbin/hotplug`銆傝鍙傛暟鎸囧畾浜嗕竴涓?
绋嬪簭锛屽唴鏍稿彲鍦ㄤ笉鍚屾椂鏈鸿皟鐢ㄥ畠銆?

/sbin/hotplug 绋嬪簭鍙敱浠讳綍瀛愮郴缁熶綔涓哄叾鍝嶅簲閰嶇疆鍙樻洿鐨勪竴閮ㄥ垎锛屼粠璇ョ郴缁熶腑鐨勪竴涓嚎绋嬭皟鐢ㄣ€傚彧闇€瑕佷竴涓弬鏁帮細
琚€氱煡鏌愬唴鏍镐簨浠剁殑瀛愮郴缁熷悕绉般€傝鍚嶇О琚敤浣滆繘涓€姝ヤ簨浠跺垎娲剧殑绗竴鎶婇挜鍖欙紱浠讳綍鍏朵粬鍙傛暟鍜岀幆澧冨弬鏁扮敱
鍙戣捣璇ヨ皟鐢ㄧ殑瀛愮郴缁熸寚瀹氥€?

鐑彃鎷旇蒋浠跺強鍏朵粬璧勬簮鍙湪浠ヤ笅浣嶇疆鑾峰彇锛?

	http://linux-hotplug.sourceforge.net

閭欢鍒楄〃淇℃伅涔熷彲鍦ㄨ绔欑偣鑾峰彇銆?


## USB 绛栫暐浠ｇ悊


USB 瀛愮郴缁熷綋鍓嶅湪 USB 璁惧琚坊鍔犳垨浠庣郴缁熶腑绉婚櫎鏃惰皟鐢?`/sbin/hotplug`銆傝璋冪敤鐢卞唴鏍?hub 宸ヤ綔闃熷垪
[hub_wq] 瀹屾垚锛屾垨浣滀负鏍?hub 鍒濆鍖栫殑涓€閮ㄥ垎锛堢敱 init銆乵odprobe銆乲apmd 绛夊畬鎴愶級銆傚畠鍞竴鐨勫懡浠よ鍙傛暟鏄?
瀛楃涓?"usb"锛屽苟浼犻€掍互涓嬬幆澧冨彉閲忥細

========== ============================================
ACTION     `add`, `remove`
PRODUCT    USB 鍘傚晢銆佷骇鍝佸拰鐗堟湰浠ｇ爜锛堝崄鍏繘鍒讹級
TYPE       璁惧绫诲埆浠ｇ爜锛堝崄杩涘埗锛?
INTERFACE  鎺ュ彛 0 鐨勭被鍒唬鐮侊紙鍗佽繘鍒讹級
========== ============================================

濡傛灉閰嶇疆浜?"usbdevfs"锛屽垯杩樹細浼犻€?DEVICE 鍜?DEVFS銆侱EVICE 鏄璁惧鐨勮矾寰勫悕锛屽浜庡叿鏈夊涓拰/鎴?
澶囩敤鎺ュ彛銆佷粠鑰屼护椹卞姩閫夋嫨澶嶆潅鍖栫殑璁惧寰堟湁鐢ㄣ€傛寜璁捐锛孶SB 鐑彃鎷旂嫭绔嬩簬 `usbdevfs`锛氫綘鍙互鍦ㄤ笉浣跨敤璇?
鏂囦欢绯荤粺銆佷篃涓嶈繍琛岀敤鎴锋€佸畧鎶よ繘绋嬫潵妫€娴嬬郴缁熼厤缃彉鍖栫殑鎯呭喌涓嬶紝瀹屾垚 USB 璁惧璁剧疆鐨勫ぇ閮ㄥ垎鍏抽敭閮ㄥ垎銆?

褰撳墠鍙敤鐨勭瓥鐣ヤ唬鐞嗗疄鐜板彲浠ヤ负妯″潡鍔犺浇椹卞姩锛屽苟鍙互璋冪敤椹卞姩鐗瑰畾鐨勮缃剼鏈€傛渶鏂扮殑瀹炵幇鍒╃敤浜?USB
module-init-tools 鏀寔銆傚悗缁殑浠ｇ悊鍙兘浼氬嵏杞介┍鍔ㄣ€?


## USB Modutils 鏀寔


褰撳墠鐗堟湰鐨?module-init-tools 浼氬垱寤轰竴涓?`modules.usbmap` 鏂囦欢锛屽叾涓寘鍚瘡涓┍鍔?`MODULE_DEVICE_TABLE`
涓殑鏉＄洰銆傛绫绘枃浠跺彲琚悇绉嶇敤鎴锋€佺瓥鐣ヤ唬鐞嗙敤鏉ョ‘淇濆姞杞芥墍鏈夋纭殑椹卞姩妯″潡锛屾棤璁烘槸鍦ㄥ惎鍔ㄦ椂鍒昏繕鏄箣鍚庛€?

鏈夊叧姝ょ被琛ㄦ潯鐩殑瀹屾暣淇℃伅锛岃鍙傝 `linux/usb.h`锛涙垨鏌ョ湅鐜版湁椹卞姩銆傛瘡涓〃鏉＄洰鎻忚堪浜嗕竴涓垨澶氫釜鍦ㄥ皢椹卞姩
涓庢煇璁惧鎴栬澶囩被鍒繘琛屽尮閰嶆椂鎵€浣跨敤鐨勫垽鎹€傚叿浣撶殑鍒ゆ嵁鐢?"match_flags" 涓疆浣嶇殑浣嶄笌瀛楁鍊奸厤瀵规潵鏍囪瘑銆?
浣犲彲浠ョ洿鎺ユ瀯閫犺繖浜涘垽鎹紝鎴栦娇鐢?
```

    USB_DEVICE (vendorId, productId)
	... matching devices with specified vendor and product ids
    USB_DEVICE_VER (vendorId, productId, lo, hi)
	... like USB_DEVICE with lo <= productversion <= hi
    USB_INTERFACE_INFO (class, subclass, protocol)
	... matching specified interface class info
    USB_DEVICE_INFO (class, subclass, protocol)
	... matching specified device class info

```
涓嬮潰鏄竴涓畝鐭ず渚嬶紝閫傜敤浜庝竴涓敮鎸佽嫢骞茬壒瀹?USB 璁惧鐨勯┍鍔?
```

    static const struct usb_device_id mydriver_id_table[] = {
	{ USB_DEVICE (0x9999, 0xaaaa), driver_info: QUIRK_X },
	{ USB_DEVICE (0xbbbb, 0x8888), driver_info: QUIRK_Y|QUIRK_Z },
	...
	{ } /* end with an all-zeroes entry */
    };
    MODULE_DEVICE_TABLE(usb, mydriver_id_table);

```
澶у鏁?USB 璁惧椹卞姩搴斿綋灏嗚繖浜涜〃鍚屾椂浼犻€掔粰 USB 瀛愮郴缁熷拰妯″潡绠＄悊瀛愮郴缁熴€備笉杩囧苟闈炴墍鏈夐┍鍔ㄩ兘濡傛锛?
鏌愪簺椹卞姩妗嗘灦閫氳繃鏋勫缓鍦?USB 涔嬩笂鐨勬帴鍙ｈ繘琛岃繛鎺ワ紝鍥犳瀹冧滑涓嶉渶瑕佽繖鏍风殑 struct usb_driver銆?

鐩存帴杩炴帴鍒?USB 瀛愮郴缁熺殑椹卞姩搴斿綋鎸夊涓嬫柟寮忓０鏄?
```

    static struct usb_driver mydriver = {
	.name		= "mydriver",
	.id_table	= mydriver_id_table,
	.probe		= my_probe,
	.disconnect	= my_disconnect,

	/*
	if using the usb chardev framework:
	    .minor		= MY_USB_MINOR_START,
	    .fops		= my_file_ops,
	if exposing any operations through usbdevfs:
	    .ioctl		= my_ioctl,
	*/
    };

```
褰?USB 瀛愮郴缁熻幏鐭ユ煇涓┍鍔ㄧ殑 device ID 琛ㄥ悗锛屽畠浼氬湪閫夋嫨瑕?probe() 鐨勯┍鍔ㄦ椂浣跨敤璇ヨ〃銆傝礋璐ｆ柊璁惧
澶勭悊鐨勭嚎绋嬩細灏嗗悇椹卞姩鐨?device ID 鏉＄洰涓庤澶囩殑鎺ュ彛鍜?device 鎻忚堪绗﹁繘琛屾瘮瀵广€傚彧鏈夊湪鍖归厤鏃跺畠鎵嶄細璋冪敤
`probe()`锛岃€屼紶缁?`probe()` 鐨勭涓変釜鍙傛暟灏嗘槸鎵€鍖归厤鐨勬潯鐩€?

濡傛灉浣犳病鏈変负椹卞姩鎻愪緵 `id_table`锛岄偅涔堜綘鐨勯┍鍔ㄥ彲鑳戒細閽堝姣忎釜鏂拌澶囬兘琚?probe 涓€娆★紱浼犵粰 `probe()`
鐨勭涓変釜鍙傛暟灏嗘槸 `NULL`銆?
