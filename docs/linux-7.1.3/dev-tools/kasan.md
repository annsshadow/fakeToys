
## Kernel Address Sanitizer (KASAN锛屽唴鏍稿湴鍧€娑堟瘨鍓?


### Overview锛堟杩帮級


Kernel Address Sanitizer (KASAN) 鏄竴涓姩鎬佸唴瀛樺畨鍏ㄩ敊璇娴嬪櫒锛屾棬鍦ㄥ彂鐜拌秺鐣岃闂拰閲婃斁鍚庝娇鐢紙use-after-free锛夌己闄枫€?

KASAN 鏈変笁绉嶆ā寮忥細

1. Generic KASAN锛堥€氱敤 KASAN锛?
2. Software Tag-Based KASAN锛堝熀浜庤蒋浠舵爣绛剧殑 KASAN锛?
3. Hardware Tag-Based KASAN锛堝熀浜庣‖浠舵爣绛剧殑 KASAN锛?

Generic KASAN锛岄€氳繃 CONFIG_KASAN_GENERIC 鍚敤锛屾槸闈㈠悜璋冭瘯鐨勬ā寮忥紝绫讳技浜庣敤鎴风┖闂?ASan銆傝妯″紡鏀寔澶氱 CPU 鏋舵瀯锛屼絾鍏锋湁鏄捐憲鐨勬€ц兘鍜屽唴瀛樺紑閿€銆?

Software Tag-Based KASAN锛堟垨绉?SW_TAGS KASAN锛夛紝閫氳繃 CONFIG_KASAN_SW_TAGS 鍚敤锛屽彲鐢ㄤ簬璋冭瘯鍜?dogfood 娴嬭瘯锛岀被浼间簬鐢ㄦ埛绌洪棿 HWASan銆傝妯″紡浠呮敮鎸?arm64锛屼絾鍏堕€傚害鐨勫唴瀛樺紑閿€鍏佽鍦ㄥ彈鍐呭瓨闄愬埗鐨勮澶囦笂浠ョ湡瀹炲伐浣滆礋杞借繘琛屾祴璇曘€?

Hardware Tag-Based KASAN锛堟垨绉?HW_TAGS KASAN锛夛紝閫氳繃 CONFIG_KASAN_HW_TAGS 鍚敤锛屾槸鏃ㄥ湪鐢ㄤ綔鐜板満鍐呭瓨缂洪櫡妫€娴嬪櫒鎴栧畨鍏ㄧ紦瑙ｆ帾鏂界殑鐨勬ā寮忋€傝妯″紡浠呴€傜敤浜庢敮鎸?MTE锛圡emory Tagging Extension锛屽唴瀛樻爣璁版墿灞曪級鐨?arm64 CPU锛屼絾鍏跺唴瀛樺拰鎬ц兘寮€閿€寰堜綆锛屽洜姝ゅ彲鐢ㄤ簬鐢熶骇鐜銆?

鍏充簬姣忕 KASAN 妯″紡鐨勫唴瀛樹笌鎬ц兘褰卞搷锛岃瑙佺浉搴?Kconfig 閫夐」鐨勬弿杩般€?

Generic 鍜?Software Tag-Based 妯″紡閫氬父琚О涓鸿蒋浠舵ā寮忋€係oftware Tag-Based 鍜?Hardware Tag-Based 妯″紡琚О涓哄熀浜庢爣绛剧殑妯″紡銆?

### Support锛堟敮鎸侊級


#### Architectures锛堟灦鏋勶級


Generic KASAN 鏀寔 x86_64銆乤rm銆乤rm64銆乸owerpc銆乺iscv銆乻390銆亁tensa 鍜?loongarch锛岃€屽熀浜庢爣绛剧殑 KASAN 妯″紡浠呮敮鎸?arm64銆?

#### Compilers锛堢紪璇戝櫒锛?


杞欢 KASAN 妯″紡浣跨敤缂栬瘧鏈熸彃妗╋紝鍦ㄦ瘡娆″唴瀛樿闂墠鎻掑叆鏈夋晥鎬ф鏌ワ紝鍥犳闇€瑕佹彁渚涙敮鎸佽鐗规€х殑缂栬瘧鍣ㄧ増鏈€傚熀浜庣‖浠舵爣绛剧殑妯″紡渚濊禆纭欢鎵ц杩欎簺妫€鏌ワ紝浣嗕粛闇€瑕佹敮鎸佸唴瀛樻爣璁版寚浠ょ殑缂栬瘧鍣ㄧ増鏈€?

Generic KASAN 闇€瑕?GCC 8.3.0 鎴栨洿楂樼増鏈紝鎴栧唴鏍告敮鎸佺殑浠讳綍 Clang 鐗堟湰銆?

Software Tag-Based KASAN 闇€瑕?GCC 11+ 鎴栧唴鏍告敮鎸佺殑浠讳綍 Clang 鐗堟湰銆?

Hardware Tag-Based KASAN 闇€瑕?GCC 10+ 鎴?Clang 12+銆?

#### Memory types锛堝唴瀛樼被鍨嬶級


Generic KASAN 鏀寔鍦?slab銆乸age_alloc銆乿map銆乿malloc銆乻tack 鍜?global 鍐呭瓨涓彂鐜扮己闄枫€?

Software Tag-Based KASAN 鏀寔 slab銆乸age_alloc銆乿malloc 鍜?stack 鍐呭瓨銆?

Hardware Tag-Based KASAN 鏀寔 slab銆乸age_alloc 鍜岄潪鍙墽琛?vmalloc 鍐呭瓨銆?

### Usage锛堢敤娉曪級


