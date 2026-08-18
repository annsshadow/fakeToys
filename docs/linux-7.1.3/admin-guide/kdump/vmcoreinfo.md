## VMCOREINFO


## 瀹冩槸浠€涔堬紵

VMCOREINFO 鏄竴涓壒娈婄殑 ELF note 娈点€傚畠鍖呭惈鏉ヨ嚜鍐呮牳鐨勫绉嶄俊鎭紝渚嬪缁撴瀯澶у皬銆侀〉澶у皬銆?绗﹀彿鍊笺€佸瓧娈靛亸绉荤瓑銆傝繖浜涙暟鎹鎵撳寘杩涗竴涓?ELF note 娈碉紝骞惰 crash銆乵akedumpfile 绛?鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鍒嗘瀽鍐呮牳鐨勫唴瀛樺竷灞€銆?
## 閫氱敤鍙橀噺


### init_uts_ns.name.release


Linux 鍐呮牳鐨勭増鏈彿銆傜敤浜庢壘鍒版瀯寤鸿鍐呮牳鎵€瀵瑰簲鐨勬簮浠ｇ爜銆備緥濡傦紝crash 鐢ㄥ畠鏉ユ煡鎵惧搴旂殑
vmlinux锛屼互渚垮鐞?vmcore銆?
### PAGE_SIZE


椤电殑澶у皬銆傚畠鏄唴瀛樼鐞嗚鏂芥墍浣跨敤鐨勬渶灏忔暟鎹崟浣嶃€傚叾澶у皬閫氬父涓?4096 瀛楄妭锛屼笖椤垫寜 4096
瀛楄妭瀵归綈銆傜敤浜庤绠楅〉鍦板潃銆?
### init_uts_ns


UTS 鍛藉悕绌洪棿锛岀敤浜庨殧绂荤郴缁熶腑涓?uname(2) 绯荤粺璋冪敤鐩稿叧鐨勪袱涓壒瀹氬厓绱犮€傚畠浠ョ敤浜庡瓨鍌?uname(2) 绯荤粺璋冪敤鎵€杩斿洖淇℃伅鐨勬暟鎹粨鏋勫懡鍚嶃€?
鐢ㄦ埛绌洪棿宸ュ叿鍙互浠庝腑鑾峰彇鍐呮牳鍚嶇О銆佷富鏈哄悕銆佸唴鏍稿彂甯冨彿銆佸唴鏍哥増鏈€佹灦鏋勫悕鍜?OS 绫诲瀷銆?
### (uts_namespace, name)


name 鎴愬憳鐨勫亸绉婚噺銆侰rash Utility 涓?Makedumpfile 鎹鑾峰彇 init_uts_ns.name 鐨勮捣濮嬪湴鍧€銆?
### node_online_map


鏁扮粍 node_states[N_ONLINE]锛岃〃绀虹郴缁熶腑鍦ㄧ嚎鑺傜偣鐨勯泦鍚堬紝姣忎釜鑺傜偣鍙峰搴斾竴涓瘮鐗逛綅銆?鐢ㄤ簬璺熻釜鍝簺鑺傜偣鍦ㄧ郴缁熶腑涓斿浜庡湪绾跨姸鎬併€?
### swapper_pg_dir


鍐呮牳鐨勫叏灞€椤电洰褰曟寚閽堛€傜敤浜庡皢铏氭嫙鍦板潃杞崲涓虹墿鐞嗗湴鍧€銆?
### _stext


瀹氫箟 text 娈电殑璧峰浣嶇疆銆傞€氬父锛宊stext 琛ㄧず鍐呮牳鐨勮捣濮嬪湴鍧€銆傜敤浜庡皢鏉ヨ嚜鍐呮牳鐩存帴鏄犲皠鐨?铏氭嫙鍦板潃杞崲涓虹墿鐞嗗湴鍧€銆?
### VMALLOC_START


瀛樺偍 vmalloc 鍖哄煙鐨勫熀鍦板潃銆俶akedumpfile 闇€瑕佽幏鍙栬鍊硷紝鍥犱负瀹冨 vmalloc 杞崲鏄繀瑕佺殑銆?
### mem_map


