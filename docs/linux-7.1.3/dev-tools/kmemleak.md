## 鍐呮牳鍐呭瓨娉勬紡妫€娴嬪櫒


Kmemleak 鎻愪緵浜嗕竴绉嶆娴嬪彲鑳界殑鍐呮牳鍐呭瓨娉勬紡鐨勬柟娉曪紝鍏舵柟寮忕被浼间簬
` tracing garbage collector <https://en.wikipedia.org/wiki/Tracing_garbage_collection>`_
锛堣窡韪紡鍨冨溇鏀堕泦鍣級锛屽尯鍒湪浜庡鍎匡紙orphan锛夊璞′笉浼氳閲婃斁锛岃€屽彧鏄€氳繃
/sys/kernel/debug/kmemleak 鎶ュ憡銆俈algrind 宸ュ叿锛坄memcheck --leak-check`锛変篃浣跨敤
绫讳技鐨勬柟娉曟潵妫€娴嬬敤鎴风┖闂村簲鐢ㄧ▼搴忎腑鐨勫唴瀛樻硠婕忋€?
### 鐢ㄦ硶


蹇呴』鍦ㄢ€淜ernel hacking鈥濅腑鍚敤 CONFIG_DEBUG_KMEMLEAK銆備竴涓唴鏍哥嚎绋嬫瘡闅?10 鍒嗛挓
锛堥粯璁わ級鎵弿涓€娆″唴瀛橈紝骞舵墦鍗版壘鍒扮殑鏂版湭寮曠敤瀵硅薄鐨勬暟閲忋€傚鏋?`debugfs` 灏氭湭
```
  # mount -t debugfs nodev /sys/kernel/debug/
```
```
  # cat /sys/kernel/debug/kmemleak
```
```
  # echo scan > /sys/kernel/debug/kmemleak
```
```
  # echo clear > /sys/kernel/debug/kmemleak
