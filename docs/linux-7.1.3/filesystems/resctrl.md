
## 璧勬簮鎺у埗鐗规€э紙resctrl锛夌殑鐢ㄦ埛鎺ュ彛


:Copyright: |copy| 2016 Intel Corporation
:Authors: - Fenghua Yu <fenghua.yu@intel.com>
          - Tony Luck <tony.luck@intel.com>
          - Vikas Shivappa <vikas.shivappa@intel.com>


Intel 灏嗘湰鐗规€хО涓?Intel Resource Director Technology锛圛ntel(R) RDT锛夈€?
AMD 灏嗘湰鐗规€хО涓?AMD Platform Quality of Service锛圓MD QoS锛夈€?

鏈壒鎬х敱 CONFIG_X86_CPU_RESCTRL 浠ュ強 x86 鐨?/proc/cpuinfo 鏍囧織浣嶅惎鐢細

=============================================================== ================================
RDT锛堣祫婧愬鍚戞妧鏈級鍒嗛厤			"rdt_a"
CAT锛堢紦瀛樺垎閰嶆妧鏈級				"cat_l3", "cat_l2"
CDP锛堜唬鐮佷笌鏁版嵁浼樺厛绾у垝鍒嗭級				"cdp_l3", "cdp_l2"
CQM锛堢紦瀛?QoS 鐩戞帶锛?				"cqm_llc", "cqm_occup_llc"
MBM锛堝唴瀛樺甫瀹界洃鎺э級				"cqm_mbm_total", "cqm_mbm_local"
MBA锛堝唴瀛樺甫瀹藉垎閰嶏級				"mba"
SMBA锛堟參閫熷唴瀛樺甫瀹藉垎閰嶏級				""
BMEC锛堝甫瀹界洃鎺т簨浠堕厤缃級			""
ABMC锛堝彲鍒嗛厤甯﹀鐩戞帶璁℃暟鍣級			""
SDCIAE锛堟櫤鑳芥暟鎹紦瀛樻敞鍏ュ垎閰嶅己鍒讹級	""
=============================================================== ================================

鍘嗗彶涓婏紝鏂扮壒鎬ч粯璁や細鍦?/proc/cpuinfo 涓彲瑙併€傝繖瀵艰嚧杩欎簺鐗规€ф爣蹇楀彉寰楅毦浠ヨ浜鸿В鏋愩€傚鏋?
鐢ㄦ埛绌洪棿鍙互浠?resctrl 鐨?info 鐩綍鑾峰彇鍏充簬璇ョ壒鎬х殑淇℃伅锛屽垯搴旈伩鍏嶅悜 /proc/cpuinfo
娣诲姞鏂扮殑鏍囧織銆?

```

 # mount -t resctrl resctrl [-o cdp[,cdpl2][,mba_MBps][,debug]] /sys/fs/resctrl

```
鎸傝浇閫夐」濡備笅锛?

"cdp":
		鍦?L3 缂撳瓨鍒嗛厤涓惎鐢ㄤ唬鐮?鏁版嵁浼樺厛绾у垝鍒嗐€?
"cdpl2":
		鍦?L2 缂撳瓨鍒嗛厤涓惎鐢ㄤ唬鐮?鏁版嵁浼樺厛绾у垝鍒嗐€?
"mba_MBps":
		鍚敤 MBA 杞欢鎺у埗鍣紙mba_sc锛変互 MiBps 涓哄崟浣嶆寚瀹?MBA
		甯﹀
"debug":
		浣胯皟璇曟枃浠跺彲璁块棶銆傚彲鐢ㄧ殑璋冭瘯鏂囦欢鏍囨敞鏈?
		鈥滀粎鍦ㄤ娇鐢?debug 閫夐」鏃跺彲鐢ㄢ€濄€?

L2 鍜?L3 鐨?CDP 鏄垎鍒帶鍒剁殑銆?

RDT 鍚勭壒鎬у郊姝ゆ浜ゃ€傛煇涓壒瀹氱郴缁熷彲鑳戒粎鏀寔鐩戞帶銆佷粎鏀寔鎺у埗锛屾垨鍚屾椂鏀寔鐩戞帶涓庢帶鍒躲€?
缂撳瓨浼攣瀹氾紙cache pseudo-locking锛夋槸涓€绉嶅埄鐢ㄧ紦瀛樻帶鍒跺湪缂撳瓨涓€滈拤浣忊€濇垨鈥滈攣瀹氣€濇暟鎹殑
鐙壒鏂瑰紡銆傛洿澶氱粏鑺傚彲鍙傝鈥淐ache Pseudo-Locking鈥濄€?


鎸傝浇浼氬湪鍒嗛厤鎴栫洃鎺т簩鑰呬箣涓€瀛樺湪鏃舵垚鍔燂紝浣嗗彧浼氬垱寤鸿绯荤粺鎵€鏀寔鐨勬枃浠跺拰鐩綍銆?
鍏充簬鐩戞帶鍜屽垎閰嶆湡闂磋鎺ュ彛琛屼负鐨勬洿澶氱粏鑺傦紝璇峰弬瑙佲€淩esource alloc and monitor groups鈥濅竴鑺傘€?

## Info 鐩綍


'info' 鐩綍鍖呭惈鍏充簬宸插惎鐢ㄨ祫婧愮殑淇℃伅銆傛瘡涓祫婧愰兘鏈夎嚜宸辩殑瀛愮洰褰曘€傚瓙鐩綍鍚嶇О
鍙嶆槧璧勬簮鍚嶇О銆?

璧勬簮瀛愮洰褰曚腑鐨勫ぇ澶氭暟鏂囦欢鏄彧璇荤殑锛岀敤浜庢弿杩拌璧勬簮鐨勫睘鎬с€傛敮鎸佸叏灞€閰嶇疆閫夐」鐨勮祫婧?
杩樺寘鍚彲鍐欐枃浠讹紝鍙敤浜庝慨鏀硅繖浜涜缃€?

姣忎釜瀛愮洰褰曞寘鍚互涓嬩笌鍒嗛厤鐩稿叧鐨勬枃浠讹細

缂撳瓨璧勬簮锛圠3/L2锛夊瓙鐩綍鍖呭惈浠ヤ笅涓庡垎閰嶇浉鍏崇殑鏂囦欢锛?

"num_closids":
		璇ヨ祫婧愭湁鏁堢殑 CLOSID 鏁伴噺銆傚唴鏍镐娇鐢ㄦ墍鏈夊凡鍚敤璧勬簮涓?
		鏈€灏忕殑 CLOSID 鏁伴噺浣滀负涓婇檺銆?
"cbm_mask":
		璇ヨ祫婧愭湁鏁堢殑浣嶆帺鐮併€傝鎺╃爜绛変环浜?100%銆?
"min_cbm_bits":
		鍐欏叆鎺╃爜鏃跺繀椤昏缃殑杩炵画浣嶇殑鏈€灏忔暟閲忋€?

"shareable_bits":
		涓庡叾浠栨墽琛屽疄浣擄紙渚嬪 I/O锛夊叡浜殑璧勬簮鐨勪綅鎺╃爜銆?
		閫傜敤浜庤璧勬簮鐨勬墍鏈夊疄渚嬨€傜敤鎴峰湪璁剧疆鐙崰缂撳瓨鍒嗗尯鏃?
		鍙互浣跨敤瀹冦€傛敞鎰忔煇浜涘钩鍙版敮鎸佹嫢鏈夎嚜宸辩紦瀛樹娇鐢ㄨ缃殑
		璁惧锛岃繖浜涜缃彲鑳戒細瑕嗙洊杩欎簺浣嶃€?

		褰撳惎鐢?"io_alloc" 鏃讹紝姣忎釜缂撳瓨瀹炰緥鐨勪竴閮ㄥ垎鍙互
		閰嶇疆涓哄湪纭欢鍜岃蒋浠朵箣闂村叡浜娇鐢ㄣ€傚簲浣跨敤 "bit_usage"
		鏉ユ煡鐪嬫瘡涓紦瀛樺疄渚嬩腑鍝簺閮ㄥ垎閫氳繃 "io_alloc" 鐗规€?
		閰嶇疆涓轰緵纭欢浣跨敤锛屽洜涓烘瘡涓紦瀛樺疄渚嬬殑 "io_alloc" 浣嶆帺鐮?
		閮藉彲浠ラ€氳繃 "io_alloc_cbm" 鐙珛閰嶇疆銆?

"bit_usage":
		鏍囨敞鐨勫閲忎綅鎺╃爜锛屾樉绀鸿祫婧愮殑鍏ㄩ儴瀹炰緥濡備綍琚娇鐢ㄣ€傚浘渚嬪涓嬶細

			"0":
				瀵瑰簲鍖哄煙鏈浣跨敤銆傚綋绯荤粺璧勬簮宸插垎閰嶄笖鍦?"bit_usage" 涓?
				鍙戠幇 "0" 鏃讹紝琛ㄦ槑璧勬簮琚氮璐逛簡銆?

			"H":
				瀵瑰簲鍖哄煙浠呯敱纭欢浣跨敤锛屼絾鍙緵杞欢浣跨敤銆傚鏋滄煇涓祫婧?
				鍦?"shareable_bits" 鎴?"io_alloc_cbm" 涓缃簡浣嶏紝
				浣嗗苟闈炴墍鏈夎繖浜涗綅閮藉嚭鐜板湪璧勬簮缁勭殑 schemata 涓紝鍒欏嚭鐜板湪
				"shareable_bits" 鎴?"io_alloc_cbm" 涓絾娌℃湁鍑虹幇鍦?
				浠讳綍璧勬簮缁勪腑鐨勪綅灏嗚鏍囪涓?"H"銆?
			"X":
				瀵瑰簲鍖哄煙鍙緵鍏变韩锛屽苟涓旂敱纭欢鍜岃蒋浠跺叡鍚屼娇鐢ㄣ€傝繖浜涙槸鍦?
				"shareable_bits" 鎴?"io_alloc_cbm" 涓互鍙婅祫婧愮粍
				鐨勫垎閰嶄腑鍑虹幇鐨勪綅銆?
			"S":
				瀵瑰簲鍖哄煙鐢辫蒋浠朵娇鐢紝涓斿彲渚涘叡浜€?
			"E":
				瀵瑰簲鍖哄煙琚竴涓祫婧愮粍鐙崰浣跨敤銆備笉鍏佽鍏变韩銆?
			"P":
				瀵瑰簲鍖哄煙琚吉閿佸畾銆備笉鍏佽鍏变韩銆?
"sparse_masks":
		鎸囩ず鏄惁鏀寔 CBM 涓潪杩炵画鐨?1 鍊笺€?

			"0":
				浠呮敮鎸?CBM 涓繛缁殑 1 鍊笺€?
			"1":
				鏀寔 CBM 涓潪杩炵画鐨?1 鍊笺€?

"io_alloc":
		"io_alloc" 浣跨郴缁熻蒋浠惰兘澶熼厤缃垎閰嶇粰 I/O 娴侀噺鐨勭紦瀛橀儴鍒嗐€備粎褰?
		绯荤粺鍦ㄥ叾鏌愪簺缂撳瓨璧勬簮涓婃敮鎸佽鐗规€ф椂锛岃鏂囦欢鎵嶅彲鑳藉瓨鍦ㄣ€?

			"disabled":
				璧勬簮鏀寔 "io_alloc" 浣嗚鐗规€ц绂佺敤銆傜敤浜庡垎閰?I/O 娴侀噺鐨?
				缂撳瓨閮ㄥ垎鏃犳硶閰嶇疆銆?
			"enabled":
				鐢ㄤ簬鍒嗛厤 I/O 娴侀噺鐨勭紦瀛橀儴鍒嗗彲浠ヤ娇鐢?"io_alloc_cbm" 閰嶇疆銆?
			"not supported":
				璇ヨ祫婧愪笉鏀寔姝ょ壒鎬с€?

		鍙互閫氳繃鍐欏叆鎺ュ彛鏉ヤ慨鏀硅鐗规€э紝渚嬪锛?

```

			# echo 1 > /sys/fs/resctrl/info/L3/io_alloc

		To disable::

			# echo 0 > /sys/fs/resctrl/info/L3/io_alloc

		搴曞眰瀹炵幇鍙兘浼氬噺灏戝彲鐢ㄤ簬閫氱敤锛圕PU锛夌紦瀛樺垎閰嶇殑璧勬簮銆傝鍙傞槄
		涓嬫枃鐗瑰畾鏋舵瀯鐨勮鏄庛€傛牴鎹娇鐢ㄩ渶姹傦紝璇ョ壒鎬у彲浠ュ惎鐢ㄦ垨绂佺敤銆?

		鍦?AMD 绯荤粺涓婏紝io_alloc 鐗规€х敱 L3 Smart Data Cache Injection
		Allocation Enforcement锛圫DCIAE锛夋敮鎸併€俰o_alloc 鐨?CLOSID 鏄璧勬簮
		鏀寔鐨勬渶楂?CLOSID銆傚綋 io_alloc 鍚敤鏃讹紝鏈€楂樼殑 CLOSID 涓撶敤浜?
		io_alloc锛屼笉鍐嶅彲鐢ㄤ簬閫氱敤锛圕PU锛夌紦瀛樺垎閰嶃€傚綋鍚敤 CDP 鏃讹紝io_alloc
		浣跨敤鍒嗛厤缁欐寚浠ょ紦瀛橈紙CDP_CODE锛夌殑鏈€楂?CLOSID 鏉ヨ矾鐢?I/O 娴侀噺锛?
		浣垮緱璇?CLOSID 瀵逛簬 CDP_CODE 鍜?CDP_DATA 璧勬簮閮戒笉鍐嶅彲鐢ㄤ簬
		閫氱敤锛圕PU锛夌紦瀛樺垎閰嶃€?

```
"io_alloc_cbm":
		鎻忚堪缂撳瓨瀹炰緥閮ㄥ垎鐨勫閲忎綅鎺╃爜锛屽綋 "io_alloc" 鍚敤鏃讹紝鏉ヨ嚜鍙楁敮鎸?
		I/O 璁惧鐨?I/O 娴侀噺浼氳璺敱鍒拌繖浜涚紦瀛樺疄渚嬮儴鍒嗐€?

		CBM 浠ヤ笅鍒楁牸寮忔樉绀猴細

			<cache_id0>=<cbm>;<cache_id1>=<cbm>;...

```

			# cat /sys/fs/resctrl/info/L3/io_alloc_cbm
			0=ffff;1=ffff

		CBM 鍙互閫氳繃鍐欏叆鎺ュ彛鏉ラ厤缃€?

		Example::

			# echo 1=ff > /sys/fs/resctrl/info/L3/io_alloc_cbm
			# cat /sys/fs/resctrl/info/L3/io_alloc_cbm
			0=ffff;1=00ff

			# echo "0=ff;1=f" > /sys/fs/resctrl/info/L3/io_alloc_cbm
			# cat /sys/fs/resctrl/info/L3/io_alloc_cbm
			0=00ff;1=000f

		涓€涓?"*" ID 浼氱敤鎵€鎻愪緵鐨?CBM 閰嶇疆鎵€鏈夊煙銆?

		鍦ㄤ笉闇€瑕佹帺鐮佷腑鏈€灏忚繛缁綅鏁伴噺鐨勭郴缁熶笂鐨勭ず渚?:

			# echo "*=0" > /sys/fs/resctrl/info/L3/io_alloc_cbm
			# cat /sys/fs/resctrl/info/L3/io_alloc_cbm
			0=0;1=0

		褰撳惎鐢?CDP 鏃讹紝涓?CDP_DATA 鍜?CDP_CODE 璧勬簮鍏宠仈鐨?"io_alloc_cbm"
		鍙兘鍙嶆槧鐩稿悓鐨勫€笺€備緥濡傦紝浠?/sys/fs/resctrl/info/L3DATA/io_alloc_cbm
		璇诲彇鍜屽啓鍏ョ殑鍊煎彲鑳戒細鐢?/sys/fs/resctrl/info/L3CODE/io_alloc_cbm
		鍙嶆槧鍑烘潵锛屽弽涔嬩害鐒躲€?

```
鍐呭瓨甯﹀锛圡B锛夊瓙鐩綍鍖呭惈浠ヤ笅涓庡垎閰嶇浉鍏崇殑鏂囦欢锛?

"min_bandwidth":
		鐢ㄦ埛鍙互璇锋眰鐨勬渶灏忓唴瀛樺甫瀹界櫨鍒嗘瘮銆?