鐗╃悊鍦板潃閫氳繃灏嗗叾浣滀负 mem_map 鏁扮粍鐨勭储寮曟潵杞崲涓?struct page銆傚皢鐗╃悊鍦板潃鍙崇Щ
PAGE_SHIFT 浣嶅嵆鍙皢鍏惰浆鎹负椤靛抚鍙凤紝涔熷氨鏄 mem_map 鏁扮粍鐨勭储寮曘€?
鐢ㄤ簬灏嗗湴鍧€鏄犲皠鍒板搴旂殑 struct page銆?
### contig_page_data


Makedumpfile 浠庤绗﹀彿鑾峰彇 pglist_data 缁撴瀯锛岃缁撴瀯鐢ㄤ簬鎻忚堪鍐呭瓨甯冨眬銆?
鐢ㄦ埛绌洪棿宸ュ叿鍒╃敤瀹冩潵鍦ㄨ浆鍌ㄥ唴瀛樻椂鎺掗櫎绌洪棽椤点€?
### mem_section|(mem_section, NR_SECTION_ROOTS)|(mem_section, section_mem_map)


mem_section 鏁扮粍鐨勫湴鍧€銆佸叾闀垮害銆佺粨鏋勫ぇ灏忥紝浠ュ強 section_mem_map 鐨勫亸绉婚噺銆?
瀹冨瓨鍦ㄤ簬绋€鐤忓唴瀛樻槧灏勬ā鍨嬩腑锛屽苟涓斾笌 mem_map 鍙橀噺鏈変簺绫讳技锛屼簩鑰呴兘鐢ㄤ簬杞崲鍦板潃銆?
### MAX_PHYSMEM_BITS


瀹氫箟鎵€鏀寔鐨勬渶澶х墿鐞嗗湴鍧€绌洪棿鍐呭瓨銆?
### page


page 缁撴瀯鐨勫ぇ灏忋€俿truct page 鏄竴涓噸瑕佺殑鏁版嵁缁撴瀯锛岃骞挎硾鐢ㄤ簬璁＄畻杩炵画鍐呭瓨銆?
### pglist_data


pglist_data 缁撴瀯鐨勫ぇ灏忋€傝鍊肩敤浜庢鏌?pglist_data 缁撴瀯鏄惁鏈夋晥銆傚畠涔熺敤浜庢鏌ュ唴瀛樼被鍨嬨€?
### zone


zone 缁撴瀯鐨勫ぇ灏忋€傝鍊肩敤浜庢鏌ユ槸鍚﹀凡鎵惧埌 zone 缁撴瀯銆傚畠涔熺敤浜庢帓闄ょ┖闂查〉銆?
### free_area


free_area 缁撴瀯鐨勫ぇ灏忋€傚畠鎸囩ず free_area 缁撴瀯鏄惁鏈夋晥銆傚湪鎺掗櫎绌洪棽椤垫椂寰堟湁鐢ㄣ€?
### list_head


list_head 缁撴瀯鐨勫ぇ灏忋€傜敤浜庡湪鍚庨獙鍒嗘瀽浼氳瘽涓亶鍘嗛摼琛ㄣ€?
### nodemask_t


nodemask_t 绫诲瀷鐨勫ぇ灏忋€傜敤浜庤绠楀湪绾胯妭鐐圭殑鏁伴噺銆?
### (page, flags|_refcount|mapping|lru|_mapcount|private|compound_order|compound_info)


鐢ㄦ埛绌洪棿宸ュ叿鍩轰簬杩欎簺鍙橀噺鐨勫亸绉婚噺鏉ヨ绠楀畠浠殑鍊笺€傝繖浜涘彉閲忓湪鎺掗櫎涓嶅繀瑕佺殑椤垫椂浣跨敤銆?
### (pglist_data, node_zones|nr_zones|node_mem_map|node_start_pfn|node_spanned_pages|node_id)


鍦?NUMA 鏈哄櫒涓婏紝姣忎釜 NUMA 鑺傜偣閮芥湁涓€涓?pg_data_t 鏉ユ弿杩板叾鍐呭瓨甯冨眬銆傚湪 UMA 鏈哄櫒涓婏紝
鍙湁涓€涓?pglist_data 鐢ㄤ簬鎻忚堪鏁翠釜鍐呭瓨銆?
杩欎簺鍊肩敤浜庢鏌ュ唴瀛樼被鍨嬶紝骞惰绠楀唴瀛樻槧灏勭殑铏氭嫙鍦板潃銆?
### (zone, free_area|vm_stat|spanned_pages)


