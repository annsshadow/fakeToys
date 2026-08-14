## Proper Locking Under a Preemptible Kernel: Keeping Kernel Code Preempt-Safe


:Author: Robert Love <rml@tech9.net>


## Introduction


鍙姠鍗犲唴鏍稿甫鏉ヤ簡鏂扮殑閿侀棶棰樸€傝繖浜涢棶棰樹笌 SMP 涓嬬殑闂鐩稿悓锛氬苟鍙戝拰閲嶅叆銆傚€煎緱搴嗗垢鐨勬槸锛孡inux 鍙姠鍗犲唴鏍告ā鍨嬪埄鐢ㄤ簡鐜版湁鐨?SMP 閿佹満鍒躲€傚洜姝わ紝鍐呮牳浠呭湪鏋佸皯鏁伴澶栨儏鍐典笅鎵嶉渶瑕佹樉寮忕殑棰濆鍔犻攣銆?
鏈枃妗ｉ潰鍚戞墍鏈夊唴鏍搁粦瀹€傚湪鍐呮牳涓紑鍙戜唬鐮侀渶瑕佷繚鎶よ繖浜涙儏鍐点€?

##### RULE #1: Per-CPU data structures need explicit protection


```

	struct this_needs_locking tux[NR_CPUS];
	tux[smp_processor_id()] = some_value;
	/* task is preempted here... */
	something = tux[smp_processor_id()];

```
棣栧厛锛岀敱浜庢暟鎹槸 per-CPU 鐨勶紝瀹冨彲鑳芥病鏈夋槑纭娇鐢?SMP 閿侊紝浣嗗湪鍏朵粬鏂归潰闇€瑕佸畠銆傚叾娆★紝褰撲竴涓鎶㈠崰鐨勪换鍔℃渶缁堣閲嶆柊璋冨害鏃讹紝smp_processor_id 涔嬪墠鐨勫€煎彲鑳戒笉绛変簬褰撳墠鍊笺€備綘蹇呴』閫氳繃鍦ㄨ繖浜涙儏鍐靛懆鍥寸鐢ㄦ姠鍗犳潵淇濇姢瀹冧滑銆?
浣犱篃鍙互浣跨敤 put_cpu() 鍜?get_cpu()锛屽畠浠細绂佺敤鎶㈠崰銆?

##### RULE #2: CPU state must be protected.


鍦ㄦ姠鍗犱笅锛孋PU 鐨勭姸鎬佸繀椤昏淇濇姢銆傝繖涓庝綋绯绘灦鏋勭浉鍏筹紝浣嗗寘鎷笂涓嬫枃鍒囨崲鏃朵笉琚繚瀛樼殑 CPU 缁撴瀯鍜岀姸鎬併€備緥濡傦紝鍦?x86 涓婏紝杩涘叆鍜岄€€鍑?FPU 妯″紡鐜板湪鏄竴涓复鐣屽尯锛屽繀椤诲湪绂佺敤鎶㈠崰鐨勬儏鍐典笅杩涜銆傝瘯鎯充竴涓嬶紝濡傛灉鍐呮牳姝ｅ湪鎵ц涓€鏉℃诞鐐规寚浠わ紝鐒跺悗琚姠鍗狅紝浼氬彂鐢熶粈涔堛€傝璁颁綇锛屽唴鏍镐笉浼氫繚瀛?FPU 鐘舵€侊紝鍙湁鐢ㄦ埛浠诲姟鎵嶄細銆傚洜姝わ紝涓€鏃﹁鎶㈠崰锛孎PU 瀵勫瓨鍣ㄥ氨浼氬崠缁欙紙sold to锛夊嚭浠锋渶浣庤€呫€傚洜姝わ紝蹇呴』鍦ㄨ繖浜涘尯鍩熷懆鍥寸鐢ㄦ姠鍗犮€?
璇锋敞鎰忥紝鏌愪簺 FPU 鍑芥暟宸茬粡鏄庣‘鏄姠鍗犲畨鍏ㄧ殑銆備緥濡傦紝kernel_fpu_begin 鍜?kernel_fpu_end 浼氱鐢ㄥ拰鍚敤鎶㈠崰銆?

##### RULE #3: Lock acquire and release must be performed by same task


鍦ㄤ竴涓换鍔′腑鑾峰彇鐨勯攣蹇呴』鐢卞悓涓€涓换鍔￠噴鏀俱€傝繖鎰忓懗鐫€浣犱笉鑳藉仛璇稿鑾峰彇涓€涓攣鐒跺悗鍘诲共鍒殑浜嬨€佽€岀敱鍙︿竴涓换鍔℃潵閲婃斁瀹冭繖鏍风殑鎬簨銆傚鏋滀綘鎯冲仛绫讳技鐨勪簨鎯咃紝搴斿湪鍚屼竴浠ｇ爜璺緞涓幏鍙栧苟閲婃斁浠诲姟锛屽苟璁╄皟鐢ㄨ€呯瓑寰呭彟涓€涓换鍔″彂鍑虹殑浜嬩欢銆?

## Solution


