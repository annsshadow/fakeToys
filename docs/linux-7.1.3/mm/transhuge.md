## 閫忔槑澶ч〉锛圱ransparent Hugepage锛夋敮鎸?

鏈枃妗ｆ弿杩颁簡閫忔槑澶ч〉锛圱ransparent Hugepage锛孴HP锛夋敮鎸佺殑璁捐鍘熷垯鍙婂叾涓庡唴瀛樼鐞?绯荤粺鍏朵粬閮ㄥ垎鐨勪氦浜掋€?
## 璁捐鍘熷垯


- 鈥滀紭闆呭洖閫€锛坓raceful fallback锛夆€濓細涓嶄簡瑙ｉ€忔槑澶ч〉鐨?mm 缁勪欢浼氬洖閫€鍒板皢澶ч〉 pmd 鏄犲皠鎷嗗垎涓?pte 琛紝骞跺湪蹇呰鏃舵媶鍒嗕竴涓€忔槑澶ч〉銆傚洜姝よ繖浜涚粍浠?  鍙互缁х画鍦ㄥ父瑙勯〉鎴栧父瑙?pte 鏄犲皠涓婂伐浣溿€?
- 濡傛灉鐢变簬鍐呭瓨纰庣墖瀵艰嚧澶ч〉鍒嗛厤澶辫触锛?  搴斾紭闆呭湴鍒嗛厤甯歌椤碉紝骞跺湪鍚屼竴 vma 涓贩鍚堬紝鑰屾病鏈変换浣曞け璐ユ垨鏄捐憲寤惰繜锛屼笖鐢ㄦ埛绌洪棿涓嶄細瀵熻鍒般€?
- 濡傛灉鏌愪釜浠诲姟閫€鍑哄苟鏈夋洿澶氬ぇ椤靛彉寰楀彲鐢紙鏃犺鏄湪 buddy 涓珛鍗宠幏寰楋紝杩樻槸閫氳繃 VM 鑾峰緱锛夛紝鐢卞父瑙勯〉鏀拺鐨?  瀹㈡埛鏈虹墿鐞嗗唴瀛樺簲鑷姩锛堥€氳繃 khugepaged锛夐噸瀹氫綅鍒板ぇ椤典笂銆?
- 瀹冧笉闇€瑕佸唴瀛橀鐣欙紝杩涜€屽湪鍙兘鏃堕殢鏃朵娇鐢ㄥぇ椤碉紙姝ゅ鍞竴鍙兘鐨勯鐣欐槸 kernelcore=
  浠ラ伩鍏嶄笉鍙Щ鍔ㄩ〉纰庣墖鍖栦簡鎵€鏈夊唴瀛橈紝浣嗘绫昏皟鏁村苟闈為€忔槑澶ч〉鏀寔鎵€鐗规湁锛屽畠鏄€傜敤浜庡唴鏍镐腑鎵€鏈夊姩鎬侀珮闃跺垎閰嶇殑閫氱敤鐗规€э級銆?
## get_user_pages 涓?pin_user_pages


get_user_pages 涓?pin_user_pages 濡傛灉鍦ㄥぇ椤典笂杩愯锛屽皢鐓у父杩斿洖澶撮〉锛坔ead page锛夋垨灏鹃〉锛坱ail page锛夛紙姝ｅ瀹冧滑鍦?hugetlbfs 涓婃墍鍋氱殑閭ｆ牱锛夈€傚ぇ澶氭暟 GUP 鐢ㄦ埛鍙叧蹇冮〉鐨勫疄闄呯墿鐞嗗湴鍧€鍙婂叾涓存椂鍥哄畾锛坧inning锛夛紝浠ヤ究鍦?I/O
瀹屾垚鍚庨噴鏀撅紝鍥犳浠栦滑姘歌繙涓嶄細娉ㄦ剰鍒拌椤垫槸澶ч〉杩欎竴浜嬪疄銆備絾濡傛灉浠讳綍椹卞姩鎵撶畻瀵瑰熬椤电殑椤电粨鏋勫姩鎵嬭剼锛堜緥濡傛鏌?page->mapping 鎴?涓庡ご椤电浉鍏宠€岄潪灏鹃〉鐩稿叧鐨勫叾浠栦綅锛夛紝鍒欏簲鏇存柊涓鸿烦杩囧幓妫€鏌ュご椤点€傝幏鍙栦换浣曞ご/灏鹃〉涓婄殑寮曠敤灏嗛樆姝㈣椤佃浠讳綍浜烘媶鍒嗐€?
   杩欎簺瀵?GUP API 鑰岃█骞堕潪鏂扮殑绾︽潫锛屽畠浠笌閫傜敤浜?hugetlbfs 鐨勭害鏉熺浉鍚岋紝鍥犳浠讳綍鑳藉澶勭悊 hugetlbfs 涓?GUP 鐨勯┍鍔?   涔熻兘鍦ㄩ€忔槑澶ч〉鏀拺鐨勬槧灏勪笂姝ｅ父宸ヤ綔銆?
