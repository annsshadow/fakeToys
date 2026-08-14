
## RCU 涓?lockdep 妫€鏌?


鎵€鏈夌被鍨嬬殑 RCU 閮藉叿鏈夊彲鐢ㄧ殑 lockdep 妫€鏌ワ紝鍥犳 lockdep 鐭ラ亾姣忎釜浠诲姟浣曟椂杩涘叆鍜岀寮€浠绘剰
绫诲瀷鐨?RCU 璇讳晶涓寸晫鍖恒€傛瘡绉嶇被鍨嬬殑 RCU 琚崟鐙窡韪紙浣嗚娉ㄦ剰鍦?2.6.32 鍙婃洿鏃╃増鏈腑骞堕潪
濡傛锛夈€傝繖浣垮緱 lockdep 鐨勮窡韪彲浠ュ寘鍚?RCU 鐘舵€侊紝鍦ㄨ皟璇曟閿佺瓑鎯呭喌鏃舵湁鏃朵細鏈夋墍甯姪銆?

姝ゅ锛孯CU 鎻愪緵浠ヤ笅妫€鏌?lockdep 鐨勫師璇細
```

	rcu_read_lock_held() for normal RCU.
	rcu_read_lock_bh_held() for RCU-bh.
	rcu_read_lock_sched_held() for RCU-sched.
	rcu_read_lock_any_held() for any of normal RCU, RCU-bh, and RCU-sched.
	srcu_read_lock_held() for SRCU.
	rcu_read_lock_trace_held() for RCU Tasks Trace.

```
杩欎簺鍑芥暟鏄繚瀹堢殑锛屽洜姝ゅ鏋滃畠浠笉纭畾灏变細杩斿洖 1锛堜緥濡傦紝濡傛灉鏈缃?CONFIG_DEBUG_LOCK_ALLOC锛夈€?
杩欏彲浠ラ槻姝㈣濡?WARN_ON(!rcu_read_lock_held()) 鍦?lockdep 琚鐢ㄦ椂缁欏嚭璇姤銆?

姝ゅ锛屼竴涓嫭绔嬬殑鍐呮牳閰嶇疆鍙傛暟 CONFIG_PROVE_RCU 鍚敤瀵?rcu_dereference() 鍘熻鐨勬鏌ワ細

	rcu_dereference(p):
		妫€鏌?RCU 璇讳晶涓寸晫鍖恒€?
	rcu_dereference_bh(p):
		妫€鏌?RCU-bh 璇讳晶涓寸晫鍖恒€?
	rcu_dereference_sched(p):
		妫€鏌?RCU-sched 璇讳晶涓寸晫鍖恒€?
	srcu_dereference(p, sp):
		妫€鏌?SRCU 璇讳晶涓寸晫鍖恒€?
	rcu_dereference_check(p, c):
		浣跨敤鏄惧紡妫€鏌ヨ〃杈惧紡 "c" 浠ュ強 rcu_read_lock_held()銆傝繖鍦ㄦ棦琚?
		RCU 璇昏€呭張琚洿鏂拌€呰皟鐢ㄧ殑浠ｇ爜涓緢鏈夌敤銆?
	rcu_dereference_bh_check(p, c):
		浣跨敤鏄惧紡妫€鏌ヨ〃杈惧紡 "c" 浠ュ強 rcu_read_lock_bh_held()銆傝繖鍦ㄦ棦琚?
		RCU-bh 璇昏€呭張琚洿鏂拌€呰皟鐢ㄧ殑浠ｇ爜涓緢鏈夌敤銆?
	rcu_dereference_sched_check(p, c):
		浣跨敤鏄惧紡妫€鏌ヨ〃杈惧紡 "c" 浠ュ強 rcu_read_lock_sched_held()銆傝繖鍦ㄦ棦琚?
		RCU-sched 璇昏€呭張琚洿鏂拌€呰皟鐢ㄧ殑浠ｇ爜涓緢鏈夌敤銆?
	srcu_dereference_check(p, c):
		浣跨敤鏄惧紡妫€鏌ヨ〃杈惧紡 "c" 浠ュ強 srcu_read_lock_held()銆傝繖鍦ㄦ棦琚?
		SRCU 璇昏€呭張琚洿鏂拌€呰皟鐢ㄧ殑浠ｇ爜涓緢鏈夌敤銆?
	rcu_dereference_raw(p):
		涓嶆鏌ャ€傦紙灏介噺鎱庣敤锛屾渶濂戒笉鐢ㄣ€傦級
	rcu_dereference_raw_check(p):
		瀹屽叏涓嶅仛 lockdep銆傦紙灏介噺鎱庣敤锛屾渶濂戒笉鐢ㄣ€傦級
	rcu_dereference_protected(p, c):
		浣跨敤鏄惧紡妫€鏌ヨ〃杈惧紡 "c"锛屽苟鐪佺暐鎵€鏈夊睆闅滃拰缂栬瘧鍣ㄧ害鏉熴€傝繖鍦ㄦ暟鎹?
		缁撴瀯涓嶈兘鏀瑰彉鏃跺緢鏈夌敤锛屼緥濡傚湪琚粎鐢辨洿鏂拌€呰皟鐢ㄧ殑浠ｇ爜涓€?
	rcu_access_pointer(p):
		杩斿洖鎸囬拡鐨勫€煎苟鐪佺暐鎵€鏈夊睆闅滐紝浣嗕繚鐣欓槻姝㈤噸澶嶆垨鍚堝苟鐨勭紪璇戝櫒绾︽潫銆?
		杩欏湪娴嬭瘯鎸囬拡鏈韩鐨勫€兼椂锛堜緥濡備笌 NULL 姣旇緝锛夊緢鏈夌敤銆?

