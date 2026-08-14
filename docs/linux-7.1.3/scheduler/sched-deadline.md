## 鎴鏈熶换鍔¤皟搴?

    0. 璀﹀憡锛圵ARNING锛?    1. 姒傝堪锛圤verview锛?    2. 璋冨害绠楁硶锛圫cheduling algorithm锛?      2.1 涓荤畻娉曪紙Main algorithm锛?      2.2 甯﹀鍥炴敹锛圔andwidth reclaiming锛?    3. 璋冨害瀹炴椂浠诲姟锛圫cheduling Real-Time Tasks锛?      3.1 瀹氫箟锛圖efinitions锛?      3.2 鍗曞鐞嗗櫒绯荤粺鐨勫彲璋冨害鎬у垎鏋愶紙Schedulability Analysis for Uniprocessor Systems锛?      3.3 澶氬鐞嗗櫒绯荤粺鐨勫彲璋冨害鎬у垎鏋愶紙Schedulability Analysis for Multiprocessor Systems锛?      3.4 涓?SCHED_DEADLINE 鍙傛暟鐨勫叧绯伙紙Relationship with SCHED_DEADLINE Parameters锛?    4. 甯﹀绠＄悊锛圔andwidth management锛?      4.1 绯荤粺绾ц缃紙System-wide settings锛?      4.2 浠诲姟鎺ュ彛锛圱ask interface锛?      4.3 榛樿琛屼负锛圖efault behavior锛?      4.4 sched_yield() 鐨勮涓猴紙Behavior of sched_yield()锛?    5. 浠诲姟鐨?CPU 浜插拰鎬э紙Tasks CPU affinity锛?      5.1 浣跨敤 cgroup v1 cpuset 鎺у埗鍣紙Using cgroup v1 cpuset controller锛?      5.2 浣跨敤 cgroup v2 cpuset 鎺у埗鍣紙Using cgroup v2 cpuset controller锛?    6. 鏈潵璁″垝锛團uture plans锛?    A. 娴嬭瘯濂椾欢锛圱est suite锛?    B. 鏈€灏?main()锛圡inimal main()锛?

## 0. 璀﹀憡


 闅忔剰鏀瑰姩杩欎簺璁剧疆鍙兘瀵艰嚧绯荤粺琛屼负涓嶅彲棰勬祴鐢氳嚦涓嶇ǔ瀹氥€傚浜?-rt锛堢粍锛夎皟搴︼紝鍋囧畾 root 鐢ㄦ埛娓呮鑷繁鍦ㄥ仛浠€涔堛€?

## 1. 姒傝堪


 sched_dl 璋冨害绫讳腑鐨?SCHED_DEADLINE 绛栫暐鏈川涓婃槸 Earliest Deadline First锛圗DF锛屾渶鏃╂埅姝㈡湡浼樺厛锛夎皟搴︾畻娉曠殑瀹炵幇锛屽苟杈呬互涓€绉嶆満鍒讹紙绉颁负 Constant Bandwidth Server锛孋BS锛屾亽瀹氬甫瀹芥湇鍔″櫒锛夛紝浣垮緱浠诲姟涔嬮棿鐨勮涓鸿兘澶熺浉浜掗殧绂汇€?

## 2. 璋冨害绠楁硶


### 2.1 涓荤畻娉?

 SCHED_DEADLINE [^18^] 浣跨敤涓変釜鍙傛暟锛屽垎鍒负 "runtime"锛堣繍琛屾椂闂达級銆?period"锛堝懆鏈燂級鍜?"deadline"锛堟埅姝㈡湡锛夛紝瀵逛换鍔¤繘琛岃皟搴︺€備竴涓?SCHED_DEADLINE 浠诲姟搴斿綋姣?"period" 寰鑾峰緱 "runtime" 寰鐨勬墽琛屾椂闂达紝涓旇繖浜?"runtime" 寰鍦ㄥ懆鏈熷紑濮嬪悗鐨?"deadline" 寰涔嬪唴鍙敤銆備负浜嗗疄鐜拌繖涓€琛屼负锛屾瘡娆′换鍔¤鍞ら啋鏃讹紝璋冨害鍣ㄩ兘浼氫緷鎹繚璇侊紙浣跨敤 CBS[2,3] 绠楁硶锛夎绠椾竴涓?"璋冨害鎴鏈?锛坰cheduling deadline锛夈€傞殢鍚庝换鍔′緷鎹繖浜涜皟搴︽埅姝㈡湡閲囩敤 EDF[^1^] 杩涜璋冨害锛堥€夋嫨璋冨害鎴鏈熸渶鏃╃殑浠诲姟鎵ц锛夈€傝娉ㄦ剰锛屽彧鏈夊湪浣跨敤浜嗘伆褰撶殑 "鍑嗗叆鎺у埗"锛坅dmission control锛屽弬瑙佺 "4. 甯﹀绠＄悊" 鑺傦級绛栫暐鏃讹紝浠诲姟鎵嶈兘鍦ㄥ疄闄呯殑 "deadline" 鍐呰幏寰?"runtime" 鏃堕棿鍗曚綅锛堟樉鐒讹紝鑻ョ郴缁熻繃杞斤紝杩欎竴淇濊瘉鏃犳硶琚伒瀹堬級銆?
 鎬昏€岃█涔嬶紝CBS[2,3] 绠楁硶涓轰换鍔″垎閰嶈皟搴︽埅姝㈡湡锛屼娇寰楁瘡涓换鍔″湪姣忎釜鍛ㄦ湡鍐呮渶澶氳繍琛屽叾 runtime锛屼粠鑰岄伩鍏嶄笉鍚屼换鍔′箣闂寸殑鐩镐簰骞叉壈锛堝甫瀹介殧绂伙級锛涜€?EDF[^1^] 绠楁硶鍒欓€夋嫨璋冨害鎴鏈熸渶鏃╃殑浠诲姟浣滀负涓嬩竴涓鎵ц鐨勪换鍔°€傚緱鐩婁簬杩欎竴鐗规€э紝閭ｄ簺骞朵笉瀹屽叏绗﹀悎 "浼犵粺" 瀹炴椂浠诲姟妯″瀷锛堝弬瑙佺 3 鑺傦級鐨勪换鍔′篃鑳芥湁鏁堝湴浣跨敤杩欎竴鏂扮瓥鐣ャ€?
 鏇磋缁嗗湴璇达紝CBS 绠楁硶鎸夌収浠ヤ笅鏂瑰紡涓轰换鍔″垎閰嶈皟搴︽埅姝㈡湡锛?
  - 姣忎釜 SCHED_DEADLINE 浠诲姟鐢?"runtime"銆?deadline" 鍜?"period" 鍙傛暟鎵€鍒荤敾锛?
  - 浠诲姟鐨勭姸鎬佺敱涓€涓?"璋冨害鎴鏈? 鍜屼竴涓?"鍓╀綑杩愯鏃堕棿" 鎻忚堪銆傝繖涓や釜鍙傛暟鍒濆琚涓?0锛?
  - 褰撲竴涓?SCHED_DEADLINE 浠诲姟琚敜閱掞紙鍙樹负鍙墽琛岀姸鎬侊級鏃讹紝

```
                 remaining runtime                  runtime
        ----------------------------------    >    ---------
        scheduling deadline - current time           period

    then, if the scheduling deadline is smaller than the current time, or
    this condition is verified, the scheduling deadline and the
    remaining runtime are re-initialized as

         scheduling deadline = current time + deadline
         remaining runtime = runtime

    otherwise, the scheduling deadline and the remaining runtime are
    left unchanged;

  - When a SCHED_DEADLINE task executes for an amount of time t, its
    remaining runtime is decreased as::

         remaining runtime = remaining runtime - t

    (technically, the runtime is decreased at every tick, or when the
    task is descheduled / preempted);

  - When the remaining runtime becomes less or equal than 0, the task is
    said to be "throttled" (also known as "depleted" in real-time literature)
    and cannot be scheduled until its scheduling deadline. The "replenishment
    time" for this task (see next item) is set to be equal to the current
    value of the scheduling deadline;

  - When the current time is equal to the replenishment time of a
    throttled task, the scheduling deadline and the remaining runtime are
    updated as::

         scheduling deadline = scheduling deadline + period
         remaining runtime = remaining runtime + runtime

 The SCHED_FLAG_DL_OVERRUN flag in sched_attr's sched_flags field allows a task
 to get informed about runtime overruns through the delivery of SIGXCPU
 signals.

```