```
	  CONFIG_KASAN=y
```
骞朵粠 `CONFIG_KASAN_GENERIC`锛堝惎鐢?Generic KASAN锛夈€乣CONFIG_KASAN_SW_TAGS`锛堝惎鐢?Software Tag-Based KASAN锛夊拰 `CONFIG_KASAN_HW_TAGS`锛堝惎鐢?Hardware Tag-Based KASAN锛変腑閫夋嫨銆?

瀵逛簬杞欢妯″紡锛岃繕瑕佸湪 `CONFIG_KASAN_OUTLINE` 鍜?`CONFIG_KASAN_INLINE` 涔嬮棿閫夋嫨銆俹utline 鍜?inline 鏄紪璇戝櫒鎻掓々绫诲瀷銆傚墠鑰呯敓鎴愯緝灏忕殑浜岃繘鍒舵枃浠讹紝鑰屽悗鑰呴€熷害蹇嚦 2 鍊嶃€?

瑕佸皢鍙楀奖鍝?slab 瀵硅薄鐨勫垎閰嶄笌閲婃斁鏍堝洖婧撼鍏ユ姤鍛婏紝鍚敤 `CONFIG_STACKTRACE`銆傝鍖呭惈鍙楀奖鍝嶇墿鐞嗛〉鐨勫垎閰嶄笌閲婃斁鏍堝洖婧紝鍚敤 `CONFIG_PAGE_OWNER` 骞朵互 `page_owner=on` 鍚姩銆?

#### Boot parameters锛堝惎鍔ㄥ弬鏁帮級


KASAN 鍙楅€氱敤鐨?`panic_on_warn` 鍛戒护琛屽弬鏁板奖鍝嶃€傚綋瀹冨惎鐢ㄦ椂锛孠ASAN 浼氬湪鎵撳嵃缂洪櫡鎶ュ憡鍚庝娇鍐呮牳 panic銆?

榛樿鎯呭喌涓嬶紝KASAN 浠呴拡瀵圭涓€娆℃棤鏁堝唴瀛樿闂墦鍗扮己闄锋姤鍛娿€備娇鐢?`kasan_multi_shot` 鏃讹紝KASAN 浼氬湪姣忔鏃犳晥璁块棶鏃舵墦鍗版姤鍛娿€傝繖瀹為檯涓婁负 KASAN 鎶ュ憡绂佺敤浜?`panic_on_warn`銆?

鎴栬€咃紝鐙珛浜?`panic_on_warn`锛宍kasan.fault=` 鍚姩鍙傛暟鍙敤浜庢帶鍒?panic 鍜屾姤鍛婅涓猴細

- `kasan.fault=report`銆乣=panic` 鎴?`=panic_on_write` 鎺у埗鏄粎鎵撳嵃 KASAN 鎶ュ憡銆佷娇鍐呮牳 panic锛岃繕鏄粎鍦ㄦ棤鏁堝啓璁块棶鏃朵娇鍐呮牳 panic锛堥粯璁わ細`report`锛夈€傚嵆浣垮惎鐢ㄤ簡 `kasan_multi_shot`锛屼篃浼氬彂鐢?panic銆傛敞鎰忥紝褰撲娇鐢?Hardware Tag-Based KASAN 鐨勫紓姝ユā寮忔椂锛宍kasan.fault=panic_on_write` 鎬绘槸瀵瑰紓姝ユ鏌ョ殑璁块棶锛堝寘鎷锛夎Е鍙?panic銆?

Software 鍜?Hardware Tag-Based KASAN 妯″紡锛堣涓嬫枃鍏充簬鍚勭妯″紡鐨勭珷鑺傦級鏀寔鏀瑰彉鏍堝洖婧敹闆嗚涓猴細

- `kasan.stacktrace=off` 鎴?`=on` 绂佺敤鎴栧惎鐢ㄥ垎閰嶄笌閲婃斁鏍堝洖婧殑鏀堕泦锛堥粯璁わ細`on`锛夈€?
- `kasan.stack_ring_size=<number of entries>` 鎸囧畾鏍堢幆锛坰tack ring锛変腑鐨勬潯鐩暟锛堥粯璁わ細`32768`锛夈€?

Hardware Tag-Based KASAN 妯″紡鏃ㄥ湪鐢ㄤ綔鐢熶骇鐜涓殑瀹夊叏缂撹В鎺柦銆傚洜姝わ紝瀹冩敮鎸侀澶栫殑鍚姩鍙傛暟锛屽厑璁稿畬鍏ㄧ鐢?KASAN 鎴栨帶鍒跺叾鐗规€э細

- `kasan=off` 鎴?`=on` 鎺у埗鏄惁鍚敤 KASAN锛堥粯璁わ細`on`锛夈€?

- `kasan.mode=sync`銆乣=async` 鎴?`=asymm` 鎺у埗 KASAN 閰嶇疆涓哄悓姝ャ€佸紓姝ユ垨闈炲绉版墽琛屾ā寮忥紙榛樿锛歚sync`锛夈€?
  鍚屾妯″紡锛氬綋鍙戠敓鏍囩妫€鏌ユ晠闅滄椂锛岀珛鍗虫娴嬪埌閿欒璁块棶銆?
  寮傛妯″紡锛氶敊璇闂殑妫€娴嬭寤惰繜銆傚綋鍙戠敓鏍囩妫€鏌ユ晠闅滄椂锛屼俊鎭瓨鍌ㄥ湪纭欢涓紙瀵逛簬 arm64锛屽瓨鍌ㄥ湪 TFSR_EL1 瀵勫瓨鍣ㄤ腑锛夈€傚唴鏍稿畾鏈熸鏌ョ‖浠讹紝浠呭湪杩欎簺妫€鏌ユ湡闂存姤鍛婃爣绛炬晠闅溿€?
  闈炲绉版ā寮忥細閿欒璁块棶鍦ㄨ鏃跺悓姝ユ娴嬶紝鍦ㄥ啓鏃跺紓姝ユ娴嬨€?