"bandwidth_gran":
		鍒嗛厤鍐呭瓨甯﹀鐧惧垎姣旂殑绮掑害銆傚垎閰嶇殑
		b/w 鐧惧垎姣斾細琚垗鍏ュ埌纭欢涓婂彲鐢ㄧ殑
		涓嬩竴涓帶鍒舵闀裤€傚彲鐢ㄧ殑甯﹀鎺у埗姝ラ暱涓猴細
		min_bandwidth + N * bandwidth_gran銆?

"delay_linear":
		鎸囩ず寤惰繜鍒诲害鏄嚎鎬х殑杩樻槸闈炵嚎鎬х殑銆傝瀛楁
		绾补鏄俊鎭€х殑銆?

"thread_throttle_mode":
		鍦?Intel 绯荤粺涓婃寚绀哄綋涓€涓墿鐞嗘牳鐨勭嚎绋嬭姹備笉鍚岀殑鍐呭瓨甯﹀
		鐧惧垎姣旀椂锛岃繖浜涚嚎绋嬪浣曡鑺傛祦锛?

		"max":
			鏈€灏忕殑鐧惧垎姣旇搴旂敤浜庢墍鏈夌嚎绋?
		"per-thread":
			甯﹀鐧惧垎姣旇鐩存帴搴旂敤浜庤繍琛屽湪鏍镐笂鐨?
			绾跨▼

濡傛灉 L3 鐩戞帶鍙敤锛屽皢瀛樺湪涓€涓?"L3_MON" 鐩綍锛屽寘鍚互涓嬫枃浠讹細

"num_rmids":
		纭欢鏀寔鐨勭敤浜?L3 鐩戞帶浜嬩欢鐨?RMID 鏁伴噺銆?

"mon_features":
		濡傛灉涓鸿璧勬簮鍚敤浜嗙洃鎺э紝鍒欏垪鍑虹洃鎺т簨浠躲€?
```

			# cat /sys/fs/resctrl/info/L3_MON/mon_features
			llc_occupancy
			mbm_total_bytes
			mbm_local_bytes

		濡傛灉绯荤粺鏀寔甯﹀鐩戞帶浜嬩欢閰嶇疆锛圔MEC锛夛紝鍒欏甫瀹戒簨浠跺皢
		鍙厤缃€傝緭鍑哄皢涓?:

			# cat /sys/fs/resctrl/info/L3_MON/mon_features
			llc_occupancy
			mbm_total_bytes
			mbm_total_bytes_config
			mbm_local_bytes
			mbm_local_bytes_config

```
"mbm_total_bytes_config"銆?mbm_local_bytes_config":
		褰撴敮鎸佸甫瀹界洃鎺т簨浠堕厤缃紙BMEC锛夌壒鎬ф椂锛屽寘鍚?mbm_total_bytes
		鍜?mbm_local_bytes 浜嬩欢閰嶇疆淇℃伅鐨勮/鍐欐枃浠躲€備簨浠堕厤缃缃槸
		鍩熺壒瀹氱殑锛屽苟褰卞搷璇ュ煙涓殑鎵€鏈?CPU銆傚綋浠讳竴浜嬩欢閰嶇疆琚洿鏀规椂锛?
		璇ュ煙涓袱涓簨浠剁殑鎵€鏈?RMID 鐨勫甫瀹借鏁板櫒锛坢bm_total_bytes 浠ュ強
		mbm_local_bytes锛夐兘浼氳娓呴浂銆傚姣忎釜 RMID 鐨勫悗缁鍙栧皢鎶ュ憡
		鈥淯navailable鈥濓紝鍐嶄箣鍚庣殑璇诲彇灏嗘姤鍛婃湁鏁堝€笺€?

	鏀寔鐨勪簨浠剁被鍨嬪涓嬶細

	====    ========================================================
	浣?   鎻忚堪
	====    ========================================================
	6       鏉ヨ嚜 QoS 鍩熴€佸彂寰€鎵€鏈夌被鍨嬪唴瀛樼殑鑴忓彈瀹宠€咃紙Dirty Victims锛?
	5       瀵归潪鏈湴 NUMA 鍩熶腑鎱㈤€熷唴瀛樼殑璇诲彇
	4       瀵规湰鍦?NUMA 鍩熶腑鎱㈤€熷唴瀛樼殑璇诲彇
	3       瀵归潪鏈湴 NUMA 鍩熺殑闈炰复鏃讹紙non-temporal锛夊啓鍏?
	2       瀵规湰鍦?NUMA 鍩熺殑闈炰复鏃讹紙non-temporal锛夊啓鍏?
	1       瀵归潪鏈湴 NUMA 鍩熶腑鍐呭瓨鐨勮鍙?
	0       瀵规湰鍦?NUMA 鍩熶腑鍐呭瓨鐨勮鍙?
	====    ========================================================

	榛樿鎯呭喌涓嬶紝mbm_total_bytes 閰嶇疆琚涓?0x7f 浠ョ粺璁℃墍鏈変簨浠剁被鍨嬶紝
	mbm_local_bytes 閰嶇疆琚涓?0x15 浠ョ粺璁℃墍鏈夋湰鍦板唴瀛樹簨浠躲€?

	绀轰緥锛?

```

	  ::

	    # cat /sys/fs/resctrl/info/L3_MON/mbm_total_bytes_config
	    0=0x7f;1=0x7f;2=0x7f;3=0x7f

	    # cat /sys/fs/resctrl/info/L3_MON/mbm_local_bytes_config
	    0=0x15;1=0x15;3=0x15;4=0x15

	* 瑕佸皢 mbm_total_bytes 鏀逛负鍙粺璁″煙 0 涓婄殑璇绘搷浣滐紝闇€瑕佽缃綅 0銆?銆? 鍜?5锛?
	  鍗充簩杩涘埗鐨?110011b锛堝崄鍏繘鍒?0x33锛夛細
	  ::

	    # echo  "0=0x33" > /sys/fs/resctrl/info/L3_MON/mbm_total_bytes_config

	    # cat /sys/fs/resctrl/info/L3_MON/mbm_total_bytes_config
	    0=0x33;1=0x7f;2=0x7f;3=0x7f

	* 瑕佸皢 mbm_local_bytes 鏀逛负缁熻鍩?0 鍜屽煙 1 涓婃墍鏈夋參閫熷唴瀛樿鎿嶄綔锛?
	  闇€瑕佽缃綅 4 鍜?5锛屽嵆浜岃繘鍒剁殑 110000b锛堝崄鍏繘鍒?0x30锛夛細
	  ::

	    # echo  "0=0x30;1=0x30" > /sys/fs/resctrl/info/L3_MON/mbm_local_bytes_config

	    # cat /sys/fs/resctrl/info/L3_MON/mbm_local_bytes_config
	    0=0x30;1=0x30;3=0x15;4=0x15

```
"mbm_assign_mode":
	鏀寔鐨勮鏁板櫒鍒嗛厤妯″紡銆傛柟鎷彿琛ㄧず褰撳墠鍚敤鐨勬ā寮忋€傚綋 "mbm_assign_mode"
	琚洿鏀规椂锛屼笌璁℃暟鍣ㄥ叧鑱旂殑 MBM 浜嬩欢鍙兘浼氬浣嶃€?
```

	  # cat /sys/fs/resctrl/info/L3_MON/mbm_assign_mode
	  [mbm_event]
	  default

	"mbm_event":

	mbm_event 妯″紡鍏佽鐢ㄦ埛灏嗙‖浠惰鏁板櫒鍒嗛厤缁?RMID銆佷簨浠跺锛屽苟鍦ㄥ垎閰嶆湡闂?
	鐩戞帶甯﹀浣跨敤鎯呭喌銆傜‖浠朵細鎸佺画璺熻釜宸插垎閰嶇殑璁℃暟鍣紝鐩村埌鐢ㄦ埛鏄惧紡瑙ｉ櫎鍒嗛厤銆?
	resctrl 缁勫唴鐨勬瘡涓簨浠跺彲浠ョ嫭绔嬪垎閰嶃€?

	鍦ㄦ妯″紡涓嬶紝鐩戞帶浜嬩欢鍙湁鍦ㄦ湁纭欢璁℃暟鍣ㄦ敮鎾戞椂鎵嶈兘绱姞鏁版嵁銆備娇鐢ㄦ瘡涓?
	CTRL_MON 鍜?MON 缁勪腑鐨?"mbm_L3_assignments" 鏉ユ寚瀹氬摢浜涗簨浠跺簲鍒嗛厤璁℃暟鍣ㄣ€?
	鍙敤璁℃暟鍣ㄦ暟閲忓湪 "num_mbm_cntrs" 鏂囦欢涓弿杩般€傛洿鏀规ā寮忓彲鑳藉鑷磋祫婧愪笂鐨?
	鎵€鏈夎鏁板櫒澶嶄綅銆?

	鍒囨崲鍒?mbm_event 璁℃暟鍣ㄥ垎閰嶆ā寮忛渶瑕佺敤鎴峰皢璁℃暟鍣ㄥ垎閰嶇粰浜嬩欢銆傚惁鍒欙紝
	MBM 浜嬩欢璁℃暟鍣ㄥ湪璇诲彇鏃跺皢杩斿洖 'Unassigned'銆?

	璇ユā寮忓鏀寔姣斿彲鐢ㄧ‖浠惰鏁板櫒鏇村 CTRL_MON 鍜?MON 缁勭殑 AMD 骞冲彴鏈夌泭銆?
	榛樿鎯呭喌涓嬶紝璇ョ壒鎬у湪鍏锋湁 ABMC锛圓ssignable Bandwidth Monitoring Counters锛?
	鑳藉姏鐨?AMD 骞冲彴涓婂惎鐢紝纭繚鍗充娇鐩稿簲鐨?RMID 鏈浠讳綍澶勭悊鍣ㄤ富鍔ㄤ娇鐢紝
	璁℃暟鍣ㄤ篃淇濇寔鍒嗛厤鐘舵€併€?

	"default":

	鍦ㄩ粯璁ゆā寮忎笅锛宺esctrl 鍋囪姣忎釜 CTRL_MON 鍜?MON 缁勪腑鐨勬瘡涓簨浠堕兘鏈?
	涓€涓‖浠惰鏁板櫒銆傚湪 AMD 骞冲彴涓婏紝寤鸿浣跨敤 mbm_event 妯″紡锛堣嫢鏀寔锛夛紝
	浠ラ槻姝㈢敱浜庣‖浠堕噸鏂板垎閰嶈鏁板櫒瀵艰嚧璇诲彇涔嬮棿 MBM 浜嬩欢澶嶄綅銆傚鏋滄病鏈?
	涓轰簨浠跺垎閰嶈鏁板櫒锛岃繖鍙兘瀵艰嚧璇鎬ф暟鍊兼垨鏄剧ず "Unavailable"銆?

	* 鍚敤 "mbm_event" 璁℃暟鍣ㄥ垎閰嶆ā寮忥細
	  ::

	    # echo "mbm_event" > /sys/fs/resctrl/info/L3_MON/mbm_assign_mode

	* 鍚敤 "default" 鐩戞帶妯″紡锛?
	  ::

	    # echo "default" > /sys/fs/resctrl/info/L3_MON/mbm_assign_mode

```
"num_mbm_cntrs":
	褰撶郴缁熸敮鎸?mbm_event 妯″紡鏃讹紝姣忎釜鍩熶腑璁℃暟鍣紙鍙敤涓庡凡鍒嗛厤璁℃暟鍣ㄤ箣鍜岋級
	鐨勬渶澶ф暟閲忋€?

	渚嬪锛屽湪涓€涓瘡涓?L3 鍩熸渶澶氭湁 32 涓唴瀛樺甫瀹界洃鎺ц鏁板櫒鐨勭郴缁熶笂锛?
```

	  # cat /sys/fs/resctrl/info/L3_MON/num_mbm_cntrs
	  0=32;1=32

```
"available_mbm_cntrs":
	褰撶郴缁熶笂鍚敤浜?mbm_event 妯″紡鏃讹紝姣忎釜鍩熶腑鍙敤浜庡垎閰嶇殑璁℃暟鍣ㄦ暟閲忋€?

	渚嬪锛屽湪涓€涓瘡涓?L3 鍩熸湁 30 涓彲鐢╗纭欢]鍙垎閰嶈鏁板櫒鐨勭郴缁熶笂锛?
```

	  # cat /sys/fs/resctrl/info/L3_MON/available_mbm_cntrs
	  0=30;1=30

```
"event_configs":
	褰撴敮鎸?"mbm_event" 璁℃暟鍣ㄥ垎閰嶆ā寮忔椂瀛樺湪鐨勭洰褰曘€備负姣忎釜鍙垎閰嶇粰璁℃暟鍣ㄧ殑
	MBM 浜嬩欢鍖呭惈涓€涓瓙鐩綍銆?

	榛樿鏀寔涓や釜 MBM 浜嬩欢锛歮bm_local_bytes 鍜?mbm_total_bytes銆傛瘡涓?MBM 浜嬩欢鐨?
	瀛愮洰褰曞寘鍚竴涓悕涓?"event_filter" 鐨勬枃浠讹紝鐢ㄤ簬鏌ョ湅鍜屼慨鏀硅 MBM 浜嬩欢
	閰嶇疆鐨勬槸鍝簺鍐呭瓨浜嬪姟銆傝鏂囦欢浠呭湪鍚敤 "mbm_event" 璁℃暟鍣ㄥ垎閰嶆ā寮忔椂
	鍙闂€?

	鏀寔鐨勫唴瀛樹簨鍔＄被鍨嬪垪琛細

	==========================  ========================================================
	鍚嶇О			    鎻忚堪
	==========================  ========================================================
	dirty_victim_writes_all     鏉ヨ嚜 QoS 鍩熴€佸彂寰€鎵€鏈夌被鍨嬪唴瀛樼殑鑴忓彈瀹宠€咃紙Dirty Victims锛?
	remote_reads_slow_memory    瀵归潪鏈湴 NUMA 鍩熶腑鎱㈤€熷唴瀛樼殑璇诲彇
	local_reads_slow_memory     瀵规湰鍦?NUMA 鍩熶腑鎱㈤€熷唴瀛樼殑璇诲彇
	remote_non_temporal_writes  瀵归潪鏈湴 NUMA 鍩熺殑闈炰复鏃讹紙non-temporal锛夊啓鍏?
	local_non_temporal_writes   瀵规湰鍦?NUMA 鍩熺殑闈炰复鏃讹紙non-temporal锛夊啓鍏?
	remote_reads                瀵归潪鏈湴 NUMA 鍩熶腑鍐呭瓨鐨勮鍙?
	local_reads                 瀵规湰鍦?NUMA 鍩熶腑鍐呭瓨鐨勮鍙?
	==========================  ========================================================

```

	  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_total_bytes/event_filter
	  local_reads,remote_reads,local_non_temporal_writes,remote_non_temporal_writes,
	  local_reads_slow_memory,remote_reads_slow_memory,dirty_victim_writes_all

	  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_local_bytes/event_filter
	  local_reads,local_non_temporal_writes,local_reads_slow_memory

	閫氳繃鍐欏叆 "event_configs" 鐩綍涓殑 "event_filter" 鏂囦欢鏉ヤ慨鏀逛簨浠堕厤缃€?
	璇?鍐?"event_filter" 鏂囦欢鍖呭惈璇ヤ簨浠剁殑閰嶇疆锛屽弽鏄犺鍏剁粺璁＄殑鏄摢浜涘唴瀛樹簨鍔°€?

	渚嬪::

	  # echo "local_reads, local_non_temporal_writes" >
	    /sys/fs/resctrl/info/L3_MON/event_configs/mbm_total_bytes/event_filter

	  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_total_bytes/event_filter
	   local_reads,local_non_temporal_writes

```
"mbm_assign_on_mkdir":
	褰撴敮鎸?"mbm_event" 璁℃暟鍣ㄥ垎閰嶆ā寮忔椂瀛樺湪銆備粎褰撳惎鐢?"mbm_event" 璁℃暟鍣?
	鍒嗛厤妯″紡鏃跺彲璁块棶銆?

	鍐冲畾鍦ㄤ娇鐢?mkdir 鍒涘缓鍏跺叧鑱旂殑鐩戞帶缁勬椂锛屾槸鍚﹁嚜鍔ㄥ皢璁℃暟鍣ㄥ垎閰嶇粰 RMID銆丮BM 浜嬩欢瀵广€?
	鍦ㄥ惎鍔ㄦ椂榛樿鍚敤锛屼粠 "default" 妯″紡鍒囨崲鍒?"mbm_event" 璁℃暟鍣ㄥ垎閰嶆ā寮忔椂
	涔熼粯璁ゅ惎鐢ㄣ€傜敤鎴峰彲浠ラ€氳繃鍐欏叆鎺ュ彛鏉ョ鐢ㄦ鑳藉姏銆?

	"0":
		鑷姩鍒嗛厤琚鐢ㄣ€?
	"1":
		鑷姩鍒嗛厤琚惎鐢ㄣ€?

