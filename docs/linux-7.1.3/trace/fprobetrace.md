
## 鍩轰簬 Fprobe 鐨勪簨浠惰窡韪?



### 姒傝堪


Fprobe 浜嬩欢涓?kprobe 浜嬩欢绫讳技锛屼絾浠呴檺浜庡湪
鍑芥暟鐨勫叆鍙ｅ拰鍑哄彛澶勮繘琛屾帰娴嬨€傚浜庤澶氬彧璺熻釜鏌愪簺
鐗瑰畾鍑芥暟鐨勭敤渚嬫潵璇达紝杩欏凡缁忚冻澶熶簡銆?

鏈枃妗ｄ篃娑电洊 tracepoint 鎺㈡祴浜嬩欢锛坱probe锛夛紝鍥犱负瀹?
鍚屾牱鍙湪 tracepoint 鍏ュ彛澶勫伐浣溿€傜敤鎴峰彲浠ヨ窡韪?
tracepoint 鐨勪竴閮ㄥ垎鍙傛暟锛屾垨鑰呮病鏈?trace-event 鐨?tracepoint锛?
鍚庤€呬笉浼氬湪 tracefs 涓婃毚闇层€?

涓庡叾浠栧姩鎬佷簨浠朵竴鏍凤紝fprobe 浜嬩欢鍜?tracepoint 鎺㈡祴
浜嬩欢閫氳繃 tracefs 涓婄殑 `dynamic_events` 鎺ュ彛鏂囦欢瀹氫箟銆?

### fprobe 浜嬩欢鐨勮娉?

```

  f[:[GRP1/][EVENT1]] SYM [FETCHARGS]                       : Probe on function entry
  f[MAXACTIVE][:[GRP1/][EVENT1]] SYM%return [FETCHARGS]     : Probe on function exit
  t[:[GRP2/][EVENT2]] TRACEPOINT [FETCHARGS]                : Probe on tracepoint

 GRP1           : Group name for fprobe. If omitted, use "fprobes" for it.
 GRP2           : Group name for tprobe. If omitted, use "tracepoints" for it.
 EVENT1         : Event name for fprobe. If omitted, the event name is
                  "SYM__entry" or "SYM__exit".
 EVENT2         : Event name for tprobe. If omitted, the event name is
                  the same as "TRACEPOINT", but if the "TRACEPOINT" starts
                  with a digit character, "_TRACEPOINT" is used.
 MAXACTIVE      : Maximum number of instances of the specified function that
                  can be probed simultaneously, or 0 for the default value
                  as defined in Documentation/trace/fprobe.rst

 FETCHARGS      : Arguments. Each probe can have up to 128 args.
  ARG           : Fetch "ARG" function argument using BTF (only for function
                  entry or tracepoint.) (\*1)
  @ADDR         : Fetch memory at ADDR (ADDR should be in kernel)
  @SYM[+|-offs] : Fetch memory at SYM +|- offs (SYM should be a data symbol)
  $stackN       : Fetch Nth entry of stack (N >= 0)
  $stack        : Fetch stack address.
  $argN         : Fetch the Nth function argument. (N >= 1) (\*2)
  $retval       : Fetch return value.(\*3)
  $comm         : Fetch current task comm.
  +|-[u]OFFS(FETCHARG) : Fetch memory at FETCHARG +|- OFFS address.(\*4)(\*5)
  \IMM          : Store an immediate value to the argument.
  NAME=FETCHARG : Set NAME as the argument name of FETCHARG.
  FETCHARG:TYPE : Set TYPE as the type of FETCHARG. Currently, basic types
                  (u8/u16/u32/u64/s8/s16/s32/s64), hexadecimal types
                  (x8/x16/x32/x64), "char", "string", "ustring", "symbol", "symstr"
                  and bitfield are supported.

  (\*1) This is available only when BTF is enabled.
  (\*2) only for the probe on function entry (offs == 0). Note, this argument access
        is best effort, because depending on the argument type, it may be passed on
        the stack. But this only support the arguments via registers.
  (\*3) only for return probe. Note that this is also best effort. Depending on the
        return value type, it might be passed via a pair of registers. But this only
        accesses one register.
  (\*4) this is useful for fetching a field of data structures.
  (\*5) "u" means user-space dereference.

```
鏈夊叧 TYPE 鐨勮缁嗕俊鎭紝璇峰弬瑙?kprobetrace 鏂囨。 <kprobetrace_types>銆?

