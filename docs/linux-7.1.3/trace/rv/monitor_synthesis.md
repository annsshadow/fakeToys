## 杩愯鏃堕獙璇佺洃瑙嗗櫒鍚堟垚


搴旂敤杩愯鏃堕獙璇侊紙RV锛夋妧鏈殑璧风偣鏄紝瀵瑰彈瀹℃煡绯荤粺鏈熸湜锛堟垨涓嶆湡鏈涳級鐨勮涓鸿繘琛?*瑙勭害
锛坰pecification锛?*鎴?*寤烘ā锛坢odeling锛?*銆?
鎺ヤ笅鏉ワ紝闇€瑕佸皢褰㈠紡鍖栬〃绀?*鍚堟垚锛坰ynthesized锛?*涓轰竴涓?*鐩戣鍣紙monitor锛?*锛岃鐩戣鍣?闅忓悗鍙敤浜庡垎鏋愮郴缁熺殑 trace銆傜洃瑙嗗櫒閫氳繃涓€涓?*鎻掓々锛坕nstrumentation锛?*杩炴帴鍒扮郴缁燂紝璇?鎻掓々灏嗘潵鑷?*绯荤粺**鐨勪簨浠惰浆鎹负**瑙勭害**鐨勪簨浠躲€?
鍦?Linux 鐨勮澧冧腑锛岃繍琛屾椂楠岃瘉鐩戣鍣ㄨ灏佽鍦?**RV monitor** 鎶借薄鍐呴儴銆俁V monitor 鍖呭惈
鐩戣鍣ㄧ殑涓€缁勫疄渚嬶紙姣?CPU 鐩戣鍣ㄣ€佹瘡浠诲姟鐩戣鍣ㄧ瓑锛夈€佸皢鐩戣鍣ㄤ笌绯荤粺鍙傝€冩ā鍨嬬矘鍚堢殑杈呭姪
鍑芥暟锛屼互鍙婁綔涓哄浜嬩欢瑙ｆ瀽鍜屽紓甯哥殑鍙嶅簲鐨?trace 杈撳嚭锛屽涓嬪浘鎵€绀?```

 Linux   +---- RV Monitor ----------------------------------+ Formal
  Realm  |                                                  |  Realm
  +-------------------+     +----------------+     +-----------------+
  |   Linux kernel    |     |     Monitor    |     |     Reference   |
  |     Tracing       |  -> |   Instance(s)  | <-  |       Model     |
  | (instrumentation) |     | (verification) |     | (specification) |
  +-------------------+     +----------------+     +-----------------+
         |                          |                       |
         |                          V                       |
         |                     +----------+                 |
         |                     | Reaction |                 |
         |                     +--+--+--+-+                 |
         |                        |  |  |                   |
         |                        |  |  +-> trace output ?  |
         +------------------------|--|----------------------+
                                  |  +----> panic ?
                                  +-------> <user-specified>

```

### RV 鐩戣鍣ㄥ悎鎴?

灏嗚绾﹀悎鎴愪负 Linux 鐨?**RV monitor** 鎶借薄锛岀敱 rvgen 宸ュ叿鍜屽寘鍚垱寤虹洃瑙嗗櫒鍏叡浠ｇ爜鐨勫ご鏂囦欢
鑷姩瀹屾垚銆傝繖浜涘ご鏂囦欢涓猴細

  - rv/da_monitor.h锛岀敤浜庣‘瀹氭€ц嚜鍔ㄦ満锛坉eterministic automaton锛夌洃瑙嗗櫒銆?  - rv/ltl_monitor.h锛岀敤浜庣嚎鎬ф椂鎬侀€昏緫锛坙inear temporal logic锛夌洃瑙嗗櫒銆?  - rv/ha_monitor.h锛岀敤浜庢贩鍚堣嚜鍔ㄦ満锛坔ybrid automaton锛夌洃瑙嗗櫒銆?
### rvgen


rvgen 宸ュ叿灏嗚绾﹁浆鎹负 C 琛ㄧず锛屽苟鐢熸垚 C 璇█鍐呮牳鐩戣鍣ㄧ殑楠ㄦ灦銆備緥濡傦紝鍙互杞崲瀛樺湪浜?```

  $ rvgen monitor -c da -s wip.dot -t per_cpu

```

涓殑 wip.dot 妯″瀷銆傝繖灏嗗垱寤轰竴涓悕涓?wip/ 鐨勭洰褰曪紝鍖呭惈浠ヤ笅鏂囦欢锛?
- wip.h锛欳 璇█褰㈠紡鐨?wip 妯″瀷
- wip.c锛歊V monitor

