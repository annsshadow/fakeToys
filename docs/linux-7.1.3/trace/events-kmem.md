## Subsystem Trace Points: kmem


kmem 璺熻釜绯荤粺鎹曡幏鍐呮牳涓笌瀵硅薄鍜岄〉闈㈠垎閰嶇浉鍏崇殑浜嬩欢銆傚ぇ鑷村彲浠ュ垎涓轰簲涓富瑕佺被鍒€?
  - 鏈煡绫诲瀷鐨勫皬瀵硅薄鐨?Slab 鍒嗛厤锛坘malloc锛?  - 宸茬煡绫诲瀷鐨勫皬瀵硅薄鐨?Slab 鍒嗛厤
  - 椤甸潰鍒嗛厤
  - Per-CPU 鍒嗛厤鍣ㄦ椿鍔?  - 澶栭儴纰庣墖

鏈枃妗ｆ弿杩颁簡姣忎釜璺熻釜鐐规槸浠€涔堬紝浠ュ強瀹冧滑涓轰綍鍙兘鏈夌敤銆?
## 1. Slab allocation of small objects of unknown type

```

  kmalloc		call_site=%lx ptr=%p bytes_req=%zu bytes_alloc=%zu gfp_flags=%s
  kmalloc_node	call_site=%lx ptr=%p bytes_req=%zu bytes_alloc=%zu gfp_flags=%s node=%d
  kfree		call_site=%lx ptr=%p

```
杩欎簺浜嬩欢鐨勯珮搴︽椿璺冨彲鑳借〃鏄庢湁蹇呰浣跨敤涓€涓壒瀹氱殑缂撳瓨锛坈ache锛夛紝鐗瑰埆鏄綋 kmalloc slab 椤电敱浜庡垎閰嶆ā寮忚€屽嚭鐜颁弗閲嶅唴閮ㄧ鐗囨椂銆傞€氳繃灏?kmalloc 涓?kfree 鍏宠仈璧锋潵锛屾湁鍙兘璇嗗埆鍑哄唴瀛樻硠婕忎互鍙婂垎閰嶅彂鐢熺殑浣嶇疆銆?

## 2. Slab allocation of small objects of known type

```

  kmem_cache_alloc	call_site=%lx ptr=%p bytes_req=%zu bytes_alloc=%zu gfp_flags=%s
  kmem_cache_alloc_node	call_site=%lx ptr=%p bytes_req=%zu bytes_alloc=%zu gfp_flags=%s node=%d
  kmem_cache_free		call_site=%lx ptr=%p

```
杩欎簺浜嬩欢鍦ㄤ娇鐢ㄦ柟寮忎笂涓?kmalloc 鐩稿叧浜嬩欢绫讳技锛屽彧鏄洿瀹规槗灏嗕簨浠跺畾浣嶅埌鐗瑰畾鐨勭紦瀛樸€傚湪鎾板啓鏈枃鏃讹紝灏氭棤娉曡幏鍙栨鍦ㄤ粠鍝釜 slab 鍒嗛厤鐨勪俊鎭紝浣?call_site 閫氬父鍙互鐢ㄦ潵鎺ㄦ柇璇ヤ俊鎭€?
## 3. Page allocation

```

  mm_page_alloc		  page=%p pfn=%lu order=%d migratetype=%d gfp_flags=%s
  mm_page_alloc_zone_locked page=%p pfn=%lu order=%u migratetype=%d cpu=%d percpu_refill=%d
  mm_page_free		  page=%p pfn=%lu order=%d
  mm_page_free_batched	  page=%p pfn=%lu order=%d cold=%d

```
杩欏洓涓簨浠跺鐞嗛〉闈㈢殑鍒嗛厤涓庨噴鏀俱€俶m_page_alloc 鏄〉闈㈠垎閰嶅櫒娲诲姩鐨勪竴涓畝鍗曟寚绀哄櫒銆傞〉闈㈠彲鑳戒粠 per-CPU 鍒嗛厤鍣紙楂樻€ц兘锛夋垨浼欎即锛坆uddy锛夊垎閰嶅櫒鍒嗛厤銆?
濡傛灉椤甸潰鐩存帴浠庝紮浼村垎閰嶅櫒鍒嗛厤锛屽垯浼氳Е鍙?mm_page_alloc_zone_locked 浜嬩欢銆傝浜嬩欢寰堥噸瑕侊紝鍥犱负澶ч噺鐨勬椿鍔ㄦ剰鍛崇潃 zone->lock 涓婄殑娲诲姩寰堥珮銆傝幏鍙栬閿佷細閫氳繃绂佺敤涓柇銆佸湪 CPU 涔嬮棿寮勮剰缂撳瓨琛屼互鍙婂澶氫釜 CPU 涓茶鍖栬€屾崯瀹虫€ц兘銆?
褰撹皟鐢ㄨ€呯洿鎺ラ噴鏀句竴涓〉闈㈡椂锛屽彧浼氳Е鍙?mm_page_free 浜嬩欢銆傝繖閲屽ぇ閲忕殑娲诲姩鍙兘琛ㄦ槑璋冪敤鑰呭簲褰撴壒澶勭悊瀹冧滑鐨勬椿鍔ㄣ€?
褰撻〉闈㈣鎵归噺閲婃斁鏃讹紝杩樹細瑙﹀彂 mm_page_free_batched銆傚ぇ鑷磋€岃█锛岄〉闈細鎴愭壒鍦颁粠 LRU 閿佷笂鍙栦笅锛屽苟閫氳繃涓€涓〉鍒楄〃鎵归噺閲婃斁銆傝繖閲屽ぇ閲忕殑娲诲姩鍙兘琛ㄦ槑绯荤粺姝ｅ浜庡唴瀛樺帇鍔涗笅锛屼篃鍙兘琛ㄦ槑 lruvec->lru_lock 涓婂瓨鍦ㄤ簤鐢ㄣ€?
## 4. Per-CPU Allocator Activity

