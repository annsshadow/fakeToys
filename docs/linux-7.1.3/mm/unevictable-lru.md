## 涓嶅彲鍥炴敹 LRU 鍩虹璁炬柦




## 绠€浠?

鏈枃妗ｆ弿杩?Linux 鍐呭瓨绠＄悊鍣ㄧ殑"涓嶅彲鍥炴敹 LRU"锛圲nevictable LRU锛夊熀纭€璁炬柦锛屼互鍙婂埄鐢ㄥ畠鏉ョ鐞嗗嚑绉嶇被鍨嬬殑"涓嶅彲鍥炴敹"椤靛抚锛坒olio锛夈€?
鏈枃妗ｈ瘯鍥炬彁渚涜繖涓€鏈哄埗鑳屽悗鐨勬暣浣撹璁′緷鎹紝浠ュ強椹卞姩瀹炵幇鐨勪竴浜涜璁″喅绛栬儗鍚庣殑渚濇嵁銆傚悗鑰呯殑璁捐渚濇嵁鍦ㄥ疄鐜扮殑涓婁笅鏂囦腑鍔犱互璁ㄨ銆傛棤鍙惁璁わ紝閫氳繃闃呰浠ｇ爜鍗冲彲鑾峰緱瀹炵幇缁嗚妭鈥斺€斿嵆"瀹冨仛浜嗕粈涔?銆備綔鑰呭笇鏈涗笅闈㈢殑鎻忚堪鑳藉閫氳繃鍥炵瓟"瀹冧负浣曢偅鏍峰仛锛?鑰屾彁渚涢澶栫殑浠峰€笺€?


## 涓嶅彲鍥炴敹 LRU


涓嶅彲鍥炴敹 LRU 鏈哄埗鏂板浜嗕竴涓澶栫殑 LRU 閾捐〃锛岀敤浜庤窡韪笉鍙洖鏀剁殑 folio锛屽苟灏嗚繖浜?folio 瀵?vmscan 闅愯棌銆傝鏈哄埗鍩轰簬 Red Hat 鐨?Larry Woodman 鐨勪竴涓ˉ涓侊紝鏃ㄥ湪瑙ｅ喅 Linux 涓?folio 鍥炴敹鐨勮嫢骞插彲鎵╁睍鎬ч棶棰樸€傝繖浜涢棶棰樺凡鍦ㄥ鎴风殑澶у瀷鍐呭瓨 x86_64 绯荤粺涓婅瀵熷埌銆?
涓句緥璇存槑锛屼竴鍙版嫢鏈?128GB 涓诲瓨鐨勯潪 NUMA x86_64 骞冲彴鍦ㄥ崟涓妭鐐逛笂浼氭湁瓒呰繃 3200 涓囦釜 4k 椤点€傚綋杩欎簺椤典腑鏈夊緢澶т竴閮ㄥ垎鍥犱换浣曞師鍥犺€屼笉鍙洖鏀禰瑙佷笅鏂嘳鏃讹紝vmscan 灏嗚姳璐瑰ぇ閲忔椂闂存壂鎻?LRU 閾捐〃浠ュ鎵鹃偅灏忛儴鍒嗗彲鍥炴敹鐨勯〉銆傝繖鍙兘瀵艰嚧杩欐牱涓€绉嶆儏鍐碉細鎵€鏈?CPU 杩炵画鏁板皬鏃舵垨鏁板ぉ灏?100% 鐨勬椂闂磋€楄垂鍦?vmscan 涓紝绯荤粺瀹屽叏鏃犲搷搴斻€?
涓嶅彲鍥炴敹閾捐〃澶勭悊浜嗕互涓嬪嚑绫讳笉鍙洖鏀堕〉锛?
 - 鐢?ramfs 鎷ユ湁鐨勯〉銆?
 - 鐢卞甫鏈?noswap 鎸傝浇閫夐」鐨?tmpfs 鎷ユ湁鐨勯〉銆?
 - 鏄犲皠鍒?SHM_LOCK 鐨勫叡浜唴瀛樺尯鍩熺殑椤点€?
 - 鏄犲皠鍒?VM_LOCKED [mlock()ed] VMA 鐨勯〉銆?
璇ュ熀纭€璁炬柦鏈潵鎴栬杩樿兘澶勭悊鍥犲畾涔夋垨鐜鑰屼娇寰楅〉涓嶅彲鍥炴敹鐨勫叾浠栨儏鍐点€?

### 涓嶅彲鍥炴敹 LRU 鐨?folio 閾捐〃


涓嶅彲鍥炴敹 LRU folio 閾捐〃鏄釜"璋庤█"銆傚畠浠庢潵灏变笉鏄竴涓寜 LRU 鎺掑簭鐨勯摼琛紝鑰屾槸涓庢寜 LRU 鎺掑簭鐨勫尶鍚嶅拰鏂囦欢銆佹椿璺冨拰闈炴椿璺?folio 閾捐〃鐩镐即鑰岀敓鐨勶紱鑰屽浠婂畠鐢氳嚦涓嶅啀鏄?folio 閾捐〃銆備絾閬靛惊鎯敤绾﹀畾锛屽湪鏈枃妗ｅ拰婧愮爜涓紝鎴戜滑缁忓父鎶婂畠鎯宠薄鎴愮浜斾釜 LRU folio 閾捐〃銆?
涓嶅彲鍥炴敹 LRU 鍩虹璁炬柦鍖呭惈涓€涓澶栫殑銆佹瘡鑺傜偣鐨?LRU 閾捐〃锛岀О涓?unevictable"閾捐〃锛屼互鍙婁竴涓浉鍏崇殑 folio 鏍囧織 PG_unevictable锛岀敤浜庤〃鏄庤 folio 姝ｇ敱涓嶅彲鍥炴敹閾捐〃绠＄悊銆?
PG_unevictable 鏍囧織绫讳技浜?PG_active 鏍囧織涓斾笌涔嬩簰鏂ワ紝鍥犱负褰?PG_lru 琚疆浣嶆椂锛屽畠琛ㄧず folio 浣嶄簬鍝釜 LRU 閾捐〃涓娿€?
涓嶅彲鍥炴敹 LRU 鍩虹璁炬柦灏嗕笉鍙洖鏀?folio 褰撲綔瀹冧滑鍦ㄩ澶栫殑 LRU 閾捐〃涓婁竴鏍锋潵缁存姢锛屽師鍥犳湁鍑犵偣锛?
 (1) 鎴戜滑鍙互"鍍忓寰呯郴缁熶腑鍏朵粬 folio 涓€鏍峰寰呬笉鍙洖鏀?folio鈥斺€旇繖鎰忓懗鐫€鎴戜滑鍙互浣跨敤鐩稿悓鐨勪唬鐮佹潵鎿嶄綔瀹冧滑銆佷娇鐢ㄧ浉鍚岀殑浠ｇ爜鏉ラ殧绂诲畠浠紙鐢ㄤ簬杩佺Щ绛夛級銆佷娇鐢ㄧ浉鍚岀殑浠ｇ爜鏉ヨ窡韪粺璁′俊鎭瓑鈥︹€? [Rik van Riel]

 (2) 鎴戜滑甯屾湜鑳藉鍦ㄨ妭鐐逛箣闂磋縼绉讳笉鍙洖鏀?folio锛屼互杩涜鍐呭瓨纰庣墖鏁寸悊銆佸伐浣滆礋杞界鐞嗗拰鍐呭瓨鐑彃鎷斻€侺inux 鍐呮牳鍙兘杩佺Щ閭ｄ簺鑳藉鎴愬姛浠?LRU 閾捐〃闅旂鍑烘潵鐨?folio锛堟垨"鍙Щ鍔?鐨?folio锛氳繖閲屼笉鍦ㄨ€冭檻鑼冨洿鍐咃級銆傚鏋滄垜浠妸杩欎簺 folio 缁存姢鍦?LRU 绫婚摼琛ㄤ箣澶栵紙鍗?folio_isolate_lru() 鏃犳硶妫€娴嬪埌鐨勫湴鏂癸級锛屽氨浼氶樆姝㈠畠浠殑杩佺Щ銆?