- `kasan.write_only=off` 鎴?`kasan.write_only=on` 鎺у埗 KASAN 鏄粎妫€鏌ュ啓锛坰tore锛夎闂繕鏄鏌ユ墍鏈夎闂紙榛樿锛歚off`锛夈€?

- `kasan.vmalloc=off` 鎴?`=on` 绂佺敤鎴栧惎鐢?vmalloc 鍒嗛厤鐨勬爣璁帮紙榛樿锛歚on`锛夈€?

- `kasan.page_alloc.sample=<閲囨牱闂撮殧>` 浣?KASAN 浠呭姣忕 N 涓?order 绛変簬鎴栧ぇ浜?`kasan.page_alloc.sample.order` 鐨?page_alloc 鍒嗛厤杩涜鏍囪锛屽叾涓?N 涓?`sample` 鍙傛暟鐨勫€硷紙榛樿锛歚1`锛屽嵆瀵规瘡涓绫诲垎閰嶉兘鏍囪锛夈€?
  璇ュ弬鏁版棬鍦ㄧ紦瑙?KASAN 寮曞叆鐨勬€ц兘寮€閿€銆?
  娉ㄦ剰锛屽惎鐢ㄦ鍙傛暟浼氫娇 Hardware Tag-Based KASAN 璺宠繃瀵归噰鏍锋墍閫夊垎閰嶇殑妫€鏍革紝浠庤€屾紡鎺夊杩欎簺鍒嗛厤鐨勫潖璁块棶銆備负鍑嗙‘妫€娴嬬己闄凤紝璇蜂娇鐢ㄩ粯璁ゅ€笺€?

- `kasan.page_alloc.sample.order=<鏈€灏忛〉 order>` 鎸囧畾鍙楅噰鏍峰奖鍝嶇殑鍒嗛厤鐨勬渶灏?order锛堥粯璁わ細`3`锛夈€?
  浠呭綋 `kasan.page_alloc.sample` 璁剧疆涓哄ぇ浜?`1` 鐨勫€兼椂閫傜敤銆?
  璇ュ弬鏁版棬鍦ㄤ粎鍏佽瀵瑰ぇ鍨?page_alloc 鍒嗛厤杩涜閲囨牱锛岃繖绫诲垎閰嶆槸鎬ц兘寮€閿€鐨勬渶澶ф潵婧愩€?

#### Error reports锛堥敊璇姤鍛婏級


```
    ==================================================================
    BUG: KASAN: slab-out-of-bounds in kmalloc_oob_right+0xa8/0xbc [kasan_test]
    Write of size 1 at addr ffff8801f44ec37b by task insmod/2760

    CPU: 1 PID: 2760 Comm: insmod Not tainted 4.19.0-rc3+ #698
    Hardware name: QEMU Standard PC (i440FX + PIIX, 1996), BIOS 1.10.2-1 04/01/2014
    Call Trace:
     dump_stack+0x94/0xd8
     print_address_description+0x73/0x280
     kasan_report+0x144/0x187
     __asan_report_store1_noabort+0x17/0x20
     kmalloc_oob_right+0xa8/0xbc [kasan_test]
     kmalloc_tests_init+0x16/0x700 [kasan_test]
     do_one_initcall+0xa5/0x3ae
     do_init_module+0x1b6/0x547
     load_module+0x75df/0x8070
     __do_sys_init_module+0x1c6/0x200
     __x64_sys_init_module+0x6e/0xb0
     do_syscall_64+0x9f/0x2c0
     entry_SYSCALL_64_after_hwframe+0x44/0xa9
    RIP: 0033:0x7f96443109da
    RSP: 002b:00007ffcf0b51b08 EFLAGS: 00000202 ORIG_RAX: 00000000000000af
    RAX: ffffffffffffffda RBX: 000055dc3ee521a0 RCX: 00007f96443109da
    RDX: 00007f96445cff88 RSI: 0000000000057a50 RDI: 00007f9644992000
    RBP: 000055dc3ee510b0 R08: 0000000000000003 R09: 0000000000000000
    R10: 00007f964430cd0a R11: 0000000000000202 R12: 00007f96445cff88
    R13: 000055dc3ee51090 R14: 0000000000000000 R15: 0000000000000000

    Allocated by task 2760:
     save_stack+0x43/0xd0
     kasan_kmalloc+0xa7/0xd0
     kmem_cache_alloc_trace+0xe1/0x1b0
     kmalloc_oob_right+0x56/0xbc [kasan_test]
     kmalloc_tests_init+0x16/0x700 [kasan_test]
     do_one_initcall+0xa5/0x3ae
     do_init_module+0x1b6/0x547
     load_module+0x75df/0x8070
     __do_sys_init_module+0x1c6/0x200
     __x64_sys_init_module+0x6e/0xb0
     do_syscall_64+0x9f/0x2c0
     entry_SYSCALL_64_after_hwframe+0x44/0xa9

    Freed by task 815:
     save_stack+0x43/0xd0
     __kasan_slab_free+0x135/0x190
     kasan_slab_free+0xe/0x10
     kfree+0x93/0x1a0
     umh_complete+0x6a/0xa0
     call_usermodehelper_exec_async+0x4c3/0x640
     ret_from_fork+0x35/0x40

    The buggy address belongs to the object at ffff8801f44ec300
     which belongs to the cache kmalloc-128 of size 128
    The buggy address is located 123 bytes inside of
     128-byte region [ffff8801f44ec300, ffff8801f44ec380)
    The buggy address belongs to the page:
    page:ffffea0007d13b00 count:1 mapcount:0 mapping:ffff8801f7001640 index:0x0
    flags: 0x200000000000100(slab)
    raw: 0200000000000100 ffffea0007d11dc0 0000001a0000001a ffff8801f7001640
    raw: 0000000000000000 0000000080150015 00000001ffffffff 0000000000000000
    page dumped because: kasan: bad access detected

    Memory state around the buggy address:
     ffff8801f44ec200: fc fc fc fc fc fc fc fc fb fb fb fb fb fb fb fb
     ffff8801f44ec280: fb fb fb fb fb fb fb fb fc fc fc fc fc fc fc fc
    >ffff8801f44ec300: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 03
                                                                    ^
     ffff8801f44ec380: fc fc fc fc fc fc fc fc fb fb fb fb fb fb fb fb
     ffff8801f44ec400: fb fb fb fb fb fb fb fb fc fc fc fc fc fc fc fc
    ==================================================================
