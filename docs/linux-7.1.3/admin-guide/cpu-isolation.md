
## CPU 闅旂

## 绠€浠?
鈥淐PU 闅旂鈥濇槸鎸囪涓€涓?CPU 涓撶敤浜庣粰瀹氱殑宸ヤ綔璐熻浇锛岃€屾病鏈変换浣曟潵鑷唴鏍哥殑涓嶆湡鏈涚殑浠ｇ爜骞叉壈銆?
杩欎簺骞叉壈閫氬父琚О涓衡€滃櫔澹扳€濓紝鍙兘鐢卞紓姝ヤ簨浠讹紙涓柇銆佸畾鏃跺櫒銆佺敱宸ヤ綔闃熷垪鍜屽唴鏍哥嚎绋嬪紩璧风殑璋冨害鍣?鎶㈠崰鈥︹€︼級鎴栧悓姝ヤ簨浠讹紙绯荤粺璋冪敤鍜岀己椤碉級瑙﹀彂銆?
杩欑鍣０閫氬父涓嶆槗琚療瑙夈€傛瘯绔燂紝鍚屾浜嬩欢鏄墍璇锋眰鐨勫唴鏍告湇鍔＄殑缁勬垚閮ㄥ垎銆傝€屽紓姝ヤ簨浠惰涔堜綔涓轰换鍔?鎵ц鏃惰璋冨害鍣ㄥ厖鍒嗗潎鍖€鍦板垎甯冿紝瑕佷箞浣滀负涓柇鎵ц鏃惰冻澶熷揩銆傚畾鏃跺櫒涓柇鐢氳嚦姣忕鍙互鎵ц 1024 娆★紝
鑰屽ぇ澶氭暟鏃跺€欎笉浼氫骇鐢熸樉钁椾笖鍙祴閲忕殑褰卞搷銆?
鐒惰€屼竴浜涚綍瑙佷笖鏋佺鐨勫伐浣滆礋杞藉彲鑳藉杩欑被鍣０鐩稿綋鏁忔劅銆備緥濡傞珮甯﹀缃戠粶澶勭悊锛堜笉鑳戒涪澶卞崟涓暟鎹寘锛?鎴栨瀬浣庡欢杩熺綉缁滃鐞嗗氨鏄繖绉嶆儏鍐点€傝繖绫荤敤渚嬮€氬父娑夊強 DPDK锛岀粫杩囧唴鏍哥綉缁滄爤骞朵粠鐢ㄦ埛绌洪棿鐩存帴璁块棶
缃戠粶璁惧銆?
涓轰簡鍦ㄦ病鏈夋垨浠呮湁鏈夐檺鍐呮牳鍣０鐨勬儏鍐典笅杩愯涓€涓?CPU锛岀浉鍏崇殑鍐呭姟锛坔ousekeeping锛夊伐浣滆涔堥渶瑕佽
鍏抽棴锛岃涔堣杩佺Щ锛岃涔堣鍗歌浇銆?
## 鍐呭姟澶勭悊锛圚ousekeeping锛?
鍦?CPU 闅旂鏈涓紝housekeeping 鏄唴鏍镐负浜嗙淮鎸佸叾鎵€鏈夋湇鍔¤€岄渶瑕佸鐞嗙殑宸ヤ綔锛岄€氬父鏄紓姝ョ殑銆傚畠
瀵瑰簲浜庝笂闈㈠垪涓剧殑鍣０锛岄櫎闈炶嚦灏戞湁涓€涓?CPU 琚殧绂汇€傚綋瀛樺湪琚殧绂荤殑 CPU 鏃讹紝濡傛灉涓?CPU 缁戝畾鐨勫伐浣?蹇呴』琚嵏杞斤紝閭ｄ箞 housekeeping 鍙兘浼氫娇鐢ㄨ繘涓€姝ョ殑搴斿鏈哄埗銆?
Housekeeping CPU 鏄偅浜涢潪闅旂鐨?CPU锛屽唴鏍稿櫔澹颁細浠庨殧绂荤殑 CPU 涓婅縼绉诲埌杩欎簺 CPU 涓娿€?
闅旂鍙互鏍规嵁鍣０鐨勬€ц川浠ュ绉嶆柟寮忓疄鐜帮細

