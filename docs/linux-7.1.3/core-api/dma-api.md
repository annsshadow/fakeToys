## 浣跨敤閫氱敤璁惧鐨勫姩鎬?DMA 鏄犲皠


:Author: James E.J. Bottomley <James.Bottomley@HansenPartnership.com>

鏈枃妗ｆ弿杩?DMA API銆傚鏋滀綘鎯虫洿骞崇紦鍦颁簡瑙ｈ API锛堜互鍙婂疄闄呯ず渚嬶級锛岃鍙傞槄
Documentation/core-api/dma-api-howto.rst銆?
璇?API 鍒嗕负涓ら儴鍒嗐€傜涓€閮ㄥ垎鎻忚堪鍩虹 API銆傜浜岄儴鍒嗘弿杩扮敤浜庢敮鎸侀潪涓€鑷存€у唴瀛?鏈哄櫒鐨勬墿灞曘€傞櫎闈炰綘鏄庣‘鐭ラ亾浣犵殑椹卞姩蹇呴』鏀寔闈炰竴鑷存€у钩鍙帮紙閫氬父鍙湁閬楃暀骞冲彴锛夛紝
鍚﹀垯浣犲彧搴斾娇鐢ㄧ涓€閮ㄥ垎鎻忚堪鐨?API銆?
### 绗竴閮ㄥ垎 - DMA API

瑕佽幏鍙?DMA API锛屼綘蹇呴』 #include <linux/dma-mapping.h>銆傝繖鎻愪緵浜?dma_addr_t
浠ュ強涓嬮潰鎻忚堪鐨勬帴鍙ｃ€?
dma_addr_t 鍙互鎸佹湁璇ュ钩鍙颁笂浠讳綍鏈夋晥鐨?DMA 鍦板潃銆傚畠鍙互琚氦缁欒澶囷紝鐢ㄤ綔 DMA
婧愭垨鐩爣銆侰PU 涓嶈兘鐩存帴寮曠敤 dma_addr_t锛屽洜涓哄湪鍏剁墿鐞嗗湴鍧€绌洪棿涓?DMA 鍦板潃绌洪棿涔嬮棿
鍙兘瀛樺湪杞崲銆?
### 绗竴閮ㄥ垎 a - 浣跨敤澶у瀷 DMA 涓€鑷存€х紦鍐插尯

```

	void *
	dma_alloc_coherent(struct device *dev, size_t size,
			   dma_addr_t *dma_handle, gfp_t flag)

```
涓€鑷存€у唴瀛橈紙coherent memory锛夋槸鎸囪澶囨垨澶勭悊鍣ㄤ换鎰忎竴鏂瑰啓鍏ュ悗锛屽鐞嗗櫒鎴栬澶囬兘鑳?绔嬪嵆璇诲彇銆佽€屾棤闇€鎷呭績缂撳瓨褰卞搷鐨勫唴瀛樸€傦紙涓嶈繃鍦ㄥ憡璇夎澶囧幓璇昏鍐呭瓨涔嬪墠锛屼綘鍙兘浠嶉渶瑕?纭繚鍒锋柊澶勭悊鍣ㄧ殑鍐欑紦鍐插尯銆傦級

璇ヤ緥绋嬪垎閰嶄竴鍧?<size> 瀛楄妭鐨勪竴鑷存€у唴瀛樺尯鍩熴€?
瀹冭繑鍥炰竴涓寚鍚戞墍鍒嗛厤鍖哄煙锛堝湪澶勭悊鍣ㄨ櫄鎷熷湴鍧€绌洪棿涓級鐨勬寚閽堬紝濡傛灉鍒嗛厤澶辫触鍒欒繑鍥?NULL銆?
瀹冭繕浼氳繑鍥炰竴涓?<dma_handle>锛屽畠鍙互琚浆鎹负涓庢€荤嚎鍚屽鐨勬棤绗﹀彿鏁存暟锛屽苟浣滀负璇ュ尯鍩熺殑
DMA 鍦板潃鍩哄湴鍧€浜ょ粰璁惧銆?
娉ㄦ剰锛氬湪鏌愪簺骞冲彴涓婁竴鑷存€у唴瀛樺彲鑳藉緢鏄傝吹锛屼笖鏈€灏忓垎閰嶉暱搴﹀彲鑳藉拰涓€涓〉涓€鏍峰ぇ锛屽洜姝や綘
搴斿敖鍙兘鍚堝苟瀵逛竴鑷存€у唴瀛樼殑璇锋眰銆傛渶绠€鍗曠殑鏂规硶鏄娇鐢?dma_pool 璋冪敤锛堣涓嬫枃锛夈€?
flag 鍙傛暟鍏佽璋冪敤鑰呮寚瀹氬垎閰嶇殑 `GFP_` 鏍囧織锛堣 kmalloc()锛夛紙瀹炵幇鍙兘浼氬拷鐣ュ奖鍝嶈繑鍥?鍐呭瓨浣嶇疆鐨勬爣蹇楋紝濡?GFP_DMA锛夈€?
```

	void
	dma_free_coherent(struct device *dev, size_t size, void *cpu_addr,
			  dma_addr_t dma_handle)

```
閲婃斁鍏堝墠鍒嗛厤鐨勪竴鑷存€у唴瀛樺尯鍩熴€俤ev銆乻ize 鍜?dma_handle 蹇呴』鍏ㄩ儴涓庝紶鍏?dma_alloc_coherent() 鐨勪竴鑷淬€俢pu_addr 蹇呴』鏄?dma_alloc_coherent() 杩斿洖鐨勮櫄鎷熷湴鍧€銆?
娉ㄦ剰锛屼笌璇ュ垎閰嶇殑鍏勫紵璋冪敤涓嶅悓锛屾渚嬬▼鍙兘鍦?IRQ 鍚敤鏃惰璋冪敤銆?

### 绗竴閮ㄥ垎 b - 浣跨敤灏忓瀷 DMA 涓€鑷存€х紦鍐插尯

瑕佽幏鍙?DMA API 鐨勮繖涓€閮ㄥ垎锛屼綘蹇呴』 #include <linux/dmapool.h>

璁稿椹卞姩闇€瑕佸ぇ閲忓皬鍨?DMA 涓€鑷存€у唴瀛樺尯鍩熸潵瀛樻斁 DMA 鎻忚堪绗︽垨 I/O 缂撳啿鍖恒€備笌鍏剁敤
dma_alloc_coherent() 浠ラ〉鎴栨洿澶х殑鍗曚綅鍒嗛厤锛屼綘鍙互浣跨敤 DMA 姹狅紙pool锛夈€傚畠浠殑
宸ヤ綔鏂瑰紡寰堝儚 struct kmem_cache锛屽彧鏄畠浠娇鐢?DMA 涓€鑷存€у垎閰嶅櫒锛岃€屼笉鏄?__get_free_pages()銆傛澶栵紝瀹冧滑鐞嗚В甯歌鐨勭‖浠跺榻愮害鏉燂紝姣斿闃熷垪澶撮渶瑕佸榻愬埌 N 瀛楄妭
杈圭晫銆?
   :export:



### 绗竴閮ㄥ垎 c - DMA 瀵诲潃闄愬埗

