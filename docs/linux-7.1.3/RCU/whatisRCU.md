## What is RCU?  --  "Read, Copy, Update"


璇锋敞鎰忥紝"What is RCU?" LWN 绯诲垪鏄涔?RCU 鐨勭粷浣宠捣鐐癸細

| 1.	What is RCU, Fundamentally?  https://lwn.net/Articles/262464/
| 2.	What is RCU? Part 2: Usage   https://lwn.net/Articles/263130/
| 3.	RCU part 3: the RCU API      https://lwn.net/Articles/264090/
| 4.	The RCU API, 2010 Edition    https://lwn.net/Articles/418853/
| 	2010 Big API Table           https://lwn.net/Articles/419086/
| 5.	The RCU API, 2014 Edition    https://lwn.net/Articles/609904/
|	2014 Big API Table           https://lwn.net/Articles/609973/
| 6.	The RCU API, 2019 Edition    https://lwn.net/Articles/777036/
|	2019 Big API Table           https://lwn.net/Articles/777165/
| 7.	The RCU API, 2024 Edition    https://lwn.net/Articles/988638/
|       2024 Background Information  https://lwn.net/Articles/988641/
|	2024 Big API Table           https://lwn.net/Articles/988666/

瀵逛簬鍋忓ソ瑙嗛鐨勮鑰咃細

| 1.	Unraveling RCU Mysteries: Fundamentals          https://www.linuxfoundation.org/webinars/unraveling-rcu-usage-mysteries
| 2.	Unraveling RCU Mysteries: Additional Use Cases  https://www.linuxfoundation.org/webinars/unraveling-rcu-usage-mysteries-additional-use-cases


浠€涔堟槸 RCU锛?

RCU 鏄竴绉嶅湪 2.5 寮€鍙戝懆鏈熶腑鍔犲叆 Linux 鍐呮牳鐨勫悓姝ユ満鍒讹紝閽堝"璇诲"鍦烘櫙杩涜浜嗕紭鍖栥€傚敖绠?RCU 瀹為檯涓婄浉褰撶畝鍗曪紝浣嗚鏈夋晥浣跨敤瀹冿紝浣犻渶瑕佷互涓嶅悓鐨勬柟寮忔€濊€冧綘鐨勪唬鐮併€傞棶棰樼殑鍙︿竴閮ㄥ垎鍦ㄤ簬涓€绉嶉敊璇亣璁撅紝鍗冲瓨鍦ㄦ弿杩板拰浣跨敤 RCU 鐨?鍞竴姝ｇ‘鏂瑰紡"銆傜浉鍙嶏紝缁忛獙琛ㄦ槑锛屼笉鍚岀殑浜哄繀椤昏蛋涓嶅悓鐨勮矾寰勶紝鎵嶈兘杈炬垚瀵?RCU 鐨勭悊瑙ｏ紝杩欏彇鍐充簬浠栦滑鐨勭粡楠屽拰鐢ㄤ緥銆傛湰鏂囨。鎻愪緵浜嗗涓嬪嚑鏉′笉鍚岀殑璺緞锛?

1.	RCU 姒傝堪 <1_whatisRCU>

2.	RCU 鐨勬牳蹇?API 鏄粈涔堬紵 <2_whatisRCU>

3.	鏍稿績 RCU API 鐨勪竴浜涚ず渚嬬敤娉?<3_whatisRCU>

4.	濡傛灉鎴戠殑鏇存柊绾跨▼涓嶈兘闃诲鎬庝箞鍔烇紵 <4_whatisRCU>

5.	RCU 鐨勪竴浜涚畝鍗曞疄鐜?<5_whatisRCU>

6.	涓庤鍐欓攣瀹氱殑绫绘瘮 <6_whatisRCU>

7.	涓庡紩鐢ㄨ鏁扮殑绫绘瘮 <7_whatisRCU>

8.	RCU API 瀹屾暣鍒楄〃 <8_whatisRCU>

9.	蹇€熸祴楠岀瓟妗?<9_whatisRCU>

鍋忓ソ浠庢蹇垫€ф杩板叆鎵嬬殑浜哄簲閲嶇偣鍏虫敞绗?1 鑺傦紝涓嶈繃澶у鏁拌鑰呭湪鏌愪釜鏃跺埢闃呰鏈妭閮戒細鏈夋墍鏀惰幏銆傚亸濂戒粠鍙互璇曢獙鐨?API 鍏ユ墜鐨勪汉搴旈噸鐐瑰叧娉ㄧ 2 鑺傘€傚亸濂戒粠绀轰緥鐢ㄦ硶鍏ユ墜鐨勪汉搴旈噸鐐瑰叧娉ㄧ 3 鑺傚拰绗?4 鑺傘€傞渶瑕佺悊瑙?RCU 瀹炵幇鐨勪汉搴旈噸鐐瑰叧娉ㄧ 5 鑺傦紝鐒跺悗鍐嶆繁鍏ュ唴鏍告簮浠ｇ爜銆傛渶鎿呴暱閫氳繃绫绘瘮鏉ユ€濊€冪殑浜哄簲閲嶇偣鍏虫敞绗?6 鑺傚拰绗?7 鑺傘€傜 8 鑺備綔涓?docbook API 鏂囨。鐨勭储寮曪紝绗?9 鑺傛槸浼犵粺鐨勭瓟妗堝瘑閽ャ€?

鍥犳锛屼粠瀵逛綘鍜屼綘鐨勫涔犱範鎯渶鏈夋剰涔夌殑鑺傚紑濮嬨€傚鏋滀綘鎯崇煡閬撳叧浜庝竴鍒囩殑鐨勪竴鍒囷紝灏界閫氳鍏ㄦ枃鈥斺€斾絾濡傛灉浣犵湡鐨勬槸杩欑浜猴紝浣犳棭宸茬炕闃呰繃婧愪唬鐮侊紝鍥犺€屾牴鏈笉闇€瑕佹湰鏂囨。銆?-)


### 1.  RCU 姒傝堪


RCU 鑳屽悗鐨勫熀鏈€濇兂鏄皢鏇存柊鎷嗗垎涓?绉婚櫎"鍜?鍥炴敹"涓や釜闃舵銆傜Щ闄ら樁娈电Щ闄ゆ暟鎹粨鏋勪腑瀵规暟鎹」鐨勫紩鐢紙鍙兘閫氳繃灏嗗畠浠浛鎹负杩欎簺鏁版嵁椤圭殑鏂扮増鏈殑寮曠敤鏉ュ疄鐜帮級锛屽苟涓斿彲浠ヤ笌璇昏€呭苟鍙戣繍琛屻€傜Щ闄ら樁娈佃兘澶熶笌璇昏€呭苟鍙戣繍琛岀殑鍘熷洜鏄紝鐜颁唬 CPU 鐨勮涔変繚璇佽鑰呭皢鐪嬪埌鏁版嵁缁撴瀯鐨勬棫鐗堟湰鎴栨柊鐗堟湰锛岃€屼笉浼氱湅鍒伴儴鍒嗘洿鏂扮殑寮曠敤銆傚洖鏀堕樁娈靛畬鎴愬洖鏀讹紙渚嬪閲婃斁锛夊湪绉婚櫎闃舵浠庢暟鎹粨鏋勪腑绉婚櫎鐨勬暟鎹」鐨勫伐浣溿€傜敱浜庡洖鏀舵暟鎹」浼氬共鎵颁换浣曟鍦ㄥ苟鍙戝紩鐢ㄨ繖浜涙暟鎹」鐨勮鑰咃紝鍥炴敹闃舵蹇呴』绛夊埌璇昏€呬笉鍐嶆寔鏈夊杩欎簺鏁版嵁椤圭殑寮曠敤鍚庢墠鑳藉紑濮嬨€?

灏嗘洿鏂版媶鍒嗕负绉婚櫎鍜屽洖鏀朵袱涓樁娈碉紝浣垮緱鏇存柊鑰呭彲浠ョ珛鍗虫墽琛岀Щ闄ら樁娈碉紝骞跺皢鍥炴敹闃舵鎺ㄨ繜鍒扮Щ闄ら樁娈垫湡闂存椿璺冪殑鎵€鏈夎鑰呴兘瀹屾垚涔嬪悗锛屾垨鑰呴€氳繃闃诲鐩村埌瀹冧滑瀹屾垚锛屾垨鑰呴€氳繃娉ㄥ唽涓€涓湪瀹冧滑瀹屾垚涔嬪悗琚皟鐢ㄧ殑鍥炶皟鍑芥暟銆傚彧闇€鑰冭檻鍦ㄧЩ闄ら樁娈垫湡闂存椿璺冪殑璇昏€咃紝鍥犱负浠讳綍鍦ㄧЩ闄ら樁娈典箣鍚庡紑濮嬬殑璇昏€呴兘灏嗘棤娉曡幏寰楀宸茬Щ闄ゆ暟鎹」鐨勫紩鐢紝鍥犳涓嶄細琚洖鏀堕樁娈靛共鎵般€?

鍥犳鍏稿瀷鐨?RCU 鏇存柊搴忓垪澶ц嚧濡備笅锛?

a.	绉婚櫎鎸囧悜鏌愪釜鏁版嵁缁撴瀯鐨勬寚閽堬紝浣垮緱鍚庣画鐨勮鑰呮棤娉曡幏寰楀瀹冪殑寮曠敤銆?

b.	绛夊緟鎵€鏈夊厛鍓嶇殑璇昏€呭畬鎴愬叾 RCU 璇荤涓寸晫鍖恒€?

c.	姝ゆ椂锛屼笉鍙兘鏈変换浣曟寔鏈夊璇ユ暟鎹粨鏋勫紩鐢ㄧ殑璇昏€咃紝鍥犳鐜板湪鍙互瀹夊叏鍦板洖鏀跺畠锛堜緥濡?kfree()锛夈€?

涓婅堪姝ラ (b) 鏄?RCU 寤惰繜閿€姣佽儗鍚庢牳蹇冩€濇兂鐨勪綋鐜般€傝兘澶熺瓑寰呮墍鏈夎鑰呭畬鎴愶紝浣垮緱 RCU 璇昏€呭彲浠ヤ娇鐢ㄦ洿杞婚噺绾х殑鍚屾锛屽湪鏌愪簺鎯呭喌涓嬶紝瀹屽叏涓嶉渶瑕佷换浣曞悓姝ャ€傜浉姣斾箣涓嬶紝鍦ㄦ洿浼犵粺鐨勫熀浜庨攣鐨勬柟妗堜腑锛岃鑰呭繀椤讳娇鐢ㄩ噸閲忕骇鍚屾锛屼互闃叉鏇存柊鑰呭皢鏁版嵁缁撴瀯浠庝粬浠剼涓嬪垹闄ゃ€傝繖鏄洜涓哄熀浜庨攣鐨勬洿鏂拌€呴€氬父鍦ㄥ師鍦版洿鏂版暟鎹」锛屽洜姝ゅ繀椤绘帓闄よ鑰呫€傜浉姣斾箣涓嬶紝鍩轰簬 RCU 鐨勬洿鏂拌€呴€氬父鍒╃敤杩欐牱涓€涓簨瀹烇細鍦ㄧ幇浠?CPU 涓婏紝瀵瑰崟涓榻愭寚閽堢殑鍐欏叆鏄師瀛愮殑锛屼粠鑰屽彲浠ュ湪涓嶅共鎵拌鑰呯殑鎯呭喌涓嬶紝鍘熷瓙鍦版彃鍏ャ€佺Щ闄ゅ拰鏇挎崲閾捐〃缁撴瀯涓殑鏁版嵁椤广€傚苟鍙戠殑 RCU 璇昏€呭彲浠ョ户缁闂棫鐗堟湰锛屽苟涓斿彲浠ョ渷鍘诲湪褰撲粖 SMP 璁＄畻鏈虹郴缁熶笂浠ｄ环楂樻槀鐨勫師瀛愭搷浣溿€佸唴瀛樺睆闅滃拰閫氫俊缂撳瓨鏈懡涓紝鍗充究鍦ㄦ病鏈夐攣绔炰簤鐨勬儏鍐典笅涔熸槸濡傛銆?