涓嶅彲鍥炴敹閾捐〃涓嶅尯鍒嗘枃浠跺悗澶囦笌鍖垮悕銆乻wap 鍚庡鐨?folio銆傝繖绉嶅尯鍒嗕粎鍦?folio 纭疄鍙洖鏀舵椂鎵嶆湁鎰忎箟銆?
涓嶅彲鍥炴敹閾捐〃鍙楃泭浜?Christoph Lameter 鏈€鍒濇彁鍑哄苟鍙戝竷鐨勬瘡鑺傜偣 LRU 閾捐〃涓庣粺璁＄殑"鏁扮粍鍖?锛坅rrayification锛夈€?

### 鍐呭瓨鎺у埗缁勪氦浜?

涓嶅彲鍥炴敹 LRU 鏈哄埗閫氳繃鎵╁睍 lru_list 鏋氫妇锛屼笌鍐呭瓨鎺у埗缁刐鍗?memory controller锛涜 Documentation/admin-guide/cgroup-v1/memory.rst]浜や簰銆?
鐢变簬姣忚妭鐐?LRU 閾捐〃鐨?鏁扮粍鍖?锛堟瘡涓?lru_list 鏋氫妇鍏冪礌瀵瑰簲涓€涓級锛屽唴瀛樻帶鍒跺櫒鏁版嵁缁撴瀯浼氳嚜鍔ㄨ幏寰椾竴涓瘡鑺傜偣鐨勪笉鍙洖鏀堕摼琛ㄣ€傚唴瀛樻帶鍒跺櫒璺熻釜椤佃繘鍑轰笉鍙洖鏀堕摼琛ㄧ殑绉诲姩銆?
褰撴煇涓唴瀛樻帶鍒剁粍闈复鍐呭瓨鍘嬪姏鏃讹紝鎺у埗鍣ㄤ笉浼氬皾璇曞洖鏀朵笉鍙洖鏀堕摼琛ㄤ笂鐨勯〉銆傝繖鏈夊嚑涓晥鏋滐細

 (1) 鍥犱负杩欎簺椤靛湪涓嶅彲鍥炴敹閾捐〃涓婂鍥炴敹"闅愯棌"锛屽洖鏀惰繃绋嬪彲浠ユ洿楂樻晥锛屽彧澶勭悊閭ｄ簺鏈夊彲鑳借鍥炴敹鐨勯〉銆?
 (2) 鍙︿竴鏂归潰锛屽鏋滆鍏ヨ鎺у埗缁勭殑椤典腑鏈夊お澶氫笉鍙洖鏀讹紝璇ユ帶鍒剁粍浠诲姟鐨?working set 涓彲鍥炴敹鐨勯儴鍒嗗彲鑳芥棤娉曟斁鍏ュ彲鐢ㄥ唴瀛樸€傝繖鍙兘瀵艰嚧鎺у埗缁勫彂鐢熼绨革紙thrash锛夋垨瀵逛换鍔℃墽琛?OOM-kill銆?

### 灏嗗湴鍧€绌洪棿鏍囪涓轰笉鍙洖鏀?

瀵逛簬鍍?ramfs 杩欐牱鐨勮鏂斤紝闄勫姞鍒板湴鍧€绌洪棿鐨勯〉閮戒笉鍙鍥炴敹銆備负浜嗛槻姝换浣曟绫婚〉琚洖鏀讹紝鎻愪緵浜?AS_UNEVICTABLE 鍦板潃绌洪棿鏍囧織锛屾枃浠剁郴缁熷彲浠ヤ娇鐢ㄨ嫢骞插寘瑁呭嚱鏁版潵鎿嶄綔瀹冿細

 - `void mapping_set_unevictable(struct address_space *mapping);`

	灏嗚鍦板潃绌洪棿鏍囪涓哄畬鍏ㄤ笉鍙洖鏀躲€?
 - `void mapping_clear_unevictable(struct address_space *mapping);`

	灏嗚鍦板潃绌洪棿鏍囪涓哄彲鍥炴敹銆?
 - `int mapping_unevictable(struct address_space *mapping);`

	鏌ヨ璇ュ湴鍧€绌洪棿锛屽鏋滃畠瀹屽叏涓嶅彲鍥炴敹鍒欒繑鍥?true銆?
杩欎簺鍑芥暟鐩墠鍦ㄥ唴鏍哥殑涓変釜鍦版柟浣跨敤锛?
 (1) 鐢?ramfs 鍦ㄥ叾 inode 鍒涘缓鏃舵爣璁板叾鍦板潃绌洪棿锛岃鏍囪鍦?inode 鐨勬暣涓敓鍛藉懆鏈熷唴淇濇寔銆?
 (2) 鐢?SYSV SHM 鏍囪 SHM_LOCK 鐨勫湴鍧€绌洪棿锛岀洿鍒拌皟鐢?SHM_UNLOCK銆傛敞鎰忥紝濡傛灉閿佸畾鐨勯〉宸茶鎹㈠嚭锛孲HM_LOCK 骞朵笉瑕佹眰灏嗗畠浠皟椤靛叆鍐呭瓨锛涘簲鐢ㄧ▼搴忚嫢鎯崇‘淇濆畠浠湪鍐呭瓨涓紝蹇呴』鎵嬪姩璁块棶杩欎簺椤点€?
 (3) 鐢?i915 椹卞姩鏍囪琚浐瀹氱殑鍦板潃绌洪棿锛岀洿鍒板叾琚В闄ゅ浐瀹氥€俰915 椹卞姩鏍囪鐨勪笉鍙洖鏀跺唴瀛橀噺澶х害瀵瑰簲浜?debugfs/dri/0/i915_gem_objects 涓殑鏈夌晫瀵硅薄澶у皬銆?

### 妫€娴嬩笉鍙洖鏀堕〉


