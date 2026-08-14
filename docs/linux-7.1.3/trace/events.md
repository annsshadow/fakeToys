## 浜嬩欢璺熻釜


:Author: Theodore Ts'o
:Updated: Li Zefan and Tom Zanussi

## 1. 绠€浠?


璺熻釜鐐癸紙tracepoints锛岃 Documentation/trace/tracepoints.rst锛夊彲浠ュ湪涓嶅垱寤?
鑷畾涔夊唴鏍告ā鍧楃殑鎯呭喌涓嬩娇鐢紝閫氳繃浜嬩欢璺熻釜锛坋vent tracing锛夊熀纭€璁炬柦鏉ユ敞鍐?
鎺㈡祴锛坧robe锛夊嚱鏁般€?

骞堕潪鎵€鏈夎窡韪偣閮借兘閫氳繃浜嬩欢璺熻釜绯荤粺杩涜璺熻釜锛涘唴鏍稿紑鍙戣€呭繀椤绘彁渚涗唬鐮佺墖娈碉紝
瀹氫箟璺熻釜淇℃伅濡備綍淇濆瓨鍒拌窡韪紦鍐插尯锛坱racing buffer锛夛紝浠ュ強濡備綍鎵撳嵃杩欎簺
璺熻釜淇℃伅銆?

## 2. 浣跨敤浜嬩欢璺熻釜


### 2.1 閫氳繃 'set_event' 鎺ュ彛


鍙敤浜庤窡韪殑浜嬩欢鍙湪 /sys/kernel/tracing/available_events 鏂囦欢涓壘鍒般€?

瑕佸惎鐢ㄦ煇涓壒瀹氫簨浠讹紙渚嬪 'sched_wakeup'锛夛紝鍙渶灏嗗叾 echo 鍐欏叆锛?

```
	# echo sched_wakeup >> /sys/kernel/tracing/set_event
```

瑕佺鐢ㄦ煇涓簨浠讹紝灏嗕簨浠跺悕鍔犱笂鍓嶇紑鍚?echo 鍐欏叆 set_event 鏂囦欢锛?

```
	# echo '!sched_wakeup' >> /sys/kernel/tracing/set_event
```

```
	# echo > /sys/kernel/tracing/set_event
```

```
	# echo *:* > /sys/kernel/tracing/set_event
```

杩欎簺浜嬩欢琚粍缁囧埌鍚勪釜瀛愮郴缁熶腑锛屼緥濡?ext4銆乮rq銆乻ched 绛夛紝涓€涓畬鏁寸殑浜嬩欢鍚?
褰㈠锛?subsystem>:<event>銆傚瓙绯荤粺鍚嶆槸鍙€夌殑锛屼絾浼氭樉绀哄湪 available_events
鏂囦欢涓€傛煇涓瓙绯荤粺涓殑鎵€鏈変簨浠跺彲浠ラ€氳繃 `<subsystem>:*` 璇硶鏉ユ寚瀹氾紱渚嬪锛?
瑕佸惎鐢ㄦ墍鏈?irq 浜嬩欢锛屽彲浠ヤ娇鐢細

```
	# echo 'irq:*' > /sys/kernel/tracing/set_event
```

set_event 鏂囦欢涔熷彲鐢ㄤ簬鍚敤浠呬笌鏌愪釜妯″潡鍏宠仈鐨勪簨浠讹細

```
	# echo ':mod:<module>' > /sys/kernel/tracing/set_event
```

杩欏皢鍚敤妯″潡 `<module>` 涓殑鎵€鏈変簨浠躲€傚鏋滆妯″潡灏氭湭鍔犺浇锛岃繖涓瓧绗︿覆浼氳
淇濆瓨涓嬫潵锛屽綋鍔犺浇鍒颁笌涔嬪尮閰嶇殑妯″潡 `<module>` 鏃讹紝灏变細搴旂敤浜嬩欢鐨勫惎鐢ㄨ缃€?

`:mod:` 涔嬪墠鐨勬枃鏈細琚В鏋愶紝鐢ㄤ互鎸囧畾璇ユā鍧椾腑闇€瑕佸惎鐢ㄧ殑鍏蜂綋浜嬩欢锛?

```
	# echo '<match>:mod:<module>' > /sys/kernel/tracing/set_event
```

涓婅堪鍛戒护灏嗗惎鐢ㄤ换浣曚笌 `<match>` 鍖归厤鐨勭郴缁熸垨浜嬩欢銆傚鏋?`<match>` 涓?`"*"`锛?
鍒欎細鍖归厤鎵€鏈変簨浠躲€?

```
	# echo '<system>:<event>:mod:<module>' > /sys/kernel/tracing/set_event
```

濡傛灉 `<event>` 涓?`"*"`锛屽垯浼氬尮閰嶈妯″潡缁欏畾绯荤粺涓殑鎵€鏈変簨浠躲€?

### 2.2 閫氳繃 'enable' 寮€鍏?


鍙敤鐨勪簨浠朵篃浠ョ洰褰曞眰绾х殑褰㈠紡鍒楀湪 /sys/kernel/tracing/events/ 涓嬨€?

```
	# echo 1 > /sys/kernel/tracing/events/sched/sched_wakeup/enable
```

```
	# echo 0 > /sys/kernel/tracing/events/sched/sched_wakeup/enable
```

```
	# echo 1 > /sys/kernel/tracing/events/sched/enable
```

```
	# echo 1 > /sys/kernel/tracing/events/enable
```

璇诲彇杩欎簺 enable 鏂囦欢鏃讹紝浼氭湁鍥涚缁撴灉锛?

 - 0 - 璇ユ枃浠跺奖鍝嶇殑鎵€鏈変簨浠堕兘宸茬鐢?
 - 1 - 璇ユ枃浠跺奖鍝嶇殑鎵€鏈変簨浠堕兘宸插惎鐢?
 - X - 鍚敤鍜岀鐢ㄧ殑浜嬩欢娣峰悎瀛樺湪
 - ? - 璇ユ枃浠朵笉褰卞搷浠讳綍浜嬩欢

### 2.3 鍚姩鍙傛暟


```
	trace_event=[event-list]
```

event-list 鏄竴涓互閫楀彿鍒嗛殧鐨勪簨浠跺垪琛ㄣ€備簨浠舵牸寮忚绗?2.1 鑺傘€?

## 3. 瀹氫箟涓€涓惎鐢ㄤ簨浠惰窡韪殑璺熻釜鐐?


鍙傝 samples/trace_events 涓彁渚涚殑绀轰緥銆?

## 4. 浜嬩欢鏍煎紡


姣忎釜璺熻釜浜嬩欢閮芥湁涓€涓叧鑱旂殑 'format' 鏂囦欢锛屽叾涓寘鍚鏃ュ織璁板綍浜嬩欢涓瘡涓瓧娈?
鐨勬弿杩般€傝繖浜涗俊鎭彲鐢ㄤ簬瑙ｆ瀽浜岃繘鍒剁殑璺熻釜娴侊紝鍚屾椂涔熸槸鏌ユ壘鍙敤浜庝簨浠惰繃婊ゅ櫒
锛堣绗?5 鑺傦級鐨勫瓧娈靛悕鐨勫湴鏂广€?

瀹冭繕鏄剧ず浜嗙敤浜庝互鏂囨湰妯″紡鎵撳嵃浜嬩欢鐨勬牸寮忓瓧绗︿覆锛屼互鍙婄敤浜庢€ц兘鍒嗘瀽鐨?
浜嬩欢鍚嶅拰 ID銆?

姣忎釜浜嬩欢閮芥湁涓€缁勪笌涔嬪叧鑱旂殑 `common` 瀛楁锛涜繖浜涙槸浠?`common_` 涓哄墠缂€鐨勫瓧娈点€?
鍏朵粬瀛楁鍦ㄤ笉鍚岀殑浜嬩欢闂存湁鎵€涓嶅悓锛屽搴斾簬璇ヤ簨浠跺湪 TRACE_EVENT 瀹氫箟涓０鏄庣殑瀛楁銆?

```
     field:field-type field-name; offset:N; size:N;
```

鍏朵腑 offset 鏄瓧娈靛湪璺熻釜璁板綍涓殑鍋忕Щ锛宻ize 鏄暟鎹」鐨勫ぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€?

渚嬪锛屼笅闈㈡槸 'sched_wakeup' 浜嬩欢鎵€鏄剧ず鐨勪俊鎭細

```
	# cat /sys/kernel/tracing/events/sched/sched_wakeup/format

	name: sched_wakeup
	ID: 60
	format:
		field:unsigned short common_type;	offset:0;	size:2;
		field:unsigned char common_flags;	offset:2;	size:1;
		field:unsigned char common_preempt_count;	offset:3;	size:1;
		field:int common_pid;	offset:4;	size:4;
		field:int common_tgid;	offset:8;	size:4;

		field:char comm[TASK_COMM_LEN];	offset:12;	size:16;
		field:pid_t pid;	offset:28;	size:4;
		field:int prio;	offset:32;	size:4;
		field:int success;	offset:36;	size:4;
		field:int cpu;	offset:40;	size:4;

	print fmt: "task %s:%d [%d] success=%d [%03d]", REC->comm, REC->pid,
		   REC->prio, REC->success, REC->cpu
```

