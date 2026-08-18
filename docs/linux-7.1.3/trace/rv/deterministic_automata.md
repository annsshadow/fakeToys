## Deterministic Automata


褰㈠紡涓婏紝涓€涓‘瀹氭€ц嚜鍔ㄦ満锛堣浣?G锛夎瀹氫箟涓轰竴涓簲鍏冪粍锛?
        **G** = { **X**, **E**, **f**, x\ `0`, X\ `m` }

鍏朵腑锛?
- **X** 鏄姸鎬佺殑闆嗗悎锛?- **E** 鏄簨浠剁殑鏈夐檺闆嗗悎锛?- x\ `0` 鏄垵濮嬬姸鎬侊紱
- X\ `m`锛?*X** 鐨勫瓙闆嗭級鏄爣璁帮紙鎴栨渶缁堬級鐘舵€佺殑闆嗗悎銆?- **f** : **X** x **E** -> **X** 鏄浆绉诲嚱鏁般€傚畠瀹氫箟浜嗗湪鐘舵€?**X** 涓彂鐢熸潵鑷?**E** 鐨勪簨浠舵椂鐨勭姸鎬佽浆绉汇€傚湪纭畾鎬ц嚜鍔ㄦ満鐨勭壒娈婃儏鍐典笅锛屽湪 **X** 涓殑鏌愪釜鐘舵€佷笅鍙戠敓 **E** 涓殑浜嬩欢锛屼細纭畾鎬у湴寰楀埌 **X** 涓殑涓嬩竴涓姸鎬併€?
渚嬪锛屼竴涓О涓?'wip'锛坵akeup in preemptive锛屾姠鍗犲紡鍞ら啋锛夌殑缁欏畾鑷姩鏈哄彲浠ュ畾涔変负锛?
- **X** = { `preemptive`, `non_preemptive`}
- **E** = { `preempt_enable`, `preempt_disable`, `sched_waking`}
- x\ `0` = `preemptive`
- X\ `m` = {`preemptive`}
- **f** =
   - **f**\ (`preemptive`, `preempt_disable`) = `non_preemptive`
   - **f**\ (`non_preemptive`, `sched_waking`) = `non_preemptive`
   - **f**\ (`non_preemptive`, `preempt_enable`) = `preemptive`

杩欑褰㈠紡鍖栧畾涔夌殑濂藉涔嬩竴鏄畠鍙互鐢ㄥ绉嶆牸寮忓憟鐜般€備緥濡傦紝浣跨敤瀵?*鎿嶄綔绯荤粺**浠庝笟鑰呴潪甯哥洿瑙傜殑銆佺敱椤剁偣锛堣妭鐐癸級鍜岃竟缁勬垚鐨?*鍥惧舰琛ㄧず**锛屼笖娌℃湁浠讳綍淇℃伅鎹熷け銆?
```

                       preempt_enable
          +---------------------------------+
          v                                 |
        #============#  preempt_disable   +------------------+
    --> H preemptive H -----------------> |  non_preemptive  |
        #============#                    +------------------+
                                            ^              |
                                            | sched_waking |
                                            +--------------+

```
### Deterministic Automaton in C


鍦ㄨ鏂?"Efficient formal verification for the Linux kernel" 涓紝浣滆€呮彁鍑轰簡涓€绉嶅湪 C 涓〃绀鸿嚜鍔ㄦ満鐨勭畝鍗曟柟娉曪紝璇ユ柟娉曞彲浣滀负 Linux 鍐呮牳涓殑甯歌浠ｇ爜浣跨敤銆?
```

  /* enum representation of X (set of states) to be used as index */
  enum states {
	preemptive = 0,
	non_preemptive,
	state_max
  };

  #define INVALID_STATE state_max

  /* enum representation of E (set of events) to be used as index */
  enum events {
	preempt_disable = 0,
	preempt_enable,
	sched_waking,
	event_max
  };

  struct automaton {
	char *state_names[state_max];                   // X: the set of states
	char *event_names[event_max];                   // E: the finite set of events
	unsigned char function[state_max][event_max];   // f: transition function
	unsigned char initial_state;                    // x_0: the initial state
	bool final_states[state_max];                   // X_m: the set of marked states
  };

  struct automaton aut = {
	.state_names = {
		"preemptive",
		"non_preemptive"
	},
	.event_names = {
		"preempt_disable",
		"preempt_enable",
		"sched_waking"
	},
	.function = {
		{ non_preemptive,  INVALID_STATE,  INVALID_STATE },
		{  INVALID_STATE,     preemptive, non_preemptive },
	},
	.initial_state = preemptive,
	.final_states = { 1, 0 },
  };

```
**杞Щ鍑芥暟**琛ㄧず涓虹姸鎬侊紙琛岋級鍜屼簨浠讹紙鍒楋級鐨勭煩闃碉紝鍥犳鍑芥暟 **f** : **X** x **E** -> **X** 鍙互閫氳繃浠ヤ笅鏂瑰紡姹傝В
```

  next_state = automaton_wip.function[curr_state][event];

```
### Graphviz .dot format


