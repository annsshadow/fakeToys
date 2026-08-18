## DTL锛圖ispatch Trace Log锛岃皟搴﹁窡韪棩蹇楋級


Athira Rajeev锛?025 骞?4 鏈?19 鏃?
    :depth: 3


## 鍩烘湰姒傝堪锛圔asic overview锛?

pseries 鐨勫叡浜鐞嗗櫒閫昏緫鍒嗗尯锛圫PLPAR锛夋満鍣ㄥ彲浠ヤ娇鐢ㄦ潵鑷皟搴﹁窡韪棩蹇楋紙DTL锛夌紦鍐插尯鐨勬暟鎹紝浠?hypervisor 妫€绱㈣皟搴︼紙dispatch锛夊拰鎶㈠崰锛坧reempt锛変簨浠剁殑鏃ュ織銆傚埄鐢ㄨ繖浜涗俊鎭紝鐢ㄦ埛鍙互妫€绱㈡瘡娆¤皟搴﹀拰鎶㈠崰鍙戠敓鐨勬椂闂翠笌鍘熷洜銆?vpa-dtl PMU 閫氳繃 perf 鏆撮湶铏氭嫙澶勭悊鍣ㄥ尯锛圴PA锛夌殑 DTL 璁℃暟鍣ㄣ€?
## 浣跨敤鐨勫熀纭€璁炬柦锛圛nfrastructure used锛?

VPA DTL PMU 璁℃暟鍣ㄥ湪婧㈠嚭鏃朵笉浼氫腑鏂紝涔熶笉浼氫骇鐢熶换浣?PMI 涓柇銆傚洜姝わ紝浣跨敤 hrtimer 鏉ヨ疆璇?DTL 鏁版嵁銆傝瀹氭椂鍣ㄩ棿闅斿彲鐢辩敤鎴烽€氳繃 sample_period 瀛楁浠ョ撼绉掍负鍗曚綅鎻愪緵銆?vpa dtl pmu 涓烘瘡涓?vpa-dtl pmu 绾跨▼娣诲姞涓€涓?hrtimer銆侱TL锛堣皟搴﹁窡韪棩蹇楋級鍖呭惈鍏充簬璋冨害/鎶㈠崰銆佸叆闃熸椂闂寸瓑淇℃伅銆?鎴戜滑鐩存帴灏?DTL 缂撳啿鍖烘暟鎹綔涓鸿緟鍔╃紦鍐插尯锛坅uxiliary buffer锛夌殑涓€閮ㄥ垎澶嶅埗锛岀◢鍚庡啀澶勭悊銆傝繖灏嗛伩鍏嶅湪鍐呮牳绌洪棿涓垱寤洪噰鏍锋墍鑺辫垂鐨勬椂闂淬€?鏀堕泦璋冨害璺熻釜鏃ュ織锛圖TL锛夋潯鐩殑 PMU 椹卞姩鍒╃敤浜?perf 鍩虹璁炬柦涓殑 AUX 鏀寔銆傚湪宸ュ叿渚э紝杩欎簺鏁版嵁浠?PERF_RECORD_AUXTRACE 璁板綍鐨勫舰寮忔彁渚涖€?
涓轰簡灏嗘瘡涓?DTL 鏉＄洰涓庤法 CPU 鐨勫叾浠栦簨浠跺叧鑱旇捣鏉ワ紝涓烘瘡涓?CPU 鍒涘缓涓€涓?auxtrace_queue銆傛瘡涓?auxtrace 闃熷垪閮芥湁涓€涓?auxtrace 缂撳啿鍖烘暟缁?鍒楄〃銆?鎵€鏈?auxtrace 闃熷垪閮界淮鎶ゅ湪 auxtrace 鍫嗭紙heap锛変腑銆傞槦鍒楁牴鎹椂闂存埑鎺掑簭銆傚湪澶勭悊涓嶅悓鐨?PERF_RECORD_XX 璁板綍鏃讹紝灏?perf 璁板綍鐨勬椂闂存埑涓?auxtrace 鍫嗕腑鏍堥《鍏冪礌鐨勬椂闂存埑杩涜姣旇緝锛屼粠鑰屽彲浠ュ皢 DTL 浜嬩欢涓庡叾浠栦簨浠跺叧鑱旇捣鏉ャ€?濡傛灉鍫嗕腑鍏冪礌鐨勬椂闂存埑浣庝簬 perf 璁板綍涓潯鐩殑鏃堕棿鎴筹紝鍒欏鐞?auxtrace 闃熷垪锛屼互渚?DTL 浜嬩欢鍙互涓庡叾浠栦簨浠跺叧鑱斻€?鏈夋椂涓€涓紦鍐插尯鍙兘鍙閮ㄥ垎澶勭悊銆傚鏋滃彟涓€涓簨浠跺彂鐢熺殑鏃堕棿鎴冲ぇ浜庨槦鍒椾腑褰撳墠宸插鐞嗙殑鍏冪礌锛屽畠灏嗚浆鍒颁笅涓€涓?perf 璁板綍銆傚洜姝よ璁板綍缂撳啿鍖虹殑浣嶇疆锛屼互渚夸笅娆＄户缁鐞嗐€傜敤 auxtrace 缂撳啿鍖轰腑鏈€鍚庡鐞嗙殑鏉＄洰鐨勬椂闂存埑鏇存柊 auxtrace 鍫嗙殑鏃堕棿鎴炽€?
璇ュ熀纭€璁炬柦纭繚璋冨害璺熻釜鏃ュ織鏉＄洰鑳藉涓庡叾浠栦簨浠讹紙濡?sched锛夊叧鑱斿苟涓€璧峰憟鐜般€?
## vpa-dtl PMU 浣跨敤绀轰緥锛坴pa-dtl PMU example usage锛?

  # ls /sys/devices/vpa_dtl/
  events  format  perf_event_mux_interval_ms  power  subsystem  type  uevent


