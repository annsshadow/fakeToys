
## 鐗╃悊鍐呭瓨


Linux 鍙敤浜庤寖鍥村箍娉涚殑鏋舵瀯锛屽洜姝ら渶瑕佷竴绉嶄笌鏋舵瀯鏃犲叧鐨勬娊璞℃潵琛ㄧず鐗╃悊鍐呭瓨銆?鏈珷鎻忚堪浜嗗湪杩愯涓殑绯荤粺涓敤浜庣鐞嗙墿鐞嗗唴瀛樼殑缁撴瀯銆?
鍐呭瓨绠＄悊涓渶涓昏鐨勬蹇垫槸 `Non-Uniform Memory Access (NUMA)
<https://en.wikipedia.org/wiki/Non-uniform_memory_access>`_銆?鍦ㄥ鏍镐笌澶氳矾鎻掓Ы鐨勬満鍣ㄤ笂锛屽唴瀛樺彲鑳借鍒掑垎鎴愬涓瓨鍌ㄥ潡锛坆ank锛夛紝鏍规嵁涓?澶勭悊鍣ㄧ殑鈥滆窛绂烩€濊繙杩戯紝璁块棶瀹冧滑鎵€闇€鐨勪唬浠峰悇涓嶇浉鍚屻€備緥濡傦紝鍙兘涓烘瘡涓?CPU
鍒嗛厤涓€涓唴瀛樺潡锛屾垨鑰呭湪澶栧洿璁惧闄勮繎鏈変竴鍧楅潪甯搁€傚悎 DMA 鐨勫唴瀛樸€?
姣忎釜瀛樺偍鍧楄绉颁负涓€涓妭鐐癸紙node锛夛紝鍗充究鏋舵瀯鏄?UMA锛岃姒傚康鍦?Linux 涓嬩篃鐢?`struct pglist_data` 琛ㄧず銆傝缁撴瀯鎬绘槸浠ュ叾 typedef `pg_data_t` 琚紩鐢ㄣ€?鐗瑰畾鑺傜偣鐨?`pg_data_t` 缁撴瀯鍙€氳繃 `NODE_DATA(nid)` 瀹忔潵寮曠敤锛屽叾涓?`nid`
鏄鑺傜偣鐨?ID銆?
瀵逛簬 NUMA 鏋舵瀯锛岃妭鐐圭粨鏋勭敱鏋舵瀯鐩稿叧鐨勪唬鐮佸湪鍚姩鏃╂湡鍒嗛厤銆傞€氬父锛岃繖浜涚粨鏋?鍒嗛厤鍦ㄥ畠浠墍琛ㄧず鐨勯偅涓唴瀛樺潡鏈湴銆傚浜?UMA 鏋舵瀯锛屽彧浼氫娇鐢ㄤ竴涓悕涓?`contig_page_data` 鐨勯潤鎬?`pg_data_t` 缁撴瀯銆傝妭鐐瑰皢鍦?Section Nodes <nodes>
涓繘涓€姝ヨ璁恒€?
鏁翠釜鐗╃悊鍦板潃绌洪棿琚垝鍒嗘垚涓€涓垨澶氫釜绉颁负 zone锛堝尯锛夌殑鍧楋紝瀹冧滑琛ㄧず鍐呭瓨涓殑
鑼冨洿銆傝繖浜涜寖鍥撮€氬父鐢辫闂墿鐞嗗唴瀛樼殑鏋舵瀯绾︽潫鍐冲畾銆備竴涓妭鐐瑰唴瀵瑰簲浜庢煇涓壒瀹?zone 鐨勫唴瀛樿寖鍥寸敱 `struct zone` 鎻忚堪銆傛瘡涓?zone 鍏锋湁涓嬮潰鎻忚堪鐨勭被鍨嬩箣涓€銆?
- `ZONE_DMA` 涓?`ZONE_DMA32` 鍘嗗彶涓婅〃绀洪€傚悎鐢辨棤娉曡闂叏閮ㄥ彲瀵诲潃鍐呭瓨鐨?  澶栧洿璁惧杩涜 DMA 鐨勫唴瀛樸€傚骞存潵宸茬粡鏈変簡鏇村ソ銆佹洿鍋ュ．鐨勬帴鍙ｆ潵鑾峰彇婊¤冻
  DMA 鐗瑰畾瑕佹眰鐨勫唴瀛橈紙Documentation/core-api/dma-api.rst锛夛紝浣?`ZONE_DMA`
  鍜?`ZONE_DMA32` 浠嶇劧琛ㄧず鍦ㄥ浣曡璁块棶涓婂彈闄愮殑鍐呭瓨鑼冨洿銆備緷鎹灦鏋勭殑涓嶅悓锛?  杩欎袱绉?zone 绫诲瀷涔嬩竴銆佺敋鑷充袱鑰呴兘鍙互鍦ㄦ瀯寤烘椂閫氳繃 `CONFIG_ZONE_DMA` 鍜?  `CONFIG_ZONE_DMA32` 閰嶇疆閫夐」绂佺敤銆傛煇浜?64 浣嶅钩鍙板彲鑳介渶瑕佷袱涓?zone锛屽洜涓?  瀹冧滑鏀寔鍏锋湁涓嶅悓 DMA 瀵诲潃闄愬埗鐨勫鍥磋澶囥€?
- `ZONE_NORMAL` 鐢ㄤ簬鍐呮牳濮嬬粓鍙互璁块棶鐨勬櫘閫氬唴瀛樸€傚鏋?DMA 璁惧鏀寔浼犺緭鍒?  鎵€鏈夊彲瀵诲潃鍐呭瓨锛屽垯鍙湪璇?zone 鐨勯〉涓婃墽琛?DMA 鎿嶄綔銆俙ZONE_NORMAL` 濮嬬粓
  鍚敤銆?
- `ZONE_HIGHMEM` 鏄墿鐞嗗唴瀛樹腑鏈鍐呮牳椤佃〃姘镐箙鏄犲皠鎵€瑕嗙洊鐨勯儴鍒嗐€傝 zone 涓?  鐨勫唴瀛樺彧鑳介€氳繃涓存椂鏄犲皠琚唴鏍歌闂€傝 zone 浠呭湪鏌愪簺 32 浣嶆灦鏋勪笂鍙敤锛屽苟
  閫氳繃 `CONFIG_HIGHMEM` 鍚敤銆?
- `ZONE_MOVABLE` 鐢ㄤ簬鍙甯歌闂殑鍐呭瓨锛屽氨鍍?`ZONE_NORMAL` 涓€鏍枫€備笉鍚屼箣澶勫湪浜?  `ZONE_MOVABLE` 涓ぇ澶氭暟椤电殑鍐呭鏄彲绉诲姩鐨勩€傝繖鎰忓懗鐫€铏界劧杩欎簺椤电殑铏氭嫙鍦板潃
  涓嶅彉锛屼絾鍏跺唴瀹瑰彲鑳藉湪涓嶅悓鐗╃悊椤典箣闂寸Щ鍔ㄣ€傞€氬父 `ZONE_MOVABLE` 鏄湪鍐呭瓨鐑彃鎷?  鏈熼棿濉厖鐨勶紝浣嗕篃鍙互鍦ㄥ惎鍔ㄦ椂閫氳繃 `kernelcore`銆乣movablecore` 鍜?  `movable_node` 杩欏嚑涓唴鏍稿懡浠よ鍙傛暟涔嬩竴鏉ュ～鍏呫€傛洿澶氱粏鑺傚弬瑙?  Documentation/mm/page_migration.rst 涓?  Documentation/admin-guide/mm/memory-hotplug.rst銆?
