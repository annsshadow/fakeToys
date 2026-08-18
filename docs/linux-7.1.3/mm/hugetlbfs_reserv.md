## Hugetlbfs 棰勭暀


## 姒傝堪


Documentation/admin-guide/mm/hugetlbpage.rst 涓弿杩扮殑宸ㄩ〉锛坔uge pages锛夐€氬父
涓哄簲鐢ㄧ▼搴忎娇鐢ㄨ€岄鍒嗛厤銆傚鏋滆繖浜涘法椤靛搴旂殑 VMA 鎸囩ず瑕佷娇鐢ㄥ法椤碉紝鍒欏畠浠細鍦?
缂洪〉锛坧age fault锛夋椂瀹炰緥鍖栧埌浠诲姟锛坱ask锛夌殑鍦板潃绌洪棿涓€傚鏋滃湪缂洪〉鏃朵笉瀛樺湪
宸ㄩ〉锛屼换鍔′細鏀跺埌 SIGBUS 淇″彿骞跺父甯镐互涓嶆剦蹇殑鏂瑰紡缁堟銆傚湪鍔犲叆宸ㄩ〉鏀寔鍚庝笉涔咃紝
浜轰滑纭畾鏈€濂藉湪 mmap() 鏃跺氨妫€娴嬪嚭宸ㄩ〉鐨勭煭缂恒€傚叾鎬濊矾鏄紝濡傛灉娌℃湁瓒冲鐨勫法椤?
鏉ヨ鐩栬鏄犲皠锛宮map() 灏变細澶辫触銆傝繖鏈€鍒濇槸閫氳繃鍦?mmap() 鏃跺仛涓€娆＄畝鍗曟鏌ワ紝
鍒ゆ柇鏄惁鏈夎冻澶熺殑绌洪棽宸ㄩ〉鏉ヨ鐩栬鏄犲皠鏉ュ畬鎴愮殑銆備笌鍐呮牳涓ぇ澶氭暟浜嬬墿涓€鏍凤紝
浠ｇ爜闅忕潃鏃堕棿涓嶆柇婕旇繘銆備笉杩囷紝鍏跺熀鏈€濊矾鏄湪 mmap() 鏃垛€滈鐣欌€濓紙reserve锛夊法椤碉紝
浠ョ‘淇濊鏄犲皠涓殑缂洪〉鑳藉鑾峰彇鍒板法椤点€備笅闈㈢殑鎻忚堪璇曞浘璇存槑鍦?v4.10 鍐呮牳涓?
宸ㄩ〉棰勭暀澶勭悊鏄浣曡繘琛岀殑銆?


## 璇昏€呭璞?


鏈弿杩颁富瑕侀潰鍚戞鍦ㄤ慨鏀?hugetlbfs 浠ｇ爜鐨勬牳寮€鍙戜汉鍛橈紙kernel developers锛夈€?


## 鏁版嵁缁撴瀯


resv_huge_pages
	杩欐槸涓€涓叏灞€鐨勶紙姣忎釜 hstate锛夊凡棰勭暀宸ㄩ〉璁℃暟銆傚凡棰勭暀鐨勫法椤?
	浠呭棰勭暀瀹冧滑鐨勪换鍔″彲鐢ㄣ€傚洜姝わ紝閫氬父鍙敤鐨勫法椤垫暟閲忚绠椾负
	(`free_huge_pages - resv_huge_pages`)銆?
Reserve Map
```

		struct resv_map {
			struct kref refs;
			spinlock_t lock;
			struct list_head regions;
			long adds_in_progress;
			struct list_head region_cache;
			long region_cache_count;
		};

	There is one reserve map for each huge page mapping in the system.
	The regions list within the resv_map describes the regions within
	the mapping.  A region is described as::

		struct file_region {
			struct list_head link;
			long from;
			long to;
		};

	The 'from' and 'to' fields of the file region structure are huge page
	indices into the mapping.  Depending on the type of mapping, a
	region in the reserv_map may indicate reservations exist for the
	range, or reservations do not exist.
```
Flags for MAP_PRIVATE Reservations
	杩欎簺鏍囧織瀛樺偍鍦ㄩ鐣欐槧灏勬寚閽堢殑浣庢瘮鐗逛綅涓€?

	`#define HPAGE_RESV_OWNER    (1UL << 0)`
		鎸囩ず姝や换鍔℃槸涓庢槧灏勫叧鑱旂殑棰勭暀鐨勬墍鏈夎€呫€?
	`#define HPAGE_RESV_UNMAPPED (1UL << 1)`
		鎸囩ず鏈€鍒濇槧灏勬鑼冨洿锛堝苟鍒涘缓棰勭暀锛夌殑浠诲姟鐢变簬涓€娆″け璐ョ殑
		鍐欐椂澶嶅埗锛圕OW锛夎€屽皢姝ら〉浠庤浠诲姟锛堝瓙杩涚▼锛夎В闄ゆ槧灏勩€?
Page Flags
	PagePrivate 椤垫爣蹇楃敤浜庢寚绀哄湪宸ㄩ〉琚噴鏀炬椂蹇呴』鎭㈠宸ㄩ〉棰勭暀銆?
	鏇村缁嗚妭灏嗗湪鈥滈噴鏀惧法椤碉紙Freeing huge pages锛夆€濅竴鑺備腑璁ㄨ銆?


## 棰勭暀鏄犲皠鐨勪綅缃紙绉佹湁鎴栧叡浜級


涓€涓法椤垫槧灏勬垨娈碉紙segment锛夎涔堟槸绉佹湁鐨勶紝瑕佷箞鏄叡浜殑銆傚鏋滄槸绉佹湁鐨勶紝
瀹冮€氬父浠呭鍗曚釜鍦板潃绌洪棿锛堜换鍔★級鍙敤銆傚鏋滄槸鍏变韩鐨勶紝瀹冨彲浠ユ槧灏勫埌澶氫釜
鍦板潃绌洪棿锛堜换鍔★級銆傞鐣欐槧灏勭殑浣嶇疆鍜岃涔夊浜庤繖涓ょ被鏄犲皠鏈夋樉钁椾笉鍚屻€?
浣嶇疆涓婄殑宸紓涓猴細

- 瀵逛簬绉佹湁鏄犲皠锛岄鐣欐槧灏勬寕鍦?VMA 缁撴瀯涓娿€傚叿浣撴潵璇达紝鍗?vma->vm_private_data銆?
  璇ラ鐣欐槧灏勫湪鏄犲皠锛坢map(MAP_PRIVATE)锛夊垱寤烘椂寤虹珛銆?
- 瀵逛簬鍏变韩鏄犲皠锛岄鐣欐槧灏勬寕鍦?inode 涓娿€傚叿浣撴潵璇达紝鍗?inode->i_mapping->private_data銆?
  鐢变簬鍏变韩鏄犲皠鎬绘槸鐢?hugetlbfs 鏂囦欢绯荤粺涓殑鏂囦欢浣滃悗澶囷紝hugetlbfs 浠ｇ爜浼氱‘淇?
  姣忎釜 inode 閮藉寘鍚竴涓鐣欐槧灏勩€傚洜姝わ紝棰勭暀鏄犲皠鍦?inode 鍒涘缓鏃跺垎閰嶃€?


