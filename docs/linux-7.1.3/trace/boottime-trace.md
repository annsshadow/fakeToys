
## 鍚姩鏈熻拷韪紙Boot-time tracing锛?


:Author: Masami Hiramatsu <mhiramat@kernel.org>

## 姒傝堪


鍚姩鏈熻拷韪厑璁哥敤鎴峰湪鍚姩闃舵锛堝寘鎷澶囧垵濮嬪寲锛夎繘琛岃拷韪紝骞跺彲浣跨敤 ftrace 鐨勫叏閮ㄥ姛鑳斤紝
鍖呮嫭鎸変簨浠剁殑杩囨护涓庡姩浣溿€佺洿鏂瑰浘銆乲probe 浜嬩欢锛坘probe-events锛変笌鍚堟垚浜嬩欢
锛坰ynthetic-events锛夛紝浠ュ強杩借釜瀹炰緥锛坱race instances锛夈€?
鐢变簬鍐呮牳鍛戒护琛屼笉瓒充互鎺у埗杩欎簺澶嶆潅鐨勫姛鑳斤紝杩欓噷浣跨敤 bootconfig 鏂囦欢鏉ユ弿杩拌拷韪姛鑳界殑
缂栫▼閰嶇疆銆?

## Boot Config 涓殑閫夐」


浠ヤ笅鏄惎鍔ㄦ湡杩借釜鍦?boot config 鏂囦欢 [^1^]_ 涓彲鐢ㄧ殑閫夐」鍒楄〃銆傛墍鏈夐€夐」閮戒綅浜?"ftrace."
鎴?"kernel." 鍓嶇紑涔嬩笅銆備互 "kernel." 鍓嶇紑寮€澶寸殑閫夐」璇峰弬瑙佸唴鏍稿弬鏁?[^2^]_銆?

### Ftrace 鍏ㄥ眬閫夐」


Ftrace 鍏ㄥ眬閫夐」鍦?boot config 涓娇鐢?"kernel." 鍓嶇紑锛岃繖鎰忓懗鐫€杩欎簺閫夐」鏄綔涓哄唴鏍?
浼犵粺鍛戒护琛岀殑涓€閮ㄥ垎浼犲叆鐨勩€?

kernel.tp_printk
   鍚屾椂灏嗚拷韪簨浠舵暟鎹緭鍑哄埌 printk 缂撳啿鍖恒€?

kernel.dump_on_oops [= MODE]
   鍦?Oops 鏃惰浆鍌?ftrace銆傚鏋?MODE = 1 鎴栫渷鐣ワ紝鍒欒浆鍌ㄦ墍鏈?CPU 涓婄殑杩借釜缂撳啿鍖恒€?
   濡傛灉 MODE = 2锛屽垯鍙浆鍌ㄨЕ鍙?Oops 鐨勯偅涓?CPU 涓婄殑缂撳啿鍖恒€?

kernel.traceoff_on_warning
   濡傛灉鍙戠敓 WARN_ON()锛屽垯鍋滄杩借釜銆?

kernel.fgraph_max_depth = MAX_DEPTH
   灏?fgraph tracer 鐨勬渶澶ф繁搴﹁涓?MAX_DEPTH銆?

kernel.fgraph_filters = FILTER[, FILTER2...]
   娣诲姞 fgraph 杩借釜鐨勫嚱鏁拌繃婊ゅ櫒銆?

kernel.fgraph_notraces = FILTER[, FILTER2...]
   娣诲姞 fgraph 闈炶拷韪殑鍑芥暟杩囨护鍣ㄣ€?

### Ftrace 姣忓疄渚嬮€夐」


杩欎簺閫夐」鍙敤浜庢瘡涓疄渚嬶紝鍖呮嫭鍏ㄥ眬 ftrace 鑺傜偣銆?

ftrace.[instance.INSTANCE.]options = OPT1[, OPT2[...]]
   鍚敤缁欏畾鐨?ftrace 閫夐」銆?

ftrace.[instance.INSTANCE.]tracing_on = 0|1
   鍦ㄥ惎鍔ㄦ湡杩借釜寮€濮嬫椂锛屽惎鐢?绂佺敤璇ュ疄渚嬩笂鐨勮拷韪€?
   锛堜綘涔熷彲浠ラ€氳繃 "traceon" 浜嬩欢瑙﹀彂鍔ㄤ綔鏉ュ惎鐢ㄥ畠锛?