```

	  # echo 0 > /sys/fs/resctrl/info/L3_MON/mbm_assign_on_mkdir
	  # cat /sys/fs/resctrl/info/L3_MON/mbm_assign_on_mkdir
	  0

```
"max_threshold_occupancy":
		璇?鍐欐枃浠讹紝鎻愪緵鍏堝墠浣跨敤鐨?LLC_occupancy 璁℃暟鍣ㄥ彲琚€冭檻
		澶嶇敤鏃剁殑鏈€澶у€硷紙浠ュ瓧鑺備负鍗曚綅锛夈€?

濡傛灉閬ユ祴鐩戞帶鍙敤锛屽皢瀛樺湪涓€涓?"PERF_PKG_MON" 鐩綍锛屽寘鍚互涓嬫枃浠讹細

"num_rmids":
		鐢ㄤ簬閬ユ祴鐩戞帶浜嬩欢鐨?RMID 鏁伴噺銆?

		鍦?Intel 涓婏紝濡傛灉鍙苟鍙戣窡韪殑 RMID 鏁伴噺浣庝簬鏀寔鐨?RMID 鎬绘暟锛?
		resctrl 灏嗕笉浼氬惎鐢ㄩ仴娴嬩簨浠躲€傚彲浠ヤ娇鐢?"rdt=" 鍐呮牳鍙傛暟寮哄埗鍚敤
		閬ユ祴浜嬩欢锛屼絾杩欏彲鑳戒細鍑忓皯鍙垱寤虹殑鐩戞帶缁勬暟閲忋€?

"mon_features":
		鍒楀嚭璇ョ郴缁熶笂宸插惎鐢ㄧ殑閬ユ祴鐩戞帶浜嬩欢銆?

鍙垱寤虹殑 "CTRL_MON" + "MON" 鏁伴噺涓婇檺锛屾槸 L3_MON 鍜?PERF_PKG_MON 鐨?
"num_rmids" 鍊间腑鐨勮緝灏忚€呫€?

鏈€鍚庯紝鍦?"info" 鐩綍鐨勯《灞傛湁涓€涓悕涓?"last_cmd_status" 鐨勬枃浠躲€傚畠闅忕潃姣忔閫氳繃
鏂囦欢绯荤粺鍙戝嚭鐨勨€滃懡浠も€濓紙鍒涘缓鏂扮洰褰曟垨鍐欏叆浠讳綍鎺у埗鏂囦欢锛夎€岄噸缃€傚鏋滃懡浠ゆ垚鍔燂紝
瀹冨皢璇讳负 "ok"銆傚鏋滃懡浠ゅけ璐ワ紝瀹冨皢鎻愪緵姣旀枃浠舵搷浣滈敊璇繑鍥炰腑鑳戒紶杈剧殑鏇村淇℃伅銆備緥濡傦細
```

	# echo L3:0=f7 > schemata
	bash: echo: write error: Invalid argument
	# cat info/last_cmd_status
	mask f7 has non-consecutive 1-bits

```
## 璧勬簮鍒嗛厤涓庣洃鎺х粍


璧勬簮缁勫湪 resctrl 鏂囦欢绯荤粺涓互鐩綍褰㈠紡琛ㄧず銆傞粯璁ょ粍鏄牴鐩綍锛屾寕杞藉悗
绔嬪嵆鎷ユ湁绯荤粺涓殑鎵€鏈変换鍔″拰 CPU锛屽苟鍙互鍏呭垎鍒╃敤鎵€鏈夎祫婧愩€?

鍦ㄥ叿鏈?RDT 鎺у埗鐗规€х殑绯荤粺涓婏紝鍙互鍦ㄦ牴鐩綍涓垱寤洪澶栫殑鐩綍锛岀敤浠ユ寚瀹?
姣忕璧勬簮鐨勪笉鍚屾暟閲忥紙鍙傝涓嬫枃鐨?"schemata"锛夈€傛牴鐩綍浠ュ強杩欎簺棰濆鐨勯《绾?
鐩綍鍦ㄤ笅鏂囦腑琚О涓?"CTRL_MON" 缁勩€?

鍦ㄥ叿鏈?RDT 鐩戞帶鐨勭郴缁熶笂锛屾牴鐩綍鍜屽叾浠栭《绾х洰褰曞寘鍚竴涓悕涓?"mon_groups" 鐨?
鐩綍锛屽湪鍏朵腑鍙互鍒涘缓棰濆鐨勭洰褰曟潵鐩戞帶浣滀负鍏剁鍏堢殑 CTRL_MON 缁勪腑浠诲姟鐨勫瓙闆嗐€?
鍦ㄦ湰鏂囨。鐨勫叾浣欓儴鍒嗕腑锛岃繖浜涜绉颁负 "MON" 缁勩€?

绉婚櫎涓€涓洰褰曚細灏嗗叾鎵€浠ｈ〃鐨勭粍鎷ユ湁鐨勬墍鏈変换鍔″拰 CPU 绉诲姩鍒扮埗鐩綍銆傜Щ闄ゆ煇涓?
宸插垱寤虹殑 CTRL_MON 缁勪細鑷姩绉婚櫎鍏朵笅鏂圭殑鎵€鏈?MON 缁勩€?

鏀寔灏?MON 缁勭洰褰曠Щ鍔ㄥ埌鏂扮殑鐖?CTRL_MON 缁勶紝鐩殑鏄湪涓嶅奖鍝嶅叾鐩戞帶鏁版嵁鎴?
宸插垎閰嶄换鍔＄殑鎯呭喌涓嬫洿鏀?MON 缁勭殑璧勬簮鍒嗛厤銆傚浜庣洃鎺?CPU 鐨?MON 缁勪笉鍏佽
姝ゆ搷浣溿€傞櫎浜嗙畝鍗曞湴閲嶅懡鍚?CTRL_MON 鎴?MON 缁勪箣澶栵紝褰撳墠涓嶅厑璁稿叾浠栫Щ鍔ㄦ搷浣溿€?

鎵€鏈夌粍閮藉寘鍚互涓嬫枃浠讹細

"tasks":
		璇诲彇璇ユ枃浠朵細鏄剧ず灞炰簬璇ョ粍鐨勬墍鏈変换鍔＄殑鍒楄〃銆傚悜鏂囦欢鍐欏叆涓€涓?
		浠诲姟 id 浼氬皢璇ヤ换鍔℃坊鍔犲埌缁勪腑銆傚彲浠ラ€氳繃鐢ㄩ€楀彿鍒嗛殧浠诲姟 id 鏉?
		娣诲姞澶氫釜浠诲姟銆備换鍔″皢鎸夐『搴忓垎閰嶃€備笉鏀寔澶氫釜澶辫触銆傚湪灏濊瘯鍒嗛厤
		浠诲姟鏃堕亣鍒扮殑鍗曟澶辫触灏嗗鑷存搷浣滀腑姝紝鑰屽け璐ヤ箣鍓嶅凡娣诲姞鐨勪换鍔?
		灏嗕繚鐣欏湪缁勪腑銆傚け璐ュ皢琚褰曞埌 /sys/fs/resctrl/info/last_cmd_status銆?

		濡傛灉璇ョ粍鏄?CTRL_MON 缁勶紝鍒欒浠诲姟浼氫粠鍏堝墠鎷ユ湁璇ヤ换鍔＄殑 CTRL_MON 缁?
		浠ュ強浠讳綍鎷ユ湁璇ヤ换鍔＄殑 MON 缁勪腑绉婚櫎銆傚鏋滆缁勬槸 MON 缁勶紝鍒欒浠诲姟
		蹇呴』宸茬粡灞炰簬璇ョ粍鐨?CTRL_MON 鐖剁粍銆傝浠诲姟浼氫粠浠讳綍鍏堝墠鐨?MON 缁勪腑绉婚櫎銆?


"cpus":
		璇诲彇璇ユ枃浠朵細鏄剧ず璇ョ粍鎷ユ湁鐨勯€昏緫 CPU 鐨勪綅鎺╃爜銆傚悜璇ユ枃浠跺啓鍏?
		涓€涓帺鐮佷細鍚戣缁勬坊鍔犳垨浠庝腑绉婚櫎 CPU銆備笌 tasks 鏂囦欢涓€鏍凤紝浼氱淮鎶?
		涓€涓眰绾у叧绯伙紝鍗?MON 缁勫彧鑳藉寘鍚埗 CTRL_MON 缁勬嫢鏈夌殑 CPU銆?
		褰撹祫婧愮粍澶勪簬浼攣瀹氭ā寮忔椂锛岃鏂囦欢灏嗕粎涓哄彧璇伙紝鍙嶆槧涓庝吉閿佸畾
		鍖哄煙鍏宠仈鐨?CPU銆?


"cpus_list":
		涓?"cpus" 绫讳技锛屽彧鏄娇鐢?CPU 鑼冨洿鑰屼笉鏄綅鎺╃爜銆?


褰撳惎鐢ㄦ帶鍒舵椂锛屾墍鏈?CTRL_MON 缁勮繕灏嗗寘鍚細

"schemata":
		鍙緵璇ョ粍浣跨敤鐨勬墍鏈夎祫婧愮殑鍒楄〃銆傛瘡涓祫婧愭湁鑷繁鐨勮鍜屾牸寮?
		鈥斺€旇瑙佷笅鏂囥€?

"size":
		闀滃儚 "schemata" 鏂囦欢鐨勬樉绀猴紝浠ュ瓧鑺傛樉绀烘瘡涓垎閰嶇殑澶у皬锛?
		鑰屼笉鏄樉绀轰唬琛ㄥ垎閰嶇殑浣嶃€?

"mode":
		璧勬簮缁勭殑 "mode" 鍐冲畾鍏跺垎閰嶇殑鍏变韩鏂瑰紡銆?shareable" 璧勬簮缁?
		鍏佽鍏变韩鍏跺垎閰嶏紝鑰?"exclusive" 璧勬簮缁勫垯涓嶅厑璁搞€傜紦瀛樹吉閿佸畾
		鍖哄煙鏄€氳繃鍏堝悜 "mode" 鏂囦欢鍐欏叆 "pseudo-locksetup"锛屽啀灏嗙紦瀛?
		浼攣瀹氬尯鍩熺殑 schemata 鍐欏叆璧勬簮缁勭殑 "schemata" 鏂囦欢鏉ュ垱寤虹殑銆?
		浼攣瀹氬尯鍩熷垱寤烘垚鍔熷悗锛屾ā寮忎細鑷姩鍙樹负 "pseudo-locked"銆?

"ctrl_hw_id":
		浠呭湪浣跨敤 debug 閫夐」鏃跺彲鐢ㄣ€傜‖浠剁敤浜庢爣璇嗘帶鍒剁粍鐨勬爣璇嗙銆?
		鍦?x86 涓婅繖灏辨槸 CLOSID銆?

褰撳惎鐢ㄧ洃鎺ф椂锛屾墍鏈?MON 缁勮繕灏嗗寘鍚細

"mon_data":
		瀹冨寘鍚瘡涓洃鎺у煙鐨勭洰褰曘€?

		濡傛灉鍚敤浜?L3 鐩戞帶锛屽皢涓烘瘡涓?L3 缂撳瓨瀹炰緥鎻愪緵涓€涓?"mon_L3_XX" 鐩綍銆?
		姣忎釜鐩綍鍖呭惈宸插惎鐢?L3 浜嬩欢鐨勬枃浠讹紙渚嬪 "llc_occupancy"銆?
		"mbm_total_bytes" 鍜?"mbm_local_bytes"锛夈€?

		濡傛灉鍚敤浜嗛仴娴嬬洃鎺э紝灏嗕负姣忎釜鐗╃悊澶勭悊鍣ㄥ皝瑁呮彁渚涗竴涓?"mon_PERF_PKG_YY"
		鐩綍銆傛瘡涓洰褰曞寘鍚凡鍚敤閬ユ祴浜嬩欢鐨勬枃浠讹紙渚嬪 "core_energy"銆?
		"activity"銆?uops_retired" 绛夛級銆?

		info/`*`/mon_features 鏂囦欢鎻愪緵宸插惎鐢ㄤ簨浠?鏂囦欢鍚嶇殑瀹屾暣鍒楄〃銆?

		"core energy" 鎶ュ憡涓€涓诞鐐规暟锛岃〃绀哄湪褰撳墠鐩戞帶缁勬墍瀵瑰簲鐨勫皝瑁呬笂锛?
		鎵€鏈夐€昏緫 CPU 鍦ㄦ墽琛屾寚浠ゆ湡闂存牳蹇冿紙瀵勫瓨鍣ㄣ€佺畻鏈崟鍏冦€乀LB 鍜?
		L1/L2 缂撳瓨锛夋秷鑰楃殑鑳介噺锛堜互鐒﹁€充负鍗曚綅锛夈€?

		"activity" 涔熸姤鍛婁竴涓诞鐐瑰€硷紙浠ユ硶鎷夌涓哄崟浣嶏級銆傚畠鎻愪緵涓?CPU
		鐢ㄤ簬鎵ц鐨勯鐜囨棤鍏崇殑宸插畬鎴愬伐浣滅殑浼拌銆?

		娉ㄦ剰 "core energy" 鍜?"activity" 浠呮祴閲?CPU "core" 涓殑鑳介噺/娲诲姩
		锛堢畻鏈崟鍏冦€乀LB銆丩1 鍜?L2 缂撳瓨绛夛級銆傚畠浠笉鍖呮嫭 L3 缂撳瓨銆佸唴瀛樸€?
		I/O 璁惧绛夈€?

		鎵€鏈夊叾浠栦簨浠舵姤鍛婂崄杩涘埗鏁存暟鍊笺€?

		鍦?MON 缁勪腑锛岃繖浜涙枃浠舵彁渚涜缁勪腑鎵€鏈変换鍔＄殑浜嬩欢褰撳墠鍊肩殑璇绘暟銆?
		鍦?CTRL_MON 缁勪腑锛岃繖浜涙枃浠舵彁渚?CTRL_MON 缁勪互鍙婃墍鏈?MON 缁勪腑
		鎵€鏈変换鍔＄殑鍚堣鍊笺€傛洿澶氫娇鐢ㄧ粏鑺傝鍙傝绀轰緥閮ㄥ垎銆?

		鍦ㄥ惎鐢ㄤ簡 Sub-NUMA Cluster锛圫NC锛夌殑绯荤粺涓婏紝姣忎釜鑺傜偣閮芥湁棰濆鐨?
		鐩綍锛堜綅浜庡叾鎵€鍗犳嵁鐨?L3 缂撳瓨鐨?"mon_L3_XX" 鐩綍鍐咃級銆傝繖浜涚洰褰?
		鍛藉悕涓?"mon_sub_L3_YY"锛屽叾涓?"YY" 鏄妭鐐圭紪鍙枫€?

		褰撳惎鐢?'mbm_event' 璁℃暟鍣ㄥ垎閰嶆ā寮忔椂锛屽鏋?MON 缁勭殑鏌愪釜 MBM 浜嬩欢
		娌℃湁鍒嗛厤纭欢璁℃暟鍣紝璇诲彇璇ヤ簨浠跺皢杩斿洖 'Unassigned'銆傚浜?CTRL_MON
		缁勶紝濡傛灉鏌愪釜 MBM 浜嬩欢鍦?CTRL_MON 缁勫強鍏朵换浣曞叧鑱旂殑 MON 缁勪腑閮芥病鏈?
		宸插垎閰嶇殑璁℃暟鍣紝鍒欒繑鍥?'Unassigned'銆?

"mon_hw_id":
		浠呭湪浣跨敤 debug 閫夐」鏃跺彲鐢ㄣ€傜‖浠剁敤浜庢爣璇嗙洃鎺х粍鐨勬爣璇嗙銆?
		鍦?x86 涓婅繖灏辨槸 RMID銆?

