
## Kernel Memory Sanitizer (KMSAN)


KMSAN 鏄竴涓姩鎬侀敊璇娴嬪櫒锛屾棬鍦ㄥ彂鐜板鏈垵濮嬪寲鍊肩殑浣跨敤銆傚畠鍩轰簬缂栬瘧鍣ㄦ彃妗╁疄鐜帮紝涓庣敤鎴风┖闂寸殑 `MemorySanitizer tool`_ 鍗佸垎鐩镐技銆?

闇€瑕佺壒鍒敞鎰忕殑鏄紝KMSAN 骞堕潪涓虹敓浜х幆澧冧娇鐢ㄨ€岃璁★紝鍥犱负瀹冧細鏄捐憲澧炲姞鍐呮牳鍐呭瓨鍗犵敤骞舵嫋鎱㈡暣涓郴缁熴€?

## 鐢ㄦ硶


### 鏋勫缓鍐呮牳


涓轰簡鏋勫缓甯︽湁 KMSAN 鐨勫唴鏍革紝浣犻渶瑕佷竴涓緝鏂扮殑 Clang锛?4.0.6+锛夈€傛湁鍏冲浣曟瀯寤?Clang 鐨勮鏄庯紝璇峰弬鑰?`LLVM documentation`_銆?

鐜板湪锛屽湪鍚敤 `CONFIG_KMSAN` 鐨勬儏鍐典笅閰嶇疆骞舵瀯寤哄唴鏍搞€?

### 绀轰緥鎶ュ憡


```

  =====================================================
  BUG: KMSAN: uninit-value in test_uninit_kmsan_check_memory+0x1be/0x380 [kmsan_test]
   test_uninit_kmsan_check_memory+0x1be/0x380 mm/kmsan/kmsan_test.c:273
   kunit_run_case_internal lib/kunit/test.c:333
   kunit_try_run_case+0x206/0x420 lib/kunit/test.c:374
   kunit_generic_run_threadfn_adapter+0x6d/0xc0 lib/kunit/try-catch.c:28
   kthread+0x721/0x850 kernel/kthread.c:327
   ret_from_fork+0x1f/0x30 ??:?

  Uninit was stored to memory at:
   do_uninit_local_array+0xfa/0x110 mm/kmsan/kmsan_test.c:260
   test_uninit_kmsan_check_memory+0x1a2/0x380 mm/kmsan/kmsan_test.c:271
   kunit_run_case_internal lib/kunit/test.c:333
   kunit_try_run_case+0x206/0x420 lib/kunit/test.c:374
   kunit_generic_run_threadfn_adapter+0x6d/0xc0 lib/kunit/try-catch.c:28
   kthread+0x721/0x850 kernel/kthread.c:327
   ret_from_fork+0x1f/0x30 ??:?

  Local variable uninit created at:
   do_uninit_local_array+0x4a/0x110 mm/kmsan/kmsan_test.c:256
   test_uninit_kmsan_check_memory+0x1a2/0x380 mm/kmsan/kmsan_test.c:271

  Bytes 4-7 of 8 are uninitialized
  Memory access of size 8 starts at ffff888083fe3da0

  CPU: 0 PID: 6731 Comm: kunit_try_catch Tainted: G    B       E     5.16.0-rc3+ #104
  Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.14.0-2 04/01/2014
  =====================================================

```
璇ユ姤鍛婅〃鏄庯紝灞€閮ㄥ彉閲?`uninit` 鍦?`do_uninit_local_array()` 涓互鏈垵濮嬪寲鐘舵€佸垱寤恒€傜涓夋潯鏍堝洖婧搴旇鍙橀噺琚垱寤虹殑浣嶇疆銆?

绗竴鏉℃爤鍥炴函灞曠ず浜嗘湭鍒濆鍖栧€煎湪浣曞琚娇鐢紙鍦?`test_uninit_kmsan_check_memory()` 涓級銆傝宸ュ叿杩樻樉绀轰簡灞€閮ㄥ彉閲忎腑鍝簺瀛楄妭鏈鍒濆鍖栵紝浠ュ強鍦ㄤ娇鐢ㄥ墠璇ュ€艰澶嶅埗鍒板彟涓€澶勫唴瀛樹綅缃殑鏍堜俊鎭€?

