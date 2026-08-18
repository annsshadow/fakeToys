## 娣峰悎鑷姩鏈猴紙Hybrid Automata锛?
娣峰悎鑷姩鏈烘槸纭畾鎬ц嚜鍔ㄦ満锛坉eterministic automata锛夌殑涓€绉嶆墿灞曪紝鏂囩尞涓湁鍑犵娣峰悎鑷姩鏈虹殑瀹氫箟銆傝繖閲屽疄鐜扮殑閫傞厤褰㈠紡琚寮忚浣?G锛屽苟瀹氫箟涓轰竴涓?7 鍏冪粍锛?
        **G** = { **X**, **E**, **V**, **f**, x\ `0`, X\ `m`, **i** }

- **X** 鏄姸鎬侀泦鍚堬紱
- **E** 鏄簨浠剁殑鏈夐檺闆嗗悎锛?- **V** 鏄幆澧冨彉閲忕殑鏈夐檺闆嗗悎锛?- x\ `0` 鏄垵濮嬬姸鎬侊紱
- X\ `m`锛?*X** 鐨勫瓙闆嗭級鏄爣璁帮紙鎴栨渶缁堬級鐘舵€佺殑闆嗗悎銆?- **f** : **X** x **E** x **C(V)** -> **X** 鏄浆绉诲嚱鏁般€?  瀹冨畾涔変簡鍦ㄧ姸鎬?**X** 涓彂鐢熸潵鑷?**E** 鐨勪簨浠舵椂鐨勭姸鎬佽浆绉汇€備笌纭畾鎬ц嚜鍔ㄦ満涓嶅悓锛岃浆绉诲嚱鏁拌繕鍖呮嫭鏉ヨ嚜鎵€鏈夊彲鑳界害鏉熼泦鍚堬紙瀹氫箟涓?**C(V)**锛夌殑瀹堝崼锛坓uard锛夈€傚畧鍗湪浜嬩欢鍙戠敓鏃舵牴鎹?**V** 鐨勫彇鍊煎彲浠ヤ负鐪熸垨鍋囷紝骞朵笖浠呭綋绾︽潫涓虹湡鏃惰浆绉绘墠鍙兘銆備笌纭畾鎬ц嚜鍔ㄦ満绫讳技锛屽湪 **X** 涓殑鐘舵€佸彂鐢?**E** 涓殑浜嬩欢鏃讹紝濡傛灉瀹堝崼涓虹湡锛屽垯鏈変竴涓‘瀹氭€х殑涓嬩竴鐘舵€佹潵鑷?**X**銆?- **i** : **X** -> **C'(V)** 鏄笉鍙橀噺璧嬪€煎嚱鏁帮紝杩欐槸鍒嗛厤缁?**X** 涓瘡涓姸鎬佺殑绾︽潫锛屽湪 **X** 涓殑姣忎釜鐘舵€侀兘蹇呴』鍦ㄤ笉鍙橀噺涓哄亣涔嬪墠绂诲紑銆傚浜庢棤璁?**V** 鍙栧€煎浣曢兘涓虹湡鐨勯偅浜涗笉鍙橀噺锛屾垜浠彲浠ョ渷鐣ュ叾琛ㄧず銆?
鎵€鏈夊彲鑳界害鏉熺殑闆嗗悎 **C(V)** 鏍规嵁浠ヤ笅璇硶瀹氫箟锛?
        g = v < c | v > c | v <= c | v >= c | v == c | v != c | g && g | true

鍏朵腑 v 鏄?**V** 涓殑鍙橀噺锛宑 鏄暟鍊笺€?
鎴戜滑灏嗗彉閲忎互鍧囧寑閫熺巼澧為暱鐨勬贩鍚堣嚜鍔ㄦ満鐨勭壒渚嬪畾涔変负鏃堕棿鑷姩鏈猴紙timed automata锛夈€傚湪杩欑鎯呭喌涓嬶紝鍙橀噺琚О涓烘椂閽燂紙clock锛夈€傞【鍚嶆€濅箟锛屾椂闂磋嚜鍔ㄦ満鍙敤浜庢弿杩板疄鏃躲€傛澶栵紝鏃堕挓鏀寔鍙︿竴绉嶆€绘槸姹傚€间负鐪熺殑瀹堝崼锛?
        reset(v)