褰撳惎鐢ㄧ洃鎺ф椂锛屾墍鏈?MON 缁勮繕鍙兘鍖呭惈锛?

"mbm_L3_assignments":
		褰撴敮鎸?"mbm_event" 璁℃暟鍣ㄥ垎閰嶆ā寮忔椂瀛樺湪锛屽苟鍒楀嚭璇ョ粍鐨勮鏁板櫒
		鍒嗛厤鐘舵€併€?

		鍒嗛厤鍒楄〃浠ヤ笅鍒楁牸寮忔樉绀猴細

	<Event>:<Domain ID>=<Assignment state>;<Domain ID>=<Assignment state>

	Event: 涓€涓湁鏁堢殑 MBM 浜嬩欢锛屼綅浜?
	       /sys/fs/resctrl/info/L3_MON/event_configs 鐩綍涓€?

	Domain ID: 涓€涓湁鏁堢殑鍩?ID銆傚啓鍏ユ椂锛?*' 灏嗘洿鏀瑰簲鐢ㄥ埌
		   鎵€鏈夊煙銆?

	Assignment states:

	_ : 鏈垎閰嶈鏁板櫒銆?

	e : 浠ョ嫭鍗犳柟寮忓垎閰嶄簡璁℃暟鍣ㄣ€?

	绀轰緥锛?

	鏄剧ず榛樿缁勭殑璁℃暟鍣ㄥ垎閰嶇姸鎬併€?
```

	 # cd /sys/fs/resctrl
	 # cat /sys/fs/resctrl/mbm_L3_assignments
	   mbm_total_bytes:0=e;1=e
	   mbm_local_bytes:0=e;1=e

	鍒嗛厤鍙互閫氳繃鍐欏叆鎺ュ彛鏉ヤ慨鏀广€?

	绀轰緥锛?

	瑙ｉ櫎鍩?0 涓婁笌 mbm_total_bytes 浜嬩欢鍏宠仈鐨勮鏁板櫒鐨勫垎閰嶏細
	::

	 # echo "mbm_total_bytes:0=_" > /sys/fs/resctrl/mbm_L3_assignments
	 # cat /sys/fs/resctrl/mbm_L3_assignments
	   mbm_total_bytes:0=_;1=e
	   mbm_local_bytes:0=e;1=e

	瑙ｉ櫎鎵€鏈夊煙涓婁笌 mbm_total_bytes 浜嬩欢鍏宠仈鐨勮鏁板櫒鐨勫垎閰嶏細
	::

	 # echo "mbm_total_bytes:*=_" > /sys/fs/resctrl/mbm_L3_assignments
	 # cat /sys/fs/resctrl/mbm_L3_assignments
	   mbm_total_bytes:0=_;1=_
	   mbm_local_bytes:0=e;1=e

	浠ョ嫭鍗犳ā寮忎负鎵€鏈夊煙鍒嗛厤涓?mbm_total_bytes 浜嬩欢鍏宠仈鐨勮鏁板櫒锛?
	::

	 # echo "mbm_total_bytes:*=e" > /sys/fs/resctrl/mbm_L3_assignments
	 # cat /sys/fs/resctrl/mbm_L3_assignments
	   mbm_total_bytes:0=e;1=e
	   mbm_local_bytes:0=e;1=e

```
褰撲娇鐢?"mba_MBps" 鎸傝浇閫夐」鏃讹紝鎵€鏈?CTRL_MON 缁勮繕灏嗗寘鍚細

"mba_MBps_event":
		璇诲彇璇ユ枃浠朵細鏄剧ず鍝釜鍐呭瓨甯﹀浜嬩欢琚敤浣滆蒋浠跺弽棣堝洖璺殑杈撳叆锛?
		璇ュ洖璺娇鍐呭瓨甯﹀淇濇寔鍦?schemata 鏂囦欢涓寚瀹氱殑鍊间互涓嬨€傚啓鍏ュ湪
		/sys/fs/resctrl/info/L3_MON/mon_features 涓壘鍒扮殑鏌愪釜鍙楁敮鎸?
		鍐呭瓨甯﹀浜嬩欢鐨勫悕绉帮紝鍙洿鏀硅緭鍏ヤ簨浠躲€?

### 璧勬簮鍒嗛厤瑙勫垯


褰撲换鍔¤繍琛屾椂锛屼互涓嬭鍒欏畾涔変簡鍝簺璧勬簮瀵瑰叾鍙敤锛?

1) 濡傛灉浠诲姟鏄潪榛樿缁勭殑鎴愬憳锛屽垯浣跨敤璇ョ粍鐨?schemata銆?

2) 鍚﹀垯锛屽鏋滀换鍔″睘浜庨粯璁ょ粍锛屼絾杩愯鍦ㄥ垎閰嶇粰鏌愪釜鐗瑰畾缁勭殑 CPU 涓婏紝鍒?
   浣跨敤璇?CPU 鎵€鍦ㄧ粍鐨?schemata銆?

3) 鍏朵粬鎯呭喌涓嬶紝浣跨敤榛樿缁勭殑 schemata銆?

### 璧勬簮鐩戞帶瑙勫垯


1) 濡傛灉浠诲姟鏄?MON 缁勬垨闈為粯璁?CTRL_MON 缁勭殑鎴愬憳锛屽垯璇ヤ换鍔＄殑 RDT 浜嬩欢
   灏嗗湪璇ョ粍涓姤鍛娿€?

2) 濡傛灉浠诲姟鏄粯璁?CTRL_MON 缁勭殑鎴愬憳锛屼絾杩愯鍦ㄥ垎閰嶇粰鏌愪釜鐗瑰畾缁勭殑 CPU 涓婏紝
   鍒欒浠诲姟鐨?RDT 浜嬩欢灏嗗湪璇ョ粍涓姤鍛娿€?

3) 鍏朵粬鎯呭喌涓嬶紝璇ヤ换鍔＄殑 RDT 浜嬩欢灏嗗湪鏍圭骇鍒殑 "mon_data" 缁勪腑鎶ュ憡銆?


## 缂撳瓨鍗犵敤鐩戞帶涓庢帶鍒剁殑娉ㄦ剰浜嬮」


褰撳皢浠诲姟浠庝竴涓粍绉诲姩鍒板彟涓€涓粍鏃讹紝浣犲簲褰撹浣忚繖鍙奖鍝嶄换鍔?*鏂?*鐨?
缂撳瓨鍒嗛厤銆備緥濡傦紝浣犲彲鑳芥湁涓€涓换鍔″湪鐩戞帶缁勪腑鏄剧ず 3 MB 鐨勭紦瀛樺崰鐢ㄣ€?
濡傛灉浣犵Щ鍔ㄥ埌涓€涓柊缁勫苟绔嬪嵆妫€鏌ユ棫缁勫拰鏂扮粍鐨勫崰鐢紝浣犲緢鍙兘浼氱湅鍒版棫缁?
浠嶆樉绀?3 MB锛岃€屾柊缁勪负闆躲€傚綋浠诲姟璁块棶绉诲姩鍓嶄粛鍦ㄧ紦瀛樹腑鐨勪綅缃椂锛岀‖浠?
涓嶄細鏇存柊浠讳綍璁℃暟鍣ㄣ€傚湪绻佸繖鐨勭郴缁熶笂锛屼綘浼氱湅鍒版棫缁勭殑鍗犵敤闅忕潃缂撳瓨琛岃
椹遍€愬拰閲嶇敤鑰屼笅闄嶏紝鑰屾柊缁勭殑鍗犵敤闅忕潃浠诲姟璁块棶鍐呭瓨骞跺姞杞藉埌缂撳瓨锛堝熀浜庡叾鍦?
鏂扮粍涓殑鎴愬憳鍏崇郴璁℃暟锛夎€屼笂鍗囥€?

杩欏悓鏍烽€傜敤浜庣紦瀛樺垎閰嶆帶鍒躲€傚皢浠诲姟绉诲姩鍒板叿鏈夋洿灏忕紦瀛樺垎鍖虹殑缁勪笉浼?
椹遍€愪换浣曠紦瀛樿銆傝繘绋嬪彲鑳戒細缁х画浠庢棫鍒嗗尯浣跨敤瀹冧滑銆?

纭欢浣跨敤 CLOSid锛圕lass of service ID锛夊拰 RMID锛圧esource monitoring ID锛?
鍒嗗埆鏍囪瘑鎺у埗缁勫拰鐩戞帶缁勩€傛瘡涓祫婧愮粍鏍规嵁杩欎簺缁勭殑绫诲瀷鏄犲皠鍒拌繖浜?ID銆?
CLOSid 鍜?RMID 鐨勬暟閲忓彈纭欢闄愬埗锛屽洜姝ゅ綋 CLOSID 鎴?RMID 浠讳竴鑰楀敖鏃讹紝
鍒涘缓 "CTRL_MON" 鐩綍鍙兘浼氬け璐ワ紱鑰屽綋 RMID 鑰楀敖鏃讹紝鍒涘缓 "MON" 缁?
鍙兘浼氬け璐ャ€?

### max_threshold_occupancy 鈥斺€?閫氱敤姒傚康


娉ㄦ剰锛屼竴涓?RMID 涓€鏃﹁閲婃斁锛屽彲鑳戒笉浼氱珛鍗冲彲鐢紝鍥犱负璇?RMID 浠嶆爣璁扮潃
鍏堝墠 RMID 鐢ㄦ埛鐨勭紦瀛樿銆傚洜姝わ紝姝ょ被 RMID 浼氳鏀惧叆 limbo锛堣竟缂橈級鍒楄〃锛?
骞跺湪缂撳瓨鍗犵敤涓嬮檷鏃堕噸鏂版鏌ャ€傚鏋滅郴缁熶腑瀛樺湪澶ч噺 limbo RMID 浣嗗皻鏈?
鍑嗗濂戒娇鐢紝鐢ㄦ埛鍦?mkdir 鏃跺彲鑳戒細鐪嬪埌 -EBUSY銆?

max_threshold_occupancy 鏄竴涓敤鎴峰彲閰嶇疆鐨勫€硷紝鐢ㄤ簬纭畾 RMID 鍙互琚?
閲婃斁鏃剁殑鍗犵敤姘村钩銆?

mon_llc_occupancy_limbo 璺熻釜鐐圭粰鍑轰簡涓嶅彲绔嬪嵆鍒嗛厤鐨勪竴閮ㄥ垎 RMID 鐨勭簿纭?
鍗犵敤锛堜互瀛楄妭涓哄崟浣嶏級銆備笉鑳戒緷璧栧畠姣忕閮戒骇鐢熻緭鍑猴紝鍙兘闇€瑕佸皾璇曞垱寤轰竴涓?
绌虹殑鐩戞帶缁勬潵寮哄埗鏇存柊銆傚彧鏈夊湪鍒涘缓鎺у埗缁勬垨鐩戞帶缁勫け璐ユ椂鎵嶄細浜х敓杈撳嚭銆?

### Schemata 鏂囦欢 鈥斺€?閫氱敤姒傚康


鏂囦欢涓殑姣忎竴琛屾弿杩颁竴涓祫婧愩€傝琛屼互璧勬簮鍚嶇О寮€澶达紝鍚庤窡瑕佸簲鐢ㄤ簬绯荤粺涓?
璇ヨ祫婧愭瘡涓疄渚嬬殑鐗瑰畾鍊笺€?

### 缂撳瓨 ID


鍦ㄥ綋鍓嶄竴浠ｇ郴缁熶笂锛屾瘡涓彃妲斤紙socket锛夋湁涓€涓?L3 缂撳瓨锛岃€?L2 缂撳瓨閫氬父
浠呯敱鏍镐笂鐨勮秴绾跨▼鍏变韩锛屼絾杩欏苟闈炴灦鏋勮姹傘€傛垜浠彲鑳藉湪涓€涓彃妲戒笂鏈夊涓?
鐙珛鐨?L3 缂撳瓨锛屽涓牳涔熷彲鑳藉叡浜竴涓?L2 缂撳瓨銆傚洜姝わ紝鎴戜滑涓嶄娇鐢?
"socket" 鎴?"core" 鏉ュ畾涔夊叡浜煇涓祫婧愮殑閫昏緫 CPU 闆嗗悎锛岃€屾槸浣跨敤
"Cache ID"銆傚湪缁欏畾鐨勭紦瀛樼骇鍒笂锛屽畠灏嗘槸鏁翠釜绯荤粺涓殑涓€涓敮涓€缂栧彿锛堜絾涓?
淇濊瘉鏄繛缁簭鍒楋紝鍙兘瀛樺湪绌洪殭锛夈€傝鏌ユ壘姣忎釜閫昏緫 CPU 鐨?ID锛岃鏌ョ湅
/sys/devices/system/cpu/cpu**/cache/index**/id

### 缂撳瓨浣嶆帺鐮侊紙CBM锛?


瀵逛簬缂撳瓨璧勬簮锛屾垜浠娇鐢ㄤ綅鎺╃爜鎻忚堪鍙敤浜庡垎閰嶇殑缂撳瓨閮ㄥ垎銆傛帺鐮佺殑鏈€澶у€?
鐢辨瘡涓?CPU 鍨嬪彿瀹氫箟锛堝苟涓斿浜庝笉鍚岀殑缂撳瓨绾у埆鍙兘涓嶅悓锛夈€傚畠鍙互閫氳繃
CPUID 鎵惧埌锛屼絾涔熺敱 resctrl 鏂囦欢绯荤粺鐨?"info" 鐩綍浠?
"info/{resource}/cbm_mask" 鎻愪緵銆傛煇浜?Intel 纭欢瑕佹眰杩欎簺鎺╃爜鐨勬墍鏈?
'1' 浣嶉兘鍦ㄤ竴涓繛缁殑鍧椾腑銆傚洜姝?0x3銆?x6 鍜?0xC 鏄寘鍚袱涓疆浣嶄綅鐨?
鍚堟硶 4 浣嶆帺鐮侊紝浣?0x5銆?x9 鍜?0xA 涓嶆槸銆傝鏌ョ湅
/sys/fs/resctrl/info/{resource}/sparse_masks 浠ョ‘璁ゆ槸鍚︽敮鎸侀潪杩炵画鐨?
1 鍊笺€傚湪涓€涓叿鏈?20 浣嶆帺鐮佺殑绯荤粺涓婏紝姣忎釜浣嶄唬琛ㄧ紦瀛樺閲忕殑 5%銆備綘鍙互鐢?
鎺╃爜 0x1f銆?x3e0銆?x7c00銆?xf8000 灏嗙紦瀛樺垎鎴愬洓涓浉绛夌殑閮ㄥ垎銆?

## 鍏充簬 Sub-NUMA Cluster 妯″紡鐨勬敞鎰忎簨椤?


褰撳惎鐢?SNC 妯″紡鏃讹紝Linux 鍙兘姣斿湪甯歌 NUMA 鑺傜偣涔嬮棿鏇寸Н鏋佸湴鍦?Sub-NUMA
鑺傜偣涔嬮棿骞宠　浠诲姟锛屽洜涓?Sub-NUMA 鑺傜偣涓婄殑 CPU 鍏变韩鍚屼竴涓?L3 缂撳瓨锛屽苟涓?
绯荤粺鎶ュ憡鐨?Sub-NUMA 鑺傜偣涔嬮棿鐨?NUMA 璺濈鍙兘浣庝簬甯歌 NUMA 鑺傜偣浣跨敤鐨?
鍊笺€?

姣忎釜 "mon_L3_XX" 鐩綍涓殑椤剁骇鐩戞帶鏂囦欢鎻愪緵鍏变韩涓€涓?L3 缂撳瓨瀹炰緥鐨勬墍鏈?
SNC 鑺傜偣涓婄殑鏁版嵁鎬诲拰銆傚皢浠诲姟缁戝畾鍒扮壒瀹?Sub-NUMA 鑺傜偣 CPU 鐨勭敤鎴峰彲浠?
璇诲彇 "mon_sub_L3_YY" 鐩綍涓殑 "llc_occupancy"銆?mbm_total_bytes" 鍜?
"mbm_local_bytes" 鏉ヨ幏鍙栬妭鐐规湰鍦版暟鎹€?

鍐呭瓨甯﹀鍒嗛厤浠嶇劧鍦?L3 缂撳瓨绾у埆鎵ц銆傚嵆鑺傛祦鎺у埗搴旂敤浜庢墍鏈?SNC 鑺傜偣銆?

