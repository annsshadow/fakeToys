
## 鍐呭瓨鍒嗛厤鎬ц兘鍒嗘瀽锛圡EMORY ALLOCATION PROFILING锛?

瀵规墍鏈夊唴瀛樺垎閰嶈繘琛屼綆寮€閿€锛堥€傜敤浜庣敓浜х幆澧冿級鐨勮璐︼紝鎸夋枃浠朵笌琛屽彿璺熻釜銆?
鐢ㄦ硶锛?kconfig 閫夐」锛?- CONFIG_MEM_ALLOC_PROFILING

- CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT

- CONFIG_MEM_ALLOC_PROFILING_DEBUG
  涓洪偅浜涘洜缂哄皯娉ㄨВ鑰屾湭琚璐︾殑鍒嗛厤澧炲姞璀﹀憡

鍚姩鍙傛暟锛?  sysctl.vm.mem_profiling={0|1|never}[,compressed]

  褰撹缃负 "never" 鏃讹紝鍐呭瓨鍒嗛厤鎬ц兘鍒嗘瀽鐨勫紑閿€琚渶灏忓寲锛屽苟涓旀棤娉曞湪杩愯鏃跺惎鐢紙sysctl 鍙樹负鍙锛夈€?  褰?CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT=y 鏃讹紝榛樿鍊间负 "1"銆?  褰?CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT=n 鏃讹紝榛樿鍊间负 "never"銆?  "compressed" 鍙€夊弬鏁颁細灏濊瘯浠ョ揣鍑戞牸寮忓瓨鍌ㄩ〉鏍囪寮曠敤锛岄伩鍏嶄娇鐢ㄩ〉鎵╁睍銆傝繖浼氭敼鍠勬€ц兘涓庡唴瀛樺崰鐢紝
  浣嗗彲鑳戒細鍥犵郴缁熼厤缃€屽け璐ャ€傚鏋滃帇缂╁け璐ワ紝浼氬彂鍑鸿鍛婂苟绂佺敤鍐呭瓨鍒嗛厤鎬ц兘鍒嗘瀽銆?
sysctl锛?  /proc/sys/vm/mem_profiling

  1锛氬惎鐢ㄥ唴瀛樻€ц兘鍒嗘瀽銆?
  0锛氱鐢ㄥ唴瀛樻€ц兘鍒嗘瀽銆?
  榛樿鍊煎彇鍐充簬 CONFIG_MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT銆?
  褰?CONFIG_MEM_ALLOC_PROFILING_DEBUG=y 鏃讹紝姝ゆ帶浠朵负鍙锛屼互閬垮厤鍦ㄦ€ц兘鍒嗘瀽琚鐢ㄦ椂杩涜鐨勫垎閰嶃€?  浠ュ強鍦ㄥ畠琚惎鐢ㄦ椂閲婃斁鎵€浜х敓鐨勮鍛娿€?
杩愯鏃朵俊鎭細
  /proc/allocinfo

```

  root@moria-kvm:~# sort -g /proc/allocinfo|tail|numfmt --to=iec
        2.8M    22648 fs/kernfs/dir.c:615 func:__kernfs_new_node
        3.8M      953 mm/memory.c:4214 func:alloc_anon_folio
        4.0M     1010 drivers/staging/ctagmod/ctagmod.c:20 [ctagmod] func:ctagmod_start
        4.1M        4 net/netfilter/nf_conntrack_core.c:2567 func:nf_ct_alloc_hashtable
        6.0M     1532 mm/filemap.c:1919 func:__filemap_get_folio
        8.8M     2785 kernel/fork.c:307 func:alloc_thread_stack_node
         13M      234 block/blk-mq.c:3421 func:blk_mq_alloc_rqs
         14M     3520 mm/mm_init.c:2530 func:alloc_large_system_hash
         15M     3656 mm/readahead.c:247 func:page_cache_ra_unbounded
         55M     4887 mm/slub.c:2259 func:alloc_slab_page
        122M    31168 mm/page_ext.c:270 func:alloc_page_ext

```
## 宸ヤ綔鍘熺悊


