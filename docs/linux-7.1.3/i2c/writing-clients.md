## 瀹炵幇 I2C 璁惧椹卞姩


杩欐槸涓€浠戒负 I2C 鎴?SMBus 璁惧缂栧啓鍐呮牳椹卞姩鐨勫皬鎸囧崡锛屼娇鐢?Linux 浣滀负鍗忚涓绘満/涓昏澶?锛坢aster锛岃€岄潪浠庤澶?slave锛夈€?
瑕佸缓绔嬩竴涓┍鍔紝浣犻渶瑕佸仛鑻ュ共浠朵簨銆傛湁浜涙槸鍙€夌殑锛屾湁浜涗簨鎯呭彲浠ョ敤鐣ユ湁涓嶅悓鎴栧畬鍏ㄤ笉鍚岀殑
鏂瑰紡瀹屾垚銆傝灏嗘湰鏂囦綔涓烘寚鍗楋紝鑰岄潪瑙勫垯鎵嬪唽锛?
## 鎬讳綋璇存槑


灏介噺淇濇寔鍐呮牳鍛藉悕绌洪棿灏藉彲鑳藉共鍑€銆傛渶濂界殑鍔炴硶鏄负鎵€鏈夊叏灞€绗﹀彿浣跨敤涓€涓敮涓€鐨勫墠缂€銆傝繖瀵?瀵煎嚭鐨勭鍙峰挨鍏堕噸瑕侊紝浣嗕负闈炲鍑虹殑绗﹀彿杩欐牱鍋氫篃鏄釜濂戒富鎰忋€傚湪鏈暀绋嬩腑鎴戜滑灏嗕娇鐢ㄥ墠缂€
`foo_`銆?
## 椹卞姩缁撴瀯


閫氬父锛屼綘浼氬疄鐜颁竴涓崟鐙殑椹卞姩缁撴瀯浣擄紝骞朵粠涓疄渚嬪寲鎵€鏈夊鎴风锛坈lient锛夈€傝璁颁綇锛岄┍鍔?缁撴瀯浣撳寘鍚€氱敤鐨勮闂緥绋嬶紝闄や簡浣犳彁渚涙暟鎹殑瀛楁澶栵紝搴斿仛闆跺垵濮嬪寲銆傚鎴风缁撴瀯浣撲繚瀛?璁惧鐗瑰畾鐨勪俊鎭紝濡傞┍鍔ㄦā鍨嬶紙driver model锛夎澶囪妭鐐瑰強鍏?I2C 鍦板潃銆?
```

  static const struct i2c_device_id foo_idtable[] = {
	{ "foo", my_id_for_foo },
	{ "bar", my_id_for_bar },
	{ }
  };
  MODULE_DEVICE_TABLE(i2c, foo_idtable);

  static struct i2c_driver foo_driver = {
	.driver = {
		.name	= "foo",
		.pm	= &foo_pm_ops,	/* 鍙€?*/
	},

	.id_table	= foo_idtable,
	.probe		= foo_probe,
	.remove		= foo_remove,

	.shutdown	= foo_shutdown,	/* 鍙€?*/
	.command	= foo_command,	/* 鍙€夛紝宸插簾寮?*/
  }

```

name 瀛楁鏄┍鍔ㄥ悕绉帮紝涓斾笉鑳藉寘鍚┖鏍笺€傚畠搴旇涓庢ā鍧楀悕绉板尮閰嶏紙濡傛灉椹卞姩鍙互缂栬瘧涓烘ā鍧楋級锛?灏界浣犲彲浠ヤ娇鐢?MODULE_ALIAS锛堟湰渚嬩腑浼犲叆鈥渇oo鈥濓級鏉ヤ负妯″潡娣诲姞鍙︿竴涓悕绉般€傚鏋滈┍鍔ㄥ悕绉?涓庢ā鍧楀悕绉颁笉鍖归厤锛屾ā鍧楀皢涓嶄細琚嚜鍔ㄥ姞杞斤紙鐑彃鎷?hotplug/鍐锋彃鎷旓級銆?
鎵€鏈夊叾浠栧瓧娈甸兘鏄洖璋冨嚱鏁帮紝灏嗗湪涓嬫枃璇存槑銆?
## 棰濆鐨勫鎴风鏁版嵁


