## PMBus 鏍稿績椹卞姩涓庡唴閮?API


## 绠€浠?

[from pmbus.org] 鐢垫簮绠＄悊鎬荤嚎锛圥MBus锛孭ower Management Bus锛夋槸涓€绉嶅紑鏀剧殑鐢垫簮绠＄悊鍗忚鏍囧噯锛?鍏跺畾涔変簡瀹屾暣鐨勫懡浠よ瑷€锛屼究浜庝笌鐢垫簮绯荤粺涓殑鐢垫簮杞崲鍣ㄤ欢鍙婂叾浠栬澶囪繘琛岄€氫俊銆傝鍗忚鏋勫缓浜?宸ヤ笟鏍囧噯鐨?SMBus 涓茶鎺ュ彛涔嬩笂锛屽彲瀵圭鍚堣鑼冪殑鐢垫簮杞崲浜у搧杩涜缂栫▼銆佹帶鍒跺拰瀹炴椂鐩戞帶銆傝繖涓€
鐏垫椿涓旈珮搴﹂€氱敤鐨勬爣鍑嗘敮鎸佸熀浜庢ā鎷熶笌鏁板瓧鎶€鏈澶囦箣闂寸殑閫氫俊锛屽苟鎻愪緵鐪熸鐨勪簰鎿嶄綔鎬э紝浠庤€岄檷浣?鐢垫簮绯荤粺璁捐鑰呯殑璁捐澶嶆潅搴﹀苟缂╃煭浜у搧涓婂競鏃堕棿銆傝寮€鏀剧殑鐢垫簮绯荤粺鏍囧噯鐢遍鍏堢殑鐢垫簮涓庡崐瀵间綋
鍏徃鍒涚珛锛屽苟鐢?PMBus 瀹炴柦鑰呰鍧涳紙PMBus-IF锛孭MBus Implementers Forum锛夌淮鎶や笌鎺ㄥ箍锛岃璁哄潧
鍖呭惈 30 浣欏閲囩敤鏂癸紝鏃ㄥ湪涓虹敤鎴锋彁渚涙敮鎸佸苟淇冭繘鍏堕噰鐢ㄣ€?
涓嶅垢鐨勬槸锛岃櫧鐒?PMBus 鍛戒护鏄爣鍑嗗寲鐨勶紝浣嗗苟娌℃湁寮哄埗鎬х殑鍛戒护锛屽埗閫犲晢鍙互娣诲姞浠绘剰澶氱殑闈炴爣鍑?鍛戒护銆傛澶栵紝涓嶅悓鐨?PMBus 璁惧鍦ㄦ帴鏀跺埌涓嶆敮鎸佺殑鍛戒护鏃惰涓哄悇寮傦細鏈変簺璁惧杩斿洖閿欒锛屾湁浜涜繑鍥?0xff 鎴?0xffff 骞惰缃姸鎬侀敊璇爣蹇楋紝杩樻湁浜涜澶囧彲鑳界洿鎺ユ寕璧枫€?
灏界瀛樺湪涓婅堪绉嶇鍥伴毦锛屼竴涓€氱敤鐨?PMBus 璁惧椹卞姩浠嶇劧鏈夌敤锛屽苟涓旇嚜鍐呮牳鐗堟湰 2.6.39 璧峰緱鍒版敮鎸併€?鐒惰€岋紝闄や簡鏍稿績 PMBus 椹卞姩涔嬪锛岃繕蹇呴』鏀寔璁惧鐗瑰畾鐨勬墿灞曪紝鍥犱负 PMBus 璁惧寮€鍙戣€呮帴涓嬫潵浼?鎺ㄥ嚭浣曠鏂扮殑璁惧鐗瑰畾鍔熻兘锛岀洰鍓嶆牴鏈棤浠庡緱鐭ャ€?
涓轰簡浣胯澶囩壒瀹氱殑鎵╁睍灏藉彲鑳藉叿澶囧彲鎵╁睍鎬э紝骞堕伩鍏嶄负鏂板瀷璁惧鍙嶅淇敼鏍稿績 PMBus 椹卞姩锛孭MBus
椹卞姩琚媶鍒嗕负鏍稿績浠ｇ爜銆侀€氱敤浠ｇ爜鍜岃澶囩壒瀹氫唬鐮併€傛牳蹇冧唬鐮侊紙浣嶄簬 `pmbus_core.c`锛夋彁渚涢€氱敤鍔熻兘銆?閫氱敤浠ｇ爜锛堜綅浜?`pmbus.c`锛夋彁渚涘閫氱敤 PMBus 璁惧鐨勬敮鎸併€傝澶囩壒瀹氫唬鐮佽礋璐ｈ澶囩壒瀹氱殑鍒濆鍖栵紝
骞跺湪闇€瑕佹椂鎶婅澶囩壒瀹氬姛鑳芥槧灏勪负閫氱敤鍔熻兘銆傝繖鍦ㄦ煇绉嶇▼搴︿笂绫讳技浜?PCI 浠ｇ爜锛屽叾涓€氱敤浠ｇ爜浼氭牴鎹?闇€瑕侀拡瀵瑰悇绫昏澶囦互鐗瑰畾鐨?quirk锛堟€櫀/鍏煎鎬у鐞嗭級杩涜鎵╁厖銆?
## PMBus 璁惧鑳藉姏鑷姩妫€娴?