鍦ㄤ笂闈㈡墍绀虹殑涓夋娴佺▼涓紝鏇存柊鑰呭悓鏃舵墽琛岀Щ闄ゅ拰鍥炴敹姝ラ锛屼絾璁╀竴涓畬鍏ㄤ笉鍚岀殑绾跨▼鏉ユ墽琛屽洖鏀堕€氬父寰堟湁甯姪锛孡inux 鍐呮牳鐨勭洰褰曢」缂撳瓨锛坉cache锛夊疄闄呬笂灏辨槸杩欑鎯呭喌銆傚嵆浣垮悓涓€涓嚎绋嬫墽琛屾洿鏂版楠わ紙涓婇潰鐨勬楠?(a)锛夊拰鍥炴敹姝ラ锛堜笂闈㈢殑姝ラ (c)锛夛紝灏嗕簩鑰呭垎寮€鎬濊€冮€氬父涔熷緢鏈夊府鍔┿€備緥濡傦紝RCU 璇昏€呭拰鏇存柊鑰呮牴鏈笉闇€瑕侀€氫俊锛屼絾 RCU 鍦ㄨ鑰呭拰鍥炴敹鑰呬箣闂存彁渚涗簡闅愬紡鐨勪綆寮€閿€閫氫俊锛屽嵆涓婇潰鐨勬楠?(b)銆?

閭ｄ箞锛屾棦鐒惰鑰呮病鏈夋墽琛屼换浣曞悓姝ユ搷浣滐紝鍥炴敹鑰呭埌搴曟€庝箞鐭ラ亾璇昏€呬綍鏃跺畬鎴愬憿锛熻缁х画闃呰锛屼簡瑙?RCU 鐨?API 濡備綍璁╄繖涓€鍒囧彉寰楃畝鍗曘€?


### 2.  RCU 鐨勬牳蹇?API 鏄粈涔堬紵


鏍稿績 RCU API 闈炲父灏忥細

a.	rcu_read_lock()
b.	rcu_read_unlock()
c.	synchronize_rcu() / call_rcu()
d.	rcu_assign_pointer()
e.	rcu_dereference()

RCU API 杩樻湁璁稿鍏朵粬鎴愬憳锛屼絾鍏朵綑鎴愬憳閮藉彲浠ョ敤杩欎簲涓潵琛ㄧず锛屼笉杩囧ぇ澶氭暟瀹炵幇杞€岀敤 call_rcu() 鍥炶皟 API 鏉ヨ〃绀?synchronize_rcu()銆?

涓嬮潰鎻忚堪杩欎簲涓牳蹇?RCU API锛屽彟澶?18 涓◢鍚庡垪涓俱€傛洿澶氫俊鎭鍙傞槄鍐呮牳 docbook 鏂囨。锛屾垨鐩存帴鏌ョ湅鍑芥暟澶存敞閲娿€?

##### rcu_read_lock()

	void rcu_read_lock(void);

	杩欎竴鏃舵€佸師璇敱璇昏€呯敤鏉ュ憡鐭ュ洖鏀惰€咃紝璇ヨ鑰呮鍦ㄨ繘鍏ヤ竴涓?RCU 璇荤涓寸晫鍖恒€傚湪 RCU 璇荤涓寸晫鍖哄唴闃诲鏄潪娉曠殑锛屼笉杩囦娇鐢?CONFIG_PREEMPT_RCU 鏋勫缓鐨勫唴鏍稿彲浠ユ姠鍗?RCU 璇荤涓寸晫鍖恒€傚湪 RCU 璇荤涓寸晫鍖哄唴璁块棶鐨勪换浣?RCU 淇濇姢鐨勬暟鎹粨鏋勶紝閮戒繚璇佸湪璇ヤ复鐣屽尯鐨勬暣涓寔缁湡闂翠繚鎸佹湭琚洖鏀躲€傚紩鐢ㄨ鏁板彲浠ヤ笌 RCU 缁撳悎浣跨敤锛屼互缁存姢瀵规暟鎹粨鏋勭殑鏇撮暱鏈熷紩鐢ㄣ€?

	璇锋敞鎰忥紝浠讳綍绂佺敤搴曞崐閮ㄣ€佹姠鍗犳垨涓柇鐨勬搷浣滐紝涔熷悓鏍疯繘鍏ヤ簡涓€涓?RCU 璇荤涓寸晫鍖恒€傝幏鍙栬嚜鏃嬮攣涔熷悓鏍疯繘鍏ヤ竴涓?RCU 璇荤涓寸晫鍖猴紝鍗充究鏄偅浜涗笉绂佺敤鎶㈠崰鐨勮嚜鏃嬮攣涔熸槸濡傛锛屽湪浣跨敤 CONFIG_PREEMPT_RT=y 鏋勫缓鐨勫唴鏍镐腑灏辨槸杩欑鎯呭喌銆傜潯鐪犻攣 **骞朵笉** 杩涘叆 RCU 璇荤涓寸晫鍖恒€?

##### rcu_read_unlock()

	void rcu_read_unlock(void);

	杩欎竴鏃舵€佸師璇敱璇昏€呯敤鏉ュ憡鐭ュ洖鏀惰€咃紝璇ヨ鑰呮鍦ㄩ€€鍑轰竴涓?RCU 璇荤涓寸晫鍖恒€備换浣曞惎鐢ㄥ簳鍗婇儴銆佹姠鍗犳垨涓柇鐨勬搷浣滐紝涔熷悓鏍烽€€鍑轰簡涓€涓?RCU 璇荤涓寸晫鍖恒€傞噴鏀捐嚜鏃嬮攣涔熷悓鏍烽€€鍑轰簡涓€涓?RCU 璇荤涓寸晫鍖恒€?

	璇锋敞鎰忥紝RCU 璇荤涓寸晫鍖哄彲浠ユ槸宓屽鐨勫拰/鎴栭噸鍙犵殑銆?

##### synchronize_rcu()

	void synchronize_rcu(void);

	杩欎竴鏃舵€佸師璇爣璁颁簡鏇存柊鑰呬唬鐮佺殑缁撴潫鍜屽洖鏀惰€呬唬鐮佺殑寮€濮嬨€傚畠閫氳繃闃诲锛岀洿鍒版墍鏈?CPU 涓婃墍鏈夐鍏堝瓨鍦ㄧ殑 RCU 璇荤涓寸晫鍖洪兘宸插畬鎴愭潵瀹炵幇杩欎竴鐐广€傝娉ㄦ剰锛宻ynchronize_rcu() **涓嶄竴瀹?* 浼氱瓑寰呬换浣曞悗缁殑 RCU 璇荤涓寸晫鍖哄畬鎴愩€備緥濡傦紝鑰冭檻涓嬮潰鐨勪唬鐮?
```

	         CPU 0                  CPU 1                 CPU 2
	     ----------------- ------------------------- ---------------
	 1.  rcu_read_lock()
	 2.                    enters synchronize_rcu()
	 3.                                               rcu_read_lock()
	 4.  rcu_read_unlock()
	 5.                     exits synchronize_rcu()
	 6.                                              rcu_read_unlock()

	To reiterate, synchronize_rcu() waits only for ongoing RCU
	read-side critical sections to complete, not necessarily for
	any that begin after synchronize_rcu() is invoked.

	Of course, synchronize_rcu() does not necessarily return
	**immediately** after the last pre-existing RCU read-side critical
	section completes.  For one thing, there might well be scheduling
	delays.  For another thing, many RCU implementations process
	requests in batches in order to improve efficiencies, which can
	further delay synchronize_rcu().

	Since synchronize_rcu() is the API that must figure out when
	readers are done, its implementation is key to RCU.  For RCU
	to be useful in all but the most read-intensive situations,
	synchronize_rcu()'s overhead must also be quite small.

	The call_rcu() API is an asynchronous callback form of
	synchronize_rcu(), and is described in more detail in a later
	section.  Instead of blocking, it registers a function and
	argument which are invoked after all ongoing RCU read-side
	critical sections have completed.  This callback variant is
	particularly useful in situations where it is illegal to block
	or where update-side performance is critically important.

	However, the call_rcu() API should not be used lightly, as use
	of the synchronize_rcu() API generally results in simpler code.
	In addition, the synchronize_rcu() API has the nice property
	of automatically limiting update rate should grace periods
	be delayed.  This property results in system resilience in face
	of denial-of-service attacks.  Code using call_rcu() should limit
	update rate in order to gain this same sort of resilience.  See
	checklist.rst for some approaches to limiting the update rate.

```

##### rcu_assign_pointer()

	void rcu_assign_pointer(p, typeof(p) v);

	鏄殑锛宺cu_assign_pointer() **纭疄** 鏄綔涓轰竴涓畯瀹炵幇鐨勶紝灏界濡傛灉鑳戒互杩欑鏂瑰紡澹版槑涓€涓嚱鏁颁細寰堥叿銆傦紙骞朵笖鏇剧粡鏈夎繃鍏充簬鍚?C 璇█娣诲姞閲嶈浇鍑芥暟鐨勮璁猴紝鎵€浠ヨ皝鐭ラ亾鍛紵锛?

	鏇存柊鑰呬娇鐢ㄨ繖涓€绌洪棿瀹忔潵涓哄彈 RCU 淇濇姢鐨勬寚閽堣祴涓€涓柊鍊硷紝浠ヤ究灏嗗€肩殑鍙樺寲瀹夊叏鍦颁粠鏇存柊鑰呬紶杈剧粰璇昏€呫€傝繖鏄竴涓┖闂达紙鐩稿浜庢椂鎬侊級瀹忋€傚畠涓嶄細姹傚€间负涓€涓彸鍊硷紝浣嗗畠纭疄鎻愪緵浜嗙粰瀹氱紪璇戝櫒鎴?CPU 鏋舵瀯鎵€闇€鐨勪换浣曠紪璇戝櫒鎸囦护鍜屽唴瀛樺睆闅滄寚浠ゃ€傚畠鐨勬帓搴忕壒鎬х浉褰撲簬涓€涓?store-release锛堝瓨鍌?閲婃斁锛夋搷浣滐紝涔熷氨鏄锛岀敤浜庡垵濮嬪寲璇ョ粨鏋勭殑浠讳綍鍏堝墠鐨勫姞杞藉拰瀛樺偍锛岄兘琚帓搴忓湪璇ョ粨鏋勫彂甯冩寚閽堢殑瀛樺偍涔嬪墠銆?

	涔熻鍚屾牱閲嶈鐨勬槸锛宺cu_assign_pointer() 鐢ㄤ簬璁板綍锛?锛夊摢浜涙寚閽堝彈 RCU 淇濇姢锛屼互鍙婏紙2锛夌粰瀹氱粨鏋勫鍏朵粬 CPU 鍙樺緱鍙闂殑鏃堕棿鐐广€傝瘽铏藉姝わ紝rcu_assign_pointer() 鏈€缁忓父鏄€氳繃 _rcu 閾捐〃鎿嶄綔鍘熻锛堜緥濡?list_add_rcu()锛夐棿鎺ヤ娇鐢ㄧ殑銆?

