## 鍦ㄧ敤鎴风┖闂村簲鐢ㄧ▼搴忎腑浣跨敤 XSTATE 鐗规€?

x86 鏋舵瀯鏀寔閫氳繃 CPUID 鏋氫妇鐨勬诞鐐规墿灞曘€傚簲鐢ㄧ▼搴忛€氳繃鏌ヨ CPUID 骞朵娇鐢?XGETBV 鏉ヨ瘎浼板唴鏍?XCR0 宸插惎鐢ㄥ摢浜涚壒鎬с€?
鐩村埌 AVX-512 鍜?PKRU 鐘舵€侊紝濡傛灉鍙敤锛岃繖浜涚壒鎬т細鐢卞唴鏍歌嚜鍔ㄥ惎鐢ㄣ€傚儚 AMX
TILE_DATA锛圶STATE 缁勪欢 18锛夎繖鏍风殑鐗规€у悓鏍风敱 XCR0 鍚敤锛屼絾鐩稿叧鎸囦护鐨勯娆?浣跨敤浼氳鍐呮牳鎹曡幏锛屽洜涓洪粯璁ゆ儏鍐典笅骞朵笉浼氳嚜鍔ㄥ垎閰嶆墍闇€鐨勫ぇ鍨?XSTATE 缂撳啿鍖恒€?
### 寮曞叆鍔ㄦ€佺壒鎬х殑鐩殑


浼犵粺鐨勭敤鎴风┖闂村簱閫氬父涓哄鐢ㄤ俊鍙锋爤锛坅lternate signal stack锛夌‖缂栫爜浜嗛潤鎬?澶у皬锛屽父甯镐娇鐢?MINSIGSTKSZ锛堥€氬父涓?2KB锛夈€傝鏍堝繀椤昏嚦灏戣兘澶熷瓨鏀惧唴鏍稿湪璺冲叆
淇″彿澶勭悊鍑芥暟涔嬪墠寤虹珛鐨勪俊鍙峰抚銆傝淇″彿甯у繀椤诲寘鍚敱 CPU 瀹氫箟鐨?XSAVE 缂撳啿鍖恒€?
鐒惰€岋紝杩欐剰鍛崇潃淇″彿鏍堢殑澶у皬鏄姩鎬佺殑鑰岄潪闈欐€佺殑锛屽洜涓轰笉鍚岀殑 CPU 鎷ユ湁涓嶅悓
澶у皬鐨?XSAVE 缂撳啿鍖恒€傚鐜版湁搴旂敤绋嬪簭鑰岃█锛岀紪璇戞湡纭畾鐨?2KB 澶у皬瀵逛簬 AMX
杩欑被鏂?CPU 鐗规€ф潵璇村お灏忎簡銆備笌鍏舵櫘閬嶈姹傛洿澶х殑鏍堬紝鍊熷姪鍔ㄦ€佸惎鐢ㄦ満鍒讹紝鍐呮牳
鍙互寮哄埗鐢ㄦ埛绌洪棿搴旂敤绋嬪簭浣跨敤灏哄鎭板綋鐨?altstack銆?
### 鍦ㄧ敤鎴风┖闂村簲鐢ㄧ▼搴忎腑浣跨敤鍔ㄦ€佸惎鐢ㄧ殑 XSTATE 鐗规€?

鍐呮牳鎻愪緵浜嗕竴绉嶅熀浜?arch_prctl(2) 鐨勬満鍒讹紝渚涘簲鐢ㄧ▼搴忚姹備娇鐢ㄦ绫荤壒鎬с€?涓庢绫绘搷浣滅浉鍏崇殑 arch_prctl(2) 閫夐」濡備笅锛?
-ARCH_GET_XCOMP_SUPP

 arch_prctl(ARCH_GET_XCOMP_SUPP, &features);

 ARCH_GET_XCOMP_SUPP 灏嗗彈鏀寔鐨勭壒鎬у瓨鍌ㄥ埌 uint64_t 绫诲瀷鐨勭敤鎴风┖闂村瓨鍌ㄤ腑銆? 绗簩涓弬鏁版槸鎸囧悜璇ュ瓨鍌ㄧ殑鎸囬拡銆?
