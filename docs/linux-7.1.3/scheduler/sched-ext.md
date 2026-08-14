
## 鍙墿灞曡皟搴︾被


sched_ext 鏄竴涓皟搴︾被锛屽叾琛屼负鍙互鐢变竴缁?BPF 绋嬪簭鈥斺€擝PF 璋冨害鍣ㄢ€斺€旀潵瀹氫箟銆?
- sched_ext 瀵煎嚭浜嗕竴涓畬鏁寸殑璋冨害鎺ュ彛锛屼粠鑰屽彲浠ュ湪鍏朵笂瀹炵幇浠绘剰璋冨害绠楁硶銆?
- BPF 璋冨害鍣ㄥ彲浠ヤ换鎰忓鍏惰涓哄悎閫傜殑鏂瑰紡瀵?CPU 杩涜鍒嗙粍锛屽苟灏嗗畠浠竴璧疯皟搴︼紝鍥犱负
  浠诲姟鍦ㄥ敜閱掓椂骞舵湭缁戝畾鍒扮壒瀹氱殑 CPU銆?
- BPF 璋冨害鍣ㄥ彲浠ラ殢鏃跺姩鎬佸紑鍚拰鍏抽棴銆?
- 鏃犺 BPF 璋冨害鍣ㄥ仛浠€涔堬紝绯荤粺瀹屾暣鎬ч兘寰楀埌淇濇寔銆傚湪浠讳綍鏃跺€欐娴嬪埌閿欒銆佸彲杩愯浠诲姟
  鍋滄粸锛屾垨璋冪敤 SysRq 閿簭鍒?`SysRq-S` 鏃讹紝閮戒細鎭㈠榛樿鐨勮皟搴﹁涓恒€?
- 褰?BPF 璋冨害鍣ㄨЕ鍙戦敊璇椂锛屼細杞偍璋冭瘯淇℃伅浠ヨ緟鍔╄皟璇曘€傝皟璇曡浆鍌ㄤ細浼犻€掔粰璋冨害鍣ㄤ簩杩涘埗
  骞剁敱鍏舵墦鍗般€備篃鍙互閫氳繃 `sched_ext_dump` tracepoint 璁块棶璋冭瘯杞偍銆係ysRq 閿簭鍒?  `SysRq-D` 浼氳Е鍙戣皟璇曡浆鍌ㄣ€傝繖涓嶄細缁堟 BPF 璋冨害鍣紝骞朵笖鍙兘閫氳繃 tracepoint 璇诲彇銆?
## 鍒囨崲杩涘嚭 sched_ext