mm/internal.h 涓殑鍑芥暟 folio_evictable() 浣跨敤涓婃枃姒傝堪鐨勬煡璇㈠嚱鏁癧瑙?Marking address spaces unevictable <mark_addr_space_unevict> 涓€鑺俔鏉ユ鏌?AS_UNEVICTABLE 鏍囧織锛屼粠鑰屽垽鏂竴涓?folio 鏄惁鍙洖鏀躲€?
瀵逛簬鍦ㄥ～鍏呬箣鍚庢墠琚姝ゆ爣璁扮殑鍦板潃绌洪棿锛圫HM 鍖哄煙鍙兘濡傛锛夛紝鍔犻攣鍔ㄤ綔锛堜緥濡?SHM_LOCK锛夊彲浠ユ槸鎯版€х殑锛屾棤闇€鍍?mlock() 閭ｆ牱濉厖璇ュ尯鍩熺殑椤佃〃锛屼篃鏃犻渶鐗规剰灏?SHM_LOCK 鍖哄煙鍐呯殑浠讳綍椤垫帹鍏ヤ笉鍙洖鏀堕摼琛ㄣ€傜浉鍙嶏紝vmscan 浼氬湪鍥炴敹鎵弿涓亣鍒拌繖浜?folio 鏃跺啀鍋氳繖浠朵簨銆?
鍦ㄨВ閿佸姩浣滐紙濡?SHM_UNLOCK锛夋椂锛岃В閿佽€咃紙濡?shmctl()锛夊繀椤绘壂鎻忚鍖哄煙鐨勯〉锛屽苟鍦ㄦ病鏈夊叾浠栨潯浠朵娇鍏朵繚鎸佷笉鍙洖鏀剁殑鎯呭喌涓嬶紝灏嗗畠浠粠涓嶅彲鍥炴敹閾捐〃涓?瑙ｆ晳"鍑烘潵銆傚鏋滀竴涓笉鍙洖鏀跺尯鍩熻閿€姣侊紝杩欎簺椤典篃浼氬湪閲婃斁杩囩▼涓粠涓嶅彲鍥炴敹閾捐〃涓"瑙ｆ晳"鍑烘潵銆?
folio_evictable() 杩樹細閫氳繃璋冪敤 folio_test_mlocked() 鏉ユ鏌?mlocked 鐨?folio锛屽悗鑰呭湪 folio 琚己椤垫槧灏勮繘 VM_LOCKED VMA銆佹垨鍦ㄦ琚?VM_LOCKED 鐨?VMA 涓鍙戠幇鏃剁疆浣嶃€?

### Vmscan 瀵逛笉鍙洖鏀?folio 鐨勫鐞?

濡傛灉涓嶅彲鍥炴敹 folio 鍦ㄧ己椤佃矾寰勪腑琚墧闄わ紙cull锛夛紝鎴栧湪 mlock()/mmap() 鏃惰绉诲埌涓嶅彲鍥炴敹閾捐〃锛岄偅涔?vmscan 鍦ㄥ畠浠噸鏂板彉涓哄彲鍥炴敹锛堜緥濡傞€氳繃 munlock()锛夊苟浠庝笉鍙洖鏀堕摼琛ㄨ"瑙ｆ晳"涔嬪墠锛屼笉浼氶亣鍒拌繖浜?folio銆傜劧鑰岋紝鍑轰簬渚垮埄锛屾垜浠彲鑳戒細鍐冲畾鎶婁竴涓笉鍙洖鏀?folio 鐣欏湪鏌愪釜甯歌鐨勬椿璺?闈炴椿璺?LRU 閾捐〃涓婏紝浜ょ敱 vmscan 澶勭悊銆倂mscan 鍦ㄦ墍鏈?shrink_{active|inactive|folio}_list() 鍑芥暟涓兘浼氭鏌ユ绫?folio锛屽苟浼?鍓旈櫎"閬囧埌鐨勮繖绫?folio锛氬嵆鎶婇偅浜?folio 杞悜姝ｅ湪鎵弿鐨勫唴瀛?cgroup 鍜岃妭鐐圭殑涓嶅彲鍥炴敹閾捐〃銆?
鍦ㄦ煇浜涙儏鍐典笅锛屼竴涓?folio 琚槧灏勫埌 VM_LOCKED VMA锛屼絾璇?folio 娌℃湁璁剧疆 mlocked 鏍囧織銆傝繖鏍风殑 folio 浼氫竴璺埌杈?shrink_active_list() 鎴?shrink_folio_list()锛屽湪 vmscan 閫氳繃 folio_referenced() 鎴?try_to_unmap() 閬嶅巻鍙嶅悜鏄犲皠鏃惰妫€娴嬪埌銆傚綋璇?folio 琚?shrinker 閲婃斁鏃讹紝瀹冧細琚墧闄ゅ埌涓嶅彲鍥炴敹閾捐〃銆?
瑕?鍓旈櫎"涓€涓笉鍙洖鏀?folio锛寁mscan 鍦ㄩ噴鏀?folio 閿佸悗锛岀畝鍗曞湴閫氳繃 folio_putback_lru()锛坒olio_isolate_lru() 鐨勯€嗘搷浣滐級鎶?folio 鏀惧洖 LRU 閾捐〃銆傚洜涓轰娇 folio 涓嶅彲鍥炴敹鐨勬潯浠跺湪 folio 瑙ｉ攣鍚庡彲鑳芥敼鍙橈紝__pagevec_lru_add_fn() 浼氬湪鎶婂畠鏀惧埌涓嶅彲鍥炴敹閾捐〃涔嬪墠閲嶆柊妫€鏌ュ叾涓嶅彲鍥炴敹鐘舵€併€?

## MLOCKED 椤?

闄や簡 ramfs 鍜?SYSV SHM 涔嬪锛屼笉鍙洖鏀?folio 閾捐〃瀵?mlock() 涔熷緢鏈夌敤銆傛敞鎰忥紝mlock() 浠呭湪 CONFIG_MMU=y 鐨勬儏鍐典笅鍙敤锛涘湪 NOMMU 鎯呭喌涓嬶紝鎵€鏈夋槧灏勫疄闄呬笂閮芥槸 mlocked 鐨勩€?

### 鍘嗗彶