Graphviz 寮€婧愬伐鍏峰彲浠ヤ娇鐢紙鏂囨湰褰㈠紡鐨勶級DOT 璇█浣滀负婧愭潵鐢熸垚鑷姩鏈虹殑鍥惧舰琛ㄧず銆侱OT 鏍煎紡琚箍娉涗娇鐢紝骞朵笖鍙互杞崲涓鸿澶氬叾浠栨牸寮忋€?
```

  digraph state_automaton {
        {node [shape = circle] "non_preemptive"};
        {node [shape = plaintext, style=invis, label=""] "__init_preemptive"};
        {node [shape = doublecircle] "preemptive"};
        {node [shape = circle] "preemptive"};
        "__init_preemptive" -> "preemptive";
        "non_preemptive" [label = "non_preemptive"];
        "non_preemptive" -> "non_preemptive" [ label = "sched_waking" ];
        "non_preemptive" -> "preemptive" [ label = "preempt_enable" ];
        "preemptive" [label = "preemptive"];
        "preemptive" -> "non_preemptive" [ label = "preempt_disable" ];
        { rank = min ;
                "__init_preemptive";
                "preemptive";
        }
  }

```
杩欑 DOT 鏍煎紡鍙互浣跨敤 dot 宸ュ叿杞崲涓轰綅鍥炬垨鐭㈤噺鍥惧儚锛屾垨浣跨敤 graph-easy 杞崲涓?ASCII art銆傚浜?```

  $ dot -Tsvg -o wip.svg wip.dot
  $ graph-easy wip.dot > wip.txt

```
### dot2c


dot2c 鏄竴涓伐鍏凤紝鍙互瑙ｆ瀽鍖呭惈濡備笂渚嬫墍绀鸿嚜鍔ㄦ満鐨?.dot 鏂囦欢锛屽苟鑷姩灏嗗叾杞崲涓?[^3^] 涓粙缁嶇殑 C 琛ㄧず銆?
渚嬪锛屽皢鍓嶉潰鐨?'wip' 妯″瀷鏀惧叆鍚嶄负 'wip.dot' 鐨勬枃浠朵腑锛屼互涓嬪懡浠ゅ皢鎶?.dot 鏂囦欢杞崲涓?C
```

  $ dot2c wip.dot > wip.h

```
'wip.h' 鐨勫唴瀹瑰氨鏄?'Deterministic Automaton in C' 涓€鑺備腑鐨勪唬鐮佺ず渚嬨€?
### Remarks


鑷姩鏈哄舰寮忓寲鍏佽浠ュ绉嶆牸寮忓绂绘暎浜嬩欢绯荤粺锛圖ES锛夊缓妯★紝浠ラ€傚簲涓嶅悓鐨勫簲鐢?鐢ㄦ埛銆?
渚嬪锛屼娇鐢ㄩ泦鍚堣鐨勫舰寮忓寲鎻忚堪鏇撮€傚悎鑷姩鏈鸿繍绠楋紝鑰屽浘褰㈡牸寮忔洿閫傚悎浜哄伐瑙ｈ锛涜绠楁満璇█鍒欓€傚悎鏈哄櫒鎵ц銆?
### References


```

  O'Regan, Gerard. Concise guide to software engineering. Springer,
  Cham, 2017.

```
璇︾粏鎻忚堪锛堝寘鎷繍绠椾互鍙婂湪绂绘暎浜嬩欢绯荤粺涓婄殑搴旂敤锛夊彲鍙傝
```

  Cassandras, Christos G., and Stephane Lafortune, eds. Introduction to discrete
  event systems. Boston, MA: Springer US, 2008.

```

```

  De Oliveira, Daniel Bristot; Cucinotta, Tommaso; De Oliveira, Romulo
  Silva. Efficient formal verification for the Linux kernel. In:
  International Conference on Software Engineering and Formal Methods.
  Springer, Cham, 2019. p. 315-332.

```
