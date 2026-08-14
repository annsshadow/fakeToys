## 鎺у埗缁勶紙Control Groups锛?

鐢?Paul Menage <menage@google.com> 鍩轰簬
Documentation/admin-guide/cgroup-v1/cpusets.rst 鎾板啓

cpusets.txt 鐨勫師濮嬬増鏉冨０鏄庯細

Portions Copyright (C) 2004 BULL SA.

Portions Copyright (c) 2004-2006 Silicon Graphics, Inc.

Modified by Paul Jackson <pj@sgi.com>

Modified by Christoph Lameter <cl@gentwo.org>


 1. 鎺у埗缁勶紙Control Groups锛?	1.1 浠€涔堟槸 cgroups锛?	1.2 涓轰綍闇€瑕?cgroups锛?	1.3 cgroups 鏄浣曞疄鐜扮殑锛?	1.4 notify_on_release 鐨勪綔鐢ㄦ槸浠€涔堬紵
	1.5 clone_children 鐨勪綔鐢ㄦ槸浠€涔堬紵
	1.6 濡備綍浣跨敤 cgroups锛? 2. 浣跨敤绀轰緥涓庤娉?	2.1 鍩烘湰鐢ㄦ硶
	2.2 闄勫姞杩涚▼
	2.3 鎸夊悕绉版寕杞藉眰绾? 3. 鍐呮牳 API
	3.1 姒傝堪
	3.2 鍚屾
	3.3 瀛愮郴缁?API
 4. 鎵╁睍灞炴€х殑浣跨敤
 5. 闂

## 1. 鎺у埗缁勶紙Control Groups锛?

### 1.1 浠€涔堟槸 cgroups锛?

鎺у埗缁勶紙Control Groups锛夋彁渚涗簡涓€绉嶆満鍒讹紝鐢ㄤ簬灏嗕竴缁勪换鍔″強鍏舵墍鏈夋湭鏉ュ瓙浠诲姟鑱氬悎/鍒掑垎鍒板叿鏈夌壒瀹氳涓虹殑灞傜骇鍖栫粍涓€?
瀹氫箟锛?
**cgroup** 灏嗕竴缁勪换鍔′笌涓€涓垨澶氫釜瀛愮郴缁熺殑鍙傛暟闆嗗悎鍏宠仈璧锋潵銆?
**subsystem锛堝瓙绯荤粺锛?* 鏄竴涓ā鍧楋紝瀹冨埄鐢?cgroups 鎻愪緵鐨勪换鍔″垎缁勬満鍒讹紝浠ョ壒瀹氭柟寮忓鐞嗕换鍔＄粍銆傚瓙绯荤粺閫氬父鏄竴涓?璧勬簮鎺у埗鍣?锛屽畠璋冨害鏌愮璧勬簮鎴栨柦鍔犳瘡 cgroup 闄愬埗锛屼絾瀹冧篃鍙互鏄换浣曞笇鏈涘涓€缁勮繘绋嬫柦鍔犱綔鐢ㄧ殑涓滆タ锛屼緥濡傝櫄鎷熷寲瀛愮郴缁熴€?
**hierarchy锛堝眰绾э級** 鏄竴缁勪互鏍戠姸鎺掑垪鐨?cgroup锛屼娇寰楃郴缁熶腑鐨勬瘡涓换鍔￠兘鎭板ソ浣嶄簬璇ュ眰绾т腑鏌愪竴涓?cgroup 鍐咃紝鍚屾椂杩樻湁涓€缁勫瓙绯荤粺锛涙瘡涓瓙绯荤粺閮芥湁闄勭潃浜庤灞傜骇涓瘡涓?cgroup 鐨勩€佺郴缁熺壒瀹氱殑鐘舵€併€傛瘡涓眰绾ч兘鍏宠仈鐫€涓€涓?cgroup 铏氭嫙鏂囦欢绯荤粺鐨勫疄渚嬨€?
浠讳綍鏃跺埢閮藉彲鑳藉瓨鍦ㄥ涓浜庢椿鍔ㄧ姸鎬佺殑浠诲姟 cgroup 灞傜骇銆傛瘡涓眰绾ч兘鏄绯荤粺涓墍鏈変换鍔＄殑涓€绉嶅垝鍒嗐€?
鐢ㄦ埛鎬佷唬鐮佸彲浠ュ湪 cgroup 铏氭嫙鏂囦欢绯荤粺鐨勪竴涓疄渚嬩腑鎸夊悕绉板垱寤哄拰閿€姣?cgroup銆佹寚瀹氬苟鏌ヨ鏌愪换鍔¤鍒嗛厤鍒板摢涓?cgroup锛屼互鍙婂垪鍑哄垎閰嶇粰鏌?cgroup 鐨勪换鍔?PID銆傝繖浜涘垱寤轰笌鍒嗛厤鍙細褰卞搷涓庤 cgroup 鏂囦欢绯荤粺瀹炰緥鐩稿叧鑱旂殑灞傜骇銆?
灏辫嚜韬€岃█锛宑groups 鐨勫敮涓€鐢ㄩ€旀槸杩涜绠€鍗曠殑浣滀笟璺熻釜銆傚叾鎰忓浘鍦ㄤ簬璁╁叾浠栧瓙绯荤粺鎸傛帴鍒伴€氱敤鐨?cgroup 鏀寔涓婏紝涓?cgroups 鎻愪緵鏂板睘鎬э紝渚嬪瀵?cgroup 涓繘绋嬪彲璁块棶鐨勮祫婧愯繘琛岀粺璁?闄愬埗銆備緥濡傦紝cpusets锛堣 Documentation/admin-guide/cgroup-v1/cpusets.rst锛夊厑璁镐綘灏嗕竴缁?CPU 鍜屼竴缁勫唴瀛樿妭鐐逛笌姣忎釜 cgroup 涓殑浠诲姟鍏宠仈璧锋潵銆?

### 1.2 涓轰綍闇€瑕?cgroups锛?

