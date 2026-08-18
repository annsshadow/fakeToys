## 閫忔槑澶ч〉鏀寔


## 鐩爣


澶勭悊澶у瀷鍐呭瓨宸ヤ綔闆嗐€佸鎬ц兘瑕佹眰鑻涘埢鐨勮绠楀簲鐢紝宸茬粡鍦?libhugetlbfs 涔嬩笂銆佽繘鑰屽湪
hugetlbfs 涔嬩笂杩愯銆傞€忔槑澶ч〉鏀寔锛圱HP锛孴ransparent HugePage锛夋槸鍙︿竴绉嶄娇鐢ㄥぇ椤典綔涓?铏氭嫙鍐呭瓨鍚庣鐨勬柟寮忥紝瀹冩敮鎸侀〉澶у皬鐨勮嚜鍔ㄦ彁鍗囷紙promotion锛夊拰闄嶇骇锛坉emotion锛夛紝骞朵笖
娌℃湁 hugetlbfs 鐨勯偅浜涚己鐐广€?
鐩墠 THP 浠呴€傜敤浜庡尶鍚嶅唴瀛樻槧灏勪互鍙?tmpfs/shmem銆備絾鏈潵鍙互鎵╁睍鍒板叾瀹冩枃浠剁郴缁熴€?
   鍦ㄤ笅闈㈢殑绀轰緥涓紝鎴戜滑鍋囪鍩烘湰椤靛ぇ灏忎负 4K锛屽ぇ椤靛ぇ灏忎负 2M锛屽敖绠″疄闄呮暟鍊煎彲鑳藉洜
   CPU 鏋舵瀯鑰屽紓銆?
搴旂敤绋嬪簭杩愯寰楁洿蹇湁涓や釜鍘熷洜銆傜涓€涓洜绱犲嚑涔庡畬鍏ㄦ棤鍏崇揣瑕侊紝鑰屼笖瀹冩病鏈夐噸澶т环鍊硷紝
鍥犱负瀹冧篃鏈夌己鐐癸細鍦ㄩ〉閿欒锛坧age fault锛夋椂闇€瑕佹洿澶х殑 clear-page / copy-page锛岃繖
鏄竴涓綔鍦ㄧ殑璐熼潰褰卞搷銆傜涓€涓洜绱犵敱鐢ㄦ埛鎬佹瘡瑙﹀強 2M 铏氭嫙鍖哄煙鍙骇鐢熶竴娆￠〉閿欒缁勬垚
锛堜粠鑰屽皢杩涘叆/閫€鍑哄唴鏍哥殑棰戠巼闄嶄綆浜?512 鍊嶏級銆傝繖鍙鍐呭瓨鏄犲皠鐢熷懡鍛ㄦ湡鍐呴娆¤闂?鍐呭瓨鏃舵墠鏈夋剰涔夈€傜浜屼釜褰卞搷鎸佷箙涓旈噸瑕佸緱澶氱殑鍥犵礌锛屼細鍦ㄥ簲鐢ㄧ▼搴忔暣涓繍琛屾湡闂村奖鍝?瀵瑰唴瀛樼殑鎵€鏈夊悗缁闂€傜浜屼釜鍥犵礌鐢变袱涓儴鍒嗙粍鎴愶細

1) TLB 缂哄け锛坢iss锛変細杩愯寰楁洿蹇紙灏ゅ叾鏄湪浣跨敤宓屽椤佃〃锛坣ested pagetables锛夌殑
   铏氭嫙鍖栦腑锛屼絾鍑犱箮鍦ㄨ８鏈烘病鏈夎櫄鎷熷寲鏃朵篃鎬绘槸濡傛锛?
2) 鍗曚釜 TLB 琛ㄩ」灏嗘槧灏勫ぇ寰楀鐨勮櫄鎷熷唴瀛橀噺锛岃繘鑰屽噺灏?TLB 缂哄け鐨勬鏁般€傚湪铏氭嫙鍖栧拰
   宓屽椤佃〃涓嬶紝鍙湁褰?KVM 鍜?Linux 瀹㈡埛鏈洪兘浣跨敤澶ч〉鏃讹紝TLB 鎵嶈兘鏄犲皠鏇村ぇ鐨勫昂瀵革紝
   浣嗗彧瑕佷袱鑰呬箣涓€浣跨敤澶ч〉锛屽氨宸茬粡浼氭湁鏄捐憲鐨勫姞閫燂紝鍘熷洜浠呬粎鍦ㄤ簬 TLB 缂哄け浼氳繍琛?   寰楁洿蹇€?