璇ヤ簨浠跺寘鍚?10 涓瓧娈碉紝鍓?5 涓槸閫氱敤瀛楁锛屽叾浣?5 涓槸浜嬩欢鐗规湁鐨勫瓧娈点€傞櫎 'comm'
锛堜竴涓瓧绗︿覆锛夊锛岃浜嬩欢鐨勬墍鏈夊瓧娈甸兘鏄暟瀛楃被鍨嬶紝杩欎竴鍖哄埆鍦ㄨ繘琛屼簨浠惰繃婊ゆ椂
寰堥噸瑕併€?

## 5. 浜嬩欢杩囨护


璺熻釜浜嬩欢鍙互閫氳繃鍦ㄥ唴鏍镐腑涓哄叾鍏宠仈甯冨皵鍨嬧€滆繃婊よ〃杈惧紡鈥濇潵杩涜杩囨护銆備竴鏃︽煇涓簨浠?
琚褰曞埌璺熻釜缂撳啿鍖猴紝灏变細鐢ㄨ浜嬩欢绫诲瀷鍏宠仈鐨勮繃婊よ〃杈惧紡妫€鏌ュ叾瀛楁銆傚瓧娈靛€?
鈥滃尮閰嶁€濊繃婊ゅ櫒鐨勪簨浠朵細鏄剧ず鍦ㄨ窡韪緭鍑轰腑锛岃€屼笉鍖归厤鐨勪簨浠跺皢琚涪寮冦€傛病鏈夊叧鑱?
杩囨护鍣ㄧ殑浜嬩欢浼氬尮閰嶆墍鏈夊唴瀹癸紝杩欎篃鏄煇涓簨浠跺皻鏈缃繃婊ゅ櫒鏃剁殑榛樿琛屼负銆?

### 5.1 琛ㄨ揪寮忚娉?


涓€涓繃婊よ〃杈惧紡鐢变竴涓垨澶氫釜鈥滆皳璇嶁€濓紙predicate锛夌粍鎴愶紝鍙互浣跨敤閫昏緫杩愮畻绗?
'&&' 鍜?'||' 杩涜缁勫悎銆傝皳璇嶅氨鏄竴涓畝鍗曠殑瀛愬彞锛屽畠灏嗘棩蹇楄褰曚簨浠朵腑鍖呭惈鐨?
鏌愪釜瀛楁鐨勫€间笌甯搁噺鍊艰繘琛屾瘮杈冿紝骞舵牴鎹粨鏋滆繑鍥?0 鎴?1锛?

```
	  field-name relational-operator value
```

鍙互浣跨敤鎷彿鏉ユ彁渚涗换鎰忕殑閫昏緫鍒嗙粍锛屽弻寮曞彿鍙敤浜庨槻姝?shell 灏嗚繍绠楃瑙ｉ噴涓?
shell 鍏冨瓧绗︺€?

鍙敤浜庤繃婊ょ殑瀛楁鍚嶅彲浠ュ湪璺熻釜浜嬩欢鐨?'format' 鏂囦欢涓壘鍒帮紙瑙佺 4 鑺傦級銆?

鍏崇郴杩愮畻绗﹀彇鍐充簬琚祴璇曞瓧娈电殑绫诲瀷锛?

鏁板瓧瀛楁鍙敤鐨勮繍绠楃涓猴細

==, !=, <, <=, >, >=, &

鑰屽瓧绗︿覆瀛楁鍙敤鐨勮繍绠楃涓猴細

==, !=, ~

glob锛垀锛夋帴鍙楅€氶厤绗︼紙\*,?锛夊拰瀛楃绫伙細

```
  prev_comm ~ "*sh"
  prev_comm ~ "sh*"
  prev_comm ~ "*sh*"
  prev_comm ~ "ba*sh"
```

濡傛灉璇ュ瓧娈垫槸涓€涓寚鍚戠敤鎴风┖闂达紙user space锛夌殑鎸囬拡锛堜緥濡傛潵鑷?sys_enter_openat
鐨?"filename"锛夛紝鍒欏繀椤诲湪鍏跺悗闄勫姞 ".ustring"锛?

```
  filename.ustring ~ "password"
```

鍥犱负鍐呮牳闇€瑕佺煡閬撳浣曚粠璇ユ寚閽堟墍鎸囧悜鐨勭敤鎴风┖闂村唴瀛樹腑鑾峰彇鏁版嵁銆?

```
  call_site.function == security_prepare_creds
```

涓婅堪杩囨护浼氬湪瀛楁 "call_site" 钀藉湪 "security_prepare_creds" 鍑芥暟鍦板潃鑼冨洿鍐呮椂
鐢熸晥銆備篃灏辨槸璇达紝瀹冧細姣旇緝 "call_site" 鐨勫€硷紝濡傛灉瀹冨ぇ浜庢垨绛変簬璇ュ嚱鏁拌捣濮嬪湴鍧€
涓斿皬浜庤鍑芥暟缁撴潫鍦板潃锛屽垯杩囨护杩斿洖鐪熴€?

".function" 鍚庣紑鍙兘闄勫姞鍒板ぇ灏忎负 long 鐨勫€间笂锛屽苟涓斿彧鑳戒笌 "==" 鎴?"!=" 杩涜姣旇緝銆?

Cpumask 瀛楁鎴栫紪鐮佷簡 CPU 缂栧彿鐨勬爣閲忓瓧娈靛彲浠ヤ娇鐢ㄤ互涓嬫柟寮忚繘琛岃繃婊わ細

```
  CPUS{$cpulist}
```

鐢ㄤ簬 cpumask 杩囨护鐨勮繍绠楃鏈夛細

&锛堜氦闆嗭級, ==, !=

渚嬪锛岃繖灏嗚繃婊ゆ帀 .target_cpu 瀛楁瀛樺湪浜庝互涓嬪垪琛ㄤ腑鐨勪簨浠讹細

```
  target_cpu & CPUS{17-42}
```

### 5.2 璁剧疆杩囨护鍣?


鍗曚釜浜嬩欢鐨勮繃婊ゅ櫒鏄€氳繃灏嗚繃婊よ〃杈惧紡鍐欏叆璇ヤ簨浠剁殑 'filter' 鏂囦欢鏉ヨ缃殑銆?

```
	# cd /sys/kernel/tracing/events/sched/sched_wakeup
	# echo "common_preempt_count > 4" > filter
```

```
	# cd /sys/kernel/tracing/events/signal/signal_generate
	# echo "((sig >= 10 && sig < 15) || sig == 17) && comm != bash" > filter
```

濡傛灉琛ㄨ揪寮忎腑瀛樺湪閿欒锛屽湪璁剧疆鏃朵細寰楀埌鈥淚nvalid argument鈥濋敊璇紝骞朵笖閿欒鐨?
瀛楃涓蹭細杩炲悓锛?

```
	# cd /sys/kernel/tracing/events/signal/signal_generate
	# echo "((sig >= 10 && sig < 15) || dsig == 17) && comm != bash" > filter
	-bash: echo: write error: Invalid argument
	# cat filter
	((sig >= 10 && sig < 15) || dsig == 17) && comm != bash
	^
	parse_error: Field not found
```

鐩墠閿欒浣嶇疆鐨勮劚瀛楃锛?^'锛夋€绘槸鍑虹幇鍦ㄨ繃婊ゅ瓧绗︿覆鐨勫紑澶达紱涓嶈繃鍗充究娌℃湁鏇寸簿纭殑
浣嶇疆淇℃伅锛岄敊璇秷鎭粛搴斿叿鏈夊弬鑰冧环鍊笺€?

### 5.2.1 杩囨护鍣ㄩ檺鍒?


濡傛灉杩囨护鍣ㄨ鏀剧疆鍦ㄥ瓧绗︿覆鎸囬拡 `(char *)` 涓婏紝鑰岃鎸囬拡骞朵笉鎸囧悜鐜舰缂撳啿鍖?
锛坮ing buffer锛変腑鐨勫瓧绗︿覆锛岃€屾槸鎸囧悜鍐呮牳鎴栫敤鎴风┖闂村唴瀛橈紝閭ｄ箞鍑轰簬瀹夊叏鍘熷洜锛?
鏈€澶氫細灏?1024 瀛楄妭鐨勫唴瀹瑰鍒跺埌涓存椂缂撳啿鍖轰腑杩涜姣旇緝銆傚鏋滃唴瀛樺鍒舵椂鍙戠敓
缂洪〉锛堟寚閽堟寚鍚戜笉搴旇璁块棶鐨勫唴瀛橈級锛屽垯璇ュ瓧绗︿覆姣旇緝灏嗚瑙嗕负涓嶅尮閰嶃€?

### 5.3 娓呴櫎杩囨护鍣?


瑕佹竻闄ゆ煇涓簨浠剁殑杩囨护鍣紝鍚戣浜嬩欢鐨?filter 鏂囦欢鍐欏叆 '0'銆?

瑕佹竻闄ゆ煇涓瓙绯荤粺涓墍鏈変簨浠剁殑杩囨护鍣紝鍚戣瀛愮郴缁熺殑 filter 鏂囦欢鍐欏叆 '0'銆?

### 5.4 瀛愮郴缁熻繃婊ゅ櫒


