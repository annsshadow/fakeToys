## PHY 鎶借薄灞?


## 鐩殑


澶у鏁扮綉缁滆澶囬兘鐢变竴缁勫瘎瀛樺櫒鏋勬垚锛岃繖浜涘瘎瀛樺櫒鎻愪緵浜嗕笌 MAC 灞傜殑鎺ュ彛锛岃€?MAC 灞傞€氳繃 PHY 涓庣墿鐞嗚繛鎺ラ€氫俊銆侾HY 璐熻矗涓庣綉缁滆繛鎺ワ紙閫氬父鏄互澶綉绾跨紗锛夊彟涓€绔殑閾捐矾浼欎即鍗忓晢閾捐矾鍙傛暟锛屽苟鎻愪緵涓€涓瘎瀛樺櫒鎺ュ彛锛岃椹卞姩鑳藉纭畾閫夋嫨浜嗗摢浜涜缃€佷互鍙婇厤缃厑璁稿摢浜涜缃€?

铏界劧杩欎簺璁惧涓庣綉缁滆澶囦笉鍚岋紝骞朵笖瀵勫瓨鍣ㄩ伒寰爣鍑嗗竷灞€锛屼絾鎶?PHY 绠＄悊浠ｇ爜涓庣綉缁滈┍鍔ㄩ泦鎴愬湪涓€璧锋槸涓€绉嶅父瑙佸仛娉曘€傝繖瀵艰嚧浜嗗ぇ閲忓啑浣欎唬鐮併€傛澶栵紝鍦ㄥ甫鏈夊涓紙鏈夋椂宸紓寰堝ぇ锛夎繛鎺ュ埌鍚屼竴绠＄悊鎬荤嚎鐨勪互澶綉鎺у埗鍣ㄧ殑宓屽叆寮忕郴缁熶笂锛屽緢闅剧‘淇濆鎬荤嚎鐨勫畨鍏ㄤ娇鐢ㄣ€?

鐢变簬 PHY 鏄澶囷紝鑰岃闂畠浠殑绠＄悊鎬荤嚎瀹為檯涓婁篃鏄€荤嚎锛孭HY 鎶借薄灞傦紙PAL锛夊氨鎸夋瀵瑰緟瀹冧滑銆傝繖鏍峰仛鏃讹紝瀹冩湁浠ヤ笅鐩爣锛?

#. 鎻愰珮浠ｇ爜澶嶇敤
#. 鎻愰珮鏁翠綋浠ｇ爜鐨勫彲缁存姢鎬?
#. 鍔犲揩鏂扮綉缁滈┍鍔ㄤ互鍙婃柊绯荤粺鐨勫紑鍙戞椂闂?

鍩烘湰涓婏紝杩欎竴灞傛棬鍦ㄤ负 PHY 璁惧鎻愪緵涓€涓帴鍙ｏ紝璁╃綉缁滈┍鍔ㄧ紪鍐欒€呭敖鍙兘灏戝啓浠ｇ爜锛屽悓鏃朵粛鑳芥彁渚涘畬鏁寸殑鍔熻兘闆嗐€?

## MDIO 鎬荤嚎


澶у鏁扮綉缁滆澶囬€氳繃涓€鏉＄鐞嗘€荤嚎杩炴帴鍒?PHY銆備笉鍚岀殑璁惧浣跨敤涓嶅悓鐨勬€荤嚎锛堝敖绠℃湁浜涘叡浜€氱敤鎺ュ彛锛夈€備负浜嗗埄鐢?PAL锛屾瘡涓€荤嚎鎺ュ彛閮介渶瑕佷綔涓轰竴涓嫭绔嬬殑璁惧娉ㄥ唽銆?

```

	int write(struct mii_bus *bus, int mii_id, int regnum, u16 value);
	int read(struct mii_bus *bus, int mii_id, int regnum);

   mii_id is the address on the bus for the PHY, and regnum is the register
   number.  These functions are guaranteed not to be called from interrupt
   time, so it is safe for them to block, waiting for an interrupt to signal
   the operation is complete

```
#. 澶嶄綅鍑芥暟鏄彲閫夌殑銆傚畠鐢ㄤ簬灏嗘€荤嚎杩斿洖鍒板垵濮嬪寲鐘舵€併€?

#. 闇€瑕佷竴涓?probe 鍑芥暟銆傝繖涓嚱鏁板簲褰撹缃€荤嚎椹卞姩鎵€闇€鐨勪换浣曚笢瑗匡紝璁剧疆 mii_bus 缁撴瀯浣擄紝骞朵娇鐢?mdiobus_register 鍚?PAL 娉ㄥ唽銆傜被浼煎湴锛岃繕鏈変竴涓?remove 鍑芥暟鐢ㄤ簬鎾ら攢鎵€鏈夎繖浜涳紙浣跨敤 mdiobus_unregister锛夈€?

#. 鍍忎换浣曢┍鍔ㄤ竴鏍凤紝蹇呴』閰嶇疆 device_driver 缁撴瀯浣擄紝骞朵娇鐢?init 鍜?exit 鍑芥暟鏉ユ敞鍐岃椹卞姩銆?

#. 璇ユ€荤嚎杩樺繀椤诲湪鏌愬琚０鏄庝负涓€涓澶囷紝骞舵敞鍐屻€?

鍏充簬涓€涓┍鍔ㄥ浣曞疄鐜?mdio 鎬荤嚎椹卞姩鐨勭ず渚嬶紝璇峰弬瑙?drivers/net/ethernet/freescale/fsl_pq_mdio.c 浠ュ強鍏朵腑涓€涓敤鎴峰搴旂殑 DTS 鏂囦欢銆傦紙渚嬪 "git grep fsl,.*-mdio arch/powerpc/boot/dts/"锛?

## (RG)MII/鐢垫皵鎺ュ彛鑰冮噺


