## 鍩轰簬 Kprobe 鐨勪簨浠惰窡韪紙Kprobe-based Event Tracing锛?
:Author: Masami Hiramatsu

### 姒傝堪锛圤verview锛?
杩欎簺浜嬩欢涓庡熀浜?tracepoint 鐨勪簨浠剁被浼笺€備笌 tracepoint 涓嶅悓锛屽畠鍩轰簬 kprobe锛坘probe 鍜?kretprobe锛夈€?鍥犳瀹冨彲浠ユ帰娴?kprobe 鑳藉鎺㈡祴鐨勪换浣曞湴鏂癸紙杩欐剰鍛崇潃锛岄櫎浜嗗甫鏈?`__kprobes`/`nokprobe_inline` 娉ㄨВ浠ュ強
鏍囪涓?NOKPROBE_SYMBOL 鐨勫嚱鏁颁箣澶栫殑鎵€鏈夊嚱鏁帮級銆備笌鍩轰簬 tracepoint 鐨勪簨浠朵笉鍚岋紝瀹冨彲浠ュ姩鎬佸湴銆佸湪杩愯鏃?娣诲姞鍜岀Щ闄ゃ€?
瑕佸惎鐢ㄦ鍔熻兘锛岃浠ュ唴鏍?CONFIG_KPROBE_EVENTS=y 鏋勫缓浣犵殑鍐呮牳銆?
涓庝簨浠惰窡韪櫒锛坋vent tracer锛夌被浼硷紝瀹冧笉闇€瑕侀€氳繃 current_tracer 婵€娲汇€傚彇鑰屼唬涔嬬殑鏄紝閫氳繃
/sys/kernel/tracing/kprobe_events 娣诲姞鎺㈡祴鐐癸紝骞堕€氳繃
/sys/kernel/tracing/events/kprobes/<EVENT>/enable 鍚敤瀹冦€?
浣犱篃鍙互浣跨敤 /sys/kernel/tracing/dynamic_events 浠ｆ浛 kprobe_events銆傝鎺ュ彛涔熷皢涓哄叾瀹冨姩鎬佷簨浠舵彁渚?缁熶竴鐨勮闂柟寮忋€?
### kprobe_events 璇硶锛圫ynopsis of kprobe_events锛?
```

  p[:[GRP/][EVENT]] [MOD:]SYM[+offs]|MEMADDR [FETCHARGS]	: Set a probe
  r[MAXACTIVE][:[GRP/][EVENT]] [MOD:]SYM[+0] [FETCHARGS]	: Set a return probe
  p[:[GRP/][EVENT]] [MOD:]SYM[+0]%return [FETCHARGS]	: Set a return probe
  -:[GRP/][EVENT]						: Clear a probe

 GRP		: Group name. If omitted, use "kprobes" for it.
 EVENT		: Event name. If omitted, the event name is generated
		  based on SYM+offs or MEMADDR.
 MOD		: Module name which has given SYM.
 SYM[+offs]	: Symbol+offset where the probe is inserted.
 SYM%return	: Return address of the symbol
 MEMADDR	: Address where the probe is inserted.
 MAXACTIVE	: Maximum number of instances of the specified function that
		  can be probed simultaneously, or 0 for the default value
		  as defined in Documentation/trace/kprobes.rst section 1.3.1.

 FETCHARGS	: Arguments. Each probe can have up to 128 args.
  %REG		: Fetch register REG
  @ADDR		: Fetch memory at ADDR (ADDR should be in kernel)
  @SYM[+|-offs]	: Fetch memory at SYM +|- offs (SYM should be a data symbol)
  $stackN	: Fetch Nth entry of stack (N >= 0)
  $stack	: Fetch stack address.
  $argN		: Fetch the Nth function argument. (N >= 1) (\*1)
  $retval	: Fetch return value.(\*2)
  $comm		: Fetch current task comm.
  +|-[u]OFFS(FETCHARG) : Fetch memory at FETCHARG +|- OFFS address.(\*3)(\*4)
  \IMM		: Store an immediate value to the argument.
  NAME=FETCHARG : Set NAME as the argument name of FETCHARG.
  FETCHARG:TYPE : Set TYPE as the type of FETCHARG. Currently, basic types
		  (u8/u16/u32/u64/s8/s16/s32/s64), hexadecimal types
		  (x8/x16/x32/x64), VFS layer common type(%pd/%pD), "char",
                  "string", "ustring", "symbol", "symstr" and bitfield are
                  supported.

  (\*1) only for the probe on function entry (offs == 0). Note, this argument access
        is best effort, because depending on the argument type, it may be passed on
        the stack. But this only support the arguments via registers.
  (\*2) only for return probe. Note that this is also best effort. Depending on the
        return value type, it might be passed via a pair of registers. But this only
        accesses one register.
  (\*3) this is useful for fetching a field of data structures.
  (\*4) "u" means user-space dereference. See :ref:`user_mem_access`.

```
### kretprobe 澶勭殑鍑芥暟鍙傛暟锛團unction arguments at kretprobe锛?
鍑芥暟鍙傛暟鍙互鍦?kretprobe 澶勪娇鐢?$arg<N> fetch 鍙傛暟鏉ヨ闂€傝繖瀵逛簬涓€娆℃€ц褰曞嚱鏁板弬鏁板拰杩斿洖鍊硷紝骞惰窡韪?缁撴瀯浣撳瓧娈电殑宸紓锛堢敤浜庤皟璇曟煇涓嚱鏁版槸鍚︽纭洿鏂颁簡缁欏畾鐨勬暟鎹粨鏋勶級寰堟湁鐢ㄣ€傚叧浜庡叾宸ヤ綔鍘熺悊锛岃鍙傝
fprobe 浜嬩欢涓殑绀轰緥<fprobetrace_exit_args_sample>銆?
### 绫诲瀷锛圱ypes锛?
fetcharg 鏀寔澶氱绫诲瀷銆侹probe 璺熻釜鍣ㄤ細鎸夌粰瀹氱被鍨嬭闂唴瀛樸€傚墠缂€ 's' 鍜?'u' 鍒嗗埆琛ㄧず杩欎簺绫诲瀷鏄湁绗﹀彿
鍜屾棤绗﹀彿鐨勩€?x' 鍓嶇紑琛ㄧず瀹冩槸鏃犵鍙风殑銆傝璺熻釜鐨勫弬鏁颁互鍗佽繘鍒讹紙's' 鍜?'u'锛夋垨鍗佸叚杩涘埗锛?x'锛夋樉绀恒€備笉
杩涜绫诲瀷杞崲鏃讹紝鏍规嵁鏋舵瀯浣跨敤 'x32' 鎴?'x64'锛堜緥濡?x86-32 浣跨敤 x32锛寈86-64 浣跨敤 x64锛夈€?
杩欎簺鍊肩被鍨嬪彲浠ユ槸鏁扮粍銆傝璁板綍鏁扮粍鏁版嵁锛屼綘鍙互缁欏熀绫诲瀷娣诲姞 '[N]'锛堝叾涓?N 鏄竴涓皬浜?64 鐨勫浐瀹氭暟瀛楋級銆?渚嬪 'x16[^4^]' 琛ㄧず鏈?4 涓厓绱犵殑 x16锛? 瀛楄妭鍗佸叚杩涘埗锛夋暟缁勩€傛敞鎰忥紝鏁扮粍鍙互搴旂敤浜庡唴瀛樼被鍨嬬殑
fetcharg锛屼絾涓嶈兘搴旂敤浜庡瘎瀛樺櫒/鏍堟潯鐩瓑锛堜緥濡?'$stack1:x8[^8^]' 鏄敊璇殑锛屼絾 '+8($stack):x8[^8^]'
鏄纭殑锛夈€?
Char 绫诲瀷鍙敤浜庢樉绀鸿璺熻釜鍙傛暟鐨勫瓧绗﹀€笺€?
String 绫诲瀷鏄竴绉嶇壒娈婄被鍨嬶紝瀹冧粠鍐呮牳绌洪棿鑾峰彇涓€涓?浠?null 缁撳熬"鐨勫瓧绗︿覆銆傝繖鎰忓懗鐫€濡傛灉璇ュ瓧绗︿覆鎵€鍦ㄧ殑
瀹瑰櫒宸茶鎹㈠嚭锛屽畠灏嗗け璐ュ苟瀛樺偍 NULL銆?ustring" 绫诲瀷鏄?string 闈㈠悜鐢ㄦ埛绌洪棿鐨勬浛浠ｇ被鍨嬨€傛洿澶氫俊鎭弬瑙?user_mem_access銆?
瀛楃涓叉暟缁勭被鍨嬩笌鍏跺畠绫诲瀷鐣ユ湁涓嶅悓銆傚浜庡叾瀹冨熀绫诲瀷锛?base-type>[^1^] 绛変簬 <base-type>锛堜緥濡?+0(%di):x32[^1^] 涓?+0(%di):x32 鐩稿悓锛夈€備絾 string[^1^] 涓嶇瓑浜?string銆俿tring 绫诲瀷鏈韩琛ㄧず"瀛楃鏁扮粍"锛?鑰屽瓧绗︿覆鏁扮粍绫诲瀷琛ㄧず"char * 鏁扮粍"銆傚洜姝わ紝渚嬪 +0(%di):string[^1^] 绛変簬 +0(+0(%di)):string銆?Bitfield 鏄彟涓€绉嶇壒娈婄被鍨嬶紝瀹冩帴鍙?3 涓弬鏁帮細浣嶅銆佷綅鍋忕Щ鍜屽鍣ㄥぇ灏忥細
```

 b<bit-width>@<bit-offset>/<container-size>

```
Symbol 绫诲瀷锛?symbol'锛夋槸 u32 鎴?u64 绫诲瀷锛堝彇鍐充簬 BITS_PER_LONG锛夌殑鍒悕锛屼互 "symbol+offset" 鏍峰紡
鏄剧ず缁欏畾鐨勬寚閽堛€傚彟涓€鏂归潰锛宻ymbol-string 绫诲瀷锛?symstr'锛夋妸缁欏畾鐨勫湴鍧€杞崲涓?"symbol+offset/symbolsize"
鏍峰紡锛屽苟灏嗗叾浣滀负浠?null 缁撳熬鐨勫瓧绗︿覆瀛樺偍銆備娇鐢?'symstr' 绫诲瀷锛屼綘鍙互鐢ㄧ鍙风殑閫氶厤绗︽ā寮忚繃婊や簨浠讹紝鑰?鏃犻渶鑷繁瑙ｆ瀽绗﹀彿鍚嶃€傚浜?$comm锛岄粯璁ょ被鍨嬫槸 "string"锛涗换浣曞叾瀹冪被鍨嬮兘鏄棤鏁堢殑銆?
VFS 灞傞€氱敤绫诲瀷锛?pd/%pD锛夋槸涓€绉嶇壒娈婄被鍨嬶紝瀹冧粠 struct dentry 鐨勫湴鍧€鎴?struct file 鐨勫湴鍧€鑾峰彇 dentry
鎴栨枃浠跺悕銆?
### 鐢ㄦ埛鍐呭瓨璁块棶锛圲ser Memory Access锛?
Kprobe 浜嬩欢鏀寔鐢ㄦ埛绌洪棿鍐呭瓨璁块棶銆備负姝わ紝浣犲彲浠ヤ娇鐢ㄧ敤鎴风┖闂磋В寮曠敤璇硶鎴?'ustring' 绫诲瀷銆?
鐢ㄦ埛绌洪棿瑙ｅ紩鐢ㄨ娉曞厑璁镐綘璁块棶鐢ㄦ埛绌洪棿涓煇涓暟鎹粨鏋勭殑瀛楁銆傝繖鏄€氳繃缁欒В寮曠敤璇硶娣诲姞 "u" 鍓嶇紑鏉ュ疄鐜扮殑銆?渚嬪锛?u4(%si) 琛ㄧず瀹冨皢浠庡瘎瀛樺櫒 %si 涓湴鍧€鍋忕Щ 4 鐨勪綅缃鍙栧唴瀛橈紝骞朵笖璇ュ唴瀛橀鏈熶綅浜庣敤鎴风┖闂淬€備綘涔熷彲浠?鎶婂畠鐢ㄤ簬瀛楃涓诧紝渚嬪 +u0(%si):string 灏嗕粠瀵勫瓨鍣?%si 涓鏈熶綅浜庣敤鎴风┖闂寸殑鍦板潃璇诲彇涓€涓瓧绗︿覆銆?ustring'
鏄墽琛岀浉鍚屼换鍔＄殑蹇嵎鏂瑰紡銆備篃灏辨槸璇达紝+0(%si):ustring 绛変环浜?+u0(%si):string銆?
娉ㄦ剰锛宬probe-event 鎻愪緵浜嗙敤鎴峰唴瀛樿闂娉曪紝浣嗗畠骞朵笉浼氶€忔槑鍦颁娇鐢ㄥ畠銆傝繖鎰忓懗鐫€濡傛灉浣犲鐢ㄦ埛鍐呭瓨浣跨敤鏅€氱殑
瑙ｅ紩鐢ㄦ垨 string 绫诲瀷锛屽畠鍙兘浼氬け璐ワ紝骞朵笖鍦ㄦ煇浜涙灦鏋勪笂鍙兘鎬绘槸澶辫触銆傜敤鎴峰繀椤讳粩缁嗘鏌ョ洰鏍囨暟鎹槸鍦ㄥ唴鏍?绌洪棿杩樻槸鐢ㄦ埛绌洪棿銆?
### 姣忔帰娴嬩簨浠惰繃婊わ紙Per-Probe Event Filtering锛?
姣忔帰娴嬩簨浠惰繃婊ゅ姛鑳藉厑璁镐綘鍦ㄦ瘡涓帰娴嬩笂璁剧疆涓嶅悓鐨勮繃婊ゅ櫒锛屽苟鍐冲畾鍝簺鍙傛暟浼氭樉绀哄湪璺熻釜缂撳啿鍖轰腑銆傚鏋滃湪
kprobe_events 涓?'p:' 鎴?'r:' 涔嬪悗鎸囧畾浜嗕簨浠跺悕锛屽畠浼氬湪 tracing/events/kprobes/<EVENT> 涓嬫坊鍔犱竴涓簨浠讹紝
鍦ㄨ鐩綍涓綘鍙互鐪嬪埌 'id'銆?enable'銆?format'銆?filter' 鍜?'trigger'銆?
enable:
  浣犲彲浠ラ€氳繃鍚戝叾鍐欏叆 1 鎴?0 鏉ュ惎鐢?绂佺敤璇ユ帰娴嬨€?