涓烘柟渚胯捣瑙侊紝鍙互閫氳繃鍚戝瓙绯荤粺鏍圭洰褰曚笅鐨?filter 鏂囦欢鍐欏叆杩囨护琛ㄨ揪寮忥紝灏嗗瓙绯荤粺涓?
姣忎釜浜嬩欢鐨勮繃婊ゅ櫒浣滀负涓€涓暣浣撹繘琛岃缃垨娓呴櫎銆備絾瑕佹敞鎰忥紝濡傛灉瀛愮郴缁熶腑浠讳綍
浜嬩欢鐨勮繃婊ゅ櫒缂哄皯瀛愮郴缁熻繃婊ゅ櫒涓寚瀹氱殑瀛楁锛屾垨鑰呯敱浜庝换浣曞叾浠栧師鍥犳棤娉曞簲鐢ㄨ
杩囨护鍣紝璇ヤ簨浠剁殑杩囨护鍣ㄥ皢淇濈暀鍏朵箣鍓嶇殑璁剧疆銆傝繖鍙兘瀵艰嚧鍑虹幇鎰忔枡涔嬪鐨勮繃婊ゅ櫒
娣峰悎锛岃繘鑰屼骇鐢熶护浜哄洶鎯戯紙瀵瑰彲鑳戒互涓哄簲鐢ㄤ簡涓嶅悓杩囨护鍣ㄧ殑鐢ㄦ埛鑰岃█锛夌殑璺熻釜杈撳嚭銆?
鍙湁寮曠敤浜嗕粎閫氱敤锛坈ommon锛夊瓧娈电殑杩囨护鍣紝鎵嶈兘淇濊瘉鎴愬姛浼犳挱鍒版墍鏈変簨浠躲€?

浠ヤ笅鏄涓婅堪鍑犵偣杩涜璇存槑鐨勫嚑涓瓙绯荤粺杩囨护鍣ㄧず渚嬶細

```
	# cd /sys/kernel/tracing/events/sched
	# echo 0 > filter
	# cat sched_switch/filter
	none
	# cat sched_wakeup/filter
	none
```

浣跨敤浠呭寘鍚€氱敤瀛楁鐨勮繃婊ゅ櫒鏉ヨ缃?sched 瀛愮郴缁熶腑鎵€鏈変簨浠剁殑杩囨护鍣細

```
	# cd /sys/kernel/tracing/events/sched
	# echo common_pid == 0 > filter
	# cat sched_switch/filter
	common_pid == 0
	# cat sched_wakeup/filter
	common_pid == 0
```

灏濊瘯浣跨敤闈為€氱敤瀛楁涓?sched 瀛愮郴缁熶腑鎵€鏈変簨浠惰缃繃婊ゅ櫒锛堥櫎鍏锋湁 prev_pid
瀛楁鐨勪簨浠跺锛屽叾浣欎簨浠堕兘淇濈暀浜嗭細

```
	# cd /sys/kernel/tracing/events/sched
	# echo prev_pid == 0 > filter
	# cat sched_switch/filter
	prev_pid == 0
	# cat sched_wakeup/filter
	common_pid == 0
```

### 5.5 PID 杩囨护


鍦ㄤ笌椤剁骇 events 鐩綍鍚岀骇鐨勭洰褰曚笅锛屽瓨鍦ㄤ竴涓?set_event_pid 鏂囦欢锛屽畠浼氳繃婊ゆ帀
鎵€鏈?PID 鏈垪鍦?set_event_pid 鏂囦欢涓殑浠诲姟鐨勪簨浠讹細

```
	# cd /sys/kernel/tracing
	# echo $$ > set_event_pid
	# echo 1 > events/enable
```

杩欏皢鍙窡韪綋鍓嶄换鍔＄殑浜嬩欢銆?

瑕佸湪涓嶄涪澶卞凡鍖呭惈 PID 鐨勬儏鍐典笅娣诲姞鏇村 PID锛屼娇鐢?'>>'锛?

```
	# echo 123 244 1 >> set_event_pid
```

## 6. 浜嬩欢瑙﹀彂鍣?


璺熻釜浜嬩欢鍙互琚缃负鏈夋潯浠跺湴璋冪敤瑙﹀彂鍣ㄢ€滃懡浠も€濓紙trigger 'commands'锛夛紝杩欎簺鍛戒护
鏈夊绉嶅舰寮忥紝涓嬫枃灏嗚缁嗘弿杩帮紱渚嬪鍙互鏄瘡褰撳懡涓璺熻釜浜嬩欢鏃讹紝鍚敤鎴栫鐢?
鍏朵粬璺熻釜浜嬩欢锛屾垨鑰呰皟鐢ㄦ爤鍥炴函锛坰tack trace锛夈€傛瘡褰撹皟鐢ㄥ甫鏈夐檮鍔犺Е鍙戝櫒鐨?
璺熻釜浜嬩欢鏃讹紝灏变細璋冪敤涓庤浜嬩欢鍏宠仈鐨勯偅缁勮Е鍙戝櫒鍛戒护銆備换浣曠粰瀹氱殑瑙﹀彂鍣ㄨ繕鍙互
鏈変竴涓笌绗?5 鑺傦紙浜嬩欢杩囨护锛夋弿杩板舰寮忕浉鍚岀殑浜嬩欢杩囨护鍣ㄤ笌涔嬪叧鑱斺€斺€斿彧鏈夊綋琚皟鐢?
鐨勪簨浠堕€氳繃浜嗗叧鑱旂殑杩囨护鍣ㄦ椂锛岃鍛戒护鎵嶄細琚皟鐢ㄣ€傚鏋滄病鏈変笌瑙﹀彂鍣ㄥ叧鑱旂殑杩囨护鍣紝
鍒欐€绘槸閫氳繃銆?

瑙﹀彂鍣ㄦ槸閫氳繃灏嗚Е鍙戝櫒琛ㄨ揪寮忓啓鍏ョ粰瀹氫簨浠剁殑 'trigger' 鏂囦欢鏉ユ坊鍔犲拰绉婚櫎鐨勩€?

涓€涓粰瀹氱殑浜嬩欢鍙互鍏宠仈浠绘剰鏁伴噺鐨勮Е鍙戝櫒锛屼絾闇€閬靛畧鍚勪釜鍛戒护鍦ㄨ繖鏂归潰鍙兘鏈夌殑
浠讳綍闄愬埗銆?

浜嬩欢瑙﹀彂鍣ㄥ缓绔嬪湪鈥滆蒋鈥濓紙soft锛夋ā寮忎箣涓婏紝杩欐剰鍛崇潃姣忓綋鏌愪釜璺熻釜浜嬩欢鍏宠仈浜嗕竴涓?
鎴栧涓Е鍙戝櫒鏃讹紝鍗充娇璇ヤ簨浠跺疄闄呬笂骞舵湭琚惎鐢紝瀹冧篃浼氳婵€娲伙紝浣嗗浜庘€滆蒋鈥濇ā寮忎笅
琚鐢ㄣ€備篃灏辨槸璇达紝璺熻釜鐐逛細琚皟鐢紝浣嗕笉浼氬疄闄呰璺熻釜锛岄櫎闈炲畠纭疄琚惎鐢ㄤ簡銆?
杩欎竴鏈哄埗浣垮緱鍗充娇瀵逛簬鏈惎鐢ㄧ殑浜嬩欢涔熻兘璋冪敤瑙﹀彂鍣紝鍚屾椂涔熶娇寰楀綋鍓嶇殑浜嬩欢杩囨护鍣?
瀹炵幇鍙敤浜庢湁鏉′欢鍦拌皟鐢ㄨЕ鍙戝櫒銆?

浜嬩欢瑙﹀彂鍣ㄧ殑璇硶澶ц嚧鍩轰簬 set_ftrace_filter 鐨勨€渇trace 杩囨护鍣ㄥ懡浠も€濊娉曪紙瑙?
Documentation/trace/ftrace.rst 鐨勨€滆繃婊ゅ櫒鍛戒护鈥濅竴鑺傦級锛屼絾涓よ€呭瓨鍦ㄩ噸澶у樊寮傦紝
涓旂洰鍓嶇殑瀹炵幇骞舵湭浠ヤ换浣曟柟寮忎笌涔嬬粦瀹氾紝鍥犳涓嶈瀵逛簩鑰呭鍔犵被姣斻€?

     鍐欏叆 trace_marker锛堣 Documentation/trace/ftrace.rst锛?
     涔熷彲浠ュ惎鐢ㄥ啓鍏?
     /sys/kernel/tracing/events/ftrace/print/trigger 鐨勮Е鍙戝櫒

### 6.1 琛ㄨ揪寮忚娉?


```
  # echo 'command[:count] [if filter]' > trigger
```

瑙﹀彂鍣ㄩ€氳繃 echo 鐩稿悓鐨勫懡浠や絾浠ュ墠瀵肩殑 '!' 寮€澶存潵绉婚櫎锛?

```
  # echo '!command[:count] [if filter]' > trigger
```

鍦ㄧЩ闄ゆ椂锛孾if filter] 閮ㄥ垎涓嶅弬涓庡懡浠ゅ尮閰嶏紝鍥犳鍦ㄤ娇鐢?'!' 鍛戒护鏃剁渷鐣ュ畠
涓庡寘鍚畠鐨勬晥鏋滅浉鍚屻€?