Linux 鍐呮牳涓湁澶氱涓鸿繘绋嬭仛鍚堬紙涓昏鍑轰簬璧勬簮璺熻釜鐩殑锛夋墍鍋氱殑鍔姏銆傝繖浜涘姫鍔涘寘鎷?cpusets銆丆KRM/ResGroups銆乁serBeanCounters 浠ュ強铏氭嫙鏈嶅姟鍣ㄥ懡鍚嶇┖闂淬€傚畠浠兘闇€瑕?瀵硅繘绋嬭繘琛屽垎缁?鍒掑垎"杩欎竴鍩烘湰姒傚康锛屼娇寰楁柊 fork 鍑虹殑杩涚▼鏈€缁堜笌鍏剁埗杩涚▼澶勪簬鍚屼竴缁勶紙cgroup锛変腑銆?
鍐呮牳 cgroup 琛ヤ竵鎻愪緵浜嗛珮鏁堝疄鐜版绫诲垎缁勬墍闇€鐨勬渶灏忓繀瑕佸唴鏍告満鍒躲€傚畠瀵圭郴缁熷揩閫熻矾寰勭殑褰卞搷鏋佸皬锛屽苟涓鸿濡?cpusets 涔嬬被鐨勭壒瀹氬瓙绯荤粺鎻愪緵閽╁瓙锛屼互渚挎寜闇€鎻愪緵棰濆琛屼负銆?
鎻愪緵澶氬眰绾ф敮鎸侊紝鏄负浜嗗簲瀵逛笉鍚屽瓙绯荤粺瀵逛换鍔″垝鍒嗗埌 cgroup 鐨勬柟寮忔埅鐒朵笉鍚岀殑鎯呭喌鈥斺€旀嫢鏈夊苟琛屽眰绾т娇寰楁瘡涓眰绾ч兘鍙互鎴愪负浠诲姟鐨勪竴绉嶈嚜鐒跺垝鍒嗭紝鑰屼笉蹇呭鐞嗗綋鍑犱釜浜掍笉鐩稿叧鐨勫瓙绯荤粺琚揩濉炶繘鍚屼竴妫?cgroup 鏍戞椂浼氬嚭鐜扮殑澶嶆潅浠诲姟缁勫悎銆?
鍦ㄤ竴涓瀬绔紝姣忎釜璧勬簮鎺у埗鍣ㄦ垨瀛愮郴缁熷彲浠ュ浜庣嫭绔嬬殑灞傜骇涓紱鍦ㄥ彟涓€涓瀬绔紝鎵€鏈夊瓙绯荤粺閮介檮鍔犲埌鍚屼竴涓眰绾с€?
浣滀负鍙互浠庡灞傜骇鍙楃泭鐨勫満鏅ず渚嬶紙鏈€鍒濈敱 vatsa@in.ibm.com 鎻愬嚭锛夛紝鑰冭檻涓€鍙版嫢鏈夊悇绫荤敤鎴凤紙瀛︾敓銆佹暀鎺堛€佺郴缁熶换鍔＄瓑锛夌殑澶у瀷澶у鏈嶅姟鍣ㄣ€傝鏈嶅姟鍣ㄧ殑璧勬簮瑙勫垝鍙互娌夸互涓嬫柟鍚戣繘琛?```

       CPU :          "Top cpuset"
                       /       \
               CPUSet1         CPUSet2
                  |               |
               (Professors)    (Students)

               In addition (system tasks) are attached to topcpuset (so
               that they can run anywhere) with a limit of 20%

       Memory : Professors (50%), Students (30%), system (20%)

       Disk : Professors (50%), Students (30%), system (20%)

       Network : WWW browsing (20%), Network File System (60%), others (20%)
                               / \
               Professors (15%)  students (5%)

```
鍍?Firefox/Lynx 杩欐牱鐨勬祻瑙堝櫒褰掑叆 WWW 缃戠粶绫伙紝鑰?(k)nfsd 褰掑叆 NFS 缃戠粶绫汇€?
鍚屾椂锛孎irefox/Lynx 浼氭牴鎹惎鍔ㄨ€咃紙鏁欐巿/瀛︾敓锛夊叡浜浉搴旂殑 CPU/鍐呭瓨绫汇€?
鐢变簬鑳藉閽堝涓嶅悓璧勬簮瀵逛换鍔¤繘琛屼笉鍚屽垎绫伙紙閫氳繃灏嗚繖浜涜祫婧愬瓙绯荤粺鏀惧叆涓嶅悓灞傜骇锛夛紝绠＄悊鍛樺彲浠ヨ交鏉捐缃竴涓帴鏀?exec 閫氱煡鐨勮剼鏈?```

    # echo browser_pid > /sys/fs/cgroup/<restype>/<userclass>/tasks

```
濡傛灉鍙湁鍗曚竴灞傜骇锛屼粬鐜板湪鍙兘闇€瑕佷负鍚姩鐨勬瘡涓祻瑙堝櫒鍒涘缓涓€涓嫭绔嬬殑 cgroup锛屽苟灏嗗叾涓庣浉搴旂殑缃戠粶鍙婂叾浠栬祫婧愮被鍏宠仈銆傝繖鍙兘瀵艰嚧姝ょ被 cgroup 澶ч噺澧炴畺銆?
鍐嶅亣璁剧鐞嗗憳鎯充复鏃剁粰浜堟煇瀛︾敓鐨勬祻瑙堝櫒鏇撮珮鐨勭綉缁滆闂潈闄愶紙鍥犱负宸叉槸娣卞锛岃鐢ㄦ埛鎯宠繘琛屽湪绾挎父鎴?:))锛屾垨鑰呯粰浜堣瀛︾敓鐨勬煇涓豢鐪熺▼搴忔洿楂樼殑 CPU 绠楀姏銆?
鍊熷姪鐩存帴灏?PID 鍐欏叆璧勬簮绫荤殑鑳藉姏锛屽彧闇€
```

       # echo pid > /sys/fs/cgroup/network/<new_class>/tasks
       (after some time)
       # echo pid > /sys/fs/cgroup/network/<orig_class>/tasks

```
娌℃湁杩欑鑳藉姏锛岀鐞嗗憳灏变笉寰椾笉鎶婅繖涓?cgroup 鎷嗗垎鎴愬涓嫭绔嬬殑 cgroup锛岀劧鍚庡皢鏂扮殑 cgroup 涓庢柊鐨勮祫婧愮被鍏宠仈璧锋潵銆?


### 1.3 cgroups 鏄浣曞疄鐜扮殑锛?

