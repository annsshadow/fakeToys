
## Linux 涓殑閾捐〃


:Author: Nicolas Frattaroli <nicolas.frattaroli@collabora.com>


## 绠€浠?

閾捐〃鏄澶氱▼搴忎腑鏈€鍩烘湰鐨勬暟鎹粨鏋勪箣涓€銆侺inux 鍐呮牳瀹炵幇浜嗗嚑绉嶄笉鍚岄鏍肩殑閾捐〃銆傛湰鏂囨。
鐨勭洰鐨勫苟闈炴硾娉涘湴瑙ｉ噴閾捐〃锛岃€屾槸鍚戞柊鐨勫唴鏍稿紑鍙戣€呭睍绀哄浣曚娇鐢?Linux 鍐呮牳鐨勯摼琛ㄥ疄鐜般€?
璇锋敞鎰忥紝灏界閾捐〃纭疄鏃犲涓嶅湪锛屼絾鍦ㄧ畝鍗曠殑鏁扮粍宸茬粡鏃犳硶婊¤冻闇€姹傜殑鍦哄悎锛岄摼琛ㄥ緢灏戞槸
鏈€浣崇殑鏁版嵁缁撴瀯閫夋嫨銆傜壒鍒槸锛岀敱浜庡叾鏁版嵁灞€閮ㄦ€э紙data locality锛夎緝宸紝鍦ㄩ渶瑕佽€冭檻鎬ц兘鐨?鎯呭舰涓嬶紝閾捐〃鏄釜绯熺硶鐨勯€夋嫨銆傚己鐑堝缓璁幓鐔熸倝鍐呮牳涓殑鍏朵粬閫氱敤鏁版嵁缁撴瀯锛屽挨鍏舵槸鍦ㄥ苟鍙?璁块棶鏂归潰銆?
## Linux 鍙屽悜閾捐〃鐨勫疄鐜?

Linux 鐨勯摼琛ㄥ疄鐜板彲浠ラ€氳繃鍖呭惈澶存枃浠?`<linux/list.h>` 鏉ヤ娇鐢ㄣ€?
鍙屽悜閾捐〃瀵硅澶氳鑰呮潵璇村彲鑳芥槸鏈€鐔熸倝鐨勩€傚畠鍙互琚珮鏁堝湴姝ｅ悜鍜屽弽鍚戦亶鍘嗐€?
Linux 鍐呮牳鐨勫弻鍚戦摼琛ㄦ湰璐ㄤ笂鏄幆褰㈢殑锛坈ircular锛夈€傝繖鎰忓懗鐫€锛岃浠庡ご鑺傜偣鍒拌揪灏捐妭鐐癸紝
鎴戜滑鍙渶鍚戝悗璧颁竴鏉¤竟锛涚被浼煎湴锛岃浠庡熬鑺傜偣鍒拌揪澶磋妭鐐癸紝鎴戜滑鍙渶鍚戝墠鈥滆秺杩団€濆熬閮紝鍗冲彲
鍥炲埌澶磋妭鐐广€?
### 澹版槑涓€涓妭鐐?

鍙屽悜閾捐〃涓殑鑺傜偣锛屾槸閫氳繃鍦ㄤ綘鎯虫斁鍏ラ摼琛ㄧ殑鏁版嵁缁撴瀯涓坊鍔犱竴涓?struct list_head 鎴愬憳鏉?澹版槑鐨勶細


  struct clown {
          unsigned long long shoe_size;
          const char *name;
          struct list_head node;  /** the aforementioned member **/
  };

瀵逛竴浜涗汉鏉ヨ锛岃繖鍙兘鏄竴绉嶉檶鐢熺殑鍋氭硶锛屽洜涓虹粡鍏哥殑瀵归摼琛ㄧ殑瑙ｉ噴鏄細閾捐〃鑺傜偣鏁版嵁缁撴瀯
甯︽湁鎸囧悜鍓嶄竴涓拰鍚庝竴涓摼琛ㄨ妭鐐圭殑鎸囬拡锛屼互鍙婅礋杞斤紙payload锛夋暟鎹€侺inux 閫夋嫨杩欑鍋氭硶锛?鏄洜涓哄畠鍏佽缂栧啓閫氱敤鐨勯摼琛ㄤ慨鏀逛唬鐮侊紝鑰屾棤闇€鍏冲績閾捐〃涓寘鍚殑鏄粈涔堟暟鎹粨鏋勩€傜敱浜?struct list_head 鎴愬憳涓嶆槸鎸囬拡锛岃€屾槸鏁版嵁缁撴瀯鏈韩鐨勭粍鎴愰儴鍒嗭紝閾捐〃瀹炵幇鍙互浣跨敤
container_of() 妯″紡鏉ヨ闂礋杞芥暟鎹紝鑰屼笉绠″叾绫诲瀷鏄粈涔堬紝鍚屾椂瀵规墍瑷€涔嬬被鍨嬬┒绔熸槸浠€涔?涓€鏃犳墍鐭ャ€?
### 澹版槑骞跺垵濮嬪寲涓€涓摼琛?