DMA 鎺╃爜锛坢ask锛夋槸璇ヨ澶囧彲瀵诲潃鍖哄煙鐨勪綅鎺╃爜銆傛崲鍙ヨ瘽璇达紝濡傛灉瀵规煇涓€鍐呭瓨鍖哄煙鐨?DMA 鍦板潃
搴旂敤 DMA 鎺╃爜锛堟寜浣嶄笌鎿嶄綔锛変笉浼氭竻闄ゅ湴鍧€涓殑浠讳綍浣嶏紝閭ｄ箞璇ヨ澶囧氨鑳藉璇ュ唴瀛樺尯鍩熸墽琛?DMA銆?
涓嬮潰鎵€鏈夎缃?DMA 鎺╃爜鐨勫嚱鏁帮紝濡傛灉鎵€璇锋眰鐨勬帺鐮佹棤娉曠敤浜庤璁惧锛屾垨鑰呰璁惧涓嶅叿澶囨墽琛?DMA
鐨勮兘鍔涳紝閮藉彲鑳藉け璐ャ€?
```

	int
	dma_set_mask_and_coherent(struct device *dev, u64 mask)

```
鍚屾椂鏇存柊娴佸紡锛坰treaming锛夊拰涓€鑷存€э紙coherent锛塂MA 鎺╃爜銆?
杩斿洖锛氭垚鍔熻繑鍥?0锛屽け璐ヨ繑鍥炶礋鐨勯敊璇爜銆?
```

	int
	dma_set_mask(struct device *dev, u64 mask)

```
浠呮洿鏂版祦寮?DMA 鎺╃爜銆?
杩斿洖锛氭垚鍔熻繑鍥?0锛屽け璐ヨ繑鍥炶礋鐨勯敊璇爜銆?
```

	int
	dma_set_coherent_mask(struct device *dev, u64 mask)

```
浠呮洿鏂颁竴鑷存€?DMA 鎺╃爜銆?
杩斿洖锛氭垚鍔熻繑鍥?0锛屽け璐ヨ繑鍥炶礋鐨勯敊璇爜銆?
```

	u64
	dma_get_required_mask(struct device *dev)

```
姝?API 杩斿洖骞冲彴涓轰簡楂樻晥杩愯鎵€闇€鐨勬帺鐮併€傞€氬父杩欐剰鍛崇潃杩斿洖鐨勬帺鐮佹槸瑕嗙洊鍏ㄩ儴鍐呭瓨鎵€闇€鐨?鏈€灏忔帺鐮併€傛鏌ユ墍闇€鎺╃爜鍙互璁╁叿鏈夊彲鍙樻弿杩扮澶у皬鐨勯┍鍔ㄦ湁鏈轰細鍦ㄥ繀瑕佹椂浣跨敤鏇村皬鐨勬弿杩扮銆?
璇锋眰鎵€闇€鎺╃爜涓嶄細鏀瑰彉褰撳墠鎺╃爜銆傚鏋滀綘鎯冲埄鐢ㄥ畠锛屽簲璇ヨ皟鐢?dma_set_mask() 灏嗘帺鐮佽涓?杩斿洖鐨勫€笺€?
```

	size_t
	dma_max_mapping_size(struct device *dev);

```
杩斿洖璇ヨ澶囨槧灏勭殑鏈€澶уぇ灏忋€俤ma_map_single()銆乨ma_map_page() 绛夋槧灏勫嚱鏁扮殑 size 鍙傛暟
涓嶅簲澶т簬杩斿洖鍊笺€?
```

	size_t
	dma_opt_mapping_size(struct device *dev);

```
杩斿洖璇ヨ澶囨槧灏勭殑鏈€澶ф渶浼樺ぇ灏忋€?
鏄犲皠鏇村ぇ鐨勭紦鍐插尯鍦ㄦ煇浜涘満鏅笅鍙兘鑺辫垂闀垮緱澶氱殑鏃堕棿銆傛澶栵紝瀵逛簬楂橀€熴€佺煭鐢熷懡鍛ㄦ湡鐨勬祦寮?鏄犲皠锛屾槧灏勬墍鑺辫垂鐨勫墠鏈熸椂闂村彲鑳藉崰鏁翠釜璇锋眰鐢熷懡鍛ㄦ湡涓浉褰撳彲瑙傜殑涓€閮ㄥ垎銆傚洜姝わ紝濡傛灉鎷嗗垎
鏇村ぇ鐨勮姹備笉浼氬甫鏉ユ槑鏄剧殑鎬ц兘鎹熷け锛屽缓璁澶囬┍鍔ㄥ皢 DMA 娴佸紡鏄犲皠鐨勬€婚暱搴﹂檺鍒跺湪杩斿洖鍊?浠ュ唴銆?
```

	bool
	dma_need_sync(struct device *dev, dma_addr_t dma_addr);

```
濡傛灉杞Щ鍐呭瓨鎵€鏈夋潈闇€瑕?dma_sync_single_for_{device,cpu} 璋冪敤锛屽垯杩斿洖 %true銆傚鏋?鍙互璺宠繃杩欎簺璋冪敤锛屽垯杩斿洖 %false銆?
```

	unsigned long
	dma_get_merge_boundary(struct device *dev);

```
杩斿洖 DMA 鍚堝苟杈圭晫銆傚鏋滆澶囨棤娉曞悎骞朵换浣?DMA 鍦板潃娈碉紝璇ュ嚱鏁拌繑鍥?0銆?
### 绗竴閮ㄥ垎 d - 娴佸紡 DMA 鏄犲皠

娴佸紡 DMA 鍏佽鏄犲皠涓€涓凡鏈夌殑缂撳啿鍖虹敤浜?DMA 浼犺緭锛屽苟鍦ㄥ畬鎴愬悗瑙ｉ櫎鏄犲皠銆傛槧灏勫嚱鏁颁笉淇濊瘉
鎴愬姛锛屽洜姝ゅ繀椤绘鏌ヨ繑鍥炲€笺€?

	鐗瑰埆鍦帮紝瀵逛簬璁惧涓嶅彲瀵诲潃鐨勫唴瀛橈紝鏄犲皠鍙兘浼氬け璐ワ紝渚嬪瀹冧笉鍦ㄨ澶囩殑 DMA 鎺╃爜鍜?鎴?	杩炴帴鐨勬€荤嚎妗ョ殑瀵诲潃鑼冨洿鍐呫€傛祦寮?DMA 鍑芥暟璇曞浘鍏嬫湇杩欐牱鐨勫鍧€绾︽潫锛岃涔堥€氳繃浣跨敤
	IOMMU锛堜竴涓皢 I/O DMA 鍦板潃鏄犲皠鍒扮墿鐞嗗唴瀛樺湴鍧€鐨勮澶囷級锛岃涔堝湪閰嶇疆浜?	[SWIOTLB <swiotlb>](SWIOTLB <swiotlb>) 鐨勫唴鏍镐腑锛屾妸鏁版嵁澶嶅埗鍒板脊璺崇紦鍐插尯锛坆ounce
	buffer锛夋垨浠庡脊璺崇紦鍐插尯澶嶅埗鍑烘潵銆傜劧鑰岋紝杩欎簺鏂规硶骞朵笉鎬绘槸鍙敤锛岃€屼笖鍗充究鍙敤锛屼篃鍙兘
	鍥犱负澶氱鍘熷洜澶辫触銆?
	绠€鑰岃█涔嬶紝璁惧椹卞姩鍙兘闇€瑕佽鎯曠紦鍐插尯鍦ㄧ墿鐞嗗唴瀛樹腑鐨勪綅缃紝灏ゅ叾鏄綋 DMA 鎺╃爜灏忎簬 32
	浣嶆椂銆?
```

	dma_addr_t
	dma_map_single(struct device *dev, void *cpu_addr, size_t size,
		       enum dma_data_direction direction)

```
鏄犲皠涓€鍧楀鐞嗗櫒铏氭嫙鍐呭瓨锛屼娇鍏惰兘琚澶囪闂紝骞惰繑鍥炶鍐呭瓨鐨?DMA 鍦板潃銆?
DMA API 瀵瑰叾鏂瑰悜浣跨敤寮虹被鍨嬫灇涓撅細