瑕佷娇鐢?perf record 鎹曡幏 DTL 鏁版嵁锛?
  # ./perf record -a -e sched:\*,vpa_dtl/dtl_all/ -c 1000000000 sleep 1

缁撴灉鍙互浣跨敤 perf record 瑙ｉ噴銆備笅闈㈡槸 perf report -D 鐨勭墖娈?

  # ./perf report -D

瀛樺湪涓嶅悓鐨?PERF_RECORD_XX 璁板綍銆傚叾涓笌 auxtrace 缂撳啿鍖哄搴旂殑璁板綍鍖呮嫭锛?
1. PERF_RECORD_AUX
   琛ㄧず AUX 鍖哄煙涓湁鏂版暟鎹彲鐢?
2. PERF_RECORD_AUXTRACE_INFO
   鎻忚堪缂撳啿鍖轰腑 auxtrace 鏁版嵁鐨勫亸绉诲拰澶у皬

3. PERF_RECORD_AUXTRACE
   杩欐槸瀹氫箟 auxtrace 鏁版嵁鐨勮褰曪紝鍦?vpa-dtl pmu 鐨勬儏鍐典笅锛岃繖閲屽氨鏄皟搴﹁窡韪棩蹇楁暟鎹€?
涓嬮潰鏄敱 perf report -D 鏄剧ず鐨?PERF_RECORD_AUXTRACE dump 鐗囨


0 0 0x39b10 [0x30]: PERF_RECORD_AUXTRACE size: 0x690  offset: 0  ref: 0  idx: 0  tid: -1  cpu: 0
.
. ... VPA DTL PMU data: size 1680 bytes, entries is 35
.  00000000: boot_tb: 21349649546353231, tb_freq: 512000000
.  00000030: dispatch_reason:decrementer interrupt, preempt_reason:H_CEDE, enqueue_to_dispatch_time:7064, ready_to_enqueue_time:187, waiting_to_ready_time:6611773
.  00000060: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:146, ready_to_enqueue_time:0, waiting_to_ready_time:15359437
.  00000090: dispatch_reason:decrementer interrupt, preempt_reason:H_CEDE, enqueue_to_dispatch_time:4868, ready_to_enqueue_time:232, waiting_to_ready_time:5100709
.  000000c0: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:179, ready_to_enqueue_time:0, waiting_to_ready_time:30714243
.  000000f0: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:197, ready_to_enqueue_time:0, waiting_to_ready_time:15350648
.  00000120: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:213, ready_to_enqueue_time:0, waiting_to_ready_time:15353446
.  00000150: dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:212, ready_to_enqueue_time:0, waiting_to_ready_time:15355126
.  00000180: dispatch_reason:decrementer interrupt, preempt_reason:H_CEDE, enqueue_to_dispatch_time:6368, ready_to_enqueue_time:164, waiting_to_ready_time:5104665

