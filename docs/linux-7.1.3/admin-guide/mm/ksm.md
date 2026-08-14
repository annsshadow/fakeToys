## 鍐呮牳鍚岄〉鍚堝苟锛圞ernel Samepage Merging锛孠SM锛?


## 姒傝堪


KSM 鏄竴涓妭鐪佸唴瀛樼殑鍘婚噸鐗规€э紝鐢?CONFIG_KSM=y 鍚敤锛屽湪 2.6.32 涓姞鍏?Linux 鍐呮牳銆傚叾瀹炵幇鍙傝 `mm/ksm.c`锛屼互鍙?http://lwn.net/Articles/306704/ 涓?https://lwn.net/Articles/330589/銆?

KSM 鏈€鍒濅负閰嶅悎 KVM 寮€鍙戯紙褰撴椂绉颁负 Kernel Shared Memory锛屽唴鏍稿叡浜唴瀛橈級锛岄€氳繃鍏变韩铏氭嫙鏈轰箣闂寸殑鍏叡鏁版嵁锛屼娇鏇村铏氭嫙鏈鸿兘澶熻鍏ョ墿鐞嗗唴瀛樸€備絾浠讳綍浼氱敓鎴愬ぇ閲忕浉鍚屾暟鎹疄渚嬬殑搴旂敤閮藉彲浠ヤ粠涓彈鐩娿€?

KSM 瀹堟姢杩涚▼ ksmd 浼氬懆鏈熸€у湴鎵弿閭ｄ簺宸插悜鍏舵敞鍐岀殑鐢ㄦ埛鍐呭瓨鍖哄煙锛屽鎵惧唴瀹圭浉鍚岀殑椤碉紝骞剁敤涓€涓啓淇濇姢鐨勫崟涓€椤垫潵鏇挎崲瀹冧滑锛堝鏋滄煇涓繘绋嬬◢鍚庢兂瑕佹洿鏂板叾鍐呭锛岃椤典細琚嚜鍔ㄥ鍒讹級銆侹SM 瀹堟姢杩涚▼鍗曟鎵弿鐨勯〉鏁颁互鍙婃壂鎻忎箣闂寸殑闂撮殧閫氳繃 :ref:`sysfs 鎺ュ彛 <ksm_sysfs>` 閰嶇疆銆?

KSM 鍙悎骞跺尶鍚嶏紙绉佹湁锛夐〉锛屼粠涓嶅悎骞?pagecache锛堟枃浠讹級椤点€侹SM 鍚堝苟鐨勯〉鏈€鍒濊閿佸畾鍦ㄥ唴鏍稿唴瀛樹腑锛屼絾鐜板湪鍙互鍍忓叾浠栫敤鎴烽〉涓€鏍疯鎹㈠嚭锛堜絾鍦ㄦ崲鍥炴椂鍏变韩浼氳鎵撶牬锛歬smd 蹇呴』閲嶆柊鍙戠幇瀹冧滑鐨勫悓涓€鎬у苟鍐嶆鍚堝苟锛夈€?

## 鐢?madvise 鎺у埗 KSM


KSM 鍙搴旂敤绋嬪簭閫氳繃 madvise(2) 寤鸿涓哄彲鑳藉悎骞跺€欓€夌殑鍦板潃绌洪棿鍖哄煙璧蜂綔鐢?
```
	int madvise(addr, length, MADV_MERGEABLE)
```
搴旂敤鍙互璋冪敤
```
	int madvise(addr, length, MADV_UNMERGEABLE)
```
鏉ュ彇娑堣寤鸿骞舵仮澶嶆湭鍏变韩鐨勯〉锛氫簬鏄?KSM 浼氬彇娑堝湪璇ヨ寖鍥村唴鍚堝苟鐨勬墍鏈夊唴瀹广€傛敞鎰忥細杩欎釜鍙栨秷鍚堝苟鐨勮皟鐢ㄥ彲鑳界獊鐒堕渶瑕佹瘮鍙敤閲忔洿澶氱殑鍐呭瓨鈥斺€斿彲鑳戒互 EAGAIN 澶辫触锛屼絾鏇村彲鑳芥嫑鑷?Out-Of-Memory killer銆?

