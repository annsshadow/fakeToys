
## Fprobe - 鍑芥暟鍏ュ彛/鍑哄彛鎺㈤拡

## 绠€浠?
Fprobe 鏄竴绉嶅熀浜?ftrace 涓?function-graph 杩借釜鐗规€х殑鍑芥暟鍏ュ彛/鍑哄彛鎺㈤拡銆?濡傛灉浣犱笉鎯宠拷韪墍鏈夊嚱鏁帮紝鑰屾槸鎯冲湪鐗瑰畾鍑芥暟鐨勫叆鍙ｅ拰鍑哄彛闄勫姞鍥炶皟锛堢被浼间簬 kprobes 鍜?kretprobes锛夛紝鍙互浣跨敤 fprobe銆備笌 kprobes 鍜?kretprobes 鐩告瘮锛宖probe 閫氳繃鍗曚釜
澶勭悊鍑芥暟涓哄涓嚱鏁版彁渚涙洿蹇€熺殑妫€娴嬨€傛湰鏂囨。鎻忚堪濡備綍浣跨敤 fprobe銆?
## fprobe 鐨勭敤娉?
fprobe 鏄?ftrace锛堝姞涓婄被浼?kretprobe 鐨勮繑鍥炲洖璋冿級鐨勪竴涓皝瑁咃紝鐢ㄤ簬鍚戝涓嚱鏁扮殑
鍏ュ彛鍜屽嚭鍙ｉ檮鍔犲洖璋冦€傜敤鎴烽渶瑕佽缃ソ `struct fprobe` 骞跺皢鍏朵紶閫掔粰 `register_fprobe()`銆?
閫氬父锛宍fprobe` 鏁版嵁缁撴瀯浼氬儚涓嬮潰杩欐牱鐢?`entry_handler` 鍜?鎴?`exit_handler` 杩涜
鍒濆鍖栥€?
 struct fprobe fp = {
        .entry_handler  = my_entry_callback,
        .exit_handler   = my_exit_callback,
 };

瑕佸惎鐢?fprobe锛屽彲浠ヨ皟鐢?register_fprobe()銆乺egister_fprobe_ips() 鍜?register_fprobe_syms() 涓殑涓€涓€傝繖浜涘嚱鏁颁娇鐢ㄤ笉鍚岀被鍨嬬殑鍙傛暟鏉ユ敞鍐?fprobe銆?
register_fprobe() 閫氳繃鍑芥暟鍚嶈繃婊ゅ櫒鏉ュ惎鐢?fprobe銆?```

  register_fprobe(&fp, "func*", "func2");

```
register_fprobe_ips() 閫氳繃 ftrace 浣嶇疆鍦板潃鏉ュ惎鐢?fprobe銆備緥濡傦細


  unsigned long ips[] = { 0x.... };

  register_fprobe_ips(&fp, ips, ARRAY_SIZE(ips));

鑰?register_fprobe_syms() 閫氳繃绗﹀彿鍚嶆潵鍚敤 fprobe銆備緥濡傦細


  char syms[] = {"func1", "func2", "func3"};

  register_fprobe_syms(&fp, syms, ARRAY_SIZE(syms));

```

  unregister_fprobe(&fp);

```
```

  disable_fprobe(&fp);

```
```

  enable_fprobe(&fp);

```
```

  #include <linux/fprobe.h>