姣忎釜鑺傜偣琚垝鍒嗕负鑻ュ共涓О涓?zone 鐨勫潡锛屽畠浠〃绀哄唴瀛樹腑鐨勮寖鍥淬€備竴涓?zone 鐢?zone 缁撴瀯鏉ユ弿杩般€?
鐢ㄦ埛绌洪棿宸ュ叿鍩轰簬杩欎簺鍙橀噺鐨勫亸绉婚噺鏉ヨ绠楁墍闇€鐨勫€笺€?
### (free_area, free_list)


free_list 鎴愬憳鐨勫亸绉婚噺銆傝鍊肩敤浜庤绠楃┖闂查〉鐨勬暟閲忋€?
姣忎釜 zone 閮芥湁涓€涓悕涓?free_area[NR_PAGE_ORDERS] 鐨?free_area 缁撴瀯鏁扮粍銆?free_list 琛ㄧず绌洪棽椤靛潡鐨勯摼琛ㄣ€?
### (list_head, next|prev)


list_head 鍚勬垚鍛樼殑鍋忕Щ閲忋€俵ist_head 鐢ㄤ簬瀹氫箟寰幆閾捐〃銆傜敤鎴风┖闂村伐鍏烽渶瑕佸畠浠互渚块亶鍘嗛摼琛ㄣ€?
### (vmap_area, va_start|list)


vmap_area 鍚勬垚鍛樼殑鍋忕Щ閲忋€傚畠浠惡甯?vmalloc 鐗规湁鐨勪俊鎭€侻akedumpfile 鎹鑾峰彇 vmalloc
鍖哄煙鐨勮捣濮嬪湴鍧€銆?
### (zone.free_area, NR_PAGE_ORDERS)


绌洪棽鍖哄煙鎻忚堪绗︺€傜敤鎴风┖闂村伐鍏蜂娇鐢ㄨ鍊兼潵閬嶅巻 free_area 鑼冨洿銆侼R_PAGE_ORDERS 鐢?zone
浼欎即鍒嗛厤鍣ㄤ娇鐢ㄣ€?
### prb


鎸囧悜 printk 鐜舰缂撳啿鍖猴紙struct printk_ringbuffer锛夌殑鎸囬拡銆傛牴鎹牳蹇冭浆鍌ㄥ彂鐢熺殑鏃舵満锛?瀹冨彲鑳藉凡缁忔寚鍚戦潤鎬佸惎鍔ㄧ幆褰㈢紦鍐插尯锛屼篃鍙兘鎸囧悜鍔ㄦ€佸垎閰嶇殑鐜舰缂撳啿鍖恒€?鐢辩敤鎴风┖闂村伐鍏风敤浜庤鍙栧綋鍓嶆椿璺冪殑鍐呮牳鏃ュ織缂撳啿鍖恒€?
### printk_rb_static


鎸囧悜闈欐€佸惎鍔?printk 鐜舰缂撳啿鍖虹殑鎸囬拡銆傚鏋?@prb 鐨勫€间笉鍚岋紝杩欏浜庢煡鐪嬪垵濮嬪惎鍔ㄦ秷鎭緢鏈夌敤锛?閭ｄ簺娑堟伅鍙兘宸插湪鍔ㄦ€佸垎閰嶇殑鐜舰缂撳啿鍖轰腑琚鐩栥€?
### clear_seq


涓婁竴娆?clear 鍛戒护涔嬪悗鐨?printk() 璁板綍搴忓彿銆傚畠琛ㄧず涓婁竴娆?SYSLOG_ACTION_CLEAR
锛堜緥濡傜敱 'dmesg -c' 鍙戝嚭鐨勶級涔嬪悗鐨勭涓€鏉¤褰曘€傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬杞偍 dmesg 鏃ュ織鐨勪竴涓瓙闆嗐€?
### printk_ringbuffer


printk_ringbuffer 缁撴瀯鐨勫ぇ灏忋€傝缁撴瀯鍖呭惈璁块棶鍐呮牳鏃ュ織缂撳啿鍖哄悇涓粍鎴愰儴鍒嗘墍闇€鐨勫叏閮ㄤ俊鎭€?
### (printk_ringbuffer, desc_ring|text_data_ring|dict_data_ring|fail)