```

鎶ュ憡澶撮儴姒傛嫭浜嗗彂鐢熶簡浣曠缂洪櫡浠ュ強鐢变綍绉嶈闂紩璧枫€傚叾鍚庤窡闅忛敊璇闂殑鏍堝洖婧€佽璁块棶鍐呭瓨琚垎閰嶄綅缃殑鏍堝洖婧紙鑻ヨ闂殑鏄?slab 瀵硅薄锛夛紝浠ュ強瀵硅薄琚噴鏀句綅缃殑鏍堝洖婧紙鑻ユ槸 use-after-free 缂洪櫡鎶ュ憡锛夈€傛帴涓嬫潵鏄墍璁块棶 slab 瀵硅薄鐨勬弿杩颁互鍙婅璁块棶鍐呭瓨椤电殑淇℃伅銆?

鏈€鍚庯紝鎶ュ憡灞曠ず琚闂湴鍧€鍛ㄥ洿鐨勫唴瀛樼姸鎬併€傚湪鍐呴儴锛孠ASAN 瀵规瘡涓唴瀛橀绮掞紙memory granule锛夊崟鐙窡韪唴瀛樼姸鎬侊紝璇ラ绮掓牴鎹?KASAN 妯″紡涓?8 鎴?16 瀛楄妭瀵归綈銆傛姤鍛婂唴瀛樼姸鎬侀儴鍒嗕腑鐨勬瘡涓暟瀛楁樉绀哄洿缁曡璁块棶鍦板潃鐨勬煇涓唴瀛橀绮掔殑鐘舵€併€?

瀵逛簬 Generic KASAN锛屾瘡涓唴瀛橀绮掔殑澶у皬涓?8銆傛瘡涓绮掔殑鐘舵€佺紪鐮佸湪涓€涓奖瀛愬瓧鑺傦紙shadow byte锛変腑銆傝繖 8 涓瓧鑺傚彲浠ユ槸鍙闂殑銆侀儴鍒嗗彲璁块棶鐨勩€佸凡閲婃斁鐨勶紝鎴栨槸 redzone 鐨勪竴閮ㄥ垎銆侹ASAN 瀵规瘡涓奖瀛愬瓧鑺備娇鐢ㄥ涓嬬紪鐮侊細00 琛ㄧず瀵瑰簲鍐呭瓨鍖哄煙鐨勫叏閮?8 涓瓧鑺傚潎鍙闂紱鏁板瓧 N锛? <= N <= 7锛夎〃绀哄墠 N 涓瓧鑺傚彲璁块棶锛屽叾浣欙紙8 - N锛変釜瀛楄妭涓嶅彲璁块棶锛涗换浣曡礋鍊艰〃绀烘暣涓?8 瀛楄妭瀛椾笉鍙闂€侹ASAN 浣跨敤涓嶅悓鐨勮礋鍊兼潵鍖哄垎涓嶅悓绫诲瀷涓嶅彲璁块棶鍐呭瓨锛屽 redzone 鎴栧凡閲婃斁鍐呭瓨锛堣 mm/kasan/kasan.h锛夈€?

鍦ㄤ笂闈㈢殑鎶ュ憡涓紝绠ご鎸囧悜褰卞瓙瀛楄妭 `03`锛岃繖鎰忓懗鐫€琚闂湴鍧€鏄儴鍒嗗彲璁块棶鐨勩€?

瀵逛簬鍩轰簬鏍囩鐨?KASAN 妯″紡锛岃繖鏈€鍚庝竴閮ㄥ垎鎶ュ憡鏄剧ず琚闂湴鍧€鍛ㄥ洿鐨勫唴瀛樻爣绛撅紙瑙?`Implementation details`_ 绔犺妭锛夈€?

娉ㄦ剰锛孠ASAN 缂洪櫡鏍囬锛堝 `slab-out-of-bounds` 鎴?`use-after-free`锛夋槸灏藉姏鑰屼负鐨勶細KASAN 鏍规嵁鍏舵墍鎷ユ湁鐨勬湁闄愪俊鎭墦鍗版渶鍙兘鐨勭己闄风被鍨嬨€傚疄闄呯己闄风被鍨嬪彲鑳戒笉鍚屻€?

Generic KASAN 杩樹細鎶ュ憡鏈€澶氫袱鏉¤緟鍔╄皟鐢ㄦ爤鍥炴函銆傝繖浜涙爤鍥炴函鎸囧悜涓庡璞′氦浜掍絾鏈洿鎺ュ嚭鐜板湪閿欒璁块棶鏍堝洖婧腑鐨勪唬鐮佷綅缃€傜洰鍓嶏紝杩欏寘鎷?call_rcu() 鍜?workqueue 鎺掗槦銆?

#### CONFIG_KASAN_EXTRA_INFO


鍚敤 CONFIG_KASAN_EXTRA_INFO 鍏佽 KASAN 璁板綍骞舵姤鍛婃洿澶氫俊鎭€傚綋鍓嶆敮鎸佺殑棰濆淇℃伅鏄垎閰嶄笌閲婃斁鏃剁殑 CPU 缂栧彿鍜屾椂闂存埑銆傛洿澶氫俊鎭湁鍔╀簬鎵惧埌缂洪櫡鍘熷洜骞跺皢閿欒涓庡叾浠栫郴缁熶簨浠跺叧鑱旓紝浠ｄ环鏄娇鐢ㄩ澶栧唴瀛樻潵璁板綍鏇村淇℃伅锛堟洿澶氫唬浠风粏鑺傝 CONFIG_KASAN_EXTRA_INFO 鐨勫府鍔╂枃鏈級銆?

浠ヤ笅鏄惎鐢?CONFIG_KASAN_EXTRA_INFO 鍚庣殑鎶ュ憡锛堜粎
```
    ==================================================================
    ...
    Allocated by task 134 on cpu 5 at 229.133855s:
    ...
    Freed by task 136 on cpu 3 at 230.199335s:
    ...
    ==================================================================