"涓嶅彲鍥炴敹鐨?mlocked 椤?鍩虹璁炬柦鍩轰簬 Nick Piggin 鏈€鍒濆湪涓€涓涓?"mm: mlocked pages off LRU" 鐨?RFC 琛ヤ竵涓彂琛ㄧ殑宸ヤ綔銆侼ick 鍙戝竷浠栫殑琛ヤ竵锛屼綔涓?Christoph Lameter 鍙戝竷鐨勩€佽揪鎴愬悓涓€鐩爣锛堝皢 mlocked 椤靛 vmscan 闅愯棌锛夌殑琛ヤ竵鐨勬浛浠ｆ柟妗堛€?
鍦?Nick 鐨勮ˉ涓佷腑锛屼粬鐢?struct page 鐨?LRU 閾捐〃閾炬帴瀛楁涔嬩竴锛屼綔涓烘槧灏勮椤电殑 VM_LOCKED VMA 鐨勮鏁帮紙Rik van Riel 涓夊勾鍓嶆湁杩囧悓鏍风殑鎯虫硶锛夈€備絾杩欑灏嗚閾炬帴瀛楁鐢ㄤ簬璁℃暟鐨勫仛娉曪紝濡ㄧ浜嗗湪 LRU 閾捐〃涓婄鐞嗚繖浜涢〉锛屽洜姝?mlocked 椤典笉鍙縼绉伙紝鍥犱负 folio_isolate_lru() 鏃犳硶妫€娴嬪埌瀹冧滑锛屽苟涓?LRU 閾捐〃閾炬帴瀛楁涔熸棤娉曚緵杩佺Щ瀛愮郴缁熶娇鐢ㄣ€?
Nick 閫氳繃鍦ㄨ繘琛岄殧绂讳箣鍓嶆妸 mlocked 椤垫斁鍥?LRU 閾捐〃瑙ｅ喅浜嗚繖涓棶棰橈紝浠庤€屾斁寮冧簡 VM_LOCKED VMA 鐨勮鏁般€傚綋 Nick 鐨勮ˉ涓佷笌涓嶅彲鍥炴敹 LRU 宸ヤ綔鏁村悎鏃讹紝璇ヨ鏁拌鏇挎崲涓哄湪 munlock 鏃堕亶鍘嗗弽鍚戞槧灏勶紝浠ュ垽鏂槸鍚﹁繕鏈夊叾浠?VM_LOCKED VMA 浠嶆槧灏勭潃璇ラ〉銆?
鐒惰€岋紝鍦?munlock 鏃朵负姣忎釜椤甸亶鍘嗗弽鍚戞槧灏勬棦涓戦檵鍙堜綆鏁堬紝骞朵笖褰撹澶氬凡 mlock 瀹冪殑杩涚▼璇曞浘閫€鍑烘椂锛屼細瀵艰嚧鏂囦欢 rmap 閿佷笂鐏鹃毦鎬х殑浜夌敤銆傚湪 5.18 涓紝灏?mlock_count 淇濆瓨鍦ㄤ笉鍙洖鏀?LRU 閾捐〃閾炬帴瀛楁涓殑鎯虫硶琚噸鏂板惎鐢ㄥ苟浠樿瀹炶返锛屽悓鏃朵笉濡ㄧ mlocked 椤电殑杩佺Щ銆傝繖灏辨槸涓轰粈涔?涓嶅彲鍥炴敹 LRU 閾捐〃"鐜板湪涓嶈兘鏄竴涓〉鐨勯摼琛紱涓嶈繃閭ｄ釜閾捐〃鏈潵涔熸病浠€涔堢敤澶勨€斺€斿敖绠″叾澶у皬浠嶈缁存姢浠ョ敤浜?meminfo銆?

### 鍩烘湰绠＄悊


mlocked 椤碘€斺€斿嵆鏄犲皠鍒?VM_LOCKED VMA 鐨勯〉鈥斺€旀槸涓€绫讳笉鍙洖鏀堕〉銆傚綋鍐呭瓨绠＄悊瀛愮郴缁?娉ㄦ剰鍒?杩欐牱鐨勯〉鏃讹紝璇?folio 浼氳鏍囪涓?PG_mlocked 鏍囧織銆傝繖鍙互鐢?folio_set_mlocked() 鍜?folio_clear_mlocked() 鍑芥暟鎿嶄綔銆?
涓€涓?PG_mlocked 椤靛湪鍔犲叆 LRU 鏃朵細琚斁鍒颁笉鍙洖鏀堕摼琛ㄤ笂銆傝繖鏍风殑椤靛彲浠ュ湪澶氫釜鍦版柟琚唴瀛樼鐞?娉ㄦ剰鍒?锛?
 (1) 鍦?mlock()/mlock2()/mlockall() 绯荤粺璋冪敤澶勭悊绋嬪簭涓紱

 (2) 鍦?mmap() 绯荤粺璋冪敤澶勭悊绋嬪簭涓紝褰撲互 MAP_LOCKED 鏍囧織 mmap 涓€涓尯鍩熸椂锛?
 (3) 鍦ㄤ竴涓浘浠?MCL_FUTURE 鏍囧織璋冪敤 mlockall() 鐨勪换鍔′腑 mmap 涓€涓尯鍩熸椂锛?
 (4) 鍦ㄧ己椤佃矾寰勪腑锛屼互鍙婂綋 VM_LOCKED 鏍堟琚墿灞曟椂锛涙垨

 (5) 濡備笂鎵€杩帮紝鍦?vmscan:shrink_folio_list() 涓紝褰撹瘯鍥鹃€氳繃 folio_referenced() 鎴?try_to_unmap() 鍥炴敹鏌愪釜 VM_LOCKED VMA 涓殑椤垫椂銆?
mlocked 椤靛湪浠ヤ笅鎯呭喌琚В閿佸苟浠庝笉鍙洖鏀堕摼琛ㄨВ鏁戝嚭鏉ワ細

 (1) 澶勪簬閫氳繃 munlock()/munlockall() 绯荤粺璋冪敤瑙ｉ攣鐨勮寖鍥村唴鐨勬槧灏勶紱

 (2) 閫氳繃 munmap() 浠庢槧灏勮椤电殑鏈€鍚庝竴涓?VM_LOCKED VMA 涓Щ闄わ紝鍖呮嫭鍦ㄤ换鍔￠€€鍑烘椂鐨勮В闄ゆ槧灏勶紱

 (3) 褰撹椤典粠鏌愪釜 mmapped 鏂囦欢鐨勬渶鍚庝竴涓?VM_LOCKED VMA 涓鎴柇鏃讹紱鎴?
 (4) 鍦?VM_LOCKED VMA 涓椤垫墽琛?COW锛堝啓鏃跺鍒讹級涔嬪墠銆?

### mlock()/mlock2()/mlockall() 绯荤粺璋冪敤澶勭悊


mlock()銆乵lock2() 鍜?mlockall() 绯荤粺璋冪敤澶勭悊绋嬪簭浼氬璋冪敤鎵€鎸囧畾鑼冨洿鍐呯殑姣忎釜 VMA 璋冪敤 mlock_fixup()銆傚湪 mlockall() 鐨勬儏鍐典笅锛岃繖灏辨槸浠诲姟鐨勬暣涓椿鍔ㄥ湴鍧€绌洪棿銆傛敞鎰忥紝mlock_fixup() 鏃㈢敤浜?mlock 涔熺敤浜?munlock 涓€娈靛唴瀛樸€傚宸茬粡鏄?VM_LOCKED 鐨?VMA 璋冪敤 mlock()锛屾垨瀵逛笉鏄?VM_LOCKED 鐨?VMA 璋冪敤 munlock()锛岄兘琚涓虹┖鎿嶄綔锛宮lock_fixup() 鐩存帴杩斿洖銆?
濡傛灉 VMA 閫氳繃浜嗕笅鏂?Filtering Special VMAs"涓弿杩扮殑鏌愪簺杩囨护锛宮lock_fixup() 浼氬皾璇曞皢 VMA 涓庡叾鐩搁偦鑰呭悎骞讹紝鎴栬€呭湪鑼冨洿鏈鐩栨暣涓?VMA 鏃跺垏鍒嗗嚭 VMA 鐨勪竴涓瓙闆嗐€俈MA 涓凡鏈夌殑浠讳綍椤甸殢鍚庝細閫氳繃 mlock_vma_pages_range() 鈫?walk_page_range() 鈫?mlock_pte_range() 鈫?mlock_folio() 琚爣璁颁负 mlocked銆?
鍦ㄤ粠绯荤粺璋冪敤杩斿洖涔嬪墠锛宒o_mlock() 鎴?mlockall() 浼氳皟鐢?__mm_populate()锛岄€氳繃 get_user_pages() 灏嗗墿浣欓〉缂洪〉璋冨叆锛屽苟鍦ㄥ畠浠缂洪〉鏃舵爣璁颁负 mlocked銆?
娉ㄦ剰锛岃 mlock 鐨?VMA 鍙兘浠?PROT_NONE 鏄犲皠銆傚湪杩欑鎯呭喌涓嬶紝get_user_pages() 灏嗘棤娉曞皢杩欎簺椤电己椤佃皟鍏ャ€傝繖娌″叧绯汇€傚鏋滈〉鏈€缁堣缂洪〉鏄犲皠杩涜繖涓?VM_LOCKED VMA锛屽畠浠細鍦ㄧ己椤佃矾寰勪腑澶勭悊鈥斺€攎lock2() 鐨?MLOCK_ONFAULT 鍖哄煙涔熸槸杩欐牱澶勭悊鐨勩€?
瀵逛簬琚己椤垫槧灏勮繘 VMA 鐨勬瘡涓?PTE锛堟垨 PMD锛夛紝椤电殑 rmap 娣诲姞鍑芥暟浼氳皟鐢?mlock_vma_folio()锛屽綋 VMA 涓?VM_LOCKED 鏃跺畠浼氳皟鐢?mlock_folio()锛堥櫎闈炲畠鏄€忔槑澶ч〉涓€閮ㄥ垎鐨?PTE 鏄犲皠锛夈€傛垨鑰咃紝褰撳畠鏄竴涓柊鍒嗛厤鐨勫尶鍚嶉〉鏃讹紝folio_add_lru_vma() 浼氭敼涓鸿皟鐢?mlock_new_folio()锛氫笌 mlock_folio() 绫讳技锛屼絾鑳藉仛鍑烘洿濂界殑鍒ゆ柇锛屽洜涓鸿椤佃鐙崰鎸佹湁涓斿凡鐭ュ皻鏈湪 LRU 涓娿€?
mlock_folio() 绔嬪嵆璁剧疆 PG_mlocked锛岀劧鍚庢妸椤垫斁鍒?CPU 鐨?mlock folio 鎵瑰鐞嗕腑锛屼互灏嗗墿浣欏伐浣滄壒澶勭悊銆佸湪 lru_lock 涓嬬敱 __mlock_folio() 瀹屾垚銆俖_mlock_folio() 璁剧疆 PG_unevictable锛屽垵濮嬪寲 mlock_count锛屽苟灏嗛〉杞Щ鍒颁笉鍙洖鏀剁姸鎬侊紙"涓嶅彲鍥炴敹 LRU"锛屼絾浠?mlock_count 浠ｆ浛 LRU 閾炬帴锛夈€傛垨鑰咃紝濡傛灉椤靛凡缁忔槸 PG_lru銆丳G_unevictable 鍜?PG_mlocked锛屽垯鍙槸閫掑 mlock_count銆?
浣嗗湪瀹炶返涓繖鏈繀鐞嗘兂锛氶〉鍙兘灏氭湭鍦?LRU 涓婏紝鎴栬€呭彲鑳藉凡琚复鏃朵粠 LRU 闅旂銆傚湪杩欑鎯呭喌涓嬩笉鑳芥帴瑙?mlock_count 瀛楁锛屼絾瀹冧細鍦?__munlock_folio() 灏嗛〉褰掕繕"LRU"鏃惰璁句负 0銆傜珵鎬佺姝㈡鏃跺皢 mlock_count 璁句负 1锛氫笌鍏跺啋鐫€灏嗛〉姘镐箙瀛ょ珛涓轰笉鍙洖鏀剁殑椋庨櫓锛屼笉濡傛€绘槸璁?mlock_count 鍋忓悜浣庡€硷紝杩欐牱鍦?munlock 鏃惰椤典細琚В鏁戝埌鍙洖鏀?LRU锛岃嫢涔嬪悗 vmscan 鍦?VM_LOCKED VMA 涓彂鐜板畠锛屽彲鑳藉啀娆¤ mlock銆?

### 杩囨护鐗规畩 VMA


mlock_fixup() 杩囨护鍑犵被"鐗规畩" VMA锛?
1) 璁剧疆浜?VM_IO 鎴?VM_PFNMAP 鐨?VMA 琚畬鍏ㄨ烦杩囥€傝繖浜涙槧灏勮儗鍚庣殑椤垫湰璐ㄤ笂鏄鍥哄畾鐨勶紝鍥犳鎴戜滑涓嶉渶瑕佸皢瀹冧滑鏍囪涓?mlocked銆傛棤璁哄浣曪紝杩欎簺椤靛ぇ澶氭病鏈夊彲渚涙爣璁扮殑 struct page銆傚洜姝わ紝get_user_pages() 瀵硅繖浜?VMA 浼氬け璐ワ紝鎵€浠ュ皾璇曡闂畠浠病鏈夋剰涔夈€?
2) 鏄犲皠 hugetlbfs 椤电殑 VMA 瀹為檯涓婂凡缁忚鍥哄畾鍒板唴瀛樹腑銆傛垜浠棦涓嶉渶瑕佷篃涓嶆兂瀵硅繖浜涢〉鍋?mlock()銆備絾 __mm_populate() 浼氬寘鍚?hugetlbfs 鑼冨洿锛屽垎閰嶅ぇ椤靛苟濉厖 PTE銆?
3) 甯︽湁 VM_DONTEXPAND 鐨?VMA 閫氬父鏄唴鏍搁〉鐨勭敤鎴锋€佹槧灏勶紝渚嬪 VDSO 椤点€乺elay 閫氶亾椤电瓑銆傝繖浜涢〉鏈川涓婁笉鍙洖鏀讹紝涓斾笉鍦?LRU 閾捐〃涓婄鐞嗐€俖_mm_populate() 浼氬寘鍚繖浜涜寖鍥达紝鍦ㄥ皻鏈～鍏呮椂濉厖 PTE銆?
4) 璁剧疆浜?VM_MIXEDMAP 鐨?VMA 涓嶄細琚爣璁颁负 VM_LOCKED锛屼絾 __mm_populate() 浼氬寘鍚繖浜涜寖鍥达紝鍦ㄥ皻鏈～鍏呮椂濉厖 PTE銆?
娉ㄦ剰锛屽浜庢墍鏈夎繖浜涚壒娈?VMA锛宮lock_fixup() 涓嶄細璁剧疆 VM_LOCKED 鏍囧織銆傚洜姝わ紝鎴戜滑涔嬪悗鍦?munlock()銆乵unmap() 鎴栦换鍔￠€€鍑烘椂涓嶅繀澶勭悊瀹冧滑銆俶lock_fixup() 涔熶笉浼氬皢杩欎簺 VMA 璁″叆浠诲姟鐨?"locked_vm"銆?

### munlock()/munlockall() 绯荤粺璋冪敤澶勭悊


munlock() 鍜?munlockall() 绯荤粺璋冪敤鐢变笌 mlock()銆乵lock2() 鍜?mlockall() 鐩稿悓鐨?mlock_fixup() 鍑芥暟澶勭悊銆傚鏋滆皟鐢?munlock 涓€涓凡缁?munlock 鐨?VMA锛宮lock_fixup() 鐩存帴杩斿洖銆傜敱浜庝笂杩?VMA 杩囨护锛屼换浣?鐗规畩" VMA 涓兘涓嶄細璁剧疆 VM_LOCKED銆傚洜姝わ紝閭ｄ簺 VMA 鍦?munlock 鏃朵細琚拷鐣ャ€?
濡傛灉 VMA 鏄?VM_LOCKED锛宮lock_fixup() 浼氬啀娆″皾璇曞悎骞舵垨鍒囧垎鍑烘寚瀹氱殑鑼冨洿銆傜劧鍚?VMA 涓殑鎵€鏈夐〉閫氳繃 mlock_vma_pages_range() 鈫?walk_page_range() 鈫?mlock_pte_range() 鈫?munlock_folio() 琚?munlock鈥斺€旇繖涓?mlock 涓€涓?VMA 鑼冨洿鏃朵娇鐢ㄧ殑鍑芥暟鐩稿悓锛屽彧鏄?VMA 涓婂甫鏈夋爣鏄庢鍦ㄦ墽琛?munlock() 鐨勬柊鏍囧織銆?
munlock_folio() 浣跨敤 mlock pagevec 鏉ユ壒澶勭悊灏嗗湪 lru_lock 涓嬬敱 __munlock_folio() 瀹屾垚鐨勫伐浣溿€俖_munlock_folio() 閫掑噺 folio 鐨?mlock_count锛屽綋鍑忓埌 0 鏃舵竻闄?mlocked 鏍囧織鍜?unevictable 鏍囧織锛屽皢 folio 浠庝笉鍙洖鏀剁姸鎬佽浆绉诲埌闈炴椿璺?LRU銆?
浣嗗湪瀹炶返涓繖鏈繀鐞嗘兂锛歠olio 鍙兘灏氭湭鍒拌揪"涓嶅彲鍥炴敹 LRU"锛屾垨鑰呭彲鑳藉凡琚复鏃朵粠涓殧绂汇€傚湪杩欎簺鎯呭喌涓嬪畠鐨?mlock_count 瀛楁涓嶅彲鐢紝蹇呴』鍋囧畾涓?0锛氳繖鏍?folio 浼氳瑙ｆ晳鍒板彲鍥炴敹 LRU锛岃嫢涔嬪悗 vmscan 鍦?VM_LOCKED VMA 涓彂鐜板畠锛屽彲鑳藉啀娆¤ mlock銆?

### 杩佺Щ MLOCKED 椤?

姝ｅ湪杩佺Щ鐨勯〉宸茶浠?LRU 閾捐〃闅旂锛屽苟鍦ㄨ椤电殑瑙ｆ槧灏勩€佹洿鏂伴〉鐨勫湴鍧€绌洪棿椤广€佸鍒跺唴瀹瑰拰鐘舵€佹湡闂翠繚鎸侀攣瀹氾紝鐩村埌椤佃〃椤硅鏇挎崲涓烘寚鍚戞柊椤电殑椤广€侺inux 鏀寔杩佺Щ mlocked 椤靛拰鍏朵粬涓嶅彲鍥炴敹椤点€傚綋鏃ч〉浠庢渶鍚庝竴涓?VM_LOCKED VMA 瑙ｆ槧灏勬椂锛孭G_mlocked 浠庢棫椤垫竻闄わ紱褰撴柊椤佃鏄犲皠鍒?VM_LOCKED VMA 涓彇浠ｈ縼绉婚」鏃讹紝PG_mlocked 琚缃€傚鏋滈〉鍥?mlocked 鑰屼笉鍙洖鏀讹紝PG_unevictable 璺熼殢 PG_mlocked锛涗絾濡傛灉椤靛洜鍏朵粬鍘熷洜鑰屼笉鍙洖鏀讹紝鍒欐樉寮忓鍒?PG_unevictable銆?
娉ㄦ剰锛岄〉杩佺Щ鍙兘涓庡悓涓€椤电殑 mlock 鎴?munlock 鍙戠敓绔炴€併€傝繖鍩烘湰娌℃湁闂锛屽洜涓洪〉杩佺Щ闇€瑕佽В鏄犲皠鏃ч〉鐨勬墍鏈?PTE锛堝寘鎷?VM_LOCKED 鏃剁殑 munlock锛夛紝鐒跺悗鏄犲皠鏂伴〉锛堝寘鎷?VM_LOCKED 鏃剁殑 mlock锛夈€傞〉琛ㄩ攣鎻愪緵浜嗗厖鍒嗙殑鍚屾銆?
鐒惰€岋紝鐢变簬 mlock_vma_pages_range() 浠庡湪 VMA 涓婅缃?VM_LOCKED 寮€濮嬶紝涔嬪悗鎵?mlock 浠讳綍宸插瓨鍦ㄧ殑椤碉紝濡傛灉鍏朵腑鏌愪釜椤靛湪 mlock_pte_range() 鍒拌揪瀹冧箣鍓嶈杩佺Щ浜嗭紝瀹冧細鍦?mlock_count 涓璁℃暟涓ゆ銆備负闃叉杩欑鎯呭喌锛宮lock_vma_pages_range() 涓存椂灏?VMA 鏍囪涓?VM_IO锛屼娇 mlock_vma_folio() 璺宠繃瀹冦€?
涓哄畬鎴愰〉杩佺Щ锛屾垜浠湪涔嬪悗灏嗘棫椤靛拰鏂伴〉鏀惧洖 LRU銆傞偅涓?涓嶉渶瑕佺殑"椤碘€斺€旀垚鍔熸椂鏄棫椤碉紝澶辫触鏃舵槸鏂伴〉鈥斺€斿湪杩佺Щ杩囩▼鎸佹湁鐨勫紩鐢ㄨ鏁拌閲婃斁鏃惰閲婃斁銆?

### 鍘嬬缉 MLOCKED 椤?

鍙互鎵弿鍐呭瓨鏄犲皠浠ュ鎵惧彲鍘嬬缉鍖哄煙锛岄粯璁よ涓烘槸鍏佽绉诲姩涓嶅彲鍥炴敹椤点€?proc/sys/vm/compact_unevictable_allowed 鎺у埗杩欎竴琛屼负锛堣 Documentation/admin-guide/sysctl/vm.rst锛夈€傚帇缂╁伐浣滀富瑕佺敱椤佃縼绉讳唬鐮佸鐞嗭紝骞跺鐢?Migrating MLOCKED Pages 涓弿杩扮殑宸ヤ綔娴佺▼銆?

### 瀵归€忔槑澶ч〉鎵ц MLOCK


閫忔槑澶ч〉鐢?LRU 閾捐〃涓婄殑涓€涓崟鐙」琛ㄧず銆傚洜姝わ紝鎴戜滑鍙兘浣挎暣涓鍚堥〉涓嶅彲鍥炴敹锛岃€屼笉鑳戒娇鍗曚釜瀛愰〉涓嶅彲鍥炴敹銆?
濡傛灉鐢ㄦ埛灏濊瘯 mlock() 澶ч〉鐨勪竴閮ㄥ垎锛屽苟涓旀病鏈夊叾浠栫敤鎴?mlock() 鏁翠釜澶ч〉锛屾垜浠笇鏈涘ぇ椤电殑鍏朵綑閮ㄥ垎鍙洖鏀躲€?
鎴戜滑涓嶈兘鍦ㄩ儴鍒?mlock() 鏃剁洿鎺ユ媶鍒嗚椤碉紝鍥犱负 split_huge_page() 鍙兘澶辫触锛岃€屼笖鎴戜滑涓嶅笇鏈涚郴缁熻皟鐢ㄥ嚭鐜版柊鐨勯棿姝囨€уけ璐ユā寮忋€?
鎴戜滑鐨勫鐞嗘柟寮忔槸锛氬皢 PTE-mlocked 鐨勫ぇ椤典繚鐣欏湪鍙洖鏀?LRU 閾捐〃涓婏細VM_LOCKED VMA 杈圭晫澶勭殑 PMD 浼氳鎷嗗垎涓?PTE 琛ㄣ€?
杩欐牱澶ч〉瀵?vmscan 鏄彲璁块棶鐨勩€傚湪鍐呭瓨鍘嬪姏涓嬶紝璇ラ〉浼氳鎷嗗垎锛屽睘浜?VM_LOCKED VMA 鐨勫瓙椤典細琚Щ鍒颁笉鍙洖鏀?LRU锛屽叾浣欓儴鍒嗗彲琚洖鏀躲€?
/proc/meminfo 鐨?Unevictable 鍜?Mlocked 鏁板€间笉鍖呭惈閭ｄ簺浠呯敱 VM_LOCKED VMA 涓?PTE 鏄犲皠鐨勯€忔槑澶ч〉閮ㄥ垎銆?

### mmap(MAP_LOCKED) 绯荤粺璋冪敤澶勭悊


闄や簡 mlock()銆乵lock2() 鍜?mlockall() 绯荤粺璋冪敤涔嬪锛屽簲鐢ㄧ▼搴忚繕鍙互閫氳繃鍚?mmap() 璋冪敤鎻愪緵 MAP_LOCKED 鏍囧織锛岃姹傚皢涓€娈靛唴瀛樺尯鍩?mlock銆備笉杩囪繖閲屾湁涓€涓噸瑕佷笖寰鐨勫尯鍒€俶map() + mlock() 鍦ㄨ寖鍥存棤娉曡缂洪〉璋冨叆鏃讹紙渚嬪鍥犱负 mm_populate 澶辫触锛変細澶辫触骞惰繑鍥?ENOMEM锛岃€?mmap(MAP_LOCKED) 涓嶄細澶辫触銆傝 mmap 鐨勫尯鍩熶粛灏嗗叿鏈夐攣瀹氬尯鍩熺殑灞炴€р€斺€旈〉涓嶄細琚崲鍑衡€斺€斾絾灏嗗唴瀛樼己椤佃皟鍏ョ殑閲嶅ぇ缂洪〉寮傚父浠嶅彲鑳藉彂鐢熴€?
姝ゅ锛屼换浣曟浘浠?MCL_FUTURE 鏍囧織璋冪敤 mlockall() 鐨勪换鍔℃墍鍋氱殑銆佹墿灞曞爢鐨?mmap() 璋冪敤鎴?brk() 璋冪敤锛岄兘浼氬鑷存柊鏄犲皠鐨勫唴瀛樿 mlock銆傚湪涓嶅彲鍥炴敹/mlock 鏀瑰姩涔嬪墠锛屽唴鏍稿彧鏄皟鐢?make_pages_present() 鏉ュ垎閰嶉〉骞跺～鍏呴〉琛ㄣ€?
瑕佸湪涓嶅彲鍥炴敹/mlock 鍩虹璁炬柦涓?mlock 涓€娈靛唴瀛樿寖鍥达紝mmap() 澶勭悊绋嬪簭鍜屼换鍔″湴鍧€绌洪棿鎵╁睍鍑芥暟浼氳皟鐢?populate_vma_page_range()锛屾寚瀹氳 mlock 鐨?vma 鍜屽湴鍧€鑼冨洿銆?

### munmap()/exit()/exec() 绯荤粺璋冪敤澶勭悊


褰撹В闄ゆ槧灏勪竴娈?mlocked 鐨勫唴瀛樺尯鍩熸椂锛屾棤璁烘槸閫氳繃鏄惧紡璋冪敤 munmap()锛岃繕鏄粡鐢?exit() 鎴?exec() 澶勭悊涓殑鍐呴儴瑙ｆ槧灏勶紝濡傛灉鎴戜滑姝ｅ湪绉婚櫎鏄犲皠杩欎簺椤电殑鏈€鍚庝竴涓?VM_LOCKED VMA锛屽氨蹇呴』 munlock 杩欎簺椤点€傚湪涓嶅彲鍥炴敹/mlock 鏀瑰姩涔嬪墠锛宮lock 涓嶄細浠ヤ换浣曟柟寮忔爣璁拌繖浜涢〉锛屽洜姝よВ闄ゅ畠浠殑鏄犲皠鏃犻渶浠讳綍澶勭悊銆?
瀵逛簬姝ｄ粠 VMA 瑙ｉ櫎鏄犲皠鐨勬瘡涓?PTE锛堟垨 PMD锛夛紝folio_remove_rmap_*() 浼氳皟鐢?munlock_vma_folio()锛屽綋 VMA 涓?VM_LOCKED 鏃跺畠浼氳皟鐢?munlock_folio()锛堥櫎闈炲畠鏄€忔槑澶ч〉涓€閮ㄥ垎鐨?PTE 鏄犲皠锛夈€?
munlock_folio() 浣跨敤 mlock pagevec 鏉ユ壒澶勭悊灏嗗湪 lru_lock 涓嬬敱 __munlock_folio() 瀹屾垚鐨勫伐浣溿€俖_munlock_folio() 閫掑噺 folio 鐨?mlock_count锛屽綋鍑忓埌 0 鏃舵竻闄?mlocked 鏍囧織鍜?unevictable 鏍囧織锛屽皢 folio 浠庝笉鍙洖鏀剁姸鎬佽浆绉诲埌闈炴椿璺?LRU銆?
浣嗗湪瀹炶返涓繖鏈繀鐞嗘兂锛歠olio 鍙兘灏氭湭鍒拌揪"涓嶅彲鍥炴敹 LRU"锛屾垨鑰呭彲鑳藉凡琚复鏃朵粠涓殧绂汇€傚湪杩欎簺鎯呭喌涓嬪畠鐨?mlock_count 瀛楁涓嶅彲鐢紝蹇呴』鍋囧畾涓?0锛氳繖鏍?folio 浼氳瑙ｆ晳鍒板彲鍥炴敹 LRU锛岃嫢涔嬪悗 vmscan 鍦?VM_LOCKED VMA 涓彂鐜板畠锛屽彲鑳藉啀娆¤ mlock銆?

### 鎴柇 MLOCKED 椤?

鏂囦欢鎴柇鎴栨墦娲炰細寮哄埗灏嗗凡鍒犻櫎鐨勯〉浠庣敤鎴风┖闂磋В鏄犲皠锛涙埅鏂敋鑷充細瑙ｆ槧灏勫苟鍒犻櫎浠讳綍浠庢琚埅鏂殑鏂囦欢椤?Copy-On-Write锛堝啓鏃跺鍒讹級鑰屾潵鐨勭鏈夊尶鍚嶉〉銆?
Mlocked 椤靛彲浠ヤ互杩欑鏂瑰紡琚?munlock 骞跺垹闄わ細涓?munmap() 绫讳技锛屽浜庢浠?VMA 瑙ｉ櫎鏄犲皠鐨勬瘡涓?PTE锛堟垨 PMD锛夛紝folio_remove_rmap_*() 浼氳皟鐢?munlock_vma_folio()锛屽綋 VMA 涓?VM_LOCKED 鏃跺畠浼氳皟鐢?munlock_folio()锛堥櫎闈炲畠鏄€忔槑澶ч〉涓€閮ㄥ垎鐨?PTE 鏄犲皠锛夈€?
鐒惰€岋紝濡傛灉瀛樺湪绔炰簤鐨?munlock()锛岀敱浜?mlock_vma_pages_range() 閫氳繃浠?VMA 娓呴櫎 VM_LOCKED 鏉ュ紑濮?munlock锛屽湪 munlock 鎵€鏈夊凡瀛樺湪鐨勯〉涔嬪墠锛屽鏋滃叾涓煇涓〉鍦?mlock_pte_range() 鍒拌揪瀹冧箣鍓嶅氨琚埅鏂垨鎵撴礊瑙ｆ槧灏勪簡锛岄偅涔堟湰 VMA 灏变笉浼氬皢鍏惰瘑鍒负 mlocked锛屼篃涓嶄細浠?mlock_count 涓噺闄ゃ€傚湪杩欑缃曡鎯呭喌涓嬶紝涓€涓〉鍦ㄥ畬鍏ㄨВ鏄犲皠鍚庡彲鑳戒粛鏄剧ず涓?PG_mlocked锛氭鏃朵氦鐢?release_pages()锛堟垨 __page_cache_release()锛夊湪閲婃斁鍓嶆竻闄ゅ畠骞舵洿鏂扮粺璁★紙姝や簨浠惰鍏?/proc/vmstat 鐨?unevictable_pgs_cleared锛岃鍊奸€氬父涓?0锛夈€?

### shrink_*_list() 涓殑椤靛洖鏀?

vmscan 鐨?shrink_active_list() 浼氬墧闄や换浣曟槑鏄句笉鍙洖鏀剁殑椤碘€斺€斿嵆 !page_evictable(page) 鐨勯〉鈥斺€斿皢瀹冧滑杞悜涓嶅彲鍥炴敹閾捐〃銆傜劧鑰岋紝shrink_active_list() 鍙兘鐪嬪埌閭ｄ簺杩涘叆浜嗘椿璺?闈炴椿璺?LRU 閾捐〃鐨勪笉鍙洖鏀堕〉銆傛敞鎰忥紝杩欎簺椤垫病鏈夎缃?PG_unevictable鈥斺€斿惁鍒欏畠浠細鍦ㄤ笉鍙洖鏀堕摼琛ㄤ笂锛岃€?shrink_active_list() 姘歌繙涓嶄細鐪嬪埌瀹冧滑銆?
LRU 閾捐〃涓婅繖绫讳笉鍙洖鏀堕〉鐨勪竴浜涗緥瀛愭槸锛?
 (1) 棣栨鍒嗛厤鏃跺氨琚斁鍒?LRU 閾捐〃涓婄殑 ramfs 椤点€?
 (2) SHM_LOCK 鐨勫叡浜唴瀛橀〉銆俿hmctl(SHM_LOCK) 涓嶄細灏濊瘯鍒嗛厤鎴栬皟鍏ュ叡浜唴瀛樺尯鍩熶腑鐨勯〉銆傝繖鍙戠敓鍦ㄥ簲鐢ㄧ▼搴忓湪 SHM_LOCK 璇ユ涔嬪悗绗竴娆¤闂椤垫椂銆?
 (3) 浠嶆槧灏勫埌 VM_LOCKED VMA 鐨勯〉锛屾湰搴旇鏍囪涓?mlocked锛屼絾浜嬩欢瀵艰嚧 mlock_count 杩囦綆锛屽洜姝ゅ畠浠杩囨棭 munlock 浜嗐€?
vmscan 鐨?shrink_inactive_list() 鍜?shrink_folio_list() 涔熶細灏嗛潪娲昏穬閾捐〃涓婂彂鐜扮殑鏄庢樉涓嶅彲鍥炴敹椤碉紝杞悜閫傚綋鐨?memory cgroup 鍜岃妭鐐圭殑涓嶅彲鍥炴敹閾捐〃銆?
rmap 鐨?folio_referenced_one()锛堢粡鐢?vmscan 鐨?shrink_active_list() 鎴?shrink_folio_list() 璋冪敤锛変互鍙?rmap 鐨?try_to_unmap_one()锛堢粡鐢?shrink_folio_list() 璋冪敤锛変細妫€鏌ヤ粛鐒舵槧灏勫埌 VM_LOCKED VMA 鐨?(3) 绫婚〉锛屽苟璋冪敤 mlock_vma_folio() 鏉ョ籂姝ｅ畠浠€傝繖绫婚〉鍦ㄨ shrinker 閲婃斁鏃朵細琚墧闄ゅ埌涓嶅彲鍥炴敹閾捐〃銆?