```

  mm_page_alloc_zone_locked	page=%p pfn=%lu order=%u migratetype=%d cpu=%d percpu_refill=%d
  mm_page_pcpu_drain		page=%p pfn=%lu order=%d cpu=%d migratetype=%d

```
鍦ㄩ〉闈㈠垎閰嶅櫒鍓嶉潰鏄竴涓?per-cpu 椤甸潰鍒嗛厤鍣ㄣ€傚畠浠呯敤浜?order-0 椤甸潰锛屽彲浠ュ噺灏?zone->lock 涓婄殑浜夌敤锛屽苟鍑忓皯鍦?struct page 涓婄殑鍐欏叆閲忋€?
褰?per-CPU 鍒楄〃涓虹┖鎴栧垎閰嶄簡閿欒绫诲瀷鐨勯〉闈㈡椂锛屼細鑾峰彇涓€娆?zone->lock 骞堕噸鏂板～鍏?per-CPU 鍒楄〃銆傚姣忎釜鍒嗛厤鐨勯〉闈㈤兘浼氳Е鍙?mm_page_alloc_zone_locked 浜嬩欢锛岃浜嬩欢浼氭寚绀哄畠鏄惁鐢ㄤ簬 percpu_refill銆?
褰?per-CPU 鍒楄〃杩囨弧鏃讹紝浼氶噴鏀句竴瀹氭暟閲忕殑椤甸潰锛屾瘡涓〉闈㈤兘浼氳Е鍙戜竴涓?mm_page_pcpu_drain 浜嬩欢銆?
杩欎簺浜嬩欢鐨勪釜浣撴€ц川鏄负浜嗚兘澶熷湪鍒嗛厤鍜岄噴鏀句箣闂磋窡韪〉闈€傝繛缁彂鐢熺殑涓€鎵?drain 鎴?refill 鎰忓懗鐫€鑾峰彇浜嗕竴娆?zone->lock銆傚ぇ閲忕殑 per-CPU refill 鍜?drain 鍙兘鎰忓懗鐫€ CPU 涔嬮棿鐨勮礋杞戒笉鍧囪　锛屽嵆杩囧鐨勫伐浣滈泦涓湪涓€涓湴鏂广€傚畠涔熷彲鑳借〃鏄?per-CPU 鍒楄〃搴斿綋鏇村ぇ鐨勫昂瀵搞€傛渶鍚庯紝澶ч噺鍦ㄤ竴涓?CPU 涓?refill 鑰屽湪鍙︿竴涓?CPU 涓?drain锛屽彲鑳芥槸瀵艰嚧澶ч噺鍥?CPU 涔嬮棿鍐欏叆鑰屼骇鐢熺殑缂撳瓨琛屽脊璺筹紙cache line bounce锛夌殑涓€涓洜绱狅紝鍊煎緱璋冩煡鏄惁鍙互閫氳繃鏌愮绠楁硶鍙樻洿璁╅〉闈㈠湪鍚屼竴涓?CPU 涓婂垎閰嶅拰閲婃斁銆?
## 5. External Fragmentation

```

  mm_page_alloc_extfrag		page=%p pfn=%lu alloc_order=%d fallback_order=%d pageblock_order=%d alloc_migratetype=%d fallback_migratetype=%d fragmenting=%d change_ownership=%d

```
澶栭儴纰庣墖浼氬奖鍝嶉珮闃跺垎閰嶆槸鍚︿細鎴愬姛銆傚浜庢煇浜涚被鍨嬬殑纭欢锛岃繖寰堥噸瑕侊紝涓嶈繃鍦ㄥ彲鑳界殑鎯呭喌涓嬩細灏介噺閬垮厤銆傚鏋滅郴缁熸鍦ㄤ娇鐢ㄥぇ椤碉紙huge page锛夛紝骞朵笖闇€瑕佸湪绯荤粺鐢熷懡鍛ㄦ湡鍐呰兘澶熻皟鏁存睜鐨勫ぇ灏忥紝閭ｄ箞杩欎釜鍊煎緢閲嶈銆?
璇ヤ簨浠剁殑澶ч噺鍑虹幇鎰忓懗鐫€鍐呭瓨姝ｅ湪纰庣墖鍖栵紝楂橀樁鍒嗛厤灏嗗湪鏈潵鐨勬煇涓椂鍒诲紑濮嬪け璐ャ€傚噺灏戣浜嬩欢鍙戠敓鐨勫叾涓竴绉嶆柟娉曟槸锛屾寜 3**pageblock_size**nr_online_nodes 鐨勫閲忓澶?min_free_kbytes锛屽叾涓?pageblock_size 閫氬父鏄粯璁ゅぇ椤靛ぇ灏忋€?