```

### Implementation details锛堝疄鐜扮粏鑺傦級


#### Generic KASAN


杞欢 KASAN 妯″紡浣跨敤褰卞瓙鍐呭瓨鏉ヨ褰曟瘡涓唴瀛樺瓧鑺傛槸鍚﹀彲瀹夊叏璁块棶锛屽苟浣跨敤缂栬瘧鏈熸彃妗╁湪姣忔鍐呭瓨璁块棶鍓嶆彃鍏ュ奖瀛愬唴瀛樻鏌ャ€?

Generic KASAN 灏嗗叾褰卞瓙鍐呭瓨鍗犱负鍐呮牳鍐呭瓨鐨?1/8锛堝湪 x86_64 涓婁负 16TB 浠ヨ鐩?128TB锛夛紝骞朵娇鐢ㄥ甫姣斾緥鍜屽亸绉荤殑鐩存槧灏勫皢鍐呭瓨鍦板潃杞崲涓哄叾瀵瑰簲鐨勫奖瀛愬湴鍧€銆?

浠ヤ笅鏄敤浜庡皢鍦板潃杞崲涓哄叾瀵瑰簲褰卞瓙鍦板潃鐨勫嚱鏁?
```
    static inline void *kasan_mem_to_shadow(const void *addr)
    {
	return (void *)((unsigned long)addr >> KASAN_SHADOW_SCALE_SHIFT)
		+ KASAN_SHADOW_OFFSET;
    }