wip.c 鏂囦欢鍖呭惈鐩戣鍣ㄥ０鏄庝互鍙婄郴缁熸彃妗╃殑璧峰鐐广€?
绫讳技鍦帮紝鍙互鐢ㄤ互涓嬪懡浠ょ敓鎴愮嚎鎬ф椂鎬侀€昏緫鐩戣鍣?```

  $ rvgen monitor -c ltl -s pagefault.ltl -t per_task

```

杩欏皢鐢熸垚 pagefault/ 鐩綍锛屽寘鍚細

- pagefault.h锛欱uchi 鑷姩鏈猴紙鐢ㄤ簬楠岃瘉瑙勭害鐨勯潪纭畾鎬х姸鎬佹満锛?- pagefault.c锛歊V monitor 鐨勯鏋?
### 鐩戣鍣ㄥご鏂囦欢


澶存枃浠讹細

- `rv/da_monitor.h`锛岀敤浜庣‘瀹氭€ц嚜鍔ㄦ満鐩戣鍣?- `rv/ltl_monitor` 鐢ㄤ簬绾挎€ф椂鎬侀€昏緫鐩戣鍣?
鍖呭惈鐢ㄤ簬瀹炵幇*鐩戣鍣ㄥ疄渚嬶紙Monitor Instance(s)锛?鐨勫叕鍏卞畯鍜岄潤鎬佸嚱鏁般€?
灏嗘墍鏈夊叕鍏卞姛鑳芥斁鍦ㄥ崟涓ご鏂囦欢涓殑濂藉鏈変笁锛?
  - 鍑忓皯浠ｇ爜閲嶅锛?  - 渚夸簬淇/鏀硅繘锛?  - 閬垮厤寮€鍙戣€呬负锛堟瘮濡傝锛変互闈炴爣鍑嗘柟寮忔搷绾垫ā鍨嬭€屾敼鍔ㄧ洃瑙嗗櫒鏍稿績浠ｇ爜鐨勬儏鍐点€?
rv/da_monitor.h
+++++++++++++++

杩欎釜鍒濆瀹炵幇鎻愪緵浜嗕笁绉嶄笉鍚岀被鍨嬬殑鐩戣鍣ㄥ疄渚嬶細

- `#define RV_MON_TYPE RV_MON_GLOBAL`
- `#define RV_MON_TYPE RV_MON_PER_CPU`
- `#define RV_MON_TYPE RV_MON_PER_TASK`

绗竴绉嶄负鍏ㄥ眬纭畾鎬ц嚜鍔ㄦ満鐩戣鍣ㄥ０鏄庡嚱鏁帮紝绗簩绉嶄负姣?CPU 瀹炰緥鐨勭洃瑙嗗櫒锛岀涓夌涓烘瘡浠诲姟
瀹炰緥鐨勭洃瑙嗗櫒銆?
鍦ㄦ墍鏈夋儏鍐典笅锛孋 鏂囦欢蹇呴』鍖呭惈 `rvgen` 鐢熸垚鐨?$(MODEL_NAME).h 鏂囦欢锛堜緥濡傦紝瑕佸畾涔夋瘡 CPU 鐨?鈥渨ip鈥濈洃瑙嗗櫒锛宍wip.c` 婧愭枃浠跺簲
```

  #define RV_MON_TYPE RV_MON_PER_CPU
  #include "wip.h"
  #include <rv/da_monitor.h>

```

鐩戣鍣ㄩ€氳繃鍙戦€佸緟澶勭悊鐨勪簨浠舵潵鎵ц锛屼娇鐢ㄤ互涓嬪嚱鏁?```

  da_handle_event($(event from event enum));
  da_handle_start_event($(event from event enum));
  da_handle_start_run_event($(event from event enum));

