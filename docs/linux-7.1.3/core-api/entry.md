## 寮傚父銆佷腑鏂€佺郴缁熻皟鐢ㄤ笌 KVM 鐨勮繘鍏?閫€鍑哄鐞?


鍚勬墽琛屽煙涔嬮棿鐨勫垏鎹㈤兘闇€瑕佽繘琛岀姸鎬佹洿鏂帮紝杩欎簺鏇存柊
鍙椾弗鏍肩殑椤哄簭绾︽潫銆備互涓嬫儏褰㈤渶瑕佹墽琛岀姸鎬佹洿鏂帮細


  - Lockdep
  - RCU / 涓婁笅鏂囪窡韪?
  - 鎶㈠崰璁℃暟鍣?
  - 璺熻釜
  - 鏃堕棿璁拌处

鏇存柊椤哄簭鍙栧喅浜庡垏鎹㈢被鍨嬶紝骞跺湪涓嬫枃鐨勫垏鎹㈢被鍨嬬珷鑺備腑璇存槑锛歚Syscalls`_銆乣KVM`_銆乣Interrupts and regular
exceptions`_, `NMI and NMI-like exceptions`_銆?
exceptions`_, `NMI and NMI-like exceptions`_.

### 涓嶅彲鎻掓々浠ｇ爜 - noinstr


澶у鏁版彃妗╂満鍒朵緷璧?RCU锛屽洜姝ゅ湪 RCU 寮€濮嬬洃瑙嗕箣鍓嶇殑杩涘叆浠ｇ爜锛屼互鍙?RCU 鍋滄鐩戣涔嬪悗鐨勯€€鍑轰唬鐮佷腑锛岀姝㈣繘琛屾彃妗┿€傛澶栵紝璁稿浣撶郴缁撴瀯蹇呴』淇濆瓨鍜屾仮澶嶅瘎瀛樺櫒鐘舵€侊紝杩欐剰鍛崇潃锛堜緥濡傦級鍦ㄦ柇鐐硅繘鍏ヤ唬鐮佷腑鏀剧疆涓€涓柇鐐逛細瑕嗙洊鍒濆鏂偣鐨勮皟璇曞瘎瀛樺櫒銆?





姝ょ被浠ｇ爜蹇呴』浣跨敤 'noinstr' 灞炴€ф爣璁帮紝灏嗗叾鏀惧叆鎻掓々涓庤皟璇曞伐鍏烽兘鏃犳硶璁块棶鐨勭壒娈婃涓€傞儴鍒嗗嚱鏁板彲閮ㄥ垎鎻掓々锛屽叾澶勭悊鏂瑰紡鏄皢鍑芥暟鏍囪涓?noinstr锛屽苟浣跨敤 instrumentation_begin() 涓?instrumentation_end() 鏉ユ爣璁板彲鎻掓々鐨勪唬鐮佽寖鍥达細






  noinstr void entry(void)
  {
  	handle_entry();     // <-- must be 'noinstr' or '__always_inline'
	...

	instrumentation_begin();
	handle_context();   // <-- instrumentable code
	instrumentation_end();

	...
	handle_exit();      // <-- must be 'noinstr' or '__always_inline'
  }

杩欐牱渚垮彲鍦ㄥ彈鏀寔鐨勪綋绯荤粨鏋勪笂閫氳繃 objtool 楠岃瘉 'noinstr' 闄愬埗銆?


浠庡彲鎻掓々涓婁笅鏂囦腑璋冪敤涓嶅彲鎻掓々鍑芥暟娌℃湁浠讳綍闄愬埗锛屽苟涓旀湁鍔╀簬淇濇姢閭ｄ簺涓€鏃﹁鎻掓々灏变細鍑洪敊鐨勪唬鐮侊紝渚嬪鐘舵€佸垏鎹€?