鎺у埗缁勫鍐呮牳鐨勬墿灞曞涓嬶細

 - 绯荤粺涓殑姣忎釜浠诲姟閮芥湁涓€涓寚鍚?css_set 鐨勫紩鐢ㄨ鏁版寚閽堛€?
 - 涓€涓?css_set 鍖呭惈涓€缁勬寚鍚?cgroup_subsys_state 瀵硅薄鐨勫紩鐢ㄨ鏁版寚閽堬紝绯荤粺涓敞鍐岀殑姣忎釜 cgroup 瀛愮郴缁熷搴斾竴涓€備换鍔′笌鍏舵墍灞炵殑姣忎釜灞傜骇涓殑 cgroup 涔嬮棿娌℃湁鐩存帴鐨勯摼鎺ワ紝浣嗗彲浠ラ€氳繃 cgroup_subsys_state 瀵硅薄涓殑鎸囬拡鏉ョ‘瀹氥€傝繖鏄洜涓鸿闂瓙绯荤粺鐘舵€佹槸棰勬湡浼氶绻佸彂鐢熷湪鎬ц兘鍏抽敭浠ｇ爜涓殑鎿嶄綔锛岃€岄渶瑕佷换鍔″疄闄?cgroup 鍒嗛厤锛堝挨鍏舵槸 cgroup 闂磋縼绉伙級鐨勬搷浣滃垯杈冨皯瑙併€備竴鏉￠摼琛ㄩ€氳繃 css_set 璐┛姣忎釜 task_struct 鐨?cg_list 瀛楁锛岄敋瀹氫簬 css_set->tasks銆?
 - 鍙互鎸傝浇涓€涓?cgroup 灞傜骇鏂囦欢绯荤粺锛屼互渚夸粠鐢ㄦ埛绌洪棿杩涜娴忚鍜屾搷浣溿€?
 - 浣犲彲浠ュ垪鍑洪檮鍔犲埌浠讳綍 cgroup 鐨勬墍鏈変换鍔★紙鎸?PID锛夈€?
cgroups 鐨勫疄鐜伴渶瑕佸湪鍐呮牳鍏朵綑閮ㄥ垎鍔犲叆灏戦噺绠€鍗曢挬瀛愶紝涓旈兘涓嶅湪鎬ц兘鍏抽敭璺緞涓婏細

 - 鍦?init/main.c 涓紝鐢ㄤ簬鍦ㄧ郴缁熷惎鍔ㄦ椂鍒濆鍖栨牴 cgroups 鍜屽垵濮?css_set銆?
 - 鍦?fork 鍜?exit 涓紝鐢ㄤ簬灏嗕换鍔￠檮鍔犲埌鍏?css_set 鎴栦粠鍏朵腑鍒嗙銆?
姝ゅ锛屽彲浠ユ寕杞戒竴涓?"cgroup" 绫诲瀷鐨勬柊鏂囦欢绯荤粺锛屼互渚挎祻瑙堝拰淇敼鍐呮牳褰撳墠宸茬煡鐨?cgroups銆傛寕杞?cgroup 灞傜骇鏃讹紝浣犲彲浠ユ寚瀹氫竴涓€楀彿鍒嗛殧鐨勫瓙绯荤粺鍒楄〃浣滀负鏂囦欢绯荤粺鎸傝浇閫夐」銆傞粯璁ゆ儏鍐典笅锛屾寕杞?cgroup 鏂囦欢绯荤粺浼氬皾璇曟寕杞戒竴涓寘鍚墍鏈夊凡娉ㄥ唽瀛愮郴缁熺殑灞傜骇銆?
濡傛灉宸茬粡瀛樺湪涓€涓敱瀹屽叏鐩稿悓瀛愮郴缁熼泦鍚堟瀯鎴愮殑娲诲姩灞傜骇锛屽畠灏嗚鏂版寕杞藉鐢ㄣ€傚鏋滄病鏈夊尮閰嶇殑鐜版湁灞傜骇锛屼笖鎵€璇锋眰鐨勪换涓€瀛愮郴缁熷凡鍦ㄦ煇涓幇鏈夊眰绾т腑浣跨敤锛屽垯鎸傝浇灏嗕互 -EBUSY 澶辫触銆傚惁鍒欙紝灏嗘縺娲讳竴涓柊鐨勫眰绾э紝骞朵笌鎵€璇锋眰鐨勫瓙绯荤粺鍏宠仈銆?
鐩墠鏃犳硶灏嗘柊瀛愮郴缁熺粦瀹氬埌涓€涓椿鍔ㄧ殑 cgroup 灞傜骇锛屼篃鏃犳硶灏嗗瓙绯荤粺浠庢椿鍔ㄧ殑 cgroup 灞傜骇瑙ｇ粦銆傛湭鏉ユ垨璁稿彲浠ュ疄鐜帮紝浣嗚繖鍏呮弧浜嗘鎵嬬殑閿欒澶勭悊鎭㈠闂銆?
褰撴煇涓?cgroup 鏂囦欢绯荤粺琚嵏杞芥椂锛屽鏋滃湪椤跺眰 cgroup 涔嬩笅鍒涘缓浜嗕换浣曞瓙 cgroup锛岃灞傜骇鍗充娇琚嵏杞戒粛浼氫繚鎸佹椿鍔紱濡傛灉娌℃湁瀛?cgroup锛屽垯璇ュ眰绾у皢琚仠鐢ㄣ€?
娌℃湁涓?cgroups 鏂板浠讳綍绯荤粺璋冪敤鈥斺€旀墍鏈夌敤浜庢煡璇㈠拰淇敼 cgroups 鐨勬敮鎸侀兘閫氳繃杩欎釜 cgroup 鏂囦欢绯荤粺瀹炵幇銆?
/proc 涓嬫瘡涓换鍔￠兘澶氫簡涓€涓悕涓?'cgroup' 鐨勬枃浠讹紝瀹冮拡瀵规瘡涓椿鍔ㄥ眰绾ф樉绀哄瓙绯荤粺鍚嶇О浠ュ強 cgroup 鍚嶇О锛堜綔涓虹浉瀵逛簬 cgroup 鏂囦欢绯荤粺鏍圭殑璺緞锛夈€?
姣忎釜 cgroup 鐢?cgroup 鏂囦欢绯荤粺涓殑鐩綍琛ㄧず锛屽叾涓寘鍚涓嬫弿杩拌 cgroup 鐨勬枃浠讹細

 - tasks锛氶檮鍔犲埌璇?cgroup 鐨勪换鍔″垪琛紙鎸?PID锛夈€傛鍒楄〃涓嶄繚璇佹湁搴忋€傚悜璇ユ枃浠跺啓鍏ヤ竴涓嚎绋?ID 浼氬皢璇ョ嚎绋嬬Щ鍏ユ cgroup銆? - cgroup.procs锛氳 cgroup 涓殑绾跨▼缁?ID 鍒楄〃銆傛鍒楄〃涓嶄繚璇佹湁搴忥紝涔熶笉淇濊瘉涓嶅惈閲嶅鐨?TGID锛屽鏋滅‘瀹為渶瑕侊紝鐢ㄦ埛鎬佸簲瀵瑰叾鎺掑簭/鍘婚噸銆傚悜璇ユ枃浠跺啓鍏ョ嚎绋嬬粍 ID 浼氬皢璇ョ粍涓墍鏈夌嚎绋嬬Щ鍏ユ cgroup銆? - notify_on_release 鏍囧織锛氶€€鍑烘椂鏄惁杩愯 release agent锛? - release_agent锛氱敤浜庨噴鏀鹃€氱煡鐨勮矾寰勶紙璇ユ枃浠朵粎瀛樺湪浜庨《灞?cgroup锛夈€?