浠ヤ笂鏄涓嬫牸寮忕殑 dtl 鏉＄洰鐨勮〃绀猴細

struct dtl_entry {
        u8      dispatch_reason;
        u8      preempt_reason;
        u16     processor_id;
        u32     enqueue_to_dispatch_time;
        u32     ready_to_enqueue_time;
        u32     waiting_to_ready_time;
        u64     timebase;
        u64     fault_addr;
        u64     srr0;
        u64     srr1;

};

鍓嶄袱涓瓧娈佃〃绀鸿皟搴﹀師鍥犲拰鎶㈠崰鍘熷洜銆侾ERF_RECORD_AUXTRACE 璁板綍鐨勫悗鏈熷鐞嗗皢杞崲涓哄鐢ㄦ埛鏈夋剰涔夌殑鏁版嵁銆?
## 浣跨敤 perf report 鍙鍖栬皟搴﹁窡韪棩蹇楁潯鐩紙Visualize the dispatch trace log entries with perf report锛?

  # ./perf record -a -e sched:\*,vpa_dtl/dtl_all/ -c 1000000000 sleep 1
  [ perf record: Woken up 1 times to write data ]
  [ perf record: Captured and wrote 0.300 MB perf.data ]

  # ./perf report
  # Samples: 321  of event 'vpa-dtl'
  # Event count (approx.): 321
  #
  # Children      Self  Command  Shared Object      Symbol
  # ........  ........  .......  .................  ..............................
  #
     100.00%   100.00%  swapper  [kernel.kallsyms]  [k] plpar_hcall_norets_notrace

## 浣跨敤 perf script 鍙鍖栬皟搴﹁窡韪棩蹇楁潯鐩紙Visualize the dispatch trace log entries with perf script锛?

   # ./perf script
     migration/9      67 [^009^] 105373.359903:                     sched:sched_waking: comm=perf pid=13418 prio=120 target_cpu=009
     migration/9      67 [^009^] 105373.359904:               sched:sched_migrate_task: comm=perf pid=13418 prio=120 orig_cpu=9 dest_cpu=10
     migration/9      67 [^009^] 105373.359907:               sched:sched_stat_runtime: comm=migration/9 pid=67 runtime=4050 [ns]
     migration/9      67 [^009^] 105373.359908:                     sched:sched_switch: prev_comm=migration/9 prev_pid=67 prev_prio=0 prev_state=S ==> next_comm=swapper/9 next_pid=0 next_prio=120
            :256     256 [^016^] 105373.359913:                                vpa-dtl: timebase: 21403600706628832 dispatch_reason:decrementer interrupt, preempt_reason:H_CEDE, enqueue_to_dispatch_time:4854,                        ready_to_enqueue_time:139, waiting_to_ready_time:511842115 c0000000000fcd28 plpar_hcall_norets_notrace+0x18 ([kernel.kallsyms])
            :256     256 [^017^] 105373.360012:                                vpa-dtl: timebase: 21403600706679454 dispatch_reason:priv doorbell, preempt_reason:H_CEDE, enqueue_to_dispatch_time:236,                         ready_to_enqueue_time:0, waiting_to_ready_time:133864583 c0000000000fcd28 plpar_hcall_norets_notrace+0x18 ([kernel.kallsyms])
            perf   13418 [^010^] 105373.360048:               sched:sched_stat_runtime: comm=perf pid=13418 runtime=139748 [ns]
            perf   13418 [^010^] 105373.360052:                     sched:sched_waking: comm=migration/10 pid=72 prio=0 target_cpu=010