鐒跺悗锛屽弻鍚戦摼琛ㄥ彲浠ヨ澹版槑涓哄張涓€涓?struct list_head锛屽苟鍦ㄥ垵濮嬭祴鍊兼椂鐢?LIST_HEAD_INIT()
瀹忓垵濮嬪寲锛屾垨鑰呯◢鍚庣敤 INIT_LIST_HEAD() 鍑芥暟鍒濆鍖栵細


  struct clown_car {
          int tyre_pressure[^4^];
          struct list_head clowns;        /** Looks like a node! **/
  };

  /** ... Somewhere later in our driver ... **/

  static int circus_init(struct circus_priv *circus)
  {
          struct clown_car other_car = {
                .tyre_pressure = {10, 12, 11, 9},
                .clowns = LIST_HEAD_INIT(other_car.clowns)
          };

          INIT_LIST_HEAD(&circus->car.clowns);

          return 0;
  }

鍙︿竴涓彲鑳借涓€浜涗汉鍥版儜鐨勭偣鏄紝閾捐〃鏈韩骞舵病鏈夎嚜宸辩殑绫诲瀷銆傛暣涓摼琛ㄧ殑杩欎竴姒傚康锛屼笌鎸囧悜
閾捐〃涓叾浠栨潯鐩殑 struct list_head 鎴愬憳锛屼簩鑰呮槸鍚屼竴鍥炰簨銆?
### 鍚戦摼琛ㄤ腑娣诲姞鑺傜偣


鍚戦摼琛ㄤ腑娣诲姞鑺傜偣鏄€氳繃 list_add() 瀹忓畬鎴愮殑銆?
鎴戜滑灏嗗洖鍒板皬涓戣溅鐨勪緥瀛愶紝鏉ヨ鏄庤妭鐐规槸濡備綍琚坊鍔犲埌閾捐〃涓殑锛?

  static int circus_fill_car(struct circus_priv *circus)
  {
          struct clown_car *car = &circus->car;
          struct clown *grock;
          struct clown *dimitri;

          /** State 1 **/

          grock = kzalloc(sizeof(*grock), GFP_KERNEL);
          if (!grock)
                  return -ENOMEM;
          grock->name = "Grock";
          grock->shoe_size = 1000;

          /** Note that we're adding the "node" member **/
          list_add(&grock->node, &car->clowns);

          /** State 2 **/

          dimitri = kzalloc(sizeof(*dimitri), GFP_KERNEL);
          if (!dimitri)
                  return -ENOMEM;
          dimitri->name = "Dimitri";
          dimitri->shoe_size = 50;

          list_add(&dimitri->node, &car->clowns);

          /** State 3 **/

          return 0;
  }

```

         .------.
         v      |
    .--------.  |
    | clowns |--'
    '--------'

```
璇ュ浘鏄剧ず浜嗗崟鐙殑 "clowns" 鑺傜偣鎸囧悜鑷韩銆傚湪鏈浘浠ュ強涔嬪悗鎵€鏈夊浘涓紝涓轰簡娓呮櫚璧疯锛屽彧鐢?鍑轰簡姝ｅ悜鐨勮竟銆?
```

         .--------------------.
         v                    |
    .--------.     .-------.  |
    | clowns |---->| Grock |--'
    '--------'     '-------'

```
璇ュ浘鏄剧ず浜?"clowns" 鑺傜偣鎸囧悜涓€涓爣璁颁负 "Grock" 鐨勬柊鑺傜偣銆侴rock 鑺傜偣鎸囧洖浜?"clowns"
鑺傜偣銆?
```

         .------------------------------------.
         v                                    |
    .--------.     .---------.     .-------.  |
    | clowns |---->| Dimitri |---->| Grock |--'
    '--------'     '---------'     '-------'

```
璇ュ浘鏄剧ず浜?"clowns" 鑺傜偣鎸囧悜涓€涓爣璁颁负 "Dimitri" 鐨勬柊鑺傜偣锛岃€岃鑺傜偣鍙堟寚鍚戞爣璁颁负
"Grock" 鐨勮妭鐐广€?Grock" 鑺傜偣浠嶆寚鍥?"clowns" 鑺傜偣銆?
濡傛灉鎴戜滑鎯宠 Dimitri 鎻掑埌閾捐〃鏈熬锛屽氨搴斾娇鐢?list_add_tail()銆傛垜浠殑浠ｇ爜灏嗗涓嬫墍绀猴細


  static int circus_fill_car(struct circus_priv *circus)
  {
          /** ... **/

          list_add_tail(&dimitri->node, &car->clowns);

          /** State 3b **/

          return 0;
  }