======================= =============================================
DMA_NONE		鏃犳柟鍚戯紙鐢ㄤ簬璋冭瘯锛?DMA_TO_DEVICE		鏁版嵁姝ｄ粠鍐呭瓨鍙戝線璁惧
DMA_FROM_DEVICE		鏁版嵁姝ｄ粠璁惧鍙戝線鍐呭瓨
DMA_BIDIRECTIONAL	鏂瑰悜鏈煡
======================= =============================================


	杩炵画鐨勫唴鏍歌櫄鎷熺┖闂村湪鐗╃悊鍐呭瓨涓婃湭蹇呰繛缁€傜敱浜庢 API 涓嶆彁渚涗换浣曞垎鏁?鑱氶泦
	锛坰catter/gather锛夎兘鍔涳紝濡傛灉鐢ㄦ埛璇曞浘鏄犲皠涓€鍧楃墿鐞嗕笂涓嶈繛缁殑鍐呭瓨锛屽畠浼氬け璐ャ€傚洜姝わ紝
	瑕佺敱姝?API 鏄犲皠鐨勫唴瀛樺簲褰撴潵鑷兘淇濊瘉鍏剁墿鐞嗕笂杩炵画鐨勫湴鏂癸紙濡?kmalloc锛夈€?

	鍐呭瓨涓€鑷存€э紙coherency锛変互绉颁负缂撳瓨琛屽搴︾殑绮掑害杩愪綔銆備负浜嗚姝?API 鏄犲皠鐨勫唴瀛樻纭?	宸ヤ綔锛岃鏄犲皠鍖哄煙蹇呴』鎭板ソ璧峰浜庝竴涓紦瀛樿杈圭晫銆佸苟鎭板ソ缁撴潫浜庝竴涓紦瀛樿杈圭晫锛堜互闃叉
	涓や釜鍒嗗埆鏄犲皠鐨勫尯鍩熷叡浜悓涓€涓紦瀛樿锛夈€傜敱浜庣紦瀛樿澶у皬鍦ㄧ紪璇戞椂鍙兘鏈煡锛岃 API 涓嶄細
	寮哄埗杩欎竴瑕佹眰銆傚洜姝わ紝寤鸿閭ｄ簺涓嶇壒鍒幓纭畾杩愯鏃剁紦瀛樿澶у皬鐨勯┍鍔ㄤ綔鑰咃紝鍙槧灏勮捣濮嬪拰
	缁撴潫閮藉湪椤佃竟鐣屼笂鐨勮櫄鎷熷尯鍩燂紙椤佃竟鐣屼繚璇佷篃鏄紦瀛樿杈圭晫锛夈€?
	DMA_TO_DEVICE 鍚屾蹇呴』鍦ㄨ蒋浠舵渶鍚庝竴娆′慨鏀瑰唴瀛樺尯鍩熶箣鍚庛€佷笖鍦ㄦ妸瀹冧氦缁欒澶囦箣鍓嶅畬鎴愩€?	涓€鏃︿娇鐢ㄤ簡杩欎竴鍘熻锛岃鍘熻鎵€瑕嗙洊鐨勫唴瀛樺簲褰撹璁惧瑙嗕负鍙銆傚鏋滆澶囧彲鑳藉湪浠讳綍鏃跺埢
	鍐欏叆瀹冿紝閭ｅ畠搴旇鏄?DMA_BIDIRECTIONAL锛堣涓嬫枃锛夈€?
	DMA_FROM_DEVICE 鍚屾蹇呴』鍦ㄩ┍鍔ㄨ闂彲鑳借璁惧鏀瑰彉鐨勬暟鎹箣鍓嶅畬鎴愩€傝繖鍧楀唴瀛樺簲褰撹
	椹卞姩瑙嗕负鍙銆傚鏋滈┍鍔ㄩ渶瑕佸湪浠讳綍鏃跺埢鍐欏叆瀹冿紝閭ｅ畠搴旇鏄?DMA_BIDIRECTIONAL锛堣
	涓嬫枃锛夈€?
	DMA_BIDIRECTIONAL 闇€瑕佺壒娈婂鐞嗭細瀹冩剰鍛崇潃椹卞姩鏃笉纭畾鍐呭瓨鍦ㄤ氦缁欒澶囦箣鍓嶆槸鍚﹁淇敼
	杩囷紝涔熶笉纭畾璁惧鏄惁涔熶細淇敼瀹冦€傚洜姝わ紝浣犲繀椤绘€绘槸鍚屾鍙屽悜鍐呭瓨涓ゆ锛氫竴娆″湪鎶婂唴瀛樹氦缁?	璁惧涔嬪墠锛堜互纭繚鎵€鏈夊唴瀛樹慨鏀归兘宸蹭粠澶勭悊鍣ㄥ埛鏂帮級锛屼竴娆″湪鍐呭瓨琚澶囦娇鐢ㄤ箣鍚庛€佹暟鎹彲鑳?	琚闂箣鍓嶏紙浠ョ‘淇濅换浣曞鐞嗗櫒缂撳瓨琛岄兘鏇存柊涓鸿澶囧彲鑳藉凡淇敼鐨勬暟鎹級銆?