- `ZONE_DEVICE` 琛ㄧず椹荤暀鍦ㄨ澶囷紙濡?PMEM 鍜?GPU锛変笂鐨勫唴瀛樸€傚畠涓?RAM zone 绫诲瀷
  鍏锋湁涓嶅悓鐨勭壒鎬э紝鍏跺瓨鍦ㄦ槸涓轰簡缁欒澶囬┍鍔ㄦ墍鏍囪瘑鐨勭墿鐞嗗湴鍧€鑼冨洿鎻愪緵 struct page
  <Pages> 涓庡唴瀛樻槧灏勶紙memory map锛夋湇鍔°€俙ZONE_DEVICE` 鐢遍厤缃€夐」
  `CONFIG_ZONE_DEVICE` 鍚敤銆?
闇€瑕佹敞鎰忥紝璁稿鍐呮牳鎿嶄綔鍙兘浣跨敤 `ZONE_NORMAL` 杩涜锛屽洜姝ゅ畠鏄€ц兘鏈€鍏抽敭鐨?zone銆倆one 灏嗗湪 Section Zones <zones> 涓繘涓€姝ヨ璁恒€?
鑺傜偣涓?zone 鑼冨洿涔嬮棿鐨勫叧绯荤敱鍥轰欢鎶ュ憡鐨勭墿鐞嗗唴瀛樻槧灏勩€佸唴瀛樺鍧€鐨勬灦鏋勭害鏉熶互鍙?鍐呮牳鍛戒护琛屼腑鐨勬煇浜涘弬鏁板喅瀹氥€?
渚嬪锛屽湪鍏锋湁 2 Gbytes RAM 鐨?x86 UMA 鏈哄櫒涓婅繍琛?32 浣嶅唴鏍告椂锛屾暣涓唴瀛樺皢浣嶄簬
鑺傜偣 0锛屽苟浼氭湁涓変釜 zone锛歚ZONE_DMA`銆?```

  0                                                            2G
  +-------------------------------------------------------------+
  |                            node 0                           |
  +-------------------------------------------------------------+

  0         16M                    896M                        2G
  +----------+-----------------------+--------------------------+
  | ZONE_DMA |      ZONE_NORMAL      |       ZONE_HIGHMEM       |
  +----------+-----------------------+--------------------------+


```
浣跨敤绂佺敤浜?`ZONE_DMA`銆佸惎鐢ㄤ簡 `ZONE_DMA32` 鐨勫唴鏍革紝骞跺湪鍏锋湁 16 Gbytes RAM銆?鍧囧寑鍒嗗竷浜庝袱涓妭鐐圭殑 arm64 鏈哄櫒涓婁互 `movablecore=80%` 鍙傛暟寮曞鏃讹紝鑺傜偣 0 涓?灏嗕細鏈?`ZONE_DMA32`銆乣ZONE_NORMAL` 鍜?`ZONE_MOVABLE`锛岃€岃妭鐐?1 涓婁細鏈?`ZONE_NORMAL` 鍜?```


  1G                                9G                         17G
  +--------------------------------+ +--------------------------+
  |              node 0            | |          node 1          |
  +--------------------------------+ +--------------------------+

  1G       4G        4200M          9G          9320M          17G
  +---------+----------+-----------+ +------------+-------------+
  |  DMA32  |  NORMAL  |  MOVABLE  | |   NORMAL   |   MOVABLE   |
  +---------+----------+-----------+ +------------+-------------+


```
鍐呭瓨鍧楀彲鑳藉睘浜庝氦閿欙紙interleaving锛夌殑鑺傜偣銆傚湪涓嬮潰杩欎釜渚嬪瓙涓紝涓€鍙?x86 鏈哄櫒鏈?16 Gbytes RAM锛屽垎甯冨湪 4 涓唴瀛樺潡涓紝鍋舵暟鍧楀睘浜庤妭鐐?0
```


  0              4G              8G             12G            16G
  +-------------+ +-------------+ +-------------+ +-------------+
  |    node 0   | |    node 1   | |    node 0   | |    node 1   |
  +-------------+ +-------------+ +-------------+ +-------------+

  0   16M      4G
  +-----+-------+ +-------------+ +-------------+ +-------------+
  | DMA | DMA32 | |    NORMAL   | |    NORMAL   | |    NORMAL   |
  +-----+-------+ +-------------+ +-------------+ +-------------+

