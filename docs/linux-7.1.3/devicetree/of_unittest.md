
## Open Firmware Devicetree 鍗曞厓娴嬭瘯


浣滆€咃細Gaurav Minocha <gaurav.minocha.os@gmail.com>

## 1. 寮曡█


鏈枃妗ｈ鏄庢墽琛?OF 鍗曞厓娴嬭瘯鎵€闇€鐨勬祴璇曟暟鎹?
濡備綍鍔ㄦ€佸湴闄勫姞鍒版椿鍔ㄦ爲锛坙ive tree锛変笂锛岃€屼笌鏈哄櫒鐨?
浣撶郴缁撴瀯鏃犲叧銆?

寤鸿鍦ㄧ户缁箣鍓嶉槄璇讳互涓嬫枃妗ｃ€?

(1) Documentation/devicetree/usage-model.rst
(2) http://www.devicetree.org/Device_Tree_Usage

OF Selftest 鏃ㄥ湪娴嬭瘯鎻愪緵缁欒澶囬┍鍔ㄥ紑鍙戣€呯殑鎺ュ彛锛坕nclude/linux/of.h锛夛紝
浠ヤ粠涓幏鍙栬澶囦俊鎭瓑銆?
璇ユ帴鍙ｄ粠灞曞紑锛坲nflattened锛夌殑璁惧鏍戞暟鎹粨鏋勪腑鑾峰彇淇℃伅锛岃
澶у鏁拌澶囬┍鍔ㄥ湪鍚勭鐢ㄤ緥涓娇鐢ㄣ€?


## 2. 璇︾粏杈撳嚭锛圗XPECT锛?


濡傛灉 unittest 妫€娴嬪埌闂锛屽畠浼氬悜鎺у埗鍙版墦鍗拌鍛婃垨閿欒娑堟伅銆?
Unittest 杩樹細鏁呮剰浣跨敤閿欒鐨勬祴璇曟暟鎹潵瑙﹀彂鏉ヨ嚜鍏朵粬
鍐呮牳浠ｇ爜鐨勮鍛婂拰閿欒娑堟伅銆傝繖瀵艰嚧浜?
娣锋穯锛氳瑙﹀彂鐨勬秷鎭┒绔熸槸娴嬭瘯鐨?
棰勬湡缁撴灉锛岃繕鏄瓨鍦ㄤ笌 unittest 鏃犲叧鐨勭湡姝ｉ棶棰樸€?

宸插悜 unittest 涓坊鍔?'EXPECT \ : text'锛堝紑濮嬶級鍜?'EXPECT / : text'锛堢粨鏉燂級娑堟伅锛?
浠ユ姤鍛婃煇涓鍛婃垨閿欒鏄鏈熶箣涓殑銆傚叾
涓紑濮嬫秷鎭湪瑙﹀彂璀﹀憡鎴栭敊璇箣鍓嶆墦鍗帮紝缁撴潫娑堟伅
鍦ㄨЕ鍙戜箣鍚庢墦鍗般€?

EXPECT 娑堟伅浼氬鑷存帶鍒跺彴杈撳嚭闈炲父鍢堟潅銆侀毦浠?
闃呰銆備负姝ゅ垱寤轰簡鑴氭湰 scripts/dtc/of_unittest_expect 鏉ヨ繃婊?
杩欎簺鍐椾綑淇℃伅锛屽苟楂樹寒鏄剧ず琚Е鍙戠殑璀﹀憡鍜岄敊璇?
涓庨鏈熻鍛婂拰閿欒涔嬮棿鐨勪笉鍖归厤銆傛洿澶氫俊鎭彲
閫氳繃 'scripts/dtc/of_unittest_expect --help' 鑾峰彇銆?


## 3. 娴嬭瘯鏁版嵁


璁惧鏍戞簮鏂囦欢锛坉rivers/of/unittest-data/testcases.dtso锛夊寘鍚?
鎵ц鑷姩鍖栧崟鍏冩祴璇曟墍闇€鐨?
```

    drivers/of/unittest-data/tests-*.dtsi

```
閽堝 testcases.dtso 涓墍鍖呭惈鐨?Device Tree Source Include 鏂囦欢锛?dtsi锛夌殑

褰撳唴鏍稿湪鍚敤 CONFIG_OF_UNITTEST 鐨勬儏鍐典笅鏋勫缓鏃讹紝浼氫娇鐢ㄤ互涓?make
```

    $(obj)/%.dtbo: $(src)/%.dtso $(DTC) FORCE
	    $(call if_changed_dep,dtc)

```
灏?DT 婧愭枃浠讹紙testcases.dtso锛夌紪璇戜负浜岃繘鍒?blob
锛坱estcases.dtbo锛夛紝涔熺О涓烘墎骞冲寲 DT锛坒lattened DT锛夈€?