RCU 鐘舵€佸垏鎹箣鍓嶄笌涔嬪悗鐨勬墍鏈変笉鍙彃妗╄繘鍏?閫€鍑轰唬鐮佹閮藉繀椤诲湪涓柇琚姝㈢殑鎯呭喌涓嬭繍琛屻€?


### 绯荤粺璋冪敤


绯荤粺璋冪敤杩涘叆浠ｇ爜濮嬩簬姹囩紪浠ｇ爜锛屽湪寤虹珛搴曞眰涓庝綋绯荤粨鏋勭浉鍏崇殑鐘舵€佸拰鏍堝抚涔嬪悗锛岃皟鐢ㄥ簳灞?C 浠ｇ爜銆傝繖娈靛簳灞?C 浠ｇ爜涓嶈兘琚彃妗┿€備竴涓粠搴曞眰姹囩紪浠ｇ爜璋冪敤鐨勫吀鍨嬬郴缁熻皟鐢ㄥ鐞嗗嚱鏁板涓嬫墍绀猴細





  noinstr void syscall(struct pt_regs *regs, int nr)
  {
	arch_syscall_enter(regs);
	nr = syscall_enter_from_user_mode(regs, nr);

	instrumentation_begin();
	if (!invoke_syscall(regs, nr) && nr != -1)
	 	result_reg(regs) = __sys_ni_syscall(regs);
	instrumentation_end();

	syscall_exit_to_user_mode(regs);
  }

syscall_enter_from_user_mode() 棣栧厛璋冪敤 enter_from_user_mode()锛岃鍑芥暟鎸変互涓嬮『搴忓缓绔嬬姸鎬侊細


  - Lockdep
  - RCU / 涓婁笅鏂囪窡韪?
  - 璺熻釜

闅忓悗璋冪敤鍚勭杩涘叆闃舵鐨勫伐浣滃嚱鏁帮紝濡?ptrace銆乻eccomp銆乤udit銆乻yscall tracing 绛夈€傚畬鎴愯繖浜涘伐浣滃悗锛屾柟鍙皟鐢ㄥ彲鎻掓々鐨?invoke_syscall 鍑芥暟銆傚彲鎻掓々浠ｇ爜娈靛埌姝ょ粨鏉燂紝涔嬪悗璋冪敤 syscall_exit_to_user_mode()銆?




syscall_exit_to_user_mode() 澶勭悊杩斿洖鐢ㄦ埛绌洪棿涔嬪墠闇€瑕佸畬鎴愮殑鎵€鏈夊伐浣滐紝渚嬪璺熻釜銆乤udit銆佷俊鍙枫€乼ask work 绛夈€備箣鍚庡畠璋冪敤 exit_to_user_mode()锛岃鍑芥暟浠ョ浉鍙嶇殑椤哄簭鍐嶆澶勭悊鐘舵€佸垏鎹細




  - 璺熻釜
  - RCU / 涓婁笅鏂囪窡韪?
  - Lockdep

syscall_enter_from_user_mode() 涓?syscall_exit_to_user_mode() 涔熷彲浣滀负缁嗙矑搴︾殑瀛愬嚱鏁颁娇鐢紝閫傜敤浜庝綋绯荤粨鏋勪唬鐮侀渶瑕佸湪鍚勬楠や箣闂村畬鎴愰澶栧伐浣滅殑鍦烘櫙銆傝繖绉嶆儏鍐典笅锛屽繀椤荤‘淇濆湪杩涘叆鏃堕鍏堣皟鐢?enter_from_user_mode()锛屽苟鍦ㄩ€€鍑烘椂鏈€鍚庤皟鐢?exit_to_user_mode()銆?





涓嶈宓屽绯荤粺璋冪敤銆傚祵濂楃郴缁熻皟鐢ㄤ細瀵艰嚧 RCU 鍜?鎴栦笂涓嬫枃璺熻釜鎵撳嵃璀﹀憡銆?


### KVM