```
鍦ㄨ繖绉嶆儏鍐典笅锛岃妭鐐?0 灏嗕粠 0 璺ㄥ埌 12 Gbytes锛岃妭鐐?1 灏嗕粠 4 璺ㄥ埌 16 Gbytes銆?

## 鑺傜偣


濡傚墠鎵€杩帮紝鍐呭瓨涓殑姣忎釜鑺傜偣鐢变竴涓?`pg_data_t` 鎻忚堪锛屽畠鏄?`struct pglist_data`
鐨?typedef銆傚湪鍒嗛厤涓€涓〉鏃讹紝榛樿鎯呭喌涓?Linux 浣跨敤鑺傜偣鏈湴锛坣ode-local锛夊垎閰?绛栫暐锛屼粠璺濈姝ｅ湪杩愯鐨?CPU 鏈€杩戠殑鑺傜偣鍒嗛厤鍐呭瓨銆傜敱浜庤繘绋嬪線寰€杩愯鍦ㄧ浉鍚岀殑 CPU
涓婏紝褰撳墠鑺傜偣鐨勫唴瀛樺緢鍙兘琚娇鐢ㄣ€傚垎閰嶇瓥鐣ュ彲鐢辩敤鎴锋帶鍒讹紝濡?Documentation/admin-guide/mm/numa_memory_policy.rst 涓墍杩般€?
澶у鏁?NUMA 鏋舵瀯缁存姢涓€涓寚鍚戣妭鐐圭粨鏋勭殑鎸囬拡鏁扮粍銆傚疄闄呯殑缁撴瀯鍦ㄥ惎鍔ㄦ棭鏈熺敱
鏋舵瀯鐩稿叧浠ｇ爜瑙ｆ瀽鍥轰欢鎶ュ憡鐨勭墿鐞嗗唴瀛樻槧灏勬椂鍒嗛厤銆傝妭鐐瑰垵濮嬪寲鐨勪富浣撻儴鍒嗙◢鍚庡湪
鍚姩娴佺▼涓敱 free_area_init() 鍑芥暟瀹屾垚锛岀◢鍚庡皢鍦?Section Initialization
<initialization> 涓弿杩般€?
闄や簡鑺傜偣缁撴瀯锛屽唴鏍歌繕缁存姢涓€涓О涓?`node_states` 鐨?`nodemask_t` 浣嶆帺鐮佹暟缁勩€?璇ユ暟缁勪腑鐨勬瘡涓綅鎺╃爜琛ㄧず涓€缁勫叿鏈?`enum node_states` 鎵€瀹氫箟鐗瑰畾灞炴€х殑鑺傜偣锛?
`N_POSSIBLE`
  璇ヨ妭鐐瑰彲鑳藉湪鏌愪釜鏃跺埢涓婄嚎锛坥nline锛夈€?`N_ONLINE`
  璇ヨ妭鐐瑰凡涓婄嚎銆?`N_NORMAL_MEMORY`
  璇ヨ妭鐐瑰叿鏈夊父瑙勫唴瀛樸€?`N_HIGH_MEMORY`
  璇ヨ妭鐐瑰叿鏈夊父瑙勬垨楂樼鍐呭瓨銆傚綋 `CONFIG_HIGHMEM` 琚鐢ㄦ椂锛屼笌
  `N_NORMAL_MEMORY` 鍒悕涓哄悓涓€鍚箟銆?`N_MEMORY`
  璇ヨ妭鐐瑰叿鏈夊唴瀛橈紙甯歌銆侀珮绔€佸彲绉诲姩锛夈€?`N_CPU`
  璇ヨ妭鐐瑰叿鏈変竴涓垨澶氫釜 CPU銆?`N_GENERIC_INITIATOR`
  璇ヨ妭鐐瑰叿鏈変竴涓垨澶氫釜 Generic Initiator銆?
瀵逛簬鍏锋湁涓婅堪灞炴€х殑姣忎釜鑺傜偣锛屼細鍦?`node_states[<property>]` 浣嶆帺鐮佷腑璁剧疆瀵瑰簲浜?璇ヨ妭鐐?ID 鐨勪綅銆?
```

  node_states[N_POSSIBLE]
  node_states[N_ONLINE]
  node_states[N_NORMAL_MEMORY]
  node_states[N_HIGH_MEMORY]
  node_states[N_MEMORY]
  node_states[N_CPU]

```
鍏充簬 nodemask 鍙墽琛岀殑鍚勭鎿嶄綔锛岃鍙傝€?`include/linux/nodemask.h`銆?
闄ゆ涔嬪锛宯odemask 杩樼敤浜庢彁渚涜妭鐐归亶鍘嗙殑瀹忥紝鍗?`for_each_node()` 涓?`for_each_online_node()`銆?
```

	for_each_online_node(nid) {
		pg_data_t *pgdat = NODE_DATA(nid);

		foo(pgdat);
	}

```
### 鑺傜偣缁撴瀯


鑺傜偣缁撴瀯 `struct pglist_data` 澹版槑浜?`include/linux/mmzone.h`銆傝繖閲屾垜浠畝瑕?鎻忚堪璇ョ粨鏋勭殑瀛楁锛?
#### 閫氱敤


`node_zones`
  璇ヨ妭鐐圭殑鍚勪釜 zone銆傚苟闈炴墍鏈?zone 閮藉彲鑳借濉厖锛屼絾杩欐槸瀹屾暣鐨勫垪琛ㄣ€傚畠琚鑺傜偣
  鐨?node_zonelists 浠ュ強鍏朵粬鑺傜偣鐨?node_zonelists 寮曠敤銆?
`node_zonelists`
  鎵€鏈夎妭鐐逛腑鎵€鏈?zone 鐨勫垪琛ㄣ€傝鍒楄〃瀹氫箟浜嗕紭鍏堜粠涓垎閰嶇殑 zone 椤哄簭銆?  `node_zonelists` 鐢?`mm/page_alloc.c` 涓殑 `build_zonelists()` 鍦ㄦ牳蹇冨唴瀛?  绠＄悊缁撴瀯鍒濆鍖栨湡闂村缓绔嬨€?
`nr_zones`
  璇ヨ妭鐐逛腑宸插～鍏?zone 鐨勬暟閲忋€?
`node_mem_map`
  瀵逛簬浣跨敤 FLATMEM 鍐呭瓨妯″瀷鐨?UMA 绯荤粺锛? 鍙疯妭鐐圭殑 `node_mem_map` 鏄〃绀烘瘡涓?  鐗╃悊椤靛抚锛坒rame锛夌殑 struct page 鏁扮粍銆?
`node_page_ext`
  瀵逛簬浣跨敤 FLATMEM 鍐呭瓨妯″瀷鐨?UMA 绯荤粺锛? 鍙疯妭鐐圭殑 `node_page_ext` 鏄?struct
  page 鎵╁睍椤圭殑鏁扮粍銆備粎鍦ㄥ惎鐢ㄤ簡 `CONFIG_PAGE_EXTENSION` 鐨勫唴鏍镐腑鍙敤銆?
`node_start_pfn`
  璇ヨ妭鐐逛腑璧峰椤靛抚鐨勯〉甯у彿锛坧age frame number锛夈€?
`node_present_pages`
  璇ヨ妭鐐逛腑瀛樺湪鐨勭墿鐞嗛〉鎬绘暟銆?
`node_spanned_pages`
  鐗╃悊椤佃寖鍥寸殑鎬诲ぇ灏忥紝鍖呭惈绌烘礊锛坔ole锛夈€?
`node_size_lock`
  淇濇姢鐢ㄤ簬瀹氫箟鑺傜偣鑼冨洿锛坋xtent锛夊瓧娈电殑閿併€備粎褰撹嚦灏戝惎鐢ㄤ簡 `CONFIG_MEMORY_HOTPLUG`
  鎴?`CONFIG_DEFERRED_STRUCT_PAGE_INIT` 鍏朵腑涔嬩竴鏃跺畾涔夈€俙pgdat_resize_lock()`
  涓?`pgdat_resize_unlock()` 琚彁渚涚敤浜庢搷浣?`node_size_lock`锛岃€屾棤闇€妫€鏌?  `CONFIG_MEMORY_HOTPLUG` 鎴?`CONFIG_DEFERRED_STRUCT_PAGE_INIT`銆?
`node_id`
  鑺傜偣鐨勮妭鐐?ID锛圢ID锛夛紝浠?0 寮€濮嬨€?
`totalreserve_pages`
  杩欐槸姣忚妭鐐逛繚鐣欑殑銆佸鐢ㄦ埛绌洪棿鍒嗛厤涓嶅彲鐢ㄧ殑椤点€?
`first_deferred_pfn`
  濡傛灉鍦ㄥぇ鍨嬫満鍣ㄤ笂鍐呭瓨鍒濆鍖栬寤惰繜锛屽垯杩欐槸闇€瑕佽鍒濆鍖栫殑绗竴涓?PFN銆備粎褰?  鍚敤浜?`CONFIG_DEFERRED_STRUCT_PAGE_INIT` 鏃跺畾涔夈€?
`deferred_split_queue`
  姣忚妭鐐圭殑宸ㄩ〉锛坔uge page锛夐槦鍒楋紝杩欎簺宸ㄩ〉鐨勬媶鍒嗚寤惰繜銆備粎褰撳惎鐢ㄤ簡
  `CONFIG_TRANSPARENT_HUGEPAGE` 鏃跺畾涔夈€?
`__lruvec`
  姣忚妭鐐圭殑 lruvec锛屾寔鏈?LRU 閾捐〃鍙婄浉鍏崇殑鍙傛暟銆備粎鍦ㄥ唴瀛?cgroup 琚鐢ㄦ椂浣跨敤銆?  涓嶅簲鐩存帴璁块棶瀹冿紝鑰屽簲鏀圭敤 `mem_cgroup_lruvec()` 鏉ユ煡鎵?lruvec銆?
#### 鍥炴敹鎺у埗


鍙﹁ Documentation/mm/page_reclaim.rst銆?
`kswapd`
  姣忚妭鐐圭殑 kswapd 鍐呮牳绾跨▼瀹炰緥銆?
`kswapd_wait`銆乣pfmemalloc_wait`銆乣reclaim_wait`
  鐢ㄤ簬鍚屾鍐呭瓨鍥炴敹浠诲姟鐨?workqueue銆?
`nr_writeback_throttled`
  鍥犵瓑寰呰剰椤靛洖鍐欒€岃鑺傛祦锛坱hrottled锛夌殑浠诲姟鏁伴噺銆?
`nr_reclaim_start`
  鍥炴敹琚妭娴佷互绛夊緟鍥炲啓鏈熼棿鍐欏叆鐨勯〉鏁般€?
`kswapd_order`
  鎺у埗 kswapd 灏濊瘯鍥炴敹鐨勯樁锛坥rder锛夈€?
`kswapd_highest_zoneidx`
  鐢?kswapd 鍥炴敹鐨勬渶楂?zone 绱㈠紩銆?
`kswapd_failures`
  kswapd 鏃犳硶鍥炴敹浠讳綍椤电殑杩愯娆℃暟銆?
`min_unmapped_pages`
  涓嶅彲琚洖鏀剁殑鏈槧灏勬枃浠舵敮鎾戯紙file backed锛夐〉鐨勬渶灏忔暟閲忋€傜敱
  `vm.min_unmapped_ratio` sysctl 鍐冲畾銆備粎褰撳惎鐢ㄤ簡 `CONFIG_NUMA` 鏃跺畾涔夈€?
`min_slab_pages`
  涓嶅彲琚洖鏀剁殑 SLAB 椤电殑鏈€灏忔暟閲忋€傜敱 `vm.min_slab_ratio` sysctl 鍐冲畾銆備粎褰?  鍚敤浜?`CONFIG_NUMA` 鏃跺畾涔夈€?
`flags`
  鎺у埗鍥炴敹琛屼负鐨勬爣蹇椼€?
#### 瑙勬暣锛圕ompaction锛夋帶鍒?

`kcompactd_max_order`
  kcompactd 搴斿皾璇曡揪鍒扮殑椤甸樁锛坧age order锛夈€?
`kcompactd_highest_zoneidx`
  鐢?kcompactd 杩涜瑙勬暣鐨勬渶楂?zone 绱㈠紩銆?
`kcompactd_wait`
  鐢ㄤ簬鍚屾鍐呭瓨瑙勬暣浠诲姟鐨?workqueue銆?
`kcompactd`
  姣忚妭鐐圭殑 kcompactd 鍐呮牳绾跨▼瀹炰緥銆?
`proactive_compact_trigger`
  鍐冲畾鏄惁鍚敤涓诲姩瑙勬暣锛坧roactive compaction锛夈€傜敱 `vm.compaction_proactiveness`
  sysctl 鎺у埗銆?
#### 缁熻


`per_cpu_nodestats`
  璇ヨ妭鐐圭殑姣?CPU VM 缁熻淇℃伅銆?
`vm_stat`
  璇ヨ妭鐐圭殑 VM 缁熻淇℃伅銆?

## 鍖猴紙Zones锛?

濡傚墠鎵€杩帮紝鍐呭瓨涓殑姣忎釜 zone 鐢?`struct zone` 鎻忚堪锛屽畠鏄叾鎵€灞炶妭鐐圭殑
`node_zones` 鏁扮粍鐨勪竴涓厓绱犮€俙struct zone` 鏄〉鍒嗛厤鍣紙page allocator锛夌殑鏍稿績
鏁版嵁缁撴瀯銆備竴涓?zone 琛ㄧず涓€娈电墿鐞嗗唴瀛樿寖鍥达紝骞跺彲鑳藉寘鍚┖娲炪€?
椤靛垎閰嶅櫒浣跨敤鐢卞唴瀛樺垎閰嶆寚瀹氱殑 GFP 鏍囧織锛堝弬瑙?mm-api-gfp-flags锛夋潵纭畾璇ュ唴瀛?鍒嗛厤鍙粠鑺傜偣涓殑鍝釜鏈€楂?zone 鍒嗛厤鍐呭瓨銆傞〉鍒嗛厤鍣ㄩ鍏堜粠璇?zone 鍒嗛厤鍐呭瓨锛涘鏋?椤靛垎閰嶅櫒鏃犳硶浠庤 zone 鍒嗛厤鍑烘墍璇锋眰鏁伴噺鐨勫唴瀛橈紝瀹冨皢浠庤妭鐐逛腑涓嬩竴涓緝浣庣殑 zone
鍒嗛厤锛屾杩囩▼涓€鐩村悜涓婅繘琛岋紝鐩村埌骞跺寘鎷渶浣庣殑 zone銆備緥濡傦紝濡傛灉涓€涓妭鐐瑰寘鍚?`ZONE_DMA32`銆乣ZONE_NORMAL` 鍜?`ZONE_MOVABLE`锛屼笖鏌愭鍐呭瓨鍒嗛厤鐨勬渶楂?zone 鏄?`ZONE_MOVABLE`锛屽垯椤靛垎閰嶅櫒浠庝腑鍒嗛厤鍐呭瓨鐨?zone 椤哄簭涓?`ZONE_MOVABLE` >
`ZONE_NORMAL` > `ZONE_DMA32`銆?
鍦ㄨ繍琛屾椂锛寊one 涓殑绌洪棽椤典綅浜庢瘡 CPU 椤甸泦锛圥er-CPU Pagesets锛孭CP锛夋垨璇?zone 鐨?绌洪棽鍖哄煙锛坒ree areas锛変腑銆傛瘡 CPU 椤甸泦鏄唴鏍稿唴瀛樼鐞嗙郴缁熶腑鐨勪竴椤瑰叧閿満鍒躲€傞€氳繃
鍦ㄦ瘡涓?CPU 涓婃湰鍦板鐞嗘渶棰戠箒鐨勫垎閰嶄笌閲婃斁锛屾瘡 CPU 椤甸泦鎻愬崌浜嗘€ц兘涓庡彲鎵╁睍鎬э紝
灏ゅ叾鏄湪鎷ユ湁浼楀鏍稿績鐨勭郴缁熶笂銆傚唴鏍镐腑鐨勯〉鍒嗛厤鍣ㄩ噰鐢ㄤ袱姝ョ瓥鐣ヨ繘琛屽唴瀛樺垎閰嶏紝鍏?浠庢瘡 CPU 椤甸泦寮€濮嬶紝鍐嶅洖閫€锛坒all back锛夊埌浼欎即鍒嗛厤鍣紙buddy allocator锛夈€傞〉鍦ㄦ瘡
CPU 椤甸泦涓庡叏灞€绌洪棽鍖哄煙锛堢敱浼欎即鍒嗛厤鍣ㄧ鐞嗭級涔嬮棿浠ユ壒澶勭悊鏂瑰紡杞Щ銆傝繖鏈€灏忓寲浜嗗
鍏ㄥ眬浼欎即鍒嗛厤鍣ㄩ绻佷氦浜掔殑寮€閿€銆?
鏋舵瀯鐩稿叧浠ｇ爜璋冪敤 free_area_init() 鏉ュ垵濮嬪寲 zone銆?
### 鍖虹粨鏋?

zone 缁撴瀯 `struct zone` 瀹氫箟浜?`include/linux/mmzone.h`銆傝繖閲屾垜浠畝瑕佹弿杩拌
缁撴瀯鐨勫瓧娈碉細

#### 閫氱敤


`_watermark`
  璇?zone 鐨勬按浣嶇嚎锛坵atermark锛夈€傚綋涓€涓?zone 涓┖闂查〉鐨勬暟閲忎綆浜?min 姘翠綅绾挎椂锛?  浼氬拷鐣?boosting锛屼竴娆″垎閰嶅彲鑳借Е鍙戠洿鎺ュ洖鏀讹紙direct reclaim锛変笌鐩存帴瑙勬暣
  锛坉irect compaction锛夛紝瀹冧篃鐢ㄤ簬闄愬埗鐩存帴鍥炴敹銆傚綋涓€涓?zone 涓┖闂查〉鐨勬暟閲忎綆浜?  low 姘翠綅绾挎椂锛屼細鍞ら啋 kswapd銆傚綋涓€涓?zone 涓┖闂查〉鐨勬暟閲忛珮浜?high 姘翠綅绾挎椂锛?  鑻?`sysctl_numa_balancing_mode` 鐨?`NUMA_BALANCING_MEMORY_TIERING` 浣嶆湭璁剧疆锛?  kswapd 浼氬仠姝㈠洖鏀讹紙涓€涓?zone 杈惧埌骞宠　锛夈€俻romo 姘翠綅绾跨敤浜庡唴瀛樺垎灞傦紙memory
  tiering锛変笌 NUMA 骞宠　銆傚綋涓€涓?zone 涓┖闂查〉鐨勬暟閲忛珮浜?promo 姘翠綅绾挎椂锛岃嫢
  `sysctl_numa_balancing_mode` 鐨?`NUMA_BALANCING_MEMORY_TIERING` 浣嶅凡璁剧疆锛?  kswapd 浼氬仠姝㈠洖鏀躲€傛按浣嶇嚎鐢?`__setup_per_zone_wmarks()` 璁剧疆銆俶in 姘翠綅绾挎牴鎹?  `vm.min_free_kbytes` sysctl 璁＄畻銆傚彟澶栦笁涓按浣嶇嚎鏍规嵁涓や釜姘翠綅绾夸箣闂寸殑璺濈璁剧疆銆?  璺濈鏈韩鐨勮绠椾細鑰冭檻 `vm.watermark_scale_factor` sysctl銆?
`watermark_boost`
  鐢ㄤ簬鎻愬崌姘翠綅绾跨殑椤垫暟锛屼互澧炲姞鍥炴敹鍘嬪姏鏉ラ檷浣庡皢鏉ュ彂鐢熷洖閫€锛坒allback锛夌殑鍙兘鎬э紝
  骞剁珛鍗冲敜閱?kswapd锛屽洜涓鸿鑺傜偣鏁翠綋涓婂彲鑳藉凡骞宠　锛宬swapd 涓嶄細鑷劧鍞ら啋銆?
`nr_reserved_highatomic`
  涓洪珮闃跺師瀛愶紙high-order atomic锛夊垎閰嶆墍淇濈暀鐨勯〉鏁般€?
`nr_free_highatomic`
  宸蹭繚鐣欑殑 highatomic pageblock 涓殑绌洪棽椤垫暟銆?
`lowmem_reserve`
  璇?zone 涓负鍐呭瓨鍒嗛厤鎵€淇濈暀鐨勫唴瀛樻暟閲忔暟缁勩€備緥濡傦紝濡傛灉鏌愭鍐呭瓨鍒嗛厤鍙粠涓垎閰?  鍐呭瓨鐨勬渶楂?zone 鏄?`ZONE_MOVABLE`锛屽垯鍦ㄥ皾璇曚粠璇?zone 鍒嗛厤鍐呭瓨鏃讹紝涓鸿鍒嗛厤鎵€
  淇濈暀鐨勫唴瀛橀噺鍗充负 `lowmem_reserve[ZONE_MOVABLE]`銆傝繖鏄〉鍒嗛厤鍣ㄧ敤鏉ラ槻姝㈡湰鍙娇鐢?  `highmem` 鐨勫垎閰嶅崰鐢ㄨ繃澶?`lowmem` 鐨勪竴绉嶆満鍒躲€傚浜?`highmem` 鏈哄櫒涓婃煇浜涗笓闂ㄧ殑
  宸ヤ綔璐熻浇锛屽唴鏍稿厑璁歌繘绋嬪唴瀛樹粠 `lowmem` zone 鍒嗛厤鏄嵄闄╃殑銆傝繖鏄洜涓洪偅鏍风殑鍐呭瓨闅忓悗
  鍙兘琚?`mlock()` 绯荤粺璋冪敤閽変綇锛坧in锛夛紝鎴栬€呭洜浜ゆ崲绌洪棿涓嶅彲鐢ㄨ€屾棤娉曞洖鏀躲€?  `vm.lowmem_reserve_ratio` sysctl 鍐冲畾浜嗗唴鏍稿湪鎹嶅崼杩欎簺杈冧綆 zone 鏃舵湁澶氭縺杩涖€傝
  鏁扮粍鍦ㄨ繍琛屾椂鐢?`setup_per_zone_lowmem_reserve()` 閲嶆柊璁＄畻锛屽墠鎻愭槸
  `vm.lowmem_reserve_ratio` sysctl 鍙戠敓鍙樺寲銆?
`node`
  璇?zone 鎵€灞炶妭鐐圭殑绱㈠紩銆備粎褰撳惎鐢ㄤ簡 `CONFIG_NUMA` 鏃跺彲鐢紝鍥犱负 UMA 绯荤粺涓彧鏈?  涓€涓?zone銆?
`zone_pgdat`
  鎸囧悜璇?zone 鎵€灞炶妭鐐圭殑 `struct pglist_data` 鐨勬寚閽堛€?
`per_cpu_pageset`
  鎸囧悜鐢?`setup_zone_pageset()` 鍒嗛厤骞跺垵濮嬪寲鐨勬瘡 CPU 椤甸泦锛圥CP锛夌殑鎸囬拡銆傞€氳繃
  鍦ㄦ瘡涓?CPU 涓婃湰鍦板鐞嗘渶棰戠箒鐨勫垎閰嶄笌閲婃斁锛孭CP 鍦ㄦ嫢鏈変紬澶氭牳蹇冪殑绯荤粺涓婃彁鍗囦簡鎬ц兘
  涓庡彲鎵╁睍鎬с€?
`pageset_high_min`
  澶嶅埗鍒版瘡 CPU 椤甸泦鐨?`high_min`锛屼互渚挎洿蹇闂€?
`pageset_high_max`
  澶嶅埗鍒版瘡 CPU 椤甸泦鐨?`high_max`锛屼互渚挎洿蹇闂€?
`pageset_batch`
  澶嶅埗鍒版瘡 CPU 椤甸泦鐨?`batch`锛屼互渚挎洿蹇闂€傛瘡 CPU 椤甸泦鐨?`batch`銆乣high_min`
  鍜?`high_max` 鐢ㄤ簬鍦ㄥ崟娆℃寔鏈夐攣鐨勬儏鍐典笅璁＄畻姣?CPU 椤甸泦浠庝紮浼村垎閰嶅櫒鑾峰彇鐨勫厓绱?  鏁伴噺锛屼互鎻愬崌鏁堢巼銆傚畠浠繕鐢ㄤ簬鍦ㄩ〉閲婃斁杩囩▼涓喅瀹氭槸鍚﹀皢椤佃繑鍥炵粰浼欎即鍒嗛厤鍣ㄣ€?
`pageblock_flags`
  鎸囧悜璇?zone 涓?pageblock 鏍囧織鐨勬寚閽堬紙鏍囧織鍒楄〃瑙?  `include/linux/pageblock-flags.h`锛夈€傚唴瀛樺湪 `setup_usemap()` 涓垎閰嶃€傛瘡涓?  pageblock 鍗犵敤 `NR_PAGEBLOCK_BITS` 浣嶃€備粎褰撳惎鐢ㄤ簡 `CONFIG_FLATMEM` 鏃跺畾涔夈€?  褰撳惎鐢ㄤ簡 `CONFIG_SPARSEMEM` 鏃讹紝鏍囧織瀛樺偍鍦?`mem_section` 涓€?
`zone_start_pfn`
  zone 鐨勮捣濮?pfn銆傜敱 `calculate_node_totalpages()` 鍒濆鍖栥€?
`managed_pages`
  鐢变紮浼寸郴缁熺鐞嗙殑瀛樺湪椤碉紙present pages锛夛紝璁＄畻鏂瑰紡涓猴細`managed_pages` =
  `present_pages` - `reserved_pages`锛屽叾涓?`reserved_pages` 鍖呭惈鐢?memblock
  鍒嗛厤鍣ㄥ垎閰嶇殑椤点€傚畠搴旂敱椤靛垎閰嶅櫒涓?vm 鎵弿鍣ㄧ敤鏉ヨ绠楀悇绉嶆按浣嶇嚎涓庨槇鍊笺€傚畠浣跨敤
  `atomic_long_xxx()` 鍑芥暟璁块棶銆傚畠鍦?`free_area_init_core()` 涓垵濮嬪寲锛岀劧鍚庡湪
  memblock 鍒嗛厤鍣ㄥ皢椤甸噴鏀惧洖浼欎即绯荤粺鏃堕噸鏂板垵濮嬪寲銆?
`spanned_pages`
  璇?zone 鎵€璺ㄨ秺鐨勬€婚〉鏁帮紝鍖呭惈绌烘礊锛岃绠楁柟寮忎负锛歚spanned_pages` = `zone_end_pfn`
  - `zone_start_pfn`銆傜敱 `calculate_node_totalpages()` 鍒濆鍖栥€?
`present_pages`
  璇?zone 涓瓨鍦ㄧ殑鐗╃悊椤碉紝璁＄畻鏂瑰紡涓猴細`present_pages` = `spanned_pages` -
  `absent_pages`锛堢┖娲炰腑鐨勯〉锛夈€傚畠鍙鍐呭瓨鐑彃鎷旀垨鍐呭瓨鐢垫簮绠＄悊閫昏緫閫氳繃妫€鏌?  锛坄present_pages` - `managed_pages`锛夋潵鎺ㄧ畻鏈彈绠＄悊鐨勯〉銆傝繍琛屾椂瀵?  `present_pages` 鐨勫啓璁块棶搴旂敱 `mem_hotplug_begin/done()` 淇濇姢銆備换浣曟棤娉曞蹇?  `present_pages` 婕傜Щ鐨勮鍙栬€呭簲浣跨敤 `get_online_mems()` 鑾峰彇绋冲畾鐨勫€笺€傚畠鐢?  `calculate_node_totalpages()` 鍒濆鍖栥€?
`present_early_pages`
  璇?zone 涓綅浜庤嚜鏃╂湡鍚姩鍗冲彲鐢ㄥ唴瀛樹笂鐨勫瓨鍦ㄩ〉锛屼笉鍖呮嫭鐑彃鎷斿唴瀛樸€備粎褰撳惎鐢ㄤ簡
  `CONFIG_MEMORY_HOTPLUG` 鏃跺畾涔夛紝骞剁敱 `calculate_node_totalpages()` 鍒濆鍖栥€?
`cma_pages`
  涓?CMA 浣跨敤鎵€淇濈暀鐨勯〉銆傚綋杩欎簺椤垫湭琚敤浜?CMA 鏃讹紝瀹冧滑鐨勮涓虹被浼间簬
  `ZONE_MOVABLE`銆備粎褰撳惎鐢ㄤ簡 `CONFIG_CMA` 鏃跺畾涔夈€?
`name`
  zone 鐨勫悕绉般€傚畠鏄寚鍚?`zone_names` 鏁扮粍瀵瑰簲鍏冪礌鐨勬寚閽堛€?
`nr_isolate_pageblock`
  宸查殧绂荤殑 pageblock 鏁伴噺銆傚畠鐢ㄤ簬瑙ｅ喅鐢变簬绔炰簤鍦拌幏鍙?pageblock 鐨勮縼绉荤被鍨?  锛坢igratetype锛夎€屽鑷寸殑绌洪棽椤佃鏁颁笉姝ｇ‘鐨勯棶棰樸€傜敱 `zone->lock` 淇濇姢銆備粎褰?  鍚敤浜?`CONFIG_MEMORY_ISOLATION` 鏃跺畾涔夈€?
`span_seqlock`
  淇濇姢 `zone_start_pfn` 涓?`spanned_pages` 鐨勯『搴忛攣锛坰eqlock锛夈€傚畠鏄『搴忛攣锛屽洜涓?  蹇呴』鍦?`zone->lock` 涔嬪璇诲彇锛屽苟涓旀槸鍦ㄤ富鍒嗛厤鍣ㄨ矾寰勪腑杩涜鐨勩€備笉杩囷紝璇ラ『搴忛攣鐨?  鍐欏叆鐩稿綋涓嶉绻併€備粎褰撳惎鐢ㄤ簡 `CONFIG_MEMORY_HOTPLUG` 鏃跺畾涔夈€?
`initialized`
  鎸囩ず璇?zone 鏄惁宸插垵濮嬪寲鐨勬爣蹇椼€傚湪鍚姩鏈熼棿鐢?`init_currently_empty_zone()`
  璁剧疆銆?
`free_area`
  绌洪棽鍖哄煙鏁扮粍锛屽叾涓瘡涓厓绱犲搴斾簬涓€涓壒瀹氱殑闃讹紙order锛夛紝鍗?2 鐨勫箓銆備紮浼村垎閰嶅櫒
  浣跨敤璇ョ粨鏋勬潵楂樻晥绠＄悊绌洪棽鍐呭瓨銆傚垎閰嶆椂锛屽畠灏濊瘯瀵绘壘鏈€灏忕殑瓒冲鍧楋紱濡傛灉鏈€灏忕殑瓒冲
  鍧楀ぇ浜庤姹傜殑澶у皬锛屽畠灏嗚閫掑綊鍦版媶鍒嗘垚鏇村皬鐨勪笅涓€绾у潡锛岀洿鍒拌揪鍒版墍闇€澶у皬銆傚綋閲婃斁
  涓€椤垫椂锛屽畠鍙兘涓庡叾浼欎即锛坆uddy锛夊悎骞跺舰鎴愪竴涓洿澶х殑鍧椼€傚畠鐢?`zone_init_free_lists()`
  鍒濆鍖栥€?
`unaccepted_pages`
  寰呮帴鍙楋紙accept锛夌殑椤靛垪琛ㄣ€傚垪琛ㄤ腑鐨勬墍鏈夐〉閮芥槸 `MAX_PAGE_ORDER`銆備粎褰撳惎鐢ㄤ簡
  `CONFIG_UNACCEPTED_MEMORY` 鏃跺畾涔夈€?
`flags`
  zone 鐨勬爣蹇椼€傛渶浣庝笁浣嶈浣跨敤锛岀敱 `enum zone_flags` 瀹氫箟銆俙ZONE_BOOSTED_WATERMARK`
  锛堜綅 0锛夛細zone 鏈€杩戞彁鍗囦簡姘翠綅绾裤€傚湪鍞ら啋 kswapd 鏃舵竻闄ゃ€俙ZONE_RECLAIM_ACTIVE`
  锛堜綅 1锛夛細kswapd 鍙兘姝ｅ湪鎵弿璇?zone銆俙ZONE_BELOW_HIGH`锛堜綅 2锛夛細zone 浣庝簬 high
  姘翠綅绾裤€?
`lock`
  淇濇姢椤靛垎閰嶅櫒鐗瑰畾浜庤 zone 鐨勫唴閮ㄦ暟鎹粨鏋勭殑涓昏閿侊紝灏ゅ叾淇濇姢 `free_area`銆?
`percpu_drift_mark`
  褰撶┖闂查〉鏁颁綆浜庢鐐规椂锛屽湪璇诲彇绌洪棽椤垫暟閲忔椂浼氶噰鍙栭澶栨楠わ紝浠ラ伩鍏嶆瘡 CPU 璁℃暟鍣?  婕傜Щ瀵艰嚧姘翠綅绾胯绐佺牬銆傚畠鍦?`refresh_zone_stat_thresholds()` 涓洿鏂般€?
#### 瑙勬暣鎺у埗


`compact_cached_free_pfn`
  瑙勬暣绌洪棽鎵弿鍣紙free scanner锛夊湪涓嬩竴娆℃壂鎻忎腑搴斿紑濮嬬殑浣嶇疆 PFN銆?
`compact_cached_migrate_pfn`
  瑙勬暣杩佺Щ鎵弿鍣紙migration scanner锛夊湪涓嬩竴娆℃壂鎻忎腑搴斿紑濮嬬殑浣嶇疆 PFN銆傝鏁扮粍鏈変袱涓?  鍏冪礌锛氱涓€涓敤浜?`MIGRATE_ASYNC` 妯″紡锛屽彟涓€涓敤浜?`MIGRATE_SYNC` 妯″紡銆?
`compact_init_migrate_pfn`
  鍒濆杩佺Щ PFN锛屽湪鍚姩鏃惰鍒濆鍖栦负 0锛屽湪瀹屾暣瑙勬暣缁撴潫鍚庤鍒濆鍖栦负璇?zone 涓叿鏈?  鍙縼绉婚〉鐨勭涓€涓?pageblock銆傚畠鐢ㄤ簬妫€鏌ヤ竴娆℃壂鎻忔槸鍚︿负鏁翠釜 zone 鐨勬壂鎻忋€?
`compact_init_free_pfn`
  鍒濆绌洪棽 PFN锛屽湪鍚姩鏃惰鍒濆鍖栦负 0锛屽苟琚垵濮嬪寲涓鸿 zone 涓叿鏈夌┖闂?  `MIGRATE_MOVABLE` 椤电殑鏈€鍚庝竴涓?pageblock銆傚畠鐢ㄤ簬妫€鏌ユ槸鍚︿负涓€娆℃壂鎻忕殑璧风偣銆?
`compact_considered`
  鑷笂娆″け璐ヤ互鏉ュ凡灏濊瘯鐨勮鏁存鏁般€傚綋涓€娆¤鏁存湭鑳芥垚鍔熷垎閰嶅嚭椤垫椂锛屽畠鍦?  `defer_compaction()` 涓閲嶇疆銆傚綋涓€娆¤鏁村簲琚烦杩囨椂锛屽畠鍦?`compaction_deferred()`
  涓姞 1銆俙compaction_deferred()` 鍦?`compact_zone()` 琚皟鐢ㄤ箣鍓嶈皟鐢紝
  `compaction_defer_reset()` 鍦?`compact_zone()` 杩斿洖 `COMPACT_SUCCESS` 鏃惰皟鐢紝
  `defer_compaction()` 鍦?`compact_zone()` 杩斿洖 `COMPACT_PARTIAL_SKIPPED` 鎴?  `COMPACT_COMPLETE` 鏃惰皟鐢ㄣ€?
`compact_defer_shift`
  鍦ㄥ啀娆″皾璇曚箣鍓嶈璺宠繃鐨勮鏁存鏁颁负 `1<<compact_defer_shift`銆傚畠鍦?`defer_compaction()`
  涓姞 1銆傚畠鍦?`compaction_defer_reset()` 涓紝褰撲竴娆＄洿鎺ヨ鏁存垚鍔熷垎閰嶅嚭椤垫椂琚噸缃€?  鍏舵渶澶у€间负 `COMPACT_MAX_DEFER_SHIFT`銆?
`compact_order_failed`
  鏈€灏忕殑瑙勬暣澶辫触闃躲€傚畠鍦ㄤ竴娆¤鏁存垚鍔熸椂浜?`compaction_defer_reset()` 涓缃紝骞跺湪
  涓€娆¤鏁存湭鑳芥垚鍔熷垎閰嶅嚭椤垫椂浜?`defer_compaction()` 涓缃€?
`compact_blockskip_flush`
  褰撹鏁磋縼绉绘壂鎻忓櫒涓庣┖闂叉壂鎻忓櫒鐩搁亣鏃惰涓?true锛岃繖鎰忓懗鐫€ `PB_compact_skip` 浣嶅簲琚?  娓呴櫎銆?
`contiguous`
  褰撹 zone 鏄繛缁殑锛堟崲瑷€涔嬶紝鏃犵┖娲烇級鏃惰涓?true銆?
#### 缁熻


`vm_stat`
  璇?zone 鐨?VM 缁熻淇℃伅銆傛墍杩借釜鐨勯」鐩敱 `enum zone_stat_item` 瀹氫箟銆?
`vm_numa_event`
  璇?zone 鐨?VM NUMA 浜嬩欢缁熻淇℃伅銆傛墍杩借釜鐨勯」鐩敱 `enum numa_stat_item` 瀹氫箟銆?
`per_cpu_zonestats`
  璇?zone 鐨勬瘡 CPU VM 缁熻淇℃伅銆傚畠鎸夋瘡 CPU 鏂瑰紡璁板綍 VM 缁熻淇℃伅涓?VM NUMA 浜嬩欢
  缁熻淇℃伅銆傚畠鍑忓皯浜嗗璇?zone 鍏ㄥ眬 `vm_stat` 涓?`vm_numa_event` 瀛楁鐨勬洿鏂帮紝浠ユ彁鍗?  鎬ц兘銆?

## 椤碉紙Pages锛?


   鏈妭灏氭湭瀹屾垚銆傝鍒楀嚭骞舵弿杩扮浉搴旂殑瀛楁銆?

## 澶ч〉锛團olios锛?


   鏈妭灏氭湭瀹屾垚銆傝鍒楀嚭骞舵弿杩扮浉搴旂殑瀛楁銆?

## 鍒濆鍖栵紙Initialization锛?


   鏈妭灏氭湭瀹屾垚銆傝鍒楀嚭骞舵弿杩扮浉搴旂殑瀛楁銆?