##### rcu_dereference()

	typeof(p) rcu_dereference(p);

	涓?rcu_assign_pointer() 涓€鏍凤紝rcu_dereference() 蹇呴』浣滀负涓€涓畯鏉ュ疄鐜般€?

	璇昏€呬娇鐢ㄧ┖闂村畯 rcu_dereference() 鏉ヨ幏鍙栦竴涓彈 RCU 淇濇姢鐨勬寚閽堬紝瀹冭繑鍥炰竴涓殢鍚庡彲浠ュ畨鍏ㄨВ寮曠敤鐨勬寚閽堝€笺€傝娉ㄦ剰锛宺cu_dereference() 瀹為檯涓婂苟涓嶈В寮曠敤璇ユ寚閽堬紝鐩稿弽锛屽畠淇濇姢璇ユ寚閽堜互渚垮悗缁В寮曠敤銆傚畠杩樹负缁欏畾鐨?CPU 鏋舵瀯鎵ц浠讳綍鎵€闇€鐨勫唴瀛樺睆闅滄寚浠ゃ€傜洰鍓嶏紝鍙湁 Alpha 闇€瑕佸湪 rcu_dereference() 鍐呴儴浣跨敤鍐呭瓨灞忛殰鈥斺€斿湪鍏朵粬 CPU 涓婏紝瀹冭缂栬瘧涓轰竴涓?volatile 鍔犺浇銆傜劧鑰岋紝娌℃湁涓绘祦 C 缂栬瘧鍣ㄥ皧閲嶅湴鍧€渚濊禆锛屽洜姝?rcu_dereference() 浣跨敤 volatile 寮哄埗杞崲锛岀粨鍚?rcu_dereference.rst 涓垪鍑虹殑缂栫爜鍑嗗垯锛屽彲浠ラ槻姝㈠綋鍓嶇紪璇戝櫒鐮村潖杩欎簺渚濊禆鍏崇郴銆?

	甯歌鐨勭紪鐮佸疄璺垫槸浣跨敤 rcu_dereference() 灏嗗彈 RCU 淇濇姢鐨勬寚閽堝鍒跺埌涓€涓眬閮ㄥ彉閲忥紝鐒跺悗瑙ｅ紩鐢?
```

		p = rcu_dereference(head.next);
		return p->data;

	However, in this case, one could just as easily combine these
	into one statement::

		return rcu_dereference(head.next)->data;

	If you are going to be fetching multiple fields from the
	RCU-protected structure, using the local variable is of
	course preferred.  Repeated rcu_dereference() calls look
	ugly, do not guarantee that the same pointer will be returned
	if an update happened while in the critical section, and incur
	unnecessary overhead on Alpha CPUs.

	Note that the value returned by rcu_dereference() is valid
	only within the enclosing RCU read-side critical section [1]_.
	For example, the following is **not** legal::

		rcu_read_lock();
		p = rcu_dereference(head.next);
		rcu_read_unlock();
		x = p->address;	/* BUG!!! */
		rcu_read_lock();
		y = p->data;	/* BUG!!! */
		rcu_read_unlock();

	Holding a reference from one RCU read-side critical section
	to another is just as illegal as holding a reference from
	one lock-based critical section to another!  Similarly,
	using a reference outside of the critical section in which
	it was acquired is just as illegal as doing so with normal
	locking.

	As with rcu_assign_pointer(), an important function of
	rcu_dereference() is to document which pointers are protected by
	RCU, in particular, flagging a pointer that is subject to changing
	at any time, including immediately after the rcu_dereference().
	And, again like rcu_assign_pointer(), rcu_dereference() is
	typically used indirectly, via the _rcu list-manipulation
	primitives, such as list_for_each_entry_rcu() [2]_.

```
	鍙?RCU 璇荤涓寸晫鍖轰繚鎶わ紝鍙璇ョ敤娉曞彈鍒版洿鏂颁晶浠ｇ爜鎵€鑾峰彇閿佺殑淇濇姢鍗冲彲銆傝繖涓€鍙樹綋閬垮厤浜嗗湪浣跨敤锛堜緥濡傦級娌℃湁 rcu_read_lock() 淇濇姢鐨?rcu_dereference() 鏃朵細鍙戠敓鐨?lockdep 璀﹀憡銆?
	浣跨敤 rcu_dereference_protected() 杩樻湁涓€涓紭鐐癸紝鍗冲厑璁?rcu_dereference() 蹇呴』绂佹鐨勭紪璇戝櫒浼樺寲銆俽cu_dereference_protected() 鍙樹綋鎺ュ彈涓€涓?lockdep 琛ㄨ揪寮忥紝鐢ㄤ互鎸囩ず璋冪敤鑰呭繀椤昏幏鍙栧摢浜涢攣銆傚鏋滄病鏈夋彁渚涙墍鎸囨槑鐨勪繚鎶わ紝灏变細鍙戝嚭涓€涓?lockdep splat銆傛洿澶氱粏鑺傚拰绀轰緥鐢ㄦ硶璇峰弬闃?Design/Requirements/Requirements.rst 浠ュ強璇?API 鐨勪唬鐮佹敞閲娿€?

	濡傛灉鏌愪釜鎸囬拡鏃㈣鏇存柊渚т唬鐮佷娇鐢紝涔熻 RCU 璇昏€呬娇鐢紝閭ｄ箞鍙互鍚戝畠鐨勫弬鏁板垪琛ㄤ腑娣诲姞涓€涓澶栫殑 lockdep 琛ㄨ揪寮忋€備緥濡傦紝缁欏畾涓€涓澶栫殑 "lock_is_held(&mylock)" 鍙傛暟锛孯CU lockdep 浠ｇ爜灏嗕粎鍦ㄨ瀹炰緥鍦?RCU 璇荤涓寸晫鍖轰箣澶栦笖娌℃湁 mylock 淇濇姢鐨勬儏鍐典笅琚皟鐢ㄦ椂鎵嶄細鎶ヨ銆?

涓嬪浘灞曠ず浜嗘瘡涓?API 濡備綍鍦ㄨ鑰呫€佹洿鏂拌€呭拰鍥炴敹鑰呬箣闂磋繘琛岄€氫俊銆?
```


	    rcu_assign_pointer()
	                            +--------+
	    +---------------------->| reader |---------+
	    |                       +--------+         |
	    |                           |              |
	    |                           |              | Protect:
	    |                           |              | rcu_read_lock()
	    |                           |              | rcu_read_unlock()
	    |        rcu_dereference()  |              |
	    +---------+                 |              |
	    | updater |<----------------+              |
	    +---------+                                V
	    |                                    +-----------+
	    +----------------------------------->| reclaimer |
	                                         +-----------+
	      Defer:
	      synchronize_rcu() & call_rcu()


```
RCU 鍩虹璁炬柦瑙傚療 rcu_read_lock()銆乺cu_read_unlock()銆乻ynchronize_rcu() 鍜?call_rcu() 璋冪敤鐨勬椂鎬佸簭鍒楋紝浠ョ‘瀹氾紙1锛塻ynchronize_rcu() 璋冪敤浣曟椂鍙互杩斿洖缁欏叾璋冪敤鑰咃紝浠ュ強锛?锛塩all_rcu() 鍥炶皟浣曟椂鍙互琚皟鐢ㄣ€俁CU 鍩虹璁炬柦鐨勯珮鏁堝疄鐜板ぇ閲忎娇鐢ㄦ壒澶勭悊锛屼互渚垮皢寮€閿€鍒嗘憡鍒扮浉搴?API 鐨勫娆′娇鐢ㄤ箣涓娿€俽cu_assign_pointer() 鍜?rcu_dereference() 璋冪敤閫氳繃瀵圭浉鍏冲彈 RCU 淇濇姢鐨勬寚閽堣繘琛屽瓨鍌ㄥ拰鍔犺浇鏉ヤ紶杈剧┖闂村彉鍖栥€?

Linux 鍐呮牳涓嚦灏戞湁涓夌 RCU 鐢ㄦ硶椋庢牸銆備笂鍥惧睍绀轰簡鏈€甯歌鐨勪竴绉嶃€傚湪鏇存柊鑰呬竴渚э紝鎵€浣跨敤鐨?rcu_assign_pointer()銆乻ynchronize_rcu() 鍜?call_rcu() 鍘熻瀵硅繖涓夌椋庢牸閮芥槸鐩稿悓鐨勩€傜劧鑰屽浜庝繚鎶わ紙璇昏€呬竴渚э級锛屾墍浣跨敤鐨勫師璇洜椋庢牸鑰屽紓锛?

a.	rcu_read_lock() / rcu_read_unlock()
	rcu_dereference()

b.	rcu_read_lock_bh() / rcu_read_unlock_bh()
	local_bh_disable() / local_bh_enable()
	rcu_dereference_bh()

c.	rcu_read_lock_sched() / rcu_read_unlock_sched()
	preempt_disable() / preempt_enable()
	local_irq_save() / local_irq_restore()
	hardirq enter / hardirq exit
	NMI enter / NMI exit
	rcu_dereference_sched()

杩欎笁绉嶉鏍肩殑浣跨敤鏂瑰紡濡備笅锛?

a.	搴旂敤浜庢櫘閫氭暟鎹粨鏋勭殑 RCU銆?

b.	搴旂敤浜庡彲鑳介伃鍙楄繙绋嬫嫆缁濇湇鍔℃敾鍑荤殑缃戠粶鏁版嵁缁撴瀯鐨?RCU銆?

c.	搴旂敤浜庤皟搴﹀櫒浠ュ強涓柇/NMI 澶勭悊绋嬪簭浠诲姟鐨?RCU銆?

鍚屾牱锛屽ぇ澶氭暟鐢ㄦ硶灞炰簬 (a)銆?b) 鍜?(c) 鐨勬儏鍐靛浜庝笓闂ㄧ敤閫斿緢閲嶈锛屼絾鐩稿灏戣銆係RCU銆丷CU-Tasks銆丷CU-Tasks-Rude 鍜?RCU-Tasks-Trace 鍦ㄥ叾鍚勭鍘熻涔嬮棿鍏锋湁绫讳技鐨勫叧绯汇€?


### 3.  鏍稿績 RCU API 鐨勪竴浜涚ず渚嬬敤娉?


鏈妭灞曠ず浜嗕竴涓娇鐢ㄦ牳蹇?RCU API 鏉ヤ繚鎶ゆ寚鍚戝姩鎬佸垎閰嶇粨鏋勭殑鍏ㄥ眬鎸囬拡鐨勭畝鍗曠ず渚嬨€傛洿鍏稿瀷鐨?RCU 鐢ㄦ硶鍙互鍦?listRCU.rst 鍜?NMI-RCU.rst 涓壘鍒般€?
```

	struct foo {
		int a;
		char b;
		long c;
	};
	DEFINE_SPINLOCK(foo_mutex);

	struct foo __rcu *gbl_foo;

	/*
	 * Create a new struct foo that is the same as the one currently
	 * pointed to by gbl_foo, except that field "a" is replaced
	 * with "new_a".  Points gbl_foo to the new structure, and
	 * frees up the old structure after a grace period.
	 *
	 * Uses rcu_assign_pointer() to ensure that concurrent readers
	 * see the initialized version of the new structure.
	 *
	 * Uses synchronize_rcu() to ensure that any readers that might
	 * have references to the old structure complete before freeing
	 * the old structure.
	 */
	void foo_update_a(int new_a)
	{
		struct foo *new_fp;
		struct foo *old_fp;

		new_fp = kmalloc(sizeof(*new_fp), GFP_KERNEL);
		spin_lock(&foo_mutex);
		old_fp = rcu_dereference_protected(gbl_foo, lockdep_is_held(&foo_mutex));
		*new_fp = *old_fp;
		new_fp->a = new_a;
		rcu_assign_pointer(gbl_foo, new_fp);
		spin_unlock(&foo_mutex);
		synchronize_rcu();
		kfree(old_fp);
	}

	/*
	 * Return the value of field "a" of the current gbl_foo
	 * structure.  Use rcu_read_lock() and rcu_read_unlock()
	 * to ensure that the structure does not get deleted out
	 * from under us, and use rcu_dereference() to ensure that
	 * we see the initialized version of the structure (important
	 * for DEC Alpha and for people reading the code).
	 */
	int foo_get_a(void)
	{
		int retval;

		rcu_read_lock();
		retval = rcu_dereference(gbl_foo)->a;
		rcu_read_unlock();
		return retval;
	}

```
缁间笂鎵€杩帮細