printk 鐜舰缂撳啿鍖哄悇缁勬垚閮ㄥ垎鐨勫亸绉婚噺銆傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鍦ㄤ笉瑕佹眰澹版槑璇ョ粨鏋勭殑鎯呭喌涓嬫煡鐪?鍐呮牳鏃ュ織缂撳啿鍖恒€?
### prb_desc_ring


prb_desc_ring 缁撴瀯鐨勫ぇ灏忋€傝缁撴瀯鍖呭惈鍏充簬涓€缁勮褰曟弿杩扮鐨勪俊鎭€?
### (prb_desc_ring, count_bits|descs|head_id|tail_id)


鎻忚堪涓€缁勮褰曟弿杩扮鐨勫悇涓瓧娈电殑鍋忕Щ閲忋€傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鍦ㄤ笉瑕佹眰澹版槑璇ョ粨鏋勭殑鎯呭喌涓?閬嶅巻杩欎簺鎻忚堪绗︺€?
### prb_desc


prb_desc 缁撴瀯鐨勫ぇ灏忋€傝缁撴瀯鍖呭惈鍏充簬鍗曚釜璁板綍鎻忚堪绗︾殑淇℃伅銆?
### (prb_desc, info|state_var|text_blk_lpos|dict_blk_lpos)


鎻忚堪涓€涓褰曟弿杩扮鐨勫悇涓瓧娈电殑鍋忕Щ閲忋€傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鍦ㄤ笉瑕佹眰澹版槑璇ョ粨鏋勭殑鎯呭喌涓?璇诲彇杩欎簺鎻忚堪绗︺€?
### prb_data_blk_lpos


prb_data_blk_lpos 缁撴瀯鐨勫ぇ灏忋€傝缁撴瀯鍖呭惈鍏充簬鏂囨湰鎴栧瓧鍏告暟鎹紙鏁版嵁鍧楋級鍦ㄧ浉搴旀暟鎹幆褰?缂撳啿鍖轰腑浣嶇疆鐨勪俊鎭€?
### (prb_data_blk_lpos, begin|next)


鎻忚堪涓€涓暟鎹潡浣嶇疆鐨勫悇涓瓧娈电殑鍋忕Щ閲忋€傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鍦ㄤ笉瑕佹眰澹版槑璇ョ粨鏋勭殑鎯呭喌涓?瀹氫綅鏁版嵁鍧椼€?
### printk_info


printk_info 缁撴瀯鐨勫ぇ灏忋€傝缁撴瀯鍖呭惈涓€鏉¤褰曠殑鍏ㄩ儴鍏冩暟鎹€?
### (printk_info, seq|ts_nsec|text_len|dict_len|caller_id)


鎻愪緵涓€鏉¤褰曞厓鏁版嵁鐨勫悇涓瓧娈电殑鍋忕Щ閲忋€傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鍦ㄤ笉瑕佹眰澹版槑璇ョ粨鏋勭殑鎯呭喌涓?璇诲彇璇ヤ俊鎭€?
### prb_data_ring


prb_data_ring 缁撴瀯鐨勫ぇ灏忋€傝缁撴瀯鍖呭惈鍏充簬涓€缁勬暟鎹潡鐨勪俊鎭€?
### (prb_data_ring, size_bits|data|head_lpos|tail_lpos)


鎻忚堪涓€缁勬暟鎹潡鐨勫悇涓瓧娈电殑鍋忕Щ閲忋€傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鍦ㄤ笉瑕佹眰澹版槑璇ョ粨鏋勭殑鎯呭喌涓?璁块棶杩欎簺鏁版嵁鍧椼€?
### atomic_long_t


atomic_long_t 缁撴瀯鐨勫ぇ灏忋€傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鑳藉澶嶅埗鏁翠釜缁撴瀯锛岃€屼笉璁哄叾鏋舵瀯鐩稿叧鐨勫疄鐜般€?
### (atomic_long_t, counter)


atomic_long_t 鍙橀噺鐨勯暱鏁村瀷鍊肩殑鍋忕Щ閲忋€傜敱鐢ㄦ埛绌洪棿宸ュ叿鐢ㄤ簬鍦ㄤ笉瑕佹眰鏋舵瀯鐩稿叧澹版槑鐨勬儏鍐典笅
璁块棶璇ラ暱鏁村瀷鍊笺€?
### (free_area.free_list, MIGRATE_TYPES)