### 2.2 甯﹀鍥炴敹


 鎴鏈熶换鍔＄殑甯﹀鍥炴敹鍩轰簬 GRUB锛圙reedy Reclamation of Unused Bandwidth锛屾湭浣跨敤甯﹀鐨勮椽濠洖鏀讹級绠楁硶 [15, 16, 17]锛屽苟鍦ㄨ缃?SCHED_FLAG_RECLAIM 鏍囧織鏃跺惎鐢ㄣ€?
```

                             ------------
                 (d)        |   Active   |
              ------------->|            |
              |             | Contending |
              |              ------------
              |                A      |
          ----------           |      |
         |          |          |      |
         | Inactive |          |(b)   | (a)
         |          |          |      |
          ----------           |      |
              A                |      V
              |              ------------
              |             |   Active   |
              --------------|     Non    |
                 (c)        | Contending |
                             ------------

 A task can be in one of the following states:

  - ActiveContending: if it is ready for execution (or executing);

  - ActiveNonContending: if it just blocked and has not yet surpassed the 0-lag
    time;

  - Inactive: if it is blocked and has surpassed the 0-lag time.

 State transitions:

  (a) When a task blocks, it does not become immediately inactive since its
      bandwidth cannot be immediately reclaimed without breaking the
      real-time guarantees. It therefore enters a transitional state called
      ActiveNonContending. The scheduler arms the "inactive timer" to fire at
      the 0-lag time, when the task's bandwidth can be reclaimed without
      breaking the real-time guarantees.

      The 0-lag time for a task entering the ActiveNonContending state is
      computed as::

                        (runtime * dl_period)
             deadline - ---------------------
                             dl_runtime

      where runtime is the remaining runtime, while dl_runtime and dl_period
      are the reservation parameters.

  (b) If the task wakes up before the inactive timer fires, the task re-enters
      the ActiveContending state and the "inactive timer" is canceled.
      In addition, if the task wakes up on a different runqueue, then
      the task's utilization must be removed from the previous runqueue's active
      utilization and must be added to the new runqueue's active utilization.
      In order to avoid races between a task waking up on a runqueue while the
      "inactive timer" is running on a different CPU, the "dl_non_contending"
      flag is used to indicate that a task is not on a runqueue but is active
      (so, the flag is set when the task blocks and is cleared when the
      "inactive timer" fires or when the task  wakes up).

  (c) When the "inactive timer" fires, the task enters the Inactive state and
      its utilization is removed from the runqueue's active utilization.

  (d) When an inactive task wakes up, it enters the ActiveContending state and
      its utilization is added to the active utilization of the runqueue where
      it has been enqueued.

 For each runqueue, the algorithm GRUB keeps track of two different bandwidths:

  - Active bandwidth (running_bw): this is the sum of the bandwidths of all
    tasks in active state (i.e., ActiveContending or ActiveNonContending);

  - Total bandwidth (this_bw): this is the sum of all tasks "belonging" to the
    runqueue, including the tasks in Inactive state.

  - Maximum usable bandwidth (max_bw): This is the maximum bandwidth usable by
    deadline tasks and is currently set to the RT capacity.


 The algorithm reclaims the bandwidth of the tasks in Inactive state.
 It does so by decrementing the runtime of the executing task Ti at a pace equal
 to

           dq = -(max{ Ui, (Umax - Uinact - Uextra) } / Umax) dt

 where:

  - Ui is the bandwidth of task Ti;
  - Umax is the maximum reclaimable utilization (subjected to RT throttling
    limits);
  - Uinact is the (per runqueue) inactive utilization, computed as
    (this_bq - running_bw);
  - Uextra is the (per runqueue) extra reclaimable utilization
    (subjected to RT throttling limits).


 Let's now see a trivial example of two deadline tasks with runtime equal
 to 4 and period equal to 8 (i.e., bandwidth equal to 0.5)::

         A            Task T1
         |
         |                               |
         |                               |
         |--------                       |----
         |       |                       V
         |---|---|---|---|---|---|---|---|--------->t
         0   1   2   3   4   5   6   7   8


         A            Task T2
         |
         |                               |
         |                               |
         |       ------------------------|
         |       |                       V
         |---|---|---|---|---|---|---|---|--------->t
         0   1   2   3   4   5   6   7   8


         A            running_bw
         |
       1 -----------------               ------
         |               |               |
      0.5-               -----------------
         |                               |
         |---|---|---|---|---|---|---|---|--------->t
         0   1   2   3   4   5   6   7   8


  - Time t = 0:

    Both tasks are ready for execution and therefore in ActiveContending state.
    Suppose Task T1 is the first task to start execution.
    Since there are no inactive tasks, its runtime is decreased as dq = -1 dt.

  - Time t = 2:

    Suppose that task T1 blocks
    Task T1 therefore enters the ActiveNonContending state. Since its remaining
    runtime is equal to 2, its 0-lag time is equal to t = 4.
    Task T2 start execution, with runtime still decreased as dq = -1 dt since
    there are no inactive tasks.

  - Time t = 4:

    This is the 0-lag time for Task T1. Since it didn't woken up in the
    meantime, it enters the Inactive state. Its bandwidth is removed from
    running_bw.
    Task T2 continues its execution. However, its runtime is now decreased as
    dq = - 0.5 dt because Uinact = 0.5.
    Task T2 therefore reclaims the bandwidth unused by Task T1.

  - Time t = 8:

    Task T1 wakes up. It enters the ActiveContending state again, and the
    running_bw is incremented.


```

### 2.3 鑳芥晥鎰熺煡璋冨害


 褰撻€夋嫨 cpufreq 鐨?schedutil 璋冩帶鍣紙governor锛夋椂锛孲CHED_DEADLINE 浼氬疄鐜?GRUB-PA [^19^] 绠楁硶锛屽皢 CPU 宸ヤ綔棰戠巼闄嶄綆鍒颁粛鑳芥弧瓒虫埅姝㈡湡鐨勬渶灏忓€笺€傝琛屼负鐩墠浠呴拡瀵?ARM 鏋舵瀯瀹炵幇銆?
 鑻ユ敼鍙橀鐜囨墍闇€鐨勬椂闂翠笌棰勭暀鍛ㄦ湡澶勪簬鍚屼竴鏁伴噺绾э紝鍒欓渶鏍煎娉ㄦ剰銆傚湪杩欑鎯呭喌涓嬶紝璁剧疆鍥哄畾鐨?CPU 棰戠巼鍙嶈€屼細甯︽潵鏇村皯鐨勬埅姝㈡湡閿欏け銆?

## 3. 璋冨害瀹炴椂浠诲姟



 ..  BIG FAT WARNING ******************************************************

```

   This section contains a (not-thorough) summary on classical deadline
   scheduling theory, and how it applies to SCHED_DEADLINE.
   The reader can "safely" skip to Section 4 if only interested in seeing
   how the scheduling policy can be used. Anyway, we strongly recommend
   to come back here and continue reading (once the urge for testing is
   satisfied :P) to be sure of fully understanding all technical details.

 .. ************************************************************************

```

 浠讳綍绫诲瀷鐨勪换鍔￠兘鍙互鍒╃敤杩欎竴鏂扮殑璋冨害鏈哄埗锛屽敖绠″簲璇ヨ瀹冪壒鍒€傚悎閭ｄ簺闇€瑕佸鏃跺簭琛屼负鎻愪緵淇濊瘉鐨勫懆鏈熸€ф垨闆舵槦锛坰poradic锛夊疄鏃朵换鍔★紝渚嬪澶氬獟浣撱€佹祦濯掍綋銆佹帶鍒跺簲鐢ㄧ瓑銆?