- 浣跨敤 rcu_read_lock() 鍜?rcu_read_unlock() 鏉ヤ繚鎶?RCU 璇荤涓寸晫鍖恒€?

- 鍦?RCU 璇荤涓寸晫鍖哄唴锛屼娇鐢?rcu_dereference() 鏉ヨВ寮曠敤鍙?RCU 淇濇姢鐨勬寚閽堛€?

- 浣跨敤鏌愮绋冲Ε鐨勮璁★紙渚嬪閿佹垨淇″彿閲忥級鏉ラ槻姝㈠苟鍙戞洿鏂扮浉浜掑共鎵般€?

- 浣跨敤 rcu_assign_pointer() 鏉ユ洿鏂板彈 RCU 淇濇姢鐨勬寚閽堛€傝繖涓€鍘熻淇濇姢骞跺彂璇昏€呭厤鍙楁洿鏂拌€呯殑褰卞搷锛?*骞朵笉**淇濇姢骞跺彂鏇存柊褰兼涔嬮棿涓嶅彈褰卞搷锛佸洜姝や綘浠嶇劧闇€瑕佷娇鐢ㄩ攣锛堟垨绫讳技鏈哄埗锛夋潵闃叉骞跺彂鐨?rcu_assign_pointer() 鍘熻鐩镐簰骞叉壈銆?

- 鍦ㄤ粠鍙?RCU 淇濇姢鐨勬暟鎹粨鏋勪腑绉婚櫎鏁版嵁鍏冪礌**涔嬪悗**锛屼絾鍦ㄥ洖鏀?閲婃斁璇ユ暟鎹厓绱?*涔嬪墠**锛屼娇鐢?synchronize_rcu() 鏉ョ瓑寰呮墍鏈夊彲鑳芥鍦ㄥ紩鐢ㄨ鏁版嵁椤圭殑鎵€鏈?RCU 璇荤涓寸晫鍖哄畬鎴愩€?

鏇村浣跨敤 RCU 鏃堕渶瑕侀伒寰殑瑙勫垯锛岃鍙傞槄 checklist.rst銆傚啀娆¤鏄庯紝鏇村吀鍨嬬殑 RCU 鐢ㄦ硶鍙互鍦?listRCU.rst 鍜?NMI-RCU.rst 涓壘鍒般€?


### 4.  濡傛灉鎴戠殑鏇存柊绾跨▼涓嶈兘闃诲鎬庝箞鍔烇紵


鍦ㄤ笂闈㈢殑绀轰緥涓紝foo_update_a() 浼氶樆濉烇紝鐩村埌瀹介檺鏈熻繃鍘汇€傝繖闈炲父绠€鍗曪紝浣嗗湪鏌愪簺鎯呭喌涓嬶紝浜轰滑鏃犳硶鎵垮彈绛夊緟杩欎箞涔呪€斺€斿彲鑳借繕鏈夊叾浠栭珮浼樺厛绾х殑宸ヤ綔瑕佸仛銆?

鍦ㄨ繖绉嶆儏鍐典笅锛屽簲璇ヤ娇鐢?call_rcu() 鑰屼笉鏄?synchronize_rcu()銆?
```

	void call_rcu(struct rcu_head *head, rcu_callback_t func);

```
璇ュ嚱鏁板湪瀹介檺鏈熻繃鍘讳箣鍚庤皟鐢?func(head)銆傝繖涓€璋冪敤鍙兘鍙戠敓鍦ㄨ蒋涓柇鎴栬繘绋嬩笂涓嬫枃涓紝鍥犳璇ュ嚱鏁颁笉鍏佽闃诲銆俧oo 缁撴瀯浣撻渶瑕?
```

	struct foo {
		int a;
		char b;
		long c;
		struct rcu_head rcu;
	};

```
```

	/*
	 * Create a new struct foo that is the same as the one currently
	 * pointed to by gbl_foo, except that field "a" is replaced
	 * with "new_a".  Points gbl_foo to the new structure, and
	 * frees up the old structure after a grace period.
	 *
	 * Uses rcu_assign_pointer() to ensure that concurrent readers
	 * see the initialized version of the new structure.
	 *
	 * Uses call_rcu() to ensure that any readers that might have
	 * references to the old structure complete before freeing the
	 * old structure.
	 */
	void foo_update_a(int new_a)
	{
		struct foo *new_fp;
		struct foo *old_fp;

		new_fp = kmalloc(sizeof(*new_fp), GFP_KERNEL);
		spin_lock(&foo_mutex);
		old_fp = rcu_dereference_protected(gbl_foo, lockdep_is_held(&foo_mutex));
		*new_fp = *old_fp;
		new_fp->a = new_a;
		rcu_assign_pointer(gbl_foo, new_fp);
		spin_unlock(&foo_mutex);
		call_rcu(&old_fp->rcu, foo_reclaim);
	}

```
```

	void foo_reclaim(struct rcu_head *rp)
	{
		struct foo *fp = container_of(rp, struct foo, rcu);

		foo_cleanup(fp->a);

		kfree(fp);
	}

```
container_of() 鍘熻鏄竴涓畯锛岀粰瀹氫竴涓寚鍚戠粨鏋勪綋鍐呴儴鐨勬寚閽堛€佺粨鏋勪綋鐨勭被鍨嬩互鍙婄粨鏋勪綋鍐呰鎸囧悜鐨勫瓧娈碉紝瀹冭繑鍥炴寚鍚戣缁撴瀯浣撹捣濮嬩綅缃殑鎸囬拡銆?

浣跨敤 call_rcu() 鍏佽 foo_update_a() 鐨勮皟鐢ㄨ€呯珛鍗抽噸鏂拌幏寰楁帶鍒舵潈锛岃€屼笉蹇呰繘涓€姝ユ媴蹇冩柊鏇存柊鍏冪礌鐨勬棫鐗堟湰銆傚畠杩樻竻妤氬湴灞曠ず浜?RCU 鍦ㄦ洿鏂拌€咃紙鍗?foo_update_a()锛夊拰鍥炴敹鑰咃紙鍗?foo_reclaim()锛変箣闂寸殑鍖哄埆銆?

寤鸿鎬荤粨涓庝笂涓€鑺傜浉鍚岋紝鍙槸鎴戜滑鐜板湪浣跨敤 call_rcu() 鑰屼笉鏄?synchronize_rcu()锛?

- 鍦ㄤ粠鍙?RCU 淇濇姢鐨勬暟鎹粨鏋勪腑绉婚櫎鏁版嵁鍏冪礌**涔嬪悗**浣跨敤 call_rcu()锛屼互娉ㄥ唽涓€涓洖璋冨嚱鏁帮紝璇ュ洖璋冨嚱鏁板皢鍦ㄦ墍鏈夊彲鑳芥鍦ㄥ紩鐢ㄨ鏁版嵁椤圭殑鎵€鏈?RCU 璇荤涓寸晫鍖哄畬鎴愪箣鍚庤璋冪敤銆?

濡傛灉 call_rcu() 鐨勫洖璋冩墍鍋氱殑鏃犻潪鏄璇ョ粨鏋勮皟鐢?kfree()锛岄偅涔堝彲浠ヤ娇鐢?kfree_rcu() 鏉ヤ唬鏇?call_rcu()銆?
```

	kfree_rcu(old_fp, rcu);

```
濡傛灉鍏佽鍋跺皵鐫＄湢锛屽垯鍙互浣跨敤鍗曞弬鏁板舰寮忥紝浠庤€屼粠 struct foo 涓渷鐣?rcu_head 缁撴瀯銆?

	kfree_rcu_mightsleep(old_fp);

杩欎竴鍙樹綋鍑犱箮浠庝笉闃诲锛屼絾鍙兘鍥犲唴瀛樺垎閰嶅け璐ヨ€岄€氳繃璋冪敤 synchronize_rcu() 鏉ラ樆濉炪€?

鍚屾牱锛岃鍙傞槄 checklist.rst 浜嗚В浣跨敤 RCU 鐨勫叾浠栬鍒欍€?


### 5.  RCU 鏈夊摢浜涚畝鍗曠殑瀹炵幇锛?


RCU 鐨勫ソ澶勪箣涓€鍦ㄤ簬锛屽畠鏈夋瀬鍏剁畝鍗曠殑"鐜╁叿"瀹炵幇锛屾槸鐞嗚В Linux 鍐呮牳涓敓浜х骇瀹炵幇鐨勪竴涓壇濂界涓€姝ャ€傛湰鑺傜粰鍑?RCU 鐨勪袱涓繖鏍风殑"鐜╁叿"瀹炵幇锛屼竴涓熀浜庣啛鎮夌殑閿佸畾鍘熻锛屽彟涓€涓洿鎺ヨ繎浜?缁忓吀"RCU銆備簩鑰呴兘杩囦簬绠€鍗曪紝鏃犳硶鐢ㄤ簬鐪熷疄涓栫晫锛屾棦缂轰箯鍔熻兘涔熺己涔忔€ц兘銆備笉杩囷紝瀹冧滑鏈夊姪浜庝綋浼?RCU 鐨勫伐浣滄柟寮忋€傛湁鍏崇敓浜х骇瀹炵幇锛岃鍙傞槄 kernel/rcu/update.c锛屽苟鍙傞槄锛?

	https://docs.google.com/document/d/1X0lThx8OK0ZgLMqVoXiR4ZrGURHrXK6NyLRbeXe3Xac/edit

浠ヤ簡瑙ｆ弿杩?Linux 鍐呮牳 RCU 瀹炵幇鐨勮鏂囥€侽LS'01 鍜?OLS'02 璁烘枃鏄緢濂界殑鍏ラ棬锛岃€屽浣嶈鏂囨彁渚涗簡鎴嚦 2004 骞村垵褰撳墠瀹炵幇鐨勬洿澶氱粏鑺傘€?


##### 5A.  "鐜╁叿"瀹炵幇 #1锛氶攣瀹?


鏈妭缁欏嚭涓€涓熀浜庣啛鎮夐攣瀹氬師璇殑"鐜╁叿"RCU 瀹炵幇銆傚畠鐨勫紑閿€浣垮緱瀹冩棤娉曠敤浜庡疄闄呭満鏅紝缂轰箯鍙墿灞曟€т篃鏄竴鏍枫€傚畠涔熶笉閫傚悎瀹炴椂浣跨敤锛屽洜涓哄畠鍏佽璋冨害寤惰繜浠庝竴涓绔复鐣屽尯"娓楅€?鍒板彟涓€涓€傚畠杩樺亣璁句簡閫掑綊鐨勮鍐欓攣锛氬鏋滀綘鍦ㄩ潪閫掑綊閿佷笂灏濊瘯杩欐牱鍋氾紝骞朵笖鍏佽宓屽鐨?rcu_read_lock() 璋冪敤锛屽氨鍙兘鍙戠敓姝婚攣銆?