鍏朵粬瀛愮郴缁燂紙濡?cpusets锛夊彲鑳戒細鍦ㄦ瘡涓?cgroup 鐩綍涓坊鍔犻澶栫殑鏂囦欢銆?
鏂?cgroups 閫氳繃 mkdir 绯荤粺璋冪敤鎴?shell 鍛戒护鍒涘缓銆俢group 鐨勫睘鎬э紙渚嬪鍏舵爣蹇楋級閫氳繃鍐欏叆璇?cgroup 鐩綍涓殑鐩稿簲鏂囦欢鏉ヤ慨鏀癸紝濡備笂鎵€鍒椼€?
宓屽 cgroups 鐨勫叿鍚嶅眰绾х粨鏋勫厑璁稿皢澶у瀷绯荤粺鍒掑垎涓哄祵濂楃殑銆佸彲鍔ㄦ€佸彉鏇寸殑"杞垎鍖?銆?
姣忎釜浠诲姟瀵?cgroup 鐨勯檮鍔狅紙鍦?fork 鏃剁敱鍏朵换鎰忓瓙浠诲姟鑷姩缁ф壙锛変娇寰楀彲浠ュ皢绯荤粺涓婄殑宸ヤ綔璐熻浇缁勭粐鎴愮浉鍏崇殑浠诲姟闆嗗悎銆傚鏋滅浉搴?cgroup 鏂囦欢绯荤粺鐩綍鐨勬潈闄愬厑璁革紝浠诲姟鍙互琚噸鏂伴檮鍔犲埌浠讳綍鍏朵粬 cgroup銆?
褰撲换鍔′粠涓€涓?cgroup 绉诲埌鍙︿竴涓椂锛屽畠浼氳幏寰椾竴涓柊鐨?css_set 鎸囬拡鈥斺€斿鏋滃凡缁忓瓨鍦ㄤ竴涓寘鍚墍闇€ cgroup 闆嗗悎鐨?css_set锛屽垯澶嶇敤璇ョ粍锛屽惁鍒欏垎閰嶄竴涓柊鐨?css_set銆傞€氳繃鏌ユ壘鍝堝笇琛ㄦ潵瀹氫綅鍚堥€傜殑鐜版湁 css_set銆?
涓轰簡鍏佽浠庢煇涓?cgroup 璁块棶鏋勬垚瀹冪殑 css_set锛堣繘鑰岃闂换鍔★級锛屼竴缁?cg_cgroup_link 瀵硅薄鏋勬垚浜嗕竴涓牸锛涙瘡涓?cg_cgroup_link 閫氳繃鍏?cgrp_link_list 瀛楁閾炬帴杩涘崟涓?cgroup 鐨?cg_cgroup_links 鍒楄〃锛屽苟閫氳繃鍏?cg_link_list 瀛楁閾炬帴杩涘崟涓?css_set 鐨?cg_cgroup_links 鍒楄〃銆?
鍥犳锛屽彲浠ラ€氳繃閬嶅巻寮曠敤璇?cgroup 鐨勬瘡涓?css_set銆佸苟杩涗竴姝ラ亶鍘嗘瘡涓?css_set 鐨勪换鍔￠泦锛屾潵鍒楀嚭璇?cgroup 涓殑浠诲姟闆嗗悎銆?
浣跨敤 Linux 铏氭嫙鏂囦欢绯荤粺锛坴fs锛夋潵琛ㄧず cgroup 灞傜骇锛屼负 cgroups 鎻愪緵浜嗕竴涓啛鎮夌殑鏉冮檺鍜屽懡鍚嶇┖闂达紝鍚屾椂鍙渶鏈€灏戠殑棰濆鍐呮牳浠ｇ爜銆?
### 1.4 notify_on_release 鐨勪綔鐢ㄦ槸浠€涔堬紵


濡傛灉鏌?cgroup 涓殑 notify_on_release 鏍囧織琚惎鐢紙1锛夛紝閭ｄ箞姣忓綋璇?cgroup 涓殑鏈€鍚庝竴涓换鍔＄寮€锛堥€€鍑烘垨闄勫姞鍒板叾浠?cgroup锛夛紝骞朵笖璇?cgroup 鐨勬渶鍚庝竴涓瓙 cgroup 琚Щ闄ゆ椂锛屽唴鏍稿氨浼氳繍琛岃灞傜骇鏍圭洰褰曚腑 "release_agent" 鏂囦欢鍐呭鎵€鎸囧畾鐨勫懡浠わ紝骞舵彁渚涜閬楀純 cgroup 鐨勮矾寰勫悕锛堢浉瀵逛簬 cgroup 鏂囦欢绯荤粺鐨勬寕杞界偣锛夈€傝繖鏍峰氨瀹炵幇浜嗚閬楀純 cgroups 鐨勮嚜鍔ㄧЩ闄ゃ€傜郴缁熷惎鍔ㄦ椂鏍?cgroup 涓?notify_on_release 鐨勯粯璁ゅ€间负绂佺敤锛?锛夈€傚叾浠?cgroup 鍦ㄥ垱寤烘椂鐨勯粯璁ゅ€硷紝鏄叾鐖剁骇 notify_on_release 璁剧疆鐨勫綋鍓嶅€笺€俢group 灞傜骇 release_agent 璺緞鐨勯粯璁ゅ€间负绌恒€?
### 1.5 clone_children 鐨勪綔鐢ㄦ槸浠€涔堬紵


璇ユ爣蹇楀彧褰卞搷 cpuset 鎺у埗鍣ㄣ€傚鏋滄煇 cgroup 涓惎鐢ㄤ簡 clone_children 鏍囧織锛?锛夛紝鏂扮殑 cpuset cgroup 灏嗗湪鍒濆鍖栨椂浠庣埗绾у鍒跺叾閰嶇疆銆?
### 1.6 濡備綍浣跨敤 cgroups锛?

瑕佸惎鍔ㄤ竴涓皢琚寘鍚湪 cgroup 涓殑鏂颁綔涓氾紝浣跨敤
```

 1) mount -t tmpfs cgroup_root /sys/fs/cgroup
 2) mkdir /sys/fs/cgroup/cpuset
 3) mount -t cgroup -ocpuset cpuset /sys/fs/cgroup/cpuset
 4) Create the new cgroup by doing mkdir's and write's (or echo's) in
    the /sys/fs/cgroup/cpuset virtual file system.
 5) Start a task that will be the "founding father" of the new job.
 6) Attach that task to the new cgroup by writing its PID to the
    /sys/fs/cgroup/cpuset tasks file for that cgroup.
 7) fork, exec or clone the job tasks from this founding father task.

```
渚嬪锛屼笅闈㈣繖缁勫懡浠ゅ皢寤虹珛涓€涓悕涓?"Charlie" 鐨?cgroup锛屽叾涓彧鍖呭惈 CPU 2 鍜?3锛屼互鍙婂唴瀛樿妭鐐?1锛?```

  mount -t tmpfs cgroup_root /sys/fs/cgroup
  mkdir /sys/fs/cgroup/cpuset
  mount -t cgroup cpuset -ocpuset /sys/fs/cgroup/cpuset
  cd /sys/fs/cgroup/cpuset
  mkdir Charlie
  cd Charlie
  /bin/echo 2-3 > cpuset.cpus
  /bin/echo 1 > cpuset.mems
  /bin/echo $$ > tasks
  sh
  # The subshell 'sh' is now running in cgroup Charlie
  # The next line should display '/Charlie'
  cat /proc/self/cgroup

```
## 2. 浣跨敤绀轰緥涓庤娉?

### 2.1 鍩烘湰鐢ㄦ硶


鍒涘缓銆佷慨鏀广€佷娇鐢?cgroups 閮藉彲浠ラ€氳繃 cgroup 铏氭嫙鏂囦欢绯荤粺瀹屾垚銆?
```

  # mount -t cgroup xxx /sys/fs/cgroup

```
"xxx" 涓嶄細琚?cgroup 浠ｇ爜瑙ｉ噴锛屼絾瀹冧細鍑虹幇鍦?/proc/mounts 涓紝鍥犳鍙互鏄綘鍠滄鐨勪换浣曟湁鐢ㄧ殑鏍囪瘑瀛楃涓层€?
娉ㄦ剰锛氭煇浜涘瓙绯荤粺鍦ㄦ病鏈夌敤鎴峰厛鎻愪緵涓€浜涜緭鍏ユ椂鏃犳硶宸ヤ綔銆備緥濡傦紝濡傛灉鍚敤浜?cpusets锛岀敤鎴峰繀椤诲厛涓烘瘡涓柊寤虹殑 cgroup 濉厖 cpus 鍜?mems 鏂囦欢锛岃缁勬墠鑳借浣跨敤銆?
濡?`1.2 Why are cgroups needed?` 涓€鑺傛墍杩帮紝浣犲簲璇ヤ负鎯宠鎺у埗鐨勬瘡绉嶅崟涓€璧勬簮鎴栬祫婧愮粍鍒涘缓涓嶅悓鐨?cgroups 灞傜骇銆傚洜姝わ紝浣犲簲璇ュ湪 /sys/fs/cgroup 涓婃寕杞戒竴涓?tmpfs锛屽苟涓烘瘡涓?cgroup 璧勬簮鎴栬祫婧?```

  # mount -t tmpfs cgroup_root /sys/fs/cgroup
  # mkdir /sys/fs/cgroup/rg1

```
瑕佹寕杞戒竴涓粎鍖呭惈 cpuset 鍜?memory 鐨?cgroup 灞傜骇
```

  # mount -t cgroup -o cpuset,memory hier1 /sys/fs/cgroup/rg1

```
铏界劧鐩墠鏀寔閲嶆柊鎸傝浇 cgroups锛屼絾涓嶆帹鑽愪娇鐢ㄣ€傞噸鏂版寕杞藉厑璁告洿鏀圭粦瀹氱殑瀛愮郴缁熷拰 release_agent銆傞噸鏂扮粦瀹氬嚑涔庢病浠€涔堢敤澶勶紝鍥犱负瀹冨彧鍦ㄥ眰绾т负绌烘椂鏈夋晥锛岃€屼笖 release_agent 鏈韩搴旇甯歌鐨?fsnotify 鍙栦唬銆傚閲嶆柊鎸傝浇鐨勬敮鎸佸皢鍦ㄦ湭鏉ヨ绉婚櫎銆?
```

  # mount -t cgroup -o cpuset,release_agent="/sbin/cpuset_release_agent" \
    xxx /sys/fs/cgroup/rg1

```
娉ㄦ剰锛屽娆℃寚瀹?'release_agent' 灏嗚繑鍥炲け璐ャ€?
娉ㄦ剰锛屾洿鏀瑰瓙绯荤粺闆嗗悎鐩墠浠呭湪灞傜骇鐢卞崟涓€锛堟牴锛塩group 鏋勬垚鏃舵墠鍙楁敮鎸併€傛敮鎸佷粠鐜版湁 cgroup 灞傜骇浠绘剰缁戝畾/瑙ｇ粦瀛愮郴缁熺殑鑳藉姏锛岃鍒掑湪灏嗘潵瀹炵幇銆?
鐒跺悗鍦?/sys/fs/cgroup/rg1 涓嬶紝浣犲彲浠ユ壘鍒颁竴涓笌绯荤粺涓?cgroups 鏍戠浉瀵瑰簲鐨勬爲銆備緥濡傦紝/sys/fs/cgroup/rg1 鏄寔鏈夋暣涓郴缁熺殑 cgroup銆?
```

  # echo "/sbin/new_release_agent" > /sys/fs/cgroup/rg1/release_agent

```
瀹冧篃鍙互閫氳繃閲嶆柊鎸傝浇鏉ユ洿鏀广€?
```

  # cd /sys/fs/cgroup/rg1
  # mkdir my_cgroup

```
鐜板湪浣犳兂鐢ㄨ繖涓?cgroup 鍋氱偣浠€涔堬細

  # cd my_cgroup

```

  # ls
  cgroup.procs notify_on_release tasks
  (plus whatever files added by the attached subsystems)

```

```

  # /bin/echo $$ > tasks

```
浣犱篃鍙互鍦ㄤ綘鐨?cgroup 鍐呴儴浣跨敤 mkdir 鍒涘缓 cgroups
```

  # mkdir my_sub_cs

```

```

  # rmdir my_sub_cs

```
濡傛灉璇?cgroup 姝ｅ湪浣跨敤涓紙鍐呴儴鏈?cgroup銆佹垨宸查檮鍔犺繘绋嬨€佹垨琚叾浠栧瓙绯荤粺鐗瑰畾鐨勫紩鐢ㄤ繚鎸佸瓨娲伙級锛岃繖灏嗗け璐ャ€?
### 2.2 闄勫姞杩涚▼


```

  # /bin/echo PID > tasks

```
娉ㄦ剰锛屾槸 PID 鑰屼笉鏄?PIDs銆備綘涓€娆″彧鑳介檮鍔犱竴涓换鍔°€?```

  # /bin/echo PID1 > tasks
  # /bin/echo PID2 > tasks
	  ...
  # /bin/echo PIDn > tasks

```

```

  # echo 0 > tasks

```
浣犲彲浠ヤ娇鐢?cgroup.procs 鏂囦欢浠ｆ浛 tasks 鏂囦欢锛屼竴娆℃€хЩ鍔ㄤ竴涓嚎绋嬬粍涓殑鎵€鏈夌嚎绋嬨€傚皢绾跨▼缁勪腑浠绘剰浠诲姟鐨?PID 鍥炴樉鍒?cgroup.procs 浼氫娇璇ョ嚎绋嬬粍涓殑鎵€鏈変换鍔￠兘闄勫姞鍒拌 cgroup銆傚悜 cgroup.procs 鍐欏叆 0 浼氱Щ鍔ㄥ啓鍏ヤ换鍔℃墍鍦ㄧ嚎绋嬬粍涓殑鎵€鏈変换鍔°€?
娉ㄦ剰锛氱敱浜庢瘡涓换鍔″湪姣忎釜宸叉寕杞藉眰绾т腑濮嬬粓鎭板ソ鏄竴涓?cgroup 鐨勬垚鍛橈紝瑕佸皢浠诲姟浠庡叾褰撳墠 cgroup 绉婚櫎锛屼綘蹇呴』閫氳繃鍐欏叆鏂?cgroup 鐨?tasks 鏂囦欢灏嗗畠绉诲叆涓€涓柊鐨?cgroup锛堝彲鑳芥槸鏍?cgroup锛夈€?
娉ㄦ剰锛氱敱浜庢煇浜?cgroup 瀛愮郴缁熸柦鍔犵殑闄愬埗锛屽皢杩涚▼绉诲姩鍒板彟涓€涓?cgroup 鍙兘浼氬け璐ャ€?
### 2.3 鎸夊悕绉版寕杞藉眰绾?

鍦ㄦ寕杞?cgroups 灞傜骇鏃朵紶鍏?name=<x> 閫夐」锛屼細灏嗙粰瀹氬悕绉颁笌璇ュ眰绾у叧鑱斻€傝繖鍙互鍦ㄦ寕杞戒竴涓凡瀛樺湪鐨勫眰绾ф椂浣跨敤锛屼互渚挎寜鍚嶇О鑰屼笉鏄寜鍏舵椿鍔ㄥ瓙绯荤粺闆嗗悎鏉ュ紩鐢ㄥ畠銆傛瘡涓眰绾ц涔堟棤鍚嶏紝瑕佷箞鍏锋湁涓€涓敮涓€鍚嶇О銆?
鍚嶇О搴斿綋鍖归厤 [\w.-]+

褰撲负鏂板眰绾т紶鍏?name=<x> 閫夐」鏃讹紝浣犻渶瑕佹墜鍔ㄦ寚瀹氬瓙绯荤粺锛涘綋浣犱负瀛愮郴缁熸寚瀹氬悕绉版椂锛屼笉鏀寔"鏈樉寮忔寚瀹氫换浣曞瓙绯荤粺鏃舵寕杞芥墍鏈夊瓙绯荤粺"鐨勪紶缁熻涓恒€?
瀛愮郴缁熺殑鍚嶇О浼氫綔涓哄眰绾ф弿杩扮殑涓€閮ㄥ垎鍑虹幇鍦?/proc/mounts 鍜?/proc/<pid>/cgroups 涓€?

## 3. 鍐呮牳 API


### 3.1 姒傝堪


姣忎釜鎯宠鎸傛帴鍒伴€氱敤 cgroup 绯荤粺鐨勫唴鏍稿瓙绯荤粺閮介渶瑕佸垱寤轰竴涓?cgroup_subsys 瀵硅薄銆傚畠鍖呭惈鍚勭鏂规硶锛堝嵆鏉ヨ嚜 cgroup 绯荤粺鐨勫洖璋冿級锛屼互鍙婁竴涓皢鐢?cgroup 绯荤粺鍒嗛厤鐨勫瓙绯荤粺 ID銆?
cgroup_subsys 瀵硅薄涓殑鍏朵粬瀛楁鍖呮嫭锛?
- subsys_id锛氬瓙绯荤粺鐨勫敮涓€鏁扮粍绱㈠紩锛屾寚绀鸿瀛愮郴缁熷簲绠＄悊鐨?cgroup->subsys[] 涓殑鏉＄洰銆?
- name锛氬簲鍒濆鍖栦负涓€涓敮涓€鐨勫瓙绯荤粺鍚嶇О銆傞暱搴︿笉搴旇秴杩?MAX_CGROUP_TYPE_NAMELEN銆?
- early_init锛氭寚绀鸿瀛愮郴缁熸槸鍚﹂渶瑕佸湪绯荤粺鍚姩鏃舵彁鍓嶅垵濮嬪寲銆?
绯荤粺鍒涘缓鐨勬瘡涓?cgroup 瀵硅薄閮芥湁涓€涓寜瀛愮郴缁?ID 绱㈠紩鐨勬寚閽堟暟缁勶紱璇ユ寚閽堝畬鍏ㄧ敱瀛愮郴缁熺鐞嗭紱閫氱敤鐨?cgroup 浠ｇ爜姘歌繙涓嶄細瑙﹀強杩欎釜鎸囬拡銆?
### 3.2 鍚屾


cgroup 绯荤粺浣跨敤涓€涓叏灞€浜掓枼浣?cgroup_mutex銆備换浣曟兂瑕佷慨鏀?cgroup 鐨勪唬鐮侀兘搴旇幏鍙栧畠銆傚畠涔熷彲浠ヨ鑾峰彇浠ラ樆姝?cgroups 琚慨鏀癸紝浣嗗湪閭ｇ鎯呭喌涓嬩娇鐢ㄦ洿鍏蜂綋鐨勯攣鍙兘鏇村悎閫傘€?
鏇村缁嗚妭瑙?kernel/cgroup.c銆?
瀛愮郴缁熷彲浠ラ€氳繃 cgroup_lock()/cgroup_unlock() 鍑芥暟鑾峰彇/閲婃斁 cgroup_mutex銆?
鍙互閫氳繃浠ヤ笅鏂瑰紡璁块棶浠诲姟鐨?cgroup 鎸囬拡锛?- 鎸佹湁 cgroup_mutex 鏃?- 鎸佹湁浠诲姟鐨?alloc_lock 鏃讹紙閫氳繃 task_lock()锛?- 鍦?rcu_read_lock() 涓寸晫鍖哄唴閫氳繃 rcu_dereference()