## 鍒涘缓棰勭暀


褰撳垱寤轰竴涓敱宸ㄩ〉浣滃悗澶囩殑鍏变韩鍐呭瓨娈碉紙shmget(SHM_HUGETLB)锛夋垨閫氳繃
mmap(MAP_HUGETLB) 鍒涘缓鏄犲皠鏃讹紝浼氬垱寤洪鐣欍€?
```

	int hugetlb_reserve_pages(struct inode *inode,
				  long from, long to,
				  struct vm_area_struct *vma,
				  vm_flags_t vm_flags)

```
hugetlb_reserve_pages() 棣栧厛鍋氱殑鏄鏌?NORESERVE 鏍囧織鏄惁鍦?
shmget() 鎴?mmap() 璋冪敤涓鎸囧畾銆傚鏋滄寚瀹氫簡 NORESERVE锛屽垯璇ヤ緥绋嬬珛鍗宠繑鍥烇紝
鍥犱负涓嶉渶瑕佷换浣曢鐣欍€?

鍙傛暟 'from' 鍜?'to' 鏄槧灏勬垨搴曞眰鏂囦欢涓殑宸ㄩ〉绱㈠紩銆傚浜?shmget()锛?
'from' 濮嬬粓涓?0锛?to' 瀵瑰簲浜庢/鏄犲皠鐨勯暱搴︺€傚浜?mmap()锛宱ffset 鍙傛暟
鍙敤浜庢寚瀹氬簳灞傛枃浠朵腑鐨勫亸绉婚噺銆傚湪杩欑鎯呭喌涓嬶紝'from' 鍜?'to' 鍙傛暟宸茬粡
鏍规嵁璇ュ亸绉婚噺杩涜浜嗚皟鏁淬€?

PRIVATE 涓?SHARED 鏄犲皠涔嬮棿鐨勪竴涓噸澶у尯鍒湪浜庨鐣欏湪棰勭暀鏄犲皠涓殑琛ㄧず鏂瑰紡銆?

- 瀵逛簬鍏变韩鏄犲皠锛岄鐣欐槧灏勪腑鐨勪竴涓潯鐩寚绀哄搴旈〉瀛樺湪鎴栨浘缁忓瓨鍦ㄩ鐣欍€?
  闅忕潃棰勭暀琚秷璐癸紝棰勭暀鏄犲皠涓嶄細琚慨鏀广€?
- 瀵逛簬绉佹湁鏄犲皠锛岄鐣欐槧灏勪腑缂哄皯鏉＄洰鎸囩ず瀵瑰簲椤靛瓨鍦ㄩ鐣欍€傞殢鐫€棰勭暀琚秷璐癸紝
  浼氬悜棰勭暀鏄犲皠涓坊鍔犳潯鐩€傚洜姝わ紝棰勭暀鏄犲皠涔熷彲鐢ㄤ簬纭畾鍝簺棰勭暀宸茬粡琚秷璐广€?

瀵逛簬绉佹湁鏄犲皠锛宧ugetlb_reserve_pages() 鍒涘缓棰勭暀鏄犲皠骞跺皢鍏舵寕鍦?VMA 缁撴瀯涓娿€?
姝ゅ锛屼細璁剧疆 HPAGE_RESV_OWNER 鏍囧織浠ユ寚绀烘 VMA 鎷ユ湁杩欎簺棰勭暀銆?

鏌ヨ棰勭暀鏄犲皠浠ョ‘瀹氬綋鍓嶆槧灏?娈甸渶瑕佸灏戝法椤甸鐣欍€傚浜庣鏈夋槧灏勶紝杩欏缁堟槸
鍊?(to - from)銆傜劧鑰岋紝瀵逛簬鍏变韩鏄犲皠锛屾湁鍙兘璇ヨ寖鍥?(to - from) 鍐呭凡缁忓瓨鍦?
鏌愪簺棰勭暀銆傛湁鍏宠繖鏄浣曞畬鎴愮殑缁嗚妭锛岃鍙傝棰勭暀鏄犲皠鐨勪慨鏀逛竴鑺?
<resv_map_modifications>銆?

鏄犲皠鍙兘涓庝竴涓瓙姹狅紙subpool锛夊叧鑱斻€傚鏋滄槸杩欐牱锛屼細鏌ヨ瀛愭睜浠ョ‘淇濇槧灏勬湁
瓒冲鐨勭┖闂淬€傚瓙姹犳湁鍙兘棰勭暀浜嗕竴浜涘彲渚涜鏄犲皠浣跨敤鐨勯鐣欍€傛洿澶氱粏鑺傝鍙傝
瀛愭睜棰勭暀涓€鑺?<sub_pool_resv>銆?

鍦ㄦ煡璇㈤鐣欐槧灏勫拰瀛愭睜涔嬪悗锛屾墍闇€鐨勬柊棰勭暀鏁伴噺渚垮凡鐭ャ€傝皟鐢ㄤ緥绋?
hugetlb_acct_memory() 鏉ユ鏌ュ苟鍙栧緱鎵€璇锋眰鐨勯鐣欐暟閲忋€俬ugetlb_acct_memory()
浼氳皟鐢ㄤ竴浜涗緥绋嬶紝杩欎簺渚嬬▼鍙兘浼氬垎閰嶅苟璋冩暣鐩堜綑锛坰urplus锛夐〉璁℃暟銆?
鐒惰€岋紝鍦ㄨ繖浜涗緥绋嬪唴閮紝浠ｇ爜鍙槸绠€鍗曞湴妫€鏌ヤ互纭繚鏈夎冻澶熺殑绌洪棽宸ㄩ〉鏉ユ弧瓒?
璇ラ鐣欍€傚鏋滄湁锛屽叏灞€棰勭暀璁℃暟 resv_huge_pages 浼氳绫讳技濡備笅鏂瑰紡璋冩暣锛?
```

	if (resv_needed <= (free_huge_pages - resv_huge_pages)
		resv_huge_pages += resv_needed;

```
娉ㄦ剰锛屽湪妫€鏌ュ拰璋冩暣杩欎簺璁℃暟鍣ㄦ椂锛屾寔鏈夊叏灞€閿?hugetlb_lock銆?

濡傛灉褰撴椂鏈夎冻澶熺殑绌洪棽宸ㄩ〉涓斿叏灞€璁℃暟 resv_huge_pages 宸茶璋冩暣锛屽垯浼氫慨鏀?
涓庤鏄犲皠鍏宠仈鐨勯鐣欐槧灏勪互鍙嶆槧杩欎簺棰勭暀銆傚浜庡叡浜槧灏勶紝浼氬瓨鍦ㄤ竴涓寘鍚?
鑼冨洿 'from' - 'to' 鐨?file_region銆傚浜庣鏈夋槧灏勶紝涓嶄細瀵归鐣欐槧灏勫仛浠讳綍
淇敼锛屽洜涓虹己灏戞潯鐩嵆琛ㄧず瀛樺湪棰勭暀銆?