Reduced Gigabit Medium Independent Interface锛圧GMII锛岀簿绠€鍗冨厗濯掍綋鐙珛鎺ュ彛锛夋槸涓€涓?12 閽堢殑鐢典俊鍙锋帴鍙ｏ紝浣跨敤鍚屾鐨?125Mhz 鏃堕挓淇″彿鍜岃嫢骞叉暟鎹嚎銆傜敱浜庤繖涓€璁捐鍐冲畾锛屽繀椤诲湪鏃堕挓绾匡紙RXC 鎴?TXC锛変笌鏁版嵁绾夸箣闂村姞鍏?1.5ns 鍒?2ns 鐨勫欢杩燂紝浠ヤ究 PHY锛堟椂閽熸帴鏀剁锛夋湁瓒冲澶х殑寤虹珛涓庝繚鎸佹椂闂存潵姝ｇ‘閲囨牱鏁版嵁绾裤€侾HY 搴撴彁渚涗簡涓嶅悓绫诲瀷鐨?PHY_INTERFACE_MODE_RGMII* 鍊硷紝璁?PHY 椹卞姩锛堜互鍙婂彲閫夌殑 MAC 椹卞姩锛夊疄鐜版墍闇€鐨勫欢杩熴€俻hy_interface_t 鐨勫彇鍊煎繀椤讳粠 PHY 璁惧鑷韩鐨勮搴︽潵鐞嗚В锛岀敱姝ゅ緱鍑轰互涓嬪唴瀹癸細

- PHY_INTERFACE_MODE_RGMII锛歅HY 涓嶈礋璐ｈ嚜琛屾彃鍏ヤ换浣曞唴閮ㄥ欢杩燂紝瀹冨亣璁捐涔堟槸浠ュお缃?MAC锛堝鏋滄湁鑳藉姏锛夎涔?PCB 璧扮嚎鎻掑叆浜嗘纭殑 1.5-2ns 寤惰繜

- PHY_INTERFACE_MODE_RGMII_TXID锛歅HY 搴斿綋涓?PHY 璁惧澶勭悊鐨勫彂閫佹暟鎹嚎锛圱XD[3:0]锛夋彃鍏ュ唴閮ㄥ欢杩?

- PHY_INTERFACE_MODE_RGMII_RXID锛歅HY 搴斿綋涓?PHY 璁惧澶勭悊鐨勬帴鏀舵暟鎹嚎锛圧XD[3:0]锛夋彃鍏ュ唴閮ㄥ欢杩?

- PHY_INTERFACE_MODE_RGMII_ID锛歅HY 搴斿綋涓鸿繘鍑?PHY 璁惧鐨勫彂閫?***鍜?***鎺ユ敹鏁版嵁绾块兘鎻掑叆鍐呴儴寤惰繜

鍙鍙兘锛屽嚭浜庝互涓嬪師鍥犲簲浣跨敤 PHY 渚х殑 RGMII 寤惰繜锛?

- PHY 璁惧鍙兘鍦ㄦ帴鏀?鍙戦€佷晶寤惰繜鐨勬寚瀹氫笂鎻愪緵浜氱撼绉掔骇绮掑害锛堜緥濡傦細0.5銆?.0銆?.5ns锛夈€傝繖绉嶇簿搴﹀彲鑳芥槸澶勭悊 PCB 璧扮嚎闀垮害宸紓鎵€蹇呴渶鐨?

- PHY 璁惧閫氬父閫傜敤浜庡ぇ鑼冨洿鐨勫簲鐢紙宸ヤ笟銆佸尰鐤椼€佹苯杞︹€︹€︼級锛屽苟涓斿畠浠湪娓╁害/鍘嬪姏/鐢靛帇鑼冨洿鍐呮彁渚涙亽瀹氫笖鍙潬鐨勫欢杩?

- PHYLIB 涓殑 PHY 璁惧椹卞姩鏈川涓婂彲澶嶇敤锛岃兘澶熸纭厤缃寚瀹氱殑寤惰繜锛屽彲浣挎洿澶氬叿鏈夌被浼煎欢杩熼渶姹傜殑璁捐姝ｇ‘宸ヤ綔

瀵逛簬 PHY 鏃犳硶鎻愪緵璇ュ欢杩熴€佷絾浠ヤ互澶綉 MAC 椹卞姩鑳藉鎻愪緵鐨勬儏鍐碉紝姝ｇ‘鐨?phy_interface_t 鍊煎簲褰撴槸 PHY_INTERFACE_MODE_RGMII锛屽苟涓斾互澶綉 MAC 椹卞姩搴斿綋琚纭厤缃紝浠ヤ究浠?PHY 璁惧鐨勮搴︽彁渚涙墍闇€鐨勫彂閫佸拰/鎴栨帴鏀朵晶寤惰繜銆傚弽涔嬶紝濡傛灉浠ュお缃?MAC 椹卞姩鏌ョ湅 phy_interface_t 鍊硷紝瀵逛簬闄?PHY_INTERFACE_MODE_RGMII 涔嬪鐨勪换浣曟ā寮忥紝瀹冨簲褰撶‘淇?MAC 绾х殑寤惰繜琚鐢ㄣ€?

濡傛灉浠ュお缃?MAC 鍜?PHY 閮芥棤娉曟寜 RGMII 鏍囧噯鐨勫畾涔夋彁渚涙墍闇€鐨勫欢杩燂紝鍙兘鏈変互涓嬪嚑绉嶉€夋嫨锛?

- 鏌愪簺 SoC 鍙兘鎻愪緵涓€涓紩鑴氱剨鐩?mux/鎺у埗鍣紝鑳藉閰嶇疆缁欏畾涓€缁勫紩鑴氱殑椹卞姩寮哄害銆佸欢杩熷拰鐢靛帇锛涘畠鍙兘鏄彃鍏ラ鏈?2ns RGMII 寤惰繜鐨勫悎閫傞€夐」銆?

- 淇敼 PCB 璁捐浠ュ寘鍚浐瀹氬欢杩燂紙渚嬪锛氫娇鐢ㄤ笓闂ㄨ璁＄殑铔囧舰璧扮嚎锛夛紝杩欏彲鑳藉畬鍏ㄤ笉闇€瑕佽蒋浠堕厤缃€?