### 3.1 瀹氫箟


 涓€涓吀鍨嬬殑瀹炴椂浠诲姟鐢变竴绯诲垪璁＄畻闃舵锛堜换鍔″疄渚嬶紝鎴栫О浣滀笟锛宩obs锛夌殑閲嶅缁勬垚锛岃繖浜涢樁娈典互鍛ㄦ湡鎬ф垨闆舵槦锛坰poradic锛夌殑鏂瑰紡琚縺娲汇€傛瘡涓綔涓?J_j锛堝叾涓?J_j 鏄换鍔＄殑绗?j 涓綔涓氾級鐢卞埌杈炬椂闂?r_j锛堜綔涓氬紑濮嬬殑鏃堕棿锛夈€佸畬鎴愪綔涓氭墍闇€鐨勮绠楁椂闂?c_j锛屼互鍙婁綔涓氱殑缁濆鎴鏈?d_j锛堜綔涓氬簲褰撳湪璇ユ椂闂翠箣鍓嶅畬鎴愶級鎵€鍒荤敾銆傛渶澶ф墽琛屾椂闂?max{c_j} 琚О涓鸿浠诲姟鐨?"鏈€鍧忔儏鍐垫墽琛屾椂闂?锛圵orst Case Execution Time锛學CET锛夈€傚鏋?r_{j+1} = r_j + P锛屽垯瀹炴椂浠诲姟鍙互鏄懆鏈熶负 P 鐨勫懆鏈熶换鍔★紱鎴栬€呬互鏈€灏忓埌杈鹃棿闅?P 婊¤冻 r_{j+1} >= r_j + P 鐨勯浂鏄熶换鍔°€傛渶鍚庯紝d_j = r_j + D锛屽叾涓?D 鏄换鍔＄殑鐩稿鎴鏈熴€傛€昏€岃█涔嬶紝涓€涓疄鏃朵换鍔″彲浠ユ弿杩颁负

	Task = (WCET, D, P)

 瀹炴椂浠诲姟鐨勫埄鐢ㄧ巼锛坲tilization锛夊畾涔変负鍏?WCET 涓庡懆鏈燂紙鎴栨渶灏忓埌杈鹃棿闅旓級涔嬫瘮锛岃〃绀烘墽琛岃浠诲姟鎵€闇€鐨?CPU 鏃堕棿姣斾緥銆?
 濡傛灉鎬诲埄鐢ㄧ巼 U=sum(WCET_i/P_i) 澶т簬 M锛堝叾涓?M 绛変簬 CPU 鏁伴噺锛夛紝閭ｄ箞璋冨害鍣ㄥ皢鏃犳硶閬靛畧鎵€鏈夋埅姝㈡湡銆傝娉ㄦ剰锛屾€诲埄鐢ㄧ巼瀹氫箟涓虹郴缁熶腑鎵€鏈夊疄鏃朵换鍔＄殑鍒╃敤鐜?WCET_i/P_i 涔嬪拰銆傚綋鑰冭檻澶氫釜瀹炴椂浠诲姟鏃讹紝绗?i 涓换鍔＄殑鍙傛暟鐢?"_i" 鍚庣紑琛ㄧず銆傛澶栵紝濡傛灉鎬诲埄鐢ㄧ巼澶т簬 M锛岄偅涔堟垜浠氨鏈夎瀹炴椂浠诲姟楗挎闈炲疄鏃朵换鍔＄殑椋庨櫓銆傚鏋滐紝鍙嶄箣锛屾€诲埄鐢ㄧ巼灏忎簬 M锛岄偅涔堥潪瀹炴椂浠诲姟灏嗕笉浼氳楗挎锛岀郴缁熸垨璁歌兘澶熼伒瀹堟墍鏈夋埅姝㈡湡銆備簨瀹炰笂锛屽湪杩欑鎯呭喌涓嬪彲浠ヤ负 tardiness锛堣繜鍒版椂闂达紝瀹氫箟涓?0 涓庝綔涓氱殑瀹屾垚鏃堕棿鍙婂叾缁濆鎴鏈熶箣宸箣闂寸殑鏈€澶у€硷級鎻愪緵涓€涓笂鐣屻€傛洿绮剧‘鍦拌锛屽彲浠ヨ瘉鏄庡湪浣跨敤鍏ㄥ眬 EDF 璋冨害鍣ㄦ椂锛屾瘡涓换鍔＄殑鏈€澶?tardiness 灏忎簬绛変簬

	((M 鈭?1) 路 WCET_max 鈭?WCET_min)/(M 鈭?(M 鈭?2) 路 U_max) + WCET_max

 鍏朵腑 WCET_max = max{WCET_i} 涓烘渶澶?WCET锛學CET_min=min{WCET_i} 涓烘渶灏?WCET锛孶_max = max{WCET_i/P_i} 涓烘渶澶у埄鐢ㄧ巼[^12^]銆?
### 3.2 鍗曞鐞嗗櫒绯荤粺鐨勫彲璋冨害鎬у垎鏋?

 濡傛灉 M=1锛堝崟澶勭悊鍣ㄧ郴缁燂級锛屾垨鑰呭湪閲囩敤鍒嗗尯璋冨害锛堟瘡涓疄鏃朵换鍔¤闈欐€佸湴鍒嗛厤鍒板敮涓€涓€涓?CPU锛夌殑鎯呭喌涓嬶紝鍙互褰㈠紡鍖栧湴妫€鏌ユ槸鍚︽墍鏈夋埅姝㈡湡閮借閬靛畧銆傚鏋滃鎵€鏈変换鍔￠兘鏈?D_i = P_i锛岄偅涔堝綋涓斾粎褰撹繍琛屼簬璇?CPU 涓婄殑浠诲姟鎬诲埄鐢ㄧ巼灏忎簬绛変簬 1 鏃讹紝EDF 鎵嶈兘閬靛畧杩愯浜庤 CPU 涓婃墍鏈変换鍔＄殑鍏ㄩ儴鎴鏈熴€傚鏋滄煇浜涗换鍔＄殑 D_i != P_i锛屽垯鍙互灏嗕换鍔＄殑瀵嗗害瀹氫箟涓?WCET_i/min{D_i,P_i}锛涘綋杩愯浜庤 CPU 涓婄殑浠诲姟瀵嗗害涔嬪拰灏忎簬绛変簬 1 鏃讹紝EDF 鑳藉閬靛畧杩愯浜庤 CPU 涓婃墍鏈変换鍔＄殑鍏ㄩ儴鎴鏈燂細

	sum(WCET_i / min{D_i, P_i}) <= 1

 闇€瑕佹敞鎰忕殑鏄紝杩欎竴鏉′欢鍙槸鍏呭垎鐨勶紝鑰岄潪蹇呰鐨勶細瀛樺湪涓€浜涗换鍔￠泦鏄彲璋冨害鐨勶紝鍗翠笉婊¤冻璇ユ潯浠躲€備緥濡傦紝鑰冭檻浠诲姟闆?{Task_1,Task_2}锛屽叾涓?Task_1=(50ms,50ms,100ms)锛孴ask_2=(10ms,100ms,100ms)銆傛樉鐒?EDF 鑳藉鍦ㄤ笉閿欏け浠讳綍鎴鏈熺殑鎯呭喌涓嬭皟搴﹁繖涓や釜浠诲姟锛圱ask_1 涓€鏃﹂噴鏀惧嵆琚皟搴︼紝骞跺垰濂藉湪鎴鏈熷墠瀹屾垚锛汿ask_2 鍦?Task_1 涔嬪悗绔嬪嵆琚皟搴︼紝鍥犳鍏跺搷搴旀椂闂翠笉浼氬ぇ浜?50ms + 10ms = 60ms锛夛紝鍗充娇

	50 / min{50,100} + 10 / min{100, 100} = 50 / 50 + 10 / 100 = 1.1

 褰撶劧锛屼篃鍙互妫€楠?D_i != P_i 浠诲姟鐨勭簿纭彲璋冨害鎬э紙鍗冲悓鏃舵弧瓒冲厖鍒嗕笖蹇呰鐨勬潯浠讹級锛屼絾杩欐棤娉曢€氳繃鎶婃€诲埄鐢ㄧ巼鎴栧瘑搴︿笌鏌愪釜甯告暟姣旇緝鏉ュ畬鎴愩€傚彇鑰屼唬涔嬶紝鍙互浣跨敤鎵€璋撶殑 "澶勭悊鍣ㄩ渶姹?锛坧rocessor demand锛夋柟娉曪細璁＄畻鍦ㄦ椂闂撮暱搴︿负 t 鐨勫尯闂村唴锛屾墍鏈変换鍔′负閬靛畧鍏跺叏閮ㄦ埅姝㈡湡鎵€闇€鐨勬€?CPU 鏃堕棿 h(t)锛屽苟灏嗚鏃堕棿涓庡尯闂撮暱搴?t 杩涜姣旇緝銆傚鏋滃鎵€鏈夊彲鑳界殑 t 鍊奸兘鏈?h(t) 灏忎簬 t锛堝嵆鍦ㄩ暱搴︿负 t 鐨勬椂闂村尯闂村唴浠诲姟鎵€闇€鐨勬椂闂村皬浜庡尯闂撮暱搴︼級锛岄偅涔?EDF 鑳藉璋冨害杩欎簺浠诲姟骞堕伒瀹堝叾鍏ㄩ儴鎴鏈熴€傜敱浜庡鎵€鏈夊彲鑳界殑 t 鍊兼墽琛屾妫€鏌ユ槸涓嶅彲鑳界殑锛屾枃鐚甗4,5,6]宸茶瘉鏄庡彧闇€瀵?0 鍒版渶澶у€?L 涔嬮棿鐨?t 鍊兼墽琛屾祴璇曞嵆鍙€傛墍寮曠敤鐨勮鏂囧寘鍚簡鍏ㄩ儴鏁板缁嗚妭锛屽苟瑙ｉ噴浜嗗浣曡绠?h(t) 鍜?L銆傛棤璁哄浣曪紝杩欑被鍒嗘瀽杩囦簬澶嶆潅涓旇€楁椂锛屾棤娉曞湪绾挎墽琛屻€傚洜姝わ紝濡傜 4 鑺傛墍杩帮紝Linux 浣跨敤涓€涓熀浜庝换鍔″埄鐢ㄧ巼鐨勫噯鍏ユ祴璇曘€?