鐜颁唬鍐呮牳鏀寔鈥渕ulti-size THP鈥濓紙mTHP锛屽灏哄 THP锛夛紝瀹冨紩鍏ヤ簡浠ュぇ浜庡熀鏈〉銆佷絾灏忎簬
浼犵粺 PMD 灏哄锛堝涓婃墍杩帮級鐨勫潡鏉ュ垎閰嶅唴瀛樼殑鑳藉姏锛屼互 2 鐨勫箓娆￠〉鏁颁负澧為噺銆俶THP 鍙互
浣滀负鍖垮悕鍐呭瓨鐨勫悗绔紙渚嬪 16K銆?2K銆?4K 绛夛級銆傝繖浜?THP 浠嶇劧鏄?PTE 鏄犲皠鐨勶紝浣嗗湪璁稿
鎯呭喌涓嬩粛鑳芥彁渚涗笌涓婇潰姒傝堪绫讳技鐨勬敹鐩婏細椤甸敊璇樉钁楀噺灏戯紙鍑忓皯绯绘暟涓轰緥濡?4銆?銆?6 绛夛級锛?浣嗗欢杩熷皷宄颁笉閭ｄ箞鏄庢樉锛屽洜涓烘瘡椤电殑澶у皬涓嶅儚 PMD 灏哄鐨勫彉浣撻偅涔堝ぇ锛屽苟涓旀瘡娆￠〉閿欒涓?闇€瑕佹竻闆剁殑鍐呭瓨涔熸洿灏戙€傛煇浜涙灦鏋勮繕閲囩敤 TLB 鍘嬬缉鏈哄埗锛屽綋涓€缁?PTE 鍦ㄨ櫄鎷熷拰鐗╃悊涓婅繛缁?骞堕€傚綋瀵归綈鏃讹紝鎶婃洿澶氳〃椤规尋杩涘幓銆傚湪杩欑鎯呭喌涓嬶紝TLB 缂哄け浼氭洿灏戝彂鐢熴€?
THP 鍙互鍦ㄧ郴缁熻寖鍥村唴鍚敤锛屼篃鍙互闄愬埗鍒版煇浜涗换鍔★紝鐢氳嚦浠诲姟鍦板潃绌洪棿鍐呯殑鏌愪簺鍐呭瓨鑼冨洿銆?闄ら潪瀹屽叏绂佺敤 THP锛屽惁鍒欎細鏈変竴涓?`khugepaged` 瀹堟姢杩涚▼鎵弿鍐呭瓨锛屽苟灏嗕竴绯诲垪鍩烘湰椤?鎶樺彔锛坈ollapse锛変负 PMD 灏哄鐨勫ぇ椤点€?
THP 鐨勮涓洪€氳繃 sysfs <thp_sysfs> 鎺ュ彛浠ュ強浣跨敤 madvise(2) 鍜?prctl(2) 绯荤粺璋冪敤鏉?鎺у埗銆?
閫忔槑澶ч〉鏀寔涓?hugetlbfs 鐨勯鐣欙紙reservation锛夋柟寮忕浉姣旓紝閫氳繃鍏佽鎵€鏈夋湭浣跨敤鐨勫唴瀛?琚敤浣滅紦瀛樻垨鍏跺畠鍙Щ鍔紙鐢氳嚦涓嶅彲绉诲姩锛夊疄浣擄紝鏈€澶у寲浜嗙┖闂插唴瀛樼殑鐢ㄥ銆傚畠涓嶉渶瑕侀鐣?鏉ラ槻姝粠鐢ㄦ埛鎬佸療瑙夊埌鐨勫ぇ椤靛垎閰嶅け璐ャ€傚畠鍏佽鍦ㄥぇ椤典笂浣跨敤鍒嗛〉浠ュ強鎵€鏈夊叾瀹冮珮绾?VM
鐗规€с€傚簲鐢ㄧ▼搴忓埄鐢ㄥ畠鏃犻渶浠讳綍淇敼銆?
涓嶈繃锛屽簲鐢ㄧ▼搴忓彲浠ヨ繘涓€姝ヤ紭鍖栦互鍒╃敤姝ょ壒鎬э紝灏卞儚浠ュ墠瀹冧滑琚紭鍖栦互閬垮厤姣忔 malloc(4k)
閮芥秾鍏ュぇ閲?mmap 绯荤粺璋冪敤涓€鏍枫€備紭鍖栫敤鎴锋€佽繙闈炲己鍒舵€х殑锛岃€屼笖 khugepaged 宸茬粡鍙互涓?闀挎湡瀛樺湪鐨勯〉鍒嗛厤鍏滃簳锛屽嵆浣挎槸閭ｄ簺瀵瑰ぇ椤垫棤鎰熺煡銆佸嵈澶勭悊澶ч噺鍐呭瓨鐨勫簲鐢ㄧ▼搴忎篃鏄姝ゃ€?
鍦ㄦ煇浜涙儏鍐典笅锛屽綋澶ч〉鍦ㄧ郴缁熻寖鍥村唴鍚敤鏃讹紝搴旂敤绋嬪簭鍙兘鏈€缁堝垎閰嶆洿澶氬唴瀛樿祫婧愩€備竴涓簲鐢?鍙兘 mmap 涓€涓緢澶х殑鍖哄煙浣嗗彧瑙﹀強鍏朵腑鐨?1 瀛楄妭锛岃繖绉嶆儏鍐典笅鍙兘浼氬垎閰嶄竴涓?2M 椤佃€岄潪
4K 椤碉紝鐧界櫧娴垂銆傝繖灏辨槸涓轰粈涔堝彲浠ョ郴缁熻寖鍥村唴绂佺敤澶ч〉銆佸苟鍙湪 MADV_HUGEPAGE madvise
鍖哄煙鍐呮嫢鏈夊畠浠殑鍘熷洜銆?
宓屽叆寮忕郴缁熷簲鍙湪 madvise 鍖哄煙鍐呭惎鐢ㄥぇ椤碉紝浠ユ秷闄ゆ氮璐逛换浣曞疂璐靛唴瀛樺瓧鑺傜殑椋庨櫓锛屽苟涓斿彧
杩愯寰楁洿蹇€?
浠庡ぇ椤典腑鑾风泭鑹銆佷笖涓嶄細鍥犱娇鐢ㄥぇ椤佃€岄潰涓翠涪澶卞唴瀛橀闄╃殑搴旂敤绋嬪簭锛屽簲鍦ㄥ畠浠殑鍏抽敭
mmap 鍖哄煙涓婁娇鐢?madvise(MADV_HUGEPAGE)銆?

## sysfs


### 鍏ㄥ眬 THP 鎺у埗


鐢ㄤ簬鍖垮悕鍐呭瓨鐨勯€忔槑澶ч〉鏀寔鍙互琚鐢紙涓昏鍑轰簬璋冭瘯鐩殑锛夛紝鎴栧彧鍦?MADV_HUGEPAGE
鍖哄煙鍐呭惎鐢紙浠ラ伩鍏嶆秷鑰楁洿澶氬唴瀛樿祫婧愮殑椋庨櫓锛夛紝鎴栧惎鐢?```

	echo always >/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/enabled
	echo madvise >/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/enabled
	echo never >/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/enabled

```
鍏朵腑 <size> 鏄墍閽堝鐨勫ぇ椤靛ぇ灏忥紝鍏跺彲鐢ㄥぇ灏忓洜绯荤粺鑰屽紓銆?
          閫忔槑宸ㄩ〉鍏ㄥ眬銆傝繖鏄洜涓?``madvise(..., MADV_COLLAPSE)`` 蹇界暐杩欎簺璁剧疆锛屽苟
          鏃犳潯浠跺湴灏嗚寖鍥存姌鍙犱负 PMD 灏哄鐨勫ぇ椤点€?```

	echo always >/sys/kernel/mm/transparent_hugepage/hugepages-2048kB/enabled

```
鎴栬€咃紝涔熷彲浠ユ寚瀹氱粰瀹氱殑澶ч〉澶у皬
```

	echo inherit >/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/enabled

```
```

	echo inherit >/sys/kernel/mm/transparent_hugepage/hugepages-2048kB/enabled

```
鐢ㄤ簬鈥渋nherit鈥濈殑椤跺眰璁剧疆鍙互閫氳繃鍙戝嚭浠ヤ笅鍛戒护鏉ヨ缃?```

	echo always >/sys/kernel/mm/transparent_hugepage/enabled
	echo madvise >/sys/kernel/mm/transparent_hugepage/enabled
	echo never >/sys/kernel/mm/transparent_hugepage/enabled

```
榛樿鎯呭喌涓嬶紝PMD 灏哄鐨勫ぇ椤?enabled="inherit"锛屾墍鏈夊叾瀹冨ぇ椤靛ぇ灏?enabled="never"銆?濡傛灉鍚敤澶氫釜澶ч〉澶у皬锛屽唴鏍稿皢涓虹粰瀹氱殑鍒嗛厤閫夋嫨鏈€鍚堥€傜殑宸插惎鐢ㄥぇ灏忋€?
涔熷彲鑳藉湪 VM 涓檺鍒剁鐗囨暣鐞嗭紙defrag锛夊姫鍔涳紝浠ョ敓鎴愬尶鍚嶅ぇ椤碉紝浠ラ槻瀹冧滑涓嶈兘绔嬪嵆绌洪棽
鐢ㄤ簬 madvise 鍖哄煙锛屾垨鑰呮案杩滀笉灏濊瘯鏁寸悊鍐呭瓨銆佺洿鎺ュ洖閫€鍒板父瑙勯〉锛岄櫎闈炲ぇ椤电珛鍗冲彲鐢ㄣ€?鏄剧劧锛屽鏋滄垜浠姳璐?CPU 鏃堕棿鏉ユ暣鐞嗗唴瀛橈紝鎴戜滑浼氭湡鏈涚敱浜庝箣鍚庝娇鐢ㄥぇ椤佃€岄潪甯歌椤佃€岃幏寰?鏇村鏀剁泭銆傝繖骞朵笉鎬绘槸鏈変繚璇佺殑锛屼絾鍦ㄥ垎閰嶆槸闈㈠悜 MADV_HUGEPAGE 鍖哄煙鐨勬儏鍐典笅鏇存湁鍙兘銆?```

	echo always >/sys/kernel/mm/transparent_hugepage/defrag
	echo defer >/sys/kernel/mm/transparent_hugepage/defrag
	echo defer+madvise >/sys/kernel/mm/transparent_hugepage/defrag
	echo madvise >/sys/kernel/mm/transparent_hugepage/defrag
	echo never >/sys/kernel/mm/transparent_hugepage/defrag

```
always
	琛ㄧず璇锋眰 THP 鐨勫簲鐢ㄧ▼搴忓皢鍦ㄥ垎閰嶅け璐ユ椂鍋滄粸锛屽苟鐩存帴鍥炴敹椤点€佸帇缂╁唴瀛橈紝
	浠ュ姫鍔涚珛鍗冲垎閰嶄竴涓?THP銆傝繖瀵逛簬浠?THP 浣跨敤涓幏鐩婂尓娴呫€佸苟鎰挎剰寤惰繜铏氭嫙鏈?	鍚姩鏉ュ埄鐢ㄥ畠浠殑铏氭嫙鏈哄彲鑳芥槸鐞嗘兂鐨勩€?
