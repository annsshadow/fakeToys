## Linux 纭欢鐩戞帶鍐呮牳 API


Guenter Roeck

### 绠€浠?

鏈枃妗ｆ弿杩颁簡甯屾湜浣跨敤纭欢鐩戞帶妗嗘灦鐨勭‖浠剁洃鎺ч┍鍔ㄦ墍鑳戒娇鐢ㄧ殑 API銆?
鏈枃妗ｅ苟涓嶆弿杩颁粈涔堟槸纭欢鐩戞帶锛坔wmon锛夐┍鍔ㄦ垨璁惧锛屼篃涓嶆弿杩扮敤鎴风┖闂村彲鐢ㄤ簬涓庣‖浠剁洃鎺ц澶?閫氫俊鐨?API銆傚鏋滀綘鎯崇煡閬撹繖浜涳紝璇烽槄璇讳互涓嬫枃浠讹細Documentation/hwmon/sysfs-interface.rst銆?
鍏充簬濡備綍缂栧啓鍜屾敼杩?hwmon 椹卞姩鐨勬洿澶氭寚鍗楋紝涔熻闃呰 Documentation/hwmon/submitting-patches.rst銆?
### API

姣忎釜纭欢鐩戞帶椹卞姩蹇呴』 `#include <linux/hwmon.h>`锛屽湪鏌愪簺鎯呭喌涓嬭繕瑕?`#include <linux/hwmon-sysfs.h>`銆?linux/hwmon.h 澹版槑浜嗕互涓嬪唴瀹癸細

```

  struct device *
  hwmon_device_register_with_info(struct device *dev,
				  const char *name, void *drvdata,
				  const struct hwmon_chip_info *info,
				  const struct attribute_group **extra_groups);

  struct device *
  devm_hwmon_device_register_with_info(struct device *dev,
				       const char *name,
				       void *drvdata,
				       const struct hwmon_chip_info *info,
				       const struct attribute_group **extra_groups);

  void hwmon_device_unregister(struct device *dev);

  char *hwmon_sanitize_name(const char *name);

  char *devm_hwmon_sanitize_name(struct device *dev, const char *name);

  void hwmon_lock(struct device *dev);
  void hwmon_unlock(struct device *dev);

```