### RGMII 寤惰繜涓嶅尮閰嶇殑甯歌闂
褰撲互澶綉 MAC 涓?PHY 涔嬮棿瀛樺湪 RGMII 寤惰繜涓嶅尮閰嶆椂锛岃繖鏋佸彲鑳藉鑷存椂閽熶笌鏁版嵁绾夸俊鍙峰湪 PHY 鎴?MAC 瀵硅繖浜涗俊鍙烽噰鏍枫€佸皢鍏惰浆鎹负閫昏緫 1 鎴?0 鐘舵€佸苟閲嶅缓鏀跺彂鏁版嵁鏃朵笉绋冲畾銆傚吀鍨嬬棁鐘跺寘鎷細

- 鍙戦€?鎺ユ敹閮ㄥ垎宸ヤ綔锛屽苟涓旇瀵熷埌棰戠箒鎴栧伓鍙戠殑涓㈠寘

- 浠ュお缃?MAC 鍙兘鎶ュ憡閮ㄥ垎鎴栧叏閮ㄥ叆鍚戞暟鎹寘甯︽湁 FCS/CRC 閿欒锛屾垨鑰呯洿鎺ユ妸瀹冧滑鍏ㄩ儴涓㈠純

- 鍒囨崲鍒拌緝浣庨€熺巼锛堝 10/100Mbits/sec锛夋椂闂娑堝け锛堝洜涓烘鏃舵湁瓒冲鐨勫缓绔?淇濇寔鏃堕棿锛?

## 杩炴帴鍒?PHY


鍦ㄥ惎鍔ㄨ繃绋嬩腑鐨勬煇涓椂鍒伙紝缃戠粶椹卞姩闇€瑕佸湪 PHY 璁惧涓庣綉缁滆澶囦箣闂村缓绔嬭繛鎺ャ€傛鏃讹紝PHY 鐨勬€荤嚎鍜岄┍鍔ㄩ兘闇€瑕佸凡缁忓姞杞藉ソ锛屼互渚夸负杩炴帴鍋氬ソ鍑嗗銆傛鏃舵湁鍑犵杩炴帴鍒?PHY 鐨勬柟寮忥細

#. PAL 澶勭悊涓€鍒囷紝骞朵笖鍙湪閾捐矾鐘舵€佸彉鍖栨椂璋冪敤缃戠粶椹卞姩锛屼互渚垮畠鑳藉鍋氬嚭鍙嶅簲銆?

#. PAL 澶勭悊闄や腑鏂箣澶栫殑涓€鍒囷紙閫氬父鏄洜涓烘帶鍒跺櫒鎷ユ湁涓柇瀵勫瓨鍣級銆?

#. PAL 澶勭悊涓€鍒囷紝浣嗘瘡绉掍笌椹卞姩鏍稿涓€娆★紝璁╃綉缁滈┍鍔ㄥ湪 PAL 涔嬪墠鍏堝浠讳綍鍙樺寲鍋氬嚭鍙嶅簲銆?

#. PAL 浠呬綔涓哄嚱鏁板簱浣跨敤锛岀敱缃戠粶璁惧鎵嬪姩璋冪敤鍑芥暟鏉ユ洿鏂扮姸鎬併€佸苟閰嶇疆 PHY銆?

## 璁?PHY 鎶借薄灞傚鐞嗕竴鍒?


濡傛灉浣犻€夋嫨閫夐」 1锛堝笇鏈涙瘡涓┍鍔ㄩ兘鑳藉姝わ紝浣嗗涓嶈兘鐨勯┍鍔ㄤ粛鏈夌敤澶勶級锛岃繛鎺ュ埌 PHY 寰堢畝鍗曪細

棣栧厛锛屼綘闇€瑕佷竴涓嚱鏁版潵瀵归摼璺姸鎬佺殑鍙樺寲鍋氬嚭鍙嶅簲銆傝繖涓?
```

	static void adjust_link(struct net_device *dev);

```
鎺ヤ笅鏉ワ紝浣犻渶瑕佺煡閬撹繛鎺ュ埌姝よ澶囩殑 PHY 鐨勮澶囧悕銆傚悕瀛楃湅璧锋潵绫讳技 "0:00"锛屽叾涓涓€涓暟瀛楁槸鎬荤嚎 id锛岀浜屼釜鏄鎬荤嚎涓?PHY 鐨勫湴鍧€銆傞€氬父锛屾€荤嚎璐熻矗浣垮叾 ID 鍞竴銆?

```

	phydev = phy_connect(dev, phy_name, &adjust_link, interface);

```
**phydev** 鏄竴涓寚鍚戜唬琛?PHY 鐨?phy_device 缁撴瀯浣撶殑鎸囬拡銆傚鏋?phy_connect 鎴愬姛锛屽畠浼氳繑鍥炶鎸囬拡銆傝繖閲岀殑 dev 鏄寚鍚戜綘鐨?net_device 鐨勬寚閽堛€備竴鏃﹀畬鎴愶紝杩欎釜鍑芥暟灏变細鍚姩 PHY 鐨勮蒋浠剁姸鎬佹満锛屽苟鍦?PHY 鏈変腑鏂椂涓哄叾娉ㄥ唽涓柇銆俻hydev 缁撴瀯浣撲細琚～鍏ュ叧浜庡綋鍓嶇姸鎬佺殑淇℃伅锛屽敖绠℃鏃?PHY 灏氭湭鐪熸鍙繍琛屻€?

PHY 鐗瑰畾鐨勬爣蹇楀簲褰撳湪璋冪敤 phy_connect() 涔嬪墠璁剧疆鍒?phydev->dev_flags 涓紝浠ヤ究搴曞眰 PHY 椹卞姩鑳藉妫€鏌ヨ繖浜涙爣蹇楀苟鎹鎵ц鐗瑰畾鎿嶄綔銆傚鏋滅郴缁熷 PHY/鎺у埗鍣ㄦ柦鍔犱簡纭欢闄愬埗銆佽€?PHY 闇€瑕佺煡閬撹繖浜涢檺鍒讹紝杩欏氨寰堟湁鐢ㄣ€?