reset 绾︽潫鐢ㄤ簬灏嗘椂閽熺殑鍊艰涓?0銆?
涓嶅彉閲忕害鏉熼泦鍚?**C'(V)** 鏄?**C(V)** 鐨勫瓙闆嗭紝鍙寘鍚互涓嬪舰寮忕殑绾︽潫锛?
        g = v < c | true

杩欑畝鍖栦簡瀹炵幇锛屽洜涓烘椂閽熻繃鏈熸槸涓嶅彉閲忚杩濆弽鐨勫繀瑕佷笖鍏呭垎鏉′欢锛屽悓鏃朵粛鍏佽灏嗘洿澶嶆潅鐨勭害鏉熸寚瀹氫负瀹堝崼銆?
闇€瑕佹敞鎰忕殑鏄紝浠讳綍娣峰悎鑷姩鏈洪兘鏄竴涓甫鏈夐澶栧畧鍗拰涓嶅彉閲忕殑鏈夋晥纭畾鎬ц嚜鍔ㄦ満銆傝繖浜涘彧鑳借繘涓€姝ョ害鏉熷摢浜涜浆绉绘槸鏈夋晥鐨勶紝浣嗕笉鍙兘鍩轰簬 **V** 鐨勫彇鍊硷紝瀹氫箟浠?**X** 涓悓涓€鐘舵€佸拰 **E** 涓悓涓€浜嬩欢寮€濮嬪嵈浠?**X** 涓笉鍚岀姸鎬佺粨鏉熺殑杞Щ鍑芥暟銆?
### 绀轰緥

#### 浣滀负娣峰悎鑷姩鏈虹殑 Wip

浣滀负纭畾鎬ц嚜鍔ㄦ満寮曞叆鐨?鈥榳ip鈥欙紙wakeup in preemptive锛屾姠鍗犱腑鐨勫敜閱掞級绀轰緥涔熷彲浠ユ弿杩颁负锛?
- **X** = { `any_thread_running` }
- **E** = { `sched_waking` }
- **V** = { `preemptive` }
- x\ `0` = `any_thread_running`
- X\ `m` = {`any_thread_running`}
- **f** =
   - **f**\ (`any_thread_running`, `sched_waking`, `preemptive==0`) = `any_thread_running`
- **i** =
   - **i**\ (`any_thread_running`) = `true`

```
     |
     |
     v
   #====================#   sched_waking;preemptive==0
   H                    H ------------------------------+
   H any_thread_running H                               |
   H                    H <-----------------------------+
   #====================#

```
鍦ㄦ绀轰緥涓紝閫氳繃灏嗙郴缁熺殑鎶㈠崰鐘舵€佺敤浣滅幆澧冨彉閲忥紝鎴戜滑鍙互鍦ㄤ笉瑕佹眰鎶㈠崰浜嬩欢锛堟濡傛垜浠湪纭畾鎬ц嚜鍔ㄦ満涓墍鍋氱殑閭ｆ牱锛夌殑鎯呭喌涓嬶紝瀵?`sched_waking` 鏂█姝ょ害鏉燂紝杩欏湪閭ｄ簺浜嬩欢鍦ㄧ郴缁熶笂涓嶅彲鐢ㄦ垨涓嶅彲闈犳椂寰堟湁鐢ㄣ€?
鐢变簬 **i** 涓殑鎵€鏈変笉鍙橀噺閮戒负鐪燂紝鎴戜滑鍙互浠庤〃绀轰腑鐪佺暐瀹冧滑銆?
#### 甯﹀畧鍗殑鍋滄粸妯″瀷锛堣凯浠?1锛?
浣滀负鏃堕棿鑷姩鏈虹殑绀轰緥锛屾垜浠彲浠ュ皢 鈥榮tall鈥?瀹氫箟涓猴細