```

	void
	dma_unmap_single(struct device *dev, dma_addr_t dma_addr, size_t size,
			 enum dma_data_direction direction)

```
瑙ｉ櫎鍏堝墠鏄犲皠鐨勫尯鍩熴€備紶鍏ョ殑鎵€鏈夊弬鏁板繀椤讳笌浼犲叆锛堝拰鐢憋級dma_map_single() 鐨勶紙杩斿洖鐨勶級瀹屽叏涓€鑷淬€?
```

	dma_addr_t
	dma_map_page(struct device *dev, struct page *page,
		     unsigned long offset, size_t size,
		     enum dma_data_direction direction)

	void
	dma_unmap_page(struct device *dev, dma_addr_t dma_address, size_t size,
		       enum dma_data_direction direction)

```
鐢ㄤ簬椤垫槧灏勫拰瑙ｉ櫎鏄犲皠鐨?API銆傚叾浠栨槧灏?API 鐨勬墍鏈夋敞鎰忎簨椤瑰拰璀﹀憡閮介€傜敤浜庤繖閲屻€傛澶栵紝
铏界劧鎻愪緵浜?<offset> 鍜?<size> 鍙傛暟鐢ㄤ簬鍋氶儴鍒嗛〉鏄犲皠锛屼絾寤鸿浣犻櫎闈炵‘瀹炵煡閬撶紦瀛樺搴?鏄粈涔堬紝鍚﹀垯缁濅笉瑕佷娇鐢ㄥ畠浠€?
```

	dma_addr_t
	dma_map_resource(struct device *dev, phys_addr_t phys_addr, size_t size,
			 enum dma_data_direction dir, unsigned long attrs)

	void
	dma_unmap_resource(struct device *dev, dma_addr_t addr, size_t size,
			   enum dma_data_direction dir, unsigned long attrs)

```
鐢ㄤ簬 MMIO 璧勬簮鏄犲皠鍜岃В闄ゆ槧灏勭殑 API銆傚叾浠栨槧灏?API 鐨勬墍鏈夋敞鎰忎簨椤瑰拰璀﹀憡閮介€傜敤浜庤繖閲屻€?璇?API 鍙簲鐢ㄤ簬鏄犲皠璁惧 MMIO 璧勬簮锛屼笉鍏佽鏄犲皠 RAM銆?
```

	int
	dma_mapping_error(struct device *dev, dma_addr_t dma_addr)

```
鍦ㄦ煇浜涙儏鍐典笅 dma_map_single()銆乨ma_map_page() 鍜?dma_map_resource() 浼氬垱寤烘槧灏勫け璐ャ€?椹卞姩鍙互閫氳繃鐢?dma_mapping_error() 娴嬭瘯杩斿洖鐨?DMA 鍦板潃鏉ユ鏌ヨ繖浜涢敊璇€傞潪闆惰繑鍥炲€兼剰鍛崇潃
鏃犳硶鍒涘缓鏄犲皠锛岄┍鍔ㄥ簲褰撻噰鍙栭€傚綋鎺柦锛堜緥濡傚噺灏戝綋鍓?DMA 鏄犲皠鐨勪娇鐢ㄩ噺锛屾垨寤惰繜绋嶅悗閲嶈瘯锛夈€?
```

	int
	dma_map_sg(struct device *dev, struct scatterlist *sg,
		   int nents, enum dma_data_direction direction)

```
涓?DMA 鏄犲皠涓€涓垎鏁?鑱氶泦鍒楄〃銆傝繑鍥炶鏄犲皠鐨?DMA 鍦板潃娈垫暟閲忥紝濡傛灉鑻ュ共杩炵画鐨?sglist 鏉＄洰
琚悎骞讹紙渚嬪閫氳繃 IOMMU锛屾垨鑰呮煇浜涚浉閭荤殑娈靛垰濂界宸х墿鐞嗕笂杩炵画锛夛紝璇ユ暟閲忓彲鑳藉皬浜庝紶鍏ョ殑
<nents>銆?
璇锋敞鎰忥紝sg 涓€鏃﹁鏄犲皠灏变笉鑳藉啀娆℃槧灏勩€傛槧灏勮繃绋嬪厑璁哥牬鍧?sg 涓殑淇℃伅銆?
涓庡叾浠栨槧灏勬帴鍙ｄ竴鏍凤紝dma_map_sg() 鍙兘澶辫触銆傚綋瀹冨け璐ユ椂锛岃繑鍥?0锛岄┍鍔ㄥ繀椤婚噰鍙栭€傚綋鎺柦銆?椹卞姩鍋氱偣浠€涔堣嚦鍏抽噸瑕侊紝瀵逛簬鍧楄澶囨潵璇达紝涓璇锋眰鐢氳嚦瑙﹀彂 oops 閮芥瘮浠€涔堥兘涓嶅仛銆佽繘鑰屾崯鍧?鏂囦欢绯荤粺瑕佸ソ銆?
```

	int i, count = dma_map_sg(dev, sglist, nents, direction);
	struct scatterlist *sg;

	for_each_sg(sglist, sg, count, i) {
		hw_address[i] = sg_dma_address(sg);
		hw_len[i] = sg_dma_len(sg);
	}

```
鍏朵腑 nents 鏄?sglist 涓殑鏉＄洰鏁伴噺銆?
瀹炵幇鍙互鑷敱鍦板皢鑻ュ共杩炵画鐨?sglist 鏉＄洰鍚堝苟涓轰竴涓€傝繑鍥炵殑鏁伴噺鏄畠瀹為檯鏄犲皠鍒扮殑 sg 鏉＄洰
鏁般€傚け璐ユ椂杩斿洖 0銆?
鐒跺悗浣犲簲璇ュ惊鐜?count 娆★紙娉ㄦ剰锛氳繖鍙兘灏戜簬 nents 娆★級锛屽苟鍦ㄤ綘鍏堝墠璁块棶 sg->address 鍜?sg->length 鐨勫湴鏂逛娇鐢?sg_dma_address() 鍜?sg_dma_len() 瀹忥紝濡備笂鎵€绀恒€?
```

	void
	dma_unmap_sg(struct device *dev, struct scatterlist *sg,
		     int nents, enum dma_data_direction direction)

```
瑙ｉ櫎鍏堝墠鏄犲皠鐨勫垎鏁?鑱氶泦鍒楄〃銆傛墍鏈夊弬鏁板繀椤讳笌浼犲叆鍒嗘暎/鑱氶泦鏄犲皠 API 鐨勭浉鍚屻€?
娉ㄦ剰锛?nents> 蹇呴』鏄綘浼犲叆鐨勬暟閲忥紝**涓嶆槸** 杩斿洖鐨?DMA 鍦板潃鏉＄洰鏁伴噺銆?
```

	void
	dma_sync_single_for_cpu(struct device *dev, dma_addr_t dma_handle,
				size_t size,
				enum dma_data_direction direction)

	void
	dma_sync_single_for_device(struct device *dev, dma_addr_t dma_handle,
				   size_t size,
				   enum dma_data_direction direction)

	void
	dma_sync_sg_for_cpu(struct device *dev, struct scatterlist *sg,
			    int nents,
			    enum dma_data_direction direction)

	void
	dma_sync_sg_for_device(struct device *dev, struct scatterlist *sg,
			       int nents,
			       enum dma_data_direction direction)

```
涓?CPU 鍜岃澶囧悓姝ヤ竴涓繛缁殑鎴栧垎鏁?鑱氶泦鐨勬槧灏勩€傚浜?sync_sg API锛屾墍鏈夊弬鏁板繀椤讳笌浼犲叆
sg 鏄犲皠 API 鐨勭浉鍚屻€傚浜?sync_single API锛屼綘鍙互浣跨敤涓庝紶鍏ュ崟娆℃槧灏?API 涓嶅畬鍏ㄧ浉鍚岀殑
dma_handle 鍜?size 鍙傛暟锛屼互杩涜閮ㄥ垎鍚屾銆?


   浣犲繀椤昏繖鏍峰仛锛?
   - 鍦ㄨ鍙栫敱璁惧閫氳繃 DMA 鍐欏叆鐨勫€间箣鍓嶏紙浣跨敤 DMA_FROM_DEVICE 鏂瑰悜锛?   - 鍦ㄥ啓鍏ュ皢閫氳繃 DMA 鍐欏叆璁惧鐨勫€间箣鍚庯紙浣跨敤 DMA_TO_DEVICE 鏂瑰悜锛?   - 鍦ㄦ妸鍐呭瓨浜ょ粰璁惧**涔嬪墠鍜屼箣鍚?*锛屽鏋滃唴瀛樻槸 DMA_BIDIRECTIONAL

