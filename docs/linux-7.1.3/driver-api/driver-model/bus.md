## 鎬荤嚎绫诲瀷锛圔us Types锛?

#### 瀹氫箟

鍙傝 struct bus_type 鐨勫唴鏍告枃妗ｏ紙kerneldoc锛夈€?
int bus_register(struct bus_type * bus);


#### 澹版槑


鍐呮牳涓殑姣忕鎬荤嚎绫诲瀷锛圥CI銆乁SB 绛夛級閮藉簲澹版槑涓€涓绫诲瀷鐨勯潤鎬佸璞°€傚畠浠繀椤诲垵濮嬪寲 name 瀛楁锛屽苟涓斿彲浠?```

   struct bus_type pci_bus_type = {
          .name	= "pci",
          .match	= pci_bus_match,
   };

```
璇ョ粨鏋勫簲鍦ㄥご鏂囦欢涓鍑虹粰椹卞姩锛?
extern struct bus_type pci_bus_type;


#### 娉ㄥ唽


褰撴€荤嚎椹卞姩琚垵濮嬪寲鏃讹紝瀹冭皟鐢?bus_register銆傝繖浼氬垵濮嬪寲鎬荤嚎瀵硅薄涓叾浣欑殑瀛楁锛屽苟灏嗗叾鎻掑叆鍒板叏灞€鎬荤嚎绫诲瀷鍒楄〃涓€備竴鏃︽€荤嚎瀵硅薄琚敞鍐岋紝鍏朵腑鐨勫瓧娈靛嵆鍙敱鎬荤嚎椹卞姩浣跨敤銆?

#### 鍥炶皟


#### match()锛氬皢椹卞姩闄勫姞鍒拌澶?

璁惧 ID 缁撴瀯鐨勬牸寮忎互鍙婃瘮杈冨畠浠殑璇箟鏈川涓婃槸鎬荤嚎鐩稿叧鐨勩€傞┍鍔ㄩ€氬父鍦ㄦ€荤嚎鐩稿叧鐨勯┍鍔ㄧ粨鏋勪腑澹版槑涓€涓畠浠墍鏀寔璁惧鐨勮澶?ID 鏁扮粍銆?
match 鍥炶皟鐨勭洰鐨勬槸鍦ㄤ笉鐗虹壊鎬荤嚎鐩稿叧鍔熻兘鎴栫被鍨嬪畨鍏ㄧ殑鍓嶆彁涓嬶紝缁欐€荤嚎涓€涓満浼氾紝閫氳繃姣旇緝椹卞姩鎵€鏀寔鐨勮澶?ID 涓庣壒瀹氳澶囩殑璁惧 ID锛屾潵鍒ゆ柇鏌愪釜鐗瑰畾椹卞姩鏄惁鏀寔鏌愪釜鐗瑰畾璁惧銆?
褰撳湪鏌愪釜鎬荤嚎涓婃敞鍐屼竴涓┍鍔ㄦ椂锛屼細閬嶅巻璇ユ€荤嚎鐨勮澶囧垪琛紝骞跺姣忎釜灏氭湭鍏宠仈椹卞姩鐨勮澶囪皟鐢?match 鍥炶皟銆?

#### 璁惧鍜岄┍鍔ㄥ垪琛?

璁惧鍜岄┍鍔ㄥ垪琛ㄦ棬鍦ㄥ彇浠ｈ澶氭€荤嚎缁存姢鐨勬湰鍦板垪琛ㄣ€傚畠浠垎鍒槸 struct device 鍜?struct device_driver 鐨勫垪琛ㄣ€傛€荤嚎椹卞姩鍙互闅忔剰浣跨敤杩欎簺鍒楄〃锛屼絾鍙兘闇€瑕佸皢鍏惰浆鎹负鎬荤嚎鐩稿叧鐨勭被鍨嬨€?```

  int bus_for_each_dev(struct bus_type * bus, struct device * start,
		       void * data,
		       int (*fn)(struct device *, void *));

  int bus_for_each_drv(struct bus_type * bus, struct device_driver * start,
		       void * data, int (*fn)(struct device_driver *, void *));

```
杩欎簺杈呭姪鍑芥暟閬嶅巻鐩稿簲鐨勫垪琛紝骞朵负鍒楄〃涓殑姣忎釜璁惧鎴栭┍鍔ㄨ皟鐢ㄥ洖璋冦€傛墍鏈夊垪琛ㄨ闂兘閫氳繃鑾峰彇鎬荤嚎鐨勯攣锛堢洰鍓嶄负璇婚攣锛夎繘琛屽悓姝ャ€傚湪璋冪敤鍥炶皟涔嬪墠锛屽垪琛ㄤ腑姣忎釜瀵硅薄鐨勫紩鐢ㄨ鏁颁細閫掑锛涘湪鑾峰彇涓嬩竴涓璞′箣鍚庝細閫掑噺銆傝皟鐢ㄥ洖璋冩椂涓嶆寔鏈夐攣銆?

#### sysfs

瀛樺湪涓€涓悕涓?'bus' 鐨勯《灞傜洰褰曘€?
姣忕鎬荤嚎鍦?bus 鐩綍涓嬮兘鏈変竴涓洰褰曪紝浠ュ強涓や釜榛樿
```

	/sys/bus/pci/
	|-- devices
	`-- drivers

```
鍦ㄦ€荤嚎涓婃敞鍐岀殑椹卞姩鍦ㄦ€荤嚎鐨?drivers 鐩綍涓嬭幏寰椾竴涓洰褰?```

	/sys/bus/pci/
	|-- devices
	`-- drivers
	    |-- Intel ICH
	    |-- Intel ICH Joystick
	    |-- agpgart
	    `-- e100

```
鍦ㄨ绫诲瀷鎬荤嚎涓婂彂鐜扮殑姣忎釜璁惧閮戒細鍦ㄦ€荤嚎鐨?devices 鐩綍涓嬭幏寰椾竴涓寚鍚戣璁惧鍦ㄧ墿鐞?```

	/sys/bus/pci/
	|-- devices
	|   |-- 00:00.0 -> ../../../root/pci0/00:00.0
	|   |-- 00:01.0 -> ../../../root/pci0/00:01.0
	|   `-- 00:02.0 -> ../../../root/pci0/00:02.0
	`-- drivers


```
#### 瀵煎嚭灞炴€?

```

  struct bus_attribute {
	struct attribute	attr;
	ssize_t (*show)(const struct bus_type *, char * buf);
	ssize_t (*store)(const struct bus_type *, const char * buf, size_t count);
  };

```
鎬荤嚎椹卞姩鍙互浣跨敤 BUS_ATTR_RW 瀹忓鍑哄睘鎬э紝鍏跺伐浣滄柟寮忕被浼间簬鐢ㄤ簬璁惧鐨?DEVICE_ATTR_RW 瀹忋€備緥濡傦紝
```

	static BUS_ATTR_RW(debug);

```
```

	static bus_attribute bus_attr_debug;

```
闅忓悗鍙皢鍏剁敤浜庡湪鎬荤嚎涓婃坊鍔犲拰鍒犻櫎灞炴€?```

	int bus_create_file(struct bus_type *, struct bus_attribute *);
	void bus_remove_file(struct bus_type *, struct bus_attribute *);

```