瀵逛簬閫氱敤 PMBus 璁惧锛宍pmbus.c` 涓殑浠ｇ爜浼氬皾璇曡嚜鍔ㄦ娴嬫墍鏈夊彈鏀寔鐨?PMBus 鍛戒护銆傝嚜鍔ㄦ娴嬪湪
涓€瀹氱▼搴︿笂鍙楀埌闄愬埗锛屽洜涓洪渶瑕佽€冭檻鐨勫彉閲忓疄鍦ㄥお澶氥€備緥濡傦紝鍑犱箮涓嶅彲鑳借嚜鍔ㄦ娴嬪埌鍝簺 PMBus 鍛戒护
鏄垎椤电殑銆佸摢浜涘懡浠ゅ湪鎵€鏈夐〉闈㈤棿琚鍒讹紙鏈夊叧澶氶〉 PMBus 璁惧鐨勭粏鑺傦紝璇峰弬闃?PMBus 瑙勮寖锛夈€?
鍥犳锛屽綋骞堕潪鎵€鏈夊懡浠ら兘鑳借鑷姩妫€娴嬫椂锛岄€氬父鎻愪緵涓€涓澶囩壒瀹氱殑椹卞姩鏄洿鍚堢悊鐨勫仛娉曘€傝椹卞姩涓殑
鏁版嵁缁撴瀯鍙敤浜庡悜鏍稿績椹卞姩鍛婄煡鍚勪釜鑺墖鎵€鏀寔鐨勫姛鑳姐€?
鏈変簺鍛戒护濮嬬粓浼氳鑷姩妫€娴嬨€傝繖閫傜敤浜庢墍鏈夐檺鍒剁被鍛戒护锛坙crit銆乵in銆乵ax 浠ュ強 crit 灞炴€э級浠ュ強鐩稿叧鐨?鎶ヨ灞炴€с€傞檺鍒跺拰鎶ヨ灞炴€ц鑷姩妫€娴嬶紝鏄洜涓哄彲鑳界殑缁勫悎瀹炲湪澶锛屾棤娉曟彁渚涗竴涓墜宸ラ厤缃帴鍙ｃ€?
## PMBus 鍐呴儴 API


鏍稿績浠ｇ爜涓庤澶囩壒瀹?PMBus 浠ｇ爜涔嬮棿鐨?API 瀹氫箟浜?`drivers/hwmon/pmbus/pmbus.h`銆傞櫎鍐呴儴 API 澶栵紝
`pmbus.h` 杩樺畾涔変簡鏍囧噯 PMBus 鍛戒护鍜岃櫄鎷?PMBus 鍛戒护銆?
### 鏍囧噯 PMBus 鍛戒护


鏍囧噯 PMBus 鍛戒护锛堝懡浠ゅ€?0x00 鑷?0xff锛夊湪 PMBus 瑙勮寖涓畾涔夈€?
### 铏氭嫙 PMBus 鍛戒护


鎻愪緵铏氭嫙 PMBus 鍛戒护鏄负浜嗘敮鎸佷竴浜涘凡鐢卞涓姱鐗囧巶鍟嗗疄鐜般€佸洜姝ゅ€煎緱鏀寔鐨勯潪鏍囧噯鍔熻兘銆?
铏氭嫙 PMBus 鍛戒护浠庡懡浠ゅ€?0x100 寮€濮嬶紝鍥犳寰堝鏄撲笌鏍囧噯 PMBus 鍛戒护鍖哄垎寮€鏉ワ紙鏍囧噯鍛戒护鐨勫€间笉鍙兘
澶т簬 0xff锛夈€傝櫄鎷?PMBus 鍛戒护鐨勬敮鎸佹槸璁惧鐗瑰畾鐨勶紝鍥犳蹇呴』鍦ㄨ澶囩壒瀹氫唬鐮佷腑瀹炵幇銆?
铏氭嫙鍛戒护鍛藉悕涓?`PMBUS_VIRT_xxx`锛屽苟浠?`PMBUS_VIRT_BASE` 涓鸿捣濮嬨€傛墍鏈夎櫄鎷熷懡浠ゅ潎涓哄瓧锛坵ord锛?澶у皬銆?
鐩墠鏈変袱绉嶇被鍨嬬殑铏氭嫙鍛戒护銆?
- READ 鍛戒护涓哄彧璇伙紱鍐欏叆鎿嶄綔瑕佷箞琚拷鐣ワ紝瑕佷箞杩斿洖閿欒銆?- RESET 鍛戒护鍙鍙啓銆傝鍙栧浣嶅瘎瀛樺櫒杩斿洖闆讹紙鐢ㄤ簬妫€娴嬶級锛屽啓鍏ヤ换鎰忓€间細瀵艰嚧鐩稿叧鐨勫巻鍙茶褰曡澶嶄綅銆?
铏氭嫙鍛戒护蹇呴』鍦ㄨ澶囩壒瀹氶┍鍔ㄤ唬鐮佷腑杩涜澶勭悊銆傝嫢鏌愯櫄鎷熷懡浠ゅ彈鏀寔锛岃姱鐗囬┍鍔ㄤ唬鐮佽繑鍥為潪璐熷€硷紱鑻ヤ笉鍙?鏀寔锛屽垯杩斿洖璐熺殑閿欒鐮併€傚湪杩欑鎯呭喌涓嬶紝鑺墖椹卞姩鍙互杩斿洖 `-ENODATA` 鎴栦换浣曞叾浠?Linux 閿欒鐮侊紝
涓嶈繃浣跨敤 `-ENODATA` 涔嬪鐨勯敊璇爜澶勭悊鏁堢巼鏇撮珮锛屽洜鑰屾洿鍙楁帹鑽愩€傛棤璁哄摢绉嶆儏鍐碉紝褰撹鍙栨垨鍐欏叆铏氭嫙
瀵勫瓨鍣ㄦ椂锛岃嫢鑺墖椹卞姩杩斿洖閿欒鐮侊紝璋冪敤鐨?PMBus 鏍稿績浠ｇ爜閮戒細涓锛堟崲鍙ヨ瘽璇达紝PMBus 鏍稿績浠ｇ爜姘歌繙
涓嶄細鍚戣姱鐗囧彂閫佽櫄鎷熷懡浠わ級銆?
### PMBus 椹卞姩淇℃伅


PMBus 椹卞姩淇℃伅瀹氫箟浜?`struct pmbus_driver_info`锛屾槸璁惧鐗瑰畾椹卞姩鍚戞牳蹇?PMBus 椹卞姩浼犻€掍俊鎭殑
涓昏鎵嬫銆傚叿浣撹€岃█锛屽畠鎻愪緵浠ヤ笅淇℃伅銆?
- 瀵逛簬浠ユ敮鎸?Direct Data Format锛堢洿鎺ユ暟鎹牸寮忥級淇濆瓨鍏舵暟鎹殑璁惧锛屽畠鎻愪緵灏嗗瘎瀛樺櫒鍊艰浆鎹负
  瑙勮寖鍖栨暟鎹殑绯绘暟銆傝繖浜涙暟鎹€氬父鐢辫姱鐗囧埗閫犲晢鍦ㄥ櫒浠舵暟鎹墜鍐屼腑鎻愪緵銆?- 鍙皢鑺墖鎵€鏀寔鐨勫姛鑳藉憡鐭ユ牳蹇冮┍鍔ㄣ€傝繖瀵逛簬閭ｄ簺鍦ㄦ墽琛屼笉鍙楁敮鎸佺殑鍛戒护鏃朵細琛ㄧ幇寮傚父銆佸拰/鎴栦负浜?  鍔犻€熻澶囨娴嬩笌鍒濆鍖栫殑鑺墖鑰岃█鍙兘鏄繀瑕佺殑銆?- 鎻愪緵鑻ュ共鍑芥暟鍏ュ彛鐐癸紝鐢ㄤ簬鏀寔瀵归€氱敤鍛戒护鎵ц鐨勮鐩栧拰/鎴栧寮恒€傛鍔熻兘鍙敤浜庡皢闈炴爣鍑?PMBus
  鍛戒护鏄犲皠涓烘爣鍑嗗懡浠わ紝鎴栬€呯敤璁惧鐗瑰畾淇℃伅澧炲己鏍囧噯鍛戒护鐨勮繑鍥炲€笺€?
## PEC 鏀寔


璁稿 PMBus 璁惧鏀寔 SMBus PEC锛圥acket Error Checking锛屽寘閿欒妫€鏌ワ級銆傝嫢 I2C 閫傞厤鍣ㄤ笌 PMBus
鑺墖鍙屾柟閮芥敮鎸侊紝鍒欓粯璁ゅ惎鐢ㄣ€傝嫢鏀寔 PEC锛孭MBus 鏍稿績椹卞姩浼氬悜 I2C 璁惧娣诲姞涓€涓悕涓?`pec` 鐨勫睘鎬с€?璇ュ睘鎬у彲鐢ㄤ簬鎺у埗涓?PMBus 鑺墖閫氫俊鏃剁殑 PEC 鏀寔銆?
## API 鍑芥暟


### 鑺墖椹卞姩鎻愪緵鐨勫嚱鏁?

鎵€鏈夊嚱鏁拌嫢鎴愬姛鍒欒繑鍥炲懡浠よ繑鍥炲€硷紙璇伙級鎴栭浂锛堝啓锛夈€傝繑鍥炲€?`-ENODATA` 琛ㄧず璇ュ懡浠ゆ病鏈夊巶鍟嗙壒瀹?瀹炵幇锛屼絾鍙兘瀛樺湪鏍囧噯鐨?PMBus 鍛戒护銆備换浣曞叾浠栬礋鐨勮繑鍥炲€艰〃绀鸿鍛戒护鍦ㄦ鑺墖涓婁笉瀛樺湪锛屼笖涓嶅簲鍐嶅皾璇?璇诲彇鎴栧啓鍏ヨ鏍囧噯鍛戒护銆?
濡備笂鎵€杩帮紝铏氭嫙鍛戒护鏄瑙勫垯鐨勪竴涓緥澶栵紝**蹇呴』**鍦ㄩ┍鍔ㄧ壒瀹氫唬鐮佷腑澶勭悊銆傛洿澶氱粏鑺傝鍙傞槄涓婃枃鐨?鈥滆櫄鎷?PMBus 鍛戒护鈥濄€?
```

	if (chip_access_function) {
		status = chip_access_function();
		if (status != -ENODATA)
			return status;
	}
	if (command >= PMBUS_VIRT_BASE)	/* For word commands/registers only */
		return -EINVAL;
	return generic_access();