- 鏈粦瀹氾紙unbound锛夊伐浣滐紝鍏朵腑鈥滄湭缁戝畾鈥濇寚涓嶄笌浠讳綍 CPU 缁戝畾锛屽彲浠ョ畝鍗曞湴浠庨殧绂荤殑 CPU 杩佺Щ鍒?  housekeeping CPU銆傛湭缁戝畾鐨勫伐浣滈槦鍒椼€佸唴鏍哥嚎绋嬪拰瀹氭椂鍣ㄥ氨鏄繖绉嶆儏鍐点€?
- 缁戝畾锛坆ound锛夊伐浣滐紝鍏朵腑鈥滅粦瀹氣€濇寚涓庣壒瀹?CPU 缁戝畾锛岄€氬父鍥犲叾鎬ц川鑰屾棤娉曞師鏍风Щ璧般€傝涔堬細

 - 璇ュ伐浣滃繀椤诲垏鎹㈠埌涓€绉嶅姞閿佺殑瀹炵幇銆備緥濡傦細閰嶇疆浜?CONFIG_RCU_NOCB_CPU 鐨?RCU 灏辨槸杩欑鎯呭喌銆?
 - 鐩稿叧鐗规€у繀椤昏鍏抽棴锛屽苟瑙嗕负涓庨殧绂荤殑 CPU 涓嶅吋瀹广€備緥濡傦細Lockup 鐪嬮棬鐙椼€佷笉鍙潬鐨勬椂閽熸簮绛夈€?
 - 涓€绉嶇簿缁嗕笖閲嶉噺绾х殑搴斿鏈哄埗浣滀负鏇夸唬銆備緥濡傦細鍦?nohz_full CPU 涓婂畾鏃跺櫒婊寸瓟琚叧闂紝浣嗙害鏉熸槸
   鍏朵笂鍙兘杩愯鍗曚釜浠诲姟銆傚唴鏍歌繘鍏?閫€鍑轰細澧炲姞鏄捐憲鐨勬垚鏈紑閿€锛屽苟涓旀畫鐣欑殑 1Hz 璋冨害鍣ㄦ淮绛旇
   鍗歌浇鍒?housekeeping CPU銆?
鏃犺濡備綍锛宧ousekeeping 宸ヤ綔閮藉繀椤昏澶勭悊锛岃繖灏辨槸涓轰粈涔堢郴缁熶腑蹇呴』鑷冲皯鏈変竴涓?housekeeping CPU锛?濡傛灉鏈哄櫒杩愯澶ч噺 CPU锛屾渶濂芥洿澶氥€備緥濡傚湪 NUMA 绯荤粺涓婃瘡涓妭鐐逛竴涓€?
姝ゅ锛孋PU 闅旂閫氬父鎰忓懗鐫€鍦ㄦ棤鍣０鐨勯殧绂?CPU 涓?housekeeping CPU 涓婂鍔犵殑寮€閿€涔嬮棿杩涜鏉冭　锛?鏈夋椂鐢氳嚦鍖呮嫭杩涘叆鍐呮牳鐨勯殧绂?CPU銆?
## 闅旂鐗规€?
鍙互鍦ㄥ唴鏍镐腑閰嶇疆涓嶅悓绾у埆鐨勯殧绂伙紝姣忕閮芥湁鍏惰嚜韬殑缂虹偣鍜屾潈琛°€?
### 璋冨害鍩熼殧绂?
璇ョ壒鎬у皢 CPU 浠庤皟搴﹀櫒鎷撴墤涓殧绂诲嚭鏉ャ€傜粨鏋滐紝鐩爣涓嶅啀鍙備笌璐熻浇鍧囪　銆備换鍔′篃涓嶄細杩佺Щ鍒板畠鎴栦粠瀹?杩佺Щ璧帮紝闄ら潪鏄惧紡璁剧疆浜嗕翰鍜屾€с€?
浣滀负鍓綔鐢紝璇?CPU 涔熶粠鏈粦瀹氱殑宸ヤ綔闃熷垪鍜屾湭缁戝畾鐨勫唴鏍哥嚎绋嬩腑闅旂鍑烘潵銆?
#### 闇€姹?
- 鍩轰簬 cpusets 鐨勬帴鍙ｉ渶瑕?CONFIG_CPUSETS=y

#### 鏉冭　

灏卞叾鏈川鑰岃█锛岀敱浜庝竴浜?CPU 浠庡叏灞€璐熻浇鍧囪　涓鎶界锛岀郴缁熻礋杞芥€讳綋涓婂垎甯冨緱鏇村皯銆?
#### 鎺ュ彛

- 鎺ㄨ崘 Documentation/admin-guide/cgroup-v2.rst 涓殑 cpuset 闅旂鍒嗗尯锛屽洜涓哄畠浠彲浠ュ湪杩愯鏃?  璋冩暣銆?
- 'isolcpus=' 鍐呮牳鍚姩鍙傛暟甯︽湁 'domain' 鏍囧織锛屾槸涓€涓伒娲绘€ц緝浣庣殑鏇夸唬鏂规锛屼笉鍏佽杩愯鏃堕噸鏂?  閰嶇疆銆?
### IRQ 闅旂