杩涘叆鎴栭€€鍑哄鎴锋満妯″紡涓庣郴缁熻皟鐢ㄩ潪甯哥浉浼笺€備粠瀹夸富鍐呮牳鐨勮搴︾湅锛孋PU 鍦ㄨ繘鍏ュ鎴锋満鏃朵豢浣涜繘鍏ヤ簡鐢ㄦ埛绌洪棿锛岃€屽湪閫€鍑烘椂杩斿洖鍐呮牳銆?



guest_state_enter_irqoff() 鏄?exit_to_user_mode() 鐨?KVM 涓撶敤鍙樹綋锛実uest_state_exit_irqoff() 鏄?enter_from_user_mode() 鐨?KVM 鍙樹綋銆傚叾鐘舵€佹搷浣滅殑椤哄簭鐩稿悓銆?



瀵逛簬瀹㈡埛鏈猴紝浠诲姟宸ヤ綔鐨勫鐞嗗湪 vcpu_run() 寰幆鐨勮竟鐣屽閫氳繃 xfer_to_guest_mode_handle_work() 鍗曠嫭杩涜锛岃鍑芥暟澶勭悊鐨勬槸杩斿洖鐢ㄦ埛绌洪棿鏃跺鐞嗗伐浣滅殑涓€閮ㄥ垎瀛愰泦銆?



涓嶈宓屽 KVM 鐨勮繘鍏?閫€鍑哄垏鎹紝鍥犱负杩欐牱鍋氭鏃犳剰涔夈€?

### 涓柇涓庡父瑙勫紓甯?


涓柇鐨勮繘鍏ヤ笌閫€鍑哄鐞嗘瘮绯荤粺璋冪敤鍜?KVM 鍒囨崲瑕佺◢绋嶅鏉備竴浜涖€?


濡傛灉涓柇鏄湪 CPU 鎵ц鐢ㄦ埛绌洪棿浠ｇ爜鏃惰Е鍙戠殑锛屽叾杩涘叆涓庨€€鍑哄鐞嗕笌绯荤粺璋冪敤瀹屽叏鐩稿悓銆?


濡傛灉涓柇鏄湪 CPU 鎵ц鍐呮牳绌洪棿浠ｇ爜鏃惰Е鍙戠殑锛屽叾杩涘叆涓庨€€鍑哄鐞嗗垯鐣ユ湁涓嶅悓銆傚彧鏈夊綋涓柇鏄湪 CPU 绌洪棽浠诲姟鐨勪笂涓嬫枃涓Е鍙戞椂锛屾墠浼氭洿鏂?RCU 鐘舵€侊紱鍚﹀垯 RCU 宸茬粡鍦ㄧ洃瑙嗕腑銆侺ockdep 涓?tracing 蹇呴』鏃犳潯浠舵洿鏂般€?




irqentry_enter() 涓?irqentry_exit() 鎻愪緵浜嗘鍔熻兘鐨勫疄鐜般€?

涓庝綋绯荤粨鏋勭浉鍏崇殑閮ㄥ垎涓庣郴缁熻皟鐢ㄥ鐞嗙被浼硷細


  noinstr void interrupt(struct pt_regs *regs, int nr)
  {
	arch_interrupt_enter(regs);
	state = irqentry_enter(regs);

	instrumentation_begin();

	irq_enter_rcu();
	invoke_irq_handler(regs, nr);
	irq_exit_rcu();

	instrumentation_end();

	irqentry_exit(regs, state);
  }

璇锋敞鎰忥紝瀹為檯涓柇澶勭悊绋嬪簭鐨勮皟鐢ㄤ綅浜?irq_enter_rcu() 涓?irq_exit_rcu() 杩欎竴瀵硅皟鐢ㄤ箣闂淬€?


irq_enter_rcu() 浼氭洿鏂版姠鍗犺鏁帮紝浣?in_hardirq() 杩斿洖 true锛屽苟澶勭悊 NOHZ tick 鐘舵€佷笌涓柇鏃堕棿璁拌处銆傝繖鎰忓懗鐫€鍦ㄨ皟鐢?irq_enter_rcu() 涔嬪墠锛宨n_hardirq() 涓€鐩磋繑鍥?false銆?




