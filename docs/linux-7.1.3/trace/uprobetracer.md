## Uprobe-tracer锛氬熀浜?Uprobe 鐨勪簨浠惰窡韪?

:Author: Srikar Dronamraju


### 姒傝堪


鍩轰簬 uprobe 鐨勮窡韪簨浠朵笌鍩轰簬 kprobe 鐨勮窡韪簨浠剁被浼笺€傝鍚敤姝ゅ姛鑳斤紝璇风敤
CONFIG_UPROBE_EVENTS=y 鏋勫缓浣犵殑鍐呮牳銆?
涓?kprobe-event 璺熻釜鍣ㄧ被浼硷紝杩欎笉闇€瑕侀€氳繃 current_tracer 婵€娲汇€傚彇鑰屼唬涔嬬殑鏄紝閫氳繃
/sys/kernel/tracing/uprobe_events 娣诲姞鎺㈡祴鐐癸紝骞堕€氳繃
/sys/kernel/tracing/events/uprobes/<EVENT>/enable 鍚敤瀹冦€?
浣嗕笌 kprobe-event 璺熻釜鍣ㄤ笉鍚岋紝uprobe 浜嬩欢鎺ュ彛鏈熸湜鐢ㄦ埛璁＄畻鎺㈡祴鐐瑰湪瀵硅薄涓殑鍋忕Щ閲忋€?
浣犱篃鍙互浣跨敤 /sys/kernel/tracing/dynamic_events 浠ｆ浛 uprobe_events銆傝鎺ュ彛杩樺皢涓?鍏朵粬鍔ㄦ€佷簨浠舵彁渚涚粺涓€璁块棶銆?
### uprobe_tracer 璇硶


```

  p[:[GRP/][EVENT]] PATH:OFFSET [FETCHARGS] : Set a uprobe
  r[:[GRP/][EVENT]] PATH:OFFSET [FETCHARGS] : Set a return uprobe (uretprobe)
  p[:[GRP/][EVENT]] PATH:OFFSET%return [FETCHARGS] : Set a return uprobe (uretprobe)
  -:[GRP/][EVENT]                           : Clear uprobe or uretprobe event

  GRP           : Group name. If omitted, "uprobes" is the default value.
  EVENT         : Event name. If omitted, the event name is generated based
                  on PATH+OFFSET.
  PATH          : Path to an executable or a library.
  OFFSET        : Offset where the probe is inserted.
  OFFSET%return : Offset where the return probe is inserted.

  FETCHARGS     : Arguments. Each probe can have up to 128 args.
   %REG         : Fetch register REG
   @ADDR	: Fetch memory at ADDR (ADDR should be in userspace)
   @+OFFSET	: Fetch memory at OFFSET (OFFSET from same file as PATH)
   $stackN	: Fetch Nth entry of stack (N >= 0)
   $stack	: Fetch stack address.
   $retval	: Fetch return value.(\*1)
   $comm	: Fetch current task comm.
   +|-[u]OFFS(FETCHARG) : Fetch memory at FETCHARG +|- OFFS address.(\*2)(\*3)
   \IMM		: Store an immediate value to the argument.
   NAME=FETCHARG     : Set NAME as the argument name of FETCHARG.
   FETCHARG:TYPE     : Set TYPE as the type of FETCHARG. Currently, basic types
		       (u8/u16/u32/u64/s8/s16/s32/s64), hexadecimal types
		       (x8/x16/x32/x64), "string" and bitfield are supported.

  (\*1) only for return probe.
  (\*2) this is useful for fetching a field of data structures.
  (\*3) Unlike kprobe event, "u" prefix will just be ignored, because uprobe
        events can access only user-space memory.

```
### 绫诲瀷


fetch-args 鏀寔澶氱绫诲瀷銆俇probe 璺熻釜鍣ㄥ皢鎸夌粰瀹氱被鍨嬭闂唴瀛樸€傚墠缂€ 's' 鍜?'u' 鍒嗗埆
琛ㄧず杩欎簺绫诲瀷鏄湁绗﹀彿鍜屾棤绗﹀彿鐨勩€?x' 鍓嶇紑琛ㄧず瀹冩槸鏃犵鍙风殑銆傝璺熻釜鐨勫弬鏁颁互鍗佽繘鍒?锛?s' 鍜?'u'锛夋垨鍗佸叚杩涘埗锛?x'锛夋樉绀恒€傚湪娌℃湁绫诲瀷杞崲鐨勬儏鍐典笅锛屾牴鎹灦鏋勪娇鐢?'x32'
鎴?'x64'锛堜緥濡?x86-32 浣跨敤 x32锛寈86-64 浣跨敤 x64锛夈€?瀛楃涓茬被鍨嬫槸涓€绉嶇壒娈婄被鍨嬶紝瀹冧粠鐢ㄦ埛绌洪棿鑾峰彇涓€涓?浠?null 缁撳熬"鐨勫瓧绗︿覆銆?浣嶅煙鏄彟涓€绉嶇壒娈婄被鍨嬶紝瀹冩帴鍙?3 涓弬鏁帮細浣嶅銆佷綅
```

 b<bit-width>@<bit-offset>/<container-size>

```
瀵逛簬 $comm锛岄粯璁ょ被鍨嬫槸 "string"锛涗换浣曞叾浠栫被鍨嬮兘鏃犳晥銆?

### 浜嬩欢缁熻


浣犲彲浠ラ€氳繃 /sys/kernel/tracing/uprobe_profile 妫€鏌ユ瘡涓簨浠剁殑鎺㈡祴鍛戒腑鎬绘暟銆傜涓€鍒?鏄枃浠跺悕锛岀浜屽垪鏄簨浠跺悕锛岀涓夊垪鏄帰娴嬪懡涓鏁般€?
### 浣跨敤绀轰緥


 - Add a probe as a new uprobe event, write a new definition to uprobe_events
```

    echo 'p /bin/bash:0x4245c0' > /sys/kernel/tracing/uprobe_events

 * Add a probe as a new uretprobe event::

    echo 'r /bin/bash:0x4245c0' > /sys/kernel/tracing/uprobe_events

 * Unset registered event::

    echo '-:p_bash_0x4245c0' >> /sys/kernel/tracing/uprobe_events

 * Print out the events that are registered::

    cat /sys/kernel/tracing/uprobe_events

 * Clear all events::

    echo > /sys/kernel/tracing/uprobe_events

```
浠ヤ笅绀轰緥灞曠ず浜嗗浣曡浆鍌ㄦ寚浠ゆ寚閽堝拰 %ax 瀵勫瓨鍣?```

    # cd /sys/kernel/tracing/
    # cat /proc/`pgrep zsh`/maps | grep /bin/zsh | grep r-xp
    00400000-0048a000 r-xp 00000000 08:03 130904 /bin/zsh
    # objdump -T /bin/zsh | grep -w zfree
    0000000000446420 g    DF .text  0000000000000012  Base        zfree

```
0x46420 鏄璞?/bin/zsh 涓?zfree 鐨勫亸绉婚噺锛岃瀵硅薄琚姞杞藉埌
```

    # echo 'p:zfree_entry /bin/zsh:0x46420 %ip %ax' > uprobe_events

```
```

    # echo 'r:zfree_exit /bin/zsh:0x46420 %ip %ax' >> uprobe_events

```
	涓€?
鎴戜滑鍙互閫氳繃鏌ョ湅 uprobe_events 鏂囦欢鏉ユ煡鐪嬪凡娉ㄥ唽鐨勪簨浠躲€?```

    # cat uprobe_events
    p:uprobes/zfree_entry /bin/zsh:0x00046420 arg1=%ip arg2=%ax
    r:uprobes/zfree_exit /bin/zsh:0x00046420 arg1=%ip arg2=%ax

```
浜嬩欢鐨勬牸寮忓彲浠ラ€氳繃鏌ョ湅鏂囦欢 events/uprobes/zfree_entry/format 鏉ユ煡鐪嬨€?```

    # cat events/uprobes/zfree_entry/format
    name: zfree_entry
    ID: 922
    format:
         field:unsigned short common_type;         offset:0;  size:2; signed:0;
         field:unsigned char common_flags;         offset:2;  size:1; signed:0;
         field:unsigned char common_preempt_count; offset:3;  size:1; signed:0;
         field:int common_pid;                     offset:4;  size:4; signed:1;
         field:int common_padding;                 offset:8;  size:4; signed:1;

         field:unsigned long __probe_ip;           offset:12; size:4; signed:0;
         field:u32 arg1;                           offset:16; size:4; signed:0;
         field:u32 arg2;                           offset:20; size:4; signed:0;

    print fmt: "(%lx) arg1=%lx arg2=%lx", REC->__probe_ip, REC->arg1, REC->arg2

```
瀹氫箟涔嬪悗锛屾瘡涓簨浠堕粯璁ゆ槸绂佺敤鐨勩€備负浜嗚窡韪繖浜涗簨浠?```

    # echo 1 > events/uprobes/enable

```
璁╂垜浠紑濮嬭窡韪紝鐫＄湢涓€娈垫椂闂寸劧鍚庡仠姝㈣窡韪€?```

    # echo 1 > tracing_on
    # sleep 20
    # echo 0 > tracing_on

```
```

    # echo 0 > events/uprobes/enable

```
浣犲彲浠ラ€氳繃 /sys/kernel/tracing/trace 鏌ョ湅璺熻釜淇℃伅銆?```

    # cat trace
    # tracer: nop
    #
    #           TASK-PID    CPU#    TIMESTAMP  FUNCTION
    #              | |       |          |         |
                 zsh-24842 [006] 258544.995456: zfree_entry: (0x446420) arg1=446420 arg2=79
                 zsh-24842 [007] 258545.000270: zfree_exit:  (0x446540 <- 0x446420) arg1=446540 arg2=0
                 zsh-24842 [002] 258545.043929: zfree_entry: (0x446420) arg1=446420 arg2=79
                 zsh-24842 [004] 258547.046129: zfree_exit:  (0x446540 <- 0x446420) arg1=446540 arg2=0

```
杈撳嚭鏄剧ず锛寀probe 琚?pid 24842 瑙﹀彂锛宨p 涓?0x446420锛宎x 瀵勫瓨鍣ㄥ唴瀹逛负 79銆傝€?uretprobe
琚Е鍙戞椂 ip 鍦?0x446540锛屽搴旂殑鍑芥暟鍏ュ彛鍦?0x446420銆?