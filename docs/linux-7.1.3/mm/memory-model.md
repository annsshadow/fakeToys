
## 鐗╃悊鍐呭瓨妯″瀷


绯荤粺涓殑鐗╃悊鍐呭瓨鍙互閫氳繃涓嶅悓鏂瑰紡瀵诲潃銆傛渶绠€鍗曠殑鎯呭喌鏄墿鐞嗗唴瀛樹粠鍦板潃 0 寮€濮嬶紝
骞惰法瓒婁竴娈佃繛缁殑鍦板潃鑼冨洿鐩村埌鏈€澶у湴鍧€銆傜劧鑰岋紝杩欐鑼冨洿涓彲鑳藉寘鍚?CPU 鏃犳硶璁块棶
鐨勫皬绌烘礊銆傛澶栵紝涔熷彲鑳藉湪瀹屽叏涓嶅悓鐨勫湴鍧€涓婂瓨鍦ㄨ嫢骞叉杩炵画鐨勮寖鍥淬€傝€屼笖锛屽埆蹇樹簡
NUMA锛屽湪 NUMA 涓笉鍚岀殑鍐呭瓨搴撴寕杞藉湪涓嶅悓鐨?CPU 涓娿€?
Linux 浣跨敤涓ょ鍐呭瓨妯″瀷涔嬩竴鏉ユ娊璞¤繖绉嶅鏍锋€э細FLATMEM 鍜?SPARSEMEM銆傛瘡涓灦鏋?瀹氫箟瀹冩敮鎸佸摢浜涘唴瀛樻ā鍨嬨€侀粯璁ゅ唴瀛樻ā鍨嬫槸浠€涔堬紝浠ュ強鏄惁鍙兘鎵嬪姩瑕嗙洊璇ラ粯璁ゅ€笺€?
鎵€鏈夊唴瀛樻ā鍨嬮兘浣跨敤鎺掑垪鍦ㄤ竴涓垨澶氫釜鏁扮粍涓殑 struct page 鏉ヨ窡韪墿鐞嗛〉甯х殑鐘舵€併€?
鏃犺閫夋嫨鍝鍐呭瓨妯″瀷锛岀墿鐞嗛〉甯у彿锛圥FN锛変笌瀵瑰簲鐨?`struct page` 涔嬮棿閮藉瓨鍦?涓€涓€鏄犲皠銆?
姣忎釜鍐呭瓨妯″瀷閮藉畾涔変簡 `pfn_to_page` 鍜?`page_to_pfn` 杈呭姪鍑芥暟锛岀敤浜庡湪 PFN 涓?`struct page` 涔嬮棿鐩镐簰杞崲銆?
## FLATMEM