```
鎸傝浇锛屽垯鍐嶆璇诲彇 `/sys/kernel/debug/kmemleak` 鏃跺氨浼氭樉绀哄嚭鏂扮殑娉勬紡銆?
璇锋敞鎰忥紝瀛ゅ効瀵硅薄鏄寜瀹冧滑琚垎閰嶇殑椤哄簭鍒楀嚭鐨勶紝鍒楄〃涓紑澶寸殑涓€涓璞″彲鑳戒細瀵艰嚧鍚庣画
鍏朵粬瀵硅薄涔熻鎶ュ憡涓哄鍎裤€?
鍐呭瓨鎵弿鍙傛暟鍙互鍦ㄨ繍琛屾椂閫氳繃鍐欏叆 `/sys/kernel/debug/kmemleak` 鏂囦欢鏉ヤ慨鏀广€傛敮鎸佺殑
鍙傛暟濡備笅锛?
- off
    绂佺敤 kmemleak锛堜笉鍙€嗭級
- stack=on
    鍚敤浠诲姟鏍堟壂鎻忥紙榛樿锛?- stack=off
    绂佺敤浠诲姟鏍堟壂鎻?- scan=on
    鍚姩鑷姩鍐呭瓨鎵弿绾跨▼锛堥粯璁わ級
- scan=off
    鍋滄鑷姩鍐呭瓨鎵弿绾跨▼
- scan=<secs>
    璁剧疆鑷姩鍐呭瓨鎵弿鍛ㄦ湡锛堢锛?    锛堥粯璁?600锛岃涓?0 鍒欏仠姝㈣嚜鍔ㄦ壂鎻忥級
- scan
    瑙﹀彂涓€娆″唴瀛樻壂鎻?- clear
    娓呴櫎褰撳墠鍐呭瓨娉勬紡瀚岀枒瀵硅薄鍒楄〃锛屽仛娉曟槸灏嗘墍鏈夊綋鍓嶅凡鎶ュ憡鐨勬湭寮曠敤瀵硅薄鏍囪涓?    鐏拌壊锛屾垨鑰呭湪 kmemleak 宸茶绂佺敤鏃堕噴鏀炬墍鏈?kmemleak 瀵硅薄銆?- dump=<addr>
    杞偍鍦?<addr> 澶勬壘鍒扮殑瀵硅薄鐨勪俊鎭?
Kmemleak 涔熷彲浠ラ€氳繃鍦ㄥ唴鏍稿懡浠よ浼犲叆 `kmemleak=off` 鍦ㄥ惎鍔ㄦ椂绂佺敤銆?
鍦ㄥ唴鏍稿垎閰嶆垨閲婃斁鍐呭瓨鐨勫姩浣滃彲鑳藉彂鐢熷湪 kmemleak 鍒濆鍖栦箣鍓嶏紝杩欎簺鍔ㄤ綔琚瓨鍌ㄥ湪涓€涓棭鏈?鏃ュ織缂撳啿鍖轰腑銆傝缂撳啿鍖虹殑澶у皬閫氳繃 CONFIG_DEBUG_KMEMLEAK_MEM_POOL_SIZE 閫夐」閰嶇疆銆?
濡傛灉鍚敤浜?CONFIG_DEBUG_KMEMLEAK_DEFAULT_OFF锛屽垯 kmemleak 榛樿鏄鐢ㄧ殑銆傚湪鍐呮牳
鍛戒护琛屼紶鍏?`kmemleak=on` 鍙惎鐢ㄨ鍔熻兘銆?
濡傛灉浣犻亣鍒扮被浼?鈥淓rror while writing to stdout鈥?鎴?鈥渨rite_loop: Invalid argument鈥?鐨勯敊璇紝璇风‘淇?kmemleak 宸茶姝ｇ‘鍚敤銆?
### 鍩烘湰绠楁硶


閫氳繃 `kmalloc`銆乣vmalloc`銆乣kmem_cache_alloc` 鍙婂叾鍚岀被鍑芥暟杩涜鐨勫唴瀛樺垎閰嶄細琚窡韪紝
鎸囬拡杩炲悓澶у皬銆佹爤鍥炴函绛夐檮鍔犱俊鎭竴璧峰瓨鍌ㄥ湪涓€妫?rbtree 涓€傜浉搴旂殑閲婃斁鍑芥暟璋冪敤浼氳
璺熻釜锛屽苟涓旀寚閽堜細浠?kmemleak 鐨勬暟鎹粨鏋勪腑绉婚櫎銆?
濡傛灉涓€涓凡鍒嗛厤鐨勫唴瀛樺潡锛屽湪鎵弿鍐呭瓨锛堝寘鎷繚瀛樼殑瀵勫瓨鍣級鏃讹紝鎵句笉鍒版寚鍚戝叾璧峰鍦板潃鎴?鍧楀唴浠讳綍浣嶇疆鐨勬寚閽堬紝鍒欒鍐呭瓨鍧楄瑙嗕负瀛ゅ効銆傝繖鎰忓懗鐫€鍐呮牳鍙兘娌℃湁閫斿緞鎶婅鍐呭瓨鍧楃殑
鍦板潃浼犻€掔粰閲婃斁鍑芥暟锛屽洜姝よ鍧楄瑙嗕负鍐呭瓨娉勬紡銆?
鎵弿绠楁硶鐨勬楠わ細

  1. 鎶婃墍鏈夊璞℃爣璁颁负鐧借壊锛堝墿浣欑殑鐧借壊瀵硅薄涔嬪悗灏嗚瑙嗕负瀛ゅ効锛?  2. 浠庢暟鎹鍜屾爤寮€濮嬫壂鎻忓唴瀛橈紝鎶婅鍒扮殑鍊间笌 rbtree 涓瓨鍌ㄧ殑鍦板潃杩涜姣斿銆傚鏋滄壘鍒?     涓€涓寚鍚戠櫧鑹插璞＄殑鎸囬拡锛屽垯鎶婅瀵硅薄鍔犲叆鐏拌壊鍒楄〃
  3. 鎵弿鐏拌壊瀵硅薄浠ュ鎵惧尮閰嶇殑鍦板潃锛堟煇浜涚櫧鑹插璞″彲鑳藉彉涓虹伆鑹插苟琚姞鍒扮伆鑹插垪琛ㄦ湯灏撅級锛?     鐩村埌鐏拌壊闆嗗悎澶勭悊瀹屾瘯
  4. 鍓╀綑鐨勭櫧鑹插璞¤瑙嗕负瀛ゅ効锛屽苟閫氳繃 /sys/kernel/debug/kmemleak 鎶ュ憡

涓€浜涘凡鍒嗛厤鐨勫唴瀛樺潡鎶婃寚閽堝瓨鍌ㄥ湪鍐呮牳鐨勫唴閮ㄦ暟鎹粨鏋勪腑锛屽畠浠棤娉曡妫€娴嬩负瀛ゅ効銆備负閬垮厤
杩欎竴鐐癸紝kmemleak 杩樺彲浠ュ瓨鍌ㄩ渶瑕佽鎵惧埌鐨勩€佹寚鍚戝潡鍦板潃鑼冨洿鍐呯殑鍦板潃鐨勫€肩殑鏁伴噺锛屼互渚?璇ュ潡涓嶈瑙嗕负娉勬紡銆備竴涓緥瀛愭槸 __vmalloc()銆?
### 鐢?kmemleak 娴嬭瘯鐗瑰畾浠ｇ爜娈?

鍦ㄥ垵濮嬪惎鍔ㄥ悗锛屼綘鐨?/sys/kernel/debug/kmemleak 杈撳嚭椤靛彲鑳戒細鐩稿綋闀裤€傚鏋滀綘鍦ㄥ紑鍙戞椂
鏈夐潪甯稿缂洪櫡鐨勪唬鐮侊紝涔熷彲鑳藉嚭鐜拌繖绉嶆儏鍐点€備负浜嗗簲瀵硅繖浜涙儏鍐碉紝浣犲彲浠ヤ娇鐢?'clear' 鍛戒护
浠?/sys/kernel/debug/kmemleak 鐨勮緭鍑轰腑娓呴櫎鎵€鏈夊凡鎶ュ憡鐨勬湭寮曠敤瀵硅薄銆傚湪 'clear' 涔嬪悗
鍙戝嚭涓€涓?'scan'锛屼綘灏卞彲浠ユ壘鍒版柊鐨勬湭寮曠敤瀵硅薄锛涜繖搴旀湁鍔╀簬娴嬭瘯鐗瑰畾鐨勪唬鐮佹銆?
```
  # echo clear > /sys/kernel/debug/kmemleak
  ... 娴嬭瘯浣犵殑鍐呮牳鎴栨ā鍧?...
  # echo scan > /sys/kernel/debug/kmemleak
