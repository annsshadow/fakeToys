
### V4L2 璁惧瀹炰緥


姣忎釜璁惧瀹炰緥鐢变竴涓?struct v4l2_device 琛ㄧず銆傞潪甯哥畝鍗曠殑璁惧鍙互鐩存帴鍒嗛厤杩欎釜缁撴瀯浣擄紝
浣嗗湪澶у鏁版儏鍐典笅浣犱細鎶婂畠宓屽叆鍒颁竴涓洿澶х殑缁撴瀯浣撲腑銆?
浣犲繀椤婚€氳繃璋冪敤浠ヤ笅鍑芥暟鏉ユ敞鍐岃澶囧疄渚嬶細

	`v4l2_device_register <v4l2_device_register>`
	(dev, `v4l2_dev <v4l2_device>`)銆?
娉ㄥ唽灏嗗垵濮嬪寲 `v4l2_device` 缁撴瀯浣撱€傚鏋?dev->driver_data 瀛楁涓?`NULL`锛?瀹冨皢琚摼鎺ュ埌 `v4l2_dev <v4l2_device>` 鍙傛暟銆?
甯屾湜涓庡獟浣撹澶囨鏋堕泦鎴愮殑椹卞姩锛岄渶瑕佹墜鍔ㄨ缃?dev->driver_data锛屼娇鍏舵寚鍚戝祵鍏ヤ簡
struct v4l2_device 瀹炰緥鐨勯┍鍔ㄧ壒瀹氳澶囩粨鏋勩€傝繖鏄€氳繃鍦ㄦ敞鍐?V4L2 璁惧瀹炰緥涔嬪墠璋冪敤涓€娆?`dev_set_drvdata()` 鏉ュ疄鐜扮殑銆傚畠浠繕蹇呴』灏?struct v4l2_device 鐨?mdev 瀛楁璁剧疆涓?鎸囧悜涓€涓凡姝ｇ‘鍒濆鍖栧苟娉ㄥ唽鐨?`media_device` 瀹炰緥銆?
濡傛灉 `v4l2_dev <v4l2_device>`\ ->name 涓虹┖锛屽垯瀹冨皢琚涓轰粠 dev 娲剧敓鐨勫€?锛堢‘鍒囧湴璇达紝鏄┍鍔ㄥ悕鍚庤窡 bus_id锛夈€傚鏋滀綘鍦ㄨ皟鐢?`v4l2_device_register` 涔嬪墠璁剧疆濂藉畠锛?瀹冨皢淇濇寔涓嶅彉銆傚鏋?dev 涓?`NULL`锛岄偅涔堜綘**蹇呴』**鍦ㄨ皟鐢?`v4l2_device_register`
涔嬪墠璁剧疆濂?`v4l2_dev <v4l2_device>`\ ->name銆?
浣犲彲浠ヤ娇鐢?`v4l2_device_set_name` 鏉ュ熀浜庨┍鍔ㄥ悕鍜屼竴涓┍鍔ㄥ叏灞€鐨?atomic_t 瀹炰緥璁剧疆鍚嶇О銆?杩欏皢鐢熸垚璇稿 `ivtv0`銆乣ivtv1` 杩欐牱鐨勫悕绉般€傚鏋滃悕绉颁互鏁板瓧缁撳熬锛屽垯浼氭彃鍏ヤ竴涓煭妯嚎锛?`cx18-0`銆乣cx18-1` 绛夈€傝鍑芥暟杩斿洖瀹炰緥缂栧彿銆?
绗竴涓?`dev` 鍙傛暟閫氬父鏄?`pci_dev`銆乣usb_interface` 鎴?`platform_device` 鐨?`struct device` 鎸囬拡銆俤ev 涓?`NULL` 鐨勬儏鍐靛緢灏戣锛屼絾鍦?ISA 璁惧鎴栧綋涓€涓澶囧垱寤哄涓?PCI 璁惧鏃朵細鍙戠敓锛屼粠鑰屼娇寰楁棤娉曞皢 `v4l2_dev <v4l2_device>` 鍏宠仈鍒版煇涓壒瀹氱殑鐖惰澶囥€?
浣犺繕鍙互鎻愪緵涓€涓?`notify()` 鍥炶皟锛屽瓙璁惧鍙互璋冪敤瀹冩潵閫氱煡浣犱簨浠躲€傛槸鍚﹂渶瑕佽缃畠鍙栧喅浜?瀛愯澶囥€傚瓙璁惧鏀寔鐨勪换浣曢€氱煡蹇呴』鍦?`include/media/subdevice.h` 涓殑涓€涓ご鏂囦欢閲屽畾涔夈€?
V4L2 璁惧閫氳繃璋冪敤浠ヤ笅鍑芥暟娉ㄩ攢锛?
	`v4l2_device_unregister`
	(`v4l2_dev <v4l2_device>`)銆?
濡傛灉 dev->driver_data 瀛楁鎸囧悜 `v4l2_dev <v4l2_device>`锛屽畠灏嗚閲嶇疆涓?`NULL`銆?娉ㄩ攢涔熶細鑷姩娉ㄩ攢璇ヨ澶囦笂鐨勬墍鏈夊瓙璁惧锛坰ubdev锛夈€?
濡傛灉浣犳湁涓€涓彲鐑彃鎷旇澶囷紙渚嬪 USB 璁惧锛夛紝閭ｄ箞鍦ㄦ柇寮€杩炴帴鍙戠敓鏃剁埗璁惧浼氬彉寰楁棤鏁堛€?鐢变簬 `v4l2_device` 鏈変竴涓寚鍚戣鐖惰澶囩殑鎸囬拡锛屽畠涔熷繀椤昏娓呴櫎锛屼互鏍囪鐖惰澶囧凡缁忔秷澶便€?涓烘璇疯皟鐢細

	`v4l2_device_disconnect`
	(`v4l2_dev <v4l2_device>`)銆?
杩欏苟**涓嶄細**娉ㄩ攢瀛愯澶囷紝鍥犳浣犱粛鐒堕渶瑕佷负姝よ皟鐢?`v4l2_device_unregister` 鍑芥暟銆?濡傛灉浣犵殑椹卞姩涓嶅彲鐑彃鎷旓紝鍒欐棤闇€璋冪敤 `v4l2_device_disconnect`銆?
鏈夋椂浣犻渶瑕侀亶鍘嗘煇涓壒瀹氶┍鍔ㄦ敞鍐岀殑鎵€鏈夎澶囥€傚綋澶氫釜璁惧椹卞姩浣跨敤鐩稿悓鐨勭‖浠舵椂閫氬父鏄繖绉?鎯呭喌銆備緥濡傦紝ivtvfb 椹卞姩鏄竴涓娇鐢?ivtv 纭欢鐨勫抚缂撳啿椹卞姩銆侫LSA 椹卞姩鍚岀悊銆?
浣犲彲浠ュ涓嬮亶鍘嗘墍鏈夊凡娉ㄥ唽鐨勮澶囷細


	static int callback(struct device **dev, void **p)
	{
		struct v4l2_device *v4l2_dev = dev_get_drvdata(dev);

		/** 娴嬭瘯璇ヨ澶囨槸鍚﹀凡鍒濆鍖?**/
		if (v4l2_dev == NULL)
			return 0;
		...
		return 0;
	}

	int iterate(void *p)
	{
		struct device_driver *drv;
		int err;

		/* 鍦?PCI 鎬荤嚎涓婃煡鎵鹃┍鍔?'ivtv'銆?		pci_bus_type 鏄竴涓叏灞€鍙橀噺銆傚浜?USB 鎬荤嚎浣跨敤 usb_bus_type銆?*/
		drv = driver_find("ivtv", &pci_bus_type);
		/** 閬嶅巻鎵€鏈?ivtv 璁惧瀹炰緥 **/
		err = driver_for_each_device(drv, NULL, p, callback);
		put_driver(drv);
		return err;
	}

鏈夋椂浣犻渶瑕佺淮鎶や竴涓澶囧疄渚嬬殑杩愯璁℃暟鍣ㄣ€傝繖閫氬父鐢ㄤ簬灏嗚澶囧疄渚嬫槧灏勫埌妯″潡閫夐」鏁扮粍鐨勭储寮曘€?
鎺ㄨ崘鐨勫仛娉曞涓嬶細


	static atomic_t drv_instance = ATOMIC_INIT(0);

	static int drv_probe(struct pci_dev **pdev, const struct pci_device_id **pci_id)
	{
		...
		state->instance = atomic_inc_return(&drv_instance) - 1;
	}

濡傛灉浣犳湁澶氫釜璁惧鑺傜偣锛岄偅涔堝浜庡彲鐑彃鎷旇澶囷紝鍙兘寰堥毦鐭ラ亾浣曟椂娉ㄩ攢 `v4l2_device` 鎵嶆槸
瀹夊叏鐨勩€備负姝?`v4l2_device` 鎻愪緵浜嗗紩鐢ㄨ鏁帮紙refcounting锛夋敮鎸併€傛瘡褰撹皟鐢?`video_register_device` 鏃跺紩鐢ㄨ鏁板姞涓€锛屾瘡褰撹璁惧鑺傜偣琚噴鏀炬椂鍑忎竴銆傚綋寮曠敤璁℃暟
褰掗浂鏃讹紝灏嗚皟鐢?`v4l2_device` 鐨?release() 鍥炶皟銆備綘鍙互鍦ㄩ偅閲屽仛鏈€缁堢殑娓呯悊銆?
濡傛灉鍒涘缓浜嗗叾瀹冭澶囪妭鐐癸紙渚嬪 ALSA锛夛紝浣犱篃鍙互閫氳繃璋冪敤浠ヤ笅鍑芥暟鎵嬪姩澧炲噺寮曠敤璁℃暟锛?
	`v4l2_device_get`
	(`v4l2_dev <v4l2_device>`)銆?
鎴栵細

	`v4l2_device_put`
	(`v4l2_dev <v4l2_device>`)銆?
鐢变簬鍒濆寮曠敤璁℃暟涓?1锛屼綘杩橀渶瑕佸湪 `disconnect()` 鍥炶皟锛堝浜?USB 璁惧锛夋垨 `remove()`
鍥炶皟锛堜緥濡傚浜?PCI 璁惧锛変腑璋冪敤 `v4l2_device_put`锛屽惁鍒欏紩鐢ㄨ鏁版案杩滀笉浼氬綊闆躲€?
##### v4l2_device 鍑芥暟涓庢暟鎹粨鏋?