## 鍑芥暟璺熻釜鍣紙Function Tracer锛夎璁?

:Author: Mike Frysinger

	鏈枃妗ｅ凡缁忚繃鏃躲€備笅闈㈡弿杩扮殑鏌愪簺鍐呭宸茬粡涓庡綋鍓嶇殑瀹炵幇涓嶇銆?
### 绠€浠?

杩欓噷鎴戜滑浠嬬粛鍏叡鍑芥暟璺熻釜浠ｇ爜璧栦互姝ｅ父宸ヤ綔鐨勬灦鏋勭浉鍏抽儴鍒嗐€傚唴瀹规寜澶嶆潅搴﹂€掑
缁勭粐锛屼互渚夸綘鍙互浠庣畝鍗曞叆鎵嬶紝鑷冲皯鑾峰緱鍩烘湰鍔熻兘銆?
娉ㄦ剰锛屾湰鏂囨。鍙叧娉ㄦ灦鏋勫疄鐜扮粏鑺傘€傚鏋滀綘甯屾湜浠庡叕鍏变唬鐮佽搴︿簡瑙ｆ煇涓姛鑳界殑
鏇村璇存槑锛岃鏌ラ槄 ftrace.txt 鏂囦欢銆?
鐞嗘兂鎯呭喌涓嬶紝浠讳綍甯屾湜鍦ㄦ敮鎸佽窡韪殑鍚屾椂淇濇寔鎬ц兘鐨勫唴鏍革紝閮藉簲璇ヤ竴璺仛鍒版敮鎸?鍔ㄦ€?ftrace銆?
### 鍏堝喅鏉′欢


ftrace 渚濊禆浜庝互涓嬬壒鎬х殑瀹炵幇锛?  - STACKTRACE_SUPPORT - 瀹炵幇 save_stack_trace()
  - TRACE_IRQFLAGS_SUPPORT - 瀹炵幇 include/asm/irqflags.h

### HAVE_FUNCTION_TRACER


浣犻渶瑕佸疄鐜?mcount 鍜?ftrace_stub 鍑芥暟銆?
鍏蜂綋鐨?mcount 绗﹀彿鍚嶅彇鍐充簬浣犵殑宸ュ叿閾俱€傛湁鐨勫彨 鈥渕count鈥濄€佲€淿mcount鈥濓紝鐢氳嚦
鈥淿_mcount鈥濄€備綘澶ф鍙互閫氳繃涓嬮潰鐨勬柟寮忔煡鍑烘潵锛?```
	$ echo 'main(){}' | gcc -x c -S -o - - -pg | grep mcount
	        call    mcount
```
涓轰簡涓嬫柟绀轰緥绠€娲佹竻鏅帮紝鎴戜滑鍋囧畾绗﹀彿鍚嶄负 鈥渕count鈥濄€?
璇疯浣忥紝mcount 鍑芥暟鍐呴儴鐢熸晥鐨?ABI 鏄?**楂樺害** 渚濊禆浜庢灦鏋?宸ュ叿閾剧殑銆傝繖鏂归潰
鎴戜滑甯笉浜嗕綘锛屾姳姝夈€傝缈诲嚭涓€浜涜€佹枃妗ｏ紝鎴栬€呮壘涓瘮浣犳洿鐔熸倝鐨勪汉涓€璧锋帰璁ㄣ€傞€氬父
鎯呭喌涓嬶紝瀵勫瓨鍣ㄧ殑浣跨敤锛堝弬鏁?涓存椂/绛夌瓑锛夊湪杩欎竴鐐逛笂鏄富瑕侀棶棰橈紝灏ゅ叾鏄笌 mcount
璋冪敤浣嶇疆锛堝湪鍑芥暟搴忚█涔嬪墠/涔嬪悗锛夌浉鍏虫椂銆備綘杩樺彲鑳芥兂鐪嬬湅 glibc 鏄浣曚负浣犵殑
鏋舵瀯瀹炵幇 mcount 鍑芥暟鐨勶紝鎴栬锛堝崐锛夌浉鍏炽€?
mcount 鍑芥暟搴旀鏌ュ嚱鏁版寚閽?ftrace_trace_function锛岀湅瀹冩槸鍚﹁璁剧疆涓?ftrace_stub銆傚鏋滄槸锛岄偅浣犳棤浜嬪彲鍋氾紝鐩存帴杩斿洖鍗冲彲銆傚鏋滀笉鏄紝鍒欏儚 mcount 鍑芥暟
閫氬父璋冪敤 __mcount_internal 閭ｆ牱璋冪敤璇ュ嚱鏁扳€斺€旂涓€涓弬鏁版槸 鈥渇rompc鈥濓紝绗簩涓?鍙傛暟鏄?鈥渟elfpc鈥濓紙宸茶皟鏁翠互鍘婚櫎鍐呭祵浜庡嚱鏁颁腑鐨?mcount 璋冪敤鐨勫ぇ灏忥級銆?
渚嬪锛岃嫢鍑芥暟 foo() 璋冪敤 bar()锛屽綋 bar() 鍑芥暟璋冪敤 mcount() 鏃讹紝mcount() 灏?浼犻€掔粰璺熻釜鍣ㄧ殑鍙傛暟涓猴細

  - 鈥渇rompc鈥?- bar() 鐢ㄦ潵杩斿洖 foo() 鐨勫湴鍧€
  - 鈥渟elfpc鈥?- bar() 鐨勫湴鍧€锛堝凡鍋?mcount() 澶у皬璋冩暣锛?
