## Kconfig 瀹忚瑷€


### 姒傚康


鍏跺熀鏈€濇兂鍙?Make 鐨勫惎鍙戙€傚綋鎴戜滑鐪?Make 鏃讹紝浼氭敞鎰忓埌瀹冩湁鐐瑰儚鏄妸涓ょ璇█
鍚堜簩涓轰竴銆備竴绉嶈瑷€鎻忚堪鐢辩洰鏍囧拰鍏堝喅鏉′欢缁勬垚鐨勪緷璧栧浘銆傚彟涓€绉嶆槸鎵ц鏂囨湰鏇挎崲鐨?瀹忚瑷€銆?
杩欎袱涓瑷€闃舵涔嬮棿鏈夋竻鏅扮殑鍖哄垎銆備緥濡傦紝浣?```

    APP := foo
    SRC := foo.c
    CC := gcc

    $(APP): $(SRC)
            $(CC) -o $(APP) $(SRC)

```
瀹忚瑷€灏嗗彉閲忓紩鐢ㄦ浛鎹负灞曞紑鍚庣殑褰㈠紡锛?```

    foo: foo.c
            gcc -o foo foo.c

```
鐒跺悗锛孧ake 鍒嗘瀽渚濊禆鍥惧苟纭畾瑕佹洿鏂扮殑鐩爣銆?
Kconfig 涓殑鎬濊矾闈炲父鐩镐技鈥斺€斿彲浠ュ儚涓嬮潰杩欐牱鎻忚堪涓€涓?Kconfig
```

    CC := gcc

    config CC_HAS_FOO
            def_bool $(shell, $(srctree)/scripts/gcc-check-foo.sh $(CC))

```
Kconfig 涓殑瀹忚瑷€灏嗘簮鏂囦欢澶勭悊涓轰互涓嬪唴瀹?```

    config CC_HAS_FOO
            def_bool y

```
鐒跺悗锛孠config 杩涘叆姹傚€奸樁娈碉紝浠ヨВ鏋愮鍙烽棿鐨勪緷璧栧叧绯伙紝濡?kconfig-language.rst
涓墍杩般€?

### 鍙橀噺


涓?Make 涓竴鏍凤紝Kconfig 涓殑鍙橀噺鍏呭綋瀹忓彉閲忋€傚畯鍙橀噺琚€滃氨鍦扳€濆睍寮€锛屼骇鐢熶竴涓?鍙兘杩涗竴姝ヨ灞曞紑鐨勫瓧绗︿覆銆傝鑾峰彇鍙橀噺鐨勫€硷紝璇峰皢鍙橀噺鍚嶆嫭鍦?$( ) 涓€傚嵆浣挎槸
鍗曞瓧姣嶅彉閲忓悕涔熼渶瑕佹嫭鍙凤紱$X 鏄娉曢敊璇€?{CC} 杩欑鑺辨嫭鍙峰舰寮忎篃涓嶈鏀寔銆?
鍙橀噺鏈変袱绉嶇被鍨嬶細绠€鍗曞睍寮€鍙橀噺鍜岄€掑綊灞曞紑鍙橀噺銆?
绠€鍗曞睍寮€鍙橀噺浣跨敤 := 璧嬪€艰繍绠楃瀹氫箟銆傝鍙?Kconfig 鏂囦欢涓殑璇ヨ鏃讹紝鍏跺彸渚т細
绔嬪嵆灞曞紑銆?
閫掑綊灞曞紑鍙橀噺浣跨敤 = 璧嬪€艰繍绠楃瀹氫箟銆傚叾鍙充晶鍙槸鍘熸牱瀛樺偍涓哄彉閲忕殑鍊硷紝涓嶈繘琛屼换浣?灞曞紑銆傜浉鍙嶏紝灞曞紑鏄湪鍙橀噺琚娇鐢ㄦ椂杩涜鐨勩€?
杩樻湁鍙︿竴绉嶈祴鍊艰繍绠楃锛?= 鐢ㄤ簬鍚戝彉閲忚拷鍔犳枃鏈€傚鏋滃乏渚ф渶鍒濊瀹氫箟涓虹畝鍗曞彉閲忥紝
鍒?+= 鐨勫彸渚т細绔嬪嵆灞曞紑銆傚惁鍒欙紝鍏舵眰鍊艰寤惰繜銆?
```

  $(name,arg1,arg2,arg3)

```
浣犲彲浠ュ皢鍙傛暟鍖栧紩鐢ㄨ涓轰竴涓嚱鏁般€傦紙鏇村噯纭湴璇达紝鏄浉瀵逛簬涓嬫枃鍒楀嚭鐨勨€滃唴缃嚱鏁扳€?鑰岃█鐨勨€滅敤鎴峰畾涔夊嚱鏁扳€濄€傦級