杩囨护鍣ㄨ娉曚笌涓婅堪鈥滀簨浠惰繃婊も€濅竴鑺備腑鎻忚堪鐨勭浉鍚屻€?

涓烘柟渚胯捣瑙侊紝鐩墠浣跨敤 '>' 鍐欏叆 trigger 鏂囦欢鍙槸娣诲姞鎴栫Щ闄ゅ崟涓Е鍙戝櫒锛屽苟涓?
鏄惧紡鏀寔 '>>'锛?>' 瀹為檯涓婅〃鐜板緱鍍?'>>'锛夛紝涔熶笉鏀寔閫氳繃鎴柇鏉ョЩ闄ゆ墍鏈夎Е鍙戝櫒
锛堜綘蹇呴』瀵规瘡涓坊鍔犵殑瑙﹀彂鍣ㄤ娇鐢?'!'锛夈€?

### 6.2 鏀寔鐨勮Е鍙戝櫒鍛戒护


浠ヤ笅鍛戒护鍙楁敮鎸侊細

- enable_event/disable_event

  杩欎簺鍛戒护鍙互鍦ㄨЕ鍙戜簨浠跺懡涓椂鍚敤鎴栫鐢ㄥ彟涓€涓窡韪簨浠躲€傚綋娉ㄥ唽杩欎簺鍛戒护鏃讹紝
  鍙︿竴涓窡韪簨浠朵細琚縺娲伙紝浣嗗浜庘€滆蒋鈥濇ā寮忎笅琚鐢ㄣ€備篃灏辨槸璇达紝璺熻釜鐐逛細琚皟鐢紝
  浣嗕笉浼氬疄闄呰璺熻釜銆傚彧瑕佸瓨鍦ㄨ兘澶熻Е鍙戝畠鐨勭敓鏁堣Е鍙戝櫒锛岃浜嬩欢璺熻釜鐐瑰氨淇濇寔杩欑妯″紡銆?

  渚嬪锛屽綋杩涘叆 read 绯荤粺璋冪敤鏃讹紝浠ヤ笅瑙﹀彂鍣ㄤ細瀵艰嚧 kmalloc 浜嬩欢琚窡韪紝鏈熬鐨?
  :1 琛ㄧず浠呰Е鍙戜竴娆★細

```
	  # echo 'enable_event:kmem:kmalloc:1' > \
	      /sys/kernel/tracing/events/syscalls/sys_enter_read/trigger
```

  褰?read 绯荤粺璋冪敤閫€鍑烘椂锛屼互涓嬭Е鍙戝櫒浼氬鑷?kmalloc 浜嬩欢鍋滄琚窡韪€傝繖绉嶇鐢?
  鍦ㄦ瘡娆?read 绯荤粺璋冪敤閫€鍑烘椂閮戒細鍙戠敓锛?

```
	  # echo 'disable_event:kmem:kmalloc' > \
	      /sys/kernel/tracing/events/syscalls/sys_exit_read/trigger
```

  鏍煎紡涓猴細

```
      enable_event:<system>:<event>[:count]
      disable_event:<system>:<event>[:count]
```

  瑕佺Щ闄や笂杩板懡浠わ細

```
	  # echo '!enable_event:kmem:kmalloc:1' > \
	      /sys/kernel/tracing/events/syscalls/sys_enter_read/trigger

	  # echo '!disable_event:kmem:kmalloc' > \
	      /sys/kernel/tracing/events/syscalls/sys_exit_read/trigger
```

  娉ㄦ剰锛屾瘡涓Е鍙戜簨浠跺彲浠ユ湁浠绘剰鏁伴噺鐨?enable/disable_event 瑙﹀彂鍣紝浣嗘瘡涓
  瑙﹀彂鐨勪簨浠跺彧鑳芥湁涓€涓Е鍙戝櫒銆備緥濡傦紝sys_enter_read 鍙互鏈変袱涓Е鍙戝櫒鍒嗗埆鍚敤
  kmem:kmalloc 鍜?sched:sched_switch锛屼絾涓嶈兘鏈変袱涓?kmem:kmalloc 鐗堟湰锛屼緥濡?
  kmem:kmalloc 鍜?kmem:kmalloc:1锛屾垨鑰?'kmem:kmalloc if bytes_req == 256' 鍜?
  'kmem:kmalloc if bytes_alloc == 256'锛堜笉杩囧畠浠彲浠ュ悎骞朵负 kmem:kmalloc 涓婄殑
  鍗曚釜杩囨护鍣級銆?

- stacktrace

  璇ュ懡浠ゅ湪瑙﹀彂浜嬩欢鍙戠敓鏃跺皢鏍堝洖婧紙stacktrace锛夎浆鍌ㄥ埌璺熻釜缂撳啿鍖轰腑銆?

  渚嬪锛屼互涓嬭Е鍙戝櫒姣忔鍛戒腑鏃堕兘浼氳浆鍌ㄤ竴娆℃爤鍥炴函锛?

```
	  # echo 'stacktrace' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger
```

  浠ヤ笅瑙﹀彂鍣ㄥ湪 kmalloc 璇锋眰澶у皬 >= 64K 鏃讹紝鍓?5 娆″懡涓殑姣忎竴娆￠兘杞偍鏍堝洖婧細

```
	  # echo 'stacktrace:5 if bytes_req >= 65536' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger
```

  鏍煎紡涓猴細

```
      stacktrace[:count]
```

  瑕佺Щ闄や笂杩板懡浠わ細

```
	  # echo '!stacktrace' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger

	  # echo '!stacktrace:5 if bytes_req >= 65536' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger
```

  鍚庤€呬篃鍙互鏇寸畝鍗曞湴閫氳繃浠ヤ笅鏂瑰紡锛堜笉甯﹁繃婊ゅ櫒锛夌Щ闄わ細

```
	  # echo '!stacktrace:5' > \
		/sys/kernel/tracing/events/kmem/kmalloc/trigger
```

  娉ㄦ剰锛屾瘡涓Е鍙戜簨浠跺彧鑳芥湁涓€涓?stacktrace 瑙﹀彂鍣ㄣ€?

- snapshot

  璇ュ懡浠ゅ湪瑙﹀彂浜嬩欢鍙戠敓鏃惰Е鍙戜竴娆″揩鐓э紙snapshot锛夈€?

  浠ヤ笅鍛戒护鍦ㄥ潡璇锋眰闃熷垪浠ユ繁搴?> 1 鎷斿嚭锛坲nplug锛夋椂鍒涘缓涓€娆″揩鐓с€傚鏋滀綘褰撴椂
  姝ｅ湪璺熻釜涓€缁勪簨浠舵垨鍑芥暟锛屽揩鐓ц窡韪紦鍐插尯灏嗘崟鑾疯Е鍙戦偅涓€鍒荤殑璺熻釜鍐呭锛?

```
	  # echo 'snapshot if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  鍙揩鐓т竴娆★細

```
	  # echo 'snapshot:1 if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  瑕佺Щ闄や笂杩板懡浠わ細

```
	  # echo '!snapshot if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger

	  # echo '!snapshot:1 if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  娉ㄦ剰锛屾瘡涓Е鍙戜簨浠跺彧鑳芥湁涓€涓?snapshot 瑙﹀彂鍣ㄣ€?

- traceon/traceoff

  杩欎簺鍛戒护鍦ㄦ寚瀹氫簨浠跺懡涓椂鎵撳紑鎴栧叧闂窡韪€傚弬鏁板喅瀹氫簡璺熻釜绯荤粺琚墦寮€鍜屽叧闂?
  鐨勬鏁般€傚鏋滄湭鎸囧畾锛屽垯娌℃湁娆℃暟闄愬埗銆?

  浠ヤ笅鍛戒护鍦ㄥ潡璇锋眰闃熷垪浠ユ繁搴?> 1 鎷斿嚭鏃剁涓€娆″叧闂窡韪€傚鏋滀綘褰撴椂姝ｅ湪璺熻釜
  涓€缁勪簨浠舵垨鍑芥暟锛屽氨鍙互妫€鏌ヨ窡韪紦鍐插尯锛屾煡鐪嬪鑷磋浜嬩欢鍙戠敓鐨勪簨浠跺簭鍒楋細

```
	  # echo 'traceoff:1 if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  褰?nr_rq > 1 鏃跺缁堢鐢ㄨ窡韪細

```
	  # echo 'traceoff if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  瑕佺Щ闄や笂杩板懡浠わ細

```
	  # echo '!traceoff:1 if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger

	  # echo '!traceoff if nr_rq > 1' > \
		/sys/kernel/tracing/events/block/block_unplug/trigger