```
```
  # cat /sys/kernel/debug/kmemleak
```

### 閲婃斁 kmemleak 鍐呴儴瀵硅薄


涓轰簡鍦?kmemleak 琚敤鎴风鐢ㄦ垨鍥犺嚧鍛介敊璇鐢ㄤ箣鍚庯紝浠嶈兘璁块棶涔嬪墠鍙戠幇鐨勫唴瀛樻硠婕忥紝kmemleak
鐨勫唴閮ㄥ璞″湪 kmemleak 琚鐢ㄦ椂涓嶄細琚噴鏀撅紝鑰岃繖浜涘璞″彲鑳戒細鍗犳嵁鐗╃悊鍐呭瓨鐨勫緢澶т竴閮ㄥ垎銆?
```
  # echo clear > /sys/kernel/debug/kmemleak
```

### Kmemleak API


鍑芥暟鍘熷瀷璇峰弬闃?include/linux/kmemleak.h 澶存枃浠躲€?
- `kmemleak_init`		 - 鍒濆鍖?kmemleak
- `kmemleak_alloc`		 - 閫氱煡涓€娆″唴瀛樺潡鍒嗛厤
- `kmemleak_alloc_percpu`	 - 閫氱煡涓€娆?percpu 鍐呭瓨鍧楀垎閰?- `kmemleak_vmalloc`		 - 閫氱煡涓€娆?vmalloc() 鍐呭瓨鍒嗛厤
- `kmemleak_free`		 - 閫氱煡涓€娆″唴瀛樺潡閲婃斁
- `kmemleak_free_part`	 - 閫氱煡涓€娆￠儴鍒嗗唴瀛樺潡閲婃斁
- `kmemleak_free_percpu`	 - 閫氱煡涓€娆?percpu 鍐呭瓨鍧楅噴鏀?- `kmemleak_update_trace`	 - 鏇存柊瀵硅薄鍒嗛厤鏍堝洖婧?- `kmemleak_not_leak`	 - 鎶婁竴涓璞℃爣璁颁负涓嶆槸娉勬紡
- `kmemleak_transient_leak`	 - 鎶婁竴涓璞℃爣璁颁负鏆傛椂鎬ф硠婕?- `kmemleak_ignore`		 - 涓嶆壂鎻忔垨涓嶆妸鏌愪釜瀵硅薄鎶ュ憡涓烘硠婕?- `kmemleak_scan_area`	 - 鍦ㄥ唴瀛樺潡鍐呭鍔犳壂鎻忓尯鍩?- `kmemleak_no_scan`	 - 涓嶆壂鎻忔煇涓唴瀛樺潡
- `kmemleak_erase`		 - 鎿﹂櫎鎸囬拡鍙橀噺涓殑鏃у€?- `kmemleak_alloc_recursive` - 绫讳技 kmemleak_alloc锛屼絾妫€鏌ラ€掑綊鎬?- `kmemleak_free_recursive`	 - 绫讳技 kmemleak_free锛屼絾妫€鏌ラ€掑綊鎬?
浠ヤ笅鍑芥暟浠ョ墿鐞嗗湴鍧€浣滀负瀵硅薄鎸囬拡锛屽苟涓斿彧鍦ㄥ湴鍧€鍏锋湁 lowmem 鏄犲皠鏃舵墠鎵ц鐩稿簲鍔ㄤ綔锛?
- `kmemleak_alloc_phys`
- `kmemleak_free_part_phys`
- `kmemleak_ignore_phys`

### 澶勭悊鍋囬槾鎬?鍋囬槼鎬?

鍋囬槾鎬ф槸鐪熷疄鐨勫唴瀛樻硠婕忥紙瀛ゅ効瀵硅薄锛夛紝浣嗙敱浜庡唴瀛樻壂鎻忔湡闂存壘鍒扮殑鍊兼寚鍚戜簡杩欑被瀵硅薄鑰屾湭琚?kmemleak 鎶ュ憡銆備负浜嗗噺灏戝亣闃存€х殑鏁伴噺锛宬memleak 鎻愪緵浜?kmemleak_ignore銆乲memleak_scan_area銆?kmemleak_no_scan 鍜?kmemleak_erase 鍑芥暟锛堣涓婏級銆備换鍔℃爤涔熶細澧炲姞鍋囬槾鎬х殑鏁伴噺锛屼笖榛樿
涓嶅惎鐢ㄥ瀹冧滑鐨勬壂鎻忋€?
鍋囬槼鎬ф槸琚敊璇湴鎶ュ憡涓哄唴瀛樻硠婕忥紙瀛ゅ効锛夌殑瀵硅薄銆傚浜庡凡鐭ヤ笉鏄硠婕忕殑瀵硅薄锛宬memleak 鎻愪緵浜?kmemleak_not_leak 鍑芥暟銆傚鏋滃凡鐭ヨ鍐呭瓨鍧椾笉鍖呭惈鍏朵粬鎸囬拡锛屼篃鍙互浣跨敤 kmemleak_ignore锛?杩欐牱瀹冨皢涓嶅啀琚壂鎻忋€?
涓€浜涙姤鍛婄殑娉勬紡鍙槸鏆傛椂鎬х殑锛屽湪 SMP 绯荤粺涓婂挨鍏跺姝わ紝鍥犱负鎸囬拡浼氫复鏃跺瓨鏀惧湪 CPU 瀵勫瓨鍣?鎴栨爤涓€侹memleak 瀹氫箟浜?MSECS_MIN_AGE锛堥粯璁や负 1000锛夛紝琛ㄧず涓€涓璞¤鎶ュ憡涓哄唴瀛樻硠婕忔墍
蹇呴』鍏锋湁鐨勬渶灏忓瓨娲绘椂闂淬€?
### 灞€闄愪笌缂虹偣


涓昏鐨勭己鐐规槸鍐呭瓨鍒嗛厤鍜岄噴鏀剧殑鎬ц兘涓嬮檷銆備负浜嗛伩鍏嶅叾浠栦唬浠凤紝鍐呭瓨鎵弿鍙湪璇诲彇
/sys/kernel/debug/kmemleak 鏂囦欢鏃舵墠鎵ц銆傛€讳箣锛岃繖涓伐鍏风敤浜庤皟璇曠洰鐨勶紝鍦ㄨ繖浜涘満鏅笅鎬ц兘
鏈繀鏄渶閲嶈鐨勮姹傘€?
涓轰簡璁╃畻娉曚繚鎸佺畝鍗曪紝kmemleak 鎵弿鎸囧悜涓€涓潡鍦板潃鑼冨洿鍐呬换浣曞湴鍧€鐨勫€笺€傝繖鍙兘瀵艰嚧鍋囬槾鎬?鏁伴噺澧炲姞銆備笉杩囷紝鐪熷疄鐨勫唴瀛樻硠婕忔渶缁堝緢鍙兘浼氭樉鐜板嚭鏉ャ€?
鍋囬槾鎬х殑鍙︿竴涓潵婧愭槸瀛樺偍浜庨潪鎸囬拡鍊间腑鐨勬暟鎹€傚湪鏈潵鐨勭増鏈腑锛宬memleak 鍙互鍙壂鎻忓凡鍒嗛厤
缁撴瀯浣撲腑鐨勬寚閽堟垚鍛樸€傝繖涓€鐗规€у皢瑙ｅ喅涓婇潰鎻忚堪鐨勮澶氬亣闃存€ф儏鍐点€?
璇ュ伐鍏峰彲鑳芥姤鍛婂亣闃虫€с€傝繖浜涙儏鍐靛寘鎷細涓€涓凡鍒嗛厤鍧椾笉闇€瑕佽閲婃斁锛坕nit_call 鍑芥暟涓殑鏌愪簺
鎯呭喌锛夈€佹寚閽堟槸閫氳繃 container_of 瀹忎互澶栫殑鏂规硶璁＄畻寰楀埌鐨勶紝鎴栬€呮寚閽堝瓨鍌ㄥ湪 kmemleak 鏈?鎵弿鐨勪綅缃€?
椤靛垎閰嶅拰 ioremap 涓嶈璺熻釜銆?
### 鐢?kmemleak-test 杩涜娴嬭瘯


瑕佹鏌ヤ綘鏄惁宸插噯澶囧ソ浣跨敤 kmemleak锛屽彲浠ヤ娇鐢?kmemleak-test 妯″潡锛岃繖鏄竴涓細鏁呮剰娉勬紡
鍐呭瓨鐨勬ā鍧椼€傛妸 CONFIG_SAMPLE_KMEMLEAK 璁句负妯″潡锛堝畠涓嶈兘鐢ㄤ綔鍐呭缓锛夛紝骞剁敤 kmemleak 鍚姩
鍐呮牳
```
        # modprobe kmemleak-test
        # echo scan > /sys/kernel/debug/kmemleak