涓嶈繃锛屽畠鍙兘鏄渶瀹规槗涓庝箣寤虹珛鑱旂郴鐨勫疄鐜颁簡锛屽洜姝ゆ槸涓€涓壇濂界殑璧风偣銆?
```

	static DEFINE_RWLOCK(rcu_gp_mutex);

	void rcu_read_lock(void)
	{
		read_lock(&rcu_gp_mutex);
	}

	void rcu_read_unlock(void)
	{
		read_unlock(&rcu_gp_mutex);
	}

	void synchronize_rcu(void)
	{
		write_lock(&rcu_gp_mutex);
		smp_mb__after_spinlock();
		write_unlock(&rcu_gp_mutex);
	}

```
锛堜綘鍙互蹇界暐 rcu_assign_pointer() 鍜?rcu_dereference() 鑰屼笉浼氶敊杩囧お澶氥€備笉杩囪繖閲岃繕鏄粰鍑虹畝鍖栫増鏈€傝€屼笖鏃犺浣犲仛浠€涔堬紝
```

	#define rcu_assign_pointer(p, v) \
	({ \
		smp_store_release(&(p), (v)); \
	})

	#define rcu_dereference(p) \
	({ \
		typeof(p) _________p1 = READ_ONCE(p); \
		(_________p1); \
	})


```
rcu_read_lock() 鍜?rcu_read_unlock() 鍘熻璇诲彇-鑾峰彇骞堕噴鏀句竴涓叏灞€璇诲啓閿併€俿ynchronize_rcu() 鍘熻鍐欏叆-鑾峰彇鍚屼竴涓攣锛岀劧鍚庨噴鏀惧畠銆傝繖鎰忓懗鐫€锛屼竴鏃?synchronize_rcu() 閫€鍑猴紝鍦?synchronize_rcu() 琚皟鐢ㄤ箣鍓嶆墍鏈夋鍦ㄨ繘琛岀殑 RCU 璇荤涓寸晫鍖洪兘淇濊瘉宸茬粡瀹屾垚鈥斺€斿惁鍒?synchronize_rcu() 涓嶅彲鑳藉啓鍏?鑾峰彇璇ラ攣銆俿mp_mb__after_spinlock() 灏?synchronize_rcu() 鎻愬崌涓轰竴涓畬鍏ㄥ唴瀛樺睆闅滐紝浠ョ鍚堜互涓嬫枃妗ｄ腑鍒楀嚭鐨?鍐呭瓨灞忛殰淇濊瘉"锛?

	Design/Requirements/Requirements.rst

鐢变簬璇诲啓閿佸彲浠ヨ閫掑綊鑾峰彇锛屽洜姝ゅ祵濂?rcu_read_lock() 鏄彲鑳界殑銆傝繕瑕佹敞鎰忥紝rcu_read_lock() 涓嶄細姝婚攣锛堣繖鏄?RCU 鐨勪竴涓噸瑕佸睘鎬э級銆傚師鍥犲湪浜庯紝鍞竴鑳藉闃诲 rcu_read_lock() 鐨勬槸 synchronize_rcu()銆備絾 synchronize_rcu() 鍦ㄦ寔鏈?rcu_gp_mutex 鏃朵笉浼氳幏鍙栦换浣曢攣锛屽洜姝や笉鍙兘褰㈡垚姝婚攣鐜€?


蹇€熸祴楠?#1锛?
		涓轰粈涔堣繖涓鐐瑰緢澶╃湡锛熷湪鐪熷疄涓栫晫鐨?Linux 鍐呮牳涓娇鐢ㄨ绠楁硶鏃讹紝姝婚攣鏄浣曞彂鐢熺殑锛熷張璇ュ浣曢伩鍏嶈繖绉嶆閿侊紵

蹇€熸祴楠岀瓟妗?<9_whatisRCU>


##### 5B.  "鐜╁叿"绀轰緥 #2锛氱粡鍏?RCU


鏈妭缁欏嚭涓€涓熀浜?缁忓吀 RCU"鐨?鐜╁叿"RCU 瀹炵幇銆傚畠鍦ㄦ€ц兘涓婏紙浣嗕粎閽堝鏇存柊锛変互鍙婅濡傜儹鎻掓嫈 CPU 鍜屽湪 CONFIG_PREEMPTION 鍐呮牳涓繍琛岀瓑鐗规€ф柟闈篃寰堟瑺缂恒€俽cu_dereference() 鍜?rcu_assign_pointer() 鐨勫畾涔変笌鍓嶄竴鑺傛墍绀虹浉鍚岋紝鍥犳姝ゅ鐪佺暐銆?
```

	void rcu_read_lock(void) { }

	void rcu_read_unlock(void) { }

	void synchronize_rcu(void)
	{
		int cpu;

		for_each_possible_cpu(cpu)
			run_on(cpu);
	}

```
璇锋敞鎰忥紝rcu_read_lock() 鍜?rcu_read_unlock() 缁濆浠€涔堥兘涓嶅仛銆傝繖鏄粡鍏?RCU 鍦ㄩ潪鎶㈠崰寮忓唴鏍镐腑鐨勫法澶т紭鍔匡細璇荤寮€閿€鎭板ソ涓洪浂锛岃嚦灏戝湪闈?Alpha CPU 涓婃槸濡傛銆傝€屼笖 rcu_read_lock() 缁濆涓嶅彲鑳藉弬涓庢閿佺幆锛?

synchronize_rcu() 鐨勫疄鐜板彧鏄緷娆″湪姣忎釜 CPU 涓婅皟搴﹁嚜韬€俽un_on() 鍘熻鍙互鍒╃敤 sched_setaffinity() 鍘熻鐩存帴瀹炵幇銆傚綋鐒讹紝涓€涓笉閭ｄ箞"鐜╁叿"鐨勫疄鐜颁細鍦ㄥ畬鎴愭椂鎭㈠浜插拰鎬э紝鑰屼笉鏄鎵€鏈変换鍔￠兘鐣欏湪鏈€鍚庝竴涓?CPU 涓婅繍琛岋紝浣嗗綋鎴戣"鐜╁叿"鏃讹紝鎴戞槸璇?*鐜╁叿**锛?

閭ｄ箞锛岃繖鍒板簳鏄€庝箞宸ヤ綔鐨勶紵锛燂紵

璇疯浣忥紝鍦?RCU 璇荤涓寸晫鍖哄唴闃诲鏄潪娉曠殑銆傚洜姝わ紝濡傛灉鏌愪釜 CPU 鎵ц浜嗕竴娆′笂涓嬫枃鍒囨崲锛屾垜浠氨鐭ラ亾瀹冨繀瀹氬凡缁忓畬鎴愪簡鎵€鏈夊厛鍓嶇殑 RCU 璇荤涓寸晫鍖恒€備竴鏃?*鎵€鏈?* CPU 閮芥墽琛屼簡涓€娆′笂涓嬫枃鍒囨崲锛岄偅涔?*鎵€鏈?*鍏堝墠鐨?RCU 璇荤涓寸晫鍖哄氨閮藉凡瀹屾垚浜嗐€?

鍥犳锛屽亣璁炬垜浠粠鍏剁粨鏋勪腑绉婚櫎涓€涓暟鎹」锛岀劧鍚庤皟鐢?synchronize_rcu()銆備竴鏃?synchronize_rcu() 杩斿洖锛屾垜浠氨淇濊瘉娌℃湁浠讳綍 RCU 璇荤涓寸晫鍖烘寔鏈夊璇ユ暟鎹」鐨勫紩鐢紝鍥犳鎴戜滑鍙互瀹夊叏鍦板洖鏀跺畠銆?


蹇€熸祴楠?#2锛?
		缁欏嚭涓€涓粡鍏?RCU 璇荤寮€閿€涓?*璐?*鐨勪緥瀛愩€?

蹇€熸祴楠岀瓟妗?<9_whatisRCU>


蹇€熸祴楠?#3锛?
		濡傛灉鍦?RCU 璇荤涓寸晫鍖哄唴闃诲鏄潪娉曠殑锛岄偅涔堝湪 CONFIG_PREEMPT_RT 涓綘鍒板簳璇ユ€庝箞鍋氾紵鍦ㄩ偅閲屾櫘閫氱殑鑷棆閿佷篃浼氶樆濉烇紵锛燂紵

蹇€熸祴楠岀瓟妗?<9_whatisRCU>


### 6.  涓庤鍐欓攣瀹氱殑绫绘瘮


灏界 RCU 鍙互浠ヨ澶氫笉鍚岀殑鏂瑰紡浣跨敤锛屼絾 RCU 鐨勪竴绉嶉潪甯稿父瑙佺殑鐢ㄦ硶绫讳技浜庤鍐欓攣瀹氥€備笅闈㈢殑缁熶竴 diff 灞曠ず浜?RCU 涓庤鍐欓攣瀹氬彲浠ユ湁澶氫箞瀵嗗垏鐨勫叧绯汇€?
```

	@@ -5,5 +5,5 @@ struct el {
	 	int data;
	 	/* Other data fields */
	 };
	-rwlock_t listmutex;
	+spinlock_t listmutex;
	 struct el head;

	@@ -13,15 +14,15 @@
		struct list_head *lp;
		struct el *p;

	-	read_lock(&listmutex);
	-	list_for_each_entry(p, head, lp) {
	+	rcu_read_lock();
	+	list_for_each_entry_rcu(p, head, lp) {
			if (p->key == key) {
				*result = p->data;
	-			read_unlock(&listmutex);
	+			rcu_read_unlock();
				return 1;
			}
		}
	-	read_unlock(&listmutex);
	+	rcu_read_unlock();
		return 0;
	 }

	@@ -29,15 +30,16 @@
	 {
		struct el *p;

	-	write_lock(&listmutex);
	+	spin_lock(&listmutex);
		list_for_each_entry(p, head, lp) {
			if (p->key == key) {
	-			list_del(&p->list);
	-			write_unlock(&listmutex);
	+			list_del_rcu(&p->list);
	+			spin_unlock(&listmutex);
	+			synchronize_rcu();
				kfree(p);
				return 1;
			}
		}
	-	write_unlock(&listmutex);
	+	spin_unlock(&listmutex);
		return 0;
	 }

```
```

 1 struct el {                          1 struct el {
 2   struct list_head list;             2   struct list_head list;
 3   long key;                          3   long key;
 4   spinlock_t mutex;                  4   spinlock_t mutex;
 5   int data;                          5   int data;
 6   /* Other data fields */            6   /* Other data fields */
 7 };                                   7 };
 8 rwlock_t listmutex;                  8 spinlock_t listmutex;
 9 struct el head;                      9 struct el head;

```
```

  1 int search(long key, int *result)    1 int search(long key, int *result)
  2 {                                    2 {
  3   struct list_head *lp;              3   struct list_head *lp;
  4   struct el *p;                      4   struct el *p;
  5                                      5
  6   read_lock(&listmutex);             6   rcu_read_lock();
  7   list_for_each_entry(p, head, lp) { 7   list_for_each_entry_rcu(p, head, lp) {
  8     if (p->key == key) {             8     if (p->key == key) {
  9       *result = p->data;             9       *result = p->data;
 10       read_unlock(&listmutex);      10       rcu_read_unlock();
 11       return 1;                     11       return 1;
 12     }                               12     }
 13   }                                 13   }
 14   read_unlock(&listmutex);          14   rcu_read_unlock();
 15   return 0;                         15   return 0;
 16 }                                   16 }

```
```

  1 int delete(long key)                 1 int delete(long key)
  2 {                                    2 {
  3   struct el *p;                      3   struct el *p;
  4                                      4
  5   write_lock(&listmutex);            5   spin_lock(&listmutex);
  6   list_for_each_entry(p, head, lp) { 6   list_for_each_entry(p, head, lp) {
  7     if (p->key == key) {             7     if (p->key == key) {
  8       list_del(&p->list);            8       list_del_rcu(&p->list);
  9       write_unlock(&listmutex);      9       spin_unlock(&listmutex);
                                        10       synchronize_rcu();
 10       kfree(p);                     11       kfree(p);
 11       return 1;                     12       return 1;
 12     }                               13     }
 13   }                                 14   }
 14   write_unlock(&listmutex);         15   spin_unlock(&listmutex);
 15   return 0;                         16   return 0;
 16 }                                   17 }

```
鏃犺鍝鏂瑰紡锛屽樊鍒兘寰堝皬銆傝绔攣瀹氳浆绉诲埌 rcu_read_lock() 鍜?rcu_read_unlock锛屾洿鏂扮閿佸畾浠庤鍐欓攣杞Щ鍒颁竴涓畝鍗曠殑鑷棆閿侊紝骞朵笖 kfree() 涔嬪墠鏈変竴涓?synchronize_rcu()銆?