```

  娉ㄦ剰锛屾瘡涓Е鍙戜簨浠跺彧鑳芥湁涓€涓?traceon 鎴?traceoff 瑙﹀彂鍣ㄣ€?

- hist

  璇ュ懡浠ゅ皢鍛戒腑鐨勪簨浠惰仛鍚堝埌涓€涓搱甯岃〃涓紝鍝堝笇琛ㄧ殑閿熀浜庝竴涓垨澶氫釜璺熻釜浜嬩欢
  鏍煎紡瀛楁锛堟垨鏍堝洖婧級锛屼互鍙婁粠涓€鎴栧涓窡韪簨浠舵牸寮忓瓧娈靛拰/鎴栦簨浠惰鏁?
  锛坔itcount锛夋淳鐢熷嚭鏉ョ殑涓€缁勭疮璁℃€诲€笺€?

  璇﹁ Documentation/trace/histogram.rst 鑾峰彇璇︾粏淇℃伅鍜岀ず渚嬨€?

## 7. 鍐呮牳鎬佽窡韪簨浠?API


鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝璺熻釜浜嬩欢鐨勫懡浠よ鎺ュ彛宸茬粡缁扮话鏈変綑銆備笉杩囨湁鏃跺簲鐢ㄧ▼搴忓彲鑳介渶瑕?
琛ㄨ揪姣旂畝鍗曠殑涓€绯诲垪閾炬帴鍛戒护琛岃〃杈惧紡鏇村鏉傜殑鍏宠仈鍏崇郴锛屾垨鑰呭皢涓€缁勫懡浠ょ粍鍚?
璧锋潵鏈韩灏辫繃浜庣箒鐞愩€備緥濡傦紝鏌愪釜搴旂敤绋嬪簭鍙兘闇€瑕佲€滅洃鍚€濊窡韪祦锛屼互渚跨淮鎶や竴涓?
鍐呮牳鎬佺姸鎬佹満锛屾娴嬶紙姣斿璇达級璋冨害鍣ㄤ腑浣曟椂鍑虹幇浜嗛潪娉曠殑鍐呮牳鐘舵€併€?

璺熻釜浜嬩欢瀛愮郴缁熸彁渚涗簡涓€涓唴鏍告€?API锛屽厑璁告ā鍧楁垨鍏朵粬鍐呮牳浠ｇ爜鎸夐渶鐢熸垚鐢ㄦ埛瀹氫箟鐨?
鈥滃悎鎴愨€濓紙synthetic锛変簨浠讹紝杩欎簺浜嬩欢鏃㈠彲鐢ㄤ簬鎵╁厖鐜版湁鐨勮窡韪祦锛屼篃鍙敤浜庡彂鍑?
鏌愪釜鐗瑰畾閲嶈鐘舵€佸凡鍙戠敓鐨勪俊鍙枫€?

绫讳技鐨勫唴鏍告€?API 涔熷彲鐢ㄤ簬鍒涘缓 kprobe 鍜?kretprobe 浜嬩欢銆?

鍚堟垚浜嬩欢 API 涓?k/ret/probe 浜嬩欢 API 閮藉缓绔嬪湪鏇翠綆灞傜殑 "dynevent_cmd" 浜嬩欢
鍛戒护 API 涔嬩笂锛岃 API 涔熷彲鐢ㄤ簬鏇翠笓闂ㄧ殑搴旂敤锛屾垨浣滀负鍏朵粬鏇撮珮绾ц窡韪簨浠?API
鐨勫熀纭€銆?

涓烘鎻愪緵鐨?API 濡備笅鎵€杩帮紝骞跺厑璁革細

  - 鍔ㄦ€佸垱寤哄悎鎴愪簨浠跺畾涔?
  - 鍔ㄦ€佸垱寤?kprobe 鍜?kretprobe 浜嬩欢瀹氫箟
  - 浠庡唴鏍告€佷唬鐮佽窡韪悎鎴愪簨浠?
  - 浣庡眰绾х殑 "dynevent_cmd" API

### 7.1 鍔ㄦ€佸垱寤哄悎鎴愪簨浠跺畾涔?


鏈夊嚑绉嶆柟娉曞彲浠ヤ粠鍐呮牳妯″潡鎴栧叾浠栧唴鏍镐唬鐮佸垱寤烘柊鐨勫悎鎴愪簨浠躲€?

绗竴绉嶆柟娉曚娇鐢?synth_event_create() 涓€姝ュ垱寤轰簨浠躲€傚湪杩欑鏂规硶涓紝瑕佸垱寤虹殑
浜嬩欢鍚嶄互鍙婁竴涓畾涔夊瓧娈电殑鏁扮粍琚彁渚涚粰 synth_event_create()銆傚鏋滄垚鍔燂紝灏变細
鍦ㄨ皟鐢ㄤ箣鍚庡瓨鍦ㄥ叿鏈夎鍚嶇О鍜屽瓧娈电殑鍚堟垚浜嬩欢锛?

```
  ret = synth_event_create("schedtest", sched_fields,
                           ARRAY_SIZE(sched_fields), THIS_MODULE);
```

姝ょず渚嬩腑鐨?sched_fields 鍙傛暟鎸囧悜涓€涓?struct synth_field_desc 鏁扮粍锛屽叾涓?
姣忎竴椤归€氳繃绫诲瀷鍜屽悕绉版弿杩颁竴涓簨浠跺瓧娈碉細

```
  static struct synth_field_desc sched_fields[] = {
        { .type = "pid_t",              .name = "next_pid_field" },
        { .type = "char[16]",           .name = "next_comm_field" },
        { .type = "u64",                .name = "ts_ns" },
        { .type = "u64",                .name = "ts_ms" },
        { .type = "unsigned int",       .name = "cpu" },
        { .type = "char[64]",           .name = "my_string_field" },
        { .type = "int",                .name = "my_int_field" },
  };
```

鍙敤绫诲瀷鍙傝 synth_field_size()銆?

濡傛灉 field_name 鍖呭惈 [n]锛屽垯璇ュ瓧娈佃瑙嗕负闈欐€佹暟缁勩€?

濡傛灉 field_names 鍖呭惈 []锛堟棤涓嬫爣锛夛紝鍒欒瀛楁琚涓哄姩鎬佹暟缁勶紝瀹冨彧浼氬崰鐢?
鍦ㄤ簨浠朵腑淇濆瓨璇ユ暟缁勬墍闇€鐨勭┖闂淬€?

鐢变簬浜嬩欢鐨勭┖闂存槸鍦ㄤ负瀛楁璧嬪€间箣鍓嶅氨棰勭暀濂界殑锛屽洜姝や娇鐢ㄥ姩鎬佹暟缁勬剰鍛崇潃涓嬮潰
鎻忚堪鐨勯€愭锛坧iecewise锛夊唴鏍告€?API 涓嶈兘涓庡姩鎬佹暟缁勪竴璧蜂娇鐢ㄣ€備笉杩囷紝鍏朵粬闈為€愭鐨?
鍐呮牳鎬?API 鍙互涓庡姩鎬佹暟缁勪竴璧蜂娇鐢ㄣ€?

濡傛灉璇ヤ簨浠舵槸浠庢ā鍧楀唴閮ㄥ垱寤虹殑锛屽垯蹇呴』鍚?synth_event_create() 浼犻€掍竴涓寚鍚?
璇ユā鍧楃殑鎸囬拡銆傝繖灏嗙‘淇濆湪璇ユā鍧楄绉婚櫎鏃讹紝璺熻釜缂撳啿鍖轰笉浼氬寘鍚笉鍙鐨勪簨浠躲€?

姝ゆ椂锛屼簨浠跺璞″凡鍑嗗濂界敤浜庣敓鎴愭柊鐨勪簨浠躲€?

鍦ㄧ浜岀鏂规硶涓紝浜嬩欢鏄垎鑻ュ共姝ュ垱寤虹殑銆傝繖鍏佽鍔ㄦ€佸垱寤轰簨浠讹紝鑰屾棤闇€浜嬪厛鍒涘缓
骞跺～鍏呬竴涓瓧娈垫暟缁勩€?

瑕佷娇鐢ㄨ繖绉嶆柟娉曪紝搴旈鍏堜娇鐢?synth_event_gen_cmd_start() 鎴?
synth_event_gen_cmd_array_start() 鍒涘缓绌烘垨閮ㄥ垎濉厖鐨勫悎鎴愪簨浠躲€傚浜?
synth_event_gen_cmd_start()锛屽簲鎻愪緵浜嬩欢鍚嶄互鍙婁竴涓垨澶氫釜鍙傛暟瀵癸紙姣忓鍙傛暟
琛ㄧず涓€涓?'type field_name;' 瀛楁瑙勬牸锛夈€傚浜?
synth_event_gen_cmd_array_start()锛屽簲鎻愪緵浜嬩欢鍚嶄互鍙婁竴涓?struct
synth_field_desc 鏁扮粍銆傚湪璋冪敤 synth_event_gen_cmd_start() 鎴?
synth_event_gen_cmd_array_start() 涔嬪墠锛岀敤鎴峰簲浣跨敤 synth_event_cmd_init()
鍒涘缓骞跺垵濮嬪寲涓€涓?dynevent_cmd 瀵硅薄銆?

渚嬪锛岃鍒涘缓涓€涓甫涓や釜瀛楁鐨勫悕涓?"schedtest" 鐨勫悎鎴愪簨浠讹細

```
  struct dynevent_cmd cmd;
  char *buf;

  /* Create a buffer to hold the generated command */
  buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);

  /* Before generating the command, initialize the cmd object */
  synth_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);

  ret = synth_event_gen_cmd_start(&cmd, "schedtest", THIS_MODULE,
                                  "pid_t", "next_pid_field",
                                  "u64", "ts_ns");