```
鑺墖椹卞姩鍙湪 `struct pmbus_driver_info` 涓彁渚涙寚鍚戜互涓嬪嚱鏁扮殑鎸囬拡銆傛墍鏈夊嚱鏁板潎涓哄彲閫夐」銆?
```

  int (*read_byte_data)(struct i2c_client *client, int page, int reg);

```
浠庨〉闈?`<page>`銆佸瘎瀛樺櫒 `<reg>` 璇诲彇涓€涓瓧鑺傘€?`<page>` 鍙互涓?-1锛岃〃绀衡€滃綋鍓嶉〉闈⑩€濄€?

```

  int (*read_word_data)(struct i2c_client *client, int page, int phase,
                        int reg);

```
浠庨〉闈?`<page>`銆佺浉浣?`<phase>`銆佸瘎瀛樺櫒 `<reg>` 璇诲彇涓€涓瓧銆傝嫢鑺墖涓嶆敮鎸佸鐩镐綅锛屽垯 phase 鍙傛暟
鍙蹇界暐銆傝嫢鑺墖鏀寔澶氱浉浣嶏紝鐩镐綅鍊?0xff 琛ㄧず鎵€鏈夌浉浣嶃€?
```

  int (*write_word_data)(struct i2c_client *client, int page, int reg,
			 u16 word);