KMSAN 鍦ㄤ互涓嬫儏鍐典笅浼氭姤鍛婂鏈垵濮嬪寲鍊?`v` 鐨勪娇鐢細

 - 鍦ㄦ潯浠跺垽鏂腑锛屼緥濡?`if (v) { ... }`锛?
 - 鍦ㄧ储寮曟垨鎸囬拡瑙ｅ紩鐢ㄤ腑锛屼緥濡?`array[v]` 鎴?`*v`锛?
 - 褰撳畠琚鍒跺埌鐢ㄦ埛绌洪棿鎴栫‖浠舵椂锛屼緥濡?`copy_to_user(..., &v, ...)`锛?
 - 褰撳畠浣滀负鍙傛暟浼犻€掔粰鍑芥暟锛屼笖
   `CONFIG_KMSAN_CHECK_PARAM_RETVAL` 宸插惎鐢紙瑙佷笅鏂囷級銆?

鎵€鎻愬強鐨勬儏鍐碉紙闄や簡鍚戠敤鎴风┖闂存垨纭欢澶嶅埗鏁版嵁杩欑鎯呭喌锛屽畠灞炰簬瀹夊叏闂锛変粠 C11 鏍囧噯鐨勮搴︽潵鐪嬮兘琚涓烘湭瀹氫箟琛屼负銆?

### 绂佺敤鎻掓々


鍙互浣跨敤鍑芥暟灞炴€?`__no_kmsan_checks` 鏍囪鏌愪釜鍑芥暟銆傝繖鏍峰仛浼氫娇 KMSAN 蹇界暐璇ュ嚱鏁颁腑鐨勬湭鍒濆鍖栧€硷紝骞跺皢鍏惰緭鍑烘爣璁颁负宸插垵濮嬪寲銆傜粨鏋滄槸锛岀敤鎴峰皢涓嶄細鍐嶆敹鍒颁笌璇ュ嚱鐩稿叧鐨?KMSAN 鎶ュ憡銆?

KMSAN 鏀寔鐨勫彟涓€涓嚱鏁板睘鎬ф槸 `__no_sanitize_memory`銆傚皢璇ュ睘鎬у簲鐢ㄤ簬鏌愪釜鍑芥暟浼氫娇 KMSAN 涓嶅鍏舵彃妗╋紝濡傛灉鎴戜滑涓嶅笇鏈涚紪璇戝櫒骞叉壈鏌愪簺搴曞眰浠ｇ爜锛堜緥濡傝鏍囪涓?`noinstr` 鐨勪唬鐮侊紝瀹冧細闅愬紡娣诲姞 `__no_sanitize_memory`锛夛紝杩欎細寰堟湁甯姪銆?

鐒惰€岃繖浼氬甫鏉ヤ唬浠凤細姝ょ被鍑芥暟涓殑鏍堝垎閰嶅皢鍏锋湁涓嶆纭殑 shadow/origin 鍊硷紝寰堝彲鑳藉鑷磋鎶ャ€備粠闈炴彃妗╀唬鐮佽皟鐢ㄧ殑鍑芥暟涔熷彲鑳芥敹鍒颁笉姝ｇ‘鐨勫弬鏁板厓鏁版嵁銆?

浣滀负缁忛獙娉曞垯锛屽簲閬垮厤鏄惧紡浣跨敤 `__no_sanitize_memory`銆?

```

  KMSAN_SANITIZE_main.o := n

```
```
  KMSAN_SANITIZE := n

```
鍦?Makefile 涓€傚彲浠ュ皢鍏惰涓哄璇ユ枃浠舵垨鐩綍涓殑姣忎釜鍑芥暟搴旂敤 `__no_sanitize_memory`銆傚ぇ澶氭暟鐢ㄦ埛涓嶉渶瑕?`KMSAN_SANITIZE`锛岄櫎闈炰粬浠殑浠ｇ爜琚?KMSAN 鐮村潖锛堜緥濡傚湪鏃╂湡鍚姩闃舵杩愯锛夈€?

KMSAN 妫€鏌ヤ篃鍙互浣跨敤 `kmsan_disable_current()` 鍜?`kmsan_enable_current()` 璋冪敤涓哄綋鍓嶄换鍔′复鏃剁鐢ㄣ€傛瘡涓?`kmsan_enable_current()` 璋冪敤涔嬪墠蹇呴』鏈変竴涓?`kmsan_disable_current()` 璋冪敤锛涜繖浜涜皟鐢ㄥ鍙互宓屽銆備娇鐢ㄨ繖浜涜皟鐢ㄦ椂闇€瑕佸皬蹇冿紝淇濇寔绂佺敤鍖哄煙绠€鐭紝骞跺敖鍙兘浼樺厛浣跨敤鍏朵粬绂佺敤鎻掓々鐨勬柟寮忋€?