鏈€绠€鍗曠殑鍐呭瓨妯″瀷鏄?FLATMEM銆傝妯″瀷閫傜敤浜庡叿鏈夎繛缁垨杩戜箮杩炵画鐨勭墿鐞嗗唴瀛樼殑闈?NUMA
绯荤粺銆?
鍦?FLATMEM 鍐呭瓨妯″瀷涓紝鏈変竴涓叏灞€鐨?`mem_map` 鏁扮粍鏉ユ槧灏勬暣涓墿鐞嗗唴瀛樸€傚浜?澶у鏁版灦鏋勶紝绌烘礊鍦?`mem_map` 鏁扮粍涓篃鏈夊搴旂殑琛ㄩ」銆傚搴斾簬绌烘礊鐨?`struct page`
瀵硅薄浠庢湭琚畬鍏ㄥ垵濮嬪寲銆?
涓轰簡鍒嗛厤 `mem_map` 鏁扮粍锛屾灦鏋勭浉鍏崇殑 setup 浠ｇ爜搴斿綋璋冪敤 `free_area_init` 鍑芥暟銆?鐒惰€岋紝鍦ㄨ皟鐢?`memblock_free_all` 灏嗗叏閮ㄥ唴瀛樹氦缁欓〉鍒嗛厤鍣ㄤ箣鍓嶏紝璇ユ槧灏勬暟缁勬槸
涓嶅彲鐢ㄧ殑銆?
鏋舵瀯鍙互閲婃斁 `mem_map` 鏁扮粍涓湭瑕嗙洊瀹為檯鐗╃悊椤电殑閮ㄥ垎銆傚湪杩欑鎯呭喌涓嬶紝鏋舵瀯鐩稿叧鐨?`pfn_valid` 瀹炵幇搴斿綋鎶?`mem_map` 涓殑绌烘礊鑰冭檻鍦ㄥ唴銆?
鍦?FLATMEM 涓嬶紝PFN 涓?`struct page` 涔嬮棿鐨勮浆鎹㈠緢鐩存帴锛歚PFN - ARCH_PFN_OFFSET`
鏄?`mem_map` 鏁扮粍鐨勭储寮曘€?
`ARCH_PFN_OFFSET` 瀹氫箟浜嗙墿鐞嗗唴瀛樿捣濮嬪湴鍧€涓嶄负 0 鐨勭郴缁熺殑绗竴涓〉甯у彿銆?
## SPARSEMEM


SPARSEMEM 鏄?Linux 涓€氱敤鎬ф渶寮哄唴瀛樻ā鍨嬶紝涔熸槸鍞竴鏀寔鑻ュ共楂樼骇鐗规€х殑鍐呭瓨妯″瀷锛?渚嬪鐗╃悊鍐呭瓨鐨勭儹鎻掓嫈涓庣儹绉婚櫎銆侀潪鏄撳け鎬у唴瀛樿澶囩殑鏇夸唬鍐呭瓨鏄犲皠锛屼互鍙婂ぇ鍨嬬郴缁熺殑
鍐呭瓨鏄犲皠寤惰繜鍒濆鍖栥€?
SPARSEMEM 妯″瀷灏嗙墿鐞嗗唴瀛樺憟鐜颁负涓€缁?section 鐨勯泦鍚堛€備竴涓?section 鐢?struct
mem_section 琛ㄧず锛屽叾涓寘鍚?`section_mem_map`锛屼粠閫昏緫涓婅锛屽畠鏄寚鍚?struct page
鏁扮粍鐨勬寚閽堛€傜劧鑰岋紝瀹冭繕瀛樺偍浜嗕竴浜涘叾瀹冪殑鈥滈瓟娉曗€濅俊鎭紝浠ヨ緟鍔?section 鐨勭鐞嗐€俿ection
鐨勫ぇ灏忓拰 section 鐨勬渶澶ф暟閲忕敱姣忎釜鏀寔 SPARSEMEM 鐨勬灦鏋勫畾涔夌殑 `SECTION_SIZE_BITS`
鍜?`MAX_PHYSMEM_BITS` 甯搁噺鎸囧畾銆傝櫧鐒?`MAX_PHYSMEM_BITS` 鏄竴涓灦鏋勬敮鎸佺殑鐗╃悊
鍦板潃鐨勫疄闄呭搴︼紝浣?`SECTION_SIZE_BITS` 鏄竴涓换鎰忓€笺€?
section 鐨勬渶澶ф暟閲忚涓?`NR_MEM_SECTIONS`锛屽畾涔変负


   NR\_MEM\_SECTIONS = 2 ^ {(MAX\_PHYSMEM\_BITS - SECTION\_SIZE\_BITS)}

`mem_section` 瀵硅薄鎺掑垪鍦ㄤ竴涓О涓?`mem_sections` 鐨勪簩缁存暟缁勪腑銆傝鏁扮粍鐨勫ぇ灏忓拰
浣嶇疆鍙栧喅浜?`CONFIG_SPARSEMEM_EXTREME` 浠ュ強 section 鐨勬渶澶у彲鑳芥暟閲忥細

- 褰?`CONFIG_SPARSEMEM_EXTREME` 琚鐢ㄦ椂锛宍mem_sections` 鏁扮粍鏄潤鎬佺殑锛屽苟涓?  鍏锋湁 `NR_MEM_SECTIONS` 琛屻€傛瘡琛屼繚瀛樹竴涓?`mem_section` 瀵硅薄銆?- 褰?`CONFIG_SPARSEMEM_EXTREME` 琚惎鐢ㄦ椂锛宍mem_sections` 鏁扮粍鏄姩鎬佸垎閰嶇殑銆?  姣忚鍖呭惈 PAGE_SIZE 澶у皬鐨?`mem_section` 瀵硅薄锛岃鏁扮粡杩囪绠椾互瀹圭撼鎵€鏈夊唴瀛?  section銆?
鍦?SPARSEMEM 涓嬶紝灏?PFN 杞崲涓哄搴旂殑 `struct page` 鏈変袱绉嶅彲鑳界殑鏂瑰紡鈥斺€斺€渃lassic
sparse鈥濆拰鈥渟parse vmemmap鈥濄€傞€夋嫨鏄湪鏋勫缓鏃跺仛鍑虹殑锛岀敱 `CONFIG_SPARSEMEM_VMEMMAP`
鐨勫€煎喅瀹氥€?
classic sparse 灏嗛〉鐨?section 缂栧彿缂栫爜鍦?page->flags 涓紝骞朵娇鐢?PFN 鐨勯珮浣嶆潵
璁块棶鏄犲皠璇ラ〉甯х殑 section銆傚湪涓€涓?section 鍐呴儴锛孭FN 鏄〉鏁扮粍鐨勭储寮曘€?
sparse vmemmap 浣跨敤铏氭嫙鏄犲皠鐨勫唴瀛樻槧灏勬潵浼樺寲 pfn_to_page 鍜?page_to_pfn 鎿嶄綔銆?鏈変竴涓叏灞€鐨?`struct page *vmemmap` 鎸囬拡锛屾寚鍚戜竴涓櫄鎷熻繛缁殑 `struct page`
瀵硅薄鏁扮粍銆侾FN 鏄鏁扮粍鐨勭储寮曪紝`struct page` 鐩稿 `vmemmap` 鐨勫亸绉婚噺灏辨槸璇ラ〉鐨?PFN銆?
瑕佷娇鐢?vmemmap锛屾灦鏋勫繀椤讳繚鐣欎竴娈佃櫄鎷熷湴鍧€鑼冨洿锛岀敤浜庢槧灏勫寘鍚唴瀛樻槧灏勭殑鐗╃悊椤碉紝
骞剁‘淇?`vmemmap` 鎸囧悜璇ヨ寖鍥淬€傛澶栵紝鏋舵瀯搴斿疄鐜?`vmemmap_populate` 鏂规硶锛屼互
鍒嗛厤鐗╃悊鍐呭瓨骞朵负铏氭嫙鍐呭瓨鏄犲皠鍒涘缓椤佃〃銆傚鏋滄灦鏋勫 vmemmap 鏄犲皠娌℃湁浠讳綍鐗规畩瑕佹眰锛?瀹冨彲浠ヤ娇鐢ㄩ€氱敤鍐呭瓨绠＄悊鎻愪緵鐨勯粯璁?`vmemmap_populate_basepages`銆?
铏氭嫙鏄犲皠鐨勫唴瀛樻槧灏勫厑璁稿皢鎸佷箙鍐呭瓨璁惧鐨?`struct page` 瀵硅薄瀛樺偍鍦ㄩ偅浜涜澶囦笂棰勫垎閰嶇殑
瀛樺偍涓€傝瀛樺偍鐢?struct vmem_altmap 琛ㄧず锛屾渶缁堥€氳繃涓€闀夸覆鍑芥暟璋冪敤浼犻€掔粰
vmemmap_populate()銆倂memmap_populate() 鐨勫疄鐜板彲浠ヤ娇鐢?`vmem_altmap` 浠ュ強
`vmemmap_alloc_block_buf` 杈呭姪鍑芥暟鍦ㄦ寔涔呭唴瀛樿澶囦笂鍒嗛厤鍐呭瓨鏄犲皠銆?
## ZONE_DEVICE


`ZONE_DEVICE` 鏈哄埗寤虹珛鍦?`SPARSEMEM_VMEMMAP` 涔嬩笂锛屼负璁惧椹卞姩璇嗗埆鐨勭墿鐞嗗湴鍧€
鑼冨洿鎻愪緵 `struct page` 鐨?`mem_map` 鏈嶅姟銆俙ZONE_DEVICE` 鐨勨€滆澶団€濇柟闈笌浠ヤ笅
浜嬪疄鐩稿叧锛氳繖浜涘湴鍧€鑼冨洿鐨勯〉瀵硅薄姘歌繙涓嶄細琚爣璁颁负鍦ㄧ嚎锛屽苟涓斿繀椤诲璇ヨ澶囷紙鑰屼笉浠呬粎鏄?椤碉級鎸佹湁寮曠敤锛屾墠鑳藉皢鍐呭瓨淇濇寔涓鸿鍥哄畾浣跨敤鐘舵€併€俙ZONE_DEVICE` 閫氳繃
`devm_memremap_pages` 鎵ц浜嗚冻澶熺殑鍐呭瓨鐑彃鎷旓紝浠ュ紑鍚粰瀹?PFN 鑼冨洿鐨?`pfn_to_page`銆乣page_to_pfn` 鍜?`get_user_pages` 鏈嶅姟銆傜敱浜庨〉寮曠敤璁℃暟姘歌繙涓嶄細
闄嶅埌 1 浠ヤ笅锛岃椤垫案杩滀笉浼氳浣滀负绌洪棽鍐呭瓨璺熻釜锛屽苟涓旇椤电殑 `struct list_head lru`
绌洪棿琚噸鏂板埄鐢紝鐢ㄤ簬鍙嶅悜寮曠敤鏄犲皠璇ュ唴瀛樼殑涓绘満璁惧/椹卞姩銆?
铏界劧 `SPARSEMEM` 灏嗗唴瀛樺憟鐜颁负涓€缁?section锛堝彲閫夊湴鏀堕泦涓哄唴瀛樺潡锛夛紝浣?`ZONE_DEVICE` 鐨勭敤鎴烽渶瑕佹洿缁嗙殑绮掑害鏉ュ～鍏?`mem_map`銆傞壌浜?`ZONE_DEVICE` 鍐呭瓨
姘歌繙涓嶄細琚爣璁颁负鍦ㄧ嚎锛屽洜姝ゅ畠闅忓悗姘歌繙涓嶄細閫氳繃 sysfs 鍐呭瓨鐑彃鎷?API 鍦ㄥ唴瀛樺潡杈圭晫
鏆撮湶鍏跺唴瀛樿寖鍥淬€傝瀹炵幇渚濊禆杩欑缂轰箯鐢ㄦ埛绌洪棿 API 绾︽潫鐨勭壒鎬э紝鍏佽鍚?`arch_add_memory`锛堝唴瀛樼儹鎻掓嫈鐨勪笂鍗婇儴鍒嗭級鎸囧畾瀛?section 澶у皬鐨勫唴瀛樿寖鍥淬€傚瓙
section 鏀寔鍏佽浠?2MB 浣滀负 `devm_memremap_pages` 鐨勮法鏋舵瀯閫氱敤瀵归綈绮掑害銆?
`ZONE_DEVICE` 鐨勭敤鎴锋湁锛?
- pmem锛氬皢骞冲彴鎸佷箙鍐呭瓨鏄犲皠涓洪€氳繃 DAX 鏄犲皠鐢ㄤ綔鐩存帴 I/O 鐩爣銆?
- hmm锛氱敤 `->page_fault()` 鍜?`->folio_free()` 浜嬩欢鍥炶皟鎵╁睍 `ZONE_DEVICE`锛?  浠ュ厑璁歌澶囬┍鍔ㄥ崗璋冧笌璁惧鍐呭瓨锛堥€氬父鏄?GPU 鍐呭瓨锛夌浉鍏崇殑鍐呭瓨绠＄悊浜嬩欢銆傚弬瑙?  Documentation/mm/hmm.rst銆?
- p2pdma锛氬垱寤?`struct page` 瀵硅薄锛屼互鍏佽 PCI/-E 鎷撴墤涓殑瀵圭瓑璁惧鍦ㄥ郊姝や箣闂?  鍗忚皟鐩存帴 DMA 鎿嶄綔锛屽嵆缁曡繃涓绘満鍐呭瓨銆?