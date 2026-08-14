
## Eprobe - 鍩轰簬浜嬩欢鐨勬帰閽堣拷韪?
:Author: Steven Rostedt <rostedt@goodmis.org>

- 涓?v6.17 鎾板啓

## 姒傝堪

Eprobes 鏄斁缃湪鐜版湁浜嬩欢涔嬩笂鐨勫姩鎬佷簨浠讹紝鐢ㄤ簬瑙ｅ紩鐢ㄤ綔涓烘寚閽堢殑瀛楁锛屾垨鍙槸闄愬埗璁板綍鍒拌拷韪簨浠朵腑鐨勫瓧娈点€?
Eprobes 渚濊禆浜?kprobe 浜嬩欢锛屽洜姝よ鍚敤姝ゅ姛鑳斤紝璇风敤 `CONFIG_EPROBE_EVENTS=y` 鏋勫缓浣犵殑鍐呮牳銆?
Eprobes 閫氳繃 /sys/kernel/tracing/dynamic_events 鏂囦欢鍒涘缓銆?
### eprobe_events 姒傝

```

  e[:[EGRP/][EEVENT]] GRP.EVENT [FETCHARGS]	: Set a probe
  -:[EGRP/][EEVENT]				: Clear a probe

 EGRP		: Group name of the new event. If omitted, use "eprobes" for it.
 EEVENT		: Event name. If omitted, the event name is generated and will
		  be the same event name as the event it attached to.
 GRP		: Group name of the event to attach to.
 EVENT		: Event name of the event to attach to.

 FETCHARGS	: Arguments. Each probe can have up to 128 args.
  $FIELD	: Fetch the value of the event field called FIELD.
  @ADDR		: Fetch memory at ADDR (ADDR should be in kernel)
  @SYM[+|-offs]	: Fetch memory at SYM +|- offs (SYM should be a data symbol)
  $comm		: Fetch current task comm.
  +|-[u]OFFS(FETCHARG) : Fetch memory at FETCHARG +|- OFFS address.(\*3)(\*4)
  \IMM		: Store an immediate value to the argument.
  NAME=FETCHARG : Set NAME as the argument name of FETCHARG.
  FETCHARG:TYPE : Set TYPE as the type of FETCHARG. Currently, basic types
		  (u8/u16/u32/u64/s8/s16/s32/s64), hexadecimal types
		  (x8/x16/x32/x64), VFS layer common type(%pd/%pD), "char",
                  "string", "ustring", "symbol", "symstr" and "bitfield" are
                  supported.

```
### 绫诲瀷

涓婇潰鐨?FETCHARGS 涓?Documentation/trace/kprobetrace.rst 涓弿杩扮殑 kprobe 浜嬩欢闈炲父鐩镐技銆?
eprobes 涓?kprobes 鐨?FETCHARGS 涔嬮棿鐨勫尯鍒湪浜庯紝eprobes 鏈変竴涓?`$FIELD` 鍛戒护锛岀敤浜庤繑鍥炴墍闄勫姞浜嬩欢瀛楁鐨勫唴瀹广€侲probes 鏃犳硶璁块棶 kprobes 鎵€鎷ユ湁鐨勫瘎瀛樺櫒銆佹爤鍜屽嚱鏁板弬鏁般€?
濡傛灉涓€涓瓧娈靛弬鏁版槸涓€涓寚閽堬紝瀹冨彲浠ュ儚鍐呭瓨鍦板潃涓€鏍蜂娇鐢?FETCHARGS 璇硶杩涜瑙ｅ紩鐢ㄣ€?
### 闄勫姞鍒板姩鎬佷簨浠?
Eprobes 鍙互闄勫姞鍒板姩鎬佷簨浠讹紝涔熷彲浠ラ檮鍔犲埌鏅€氫簨浠躲€傚畠鍙互闄勫姞鍒?kprobe 浜嬩欢銆乻ynthetic 浜嬩欢鎴?fprobe 浜嬩欢銆傚鏋滀竴涓瓧娈电殑绫诲瀷闇€瑕佹敼鍙橈紝杩欎細寰堟湁鐢ㄣ€傝鍙傞槄涓嬮潰鐨勭ず渚?2銆?
## 鐢ㄦ硶绀轰緥

### 绀轰緥 1

eprobes 鐨勫熀鏈敤閫旀槸闄愬埗璁板綍鍒拌拷韪紦鍐插尯涓殑鏁版嵁銆備緥濡傦紝涓€涓父瑙佺殑瑕佽拷韪殑浜嬩欢鏄?sched_switch
```

	field:unsigned short common_type;	offset:0;	size:2;	signed:0;
	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;
	field:unsigned char common_preempt_count;	offset:3;	size:1;	signed:0;
	field:int common_pid;	offset:4;	size:4;	signed:1;

	field:char prev_comm[16];	offset:8;	size:16;	signed:0;
	field:pid_t prev_pid;	offset:24;	size:4;	signed:1;
	field:int prev_prio;	offset:28;	size:4;	signed:1;
	field:long prev_state;	offset:32;	size:8;	signed:1;
	field:char next_comm[16];	offset:40;	size:16;	signed:0;
	field:pid_t next_pid;	offset:56;	size:4;	signed:1;
	field:int next_prio;	offset:60;	size:4;	signed:1;

```
鍓嶅洓涓瓧娈垫槸鎵€鏈変簨浠跺叡鏈夌殑锛屾棤娉曡闄愬埗銆備絾璇ヤ簨浠剁殑鍏朵綑閮ㄥ垎鏈?60 瀛楄妭鐨勪俊鎭€傚畠璁板綍浜嗚璋冨害鍑哄拰璋冨叆鐨勫墠鍚庝换鍔＄殑鍚嶇О锛屼互鍙婂畠浠殑 pid 鍜屼紭鍏堢骇銆傚畠杩樿褰曚簡鍓嶄竴浠诲姟鐨勭姸鎬併€傚鏋滃彧鍏冲績浠诲姟鐨?pid锛屼负浠€涔堣娴垂鐜舰缂撳啿鍖烘潵璁板綍鎵€鏈夊叾浠栧瓧娈靛憿锛?
Eprobe 鍙互闄愬埗璁板綍鐨勫唴瀹广€傛敞鎰忥紝杩欏鎬ц兘娌℃湁甯姪锛屽洜涓烘墍鏈夊瓧娈甸兘浼氳褰曞湪涓€涓复鏃剁紦鍐插尯涓互澶勭悊 eprobe銆?```

 # echo 'e:sched/switch sched.sched_switch prev=$prev_pid:u32 next=$next_pid:u32' >> /sys/kernel/tracing/dynamic_events
 # echo 1 > /sys/kernel/tracing/events/sched/switch/enable
 # cat /sys/kernel/tracing/trace

 # tracer: nop
 #
 # entries-in-buffer/entries-written: 2721/2721   #P:8
 #
 #                                _-----=> irqs-off/BH-disabled
 #                               / _----=> need-resched
 #                              | / _---=> hardirq/softirq
 #                              || / _--=> preempt-depth
 #                              ||| / _-=> migrate-disable
 #                              |||| /     delay
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
     sshd-session-1082    [004] d..4.  5041.239906: switch: (sched.sched_switch) prev=1082 next=0
             bash-1085    [001] d..4.  5041.240198: switch: (sched.sched_switch) prev=1085 next=141
    kworker/u34:5-141     [001] d..4.  5041.240259: switch: (sched.sched_switch) prev=141 next=1085
           <idle>-0       [004] d..4.  5041.240354: switch: (sched.sched_switch) prev=0 next=1082
             bash-1085    [001] d..4.  5041.240385: switch: (sched.sched_switch) prev=1085 next=141
    kworker/u34:5-141     [001] d..4.  5041.240410: switch: (sched.sched_switch) prev=141 next=1085
             bash-1085    [001] d..4.  5041.240478: switch: (sched.sched_switch) prev=1085 next=0
     sshd-session-1082    [004] d..4.  5041.240526: switch: (sched.sched_switch) prev=1082 next=0
           <idle>-0       [001] d..4.  5041.247524: switch: (sched.sched_switch) prev=0 next=90
           <idle>-0       [002] d..4.  5041.247545: switch: (sched.sched_switch) prev=0 next=16
      kworker/1:1-90      [001] d..4.  5041.247580: switch: (sched.sched_switch) prev=90 next=0
        rcu_sched-16      [002] d..4.  5041.247591: switch: (sched.sched_switch) prev=16 next=0
           <idle>-0       [002] d..4.  5041.257536: switch: (sched.sched_switch) prev=0 next=16
        rcu_sched-16      [002] d..4.  5041.257573: switch: (sched.sched_switch) prev=16 next=0

```
娉ㄦ剰锛屽鏋滃湪 prev_pid 鍜?next_pid 涔嬪悗涓嶅姞涓娾€渦32鈥濓紝杩欎簺鍊奸粯璁や細浠ュ崄鍏繘鍒舵樉绀恒€?
### 绀轰緥 2

濡傛灉瑕佽褰曟煇涓壒瀹氱殑绯荤粺璋冪敤锛屼絾 syscalls 浜嬩欢鏈惎鐢紝浠嶇劧鍙互浣跨敤 raw_syscalls锛堢郴缁熻皟鐢ㄤ簨浠朵笉鏄櫘閫氫簨浠讹紝鑰屾槸鍦ㄥ唴鏍镐腑鐢?raw_syscalls 浜嬩欢鍒涘缓锛夈€備负浜嗚拷韪?openat 绯荤粺璋冪敤锛屽彲浠ュ湪 raw_syscalls 浜嬩欢涔嬩笂鍒涘缓涓€涓簨浠舵帰閽堬細
```

 # cd /sys/kernel/tracing
 # cat events/raw_syscalls/sys_enter/format
 name: sys_enter
 ID: 395
 format:
	field:unsigned short common_type;	offset:0;	size:2;	signed:0;
	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;
	field:unsigned char common_preempt_count;	offset:3;	size:1;	signed:0;
	field:int common_pid;	offset:4;	size:4;	signed:1;

	field:long id;	offset:8;	size:8;	signed:1;
	field:unsigned long args[6];	offset:16;	size:48;	signed:0;

 print fmt: "NR %ld (%lx, %lx, %lx, %lx, %lx, %lx)", REC->id, REC->args[0], REC->args[1], REC->args[2], REC->args[3], REC->args[4], REC->args[5]

```
浠庢簮浠ｇ爜鐪嬶紝sys_openat() 鍏锋湁锛?```

 int sys_openat(int dirfd, const char *path, int flags, mode_t mode)
 {
	return my_syscall4(__NR_openat, dirfd, path, flags, mode);
 }

```
path 鏄浜屼釜鍙傛暟锛岃€岃繖姝ｆ槸鎯宠鐨勩€?```

 # echo 'e:openat raw_syscalls.sys_enter nr=$id filename=+8($args):ustring' >> dynamic_events

```
杩欐槸鍦?x86_64 涓婅繍琛岀殑锛屽叾涓瓧澶у皬涓?8 瀛楄妭锛宱penat 绯荤粺璋冪敤 __NR_openat 璁剧疆涓?257銆?```

 # echo 'nr == 257' > events/eprobes/openat/filter

```
鐜板湪鍚敤璇ヤ簨浠跺苟鏌ョ湅杩借釜璁板綍銆?```

 # echo 1 > events/eprobes/openat/enable
 # cat trace

 # tracer: nop
 #
 # entries-in-buffer/entries-written: 4/4   #P:8
 #
 #                                _-----=> irqs-off/BH-disabled
 #                               / _----=> need-resched
 #                              | / _---=> hardirq/softirq
 #                              || / _--=> preempt-depth
 #                              ||| / _-=> migrate-disable
 #                              |||| /     delay
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
              cat-1298    [003] ...2.  2060.875970: openat: (raw_syscalls.sys_enter) nr=0x101 filename=(fault)
              cat-1298    [003] ...2.  2060.876197: openat: (raw_syscalls.sys_enter) nr=0x101 filename=(fault)
              cat-1298    [003] ...2.  2060.879126: openat: (raw_syscalls.sys_enter) nr=0x101 filename=(fault)
              cat-1298    [003] ...2.  2060.879639: openat: (raw_syscalls.sys_enter) nr=0x101 filename=(fault)

```
filename 鏄剧ず鈥?fault)鈥濄€傝繖寰堝彲鑳芥槸鍥犱负 filename 灏氭湭琚媺鍏ュ唴瀛橈紝鑰屽綋鍓嶇殑 trace 浜嬩欢鏃犳硶 fault in锛堟寜闇€璋冨叆锛夊皻鏈嚭鐜扮殑鍐呭瓨銆傚綋 eprobe 灏濊瘯璇诲彇灏氭湭琚?fault in 鐨勫唴瀛樻椂锛屽畠浼氭樉绀衡€?fault)鈥濇枃鏈€?
涓轰簡缁曡繃杩欎竴鐐癸紝鐢变簬鍐呮牳寰堝彲鑳藉皢杩欎釜 filename 鎷夊叆骞朵娇鍏跺瓨鍦紝灏嗗叾闄勫姞鍒颁竴涓?synthetic 浜嬩欢涓婏紝璇ヤ簨浠跺彲浠ュ皢 filename 鐨勫湴鍧€浠庝簨浠剁殑鍏ュ彛浼犻€掑埌浜嬩欢鐨勬湯灏撅紝杩欏彲鐢ㄤ簬鍦ㄧ郴缁熻皟鐢ㄨ繑鍥炴椂鏄剧ず filename銆?
```

 # echo 1 > events/eprobes/openat/enable
 # echo '-:openat' >> dynamic_events

```
```

 # echo 'e:openat_start raw_syscalls.sys_enter nr=$id filename=+8($args):x64' >> dynamic_events

```
鍒涘缓涓€涓?synthetic 浜嬩欢锛屽皢 filename 鐨勫湴鍧€浼犻€掑埌
```

 # echo 's:filename u64 file' >> dynamic_events
 # echo 'hist:keys=common_pid:f=filename if nr == 257' > events/eprobes/openat_start/trigger
 # echo 'hist:keys=common_pid:file=$f:onmatch(eprobes.openat_start).trace(filename,$file) if id == 257' > events/raw_syscalls/sys_exit/trigger

```
鏃㈢劧 filename 鐨勫湴鍧€宸茶浼犻€掑埌绯荤粺璋冪敤鐨勬湯灏撅紝鍒涘缓鍙︿竴涓?eprobe 闄勫姞鍒伴€€鍑轰簨浠朵互鏄剧ず
```

 # echo 'e:openat synthetic.filename filename=+0($file):ustring' >> dynamic_events
 # echo 1 > events/eprobes/openat/enable
 # cat trace

 # tracer: nop
 #
 # entries-in-buffer/entries-written: 4/4   #P:8
 #
 #                                _-----=> irqs-off/BH-disabled
 #                               / _----=> need-resched
 #                              | / _---=> hardirq/softirq
 #                              || / _--=> preempt-depth
 #                              ||| / _-=> migrate-disable
 #                              |||| /     delay
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
              cat-1331    [001] ...5.  2944.787977: openat: (synthetic.filename) filename="/etc/ld.so.cache"
              cat-1331    [001] ...5.  2944.788480: openat: (synthetic.filename) filename="/lib/x86_64-linux-gnu/libc.so.6"
              cat-1331    [001] ...5.  2944.793426: openat: (synthetic.filename) filename="/usr/lib/locale/locale-archive"
              cat-1331    [001] ...5.  2944.831362: openat: (synthetic.filename) filename="trace"

```
### 绀轰緥 3

濡傛灉鏈夊彲鐢ㄧ殑 syscall trace 浜嬩欢锛屼笂杩板仛娉曞氨涓嶉渶瑕佺涓€涓?```

 # echo 's:filename u64 file' >> dynamic_events
 # echo 'hist:keys=common_pid:f=filename' > events/syscalls/sys_enter_openat/trigger
 # echo 'hist:keys=common_pid:file=$f:onmatch(syscalls.sys_enter_openat).trace(filename,$file)' > events/syscalls/sys_exit_openat/trigger
 # echo 'e:openat synthetic.filename filename=+0($file):ustring' >> dynamic_events
 # echo 1 > events/eprobes/openat/enable

```
鑰岃繖浼氫骇鐢熶笌绀轰緥 2 鐩稿悓鐨勭粨鏋溿€?