## 浼橀泤鍥為€€


閬嶅巻椤佃〃浣嗕笉浜嗚В澶ч〉 pmd 鐨勪唬鐮侊紝鍙渶鍦?pmd 鏄?pmd_offset 杩斿洖鐨?pmd 鏃惰皟鐢?split_huge_pmd(vma, pmd, addr)銆備粎閫氳繃 grep 鏌ユ壘 "pmd_offset" 骞跺湪 pmd_offset 杩斿洖 pmd 鍚?缂哄け澶勬坊鍔?split_huge_pmd锛屽氨鑳借交鏉捐浠ｇ爜鍙樺緱閫忔槑澶ч〉鎰熺煡銆傚浜忎簡浼橀泤鍥為€€璁捐锛屽彧闇€涓€琛屾敼鍔紝浣犲氨鑳介伩鍏嶇紪鍐?鏁扮櫨涔冭嚦鏁板崈琛屽鏉備唬鐮佹潵璁╀綘鐨勪唬鐮佸彉寰楀ぇ椤垫劅鐭ャ€?
濡傛灉浣犱笉鏄亶鍘嗛〉琛紝鑰屾槸閬囧埌浜嗕綘鐨勪唬鐮佹棤娉曞師鐢熷鐞嗙殑鐗╃悊澶ч〉锛屼綘鍙互閫氳繃璋冪敤 split_huge_page(page) 鏉ユ媶鍒嗗畠銆?渚嬪锛岃繖姝ｆ槸 Linux VM 鍦ㄥ皾璇曟崲鍑猴紙swapout锛夎澶ч〉涔嬪墠鎵€鍋氱殑銆傚鏋滈〉琚浐瀹氾紙pinned锛夛紝split_huge_page() 鍙兘澶辫触锛?浣犲繀椤绘纭鐞嗚繖涓€鐐广€?
璁?mremap.c 鍙樺緱閫忔槑澶ч〉鎰熺煡鐨勫崟琛岀ず渚?
```
	diff --git a/mm/mremap.c b/mm/mremap.c
	--- a/mm/mremap.c
	+++ b/mm/mremap.c
	@@ -41,6 +41,7 @@ static pmd_t *get_old_pmd(struct mm_stru
			return NULL;

		pmd = pmd_offset(pud, addr);
	+	split_huge_pmd(vma, pmd, addr);
		if (pmd_none_or_clear_bad(pmd))
			return NULL;
```
## 澶ч〉鎰熺煡浠ｇ爜涓殑鍔犻攣


鎴戜滑甯屾湜灏藉彲鑳藉鐨勪唬鐮佸彉寰楀ぇ椤垫劅鐭ワ紝鍥犱负璋冪敤 split_huge_page() 鎴?split_huge_pmd() 鏄湁浠ｄ环鐨勩€?
瑕佽椤佃〃閬嶅巻鍙樺緱澶ч〉 pmd 鎰熺煡锛屼綘鍙渶瀵?pmd_offset 杩斿洖鐨?pmd 璋冪敤 pmd_trans_huge()銆備綘蹇呴』浠ヨ锛堟垨鍐欙級妯″紡鎸佹湁
mmap_lock锛屼互纭繚澶ч〉 pmd 涓嶄細琚?khugepaged 浠庝綘鑴氫笅鍒涘缓鍑烘潵锛坘hugepaged 鐨?collapse_huge_page 闄や簡 anon_vma 閿佸锛岃繕浼氫互鍐欐ā寮忚幏鍙?mmap_lock锛夈€?濡傛灉 pmd_trans_huge 杩斿洖 false锛屼綘灏卞洖閫€鍒版棫浠ｇ爜璺緞銆傚鏋?pmd_trans_huge 杩斿洖 true锛屼綘蹇呴』鑾峰彇椤佃〃閿侊紙pmd_lock()锛夊苟閲嶆柊杩愯 pmd_trans_huge銆傝幏鍙栭〉琛ㄩ攣灏嗛樆姝㈠ぇ椤?pmd 鍦ㄤ綘鑴氫笅琚浆鎹负甯歌 pmd锛坰plit_huge_pmd 鍙互涓庨〉琛ㄩ亶鍘嗗苟琛岃繍琛岋級銆傚鏋滅浜屾 pmd_trans_huge 杩斿洖 false锛屼綘鍙渶閲婃斁椤佃〃閿佸苟鍍忎箣鍓嶄竴鏍峰洖閫€鍒版棫浠ｇ爜銆傚惁鍒欙紝浣犲彲浠ョ户缁師鐢熷鐞嗗ぇ椤?pmd 鍜屽ぇ椤点€傚畬鎴愬悗锛岄噴鏀鹃〉琛ㄩ攣銆?
## 寮曠敤璁℃暟涓庨€忔槑澶ч〉


