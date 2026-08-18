
### V4L2 瀛愯澶囷紙sub-devices锛?


璁稿椹卞姩闇€瑕佷笌鍏跺瓙璁惧锛坰ub-devices锛夐€氫俊銆傝繖浜涜澶囧彲浠ュ畬鎴愬悇绉?
浠诲姟锛屼絾鏈€甯歌鐨勬槸澶勭悊闊抽鍜?鎴栬棰戠殑澶嶇敤锛坢uxing锛夈€?
缂栫爜鎴栬В鐮併€傚浜庣綉缁滄憚鍍忓ご锛屽父瑙佺殑瀛愯澶囨槸浼犳劅鍣ㄥ拰鎽勫儚澶?
鎺у埗鍣ㄣ€?

閫氬父杩欎簺鏄?I2C 璁惧锛屼絾涓嶄竴瀹氥€備负浜嗙粰椹卞姩鎻愪緵涓€涓竴鑷寸殑鎺ュ彛鏉ヨ闂繖浜涘瓙璁惧锛?
鍒涘缓浜?
`v4l2_subdev` 缁撴瀯浣擄紙v4l2-subdev.h锛夈€?

姣忎釜瀛愯澶囬┍鍔ㄩ兘蹇呴』鏈変竴涓?`v4l2_subdev` 缁撴瀯浣撱€傚浜庣畝鍗曠殑瀛愯澶囷紝璇ョ粨鏋勪綋
鍙互鐙珛瀛樺湪锛屾垨鑰咃紝濡傛灉闇€瑕佸瓨鍌ㄦ洿澶氱姸鎬佷俊鎭紝瀹冨彲鑳借宓屽叆鍒颁竴涓洿澶х殑
缁撴瀯浣撲腑銆傞€氬父浼氭湁涓€涓?
搴曞眰璁惧缁撴瀯浣擄紙渚嬪 `i2c_client`锛夛紝鍏朵腑鍖呭惈鐢卞唴鏍歌缃殑璁惧鏁版嵁銆傚缓璁?
浣跨敤 `v4l2_set_subdevdata` 灏嗚鎸囬拡瀛樺偍鍦?`v4l2_subdev` 鐨勭鏈夋暟鎹腑銆傝繖鏍?
鍙互鏂逛究鍦颁粠 `v4l2_subdev` 鎵惧埌瀹為檯鐨勫簳灞傛€荤嚎鐩稿叧
璁惧鏁版嵁銆?


浣犺繕闇€瑕佷竴绉嶄粠搴曞眰缁撴瀯浣撳埌 `v4l2_subdev` 鐨勬柟娉曘€?
瀵逛簬甯歌鐨?i2c_client 缁撴瀯浣擄紝浣跨敤 i2c_set_clientdata() 璋冪敤鏉ュ瓨鍌?
涓€涓?`v4l2_subdev` 鎸囬拡锛涘浜庡叾浠栨€荤嚎锛屼綘鍙兘蹇呴』浣跨敤鍏朵粬鏂规硶銆?