濡傛灉 hugetlb_reserve_pages() 鎴愬姛锛屽垯浼氭牴鎹渶瑕佷慨鏀逛笌璇ユ槧灏勫叧鑱旂殑鍏ㄥ眬
棰勭暀璁℃暟鍜岄鐣欐槧灏勶紝浠ョ‘淇濊寖鍥?'from' - 'to' 鍐呭瓨鍦ㄩ鐣欍€?


## 娑堣垂棰勭暀/鍒嗛厤宸ㄩ〉


褰撲笌棰勭暀鍏宠仈鐨勫法椤佃鍒嗛厤骞跺疄渚嬪寲鍒扮浉搴旀槧灏勪腑鏃讹紝棰勭暀鍗宠娑堣垂銆傚垎閰?
```

	struct folio *alloc_hugetlb_folio(struct vm_area_struct *vma,
				     unsigned long addr, int avoid_reserve)

```
alloc_hugetlb_folio 琚紶鍏ヤ竴涓?VMA 鎸囬拡鍜屼竴涓櫄鎷熷湴鍧€锛屽洜姝ゅ畠鍙互
鏌ヨ棰勭暀鏄犲皠浠ョ‘瀹氭槸鍚﹀瓨鍦ㄩ鐣欍€傛澶栵紝alloc_hugetlb_folio 鎺ュ彈鍙傛暟
avoid_reserve锛岃鍙傛暟鎸囩ず鍗充娇鐪嬩技宸蹭负鎸囧畾鍦板潃棰勭暀浜嗚祫婧愶紝涔熶笉搴斾娇鐢?
杩欎簺棰勭暀銆俛void_reserve 鍙傛暟鏈€甯哥敤浜庡啓鏃跺鍒讹紙Copy on Write锛夊拰椤佃縼绉?
锛圥age Migration锛夌殑鍦烘櫙锛屾鏃舵鍦ㄥ垎閰嶇幇鏈夐〉鐨勯澶栧壇鏈€?

杈呭姪渚嬬▼ vma_needs_reservation() 琚皟鐢ㄦ潵纭畾鏄犲皠锛坴ma锛変腑璇ュ湴鍧€鏄惁
瀛樺湪棰勭暀銆傛湁鍏宠渚嬬▼琛屼负鐨勮缁嗕俊鎭紝璇峰弬瑙侀鐣欐槧灏勮緟鍔╀緥绋嬩竴鑺?
<resv_map_helpers>銆?
vma_needs_reservation() 鐨勮繑鍥炲€奸€氬父涓?0 鎴?1銆傚鏋滆鍦板潃瀛樺湪棰勭暀鍒欎负 0锛?
濡傛灉涓嶅瓨鍦ㄩ鐣欏垯涓?1銆傚鏋滀笉瀛樺湪棰勭暀锛屼笖鏄犲皠鍏宠仈浜嗕竴涓瓙姹狅紝鍒欐煡璇㈣瀛愭睜
浠ョ‘瀹氬畠鏄惁鍖呭惈棰勭暀銆傚鏋滃瓙姹犲寘鍚鐣欙紝鍒欏叾涓竴浠藉彲鐢ㄤ簬姝ゆ鍒嗛厤銆?
鐒惰€岋紝鍦ㄤ换浣曟儏鍐典笅锛宎void_reserve 鍙傛暟閮戒細瑕嗙洊瀵硅棰勭暀鐨勪娇鐢ㄣ€傚湪纭畾浜?
鏄惁瀛樺湪棰勭暀涓斿彲鐢ㄤ簬姝ゆ鍒嗛厤涔嬪悗锛屼細璋冪敤渚嬬▼ dequeue_huge_page_vma()銆?
璇ヤ緥绋嬫帴鍙椾袱涓笌棰勭暀鐩稿叧鐨勫弬鏁帮細

- avoid_reserve锛岃繖鏄紶鍏?alloc_hugetlb_folio() 鐨勫悓涓€涓€?鍙傛暟銆?
- chg锛屽敖绠¤鍙傛暟鐨勭被鍨嬩负 long锛屼絾鍙紶鍏ュ€?0 鎴?1 缁?dequeue_huge_page_vma銆?
  濡傛灉鍊间负 0锛岃〃绀哄瓨鍦ㄩ鐣欙紙鍙兘瀛樺湪鐨勯棶棰樿鍙傝鈥滃唴瀛樼瓥鐣ヤ笌棰勭暀鈥濅竴鑺傦級銆?
  濡傛灉鍊间负 1锛岃〃绀轰笉瀛樺湪棰勭暀锛屼笖濡傛灉鍙兘鐨勮瘽锛岃椤靛繀椤讳粠鍏ㄥ眬绌洪棽姹犱腑鑾峰彇銆?

鎼滅储涓?VMA 鍐呭瓨绛栫暐鍏宠仈鐨勭┖闂查摼琛ㄤ互瀵绘壘绌洪棽椤点€傚鏋滄壘鍒颁竴椤碉紝褰撹椤典粠
绌洪棽閾捐〃涓Щ闄ゆ椂锛宖ree_huge_pages 鐨勫€间細閫掑噺銆傚鏋滃瓨鍦ㄩ鐣?
```

	SetPagePrivate(page);	/* 鎸囩ず鍒嗛厤姝ら〉娑堣垂浜嗕竴涓鐣欙紝骞朵笖
				 * 濡傛灉閬囧埌閿欒蹇呴』閲婃斁璇ラ〉锛?
				 * 棰勭暀灏嗚鎭㈠銆?*/
	resv_huge_pages--;	/* 閫掑噺鍏ㄥ眬棰勭暀璁℃暟 */

```
娉ㄦ剰锛屽鏋滄壘涓嶅埌婊¤冻 VMA 鍐呭瓨绛栫暐鐨勫法椤碉紝灏嗗皾璇曚娇鐢ㄤ紮浼村垎閰嶅櫒锛坆uddy allocator锛?
鍒嗛厤涓€椤点€傝繖灏卞紩鍑轰簡鐩堜綑宸ㄩ〉鍜岃繃閲忔彁浜わ紙overcommit锛夌殑闂锛岃繖瓒呭嚭浜嗛鐣欑殑
璁ㄨ鑼冨洿銆傚嵆浣垮垎閰嶄簡鐩堜綑椤碉紝涔熶細杩涜涓庝笂杩扮浉鍚岀殑鍩轰簬棰勭暀鐨勮皟鏁达細
SetPagePrivate(page) 鍜?resv_huge_pages--銆?

