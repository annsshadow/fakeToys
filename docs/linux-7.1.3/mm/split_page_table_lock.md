## 鎷嗗垎椤佃〃閿侊紙Split page table lock锛?
鏈€鍒濓紝`mm->page_table_lock` 鑷棆閿佷繚鎶?`mm_struct` 鐨勬墍鏈夐〉琛ㄣ€備絾杩欑鏂规硶鐢变簬閿佺珵浜夋縺鐑堬紝瀵艰嚧澶氱嚎绋嬪簲鐢ㄧ殑缂洪〉寮傚父鍙墿灞曟€ц緝宸€備负鏀瑰杽鍙墿灞曟€э紝寮曞叆浜嗘媶鍒嗛〉琛ㄩ攣銆?
閲囩敤鎷嗗垎椤佃〃閿佸悗锛屾瘡涓〃閮芥嫢鏈夌嫭绔嬬殑 per-table 閿佹潵涓茶鍖栧璇ヨ〃鐨勮闂€傜洰鍓嶆垜浠 PTE 鍜?PMD 琛ㄤ娇鐢ㄦ媶鍒嗛攣銆傚鏇撮珮绾у埆琛ㄧ殑璁块棶鐢?`mm->page_table_lock` 淇濇姢銆?
鎻愪緵浜嗕竴缁勭敤浜庨攣瀹?瑙ｉ攣琛ㄤ互鍙婂叾浠栬闂櫒鍑芥暟鐨勮緟鍔╁嚱鏁帮細

 - pte_offset_map_lock()
	鏄犲皠 PTE 骞惰幏鍙?PTE 琛ㄩ攣锛岃繑鍥炴寚鍚?PTE 鐨勬寚閽堝強鍏?PTE 琛ㄩ攣鐨勬寚閽堬紝鑻ユ病鏈?PTE 琛ㄥ垯杩斿洖 NULL锛? - pte_offset_map_ro_nolock()
	鏄犲皠 PTE锛岃繑鍥炴寚鍚?PTE 鐨勬寚閽堝強鍏?PTE 琛ㄩ攣鐨勬寚閽堬紙鏈幏鍙栵級锛岃嫢娌℃湁 PTE 琛ㄥ垯杩斿洖 NULL锛? - pte_offset_map_rw_nolock()
	鏄犲皠 PTE锛岃繑鍥炴寚鍚?PTE 鐨勬寚閽堝強鍏?PTE 琛ㄩ攣鐨勬寚閽堬紙鏈幏鍙栵級浠ュ強鍏?pmd 椤圭殑鍊硷紝鑻ユ病鏈?PTE 琛ㄥ垯杩斿洖 NULL锛? - pte_offset_map()
	鏄犲皠 PTE锛岃繑鍥炴寚鍚?PTE 鐨勬寚閽堬紝鑻ユ病鏈?PTE 琛ㄥ垯杩斿洖 NULL锛? - pte_unmap()
	鍙栨秷鏄犲皠 PTE 琛紱
 - pte_unmap_unlock()
	瑙ｉ攣骞跺彇娑堟槧灏?PTE 琛紱
 - pte_alloc_map_lock()
	蹇呰鏃跺垎閰?PTE 琛ㄥ苟鑾峰彇鍏堕攣锛岃繑鍥炴寚鍚?PTE 鐨勬寚閽堝強鍏堕攣鐨勬寚閽堬紝鑻ュ垎閰嶅け璐ュ垯杩斿洖 NULL锛? - pmd_lock()
	鑾峰彇 PMD 琛ㄩ攣锛岃繑鍥炴寚鍚戝凡鑾峰彇閿佺殑鎸囬拡锛? - pmd_lockptr()
	杩斿洖鎸囧悜 PMD 琛ㄩ攣鐨勬寚閽堬紱

PTE 琛ㄧ殑鎷嗗垎椤佃〃閿佸湪缂栬瘧鏈熷惎鐢紝鏉′欢鏄?`CONFIG_SPLIT_PTLOCK_CPUS`锛堥€氬父涓?4锛夊皬浜庢垨绛変簬 `NR_CPUS`銆傚鏋滄媶鍒嗛攣琚鐢紝鎵€鏈夎〃閮界敱 `mm->page_table_lock` 淇濇姢銆?
PMD 琛ㄧ殑鎷嗗垎椤佃〃閿佸湪 PTE 琛ㄥ惎鐢ㄤ笖鏋舵瀯鏀寔鏃跺惎鐢紙瑙佷笅鏂囷級銆?
## Hugetlb 涓庢媶鍒嗛〉琛ㄩ攣

