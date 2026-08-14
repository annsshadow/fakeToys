
## Intel Uncore 棰戠巼璋冭妭


:Copyright: |copy| 2022-2023 Intel Corporation

:Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>

### 绠€浠?

鍦ㄥ熀浜?Intel 鐨勫伐浣滆礋杞界壒寰佷笅锛寀ncore锛堥潪鏍革級閮ㄥ垎浼氭秷鑰楃浉褰撳彲瑙傜殑鍔熻€椼€?涓轰簡浼樺寲鎬诲姛鑰楀苟鎻愬崌鏁翠綋鎬ц兘锛孲oC 鍐呴儴鎻愪緵浜嗙敤浜庤皟鑺?uncore 棰戠巼鐨勭畻娉曘€?杩欎簺绠楁硶鐩戞帶 uncore 鐨勫伐浣滆礋杞戒娇鐢ㄦ儏鍐碉紝骞惰缃竴涓悎閫傜殑棰戠巼銆?
鐢ㄦ埛鍙兘瀵?uncore 鎬ц兘鏈変笉鍚岀殑鏈熸湜锛屽苟甯屾湜瀵瑰叾鍔犱互鎺у埗銆傚叾鐩爣绫讳技浜庡厑璁?鐢ㄦ埛閫氳繃 cpufreq 鐨?sysfs 鎺ュ彛璁剧疆缂╂斁鐨勬渶灏?鏈€澶ч鐜囨潵鎻愬崌 CPU 鎬ц兘銆?鐢ㄦ埛鍙兘鏈変竴浜涘寤惰繜鏁忔劅鐨勫伐浣滆礋杞斤紝涓嶅笇鏈?uncore 棰戠巼鍙戠敓浠讳綍鍙樺寲銆傛澶栵紝
鐢ㄦ埛涔熷彲鑳芥湁鍦ㄥ垎闃舵闃舵闇€瑕佷笉鍚?core 涓?uncore 鎬ц兘鐨勫伐浣滆礋杞斤紝骞跺彲鑳藉笇鏈?鍚屾椂浣跨敤 cpufreq 鍜?uncore 缂╂斁鎺ュ彛鏉ュ垎閰嶅姛鑰椼€佹彁鍗囨暣浣撴€ц兘銆?
### Sysfs 鎺ュ彛


涓轰簡鎺у埗 uncore 棰戠巼锛屽湪浠ヤ笅鐩綍鎻愪緵浜?sysfs 鎺ュ彛锛?`/sys/devices/system/cpu/intel_uncore_frequency/`銆?
瀵逛簬姣忎竴涓?package锛堝皝瑁咃級鍜?die 鐨勭粍鍚堥兘鏈変竴涓洰褰曪紝鍥犱负 uncore 缂╂斁鎺у埗鐨?浣滅敤鑼冨洿锛屽湪澶?die/package 鐨?SoC 涓婃槸鎸?die 鍒掑垎锛岃€屽湪姣?package 鍗?die 鐨?SoC 涓婃槸鎸?package 鍒掑垎銆傜洰褰曞悕绉颁唬琛ㄤ簡鎺у埗鐨勪綔鐢ㄨ寖鍥淬€備緥濡傦細
'package_00_die_00' 瀵瑰簲 package id 0 鍜?die 0銆?
姣忎釜 package_**_die_** 鍖呭惈浠ヤ笅灞炴€э細

`initial_max_freq_khz`
	澶嶄綅鍚庯紝璇ュ睘鎬т唬琛ㄥ彲鑳界殑鏈€澶ч鐜囥€?	杩欐槸涓€涓彧璇诲睘鎬с€傚鏋滅敤鎴疯皟鏁翠簡 max_freq_khz锛?	浠栦滑闅忔椂鍙互浣跨敤璇ュ睘鎬х殑鍊煎洖鍒版渶澶ч鐜囥€?
`initial_min_freq_khz`
	澶嶄綅鍚庯紝璇ュ睘鎬т唬琛ㄥ彲鑳界殑鏈€灏忛鐜囥€?	杩欐槸涓€涓彧璇诲睘鎬с€傚鏋滅敤鎴疯皟鏁翠簡 min_freq_khz锛?	浠栦滑闅忔椂鍙互浣跨敤璇ュ睘鎬х殑鍊煎洖鍒版渶灏忛鐜囥€?
`max_freq_khz`
	璇ュ睘鎬х敤浜庤缃?uncore 鐨勬渶澶ч鐜囥€?
`min_freq_khz`
	璇ュ睘鎬х敤浜庤缃?uncore 鐨勬渶灏忛鐜囥€?
`current_freq_khz`
	璇ュ睘鎬х敤浜庤幏鍙栧綋鍓?uncore 棰戠巼銆?
### 甯︽湁 TPMI锛圱opology Aware Register and PM Capsule Interface锛屾嫇鎵戞劅鐭ュ瘎瀛樺櫒涓庡姛鑰楃鐞嗚兌鍥婃帴鍙ｏ級鐨?SoC


涓€涓?SoC 鍙互鍖呭惈澶氫釜鍔熻€楀煙锛屽叾涓惈鏈夌嫭绔嬫垨鎴愮粍鐨?mesh锛堢綉鐘讹級鍒嗗尯銆傝繖绉?鍒嗗尯琚О涓?fabric cluster锛堜簰杩炵皣锛夈€?
鏌愪簺绫诲瀷鐨?mesh 闇€瑕佷互鐩稿悓棰戠巼杩愯锛屽畠浠細琚斁鍦ㄥ悓涓€涓?fabric cluster 涓€?fabric cluster 鐨勫ソ澶勫湪浜庯紝瀹冩彁渚涗簡涓€绉嶅彲鎵╁睍鐨勬満鍒舵潵澶勭悊 SoC 涓垎鍖哄寲鐨?浜掕繛缁撴瀯銆?
褰撳墠鐨?sysfs 鎺ュ彛鏀寔鍦?package 鍜?die 绾у埆杩涜鎺у埗銆傝鎺ュ彛涓嶈冻浠ユ敮鎸佸湪
fabric cluster 绾у埆杩涜鏇寸簿缁嗙殑鎺у埗銆?
鏀寔 TPMI锛圱opology Aware Register and PM Capsule Interface锛屾嫇鎵戞劅鐭ュ瘎瀛樺櫒涓?鍔熻€楃鐞嗚兌鍥婃帴鍙ｏ級鐨?SoC 鍙互鎷ユ湁澶氫釜鍔熻€楀煙銆傛瘡涓姛鑰楀煙鍙寘鍚竴涓垨澶氫釜
fabric cluster銆?
涓轰簡鍦?package 鍜?die 绾у埆鎺у埗锛堝鍚屼笉鏀寔 TPMI 鐨勭郴缁燂級涔嬪锛岃繕鑳借〃杈?fabric cluster 绾у埆鐨勬帶鍒讹紝sysfs 寰楀埌浜嗗寮恒€傝繖绉嶆洿绮剧粏鐨勬帴鍙ｅ湪 sysfs 涓?浠ュ悕涓?"uncore" 鍓嶇紑鐨勭洰褰曞憟鐜般€備緥濡傦細uncore00銆乽ncore01 绛夈€?
鎺у埗鐨勪綔鐢ㄨ寖鍥寸敱鐩綍涓殑 "package_id"銆?domain_id" 鍜?"fabric_cluster_id"
灞炴€ф寚瀹氥€?
姣忎釜鐩綍涓殑灞炴€э細

`domain_id`
	璇ュ睘鎬х敤浜庤幏鍙栬瀹炰緥鐨勫姛鑰楀煙 id銆?
`die_id`
	璇ュ睘鎬х敤浜庤幏鍙栬瀹炰緥鐨?Linux die id銆?	璇ュ睘鎬т粎鍦ㄤ娇鐢?core agent 鐨勫煙涓瓨鍦紝
	涓斿綋 CPUID leaf 0x1f 鎻愪緵 die ID 鏃舵墠鍑虹幇銆?
`fabric_cluster_id`
	璇ュ睘鎬х敤浜庤幏鍙栬瀹炰緥鐨?fabric cluster id銆?
`package_id`
	璇ュ睘鎬х敤浜庤幏鍙栬瀹炰緥鐨?package id銆?
`agent_types`
	璇ュ睘鎬ф樉绀哄煙鍐呭瓨鍦ㄧ殑鎵€鏈夌‖浠?agent锛堜唬鐞嗭級銆傛瘡涓?agent 鑳藉鎺у埗
	涓€涓垨澶氫釜纭欢瀛愮郴缁燂紝鍖呮嫭锛歝ore銆乧ache銆乵emory锛堝唴瀛橈級鍜?I/O銆?
鍏朵綑灞炴€т笌 package_**_die_** 绾у埆鎵€鍛堢幇鐨勭浉鍚屻€?
鍦ㄥ綋鍓嶅ぇ澶氭暟鐢ㄤ緥涓紝"max_freq_khz" 鍜?"min_freq_khz" 鏄湪 "package_**_die_**"
绾у埆鏇存柊鐨勩€備互涓嬫柟寮忎粛灏嗘敮鎸佽妯″紡锛?
褰撶敤鎴蜂娇鐢?"package_**_die_**" 绾у埆鐨勬帶鍒舵椂锛岃 package 鍜?die 涓殑姣忎竴涓?fabric cluster 閮戒細鍙楀埌褰卞搷銆備緥濡傦細鐢ㄦ埛鍦?package_00_die_00 涓慨鏀逛簡
"max_freq_khz"锛岄偅涔堝叿鏈夌浉鍚?package id 鐨?uncore* 鐩綍涓殑 "max_freq_khz"
涔熶細琚洿鏂般€傚湪杩欑鎯呭喌涓嬶紝鐢ㄦ埛浠嶅彲浠ュ湪姣忎釜 uncore* 绾у埆鏇存柊 "max_freq_khz"锛?杩欎細鏇翠弗鏍笺€傜被浼煎湴锛岀敤鎴峰彲浠ュ湪 "package_**_die_**" 绾у埆鏇存柊 "min_freq_khz"
浠ュ簲鐢ㄥ埌姣忎釜 uncore* 绾у埆銆?
"current_freq_khz" 鐨勬敮鎸佷粎瀛樺湪浜庢瘡涓?fabric cluster 绾у埆锛堝嵆鍦?uncore* 鐩綍涓級銆?
### 鏁堢巼涓庡欢杩熺殑鏉冭　


