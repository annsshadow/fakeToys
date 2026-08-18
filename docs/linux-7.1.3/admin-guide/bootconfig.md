

## Boot Configuration


:Author: Masami Hiramatsu <mhiramat@kernel.org>

## Overview


寮曞閰嶇疆锛坆oot configuration锛夋墿灞曚簡褰撳墠鐨勫唴鏍稿懡浠よ锛屼互渚垮湪鍐呮牳寮曞鏃朵互楂樻晥鐨勬柟寮忔敮鎸侀澶栫殑閿€兼暟鎹€傝繖鍏佽绠＄悊鍛樹紶閫掍竴涓粨鏋勫寲鐨勯敭鍊奸厤缃枃浠躲€?

## Config File Syntax


寮曞閰嶇疆鐨勮娉曟槸涓€绉嶇畝鍗曠殑缁撴瀯鍖栭敭鍊煎舰寮忋€傛瘡涓敭鐢变互鍙ョ偣杩炴帴鐨勫崟璇嶇粍鎴愶紝閿笌鍊间箣闂寸敤 `=` 杩炴帴銆傚€煎瓧绗︿覆蹇呴』鐢变笅鏂囨弿杩扮殑浠ヤ笅鍒嗛殧绗︿箣涓€缁堟銆?

姣忎釜閿崟璇嶅彧鑳藉寘鍚瓧姣嶃€佹暟瀛椼€佽繛瀛楃锛坄-`锛夋垨涓嬪垝绾匡紙`_`锛夈€傝€屾瘡涓€煎彧鑳藉寘鍚彲鎵撳嵃瀛楃鎴栫┖鏍硷紝鍒嗛殧绗﹂櫎澶栵紝渚嬪鍒嗗彿锛坄;`锛夈€佹崲琛岋紙`\n`锛夈€侀€楀彿锛坄,`锛夈€佷簳鍙凤紙`#`锛変笌鍙宠姳鎷彿锛坄}`锛夈€?

濡傛灉 `=` 涔嬪悗鍒板叾涓竴涓垎闅旂涔嬮棿浠呬负绌虹櫧瀛楃锛屽垯璇ラ敭琚祴浜堜竴涓┖鍊笺€?

瀵逛簬鏁扮粍锛屾暟缁勫€间互閫楀彿锛坄,`锛夊垎闅旓紝骞朵笖涓轰簡鍙鎬э紝鍏佽鍦ㄦ暟缁勫€间箣闂翠娇鐢ㄦ敞閲婁互鍙婃崲琛岋紙`\n`锛夈€傚洜姝わ紝鏁扮粍鐨勭涓€涓厓绱犲繀椤讳笌锛堥敭锛変綅浜庡悓涓€琛屼笂
```

  KEY[.WORD[...]] = VALUE[, VALUE2[...]][;]

```
涓庡唴鏍稿懡浠よ璇硶涓嶅悓锛岄€楀彿涓?`=` 鍛ㄥ洿鐨勭┖鐧藉瓧绗︼紙鍖呮嫭鍒惰〃绗︼級浼氳蹇界暐銆?

濡傛灉瑕佸湪鍊间腑浣跨敤杩欎簺鍒嗛殧绗︼紝鍙互浣跨敤鍙屽紩鍙凤紙`"VALUE"`锛夋垨鍗曞紩鍙凤紙`'VALUE'`锛夊皢鍏舵嫭璧锋潵銆傝娉ㄦ剰锛岃繖浜涘紩鍙锋棤娉曡杞箟銆?

鍙互瀛樺湪涓€涓病鏈夊€兼垨鍏锋湁绌哄€肩殑閿€傝繖绫婚敭鐢ㄤ簬妫€鏌ヨ閿槸鍚﹀瓨鍦紙绫讳技浜庡竷灏斿€硷級銆?

### Key-Value Syntax


寮曞閰嶇疆鏂囦欢璇硶鍏佽鐢ㄦ埛鍚堝苟閮ㄥ垎鐩稿悓鐨勫崟璇嶉敭
```

 foo.bar.baz = value1
 foo.bar.qux.quux = value2