```
涓?ftrace 鐩稿悓锛屽凡娉ㄥ唽鐨勫洖璋冧細鍦?register_fprobe() 琚皟鐢ㄤ箣鍚庛€佽繑鍥炰箣鍓嶇殑鏌愪釜鏃跺埢
寮€濮嬭璋冪敤銆傚弬瑙?Documentation/trace/ftrace.rst銆?
姝ゅ锛寀nregister_fprobe() 浼氫繚璇侊紝鍦ㄥ畠杩斿洖涔嬪悗锛宔nter 鍜?exit 澶勭悊鍑芥暟閮戒笉鍐嶈
鍑芥暟璋冪敤锛岃繖涓?unregister_ftrace_function() 鐩稿悓銆?
## fprobe 鍏ュ彛/鍑哄彛澶勭悊鍑芥暟

鍏ュ彛/鍑哄彛鍥炶皟鍑芥暟鐨勫師鍨嬪涓嬶細

 int entry_callback(struct fprobe **fp, unsigned long entry_ip, unsigned long ret_ip, struct ftrace_regs **fregs, void *entry_data);

 void exit_callback(struct fprobe **fp, unsigned long entry_ip, unsigned long ret_ip, struct ftrace_regs **fregs, void *entry_data);

娉ㄦ剰锛孈entry_ip 鍦ㄥ嚱鏁板叆鍙ｅ琚繚瀛橈紝骞朵紶閫掔粰 exit 澶勭悊鍑芥暟銆?濡傛灉鍏ュ彛鍥炶皟鍑芥暟杩斿洖 !0锛屽垯鐩稿簲鐨?exit 鍥炶皟灏嗚鍙栨秷銆?
@fp
        杩欐槸涓庢澶勭悊鍑芥暟鐩稿叧鐨?`fprobe` 鏁版嵁缁撴瀯鐨勫湴鍧€銆?        浣犲彲浠ュ皢 `fprobe` 宓屽叆鍒拌嚜宸辩殑鏁版嵁缁撴瀯涓紝骞堕€氳繃 container_of() 瀹忎粠
        @fp 鑾峰彇瀹冦€侤fp 缁濅笉鑳戒负 NULL銆?
@entry_ip
        杩欐槸琚拷韪嚱鏁扮殑 ftrace 鍦板潃锛堝叆鍙ｅ拰鍑哄彛閮芥槸锛夈€傛敞鎰忥紝杩欏彲鑳戒笉鏄嚱鏁扮殑
        瀹為檯鍏ュ彛鍦板潃锛岃€屾槸 ftrace 杩涜妫€娴嬬殑浣嶇疆鍦板潃銆?
@ret_ip
        杩欐槸琚拷韪嚱鏁板皢杩斿洖鍒扮殑鍦板潃锛屼綅浜庤皟鐢ㄨ€呮煇澶勩€傚畠鍙互鍦ㄥ叆鍙ｅ拰鍑哄彛澶勯兘浣跨敤銆?
@fregs
        杩欐槸鍏ュ彛鍜屽嚭鍙ｅ鐨?`ftrace_regs` 鏁版嵁缁撴瀯銆傚畠鍖呭惈鍑芥暟鍙傛暟鎴栬繑鍥炲€笺€傚洜姝?        鐢ㄦ埛鍙互閫氳繃閫傚綋鐨?`ftrace_regs_*` API 鏉ヨ闂繖浜涘€笺€?
@entry_data
        杩欐槸涓€涓敤浜庡湪鍏ュ彛鍜屽嚭鍙ｅ鐞嗗嚱鏁颁箣闂村叡浜暟鎹殑鏈湴瀛樺偍銆傞粯璁ゆ儏鍐典笅璇ュ瓨鍌?        涓?NULL銆傚鏋滅敤鎴峰湪娉ㄥ唽 fprobe 鏃舵寚瀹氫簡 `exit_handler` 瀛楁鍜?`entry_data_size`
        瀛楁锛屽垯浼氬垎閰嶈瀛樺偍锛屽苟浼犻€掔粰 `entry_handler` 鍜?`exit_handler`銆?
## 鍏ュ彛鏁版嵁澶у皬涓庡悓涓€鍑芥暟涓婄殑鍑哄彛澶勭悊鍑芥暟

鐢变簬鍏ュ彛鏁版嵁鏄€氳繃姣忎换鍔℃爤浼犻€掔殑锛屼笖澶у皬鏈夐檺锛屾瘡涓帰閽堢殑鍏ュ彛鏁版嵁澶у皬琚檺鍒朵负
`15 * sizeof(long)`銆備綘杩橀渶瑕佹敞鎰忥紝褰撲笉鍚岀殑 fprobe 鎺㈡祴鍚屼竴涓嚱鏁版椂锛岃繖涓檺鍒朵細
鍙樺緱鏇村皬銆傚叆鍙ｆ暟鎹ぇ灏忔寜 `sizeof(long)` 瀵归綈锛屾瘡涓甫鏈?exit 澶勭悊鍑芥暟鐨?fprobe
浼氬湪鏍堜笂浣跨敤 `sizeof(long)` 澶у皬鐨勭┖闂达紝鍥犳浣犲簲璁╁悓涓€涓嚱鏁颁笂鐨?fprobe 鏁伴噺
灏藉彲鑳藉皯銆?
## 涓?kprobes 鍏变韩鍥炶皟

鐢变簬 fprobe锛堝拰 ftrace锛夌殑閫掑綊瀹夊叏鎬т笌 kprobes 鐣ユ湁涓嶅悓锛屽鏋滅敤鎴峰笇鏈涗粠 fprobe
鍜?kprobes 杩愯鐩稿悓鐨勪唬鐮侊紝杩欏彲鑳戒細寮曞彂闂銆?
Kprobes 鏈変竴涓瘡 CPU 鐨?'current_kprobe' 鍙橀噺锛屽畠鍦ㄦ墍鏈夋儏鍐典笅閮戒繚鎶?kprobe 澶勭悊
鍑芥暟鍏嶅彈閫掑綊銆傚彟涓€鏂归潰锛宖probe 鍙娇鐢?ftrace_test_recursion_trylock()銆傝繖鍏佽鍦?fprobe 鐢ㄦ埛澶勭悊鍑芥暟杩愯鏃讹紝涓柇涓婁笅鏂囪皟鐢ㄥ彟涓€涓紙鎴栧悓涓€涓級fprobe銆?
濡傛灉鍏叡鍥炶皟浠ｇ爜鑷韩鍏锋湁閫掑綊妫€娴嬶紝鎴栬€呰兘澶熷鐞嗕笉鍚屼笂涓嬫枃锛堟櫘閫?涓柇/NMI锛変腑鐨?閫掑綊锛岃繖灏变笉鏄棶棰樸€備絾濡傛灉瀹冧緷璧栦簬 'current_kprobe' 閫掑綊閿侊紝鍒欏繀椤绘鏌?kprobe_running() 骞朵娇鐢?kprobe_busy_*() API銆?
Fprobe 鎻愪緵浜?FPROBE_FL_KPROBE_SHARED 鏍囧織鏉ュ疄鐜拌繖涓€鐐广€傚鏋滀綘鐨勫叕鍏卞洖璋冧唬鐮佸皢
涓?kprobes 鍏变韩锛岃鍦ㄦ敞鍐?fprobe **涔嬪墠**璁剧疆 FPROBE_FL_KPROBE_SHARED锛屼緥濡傦細


 fprobe.flags = FPROBE_FL_KPROBE_SHARED;

 register_fprobe(&fprobe, "func*", NULL);

杩欏皢淇濇姢浣犵殑鍏叡鍥炶皟鍏嶅彈宓屽璋冪敤銆?
## 鏈懡涓鏁板櫒

**`fprobe`** 鏁版嵁缁撴瀯鎷ユ湁涓?kprobes 鐩稿悓鐨?`: nmissed` 璁℃暟鍣ㄥ瓧娈点€?褰撲互涓嬫儏鍐靛彂鐢熸椂锛岃璁℃暟鍣ㄤ細閫掑锛?
 - fprobe 鏈兘鑾峰彇 ftrace_recursion 閿併€傝繖閫氬父鎰忓懗鐫€浠?entry_handler 涓皟鐢ㄤ簡
   琚叾浠?ftrace 鐢ㄦ埛杩借釜鐨勫嚱鏁般€?
 - fprobe 鐢变簬鏃犳硶浠庢瘡浠诲姟褰卞瓙鏍堝垎閰嶆暟鎹紦鍐插尯锛岃€屾湭鑳借缃嚱鏁板嚭鍙ｃ€?
**`fprobe`** 鐨?`: nmissed` 瀛楁鍦ㄤ笂杩颁袱绉嶆儏鍐典笅閮戒細閫掑銆傚洜姝わ紝鍓嶈€呬細璺宠繃鍏ュ彛鍜?鍑哄彛鍥炶皟锛屽悗鑰呬細璺宠繃鍑哄彛鍥炶皟锛屼絾鍦ㄤ袱绉嶆儏鍐典笅璁℃暟鍣ㄩ兘浼氬姞 1銆?
娉ㄦ剰锛屽鏋滀綘鍦ㄦ敞鍐?fprobe 鏃跺皢 FTRACE_OPS_FL_RECURSION 鍜?鎴?FTRACE_OPS_FL_RCU
璁剧疆鍒?**`fprobe`** 鐨?`ops::flags`锛坒trace_ops::flags锛夛紝璇ヨ鏁板櫒鍙兘鏃犳硶姝ｇ‘
宸ヤ綔锛屽洜涓?ftrace 浼氳烦杩囩敤浜庨€掑璇ヨ鏁板櫒鐨?fprobe 鍑芥暟銆?
## 鍑芥暟涓庣粨鏋勪綋