hwmon_device_register_with_info 娉ㄥ唽涓€涓‖浠剁洃鎺ц澶囥€傚畠鍦ㄧ‖浠剁洃鎺ф牳蹇冧腑鍒涘缓鏍囧噯鐨?sysfs
灞炴€э紝璁╅┍鍔ㄤ笓娉ㄤ簬璇诲啓鑺墖锛岃€屼笉蹇呮搷蹇?sysfs 灞炴€с€傜埗璁惧鍙傛暟浠ュ強鑺墖鍙傛暟閮戒笉鑳戒负 NULL銆傚叾
鍙傛暟鍦ㄤ笅闈㈡洿璇︾粏鍦版弿杩般€?
devm_hwmon_device_register_with_info 涓?hwmon_device_register_with_info 绫讳技銆備笉杩囷紝瀹冩槸
璁惧鎵樼鐨勶紙device managed锛夛紝鎰忓懗鐫€纭欢鐩戞帶璁惧鏃犻渶鐢辩Щ闄ゅ嚱鏁版樉寮忕Щ闄ゃ€?
鎵€鏈夊叾浠栫‖浠剁洃鎺ц澶囨敞鍐屽嚱鏁伴兘宸插純鐢紝涓嶅緱鍦ㄦ柊椹卞姩涓娇鐢ㄣ€?
hwmon_device_unregister 娉ㄩ攢涓€涓凡娉ㄥ唽鐨勭‖浠剁洃鎺ц澶囥€傝鍑芥暟鐨勫弬鏁版槸鎸囧悜宸叉敞鍐岀‖浠剁洃鎺ц澶?缁撴瀯鐨勬寚閽堛€傚鏋滅‖浠剁洃鎺ц澶囨槸閫氳繃 hwmon_device_register_with_info 娉ㄥ唽鐨勶紝鍒欏繀椤讳粠椹卞姩鐨?remove 鍑芥暟涓皟鐢ㄦ鍑芥暟銆?
鎵€鏈夊彈鏀寔鐨?hwmon 璁惧娉ㄥ唽鍑芥暟鍙帴鍙楁湁鏁堢殑璁惧鍚嶇О銆傚寘鍚棤鏁堝瓧绗︼紙绌虹櫧銆乣*` 鎴?`-`锛夌殑璁惧
鍚嶇О灏嗚鎷掔粷銆傚鏋滀互 NULL 浣滀负 name 鍙傛暟浼犲叆锛岀‖浠剁洃鎺ц澶囧悕绉板皢浠庣埗璁惧鍚嶇О娲剧敓銆?
濡傛灉椹卞姩涓嶄娇鐢ㄩ潤鎬佽澶囧悕绉帮紙渚嬪瀹冧娇鐢?dev_name()锛夛紝鍥犳鏃犳硶纭繚鍚嶇О鍙寘鍚湁鏁堝瓧绗︼紝鍙互浣?鐢?hwmon_sanitize_name銆傛渚挎嵎鍑芥暟浼氬鍒跺瓧绗︿覆骞跺皢浠讳綍鏃犳晥瀛楃鏇挎崲涓轰笅鍒掔嚎銆傚畠浼氫负鏂板瓧绗︿覆
鍒嗛厤鍐呭瓨锛岃皟鐢ㄨ€呮湁璐ｄ换鍦ㄨ澶囩Щ闄ゆ椂閲婃斁璇ュ唴瀛樸€?
devm_hwmon_sanitize_name 鏄?hwmon_sanitize_name 鐨勮祫婧愭墭绠＄増鏈紱鍐呭瓨灏嗗湪璁惧绉婚櫎鏃惰嚜鍔ㄩ噴鏀俱€?
褰撲娇鐢?`[devm_]hwmon_device_register_with_info()` 娉ㄥ唽纭欢鐩戞帶璁惧鏃讹紝浣跨敤鐩稿叧璁块棶鍑芥暟鐨勮闂?鐢辩‖浠剁洃鎺ф牳蹇冧覆琛屽寲銆傚鏋滈┍鍔ㄩ渶瑕佷负鍏朵粬鍑芥暟锛堜緥濡備腑鏂鐞嗙▼搴忥紝鎴栧畬鍏ㄥ湪椹卞姩涓疄鐜扮殑灞炴€э級鍔犻攣锛?鍙互浣跨敤 hwmon_lock() 鍜?hwmon_unlock() 鏉ョ‘淇濆杩欎簺鍑芥暟鐨勮皟鐢ㄨ涓茶鍖栥€?
### 浣跨敤 devm_hwmon_device_register_with_info()

hwmon_device_register_with_info() 娉ㄥ唽涓€涓‖浠剁洃鎺ц澶囥€傝鍑芥暟鐨勫弬鏁版槸

=============================================== ===============================================
`struct device *dev`			鎸囧悜鐖惰澶囩殑鎸囬拡
`const char *name`			璁惧鍚嶇О
`void *drvdata`				椹卞姩绉佹湁鏁版嵁
`const struct hwmon_chip_info *info`	鎸囧悜鑺墖鎻忚堪鐨勬寚閽堛€?`const struct attribute_group **extra_groups` 	浠?NULL 缁撳熬鐨勯檮鍔犻潪鏍囧噯
						sysfs 灞炴€х粍鍒楄〃銆?=============================================== ===============================================

姝ゅ嚱鏁板湪鎴愬姛鏃惰繑鍥炴寚鍚戞墍鍒涘缓纭欢鐩戞帶璁惧鐨勬寚閽堬紝澶辫触鍒欒繑鍥炶礋鐨勯敊璇爜銆?
```

	struct hwmon_chip_info {
		const struct hwmon_ops *ops;
		const struct hwmon_channel_info * const *info;
	};