rcu_dereference_check() 鐨勬鏌ヨ〃杈惧紡鍙互鏄换浣曞竷灏旇〃杈惧紡锛屼絾閫氬父浼氬寘鍚竴涓?lockdep 琛ㄨ揪寮忋€?
瀵逛簬涓€涓?
```

	file = rcu_dereference_check(fdt->fd[fd],
				     lockdep_is_held(&files->file_lock) ||
				     atomic_read(&files->count) == 1);

```
璇ヨ〃杈惧紡浠ヤ竴绉?RCU 瀹夊叏鐨勬柟寮忚幏鍙栨寚閽?"fdt->fd[fd]"锛屽苟涓旓紝濡傛灉閰嶇疆浜?CONFIG_PROVE_RCU锛?
鍒欓獙璇佽琛ㄨ揪寮忚鐢ㄤ簬锛?

1. 涓€涓?RCU 璇讳晶涓寸晫鍖猴紙闅愬紡锛夛紝鎴?
2. 鎸佹湁 files->file_lock锛屾垨
3. 涓€涓湭鍏变韩鐨?files_struct銆?

鍦ㄦ儏鍐?(1) 涓紝鎸囬拡浠?RCU 瀹夊叏鐨勬柟寮忚幏鍙栵紝閫傜敤浜庢櫘閫氱殑 RCU 璇讳晶涓寸晫鍖猴紱鍦ㄦ儏鍐?(2) 涓紝
->file_lock 闃绘浠讳綍鏀瑰彉鍙戠敓锛涙渶鍚庯紝鍦ㄦ儏鍐?(3) 涓紝褰撳墠浠诲姟鏄敮涓€璁块棶璇?file_struct 鐨?
浠诲姟锛屽悓鏍烽樆姝换浣曟敼鍙樺彂鐢熴€傚鏋滀笂杩拌鍙ヤ粎鐢辨洿鏂拌€呰皟鐢?
```

	file = rcu_dereference_protected(fdt->fd[fd],
					 lockdep_is_held(&files->file_lock) ||
					 atomic_read(&files->count) == 1);

```
杩欏皢楠岃瘉涓婇潰鐨?#2 鍜?#3 鎯呭喌锛岃€屼笖 lockdep 杩樹細鎶辨€紝鍗充娇瀹冭鐢ㄤ簬 RCU 璇讳晶涓寸晫鍖猴紝闄ら潪
杩欎袱绉嶆儏鍐典箣涓€鎴愮珛銆傚洜涓?rcu_dereference_protected() 鐪佺暐浜嗘墍鏈夊睆闅滃拰缂栬瘧鍣ㄧ害鏉燂紝瀹冪敓鎴愮殑
浠ｇ爜姣斿叾瀹?rcu_dereference() 鍙樹綋鏇村ソ銆傚彟涓€鏂归潰锛屽鏋滃彈 RCU 淇濇姢鐨勬寚閽堟垨瀹冩寚鍚戠殑鍙?RCU
淇濇姢鐨勬暟鎹彲鑳藉苟鍙戞敼鍙橈紝鍒欎娇鐢?rcu_dereference_protected() 鏄潪娉曠殑銆?

涓?rcu_dereference() 绫讳技锛屽綋鍚敤 lockdep 鏃讹紝RCU 鐨?list 鍜?hlist 閬嶅巻鍘熻浼氭鏌ユ槸鍚︿粠
RCU 璇讳晶涓寸晫鍖哄唴璋冪敤銆傜劧鑰岋紝鍙互鎶婁竴涓?lockdep 琛ㄨ揪寮忎綔涓洪澶栫殑鍙€夊弬鏁颁紶缁欏畠浠€傛湁浜?
杩欎釜 lockdep 琛ㄨ揪寮忥紝杩欎簺閬嶅巻鍘熻浠呭綋 lockdep 琛ㄨ揪寮忎负鍋囦笖瀹冧滑浠庝换浣?RCU 璇讳晶涓寸晫鍖轰箣澶?
璋冪敤鏃舵墠浼氭姳鎬ㄣ€?

渚嬪锛寃orkqueue 鐨?for_each_pwq() 瀹忔棬鍦ㄦ棦鍦?RCU 璇讳晶涓寸晫鍖哄唴浣跨敤锛屼篃鍦ㄦ寔鏈?wq->mutex
鏃朵娇鐢ㄣ€?
```

	#define for_each_pwq(pwq, wq)
		list_for_each_entry_rcu((pwq), &(wq)->pwqs, pwqs_node,
					lock_is_held(&(wq->mutex).dep_map))

```