## Support


涓轰簡璁?KMSAN 姝ｅ父宸ヤ綔锛屽唴鏍稿繀椤讳娇鐢?Clang 鏋勫缓锛岃縿浠婁负姝?Clang 鏄敮涓€鏀寔 KMSAN 鐨勭紪璇戝櫒銆傚唴鏍告彃妗╄繃绋嬪熀浜庣敤鎴风┖闂寸殑 `MemorySanitizer tool`_銆?

鐩墠杩愯鏃跺簱浠呮敮鎸?x86_64銆?

## KMSAN 宸ヤ綔鍘熺悊


### KMSAN 褰卞瓙鍐呭瓨


KMSAN 涓哄唴鏍稿唴瀛樼殑姣忎竴涓瓧鑺傚叧鑱斾竴涓厓鏁版嵁瀛楄妭锛堜篃绉颁负 shadow 瀛楄妭锛夈€傚鏋滃唴鏍稿唴瀛樺瓧鑺備腑鐨勫搴斾綅鏈垵濮嬪寲锛屽垯 shadow 瀛楄妭涓殑鐩稿簲浣嶈缃綅銆傚皢鍐呭瓨鏍囪涓烘湭鍒濆鍖栵紙鍗虫妸鍏?shadow 瀛楄妭璁句负 `0xff`锛夌О涓?poisoning锛堟薄鏌擄級锛屽皢鍏舵爣璁颁负宸插垵濮嬪寲锛堟妸 shadow 瀛楄妭璁句负 `0x00`锛夌О涓?unpoisoning锛堝幓姹℃煋锛夈€?

褰撲竴涓柊鐨勫彉閲忓湪鏍堜笂鍒嗛厤鏃讹紝榛樿鎯呭喌涓嬩細琚紪璇戝櫒鎻掑叆鐨勬彃妗╀唬鐮佹薄鏌擄紙闄ら潪瀹冩槸涓€涓珛鍗宠鍒濆鍖栫殑鏍堝彉閲忥級銆備换浣曟病鏈変娇鐢?`__GFP_ZERO` 鐨勬柊鍫嗗垎閰嶄篃浼氳姹℃煋銆?

缂栬瘧鍣ㄦ彃妗╄繕浼氳窡韪?shadow 鍊奸殢浠ｇ爜浣跨敤鐨勪紶鎾繃绋嬨€傚湪闇€瑕佹椂锛屾彃妗╀唬鐮佷細璋冪敤 `mm/kmsan/` 涓殑杩愯鏃跺簱鏉ユ寔涔呭寲 shadow 鍊笺€?

鍩烘湰绫诲瀷鎴栧鍚堢被鍨嬬殑 shadow 鍊兼槸涓€涓笌鍏剁瓑闀跨殑瀛楄妭鏁扮粍銆傚綋鍚戝唴瀛樺啓鍏ュ父閲忓€兼椂锛岃鍐呭瓨琚幓姹℃煋銆傚綋浠庡唴瀛樿鍙栧€兼椂锛屼篃浼氳幏鍙栧叾 shadow 鍐呭瓨锛屽苟灏嗗叾浼犳挱鍒版墍鏈変娇鐢ㄨ鍊肩殑鎿嶄綔涓€傚浜庢瘡涓€涓彇涓€涓垨澶氫釜鍊肩殑鎸囦护锛岀紪璇戝櫒浼氱敓鎴愪唬鐮侊紝鏍规嵁杩欎簺鍊煎強鍏?shadow 璁＄畻缁撴灉鐨?shadow銆?