鍙﹁ dma_map_single()銆?
```

	dma_addr_t
	dma_map_single_attrs(struct device *dev, void *cpu_addr, size_t size,
			     enum dma_data_direction dir,
			     unsigned long attrs)

	void
	dma_unmap_single_attrs(struct device *dev, dma_addr_t dma_addr,
			       size_t size, enum dma_data_direction dir,
			       unsigned long attrs)

	int
	dma_map_sg_attrs(struct device *dev, struct scatterlist *sgl,
			 int nents, enum dma_data_direction dir,
			 unsigned long attrs)

	void
	dma_unmap_sg_attrs(struct device *dev, struct scatterlist *sgl,
			   int nents, enum dma_data_direction dir,
			   unsigned long attrs)

```
涓婇潰杩欏洓涓嚱鏁颁笌涓嶅甫 _attrs 鍚庣紑鐨勫搴斿嚱鏁扮被浼硷紝鍙槸瀹冧滑浼犲叆涓€涓彲閫夌殑 dma_attrs銆?
DMA 灞炴€х殑瑙ｉ噴鏄灦鏋勭浉鍏崇殑锛屾瘡涓睘鎬ч兘搴斿湪
Documentation/core-api/dma-attributes.rst 涓褰曘€?
濡傛灉 dma_attrs 涓?0锛岃繖浜涘嚱鏁颁腑姣忎竴涓殑璇箟閮戒笌涓嶅甫 _attrs 鍚庣紑鐨勫搴斿嚱鏁扮浉鍚屻€傚洜姝?dma_map_single_attrs() 閫氬父鍙互鏇夸唬 dma_map_single() 绛夈€?
浣滀负浣跨敤 `*_attrs` 鍑芥暟鐨勪竴涓緥瀛愶紝涓嬮潰鏄綘濡備綍鍦ㄦ槧灏勫唴瀛樻椂浼犲叆灞炴€?DMA_ATTR_FOO
```

	#include <linux/dma-mapping.h>
	/* DMA_ATTR_FOO 搴斿綋瀹氫箟鍦?linux/dma-mapping.h 涓紝骞跺湪
	* Documentation/core-api/dma-attributes.rst 涓褰?*/
	...

		unsigned long attr;
		attr |= DMA_ATTR_FOO;
		....
		n = dma_map_sg_attrs(dev, sg, nents, DMA_TO_DEVICE, attr);
		....

```
鍏冲績 DMA_ATTR_FOO 鐨勬灦鏋勪細鍦ㄥ畠浠槧灏勫拰瑙ｉ櫎鏄犲皠鐨勫疄鐜颁腑妫€鏌ュ畠鐨勫瓨鍦?```

	void whizco_dma_map_sg_attrs(struct device *dev, dma_addr_t dma_addr,
				     size_t size, enum dma_data_direction dir,
				     unsigned long attrs)
	{
		....
		if (attrs & DMA_ATTR_FOO)
			/* twizzle the frobnozzle */
		....
	}

```
### 绗竴閮ㄥ垎 e - 鍩轰簬 IOVA 鐨?DMA 鏄犲皠

杩欎簺 API 鍦ㄤ娇鐢?IOMMU 鏃跺厑璁搁潪甯搁珮鏁堢殑鏄犲皠銆傚畠浠槸涓€鏉″彲閫夎矾寰勶紝闇€瑕侀澶栫殑浠ｇ爜锛屼粎
鎺ㄨ崘鐢ㄤ簬 DMA 鏄犲皠鎬ц兘銆佹垨鐢ㄤ簬瀛樺偍 DMA 鍦板潃鐨勭┖闂村崰鐢ㄥ緢閲嶈鐨勯┍鍔ㄣ€備笂涓€鑺傜殑鎵€鏈夋敞鎰?浜嬮」鍚屾牱閫傜敤浜庤繖閲屻€?
```

    bool dma_iova_try_alloc(struct device *dev, struct dma_iova_state *state,
		phys_addr_t phys, size_t size);

```
鐢ㄤ簬灏濊瘯鍒嗛厤鐢ㄤ簬鏄犲皠鎿嶄綔鐨?IOVA 绌洪棿銆傚鏋滆繑鍥?false锛屽垯璇?API 涓嶈兘鐢ㄤ簬缁欏畾璁惧锛?搴斿綋浣跨敤姝ｅ父鐨勬祦寮?DMA 鏄犲皠 API銆俙struct dma_iova_state` 鐢遍┍鍔ㄥ垎閰嶏紝骞朵笖蹇呴』淇濈暀
鍒拌В闄ゆ槧灏勬椂銆?
```

    static inline bool dma_use_iova(struct dma_iova_state *state)

```
鍙敱椹卞姩鐢ㄦ潵鍦ㄨ皟鐢?dma_iova_try_alloc 涔嬪悗妫€鏌ユ槸鍚︿娇鐢ㄤ簡鍩轰簬 IOVA 鐨?API銆傝繖鍦?瑙ｉ櫎鏄犲皠璺緞涓婂彲鑳藉緢鏈夌敤銆?
```

    int dma_iova_link(struct device *dev, struct dma_iova_state *state,
		phys_addr_t phys, size_t offset, size_t size,
		enum dma_data_direction dir, unsigned long attrs);

```
鐢ㄤ簬灏嗚寖鍥撮摼鎺ュ埌鍏堝墠鍒嗛厤鐨?IOVA銆傚浜庣粰瀹氱殑 state锛岄櫎绗竴娆¤皟鐢ㄥ鎵€鏈?dma_iova_link
璋冪敤鐨勮捣濮嬪湴鍧€蹇呴』瀵归綈鍒?`dma_get_merge_boundary()` 杩斿洖鐨?DMA 鍚堝苟杈圭晫锛屽苟涓旈櫎鏈€鍚?涓€涓寖鍥村鐨勬墍鏈夎寖鍥寸殑澶у皬涔熷繀椤诲榻愬埌 DMA 鍚堝苟杈圭晫銆?
```

    int dma_iova_sync(struct device *dev, struct dma_iova_state *state,
		size_t offset, size_t size);

```
蹇呴』琚皟鐢紝浠ュ悓姝ョ敱涓€涓垨澶氫釜 `dma_iova_link()` 璋冪敤鎵€鏄犲皠鐨?IOVA 鑼冨洿鐨?IOMMU
椤佃〃銆?
瀵逛簬浣跨敤涓€娆℃€ф槧灏勭殑椹卞姩锛屾墍鏈夎寖鍥撮兘鍙互琚В闄ゆ槧灏勶紝骞朵笖閫氳繃璋冪敤浠ヤ笅鍑芥暟閲婃斁 IOVA锛?
```

   void dma_iova_destroy(struct device *dev, struct dma_iova_state *state,
		size_t mapped_len, enum dma_data_direction dir,
                unsigned long attrs);

```
鎴栬€咃紝椹卞姩鍙互閫氳繃瑙ｉ櫎鏄犲皠鍜屾槧灏勫崟鐙殑鍖哄煙鏉ュ姩鎬佺鐞?IOVA 绌洪棿銆傚湪閭ｇ鎯呭喌涓?
```

    void dma_iova_unlink(struct device *dev, struct dma_iova_state *state,
		size_t offset, size_t size, enum dma_data_direction dir,
		unsigned long attrs);

```
鐢ㄤ簬瑙ｉ櫎鏄犲皠鍏堝墠鏄犲皠鐨勮寖鍥达紝浠ュ強

```

   void dma_iova_free(struct device *dev, struct dma_iova_state *state);

```
鐢ㄤ簬閲婃斁 IOVA 绌洪棿銆傚湪璋冪敤 `dma_iova_free()` 涔嬪墠锛屾墍鏈夊尯鍩熷繀椤诲凡缁忕敤
`dma_iova_unlink()` 瑙ｉ櫎鏄犲皠銆?
### 绗簩閮ㄥ垎 - 闈炰竴鑷存€?DMA 鍒嗛厤

杩欎簺 API 鍏佽鍒嗛厤淇濊瘉鑳借浼犲叆璁惧閫氳繃 DMA 瀵诲潃鐨勯〉锛屼絾杩欎簺椤甸渶瑕佺敱鍐呮牳涓庤澶囨樉寮忓湴
绠＄悊鍐呭瓨鎵€鏈夋潈銆?
濡傛灉浣犱笉鐞嗚В缂撳瓨琛屼竴鑷存€у湪澶勭悊鍣ㄤ笌 I/O 璁惧涔嬮棿濡備綍宸ヤ綔锛屼綘涓嶅簲璇ヤ娇鐢ㄨ繖閮ㄥ垎 API銆?
```

	struct page *
	dma_alloc_pages(struct device *dev, size_t size, dma_addr_t *dma_handle,
			enum dma_data_direction dir, gfp_t gfp)

```
璇ヤ緥绋嬪垎閰嶄竴鍧?<size> 瀛楄妭鐨勯潪涓€鑷存€у唴瀛樸€傚畠杩斿洖鎸囧悜璇ュ尯鍩熺涓€涓?struct page 鐨勬寚閽堬紝
濡傛灉鍒嗛厤澶辫触鍒欒繑鍥?NULL銆傚緱鍒扮殑 struct page 鍙敤浜?struct page 閫傜敤鐨勪竴鍒囧満鍚堛€?
瀹冭繕浼氳繑鍥炰竴涓?<dma_handle>锛屽畠鍙互琚浆鎹负涓庢€荤嚎鍚屽鐨勬棤绗﹀彿鏁存暟锛屽苟浣滀负璇ュ尯鍩熺殑
DMA 鍦板潃鍩哄湴鍧€浜ょ粰璁惧銆?
dir 鍙傛暟鎸囧畾鏁版嵁鏄惁琚澶囪鍙栧拰/鎴栧啓鍏ワ紝璇﹁ dma_map_single()銆?
gfp 鍙傛暟鍏佽璋冪敤鑰呮寚瀹氬垎閰嶇殑 `GFP_` 鏍囧織锛堣 kmalloc()锛夛紝浣嗘嫆缁濈敤浜庢寚瀹氬唴瀛樺尯鍩燂紙濡?GFP_DMA 鎴?GFP_HIGHMEM锛夌殑鏍囧織銆?
鍦ㄦ妸鍐呭瓨浜ょ粰璁惧涔嬪墠锛岄渶瑕佽皟鐢?dma_sync_single_for_device()锛岃€屽湪璇诲彇鐢辫澶囧啓鍏ョ殑鍐呭瓨
涔嬪墠锛岄渶瑕佽皟鐢?dma_sync_single_for_cpu()锛屽氨鍍忚澶嶇敤鐨勬祦寮?DMA 鏄犲皠涓€鏍枫€?
```

	void
	dma_free_pages(struct device *dev, size_t size, struct page *page,
			dma_addr_t dma_handle, enum dma_data_direction dir)

```
閲婃斁鍏堝墠浣跨敤 dma_alloc_pages() 鍒嗛厤鐨勫唴瀛樺尯鍩熴€俤ev銆乻ize銆乨ma_handle 鍜?dir 蹇呴』
鍏ㄩ儴涓庝紶鍏?dma_alloc_pages() 鐨勪竴鑷淬€俻age 蹇呴』鏄?dma_alloc_pages() 杩斿洖鐨勬寚閽堛€?
```

	int
	dma_mmap_pages(struct device *dev, struct vm_area_struct *vma,
		       size_t size, struct page *page)

```
灏?dma_alloc_pages() 杩斿洖鐨勫垎閰嶆槧灏勫埌鐢ㄦ埛鍦板潃绌洪棿銆俤ev 鍜?size 蹇呴』涓庝紶鍏?dma_alloc_pages() 鐨勪竴鑷淬€俻age 蹇呴』鏄?dma_alloc_pages() 杩斿洖鐨勬寚閽堛€?
```

	void *
	dma_alloc_noncoherent(struct device *dev, size_t size,
			dma_addr_t *dma_handle, enum dma_data_direction dir,
			gfp_t gfp)

```
璇ヤ緥绋嬫槸 dma_alloc_pages 鐨勪竴涓究鎹峰寘瑁咃紝杩斿洖鎵€鍒嗛厤鍐呭瓨鐨勫唴鏍歌櫄鎷熷湴鍧€锛岃€屼笉鏄〉缁撴瀯銆?
```

	void
	dma_free_noncoherent(struct device *dev, size_t size, void *cpu_addr,
			dma_addr_t dma_handle, enum dma_data_direction dir)

```
閲婃斁鍏堝墠浣跨敤 dma_alloc_noncoherent() 鍒嗛厤鐨勫唴瀛樺尯鍩熴€俤ev銆乻ize銆乨ma_handle 鍜?dir
蹇呴』鍏ㄩ儴涓庝紶鍏?dma_alloc_noncoherent() 鐨勪竴鑷淬€俢pu_addr 蹇呴』鏄?dma_alloc_noncoherent()
杩斿洖鐨勮櫄鎷熷湴鍧€銆?
```

	struct sg_table *
	dma_alloc_noncontiguous(struct device *dev, size_t size,
				enum dma_data_direction dir, gfp_t gfp,
				unsigned long attrs);

```
璇ヤ緥绋嬪垎閰?<size> 瀛楄妭鐨勯潪涓€鑷淬€佷笖鍙兘闈炶繛缁殑鍐呭瓨銆傚畠杩斿洖涓€涓寚鍚?struct sg_table 鐨?鎸囬拡锛屾弿杩板凡鍒嗛厤骞跺凡瀹屾垚 DMA 鏄犲皠鐨勫唴瀛橈紝濡傛灉鍒嗛厤澶辫触鍒欒繑鍥?NULL銆傚緱鍒扮殑鍐呭瓨鍙敤浜?struct page 鏄犲皠鍒板垎鏁ｅ垪琛ㄦ墍閫傜敤鐨勫満鍚堛€?
杩斿洖鐨?sg_table 淇濊瘉鍙湁涓€涓崟涓€鐨?DMA 鏄犲皠娈碉紝鐢?sgt->nents 鎸囩ず锛屼絾瀹冨彲鑳芥湁澶氫釜
CPU 渚ф锛岀敱 sgt->orig_nents 鎸囩ず銆?
dir 鍙傛暟鎸囧畾鏁版嵁鏄惁琚澶囪鍙栧拰/鎴栧啓鍏ワ紝璇﹁ dma_map_single()銆?
gfp 鍙傛暟鍏佽璋冪敤鑰呮寚瀹氬垎閰嶇殑 `GFP_` 鏍囧織锛堣 kmalloc()锛夛紝浣嗘嫆缁濈敤浜庢寚瀹氬唴瀛樺尯鍩燂紙濡?GFP_DMA 鎴?GFP_HIGHMEM锛夌殑鏍囧織銆?
attrs 鍙傛暟蹇呴』涓?0 鎴?DMA_ATTR_ALLOC_SINGLE_PAGES銆?
鍦ㄦ妸鍐呭瓨浜ょ粰璁惧涔嬪墠锛岄渶瑕佽皟鐢?dma_sync_sgtable_for_device()锛岃€屽湪璇诲彇鐢辫澶囧啓鍏ョ殑鍐呭瓨
涔嬪墠锛岄渶瑕佽皟鐢?dma_sync_sgtable_for_cpu()锛屽氨鍍忚澶嶇敤鐨勬祦寮?DMA 鏄犲皠涓€鏍枫€?
```

	void
	dma_free_noncontiguous(struct device *dev, size_t size,
			       struct sg_table *sgt,
			       enum dma_data_direction dir)

```
閲婃斁鍏堝墠浣跨敤 dma_alloc_noncontiguous() 鍒嗛厤鐨勫唴瀛樸€俤ev銆乻ize 鍜?dir 蹇呴』鍏ㄩ儴涓庝紶鍏?dma_alloc_noncontiguous() 鐨勪竴鑷淬€俿gt 蹇呴』鏄?dma_alloc_noncontiguous() 杩斿洖鐨勬寚閽堛€?
```

	void *
	dma_vmap_noncontiguous(struct device *dev, size_t size,
		struct sg_table *sgt)

```
涓?dma_alloc_noncontiguous() 杩斿洖鐨勫垎閰嶈繑鍥炰竴鍧楄繛缁殑鍐呮牳鏄犲皠銆俤ev 鍜?size 蹇呴』涓庝紶鍏?dma_alloc_noncontiguous() 鐨勪竴鑷淬€俿gt 蹇呴』鏄?dma_alloc_noncontiguous() 杩斿洖鐨勬寚閽堛€?
涓€鏃︿竴涓潪杩炵画鍒嗛厤琚鍑芥暟鏄犲皠锛屽氨蹇呴』浣跨敤 flush_kernel_vmap_range() 鍜?invalidate_kernel_vmap_range() API 鏉ョ鐞嗗唴鏍告槧灏勩€佽澶囦笌鐢ㄦ埛绌洪棿鏄犲皠锛堝鏋滄湁锛変箣闂寸殑
涓€鑷存€с€?
```

	void
	dma_vunmap_noncontiguous(struct device *dev, void *vaddr)