杩樿璁颁綇锛岃繖涓?mcount 鍑芥暟浼氳 **棰戠箒** 璋冪敤锛屽洜姝ら拡瀵规棤璺熻釜鍣ㄧ殑榛樿鎯呭喌杩涜
浼樺寲锛屽皢鏈夊姪浜庡湪绂佺敤璺熻釜鏃剁郴缁熺殑骞崇ǔ杩愯銆傛墍浠?mcount 鍑芥暟鐨勫紑澶撮€氬父鍙仛
鏈€灏戦噺鐨勬鏌ヤ究杩斿洖銆傝繖涔熸剰鍛崇潃浠ｇ爜娴佺▼閫氬父搴斾繚鎸佺嚎鎬э紙鍗冲湪 nop 鎯呭喌涓嬩笉
鍒嗘敮锛夈€傝繖褰撶劧鏄竴绉嶄紭鍖栬€岄潪纭€ц姹傘€?
涓嬮潰鏄竴浜涘簲璇ユ湁甯姪鐨勪吉浠ｇ爜锛堣繖浜涘嚱鏁板疄闄呬笂搴斿綋
```
	void ftrace_stub(void)
	{
		return;
	}

	void mcount(void)
	{
		/* save any bare state needed in order to do initial checking */

		extern void (*ftrace_trace_function)(unsigned long, unsigned long);
		if (ftrace_trace_function != ftrace_stub)
			goto do_trace;

		/* restore any bare state */

		return;

	do_trace:

		/* save all state needed by the ABI (see paragraph above) */

		unsigned long frompc = ...;
		unsigned long selfpc = <return address> - MCOUNT_INSN_SIZE;
		ftrace_trace_function(frompc, selfpc);

		/* restore all state needed by the ABI */
	}
```
鍒繕浜嗕负妯″潡瀵煎嚭 mcount锛?```
	extern void mcount(void);
	EXPORT_SYMBOL(mcount);

```
### HAVE_FUNCTION_GRAPH_TRACER


娣卞惛涓€鍙ｆ皵鈥︹€︽槸鏃跺€欏共鐐圭湡娲讳簡銆傝繖閲屼綘闇€瑕佹洿鏂?mcount 鍑芥暟浠ユ鏌?ftrace 鍥?鍑芥暟鎸囬拡锛屽苟瀹炵幇涓€浜涘嚱鏁版潵淇濆瓨锛堝姭鎸侊級涓庢仮澶嶈繑鍥炲湴鍧€銆?
mcount 鍑芥暟搴旀鏌ュ嚱鏁版寚閽?ftrace_graph_return锛堜笌 ftrace_stub 姣旇緝锛夊拰
ftrace_graph_entry锛堜笌 ftrace_graph_entry_stub 姣旇緝锛夈€傚鏋滃叾涓换鎰忎竴涓湭琚?璁句负鐩稿簲鐨?stub 鍑芥暟锛屽垯璋冪敤鏋舵瀯鐩稿叧鐨勫嚱鏁?ftrace_graph_caller锛屽悗鑰呰繘鑰?璋冪敤鏋舵瀯鐩稿叧鐨勫嚱鏁?prepare_ftrace_return銆傝繖涓や釜鍑芥暟鍚嶉兘涓嶆槸纭€ц姹傜殑锛屼絾浣?浠嶅簲浣跨敤瀹冧滑锛屼互鍦ㄤ笉鍚屾灦鏋勭Щ妞嶄箣闂翠繚鎸佷竴鑷存€р€斺€斾究浜庢瘮杈冨拰瀵圭収銆?
prepare_ftrace_return 鐨勫弬鏁颁笌浼犵粰 ftrace_trace_function 鐨勭暐鏈変笉鍚屻€傜浜屼釜
鍙傛暟 鈥渟elfpc鈥?鐩稿悓锛屼絾绗竴涓弬鏁板簲鏄寚鍚?鈥渇rompc鈥?鐨勬寚閽堛€傞€氬父瀹冧綅浜庢爤涓娿€?杩欎娇寰楄鍑芥暟鍙互涓存椂鍔寔杩斿洖鍦板潃锛屼娇鍏舵寚鍚戞灦鏋勭浉鍏崇殑鍑芥暟 return_to_handler銆?璇ュ嚱鏁板彧闇€璋冪敤鍏叡鐨?ftrace_return_to_handler 鍑芥暟锛屽畠灏嗚繑鍥炲師濮嬬殑杩斿洖鍦板潃锛?鎹浣犲彲浠ヨ繑鍥炲埌鍘熷鐨勮皟鐢ㄧ偣銆?```
	void mcount(void)
	{
	...
		if (ftrace_trace_function != ftrace_stub)
			goto do_trace;

	+#ifdef CONFIG_FUNCTION_GRAPH_TRACER
	+	extern void (*ftrace_graph_return)(...);
	+	extern void (*ftrace_graph_entry)(...);
	+	if (ftrace_graph_return != ftrace_stub ||
	+	    ftrace_graph_entry != ftrace_graph_entry_stub)
	+		ftrace_graph_caller();
	+#endif

		/* restore any bare state */
	...
```
```
	#ifdef CONFIG_FUNCTION_GRAPH_TRACER
	void ftrace_graph_caller(void)
	{
		/* save all state needed by the ABI */

		unsigned long *frompc = &...;
		unsigned long selfpc = <return address> - MCOUNT_INSN_SIZE;
		/* passing frame pointer up is optional -- see below */
		prepare_ftrace_return(frompc, selfpc, frame_pointer);

		/* restore all state needed by the ABI */
	}
	#endif
```
鍏充簬濡備綍瀹炵幇 prepare_ftrace_return()锛屽彧闇€鏌ョ湅 x86 鐗堟湰鍗冲彲锛坒rame pointer
鐨勪紶閫掓槸鍙€夌殑锛涜瑙佷笅涓€鑺傦級銆傚叾涓敮涓€鏋舵瀯鐩稿叧鐨勯儴鍒嗘槸閿欒鎭㈠琛紙鍗?asm(...) 浠ｇ爜锛夌殑鎼缓銆傚叾浣欓儴鍒嗗湪鍚勬灦鏋勯棿搴斿綋鐩稿悓銆?
涓嬮潰鏄柊鐨?return_to_handler 姹囩紪鍑芥暟鐨勪吉浠ｇ爜銆傛敞鎰忥紝杩欓噷閫傜敤鐨?ABI 涓?mcount
浠ｇ爜閫傜敤鐨勪笉鍚屻€傜敱浜庝綘鏄粠涓€涓嚱鏁拌繑鍥烇紙鍦ㄥ熬澹颁箣鍚庯級锛屼綘鍙兘鍙互鐪佸幓閮ㄥ垎
淇濆瓨/鎭㈠鐨勭姸鎬侊紙閫氬父鍙槸鐢ㄤ簬浼犻€掕繑鍥炲€肩殑瀵勫瓨鍣級銆?```
	#ifdef CONFIG_FUNCTION_GRAPH_TRACER
	void return_to_handler(void)
	{
		/* save all state needed by the ABI (see paragraph above) */

		void (*original_return_point)(void) = ftrace_return_to_handler();

		/* restore all state needed by the ABI */

		/* this is usually either a return or a jump */
		original_return_point();
	}
	#endif

```
### HAVE_FUNCTION_GRAPH_FP_TEST


涓€涓灦鏋勫彲浠ュ悜鍑芥暟鐨勮繘鍏ヤ笌閫€鍑轰紶鍏ヤ竴涓敮涓€鐨勫€硷紙frame pointer锛夈€傚湪閫€鍑烘椂锛?璇ュ€间細琚瘮杈冿紝濡傛灉涓嶅尮閰嶏紝鍒欎細璁╁唴鏍?panic銆傝繖涓昏鏄 gcc 閿欒浠ｇ爜鐢熸垚鐨勪竴
绉嶅仴鍏ㄦ€ф鏌ャ€傚鏋滀綘鐨勭Щ妞嶇増鏈湪 gcc 涓嶅悓浼樺寲绾у埆涓嬭兘鍚堢悊鍦版洿鏂?frame pointer锛?閭ｄ箞鍙互蹇界暐姝ら€夐」銆?
涓嶈繃锛屼负鍏舵坊鍔犳敮鎸佸苟涓嶅お闅俱€傚湪浣犺皟鐢?prepare_ftrace_return() 鐨勬眹缂栦唬鐮佷腑锛?灏?frame pointer 浣滀负绗?3 涓弬鏁颁紶鍏ャ€傜劧鍚庡湪閭ｄ釜鍑芥暟鐨?C 鐗堟湰涓紝鍍?x86 绉绘
閭ｆ牱锛屽皢鍏朵紶閫掔粰 ftrace_push_return_trace()锛岃€屼笉鏄紶鍏?stub 鍊?0銆?
绫讳技鍦帮紝褰撲綘璋冪敤 ftrace_return_to_handler() 鏃讹紝灏?frame pointer 浼犵粰瀹冦€?
### HAVE_SYSCALL_TRACEPOINTS


浣犲彧闇€瑕佸緢灏戠殑涓滆タ灏辫兘鍦ㄦ煇涓灦鏋勪笂鑾峰緱绯荤粺璋冪敤璺熻釜銆?
  - 鏀寔 HAVE_ARCH_TRACEHOOK锛堣 arch/Kconfig锛夈€?  - 鍦?<asm/unistd.h> 涓湁涓€涓?NR_syscalls 鍙橀噺锛屾彁渚涜鏋舵瀯鏀寔鐨勭郴缁熻皟鐢?    鏁伴噺銆?  - 鏀寔 TIF_SYSCALL_TRACEPOINT 绾跨▼鏍囧織銆?  - 鍦?ptrace 鐨勭郴缁熻皟鐢ㄨ窡韪矾寰勪腑锛屼粠 ptrace 璋冪敤 trace_sys_enter() 鍜?    trace_sys_exit() tracepoint銆?  - 濡傛灉璇ユ灦鏋勪笂鐨勭郴缁熻皟鐢ㄨ〃姣斾竴涓湴鍧€鐨勭畝鍗曟暟缁勬洿澶嶆潅锛屽垯瀹炵幇涓€涓?    arch_syscall_addr 浠ヨ繑鍥炵粰瀹氱郴缁熻皟鐢ㄧ殑鍦板潃銆?  - 濡傛灉璇ユ灦鏋勪笂绯荤粺璋冪敤鐨勭鍙峰悕涓庡嚱鏁板悕涓嶅尮閰嶏紝鍒欏湪 asm/ftrace.h 涓畾涔?    ARCH_HAS_SYSCALL_MATCH_SYM_NAME 骞跺疄鐜?arch_syscall_match_sym_name锛屽姞鍏?    閫傚綋鐨勯€昏緫锛氳嫢鍔熻兘鍚嶄笌绗﹀彿鍚嶅搴斿垯杩斿洖 true銆?  - 灏嗚鏋舵瀯鏍囪涓?HAVE_SYSCALL_TRACEPOINTS銆?
### HAVE_DYNAMIC_FTRACE


璇﹁ scripts/recordmcount.pl銆傚彧闇€濉啓鏋舵瀯鐩稿叧缁嗚妭锛岃鏄庡浣曢€氳繃 objdump 瀹氫綅
mcount 璋冪敤鐐圭殑鍦板潃銆備笉瀹炵幇鍔ㄦ€?ftrace 鐨勮瘽锛屾閫夐」鎰忎箟涓嶅ぇ銆?
浣犻鍏堥渶瑕?HAVE_FUNCTION_TRACER锛屾墍浠ュ鏋滀綘杩囦簬蹇冩€ワ紝璇锋妸闃呰鍣ㄥ線鍥炴粴銆?
涓€鏃﹁繖浜涘氨缁紝浣犻渶瑕佸疄鐜帮細
 - asm/ftrace.h:
  - MCOUNT_ADDR
  - ftrace_call_adjust()
  - struct dyn_arch_ftrace{}
 - asm 浠ｇ爜:
  - mcount()锛堟柊 stub锛?  - ftrace_caller()
  - ftrace_call()
  - ftrace_stub()
 - C 浠ｇ爜:
  - ftrace_dyn_arch_init()
  - ftrace_make_nop()
  - ftrace_make_call()
  - ftrace_update_ftrace_func()