```

  int a = 0xff;  // i.e. 0x000000ff
  int b;
  int c = a | b;

```
鍦ㄨ繖绉嶆儏鍐典笅锛宍a` 鐨?shadow 鏄?`0`锛宍b` 鐨?shadow 鏄?`0xffffffff`锛宍c` 鐨?shadow 鏄?`0xffffff00`銆傝繖鎰忓懗鐫€ `c` 鐨勯珮涓変釜瀛楄妭鏈垵濮嬪寲锛岃€屼綆瀛楄妭宸插垵濮嬪寲銆?

### 鏉ユ簮璺熻釜


鍐呮牳鍐呭瓨鐨勬瘡鍥涗釜瀛楄妭涔熸槧灏勪簡涓€涓墍璋撶殑 origin锛堟潵婧愶級銆傝 origin 鎻忚堪浜嗗湪绋嬪簭鎵ц杩囩▼涓垱寤鸿鏈垵濮嬪寲鍊肩殑浣嶇疆銆傛瘡涓?origin 瑕佷箞鍏宠仈鍒板畬鏁寸殑鍒嗛厤鏍堬紙瀵逛簬鍫嗗垎閰嶇殑鍐呭瓨锛夛紝瑕佷箞鍏宠仈鍒板寘鍚鏈垵濮嬪寲鍙橀噺鐨勫嚱鏁帮紙瀵逛簬灞€閮ㄥ彉閲忥級銆?

褰撴湭鍒濆鍖栧彉閲忓湪鏍堟垨鍫嗕笂鍒嗛厤鏃讹紝浼氬垱寤轰竴涓柊鐨?origin 鍊硷紝骞剁敤璇ュ€煎～鍏呰鍙橀噺鐨?origin銆傚綋浠庡唴瀛樿鍙栧€兼椂锛屼篃浼氳鍙栧叾 origin锛屽苟涓?shadow 涓€璧蜂繚瀛樸€傚浜庢瘡涓€涓彇涓€涓垨澶氫釜鍊肩殑鎸囦护锛岀粨鏋滅殑 origin 鏄搴斾簬浠讳竴鏈垵濮嬪寲杈撳叆鐨?origin 涔嬩竴銆傚鏋滃皢姹℃煋鍊煎啓鍏ュ唴瀛橈紝鍏?origin 涔熶細琚啓鍏ュ搴旂殑瀛樺偍浣嶇疆銆?

```

  int a = 42;
  int b;
  int c = a + b;

```
鍦ㄨ繖绉嶆儏鍐典笅锛宍b` 鐨?origin 鍦ㄥ嚱鏁板叆鍙ｆ椂鐢熸垚锛屽苟鍦ㄥ姞娉曠粨鏋滃啓鍏ュ唴瀛樹箣鍓嶅瓨鍌ㄥ埌 `c` 鐨?origin 涓€?

濡傛灉澶氫釜鍙橀噺瀛樺偍鍦ㄥ悓涓€涓洓瀛楄妭鍧椾腑锛屽畠浠彲鑳戒細鍏变韩鐩稿悓鐨?origin 鍦板潃銆傚湪杩欑鎯呭喌涓嬶紝瀵逛换鎰忎竴涓彉閲忕殑姣忔鍐欏叆閮戒細鏇存柊鎵€鏈夎繖浜涘彉閲忕殑 origin銆傚湪杩欑鎯呭喌涓嬫垜浠笉寰椾笉鐗虹壊绮惧害锛屽洜涓轰负鍗曚釜浣嶏紙鐢氳嚦瀛楄妭锛夊瓨鍌?origin 浠ｄ环杩囬珮銆?