`CONFIG_SCHED_CLASS_EXT` 鏄惎鐢?sched_ext 鐨勯厤缃€夐」锛宍tools/sched_ext` 鍖呭惈绀轰緥
璋冨害鍣ㄣ€傚簲浣跨敤浠ヤ笅閰嶇疆閫夐」鏉ヤ娇鐢?sched_ext锛?

    CONFIG_BPF=y
    CONFIG_SCHED_CLASS_EXT=y
    CONFIG_BPF_SYSCALL=y
    CONFIG_BPF_JIT=y
    CONFIG_DEBUG_INFO_BTF=y
    CONFIG_BPF_JIT_ALWAYS_ON=y
    CONFIG_BPF_JIT_DEFAULT_ON=y

sched_ext 浠呭湪 BPF 璋冨害鍣ㄥ凡鍔犺浇骞惰繍琛屾椂浣跨敤銆?
濡傛灉浠诲姟鏄惧紡鍦板皢鍏惰皟搴︾瓥鐣ヨ缃负 `SCHED_EXT`锛屽湪 BPF 璋冨害鍣ㄥ姞杞戒箣鍓嶏紝瀹冨皢琚綋浣?`SCHED_NORMAL` 骞剁敱鍏钩绫昏皟搴﹀櫒璋冨害銆?
褰?BPF 璋冨害鍣ㄥ凡鍔犺浇涓?`ops->flags` 涓湭璁剧疆 `SCX_OPS_SWITCH_PARTIAL` 鏃讹紝鎵€鏈?`SCHED_NORMAL`銆乣SCHED_BATCH`銆乣SCHED_IDLE` 鍜?`SCHED_EXT` 浠诲姟閮界敱 sched_ext 璋冨害銆?
鐒惰€岋紝褰?BPF 璋冨害鍣ㄥ凡鍔犺浇涓?`ops->flags` 涓缃簡 `SCX_OPS_SWITCH_PARTIAL` 鏃讹紝
鍙湁鍏锋湁 `SCHED_EXT` 绛栫暐鐨勪换鍔＄敱 sched_ext 璋冨害锛岃€屽叿鏈?`SCHED_NORMAL`銆?`SCHED_BATCH` 鍜?`SCHED_IDLE` 绛栫暐鐨勪换鍔＄敱鍏钩绫昏皟搴﹀櫒璋冨害锛屽悗鑰呯殑 sched_class
浼樺厛绾ч珮浜?`SCHED_EXT`銆?
缁堟 sched_ext 璋冨害鍣ㄧ▼搴忋€佽Е鍙?`SysRq-S`锛屾垨妫€娴嬪埌鍖呮嫭鍙繍琛屼换鍔″仠婊炲湪鍐呯殑浠讳綍
鍐呴儴閿欒锛岄兘浼氫腑姝?BPF 璋冨害鍣ㄥ苟灏嗘墍鏈変换鍔′氦杩樼粰鍏钩绫昏皟搴﹀櫒銆?

    # make -j16 -C tools/sched_ext
    # tools/sched_ext/build/bin/scx_simple
    local=0 global=3
    local=5 global=24
    local=9 global=44
    local=13 global=56
    local=17 global=72
    ^CEXIT: BPF scheduler unregistered

BPF 璋冨害鍣ㄧ殑褰撳墠鐘舵€佸彲濡備笅纭畾锛?

    # cat /sys/kernel/sched_ext/state
    enabled
    # cat /sys/kernel/sched_ext/root/ops
    simple

浣犲彲浠ラ€氳繃妫€鏌ヨ繖涓崟璋冮€掑璁℃暟鍣ㄦ潵鍒ゆ柇鑷惎鍔ㄤ互鏉ユ槸鍚︽浘鍔犺浇杩囦换浣?BPF 璋冨害鍣?锛堝€间负闆惰〃绀哄皻鏈姞杞戒换浣?BPF 璋冨害鍣級锛?

    # cat /sys/kernel/sched_ext/enable_seq
    1

姣忎釜姝ｅ湪杩愯鐨勮皟搴﹀櫒杩樹細鍦?`/sys/kernel/sched_ext/<scheduler-name>/events` 涓嬫毚闇?涓€涓瘡璋冨害鍣ㄧ殑 `events` 鏂囦欢锛岀敤浜庤窡韪瘖鏂鏁板櫒銆傛瘡涓鏁板櫒鍗犱竴琛?`name value`锛?

    # cat /sys/kernel/sched_ext/simple/events
    SCX_EV_SELECT_CPU_FALLBACK 0
    SCX_EV_DISPATCH_LOCAL_DSQ_OFFLINE 0
    SCX_EV_DISPATCH_KEEP_LAST 123
    SCX_EV_ENQ_SKIP_EXITING 0
    SCX_EV_ENQ_SKIP_MIGRATION_DISABLED 0
    SCX_EV_REENQ_IMMED 0
    SCX_EV_REENQ_LOCAL_REPEAT 0
    SCX_EV_REFILL_SLICE_DFL 456789
    SCX_EV_BYPASS_DURATION 0
    SCX_EV_BYPASS_DISPATCH 0
    SCX_EV_BYPASS_ACTIVATE 0
    SCX_EV_INSERT_NOT_OWNED 0
    SCX_EV_SUB_BYPASS_DISPATCH 0

杩欎簺璁℃暟鍣ㄥ湪 `kernel/sched/ext_internal.h` 涓湁鎻忚堪锛涚畝瑕佸湴璇达細

- `SCX_EV_SELECT_CPU_FALLBACK`锛歰ps.select_cpu() 杩斿洖浜嗕竴涓换鍔′笉鍙敤鐨?CPU锛屾牳蹇?  璋冨害鍣ㄩ潤榛樺湴閫夋嫨浜嗕竴涓洖閫€ CPU銆?- `SCX_EV_DISPATCH_LOCAL_DSQ_OFFLINE`锛氱敱浜庣洰鏍?CPU 涓嬬嚎锛屾湰鍦?DSQ 鍒嗗彂琚噸瀹氬悜鍒?  鍏ㄥ眬 DSQ銆?- `SCX_EV_DISPATCH_KEEP_LAST`锛氱敱浜庢病鏈夊叾瀹冨彲鐢ㄤ换鍔★紝涓€涓换鍔＄户缁繍琛岋紙浠呭綋鏈缃?  `SCX_OPS_ENQ_LAST` 鏃讹級銆?- `SCX_EV_ENQ_SKIP_EXITING`锛氫竴涓鍦ㄩ€€鍑虹殑浠诲姟琚洿鎺ュ垎鍙戝埌鏈湴 DSQ锛岀粫杩囦簡
  ops.enqueue()锛堜粎褰撴湭璁剧疆 `SCX_OPS_ENQ_EXITING` 鏃讹級銆?- `SCX_EV_ENQ_SKIP_MIGRATION_DISABLED`锛氫竴涓鐢ㄤ簡杩佺Щ鐨勪换鍔¤鐩存帴鍒嗗彂鍒板叾鏈湴 DSQ
  锛堜粎褰撴湭璁剧疆 `SCX_OPS_ENQ_MIGRATION_DISABLED` 鏃讹級銆?- `SCX_EV_REENQ_IMMED`锛氫竴涓互 `SCX_ENQ_IMMED` 鍒嗗彂鐨勪换鍔＄敱浜庣洰鏍?CPU 鏃犳硶绔嬪嵆鎵ц
  鑰岃閲嶆柊鍏ラ槦銆?- `SCX_EV_REENQ_LOCAL_REPEAT`锛氭湰鍦?DSQ 鐨勯噸鏂板叆闃熻Е鍙戜簡鍙︿竴娆￠噸鏂板叆闃燂紱鍙嶅鍑虹幇
  鐨勮鏁拌〃鏄?BPF 璋冨害鍣ㄤ腑 `SCX_ENQ_REENQ` 澶勭悊涓嶆纭€?- `SCX_EV_REFILL_SLICE_DFL`锛氫换鍔＄殑鏃堕棿鐗囪浠ラ粯璁ゅ€硷紙`SCX_SLICE_DFL`锛夎ˉ瓒炽€?- `SCX_EV_BYPASS_DURATION`锛氬湪 bypass 妯″紡涓嬭姳璐圭殑鎬荤撼绉掓暟銆?- `SCX_EV_BYPASS_DISPATCH`锛氬湪 bypass 妯″紡涓嬪垎鍙戠殑浠诲姟鏁般€?- `SCX_EV_BYPASS_ACTIVATE`锛歜ypass 妯″紡琚縺娲荤殑娆℃暟銆?- `SCX_EV_INSERT_NOT_OWNED`锛氳瘯鍥惧皢涓€涓笉灞炰簬姝よ皟搴﹀櫒鐨勪换鍔℃彃鍏?DSQ锛涙绫诲皾璇曚細琚?  闈欓粯蹇界暐銆?- `SCX_EV_SUB_BYPASS_DISPATCH`锛氫粠瀛愯皟搴﹀櫒 bypass DSQ 鍒嗗彂鐨勪换鍔★紙浠呬笌
  `CONFIG_EXT_SUB_SCHED` 鐩稿叧锛夈€?
`tools/sched_ext/scx_show_state.py` 鏄竴涓?drgn 鑴氭湰锛屽彲鏄剧ず鏇磋缁嗙殑淇℃伅锛?

    # tools/sched_ext/scx_show_state.py
    ops           : simple
    enabled       : 1
    switching_all : 1
    switched_all  : 1
    enable_state  : enabled (2)
    bypass_depth  : 0
    nr_rejected   : 0
    enable_seq    : 1

鏌愪釜缁欏畾浠诲姟鏄惁浣嶄簬 sched_ext 涓婂彲濡備笅纭畾锛?

    # grep ext /proc/self/sched
    ext.enabled                                  :                    1

## 鍩虹


鐢ㄦ埛绌洪棿鍙互閫氳繃鍔犺浇涓€缁勫疄鐜?`struct sched_ext_ops` 鐨?BPF 绋嬪簭鏉ュ疄鐜颁换鎰?BPF
璋冨害鍣ㄣ€傚敮涓€鐨勫繀濉瓧娈垫槸 `ops.name`锛屽畠蹇呴』鏄竴涓湁鏁堢殑 BPF 瀵硅薄鍚嶃€傛墍鏈夋搷浣滈兘鏄?鍙€夌殑銆備互涓嬬粡杩囦慨鏀圭殑鎽樺綍鏉ヨ嚜 `tools/sched_ext/scx_simple.bpf.c`锛屽睍绀轰簡涓€涓?鏈€灏忕殑鍏ㄥ眬 FIFO 璋冨害鍣ㄣ€?

    /*
     - Decide which CPU a task should be migrated to before being
     - enqueued (either at wakeup, fork time, or exec time). If an
     - idle core is found by the default ops.select_cpu() implementation,
     - then insert the task directly into SCX_DSQ_LOCAL and skip the
     - ops.enqueue() callback.
     *
     - Note that this implementation has exactly the same behavior as the
     - default ops.select_cpu implementation. The behavior of the scheduler
     - would be exactly same if the implementation just didn't define the
     - simple_select_cpu() struct_ops prog.
     */
    s32 BPF_STRUCT_OPS(simple_select_cpu, struct task_struct *p,
                       s32 prev_cpu, u64 wake_flags)
    {
            s32 cpu;
            /** Need to initialize or the BPF verifier will reject the program **/
            bool direct = false;

            cpu = scx_bpf_select_cpu_dfl(p, prev_cpu, wake_flags, &direct);

            if (direct)
                    scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, 0);

            return cpu;
    }

    /*
     - Do a direct insertion of a task to the global DSQ. This ops.enqueue()
     - callback will only be invoked if we failed to find a core to insert
     - into in ops.select_cpu() above.
     *
     - Note that this implementation has exactly the same behavior as the
     - default ops.enqueue implementation, which just dispatches the task
     - to SCX_DSQ_GLOBAL. The behavior of the scheduler would be exactly same
     - if the implementation just didn't define the simple_enqueue struct_ops
     - prog.
     */
    void BPF_STRUCT_OPS(simple_enqueue, struct task_struct *p, u64 enq_flags)
    {
            scx_bpf_dsq_insert(p, SCX_DSQ_GLOBAL, SCX_SLICE_DFL, enq_flags);
    }

    s32 BPF_STRUCT_OPS_SLEEPABLE(simple_init)
    {
            /*
             - By default, all SCHED_EXT, SCHED_OTHER, SCHED_IDLE, and
             - SCHED_BATCH tasks should use sched_ext.
             */
            return 0;
    }

    void BPF_STRUCT_OPS(simple_exit, struct scx_exit_info *ei)
    {
            exit_type = ei->type;
    }

    SEC(".struct_ops")
    struct sched_ext_ops simple_ops = {
            .select_cpu             = (void *)simple_select_cpu,
            .enqueue                = (void *)simple_enqueue,
            .init                   = (void *)simple_init,
            .exit                   = (void *)simple_exit,
            .name                   = "simple",
    };

### 鍒嗗彂闃熷垪


涓轰簡鍖归厤璋冨害鍣ㄦ牳蹇冧笌 BPF 璋冨害鍣ㄤ箣闂寸殑闃绘姉锛宻ched_ext 浣跨敤 DSQ锛堝垎鍙戦槦鍒楋級锛屽畠鍙互
鍚屾椂浣滀负 FIFO 鍜屼紭鍏堢骇闃熷垪杩愯銆傞粯璁ゆ儏鍐典笅锛屾湁涓€涓叏灞€ FIFO锛坄SCX_DSQ_GLOBAL`锛夊拰
姣忎釜 CPU 涓€涓湰鍦?DSQ锛坄SCX_DSQ_LOCAL`锛夈€侭PF 璋冨害鍣ㄥ彲浠ヤ娇鐢?`scx_bpf_create_dsq()`
鍜?`scx_bpf_destroy_dsq()` 绠＄悊浠绘剰鏁伴噺鐨?DSQ銆?
CPU 鎬绘槸鎵ц鍏舵湰鍦?DSQ 涓殑浠诲姟銆備换鍔¤鈥滄彃鍏モ€濆埌涓€涓?DSQ 涓€備綅浜庨潪鏈湴 DSQ 涓殑
浠诲姟琚€滅Щ鍔ㄢ€濆埌鐩爣 CPU 鐨勬湰鍦?DSQ銆?
褰?CPU 瀵绘壘涓嬩竴涓杩愯鐨勪换鍔℃椂锛屽鏋滄湰鍦?DSQ 涓嶄负绌猴紝鍒欓€夊彇绗竴涓换鍔°€傚惁鍒欙紝CPU
灏濊瘯浠庡叏灞€ DSQ 绉诲姩涓€涓换鍔°€傚鏋滈偅涔熸病鏈変骇鐢熷彲杩愯浠诲姟锛屽垯璋冪敤 `ops.dispatch()`銆?
### 璋冨害鍛ㄦ湡


浠ヤ笅绠€瑕佸睍绀轰簡鍞ら啋鐨勪换鍔″浣曡璋冨害鍜屾墽琛屻€?
1. 褰撲竴涓换鍔¤鍞ら啋鏃讹紝`ops.select_cpu()` 鏄涓€涓璋冪敤鐨勬搷浣溿€傝繖鏈変袱涓洰鐨勩€?   绗竴锛孋PU 閫夋嫨浼樺寲鎻愮ず銆傜浜岋紝濡傛灉绌洪棽鍒欏敜閱掓墍閫?CPU銆?
   `ops.select_cpu()` 閫夋嫨鐨?CPU 鏄竴涓紭鍖栨彁绀鸿€岄潪缁戝畾銆傛渶缁堢殑鍐冲畾鍦ㄨ皟搴︾殑鏈€鍚庝竴
   姝ュ仛鍑恒€傜劧鑰岋紝濡傛灉 `ops.select_cpu()` 杩斿洖鐨?CPU 涓庝换鍔℃渶缁堣繍琛岀殑 CPU 鐩稿尮閰嶏紝
   浼氭湁灏忓皬鐨勬€ц兘鏀剁泭銆?
   閫夋嫨 CPU 鐨勪竴涓壇浣滅敤鏄皢瀹冧粠绌洪棽涓敜閱掋€傝櫧鐒?BPF 璋冨害鍣ㄥ彲浠ヤ娇鐢?`scx_bpf_kick_cpu()`
   杈呭姪鍑芥暟鍞ら啋浠讳綍 CPU锛屼絾鏄庢櫤鍦颁娇鐢?`ops.select_cpu()` 鍙互鏇寸畝鍗曘€佹洿楂樻晥銆?
   娉ㄦ剰锛岃皟搴﹀櫒鏍稿績浼氬拷鐣ユ棤鏁堢殑 CPU 閫夋嫨锛屼緥濡傦紝濡傛灉瀹冭秴鍑轰簡浠诲姟鐨勫厑璁?cpumask銆?
   涓€涓换鍔″彲浠ラ€氳繃璋冪敤 `scx_bpf_dsq_insert()` 鎴?`scx_bpf_dsq_insert_vtime()`
   浠?`ops.select_cpu()` 鐩存帴鎻掑叆鍒颁竴涓?DSQ 涓€?
   濡傛灉涓€涓换鍔′粠 `ops.select_cpu()` 琚彃鍏ュ埌 `SCX_DSQ_LOCAL`锛屽畠灏嗚娣诲姞鍒颁粠
   `ops.select_cpu()` 杩斿洖鐨勯偅涓?CPU 鐨勬湰鍦?DSQ 涓€傛澶栵紝浠?`ops.select_cpu()`
   鐩存帴鎻掑叆灏嗗鑷磋烦杩?`ops.enqueue()` 鍥炶皟銆?
   浠讳綍鍏跺畠灏嗕换鍔″瓨鍌ㄥ湪 BPF 鍐呴儴鏁版嵁缁撴瀯涓殑灏濊瘯骞朵笉鑳介樆姝?`ops.enqueue()` 琚皟鐢ㄣ€?   杩欎笉榧撳姳杩欐牱鍋氾紝鍥犱负瀹冨彲鑳藉紩鍏ョ珵鎬佽涓烘垨涓嶄竴鑷寸姸鎬併€?
2. 涓€鏃︾洰鏍?CPU 琚€夊畾锛屽氨浼氳皟鐢?`ops.enqueue()`锛堥櫎闈炰换鍔℃槸浠?`ops.select_cpu()`
   鐩存帴鎻掑叆鐨勶級銆俙ops.enqueue()` 鍙互鍋氬嚭浠ヤ笅鍐冲畾涔嬩竴锛?
   - 閫氳繃璋冪敤甯︿互涓嬮€夐」涔嬩竴鐨?`scx_bpf_dsq_insert()` 灏嗕换鍔＄珛鍗虫彃鍏ュ叏灞€鎴栨湰鍦?DSQ锛?     `SCX_DSQ_GLOBAL`銆乣SCX_DSQ_LOCAL` 鎴?`SCX_DSQ_LOCAL_ON | cpu`銆?
   - 閫氳繃璋冪敤甯︽湁灏忎簬 2^63 鐨?DSQ ID 鐨?`scx_bpf_dsq_insert()` 灏嗕换鍔＄珛鍗虫彃鍏ヨ嚜瀹氫箟
     DSQ銆?
   - 鍦?BPF 渚у皢浠诲姟鎺掗槦銆?
   **浠诲姟鐘舵€佽窡韪笌 ops.dequeue() 璇箟**

   褰?BPF 璋冨害鍣ㄨ礋璐ｇ鐞嗕竴涓换鍔＄殑鐢熷懡鍛ㄦ湡鏃讹紝璇ヤ换鍔″浜庘€淏PF 璋冨害鍣ㄧ殑鐩戠锛坈ustody锛夆€?   涔嬩腑銆傚綋涓€涓换鍔¤鍒嗗彂鍒扮敤鎴?DSQ 鎴栧瓨鍌ㄥ湪 BPF 璋冨害鍣ㄧ殑鍐呴儴鏁版嵁缁撴瀯涓椂锛屽畠杩涘叆
   鐩戠鐘舵€併€傚浜庤繖浜涙搷浣滐紝鐩戠鍙粠 `ops.enqueue()` 杩涘叆銆傚敮涓€鐨勪緥澶栨槸浠?   `ops.select_cpu()` 鍒嗗彂鍒扮敤鎴?DSQ锛氬敖绠″湪閭ｆ椂璇ヤ换鍔″湪鎶€鏈笂灏氭湭澶勪簬 BPF 璋冨害鍣?   鐩戠涓紝浣嗗浜庝笌鐩戠鐩稿叧鐨勭洰鐨勮€岃█锛岃鍒嗗彂鍏锋湁涓庝粠 `ops.enqueue()` 鍒嗗彂鐩稿悓鐨?   璇箟鏁堟灉銆?
   涓€鏃﹁皟鐢ㄤ簡 `ops.enqueue()`锛屾牴鎹皟搴﹀櫒鐨勮涓猴紝浠诲姟鍙兘浼氭垨鍙兘涓嶄細杩涘叆鐩戠锛?
   - **鐩存帴鍒嗗彂鍒扮粓缁?DSQ**锛坄SCX_DSQ_LOCAL`銆乣SCX_DSQ_LOCAL_ON | cpu` 鎴?     `SCX_DSQ_GLOBAL`锛夛細BPF 璋冨害鍣ㄥ璇ヤ换鍔＄殑澶勭悊宸插畬鎴愨€斺€斿畠瑕佷箞鐩存帴杩涘叆 CPU 鐨勬湰鍦?     杩愯闃熷垪锛岃涔堜綔涓哄洖閫€杩涘叆鍏ㄥ眬 DSQ銆備换鍔℃案杩滀笉浼氳繘鍏ワ紙鎴栭€€鍑猴級BPF 鐩戠锛屽苟涓?     涓嶄細璋冪敤 `ops.dequeue()`銆?
   - **鍒嗗彂鍒扮敤鎴峰垱寤虹殑 DSQ**锛堣嚜瀹氫箟 DSQ锛夛細浠诲姟杩涘叆 BPF 璋冨害鍣ㄧ殑鐩戠銆傚綋浠诲姟绋嶅悗
     绂诲紑 BPF 鐩戠锛堣鍒嗗彂鍒扮粓缁?DSQ銆佽鏍稿績璋冨害閫変腑锛屾垨鍥犵潯鐪?灞炴€у彉鏇磋€屽嚭闃燂級鏃讹紝
     `ops.dequeue()` 灏嗚鎭板ソ璋冪敤涓€娆°€?
   - **瀛樺偍鍦?BPF 鏁版嵁缁撴瀯涓?*锛堜緥濡傚唴閮?BPF 闃熷垪锛夛細浠诲姟澶勪簬 BPF 鐩戠涓€傚綋浠诲姟
     绂诲紑鏃讹紙渚嬪锛屽綋 `ops.dispatch()` 灏嗗畠绉诲姩鍒扮粓缁?DSQ锛屾垨鍙戠敓灞炴€у彉鏇?鐫＄湢鏃讹級锛?     灏嗚皟鐢?`ops.dequeue()`銆?
   褰撲换鍔＄寮€ BPF 璋冨害鍣ㄧ洃绠℃椂锛屼細璋冪敤 `ops.dequeue()`銆傚嚭闃熷彲鑳藉洜涓嶅悓鍘熷洜鍙戠敓锛岀敱
   鏍囧織鍖哄垎锛?
   1. **甯歌鍒嗗彂**锛氬綋澶勪簬 BPF 鐩戠涓殑浠诲姟浠?`ops.dispatch()` 琚垎鍙戝埌缁堢粨 DSQ
      锛堢寮€ BPF 鐩戠浠ユ墽琛岋級鏃讹紝浼氳Е鍙?`ops.dequeue()`锛屼笉甯︿换浣曠壒娈婃爣蹇椼€?
   2. **鏍稿績璋冨害閫夊彇**锛氬綋鍚敤 `CONFIG_SCHED_CORE` 涓旀牳蹇冭皟搴﹀湪璇ヤ换鍔′粛澶勪簬 BPF 鐩戠
      涓椂閫夊彇瀹冩潵鎵ц锛宍ops.dequeue()` 浼氬甫鏈?`SCX_DEQ_CORE_SCHED_EXEC` 鏍囧織琚皟鐢ㄣ€?
   3. **璋冨害灞炴€у彉鏇?*锛氬綋浠诲姟灞炴€у彂鐢熷彉鍖栵紙閫氳繃 `sched_setaffinity()`銆?      `sched_setscheduler()`銆佷紭鍏堢骇鍙樻洿銆丆PU 杩佺Щ绛夋搷浣滐級鑰屼换鍔′粛澶勪簬 BPF 鐩戠涓椂锛?      `ops.dequeue()` 浼氳璋冪敤锛屽苟鍦?`deq_flags` 涓缃?`SCX_DEQ_SCHED_CHANGE` 鏍囧織銆?
   **閲嶈**锛氫竴鏃︿换鍔＄寮€浜?BPF 鐩戠锛堜緥濡傝鍒嗗彂鍒扮粓缁?DSQ 涔嬪悗锛夛紝灞炴€у彉鏇村皢涓嶄細瑙﹀彂
   `ops.dequeue()`锛屽洜涓鸿浠诲姟涓嶅啀鐢?BPF 璋冨害鍣ㄧ鐞嗐€?
3. 褰撲竴涓?CPU 鍑嗗濂借皟搴︽椂锛屽畠棣栧厛鏌ョ湅鍏舵湰鍦?DSQ銆傚鏋滀负绌猴紝鍒欐煡鐪嬪叏灞€ DSQ銆傚鏋?   浠嶇劧娌℃湁鍙繍琛岀殑浠诲姟锛屽垯璋冪敤 `ops.dispatch()`锛屽畠鍙互浣跨敤浠ヤ笅涓や釜鍑芥暟鏉ュ～鍏呮湰鍦?   DSQ銆?
   - `scx_bpf_dsq_insert()` 灏嗕竴涓换鍔℃彃鍏?DSQ銆傚彲浠ヤ娇鐢ㄤ换浣曠洰鏍?DSQ鈥斺€擿SCX_DSQ_LOCAL`銆?     `SCX_DSQ_LOCAL_ON | cpu`銆乣SCX_DSQ_GLOBAL` 鎴栬嚜瀹氫箟 DSQ銆傝櫧鐒?`scx_bpf_dsq_insert()`
     鐩墠涓嶈兘鍦ㄦ寔鏈?BPF 閿佺殑鎯呭喌涓嬭皟鐢紝浣嗚繖涓€闄愬埗姝ｅ湪鏀硅繘涓苟灏嗚鏀寔銆?     `scx_bpf_dsq_insert()` 瀹夋帓鎻掑叆鑰岄潪绔嬪嵆鎵ц銆傛渶澶氬彲浠ユ湁 `ops.dispatch_max_batch`
     涓緟澶勭悊浠诲姟銆?
   - `scx_bpf_dsq_move_to_local()` 灏嗕换鍔′粠鎸囧畾鐨勯潪鏈湴 DSQ 绉诲姩鍒版鍦ㄥ垎鍙戠殑 DSQ銆?     姝ゅ嚱鏁颁笉鑳藉湪鎸佹湁浠讳綍 BPF 閿佺殑鎯呭喌涓嬭皟鐢ㄣ€俙scx_bpf_dsq_move_to_local()` 鍦ㄥ皾璇曚粠
     鎸囧畾 DSQ 绉诲姩涔嬪墠浼氬埛鏂板緟澶勭悊鐨勬彃鍏ヤ换鍔°€?
4. `ops.dispatch()` 杩斿洖鍚庯紝濡傛灉鏈湴 DSQ 涓湁浠诲姟锛孋PU 杩愯绗竴涓€傚鏋滀负绌猴紝鍒欓噰鍙?   浠ヤ笅姝ラ锛?
   - 灏濊瘯浠庡叏灞€ DSQ 绉诲姩銆傚鏋滄垚鍔燂紝杩愯璇ヤ换鍔°€?
   - 濡傛灉 `ops.dispatch()` 宸插垎鍙戣繃浠讳綍浠诲姟锛岄噸璇?#3銆?
   - 濡傛灉鍓嶄竴涓换鍔℃槸 SCX 浠诲姟涓斾粛鐒跺彲杩愯锛岀户缁繍琛屽畠锛堣 `SCX_OPS_ENQ_LAST`锛夈€?
   - 杩涘叆绌洪棽銆?
娉ㄦ剰锛孊PF 璋冨害鍣ㄦ€绘槸鍙互閫夋嫨鍦?`ops.enqueue()` 涓珛鍗冲垎鍙戜换鍔★紝濡備笂闈㈢殑绠€鍗曠ず渚?鎵€绀恒€傚鏋滃彧浣跨敤鍐呯疆 DSQ锛屽垯鏃犻渶瀹炵幇 `ops.dispatch()`锛屽洜涓轰换鍔℃案杩滀笉浼氬湪 BPF
璋冨害鍣ㄤ笂鎺掗槦锛屽苟涓旀湰鍦板拰鍏ㄥ眬 DSQ 閮戒細鑷姩鎵ц銆?
`scx_bpf_dsq_insert()` 灏嗕换鍔℃彃鍏ョ洰鏍?DSQ 鐨?FIFO銆傚浼樺厛绾ч槦鍒楄浣跨敤
`scx_bpf_dsq_insert_vtime()`銆傚唴閮?DSQ锛堝 `SCX_DSQ_LOCAL` 鍜?`SCX_DSQ_GLOBAL`锛変笉
鏀寔浼樺厛绾ч槦鍒楀垎鍙戯紝蹇呴』鐢?`scx_bpf_dsq_insert()` 鍒嗗彂銆傛洿澶氫俊鎭鍙傞槄
`tools/sched_ext/scx_simple.bpf.c` 涓殑鍑芥暟鏂囨。鍜岀敤娉曘€?
### 浠诲姟鐢熷懡鍛ㄦ湡


浠ヤ笅浼唬鐮佸ぇ鑷存杩颁簡鐢?sched_ext 璋冨害鍣ㄧ鐞嗙殑浠诲姟鐨勬暣涓敓鍛藉懆鏈燂細


    ops.init_task();            /** A new task is created **/
    ops.enable();               /** Enable BPF scheduling for the task **/

    while (task in SCHED_EXT) {
        if (task can migrate)
            ops.select_cpu();   /** Called on wakeup (optimization) **/

        ops.runnable();         /** Task becomes ready to run **/

        while (task_is_runnable(task)) {
            if (task is not in a DSQ || task->scx.slice == 0) {
                ops.enqueue();  /** Task can be added to a DSQ **/

                /** Task property change (i.e., affinity, nice, etc.)? **/
                if (sched_change(task)) {
                    ops.dequeue(); /** Exiting BPF scheduler custody **/
                    ops.quiescent();

                    /** Property change callback, e.g. ops.set_weight() **/

                    ops.runnable();
                    continue;
                }

                /** Any usable CPU becomes available **/

                ops.dispatch();     /** Task is moved to a local DSQ **/
                ops.dequeue();      /** Exiting BPF scheduler custody **/
            }

            ops.running();      /** Task starts running on its assigned CPU **/

            while (task_is_runnable(task) && task->scx.slice > 0) {
                ops.tick();     /** Called every 1/HZ seconds **/

                if (task->scx.slice == 0)
                    ops.dispatch(); /** task->scx.slice can be refilled **/
            }

            ops.stopping();     /** Task stops running (time slice expires or wait) **/
        }

        ops.quiescent();        /** Task releases its assigned CPU (wait) **/
    }

    ops.disable();              /** Disable BPF scheduling for the task **/
    ops.exit_task();            /** Task is destroyed **/

娉ㄦ剰锛屼笂杩颁吉浠ｇ爜骞舵湭娑电洊鎵€鏈夊彲鑳界殑鐘舵€佽浆鎹㈠拰杈圭晫鎯呭喌锛屼粎涓惧嚑涓緥瀛愶細

- `ops.dispatch()` 鍙兘鐢变簬璇ヤ换鍔′笂鐨勭珵鎬佸睘鎬у彉鏇磋€屾湭鑳藉皢浠诲姟绉诲姩鍒版湰鍦?DSQ锛屽湪杩欑
  鎯呭喌涓?`ops.dispatch()` 灏嗚閲嶈瘯銆?
- 浠诲姟鍙兘浠?`ops.enqueue()` 琚洿鎺ュ垎鍙戝埌鏈湴 DSQ锛屽湪杩欑鎯呭喌涓嬩細璺宠繃 `ops.dispatch()`
  鍜?`ops.dequeue()`锛岀洿鎺ヨ繘鍏?`ops.running()`銆?
- 灞炴€у彉鏇村彲鑳藉彂鐢熷湪浠诲姟鐢熷懡鍛ㄦ湡鐨勫嚑涔庝换浣曟椂鍒伙紝鑰屼笉浠呬粎鏄湪浠诲姟鎺掗槦骞剁瓑寰呭垎鍙戞椂銆?  渚嬪锛屾洿鏀规鍦ㄨ繍琛岀殑浠诲姟鐨勫睘鎬у皢瀵艰嚧鍥炶皟搴忓垪 `ops.stopping()` -> `ops.quiescent()`
  ->锛堝睘鎬у彉鏇村洖璋冿級-> `ops.runnable()` -> `ops.running()`銆?
- 涓€涓?sched_ext 浠诲姟鍙兘琚潵鑷洿楂樹紭鍏堢骇璋冨害绫荤殑浠诲姟鎶㈠崰锛屽湪杩欑鎯呭喌涓嬶紝鍗充娇瀹冩槸
  鍙繍琛岀殑骞朵笖鍏锋湁闈為浂鏃堕棿鐗囷紝瀹冧篃浼氶€€鍑?tick-dispatch 寰幆銆?
鏈夊叧鍒氬敜閱掔殑浠诲姟濡備綍涓?CPU 鐨勬洿璇︾粏鎻忚堪锛岃鍙傝鈥滆皟搴﹀懆鏈熲€濅竴鑺傘€?
## 鍙傝€冧綅缃?

- `include/linux/sched/ext.h` 瀹氫箟浜嗘牳蹇冩暟鎹粨鏋勩€乷ps 琛ㄥ拰甯搁噺銆?
- `kernel/sched/ext.c` 鍖呭惈 sched_ext 鏍稿績瀹炵幇鍜岃緟鍔╁嚱鏁般€備互 `scx_bpf_` 涓哄墠缂€鐨?  鍑芥暟鍙互浠?BPF 璋冨害鍣ㄨ皟鐢ㄣ€?
- `kernel/sched/ext_idle.c` 鍖呭惈鍐呯疆鐨勭┖闂?CPU 閫夋嫨绛栫暐銆?
- `tools/sched_ext/` 鎵樼绀轰緥 BPF 璋冨害鍣ㄥ疄鐜般€?
  - `scx_simple[.bpf].c`锛氫娇鐢ㄨ嚜瀹氫箟 DSQ 鐨勬渶灏忓叏灞€ FIFO 璋冨害鍣ㄧず渚嬨€?
  - `scx_qmap[.bpf].c`锛氫竴涓绾?FIFO 璋冨害鍣紝浣跨敤 `BPF_MAP_TYPE_QUEUE` 瀹炵幇浜旂骇
    浼樺厛绾с€?
  - `scx_central[.bpf].c`锛氫竴涓腑蹇?FIFO 璋冨害鍣紝鎵€鏈夎皟搴﹀喅绛栭兘鍦ㄤ竴涓?CPU 涓婂仛鍑猴紝
    婕旂ず浜?`LOCAL_ON` 鍒嗗彂銆佹棤婊寸瓟鎿嶄綔浠ュ強 kthread 鎶㈠崰銆?
  - `scx_cpu0[.bpf].c`锛氫竴涓皢鎵€鏈変换鍔℃帓闃熷埌鍏变韩 DSQ 骞朵粎鍦?CPU0 涓婁互 FIFO 椤哄簭鍒嗗彂鐨?    璋冨害鍣ㄣ€傚娴嬭瘯 bypass 琛屼负寰堟湁鐢ㄣ€?
  - `scx_flatcg[.bpf].c`锛氫竴涓墎骞冲寲 cgroup 灞傜骇璋冨害鍣紝閫氳繃灏嗘瘡涓?cgroup 鐨勪唤棰濆湪
    姣忎竴绾у鍚堜负鍗曚竴鐨勬墎骞宠皟搴﹀眰锛屽疄鐜板熀浜庡眰绾ф潈閲嶇殑 cgroup CPU 鎺у埗銆?
  - `scx_pair[.bpf].c`锛氫竴涓牳蹇冭皟搴︾ず渚嬶紝鎬绘槸璁╁厔寮?CPU 瀵规墽琛屾潵鑷悓涓€ CPU cgroup
    鐨勪换鍔°€?
  - `scx_sdt[.bpf].c`锛歚scx_simple` 鐨勪竴涓彉浣擄紝婕旂ず浜嗙敤浜庢瘡浠诲姟鏁版嵁鐨?BPF arena
    鍐呭瓨绠＄悊銆?
  - `scx_userland[.bpf].c`锛氫竴涓渶灏忚皟搴﹀櫒锛屾紨绀虹敤鎴风┖闂磋皟搴︺€傚叿鏈?CPU 浜插拰鎬х殑浠诲姟
    浠?FIFO 椤哄簭鐩存帴鍒嗗彂锛涙墍鏈夊叾瀹冧换鍔＄敱涓€涓畝鍗曠殑 vruntime 璋冨害鍣ㄥ湪鐢ㄦ埛绌洪棿涓皟搴︺€?
## 妯″潡鍙傛暟


sched_ext 鍦?`sched_ext.` 鍓嶇紑涓嬫毚闇蹭袱涓ā鍧楀弬鏁帮紝鐢ㄤ簬鎺у埗 bypass 妯″紡琛屼负銆傝繖浜?鏃嬮挳涓昏鐢ㄤ簬璋冭瘯锛涘湪姝ｅ父鎿嶄綔鏈熼棿閫氬父娌℃湁鐞嗙敱鏇存敼瀹冧滑銆傚畠浠彲浠ュ湪杩愯鏃讹紙妯″紡 0600锛?閫氳繃 `/sys/module/sched_ext/parameters/` 璇诲啓銆?
`sched_ext.slice_bypass_us`锛堥粯璁わ細5000 碌s锛?    褰撹皟搴﹀櫒澶勪簬 bypass 妯″紡锛堝嵆鍦?BPF 璋冨害鍣ㄥ姞杞姐€佸嵏杞藉拰閿欒鎭㈠鏈熼棿锛夋椂鍒嗛厤缁欐墍鏈?    浠诲姟鐨勬椂闂寸墖銆傛湁鏁堣寖鍥存槸 100 碌s 鍒?100 ms銆?
`sched_ext.bypass_lb_intv_us`锛堥粯璁わ細500000 碌s锛?    bypass 妯″紡璐熻浇骞宠　鍣ㄥ湪 CPU 涔嬮棿閲嶆柊鍒嗛厤浠诲姟鐨勯棿闅斻€傝涓?0 鍙湪 bypass 妯″紡鏈熼棿
    绂佺敤璐熻浇骞宠　銆傛湁鏁堣寖鍥存槸 0 鍒?10 s銆?
## ABI 涓嶇ǔ瀹氭€?

sched_ext 鎻愪緵缁?BPF 璋冨害鍣ㄧ▼搴忕殑 API 娌℃湁绋冲畾鎬т繚璇併€傝繖鍖呮嫭鍦?`include/linux/sched/ext.h`
涓畾涔夌殑 ops 琛ㄥ洖璋冨拰甯搁噺锛屼互鍙?`kernel/sched/ext.c` 鍜?`kernel/sched/ext_idle.c`
涓畾涔夌殑 `scx_bpf_` kfunc銆?
铏界劧鎴戜滑浼氬湪鍙兘鐨勬儏鍐典笅灏濊瘯鎻愪緵涓€涓浉瀵圭ǔ瀹氱殑 API 闈紝浣嗗畠浠湪涓嶅悓鍐呮牳鐗堟湰涔嬮棿鍙兘
鍦ㄦ病鏈変换浣曡鍛婄殑鎯呭喌涓嬪彂鐢熷彉鍖栥€?