棣栧厛浣犻渶瑕佸湪 asm/ftrace.h 涓～鍐欎竴浜涙灦鏋勭粏鑺傘€?```
	#define MCOUNT_ADDR ((unsigned long)mcount)
```
```
	extern void mcount(void);
```
浣犺繕闇€瑕佽緟鍔╁嚱鏁?ftrace_call_adjust()銆傚ぇ澶氭暟浜?```
	static inline unsigned long ftrace_call_adjust(unsigned long addr)
	{
		return addr;
	}
```
<details to be filled>

鏈€鍚庯紝浣犻渶瑕佽嚜瀹氫箟鐨?dyn_arch_ftrace 缁撴瀯浣撱€傚鏋滃湪杩愯鏃剁粰浠绘剰璋冪敤鐐规墦琛ヤ竵鏃?闇€瑕佷竴浜涢澶栫姸鎬侊紝杩欏氨鏄?```
	struct dyn_arch_ftrace {
		/* No extra data needed */
	};
```
澶存枃浠跺鐞嗗畬鍚庯紝鎴戜滑鍙互濉啓姹囩紪浠ｇ爜銆傝櫧鐒跺墠闈㈡垜浠凡缁忓垱寤轰簡 mcount() 鍑芥暟锛?浣嗗姩鎬?ftrace 鍙渶瑕佷竴涓?stub 鍑芥暟銆傝繖鏄洜涓?mcount() 鍙細鍦ㄥ惎鍔ㄦ湡闂翠娇鐢紝涔嬪悗
鎵€鏈夊瀹冪殑寮曠敤閮戒細琚墦琛ヤ竵鏇挎崲鎺夛紝姘镐笉杩斿洖銆傚彇鑰屼唬涔嬬殑鏄紝鏃?mcount() 鐨勬牳蹇?灏嗚鐢ㄦ潵鍒涘缓涓€涓柊鐨?ftrace_caller() 鍑芥暟銆傜敱浜庝簩鑰呴毦浠ュ悎骞讹紝鏈€鐪佷簨鐨勫姙娉曞ぇ姒?鏄敤 #ifdef 鍒嗘垚涓や釜鐙珛鐨勫畾涔夈€俧trace_stub() 涔熸槸濡傛锛屽洜涓哄畠鐜板湪灏嗚鍐呰仈杩?ftrace_caller()銆?
鍦ㄦ洿鍥版儜涔嬪墠锛屾垜浠厛鐪嬩竴浜涗吉浠ｇ爜锛屼互渚夸綘
```
	void mcount(void)
	{
		return;
	}

	void ftrace_caller(void)
	{
		/* save all state needed by the ABI (see paragraph above) */

		unsigned long frompc = ...;
		unsigned long selfpc = <return address> - MCOUNT_INSN_SIZE;

	ftrace_call:
		ftrace_stub(frompc, selfpc);

		/* restore all state needed by the ABI */

	ftrace_stub:
		return;
	}
```
杩欎箥鐪嬪彲鑳芥湁鐐瑰鎬紝浣嗚璁颁綇鎴戜滑灏嗗湪杩愯鏃舵墦琛ヤ竵澶氬銆傞鍏堬紝鍙湁鎴戜滑鐪熸鎯?璺熻釜鐨勫嚱鏁版墠浼氳鎵撹ˉ涓佷互璋冪敤 ftrace_caller()銆傚叾娆★紝鐢变簬鎴戜滑鍚屼竴鏃堕棿鍙縺娲讳竴涓?璺熻釜鍣紝鎴戜滑浼氱粰 ftrace_caller() 鍑芥暟鏈韩鎵撹ˉ涓侊紝浠ヨ皟鐢ㄧ浉鍏崇殑閭ｄ釜璺熻釜鍣ㄣ€傝繖姝?鏄?ftrace_call 鏍囩鐨勭敤閫斻€?
閴翠簬姝わ紝璁╂垜浠户缁湅鐪熸鎵ц杩愯鏃舵墦琛ヤ竵鐨?C 浠ｇ爜銆傝搴﹁繃涓嬩竴鑺傦紝浣犻渶瑕佸
鑷繁鏋舵瀯鐨勬搷浣滅爜鏈変竴鐐逛簡瑙ｃ€?
姣忎釜鏋舵瀯閮芥湁涓€涓?init 鍥炶皟鍑芥暟銆傚鏋滀綘闇€瑕佸敖鏃╁仛浜涘垵濮嬪寲鐘舵€佺殑宸ヤ綔锛岃繖灏辨槸
鏃舵満銆傚惁鍒欙紝杩欎釜绠€鍗曠殑
```
	int __init ftrace_dyn_arch_init(void)
	{
		return 0;
	}
```
鏈変袱涓嚱鏁扮敤浜庡浠绘剰鍑芥暟杩涜杩愯鏃舵墦琛ヤ竵銆傜涓€涓敤浜庢妸 mcount 璋冪敤鐐瑰彉鎴?nop
锛堣繖姝ｆ湁鍔╀簬鎴戜滑鍦ㄤ笉璺熻釜鏃朵繚鎸佽繍琛屾椂鎬ц兘锛夈€傜浜屼釜鐢ㄤ簬鎶?mcount 璋冪敤鐐瑰彉鎴?瀵规煇涓换鎰忎綅缃殑璋冪敤锛堜絾閫氬父閭ｆ槸 ftracer_caller()锛夈€傚弬瑙?```
	ftrace_make_nop()
	ftrace_make_call()
```
rec->ip 鍊兼槸鍦ㄦ瀯寤烘湡鐢?scripts/recordmcount.pl 鏀堕泦鐨?mcount 璋冪敤鐐瑰湴鍧€銆?
鏈€鍚庝竴涓嚱鏁扮敤浜庡娲诲姩鐨勮窡韪櫒杩涜杩愯鏃舵墦琛ヤ竵銆傚畠灏嗕慨鏀?ftrace_caller()
鍑芥暟鍐?ftrace_call 绗﹀彿鎵€鍦ㄤ綅缃殑姹囩紪浠ｇ爜銆傚洜姝や綘搴斿湪璇ヤ綅缃繚鐣欒冻澶熺殑濉厖
锛坧adding锛変互鏀寔灏嗚鎻掑叆鐨勬柊鍑芥暟璋冪敤銆傛湁浜轰細鐢?鈥渃all鈥?绫绘寚浠わ紝涔熸湁浜轰細鐢?```
	ftrace_update_ftrace_func()


```
### HAVE_DYNAMIC_FTRACE + HAVE_FUNCTION_GRAPH_TRACER