濡傛灉 KSM 娌℃湁閰嶇疆杩涜繍琛屼腑鐨勫唴鏍革紝madvise MADV_MERGEABLE 涓?MADV_UNMERGEABLE 浼氱畝鍗曞湴浠?EINVAL 澶辫触銆傚鏋滆繍琛屼腑鐨勫唴鏍镐互 CONFIG_KSM=y 鏋勫缓锛岃繖浜涜皟鐢ㄩ€氬父閮戒細鎴愬姛锛氬嵆浣?KSM 瀹堟姢杩涚▼褰撳墠娌℃湁杩愯锛孧ADV_MERGEABLE 浠嶄細涓?KSM 瀹堟姢杩涚▼鍚姩鍚庣殑浠绘剰鏃跺埢娉ㄥ唽璇ヨ寖鍥达紱鍗充娇璇ヨ寖鍥翠笉鍙兘鍖呭惈浠讳綍 KSM 瀹為檯鑳藉鍚堝苟鐨勯〉锛涘嵆浣?MADV_UNMERGEABLE 琚簲鐢ㄥ埌浠庢湭 MADV_MERGEABLE 鐨勮寖鍥淬€?

濡傛灉涓€涓唴瀛樺尯鍩熷繀椤昏鎷嗗垎涓鸿嚦灏戜竴涓柊鐨?MADV_MERGEABLE 鎴?MADV_UNMERGEABLE 鍖哄煙锛屽綋杩涚▼灏嗚秴杩?`vm.max_map_count` 鏃讹紝madvise 鍙兘杩斿洖 ENOMEM锛堝弬瑙?Documentation/admin-guide/sysctl/vm.rst锛夈€?

涓庡叾浠?madvise 璋冪敤涓€鏍凤紝瀹冧滑鐢ㄤ簬鐢ㄦ埛鍦板潃绌洪棿鐨勫凡鏄犲皠鍖哄煙锛氬鏋滄寚瀹氳寖鍥村寘鍚湭鏄犲皠鐨勯棿闅欙紝瀹冧滑浼氭姤鍛?ENOMEM锛堜笉杩囦粛浼氬鐞嗗叾涓凡鏄犲皠鐨勫尯鍩燂級锛屽苟涓斿鏋滃唴閮ㄧ粨鏋勭殑鍙敤鍐呭瓨涓嶈冻锛屽彲鑳戒互 EAGAIN 澶辫触銆?

搴旂敤鍦ㄤ娇鐢?MADV_MERGEABLE 鏃跺簲淇濇寔鑺傚埗锛屽皢鍏堕檺鍒跺湪鍙兘鍙楃泭鐨勫尯鍩熴€侹SM 鐨勬壂鎻忓彲鑳芥秷鑰楀ぇ閲忓鐞嗚兘鍔涳細鍑轰簬杩欎釜鍘熷洜锛屾煇浜涢儴缃蹭細绂佺敤 KSM銆?


## KSM 瀹堟姢杩涚▼ sysfs 鎺ュ彛


KSM 瀹堟姢杩涚▼鐢?`/sys/kernel/mm/ksm/` 涓嬬殑 sysfs 鏂囦欢鎺у埗锛屾墍鏈夌敤鎴峰彲璇伙紝浣嗗彧鏈?root 鍙啓锛?

pages_to_scan
        鍦?ksmd 杩涘叆鐫＄湢涔嬪墠瑕佹壂鎻忓灏戦〉
        渚嬪 `echo 100 > /sys/kernel/mm/ksm/pages_to_scan`銆?

        濡傛灉 `advisor_mode` 宸茶缃负 scan-time锛屽垯 pages_to_scan 鍊间笉鑳借鏀瑰彉銆?

        榛樿锛?00锛堝嚭浜庢紨绀虹洰鐨勮€岄€夋嫨锛?

sleep_millisecs
        ksmd 鍦ㄤ笅娆℃壂鎻忎箣鍓嶅簲鐫＄湢澶氬皯姣
        渚嬪 `echo 20 > /sys/kernel/mm/ksm/sleep_millisecs`

        榛樿锛?0锛堝嚭浜庢紨绀虹洰鐨勮€岄€夋嫨锛?

merge_across_nodes
        鎸囧畾鏉ヨ嚜涓嶅悓 NUMA 鑺傜偣鐨勯〉鏄惁鍙互鍚堝苟銆?
        褰撹缃负 0 鏃讹紝ksm 鍙悎骞剁墿鐞嗕笂浣嶄簬鍚屼竴 NUMA 鑺傜偣鍐呭瓨鍖哄煙鐨勯〉銆傝繖浼氶檷浣?
        璁块棶鍏变韩椤电殑寤惰繜銆傚叿鏈夋洿澶氳妭鐐广€佷笖 NUMA 璺濈鏄捐憲鐨勭郴缁燂紝寰堝彲鑳戒粠 0 鐨?
        杈冧綆寤惰繜涓彈鐩娿€傞渶瑕佹渶灏忓寲鍐呭瓨浣跨敤鐨勫皬鍨嬬郴缁燂紝寰堝彲鑳戒粠 1锛堥粯璁わ級鐨勬洿楂?
        鍏变韩涓彈鐩娿€傚湪鍐冲畾浣跨敤鍝釜璁剧疆涔嬪墠锛屼綘鍙兘甯屾湜姣旇緝浣犵殑绯荤粺鍦ㄨ繖涓ょ璁剧疆涓嬬殑琛ㄧ幇銆俙merge_across_nodes` 璁剧疆鍙兘鍦ㄧ郴缁熶腑娌℃湁 ksm 鍏变韩椤垫椂鏇存敼锛氬厛灏?run 璁句负 2 浠ュ厛鍙栨秷鍚堝苟椤碉紝鐒跺悗鍦ㄦ洿鏀?`merge_across_nodes` 鍚庤涓?1锛屼互鏍规嵁鏂拌缃噸鏂板悎骞躲€?

        榛樿锛?锛堣法鑺傜偣鍚堝苟锛屼笌鏃╂湡鐗堟湰鐩稿悓锛?

run
        - 璁句负 0 鍋滄 ksmd 杩愯浣嗕繚鐣欏凡鍚堝苟鐨勯〉锛?
        - 璁句负 1 杩愯 ksmd锛屼緥濡?`echo 1 > /sys/kernel/mm/ksm/run`锛?
        - 璁句负 2 鍋滄 ksmd 骞跺彇娑堝悎骞跺綋鍓嶆墍鏈夊凡鍚堝苟鐨勯〉锛屼絾
	  淇濈暀鍙悎骞跺尯鍩熶互澶囦笅娆¤繍琛屻€?

        榛樿锛?锛堝繀椤绘敼涓?1 鎵嶈兘婵€娲?KSM锛岄櫎闈?CONFIG_SYSFS 琚鐢級

use_zero_pages
        鎸囧畾绌洪〉锛堝嵆鍙寘鍚浂鐨勫凡鍒嗛厤椤碉級鏄惁搴旇鐗规畩澶勭悊銆傚綋璁剧疆涓?1 鏃讹紝
        绌洪〉浼氫笌鍐呮牳闆堕〉鍚堝苟锛岃€岄潪鍍忛€氬父閭ｆ牱褰兼鍚堝苟銆傛牴鎹伐浣滆礋杞界殑涓嶅悓锛?
        杩欏彲浠ュ湪甯︽湁鐫€鑹查浂椤电殑鏋舵瀯涓婃彁鍗囨€ц兘銆傚惎鐢ㄦ璁剧疆鏃跺簲璋ㄦ厧锛屽洜涓哄畠
        鍙兘闄嶄綆鏌愪簺宸ヤ綔璐熻浇涓?KSM 鐨勬€ц兘锛屼緥濡傚綋鍊欓€夊悎骞堕〉鐨勬牎楠屽拰涓?
        绌洪〉鐨勬牎楠屽拰鍖归厤鏃躲€傝璁剧疆鍙互闅忔椂鏇存敼锛屽畠鍙鏇存敼涔嬪悗鍚堝苟鐨勯〉鏈夋晥銆?

        榛樿锛?锛堜笌鏃╂湡鐗堟湰鐩稿悓鐨勬甯?KSM 琛屼负锛?

max_page_sharing
        姣忎釜 KSM 椤靛厑璁哥殑鏈€澶у叡浜暟銆傝繖寮哄埗浜嗕竴涓幓閲嶄笂闄愶紝浠ラ伩鍏嶆秹鍙婇亶鍘?
        鍏变韩璇?KSM 椤电殑铏氭嫙鏄犲皠鐨勮櫄鎷熷唴瀛樻搷浣滀骇鐢熼珮寤惰繜銆傛渶灏忓€间负 2锛屽洜涓?
        鏂板垱寤虹殑 KSM 椤佃嚦灏戞湁涓や釜鍏变韩鑰呫€傝鍊艰秺楂橈紝KSM 鍚堝苟鍐呭瓨鐨勯€熷害瓒婂揩锛?
        鍘婚噸鍥犲瓙涔熻秺楂橈紝浣嗘渶鍧忔儏鍐典笅缁欏畾 KSM 椤电殑铏氭嫙鏄犲皠閬嶅巻鍙兘瓒婃參銆傚噺缂?
        杩欑閬嶅巻鎰忓懗鐫€鍦ㄦ崲鍑恒€佸帇缂┿€丯UMA 骞宠　涓庨〉杩佺Щ鏈熼棿鏌愪簺铏氭嫙鍐呭瓨鎿嶄綔鐨?
        寤惰繜浼氭洿楂橈紝杩涜€岄檷浣庤繖浜涜櫄鎷熷唴瀛樻搷浣滆皟鐢ㄦ柟鐨勫搷搴旀€с€備笉鍙備笌杩欎簺鍋?
        铏氭嫙鏄犲皠閬嶅巻鐨?VM 鎿嶄綔鐨勫叾浠栦换鍔＄殑璋冨害鍣ㄥ欢杩熶笉鍙楁鍙傛暟褰卞搷锛屽洜涓鸿繖浜?
        閬嶅巻鏈韩濮嬬粓鏄皟搴﹀弸濂界殑銆?

stable_node_chains_prune_millisecs
        鎸囧畾 KSM 澶氶绻佹鏌ュ懡涓幓閲嶄笂闄愮殑椤电殑鍏冩暟鎹腑鐨勮繃鏈熶俊鎭€?
        杈冨皬鐨勬绉掑€间細浠ユ洿浣庡欢杩熼噴鏀?KSM 鍏冩暟鎹紝浣嗕細璁?ksmd 鍦ㄦ壂鎻忔湡闂?
        浣跨敤鏇村 CPU銆傚鏋滆繕娌℃湁浠讳綍 KSM 椤靛懡涓?`max_page_sharing`锛屽垯瀹?
        鏄竴涓┖鎿嶄綔锛坣oop锛夈€?

smart_scan
        鍘嗗彶涓?KSM 鍦ㄦ瘡娆℃壂鎻忎腑妫€鏌ユ瘡涓€欓€夐〉銆傚畠娌℃湁鑰冭檻鍘嗗彶淇℃伅銆傚惎鐢?smart scan 鍚庯紝鍏堝墠鏈鍘婚噸鐨勯〉浼氳璺宠繃銆傝繖浜涢〉琚烦杩囩殑棰戠巼鍙栧喅浜庡幓閲嶅凡缁忓皾璇曞苟澶辫触鐨勬鏁般€傞粯璁ゅ惎鐢ㄦ浼樺寲銆俙pages_skipped` 鎸囨爣鏄剧ず浜嗚璁剧疆鐨勬湁鏁堟€с€?

advisor_mode
        `advisor_mode` 閫夋嫨褰撳墠鐨勯【闂紙advisor锛夈€傛敮鎸佷袱绉嶆ā寮忥細none 涓?scan-time銆傞粯璁や负 none銆傚皢 `advisor_mode` 璁句负 scan-time 鍙惎鐢ㄦ壂鎻忔椂闂撮【闂€傚叧浜?`advisor` 鐨勫皬鑺傝缁嗚В閲婁簡鎵弿鏃堕棿椤鹃棶鐨勫伐浣滃師鐞嗐€?

adivsor_max_cpu
        鎸囧畾 ksmd 鍚庡彴绾跨▼ CPU 浣跨敤鐧惧垎姣旂殑涓婇檺銆傞粯璁や负 70銆?

advisor_target_scan_time
        鎸囧畾鎵弿鎵€鏈夊€欓€夐〉鐨勭洰鏍囨壂鎻忔椂闂达紝浠ョ涓哄崟浣嶃€傞粯璁ゅ€间负 200 绉掋€?

advisor_min_pages_to_scan
        鎸囧畾鎵弿鏃堕棿椤鹃棶鐨?`pages_to_scan` 鍙傛暟鐨勪笅闄愩€傞粯璁や负 500銆?

adivsor_max_pages_to_scan
        鎸囧畾鎵弿鏃堕棿椤鹃棶鐨?`pages_to_scan` 鍙傛暟鐨勪笂闄愩€傞粯璁や负 30000銆?

KSM 涓?MADV_MERGEABLE 鐨勬湁鏁堟€ф樉绀哄湪 `/sys/kernel/mm/ksm/` 涓細

general_profit
        KSM 鐨勬湁鏁堟€у浣曘€傝绠楁柟寮忚В閲婂涓嬨€?
pages_scanned
        鏈夊灏戦〉姝ｅ湪琚壂鎻忎互鐢ㄤ簬 ksm
pages_shared
        姝ｅ湪浣跨敤澶氬皯涓叡浜〉
pages_sharing
        杩樻湁澶氬皯澶勭珯鐐瑰湪鍏变韩瀹冧滑锛屽嵆鑺傜渷浜嗗灏?
pages_unshared
        澶氬皯椤垫槸鍞竴鐨勶紝浣嗚鍙嶅妫€鏌ヤ互杩涜鍚堝苟
pages_volatile
        澶氬皯椤靛彉鍖栧お蹇€屾棤娉曟斁鍏ユ爲涓?
pages_skipped
        鈥渟mart鈥?椤垫壂鎻忕畻娉曡烦杩囦簡澶氬皯椤?
full_scans
        鎵€鏈夊彲鍚堝苟鍖哄煙宸茶鎵弿浜嗗灏戞
stable_node_chains
        鍛戒腑 `max_page_sharing` 闄愬埗鐨?KSM 椤垫暟閲?
stable_node_dups
        閲嶅鐨?KSM 椤垫暟閲?
ksm_zero_pages
        褰撳幓閲嶆椂鐢?KSM 鏄犲皠銆佷笖浠嶆槧灏勫埌杩涚▼涓殑闆堕〉鏁伴噺銆?

褰?`use_zero_pages` 宸?鏇捐鍚敤鏃讹紝`pages_sharing` + `ksm_zero_pages` 鐨勫拰琛ㄧず KSM 瀹為檯鑺傜渷鐨勯〉鏁伴噺銆傚鏋?`use_zero_pages` 浠庢湭琚惎鐢紝鍒?`ksm_zero_pages` 涓?0銆?

`pages_sharing` 涓?`pages_shared` 鐨勯珮姣旂巼琛ㄧず鑹ソ鐨勫叡浜紝浣?`pages_unshared` 涓?`pages_sharing` 鐨勯珮姣旂巼琛ㄧず娴垂浜嗙簿鍔涖€俙pages_volatile` 鍖呭惈鍑犵涓嶅悓绫诲瀷鐨勬椿鍔紝浣嗗叾涓崰杈冮珮姣斾緥涔熻〃鏄庡 madvise MADV_MERGEABLE 鐨勪娇鐢ㄤ笉褰撱€?

鏈€澶у彲鑳界殑 `pages_sharing/pages_shared` 姣旂巼鍙?`max_page_sharing` 鍙皟鍙傛暟闄愬埗銆傝鎻愰珮璇ユ瘮鐜囷紝蹇呴』鐩稿簲鍦板澶?`max_page_sharing`銆?

## 鐩戞帶 KSM 鏀剁泭


KSM 鍙互閫氳繃鍚堝苟鐩稿悓鐨勯〉鏉ヨ妭鐪佸唴瀛橈紝浣嗕篃鍙兘娑堣€楅澶栫殑鍐呭瓨锛屽洜涓哄畠闇€瑕佺敓鎴愯嫢骞?rmap_item 鏉ヤ繚瀛樻瘡涓鎵弿椤电殑绠€瑕?rmap 淇℃伅銆傝繖浜涢〉涓湁浜涘彲鑳借鍚堝苟锛屼絾鏈変簺鍦ㄥ娆℃鏌ュ悗鍙兘浠嶆棤娉曞悎骞讹紝杩欎簺灏辨槸琚秷鑰楃殑鏃犳晥鍐呭瓨銆?

1) 濡備綍鍒ゆ柇 KSM 鏄湪绯荤粺鑼冨洿鍐呰妭鐪佸唴瀛樿繕鏄秷鑰楀唴瀛?
```
	general_profit =~ ksm_saved_pages * sizeof(page) - (all_rmap_items) *
			  sizeof(rmap_item);

   鍏朵腑 ksm_saved_pages 绛変簬绯荤粺鐨?``pages_sharing`` +
   ``ksm_zero_pages`` 涔嬪拰锛岃€?all_rmap_items 鍙互寰堝鏄撳湴閫氳繃鎶?
   ``pages_sharing``銆乣`pages_shared``銆乣`pages_unshared`` 涓?
   ``pages_volatile`` 鐩稿姞寰楀埌銆?
```
2) 鍗曚釜杩涚▼鍐呴儴鐨?KSM 鏀剁泭鍙互閫氳繃绫讳技鏂瑰紡寰楀埌
```
	process_profit =~ ksm_saved_pages * sizeof(page) -
			  ksm_rmap_items * sizeof(rmap_item).

   鍏朵腑 ksm_saved_pages 绛変簬 ``ksm_merging_pages`` 涓?``ksm_zero_pages`` 涔嬪拰锛?
   浜岃€呴兘鏄剧ず鍦?``/proc/<pid>/ksm_stat`` 鐩綍涓嬶紝ksm_rmap_items 涔熸樉绀哄湪
   ``/proc/<pid>/ksm_stat`` 涓€傝繘绋嬫敹鐩婁篃浣滀负 ksm_process_profit 鏄剧ず鍦?
   ``/proc/<pid>/ksm_stat`` 涓€?
```
浠庡簲鐢ㄧ殑瑙掑害鐪嬶紝`ksm_rmap_items` 涓?`ksm_merging_pages` 鐨勯珮姣旂巼鎰忓懗鐫€涓€涓碂绯曠殑 madvise 搴旂敤绛栫暐锛屽洜姝ゅ紑鍙戣€呮垨绠＄悊鍛樺繀椤婚噸鏂拌€冭檻濡備綍鏇存敼 madvise 绛栫暐銆傜粰鍑轰竴涓緵鍙傝€冪殑渚嬪瓙锛氶〉鐨勫ぇ灏忛€氬父涓?4K锛岃€?rmap_item 鐨勫ぇ灏忓湪 32 浣?CPU 鏋舵瀯涓婁负 32B锛屽湪 64 浣?CPU 鏋舵瀯涓婁负 64B銆傚洜姝わ紝濡傛灉 `ksm_rmap_items/ksm_merging_pages` 姣旂巼鍦?64 浣?CPU 涓婅秴杩?64锛屾垨鍦?32 浣?CPU 涓婅秴杩?128锛岄偅涔堝簲鐢ㄧ殑 madvise 绛栫暐搴斿綋琚斁寮冿紝鍥犱负 KSM 鏀剁泭杩戜技涓洪浂鎴栦负璐熴€?

## 鐩戞帶 KSM 浜嬩欢


/proc/vmstat 涓湁涓€浜涜鏁板櫒鍙敤浜庣洃鎺?KSM 浜嬩欢銆侹SM 鍙兘鏈夊姪浜庤妭鐪佸唴瀛橈紝浣嗗畠鏄竴绉嶆潈琛★紝鍙兘鎵垮彈 KSM COW 鎴栧湪鎹㈠叆鍓湰鏃剁殑寤惰繜銆傝繖浜涗簨浠跺彲浠ュ府鍔╃敤鎴疯瘎浼版槸鍚︿互鍙婂浣曚娇鐢?KSM銆備緥濡傦紝濡傛灉 cow_ksm 澧為暱杩囧揩锛岀敤鎴峰彲浠ョ缉灏?madvise(, , MADV_MERGEABLE) 鐨勮寖鍥淬€?

cow_ksm
        姣忓綋涓€涓?KSM 椤佃Е鍙戝啓鏃跺鍒讹紙COW锛夋椂閫掑銆?
        褰撶敤鎴峰皾璇曞啓鍏ヤ竴涓?KSM 椤垫椂锛屾垜浠繀椤诲埗浣滀竴浠藉壇鏈€?

ksm_swpin_copy
        姣忓綋涓€涓?KSM 椤靛湪鎹㈠叆鏃惰澶嶅埗鏃堕€掑銆?
        娉ㄦ剰 KSM 椤靛湪鎹㈠叆鏃跺彲鑳借澶嶅埗锛屽洜涓?do_swap_page()
        鏃犳硶杩涜閲嶅缓涓€涓法 anon_vma 鐨?KSM 椤垫墍闇€鐨勫叏閮ㄥ姞閿併€?

## 椤鹃棶锛圓dvisor锛?


KSM 鍊欓€夐〉鐨勬暟閲忔槸鍔ㄦ€佺殑銆傜粡甯稿彲浠ヨ瀵熷埌锛屽湪搴旂敤鍚姩鏈熼棿闇€瑕佸鐞嗘洿澶氬€欓€夐〉銆傚鏋滄病鏈夐【闂紝`pages_to_scan` 鍙傛暟闇€瑕佹寜鐓ф渶澶у€欓€夐〉鏁伴噺鏉ヨ瀹氬ぇ灏忋€傛壂鎻忔椂闂撮【闂彲浠ユ牴鎹渶姹傛敼鍙?`pages_to_scan` 鍙傛暟銆?

鍙互鍚敤椤鹃棶锛岃繖鏍?KSM 灏辫兘鑷姩閫傚簲寰呮壂鎻忓€欓€夐〉鏁伴噺鐨勫彉鍖栥€傚疄鐜颁簡涓ょ椤鹃棶锛歯one 涓?scan-time銆備娇鐢?none 鏃朵笉鍚敤浠讳綍椤鹃棶銆傞粯璁や负 none銆?

鎵弿鏃堕棿椤鹃棶鏍规嵁瑙傚療鍒扮殑鎵弿鏃堕棿鏀瑰彉 `pages_to_scan` 鍙傛暟銆傝鍙傛暟 `pages_to_scan` 鐨勫彲鑳藉彇鍊煎彈 `advisor_max_cpu` 鍙傛暟闄愬埗銆傛澶栬繕鏈?`advisor_target_scan_time` 鍙傛暟銆傝鍙傛暟璁惧畾鎵弿鎵€鏈?KSM 鍊欓€夐〉鐨勭洰鏍囨椂闂淬€俙advisor_target_scan_time` 鍙傛暟鍐冲畾鎵弿鏃堕棿椤鹃棶鎵弿鍊欓€夐〉鐨勬縺杩涚▼搴︺€傝緝浣庣殑鍊间娇鎵弿鏃堕棿椤鹃棶鎵弿寰楁洿婵€杩涖€傝繖鏄壂鎻忔椂闂撮【闂厤缃腑鏈€閲嶈鐨勫弬鏁般€?

鍒濆鍊间笌鏈€澶у€煎彲浠ラ€氳繃 `advisor_min_pages_to_scan` 涓?`advisor_max_pages_to_scan` 鏇存敼銆傞粯璁ゅ€煎澶у鏁板伐浣滆礋杞戒笌鐢ㄤ緥閮藉凡瓒冲銆?

`pages_to_scan` 鍙傛暟鍦ㄤ竴娆℃壂鎻忓畬鎴愬悗琚噸鏂拌绠椼€?


--
Izik Eidus,
Hugh Dickins, 2009 骞?11 鏈?17 鏃?