涓嶈繃锛屾湁涓€涓綔鍦ㄧ殑闄烽槺锛氳绔拰鏇存柊鐨勪复鐣屽尯鐜板湪鍙互骞跺彂杩愯銆傚湪璁稿鎯呭喌涓嬶紝杩欎笉浼氭垚涓洪棶棰橈紝浣嗘棤璁哄浣曢兘蹇呴』浠旂粏妫€鏌ャ€備緥濡傦紝濡傛灉澶氫釜鐙珛鐨勯摼琛ㄦ洿鏂板繀椤昏瑙嗕负鍗曚釜鍘熷瓙鏇存柊锛岄偅涔堣浆鎹负 RCU 灏嗛渶瑕佺壒鍒皬蹇冦€?

姝ゅ锛宻ynchronize_rcu() 鐨勫瓨鍦ㄦ剰鍛崇潃 delete() 鐨?RCU 鐗堟湰鐜板湪鍙兘浼氶樆濉炪€傚鏋滆繖鏄釜闂锛屽彲浠ヤ娇鐢ㄥ熀浜庡洖璋冦€佹案涓嶉樆濉炵殑鏈哄埗锛屽嵆 call_rcu() 鎴?kfree_rcu()锛屾潵浠ｆ浛 synchronize_rcu()銆?


### 7.  涓庡紩鐢ㄨ鏁扮殑绫绘瘮


璇诲啓绫绘瘮锛堢敱涓婁竴鑺傝鏄庯級骞朵笉鎬绘槸鎬濊€冨浣曚娇鐢?RCU 鐨勬渶浣虫柟寮忋€傚彟涓€涓湁鐢ㄧ殑绫绘瘮鏄紝灏?RCU 瑙嗕负瀵瑰彈 RCU 淇濇姢鐨勪竴鍒囦簨鐗╃殑涓€绉嶆湁鏁堝紩鐢ㄨ鏁般€?

寮曠敤璁℃暟閫氬父骞朵笉闃绘琚紩鐢ㄥ璞＄殑鍊煎彂鐢熷彉鍖栵紝浣嗙‘瀹為樆姝㈢被鍨嬪彂鐢熷彉鍖栤€斺€旂壒鍒槸褰撹瀵硅薄鐨勫唴瀛樿閲婃斁骞堕噸鏂板垎閰嶇粰鍏朵粬鐢ㄩ€旀椂鍙戠敓鐨勯偅绉嶆暣浣撶被鍨嬪彉鏇淬€備竴鏃﹁幏寰楀璇ュ璞＄殑绫诲瀷瀹夊叏寮曠敤锛屽氨闇€瑕佹煇绉嶅叾浠栨満鍒舵潵纭繚瀵硅瀵硅薄涓暟鎹殑涓€鑷磋闂€傝繖鍙兘娑夊強鑾峰彇涓€涓嚜鏃嬮攣锛屼絾鍦?RCU 涓紝鍏稿瀷鐨勬柟娉曟槸浣跨敤鍏峰 SMP 鎰熺煡鐨勬搷浣滐紙渚嬪 smp_load_acquire()锛夋潵鎵ц璇诲彇锛屼娇鐢ㄥ師瀛愯-淇敼-鍐欐搷浣滄潵鎵ц鏇存柊锛屽苟鎻愪緵蹇呰鐨勬帓搴忋€俁CU 鎻愪緵浜嗚澶氬唴宓屼簡鎵€闇€鎿嶄綔鍜屾帓搴忕殑鏀寔鍑芥暟锛屼緥濡備笂涓€鑺備腑浣跨敤鐨?list_for_each_entry_rcu() 瀹忋€?

瀵瑰紩鐢ㄨ鏁拌涓烘洿鑱氱劍鐨勭湅娉曟槸锛氬湪 rcu_read_lock() 鍜?rcu_read_unlock() 涔嬮棿锛屼娇鐢?rcu_dereference() 鍦ㄦ爣璁颁负 `__rcu` 鐨勬寚閽堜笂鑾峰彇鐨勪换浣曞紩鐢紝閮藉彲浠ヨ瑙嗕负璇ュ璞＄殑寮曠敤璁℃暟琚复鏃跺鍔犱簡銆傝繖闃叉浜嗗璞℃敼鍙樼被鍨嬨€傝繖绌剁珶鎰忓懗鐫€浠€涔堬紝灏嗗彇鍐充簬瀵硅绫诲瀷瀵硅薄鐨勪竴鑸鏈燂紝浣嗗畠閫氬父鍖呮嫭锛氳嚜鏃嬮攣浠嶇劧鍙互瀹夊叏鍔犻攣銆佹櫘閫氬紩鐢ㄨ鏁板櫒鍙互瀹夊叏鎿嶄綔锛屼互鍙?`__rcu` 鎸囬拡鍙互瀹夊叏瑙ｅ紩鐢ㄣ€?

浜轰滑鍙兘鏈熸湜鍦ㄦ寔鏈?RCU 寮曠敤鐨勫璞′笂鐪嬪埌鐨勪竴浜涙搷浣滃寘鎷細

 - 澶嶅埗鍑虹敱瀵硅薄绫诲瀷淇濊瘉绋冲畾鐨勬暟鎹€?
 - 浣跨敤 kref_get_unless_zero() 鎴栫被浼兼柟娉曡幏鍙栨洿闀挎湡鐨勫紩鐢ㄣ€傚綋鐒讹紝杩欏彲鑳戒細澶辫触銆?
 - 鑾峰彇瀵硅薄涓殑鑷棆閿侊紝骞舵鏌ヨ瀵硅薄鏄惁浠嶇劧鏄湡鏈涚殑瀵硅薄锛屽鏋滄槸锛屽垯鑷敱鍦版搷浣滃畠銆?

RCU 鎻愪緵鐨勫紩鐢ㄤ粎闃叉绫诲瀷鍙樺寲杩欎竴鐞嗚В锛屽湪浣跨敤浠庢爣璁颁负 `SLAB_TYPESAFE_BY_RCU` 鐨?slab 缂撳瓨鍒嗛厤鐨勫璞℃椂灏や负鏄庢樉銆俁CU 鎿嶄綔鍙兘浜х敓瀵规潵鑷绫荤紦瀛樼殑瀵硅薄鐨勫紩鐢紝璇ュ璞″凡琚苟鍙戦噴鏀撅紝骞朵笖鍐呭瓨琚噸鏂板垎閰嶇粰涓€涓畬鍏ㄤ笉鍚岀殑瀵硅薄锛屽敖绠℃槸鍚屼竴绫诲瀷銆傚湪杩欑鎯呭喌涓嬶紝RCU 鐢氳嚦涓嶄繚鎶ゅ璞＄殑韬唤鍏嶉伃鏀瑰彉锛屽彧淇濇姢鍏剁被鍨嬨€傚洜姝ゆ壘鍒扮殑瀵硅薄鍙兘涓嶆槸鏈熸湜鐨勯偅涓紝浣嗗畠浼氭槸涓€涓彲浠ュ畨鍏ㄨ幏鍙栧紩鐢紙浠ュ強闅忓悗鍙兘鑾峰彇鑷棆閿侊級鐨勫璞★紝浠庤€屽厑璁稿悗缁唬鐮佹鏌ヨ韩浠芥槸鍚︾鍚堥鏈熴€備汉浠緢瀹规槗鎯冲湪涓嶅厛鑾峰彇寮曠敤鐨勬儏鍐典笅鐩存帴鑾峰彇鑷棆閿侊紝浣嗕笉骞哥殑鏄紝`SLAB_TYPESAFE_BY_RCU` 瀵硅薄涓殑浠讳綍鑷棆閿侀兘蹇呴』鍦ㄦ瘡娆¤皟鐢?kmem_cache_alloc() 涔嬪悗閲嶆柊鍒濆鍖栵紝杩欎娇寰楁棤寮曠敤鐨勮嚜鏃嬮攣鑾峰彇瀹屽叏涓嶅畨鍏ㄣ€傚洜姝わ紝褰撲娇鐢?`SLAB_TYPESAFE_BY_RCU` 鏃讹紝瑕佹纭湴浣跨敤寮曠敤璁℃暟鍣ㄣ€傚鏋滀娇鐢?refcount_t锛屽垯搴斾娇鐢ㄤ笓闂ㄧ殑 refcount_{add|inc}_not_zero_acquire() 鍜?refcount_set_release() API锛屼互纭繚鍦ㄩ獙璇佸璞¤韩浠藉拰鍒濆鍖栨柊鍒嗛厤瀵硅薄鏃舵搷浣滈『搴忕殑姝ｇ‘鎬с€俽efcount_{add|inc}_not_zero_acquire() 涓殑鑾峰彇鏍呮爮纭繚韬唤妫€鏌ュ彂鐢熷湪寮曠敤璁℃暟琚幏鍙?*涔嬪悗**銆俽efcount_set_release() 搴斿綋鍦ㄤ竴涓柊鍒嗛厤鐨勫璞¤瀹屽叏鍒濆鍖栦箣鍚庤皟鐢紝鍏堕噴鏀炬爡鏍忕‘淇濇柊鍊煎湪寮曠敤璁℃暟鑳借鍏朵粬鐢ㄦ埛鎴愬姛鑾峰彇**涔嬪墠**鍙銆備竴鏃﹁皟鐢ㄤ簡 refcount_set_release()锛岃瀵硅薄灏卞簲琚涓哄鍏朵粬浠诲姟鍙銆?
锛堥偅浜涙効鎰忓湪 kmem_cache 鏋勯€犲嚱鏁颁腑鍒濆鍖栧叾閿佺殑浜猴紝涔熷彲浠ヤ娇鐢ㄩ攣瀹氾紝鍖呮嫭缂撳瓨鍙嬪ソ鐨勯『搴忛攣銆傦級