-ARCH_GET_XCOMP_PERM

 arch_prctl(ARCH_GET_XCOMP_PERM, &features);

 ARCH_GET_XCOMP_PERM 灏嗙敤鎴风┖闂磋繘绋嬪凡鑾峰緱璁稿彲鐨勭壒鎬у瓨鍌ㄥ埌 uint64_t 绫诲瀷鐨? 鐢ㄦ埛绌洪棿瀛樺偍涓€傜浜屼釜鍙傛暟鏄寚鍚戣瀛樺偍鐨勬寚閽堛€?
-ARCH_REQ_XCOMP_PERM

 arch_prctl(ARCH_REQ_XCOMP_PERM, feature_nr);

 ARCH_REQ_XCOMP_PERM 鐢ㄤ簬璇锋眰鏌愪釜鍔ㄦ€佸惎鐢ㄧ殑鐗规€ф垨鐗规€ч泦鍚堢殑璁稿彲銆備竴涓? 鐗规€ч泦鍚堝彲浠ユ槧灏勫埌涓€涓鏂斤紙facility锛夛紝渚嬪 AMX锛屽苟涓斿彲鑳介渶瑕佸惎鐢ㄤ竴涓? 鎴栧涓?XSTATE 缁勪欢銆?
 璇?feature 鍙傛暟鏄煇涓鏂芥甯稿伐浣滄墍闇€鐨勬渶楂?XSTATE 缁勪欢缂栧彿銆?
璇锋眰鏌愪釜鐗规€х殑璁稿彲鏃讹紝鍐呮牳浼氭鏌ュ叾鍙敤鎬с€傚唴鏍镐細纭繚杩涚▼鍚勪换鍔＄殑 sigaltstack
瓒冲澶э紝浠ュ绾崇敱姝や骇鐢熺殑澶у瀷淇″彿甯с€傛棤璁烘槸鍦?ARCH_REQ_XCOMP_SUPP 鏈熼棿锛岃繕鏄?鍦ㄥ悗缁换浣?sigaltstack(2) 璋冪敤鏈熼棿锛屽唴鏍搁兘浼氬己鍒惰繖涓€绾︽潫銆傚鏋滃凡瀹夎鐨?sigaltstack 灏忎簬鐢辨浜х敓鐨?sigframe 澶у皬锛孉RCH_REQ_XCOMP_SUPP 浼氳繑鍥?-ENOSUPP銆傚悓鏍凤紝濡傛灉璇锋眰鐨?altstack 瀵逛簬宸茶鍙殑鐗规€ц€岃█杩囧皬锛宻igaltstack(2)
浼氳繑鍥?-ENOMEM銆?
璁稿彲涓€缁忔巿浜堝嵆瀵硅繘绋嬫湁鏁堛€傝鍙湪 fork(2) 鏃剁户鎵匡紝鍦?exec(3) 鏃舵竻闄ゃ€?
涓庡姩鎬佸惎鐢ㄧ壒鎬х浉鍏崇殑鎸囦护棣栨琚娇鐢ㄦ椂浼氳鍐呮牳鎹曡幏銆傞櫡闃卞鐞嗙▼搴忎細妫€鏌ヨ
杩涚▼鏄惁鍏锋湁浣跨敤璇ョ壒鎬х殑鏉冮檺銆傚鏋滆繘绋嬫病鏈夋潈闄愶紝鍐呮牳浼氬悜搴旂敤绋嬪簭鍙戦€?SIGILL銆傚鏋滆繘绋嬫嫢鏈夋潈闄愶紝鍒欏鐞嗙▼搴忎細涓鸿浠诲姟鍒嗛厤鏇村ぇ鐨?xstate 缂撳啿鍖猴紝
浠ヤ究瀵瑰ぇ鍨嬬姸鎬佽繘琛屼笂涓嬫枃鍒囨崲銆傚湪鍒嗛厤澶辫触鐨勭綍瑙佹儏鍐典笅锛屽唴鏍镐細鍙戦€?SIGSEGV銆?
##### AMX TILE_DATA 鍚敤绀轰緥


涓嬮潰鏄敤鎴风┖闂村簲鐢ㄧ▼搴忓浣曞姩鎬佸惎鐢?TILE_DATA 鐨勭ず渚嬶細

  1. 搴旂敤绋嬪簭棣栧厛闇€瑕佸悜鍐呮牳鏌ヨ AMX