鍦ㄨ幏寰椾竴涓柊鐨?hugetlb folio 涔嬪悗锛屽鏋滆椤靛叧鑱旂殑瀛愭睜瀛樺湪锛屽垯 (folio)->_hugetlb_subpool
浼氳璁句负璇ュ瓙姹犵殑鍊笺€傝繖灏嗗湪 folio 琚噴鏀炬椂鐢ㄤ簬瀛愭睜璁拌处銆?

鐒跺悗璋冪敤渚嬬▼ vma_commit_reservation() 浠ユ牴鎹鐣欑殑娑堣垂鎯呭喌璋冩暣棰勭暀鏄犲皠銆?
涓€鑸潵璇达紝杩欐秹鍙婄‘淇濆湪鍖哄煙鏄犲皠鐨?file_region 缁撴瀯涓〃绀鸿椤点€傚浜庨鐣欏凡
瀛樺湪鐨勫叡浜槧灏勶紝棰勭暀鏄犲皠涓凡瀛樺湪鏉＄洰锛屽洜姝や笉鍋氭洿鏀广€傜劧鑰岋紝濡傛灉鏄叡浜槧灏勪腑
娌℃湁棰勭暀锛屾垨鑰呰繖鏄鏈夋槧灏勶紝鍒欏繀椤诲垱寤轰竴涓柊鏉＄洰銆?

鍦?alloc_hugetlb_folio() 寮€澶磋皟鐢?vma_needs_reservation() 涓?folio 鍒嗛厤鍚庤皟鐢?
vma_commit_reservation() 涔嬮棿锛岄鐣欐槧灏勬湁鍙兘宸茶鏇存敼銆傚鏋滃湪鍏变韩鏄犲皠涓
鍚屼竴椤佃皟鐢ㄤ簡 hugetlb_reserve_pages锛屽氨浼氬彂鐢熻繖绉嶆儏鍐点€傚湪杩欑鎯呭喌涓嬶紝棰勭暀璁℃暟
鍜屽瓙姹犵┖闂查〉璁℃暟浼氱浉宸竴銆傝繖绉嶇綍瑙佺殑鎯呭喌鍙互閫氳繃姣旇緝 vma_needs_reservation
鍜?vma_commit_reservation 鐨勮繑鍥炲€兼潵璇嗗埆銆傚鏋滄娴嬪埌杩欑绔炰簤锛屼細璋冩暣瀛愭睜鍜?
鍏ㄥ眬棰勭暀璁℃暟浠ヨ繘琛岃ˉ鍋裤€傛湁鍏宠繖浜涗緥绋嬬殑鏇村淇℃伅锛岃鍙傝棰勭暀鏄犲皠杈呭姪渚嬬▼涓€鑺?
<resv_map_helpers>銆?


## 瀹炰緥鍖栧法椤?


鍦ㄥ垎閰嶅法椤典箣鍚庯紝璇ラ〉閫氬父浼氳鍔犲叆鍒嗛厤浠诲姟鐨勯〉琛ㄤ腑銆傚湪姝や箣鍓嶏紝鍏变韩鏄犲皠涓殑
椤典細琚姞鍏ラ〉缂撳瓨锛坧age cache锛夛紝绉佹湁鏄犲皠涓殑椤典細琚姞鍏ュ尶鍚嶅弽鍚戞槧灏?
锛坅nonymous reverse mapping锛夈€傚湪杩欎袱绉嶆儏鍐典笅锛孭agePrivate 鏍囧織閮戒細琚竻闄ゃ€?
鍥犳锛屽綋宸插疄渚嬪寲鐨勫法椤佃閲婃斁鏃讹紝涓嶄細瀵瑰叏灞€棰勭暀璁℃暟锛坮esv_huge_pages锛夊仛璋冩暣銆?


## 閲婃斁宸ㄩ〉


宸ㄩ〉鐢?free_huge_folio() 閲婃斁銆傜敱浜庡畠鏄粠閫氱敤 MM 浠ｇ爜璋冪敤鐨勶紝鍥犳鍙紶鍏?
涓€涓寚鍚?folio 鐨勬寚閽堛€傚綋閲婃斁涓€涓法椤垫椂锛屽彲鑳介渶瑕佹墽琛岄鐣欒璐︺€傚鏋?
璇ラ〉鍏宠仈浜嗕竴涓寘鍚鐣欑殑瀛愭睜锛屾垨鑰呰椤垫鍦ㄩ敊璇矾寰勪笂琚噴鏀句笖蹇呴』鎭㈠
鍏ㄥ眬棰勭暀璁℃暟锛屽氨灞炰簬杩欑鎯呭喌銆?

page->private 瀛楁鎸囧悜浠讳綍涓庤椤靛叧鑱旂殑瀛愭睜銆傚鏋滆缃簡 PagePrivate 鏍囧織锛?
鍒欐寚绀哄簲璋冩暣鍏ㄥ眬棰勭暀璁℃暟锛堟湁鍏宠繖浜涙爣蹇楀浣曡缃殑淇℃伅锛岃鍙傝娑堣垂棰勭暀/
鍒嗛厤宸ㄩ〉涓€鑺?<consume_resv>锛夈€?

璇ヤ緥绋嬮鍏堜负璇ラ〉璋冪敤 hugepage_subpool_put_pages()銆傚鏋滄渚嬬▼杩斿洖鍊间负 0
锛堜笉绛変簬浼犲叆鐨勫€?1锛夛紝鍒欐寚绀烘湁棰勭暀涓庤瀛愭睜鍏宠仈锛屽苟涓旇繖涓柊閲婃斁鐨勯〉蹇呴』
鐢ㄤ簬淇濇寔瀛愭睜棰勭暀鏁伴噺涓嶄綆浜庢渶灏忓ぇ灏忋€傚洜姝わ紝鍦ㄨ繖绉嶆儏鍐典笅鍏ㄥ眬 resv_huge_pages
璁℃暟鍣ㄤ細閫掑銆?

濡傛灉椤典腑璁剧疆浜?PagePrivate 鏍囧織锛屽叏灞€ resv_huge_pages 璁℃暟鍣ㄥ皢鎬绘槸琚€掑銆?


## 瀛愭睜锛圫ubpool锛夐鐣?


姣忎釜宸ㄩ〉澶у皬閮藉叧鑱斾竴涓?struct hstate銆俬state 璺熻釜鎸囧畾澶у皬鐨勬墍鏈夊法椤点€?
瀛愭睜琛ㄧず涓€涓?hstate 涓笌绠＄悊鎸傝浇鐨?hugetlbfs 鏂囦欢绯荤粺鍏宠仈鐨勪竴閮ㄥ垎椤点€?