妗ワ紙bridge锛夐┍鍔ㄥ彲鑳借繕闇€瑕佸瓨鍌ㄦ瘡涓瓙璁惧鐨勭鏈夋暟鎹紝渚嬪鎸囧悜
妗ョ浉鍏崇殑姣忓瓙璁惧绉佹湁鏁版嵁鐨勬寚閽堛€俙v4l2_subdev` 缁撴瀯浣撲负姝ゆ彁渚涗簡
涓绘満绉佹湁鏁版嵁锛屽彲閫氳繃
`v4l2_get_subdev_hostdata` 鍜?`v4l2_set_subdev_hostdata` 璁块棶銆?

浠庢ˉ椹卞姩鐨勮瑙掓潵鐪嬶紝浣犲姞杞藉瓙璁惧妯″潡骞朵互鏌愮鏂瑰紡
鑾峰彇 `v4l2_subdev` 鎸囬拡銆傚浜?i2c 璁惧杩欏緢瀹规槗锛氫綘璋冪敤
`i2c_get_clientdata()`銆傚浜庡叾浠栨€荤嚎鍒欓渶瑕佸仛绫讳技鐨勬搷浣溿€?
瀵逛簬 I2C 鎬荤嚎涓婄殑瀛愯澶囷紝瀛樺湪杈呭姪鍑芥暟涓轰綘瀹屾垚澶ч儴鍒嗘绫?
妫樻墜鐨勫伐浣溿€?

姣忎釜 `v4l2_subdev` 閮藉寘鍚瓙璁惧椹卞姩鍙互瀹炵幇鐨?
鍑芥暟鎸囬拡锛堝鏋滀笉閫傜敤鍙互淇濈暀涓?`NULL`锛夈€傜敱浜庡瓙璁惧鍙互鍋氬緢澶氫笉鍚岀殑浜嬫儏锛?
鑰屼綘鍙堜笉甯屾湜鏈€缁堝緱鍒颁竴涓簽澶х殑 ops 缁撴瀯浣撱€佸叾涓彧鏈夊皯鏁?ops 琚櫘閬嶅疄鐜帮紝鍥犳
鍑芥暟鎸囬拡鎸夌被鍒垎绫伙紝姣忎釜绫诲埆鏈夎嚜宸辩嫭绔嬬殑 ops 缁撴瀯浣撱€?


椤跺眰鐨?ops 缁撴瀯浣撳寘鍚寚鍚戝悇绫诲埆 ops 缁撴瀯浣撶殑鎸囬拡锛屽鏋滃瓙璁惧椹卞姩
涓嶆敮鎸佽绫诲埆涓殑浠讳綍鍔熻兘锛屽垯鍙互涓?NULL銆?

鍏剁粨鏋勫涓嬫墍绀猴細


	struct v4l2_subdev_core_ops {
		int (**log_status)(struct v4l2_subdev **sd);
		int (**init)(struct v4l2_subdev **sd, u32 val);
		...
	};

	struct v4l2_subdev_tuner_ops {
		...
	};

	struct v4l2_subdev_audio_ops {
		...
	};

	struct v4l2_subdev_video_ops {
		...
	};

	struct v4l2_subdev_pad_ops {
		...
	};

	struct v4l2_subdev_ops {
		const struct v4l2_subdev_core_ops  *core;
		const struct v4l2_subdev_tuner_ops *tuner;
		const struct v4l2_subdev_audio_ops *audio;
		const struct v4l2_subdev_video_ops *video;
		const struct v4l2_subdev_pad_ops *video;
	};

鏍稿績锛坈ore锛塷ps 鏄墍鏈夊瓙璁惧鍏辨湁鐨勶紝鍏朵粬绫诲埆鍒欐牴鎹瓙璁惧鐨勪笉鍚屽垎鍒疄鐜般€備緥濡傦紝
瑙嗛璁惧涓嶅お鍙兘鏀寔 audio ops锛屽弽涔嬩害鐒躲€?


杩欑缁勭粐鏂瑰紡鍦ㄩ檺鍒跺嚱鏁版寚閽堟暟閲忕殑鍚屾椂锛屼粛鐒朵究浜?
娣诲姞鏂扮殑 ops 鍜岀被鍒€?

瀛愯澶囬┍鍔ㄤ娇鐢ㄤ互涓嬫柟寮忓垵濮嬪寲 `v4l2_subdev` 缁撴瀯浣擄細

	`v4l2_subdev_init <v4l2_subdev_init>`
	(`sd <v4l2_subdev>`, &\ `ops <v4l2_subdev_ops>`).


涔嬪悗锛屼綘闇€瑕佺敤鍞竴鐨勫悕绉板垵濮嬪寲 `sd <v4l2_subdev>`->name 骞惰缃ā鍧楁墍鏈夎€呫€?
濡傛灉浣犱娇鐢?i2c 杈呭姪鍑芥暟锛岃繖浜涗細涓轰綘鑷姩瀹屾垚銆?


濡傛灉闇€瑕佷笌 media 妗嗘灦闆嗘垚锛屼綘蹇呴』閫氳繃璋冪敤 `media_entity_pads_init` 鏉ュ垵濮嬪寲
宓屽叆鍦?`v4l2_subdev` 缁撴瀯浣撲腑鐨?`media_entity` 缁撴瀯浣擄紙entity 瀛楁锛夛紝
鍓嶆彁鏄瀹炰綋鍏锋湁
pads锛?


	struct media_pad *pads = &my_sd->pads;
	int err;

	err = media_entity_pads_init(&sd->entity, npads, pads);

pads 鏁扮粍蹇呴』浜嬪厛鍒濆鍖栥€傛棤闇€鎵嬪姩璁剧疆 struct media_entity 鐨?function 鍜?name 瀛楁锛?
浣嗗鏈夐渶瑕侊紝蹇呴』鍒濆鍖?revision 瀛楁銆?


褰撳瓙璁惧鑺傜偣锛堝鏋滄湁锛夎鎵撳紑/鍏抽棴鏃讹紝瀵硅瀹炰綋鐨勫紩鐢ㄤ細琚嚜鍔?
鑾峰彇/閲婃斁銆?

鍦ㄥ瓙璁惧琚攢姣佷箣鍓嶏紝涓嶈蹇樿娓呯悊 media 瀹炰綋锛?


	media_entity_cleanup(&sd->entity);

濡傛灉瀛愯澶囬┍鍔ㄥ疄鐜颁簡 sink pads锛屽瓙璁惧椹卞姩鍙互璁剧疆 `v4l2_subdev_pad_ops` 涓殑
link_validate 瀛楁锛屼互鎻愪緵鍏惰嚜宸辩殑閾捐矾
楠岃瘉鍑芥暟銆傚浜庣绾夸腑鐨勬瘡鏉￠摼璺紝閮戒細璋冪敤璇ラ摼璺?sink 绔殑 link_validate pad
鎿嶄綔銆傚湪杩欎袱绉嶆儏鍐典笅锛岄┍鍔ㄤ粛鐒惰礋璐ｉ獙璇佸瓙璁惧涓庤棰戣妭鐐逛箣闂?
鏍煎紡閰嶇疆鐨勬纭€с€?


濡傛灉鏈缃?link_validate op锛屽垯鏀圭敤榛樿鍑芥暟
`v4l2_subdev_link_validate_default`銆傝鍑芥暟纭繚閾捐矾鐨勬簮绔拰
sink 绔殑瀹藉害銆侀珮搴﹀拰 media 鎬荤嚎鍍忕礌鐮佺浉绛夈€傚瓙璁惧椹卞姩涔熷彲浠ヨ嚜鐢变娇鐢ㄦ鍑芥暟锛?
鍦ㄥ畠浠嚜宸辩殑妫€鏌ヤ箣澶栧啀鎵ц涓婅堪妫€鏌ャ€?


#### 瀛愯澶囨敞鍐岋紙Subdev registration锛?


鐩墠鏈変袱绉嶆柟寮忓悜 V4L2 鏍稿績娉ㄥ唽瀛愯澶囥€傜涓€绉嶏紙浼犵粺锛夋柟寮忔槸璁╂ˉ椹卞姩
娉ㄥ唽瀛愯澶囥€傚綋妗ラ┍鍔ㄦ嫢鏈変笌鍏惰繛鎺ョ殑瀛愯澶囩殑瀹屾暣淇℃伅锛屽苟涓旂‘鍒囩煡閬?
浣曟椂娉ㄥ唽瀹冧滑鏃讹紝灏卞彲浠ヨ繖鏍峰仛銆傚浜庡唴閮ㄥ瓙璁惧锛堝 SoC 鍐呴儴鐨勮棰戞暟鎹鐞嗗崟鍏?
鎴栧鏉傜殑 PCI(e) 鏉垮崱銆乁SB 鎽勫儚澶翠腑鐨勬憚鍍忓ご浼犳劅鍣ㄦ垨杩炴帴鍒?SoC 鐨勪紶鎰熷櫒锛夋潵璇达紝
閫氬父灏辨槸杩欑鎯呭喌锛屽畠浠€氬父鍦ㄥ叾骞冲彴鏁版嵁涓皢杩欎簺淇℃伅浼犻€掔粰妗ラ┍鍔ㄣ€?




鐒惰€岋紝涔熷瓨鍦ㄥ瓙璁惧蹇呴』鐩稿浜庢ˉ璁惧寮傛娉ㄥ唽鐨勬儏鍐点€傝繖绉嶉厤缃殑涓€涓緥瀛愭槸
鍩轰簬璁惧鏍戯紙Device Tree锛夌殑绯荤粺锛屽叾涓叧浜庡瓙璁惧鐨勪俊鎭嫭绔嬩簬妗ヨ澶囨彁渚涚粰绯荤粺锛?
渚嬪褰撳瓙璁惧鍦?DT 涓畾涔変负 I2C 璁惧鑺傜偣鏃躲€傜浜岀鎯呭喌浣跨敤鐨?API 灏嗗湪涓嬫枃杩涗竴姝ユ弿杩般€?




浣跨敤鍝娉ㄥ唽鏂规硶鍙奖鍝嶆帰娴嬶紙probing锛夎繃绋嬶紝杩愯鏃剁殑妗?瀛愯澶?
浜や簰鍦ㄤ袱绉嶆儏鍐典笅閮芥槸鐩稿悓鐨勩€?

##### 娉ㄥ唽鍚屾瀛愯澶?


鍦?*鍚屾锛坰ynchronous锛?*鎯呭喌涓嬶紝璁惧锛堟ˉ锛夐┍鍔ㄩ渶瑕佷娇鐢?v4l2_device 娉ㄥ唽
`v4l2_subdev`锛?

	`v4l2_device_register_subdev <v4l2_device_register_subdev>`
	(`v4l2_dev <v4l2_device>`, `sd <v4l2_subdev>`).

濡傛灉鍦ㄦ敞鍐屼箣鍓嶅瓙璁惧妯″潡宸叉秷澶憋紝鍒欏彲鑳戒細澶辫触銆?
璇ュ嚱鏁版垚鍔熻皟鐢ㄥ悗锛宻ubdev->dev 瀛楁鎸囧悜
`v4l2_device`銆?

濡傛灉 v4l2_device 鐖惰澶囩殑 mdev 瀛楁闈炵┖锛屽垯璇ュ瓙璁惧
瀹炰綋灏嗚嚜鍔ㄥ悜 media 璁惧娉ㄥ唽銆?

浣犲彲浠ヤ娇鐢ㄤ互涓嬫柟寮忔敞閿€瀛愯澶囷細

	`v4l2_device_unregister_subdev <v4l2_device_unregister_subdev>`
	(`sd <v4l2_subdev>`).

涔嬪悗锛屽瓙璁惧妯″潡鍙互琚嵏杞斤紝骞朵笖
`sd <v4l2_subdev>`->dev == `NULL`銆?


##### 娉ㄥ唽寮傛瀛愯澶?


鍦?*寮傛锛坅synchronous锛?*鎯呭喌涓嬶紝瀛愯澶囩殑鎺㈡祴鍙互鐙珛浜?
妗ラ┍鍔ㄧ殑鍙敤鎬ц€岃璋冪敤銆傚瓙璁惧椹卞姩闅忓悗蹇呴』楠岃瘉鎴愬姛鎺㈡祴鎵€闇€鐨?
鎵€鏈夋潯浠舵槸鍚﹂兘婊¤冻銆傝繖鍙兘鍖呮嫭瀵逛富鏃堕挓鍙敤鎬х殑妫€鏌ャ€傚鏋滀换浣曟潯浠朵笉婊¤冻锛?
椹卞姩鍙兘浼氬喅瀹氳繑鍥?`-EPROBE_DEFER` 浠ヨ姹傝繘涓€姝ョ殑閲嶆帰娴嬪皾璇曘€備竴鏃︽墍鏈夋潯浠堕兘婊¤冻锛?
瀛愯澶囧簲浣跨敤 `v4l2_async_register_subdev` 鍑芥暟娉ㄥ唽銆傛敞閿€鍒欎娇鐢?
`v4l2_async_unregister_subdev` 璋冪敤銆備互杩欑鏂瑰紡娉ㄥ唽鐨勫瓙璁惧瀛樺偍鍦ㄥ叏灞€瀛愯澶囧垪琛ㄤ腑锛?
闅忔椂鍑嗗琚ˉ椹卞姩鎷惧彇銆?




椹卞姩蹇呴』鍦ㄤ娇鐢?`v4l2_async_register_subdev` 娉ㄥ唽瀛愯澶囦箣鍓嶅畬鎴愬叾鎵€鏈夊垵濮嬪寲锛?
鍖呮嫭鍚敤杩愯鏃?PM锛坮untime PM锛夈€傝繖鏄洜涓哄瓙璁惧涓€鏃︽敞鍐屽氨绔嬪嵆鍙璁块棶銆?



##### 寮傛瀛愯澶囬€氱煡鍣紙notifiers锛?


妗ラ┍鍔ㄥ弽杩囨潵蹇呴』娉ㄥ唽涓€涓?notifier 瀵硅薄銆傝繖鏄€氳繃
浣跨敤 `v4l2_async_nf_register` 璋冪敤瀹屾垚銆傝娉ㄩ攢 notifier锛屽垯浣跨敤
`v4l2_async_nf_unregister`銆傚湪閲婃斁宸叉敞閿€ notifier 鐨勫唴瀛樹箣鍓嶏紝蹇呴』璋冪敤
`v4l2_async_nf_cleanup` 瀵瑰叾杩涜娓呯悊銆?


鍦ㄦ敞鍐?notifier 涔嬪墠锛屾ˉ椹卞姩蹇呴』鍋氫袱浠朵簨锛氶鍏堬紝蹇呴』浣跨敤
`v4l2_async_nf_init` 鍒濆鍖?notifier銆傚叾娆★紝妗ラ┍鍔ㄥ彲浠ュ紑濮嬪舰鎴?
妗ヨ澶囪繍琛屾墍闇€鐨勫紓姝ヨ繛鎺ユ弿杩扮鍒楄〃銆?


`v4l2_async_nf_add_fwnode_remote` 涓?`v4l2_async_nf_add_i2c`

寮傛杩炴帴鎻忚堪绗︽弿杩颁笌灏氭湭琚帰娴嬬殑澶栭儴瀛愯澶囩殑杩炴帴銆傚熀浜庝竴涓紓姝ヨ繛鎺ワ紝
褰撶浉鍏冲瓙璁惧鍙敤鏃讹紝鍙兘浼氬垱寤轰竴涓?media 鏁版嵁閾捐矾鎴栬緟鍔╅摼璺€傚浜庝竴涓粰瀹氱殑瀛愯澶囷紝
鍙兘鏈変竴涓垨澶氫釜寮傛杩炴帴锛屼絾鍦ㄥ皢杩欎簺杩炴帴娣诲姞鍒?notifier 鏃惰繕涓嶇煡閬撱€傚紓姝ヨ繛鎺ヤ細闅忕潃
鍖归厤鍒扮殑寮傛瀛愯澶囪閫愪釜缁戝畾銆?



##### 鐢ㄤ簬瀛愯澶囩殑寮傛瀛愯澶囬€氱煡鍣?


娉ㄥ唽寮傛瀛愯澶囩殑椹卞姩涔熷彲浠ユ敞鍐屼竴涓紓姝?notifier銆傝繖绉颁负寮傛瀛愯澶?notifier锛?
鍏惰繃绋嬩笌妗ラ┍鍔ㄧ被浼硷紝涓嶅悓涔嬪鍦ㄤ簬 notifier 鏄娇鐢?
 `v4l2_async_subdev_nf_init` 鍒濆鍖栫殑锛堣€岄潪妗ラ┍鍔ㄩ偅鏍凤級銆傚瓙璁惧
鍙敤涔嬪悗鎵嶈兘瀹屾垚锛屽嵆瀛樺湪涓€鏉＄粡鐢卞紓姝ュ瓙璁惧鍜?notifier 鍒拌揪鏌愪釜闈炲紓姝ュ瓙璁惧 notifier 鐨勮矾寰勩€?



##### 鐢ㄤ簬鎽勫儚澶翠紶鎰熷櫒椹卞姩鐨勫紓姝ュ瓙璁惧娉ㄥ唽杈呭姪鍑芥暟


`v4l2_async_register_subdev_sensor` 鏄竴涓敤浜庝紶鎰熷櫒椹卞姩鐨勮緟鍔╁嚱鏁帮紝瀹冩敞鍐?
鑷繁鐨勫紓姝ヨ繛鎺ワ紝鍚屾椂杩樹細娉ㄥ唽涓€涓?notifier锛屽苟杩涗竴姝ヤ负鍦ㄥ浐浠朵腑鎵惧埌鐨勯暅澶村拰闂厜鐏澶?
娉ㄥ唽寮傛杩炴帴銆傝瀛愯澶囩殑 notifier 浼氫娇鐢?`v4l2_async_unregister_subdev`
闅忚寮傛瀛愯澶囦竴璧疯娉ㄩ攢鍜屾竻鐞嗐€?


##### 寮傛瀛愯澶囬€氱煡鍣ㄧず渚?


杩欎簺鍑芥暟鍒嗛厤涓€涓紓姝ヨ繛鎺ユ弿杩扮锛屽叾绫诲瀷涓?struct
`v4l2_async_connection`锛屽祵鍏ュ湪涓€涓┍鍔ㄧ壒瀹氱殑缁撴瀯浣撲腑銆?struct
`v4l2_async_connection` 搴斾负璇ョ粨鏋勪綋鐨勭涓€涓垚鍛橈細


	struct my_async_connection {
		struct v4l2_async_connection asc;
		...
	};

	struct my_async_connection *my_asc;
	struct fwnode_handle *ep;

	...

	my_asc = v4l2_async_nf_add_fwnode_remote(&notifier, ep,
						 struct my_async_connection);
	fwnode_handle_put(ep);

	if (IS_ERR(my_asc))
		return PTR_ERR(my_asc);

##### Asynchronous sub-device notifier callbacks


鐒跺悗 V4L2 鏍稿績灏嗕娇鐢ㄨ繖浜涜繛鎺ユ弿杩扮锛屽皢寮傛娉ㄥ唽鐨勫瓙璁惧涓庝箣鍖归厤銆傚鏋滄娴嬪埌鍖归厤锛?
鍒欒皟鐢?`.bound()` notifier 鍥炶皟銆傚湪鎵€鏈夎繛鎺ラ兘缁戝畾鍚庯紝璋冪敤 .complete() 鍥炶皟銆?
褰撴煇涓繛鎺ヤ粠绯荤粺涓Щ闄ゆ椂锛岃皟鐢?`.unbind()` 鏂规硶銆傝繖涓変釜鍥炶皟閮芥槸鍙€夌殑銆?



椹卞姩鍙互鍦ㄥ叾椹卞姩鐗瑰畾鐨?
`v4l2_async_connection` 鍖呰鍣ㄤ腑瀛樺偍浠讳綍绫诲瀷鐨勮嚜瀹氫箟鏁版嵁銆傚鏋滆鏁版嵁鍦ㄧ粨鏋勪綋
琚噴鏀炬椂闇€瑕佺壒娈婂鐞嗭紝椹卞姩蹇呴』瀹炵幇 `.destroy()` notifier 鍥炶皟銆傛鏋跺皢鍦ㄩ噴鏀?
`v4l2_async_connection` 涔嬪墠绔嬪嵆璋冪敤瀹冦€?


#### 璋冪敤瀛愯澶囨搷浣?


浣跨敤 `v4l2_subdev` 鐨勪紭鐐瑰湪浜庡畠鏄竴涓€氱敤缁撴瀯浣擄紝
涓嶅寘鍚换浣曞叧浜庡簳灞傜‖浠剁殑淇℃伅銆傚洜姝わ紝涓€涓┍鍔ㄥ彲鑳藉寘鍚涓娇鐢?I2C 鎬荤嚎鐨勫瓙璁惧锛?
涔熷彲鑳藉寘鍚竴涓€氳繃 GPIO 寮曡剼鎺у埗鐨勫瓙璁惧銆傝繖绉嶅尯鍒彧鍦ㄨ缃澶囨椂鐩稿叧锛?
涓€鏃﹀瓙璁惧娉ㄥ唽瀹屾垚灏卞畬鍏ㄩ€忔槑浜嗐€?


涓€鏃﹀瓙璁惧娉ㄥ唽瀹屾垚锛屼綘鍙互鐩存帴璋冪敤涓€涓?ops 鍑芥暟锛?



	err = sd->ops->core->g_std(sd, &norm);

浣嗕娇鐢ㄨ繖涓畯鏇村ソ涔熸洿绠€鍗曪細


	err = v4l2_subdev_call(sd, core, g_std, &norm);

璇ュ畯浼氭墽琛屾纭殑 `NULL` 鎸囬拡妫€鏌ワ紝骞跺湪 `sd <v4l2_subdev>` 涓?`NULL` 鏃惰繑鍥?`-ENODEV`锛?
鍦?`sd <v4l2_subdev>`->core 鎴?`sd <v4l2_subdev>`->core->g_std 涓?`NULL` 鏃惰繑鍥?`-ENOIOCTLCMD`锛?
鍚﹀垯杩斿洖 `sd <v4l2_subdev>`->ops->core->g_std ops 鐨勫疄闄呯粨鏋溿€?


涔熷彲浠ヨ皟鐢ㄦ墍鏈夋垨涓€閮ㄥ垎瀛愯澶囷細


	v4l2_device_call_all(v4l2_dev, 0, core, g_std, &norm);

浠讳綍涓嶆敮鎸佽 ops 鐨勫瓙璁惧閮戒細琚烦杩囷紝閿欒缁撴灉琚拷鐣ャ€傚鏋滆妫€鏌ラ敊璇紝璇蜂娇鐢細



	err = v4l2_device_call_until_err(v4l2_dev, 0, core, g_std, &norm);

闄?`-ENOIOCTLCMD` 涔嬪鐨勪换浣曢敊璇兘浼氫互璇ラ敊璇€€鍑哄惊鐜€傚鏋滄病鏈?
鍙戠敓閿欒锛堥櫎 `-ENOIOCTLCMD` 澶栵級锛屽垯杩斿洖 0銆?

杩欎袱涓皟鐢ㄧ殑绗簩涓弬鏁版槸涓€涓粍 ID锛坓roup ID锛夈€傚鏋滀负 0锛屽垯璋冪敤鎵€鏈夊瓙璁惧銆?
濡傛灉闈為浂锛屽垯鍙皟鐢ㄧ粍 ID 涓庤鍊煎尮閰嶇殑瀛愯澶囥€傚湪妗ラ┍鍔ㄦ敞鍐屽瓙璁惧涔嬪墠锛屽畠鍙互灏?
`sd <v4l2_subdev>`->grp_id 璁剧疆涓哄畠鎯宠鐨勪换浣曞€硷紙榛樿涓?0锛夈€傝鍊肩敱妗ラ┍鍔ㄦ嫢鏈夛紝
瀛愯澶囬┍鍔ㄦ案杩滀笉浼氫慨鏀规垨浣跨敤瀹冦€?



缁?ID 璁╂ˉ椹卞姩鑳芥洿濂藉湴鎺у埗鍥炶皟鐨勮皟鐢ㄦ柟寮忋€備緥濡傦紝鏉夸笂鍙兘鏈夊涓煶棰戣姱鐗囷紝
姣忎釜閮借兘鏀瑰彉闊抽噺銆備絾閫氬父褰撶敤鎴锋兂瑕佹敼鍙橀煶閲忔椂锛屽彧鏈夊叾涓竴涓細鐪熸琚娇鐢ㄣ€備綘鍙互灏嗚瀛愯澶囩殑
缁?ID 璁剧疆涓轰緥濡?AUDIO_CONTROLLER锛屽苟鍦ㄨ皟鐢?`v4l2_device_call_all()` 鏃跺皢鍏舵寚瀹氫负缁?ID 鍊笺€?
杩欑‘淇濆畠鍙細鍙戦€佸埌闇€瑕佸畠鐨勯偅涓瓙璁惧銆?




濡傛灉瀛愯澶囬渶瑕佸悜鍏?v4l2_device 鐖惰澶囬€氱煡鏌愪釜浜嬩欢锛屽畠鍙互璋冪敤
`v4l2_subdev_notify(sd, notification, arg)`銆傝瀹忔鏌ユ槸鍚﹀畾涔変簡 `notify()` 鍥炶皟锛?
濡傛灉娌℃湁鍒欒繑鍥?`-ENODEV`銆傚惁鍒欒繑鍥?`notify()` 璋冪敤鐨勭粨鏋溿€?


### V4L2 瀛愯澶囩敤鎴风┖闂?API


妗ラ┍鍔ㄤ紶缁熶笂鍚戠敤鎴风┖闂存毚闇蹭竴涓垨澶氫釜瑙嗛鑺傜偣锛屽苟閫氳繃瀵硅棰戣妭鐐规搷浣滅殑
鍝嶅簲锛岄€氳繃 `v4l2_subdev_ops` 鎿嶄綔鏉ユ帶鍒跺瓙璁惧銆傝繖鍚戝簲鐢ㄧ▼搴忛殣钘忎簡
搴曞眰纭欢鐨勫鏉傛€с€傚浜庡鏉傝澶囷紝鍙兘闇€瑕佹瘮瑙嗛鑺傜偣鎻愪緵鐨勬洿缁嗙矑搴︾殑璁惧鎺у埗銆?
鍦ㄨ繖绉嶆儏鍐典笅锛屽疄鐜颁簡 media controller API <media_controller> 鐨勬ˉ椹卞姩鍙互閫夋嫨
璁╁瓙璁惧鎿嶄綔鍙洿鎺ヤ粠鐢ㄦ埛绌洪棿璁块棶銆?



Device nodes named `v4l-subdev`\ **X** can be created in `/dev` to access
瀛愯澶囥€傚鏋滃瓙璁惧鏀寔鐩存帴鐨勭敤鎴风┖闂撮厤缃紝瀹冨繀椤诲湪娉ㄥ唽涔嬪墠璁剧疆
`V4L2_SUBDEV_FL_HAS_DEVNODE` 鏍囧織銆?

娉ㄥ唽瀛愯澶囧悗锛宍v4l2_device` 椹卞姩鍙互閫氳繃璋冪敤
`v4l2_device_register_subdev_nodes` 涓烘墍鏈夋爣璁颁簡
`V4L2_SUBDEV_FL_HAS_DEVNODE` 鐨勫凡娉ㄥ唽瀛愯澶囧垱寤鸿澶囪妭鐐广€傚綋瀛愯澶囪娉ㄩ攢鏃讹紝
杩欎簺璁惧鑺傜偣浼氳鑷姩绉婚櫎銆?