```

  int combine(short a, short b) {
    union ret_t {
      int i;
      short s[2];
    } ret;
    ret.s[0] = a;
    ret.s[1] = b;
    return ret.i;
  }

```
濡傛灉 `a` 宸插垵濮嬪寲鑰?`b` 鏈垵濮嬪寲锛屽垯缁撴灉鐨?shadow 灏嗘槸 `0xffff0000`锛岀粨鏋滅殑 origin 灏嗘槸 `b` 鐨?origin銆俙ret.s[^0^]` 灏嗗叿鏈夌浉鍚岀殑 origin锛屼絾瀹冩案杩滀笉浼氳浣跨敤锛屽洜涓鸿鍙橀噺宸插垵濮嬪寲銆?

濡傛灉涓や釜鍑芥暟鍙傛暟閮芥湭鍒濆鍖栵紝鍒欏彧淇濈暀绗簩涓弬鏁扮殑 origin銆?

#### 鏉ユ簮閾?


涓轰簡绠€鍖栬皟璇曪紝KMSAN 浼氫负姣忔灏嗘湭鍒濆鍖栧€煎瓨鍌ㄥ埌鍐呭瓨鍒涘缓鏂扮殑 origin銆傛柊鐨?origin 鍚屾椂寮曠敤鍏跺垱寤烘爤浠ュ強璇ュ€煎厛鍓嶆嫢鏈夌殑 origin銆傝繖鍙兘瀵艰嚧鍐呭瓨娑堣€楀鍔狅紝鍥犳鎴戜滑鍦ㄨ繍琛屾椂涓檺鍒朵簡 origin 閾剧殑闀垮害銆?

### Clang 鎻掓々 API


Clang 鎻掓々杩囩▼浼氬悜鍐呮牳浠ｇ爜涓彃鍏ュ `mm/kmsan/nstrumentation.c` 涓畾涔夌殑鍑芥暟鐨勮皟鐢ㄣ€?

#### 褰卞瓙鎿嶄綔


瀵逛簬姣忔鍐呭瓨璁块棶锛岀紪璇戝櫒閮戒細鍙戝嚭涓€涓皟鐢紝璋冪敤涓€涓繑鍥炰互涓嬪唴瀹圭殑鍑芥暟锛?

```

  typedef struct {
    void *shadow, *origin;
  } shadow_origin_ptr_t

  shadow_origin_ptr_t __msan_metadata_ptr_for_load_{1,2,4,8}(void *addr)
  shadow_origin_ptr_t __msan_metadata_ptr_for_store_{1,2,4,8}(void *addr)
  shadow_origin_ptr_t __msan_metadata_ptr_for_load_n(void *addr, uintptr_t size)
  shadow_origin_ptr_t __msan_metadata_ptr_for_store_n(void *addr, uintptr_t size)

```
鍑芥暟鍚嶅彇鍐充簬鍐呭瓨璁块棶鐨勫ぇ灏忋€?

缂栬瘧鍣ㄧ‘淇濆浜庢瘡涓€涓鍔犺浇鐨勫€硷紝鍏?shadow 鍜?origin 鍊奸兘浠庡唴瀛樹腑璇诲彇銆傚綋鍊艰瀛樺偍鍒板唴瀛樻椂锛屽叾 shadow 鍜?origin 涔熶細浣跨敤鍏冩暟鎹寚閽堜竴骞跺瓨鍌ㄣ€?

#### 澶勭悊灞€閮ㄥ彉閲?


浣跨敤鐗规畩鍑芥暟涓哄眬閮ㄥ彉閲忓垱寤烘柊鐨?origin 鍊硷細

```
  void __msan_poison_alloca(void *addr, uintptr_t size, char *descr)

```
#### 瀵规瘡涓换鍔℃暟鎹殑璁块棶


鍦ㄦ瘡涓鎻掓々鍑芥暟鐨勫紑澶达紝KMSAN 浼氭彃鍏ュ浠ヤ笅鍑芥暟鐨勮皟鐢細

```
  kmsan_context_state *__msan_get_context_state(void)

```
```
  struct kmsan_context_state {
    char param_tls[KMSAN_PARAM_SIZE];
    char retval_tls[KMSAN_RETVAL_SIZE];
    char va_arg_tls[KMSAN_PARAM_SIZE];
    char va_arg_origin_tls[KMSAN_PARAM_SIZE];
    u64 va_arg_overflow_size_tls;
    char param_origin_tls[KMSAN_PARAM_SIZE];
    depot_stack_handle_t retval_origin_tls;
  };