```

瀹冨寘鍚互涓嬪瓧娈碉細

- ops:
	鎸囧悜璁惧鎿嶄綔鐨勬寚閽堛€?- info:
	浠?NULL 缁撳熬鐨勮澶囬€氶亾鎻忚堪绗﹀垪琛ㄣ€?
```

  struct hwmon_ops {
	umode_t (*is_visible)(const void *, enum hwmon_sensor_types type,
			      u32 attr, int);
	int (*read)(struct device *, enum hwmon_sensor_types type,
		    u32 attr, int, long *);
	int (*write)(struct device *, enum hwmon_sensor_types type,
		     u32 attr, int, long);
  };

```

瀹冨畾涔変簡浠ヤ笅鎿嶄綔銆?
- is_visible:
    鎸囧悜涓€涓嚱鏁扮殑鎸囬拡锛岃繑鍥炴瘡涓彈鏀寔灞炴€х殑鏂囦欢妯″紡銆傛鍑芥暟鏄繀闇€鐨勩€?
- read:
    鎸囧悜涓€涓嚱鏁扮殑鎸囬拡锛岀敤浜庝粠鑺墖璇诲彇鍊笺€傛鍑芥暟鏄彲閫夌殑锛屼絾鑻ュ瓨鍦ㄤ换浣曞彲璇诲睘鎬у垯蹇呴』鎻愪緵銆?
- write:
    鎸囧悜涓€涓嚱鏁扮殑鎸囬拡锛岀敤浜庡悜鑺墖鍐欏叆鍊笺€傛鍑芥暟鏄彲閫夌殑锛屼絾鑻ュ瓨鍦ㄤ换浣曞彲鍐欏睘鎬у垯蹇呴』鎻愪緵銆?
姣忎釜浼犳劅鍣ㄩ€氶亾閮戒娇鐢?struct hwmon_channel_info 鎻忚堪锛屽嵆

```

	struct hwmon_channel_info {
		enum hwmon_sensor_types type;
		u32 *config;
	};