瀵逛簬浼犵粺鐨勫紩鐢ㄨ鏁扳€斺€斾緥濡?Linux 涓敱 kref 搴撳疄鐜扮殑閭ｇ鈥斺€旈€氬父浼氭湁鍦ㄥ璞＄殑鏈€鍚庝竴涓紩鐢ㄨ涓㈠純鏃惰繍琛岀殑浠ｇ爜銆傚浜?kref锛岃繖灏辨槸浼犵粰 kref_put() 鐨勫嚱鏁般€傚綋浣跨敤 RCU 鏃讹紝杩欐牱鐨勭粓缁撲唬鐮佸繀椤荤瓑鍒版墍鏈夊紩鐢ㄨ瀵硅薄鐨?`__rcu` 鎸囬拡閮藉凡琚洿鏂帮紝骞朵笖瀹介檺鏈熷凡缁忚繃鍘讳箣鍚庢墠鑳借繍琛屻€傛瘡涓€涓墿浣欑殑瀵硅瀵硅薄鐨勫叏灞€鍙鎸囬拡閮藉繀椤昏瑙嗕负涓€涓綔鍦ㄧ殑璁℃暟寮曠敤锛岃€岀粓缁撲唬鐮侀€氬父鍙湪鎵€鏈夐偅浜涙寚閽堥兘琚洿鏀逛箣鍚庯紝浣跨敤 call_rcu() 鏉ヨ繍琛屻€?

瑕佸紕娓呮濡備綍鍦ㄨ繖涓や釜绫绘瘮涔嬮棿鈥斺€斿皢 RCU 瑙嗕负璇诲啓閿侊紝浠ュ強灏?RCU 瑙嗕负寮曠敤璁℃暟绯荤粺鈥斺€斿仛鍑洪€夋嫨锛屽弽鎬濊淇濇姢浜嬬墿鐨勮妯′細鏈夋墍甯姪銆傝鍐欓攣绫绘瘮鐫€鐪间簬鏇村ぇ鐨勫閮ㄥ垎瀵硅薄锛堜緥濡傞摼琛級锛屽苟灞曠ず RCU 濡備綍鍦ㄥ厓绱犺娣诲姞杩涢摼琛ㄤ互鍙婁粠閾捐〃涓Щ闄ゆ椂淇冭繘骞跺彂銆傚紩鐢ㄨ鏁扮被姣旂潃鐪间簬鍗曚釜瀵硅薄锛屽苟鑰冨療瀹冧滑鍦ㄥ叾鎵€灞炵殑鏁翠綋涓浣曡瀹夊叏璁块棶銆?


### 8.  RCU API 瀹屾暣鍒楄〃


RCU API 璁板綍鍦?Linux 鍐呮牳婧愪唬鐮佷腑鐨?docbook 鏍煎紡澶存敞閲婇噷锛屼絾鏈変竴浠藉畬鏁寸殑 API 鍒楄〃浼氬緢鏈夊府鍔╋紝鍥犱负浼间箮鏃犳硶鍦?docbook 涓瀹冧滑杩涜鍒嗙被銆備互涓嬫槸鎸夌被鍒帓鍒楃殑鍒楄〃銆?
```

	list_entry_rcu
	list_entry_lockless
	list_first_entry_rcu
	list_first_or_null_rcu
	list_tail_rcu
	list_next_rcu
	list_next_or_null_rcu
	list_for_each_entry_rcu
	list_for_each_entry_continue_rcu
	list_for_each_entry_from_rcu
	list_for_each_entry_lockless
	hlist_first_rcu
	hlist_next_rcu
	hlist_pprev_rcu
	hlist_for_each_entry_rcu
	hlist_for_each_entry_rcu_notrace
	hlist_for_each_entry_rcu_bh
	hlist_for_each_entry_from_rcu
	hlist_for_each_entry_continue_rcu
	hlist_for_each_entry_continue_rcu_bh
	hlist_nulls_first_rcu
	hlist_nulls_next_rcu
	hlist_nulls_for_each_entry_rcu
	hlist_nulls_for_each_entry_safe
	hlist_bl_first_rcu
	hlist_bl_for_each_entry_rcu

```
```

	rcu_assign_pointer
	rcu_replace_pointer
	INIT_LIST_HEAD_RCU
	list_add_rcu
	list_add_tail_rcu
	list_del_rcu
	list_replace_rcu
	list_splice_init_rcu
	list_splice_tail_init_rcu
	hlist_add_behind_rcu
	hlist_add_before_rcu
	hlist_add_head_rcu
	hlist_add_tail_rcu
	hlist_del_rcu
	hlist_del_init_rcu
	hlist_replace_rcu
	hlist_nulls_del_init_rcu
	hlist_nulls_del_rcu
	hlist_nulls_add_head_rcu
	hlist_nulls_add_tail_rcu
	hlist_nulls_add_fake
	hlists_swap_heads_rcu
	hlist_bl_add_head_rcu
	hlist_bl_del_rcu
	hlist_bl_set_first_rcu

```
```

	Critical sections		Grace period		Barrier

	rcu_read_lock			synchronize_net		rcu_barrier
	rcu_read_unlock			synchronize_rcu
	guard(rcu)()			synchronize_rcu_expedited
	scoped_guard(rcu)		synchronize_rcu_mult
	rcu_dereference			call_rcu
	rcu_dereference_check		call_rcu_hurry
	rcu_dereference_protected	kfree_rcu
	rcu_read_lock_held		kvfree_rcu
	rcu_read_lock_any_held		kfree_rcu_mightsleep
	rcu_pointer_handoff		cond_synchronize_rcu
	unrcu_pointer			cond_synchronize_rcu_full
					cond_synchronize_rcu_expedited
					cond_synchronize_rcu_expedited_full
					get_completed_synchronize_rcu
					get_completed_synchronize_rcu_full
					get_state_synchronize_rcu
					get_state_synchronize_rcu_full
					poll_state_synchronize_rcu
					poll_state_synchronize_rcu_full
					same_state_synchronize_rcu
					same_state_synchronize_rcu_full
					start_poll_synchronize_rcu
					start_poll_synchronize_rcu_full
					start_poll_synchronize_rcu_expedited
					start_poll_synchronize_rcu_expedited_full

```
```

	Critical sections	Grace period		Barrier

	rcu_read_lock_bh	[Same as RCU]		[Same as RCU]
	rcu_read_unlock_bh
	[local_bh_disable]
	[and friends]
	rcu_dereference_bh
	rcu_dereference_bh_check
	rcu_dereference_bh_protected
	rcu_read_lock_bh_held

```
```

	Critical sections	Grace period		Barrier

	rcu_read_lock_sched	[Same as RCU]		[Same as RCU]
	rcu_read_unlock_sched
	[preempt_disable]
	[and friends]
	rcu_read_lock_sched_notrace
	rcu_read_unlock_sched_notrace
	rcu_dereference_sched
	rcu_dereference_sched_check
	rcu_dereference_sched_protected
	rcu_read_lock_sched_held


```
```

	RCU_INIT_POINTER
	RCU_INITIALIZER
	RCU_POINTER_INITIALIZER
	init_rcu_head
	destroy_rcu_head
	init_rcu_head_on_stack
	destroy_rcu_head_on_stack
	SLAB_TYPESAFE_BY_RCU


```
```

	cond_resched_tasks_rcu_qs
	rcu_all_qs
	rcu_softirq_qs_periodic
	rcu_end_inkernel_boot
	rcu_expedite_gp
	rcu_gp_is_expedited
	rcu_unexpedite_gp
	rcu_cpu_stall_reset
	rcu_head_after_call_rcu
	rcu_is_watching


```
```

	rcu_sync_is_idle
	rcu_sync_init
	rcu_sync_enter
	rcu_sync_exit
	rcu_sync_dtor


```
```

	Critical sections	Grace period			Barrier

	N/A			call_rcu_tasks			rcu_barrier_tasks
				synchronize_rcu_tasks


```
```

	Critical sections	Grace period			Barrier

	N/A			synchronize_rcu_tasks_rude	rcu_barrier_tasks_rude
				call_rcu_tasks_rude


```
```

	Critical sections	Grace period			Barrier

	rcu_read_lock_trace	call_rcu_tasks_trace		rcu_barrier_tasks_trace
	rcu_read_unlock_trace	synchronize_rcu_tasks_trace
	guard(rcu_tasks_trace)()
	scoped_guard(rcu_tasks_trace)


```
```
	list_for_each_entry_srcu
	hlist_for_each_entry_srcu


```
```

	Critical sections		Grace period		Barrier

	srcu_read_lock			call_srcu		srcu_barrier
	srcu_read_unlock		synchronize_srcu
	srcu_read_lock_fast		synchronize_srcu_expedited
	srcu_read_unlock_fast		get_state_synchronize_srcu
	srcu_read_lock_nmisafe		start_poll_synchronize_srcu
	srcu_read_unlock_nmisafe	start_poll_synchronize_srcu_expedited
	srcu_read_lock_notrace		poll_state_synchronize_srcu
	srcu_read_unlock_notrace
	srcu_down_read
	srcu_up_read
	srcu_down_read_fast
	srcu_up_read_fast
	guard(srcu)()
	scoped_guard(srcu)
	srcu_read_lock_held
	srcu_dereference
	srcu_dereference_check
	srcu_dereference_notrace
	srcu_read_lock_held


```
```

	DEFINE_SRCU
	DEFINE_STATIC_SRCU
	DEFINE_SRCU_FAST        // for srcu_read_lock_fast() and friends
	DEFINE_STATIC_SRCU_FAST // for srcu_read_lock_fast() and friends
	init_srcu_struct
	init_srcu_struct_fast
	cleanup_srcu_struct
	smp_mb__after_srcu_read_unlock

```
```

	RCU_LOCKDEP_WARN
	rcu_sleep_check

```
```

	rcu_dereference_raw

```
```

	rcu_access_pointer

```
鏈夊叧鏇村淇℃伅锛岃鍙傞槄婧愪唬鐮佷腑鐨勬敞閲婂ご锛堟垨浠庝腑鐢熸垚鐨?docbook锛夈€?

鐒惰€岋紝閴翠簬 Linux 鍐呮牳涓嚦灏戞湁鍥涗釜绯诲垪鐨?RCU API锛屼綘璇ュ浣曢€夋嫨浣跨敤鍝竴涓紵浠ヤ笅鍒楄〃鍙兘浼氭湁甯姪锛?

a.	璇昏€呮槸鍚﹂渶瑕侀樆濉烇紵濡傛灉鏄紝浣犻渶瑕?SRCU銆?

b.	璇昏€呮槸鍚﹂渶瑕侀樆濉烇紝骞朵笖浣犳槸鍦ㄥ仛璺熻釜锛堜緥濡?ftrace 鎴?BPF锛夛紵濡傛灉鏄紝浣犻渶瑕?RCU-tasks銆丷CU-tasks-rude 鍜?鎴?RCU-tasks-trace銆?

c.	閭ｄ箞 -rt 琛ヤ竵闆嗗憿锛熷鏋滆鑰呭湪闈?rt 鍐呮牳涓渶瑕侀樆濉烇紝浣犻渶瑕?SRCU銆傚鏋滆鑰呭湪 -rt 鍐呮牳涓幏鍙栬嚜鏃嬮攣鏃朵細闃诲锛岃€屽湪闈?rt 鍐呮牳涓笉浼氾紝鍒欎笉闇€瑕?SRCU銆傦紙-rt 琛ヤ竵闆嗗皢鑷棆閿佽浆鍙樹负鐫＄湢閿侊紝鍥犳鏈変簡杩欑鍖哄垎銆傦級

d.	浣犳槸鍚﹂渶瑕佸皢 NMI 澶勭悊绋嬪簭銆乭ardirq 澶勭悊绋嬪簭锛屼互鍙婄鐢ㄤ簡鎶㈠崰鐨勪唬鐮佹锛堟棤璁烘槸閫氳繃 preempt_disable()銆乴ocal_irq_save()銆乴ocal_bh_disable() 杩樻槸鍏朵粬鏌愮鏈哄埗锛夎涓烘樉寮忕殑 RCU 璇昏€咃紵濡傛灉鏄紝RCU-sched 璇昏€呮槸鍞竴鍙鐨勯€夋嫨锛屼絾鑷ぇ绾?v4.20 璧凤紝浣犲彲浠ヤ娇鐢ㄥ師鐢熺殑 RCU 鏇存柊鍘熻銆?