THP 涓婄殑寮曠敤璁℃暟涓庡叾浠栧鍚堬紙compound锛夐〉鐨勫紩鐢ㄨ鏁板熀鏈竴鑷达細

  - get_page()/put_page() 涓?GUP 鎿嶄綔 folio->_refcount銆?
  - 灏鹃〉涓殑 ->_refcount 濮嬬粓涓洪浂锛歡et_page_unless_zero() 鍦ㄥ熬椤典笂姘歌繙涓嶄細鎴愬姛銆?
  - 鏁翠釜 THP 鐨?PMD 琛ㄩ」鐨勬槧灏?瑙ｉ櫎鏄犲皠浼氶€掑/閫掑噺 folio->_entire_mapcount 涓?folio->_large_mapcount銆?
    鎴戜滑杩樼淮鎶や袱涓敤浜庤窡韪?MM 鎷ユ湁鑰咃紙MM ID 涓庡搴旂殑 mapcount锛変互鍙婂綋鍓嶇姸鎬侊紙鈥渕aybe mapped shared鈥?涓?    鈥渕apped exclusively鈥濓級鐨勬Ы浣嶃€?
    鍦?CONFIG_PAGE_MAPCOUNT 涓嬶紝褰?_entire_mapcount 浠?-1 鍙樺埌 0 鎴栦粠 0 鍙樺埌 -1 鏃讹紝鎴戜滑杩橀€氳繃 ENTIRELY_MAPPED 閫掑/閫掑噺 folio->_nr_pages_mapped銆?
  - 浣跨敤 PTE 琛ㄩ」瀵瑰崟涓〉鐨勬槧灏?瑙ｉ櫎鏄犲皠浼氶€掑/閫掑噺 folio->_large_mapcount銆?
    鎴戜滑杩樼淮鎶や袱涓敤浜庤窡韪?MM 鎷ユ湁鑰咃紙MM ID 涓庡搴旂殑 mapcount锛変互鍙婂綋鍓嶇姸鎬侊紙鈥渕aybe mapped shared鈥?涓?    鈥渕apped exclusively鈥濓級鐨勬Ы浣嶃€?
    鍦?CONFIG_PAGE_MAPCOUNT 涓嬶紝褰?page->_mapcount 浠?-1 鍙樺埌 0 鎴栦粠 0 鍙樺埌 -1 鏃讹紝鎴戜滑杩橀€掑/閫掑噺 page->_mapcount 浠ュ強 folio->_nr_pages_mapped锛屽洜涓鸿繖璁＄畻浜嗙敱 PTE 鏄犲皠鐨勯〉鏁伴噺銆?
split_huge_page 鍐呴儴蹇呴』鍦ㄤ粠椤电粨鏋勪腑娓呴櫎鎵€鏈?PG_head/tail 浣嶄箣鍓嶏紝灏嗗ご椤典腑鐨勫紩鐢ㄨ鏁板垎閰嶅埌灏鹃〉銆傚浜庣敱椤佃〃
琛ㄩ」鑾峰彇鐨勫紩鐢ㄨ鏁拌繖寰堝鏄撳仛鍒帮紝浣嗘垜浠濡備綍鍒嗛厤浠讳綍鍏朵粬鍥哄畾锛堝嵆鏉ヨ嚜 get_user_pages 鐨勶級缂轰箯瓒冲淇℃伅銆俿plit_huge_page() 浼氭嫆缁濅换浣曟媶鍒嗗凡琚浐瀹氬ぇ椤电殑璇锋眰锛氬畠鏈熸湜椤佃鏁扮瓑浜庢墍鏈夊瓙椤?mapcount 涔嬪拰鍔犱竴锛坰plit_huge_page 璋冪敤鑰呭繀椤绘寔鏈夊澶撮〉鐨勫紩鐢級銆?
split_huge_page 浣跨敤杩佺Щ锛坢igration锛夎〃椤规潵绋冲畾鍖垮悕椤电殑 page->_refcount 涓?page->_mapcount銆傛枃浠堕〉鍙槸琚В闄ゆ槧灏勩€?
鎴戜滑瀵圭墿鐞嗗唴瀛樻壂鎻忓櫒涔熸槸瀹夊叏鐨勶細鎵弿鍣ㄨ幏鍙栭〉寮曠敤鐨勫敮涓€鍚堟硶鏂瑰紡鏄?get_page_unless_zero()銆?
鎵€鏈夊熬椤靛湪 atomic_add() 涔嬪墠閮藉叿鏈夐浂鐨?->_refcount銆傝繖闃绘浜嗘壂鎻忓櫒鍦ㄦ涔嬪墠鑾峰彇瀵瑰熬椤电殑寮曠敤銆傚湪 atomic_add() 涔嬪悗锛屾垜浠笉鍐嶅叧蹇?->_refcount 鐨勫€笺€傛垜浠凡缁忕煡閬撳簲璇ヤ粠澶撮〉涓噴鏀惧灏戝紩鐢ㄣ€?
瀵逛簬澶撮〉锛実et_page_unless_zero() 浼氭垚鍔燂紝鎴戜滑骞朵笉浠嬫剰銆傛媶鍒嗗悗寮曠敤搴斿幓鍚戜綍澶勬槸鏄庣‘鐨勶細瀹冨皢鐣欏湪澶撮〉涓娿€?
娉ㄦ剰 split_huge_pmd() 鍦ㄥ紩鐢ㄨ鏁版柟闈㈡病鏈変换浣曢檺鍒讹細pmd 鍙互鍦ㄤ换浣曠偣鎷嗗垎涓旀案杩滀笉浼氬け璐ャ€?
## 閮ㄥ垎瑙ｉ櫎鏄犲皠涓?deferred_split_folio()锛堜粎鍖垮悕 THP锛?

瑙ｉ櫎 THP 鐨勪竴閮ㄥ垎鏄犲皠锛堥€氳繃 munmap() 鎴栧叾浠栨柟寮忥級涓嶄細绔嬪嵆閲婃斁鍐呭瓨銆傜浉鍙嶏紝鎴戜滑鍦?folio_remove_rmap_*() 涓娴嬪埌 THP 鐨勬煇涓瓙椤垫湭琚娇鐢紝
骞跺湪鍑虹幇鍐呭瓨鍘嬪姏鏃跺皢璇?THP 鎺掗槦绛夊緟鎷嗗垎銆傛媶鍒嗗皢閲婃斁鏈娇鐢ㄧ殑瀛愰〉銆?
鐢变簬鍦ㄥ彲浠ユ娴嬮儴鍒嗚В闄ゆ槧灏勭殑浣嶇疆锛岄攣涓婁笅鏂囦笉鍏佽绔嬪嵆鎷嗗垎璇ラ〉銆傜敱浜庡湪寰堝鎯呭喌涓嬶紝褰?THP 璺ㄨ秺 VMA 杈圭晫鏃讹紝閮ㄥ垎瑙ｉ櫎鏄犲皠鍙戠敓鍦?exit(2) 鏈熼棿锛岃繖涔熷彲鑳介€傚緱鍏跺弽銆?
deferred_split_folio() 鍑芥暟鐢ㄤ簬灏?folio 鎺掗槦绛夊緟鎷嗗垎銆傛媶鍒嗘湰韬皢鍦ㄦ垜浠€氳繃 shrinker 鎺ュ彛閬囧埌鍐呭瓨鍘嬪姏鏃跺彂鐢熴€?
鍦?CONFIG_PAGE_MAPCOUNT 涓嬶紝鎴戜滑鍩轰簬 folio->_nr_pages_mapped 鍙潬鍦版娴嬮儴鍒嗘槧灏勩€?
鍦?CONFIG_NO_PAGE_MAPCOUNT 涓嬶紝鎴戜滑鍩轰簬 THP 涓瘡椤靛钩鍧?mapcount 鏉ユ娴嬮儴鍒嗘槧灏勶細濡傛灉骞冲潎鍊?< 1锛屽垯涓€涓尶鍚?THP 鑲畾鏄儴鍒嗘槧灏勭殑銆傚彧瑕佸彧鏈変竴涓繘绋嬫槧灏勪竴涓?THP锛屾妫€娴嬪氨鏄彲闈犵殑銆傚浜庨暱鏃堕棿杩愯鐨勫瓙杩涚▼锛屽彲鑳藉瓨鍦ㄥ綋鍓嶆棤娉曟娴嬪埌閮ㄥ垎鏄犲皠鐨勫満鏅紝鏈潵鍙兘闇€瑕佸湪鍐呭瓨鍥炴敹鏈熼棿杩涜寮傛妫€娴嬨€?