```
瑙ｉ櫎鐢?dma_vmap_noncontiguous() 杩斿洖鐨勫唴鏍告槧灏勩€俤ev 蹇呴』涓庝紶鍏?dma_alloc_noncontiguous()
鐨勪竴鑷淬€倂addr 蹇呴』鏄?dma_vmap_noncontiguous() 杩斿洖鐨勬寚閽堛€?

```

	int
	dma_mmap_noncontiguous(struct device *dev, struct vm_area_struct *vma,
			       size_t size, struct sg_table *sgt)

```
灏?dma_alloc_noncontiguous() 杩斿洖鐨勫垎閰嶆槧灏勫埌鐢ㄦ埛鍦板潃绌洪棿銆俤ev 鍜?size 蹇呴』涓庝紶鍏?dma_alloc_noncontiguous() 鐨勪竴鑷淬€俿gt 蹇呴』鏄?dma_alloc_noncontiguous() 杩斿洖鐨勬寚閽堛€?
```

	int
	dma_get_cache_alignment(void)

```
杩斿洖澶勭悊鍣ㄧ紦瀛樺榻愩€傝繖鏄綘鍦ㄦ槧灏勫唴瀛樻垨杩涜閮ㄥ垎鍒锋柊鏃?*蹇呴』**閬靛畧鐨勭粷瀵规渶灏忓榻?*鍜?*
瀹藉害銆?

	姝?API 鍙兘杩斿洖涓€涓瘮瀹為檯缂撳瓨琛?*鏇村ぇ**鐨勬暟瀛楋紝浣嗗畠淇濊瘉涓€涓垨澶氫釜缂撳瓨琛屾伆濂?	閫傞厤鍒版娆¤皟鐢ㄨ繑鍥炵殑瀹藉害涓€傚畠涔熷皢濮嬬粓鏄?2 鐨勫箓锛屼究浜庡榻愩€?

### 绗笁閮ㄥ垎 - 璋冭瘯椹卞姩瀵?DMA API 鐨勪娇鐢?
濡備笂鎵€杩扮殑 DMA API 鏈変竴浜涚害鏉熴€備緥濡傦紝DMA 鍦板潃蹇呴』鐢ㄧ浉鍚屽ぇ灏忋€佺浉搴旂殑鍑芥暟閲婃斁銆傞殢鐫€
纭欢 IOMMU 鐨勫嚭鐜帮紝椹卞姩涓嶈繚鍙嶈繖浜涚害鏉熷彉寰楄秺鏉ヨ秺閲嶈銆傚湪鏈€鍧忔儏鍐典笅锛屾绫昏繚瑙勫彲鑳藉鑷?鏁版嵁鎹熷潖锛岀洿鑷虫懅姣佹枃浠剁郴缁熴€?
涓轰簡璋冭瘯椹卞姩骞跺彂鐜?DMA API 浣跨敤涓殑缂洪櫡锛屽彲浠ユ妸妫€鏌ヤ唬鐮佺紪璇戣繘鍐呮牳锛屽畠浼氭妸杩欑被杩濊
鍛婅瘔寮€鍙戣€呫€傚鏋滀綘鐨勬灦鏋勬敮鎸侊紝浣犲彲浠ュ湪鍐呮牳閰嶇疆涓€夋嫨 "Enable debugging of DMA API
usage" 閫夐」銆傚惎鐢ㄦ閫夐」浼氭湁鎬ц兘褰卞搷銆備笉瑕佸湪鐢熶骇鍐呮牳涓惎鐢ㄥ畠銆?
濡傛灉浣犲惎鍔紝寰楀埌鐨勫唴鏍稿皢鍖呭惈涓€浜涘璐︿唬鐮侊紝璁板綍涓哄摢涓澶囧垎閰嶄簡鍝簺 DMA 鍐呭瓨銆傚鏋?杩欐浠ｇ爜妫€娴嬪埌閿欒锛屽畠浼氬皢涓€鏉″甫鏈変竴浜涚粏鑺傜殑璀﹀憡娑堟伅鎵撳嵃鍒颁綘鐨勫唴鏍告棩蹇椾腑銆備竴涓?```

	WARNING: at /data2/repos/linux-2.6-iommu/lib/dma-debug.c:448
		check_unmap+0x203/0x490()
	Hardware name:
	forcedeth 0000:00:08.0: DMA-API: device driver frees DMA memory with wrong
		function [device address=0x00000000640444be] [size=66 bytes] [mapped as
		single] [unmapped as page]
	Modules linked in: nfsd exportfs bridge stp llc r8169
	Pid: 0, comm: swapper Tainted: G        W  2.6.28-dmatest-09289-g8bb99c0 #1
	Call Trace:
	<IRQ>  [<ffffffff80240b22>] warn_slowpath+0xf2/0x130
	[<ffffffff80647b70>] _spin_unlock+0x10/0x30
	[<ffffffff80537e75>] usb_hcd_link_urb_to_ep+0x75/0xc0
	[<ffffffff80647c22>] _spin_unlock_irqrestore+0x12/0x40
	[<ffffffff8055347f>] ohci_urb_enqueue+0x19f/0x7c0
	[<ffffffff80252f96>] queue_work+0x56/0x60
	[<ffffffff80237e10>] enqueue_task_fair+0x20/0x50
	[<ffffffff80539279>] usb_hcd_submit_urb+0x379/0xbc0
	[<ffffffff803b78c3>] cpumask_next_and+0x23/0x40
	[<ffffffff80235177>] find_busiest_group+0x207/0x8a0
	[<ffffffff8064784f>] _spin_lock_irqsave+0x1f/0x50
	[<ffffffff803c7ea3>] check_unmap+0x203/0x490
	[<ffffffff803c8259>] debug_dma_unmap_phys+0x49/0x50
	[<ffffffff80485f26>] nv_tx_done_optimized+0xc6/0x2c0
	[<ffffffff80486c13>] nv_nic_irq_optimized+0x73/0x2b0
	[<ffffffff8026df84>] handle_IRQ_event+0x34/0x70
	[<ffffffff8026ffe9>] handle_edge_irq+0xc9/0x150
	[<ffffffff8020e3ab>] do_IRQ+0xcb/0x1c0
	[<ffffffff8020c093>] ret_from_intr+0x0/0xa
	<EOI> <4>---[ end trace f6435a98e2a38c0e ]---