defer
	琛ㄧず搴旂敤绋嬪簭灏嗗湪鍚庡彴鍞ら啋 kswapd 鏉ュ洖鏀堕〉銆佸敜閱?kcompactd 鏉ュ帇缂╁唴瀛橈紝
	浠ヤ究 THP 鍦ㄤ笉涔呯殑灏嗘潵鍙敤銆備箣鍚庣敱 khugepaged 璐熻矗绋嶅悗瀹夎 THP 椤点€?
defer+madvise
	灏嗗儚 `always` 涓€鏍疯繘鍏ョ洿鎺ュ洖鏀跺拰鍘嬬缉锛屼絾浠呴拡瀵逛娇鐢ㄤ簡
	madvise(MADV_HUGEPAGE) 鐨勫尯鍩燂紱鎵€鏈夊叾瀹冨尯鍩熷皢鍦ㄥ悗鍙板敜閱?kswapd 鍥炴敹
	椤点€佸敜閱?kcompactd 鍘嬬缉鍐呭瓨锛屼互渚?THP 鍦ㄤ笉涔呯殑灏嗘潵鍙敤銆?
madvise
	灏嗗儚 `always` 涓€鏍疯繘鍏ョ洿鎺ュ洖鏀讹紝浣嗕粎閽堝浣跨敤浜?	madvise(MADV_HUGEPAGE) 鐨勫尯鍩熴€傝繖鏄粯璁よ涓恒€?
never
	搴旇涓嶈█鑷槑銆傛敞鎰忥紝鍗充娇鍒板閮芥寚瀹氫簡姝ゆā寮忥紝``madvise(..., MADV_COLLAPSE)``
	浠嶅彲鑳藉鑷磋幏寰楅€忔槑澶ч〉銆?
榛樿鎯呭喌涓嬶紝鍐呮牳鍦ㄥ尶鍚嶆槧灏勭殑璇婚〉閿欒鏃跺皾璇曚娇鐢ㄥ法澶х殑銆丳MD 鍙槧灏勭殑闆堕〉銆傚彲浠?绂佺敤宸ㄥぇ闆堕〉
```

	echo 0 >/sys/kernel/mm/transparent_hugepage/use_zero_page
	echo 1 >/sys/kernel/mm/transparent_hugepage/use_zero_page

```
鏌愪簺鐢ㄦ埛绌洪棿锛堜緥濡傛祴璇曠▼搴忥紝鎴栦紭鍖栬繃鐨勫唴瀛樺垎閰嶅簱锛夊彲鑳芥兂鐭ラ亾锛堜互瀛楄妭涓哄崟浣嶇殑锛?澶у皬
```

	cat /sys/kernel/mm/transparent_hugepage/hpage_pmd_size

```
鏁呴殰鍜屾姌鍙犳椂鐨勬墍鏈?THP 閮戒細琚姞鍏?_deferred_list锛屽洜姝ゅ鏋滃畠浠瑙嗕负鈥滄湭鍏呭垎鍒╃敤鈥?锛坲nderused锛夛紝灏嗗湪鍐呭瓨鍘嬪姏涓嬭鎷嗗垎銆傚鏋滀竴涓?THP 涓浂濉厖椤电殑鏁伴噺瓒呰繃
max_ptes_none锛堣涓嬫枃锛夛紝璇?THP 灏辨槸鏈厖鍒嗗埄鐢ㄧ殑銆傚彲浠ラ€氳繃鍚?shrink_underused 鍐欏叆
0 鏉ョ鐢ㄦ琛屼负锛屽啓鍏?1 鏉ュ惎鐢ㄥ畠
```

	echo 0 > /sys/kernel/mm/transparent_hugepage/shrink_underused
	echo 1 > /sys/kernel/mm/transparent_hugepage/shrink_underused

```
褰?PMD 灏哄鐨?THP 琚惎鐢ㄦ椂锛坧er-size anon 鎺у埗鎴栭《灞傛帶鍒朵箣涓€琚涓衡€渁lways鈥濇垨
鈥渕advise鈥濓級锛宬hugepaged 浼氳嚜鍔ㄥ惎鍔紱褰?PMD 灏哄鐨?THP 琚鐢ㄦ椂锛坧er-size anon 鎺у埗
鍜岄《灞傛帶鍒堕兘涓衡€渘ever鈥濓級锛屽畠浼氳嚜鍔ㄥ叧闂€?
### 杩涚▼绾?THP 鎺у埗


涓€涓繘绋嬪彲浠ヤ娇鐢?`PR_SET_THP_DISABLE` 鍜?`PR_GET_THP_DISABLE` 杩欏 prctl(2) 璋冪敤鏉?鎺у埗鑷繁鐨?THP 琛屼负銆備娇鐢?`PR_SET_THP_DISABLE` 璁剧疆鐨?THP 琛屼负浼氳法 fork(2) 鍜?execve(2) 缁ф壙銆傝繖浜涜皟鐢?```

	prctl(PR_SET_THP_DISABLE, 1, 0, 0, 0):
		This will disable THPs completely for the process, irrespective
		of global THP controls or madvise(..., MADV_COLLAPSE) being used.

	prctl(PR_SET_THP_DISABLE, 1, PR_THP_DISABLE_EXCEPT_ADVISED, 0, 0):
		This will disable THPs for the process except when the usage of THPs is
		advised. Consequently, THPs will only be used when:
		- Global THP controls are set to "always" or "madvise" and
		  madvise(..., MADV_HUGEPAGE) or madvise(..., MADV_COLLAPSE) is used.
		- Global THP controls are set to "never" and madvise(..., MADV_COLLAPSE)
		  is used. This is the same behavior as if THPs would not be disabled on
		  a process level.
		Note that MADV_COLLAPSE is currently always rejected if
		madvise(..., MADV_NOHUGEPAGE) is set on an area.

	prctl(PR_SET_THP_DISABLE, 0, 0, 0, 0):
		This will re-enable THPs for the process, as if they were never disabled.
		Whether THPs will actually be used depends on global THP controls and
		madvise() calls.

	prctl(PR_GET_THP_DISABLE, 0, 0, 0, 0):
		This returns a value whose bits indicate how THP-disable is configured:
		Bits
		 1 0  Value  Description
		|0|0|   0    No THP-disable behaviour specified.
		|0|1|   1    THP is entirely disabled for this process.
		|1|1|   3    THP-except-advised mode is set for this process.

```
### Khugepaged 鎺у埗


   khugepaged 鐩墠鍙悳绱㈡姌鍙犱负 PMD 灏哄 THP 鐨勬満浼氾紝涓嶄細灏濊瘯鎶樺彔涓哄叾瀹?THP
   灏哄銆?