```

        #include <asm/prctl.h>
        #include <sys/syscall.h>
        #include <stdio.h>
        #include <unistd.h>

        #ifndef ARCH_GET_XCOMP_SUPP
        #define ARCH_GET_XCOMP_SUPP  0x1021
        #endif

        #ifndef ARCH_XCOMP_TILECFG
        #define ARCH_XCOMP_TILECFG   17
        #endif

        #ifndef ARCH_XCOMP_TILEDATA
        #define ARCH_XCOMP_TILEDATA  18
        #endif

        #define MASK_XCOMP_TILE      ((1 << ARCH_XCOMP_TILECFG) | \
                                      (1 << ARCH_XCOMP_TILEDATA))

        unsigned long features;
        long rc;

        ...

        rc = syscall(SYS_arch_prctl, ARCH_GET_XCOMP_SUPP, &features);

        if (!rc && (features & MASK_XCOMP_TILE) == MASK_XCOMP_TILE)
            printf("AMX is available.\n");

  2. After that, determining support for AMX, an application must
     explicitly ask permission to use it::

        #ifndef ARCH_REQ_XCOMP_PERM
        #define ARCH_REQ_XCOMP_PERM  0x1023
        #endif

        ...

        rc = syscall(SYS_arch_prctl, ARCH_REQ_XCOMP_PERM, ARCH_XCOMP_TILEDATA);

        if (!rc)
            printf("AMX is ready for use.\n");

```
Note this example does not include the sigaltstack preparation.

### 淇″彿甯т腑鐨勫姩鎬佺壒鎬?

鍔ㄦ€佸惎鐢ㄧ殑鐗规€у鏋滃湪鍒濆閰嶇疆涓嬶紝鍒欏湪淇″彿杩涘叆鏃朵笉浼氳鍐欏叆淇″彿甯с€傝繖涓?闈炲姩鎬佺壒鎬т笉鍚岋紝鍚庤€呮棤璁哄叾閰嶇疆濡備綍鎬讳細琚啓鍏ャ€備俊鍙峰鐞嗙▼搴忓彲浠ユ鏌?XSAVE 缂撳啿鍖虹殑 XSTATE_BV 瀛楁鏉ュ垽鏂煇涓壒鎬ф槸鍚﹀凡琚啓鍏ャ€?
### 铏氭嫙鏈虹殑鍔ㄦ€佺壒鎬?

璁垮锛坓uest锛夌姸鎬佺粍浠剁殑璁稿彲闇€瑕佷笌瀹夸富鏈猴紙host锛夊垎寮€绠＄悊锛屽洜涓哄畠浠郊姝?浜掓枼銆傜郴缁熸墿灞曚簡鑻ュ共閫夐」鐢ㄤ簬鎺у埗璁垮璁稿彲锛?
-ARCH_GET_XCOMP_GUEST_PERM

 arch_prctl(ARCH_GET_XCOMP_GUEST_PERM, &features);

 ARCH_GET_XCOMP_GUEST_PERM 鏄?ARCH_GET_XCOMP_PERM 鐨勪竴涓彉浣撱€傚洜姝ゅ畠鎻愪緵
 鐩稿悓鐨勮涔夊拰鍔熻兘锛屼絾闈㈠悜鐨勬槸璁垮缁勪欢銆?
-ARCH_REQ_XCOMP_GUEST_PERM

 arch_prctl(ARCH_REQ_XCOMP_GUEST_PERM, feature_nr);

 ARCH_REQ_XCOMP_GUEST_PERM 鏄?ARCH_REQ_XCOMP_PERM 鐨勪竴涓彉浣撱€傚畠瀵硅瀹㈣鍙? 鍏锋湁鐩稿悓鐨勮涔夈€傚湪鎻愪緵绫讳技鍔熻兘鐨勫悓鏃讹紝瀹冧篃甯︽潵涓€涓害鏉燂細鍦ㄥ垱寤虹涓€涓? VCPU 鏃惰鍙細琚喕缁撱€傛鍚庝换浣曟洿鏀硅鍙殑灏濊瘯閮藉皢琚嫆缁濄€傚洜姝わ紝蹇呴』鍦ㄥ垱寤? 绗竴涓?VCPU 涔嬪墠璇锋眰璁稿彲銆?
璇锋敞鎰忥紝鏌愪簺 VMM 鍙兘宸茬粡寤虹珛浜嗕竴缁勫彈鏀寔鐨勭姸鎬佺粍浠躲€傝繖浜涢€夐」骞朵笉鍋囧畾
鏀寔浠讳綍鐗瑰畾鐨?VMM銆?