姣忎釜瀹㈡埛绔粨鏋勪綋閮芥湁涓€涓壒娈婄殑 `data` 瀛楁锛屽彲浠ユ寚鍚戜换鎰忕粨鏋勪綋銆備綘搴旇鐢ㄥ畠鏉ヤ繚瀛?璁惧鐗瑰畾鐨勬暟鎹€?
```

	/* 瀛樺偍鍊?*/
	void i2c_set_clientdata(struct i2c_client *client, void *data);

	/* 鍙栧嚭鍊?*/
	void *i2c_get_clientdata(const struct i2c_client *client);

```

娉ㄦ剰锛屼粠鍐呮牳 2.6.34 璧凤紝浣犱笉鍐嶉渶瑕佸湪 remove() 涓垨 probe() 澶辫触鏃跺皢璇?`data` 瀛楁
璁句负 NULL銆俰2c-core 浼氬湪杩欎簺鎯呭喌涓嬭嚜鍔ㄥ畬鎴愩€傝繖浜涗篃鏄牳蹇冨敮涓€浼氳Е纰拌瀛楁鐨勬椂鏈恒€?
## 璁块棶瀹㈡埛绔?

鍋囪鎴戜滑鏈変竴涓湁鏁堢殑瀹㈡埛绔粨鏋勪綋銆傚湪鏌愪簺鏃跺埢锛屾垜浠渶瑕佷粠璇ュ鎴风鏀堕泦淇℃伅锛屾垨鍚?瀹㈡埛绔啓鍏ユ柊淇℃伅銆?
鎴戝彂鐜颁负姝ゅ畾涔?foo_read 鍜?foo_write 鍑芥暟寰堟湁鐢ㄣ€傚湪鏌愪簺鎯呭喌涓嬶紝鐩存帴璋冪敤 I2C 鍑芥暟
浼氭洿绠€鍗曪紝浣嗚澶氳姱鐗囬兘鏈夋煇绉嶅瘎瀛樺櫒-鍊肩殑鎶借薄锛屽彲浠ヨ交鏉惧皝瑁呫€?
涓嬮潰鐨勫嚱鏁版槸绠€鍗曠ず渚嬶紝涓嶅簲鐩存帴鐓ф惉
```

  int foo_read_value(struct i2c_client *client, u8 reg)
  {
	if (reg < 0x10)	/* 瀛楄妭澶у皬鐨勫瘎瀛樺櫒 */
		return i2c_smbus_read_byte_data(client, reg);
	else		/* 瀛楀ぇ灏忕殑瀵勫瓨鍣?*/
		return i2c_smbus_read_word_data(client, reg);
  }

  int foo_write_value(struct i2c_client *client, u8 reg, u16 value)
  {
	if (reg == 0x10)	/* 涓嶅彲鍐?- 椹卞姩閿欒锛?*/
		return -EINVAL;
	else if (reg < 0x10)	/* 瀛楄妭澶у皬鐨勫瘎瀛樺櫒 */
		return i2c_smbus_write_byte_data(client, reg, value);
	else			/* 瀛楀ぇ灏忕殑瀵勫瓨鍣?*/
		return i2c_smbus_write_word_data(client, reg, value);
  }


```

## 鎺㈡祴涓庢寕鎺?