鎸傝浇 hugetlbfs 鏂囦欢绯荤粺鏃讹紝鍙互鎸囧畾 min_size 閫夐」锛屾寚绀鸿鏂囦欢绯荤粺鎵€闇€鐨勬渶灏?
宸ㄩ〉鏁伴噺銆傚鏋滄寚瀹氫簡姝ら€夐」锛屽垯瀵瑰簲 min_size 鐨勫法椤垫暟閲忎細琚鐣欎緵璇ユ枃浠剁郴缁?
浣跨敤銆傛鏁伴噺璁板綍鍦?struct hugepage_subpool 鐨?min_hpages 瀛楁涓€傚湪鎸傝浇鏃讹紝
浼氳皟鐢?hugetlb_acct_memory(min_hpages) 鏉ラ鐣欐寚瀹氭暟閲忕殑宸ㄩ〉銆傚鏋滄棤娉曢鐣欙紝
鎸傝浇澶辫触銆?

褰撲粠瀛愭睜鑾峰彇椤垫垨灏嗗叾閲婃斁鍥炲瓙姹犳椂锛屼細璋冪敤渚嬬▼ hugepage_subpool_get/put_pages()銆?
瀹冧滑鎵ц鎵€鏈夊瓙姹犺璐︼紝骞惰窡韪笌瀛愭睜鍏宠仈鐨勪换浣曢鐣欍€俬ugepage_subpool_get/put_pages
琚紶鍏ョ敤浜庤皟鏁村瓙姹犫€滃凡鐢ㄩ〉鈥濊鏁扮殑宸ㄩ〉鏁伴噺锛坓et 鏃堕€掑噺锛宲ut 鏃堕€掑锛夈€傞€氬父锛?
瀹冧滑杩斿洖浼犲叆鐨勫悓涓€鍊硷紝鎴栬€呭鏋滃瓙姹犱腑娌℃湁瓒冲鐨勯〉鍒欒繑鍥為敊璇€?

鐒惰€岋紝濡傛灉瀛愭睜鍏宠仈浜嗛鐣欙紝鍒欏彲鑳借繑鍥炲皬浜庝紶鍏ュ€肩殑鍊笺€傝杩斿洖鍊兼寚绀哄繀椤?
杩涜鐨勯澶栧叏灞€姹犺皟鏁寸殑鏁伴噺銆備緥濡傦紝鍋囪涓€涓瓙姹犲寘鍚?3 涓凡棰勭暀鐨勫法椤碉紝鑰?
鏈変汉璇锋眰 5 涓€備笌璇ュ瓙姹犲叧鑱旂殑 3 涓鐣欓〉鍙敤浜庢弧瓒抽儴鍒嗚姹傘€備絾鏄紝杩樺繀椤绘湁
2 椤典粠鍏ㄥ眬姹犱腑鑾峰彇銆備负浜嗗皢姝や俊鎭紶閫掔粰璋冪敤鑰咃紝浼氳繑鍥炲€?2銆傜劧鍚庤皟鐢ㄨ€呰礋璐?
灏濊瘯浠庡叏灞€姹犱腑鑾峰彇棰濆鐨?2 椤点€?


## 鍐欐椂澶嶅埗锛圕OW锛変笌棰勭暀


鐢变簬鍏变韩鏄犲皠閮芥寚鍚戝苟浣跨敤鐩稿悓鐨勫簳灞傞〉锛孋OW 鏈€澶х殑棰勭暀闂鍦ㄤ簬绉佹湁鏄犲皠銆?
鍦ㄨ繖绉嶆儏鍐典笅锛屼袱涓换鍔″彲鑳芥寚鍚戝悓涓€涓厛鍓嶅凡鍒嗛厤鐨勯〉銆備竴涓换鍔″皾璇曞啓鍏ヨ椤碉紝
鍥犳蹇呴』鍒嗛厤涓€涓柊椤碉紝浣挎瘡涓换鍔￠兘鎸囧悜鑷繁鐨勯〉銆?

褰撹椤垫渶鍒濊鍒嗛厤鏃讹紝璇ラ〉鐨勯鐣欏凡琚秷璐广€傚綋鐢变簬 COW 鑰屽皾璇曞垎閰嶆柊椤垫椂锛?
鏈夊彲鑳芥病鏈夌┖闂插法椤碉紝鍒嗛厤灏嗗け璐ャ€?

褰撶鏈夋槧灏勬渶鍒濊鍒涘缓鏃讹紝閫氳繃璁剧疆鎷ユ湁鑰呴鐣欐槧灏勬寚閽堜腑鐨?HPAGE_RESV_OWNER
浣嶆潵璁板綍璇ユ槧灏勭殑鎷ユ湁鑰呫€傜敱浜庢嫢鏈夎€呭垱寤轰簡鏄犲皠锛屾嫢鏈夎€呭氨鎷ユ湁涓庤鏄犲皠鍏宠仈鐨?
鎵€鏈夐鐣欍€傚洜姝わ紝褰撳彂鐢熷啓缂洪〉锛坵rite fault锛変笖娌℃湁鍙敤椤垫椂锛屽棰勭暀鐨勬嫢鏈夎€?
鍜岄潪鎷ユ湁鑰呬細閲囧彇涓嶅悓鐨勫姩浣溿€?

鍦ㄧ己椤典换鍔′笉鏄嫢鏈夎€呯殑鎯呭喌涓嬶紝缂洪〉灏嗗け璐ワ紝璇ヤ换鍔￠€氬父浼氭敹鍒?SIGBUS銆?

濡傛灉缂洪〉浠诲姟灏辨槸鎷ユ湁鑰咃紝鎴戜滑甯屾湜瀹冩垚鍔燂紝鍥犱负瀹冩嫢鏈夊師濮嬬殑棰勭暀銆備负浜嗗疄鐜拌繖涓€鐐癸紝
灏嗚椤典粠闈炴嫢鏈夎€呬换鍔¤В闄ゆ槧灏勩€傝繖鏍凤紝鍞竴鐨勫紩鐢ㄦ潵鑷嫢鏈夎€呬换鍔°€傛澶栵紝浼?
鍦ㄩ潪鎷ユ湁鑰呬换鍔＄殑棰勭暀鏄犲皠鎸囬拡涓缃?HPAGE_RESV_UNMAPPED 浣嶃€傚鏋滈潪鎷ユ湁鑰呬换鍔?
绋嶅悗瀵逛笉瀛樺湪鐨勯〉鍙戠敓缂洪〉锛屽畠鍙兘浼氭敹鍒?SIGBUS銆備絾鏄紝鏄犲皠/棰勭暀鐨勫師濮嬫嫢鏈夎€?
浼氭寜棰勬湡琛屼负銆?



## 棰勭暀鏄犲皠鐨勪慨鏀?


浠ヤ笅搴曞眰渚嬬▼鐢ㄤ簬瀵归鐣欐槧灏勮繘琛屼慨鏀广€傞€氬父锛屼笉浼氱洿鎺ヨ皟鐢ㄨ繖浜涗緥绋嬨€傜浉鍙嶏紝
浼氳皟鐢ㄤ竴涓鐣欐槧灏勮緟鍔╀緥绋嬶紝鐢卞畠鍐嶅幓璋冪敤鍏朵腑涓€涓簳灞備緥绋嬨€傝繖浜涘簳灞備緥绋嬪湪
婧愮爜涓湁鐩稿綋瀹屽杽鐨勬枃妗ｈ鏄?
```

	long region_chg(struct resv_map *resv, long f, long t);
	long region_add(struct resv_map *resv, long f, long t);
	void region_abort(struct resv_map *resv, long f, long t);
	long region_count(struct resv_map *resv, long f, long t);

```
瀵归鐣欐槧灏勭殑鎿嶄綔閫氬父娑夊強涓や釜姝ラ锛?