### 3.3 澶氬鐞嗗櫒绯荤粺鐨勫彲璋冨害鎬у垎鏋?

 鍦ㄩ噰鐢ㄥ叏灞€ EDF 璋冨害锛堥潪鍒嗗尯绯荤粺锛夌殑澶氬鐞嗗櫒绯荤粺涓婏紝鍙皟搴︽€х殑鍏呭垎鎬ф祴璇曚笉鑳藉熀浜庡埄鐢ㄧ巼鎴栧瘑搴︼細鍙互璇佹槑锛屽嵆渚?D_i = P_i锛屽埄鐢ㄧ巼鐣ュぇ浜?1 鐨勪换鍔￠泦涔熸湁鍙兘閿欏け鎴鏈燂紝鑰屼笌 CPU 鏁伴噺鏃犲叧銆?
 鑰冭檻涓€涓寘鍚?M+1 涓换鍔＄殑闆嗗悎 {Task_1,...Task_{M+1}}锛岃繍琛屽湪鍏锋湁 M 涓?CPU 鐨勭郴缁熶笂銆傚叾涓涓€涓换鍔?Task_1=(P,P,P) 鐨勫懆鏈熴€佺浉瀵规埅姝㈡湡鍜?WCET 閮界瓑浜?P銆傚叾浣?M 涓换鍔?Task_i=(e,P-1,P-1) 鍏锋湁浠绘剰灏忕殑鏈€鍧忔儏鍐垫墽琛屾椂闂达紙姝ゅ璁颁负 "e"锛変互鍙婃瘮绗竴涓换鍔℃洿灏忕殑鍛ㄦ湡銆傚洜姝わ紝濡傛灉鎵€鏈変换鍔￠兘鍦ㄥ悓涓€鏃跺埢 t 琚縺娲伙紝鍏ㄥ眬 EDF 浼氬厛璋冨害杩?M 涓换鍔★紙鍥犱负瀹冧滑鐨勭粷瀵规埅姝㈡湡绛変簬 t + P - 1锛屾瘮 Task_1 鐨勭粷瀵规埅姝㈡湡 t + P 鏇村皬锛夈€傜粨鏋滐紝Task_1 鍙兘鍦ㄦ椂鍒?t + e 琚皟搴︼紝骞跺皢鍦ㄦ椂鍒?t + e + P 瀹屾垚锛屽嵆鍦ㄥ叾缁濆鎴鏈熶箣鍚庛€傝浠诲姟闆嗙殑鎬诲埄鐢ㄧ巼涓?U = M 路 e / (P - 1) + P / P = M 路 e / (P - 1) + 1锛屽綋 e 鍙栧緢灏忕殑鍊兼椂锛岃鍊煎彲浠ラ潪甯告帴杩?1銆傝繖琚О涓?"Dhall 鏁堝簲"[^7^]锛圖hall's effect锛夈€傛敞锛欴hall 鍘熷璁烘枃涓殑渚嬪瓙鍦ㄦ琚暐寰畝鍖栵紙渚嬪锛孌hall 鏇存纭湴璁＄畻浜?lim_{e->0}U锛夈€?
 瀹炴椂鏂囩尞[8,9]涓凡鍙戝睍浜嗘洿澶嶆潅鐨勫叏灞€ EDF 鍙皟搴︽€ф祴璇曪紝浣嗗畠浠悓鏍蜂笉鏄熀浜庢€诲埄鐢ㄧ巼锛堟垨瀵嗗害锛変笌鍥哄畾甯告暟鐨勭畝鍗曟瘮杈冦€傚鏋滄墍鏈変换鍔￠兘鏈?D_i = P_i锛屽垯涓€涓厖鍒嗙殑鍙皟搴︽€ф潯浠跺彲浠ョ畝鍗曞湴琛ㄨ揪涓猴細

	sum(WCET_i / P_i) <= M - (M - 1) 路 U_max

 鍏朵腑 U_max = max{WCET_i / P_i}[^10^]銆傛敞鎰忓綋 U_max = 1 鏃讹紝M - (M - 1) 路 U_max 鍙樹负 M - M + 1 = 1锛岃繖涓€鍙皟搴︽€ф潯浠舵伆濂藉嵃璇佷簡 Dhall 鏁堝簲銆傚叧浜庡澶勭悊鍣ㄥ疄鏃惰皟搴﹀彲璋冨害鎬ф祴璇曠殑鏇村畬鏁存枃鐚患杩板彲鍙傝 [^11^]銆?
 濡備笂鎵€杩帮紝寮哄埗鎬诲埄鐢ㄧ巼灏忎簬 M 骞朵笉鑳戒繚璇佸叏灞€ EDF 璋冨害浠诲姟鑰屼笉閿欏け浠讳綍鎴鏈燂紙鎹㈣█涔嬶紝鍏ㄥ眬 EDF 骞堕潪鏈€浼樿皟搴︾畻娉曪級銆傜劧鑰岋紝鎬诲埄鐢ㄧ巼灏忎簬 M 瓒充互淇濊瘉闈炲疄鏃朵换鍔′笉浼氳楗挎锛屼笖瀹炴椂浠诲姟鐨?tardiness 鍏锋湁涓婄晫[^12^]锛堝鍓嶆墍杩帮級銆傚悇绉嶈鏂嘯13,14]涓凡鎻愬嚭浜嗗疄鏃朵换鍔℃渶澶?tardiness 鐨勪笉鍚屼笂鐣岋紝浣嗗 SCHED_DEADLINE 鑰岃█閲嶈鐨勭悊璁虹粨璁烘槸锛氬鏋滄€诲埄鐢ㄧ巼灏忎簬绛変簬 M锛岄偅涔堜换鍔＄殑鍝嶅簲鏃堕棿灏辨槸鏈夌晫鐨勩€?