Linux I2C 鍗忚鏍堟渶鍒濇槸涓鸿闂?PC 涓绘澘涓婄殑纭欢鐩戞帶鑺墖鑰岀紪鍐欑殑锛屽洜姝ゆ浘鍐呭祵涓€浜涙洿
閫傜敤浜?SMBus锛堜互鍙?PC锛夎€岄潪 I2C 鐨勫亣璁俱€傚叾涓竴涓亣璁炬槸澶у鏁伴€傞厤鍣ㄥ拰璁惧椹卞姩鏀寔
SMBUS_QUICK 鍗忚鏉ユ帰娴嬭澶囨槸鍚﹀瓨鍦ㄣ€傚彟涓€涓亣璁炬槸锛屼粎浣跨敤姝ょ被鎺㈡祴鍘熻灏辫冻浠ュ厖鍒嗛厤缃?璁惧鍜屽畠浠殑椹卞姩銆?
闅忕潃 Linux 鍙婂叾 I2C 鍗忚鏍堝湪宓屽叆寮忕郴缁熶互鍙?DVB 閫傞厤鍣ㄧ瓑澶嶆潅缁勪欢涓緱鍒颁簡鏇村箍娉涚殑浣跨敤锛?杩欎簺鍋囪鍙樺緱闂鏇村ぇ銆傚彂鍑轰腑鏂殑 I2C 璁惧椹卞姩闇€瑕佹洿澶氾紙涓斾笉鍚岋級鐨勯厤缃俊鎭紱鏃犳硶
閫氳繃鍗忚鎺㈡祴鍖哄垎鐨勮姱鐗囧彉浣擄紝鎴栭渶瑕佹煇浜涙澘绾х壒瀹氫俊鎭墠鑳芥纭繍琛岀殑鑺墖锛屽叾椹卞姩涔熸槸濡傛銆?
### 璁惧/椹卞姩缁戝畾


绯荤粺鍩虹璁炬柦锛堥€氬父鏄澘绾х壒瀹氱殑鍒濆鍖栦唬鐮佹垨寮曞鍥轰欢锛変細鎶ュ憡瀛樺湪鍝簺 I2C 璁惧銆備緥濡傦紝
鍦ㄥ唴鏍告垨寮曞鍔犺浇绋嬪簭涓彲鑳芥湁涓€寮犺〃锛屾爣璇?I2C 璁惧骞跺皢瀹冧滑涓庢湁鍏?IRQ 鍙婂叾浠栬繛绾夸俊鎭€?鑺墖绫诲瀷绛夌殑鏉跨骇鐗瑰畾閰嶇疆鐩稿叧鑱斻€傝繖鍙敤浜庝负姣忎釜 I2C 璁惧鍒涘缓 i2c_client 瀵硅薄銆?
浣跨敤杩欑缁戝畾妯″瀷鐨?I2C 璁惧椹卞姩涓?Linux 涓换浣曞叾浠栫被鍨嬬殑椹卞姩宸ヤ綔鏂瑰紡涓€鏍凤細瀹冧滑鎻愪緵
涓€涓?probe() 鏂规硶浠ョ粦瀹氬埌杩欎簺璁惧锛屼互鍙婁竴涓?remove() 鏂规硶浠ヨВ缁戙€?
```

	static int foo_probe(struct i2c_client *client);
	static void foo_remove(struct i2c_client *client);

```

璇疯浣忥紝i2c_driver 骞朵笉浼氬垱寤洪偅浜涘鎴风鍙ユ焺銆傝鍙ユ焺鍙兘鍦?foo_probe() 鏈熼棿琚娇鐢ㄣ€?濡傛灉 foo_probe() 鎶ュ憡鎴愬姛锛堥浂鑰岄潪璐熺殑鐘舵€佺爜锛夛紝瀹冨彲浠ヤ繚瀛樿鍙ユ焺骞跺湪 foo_remove() 杩斿洖
鍓嶄竴鐩翠娇鐢ㄣ€傚ぇ澶氭暟 Linux 椹卞姩閮戒娇鐢ㄨ繖绉嶇粦瀹氭ā鍨嬨€?
褰?id_table 鐨?name 瀛楁涓庤澶囧悕绉板尮閰嶆椂锛屼細璋冪敤 probe 鍑芥暟銆傚鏋?probe 鍑芥暟闇€瑕佽
鏉＄洰锛屽畠鍙互浣跨敤浠ヤ笅鏂瑰紡鑾峰彇

```

	const struct i2c_device_id *id = i2c_match_id(foo_idtable, client);


```

### 璁惧鍒涘缓