```
鍚戦〉闈?`<page>`銆佸瘎瀛樺櫒 `<reg>` 鍐欏叆涓€涓瓧銆?
```

  int (*write_byte)(struct i2c_client *client, int page, u8 value);

```
鍚戦〉闈?`<page>`銆佸瘎瀛樺櫒 `<reg>` 鍐欏叆涓€涓瓧鑺傘€?`<page>` 鍙互涓?-1锛岃〃绀衡€滃綋鍓嶉〉闈⑩€濄€?
```

  int (*identify)(struct i2c_client *client, struct pmbus_driver_info *info);

```
纭畾鎵€鏀寔鐨?PMBus 鍔熻兘銆備粎褰撹姱鐗囬┍鍔ㄦ敮鎸佸绉嶈姱鐗囥€佷笖鑺墖鍔熻兘鏃犳硶棰勫厛纭畾鏃讹紝姝ゅ嚱鏁版墠鏄繀闇€鐨勩€?鐩墠浠呯敱閫氱敤 pmbus 椹卞姩锛坄pmbus.c`锛変娇鐢ㄣ€?
### 鏍稿績椹卞姩瀵煎嚭鐨勫嚱鏁?

鑺墖椹卞姩搴斾娇鐢ㄤ互涓嬪嚱鏁版潵璇诲彇鎴栧啓鍏?PMBus 瀵勫瓨鍣ㄣ€傝姱鐗囬┍鍔ㄤ篃鍙互浣跨敤鐩存帴鐨?I2C 鍛戒护銆傝嫢浣跨敤
鐩存帴 I2C 鍛戒护锛岃姱鐗囬┍鍔ㄤ唬鐮佷笉寰楃洿鎺ヤ慨鏀瑰綋鍓嶉〉闈紝鍥犱负鎵€閫夐〉闈㈠凡琚紦瀛樺湪鏍稿績椹卞姩涓紝鏍稿績椹卞姩浼?鍋囧畾璇ラ〉闈㈠凡琚€変腑銆傚繀椤讳娇鐢?`pmbus_set_page()` 鏉ラ€夋嫨鏂伴〉闈€?
```

  int pmbus_set_page(struct i2c_client *client, u8 page, u8 phase);

```
灏?PMBus 椤甸潰瀵勫瓨鍣ㄨ缃负 `<page>` 鍜?`<phase>`锛屼緵鍚庣画鍛戒护浣跨敤銆?鑻ヨ姱鐗囦笉鏀寔澶氱浉浣嶏紝鍒?phase 鍙傛暟琚拷鐣ャ€傚惁鍒欙紝鐩镐綅鍊?0xff 閫夋嫨鎵€鏈夌浉浣嶃€?
```

  int pmbus_read_word_data(struct i2c_client *client, u8 page, u8 phase,
                           u8 reg);

```
浠?`<page>`銆乣<phase>`銆乣<reg>` 璇诲彇瀛楁暟鎹€傜被浼间簬 `i2c_smbus_read_word_data()`锛屼絾浼氬厛閫夋嫨
椤甸潰鍜岀浉浣嶃€傝嫢鑺墖涓嶆敮鎸佸鐩镐綅锛屽垯 phase 鍙傛暟琚拷鐣ャ€傚惁鍒欙紝鐩镐綅鍊?0xff 閫夋嫨鎵€鏈夌浉浣嶃€?
```

  int pmbus_write_word_data(struct i2c_client *client, u8 page, u8 reg,
			    u16 word);

```
鍚?`<page>`銆乣<reg>` 鍐欏叆瀛楁暟鎹€傜被浼间簬 `i2c_smbus_write_word_data()`锛屼絾浼氬厛閫夋嫨椤甸潰銆?
```

  int pmbus_read_byte_data(struct i2c_client *client, int page, u8 reg);