灏藉彲鑳介殧绂?IRQ锛屼娇鍏朵笉鍦ㄧ洰鏍?CPU 涓婅Е鍙戙€?
#### 鎺ュ彛

- 鏂囦欢 /proc/irq/\*/smp_affinity锛岃瑙?Documentation/core-api/irq/irq-affinity.rst 椤甸潰銆?
- 鐢ㄤ簬榛樿璁剧疆鐨?"irqaffinity=" 鍐呮牳鍚姩鍙傛暟銆?
- "isolcpus=" 鍐呮牳鍚姩鍙傛暟涓殑 "managed_irq" 鏍囧織浼氬鍙楃鐞嗙殑 IRQ 灏藉姏杩涜浜插拰鎬ц鐩栥€?
### 瀹屽叏鍔ㄦ€?tick锛團ull Dynticks锛屽嵆 nohz_full锛?
瀹屽叏鍔ㄦ€?tick 灏?dynticks 绌洪棽妯″紡锛圕PU 绌洪棽鏃跺仠姝?tick锛夋墿灞曞埌杩愯鍗曚釜鐢ㄦ埛绌洪棿浠诲姟鐨?CPU銆?涔熷氨鏄锛屽鏋滅幆澧冨厑璁革紝瀹氭椂鍣?tick 浼氳鍋滄銆?
鍏ㄥ眬瀹氭椂鍣ㄥ洖璋冧篃浠?nohz_full CPU 涓婇殧绂汇€?
#### 闇€姹?
- CONFIG_NO_HZ_FULL=y

#### 绾︽潫

- 闅旂鐨?CPU 蹇呴』鍙繍琛屽崟涓换鍔°€傚浠诲姟闇€瑕?tick 鏉ョ淮鎸佹姠鍗犮€傝繖閫氬父娌￠棶棰橈紝鍥犱负宸ヤ綔璐熻浇閫氬父
  鏃犳硶鎵垮彈闅忔満涓婁笅鏂囧垏鎹㈢殑寤惰繜銆?
- 闅旂鐨?CPU 涓嶅緱璋冪敤鍐呮牳锛屽惁鍒欐湁瑙﹀彂闅忔満鍣０鐨勯闄┿€?
- 闅旂鐨?CPU 涓婁笉寰椾娇鐢?POSIX CPU 瀹氭椂鍣ㄣ€?
- 鏋舵瀯蹇呴』鎷ユ湁绋冲畾鍙潬鐨勬椂閽熸簮锛堟病鏈夐渶瑕佺湅闂ㄧ嫍鐨勪笉鍙潬 TSC锛夈€?
#### 鏉冭　

灏辨垚鏈€岃█锛岃繖鏄镜鍏ユ€ф渶寮虹殑闅旂鐗规€с€傚亣瀹氬湪宸ヤ綔璐熻浇灏嗗ぇ閮ㄥ垎鏃堕棿鑺卞湪鐢ㄦ埛绌洪棿銆佸苟涓旈櫎浜嗗噯澶?鎬у伐浣滃涓嶄緷璧栧唴鏍告椂浣跨敤锛屽洜涓猴細

- RCU 鐢变簬鍔犻攣銆佸嵏杞藉拰绾跨▼鍖栫殑鍥炶皟澶勭悊锛堜笌 "rcu_nocbs" 鍚姩鍙傛暟鎵€鑾峰緱鐨勬晥鏋滅浉鍚岋級鑰屽鍔?  浜嗘洿澶氬紑閿€銆?
- 閫氳繃绯荤粺璋冪敤銆佸紓甯稿拰 IRQ 杩涘叆/閫€鍑哄唴鏍镐唬浠锋洿楂橈紝鍥犱负瑕佸皢鐢ㄦ埛绌洪棿缁存寔涓?RCU 鎵╁睍闈欐鐘舵€佽€?  杩涜浜嗗畬鍏ㄦ湁搴忕殑 RmW 鎿嶄綔銆傛澶栵紝CPU 鏃堕棿鏄湪鍐呮牳杈圭晫涓婅璐︼紝鑰岄潪閫氳繃 tick 鍛ㄦ湡鎬ц璐︺€?
- Housekeeping CPU 蹇呴』浠ｈ〃闅旂鐨?CPU 杩愯涓€涓?1Hz 鐨勬畫鐣欒繙绋嬭皟搴﹀櫒 tick銆?
## 妫€鏌ユ竻鍗?
浣犲凡缁忚缃簡涓婅堪姣忕闅旂鐗规€э紝浣嗕粛鐒惰瀵熷埌鎶栧姩姣佹帀浜嗕綘鐨勫伐浣滆礋杞斤紵鍦ㄨ繘琛屼箣鍓嶏紝鍔″繀妫€鏌ュ嚑涓?瑕佺偣銆?
鍏朵腑涓€浜涙鏌ユ竻鍗曢」涓庡疄鏃跺伐浣滆礋杞界被浼硷細