Efficiency Latency Control锛圗LC锛屾晥鐜囧欢杩熸帶鍒讹級鐗规€у彲鎻愬崌姣忕摝鎬ц兘銆傚€熷姪璇ョ壒鎬э紝
纭欢鍔熻€楃鐞嗙畻娉曚細鍦ㄥ欢杩熷拰鍔熻€椾箣闂磋繘琛屼紭鍖栨潈琛°€傚浜庝竴浜涘寤惰繜鏁忔劅鐨勫伐浣滆礋杞斤紝
鍙互閫氳繃杞欢杩涜杩涗竴姝ヨ皟浼橈紝浠ヨ幏寰楁湡鏈涚殑鎬ц兘銆?
纭欢浠ュ浐瀹氶棿闅旂洃鎺т竴涓姛鑰楀煙鍐呮墍鏈?core 鐨勫钩鍧?CPU 鍒╃敤鐜囷紝骞跺喅瀹氫竴涓?uncore
棰戠巼銆傝櫧鐒惰繖鍙兘甯︽潵鏈€浣崇殑姣忕摝鎬ц兘锛屼絾宸ヤ綔璐熻浇鍙兘鏈熸湜浠ユ洿楂樼殑鍔熻€椾负浠ｄ环鑾峰緱
鏇撮珮鐨勬€ц兘銆傝€冭檻涓€涓湪绌洪棽绯荤粺涓婇棿姝囨€у敜閱掍互鎵ц鍐呭瓨璇诲彇鐨勫簲鐢ㄧ▼搴忋€傚湪杩欑
鎯呭喌涓嬶紝濡傛灉纭欢闄嶄綆浜?uncore 棰戠巼锛岄偅涔堥鐜囩埇鍗囧埌婊¤冻鐩爣鎬ц兘鍙兘瀛樺湪寤惰繜銆?
ELC 鎺у埗瀹氫箟浜嗕竴浜涘彲鐢辫蒋浠舵洿鏀圭殑鍙傛暟銆傚鏋滃钩鍧?CPU 鍒╃敤鐜囦綆浜庣敤鎴峰畾涔夌殑闃堝€?锛堜笅闈㈢殑 elc_low_threshold_percent 灞炴€э級锛屽皢浣跨敤鐢ㄦ埛瀹氫箟鐨?uncore 涓嬮檺棰戠巼
锛堜笅闈㈢殑 elc_floor_freq_khz 灞炴€э級锛岃€屼笉鏄‖浠惰绠楀嚭鐨勬渶灏忓€笺€?
绫讳技鍦帮紝鍦ㄩ珮璐熻浇鍦烘櫙涓嬶紝褰?CPU 鍒╃敤鐜囪秴杩囬珮闃堝€硷紙涓嬮潰鐨?elc_high_threshold_percent
灞炴€э級鏃讹紝棰戠巼浼氫互 100MHz 鐨勬闀块€掑锛岃€屼笉鏄洿鎺ヨ烦鍒版渶澶?uncore 棰戠巼銆傝繖閬垮厤浜?鍥?CPU 鍒╃敤鐜囩獊澧炶€岀珛鍗虫秷鑰椾笉蹇呰鐨勯珮鍔熻€椼€?
鏁堢巼寤惰繜鎺у埗鐨勫睘鎬э細

`elc_floor_freq_khz`
	璇ュ睘鎬х敤浜庤幏鍙?璁剧疆鏁堢巼寤惰繜涓嬮檺棰戠巼銆?	濡傛灉璇ュ€间綆浜?'min_freq_khz'锛屽浐浠跺皢蹇界暐瀹冦€?
`elc_low_threshold_percent`
	璇ュ睘鎬х敤浜庤幏鍙?璁剧疆鏁堢巼寤惰繜鎺у埗鐨勪綆闃堝€笺€傝灞炴€т互 CPU 鍒╃敤鐜囩殑鐧惧垎姣旇〃绀恒€?
`elc_high_threshold_percent`
	璇ュ睘鎬х敤浜庤幏鍙?璁剧疆鏁堢巼寤惰繜鎺у埗鐨勯珮闃堝€笺€傝灞炴€т互 CPU 鍒╃敤鐜囩殑鐧惧垎姣旇〃绀恒€?
`elc_high_threshold_enable`
	璇ュ睘鎬х敤浜庡惎鐢?绂佺敤鏁堢巼寤惰繜鎺у埗鐨勯珮闃堝€笺€傚啓 '1' 鍚敤锛?0' 绂佺敤銆?
涓嬮潰鐨勭ず渚嬬郴缁熼厤缃仛浜嗗涓嬩簨鎯咃細
  - 褰?CPU 鍒╃敤鐜囦綆浜?10% 鏃讹細灏?uncore 棰戠巼璁剧疆涓?800MHz
  - 褰?CPU 鍒╃敤鐜囬珮浜?95% 鏃讹細浠?100MHz 姝ラ暱閫掑 uncore 棰戠巼锛岀洿鍒拌揪鍒板姛鑰椾笂闄?
  elc_floor_freq_khz:800000
  elc_high_threshold_percent:95
  elc_high_threshold_enable:1
  elc_low_threshold_percent:10