```
浠?`<page>`銆乣<reg>` 璇诲彇瀛楄妭鏁版嵁銆傜被浼间簬 `i2c_smbus_read_byte_data()`锛屼絾浼氬厛閫夋嫨椤甸潰銆俙<page>`
鍙互涓?-1锛岃〃绀衡€滃綋鍓嶉〉闈⑩€濄€?
```

  int pmbus_write_byte(struct i2c_client *client, int page, u8 value);

```
鍚?`<page>`銆乣<reg>` 鍐欏叆瀛楄妭鏁版嵁銆傜被浼间簬 `i2c_smbus_write_byte()`锛屼絾浼氬厛閫夋嫨椤甸潰銆俙<page>` 鍙互
涓?-1锛岃〃绀衡€滃綋鍓嶉〉闈⑩€濄€?
```

  void pmbus_clear_faults(struct i2c_client *client);

```
鍦ㄦ墍鏈夎姱鐗囬〉闈笂鎵ц PMBus 鐨勨€滄竻闄ゆ晠闅滐紙Clear Fault锛夆€濆懡浠ゃ€?姝ゅ嚱鏁颁細璋冪敤璁惧鐗瑰畾鐨?write_byte 鍑芥暟锛堣嫢宸插畾涔夛級銆傚洜姝わ紝缁濆涓嶈兘浠庤鍑芥暟涓皟鐢ㄥ畠銆?
```

  bool pmbus_check_byte_register(struct i2c_client *client, int page, int reg);

```
妫€鏌ュ瓧鑺傚瘎瀛樺櫒鏄惁瀛樺湪銆傝嫢瀛樺湪鍒欒繑鍥?true锛屽惁鍒欒繑鍥?false銆?姝ゅ嚱鏁颁細璋冪敤璁惧鐗瑰畾鐨?write_byte 鍑芥暟锛堣嫢宸插畾涔夛級浠ヨ幏鍙栬姱鐗囩姸鎬併€傚洜姝わ紝缁濆涓嶈兘浠庤鍑芥暟涓皟鐢ㄥ畠銆?
```

  bool pmbus_check_word_register(struct i2c_client *client, int page, int reg);

```
妫€鏌ュ瓧瀵勫瓨鍣ㄦ槸鍚﹀瓨鍦ㄣ€傝嫢瀛樺湪鍒欒繑鍥?true锛屽惁鍒欒繑鍥?false銆?姝ゅ嚱鏁颁細璋冪敤璁惧鐗瑰畾鐨?write_byte 鍑芥暟锛堣嫢宸插畾涔夛級浠ヨ幏鍙栬姱鐗囩姸鎬併€傚洜姝わ紝缁濆涓嶈兘浠庤鍑芥暟涓皟鐢ㄥ畠銆?
```

  int pmbus_do_probe(struct i2c_client *client, struct pmbus_driver_info *info);

```
鎵ц probe 鍑芥暟銆傜被浼间簬鍏朵粬椹卞姩鐨勬爣鍑?probe 鍑芥暟锛屼絾棰濆甯︽湁涓€涓寚鍚?`struct pmbus_driver_info`
鐨勬寚閽堜綔涓哄弬鏁般€傝嫢鏀寔 identify 鍑芥暟鍒欎細璋冪敤瀹冦€傚彧鑳戒粠璁惧鐨?probe 鍑芥暟涓皟鐢ㄣ€?
```

  const struct pmbus_driver_info
	*pmbus_get_driver_info(struct i2c_client *client);