```
```

 foo.bar {
    baz = value1
    qux.quux = value2
 }

```
```

 foo.bar { baz = value1; qux.quux = value2 }

```
鍦ㄨ繖涓ょ椋庢牸涓紝鐩稿悓鐨勯敭鍗曡瘝鍦ㄥ紩瀵兼椂瑙ｆ瀽鏃朵細鑷姩鍚堝苟銆傚洜姝や綘鍙互杩藉姞鐩镐技鐨勬爲鎴栭敭鍊笺€?

### Same-key Values


绂佹涓や釜鎴栨洿澶氱殑鍊兼垨鏁扮粍鍏变韩鍚屼竴涓敭銆?
```

 foo = bar, baz
 foo = qux  # !ERROR! we can not re-define same key

```
濡傛灉瑕佹洿鏂板€硷紝蹇呴』浣跨敤瑕嗙洊鎿嶄綔绗?
```

 foo = bar, baz
 foo := qux

```
鐒跺悗锛宍qux` 浼氳璧嬬粰 `foo` 閿€傝繖瀵逛簬閫氳繃娣诲姞锛堥儴鍒嗭級鑷畾涔夊紩瀵奸厤缃潵瑕嗙洊榛樿鍊奸潪甯告湁鐢紝鑰屾棤闇€瑙ｆ瀽榛樿寮曞閰嶇疆銆?

濡傛灉瑕佸皢鍊间綔涓烘暟缁勬垚鍛樿拷鍔犲埌鐜版湁閿笂锛?
```

 foo = bar, baz
 foo += qux

```
鍦ㄦ鎯呭喌涓嬶紝閿?`foo` 鎷ユ湁 `bar`銆乣baz` 鍜?`qux`銆?

姝ゅ锛屽瓙閿笌鍊煎彲浠ュ湪涓€涓埗閿笅鍏卞瓨銆?
```

 foo = value1
 foo.bar = value2
 foo := value3 # This will update foo's value.

```
娉ㄦ剰锛岀敱浜庢病鏈夎娉曞彲灏嗗師濮嬪€肩洿鎺ユ斁鍦紙涓€涓埗閿級涓?
```

 foo {
     bar = value1
     bar {
         baz = value2
         qux = value3
     }
 }

```
鍙﹀锛岄敭涓嬪€艰妭鐐圭殑椤哄簭鏄浐瀹氱殑銆傚鏋滄棦瀛樺湪鍊煎張瀛樺湪瀛愰敭锛屽垯鍊煎缁堟槸绗竴涓瓙鑺傜偣
```

 foo.bar = value1
 foo = value2

```
```

 foo = value2
 foo.bar = value1

```
### Comments


璇ラ厤缃娉曟帴鍙?shell 鑴氭湰椋庢牸鐨勬敞閲娿€備互浜曞彿锛?#"锛夊紑濮嬬洿鍒版崲琛岋紙"\n"锛夌殑娉ㄩ噴灏嗚蹇界暐銆?

```

 # comment line
 foo = value # value is set to foo.
 bar = 1, # 1st element
       2, # 2nd element
       3  # 3rd element

```
```

 foo = value
 bar = 1, 2, 3

```
娉ㄦ剰锛屼笉鑳藉湪鍊间笌鍒嗛殧绗︿箣闂存斁缃敞閲婃垨鎹㈣绗?
```

 key = 1 # comment
       ,2


```
## /proc/bootconfig


/proc/bootconfig 鏄紩瀵奸厤缃殑鐢ㄦ埛绌洪棿鎺ュ彛銆備笌 /proc/cmdline 涓嶅悓锛岃鏂囦欢鏄剧ず閿€奸鏍肩殑鍒楄〃銆?
```

 KEY[.WORDS...] = "[VALUE]"[,"VALUE2"...]