### 3.4 涓?SCHED_DEADLINE 鍙傛暟鐨勫叧绯?

 鏈€鍚庯紝鐞嗚В绗?2 鑺傛弿杩扮殑 SCHED_DEADLINE 璋冨害鍙傛暟锛坮untime銆乨eadline 鍜?period锛変笌鏈妭鎻忚堪鐨勫疄鏃朵换鍔″弬鏁帮紙WCET銆丏銆丳锛変箣闂寸殑鍏崇郴闈炲父閲嶈銆傝娉ㄦ剰锛屼换鍔＄殑鏃堕棿绾︽潫鐢变笂闈㈡弿杩扮殑缁濆鎴鏈?d_j = r_j + D 琛ㄧず锛岃€?SCHED_DEADLINE 鏄緷鎹皟搴︽埅姝㈡湡瀵逛换鍔¤繘琛岃皟搴︾殑锛堝弬瑙佺 2 鑺傦級銆傚鏋滀娇鐢ㄥ噯鍏ユ祴璇曟潵淇濊瘉璋冨害鎴鏈熻閬靛畧锛岄偅涔?SCHED_DEADLINE 灏卞彲浠ョ敤鏉ヨ皟搴﹀疄鏃朵换鍔★紝骞朵繚璇佷竴涓换鍔＄殑鎵€鏈変綔涓氭埅姝㈡湡閮借閬靛畧銆備负姝わ紝蹇呴』鎸夊涓嬫柟寮忚缃换鍔★細

  - runtime >= WCET
  - deadline = D
  - period <= P

 鎹㈣█涔嬶紙IOW锛夛紝濡傛灉 runtime >= WCET 涓?period <= P锛岄偅涔堣皟搴︽埅姝㈡湡涓庣粷瀵规埅姝㈡湡锛坉_j锛夐噸鍚堬紝鍥犳鎭板綋鐨勫噯鍏ユ帶鍒跺彲浠ヤ繚璇侀伒瀹堣浠诲姟鍚勪綔涓氱殑缁濆鎴鏈燂紙杩欒绉颁负 "纭彲璋冨害鎬у睘鎬?锛宧ard schedulability property锛屾槸 [^2^] 涓紩鐞?1 鐨勬墿灞曪級銆傝娉ㄦ剰锛屽鏋?runtime > deadline锛屽噯鍏ユ帶鍒朵竴瀹氫細鎷掔粷璇ヤ换鍔★紝鍥犱负鍏舵椂闂寸害鏉熸棤娉曡閬靛畧銆?

 鍙傝€冩枃鐚細

  1 - C. L. Liu and J. W. Layland. Scheduling algorithms for multiprogram-
      ming in a hard-real-time environment. Journal of the Association for
      Computing Machinery, 20(1), 1973.
  2 - L. Abeni , G. Buttazzo. Integrating Multimedia Applications in Hard
      Real-Time Systems. Proceedings of the 19th IEEE Real-time Systems
      Symposium, 1998. http://retis.sssup.it/~giorgio/paps/1998/rtss98-cbs.pdf
  3 - L. Abeni. Server Mechanisms for Multimedia Applications. ReTiS Lab
      Technical Report. http://disi.unitn.it/~abeni/tr-98-01.pdf
  4 - J. Y. Leung and M.L. Merril. A Note on Preemptive Scheduling of
      Periodic, Real-Time Tasks. Information Processing Letters, vol. 11,
      no. 3, pp. 115-118, 1980.
  5 - S. K. Baruah, A. K. Mok and L. E. Rosier. Preemptively Scheduling
      Hard-Real-Time Sporadic Tasks on One Processor. Proceedings of the
      11th IEEE Real-time Systems Symposium, 1990.
  6 - S. K. Baruah, L. E. Rosier and R. R. Howell. Algorithms and Complexity
      Concerning the Preemptive Scheduling of Periodic Real-Time tasks on
      One Processor. Real-Time Systems Journal, vol. 4, no. 2, pp 301-324,
      1990.
  7 - S. J. Dhall and C. L. Liu. On a real-time scheduling problem. Operations
      research, vol. 26, no. 1, pp 127-140, 1978.
  8 - T. Baker. Multiprocessor EDF and Deadline Monotonic Schedulability
      Analysis. Proceedings of the 24th IEEE Real-Time Systems Symposium, 2003.
  9 - T. Baker. An Analysis of EDF Schedulability on a Multiprocessor.
      IEEE Transactions on Parallel and Distributed Systems, vol. 16, no. 8,
      pp 760-768, 2005.
  10 - J. Goossens, S. Funk and S. Baruah, Priority-Driven Scheduling of
       Periodic Task Systems on Multiprocessors. Real-Time Systems Journal,
       vol. 25, no. 2鈥?, pp. 187鈥?05, 2003.
  11 - R. Davis and A. Burns. A Survey of Hard Real-Time Scheduling for
       Multiprocessor Systems. ACM Computing Surveys, vol. 43, no. 4, 2011.
       http://www-users.cs.york.ac.uk/~robdavis/papers/MPSurveyv5.0.pdf
  12 - U. C. Devi and J. H. Anderson. Tardiness Bounds under Global EDF
       Scheduling on a Multiprocessor. Real-Time Systems Journal, vol. 32,
       no. 2, pp 133-189, 2008.
  13 - P. Valente and G. Lipari. An Upper Bound to the Lateness of Soft
       Real-Time Tasks Scheduled by EDF on Multiprocessors. Proceedings of
       the 26th IEEE Real-Time Systems Symposium, 2005.
  14 - J. Erickson, U. Devi and S. Baruah. Improved tardiness bounds for
       Global EDF. Proceedings of the 22nd Euromicro Conference on
       Real-Time Systems, 2010.
  15 - G. Lipari, S. Baruah, Greedy reclamation of unused bandwidth in
       constant-bandwidth servers, 12th IEEE Euromicro Conference on Real-Time
       Systems, 2000.
  16 - L. Abeni, J. Lelli, C. Scordino, L. Palopoli, Greedy CPU reclaiming for
       SCHED DEADLINE. In Proceedings of the Real-Time Linux Workshop (RTLWS),
       Dusseldorf, Germany, 2014.
  17 - L. Abeni, G. Lipari, A. Parri, Y. Sun, Multicore CPU reclaiming: parallel
       or sequential?. In Proceedings of the 31st Annual ACM Symposium on Applied
       Computing, 2016.
  18 - J. Lelli, C. Scordino, L. Abeni, D. Faggioli, Deadline scheduling in the
       Linux kernel, Software: Practice and Experience, 46(6): 821-839, June
       2016.
  19 - C. Scordino, L. Abeni, J. Lelli, Energy-Aware Real-Time Scheduling in
       the Linux Kernel, 33rd ACM/SIGAPP Symposium On Applied Computing (SAC
       2018), Pau, France, April 2018.