Hugetlb 鍙互鏀寔澶氱椤靛ぇ灏忋€傛垜浠粎瀵?PMD 绾у埆浣跨敤鎷嗗垎閿侊紝鑰屼笉瀵?PUD 浣跨敤銆?
Hugetlb 涓撶敤鐨勮緟鍔╁嚱鏁帮細

 - huge_pte_lock()
	涓?PMD_SIZE 椤佃幏鍙?pmd 鎷嗗垎閿侊紝鍚﹀垯鑾峰彇 `mm->page_table_lock`锛? - huge_pte_lockptr()
	杩斿洖鎸囧悜琛ㄩ攣鐨勬寚閽堬紱

## 鏋舵瀯瀵规媶鍒嗛〉琛ㄩ攣鐨勬敮鎸?
涓嶉渶瑕佸 PTE 鎷嗗垎椤佃〃閿佸仛鐗规畩鍚敤锛氭墍闇€鐨勫叏閮ㄥ伐浣滅敱 `pagetable_pte_ctor()` 鍜?`pagetable_dtor()` 瀹屾垚锛屽畠浠繀椤诲湪 PTE 琛ㄥ垎閰?閲婃斁鏃惰皟鐢ㄣ€?
璇风‘淇濇灦鏋勬病鏈変娇鐢?slab 鍒嗛厤鍣ㄦ潵鍒嗛厤椤佃〃锛歴lab 浣跨敤 `page->slab_cache` 浣滀负鍏堕〉鐨勫瓧娈点€傝瀛楁涓?`page->ptl` 鍏变韩瀛樺偍绌洪棿銆?
PMD 鎷嗗垎閿佸彧鏈夊湪鎷ユ湁澶氫簬涓ょ骇椤佃〃鏃舵墠鏈夋剰涔夈€?
PMD 鎷嗗垎閿佺殑鍚敤闇€瑕佸湪 PMD 琛ㄥ垎閰嶆椂璋冪敤 `pagetable_pmd_ctor()`锛屽湪閲婃斁鏃惰皟鐢?`pagetable_dtor()`銆?
鍒嗛厤閫氬父鍙戠敓鍦?`pmd_alloc_one()` 涓紝閲婃斁鍦?`pmd_free()` 鍜?`pmd_free_tlb()` 涓紝浣嗚纭繚瑕嗙洊鎵€鏈?PMD 琛ㄥ垎閰?閲婃斁璺緞锛氫緥濡?X86_PAE 浼氬湪 `pgd_alloc()` 涓鍒嗛厤鑻ュ共 PMD銆?
涓€鍒囧氨缁悗锛屽彲浠ヨ缃?`CONFIG_ARCH_ENABLE_SPLIT_PMD_PTLOCK`銆?
娉ㄦ剰锛歚pagetable_pte_ctor()` 鍜?`pagetable_pmd_ctor()` 鍙兘澶辫触鈥斺€斿繀椤诲Ε鍠勫鐞嗐€?
## page->ptl

`page->ptl` 鐢ㄤ簬璁块棶鎷嗗垎椤佃〃閿侊紝鍏朵腑 `page` 鏄寘鍚琛ㄧ殑椤靛搴旂殑 `struct page`銆傚畠涓?`page->private`锛堜互鍙?union 涓殑鍏朵粬鑻ュ共瀛楁锛夊叡浜瓨鍌ㄧ┖闂淬€?
涓洪伩鍏嶅澶?`struct page` 鐨勫昂瀵稿苟鑾峰緱鏈€浣虫€ц兘锛屾垜浠娇鐢ㄤ簡涓€涓妧宸э細

 - 濡傛灉 `spinlock_t` 鑳芥斁鍏?`long`锛屾垜浠皢 `page->ptr` 鐢ㄤ綔鑷棆閿侊紝浠庤€岄伩鍏嶉棿鎺ヨ闂苟鑺傜渷涓€涓紦瀛樿銆? - 濡傛灉 `spinlock_t` 鐨勫ぇ灏忓ぇ浜?`long` 鐨勫ぇ灏忥紝鎴戜滑灏?`page->ptl` 鐢ㄤ綔鎸囧悜 `spinlock_t` 鐨勬寚閽堝苟鍔ㄦ€佸垎閰嶅畠銆傝繖鍏佽鍦ㄥ惎鐢?`DEBUG_SPINLOCK` 鎴?`DEBUG_LOCK_ALLOC` 鏃朵娇鐢ㄦ媶鍒嗛攣锛屼絾闂存帴璁块棶浼氬娑堣€椾竴涓紦瀛樿锛?
`spinlock_t` 鍦?PTE 琛ㄧ殑 `pagetable_pte_ctor()` 涓垎閰嶏紝鍦?PMD 琛ㄧ殑 `pagetable_pmd_ctor()` 涓垎閰嶃€?
璇风粷涓?鐩存帴璁块棶 `page->ptl`鈥斺€旇浣跨敤鐩稿簲鐨勮緟鍔╁嚱鏁般€?