涔嬪悗锛屼娇鐢ㄤ互涓嬭鍒欏皢涓婅堪浜岃繘鍒?blob 鍖呰涓?
```

    $(obj)/%.dtbo.S: $(obj)/%.dtbo FORCE
	    $(call if_changed,wrap_S_dtb)

```
璇ユ眹缂栨枃浠惰缂栬瘧涓虹洰鏍囨枃浠讹紙testcases.dtbo.o锛夛紝骞?
閾炬帴杩涘唴鏍搁暅鍍忋€?


### 3.1 娣诲姞娴嬭瘯鏁版嵁


灞曞紑鐨勮澶囨爲缁撴瀯锛?

灞曞紑鐨勮澶囨爲鐢变互鏍戝舰杩炴帴鐨?device_node 缁勬垚锛?
```

    // following struct members are used to construct the tree
    struct device_node {
	...
	struct  device_node *parent;
	struct  device_node *child;
	struct  device_node *sibling;
	...
    };

```
鍥?1 鎻忚堪浜嗘満鍣ㄥ睍寮€璁惧鏍戠殑閫氱敤缁撴瀯锛?
浠呰€冭檻瀛愯妭鐐逛笌鍏勫紵鑺傜偣鎸囬拡銆傝繕瀛樺湪鍙︿竴涓寚閽?
`*parent`锛岀敤浜庡弽鍚戦亶鍘嗘爲銆傚洜姝わ紝鍦?
鐗瑰畾灞傜骇涓婏紝瀛愯妭鐐逛笌鎵€鏈夊厔寮熻妭鐐归兘浼氭湁涓€涓寚鍚?
鍏叡鑺傜偣鐨勭埗鎸囬拡锛堜緥濡?child1銆乻ibling2銆乻ibling3銆乻ibling4 鐨?
```

    root ('/')
    |
    child1 -> sibling2 -> sibling3 -> sibling4 -> null
    |         |           |           |
    |         |           |          null
    |         |           |
    |         |        child31 -> sibling32 -> null
    |         |           |          |
    |         |          null       null
    |         |
    |      child21 -> sibling22 -> sibling23 -> null
    |         |          |            |
    |        null       null         null
    |
    child11 -> sibling12 -> sibling13 -> sibling14 -> null
    |           |           |            |
    |           |           |           null
    |           |           |
    null        null       child131 -> null
			    |
			    null

```
鍥?1锛氬睍寮€璁惧鏍戠殑閫氱敤缁撴瀯


鍦ㄦ墽琛?OF unittest 涔嬪墠锛岄渶瑕佸皢娴嬭瘯鏁版嵁闄勫姞鍒?
鏈哄櫒鐨勮澶囨爲锛堝鏋滃瓨鍦級銆傚洜姝わ紝褰撹皟鐢?selftest_data_add() 鏃讹紝
瀹冮鍏堣鍙栭摼鎺ヨ繘鍐呮牳闀滃儚鐨勫睍寮€璁惧鏍戞暟鎹紝
```

    __dtb_testcases_begin - address marking the start of test data blob
    __dtb_testcases_end   - address marking the end of test data blob

```
鍏舵锛屽畠璋冪敤 of_fdt_unflatten_tree() 鏉ュ睍寮€锛坲nflatten锛?
blob銆傛渶鍚庯紝濡傛灉鏈哄櫒鐨勮澶囨爲锛堝嵆 live tree锛夊瓨鍦紝
鍒欏畠灏嗗睍寮€鍚庣殑娴嬭瘯鏁版嵁鏍戦檮鍔犲埌 live tree锛涘惁鍒?
瀹冨皢鑷韩浣滀负 live 璁惧鏍戦檮鍔犮€?

attach_node_and_children() 浣跨敤 of_attach_node() 灏嗚妭鐐归檮鍔犲埌
live tree锛屽涓嬫墍杩般€備负璇存槑杩欎竴鐐癸紝涓嬮潰鎻忚堪鐨勬祴璇曟暟鎹爲
```

    root ('/')
	|
    testcase-data
	|
    test-child0 -> test-sibling1 -> test-sibling2 -> test-sibling3 -> null
	|               |                |                |
    test-child01      null             null             null


```
鍥?2锛氳闄勫姞鍒?live tree 鐨勭ず渚嬫祴璇曟暟鎹爲銆?

鏍规嵁涓婅堪鍦烘櫙锛宭ive tree 宸茬粡瀛樺湪锛屽洜姝ゆ棤闇€
闄勫姞鏍癸紙'/'锛夎妭鐐广€傛墍鏈夊叾浠栬妭鐐归€氳繃璋冪敤
姣忎釜鑺傜偣涓婄殑 of_attach_node() 鏉ラ檮鍔犮€?