khugepaged 閫氬父浠ヨ緝浣庨鐜囪繍琛岋紝鍥犳铏界劧鍙兘涓嶆兂鍦ㄩ〉閿欒鏈熼棿鍚屾璋冪敤纰庣墖鏁寸悊
绠楁硶锛屼絾鍦?khugepaged 涓嚦灏戣皟鐢ㄤ竴娆＄鐗囨暣鐞嗘槸鍊煎緱鐨勩€備笉杩囦篃鍙互閫氳繃鍐欏叆 0 绂佺敤
khugepaged 涓殑纰庣墖鏁寸悊锛屾垨鍐欏叆 1 鍚敤
```

	echo 0 >/sys/kernel/mm/transparent_hugepage/khugepaged/defrag
	echo 1 >/sys/kernel/mm/transparent_hugepage/khugepaged/defrag

```
浣犱篃鍙互鎺у埗 khugepaged 姣忔鎵弿搴旀壂鎻忓灏戦〉
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/pages_to_scan

```
浠ュ強 khugepaged 鍦ㄦ瘡杞箣闂寸瓑寰呭灏戞绉掞紙浣?```

	/sys/kernel/mm/transparent_hugepage/khugepaged/scan_sleep_millisecs

```
浠ュ強濡傛灉鏈変竴涓ぇ椤碉紝khugepaged 绛夊緟澶氬皯姣
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/alloc_sleep_millisecs

```
khugepaged 鐨勮繘搴﹀彲浠ヤ粠宸叉姌鍙犻〉鐨勬暟閲忕湅鍑猴紙娉ㄦ剰锛岃繖涓鏁板櫒鍙兘涓嶆槸宸叉姌鍙犻〉鏁伴噺鐨?绮剧‘璁℃暟锛屽洜涓衡€滃凡鎶樺彔鈥濆彲鑳芥湁澶氱鍚箟锛?1) PTE 鏄犲皠琚?PMD 鏄犲皠鏇挎崲锛屾垨 (2) 鎵€鏈?4K
鐗╃悊椤佃涓€涓?2M 澶ч〉鏇挎崲銆傛瘡绉嶆儏鍐靛彲鑳界嫭绔嬪彂鐢燂紝涔熷彲鑳戒竴璧峰彂鐢燂紝鍙栧喅浜庡唴瀛樼被鍨嬪拰
鍙戠敓鐨勫け璐ャ€傚洜姝わ紝杩欎釜鍊煎簲澶ц嚧瑙ｉ噴涓鸿繘搴︾殑鏍囧織锛岃€?/proc/vmstat 涓殑璁℃暟鍣?```

	/sys/kernel/mm/transparent_hugepage/khugepaged/pages_collapsed

```
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/full_scans

```
`max_ptes_none` 鎸囧畾鍦ㄦ姌鍙犱竴缁勯〉鏃跺彲浠ュ垎閰嶅灏戜釜棰濆鐨勶紙灏氭湭鏄犲皠鐨勶級灏忛〉
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/max_ptes_none

```
杈冮珮鐨勫€间細瀵艰嚧涓虹▼搴忎娇鐢ㄩ澶栧唴瀛樸€傝緝浣庣殑鍊间細瀵艰嚧鑾峰緱鐨?thp 鎬ц兘鏇村皯銆?max_ptes_none 鐨勫€兼氮璐圭殑 cpu 鏃堕棿鏋佸皯锛屽彲浠ュ拷鐣ュ畠銆?
`max_ptes_swap` 鎸囧畾鍙互浠庝互涓嬩綅缃紩鍏ュ灏戦〉
```

	/sys/kernel/mm/transparent_hugepage/khugepaged/max_ptes_swap

```
杈冮珮鐨勫€煎彲鑳藉鑷磋繃搴︾殑浜ゆ崲 IO 骞舵氮璐瑰唴瀛樸€傝緝浣庣殑鍊煎彲鑳介樆姝?THP 琚姌鍙狅紝瀵艰嚧鎶樺彔
涓?THP 鐨勯〉鏇村皯锛屼互鍙婃洿浣庣殑鍐呭瓨璁块棶鎬ц兘銆?
`max_ptes_shared` 鎸囧畾鍙互璺ㄥ涓繘绋嬪叡浜灏戦〉銆傚鏋?THP 鐨勪换浣曚竴椤?```

	/sys/kernel/mm/transparent_hugepage/khugepaged/max_ptes_shared

```
杈冮珮鐨勫€煎彲鑳戒細澧炲姞鏌愪簺宸ヤ綔璐熻浇鐨勫唴瀛樺崰鐢ㄣ€?
## 鍚姩鍙傛暟