```

鍑芥暟 `da_handle_event()` 鏄父瑙勬儏鍐碉紝鍗冲綋鐩戣鍣ㄦ鍦ㄥ鐞嗕簨浠舵椂浼氬鐞嗚浜嬩欢銆?
褰撶洃瑙嗗櫒琚惎鐢ㄦ椂锛屽畠琚疆浜庤嚜鍔ㄦ満鐨勫垵濮嬬姸鎬併€傜劧鑰岋紝鐩戣鍣ㄥ苟涓嶇煡閬撶郴缁熸槸鍚﹀浜?*鍒濆
鐘舵€?*銆?
`da_handle_start_event()` 鍑芥暟鐢ㄤ簬閫氱煡鐩戣鍣ㄧ郴缁熸鍦ㄨ繑鍥炲垵濮嬬姸鎬侊紝浠庤€岀洃瑙嗗櫒鍙互寮€濮?鐩戣涓嬩竴涓簨浠躲€?
`da_handle_start_run_event()` 鍑芥暟鐢ㄤ簬閫氱煡鐩戣鍣ㄧ郴缁熷凡鐭ュ浜庡垵濮嬬姸鎬侊紝浠庤€岀洃瑙嗗櫒鍙互
寮€濮嬬洃瑙嗗苟澶勭悊褰撳墠浜嬩欢銆?
浠?wip 妯″瀷涓轰緥锛屼簨浠?"preempt_disable" 鍜?```

  da_handle_event(preempt_disable_wip);
  da_handle_event(sched_waking_wip);

```

```

  da_handle_start_event(preempt_enable_wip);

```

鐢ㄤ簬閫氱煡鐩戣鍣ㄧ郴缁熷皢杩斿洖鍒濆鐘舵€侊紝浠庤€岀郴缁熶笌鐩戣鍣ㄥ簲褰撲繚鎸佸悓姝ャ€?
rv/ltl_monitor.h
++++++++++++++++
璇ユ枃浠跺繀椤讳笌 `rvgen` 鐢熸垚鐨?$(MODEL_NAME).h 鏂囦欢缁撳悎鎵嶅畬鏁淬€備緥濡傦紝瀵逛簬 `pagefault`
鐩戣鍣紝`pagefault.c` 搴?```

  #include "pagefault.h"
  #include <rv/ltl_monitor.h>

```

锛坄rvgen` 鐢熸垚鐨勯鏋剁洃瑙嗗櫒鏂囦欢宸茬粡杩欐牱鍋氫簡锛夈€?
`$(MODEL_NAME).h`锛堜笂渚嬩腑鐨?`pagefault.h`锛夊寘鍚?Buchi 鑷姩鏈虹殑瀹炵幇鈥斺€斾竴涓獙璇?LTL 瑙勭害
鐨勯潪纭畾鎬х姸鎬佹満銆傝€?`rv/ltl_monitor.h` 鍖呭惈涓?Buchi 鑷姩鏈轰氦浜掑苟瀹炵幇 RV 鐩戣鍣ㄧ殑鍏叡
杈呭姪鍑芥暟
```

  enum ltl_atom {
      LTL_$(FIRST_ATOMIC_PROPOSITION),
      LTL_$(SECOND_ATOMIC_PROPOSITION),
      ...
      LTL_NUM_ATOM
  };

```

杩欐槸 LTL 瑙勭害涓瓨鍦ㄧ殑鍘熷瓙鍛介锛坅tomic proposition锛夊垪琛紙甯︽湁鈥淟TL\_鈥濆墠缂€浠ラ伩鍏嶅懡鍚嶅啿绐侊級銆?杩欎釜 `enum` 琚紶閫掔粰涓?Buchi 鑷姩鏈轰氦浜掔殑鍑芥暟銆?
鐢熸垚浠ｇ爜鏃讹紝`rvgen` 鏃犳硶鐞嗚В鍘熷瓙鍛介鐨勫惈涔夈€傚洜姝よ浠诲姟鐣欑粰浜哄伐瀹屾垚銆傛帹鑽愮殑鍋氭硶鏄紝鍦?鍘熷瓙鍛介鍙戠敓鍙樺寲鐨勫湴鏂规坊鍔?tracepoints锛屽苟鍦?```

  void ltl_atom_update(struct task_struct *task, enum ltl_atom atom, bool value)

```

涓憡鐭?Buchi 鑷姩鏈哄師瀛愬懡棰?`atom` 鐜板湪涓?`value`銆侭uchi 鑷姩鏈烘鏌?LTL 瑙勭害鏄惁浠嶈
婊¤冻锛屽苟鍦ㄦ娴嬪埌杩濅緥鏃惰皟鐢ㄧ洃瑙嗗櫒鐨勯敊璇?tracepoint 鍜屽弽搴斿櫒锛坮eactor锛夈€?
搴斿敖鍙兘鍦颁娇鐢?tracepoints 鍜?`ltl_atom_update()`銆傜劧鑰岋紝鏈夋椂杩欏苟涓嶆槸鏈€鏂逛究鐨勬柟寮忋€?瀵逛簬鍦ㄥ唴鏍稿涓綅缃彂鐢熷彉鍖栫殑鏌愪簺鍘熷瓙鍛介锛岃拷韪墍鏈夎繖浜涗綅缃細寰堥夯鐑︺€傛澶栵紝鍘熷瓙鍛介鍦?绮剧‘鏃跺埢琚洿鏂板彲鑳藉苟涓嶉噸瑕併€備緥濡傦紝鑰冭檻浠ヤ笅绾挎€ф椂鎬?```

  RULE = always (RT imply not PAGEFAULT)

```