format:
  杩欐樉绀鸿鎺㈡祴浜嬩欢鐨勬牸寮忋€?
filter:
  浣犲彲浠ュ啓鍏ヨ浜嬩欢鐨勮繃婊よ鍒欍€?
id:
  杩欐樉绀鸿鎺㈡祴浜嬩欢鐨?id銆?
trigger:
  杩欏厑璁稿畨瑁呭綋浜嬩欢鍛戒腑鏃舵墽琛岀殑瑙﹀彂鍛戒护锛堣鎯呭弬瑙?Documentation/trace/events.rst 绗?6 鑺傦級銆?
### 浜嬩欢缁熻锛圗vent Profiling锛?
浣犲彲浠ラ€氳繃 /sys/kernel/tracing/kprobe_profile 鏌ョ湅鎺㈡祴鍛戒腑鍜屾湭鍛戒腑鐨勬€绘鏁般€傜涓€鍒楁槸浜嬩欢鍚嶏紝绗簩鍒?鏄帰娴嬪懡涓鏁帮紝绗笁鍒楁槸鎺㈡祴鏈懡涓鏁般€?
### 鍐呮牳鍚姩鍙傛暟锛圞ernel Boot Parameter锛?
浣犲彲浠ラ€氳繃 "kprobe_event=" 鍙傛暟鍦ㄥ唴鏍稿惎鍔ㄦ椂娣诲姞骞跺惎鐢ㄦ柊鐨?kprobe 浜嬩欢銆傝鍙傛暟鎺ュ彈浠ュ垎鍙峰垎闅旂殑 kprobe
浜嬩欢锛屽叾鏍煎紡涓?kprobe_events 绫讳技銆傚尯鍒湪浜庢帰娴嬪畾涔夊弬鏁版槸浠ラ€楀彿鍒嗛殧鐨勶細
```

  p:myprobe do_sys_open dfd=%ax filename=%dx flags=%cx mode=+4($stack)

```
```

  p:myprobe,do_sys_open,dfd=%ax,filename=%dx,flags=%cx,mode=+4($stack)


```
### 浣跨敤绀轰緥锛圲sage examples锛?
瑕佹坊鍔犱竴涓柊鐨勪簨浠朵綔涓烘帰娴嬶紝鍚?kprobe_events 鍐欏叆涓€涓柊鐨勫畾涔夛細
```

  echo 'p:myprobe do_sys_open dfd=%ax filename=%dx flags=%cx mode=+4($stack)' > /sys/kernel/tracing/kprobe_events

```
杩欎細鍦?do_sys_open() 鍑芥暟椤堕儴璁剧疆浜嗕竴涓?kprobe锛屾妸绗?1 鍒扮 4 涓弬鏁拌褰曚负 "myprobe" 浜嬩欢銆傛敞鎰忥紝姣忎釜
鍑芥暟鍙傛暟琚垎閰嶅埌鍝釜瀵勫瓨鍣?鏍堟潯鐩彇鍐充簬鏋舵瀯鐩稿叧鐨?ABI銆傚鏋滀綘涓嶇‘瀹?ABI锛岃灏濊瘯浣跨敤 perf-tools 鐨?probe
瀛愬懡浠わ紙浣犲彲浠ュ湪 tools/perf/ 涓嬫壘鍒板畠锛夈€傛濡傝繖涓ず渚嬫墍绀猴紝鐢ㄦ埛鍙互涓烘瘡涓弬鏁伴€夋嫨鏇寸啛鎮夌殑鍚嶇О銆?```

  echo 'r:myretprobe do_sys_open $retval' >> /sys/kernel/tracing/kprobe_events

```
杩欎細鍦?do_sys_open() 鍑芥暟鐨勮繑鍥炵偣璁剧疆浜嗕竴涓?kretprobe锛屾妸杩斿洖鍊艰褰曚负 "myretprobe" 浜嬩欢銆?浣犲彲浠ラ€氳繃 /sys/kernel/tracing/events/kprobes/<EVENT>/format 鏌ョ湅杩欎簺浜嬩欢鐨勬牸寮忋€?```

  cat /sys/kernel/tracing/events/kprobes/myprobe/format
  name: myprobe
  ID: 780
  format:
          field:unsigned short common_type;       offset:0;       size:2; signed:0;
          field:unsigned char common_flags;       offset:2;       size:1; signed:0;
          field:unsigned char common_preempt_count;       offset:3; size:1;signed:0;
          field:int common_pid;   offset:4;       size:4; signed:1;

          field:unsigned long __probe_ip; offset:12;      size:4; signed:0;
          field:int __probe_nargs;        offset:16;      size:4; signed:1;
          field:unsigned long dfd;        offset:20;      size:4; signed:0;
          field:unsigned long filename;   offset:24;      size:4; signed:0;
          field:unsigned long flags;      offset:28;      size:4; signed:0;
          field:unsigned long mode;       offset:32;      size:4; signed:0;


  print fmt: "(%lx) dfd=%lx filename=%lx flags=%lx mode=%lx", REC->__probe_ip,
  REC->dfd, REC->filename, REC->flags, REC->mode

```
浣犲彲浠ョ湅鍒帮紝璇ヤ簨浠舵嫢鏈?4 涓弬鏁帮紝姝ｅ浣犳墍鎸囧畾鐨勮〃杈惧紡閭ｆ牱銆?```

  echo > /sys/kernel/tracing/kprobe_events

```
杩欎細娓呴櫎鎵€鏈夋帰娴嬬偣銆?
鎴栬€咃紝
```

  echo -:myprobe >> kprobe_events

```
杩欎細閫夋嫨鎬у湴娓呴櫎鎺㈡祴鐐广€?
鍦ㄥ畾涔変箣鍚庯紝姣忎釜浜嬩欢榛樿鏄鐢ㄧ殑銆傝璺熻釜杩欎簺浜嬩欢锛屼綘闇€瑕佸惎鐢ㄥ畠銆?```

  echo 1 > /sys/kernel/tracing/events/kprobes/myprobe/enable
  echo 1 > /sys/kernel/tracing/events/kprobes/myretprobe/enable

```
浣跨敤浠ヤ笅鍛戒护鍦ㄤ竴娈靛尯闂村唴寮€濮嬭窡韪€?```

    # echo 1 > tracing_on
    Open something...
    # echo 0 > tracing_on

```
浣犲彲浠ラ€氳繃 /sys/kernel/tracing/trace 鏌ョ湅璺熻釜鍒扮殑淇℃伅銆?```

  cat /sys/kernel/tracing/trace
  # tracer: nop
  #
  #           TASK-PID    CPU#    TIMESTAMP  FUNCTION
  #              | |       |          |         |
             <...>-1447  [001] 1038282.286875: myprobe: (do_sys_open+0x0/0xd6) dfd=3 filename=7fffd1ec4440 flags=8000 mode=0
             <...>-1447  [001] 1038282.286878: myretprobe: (sys_openat+0xc/0xe <- do_sys_open) $retval=fffffffffffffffe
             <...>-1447  [001] 1038282.286885: myprobe: (do_sys_open+0x0/0xd6) dfd=ffffff9c filename=40413c flags=8000 mode=1b6
             <...>-1447  [001] 1038282.286915: myretprobe: (sys_open+0x1b/0x1d <- do_sys_open) $retval=3
             <...>-1447  [001] 1038282.286969: myprobe: (do_sys_open+0x0/0xd6) dfd=ffffff9c filename=4041c6 flags=98800 mode=10
             <...>-1447  [001] 1038282.286976: myretprobe: (sys_open+0x1b/0x1d <- do_sys_open) $retval=3


```
姣忚鏄剧ず鍐呮牳鍛戒腑涓€涓簨浠剁殑鏃跺埢锛岃€?<- SYMBOL 琛ㄧず鍐呮牳浠?SYMBOL 杩斿洖锛堜緥濡?"sys_open+0x1b/0x1d <- do_sys_open"
琛ㄧず鍐呮牳浠?do_sys_open 杩斿洖鍒?sys_open+0x1b锛夈€?