璇ヨ澶囪妭鐐瑰鐞?V4L2 API 鐨勪竴涓瓙闆嗐€?

`VIDIOC_QUERYCTRL`,
`VIDIOC_QUERYMENU`,
`VIDIOC_G_CTRL`,
`VIDIOC_S_CTRL`,
`VIDIOC_G_EXT_CTRLS`,
`VIDIOC_S_EXT_CTRLS` and
`VIDIOC_TRY_EXT_CTRLS`:

	杩欎簺 controls ioctl 涓?V4L2 涓畾涔夌殑瀹屽叏鐩稿悓銆傚畠浠殑琛屼负涔熺浉鍚岋紝鍞竴鐨勪緥澶栨槸
	瀹冧滑鍙鐞嗙敱瀛愯澶囩敓鎴愮殑浜嬩欢銆傚彇鍐充簬椹卞姩锛岃繖浜涗簨浠朵篃鍙互鐢变竴涓紙鎴栧涓級
	V4L2 璁惧鑺傜偣璁块棶銆?
	
	

`VIDIOC_DQEVENT`,
`VIDIOC_SUBSCRIBE_EVENT` and
`VIDIOC_UNSUBSCRIBE_EVENT`

	杩欎簺 events ioctl 涓?V4L2 涓畾涔夌殑瀹屽叏鐩稿悓銆傚畠浠殑琛屼负涔熺浉鍚岋紝鍞竴鐨勪緥澶栨槸
	瀹冧滑鍙鐞嗙敱瀛愯澶囩敓鎴愮殑浜嬩欢銆傚彇鍐充簬椹卞姩锛岃繖浜涗簨浠朵篃鍙互鐢变竴涓紙鎴栧涓級
	V4L2 璁惧鑺傜偣鎶ュ憡銆?
	

	甯屾湜浣跨敤浜嬩欢鐨勫瓙璁惧椹卞姩闇€瑕佸湪娉ㄥ唽瀛愯澶囦箣鍓嶈缃?
	`V4L2_SUBDEV_FL_HAS_EVENTS` `v4l2_subdev`.flags銆傛敞鍐屽悗锛屼簨浠跺彲浠ュ儚寰€甯镐竴鏍?
	鍦?`v4l2_subdev`.devnode 璁惧鑺傜偣涓婃帓闃熴€?
	

	涓轰簡姝ｇ‘鏀寔浜嬩欢锛宍poll()` 鏂囦欢鎿嶄綔涔熻瀹炵幇銆?
	

绉佹湁 ioctl

	涓婅堪鍒楄〃涔嬪鐨勬墍鏈?ioctl 閮介€氳繃 core::ioctl 鎿嶄綔鐩存帴浼犻€掔粰瀛愯澶?
	椹卞姩銆?

### 鍙瀛愯澶囩敤鎴风┖闂?API


閫氳繃 `v4l2_subdev_ops` 缁撴瀯浣撳疄鐜扮殑 kernel API 鐩存帴璋冪敤鎺у埗鍏惰繛鎺ュ瓙璁惧鐨勬ˉ椹卞姩锛?
閫氬父涓嶅笇鏈涚敤鎴风┖闂磋兘澶熼€氳繃瀛愯澶囪澶囪妭鐐规洿鏀圭浉鍚岀殑鍙傛暟锛屽洜姝ら€氬父涓嶄細娉ㄥ唽浠讳綍姝ょ被鑺傜偣銆?



鏈夋椂閫氳繃鍙 API 鍚戠敤鎴风┖闂存姤鍛婂綋鍓嶅瓙璁惧鐨勯厤缃槸寰堟湁鐢ㄧ殑锛岃 API 涓嶅厑璁?
搴旂敤绋嬪簭鏇存敼璁惧鍙傛暟锛屼絾鍏佽涓庡瓙璁惧璁惧鑺傜偣浜や簰浠ユ鏌ュ畠浠€?



渚嬪锛屼负浜嗗疄鐜板熀浜庤绠楁憚褰辩殑鎽勫儚澶达紝鐢ㄦ埛绌洪棿闇€瑕佷簡瑙ｆ瘡涓彈鏀寔杈撳嚭鍒嗚鲸鐜囦笅
璇︾粏鐨勬憚鍍忓ご浼犳劅鍣ㄩ厤缃紙鍖呮嫭璺宠繃銆佸悎骞讹紙binning锛夈€佽鍓拰缂╂斁锛夈€備负浜嗘敮鎸佹绫荤敤渚嬶紝
妗ラ┍鍔ㄥ彲浠ラ€氳繃鍙 API 灏嗗瓙璁惧鎿嶄綔鏆撮湶缁欑敤鎴风┖闂淬€?



瑕佷负鎵€鏈変娇鐢?`V4L2_SUBDEV_FL_HAS_DEVNODE` 娉ㄥ唽鐨勫瓙璁惧鍒涘缓鍙璁惧鑺傜偣锛?
`v4l2_device` 椹卞姩搴旇皟鐢?`v4l2_device_register_ro_subdev_nodes`銆?


瀵逛簬浣跨敤 `v4l2_device_register_ro_subdev_nodes` 娉ㄥ唽鐨勫瓙璁惧璁惧鑺傜偣锛?
鐢ㄦ埛绌洪棿搴旂敤绋嬪簭瀵逛互涓?ioctl 鐨勮闂彈鍒伴檺鍒躲€?


`VIDIOC_SUBDEV_S_FMT`,
`VIDIOC_SUBDEV_S_CROP`,
`VIDIOC_SUBDEV_S_SELECTION`:

	杩欎簺 ioctl 浠呭湪鍙瀛愯澶囪澶囪妭鐐逛笂琚厑璁哥敤浜?
	V4L2_SUBDEV_FORMAT_TRY <v4l2-subdev-format-whence> 鐨勬牸寮忓拰閫夋嫨鐭╁舰銆?
	

`VIDIOC_SUBDEV_S_FRAME_INTERVAL`,
`VIDIOC_SUBDEV_S_DV_TIMINGS`,
`VIDIOC_SUBDEV_S_STD`:

	杩欎簺 ioctl 鍦ㄥ彧璇诲瓙璁惧鑺傜偣涓婁笉琚厑璁搞€?

濡傛灉 ioctl 涓嶈鍏佽锛屾垨鑰呰淇敼鐨勬牸寮忚璁剧疆涓?
`V4L2_SUBDEV_FORMAT_ACTIVE`锛屾牳蹇冭繑鍥炰竴涓礋鐨勯敊璇爜锛屽苟涓?errno 鍙橀噺琚缃负 `-EPERM`銆?


### I2C 瀛愯澶囬┍鍔?


鐢变簬杩欎簺椹卞姩闈炲父甯歌锛屾彁渚涗簡涓撻棬鐨勮緟鍔╁嚱鏁颁互绠€鍖栧叾浣跨敤锛坄v4l2-common.h`锛夈€?