濡傛灉浣犵‘鍒囩煡閬撴煇涓?I2C 璁惧杩炴帴鍒颁簡缁欏畾鐨?I2C 鎬荤嚎涓婏紝浣犲彲浠ラ€氳繃绠€鍗曞湴濉厖涓€涓?i2c_board_info 缁撴瀯浣擄紙鍖呭惈璁惧鍦板潃鍜岄┍鍔ㄥ悕绉帮級骞惰皟鐢?i2c_new_client_device() 鏉?瀹炰緥鍖栬璁惧銆傝繖灏嗗垱寤鸿澶囷紝鐒跺悗椹卞姩鏍稿績浼氳礋璐ｆ壘鍒版纭殑椹卞姩骞惰皟鐢ㄥ叾 probe() 鏂规硶銆?濡傛灉椹卞姩鏀寔涓嶅悓鐨勮澶囩被鍨嬶紝浣犲彲浠ヤ娇鐢?type 瀛楁鎸囧畾浣犳兂瑕佺殑绫诲瀷銆傚鏋滈渶瑕侊紝浣犺繕鍙互
鎸囧畾涓€涓?IRQ 鍜屽钩鍙版暟鎹紙platform data锛夈€?
鏈夋椂浣犵煡閬撴煇璁惧杩炴帴鍒颁簡缁欏畾鐨?I2C 鎬荤嚎锛屼絾涓嶇煡閬撳畠浣跨敤鐨勭‘鍒囧湴鍧€銆備緥濡?TV 閫傞厤鍣?灏卞瓨鍦ㄨ繖绉嶆儏鍐碉細鍚屼竴涓┍鍔ㄦ敮鎸佸嚑鍗佺鐣ユ湁涓嶅悓鐨勫瀷鍙凤紝鑰?I2C 璁惧鍦板潃鍦ㄤ笉鍚屽瀷鍙烽棿浼氬彉鍖栥€?鍦ㄨ繖绉嶆儏鍐典笅锛屼綘鍙互浣跨敤 i2c_new_scanned_device() 鍙樹綋锛屽畠涓?i2c_new_client_device()
绫讳技锛屽彧鏄畠棰濆鎺ュ彈涓€涓渶瑕佹帰娴嬬殑鍙兘鐨?I2C 鍦板潃鍒楄〃銆備細涓哄垪琛ㄤ腑绗竴涓湁鍝嶅簲鐨勫湴鍧€
鍒涘缓璁惧銆傚鏋滀綘鏈熸湜鍦ㄨ鍦板潃鑼冨洿鍐呭瓨鍦ㄥ涓澶囷紝鍙渶澶氭璋冪敤 i2c_new_scanned_device() 鍗冲彲銆?
瀵?i2c_new_client_device() 鎴?i2c_new_scanned_device() 鐨勮皟鐢ㄩ€氬父鍙戠敓鍦?I2C 鎬荤嚎
椹卞姩涓€備綘鍙兘鎯充繚瀛樿繑鍥炵殑 i2c_client 寮曠敤浠ヤ究鍚庣画浣跨敤銆?
### 璁惧鎺㈡祴


璁惧鎺㈡祴鏈哄埗鏈変竴浜涚己鐐广€備綘闇€瑕佹煇绉嶅彲闈犵殑鏂瑰紡鏉ヨ瘑鍒彈鏀寔鐨勮澶囷紙閫氬父浣跨敤璁惧鐗瑰畾鐨勩€?涓撶敤鐨勮瘑鍒瘎瀛樺櫒锛夛紝鍚﹀垯寰堝彲鑳藉彂鐢熻鎺紝浜嬫儏浼氬緢蹇彉绯熴€傝璁颁綇锛孖2C 鍗忚涓嶅寘鍚换浣?妫€娴嬬粰瀹氬湴鍧€涓婃槸鍚﹀瓨鍦ㄨ姱鐗囩殑鏍囧噯鏂规硶锛屾洿涓嶇敤璇磋瘑鍒澶囩殑鏍囧噯鏂规硶浜嗐€傛洿绯熺殑鏄€荤嚎
浼犺緭缂哄皯璇箟鍏宠仈锛岃繖鎰忓懗鐫€鍚屼竴涓紶杈撳彲鑳借涓€涓姱鐗囪涓鸿鎿嶄綔锛岃€岃鍙︿竴涓姱鐗囪涓哄啓鎿嶄綔銆?鍑轰簬杩欎簺鍘熷洜锛岃澶囨帰娴嬭瑙嗕负涓€绉嶉仐鐣欐満鍒讹紝涓嶅簲鍦ㄦ柊浠ｇ爜涓娇鐢ㄣ€?
### 璁惧鍒犻櫎