e.	浣犳槸鍚﹂渶瑕?RCU 瀹介檺鏈熷湪涓€涓垨澶氫釜 CPU 琚蒋涓柇鍨勬柇鐨勬儏鍐典笅涔熻兘瀹屾垚锛熶緥濡傦紝浣犵殑浠ｇ爜鏄惁浼氬彈鍒板熀浜庣綉缁滅殑鎷掔粷鏈嶅姟鏀诲嚮锛熷鏋滄槸锛屼綘搴旇璺ㄨ鑰呯鐢ㄨ蒋涓柇锛屼緥濡傞€氳繃浣跨敤 rcu_read_lock_bh()銆傝嚜澶х害 v4.20 璧凤紝浣犲彲浠ヤ娇鐢ㄥ師鐢熺殑 RCU 鏇存柊鍘熻銆?

f.	浣犵殑宸ヤ綔璐熻浇鏄惁瀵?RCU 鐨勬櫘閫氫娇鐢ㄨ€岃█鏇存柊杩囦簬瀵嗛泦锛屼絾鍙堜笉閫傚悎鍏朵粬鍚屾鏈哄埗锛熷鏋滄槸锛岃€冭檻 SLAB_TYPESAFE_BY_RCU锛堝畠鏈€鍒濆悕涓?SLAB_DESTROY_BY_RCU锛夈€備絾璇峰姟蹇呭皬蹇冿紒

g.	浣犳槸鍚﹂渶瑕佸湪閭ｄ簺娣遍櫡绌洪棽寰幆銆佸湪鐢ㄦ埛鎬佹墽琛岀殑杩涘叆鎴栭€€鍑烘湡闂淬€佹垨鍦ㄧ绾?CPU 涓婄殑 CPU 涓婏紝璇荤涓寸晫鍖轰篃鍙楀埌灏婇噸锛熷鏋滄槸锛孲RCU 鍜?RCU Tasks Trace 鏄敮涓€鍙鐨勯€夋嫨锛屽叾涓?SRCU 鍦ㄥ嚑涔庢墍鏈夋儏鍐典笅閮借寮虹儓浼樺厛鎺ㄨ崘銆?

h.	鍚﹀垯锛屼娇鐢?RCU銆?

褰撶劧锛岃繖涓€鍒囬兘鍋囪浣犲凡缁忕‘瀹?RCU 纭疄鏄綘宸ヤ綔鐨勬纭伐鍏枫€?


### 9.  蹇€熸祴楠岀瓟妗?


蹇€熸祴楠?#1锛?
		涓轰粈涔堣繖涓鐐瑰緢澶╃湡锛熷湪鐪熷疄涓栫晫鐨?Linux 鍐呮牳涓娇鐢ㄨ绠楁硶鏃讹紝姝婚攣鏄浣曞彂鐢熺殑锛焄鎸囩殑鏄熀浜庨攣鐨?鐜╁叿"RCU 绠楁硶銆俔

绛旀锛?
		鑰冭檻浠ヤ笅浜嬩欢搴忓垪锛?

  1. CPU 0 鑾峰彇鏌愪釜涓嶇浉鍏崇殑閿侊紝绉颁箣涓?
			"problematic_lock"锛岄€氳繃
			spin_lock_irqsave() 绂佺敤 irq銆?

  2. CPU 1 杩涘叆 synchronize_rcu()锛屽啓鍏?鑾峰彇
			rcu_gp_mutex銆?

  3. CPU 0 杩涘叆 rcu_read_lock()锛屼絾蹇呴』绛夊緟锛?
			鍥犱负 CPU 1 鎸佹湁 rcu_gp_mutex銆?

  4. CPU 1 琚腑鏂紝鑰岃 irq 澶勭悊绋嬪簭
			璇曞浘鑾峰彇 problematic_lock銆?

		绯荤粺鐜板湪鍙戠敓浜嗘閿併€?

		閬垮厤杩欑姝婚攣鐨勪竴绉嶆柟娉曟槸閲囩敤绫讳技浜?CONFIG_PREEMPT_RT 鐨勫仛娉曪紝鍗虫墍鏈夋櫘閫氳嚜鏃嬮攣閮藉彉鎴愰樆濉為攣锛屽苟涓旀墍鏈?irq 澶勭悊绋嬪簭閮藉湪鐗规畩浠诲姟鐨勪笂涓嬫枃涓墽琛屻€傚湪杩欑鎯呭喌涓嬶紝鍦ㄤ笂杩扮 4 姝ヤ腑锛宨rq 澶勭悊绋嬪簭浼氶樆濉烇紝浠庤€屽厑璁?CPU 1 閲婃斁 rcu_gp_mutex锛岄伩鍏嶆閿併€?

		鍗充娇娌℃湁姝婚攣锛岃繖绉?RCU 瀹炵幇涔熷厑璁稿欢杩熼€氳繃 synchronize_rcu() 浠庤鑰?娓楅€?鍒板叾浠栬鑰呫€傝鐪嬪嚭杩欎竴鐐癸紝鑰冭檻澶勪簬 RCU 璇荤涓寸晫鍖轰腑鐨勪换鍔?A锛堝洜鑰岃鎸佹湁 rcu_gp_mutex锛夈€佽瘯鍥惧啓鍏?鑾峰彇 rcu_gp_mutex 鑰岃闃诲鐨勪换鍔?B锛屼互鍙婅瘯鍥捐鑾峰彇 rcu_gp_mutex 鑰屽湪 rcu_read_lock() 涓樆濉炵殑浠诲姟 C銆備换鍔?A 鐨?RCU 璇荤寤惰繜姝ｅ湪鎷栦綇浠诲姟 C锛屽敖绠℃槸閫氳繃浠诲姟 B 闂存帴鍋氬埌鐨勩€?

		瀹炴椂 RCU 瀹炵幇鍥犳浣跨敤浜嗕竴绉嶅熀浜庤鏁板櫒鐨勬柟娉曪紝鍏朵腑澶勪簬 RCU 璇荤涓寸晫鍖轰腑鐨勪换鍔′笉浼氳鎵ц synchronize_rcu() 鐨勪换鍔￠樆濉炪€?

鍥炲埌蹇€熸祴楠?#1 <quiz_1>

蹇€熸祴楠?#2锛?
		缁欏嚭涓€涓粡鍏?RCU 璇荤寮€閿€涓?*璐?*鐨勪緥瀛愩€?

绛旀锛?
		璁炬兂涓€涓崟 CPU 绯荤粺锛岃繍琛岄潪 CONFIG_PREEMPTION 鍐呮牳锛屽叾涓矾鐢辫〃鐢辫繘绋嬩笂涓嬫枃浠ｇ爜浣跨敤锛屼絾鍙互琚?irq 涓婁笅鏂囦唬鐮佹洿鏂帮紙渚嬪锛岄€氳繃涓€涓?ICMP REDIRECT"鍖咃級銆傞€氬父鐨勫鐞嗘柟寮忔槸璁╄繘绋嬩笂涓嬫枃浠ｇ爜鍦ㄦ煡鎵捐矾鐢辫〃鏃剁鐢ㄤ腑鏂€備娇鐢?RCU 鍒欏彲浠ョ渷鍘昏繖绉嶇鐢ㄤ腑鏂殑鎿嶄綔銆傚洜姝わ紝娌℃湁 RCU 鏃讹紝浣犺浠樺嚭绂佺敤涓柇鐨勪唬浠凤紱鑰屾湁浜?RCU锛屼綘鍒欎笉闇€瑕併€?

		鏈変汉鍙互浜夎京璇达紝鍦ㄨ繖绉嶆儏鍐典笅 RCU 鐨勫紑閿€鐩稿浜庡崟 CPU 鐨勭鐢ㄤ腑鏂柟妗堟槸璐熺殑銆傚叾浠栦汉鍙兘浼氫簤杈╄锛孯CU 鐨勫紑閿€浠呬粎鏄浂锛岃€岀敤闆跺紑閿€鐨?RCU 鏂规鍙栦唬姝ｅ紑閿€鐨勭鐢ㄤ腑鏂柟妗堝苟涓嶆瀯鎴愯礋寮€閿€銆?

		褰撶劧锛屽湪鐜板疄鐢熸椿涓紝浜嬫儏瑕佸鏉傚緱澶氥€備絾鍗充究鏄竴涓悓姝ュ師璇紑閿€鍙兘涓鸿礋鐨勭悊璁哄彲鑳芥€э紝涔熸湁浜涘嚭浜烘剰鏂欍€?-)

鍥炲埌蹇€熸祴楠?#2 <quiz_2>

蹇€熸祴楠?#3锛?
		濡傛灉鍦?RCU 璇荤涓寸晫鍖哄唴闃诲鏄潪娉曠殑锛岄偅涔堝湪 CONFIG_PREEMPT_RT 涓綘鍒板簳璇ユ€庝箞鍋氾紵鍦ㄩ偅閲屾櫘閫氱殑鑷棆閿佷篃浼氶樆濉烇紵锛燂紵

绛旀锛?
		姝ｅ CONFIG_PREEMPT_RT 鍏佽鎶㈠崰鑷棆閿佷复鐣屽尯涓€鏍凤紝瀹冧篃鍏佽鎶㈠崰 RCU 璇荤涓寸晫鍖恒€傚畠杩樺厑璁稿湪 RCU 璇荤涓寸晫鍖哄唴鑷棆閿侀樆濉炪€?

		涓轰粈涔堝瓨鍦ㄨ繖绉嶆槑鏄剧殑涓嶄竴鑷达紵鍥犱负濡傛灉鏈夐渶瑕侊紙渚嬪鍐呭瓨鐭己鏃讹級锛屾湁鍙兘浣跨敤浼樺厛绾ф彁鍗囨潵淇濇寔 RCU 瀹介檺鏈熻緝鐭€傜浉姣斾箣涓嬶紝濡傛灉闃诲绛夊緟锛堟瘮濡傝锛夌綉缁滄帴鏀讹紝鍒欐棤娉曠煡閬撳簲璇ユ彁鍗囦粈涔堛€傜壒鍒槸鑰冭檻鍒版垜浠渶瑕佹彁鍗囩殑杩涚▼寰堝彲鑳芥槸涓€涓垰鍑哄幓涔版姭钀ㄦ垨浠€涔堢殑娲讳汉銆傝€屼笖灏界璁＄畻鏈烘搷浣滅殑璧剁墰妫掑彲鑳藉紩璧蜂弗閲嶅叴瓒ｏ紝瀹冧篃鍙兘鎷涜嚧涓ラ噸鍙嶅銆傛澶栵紝璁＄畻鏈烘€庝箞鐭ラ亾閭ｄ釜浜哄幓浜嗗摢瀹舵姭钀ㄥ簵锛燂紵锛?

鍥炲埌蹇€熸祴楠?#3 <quiz_3>

鑷磋阿

鎰熻阿閭ｄ簺甯姪浣挎湰鏂囧叿澶囧彲璇绘€х殑浜哄憳锛屽寘鎷?Jon Walpole銆丣osh Triplett銆丼erge Hallyn銆丼uzanne Wood 鍜?Alan Stern銆?


鏇村淇℃伅锛岃鍙傞槄 http://www.rdrop.com/users/paulmck/RCU銆?
