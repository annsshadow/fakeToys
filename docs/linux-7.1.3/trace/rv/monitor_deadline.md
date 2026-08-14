## 鎴鏃堕棿鐩戣鍣紙Deadline monitors锛?

- 鍚嶇О锛歞eadline
- 绫诲瀷锛氬涓洃瑙嗗櫒鐨勫鍣?- 浣滆€咃細Gabriele Monaco <gmonaco@redhat.com>

### 鎻忚堪


deadline 鐩戣鍣ㄦ槸涓€缁勭敤浜庢弿杩版埅姝㈡椂闂磋皟搴﹀櫒锛坉eadline scheduler锛夎涓虹殑
瑙勮寖銆傚畠鍖呭惈閽堝姣忎釜璋冨害瀹炰綋锛堟埅姝㈡椂闂翠换鍔′笌鏈嶅姟鍣級鐨勭洃瑙嗗櫒锛岃繖浜涚洃瑙嗗櫒
鐙珛宸ヤ綔锛屼互楠岃瘉鎴鏃堕棿璋冨害鍣ㄥ簲褰撻伒寰殑涓嶅悓瑙勮寖銆?
### 瑙勮寖


#### 鐩戣鍣?nomiss


nomiss 鐩戣鍣ㄧ‘淇?dl 瀹炰綋鍦ㄥ叾鎴鏃堕棿**涔嬪墠**寰椾互杩愯**骞朵笖**杩愯鑷冲畬鎴愶紝
灏界鍙欢杩燂紙deferrable锛夋湇鍔″櫒鍙兘涓嶈繍琛屻€傚鏋滀竴涓疄浣?`throttled`
锛堟棤璁烘槸鍥犱负瀹冧富鍔ㄨ姝ヨ繕鏄敤瀹屼簡鍏惰繍琛屾椂闂达級锛屾垨鑰呭綋瀹冧富鍔ㄥ紑濮?`sleeping`
鏃讹紝鍗宠瑙嗕负瀹屾垚銆?璇ョ洃瑙嗗櫒鍖呭惈涓€涓敤鎴峰彲閰嶇疆鐨勬埅姝㈡椂闂撮槇鍊笺€傚鏋滄埅姝㈡椂闂翠换鍔＄殑鎬诲埄鐢ㄧ巼
澶т簬 1锛屽垯瀹冧滑浠呬繚璇佹湁鐣屽欢杩燂紙bounded tardiness锛夈€傛洿澶氱粏鑺傝鍙傝
Documentation/scheduler/sched-deadline.rst銆傚彲浠ュ皢闃堝€硷紙妯″潡鍙傛暟
`nomiss.deadline_thresh`锛夐厤缃负閬垮厤鐩戣鍣ㄥ熀浜庣郴缁熶腑鍙帴鍙楃殑寤惰繜鑰屽け璐ャ€?鐢变簬 `dl_throttle` 鏄疄浣撳畬鎴愮殑鍚堟硶缁撴灉锛岄櫎闈?`HRTICK_DL` 璋冨害鍣ㄧ壒鎬у浜庢椿鍔ㄧ姸鎬侊紝鍚﹀垯瑕佽€冭檻鑺傛祦寤惰繜锛屾渶灏忓欢杩熼渶瑕佷负 1 涓?tick銆?
鏈嶅姟鍣ㄨ繕鏈変竴涓腑闂寸殑 `idle` 鐘舵€侊紝鍦ㄦ病鏈変换浣曞彲杩愯浠诲姟锛堜粠 ready 鎴?running锛?涓旀湭鏂藉姞鏃跺簭绾︽潫鏃剁珛鍗冲嚭鐜般€傛湇鍔″櫒閫氳繃鍋滄杩涘叆浼戠湢锛屾病鏈夌瓑鏁堢殑鍞ら啋锛?鍥犱负鏈嶅姟鍣ㄥ惎鍔ㄤ笌琛ュ厖鐨勯『搴忔湭瀹氫箟锛屽洜姝や竴涓?```

                                  |
  sched_wakeup                    v
  dl_replenish;reset(clk) -- #=========================#
               |             H                         H dl_replenish;reset(clk)
               +-----------> H                         H <--------------------+
                             H                         H                      |
      +- dl_server_stop ---- H          ready          H                      |
      |  +-----------------> H   clk < DEADLINE_NS()   H   dl_throttle;       |
      |  |                   H                         H     is_defer == 1    |
      |  | sched_switch_in - H                         H -----------------+   |
      |  |   |               #=========================#                  |   |
      |  |   |                       |            ^                       |   |
      |  |   |             dl_server_idle    dl_replenish;reset(clk)      |   |
      |  |   |                       v            |                       |   |
      |  |   |                      +--------------+                      |   |
      |  |   |              +------ |              |                      |   |
      |  |   |     dl_server_idle   |              | dl_throttle          |   |
      |  |   |              |       |     idle     | -----------------+   |   |
      |  |   |              +-----> |              |                  |   |   |
      |  |   |                      |              |                  |   |   |
      |  |   |                      |              |                  |   |   |
   +--+--+---+--- dl_server_stop -- +--------------+                  |   |   |
   |  |  |   |                       |           ^                    |   |   |
   |  |  |   |            sched_switch_in    dl_server_idle           |   |   |
   |  |  |   |                       v           |                    |   |   |
   |  |  |   |      +---------- +---------------------+               |   |   |
   |  |  |   | sched_switch_in  |                     |               |   |   |
   |  |  |   | sched_wakeup     |                     |               |   |   |
   |  |  |   | dl_replenish;    |      running        | -------+      |   |   |
   |  |  |   |      reset(clk)  | clk < DEADLINE_NS() |        |      |   |   |
   |  |  |   |      +---------> |                     | dl_throttle   |   |   |
   |  |  |   +----------------> |                     |        |      |   |   |
   |  |  |                      +---------------------+        |      |   |   |
   |  | sched_wakeup                ^   sched_switch_suspend   |      |   |   |
   v  v dl_replenish;reset(clk)     |   dl_server_stop         |      |   |   |
 +--------------+                   |   |                      v      v   v   |
 |              | - sched_switch_in +   |                     +---------------+
 |              | <---------------------+     dl_throttle +-- |               |
 |   sleeping   |                            sched_wakeup |   |   throttled   |
 |              | -- dl_server_stop        dl_server_idle +-> |               |
 |              |    dl_server_idle     sched_switch_suspend  +---------------+
 +--------------+ <---------+                                        ^
        |                                                            |
        +------ dl_throttle;is_constr_dl == 1 || is_defer == 1 ------+

```