椤电殑杩佺Щ绫诲瀷鏁伴噺銆俧ree_list 鐢辫鏁扮粍鎻忚堪銆傜敱宸ュ叿鐢ㄤ簬璁＄畻绌洪棽椤电殑鏁伴噺銆?
### NR_FREE_PAGES


鍦?linux-2.6.21 鎴栨洿楂樼増鏈腑锛岀┖闂查〉鏁伴噺浣嶄簬 vm_stat[NR_FREE_PAGES]銆傜敤浜庤幏鍙栫┖闂查〉
鏁伴噺銆?
### PG_lru|PG_private|PG_swapcache|PG_swapbacked|PG_hwpoison|PG_head_mask


椤靛睘鎬с€傝繖浜涙爣蹇楃敤浜庤繃婊ゅ悇绉嶈浆鍌ㄦ椂涓嶉渶瑕佺殑椤点€?
### PAGE_SLAB_MAPCOUNT_VALUE|PAGE_BUDDY_MAPCOUNT_VALUE|PAGE_OFFLINE_MAPCOUNT_VALUE|PAGE_HUGETLB_MAPCOUNT_VALUE|PAGE_UNACCEPTED_MAPCOUNT_VALUE


鏇村椤靛睘鎬с€傝繖浜涙爣蹇楃敤浜庤繃婊ゅ悇绉嶈浆鍌ㄦ椂涓嶉渶瑕佺殑椤点€?

## x86_64


### phys_base


鐢ㄤ簬灏嗗鍑虹殑鍐呮牳绗﹀彿鐨勮櫄鎷熷湴鍧€杞崲涓哄叾瀵瑰簲鐨勭墿鐞嗗湴鍧€銆?
### init_top_pgt


鐢ㄤ簬閬嶅巻鏁翠釜椤佃〃骞跺皢铏氭嫙鍦板潃杞崲涓虹墿鐞嗗湴鍧€銆俰nit_top_pgt 涓?swapper_pg_dir 鏈変簺绫讳技锛?浣嗕粎鍦?x86_64 涓娇鐢ㄣ€?
### pgtable_l5_enabled


鐢ㄦ埛绌洪棿宸ュ叿闇€瑕佷簡瑙ｅ穿婧冨唴鏍告槸鍚﹀浜?5 绾у垎椤垫ā寮忋€?
### node_data


杩欐槸涓€涓?struct pglist_data 鏁扮粍锛屽瓨鍌ㄦ墍鏈?NUMA 鑺傜偣鐨勪俊鎭€侻akedumpfile 浠庝腑鑾峰彇
pglist_data 缁撴瀯銆?
### (node_data, MAX_NUMNODES)


绯荤粺涓妭鐐圭殑鏈€澶ф暟閲忋€?
### KERNELOFFSET


鍐呮牳闅忔満鍖栧亸绉婚噺銆傜敤浜庤绠楅〉鍋忕Щ銆傚鏋?KASLR 琚鐢紝鍒欒鍊间负闆躲€?
### KERNEL_IMAGE_SIZE


鐩墠鏈 Makedumpfile 浣跨敤銆傜敤浜庣敱 Crash 璁＄畻妯″潡铏氭嫙鍦板潃銆?
### sme_mask


AMD 鐗规湁锛屾敮鎸?SME锛氬畠琛ㄧず瀹夊叏鍐呭瓨鍔犲瘑鎺╃爜銆侻akedumpfile 宸ュ叿闇€瑕佷簡瑙ｅ穿婧冨唴鏍告槸鍚?琚姞瀵嗐€傚鏋滅涓€涓唴鏍稿惎鐢ㄤ簡 SME锛屽穿婧冨唴鏍哥殑椤佃〃椤癸紙pgd/pud/pmd/pte锛変腑鍖呭惈璇ュ唴瀛?鍔犲瘑鎺╃爜銆傝繖鐢ㄤ簬鍘婚櫎 SME 鎺╃爜骞惰幏鍙栫湡瀹炵殑鐗╃悊鍦板潃銆?
鐩墠锛宻me_mask 瀛樺偍 C 浣嶇殑浣嶇疆銆傚鏋滈渶瑕侊紝鍙互灏嗛澶栫殑 SME 鐩稿叧淇℃伅鏀惧叆璇ュ彉閲忎腑銆?
```

  [ misc	        ][ enc bit  ][ other misc SME info       ]
  0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_..._0000
  63   59   55   51   47   43   39   35   31   27   ... 3

```
## x86_32