```
## Boot Kernel With a Boot Config


浣跨敤寮曞閰嶇疆寮曞鍐呮牳鏈変袱绉嶆柟寮忥細灏嗗紩瀵奸厤缃檮鍔犲埌 initrd 闀滃儚锛屾垨灏嗗叾鍐呭祵鍒板唴鏍告湰韬腑銆?

### Attaching a Boot Config to Initrd


鐢变簬寮曞閰嶇疆鏂囦欢榛樿闅?initrd 涓€璧峰姞杞斤紝瀹冧細琚互濉厖銆佸ぇ灏忋€佹牎楠屽拰浠ュ強 12 瀛楄妭榄旀暟锛坢agic word锛夌殑褰㈠紡杩藉姞鍒?initrd锛坕nitramfs锛夐暅鍍忔枃浠剁殑鏈熬锛屽涓嬫墍绀恒€?

[initrd][bootconfig][padding][size(le32)][checksum(le32)][#BOOTCONFIG\n]

澶у皬鍜屾牎楠屽拰瀛楁鍧囦负鏃犵鍙?32 浣嶅皬绔€笺€?

褰撳紩瀵奸厤缃娣诲姞鍒?initrd 闀滃儚鏃讹紝鏁翠釜鏂囦欢澶у皬浼氬榻愬埌 4 瀛楄妭銆備负濉ˉ绌洪殭锛屼細娣诲姞绌哄瓧绗︼紙`\0`锛夈€傚洜姝?`size` 涓哄紩瀵奸厤缃枃浠剁殑闀垮害鍔犱笂濉厖瀛楄妭銆?

Linux 鍐呮牳浼氳В鐮佸唴瀛樹腑 initrd 闀滃儚鐨勬渶鍚庝竴閮ㄥ垎浠ヨ幏鍙栧紩瀵奸厤缃暟鎹€傜敱浜庤繖绉?piggyback"锛堣儗璐熷紡锛夋柟娉曪紝鍙寮曞鍔犺浇绋嬪簭浼犻€掓纭殑 initrd 鏂囦欢澶у皬锛屽氨鏃犻渶鏇存敼鎴栨洿鏂板紩瀵煎姞杞界▼搴忓強鍐呮牳闀滃儚鏈韩銆備竾涓€寮曞鍔犺浇绋嬪簭浼犻€掍簡鏇村ぇ鐨勫ぇ灏忥紝鍐呮牳灏嗘棤娉曟壘鍒板紩瀵奸厤缃暟鎹€?

涓烘鎿嶄綔锛孡inux 鍐呮牳鍦?tools/bootconfig 涓嬫彁渚涗簡 `bootconfig` 鍛戒护锛屽厑璁哥鐞嗗憳搴旂敤鎴栧垹闄ら厤缃枃浠?
```

 # make -C tools/bootconfig

```
瑕佸皢浣犵殑寮曞閰嶇疆鏂囦欢娣诲姞鍒?initrd 闀滃儚锛屾寜濡備笅鏂瑰紡杩愯 bootconfig
```

 # tools/bootconfig/bootconfig -a your-config /boot/initrd.img-X.Y.Z

```
```

 # tools/bootconfig/bootconfig -d /boot/initrd.img-X.Y.Z