## 4. 甯﹀绠＄悊


 濡傚墠鎵€杩帮紝涓轰簡浣?-deadline 璋冨害鏈夋晥涓旀湁鐢紙鍗宠兘澶熷湪 "deadline" 鍐呮彁渚?"runtime" 鏃堕棿鍗曚綅锛夛紝蹇呴』鏈変竴浜涙柟娉曟潵鎺у埗灏嗗彲鐢?CPU 鏃堕棿浠介鍒嗛厤缁欏悇涓换鍔＄殑鏂瑰紡銆傝繖閫氬父琚О涓?"鍑嗗叆鎺у埗"锛坅dmission control锛夛紱濡傛灉涓嶆墽琛屽畠锛屽氨鏃犳硶瀵?-deadline 浠诲姟鐨勫疄闄呰皟搴︽彁渚涗换浣曚繚璇併€?
 濡傜 3 鑺傚凡缁忚鏄庯紝姝ｇ‘璋冨害涓€缁勫疄鏃朵换鍔℃墍闇€閬靛畧鐨勪竴涓繀瑕佹潯浠舵槸鎬诲埄鐢ㄧ巼灏忎簬 M銆傚浜?-deadline 浠诲姟鑰岃█锛岃繖瑕佹眰鎵€鏈変换鍔＄殑 runtime 涓?period 涔嬫瘮鐨勫拰灏忎簬 M銆傛敞鎰忥紝runtime/period 涔嬫瘮绛変环浜?"浼犵粺" 瀹炴椂浠诲姟鐨勫埄鐢ㄧ巼锛屼篃甯歌绉颁负 "甯﹀"锛坆andwidth锛夈€傜敤浜庢帶鍒跺彲鍒嗛厤缁?-deadline 浠诲姟鐨?CPU 甯﹀鐨勬帴鍙ｏ紝涓庡凡鐢ㄤ簬 -rt 浠诲姟鐨勫疄鏃剁粍璋冨害锛堝嵆 RT-throttling锛屽弬瑙?Documentation/scheduler/sched-rt-group.rst锛夌殑鎺ュ彛绫讳技锛屽苟鍩轰簬浣嶄簬 procfs 涓€佸彲璇诲彲鍐欑殑鎺у埗鏂囦欢锛堢敤浜庣郴缁熺骇璁剧疆锛夈€傝娉ㄦ剰锛岄拡瀵?-deadline 浠诲姟鐨勬瘡涓粍锛坧er-group锛夎缃紙閫氳繃 cgroupfs 鎺у埗锛夌洰鍓嶅皻鏈畾涔夛紝鍥犱负杩橀渶瑕佹洿澶氳璁烘潵纭畾鎴戜滑鎯冲湪浠诲姟缁勫眰闈㈠浣曠鐞?SCHED_DEADLINE 甯﹀銆?
 鎴鏈熷甫瀹界鐞嗕笌 RT-throttling 鐨勪竴涓富瑕佸尯鍒湪浜庯細-deadline 浠诲姟鑷韩鎷ユ湁甯﹀锛堣€?-rt 浠诲姟娌℃湁锛侊級锛屽洜姝ゆ垜浠棤闇€鏇撮珮灞傜殑闄愭祦鏈哄埗鏉ュ己鍒跺疄鏂芥湡鏈涚殑甯﹀銆傛崲瑷€涔嬶紝杩欐剰鍛崇潃鎺ュ彛鍙傛暟浠呭湪鍑嗗叆鎺у埗鏃讹紙鍗崇敤鎴疯皟鐢?sched_setattr() 鏃讹級浣跨敤銆傞殢鍚庤皟搴︿細渚濇嵁浠诲姟鐨勫疄闄呭弬鏁版墽琛岋紝浠庤€屼互绗﹀悎鍏剁矑搴﹂渶姹傜殑鏂瑰紡灏?CPU 甯﹀鍒嗛厤缁?SCHED_DEADLINE 浠诲姟銆傚洜姝わ紝鍒╃敤杩欎竴绠€鍗曟帴鍙ｏ紝鎴戜滑鍙互瀵?-deadline 浠诲姟鐨勬€诲埄鐢ㄧ巼璁剧疆涓婇檺锛堝嵆 \Sum (runtime_i / period_i) < global_dl_utilization_cap锛夈€?
### 4.1 绯荤粺绾ц缃?

 绯荤粺绾ц缃湪 /proc 铏氭嫙鏂囦欢绯荤粺涓嬮厤缃€?
 鐩墠 -rt 鐨勬棆閽紙knobs锛夎鐢ㄤ簬 -deadline 鐨勫噯鍏ユ帶鍒讹紱鍦ㄥ惎鐢?CONFIG_RT_GROUP_SCHED 鏃讹紝-deadline 鐨勮繍琛屾椂闂磋鍏ワ紙鏍癸級-rt 杩愯鏃堕棿銆傚湪涓嶅惎鐢?CONFIG_RT_GROUP_SCHED 鏃讹紝璇ユ棆閽粎鐢ㄤ簬 -dl 鐨勫噯鍏ユ帶鍒躲€傛垜浠剰璇嗗埌杩欏苟闈炲畬鍏ㄧ悊鎯筹紱涓嶈繃锛屾殏鏃舵嫢鏈変竴涓緝灏忕殑鎺ュ彛銆佷笖渚夸簬鏃ュ悗淇敼锛屾槸鏇村ソ鐨勯€夋嫨銆傜悊鎯崇殑鎯呭喌锛堣绗?5 鑺傦級鏄粠涓€涓?-deadline 鏈嶅姟鍣ㄨ繍琛?-rt 浠诲姟锛涘湪杩欑鎯呭喌涓嬶紝-rt 甯﹀灏辨槸 dl_bw 鐨勭洿鎺ュ瓙闆嗐€?
 杩欐剰鍛崇潃锛屽浜庝竴涓寘鍚?M 涓?CPU 鐨?root_domain锛屽彧瑕佸叾甯﹀涔嬪拰淇濇寔鍦ㄤ互涓嬪€间箣涓嬶紝灏卞彲浠ュ垱寤?-deadline 浠诲姟锛?
   M * (sched_rt_runtime_us / sched_rt_period_us)

 涔熷彲浠ョ鐢ㄨ繖涓€甯﹀绠＄悊閫昏緫锛屼粠鑰屽彲浠ヤ换鎰忓湴瓒呴璁㈤槄绯荤粺銆傝繖鏄€氳繃鍚?/proc/sys/kernel/sched_rt_runtime_us 鍐欏叆 -1 鏉ュ疄鐜扮殑銆?

### 4.2 浠诲姟鎺ュ彛


 鎸囧畾涓€涓懆鏈?闆舵槦浠诲姟锛堝湪姣忔瀹炰緥涓墽琛岀粰瀹氱殑杩愯鏃堕棿锛屽苟鏍规嵁鑷韩鏃跺簭绾︽潫鐨勭揣杩€ц繘琛岃皟搴︼級锛岄€氬父闇€瑕佷竴绉嶆柟寮忔潵澹版槑锛?
  - 锛堟渶澶?鍏稿瀷锛夊疄渚嬫墽琛屾椂闂达紝
  - 杩炵画瀹炰緥涔嬮棿鐨勬渶灏忛棿闅旓紝
  - 姣忎釜瀹炰緥蹇呴』瀹屾垚鐨勬椂闂寸害鏉熴€?
 鍥犳锛?
  - 鎻愪緵浜嗕竴涓柊鐨?struct sched_attr锛屽寘鍚叏閮ㄥ繀瑕佸瓧娈碉紱
  - 瀹炵幇浜嗘搷浣滃畠鐨勬柊鐨勮皟搴︾浉鍏崇郴缁熻皟鐢紝鍗?sched_setattr() 鍜?sched_getattr()銆?
 SCHED_DEADLINE 浠诲姟鐨勫墿浣欒繍琛屾椂闂村拰缁濆鎴鏈熷彲浠ラ€氳繃 sched_getattr() 绯荤粺璋冪敤璇诲彇锛屽彧闇€灏嗚绯荤粺璋冪敤鐨勬渶鍚庝竴涓弬鏁?flags 璁句负 SCHED_GETATTR_FLAG_DL_DYNAMIC=1銆傝繖浼氭洿鏂板墿浣欒繍琛屾椂闂达紝灏嗙粷瀵规埅姝㈡湡杞崲涓?CLOCK_MONOTONIC 鍙傝€冪郴锛岀劧鍚庡皢杩欎簺鍙傛暟杩斿洖缁欑敤鎴风┖闂淬€傜粷瀵规埅姝㈡湡浠ヨ嚜 CLOCK_MONOTONIC 鏃堕棿鍙傝€冪郴锛堝惎鍔ㄦ椂鍒伙級浠ユ潵鐨勭撼绉掓暟褰㈠紡杩斿洖锛屼綔涓?sched_attr 鐨?sched_deadline 瀛楁涓殑涓€涓?u64锛屽叾鍙〃绀鸿嚜鍚姩浠ユ潵杩?585 骞达紙鑰屼互 flags=0 璋冪敤 sched_getattr() 鍒欒繑鍥為潤鎬佸弬鏁帮級銆?
 鍑轰簬璋冭瘯鐩殑锛岃繖浜涘弬鏁颁篃鍙互閫氳繃 /proc/<pid>/sched 鑾峰彇锛堟潯鐩?dl.runtime 鍜?dl.deadline锛屼袱鑰呭崟浣嶅潎涓?ns锛夛紝浣嗘槸锛氳繖绉嶆柟寮忔晥鐜囨瀬浣庯紱杩斿洖鐨勫墿浣欒繍琛屾椂闂翠笉鍍?sched_getattr() 閭ｆ牱琚洿鏂帮紱鎴鏈熸槸浠ュ唴鏍?rq_clock 鏃堕棿鍙傝€冪郴鎻愪緵鐨勶紝鏃犳硶鐩存帴浠庣敤鎴风┖闂翠娇鐢ㄣ€?