L3 缂撳瓨鍒嗛厤浣嶅浘涔熷簲鐢ㄤ簬鎵€鏈?SNC 鑺傜偣銆備絾璇锋敞鎰忥紝姣忎釜浣嶆墍浠ｈ〃鐨?L3
缂撳瓨閲忚闄や互姣忎釜 L3 缂撳瓨鐨?SNC 鑺傜偣鏁般€備緥濡傦紝鍦ㄤ竴涓叿鏈?100MB 缂撳瓨銆?
10 浣嶅垎閰嶆帺鐮佺殑绯荤粺涓婏紝姣忎釜浣嶉€氬父浠ｈ〃 10MB銆傚惎鐢?SNC 妯″紡涓旀瘡涓?L3
缂撳瓨鏈変袱涓?SNC 鑺傜偣鏃讹紝姣忎釜浣嶄粎浠ｈ〃 5MB銆?

## 鍐呭瓨甯﹀鐨勫垎閰嶄笌鐩戞帶


瀵逛簬鍐呭瓨甯﹀璧勬簮锛岄粯璁ゆ儏鍐典笅鐢ㄦ埛閫氳繃鎸囩ず鎬诲唴瀛樺甫瀹界殑鐧惧垎姣旀潵鎺у埗
璇ヨ祫婧愩€?

姣忎釜 CPU 鍨嬪彿鐨勬渶灏忓甫瀹界櫨鍒嗘瘮鍊兼槸棰勫畾涔夌殑锛屽彲浠ラ€氳繃 "info/MB/min_bandwidth"
鏌ユ壘銆傛墍鍒嗛厤鐨勫甫瀹界矑搴︿篃鍙栧喅浜?CPU 鍨嬪彿锛屽彲浠ラ€氳繃 "info/MB/bandwidth_gran"
鏌ユ壘銆傚彲鐢ㄧ殑甯﹀鎺у埗姝ラ暱涓猴細min_bw + N * bw_gran銆備腑闂村€间細琚垗鍏ュ埌
纭欢涓婂彲鐢ㄧ殑涓嬩竴涓帶鍒舵闀裤€?

鍦ㄦ煇浜?Intel SKU 涓婏紝甯﹀鑺傛祦鏄竴绉嶆牳鐗瑰畾鐨勬満鍒躲€傚湪涓や釜鍏变韩涓€涓牳鐨?
绾跨▼涓婁娇鐢ㄩ珮甯﹀鍜屼綆甯﹀璁剧疆锛屽彲鑳藉鑷翠袱涓嚎绋嬮兘琚妭娴佷负浣跨敤浣庡甫瀹?
锛堝弬瑙?"thread_throttle_mode"锛夈€?

鍐呭瓨甯﹀鍒嗛厤锛圡BA锛夊彲鑳芥槸鏍哥壒瀹氱殑鏈哄埗锛岃€屽唴瀛樺甫瀹界洃鎺э紙MBM锛夋槸鍦?
灏佽绾у埆瀹屾垚鐨勶紝杩欎竴浜嬪疄鍙兘瀵艰嚧鐢ㄦ埛鍦ㄥ皾璇曢€氳繃 MBA 搴旂敤鎺у埗鐒跺悗鐩戞帶
甯﹀浠ユ煡鐪嬫帶鍒舵槸鍚︽湁鏁堟椂鎰熷埌鍥版儜銆備互涓嬫槸姝ょ被鍦烘櫙锛?

1. 褰撶敤鎴锋彁楂樼櫨鍒嗘瘮鍊兼椂锛屽彲鑳?*涓嶄細**鐪嬪埌瀹為檯甯﹀澧炲姞锛?

褰撹仛鍚堢殑 L2 澶栭儴甯﹀澶т簬 L3 澶栭儴甯﹀鏃朵細鍙戠敓杩欑鎯呭喌銆傝€冭檻涓€涓?SKL
SKU锛屼竴涓皝瑁呬笂鏈?24 涓牳锛孡2 澶栭儴甯﹀涓?10GBps锛堝洜姝よ仛鍚?L2 澶栭儴甯﹀涓?
240GBps锛夛紝L3 澶栭儴甯﹀涓?100GBps銆傜幇鍦ㄤ竴涓?'20 涓嚎绋嬨€佸叿鏈?50% 甯﹀銆?
姣忎釜娑堣€?5GBps' 鐨勫伐浣滆礋杞芥秷鑰椾簡 100GBps 鐨勬渶澶?L3 甯﹀锛屽敖绠℃寚瀹氱殑
鐧惧垎姣斿€间粎涓?50% << 100%銆傚洜姝ゅ鍔犲甫瀹界櫨鍒嗘瘮涓嶄細浜х敓鏇村甯﹀銆傝繖鏄?
鍥犱负灏界 L2 澶栭儴甯﹀浠嶆湁瀹归噺锛屼絾 L3 澶栭儴甯﹀宸插畬鍏ㄧ敤灏姐€傚彟璇锋敞鎰忥紝杩?
灏嗗彇鍐充簬鍩哄噯娴嬭瘯杩愯鐨勬牳鏁般€?

2. 鐩稿悓鐨勫甫瀹界櫨鍒嗘瘮鍙兘鎰忓懗鐫€涓嶅悓鐨勫疄闄呭甫瀹斤紝鍏蜂綋鍙栧喅浜庣嚎绋嬫暟锛?

瀵逛簬 #1 涓浉鍚岀殑 SKU锛?鍗曠嚎绋嬨€?0% 甯﹀' 鍜?'4 绾跨▼銆?0% 甯﹀' 鍙互
鍒嗗埆娑堣€楅珮杈?10GBps 鍜?40GBps锛屽敖绠″畠浠叿鏈夌浉鍚岀殑 10% 甯﹀鐧惧垎姣斻€?
杩欎粎浠呮槸鍥犱负闅忕潃绾跨▼寮€濮嬪湪 rdtgroup 涓娇鐢ㄦ洿澶氭牳锛屽疄闄呭甫瀹藉彲鑳戒細澧炲姞鎴?
鍙樺寲锛屽嵆浣跨敤鎴锋寚瀹氱殑甯﹀鐧惧垎姣旂浉鍚屻€?

涓轰簡缂撹В杩欑鎯呭喌骞朵娇鎺ュ彛鏇村弸濂斤紝resctrl 澧炲姞浜嗕互 MiBps 鎸囧畾甯﹀鐨勬敮鎸併€?
搴曞眰鍐呮牳灏嗕娇鐢ㄨ蒋浠跺弽棣堟満鍒舵垨 "Software Controller锛坢ba_sc锛?锛屽畠浣跨敤
MBM 璁℃暟鍣ㄨ鍙栧疄闄呭甫瀹?
```

	"actual bandwidth < user specified bandwidth".

```
榛樿鎯呭喌涓嬶紝schemata 閲囩敤甯﹀鐧惧垎姣斿€硷紝鑰岀敤鎴峰彲浠ヤ娇鐢ㄦ寕杞介€夐」 'mba_MBps'
鍒囨崲鍒?"MBA software controller" 妯″紡銆俿chemata 鏍煎紡鍦ㄤ笅鏂囧悇鑺備腑鎸囧畾銆?
### L3 schemata 鏂囦欢缁嗚妭锛堜唬鐮佷笌鏁版嵁浼樺厛绾у垝鍒嗗凡绂佺敤锛?


```

	L3:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...

```
### L3 schemata 鏂囦欢缁嗚妭锛堥€氳繃鎸傝浇閫夐」涓?resctrl 鍚敤 CDP锛?


褰撳惎鐢?CDP 鏃讹紝L3 鎺у埗琚媶鍒嗕负涓や釜鐙珛鐨勮祫婧?
```

	L3DATA:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...
	L3CODE:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...

```
### L2 schemata 鏂囦欢缁嗚妭


L2 閫氳繃 'cdpl2' 鎸傝浇閫夐」鏀寔 CDP銆傚叾 schemata
```

	L2:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...

```
鎴?

	L2DATA:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...
	L2CODE:<cache_id0>=<cbm>;<cache_id1>=<cbm>;...


### 鍐呭瓨甯﹀鍒嗛厤锛堥粯璁ゆā寮忥級


鍐呭瓨 b/w 鍩熸槸 L3 缂撳瓨銆?
```

	MB:<cache_id0>=bandwidth0;<cache_id1>=bandwidth1;...

```
### 浠?MiBps 鎸囧畾鐨勫唴瀛樺甫瀹藉垎閰?


鍐呭瓨甯﹀鍩熸槸 L3 缂撳瓨銆?
```

	MB:<cache_id0>=bw_MiBps0;<cache_id1>=bw_MiBps1;...

```
### 鎱㈤€熷唴瀛樺甫瀹藉垎閰嶏紙SMBA锛?


AMD 纭欢鏀寔鎱㈤€熷唴瀛樺甫瀹藉垎閰嶏紙SMBA锛夈€?
CXL.memory 鏄敮涓€鍙楁敮鎸佺殑鈥滄參閫熲€濆唴瀛樿澶囥€傚€熷姪 SMBA 鐨勬敮鎸侊紝纭欢
鍦ㄦ參閫熷唴瀛樿澶囦笂鍚敤甯﹀鍒嗛厤銆傚鏋滅郴缁熶腑鏈夊涓绫昏澶囷紝鑺傛祦閫昏緫浼?
灏嗘墍鏈夋參閫熸潵婧愬綊涓轰竴缁勶紝骞跺瀹冧滑鏁翠綋鏂藉姞闄愬埗銆?

SMBA锛堥厤鍚?CXL.memory锛夌殑瀛樺湪涓庢槸鍚﹀瓨鍦ㄦ參閫熷唴瀛樿澶囨棤鍏炽€傚鏋滅郴缁熶笂
娌℃湁姝ょ被璁惧锛屽垯閰嶇疆 SMBA 涓嶄細瀵圭郴缁熸€ц兘浜х敓褰卞搷銆?

鎱㈤€熷唴瀛樼殑甯﹀鍩熸槸 L3 缂撳瓨銆傚叾 schemata 鏂囦欢鏍煎紡濡備笅锛?
```

	SMBA:<cache_id0>=bandwidth0;<cache_id1>=bandwidth1;...

```
### 璇?鍐?schemata 鏂囦欢


璇诲彇 schemata 鏂囦欢浼氭樉绀烘墍鏈夊煙涓婃墍鏈夎祫婧愮殑鐘舵€併€傚啓鍏ユ椂鍙渶鎸囧畾
浣犲笇鏈涙洿鏀圭殑閭ｄ簺鍊笺€備緥濡傦細
```

  # cat schemata
  L3DATA:0=fffff;1=fffff;2=fffff;3=fffff
  L3CODE:0=fffff;1=fffff;2=fffff;3=fffff
  # echo "L3DATA:2=3c0;" > schemata
  # cat schemata
  L3DATA:0=fffff;1=fffff;2=3c0;3=fffff
  L3CODE:0=fffff;1=fffff;2=fffff;3=fffff

```
### 璇?鍐?schemata 鏂囦欢锛堝湪 AMD 绯荤粺涓婏級


璇诲彇 schemata 鏂囦欢浼氭樉绀烘墍鏈夊煙涓婄殑褰撳墠甯﹀闄愬埗銆傛墍鍒嗛厤鐨勮祫婧愭槸
鍏垎涔嬩竴 GB/s 鐨勬暣鏁板€嶃€傚啓鍏ユ枃浠舵椂锛岄渶瑕佹寚瀹氳閰嶇疆甯﹀闄愬埗鐨勭紦瀛?id銆?

渚嬪锛岃鍦ㄧ涓€涓紦瀛?id 涓婂垎閰?2GB/s 鐨勯檺鍒讹細

```

  # cat schemata
    MB:0=2048;1=2048;2=2048;3=2048
    L3:0=ffff;1=ffff;2=ffff;3=ffff

  # echo "MB:1=16" > schemata
  # cat schemata
    MB:0=2048;1=  16;2=2048;3=2048
    L3:0=ffff;1=ffff;2=ffff;3=ffff

```
### 璇?鍐?schemata 鏂囦欢锛堝湪 AMD 绯荤粺涓婏紝甯?SMBA 鐗规€э級


schemata 鏂囦欢鐨勮鍐欎笌涓婁竴鑺備腑涓嶅甫 SMBA 鏃剁浉鍚屻€?

渚嬪锛岃鍦ㄧ涓€涓紦瀛?id 涓婂垎閰?8GB/s 鐨勯檺鍒讹細

```

  # cat schemata
    SMBA:0=2048;1=2048;2=2048;3=2048
      MB:0=2048;1=2048;2=2048;3=2048
      L3:0=ffff;1=ffff;2=ffff;3=ffff

  # echo "SMBA:1=64" > schemata
  # cat schemata
    SMBA:0=2048;1=  64;2=2048;3=2048
      MB:0=2048;1=2048;2=2048;3=2048
      L3:0=ffff;1=ffff;2=ffff;3=ffff

```
## 缂撳瓨浼攣瀹?


CAT 浣跨敤鎴疯兘澶熸寚瀹氬簲鐢ㄧ▼搴忓彲浠ュ～鍏呯殑缂撳瓨绌洪棿澶у皬銆傜紦瀛樹吉閿佸畾寤虹珛鍦ㄤ竴涓?
浜嬪疄涔嬩笂锛欳PU 鍦ㄧ紦瀛樺懡涓椂浠嶇劧鍙互璇诲啓鍏跺綋鍓嶅垎閰嶅尯鍩熶箣澶栥€侀鍏堝垎閰嶇殑鏁版嵁銆?
閫氳繃缂撳瓨浼攣瀹氾紝鏁版嵁鍙互琚鍔犺浇鍒扮紦瀛樹腑涓€涓繚鐣欑殑銆佷换浣曞簲鐢ㄧ▼搴忛兘鏃犳硶
濉厖鐨勯儴鍒嗭紝骞朵粠閭ｆ椂璧峰彧鏈嶅姟浜庣紦瀛樺懡涓€傜紦瀛樹吉閿佸畾鐨勫唴瀛樿鎻愪緵缁欑敤鎴风┖闂达紝
搴旂敤绋嬪簭鍙互灏嗗叾鏄犲皠鍒拌嚜宸辩殑铏氭嫙鍦板潃绌洪棿锛屼粠鑰屾嫢鏈変竴鍧楀钩鍧囪鍙栧欢杩熼檷浣庣殑
鍐呭瓨鍖哄煙銆?

缂撳瓨浼攣瀹氬尯鍩熺殑鍒涘缓鐢辩敤鎴峰彂鍑虹殑涓€鍒欒姹傝Е鍙戯紝璇ヨ姹傞檮甯﹀緟浼攣瀹氬尯鍩熺殑
schemata銆傜紦瀛樹吉閿佸畾鍖哄煙鎸夊涓嬫柟寮忓垱寤猴細

- 鍒涘缓涓€涓?CAT 鍒嗛厤 CLOSNEW锛屽叾 CBM 鍖归厤灏嗗寘鍚吉閿佸畾鍐呭瓨鐨勭紦瀛樺尯鍩熺殑
  鐢ㄦ埛 schemata銆傝鍖哄煙涓嶅緱涓庣郴缁熶笂浠讳綍褰撳墠鐨?CAT 鍒嗛厤/CLOS 閲嶅彔锛屽苟涓斿湪
  浼攣瀹氬尯鍩熷瓨鍦ㄦ湡闂翠笉鍏佽灏嗘潵涓庤缂撳瓨鍖哄煙鍙戠敓閲嶅彔銆?
- 鍒涘缓涓€鍧椾笌缂撳瓨鍖哄煙澶у皬鐩稿悓鐨勮繛缁唴瀛樺尯鍩熴€?
- 鍒锋柊缂撳瓨锛岀鐢ㄧ‖浠堕鍙栧櫒锛岀鐢ㄦ姠鍗犮€?
- 灏?CLOSNEW 璁句负娲诲姩 CLOS锛屽苟瑙︾宸插垎閰嶇殑鍐呭瓨浠ュ皢鍏跺姞杞藉埌缂撳瓨涓€?
- 灏嗗厛鍓嶇殑 CLOS 璁句负娲诲姩 CLOS銆?
- 姝ゆ椂鍙互閲婃斁 closid CLOSNEW 鈥斺€?鍙鍏?CBM 涓嶅嚭鐜板湪浠讳綍 CAT 鍒嗛厤涓紝
  缂撳瓨浼攣瀹氬尯鍩熷氨鍙楀埌淇濇姢銆傚敖绠＄紦瀛樹吉閿佸畾鍖哄煙浠庢鏃惰捣涓嶄細鍑虹幇鍦ㄤ换浣?
  CLOS 鐨勪换浣?CBM 涓紝浣嗚繍琛屼簬浠讳綍 CLOS 涓嬬殑搴旂敤绋嬪簭閮藉皢鑳藉璁块棶浼攣瀹?
  鍖哄煙涓殑鍐呭瓨锛屽洜涓鸿鍖哄煙浼氱户缁湇鍔′簬缂撳瓨鍛戒腑銆?
- 鍔犺浇鍒扮紦瀛樹腑鐨勮繛缁唴瀛樺尯鍩熶綔涓哄瓧绗﹁澶囨毚闇茬粰鐢ㄦ埛绌洪棿銆?

缂撳瓨浼攣瀹氶€氳繃浠旂粏閰嶇疆 CAT 鐗规€у苟鎺у埗搴旂敤绋嬪簭琛屼负锛屾潵鎻愰珮鏁版嵁淇濈暀鍦?
缂撳瓨涓殑姒傜巼銆備絾鏃犳硶淇濊瘉鏁版嵁涓€瀹氳鏀惧叆缂撳瓨銆傝濡?INVD銆乄BINVD銆丆LFLUSH
绛夋寚浠や粛鍙兘灏嗏€滈攣瀹氣€濈殑鏁版嵁浠庣紦瀛樹腑椹遍€愩€傜數婧愮鐞?C-states 鍙兘浼氭敹缂╂垨
鍏抽棴缂撳瓨銆傚湪鍒涘缓浼攣瀹氬尯鍩熸椂锛屾洿娣辩殑 C-states 浼氳鑷姩闄愬埗銆?

浣跨敤浼攣瀹氬尯鍩熺殑搴旂敤绋嬪簭蹇呴』浠ヤ翰鍜屾€ц繍琛屽湪涓庝吉閿佸畾鍖哄煙鎵€鍦ㄧ紦瀛樺叧鑱旂殑
鏍革紙鎴栨牳鐨勫瓙闆嗭級涓娿€備唬鐮佷腑鐨勪竴椤瑰仴鍏ㄦ€ф鏌ュ皢涓嶅厑璁稿簲鐢ㄧ▼搴忔槧灏勪吉閿佸畾
鍐呭瓨锛岄櫎闈炲畠浠ヤ翰鍜屾€ц繍琛屽湪涓庝吉閿佸畾鍖哄煙鎵€鍦ㄧ紦瀛樺叧鑱旂殑鏍镐笂銆傝鍋ュ叏鎬ф鏌?
浠呭湪鍒濆鐨?mmap() 澶勭悊鏈熼棿杩涜锛屼箣鍚庢病鏈夊己鍒讹紝搴旂敤绋嬪簭鑷韩闇€瑕佺‘淇濅繚鎸?
瀵规纭牳鐨勪翰鍜屾€с€?

浼攣瀹氬垎涓や釜闃舵瀹屾垚锛?

1) 鍦ㄧ涓€闃舵锛岀郴缁熺鐞嗗憳鍒嗛厤涓€閮ㄥ垎搴斾笓鐢ㄤ簬浼攣瀹氱殑缂撳瓨銆傛鏃朵細鍒嗛厤
   绛夐噺鐨勫唴瀛橈紝鍔犺浇鍒板凡鍒嗛厤鐨勭紦瀛橀儴鍒嗭紝骞朵綔涓哄瓧绗﹁澶囨毚闇层€?
2) 鍦ㄧ浜岄樁娈碉紝鐢ㄦ埛绌洪棿搴旂敤绋嬪簭灏嗕吉閿佸畾鍐呭瓨鏄犲皠锛坢map()锛夊埌鍏跺湴鍧€绌洪棿銆?

### 缂撳瓨浼攣瀹氭帴鍙?


浣跨敤 resctrl 鎺ュ彛鍒涘缓浼攣瀹氬尯鍩熺殑鏂瑰紡濡備笅锛?

1) 鍦?/sys/fs/resctrl 涓垱寤轰竴涓柊鐩綍鏉ュ垱寤烘柊鐨勮祫婧愮粍銆?
2) 閫氳繃鍚?"mode" 鏂囦欢鍐欏叆 "pseudo-locksetup"锛屽皢鏂拌祫婧愮粍鐨勬ā寮忔敼涓?
   "pseudo-locksetup"銆?
3) 灏嗕吉閿佸畾鍖哄煙鐨?schemata 鍐欏叆 "schemata" 鏂囦欢銆傛牴鎹?"bit_usage" 鏂囦欢锛?
   schemata 涓殑鎵€鏈変綅閮藉簲涓?"unused"銆?

浼攣瀹氬尯鍩熷垱寤烘垚鍔熷悗锛?mode" 鏂囦欢灏嗗寘鍚?"pseudo-locked"锛屽苟涓斾竴涓笌璧勬簮缁?
鍚屽悕鐨勬柊瀛楃璁惧灏嗗瓨鍦ㄤ簬 /dev/pseudo_lock 涓€傜敤鎴风┖闂村彲浠ュ杩欎釜瀛楃璁惧
杩涜 mmap()锛屼互鑾峰彇瀵逛吉閿佸畾鍐呭瓨鍖哄煙鐨勮闂€?

缂撳瓨浼攣瀹氬尯鍩熺殑鍒涘缓涓庝娇鐢ㄧず渚嬭涓嬫枃銆?

### 缂撳瓨浼攣瀹氳皟璇曟帴鍙?


浼攣瀹氳皟璇曟帴鍙ｉ粯璁ゅ惎鐢紙濡傛灉鍚敤浜?CONFIG_DEBUG_FS锛夛紝鍙互鍦?
/sys/kernel/debug/resctrl 涓壘鍒般€?

鍐呮牳娌℃湁鏄惧紡鐨勬柟娉曟祴璇曟煇涓粰瀹氱殑鍐呭瓨浣嶇疆鏄惁瀛樺湪浜庣紦瀛樹腑銆備吉閿佸畾璋冭瘯鎺ュ彛
浣跨敤璺熻釜鍩虹璁炬柦鎻愪緵涓ょ娴嬮噺浼攣瀹氬尯鍩熺紦瀛橀┗鐣欏害鐨勬柟寮忥細

1) 浣跨敤 pseudo_lock_mem_latency 璺熻釜鐐圭殑鍐呭瓨璁块棶寤惰繜銆傝繖浜涙祴閲忕殑鏁版嵁鏈€濂?
   浣跨敤 hist 瑙﹀彂鍣ㄥ彲瑙嗗寲锛堣涓嬩緥锛夈€傚湪姝ゆ祴璇曚腑锛屼吉閿佸畾鍖哄煙浠?32 瀛楄妭鐨?
   姝ラ暱琚亶鍘嗭紝鍚屾椂纭欢棰勫彇鍣ㄥ拰鎶㈠崰琚鐢ㄣ€傝繖涔熸彁渚涗簡缂撳瓨鍛戒腑涓庢湭鍛戒腑鐨?
   鏇夸唬鍙鍖栥€?
2) 濡傛灉鍙敤锛屼娇鐢ㄧ壒瀹氫簬鍨嬪彿鐨勭簿纭鏁板櫒娴嬮噺缂撳瓨鍛戒腑涓庢湭鍛戒腑銆傛牴鎹郴缁熶笂
   缂撳瓨鐨勭骇鍒紝pseudo_lock_l2 鍜?pseudo_lock_l3 璺熻釜鐐瑰彲鐢ㄣ€?

褰撳垱寤轰吉閿佸畾鍖哄煙鏃讹紝浼氬湪 debugfs 涓负鍏跺垱寤轰竴涓柊鐩綍锛?
/sys/kernel/debug/resctrl/<newdir>銆傝鐩綍涓瓨鍦ㄤ竴涓彧鍐欐枃浠?
pseudo_lock_measure銆備吉閿佸畾鍖哄煙鐨勬祴閲忓彇鍐充簬鍐欏叆姝?debugfs 鏂囦欢鐨勬暟瀛楋細

1:
     鍚?pseudo_lock_measure 鏂囦欢鍐欏叆 "1" 灏嗚Е鍙?pseudo_lock_mem_latency
     璺熻釜鐐规崟鑾风殑寤惰繜娴嬮噺銆傝涓嬩緥銆?
2:
     鍚?pseudo_lock_measure 鏂囦欢鍐欏叆 "2" 灏嗚Е鍙?L2 缂撳瓨椹荤暀搴︼紙缂撳瓨鍛戒腑涓?
     鏈懡涓級娴嬮噺锛岀敱 pseudo_lock_l2 璺熻釜鐐规崟鑾枫€傝涓嬩緥銆?
3:
     鍚?pseudo_lock_measure 鏂囦欢鍐欏叆 "3" 灏嗚Е鍙?L3 缂撳瓨椹荤暀搴︼紙缂撳瓨鍛戒腑涓?
     鏈懡涓級娴嬮噺锛岀敱 pseudo_lock_l3 璺熻釜鐐规崟鑾枫€?

鎵€鏈夋祴閲忛兘閫氳繃璺熻釜鍩虹璁炬柦璁板綍銆傝繖瑕佹眰鍦ㄨЕ鍙戞祴閲忎箣鍓嶅惎鐢ㄧ浉鍏崇殑璺熻釜鐐广€?

#### 寤惰繜璋冭瘯鎺ュ彛绀轰緥


鍦ㄦ绀轰緥涓紝鍒涘缓浜嗕竴涓悕涓?"newlock" 鐨勪吉閿佸畾鍖哄煙銆備笅闈㈡垜浠睍绀哄浣曟祴閲?
浠庤鍖哄煙璇诲彇鐨勫欢杩燂紙浠ュ懆鏈熶负鍗曚綅锛夛紝骞朵娇鐢ㄥ湪鍚敤 CONFIG_HIST_TRIGGERS 鏃?
鍙敤鐨勭洿鏂瑰浘灏嗗叾鍙鍖?
```

  # :> /sys/kernel/tracing/trace
  # echo 'hist:keys=latency' > /sys/kernel/tracing/events/resctrl/pseudo_lock_mem_latency/trigger
  # echo 1 > /sys/kernel/tracing/events/resctrl/pseudo_lock_mem_latency/enable
  # echo 1 > /sys/kernel/debug/resctrl/newlock/pseudo_lock_measure
  # echo 0 > /sys/kernel/tracing/events/resctrl/pseudo_lock_mem_latency/enable
  # cat /sys/kernel/tracing/events/resctrl/pseudo_lock_mem_latency/hist

  # event histogram
  #
  # trigger info: hist:keys=latency:vals=hitcount:sort=hitcount:size=2048 [active]
  #

  { latency:        456 } hitcount:          1
  { latency:         50 } hitcount:         83
  { latency:         36 } hitcount:         96
  { latency:         44 } hitcount:        174
  { latency:         48 } hitcount:        195
  { latency:         46 } hitcount:        262
  { latency:         42 } hitcount:        693
  { latency:         40 } hitcount:       3204
  { latency:         38 } hitcount:       3484

  Totals:
      Hits: 8192
      Entries: 9
    Dropped: 0

```
#### 缂撳瓨鍛戒腑/鏈懡涓皟璇曠ず渚?


鍦ㄦ绀轰緥涓紝鍦ㄤ竴涓钩鍙扮殑 L2 缂撳瓨涓婂垱寤轰簡涓€涓悕涓?"newlock" 鐨勪吉閿佸畾鍖哄煙銆?
涓嬮潰鎴戜滑灞曠ず濡備綍浣跨敤骞冲彴鐨勭簿纭鏁板櫒鑾峰彇缂撳瓨鍛戒腑涓庢湭鍛戒腑鐨勮鎯呫€?
```

  # :> /sys/kernel/tracing/trace
  # echo 1 > /sys/kernel/tracing/events/resctrl/pseudo_lock_l2/enable
  # echo 2 > /sys/kernel/debug/resctrl/newlock/pseudo_lock_measure
  # echo 0 > /sys/kernel/tracing/events/resctrl/pseudo_lock_l2/enable
  # cat /sys/kernel/tracing/trace

  # tracer: nop
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            ||| /     delay
  #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
  #              | |       |   ||||       |         |
  pseudo_lock_mea-1672  [002] ....  3132.860500: pseudo_lock_l2: hits=4097 miss=0


```
#### RDT 鍒嗛厤浣跨敤绀轰緥


1) 绀轰緥 1

鍦ㄤ竴鍙板弻鎻掓Ы鏈哄櫒锛堟瘡鎻掓Ы涓€涓?L3 缂撳瓨锛変笂锛岀紦瀛樹綅鎺╃爜浠呮湁 4 浣嶏紝鏈€灏?
b/w 涓?10%锛屽唴瀛樺甫瀹界矑搴︿负 10%銆?
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir p0 p1
  # echo "L3:0=3;1=c\nMB:0=50;1=50" > /sys/fs/resctrl/p0/schemata
  # echo "L3:0=3;1=3\nMB:0=50;1=50" > /sys/fs/resctrl/p1/schemata

```
榛樿璧勬簮缁勬湭琚慨鏀癸紝鍥犳鎴戜滑鍙互璁块棶鎵€鏈夌紦瀛樼殑鎵€鏈夐儴鍒嗭紙鍏?schemata 鏂囦欢
璇讳负 "L3:0=f;1=f"锛夈€?

澶勪簬 "p0" 缁勬帶鍒朵笅鐨勪换鍔″彧鑳戒粠缂撳瓨 ID 0 鐨勨€滆緝浣庘€?50% 鍜岀紦瀛?ID 1 鐨?
鈥滆緝楂樷€?50% 涓垎閰嶃€傚浜?"p1" 缁勪腑鐨勪换鍔″湪涓や釜鎻掓Ы涓婇兘浣跨敤缂撳瓨鐨勨€滆緝浣庘€?
50%銆?

绫讳技鍦帮紝澶勪簬 "p0" 缁勬帶鍒朵笅鐨勪换鍔″湪 socket0 涓婃渶澶氬彲浣跨敤 50% 鐨勫唴瀛?b/w锛?
鍦?socket 1 涓婃渶澶?50%銆傚浜?"p1" 缁勪腑鐨勪换鍔″湪涓や釜鎻掓Ы涓婁篃鏈€澶氬彲浣跨敤 50%
鐨勫唴瀛?b/w銆傛敞鎰忥紝涓庣紦瀛樻帺鐮佷笉鍚岋紝鍐呭瓨 b/w 鏃犳硶鎸囧畾杩欎簺鍒嗛厤鏄惁鍙互閲嶅彔銆?
鍒嗛厤鎸囧畾鐨勬槸璇ョ粍鍙兘鑳藉浣跨敤鐨勬渶澶?b/w锛岀郴缁熺鐞嗗憳鍙互鐩稿簲鍦伴厤缃?b/w銆?

濡傛灉 resctrl 浣跨敤杞欢鎺у埗鍣紙mba_sc锛夛紝鍒欑敤鎴峰彲浠ヨ緭鍏ヤ互 MB 涓哄崟浣嶇殑鏈€澶?
b/w锛岃€屼笉鏄櫨鍒嗘瘮鍊笺€?
```

  # echo "L3:0=3;1=c\nMB:0=1024;1=500" > /sys/fs/resctrl/p0/schemata
  # echo "L3:0=3;1=3\nMB:0=1024;1=500" > /sys/fs/resctrl/p1/schemata