**interface** 鏄竴涓?u32锛屾寚瀹氭帶鍒跺櫒涓?PHY 涔嬮棿浣跨敤鐨勮繛鎺ョ被鍨嬨€備緥濡?GMII銆丮II銆丷GMII 鍜?SGMII銆傚弬瑙佷笅鏂囩殑鈥淧HY 鎺ュ彛妯″紡鈥濄€傚畬鏁村垪琛ㄨ include/linux/phy.h銆?

鐜板湪鍙渶纭繚浠?phydev->supported 鍜?phydev->advertising 涓壀闄ゅ浣犵殑鎺у埗鍣ㄦ棤鎰忎箟鐨勫€硷紙涓€涓?10/100 鎺у埗鍣ㄥ彲鑳借繛鎺ュ埌涓€涓敮鎸佸崈鍏嗙殑 PHY锛屽洜姝や綘闇€瑕佸睆钄芥帀 SUPPORTED_1000baseT*锛夈€傝繖浜涗綅鍩熺殑瀹氫箟瑙?include/linux/ethtool.h銆傛敞鎰忎綘涓嶅簲 SET 浠讳綍浣嶏紝闄ら潪鏄?SUPPORTED_Pause 鍜?SUPPORTED_AsymPause 浣嶏紙瑙佷笅鏂囷級锛屽惁鍒?PHY 鍙兘杩涘叆涓嶅彈鏀寔鐨勭姸鎬併€?

鏈€鍚庯紝涓€鏃︽帶鍒跺櫒鍑嗗濂藉鐞嗙綉缁滄祦閲忥紝浣犲氨璋冪敤 phy_start(phydev)銆傝繖鍛婅瘔 PAL 浣犲凡缁忓氨缁紝骞堕厤缃?PHY 杩炴帴鍒扮綉缁溿€傚鏋滀綘缃戠粶椹卞姩鐨?MAC 涓柇涔熷鐞?PHY 鐘舵€佸彉鍖栵紝鍙渶鍦ㄨ皟鐢?phy_start 涔嬪墠鎶?phydev->irq 璁句负 PHY_MAC_INTERRUPT锛屽苟鍦ㄧ綉缁滈┍鍔ㄤ腑浣跨敤 phy_mac_interrupt()銆傚鏋滀綘涓嶆兂浣跨敤涓柇锛屾妸 phydev->irq 璁句负 PHY_POLL銆俻hy_start() 浼氬惎鐢?PHY 涓柇锛堣嫢閫傜敤锛夊苟鍚姩 phylib 鐘舵€佹満銆?

褰撲綘鎯虫柇寮€涓庣綉缁滅殑杩炴帴鏃讹紙鍗充究鍙槸鐭殏鏂紑锛夛紝浣犺皟鐢?phy_stop(phydev)銆傝繖涓嚱鏁颁篃浼氬仠姝?phylib 鐘舵€佹満骞剁鐢?PHY 涓柇銆?

## PHY 鎺ュ彛妯″紡


phy_connect() 绯诲垪鍑芥暟鎵€鎻愪緵鐨?PHY 鎺ュ彛妯″紡锛屽畾涔変簡 PHY 鎺ュ彛鐨勫垵濮嬭繍琛屾ā寮忋€傝繖骞朵笉淇濊瘉淇濇寔涓嶅彉锛涙湁浜?PHY 浼氭牴鎹崗鍟嗙粨鏋溿€佸湪鏃犻渶杞欢浠嬪叆鐨勬儏鍐典笅鍔ㄦ€佹敼鍙樺叾鎺ュ彛妯″紡銆?

涓嬮潰鎻忚堪鍏朵腑涓€浜涙帴鍙ｆā寮忥細

`PHY_INTERFACE_MODE_SMII`
    杩欐槸涓茶 MII锛屼互 125MHz 鏃堕挓杩愯锛屾敮鎸?100M 鍜?10M 閫熺巼銆?
    閮ㄥ垎缁嗚妭鍙弬瑙?
    https://opencores.org/ocsvn/smii/smii/trunk/doc/SMII.pdf

`PHY_INTERFACE_MODE_1000BASEX`
    杩欏畾涔変簡 802.3 鏍囧噯绗?36 鑺傛墍瑙勫畾鐨?1000BASE-X 鍗曢€氶亾 serdes 閾捐矾銆傝閾捐矾浠?1.25Gbaud 鐨勫浐瀹氭瘮鐗圭巼杩愯锛屼娇鐢?10B/8B 缂栫爜鏂规锛屼粠鑰屽緱鍒?1Gbps 鐨勫簳灞傛暟鎹巼銆傛暟鎹祦涓祵鍏ヤ簡涓€涓?16 浣嶆帶鍒跺瓧锛岀敤浜庝笌杩滅鍗忓晢鍙屽伐鍜屾殏鍋滄ā寮忋€傝繖涓嶅寘鎷€滃崌棰戔€濆彉浣擄紙濡?2.5Gbps 閫熺巼锛岃涓嬫枃锛夈€?

`PHY_INTERFACE_MODE_2500BASEX`
    杩欏畾涔変簡 1000BASE-X 鐨勪竴涓彉浣擄紝鍏舵椂閽熼€熷害鏄?802.3 鏍囧噯鐨?2.5 鍊嶏紝寰楀埌 3.125Gbaud 鐨勫浐瀹氭瘮鐗圭巼銆?

`PHY_INTERFACE_MODE_SGMII`
    杩欑敤浜?Cisco SGMII锛屽畠鏄?802.3 鏍囧噯鎵€瀹氫箟鐨?1000BASE-X 鐨勪竴绉嶄慨鏀广€係GMII 閾捐矾鐢卞崟鏉′互 1.25Gbaud 鍥哄畾姣旂壒鐜囪繍琛屻€侀噰鐢?10B/8B 缂栫爜鐨?serdes 閫氶亾缁勬垚銆傚簳灞傛暟鎹巼涓?1Gbps锛屾洿鎱㈢殑 100Mbps 鍜?10Mbps 閫熺巼閫氳繃瀵规瘡涓暟鎹鍙疯繘琛屽鍒舵潵瀹炵幇銆?02.3 鎺у埗瀛楄鏀逛綔浠栫敤锛岀敤浜庢妸鍗忓晢寰楀埌鐨勯€熷害鍜屽弻宸ヤ俊鎭粠 PHY 鍙戦€佺粰 MAC锛屽苟鐢?MAC 纭鏀跺埌銆傝繖涓嶅寘鎷€滃崌棰戔€濆彉浣擄紙濡?2.5Gbps 閫熺巼锛夈€?

    娉ㄦ剰锛氬湪鏌愪簺鎯呭舰涓嬶紝閾捐矾涓婄殑 SGMII 涓?1000BASE-X 閰嶇疆涓嶅尮閰嶄粛鑳芥垚鍔熶紶杈撴暟鎹紝浣?16 浣嶆帶鍒跺瓧涓嶄細琚纭В閲婏紝杩欏彲鑳藉鑷村弻宸ャ€佹殏鍋滄垨鍏朵粬璁剧疆鐨勪笉鍖归厤銆傝繖鍙栧喅浜?MAC 鍜?鎴?PHY 鐨勮涓恒€?

`PHY_INTERFACE_MODE_5GBASER`
    杩欐槸 IEEE 802.3 绗?129 鏉″畾涔夌殑 5GBASE-R 鍗忚銆傚畠涓庣 49 鏉″畾涔夌殑 10GBASE-R 鍗忚鐩稿悓锛屽敮涓€渚嬪鏄畠浠ヤ竴鍗婄殑棰戠巼杩愯銆傚畾涔夎鍙傞槄 IEEE 鏍囧噯銆?

`PHY_INTERFACE_MODE_10GBASER`
    杩欐槸 IEEE 802.3 绗?49 鏉″畾涔夌殑 10GBASE-R 鍗忚锛岀敤浜庡悇绉嶄笉鍚岀殑浠嬭川銆傚畾涔夎鍙傞槄 IEEE 鏍囧噯銆?

    娉ㄦ剰锛?0GBASE-R 鍙槸鍙互涓?XFI 鍜?SFI 涓€璧蜂娇鐢ㄧ殑鍗忚涔嬩竴銆俋FI 鍜?SFI 鍏佽鍦ㄥ崟鏉?SERDES 閫氶亾涓婁娇鐢ㄥ绉嶅崗璁紝骞朵笖鍦ㄤ富鏈?XFP/SFP 杩炴帴鍣ㄦ彃鍏ヤ富鏈哄悎瑙勬澘鏃讹紝杩樺畾涔変簡淇″彿鐨勭數姘旂壒鎬с€傚洜姝わ紝XFI 鍜?SFI 鏈韩骞朵笉鏄?PHY 鎺ュ彛绫诲瀷銆?

`PHY_INTERFACE_MODE_10GKR`
    杩欐槸 IEEE 802.3 绗?49 鏉″畾涔夌殑銆佸甫鏈夌 73 鏉¤嚜鍔ㄥ崗鍟嗙殑 10GBASE-R銆傛洿澶氫俊鎭鍙傞槄 IEEE 鏍囧噯銆?

    娉ㄦ剰锛氱敱浜庡巻鍙茬敤娉曪紝涓€浜?10GBASE-R 鐢ㄦ硶閿欒鍦颁娇鐢ㄤ簡杩欎釜瀹氫箟銆?

`PHY_INTERFACE_MODE_25GBASER`
    杩欐槸 IEEE 802.3 PCS 绗?107 鏉″畾涔夌殑 25GBASE-R 鍗忚銆傚叾 PCS 涓?10GBASE-R 鐩稿悓锛屽嵆浠?2.5 鍊嶉€熷害杩愯鐨?64B/66B 缂栫爜锛屽緱鍒?25.78125 Gbaud 鐨勫浐瀹氭瘮鐗圭巼銆傛洿澶氫俊鎭鍙傞槄 IEEE 鏍囧噯銆?

`PHY_INTERFACE_MODE_100BASEX`
    杩欏畾涔変簡 IEEE 802.3 绗?24 鏉°€傝閾捐矾浠?125Mpbs 鐨勫浐瀹氭暟鎹巼杩愯锛屼娇鐢?4B/5B 缂栫爜鏂规锛屽緱鍒?100Mpbs 鐨勫簳灞傛暟鎹巼銆?

`PHY_INTERFACE_MODE_QUSGMII`
    杩欏畾涔変簡 Cisco 鐨?Quad USGMII 妯″紡锛屽嵆 USGMII锛圲niversal SGMII锛夐摼璺殑 Quad 鍙樹綋銆傚畠涓?QSGMII 闈炲父鐩镐技锛屼絾浣跨敤 Packet Control Header锛圥CH锛屾暟鎹寘鎺у埗澶达級鑰岄潪 7 瀛楄妭鍓嶅鐮侊紝涓嶄粎鎼哄甫绔彛 id锛岃繕鎼哄甫鎵€璋撶殑鈥滄墿灞曗€濄€傝鑼冧腑杩勪粖涓烘鍞竴鏈夋枃妗ｈ杞界殑鎵╁睍鏄寘鍚椂闂存埑锛岀敤浜庢敮鎸?PTP 鐨?PHY銆傝繖绉嶆ā寮忎笌 QSGMII 涓嶅吋瀹癸紝浣嗗湪閾捐矾閫熺巼鍜屽崗鍟嗘柟闈㈡彁渚涚浉鍚岀殑鑳藉姏銆?