浣犲彲浠ラ€氳繃灏嗗弬鏁?`transparent_hugepage=always` 鎴?`transparent_hugepage=madvise` 鎴?`transparent_hugepage=never` 浼犵粰鍐呮牳鍛戒护琛岋紝鏉ユ洿鏀归《灞傗€渆nabled鈥濇帶鍒剁殑 sysfs 鍚姩
鏃堕棿榛樿鍊笺€?
鍙﹀锛屾瘡涓彈鏀寔鐨勫尶鍚?THP 澶у皬閮藉彲浠ラ€氳繃浼犻€?`thp_anon=<size>[KMG],<size>[KMG]:<state>;<size>[KMG]-<size>[KMG]:<state>` 鏉ユ帶鍒讹紝
鍏朵腑 `<size>` 鏄?THP 澶у皬锛堝繀椤绘槸 PAGE_SIZE 鐨?2 鐨勫箓锛屼笖涓哄彈鏀寔鐨勫尶鍚?THP锛夛紝
`<state>` 鏄?`always`銆乣madvise`銆乣never` 鎴?`inherit` 涔嬩竴銆?
渚嬪锛屼互涓嬪皢鎶?16K銆?2K銆?4K THP 璁句负 `always`锛屾妸 128K銆?12K 璁句负 `inherit`锛?鎶?256K 璁句负 `madvise`锛屾妸 1M銆?M
```

	thp_anon=16K-64K:always;128K,512K:inherit;256K:madvise;1M-2M:never

```
`thp_anon=` 鍙互澶氭鎸囧畾锛屼互鎸夐渶閰嶇疆鎵€鏈?THP 澶у皬銆傚鏋滆嚦灏戞寚瀹氫簡涓€娆?`thp_anon=`锛?鍒欏懡浠よ涓婃湭鏄惧紡閰嶇疆鐨勪换浣曞尶鍚?THP 澶у皬閮介殣寮忚涓?`never`銆?
`transparent_hugepage` 璁剧疆鍙奖鍝嶅叏灞€寮€鍏炽€傚鏋滄湭鎸囧畾 `thp_anon`锛孭MD_ORDER THP 灏?榛樿涓?`inherit`銆備絾鏄紝濡傛灉鐢ㄦ埛鎻愪緵浜嗘湁鏁堢殑 `thp_anon` 璁剧疆锛孭MD_ORDER THP 绛栫暐灏嗚
瑕嗙洊銆傚鏋?PMD_ORDER 鐨勭瓥鐣ユ湭鍦ㄦ煇涓湁鏁堢殑 `thp_anon` 涓畾涔夛紝鍏剁瓥鐣ュ皢榛樿涓?`never`銆?
涓?`transparent_hugepage` 绫讳技锛屼綘鍙互浣跨敤鍐呮牳鍙傛暟
`transparent_hugepage_shmem=<policy>` 鎺у埗鍐呴儴 shmem 鎸傝浇鐨勫ぇ椤靛垎閰嶇瓥鐣ワ紝鍏朵腑
`<policy>` 鏄?shmem 鐨勪竷涓湁鏁堢瓥鐣ヤ箣涓€锛坄always`銆乣within_size`銆乣advise`銆乣never`銆?`deny` 鍜?`force`锛夈€?
涓?`transparent_hugepage_shmem` 绫讳技锛屼綘鍙互浣跨敤鍐呮牳鍙傛暟
`transparent_hugepage_tmpfs=<policy>` 鎺у埗 tmpfs 鎸傝浇鐨勯粯璁ゅぇ椤靛垎閰嶇瓥鐣ワ紝鍏朵腑
`<policy>` 鏄?tmpfs 鐨勫洓涓湁鏁堢瓥鐣ヤ箣涓€锛坄always`銆乣within_size`銆乣advise`銆乣never`锛夈€?tmpfs 鎸傝浇鐨勯粯璁ょ瓥鐣ユ槸 `never`銆?
姝ゅ锛孠config 閫夐」鍙敤浜庡湪鏋勫缓鏃惰缃?shmem 鐨勯粯璁ゅぇ椤电瓥鐣?锛坄CONFIG_TRANSPARENT_HUGEPAGE_SHMEM_HUGE_*`锛夊拰 tmpfs 鐨勯粯璁ゅぇ椤电瓥鐣?锛坄CONFIG_TRANSPARENT_HUGEPAGE_TMPFS_HUGE_*`锛夈€傛洿澶氱粏鑺傝鍙傞槄 Kconfig 甯姪銆?
涓?`thp_anon` 鎺у埗姣忎釜鍙楁敮鎸佺殑鍖垮悕 THP 澶у皬涓€鏍凤紝`thp_shmem` 鎺у埗姣忎釜鍙楁敮鎸佺殑 shmem
THP 澶у皬銆俙thp_shmem` 涓?`thp_anon` 鏍煎紡鐩稿悓锛屼絾涔熸敮鎸?`within_size` 绛栫暐銆?
`thp_shmem=` 鍙互澶氭鎸囧畾锛屼互鎸夐渶閰嶇疆鎵€鏈?THP 澶у皬銆傚鏋滆嚦灏戞寚瀹氫簡涓€娆?`thp_shmem=`锛?鍒欏懡浠よ涓婃湭鏄惧紡閰嶇疆鐨勪换浣?shmem THP 澶у皬閮介殣寮忚涓?`never`銆?
`transparent_hugepage_shmem` 璁剧疆鍙奖鍝嶅叏灞€寮€鍏炽€傚鏋滄湭鎸囧畾 `thp_shmem`锛?PMD_ORDER 澶ч〉灏嗛粯璁や负 `inherit`銆備絾鏄紝濡傛灉鐢ㄦ埛鎻愪緵浜嗘湁鏁堢殑 `thp_shmem` 璁剧疆锛?PMD_ORDER 澶ч〉绛栫暐灏嗚瑕嗙洊銆傚鏋?PMD_ORDER 鐨勭瓥鐣ユ湭鍦ㄦ煇涓湁鏁堢殑 `thp_shmem` 涓?瀹氫箟锛屽叾绛栫暐灏嗛粯璁や负 `never`銆?
## tmpfs/shmem 涓殑澶ч〉


浼犵粺涓婏紝tmpfs 鍙敮鎸佸崟涓€鐨勫ぇ椤靛ぇ灏忥紙鈥淧MD鈥濓級銆傚浠婏紝瀹冧篃鍍忓尶鍚嶅唴瀛樹竴鏍锋敮鎸佹洿灏忕殑
澶у皬锛岄€氬父琚О涓衡€渕ulti-size THP鈥濓紙mTHP锛屽灏哄 THP锛夈€備换浣曞ぇ灏忕殑澶ч〉鍦ㄥ唴鏍镐腑閫氬父
琛ㄧず涓衡€渓arge folios鈥濓紙澶?folio锛夈€?
铏界劧瀵圭敤浜庡唴閮?shmem 鎸傝浇浣跨敤鐨勫ぇ椤靛ぇ灏忔湁绮剧粏鎺у埗锛堣涓嬫枃锛夛紝浣嗘櫘閫氱殑 tmpfs 鎸傝浇
浼氬埄鐢ㄦ墍鏈夊彲鐢ㄧ殑澶ч〉澶у皬锛岃€屾棤闇€瀵圭‘鍒囧ぇ灏忚繘琛屾帶鍒讹紝琛ㄧ幇寰楁洿鍍忓叾瀹冩枃浠剁郴缁熴€?
### tmpfs 鎸傝浇


tmpfs 鎸傝浇鐨?THP 鍒嗛厤绛栫暐鍙互浣跨敤鎸傝浇閫夐」锛歚huge=` 鏉ヨ皟鏁淬€傚畠鍙互鏈変互涓嬪彇鍊硷細

always
    姣忔闇€瑕佹柊椤垫椂閮藉皾璇曞垎閰嶅ぇ椤碉紱
    鎬绘槸鍏堝皾璇?PMD 灏哄鐨勫ぇ椤碉紝濡傛灉 PMD 灏哄鐨勫ぇ椤靛垎閰嶅け璐ワ紝鍒欏洖閫€鍒版洿灏忓昂瀵哥殑
    澶ч〉锛?
never
    涓嶅垎閰嶅ぇ椤点€傛敞鎰忥紝鍗充娇鍒板閮芥寚瀹氫簡姝ゆā寮忥紝`madvise(..., MADV_COLLAPSE)` 浠嶅彲鑳?    瀵艰嚧鑾峰緱閫忔槑澶ч〉锛?
within_size
    鍙湁褰撳ぇ椤靛皢瀹屽叏浣嶄簬 i_size 鍐呮椂鎵嶅垎閰嶏紱
    鎬绘槸鍏堝皾璇?PMD 灏哄鐨勫ぇ椤碉紝濡傛灉 PMD 灏哄鐨勫ぇ椤靛垎閰嶅け璐ワ紝鍒欏洖閫€鍒版洿灏忓昂瀵哥殑
    澶ч〉锛?    涔熷皧閲?madvise() 鎻愮ず锛?
advise
    鍙湁鍦ㄤ娇鐢?madvise() 璇锋眰鏃舵墠鍒嗛厤澶ч〉锛?