### 4.3 榛樿琛屼负


 SCHED_DEADLINE 甯﹀鐨勯粯璁ゅ€煎皢 rt_runtime 璁句负 950000銆傜敱浜?rt_period 绛変簬 1000000锛岄粯璁ゆ儏鍐典笅杩欐剰鍛崇潃瀵逛簬姣忎釜 root_domain锛?deadline 浠诲姟鏈€澶氬彲浣跨敤 95% 涔樹互缁勬垚璇?root_domain 鐨?CPU 鏁伴噺銆傝繖鎰忓懗鐫€闈?-deadline 浠诲姟灏嗚嚦灏戣幏寰?5% 鐨?CPU 鏃堕棿锛屽苟涓?-deadline 浠诲姟灏嗕互淇濊瘉鐨勬渶鍧忔儏鍐靛欢杩熻幏寰楀叾杩愯鏃堕棿锛堢浉瀵逛簬 "deadline" 鍙傛暟锛夈€傚鏋?"deadline" = "period"锛屽苟涓斾娇鐢?cpuset 鏈哄埗鏉ュ疄鐜板垎鍖鸿皟搴︼紙鍙傝绗?5 鑺傦級锛岄偅涔堣繖涓€绠€鍗曠殑甯﹀绠＄悊璁剧疆灏辫兘澶熺‘瀹氭€у湴淇濊瘉 -deadline 浠诲姟鍦ㄤ竴涓懆鏈熷唴鑾峰緱鍏惰繍琛屾椂闂淬€?
 鏈€鍚庤娉ㄦ剰锛屼负浜嗕笉鐮村潖鍑嗗叆鎺у埗锛?deadline 浠诲姟涓嶈兘 fork锛堝垱寤哄瓙杩涚▼锛夈€?

### 4.4 sched_yield() 鐨勮涓?

 褰撲竴涓?SCHED_DEADLINE 浠诲姟璋冪敤 sched_yield() 鏃讹紝瀹冧細鏀惧純鍏跺墿浣欒繍琛屾椂闂村苟琚珛鍗抽檺娴侊紝鐩村埌涓嬩竴涓懆鏈熷叾杩愯鏃堕棿琚ˉ鍏呬负姝紙浼氳缃竴涓壒娈婃爣蹇?dl_yielded锛岀敤浜庢纭鐞嗚皟鐢?sched_yield() 涔嬪悗鐨勯檺娴佷笌杩愯鏃堕棿琛ュ厖锛夈€?
 sched_yield() 鐨勮繖涓€琛屼负浣垮緱浠诲姟鑳藉湪涓嬩竴涓懆鏈熷紑濮嬫椂鎭板ソ琚敜閱掋€傛澶栵紝杩欏湪鏈潵涓庡甫瀹藉洖鏀舵満鍒剁粨鍚堟椂鍙兘鏈夌敤锛屽眾鏃?sched_yield() 浼氫娇鍓╀綑鐨勮繍琛屾椂闂村彲渚涘叾浠?SCHED_DEADLINE 浠诲姟鍥炴敹銆?

## 5. 浠诲姟鐨?CPU 浜插拰鎬?

 鎴鏈熶换鍔＄殑 CPU 浜插拰鎬ф帺鐮佷笉鑳藉皬浜庡叾鍒涘缓鎵€鍦ㄧ殑 root domain銆傚洜姝わ紝浣跨敤 `sched_setaffinity(2)` 涓嶄細鐢熸晥銆傜浉鍙嶏紝鎴鏈熶换鍔″簲褰撳垱寤哄湪涓€涓彈闄愮殑 root domain 涓€傝繖鍙互閫氳繃浣跨敤 cgroup v1锛堝凡寮冪敤锛夋垨 cgroup v2 鐨?cpuset 鎺у埗鍣ㄦ潵瀹炵幇銆傛洿澶氫俊鎭鍙傝 Documentation/admin-guide/cgroup-v1/cpusets.rst <cpusets> 鍜?Documentation/admin-guide/cgroup-v2.rst <cgroup-v2>銆?
### 5.1 浣跨敤 cgroup v1 cpuset 鎺у埗鍣?

```

   mkdir /dev/cpuset
   mount -t cgroup -o cpuset cpuset /dev/cpuset
   cd /dev/cpuset
   mkdir cpu0
   echo 0 > cpu0/cpuset.cpus
   echo 0 > cpu0/cpuset.mems
   echo 1 > cpuset.cpu_exclusive
   echo 0 > cpuset.sched_load_balance
   echo 1 > cpu0/cpuset.cpu_exclusive
   echo 1 > cpu0/cpuset.mem_exclusive
   echo $$ > cpu0/tasks
   chrt --sched-runtime 100000 --sched-period 200000 --deadline 0 yes > /dev/null

```

### 5.2 浣跨敤 cgroup v2 cpuset 鎺у埗鍣?

 鍋囧畾 cgroup v2 鏍规寕杞藉湪 `/sys/fs/cgroup`锛屼笅闈㈡槸涓€涓?
```

   cd /sys/fs/cgroup
   echo '+cpuset' > cgroup.subtree_control
   mkdir deadline_group
   echo 0 > deadline_group/cpuset.cpus
   echo 'root' > deadline_group/cpuset.cpus.partition
   echo $$ > deadline_group/cgroup.procs
   chrt --sched-runtime 100000 --sched-period 200000 --deadline 0 yes > /dev/null

```