- **X** = { `dequeued`, `enqueued`, `running`}
- **E** = { `enqueue`, `dequeue`, `switch_in`}
- **V** = { `clk` }
- x\ `0` = `dequeue`
- X\ `m` = {`dequeue`}
- **f** =
   - **f**\ (`enqueued`, `switch_in`, `clk < threshold`) = `running`
   - **f**\ (`running`, `dequeue`) = `dequeued`
   - **f**\ (`dequeued`, `enqueue`, `reset(clk)`) = `enqueued`
- **i** = **鐪佺暐锛屽洜涓哄叏涓虹湡**

```
       |
       |
       v
     #============================#
     H          dequeued          H <+
     #============================#  |
       |                             |
       | enqueue; reset(clk)         |
       v                             |
     +----------------------------+  |
     |          enqueued          |  | dequeue
     +----------------------------+  |
       |                             |
       | switch_in; clk < threshold  |
       v                             |
     +----------------------------+  |
     |          running           | -+
     +----------------------------+

```
璇ユā鍨嬭瀹氾紝涓€涓换鍔′粠鍏ラ槦锛堝彉涓哄彲杩愯锛夊埌鐪熸杩愯涔嬮棿鐨勬椂闂村繀椤讳綆浜庢煇涓槇鍊笺€傝妯″瀷涓殑澶辫触鎰忓懗鐫€浠诲姟姝ｅ湪楗ラタ锛坰tarving锛夈€?鍦ㄨ繖绉嶆儏鍐典笅锛屽湪杈逛笂浣跨敤瀹堝崼鐨勪竴涓棶棰樻槸锛屾ā鍨嬪湪 `switch_in` 浜嬩欢鍙戠敓涔嬪墠涓嶄細鎶ュ憡澶辫触銆傝繖鎰忓懗鐫€锛屾牴鎹妯″瀷锛屼换鍔℃案杩滀笉杩愯涔熸槸鏈夋晥鐨勩€?
#### 甯︿笉鍙橀噺鐨勫仠婊炴ā鍨嬶紙杩唬 2锛?
绗竴娆¤凯浠ｅ苟涓嶅畬鍏ㄧ鍚堥鏈燂紝鎴戜滑鍙互灏嗘ā鍨嬫洿鏀逛负锛?
- **X** = { `dequeued`, `enqueued`, `running`}
- **E** = { `enqueue`, `dequeue`, `switch_in`}
- **V** = { `clk` }
- x\ `0` = `dequeue`
- X\ `m` = {`dequeue`}
- **f** =
   - **f**\ (`enqueued`, `switch_in`) = `running`
   - **f**\ (`running`, `dequeue`) = `dequeued`
   - **f**\ (`dequeued`, `enqueue`, `reset(clk)`) = `enqueued`
- **i** =
   - **i**\ (`enqueued`) = `clk < threshold`

