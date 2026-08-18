
## RCU 涓庡彲鍗歌浇妯″潡


[Originally published in LWN Jan. 14, 2007: http://lwn.net/Articles/217484/]

RCU 鏇存柊鑰呮湁鏃朵細浣跨敤 `call_rcu()` 鏉ュ彂璧蜂竴娆″紓姝ョ瓑寰咃紝鐩村埌瀹介檺鏈燂紙grace
period锛夌粨鏉熴€傝鍘熻鎺ュ彈涓€涓寚鍚?RCU 淇濇姢鏁版嵁缁撴瀯鍐呴儴鐨?`rcu_head` 缁撴瀯浣撶殑
鎸囬拡锛屼互鍙婂彟涓€涓寚鍚戞煇涓嚱鏁扮殑鎸囬拡锛岃鍑芥暟鍙兘鍦ㄧ◢鍚庤璋冪敤鏉ラ噴鏀捐缁撴瀯浣撱€?浠?IRQ 涓婁笅鏂囦腑鍒犻櫎閾捐〃涓厓绱?p 鐨勪唬鐮佸彲鑳藉涓嬶細
```
	list_del_rcu(p);
	call_rcu(&p->rcu, p_callback);
```
鐢变簬 `call_rcu()` 浠庝笉闃诲锛屽洜姝よ繖娈典唬鐮佸彲浠ュ畨鍏ㄥ湴鐢ㄤ簬
```
	static void p_callback(struct rcu_head *rp)
	{
		struct pstruct *p = container_of(rp, struct pstruct, rcu);

		kfree(p);
	}
```

### 鍗歌浇浣跨敤 call_rcu() 鐨勬ā鍧?

浣嗗鏋?`p_callback()` 鍑芥暟瀹氫箟鍦ㄤ竴涓彲鍗歌浇鐨勬ā鍧椾腑鍛紵

濡傛灉鎴戜滑鍦ㄦ煇浜?RCU 鍥炶皟浠嶇劧鎸傝捣鏃跺嵏杞借妯″潡锛岄偅涔堢◢鍚庢墽琛岃繖浜涘洖璋冪殑 CPU
鍦ㄨ皟鐢ㄦ椂灏嗛伃閬囦弗閲嶇殑涓嶆剦蹇紝鐩稿叧鎯呭舰鍙弬瑙?http://lwn.net/images/ns/kernel/rcu-drop.jpg 涓殑鐢熷姩鎻忕粯銆?
鎴戜滑鍙互灏濊瘯鍦ㄦā鍧楃殑閫€鍑轰唬鐮佽矾寰勪腑鏀剧疆涓€涓?`synchronize_rcu()`锛屼絾杩欏苟涓?鍏呭垎銆傚敖绠?`synchronize_rcu()` 纭疄浼氱瓑寰呬竴涓闄愭湡缁撴潫锛屼絾瀹冨苟涓嶄細绛夊緟
鍥炶皟瀹屾垚銆?
鏈変汉鍙兘鎯宠繛缁皟鐢ㄥ嚑涓?`synchronize_rcu()`锛屼絾杩欎粛鐒舵棤娉曚繚璇佸彲琛屻€傚鏋滃瓨鍦?闈炲父绻侀噸鐨?RCU 鍥炶皟璐熻浇锛岄偅涔堟煇浜涘洖璋冨彲鑳戒細琚欢鍚庯紝浠ヤ究璁╁叾浠栧鐞嗗緱浠ョ户缁?杩涜銆備粎涓句竴渚嬶細鍦ㄥ疄鏃跺唴鏍镐腑锛屼负浜嗛伩鍏嶈繃搴︾殑璋冨害寤惰繜锛岃繖绉嶅欢鍚庢槸蹇呴渶鐨勩€?
### rcu_barrier()


杩欑鎯呭喌鍙互閫氳繃 `rcu_barrier()` 鍘熻鏉ュ鐞嗐€備笌绛夊緟瀹介檺鏈熺粨鏉熶笉鍚岋紝
`rcu_barrier()` 绛夊緟鎵€鏈夋湭鍐崇殑 RCU 鍥炶皟瀹屾垚銆傝娉ㄦ剰锛宍rcu_barrier()` 骞?**涓?* 闅愬惈 `synchronize_rcu()`锛涚壒鍒湴锛屽鏋滀换浣曞湴鏂归兘娌℃湁鎺掗槦鐨?RCU 鍥炶皟锛?`rcu_barrier()` 鏈夋潈绔嬪嵆杩斿洖锛岃€屾棤闇€绛夊緟浠讳綍浜嬫儏锛屾洿涓嶇敤璇村闄愭湡浜嗐€?
浣跨敤 `rcu_barrier()` 鐨勪吉浠ｇ爜濡備笅锛?```
   1. Prevent any new RCU callbacks from being posted.
   2. Execute rcu_barrier().
   3. Allow the module to be unloaded.
```
閽堝 SRCU 杩樻湁涓€涓?`srcu_barrier()` 鍑芥暟锛屽綋鐒朵綘蹇呴』浣?`srcu_barrier()` 鐨?绫诲瀷涓?`call_srcu()` 鐩稿尮閰嶃€傚鏋滀綘鐨勬ā鍧椾娇鐢ㄤ簡澶氫釜 `srcu_struct` 缁撴瀯浣擄紝
閭ｄ箞鍦ㄥ嵏杞借妯″潡鏃朵篃蹇呴』澶氭璋冪敤 `srcu_barrier()`銆備緥濡傦紝濡傛灉瀹冧娇鐢ㄤ簡
`call_rcu()`銆乣srcu_struct_1` 涓婄殑 `call_srcu()`锛屼互鍙?`srcu_struct_2` 涓婄殑
`call_srcu()`锛岄偅涔堜笅闈㈣繖涓夎浠ｇ爜
```
  1  rcu_barrier();
  2  srcu_barrier(&srcu_struct_1);
  3  srcu_barrier(&srcu_struct_2);
```
濡傛灉寤惰繜鑷冲叧閲嶈锛屽彲浠ヤ娇鐢ㄥ伐浣滈槦鍒楋紙workqueue锛夊苟鍙戝湴杩愯杩欎笁涓嚱鏁般€?
rcutorture 妯″潡鐨勪竴涓彜鑰佺増鏈娇鐢ㄤ簡 `rcu_barrier()`锛?```
  1  static void
  2  rcu_torture_cleanup(void)
  3  {
  4    int i;
  5
  6    fullstop = 1;
  7    if (shuffler_task != NULL) {
  8      VERBOSE_PRINTK_STRING("Stopping rcu_torture_shuffle task");
  9      kthread_stop(shuffler_task);
 10    }
 11    shuffler_task = NULL;
 12
 13    if (writer_task != NULL) {
 14      VERBOSE_PRINTK_STRING("Stopping rcu_torture_writer task");
 15      kthread_stop(writer_task);
 16    }
 17    writer_task = NULL;
 18
 19    if (reader_tasks != NULL) {
 20      for (i = 0; i < nrealreaders; i++) {
 21        if (reader_tasks[i] != NULL) {
 22          VERBOSE_PRINTK_STRING(
 23            "Stopping rcu_torture_reader task");
 24          kthread_stop(reader_tasks[i]);
 25        }
 26        reader_tasks[i] = NULL;
 27      }
 28      kfree(reader_tasks);
 29      reader_tasks = NULL;
 30    }
 31    rcu_torture_current = NULL;
 32
 33    if (fakewriter_tasks != NULL) {
 34      for (i = 0; i < nfakewriters; i++) {
 35        if (fakewriter_tasks[i] != NULL) {
 36          VERBOSE_PRINTK_STRING(
 37            "Stopping rcu_torture_fakewriter task");
 38          kthread_stop(fakewriter_tasks[i]);
 39        }
 40        fakewriter_tasks[i] = NULL;
 41      }
 42      kfree(fakewriter_tasks);
 43      fakewriter_tasks = NULL;
 44    }
 45
 46    if (stats_task != NULL) {
 47      VERBOSE_PRINTK_STRING("Stopping rcu_torture_stats task");
 48      kthread_stop(stats_task);
 49    }
 50    stats_task = NULL;
 51
 52    /* Wait for all RCU callbacks to fire. */
 53    rcu_barrier();
 54
 55    rcu_torture_stats_print(); /* -After- the stats thread is stopped! */
 56
 57    if (cur_ops->cleanup != NULL)
 58      cur_ops->cleanup();
 59    if (atomic_read(&n_rcu_torture_error))
 60      rcu_torture_print_module_parms("End of test: FAILURE");
 61    else
 62      rcu_torture_print_module_parms("End of test: SUCCESS");
 63  }
```
绗?6 琛岃缃竴涓叏灞€鍙橀噺锛岄樆姝换浣?RCU 鍥炶皟鍐嶆鎻愪氦鑷韩銆傚湪澶у鏁版儏鍐典笅杩欏苟闈?蹇呰锛屽洜涓?RCU 鍥炶皟寰堝皯鍖呭惈瀵?`call_rcu()` 鐨勮皟鐢ㄣ€備笉杩囷紝rcutorture 妯″潡鏄?杩欐潯瑙勫垯鐨勪竴涓緥澶栵紝鍥犳闇€瑕佽缃鍏ㄥ眬鍙橀噺銆?
绗?7-50 琛屽仠姝㈡墍鏈変笌 rcutorture 妯″潡鍏宠仈鐨?kernel task銆傚洜姝わ紝涓€鏃︽墽琛屽埌杈?绗?53 琛岋紝灏变笉浼氬啀鏈?rcutorture 鐨?RCU 鍥炶皟琚彁浜ゃ€傜 53 琛岀殑 `rcu_barrier()`
璋冪敤浼氱瓑寰呬换浣曢鍏堝瓨鍦ㄧ殑鍥炶皟瀹屾垚銆?
鐒跺悗绗?55-62 琛屾墦鍗扮姸鎬佸苟杩涜鐗瑰畾浜庢搷浣滅殑娓呯悊锛屼箣鍚庤繑鍥烇紝浠庤€屽厑璁告ā鍧楀嵏杞?鎿嶄綔瀹屾垚銆?
Quick Quiz #1:
	Is there any other situation where rcu_barrier() might
	be required?

Answer to Quick Quiz #1 <answer_rcubarrier_quiz_1>

浣犵殑妯″潡鍙兘浼氭湁棰濆鐨勫鏉傛儏鍐点€備緥濡傦紝濡傛灉浣犵殑妯″潡浠庡畾鏃跺櫒锛坱imer锛変腑璋冪敤
`call_rcu()`锛屼綘灏嗛渶瑕佸厛鍋滄鎻愪氦鏂扮殑瀹氭椂鍣ㄣ€佸彇娑堬紙鎴栫瓑寰咃級鎵€鏈夊凡缁忔彁浜ょ殑
瀹氭椂鍣紝鐒跺悗鎵嶈兘璋冪敤 `rcu_barrier()` 鏉ョ瓑寰呬换浣曞墿浣欑殑 RCU 鍥炶皟瀹屾垚銆?
褰撶劧锛屽鏋滀綘鐨勬ā鍧椾娇鐢?`call_rcu()`锛屼綘闇€瑕佸湪鍗歌浇鍓嶈皟鐢?`rcu_barrier()`銆?绫讳技鍦帮紝濡傛灉浣犵殑妯″潡浣跨敤 `call_srcu()`锛屼綘闇€瑕佸湪鍗歌浇鍓嶈皟鐢?`srcu_barrier()`锛?涓旇鍦ㄥ悓涓€涓?`srcu_struct` 缁撴瀯浣撲笂銆傚鏋滀綘鐨勬ā鍧楀悓鏃朵娇鐢ㄤ簡 `call_rcu()`
**鍜?* `call_srcu()`锛岄偅涔堬紙濡備笂鎵€杩帮級浣犻渶瑕佸悓鏃惰皟鐢?`rcu_barrier()`
**鍜?* `srcu_barrier()`銆?
### 瀹炵幇 rcu_barrier()


Dipankar Sarma 瀵?`rcu_barrier()` 鐨勫疄鐜板埄鐢ㄤ簡杩欐牱涓€涓簨瀹烇細涓€鏃?RCU 鍥炶皟琚?鎺掗槦鍒版煇涓瘡-CPU 闃熷垪涓婏紝瀹冧滑灏辨案杩滀笉浼氳閲嶆帓搴忋€備粬鐨勫疄鐜板湪姣忎釜姣?CPU 鍥炶皟
闃熷垪涓婇兘鎺掗槦涓€涓?RCU 鍥炶皟锛岀劧鍚庣瓑寰呭畠浠叏閮ㄥ紑濮嬫墽琛岋紱姝ゆ椂锛屾墍鏈夋洿鏃╃殑 RCU
鍥炶皟灏变繚璇佸凡缁忓畬鎴愪簡銆?```
  1  void rcu_barrier(void)
  2  {
  3    BUG_ON(in_interrupt());
  4    /* Take cpucontrol mutex to protect against CPU hotplug */
  5    mutex_lock(&rcu_barrier_mutex);
  6    init_completion(&rcu_barrier_completion);
  7    atomic_set(&rcu_barrier_cpu_count, 1);
  8    on_each_cpu(rcu_barrier_func, NULL, 0, 1);
  9    if (atomic_dec_and_test(&rcu_barrier_cpu_count))
 10      complete(&rcu_barrier_completion);
 11    wait_for_completion(&rcu_barrier_completion);
 12    mutex_unlock(&rcu_barrier_mutex);
 13  }
```
绗?3 琛岄獙璇佽皟鐢ㄨ€呭浜庤繘绋嬩笂涓嬫枃锛岀 5 琛屽拰绗?12 琛屼娇鐢?`rcu_barrier_mutex`
纭繚鍚屼竴鏃跺埢鍙湁涓€涓?`rcu_barrier()` 鍦ㄤ娇鐢ㄥ叏灞€ completion 鍜岃鏁板櫒锛岃繖浜?鍦?6銆? 琛岃鍒濆鍖栥€傜 8 琛屼娇姣忎釜 CPU 璋冪敤 `rcu_barrier_func()`锛屽涓嬫墍绀恒€?娉ㄦ剰锛宍on_each_cpu()` 鍙傛暟鍒楄〃鏈熬鐨?鈥?鈥?纭繚浜嗘墍鏈夊 `rcu_barrier_func()`
鐨勮皟鐢ㄩ兘灏嗗湪 `on_each_cpu()` 杩斿洖鍓嶅畬鎴愩€傜 9 琛屼粠 `rcu_barrier_cpu_count`
涓Щ闄ゅ垵濮嬭鏁帮紝濡傛灉璇ヨ鏁扮幇鍦ㄤ负闆讹紝绗?10 琛屽氨瀹屾垚 completion锛屼粠鑰岄樆姝㈢ 11
琛岄樆濉炪€傛棤璁哄摢绉嶆儏鍐碉紝绗?11 琛岄殢鍚庯紙濡傛灉闇€瑕侊級绛夊緟 completion銆?
Quick Quiz #2:
	Why doesn't line 8 initialize rcu_barrier_cpu_count to zero,
	thereby avoiding the need for lines 9 and 10?

Answer to Quick Quiz #2 <answer_rcubarrier_quiz_2>

杩欐浠ｇ爜鍦?2008 骞翠互鍙婃鍚庡張閲嶅啓浜嗗嚑娆★紝浣嗗ぇ浣撴€濊矾渚濈劧濡傛銆?
`rcu_barrier_func()` 鍦ㄦ瘡涓?CPU 涓婅繍琛岋紝鍦ㄨ繍琛屾椂瀹冭皟鐢?`call_rcu()`锛?```
  1  static void rcu_barrier_func(void *notused)
  2  {
  3    int cpu = smp_processor_id();
  4    struct rcu_data *rdp = &per_cpu(rcu_data, cpu);
  5    struct rcu_head *head;
  6
  7    head = &rdp->barrier;
  8    atomic_inc(&rcu_barrier_cpu_count);
  9    call_rcu(head, rcu_barrier_callback);
 10  }
```
绗?3銆? 琛屽畾浣?RCU 鍐呴儴鐨勬瘡-CPU `rcu_data` 缁撴瀯浣擄紝鍏朵腑鍖呭惈绋嶅悗璋冪敤
`call_rcu()` 鎵€闇€鐨?`struct rcu_head`銆傜 7 琛屽彇寰楁寚鍚戣 `struct rcu_head`
鐨勬寚閽堬紝绗?8 琛岄€掑鍏ㄥ眬璁℃暟鍣ㄣ€傝璁℃暟鍣ㄧ◢鍚庝細琚洖璋冮€掑噺銆傜 9 琛岄殢鍚庡湪褰撳墠
CPU 鐨勯槦鍒椾笂娉ㄥ唽 `rcu_barrier_callback()`銆?
`rcu_barrier_callback()` 鍑芥暟鍙槸鍘熷瓙鍦伴€掑噺 `rcu_barrier_cpu_count` 鍙橀噺锛?骞跺湪鍏朵负闆舵椂瀹屾垚 completion锛?```
  1  static void rcu_barrier_callback(struct rcu_head *notused)
  2  {
  3    if (atomic_dec_and_test(&rcu_barrier_cpu_count))
  4      complete(&rcu_barrier_completion);
  5  }
```

Quick Quiz #3:
	What happens if CPU 0's rcu_barrier_func() executes
	immediately (thus incrementing rcu_barrier_cpu_count to the
	value one), but the other CPU's rcu_barrier_func() invocations
	are delayed for a full grace period? Couldn't this result in
	rcu_barrier() returning prematurely?

Answer to Quick Quiz #3 <answer_rcubarrier_quiz_3>

褰撳墠鐨?`rcu_barrier()` 瀹炵幇鏇翠负澶嶆潅锛屽洜涓洪渶瑕侀伩鍏嶆墦鎵扮┖闂?CPU锛堝挨鍏舵槸鍦?鐢垫睜渚涚數鐨勭郴缁熶笂锛夛紝骞朵笖闇€瑕佹渶灏忓寲瀵瑰疄鏃剁郴缁熼潪绌洪棽 CPU 鐨勬墦鎵般€傛澶栬繕搴旂敤浜?澶ч噺浼樺寲銆備笉杩囷紝涓婇潰鐨勪唬鐮佽鏄庝簡鍏跺師鐞嗐€?
### rcu_barrier() 灏忕粨


`rcu_barrier()` 鍘熻浣跨敤寰楃浉瀵硅緝灏戯紝鍥犱负澶у鏁颁娇鐢?RCU 鐨勪唬鐮佷綅浜庢牳蹇冨唴鏍?鑰岄潪妯″潡涓€備笉杩囷紝濡傛灉浣犲湪鍙嵏杞芥ā鍧椾腑浣跨敤 RCU锛屽氨闇€瑕佷娇鐢?`rcu_barrier()`锛?浠ヤ究浣犵殑妯″潡鑳藉琚畨鍏ㄥ嵏杞姐€?
### 蹇€熸祴楠岀瓟妗?

Quick Quiz #1:
	Is there any other situation where rcu_barrier() might
	be required?

Answer:
	鏈夎叮鐨勬槸锛宍rcu_barrier()` 鏈€鍒濆苟涓嶆槸涓哄疄鐜版ā鍧楀嵏杞借€屽疄鐜扮殑銆侼ikita
	Danilov 鏇惧湪涓€涓枃浠剁郴缁熶腑浣跨敤 RCU锛岀粨鏋滃湪鏂囦欢绯荤粺鍗歌浇鏃堕亣鍒颁簡绫讳技鐨?	鎯呭喌銆侱ipankar Sarma 涓烘缂栧啓浜?`rcu_barrier()`锛屼互渚?Nikita 鍙互鍦?	鏂囦欢绯荤粺鍗歌浇杩囩▼涓皟鐢ㄥ畠銆?
	寰堜箙浠ュ悗锛屾湰浜哄湪瀹炵幇 rcutorture 鏃堕亣鍒颁簡 RCU 妯″潡鍗歌浇闂锛屽苟鍙戠幇
	`rcu_barrier()` 鍚屾牱瑙ｅ喅浜嗚繖涓棶棰樸€?
Back to Quick Quiz #1 <rcubarrier_quiz_1>


Quick Quiz #2:
	Why doesn't line 8 initialize rcu_barrier_cpu_count to zero,
	thereby avoiding the need for lines 9 and 10?

Answer:
	鍋囪绗?8 琛屾墍绀虹殑 `on_each_cpu()` 鍑芥暟琚欢杩熶簡锛屼娇寰?CPU 0 鐨?	`rcu_barrier_func()` 鍏堟墽琛屻€佸搴旂殑瀹介檺鏈熶篃鍏堢粨鏉燂紝鑰岃繖涓€鍒囬兘鍙戠敓鍦?	CPU 1 鐨?`rcu_barrier_func()` 寮€濮嬫墽琛屼箣鍓嶃€傝繖灏嗗鑷?`rcu_barrier_cpu_count`
	琚€掑噺鍒伴浂锛屼粠鑰岀 11 琛岀殑 `wait_for_completion()` 浼氱珛鍗宠繑鍥烇紝鏈兘绛夊緟
	CPU 1 鐨勫洖璋冭璋冪敤銆?
	娉ㄦ剰锛屽湪 `rcu_barrier()` 浠ｇ爜浜?2005 骞撮娆″姞鍏ユ椂锛岃繖骞朵笉鏄竴涓?	闂銆傝繖鏄洜涓?`on_each_cpu()` 浼氱鐢ㄦ姠鍗狅紝鑰岀鐢ㄦ姠鍗犵瓑鍚屼簬涓€涓?RCU
	璇荤涓寸晫鍖猴紝浠庤€岄樆姝?CPU 0 鐨勫闄愭湡鍦?`on_each_cpu()` 澶勭悊瀹屾墍鏈?CPU
	涔嬪墠瀹屾垚銆?
	涓嶈繃锛岄殢鐫€ v4.20 鍓嶅悗鐨?RCU 绫诲瀷鍚堝苟锛岃繖绉嶅彲鑳芥€у啀娆¤鎺掗櫎锛屽洜涓哄悎骞跺悗
	鐨?RCU 浼氬啀娆＄瓑寰呴潪鎶㈠崰鐨勪唬鐮佸尯鍩熴€?
	灏界濡傛锛岄偅涓澶栫殑璁℃暟鍙兘浠嶆槸涓ソ涓绘剰銆備緷璧栬繖绫诲疄鐜颁笂鐨勫伓鐒舵€э紝鍙兘浼?	鍦ㄥ疄鐜板彂鐢熷彉鍖栨椂瀵艰嚧鏃ュ悗浠や汉鎯婅鐨?bug銆?
Back to Quick Quiz #2 <rcubarrier_quiz_2>


Quick Quiz #3:
	What happens if CPU 0's rcu_barrier_func() executes
	immediately (thus incrementing rcu_barrier_cpu_count to the
	value one), but the other CPU's rcu_barrier_func() invocations
	are delayed for a full grace period? Couldn't this result in
	rcu_barrier() returning prematurely?

Answer:
	杩欑鎯呭喌涓嶄細鍙戠敓銆傚師鍥犲湪浜?`on_each_cpu()` 鐨勬渶鍚庝竴涓弬鏁般€佸嵆绛夊緟鏍囧織
	琚涓?鈥?鈥濄€傝鏍囧織浼氳浼犻€掕繘 `smp_call_function()`锛屽苟杩涗竴姝ヤ紶閫掑埌
	`smp_call_function_on_cpu()`锛屼娇寰楀悗鑰呰嚜鏃嬶紝鐩村埌璺?CPU 鐨?`rcu_barrier_func()`
	璋冪敤瀹屾垚涓烘銆傝繖鏈韩灏辫兘闃绘瀹介檺鏈熷湪闈?`CONFIG_PREEMPTION` 鍐呮牳涓婂畬鎴愶紝
	鍥犱负鍦ㄥ闄愭湡瀹屾垚涔嬪墠锛屾瘡涓?CPU 閮藉繀椤荤粡鍘嗕竴娆′笂涓嬫枃鍒囨崲锛堟垨鍏朵粬闈欐鐘舵€侊級銆?	鐒惰€岋紝杩欏湪 `CONFIG_PREEMPTION` 鍐呮牳涓鏃犵敤澶勩€?
	鍥犳锛宍on_each_cpu()` 浼氬湪鍏惰皟鐢?`smp_call_function()` 鐨勬暣涓繃绋嬩腑浠ュ強
	鏈湴璋冪敤 `rcu_barrier_func()` 鐨勮繃绋嬩腑绂佺敤鎶㈠崰銆傜敱浜庤繎鏈熺殑 RCU 瀹炵幇灏嗙鐢?	鎶㈠崰鐨勪唬鐮佸尯鍩熻涓?RCU 璇荤涓寸晫鍖猴紝杩欏氨闃绘浜嗗闄愭湡瀹屾垚銆傝繖鎰忓懗鐫€鍦ㄦ墍鏈?	CPU 閮芥墽琛屽畬 `rcu_barrier_func()` 涔嬪墠锛岀涓€涓?`rcu_barrier_callback()`
	閮戒笉鍙兘鎵ц锛岃繘鑰岄樆姝?`rcu_barrier_cpu_count` 杩囨棭鍦拌揪鍒伴浂銆?
	浣嗘槸锛屽鏋?`on_each_cpu()` 鍐冲畾鏀惧純绂佺敤鎶㈠崰锛堢敱浜庡疄鏃跺欢杩熸柟闈㈢殑鑰冮噺杩欏緢
	鏈夊彲鑳藉彂鐢燂級锛岄偅涔堝皢 `rcu_barrier_cpu_count` 鍒濆鍖栦负 1 灏变細鎸芥晳灞€闈€?
Back to Quick Quiz #3 <rcubarrier_quiz_3>