鏈夌敤鐨勫嚱鏁板繀椤诲湪浣跨敤鏃跺睍寮€锛屽洜涓轰紶鍏ヤ笉鍚屽弬鏁版椂鍚屼竴鍑芥暟鐨勫睍寮€缁撴灉涓嶅悓銆傚洜姝わ紝
鐢ㄦ埛瀹氫箟鍑芥暟浣跨敤 = 璧嬪€艰繍绠楃瀹氫箟銆傚弬鏁板湪鍑芥暟浣撳畾涔変腑閫氳繃 $(1)銆?(2) 绛夊紩鐢ㄣ€?
浜嬪疄涓婏紝閫掑綊灞曞紑鍙橀噺鍜岀敤鎴峰畾涔夊嚱鏁板湪鍐呴儴鏄浉鍚岀殑銆傦紙鎹㈠彞璇濊锛屸€滃彉閲忊€濆氨鏄?鈥滈浂鍙傛暟鍑芥暟鈥濄€傦級褰撴垜浠箍涔夊湴璇粹€滃彉閲忊€濇椂锛屽畠鍖呭惈浜嗏€滅敤鎴峰畾涔夊嚱鏁扳€濄€?

### 鍐呯疆鍑芥暟


涓?Make 涓€鏍凤紝Kconfig 鎻愪緵鑻ュ共鍐呯疆鍑芥暟銆傛瘡涓嚱鏁版帴鍙楃壒瀹氭暟閲忕殑鍙傛暟銆?
鍦?Make 涓紝姣忎釜鍐呯疆鍑芥暟鑷冲皯鎺ュ彈涓€涓弬鏁般€侹config 鍏佽鍐呯疆鍑芥暟鎺ュ彈闆朵釜鍙傛暟锛?渚嬪 $(filename)銆?(lineno)銆備綘鍙互鎶婂畠浠湅浣溾€滃唴缃彉閲忊€濓紝浣嗚繖缁堢┒鍙槸鎴戜滑
鍙硶鐨勯棶棰樸€傝繖閲屾垜浠氨绉扳€滃唴缃嚱鏁扳€濓紝鐢ㄦ潵鎸囦唬鍘熺敓鏀寔鐨勫姛鑳姐€?
Kconfig 鐩墠鏀寔浠ヤ笅鍐呯疆鍑芥暟銆?
 - $(shell,command)

  鈥渟hell鈥濆嚱鏁版帴鍙楀崟涓弬鏁帮紝璇ュ弬鏁拌灞曞紑鍚庝紶閫掔粰瀛?shell 鎵ц銆傚懡浠ょ殑鏍囧噯
  杈撳嚭闅忓悗琚鍙栧苟浣滀负鍑芥暟鐨勫€艰繑鍥炪€傝緭鍑轰腑鐨勬瘡涓崲琛岀閮借鏇挎崲涓虹┖鏍笺€備换浣?  灏鹃儴鎹㈣绗﹂兘浼氳鍒犻櫎銆傛爣鍑嗛敊璇笉浼氳杩斿洖锛屼换浣曠▼搴忕殑閫€鍑虹姸鎬佷篃涓嶄細杩斿洖銆?
 - $(info,text)

  鈥渋nfo鈥濆嚱鏁版帴鍙楀崟涓弬鏁板苟灏嗗叾鎵撳嵃鍒?stdout銆傚叾姹傚€肩粨鏋滀负绌哄瓧绗︿覆銆?
 - $(warning-if,condition,text)

  鈥渨arning-if鈥濆嚱鏁版帴鍙椾袱涓弬鏁般€傚鏋?condition 閮ㄥ垎涓衡€測鈥濓紝text 閮ㄥ垎浼氳
  鍙戦€佸埌 stderr銆倀ext 涔嬪墠浼氬姞涓婂綋鍓?Kconfig 鏂囦欢鍚嶅拰褰撳墠琛屽彿銆?
 - $(error-if,condition,text)

  鈥渆rror-if鈥濆嚱鏁颁笌鈥渨arning-if鈥濈被浼硷紝浣嗗鏋?condition 閮ㄥ垎涓衡€測鈥濓紝瀹冧細
  绔嬪嵆缁堟瑙ｆ瀽銆?
 - $(filename)

  'filename' 涓嶆帴鍙楀弬鏁帮紝骞朵笖 $(filename) 琚睍寮€涓烘鍦ㄨ瑙ｆ瀽鐨勬枃浠跺悕銆?
 - $(lineno)

  'lineno' 涓嶆帴鍙楀弬鏁帮紝骞朵笖 $(lineno) 琚睍寮€涓烘鍦ㄨ瑙ｆ瀽鐨勮鍙枫€?

### Make 涓?Kconfig 瀵规瘮