```
鐒跺悗鍦ㄦ甯哥殑鍐呮牳鍛戒护琛屼笂娣诲姞 "bootconfig"锛屼互鍛婄煡鍐呮牳鍦?initrd 鏂囦欢鏈熬鏌ユ壘寮曞閰嶇疆銆傛垨鑰咃紝鍦ㄧ紪璇戝唴鏍告椂閫変腑 `CONFIG_BOOT_CONFIG_FORCE` Kconfig 閫夐」銆?

### Embedding a Boot Config into Kernel


濡傛灉鏃犳硶浣跨敤 initrd锛屼綘涔熷彲浠ラ€氳繃 Kconfig 閫夐」灏嗗紩瀵奸厤缃枃浠跺唴宓屽埌鍐呮牳涓€傚湪杩欑鎯呭喌涓嬶紝浣犻渶瑕侀噸鏂扮紪璇戝唴鏍?
```

 CONFIG_BOOT_CONFIG_EMBED=y
 CONFIG_BOOT_CONFIG_EMBED_FILE="/PATH/TO/BOOTCONFIG/FILE"

```
`CONFIG_BOOT_CONFIG_EMBED_FILE` 闇€瑕佷竴涓寚鍚戝紩瀵奸厤缃枃浠剁殑缁濆璺緞锛屾垨鐩稿浜庢簮浠ｇ爜鏍?瀵硅薄鏍戠殑鐩稿璺緞銆傚唴鏍镐細灏嗗叾浣滀负榛樿寮曞閰嶇疆鍐呭祵銆?

涓庡皢寮曞閰嶇疆闄勫姞鍒?initrd 鏃朵竴鏍凤紝闇€瑕佸湪鍐呮牳鍛戒护琛屼笂浣跨敤 `bootconfig` 閫夐」鏉ュ惎鐢ㄥ唴宓岀殑寮曞閰嶇疆锛屾垨鑰呬篃鍙互鍦ㄧ紪璇戝唴鏍告椂閫変腑 `CONFIG_BOOT_CONFIG_FORCE` Kconfig 閫夐」銆?

璇锋敞鎰忥紝鍗充娇璁剧疆浜嗚閫夐」锛屼綘涔熷彲浠ョ敤闄勫姞鍒?initrd 鐨勫彟涓€涓紩瀵奸厤缃潵瑕嗙洊鍐呭祵鐨勫紩瀵奸厤缃€?

## Kernel parameters via Boot Config


闄や簡鍐呮牳鍛戒护琛屼箣澶栵紝寮曞閰嶇疆杩樺彲鐢ㄤ簬浼犻€掑唴鏍稿弬鏁般€備綅浜?`kernel` 閿笅鐨勬墍鏈夐敭鍊煎灏嗙洿鎺ヤ紶閫掔粰鍐呮牳鍛戒护琛屻€傛澶栵紝浣嶄簬 `init` 閿笅鐨勯敭鍊煎灏嗛€氳繃鍛戒护琛屼紶閫掔粰 init 杩涚▼銆傝繖浜涘弬鏁颁笌鐢ㄦ埛缁欏畾鐨勫唴鏍稿懡浠よ瀛楃涓叉寜涓嬭堪椤哄簭鎷兼帴锛屽洜姝ゅ懡浠よ鍙傛暟鍙互瑕嗙洊寮曞閰嶇疆鍙傛暟锛堣繖鍙栧喅浜庡悇瀛愮郴缁熷浣曞鐞嗗弬鏁?
```

 [bootconfig params][cmdline params] -- [bootconfig init params][cmdline init params]

```
```

 kernel {
   root = 01234567-89ab-cdef-0123-456789abcd
 }
 init {
  splash
 }

```
```

 root="01234567-89ab-cdef-0123-456789abcd" -- splash

```
```

 ro bootconfig -- quiet

```
```

 root="01234567-89ab-cdef-0123-456789abcd" ro bootconfig -- splash quiet