鍦ㄦ姠鍗犱笅淇濇姢鏁版嵁鏄€氳繃鍦ㄤ复鐣屽尯鎸佺画鏈熼棿绂佺敤鎶㈠崰鏉ュ疄鐜扮殑銆?
```

  preempt_enable()		decrement the preempt counter
  preempt_disable()		increment the preempt counter
  preempt_enable_no_resched()	decrement, but do not immediately preempt
  preempt_check_resched()	if needed, reschedule
  preempt_count()		return the preempt counter

```
杩欎簺鍑芥暟鏄彲宓屽鐨勩€傛崲鍙ヨ瘽璇达紝浣犲彲浠ュ湪涓€鏉′唬鐮佽矾寰勪腑璋冪敤 preempt_disable n 娆★紝鑰屽湪绗?n 娆¤皟鐢?preempt_enable 涔嬪墠锛屾姠鍗犱笉浼氳閲嶆柊鍚敤銆傚鏋滄湭鍚敤鎶㈠崰锛岃繖浜?preempt 璇彞瀹氫箟涓虹┖銆?
璇锋敞鎰忥紝濡傛灉浣犳寔鏈変换浣曢攣鎴栦腑鏂绂佺敤锛屽垯涓嶉渶瑕佹樉寮忛槻姝㈡姠鍗狅紝鍥犱负鍦ㄨ繖浜涙儏鍐典笅鎶㈠崰鏄殣寮忕鐢ㄧ殑銆?
浣嗚璁颁綇锛?irqs disabled' 鏄竴绉嶄粠鏍规湰涓婄湅涓嶅畨鍏ㄧ殑绂佺敤鎶㈠崰鏂瑰紡鈥斺€斾换浣?cond_resched() 鎴?cond_resched_lock() 閮藉彲鑳藉湪鎶㈠崰璁℃暟涓?0 鏃惰Е鍙戦噸鏂拌皟搴︺€備竴涓畝鍗曠殑 printk() 灏卞彲鑳借Е鍙戦噸鏂拌皟搴︺€傚洜姝わ紝鍙湁鍦ㄤ綘鐭ラ亾鍙楀奖鍝嶇殑浠ｇ爜璺緞涓嶄細鍋氫换浣曡繖绫讳簨鎯呮椂锛屾墠浣跨敤杩欑闅愬紡绂佺敤鎶㈠崰鐨勭壒鎬с€傛渶浣崇瓥鐣ユ槸浠呭皢鍏剁敤浜庝綘缂栧啓鐨勩€佸皬鑰屽師瀛愮殑銆佷笖涓嶈皟鐢ㄥ鏉傚嚱鏁扮殑浠ｇ爜銆?
```

	cpucache_t *cc; /* this is per-CPU */
	preempt_disable();
	cc = cc_data(searchp);
	if (cc && cc->avail) {
		__free_block(searchp, cc_entry(cc), cc->avail);
		cc->avail = 0;
	}
	preempt_enable();
	return 0;

```
娉ㄦ剰锛屾姠鍗犺鍙ュ繀椤绘兜鐩栧浠ヤ笅鍐呭鐨勬瘡涓€娆″紩鐢?
```

	int buf[NR_CPUS];
	set_cpu_val(buf);
	if (buf[smp_processor_id()] == -1) printf(KERN_INFO "wee!\n");
	spin_lock(&buf_lock);
	/* ... */

```
杩欐浠ｇ爜涓嶆槸鎶㈠崰瀹夊叏鐨勶紝浣嗙湅鐪嬫垜浠彧闇€灏?spin_lock 涓婄Щ涓よ灏辫兘澶氬鏄撳湴淇瀹冦€?

## Preventing preemption using interrupt disabling


鍙互浣跨敤 local_irq_disable 鍜?local_irq_save 鏉ラ槻姝㈡姠鍗犱簨浠躲€傝娉ㄦ剰锛岃繖鏍峰仛鏃讹紝浣犲繀椤婚潪甯稿皬蹇冿紝涓嶈寮曞彂浼氳缃?need_resched 骞跺鑷存姠鍗犳鏌ョ殑浜嬩欢銆傚綋鏈夌枒闂椂锛屼緷闈犻攣鎴栨樉寮忕殑鎶㈠崰绂佺敤銆?
璇锋敞鎰忥紝鍦?2.5 涓紝绂佺敤涓柇鐜板湪鍙槸 per-CPU 鐨勶紙鍗虫湰鍦扮殑锛夈€?
鍙︿竴涓叧娉ㄧ偣鏄 local_irq_disable 鍜?local_irq_save 鐨勬纭娇鐢ㄣ€傚畠浠彲鐢ㄤ簬淇濇姢鍏嶅彈鎶㈠崰锛屼絾鏄紝鍦ㄩ€€鍑烘椂锛屽鏋滃彲鑳藉惎鐢ㄦ姠鍗狅紝鍒欏簲褰撳仛涓€娆℃鏌ワ紝鐪嬫槸鍚﹂渶瑕佹姠鍗犮€傚鏋滃畠浠槸浠?spin_lock 鍜?read/write lock 瀹忎腑璋冪敤鐨勶紝灏变細鍋氭纭殑浜嬫儏銆傚畠浠篃鍙互鍦ㄨ嚜鏃嬮攣淇濇姢鐨勫尯鍩熷唴璋冪敤锛屼絾鏄紝濡傛灉瀹冧滑鍦ㄨ涓婁笅鏂囦箣澶栬璋冪敤锛屽垯搴斿綋鍋氫竴娆℃姠鍗犳鏌ャ€傝娉ㄦ剰锛屾潵鑷腑鏂笂涓嬫枃鎴栧簳鍗婇儴/tasklet 鐨勮皟鐢ㄤ篃鍙楁姠鍗犻攣淇濇姢锛屽洜姝ゅ彲浠ヤ娇鐢ㄤ笉妫€鏌ユ姠鍗犵殑鐗堟湰銆?