- 浣跨敤 mlock() 闃叉浣犵殑椤甸潰琚崲鍑恒€傜己椤甸€氬父涓庡鎶栧姩鏁忔劅鐨勫伐浣滆礋杞戒笉鍏煎銆?
- 閬垮厤 SMT锛屼互闃叉浣犵殑纭欢绾跨▼琚彟涓€涓嚎绋嬧€滄姠鍗犫€濄€?
- CPU 棰戠巼鍙樺寲鍙兘鍦ㄥ伐浣滆礋杞戒腑寮曞彂寰鐨勬姈鍔ㄣ€侰pufreq 搴斿綋璋ㄦ厧浣跨敤鍜岃皟浼樸€?
- 娣卞害 C-state 鍙兘鍦ㄥ敜閱掓椂瀵艰嚧寤惰繜闂銆傚鏋滆繖鎴愪负闂锛屽彲浠ラ€氳繃 processor.max_cstate 鎴?  intel_idle.max_cstate 绛夊唴鏍稿惎鍔ㄥ弬鏁伴檺鍒?C-state銆傛洿缁嗙矑搴︾殑璋冧紭鍦?  Documentation/admin-guide/pm/cpuidle.rst 椤甸潰涓弿杩般€?
- 浣犵殑绯荤粺鍙兘浼氬彈鍒版簮鑷浐浠剁殑涓柇褰卞搷鈥斺€斾緥濡?x86 鏈夌郴缁熺鐞嗕腑鏂紙SMI锛夈€傛鏌ヤ綘鐨勭郴缁?BIOS
  浠ョ鐢ㄦ绫诲共鎵帮紝杩愭皵濂界殑璇濅綘鐨勪緵搴斿晢浼氭彁渚涢拡瀵逛綆寤惰繜鎿嶄綔鐨?BIOS 璋冧紭鎸囧崡銆?
## 瀹屽叏闅旂绀轰緥

鍦ㄦ湰渚嬩腑锛岀郴缁熸湁 8 涓?CPU锛岀 8 涓皢琚畬鍏ㄩ殧绂汇€傜敱浜?CPU 浠?0 寮€濮嬬紪鍙凤紝绗?8 涓?CPU 鏄?CPU 7銆?
### 鍐呮牳鍙傛暟

璁剧疆浠ヤ笅鍐呮牳鍚姩鍙傛暟浠ョ鐢?SMT锛屽苟璁剧疆 tick 鍜?IRQ 闅旂锛?
- 瀹屽叏鍔ㄦ€?tick锛歯ohz_full=7

- IRQ 闅旂锛歩rqaffinity=0-6

- 鍙楃鐞?IRQ 闅旂锛歩solcpus=managed_irq,7

- 闃绘 SMT锛歯osmt

瀹屾暣鐨勫懡浠よ濡備笅锛?
  nohz_full=7 irqaffinity=0-6 isolcpus=managed_irq,7 nosmt

### CPUSET 閰嶇疆锛坈group v2锛?
鍋囪 cgroup v2 宸叉寕杞藉埌 /sys/fs/cgroup锛屼互涓嬭剼鏈皢 CPU 7 浠庤皟搴﹀煙涓殧绂汇€?
```
  cd /sys/fs/cgroup
  # Activate the cpuset subsystem
  echo +cpuset > cgroup.subtree_control
  # Create partition to be isolated
  mkdir test
  cd test
  echo +cpuset > cgroup.subtree_control
  # Isolate CPU 7
  echo 7 > cpuset.cpus
  echo "isolated" > cpuset.cpus.partition

```
### 鐢ㄦ埛绌洪棿宸ヤ綔璐熻浇

