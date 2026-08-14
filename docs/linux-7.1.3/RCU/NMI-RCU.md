
## 浣跨敤 RCU 淇濇姢鍔ㄦ€?NMI 澶勭悊绋嬪簭

铏界劧 RCU 閫氬父鐢ㄤ簬淇濇姢浠ヨ涓轰富鐨勬暟鎹粨鏋勶紝浣嗕篃鍙互浣跨敤 RCU 鏉ユ彁渚涘姩鎬佺殑
涓嶅彲灞忚斀涓柇锛圢MI锛夊鐞嗙▼搴忥紝浠ュ強鍔ㄦ€佺殑 irq 澶勭悊绋嬪簭銆傛湰鏂囨。鎻忚堪浜嗗浣曞仛鍒?杩欎竴鐐癸紝瀹冨ぇ鑷村€熼壌浜?Zwane Mwaikambo 鍦ㄦ棫鐗堟湰 "arch/x86/kernel/traps.c" 涓殑
NMI-timer 宸ヤ綔銆?
鐩稿叧鐨勪唬鐮佺墖娈靛垪鍦ㄤ笅闈紝姣忎竴娈典箣鍚庨兘闄勬湁
```

	static int dummy_nmi_callback(struct pt_regs *regs, int cpu)
	{
		return 0;
	}

```
dummy_nmi_callback() 鍑芥暟鏄竴涓€渄ummy鈥濓紙绌猴級NMI 澶勭悊绋嬪簭锛屽畠浠€涔堜篃涓嶅仛锛屼絾杩斿洖
闆讹紝浠庤€岃〃绀哄畠浠€涔堜篃娌″仛锛屽苟鍏佽
```

	static nmi_callback_t nmi_callback = dummy_nmi_callback;

```
杩欎釜 nmi_callback 鍙橀噺鏄寚鍚戝綋鍓?NMI 澶勭悊绋嬪簭鐨勫叏灞€鍑芥暟鎸囬拡
```

	void do_nmi(struct pt_regs * regs, long error_code)
	{
		int cpu;

		nmi_enter();

		cpu = smp_processor_id();
		++nmi_count(cpu);

		if (!rcu_dereference_sched(nmi_callback)(regs, cpu))
			default_do_nmi(regs);

		nmi_exit();
	}

```
do_nmi() 鍑芥暟澶勭悊姣忎釜 NMI銆傚畠棣栧厛浠ヤ笌纭欢 irq 鐩稿悓鐨勬柟寮忕鐢ㄦ姠鍗狅紝鐒跺悗閫掑 per-CPU
鐨?NMI 璁℃暟銆傛帴鐫€瀹冭皟鐢ㄥ瓨鍌ㄥ湪 nmi_callback 鍑芥暟鎸囬拡涓殑 NMI 澶勭悊绋嬪簭銆傚鏋滆澶勭悊绋嬪簭
杩斿洖闆讹紝do_nmi() 灏辫皟鐢?default_do_nmi() 鍑芥暟鏉ュ鐞嗘満鍣ㄧ壒瀹氱殑 NMI銆傛渶鍚庯紝鎭㈠鎶㈠崰銆?
鐞嗚涓婏紝rcu_dereference_sched() 骞朵笉鏄繀闇€鐨勶紝鍥犱负杩欐浠ｇ爜鍙繍琛屽湪 i386 涓婏紝鑰?i386
鐞嗚涓婃湰鏉ヤ篃涓嶉渶瑕?rcu_dereference_sched()銆傜劧鑰岋紝鍦ㄥ疄璺典腑瀹冩槸涓€涓緢濂界殑鏂囨。杈呭姪锛?鐗瑰埆鏄浜庨偅浜涜瘯鍥惧湪 Alpha 鎴栦娇鐢ㄤ簡婵€杩涗紭鍖栫紪璇戝櫒鐨勭郴缁熶笂鍋氱被浼间簨鎯呯殑浜恒€?
蹇€熸祴楠岋細
		鑰冭檻鍒版寚閽堟墍寮曠敤鐨勪唬鐮佹槸鍙鐨勶紝涓轰粈涔堝湪 Alpha 涓婂彲鑳戒粛鐒堕渶瑕?		rcu_dereference_sched()锛?
蹇€熸祴楠岀殑绛旀 <answer_quick_quiz_NMI>

```

	void set_nmi_callback(nmi_callback_t callback)
	{
		rcu_assign_pointer(nmi_callback, callback);
	}

```
set_nmi_callback() 鍑芥暟娉ㄥ唽涓€涓?NMI 澶勭悊绋嬪簭銆傛敞鎰忥紝浠讳綍瑕佽鍥炶皟浣跨敤鐨勬暟鎹兘蹇呴』鍦?璋冪敤 set_nmi_callback() *涔嬪墠* 瀹屾垚鍒濆鍖栥€傚湪涓嶅鍐欏叆杩涜鎺掑簭鐨勬灦鏋勪笂锛?rcu_assign_pointer() 纭繚 NMI 澶勭悊绋嬪簭鑳界湅鍒?```

	void unset_nmi_callback(void)
	{
		rcu_assign_pointer(nmi_callback, dummy_nmi_callback);
	}

```
杩欎釜鍑芥暟娉ㄩ攢涓€涓?NMI 澶勭悊绋嬪簭锛屾仮澶嶅師濮嬬殑 dummy_nmi_handler()銆備絾鏄紝鍙兘鎭板ソ鏈夋煇涓?鍏跺畠 CPU 涓婃鍦ㄦ墽琛屼竴涓?NMI 澶勭悊绋嬪簭銆傚洜姝わ紝鍦ㄦ墍鏈夊叾瀹?CPU 涓婅澶勭悊绋嬪簭鎵ц瀹屾瘯涔嬪墠锛?鎴戜滑涓嶈兘閲婃斁鏃х殑 NMI 澶勭悊绋嬪簭鎵€浣跨敤鐨勪换浣曟暟鎹粨鏋勩€?
涓€绉嶅疄鐜版柟寮忔槸鍊熷姪 synchronize_rcu()锛屼緥濡?```

	unset_nmi_callback();
	synchronize_rcu();
	kfree(my_nmi_data);

```
杩欐槸鍙鐨勶紝鍥犱负锛堜粠 v4.20 璧凤級synchronize_rcu() 浼氫竴鐩撮樆濉烇紝鐩村埌鎵€鏈?CPU 瀹屾垚瀹冧滑
姝ｅ湪鎵ц鐨勪换浣曠鐢ㄦ姠鍗犵殑浠ｇ爜娈点€傜敱浜?NMI 澶勭悊绋嬪簭浼氱鐢ㄦ姠鍗狅紝synchronize_rcu() 淇濊瘉
鍦ㄦ墍鏈夋鍦ㄨ繘琛岀殑 NMI 澶勭悊绋嬪簭閫€鍑轰箣鍓嶄笉浼氳繑鍥炪€傚洜姝わ紝涓€鏃?synchronize_rcu() 杩斿洖锛?灏卞彲浠ュ畨鍏ㄥ湴閲婃斁璇ュ鐞嗙▼搴忕殑鏁版嵁銆?
閲嶈鎻愮ず锛氳璁╀笂杩版満鍒跺伐浣滐紝鐩稿叧鏋舵瀯蹇呴』鍦?NMI 杩涘叆鍜岄€€鍑烘椂鍒嗗埆璋冪敤 nmi_enter() 鍜?nmi_exit()銆?

蹇€熸祴楠岀殑绛旀锛?		鑰冭檻鍒版寚閽堟墍寮曠敤鐨勪唬鐮佹槸鍙鐨勶紝涓轰粈涔堝湪 Alpha 涓婂彲鑳戒粛鐒堕渶瑕?		rcu_dereference_sched()锛?
		璋冪敤 set_nmi_callback() 鐨勪汉寰堝彲鑳藉凡缁忓垵濮嬪寲浜嗘煇浜涜琚柊鐨?NMI
		澶勭悊绋嬪簭浣跨敤鐨勬暟鎹€傚湪杩欑鎯呭喌涓嬶紝灏遍渶瑕?rcu_dereference_sched()锛?		鍥犱负鍚﹀垯锛屽湪璁剧疆浜嗘柊澶勭悊绋嬪簭涔嬪悗绔嬪嵆鏀跺埌 NMI 鐨?CPU锛屽彲鑳戒細鐪嬪埌
		鎸囧悜鏂?NMI 澶勭悊绋嬪簭鐨勬寚閽堬紝浣嗙湅鍒扮殑鏄鐞嗙▼搴忔暟鎹殑鏃э紙棰勫垵濮嬪寲锛?		鐗堟湰銆?
		褰撲娇鐢ㄤ竴涓甫鏈夋縺杩涙寚閽堝€兼帹娴嬶紙pointer-value speculation锛変紭鍖栫殑
		缂栬瘧鍣ㄦ椂锛屽悓鏍风殑绯熺硶鎯呭喌涔熶細鍙戠敓鍦ㄥ叾瀹?CPU 涓娿€傦紙浣嗚鍒繖鏍凤紒锛?
		鏇撮噸瑕佺殑鏄紝rcu_dereference_sched() 璁╅槄璇讳唬鐮佺殑浜烘竻妤氬湴鐭ラ亾锛岃
		鎸囬拡姝ｅ彈鍒?RCU-sched 鐨勪繚鎶ゃ€?