```
鍦ㄤ笂杩扮ず渚嬩腑锛宻ocket 0 涓?"p1" 鍜?"p0" 涓殑浠诲姟灏嗕娇鐢?1024MB 鐨勬渶澶?b/w锛?
鑰屽湪 socket 1 涓婂畠浠皢浣跨敤 500MB銆?

2) 绀轰緥 2

鍚屾牱鏄弻鎻掓Ы锛屼絾杩欐浣跨敤鏇村疄闄呯殑 20 浣嶆帺鐮併€?

鍦ㄤ竴鍙板弻鎻掓Ы鍙屾牳鏈哄櫒涓婏紝socket 0 涓婃湁涓や釜瀹炴椂浠诲姟锛氳繍琛屽湪澶勭悊鍣?0 涓婄殑
pid=1234 鍜岃繍琛屽湪澶勭悊鍣?1 涓婄殑 pid=5678銆備负浜嗛伩鍏嶅惖闂圭殑閭诲眳锛岃繖涓や釜瀹炴椂
浠诲姟鍚勮嚜鐙崰鍗犵敤 socket 0 涓?L3 缂撳瓨鐨勫洓鍒嗕箣涓€銆?
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl

```
棣栧厛鎴戜滑閲嶇疆榛樿缁勭殑 schemata锛屼娇寰?socket 0 涓?L3 缂撳瓨鐨勨€滆緝楂樷€?50% 鍜?
50% 鐨勫唴瀛?b/w 鏃犳硶琚娇鐢?
```

  # echo "L3:0=3ff;1=fffff\nMB:0=50;1=100" > schemata

```
鎺ヤ笅鏉ユ垜浠负绗竴涓疄鏃朵换鍔″垱寤轰竴涓祫婧愮粍锛屽苟璁╁畠璁块棶 socket 0 涓婄紦瀛樼殑
鈥滈《閮ㄢ€?25%銆?
```

  # mkdir p0
  # echo "L3:0=f8000;1=fffff" > p0/schemata

```
鏈€鍚庢垜浠皢绗竴涓疄鏃朵换鍔＄Щ鍏ヨ繖涓祫婧愮粍銆傛垜浠繕浣跨敤 taskset(1) 纭繚璇ヤ换鍔?
濮嬬粓杩愯鍦?socket 0 涓婁笓鐢ㄧ殑 CPU 涓娿€傚ぇ澶氭暟璧勬簮缁勭殑浣跨敤涔熶細闄愬埗浠诲姟杩愯鍦?
鍝簺澶勭悊鍣ㄤ笂銆?
```

  # echo 1234 > p0/tasks
  # taskset -cp 1 1234

```
```

  # mkdir p1
  # echo "L3:0=7c00;1=fffff" > p1/schemata
  # echo 5678 > p1/tasks
  # taskset -cp 2 5678

```
瀵逛簬鍚屾牱鐨勫弻鎻掓Ы绯荤粺锛屽甫鏈夊唴瀛?b/w 璧勬簮鍜?CAT L3锛宻chemata 灏嗗涓嬫墍绀?
锛堝亣璁?min_bandwidth 涓?10锛宐andwidth_gran 涓?10锛夛細

瀵逛簬绗竴涓疄鏃朵换鍔★紝杩欏皢璇锋眰 socket 0 涓?20% 鐨勫唴瀛?b/w銆?
```

  # echo -e "L3:0=f8000;1=fffff\nMB:0=20;1=100" > p0/schemata

```
瀵逛簬绗簩涓疄鏃朵换鍔★紝杩欏皢璇锋眰 socket 0 涓婂彟澶?20% 鐨勫唴瀛?b/w銆?
```

  # echo -e "L3:0=f8000;1=fffff\nMB:0=20;1=100" > p0/schemata

```
3) 绀轰緥 3

涓€涓崟鎻掓Ы绯荤粺锛屽疄鏃朵换鍔¤繍琛屽湪鏍?4-7 涓婏紝闈炲疄鏃跺伐浣滆礋杞藉垎閰嶅埌鏍?0-3銆?
瀹炴椂浠诲姟鍏变韩浠ｇ爜鍜屾暟鎹紝鍥犳涓嶉渶瑕侀€愪换鍔＄殑鍏宠仈锛涘苟涓旂敱浜庝笌鍐呮牳鐨勪氦浜掞紝
甯屾湜杩欎簺鏍镐笂鐨勫唴鏍镐笌浠诲姟鍏变韩 L3銆?
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl

```
棣栧厛鎴戜滑閲嶇疆榛樿缁勭殑 schemata锛屼娇寰?socket 0 涓?L3 缂撳瓨鐨勨€滆緝楂樷€?50%锛屼互鍙?
socket 0 涓?50% 鐨勫唴瀛樺甫瀹芥棤娉曡浣跨敤
```

  # echo "L3:0=3ff\nMB:0=50" > schemata

```
鎺ヤ笅鏉ユ垜浠负瀹炴椂鏍稿垱寤轰竴涓祫婧愮粍锛屽苟璁╁畠璁块棶 socket 0 涓婄紦瀛樼殑鈥滈《閮ㄢ€?50%
浠ュ強 socket 0 涓?50% 鐨勫唴瀛樺甫瀹姐€?
```

  # mkdir p0
  # echo "L3:0=ffc00\nMB:0=50" > p0/schemata

```
鏈€鍚庢垜浠皢鏍?4-7 绉诲埌鏂扮粍锛屽苟纭繚杩愯鍦ㄩ偅閲岀殑鍐呮牳鍜屼换鍔¤幏寰?50% 鐨勭紦瀛樸€?
鍋囪鏍?4-7 鏄?SMT 鍏勫紵鏍革紝骞朵笖鍙湁瀹炴椂绾跨▼琚皟搴﹀埌鏍?4-7 涓婏紝瀹冧滑涔熷簲璇?
鑾峰緱 50% 鐨勫唴瀛樺甫瀹姐€?
```

  # echo F0 > p0/cpus

```
4) 绀轰緥 4

鍓嶉潰绀轰緥涓殑璧勬簮缁勯兘澶勪簬榛樿鐨?"shareable" 妯″紡锛屽厑璁稿叡浜叾缂撳瓨鍒嗛厤銆傚鏋?
涓€涓祫婧愮粍閰嶇疆浜嗙紦瀛樺垎閰嶏紝娌℃湁浠讳綍涓滆タ鑳介樆姝㈠彟涓€涓祫婧愮粍涓庤鍒嗛厤閲嶅彔銆?

鍦ㄦ绀轰緥涓紝灏嗗湪涓€涓叿鏈変袱涓?L2 缂撳瓨瀹炰緥鐨?L2 CAT 绯荤粺涓婂垱寤轰竴涓柊鐨勭嫭鍗?
璧勬簮缁勶紝杩欎袱涓疄渚嬪彲浠ョ敤 8 浣嶅閲忎綅鎺╃爜閰嶇疆銆傛柊鐨勭嫭鍗犺祫婧愮粍灏嗚閰嶇疆涓轰娇鐢?
姣忎釜缂撳瓨瀹炰緥鐨?25%銆?
```

  # mount -t resctrl resctrl /sys/fs/resctrl/
  # cd /sys/fs/resctrl

```
棣栧厛锛屾垜浠瀵熷埌榛樿缁勮閰嶇疆涓哄垎閰嶅埌鎵€鏈?L2
```

  # cat schemata
  L2:0=ff;1=ff

```
鎴戜滑鏈彲浠ュ湪姝ゆ椂灏濊瘯鍒涘缓鏂扮殑璧勬簮缁勶紝浣嗗畠浼?
```

  # mkdir p0
  # echo 'L2:0=0x3;1=0x3' > p0/schemata
  # cat p0/mode
  shareable
  # echo exclusive > p0/mode
  -sh: echo: write error: Invalid argument
  # cat info/last_cmd_status
  schemata overlaps

```
涓虹‘淇濅笉涓庡彟涓€涓祫婧愮粍閲嶅彔锛屽繀椤绘洿鏀归粯璁よ祫婧愮粍鐨?schemata锛屼娇鏂扮殑璧勬簮缁?
鑳藉鍙樹负鐙崰銆?
```

  # echo 'L2:0=0xfc;1=0xfc' > schemata
  # echo exclusive > p0/mode
  # grep . p0/*
  p0/cpus:0
  p0/mode:exclusive
  p0/schemata:L2:0=03;1=03
  p0/size:L2:0=262144;1=262144

```
鏂板垱寤虹殑璧勬簮缁勪笉浼氫笌鐙崰璧勬簮缁勯噸鍙?
```

  # mkdir p1
  # grep . p1/*
  p1/cpus:0
  p1/mode:shareable
  p1/schemata:L2:0=fc;1=fc
  p1/size:L2:0=786432;1=786432

```
```

  # cat info/L2/bit_usage
  0=SSSSSSEE;1=SSSSSSEE

```
```

  # echo 'L2:0=0x1;1=0x1' > p1/schemata
  -sh: echo: write error: Invalid argument
  # cat info/last_cmd_status
  overlaps with exclusive group

```
#### 缂撳瓨浼攣瀹氱ず渚?


浣跨敤 CBM 0x3 閿佸畾缂撳瓨 id 1 涓婄殑閮ㄥ垎 L2 缂撳瓨銆備吉閿佸畾鍖哄煙鏆撮湶鍦?
/dev/pseudo_lock/newlock锛屽彲浠ヤ綔涓?mmap() 鐨勫弬鏁版彁渚涚粰搴旂敤绋嬪簭銆?
```

  # mount -t resctrl resctrl /sys/fs/resctrl/
  # cd /sys/fs/resctrl

```
纭繚鏈夊彲鐢ㄤ簬浼攣瀹氱殑浣嶏紝鍥犱负鍙湁鏈娇鐢ㄧ殑浣嶆墠鑳借浼攣瀹氾紝寰呬吉閿佸畾鐨勪綅闇€瑕?
```

  # cat info/L2/bit_usage
  0=SSSSSSSS;1=SSSSSSSS
  # echo 'L2:1=0xfc' > schemata
  # cat info/L2/bit_usage
  0=SSSSSSSS;1=SSSSSS00

```
鍒涘缓涓€涓皢涓庝吉閿佸畾鍖哄煙鍏宠仈鐨勬柊璧勬簮缁勶紝鎸囨槑瀹冨皢鐢ㄤ簬浼攣瀹氬尯鍩燂紝骞?
```

  # mkdir newlock
  # echo pseudo-locksetup > newlock/mode
  # echo 'L2:1=0x3' > newlock/schemata

```
鎴愬姛鍚庯紝璧勬簮缁勭殑妯″紡灏嗗彉涓?pseudo-locked锛宐it_usage 灏嗗弽鏄犱吉閿佸畾鍖哄煙锛?
骞朵笖瀛楃璁惧
```

  # cat newlock/mode
  pseudo-locked
  # cat info/L2/bit_usage
  0=SSSSSSSS;1=SSSSSSPP
  # ls -l /dev/pseudo_lock/newlock
  crw------- 1 root root 243, 0 Apr  3 05:01 /dev/pseudo_lock/newlock

```
```

  /*
  * Example code to access one page of pseudo-locked cache region
  * from user space.
  */
  #define _GNU_SOURCE
  #include <fcntl.h>
  #include <sched.h>
  #include <stdio.h>
  #include <stdlib.h>
  #include <unistd.h>
  #include <sys/mman.h>

  /*
  * It is required that the application runs with affinity to only
  * cores associated with the pseudo-locked region. Here the cpu
  * is hardcoded for convenience of example.
  */
  static int cpuid = 2;

  int main(int argc, char *argv[])
  {
    cpu_set_t cpuset;
    long page_size;
    void *mapping;
    int dev_fd;
    int ret;

    page_size = sysconf(_SC_PAGESIZE);

    CPU_ZERO(&cpuset);
    CPU_SET(cpuid, &cpuset);
    ret = sched_setaffinity(0, sizeof(cpuset), &cpuset);
    if (ret < 0) {
      perror("sched_setaffinity");
      exit(EXIT_FAILURE);
    }

    dev_fd = open("/dev/pseudo_lock/newlock", O_RDWR);
    if (dev_fd < 0) {
      perror("open");
      exit(EXIT_FAILURE);
    }

    mapping = mmap(0, page_size, PROT_READ | PROT_WRITE, MAP_SHARED,
            dev_fd, 0);
    if (mapping == MAP_FAILED) {
      perror("mmap");
      close(dev_fd);
      exit(EXIT_FAILURE);
    }

    /* Application interacts with pseudo-locked memory @mapping */

    ret = munmap(mapping, page_size);
    if (ret < 0) {
      perror("munmap");
      close(dev_fd);
      exit(EXIT_FAILURE);
    }

    close(dev_fd);
    exit(EXIT_SUCCESS);
  }

```
### 搴旂敤绋嬪簭涔嬮棿鐨勯攣瀹?


resctrl 鏂囦欢绯荤粺涓婄殑鏌愪簺鎿嶄綔鐢卞澶氫釜鏂囦欢鐨勮/鍐欑粍鎴愶紝蹇呴』鏄師瀛愮殑銆?

渚嬪锛屽垎閰?L3 缂撳瓨鐨勭嫭鍗犱繚鐣欐秹鍙婏細

  1. 浠庢瘡涓洰褰曟垨姣忚祫婧愮殑 "bit_usage" 璇诲彇 cbmmask
  2. 鍦ㄥ叏灞€ CBM 浣嶆帺鐮佷腑鎵惧埌涓€涓湪浠讳綍鐩綍 cbmmask 涓兘娓呮櫚鐨勮繛缁綅闆嗗悎
  3. 鍒涘缓涓€涓柊鐩綍
  4. 灏嗗湪绗?2 姝ヤ腑鎵惧埌鐨勪綅璁剧疆鍒版柊鐩綍鐨?"schemata" 鏂囦欢

濡傛灉涓や釜搴旂敤绋嬪簭灏濊瘯骞跺彂鍒嗛厤绌洪棿锛屽畠浠渶缁堝彲鑳藉垎閰嶅埌鐩稿悓鐨勪綅锛屼粠鑰屼娇寰?
淇濈暀鏄叡浜殑鑰岄潪鐙崰鐨勩€?

涓轰簡鍗忚皟 resctrlfs 涓婄殑鍘熷瓙鎿嶄綔骞堕伩鍏嶄笂杩伴棶棰橈紝寤鸿浣跨敤浠ヤ笅閿佸畾杩囩▼锛?

閿佸畾鍩轰簬 flock锛屽畠鍦?libc 涓彲鐢紝涔熷彲浠ヤ綔涓?shell 鑴氭湰鍛戒护浣跨敤

鍐欓攣锛?

 A) 瀵?/sys/fs/resctrl 鎵ц flock(LOCK_EX)
 B) 璇?鍐欑洰褰曠粨鏋勩€?
 C) funlock

璇婚攣锛?

 A) 瀵?/sys/fs/resctrl 鎵ц flock(LOCK_SH)
 B) 鑻ユ垚鍔燂紝璇诲彇鐩綍缁撴瀯銆?
 C) funlock

```

  # Atomically read directory structure
  $ flock -s /sys/fs/resctrl/ find /sys/fs/resctrl

  # Read directory contents and create new subdirectory

  $ cat create-dir.sh
  find /sys/fs/resctrl/ > output.txt
  mask = function-of(output.txt)
  mkdir /sys/fs/resctrl/newres/
  echo mask > /sys/fs/resctrl/newres/schemata

  $ flock /sys/fs/resctrl/ ./create-dir.sh

```
```

  /*
  * Example code do take advisory locks
  * before accessing resctrl filesystem
  */
  #include <sys/file.h>
  #include <stdlib.h>

  void resctrl_take_shared_lock(int fd)
  {
    int ret;

    /* take shared lock on resctrl filesystem */
    ret = flock(fd, LOCK_SH);
    if (ret) {
      perror("flock");
      exit(-1);
    }
  }

  void resctrl_take_exclusive_lock(int fd)
  {
    int ret;

    /* release lock on resctrl filesystem */
    ret = flock(fd, LOCK_EX);
    if (ret) {
      perror("flock");
      exit(-1);
    }
  }

  void resctrl_release_lock(int fd)
  {
    int ret;

    /* take shared lock on resctrl filesystem */
    ret = flock(fd, LOCK_UN);
    if (ret) {
      perror("flock");
      exit(-1);
    }
  }

  void main(void)
  {
    int fd, ret;

    fd = open("/sys/fs/resctrl", O_DIRECTORY);
    if (fd == -1) {
      perror("open");
      exit(-1);
    }
    resctrl_take_shared_lock(fd);
    /* code to read directory contents */
    resctrl_release_lock(fd);

    resctrl_take_exclusive_lock(fd);
    /* code to read and write directory contents */
    resctrl_release_lock(fd);
  }

```
## RDT 鐩戞帶涓庡垎閰嶄娇鐢ㄧず渚?


### 璇诲彇鐩戞帶鏁版嵁


璇诲彇涓€涓簨浠舵枃浠讹紙渚嬪锛歮on_data/mon_L3_00/llc_occupancy锛変細鏄剧ず鐩稿簲
MON 缁勬垨 CTRL_MON 缁勭殑 LLC 鍗犵敤鎯呭喌鐨勫綋鍓嶅揩鐓с€?


### 绀轰緥 1锛堢洃鎺?CTRL_MON 缁勪互鍙?CTRL_MON 缁勪腑鐨勪换鍔″瓙闆嗭級


鍦ㄤ竴鍙板弻鎻掓Ы鏈哄櫒锛堟瘡鎻掓Ы涓€涓?L3 缂撳瓨锛変笂锛屼粎鏈?4 浣?
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir p0 p1
  # echo "L3:0=3;1=c" > /sys/fs/resctrl/p0/schemata
  # echo "L3:0=3;1=3" > /sys/fs/resctrl/p1/schemata
  # echo 5678 > p1/tasks
  # echo 5679 > p1/tasks

```
榛樿璧勬簮缁勬湭琚慨鏀癸紝鍥犳鎴戜滑鍙互璁块棶鎵€鏈夌紦瀛樼殑鎵€鏈夐儴鍒嗭紙鍏?schemata 鏂囦欢
璇讳负 "L3:0=f;1=f"锛夈€?

澶勪簬 "p0" 缁勬帶鍒朵笅鐨勪换鍔″彧鑳戒粠缂撳瓨 ID 0 鐨勨€滆緝浣庘€?50% 鍜岀紦瀛?ID 1 鐨?
鈥滆緝楂樷€?50% 涓垎閰嶃€傚浜?"p1" 缁勪腑鐨勪换鍔″湪涓や釜鎻掓Ы涓婇兘浣跨敤缂撳瓨鐨勨€滆緝浣庘€?
50%銆?

鍒涘缓鐩戞帶缁勶紝骞朵负姣忎釜鐩戞帶缁勫垎閰嶄竴閮ㄥ垎浠诲姟銆?
```

  # cd /sys/fs/resctrl/p1/mon_groups
  # mkdir m11 m12
  # echo 5678 > m11/tasks
  # echo 5679 > m12/tasks

```
鑾峰彇鏁版嵁锛堟暟鎹互瀛楄妭鏄剧ず锛?
```

  # cat m11/mon_data/mon_L3_00/llc_occupancy
  16234000
  # cat m11/mon_data/mon_L3_01/llc_occupancy
  14789000
  # cat m12/mon_data/mon_L3_00/llc_occupancy
  16789000

```
鐖?ctrl_mon 缁勬樉绀鸿仛鍚堟暟鎹€?
```

  # cat /sys/fs/resctrl/p1/mon_data/mon_l3_00/llc_occupancy
  31234000

```
### 绀轰緥 2锛堜粠浠诲姟鍒涘缓璧峰紑濮嬬洃鎺э級


```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir p0 p1

```
涓€鏃︾粍琚垱寤猴紝灏变細涓哄畠鍒嗛厤涓€涓?RMID锛屽洜姝や笅闈㈢殑 <cmd> 浠庡叾鍒涘缓璧峰氨琚洃鎺с€?
```

  # echo $$ > /sys/fs/resctrl/p1/tasks
  # <cmd>

```
```

  # cat /sys/fs/resctrl/p1/mon_data/mon_l3_00/llc_occupancy
  31789000

```
### 绀轰緥 3锛堝湪娌℃湁 CAT 鏀寔鏃舵垨鍒涘缓 CAT 缁勪箣鍓嶈繘琛岀洃鎺э級


鍋囪涓€涓被浼?HSW 鐨勭郴缁熷彧鏈?CQM 鑰屾病鏈?CAT 鏀寔銆傚湪杩欑鎯呭喌涓?resctrl 浠嶄細
鎸傝浇锛屼絾鏃犳硶鍒涘缓 CTRL_MON 鐩綍銆備絾鐢ㄦ埛鍙互鍦ㄦ牴缁勫唴鍒涘缓涓嶅悓鐨?MON 缁勶紝浠庤€?
鑳藉鐩戞帶鍖呮嫭鍐呮牳绾跨▼鍦ㄥ唴鐨勬墍鏈変换鍔°€?

杩欎篃鍙敤浜庡湪鑳藉灏嗕綔涓氬垎閰嶅埌涓嶅悓鐨勫垎閰嶇粍涔嬪墠锛屽鍏剁紦瀛樺崰鐢ㄥぇ灏忚繘琛屽墫鏋愩€?
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir mon_groups/m01
  # mkdir mon_groups/m02

  # echo 3478 > /sys/fs/resctrl/mon_groups/m01/tasks
  # echo 2467 > /sys/fs/resctrl/mon_groups/m02/tasks

```
鍒嗗埆鐩戞帶杩欎簺缁勶紝涔熷彲浠ヨ幏鍙栨瘡鍩熸暟鎹€備粠涓嬮潰鐨勭粨鏋滃彲浠ョ湅鍑猴紝杩欎簺浠诲姟涓昏
鍦ㄥ煙锛堟彃妲斤級0 涓婂伐浣溿€?
```

  # cat /sys/fs/resctrl/mon_groups/m01/mon_L3_00/llc_occupancy
  31234000
  # cat /sys/fs/resctrl/mon_groups/m01/mon_L3_01/llc_occupancy
  34555
  # cat /sys/fs/resctrl/mon_groups/m02/mon_L3_00/llc_occupancy
  31234000
  # cat /sys/fs/resctrl/mon_groups/m02/mon_L3_01/llc_occupancy
  32789


```
### 绀轰緥 4锛堢洃鎺у疄鏃朵换鍔★級


涓€涓崟鎻掓Ы绯荤粺锛屽疄鏃朵换鍔¤繍琛屽湪鏍?4-7 涓婏紝闈炲疄鏃朵换鍔¤繍琛屽湪鍏朵粬 CPU 涓娿€?
鎴戜滑甯屾湜鐩戞帶杩欎簺鏍镐笂瀹炴椂绾跨▼鐨勭紦瀛樺崰鐢ㄦ儏鍐点€?
```

  # mount -t resctrl resctrl /sys/fs/resctrl
  # cd /sys/fs/resctrl
  # mkdir p1

```
```

  # echo f0 > p1/cpus

```
```

  # cat /sys/fs/resctrl/p1/mon_data/mon_L3_00/llc_occupancy
  11234000


```
## 浣跨敤 mbm_assign_mode 鐨勭ず渚?


a. 妫€鏌ユ槸鍚︽敮鎸?MBM 璁℃暟鍣ㄥ垎閰嶆ā寮忋€?
```

  # mount -t resctrl resctrl /sys/fs/resctrl/

  # cat /sys/fs/resctrl/info/L3_MON/mbm_assign_mode
  [mbm_event]
  default

```
"mbm_event" 妯″紡琚娴嬪埌骞跺惎鐢ㄣ€?

b. 妫€鏌ユ敮鎸佸灏戜釜鍙垎閰嶈鏁板櫒銆?
```

  # cat /sys/fs/resctrl/info/L3_MON/num_mbm_cntrs
  0=32;1=32

```
c. 妫€鏌ユ瘡涓煙涓湁澶氬皯涓彲鍒嗛厤璁℃暟鍣ㄥ彲鐢ㄤ簬鍒嗛厤銆?
```

  # cat /sys/fs/resctrl/info/L3_MON/available_mbm_cntrs
  0=30;1=30

```
d. 鍒楀嚭榛樿缁勭殑鍒嗛厤鐘舵€併€?
```

  # cat /sys/fs/resctrl/mbm_L3_assignments
  mbm_total_bytes:0=e;1=e
  mbm_local_bytes:0=e;1=e

```
e. 瑙ｉ櫎鍩?0 涓婁笌 mbm_total_bytes 浜嬩欢鍏宠仈鐨勮鏁板櫒鐨勫垎閰嶃€?
```

  # echo "mbm_total_bytes:0=_" > /sys/fs/resctrl/mbm_L3_assignments
  # cat /sys/fs/resctrl/mbm_L3_assignments
  mbm_total_bytes:0=_;1=e
  mbm_local_bytes:0=e;1=e

```
f. 瑙ｉ櫎鎵€鏈夊煙涓婁笌 mbm_total_bytes 浜嬩欢鍏宠仈鐨勮鏁板櫒鐨勫垎閰嶃€?
```

  # echo "mbm_total_bytes:*=_" > /sys/fs/resctrl/mbm_L3_assignments
  # cat /sys/fs/resctrl/mbm_L3_assignment
  mbm_total_bytes:0=_;1=_
  mbm_local_bytes:0=e;1=e

```
g. 浠ョ嫭鍗犳ā寮忎负鎵€鏈夊煙鍒嗛厤涓?mbm_total_bytes 浜嬩欢鍏宠仈鐨勮鏁板櫒銆?
```

  # echo "mbm_total_bytes:*=e" > /sys/fs/resctrl/mbm_L3_assignments
  # cat /sys/fs/resctrl/mbm_L3_assignments
  mbm_total_bytes:0=e;1=e
  mbm_local_bytes:0=e;1=e

```
h. 璇诲彇榛樿缁勭殑浜嬩欢 mbm_total_bytes 鍜?mbm_local_bytes銆傚垎閰嶅悗璇诲彇浜嬩欢娌℃湁鍙樺寲銆?
```

  # cat /sys/fs/resctrl/mon_data/mon_L3_00/mbm_total_bytes
  779247936
  # cat /sys/fs/resctrl/mon_data/mon_L3_01/mbm_total_bytes
  562324232
  # cat /sys/fs/resctrl/mon_data/mon_L3_00/mbm_local_bytes
  212122123
  # cat /sys/fs/resctrl/mon_data/mon_L3_01/mbm_local_bytes
  121212144

```
i. 妫€鏌ヤ簨浠堕厤缃€?
```

  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_total_bytes/event_filter
  local_reads,remote_reads,local_non_temporal_writes,remote_non_temporal_writes,
  local_reads_slow_memory,remote_reads_slow_memory,dirty_victim_writes_all

  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_local_bytes/event_filter
  local_reads,local_non_temporal_writes,local_reads_slow_memory

```
j. 鏇存敼 mbm_local_bytes 鐨勪簨浠堕厤缃€?
```

  # echo "local_reads, local_non_temporal_writes, local_reads_slow_memory, remote_reads" >
  /sys/fs/resctrl/info/L3_MON/event_configs/mbm_local_bytes/event_filter

  # cat /sys/fs/resctrl/info/L3_MON/event_configs/mbm_local_bytes/event_filter
  local_reads,local_non_temporal_writes,local_reads_slow_memory,remote_reads

```
k. 鐜板湪鍐嶆璇诲彇鏈湴浜嬩欢銆傜涓€娆¤鍙栧彲鑳借繑鍥?"Unavailable" 鐘舵€併€傞殢鍚庡
mbm_local_bytes 鐨勮鍙栧皢鏄剧ず褰撳墠鍊笺€?
```

  # cat /sys/fs/resctrl/mon_data/mon_L3_00/mbm_local_bytes
  Unavailable
  # cat /sys/fs/resctrl/mon_data/mon_L3_00/mbm_local_bytes
  2252323
  # cat /sys/fs/resctrl/mon_data/mon_L3_01/mbm_local_bytes
  Unavailable
  # cat /sys/fs/resctrl/mon_data/mon_L3_01/mbm_local_bytes
  1566565

```
l. 鐢ㄦ埛鍙互閫夋嫨鍦ㄩ渶瑕佹椂鍥炲埌 'default' mbm_assign_mode銆傝繖鍙互浣跨敤浠ヤ笅鍛戒护瀹屾垚銆?
娉ㄦ剰锛屽垏鎹?mbm_assign_mode 鍙兘浼氶噸缃墍鏈?resctrl 缁勭殑鎵€鏈?MBM 璁℃暟鍣紙浠ュ強
鍥犳鎵€鏈?MBM 浜嬩欢锛夈€?
```

  # echo "default" > /sys/fs/resctrl/info/L3_MON/mbm_assign_mode
  # cat /sys/fs/resctrl/info/L3_MON/mbm_assign_mode
  mbm_event
  [default]

```
m. 鍗歌浇 resctrl 鏂囦欢绯荤粺銆?
```

  # umount /sys/fs/resctrl/

```
## Intel RDT 鍕樿


### Intel MBM 璁℃暟鍣ㄥ彲鑳介敊璇湴鎶ュ憡绯荤粺鍐呭瓨甯﹀


Skylake 鏈嶅姟鍣ㄧ殑鍕樿 SKX99 鍜?Broadwell 鏈嶅姟鍣ㄧ殑鍕樿 BDF102銆?

闂锛欼ntel 鍐呭瓨甯﹀鐩戞帶锛圡BM锛夎鏁板櫒鏍规嵁閫昏緫鏍稿垎閰嶇殑 Resource Monitor ID
锛圧MID锛夎窡韪寚鏍囥€傜敤浜庢姤鍛婅繖浜涙寚鏍囩殑 IA32_QM_CTR 瀵勫瓨鍣紙MSR 0xC8E锛夊彲鑳?
瀵规煇浜?RMID 鍊兼姤鍛婁笉姝ｇ‘鐨勭郴缁熷甫瀹姐€?

褰卞搷锛氱敱浜庤鍕樿锛岀郴缁熷唴瀛樺甫瀹藉彲鑳戒笌鎶ュ憡鍊间笉鍖归厤銆?

瑙勯伩鏂规硶锛歁BM 鎬昏鏁板拰鏈湴璇绘暟鏍规嵁浠ヤ笅鏍℃鍥犲瓙琛ㄨ繘琛屾牎姝ｏ細

+---------------+---------------+---------------+-----------------+
|鏍告暟閲?|RMID 鏁伴噺	|RMID 闃堝€?|鏍℃鍥犲瓙|
+---------------+---------------+---------------+-----------------+
|1		|8		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|2		|16		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|3		|24		|15		|0.969650	  |
+---------------+---------------+---------------+-----------------+
|4		|32		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|6		|48		|31		|0.969650	  |
+---------------+---------------+---------------+-----------------+
|7		|56		|47		|1.142857	  |
+---------------+---------------+---------------+-----------------+
|8		|64		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|9		|72		|63		|1.185115	  |
+---------------+---------------+---------------+-----------------+
|10		|80		|63		|1.066553	  |
+---------------+---------------+---------------+-----------------+
|11		|88		|79		|1.454545	  |
+---------------+---------------+---------------+-----------------+
|12		|96		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|13		|104		|95		|1.230769	  |
+---------------+---------------+---------------+-----------------+
|14		|112		|95		|1.142857	  |
+---------------+---------------+---------------+-----------------+
|15		|120		|95		|1.066667	  |
+---------------+---------------+---------------+-----------------+
|16		|128		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|17		|136		|127		|1.254863	  |
+---------------+---------------+---------------+-----------------+
|18		|144		|127		|1.185255	  |
+---------------+---------------+---------------+-----------------+
|19		|152		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|20		|160		|127		|1.066667	  |
+---------------+---------------+---------------+-----------------+
|21		|168		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|22		|176		|159		|1.454334	  |
+---------------+---------------+---------------+-----------------+
|23		|184		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|24		|192		|127		|0.969744	  |
+---------------+---------------+---------------+-----------------+
|25		|200		|191		|1.280246	  |
+---------------+---------------+---------------+-----------------+
|26		|208		|191		|1.230921	  |
+---------------+---------------+---------------+-----------------+
|27		|216		|0		|1.000000	  |
+---------------+---------------+---------------+-----------------+
|28		|224		|191		|1.143118	  |
+---------------+---------------+---------------+-----------------+

濡傛灉 rmid > rmid 闃堝€硷紝MBM 鎬昏鏁板拰鏈湴璇绘暟搴斾箻浠ユ牎姝ｅ洜瀛愩€?

鍙傝锛?

1. Intel Xeon 澶勭悊鍣ㄥ彲鎵╁睍瀹舵棌瑙勬牸鏇存柊涓殑鍕樿 SKX99锛?
http://web.archive.org/web/20200716124958/https://www.intel.com/content/www/us/en/processors/xeon/scalable/xeon-scalable-spec-update.html

2. Intel Xeon E5-2600 v4 澶勭悊鍣ㄤ骇鍝佸鏃忚鏍兼洿鏂颁腑鐨勫嫎璇?BDF102锛?
http://web.archive.org/web/20191125200531/https://www.intel.com/content/dam/www/public/us/en/documents/specification-updates/xeon-e5-v4-spec-update.pdf

3. 绗簩浠?Intel Xeon 鍙墿灞曞鐞嗗櫒鍙傝€冩墜鍐屼腑 Intel Resource Director Technology锛圛ntel RDT锛夌殑鍕樿锛?
https://software.intel.com/content/www/us/en/develop/articles/intel-resource-director-technology-rdt-reference-manual.html

浠ヨ幏鍙栨洿澶氫俊鎭€?