```

         .------------------------------------.
         v                                    |
    .--------.     .-------.     .---------.  |
    | clowns |---->| Grock |---->| Dimitri |--'
    '--------'     '-------'     '---------'

```
璇ュ浘鏄剧ず浜?"clowns" 鑺傜偣鎸囧悜鏍囪涓?"Grock" 鐨勮妭鐐癸紝鑰屽悗鑰呭張鎸囧悜鏍囪涓?"Dimitri" 鐨勬柊
鑺傜偣銆?Dimitri" 鑺傜偣鎸囧洖 "clowns" 鑺傜偣銆?
### 閬嶅巻閾捐〃


瑕侀亶鍘嗛摼琛紝鎴戜滑鍙互鐢?list_for_each() 寰幆缁忚繃閾捐〃涓殑鎵€鏈夎妭鐐广€?
鍦ㄦ垜浠殑灏忎笐渚嬪瓙涓紝杩欎細寰楀嚭濡備笅鏈変簺绗ㄦ嫏鐨勪唬鐮侊細


  static unsigned long long circus_get_max_shoe_size(struct circus_priv *circus)
  {
          unsigned long long res = 0;
          struct clown *e;
          struct list_head *cur;

          list_for_each(cur, &circus->car.clowns) {
                  e = list_entry(cur, struct clown, node);
                  if (e->shoe_size > res)
                          res = e->shoe_size;
          }

          return res;
  }

list_entry() 瀹忓湪鍐呴儴浣跨敤浜嗗墠闈㈡彁鍒扮殑 container_of() 鏉ュ彇鍥?`node` 浣滀负鍏舵垚鍛樼殑閭ｄ釜
鏁版嵁缁撴瀯瀹炰緥銆?
娉ㄦ剰杩欓噷棰濆鐨?list_entry() 璋冪敤鏈夌偣绗ㄦ嫏銆傚畠涔嬫墍浠ュ瓨鍦紝鏄洜涓烘垜浠槸鍦ㄩ亶鍘?`node`
鎴愬憳锛屼絾鎴戜滑鐪熸鎯宠閬嶅巻鐨勬槸璐熻浇锛屽嵆鍖呭惈姣忎釜鑺傜偣鐨?struct list_head 鐨勯偅涓?`struct clown`銆備负姝わ紝鎻愪緵浜嗙浜屼釜瀹忥細list_for_each_entry()

浣跨敤瀹冧細鎶婃垜浠殑浠ｇ爜鏀规垚绫讳技杩欐牱锛?

  static unsigned long long circus_get_max_shoe_size(struct circus_priv *circus)
  {
          unsigned long long res = 0;
          struct clown *e;

          list_for_each_entry(e, &circus->car.clowns, node) {
                  if (e->shoe_size > res)
                          res = e->shoe_size;
          }

          return res;
  }

杩欑渷鍘讳簡 list_entry() 杩欎竴姝ワ紝鎴戜滑鐨勫惊鐜父鏍囩幇鍦ㄦ槸鎴戜滑璐熻浇鐨勭被鍨嬨€傝瀹忚缁欏畾浜嗗湪
clown 鏁版嵁缁撴瀯涓搴斾簬閾捐〃鐨?struct list_head 鐨勯偅涓垚鍛樺悕锛岃繖鏍峰畠浠嶇劧鑳藉閬嶅巻閾捐〃銆?
### 浠庨摼琛ㄤ腑绉婚櫎鑺傜偣


list_del() 鍑芥暟鍙敤浜庝粠閾捐〃涓Щ闄ゆ潯鐩€傚畠涓嶄粎浠庨摼琛ㄤ腑绉婚櫎缁欏畾鐨勬潯鐩紝杩樹細姣掑寲
锛坧oison锛夎鏉＄洰鐨?`prev` 鍜?`next` 鎸囬拡锛屼娇寰楃Щ闄ゅ悗瀵硅鏉＄洰鐨勬棤鎰忎娇鐢ㄤ笉浼氳蹇借銆?
鎴戜滑鍙互鎵╁睍涔嬪墠鐨勪緥瀛愭潵绉婚櫎鍏朵腑涓€涓潯鐩細


  static int circus_fill_car(struct circus_priv *circus)
  {
          /** ... **/

          list_add(&dimitri->node, &car->clowns);

          /** State 3 **/

          list_del(&dimitri->node);

          /** State 4 **/

          return 0;
  }

```

         .--------------------.
         v                    |
    .--------.     .-------.  |      .---------.
    | clowns |---->| Grock |--'      | Dimitri |
    '--------'     '-------'         '---------'

```
璇ュ浘鏄剧ず浜?"clowns" 鑺傜偣鎸囧悜鏍囪涓?"Grock" 鐨勮妭鐐癸紝鑰屽悗鑰呭張鎸囧洖 "clowns" 鑺傜偣銆傚湪涓€鏃?鏄竴涓绔嬬殑銆佹爣璁颁负 "Dimitri" 鐨勮妭鐐癸紝娌℃湁浠讳綍绠ご鎸囧悜浠讳綍鍦版柟銆?
娉ㄦ剰 Dimitri 鑺傜偣鏄浣曚笉鎸囧悜鑷韩鐨勶紱瀹冪殑鎸囬拡琚晠鎰忚涓轰竴涓€滄瘨鍖栤€濆€硷紝閾捐〃浠ｇ爜鎷掔粷
閬嶅巻瀹冦€?
濡傛灉鎴戜滑鎯虫敼涓洪噸鏂板垵濮嬪寲琚Щ闄ょ殑鑺傜偣锛屼娇鍏跺啀娆″儚绌虹殑閾捐〃澶翠竴鏍锋寚鍚戣嚜韬紝鎴戜滑鍙互
鏀圭敤 list_del_init()锛?

  static int circus_fill_car(struct circus_priv *circus)
  {
          /** ... **/

          list_add(&dimitri->node, &car->clowns);

          /** State 3 **/

          list_del_init(&dimitri->node);

          /** State 4b **/

          return 0;
  }

```

         .--------------------.           .-------.
         v                    |           v       |
    .--------.     .-------.  |      .---------.  |
    | clowns |---->| Grock |--'      | Dimitri |--'
    '--------'     '-------'         '---------'

```
璇ュ浘鏄剧ず浜?"clowns" 鑺傜偣鎸囧悜鏍囪涓?"Grock" 鐨勮妭鐐癸紝鑰屽悗鑰呭張鎸囧洖 "clowns" 鑺傜偣銆傚湪涓€鏃?鏄竴涓绔嬬殑銆佹爣璁颁负 "Dimitri" 鐨勮妭鐐癸紝瀹冩寚鍚戣嚜韬€?
### 鍦ㄩ亶鍘嗙殑鍚屾椂绉婚櫎鑺傜偣


濡傛灉鎴戜滑浣跨敤 list_for_each() 鍜?list_for_each_entry()锛屽湪閬嶅巻閾捐〃鐨勫悓鏃跺垹闄ゆ潯鐩細
寮曞彂闂锛屽洜涓哄垹闄ゅ綋鍓嶆潯鐩細淇敼瀹冪殑 `next` 鎸囬拡锛岃繖鎰忓懗鐫€閬嶅巻鏃犳硶姝ｇ‘鍦板墠杩涘埌
涓嬩竴涓摼琛ㄦ潯鐩€?
涓嶈繃鏈変竴涓В鍐虫柟妗堬細list_for_each_safe() 鍜?list_for_each_entry_safe()銆傚畠浠澶?鎺ュ彈涓€涓寚鍚?struct list_head 鐨勬寚閽堜綔涓哄弬鏁帮紝鐢ㄤ綔閬嶅巻鏈熼棿涓嬩竴涓潯鐩殑涓存椂瀛樺偍锛屼粠鑰?瑙ｅ喅璇ラ棶棰樸€?
濡備綍浣跨敤瀹冪殑绀轰緥锛?

  static void circus_eject_insufficient_clowns(struct circus_priv *circus)
  {
          struct clown *e;
          struct clown **n;      /** temporary storage for safe iteration */

          list_for_each_entry_safe(e, n, &circus->car.clowns, node) {
                if (e->shoe_size < 500)
                        list_del(&e->node);
          }
  }

杩欑鎯呭喌涓嬶紝鎭板綋鐨勫唴瀛樼鐞嗭紙鍗抽噴鏀捐鍒犻櫎鐨勮妭鐐癸紝鍚屾椂纭繚娌℃湁浠讳綍涓滆タ浠嶅湪寮曠敤瀹冿級鐣欎綔
缁冧範缁欒鑰呫€?
### 鍒囧壊閾捐〃


鏈変袱涓緟鍔╁嚱鏁板彲鐢ㄤ簬鍒囧壊閾捐〃銆備簩鑰呴兘浠庨摼琛?`head` 涓彇鍑哄厓绱狅紝骞舵浛鎹㈤摼琛?`list`
鐨勫唴瀹广€?
绗竴涓繖鏍风殑鍑芥暟鏄?list_cut_position()銆傚畠灏?`head` 涓洿鍒板苟鍖呮嫭 `entry` 鐨勬墍鏈夐摼琛?鏉＄洰绉婚櫎锛岃浆鑰屼粠 `list` 涓斁缃畠浠€?
```

         .----------------------------------------------------------------.
         v                                                                |
    .--------.     .-------.     .---------.     .-----.     .---------.  |
    | clowns |---->| Grock |---->| Dimitri |---->| Pic |---->| Alfredo |--'
    '--------'     '-------'     '---------'     '-----'     '---------'

```
閫氳繃涓嬮潰鐨勪唬鐮侊紝浠?"clowns" 鐩村埌骞跺寘鎷?"Pic" 鐨勬瘡涓皬涓戦兘琚粠 "clowns" 閾捐〃澶寸Щ鍔ㄥ埌涓€涓?鍗曠嫭鐨?struct list_head锛岃缁撴瀯鍦ㄥ眬閮ㄦ爤鍙橀噺 `retirement` 澶勫垵濮嬪寲锛?

  static void circus_retire_clowns(struct circus_priv *circus)
  {
          struct list_head retirement = LIST_HEAD_INIT(retirement);
          struct clown **grock, **dimitri, **pic, **alfredo;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          list_cut_position(&retirement, &car->clowns, &pic->node);

          /** State 1 **/
  }

```

         .----------------------.
         v                      |
    .--------.     .---------.  |
    | clowns |---->| Alfredo |--'
    '--------'     '---------'

```
```

           .--------------------------------------------------.
           v                                                  |
    .------------.     .-------.     .---------.     .-----.  |
    | retirement |---->| Grock |---->| Dimitri |---->| Pic |--'
    '------------'     '-------'     '---------'     '-----'

```
绗簩涓嚱鏁?list_cut_before() 鍩烘湰涓€鏍凤紝鍙槸瀹冨湪 `entry` 鑺傜偣涔嬪墠鍒囧壊锛屽嵆瀹冪Щ闄?`head`
涓洿鍒颁絾涓嶅寘鍚?`entry` 鐨勬墍鏈夐摼琛ㄦ潯鐩紝杞€屼粠 `list` 涓斁缃畠浠€傛渚嬪亣瀹氫笌鍓嶉潰渚嬪瓙
鐩稿悓鐨勫垵濮嬭捣濮嬮摼琛細


  static void circus_retire_clowns(struct circus_priv *circus)
  {
          struct list_head retirement = LIST_HEAD_INIT(retirement);
          struct clown **grock, **dimitri, **pic, **alfredo;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          list_cut_before(&retirement, &car->clowns, &pic->node);

          /** State 1b **/
  }

```

         .----------------------------------.
         v                                  |
    .--------.     .-----.     .---------.  |
    | clowns |---->| Pic |---->| Alfredo |--'
    '--------'     '-----'     '---------'

```
```

           .--------------------------------------.
           v                                      |
    .------------.     .-------.     .---------.  |
    | retirement |---->| Grock |---->| Dimitri |--'
    '------------'     '-------'     '---------'

```
搴斿綋娉ㄦ剰锛岃繖涓や釜鍑芥暟閮戒細閿€姣佸埌鐩爣 `struct list_head *list` 涓换浣曠幇瀛樿妭鐐圭殑閾炬帴銆?
### 绉诲姩鏉＄洰涓庨儴鍒嗛摼琛?

list_move() 鍜?list_move_tail() 鍑芥暟鍙敤浜庡皢涓€涓潯鐩粠涓€涓摼琛ㄧЩ鍔ㄥ埌鍙︿竴涓摼琛紝
鍒嗗埆绉诲姩鍒拌捣濮嬫垨鏈熬銆?
鍦ㄤ笅闈㈢殑渚嬪瓙涓紝鎴戜滑鍋囪浠庝袱涓摼琛ㄥ紑濮嬶紙鈥渃lowns鈥?涓?```

         .----------------------------------------------------------------.
         v                                                                |
    .--------.     .-------.     .---------.     .-----.     .---------.  |
    | clowns |---->| Grock |---->| Dimitri |---->| Pic |---->| Alfredo |--'
    '--------'     '-------'     '---------'     '-----'     '---------'

          .-------------------.
          v                   |
    .----------.     .-----.  |
    | sidewalk |---->| Pio |--'
    '----------'     '-----'

```
鎴戜滑灏嗕笅闈㈢殑绀轰緥浠ｇ爜搴旂敤浜庤繖涓や釜閾捐〃锛?

  static void circus_clowns_exit_car(struct circus_priv *circus)
  {
          struct list_head sidewalk = LIST_HEAD_INIT(sidewalk);
          struct clown **grock, **dimitri, **pic, **alfredo, *pio;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          /** State 0 **/

          list_move(&pic->node, &sidewalk);

          /** State 1 **/

          list_move_tail(&dimitri->node, &sidewalk);

          /** State 2 **/
  }

```

        .-----------------------------------------------------.
        |                                                     |
        v                                                     |
    .--------.     .-------.     .---------.     .---------.  |
    | clowns |---->| Grock |---->| Dimitri |---->| Alfredo |--'
    '--------'     '-------'     '---------'     '---------'

          .-------------------------------.
          v                               |
    .----------.     .-----.     .-----.  |
    | sidewalk |---->| Pic |---->| Pio |--'
    '----------'     '-----'     '-----'

```
鍦?State 2 涓紝鍦ㄦ垜浠皢 Dimitri 绉诲姩鍒?sidewalk 鐨勬湯灏句箣鍚庯紝鎯呭舰鍙樹负
```

        .-------------------------------------.
        |                                     |
        v                                     |
    .--------.     .-------.     .---------.  |
    | clowns |---->| Grock |---->| Alfredo |--'
    '--------'     '-------'     '---------'

          .-----------------------------------------------.
          v                                               |
    .----------.     .-----.     .-----.     .---------.  |
    | sidewalk |---->| Pic |---->| Pio |---->| Dimitri |--'
    '----------'     '-----'     '-----'     '---------'

```
鍙婧愰摼琛ㄥご涓庣洰鏍囬摼琛ㄥご灞炰簬鍚屼竴涓摼琛紝鎴戜滑杩樺彲浠ラ珮鏁堝湴灏嗛摼琛ㄧ殑涓€娈垫壒閲忕Щ鍔ㄥ埌閾捐〃鐨?灏剧銆傛垜浠湪鍓嶄竴涓緥瀛愮殑鍩虹涓婏紝鍦?State 2 涔嬪悗娣诲姞涓€娆?list_bulk_move_tail()锛屽皢 Pic
鍜?Pio 绉诲姩鍒?sidewalk 閾捐〃鐨勫熬绔€?

  static void circus_clowns_exit_car(struct circus_priv *circus)
  {
          struct list_head sidewalk = LIST_HEAD_INIT(sidewalk);
          struct clown **grock, **dimitri, **pic, **alfredo, *pio;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          /** State 0 **/

          list_move(&pic->node, &sidewalk);

          /** State 1 **/

          list_move_tail(&dimitri->node, &sidewalk);

          /** State 2 **/

          list_bulk_move_tail(&sidewalk, &pic->node, &pio->node);

          /** State 3 **/
  }

涓虹畝娲佽捣瑙侊紝鍙弿缁樹簡 State 3 涓彂鐢熷彉鍖栫殑 "sidewalk" 閾捐〃
```

          .-----------------------------------------------.
          v                                               |
    .----------.     .---------.     .-----.     .-----.  |
    | sidewalk |---->| Dimitri |---->| Pic |---->| Pio |--'
    '----------'     '---------'     '-----'     '-----'

```
璇锋敞鎰忥紝list_bulk_move_tail() 涓嶄細妫€鏌ユ墍缁欏嚭鐨勪笁涓?`struct list_head *` 鍙傛暟鏄惁鐪熺殑
閮藉睘浜庡悓涓€涓摼琛ㄣ€傚鏋滀綘鍦ㄦ枃妗ｇ粰鍑虹殑绾︽潫涔嬪浣跨敤瀹冿紝閭ｄ箞缁撴灉灏辨槸浣犱笌瀹炵幇涔嬮棿鐨勪簨鎯呬簡銆?
### 鏃嬭浆鏉＄洰


閾捐〃涓婁竴绉嶅父瑙佺殑鍐欐搷浣滐紝灏ゅ叾鏄湪灏嗗叾鐢ㄤ綔闃熷垪鏃讹紝鏄皢鍏舵棆杞紙rotate锛夈€傞摼琛ㄦ棆杞剰鍛崇潃
鍓嶇鐨勬潯鐩閫佸埌鍚庣銆?
瀵逛簬鏃嬭浆锛孡inux 涓烘垜浠彁渚涗簡涓や釜鍑芥暟锛歭ist_rotate_left() 鍜?list_rotate_to_front()銆?鍓嶈€呭彲浠ヨ鎯宠薄鎴愯嚜琛岃溅閾炬潯锛屽彇鎵€缁欏嚭鐨?`struct list_head *` 涔嬪悗鐨勯偅涓潯鐩苟灏嗗叾绉诲姩鍒?灏鹃儴锛岀敱浜庨摼琛ㄧ殑鐜舰鏈川锛岃繖瀹炶川涓婃剰鍛崇潃鏁翠釜閾捐〃鏃嬭浆浜嗕竴涓綅缃€?
鍚庤€?list_rotate_to_front() 灏嗗悓涓€姒傚康鎺ㄨ繘涓€姝ワ細瀹冧笉鏄閾捐〃鍓嶈繘涓€涓潯鐩紝鑰屾槸鍓嶈繘
**鐩村埌**鎸囧畾鐨勬潯鐩垚涓烘柊鐨勫墠绔€?
```

         .-----------------------------------------------------------------.
         v                                                                 |
    .--------.   .-------.   .---------.   .-----.   .---------.   .-----. |
    | clowns |-->| Grock |-->| Dimitri |-->| Pic |-->| Alfredo |-->| Pio |-'
    '--------'   '-------'   '---------'   '-----'   '---------'   '-----'

```
鐢ㄤ簬婕旂ず閾捐〃鏃嬭浆鐨勭ず渚嬩唬鐮佸涓嬶細


  static void circus_clowns_rotate(struct circus_priv *circus)
  {
          struct clown **grock, **dimitri, **pic, **alfredo, *pio;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          /** State 0 **/

          list_rotate_left(&car->clowns);

          /** State 1 **/

          list_rotate_to_front(&alfredo->node, &car->clowns);

          /** State 2 **/

  }

```

         .-----------------------------------------------------------------.
         v                                                                 |
    .--------.   .---------.   .-----.   .---------.   .-----.   .-------. |
    | clowns |-->| Dimitri |-->| Pic |-->| Alfredo |-->| Pio |-->| Grock |-'
    '--------'   '---------'   '-----'   '---------'   '-----'   '-------'

```
鎺ヤ笅鏉ワ紝鍦?list_rotate_to_front() 璋冪敤涔嬪悗锛屾垜浠埌杈惧涓嬬姸鎬?```

         .-----------------------------------------------------------------.
         v                                                                 |
    .--------.   .---------.   .-----.   .-------.   .---------.   .-----. |
    | clowns |-->| Alfredo |-->| Pio |-->| Grock |-->| Dimitri |-->| Pic |-'
    '--------'   '---------'   '-----'   '-------'   '---------'   '-----'

```
甯屾湜浠庡浘涓彲浠ユ槑鏄剧湅鍑猴紝"Alfredo" 涔嬪墠鐨勬潯鐩寰幆鍒颁簡閾捐〃鐨勫熬绔€?
### 浜ゆ崲鏉＄洰


鍙︿竴涓父瑙佹搷浣滄槸涓や釜鏉＄洰闇€瑕佸郊姝や氦鎹€?
涓烘锛孡inux 涓烘垜浠彁渚涗簡 list_swap()銆?
鍦ㄤ笅闈㈢殑渚嬪瓙涓紝鎴戜滑鏈変竴涓寘鍚笁涓潯鐩殑閾捐〃锛屽苟浜ゆ崲鍏朵腑涓や釜
```

         .-----------------------------------------.
         v                                         |
    .--------.   .-------.   .---------.   .-----. |
    | clowns |-->| Grock |-->| Dimitri |-->| Pic |-'
    '--------'   '-------'   '---------'   '-----'

```


  static void circus_clowns_swap(struct circus_priv *circus)
  {
          struct clown **grock, **dimitri, *pic;
          struct clown_car *car = &circus->car;

          /** ... clown initialization, list adding ... **/

          /** State 0 **/

          list_swap(&dimitri->node, &pic->node);

          /** State 1 **/
  }

```

         .-----------------------------------------.
         v                                         |
    .--------.   .-------.   .-----.   .---------. |
    | clowns |-->| Grock |-->| Pic |-->| Dimitri |-'
    '--------'   '-------'   '-----'   '---------'

```
瀵规瘮涓ゅ浘鍙互鏄庢樉鐪嬪嚭锛?Pic" 鍜?"Dimitri" 鑺傜偣浜ゆ崲浜嗕綅缃€?
### 灏嗕袱涓摼琛ㄦ嫾鎺ュ湪涓€璧?

鍋囪鎴戜滑鏈変袱涓摼琛紝鍦ㄤ笅闈㈢殑渚嬪瓙涓紝涓€涓敱鎴戜滑绉颁负 "knie" 鐨勯摼琛ㄥご琛ㄧず锛屽彟涓€涓敱鎴戜滑
绉颁负 "stey" 鐨勯摼琛ㄥご琛ㄧず銆傚湪涓€娆″亣鎯崇殑椹垙鍥㈡敹璐腑锛岃繖涓ょ粍灏忎笐搴斿綋琚嫾鎺ュ湪涓€璧枫€傛垜浠?鐨?```

        .-----------------------------------------.
        |                                         |
        v                                         |
    .------.   .-------.   .---------.   .-----.  |
    | knie |-->| Grock |-->| Dimitri |-->| Pic |--'
    '------'   '-------'   '---------'   '-----'

        .-----------------------------.
        v                             |
    .------.   .---------.   .-----.  |
    | stey |-->| Alfredo |-->| Pio |--'
    '------'   '---------'   '-----'

```
灏嗚繖涓や釜閾捐〃鎷兼帴鍦ㄤ竴璧风殑鍑芥暟鏄?list_splice()銆傛垜浠殑绀轰緥浠ｇ爜濡備笅锛?

  static void circus_clowns_splice(void)
  {
          struct clown **grock, **dimitri, **pic, **alfredo, *pio;
          struct list_head knie = LIST_HEAD_INIT(knie);
          struct list_head stey = LIST_HEAD_INIT(stey);

          /** ... Clown allocation and initialization here ... **/

          list_add_tail(&grock->node, &knie);
          list_add_tail(&dimitri->node, &knie);
          list_add_tail(&pic->node, &knie);
          list_add_tail(&alfredo->node, &stey);
          list_add_tail(&pio->node, &stey);

          /** State 0 **/

          list_splice(&stey, &dimitri->node);

          /** State 1 **/
  }

杩欓噷鐨?list_splice() 璋冪敤灏?`stey` 涓殑鎵€鏈夋潯鐩坊鍔犲埌 `dimitri` 鐨?`node` 閾捐〃澶存墍鍦?鐨勯摼琛ㄤ腑锛屼綅浜?`dimitri` 鐨?`node` 涔嬪悗銆備竴涓?```

        .-----------------------------------------------------------------.
        |                                                                 |
        v                                                                 |
    .------.   .-------.   .---------.   .---------.   .-----.   .-----.  |
    | knie |-->| Grock |-->| Dimitri |-->| Alfredo |-->| Pio |-->| Pic |--'
    '------'   '-------'   '---------'   '---------'   '-----'   '-----'
                                              ^
              .-------------------------------'
              |
    .------.  |
    | stey |--'
    '------'

```
閬嶅巻 `stey` 閾捐〃涓嶅啀浜х敓姝ｇ‘鐨勮涓恒€傚湪 `stey` 涓婅皟鐢?list_for_each() 浼氬鑷存棤闄愬惊鐜紝
鍥犱负瀹冩案杩滀笉浼氬洖鍒?`stey` 閾捐〃澶淬€?
杩欐槸鍥犱负 list_splice() 娌℃湁閲嶆柊鍒濆鍖栧畠鍙栬蛋鏉＄洰鐨勯偅涓摼琛ㄥご锛屼娇鍏舵寚閽堟寚鍚戜簡鐜板湪宸叉槸
涓嶅悓鐨勯摼琛ㄧ殑浣嶇疆銆?
濡傛灉鎴戜滑鎯抽伩鍏嶈繖绉嶆儏鍐碉紝鍙互浣跨敤 list_splice_init()銆傚畠鍋氫笌 list_splice() 鐩稿悓鐨勪簨锛?鍙槸浼氬湪绉绘涔嬪悗閲嶆柊鍒濆鍖栭偅涓緵浣撻摼琛ㄥご銆?
### 骞跺彂鑰冮噺


鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝瀵归摼琛ㄧ殑骞跺彂璁块棶涓庝慨鏀归渶瑕佺敤閿佹潵淇濇姢銆傚彟涓€绉嶏紙涔熸槸鏇村彲鍙栫殑锛夋柟寮忔槸锛?鍦ㄢ€滆澶氬啓灏戔€濈殑浣跨敤鍦烘櫙涓嬶紝瀵归摼琛ㄤ娇鐢?RCU 鍘熻锛屽嵆璇诲彇閾捐〃寰堝父瑙併€佷絾淇敼閾捐〃杈冨皯鐨勬儏褰€?鏇村缁嗚妭鍙傝 Documentation/RCU/listRCU.rst銆?
### 寤朵几闃呰


- `How does the kernel implements Linked Lists? - KernelNewbies <https://kernelnewbies.org/FAQ/LinkedLists>`_

## 瀹屾暣閾捐〃 API


   :internal:

## 绉佹湁閾捐〃 API


   :doc: Private List Primitives

   :internal:
