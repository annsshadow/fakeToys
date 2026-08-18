## VME Device Drivers


### Driver registration


涓庡叾浠?Linux 鍐呮牳瀛愮郴缁熶竴鏍凤紝VME 璁惧椹卞姩鍚?VME 瀛愮郴缁熸敞鍐岋紝閫氬父浠庤澶囩殑 init
渚嬬▼涓皟鐢ㄣ€傝繖鏄€氳繃璋冪敤 `vme_register_driver` 瀹炵幇鐨勩€?

蹇呴』鍚戞敞鍐屽嚱鏁版彁渚涗竴涓寚鍚?`struct vme_driver <vme_driver>` 绫诲瀷缁撴瀯鐨勬寚閽堬紝
浠ュ強浣犵殑椹卞姩鎵€鑳芥敮鎸佺殑鏈€澶ц澶囨暟閲忋€?

鑷冲皯锛宍struct vme_driver <vme_driver>` 鐨?'.name'銆?.match' 鍜?'.probe' 鍏冪礌搴?
琚纭缃€?.name' 鍏冪礌鏄竴涓寚鍚戜繚瀛樿澶囬┍鍔ㄥ悕绉板瓧绗︿覆鐨勬寚閽堛€?

'.match' 鍑芥暟鐢ㄤ簬鎺у埗鍝簺 VME 璁惧搴斿綋娉ㄥ唽鍒拌椹卞姩銆傚鏋滄煇璁惧搴斿綋琚帰娴嬶紝match
鍑芥暟搴旇繑鍥?1锛屽惁鍒欒繑鍥?0銆備笅闈㈣繖涓?match 鍑芥暟绀轰緥锛堟潵鑷?vme_user.c锛夊皢鎺㈡祴鐨勮澶?
鏁伴噺闄愬埗涓轰竴涓細


	#define USER_BUS_MAX	1
	...
	static int vme_user_match(struct vme_dev *vdev)
	{
		if (vdev->id.num >= USER_BUS_MAX)
			return 0;
		return 1;
	}

'.probe' 鍏冪礌搴斿寘鍚寚鍚戞帰娴嬩緥绋嬬殑鎸囬拡銆傛帰娴嬩緥绋嬩互涓€涓?`struct vme_dev <vme_dev>`
鎸囬拡浣滀负鍙傛暟銆?

杩欓噷锛?num' 瀛楁鎸囩殑鏄鐗瑰畾椹卞姩鐨勯『搴忚澶?ID銆傛ˉ鍙凤紙鎴栨€荤嚎鍙凤級鍙€氳繃
dev->bridge->num 璁块棶銆?

杩樻彁渚涗簡涓€涓敤浜庝粠 VME 鏍稿績娉ㄩ攢椹卞姩鐨勫嚱鏁?`vme_unregister_driver`锛岄€氬父搴斿湪璁惧
椹卞姩鐨勯€€鍑轰緥绋嬩腑璋冪敤銆?


### Resource management


涓€鏃﹂┍鍔ㄥ悜 VME 鏍稿績娉ㄥ唽锛屾墍鎻愪緵鐨?match 渚嬬▼灏嗚璋冪敤娉ㄥ唽鏃舵寚瀹氱殑娆℃暟銆傚鏋滃尮閰?
鎴愬姛锛屽簲杩斿洖涓€涓潪闆跺€硷紱杩斿洖闆惰〃绀哄け璐ャ€傚浜庢墍鏈夋垚鍔熷尮閰嶇殑鎯呭喌锛屼細璋冪敤瀵瑰簲椹卞姩鐨?
probe 渚嬬▼銆俻robe 渚嬬▼浼氫紶鍏ヤ竴涓寚鍚戣澶?device 缁撴瀯鐨勬寚閽堛€傝鎸囬拡搴斿綋琚繚瀛橈紝鍦?
璇锋眰 VME 璧勬簮鏃朵細鐢ㄥ埌瀹冦€?

椹卞姩鍙互璇锋眰涓€涓垨澶氫釜涓荤獥鍙ｏ紙`vme_master_request`锛夈€佷粠绐楀彛
锛坄vme_slave_request`锛夊拰/鎴?DMA 閫氶亾锛坄vme_dma_request`锛夌殑鎵€鏈夋潈銆侫PI 涓嶆槸璁╄澶?
椹卞姩璇锋眰鐗瑰畾鐨勭獥鍙ｆ垨 DMA 閫氶亾锛堝彲鑳借鍏朵粬椹卞姩鍗犵敤锛夛紝鑰屾槸鏍规嵁鎵€璁ㄨ椹卞姩鎵€闇€鐨?
灞炴€ф潵鍒嗛厤璧勬簮銆傚浜庝粠绐楀彛锛岃繖浜涘睘鎬у垎涓洪渶瑕佽闂殑 VME 鍦板潃绌洪棿锛?aspace'锛夊拰
鎵€闇€鐨?VME 鎬荤嚎鍛ㄦ湡绫诲瀷锛?cycle'锛夈€備富绐楀彛鍙﹀澧炲姞涓€缁?'width' 灞炴€э紝鎸囧畾鎵€闇€鐨?
鏁版嵁浼犺緭瀹藉害銆傝繖浜涘睘鎬у畾涔変负浣嶆帺鐮侊紝鍥犳鍙互涓哄崟涓獥鍙ｈ姹備换鎰忕粍鍚堢殑灞炴€э紱鏍稿績浼?
鍒嗛厤涓€涓弧瓒宠姹傜殑绐楀彛锛屽苟杩斿洖涓€涓?vme_resource 绫诲瀷鐨勬寚閽堬紝鐢ㄤ簬鍦ㄥ悗缁娇鐢ㄥ凡鍒嗛厤
璧勬簮鏃舵爣璇嗗畠銆傚浜?DMA 鎺у埗鍣紝璇锋眰鍑芥暟闇€瑕佹彁渚涗换浣曚紶杈撳彲鑳界殑鏂瑰悜锛坮oute 灞炴€э級銆?
閫氬父鏄?VME-to-MEM 鍜?鎴?MEM-to-VME锛屼笉杩囨煇浜涚‖浠惰繕鑳芥敮鎸?VME-to-VME 涓?MEM-to-MEM
浼犺緭浠ュ強娴嬭瘯妯″紡鐢熸垚銆傚鏋滄壘涓嶅埌绗﹀悎瑕佹眰鐨勬湭鍒嗛厤绐楀彛锛屽皢杩斿洖 NULL 鎸囬拡銆?

杩樻彁渚涗簡鍦ㄤ笉鍐嶉渶瑕佹椂鍒嗛噴鏀剧獥鍙ｅ垎閰嶇殑鍑芥暟銆傝繖浜涘嚱鏁帮紙`vme_master_free`銆?
`vme_slave_free` 鍜?`vme_dma_free`锛夊簲浼犲叆璧勬簮鍒嗛厤鏃舵彁渚涚殑璧勬簮鎸囬拡銆?


### Master windows


涓荤獥鍙ｆ彁渚涗粠鏈湴澶勭悊鍣ㄨ闂?VME 鎬荤嚎鐨勮兘鍔涖€傚彲鐢ㄧ獥鍙ｆ暟閲忎互鍙婂彲鐢ㄧ殑璁块棶妯″紡鍙栧喅浜?
搴曞眰鑺墖缁勩€傜獥鍙ｅ湪浣跨敤鍓嶅繀椤诲厛閰嶇疆銆?


#### Master window configuration


涓荤獥鍙ｅ垎閰嶅悗锛屽彲鐢?`vme_master_set` 閰嶇疆瀹冿紝鐢?`vme_master_get` 鑾峰彇褰撳墠璁剧疆銆?
鍦板潃绌洪棿銆佷紶杈撳搴﹀拰鍛ㄦ湡绫诲瀷涓庤祫婧愮鐞嗕腑鎻忚堪鐨勭浉鍚岋紝浣嗗叾涓竴浜涢€夐」鏄簰鏂ョ殑銆備緥濡傦紝
鍙兘鎸囧畾涓€涓湴鍧€绌洪棿銆?


#### Master window access


鍑芥暟 `vme_master_read` 鍙敤浜庝粠宸查厤缃殑涓荤獥鍙ｈ鍙栵紝`vme_master_write` 鐢ㄤ簬鍐欏叆銆?

闄や簡绠€鍗曠殑璇诲啓锛宍vme_master_rmw` 鎻愪緵璇?淇敼-鍐欎簨鍔°€俈ME 绐楀彛鐨勯儴鍒嗗尯鍩熶篃鍙互浣跨敤
`vme_master_mmap_prepare` 鏄犲皠鍒扮敤鎴风┖闂村唴瀛樸€?


### Slave windows


浠庣獥鍙ｆ彁渚?VME 鎬荤嚎涓婄殑璁惧璁块棶鏈湴鍐呭瓨鏄犲皠鍖哄煙鐨勯€斿緞銆傚彲鐢ㄧ獥鍙ｆ暟閲忎互鍙婂彲浣跨敤鐨?
璁块棶妯″紡鍙栧喅浜庡簳灞傝姱鐗囩粍銆傜獥鍙ｅ湪浣跨敤鍓嶅繀椤诲厛閰嶇疆銆?


#### Slave window configuration


浠庣獥鍙ｅ垎閰嶅悗锛屽彲鐢?`vme_slave_set` 閰嶇疆瀹冿紝鐢?`vme_slave_get` 鑾峰彇褰撳墠璁剧疆銆?

鍦板潃绌洪棿銆佷紶杈撳搴﹀拰鍛ㄦ湡绫诲瀷涓庤祫婧愮鐞嗕腑鎻忚堪鐨勭浉鍚岋紝浣嗗叾涓竴浜涢€夐」鏄簰鏂ョ殑銆備緥濡傦紝
鍙兘鎸囧畾涓€涓湴鍧€绌洪棿銆?


#### Slave window buffer allocation


鎻愪緵浜嗕竴浜涘嚱鏁帮紝鍏佽鐢ㄦ埛鍒嗛厤锛坄vme_alloc_consistent`锛夊拰閲婃斁锛坄vme_free_consistent`锛?
涓€娈佃繛缁殑銆乂ME 妗ュ彲璁块棶鐨勭紦鍐插尯銆備笉涓€瀹氳浣跨敤杩欎簺鍑芥暟锛屼篃鍙互鐢ㄥ叾浠栨柟娉曞垎閰嶇紦鍐插尯锛?
浣嗗繀椤绘敞鎰忕‘淇濆畠浠槸杩炵画鐨勪笖 VME 妗ュ彲璁块棶銆?


#### Slave window access


浠庣獥鍙ｅ皢鏈湴鍐呭瓨鏄犲皠鍒?VME 鎬荤嚎锛屽簲浣跨敤璁块棶鍐呭瓨鐨勬爣鍑嗘柟娉曘€?


### DMA channels


VME DMA 浼犺緭鎻愪緵杩愯閾捐〃 DMA 浼犺緭鐨勮兘鍔涖€傝 API 寮曞叆浜?DMA 鍒楄〃鐨勬蹇点€傛瘡涓?DMA
鍒楄〃鏄竴涓彲浼犵粰 DMA 鎺у埗鍣ㄧ殑閾捐〃銆傚彲浠ュ垱寤恒€佹墿灞曘€佹墽琛屻€佸鐢ㄥ拰閿€姣佸涓垪琛ㄣ€?


#### List Management


鍑芥暟 `vme_new_dma_list` 鐢ㄤ簬鍒涘缓 DMA 鍒楄〃锛宍vme_dma_list_free` 鐢ㄤ簬閿€姣併€傛墽琛屽垪琛?
涓嶄細鑷姩閿€姣佸畠锛屽洜姝ゅ垪琛ㄥ彲琚鐢ㄤ簬閲嶅鎬т换鍔°€?


#### List Population


鍙互浣跨敤 `vme_dma_list_add` 鍚戝垪琛ㄦ坊鍔犱竴椤癸紙婧愬拰鐩爣鐨勫睘鎬ч渶瑕佸湪璋冪敤璇ュ嚱鏁板墠鍒涘缓锛?
杩欓儴鍒嗗湪鈥淭ransfer Attributes鈥濅腑浠嬬粛锛夈€?


	浼犺緭婧愬拰鐩爣鐨勮缁嗗睘鎬х洿鍒板悜 DMA 鍒楄〃娣诲姞鏉＄洰鏃舵墠浼氳妫€鏌ワ紱璇锋眰 DMA 閫氶亾
	鍙槸妫€鏌ユ帶鍒跺櫒棰勬湡浼犺緭鏁版嵁鐨勬柟鍚戙€傚洜姝よ繖娆¤皟鐢ㄦ湁鍙兘杩斿洖閿欒锛屼緥濡傛簮鎴?
	鐩爣浣嶄簬涓嶆敮鎸佺殑 VME 鍦板潃绌洪棿涓€?

#### Transfer Attributes


婧愬拰鐩爣鐨勫睘鎬т笌鍚戝垪琛ㄦ坊鍔犳潯鐩槸鍒嗗紑澶勭悊鐨勩€傝繖鏄洜涓烘瘡绉嶇被鍨嬬殑婧愬拰鐩爣鎵€闇€鐨勫睘鎬?
宸紓寰堝ぇ銆傛彁渚涗簡涓?PCI銆乂ME 浠ュ強 pattern锛堝湪閫傜敤鏃讹級婧愬拰鐩爣鍒涘缓灞炴€х殑鍑芥暟锛?

 - PCI 婧愭垨鐩爣锛歚vme_dma_pci_attribute`
 - VME 婧愭垨鐩爣锛歚vme_dma_vme_attribute`
 - Pattern 婧愶細`vme_dma_pattern_attribute`

鍑芥暟 `vme_dma_free_attribute` 搴旂敤浜庨噴鏀句竴涓睘鎬с€?


#### List Execution


鍑芥暟 `vme_dma_list_exec` 灏嗕竴涓垪琛ㄦ帓鍏ユ墽琛岄槦鍒楋紝骞跺湪鍒楄〃鎵ц瀹屾瘯鍚庤繑鍥炪€?


### Interrupts


VME API 鎻愪緵浜嗗皢鍥炶皟鍑芥暟鎸傛帴/鍒嗙鍒扮壒瀹?VME 鐢靛钩涓庣姸鎬?ID 缁勫悎锛屼互鍙婁互鐗瑰畾 VME 鐢靛钩
鍜岀姸鎬?ID 鐢熸垚 VME 涓柇鐨勫嚱鏁般€?


#### Attaching Interrupt Handlers


鍑芥暟 `vme_irq_request` 鍙敤浜庢寕鎺ワ紝`vme_irq_free` 鐢ㄤ簬閲婃斁涓€涓壒瀹氱殑 VME 鐢靛钩涓?
鐘舵€?ID 缁勫悎銆備换浣曚竴涓粰瀹氱殑缁勫悎鍙兘鍒嗛厤涓€涓洖璋冨嚱鏁般€傛彁渚涗簡涓€涓?void 鎸囬拡鍙傛暟锛屽叾
鍊间細浼犵粰鍥炶皟鍑芥暟锛岃鎸囬拡鐨勭敤閫旂敱鐢ㄦ埛鑷畾涔夈€傚洖璋冨嚱鏁板弬鏁板涓嬨€傜紪鍐欏洖璋冨嚱鏁版椂蹇呴』
灏忓績锛屽洖璋冨嚱鏁拌繍琛屽湪涓柇涓婁笅鏂囦腑锛?


	void callback(int level, int statid, void *priv);


#### Interrupt Generation


鍑芥暟 `vme_irq_generate` 鍙敤浜庝互缁欏畾鐨?VME 鐢靛钩鍜?VME 鐘舵€?ID 鐢熸垚 VME 涓柇銆?


### Location monitors


VME API 鎻愪緵浠ヤ笅鍔熻兘鏉ラ厤缃?location monitor锛堜綅缃洃瑙嗗櫒锛夈€?


#### Location Monitor Management


鍑芥暟 `vme_lm_request` 鐢ㄤ簬璇锋眰浣跨敤涓€鍧椾綅缃洃瑙嗗櫒锛宍vme_lm_free` 鍦ㄤ笉鍐嶉渶瑕佹椂閲婃斁瀹冧滑銆?
姣忓潡鍙彁渚涜嫢骞蹭釜浣嶇疆鐩戣鍣紝鐩戣鐩搁偦浣嶇疆銆傚嚱鏁?`vme_lm_count` 鍙敤浜庣‘瀹氭彁渚涗簡澶氬皯
涓綅缃€?


#### Location Monitor Configuration


涓€鍧椾綅缃洃瑙嗗櫒鍒嗛厤鍚庯紝鍑芥暟 `vme_lm_set` 鐢ㄤ簬閰嶇疆浣嶇疆鐩戣鍣ㄧ殑浣嶇疆鍜屾ā寮忋€傚嚱鏁?
`vme_lm_get` 鍙敤浜庤幏鍙栧凡鏈夎缃€?


#### Location Monitor Use


鍑芥暟 `vme_lm_attach` 鐢ㄤ簬鎸傛帴鍥炶皟锛宍vme_lm_detach` 鐢ㄤ簬浠庢瘡涓綅缃洃瑙嗗櫒浣嶇疆鍒嗙銆傛瘡涓?
浣嶇疆鐩戣鍣ㄥ彲浠ョ洃瑙嗚嫢骞蹭釜鐩搁偦浣嶇疆銆傚洖璋冨嚱鏁板０鏄庡涓嬨€?


	void callback(void *data);


### Slot Detection


鍑芥暟 `vme_slot_num` 杩斿洖鎵€鎻愪緵妗ョ殑鎻掓Ы ID銆?


### Bus Detection


鍑芥暟 `vme_bus_num` 杩斿洖鎵€鎻愪緵妗ョ殑鎬荤嚎 ID銆?


### VME API


   :internal:

   :export:
