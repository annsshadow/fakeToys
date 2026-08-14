
## CPUSETS锛圕PU 闆嗗悎锛?

Copyright (C) 2004 BULL SA.

Written by Simon.Derr@bull.net

- Portions Copyright (c) 2004-2006 Silicon Graphics, Inc.
- Modified by Paul Jackson <pj@sgi.com>
- Modified by Christoph Lameter <cl@gentwo.org>
- Modified by Paul Menage <menage@google.com>
- Modified by Hidetoshi Seto <seto.hidetoshi@jp.fujitsu.com>


   1. Cpusets锛圕PU 闆嗗悎锛?     1.1 浠€涔堟槸 cpusets锛?     1.2 涓轰粈涔堥渶瑕?cpusets锛?     1.3 cpusets 鏄浣曞疄鐜扮殑锛?     1.4 浠€涔堟槸鐙崰 cpusets锛?     1.5 浠€涔堟槸 memory_pressure锛?     1.6 浠€涔堟槸 memory spread锛?     1.7 浠€涔堟槸 sched_load_balance锛?     1.8 浠€涔堟槸 sched_relax_domain_level锛?     1.9 鎴戝浣曚娇鐢?cpusets锛?   2. 浣跨敤绀轰緥涓庤娉?     2.1 鍩烘湰鐢ㄦ硶
     2.2 娣诲姞/绉婚櫎 cpus
     2.3 璁剧疆鏍囧織
     2.4 闄勫姞杩涚▼
   3. 闂
   4. 鑱旂郴鏂瑰紡

## 1. Cpusets锛圕PU 闆嗗悎锛?

### 1.1 浠€涔堟槸 cpusets锛?

Cpusets 鎻愪緵浜嗕竴绉嶆満鍒讹紝鐢ㄤ簬灏嗕竴缁?CPU 鍜屽唴瀛樿妭鐐癸紙Memory Nodes锛夊垎閰嶇粰涓€缁勪换鍔°€傚湪鏈枃妗ｄ腑锛?鍐呭瓨鑺傜偣锛圡emory Node锛?鎸囩殑鏄寘鍚唴瀛樼殑鍦ㄧ嚎鑺傜偣銆?
Cpusets 灏嗕换鍔＄殑 CPU 鍜屽唴瀛樻斁缃檺鍒朵负浠呭湪鍏跺綋鍓?cpuset 鍐呯殑璧勬簮銆傚畠浠舰鎴愪簡涓€涓祵濂楃殑銆佸湪铏氭嫙鏂囦欢绯荤粺涓彲瑙佺殑灞傛缁撴瀯銆傝繖浜涙槸闄や簡鐜版湁鏈哄埗涔嬪銆佺鐞嗗ぇ鍨嬬郴缁熶笂鍔ㄦ€佷綔涓氭斁缃墍闇€鐨勫繀瑕侀挬瀛愩€?
Cpusets 浣跨敤 Documentation/admin-guide/cgroup-v1/cgroups.rst 涓弿杩扮殑閫氱敤 cgroup 瀛愮郴缁熴€?
浠诲姟浣跨敤 sched_setaffinity(2) 绯荤粺璋冪敤灏嗗叾 CPU 鍖呭惈杩?CPU 浜插拰鎬ф帺鐮侊紝骞朵娇鐢?mbind(2) 鍜?set_mempolicy(2) 绯荤粺璋冪敤灏嗗唴瀛樿妭鐐瑰寘鍚繘鍏跺唴瀛樼瓥鐣ワ紝杩欎簺璇锋眰閮戒細缁忚繃璇ヤ换鍔＄殑 cpuset 杩囨护锛岃繃婊ゆ帀涓嶅湪璇?cpuset 涓殑浠讳綍 CPU 鎴栧唴瀛樿妭鐐广€傝皟搴﹀櫒涓嶄細鍦ㄤ换鍔＄殑 cpus_allowed 鍚戦噺鎵€涓嶅厑璁哥殑 CPU 涓婅皟搴︿换鍔★紝骞朵笖鍐呮牳椤靛垎閰嶅櫒涓嶄細鍦ㄨ姹備换鍔?mems_allowed 鍚戦噺鎵€涓嶅厑璁哥殑鑺傜偣涓婂垎閰嶉〉銆?
鐢ㄦ埛绾т唬鐮佸彲浠ュ湪 cgroup 铏氭嫙鏂囦欢绯荤粺涓寜鍚嶇О鍒涘缓鍜岄攢姣?cpusets锛岀鐞嗚繖浜?cpusets 鐨勫睘鎬т笌鏉冮檺锛屼互鍙婂垎閰嶇粰姣忎釜 cpuset 鐨?CPU 鍜屽唴瀛樿妭鐐癸紝鎸囧畾骞舵煡璇换鍔¤鍒嗛厤鍒板摢涓?cpuset锛屽苟鍒楀嚭鍒嗛厤缁欐煇涓?cpuset 鐨勪换鍔?pid銆?

### 1.2 涓轰粈涔堥渶瑕?cpusets锛?

绠＄悊鍏锋湁璁稿澶勭悊鍣紙CPU锛夈€佸鏉傚唴瀛樼紦瀛樺眰娆＄粨鏋勪互鍙婂叿鏈夐潪鍧囧寑璁块棶鏃堕棿锛圢UMA锛夌殑澶氫釜鍐呭瓨鑺傜偣鐨勫ぇ鍨嬭绠楁満绯荤粺锛屽杩涚▼鐨勯珮鏁堣皟搴﹀拰鍐呭瓨鏀剧疆鎻愬嚭浜嗛澶栫殑鎸戞垬銆?
閫氬父锛屾洿閫備腑瑙勬ā鐨勭郴缁熷彲浠ラ€氳繃璁╂搷浣滅郴缁熷湪璇锋眰鐨勪换鍔′箣闂磋嚜鍔ㄥ叡浜彲鐢ㄧ殑 CPU 鍜屽唴瀛樿祫婧愶紝浠ヨ冻澶熺殑鏁堢巼杩愯銆?
浣嗘洿澶х殑绯荤粺浠庝粩缁嗙殑澶勭悊鍣ㄥ拰鍐呭瓨鏀剧疆涓幏鐩婃洿澶氾紝浠ュ噺灏戝唴瀛樿闂椂闂村拰浜夌敤锛屽苟涓旈€氬父浠ｈ〃浜嗗鎴锋洿澶х殑鎶曡祫锛屽彲浠ヤ粠灏嗕綔涓氭樉寮忔斁缃湪閫傚綋澶у皬鐨勭郴缁熷瓙闆嗕笂鑾风泭銆?
杩欏湪浠ヤ笅鍦烘櫙灏ゅ叾鏈変环鍊硷細

    - 杩愯鍚屼竴 Web 搴旂敤澶氫釜瀹炰緥鐨?Web 鏈嶅姟鍣紝
    - 杩愯涓嶅悓搴旂敤鐨勬湇鍔″櫒锛堜緥濡傦紝涓€涓?Web 鏈嶅姟鍣ㄥ拰涓€涓暟鎹簱锛夛紝鎴?    - 杩愯鍏锋湁鑻涘埢鎬ц兘鐗瑰緛鐨勭殑澶у瀷 HPC 搴旂敤鐨?NUMA 绯荤粺銆?
杩欎簺瀛愰泦锛屾垨"杞垎鍖猴紙soft partitions锛?蹇呴』鑳藉闅忕潃浣滀笟缁勫悎鐨勫彉鍖栬€屽姩鎬佽皟鏁达紝鑰屼笉褰卞搷鍏朵粬骞跺彂鎵ц鐨勪綔涓氥€傝繍琛屼綔涓氶〉闈㈢殑浣嶇疆涔熷彲鑳藉湪鍐呭瓨浣嶇疆鏀瑰彉鏃惰绉诲姩銆?
鍐呮牳 cpuset 琛ヤ竵鎻愪緵浜嗛珮鏁堝疄鐜版绫诲瓙闆嗘墍闇€鐨勬渶灏忓繀瑕佸唴鏍告満鍒躲€傚畠鍒╃敤 Linux 鍐呮牳涓幇鏈夌殑 CPU 鍜屽唴瀛樻斁缃鏂斤紝浠ラ伩鍏嶅鍏抽敭鐨勮皟搴﹀櫒鎴栧唴瀛樺垎閰嶅櫒浠ｇ爜浜х敓浠讳綍棰濆褰卞搷銆?

### 1.3 cpusets 鏄浣曞疄鐜扮殑锛?

Cpusets 鎻愪緵浜嗕竴绉?Linux 鍐呮牳鏈哄埗锛岀敤浜庣害鏉熻繘绋嬫垨涓€缁勮繘绋嬫墍浣跨敤鐨?CPU 鍜屽唴瀛樿妭鐐广€?
Linux 鍐呮牳宸茬粡鏈変竴瀵规満鍒舵潵鎸囧畾浠诲姟鍙互鍦ㄥ摢浜?CPU 涓婅璋冨害锛坰ched_setaffinity锛変互鍙婂彲浠ヤ粠鍝簺鍐呭瓨鑺傜偣鑾峰彇鍐呭瓨锛坢bind銆乻et_mempolicy锛夈€?
Cpusets 瀵硅繖涓ょ鏈哄埗鎵╁睍濡備笅锛?
 - Cpusets 鏄唴鏍告墍鐭ョ殑銆佸厑璁哥殑 CPU 鍜屽唴瀛樿妭鐐圭殑闆嗗悎銆? - 绯荤粺涓殑姣忎釜浠诲姟閮介€氳繃涓€涓寚鍚戝紩鐢ㄨ鏁?cgroup 缁撴瀯鐨勬寚閽堥檮鍔犲埌涓€涓?cpuset銆? - 瀵?sched_setaffinity 鐨勮皟鐢ㄨ杩囨护涓轰粎闄愯浠诲姟 cpuset 涓厑璁哥殑 CPU銆? - 瀵?mbind 鍜?set_mempolicy 鐨勮皟鐢ㄨ杩囨护涓轰粎闄愯浠诲姟 cpuset 涓厑璁哥殑鍐呭瓨鑺傜偣銆? - 鏍?cpuset 鍖呭惈绯荤粺鐨勬墍鏈?CPU 鍜屽唴瀛樿妭鐐广€? - 瀵逛簬浠讳綍 cpuset锛屽彲浠ュ畾涔夊寘鍚埗绾?CPU 鍜屽唴瀛樿妭鐐硅祫婧愬瓙闆嗙殑瀛?cpusets銆? - cpusets 鐨勫眰娆＄粨鏋勫彲浠ユ寕杞藉湪 /dev/cpuset锛屼互渚夸粠鐢ㄦ埛绌洪棿娴忚鍜屾搷浣溿€? - 涓€涓?cpuset 鍙互琚爣璁颁负鐙崰锛坋xclusive锛夛紝杩欑‘淇濇病鏈夊叾浠?cpuset锛堢洿鎺ョ鍏堝拰鍚庝唬闄ゅ锛夊彲浠ュ寘鍚换浣曢噸鍙犵殑 CPU 鎴栧唴瀛樿妭鐐广€? - 浣犲彲浠ュ垪鍑洪檮鍔犲埌浠讳綍 cpuset 鐨勬墍鏈変换鍔★紙鎸?pid锛夈€?
cpusets 鐨勫疄鐜伴渶瑕佸皯閲忋€佺畝鍗曠殑閽╁瓙鎻掑叆鍐呮牳鍏朵綑閮ㄥ垎锛屼笖閮戒笉鍦ㄦ€ц兘鍏抽敭璺緞涓婏細

 - 鍦?init/main.c 涓紝鍦ㄧ郴缁熷惎鍔ㄦ椂鍒濆鍖栨牴 cpuset銆? - 鍦?fork 鍜?exit 涓紝灏嗕换鍔￠檮鍔犲埌鍜屼粠鍏?cpuset 鍒嗙銆? - 鍦?sched_setaffinity 涓紝鐢ㄨ浠诲姟 cpuset 涓厑璁哥殑鍐呭灞忚斀璇锋眰鐨?CPU銆? - 鍦?sched.c 鐨?migrate_live_tasks() 涓紝灏藉彲鑳藉皢浠诲姟淇濈暀鍦ㄥ叾 cpuset 鍏佽鐨?CPU 鍐呰縼绉汇€? - 鍦?mbind 鍜?set_mempolicy 绯荤粺璋冪敤涓紝鐢ㄨ浠诲姟 cpuset 涓厑璁哥殑鍐呭灞忚斀璇锋眰鐨勫唴瀛樿妭鐐广€? - 鍦?page_alloc.c 涓紝灏嗗唴瀛橀檺鍒朵负鍏佽鐨勮妭鐐广€? - 鍦?vmscan.c 涓紝灏嗛〉鍥炴敹闄愬埗鍦ㄥ綋鍓?cpuset 鍐呫€?
浣犲簲璇ユ寕杞?"cgroup" 鏂囦欢绯荤粺绫诲瀷锛屼互鍚敤娴忚鍜屼慨鏀瑰唴鏍稿綋鍓嶅凡鐭ョ殑 cpusets銆傛病鏈変负 cpusets 娣诲姞鏂扮殑绯荤粺璋冪敤鈥斺€旀墍鏈夋煡璇㈠拰淇敼 cpusets 鐨勬敮鎸侀兘閫氳繃姝?cpuset 鏂囦欢绯荤粺銆?
姣忎釜浠诲姟鐨?/proc/<pid>/status 鏂囦欢鏈夋柊澧炵殑鍥涜锛屾樉绀轰换鍔＄殑 cpus_allowed锛堝彲鍦ㄥ叾涓婅璋冨害鐨?CPU锛夊拰 mems_allowed锛堝彲浠庝腑鑾峰彇鍐呭瓨鐨勫唴瀛樿妭鐐癸級锛?```

  Cpus_allowed:   ffffffff,ffffffff,ffffffff,ffffffff
  Cpus_allowed_list:      0-127
  Mems_allowed:   ffffffff,ffffffff
  Mems_allowed_list:      0-63

```
姣忎釜 cpuset 鐢?cgroup 鏂囦欢绯荤粺涓殑鐩綍琛ㄧず锛岃鐩綍鍖呭惈锛堝湪鏍囧噯 cgroup 鏂囦欢涔嬩笂锛夋弿杩拌 cpuset 鐨勪互涓嬫枃浠讹細

 - cpuset.cpus锛氳 cpuset 涓殑 CPU 鍒楄〃
 - cpuset.mems锛氳 cpuset 涓殑鍐呭瓨鑺傜偣鍒楄〃
 - cpuset.memory_migrate 鏍囧織锛氳嫢璁剧疆锛屽皢椤电Щ鍔ㄥ埌 cpusets 鑺傜偣
 - cpuset.cpu_exclusive 鏍囧織锛欳PU 鏀剧疆鏄惁鐙崰锛? - cpuset.mem_exclusive 鏍囧織锛氬唴瀛樻斁缃槸鍚︾嫭鍗狅紵
 - cpuset.mem_hardwall 鏍囧織锛氬唴瀛樺垎閰嶆槸鍚︾‖澧欓殧绂? - cpuset.memory_pressure锛歝puset 涓垎椤靛帇鍔涘ぇ灏忕殑搴﹂噺
 - cpuset.memory_spread_page 鏍囧織锛氳嫢璁剧疆锛屽湪鍏佽鐨勮妭鐐逛笂鍧囧寑鍒嗘暎椤电紦瀛? - cpuset.memory_spread_slab 鏍囧織锛氬凡搴熷純銆傛病鏈変换浣曞姛鑳姐€? - cpuset.sched_load_balance 鏍囧織锛氳嫢璁剧疆锛屽湪璇?cpuset 鍐呯殑 CPU 涓婂仛璐熻浇鍧囪　
 - cpuset.sched_relax_domain_level锛氳縼绉讳换鍔℃椂鐨勬悳绱㈣寖鍥?
姝ゅ锛屽彧鏈夋牴 cpuset 鍏锋湁浠ヤ笅鏂囦欢锛?
 - cpuset.memory_pressure_enabled 鏍囧織锛氭槸鍚﹁绠?memory_pressure锛?
鏂扮殑 cpusets 鏄娇鐢?mkdir 绯荤粺璋冪敤鎴?shell 鍛戒护鍒涘缓鐨勩€俢puset 鐨勫睘鎬э紝渚嬪鍏舵爣蹇椼€佸厑璁哥殑 CPU 鍜屽唴瀛樿妭鐐癸紝浠ュ強闄勫姞鐨勪换鍔★紝閫氳繃鍐欏叆璇?cpuset 鐩綍涓殑鐩稿簲鏂囦欢鏉ヤ慨鏀癸紝濡備笂鎵€鍒椼€?
宓屽 cpusets 鐨勫懡鍚嶅眰娆＄粨鏋勫厑璁稿皢澶у瀷绯荤粺鍒掑垎涓哄祵濂楃殑銆佸彲鍔ㄦ€佸彉鏇寸殑"杞垎鍖?銆?
姣忎釜浠诲姟鐨勯檮鍔狅紙鍦ㄨ浠诲姟 fork 鏃剁敱鍏跺瓙浠诲姟鑷姩缁ф壙锛夊埌涓€涓?cpuset锛屼娇寰楀彲浠ュ皢绯荤粺涓婄殑宸ヤ綔璐熻浇缁勭粐鎴愮浉鍏崇殑浠诲姟闆嗗悎锛屾瘡涓泦鍚堣绾︽潫涓轰娇鐢ㄧ壒瀹?cpuset 鐨?CPU 鍜屽唴瀛樿妭鐐广€傚鏋滃繀瑕?cpuset 鏂囦欢绯荤粺鐩綍涓婄殑鏉冮檺鍏佽锛屼换鍔″彲浠ラ噸鏂伴檮鍔犲埌浠讳綍鍏朵粬 cpuset銆?
杩欑"澶ц寖鍥?鐨勭郴缁熺鐞嗕笌浣跨敤 sched_setaffinity銆乵bind 鍜?set_mempolicy 绯荤粺璋冪敤鍦ㄥ崟涓换鍔″拰鍐呭瓨鍖哄煙涓婂畬鎴愮殑璇︾粏鏀剧疆鐩搁泦鎴愩€?
浠ヤ笅瑙勫垯閫傜敤浜庢瘡涓?cpuset锛?
 - 瀹冪殑 CPU 鍜屽唴瀛樿妭鐐瑰繀椤绘槸鍏剁埗绾х殑瀛愰泦銆? - 闄ら潪鍏剁埗绾ф槸鐙崰鐨勶紝鍚﹀垯瀹冧笉鑳借鏍囪涓虹嫭鍗犮€? - 濡傛灉瀹冪殑 CPU 鎴栧唴瀛樻槸鐙崰鐨勶紝瀹冧滑涓嶅緱涓庝换浣曞厔寮熻妭鐐归噸鍙犮€?
杩欎簺瑙勫垯锛屼互鍙?cpusets 鐨勮嚜鐒跺眰娆＄粨鏋勶紝浣垮緱鑳藉楂樻晥瀹炴柦鐙崰淇濊瘉锛岃€屾棤闇€鍦ㄥ畠浠腑鐨勪换浣曚竴涓彂鐢熷彉鍖栨椂鎵弿鎵€鏈?cpusets 浠ョ‘淇濇病鏈変笢瑗夸笌鐙崰 cpuset 閲嶅彔銆傛澶栵紝浣跨敤 Linux 铏氭嫙鏂囦欢绯荤粺锛坴fs锛夋潵琛ㄧず cpuset 灞傛缁撴瀯锛屼负 cpusets 鎻愪緵浜嗕竴涓啛鎮夌殑鏉冮檺鍜屽懡鍚嶇┖闂达紝涓斿彧闇€鏈€灏戦噺鐨勯澶栧唴鏍镐唬鐮併€?
鏍癸紙top_cpuset锛塩puset 涓殑 cpus 鍜?mems 鏂囦欢鏄彧璇荤殑銆俢pus 鏂囦欢浣跨敤 CPU 鐑彃鎷旈€氱煡鍣ㄨ嚜鍔ㄨ窡韪?cpu_online_mask 鐨勫€硷紝mems 鏂囦欢浣跨敤 cpuset_track_online_nodes() 閽╁瓙鑷姩璺熻釜 node_states[N_MEMORY] 鐨勫€硷紙鍗冲甫鏈夊唴瀛樼殑鑺傜偣锛夈€?
cpuset.effective_cpus 鍜?cpuset.effective_mems 鏂囦欢閫氬父鏄?cpuset.cpus 鍜?cpuset.mems 鏂囦欢鐨勫彧璇诲壇鏈€傚鏋?cpuset cgroup 鏂囦欢绯荤粺浣跨敤鐗规畩鐨?"cpuset_v2_mode" 閫夐」鎸傝浇锛岃繖浜涙枃浠剁殑琛屼负灏嗗彉寰楃被浼间簬 cpuset v2 涓殑鐩稿簲鏂囦欢銆傛崲鍙ヨ瘽璇达紝鐑彃鎷斾簨浠朵笉浼氭敼鍙?cpuset.cpus 鍜?cpuset.mems銆傝繖浜涗簨浠跺彧浼氬奖鍝?cpuset.effective_cpus 鍜?cpuset.effective_mems锛屽畠浠樉绀烘 cpuset 褰撳墠瀹為檯浣跨敤鐨?CPU 鍜屽唴瀛樿妭鐐广€傛湁鍏?cpuset v2 琛屼负鐨勬洿澶氫俊鎭紝璇峰弬瑙?Documentation/admin-guide/cgroup-v2.rst銆?

### 1.4 浠€涔堟槸鐙崰 cpusets锛?

濡傛灉涓€涓?cpuset 鏄?CPU 鎴栧唴瀛樼嫭鍗犵殑锛岄偅涔堟病鏈夊叾浠?cpuset锛堢洿鎺ョ鍏堟垨鍚庝唬闄ゅ锛夊彲浠ュ叡浜换浣曠浉鍚岀殑 CPU 鎴栧唴瀛樿妭鐐广€?
涓€涓?cpuset.mem_exclusive **鎴?* cpuset.mem_hardwall 鐨?cpuset 鏄?纭闅旂锛坔ardwalled锛?鐨勶紝鍗冲畠闄愬埗鍐呮牳涓洪〉銆佺紦鍐插尯鍜屽叾浠栭€氬父琚唴鏍稿湪澶氫釜鐢ㄦ埛闂村叡浜殑鏁版嵁杩涜鐨勫垎閰嶃€傛墍鏈?cpusets锛屾棤璁烘槸鍚︾‖澧欓殧绂伙紝閮介檺鍒剁敤鎴风┖闂寸殑鍐呭瓨鍒嗛厤銆傝繖浣垮緱鍙互閰嶇疆涓€涓郴缁燂紝浣垮嚑涓嫭绔嬬殑浣滀笟鍙互鍏变韩鍏叡鐨勫唴鏍告暟鎹紝渚嬪鏂囦欢绯荤粺椤碉紝鍚屾椂灏嗘瘡涓綔涓氱殑鐢ㄦ埛鍒嗛厤闅旂鍦ㄥ畠鑷繁鐨?cpuset 涓€備负姝わ紝鏋勯€犱竴涓ぇ鐨?mem_exclusive cpuset 鏉ュ绾虫墍鏈変綔涓氾紝骞朵负姣忎釜鍗曠嫭浣滀笟鏋勯€犲瓙绾х殑銆侀潪 mem_exclusive 鐨?cpusets銆傚彧鏈夊皯閲忕殑鍏稿瀷鍐呮牳鍐呭瓨锛屼緥濡傛潵鑷腑鏂鐞嗙▼搴忕殑璇锋眰锛屾墠琚厑璁稿湪 mem_exclusive cpuset 涔嬪鑾峰彇銆?

### 1.5 浠€涔堟槸 memory_pressure锛?

cpuset 鐨?memory_pressure 鎻愪緵浜嗕竴涓畝鍗曠殑銆佹瘡 cpuset 鐨勫害閲忔寚鏍囷紝琛ㄧず cpuset 涓殑浠诲姟璇曞浘閲婃斁 cpuset 鑺傜偣涓婃鍦ㄤ娇鐢ㄧ殑鍐呭瓨浠ユ弧瓒抽澶栧唴瀛樿姹傜殑閫熷害銆?
杩欎娇寰楀湪涓撶敤 cpusets 涓洃鎺т綔涓氱殑鎵瑰鐞嗙鐞嗗櫒鑳藉楂樻晥鍦版娴嬭浣滀笟寮曡捣鐨勫唴瀛樺帇鍔涙按骞炽€?
杩欏杩欎袱绫绘儏鍐甸兘寰堟湁鐢細鍦ㄨ繍琛屽悇绉嶆彁浜や綔涓氱殑绱у瘑绠＄悊绯荤粺涓紝鍙兘浼氶€夋嫨缁堟鎴栭噸鏂颁紭鍏堟帓搴忛偅浜涜瘯鍥句娇鐢ㄨ秴杩囧垎閰嶇粰瀹冧滑鐨勮妭鐐逛笂鍏佽鍐呭瓨鐨勪綔涓氾紱浠ュ強鍦ㄧ揣瀵嗚€﹀悎銆侀暱鏃堕棿杩愯銆佸ぇ瑙勬ā骞惰鐨勭瀛﹁绠椾綔涓氫腑锛屽鏋滃畠浠紑濮嬩娇鐢ㄨ秴杩囧厑璁哥殑鍐呭瓨锛屽皢鍓х儓鍦版棤娉曡揪鍒版墍闇€鐨勬€ц兘鐩爣銆?
姝ゆ満鍒朵负鎵瑰鐞嗙鐞嗗櫒鎻愪緵浜嗕竴绉嶉潪甯哥粡娴庣殑鏂瑰紡鏉ョ洃鎺?cpuset 鐨勫唴瀛樺帇鍔涜抗璞°€傜敱鎵瑰鐞嗙鐞嗗櫒鎴栧叾浠栫敤鎴蜂唬鐮佹潵鍐冲畾濡備綍澶勭悊瀹冨苟閲囧彇琛屽姩銆?
==>
    闄ら潪閫氳繃灏?"1" 鍐欏叆鐗规畩鏂囦欢 /dev/cpuset/memory_pressure_enabled 鍚敤姝ょ壒鎬э紝鍚﹀垯 __alloc_pages() 鐨?rebalance 浠ｇ爜涓敤浜庢搴﹂噺鐨勯挬瀛愪細绠€鍖栦负绠€鍗曞湴娉ㄦ剰鍒?cpuset_memory_pressure_enabled 鏍囧織涓洪浂銆傚洜姝わ紝鍙湁鍚敤姝ょ壒鎬х殑绯荤粺鎵嶄細璁＄畻璇ュ害閲忋€?
涓轰綍浣跨敤姣?cpuset 鐨勮繍琛屽钩鍧囧€硷細

    鍥犱负姝や华琛ㄦ槸姣?cpuset 鐨勶紝鑰岄潪姣忎换鍔℃垨姣?mm锛屾墍浠ユ壒澶勭悊璋冨害鍣ㄧ洃鎺ф搴﹂噺瀵圭郴缁熸柦鍔犵殑璐熻浇鍦ㄥぇ鍨嬬郴缁熶笂鎬ュ墽闄嶄綆锛屽洜涓哄彲浠ラ伩鍏嶅湪姣忕粍鏌ヨ鏃舵壂鎻忎换鍔″垪琛ㄣ€?
    鍥犱负姝や华琛ㄦ槸杩愯骞冲潎鍊硷紝鑰岄潪绱Н璁℃暟鍣紝鎵瑰鐞嗚皟搴﹀櫒鍙互閫氳繃鍗曟璇诲彇妫€娴嬪唴瀛樺帇鍔涳紝鑰屼笉蹇呭湪涓€娈垫椂闂村唴璇诲彇鍜岀疮绉粨鏋溿€?
    鍥犱负姝や华琛ㄦ槸姣?cpuset 鐨勶紝鑰岄潪姣忎换鍔℃垨姣?mm锛屾壒澶勭悊璋冨害鍣ㄥ彲浠ラ€氳繃鍗曟璇诲彇鑾峰緱鍏抽敭淇℃伅锛坈puset 涓殑鍐呭瓨鍘嬪姏锛夛紝鑰屼笉蹇呮煡璇㈠苟绱Н cpuset 涓紙鍔ㄦ€佸彉鍖栫殑锛夋墍鏈変换鍔￠泦鍚堢殑缁撴灉銆?
涓€涓瘡 cpuset 鐨勭畝鍗曟暟瀛楁护娉㈠櫒锛堟瘡涓?cpuset 闇€瑕佷竴涓嚜鏃嬮攣鍜?3 涓瓧鐨勬暟鎹級琚淮鎶わ紝骞跺湪浠讳綍闄勫姞鍒拌 cpuset 鐨勪换鍔¤繘鍏ュ悓姝ワ紙鐩存帴锛夐〉鍥炴敹浠ｇ爜鏃舵洿鏂般€?
涓€涓瘡 cpuset 鐨勬枃浠舵彁渚涗竴涓暣鏁帮紝琛ㄧず杩戞湡锛堝崐琛版湡涓?10 绉掞級鐢?cpuset 涓换鍔″紩璧风殑鐩存帴椤靛洖鏀堕€熺巼锛屽崟浣嶄负姣忕灏濊瘯鍥炴敹娆℃暟锛屼箻浠?1000銆?

### 1.6 浠€涔堟槸 memory spread锛?

姣忎釜 cpuset 鏈変袱涓竷灏旀爣蹇楁枃浠讹紝鎺у埗鍐呮牳涓烘枃浠剁郴缁熺紦鍐插尯鍜岀浉鍏崇殑鍐呮牳鏁版嵁缁撴瀯鍒嗛厤椤电殑浣嶇疆銆傚畠浠О涓?'cpuset.memory_spread_page' 鍜?'cpuset.memory_spread_slab'銆?
濡傛灉璁剧疆浜嗘瘡 cpuset 甯冨皵鏍囧織鏂囦欢 'cpuset.memory_spread_page'锛岄偅涔堝唴鏍稿皢鎶婃枃浠剁郴缁熺紦鍐插尯锛堥〉缂撳瓨锛夊潎鍖€鍦板垎鏁ｅ湪鏁呴殰浠诲姟琚厑璁镐娇鐢ㄧ殑鎵€鏈夎妭鐐逛笂锛岃€屼笉鏄€惧悜浜庡皢杩欎簺椤垫斁鍦ㄤ换鍔¤繍琛岀殑鑺傜偣涓娿€?
濡傛灉璁剧疆浜嗘瘡 cpuset 甯冨皵鏍囧織鏂囦欢 'cpuset.memory_spread_slab'锛岄偅涔堝唴鏍稿皢鎶婁竴浜涙枃浠剁郴缁熺浉鍏崇殑 slab 缂撳瓨锛堜緥濡傜敤浜?inode 鍜?dentry锛夊潎鍖€鍦板垎鏁ｅ湪鏁呴殰浠诲姟琚厑璁镐娇鐢ㄧ殑鎵€鏈夎妭鐐逛笂锛岃€屼笉鏄€惧悜浜庡皢杩欎簺椤垫斁鍦ㄤ换鍔¤繍琛岀殑鑺傜偣涓娿€?
杩欎簺鏍囧織鐨勮缃笉褰卞搷浠诲姟鐨勫尶鍚嶆暟鎹鎴栨爤娈甸〉銆?
榛樿鎯呭喌涓嬶紝涓ょ鍐呭瓨鍒嗘暎閮芥槸鍏抽棴鐨勶紝鍐呭瓨椤靛垎閰嶅湪浠诲姟杩愯鐨勬湰鍦拌妭鐐逛笂锛岄櫎闈炲彲鑳借浠诲姟鐨?NUMA 鍐呭瓨绛栫暐鎴?cpuset 閰嶇疆淇敼锛屽彧瑕佹湁瓒冲鐨勭┖闂插唴瀛橀〉鍙敤銆?
褰撳垱寤烘柊鐨?cpusets 鏃讹紝瀹冧滑缁ф壙鍏剁埗绾х殑 memory spread 璁剧疆銆?
璁剧疆鍐呭瓨鍒嗘暎浼氬鑷村彈褰卞搷椤垫垨 slab 缂撳瓨鐨勫垎閰嶅拷鐣ヤ换鍔＄殑 NUMA 鍐呭瓨绛栫暐锛岃€岃鍒嗘暎銆備娇鐢?mbind() 鎴?set_mempolicy() 璋冪敤璁剧疆 NUMA 鍐呭瓨绛栫暐鐨勪换鍔′笉浼氭敞鎰忓埌杩欎簺璋冪敤鍥犲叾鍖呭惈浠诲姟鐨?memory spread 璁剧疆鑰屽彂鐢熶换浣曟敼鍙樸€傚鏋滃叧闂唴瀛樺垎鏁ｏ紝鍒欏綋鍓嶆寚瀹氱殑 NUMA 鍐呭瓨绛栫暐鍐嶆閫傜敤浜庡唴瀛橀〉鍒嗛厤銆?
'cpuset.memory_spread_page' 鍜?'cpuset.memory_spread_slab' 閮芥槸甯冨皵鏍囧織鏂囦欢銆傞粯璁ゆ儏鍐典笅瀹冧滑鍖呭惈 "0"锛屾剰鍛崇潃璇?cpuset 鐨勭壒鎬ф槸鍏抽棴鐨勩€傚鏋滃悜璇ユ枃浠跺啓鍏?"1"锛屽垯鎵撳紑鍛藉悕鐨勭壒鎬с€?
瀹炵幇寰堢畝鍗曘€?
璁剧疆鏍囧織 'cpuset.memory_spread_page' 浼氫负璇?cpuset 涓垨闅忓悗鍔犲叆璇?cpuset 鐨勬瘡涓换鍔℃墦寮€涓€涓瘡杩涚▼鏍囧織 PFA_SPREAD_PAGE銆備负椤电紦瀛樿繘琛岀殑椤靛垎閰嶈皟鐢ㄨ淇敼涓哄姝?PFA_SPREAD_PAGE 浠诲姟鏍囧織鎵ц鍐呰仈妫€鏌ワ紝濡傛灉璁剧疆浜嗭紝鍒欒皟鐢ㄦ柊渚嬬▼ cpuset_mem_spread_node() 杩斿洖鐢ㄤ簬鍒嗛厤鐨勫亸濂借妭鐐广€?
绫讳技鍦帮紝璁剧疆 'cpuset.memory_spread_slab' 浼氭墦寮€鏍囧織 PFA_SPREAD_SLAB锛屽苟涓旈€傚綋鏍囪鐨?slab 缂撳瓨灏嗕粠 cpuset_mem_spread_node() 杩斿洖鐨勮妭鐐瑰垎閰嶉〉銆?
cpuset_mem_spread_node() 渚嬬▼涔熷緢绠€鍗曘€傚畠浣跨敤姣忎换鍔¤浆瀛?cpuset_mem_spread_rotor 鐨勫€兼潵閫夋嫨褰撳墠浠诲姟 mems_allowed 涓殑涓嬩竴涓妭鐐逛綔涓哄垎閰嶅亸濂姐€?
杩欑鍐呭瓨鏀剧疆绛栫暐鍦ㄥ叾浠栦笂涓嬫枃涓篃绉颁负杞锛坮ound-robin锛夋垨浜ら敊锛坕nterleave锛夈€?
姝ょ瓥鐣ュ彲浠ヤ负闇€瑕佸皢绾跨▼鏈湴鏁版嵁鏀惧湪鐩稿簲鑺傜偣涓娿€佷絾闇€瑕佽闂ぇ鍨嬫枃浠剁郴缁熸暟鎹泦锛堣繖浜涙暟鎹泦闇€瑕佸垎鏁ｅ湪浣滀笟 cpuset 鐨勫嚑涓妭鐐逛笂鎵嶈兘鏀句笅锛夌殑浣滀笟甯︽潵瀹炶川鎬ф敼杩涖€傚鏋滄病鏈夋绛栫暐锛岀壒鍒槸瀵逛簬鍙兘鏈変竴涓嚎绋嬭鍙栨暟鎹泦鐨勪綔涓氾紝浣滀笟 cpuset 涓妭鐐归棿鐨勫唴瀛樺垎閰嶄細鍙樺緱闈炲父涓嶅潎鍖€銆?
### 1.7 浠€涔堟槸 sched_load_balance锛?

鍐呮牳璋冨害鍣紙kernel/sched/core.c锛夎嚜鍔ㄥ浠诲姟杩涜璐熻浇鍧囪　銆傚鏋滀竴涓?CPU 鍒╃敤鐜囦笉瓒筹紝杩愯鍦ㄨ CPU 涓婄殑鍐呮牳浠ｇ爜灏嗗鎵惧叾浠栨洿杩囪浇鐨?CPU 涓婄殑浠诲姟锛屽苟灏嗗叾绉诲姩鍒拌嚜宸辫繖閲岋紝鍙楄濡?cpusets 鍜?sched_setaffinity 绛夋斁缃満鍒剁殑绾︽潫銆?
璐熻浇鍧囪　鐨勭畻娉曟垚鏈強鍏跺浠诲姟鍒楄〃绛夊叧閿叡浜唴鏍告暟鎹粨鏋勭殑褰卞搷锛屼細闅忚鍧囪　 CPU 鏁伴噺鐨勫鍔犺€岃秴绾挎€у闀裤€傚洜姝よ皟搴﹀櫒鏀寔灏嗙郴缁熺殑 CPU 鍒掑垎涓鸿嫢骞茶皟搴﹀煙锛坰ched domains锛夛紝浣垮緱瀹冨彧鍦ㄦ瘡涓皟搴﹀煙鍐呭仛璐熻浇鍧囪　銆傛瘡涓皟搴﹀煙瑕嗙洊绯荤粺涓煇浜?CPU 瀛愰泦锛涗袱涓皟搴﹀煙涓嶉噸鍙狅紱鏌愪簺 CPU 鍙兘涓嶅湪浠讳綍璋冨害鍩熶腑锛屽洜姝や笉浼氳璐熻浇鍧囪　銆?
绠€鑰岃█涔嬶紝鍦ㄤ袱涓緝灏忕殑璋冨害鍩熶箣闂村仛鍧囪　姣斿湪涓€涓ぇ鐨勮皟搴﹀煙涓婂仛鍧囪　鎴愭湰鏇翠綆锛屼絾杩欐牱鍋氭剰鍛崇潃涓€涓煙涓殑杩囪浇涓嶄細琚礋杞藉潎琛″埌鍙︿竴涓煙銆?
榛樿鎯呭喌涓嬶紝鏈変竴涓鐩栨墍鏈?CPU 鐨勮皟搴﹀煙锛屽寘鎷偅浜涗娇鐢ㄥ唴鏍稿惎鍔ㄥ弬鏁?"isolcpus=" 鏍囪涓洪殧绂荤殑 CPU銆備絾鏄紝琚殧绂荤殑 CPU 涓嶄細鍙備笌璐熻浇鍧囪　锛屼篃涓嶄細鏈変换鍔¤繍琛屽湪瀹冧滑涓婇潰锛岄櫎闈炶鏄惧紡鍒嗛厤銆?
杩欑璺ㄦ墍鏈?CPU 鐨勯粯璁よ礋杞藉潎琛′笉閫傚悎浠ヤ笅涓ょ鎯呭喌锛?
 1) 鍦ㄥぇ鍨嬬郴缁熶笂锛岃法璁稿 CPU 鐨勮礋杞藉潎琛′唬浠烽珮鏄傘€傚鏋滅郴缁熶娇鐢?cpusets 绠＄悊浠ュ皢鐙珛浣滀笟鏀惧湪鐙珛鐨?CPU 闆嗗悎涓婏紝鍒欏畬鍏ㄨ礋杞藉潎琛℃槸涓嶅繀瑕佺殑銆? 2) 鍦ㄦ煇浜?CPU 涓婃敮鎸佸疄鏃讹紙realtime锛夌殑绯荤粺闇€瑕佹渶灏忓寲杩欎簺 CPU 涓婄殑绯荤粺寮€閿€锛屽寘鎷伩鍏嶄换鍔¤礋杞藉潎琛★紙濡傛灉涓嶉渶瑕佺殑璇濓級銆?
褰撴瘡 cpuset 鏍囧織 "cpuset.sched_load_balance" 琚惎鐢紙榛樿璁剧疆锛夋椂锛屽畠璇锋眰璇?cpuset 鍏佽鐨?'cpuset.cpus' 涓殑鎵€鏈?CPU 鍖呭惈鍦ㄤ竴涓崟涓€璋冨害鍩熶腑锛岀‘淇濊礋杞藉潎琛″彲浠ュ皢浠诲姟锛堟湭琚叾浠栨柟寮忓浐瀹氱殑锛屽 sched_setaffinity锛変粠璇?cpuset 涓殑浠讳綍 CPU 绉诲姩鍒颁换浣曞叾浠?CPU銆?
褰撴瘡 cpuset 鏍囧織 "cpuset.sched_load_balance" 琚鐢ㄦ椂锛岃皟搴﹀櫒灏嗛伩鍏嶅湪璇?cpuset 鍐呯殑 CPU 涔嬮棿鍋氳礋杞藉潎琛★紝鈥斺€旈櫎闈炩€斺€斿洜涓烘煇涓噸鍙犵殑 cpuset 鍚敤浜?"sched_load_balance" 鑰屽繀椤昏繖鏍峰仛銆?
鍥犳锛屼緥濡傦紝濡傛灉椤跺眰 cpuset 鍚敤浜嗘爣蹇?"cpuset.sched_load_balance"锛岄偅涔堣皟搴﹀櫒灏嗘湁涓€涓鐩栨墍鏈?CPU 鐨勫崟涓€璋冨害鍩燂紝浠讳綍鍏朵粬 cpusets 涓?"cpuset.sched_load_balance" 鏍囧織鐨勮缃兘鏃犲叧绱ц锛屽洜涓烘垜浠凡缁忓湪鍋氬畬鍏ㄨ礋杞藉潎琛′簡銆?
鍥犳锛屽湪涓婅堪涓ょ鎯呭喌涓嬶紝搴旂鐢ㄩ《灞?cpuset 鏍囧織 "cpuset.sched_load_balance"锛屽苟涓斿彧鏈変竴浜涜緝灏忕殑瀛?cpusets 鍚敤姝ゆ爣蹇椼€?
杩欐牱鍋氭椂锛屼綘閫氬父涓嶆兂鍦ㄩ《灞?cpuset 涓暀涓嬩换浣曞彲鑳借鍥哄畾锛坧inned锛夌殑浠诲姟锛岃繖浜涗换鍔″彲鑳戒細浣跨敤涓嶅彲蹇界暐閲忕殑 CPU锛屽洜涓烘绫讳换鍔″彲鑳借浜轰负绾︽潫鍒版煇浜?CPU 瀛愰泦锛屽彇鍐充簬鍚庝唬 cpusets 涓鏍囧織璁剧疆鐨勫叿浣撴儏鍐点€傚嵆浣挎绫讳换鍔″彲浠ヤ娇鐢ㄥ叾浠栦竴浜?CPU 涓殑绌洪棽 CPU 鍛ㄦ湡锛屽唴鏍歌皟搴﹀櫒涔熷彲鑳戒笉浼氳€冭檻灏嗚浠诲姟璐熻浇鍧囪　鍒伴偅涓湭鍏呭垎鍒╃敤鐨?CPU 涓婄殑鍙兘鎬с€?
褰撶劧锛岃鍥哄畾鍒扮壒瀹?CPU 鐨勪换鍔″彲浠ョ暀鍦ㄤ竴涓鐢?"cpuset.sched_load_balance" 鐨?cpuset 涓紝鍥犱负杩欎簺浠诲姟鏈潵涔熶笉浼氬幓浠讳綍鍏朵粬鍦版柟銆?
杩欓噷鍦?cpusets 鍜岃皟搴﹀煙涔嬮棿瀛樺湪闃绘姉澶遍厤锛坕mpedance mismatch锛夈€侰pusets 鏄眰娆″寲涓斿祵濂楃殑銆傝皟搴﹀煙鏄墎骞崇殑锛涘畠浠笉閲嶅彔锛屾瘡涓?CPU 鏈€澶氬湪涓€涓皟搴﹀煙涓€?
璋冨害鍩熷繀椤绘槸鎵佸钩鐨勶紝鍥犱负瀵归儴鍒嗛噸鍙犵殑 CPU 闆嗗悎鍋氳礋杞藉潎琛′細甯︽潵瓒呭嚭鎴戜滑鐞嗚В鐨勩€佷笉绋冲畾鐨勫姩鎬併€傚洜姝わ紝濡傛灉涓や釜閮ㄥ垎閲嶅彔鐨?cpusets 閮藉惎鐢ㄤ簡鏍囧織 'cpuset.sched_load_balance'锛岄偅涔堟垜浠細褰㈡垚涓€涓寘鍚袱涓殑瓒呴泦鐨勫崟涓€璋冨害鍩熴€傛垜浠笉浼氬皢浠诲姟绉诲姩鍒板叾 cpuset 涔嬪鐨?CPU锛屼絾璋冨害鍣ㄨ礋杞藉潎琛′唬鐮佸彲鑳戒細娴垂涓€浜涜绠楀懆鏈熸潵鑰冭檻杩欑鍙兘鎬с€?
杩欑澶遍厤灏辨槸涓轰粈涔堝摢浜?cpusets 鍚敤浜嗘爣蹇?"cpuset.sched_load_balance" 涓庤皟搴﹀煙閰嶇疆涔嬮棿娌℃湁绠€鍗曠殑涓€涓€瀵瑰簲鍏崇郴鐨勫師鍥犮€傚鏋滀竴涓?cpuset 鍚敤浜嗚鏍囧織锛屽畠灏嗚幏寰楀叾鎵€鏈?CPU 涓婄殑鍧囪　锛屼絾濡傛灉瀹冪鐢ㄤ簡璇ユ爣蹇楋紝鍒欏彧鏈夊湪娌℃湁鍏朵粬閲嶅彔 cpuset 鍚敤璇ユ爣蹇楁椂锛屾墠鑳戒繚璇佹病鏈夎礋杞藉潎琛°€?
濡傛灉涓や釜 cpusets 鐨?'cpuset.cpus' 鍏佽闆嗗悎閮ㄥ垎閲嶅彔锛屼笖鍙湁鍏朵腑涓€涓惎鐢ㄤ簡璇ユ爣蹇楋紝閭ｄ箞鍙︿竴涓彲鑳戒細鍙戠幇鍏朵换鍔′粎鍦ㄩ噸鍙犵殑 CPU 涓婅閮ㄥ垎璐熻浇鍧囪　銆傝繖鍙槸鍓嶉潰鍑犳缁欏嚭鐨勯《灞?cpuset 绀轰緥鐨勪竴鑸儏鍐点€傚湪涓€鑸儏鍐典笅锛屽鍚岄《灞?cpuset 鎯呭喌涓€鏍凤紝涓嶈灏嗗彲鑳戒娇鐢ㄤ笉鍙拷鐣ラ噺 CPU 鐨勪换鍔＄暀鍦ㄨ繖鏍风殑閮ㄥ垎璐熻浇鍧囪　 cpusets 涓紝鍥犱负瀹冧滑鍙兘琚汉涓虹害鏉熷埌鍏佽缁欏畠浠殑鏌愪簺 CPU 瀛愰泦锛屽洜涓虹己涔忓埌鍏朵粬 CPU 鐨勮礋杞藉潎琛°€?
"cpuset.isolcpus" 涓殑 CPU 琚?isolcpus= 鍐呮牳鍚姩閫夐」鎺掗櫎鍦ㄨ礋杞藉潎琛′箣澶栵紝骞朵笖鏃犺浠讳綍 cpuset 涓?"cpuset.sched_load_balance" 鐨勫€煎浣曪紝閮芥案杩滀笉浼氳璐熻浇鍧囪　銆?
### 1.7.1 sched_load_balance 瀹炵幇缁嗚妭銆?

姣?cpuset 鏍囧織 'cpuset.sched_load_balance' 榛樿鍚敤锛堜笌澶у鏁?cpuset 鏍囧織鐩稿弽锛夈€傚綋涓烘煇涓?cpuset 鍚敤鏃讹紝鍐呮牳灏嗙‘淇濆彲浠ュ湪璇?cpuset 鐨勬墍鏈?CPU 涓婂仛璐熻浇鍧囪　锛堢‘淇濊 cpuset 鐨?cpus_allowed 涓殑鎵€鏈?CPU 閮藉湪鍚屼竴涓皟搴﹀煙涓級銆?
濡傛灉涓や釜閲嶅彔鐨?cpusets 閮藉惎鐢ㄤ簡 'cpuset.sched_load_balance'锛岄偅涔堝畠浠皢锛堝繀椤伙級閮藉湪鍚屼竴涓皟搴﹀煙涓€?
濡傛灉濡傞粯璁ゆ儏鍐碉紝椤跺眰 cpuset 鍚敤浜?'cpuset.sched_load_balance'锛岄偅涔堟牴鎹笂杩帮紝鎰忓懗鐫€瀛樺湪涓€涓鐩栨暣涓郴缁熺殑鍗曚竴璋冨害鍩燂紝鏃犺浠讳綍鍏朵粬 cpuset 璁剧疆濡備綍銆?
鍐呮牳鍚戠敤鎴风┖闂存壙璇哄畠灏嗗敖鍙兘閬垮厤璐熻浇鍧囪　銆傚畠浼氶€夋嫨灏藉彲鑳界粏绮掑害鐨勮皟搴﹀煙鍒掑垎锛屽悓鏃朵粛涓轰换浣曞厑璁镐簡 'cpuset.sched_load_balance' 鐨?cpuset 鐨?CPU 闆嗗悎鎻愪緵璐熻浇鍧囪　銆?
鍐呮牳鍐呴儴 cpuset 鍒拌皟搴﹀櫒鐨勬帴鍙ｏ紝浠?cpuset 浠ｇ爜鍚戣皟搴﹀櫒浠ｇ爜浼犻€掔郴缁熶腑璐熻浇鍧囪　 CPU 鐨勪竴涓垝鍒嗭紙partition锛夈€傛鍒掑垎鏄竴缁勫瓙闆嗭紙琛ㄧず涓?struct cpumask 鏁扮粍锛夛紝涓や袱涓嶇浉浜わ紝瑕嗙洊鎵€鏈夊繀椤昏璐熻浇鍧囪　鐨?CPU銆?
cpuset 浠ｇ爜鏋勫缓涓€涓柊鐨勬绫诲垝鍒嗗苟浼犻€掔粰璋冨害鍣ㄨ皟搴﹀煙寤虹珛浠ｇ爜锛屼互鍦ㄥ繀瑕佹椂閲嶅缓璋冨害鍩燂紝鍙鍙戠敓浠ヤ笅鎯呭喌锛?
 - 鍏锋湁闈炵┖ CPU 鐨?cpuset 鐨?'cpuset.sched_load_balance' 鏍囧織鍙戠敓鍙樺寲锛? - 鎴?CPU 浠庡惎鐢ㄤ簡姝ゆ爣蹇楃殑 cpuset 涓姞鍏ユ垨绉婚櫎锛? - 鎴栧叿鏈夐潪绌?CPU 涓斿惎鐢ㄤ簡姝ゆ爣蹇楃殑 cpuset 鐨?'cpuset.sched_relax_domain_level' 鍊煎彂鐢熷彉鍖栵紝
 - 鎴栫Щ闄や簡涓€涓叿鏈夐潪绌?CPU 涓斿惎鐢ㄤ簡姝ゆ爣蹇楃殑 cpuset锛? - 鎴栨煇涓?CPU 琚笅绾?涓婄嚎銆?
姝ゅ垝鍒嗙簿纭湴瀹氫箟浜嗚皟搴﹀櫒搴斿缓绔嬪摢浜涜皟搴﹀煙鈥斺€斿垝鍒嗕腑鐨勬瘡涓厓绱狅紙struct cpumask锛夊搴斾竴涓皟搴﹀煙銆?
璋冨害鍣ㄨ浣忓綋鍓嶆椿鍔ㄧ殑璋冨害鍩熷垝鍒嗐€傚綋璋冨害鍣ㄤ緥绋?partition_sched_domains() 浠?cpuset 浠ｇ爜琚皟鐢ㄦ潵鏇存柊杩欎簺璋冨害鍩熸椂锛屽畠浼氬皢璇锋眰鐨勬柊鍒掑垎涓庡綋鍓嶅垝鍒嗚繘琛屾瘮杈冿紝骞朵负姣忎釜鍙樻洿鏇存柊鍏惰皟搴﹀煙锛岀Щ闄ゆ棫鐨勫苟娣诲姞鏂扮殑銆?

### 1.8 浠€涔堟槸 sched_relax_domain_level锛?

鍦ㄨ皟搴﹀煙涓紝璋冨害鍣ㄤ互涓ょ鏂瑰紡杩佺Щ浠诲姟锛歵ick 涓婄殑鍛ㄦ湡鎬ц礋杞藉潎琛★紝浠ュ強鍦ㄦ煇浜涜皟搴︿簨浠跺彂鐢熸椂銆?
褰撲换鍔¤鍞ら啋鏃讹紝璋冨害鍣ㄥ皾璇曞皢浠诲姟绉诲姩鍒扮┖闂?CPU 涓娿€備緥濡傦紝濡傛灉杩愯鍦?CPU X 涓婄殑浠诲姟 A 婵€娲讳簡鍚屼竴 CPU X 涓婄殑鍙︿竴涓换鍔?B锛屽苟涓斿鏋?CPU Y 鏄?X 鐨勫厔寮熶笖澶勪簬绌洪棽锛岄偅涔堣皟搴﹀櫒灏嗕换鍔?B 杩佺Щ鍒?CPU Y锛屼互渚夸换鍔?B 鍙互鍦?CPU Y 涓婂惎鍔ㄨ€屾棤闇€绛夊緟 CPU X 涓婄殑浠诲姟 A銆?
鑰屽鏋滀竴涓?CPU 鐨?runqueue 涓病鏈変换鍔′簡锛岃 CPU 浼氬湪鑷繁鍗冲皢绌洪棽涔嬪墠锛屽皾璇曚粠鍏朵粬绻佸繖 CPU 鎷夊彇棰濆浠诲姟鏉ュ府鍔╁畠浠€?
褰撶劧锛屾煡鎵惧彲绉诲姩浠诲姟鍜?鎴栫┖闂?CPU 闇€瑕佷竴瀹氱殑鎼滅储鎴愭湰锛岃皟搴﹀櫒鍙兘涓嶄細姣忔閮芥悳绱㈠煙涓殑鎵€鏈?CPU銆傚疄闄呬笂锛屽湪鏌愪簺鏋舵瀯涓婏紝浜嬩欢鏃剁殑鎼滅储鑼冨洿琚檺鍒跺湪璇?CPU 鎵€鍦ㄧ殑鍚屼竴鎻掓Ы鎴栬妭鐐瑰唴锛岃€?tick 涓婄殑璐熻浇鍧囪　鎼滅储鎵€鏈?CPU銆?
渚嬪锛屽亣璁?CPU Z 涓?CPU X 鐩稿杈冭繙銆傚嵆浣?CPU Z 绌洪棽鑰?CPU X 鍙婂叾鍏勫紵閮界箒蹇欙紝璋冨害鍣ㄤ篃鏃犳硶灏嗗敜閱掔殑浠诲姟 B 浠?X 杩佺Щ鍒?Z锛屽洜涓哄畠瓒呭嚭浜嗘悳绱㈣寖鍥淬€傜粨鏋滐紝CPU X 涓婄殑浠诲姟 B 闇€瑕佺瓑寰呬换鍔?A 鎴栫瓑寰呬笅涓€涓?tick 涓婄殑璐熻浇鍧囪　銆傚浜庢煇浜涚壒娈婃儏鍐电殑鏌愪簺搴旂敤锛岀瓑寰?1 涓?tick 鍙兘澶暱銆?
'cpuset.sched_relax_domain_level' 鏂囦欢鍏佽浣犳寜闇€璇锋眰鏇存敼姝ゆ悳绱㈣寖鍥淬€傛鏂囦欢鎺ュ彈涓€涓?int 鍊硷紝澶ц嚧鎸夊涓嬬骇鍒寚绀烘悳绱㈣寖鍥寸殑澶у皬锛屽惁鍒欏垵濮嬪€间负 -1锛岃〃绀鸿 cpuset 娌℃湁璇锋眰銆?
====== ===========================================================
  -1   鏃犺姹傘€備娇鐢ㄧ郴缁熼粯璁ゆ垨閬靛惊鍏朵粬浜虹殑璇锋眰銆?   0   涓嶆悳绱€?   1   鎼滅储鍏勫紵锛堟牳蹇冧腑鐨勮秴绾跨▼锛夈€?   2   鎼滅储灏佽锛坧ackage锛変腑鐨勬牳蹇冦€?   3   鎼滅储鑺傜偣涓殑 cpu [鍦ㄩ潪 NUMA 绯荤粺涓?= 绯荤粺鑼冨洿]
   4   鎼滅储鑺傜偣鍧楋紙chunk of node锛変腑鐨勮妭鐐?[鍦?NUMA 绯荤粺涓奭
   5   鎼滅储绯荤粺鑼冨洿 [鍦?NUMA 绯荤粺涓奭
====== ===========================================================

骞堕潪鎵€鏈夌骇鍒兘鍙兘瀛樺湪锛屽苟涓斿€煎彲鑳芥牴鎹郴缁熸灦鏋勫拰鍐呮牳閰嶇疆鑰屽彉鍖栥€傝鏌ョ湅 /sys/kernel/debug/sched/domains/cpu**/domain**/ 浜嗚В绯荤粺鐗瑰畾鐨勮缁嗕俊鎭€?
绯荤粺榛樿鍊间緷璧栦簬鏋舵瀯銆傜郴缁熼粯璁ゅ€煎彲浠ヤ娇鐢?relax_domain_level= 鍚姩鍙傛暟鏇存敼銆?
姝ゆ枃浠舵槸姣?cpuset 鐨勶紝骞跺奖鍝嶅叾鎵€灞?cpuset 鐨勮皟搴﹀煙銆傚洜姝わ紝濡傛灉涓€涓?cpuset 鐨勬爣蹇?'cpuset.sched_load_balance' 琚鐢紝閭ｄ箞 'cpuset.sched_relax_domain_level' 娌℃湁鏁堟灉锛屽洜涓轰笉瀛樺湪灞炰簬璇?cpuset 鐨勮皟搴﹀煙銆?
濡傛灉澶氫釜 cpusets 閲嶅彔骞跺洜姝ゅ舰鎴愬崟涓€璋冨害鍩燂紝鍒欎娇鐢ㄥ叾涓殑鏈€澶у€笺€傛敞鎰忥紝濡傛灉涓€涓姹?0 鑰屽叾浠栦负 -1锛屽垯浣跨敤 0銆?
娉ㄦ剰淇敼姝ゆ枃浠朵細鏈夊ソ鏈夊潖鐨勫奖鍝嶏紝鏄惁鍙帴鍙楀彇鍐充簬浣犵殑鎯呭喌銆傚鏋滀綘涓嶇‘瀹氾紝涓嶈淇敼姝ゆ枃浠躲€?
濡傛灉浣犵殑鎯呭喌鏄細

 - 鐢变簬浣犵殑鐗规畩搴旂敤琛屼负鎴?CPU 缂撳瓨绛夌壒娈婄殑纭欢鏀寔锛屾瘡涓?cpu 涔嬮棿鐨勮縼绉绘垚鏈浣犲彲浠ュ亣瀹氱浉褰撳皬銆? - 鎼滅储鎴愭湰瀵逛綘娌℃湁褰卞搷锛屾垨鑰呬綘鍙互閫氳繃绠＄悊 cpuset 浣垮叾绱у噾绛夋潵浣挎悳绱㈡垚鏈冻澶熷皬銆? - 鍗充娇鐗虹壊缂撳瓨鍛戒腑鐜囩瓑涔熼渶瑕佷綆寤惰繜銆?   閭ｄ箞澧炲姞 'sched_relax_domain_level' 浼氬浣犳湁鐩娿€?

### 1.9 鎴戝浣曚娇鐢?cpusets锛?

涓轰簡鏈€灏忓寲 cpusets 瀵瑰叧閿唴鏍镐唬鐮侊紙濡傝皟搴﹀櫒锛夌殑褰卞搷锛屽苟涓旂敱浜庡唴鏍镐笉鏀寔涓€涓换鍔＄洿鎺ユ洿鏂板彟涓€涓换鍔＄殑鍐呭瓨鏀剧疆锛屾洿鏀逛换鍔＄殑 cpuset CPU 鎴栧唴瀛樿妭鐐规斁缃紝鎴栨洿鏀逛换鍔￠檮鍔犲埌鐨?cpuset锛屽浠诲姟鐨勫奖鍝嶆槸寰鐨勩€?
濡傛灉鏌愪釜 cpuset 鐨勫唴瀛樿妭鐐硅淇敼锛岄偅涔堝浜庨檮鍔犲埌璇?cpuset 鐨勬瘡涓换鍔★紝涓嬩竴娆″唴鏍稿皾璇曚负璇ヤ换鍔″垎閰嶅唴瀛橀〉鏃讹紝鍐呮牳浼氭敞鎰忓埌浠诲姟 cpuset 鐨勫彉鍖栵紝骞舵洿鏂板叾姣忎换鍔″唴瀛樻斁缃互淇濇寔鍦ㄦ柊鐨?cpuset 鍐呭瓨鏀剧疆鑼冨洿鍐呫€傚鏋滀换鍔℃鍦ㄤ娇鐢?mempolicy MPOL_BIND锛屽苟涓斿畠鎵€缁戝畾鐨勮妭鐐逛笌鍏舵柊鐨?cpuset 閲嶅彔锛岄偅涔堜换鍔″皢缁х画浣跨敤鏂扮殑 cpuset 涓粛鐒跺厑璁哥殑 MPOL_BIND 鑺傜偣鐨勪换浣曞瓙闆嗐€傚鏋滀换鍔℃鍦ㄤ娇鐢?MPOL_BIND锛岃€岀幇鍦ㄥ叾 MPOL_BIND 鑺傜偣閮戒笉鍦ㄦ柊鐨?cpuset 涓鍏佽锛岄偅涔堜换鍔″皢鍩烘湰涓婅瑙嗕负缁戝畾鍒版柊鐨?cpuset 鐨?MPOL_BIND锛堝嵆浣垮叾閫氳繃 get_mempolicy() 鏌ヨ鐨?NUMA 鏀剧疆娌℃湁鏀瑰彉锛夈€傚鏋滀竴涓换鍔′粠涓€涓?cpuset 绉诲姩鍒板彟涓€涓?cpuset锛岄偅涔堝唴鏍镐細鍦ㄤ笅涓€娆″皾璇曚负璇ヤ换鍔″垎閰嶅唴瀛橀〉鏃讹紝濡備笂璋冩暣璇ヤ换鍔＄殑鍐呭瓨鏀剧疆銆?
濡傛灉鏌愪釜 cpuset 鐨?'cpuset.cpus' 琚慨鏀癸紝閭ｄ箞璇?cpuset 涓殑姣忎釜浠诲姟鐨勫厑璁?CPU 鏀剧疆灏嗙珛鍗虫敼鍙樸€傜被浼煎湴锛屽鏋滀竴涓换鍔＄殑 pid 琚啓鍏ュ彟涓€涓?cpuset 鐨?'tasks' 鏂囦欢锛岄偅涔堝畠鐨勫厑璁?CPU 鏀剧疆涔熶細绔嬪嵆鏀瑰彉銆傚鏋滆繖鏍风殑浠诲姟涔嬪墠浣跨敤 sched_setaffinity() 璋冪敤琚粦瀹氬埌鍏?cpuset 鐨勬煇涓瓙闆嗭紝閭ｄ箞璇ヤ换鍔″皢琚厑璁稿湪鍏舵柊 cpuset 涓厑璁哥殑浠讳綍 CPU 涓婅繍琛岋紝浠庤€屾姷娑堝厛鍓?sched_setaffinity() 璋冪敤鐨勬晥鏋溿€?
鎬讳箣锛宑puset 琚洿鏀圭殑浠诲姟鐨勫唴瀛樻斁缃敱鍐呮牳鍦ㄨ浠诲姟涓嬩竴娆″垎閰嶉〉鏃舵洿鏂帮紝鑰屽鐞嗗櫒鏀剧疆浼氱珛鍗虫洿鏂般€?
閫氬父锛屼竴鏃﹀垎閰嶄簡涓€涓〉锛堣幏寰椾簡涓诲瓨鐨勪竴涓墿鐞嗛〉锛夛紝閭ｄ箞璇ラ〉灏变細鐣欏湪瀹冭鍒嗛厤鐨勮妭鐐逛笂锛屽彧瑕佸畠淇濇寔鍒嗛厤鐘舵€侊紝鍗充娇 cpusets 鍐呭瓨鏀剧疆绛栫暐 'cpuset.mems' 闅忓悗鏀瑰彉銆傚鏋?cpuset 鏍囧織鏂囦欢 'cpuset.memory_migrate' 琚涓?true锛岄偅涔堝綋浠诲姟闄勫姞鍒拌 cpuset 鏃讹紝璇ヤ换鍔″湪鍏跺厛鍓?cpuset 鐨勮妭鐐逛笂鍒嗛厤缁欏畠鐨勪换浣曢〉閮戒細琚縼绉诲埌浠诲姟鐨勬柊 cpuset銆傚湪杩欎簺杩佺Щ鎿嶄綔涓細灏藉彲鑳戒繚鐣欓〉鍦?cpuset 鍐呯殑鐩稿鏀剧疆銆備緥濡傦紝濡傛灉椤靛湪鍏堝墠 cpuset 鐨勭浜屼釜鏈夋晥鑺傜偣涓婏紝閭ｄ箞椤靛皢琚斁鍦ㄦ柊 cpuset 鐨勭浜屼釜鏈夋晥鑺傜偣涓娿€?
鍚屾牱锛屽鏋?'cpuset.memory_migrate' 琚涓?true锛岄偅涔堝鏋滆 cpuset 鐨?'cpuset.mems' 鏂囦欢琚慨鏀癸紝鍒嗛厤缁欒 cpuset 涓换鍔＄殑銆佷綅浜庡厛鍓?'cpuset.mems' 璁剧疆鑺傜偣涓婄殑椤碉紝灏嗚绉诲姩鍒?'mems' 鏂拌缃腑鐨勮妭鐐逛笂銆備笉鍦ㄤ换鍔″厛鍓?cpuset 涓€佹垨涓嶅湪 cpuset 鍏堝墠 'cpuset.mems' 璁剧疆涓殑椤典笉浼氳绉诲姩銆?
涓婅堪鏈変竴涓緥澶栥€傚鏋滀娇鐢ㄤ簡鐑彃鎷斿姛鑳芥潵绉婚櫎褰撳墠鍒嗛厤缁欐煇涓?cpuset 鐨勬墍鏈?CPU锛岄偅涔堣 cpuset 涓殑鎵€鏈変换鍔″皢琚Щ鍔ㄥ埌鍏锋湁闈炵┖ CPU 鐨勬渶杩戠鍏堛€備絾鏄紝濡傛灉 cpuset 涓庡彟涓€涓叿鏈夋煇浜涗换鍔￠檮鍔犻檺鍒剁殑 cgroup 瀛愮郴缁熺粦瀹氾紝鏌愪簺锛堟垨鍏ㄩ儴锛変换鍔＄殑绉诲姩鍙兘浼氬け璐ャ€傚湪杩欑澶辫触鎯呭喌涓嬶紝閭ｄ簺浠诲姟灏嗙暀鍦ㄥ師濮?cpuset 涓紝鍐呮牳浼氳嚜鍔ㄦ洿鏂板畠浠殑 cpus_allowed 浠ュ厑璁告墍鏈夊湪绾?CPU銆傚綋鐢ㄤ簬绉婚櫎鍐呭瓨鑺傜偣鐨勫唴瀛樼儹鎻掓嫈鍔熻兘鍙敤鏃讹紝棰勬湡閭ｉ噷涔熼€傜敤绫讳技鐨勪緥澶栥€備竴鑸潵璇达紝鍐呮牳鍊惧悜浜庤繚鍙?cpuset 鏀剧疆锛岃€屼笉鏄涓€涓换鍔＄殑鎵€鏈夊厑璁?CPU 鎴栧唴瀛樿妭鐐归兘绂荤嚎鑰屽鑷村叾楗挎銆?
涓婅堪杩樻湁绗簩涓緥澶栥€侴FP_ATOMIC 璇锋眰鏄繀椤荤珛鍗虫弧瓒崇殑鍐呮牳鍐呴儴鍒嗛厤銆傚鏋?GFP_ATOMIC 鍒嗛厤澶辫触锛屽唴鏍稿彲鑳戒細涓㈠純鏌愪簺璇锋眰锛屽湪鏋佸皯鏁版儏鍐典笅鐢氳嚦浼氬穿婧冦€傚鏋滆姹傛棤娉曞湪褰撳墠浠诲姟鐨?cpuset 鍐呮弧瓒筹紝閭ｄ箞鎴戜滑浼氭斁瀹?cpuset锛屽苟鍦ㄤ换浣曡兘鎵惧埌鐨勫湴鏂瑰鎵惧唴瀛樸€傝繚鍙?cpuset 涔熸瘮缁欏唴鏍告柦鍔犲帇鍔涜濂姐€?
瑕佸惎鍔ㄤ竴涓寘鍚湪鏂?cpuset 涓殑鏂颁綔涓氾紝姝ラ濡備笅锛?
 1) mkdir /sys/fs/cgroup/cpuset
 2) mount -t cgroup -ocpuset cpuset /sys/fs/cgroup/cpuset
 3) 閫氳繃鍦?/sys/fs/cgroup/cpuset 铏氭嫙鏂囦欢绯荤粺涓墽琛?mkdir 鍜?write锛堟垨 echo锛夋潵鍒涘缓鏂扮殑 cpuset銆? 4) 鍚姩涓€涓皢鎴愪负鏂颁綔涓?鍒涘鐖惰繘绋嬶紙founding father锛?鐨勪换鍔°€? 5) 閫氳繃灏嗗叾 pid 鍐欏叆璇?cpuset 鐨?/sys/fs/cgroup/cpuset tasks 鏂囦欢锛屽皢璇ヤ换鍔￠檮鍔犲埌鏂?cpuset銆? 6) 浠庢鍒涘鐖朵换鍔?fork銆乪xec 鎴?clone 浣滀笟浠诲姟銆?