### X86_PAE


琛ㄧず鏄惁鍚敤浜嗙墿鐞嗗湴鍧€鎵╁睍銆傚畠浼氬甫鏉ユ洿楂樼殑椤佃〃鏌ユ壘寮€閿€锛屽苟涓旀瘡涓繘绋嬩篃娑堣€楁洿澶氱殑椤佃〃
绌洪棿銆傜敤浜庡湪灏嗚櫄鎷熷湴鍧€杞崲涓虹墿鐞嗗湴鍧€鏃舵鏌ュ穿婧冨唴鏍告槸鍚﹀惎鐢ㄤ簡 PAE銆?
## ARM64


### VA_BITS


铏氭嫙鍦板潃鐨勬渶澶т綅鏁般€傜敤浜庤绠楄櫄鎷熷唴瀛樿寖鍥淬€?
### kimage_voffset


鍐呮牳铏氭嫙鏄犲皠涓庣墿鐞嗘槧灏勪箣闂寸殑鍋忕Щ閲忋€傜敤浜庡皢铏氭嫙鍦板潃杞崲涓虹墿鐞嗗湴鍧€銆?
### PHYS_OFFSET


琛ㄧず鍐呭瓨璧峰浣嶇疆鐨勭墿鐞嗗湴鍧€銆備笌 kimage_voffset 绫讳技锛屽悗鑰呯敤浜庡皢铏氭嫙鍦板潃杞崲涓虹墿鐞嗗湴鍧€銆?
### KERNELOFFSET


鍐呮牳闅忔満鍖栧亸绉婚噺銆傜敤浜庤绠楅〉鍋忕Щ銆傚鏋?KASLR 琚鐢紝鍒欒鍊间负闆躲€?
### KERNELPACMASK


鐢ㄤ簬浠庡唴鏍歌櫄鎷熷湴鍧€涓彁鍙栨寚閽堣璇佺爜锛圥ointer Authentication Code锛夌殑鎺╃爜銆?
### TCR_EL1.T1SZ


琛ㄧず TTBR1_EL1 鎵€瀵诲潃鐨勫唴瀛樺尯鍩熺殑澶у皬鍋忕Щ銆傝鍖哄煙澶у皬涓?2^(64-T1SZ) 瀛楄妭銆?
TTBR1_EL1 鏄敱 ARMv8-A 鏋舵瀯瑙勫畾鐨勮〃鍩哄湴鍧€瀵勫瓨鍣紝鐢ㄤ簬鏌ユ壘杈冮珮 VA 鑼冨洿涓櫄鎷熷湴鍧€鐨?椤佃〃锛堟洿澶氱粏鑺傝鍙傞槄 ARMv8 ARM 鏂囨。锛夈€?
### MODULES_VADDR|MODULES_END|VMALLOC_START|VMALLOC_END|VMEMMAP_START|VMEMMAP_END


鐢ㄤ簬鑾峰彇姝ｇ‘鐨勮寖鍥达細
	MODULES_VADDR ~ MODULES_END-1 : 鍐呮牳妯″潡绌洪棿銆?	VMALLOC_START ~ VMALLOC_END-1 : vmalloc() / ioremap() 绌洪棿銆?	VMEMMAP_START ~ VMEMMAP_END-1 : vmemmap 鍖哄煙锛岀敤浜?struct page 鏁扮粍銆?
## arm


### ARM_LPAE


瀹冭〃绀哄穿婧冨唴鏍告槸鍚︽敮鎸佸ぇ鐗╃悊鍦板潃鎵╁睍銆傜敤浜庡皢铏氭嫙鍦板潃杞崲涓虹墿鐞嗗湴鍧€銆?
## s390


### lowcore_ptr


涓€涓寚鍚戞瘡涓?CPU 鐨?lowcore 鐨勬寚閽堟暟缁勩€傜敤浜庢墦鍗?psw 浠ュ強鎵€鏈夊瘎瀛樺櫒鐨勪俊鎭€?
### high_memory


鐢ㄤ簬浠?high_memory 绗﹀彿鑾峰彇 vmalloc_start 鍦板潃銆?
### (lowcore_ptr, NR_CPUS)


CPU 鐨勬渶澶ф暟閲忋€?
## powerpc



### node_data|(node_data, MAX_NUMNODES)


鍙傝涓婃枃銆?
### contig_page_data


鍙傝涓婃枃銆?
### vmemmap_list


vmemmap_list 缁存姢鏁翠釜 vmemmap 鐗╃悊鏄犲皠銆傜敤浜庤幏鍙?vmemmap 鍒楄〃璁℃暟浠ュ強宸插～鍏呯殑 vmemmap
鍖哄煙淇℃伅銆傚鏋?vmemmap 鍦板潃杞崲淇℃伅瀛樺偍鍦ㄥ穿婧冨唴鏍镐腑锛屽垯鐢ㄤ簬杞崲 vmemmap 鍐呮牳铏氭嫙鍦板潃銆?
### mmu_vmemmap_psize


椤电殑澶у皬銆傜敤浜庡皢铏氭嫙鍦板潃杞崲涓虹墿鐞嗗湴鍧€銆?
### mmu_psize_defs


椤靛ぇ灏忓畾涔夛紝鍗?4k銆?4k 鎴?16M銆?
鐢ㄤ簬杩涜 vtop 杞崲銆?
### vmemmap_backing|(vmemmap_backing, list)|(vmemmap_backing, phys)|(vmemmap_backing, virt_addr)


vmemmap 铏氭嫙鍦板潃绌洪棿绠＄悊娌℃湁浼犵粺鐨勯〉琛ㄦ潵璺熻釜鍝簺铏氭嫙 struct page 鐢辩墿鐞嗘槧灏勬敮鎾戙€?铏氭嫙鍒扮墿鐞嗙殑鏄犲皠浠ヤ竴绉嶇畝鍗曠殑閾捐〃鏍煎紡杩涜璺熻釜銆?
鐢ㄦ埛绌洪棿宸ュ叿鍦ㄨ绠?vmemmap 鍖哄煙璁℃暟鏃堕渶瑕佷簡瑙?list銆乸hys 鍜?virt_addr 鐨勫亸绉婚噺銆?
### mmu_psize_def|(mmu_psize_def, shift)


struct mmu_psize_def 鐨勫ぇ灏忎互鍙?mmu_psize_def 鎴愬憳鐨勫亸绉婚噺銆?
鐢ㄤ簬 vtop 杞崲銆?
## sh


### node_data|(node_data, MAX_NUMNODES)


鍙傝涓婃枃銆?
### X2TLB


琛ㄧず宕╂簝鍐呮牳鏄惁鍚敤浜?SH 鎵╁睍妯″紡銆?
## RISCV64


### VA_BITS


铏氭嫙鍦板潃鐨勬渶澶т綅鏁般€傜敤浜庤绠楄櫄鎷熷唴瀛樿寖鍥淬€?
### PAGE_OFFSET


琛ㄧず鐩存帴鏄犲皠 RAM 鍖哄煙鐨勮櫄鎷熷唴鏍歌捣濮嬪湴鍧€銆?
### phys_ram_base


琛ㄧず鐗╃悊 RAM 鐨勮捣濮嬪湴鍧€銆?
### MODULES_VADDR|MODULES_END|VMALLOC_START|VMALLOC_END|VMEMMAP_START|VMEMMAP_END|KERNEL_LINK_ADDR


鐢ㄤ簬鑾峰彇姝ｇ‘鐨勮寖鍥达細

  - MODULES_VADDR ~ MODULES_END : 鍐呮牳妯″潡绌洪棿銆?  - VMALLOC_START ~ VMALLOC_END : vmalloc() / ioremap() 绌洪棿銆?  - VMEMMAP_START ~ VMEMMAP_END : vmemmap 绌洪棿锛岀敤浜?struct page 鏁扮粍銆?  - KERNEL_LINK_ADDR : 鍐呮牳閾炬帴涓?BPF 鐨勮捣濮嬪湴鍧€

### va_kernel_pa_offset


琛ㄧず鍐呮牳铏氭嫙鏄犲皠涓庣墿鐞嗘槧灏勪箣闂寸殑鍋忕Щ閲忋€傜敤浜庡皢铏氭嫙鍦板潃杞崲涓虹墿鐞嗗湴鍧€銆?