### 3.3 瀛愮郴缁?API


姣忎釜瀛愮郴缁熷簲褰擄細

- 鍦?linux/cgroup_subsys.h 涓坊鍔犱竴涓潯鐩?- 瀹氫箟涓€涓悕涓?<name>_cgrp_subsys 鐨?cgroup_subsys 瀵硅薄

姣忎釜瀛愮郴缁熷彲浠ュ鍑轰互涓嬫柟娉曘€傚敮涓€蹇呴渶鐨勬柟娉曟槸 css_alloc/free銆傚叾浠栦负 null 鐨勬柟娉曡鍋囧畾涓烘垚鍔熺殑绌烘搷浣溿€?
`struct cgroup_subsys_state **css_alloc(struct cgroup **cgrp)`
(cgroup_mutex held by caller)

璋冪敤浠ヤ负 cgroup 鍒嗛厤涓€涓瓙绯荤粺鐘舵€佸璞°€傚瓙绯荤粺搴斿綋涓轰紶鍏ョ殑 cgroup 鍒嗛厤鍏跺瓙绯荤粺鐘舵€佸璞★紝鎴愬姛鏃惰繑鍥炴寚鍚戞柊瀵硅薄鐨勬寚閽堬紝鍚﹀垯杩斿洖 ERR_PTR() 鍊笺€傛垚鍔熷悗锛屽瓙绯荤粺鎸囬拡搴旀寚鍚戜竴涓?cgroup_subsys_state 绫诲瀷鐨勭粨鏋勶紙閫氬父鍐呭祵浜庢洿澶х殑銆佸瓙绯荤粺鐗瑰畾鐨勫璞′腑锛夛紝璇ョ粨鏋勫皢鐢?cgroup 绯荤粺鍒濆鍖栥€傛敞鎰忥紝鍦ㄥ垵濮嬪寲鏃朵細璋冪敤鏈嚱鏁颁互鍒涘缓璇ュ瓙绯荤粺鐨勬牴瀛愮郴缁熺姸鎬侊紱杩欑鎯呭喌鍙互閫氳繃浼犲叆鐨?cgroup 瀵硅薄鍏锋湁 NULL 鐖剁骇锛堝洜涓哄畠鏄眰绾х殑鏍癸級鏉ヨ瘑鍒紝杩欓噷涔熼€傚悎鏀剧疆鍒濆鍖栦唬鐮併€?
`int css_online(struct cgroup *cgrp)`
(cgroup_mutex held by caller)

鍦?@cgrp 鎴愬姛瀹屾垚鎵€鏈夊垎閰嶅苟瀵?cgroup_for_each_child/descendant_*() 杩唬鍣ㄥ彲瑙佷箣鍚庤皟鐢ㄣ€傚瓙绯荤粺鍙互閫氳繃杩斿洖 -errno 鏉ラ€夋嫨浣垮垱寤哄け璐ャ€傝鍥炶皟鍙敤浜庡疄鐜板彲闈犵殑鐘舵€佸叡浜笌娌垮眰绾х殑浼犳挱銆傝瑙?cgroup_for_each_live_descendant_pre() 涓婄殑娉ㄩ噴銆?
`void css_offline(struct cgroup *cgrp);`
(cgroup_mutex held by caller)

杩欐槸 css_online() 鐨勫弽鍚戞搷浣滐紝褰撲笖浠呭綋 css_online() 宸插湪 @cgrp 涓婃垚鍔熸椂鎵嶄細琚皟鐢ㄣ€傚畠鏍囧織鐫€ @cgrp 缁堢粨杩囩▼鐨勫紑濮嬨€侤cgrp 姝ｅ湪琚Щ闄わ紝瀛愮郴缁熷簲寮€濮嬩涪寮冨叾鎸佹湁鐨勫 @cgrp 鐨勬墍鏈夊紩鐢ㄣ€傚綋鎵€鏈夊紩鐢ㄩ兘琚涪寮冨悗锛宑group 绉婚櫎灏嗚繘鍏ヤ笅涓€姝モ€斺€攃ss_free()銆傚湪姝ゅ洖璋冧箣鍚庯紝瀛愮郴缁熷簲灏?@cgrp 瑙嗕负宸叉浜°€?
`void css_free(struct cgroup *cgrp)`
(cgroup_mutex held by caller)

cgroup 绯荤粺鍗冲皢閲婃斁 @cgrp锛涘瓙绯荤粺搴旈噴鏀惧叾瀛愮郴缁熺姸鎬佸璞°€傝皟鐢ㄦ鏂规硶鏃讹紝@cgrp 宸插畬鍏ㄤ笉鍐嶈浣跨敤锛汙cgrp->parent 浠嶇劧鏈夋晥銆傦紙娉ㄦ剰鈥斺€斿鏋滄湰瀛愮郴缁熺殑 create() 鏂规硶宸蹭负鏂?cgroup 璋冪敤涔嬪悗鍙戠敓閿欒锛屼篃鍙兘閽堝鏂板缓鐨?cgroup 璋冪敤銆傦級

`int can_attach(struct cgroup **cgrp, struct cgroup_taskset **tset)`
(cgroup_mutex held by caller)

鍦ㄥ皢涓€涓垨澶氫釜浠诲姟绉诲叆 cgroup 涔嬪墠璋冪敤锛涘鏋滃瓙绯荤粺杩斿洖閿欒锛岃繖灏嗕腑姝㈤檮鍔犳搷浣溿€侤tset 鍖呭惈寰呴檮鍔犵殑浠诲姟锛屼笖淇濊瘉鍏朵腑鑷冲皯鏈変竴涓换鍔°€?
濡傛灉 taskset 涓湁澶氫釜浠诲姟锛屽垯锛?  - 淇濊瘉閮芥潵鑷悓涓€涓嚎绋嬬粍
  - @tset 鍖呭惈璇ョ嚎绋嬬粍涓殑鎵€鏈変换鍔★紝鏃犺瀹冧滑鏄惁姝ｅ湪鍒囨崲 cgroup
  - 绗竴涓换鍔℃槸缁勯暱锛坙eader锛?