1) 璋冪敤 region_chg() 鏉ユ鏌ラ鐣欐槧灏勶紝纭畾鎸囧畾鑼冨洿 [f, t) 涓湁澶氬皯椤靛綋鍓?
   鏈琛ㄧず銆?

   璋冪敤浠ｇ爜鎵ц鍏ㄥ眬妫€鏌ュ拰鍒嗛厤锛屼互纭畾鏄惁鏈夎冻澶熺殑宸ㄩ〉浣挎搷浣滄垚鍔熴€?

2)
  a) 濡傛灉鎿嶄綔鍙互鎴愬姛锛岃皟鐢?region_add() 鏉ュ疄闄呬慨鏀逛箣鍓嶄紶缁?region_chg()
     鐨勫悓涓€鑼冨洿 [f, t) 鐨勯鐣欐槧灏勩€?
  b) 濡傛灉鎿嶄綔鏃犳硶鎴愬姛锛屽鍚屼竴涓寖鍥?[f, t) 璋冪敤 region_abort 鏉ヤ腑姝㈡搷浣溿€?

娉ㄦ剰锛岃繖鏄竴涓袱姝ヨ繃绋嬶紝鍦ㄥ鍚屼竴鑼冨洿鍏堣皟鐢?region_chg() 涔嬪悗锛宺egion_add()
鍜?region_abort() 淇濊瘉浼氭垚鍔熴€俽egion_chg() 璐熻矗棰勫垎閰嶄换浣曞繀瑕佺殑鏁版嵁缁撴瀯锛?
浠ョ‘淇濆悗缁搷浣滐紙鐗瑰埆鏄?region_add()锛変細鎴愬姛銆?

濡備笂鎵€杩帮紝region_chg() 纭畾鏄犲皠涓綋鍓嶆湭琚〃绀虹殑鑼冨洿鍐呯殑椤垫暟銆傝鏁板瓧杩斿洖缁?
璋冪敤鑰呫€俽egion_add() 杩斿洖娣诲姞鍒版槧灏勪腑鐨勮寖鍥村唴鐨勯〉鏁般€傚湪澶у鏁版儏鍐典笅锛?
region_add() 鐨勮繑鍥炲€间笌 region_chg() 鐨勮繑鍥炲€肩浉鍚屻€傜劧鑰岋紝瀵逛簬鍏变韩鏄犲皠锛屽湪
瀵?region_chg() 鍜?region_add() 鐨勮皟鐢ㄤ箣闂存湁鍙兘瀵归鐣欐槧灏勫仛浜嗕慨鏀广€傚湪杩欑
鎯呭喌涓嬶紝region_add() 鐨勮繑鍥炲€煎皢涓?region_chg() 鐨勮繑鍥炲€间笉鍖归厤銆傚緢鍙兘鍦?
杩欑鎯呭喌涓嬪叏灞€璁℃暟鍜屽瓙姹犺璐︿細涓嶆纭紝闇€瑕佸仛鍑鸿皟鏁淬€傝皟鐢ㄨ€呮湁璐ｄ换妫€鏌ヨ繖绉?
鎯呭喌骞惰繘琛岄€傚綋鐨勮皟鏁淬€?

璋冪敤 region_del() 鏉ヤ粠棰勭暀鏄犲皠涓Щ闄ゅ尯鍩熴€傞€氬父鍦ㄤ互涓嬫儏鍐典笅璋冪敤瀹冿細

- 褰?hugetlbfs 鏂囦欢绯荤粺涓殑鏂囦欢琚Щ闄ゆ椂锛宨node 灏嗚閲婃斁锛岄鐣欐槧灏勮閲婃斁銆?
  鍦ㄩ噴鏀鹃鐣欐槧灏勪箣鍓嶏紝蹇呴』閲婃斁鎵€鏈夊崟鐙殑 file_region 缁撴瀯銆傚湪杩欑鎯呭喌涓嬶紝
  region_del 琚紶鍏ヨ寖鍥?[0, LONG_MAX)銆?
- 褰?hugetlbfs 鏂囦欢琚埅鏂椂銆傚湪杩欑鎯呭喌涓嬶紝鏂版枃浠跺ぇ灏忎箣鍚庣殑鎵€鏈夊凡鍒嗛厤椤?
  蹇呴』琚噴鏀俱€傛澶栵紝棰勭暀鏄犲皠涓秴鍑烘枃浠舵柊鏈熬鐨勪换浣?file_region 鏉＄洰蹇呴』
  琚垹闄ゃ€傚湪杩欑鎯呭喌涓嬶紝region_del 琚紶鍏ヨ寖鍥?[new_end_of_file, LONG_MAX)銆?
- 褰撳湪 hugetlbfs 鏂囦欢涓墦瀛旓紙punch a hole锛夋椂銆傚湪杩欑鎯呭喌涓嬶紝宸ㄩ〉浼氫粠鏂囦欢
  涓棿閫愪釜绉婚櫎銆傞殢鐫€椤佃绉婚櫎锛岃皟鐢?region_del() 鏉ヤ粠棰勭暀鏄犲皠涓Щ闄ょ浉搴旂殑
  鏉＄洰銆傚湪杩欑鎯呭喌涓嬶紝region_del 琚紶鍏ヨ寖鍥?[page_idx, page_idx + 1)銆?

鍦ㄦ瘡绉嶆儏鍐典笅锛宺egion_del() 閮戒細杩斿洖浠庨鐣欐槧灏勪腑绉婚櫎鐨勯〉鏁般€傚湪闈炲父缃曡鐨?
鎯呭喌涓嬶紝region_del() 鍙兘澶辫触銆傝繖鍙彲鑳藉彂鐢熷湪鎵撳瓟鐨勬儏褰腑锛屾鏃跺畠蹇呴』鎷嗗垎
涓€涓幇鏈夌殑 file_region 鏉＄洰鍗村張鏃犳硶鍒嗛厤涓€涓柊鐨勭粨鏋勩€傚湪杩欑閿欒鎯呭喌涓嬶紝
region_del() 灏嗚繑鍥?-ENOMEM銆傝繖閲岀殑闂鏄紝棰勭暀鏄犲皠浼氭寚绀鸿椤靛瓨鍦ㄤ竴涓?
棰勭暀銆傜劧鑰岋紝瀛愭睜鍜屽叏灞€棰勭暀璁℃暟涓嶄細鍙嶆槧璇ラ鐣欍€備负浜嗗鐞嗚繖绉嶆儏鍐碉紝浼氳皟鐢?
渚嬬▼ hugetlb_fix_reserve_counts() 鏉ヨ皟鏁磋鏁板櫒锛屼娇鍏朵笌鏃犳硶鍒犻櫎鐨勯偅涓鐣?
鏄犲皠鏉＄洰鐩稿搴斻€?