```

鎴栬€咃紝浣跨敤 struct synth_field_desc 瀛楁鏁扮粍锛?

```
  ret = synth_event_gen_cmd_array_start(&cmd, "schedtest", THIS_MODULE,
                                        fields, n_fields);
```

涓€鏃﹀悎鎴愪簨浠跺璞¤鍒涘缓锛屽氨鍙互鐢ㄦ洿澶氬瓧娈靛～鍏呭畠銆傚瓧娈甸€氳繃
synth_event_add_field() 閫愪釜娣诲姞锛屾彁渚?dynevent_cmd 瀵硅薄銆佸瓧娈电被鍨嬪拰瀛楁鍚嶃€?
渚嬪锛岃娣诲姞涓€涓悕涓?intfield 鐨勬柊 int 瀛楁锛?

```
  ret = synth_event_add_field(&cmd, "int", "intfield");
```

鍙敤绫诲瀷鍙傝 synth_field_size()銆傚鏋?field_name 鍖呭惈 [n]锛屽垯璇ュ瓧娈佃瑙嗕负
鏁扮粍銆?

涔熷彲浠ヤ娇鐢?synth_field_desc 鏁扮粍锛岄€氳繃 add_synth_fields() 涓€娆℃€ф坊鍔犱竴缁勫瓧娈点€?
渚嬪锛岃繖灏嗘坊鍔狅細

```
  ret = synth_event_add_fields(&cmd, sched_fields, 4);
```

濡傛灉浣犲凡缁忔湁涓€涓舰濡?'type field_name' 鐨勫瓧绗︿覆锛屽彲浠ヤ娇鐢?
synth_event_add_field_str() 鍘熸牱娣诲姞瀹冿紱瀹冭繕浼氳嚜鍔ㄥ湪瀛楃涓插悗杩藉姞涓€涓?';'銆?

涓€鏃︽墍鏈夊瓧娈甸兘宸叉坊鍔狅紝浜嬩欢搴旇缁堢粨鍖栧苟锛?

```
  ret = synth_event_gen_cmd_end(&cmd);
```

姝ゆ椂锛屼簨浠跺璞″凡鍑嗗濂界敤浜庤窡韪柊浜嬩欢銆?

### 7.2 浠庡唴鏍告€佷唬鐮佽窡韪悎鎴愪簨浠?


瑕佽窡韪悎鎴愪簨浠讹紝鏈夊嚑绉嶉€夋嫨銆傜涓€绉嶉€夋嫨鏄娇鐢?synth_event_trace()锛堟帴鍙?
鏁伴噺鍙彉鐨勫涓€硷級鎴?synth_event_trace_array()锛堟帴鍙楄璁剧疆鐨勫€兼暟缁勶級涓€娆℃€?
璺熻釜璇ヤ簨浠躲€傜浜岀閫夋嫨鍙互閬垮厤棰勫厛鏋勯€犲€兼暟缁勬垨鍙傛暟鍒楄〃鐨勯渶瑕侊紝閫氳繃
synth_event_trace_start() 鍜?synth_event_trace_end()锛岄厤鍚?
synth_event_add_next_val() 鎴?synth_event_add_val() 鏉ラ€愭娣诲姞鍊笺€?

### 7.2.1 涓€娆℃€ц窡韪悎鎴愪簨浠?


瑕佷竴娆℃€ц窡韪悎鎴愪簨浠讹紝鍙互浣跨敤 synth_event_trace() 鎴?
synth_event_trace_array() 鍑芥暟銆?

synth_event_trace() 鍑芥暟浼犲叆琛ㄧず鍚堟垚浜嬩欢鐨?trace_event_file锛堝彲閫氳繃
trace_get_event_file() 浣跨敤鍚堟垚浜嬩欢鍚嶃€?synthetic" 浣滀负绯荤粺鍚嶏紝浠ュ強璺熻釜
瀹炰緥鍚嶏紙鑻ヤ娇鐢ㄥ叏灞€璺熻釜鏁扮粍鍒欎负 NULL锛夎幏鍙栵級锛屼互鍙婃暟閲忓彲鍙樼殑澶氫釜 u64 鍙傛暟
锛堟瘡涓悎鎴愪簨浠跺瓧娈典竴涓級鍜屼紶鍏ョ殑鍊肩殑涓暟銆?

鍥犳锛岃璺熻釜瀵瑰簲浜庡涓嬪悎鎴愪簨浠跺畾涔夌殑浜嬩欢锛?

```
  ret = synth_event_trace(create_synth_test, 7, /* number of values */
                          444,             /* next_pid_field */
                          (u64)"clackers", /* next_comm_field */
                          1000000,         /* ts_ns */
                          1000,            /* ts_ms */
                          smp_processor_id(),/* cpu */
                          (u64)"Thneed",   /* my_string_field */
                          999);            /* my_int_field */
```

鎵€鏈夊€奸兘搴旇浆鎹负 u64锛屽瓧绗︿覆鍊煎彧鏄寚鍚戝瓧绗︿覆鐨勬寚閽堬紝杞崲涓?u64銆傚瓧绗︿覆灏?
閫氳繃杩欎簺鎸囬拡澶嶅埗鍒颁簨浠朵腑涓鸿瀛楃涓查鐣欑殑绌洪棿銆?

鎴栬€咃紝鍙互浣跨敤 synth_event_trace_array() 鍑芥暟瀹屾垚鍚屾牱鐨勪簨鎯呫€傚畠浼犲叆琛ㄧず
鍚堟垚浜嬩欢鐨?trace_event_file锛堝彲閫氳繃 trace_get_event_file() 浣跨敤鍚堟垚浜嬩欢鍚嶃€?
"synthetic" 浣滀负绯荤粺鍚嶏紝浠ュ強璺熻釜瀹炰緥鍚嶏紙鑻ヤ娇鐢ㄥ叏灞€璺熻釜鏁扮粍鍒欎负 NULL锛夎幏鍙栵級锛?
浠ュ強涓€涓?u64 鏁扮粍锛屾瘡涓悎鎴愪簨浠跺瓧娈典竴涓€?

瑕佽窡韪搴斾簬濡備笅鍚堟垚浜嬩欢瀹氫箟鐨勪簨浠讹細

```
  u64 vals[7];

  vals[0] = 777;                  /* next_pid_field */
  vals[1] = (u64)"tiddlywinks";   /* next_comm_field */
  vals[2] = 1000000;              /* ts_ns */
  vals[3] = 1000;                 /* ts_ms */
  vals[4] = smp_processor_id();   /* cpu */
  vals[5] = (u64)"thneed";        /* my_string_field */
  vals[6] = 398;                  /* my_int_field */
```

'vals' 鏁扮粍鍙槸涓€涓?u64 鏁扮粍锛屽叾涓暟蹇呴』涓庡悎鎴愪簨浠朵腑鐨勫瓧娈垫暟鍖归厤锛屽苟涓?
蹇呴』涓庡悎鎴愪簨浠跺瓧娈电殑椤哄簭鐩稿悓銆?

鎵€鏈夊€奸兘搴旇浆鎹负 u64锛屽瓧绗︿覆鍊煎彧鏄寚鍚戝瓧绗︿覆鐨勬寚閽堬紝杞崲涓?u64銆傚瓧绗︿覆灏?
閫氳繃杩欎簺鎸囬拡澶嶅埗鍒颁簨浠朵腑涓鸿瀛楃涓查鐣欑殑绌洪棿銆?

涓轰簡璺熻釜鍚堟垚浜嬩欢锛岄渶瑕佷竴涓寚鍚戣窡韪簨浠舵枃浠剁殑鎸囬拡銆傚彲浠ヤ娇鐢?
trace_get_event_file() 鍑芥暟鑾峰彇瀹冣€斺€斿畠浼氬湪缁欏畾鐨勮窡韪疄渚嬶紙姝ゅ涓?NULL锛屽洜涓?
浣跨敤鐨勬槸椤跺眰璺熻釜鏁扮粍锛変腑鏌ユ壘璇ユ枃浠讹紝鍚屾椂锛?

```
       schedtest_event_file = trace_get_event_file(NULL, "synthetic",
                                                   "schedtest");
```

鍦ㄨ窡韪簨浠朵箣鍓嶏紝搴斾互鏌愮鏂瑰紡鍚敤瀹冿紝鍚﹀垯鍚堟垚浜嬩欢瀹為檯涓婁笉浼氬嚭鐜板湪璺熻釜缂撳啿鍖轰腑銆?

瑕佷粠鍐呮牳鍚敤鍚堟垚浜嬩欢锛屽彲浠ヤ娇鐢?trace_array_set_clr_event()锛堝畠骞堕潪鍚堟垚浜嬩欢
涓撶敤锛屽洜姝ら渶瑕佹樉寮忔寚瀹?"synthetic" 绯荤粺鍚嶏級銆?

```
       trace_array_set_clr_event(schedtest_event_file->tr,
                                 "synthetic", "schedtest", true);
```

```
       trace_array_set_clr_event(schedtest_event_file->tr,
                                 "synthetic", "schedtest", false);
```

鏈€鍚庯紝鍙互浣跨敤 synth_event_trace_array() 瀹為檯璺熻釜锛?

```
       ret = synth_event_trace_array(schedtest_event_file, vals,
                                     ARRAY_SIZE(vals));