姣忎釜 @tset 鏉＄洰杩樺寘鍚换鍔＄殑鏃?cgroup锛岃€屽苟鏈垏鎹?cgroup 鐨勪换鍔″彲浠ヤ娇鐢?cgroup_taskset_for_each() 杩唬鍣ㄨ交鏉捐烦杩囥€傛敞鎰忥紝鍦?fork 鏃朵笉浼氳皟鐢ㄦ湰鏂规硶銆傚鏋滄湰鏂规硶杩斿洖 0锛堟垚鍔燂級锛岄偅涔堝綋璋冪敤鑰呮寔鏈?cgroup_mutex 鏃舵鏈夋晥鎬у簲淇濇寔涓嶅彉锛屽苟涓斾繚璇佸皢鏉ヤ細璋冪敤 attach() 鎴?cancel_attach() 涔嬩竴銆?
`void css_reset(struct cgroup_subsys_state *css)`
(cgroup_mutex held by caller)

涓€涓彲閫夋搷浣滐紝搴斿皢 @css 鐨勯厤缃仮澶嶅埌鍒濆鐘舵€併€傜洰鍓嶅畠浠呯敤浜庣粺涓€灞傜骇锛坲nified hierarchy锛夛紝鍗冲綋鏌愪釜瀛愮郴缁熼€氳繃 "cgroup.subtree_control" 鍦ㄦ煇 cgroup 涓婅绂佺敤銆佷絾鍥犱负鍏朵粬瀛愮郴缁熶緷璧栧畠鑰屽簲淇濇寔鍚敤鏃躲€俢group 鏍稿績浼氶€氳繃绉婚櫎鍏宠仈鐨勬帴鍙ｆ枃浠朵娇杩欐牱鐨?css 涓嶅彲瑙侊紝骞惰皟鐢ㄦ鍥炶皟锛屼互渚胯闅愯棌鐨勫瓙绯荤粺鍙互鍥炲埌鍒濆鐨勪腑鎬х姸鎬併€傝繖鍙互闃绘鏉ヨ嚜闅愯棌 css 鐨勬剰澶栬祫婧愭帶鍒讹紝骞剁‘淇濋厤缃湪鏃ュ悗鍐嶆鍙鏃跺浜庡垵濮嬬姸鎬併€?
`void cancel_attach(struct cgroup **cgrp, struct cgroup_taskset **tset)`
(cgroup_mutex held by caller)

鍦?can_attach() 宸叉垚鍔熴€佷絾浠诲姟闄勫姞鎿嶄綔澶辫触鏃惰璋冪敤銆傚鏋滄煇涓瓙绯荤粺鐨?can_attach() 鏈夊壇浣滅敤锛屽簲鎻愪緵姝ゅ嚱鏁帮紝浠ヤ究璇ュ瓙绯荤粺鑳藉瀹炵幇鍥炴粴銆傝嫢娌℃湁鍓綔鐢ㄥ垯鏃犻渶鎻愪緵銆傛湰鍑芥暟鍙細閽堝 can_attach() 鎿嶄綔宸叉垚鍔熺殑瀛愮郴缁熻皟鐢ㄣ€傚弬鏁颁笌 can_attach() 鐩稿悓銆?
`void attach(struct cgroup **cgrp, struct cgroup_taskset **tset)`
(cgroup_mutex held by caller)

鍦ㄤ换鍔″凡闄勫姞鍒?cgroup 涔嬪悗璋冪敤锛屼互鍏佽浠讳綍闇€瑕佸唴瀛樺垎閰嶆垨闃诲鐨勯檮鍔犲悗娲诲姩銆傚弬鏁颁笌 can_attach() 鐩稿悓銆?
`void fork(struct task_struct *task)`

褰撲换鍔¤ fork 杩涗竴涓?cgroup 鏃惰皟鐢ㄣ€?
`void exit(struct task_struct *task)`

鍦ㄤ换鍔￠€€鍑烘湡闂磋皟鐢ㄣ€?
`void free(struct task_struct *task)`

鍦?task_struct 琚噴鏀炬椂璋冪敤銆?
`void bind(struct cgroup *root)`
(cgroup_mutex held by caller)

褰撴煇涓?cgroup 瀛愮郴缁熻閲嶆柊缁戝畾鍒颁笉鍚岀殑灞傜骇鍜屾牴 cgroup 鏃惰皟鐢ㄣ€傜洰鍓嶈繖鍙秹鍙婇粯璁ゅ眰绾э紙浠庝笉鍖呭惈瀛?cgroup锛変笌琚垱寤?閿€姣佺殑灞傜骇锛堝洜姝や篃娌℃湁瀛?cgroup锛変箣闂寸殑绉诲姩銆?
## 4. 鎵╁睍灞炴€х殑浣跨敤


cgroup 鏂囦欢绯荤粺鍦ㄥ叾鐩綍鍜屾枃浠朵腑鏀寔鏌愪簺绫诲瀷鐨勬墿灞曞睘鎬с€傚綋鍓嶆敮鎸佺殑绫诲瀷鏈夛細

 - 鍙椾俊浠荤殑锛圶ATTR_TRUSTED锛? - 瀹夊叏鐨勶紙XATTR_SECURITY锛?
浜岃€呴兘闇€瑕?CAP_SYS_ADMIN 鑳藉姏鎵嶈兘璁剧疆銆?
涓?tmpfs 涓竴鏍凤紝cgroup 鏂囦欢绯荤粺涓殑鎵╁睍灞炴€т娇鐢ㄥ唴鏍稿唴瀛樺瓨鍌紝寤鸿灏嗗叾浣跨敤淇濇寔鍦ㄦ渶浣庨檺搴︺€傝繖姝ｆ槸涓轰粈涔堜笉鏀寔鐢ㄦ埛鑷畾涔夋墿灞曞睘鎬х殑鍘熷洜锛屽洜涓轰换浣曠敤鎴烽兘鍙互璁剧疆瀹冧滑锛屼笖鍊肩殑澶у皬娌℃湁闄愬埗銆?
褰撳墠宸茬煡鐨勮鍔熻兘浣跨敤鑰呭寘鎷細SELinux锛堢敤浜庨檺鍒跺鍣ㄤ腑 cgroup 鐨勪娇鐢級鍜?systemd锛堢敤浜庡悇绫诲厓鏁版嵁锛屼緥濡?cgroup 涓殑涓?PID锛坰ystemd 涓烘瘡涓湇鍔″垱寤轰竴涓?cgroup锛夛級銆?
## 5. 闂


```

  Q: what's up with this '/bin/echo' ?
  A: bash's builtin 'echo' command does not check calls to write() against
     errors. If you use it in the cgroup file system, you won't be
     able to tell whether a command succeeded or failed.

  Q: When I attach processes, only the first of the line gets really attached !
  A: We can only return one error code per call to write(). So you should also
     put only ONE PID.

```