`PHY_INTERFACE_MODE_1000BASEKX`
    杩欐槸 IEEE 802.3 绗?36 鏉″畾涔夌殑銆佸甫鏈夌 73 鏉¤嚜鍔ㄥ崗鍟嗙殑 1000BASE-X銆傞€氬父瀹冧細涓庣 70 鏉?PMD 涓€璧蜂娇鐢ㄣ€備笌鐢ㄤ簬绗?38 鍜?39 鏉?PMD 鐨?1000BASE-X phy 妯″紡鐩告瘮锛岃繖绉嶆帴鍙ｆā寮忓叿鏈変笉鍚岀殑鑷姩鍗忓晢锛屽苟涓斿彧鏀寔鍏ㄥ弻宸ャ€?

`PHY_INTERFACE_MODE_PSGMII`
    杩欐槸 Penta SGMII 妯″紡锛岀被浼间簬 QSGMII锛屼絾瀹冩妸 5 鏉?SGMII 绾垮悎骞舵垚鍗曟潯閾捐矾锛岃€?QSGMII 鏄?4 鏉°€?

`PHY_INTERFACE_MODE_10G_QXGMII`
    琛ㄧず Cisco USXGMII 澶氱鍙ｉ摐鎺ュ彛鏂囨。鎵€瀹氫箟鐨?10G-QXGMII PHY-MAC 鎺ュ彛銆傚畠鍦ㄤ竴鏉?10.3125 GHz 鐨?SerDes 閫氶亾涓婃敮鎸?4 涓鍙ｏ紝姣忎釜绔彛鐨勯€熺巼涓?2.5G / 1G / 100M / 10M锛岄€氳繃绗﹀彿澶嶅埗瀹炵幇銆侾CS 鏈熸湜鏍囧噯鐨?USXGMII 鐮佸瓧銆?

`PHY_INTERFACE_MODE_MIILITE`
    闈炴爣鍑嗙殑銆佺畝鍖栫殑 MII 妯″紡锛屾病鏈変负 MII 瀹氫箟鐨?TXER銆丷XER銆丆RS 鍜?COL 淇″彿銆傜己灏?COL 淇″彿浣垮緱鍗婂弻宸ラ摼璺ā寮忎笉鍙兘锛屼絾骞朵笉浼氬共鎵?Broadcom锛堜互鍙婂叾浠栦袱绾夸互澶綉锛塒HY 涓婄殑 BroadR-Reach 閾捐矾妯″紡锛屽洜涓哄畠浠彧鏀寔鍏ㄥ弻宸ャ€?

## 鏆傚仠甯?/ 娴佹帶


闄や簡纭繚鍦?MII_ADVERTISE 涓缃?SUPPORTED_Pause 鍜?SUPPORTED_AsymPause 浣嶃€佷互鍚戦摼璺紮浼磋〃鏄庝互澶綉 MAC 鎺у埗鍣ㄦ敮鎸佹绫诲姛鑳戒箣澶栵紝PHY 骞朵笉鐩存帴鍙備笌娴佹帶/鏆傚仠甯с€傜敱浜庢祦鎺?鏆傚仠甯х殑鐢熸垚娑夊強浠ュお缃?MAC 椹卞姩锛屽缓璁椹卞姩閫氳繃鐩稿簲鍦拌缃?SUPPORTED_Pause 鍜?SUPPORTED_AsymPause 浣嶏紝鏉ュΕ鍠勬寚绀哄姝ょ被鐗规€х殑閫氬憡涓庢敮鎸併€傝繖鍙互鍦?phy_connect() 涔嬪墠鎴栦箣鍚庡畬鎴愶紝涔熷彲浠ユ槸瀹炵幇 **ethtool** 鐨?set_pauseparam 鐗规€х殑缁撴灉銆?

## 瀵嗗垏鍏虫敞 PAL


鏈夊彲鑳?PAL 鍐呯疆鐨勭姸鎬佹満闇€瑕佷竴鐐瑰府鍔╋紝鎵嶈兘璁╀綘鐨勭綉缁滆澶囧拰 PHY 淇濇寔姝ｇ‘鍚屾銆傚鏋滄槸杩欐牱锛屼綘鍙互鍦ㄨ繛鎺ュ埌 PHY 鏃舵敞鍐屼竴涓緟鍔╁嚱鏁帮紝瀹冧細鍦ㄧ姸鎬佹満瀵逛换浣曞彉鍖栧仛鍑哄弽搴斾箣鍓嶆瘡绉掕璋冪敤銆傝鍋氬埌杩欎竴鐐癸紝浣犻渶瑕佹墜鍔ㄨ皟鐢?phy_attach() 鍜?phy_prepare_link()锛岀劧鍚庢妸 phy_start_machine() 鐨勭浜屼釜鍙傛暟璁句负鎸囧悜浣犵殑鐗规畩澶勭悊鍑芥暟銆?

鐩墠杩樻病鏈夊叧浜庡浣曚娇鐢ㄨ繖涓€鍔熻兘鐨勭ず渚嬶紝骞朵笖鐢变簬浣滆€呮病鏈変换浣曚娇鐢ㄥ畠鐨勯┍鍔紙瀹冧滑閮戒娇鐢ㄩ€夐」 1锛夛紝瀵瑰畠鐨勬祴璇曚篃寰堟湁闄愩€傚洜姝?Caveat Emptor锛堜拱鑰呰嚜璐燂級銆?

## 鍏ㄩ儴鑷繁鍔ㄦ墜


PAL 鍐呯疆鐨勭姸鎬佹満鏈夊彲鑳芥棤娉曡窡韪?
PHY 涓庝綘鐨勭綉缁滆澶囦箣闂寸殑澶嶆潅浜や簰銆傚鏋滄槸杩欐牱锛屼綘鍙互绠€鍗曞湴璋冪敤 phy_attach()锛岃€屼笉璋冪敤 phy_start_machine 鎴?phy_prepare_link()銆傝繖鎰忓懗鐫€ phydev->state 瀹屽叏鐢变綘鏉ュ鐞嗭紙phy_start 鍜?phy_stop 浼氬湪鏌愪簺鐘舵€佷箣闂村垏鎹紝鎵€浠ヤ綘鍙兘闇€瑕侀伩寮€瀹冧滑锛夈€?