```
    |
    |
    v
  #=========================#
  H        dequeued         H <+
  #=========================#  |
    |                          |
    | enqueue; reset(clk)      |
    v                          |
  +-------------------------+  |
  |        enqueued         |  |
  |    clk < threshold      |  | dequeue
  +-------------------------+  |
    |                          |
    | switch_in                |
    v                          |
  +-------------------------+  |
  |         running         | -+
  +-------------------------+

```
鍦ㄨ繖绉嶆儏鍐典笅锛屾垜浠皢瀹堝崼浣滀负涓嶅彉閲忕Щ鍒颁簡 `enqueued` 鐘舵€侊紝杩欐剰鍛崇潃鎴戜滑涓嶄粎绂佹鍦?`clk` 瓒呰繃闃堝€煎悗鍙戠敓 `switch_in`锛岃€屼笖濡傛灉鎴戜滑鍦ㄩ槇鍊间箣鍚?*浠嶇劧**澶勪簬 `enqueued` 鐘舵€侊紝涔熶細鏍囪涓烘棤鏁堛€傝妯″瀷鍦ㄤ换鍔￠ゥ楗跨殑閭ｄ竴鍒诲氨瀹為檯涓婂浜庢棤鏁堢姸鎬侊紝鑰屼笉鏄湪楗ラタ鐨勪换鍔℃渶缁堣繍琛屾椂銆?
### C 璇█涓殑娣峰悎鑷姩鏈?
C 璇█涓贩鍚堣嚜鍔ㄦ満鐨勫畾涔夊ぇ閲忓熀浜庣‘瀹氭€ц嚜鍔ㄦ満鐨勫畾涔夈€傚叿浣撴潵璇达紝鎴戜滑娣诲姞鐜鍙橀噺鐨勯泦鍚堜互鍙婄害鏉燂紙杞Щ涓婄殑瀹堝崼鍜岀姸鎬佷笂鐨勪笉鍙橀噺锛夛紝濡備笅鎵€绀恒€?
```
  /* 鐢ㄤ綔绱㈠紩鐨?X锛堢姸鎬侀泦鍚堬級鐨勬灇涓捐〃绀?*/
  enum states {
	dequeued,
	enqueued,
	running,
	state_max,
  };

  #define INVALID_STATE state_max

  /* 鐢ㄤ綔绱㈠紩鐨?E锛堜簨浠堕泦鍚堬級鐨勬灇涓捐〃绀?*/
  enum events {
	dequeue,
	enqueue,
	switch_in,
	event_max,
  };

  /* 鐢ㄤ綔绱㈠紩鐨?V锛堢幆澧冨彉閲忛泦鍚堬級鐨勬灇涓捐〃绀?*/
  enum envs {
	clk,
	env_max,
	env_max_stored = env_max,
  };

  struct automaton {
	char *state_names[state_max];                  // X: 鐘舵€侀泦鍚?	char *event_names[event_max];                  // E: 浜嬩欢鏈夐檺闆嗗悎
	char *env_names[env_max];                      // V: 鐜鍙橀噺鏈夐檺闆嗗悎
	unsigned char function[state_max][event_max];  // f: 杞Щ鍑芥暟
	unsigned char initial_state;                   // x_0: 鍒濆鐘舵€?	bool final_states[state_max];                  // X_m: 鏍囪鐘舵€侀泦鍚?  };

  struct automaton aut = {
	.state_names = {
		"dequeued",
		"enqueued",
		"running",
	},
	.event_names = {
		"dequeue",
		"enqueue",
		"switch_in",
	},
	.env_names = {
		"clk",
	},
	.function = {
		{ INVALID_STATE,      enqueued, INVALID_STATE },
		{ INVALID_STATE, INVALID_STATE,       running },
		{      dequeued, INVALID_STATE, INVALID_STATE },
	},
	.initial_state = dequeued,
	.final_states = { 1, 0, 0 },
  };

  static bool verify_constraint(enum states curr_state, enum events event,
                                enum states next_state)
  {
	bool res = true;

	/* 浣滀负 f 鐨勪竴閮ㄥ垎楠岃瘉瀹堝崼 */
	if (curr_state == enqueued && event == switch_in)
		res = get_env(clk) < threshold;
	else if (curr_state == dequeued && event == enqueue)
		reset_env(clk);

	/* 楠岃瘉 i 涓殑涓嶅彉閲?*/
	if (next_state == curr_state || !res)
		return res;
	if (next_state == enqueued)
		ha_start_timer_jiffy(ha_mon, clk, threshold_jiffies);
	else if (curr_state == enqueued)
		res = !ha_cancel_timer(ha_mon);
	return res;
  }

```
鍑芥暟 `verify_constraint`锛堟澶勪互绠€鍖栧舰寮忕粰鍑猴級妫€鏌ュ畧鍗€佹墽琛岄噸缃苟鍚姩瀹氭椂鍣紝浠ユ牴鎹鑼冮獙璇佷笉鍙橀噺锛岃繖浜涙棤娉曡交鏄撳湴琛ㄧず鍦?automaton 缁撴瀯浣撲腑銆傜敱浜庣幆澧冨彉閲忕殑澶嶆潅鎬э紝鐢ㄦ埛闇€瑕佹彁渚涜幏鍙栧拰閲嶇疆闈炲父瑙勬椂閽燂紙渚嬪鍏锋湁 ns 鎴?jiffy 绮掑害鐨勬椂閽燂級鐨勭幆澧冨彉閲忕殑鍑芥暟銆?鐢变簬涓嶅彉閲忎粎瀹氫箟涓烘椂閽熻繃鏈燂紙渚嬪 *clk < threshold*锛夛紝鍒拌揪杩涘叆鐘舵€佹椂姝﹁鐨勫畾鏃跺櫒杩囨湡瀹為檯涓婃剰鍛崇潃妯″瀷涓殑澶辫触骞惰Е鍙戜竴涓弽搴斻€傜寮€璇ョ姸鎬佷細鍋滄瀹氭椂鍣ㄣ€?
闇€瑕佹敞鎰忕殑鏄紝浣跨敤 hrtimer 瀹炵幇鐨勫畾鏃跺櫒浼氬紩鍏ュ紑閿€锛屽鏋滅洃瑙嗗櫒鏈夊涓疄渚嬶紙渚嬪鎵€鏈変换鍔★級锛岃繖鍙兘鎴愪负闂銆備娇鐢ㄥ畾鏃跺櫒杞紙`HA_TIMER_TYPE` 璁句负 `HA_TIMER_WHEEL`锛夊彲浠ラ檷浣庤繖绉嶅奖鍝嶏紝杩欎笉浼氭崯瀹虫ā鍨嬬殑鍑嗙‘鎬э紝鍥犱负鍦ㄥ洖璋冨欢杩熺殑鎯呭喌涓嬶紝鍦ㄧ鐢ㄥ畾鏃跺櫒涔嬪墠浼氭鏌ヤ笉鍙橀噺鏉′欢銆傛垨鑰咃紝濡傛灉淇濊瘉鐩戣鍣?*鏈€缁?*浼氱寮€璇ョ姸鎬侊紝涓旂瓑寰呬笅涓€涓簨浠舵墍浜х敓鐨勫欢杩熸槸鍙帴鍙楃殑锛屽垯鍙互浣跨敤瀹堝崼鏉ヤ唬鏇夸笉鍙橀噺锛屽 stall 绀轰緥鎵€绀恒€?
### Graphviz .dot 鏍煎紡

鍚屾牱鍦帮紝娣峰悎鑷姩鏈虹殑 Graphviz 琛ㄧず涔熸槸纭畾鎬ц嚜鍔ㄦ満琛ㄧず鐨勬墿灞曘€傚叿浣撴潵璇达紝瀹堝崼鍙互鍦ㄤ簨浠朵腑鎻愪緵

```
    "state_start" -> "state_dest" [ label = "sched_waking;preemptible==0;reset(clk)" ];

```
```
    "enqueued" [label = "enqueued\nclk < threshold_jiffies"];

```
绾︽潫鍙互鎸囧畾涓烘湁鏁堢殑 C 姣旇緝骞跺厑璁哥┖鏍硷紝姣旇緝鐨勭涓€涓厓绱犲繀椤绘槸鏃堕挓锛岃€岀浜屼釜鏄暟鍊兼垨鍙傛暟鍖栫殑鍊笺€傚畧鍗厑璁镐娇鐢ㄥ竷灏旇繍绠楋紙`&&` 鍜?`||`锛夌粍鍚堟瘮杈冿紝閲嶇疆蹇呴』涓庡叾浠栫害鏉熷垎寮€銆?
```
  digraph state_automaton {
      {node [shape = circle] "enqueued"};
      {node [shape = plaintext, style=invis, label=""] "__init_dequeued"};
      {node [shape = doublecircle] "dequeued"};
      {node [shape = circle] "running"};
      "__init_dequeued" -> "dequeued";
      "enqueued" [label = "enqueued\nclk < threshold_jiffies"];
      "running" [label = "running"];
      "dequeued" [label = "dequeued"];
      "enqueued" -> "running" [ label = "switch_in" ];
      "running" -> "dequeued" [ label = "dequeue" ];
      "dequeued" -> "enqueued" [ label = "enqueue;reset(clk)" ];
      { rank = min ;
          "__init_dequeued";
          "dequeued";
      }
  }

```
### 鍙傝€冩枃鐚?
```
  Christel Baier and Joost-Pieter Katoen: Principles of Model Checking,
  The MIT Press, 2008.

```
```
  Thomas Henzinger: The theory of hybrid automata,
  Proceedings 11th Annual IEEE Symposium on Logic in Computer Science, 1996.

```
