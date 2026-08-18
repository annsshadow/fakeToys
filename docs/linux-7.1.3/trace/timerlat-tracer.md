# Timerlat 璺熻釜鍣?

timerlat 璺熻釜鍣ㄦ棬鍦ㄥ府鍔╂姠鍗犲紡鍐呮牳寮€鍙戣€呮壘鍒板疄鏃剁嚎绋嬪敜閱掑欢杩熺殑鏉ユ簮銆備笌 cyclictest 绫讳技锛岃璺熻釜鍣ㄨ缃竴涓懆鏈熸€у畾鏃跺櫒鏉ュ敜閱掍竴涓嚎绋嬨€傜劧鍚庤绾跨▼璁＄畻涓€涓?*鍞ら啋寤惰繜**鍊硷紝鍗?褰撳墠鏃堕棿**涓庡畾鏃跺櫒琚缃负鍒版湡鐨?*缁濆鏃堕棿*涔嬮棿鐨勫樊鍊笺€倀imerlat 鐨勪富瑕佺洰鏍囨槸浠ュ府鍔╁唴鏍稿紑鍙戣€呯殑鏂瑰紡杩涜璺熻釜銆?
### 鐢ㄦ硶


灏?ASCII 鏂囨湰 鈥渢imerlat鈥?鍐欏叆璺熻釜绯荤粺鐨?current_tracer 鏂囦欢锛堥€氬父鎸傝浇鍦?/sys/kernel/tracing锛夈€?
```

        [root@f32 ~]# cd /sys/kernel/tracing/
        [root@f32 tracing]# echo timerlat > current_tracer

```
```

  [root@f32 tracing]# cat trace
  # tracer: timerlat
  #
  #                              _-----=> irqs-off
  #                             / _----=> need-resched
  #                            | / _---=> hardirq/softirq
  #                            || / _--=> preempt-depth
  #                            || /
  #                            ||||             ACTIVATION
  #         TASK-PID      CPU# ||||   TIMESTAMP    ID            CONTEXT                LATENCY
  #            | |         |   ||||      |         |                  |                       |
          <idle>-0       [000] d.h1    54.029328: #1     context    irq timer_latency       932 ns
           <...>-867     [000] ....    54.029339: #1     context thread timer_latency     11700 ns
          <idle>-0       [001] dNh1    54.029346: #1     context    irq timer_latency      2833 ns
           <...>-868     [001] ....    54.029353: #1     context thread timer_latency      9820 ns
          <idle>-0       [000] d.h1    54.030328: #2     context    irq timer_latency       769 ns
           <...>-867     [000] ....    54.030330: #2     context thread timer_latency      3070 ns
          <idle>-0       [001] d.h1    54.030344: #2     context    irq timer_latency       935 ns
           <...>-868     [001] ....    54.030347: #2     context thread timer_latency      4351 ns


```
璇ヨ窡韪櫒鍒涘缓涓€涓叿鏈夊疄鏃朵紭鍏堢骇 SCHED_FIFO:95 鐨勬瘡 CPU 鍐呮牳绾跨▼锛屽湪姣忔婵€娲绘椂鎵撳嵃涓よ銆傜涓€琛屾槸鍦ㄧ嚎绋嬫縺娲?*涔嬪墠**銆佸湪**纭腑鏂?*涓婁笅鏂囪瀵熷埌鐨?*瀹氭椂鍣ㄥ欢杩?*銆傜浜岃鏄绾跨▼瑙傚療鍒扮殑**瀹氭椂鍣ㄥ欢杩?*銆侫CTIVATION ID 瀛楁鐢ㄤ簬灏?*irq**鎵ц涓庡叾鐩稿簲鐨?*绾跨▼**鎵ц鍏宠仈璧锋潵銆?
**irq**/**绾跨▼**鐨勬媶鍒嗗浜庢緞娓呭紓甯搁珮鐨勫€兼潵鑷摢涓笂涓嬫枃寰堥噸瑕併€?*irq** 涓婁笅鏂囧彲鑳借涓庣‖浠剁浉鍏崇殑鍔ㄤ綔寤惰繜锛屼緥濡?SMI銆丯MI銆両RQ锛屾垨鑰呰绾跨▼灞忚斀涓柇鎵€寤惰繜銆備竴鏃﹀畾鏃跺櫒瑙﹀彂锛屽欢杩熶篃鍙兘鍙楀埌绾跨▼寮曡捣鐨勯樆濉炵殑褰卞搷銆備緥濡傦紝閫氳繃 preempt_disable()銆佽皟搴﹀櫒鎵ц鎴栧睆钄戒腑鏂潵鎺ㄨ繜璋冨害鍣ㄦ墽琛屻€傜嚎绋嬩篃鍙兘琚叾浠栫嚎绋嬪拰 IRQ 鐨勫共鎵版墍寤惰繜銆?
### 璺熻釜鍣ㄩ€夐」


timerlat 璺熻釜鍣ㄥ缓绔嬪湪 osnoise 璺熻釜鍣ㄤ箣涓娿€傚洜姝ゅ畠鐨勯厤缃篃鍦?osnoise/ 閰嶇疆鐩綍涓畬鎴愩€倀imerlat 鐨勯厤缃湁锛?
 - cpus锛歵imerlat 绾跨▼灏嗗湪鍏朵笂鎵ц鐨?CPU銆? - timerlat_period_us锛歵imerlat 绾跨▼鐨勫懆鏈熴€? - stop_tracing_us锛氬鏋?*irq**涓婁笅鏂囦腑鐨勫畾鏃跺櫒寤惰繜楂樹簬閰嶇疆鐨勫€硷紝鍒欏仠姝㈢郴缁熻窡韪€傚啓鍏?0 浼氱鐢ㄦ閫夐」銆? - stop_tracing_total_us锛氬鏋?*绾跨▼**涓婁笅鏂囦腑鐨勫畾鏃跺櫒寤惰繜楂樹簬閰嶇疆鐨勫€硷紝鍒欏仠姝㈢郴缁熻窡韪€傚啓鍏?0 浼氱鐢ㄦ閫夐」銆? - print_stack锛氫繚瀛?IRQ 鍙戠敓鐨勬爤銆傝鏍堝湪**绾跨▼涓婁笅鏂?*浜嬩欢涔嬪悗鎵撳嵃锛屾垨鑰呭湪鍛戒腑 **stop_tracing_us** 鏃跺湪 IRQ 澶勭悊绋嬪簭涓墦鍗般€?
### timerlat 涓?osnoise


timerlat 涔熷彲浠ュ埄鐢?osnoise: traceevents銆?```

        [root@f32 ~]# cd /sys/kernel/tracing/
        [root@f32 tracing]# echo timerlat > current_tracer
        [root@f32 tracing]# echo 1 > events/osnoise/enable
        [root@f32 tracing]# echo 25 > osnoise/stop_tracing_total_us
        [root@f32 tracing]# tail -10 trace
             cc1-87882   [005] d..h...   548.771078: #402268 context    irq timer_latency     13585 ns
             cc1-87882   [005] dNLh1..   548.771082: irq_noise: local_timer:236 start 548.771077442 duration 7597 ns
             cc1-87882   [005] dNLh2..   548.771099: irq_noise: qxl:21 start 548.771085017 duration 7139 ns
             cc1-87882   [005] d...3..   548.771102: thread_noise:      cc1:87882 start 548.771078243 duration 9909 ns
      timerlat/5-1035    [005] .......   548.771104: #402268 context thread timer_latency     39960 ns

```
鍦ㄨ繖绉嶆儏鍐典笅锛屽畾鏃跺櫒寤惰繜鐨勬牴鏈師鍥犲苟涓嶆寚鍚戝崟涓€鍘熷洜锛岃€屾槸鎸囧悜澶氫釜鍘熷洜銆傞鍏堬紝瀹氭椂鍣?IRQ 琚欢杩熶簡 13 us锛岃繖鍙兘鎸囧悜涓€涓緝闀跨殑绂佺敤涓柇鍖烘锛堣 IRQ 鏍堣窡韪竴鑺傦級銆傜劧鍚庯紝鍞ら啋 timerlat 绾跨▼鐨勫畾鏃跺櫒涓柇鑺变簡 7597 ns锛岃€?qxl:21 璁惧 IRQ 鑺变簡 7139 ns銆傛渶鍚庯紝鍦ㄤ笂涓嬫枃鍒囨崲涔嬪墠锛宑c1 绾跨▼鍣０鍗犵敤浜?9909 ns 鐨勬椂闂淬€傝繖浜涜瘉鎹寮€鍙戣€呬娇鐢ㄥ叾浠栬窡韪柟娉曟潵寮勬竻濡備綍璋冭瘯鍜屼紭鍖栫郴缁熷緢鏈夊府鍔┿€?
鍊煎緱涓€鎻愮殑鏄紝osnoise: 浜嬩欢鎶ュ憡鐨?*duration**鍊兼槸**鍑€**鍊笺€備緥濡傦紝thread_noise 涓嶅寘鎷敱 IRQ 鎵ц寮曡捣鐨勫紑閿€鎸佺画鏃堕棿锛堝叾纭疄鍗犵敤浜?12736 ns锛夈€備絾 timerlat 璺熻釜鍣ㄦ姤鍛婄殑鍊硷紙timerlat_latency锛夋槸**姣?*鍊笺€?
涓嬮潰鐨勭ず鎰忓浘灞曠ず浜嗕竴鏉?CPU 鏃堕棿绾匡紝浠ュ強 timerlat 璺熻釜鍣ㄥ湪椤堕儴銆乷snoise: 浜嬩欢鍦ㄥ簳閮ㄥ浣曡瀵熷畠銆傛瘡涓?鈥?鈥?```

      External     timer irq                   thread
       clock        latency                    latency
       event        13585 ns                   39960 ns
         |             ^                         ^
         v             |                         |
         |-------------|                         |
         |-------------+-------------------------|
                       ^                         ^
  ========================================================================
                    [tmr irq]  [dev irq]
  [another thread...^       v..^       v.......][timerlat/ thread]  <-- CPU timeline
  =========================================================================
                    |-------|  |-------|
                            |--^       v-------|
                            |          |       |
                            |          |       + thread_noise: 9909 ns
                            |          +-> irq_noise: 6139 ns
                            +-> irq_noise: 7597 ns

```
### IRQ 鏍堣窡韪?

osnoise/print_stack 閫夐」瀵逛簬閭ｄ簺鐢变簬鎶㈠崰鎴?```

        [root@f32 tracing]# echo 500 > osnoise/stop_tracing_total_us
        [root@f32 tracing]# echo 500 > osnoise/print_stack
        [root@f32 tracing]# echo timerlat > current_tracer
        [root@f32 tracing]# tail -21 per_cpu/cpu7/trace
          insmod-1026    [007] dN.h1..   200.201948: irq_noise: local_timer:236 start 200.201939376 duration 7872 ns
          insmod-1026    [007] d..h1..   200.202587: #29800 context    irq timer_latency      1616 ns
          insmod-1026    [007] dN.h2..   200.202598: irq_noise: local_timer:236 start 200.202586162 duration 11855 ns
          insmod-1026    [007] dN.h3..   200.202947: irq_noise: local_timer:236 start 200.202939174 duration 7318 ns
          insmod-1026    [007] d...3..   200.203444: thread_noise:   insmod:1026 start 200.202586933 duration 838681 ns
      timerlat/7-1001    [007] .......   200.203445: #29800 context thread timer_latency    859978 ns
      timerlat/7-1001    [007] ....1..   200.203446: <stack trace>
  => timerlat_irq
  => __hrtimer_run_queues
  => hrtimer_interrupt
  => __sysvec_apic_timer_interrupt
  => asm_call_irq_on_stack
  => sysvec_apic_timer_interrupt
  => asm_sysvec_apic_timer_interrupt
  => delay_tsc
  => dummy_load_1ms_pd_init
  => do_one_initcall
  => do_init_module
  => __do_sys_finit_module
  => do_syscall_64
  => entry_SYSCALL_64_after_hwframe

```
绾跨▼鍣０鎴愪负瀵艰嚧瀹氭椂鍣ㄥ欢杩熺殑涓昏鍥犵礌鐨勬儏鍐靛緢鏈夊府鍔╋紝鍥犱负鍦?timerlat IRQ 澶勭悊绋嬪簭鏈熼棿淇濆瓨鐨勬爤璺熻釜鎸囧悜浜嗕竴涓悕涓?```

	static int __init dummy_load_1ms_pd_init(void)
	{
		preempt_disable();
		mdelay(1);
		preempt_enable();
		return 0;

	}

```
鐨勫嚱鏁般€?
### 鐢ㄦ埛绌洪棿鎺ュ彛


timerlat 鍏佽鐢ㄦ埛绌洪棿绾跨▼浣跨敤 timerlat 鍩虹璁炬柦鏉ユ祴閲忚皟搴﹀欢杩熴€傛鎺ュ彛鍙€氳繃 $tracing_dir/osnoise/per_cpu/cpu$ID/timerlat_fd 鍐呯殑姣?CPU 鏂囦欢鎻忚堪绗﹁闂€?
姝ゆ帴鍙ｅ湪浠ヤ笅鏉′欢涓嬪彲璁块棶锛?
 - timerlat 璺熻釜鍣ㄥ凡鍚敤
 - osnoise workload 閫夐」璁句负 NO_OSNOISE_WORKLOAD
 - 鐢ㄦ埛绌洪棿绾跨▼琚粦瀹氬埌鍗曚竴澶勭悊鍣? - 绾跨▼鎵撳紑浜嗕笌鍏跺崟涓€澶勭悊鍣ㄧ浉鍏宠仈鐨勬枃浠? - 涓€娆″彧鑳芥湁涓€涓嚎绋嬭闂鏂囦欢

濡傛灉涓嶆弧瓒充笂杩颁换浣曟潯浠讹紝open() 绯荤粺璋冪敤灏嗗け璐ャ€傛墦寮€鏂囦欢鎻忚堪绗﹀悗锛岀敤鎴风┖闂村彲浠ヤ粠涓鍙栥€?
read() 绯荤粺璋冪敤灏嗚繍琛屼竴娈?timerlat 浠ｇ爜锛屽畠浼氬儚甯歌鍐呮牳绾跨▼閭ｆ牱鍦ㄦ湭鏉ヨ缃畾鏃跺櫒骞剁瓑寰呭畠銆?
褰撳畾鏃跺櫒 IRQ 瑙﹀彂鏃讹紝timerlat IRQ 灏嗘墽琛岋紝鎶ュ憡 IRQ 寤惰繜骞跺敜閱掑湪 read 涓瓑寰呯殑绾跨▼銆傝绾跨▼灏嗚璋冨害锛屽苟鍍忓唴鏍哥嚎绋嬩竴鏍烽€氳繃璺熻釜鍣ㄦ姤鍛婄嚎绋嬪欢杩熴€?
涓庡唴鏍稿唴 timerlat 鐨勪笉鍚屼箣澶勫湪浜庯紝timerlat 涓嶄細閲嶆柊璁剧疆瀹氭椂鍣紝鑰屾槸杩斿洖鍒?read() 绯荤粺璋冪敤銆傛鏃讹紝鐢ㄦ埛鍙互杩愯浠讳綍浠ｇ爜銆?
濡傛灉鐢ㄦ埛閲嶆柊璇诲彇 timerlat 鏂囦欢鎻忚堪绗︼紝璺熻釜鍣ㄥ皢鎶ュ憡浠庣敤鎴风┖闂磋繑鍥炵殑寤惰繜锛屽嵆鎬诲欢杩熴€傚鏋滆繖鏄伐浣滅殑缁撴潫锛屽畠鍙互瑙ｉ噴涓鸿姹傜殑鍝嶅簲鏃堕棿銆?
鍦ㄦ姤鍛婃€诲欢杩熶箣鍚庯紝timerlat 灏嗛噸鍚惊鐜紝璁剧疆瀹氭椂鍣紝骞朵负涓嬩竴娆℃縺娲昏繘鍏ョ潯鐪犮€?
濡傛灉浠讳綍鏃跺€欐煇涓潯浠惰鐮村潖锛屼緥濡傜嚎绋嬪湪鐢ㄦ埛绌洪棿涓縼绉伙紝鎴栬€?timerlat 璺熻釜鍣ㄨ绂佺敤锛屽垯浼氬悜鐢ㄦ埛绌洪棿绾跨▼鍙戦€?SIG_KILL 淇″彿銆?
```

 int main(void)
 {
	char buffer[1024];
	int timerlat_fd;
	int retval;
	long cpu = 0;   /* 鏀剧疆浜?CPU 0 */
	cpu_set_t set;

	CPU_ZERO(&set);
	CPU_SET(cpu, &set);

	if (sched_setaffinity(gettid(), sizeof(set), &set) == -1)
		return 1;

	snprintf(buffer, sizeof(buffer),
		"/sys/kernel/tracing/osnoise/per_cpu/cpu%ld/timerlat_fd",
		cpu);

	timerlat_fd = open(buffer, O_RDONLY);
	if (timerlat_fd < 0) {
		printf("error opening %s: %s\n", buffer, strerror(errno));
		exit(1);
	}

	for (;;) {
		retval = read(timerlat_fd, buffer, 1024);
		if (retval < 0)
			break;
	}

	close(timerlat_fd);
	exit(0);
 }

```