宸茬粡鍋氬嚭浜嗗姫鍔涳紝浠ョ‘淇濆湪娌℃湁鐘舵€佹満杩愯鐨勬儏鍐典笅涔熻兘璁块棶鏈夌敤鐨勫姛鑳斤紝骞朵笖杩欎簺鍑芥暟澶у婧愯嚜閭ｄ簺涓嶄笌澶嶆潅鐘舵€佹満浜や簰鐨勫嚱鏁般€傜劧鑰岋紝鍚屾牱锛岀洰鍓嶈繕娌℃湁鍋氬嚭鍦ㄤ笉杩愯鐘舵€佹満鐨勬儏鍐典笅杩涜娴嬭瘯鐨勫姫鍔涳紝鎵€浠ヤ娇鐢ㄨ€呭綋蹇冦€?

```

 int phy_read(struct phy_device *phydev, u16 regnum);
 int phy_write(struct phy_device *phydev, u16 regnum, u16 val);

```
绠€鍗曠殑璇?鍐欏師璇€傚畠浠皟鐢ㄦ€荤嚎鐨勮/鍐欏嚱鏁版寚閽堛€?
```

 void phy_print_status(struct phy_device *phydev);

```
涓€涓暣娲佸湴鎵撳嵃 PHY 鐘舵€佺殑渚挎嵎鍑芥暟銆?
```

 void phy_request_interrupt(struct phy_device *phydev);

```
璇锋眰 PHY 涓柇鐨?IRQ銆?
```

 struct phy_device * phy_attach(struct net_device *dev, const char *phy_id,
		                phy_interface_t interface);

```
鎶婁竴涓綉缁滆澶囪繛鎺ュ埌涓€涓壒瀹氱殑 PHY锛屽鏋滃湪鎬荤嚎鍒濆鍖栨湡闂存病鏈夋壘鍒伴┍鍔紝灏辨妸 PHY 缁戝畾鍒颁竴涓€氱敤椹卞姩銆?
```

 int phy_start_aneg(struct phy_device *phydev);

```
浣跨敤 phydev 缁撴瀯浣撳唴閮ㄧ殑鍙橀噺锛岃涔堥厤缃€氬憡骞堕噸缃嚜鍔ㄥ崗鍟嗭紝瑕佷箞绂佺敤鑷姩鍗忓晢锛屽苟閰嶇疆寮哄埗璁剧疆銆?
```

 static inline int phy_read_status(struct phy_device *phydev);

```
鐢?PHY 涓綋鍓嶈缃殑鏈€鏂颁俊鎭～鍏?phydev 缁撴瀯浣撱€?
```

 int phy_ethtool_ksettings_set(struct phy_device *phydev,
                               const struct ethtool_link_ksettings *cmd);

```
Ethtool 渚挎嵎鍑芥暟銆?
```

 int phy_mii_ioctl(struct phy_device *phydev,
                   struct mii_ioctl_data *mii_data, int cmd);

```
MII ioctl銆傛敞鎰忥紝濡傛灉浣犲啓鍏ュ儚 BMCR銆丅MSR銆丄DVERTISE 绛夊瘎瀛樺櫒锛岃繖涓嚱鏁颁細褰诲簳鎼炰贡鐘舵€佹満銆傛渶濂藉彧鎶婂畠鐢ㄤ簬鍐欏叆闈炴爣鍑嗐€佷笖涓嶄細瑙﹀彂閲嶆柊鍗忓晢鐨勫瘎瀛樺櫒銆?

## PHY 璁惧椹卞姩


鏈変簡 PHY 鎶借薄灞傦紝涓烘柊鐨?PHY 娣诲姞鏀寔灏辩浉褰撳鏄撱€傚湪鏌愪簺鎯呭喌涓嬶紝鏍规湰涓嶉渶瑕佸仛浠讳綍宸ヤ綔锛佺劧鑰岋紝璁稿 PHY 闇€瑕佷竴鐐光€滄墜鎶婃墜鈥濆紩瀵兼墠鑳藉惎鍔ㄨ繍琛屻€?

### 閫氱敤 PHY 椹卞姩


濡傛灉鐩爣 PHY 娌℃湁浠讳綍浣犳兂瑕佹敮鎸佺殑鍕樿銆佹€櫀鎴栫壒娈婄壒鎬э紝閭ｄ箞鏈€濂戒笉瑕佹坊鍔犳敮鎸侊紝鑰屾槸璁?PHY 鎶借薄灞傜殑閫氱敤 PHY 椹卞姩鏉ュ畬鎴愭墍鏈夊伐浣溿€?

### 缂栧啓 PHY 椹卞姩


濡傛灉浣犵‘瀹炶缂栧啓 PHY 椹卞姩锛岄鍏堣鍋氱殑鏄‘淇濆畠鑳戒笌鍚堥€傜殑 PHY 璁惧鍖归厤銆傝繖鏄湪鎬荤嚎鍒濆鍖栨湡闂达紝閫氳繃璇诲彇璁惧鐨?UID锛堝瓨鍌ㄥ湪瀵勫瓨鍣?2 鍜?3 涓級锛岀劧鍚庢妸瀹冧笌姣忎釜椹卞姩鐨?phy_id 瀛楁鎸変綅涓庯紙AND锛夋瘡涓┍鍔ㄧ殑
```

   static struct phy_driver dm9161_driver = {
         .phy_id         = 0x0181b880,
	 .name           = "Davicom DM9161E",
	 .phy_id_mask    = 0x0ffffff0,
	 ...
   }

```
鏉ヨ繘琛屾瘮杈冦€?

鎺ヤ笅鏉ワ紝浣犻渶瑕佹寚瀹氫綘鐨?PHY 璁惧鍜岄┍鍔ㄦ敮鎸佸摢浜涚壒鎬э紙閫熺巼銆佸弻宸ャ€佽嚜鍗忓晢绛夛級銆傚ぇ澶氭暟 PHY 鏀寔 PHY_BASIC_FEATURES锛屼絾浣犲彲浠ュ湪 include/mii.h 涓煡鎵惧叾浠栫壒鎬с€?