```
璇ョ粨鏋勮 KMSAN 鐢ㄦ潵鍦ㄨ鎻掓々鐨勫嚱鏁颁箣闂翠紶閫掑弬鏁扮殑 shadow 鍜?origin锛堥櫎闈炲弬鏁拌 `CONFIG_KMSAN_CHECK_PARAM_RETVAL` 绔嬪嵆妫€鏌ワ級銆?

#### 灏嗘湭鍒濆鍖栧€间紶閫掔粰鍑芥暟


Clang 鐨?MemorySanitizer 鎻掓々鏈変竴涓€夐」 `-fsanitize-memory-param-retval`锛屽畠浣跨紪璇戝櫒妫€鏌ユ寜鍊间紶閫掔殑鍑芥暟鍙傛暟浠ュ強鍑芥暟杩斿洖鍊笺€?

璇ラ€夐」鐢?`CONFIG_KMSAN_CHECK_PARAM_RETVAL` 鎺у埗锛岄粯璁ゅ惎鐢紝浠ヤ娇 KMSAN 鑳藉鏇存棭鍦版姤鍛婃湭鍒濆鍖栧€笺€傛洿澶氱粏鑺傝鍙傝€?`LKML discussion`_銆?

鐢变簬杩欎簺妫€鏌ュ湪 LLVM 涓殑瀹炵幇鏂瑰紡锛堝畠浠粎搴旂敤浜庢爣璁颁负 `noundef` 鐨勫弬鏁帮級锛屽苟闈炴墍鏈夊弬鏁伴兘鑳戒繚璇佽妫€鏌ワ紝鍥犳鎴戜滑涓嶈兘鏀惧純 `kmsan_context_state` 涓殑鍏冩暟鎹瓨鍌ㄣ€?

#### 瀛楃涓插嚱鏁?


缂栬瘧鍣ㄤ細灏嗗 `memcpy()`/`memmove()`/`memset()` 鐨勮皟鐢ㄦ浛鎹负浠ヤ笅鍑芥暟銆傚綋鏁版嵁缁撴瀯琚垵濮嬪寲鎴栧鍒舵椂涔熶細璋冪敤杩欎簺鍑芥暟锛岀‘淇?shadow 鍜?origin 鍊奸殢涔嬩竴骞惰澶嶅埗锛?

```
  void *__msan_memcpy(void *dst, void *src, uintptr_t n)
  void *__msan_memmove(void *dst, void *src, uintptr_t n)
  void *__msan_memset(void *dst, int c, uintptr_t n)

```
#### 閿欒鎶ュ憡


瀵逛簬姣忔鍊肩殑浣跨敤锛岀紪璇戝櫒閮戒細鍙戝嚭涓€涓?shadow 妫€鏌ワ紝璋冪敤锛?

```
  void __msan_warning(u32 origin)

```
`__msan_warning()` 浼氫娇 KMSAN 杩愯鏃舵墦鍗伴敊璇姤鍛娿€?

#### 鍐呰仈姹囩紪鎻掓々


```
  void __msan_instrument_asm_store(void *addr, uintptr_t size)

```
锛屽畠浼氬皢璇ュ唴瀛樺尯鍩熷幓姹℃煋銆?

杩欑鏂规硶鍙兘浼氭帺鐩栨煇浜涢敊璇紝浣嗕篃鏈夊姪浜庨伩鍏嶄綅杩愮畻銆佸師瀛愭搷浣滅瓑鍦烘櫙涓ぇ閲忕殑璇姤銆?

鏈夋椂浼犲叆鍐呰仈姹囩紪鐨勬寚閽堝苟涓嶆寚鍚戞湁鏁堝唴瀛樸€傚湪杩欑鎯呭喌涓嬶紝瀹冧滑浼氬湪杩愯鏃惰蹇界暐銆?


### 杩愯鏃跺簱


浠ｇ爜浣嶄簬 `mm/kmsan/`銆?

#### 姣忎釜浠诲姟鐨?KMSAN 鐘舵€?


姣忎釜 task_struct 閮藉叧鑱斾竴涓?KMSAN 浠诲姟鐘舵€侊紝鐢ㄤ簬淇濆瓨 KMSAN锛?

```
  struct kmsan_context {
    ...
    unsigned int depth;
    struct kmsan_context_state cstate;
    ...
  }

  struct task_struct {
    ...
    struct kmsan_context kmsan;
    ...
  }

```
#### KMSAN 涓婁笅鏂?


鍦ㄨ繍琛屼簬鍐呮牳浠诲姟涓婁笅鏂囨椂锛孠MSAN 浣跨敤 `current->kmsan.cstate` 鏉ヤ繚瀛樺嚱鏁板弬鏁板拰杩斿洖鍊肩殑鍏冩暟鎹€?

浣嗗湪鍐呮牳杩愯浜庝腑鏂€乻oftirq 鎴?NMI 涓婁笅鏂囩殑鎯呭喌涓嬶細

```
  DEFINE_PER_CPU(struct kmsan_ctx, kmsan_percpu_ctx);

```
#### 鍏冩暟鎹垎閰?


鍐呮牳涓湁鍑犱釜鐢ㄤ簬瀛樻斁鍏冩暟鎹殑鍦版柟銆?

1. 姣忎釜 `struct page` 瀹炰緥閮藉寘鍚袱涓寚鍚戝叾 shadow 鍜?origin 鐨勬寚閽堬細

```
  struct page {
    ...
    struct page *shadow, *origin;
    ...
  };