### 閫€鍑烘椂鐨勫嚱鏁板弬鏁?

鍦ㄩ€€鍑烘帰娴嬩腑鍙互浣跨敤 $arg<N> fetcharg 璁块棶鍑芥暟鍙傛暟銆傝繖
鏈夊姪浜庝竴娆℃€ц褰曞嚱鏁板弬鏁板拰杩斿洖鍊硷紝骞?
璺熻釜缁撴瀯浣撳瓧娈电殑宸紓锛堢敤浜庤皟璇曞嚱鏁版槸鍚︽纭?
鏇存柊浜嗙粰瀹氱殑鏁版嵁缁撴瀯锛?
鍏跺伐浣滄柟寮忚瑙佷笅闈㈢殑绀轰緥 <fprobetrace_exit_args_sample>銆?

### BTF 鍙傛暟

BTF锛圔PF Type Format锛夊弬鏁板厑璁哥敤鎴锋寜鐓у悕绉拌€岄潪 `$argN` 鏉ヨ窡韪嚱鏁板拰 tracepoint 鐨?
鍙傛暟銆傝鐗规€у湪鍐呮牳閰嶇疆浜?CONFIG_BPF_SYSCALL 鍜?CONFIG_DEBUG_INFO_BTF 鏃跺彲鐢ㄣ€?
濡傛灉鐢ㄦ埛鍙寚瀹?BTF 鍙傛暟锛屼簨浠剁殑鍙傛暟鍚嶄篃浼?
闅忎箣纭畾銆?
```

 # echo 'f:myprobe vfs_read count pos' >> dynamic_events
 # cat dynamic_events
 f:fprobes/myprobe vfs_read count=count pos=pos

```
瀹冭繕浼氭牴鎹?BTF 淇℃伅閫夋嫨鑾峰彇绫诲瀷銆備緥濡傦紝鍦ㄤ笂闈?
鐨勭ず渚嬩腑锛宍count` 鏄?unsigned long锛岃€?`pos` 鏄竴涓寚閽堛€傚洜姝わ紝
涓よ€呴兘琚浆鎹负 64 浣嶆棤绗﹀彿 long锛屼絾鍙湁 `pos` 甯︽湁 "%Lx"
```

 # cat events/fprobes/myprobe/format
 name: myprobe
 ID: 1313
 format:
	field:unsigned short common_type;	offset:0;	size:2;	signed:0;
	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;
	field:unsigned char common_preempt_count;	offset:3;	size:1;	signed:0;
	field:int common_pid;	offset:4;	size:4;	signed:1;

	field:unsigned long __probe_ip;	offset:8;	size:8;	signed:0;
	field:u64 count;	offset:16;	size:8;	signed:0;
	field:u64 pos;	offset:24;	size:8;	signed:0;

 print fmt: "(%lx) count=%Lu pos=0x%Lx", REC->__probe_ip, REC->count, REC->pos

```
濡傛灉鐢ㄦ埛涓嶇‘瀹氬弬鏁扮殑鍚嶇О锛宍$arg**` 浼氬緢鏈夊府鍔┿€俙$arg**`
```

 # echo 'f:myprobe vfs_read $arg*' >> dynamic_events
 # cat dynamic_events
 f:fprobes/myprobe vfs_read file=file buf=buf count=count pos=pos

```
BTF 涔熶細褰卞搷 `$retval`銆傚鏋滅敤鎴锋病鏈夎缃换浣曠被鍨嬶紝杩斿洖鍊?
绫诲瀷浼氳嚜鍔ㄤ粠 BTF 涓€夊彇銆傚鏋滃嚱鏁拌繑鍥?`void`锛?
鍒?`$retval` 浼氳鎷掔粷銆?

鍙互浣跨敤璁块棶杩愮畻绗?`->` 鏉ヨ闂暟鎹粨鏋勭殑鏁版嵁瀛楁
```