```
杩斿洖浼犲叆 `pmbus_do_probe()` 鐨?`struct pmbus_driver_info` 鎸囬拡銆?

## PMBus 椹卞姩骞冲彴鏁版嵁


PMBus 骞冲彴鏁版嵁瀹氫箟浜?`include/linux/pmbus.h`銆傚钩鍙版暟鎹?
```

	#define PMBUS_SKIP_STATUS_CHECK			BIT(0)

	#define PMBUS_WRITE_PROTECTED			BIT(1)

	#define PMBUS_NO_CAPABILITY			BIT(2)

	#define PMBUS_READ_STATUS_AFTER_FAILED_CHECK	BIT(3)

	#define PMBUS_NO_WRITE_PROTECT			BIT(4)

	#define PMBUS_USE_COEFFICIENTS_CMD		BIT(5)

	#define PMBUS_OP_PROTECTED			BIT(6)

	#define PMBUS_VOUT_PROTECTED			BIT(7)

	struct pmbus_platform_data {
		u32 flags;              /* Device specific flags */

		/* regulator support */
		int num_regulators;
		struct regulator_init_data *reg_init_data;
	};


```
### 鏍囧織浣?

PMBUS_SKIP_STATUS_CHECK

鍦ㄥ瘎瀛樺櫒妫€娴嬫湡闂达紝璺宠繃瀵圭姸鎬佸瘎瀛樺櫒閫氫俊鎴栧懡浠ら敊璇殑妫€鏌ャ€?
鏈変簺 PMBus 鑺墖鍦ㄥ皾璇曡鍙栦笉鍙楁敮鎸佺殑瀵勫瓨鍣ㄦ椂浼氳繑鍥炴湁鏁堟暟鎹€傚浜庢绫昏姱鐗囷紝鍦ㄥ皾璇曠‘瀹氭煇涓姱鐗?瀵勫瓨鍣ㄦ槸鍚﹀瓨鍦ㄦ椂锛屾鏌ョ姸鎬佸瘎瀛樺櫒鏄繀闇€鐨勩€傚彟涓€浜?PMBus 鑺墖涓嶆敮鎸?STATUS_CML 瀵勫瓨鍣紝鎴栬€?浼氭棤缂樻棤鏁呭湴鎶ュ憡閫氫俊閿欒銆傚浜庢绫昏姱鐗囷紝蹇呴』绂佺敤鐘舵€佸瘎瀛樺櫒鐨勬鏌ャ€?
鏈変簺 i2c 鎺у埗鍣ㄤ笉鏀寔鍗曞瓧鑺傚懡浠わ紙鍗虫棤鏁版嵁鐨勫啓鍛戒护 `i2c_smbus_write_byte()`锛夈€傚浜庢绫绘帶鍒跺櫒锛?娓呴櫎鐘舵€佸瘎瀛樺櫒鏄笉鍙兘鐨勶紝鍥犳蹇呴』璁剧疆 `PMBUS_SKIP_STATUS_CHECK` 鏍囧織銆?
PMBUS_WRITE_PROTECTED

鑻ヨ姱鐗囧浜庡啓淇濇姢鐘舵€侊紝涓斿啓淇濇姢骞堕潪鐢辨爣鍑嗙殑 WRITE_PROTECT 鍛戒护鍐冲畾锛屽垯璁剧疆姝ゆ爣蹇椼€?
PMBUS_NO_CAPABILITY

鏈変簺 PMBus 鑺墖鍦ㄨ鍙?CAPABILITY 瀵勫瓨鍣ㄦ椂涓嶄細杩斿洖鏈夋晥鏁版嵁銆傚浜庢绫昏姱鐗囷紝搴旇缃鏍囧織锛屼互渚?PMBus 鏍稿績椹卞姩涓嶄細浣跨敤 CAPABILITY 鏉ュ垽鏂叾琛屼负銆?
PMBUS_READ_STATUS_AFTER_FAILED_CHECK

鍦ㄦ瘡娆″け璐ョ殑瀵勫瓨鍣ㄦ鏌ュ悗璇诲彇 STATUS 瀵勫瓨鍣ㄣ€?
鏈変簺 PMBus 鑺墖鍦ㄥ皾璇曡鍙栦笉鍙楁敮鎸佺殑瀵勫瓨鍣ㄦ椂浼氳繘鍏ユ湭瀹氫箟鐘舵€併€傚浜庢绫昏姱鐗囷紝鍦ㄥけ璐ョ殑瀵勫瓨鍣ㄦ鏌?鍚庯紝鏈夊繀瑕佸皢鑺墖鐨?PMBus 鎺у埗鍣ㄥ浣嶅埌宸茬煡鐘舵€併€傝繖鍙互閫氳繃璇诲彇涓€涓凡鐭ュ瘎瀛樺櫒鏉ュ疄鐜般€傝缃鏍囧織
鍚庯紝椹卞姩灏嗗湪姣忔澶辫触鐨勫瘎瀛樺櫒妫€鏌ュ悗灏濊瘯璇诲彇 STATUS 瀵勫瓨鍣ㄣ€傛璇诲彇鍙兘澶辫触锛屼絾瀹冧細鎶婅姱鐗囩疆鍏?宸茬煡鐘舵€併€?
PMBUS_NO_WRITE_PROTECT

鏈変簺 PMBus 鑺墖鍦ㄨ鍙?WRITE_PROTECT 瀵勫瓨鍣ㄦ椂浼氳繑鍥炴棤鏁堟暟鎹€傚浜庢绫昏姱鐗囷紝搴旇缃鏍囧織锛屼互渚?PMBus 鏍稿績椹卞姩涓嶄細浣跨敤 WRITE_PROTECT 鍛戒护鏉ュ垽鏂叾琛屼负銆?
PMBUS_USE_COEFFICIENTS_CMD

璁剧疆姝ゆ爣蹇楀悗锛孭MBus 鏍稿績椹卞姩灏嗕娇鐢?COEFFICIENTS 瀵勫瓨鍣ㄦ潵鍒濆鍖?direct mode锛堢洿鎺ユā寮忥級鏍煎紡
鐨勭郴鏁般€?
PMBUS_OP_PROTECTED

鑻ヨ姱鐗囩殑 OPERATION 鍛戒护鍙椾繚鎶わ紝涓斾繚鎶ゅ苟闈炵敱鏍囧噯鐨?WRITE_PROTECT 鍛戒护鍐冲畾锛屽垯璁剧疆姝ゆ爣蹇椼€?
PMBUS_VOUT_PROTECTED

鑻ヨ姱鐗囩殑 VOUT_COMMAND 鍛戒护鍙椾繚鎶わ紝涓斾繚鎶ゅ苟闈炵敱鏍囧噯鐨?WRITE_PROTECT 鍛戒护鍐冲畾锛屽垯璁剧疆姝ゆ爣蹇椼€?
### 妯″潡鍙傛暟


pmbus_core.wp锛歅MBus 鍐欎繚鎶ゅ己鍒舵ā寮?
PMBus 鍙兘浠ュ绉嶅啓淇濇姢閰嶇疆鍚姩銆俙pmbus_core.wp` 鍙敤浜庡湪闇€瑕佺壒瀹氬啓淇濇姢鏃朵娇鐢ㄣ€傚疄闄呮洿鏀逛繚鎶?鐨勮兘鍔涗篃鍙兘鍙栧喅浜庤姱鐗囷紝鍥犳杩愯鏃跺疄闄呯殑鍐欎繚鎶ら厤缃彲鑳戒笉鍚屼簬鎵€璇锋眰鐨勩€俻mbus_core 褰撳墠鏀寔
浠ヤ笅鍙栧€硷細

- 0锛氱Щ闄ゅ啓淇濇姢銆?- 1锛氱姝㈡墍鏈夊啓鍏ワ紝浠呭厑璁稿 WRITE_PROTECT銆丱PERATION銆丳AGE銆丱N_OFF_CONFIG 鍜?VOUT_COMMAND
  鍛戒护鐨勫啓鍏ャ€?- 2锛氱姝㈡墍鏈夊啓鍏ワ紝浠呭厑璁稿 WRITE_PROTECT銆丱PERATION 鍜?PAGE 鍛戒护鐨勫啓鍏ャ€?- 3锛氱姝㈡墍鏈夊啓鍏ワ紝浠呭厑璁稿 WRITE_PROTECT 鍛戒护鐨勫啓鍏ャ€傛敞鎰忥紝淇濇姢搴斿寘鍚?PAGE 瀵勫瓨鍣ㄣ€傚浜庡椤?  鑺墖锛岃嫢鑺墖涓ユ牸閬靛惊 PMBus 瑙勮寖锛岃繖鍙兘浼氭湁闂锛屽洜涓哄畠浼氶樆姝㈣姱鐗囧垏鎹㈡椿鍔ㄩ〉闈€?