```
鍏朵腑 `KASAN_SHADOW_SCALE_SHIFT = 3`銆?

缂栬瘧鏈熸彃妗╃敤浜庢彃鍏ュ唴瀛樿闂鏌ャ€傜紪璇戝櫒鍦ㄦ瘡娆″ぇ灏忎负 1銆?銆?銆? 鎴?16 鐨勫唴瀛樿闂墠鎻掑叆鍑芥暟璋冪敤锛坄__asan_load**(addr)`銆乣__asan_store**(addr)`锛夈€傝繖浜涘嚱鏁伴€氳繃妫€鏌ュ搴旂殑褰卞瓙鍐呭瓨鏉ュ垽鏂唴瀛樿闂槸鍚︽湁鏁堛€?

浣跨敤 inline 鎻掓々鏃讹紝缂栬瘧鍣ㄤ笉鐩存帴杩涜鍑芥暟璋冪敤锛岃€屾槸鐩存帴鎻掑叆妫€鏌ュ奖瀛愬唴瀛樼殑浠ｇ爜銆傛閫夐」鏄捐憲澧炲ぇ鍐呮牳浣撶Н锛屼絾鐩告瘮 outline 鎻掓々鐨勫唴鏍稿甫鏉?x1.1-x2 鐨勬€ц兘鎻愬崌銆?

Generic KASAN 鏄敮涓€閫氳繃闅旂鍖猴紙quarantine锛夊欢杩熼噴鏀惧璞￠噸鐢ㄧ殑妯″紡锛堝疄鐜拌 mm/kasan/quarantine.c锛夈€?

#### Software Tag-Based KASAN


Software Tag-Based KASAN 浣跨敤杞欢鍐呭瓨鏍囪鏂规硶鏉ユ鏌ヨ闂湁鏁堟€с€傜洰鍓嶄粎閽堝 arm64 鏋舵瀯瀹炵幇銆?

Software Tag-Based KASAN 浣跨敤 arm64 CPU 鐨?Top Byte Ignore (TBI) 鐗规€э紝鍦ㄥ唴鏍告寚閽堢殑鏈€楂樺瓧鑺備腑瀛樺偍鎸囬拡鏍囩銆傚畠浣跨敤褰卞瓙鍐呭瓨瀛樺偍涓庢瘡涓?16 瀛楄妭鍐呭瓨鍗曞厓鍏宠仈鐨勫唴瀛樻爣绛撅紙鍥犳锛屽畠鍗犲唴鏍稿唴瀛樼殑 1/16 鐢ㄤ簬褰卞瓙鍐呭瓨锛夈€?

鍦ㄦ瘡娆″唴瀛樺垎閰嶆椂锛孲oftware Tag-Based KASAN 鐢熸垚涓€涓殢鏈烘爣绛撅紝鐢ㄦ鏍囩鏍囪宸插垎閰嶅唴瀛橈紝骞跺皢鍚屼竴鏍囩宓屽叆杩斿洖鐨勬寚閽堜腑銆?

Software Tag-Based KASAN 浣跨敤缂栬瘧鏈熸彃妗╁湪姣忔鍐呭瓨璁块棶鍓嶆彃鍏ユ鏌ャ€傝繖浜涙鏌ョ‘淇濊璁块棶鍐呭瓨鐨勬爣绛剧瓑浜庣敤浜庤闂鍐呭瓨鐨勬寚閽堢殑鏍囩銆傝嫢鍙戠敓鏍囩涓嶅尮閰嶏紝Software Tag-Based KASAN 鎵撳嵃缂洪櫡鎶ュ憡銆?

Software Tag-Based KASAN 涔熸湁涓ょ鎻掓々妯″紡锛坥utline锛屽彂鍑哄洖璋冧互妫€鏌ュ唴瀛樿闂紱浠ュ強 inline锛屽唴鑱旀墽琛屽奖瀛愬唴瀛樻鏌ワ級銆傚湪 outline 鎻掓々妯″紡涓嬶紝缂洪櫡鎶ュ憡鐢辨墽琛岃闂鏌ョ殑鍑芥暟鎵撳嵃銆傚湪 inline 鎻掓々妯″紡涓嬶紝缂栬瘧鍣ㄥ彂鍑?`brk` 鎸囦护锛屽苟浣跨敤涓撶敤鐨?`brk` 澶勭悊绋嬪簭鏉ユ墦鍗扮己闄锋姤鍛娿€?

Software Tag-Based KASAN 浣跨敤 0xFF 浣滀负 match-all 鎸囬拡鏍囩锛堥€氳繃甯︽湁 0xFF 鎸囬拡鏍囩鐨勬寚閽堣繘琛岀殑璁块棶涓嶈妫€鏌ワ級銆傚€?0xFE 褰撳墠淇濈暀鐢ㄤ簬鏍囪宸查噴鏀剧殑鍐呭瓨鍖哄煙銆?

#### Hardware Tag-Based KASAN


Hardware Tag-Based KASAN 鍦ㄦ蹇典笂绫讳技浜庤蒋浠舵ā寮忥紝浣嗕娇鐢ㄧ‖浠跺唴瀛樻爣璁版敮鎸侊紝鑰岄潪缂栬瘧鍣ㄦ彃妗╁拰褰卞瓙鍐呭瓨銆?

Hardware Tag-Based KASAN 鐩墠浠呴拡瀵?arm64 鏋舵瀯瀹炵幇锛屽苟鍩轰簬 ARMv8.5 鎸囦护闆嗘灦鏋勫紩鍏ョ殑 arm64 Memory Tagging Extension (MTE) 浠ュ強 Top Byte Ignore (TBI)銆?

涓撶敤鐨?arm64 鎸囦护鐢ㄤ簬涓烘瘡涓垎閰嶅垎閰嶅唴瀛樻爣绛俱€傜浉鍚岀殑鏍囩琚垎閰嶇粰鎸囧悜杩欎簺鍒嗛厤鐨勬寚閽堛€傚湪姣忔鍐呭瓨璁块棶鏃讹紝纭欢纭繚琚闂唴瀛樼殑鏍囩绛変簬鐢ㄤ簬璁块棶璇ュ唴瀛樼殑鎸囬拡鐨勬爣绛俱€傝嫢鍙戠敓鏍囩涓嶅尮閰嶏紝鍒欑敓鎴愭晠闅滃苟鎵撳嵃鎶ュ憡銆?

Hardware Tag-Based KASAN 浣跨敤 0xFF 浣滀负 match-all 鎸囬拡鏍囩锛堥€氳繃甯︽湁 0xFF 鎸囬拡鏍囩鐨勬寚閽堣繘琛岀殑璁块棶涓嶈妫€鏌ワ級銆傚€?0xFE 褰撳墠淇濈暀鐢ㄤ簬鏍囪宸查噴鏀剧殑鍐呭瓨鍖哄煙銆?

鑻ョ‖浠朵笉鏀寔 MTE锛圓RMv8.5 涔嬪墠锛夛紝Hardware Tag-Based KASAN 灏嗕笉浼氳鍚敤銆傚湪杩欑鎯呭喌涓嬶紝鎵€鏈?KASAN 鍚姩鍙傛暟鍧囪蹇界暐銆?

娉ㄦ剰锛屽惎鐢?CONFIG_KASAN_HW_TAGS 鎬绘槸浼氬鑷村唴鏍稿唴 TBI 琚惎鐢ㄣ€傚嵆浣挎彁渚涗簡 `kasan.mode=off`锛屾垨纭欢涓嶆敮鎸?MTE锛堜絾鏀寔 TBI锛夈€?

Hardware Tag-Based KASAN 浠呮姤鍛婂彂鐜扮殑绗竴涓己闄枫€傛鍚庯紝MTE 鏍囩妫€鏌ヨ绂佺敤銆?

### Shadow memory锛堝奖瀛愬唴瀛橈級


鏈妭鍐呭浠呴€傜敤浜庤蒋浠?KASAN 妯″紡銆?

鍐呮牳鍦ㄥ湴鍧€绌洪棿鐨勫涓笉鍚岄儴鍒嗘槧灏勫唴瀛樸€傚唴鏍歌櫄鎷熷湴鍧€鐨勮寖鍥村緢澶э細娌℃湁瓒冲鐨勭墿鐞嗗唴瀛樻潵涓哄唴鏍稿彲鑳借闂殑姣忎釜鍦板潃鏀寔鐪熷疄鐨勫奖瀛愬尯鍩熴€傚洜姝わ紝KASAN 浠呬负鍦板潃绌洪棿鐨勬煇浜涢儴鍒嗘槧灏勭湡瀹炵殑褰卞瓙銆?

#### Default behaviour锛堥粯璁よ涓猴級


榛樿鎯呭喌涓嬶紝鏋舵瀯浠呬负绾挎€ф槧灏勶紙浠ュ強娼滃湪鐨勫叾浠栧皬閮ㄥ垎鍖哄煙锛変箣涓婄殑褰卞瓙鍖哄煙鏄犲皠鐪熷疄鍐呭瓨銆傚浜庢墍鏈夊叾浠栧尯鍩熲€斺€斿 vmalloc 鍜?vmemmap 绌洪棿鈥斺€斿湪褰卞瓙鍖哄煙涔嬩笂鏄犲皠鍗曚釜鍙椤点€傝繖涓彧璇诲奖瀛愰〉灏嗘墍鏈夊唴瀛樿闂０鏄庝负鍏佽銆?

杩欑粰妯″潡甯︽潵浜嗛棶棰橈細瀹冧滑涓嶄綅浜庣嚎鎬ф槧灏勪腑锛岃€屾槸浣嶄簬涓撶敤鐨勬ā鍧楃┖闂淬€傞€氳繃鎸傛帴锛坔ook锛夋ā鍧楀垎閰嶅櫒锛孠ASAN 涓存椂鏄犲皠鐪熷疄褰卞瓙鍐呭瓨鏉ヨ鐩栧畠浠€備緥濡傦紝杩欏厑璁告娴嬪妯″潡鍏ㄥ眬鍙橀噺鐨勬棤鏁堣闂€?

杩欎篃閫犳垚浜嗕笌 `VMAP_STACK` 鐨勪笉鍏煎锛氳嫢鏍堜綅浜?vmalloc 绌洪棿涓紝瀹冨皢琚鍙椤甸伄钄斤紝鍐呮牳鍦ㄥ皾璇曚负鏍堝彉閲忓缓绔嬪奖瀛愭暟鎹椂灏嗗嚭閿欍€?

#### CONFIG_KASAN_VMALLOC


閫氳繃 `CONFIG_KASAN_VMALLOC`锛孠ASAN 鍙互浠ユ洿澶х殑鍐呭瓨浣跨敤涓轰唬浠疯鐩?vmalloc 绌洪棿銆傜洰鍓嶏紝杩欏湪 x86銆乤rm64銆乺iscv銆乻390 鍜?powerpc 涓婂彈鏀寔銆?

鍏跺伐浣滄柟寮忔槸閫氳繃鎸傛帴 vmalloc 鍜?vmap锛屽苟鍔ㄦ€佸垎閰嶇湡瀹炲奖瀛愬唴瀛樻潵鏀拺鏄犲皠銆?

vmalloc 绌洪棿涓殑澶у鏁版槧灏勯兘寰堝皬锛岄渶瑕佺殑褰卞瓙绌洪棿涓嶈冻涓€鏁撮〉銆傚洜姝わ紝涓烘瘡涓槧灏勫垎閰嶄竴鏁撮〉褰卞瓙椤靛皢鏄氮璐圭殑銆傛澶栵紝涓虹‘淇濅笉鍚岀殑鏄犲皠浣跨敤涓嶅悓鐨勫奖瀛愰〉锛屾槧灏勫繀椤讳笌 `KASAN_GRANULE_SIZE * PAGE_SIZE` 瀵归綈銆?

鐩稿弽锛孠ASAN 鍦ㄥ涓槧灏勪箣闂村叡浜敮鎾戠┖闂淬€傚綋 vmalloc 绌洪棿涓殑鏄犲皠浣跨敤褰卞瓙鍖哄煙鐨勬煇涓壒瀹氶〉鏃讹紝瀹冨垎閰嶄竴涓敮鎾戦〉銆傝椤典箣鍚庡彲琚叾浠?vmalloc 鏄犲皠鍏变韩銆?

KASAN 鎸傛帴 vmap 鍩虹璁炬柦锛屼互鎯版€ф竻鐞嗘湭浣跨敤鐨勫奖瀛愬唴瀛樸€?

涓洪伩鍏嶅洿缁曟槧灏勪氦鎹㈢殑鍥伴毦锛孠ASAN 鏈熸湜瑕嗙洊 vmalloc 绌洪棿鐨勫奖瀛愬尯鍩熼儴鍒嗕笉琚棭鏈熷奖瀛愰〉瑕嗙洊锛岃€屾槸淇濇寔鏈槧灏勩€傝繖灏嗛渶瑕佹灦鏋勭浉鍏充唬鐮佺殑鏀瑰姩銆?

杩欏厑璁稿湪 x86 涓婃敮鎸?`VMAP_STACK`锛屽苟鍙畝鍖栧娌℃湁鍥哄畾妯″潡鍖哄煙鐨勬灦鏋勭殑鏀寔銆?

### For developers锛堥潰鍚戝紑鍙戣€咃級


#### Ignoring accesses锛堝拷鐣ヨ闂級


杞欢 KASAN 妯″紡浣跨敤缂栬瘧鍣ㄦ彃妗╂潵鎻掑叆鏈夋晥鎬ф鏌ャ€傛绫绘彃妗╁彲鑳戒笌鍐呮牳鐨勬煇浜涢儴鍒嗕笉鍏煎锛屽洜姝ら渶瑕佽绂佺敤銆?

鍐呮牳鐨勫叾浠栭儴鍒嗗彲鑳借闂凡鍒嗛厤瀵硅薄鐨勫厓鏁版嵁銆傞€氬父锛孠ASAN 浼氭娴嬪苟鎶ュ憡姝ょ被璁块棶锛屼絾鍦ㄦ煇浜涙儏鍐典笅锛堜緥濡傦紝鍦ㄥ唴瀛樺垎閰嶅櫒涓級锛岃繖浜涜闂槸鏈夋晥鐨勩€?

瀵逛簬杞欢 KASAN 妯″紡锛岃涓虹壒瀹氭枃浠舵垨鐩綍绂佺敤鎻掓々锛岃鍚戠浉搴旂殑鍐呮牳 Makefile 娣诲姞 `KASAN_SANITIZE` 娉ㄨВ锛?

```
    KASAN_SANITIZE_main.o := n