irq_exit_rcu() 澶勭悊涓柇鏃堕棿璁拌处锛屾挙閿€鎶㈠崰璁℃暟鐨勬洿鏂帮紝骞舵渶缁堝鐞嗚蒋涓柇涓?NOHZ tick 鐘舵€併€?


鐞嗚涓婏紝鎶㈠崰璁℃暟鍙互鍦?irqentry_enter() 涓洿鏂般€備絾瀹為檯涓婏紝灏嗘鏇存柊鎺ㄨ繜鍒?irq_enter_rcu() 鍙互璁╂姠鍗犺鏁扮浉鍏充唬鐮佽璺熻釜锛屽悓鏃朵繚鎸佷笌 irq_exit_rcu() 鍜?irqentry_exit()锛堝湪涓嬩竴娈典腑鎻忚堪锛夌殑瀵圭О鎬с€傚敮涓€鐨勭己鐐规槸锛屽湪璋冪敤 irq_enter_rcu() 涔嬪墠鐨勬棭鏈熻繘鍏ヤ唬鐮佸繀椤绘剰璇嗗埌鎶㈠崰璁℃暟灏氭湭鏇存柊涓?HARDIRQ_OFFSET 鐘舵€併€?






娉ㄦ剰锛宨rq_exit_rcu() 鍦ㄥ鐞嗚蒋涓柇涔嬪墠锛屽繀椤讳粠鎶㈠崰璁℃暟涓Щ闄?HARDIRQ_OFFSET锛屽洜涓鸿蒋涓柇鐨勫鐞嗙▼搴忓繀椤诲湪 BH 涓婁笅鏂囦腑杩愯锛岃€屼笉鏄湪涓柇琚姝㈢殑涓婁笅鏂囦腑銆傛澶栵紝irqentry_exit() 鍙兘浼氳繘琛岃皟搴︼紝杩欏悓鏍疯姹備粠鎶㈠崰璁℃暟涓Щ闄?HARDIRQ_OFFSET銆?




灏界涓柇澶勭悊绋嬪簭搴斿綋鍦ㄦ湰鍦颁腑鏂绂佹鐨勬儏鍐典笅杩愯锛屼絾浠庤繘鍏?閫€鍑虹殑瑙掑害鐪嬶紝涓柇宓屽鏄緢甯歌鐨勩€備緥濡傦紝杞腑鏂殑澶勭悊灏卞彂鐢熷湪鏈湴涓柇澶勪簬寮€鍚姸鎬佺殑 irqentry_{enter,exit}() 浠ｇ爜鍧楀唴銆傛澶栵紝铏界劧涓嶅父瑙侊紝浣嗘病鏈変换浣曟満鍒堕樆姝腑鏂鐞嗙▼搴忛噸鏂板紑鍚腑鏂€?





涓柇鐨勮繘鍏?閫€鍑轰唬鐮佸苟涓嶄弗鏍艰姹傚鐞嗗彲閲嶅叆鎬э紝鍥犱负瀹冩槸鍦ㄦ湰鍦颁腑鏂绂佹鐨勬儏鍐典笅杩愯鐨勩€備絾 NMI 鍙兘鍦ㄤ换浣曟椂鍊欏彂鐢燂紝鑰屼笖涓よ€呭叡浜ぇ閲忚繘鍏ヤ唬鐮併€?



### NMI 涓庣被 NMI 寮傚父


NMI 涓庣被 NMI 寮傚父锛坢achine checks銆乨ouble faults銆乨ebug 涓柇绛夛級鍙互鍙戠敓鍦ㄤ换浣曚笂涓嬫枃涓紝蹇呴』鏍煎璋ㄦ厧鍦板寰呯姸鎬併€?