鍦ㄥ彇娑堟槧灏勭鏈夊法椤垫槧灏勬椂锛屼細璋冪敤 region_count()銆傚湪绉佹湁鏄犲皠涓紝棰勭暀鏄犲皠涓?
缂哄皯鏉＄洰鎸囩ず瀛樺湪棰勭暀銆傚洜姝わ紝閫氳繃缁熻棰勭暀鏄犲皠涓殑鏉＄洰鏁帮紝鎴戜滑灏辩煡閬撳凡缁?
娑堣垂浜嗗灏戦鐣欙紝浠ュ強杩樻湁澶氬皯鏈喅锛坥utstanding = (end - start) - region_count(resv, start, end)锛夈€?
鐢变簬鏄犲皠姝ｅ湪娑堝け锛屽瓙姹犲拰鍏ㄥ眬棰勭暀璁℃暟浼氭寜鏈喅棰勭暀鐨勬暟閲忛€掑噺銆?


## 棰勭暀鏄犲皠杈呭姪渚嬬▼


瀛樺湪鑻ュ共杈呭姪渚嬬▼鐢ㄤ簬鏌ヨ鍜屼慨鏀归鐣欐槧灏勩€傝繖浜涗緥绋嬪彧鍏虫敞鐗瑰畾宸ㄩ〉鐨勯鐣欙紝
鍥犳瀹冧滑鍙紶鍏ヤ竴涓湴鍧€鑰岄潪涓€涓寖鍥淬€傛澶栵紝瀹冧滑浼犲叆鍏宠仈鐨?VMA銆備粠 VMA 涓?
鍙互纭畾鏄犲皠鐨勭被鍨嬶紙绉佹湁鎴栧叡浜級浠ュ強棰勭暀鏄犲皠鐨勪綅缃紙inode 鎴?VMA锛夈€傝繖浜?
渚嬬▼鍙槸绠€鍗曞湴璋冪敤鈥滈鐣欐槧灏勭殑淇敼鈥濅竴鑺備腑鎻忚堪鐨勫簳灞備緥绋嬨€傜劧鑰岋紝瀹冧滑纭疄
鑰冭檻浜嗙鏈夊拰鍏变韩鏄犲皠涓鐣欐槧灏勬潯鐩€滅浉鍙嶁€濈殑鍚箟
```

	long vma_needs_reservation(struct hstate *h,
				   struct vm_area_struct *vma,
				   unsigned long addr)

```
姝や緥绋嬪鎸囧畾椤佃皟鐢?region_chg()銆傚鏋滄病鏈夐鐣?
```

	long vma_commit_reservation(struct hstate *h,
				    struct vm_area_struct *vma,
				    unsigned long addr)

```
杩欏皢瀵规寚瀹氶〉璋冪敤 region_add()銆備笌 region_chg 鍜?region_add 鐨勬儏鍐典竴鏍凤紝
搴斿湪涔嬪墠璋冪敤杩?vma_needs_reservation 涔嬪悗鍐嶈皟鐢ㄦ渚嬬▼銆傚畠浼氫负璇ラ〉娣诲姞涓€涓?
棰勭暀鏉＄洰銆傚鏋滄坊鍔犱簡棰勭暀鍒欒繑鍥?1锛屽惁鍒欒繑鍥?0銆傝繑鍥炲€煎簲涓庝箣鍓嶅
vma_needs_reservation 鐨勮皟鐢ㄨ繑鍥炲€艰繘琛屾瘮杈冦€備竴涓剰澶栫殑宸紓鎸囩ず棰勭暀
```

	void vma_end_reservation(struct hstate *h,
				 struct vm_area_struct *vma,
				 unsigned long addr)

```
杩欏皢瀵规寚瀹氶〉璋冪敤 region_abort()銆備笌 region_chg 鍜?region_abort 鐨勬儏鍐典竴鏍凤紝
搴斿湪涔嬪墠璋冪敤杩?vma_needs_reservation 涔嬪悗鍐嶈皟鐢ㄦ渚嬬▼銆傚畠灏嗕腑姝?缁撴潫姝ｅ湪
杩涜鐨勯鐣欐坊鍔?
```

	long vma_add_reservation(struct hstate *h,
				 struct vm_area_struct *vma,
				 unsigned long addr)

```
杩欐槸涓€涓壒娈婄殑鍖呰渚嬬▼锛岀敤浜庡府鍔╁湪閿欒璺緞涓婅繘琛岄鐣欐竻鐞嗐€傚畠鍙粠渚嬬▼
restore_reserve_on_error() 涓皟鐢ㄣ€傛渚嬬▼涓?vma_needs_reservation 閰嶅悎
浣跨敤锛屼互灏濊瘯鍚戦鐣欐槧灏勬坊鍔犻鐣欍€傚畠鑰冭檻浜嗙鏈夊拰鍏变韩鏄犲皠涓嶅悓鐨勯鐣欐槧灏?
璇箟銆傚洜姝わ紝瀵逛簬鍏变韩鏄犲皠璋冪敤 region_add锛堝洜涓烘槧灏勪腑瀛樺湪鏉＄洰鍗宠〃绀烘湁棰勭暀锛夛紝
瀵逛簬绉佹湁鏄犲皠璋冪敤 region_del锛堝洜涓烘槧灏勪腑缂哄皯鏉＄洰鍗宠〃绀烘湁棰勭暀锛夈€傛湁鍏抽敊璇矾寰?
涓婇渶瑕佸仛浠€涔堢殑鏇村淇℃伅锛岃鍙傝鈥滈敊璇矾寰勪腑鐨勯鐣欐竻鐞嗏€濅竴鑺傘€?


## 閿欒璺緞涓殑棰勭暀娓呯悊


濡傞鐣欐槧灏勮緟鍔╀緥绋嬩竴鑺?<resv_map_helpers> 鎵€杩帮紝棰勭暀鏄犲皠鐨勪慨鏀瑰垎涓ゆ杩涜銆?
棣栧厛鍦ㄥ垎閰嶉〉涔嬪墠璋冪敤 vma_needs_reservation銆傚鏋滃垎閰嶆垚鍔燂紝鍒欒皟鐢?
vma_commit_reservation銆傚鏋滀笉鎴愬姛锛屽垯璋冪敤 vma_end_reservation銆傛牴鎹搷浣?
鐨勬垚鍔熸垨澶辫触鏉ヨ皟鏁村叏灞€鍜屽瓙姹犻鐣欒鏁帮紝涓€鍒囨甯搞€?

姝ゅ锛屽湪宸ㄩ〉琚疄渚嬪寲鍚庯紝PagePrivate 鏍囧織浼氳娓呴櫎锛屼互渚垮湪璇ラ〉鏈€缁堣閲婃斁鏃?
璁拌处姝ｇ‘銆?

鐒惰€岋紝瀛樺湪鑻ュ共鍦ㄥ法椤佃鍒嗛厤涔嬪悗銆佷絾鍦ㄥ叾琚疄渚嬪寲涔嬪墠閬囧埌閿欒鐨勬儏褰€傚湪杩欑
鎯呭喌涓嬶紝椤靛垎閰嶅凡缁忔秷璐逛簡棰勭暀锛屽苟鍋氫簡鐩稿簲鐨勫瓙姹犮€侀鐣欐槧灏勫拰鍏ㄥ眬璁℃暟璋冩暣銆?
濡傛灉姝ゆ椂锛堝湪瀹炰緥鍖栧拰娓呴櫎 PagePrivate 涔嬪墠锛夐噴鏀捐椤碉紝鍒?free_huge_folio
浼氶€掑鍏ㄥ眬棰勭暀璁℃暟銆備絾鏄紝棰勭暀鏄犲皠鎸囩ず棰勭暀宸茶娑堣垂銆傝繖绉嶄笉涓€鑷寸殑鐘舵€佸皢
瀵艰嚧涓€涓凡棰勭暀宸ㄩ〉鐨勨€滄硠婕忊€濓紙leak锛夈€傚叏灞€棰勭暀璁℃暟灏嗘瘮搴旀湁鐨勬洿楂橈紝骞堕樆姝?
鍒嗛厤涓€涓鍒嗛厤鐨勯〉銆?

渚嬬▼ restore_reserve_on_error() 璇曞浘澶勭悊杩欑鎯呭喌銆傚畠鏈夌浉褰撳畬鍠勭殑鏂囨。璇存槑銆?
璇ヤ緥绋嬬殑鎰忓浘鏄皢棰勭暀鏄犲皠鎭㈠鍒伴〉鍒嗛厤涔嬪墠鐨勭姸鎬併€傝繖鏍凤紝鍦ㄩ〉琚噴鏀惧悗锛岄鐣?
鏄犲皠鐨勭姸鎬佸皢涓庡叏灞€棰勭暀璁℃暟鐩稿搴斻€?

restore_reserve_on_error 渚嬬▼鏈韩鍦ㄥ皾璇曟仮澶嶉鐣欐槧灏勬潯鐩椂涔熷彲鑳介亣鍒伴敊璇€?
鍦ㄨ繖绉嶆儏鍐典笅锛屽畠浼氱畝鍗曞湴娓呴櫎璇ラ〉鐨?PagePrivate 鏍囧織銆傝繖鏍凤紝鍦ㄨ椤佃閲婃斁鏃?
鍏ㄥ眬棰勭暀璁℃暟涓嶄細琚€掑銆備絾鏄紝棰勭暀鏄犲皠浼氱户缁湅璧锋潵鍍忔槸棰勭暀宸茶娑堣垂銆備粛鐒?
鍙互涓鸿鍦板潃鍒嗛厤涓€涓〉锛屼絾瀹冧笉浼氬儚鏈€鍒濋鏈熺殑閭ｆ牱浣跨敤涓€涓凡棰勭暀鐨勯〉銆?

鏈変竴浜涗唬鐮侊紙鏈€鏄捐憲鐨勬槸 userfaultfd锛夋棤娉曡皟鐢?restore_reserve_on_error銆?
鍦ㄨ繖绉嶆儏鍐典笅锛屽畠鍙槸淇敼 PagePrivate锛屼互渚垮湪閲婃斁宸ㄩ〉鏃朵笉浼氭硠婕忛鐣欍€?


## 棰勭暀涓庡唴瀛樼瓥鐣?


褰?git 鏈€鍒濊鐢ㄤ簬绠＄悊 Linux 浠ｇ爜鏃讹紝per-node 宸ㄩ〉閾捐〃灏卞凡缁忓瓨鍦ㄤ簬 struct hstate
涓€傞鐣欑殑姒傚康鏄湪涓€娈垫椂闂翠箣鍚庢墠鍔犲叆鐨勩€傚姞鍏ラ鐣欐椂锛屽苟娌℃湁灏濊瘯灏嗗唴瀛樼瓥鐣?
鑰冭檻鍦ㄥ唴銆傝櫧鐒?cpusets 涓庡唴瀛樼瓥鐣ュ苟涓嶅畬鍏ㄧ浉鍚岋紝浣?hugetlb_acct_memory 涓殑
杩欐娉ㄩ噴姒傛嫭浜嗛鐣欎笌瀹冧滑涔嬮棿鐨勭浉浜掍綔鐢?
```

	/*
	 * When cpuset is configured, it breaks the strict hugetlb page
	 * reservation as the accounting is done on a global variable. Such
	 * reservation is completely rubbish in the presence of cpuset because
	 * the reservation is not checked against page availability for the
	 * current cpuset. Application can still potentially OOM'ed by kernel
	 * with lack of free htlb page in cpuset that the task is in.
	 * Attempt to enforce strict accounting with cpuset is almost
	 * impossible (or too ugly) because cpuset is too fluid that
	 * task or memory node can be dynamically moved between cpusets.
	 *
	 * The change of semantics for shared hugetlb mapping with cpuset is
	 * undesirable. However, in order to preserve some of the semantics,
	 * we fall back to check against current free page availability as
	 * a best attempt and hopefully to minimize the impact of changing
	 * semantics that cpuset has.
	 */

```
鍔犲叆宸ㄩ〉棰勭暀鏄负浜嗛槻姝㈠湪缂洪〉鏃跺嚭鐜版剰澶栫殑椤靛垎閰嶅け璐ワ紙OOM锛夈€傜劧鑰岋紝濡傛灉搴旂敤
绋嬪簭浣跨敤浜?cpusets 鎴栧唴瀛樼瓥鐣ワ紝鍒欐棤娉曚繚璇佸湪鎵€闇€鑺傜偣涓婃湁宸ㄩ〉鍙敤銆傚嵆浣挎湁
瓒冲鏁伴噺鐨勫叏灞€棰勭暀锛屼篃鏄姝ゃ€?

## Hugetlbfs 鍥炲綊娴嬭瘯


鏈€瀹屾暣鐨?hugetlb 娴嬭瘯闆嗕綅浜?libhugetlbfs 浠撳簱涓€傚鏋滀綘淇敼浜嗕换浣?hugetlb
鐩稿叧鐨勪唬鐮侊紝璇蜂娇鐢?libhugetlbfs 娴嬭瘯濂椾欢鏉ユ鏌ユ槸鍚﹀嚭鐜板洖褰掋€傛澶栵紝濡傛灉浣?
娣诲姞浜嗕换浣曟柊鐨?hugetlb 鍔熻兘锛岃鍚?libhugetlbfs 娣诲姞閫傚綋鐨勬祴璇曘€?

--
Mike Kravetz, 2017 骞?4 鏈?7 鏃?