```
```
    KASAN_SANITIZE := n
```
瀵逛簬杞欢 KASAN 妯″紡锛岃浠ラ€愬嚱鏁版柟寮忕鐢ㄦ彃妗╋紝浣跨敤 KASAN 鐗瑰畾鐨?`__no_sanitize_address` 鍑芥暟灞炴€ф垨閫氱敤鐨?`noinstr` 灞炴€с€?

娉ㄦ剰锛岀鐢ㄧ紪璇戝櫒鎻掓々锛堟棤璁烘槸鎸夋枃浠惰繕鏄寜鍑芥暟锛変細浣?KASAN 蹇界暐璇ヤ唬鐮佷腑鐩存帴鍙戠敓鐨勮闂紙閽堝杞欢 KASAN 妯″紡锛夈€傚綋璁块棶闂存帴鍙戠敓锛堥€氳繃瀵规彃妗╁嚱鏁扮殑璋冪敤锛夋垨浣跨敤涓嶄娇鐢ㄧ紪璇戝櫒鎻掓々鐨?Hardware Tag-Based KASAN 鏃讹紝瀹冩棤娴庝簬浜嬨€?

瀵逛簬杞欢 KASAN 妯″紡锛岃閽堝褰撳墠浠诲姟鍦ㄥ唴鏍镐唬鐮佺殑涓€閮ㄥ垎涓鐢?KASAN 鎶ュ憡锛岃鐢?`kasan_disable_current()`/`kasan_enable_current()` 鍖烘娉ㄨВ璇ラ儴鍒嗕唬鐮併€傝繖涔熶細绂佺敤閫氳繃鍑芥暟璋冪敤鍙戠敓鐨勯棿鎺ヨ闂殑鎶ュ憡銆?

瀵逛簬鍩轰簬鏍囩鐨?KASAN 妯″紡锛岃绂佺敤璁块棶妫€鏌ワ紝浣跨敤 `kasan_reset_tag()` 鎴?`page_kasan_tag_reset()`銆傛敞鎰忥紝閫氳繃 `page_kasan_tag_reset()` 涓存椂绂佺敤璁块棶妫€鏌ラ渶瑕佸€熷姪 `page_kasan_tag`/`page_kasan_tag_set` 淇濆瓨骞舵仮澶嶆瘡椤电殑 KASAN 鏍囩銆?

#### Tests锛堟祴璇曪級


鏈変竴浜?KASAN 娴嬭瘯鍙敤浜庨獙璇?KASAN 鏄惁宸ヤ綔浠ュ強鑳藉惁妫€娴嬫煇浜涚被鍨嬬殑鍐呭瓨鎹熷潖銆?

鎵€鏈?KASAN 娴嬭瘯閮戒笌 KUnit Test Framework 闆嗘垚锛屽苟鍙€氳繃 `CONFIG_KASAN_KUNIT_TEST` 鍚敤銆傛祴璇曞彲浠ヤ互鍑犵涓嶅悓鐨勬柟寮忚嚜鍔ㄨ繍琛屽拰閮ㄥ垎楠岃瘉锛涜浠ヤ笅璇存槑銆?

姣忎釜 KASAN 娴嬭瘯鍦ㄦ娴嬪埌閿欒鏃舵墦鍗板涓?KASAN 鎶ュ憡涔嬩竴銆傜劧鍚庤娴嬭瘯鎵撳嵃鍏剁紪鍙峰拰鐘舵€併€?

```
        ok 28 - kmalloc_double_kzfree