杩欎釜 LTL 琛ㄧず瀹炴椂浠诲姟涓嶄細寮曞彂椤甸敊璇紙page fault锛夈€傚浜庤瑙勭害锛屽綋 `PAGEFAULT` 涓虹湡鏃讹紝
`RT` 鍏锋湁姝ｇ‘鐨勫€煎嵆鍙紝鑷充簬 `RT` 浣曟椂鏀瑰彉骞朵笉閲嶈銆傚彈姝ゆ儏褰㈠惎鍙戯紝鎻愪緵浜嗗彟涓€涓?```

  void ltl_atom_fetch(struct task_struct *task, struct ltl_monitor *mon)

```

璇ュ嚱鏁版瘡褰?Buchi 鑷姩鏈鸿瑙﹀彂鏃惰皟鐢ㄣ€傚洜姝わ紝
```

  void ltl_atom_fetch(struct task_struct *task, struct ltl_monitor *mon)
  {
      ltl_atom_set(mon, LTL_RT, rt_task(task));
  }

```

瀹為檯涓婏紝姣忓綋閫氳繃璋冪敤 `ltl_atom_update()` 鏇存柊 `PAGEFAULT` 鏃讹紝`RT` 涔熶細琚幏鍙栥€傚洜姝わ紝LTL
瑙勭害鍙互鍦ㄤ笉杩借釜鍚勫 `RT` 鐨勬儏鍐典笅琚獙璇併€?
瀵逛簬琛ㄧ幇寰楀儚浜嬩欢鐨勫師瀛愬懡棰橈紝瀹冧滑閫氬父闇€瑕佸湪璁剧疆锛堟垨娓呴櫎锛夊悗绔嬪嵆娓呴櫎锛堟垨璁剧疆锛夈€備竴涓?鏂逛究鐨勫嚱鏁版槸
```

  void ltl_atom_pulse(struct task_struct *task, enum ltl_atom atom, bool value)

```

```

  ltl_atom_update(task, atom, value);
  ltl_atom_update(task, atom, !value);

```

瑕佸垵濮嬪寲鍘熷瓙鍛介锛屽繀椤讳娇鐢ㄤ互涓嬪嚱鏁?```

  ltl_atoms_init(struct task_struct *task, struct ltl_monitor *mon, bool task_creation)

```

褰撶洃瑙嗗櫒琚惎鐢ㄦ椂锛岃鍑芥暟涓烘墍鏈夎繍琛屼腑鐨勪换鍔¤皟鐢ㄣ€傚畠涔熶細涓哄惎鐢ㄧ洃瑙嗗櫒鍚庡垱寤虹殑鏂颁换鍔¤皟鐢ㄣ€?瀹冨簲
```

  void ltl_atom_init(struct task_struct *task, struct ltl_monitor *mon, bool task_creation)
  {
      ltl_atom_set(mon, LTL_RT, rt_task(task));
      if (task_creation)
          ltl_atom_set(mon, LTL_PAGEFAULT, false);
  }

```

鏈 `ltl_atom_init()` 鍒濆鍖栫殑鍘熷瓙鍛介灏嗗仠鐣欏湪鏈煡鐘舵€侊紝鐩村埌鍛戒腑鐩稿叧鐨?tracepoints锛岃繖
鍙兘闇€瑕佷竴浜涙椂闂淬€傜敱浜庡湪浠诲姟鐨勫叏閮ㄥ師瀛愬懡棰橀兘宸茬煡涔嬪墠鏃犳硶瀵瑰叾鎵ц鐩戣锛岀洃瑙嗗櫒鍙兘闇€瑕?涓€浜涙椂闂存潵寮€濮嬮獙璇佸湪鐩戣鍣ㄥ惎鐢ㄤ箣鍓嶅氨宸茶繍琛岀殑浠诲姟銆傚洜姝わ紝寤鸿鍦ㄥ惎鐢ㄧ洃瑙嗗櫒涔嬪悗鍐嶅惎鍔?鎰熷叴瓒ｇ殑浠诲姟銆?
rv/ha_monitor.h
+++++++++++++++