Kconfig 閲囩敤绫讳技 Make 鐨勫畯璇█锛屼絾鍑芥暟璋冪敤璇硶鐣ユ湁涓嶅悓銆?
```

  $(func-name arg1,arg2,arg3)

```
鍑芥暟鍚嶄笌绗竴涓弬鏁颁箣闂寸敤涓€涓垨澶氫釜绌虹櫧鍒嗛殧銆傜劧鍚庯紝绗竴涓弬鏁板墠闈㈢殑绌虹櫧浼氳
鍘婚櫎锛岃€屽叾瀹冨弬鏁颁腑鐨勭┖鐧戒細琚繚鐣欍€備綘闇€瑕佺敤鏌愮鎶€宸ф潵璁╃涓€涓弬鏁颁互绌烘牸寮€澶淬€?渚嬪锛屽鏋滀綘鎯宠
```

  empty :=
  space := $(empty) $(empty)
  $(info $(space)$(space)hello)

```
Kconfig 浠呬娇鐢ㄩ€楀彿浣滀负鍒嗛殧绗︼紝骞朵繚鐣欐墍鏈夌┖鐧?```

  $(func-name, arg1, arg2, arg3)

```
鍦ㄨ繖绉嶆儏鍐典笅锛屸€渇unc-name鈥濆皢鏀跺埌鈥?arg1鈥濄€佲€?arg2鈥濄€佲€?arg3鈥濄€傚墠瀵肩┖鏍肩殑
瀛樺湪鍙兘浼氬洜鍑芥暟鑰屽紓銆侻ake 涔熸槸濡傛鈥斺€斾緥濡傦紝$(subst .c, .o, $(sources)) 鏄?涓€涓吀鍨嬬殑閿欒锛涘畠浼氭妸 鈥?c鈥?鏇挎崲涓?鈥?.o鈥濄€?
鍦?Make 涓紝鐢ㄦ埛瀹氫箟鍑芥暟閫氳繃浣跨敤鍐呯疆鍑芥暟鏉ュ紩鐢紝
```

    $(call my-func,arg1,arg2,arg3)

```
Kconfig 浠ョ浉鍚岀殑鏂瑰紡璋冪敤鐢ㄦ埛瀹氫箟鍑芥暟鍜屽唴缃嚱鏁般€傜渷鐣?'call' 浣胯娉曟洿绠€鐭€?
鍦?Make 涓紝鏌愪簺鍑芥暟灏嗛€楀彿瑙嗕负瀛楅潰瀛楃鑰岄潪鍙傛暟鍒嗛殧绗︺€備緥濡傦紝$(shell echo
hello, world) 杩愯鍛戒护 鈥渆cho hello, world鈥濄€傚悓鏍凤紝$(info hello, world) 灏?鈥渉ello, world鈥?鎵撳嵃鍒?stdout銆備綘鍙互璇磋繖鏄竴绉嶁€滄湁鐢ㄢ€濈殑涓嶄竴鑷淬€?
鍦?Kconfig 涓紝涓轰簡绠€鍖栧疄鐜板苟淇濇寔璇硶涓€鑷达紝閫楀彿浼?```

  $(shell, echo hello, world)

```
鏄竴涓敊璇紝鍥犱负瀹冨湪鍚?'shell' 鍑芥暟浼犻€掍袱涓弬鏁帮紝鑰?```

  comma := ,
  $(shell, echo hello$(comma) world)


```
### 娉ㄦ剰浜嬮」


鍙橀噺锛堟垨鍑芥暟锛変笉鑳借法 token 灞曞紑銆傚洜姝わ紝浣犱笉鑳藉皢鍙橀噺鐢ㄤ綔鐢卞涓?token 缁勬垚鐨?琛ㄨ揪寮忕殑绠€鍐欍€?```

    RANGE_MIN := 1
    RANGE_MAX := 3

    config FOO
            int "foo"
            range $(RANGE_MIN) $(RANGE_MAX)

```
```

    RANGES := 1 3

    config FOO
            int "foo"
            range $(RANGES)

```
鍙橀噺涓嶈兘灞曞紑涓?Kconfig 涓殑浠讳綍鍏抽敭瀛椼€備互涓嬪啓娉?```

    MY_TYPE := tristate

    config FOO
            $(MY_TYPE) "foo"
            default y

```
浠庤璁′笂鐪嬪緢鏄剧劧锛?(shell command) 鏄湪鏂囨湰鏇挎崲闃舵灞曞紑鐨勩€備綘涓嶈兘鍚?'shell'
鍑芥暟浼犻€掔鍙枫€?
```

    config ENDIAN_FLAG
            string
            default "-mbig-endian" if CPU_BIG_ENDIAN
            default "-mlittle-endian" if CPU_LITTLE_ENDIAN

    config CC_HAS_ENDIAN_FLAG
            def_bool $(shell $(srctree)/scripts/gcc-check-flag ENDIAN_FLAG)

```
鐩稿弽锛屼綘鍙互鍍忎笅闈㈣繖鏍峰仛锛屼互渚夸换浣曞嚱鏁拌皟鐢ㄩ兘鏄潤鎬佺殑
```

    config CC_HAS_ENDIAN_FLAG
            bool
            default $(shell $(srctree)/scripts/gcc-check-flag -mbig-endian) if CPU_BIG_ENDIAN
            default $(shell $(srctree)/scripts/gcc-check-flag -mlittle-endian) if CPU_LITTLE_ENDIAN

```