璋冭瘯寮傚父涓?machine-check 寮傚父鐨勭姸鎬佸彉鍖栧彇鍐充簬杩欎簺寮傚父鍙戠敓鍦ㄧ敤鎴风┖闂达紙鏂偣鎴栬瀵熺偣锛夎繕鏄唴鏍告ā寮忥紙浠ｇ爜琛ヤ竵锛夈€傚湪鐢ㄦ埛绌洪棿涓紝瀹冧滑琚綋浣滀腑鏂鐞嗭紱鍦ㄥ唴鏍告ā寮忎腑锛屽畠浠褰撲綔 NMI 澶勭悊銆?




NMI 涓庡叾浠栫被 NMI 寮傚父澶勭悊鐘舵€佸垏鎹㈡椂锛屼笉鍖哄垎鍏舵潵婧愭槸鐢ㄦ埛妯″紡杩樻槸鍐呮牳妯″紡銆?


杩涘叆鏃剁殑鐘舵€佹洿鏂扮敱 irqentry_nmi_enter() 澶勭悊锛岃鍑芥暟鎸変互涓嬮『搴忔洿鏂扮姸鎬侊細


  - 鎶㈠崰璁℃暟鍣?
  - Lockdep
  - RCU / 涓婁笅鏂囪窡韪?
  - 璺熻釜

鍏堕€€鍑哄搴斿嚱鏁?irqentry_nmi_exit() 浠ョ浉鍙嶇殑椤哄簭鎵ц鍙嶅悜鎿嶄綔銆?


娉ㄦ剰锛屾姠鍗犺鏁扮殑鏇存柊鍦ㄨ繘鍏ユ椂蹇呴』鏄涓€涓搷浣滐紝鍦ㄩ€€鍑烘椂蹇呴』鏄渶鍚庝竴涓搷浣溿€傚師鍥犳槸 lockdep 鍜?RCU 閮戒緷璧栦簬 in_nmi() 鍦ㄨ繖绉嶆儏鍐典笅杩斿洖 true銆侼MI 杩涘叆/閫€鍑哄満鏅腑鐨勬姠鍗犺鏁颁慨鏀逛笉鑳借璺熻釜銆?





涓庝綋绯荤粨鏋勭浉鍏崇殑浠ｇ爜濡備笅鎵€绀猴細


  noinstr void nmi(struct pt_regs *regs)
  {
	arch_nmi_enter(regs);
	state = irqentry_nmi_enter(regs);

	instrumentation_begin();
	nmi_handler(regs);
	instrumentation_end();

	irqentry_nmi_exit(regs);
  }

渚嬪锛屽浜庤皟璇曞紓甯革紝浠ｇ爜鍙兘濡備笅鎵€绀猴細


  noinstr void debug(struct pt_regs *regs)
  {
	arch_nmi_enter(regs);

	debug_regs = save_debug_regs();

	if (user_mode(regs)) {
		state = irqentry_enter(regs);

		instrumentation_begin();
		user_mode_debug_handler(regs, debug_regs);
		instrumentation_end();

		irqentry_exit(regs, state);
  	} else {
  		state = irqentry_nmi_enter(regs);

		instrumentation_begin();
		kernel_mode_debug_handler(regs, debug_regs);
		instrumentation_end();

		irqentry_nmi_exit(regs, state);
	}
  }

娌℃湁鍙敤鐨勭粍鍚堝嚱鏁?irqentry_nmi_if_kernel()锛屽洜涓轰笂杩版儏鍐垫棤娉曚互涓庡紓甯告棤鍏崇殑鏂瑰紡澶勭悊銆?


NMI 鍙互鍙戠敓鍦ㄤ换浣曚笂涓嬫枃涓€備緥濡傦紝鍦ㄥ鐞?NMI 鏃跺彲鑳借Е鍙戜竴涓被 NMI 寮傚父銆傚洜姝わ紝NMI 鐨勮繘鍏ヤ唬鐮佸繀椤绘槸鍙噸鍏ョ殑锛屽苟涓旂姸鎬佹洿鏂伴渶瑕佸鐞嗗祵濂椼€?