娣峰悎鑷姩鏈虹洃瑙嗗櫒鐨勫疄鐜扮洿鎺ユ淳鐢熻嚜纭畾鎬ц嚜鍔ㄦ満銆傚敖绠′娇鐢ㄤ簡涓嶅悓鐨勫ご鏂囦欢锛坄ha_monitor.h`锛夛紝
澶勭悊浜嬩欢鐨勫嚱鏁版槸鐩稿悓鐨勶紙渚嬪 `da_handle_event`锛夈€?
姝ゅ锛宍rvgen` 宸ュ叿浼氭牴鎹洃瑙嗗櫒婧愭枃浠朵腑鐨勭洃瑙嗗櫒瑙勭害锛屼负 `ha_verify_constraint`銆?`ha_get_env` 鍜?`ha_reset_env` 濉厖楠ㄦ灦銆?
`ha_verify_constraint` 閫氬父寮€绠卞嵆鐢紝鍥犱负瀹冪敱 `rvgen` 鐢熸垚锛?
```

    res = ha_get_env(ha_mon, ENV) < VALUE;

```

```

    ha_reset_env(ha_mon, ENV);

```

- 鐘舵€佷笂鐨勭害鏉熶娇鐢ㄥ畾鏃跺櫒瀹炵幇

  - 鍦ㄨ繘鍏ョ姸鎬佸墠姝﹁锛坅rmed锛?
  - 鍦ㄨ繘鍏ヤ换浣曞叾浠栫姸鎬佹椂鍙栨秷

  - 濡傛灉浜嬩欢鏈鑷寸姸鎬佹敼鍙樺垯淇濇寔涓嶅彉

  - 濡傛灉瀹氭椂鍣ㄥ埌鏈熶絾鍥炶皟鏈繍琛屽垯妫€鏌?
  - 鍙敤鐨勫疄鐜版湁 `HA_TIMER_HRTIMER` 鍜?`HA_TIMER_WHEEL`

    - hrtimer 鏇寸簿纭絾鍙兘鏈夋洿楂樺紑閿€

```

      #define HA_TIMER_TYPE HA_TIMER_HRTIMER

```

绾︽潫鍊煎彲浠ョ敤涓嶅悓褰㈠紡鎸囧畾锛?
```

    preemptive == 0
    clk < 100ns
    threshold <= 10j

```

```

    clk < MAX_NS

```

```

    clk <= threshold_jiffies

```

```

    clk < MAX_NS()

```

```

    clk <= threshold_jiffies()

```

鍦ㄦ墍鏈夋儏鍐典笅锛宍rvgen` 浼氬皾璇曚粠鍚嶇О鎴栧崟浣嶅垽鏂幆澧冨彉閲忕殑绫诲瀷銆備緥濡傦紝浠?`_NS` 鎴?`_jiffies`
缁撳熬鐨勫父閲忔垨鍙傛暟鍒嗗埆琚綋浣?ns 鍜?jiffy 绮掑害鐨勬椂閽熴€傚甫鏈夊害閲忓崟浣?`j` 鐨勫瓧闈㈤噺鏄?jiffies锛?濡傛灉鎸囧畾浜嗘椂闂村崟浣嶏紙`ns` 鍒?`s`锛夛紝`rvgen` 浼氬皢鍊艰浆鎹负 `ns`銆?
甯搁噺闇€瑕佺敱鐢ㄦ埛瀹氫箟锛堜絾涓庡悕绉颁笉鍚岋紝瀹冧滑涓嶄竴瀹氶渶瑕佸畾涔変负甯搁噺锛夈€傚弬鏁颁細琚浆鎹负妯″潡鍙傛暟锛?鐢ㄦ埛闇€瑕佹彁渚涢粯璁ゅ€笺€傚嚱鏁板拰瀹忓悓鏍风敱鐢ㄦ埛瀹氫箟锛岄粯璁ゆ儏鍐典笅瀹冧滑浠?`ha_monitor` 浣滀负鍙傛暟锛屽父瑙?鐨勭敤娉曟槸閫氳繃杈呭姪鍑芥暟 `ha_get_target(ha_mon)` 浠庣洰鏍囷紙渚嬪姣忎换鍔＄洃瑙嗗櫒涓殑 task锛夎幏鍙?鎵€闇€鍊笺€?
濡傛灉 `rvgen` 纭畾璇ュ彉閲忔槸鏃堕挓锛屽畠浼氭牴鎹崟浣嶆彁渚?getter 鍜?resetter銆傚惁鍒欙紝鐢ㄦ埛闇€瑕佹彁渚?閫傚綋鐨勫畾涔夈€傞€氬父闈炴椂閽熺殑鐜鍙橀噺涓嶄細琚噸缃€傚湪杩欑鎯呭喌涓嬶紝鐢?`rvgen` 鐢熸垚鐨勬枃浠朵腑鍙細
瀛樺湪 getter 楠ㄦ灦銆?```

  static u64 ha_get_env(struct ha_monitor *ha_mon, enum envs env)
  {
      if (env == preemptible)
          return preempt_count() == 0;
      return ENV_INVALID_VALUE;
  }