```
## Config File Limitation


鐩墠鏈€澶ч厤缃ぇ灏忎负 32KB锛屼笖鎬荤殑閿崟璇嶆暟锛堣€岄潪閿€兼潯鐩暟锛夊繀椤诲皯浜?1024 涓妭鐐广€傛敞鎰忥細杩欓噷鎸囩殑鏄妭鐐规暟鑰岄潪鏉＄洰鏁帮紝涓€涓潯鐩嚦灏戣娑堣€?2 涓妭鐐癸紙涓€涓敭鍗曡瘝鍜屼竴涓€硷級銆傚洜姝ょ悊璁轰笂鏈€澶氬彲鏈?512 涓敭鍊煎銆傚鏋滈敭骞冲潎鍖呭惈 3 涓崟璇嶏紝鍒欏彲鍖呭惈 256 涓敭鍊煎銆傚湪澶у鏁版儏鍐典笅锛岄厤缃」鏁伴噺浼氬皯浜?100 鏉′笖灏忎簬 8KB锛屽洜姝ゅ凡缁忚冻澶熴€傚鏋滆妭鐐规暟瓒呰繃 1024锛屽嵆浣挎枃浠跺ぇ灏忓皬浜?32KB锛岃В鏋愬櫒涔熶細杩斿洖閿欒銆傦紙娉ㄦ剰锛屾鏈€澶уぇ灏忎笉鍖呭惈濉厖鐢ㄧ殑绌哄瓧绗︺€傦級鏃犺濡備綍锛岀敱浜?bootconfig 鍛戒护鍦ㄥ皢寮曞閰嶇疆杩藉姞鍒?initrd 闀滃儚鏃朵細杩涜鏍￠獙锛岀敤鎴峰彲浠ュ湪寮曞鍓嶅氨娉ㄦ剰鍒拌繖涓€鐐广€?


## Bootconfig APIs


鐢ㄦ埛鍙互鏌ヨ鎴栭亶鍘嗛敭鍊煎锛屼篃鍙互閫氳繃鏌ユ壘鏍癸紙鍓嶇紑锛夐敭鑺傜偣鏉ユ壘鍒拌鑺傜偣涓嬬殑閿€笺€?

濡傛灉鎷ユ湁涓€涓敭瀛楃涓诧紝浣犲彲浠ヤ娇鐢?xbc_find_value() 閫氳繃璇ラ敭鐩存帴鏌ヨ鍊笺€傚鏋滄兂浜嗚В寮曞閰嶇疆涓瓨鍦ㄥ摢浜涢敭锛屽彲浠ヤ娇鐢?xbc_for_each_key_value() 鏉ラ亶鍘嗛敭鍊煎銆傛敞鎰忥紝璁块棶锛堟暟缁勫€硷級鏃堕渶瑕佷娇鐢?xbc_array_for_each_value()
```

 vnode = NULL;
 xbc_find_value("key.word", &vnode);
 if (vnode && xbc_node_is_array(vnode))
    xbc_array_for_each_value(vnode, value) {
      printk("%s ", value);
    }

```
濡傛灉鎯宠仛鐒︿簬甯︽湁鍓嶇紑瀛楃涓茬殑閿紝鍙互浣跨敤 xbc_find_node() 閫氳繃璇ュ墠缂€瀛楃涓叉煡鎵捐妭鐐癸紝骞朵娇鐢?xbc_node_for_each_key_value() 閬嶅巻璇ュ墠缂€鑺傜偣涓嬬殑閿€?

浣嗘渶鍏稿瀷鐨勭敤娉曟槸鑾峰彇鍓嶇紑涓嬬殑鍏峰悕鍊?
```

 root = xbc_find_node("key.prefix");
 value = xbc_node_find_value(root, "option", &vnode);
 ...
 xbc_node_for_each_array_value(root, "array-option", value, anode) {
    ...
 }

```
杩欎細璁块棶 "key.prefix.option" 鐨勫€间互鍙?"key.prefix.array-option" 鐨勬暟缁勩€?

涓嶉渶瑕佸姞閿侊紝鍥犱负鍦ㄥ垵濮嬪寲涔嬪悗锛岄厤缃彉涓哄彧璇汇€傚鏋滈渶瑕佷慨鏀癸紝蹇呴』澶嶅埗鍏ㄩ儴鏁版嵁涓庨敭銆?


## Functions and structures