ftrace.[instance.INSTANCE.]trace_clock = CLOCK
   灏?ftrace 鐨?trace_clock 璁句负缁欏畾鐨?CLOCK銆?

ftrace.[instance.INSTANCE.]buffer_size = SIZE
   灏?ftrace 缂撳啿鍖哄ぇ灏忛厤缃负 SIZE銆傝 SIZE 鍙互浣跨敤 "KB" 鎴?"MB"銆?

ftrace.[instance.INSTANCE.]alloc_snapshot
   鍒嗛厤蹇収缂撳啿鍖恒€?

ftrace.[instance.INSTANCE.]cpumask = CPUMASK
   灏?CPUMASK 璁句负杩借釜鐨?CPU 鎺╃爜銆?

ftrace.[instance.INSTANCE.]events = EVENT[, EVENT2[...]]
   鍦ㄥ惎鍔ㄦ椂鍚敤缁欏畾鐨勪簨浠躲€侲VENT 涓彲浠ヤ娇鐢ㄩ€氶厤绗︺€?

ftrace.[instance.INSTANCE.]tracer = TRACER
   鍦ㄥ惎鍔ㄦ椂灏嗗綋鍓?tracer 璁句负 TRACER銆傦紙渚嬪 function锛?

ftrace.[instance.INSTANCE.]ftrace.filters
   鎺ュ彈涓€缁勮拷韪嚱鏁拌繃婊よ鍒欍€?

ftrace.[instance.INSTANCE.]ftrace.notraces
   鎺ュ彈涓€缁勯潪杩借釜鍑芥暟杩囨护瑙勫垯銆?

### Ftrace 姣忎簨浠堕€夐」


杩欎簺閫夐」鐢ㄤ簬璁剧疆姣忎簨浠剁殑閫夐」銆?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.enable
   鍚敤 GROUP:EVENT 鐨勮拷韪€?

ftrace.[instance.INSTANCE.]event.GROUP.enable
   鍚敤 GROUP 鍐呯殑鎵€鏈変簨浠惰拷韪€?

ftrace.[instance.INSTANCE.]event.enable
   鍚敤鎵€鏈変簨浠惰拷韪€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.filter = FILTER
   灏?FILTER 瑙勫垯璁剧疆鍒?GROUP:EVENT銆?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.actions = ACTION[, ACTION2[...]]
   灏?ACTION 璁剧疆鍒?GROUP:EVENT銆?

ftrace.[instance.INSTANCE.]event.kprobes.EVENT.probes = PROBE[, PROBE2[...]]
   鍩轰簬 PROBEs 瀹氫箟鏂扮殑 kprobe 浜嬩欢銆傚彲浠ュ湪涓€涓簨浠朵笂瀹氫箟澶氫釜鎺㈤拡锛屼絾杩欎簺鎺㈤拡
   蹇呴』鍏锋湁鐩稿悓绫诲瀷鐨勫弬鏁般€傝閫夐」浠呭缁勫悕涓?"kprobes" 鐨勪簨浠跺彲鐢ㄣ€?

ftrace.[instance.INSTANCE.]event.synthetic.EVENT.fields = FIELD[, FIELD2[...]]
   鐢?FIELDs 瀹氫箟鏂扮殑鍚堟垚浜嬩欢銆傛瘡涓瓧娈靛簲涓?"type varname"銆?

娉ㄦ剰锛宬probe 涓庡悎鎴愪簨浠剁殑瀹氫箟鍙互鍐欏湪瀹炰緥鑺傜偣涔嬩笅锛屼絾瀹冧滑鍦ㄥ叾浠栧疄渚嬩腑涔熸槸鍙鐨勩€?
鍥犳璇锋敞鎰忎簨浠跺悕鍐茬獊鐨勯棶棰樸€?

### Ftrace 鐩存柟鍥鹃€夐」


鐢变簬灏嗙洿鏂瑰浘鍔ㄤ綔浣滀负姣忎簨浠?action 閫夐」鐨勫瓧绗︿覆鏉ュ啓浼氳繃闀匡紝杩欓噷鎻愪緵浜嗕綅浜庢瘡浜嬩欢
'hist' 瀛愰敭涓嬬殑鏍戝舰閫夐」锛岀敤浜庨厤缃洿鏂瑰浘鍔ㄤ綔銆傚叧浜庢瘡涓弬鏁扮殑璇︾粏淇℃伅锛岃闃呰浜嬩欢
鐩存柟鍥炬枃妗ｏ紙Documentation/trace/histogram.rst锛夈€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]keys = KEY1[, KEY2[...]]
  璁剧疆鐩存柟鍥鹃敭鍙傛暟銆傦紙蹇呭～锛?
  'N' 鏄敤浜庡涓洿鏂瑰浘鐨勬暟鍊煎瓧绗︿覆銆傚鏋滆浜嬩欢涓婂彧鏈変竴涓洿鏂瑰浘锛屽彲浠ョ渷鐣ュ畠銆?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]values = VAL1[, VAL2[...]]
  璁剧疆鐩存柟鍥惧€煎弬鏁般€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]sort = SORT1[, SORT2[...]]
  璁剧疆鐩存柟鍥炬帓搴忓弬鏁伴€夐」銆?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]size = NR_ENTRIES
  璁剧疆鐩存柟鍥惧ぇ灏忥紙鏉＄洰鏁帮級銆?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]name = NAME
  璁剧疆鐩存柟鍥惧悕绉般€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]var.VARIABLE = EXPR
  閫氳繃 EXPR 琛ㄨ揪寮忓畾涔変竴涓柊鐨?VARIABLE銆?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]<pause|continue|clear>
  璁剧疆鐩存柟鍥炬帶鍒跺弬鏁般€傚彲浠ヨ缃叾涓殑涓€涓€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]onmatch.[M.]event = GROUP.EVENT
  璁剧疆鐩存柟鍥?'onmatch' 澶勭悊鍣ㄥ尮閰嶇殑浜嬩欢鍙傛暟銆?
  'M' 鏄敤浜庡涓?'onmatch' 澶勭悊鍣ㄧ殑鏁板€煎瓧绗︿覆銆傚鏋滄鐩存柟鍥句笂鍙湁涓€涓?'onmatch'
  澶勭悊鍣紝鍙互鐪佺暐瀹冦€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]onmatch.[M.]trace = EVENT[, ARG1[...]]
  涓?'onmatch' 璁剧疆鐩存柟鍥?'trace' 鍔ㄤ綔銆?
  EVENT 蹇呴』鏄悎鎴愪簨浠跺悕锛岃€?ARG1... 鏄浜嬩欢鐨勫弬鏁般€傚鏋滆缃簡 'onmatch.event'
  閫夐」鍒欎负蹇呭～銆?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]onmax.[M.]var = VAR
  璁剧疆鐩存柟鍥?'onmax' 澶勭悊鍣ㄥ彉閲忓弬鏁般€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]onchange.[M.]var = VAR
  璁剧疆鐩存柟鍥?'onchange' 澶勭悊鍣ㄥ彉閲忓弬鏁般€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]<onmax|onchange>.[M.]save = ARG1[, ARG2[...]]
  涓?'onmax' 鎴?'onchange' 澶勭悊鍣ㄨ缃洿鏂瑰浘 'save' 鍔ㄤ綔鍙傛暟銆?
  濡傛灉璁剧疆浜?'onmax.var' 鎴?'onchange.var' 閫夐」锛屽垯姝ら€夐」鎴栦笅闈㈢殑 'snapshot' 閫夐」涓哄繀濉€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.[N.]<onmax|onchange>.[M.]snapshot
  涓?'onmax' 鎴?'onchange' 澶勭悊鍣ㄨ缃洿鏂瑰浘 'snapshot' 鍔ㄤ綔銆?
  濡傛灉璁剧疆浜?'onmax.var' 鎴?'onchange.var' 閫夐」锛屽垯姝ら€夐」鎴栦笂闈㈢殑 'save' 閫夐」涓哄繀濉€?

ftrace.[instance.INSTANCE.]event.GROUP.EVENT.hist.filter = FILTER_EXPR
  璁剧疆鐩存柟鍥捐繃婊よ〃杈惧紡銆傚湪 FILTER_EXPR 涓笉闇€瑕佸啓 'if'銆?

娉ㄦ剰锛屽鏋滄瘡浜嬩欢鐨?'actions' 閫夐」鍖呭惈鐩存柟鍥惧姩浣滐紝鍒欒 'hist' 閫夐」鍙兘涓庡叾鍐茬獊銆?

## 浣曟椂鍚姩


鎵€鏈変互 `ftrace` 寮€澶寸殑鍚姩鏈熻拷韪€夐」閮戒細鍦?core_initcall 缁撴潫鏃跺惎鐢ㄣ€傝繖鎰忓懗鐫€浣犲彲浠?
杩借釜浠?postcore_initcall 寮€濮嬬殑浜嬩欢銆傚ぇ澶氭暟瀛愮郴缁熷拰涓庢灦鏋勭浉鍏崇殑椹卞姩浼氬湪閭ｄ箣鍚庡垵濮嬪寲
锛坅rch_initcall 鎴?subsys_initcall锛夈€傚洜姝わ紝浣犲彲浠ョ敤鍚姩鏈熻拷韪潵杩借釜瀹冧滑銆?
濡傛灉浣犲笇鏈涘湪 core_initcall 涔嬪墠杩借釜浜嬩欢锛屽彲浠ヤ娇鐢ㄤ互 `kernel` 寮€澶寸殑閫夐」銆傚叾涓儴鍒?
閫夐」浼氭瘮 initcall 澶勭悊鏇存棭鍚敤锛堜緥濡?`kernel.ftrace=function` 鍜?`kernel.trace_event`
浼氬湪 initcall 涔嬪墠鍚姩锛夈€?

## 绀轰緥


渚嬪锛岃涓烘瘡涓簨浠舵坊鍔犺繃婊ゅ櫒鍜屽姩浣溿€佸畾涔?kprobe 浜嬩欢浠ュ強甯︾洿鏂瑰浘鐨勫悎鎴愪簨浠讹紝鍙互缂栧啓
濡備笅 boot config
```

  ftrace.event {
        task.task_newtask {
                filter = "pid < 128"
                enable
        }
        kprobes.vfs_read {
                probes = "vfs_read $arg1 $arg2"
                filter = "common_pid < 200"
                enable
        }
        synthetic.initcall_latency {
                fields = "unsigned long func", "u64 lat"
                hist {
                        keys = func.sym, lat
                        values = lat
                        sort = lat
                }
        }
        initcall.initcall_start.hist {
                keys = func
                var.ts0 = common_timestamp.usecs
        }
        initcall.initcall_finish.hist {
                keys = func
                var.lat = common_timestamp.usecs - $ts0
                onmatch {
                        event = initcall.initcall_start
                        trace = initcall_latency, func, $lat
                }
        }
  }

```
姝ゅ锛屽惎鍔ㄦ湡杩借釜鏀寔 "instance" 鑺傜偣锛屽厑璁告垜浠悓鏃朵负涓嶅悓鐩殑杩愯澶氫釜 tracer銆備緥濡傦紝
涓€涓?tracer 鐢ㄤ簬杩借釜浠?"user\_" 寮€澶寸殑鍑芥暟锛屽彟涓€涓拷韪?
```
  ftrace.instance {
        foo {
                tracer = "function"
                ftrace.filters = "user_*"
        }
        bar {
                tracer = "function"
                ftrace.filters = "kernel_*"
        }
  }

```
瀹炰緥鑺傜偣涔熸帴鍙椾簨浠惰妭鐐癸紝鍥犳姣忎釜瀹炰緥鍙互鑷畾涔夊叾浜嬩欢杩借釜銆?

鍊熷姪瑙﹀彂鍔ㄤ綔涓?kprobe锛屼綘鍙互鍦ㄦ煇涓嚱鏁拌璋冪敤鏃惰拷韪叾鍑芥暟鍥撅紙function-graph锛夈€備緥濡傦紝
杩欏皢杩借釜濡備笅浠ｇ爜涓殑鍏ㄩ儴鍑芥暟璋冪敤
```
  ftrace {
        tracing_on = 0
        tracer = function_graph
        event.kprobes {
                start_event {
                        probes = "pci_proc_init"
                        actions = "traceon"
                }
                end_event {
                        probes = "pci_proc_init%return"
                        actions = "traceoff"
                }
        }
  }


```
姝ゅ惎鍔ㄦ湡杩借釜涔熼€氳繃 boot config 鏀寔 ftrace 鍐呮牳鍙傛暟銆?
```
  trace_options=sym-addr trace_event=initcall:* tp_printk trace_buf_size=1M ftrace=function ftrace_filter="vfs*"

```
```
  kernel {
        trace_options = sym-addr
        trace_event = "initcall:*"
        tp_printk
        trace_buf_size = 1M
        ftrace = function
        ftrace_filter = "vfs*"
  }

```
娉ㄦ剰锛屽弬鏁颁互 "kernel" 鍓嶇紑鑰岄潪 "ftrace" 鍓嶇紑寮€澶淬€?