璇疯浣忥紝鍐呮牳鍙兘浣跨敤鎵€鏈夊彲鐢ㄥぇ灏忕殑澶ч〉锛屽苟涓旀棤娉曞儚鍐呴儴 tmpfs 鎸傝浇閭ｆ牱杩涜绮剧粏鎺у埗銆?
杩囧幓榛樿绛栫暐鏄?`never`锛屼絾鐜板湪鍙互浣跨敤鍐呮牳鍙傛暟 `transparent_hugepage_tmpfs=<policy>`
鏉ヨ皟鏁淬€?
`mount -o remount,huge= /mountpoint` 鍦ㄦ寕杞藉悗宸ヤ綔姝ｅ父锛氶噸鏂版寕杞?`huge=never` 鏍规湰涓嶄細
灏濊瘯鎷嗗垎澶ч〉锛屽彧鏄仠姝㈠垎閰嶆洿澶氬ぇ椤点€?
闄や簡涓婇潰鍒楀嚭鐨勭瓥鐣ュ锛屽綋璁句负浠ヤ笅鍊兼椂锛宻ysfs 鏃嬮挳
/sys/kernel/mm/transparent_hugepage/shmem_enabled 浼氬奖鍝?tmpfs 鎸傝浇鐨勫垎閰嶇瓥鐣ワ細

deny
    鐢ㄤ簬绱ф€ユ儏鍐碉紝寮哄埗浠庢墍鏈夋寕杞藉叧闂?huge 閫夐」锛?
force
    瀵规墍鏈夋寕杞藉己鍒跺紑鍚?huge 閫夐」鈥斺€斿娴嬭瘯闈炲父鏈夌敤锛?
### shmem / 鍐呴儴 tmpfs


鍐呴儴 tmpfs 鎸傝浇鐢ㄤ簬 SysV SHM銆乵emfds銆佸叡浜尶鍚?mmap锛?dev/zero 鎴?MAP_ANONYMOUS锛夈€?GPU 椹卞姩鐨?DRM 瀵硅薄銆丄shmem銆?
瑕佹帶鍒舵鍐呴儴 tmpfs 鎸傝浇鐨?THP 鍒嗛厤绛栫暐锛屽彲浠ヤ娇鐢?sysfs 鏃嬮挳
/sys/kernel/mm/transparent_hugepage/shmem_enabled锛屼互鍙?'/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/shmem_enabled' 涓瘡涓?THP 澶у皬鐨勬棆閽€?
鍏ㄥ眬鏃嬮挳鐨勮涔変笌 tmpfs 鎸傝浇鐨?`huge=` 鎸傝浇閫夐」鐩稿悓锛屼笉鍚屼箣澶勫湪浜庡彲浠ュ崟鐙帶鍒朵笉鍚?鐨勫ぇ椤靛ぇ灏忥紝骞朵笖鍙湁褰?per-size 鏃嬮挳璁句负 'inherit' 鏃舵墠浼氫娇鐢ㄥ叏灞€鏃嬮挳鐨勮缃€?
鍚勪釜澶у皬鐨?'force' 鍜?'deny' 閫夐」宸茶鍘绘帀锛屽畠浠槸鏃ф椂浠ｇ殑娴嬭瘯浜х墿銆?
always
    姣忔闇€瑕佹柊椤垫椂閮藉皾璇曞垎閰?<size> 澶ч〉锛?
inherit
    缁ф壙椤跺眰鐨?"shmem_enabled" 鍊笺€傞粯璁ゆ儏鍐典笅锛孭MD 灏哄鐨勫ぇ椤?    enabled="inherit"锛屾墍鏈夊叾瀹冨ぇ椤靛ぇ灏?enabled="never"锛?
never
    涓嶅垎閰?<size> 澶ч〉銆傛敞鎰忥紝鍗充娇鍒板閮芥寚瀹氫簡姝ゆā寮忥紝``madvise(...,
    MADV_COLLAPSE)`` 浠嶅彲鑳藉鑷磋幏寰楅€忔槑澶ч〉锛?
within_size
    鍙湁褰?<size> 澶ч〉灏嗗畬鍏ㄤ綅浜?i_size 鍐呮椂鎵嶅垎閰嶃€?    涔熷皧閲?madvise() 鎻愮ず锛?
advise
    鍙湁鍦ㄤ娇鐢?madvise() 璇锋眰鏃舵墠鍒嗛厤 <size> 澶ч〉锛?
## 闇€瑕侀噸鍚簲鐢ㄧ▼搴?

transparent_hugepage/enabled 鍜?transparent_hugepage/hugepages-<size>kB/enabled 鍊?浠ュ強 tmpfs 鎸傝浇閫夐」鍙奖鍝嶆湭鏉ョ殑琛屼负銆傚洜姝わ紝瑕佷娇瀹冧滑鐢熸晥锛屼綘闇€瑕侀噸鍚换浣曞彲鑳戒竴鐩村湪
浣跨敤澶ч〉鐨勫簲鐢ㄧ▼搴忋€傝繖涔熼€傜敤浜庡湪 khugepaged 涓敞鍐岀殑鍖哄煙銆?
## 鐩戣浣跨敤鎯呭喌


绯荤粺褰撳墠浣跨敤鐨?PMD 灏哄鍖垮悕閫忔槑澶ч〉鏁伴噺锛屽彲閫氳繃璇诲彇 `/proc/meminfo` 涓殑 AnonHugePages
瀛楁鑾峰緱銆傝璇嗗埆鍝簺搴旂敤绋嬪簭姝ｅ湪浣跨敤 PMD 灏哄鍖垮悕閫忔槑澶ч〉锛岄渶瑕佽鍙?`/proc/PID/smaps`
骞跺姣忎釜鏄犲皠鐨?AnonHugePages 瀛楁璁℃暟銆傦紙娉ㄦ剰锛屽嚭浜庡巻鍙插師鍥狅紝AnonHugePages 鍙€傜敤浜?浼犵粺鐨?PMD 灏哄 THP锛屾湰搴旇绉颁负 AnonHugePmdMapped锛夈€?
鏄犲皠鍒扮敤鎴风┖闂寸殑鏂囦欢閫忔槑澶ч〉鏁伴噺锛屽彲閫氳繃璇诲彇 `/proc/meminfo` 涓殑 ShmemPmdMapped 鍜?ShmemHugePages 瀛楁鑾峰緱銆傝璇嗗埆鍝簺搴旂敤绋嬪簭姝ｅ湪鏄犲皠鏂囦欢閫忔槑澶ч〉锛岄渶瑕佽鍙?`/proc/PID/smaps` 骞跺姣忎釜鏄犲皠鐨?FilePmdMapped 瀛楁璁℃暟銆?
娉ㄦ剰锛岃鍙?smaps 鏂囦欢寮€閿€寰堝ぇ锛岄绻佽鍙栦細甯︽潵寮€閿€銆?
`/proc/vmstat` 涓湁涓€浜涜鏁板櫒锛屽彲鐢ㄤ簬鐩戣绯荤粺鎻愪緵澶ч〉渚涗娇鐢ㄧ殑鎴愬姛绋嬪害銆?
thp_fault_alloc
	姣忔鎴愬姛鍒嗛厤涓€涓ぇ椤靛苟璁″叆锛坈harge锛変互澶勭悊椤甸敊璇椂閫掑銆?
thp_collapse_alloc
	褰?khugepaged 鎵惧埌涓€娈靛簲鎶樺彔涓轰竴涓ぇ椤电殑椤佃寖鍥淬€佸苟鎴愬姛鍒嗛厤涓€涓柊澶ч〉
	鏉ュ瓨鍌ㄦ暟鎹椂閫掑銆?
thp_fault_fallback
	濡傛灉椤甸敊璇湭鑳藉垎閰嶆垨璁″叆涓€涓ぇ椤碉紝鑰屾槸鍥為€€鍒颁娇鐢ㄥ皬椤碉紝鍒欓€掑銆?
thp_fault_fallback_charge
	濡傛灉椤甸敊璇湭鑳借鍏ヤ竴涓ぇ椤碉紝鑰屾槸鍥為€€鍒颁娇鐢ㄥ皬椤碉紙鍗充娇鍒嗛厤鎴愬姛锛夛紝鍒欓€掑銆?
thp_collapse_alloc_failed
	濡傛灉 khugepaged 鎵惧埌涓€娈靛簲鎶樺彔涓轰竴涓ぇ椤电殑椤佃寖鍥翠絾鍒嗛厤澶辫触锛屽垯閫掑銆?
thp_file_alloc
	姣忔鎴愬姛鍒嗛厤涓€涓?shmem 澶ч〉鏃堕€掑锛堟敞鎰忥紝灏界浠モ€渇ile鈥濆懡鍚嶏紝璇ヨ鏁板櫒
	鍙閲?shmem锛夈€?
thp_file_fallback
	濡傛灉灏濊瘯鍒嗛厤涓€涓?shmem 澶ч〉浣嗗け璐ャ€佽€屾槸鍥為€€鍒颁娇鐢ㄥ皬椤碉紝鍒欓€掑銆傦紙娉ㄦ剰锛?	灏界浠モ€渇ile鈥濆懡鍚嶏紝璇ヨ鏁板櫒鍙閲?shmem锛夈€?
thp_file_fallback_charge
	濡傛灉涓€涓?shmem 澶ч〉鏃犳硶璁″叆銆佽€屾槸鍥為€€鍒颁娇鐢ㄥ皬椤碉紙鍗充娇鍒嗛厤鎴愬姛锛夛紝鍒欓€掑銆?	锛堟敞鎰忥紝灏界浠モ€渇ile鈥濆懡鍚嶏紝璇ヨ鏁板櫒鍙閲?shmem锛夈€?
thp_file_mapped
	姣忔涓€涓枃浠舵垨 shmem 澶ч〉琚槧灏勮繘鐢ㄦ埛鍦板潃绌洪棿鏃堕€掑銆?
thp_split_page
	姣忔涓€涓ぇ椤佃鎷嗗垎涓哄熀鏈〉鏃堕€掑銆傝繖鍙兘鐢变簬澶氱鍘熷洜鍙戠敓锛屼絾涓€涓父瑙佺殑
	鍘熷洜鏄ぇ椤靛凡鏃у苟姝ｅ湪琚洖鏀躲€傝繖涓姩浣滄剰鍛崇潃鎷嗗垎璇ラ〉鏄犲皠鐨勬墍鏈?PMD銆?
thp_split_page_failed
	濡傛灉鍐呮牳鏈兘鎷嗗垎澶ч〉锛屽垯閫掑銆傝繖鍙兘鍙戠敓鍦ㄨ椤佃鏌愪汉鍥哄畾锛坧in锛夋椂銆?
thp_deferred_split_page
	褰撲竴涓ぇ椤佃鏀惧叆鎷嗗垎闃熷垪鏃堕€掑銆傝繖鍙戠敓鍦ㄥぇ椤佃閮ㄥ垎鍙栨秷鏄犲皠銆佹媶鍒嗗畠灏?	閲婃斁涓€浜涘唴瀛樻椂銆傛媶鍒嗛槦鍒椾笂鐨勯〉灏嗗湪鍐呭瓨鍘嬪姏涓嬭鎷嗗垎銆?
thp_underused_split_page
	褰撴媶鍒嗛槦鍒椾笂鐨勪竴涓ぇ椤靛洜鍏舵湭鍏呭垎鍒╃敤鑰岃鎷嗗垎鏃堕€掑銆傚鏋滀竴涓?THP 涓殑
	闆堕〉鏁伴噺瓒呰繃鏌愪釜闃堝€?	锛?sys/kernel/mm/transparent_hugepage/khugepaged/max_ptes_none锛夛紝璇?THP 灏辨槸
	鏈厖鍒嗗埄鐢ㄧ殑銆?
thp_split_pmd
	姣忔涓€涓?PMD 琚媶鍒嗕负 PTE 琛ㄦ椂閫掑銆備緥濡傦紝褰撳簲鐢ㄧ▼搴忓澶ч〉鐨勪竴閮ㄥ垎璋冪敤
	mprotect() 鎴?munmap() 鏃跺彲鑳藉彂鐢熴€傚畠涓嶄細鎷嗗垎澶ч〉锛屽彧鎷嗗垎椤佃〃椤广€?
thp_zero_page_alloc
	姣忔鎴愬姛鍒嗛厤涓€涓敤浜?thp 鐨勫法澶ч浂椤垫椂閫掑銆傛敞鎰忥紝瀹冧笉璁℃暟宸ㄥぇ闆堕〉鐨勬瘡娆?	鏄犲皠锛屽彧璁℃暟鍏跺垎閰嶃€?
thp_zero_page_alloc_failed
	濡傛灉鍐呮牳鏈兘鍒嗛厤宸ㄥぇ闆堕〉銆佸苟鍥為€€鍒颁娇鐢ㄥ皬椤碉紝鍒欓€掑銆?
thp_swpout
	姣忔涓€涓ぇ椤靛湪涓嶆媶鍒嗙殑鎯呭喌涓嬫暣浣撲氦鎹㈠嚭鍘伙紙swapout锛夋椂閫掑銆?
thp_swpout_fallback
	濡傛灉涓€涓ぇ椤靛繀椤诲湪浜ゆ崲鍑哄幓涔嬪墠鎷嗗垎锛屽垯閫掑銆傞€氬父鏄洜涓烘湭鑳戒负璇ュぇ椤靛垎閰?	鏌愪簺杩炵画鐨勪氦鎹㈢┖闂淬€?
鍦?/sys/kernel/mm/transparent_hugepage/hugepages-<size>kB/stats 涓紝杩樻湁閽堝姣忎釜
澶ч〉澶у皬鐨勭嫭绔嬭鏁板櫒锛屽彲鐢ㄤ簬鐩戣绯荤粺鎻愪緵澶ч〉渚涗娇鐢ㄧ殑鏈夋晥鎬с€傛瘡涓鏁板櫒閮芥湁鍏跺搴旂殑
鏂囦欢銆?
anon_fault_alloc
	姣忔鎴愬姛鍒嗛厤涓€涓ぇ椤靛苟璁″叆浠ュ鐞嗛〉閿欒鏃堕€掑銆?
anon_fault_fallback
	濡傛灉椤甸敊璇湭鑳藉垎閰嶆垨璁″叆涓€涓ぇ椤碉紝鑰屾槸鍥為€€鍒颁娇鐢ㄦ洿浣庨樁鐨勫ぇ椤垫垨灏忛〉锛屽垯閫掑銆?
anon_fault_fallback_charge
	濡傛灉椤甸敊璇湭鑳借鍏ヤ竴涓ぇ椤碉紝鑰屾槸鍥為€€鍒颁娇鐢ㄦ洿浣庨樁鐨勫ぇ椤垫垨灏忛〉锛堝嵆浣垮垎閰?	鎴愬姛锛夛紝鍒欓€掑銆?
zswpout
	姣忔涓€涓ぇ椤靛湪涓嶆媶鍒嗙殑鎯呭喌涓嬫暣浣撲氦鎹㈠埌 zswap 鏃堕€掑銆?
swpin
	姣忔涓€涓ぇ椤靛湪涓嶆媶鍒嗙殑鎯呭喌涓嬩粠闈?zswap 浜ゆ崲璁惧鏁翠綋鎹㈠叆锛坰wapin锛夋椂閫掑銆?
swpin_fallback
	濡傛灉鎹㈠叆鏈兘鍒嗛厤鎴栬鍏ヤ竴涓ぇ椤碉紝鑰屾槸鍥為€€鍒颁娇鐢ㄦ洿浣庨樁鐨勫ぇ椤垫垨灏忛〉锛屽垯閫掑銆?
swpin_fallback_charge
	濡傛灉鎹㈠叆鏈兘璁″叆涓€涓ぇ椤碉紝鑰屾槸鍥為€€鍒颁娇鐢ㄦ洿浣庨樁鐨勫ぇ椤垫垨灏忛〉锛堝嵆浣垮垎閰?	鎴愬姛锛夛紝鍒欓€掑銆?
swpout
	姣忔涓€涓ぇ椤靛湪涓嶆媶鍒嗙殑鎯呭喌涓嬫暣浣撲氦鎹㈠埌闈?zswap 浜ゆ崲璁惧鏃堕€掑銆?
swpout_fallback
	濡傛灉涓€涓ぇ椤靛繀椤诲湪浜ゆ崲鍑哄幓涔嬪墠鎷嗗垎锛屽垯閫掑銆傞€氬父鏄洜涓烘湭鑳戒负璇ュぇ椤靛垎閰?	鏌愪簺杩炵画鐨勪氦鎹㈢┖闂淬€?
shmem_alloc
	姣忔鎴愬姛鍒嗛厤涓€涓?shmem 澶ч〉鏃堕€掑銆?
shmem_fallback
	濡傛灉灏濊瘯鍒嗛厤涓€涓?shmem 澶ч〉浣嗗け璐ャ€佽€屾槸鍥為€€鍒颁娇鐢ㄥ皬椤碉紝鍒欓€掑銆?
shmem_fallback_charge
	濡傛灉涓€涓?shmem 澶ч〉鏃犳硶璁″叆銆佽€屾槸鍥為€€鍒颁娇鐢ㄥ皬椤碉紙鍗充娇鍒嗛厤鎴愬姛锛夛紝鍒欓€掑銆?
split
	姣忔涓€涓ぇ椤垫垚鍔熸媶鍒嗕负鏇村皬鐨勯樁锛坥rder锛夋椂閫掑銆傝繖鍙兘鐢变簬澶氱鍘熷洜鍙戠敓锛屼絾
	涓€涓父瑙佺殑鐞嗙敱鏄ぇ鐨勯〉宸叉棫骞舵鍦ㄨ鍥炴敹銆?
split_failed
	濡傛灉鍐呮牳鏈兘鎷嗗垎澶ч〉锛屽垯閫掑銆傝繖鍙兘鍙戠敓鍦ㄨ椤佃鏌愪汉鍥哄畾锛坧in锛夋椂銆?
split_deferred
        褰撲竴涓ぇ椤佃鏀惧叆鎷嗗垎闃熷垪鏃堕€掑銆?        杩欏彂鐢熷湪澶ч〉琚儴鍒嗗彇娑堟槧灏勩€佹媶鍒嗗畠灏嗛噴鏀句竴浜涘唴瀛樻椂銆傛媶鍒嗛槦鍒椾笂鐨勯〉灏嗗湪
        鍐呭瓨鍘嬪姏涓嬭鎷嗗垎锛堝鏋滄媶鍒嗘槸鍙兘鐨勶級銆?
nr_anon
       鏁翠釜绯荤粺涓垜浠嫢鏈夌殑鍖垮悕 THP 鏁伴噺銆傝繖浜?THP 鍙兘褰撳墠琚暣浣撴槧灏勶紝鎴栬€呭叿鏈?       閮ㄥ垎鍙栨秷鏄犲皠/鏈娇鐢ㄧ殑瀛愰〉銆?
nr_anon_partially_mapped
       鍙兘閮ㄥ垎鏄犲皠銆佷粠鑰屽彲鑳芥氮璐瑰唴瀛樸€佸苟宸茶鎺掑叆寤惰繜鍐呭瓨鍥炴敹闃熷垪鐨勫尶鍚?THP 鏁伴噺銆?       娉ㄦ剰锛屽湪杈硅鎯呭喌涓嬶紙渚嬪杩佺Щ澶辫触锛夛紝鎴戜滑鍙兘灏嗕竴涓尶鍚?THP 妫€娴嬩负鈥滈儴鍒嗘槧灏勨€?       骞跺湪姝よ鏁帮紝鍗充娇瀹冨疄闄呬笂宸蹭笉鍐嶉儴鍒嗘槧灏勩€?
闅忕潃绯荤粺鑰佸寲锛屽垎閰嶅ぇ椤靛彲鑳藉緢鏄傝吹锛屽洜涓虹郴缁熶娇鐢ㄥ唴瀛樺帇缂╋紙memory compaction锛夊湪鍐呭瓨涓?澶嶅埗鏁版嵁锛屼互鑵惧嚭涓€涓ぇ椤典緵浣跨敤銆俙/proc/vmstat` 涓湁涓€浜涜鏁板櫒鍙府鍔╃洃瑙嗚繖绉嶅紑閿€銆?
compact_stall
	姣忔涓€涓繘绋嬪仠婊炰互杩愯鍐呭瓨鍘嬬缉銆佷粠鑰岃吘鍑轰竴涓ぇ椤典緵浣跨敤鏃堕€掑銆?
compact_success
	濡傛灉绯荤粺鍘嬬缉浜嗗唴瀛樺苟鑵惧嚭涓€涓ぇ椤典緵浣跨敤锛屽垯閫掑銆?
compact_fail
	濡傛灉绯荤粺灏濊瘯鍘嬬缉鍐呭瓨浣嗗け璐ワ紝鍒欓€掑銆?
鍙互浣跨敤鍑芥暟璺熻釜鍣紙function tracer锛夎褰曡姳鍦?__alloc_pages() 涓殑鏃堕棿锛屽苟浣跨敤
mm_page_alloc 璺熻釜鐐癸紙tracepoint锛夋潵璇嗗埆鍝簺鍒嗛厤鏄拡瀵瑰ぇ椤碉紝浠庤€岀‘瀹氬仠婊炴寔缁簡
澶氫箙銆?
## 浼樺寲搴旂敤绋嬪簭


瑕佷繚璇佸唴鏍镐細鍦ㄤ换浣曞唴瀛樺尯鍩熺珛鍗虫槧灏勪竴涓?THP锛宮map 鍖哄煙蹇呴』鑷劧鎸夊ぇ椤靛榻愩€?posix_memalign() 鍙互鎻愪緵杩欑淇濊瘉銆?
## Hugetlbfs


浣犲彲浠ュ湪鍚敤浜嗛€忔槑澶ч〉鏀寔鐨勫唴鏍镐笂鐓у父浣跨敤 hugetlbfs锛屾鏃犻棶棰樸€傞櫎浜嗘暣浣撶鐗囧寲浼?鏇村皯涔嬪锛宧ugetlbfs 涓笉浼氭敞鎰忓埌浠讳綍宸紓銆傚睘浜?hugetlbfs 鐨勬墍鏈夊父鐢ㄧ壒鎬ч兘寰椾互淇濈暀
涓斾笉鍙楀奖鍝嶃€俵ibhugetlbfs 涔熶細鍍忓線甯镐竴鏍锋甯稿伐浣溿€?