## 6. 鏈潵璁″垝


 灏氱己锛?
  - 浠ョ紪绋嬫柟寮忚幏鍙栧綋鍓嶈繍琛屾椂闂村拰缁濆鎴鏈熺殑鏂规硶锛?  - 瀵规埅姝㈡湡缁ф壙锛坉eadline inheritance锛夌殑鏀硅繘锛岀壒鍒槸鍏充簬鍦ㄩ潪浜や簰浠诲姟涔嬮棿淇濇寔甯﹀闅旂鐨勫彲鑳芥€с€傝繖姝ｄ粠鐞嗚鍜屽疄璺典袱涓搴﹁繘琛岀爺绌讹紝甯屾湜鎴戜滑寰堝揩鑳藉浜у嚭涓€浜涙紨绀烘€т唬鐮侊紱
  - 鍩轰簬 (c)group 鐨勫甫瀹界鐞嗭紝鐢氳嚦璋冨害锛?  - 閽堝闈?root 鐢ㄦ埛鐨勮闂帶鍒讹紙浠ュ強鐩稿叧瀹夊叏闂锛夛紝杩欐槸鍏佽闈炵壒鏉冪敤鎴蜂娇鐢ㄨ繖浜涙満鍒剁殑鏈€浣虫柟寮忥紝浠ュ強濡備綍闃叉闈?root 鐢ㄦ埛 "娆洪獥" 绯荤粺锛?
 濡傚墠鎵€杩帮紝鎴戜滑涔熻鍒掑皢杩欓」宸ヤ綔涓?EDF 闄愭祦琛ヤ竵 [https://lore.kernel.org/r/cover.1266931410.git.fabio@helm.retis] 鍚堝苟锛屼絾鍚堝苟浠嶅浜庡垵姝ラ樁娈碉紝鎴戜滑闈炲父甯屾湜鑾峰緱鍙嶉锛屼互甯姪鎴戜滑鍐冲畾鍏跺彂灞曟柟鍚戙€?
## 闄勫綍 A. 娴嬭瘯濂椾欢


 SCHED_DEADLINE 绛栫暐鍙互浣跨敤涓や釜搴旂敤绋嬪簭杞绘澗娴嬭瘯锛屽畠浠槸鏇村ぇ鐨?Linux 璋冨害鍣ㄩ獙璇佸浠剁殑涓€閮ㄥ垎銆傝濂椾欢浠?GitHub 浠撳簱褰㈠紡鎻愪緵锛歨ttps://github.com/scheduler-tools銆?
 绗竴涓祴璇曞簲鐢ㄧ▼搴忓悕涓?rt-app锛屽彲鐢ㄤ簬浠ョ壒瀹氬弬鏁板惎鍔ㄥ涓嚎绋嬨€俽t-app 鏀寔 SCHED_{OTHER,FIFO,RR,DEADLINE} 璋冨害绛栫暐鍙婂叾鐩稿叧鍙傛暟锛堜緥濡?niceness銆乸riority銆乺untime/deadline/period锛夈€俽t-app 鏄竴涓湁浠峰€肩殑宸ュ叿锛屽洜涓哄畠鍙敤浜庡悎鎴愬湴閲嶅缓鏌愪簺宸ヤ綔璐熻浇锛堟垨璁歌兘妯℃嫙鐪熷疄鐢ㄤ緥锛夛紝骞惰瘎浼拌皟搴﹀櫒鍦ㄦ绫昏礋杞戒笅鐨勮涓恒€傝繖鏍凤紝缁撴灉寰堝鏄撳鐜般€俽t-app 鍙湪浠ヤ笅鍦板潃鑾峰彇锛歨ttps://github.com/scheduler-tools/rt-app銆?
 rt-app 涓嶆帴鍙楀懡浠よ鍙傛暟锛岃€屾槸浠庝竴涓?JSON 閰嶇疆鏂囦欢涓鍙栭厤缃€備笅闈㈡槸涓€涓?`config.json` 绀轰緥锛?
 .. code-block:: json

  {
    "tasks": {
      "dl_task": {
        "policy": "SCHED_DEADLINE",
        "priority": 0,
        "dl-runtime": 10000,
        "dl-period": 100000,
        "dl-deadline": 100000
      },
      "fifo_task": {
        "policy": "SCHED_FIFO",
        "priority": 10,
        "runtime": 20000,
        "sleep": 130000
      }
    },
    "global": {
      "duration": 5
    }
  }

 杩愯 `rt-app config.json` 鏃讹紝瀹冧細鍒涘缓 2 涓嚎绋嬨€傜涓€涓敱 SCHED_DEADLINE 璋冨害锛屾瘡 100ms 鎵ц 10ms銆傜浜屼釜浠?SCHED_FIFO 浼樺厛绾?10 璋冨害锛屾瘡 150ms 鎵ц 20ms銆傛祴璇曟€诲叡杩愯 5 绉掋€?
 鏈夊叧 JSON 妯″紡鍙婃洿澶氱ず渚嬶紝璇峰弬闃?rt-app 鏂囨。銆?
 绗簩涓祴璇曞簲鐢ㄧ▼搴忎娇鐢?chrt 瀹炵幇锛屽畠鏀寔 SCHED_DEADLINE銆?
```

  # chrt -d -T 10000000 -D 100000000 0 ./my_cpuhog_app

 With this, my_cpuhog_app is put to run inside a SCHED_DEADLINE reservation
 of 10ms every 100ms (note that parameters are expressed in nanoseconds).
 You can also use chrt to create a reservation for an already running
 application, given that you know its pid::

  # chrt -d -T 10000000 -D 100000000 -p 0 my_app_pid

```

## 闄勫綍 B. 鏈€灏?main()


 涓嬮潰鎴戜滑鎻愪緵涓€涓畝鍗曪紙涓戦檵锛夌殑鑷寘鍚唬鐮佺墖娈碉紝灞曠ず瀹炴椂浠诲姟濡備綍鍒涘缓 SCHED_DEADLINE 棰勭暀锛坮eservation锛夛細

```

   #define _GNU_SOURCE
   #include <unistd.h>
   #include <stdio.h>
   #include <stdlib.h>
   #include <string.h>
   #include <time.h>
   #include <linux/unistd.h>
   #include <linux/kernel.h>
   #include <linux/types.h>
   #include <sys/syscall.h>
   #include <pthread.h>

   #define gettid() syscall(__NR_gettid)

   #define SCHED_DEADLINE	6

   /* XXX use the proper syscall numbers */
   #ifdef __x86_64__
   #define __NR_sched_setattr		314
   #define __NR_sched_getattr		315
   #endif

   #ifdef __i386__
   #define __NR_sched_setattr		351
   #define __NR_sched_getattr		352
   #endif

   #ifdef __arm__
   #define __NR_sched_setattr		380
   #define __NR_sched_getattr		381
   #endif

   static volatile int done;

   struct sched_attr {
	__u32 size;

	__u32 sched_policy;
	__u64 sched_flags;

	/* SCHED_NORMAL, SCHED_BATCH */
	__s32 sched_nice;

	/* SCHED_FIFO, SCHED_RR */
	__u32 sched_priority;

	/* SCHED_DEADLINE (nsec) */
	__u64 sched_runtime;
	__u64 sched_deadline;
	__u64 sched_period;
   };

   int sched_setattr(pid_t pid,
		  const struct sched_attr *attr,
		  unsigned int flags)
   {
	return syscall(__NR_sched_setattr, pid, attr, flags);
   }

   int sched_getattr(pid_t pid,
		  struct sched_attr *attr,
		  unsigned int size,
		  unsigned int flags)
   {
	return syscall(__NR_sched_getattr, pid, attr, size, flags);
   }

   void *run_deadline(void *data)
   {
	struct sched_attr attr;
	int x = 0;
	int ret;
	unsigned int flags = 0;

	printf("deadline thread started [%ld]\n", gettid());

	attr.size = sizeof(attr);
	attr.sched_flags = 0;
	attr.sched_nice = 0;
	attr.sched_priority = 0;

	/* This creates a 10ms/30ms reservation */
	attr.sched_policy = SCHED_DEADLINE;
	attr.sched_runtime = 10 * 1000 * 1000;
	attr.sched_period = attr.sched_deadline = 30 * 1000 * 1000;

	ret = sched_setattr(0, &attr, flags);
	if (ret < 0) {
		done = 0;
		perror("sched_setattr");
		exit(-1);
	}

	while (!done) {
		x++;
	}

	printf("deadline thread dies [%ld]\n", gettid());
	return NULL;
   }

   int main (int argc, char **argv)
   {
	pthread_t thread;

	printf("main thread [%ld]\n", gettid());

	pthread_create(&thread, NULL, run_deadline, NULL);

	sleep(10);

	done = 1;
	pthread_join(thread, NULL);

	printf("main dies [%ld]\n", gettid());
	return 0;
   }

```