鍑芥暟璺熻釜鍥惧櫒锛坒unction grapher锛夐渶瑕佷竴浜涘井璋冩墠鑳戒笌鍔ㄦ€?ftrace 閰嶅悎宸ヤ綔銆傚熀鏈?涓婏紝浣犻渶瑕侊細

 - 鏇存柊锛?  - ftrace_caller()
  - ftrace_graph_call()
  - ftrace_graph_caller()
 - 瀹炵幇锛?  - ftrace_enable_ftrace_graph_caller()
  - ftrace_disable_ftrace_graph_caller()

<details to be filled>

绠€瑕佽鏄庯細

 - 鍦?ftrace_call 浣嶇疆涔嬪悗娣诲姞涓€涓悕涓?ftrace_graph_call 鐨?nop stub锛?	  璇?stub 闇€瑕佽冻澶熷ぇ锛屼互鏀寔瀵?ftrace_graph_caller() 鐨勮皟鐢? - 鏇存柊 ftrace_graph_caller() 浠ラ厤鍚堣鏂扮殑 ftrace_caller() 璋冪敤锛屽洜涓洪儴鍒嗚涔?	  鍙兘宸叉敼鍙? - ftrace_enable_ftrace_graph_caller() 浼氬湪杩愯鏃跺皢 ftrace_graph_call 浣嶇疆
	  鎵撹ˉ涓佷负瀵?ftrace_graph_caller() 鐨勮皟鐢? - ftrace_disable_ftrace_graph_caller() 浼氬湪杩愯鏃跺皢 ftrace_graph_call 浣嶇疆
	  鎵撹ˉ涓佷负 nops