鍐呭瓨鍒嗛厤鎬ц兘鍒嗘瀽寤虹珛鍦ㄤ唬鐮佹爣璁帮紙code tagging锛変箣涓婏紝浠ｇ爜鏍囪鏄竴涓敤浜庡０鏄庨潤鎬佺粨鏋勪綋锛堥€氬父浠ユ煇绉嶆柟寮?鎻忚堪鏂囦欢涓庤鍙凤紝鍥犳绉颁负浠ｇ爜鏍囪锛夈€佸苟鍦ㄨ繍琛屾椂鏌ユ壘骞舵搷浣滃畠浠殑搴撯€斺€斾緥濡傞亶鍘嗗畠浠互鍦?debugfs/procfs 涓墦鍗般€?
瑕佷负涓€娆″垎閰嶈皟鐢ㄥ鍔犺璐︼紝鎴戜滑灏嗗叾鏇挎崲涓轰竴涓畯璋冪敤 alloc_hooks()锛岃瀹忥細
- 澹版槑涓€涓唬鐮佹爣璁?- 鍦ㄥ叾 task_struct 涓殏瀛樹竴涓寚鍚戝畠鐨勬寚閽?- 璋冪敤鐪熸鐨勫垎閰嶅嚱鏁?- 鏈€鍚庯紝灏?task_struct 鐨勫垎閰嶆爣璁版寚閽堟仮澶嶄负鍏跺厛鍓嶇殑鍊笺€?
杩欎娇寰?alloc_hooks() 璋冪敤鍙互宓屽锛屼互鏈€杩戠殑涓€娆＄敓鏁堛€傝繖瀵逛簬 mm/ 浠ｇ爜鍐呴儴銆佷笉灞炰簬澶栧眰鍒嗛厤涓婁笅鏂囥€佸簲褰?鍗曠嫭璁℃暟鐨勫垎閰嶅緢閲嶈锛氫緥濡傦紝slab 瀵硅薄鎵╁睍鍚戦噺锛屾垨鑰呭綋 slab 浠庨〉鍒嗛厤鍣ㄥ垎閰嶉〉鏃躲€?
鍥犳锛屾纭殑鐢ㄦ硶闇€瑕佺‘瀹氬垎閰嶈皟鐢ㄦ爤涓殑鍝釜鍑芥暟搴斿綋琚墦鏍囪銆傛湁璁稿杈呭姪鍑芥暟鏈川涓婂彧鏄皝瑁呬簡渚嬪 kmalloc()
骞跺鍋氫簡涓€鐐瑰伐浣滐紝鐒跺悗鍦ㄥ澶勮璋冪敤锛涙垜浠€氬父甯屾湜璁拌处鍙戠敓鍦ㄨ繖浜涜緟鍔╁嚱鏁扮殑璋冪敤鑰呬腑锛岃€屼笉鏄湪杈呭姪鍑芥暟鑷韩涓€?
瑕佷慨澶嶆煇涓粰瀹氱殑杈呭姪鍑芥暟锛屼緥濡?foo()锛岃鎵ц浠ヤ笅鎿嶄綔锛?- 灏嗗叾鍒嗛厤璋冪敤鍒囨崲涓?_noprof() 鐗堟湰锛屼緥濡?kmalloc_noprof()

- 灏嗗叾閲嶅懡鍚嶄负 foo_noprof()

- 瀹氫箟涓€涓?foo() 鐨勫畯鐗堟湰锛屽涓嬫墍绀猴細

  #define foo(...) alloc_hooks(foo_noprof(__VA_ARGS__))

涔熷彲浠ュ湪浣犺嚜宸辩殑鏁版嵁缁撴瀯涓殏瀛樹竴涓寚鍚戝垎閰嶆爣璁扮殑鎸囬拡銆?
褰撲綘姝ｅ湪瀹炵幇涓€涓€滀唬琛ㄢ€濆叾浠栨煇浜涗唬鐮佽繘琛屽垎閰嶇殑閫氱敤鏁版嵁缁撴瀯鏃垛€斺€斾緥濡?rhashtable 浠ｇ爜鈥斺€斿氨杩欐牱鍋氥€傝繖鏍凤紝
鎴戜滑灏变笉蹇呭湪 /proc/allocinfo 涓湅鍒?rhashtable.c 鐨勪竴澶ц锛岃€屾槸鍙互鎸?rhashtable 绫诲瀷鎷嗗垎瀹冦€?
涓烘锛?- 鍍忓叾浠栦换浣曞垎閰嶅嚱鏁颁竴鏍凤紝鎸傛帴浣犵殑鏁版嵁缁撴瀯鐨?init 鍑芥暟銆?
- 鍦ㄤ綘鐨?init 鍑芥暟鍐呴儴锛屼娇鐢ㄤ究鎹峰畯 alloc_tag_record() 鍦ㄤ綘鐨勬暟鎹粨鏋勪腑璁板綍鍒嗛厤鏍囪銆?
- 鐒跺悗锛屽浣犵殑鍒嗛厤浣跨敤浠ヤ笅褰㈠紡锛?  alloc_hooks_tag(ht->your_saved_tag, kmalloc_noprof(...))