```
鍦ㄥ惎鍔ㄩ樁娈碉紝鍐呮牳涓烘瘡涓€涓彲鐢ㄧ殑鍐呮牳椤靛垎閰?shadow 鍜?origin 椤点€傝繖涓€杩囩▼鍙戠敓寰楃浉褰撴櫄锛屾鏃跺唴鏍稿湴鍧€绌洪棿宸茬粡纰庣墖鍖栵紝鍥犳鏅€氭暟鎹〉鍙兘浼氫笌鍏冩暟鎹〉浠绘剰浜ら敊銆?

杩欐剰鍛崇潃锛岄€氬父瀵逛簬涓や釜杩炵画鐨勫唴瀛橀〉锛屽畠浠殑 shadow/origin 椤靛彲鑳藉苟涓嶈繛缁€傚洜姝わ紝濡傛灉涓€娆″唴瀛樿闂法瓒婁簡鏌愪釜鍐呭瓨鍧楃殑杈圭晫锛屽 shadow/origin 鍐呭瓨鐨勮闂彲鑳戒細鐮村潖鍏朵粬椤碉紝鎴栦粠杩欎簺椤典腑璇诲彇涓嶆纭殑鍊笺€?

瀹為檯涓婏紝鐢卞悓涓€娆?`alloc_pages()` 璋冪敤杩斿洖鐨勮繛缁唴瀛橀〉灏嗗叿鏈夎繛缁殑鍏冩暟鎹紝鑰屽鏋滆繖浜涢〉灞炰簬涓ゆ涓嶅悓鐨勫垎閰嶏紝瀹冧滑鐨勫厓鏁版嵁椤靛垯鍙兘纰庣墖鍖栥€?

瀵逛簬鍐呮牳鏁版嵁锛坄.data`銆乣.bss` 绛夛級鍜?percpu 鍐呭瓨鍖哄煙锛屽悓鏍蜂笉淇濊瘉鍏冩暟鎹殑杩炵画鎬с€?

褰?`__msan_metadata_ptr_for_XXX_YYY()` 鍛戒腑涓や釜鍒嗛厤涔嬮棿鐨勮竟鐣屾椂锛?

```
  char dummy_load_page[PAGE_SIZE] __attribute__((aligned(PAGE_SIZE)));
  char dummy_store_page[PAGE_SIZE] __attribute__((aligned(PAGE_SIZE)));

```
`dummy_load_page` 琚浂鍒濆鍖栵紝鍥犳浠庝腑璇诲彇鎬绘槸寰楀埌闆躲€俙dummy_store_page` 鐨勬墍鏈夊啓鍏ラ兘琚拷鐣ャ€?

2. 瀵逛簬 vmalloc 鍐呭瓨鍜屾ā鍧楋紝鍐呭瓨鍖洪棿銆佸叾 shadow 鍜?origin 涔嬮棿瀛樺湪鐩存帴鏄犲皠銆侹MSAN 灏?vmalloc 鍖哄煙缂╁噺 3/4锛屼娇寰楀彧鏈夌涓€涓洓鍒嗕箣涓€鍙敤浜?`vmalloc()`銆倂malloc 鍖哄煙鐨勭浜屼釜鍥涘垎涔嬩竴鍖呭惈绗竴涓洓鍒嗕箣涓€鐨?shadow 鍐呭瓨锛岀涓変釜鍥涘垎涔嬩竴淇濆瓨 origin銆傜鍥涗釜鍥涘垎涔嬩竴鐨勪竴灏忛儴鍒嗗寘鍚唴鏍告ā鍧楃殑 shadow 鍜?origin銆傛洿澶氱粏鑺傝鍙傝€?`arch/x86/include/asm/pgtable_64_types.h`銆?

褰撲竴缁勯〉琚槧灏勫埌杩炵画鐨勮櫄鎷熷唴瀛樼┖闂存椂锛屽畠浠殑 shadow 鍜?origin 椤典篃浼氳绫讳技鍦版槧灏勫埌杩炵画鍖哄煙銆?

## 鍙傝€冭祫鏂?


E. Stepanov, K. Serebryany. `MemorySanitizer锛欳++ 涓湭鍒濆鍖栧唴瀛樹娇鐢ㄧ殑蹇€熸娴嬪櫒
<https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/43308.pdf>`_.
In Proceedings of CGO 2015.