姣忎釜椹卞姩鐢辫嫢骞插嚱鏁版寚閽堢粍鎴愶紝杩欎簺鍦?include/linux/phy.h 鐨?phy_driver 缁撴瀯浣撲笅鏈夋枃妗ｈ鏄庛€?

鍏朵腑锛屽彧鏈?config_aneg 鍜?read_status 蹇呴』鐢遍┍鍔ㄤ唬鐮佽祴鍊笺€傚叾浣欓兘鏄彲閫夌殑銆傛澶栵紝搴斿敖鍙兘浣跨敤閫氱敤 PHY 椹卞姩鐨勮繖涓や釜鍑芥暟鐨勭増鏈細genphy_read_status 鍜?genphy_config_aneg銆傚鏋滃仛涓嶅埌锛屽緢鍙兘浣犲彧闇€瑕佸湪璋冪敤杩欎簺鍑芥暟涔嬪墠鍜屼箣鍚庢墽琛屼竴浜涙搷浣滐紝鍥犳浣犵殑鍑芥暟浼氬寘瑁呰繖浜涢€氱敤鍑芥暟銆?

娆㈣繋鏌ョ湅 drivers/net/phy/ 涓殑 Marvell銆丆icada 鍜?Davicom 椹卞姩浣滀负绀轰緥锛堝湪鎾板啓鏈枃鏃讹紝lxt 鍜?qsemi 椹卞姩灏氭湭琚祴璇曪級銆?

PHY 鐨?MMD 瀵勫瓨鍣ㄨ闂粯璁ょ敱 PAL 妗嗘灦澶勭悊锛屼絾濡傛灉鏈夐渶瑕侊紝涔熷彲浠ヨ鐗瑰畾鐨?PHY 椹卞姩瑕嗙洊銆傚鏋滀竴涓?PHY 鍦?MMD PHY 瀵勫瓨鍣ㄥ畾涔夎 IEEE 鏍囧噯鍖栦箣鍓嶅氨鍙戝竷鐢ㄤ簬鐢熶骇锛屽氨鍙兘鍑虹幇杩欑鎯呭喌銆傚ぇ澶氭暟鐜颁唬 PHY 閮借兘浣跨敤閫氱敤鐨?PAL 妗嗘灦鏉ヨ闂?PHY 鐨?MMD 瀵勫瓨鍣ㄣ€傝繖绉嶇敤娉曠殑涓€涓緥瀛愭槸 PHY 鎶借薄灞傚疄鐜扮殑鑺傝兘浠ュお缃戯紙Energy Efficient Ethernet锛夋敮鎸併€傚鏋?PHY 鏀寔 IEEE 鏍囧噯璁块棶鏈哄埗锛岃鏀寔浣跨敤 PAL 璁块棶 MMD 瀵勫瓨鍣ㄤ互杩涜 EEE 鏌ヨ鍜岄厤缃紱濡傛灉琚壒瀹?PHY 椹卞姩瑕嗙洊锛屼篃鍙互浣跨敤 PHY 鐗瑰畾鐨勮闂帴鍙ｃ€傚弬瑙?drivers/net/phy/ 涓殑 Micrel 椹卞姩锛屼簡瑙ｅ浣曞疄鐜拌繖涓€鐐广€?

## 鏉跨骇淇锛圔oard Fixups锛?


鏈夋椂锛屽钩鍙颁笌 PHY 涔嬮棿鐨勭壒瀹氫氦浜掗渶瑕佺壒娈婂鐞嗐€備緥濡傦紝鏀瑰彉 PHY 鏃堕挓杈撳叆鐨勪綅缃紝鎴栬€呬负鏁版嵁璺緞涓殑寤惰繜闂澧炲姞寤惰繜銆備负浜嗘敮鎸佹绫绘剰澶栨儏鍐碉紝PHY 灞傚厑璁稿钩鍙颁唬鐮佹敞鍐屽湪 PHY 琚媺璧凤紙鎴栭殢鍚庨噸缃級鏃惰繍琛岀殑淇绋嬪簭銆?

褰?PHY 灞傛媺璧蜂竴涓?PHY 鏃讹紝瀹冧細妫€鏌ユ槸鍚︿负瀹冩敞鍐屼簡浠讳綍淇绋嬪簭锛屽尮閰嶄緷鎹槸 UID锛堝寘鍚湪 PHY 璁惧鐨?phy_id 瀛楁涓級鍜屾€荤嚎鏍囪瘑绗︼紙鍖呭惈鍦?phydev->dev.bus_id 涓級銆備袱鑰呴兘蹇呴』鍖归厤锛屼笉杩囨彁渚涗簡涓や釜甯搁噺 PHY_ANY_ID 鍜?PHY_ANY_UID锛屽垎鍒綔涓烘€荤嚎 ID 鍜?UID 鐨勯€氶厤绗︺€?

褰撴壘鍒板尮閰嶆椂锛孭HY 灞備細璋冪敤涓庤淇绋嬪簭鍏宠仈鐨?run 鍑芥暟銆傝繖涓嚱鏁颁細浼犲叆涓€涓寚鍚戠浉鍏?phy_device 鐨勬寚閽堛€傚洜姝ゅ畠搴斿綋鍙搷浣滈偅涓?PHY銆?

```

 int phy_register_fixup_for_uid(u32 phy_uid, u32 phy_uid_mask,
		int (*run)(struct phy_device *));
 int phy_register_fixup_for_id(const char *phy_id,
		int (*run)(struct phy_device *));

```
## 鏍囧噯


IEEE 鏍囧噯 802.3锛欳SMA/CD 璁块棶鏂规硶涓庣墿鐞嗗眰瑙勮寖锛岀浜岄儴鍒嗭細
http://standards.ieee.org/getieee802/download/802.3-2008_section2.pdf

RGMII v1.3:
http://web.archive.org/web/20160303212629/http://www.hp.com/rnd/pdfs/RGMIIv1_3.pdf

RGMII v2.0:
http://web.archive.org/web/20160303171328/http://www.hp.com/rnd/pdfs/RGMIIv2_0_final_hp.pdf