```

瀹冨寘鍚互涓嬪瓧娈碉細

- type:
    纭欢鐩戞帶浼犳劅鍣ㄧ被鍨嬨€?
    鍙楁敮鎸佺殑浼犳劅鍣ㄧ被鍨嬫湁

     ================== ==================================================
     hwmon_chip		涓€涓櫄鎷熶紶鎰熷櫒绫诲瀷锛岀敤浜庢弿杩颁笉缁戝畾鍒扮壒瀹氳緭鍏ユ垨杈撳嚭鐨勫睘鎬?     hwmon_temp		娓╁害浼犳劅鍣?     hwmon_in		鐢靛帇浼犳劅鍣?     hwmon_curr		鐢垫祦浼犳劅鍣?     hwmon_power	鍔熺巼浼犳劅鍣?     hwmon_energy	鑳介噺浼犳劅鍣?     hwmon_energy64	鑳介噺浼犳劅鍣紝浠?64 浣嶆湁绗﹀彿鍊兼姤鍛?     hwmon_humidity	婀垮害浼犳劅鍣?     hwmon_fan		椋庢墖杞€熶紶鎰熷櫒
     hwmon_pwm		PWM 鎺у埗
     ================== ==================================================

- config:
    鎸囧悜缁欏畾绫诲瀷鐨勬瘡涓紶鎰熷櫒鐨?0 缁撳熬鐨勯厤缃€煎垪琛ㄧ殑鎸囬拡銆傛瘡涓€兼槸鎸囩ず鍗曚釜浼犳劅鍣ㄦ墍鏀寔鐨勫睘鎬?    鐨勪綅鍊肩殑缁勫悎銆?
浣滀负涓€涓緥瀛愶紝杩欐槸 LM75 鍏煎浼犳劅鍣ㄨ姱鐗囩殑瀹屾暣鎻忚堪鏂囦欢銆傝鑺墖鍏锋湁鍗曚釜娓╁害浼犳劅鍣ㄣ€傞┍鍔ㄥ笇鏈?鍚戠儹瀛愮郴缁熸敞鍐岋紙HWMON_C_REGISTER_TZ锛夛紝骞朵笖鏀寔 update_interval 灞炴€э紙HWMON_C_UPDATE_INTERVAL锛夈€?璇ヨ姱鐗囨敮鎸佽鍙栨俯搴︼紙HWMON_T_INPUT锛夛紝瀹冩湁涓€涓渶楂樻俯搴﹀瘎瀛樺櫒锛圚WMON_T_MAX锛変互鍙婁竴涓渶楂樻俯搴﹁繜婊?瀵勫瓨鍣紙HWMON_T_MAX_HYST锛?
```

	static const u32 lm75_chip_config[] = {
		HWMON_C_REGISTER_TZ | HWMON_C_UPDATE_INTERVAL,
		0
	};

	static const struct hwmon_channel_info lm75_chip = {
		.type = hwmon_chip,
		.config = lm75_chip_config,
	};

	static const u32 lm75_temp_config[] = {
		HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MAX_HYST,
		0
	};

	static const struct hwmon_channel_info lm75_temp = {
		.type = hwmon_temp,
		.config = lm75_temp_config,
	};

	static const struct hwmon_channel_info * const lm75_info[] = {
		&lm75_chip,
		&lm75_temp,
		NULL
	};

	HWMON_CHANNEL_INFO() 瀹忓彲浠ヤ笖搴斿綋鍦ㄥ彲鑳芥椂浼樺厛浣跨敤銆?	鍊熷姪姝ゅ畯锛屼笂闈㈢殑绀轰緥鍙互绠€鍖栨垚

	static const struct hwmon_channel_info * const lm75_info[] = {
		HWMON_CHANNEL_INFO(chip,
				HWMON_C_REGISTER_TZ | HWMON_C_UPDATE_INTERVAL),
		HWMON_CHANNEL_INFO(temp,
				HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MAX_HYST),
		NULL
	};

	鍏朵綑鐨勫０鏄庡涓嬨€?
	static const struct hwmon_ops lm75_hwmon_ops = {
		.is_visible = lm75_is_visible,
		.read = lm75_read,
		.write = lm75_write,
	};

	static const struct hwmon_chip_info lm75_chip_info = {
		.ops = &lm75_hwmon_ops,
		.info = lm75_info,
	};

```

鎸囩ず鍚勪釜灞炴€ф敮鎸佺殑浣嶅€肩殑瀹屾暣鍒楄〃瀹氫箟鍦?include/linux/hwmon.h 涓€傚畾涔夊墠缂€濡備笅銆?
=============== =================================================
HWMON_C_xxxx	鑺墖灞炴€э紝涓?hwmon_chip 涓€璧蜂娇鐢ㄣ€?HWMON_T_xxxx	娓╁害灞炴€э紝涓?hwmon_temp 涓€璧蜂娇鐢ㄣ€?HWMON_I_xxxx	鐢靛帇灞炴€э紝涓?hwmon_in 涓€璧蜂娇鐢ㄣ€?HWMON_C_xxxx	鐢垫祦灞炴€э紝涓?hwmon_curr 涓€璧蜂娇鐢ㄣ€?		娉ㄦ剰姝ゅ墠缂€涓庤姱鐗囧睘鎬у墠缂€閲嶅彔銆?HWMON_P_xxxx	鍔熺巼灞炴€э紝涓?hwmon_power 涓€璧蜂娇鐢ㄣ€?HWMON_E_xxxx	鑳介噺灞炴€э紝涓?hwmon_energy 涓€璧蜂娇鐢ㄣ€?HWMON_H_xxxx	婀垮害灞炴€э紝涓?hwmon_humidity 涓€璧蜂娇鐢ㄣ€?HWMON_F_xxxx	椋庢墖杞€熷睘鎬э紝涓?hwmon_fan 涓€璧蜂娇鐢ㄣ€?HWMON_PWM_xxxx	PWM 鎺у埗灞炴€э紝涓?hwmon_pwm 涓€璧蜂娇鐢ㄣ€?=============== =================================================

### 椹卞姩鍥炶皟鍑芥暟


姣忎釜椹卞姩鎻愪緵 is_visible銆乺ead 鍜?write 鍑芥暟銆傚弬鏁?
```

  umode_t is_visible_func(const void *data, enum hwmon_sensor_types type,
			  u32 attr, int channel)