渚嬪锛屼互涓嬪懡浠ゅ簭鍒楀皢寤虹珛涓€涓悕涓?"Charlie" 鐨?cpuset锛屼粎鍖呭惈 CPU 2 鍜?3锛屼互鍙婂唴瀛樿妭鐐?1锛?```

  mount -t cgroup -ocpuset cpuset /sys/fs/cgroup/cpuset
  cd /sys/fs/cgroup/cpuset
  mkdir Charlie
  cd Charlie
  /bin/echo 2-3 > cpuset.cpus
  /bin/echo 1 > cpuset.mems
  /bin/echo $$ > tasks
  sh
  # 瀛?shell 'sh' 鐜板湪杩愯鍦?cpuset Charlie 涓?  # 涓嬩竴琛屽簲鏄剧ず '/Charlie'
  cat /proc/self/cpuset

```
鏈夊嚑绉嶆煡璇㈡垨淇敼 cpusets 鐨勬柟寮忥細

 - 鐩存帴閫氳繃 cpuset 鏂囦欢绯荤粺锛屼娇鐢?shell 涓殑鍚勭 cd銆乵kdir銆乪cho銆乧at銆乺mdir 鍛戒护锛屾垨瀹冧滑鍦?C 涓殑绛変环鐗┿€? - 閫氳繃 C 搴?libcpuset銆? - 閫氳繃 C 搴?libcgroup銆?   (https://github.com/libcgroup/libcgroup/)
 - 閫氳繃 python 搴旂敤 cset銆?   (http://code.google.com/p/cpuset/)

sched_setaffinity 璋冪敤涔熷彲浠ュ湪 shell 鎻愮ず绗︿笅浣跨敤 SGI 鐨?runon 鎴?Robert Love 鐨?taskset 瀹屾垚銆俶bind 鍜?set_mempolicy 璋冪敤鍙互鍦?shell 鎻愮ず绗︿笅浣跨敤 numactl 鍛戒护锛圓ndi Kleen 鐨?numa 鍖呯殑涓€閮ㄥ垎锛夊畬鎴愩€?
## 2. 浣跨敤绀轰緥涓庤娉?

### 2.1 鍩烘湰鐢ㄦ硶


鍒涘缓銆佷慨鏀广€佷娇鐢?cpusets 鍙互閫氳繃 cpuset 铏氭嫙鏂囦欢绯荤粺瀹屾垚銆?
瑕佹寕杞藉畠锛岃緭鍏ワ細
# mount -t cgroup -o cpuset cpuset /sys/fs/cgroup/cpuset

鐒跺悗鍦?/sys/fs/cgroup/cpuset 涓嬩綘鍙互鎵惧埌涓€涓搴斾簬绯荤粺涓?cpusets 鏍戠殑鏍戙€備緥濡傦紝/sys/fs/cgroup/cpuset 鏄寔鏈夋暣涓郴缁熺殑 cpuset銆?
```

  # cd /sys/fs/cgroup/cpuset
  # mkdir my_cpuset

```
```

  # cd my_cpuset

```
```

  # ls
  cgroup.clone_children  cpuset.memory_pressure
  cgroup.event_control   cpuset.memory_spread_page
  cgroup.procs           cpuset.memory_spread_slab
  cpuset.cpu_exclusive   cpuset.mems
  cpuset.cpus            cpuset.sched_load_balance
  cpuset.mem_exclusive   cpuset.sched_relax_domain_level
  cpuset.mem_hardwall    notify_on_release
  cpuset.memory_migrate  tasks

```
璇诲彇瀹冧滑浼氱粰浣犲叧浜庢 cpuset 鐘舵€佺殑淇℃伅锛氬畠鍙互浣跨敤鐨?CPU 鍜屽唴瀛樿妭鐐广€佹鍦ㄤ娇鐢ㄥ畠鐨勮繘绋嬨€佸畠鐨勫睘鎬с€傞€氳繃鍐欏叆杩欎簺鏂囦欢浣犲彲浠ユ搷绾佃 cpuset銆?
```

  # /bin/echo 1 > cpuset.cpu_exclusive

```
```

  # /bin/echo 0-7 > cpuset.cpus

```
```

  # /bin/echo 0-7 > cpuset.mems

```
```

  # /bin/echo $$ > tasks

```
浣犺繕鍙互閫氳繃鍦ㄦ澶勪娇鐢?mkdir 鍦ㄤ綘鐨?cpuset 鍐呭垱寤?cpusets
```

  # mkdir my_sub_cs

```
```

  # rmdir my_sub_cs

```
濡傛灉 cpuset 姝ｅ湪浣跨敤涓紙鍐呴儴鏈?cpusets锛屾垨闄勫姞浜嗚繘绋嬶級锛岃繖灏嗗け璐ャ€?
娉ㄦ剰锛屽嚭浜庨仐鐣欏師鍥狅紝"cpuset" 鏂囦欢绯荤粺浣滀负 cgroup 鏂囦欢绯荤粺鐨勫皝瑁呭瓨鍦ㄣ€?```

  mount -t cpuset X /sys/fs/cgroup/cpuset

```
```

  mount -t cgroup -ocpuset,noprefix X /sys/fs/cgroup/cpuset
  echo "/sbin/cpuset_release_agent" > /sys/fs/cgroup/cpuset/release_agent

```
### 2.2 娣诲姞/绉婚櫎 cpus


杩欐槸鍦?cpus 鎴?mems 鏂囦欢涓啓鍏ユ椂浣跨敤鐨勮娉?```

  # /bin/echo 1-4 > cpuset.cpus		-> 灏?cpus 鍒楄〃璁剧疆涓?cpus 1,2,3,4
  # /bin/echo 1,2,3,4 > cpuset.cpus	-> 灏?cpus 鍒楄〃璁剧疆涓?cpus 1,2,3,4

```
瑕佹坊鍔?CPU 鍒?cpuset锛屽啓鍏ュ寘鍚 CPU 鐨勬柊 CPU 鍒楄〃
```

  # /bin/echo 1-4,6 > cpuset.cpus	-> 灏?cpus 鍒楄〃璁剧疆涓?cpus 1,2,3,4,6

```
绫讳技鍦帮紝瑕佷粠 cpuset 涓Щ闄?CPU锛屽啓鍏ヤ笉鍖呭惈瑕佺Щ闄?CPU 鐨勬柊 CPU 鍒楄〃銆?```

  # /bin/echo "" > cpuset.cpus		-> 娓呯┖ cpus 鍒楄〃

```
### 2.3 璁剧疆鏍囧織


```

  # /bin/echo 1 > cpuset.cpu_exclusive 	-> 璁剧疆鏍囧織 'cpuset.cpu_exclusive'
  # /bin/echo 0 > cpuset.cpu_exclusive 	-> 鍙栨秷璁剧疆鏍囧織 'cpuset.cpu_exclusive'

```
### 2.4 闄勫姞杩涚▼


```

  # /bin/echo PID > tasks

```
娉ㄦ剰杩欐槸 PID锛岃€屼笉鏄?PIDs銆備綘涓€娆″彧鑳介檮鍔犱竴涓换鍔°€?```

  # /bin/echo PID1 > tasks
  # /bin/echo PID2 > tasks
	...
  # /bin/echo PIDn > tasks


```
## 3. 闂


Q:
   '/bin/echo' 鏄€庝箞鍥炰簨锛?
A:
   bash 鍐呭缓鐨?'echo' 鍛戒护涓嶄細妫€鏌ュ write() 鐨勮皟鐢ㄦ槸鍚︽湁閿欒銆傚鏋滀綘鍦?cpuset 鏂囦欢绯荤粺涓娇鐢ㄥ畠锛屼綘灏嗘棤娉曞垽鏂懡浠ゆ槸鎴愬姛杩樻槸澶辫触銆?
Q:
   褰撴垜闄勫姞杩涚▼鏃讹紝鍙湁琛屼腑鐨勭涓€涓湡姝ｈ闄勫姞浜嗭紒

A:
   鎴戜滑姣忔瀵?write() 鐨勮皟鐢ㄥ彧鑳借繑鍥炰竴涓敊璇爜銆傛墍浠ヤ綘搴旇涔熷彧鏀句竴涓?pid銆?
## 4. 鑱旂郴鏂瑰紡


Web: http://www.bullopensource.org/cpuset