```

璇ュ嚱鏁颁紶鍏?`ha_mon` 鍙傛暟锛屼互澶囬渶瑕佸瓨鍌紙瀵规椂閽熻€岃█灏辨槸杩欑鎯呭喌锛夛紝浣嗘棤闇€閲嶇疆鐨勭幆澧冨彉閲?涓嶉渶瑕佸瓨鍌紝鍙互蹇界暐璇ュ弬鏁般€傞渶瑕佸瓨鍌ㄧ殑鐜鍙橀噺鏁伴噺鍙?`MAX_HA_ENV_LEN` 闄愬埗锛屼絾璇ラ檺鍒?涓嶉€傜敤浜庡叾浠栧彉閲忋€?
鏈€鍚庯紝鐘舵€佷笂鐨勭害鏉熶粎瀵规椂閽熸湁鏁堬紝涓斿彧鏈夊綋绾︽潫褰㈠ `clk < N` 鏃舵湁鏁堛€傝繖鏄洜涓烘绫荤害鏉熸槸
閫氳繃瀹氭椂鍣ㄥ埌鏈熷疄鐜扮殑銆傞€氬父鏃堕挓鍙橀噺鍦ㄦ瑁呭畾鏃跺櫒涔嬪墠琚噸缃紝浣嗕笉涓€瀹氶潪寰楀姝わ紝鍙敤鍑芥暟浼?澶勭悊濂借繖涓€鐐广€傜‘淇濅换鍔￠€€鍑烘椂娌℃湁瀹氭椂鍣ㄤ粛鍦ㄨ繍琛岋紝鏄瘡浠诲姟鐩戣鍣ㄧ殑璐ｄ换銆?
榛樿鎯呭喌涓嬬敓鎴愬櫒浣跨敤 hrtimer 瀹炵幇瀹氭椂鍣紙灏?`HA_TIMER_TYPE` 璁句负 `HA_TIMER_HRTIMER`锛夛紝
杩欒兘甯︽潵鏇村ソ鐨勫搷搴旀€т絾鏇撮珮鐨勫紑閿€銆傚畾鏃跺櫒杞紙timer wheel锛宍HA_TIMER_WHEEL`锛夊浜庡叿鏈夊涓?瀹炰緥锛堜緥濡傛瘡浠诲姟锛夌殑鐩戣鍣ㄦ槸涓€涓笉閿欑殑鏇夸唬鏂规锛屽畠鑳藉湪澧炲姞寤惰繜鐨勫悓鏃跺疄鐜版洿浣庣殑寮€閿€锛屼笖
涓嶇壓鐗茬簿搴︺€?
### 鏈€鍚庤鏄?

鏈変簡鍩轰簬澶存枃浠跺拰 rvgen 鐨勭洃瑙嗗櫒鍚堟垚锛屽紑鍙戣€呯殑宸ヤ綔搴斾粎闄愪簬瀵圭郴缁熸彃妗╋紝浠庤€屾彁鍗囨暣浣撴柟妗?鐨勫彲淇″害銆?
[^1^] 鍏充簬纭畾鎬ц嚜鍔ㄦ満鏍煎紡鍙婂叾杞崲鐨勭粏鑺傦紝璇峰弬闃?```

  Documentation/trace/rv/deterministic_automata.rst

```

[^2^] rvgen 浼氬皢鐩戣鍣ㄥ悕绉板悗缂€杩藉姞鍒颁簨浠舵灇涓句笂锛屼互閬垮厤鍦ㄥ鍑轰緵 BPF 绋嬪簭浣跨敤鐨勫叏灞€
vmlinux.h 鏃跺嚭鐜板彉閲忓啿绐併€?