```

鍙傛暟锛?	data:
		鎸囧悜璁惧绉佹湁鏁版嵁缁撴瀯鐨勬寚閽堛€?	type:
		浼犳劅鍣ㄧ被鍨嬨€?	attr:
		涓庣壒瀹氬睘鎬у叧鑱旂殑灞炴€ф爣璇嗙銆?		渚嬪锛孒WMON_T_INPUT 鐨勫睘鎬у€煎皢鏄?hwmon_temp_input銆傚叧浜庝綅瀛楁鍒?		灞炴€у€肩殑瀹屾暣鏄犲皠锛岃鍙傞槄 include/linux/hwmon.h銆?	channel:
		浼犳劅鍣ㄩ€氶亾鍙枫€?
杩斿洖鍊硷細
	姝ゅ睘鎬х殑鏂囦欢妯″紡銆傞€氬父锛岃繖灏嗘槸 0锛堜笉浼氬垱寤鸿灞炴€э級銆?444 鎴?0644銆?
```

	int read_func(struct device *dev, enum hwmon_sensor_types type,
		      u32 attr, int channel, long *val)

```

鍙傛暟锛?	dev:
		鎸囧悜纭欢鐩戞帶璁惧鐨勬寚閽堛€?	type:
		浼犳劅鍣ㄧ被鍨嬨€?	attr:
		涓庣壒瀹氬睘鎬у叧鑱旂殑灞炴€ф爣璇嗙銆?		渚嬪锛孒WMON_T_INPUT 鐨勫睘鎬у€煎皢鏄?hwmon_temp_input銆傚畬鏁存槧灏勮鍙傞槄
		include/linux/hwmon.h銆?	channel:
		浼犳劅鍣ㄩ€氶亾鍙枫€?	val:
		鎸囧悜灞炴€у€肩殑鎸囬拡銆?		瀵逛簬 hwmon_energy64锛宍'val`' 浣滀负 `long *` 浼犲叆锛屼絾闇€瑕佺被鍨嬭浆鎹负 `s64 *`銆?
杩斿洖鍊硷細
	鎴愬姛涓?0锛屽惁鍒欎负璐熼敊璇彿銆?
```

	int write_func(struct device *dev, enum hwmon_sensor_types type,
		       u32 attr, int channel, long val)

```

鍙傛暟锛?	dev:
		鎸囧悜纭欢鐩戞帶璁惧鐨勬寚閽堛€?	type:
		浼犳劅鍣ㄧ被鍨嬨€?	attr:
		涓庣壒瀹氬睘鎬у叧鑱旂殑灞炴€ф爣璇嗙銆?		渚嬪锛孒WMON_T_INPUT 鐨勫睘鎬у€煎皢鏄?hwmon_temp_input銆傚畬鏁存槧灏勮鍙傞槄
		include/linux/hwmon.h銆?	channel:
		浼犳劅鍣ㄩ€氶亾鍙枫€?	val:
		瑕佸啓鍏ヨ姱鐗囩殑鍊笺€?
杩斿洖鍊硷細
	鎴愬姛涓?0锛屽惁鍒欎负璐熼敊璇彿銆?

### 椹卞姩鎻愪緵鐨?sysfs 灞炴€?

鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝椹卞姩涓嶉渶瑕佹彁渚?sysfs 灞炴€э紝鍥犱负纭欢鐩戞帶鏍稿績浼氬湪鍐呴儴鍒涘缓杩欎簺灞炴€с€傚彧闇€瑕佹彁渚?棰濆鐨勯潪鏍囧噯 sysfs 灞炴€с€?
澶存枃浠?linux/hwmon-sysfs.h 鎻愪緵浜嗕竴浜涙湁鐢ㄧ殑瀹忔潵澹版槑鍜屼娇鐢ㄧ‖浠剁洃鎺?sysfs 灞炴€с€?
鍦ㄨ澶氭儏鍐典笅锛屼綘鍙互浣跨敤鐜版湁鐨勫畾涔?DEVICE_ATTR 鎴栧叾鍙樹綋 DEVICE_ATTR_{RW,RO,WO} 鏉ュ０鏄庢绫?灞炴€с€傚鏋滀竴涓睘鎬ф病鏈夐澶栫殑涓婁笅鏂囷紝杩欐槸鍙鐨勩€傜劧鑰岋紝鍦ㄨ澶氭儏鍐典笅浼氭湁闄勫姞淇℃伅锛堜緥濡備紶鎰熷櫒绱㈠紩锛?闇€瑕佷紶閫掔粰 sysfs 灞炴€у鐞嗗嚱鏁般€?
SENSOR_DEVICE_ATTR 鍜?SENSOR_DEVICE_ATTR_2 鍙敤浜庡畾涔夐渶瑕佹绫婚檮鍔犱笂涓嬫枃淇℃伅鐨勫睘鎬с€?SENSOR_DEVICE_ATTR 闇€瑕佷竴涓檮鍔犲弬鏁帮紝SENSOR_DEVICE_ATTR_2 闇€瑕佷袱涓€?
濡傛灉鏍囧噯鐨勫睘鎬ф潈闄愬拰鍑芥暟鍚嶅彲琛岋紝搴斿綋浣跨敤 SENSOR_DEVICE_ATTR 鍜?SENSOR_DEVICE_ATTR_2 鐨勭畝鍖?鍙樹綋銆傛爣鍑嗘潈闄愪负锛歋ENSOR_DEVICE_ATTR[_2]_RW 涓?0644锛孲ENSOR_DEVICE_ATTR[_2]_RO 涓?0444锛?SENSOR_DEVICE_ATTR[_2]_WO 涓?0200銆傛爣鍑嗗嚱鏁扮被浼间簬 DEVICE_ATTR_{RW,RO,WO}锛屽湪鎵€鎻愪緵鐨勫嚱鏁板悕鍚?闄勫姞 _show 鍜?_store銆?
SENSOR_DEVICE_ATTR 鍙婂叾鍙樹綋瀹氫箟浜嗕竴涓?struct sensor_device_attribute

```

	struct sensor_device_attribute {
		struct device_attribute dev_attr;
		int index;
	};

```

浣犲彲浠ヤ娇鐢?to_sensor_dev_attr 浠庡睘鎬х殑璇绘垨鍐欏嚱鏁颁腑鑾峰彇鎸囧悜姝ょ粨鏋勭殑鎸囬拡銆傚叾鍙傛暟鏄灞炴€ф墍闄勫姞鐨?璁惧銆?
SENSOR_DEVICE_ATTR_2 鍙婂叾鍙樹綋瀹氫箟浜嗕竴涓?struct sensor_device_attribute_2

```

	struct sensor_device_attribute_2 {
		struct device_attribute dev_attr;
		u8 index;
		u8 nr;
	};

```

浣跨敤 to_sensor_dev_attr_2 鑾峰彇鎸囧悜姝ょ粨鏋勭殑鎸囬拡銆傚叾鍙傛暟鏄灞炴€ф墍闄勫姞鐨勮澶囥€?