```
# echo 't sched_switch preempt prev_pid=prev->pid next_pid=next->pid' >> dynamic_events

瀛楁璁块棶杩愮畻绗?`->` 鍜?`.` 鍙互缁勫悎浣跨敤锛屼互璁块棶鏇存繁灞傜殑
鎴愬憳浠ュ強鐢辫鎴愬憳鎸囧悜鐨勫叾浠栫粨鏋勪綋鎴愬憳銆備緥濡?`foo->bar.baz->qux`
濡傛灉瀛樺湪娌℃湁鍚嶇О鐨?union 鎴愬憳锛屽彲浠ュ儚 C 浠ｇ爜閭ｆ牱鐩存帴璁块棶瀹冦€?
```

 struct {
	union {
	int a;
	int b;
	};
 } *foo;

```
瑕佽闂?`a` 鍜?`b`锛屽湪杩欑鎯呭喌涓嬭浣跨敤 `foo->a` 鍜?`foo->b`銆?

杩欑鏁版嵁瀛楁璁块棶涔熷彲閫氳繃 `$retval` 鐢ㄤ簬杩斿洖鍊硷紝
渚嬪 `$retval->name`銆?

瀵逛簬杩欎簺 BTF 鍙傛暟鍜屽瓧娈碉紝`:string` 鍜?`:ustring` 浼氭敼鍙?
琛屼负銆傚鏋滃畠浠敤浜?BTF 鍙傛暟鎴栧瓧娈碉紝浼氭鏌ヨ鍙傛暟鎴栨暟鎹瓧娈电殑 BTF 绫诲瀷
鏄惁涓?`char *` 鎴?`char []`锛?
鑻ヤ笉鏄紝鍒欐嫆缁濆簲鐢ㄥ瓧绗︿覆绫诲瀷銆傛澶栵紝鍊熷姪 BTF
鏀寔锛屽湪璁块棶鐢?`PTR` 鎸囧悜鐨勫瓧绗︿覆鏃讹紝
鎮ㄤ笉鍐嶉渶瑕佸唴瀛樿В寮曠敤杩愮畻绗︼紙`+0(PTR)`锛夈€傚畠浼氳嚜鍔ㄦ坊鍔犲唴瀛?
```

```
# echo 't sched_switch prev->comm:string' >> dynamic_events
# echo 'f getname_flags%return $retval->name:string' >> dynamic_events

`prev->comm` 鏄暟鎹粨鏋勪腑鐨勫唴宓屽瓧绗︽暟缁勶紝鑰?
`$retval->name` 鏄暟鎹粨鏋勪腑鐨勫瓧绗︽寚閽堛€備絾鍦ㄤ袱绉?
鎯呭喌涓嬶紝閮藉彲浠ヤ娇鐢?`:string` 绫诲瀷鏉ヨ幏鍙栧瓧绗︿覆銆?


### 浣跨敤绀轰緥

涓嬮潰鏄竴涓湪 `vfs_read()` 鍑芥暟鐨勫叆鍙ｅ拰
鍑哄彛澶勬坊鍔犲甫 BTF 鍙傛暟鐨?fprobe 浜嬩欢鐨勭ず渚嬨€?
```

  # echo 'f vfs_read $arg*' >> dynamic_events
  # echo 'f vfs_read%return $retval' >> dynamic_events
  # cat dynamic_events
 f:fprobes/vfs_read__entry vfs_read file=file buf=buf count=count pos=pos
 f:fprobes/vfs_read__exit vfs_read%return arg1=$retval
  # echo 1 > events/fprobes/enable
  # head -n 20 trace | tail
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
               sh-70      [000] ...1.   335.883195: vfs_read__entry: (vfs_read+0x4/0x340) file=0xffff888005cf9a80 buf=0x7ffef36c6879 count=1 pos=0xffffc900005aff08
               sh-70      [000] .....   335.883208: vfs_read__exit: (ksys_read+0x75/0x100 <- vfs_read) arg1=1
               sh-70      [000] ...1.   335.883220: vfs_read__entry: (vfs_read+0x4/0x340) file=0xffff888005cf9a80 buf=0x7ffef36c6879 count=1 pos=0xffffc900005aff08
               sh-70      [000] .....   335.883224: vfs_read__exit: (ksys_read+0x75/0x100 <- vfs_read) arg1=1
               sh-70      [000] ...1.   335.883232: vfs_read__entry: (vfs_read+0x4/0x340) file=0xffff888005cf9a80 buf=0x7ffef36c687a count=1 pos=0xffffc900005aff08
               sh-70      [000] .....   335.883237: vfs_read__exit: (ksys_read+0x75/0x100 <- vfs_read) arg1=1
               sh-70      [000] ...1.   336.050329: vfs_read__entry: (vfs_read+0x4/0x340) file=0xffff888005cf9a80 buf=0x7ffef36c6879 count=1 pos=0xffffc900005aff08
               sh-70      [000] .....   336.050343: vfs_read__exit: (ksys_read+0x75/0x100 <- vfs_read) arg1=1

```
鍙互鐪嬪埌鎵€鏈夊嚱鏁板弬鏁板拰杩斿洖鍊奸兘琚褰曚负甯︾鍙锋暣鏁般€?

姝ゅ锛屼笅闈㈡槸涓€涓湪 `sched_switch` tracepoint 涓婄殑 tracepoint 浜嬩欢绀轰緥銆?
涓轰簡瀵规瘮缁撴灉锛岃繖閲屼篃鍚敤浜?`sched_switch` traceevent銆?
```

  # echo 't sched_switch $arg*' >> dynamic_events
  # echo 1 > events/sched/sched_switch/enable
  # echo 1 > events/tracepoints/sched_switch/enable
  # echo > trace
  # head -n 20 trace | tail
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
               sh-70      [000] d..2.  3912.083993: sched_switch: prev_comm=sh prev_pid=70 prev_prio=120 prev_state=S ==> next_comm=swapper/0 next_pid=0 next_prio=120
               sh-70      [000] d..3.  3912.083995: sched_switch: (__probestub_sched_switch+0x4/0x10) preempt=0 prev=0xffff88800664e100 next=0xffffffff828229c0 prev_state=1
           <idle>-0       [000] d..2.  3912.084183: sched_switch: prev_comm=swapper/0 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=rcu_preempt next_pid=16 next_prio=120
           <idle>-0       [000] d..3.  3912.084184: sched_switch: (__probestub_sched_switch+0x4/0x10) preempt=0 prev=0xffffffff828229c0 next=0xffff888004208000 prev_state=0
      rcu_preempt-16      [000] d..2.  3912.084196: sched_switch: prev_comm=rcu_preempt prev_pid=16 prev_prio=120 prev_state=I ==> next_comm=swapper/0 next_pid=0 next_prio=120
      rcu_preempt-16      [000] d..3.  3912.084196: sched_switch: (__probestub_sched_switch+0x4/0x10) preempt=0 prev=0xffff888004208000 next=0xffffffff828229c0 prev_state=1026
           <idle>-0       [000] d..2.  3912.085191: sched_switch: prev_comm=swapper/0 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=rcu_preempt next_pid=16 next_prio=120
           <idle>-0       [000] d..3.  3912.085191: sched_switch: (__probestub_sched_switch+0x4/0x10) preempt=0 prev=0xffffffff828229c0 next=0xffff888004208000 prev_state=0

```
濡傛偍鎵€瑙侊紝`sched_switch` trace-event 鏄剧ず鐨勬槸 **cooked**锛堝凡澶勭悊锛夊弬鏁帮紝鑰?
鍙︿竴鏂归潰锛宍sched_switch` tracepoint 鎺㈡祴浜嬩欢鏄剧ず鐨勬槸 **raw**锛堝師濮嬶級
鍙傛暟銆傝繖鎰忓懗鐫€鎮ㄥ彲浠ヨ闂?task
缁撴瀯浣撲腑鐢?`prev` 鍜?`next` 鍙傛暟鎸囧悜鐨勪换浣曞瓧娈靛€笺€?

**渚嬪锛岄€氬父 ``task_struct**
鐨?start_time`` 榛樿涓嶄細琚窡韪紝浣嗗€熷姪杩欎釜
traceprobe 浜嬩欢锛屾偍鍙互鍍忎笅闈㈣繖鏍疯窡韪瀛楁銆?
```

  # echo 't sched_switch comm=next->comm:string next->start_time' > dynamic_events
  # head -n 20 trace | tail
 #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
 #              | |         |   |||||     |         |
               sh-70      [000] d..3.  5606.686577: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="rcu_preempt" usage=1 start_time=245000000
      rcu_preempt-16      [000] d..3.  5606.686602: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="sh" usage=1 start_time=1596095526
               sh-70      [000] d..3.  5606.686637: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="swapper/0" usage=2 start_time=0
           <idle>-0       [000] d..3.  5606.687190: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="rcu_preempt" usage=1 start_time=245000000
      rcu_preempt-16      [000] d..3.  5606.687202: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="swapper/0" usage=2 start_time=0
           <idle>-0       [000] d..3.  5606.690317: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="kworker/0:1" usage=1 start_time=137000000
      kworker/0:1-14      [000] d..3.  5606.690339: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="swapper/0" usage=2 start_time=0
           <idle>-0       [000] d..3.  5606.692368: sched_switch: (__probestub_sched_switch+0x4/0x10) comm="kworker/0:1" usage=1 start_time=137000000

```

杩斿洖鎺㈡祴鍏佽鎴戜滑璁块棶鏌愪簺鍑芥暟鐨勮繑鍥炵粨鏋滐紝杩欎簺鍑芥暟杩斿洖
閿欒鐮侊紝涓斿叾缁撴灉閫氳繃鍑芥暟鍙傛暟浼犻€掞紝渚嬪涓€涓?
缁撴瀯浣撳垵濮嬪寲鍑芥暟銆?

渚嬪锛寁fs_open() 浼氬皢鏂囦欢缁撴瀯浣撻摼鎺ュ埌 inode 骞舵洿鏂?
妯″紡銆傛偍鍙互浣跨敤杩斿洖鎺㈡祴鏉ヨ窡韪繖浜涘彉鏇淬€?
```

 # echo 'f vfs_open mode=file->f_mode:x32 inode=file->f_inode:x64' >> dynamic_events
 # echo 'f vfs_open%%return mode=file->f_mode:x32 inode=file->f_inode:x64' >> dynamic_events
 # echo 1 > events/fprobes/enable
 # cat trace
              sh-131     [006] ...1.  1945.714346: vfs_open__entry: (vfs_open+0x4/0x40) mode=0x2 inode=0x0
              sh-131     [006] ...1.  1945.714358: vfs_open__exit: (do_open+0x274/0x3d0 <- vfs_open) mode=0x4d801e inode=0xffff888008470168
             cat-143     [007] ...1.  1945.717949: vfs_open__entry: (vfs_open+0x4/0x40) mode=0x1 inode=0x0
             cat-143     [007] ...1.  1945.717956: vfs_open__exit: (do_open+0x274/0x3d0 <- vfs_open) mode=0x4a801d inode=0xffff888005f78d28
             cat-143     [007] ...1.  1945.720616: vfs_open__entry: (vfs_open+0x4/0x40) mode=0x1 inode=0x0
             cat-143     [007] ...1.  1945.728263: vfs_open__exit: (do_open+0x274/0x3d0 <- vfs_open) mode=0xa800d inode=0xffff888004ada8d8

```
**鎮ㄥ彲浠ョ湅鍒?`file**
: f_mode` and `file::f_inode` are updated in `vfs_open()銆?