鍦?of_attach_node() 鍑芥暟涓紝鏂拌妭鐐逛綔涓虹粰瀹氱埗鑺傜偣
鐨勫瓙鑺傜偣闄勫姞鍒?live tree銆備絾鏄紝濡傛灉鐖惰妭鐐瑰凡鏈夊瓙鑺傜偣锛屽垯鏂拌妭鐐?
浼氭浛鎹㈠綋鍓嶅瓙鑺傜偣锛屽苟灏嗗叾鍙樹负鑷繁鐨勫厔寮熻妭鐐广€傚洜姝わ紝褰撳皢涓婅堪
娴嬭瘯鏁版嵁鑺傜偣闄勫姞鍒颁笂闈㈢殑 live tree锛堝浘 1锛夋椂锛屾渶缁堢粨鏋勪负
```

    root ('/')
    |
    testcase-data -> child1 -> sibling2 -> sibling3 -> sibling4 -> null
    |               |          |           |           |
    (...)             |          |           |          null
		    |          |         child31 -> sibling32 -> null
		    |          |           |           |
		    |          |          null        null
		    |          |
		    |        child21 -> sibling22 -> sibling23 -> null
		    |          |           |            |
		    |         null        null         null
		    |
		    child11 -> sibling12 -> sibling13 -> sibling14 -> null
		    |          |            |            |
		    null       null          |           null
					    |
					    child131 -> null
					    |
					    null
    -----------------------------------------------------------------------

    root ('/')
    |
    testcase-data -> child1 -> sibling2 -> sibling3 -> sibling4 -> null
    |               |          |           |           |
    |             (...)      (...)       (...)        null
    |
    test-sibling3 -> test-sibling2 -> test-sibling1 -> test-child0 -> null
    |                |                   |                |
    null             null                null         test-child01


```
鍥?3锛氶檮鍔犳祴璇曟暟鎹悗鐨?live 璁惧鏍戠粨鏋勩€?


缁嗗績鐨勮鑰呬細娉ㄦ剰鍒帮紝test-child0 鑺傜偣鍙樻垚浜?
涓庡厛鍓嶇粨鏋勶紙鍥?2锛夌浉姣旂殑鏈€鍚庝竴涓厔寮熻妭鐐广€傚湪闄勫姞绗竴涓?
test-child0 涔嬪悗锛岄檮鍔?test-sibling1 浼氬皢瀛愯妭鐐?
锛堝嵆 test-child0锛夋帹涓哄厔寮熻妭鐐癸紝骞朵娇鑷韩鎴愪负瀛愯妭鐐癸紝
濡備笂鎵€杩般€?

濡傛灉鍙戠幇閲嶅鑺傜偣锛堝嵆瀛樺湪鍏锋湁鐩稿悓 full_name 灞炴€х殑鑺傜偣
宸茬粡瀛樺湪浜?live tree 涓級锛屽垯璇ヨ妭鐐逛笉浼氳闄勫姞锛岃€屾槸灏嗗叾
灞炴€ч€氳繃璋冪敤鍑芥暟
update_node_properties() 鏇存柊鍒?live tree 鐨勮妭鐐逛笂銆?


### 3.2 绉婚櫎娴嬭瘯鏁版嵁


涓€鏃︽祴璇曠敤渚嬫墽琛屽畬鎴愶紝灏变細璋冪敤 selftest_data_remove
浠ョЩ闄ゆ渶鍒濋檮鍔犵殑璁惧鑺傜偣锛堥鍏堝垎绂诲彾鑺傜偣锛?
鐒跺悗鍚戜笂绉婚櫎鐖惰妭鐐癸紝鏈€缁堢Щ闄?
鏁存５鏍戯級銆俿elftest_data_remove() 璋冪敤 detach_node_and_children()锛屽悗鑰呬娇鐢?
of_detach_node() 灏嗚妭鐐逛粠 live 璁惧鏍戜腑鍒嗙銆?

瑕佸垎绂讳竴涓妭鐐癸紝of_detach_node() 瑕佷箞鏇存柊缁欏畾鑺傜偣鐖惰妭鐐圭殑瀛愭寚閽?
涓哄叾鍏勫紵鑺傜偣锛岃涔堝皢鍓嶄竴涓厔寮熻妭鐐归檮鍔犲埌缁欏畾鑺傜偣鐨?
鍏勫紵鑺傜偣涓婏紝瑙嗘儏鍐佃€屽畾銆傚氨鏄繖鏍?:)