妯℃嫙涓€涓函鐢ㄦ埛绌洪棿宸ヤ綔璐熻浇锛屼笅闈㈢殑绋嬪簭鍦ㄩ殧绂荤殑 CPU 7 涓婅繍琛屼竴涓┖鐨勭敤鎴风┖闂村惊鐜€?
```
  #include <stdio.h>
  #include <fcntl.h>
  #include <unistd.h>
  #include <errno.h>
  int main(void)
  {
      // Move the current task to the isolated cpuset (bind to CPU 7)
      int fd = open("/sys/fs/cgroup/test/cgroup.procs", O_WRONLY);
      if (fd < 0) {
          perror("Can't open cpuset file...\n");
          return 0;
      }

      write(fd, "0\n", 2);
      close(fd);

      // Run an endless dummy loop until the launcher kills us
      while (1)
      ;

      return 0;
  }

```
缂栬瘧瀹冨苟淇濆瓨浠ュ鍚庣画姝ラ浣跨敤锛?
```
  # gcc user_loop.c -o user_loop

```
### 鍚姩鍣?
涓嬮潰鐨勫惎鍔ㄥ櫒杩愯涓婅堪绋嬪簭 10 绉掞紝骞惰窡韪洜鎶㈠崰浠诲姟鍜?IRQ 鑰屼骇鐢熺殑鍣０銆?
```
  TRACING=/sys/kernel/tracing/
  # Make sure tracing is off for now
  echo 0 > $TRACING/tracing_on
  # Flush previous traces
  echo > $TRACING/trace
  # Record disturbance from other tasks
  echo 1 > $TRACING/events/sched/sched_switch/enable
  # Record disturbance from interrupts
  echo 1 > $TRACING/events/irq_vectors/enable
  # Now we can start tracing
  echo 1 > $TRACING/tracing_on
  # Run the dummy user_loop for 10 seconds on CPU 7
  ./user_loop &
  USER_LOOP_PID=$!
  sleep 10
  kill $USER_LOOP_PID
  # Disable tracing and save traces from CPU 7 in a file
  echo 0 > $TRACING/tracing_on
  cat $TRACING/per_cpu/cpu7/trace > trace.7

```
濡傛灉娌℃湁鍑虹幇鐗瑰畾闂锛宼race.7 鐨勮緭鍑哄簲濡備笅鎵€绀猴細

```
  <idle>-0 [007] d..2. 1980.976624: sched_switch: prev_comm=swapper/7 prev_pid=0 prev_prio=120 prev_state=R ==> next_comm=user_loop next_pid=1553 next_prio=120
  user_loop-1553 [007] d.h.. 1990.946593: reschedule_entry: vector=253
  user_loop-1553 [007] d.h.. 1990.946593: reschedule_exit: vector=253

```
涔熷氨鏄锛屽湪 user_loop 杩愯鐨?10 绉掑唴锛岀涓€娆¤窡韪拰绗簩娆¤窡韪箣闂存病鏈夎Е鍙戠壒瀹氱殑鍣０銆?
## 璋冭瘯

褰撶劧浜嬫儏浠庢潵涓嶄細杩欎箞绠€鍗曪紝灏ゅ叾鏄湪杩欎釜闂涓娿€傚緢鍙兘鍦ㄥ墠杩?trace.7 鏂囦欢涓瀵熷埌瀹為檯鐨勫櫔澹般€?
杩涗竴姝ヨ皟鏌ョ殑鏈€浣虫柟娉曟槸鍚敤鏇寸粏绮掑害鐨勮窡韪偣锛屼緥濡備骇鐢熷紓姝ヤ簨浠剁殑瀛愮郴缁熺殑璺熻釜鐐癸細workqueue銆?timer銆乮rq_vector 绛夈€傚惎鐢?tick_stop 浜嬩欢鏉ヨ瘖鏂负浣曞湪閭ｇ鎯呭喌涓?tick 琚繚鐣欎篃寰堟湁鎰忎箟銆?
涓€浜涘伐鍏峰浜庢洿楂樺眰娆＄殑鍒嗘瀽涔熷緢鏈夌敤锛?
- Documentation/tools/rtla/rtla.rst 鎻愪緵浜嗕竴濂楃敤浜庡垎鏋愮郴缁熶腑寤惰繜鍜屽櫔澹扮殑宸ュ叿銆備緥濡?  Documentation/tools/rtla/rtla-osnoise.rst 杩愯涓€涓唴鏍歌窡韪櫒锛屽垎鏋愬苟杈撳嚭鍣０鐨勬憳瑕併€?
- dynticks-testing 鍋氱殑浜嬫儏绫讳技浜?rtla-osnoise锛屼絾鍦ㄧ敤鎴风┖闂磋繘琛屻€傚畠浣嶄簬
  git://git.kernel.org/pub/scm/linux/kernel/git/frederic/dynticks-testing.git