```

瑕佺Щ闄ゅ悎鎴愪簨浠讹紝搴斿厛绂佺敤璇ヤ簨浠讹紝骞讹細

```
       trace_array_set_clr_event(schedtest_event_file->tr,
                                 "synthetic", "schedtest", false);
       trace_put_event_file(schedtest_event_file);
```

濡傛灉杩欎簺閮芥垚鍔燂紝灏卞彲浠ヨ皟鐢?synth_event_delete()锛?

```
       ret = synth_event_delete("schedtest");
```

### 7.2.2 閫愭璺熻釜鍚堟垚浜嬩欢


瑕佷娇鐢ㄤ笂鏂囨弿杩扮殑閫愭鏂规硶璺熻釜鍚堟垚浜嬩欢锛屼娇鐢?synth_event_trace_start() 鍑芥暟
鏉モ€滄墦寮€鈥濆悎鎴愪簨浠讹細

```
       struct synth_event_trace_state trace_state;

       ret = synth_event_trace_start(schedtest_event_file, &trace_state);
```

瀹冧紶鍏ヨ〃绀哄悎鎴愪簨浠剁殑 trace_event_file锛堜娇鐢ㄤ笌涓婅堪鐩稿悓鐨勬柟娉曪級锛屼互鍙婁竴涓寚鍚?
struct synth_event_trace_state 瀵硅薄鐨勬寚閽堬紝璇ュ璞″湪浣跨敤鍓嶄細琚竻闆讹紝骞剁敤浜庡湪
鏈璋冪敤涓庡悗缁皟鐢ㄤ箣闂寸淮鎶ょ姸鎬併€?

涓€鏃︿簨浠惰鎵撳紑锛堝嵆宸插湪璺熻釜缂撳啿鍖轰腑涓哄叾棰勭暀浜嗙┖闂达級锛屽氨鍙互璁剧疆鍚勪釜瀛楁銆?
鏈変袱绉嶆柟寮忥細涓€绉嶆槸鎸変簨浠朵腑鐨勬瘡涓瓧娈典緷娆¤缃紙鏃犻渶鏌ユ壘锛夛紝鍙︿竴绉嶆槸鎸夊悕绉拌缃?
锛堥渶瑕佹煡鎵撅級銆備袱鑰呯殑鏉冭　鍦ㄤ簬璧嬪€肩殑鐏垫椿鎬т笌姣忎釜瀛楁鏌ユ壘鐨勫紑閿€涔嬮棿銆?

瑕佹棤闇€鏌ユ壘鍦颁緷娆¤祴鍊硷紝搴斾娇鐢?synth_event_add_next_val()銆傛瘡娆¤皟鐢ㄤ紶鍏ヤ笌
synth_event_trace_start() 涓浉鍚岀殑 synth_event_trace_state 瀵硅薄锛屼互鍙婅璁剧疆
浜嬩欢涓嬩竴涓瓧娈电殑鍊笺€傛瘡璁剧疆涓€涓瓧娈靛悗锛屸€滄父鏍団€濓紙cursor锛変細鎸囧悜涓嬩竴涓瓧娈碉紝
璇ュ瓧娈靛皢鐢卞悗缁皟鐢ㄨ缃紝渚濇杩涜鐩村埌鎵€鏈夊瓧娈甸兘鎸夐『搴忚缃畬姣曘€備笌涓婅堪绀轰緥
鐩稿悓鐨勮皟鐢ㄥ簭鍒椾娇鐢細

```
       /* next_pid_field */
       ret = synth_event_add_next_val(777, &trace_state);

       /* next_comm_field */
       ret = synth_event_add_next_val((u64)"slinky", &trace_state);

       /* ts_ns */
       ret = synth_event_add_next_val(1000000, &trace_state);

       /* ts_ms */
       ret = synth_event_add_next_val(1000, &trace_state);

       /* cpu */
       ret = synth_event_add_next_val(smp_processor_id(), &trace_state);

       /* my_string_field */
       ret = synth_event_add_next_val((u64)"thneed_2.01", &trace_state);

       /* my_int_field */
       ret = synth_event_add_next_val(395, &trace_state);
```

瑕佹寜浠绘剰椤哄簭璧嬪€硷紝搴斾娇鐢?synth_event_add_val()銆傛瘡娆¤皟鐢ㄤ紶鍏ヤ笌
synth_event_trace_start() 涓浉鍚岀殑 synth_event_trace_state 瀵硅薄锛屼互鍙婅璁剧疆鐨?
瀛楁鐨勫瓧娈靛悕鍜屽畠鐨勫€笺€備笌涓婅堪绀轰緥鐩稿悓鐨勮皟鐢ㄥ簭鍒椾娇鐢ㄦ鏂规硶锛堢渷鐣ヤ簡閿欒澶勭悊锛夛細

```
       ret = synth_event_add_val("next_pid_field", 777, &trace_state);
       ret = synth_event_add_val("next_comm_field", (u64)"silly putty",
                                 &trace_state);
       ret = synth_event_add_val("ts_ns", 1000000, &trace_state);
       ret = synth_event_add_val("ts_ms", 1000, &trace_state);
       ret = synth_event_add_val("cpu", smp_processor_id(), &trace_state);
       ret = synth_event_add_val("my_string_field", (u64)"thneed_9",
                                 &trace_state);
       ret = synth_event_add_val("my_int_field", 3999, &trace_state);
```

娉ㄦ剰锛宻ynth_event_add_next_val() 鍜?synth_event_add_val() 鍦ㄥ悓涓€涓簨浠剁殑
璺熻釜杩囩▼涓槸涓嶅吋瀹圭殑鈥斺€斿彲浠ヤ娇鐢ㄥ叾涓换鎰忎竴涓紝浣嗕笉鑳藉悓鏃朵娇鐢ㄤ袱鑰呫€?

鏈€鍚庯紝鍦ㄤ簨浠惰鈥滃叧闂€濅箣鍓嶏紝瀹冨疄闄呬笂涓嶄細琚窡韪紝杩欎竴姝ラ€氳繃
synth_event_trace_end() 瀹屾垚锛屽畠鍙帴鍙楋細

```
       ret = synth_event_trace_end(&trace_state);
```

娉ㄦ剰锛屾棤璁轰换浣?add 璋冪敤鏄惁澶辫触锛堜緥濡備紶鍏ヤ簡閿欒鐨勫瓧娈靛悕锛夛紝閮藉繀椤诲湪鏈€鍚庤皟鐢?
synth_event_trace_end()銆?

### 7.3 鍔ㄦ€佸垱寤?kprobe 鍜?kretprobe 浜嬩欢瀹氫箟


瑕佷粠鍐呮牳浠ｇ爜鍒涘缓 kprobe 鎴?kretprobe 璺熻釜浜嬩欢锛屽彲浠ヤ娇鐢?
kprobe_event_gen_cmd_start() 鎴?kretprobe_event_gen_cmd_start() 鍑芥暟銆?

瑕佸垱寤?kprobe 浜嬩欢锛屽簲棣栧厛浣跨敤 kprobe_event_gen_cmd_start() 鍒涘缓涓€涓┖鎴?
閮ㄥ垎濉厖鐨?kprobe 浜嬩欢銆傚簲鎸囧畾浜嬩欢鍚嶅拰鎺㈡祴浣嶇疆锛屼互鍙婁竴涓〃绀烘帰娴嬪瓧娈电殑
鍙傛暟鍒楄〃鎻愪緵缁欒鍑芥暟銆傚湪璋冪敤 kprobe_event_gen_cmd_start() 涔嬪墠锛岀敤鎴峰簲浣跨敤
kprobe_event_cmd_init() 鍒涘缓骞跺垵濮嬪寲涓€涓?dynevent_cmd 瀵硅薄銆?

```
  struct dynevent_cmd cmd;
  char *buf;

  /* Create a buffer to hold the generated command */
  buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);

  /* Before generating the command, initialize the cmd object */
  kprobe_event_cmd_init(&cmd, buf, MAX_DYNEVENT_CMD_LEN);

  /*
   * Define the gen_kprobe_test event with the first 2 kprobe
   * fields.
   */
  ret = kprobe_event_gen_cmd_start(&cmd, "gen_kprobe_test", "do_sys_open",
                                   "dfd=%ax", "filename=%dx");
```

涓€鏃?kprobe 浜嬩欢瀵硅薄琚垱寤猴紝灏卞彲浠ョ敤鏇村瀛楁濉厖瀹冦€傚彲浠ヤ娇鐢?
kprobe_event_add_fields() 娣诲姞瀛楁锛屾彁渚?dynevent_cmd 瀵硅薄浠ュ強涓€涓彲鍙樺弬鏁?
鍒楄〃鐨勬帰娴嬪瓧娈点€備緥濡傦紝瑕佹坊鍔狅細

```
  ret = kprobe_event_add_fields(&cmd, "flags=%cx", "mode=+4($stack)");
```

涓€鏃︽墍鏈夊瓧娈甸兘宸叉坊鍔狅紝灏卞簲閫氳繃璋冪敤 kprobe_event_gen_cmd_end() 鎴?
kretprobe_event_gen_cmd_end() 鍑芥暟锛堝彇鍐充簬鍒涘缓鐨勬槸 kprobe 杩樻槸 kretprobe锛?
鏉ョ粓缁撳寲骞舵敞鍐岃浜嬩欢锛?

```
  ret = kprobe_event_gen_cmd_end(&cmd);
```

```
  ret = kretprobe_event_gen_cmd_end(&cmd);
```

姝ゆ椂锛屼簨浠跺璞″凡鍑嗗濂界敤浜庤窡韪柊浜嬩欢銆?

绫讳技鍦帮紝鍙互浣跨敤 kretprobe_event_gen_cmd_start() 閰嶅悎鎺㈤拡鍚嶃€佷綅缃互鍙?
鏉ュ垱寤?kretprobe 浜嬩欢锛?

```
  ret = kretprobe_event_gen_cmd_start(&cmd, "gen_kretprobe_test",
                                      "do_sys_open", "$retval");
```

涓庡悎鎴愪簨浠剁殑鎯呭喌绫讳技锛屽涓嬩唬鐮佸彲浠ワ細

```
  gen_kprobe_test = trace_get_event_file(NULL, "kprobes", "gen_kprobe_test");

  ret = trace_array_set_clr_event(gen_kprobe_test->tr,
                                  "kprobes", "gen_kprobe_test", true);
```

鏈€鍚庯紝鍚屾牱涓庡悎鎴愪簨浠剁被浼硷紝濡備笅浠ｇ爜鍙互锛?

```
  trace_put_event_file(gen_kprobe_test);

  ret = kprobe_event_delete("gen_kprobe_test");
```

### 7.4 "dynevent_cmd" 浣庡眰 API


鍐呮牳鎬佺殑鍚堟垚浜嬩欢鎺ュ彛鍜?kprobe 鎺ュ彛閮藉缓绔嬪湪鏇翠綆灞傜殑 "dynevent_cmd" 鎺ュ彛涔嬩笂銆?
璇ユ帴鍙ｆ棬鍦ㄤ负鏇撮珮绾х殑鎺ュ彛锛堜緥濡傚悎鎴愪簨浠舵帴鍙ｅ拰 kprobe 鎺ュ彛锛屽畠浠彲浣滀负绀轰緥锛?
鎻愪緵鍩虹銆?

鍩烘湰鎬濇兂寰堢畝鍗曪紝灏辨槸鎻愪緵涓€涓彲鐢ㄤ簬鐢熸垚璺熻釜浜嬩欢鍛戒护鐨勯€氱敤灞傘€傜敓鎴愮殑鍛戒护瀛楃涓?
闅忓悗鍙互琚紶閫掔粰璺熻釜浜嬩欢瀛愮郴缁熶腑宸茬粡瀛樺湪鐨勫懡浠よВ鏋愬拰浜嬩欢鍒涘缓浠ｇ爜锛岀敤浜庡垱寤?
鐩稿簲鐨勮窡韪簨浠躲€?

绠€鑰岃█涔嬶紝瀹冪殑宸ヤ綔鏂瑰紡鏄細鏇撮珮绾х殑鎺ュ彛浠ｇ爜鍒涘缓涓€涓?struct dynevent_cmd 瀵硅薄锛?
鐒跺悗浣跨敤 dynevent_arg_add() 鍜?dynevent_arg_pair_add() 杩欎袱涓嚱鏁版潵鏋勫缓鍛戒护
瀛楃涓诧紝鏈€鍚庨€氳繃 dynevent_create() 鍑芥暟鎵ц璇ュ懡浠ゃ€傝鎺ュ彛鐨勭粏鑺傚涓嬫墍杩般€?

鏋勫缓鏂板懡浠ゅ瓧绗︿覆鐨勭涓€姝ユ槸鍒涘缓骞跺垵濮嬪寲涓€涓?dynevent_cmd 瀹炰緥銆備緥濡傦紝鎴戜滑锛?

```
  struct dynevent_cmd cmd;
  char *buf;
  int ret;

  buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL);

  dynevent_cmd_init(cmd, buf, maxlen, DYNEVENT_TYPE_FOO,
                    foo_event_run_command);
```

dynevent_cmd 鍒濆鍖栭渶瑕佺粰瀹氫竴涓敤鎴锋寚瀹氱殑缂撳啿鍖哄拰缂撳啿鍖洪暱搴︼紙鍙互浣跨敤
MAX_DYNEVENT_CMD_LEN鈥斺€斿畠澶у皬涓?2k锛岄€氬父澶ぇ鑰屼笉閫傚悎鏀惧湪鏍堜笂锛屽洜姝や細鍔ㄦ€佸垎閰嶏級銆?
涓€涓?dynevent 绫诲瀷 id锛堢敤浜庢鏌ュ悗缁?API 璋冪敤鏄惁灞炰簬姝ｇ‘鐨勫懡浠ょ被鍨嬶級锛屼互鍙婁竴涓?
鎸囧悜鐗瑰畾浜嬩欢鐨?run_command() 鍥炶皟鐨勬寚閽堬紝璇ュ洖璋冨皢琚皟鐢ㄤ互瀹為檯鎵ц璇ョ壒瀹氫簨浠剁殑
鍛戒护鍑芥暟銆?

瀹屾垚涔嬪悗锛屽氨鍙互閫氳繃杩炵画璋冪敤娣诲姞鍙傛暟鐨勫嚱鏁版潵鏋勫缓鍛戒护瀛楃涓层€?

瑕佹坊鍔犲崟涓弬鏁帮紝瀹氫箟骞跺垵濮嬪寲涓€涓?struct dynevent_arg 鎴?struct
dynevent_arg_pair 瀵硅薄銆備笅闈㈡槸涓€涓渶绠€鍗曠殑鍙傛暟娣诲姞绀轰緥锛屽畠鍙槸灏嗙粰瀹氱殑瀛楃涓?
浣滀负闄勫姞鍒板懡浠ゆ湯灏撅細

```
  struct dynevent_arg arg;

  dynevent_arg_init(&arg, NULL, 0);

  arg.str = name;

  ret = dynevent_arg_add(cmd, &arg);
```

arg 瀵硅薄棣栧厛浣跨敤 dynevent_arg_init() 鍒濆鍖栵紝鍦ㄨ繖绉嶆儏鍐典笅鐨勫弬鏁颁负 NULL 鎴?0锛?
鎰忓懗鐫€鏈熬娌℃湁闄勫姞鍙€夌殑鍋ュ叏鎬ф鏌ュ嚱鏁版垨鍒嗛殧绗︺€?

涓嬮潰鏄彟涓€涓洿澶嶆潅鐨勩€佷娇鐢ㄢ€滃弬鏁板鈥濓紙arg pair锛夌殑绀轰緥锛屽畠鐢ㄤ簬鍒涘缓涓€涓敱涓ら儴鍒?
缁勫悎涓轰竴涓崟鍏冪殑鍙傛暟锛屼緥濡備竴涓?'type field_name;' 鍙傛暟鎴栦竴涓畝鍗曪細

```
  struct dynevent_arg_pair arg_pair;

  dynevent_arg_pair_init(&arg_pair, dynevent_foo_check_arg_fn, 0, ';');

  arg_pair.lhs = type;
  arg_pair.rhs = name;

  ret = dynevent_arg_pair_add(cmd, &arg_pair);
```

鍚屾牱锛宎rg_pair 棣栧厛琚垵濮嬪寲锛屽湪杩欑鎯呭喌涓嬪甫鏈変竴涓敤浜庢鏌ュ弬鏁板仴鍏ㄦ€х殑鍥炶皟
鍑芥暟锛堜緥濡傦紝妫€鏌ヨ瀵圭殑涓ら儴鍒嗛兘涓嶄负 NULL锛夛紝浠ュ強涓€涓敤浜庡湪涓ら儴鍒嗕箣闂存坊鍔犺繍绠楃
鐨勫瓧绗︼紙姝ゅ娌℃湁锛夊拰涓€涓拷鍔犲埌鍙傛暟瀵规湯灏剧殑鍒嗛殧绗︼紙姝ゅ涓?';'锛夈€?

杩樻湁涓€涓?dynevent_str_add() 鍑芥暟锛屽彲鐢ㄤ簬绠€鍗曞湴鍘熸牱娣诲姞涓€涓瓧绗︿覆锛屼笉甯︾┖鏍笺€?
鍒嗛殧绗︽垨鍙傛暟妫€鏌ャ€?

鍙互璋冪敤浠绘剰鏁伴噺鐨?dynevent_*_add() 鏉ユ瀯寤哄瓧绗︿覆锛堢洿鍒板叾闀垮害瓒呰繃 cmd->maxlen锛夈€?
褰撴墍鏈夊弬鏁伴兘宸叉坊鍔犱笖鍛戒护瀛楃涓插畬鎴愭椂锛屽墿涓嬬殑鍞竴浜嬫儏灏辨槸杩愯鍛戒护锛岃繖鍙渶
绠€鍗曞湴璋冪敤锛?

```
  ret = dynevent_create(&cmd);
```

姝ゆ椂锛屽鏋滆繑鍥炲€间负 0锛屽垯鍔ㄦ€佷簨浠跺凡琚垱寤哄苟鍙互浣跨敤銆?

鏈夊叧璇?API 鐨勮缁嗕俊鎭紝璇峰弬瑙?dynevent_cmd 鍑芥暟瀹氫箟鏈韩銆?