姣忎釜浣跨敤 i2c_new_client_device() 鎴?i2c_new_scanned_device() 鍒涘缓鐨?I2C 璁惧锛岄兘鍙互
閫氳繃璋冪敤 i2c_unregister_device() 鏉ユ敞閿€銆傚鏋滀綘涓嶆樉寮忚皟鐢ㄥ畠锛屽畠浼氬湪搴曞眰 I2C 鎬荤嚎鑷韩
琚Щ闄や箣鍓嶈嚜鍔ㄨ皟鐢紝鍥犱负璁惧鏃犳硶鍦ㄩ┍鍔ㄦā鍨嬩腑瀛樻椿浜庡叾鐖惰澶囦箣鍚庛€?
## 鍒濆鍖栭┍鍔?

褰撳唴鏍稿惎鍔紝鎴栧綋浣犵殑 foo 椹卞姩妯″潡琚彃鍏ユ椂锛屼綘蹇呴』鍋氫竴浜涘垵濮嬪寲宸ヤ綔銆傚垢杩愮殑鏄紝閫氬父鍙?闇€娉ㄥ唽椹卞姩妯″潡灏辫冻澶熶簡銆?
```

  static int __init foo_init(void)
  {
	return i2c_add_driver(&foo_driver);
  }
  module_init(foo_init);

  static void __exit foo_cleanup(void)
  {
	i2c_del_driver(&foo_driver);
  }
  module_exit(foo_cleanup);

  module_i2c_driver() 瀹忓彲鐢ㄤ簬绮剧畝涓婅堪浠ｇ爜銆?
  module_i2c_driver(foo_driver);

```

娉ㄦ剰锛屾煇浜涘嚱鏁拌鏍囪涓?`__init`銆傝繖浜涘嚱鏁板彲浠ュ湪鍐呮牳鍚姩锛堟垨妯″潡鍔犺浇锛夊畬鎴愬悗琚Щ闄ゃ€?鍚屾牱锛屾爣璁颁负 `__exit` 鐨勫嚱鏁板湪浠ｇ爜琚瀯寤鸿繘鍐呮牳鏃朵細琚紪璇戝櫒涓㈠純锛屽洜涓哄畠浠案杩滀笉浼氳
璋冪敤銆?
## 椹卞姩淇℃伅


```

  /* 鏇挎崲涓轰綘鑷繁濮撳悕鍜岄偖绠卞湴鍧€ */
  MODULE_AUTHOR("Frodo Looijaard <frodol@dds.nl>"
  MODULE_DESCRIPTION("Driver for Barf Inc. Foo I2C devices");

  /* 涔熷厑璁稿皯鏁伴潪 GPL 璁稿彲璇佺被鍨?*/
  MODULE_LICENSE("GPL");


```

## 鐢垫簮绠＄悊


濡傛灉浣犵殑 I2C 璁惧鍦ㄨ繘鍏ョ郴缁熶綆鍔熻€楃姸鎬佹椂鈥斺€斾緥濡傚皢鏀跺彂鍣ㄧ疆浜庝綆鍔熻€楁ā寮忥紝鎴栨縺娲荤郴缁?鍞ら啋鏈哄埗鈥斺€旈渶瑕佺壒娈婂鐞嗭紝璇烽€氳繃涓洪┍鍔ㄧ殑 dev_pm_ops 瀹炵幇鐩稿簲鐨勫洖璋冿紙濡?suspend 鍜?resume锛夋潵瀹屾垚銆?
杩欎簺鏄爣鍑嗙殑椹卞姩妯″瀷璋冪敤锛屽叾宸ヤ綔鏂瑰紡涓庝换浣曞叾浠栭┍鍔ㄥ崗璁爤涓€鏍枫€傝繖浜涜皟鐢ㄥ彲浠ョ潯鐪狅紝骞朵笖
鍙互浣跨敤 I2C 娑堟伅涓庤鎸傝捣鎴栨仮澶嶇殑璁惧鐨?I2C 娑堟伅锛堝洜涓哄畠浠殑鐖?I2C 閫傞厤鍣ㄥ湪杩欎簺璋冪敤
鍙戝嚭鏃舵槸娲诲姩鐨勶紝涓?IRQ 浠嶇劧鍚敤锛夈€?
## 绯荤粺鍏虫満


濡傛灉浣犵殑 I2C 璁惧鍦ㄧ郴缁熷叧鏈烘垨閲嶅惎锛堝寘鎷?kexec锛夋椂闇€瑕佺壒娈婂鐞嗏€斺€斾緥濡傚叧闂煇浜涗笢瑗库€斺€?璇蜂娇鐢?shutdown() 鏂规硶銆?
鍚屾牱锛岃繖鏄竴涓爣鍑嗙殑椹卞姩妯″瀷璋冪敤锛屽伐浣滄柟寮忎笌鍏朵粬浠讳綍椹卞姩鍗忚鏍堜竴鏍凤細杩欎簺璋冪敤鍙互鐫＄湢锛?骞朵笖鍙互浣跨敤 I2C 娑堟伅銆?
## 鍛戒护鍑芥暟


鏀寔涓€涓被浼?ioctl 鐨勯€氱敤鍥炶皟鍑芥暟銆備綘寰堝皯闇€瑕佸畠锛岃€屼笖瀹冪殑浣跨敤宸茶搴熷純锛屽洜姝ゆ柊鐨勮璁?涓嶅簲浣跨敤瀹冦€?
## 鍙戦€佷笌鎺ユ敹


濡傛灉浣犳兂涓庤澶囬€氫俊锛屾湁鍑犱釜鍑芥暟鍙互鍋氬埌銆備綘鍙互鍦?<linux/i2c.h> 涓壘鍒板畠浠叏閮ㄣ€?
濡傛灉浣犲彲浠ュ湪鏅€?I2C 閫氫俊鍜?SMBus 绾у埆閫氫俊涔嬮棿閫夋嫨锛岃浣跨敤鍚庤€呫€傛墍鏈夐€傞厤鍣ㄩ兘鐞嗚В
SMBus 绾у埆鍛戒护锛屼絾鍙湁閮ㄥ垎鐞嗚В鏅€?I2C锛?
### 鏅€?I2C 閫氫俊


```

	int i2c_master_send(struct i2c_client *client, const char *buf,
			    int count);
	int i2c_master_recv(struct i2c_client *client, char *buf, int count);

```

杩欎簺渚嬬▼浠庡鎴风璇诲彇鎴栧悜瀹㈡埛绔啓鍏ヤ竴浜涘瓧鑺傘€傚鎴风鍖呭惈 I2C 鍦板潃锛屽洜姝や綘涓嶅繀鍖呭惈瀹冦€?绗簩涓弬鏁板寘鍚璇?鍐欑殑瀛楄妭锛岀涓変釜鏄璇?鍐欑殑瀛楄妭鏁帮紙蹇呴』灏忎簬缂撳啿鍖洪暱搴︼紝涓斾篃搴斿皬浜?64k锛屽洜涓?msg.len 鏄?u16銆傦級杩斿洖鐨勬槸瀹為檯璇?鍐欑殑瀛楄妭鏁般€?
```

	int i2c_transfer(struct i2c_adapter *adap, struct i2c_msg *msg,
			 int num);

```

杩欎細鍙戦€佷竴绯诲垪娑堟伅銆傛瘡鏉℃秷鎭彲浠ユ槸璇绘垨鍐欙紝骞朵笖鍙互浠ヤ换鎰忔柟寮忔贩鍚堛€傝繖浜涗簨鍔¤鍚堝苟锛?浜嬪姟涔嬮棿涓嶅彂鍑哄仠姝紙stop锛夋潯浠躲€俰2c_msg 缁撴瀯浣撳姣忎釜娑堟伅鍖呭惈瀹㈡埛绔湴鍧€銆佹秷鎭殑瀛楄妭鏁?浠ュ強娑堟伅鏁版嵁鏈韩銆?
浣犲彲浠ラ槄璇?i2c-protocol.rst 鏂囦欢浠ヤ簡瑙ｅ叧浜庡疄闄?I2C 鍗忚鐨勬洿澶氫俊鎭€?
### SMBus 閫氫俊


```

	s32 i2c_smbus_xfer(struct i2c_adapter *adapter, u16 addr,
			   unsigned short flags, char read_write, u8 command,
			   int size, union i2c_smbus_data *data);

```

杩欐槸閫氱敤鐨?SMBus 鍑芥暟銆備笅闈㈡墍鏈夊嚱鏁伴兘鍩轰簬瀹冨疄鐜般€傜粷涓嶈鐩存帴浣跨敤杩欎釜鍑芥暟锛?
```

	s32 i2c_smbus_read_byte(struct i2c_client *client);
	s32 i2c_smbus_write_byte(struct i2c_client *client, u8 value);
	s32 i2c_smbus_read_byte_data(struct i2c_client *client, u8 command);
	s32 i2c_smbus_write_byte_data(struct i2c_client *client,
				      u8 command, u8 value);
	s32 i2c_smbus_read_word_data(struct i2c_client *client, u8 command);
	s32 i2c_smbus_write_word_data(struct i2c_client *client,
				      u8 command, u16 value);
	s32 i2c_smbus_read_block_data(struct i2c_client *client,
				      u8 command, u8 *values);
	s32 i2c_smbus_write_block_data(struct i2c_client *client,
				       u8 command, u8 length, const u8 *values);
	s32 i2c_smbus_read_i2c_block_data(struct i2c_client *client,
					  u8 command, u8 length, u8 *values);
	s32 i2c_smbus_write_i2c_block_data(struct i2c_client *client,
					   u8 command, u8 length,
					   const u8 *values);

```

杩欎簺鍑芥暟鏇惧洜鏃犱汉浣跨敤鑰屼粠 i2c-core 涓Щ闄わ紝浣嗗彲鑳?```

	s32 i2c_smbus_write_quick(struct i2c_client *client, u8 value);
	s32 i2c_smbus_process_call(struct i2c_client *client,
				   u8 command, u16 value);
	s32 i2c_smbus_block_process_call(struct i2c_client *client,
					 u8 command, u8 length, u8 *values);

```

鎵€鏈夎繖浜涗簨鍔″湪澶辫触鏃惰繑鍥炶礋鐨?errno 鍊笺€傗€渨rite鈥濅簨鍔℃垚鍔熸椂杩斿洖 0锛涒€渞ead鈥濅簨鍔¤繑鍥炶鍙?鐨勫€硷紝浣嗗潡锛坆lock锛変簨鍔￠櫎澶栤€斺€斿畠浠繑鍥炶鍙栫殑鍊肩殑鏁伴噺銆傚潡缂撳啿鍖轰笉蹇呴暱浜?32 瀛楄妭銆?
浣犲彲浠ラ槄璇?smbus-protocol.rst 鏂囦欢浠ヤ簡瑙ｅ叧浜庡疄闄?SMBus 鍗忚鐨勬洿澶氫俊鎭€?
## 閫氱敤渚嬬▼


涓嬮潰鍒楀嚭浜嗘墍鏈夋湭琚彁鍙婄殑閫氱敤渚嬬▼
```

	/* 杩斿洖鐗瑰畾閫傞厤鍣ㄧ殑閫傞厤鍣ㄧ紪鍙?*/
	int i2c_adapter_id(struct i2c_adapter *adap);

```