鍚?I2C 椹卞姩娣诲姞 `v4l2_subdev` 鏀寔鐨勬帹鑽愭柟娉曟槸灏?`v4l2_subdev` 缁撴瀯浣?
宓屽叆鍒颁负姣忎釜 I2C 璁惧瀹炰緥鍒涘缓鐨勭姸鎬佺粨鏋勪綋涓€傞潪甯哥畝鍗曠殑璁惧娌℃湁鐘舵€佺粨鏋勪綋锛?
鍦ㄨ繖绉嶆儏鍐典笅浣犲彲浠ョ洿鎺ュ垱寤轰竴涓?`v4l2_subdev`銆?


涓€涓吀鍨嬬殑鐘舵€佺粨鏋勪綋濡備笅鎵€绀猴紙鍏朵腑 'chipname' 鏇挎崲涓鸿姱鐗囩殑鍚嶇О锛夛細



	struct chipname_state {
		struct v4l2_subdev sd;
		...  /** additional state fields **/
	};

鎸夊涓嬫柟寮忓垵濮嬪寲 `v4l2_subdev` 缁撴瀯浣擄細


	v4l2_i2c_subdev_init(&state->sd, client, subdev_ops);

璇ュ嚱鏁板皢濉厖 `v4l2_subdev` 鐨勬墍鏈夊瓧娈碉紝纭繚
`v4l2_subdev` 鍜?i2c_client 褰兼鎸囧悜瀵规柟銆?