```
璇锋敞鎰忥紝浣犲彲鑳戒笉浼氱珛鍗虫垨鍦ㄧ涓€娆℃壂鎻忔椂灏卞緱鍒扮粨鏋溿€傚綋 kmemleak 寰楀埌缁撴灉鏃讹紝瀹冧細璁板綍
``kmemleak: <count of leaks> new suspected
```
        # cat /sys/kernel/debug/kmemleak
        unreferenced object 0xffff89862ca702e8 (size 32):
          comm "modprobe", pid 2088, jiffies 4294680594 (age 375.486s)
          hex dump (first 32 bytes):
            6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b  kkkkkkkkkkkkkkkk
            6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b 6b a5  kkkkkkkkkkkkkkk.
          backtrace:
            [<00000000e0a73ec7>] 0xffffffffc01d2036
            [<000000000c5d2a46>] do_one_initcall+0x41/0x1df
            [<0000000046db7e0a>] do_init_module+0x55/0x200
            [<00000000542b9814>] load_module+0x203c/0x2480
            [<00000000c2850256>] __do_sys_finit_module+0xba/0xe0
            [<000000006564e7ef>] do_syscall_64+0x43/0x110
            [<000000007c873fa6>] entry_SYSCALL_64_after_hwframe+0x44/0xa9
        ...
```
鐢?`rmmod kmemleak_test` 绉婚櫎璇ユā鍧椾篃搴斾細瑙﹀彂涓€浜?kmemleak 缁撴灉銆?