```

```
        # kmalloc_large_oob_right: ASSERTION FAILED at mm/kasan/kasan_test.c:245
        Expected ptr is not null, but is
        not ok 5 - kmalloc_large_oob_right
```
```
        # kmalloc_double_kzfree: EXPECTATION FAILED at mm/kasan/kasan_test.c:709
        KASAN failure expected in "kfree_sensitive(ptr)", but none occurred
        not ok 28 - kmalloc_double_kzfree
```
```
        ok 1 - kasan
```
```
        not ok 1 - kasan
```

鏈夊嚑绉嶈繍琛?KASAN 娴嬭瘯鐨勬柟寮忋€?

1. 鍙姞杞芥ā鍧楋紙Loadable module锛?

   鍚敤 `CONFIG_KUNIT` 鍚庯紝娴嬭瘯鍙瀯寤轰负鍙姞杞芥ā鍧楋紝骞堕€氳繃鐢?`insmod` 鎴?`modprobe` 鍔犺浇 `kasan_test.ko` 鏉ヨ繍琛屻€?

2. 鍐呭缓锛圔uilt-In锛?

   鍚敤鍐呭缓鐨?`CONFIG_KUNIT` 鍚庯紝娴嬭瘯涔熷彲鍐呭缓銆?

   鍦ㄨ繖绉嶆儏鍐典笅锛屾祴璇曞皢鍦ㄥ惎鍔ㄦ椂浣滀负 late-init 璋冪敤杩愯銆?

3. 浣跨敤 kunit_tool

   鍚敤鍐呭缓鐨?`CONFIG_KUNIT` 鍜?`CONFIG_KASAN_KUNIT_TEST` 鏃讹紝涔熷彲浠ヤ娇鐢?`kunit_tool` 浠ユ洿鏄撹鐨勬柟寮忔煡鐪?KUnit 娴嬭瘯鐨勭粨鏋溿€傝繖涓嶄細鎵撳嵃宸查€氳繃娴嬭瘯鐨?KASAN 鎶ュ憡銆傛湁鍏?`kunit_tool` 鐨勬渶鏂颁俊鎭紝鍙傝 `KUnit 鏂囨。 <https://www.kernel.org/doc/html/latest/dev-tools/kunit/index.html>`_銆?