浣犺繕搴旇娣诲姞涓€涓緟鍔╁唴鑱斿嚱鏁帮紝鐢ㄤ簬浠?`v4l2_subdev`
鎸囬拡杞崲鍒?chipname_state 缁撴瀯浣擄細


	static inline struct chipname_state **to_state(struct v4l2_subdev **sd)
	{
		return container_of(sd, struct chipname_state, sd);
	}

鐢ㄥ畠鏉ヤ粠 `v4l2_subdev` 缁撴瀯浣撹浆鎹㈠埌 `i2c_client`
缁撴瀯浣擄細


	struct i2c_client *client = v4l2_get_subdevdata(sd);

浠ヤ笅浠ｇ爜鐢ㄤ簬浠庝竴涓?`i2c_client` 杞埌 `v4l2_subdev` 缁撴瀯浣擄細


	struct v4l2_subdev *sd = i2c_get_clientdata(client);

纭繚鍦ㄨ皟鐢?`remove()` 鍥炶皟鏃惰皟鐢?

鍗充娇璇ュ瓙璁惧浠庢湭琚敞鍐岋紝璋冪敤瀹冧篃鏄畨鍏ㄧ殑銆?



浣犻渶瑕佽繖鏍峰仛锛屽洜涓哄綋妗ラ┍鍔ㄩ攢姣?i2c 閫傞厤鍣ㄦ椂锛屼細璋冪敤璇ラ€傞厤鍣ㄤ笂 i2c 璁惧鐨?
`remove()` 鍥炶皟銆傛鍚庣浉搴旂殑 v4l2_subdev 缁撴瀯浣撳け鏁堬紝鍥犳蹇呴』鍏堝皢瀹冧滑娉ㄩ攢銆傚湪
`remove()` 鍥炶皟涓皟鐢?`v4l2_device_unregister_subdev`\ (`sd <v4l2_subdev>`)
鍙‘淇濊繖涓€鐐瑰缁堣姝ｇ‘瀹屾垚銆?