```
椹卞姩寮€鍙戣€呭彲浠ユ壘鍒伴┍鍔ㄥ拰璁惧锛屽寘鎷鑷存璀﹀憡鐨?DMA API 璋冪敤鐨勬爤鍥炴函銆?
榛樿鎯呭喌涓嬶紝鍙湁绗竴涓敊璇細瀵艰嚧璀﹀憡娑堟伅銆傛墍鏈夊叾浠栭敊璇彧浼氶粯榛樿鏁般€傚瓨鍦ㄨ繖涓€闄愬埗鏄负浜?闃叉浠ｇ爜娣规病浣犵殑鍐呮牳鏃ュ織銆備负浜嗘敮鎸佽皟璇曡澶囬┍鍔紝鍙互閫氳繃 debugfs 绂佺敤瀹冦€傝瑙佷笅鏂囩殑
debugfs 鎺ュ彛鏂囨。銆?
鐢ㄤ簬 DMA API 璋冭瘯浠ｇ爜鐨?debugfs 鐩綍鍚嶄负 dma-api/銆傚湪璇ョ洰褰曚笅锛岀洰鍓嶅彲浠ユ壘鍒颁互涓嬫枃浠讹細

=============================== ===============================================
dma-api/all_errors		姝ゆ枃浠跺惈涓€涓暟鍊笺€傚鏋滆鍊间笉涓?0锛岃皟璇曚唬鐮佷細涓?				瀹冨彂鐜扮殑姣忎釜閿欒鍚戝唴鏍告棩蹇楁墦鍗颁竴鏉¤鍛娿€傚皬蹇冧娇鐢ㄦ
				閫夐」锛屽洜涓哄畠寰堝鏄撴饭娌′綘鐨勬棩蹇椼€?
dma-api/disabled		姝ゅ彧璇绘枃浠跺湪璋冭瘯浠ｇ爜琚鐢ㄦ椂鍖呭惈瀛楃 'Y'銆傝繖鍙兘
				鍙戠敓鍦ㄥ畠鑰楀敖鍐呭瓨鏃讹紝鎴栧湪鍚姩鏃跺氨琚鐢ㄦ椂銆?
dma-api/dump			姝ゅ彧璇绘枃浠跺寘鍚綋鍓嶇殑 DMA 鏄犲皠銆?
dma-api/error_count		姝ゆ枃浠舵槸鍙鐨勶紝鏄剧ず鍙戠幇鐨勯敊璇€绘暟銆?
dma-api/num_errors		姝ゆ枃浠朵腑鐨勬暟瀛楁樉绀哄湪鍋滄涔嬪墠浼氬悜鍐呮牳鏃ュ織鎵撳嵃澶氬皯鏉?				璀﹀憡銆傝鏁板瓧鍦ㄧ郴缁熷惎鍔ㄦ椂琚垵濮嬪寲涓?1锛屽苟鍙€氳繃鍐欏叆
				姝ゆ枃浠舵潵璁剧疆銆?
dma-api/min_free_entries	姝ゅ彧璇绘枃浠跺彲璇诲嚭鍒嗛厤鍣ㄦ浘瑙佽繃鐨勬渶灏戠┖闂?				dma_debug_entries 鏁伴噺銆傚鏋滆鍊奸檷鍒?0锛屼唬鐮佷細灏濊瘯
				澧炲姞 nr_total_entries 鏉ヨˉ鍋裤€?
dma-api/num_free_entries	鍒嗛厤鍣ㄤ腑褰撳墠鐨勭┖闂?dma_debug_entries 鏁伴噺銆?
dma-api/nr_total_entries	鍒嗛厤鍣ㄤ腑 dma_debug_entries 鐨勬€绘暟锛屽寘鎷┖闂插拰宸茬敤銆?
dma-api/driver_filter		浣犲彲浠ユ妸涓€涓┍鍔ㄧ殑鍚嶅瓧鍐欏叆姝ゆ枃浠讹紝灏嗚皟璇曡緭鍑洪檺鍒朵负
				鏉ヨ嚜閭ｄ釜鐗瑰畾椹卞姩鐨勮姹傘€傚悜璇ユ枃浠跺啓鍏ョ┖瀛楃涓蹭互绂佺敤
				杩囨护鍣ㄥ苟鍐嶆鐪嬪埌鎵€鏈夐敊璇€?=============================== ===============================================

濡傛灉浣犳妸杩欐浠ｇ爜缂栬瘧杩涗簡鍐呮牳锛屽畠灏嗛粯璁よ鍚敤銆傚鏋滀綘鎯虫棤璁哄浣曢兘涓嶅甫瀵硅处鍚姩锛屽彲浠?鎻愪緵 'dma_debug=off' 浣滀负鍚姩鍙傛暟銆傝繖浼氱鐢?DMA API 璋冭瘯銆傛敞鎰忎綘鏃犳硶鍦ㄨ繍琛屾椂鍐嶆鍚敤瀹冦€?浣犲繀椤婚噸鍚墠鑳藉仛鍒般€?
濡傛灉浣犲彧鎯崇湅鍒版煇涓壒瀹氳澶囬┍鍔ㄧ殑璋冭瘯娑堟伅锛屽彲浠ユ寚瀹?dma_debug_driver=<drivername> 鍙傛暟銆?杩欎細鍦ㄥ惎鍔ㄦ椂鍚敤椹卞姩杩囨护鍣ㄣ€傛鍚庤皟璇曚唬鐮佸彧浼氭墦鍗拌椹卞姩鐨勯敊璇€傛杩囨护鍣ㄧ◢鍚庡彲浠ヤ娇鐢?debugfs 绂佺敤鎴栨洿鏀广€?
褰撲唬鐮佸湪杩愯鏃剁鐢ㄨ嚜韬椂锛屾渶鍙兘鏄洜涓哄畠鑰楀敖浜?dma_debug_entries锛屽苟涓旀棤娉曟寜闇€鍒嗛厤鏇村銆?鍚姩鏃堕鍒嗛厤浜?65536 涓潯鐩€斺€斿鏋滆繖瀵逛綘澶綆锛岃鐢?'dma_debug_entries=<浣犳湡鏈涚殑鏁板瓧>'
鍚姩浠ヨ鐩栭粯璁ゅ€笺€傛敞鎰忎唬鐮佹槸鎵归噺鍒嗛厤鏉＄洰鐨勶紝鍥犳棰勫垎閰嶆潯鐩殑纭垏鏁伴噺鍙兘澶т簬瀹為檯璇锋眰鐨?鏁伴噺銆傛瘡褰撲唬鐮佸姩鎬佸垎閰嶇殑鏉＄洰鏁拌揪鍒版渶鍒濋鍒嗛厤鐨勬暟閲忔椂锛屽畠浼氬悜鍐呮牳鏃ュ織鎵撳嵃涓€鏉℃秷鎭€傝繖鏄?涓轰簡琛ㄦ槑鍙兘闇€瑕佹洿澶х殑棰勫垎閰嶅ぇ灏忥紝鎴栬€呭鏋滆繖绉嶆儏鍐垫寔缁彂鐢燂紝鍒欒〃鏄庢煇涓┍鍔ㄥ彲鑳芥鍦ㄦ硠婕?鏄犲皠銆?
```

	void
	debug_dma_mapping_error(struct device *dev, dma_addr_t dma_addr);

```
dma-debug 鎺ュ彛 debug_dma_mapping_error() 鐢ㄤ簬璋冭瘯閭ｄ簺鏈兘妫€鏌?dma_map_single() 鍜?dma_map_page() 鎺ュ彛杩斿洖鍦板潃鐨?DMA 鏄犲皠閿欒鐨勯┍鍔ㄣ€傝鎺ュ彛娓呴櫎鐢?debug_dma_map_phys()
璁剧疆鐨勪竴涓爣蹇楋紝浠ヨ〃鏄庨┍鍔ㄥ凡缁忚皟鐢ㄤ簡 dma_mapping_error()銆傚綋椹卞姩瑙ｉ櫎鏄犲皠鏃讹紝
debug_dma_unmap() 妫€鏌ヨ鏍囧織锛屽鏋滃畠浠嶈璁剧疆锛屽垯鎵撳嵃涓€鏉″寘鍚€氬線瑙ｉ櫎鏄犲皠澶勭殑璋冪敤鏍堢殑
璀﹀憡娑堟伅銆傛鎺ュ彛鍙互浠?dma_mapping_error() 渚嬬▼涓皟鐢紝浠ュ惎鐢?DMA 鏄犲皠閿欒妫€鏌ヨ皟璇曘€?
## 鍑芥暟涓庣粨鏋勪綋