妗ラ┍鍔ㄤ篃鏈変竴浜涘彲浠ヤ娇鐢ㄧ殑杈呭姪鍑芥暟锛?


	struct v4l2_subdev *sd = v4l2_i2c_new_subdev(v4l2_dev, adapter,
					"module_foo", "chipid", 0x36, NULL);

杩欎細鍔犺浇缁欏畾鐨勬ā鍧楋紙濡傛灉涓嶉渶瑕佸姞杞芥ā鍧楀垯涓?`NULL`锛夛紝骞朵娇鐢ㄧ粰瀹氱殑 `i2c_adapter` 鍜?
鑺墖/鍦板潃鍙傛暟璋冪敤 `i2c_new_client_device`銆傚鏋滀竴鍒囬『鍒╋紝鍒欏皢璇ュ瓙璁惧娉ㄥ唽鍒?
v4l2_device銆?


浣犱篃鍙互浣跨敤 `v4l2_i2c_new_subdev` 鐨勬渶鍚庝竴涓弬鏁版潵浼犻€掍竴涓畠搴旇鎺㈡祴鐨?
鍙兘鐨?I2C 鍦板潃鏁扮粍銆傝繖浜涙帰娴嬪湴鍧€浠呭湪鍓嶄竴涓弬鏁颁负 0 鏃朵娇鐢ㄣ€傞潪闆跺弬鏁版剰鍛崇潃浣犵煡閬?
纭垏鐨?i2c 鍦板潃锛屽洜姝ゅ湪杩欑鎯呭喌涓嬩笉浼氳繘琛屾帰娴嬨€?


濡傛灉鍑虹幇闂锛岃繖涓や釜鍑芥暟閮借繑鍥?`NULL`銆?

璇锋敞鎰忥紝浣犱紶閫掔粰 `v4l2_i2c_new_subdev` 鐨?chipid 閫氬父涓庢ā鍧楀悕绉扮浉鍚屻€傚畠鍏佽浣?
鎸囧畾涓€涓姱鐗囧彉浣擄紝渚嬪 "saa7114" 鎴?"saa7115"銆備笉杩囦竴鑸潵璇达紝i2c 椹卞姩浼氳嚜鍔ㄦ娴嬭繖涓€鐐广€?
chipid 鐨勪娇鐢ㄦ槸闇€瑕佸湪鏃ュ悗鏇翠粩缁嗙爺绌剁殑浜嬫儏銆傚畠鍦ㄤ笉鍚岀殑 i2c 椹卞姩涔嬮棿鏈夋墍宸紓锛屽洜姝ゅ彲鑳戒护浜哄洶鎯戙€?
瑕佹煡鐪嬫敮鎸佸摢浜涜姱鐗囧彉浣擄紝鍙互鏌ョ湅 i2c 椹卞姩浠ｇ爜涓殑 i2c_device_id 琛ㄣ€傚畠鍒楀嚭浜嗘墍鏈夊彲鑳芥€с€?




杩樻湁涓€涓緟鍔╁嚱鏁帮細

`v4l2_i2c_new_subdev_board` 浣跨敤涓€涓?`i2c_board_info` 缁撴瀯浣擄紝
璇ョ粨鏋勪綋琚紶閫掔粰 i2c 椹卞姩锛屽苟鏇夸唬 irq銆乸latform_data 鍜?addr 鍙傛暟銆?


濡傛灉瀛愯澶囨敮鎸?s_config core ops锛屽垯鍦ㄥ瓙璁惧璁剧疆瀹屾垚鍚庯紝浼氫互 irq 鍜?platform_data 鍙傛暟璋冪敤璇?op銆?


`v4l2_i2c_new_subdev` 鍑芥暟浼氬湪鍐呴儴璋冪敤
`v4l2_i2c_new_subdev_board`锛屼娇鐢?`client_type` 鍜?
`addr` 濉厖涓€涓?`i2c_board_info` 缁撴瀯浣撱€?


### 闆嗕腑绠＄悊鐨勫瓙璁惧娲诲姩鐘舵€?


浼犵粺涓婏紝V4L2 瀛愯澶囬┍鍔ㄤ负娲诲姩璁惧閰嶇疆缁存姢鍐呴儴鐘舵€併€傝繖閫氬父瀹炵幇涓轰緥濡備竴涓?
v4l2_mbus_framefmt 缁撴瀯浣撴暟缁勶紝姣忎釜 pad 涓€涓潯鐩紝瑁佸壀锛坈rop锛夊拰鍚堟垚锛坈ompose锛夌煩褰篃绫讳技銆?



闄や簡娲诲姩閰嶇疆澶栵紝姣忎釜瀛愯澶囨枃浠跺彞鏌勯兘鏈変竴涓敱 V4L2 鏍稿績绠＄悊鐨?struct
v4l2_subdev_state锛屽叾涓寘鍚?try


涓轰簡绠€鍖栧瓙璁惧椹卞姩锛孷4L2 瀛愯澶?API 鐜板湪鍙€夊湴鏀寔鐢?
`v4l2_subdev_state` 琛ㄧず鐨勯泦涓鐞嗙殑娲诲姩閰嶇疆銆備竴涓寘鍚椿鍔ㄨ澶囬厤缃殑
鐘舵€佸疄渚嬶紝浣滀负 `v4l2_subdev` 缁撴瀯浣撶殑涓€閮ㄥ垎瀛樺偍鍦ㄥ瓙璁惧鑷韩涓紱鑰屾牳蹇冨皢
涓€涓?try 鐘舵€佸叧鑱斿埌姣忎釜鎵撳紑鐨勬枃浠跺彞鏌勶紝浠ュ瓨鍌ㄤ笌璇ユ枃浠跺彞鏌勭浉鍏崇殑 try 閰嶇疆銆?




瀛愯澶囬┍鍔ㄥ彲浠ラ€夋嫨浣跨敤 state 鏉ョ鐞嗗叾娲诲姩閰嶇疆锛屾柟娉曟槸鍦ㄦ敞鍐屽瓙璁惧涔嬪墠璋冪敤
v4l2_subdev_init_finalize() 鏉ュ垵濮嬪寲瀛愯澶囩姸鎬併€傚畠浠繕蹇呴』鍦ㄦ敞閿€瀛愯澶囦箣鍓嶈皟鐢?
v4l2_subdev_cleanup() 鏉ラ噴鏀炬墍鏈夊凡鍒嗛厤鐨勮祫婧愩€傛牳蹇冧細鑷姩涓烘瘡涓墦寮€鐨勬枃浠跺彞鏌?
鍒嗛厤骞跺垵濮嬪寲涓€涓姸鎬佷互瀛樺偍 try 閰嶇疆锛屽苟鍦ㄥ叧闂枃浠跺彞鏌勬椂閲婃斁瀹冦€?




鍚屾椂浣跨敤 :ref:`ACTIVE 鍜?TRY 鏍煎紡 <v4l2-subdev-format-whence>` 鐨?V4L2 瀛愯澶囨搷浣滐紝
閫氳繃 'state' 鍙傛暟鎺ユ敹瑕佹搷浣滅殑姝ｇ‘鐘舵€併€傝皟鐢ㄨ€呭繀椤婚€氳繃璋冪敤
`v4l2_subdev_lock_state()` 鍜?`v4l2_subdev_unlock_state()` 鏉ラ攣瀹氬拰瑙ｉ攣璇ョ姸鎬併€?
璋冪敤鑰呭彲浠ラ€氳繃 `v4l2_subdev_call_state_active()` 瀹忔潵璋冪敤瀛愯澶囨搷浣溿€?



涓嶆帴鏀?state 鍙傛暟鐨勬搷浣滈殣寮忓湴瀵瑰瓙璁惧娲诲姩鐘舵€佽繘琛屾搷浣滐紝椹卞姩鍙互閫氳繃璋冪敤
`v4l2_subdev_lock_and_get_active_state()` 鐙崰璁块棶璇ョ姸鎬併€傚瓙璁惧鐨勬椿鍔ㄧ姸鎬?
鍚屾牱蹇呴』閫氳繃璋冪敤 `v4l2_subdev_unlock_state()` 鏉ラ噴鏀俱€?


椹卞姩缁濅笉鑳界洿鎺ユ墜鍔ㄨ闂瓨鍌ㄥ湪 `v4l2_subdev` 鎴栨枃浠跺彞鏌勪腑鐨勭姸鎬侊紝鑰屼笉缁忚繃鎸囧畾鐨勮緟鍔╁嚱鏁般€?


铏界劧 V4L2 鏍稿績浼氬皢姝ｇ‘鐨?try 鎴栨椿鍔ㄧ姸鎬佷紶閫掔粰瀛愯澶囨搷浣滐紝浣嗚澶氱幇鏈夌殑璁惧椹卞姩鍦ㄨ皟鐢?
`v4l2_subdev_call()` 鎿嶄綔鏃朵細浼犻€掍竴涓?NULL 鐘舵€併€傝繖绉嶉仐鐣欏啓娉曚細缁欒 V4L2 鏍稿績绠＄悊娲诲姩鐘舵€佺殑
瀛愯澶囬┍鍔ㄥ甫鏉ラ棶棰橈紝鍥犱负瀹冧滑鏈熸湜鎺ユ敹閫傚綋鐨勭姸鎬佷綔涓哄弬鏁庢暟銆備负浜嗗府鍔╁瓙璁惧椹卞姩杞崲涓哄彈绠＄悊鐨勬椿鍔ㄧ姸鎬侊紝
鑰屾棤闇€鍚屾椂杞崲鎵€鏈夎皟鐢ㄨ€咃紝鍦?v4l2_subdev_call() 涓坊鍔犱簡涓€涓澶栫殑灏佽灞傦紝瀹冮€氳繃鑾峰彇骞堕攣瀹?
琚皟鐢ㄨ€呯殑娲诲姩鐘舵€侊紙浣跨敤 `v4l2_subdev_lock_and_get_active_state()`锛夋潵澶勭悊 NULL 鎯呭喌锛?
骞跺湪璋冪敤鍚庤В閿佽鐘舵€併€?





鏁翠釜瀛愯澶囩姸鎬佸疄闄呬笂鍒嗕负涓変釜閮ㄥ垎锛歷4l2_subdev_state銆佸瓙璁惧鎺у埗椤癸紙controls锛夊拰瀛愯澶囬┍鍔ㄧ殑
鍐呴儴鐘舵€併€傚皢鏉ヨ繖浜涢儴鍒嗗簲鍚堝苟涓哄崟涓€鐘舵€併€傜洰鍓嶆垜浠渶瑕佷竴绉嶆柟娉曟潵澶勭悊杩欎簺閮ㄥ垎鐨勯攣瀹氥€傝繖鍙互閫氳繃
鍏变韩涓€涓攣鏉ュ疄鐜般€倂4l2_ctrl_handler 宸茬粡閫氳繃鍏?'lock' 鎸囬拡鏀寔杩欎竴鐐癸紝鐘舵€佷篃浣跨敤鐩稿悓鐨勬ā鍨嬨€傞┍鍔ㄥ彲浠ュ湪
璋冪敤 v4l2_subdev_init_finalize() 涔嬪墠鎵ц浠ヤ笅鎿嶄綔锛?





	sd->ctrl_handler->lock = &priv->mutex;
	sd->state_lock = &priv->mutex;

杩欏湪鎺у埗椤瑰拰鐘舵€佷箣闂村叡浜┍鍔ㄧ殑绉佹湁浜掓枼閿併€?

### 娴併€佸璺鐢?media pads 涓庡唴閮ㄨ矾鐢?


瀛愯澶囬┍鍔ㄥ彲浠ラ€氳繃璁剧疆 V4L2_SUBDEV_FL_STREAMS 瀛愯澶囨爣蹇楋紝骞跺疄鐜伴泦涓鐞嗙殑
瀛愯澶囨椿鍔ㄧ姸鎬併€佽矾鐢变互鍙婂熀浜庢祦鐨勯厤缃紝浠庤€屽疄鐜板澶氳矾澶嶇敤娴佺殑鏀寔銆?



### V4L2 瀛愯澶囧嚱鏁颁笌鏁版嵁缁撴瀯


