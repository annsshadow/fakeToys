## RCU 闇€姹備箣鏃?


Copyright IBM Corporation, 2015

Author: Paul E. McKenney

The initial version of this document appeared in the
`LWN <https://lwn.net/>`_ on those articles:
`part 1 <https://lwn.net/Articles/652156/>`_,
`part 2 <https://lwn.net/Articles/652677/>`_, and
`part 3 <https://lwn.net/Articles/653326/>`_.

### 寮曡█


璇?澶嶅埗-鏇存柊锛圧CU锛夋槸涓€绉嶅悓姝ユ満鍒讹紝甯歌鐢ㄤ綔璇诲啓閿佺殑鏇夸唬鏂规銆俁CU 鐨勪笉鍚屽甯镐箣澶勫湪浜庢洿鏂拌€呬笉浼氶樆濉炶鑰咃紝杩欐剰鍛崇潃 RCU 鐨勮绔師璇彲浠ラ潪甯稿揩涓斿叿澶囧彲鎵╁睍鎬с€傛澶栵紝鏇存柊鑰呭彲浠ヤ笌璇昏€呭苟鍙戝湴鍙栧緱鏈夌敤鐨勫墠鍚戣繘灞曘€傜劧鑰岋紝RCU 璇昏€呬笌鏇存柊鑰呬箣闂寸殑鎵€鏈夎繖浜涘苟鍙戠‘瀹炲紩鍑轰簡涓€涓枒闂細RCU 璇昏€呯┒绔熷湪鍋氫粈涔堬紝杩涜€屽張寮曞嚭浜?RCU 鐨勯渶姹傜┒绔熸槸浠€涔堢殑鐤戦棶銆?

鍥犳锛屾湰鏂囨。鎬荤粨浜?RCU 鐨勯渶姹傦紝鍙瑙嗕负 RCU 鐨勪竴浠介潪姝ｅ紡銆侀珮灞傛鐨勮鑼冦€傞噸瑕佺殑鏄鐞嗚В锛孯CU 鐨勮鑼冩湰璐ㄤ笂鏄粡楠屾€х殑锛涗簨瀹炰笂锛屾垜鏄悆浜嗕笉灏戣嫤澶存墠浜嗚В鍒板叾涓澶氶渶姹傜殑銆傝繖绉嶆儏鍐垫垨璁镐細璁╀汉鏈変簺涓嶅畨锛屼笉杩囷紝杩欎竴瀛︿範杩囩▼涓嶄粎鍏呮弧涔愯叮锛岃€屼笖鑳戒笌浼楀鎰挎剰浠ユ湁瓒ｇ殑鏂版柟寮忓簲鐢ㄦ妧鏈殑浜哄叡浜嬶紝涔熸槸鏋佸ぇ鐨勮崳骞搞€?

鎶涘紑杩欎簺涓嶈皥锛屼互涓嬫槸褰撳墠宸茬煡鐨?RCU 闇€姹傜被鍒細

#. `Fundamental Requirements`_
#. `Fundamental Non-Requirements`_
#. `Parallelism Facts of Life`_
#. `Quality-of-Implementation Requirements`_
#. `Linux Kernel Complications`_
#. `Software-Engineering Requirements`_
#. `Other RCU Flavors`_
#. `Possible Future Changes`_

鍏跺悗鏄竴涓?summary_锛屼笉杩囷紝姣忎釜蹇€熸祴楠岀殑绛旀绱ф帴鍦ㄦ祴楠屼箣鍚庛€傜敤榧犳爣閫変腑澶х墖绌虹櫧鍖哄煙鍗冲彲鐪嬪埌绛旀銆?

### 鍩烘湰瑕佹眰


RCU 鐨勫熀鏈姹傛槸 RCU 鏈€鎺ヨ繎纭€ф暟瀛﹂渶姹傜殑涓滆タ銆傚畠浠槸锛?

#. `Grace-Period Guarantee`_
#. `Publish/Subscribe Guarantee`_
#. `Memory-Barrier Guarantees`_
#. `RCU Primitives Guaranteed to Execute Unconditionally`_
#. `Guaranteed Read-to-Write Upgrade`_

#### 瀹介檺鏈熶繚璇?


RCU 鐨勫闄愭湡淇濊瘉涔嬫墍浠ヤ笉鍚屽甯革紝鍦ㄤ簬瀹冩槸棰勫厛璁炬兂濂界殑锛欽ack Slingwine 鍜屾垜鍦?1990 骞翠唬鍒濆紑濮嬬爺绌?RCU锛堝綋鏃剁О涓衡€渞clock鈥濓級鏃讹紝灏辩墷鐗㈣鐫€杩欎竴淇濊瘉銆傝瘽铏藉姝わ紝杩囧幓浜屽崄骞翠娇鐢?RCU 鐨勭粡楠岃鎴戜滑瀵硅繖涓€淇濊瘉鏈変簡鏇翠负缁嗚嚧鐨勭悊瑙ｃ€?

RCU 鐨勫闄愭湡淇濊瘉鍏佽鏇存柊鑰呯瓑寰呮墍鏈夋棦瀛樼殑 RCU 璇荤涓寸晫鍖虹殑瀹屾垚銆備竴涓?RCU 璇荤涓寸晫鍖轰互鏍囪 rcu_read_lock() 寮€濮嬶紝浠ユ爣璁?rcu_read_unlock() 缁撴潫銆傝繖浜涙爣璁板彲浠ュ祵濂楋紝RCU 灏嗕竴缁勫祵濂楃殑鏍囪瑙嗕负涓€涓ぇ鐨?RCU 璇荤涓寸晫鍖恒€傜敓浜ц川閲忕殑 rcu_read_lock() 鍜?rcu_read_unlock() 瀹炵幇鏋佸叾杞婚噺锛屼簨瀹炰笂鍦ㄤ娇鐢?`CONFIG_PREEMPTION=n` 鏋勫缓鐨勭敤浜庣敓浜х敤閫旂殑 Linux 鍐呮牳涓紑閿€涓洪浂銆?

杩欎竴淇濊瘉浣垮緱鑳藉浠ユ瀬浣庣殑寮€閿€瀵硅鑰呭疄鏂介『搴忕害鏉燂紝渚嬪锛?

```

       1 int x, y;
       2
       3 void thread0(void)
       4 {
       5   rcu_read_lock();
       6   r1 = READ_ONCE(x);
       7   r2 = READ_ONCE(y);
       8   rcu_read_unlock();
       9 }
      10
      11 void thread1(void)
      12 {
      13   WRITE_ONCE(x, 1);
      14   synchronize_rcu();
      15   WRITE_ONCE(y, 1);
      16 }
      17
```

鐢变簬绗?14 琛岀殑 synchronize_rcu() 浼氱瓑寰呮墍鏈夋棦瀛樿鑰咃紝浠讳綍浠?`x` 鍔犺浇鍒伴浂鍊肩殑 thread0() 瀹炰緥閮藉繀椤诲湪 thread1() 鍚?`y` 瀛樺偍涔嬪墠瀹屾垚锛屽洜姝よ瀹炰緥涔熷繀椤讳粠 `y` 鍔犺浇鍒伴浂鍊笺€傜被浼煎湴锛屼换浣曚粠 `y` 鍔犺浇鍒颁竴鍊肩殑 thread0() 瀹炰緥蹇呭畾鏄湪 synchronize_rcu() 寮€濮嬩箣鍚庢墠鍚姩鐨勶紝鍥犳涔熷繀瀹氫細浠?`x` 鍔犺浇鍒颁竴鍊笺€傚洜姝わ紝濡備笅缁撴灉锛?

```

     (r1 == 0 && r2 == 1)

```

涓嶅彲鑳藉彂鐢熴€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Wait a minute! You said that updaters can make useful forward         |
| progress concurrently with readers, but pre-existing readers will     |
| block synchronize_rcu()!!!                                            |
| Just who are you trying to fool???                                    |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| First, if updaters do not wish to be blocked by readers, they can use |
| call_rcu() or kfree_rcu(), which will be discussed later.             |
| Second, even when using synchronize_rcu(), the other update-side      |
| code does run concurrently with readers, whether pre-existing or not. |
+-----------------------------------------------------------------------+

杩欎竴鍦烘櫙绫讳技浜?RCU 鍦?`DYNIX/ptx <https://en.wikipedia.org/wiki/DYNIX>`__ 涓殑鏈€鏃╃敤閫斾箣涓€锛屽畠绠＄悊涓€涓垎甯冨紡閿佺鐞嗗櫒鍚戦€傚悎澶勭悊鑺傜偣鏁呴殰鎭㈠鐨勭姸鎬佽繃娓★紝澶ц嚧濡備笅锛?

```

       1 #define STATE_NORMAL        0
       2 #define STATE_WANT_RECOVERY 1
       3 #define STATE_RECOVERING    2
       4 #define STATE_WANT_NORMAL   3
       5
       6 int state = STATE_NORMAL;
       7
       8 void do_something_dlm(void)
       9 {
      10   int state_snap;
      11
      12   rcu_read_lock();
      13   state_snap = READ_ONCE(state);
      14   if (state_snap == STATE_NORMAL)
      15     do_something();
      16   else
      17     do_something_carefully();
      18   rcu_read_unlock();
      19 }
      20
      21 void start_recovery(void)
      22 {
      23   WRITE_ONCE(state, STATE_WANT_RECOVERY);
      24   synchronize_rcu();
      25   WRITE_ONCE(state, STATE_RECOVERING);
      26   recovery();
      27   WRITE_ONCE(state, STATE_WANT_NORMAL);
      28   synchronize_rcu();
      29   WRITE_ONCE(state, STATE_NORMAL);
      30 }
      31
```

do_something_dlm() 涓殑 RCU 璇荤涓寸晫鍖轰笌 start_recovery() 涓殑 synchronize_rcu() 閰嶅悎锛屼繚璇?do_something() 姘歌繙涓嶄細涓?recovery() 骞跺彂杩愯锛岃€屽湪 do_something_dlm() 涓嚑涔庢垨瀹屽叏娌℃湁鍚屾寮€閿€銆?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Why is the synchronize_rcu() on line 28 needed?                       |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Without that extra grace period, memory reordering could result in    |
| do_something_dlm() executing do_something() concurrently with         |
| the last bits of recovery().                                          |
+-----------------------------------------------------------------------+

涓轰簡閬垮厤姝婚攣绛夎嚧鍛介棶棰橈紝RCU 璇荤涓寸晫鍖轰笉鑳藉寘鍚 synchronize_rcu() 鐨勮皟鐢ㄣ€傜被浼煎湴锛孯CU 璇荤涓寸晫鍖轰笉鑳藉寘鍚换浣曠洿鎺ユ垨鑰呴棿鎺ョ瓑寰呮煇涓?synchronize_rcu() 璋冪敤瀹屾垚鐨勫唴瀹广€?

灏界 RCU 鐨勫闄愭湡淇濊瘉鏈韩寰堟湁鐢紙鏈?`quite a few use cases <https://lwn.net/Articles/573497/>`__锛夛紝浣嗚嫢鏄兘鐢?RCU 鏉ュ崗璋冨閾捐〃鏁版嵁缁撴瀯鐨勮绔闂氨濂戒簡銆傚姝わ紝瀹介檺鏈熶繚璇佸苟涓嶅厖鍒嗭紝濡備笅闈㈢殑 add_gp_buggy() 鍑芥暟鎵€绀恒€傛垜浠◢鍚庝細鐪嬪埌璇昏€呯殑浠ｇ爜锛屼絾鍦ㄦ鏈熼棿锛屽彧闇€鎶婅鑰呯湅浣滄棤閿佸湴鍙栬蛋 `gp` 鎸囬拡锛屽苟涓斿鏋滃姞杞藉埌鐨勫€间笉鏄?`NULL`锛屽氨鏃犻攣鍦拌闂?`->a` 鍜?`->b` 瀛楁銆?

```

       1 bool add_gp_buggy(int a, int b)
       2 {
       3   p = kmalloc(sizeof(*p), GFP_KERNEL);
       4   if (!p)
       5     return -ENOMEM;
       6   spin_lock(&gp_lock);
       7   if (rcu_access_pointer(gp)) {
       8     spin_unlock(&gp_lock);
       9     return false;
      10   }
      11   p->a = a;
      12   p->b = a;
      13   gp = p; /* ORDERING BUG */
      14   spin_unlock(&gp_lock);
      15   return true;
      16 }
      17
```

闂鍦ㄤ簬锛岀紪璇戝櫒鍜屽急搴?CPU 閮芥湁鏉冨皢杩欐浠ｇ爜閲嶆帓濡備笅锛?

```

       1 bool add_gp_buggy_optimized(int a, int b)
       2 {
       3   p = kmalloc(sizeof(*p), GFP_KERNEL);
       4   if (!p)
       5     return -ENOMEM;
       6   spin_lock(&gp_lock);
       7   if (rcu_access_pointer(gp)) {
       8     spin_unlock(&gp_lock);
       9     return false;
      10   }
      11   gp = p; /* ORDERING BUG */
      12   p->a = a;
      13   p->b = a;
      14   spin_unlock(&gp_lock);
      15   return true;
      16 }
      17
```

濡傛灉鏌愪釜 RCU 璇昏€呭湪 `add_gp_buggy_optimized` 鎵ц绗?11 琛屽悗绔嬪埢鍙栬蛋 `gp`锛屽畠浼氱湅鍒?`->a` 鍜?`->b` 瀛楁涓殑鍨冨溇鏁版嵁銆傝€岃繖鍙槸缂栬瘧鍣ㄥ拰纭欢浼樺寲鍙兘閫犳垚楹荤儲鐨勪紬澶氭柟寮忎箣涓€銆傚洜姝わ紝鎴戜滑鏄剧劧闇€瑕佹煇绉嶆柟寮忔潵闃绘缂栬瘧鍣ㄥ拰 CPU 浠ヨ繖绉嶆柟寮忛噸鎺掞紝杩欏氨寮曞嚭浜嗕笅涓€鑺傝璁虹殑鍙戝竷-璁㈤槄淇濊瘉銆?

#### 鍙戝竷/璁㈤槄淇濊瘉


RCU 鐨勫彂甯?璁㈤槄淇濊瘉鍏佽鍦ㄤ笉鎵撴壈 RCU 璇昏€呯殑鎯呭喌涓嬶紝灏嗘暟鎹彃鍏ュ埌閾捐〃鏁版嵁缁撴瀯涓€傛洿鏂拌€呬娇鐢?rcu_assign_pointer() 鎻掑叆鏂版暟鎹紝璇昏€呬娇鐢?rcu_dereference() 璁块棶鏁版嵁锛堟棤璁烘槸鏂扮殑杩樻槸鏃х殑锛夈€備笅闈㈢粰鍑轰竴涓彃鍏ョず渚嬶細

```

       1 bool add_gp(int a, int b)
       2 {
       3   p = kmalloc(sizeof(*p), GFP_KERNEL);
       4   if (!p)
       5     return -ENOMEM;
       6   spin_lock(&gp_lock);
       7   if (rcu_access_pointer(gp)) {
       8     spin_unlock(&gp_lock);
       9     return false;
      10   }
      11   p->a = a;
      12   p->b = a;
      13   rcu_assign_pointer(gp, p);
      14   spin_unlock(&gp_lock);
      15   return true;
      16 }
      17
```

绗?13 琛岀殑 rcu_assign_pointer() 鍦ㄦ蹇典笂绛変环浜庝竴鏉＄畝鍗曠殑璧嬪€艰鍙ワ紝浣嗗悓鏃朵篃淇濊瘉鍏惰祴鍊间細鍙戠敓鍦ㄧ 11 琛屽拰绗?12 琛屼袱娆¤祴鍊间箣鍚庯紝绫讳技浜?C11 `memory_order_release` 瀛樺偍鎿嶄綔銆傚畠杩樿兘闃绘浠讳綍鏁伴噺鐨勨€滄湁瓒ｂ€濈殑缂栬瘧鍣ㄤ紭鍖栵紝渚嬪锛屽湪璧嬪€间箣鍓嶆妸 `gp` 鐢ㄤ綔涓存椂瀛樻斁浣嶇疆銆?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| But rcu_assign_pointer() does nothing to prevent the two              |
| assignments to `p->a` and `p->b` from being reordered. Can't that |    
| also cause problems?                                                  |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| No, it cannot. The readers cannot see either of these two fields      |
| until the assignment to `gp`, by which time both fields are fully   |  
| initialized. So reordering the assignments to `p->a` and `p->b`   |    
| cannot possibly cause any problems.                                   |
+-----------------------------------------------------------------------+

浜轰滑寰堝鏄撴兂褰撶劧鍦拌涓猴紝璇昏€呮棤闇€鍋氫换浣曠壒娈婂鐞嗘潵鎺у埗鍏跺 RCU 淇濇姢鏁版嵁鐨勮闂紝濡備笅闈㈢殑 do_something_gp_buggy() 鎵€绀猴細

```

       1 bool do_something_gp_buggy(void)
       2 {
       3   rcu_read_lock();
       4   p = gp;  /* OPTIMIZATIONS GALORE!!! */
       5   if (p) {
       6     do_something(p->a, p->b);
       7     rcu_read_unlock();
       8     return true;
       9   }
      10   rcu_read_unlock();
      11   return false;
      12 }
      13
```

鐒惰€岋紝蹇呴』鎶靛埗杩欑璇辨儜锛屽洜涓虹紪璇戝櫒锛堟垨鍍?DEC Alpha 杩欐牱鐨勫急搴?CPU锛夋湁鏁伴噺鎯婁汉涔嬪鐨勬柟寮忎細璁╄繖娈典唬鐮佸嚭閿欍€備粎涓句竴渚嬶細濡傛灉缂栬瘧鍣ㄥ瘎瀛樺櫒鐭己锛屽畠鍙兘浼氶€夋嫨浠?`gp` 閲嶆柊鑾峰彇锛岃€屼笉鏄湪 `p` 涓繚鐣欏崟鐙殑涓€浠藉壇鏈紝濡備笅鎵€绀猴細

```

       1 bool do_something_gp_buggy_optimized(void)
       2 {
       3   rcu_read_lock();
       4   if (gp) { /* OPTIMIZATIONS GALORE!!! */
       5     do_something(gp->a, gp->b);
       6     rcu_read_unlock();
       7     return true;
       8   }
       9   rcu_read_unlock();
      10   return false;
      11 }
      12
```
濡傛灉杩欎釜鍑芥暟涓庝竴绯诲垪灏嗗綋鍓嶇粨鏋勬浛鎹负鏂扮粨鏋勭殑鏇存柊骞跺彂杩愯锛屽 `gp->a` 鍜?
`gp->b` 鐨勫彇鐢ㄥ緢鍙兘鏉ヨ嚜涓や釜涓嶅悓鐨勭粨鏋勶紝杩欎細閫犳垚涓ラ噸鐨勬贩涔便€備负闃叉杩欑鎯呭喌锛堜互鍙婅澶氬叾浠栨儏鍐碉級锛?
do_something_gp() 浣跨敤 rcu_dereference() 浠?`gp` 鍙栫敤锛?

```

       1 bool do_something_gp(void)
       2 {
       3   rcu_read_lock();
       4   p = rcu_dereference(gp);
       5   if (p) {
       6     do_something(p->a, p->b);
       7     rcu_read_unlock();
       8     return true;
       9   }
      10   rcu_read_unlock();
      11   return false;
      12 }
      13
```

rcu_dereference() 鍦?Linux 鍐呮牳涓娇鐢?volatile 绫诲瀷杞崲浠ュ強锛堝浜?DEC Alpha锛夊唴瀛樺睆闅溿€傚€樿嫢灏嗘潵鍑虹幇浜?|high-quality implementation of C11 memory_order_consume [PDF]|_锛岄偅涔?rcu_dereference() 灏卞彲浠ュ疄鐜颁负涓€涓?`memory_order_consume` 鍔犺浇銆傛棤璁哄叿浣撳疄鐜板浣曪紝鐢?rcu_dereference() 鍙栧埌鐨勬寚閽堥兘涓嶈兘鍦ㄥ叾鎵€鍦ㄧ殑锛堟渶澶栧眰鐨勶級RCU 璇荤涓寸晫鍖轰箣澶栦娇鐢紝闄ら潪鐩稿簲鏁版嵁鍏冪礌鐨勪繚鎶ゅ凡浠?RCU 杞氦缁欏叾浠栨煇绉嶅悓姝ユ満鍒讹紝鏈€甯歌鐨勬槸閿佹垨寮曠敤璁℃暟锛坰ee ../../rcuref.rst锛夈€?


绠€鑰岃█涔嬶紝鏇存柊鑰呬娇鐢?rcu_assign_pointer()锛岃鑰呬娇鐢?rcu_dereference()锛岃繖涓や釜 RCU API 鍏冪礌鍗忓悓宸ヤ綔锛岀‘淇濊鑰呭鏂版坊鍔犵殑鏁版嵁鍏冪礌鏈変竴鑷寸殑瑙嗗浘銆?

褰撶劧锛岃繕闇€瑕佷粠 RCU 淇濇姢鐨勬暟鎹粨鏋勪腑绉婚櫎鍏冪礌锛屼緥濡備娇鐢ㄥ涓嬭繃绋嬶細

#. 灏嗘暟鎹厓绱犱粠澶栧眰缁撴瀯涓Щ闄ゃ€?
#. 绛夊緟鎵€鏈夋棦瀛樼殑 RCU 璇荤涓寸晫鍖哄畬鎴愶紙鍥犱负鍙湁鏃㈠瓨璇昏€呮墠鍙兘鎸佹湁瀵规柊绉婚櫎鏁版嵁鍏冪礌鐨勫紩鐢級銆?
#. 姝ゆ椂锛屽彧鏈夋洿鏂拌€呮寔鏈夊鏂扮Щ闄ゆ暟鎹厓绱犵殑寮曠敤锛屽洜姝ゅ畠鍙互瀹夊叏鍦板洖鏀惰鏁版嵁鍏冪礌锛屼緥濡傚皢鍏朵紶閫掔粰 kfree()銆?

杩欎竴杩囩▼鐢?remove_gp_synchronous() 瀹炵幇锛?

```

       1 bool remove_gp_synchronous(void)
       2 {
       3   struct foo *p;
       4
       5   spin_lock(&gp_lock);
       6   p = rcu_access_pointer(gp);
       7   if (!p) {
       8     spin_unlock(&gp_lock);
       9     return false;
      10   }
      11   rcu_assign_pointer(gp, NULL);
      12   spin_unlock(&gp_lock);
      13   synchronize_rcu();
      14   kfree(p);
      15   return true;
      16 }
      17
```

杩欎釜鍑芥暟寰堢畝鍗曪紝绗?13 琛屽湪閲婃斁鏃ф暟鎹厓绱狅紙绗?14 琛岋級涔嬪墠绛夊緟涓€涓闄愭湡銆傝繖涓€绛夊緟纭繚璇昏€呬細鍦ㄨ `p` 寮曠敤鐨勬暟鎹厓绱犺閲婃斁涔嬪墠鍒拌揪 do_something_gp() 鐨勭 7 琛屻€傜 6 琛岀殑 rcu_access_pointer() 绫讳技浜?rcu_dereference()锛屽尯鍒湪浜庯細

#. rcu_access_pointer() 杩斿洖鐨勫€间笉鑳借瑙ｅ紩鐢ㄣ€傚鏋滀綘鎯冲悓鏃惰闂墍鎸囧悜鐨勫€煎拰鎸囬拡鏈韩锛岃浣跨敤 rcu_dereference() 鑰屼笉鏄?rcu_access_pointer()銆?
#. 瀵?rcu_access_pointer() 鐨勮皟鐢ㄦ棤闇€鍙楀埌淇濇姢銆傜浉姣斾箣涓嬶紝rcu_dereference() 蹇呴』浣嶄簬 RCU 璇荤涓寸晫鍖哄唴锛屾垨鑰呬綅浜庢寚閽堜笉浼氬彉鍖栫殑浠ｇ爜娈典腑锛屼緥濡傚彈鐩稿簲鏇存柊绔攣淇濇姢鐨勪唬鐮併€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Without the rcu_dereference() or the rcu_access_pointer(),            |
| what destructive optimizations might the compiler make use of?        |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Let's start with what happens to do_something_gp() if it fails to     |
| use rcu_dereference(). It could reuse a value formerly fetched        |
| from this same pointer. It could also fetch the pointer from `gp`   |  
| in a byte-at-a-time manner, resulting in **load tearing**, in turn      |
| resulting a bytewise mash-up of two distinct pointer values. It might |
| even use value-speculation optimizations, where it makes a wrong      |
| guess, but by the time it gets around to checking the value, an       |
| update has changed the pointer to match the wrong guess. Too bad      |
| about any dereferences that returned pre-initialization garbage in    |
| the meantime!                                                         |
| For remove_gp_synchronous(), as long as all modifications to          |
| `gp` are carried out while holding `gp_lock`, the above           |    
| optimizations are harmless. However, `sparse` will complain if you  |  
| define `gp` with `__rcu` and then access it without using either  |    
| rcu_access_pointer() or rcu_dereference().                            |
+-----------------------------------------------------------------------+

绠€鑰岃█涔嬶紝RCU 鐨勫彂甯?璁㈤槄淇濊瘉鐢?rcu_assign_pointer() 鍜?rcu_dereference() 鐨勭粍鍚堟彁渚涖€傝繖涓€淇濊瘉鍏佽灏嗘暟鎹厓绱犲畨鍏ㄥ湴娣诲姞鍒?RCU 淇濇姢鐨勯摼琛ㄦ暟鎹粨鏋勪腑鑰屼笉鎵撴壈 RCU 璇昏€呫€傝繖涓€淇濊瘉鍙笌瀹介檺鏈熶繚璇佺粨鍚堜娇鐢紝浠庤€屼篃鍏佽灏嗘暟鎹厓绱犱粠 RCU 淇濇姢鐨勯摼琛ㄦ暟鎹粨鏋勪腑绉婚櫎锛屽悓鏍蜂笉鎵撴壈 RCU 璇昏€呫€?

杩欎竴淇濊瘉鍙槸閮ㄥ垎鍦伴鍏堣鎯宠繃銆侱YNIX/ptx 鍦ㄥ彂甯冩椂浣跨敤浜嗕竴鏉℃樉寮忓唴瀛樺睆闅滐紝浣嗗湪璁㈤槄鏃舵病鏈変换浣曠被浼?rcu_dereference() 鐨勪笢瑗匡紝涔熸病鏈変换浣曠被浼煎悗鏉ヨ绾冲叆 rcu_dereference()銆佸啀鍚庢潵鍙堣绾冲叆 READ_ONCE() 鐨勪緷璧栭『搴忓睆闅滅殑涓滆タ銆傚浜庤繖浜涙搷浣滅殑闇€姹傦紝鏄湪 1990 骞翠唬鏈笌 DEC Alpha 鏋舵瀯甯堢殑涓€娆′細璁笂绐佺劧鏄剧幇鍑烘潵鐨勶紝閭ｆ椂 DEC 杩樻槸涓€瀹剁嫭绔嬬殑鍏徃銆侫lpha 鏋舵瀯甯堣姳浜嗚冻瓒充竴涓皬鏃舵墠璁╂垜鐩镐俊绔熺劧浼氶渶瑕佷换浣曠绫荤殑灞忛殰锛岃€屾垜闅忓悗鍙堣姳浜嗚冻瓒?*涓?*涓皬鏃舵墠璁╀粬浠浉淇′粬浠殑鏂囨。骞舵病鏈夋妸杩欎竴鐐硅娓呮銆傝繎浜涘勾鏉ヤ笌 C 鍜?C++ 鏍囧噯濮斿憳浼氱殑鍚堜綔锛岃浜轰滑浠庣紪璇戝櫒閭ｉ噷瀛﹀埌浜嗗緢澶氭妧宸у拰闄烽槺銆傜畝鑰岃█涔嬶紝鍦?1990 骞翠唬鍒濈紪璇戝櫒杩樻病閭ｄ箞鍒侀捇锛屼絾鍦?2015 骞达紝鍗冧竾鍒兂鐫€鐪佺暐 rcu_dereference()锛?

#### 鍐呭瓨灞忛殰淇濊瘉


涓婁竴鑺傜畝鍗曠殑閾捐〃鏁版嵁缁撴瀯鍦烘櫙娓呮鍦板睍绀轰簡锛屽湪鎷ユ湁澶氫釜 CPU 鐨勭郴缁熶笂锛屼负浣曢渶瑕?RCU 涓ユ牸鐨勫唴瀛橀『搴忎繚璇侊細

#. 姣忎釜鎷ユ湁鍦ㄦ煇涓?synchronize_rcu() 寮€濮嬩箣鍓嶅紑濮嬬殑 RCU 璇荤涓寸晫鍖虹殑 CPU锛岄兘淇濊瘉鍦ㄨ RCU 璇荤涓寸晫鍖虹粨鏉熶笌璇?synchronize_rcu() 杩斿洖涔嬮棿鐨勬煇涓椂鍒绘墽琛屼竴鏉″畬鏁村唴瀛樺睆闅溿€傛病鏈夎繖涓€淇濊瘉锛屼竴涓棦瀛樼殑 RCU 璇荤涓寸晫鍖哄彲鑳戒細鍦?remove_gp_synchronous() 绗?14 琛岀殑 kfree() 涔嬪悗浠嶇劧鎸佹湁瀵规柊绉婚櫎鐨?`struct foo` 鐨勫紩鐢ㄣ€?
#. 姣忎釜鎷ユ湁鍦ㄦ煇涓?synchronize_rcu() 杩斿洖涔嬪悗缁撴潫鐨?RCU 璇荤涓寸晫鍖虹殑 CPU锛岄兘淇濊瘉鍦?synchronize_rcu() 寮€濮嬩笌璇?RCU 璇荤涓寸晫鍖哄紑濮嬩箣闂寸殑鏌愪釜鏃跺埢鎵ц涓€鏉″畬鏁村唴瀛樺睆闅溿€傛病鏈夎繖涓€淇濊瘉锛屽湪 remove_gp_synchronous() 绗?14 琛岀殑 kfree() 涔嬪悗杩愯鐨勩€佹洿鏅氱殑 RCU 璇荤涓寸晫鍖猴紝绋嶅悗鍙兘浼氳繍琛?do_something_gp() 骞舵壘鍒版柊鍒犻櫎鐨?`struct foo`銆?
#. 濡傛灉璋冪敤 synchronize_rcu() 鐨勪换鍔″仠鐣欏湪鏌愪釜缁欏畾鐨?CPU 涓婏紝閭ｄ箞璇?CPU 淇濊瘉鍦?synchronize_rcu() 鎵ц鏈熼棿鐨勬煇涓椂鍒绘墽琛屼竴鏉″畬鏁村唴瀛樺睆闅溿€傝繖涓€淇濊瘉纭繚 remove_gp_synchronous() 绗?14 琛岀殑 kfree() 纭疄鍦ㄧ 11 琛岀殑绉婚櫎涔嬪悗鎵ц銆?
#. 濡傛灉璋冪敤 synchronize_rcu() 鐨勪换鍔″湪杩欐璋冪敤鏈熼棿鍦ㄤ竴缁?CPU 涔嬮棿杩佺Щ锛岄偅涔堣缁勪腑鐨勬瘡涓?CPU 閮戒繚璇佸湪 synchronize_rcu() 鎵ц鏈熼棿鐨勬煇涓椂鍒绘墽琛屼竴鏉″畬鏁村唴瀛樺睆闅溿€傝繖涓€淇濊瘉鍚屾牱纭繚 remove_gp_synchronous() 绗?14 琛岀殑 kfree() 纭疄鍦ㄧ 11 琛岀殑绉婚櫎涔嬪悗鎵ц锛岃€屼笖娑电洊浜嗘墽琛?synchronize_rcu() 鐨勪换鍔″湪姝ゆ湡闂村彂鐢熻縼绉荤殑鎯呭喌銆?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Given that multiple CPUs can start RCU read-side critical sections at |
| any time without any ordering whatsoever, how can RCU possibly tell   |
| whether or not a given RCU read-side critical section starts before a |
| given instance of synchronize_rcu()?                                  |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| If RCU cannot tell whether or not a given RCU read-side critical      |
| section starts before a given instance of synchronize_rcu(), then     |
| it must assume that the RCU read-side critical section started first. |
| In other words, a given instance of synchronize_rcu() can avoid       |
| waiting on a given RCU read-side critical section only if it can      |
| prove that synchronize_rcu() started first.                           |
| A related question is 鈥淲hen rcu_read_lock() doesn't generate any      |
| code, why does it matter how it relates to a grace period?鈥?The       |
| answer is that it is not the relationship of rcu_read_lock()          |
| itself that is important, but rather the relationship of the code     |
| within the enclosed RCU read-side critical section to the code        |
| preceding and following the grace period. If we take this viewpoint,  |
| then a given RCU read-side critical section begins before a given     |
| grace period when some access preceding the grace period observes the |
| effect of some access within the critical section, in which case none |
| of the accesses within the critical section may observe the effects   |
| of any access following the grace period.                             |
|                                                                       |
| As of late 2016, mathematical models of RCU take this viewpoint, for  |
| example, see slides 62 and 63 of the `2016 LinuxCon                   |
| EU <http://www2.rdrop.com/users/paulmck/scalability/paper/LinuxMM.201 |
| 6.10.04c.LCE.pdf>`__                                                  |
| presentation.                                                         |
+-----------------------------------------------------------------------+

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| The first and second guarantees require unbelievably strict ordering! |
| Are all these memory barriers **really** required?                      |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Yes, they really are required. To see why the first guarantee is      |
| required, consider the following sequence of events:                  |
|                                                                       |
| #. CPU 1: rcu_read_lock()                                             |
| #. CPU 1: `q = rcu_dereference(gp); /** Very likely to return p. **/` |
| #. CPU 0: `list_del_rcu(p);`                                        |  
| #. CPU 0: synchronize_rcu() starts.                                   |
| #. CPU 1: `do_something_with(q->a);`                                |  
|    `/** No smp_mb(), so might happen after kfree(). **/`              |
| #. CPU 1: rcu_read_unlock()                                           |
| #. CPU 0: synchronize_rcu() returns.                                  |
| #. CPU 0: `kfree(p);`                                               |  
|                                                                       |
| Therefore, there absolutely must be a full memory barrier between the |
| end of the RCU read-side critical section and the end of the grace    |
| period.                                                               |
|                                                                       |
| The sequence of events demonstrating the necessity of the second rule |
| is roughly similar:                                                   |
|                                                                       |
| #. CPU 0: `list_del_rcu(p);`                                        |  
| #. CPU 0: synchronize_rcu() starts.                                   |
| #. CPU 1: rcu_read_lock()                                             |
| #. CPU 1: `q = rcu_dereference(gp);`                                |  
|    `/** Might return p if no memory barrier. **/`                     |
| #. CPU 0: synchronize_rcu() returns.                                  |
| #. CPU 0: `kfree(p);`                                               |  
| #. CPU 1: `do_something_with(q->a); /** Boom!!! **/`                  |
| #. CPU 1: rcu_read_unlock()                                           |
|                                                                       |
| And similarly, without a memory barrier between the beginning of the  |
| grace period and the beginning of the RCU read-side critical section, |
| CPU 1 might end up accessing the freelist.                            |
|                                                                       |
| The "as if" rule of course applies, so that any implementation that   |
| acts as if the appropriate memory barriers were in place is a correct |
| implementation. That said, it is much easier to fool yourself into    |
| believing that you have adhered to the as-if rule than it is to       |
| actually adhere to it!                                                |
+-----------------------------------------------------------------------+

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| You claim that rcu_read_lock() and rcu_read_unlock() generate         |
| absolutely no code in some kernel builds. This means that the         |
| compiler might arbitrarily rearrange consecutive RCU read-side        |
| critical sections. Given such rearrangement, if a given RCU read-side |
| critical section is done, how can you be sure that all prior RCU      |
| read-side critical sections are done? Won't the compiler              |
| rearrangements make that impossible to determine?                     |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| In cases where rcu_read_lock() and rcu_read_unlock() generate         |
| absolutely no code, RCU infers quiescent states only at special       |
| locations, for example, within the scheduler. Because calls to        |
| schedule() had better prevent calling-code accesses to shared         |
| variables from being rearranged across the call to schedule(), if     |
| RCU detects the end of a given RCU read-side critical section, it     |
| will necessarily detect the end of all prior RCU read-side critical   |
| sections, no matter how aggressively the compiler scrambles the code. |
| Again, this all assumes that the compiler cannot scramble code across |
| calls to the scheduler, out of interrupt handlers, into the idle      |
| loop, into user-mode code, and so on. But if your kernel build allows |
| that sort of scrambling, you have broken far more than just RCU!      |
+-----------------------------------------------------------------------+

娉ㄦ剰锛岃繖浜涘唴瀛樺睆闅滈渶姹傚苟涓嶈兘鍙栦唬 RCU 鐨勫熀鏈渶姹傦紝鍗冲闄愭湡瑕佺瓑寰呮墍鏈夋棦瀛樿鑰呫€傛伆鎭扮浉鍙嶏紝鏈妭鎸囧嚭鐨勫唴瀛樺睆闅滃繀椤讳互**寮哄埗**杩欎竴鍩烘湰闇€姹傜殑鏂瑰紡杩愪綔銆傚綋鐒讹紝涓嶅悓鐨勫疄鐜颁互涓嶅悓鐨勬柟寮忓己鍒惰繖涓€闇€姹傦紝浣嗗畠浠繀椤诲己鍒躲€?

#### 淇濊瘉鏃犳潯浠舵墽琛岀殑 RCU 鍘熻


甯歌鎯呭舰鐨?RCU 鍘熻鏄棤鏉′欢鐨勩€傚畠浠璋冪敤銆佸畬鎴愬伐浣溿€佺劧鍚庤繑鍥烇紝涓嶅彲鑳藉嚭閿欙紝涔熸棤闇€閲嶈瘯銆傝繖鏄?RCU 鐨勪竴椤瑰叧閿璁″摬瀛︺€?

鐒惰€岋紝杩欎竴鍝插鏄姟瀹炵殑鑰岄潪鍥烘墽鐨勩€傚鏋滄湁浜鸿兘涓烘煇涓壒瀹氱殑鏉′欢寮?RCU 鍘熻鎻愬嚭鍚堢悊鐨勭悊鐢憋紝瀹冨緢鍙兘浼氳瀹炵幇骞跺姞鍏ャ€傛瘯绔燂紝杩欎竴淇濊瘉鏄€嗗悜鎺ㄥ鍑烘潵鐨勶紝鑰岄潪棰勫厛璁炬兂銆俁CU 鍘熻鐨勬棤鏉′欢鐗规€ф渶鍒濆彧鏄疄鐜颁笂鐨勪竴涓剰澶栵紝鍚庢潵涓庡甫鏈夋潯浠跺紡鍘熻鐨勫悓姝ュ師璇墦浜ら亾鐨勭粡楠岋紝淇冧娇鎴戞妸杩欎竴鍋剁劧鎻愬崌涓轰繚璇併€傚洜姝わ紝鍚?RCU 娣诲姞鏉′欢寮忓師璇殑鐞嗙敱闇€瑕佸缓绔嬪湪璇﹀敖涓斾护浜轰俊鏈嶇殑鐢ㄤ緥涔嬩笂銆?

#### 淇濊瘉浠庤鍒板啓鐨勫崌绾?


灏?RCU 鑰岃█锛屽湪 RCU 璇荤涓寸晫鍖哄唴鎵ц涓€娆℃洿鏂版€绘槸鍙鐨勩€備緥濡傦紝璇?RCU 璇荤涓寸晫鍖哄彲鑳戒細鎼滅储鏌愪釜缁欏畾鐨勬暟鎹厓绱狅紝鐒跺悗鑾峰彇鏇存柊绔嚜鏃嬮攣浠ユ洿鏂拌鍏冪礌锛岃€屾暣涓繃绋嬮兘鐣欏湪璇?RCU 璇荤涓寸晫鍖哄唴銆傚綋鐒讹紝鍦ㄨ皟鐢?synchronize_rcu() 涔嬪墠蹇呴』鍏堥€€鍑?RCU 璇荤涓寸晫鍖猴紝涓嶈繃锛岃繖涓€涓嶄究鍙互閫氳繃浣跨敤鏈枃妗ｅ悗闈粙缁嶇殑 call_rcu() 鍜?kfree_rcu() API 鏉ラ伩鍏嶃€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| But how does the upgrade-to-write operation exclude other readers?    |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| It doesn't, just like normal RCU updates, which also do not exclude   |
| RCU readers.                                                          |
+-----------------------------------------------------------------------+

杩欎竴淇濊瘉鍏佽鍦ㄥ彧璇荤鍜屾洿鏂扮浠ｇ爜涔嬮棿鍏变韩鏌ユ壘浠ｇ爜锛屽苟涓旀槸棰勫厛璁炬兂杩囩殑锛屽嚭鐜板湪鏈€鏃╃殑 DYNIX/ptx RCU 鏂囨。涓€?

### 鍩烘湰闈為渶姹?


RCU 鎻愪緵鏋佸叾杞婚噺鐨勮鑰咃紝瀹冪殑璇荤淇濊瘉铏界劧鐩稿綋鏈夌敤锛屼絾鐩稿簲鍦颁篃寰堣交閲忋€傚洜姝わ紝浜轰滑澶鏄撳亣瀹?RCU 鎵€淇濊瘉鐨勬瘮瀹冨疄闄呬繚璇佺殑鏇村銆傚綋鐒讹紝RCU 涓嶄繚璇佺殑浜嬫儏鐨勬竻鍗曟槸鏃犻檺闀跨殑锛屼笉杩囷紝浠ヤ笅鍑犺妭鍒楀嚭浜嗕竴浜涙浘寮曡捣鍥版儜鐨勯潪淇濊瘉銆傞櫎鍙︽湁璇存槑澶栵紝杩欎簺闈炰繚璇侀兘鏄鍏堣鎯宠繃鐨勩€?

#. `Readers Impose Minimal Ordering`_
#. `Readers Do Not Exclude Updaters`_
#. `Updaters Only Wait For Old Readers`_
#. `Grace Periods Don't Partition Read-Side Critical Sections`_
#. `Read-Side Critical Sections Don't Partition Grace Periods`_
#### 璇昏€呮柦鍔犳渶灏忕殑椤哄簭绾︽潫


璇荤鏍囪濡?rcu_read_lock() 鍜?rcu_read_unlock() 闄や簡閫氳繃涓?synchronize_rcu() 杩欑被瀹介檺鏈?API 鐨勪氦浜掍箣澶栵紝缁濆涓嶆彁渚涗换浣曢『搴忎繚璇併€傝鏄庣櫧杩欎竴鐐癸紝璇风湅涓嬮潰杩欏绾跨▼锛?

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   WRITE_ONCE(x, 1);
       5   rcu_read_unlock();
       6   rcu_read_lock();
       7   WRITE_ONCE(y, 1);
       8   rcu_read_unlock();
       9 }
      10
      11 void thread1(void)
      12 {
      13   rcu_read_lock();
      14   r1 = READ_ONCE(y);
      15   rcu_read_unlock();
      16   rcu_read_lock();
      17   r2 = READ_ONCE(x);
      18   rcu_read_unlock();
      19 }
      20
```

鍦?thread0() 鍜?thread1() 骞跺彂鎵ц涔嬪悗锛屽緢鍙兘鍑虹幇

```

     (r1 == 1 && r2 == 0)

```

锛堜篃灏辨槸璇达紝`y` 鐪嬭捣鏉ユ槸鍦?`x` 涔嬪墠琚祴鍊肩殑锛夛紝濡傛灉 rcu_read_lock() 鍜?rcu_read_unlock() 鍏锋湁杈冨鐨勯『搴忕壒鎬э紝杩欐槸涓嶅彲鑳界殑銆備絾瀹冧滑娌℃湁锛屽洜姝?CPU 瀹屽叏鏈夋潈杩涜鏄捐憲鐨勯噸鏂版帓搴忋€傝繖鏄璁′娇鐒讹細浠讳綍鏄捐憲鐨勯『搴忕害鏉熼兘浼氭嫋鎱㈣繖浜涘揩閫熻矾寰?API銆?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Can't the compiler also reorder this code?                            |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| No, the volatile casts in READ_ONCE() and WRITE_ONCE()                |
| prevent the compiler from reordering in this particular case.         |
+-----------------------------------------------------------------------+

#### 璇昏€呬笉鎺掗櫎鏇存柊鑰?


rcu_read_lock() 鍜?rcu_read_unlock() 閮戒笉鎺掗櫎鏇存柊銆傚畠浠墍鍋氱殑鍏ㄩ儴浜嬫儏锛屽彧鏄樆姝㈠闄愭湡缁撴潫銆備笅闈㈢殑渚嬪瓙璇存槑浜嗚繖涓€鐐癸細

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   r1 = READ_ONCE(y);
       5   if (r1) {
       6     do_something_with_nonzero_x();
       7     r2 = READ_ONCE(x);
       8     WARN_ON(!r2); /* BUG!!! */
       9   }
      10   rcu_read_unlock();
      11 }
      12
      13 void thread1(void)
      14 {
      15   spin_lock(&my_lock);
      16   WRITE_ONCE(x, 1);
      17   WRITE_ONCE(y, 1);
      18   spin_unlock(&my_lock);
      19 }
      20
```

濡傛灉 thread0() 鍑芥暟鐨?rcu_read_lock() 鎺掗櫎浜?thread1() 鍑芥暟鐨勬洿鏂帮紝閭ｄ箞 WARN_ON() 灏辨案杩滀笉浼氳Е鍙戙€備絾浜嬪疄鏄紝闄や簡鍚庣画鐨勫闄愭湡涔嬪锛宺cu_read_lock() 鍑犱箮涓嶆帓闄や换浣曚笢瑗匡紝鑰?thread1() 娌℃湁浠讳綍瀹介檺鏈燂紝鍥犳 WARN_ON() 鑳藉骞朵笖纭疄浼氳Е鍙戙€?

#### 鏇存柊鑰呭彧绛夊緟鏃ц鑰?


浜轰滑寰堝鏄撴兂褰撶劧鍦拌涓猴紝鍦?synchronize_rcu() 瀹屾垚涔嬪悗锛屽氨娌℃湁璇昏€呭湪鎵ц浜嗐€傚繀椤绘姷鍒惰繖绉嶈鎯戯紝鍥犱负鏂扮殑璇昏€呭彲浠ュ湪 synchronize_rcu() 寮€濮嬩箣鍚庣珛鍗冲惎鍔紝鑰?synchronize_rcu() 娌℃湁涔夊姟绛夊緟杩欎簺鏂拌鑰呫€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Suppose that synchronize_rcu() did wait until **all** readers had       |
| completed instead of waiting only on pre-existing readers. For how    |
| long would the updater be able to rely on there being no readers?     |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| For no time at all. Even if synchronize_rcu() were to wait until      |
| all readers had completed, a new reader might start immediately after |
| synchronize_rcu() completed. Therefore, the code following            |
| synchronize_rcu() can **never** rely on there being no readers.         |
+-----------------------------------------------------------------------+

#### 瀹介檺鏈熶笉浼氬垎鍓茶绔复鐣屽尯


浜轰滑寰堝鏄撴兂褰撶劧鍦拌涓猴細濡傛灉鏌愪釜 RCU 璇荤涓寸晫鍖虹殑浠讳綍閮ㄥ垎浣嶄簬缁欏畾瀹介檺鏈熶箣鍓嶏紝鑰屽彟涓€涓?RCU 璇荤涓寸晫鍖虹殑浠讳綍閮ㄥ垎浣嶄簬鍚屼竴瀹介檺鏈熶箣鍚庯紝閭ｄ箞鏁翠釜绗竴涓?RCU 璇荤涓寸晫鍖哄繀瀹氫綅浜庢暣涓浜屼釜涔嬪墠銆傜劧鑰岋紝浜嬪疄骞堕潪濡傛锛氬崟涓闄愭湡骞朵笉浼氬 RCU 璇荤涓寸晫鍖虹殑闆嗗悎杩涜鍒嗗壊銆傝繖绉嶆儏鍐靛彲浠ュ涓嬭鏄庯紝鍏朵腑 `x`銆乣y` 鍜?`z` 鍒濆閮戒负闆讹細

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   WRITE_ONCE(a, 1);
       5   WRITE_ONCE(b, 1);
       6   rcu_read_unlock();
       7 }
       8
       9 void thread1(void)
      10 {
      11   r1 = READ_ONCE(a);
      12   synchronize_rcu();
      13   WRITE_ONCE(c, 1);
      14 }
      15
      16 void thread2(void)
      17 {
      18   rcu_read_lock();
      19   r2 = READ_ONCE(b);
      20   r3 = READ_ONCE(c);
      21   rcu_read_unlock();
      22 }
      23
```

缁撴灉

```

     (r1 == 1 && r2 == 0 && r3 == 1)

```

瀹屽叏鍙兘鍑虹幇銆備笅鍥惧睍绀轰簡杩欐槸濡備綍鍙戠敓鐨勶紝鍏朵腑姣忎釜甯﹀湀 `QS` 琛ㄧず RCU 涓鸿绾跨▼璁板綍**闈欐鐘舵€?*锛坬uiescent state锛夌殑鏃跺埢锛屼篃灏辨槸 RCU 鐭ラ亾璇ョ嚎绋嬩笉鍙兘澶勪簬鍦ㄥ綋鍓嶅闄愭湡涔嬪墠寮€濮嬬殑 RCU 璇荤涓寸晫鍖轰箣涓殑鐘舵€侊細


濡傛灉纭疄鏈夊繀瑕佷互杩欑鏂瑰紡鍒嗗壊 RCU 璇荤涓寸晫鍖猴紝灏卞繀椤讳娇鐢ㄤ袱涓闄愭湡锛屽叾涓涓€涓闄愭湡宸茬煡鍦ㄧ浜屼釜瀹介檺鏈熷紑濮嬩箣鍓嶇粨鏉燂細

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   WRITE_ONCE(a, 1);
       5   WRITE_ONCE(b, 1);
       6   rcu_read_unlock();
       7 }
       8
       9 void thread1(void)
      10 {
      11   r1 = READ_ONCE(a);
      12   synchronize_rcu();
      13   WRITE_ONCE(c, 1);
      14 }
      15
      16 void thread2(void)
      17 {
      18   r2 = READ_ONCE(c);
      19   synchronize_rcu();
      20   WRITE_ONCE(d, 1);
      21 }
      22
      23 void thread3(void)
      24 {
      25   rcu_read_lock();
      26   r3 = READ_ONCE(b);
      27   r4 = READ_ONCE(d);
      28   rcu_read_unlock();
      29 }
      30
```

杩欓噷锛屽鏋?`(r1 == 1)`锛岄偅涔?thread0() 瀵?`b` 鐨勫啓鍏ュ繀瀹氬彂鐢熷湪 thread1() 鐨勫闄愭湡缁撴潫涔嬪墠銆傚鏋滄澶栬繕鏈?`(r4 == 1)`锛岄偅涔?thread3() 瀵?`b` 鐨勮鍙栧繀瀹氬彂鐢熷湪 thread2() 鐨勫闄愭湡寮€濮嬩箣鍚庛€傚鏋滃悓鏃惰繕鏈?`(r2 == 1)`锛岄偅涔?thread1() 鐨勫闄愭湡缁撴潫蹇呭畾鏃╀簬 thread2() 鐨勫闄愭湡寮€濮嬨€傝繖鎰忓懗鐫€涓や釜 RCU 璇荤涓寸晫鍖轰笉鑳介噸鍙狅紝浠庤€屼繚璇?`(r3 == 1)`銆傚洜姝わ紝缁撴灉

```

     (r1 == 1 && r2 == 1 && r3 == 0 && r4 == 1)

```

涓嶅彲鑳藉彂鐢熴€?

杩欎竴闈為渶姹傚悓鏍蜂笉鏄鍏堣鎯崇殑锛岃€屾槸鍦ㄧ爺绌?RCU 涓庡唴瀛樺簭鐨勪氦浜掓椂鍙樺緱鏄庢樉鐨勩€?

#### 璇荤涓寸晫鍖轰笉浼氬垎鍓插闄愭湡


浜轰滑鍚屾牱寰堝鏄撴兂褰撶劧鍦拌涓猴細濡傛灉涓€涓?RCU 璇荤涓寸晫鍖轰綅浜庝竴瀵瑰闄愭湡涔嬮棿锛岄偅涔堥偅浜涘闄愭湡灏变笉鑳介噸鍙犮€傜劧鑰岋紝杩欑璇辨儜鍙細鎶婁汉寮曞悜姝ч€旓紝姝ｅ涓嬮潰鐨勪緥瀛愭墍绀猴紝鎵€鏈夊彉閲忓垵濮嬮兘涓洪浂锛?

```

       1 void thread0(void)
       2 {
       3   rcu_read_lock();
       4   WRITE_ONCE(a, 1);
       5   WRITE_ONCE(b, 1);
       6   rcu_read_unlock();
       7 }
       8
       9 void thread1(void)
      10 {
      11   r1 = READ_ONCE(a);
      12   synchronize_rcu();
      13   WRITE_ONCE(c, 1);
      14 }
      15
      16 void thread2(void)
      17 {
      18   rcu_read_lock();
      19   WRITE_ONCE(d, 1);
      20   r2 = READ_ONCE(c);
      21   rcu_read_unlock();
      22 }
      23
      24 void thread3(void)
      25 {
      26   r3 = READ_ONCE(d);
      27   synchronize_rcu();
      28   WRITE_ONCE(e, 1);
      29 }
      30
      31 void thread4(void)
      32 {
      33   rcu_read_lock();
      34   r4 = READ_ONCE(b);
      35   r5 = READ_ONCE(e);
      36   rcu_read_unlock();
      37 }
      38
```

鍦ㄨ繖绉嶆儏鍐典笅锛岀粨鏋?

```

     (r1 == 1 && r2 == 1 && r3 == 1 && r4 == 0 && r5 == 1)

```

瀹屽叏鍙兘鍑虹幇锛屽涓嬪浘鎵€绀猴細


鍚屾牱锛屼竴涓?RCU 璇荤涓寸晫鍖哄彲浠ュ嚑涔庝笌鏁翠釜缁欏畾瀹介檺鏈熼噸鍙狅紝鍙瀹冧笉涓庢暣涓闄愭湡瀹屽叏閲嶅彔鍗冲彲銆傚洜姝わ紝涓€涓?RCU 璇荤涓寸晫鍖烘棤娉曞垎鍓蹭竴瀵?RCU 瀹介檺鏈熴€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| How long a sequence of grace periods, each separated by an RCU        |
| read-side critical section, would be required to partition the RCU    |
| read-side critical sections at the beginning and end of the chain?    |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| In theory, an infinite number. In practice, an unknown number that is |
| sensitive to both implementation details and timing considerations.   |
| Therefore, even in practice, RCU users must abide by the theoretical  |
| rather than the practical answer.                                     |
+-----------------------------------------------------------------------+

### 骞惰鐢熸椿鐨勭幇瀹?


杩欎簺骞惰鐢熸椿鐨勭幇瀹炵粷涓嶄粎闄愪簬 RCU锛屼絾 RCU 鐨勫疄鐜板繀椤婚伒瀹堝畠浠€傚洜姝ゅ畠浠€煎緱閲嶇敵锛?

#. 浠讳綍 CPU 鎴栦换鍔￠兘鍙兘鍦ㄤ换浣曟椂鍊欒寤惰繜锛岃€屼换浣曡瘯鍥鹃€氳繃绂佺敤鎶㈠崰銆佷腑鏂垨鍏朵粬鎵嬫鏉ラ伩鍏嶈繖浜涘欢杩熺殑鍋氭硶閮芥槸瀹屽叏寰掑姵鐨勩€傝繖鍦ㄥ彲鎶㈠崰鐨勭敤鎴锋€佺幆澧冧互鍙婅櫄鎷熷寲鐜锛堝叾涓粰瀹氬鎴锋満鎿嶄綔绯荤粺鐨?VCPU 闅忔椂鍙兘琚簳灞?hypervisor 鎶㈠崰锛変腑鏈€鏄庢樉锛屼絾涔熷彲鑳界敱浜?ECC 閿欒銆丯MI 浠ュ強鍏朵粬纭欢浜嬩欢鑰屽彂鐢熷湪瑁告満鐜涓€傚敖绠¤秴杩囩害 20 绉掔殑寤惰繜鍙兘瀵艰嚧 splat锛屼絾 RCU 瀹炵幇鏈変箟鍔′娇鐢ㄨ兘澶熷蹇嶆瀬闀垮欢杩熺殑绠楁硶锛屽彧涓嶈繃杩欓噷鐨勨€滄瀬闀库€濊繕涓嶅闀垮埌璁?64 浣嶈鏁板櫒鍦ㄩ€掑鏃跺彂鐢熷洖缁曘€?
#. 缂栬瘧鍣ㄥ拰 CPU 閮藉彲鑳藉鍐呭瓨璁块棶杩涜閲嶆帓銆傚湪閲嶈鐨勫湴鏂癸紝RCU 蹇呴』浣跨敤缂栬瘧鍣ㄦ寚浠ゅ拰鍐呭瓨灞忛殰鎸囦护鏉ヤ繚鎸侀『搴忋€?
#. 瀵逛换涓€缁欏畾缂撳瓨琛屼腑鍐呭瓨浣嶇疆鐨勭浉浜掑啿绐佺殑鍐欏叆浼氬鑷存槀璐电殑缂撳瓨鏈懡涓€傛洿澶氭暟閲忕殑骞跺彂鍐欏叆浠ュ強鏇撮绻佺殑骞跺彂鍐欏叆浼氬鑷存洿涓ラ噸鐨勫噺閫熴€傚洜姝?RCU 鏈変箟鍔′娇鐢ㄥ叿澶囪冻澶熷眬閮ㄦ€х殑绠楁硶锛屼互閬垮厤鏄捐憲鐨勬€ц兘鍜屽彲鎵╁睍鎬ч棶棰樸€?
#. 浣滀负涓€鏉＄矖鐣ョ殑缁忛獙娉曞垯锛屽湪浠讳綍缁欏畾鎺掍粬閿佺殑淇濇姢涓嬶紝鍙兘鎵ц鐩稿綋浜庝竴涓?CPU 鐨勫鐞嗛噺銆傚洜姝?RCU 蹇呴』浣跨敤鍙墿灞曠殑鍔犻攣璁捐銆?
#. 璁℃暟鍣ㄦ槸鏈夐檺鐨勶紝鍦?32 浣嶇郴缁熶笂灏ゅ叾濡傛銆傚洜姝?RCU 瀵硅鏁板櫒鐨勪娇鐢ㄥ繀椤昏兘澶熷蹇嶈鏁板櫒鍥炵粫锛屾垨鑰呰璁捐鎴愯鏁板櫒鍥炵粫鎵€闇€鐨勬椂闂磋繙瓒呰繃鍗曚釜绯荤粺鍙兘杩愯鐨勬椂闂淬€傚崄骞寸殑姝ｅ父杩愯鏃堕棿鐩稿綋鍙兘锛屼竴涓笘绾殑杩愯鏃堕棿鍒欒繙涓嶅彲鑳姐€備綔涓哄悗鑰呯殑涓€涓緥瀛愶紝RCU 鐨?dyntick-idle 宓屽璁℃暟鍣ㄤ负涓柇宓屽灞傜骇淇濈暀浜?54 浣嶏紙鍗充娇鍦?32 浣嶇郴缁熶笂锛岃璁℃暟鍣ㄤ篃鏄?64 浣嶏級銆傝璇ヨ鏁板櫒婧㈠嚭闇€瑕佹煇涓?CPU 鍦ㄤ笉鏇捐繘鍏ョ┖闂茬殑鎯呭喌涓嬪彂鐢?2\ `54` 娆″崐涓柇銆傚鏋滄瘡寰鍙戠敓涓€娆″崐涓柇锛岄偅涔堥渶瑕?570 骞寸殑杩愯鏃堕棿鎵嶄細璁╄璁℃暟鍣ㄦ孩鍑猴紝鐩墠杩欒璁や负鏄竴娈靛彲浠ユ帴鍙楃殑闀夸箙鏃堕棿銆?
#. Linux 绯荤粺鍙互鍦ㄥ崟涓叡浜唴瀛樼幆澧冧腑璁╂暟鍗冧釜 CPU 杩愯鍚屼竴涓?Linux 鍐呮牳銆傚洜姝?RCU 蹇呴』瀵嗗垏鍏虫敞楂樼鐨勫彲鎵╁睍鎬с€?
杩欐渶鍚庝竴鏉″苟琛岀敓娲荤殑鐜板疄鎰忓懗鐫€ RCU 蹇呴』鐗瑰埆鐣欐剰鍓嶈堪閭ｄ簺鐜板疄銆侺inux 鑳藉鎵╁睍鍒版嫢鏈夋暟鍗冧釜 CPU 鐨勭郴缁熺殑鎯虫硶锛屽湪 1990 骞翠唬鎴栬浼氶伃鍒颁竴浜涙€€鐤戯紝浣嗛櫎姝や箣澶栵紝杩欎簺瑕佹眰骞朵笉鍑轰汉鎰忔枡锛屽嵆渚垮湪 1990 骞翠唬鍒濅篃鏄姝ゃ€?

### 瀹炵幇璐ㄩ噺闇€姹?


浠ヤ笅鍚勮妭鍒楀嚭浜嗗疄鐜拌川閲忔柟闈㈢殑闇€姹傘€傚敖绠′竴涓拷鐣ヨ繖浜涢渶姹傜殑 RCU 瀹炵幇浠嶇劧鍙互浣跨敤锛屼絾瀹冨緢鍙兘浼氬彈鍒扮绉嶉檺鍒讹紝浠庤€屼笉閫傚悎宸ヤ笟绾х殑鐢熶骇缁忚惀浣跨敤銆傚疄鐜拌川閲忛渶姹傜殑绫诲埆濡備笅锛?

#. `Specialization`_
#. `Performance and Scalability`_
#. `Forward Progress`_
#. `Composability`_
#. `Corner Cases`_

杩欎簺绫诲埆灏嗗湪浠ヤ笅鍚勮妭涓垎鍒粙缁嶃€?

#### 涓撻棬鍖?


RCU 杩囧幓鏄€佺幇鍦ㄤ篃涓昏闈㈠悜浠ヨ涓轰富锛坮ead-mostly锛夌殑鍦烘櫙锛岃繖鎰忓懗鐫€ RCU 鐨勮绔師璇粡杩囦簡浼樺寲锛屽線寰€浠ョ壓鐗插叾鏇存柊绔師璇负浠ｄ环銆傝縿浠婄殑缁忛獙鍙敱涓嬮潰鍒楀嚭鐨勬儏褰㈡鎷細

#. 浠ヨ涓轰富鐨勬暟鎹紝涓旇繃鏈熷拰涓嶄竴鑷寸殑鏁版嵁涓嶆垚闂锛歊CU 琛ㄧ幇鏋佷匠锛?
#. 浠ヨ涓轰富鐨勬暟鎹紝涓旀暟鎹繀椤讳繚鎸佷竴鑷达細RCU 琛ㄧ幇鑹ソ銆?
#. 璇诲啓鍏兼湁鐨勬暟鎹紝涓旀暟鎹繀椤讳繚鎸佷竴鑷达細RCU **鍙兘**杩樿銆備篃鍙兘涓嶈銆?
#. 浠ュ啓涓轰富鐨勬暟鎹紝涓旀暟鎹繀椤讳繚鎸佷竴鑷达細RCU 鏋佷笉鍙兘鏄悎閫傜殑宸ュ叿锛屼絾鏈変互涓嬩緥澶栵紝鍦ㄨ繖浜涙儏鍐典笅 RCU 鍙互鎻愪緵锛?

   a. 瀵规洿鏂板弸濂界殑鏈哄埗鎻愪緵瀛樺湪鎬т繚璇併€?
   b. 涓哄疄鏃剁敤閫旀彁渚涙棤绛夊緟锛坵ait-free锛夌殑璇荤鍘熻銆?

杩欑浠ヨ涓轰富鐨勫彇鍚戞剰鍛崇潃 RCU 蹇呴』涓庡叾浠栧悓姝ュ師璇簰閫氥€備緥濡傦紝鍓嶉潰璁ㄨ鐨?add_gp() 鍜?remove_gp_synchronous() 绀轰緥浣跨敤 RCU 淇濇姢璇昏€呫€佺敤閿佹潵鍗忚皟鏇存柊鑰呫€傜劧鑰岋紝闇€姹傝繙涓嶆浜庢锛屽畠瑕佹眰鍚勭鍚勬牱鐨勫悓姝ュ師璇湪 RCU 璇荤涓寸晫鍖哄唴閮芥槸鍚堟硶鐨勶紝鍖呮嫭鑷棆閿併€侀『搴忛攣銆佸師瀛愭搷浣溿€佸紩鐢ㄨ鏁板櫒鍜屽唴瀛樺睆闅溿€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| What about sleeping locks?                                            |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| These are forbidden within Linux-kernel RCU read-side critical        |
| sections because it is not legal to place a quiescent state (in this  |
| case, voluntary context switch) within an RCU read-side critical      |
| section. However, sleeping locks may be used within userspace RCU     |
| read-side critical sections, and also within Linux-kernel sleepable   |
| RCU `(SRCU) <Sleepable RCU_>`__ read-side critical sections. In       |
| addition, the -rt patchset turns spinlocks into a sleeping locks so   |
| that the corresponding critical sections can be preempted, which also |
| means that these sleeplockified spinlocks (but not other sleeping     |
| locks!) may be acquire within -rt-Linux-kernel RCU read-side critical |
| sections.                                                             |
| Note that it **is** legal for a normal RCU read-side critical section   |
| to conditionally acquire a sleeping locks (as in                      |
| mutex_trylock()), but only as long as it does not loop                |
| indefinitely attempting to conditionally acquire that sleeping locks. |
| The key point is that things like mutex_trylock() either return       |
| with the mutex held, or return an error indication if the mutex was   |
| not immediately available. Either way, mutex_trylock() returns        |
| immediately without sleeping.                                         |
+-----------------------------------------------------------------------+

璁稿浜轰細鎯婅鍦板彂鐜帮紝寰堝绠楁硶骞朵笉瑕佹眰鏁版嵁瑙嗗浘涓€鑷达紝浣嗚澶氱畻娉曠‘瀹炶兘鍦ㄩ偅绉嶆ā寮忎笅宸ヤ綔锛岀綉缁滆矾鐢卞氨鏄渶鍏稿瀷鐨勪緥瀛愩€備簰鑱旂綉璺敱绠楁硶浼犳挱鏇存柊闇€瑕佺浉褰撻暱鐨勬椂闂达紝鍥犳褰撲竴娆℃洿鏂板埌杈炬煇涓郴缁熸椂锛岃绯荤粺宸茬粡鎶婄綉缁滄祦閲忓線閿欒鐨勬柟鍚戝彂閫佷簡鐩稿綋闀夸竴娈垫椂闂淬€傝灏戞暟鍑犱釜绾跨▼鍐嶅線閿欒鐨勬柟鍚戝鍙戝嚑姣鐨勬祦閲忔樉鐒朵笉鏄棶棰橈細鍦ㄦ渶鍧忔儏鍐典笅锛孴CP 閲嶄紶鏈€缁堜細鎶婃暟鎹€佸埌瀹冭鍘荤殑鍦版柟銆備竴鑸€岃█锛屽湪杩借釜璁＄畻鏈轰箣澶栫殑瀹囧畽鐘舵€佹椂锛岀敱浜庡厜閫熷欢杩燂紙鍗充娇涓嶈€冭檻鍏朵粬鍘熷洜锛夛紝鏌愮绋嬪害鐨勪笉涓€鑷存槸蹇呴』瀹瑰繊鐨勩€?

姝ゅ锛屽澶栭儴鐘舵€佺殑涓嶇‘瀹氭€у湪璁稿鎯呭喌涓嬫槸鍥烘湁鐨勩€備緥濡傦紝涓€瀵瑰吔鍖诲彲鑳戒細鐢ㄥ績璺虫潵鍒ゆ柇涓€鍙粰瀹氱殑鐚槸鍚﹁繕娲荤潃銆備絾鍦ㄦ渶鍚庝竴娆″績璺充箣鍚庯紝浠栦滑搴旇绛夊涔呮墠鏂畾杩欏彧鐚‘瀹炴浜嗭紵绛夊緟灏戜簬 400 姣姣棤鎰忎箟锛屽洜涓洪偅鎰忓懗鐫€涓€鍙斁鏉剧殑鐚細琚涓烘瘡鍒嗛挓鍦ㄦ浜′笌瀛樻椿涔嬮棿鍙嶅瓒呰繃 100 娆°€傝€屼笖锛屽氨鍍忎汉涓€鏍凤紝鐚殑蹇冭剰鍙兘浼氬仠璺充竴娈垫椂闂达紝鎵€浠ョ‘鍒囩殑绛夊緟鏃堕暱鏄竴涓垽鏂棶棰樸€傛垜浠繖瀵瑰吔鍖讳腑鐨勪竴浣嶅彲鑳戒細鍦ㄥ甯冪尗姝讳骸涔嬪墠绛?30 绉掞紝鑰屽彟涓€浣嶅彲鑳藉潥鎸佽绛夋弧涓€鍒嗛挓銆備簬鏄紝鍦ㄦ渶鍚庝竴娆″績璺充箣鍚庨偅涓€鍒嗛挓閲岀殑鏈€鍚?30 绉掍腑锛屼袱浣嶅吔鍖讳細瀵圭尗鐨勭姸鎬佹剰瑙佷笉涓€銆?

鏈夎叮鐨勬槸锛屽悓鏍风殑鐘跺喌涔熼€傜敤浜庣‖浠躲€傚埌浜嗗叧閿椂鍒伙紝鎴戜滑濡備綍鍒ゆ柇鏌愪釜澶栭儴鏈嶅姟鍣ㄦ槸鍚﹀凡缁忔晠闅滐紵鎴戜滑浼氬懆鏈熸€у湴鍚戝畠鍙戦€佹秷鎭紝濡傛灉鍦ㄧ粰瀹氭椂闂村唴娌℃湁鏀跺埌鍝嶅簲锛屽氨瀹ｅ竷瀹冩晠闅溿€傜瓥鐣ュ喅绛栭€氬父鑳藉瀹瑰繊鐭椂闂寸殑涓嶄竴鑷淬€傜瓥鐣ユ槸涓€娈垫椂闂翠箣鍓嶅畾涓嬬殑锛岀幇鍦ㄦ墠浠樿瀹炴柦锛屽洜姝ゅ嚑姣鐨勫欢杩熼€氬父鏃犲叧绱ц銆?

鐒惰€岋紝鏈変簺绠楁硶缁濆蹇呴』鐪嬪埌涓€鑷寸殑鏁版嵁銆備緥濡傦紝鐢ㄦ埛鎬?SystemV 淇″彿閲?ID 鍒扮浉搴斿唴鏍告暟鎹粨鏋勭殑杞崲鐢?RCU 淇濇姢锛屼絾缁濆绂佹鏇存柊涓€涓垰鍒氳绉婚櫎鐨勪俊鍙烽噺銆傚湪 Linux 鍐呮牳涓紝杩欑涓€鑷存€ч渶姹傛槸閫氳繃鍦?RCU 璇荤涓寸晫鍖哄唴鑾峰彇浣嶄簬鍐呮牳鏁版嵁缁撴瀯涓殑鑷棆閿佹潵婊¤冻鐨勶紝杩欑敱涓婂浘涓殑缁挎鎵€鏍囨槑銆傝澶氬叾浠栨妧鏈篃鍙兘銆佸苟涓斾簨瀹炰笂鍦?Linux 鍐呮牳涓‘瀹炶浣跨敤銆?

绠€鑰岃█涔嬶紝RCU 涓嶈礋璐ｇ淮鎶や竴鑷存€э紝褰撻渶瑕佷竴鑷存€ф椂锛屽彲浠ヤ笌鍏朵粬鏈哄埗閰嶅悎 RCU 涓€璧蜂娇鐢ㄣ€俁CU 鐨勪笓闂ㄥ寲璁╁畠鎶婃湰鑱屽伐浣滃仛寰楁瀬濂斤紝鑰屽畠涓庡叾浠栧悓姝ユ満鍒朵簰閫氱殑鑳藉姏锛屼娇寰楅拡瀵圭粰瀹氫换鍔′娇鐢ㄦ伆褰撶殑鍚屾宸ュ叿缁勫悎鎴愪负鍙兘銆?

#### 鎬ц兘涓庡彲鎵╁睍鎬?


鑳芥晥鏄綋浠婃€ц兘鐨勪竴涓叧閿粍鎴愰儴鍒嗭紝鍥犳 Linux 鍐呮牳鐨?RCU 瀹炵幇蹇呴』閬垮厤涓嶅繀瑕佸湴鍞ら啋绌洪棽 CPU銆傛垜涓嶈兘澹扮О杩欎竴闇€姹傛槸棰勫厛璁炬兂杩囩殑銆備簨瀹炰笂锛屾垜鏄湪涓€娆＄數璇濅氦璋堜腑浜嗚В鍒板畠鐨勶紝鍦ㄩ偅娆′氦璋堜腑锛屾垜寰楀埌浜嗗叧浜庣數姹犱緵鐢电郴缁熶腑鑳芥晥閲嶈鎬с€佷互鍙?Linux 鍐呮牳 RCU 瀹炵幇鍦ㄥ叿浣撹兘鏁堢己闄锋柟闈⑩€滃潶璇氳€屽紑鏄庘€濈殑鍙嶉銆傛嵁鎴戠殑缁忛獙锛岀數姹犱緵鐢电殑宓屽叆寮忕ぞ鍖轰細鎶婁换浣曚笉蹇呰鐨勫敜閱掕涓烘瀬涓嶅弸濂界殑琛屼负銆備互鑷充簬浠呬粎鍦?Linux 鍐呮牳閭欢鍒楄〃涓婂彂甯栭兘涓嶈冻浠ュ娉勪粬浠殑鎬掔伀銆?

鍐呭瓨鍦ㄥぇ澶氭暟鎯呭喌涓嬪苟闈炵壒鍒噸瑕侊紝骞朵笖闅忕潃鍐呭瓨瀹归噺鎵╁ぇ銆佸唴瀛樹环鏍兼毚璺岋紝瀹冪殑閲嶈鎬ц繕鍦ㄤ笅闄嶃€傜劧鑰岋紝姝ｅ鎴戜粠 Matt Mackall 鐨?`bloatwatch <http://elinux.org/Linux_Tiny-FAQ>`__ 宸ヤ綔涓墍瀛﹀埌鐨勶紝鍐呭瓨鍦ㄥ甫鏈夐潪鍙姠鍗狅紙`CONFIG_PREEMPTION=n`锛夊唴鏍哥殑鍗?CPU 绯荤粺涓婅嚦鍏抽噸瑕侊紝浜庢槸 `tiny RCU <https://lore.kernel.org/r/20090113221724.GA15307@linux.vnet.ibm.com>`__ 搴旇繍鑰岀敓銆傛鍚庯紝Josh Triplett 鎺ヨ繃灏忓瀷鍐呭瓨鐨勫ぇ鏃楋紝鍙戣捣浠栫殑 `Linux kernel tinification <https://tiny.wiki.kernel.org/>`__ 椤圭洰锛岃繖浣垮緱 `SRCU <Sleepable RCU_>`__ 瀵逛簬閭ｄ簺涓嶉渶瑕佸畠鐨勫唴鏍告垚涓哄彲閫夐」銆?

鍏朵綑鐨勬€ц兘闇€姹傚湪澶у鏁版儏鍐典笅閮戒笉鍑轰汉鎰忔枡銆備緥濡傦紝涓?RCU 鐨勮绔笓闂ㄥ寲鐩镐竴鑷达紝rcu_dereference() 搴斿綋鏈夊彲蹇界暐鐨勫紑閿€锛堜緥濡傦紝鎶戝埗灏戞暟杞诲井鐨勭紪璇戝櫒浼樺寲锛夈€傜被浼煎湴锛屽湪闈炲彲鎶㈠崰鐜涓紝rcu_read_lock() 鍜?rcu_read_unlock() 搴斿綋鏈夌‘鍒囦负闆剁殑寮€閿€銆?

鍦ㄥ彲鎶㈠崰鐜涓紝瀵逛簬鏈鎶㈠崰鐨?RCU 璇荤涓寸晫鍖猴紙鏈€楂樹紭鍏堢骇鐨勫疄鏃惰繘绋嬪氨鏄繖绉嶆儏鍐碉級锛宺cu_read_lock() 鍜?rcu_read_unlock() 搴斿綋鏈夋渶灏忕殑寮€閿€銆傜壒鍒湴锛屽畠浠笉搴斿寘鍚師瀛愯-淇敼-鍐欐搷浣溿€佸唴瀛樺睆闅滄寚浠ゃ€佺鐢ㄦ姠鍗犮€佺鐢ㄤ腑鏂垨鍚戝悗鍒嗘敮銆傜劧鑰岋紝瀵逛簬琚姠鍗犵殑 RCU 璇荤涓寸晫鍖猴紝rcu_read_unlock() 鍙互鑾峰彇鑷棆閿佸苟绂佺敤涓柇銆傝繖灏辨槸涓轰粈涔堣嚦灏戝湪琚姠鍗犵殑瀹炴椂寤惰繜褰卞搷鍙帶锛堝嵆涓寸晫鍖鸿冻澶熺煭锛夌殑鎯呭喌涓嬶紝鎶?RCU 璇荤涓寸晫鍖哄祵濂楀湪绂佺敤鎶㈠崰鍖哄煙鍐呫€佽€岄潪鐩稿弽锛屾槸鏇村ソ鐨勫仛娉曘€?

synchronize_rcu() 瀹介檺鏈熺瓑寰呭師璇槸閽堝鍚炲悙閲忎紭鍖栫殑銆傚洜姝わ紝闄や簡鏈€闀?RCU 璇荤涓寸晫鍖虹殑鎸佺画鏃堕棿涔嬪锛屽畠鍙兘杩樹細甯︽潵鑻ュ共姣鐨勫欢杩熴€傚彟涓€鏂归潰锛屽涓苟鍙戠殑 synchronize_rcu() 璋冪敤蹇呴』杩愮敤鎵瑰鐞嗕紭鍖栵紝浣垮緱瀹冧滑鑳界敱涓€涓簳灞傚闄愭湡绛夊緟鎿嶄綔鏉ユ弧瓒炽€備緥濡傦紝鍦?Linux 鍐呮牳涓紝鍗曟瀹介檺鏈熺瓑寰呮搷浣滄湇鍔′簬瓒呰繃 `1,000 separate invocations <https://www.usenix.org/conference/2004-usenix-annual-technical-conference/making-rcu-safe-deep-sub-millisecond-response>`__ 鐨?synchronize_rcu() 骞朵笉缃曡锛屼粠鑰屾妸姣忔璋冪敤鐨勫紑閿€鍒嗘憡鍒版帴杩戜簬闆躲€傜劧鑰岋紝瀹介檺鏈熶紭鍖栧悓鏃朵篃蹇呴』閬垮厤瀹炴椂璋冨害鍜屼腑鏂欢杩熷嚭鐜板彲娴嬮噺鐨勯€€鍖栥€?

鍦ㄦ煇浜涙儏鍐典笅锛屾暟姣绾х殑 synchronize_rcu() 寤惰繜鏄笉鍙帴鍙楃殑銆傚湪杩欎簺鎯呭喌涓嬶紝鍙互浣跨敤 synchronize_rcu_expedited() 鏉ユ浛浠ｏ紝鍦ㄥ皬鍨嬬郴缁熶笂鎶婂闄愭湡寤惰繜闄嶄綆鍒板嚑鍗佸井绉掞紙鑷冲皯鍦ㄨ绔复鐣屽尯杈冪煭鐨勬儏鍐典笅锛夈€傜洰鍓嶅浜庡ぇ鍨嬬郴缁熶笂鐨?synchronize_rcu_expedited() 娌℃湁鐗规畩鐨勫欢杩熼渶姹傦紝浣嗘槸锛屼笌 RCU 瑙勮寖鐨勭粡楠屾€ф湰璐ㄤ竴鑷达紝杩欎竴鐐瑰皢鏉ュ彲鑳戒細鏀瑰彉銆備笉杩囷紝鍙墿灞曟€ч渶姹傛槸纭嚳鏃犵枒鐨勶細鍦?4096 涓?CPU 涓婄獊濡傚叾鏉ョ殑涓€娉?synchronize_rcu_expedited() 璋冪敤鑷冲皯搴斿綋鍙栧緱鍚堢悊鐨勫悜鍓嶈繘灞曘€備綔涓鸿緝鐭欢杩熺殑鍥炴姤锛宻ynchronize_rcu_expedited() 琚厑璁稿闈炵┖闂茬殑鍦ㄧ嚎 CPU 鏂藉姞閫傚害鐨勫疄鏃跺欢杩熼€€鍖栥€傝繖閲岀殑鈥滈€傚害鈥濆ぇ鑷寸瓑鍚屼簬涓€娆¤皟搴︽椂閽熶腑鏂甫鏉ョ殑寤惰繜閫€鍖栥€?

杩樻湁浜涙儏鍐碉紝杩?synchronize_rcu_expedited() 闄嶄綆鍚庣殑瀹介檺鏈熷欢杩熶篃涓嶅彲鎺ュ彈銆傚湪杩欎簺鎯呭喌涓嬶紝鍙互浣跨敤寮傛鐨?call_rcu() 鏉ユ浛浠?synchronize_rcu()锛屽涓嬫墍绀猴細

```

       1 struct foo {
       2   int a;
       3   int b;
       4   struct rcu_head rh;
       5 };
       6
       7 static void remove_gp_cb(struct rcu_head *rhp)
       8 {
       9   struct foo *p = container_of(rhp, struct foo, rh);
      10
      11   kfree(p);
      12 }
      13
      14 bool remove_gp_asynchronous(void)
      15 {
      16   struct foo *p;
      17
      18   spin_lock(&gp_lock);
      19   p = rcu_access_pointer(gp);
      20   if (!p) {
      21     spin_unlock(&gp_lock);
      22     return false;
      23   }
      24   rcu_assign_pointer(gp, NULL);
      25   call_rcu(&p->rh, remove_gp_cb);
      26   spin_unlock(&gp_lock);
      27   return true;
      28 }
      29
```

缁堜簬闇€瑕佷竴涓?`struct foo` 鐨勫畾涔変簡锛屽畠鍑虹幇鍦ㄧ 1-5 琛屻€傚嚱鏁?remove_gp_cb() 鍦ㄧ 25 琛岃浼犵粰 call_rcu()锛屽苟灏嗗湪鍚庣画涓€涓闄愭湡缁撴潫涔嬪悗琚皟鐢ㄣ€傝繖杈惧埌浜嗕笌 remove_gp_synchronous() 鐩稿悓鐨勬晥鏋滐紝浣嗘棤闇€寮鸿揩鏇存柊鑰呯瓑寰呭闄愭湡杩囧幓銆俢all_rcu() 鍑芥暟鍙互鐢ㄥ湪 synchronize_rcu() 鍜?synchronize_rcu_expedited() 閮戒笉鍚堟硶鐨勮澶氭儏鍐典笅锛屽寘鎷湪绂佺敤鎶㈠崰浠ｇ爜銆乴ocal_bh_disable() 浠ｇ爜銆佺鐢ㄤ腑鏂唬鐮佷互鍙婁腑鏂鐞嗙▼搴忎腑銆傜劧鑰岋紝鍗充究 call_rcu() 鍦?NMI 澶勭悊绋嬪簭涓紝浠ュ強鏉ヨ嚜绌洪棽鍜岀绾?CPU 鏃朵篃鏄潪娉曠殑銆傚洖璋冨嚱鏁帮紙鏈緥涓殑 remove_gp_cb()锛夊皢鍦?Linux 鍐呮牳鐨勮蒋涓柇锛坰oftware interrupt锛夌幆澧冧腑鎵ц锛屾棦鍙兘鍦ㄧ湡姝ｇ殑杞腑鏂鐞嗙▼搴忎腑锛屼篃鍙兘鍦?local_bh_disable() 鐨勪繚鎶や笅銆傚湪 Linux 鍐呮牳鍜岀敤鎴锋€佷腑锛岀紪鍐欎竴涓€楁椂杩囬暱鐨?RCU 鍥炶皟鍑芥暟閮芥槸涓嶅ソ鐨勫仛娉曘€傞暱鏃堕棿杩愯鐨勬搷浣滃簲褰撲氦缁欏崟鐙殑绾跨▼锛屾垨鍦?Linux 鍐呮牳涓氦缁欏伐浣滈槦鍒楋紙workqueue锛夈€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Why does line 19 use rcu_access_pointer()? After all,                 |
| call_rcu() on line 25 stores into the structure, which would          |
| interact badly with concurrent insertions. Doesn't this mean that     |
| rcu_dereference() is required?                                        |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Presumably the `->gp_lock` acquired on line 18 excludes any         |  
| changes, including any insertions that rcu_dereference() would        |
| protect against. Therefore, any insertions will be delayed until      |
| after `->gp_lock` is released on line 25, which in turn means that  |  
| rcu_access_pointer() suffices.                                        |
+-----------------------------------------------------------------------+

鐒惰€岋紝remove_gp_cb() 鎵€鍋氱殑鍏ㄩ儴浜嬫儏灏辨槸瀵规暟鎹厓绱犺皟鐢?kfree()銆傝繖鏄竴绉嶅父瑙佹儻鐢ㄦ硶锛屽苟寰楀埌 kfree_rcu() 鐨勬敮鎸侊紝瀹冨厑璁糕€滃彂灏勫悗涓嶇鈥濓紙fire and forget锛夊紡鐨勬搷浣滐紝濡備笅鎵€绀猴細

```

       1 struct foo {
       2   int a;
       3   int b;
       4   struct rcu_head rh;
       5 };
       6
       7 bool remove_gp_faf(void)
       8 {
       9   struct foo *p;
      10
      11   spin_lock(&gp_lock);
      12   p = rcu_dereference(gp);
      13   if (!p) {
      14     spin_unlock(&gp_lock);
      15     return false;
      16   }
      17   rcu_assign_pointer(gp, NULL);
      18   kfree_rcu(p, rh);
      19   spin_unlock(&gp_lock);
      20   return true;
      21 }
      22
```

娉ㄦ剰锛宺emove_gp_faf() 鍙槸绠€鍗曞湴璋冪敤 kfree_rcu() 鐒跺悗缁х画锛屾棤闇€鍐嶅叧娉ㄥ悗缁殑瀹介檺鏈熷拰 kfree()銆傚厑璁稿湪涓?call_rcu() 鐩稿悓鐨勭幆澧冧腑璋冪敤 kfree_rcu()銆傛湁瓒ｇ殑鏄紝DYNIX/ptx 鎷ユ湁 call_rcu() 鍜?kfree_rcu() 鐨勭瓑浠风墿锛屽嵈娌℃湁 synchronize_rcu()銆傝繖鏄洜涓?RCU 鍦?DYNIX/ptx 涓敤寰椾笉澶氾紝鎵€浠ユ瀬灏戞暟闇€瑕佺被浼?synchronize_rcu() 鐨勫湴鏂瑰共鑴嗗氨鍐呰仈瀹炵幇浜嗐€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Earlier it was claimed that call_rcu() and kfree_rcu()                |
| allowed updaters to avoid being blocked by readers. But how can that  |
| be correct, given that the invocation of the callback and the freeing |
| of the memory (respectively) must still wait for a grace period to    |
| elapse?                                                               |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| We could define things this way, but keep in mind that this sort of   |
| definition would say that updates in garbage-collected languages      |
| cannot complete until the next time the garbage collector runs, which |
| does not seem at all reasonable. The key point is that in most cases, |
| an updater using either call_rcu() or kfree_rcu() can proceed         |
| to the next update as soon as it has invoked call_rcu() or            |
| kfree_rcu(), without having to wait for a subsequent grace            |
| period.                                                               |
+-----------------------------------------------------------------------+

浣嗗鏋滄洿鏂拌€呭繀椤荤瓑寰呭湪瀹介檺鏈熺粨鏉熶箣鍚庢墠鎵ц鐨勪唬鐮佸畬鎴愶紝鑰屽湪姝ゆ湡闂村張鏈夊叾浠栦换鍔″彲浠ュ紑灞曞憿锛熻疆璇㈤鏍肩殑 get_state_synchronize_rcu() 鍜?cond_synchronize_rcu() 鍑芥暟鍙互鐢ㄤ簬姝ょ洰鐨勶紝濡備笅鎵€绀猴細
```

       1 bool remove_gp_poll(void)
       2 {
       3   struct foo *p;
       4   unsigned long s;
       5
       6   spin_lock(&gp_lock);
       7   p = rcu_access_pointer(gp);
       8   if (!p) {
       9     spin_unlock(&gp_lock);
      10     return false;
      11   }
      12   rcu_assign_pointer(gp, NULL);
      13   spin_unlock(&gp_lock);
      14   s = get_state_synchronize_rcu();
      15   do_something_while_waiting();
      16   cond_synchronize_rcu(s);
      17   kfree(p);
      18   return true;
      19 }
      20
```

鍦ㄧ 14 琛岋紝get_state_synchronize_rcu() 浠?RCU 鍙栧緱涓€涓€渃ookie鈥濓紝鐒跺悗绗?15 琛屾墽琛屽叾浠栦换鍔★紝鏈€鍚庯紝濡傛灉鍦ㄦ鏈熼棿宸茬粡鏈夊闄愭湡杩囧幓锛岀 16 琛屼細绔嬪嵆杩斿洖锛屽惁鍒欎細鎸夐渶绛夊緟銆俙get_state_synchronize_rcu` 鍜?cond_synchronize_rcu() 鐨勯渶姹傛槸鏈€杩戞墠鍑虹幇鐨勶紝鍥犳鐜板湪鍒ゆ柇瀹冧滑鑳藉惁缁忓彈浣忔椂闂寸殑鑰冮獙杩樹负鏃惰繃鏃┿€?

RCU 鍥犺€屾彁渚涗簡涓€绯诲垪宸ュ叿锛岃鏇存柊鑰呰兘澶熷湪寤惰繜銆佺伒娲绘€у拰 CPU 寮€閿€涔嬮棿鍋氬嚭鎵€闇€鐨勬潈琛°€?

#### 鍚戝墠鎺ㄨ繘


鐞嗚涓婏紝寤惰繜瀹介檺鏈熺殑瀹屾垚鍜屽洖璋冪殑璋冪敤鏄棤瀹崇殑銆傚湪瀹炶返涓紝涓嶄粎鍐呭瓨瀹归噺鏄湁闄愮殑锛岃€屼笖鍥炶皟鏈夋椂纭疄浼氬敜閱掞紝鑰屽厖鍒嗚鎺ㄨ繜鐨勫敜閱掑彲鑳藉緢闅句笌绯荤粺鎸傝捣鍖哄垎寮€鏉ャ€傚洜姝わ紝RCU 蹇呴』鎻愪緵鑻ュ共鏈哄埗鏉ヤ績杩涘悜鍓嶆帹杩涖€?

杩欎簺鏈哄埗骞堕潪涓囨棤涓€澶憋紝涔熶笉鍙兘涓囨棤涓€澶便€備妇涓€涓畝鍗曠殑渚嬪瓙锛歊CU 璇荤涓寸晫鍖轰腑鐨勬棤闄愬惊鐜紝鎸夊畾涔夊繀鐒堕樆姝㈠悗缁闄愭湡姘歌繙鏃犳硶瀹屾垚銆傚啀涓句竴涓洿澶嶆潅鐨勪緥瀛愶紝鑰冭檻涓€涓敤 `CONFIG_RCU_NOCB_CPU=y` 鏋勫缓銆佸苟浠?`rcu_nocbs=1-63` 寮曞鐨?64-CPU 绯荤粺锛屽叾涓?CPU 1 鍒?63 鍦ㄧ揣寰幆涓嚜鏃嬪苟璋冪敤 call_rcu()銆傚嵆浣胯繖浜涚揣寰幆杩樺寘鍚 cond_resched() 鐨勮皟鐢紙浠庤€屽厑璁稿闄愭湡瀹屾垚锛夛紝CPU 0 涔熸牴鏈棤娉曚互鍏朵粬 63 涓?CPU 娉ㄥ唽鍥炶皟鐨勯€熷害鏉ヨ皟鐢ㄥ洖璋冿紝鑷冲皯鍦ㄨ绯荤粺鑰楀敖鍐呭瓨涔嬪墠鏄繖鏍枫€傚湪杩欎袱涓緥瀛愪腑锛岄兘閫傜敤鈥滆湗铔涗緺鍘熷垯鈥濓細鑳藉姏瓒婂ぇ锛岃矗浠昏秺澶с€傜劧鑰岋紝鍙涓嶆互鐢ㄥ埌杩欑绋嬪害锛孯CU 灏辫瑕佹眰淇濊瘉瀹介檺鏈熺殑鍙婃椂瀹屾垚鍜屽洖璋冪殑鍙婃椂璋冪敤銆?

RCU 閲囧彇浠ヤ笅姝ラ鏉ヤ績浣垮闄愭湡鍙婃椂瀹屾垚锛?

#. 濡傛灉鏌愪釜瀹介檺鏈熸湭鑳藉湪 100 姣鍐呭畬鎴愶紝RCU 浼氳閭ｄ簺 CPU 涓婂悗缁 cond_resched() 鐨勮皟鐢ㄦ彁渚涗竴涓?RCU 闈欐鐘舵€併€俁CU 杩樹細璁╅偅浜?CPU 鐨?need_resched() 璋冪敤杩斿洖 `true`锛屼絾鍙兘鍦ㄧ浉搴?CPU 鐨勪笅涓€涓皟搴︽椂閽熶箣鍚庛€?
#. 鍦?`nohz_full` 鍐呮牳寮曞鍙傛暟涓彁鍒扮殑 CPU 鍙互鍦ㄥ唴鏍镐腑鏃犻檺鏈熻繍琛岃€屾棤闇€璋冨害鏃堕挓涓柇锛岃繖浼氭尗璐ヤ笂杩?need_resched() 绛栫暐銆傚洜姝?RCU 浼氬湪閭ｄ簺鍦?109 姣涔嬪悗浠嶇劧鍧氭寔涓嶆斁鐨?`nohz_full` CPU 涓婅皟鐢?resched_cpu()銆?
#. 鍦ㄤ娇鐢?`CONFIG_RCU_BOOST=y` 鏋勫缓鐨勫唴鏍镐腑锛屽鏋滄煇涓湪 RCU 璇荤涓寸晫鍖哄唴琚姠鍗犵殑浠诲姟鍧氭寔涓嶆斁瓒呰繃 500 姣锛孯CU 灏嗚瘔璇镐紭鍏堢骇鎻愬崌銆?
#. 濡傛灉鏌愪釜 CPU 鍦ㄥ闄愭湡杩涜鍒?10 绉掓椂浠嶇劧鍧氭寔涓嶆斁锛孯CU 浼氳皟鐢?resched_cpu() 鏉ヨ皟搴﹀畠锛岃€屾棤璁哄叾 `nohz_full` 鐘舵€佸浣曘€?

涓婅堪鏁板€兼槸杩愯鍦?`HZ=1000` 绯荤粺涓婄殑榛樿鍊笺€傚畠浠細闅?`HZ` 鍊肩殑鍙樺寲鑰屽彉鍖栵紝涔熷彲浠ヤ娇鐢ㄧ浉鍏崇殑 Kconfig 閫夐」鍜屽唴鏍稿紩瀵煎弬鏁版潵鏇存敼銆俁CU 鐩墠瀵硅繖浜涘弬鏁版病鏈夊仛澶鍚堢悊鎬ф鏌ワ紝鍥犳鏇存敼鏃惰鍔″繀灏忓績銆傛敞鎰忥紝杩欎簺鍚戝墠鎺ㄨ繘鎺柦鍙彁渚涚粰 RCU锛岃€屼笉鏄?`SRCU <Sleepable RCU_>`__ 鎴?`Tasks RCU`_銆?

RCU 鍦?call_rcu() 涓噰鍙栦互涓嬫楠わ紝浠ヤ績浣垮湪浠讳綍缁欏畾鐨勯潪 `rcu_nocbs` CPU 鎷ユ湁 10,000 涓洖璋冿紝鎴栬€呮瘮涓婃鎻愪緵榧撳姳鏃跺鍑?10,000 涓洖璋冩椂锛屽強鏃惰皟鐢ㄥ洖璋冿細

#. 濡傛灉杩樻病鏈夊闄愭湡鍦ㄨ繘琛屼腑锛屽垯鍚姩涓€涓闄愭湡銆?
#. 寮哄埗绔嬪嵆妫€鏌ラ潤姝㈢姸鎬侊紝鑰屼笉鏄瓑寰呰嚜瀹介檺鏈熷紑濮嬭捣宸茶繃鍘讳笁姣銆?
#. 绔嬪嵆鐢ㄥ悇鑷殑瀹介檺鏈熷畬鎴愮紪鍙风粰璇?CPU 鐨勫洖璋冩墦涓婃爣璁帮紝鑰屼笉鏄瓑寰?`RCU_SOFTIRQ` 澶勭悊绋嬪簭鑵惧嚭鎵嬫潵鍋氳繖浠朵簨銆?
#. 鎻愰珮鍥炶皟鎵ц鐨勬壒澶勭悊涓婇檺锛岃繖鑳戒互閫€鍖栧疄鏃跺搷搴斾负浠ｄ环鏉ュ姞閫熷洖璋冭皟鐢ㄣ€?

鍚屾牱锛岃繖浜涙槸杩愯鍦?`HZ=1000` 鏃剁殑榛樿鍊硷紝骞朵笖鍙互琚鐩栥€傚悓鏍凤紝杩欎簺鍚戝墠鎺ㄨ繘鎺柦鍙彁渚涚粰 RCU锛岃€屼笉鏄?`SRCU <Sleepable RCU_>`__ 鎴?`Tasks RCU`_銆傚嵆渚垮 RCU 鑰岃█锛宍rcu_nocbs` CPU 鐨勫洖璋冭皟鐢ㄥ悜鍓嶆帹杩涗篃杩滄湭鎴愮啛锛岄儴鍒嗗師鍥犳槸鍙楃泭浜?`rcu_nocbs` CPU 鐨勫伐浣滆礋杞藉線寰€璋冪敤 call_rcu() 鐨勯鐜囩浉瀵硅緝浣庛€傚鏋滃皢鏉ュ嚭鐜版棦闇€瑕?`rcu_nocbs` CPU 鍙堥渶瑕侀珮 call_rcu() 璋冪敤閫熺巼鐨勫伐浣滆礋杞斤紝閭ｄ箞灏遍渶瑕侀澶栫殑鍚戝墠鎺ㄨ繘宸ヤ綔銆?

#### 鍙粍鍚堟€?


鍙粍鍚堟€ц繎骞存潵鍙楀埌浜嗗緢澶氬叧娉紝鎴栬閮ㄥ垎鏄洜涓哄鏍哥‖浠朵笌涓哄崟绾跨▼鐜璁捐銆佺敤浜庡崟绾跨▼鐨勯潰鍚戝璞℃妧鏈彂鐢熶簡纰版挒銆傜悊璁轰笂锛孯CU 璇荤涓寸晫鍖哄彲浠ョ粍鍚堬紝浜嬪疄涓婂彲浠ヤ换鎰忔繁搴﹀湴宓屽銆傚疄璺典腑锛屼笌鎵€鏈夊彲缁勫悎缁撴瀯鐨勭幇瀹炲疄鐜颁竴鏍凤紝鏄湁闄愬埗鐨勩€?

瀵逛簬閭ｄ簺 rcu_read_lock() 鍜?rcu_read_unlock() 涓嶇敓鎴愪换浣曚唬鐮佺殑 RCU 瀹炵幇锛堜緥濡傚綋 `CONFIG_PREEMPTION=n` 鏃剁殑 Linux 鍐呮牳 RCU锛夛紝鍙互浠绘剰娣卞害鍦板祵濂椼€傛瘯绔熸病鏈夊紑閿€銆傚彧鏄紝濡傛灉鎵€鏈夎繖浜?rcu_read_lock() 鍜?rcu_read_unlock() 鐨勫疄渚嬪缂栬瘧鍣ㄥ彲瑙侊紝缂栬瘧鏈€缁堜細鍥犺€楀敖鍐呭瓨銆佸瓨鍌ㄧ┖闂存垨鐢ㄦ埛鑰愬績锛堣璋佸厛鍙戠敓锛夎€屽け璐ャ€傚鏋滃祵濂楀缂栬瘧鍣ㄤ笉鍙锛屽氨鍍忓悇鑷綅浜庣嫭绔嬬炕璇戝崟鍏冧腑鐨勪簰閫掑綊鍑芥暟閭ｆ牱锛屽氨浼氬鑷存爤婧㈠嚭銆傚鏋滃祵濂楅噰鍙栧惊鐜殑褰㈠紡锛屾垨璁镐吉瑁呮垚灏鹃€掑綊锛岄偅涔堣涔堟帶鍒跺彉閲忎細婧㈠嚭锛岃涔堬紙鍦?Linux 鍐呮牳涓級浣犱細寰楀埌涓€涓?RCU CPU 鍋滈】锛坰tall锛夎鍛娿€傚敖绠″姝わ紝杩欑被 RCU 瀹炵幇浠嶆槸鐜板瓨鏈€鍏峰彲缁勫悎鎬х殑鏋勯€犱箣涓€銆?

鏄惧紡璺熻釜宓屽娣卞害鐨?RCU 瀹炵幇鍙楀祵濂楁繁搴﹁鏁板櫒鐨勯檺鍒躲€備緥濡傦紝Linux 鍐呮牳鐨勫彲鎶㈠崰 RCU 鎶婂祵濂楅檺鍒朵负 `INT_MAX`銆傝繖瀵逛簬鍑犱箮鎵€鏈夊疄闄呯敤閫旈兘瓒冲浜嗐€傝瘽铏藉姝わ紝涓€瀵瑰墠鍚庣浉閭荤殑涓や釜 RCU 璇荤涓寸晫鍖猴紝濡傛灉鍦ㄥ畠浠箣闂存湁涓€涓瓑寰呭闄愭湡鐨勬搷浣滐紝灏变笉鑳借鍖呭惈鍦ㄥ彟涓€涓?RCU 璇荤涓寸晫鍖轰箣涓€傝繖鏄洜涓轰笉鍏佽鍦?RCU 璇荤涓寸晫鍖哄唴绛夊緟瀹介檺鏈燂細閭ｆ牱鍋氳涔堜細瀵艰嚧姝婚攣锛岃涔堜細瀵艰嚧 RCU 闅愬紡鍦版媶鍒嗗灞?RCU 璇荤涓寸晫鍖猴紝杩欎袱鑰呴兘涓嶅埄浜庝竴涓暱瀵夸笖绻佽崳鐨勫唴鏍搞€?

鍊煎緱涓€鎻愮殑鏄紝闄愬埗鍙粍鍚堟€у苟闈?RCU 鐙湁銆備緥濡傦紝璁稿浜嬪姟鍐呭瓨瀹炵幇绂佹缁勫悎涓€瀵硅涓€涓笉鍙挙閿€鎿嶄綔锛堜緥濡傜綉缁滄帴鏀舵搷浣滐級鍒嗛殧鐨勪簨鍔°€傚啀涓句竴涓緥瀛愶紝鍩轰簬閿佺殑涓寸晫鍖哄彲浠ユ儕浜鸿嚜鐢卞湴缁勫悎锛屼絾鍓嶆彁鏄繀椤婚伩鍏嶆閿併€?

绠€鑰岃█涔嬶紝灏界 RCU 璇荤涓寸晫鍖哄叿鏈夐珮搴﹀彲缁勫悎鎬э紝浣嗗湪鏌愪簺鎯呭喌涓嬩粛闇€瑕佸皬蹇冿紝灏卞儚浠讳綍鍏朵粬鍙粍鍚堢殑鍚屾鏈哄埗涓€鏍枫€?

#### 杈圭晫鎯呭喌


鏌愪釜缁欏畾鐨?RCU 宸ヤ綔璐熻浇鍙兘鏈夋簮婧愪笉鏂笖瀵嗛泦鐨?RCU 璇荤涓寸晫鍖猴紝鐢氳嚦鍙兘瀵嗛泦鍒板湪浠讳竴鏃跺埢閮借嚦灏戞湁涓€涓?RCU 璇荤涓寸晫鍖哄湪鎵ц銆俁CU 涓嶈兘鍏佽杩欑鎯呭喌闃诲瀹介檺鏈燂細鍙鎵€鏈?RCU 璇荤涓寸晫鍖洪兘鏄湁闄愮殑锛屽闄愭湡涔熷繀椤绘槸鏈夐檺鐨勩€?

璇濊櫧濡傛锛屽彲鎶㈠崰 RCU 瀹炵幇鍙兘浼氬鑷?RCU 璇荤涓寸晫鍖鸿鎶㈠崰寰堥暱鏃堕棿锛岃繖灏变骇鐢熶簡涓€涓暱鎸佺画鏃堕棿鐨?RCU 璇荤涓寸晫鍖恒€傝繖绉嶆儏鍐靛彧鍙兘鍑虹幇鍦ㄨ礋杞芥矇閲嶇殑绯荤粺涓紝浣嗕娇鐢ㄥ疄鏃朵紭鍏堢骇鐨勭郴缁熷綋鐒舵洿鑴嗗急銆傚洜姝わ紝鎻愪緵浜?RCU 浼樺厛绾ф彁鍗囨潵甯姪搴斿杩欑鎯呭喌銆傝瘽铏藉姝わ紝瀵?RCU 浼樺厛绾ф彁鍗囩殑纭垏闇€姹傚緢鍙兘闅忕潃缁忛獙鐨勭Н绱€屾紨鍙樸€?

鍏朵粬宸ヤ綔璐熻浇鍙兘鏈夐潪甯搁珮鐨勬洿鏂伴€熺巼銆傚敖绠℃湁浜轰細杈╃О杩欐牱鐨勫伐浣滆礋杞藉簲璇ヤ娇鐢?RCU 涔嬪鐨勫叾浠栦笢瑗匡紝浣嗕簨瀹炴槸 RCU 蹇呴』浼橀泤鍦板鐞嗚繖绫诲伐浣滆礋杞姐€傝繖涓€闇€姹傛槸鎺ㄥ姩瀹介檺鏈熸壒澶勭悊鐨勫彟涓€涓洜绱狅紝浣嗗畠涔熸槸 call_rcu() 浠ｇ爜璺緞涓鏌ュぇ閲忔帓闃?RCU 鍥炶皟鐨勮儗鍚庨┍鍔ㄥ姏銆傛渶鍚庯紝楂樻洿鏂伴€熺巼涓嶅簲寤惰繜 RCU 璇荤涓寸晫鍖猴紝灏界鍦ㄤ娇鐢?synchronize_rcu_expedited() 鏃讹紙鐢变簬璇ュ嚱鏁颁娇鐢ㄤ簡 smp_call_function_single()锛夊彲鑳戒細鍑虹幇涓€浜涘皬鐨勮绔欢杩熴€?

灏界杩欎笁绉嶈竟鐣屾儏鍐靛湪 1990 骞翠唬鍒濆氨宸茶鐞嗚В锛屼絾鍦?2000 骞翠唬鍒濓紝涓€涓敱绱у惊鐜腑鐨?`close(open(path))` 缁勬垚鐨勭畝鍗曠敤鎴锋€佹祴璇曠獊鐒惰浜哄楂樻洿鏂伴€熺巼杩欑杈圭晫鎯呭喌鏈変簡娣卞埢寰楀鐨勮璇嗐€傝繖涓祴璇曚篃淇冧娇鍔犲叆浜嗕竴浜?RCU 浠ｇ爜鏉ュ簲瀵归珮鏇存柊閫熺巼锛屼緥濡傦紝濡傛灉鏌愪釜缁欏畾 CPU 鍙戠幇鑷繁鎺掗槦鐨?RCU 鍥炶皟瓒呰繃 10,000 涓紝瀹冧細淇冧娇 RCU 閲囧彇瑙勯伩琛屽姩锛屾洿绉瀬鍦板惎鍔ㄥ闄愭湡锛屽苟鏇寸Н鏋佸湴寮哄埗瀹屾垚瀹介檺鏈熷鐞嗐€傝繖绉嶈閬胯鍔ㄤ娇瀹介檺鏈熸洿蹇湴瀹屾垚锛屼絾浠ｄ环鏄檺鍒朵簡 RCU 鐨勬壒澶勭悊浼樺寲锛屼粠鑰屽鍔犱簡璇ュ闄愭湡甯︽潵鐨?CPU 寮€閿€銆?

### 杞欢宸ョ▼闇€姹?


浠嬩簬澧ㄨ彶瀹氬緥涓庘€滅姱閿欐槸浜轰箣甯告儏鈥濅箣闂达紝鏈夊繀瑕侀槻鑼冩剰澶栦簨鏁呭拰璇敤锛?

#. 浜轰滑澶鏄撳繕璁板湪姣忎竴涓渶瑕佺殑鍦版柟浣跨敤 rcu_read_lock()锛屽洜姝ょ敤 `CONFIG_PROVE_RCU=y` 鏋勫缓鐨勫唴鏍镐細鍦?rcu_dereference() 琚敤浜?RCU 璇荤涓寸晫鍖轰箣澶栨椂鍙戝嚭 splat銆傛洿鏂扮浠ｇ爜鍙互浣跨敤 rcu_dereference_protected()锛屽畠鎺ュ彈涓€涓?`lockdep expression <https://lwn.net/Articles/371986/>`__ 鏉ヨ〃鏄庢槸浠€涔堟彁渚涗簡淇濇姢銆傚鏋滄墍鎸囩ず鐨勪繚鎶ゆ病鏈夋彁渚涳紝灏变細鍙戝嚭涓€涓?lockdep splat銆?
   璇诲啓鍏变韩鐨勪唬鐮佸彲浠ヤ娇鐢?rcu_dereference_check()锛屽畠涔熸帴鍙椾竴涓?lockdep 琛ㄨ揪寮忥紝骞朵笖濡傛灉 rcu_read_lock() 鍜屾墍鎸囩ず鐨勪繚鎶ら兘娌℃湁灏变綅锛屽氨浼氬彂鍑?lockdep splat銆傛澶栵紝rcu_dereference_raw() 鐢ㄤ簬閭ｄ簺锛堝笇鏈涘緢灏戣鐨勶級闅句互绠€渚挎弿杩版墍闇€淇濇姢鐨勫満鍚堛€傛渶鍚庯紝鎻愪緵 rcu_read_lock_held() 浠ュ厑璁告煇涓嚱鏁伴獙璇佽嚜宸辨槸鍦?RCU 璇荤涓寸晫鍖哄唴琚皟鐢ㄧ殑銆傛垜鏄湪 Thomas Gleixner 瀹℃煡浜嗚嫢骞?RCU 鐢ㄦ硶涔嬪悗涓嶄箙鎵嶆剰璇嗗埌杩欎竴缁勯渶姹傜殑銆?
#. 鏌愪釜缁欏畾鐨勫嚱鏁板彲鑳藉笇鏈涘湪鍏ュ彛澶勩€佸湪浣跨敤浠讳綍鍏朵粬 RCU API 涔嬪墠锛屾鏌?RCU 鐩稿叧鐨勫厛鍐虫潯浠躲€俽cu_lockdep_assert() 鍋氳繖浠朵簨锛屽畠鍦ㄥ惎鐢ㄤ簡 lockdep 鐨勫唴鏍镐腑鏂█璇ヨ〃杈惧紡锛屽惁鍒欎粈涔堥兘涓嶅仛銆?
#. 浜轰滑鍚屾牱瀹规槗蹇樿浣跨敤 rcu_assign_pointer() 鍜?rcu_dereference()锛屾垨璁革紙閿欒鍦帮級鐢ㄤ竴涓畝鍗曠殑璧嬪€兼潵鏇夸唬銆備负浜嗘崟鑾疯繖绫婚敊璇紝涓€涓粰瀹氱殑 RCU 淇濇姢鎸囬拡鍙互鐢?`__rcu` 鏍囪锛屼箣鍚?sparse 灏变細鎶辨€ㄥ璇ユ寚閽堢殑绠€鍗曡祴鍊艰闂€侫rnd Bergmann 璁╂垜鎰忚瘑鍒颁簡杩欎竴闇€姹傦紝骞朵笖杩樻彁渚涗簡鎵€闇€鐨?`patch series <https://lwn.net/Articles/376011/>`__銆?
#. 鐢?`CONFIG_DEBUG_OBJECTS_RCU_HEAD=y` 鏋勫缓鐨勫唴鏍镐細鍦ㄦ妸涓€涓暟鎹厓绱犺繛缁袱娆′紶缁?call_rcu()銆佽€屼腑闂存病鏈夊闄愭湡鏃跺彂鍑?splat銆傦紙杩欎釜閿欒绫讳技浜庡弻閲嶉噴鏀俱€傦級閭ｄ簺鍔ㄦ€佸垎閰嶇殑鐩稿簲 `rcu_head` 缁撴瀯浼氳鑷姩璺熻釜锛屼絾鍒嗛厤鍦ㄦ爤涓婄殑 `rcu_head` 缁撴瀯蹇呴』鐢?init_rcu_head_on_stack() 鍒濆鍖栵紝骞剁敤 destroy_rcu_head_on_stack() 娓呯悊銆傜被浼煎湴锛岄潤鎬佸垎閰嶇殑闈炴爤 `rcu_head` 缁撴瀯蹇呴』鐢?init_rcu_head() 鍒濆鍖栵紝骞剁敤 destroy_rcu_head() 娓呯悊銆侻athieu Desnoyers 璁╂垜鎰忚瘑鍒颁簡杩欎竴闇€姹傦紝骞朵笖杩樻彁渚涗簡鎵€闇€鐨?`patch <https://lore.kernel.org/r/20100319013024.GA28456@Krystal>`__銆?
#. RCU 璇荤涓寸晫鍖轰腑鐨勬棤闄愬惊鐜渶缁堜細瑙﹀彂涓€涓?RCU CPU 鍋滈】璀﹀憡 splat锛岃€屸€滄渶缁堚€濈殑鏃堕暱鐢?`RCU_CPU_STALL_TIMEOUT` `Kconfig` 閫夐」鎺у埗锛屾垨鑰咃紝涔熷彲鐢?`rcupdate.rcu_cpu_stall_timeout` 寮曞/sysfs 鍙傛暟鎺у埗銆傜劧鑰岋紝闄ら潪鏈夋煇涓闄愭湡鍦ㄧ瓑寰呴偅涓壒瀹氱殑 RCU 璇荤涓寸晫鍖猴紝鍚﹀垯 RCU 娌℃湁涔夊姟浜х敓杩欎釜 splat銆?

   鏌愪簺鏋佺鐨勫伐浣滆礋杞藉彲鑳芥湁鎰忓欢杩?RCU 瀹介檺鏈燂紝杩愯杩欎簺宸ヤ綔璐熻浇鐨勭郴缁熷彲浠ョ敤 `rcupdate.rcu_cpu_stall_suppress` 寮曞鏉ユ姂鍒?splat銆傝繖涓唴鏍稿弬鏁颁篃鍙互閫氳繃 `sysfs` 璁剧疆銆傛澶栵紝RCU CPU 鍋滈】璀﹀憡鍦?sysrq dump 鏈熼棿鍜?panic 鏈熼棿浼氶€傚緱鍏跺弽銆傚洜姝?RCU 鎻愪緵浜?rcu_sysrq_start() 鍜?rcu_sysrq_end() API 鎴愬憳锛屽垎鍒湪闀挎椂闂?sysrq dump 涔嬪墠鍜屼箣鍚庤皟鐢ㄣ€俁CU 杩樻彁渚涗簡 rcu_panic() 閫氱煡鍣紝瀹冧細鍦?panic 寮€濮嬫椂鑷姩琚皟鐢ㄦ潵鎶戝埗杩涗竴姝ョ殑 RCU CPU 鍋滈】璀﹀憡銆?

   杩欎竴闇€姹傚湪 1990 骞翠唬鍒濆氨鏄剧幇鍑烘潵浜嗭紝宸笉澶氭槸绗竴娆￠渶瑕佽皟璇?CPU 鍋滈】鏃躲€傝瘽铏藉姝わ紝涓?Linux 鐩告瘮锛孌YNIX/ptx 涓殑鍒濆瀹炵幇鏄浉褰撻€氱敤鐨勩€?

#. 灏界鑳芥娴嬪埌鎸囬拡浠?RCU 璇荤涓寸晫鍖烘硠婕忓嚭鏉ヤ細闈炲父濂斤紝浣嗙洰鍓嶈繕娌℃湁濂界殑鏂规硶鍙互鍋氬埌杩欎竴鐐广€備竴涓毦鐐规槸闇€瑕佸尯鍒嗏€滄寚閽堟硠婕忊€濅笌鈥滄寚閽堝凡浠?RCU 绉讳氦缁欏叾浠栨煇绉嶅悓姝ユ満鍒讹紙渚嬪寮曠敤璁℃暟锛夆€濊繖涓ょ鎯呭喌銆?
#. 鍦ㄧ敤 `CONFIG_RCU_TRACE=y` 鏋勫缓鐨勫唴鏍镐腑锛孯CU 鐩稿叧淇℃伅閫氳繃浜嬩欢璺熻釜鎻愪緵銆?
#. 鐩存帴浣跨敤 rcu_assign_pointer() 鍜?rcu_dereference() 鏉ュ垱寤哄吀鍨嬬殑閾捐〃鏁版嵁缁撴瀯鍙兘鎯婁汉鍦板鏄撳嚭閿欍€傚洜姝わ紝鎻愪緵浜?RCU 淇濇姢鐨?`linked lists <https://lwn.net/Articles/609973/#RCU%20List%20APIs>`__锛屼互鍙婏紙鏇磋繎鏈熺殑锛塕CU 淇濇姢鐨?`hash tables <https://lwn.net/Articles/612100/>`__銆傝澶氬叾浠栦笓闂ㄧ敤閫旂殑 RCU 淇濇姢鏁版嵁缁撴瀯鍦?Linux 鍐呮牳鍜岀敤鎴锋€?RCU 搴撲腑閮芥湁鎻愪緵銆?
#. 鏈変簺閾捐〃缁撴瀯鏄湪缂栬瘧鏃跺垱寤虹殑锛屼絾渚濈劧闇€瑕?`__rcu` 妫€鏌ャ€俁CU_POINTER_INITIALIZER() 瀹忕敤浜庢鐩殑銆?
#. 鍦ㄥ垱寤鸿閫氳繃鍗曚釜澶栭儴鎸囬拡鍙戝竷鐨勯摼琛ㄧ粨鏋勬椂锛屾病鏈夊繀瑕佷娇鐢?rcu_assign_pointer()銆備负姝ゆ彁渚涗簡 RCU_INIT_POINTER() 瀹忋€?

杩欎笉鏄竴浠界‖鎬ц€屽浐瀹氱殑娓呭崟锛歊CU 鐨勮瘖鏂兘鍔涘皢缁х画鐢辩湡瀹炰笘鐣?RCU 浣跨敤涓彂鐜扮殑缂洪櫡鐨勬暟閲忓拰绫诲瀷鏉ユ寚寮曘€?

### Linux 鍐呮牳甯︽潵鐨勫鏉傛€?


Linux 鍐呮牳涓哄寘鎷?RCU 鍦ㄥ唴鐨勫悇绫昏蒋浠舵彁渚涗簡涓€涓湁瓒ｇ殑鐜銆備竴浜涚浉鍏崇殑鍏虫敞鐐瑰涓嬶細

#. `Configuration`_
#. `Firmware Interface`_
#. `Early Boot`_
#. `Interrupts and NMIs`_
#. `Loadable Modules`_
#. `Hotplug CPU`_
#. `Scheduler and RCU`_
#. `Tracing and RCU`_
#. `Accesses to User Memory and RCU`_
#. `Energy Efficiency`_
#. `Scheduling-Clock Interrupts and RCU`_
#. `Memory Efficiency`_
#. `Performance, Scalability, Response Time, and Reliability`_

杩欎唤娓呭崟鍙兘骞朵笉瀹屾暣锛屼絾瀹冪‘瀹炶浜烘劅鍙楀埌浜嗘渶鏄捐憲鐨?Linux 鍐呮牳澶嶆潅鎬с€備互涓嬪悇鑺傚垎鍒粙缁嶄笂杩颁富棰樹箣涓€銆?

#### 閰嶇疆


RCU 鐨勭洰鏍囨槸鑷姩閰嶇疆锛岃繖鏍峰嚑涔庢病鏈変汉闇€瑕佹搷蹇?RCU 鐨?`Kconfig` 閫夐」銆傚苟涓斿浜庡嚑涔庢墍鐢ㄧ殑鐢ㄦ埛锛孯CU 纭疄鈥滃紑绠卞嵆鐢ㄢ€濆湴宸ヤ綔鑹ソ銆?

鐒惰€岋紝涔熸湁涓€浜涗笓闂ㄧ敤閫旀槸鐢卞唴鏍稿紩瀵煎弬鏁板拰 `Kconfig` 閫夐」鏉ュ鐞嗙殑銆備笉骞哥殑鏄紝`Kconfig` 绯荤粺浼氭樉寮忓湴灏辨柊鐨?`Kconfig` 閫夐」璇㈤棶鐢ㄦ埛锛岃繖灏辫姹傚嚑涔庢墍鏈夐€夐」閮介殣钘忓湪涓€涓?`CONFIG_RCU_EXPERT` `Kconfig` 閫夐」涔嬪悗銆?

杩欎竴鍒囧簲褰撶浉褰撴樉鑰屾槗瑙侊紝浣嗕簨瀹炴槸锛孡inus Torvalds 鏈€杩戜笉寰椾笉 `remind <https://lore.kernel.org/r/CA+55aFy4wcCwaL4okTs8wXhGZ5h-ibecy_Meg9C4MNQrUnwMcg@mail.gmail.com>`__ 鎴戣繖涓€闇€姹傘€?

#### 鍥轰欢鎺ュ彛


鍦ㄨ澶氭儏鍐典笅锛屽唴鏍镐粠鍥轰欢鑾峰彇鍏充簬绯荤粺鐨勪俊鎭紝鑰屾湁鏃朵俊鎭湪缈昏瘧杩囩▼涓涪澶变簡銆傛垨鑰呯炕璇戞槸鍑嗙‘鐨勶紝浣嗗師濮嬫秷鎭湰韬氨鏄亣鐨勩€?
渚嬪锛屾煇浜涚郴缁熺殑鍥轰欢浼氶珮浼?CPU 鐨勬暟閲忥紝鏈夋椂楂樺嚭寰堝鍊嶃€傚鏋?RCU 鍍忚繃鍘婚偅鏍峰ぉ鐪熷湴鐩镐俊鍥轰欢锛屽畠灏变細鍒涘缓杩囧鐨勬瘡-CPU kthread銆傚敖绠＄敱姝ゅ緱鍒扮殑绯荤粺浠嶇劧鑳芥纭繍琛岋紝浣嗛偅浜涘浣欑殑 kthread 浼氫笉蹇呰鍦版秷鑰楀唴瀛橈紝骞朵笖鍦ㄥ畠浠嚭鐜板湪 `ps` 鍒楄〃鏃朵細浠や汉鍥版儜銆?

RCU 鍥犳蹇呴』绛夊緟鏌愪釜缁欏畾 CPU 鐪熸涓婄嚎涔嬪悗锛屾墠鑳借鑷繁鐩镐俊璇?CPU 纭疄瀛樺湪銆傜敱姝や骇鐢熺殑鈥滃菇鐏?CPU鈥濓紙瀹冧滑姘歌繙涔熶笉浼氫笂绾匡級浼氶€犳垚鑻ュ共 `interesting complications <https://paulmck.livejournal.com/37494.html>`__銆?

#### 鏃╂湡鍚姩


Linux 鍐呮牳鐨勫惎鍔ㄨ繃绋嬫槸涓€涓湁瓒ｇ殑杩囩▼锛孯CU 鐢ㄥ緱寰堟棭锛岀敋鑷冲湪 rcu_init() 琚皟鐢ㄤ箣鍓嶃€備簨瀹炰笂锛孯CU 鐨勮澶氬師璇湪鍒濆浠诲姟鐨?`task_struct` 鍙敤銆佷笖寮曞 CPU 鐨勬瘡-CPU 鍙橀噺璁剧疆濂戒箣鍚庡氨鍙互浣跨敤銆傝绔師璇紙rcu_read_lock()銆乺cu_read_unlock()銆乺cu_dereference() 鍜?rcu_access_pointer()锛夊湪寰堟棭鐨勬椂鍊欏氨浼氭甯歌繍琛岋紝rcu_assign_pointer() 涔熸槸濡傛銆?

灏界 call_rcu() 鍙互鍦ㄥ惎鍔ㄦ湡闂寸殑浠讳綍鏃跺埢琚皟鐢紝浣嗗洖璋冧繚璇佽鍒?RCU 鐨勬墍鏈?kthread 閮界敓鎴愪箣鍚庢墠浼氳璋冪敤锛岃繖鍙戠敓鍦?early_initcall() 鏃跺埢銆傚洖璋冭皟鐢ㄧ殑杩欑寤惰繜鏄敱浜?RCU 鍦ㄥ畬鍏ㄥ垵濮嬪寲涔嬪墠涓嶄細璋冪敤鍥炶皟锛岃€岃繖涓€瀹屽叏鍒濆鍖栬绛夊埌璋冨害鍣ㄦ妸鑷繁鍒濆鍖栧埌 RCU 鑳藉鐢熸垚骞惰繍琛屽叾 kthread 鐨勭▼搴︿箣鍚庢墠鑳藉彂鐢熴€傜悊璁轰笂锛屾洿鏃╁湴璋冪敤鍥炶皟鏄彲鑳界殑锛岀劧鑰岋紝杩欏苟闈炰竾鐏佃嵂锛屽洜涓洪偅浜涘洖璋冭兘璋冪敤鐨勬搷浣滀細鍙楀埌涓ユ牸鐨勯檺鍒躲€?

涔熻浠や汉鎯婅鐨勬槸锛宻ynchronize_rcu() 鍜?synchronize_rcu_expedited() 鍦ㄩ潪甯告棭鐨勫惎鍔ㄩ樁娈典細姝ｅ父杩愯锛屽師鍥犳槸閭ｆ椂鍙湁涓€涓?CPU 涓旀姠鍗犺绂佺敤銆傝繖鎰忓懗鐫€瀵?synchronize_rcu()锛堟垨鍏跺悓绫伙級鏈韩鐨勮皟鐢ㄥ氨鏄竴涓潤姝㈢姸鎬侊紝浠庤€屼篃灏辨槸涓€涓闄愭湡锛屽洜姝ゆ棭鏈熷惎鍔ㄧ殑瀹炵幇鍙互鏄竴涓┖鎿嶄綔銆?

鐒惰€岋紝涓€鏃﹁皟搴﹀櫒鐢熸垚浜嗗畠鐨勭涓€涓?kthread锛岃繖绉嶆棭鏈熷惎鍔ㄦ妧宸у浜?`CONFIG_PREEMPTION=y` 鍐呮牳涓殑 synchronize_rcu()锛堜互鍙?synchronize_rcu_expedited()锛夊氨澶辨晥浜嗐€傚師鍥犳槸 RCU 璇荤涓寸晫鍖哄彲鑳戒細琚姠鍗狅紝杩欐剰鍛崇潃鍚庣画鐨?synchronize_rcu() 纭疄蹇呴』绛夊緟鏌愪簺涓滆タ锛岃€屼笉鏄畝鍗曞湴绔嬪嵆杩斿洖銆備笉骞哥殑鏄紝synchronize_rcu() 鍦ㄥ畠鐨勬墍鏈?kthread 閮界敓鎴愪箣鍓嶆棤娉曞仛鍒拌繖涓€鐐癸紝鑰岃繖瑕佸埌 early_initcalls() 鏈熼棿鐨勬煇涓椂鍒绘墠浼氬彂鐢熴€備絾杩欎笉鑳芥垚涓哄€熷彛锛歊CU 浠嶇劧琚姹傚湪杩欎竴鏃堕棿娈靛唴姝ｇ‘澶勭悊鍚屾瀹介檺鏈熴€備竴鏃﹀畠鐨勬墍鏈?kthread 閮藉惎鍔ㄥ苟杩愯锛孯CU 灏卞紑濮嬫甯歌繍琛屻€?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| How can RCU possibly handle grace periods before all of its kthreads  |
| have been spawned???                                                  |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Very carefully!                                                       |
| During the "dead zone" between the time that the scheduler spawns the |
| first task and the time that all of RCU's kthreads have been spawned, |
| all synchronous grace periods are handled by the expedited            |
| grace-period mechanism. At runtime, this expedited mechanism relies   |
| on workqueues, but during the dead zone the requesting task itself    |
| drives the desired expedited grace period. Because dead-zone          |
| execution takes place within task context, everything works. Once the |
| dead zone ends, expedited grace periods go back to using workqueues,  |
| as is required to avoid problems that would otherwise occur when a    |
| user task received a POSIX signal while driving an expedited grace    |
| period.                                                               |
|                                                                       |
| And yes, this does mean that it is unhelpful to send POSIX signals to |
| random tasks between the time that the scheduler spawns its first     |
| kthread and the time that RCU's kthreads have all been spawned. If    |
| there ever turns out to be a good reason for sending POSIX signals    |
| during that time, appropriate adjustments will be made. (If it turns  |
| out that POSIX signals are sent during this time for no good reason,  |
| other adjustments will be made, appropriate or otherwise.)            |
+-----------------------------------------------------------------------+

鎴戞槸閫氳繃涓€绯诲垪绯荤粺鎸傝捣鎵嶄簡瑙ｅ埌杩欎簺鍚姩鏈熼渶姹傜殑銆?

#### 涓柇涓?NMI


Linux 鍐呮牳鏈変腑鏂紝RCU 璇荤涓寸晫鍖哄湪涓柇澶勭悊绋嬪簭鍐呬互鍙婄鐢ㄤ腑鏂殑浠ｇ爜鍖哄煙鍐呴兘鏄悎娉曠殑锛宑all_rcu() 鐨勮皟鐢ㄤ篃鏄姝ゃ€?

鏌愪簺 Linux 鍐呮牳浣撶郴缁撴瀯鍙互浠庨潪绌洪棽鐨勮繘绋嬩笂涓嬫枃杩涘叆涓€涓腑鏂鐞嗙▼搴忥紝鐒跺悗灏卞啀涔熶笉绂诲紑瀹冿紝鑰屾槸鍋峰伔鍦拌浆鍥炶繘绋嬩笂涓嬫枃銆傝繖涓妧宸ф湁鏃惰鐢ㄦ潵浠庡唴鏍稿唴閮ㄨ皟鐢ㄧ郴缁熻皟鐢ㄣ€傝繖浜涒€滃崐涓柇鈥濇剰鍛崇潃 RCU 蹇呴』闈炲父灏忓績鍦拌绠椾腑鏂祵濂楀眰绾с€傛垜鏄湪閲嶅啓 RCU 鐨?dyntick-idle 浠ｇ爜鏃跺悆浜嗚嫤澶存墠浜嗚В鍒拌繖涓€闇€姹傜殑銆?

Linux 鍐呮牳鏈変笉鍙睆钄戒腑鏂紙NMI锛夛紝RCU 璇荤涓寸晫鍖哄湪 NMI 澶勭悊绋嬪簭鍐呮槸鍚堟硶鐨勩€傚€煎緱搴嗗垢鐨勬槸锛孯CU 鏇存柊绔師璇紙鍖呮嫭 call_rcu()锛夊湪 NMI 澶勭悊绋嬪簭鍐呮槸琚姝㈢殑銆?

灏界鍚嶄负涓嶅彲灞忚斀涓柇锛屾煇浜?Linux 鍐呮牳浣撶郴缁撴瀯鍗村彲浠ユ湁宓屽鐨?NMI锛孯CU 蹇呴』姝ｇ‘澶勭悊銆侫ndy Lutomirski 鐢ㄨ繖涓€闇€姹?`surprised me <https://lore.kernel.org/r/CALCETrXLq1y7e_dKFPgou-FKHB6Pu-r8+t-6Ds+8=va7anBWDA@mail.gmail.com>`__锛屼粬杩樺ソ蹇冨湴鐢?`an algorithm <https://lore.kernel.org/r/CALCETrXSY9JpW3uE6H8WYk81sg56qasA2aqmjMPsq5dOtzso=g@mail.gmail.com>`__ 璁╂垜鎯婂枩锛岃绠楁硶婊¤冻浜嗚繖涓€闇€姹傘€?

姝ゅ锛孨MI 澶勭悊绋嬪簭鍙兘琚湪 RCU 鐪嬫潵鏄櫘閫氫腑鏂殑涓滆タ鎵撴柇銆傚彂鐢熻繖绉嶆儏鍐电殑涓€绉嶉€斿緞鏄紝鐩存帴浠?NMI 澶勭悊绋嬪簭涓皟鐢?ct_irq_enter() 鍜?ct_irq_exit() 鐨勪唬鐮併€傝繖涓€鎯婁汉鐜板疄淇冩垚浜嗗綋鍓嶇殑浠ｇ爜缁撴瀯锛屽嵆璁?ct_irq_enter() 璋冪敤 ct_nmi_enter()銆乧t_irq_exit() 璋冪敤 ct_nmi_exit()銆傛病閿欙紝鎴戜篃鏄悆浜嗚嫤澶存墠浜嗚В鍒拌繖涓€闇€姹傜殑銆?

#### 鍙姞杞芥ā鍧?


Linux 鍐呮牳鏈夊彲鍔犺浇妯″潡锛岃繖浜涙ā鍧椾篃鍙互琚嵏杞姐€傚湪缁欏畾妯″潡琚嵏杞戒箣鍚庯紝浠讳綍璋冪敤鍏跺嚱鏁扮殑灏濊瘯閮戒細瀵艰嚧娈甸敊璇€傛ā鍧楃殑鍗歌浇鍑芥暟鍥犳蹇呴』鍙栨秷瀵逛换浣曞彲鍔犺浇妯″潡鍑芥暟鐨勫欢杩熻皟鐢紝渚嬪锛屼换浣曟湭鍐崇殑 mod_timer() 閮藉繀椤婚€氳繃 timer_shutdown_sync() 鎴栫被浼兼柟娉曟潵澶勭悊銆?

涓嶅垢鐨勬槸锛屾病鏈夊姙娉曞彇娑堜竴涓?RCU 鍥炶皟锛涗竴鏃︿綘璋冪敤浜?call_rcu()锛岃鍥炶皟鍑芥暟鏈€缁堝氨浼氳鎵ц锛岄櫎闈炵郴缁熷湪姝や箣鍓嶅畷鏈恒€傚洜涓虹敤璁╃郴缁熷穿婧冩潵鍥炲簲涓€娆℃ā鍧楀嵏杞借姹傞€氬父琚涓轰笉璐熺ぞ浼氳矗浠荤殑琛屼负锛屾垜浠渶瑕佸叾浠栨柟娉曟潵澶勭悊鍦ㄩ€旂殑 RCU 鍥炶皟銆?

RCU 鍥犺€屾彁渚?rcu_barrier()锛屽畠绛夊緟鎵€鏈夊湪閫旂殑 RCU 鍥炶皟閮借璋冪敤銆傚鏋滀竴涓ā鍧椾娇鐢ㄤ簡 call_rcu()锛屽畠鐨勯€€鍑哄嚱鏁板洜姝ゅ簲褰撻樆姝㈠皢鏉ヤ换浣曞 call_rcu() 鐨勮皟鐢紝鐒跺悗璋冪敤 rcu_barrier()銆傜悊璁轰笂锛屽簳灞傜殑妯″潡鍗歌浇浠ｇ爜鍙互鏃犳潯浠跺湴璋冪敤 rcu_barrier()锛屼絾鍦ㄥ疄璺典腑杩欎細甯︽潵涓嶅彲鎺ュ彈鐨勫欢杩熴€?

Nikita Danilov 閽堝涓€涓被浼肩殑鏂囦欢绯荤粺鍗歌浇鍦烘櫙鎸囧嚭浜嗚繖涓€闇€姹傦紝鑰?Dipankar Sarma 鎶?rcu_barrier() 寮曞叆浜?RCU銆俽cu_barrier() 鐢ㄤ簬妯″潡鍗歌浇鐨勯渶姹傛槸鍚庢潵鎵嶆樉鐜板嚭鏉ョ殑銆?


   rcu_barrier() 鍑芥暟骞朵笉鈥斺€旈噸澶嶄竴閬嶏紝**骞朵笉**鈥斺€旀湁涔夊姟绛夊緟涓€涓闄愭湡銆傚畠鍙瑕佹眰绛夊緟閭ｄ簺宸茬粡鎻愪氦鐨?RCU 鍥炶皟銆傚洜姝わ紝濡傛灉绯荤粺涓换浣曞湴鏂归兘娌℃湁鎻愪氦 RCU 鍥炶皟锛宺cu_barrier() 瀹屽叏鏈夋潈绔嬪嵆杩斿洖銆傚嵆浣跨‘鏈夊洖璋冭鎻愪氦锛宺cu_barrier() 涔熶笉涓€瀹氶渶瑕佺瓑寰呬竴涓闄愭湡銆?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Wait a minute! Each RCU callbacks must wait for a grace period to     |
| complete, and rcu_barrier() must wait for each pre-existing           |
| callback to be invoked. Doesn't rcu_barrier() therefore need to       |
| wait for a full grace period if there is even one callback posted     |
| anywhere in the system?                                               |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Absolutely not!!!                                                     |
| Yes, each RCU callbacks must wait for a grace period to complete, but |
| it might well be partly (or even completely) finished waiting by the  |
| time rcu_barrier() is invoked. In that case, rcu_barrier()            |
| need only wait for the remaining portion of the grace period to       |
| elapse. So even if there are quite a few callbacks posted,            |
| rcu_barrier() might well return quite quickly.                        |
|                                                                       |
| So if you need to wait for a grace period as well as for all          |
| pre-existing callbacks, you will need to invoke both                  |
| synchronize_rcu() and rcu_barrier(). If latency is a concern,         |
| you can always use workqueues to invoke them concurrently.            |
+-----------------------------------------------------------------------+

#### 鐑彃鎷?CPU


Linux 鍐呮牳鏀寔 CPU 鐑彃鎷旓紝杩欐剰鍛崇潃 CPU 鍙互鏉ユ潵鍘诲幓銆傚綋鐒讹紝浠庣绾?CPU 浣跨敤浠讳綍 RCU API 鎴愬憳閮芥槸闈炴硶鐨勶紝渚嬪鏄?`SRCU <Sleepable RCU_>`__ 璇荤涓寸晫鍖恒€傝繖涓€闇€姹備粠 DYNIX/ptx 鐨勭涓€澶╄捣灏卞瓨鍦紝浣嗗彟涓€鏂归潰锛孡inux 鍐呮牳鐨?CPU 鐑彃鎷斿疄鐜扳€滃緢鏈夋剰鎬濃€濄€?

Linux 鍐呮牳鐨?CPU 鐑彃鎷斿疄鐜板甫鏈夐€氱煡鍣紙notifier锛夛紝鐢ㄤ簬璁╁悇涓唴鏍稿瓙绯荤粺锛堝寘鎷?RCU锛夊缁欏畾鐨?CPU 鐑彃鎷旀搷浣滃仛鍑烘伆褰撳搷搴斻€傚ぇ澶氭暟 RCU 鎿嶄綔閮藉彲浠ヤ粠 CPU 鐑彃鎷旈€氱煡鍣ㄤ腑璋冪敤锛岀敋鑷冲寘鎷儚 synchronize_rcu() 鍜?synchronize_rcu_expedited() 杩欐牱鐨勫悓姝ュ闄愭湡鎿嶄綔銆傜劧鑰岋紝杩欎簺鍚屾鎿嶄綔纭疄浼氶樆濉烇紝鍥犳涓嶈兘浠庨€氳繃 stop_machine() 鎵ц鐨勯€氱煡鍣ㄤ腑璋冪敤锛屽叿浣撴潵璇村氨鏄偅浜涘浜?`CPUHP_AP_OFFLINE` 鍜?`CPUHP_AP_ONLINE` 鐘舵€佷箣闂寸殑閫氱煡鍣ㄣ€?

姝ゅ锛屽儚 rcu_barrier() 杩欐牱鐨勨€滅瓑寰呮墍鏈夊洖璋冣€濇搷浣滀笉鑳戒粠浠讳綍 CPU 鐑彃鎷旈€氱煡鍣ㄤ腑璋冪敤銆傝繖涓€闄愬埗鏄洜涓哄湪 CPU 鐑彃鎷旀搷浣滅殑鏌愪簺闃舵锛岀绾跨殑閭ｄ釜 CPU 鐨勫洖璋冨湪 CPU 鐑彃鎷旀搷浣滅粨鏉熶箣鍓嶄笉浼氳璋冪敤锛岃繖鍚屾牱鍙兘瀵艰嚧姝婚攣銆傝€屼笖锛宺cu_barrier() 鍦ㄥ叾鎵ц鏈熼棿浼氶樆濉?CPU 鐑彃鎷旀搷浣滐紝褰撲粠 CPU 鐑彃鎷旈€氱煡鍣ㄤ腑璋冪敤鏃讹紝杩欎細瀵艰嚧鍙︿竴绉嶆閿併€?

鏈€鍚庯紝RCU 蹇呴』閬垮厤鐢变簬鐑彃鎷斻€佸畾鏃跺櫒鍜屽闄愭湡澶勭悊涔嬮棿鐨勪氦浜掕€屽鑷寸殑姝婚攣銆傚畠閫氳繃瀵硅嚜宸辩殑閭ｅ璐︽湰杩涜缁存姢鏉ュ仛鍒拌繖涓€鐐癸紝杩欎簺璐︽湰澶嶅埗浜嗛泦涓淮鎶ょ殑 `cpu_online_mask`锛屽苟涓斿湪 CPU 绂荤嚎鏃舵樉寮忓湴鎶ュ憡闈欐鐘舵€併€傝繖绉嶅闈欐鐘舵€佺殑鏄惧紡鎶ュ憡锛岄伩鍏嶄簡寮哄埗闈欐鐘舵€佸惊鐜紙FQS锛夊幓涓虹绾?CPU 鎶ュ憡闈欐鐘舵€佺殑浠讳綍闇€瑕併€備笉杩囷紝浣滀负涓€绉嶈皟璇曟墜娈碉紝濡傛灉绂荤嚎 CPU 鎶?RCU 瀹介檺鏈熼樆濉炲お涔咃紝FQS 寰幆纭疄浼氬彂鍑?splat銆?

涓€涓绾?CPU 鐨勯潤姝㈢姸鎬佷細閫氳繃浠ヤ笅涓ょ鏂瑰紡涔嬩竴琚姤鍛婏細

1. 褰撹 CPU 閫氳繃 RCU 鐨勭儹鎻掓嫈閫氱煡鍣ㄧ绾挎椂锛坮cutree_report_cpu_dead()锛夈€?
2. 褰撳闄愭湡鍒濆鍖栵紙rcu_gp_init()锛夋娴嬪埌涓?CPU 绂荤嚎銆佹垨鑰呬笌涓€涓湪鍙跺瓙 `rcu_node` 缁撴瀯锛堝叾鎵€鏈?CPU 閮藉凡绂荤嚎锛変笂瑙ｉ櫎闃诲鐨勪换鍔′箣闂寸殑绔炴€佹椂銆?

CPU 涓婄嚎璺緞锛坮cutree_report_cpu_starting()锛夊簲褰撴案杩滀笉闇€瑕佷负绂荤嚎 CPU 鎶ュ憡闈欐鐘舵€併€備笉杩囷紝浣滀负涓€绉嶈皟璇曟墜娈碉紝濡傛灉灏氭湭涓鸿 CPU 鎶ュ憡闈欐鐘舵€侊紝瀹冪‘瀹炰細鍙戝嚭涓€涓鍛娿€?

鍦ㄦ鏌?淇敼 RCU 鐨勭儹鎻掓嫈璐︽湰鏈熼棿锛屼細鎸佹湁鐩稿簲 CPU 鐨勫彾瀛愯妭鐐归攣銆傝繖閬垮厤浜?RCU 鐨勭儹鎻掓嫈閫氱煡鍣ㄩ挬瀛愩€佸闄愭湡鍒濆鍖栦唬鐮佸拰 FQS 寰幆涔嬮棿鐨勭珵鎬佹潯浠讹紝瀹冧滑閮藉紩鐢ㄦ垨淇敼杩欏璐︽湰銆?

娉ㄦ剰锛屽闄愭湡鍒濆鍖栵紙rcu_gp_init()锛夊繀椤讳粩缁嗗湴涓?CPU 鐑彃鎷旀壂鎻忎笌瀹介檺鏈熺姸鎬佸彉鏇存帓搴忋€備緥濡傦紝濡傛灉 rcu_seq_start() 鍏堝彂鐢燂紝涓嬮潰鐨勭珵鎬佸氨鍙兘鍦?rcu_gp_init() 涓彂鐢?
```

   CPU0 (rcu_gp_init)                   CPU1                          CPU2
   ---------------------                ----                          ----
   // Hotplug scan first (WRONG ORDER)
   rcu_for_each_leaf_node(rnp) {
       rnp->qsmaskinit = rnp->qsmaskinitnext;
   }
                                        rcutree_report_cpu_starting()
                                            rnp->qsmaskinitnext |= mask;
                                        rcu_read_lock()
                                        r0 = *X;
                                                                      r1 = *X;
                                                                      X = NULL;
                                                                      cookie = get_state_synchronize_rcu();
                                                                      // cookie = 8 (future GP)
   rcu_seq_start(&rcu_state.gp_seq);
   // gp_seq = 5

   // CPU1 now invisible to this GP!
   rcu_for_each_node_breadth_first() {
       rnp->qsmask = rnp->qsmaskinit;
       // CPU1 not included!
   }

   // GP completes without CPU1
   rcu_seq_end(&rcu_state.gp_seq);
   // gp_seq = 8
                                                                      poll_state_synchronize_rcu(cookie);
                                                                      // Returns true!
                                                                      kfree(r1);
                                        r2 = *r0; // USE-AFTER-FREE!

```

閫氳繃鍏堥€掑 `gp_seq`锛屽氨鑳戒繚璇?CPU1 鐨?RCU 璇荤涓寸晫鍖轰笉浼氳 CPU2 婕忔帀銆?

##### 绂荤嚎 CPU 鐨勫苟鍙戦潤姝㈢姸鎬佹姤鍛?


RCU 蹇呴』纭繚绂荤嚎鐨?CPU 鎶ュ憡闈欐鐘舵€侊紝浠ラ伩鍏嶉樆濉炲闄愭湡銆傝繖闇€瑕佷粩缁嗙殑鍚屾鏉ュ鐞嗙珵鎬佹潯浠?

##### 瀵艰嚧绂荤嚎 CPU 鎸傝捣 GP 鐨勭珵鎬佹潯浠?


CPU 绂荤嚎涓庢柊 GP 鍒濆鍖栵紙gp_init()锛変箣闂村彲鑳藉彂鐢熺珵鎬侊紝鍥犱负 rcutree_report_cpu_dead() 涓殑 rcu_report_qs_rnp() 蹇呴』涓存椂
```

   CPU1 (going offline)                 CPU0 (GP kthread)
   --------------------                 -----------------
   rcutree_report_cpu_dead()
     rcu_report_qs_rnp()
       // Must release rnp->lock to wake GP kthread
       raw_spin_unlock_irqrestore_rcu_node()
                                        // Wakes up and starts new GP
                                        rcu_gp_init()
                                          // First loop:
                                          copies qsmaskinitnext->qsmaskinit
                                          // CPU1 still in qsmaskinitnext!

                                          // Second loop:
                                          rnp->qsmask = rnp->qsmaskinit
                                          mask = rnp->qsmask & ~rnp->qsmaskinitnext
                                          // mask is 0! CPU1 still in both masks
       // Reacquire lock (but too late)
     rnp->qsmaskinitnext &= ~mask       // Finally clears bit

```

濡傛灉娌℃湁 `ofl_lock`锛屾柊鐨勫闄愭湡灏变細鍖呭惈绂荤嚎鐨?CPU锛屽苟姘歌繙绛夊緟瀹冪殑闈欐鐘舵€侊紝浠庤€屽鑷?GP 鎸傝捣銆?

##### 浣跨敤 ofl_lock 鐨勮В鍐虫柟妗?


`ofl_lock`锛堢绾块攣锛夐樆姝?rcu_gp_init() 鍦ㄤ互涓嬫儏鍐垫湡闂磋繍琛?
```

   CPU0 (rcu_gp_init)                   CPU1 (rcutree_report_cpu_dead)
   ------------------                   ------------------------------
   rcu_for_each_leaf_node(rnp) {
       arch_spin_lock(&ofl_lock) -----> arch_spin_lock(&ofl_lock) [BLOCKED]

       // Safe: CPU1 can't interfere
       rnp->qsmaskinit = rnp->qsmaskinitnext

       arch_spin_unlock(&ofl_lock) ---> // Now CPU1 can proceed
   }                                    // But snapshot already taken

```

##### 瀵艰嚧 rcu_gp_init() 涓?GP 鎸傝捣鐨勫彟涓€绉嶇珵鎬侊細涓虹幇宸茬绾跨殑 CPU 鎶ュ憡 QS


鍦ㄧ涓€涓惊鐜鍦ㄧ嚎 CPU 鍙栦簡鍘熷瓙蹇収涔嬪悗锛堝涓婃墍绀猴級锛宺cu_gp_init() 涓殑绗簩涓惊鐜細妫€娴嬪湪閲婃斁 `ofl_lock` 涓庤幏鍙栨瘡鑺傜偣 `rnp->lock` 涔嬮棿绂荤嚎鐨?CPU銆?
杩欎竴妫€娴嬭嚦鍏抽噸瑕侊紝鍥犱负锛?

1. 璇?CPU 鍙兘鍦ㄥ揩鐓т箣鍚庛€佺浜屼釜寰幆涔嬪墠绂荤嚎浜?
2. 绂荤嚎鐨?CPU 濡傛灉宸茬粡鈥滄浜♀€濓紝灏辨棤娉曟姤鍛婂畠鑷繁鐨?QS
3. 娌℃湁杩欎竴妫€娴嬶紝瀹介檺鏈熷氨浼氭案杩滅瓑寰呴偅浜涚幇宸茬绾跨殑 CPU
```

   rcu_for_each_node_breadth_first(rnp) {
       raw_spin_lock_irqsave_rcu_node(rnp, flags);
       rnp->qsmask = rnp->qsmaskinit;  // Apply the snapshot

       // Detect CPUs offline after snapshot
       mask = rnp->qsmask & ~rnp->qsmaskinitnext;

       if (mask && rcu_is_leaf_node(rnp))
           rcu_report_qs_rnp(mask, ...)  // Report QS for offline CPUs
   }

```

杩欑鏂规硶淇濊瘉浜嗗師瀛愭€э細瀵圭绾?CPU 鐨勯潤姝㈢姸鎬佹姤鍛婅涔堝彂鐢熷湪 rcu_gp_init()锛堢浜屼釜寰幆锛変腑锛岃涔堝彂鐢熷湪 rcutree_report_cpu_dead() 涓紝姘歌繙涓嶄細涓よ€呴兘鍋氾紝涔熸案杩滀笉浼氫袱鑰呴兘涓嶅仛銆傛暣涓簭鍒楁湡闂存寔鏈夌殑 `rnp->lock` 闃叉浜嗙珵鎬佲€斺€攔cutree_report_cpu_dead() 鍦ㄦ竻闄?`qsmaskinitnext` 鏃朵篃浼氳幏鍙栬繖鎶婇攣锛屼粠鑰岀‘淇濅簰鏂ャ€?

#### 璋冨害鍣ㄤ笌 RCU


RCU 浣跨敤浜?kthread锛屽苟涓斿繀椤婚伩鍏嶈繖浜?kthread 杩囧害绱Н CPU 鏃堕棿銆傝繖涓€闇€姹傚苟涓嶄护浜烘剰澶栵紝浣?RCU 鍦ㄦ瀯寤烘椂甯︽湁 `CONFIG_NO_HZ_FULL=y`銆佸苟杩愯涓婁笅鏂囧垏鎹㈢箒閲嶇殑宸ヤ綔璐熻浇鏃惰繚鍙嶄簡瀹冿紝杩?`did come as a surprise [PDF] <http://www.rdrop.com/users/paulmck/scalability/paper/BareMetal.2015.01.15b.pdf>`__銆俁CU 鍦ㄦ弧瓒宠繖涓€闇€姹傛柟闈㈠凡缁忓彇寰椾簡鑹ソ杩涘睍锛屽嵆渚垮涓婁笅鏂囧垏鎹㈢箒閲嶇殑 `CONFIG_NO_HZ_FULL=y` 宸ヤ綔璐熻浇涔熸槸濡傛锛屼絾浠嶆湁杩涗竴姝ユ敼杩涚殑绌洪棿銆?

涓嶅啀鏈変换浣曠姝㈠湪鏌愪釜 rcu_read_unlock() 鏈熼棿鎸佹湁璋冨害鍣ㄧ殑杩愯闃熷垪閿佹垨浼樺厛绾х户鎵胯嚜鏃嬮攣锛屽嵆浣垮湪鐩稿簲 RCU 璇荤涓寸晫鍖哄唴閮ㄧ殑鏌愬鍚敤浜嗕腑鏂拰鎶㈠崰涔熸槸濡傛銆傚洜姝わ紝鐜板湪瀹屽叏鍚堟硶鍦板彲浠ュ湪鍚敤鎶㈠崰鐨勬儏鍐典笅鎵ц rcu_read_lock()銆佽幏鍙栧叾涓竴鎶婅皟搴﹀櫒閿侊紝骞跺湪涓庝箣鍖归厤鐨?rcu_read_unlock() 鏈熼棿鎸佹湁璇ラ攣銆?

绫讳技鍦帮紝RCU 椋庢牸鐨勬暣鍚堝凡缁忔秷闄や簡瀵硅礋鍚戝祵濂楃殑闇€姹傘€傜鐢ㄤ腑鏂殑浠ｇ爜鍖哄煙闅愬紡鍦板厖褰?RCU 璇荤涓寸晫鍖鸿繖涓€浜嬪疄锛岄伩鍏嶄簡鏃╂湡閭ｄ簺浼氬洜涓柇澶勭悊绋嬪簭浣跨敤 RCU 鑰屽鑷寸牬鍧忔€ч€掑綊鐨勯棶棰樸€?

#### 璺熻釜涓?RCU


鍙互鍦?RCU 浠ｇ爜涓婁娇鐢ㄨ窡韪紝浣嗚窡韪湰韬娇鐢ㄤ簡 RCU銆傚洜姝わ紝鎻愪緵浜?rcu_dereference_raw_check() 渚涜窡韪娇鐢紝瀹冮伩鍏嶄簡鍘熸湰鍙兘鍙戠敓鐨勭牬鍧忔€ч€掑綊銆傝繖涓?API 鍦ㄦ煇浜涗綋绯荤粨鏋勭殑铏氭嫙鍖栦腑涔熻浣跨敤锛岄偅閲?RCU 璇昏€呰繍琛屽湪鏃犳硶浣跨敤璺熻釜鐨勭幆澧冧腑銆傝窡韪殑寮€鍙戣€呮棦瀹氫綅鍒颁簡杩欎竴闇€姹傦紝涔熸彁渚涗簡鎵€闇€鐨勪慨澶嶏紝鎵€浠ヨ繖涓剰澶栭渶姹傜殑浠ｄ环鐩稿杈冨皬銆?

#### 瀵圭敤鎴峰唴瀛樼殑璁块棶涓?RCU


鍐呮牳闇€瑕佽闂敤鎴风┖闂村唴瀛橈紝渚嬪锛岃闂敱绯荤粺璋冪敤鍙傛暟鎵€寮曠敤鐨勬暟鎹€俫et_user() 瀹忓仛杩欎欢浜嬨€?

鐒惰€岋紝鐢ㄦ埛绌洪棿鍐呭瓨寰堝彲鑳借鎹㈠嚭椤碉紝杩欐剰鍛崇潃 get_user() 寰堝彲鑳戒細鍙戠敓椤甸敊璇紝浠庤€屽湪绛夊緟鐢辨浜х敓鐨?I/O 瀹屾垚鏃堕樆濉炪€傚鏋滅紪璇戝櫒鎶?get_user() 鐨勮皟鐢ㄩ噸鎺掕繘涓€涓?RCU 璇荤涓寸晫鍖猴紝閭ｅ皢鏄潪甯哥碂绯曠殑浜嬫儏銆?

渚嬪锛屽亣璁炬簮浠ｇ爜鐪嬭捣鏉ュ儚杩欐牱锛?

```

       1 rcu_read_lock();
       2 p = rcu_dereference(gp);
       3 v = p->value;
       4 rcu_read_unlock();
       5 get_user(user_v, user_p);
       6 do_something_with(v, user_v);

```

缁濅笉鑳藉厑璁哥紪璇戝櫒鎶婅繖娈垫簮浠ｇ爜鍙樻崲鎴愪笅闈㈣繖鏍凤細

```

       1 rcu_read_lock();
       2 p = rcu_dereference(gp);
       3 get_user(user_v, user_p); // BUG: POSSIBLE PAGE FAULT!!!
       4 v = p->value;
       5 rcu_read_unlock();
       6 do_something_with(v, user_v);

```

濡傛灉缂栬瘧鍣ㄧ湡鐨勫湪 `CONFIG_PREEMPTION=n` 鍐呮牳鏋勫缓涓仛浜嗚繖绉嶅彉鎹紝骞朵笖濡傛灉 get_user() 纭疄鍙戠敓浜嗛〉閿欒锛岀粨鏋滃氨浼氭槸鍦ㄤ竴涓?RCU 璇荤涓寸晫鍖轰腑闂村嚭鐜颁竴涓潤姝㈢姸鎬併€傝繖涓敊浣嶇殑闈欐鐘舵€佸彲鑳藉鑷寸 4 琛屾垚涓轰竴涓噴鏀惧悗浣跨敤锛坲se-after-free锛夎闂紝杩欏彲鑳藉浣犵殑鍐呮牳鐨勭簿绠楃粺璁″緢涓嶅埄銆備篃鍙互鐢ㄦ妸 get_user() 璋冪敤鏀惧湪 rcu_read_lock() 涔嬪墠鐨勬儏褰㈡瀯閫犲嚭绫讳技鐨勪緥瀛愩€?

涓嶅垢鐨勬槸锛実et_user() 娌℃湁浠讳綍鐗瑰畾鐨勯『搴忕壒鎬э紝骞朵笖鍦ㄦ煇浜涗綋绯荤粨鏋勪笂锛屽簳灞傜殑 `asm` 鐢氳嚦娌℃湁琚爣璁颁负 `volatile`銆傝€屽嵆渚垮畠琚爣璁颁负 `volatile`锛屼笂闈㈤偅涓 `p->value` 鐨勮闂篃涓嶆槸 volatile 鐨勶紝鎵€浠ョ紪璇戝櫒娌℃湁浠讳綍鐞嗙敱鎶婇偅涓ゆ璁块棶淇濇寔鏈夊簭銆?

鍥犳锛宺cu_read_lock() 鍜?rcu_read_unlock() 鐨?Linux 鍐呮牳瀹氫箟蹇呴』鍏呭綋缂栬瘧鍣ㄥ睆闅滐紝鑷冲皯瀵逛竴缁勫祵濂?RCU 璇荤涓寸晫鍖轰腑鏈€澶栧眰鐨?rcu_read_lock() 鍜?rcu_read_unlock() 瀹炰緥鑰岃█濡傛銆?

#### 鑳芥晥


鎵撴柇绌洪棽 CPU 琚涓烘槸涓嶅悎绀句細鍏痉鐨勶紝灏ゅ叾鏄浜庨偅浜涗娇鐢ㄧ數姹犱緵鐢电殑宓屽叆寮忕郴缁熺殑浜鸿€岃█銆俁CU 鍥犳閫氳繃妫€娴嬪摢浜?CPU 绌洪棽锛堝寘鎷窡韪偅浜涗粠绌洪棽涓鎵撴柇鐨?CPU锛夋潵鑺傜渷鑳借€椼€傝繖鏄兘鏁堥渶姹傜殑涓€澶ч儴鍒嗭紝鎵€浠ユ垜鏄€氳繃涓€閫氭劋鎬掔殑鐢佃瘽鎵嶄簡瑙ｅ埌瀹冪殑銆?

鍥犱负 RCU 閬垮厤鎵撴柇绌洪棽 CPU锛屾墍浠ュ湪绌洪棽 CPU 涓婃墽琛?RCU 璇荤涓寸晫鍖烘槸闈炴硶鐨勩€傦紙濡傛灉浣犲皾璇曡繖鏍峰仛锛岀敤 `CONFIG_PROVE_RCU=y` 鏋勫缓鐨勫唴鏍镐細鍙戝嚭 splat銆傦級

鎵撴柇杩愯鍦ㄧ敤鎴锋€佺殑 `nohz_full` CPU 鍚屾牱琚涓烘槸涓嶅悎绀句細鍏痉鐨勩€俁CU 鍥犳蹇呴』璺熻釜 `nohz_full` 鐨勭敤鎴锋€佹墽琛屻€俁CU 鍥犳蹇呴』鑳藉鍦ㄤ袱涓椂闂寸偣涓婇噰鏍风姸鎬侊紝骞朵笖鑳藉鍒ゆ柇鍏朵粬鏌愪釜 CPU 鏄惁鏇捐姳璐逛换浣曟椂闂村浜庣┖闂插拰/鎴栨墽琛屼簬鐢ㄦ埛鎬併€?

杩欎簺鑳芥晥闇€姹傝璇佹槑鐩稿綋闅句互鐞嗚В鍜屾弧瓒筹紝渚嬪锛孯CU 鐨勮兘鏁堜唬鐮佸凡缁忚褰诲簳閲嶅啓杩囦笉姝簲娆★紝鍏朵腑鏈€鍚庝竴娆＄粓浜庤兘澶熷湪鐪熷疄纭欢涓婃紨绀哄嚭 `real energy savings running on real hardware [PDF] <http://www.rdrop.com/users/paulmck/realtime/paper/AMPenergy.2013.04.19a.pdf>`__銆傚鍓嶆墍杩帮紝鎴戞槸閫氳繃鎰ゆ€掔殑鐢佃瘽浜嗚В鍒板叾涓澶氶渶姹傜殑锛氬湪 Linux 鍐呮牳閭欢鍒楄〃涓婂鎴戝彂鐏紝鏄剧劧涓嶈冻浠ュ畬鍏ㄥ娉勪粬浠 RCU 鑳芥晥缂洪櫡鐨勬€掔伀锛?

#### 璋冨害鏃堕挓涓柇涓?RCU


鍐呮牳鍦ㄥ唴鏍稿唴闈炵┖闂叉墽琛屻€佺敤鎴锋€佹墽琛屽拰绌洪棽寰幆涔嬮棿杞崲銆傚彇鍐充簬鍐呮牳閰嶇疆锛孯CU 瀵硅繖浜涚姸鎬佺殑澶勭悊鏂瑰紡涓嶅悓锛?

+-----------------+------------------+------------------+-----------------+
| `HZ` Kconfig    | 鍐呮牳涓?          | 鐢ㄦ埛鎬?          | 绌洪棽            |
+=================+==================+==================+=================+
| `HZ_PERIODIC` | 鍙互渚濊禆         | 鍙互渚濊禆         | 鍙互渚濊禆        |
|                 | 璋冨害鏃堕挓         | 璋冨害鏃堕挓         | RCU 鐨?         |
|                 | 涓柇銆?          | 涓柇鍙婂叾         | dyntick-idle    |
|                 |                  | 瀵规潵鑷敤鎴锋€佺殑   | 妫€娴嬨€?         |
|                 |                  | 涓柇鐨勬娴嬨€?    |                 |
+-----------------+------------------+------------------+-----------------+
| `NO_HZ_IDLE`  | 鍙互渚濊禆         | 鍙互渚濊禆         | 鍙互渚濊禆        |
|                 | 璋冨害鏃堕挓         | 璋冨害鏃堕挓         | RCU 鐨?         |
|                 | 涓柇銆?          | 涓柇鍙婂叾         | dyntick-idle    |
|                 |                  | 瀵规潵鑷敤鎴锋€佺殑   | 妫€娴嬨€?         |
|                 |                  | 涓柇鐨勬娴嬨€?    |                 |
+-----------------+------------------+------------------+-----------------+
| `NO_HZ_FULL`  | 鍙兘鏈夋椂         | 鍙互渚濊禆         | 鍙互渚濊禆        |
|                 | 渚濊禆璋冨害鏃堕挓     | RCU 鐨?          | RCU 鐨?         |
|                 | 涓柇銆傚湪鍏朵粬     | dyntick-idle     | dyntick-idle    |
|                 | 鎯呭喌涓嬶紝鏈夊繀瑕?  | 妫€娴嬨€?          | 妫€娴嬨€?         |
|                 | 闄愬埗鍐呮牳鎵ц     |                  |                 |
|                 | 鏃堕棿鍜?鎴栦娇鐢?   |                  |                 |
|                 | IPI銆?           |                  |                 |
+-----------------+------------------+------------------+-----------------+

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| Why can't `NO_HZ_FULL` in-kernel execution rely on the              |  
| scheduling-clock interrupt, just like `HZ_PERIODIC` and             |  
| `NO_HZ_IDLE` do?                                                    |  
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| Because, as a performance optimization, `NO_HZ_FULL` does not       |  
| necessarily re-enable the scheduling-clock interrupt on entry to each |
| and every system call.                                                |
+-----------------------------------------------------------------------+

鐒惰€岋紝RCU 蹇呴』琚彲闈犲湴鍛婄煡浠绘剰缁欏畾 CPU 褰撳墠鏄惁澶勪簬绌洪棽寰幆锛屽苟涓斿浜?`NO_HZ_FULL`锛岃繕瑕佽鍛婄煡璇?CPU 鏄惁姝ｅ湪鎵ц浜庣敤鎴锋€侊紝濡?`earlier <Energy Efficiency_>`__ 鎵€璁ㄨ銆傚畠杩橀渶瑕佸湪 RCU 闇€瑕佽皟搴︽椂閽熶腑鏂椂琚惎鐢細

#. 濡傛灉鏌愪釜 CPU 瑕佷箞绌洪棽銆佽涔堟墽琛屼簬鐢ㄦ埛鎬侊紝鑰?RCU 璁や负瀹冩槸闈炵┖闂茬殑锛岄偅涔堣皟搴︽椂閽熸淮绛旀渶濂芥鍦ㄨ繍琛屻€傚惁鍒欙紝浣犱細寰楀埌 RCU CPU 鍋滈】璀﹀憡銆傛垨鑰呮渶濂界殑鎯呭喌涓嬶紝鏄潪甯搁暱锛?1 绉掞級鐨勫闄愭湡锛屼即闅忕潃涓€涓棤鎰忎箟鐨?IPI 涓嶆椂鍞ら啋璇?CPU銆?
#. 濡傛灉鏌愪釜 CPU 澶勪簬鍐呮牳涓細鎵ц RCU 璇荤涓寸晫鍖虹殑閮ㄥ垎锛岃€?RCU 璁や负璇?CPU 绌洪棽锛屼綘浼氬緱鍒伴殢鏈虹殑鍐呭瓨鎹熷潖銆?*涓嶈杩欐牱鍋氾紒锛侊紒** 杩欐鏄敤 lockdep 鍋氭祴璇曠殑涓€涓悊鐢憋紝瀹冧細鎶辨€ㄨ繖绫讳簨鎯呫€?
#. 濡傛灉鏌愪釜 CPU 澶勪簬鍐呮牳涓粷瀵广€佽偗瀹氥€佺粷涓嶆墽琛屼换浣?RCU 璇荤涓寸晫鍖虹殑閮ㄥ垎锛岃€?RCU 璁や负璇?CPU 绌洪棽锛屽垯娌￠棶棰樸€傛煇浜涗綋绯荤粨鏋勬妸杩欑涓滆タ鐢ㄤ簬杞婚噺绾у紓甯稿鐞嗙▼搴忥紝浠庤€屽彲浠ラ伩鍏嶅湪寮傚父杩涘叆鍜岄€€鍑烘椂鍒嗗埆鎵垮彈 ct_irq_enter() 鍜?ct_irq_exit() 鐨勫紑閿€銆傛湁浜涜蛋寰楁洿杩滐紝杩?irq_enter() 鍜?irq_exit() 鐨勬暣浣撻兘閬垮厤浜嗐€?
   鍙闈炲父纭畾浣犵敤 `CONFIG_PROVE_RCU=y` 杩愯浜嗛儴鍒嗘祴璇曪紝浠ラ槻浣犵殑鏌愭潯浠ｇ爜璺緞鍏跺疄鏄湪寮€鐜╃瑧鍦拌鑷繁涓嶆墽琛?RCU 璇荤涓寸晫鍖恒€?
#. 濡傛灉鏌愪釜 CPU 鍦ㄥ唴鏍镐腑鎵ц銆佽皟搴︽椂閽熶腑鏂绂佺敤銆佷笖 RCU 璁や负璇?CPU 闈炵┖闂诧紝骞朵笖濡傛灉璇?CPU 姣忛殧鍑犱釜 jiffies锛堜粠 RCU 瑙掑害鐪嬶級灏辫繘鍏ョ┖闂诧紝鍒欐病闂銆傜┖闂叉湡涔嬮棿鍋跺皵鍑虹幇闀胯揪涓€绉掑乏鍙崇殑闂撮殧閫氬父鏄彲浠ョ殑銆?
   濡傛灉闂撮殧鍙樺緱澶暱锛屼綘浼氬緱鍒?RCU CPU 鍋滈】璀﹀憡銆?
#. 濡傛灉鏌愪釜 CPU 瑕佷箞绌洪棽銆佽涔堟墽琛屼簬鐢ㄦ埛鎬侊紝鑰?RCU 璁や负瀹冪┖闂诧紝鑷劧娌￠棶棰樸€?
#. 濡傛灉鏌愪釜 CPU 鍦ㄥ唴鏍镐腑鎵ц锛屽唴鏍镐唬鐮佽矾寰勪互鍚堢悊鐨勯鐜囩粡杩囬潤姝㈢姸鎬侊紙鏈€濂藉ぇ绾︽瘡鍑犱釜 jiffies 涓€娆★紝浣嗗伓灏斿欢浼稿埌涓€绉掑乏鍙抽€氬父涔熷彲浠ワ級锛屽苟涓旇皟搴︽椂閽熶腑鏂鍚敤锛岃嚜鐒舵病闂銆?
   濡傛灉杩炵画涓や釜闈欐鐘舵€佷箣闂寸殑闂撮殧鍙樺緱澶暱锛屼綘浼氬緱鍒?RCU CPU 鍋滈】璀﹀憡銆?

+-----------------------------------------------------------------------+
| **Quick Quiz**:                                                       |
+-----------------------------------------------------------------------+
| But what if my driver has a hardware interrupt handler that can run   |
| for many seconds? I cannot invoke schedule() from an hardware         |
| interrupt handler, after all!                                         |
+-----------------------------------------------------------------------+
| **Answer**:                                                           |
+-----------------------------------------------------------------------+
| One approach is to do `ct_irq_exit();ct_irq_enter();` every so      |  
| often. But given that long-running interrupt handlers can cause other |
| problems, not least for response time, shouldn't you work to keep     |
| your interrupt handler's runtime within reasonable bounds?            |
+-----------------------------------------------------------------------+

浣嗗彧瑕?RCU 琚纭憡鐭ュ唴鏍告€佹墽琛屻€佺敤鎴锋€佹墽琛屽拰绌洪棽涔嬮棿鐨勫唴鏍哥姸鎬佽浆鎹紝骞朵笖鍙璋冨害鏃堕挓涓柇鍦?RCU 闇€瑕佸畠鏃惰鍚敤锛屼綘灏卞彲浠ユ斁蹇冿紝浣犻亣鍒扮殑 bug 浼氬嚭鐜板湪 RCU 鐨勫叾浠栭儴鍒嗐€佹垨鑰呭唴鏍哥殑鍏朵粬閮ㄥ垎锛?

#### 鍐呭瓨鏁堢巼


灏界灏忓瀷鍐呭瓨鐨勯潪瀹炴椂绯荤粺鍙互绠€鍗曞湴浣跨敤 Tiny RCU锛屼絾浠ｇ爜澶у皬鍙槸鍐呭瓨鏁堢巼鐨勪竴涓柟闈€傚彟涓€涓柟闈㈡槸 call_rcu() 鍜?kfree_rcu() 鎵€浣跨敤鐨?`rcu_head` 缁撴瀯鐨勫ぇ灏忋€傚敖绠¤繖涓粨鏋勫彧鍖呭惈涓€瀵规寚閽堬紝浣嗗畠纭疄鍑虹幇鍦ㄨ澶?RCU 淇濇姢鐨勬暟鎹粨鏋勪腑锛屽寘鎷竴浜涘澶у皬鏁忔劅鐨勭粨鏋勩€俙page` 缁撴瀯灏辨槸涓€涓伆褰撶殑渚嬪瓙锛岃缁撴瀯鍐呴儴澶ч噺鍑虹幇 `union` 鍏抽敭瀛椾究璇佹槑浜嗚繖涓€鐐广€?

杩欑瀵瑰唴瀛樻晥鐜囩殑闇€姹傦紝鏄?RCU 浣跨敤鎵嬪伐鎵撻€犵殑鍗曞悜閾捐〃鏉ヨ窡韪偅浜涚瓑寰呭闄愭湡杩囧幓鐨?`rcu_head` 缁撴瀯鐨勪竴涓師鍥犮€傝繖涔熸槸 `rcu_head` 缁撴瀯涓嶅寘鍚皟璇曚俊鎭紙渚嬪璺熻釜鎻愪氦浜嗗畠浠殑 call_rcu() 鎴?kfree_rcu() 鐨勬枃浠跺拰琛岀殑瀛楁锛夌殑鍘熷洜銆傚敖绠¤繖绫讳俊鎭垨璁稿皢鏉ユ煇涓椂鍊欎細鍑虹幇鍦ㄤ粎鐢ㄤ簬璋冭瘯鐨勫唴鏍告瀯寤轰腑锛屼絾鎴嚦鐩墠锛宍->func` 瀛楁寰€寰€鑳芥彁渚涙墍闇€鐨勮皟璇曚俊鎭€?

鐒惰€岋紝鍦ㄦ煇浜涙儏鍐典笅锛屽鍐呭瓨鏁堢巼鐨勯渶姹傚鑷翠簡鏇存瀬绔殑鎺柦銆傚洖鍒?`page` 缁撴瀯锛宍rcu_head` 瀛楁涓庤澶氬叾浠栫粨鏋勫叡浜瓨鍌紝杩欎簺缁撴瀯鍦ㄨ椤电敓鍛藉懆鏈熶腑鐨勪笉鍚屾椂鍒昏浣跨敤銆備负浜嗘纭В鍐虫煇浜?`race conditions <https://lore.kernel.org/r/1439976106-137226-1-git-send-email-kirill.shutemov@linux.intel.com>`__锛孡inux 鍐呮牳鐨勫唴瀛樼鐞嗗瓙绯荤粺闇€瑕佹煇涓壒瀹氱殑浣嶅湪瀹介檺鏈熷鐞嗙殑鎵€鏈夐樁娈甸兘淇濇寔涓洪浂锛岃€岄偅涓綅鎭板ソ鏄犲皠鍒?`rcu_head` 缁撴瀯鐨?`->next` 瀛楁鐨勬渶浣庝綅銆傚彧瑕佷娇鐢?call_rcu()锛堣€屼笉鏄?kfree_rcu() 鎴栧皢鏉ユ煇澶╁彲鑳戒负鑳芥晥鐩殑鑰屽垱寤虹殑 call_rcu() 鐨勬煇涓€滄儼鎬р€濆彉浣擄級鏉ユ彁浜ゅ洖璋冿紝RCU 灏辨彁渚涜繖涓€淇濊瘉銆?

璇濊櫧濡傛锛岃繕鏄湁闄愬害鐨勩€俁CU 瑕佹眰 `rcu_head` 缁撴瀯鎸変袱瀛楄妭杈圭晫瀵归綈锛屾妸鏈榻愮殑 `rcu_head` 缁撴瀯浼犵粰 call_rcu() 绯诲垪鍑芥暟涓殑鏌愪竴涓細瀵艰嚧 splat銆傚洜姝ゅ湪鎵撳寘鍖呭惈 `rcu_head` 绫诲瀷瀛楁鐨勭粨鏋勬椂蹇呴』璋ㄦ厧銆備负浠€涔堜笉鏄洓瀛楄妭鐢氳嚦鍏瓧鑺傜殑瀵归綈瑕佹眰锛熷洜涓?m68k 浣撶郴缁撴瀯鍙彁渚涗袱瀛楄妭瀵归綈锛屽洜姝ゅ畠鍏呭綋浜嗗唴瀛樺榻愮殑鏈€灏忓叕鍒嗘瘝銆?

淇濈暀鎸囧悜 `rcu_head` 缁撴瀯鐨勬寚閽堢殑鏈€浣庝綅鐨勭悊鐢憋紝鏄负鈥滄儼鎬р€濆洖璋冪暀涓嬪ぇ闂紝杩欑被鍥炶皟鐨勮皟鐢ㄥ彲浠ヨ瀹夊叏鍦版帹杩熴€傛帹杩熻皟鐢ㄥ彲鑳芥湁娼滃湪鐨勮兘鏁堟敹鐩婏紝浣嗗墠鎻愭槸瀵逛簬鏌愪釜閲嶈宸ヤ綔璐熻浇锛岄潪鎯版€у洖璋冪殑閫熺巼鏄捐憲涓嬮檷銆傚湪姝や箣鍓嶏紝淇濈暀鏈€浣庝綅璁╄繖涓€夐」淇濇寔寮€鏀撅紝浠ュ灏嗘潵鏌愬ぉ鍙樺緱鏈夌敤銆?

#### 鎬ц兘銆佸彲鎵╁睍鎬с€佸搷搴旀椂闂翠笌鍙潬鎬?


鎵╁睍 `earlier discussion <Performance and Scalability_>`__锛孯CU 琚?Linux 鍐呮牳鐨勭綉缁溿€佸畨鍏ㄣ€佽櫄鎷熷寲鍜岃皟搴︾瓑鎬ц兘鍏抽敭浠ｇ爜璺緞涓殑鐑偣浠ｇ爜澶ч噺浣跨敤銆俁CU 鍥犳蹇呴』浣跨敤楂樻晥鐨勫疄鐜帮紝灏ゅ叾鏄湪鍏惰绔師璇腑銆備负姝わ紝濡傛灉鍙姠鍗?RCU 鐨?rcu_read_lock() 瀹炵幇鑳藉琚唴鑱斿氨濂戒簡锛岀劧鑰岋紝杩欐牱鍋氶渶瑕佽В鍐充笌 `task_struct` 缁撴瀯鐨?`#include` 闂銆?

Linux 鍐呮牳鏀寔澶氳揪 4096 涓?CPU 鐨勭‖浠堕厤缃紝杩欐剰鍛崇潃 RCU 蹇呴』鏋佸叿鍙墿灞曟€с€傚湪 RCU 瀹炵幇鍐呴儴锛屾秹鍙婇绻佽幏鍙栧叏灞€閿佹垨棰戠箒瀵瑰叏灞€鍙橀噺鍋氬師瀛愭搷浣滅殑绠楁硶鏄粷瀵规棤娉曞蹇嶇殑銆俁CU 鍥犳澶ч噺浣跨敤鍩轰簬 `rcu_node` 缁撴瀯鐨勭粍鍚堟爲銆俁CU 蹇呴』鑳藉瀹瑰繊鎵€鏈?CPU 鎸佺画鍦颁互浠绘剰缁勫悎璋冪敤 RCU 鐨勮繍琛屾椂鍘熻锛屼笖姣忔鎿嶄綔鐨勫紑閿€鏋佸皬銆備簨瀹炰笂锛屽湪璁稿鎯呭喌涓嬶紝璐熻浇鐨勫鍔犲繀椤?*闄嶄綆**姣忔鎿嶄綔鐨勫紑閿€锛宻ynchronize_rcu()銆乧all_rcu()銆乻ynchronize_rcu_expedited() 鍜?rcu_barrier() 鐨勬壒澶勭悊浼樺寲灏辨槸鏄庤瘉銆備綔涓轰竴鑸鍒欙紝RCU 蹇呴』娆ｇ劧鎺ュ彈 Linux 鍐呮牳鍏朵綑閮ㄥ垎鍐冲畾鎶涚粰瀹冪殑浠讳綍涓滆タ銆?

Linux 鍐呮牳琚敤浜庡疄鏃跺伐浣滆礋杞斤紝灏ゅ叾鏄笌 `-rt patchset <https://wiki.linuxfoundation.org/realtime/>`__ 缁撳悎浣跨敤鏃躲€傚疄鏃跺欢杩熷搷搴旈渶姹備娇寰楅偅绉嶅湪 RCU 璇荤涓寸晫鍖轰笂璺ㄥ尯绂佺敤鎶㈠崰鐨勪紶缁熷仛娉曞彉寰椾笉鍚堥€傘€傜敤 `CONFIG_PREEMPTION=y` 鏋勫缓鐨勫唴鏍稿洜姝や娇鐢ㄤ竴绉嶅厑璁?RCU 璇荤涓寸晫鍖鸿鎶㈠崰鐨?RCU 瀹炵幇銆傝繖涓€闇€姹傛槸鍦ㄧ敤鎴锋槑纭〃绀烘棭鏈熺殑涓€涓?`real-time patch <https://lwn.net/Articles/107930/>`__ 涓嶆弧瓒充粬浠殑闇€姹傘€佸苟缁撳悎 -rt patchset 鏋佹棭鏈熺増鏈亣鍒扮殑鏌愪簺 `RCU issues <https://lore.kernel.org/r/20050318031826.GA2693@us.ibm.com>`__ 涔嬪悗鎵嶆樉鐜板嚭鏉ョ殑銆?

姝ゅ锛孯CU 蹇呴』鍦ㄤ竴涓皬浜?100 寰鐨勫疄鏃跺欢杩熼绠楀唴灏嗗氨搴斾粯銆備簨瀹炰笂锛屽湪浣跨敤 -rt patchset 鐨勫皬鍨嬬郴缁熶笂锛?
Linux 鍐呮牳涓烘暣涓唴鏍革紙鍖呮嫭 RCU锛夋彁渚涗簹 20 寰鐨勫疄鏃跺欢杩熴€俁CU 鐨勫彲鎵╁睍鎬у拰寤惰繜鍥犳蹇呴』瓒充互婊¤冻杩欑被閰嶇疆銆備护鎴戞儕璁剁殑鏄紝浜?100 寰鐨勫疄鏃跺欢杩熼绠?`applies to even the largest systems [PDF] <http://www.rdrop.com/users/paulmck/realtime/paper/bigrt.2013.01.31a.LCA.pdf>`__锛屽苟涓斾竴鐩存兜鐩栧埌鎷ユ湁 4096 涓?CPU 鐨勭郴缁熴€傝繖涓€瀹炴椂闇€姹備績浣夸簡瀹介檺鏈?kthread 鐨勮癁鐢燂紝瀹冧篃绠€鍖栦簡鑻ュ共绔炴€佹潯浠剁殑澶勭悊銆?

RCU 蹇呴』閬垮厤闄嶄綆 CPU 瀵嗛泦鍨嬬嚎绋嬬殑瀹炴椂鍝嶅簲锛屾棤璁鸿繖浜涚嚎绋嬫墽琛屼簬鐢ㄦ埛鎬侊紙杩欐槸 `CONFIG_NO_HZ_FULL=y` 鐨勪竴涓敤渚嬶級杩樻槸鍦ㄥ唴鏍镐腑銆傝瘽铏藉姝わ紝鍐呮牳涓殑 CPU 瀵嗛泦鍨嬪惊鐜繀椤昏嚦灏戞瘡鍑犲崄姣鎵ц涓€娆?cond_resched()锛屼互閬垮厤鏀跺埌鏉ヨ嚜 RCU 鐨?IPI銆?

鏈€鍚庯紝RCU 浣滀负鍚屾鍘熻鐨勫湴浣嶆剰鍛崇潃锛屼换浣?RCU 鏁呴殰閮藉彲鑳藉鑷翠换鎰忕殑鍐呭瓨鎹熷潖锛岃€岃繖鍙兘鏋佸叾闅句互璋冭瘯銆傝繖鎰忓懗鐫€ RCU 蹇呴』鏋佸叾鍙潬锛岃繖鍦ㄥ疄璺典腑涔熸剰鍛崇潃 RCU 蹇呴』鏈変竴涓縺杩涚殑鍘嬪姏娴嬭瘯濂椾欢銆傝繖涓帇鍔涙祴璇曞浠跺彨鍋?`rcutorture`銆?

灏界瀵?`rcutorture` 鐨勯渶姹傚苟涓嶄护浜烘剰澶栵紝浣?Linux 鍐呮牳褰撳墠鏋侀珮鐨勬櫘鍙婂害姝ｅ湪甯︽潵鏈夎叮鈥斺€旀垨璁镐篃鏄墠鎵€鏈湁鈥斺€旂殑楠岃瘉鎸戞垬銆傝鐞嗚В杩欎竴鐐癸紝璇疯浣忥紝閴翠簬 Android 鏅鸿兘鎵嬫満銆佺敱 Linux 椹卞姩鐨勭數瑙嗗拰鏈嶅姟鍣紝褰撲粖姝ｅ湪杩愯鐨?Linux 鍐呮牳瀹炰緥杩滆繙瓒呰繃鍗佷嚎銆傞殢鐫€澹板悕鏄捐但鐨勭墿鑱旂綉鐨勫埌鏉ワ紝杩欎釜鏁板瓧棰勮浼氭€ュ墽澧炲姞銆?

鍋囪 RCU 鍚湁涓€涓钩鍧囨瘡涓€鐧句竾骞磋繍琛屾椂闀垮嚭鐜颁竴娆＄殑绔炴€佹潯浠躲€傚湪鏁翠釜瑁呮満閲忎笂锛岃繖涓?bug 澶х害姣忓ぉ浼氬彂鐢熶笁娆°€俁CU 褰撶劧鍙互韬茶棌鍦ㄧ‖浠堕敊璇巼鑳屽悗锛屾瘯绔熸病浜虹湡鐨勬寚鏈涜嚜宸辩殑鏅鸿兘鎵嬫満鑳界敤涓€鐧句竾骞淬€傜劧鑰岋紝浠讳綍浠庤繖涓兂娉曚腑鑾峰緱澶瀹夋叞鐨勪汉锛岄兘搴旇鑰冭檻杩欐牱涓€涓簨瀹烇細鍦ㄥぇ澶氭暟鍙告硶绠¤緰鍖猴紝瀵逛竴涓粰瀹氭満鍒讹紙鍏朵腑鍙兘鍖呮嫭 Linux 鍐呮牳锛夋垚鍔熺殑澶氬勾娴嬭瘯锛屽氨瓒充互婊¤冻鑻ュ共绫诲瀷鐨勫畨鍏ㄥ叧閿璇併€備簨瀹炰笂锛屾湁浼犺█璇?Linux 鍐呮牳宸茬粡鐢ㄤ簬瀹夊叏鍏抽敭鍨嬬殑鐢熶骇搴旂敤銆傛垜涓嶇煡閬撲綘鎬庝箞鎯筹紝浣嗗鏋?RCU 涓殑涓€涓?bug 瀹虫浜嗕汉锛屾垜浼氭劅鍒扮浉褰撶碂绯曘€傝繖涔熻鍙互瑙ｉ噴鎴戞渶杩戝楠岃瘉涓庣‘璁ょ殑鍏虫敞銆?

### 鍏朵粬 RCU 椋庢牸


RCU 鏇翠护浜烘儕璁剁殑浜嬫儏涔嬩竴鏄紝瀹冪幇鍦ㄨ嚦灏戞湁浜?*绉嶉鏍?*锛坒lavor锛夛紝鎴栫О API 瀹舵棌銆傛澶栵紝杩勪粖涓烘涓€鐩存槸鍞竴鐒︾偣鐨勪富椋庢牸鏈変袱绉嶄笉鍚岀殑瀹炵幇锛氫笉鍙姠鍗犵殑鍜屽彲鎶㈠崰鐨勩€傚叾浠栧洓绉嶉鏍煎垪鍦ㄤ笅闈紝姣忕鐨勯渶姹傚湪鍗曠嫭鐨勮妭涓弿杩般€?

#. `Bottom-Half Flavor (Historical)`_
#. `Sched Flavor (Historical)`_
#. `Sleepable RCU`_
#. `Tasks RCU`_
#. `Tasks Trace RCU`_

#### 搴曞崐閮ㄩ鏍硷紙鍘嗗彶锛?


RCU 鐨?RCU-bh 椋庢牸姝ゅ悗宸茬敤鍏朵粬 RCU 椋庢牸琛ㄨ揪锛屼綔涓哄皢涓夌椋庢牸鏁村悎涓哄崟涓€椋庢牸鐨勪竴閮ㄥ垎銆傝绔?API 淇濈暀浜嗕笅鏉ワ紝骞剁户缁鐢ㄨ蒋涓柇锛屼笖缁х画鐢?lockdep 璁拌处銆傚洜姝わ紝鏈妭涓殑澶ч儴鍒嗘潗鏂欎弗鏍兼潵璇存槸鍘嗗彶鎬ц川鐨勩€?

RCU 鐨勮蒋涓柇绂佺敤锛堝張绉扳€滃簳鍗婇儴鈥濓紝hence 缂╁啓涓衡€淿bh鈥濓級椋庢牸锛屾垨绉?**RCU-bh**锛岀敱 Dipankar Sarma 寮€鍙戯紝鐢ㄤ簬鎻愪緵涓€绉嶈兘澶熸壙鍙?Robert Olsson 鎵€鐮旂┒鐨勫熀浜庣綉缁滅殑鎷掔粷鏈嶅姟鏀诲嚮鐨?RCU 椋庢牸銆傝繖浜涙敾鍑荤粰绯荤粺鏂藉姞浜嗗姝や箣澶х殑缃戠粶璐熻浇锛屼互鑷充簬鏌愪簺 CPU 姘歌繙涓嶉€€鍑鸿蒋涓柇鎵ц锛岃€岃繖鍙嶈繃鏉ュ張闃绘浜嗛偅浜?CPU 鎵ц浠讳綍涓婁笅鏂囧垏鎹紝鍦ㄥ綋鏃剁殑 RCU 瀹炵幇涓紝杩欏氨闃绘浜嗗闄愭湡姘歌繙缁撴潫銆傜粨鏋滄槸鍐呭瓨鑰楀敖鍜岀郴缁熸寕璧枫€?

瑙ｅ喅鏂规鏄垱寤?RCU-bh锛屽畠鍦ㄨ嚜宸辩殑璇荤涓寸晫鍖轰笂鍋?local_bh_disable()锛屽苟涓旈櫎浜嗕笂涓嬫枃鍒囨崲銆佺┖闂层€佺敤鎴锋€佸拰绂荤嚎涔嬪锛岃繕鎶婁粠涓€绉嶈蒋涓柇澶勭悊鍒板彟涓€绉嶈蒋涓柇澶勭悊鐨勮浆鎹㈠綋浣滀竴涓潤姝㈢姸鎬併€傝繖鎰忓懗鐫€鍗充究鏌愪簺 CPU 鏃犻檺鏈熷湴鎵ц浜庤蒋涓柇涓紝RCU-bh 瀹介檺鏈熶篃鑳藉畬鎴愶紝浠庤€岃鍩轰簬 RCU-bh 鐨勭畻娉曡兘澶熸壙鍙楀熀浜庣綉缁滅殑鎷掔粷鏈嶅姟鏀诲嚮銆?

鍥犱负 rcu_read_lock_bh() 鍜?rcu_read_unlock_bh() 绂佺敤鍜岄噸鏂板惎鐢ㄨ蒋涓柇澶勭悊绋嬪簭锛屼换浣曞湪 RCU-bh 璇荤涓寸晫鍖烘湡闂村惎鍔ㄨ蒋涓柇澶勭悊绋嬪簭鐨勫皾璇曢兘浼氳鎺ㄨ繜銆傚湪杩欑鎯呭喌涓嬶紝rcu_read_unlock_bh() 浼氳皟鐢ㄨ蒋涓柇澶勭悊锛岃繖鍙兘闇€瑕佺浉褰撻暱鐨勬椂闂淬€傚綋鐒跺彲浠ヤ簤杈╄锛岃繖涓蒋涓柇寮€閿€搴斿綋褰掑睘浜?RCU-bh 璇荤涓寸晫鍖轰箣鍚庣殑浠ｇ爜锛岃€屼笉鏄?rcu_read_unlock_bh()锛屼絾浜嬪疄鏄紝澶у鏁版€ц兘鍒嗘瀽宸ュ叿鏃犳硶鍋氬埌杩欑绮剧粏鐨勫尯鍒嗐€備緥濡傦紝鍋囪涓€涓笁姣闀跨殑 RCU-bh 璇荤涓寸晫鍖哄湪缃戠粶璐熻浇娌夐噸鏃舵墽琛屻€傚湪閭ｄ笁姣鍐呮瀬鏈夊彲鑳戒細灏濊瘯璋冪敤鑷冲皯涓€涓蒋涓柇澶勭悊绋嬪簭锛屼絾浠讳綍姝ょ被璋冪敤閮戒細琚帹杩熷埌 rcu_read_unlock_bh() 鐨勬椂鍒汇€傝繖褰撶劧浼氳浜虹涓€鐪肩湅涓婂幓浠ヤ负鏄?rcu_read_unlock_bh() 鎵ц寰楅潪甯告參銆?

`RCU-bh API <https://lwn.net/Articles/609973/#RCU%20Per-Flavor%20API%20Table>`__ 鍖呮嫭 rcu_read_lock_bh()銆乺cu_read_unlock_bh()銆乺cu_dereference_bh()銆乺cu_dereference_bh_check() 鍜?rcu_read_lock_bh_held()銆傜劧鑰岋紝鏃х殑 RCU-bh 鏇存柊绔?API 鐜板湪宸茬粡娌℃湁浜嗭紝鍙栬€屼唬涔嬬殑鏄?synchronize_rcu()銆乻ynchronize_rcu_expedited()銆乧all_rcu() 鍜?rcu_barrier()銆傛澶栵紝浠讳綍绂佺敤搴曞崐閮ㄧ殑鎿嶄綔涔熼兘鏍囪浜嗕竴涓?RCU-bh 璇荤涓寸晫鍖猴紝鍖呮嫭 local_bh_disable() 鍜?local_bh_enable()銆乴ocal_irq_save() 鍜?local_irq_restore() 绛夌瓑銆?

#### 璋冨害椋庢牸锛堝巻鍙诧級


RCU 鐨?RCU-sched 椋庢牸姝ゅ悗宸茬敤鍏朵粬 RCU 椋庢牸琛ㄨ揪锛屼綔涓哄皢涓夌椋庢牸鏁村悎涓哄崟涓€椋庢牸鐨勪竴閮ㄥ垎銆傝绔?API 淇濈暀浜嗕笅鏉ワ紝骞剁户缁鐢ㄦ姠鍗狅紝涓旂户缁敱 lockdep 璁拌处銆傚洜姝わ紝鏈妭涓殑澶ч儴鍒嗘潗鏂欎弗鏍兼潵璇存槸鍘嗗彶鎬ц川鐨勩€?

鍦ㄥ彲鎶㈠崰 RCU 涔嬪墠锛岀瓑寰呬竴涓?RCU 瀹介檺鏈熻繕鏈変竴涓壇浣滅敤锛屽氨鏄篃浼氱瓑寰呮墍鏈夋棦瀛樼殑Interrupt鍜?NMI 澶勭悊绋嬪簭銆傜劧鑰岋紝瀛樺湪涓€浜涘悎娉曠殑鍙姠鍗?RCU 瀹炵幇骞朵笉鍏峰杩欎竴鎬ц川锛屽洜涓轰唬鐮佷腑鐐逛綅浜?RCU 璇荤涓寸晫鍖轰箣澶栫殑浠讳綍浣嶇疆閮藉彲浠ユ槸涓€涓潤姝㈢姸鎬併€傚洜姝わ紝鍒涘缓浜?**RCU-sched**锛屽畠閬靛惊鈥滅粡鍏糕€漅CU锛屽嵆涓€涓?RCU-sched 瀹介檺鏈熶細绛夊緟鏃㈠瓨鐨処nterrupt鍜?NMI 澶勭悊绋嬪簭銆傚湪鐢?`CONFIG_PREEMPTION=n` 鏋勫缓鐨勫唴鏍镐腑锛孯CU 鍜?RCU-sched 鐨?API 鏈夌潃鐩稿悓鐨勫疄鐜帮紝鑰岀敤 `CONFIG_PREEMPTION=y` 鏋勫缓鐨勫唴鏍稿垯涓烘瘡涓彁渚涗簡鍗曠嫭鐨勫疄鐜般€?

璇锋敞鎰忥紝鍦?`CONFIG_PREEMPTION=y` 鍐呮牳涓紝rcu_read_lock_sched() 鍜?rcu_read_unlock_sched() 鍒嗗埆绂佺敤鍜岄噸鏂板惎鐢ㄦ姠鍗犮€傝繖鎰忓懗鐫€鍦?RCU-sched 璇荤涓寸晫鍖烘湡闂村鏋滄湁鎶㈠崰灏濊瘯锛宺cu_read_unlock_sched() 灏嗕細杩涘叆璋冨害鍣紝甯︽潵闅忎箣鑰屾潵鐨勬墍鏈夊欢杩熷拰寮€閿€銆傛濡?rcu_read_unlock_bh() 涓€鏍凤紝杩欏彲鑳借浜虹湅璧锋潵鍍忔槸 rcu_read_unlock_sched() 鎵ц寰楀緢鎱€傜劧鑰岋紝鏈€楂樹紭鍏堢骇鐨勪换鍔′笉浼氳鎶㈠崰锛屽洜姝よ浠诲姟浼氫韩鏈変綆寮€閿€鐨?rcu_read_unlock_sched() 璋冪敤銆?

`RCU-sched API <https://lwn.net/Articles/609973/#RCU%20Per-Flavor%20API%20Table>`__ 鍖呮嫭 rcu_read_lock_sched()銆乺cu_read_unlock_sched()銆乺cu_read_lock_sched_notrace()銆乺cu_read_unlock_sched_notrace()銆乺cu_dereference_sched()銆乺cu_dereference_sched_check() 鍜?rcu_read_lock_sched_held()銆傜劧鑰岋紝鏃х殑 RCU-sched 鏇存柊绔?API 鐜板湪宸茬粡娌℃湁浜嗭紝鍙栬€屼唬涔嬬殑鏄?synchronize_rcu()銆乻ynchronize_rcu_expedited()銆乧all_rcu() 鍜?rcu_barrier()銆傛澶栵紝浠讳綍绂佺敤鎶㈠崰鐨勬搷浣滀篃閮芥爣璁颁簡涓€涓?RCU-sched 璇荤涓寸晫鍖猴紝鍖呮嫭 preempt_disable() 鍜?preempt_enable()銆乴ocal_irq_save() 鍜?local_irq_restore() 绛夌瓑銆?

#### 鍙潯鐪?RCU


鍗佸骞存潵锛屽彧瑕佹湁浜鸿鈥滄垜闇€瑕佸湪 RCU 璇荤涓寸晫鍖哄唴闃诲鈥濓紝杩欓兘鏄竴涓彲闈犵殑淇″彿锛岃〃鏄庤繖涓汉涓嶆噦 RCU銆傛瘯绔燂紝濡傛灉浣犳€绘槸鍦?RCU 璇荤涓寸晫鍖哄唴闃诲锛岄偅浣犲ぇ姒傚彲浠ヨ礋鎷呭緱璧蜂娇鐢ㄤ竴涓紑閿€鏇撮珮鐨勫悓姝ユ満鍒躲€傜劧鑰岋紝闅忕潃 Linux 鍐呮牳閫氱煡鍣ㄧ殑鍑虹幇锛屾儏鍐垫敼鍙樹簡锛屽畠浠殑 RCU 璇荤涓寸晫鍖哄嚑涔庝粠涓嶇潯鐪狅紝浣嗘湁鏃跺張闇€瑕佺潯鐪犮€傝繖瀵艰嚧浜?`sleepable RCU <https://lwn.net/Articles/202847/>`__锛堝嵆鍙潯鐪?RCU锛屾垨绉?**SRCU**锛夌殑寮曞叆銆?

SRCU 鍏佽瀹氫箟涓嶅悓鐨勫煙锛坉omain锛夛紝姣忎釜鍩熺敱涓€涓?`srcu_struct` 缁撴瀯鐨勫疄渚嬪畾涔夈€傚繀椤绘妸杩欎釜缁撴瀯鐨勬寚閽堜紶鍏ユ瘡涓?SRCU 鍑芥暟锛屼緥濡?`synchronize_srcu(&ss)`锛屽叾涓?`ss` 鏄?`srcu_struct` 缁撴瀯銆傝繖浜涘煙鐨勫叧閿ソ澶勬槸锛屼竴涓煙涓緝鎱㈢殑 SRCU 璇昏€呬笉浼氬欢杩熷彟涓€涓煙涓殑 SRCU 瀹介檺鏈熴€傝瘽铏藉姝わ紝杩欎簺鍩熺殑涓€涓悗鏋滄槸锛岃绔唬鐮佸繀椤诲湪 srcu_read_lock() 鍜?srcu_read_unlock() 涔嬮棿浼犻€掍竴涓€渃ookie鈥濓紝渚嬪濡備笅锛?

```

       1 int idx;
       2
       3 idx = srcu_read_lock(&ss);
       4 do_something();
       5 srcu_read_unlock(&ss, idx);

```

濡備笂鎵€杩帮紝鍦?SRCU 璇荤涓寸晫鍖轰腑闃诲鏄悎娉曠殑锛岀劧鑰岋紝鑳藉姏瓒婂ぇ锛岃矗浠昏秺澶с€傚鏋滀綘鍦ㄦ煇涓粰瀹氬煙鐨?SRCU 璇荤涓寸晫鍖轰腑姘歌繙闃诲锛岄偅涔堣鍩熺殑瀹介檺鏈熶篃浼氭案杩滆闃诲銆傚綋鐒讹紝姘歌繙闃诲鐨勪竴涓ソ鍔炴硶灏辨槸姝婚攣锛屽鏋滄煇涓粰瀹氬煙鐨?SRCU 璇荤涓寸晫鍖轰腑鐨勪换浣曟搷浣滆兘澶熺洿鎺ユ垨闂存帴鍦扮瓑寰呰鍩熺殑瀹介檺鏈熻繃鍘伙紝姝婚攣灏卞彲鑳藉彂鐢熴€備緥濡傦紝杩欎細瀵艰嚧鑷閿侊細

```

       1 int idx;
       2
       3 idx = srcu_read_lock(&ss);
       4 do_something();
       5 synchronize_srcu(&ss);
       6 srcu_read_unlock(&ss, idx);

```

鐒惰€岋紝濡傛灉绗?5 琛岃幏鍙栦簡涓€鎶婂湪 `ss` 鍩熺殑 synchronize_srcu() 鏈熼棿琚寔鏈夌殑浜掓枼閿侊紝浠嶇劧鍙兘鍙戠敓姝婚攣銆傛澶栵紝濡傛灉绗?5 琛岃幏鍙栦簡涓€鎶婂湪鍙︿竴涓煙 `ss1` 鐨?synchronize_srcu() 鏈熼棿琚寔鏈夌殑浜掓枼閿侊紝骞朵笖濡傛灉鏌愪釜 `ss1` 鍩熺殑 SRCU 璇荤涓寸晫鍖鸿幏鍙栦簡鍙︿竴鎶婂湪 `ss` 鍩熺殑 synchronize_srcu() 鏈熼棿琚寔鏈夌殑浜掓枼閿侊紝姝婚攣鍚屾牱鍙兘鍙戠敓銆傝繖鏍风殑姝婚攣鐜彲浠ヨ法瓒婁换鎰忓涓笉鍚岀殑 SRCU 鍩熴€傚啀娆″己璋冿紝鑳藉姏瓒婂ぇ锛岃矗浠昏秺澶э紝涓嶈繃 lockdep 鐜板湪鑳藉妫€娴嬭繖绫绘閿併€?

涓庡叾浠?RCU 椋庢牸涓嶅悓锛孲RCU 璇荤涓寸晫鍖哄彲浠ヨ繍琛屽湪绌洪棽銆佺敋鑷崇绾跨殑 CPU 涓婏紝srcu_read_lock_fast() 鍙婂叾鍚岀被闄ゅ銆傝繖涓€鑳藉姏瑕佹眰 srcu_read_lock() 鍜?srcu_read_unlock() 鍖呭惈鍐呭瓨灞忛殰锛岃繖鎰忓懗鐫€ SRCU 璇昏€呯殑杩愯浼氭瘮 RCU 璇昏€呯◢鎱竴浜涖€傚畠涔熶績鎴愪簡 smp_mb__after_srcu_read_unlock() API锛屽畠涓?srcu_read_unlock() 缁撳悎锛屼繚璇佷竴鏉″畬鏁村唴瀛樺睆闅溿€?

鍚屾牱涓庡叾浠?RCU 椋庢牸涓嶅悓锛宻ynchronize_srcu() **涓嶈兘**浠?CPU 鐑彃鎷旈€氱煡鍣ㄤ腑璋冪敤锛屽師鍥犳槸 SRCU 瀹介檺鏈熷埄鐢ㄤ簡瀹氭椂鍣紝浠ュ強瀹氭椂鍣ㄥ彲鑳戒复鏃垛€滄粸鐣欌€濆湪绂荤嚎鐨?CPU 涓婄殑鍙兘鎬с€傝繖绉嶅畾鏃跺櫒鐨勬粸鐣欐剰鍛崇潃鎶曢€掔粰绂荤嚎 CPU 鐨勫畾鏃跺櫒鍦?CPU 鐑彃鎷旇繃绋嬪悗鏈熶箣鍓嶄笉浼氳Е鍙戙€傞棶棰樺湪浜庯紝濡傛灉涓€涓€氱煡鍣ㄦ鍦ㄧ瓑寰呬竴涓?SRCU 瀹介檺鏈燂紝鑰岄偅涓闄愭湡姝ｅ湪绛夊緟涓€涓畾鏃跺櫒锛岃€岄偅涓畾鏃跺櫒婊炵暀鍦ㄨ绂荤嚎鐨?CPU 涓婏紝閭ｄ箞璇ラ€氱煡鍣ㄥ氨姘歌繙涓嶄細琚敜閱掞紝鎹㈣█涔嬶紝鍙戠敓浜嗘閿併€傚綋鐒讹紝鍚屾牱鐨勬儏鍐典篃绂佹浜嗕粠 CPU 鐑彃鎷旈€氱煡鍣ㄤ腑璋冪敤 srcu_barrier()銆?

SRCU 涓庡叾浠?RCU 椋庢牸鐨勫彟涓€澶勪笉鍚屽湪浜庯紝SRCU 鐨勫姞閫燂紙expedited锛夊拰闈炲姞閫熷闄愭湡鏄敱鍚屼竴鏈哄埗瀹炵幇鐨勩€傝繖鎰忓懗鐫€鍦ㄥ綋鍓嶇殑 SRCU 瀹炵幇涓紝鍔犻€熶竴涓湭鏉ョ殑瀹介檺鏈熸湁涓€涓壇浣滅敤锛屽氨鏄姞閫熶簡鎵€鏈夊皻鏈畬鎴愮殑鍏堝墠瀹介檺鏈熴€傦紙浣嗚娉ㄦ剰锛岃繖鏄綋鍓嶅疄鐜扮殑涓€涓睘鎬э紝鏈繀鏄湭鏉ュ疄鐜扮殑灞炴€с€傦級姝ゅ锛屽鏋?SRCU 宸茬粡绌洪棽浜嗚秴杩?`srcutree.exp_holdoff` 鍐呮牳寮曞鍙傛暟鎵€鎸囧畾鐨勯棿闅旓紙榛樿 25 寰锛夛紝骞朵笖濡傛灉涓€娆?synchronize_srcu() 璋冪敤缁撴潫浜嗚繖涓┖闂叉湡锛岄偅涔堣璋冪敤浼氳鑷姩鍔犻€熴€?

鑷?v4.12 璧凤紝SRCU 鐨勫洖璋冩槸姣?CPU 缁存姢鐨勶紝娑堥櫎浜嗗厛鍓嶅唴鏍哥増鏈腑瀛樺湪鐨勪竴涓姞閿佺摱棰堛€傚敖绠¤繖灏嗗厑璁哥敤鎴峰 call_srcu() 鏂藉姞鏇撮噸鐨勫帇鍔涳紝浣嗛噸瑕佺殑鏄娉ㄦ剰锛孲RCU 灏氭湭閲囧彇浠讳綍鐗规畩姝ラ鏉ュ簲瀵瑰洖璋冩椽娉涖€傛墍浠ュ鏋滀綘姣?CPU 姣忕鎻愪氦锛堟瘮濡傦級10,000 涓?SRCU 鍥炶皟锛屼綘澶ф瀹屽叏娌￠棶棰橈紱浣嗗鏋滀綘鎵撶畻姣?CPU 姣忕鎻愪氦锛堟瘮濡傦級1,000,000 涓?SRCU 鍥炶皟锛岃鍏堣繍琛屼竴浜涙祴璇曘€係RCU 鍙兘纭疄闇€瑕佷竴浜涜皟鏁存潵搴斿閭ｇ璐熻浇銆傚綋鐒讹紝鍏蜂綋鏁堟灉浼氬洜浣犵殑 CPU 閫熷害鍜屽唴瀛樺ぇ灏忚€屽紓銆?

`SRCU API <https://lwn.net/Articles/609973/#RCU%20Per-Flavor%20API%20Table>`__ 鍖呮嫭 srcu_read_lock()銆乻rcu_read_unlock()銆乻rcu_dereference()銆乻rcu_dereference_check()銆乻ynchronize_srcu()銆乻ynchronize_srcu_expedited()銆乧all_srcu()銆乻rcu_barrier() 鍜?srcu_read_lock_held()銆傚畠杩樺寘鎷敤浜庡畾涔夊拰鍒濆鍖?`srcu_struct` 缁撴瀯鐨?DEFINE_SRCU()銆丏EFINE_STATIC_SRCU()銆丏EFINE_SRCU_FAST()銆丏EFINE_STATIC_SRCU_FAST()銆乮nit_srcu_struct() 鍜?init_srcu_struct_fast() API銆?

鏇磋繎涓€浜涳紝SRCU API 澧炲姞浜嗚疆璇㈡帴鍙ｏ細

#. start_poll_synchronize_srcu() 杩斿洖涓€涓爣璇嗘湭鏉?SRCU 瀹介檺鏈熷畬鎴愮殑 cookie锛屽苟纭繚杩欎釜瀹介檺鏈熶細琚惎鍔ㄣ€?
#. poll_state_synchronize_srcu() 鍦ㄦ寚瀹?cookie 瀵瑰簲浜庝竴涓凡缁忓畬鎴?SRCU 瀹介檺鏈熸椂杩斿洖 `true`銆?
#. get_state_synchronize_srcu() 杩斿洖涓?start_poll_synchronize_srcu() 涓€鏍风殑 cookie锛屼絾鍖哄埆鍦ㄤ簬瀹冧笉鍋氫换浣曚簨鎯呮潵纭繚浠讳綍鏈潵鐨?SRCU 瀹介檺鏈熶細琚惎鍔ㄣ€?

杩欎簺鍑芥暟鐢ㄤ簬鍦ㄦ煇浜涘叿鏈夊绾ц€佸寲鏈哄埗鐨勭紦鍐插尯缂撳瓨绠楁硶涓伩鍏嶄笉蹇呰鐨?SRCU 瀹介檺鏈熴€傚叾鎬濊矾鏄紝绛夊埌璇ュ潡瀹屽叏浠庣紦瀛樹腑鑰佸寲鎺夋椂锛屼竴涓?SRCU 瀹介檺鏈熸瀬鏈夊彲鑳藉凡缁忚繃鍘汇€?

#### 浠诲姟 RCU


鏌愪簺褰㈠紡鐨勮窡韪娇鐢ㄢ€渢rampoline锛堣烦鏉匡級鈥濇潵澶勭悊瀹夎涓嶅悓绫诲瀷鎺㈤拡鎵€闇€鐨勪簩杩涘埗閲嶅啓銆傝兘澶熼噴鏀炬棫鐨?trampoline 浼氬緢濂斤紝杩欏惉璧锋潵鍍忔槸鏌愮褰㈠紡 RCU 鐨勬椿鍎裤€傜劧鑰岋紝鍥犱负蹇呴』鑳藉鍦ㄤ唬鐮佷腑鐨勪换浣曚綅缃畨瑁呰窡韪紝鎵€浠ヤ笉鍙兘浣跨敤鍍?rcu_read_lock() 鍜?rcu_read_unlock() 杩欐牱鐨勮绔爣璁般€傛澶栵紝鎶婅繖浜涙爣璁版斁鍦?trampoline 鏈韩閲屼篃涓嶈锛屽洜涓?rcu_read_unlock() 涔嬪悗闇€瑕佹湁鎸囦护璺熼殢銆傚敖绠?synchronize_rcu() 浼氫繚璇佹墽琛屽埌杈句簡 rcu_read_unlock()锛屼絾瀹冩棤娉曚繚璇佹墽琛屽凡缁忓畬鍏ㄧ寮€浜?trampoline銆傛洿绯熺殑鏄紝鍦ㄦ煇浜涙儏鍐典笅锛宼rampoline 鐨勪繚鎶ゅ繀椤诲欢浼稿埌鎵ц鍒拌揪 trampoline **涔嬪墠**鐨勫嚑鏉℃寚浠ゃ€備緥濡傦紝杩欏嚑鏉℃寚浠ゅ彲鑳戒細璁＄畻 trampoline 鐨勫湴鍧€锛屼粠鑰岃繘鍏?trampoline 浼氬湪鎵ц瀹為檯鍒拌揪 trampoline 鏈韩涔嬪墠寰堜箙灏辫棰勫厛娉ㄥ畾浜嗐€?

瑙ｅ喅鏂规浠?`Tasks RCU <https://lwn.net/Articles/607117/>`__ 鐨勫舰寮忓嚭鐜帮紝鍗虫嫢鏈夌敱鑷効涓婁笅鏂囧垏鎹㈡墍鐣屽畾鐨勯殣寮忚绔复鐣屽尯锛屼篃灏辨槸瀵?schedule()銆乧ond_resched() 鍜?synchronize_rcu_tasks() 鐨勮皟鐢ㄣ€傛澶栵紝杩涘嚭鐢ㄦ埛鎬佹墽琛岀殑杞崲涔熺晫瀹氫簡浠诲姟 RCU 璇荤涓寸晫鍖恒€傜┖闂蹭换鍔¤ Tasks RCU 蹇界暐锛孴asks Rude RCU 鍙互鐢ㄦ潵涓庡畠浠氦浜掋€?

璇锋敞鎰忥紝闈炶嚜鎰跨殑涓婁笅鏂囧垏鎹?*涓嶆槸** Tasks-RCU 闈欐鐘舵€併€傛瘯绔燂紝鍦ㄥ彲鎶㈠崰鍐呮牳涓紝鎵ц trampoline 涓唬鐮佺殑浠诲姟鍙兘琚姠鍗犮€傚湪杩欑鎯呭喌涓嬶紝Tasks-RCU 瀹介檺鏈熷湪璇ヤ换鍔℃仮澶嶅苟涓斿叾鎵ц绂诲紑璇?trampoline 涔嬪墠鏄剧劧鏃犳硶缁撴潫銆傝繖鎰忓懗鐫€锛岄櫎鍏朵粬澶栵紝cond_resched() 骞朵笉鎻愪緵 Tasks RCU 闈欐鐘舵€併€傦紙鍙栬€屼唬涔嬶紝鍦ㄨ蒋涓柇涓娇鐢?rcu_softirq_qs()锛屽惁鍒欎娇鐢?rcu_tasks_classic_qs()銆傦級

浠诲姟 RCU 鐨?API 鐩稿綋绱у噾锛屽彧鍖呭惈 call_rcu_tasks()銆乻ynchronize_rcu_tasks() 鍜?rcu_barrier_tasks()銆傚湪 `CONFIG_PREEMPTION=n` 鍐呮牳涓紝trampoline 涓嶈兘琚姠鍗狅紝鍥犳杩欎簺 API 鍒嗗埆鏄犲皠涓?call_rcu()銆乻ynchronize_rcu() 鍜?rcu_barrier()銆傚湪 `CONFIG_PREEMPTION=y` 鍐呮牳涓紝trampoline 鍙互琚姠鍗狅紝鍥犳杩欎笁涓?API 鐢卞崟鐙殑鍑芥暟瀹炵幇锛岃繖浜涘嚱鏁版鏌ヨ嚜鎰夸笂涓嬫枃鍒囨崲銆?

#### Tasks Rude RCU


鏌愪簺褰㈠紡鐨勮窡韪渶瑕佺瓑寰呰繍琛屽湪浠讳綍鍦ㄧ嚎 CPU 涓婄殑鎵€鏈夌鐢ㄦ姠鍗犵殑浠ｇ爜鍖哄煙锛屽寘鎷偅浜涘湪 RCU 娌℃湁瑙傚療鏃舵墽琛岀殑鍖哄煙銆傝繖鎰忓懗鐫€ synchronize_rcu() 鏄笉澶熺殑锛屽繀椤绘敼鐢?Tasks Rude RCU銆傝繖绉?RCU 椋庢牸閫氳繃寮哄埗鍦ㄦ瘡涓湪绾?CPU 涓婅皟搴︿竴涓伐浣滈槦鍒楁潵瀹屾垚瀹冪殑宸ヤ綔锛宧ence 寰椾簡鈥淩ude锛堢矖椴侊級鈥濊繖涓话鍙枫€傝€屽疄鏃跺伐浣滆礋杞斤紙涓嶆兂璁╁畠浠殑 `nohz_full` CPU 鏀跺埌 IPI锛変互鍙婄數姹犱緵鐢电殑绯荤粺锛堜笉鎯宠瀹冧滑鐨勭┖闂?CPU 琚敜閱掞級閮借涓鸿繖涓搷浣滅浉褰撶矖椴併€?

涓€鏃﹀唴鏍哥殑杩涘叆/閫€鍑哄拰娣辩┖闂插嚱鏁拌姝ｇ‘鏍囪涓?`noinstr`锛孴asks RCU 灏卞彲浠ュ紑濮嬪叧娉ㄧ┖闂蹭换鍔★紙RCU 瑙嗚涓嬬┖闂茬殑閭ｄ簺闄ゅ锛夛紝鐒跺悗 Tasks Rude RCU 灏卞彲浠ヤ粠鍐呮牳涓Щ闄や簡銆?
浠诲姟绮楅瞾 RCU 鐨?API 鍚屾牱娌℃湁璇荤鏍囪锛屽洜姝ょ浉褰撶揣鍑戯紝浠呯敱 synchronize_rcu_tasks_rude() 缁勬垚銆?

#### 浠诲姟璺熻釜 RCU


鏌愪簺褰㈠紡鐨勮窡韪渶瑕佸湪璇昏€呬腑鐫＄湢锛屼絾鍙堟棤娉曞蹇?SRCU 鐨勮绔紑閿€锛屽悗鑰呭湪 srcu_read_lock() 鍜?srcu_read_unlock() 涓兘鍖呭惈涓€鏉″畬鏁村唴瀛樺睆闅溿€傝繖涓€闇€姹傜敱浠诲姟璺熻釜 RCU API 澶勭悊锛屽畠琚疄鐜颁负鍥寸粫 SRCU-fast 鐨勮交閲忓寘瑁咃紝浠庤€岄伩鍏嶄簡璇荤鍐呭瓨灞忛殰锛岃嚦灏戝浜庨偅浜涘鍐呮牳杩涘叆/閫€鍑轰唬鐮佸簲鐢ㄤ簡 noinstr 鐨勪綋绯荤粨鏋勶紙鎴栬€呮瀯寤烘椂甯︽湁 `CONFIG_TASKS_TRACE_RCU_NO_MB=y` 鐨勪綋绯荤粨鏋勶級鑰岃█濡傛銆?

鏃㈢劧瀹炵幇鍩轰簬 SRCU-fast锛屽 synchronize_rcu_tasks_trace() 鐨勪竴娆¤皟鐢ㄥ氨闅愬惈浜嗚嚦灏戜竴娆″ synchronize_rcu() 鐨勮皟鐢紝涔熷氨鏄锛屾瘡涓€涓换鍔¤窡韪?RCU 瀹介檺鏈熼兘鑷冲皯鍖呭惈涓€涓櫘閫氱殑 RCU 瀹介檺鏈熴€傚鏋滃皢鏉ュ嚭鐜?synchronize_rcu_tasks_trace_expedited()锛岃繖涓€淇濊瘉**涓嶄竴瀹?*閫傜敤浜庤繖涓亣鎯崇殑 API 鎴愬憳銆?

浠诲姟璺熻釜 RCU 鐨?API 涔熺浉褰撶揣鍑戯紝鐢?rcu_read_lock_trace()銆乺cu_read_unlock_trace()銆乺cu_read_lock_trace_held()銆乧all_rcu_tasks_trace()銆乻ynchronize_rcu_tasks_trace() 鍜?rcu_barrier_tasks_trace() 缁勬垚銆?

### 鍙兘鐨勬湭鏉ュ彉鏇?


RCU 鐢ㄦ潵鑾峰緱鏇存柊绔彲鎵╁睍鎬х殑涓€涓妧宸ф槸锛岄殢鐫€ CPU 鏁伴噺鐨勫鍔犺€屾彁楂樺闄愭湡寤惰繜銆傚鏋滆繖鎴愪负涓€涓弗閲嶉棶棰橈紝灏辨湁蹇呰閲嶆柊璁捐瀹介檺鏈熺姸鎬佹満锛屼互閬垮厤瀵硅繖绉嶉澶栧欢杩熺殑闇€姹傘€?

RCU 鍦ㄥ皯鏁板湴鏂圭鐢ㄤ簡 CPU 鐑彃鎷旓紝鏈€钁楀悕鐨勬垨璁告槸鍦?rcu_barrier() 鎿嶄綔涓€傚鏋滄湁寮虹儓鐞嗙敱瑕佸湪 CPU 鐑彃鎷旈€氱煡鍣ㄤ腑浣跨敤 rcu_barrier()锛屽氨鏈夊繀瑕侀伩鍏嶇鐢?CPU 鐑彃鎷斻€傝繖浼氬紩鍏ヤ竴浜涘鏉傛€э紝鎵€浠ユ渶濂芥湁涓€涓?*闈炲父**濂界殑鐞嗙敱銆?

瀹介檺鏈熷欢杩熶笌瀵瑰叾浠?CPU 鐨勬墦鎵拌繖涓よ€呬箣闂寸殑鏉冭　鍙兘闇€瑕侀噸鏂板瑙嗐€傚綋鐒讹紝鐞嗘兂鎯呭喌鏄棦鏈夐浂瀹介檺鏈熷欢杩燂紝鍙堝湪鍔犻€熷闄愭湡鎿嶄綔鏈熼棿浜х敓闆跺鐞嗗櫒闂翠腑鏂€傚敖绠¤繖涓悊鎯充笉澶彲鑳藉疄鐜帮紝浣嗚繘涓€姝ユ敼杩涙槸鐩稿綋鏈夊彲鑳界殑銆?

RCU 鐨勫澶勭悊鍣ㄥ疄鐜颁娇鐢ㄤ竴妫电粍鍚堟爲鏉ュ CPU 鍒嗙粍锛屼互鍑忓皯閿佺珵浜夊苟澧炲姞缂撳瓨灞€閮ㄦ€с€傜劧鑰岋紝杩欐５缁勫悎鏍戝苟娌℃湁鎶婂畠鐨勫唴瀛樺垎鏁ｅ埌 NUMA 鑺傜偣涓婏紝涔熸病鏈夋妸 CPU 缁勪笌鎻掓Ы鎴栨牳杩欐牱鐨勭‖浠剁壒鎬у榻愩€傜洰鍓嶈涓鸿繖绉嶅垎鏁ｅ拰瀵归綈鏄笉蹇呰鐨勶紝鍥犱负鐑矾寰勪笂鐨勮绔師璇苟涓嶈闂粍鍚堟爲锛屽父瑙佹儏鍐典笅鐨?call_rcu() 涔熶笉浼氥€傚鏋滀綘璁や负浣犵殑浣撶郴缁撴瀯闇€瑕佽繖绉嶅垎鏁ｅ拰瀵归綈锛岄偅涔堜綘鐨勪綋绯荤粨鏋勪篃搴旇鑳戒粠 `rcutree.rcu_fanout_leaf` 寮曞鍙傛暟鍙楃泭锛屽畠鍙互璁剧疆涓轰竴涓彃妲姐€丯UMA 鑺傜偣鎴栭殢渚夸粈涔堜腑鐨?CPU 鏁伴噺銆傚鏋?CPU 鏁伴噺澶ぇ锛屽氨浣跨敤 CPU 鏁伴噺鐨勪竴涓垎鏁般€傚鏋?CPU 鏁伴噺鏄竴涓緢澶х殑绱犳暟锛屽棷锛岄偅缁濆鏄竴涓€滄湁瓒ｂ€濈殑浣撶郴缁撴瀯閫夋嫨锛佹洿鐏垫椿鐨勫畨鎺掓垨璁镐細琚€冭檻锛屼絾鍓嶆彁鏄?`rcutree.rcu_fanout_leaf` 宸茶璇佹槑涓嶅鐢紝骞朵笖杩欑涓嶅鐢ㄥ凡缁忛€氳繃涓€涓粩缁嗚繍琛屼笖鐜板疄鐨勭郴缁熺骇宸ヤ綔璐熻浇寰楀埌璇佸疄銆?

璇锋敞鎰忥紝瑕佹眰 RCU 閲嶆柊鏄犲皠 CPU 缂栧彿鐨勫畨鎺掞紝闇€瑕佹瀬濂藉湴璇佹槑闇€姹傜殑瀛樺湪锛屽苟鍏呭垎鎺㈢储鏇夸唬鏂规銆?

RCU 鐨勫悇绉?kthread 鏄浉褰撹繎鏈熺殑娣诲姞銆傚緢鍙兘闇€瑕佽繘琛岃皟鏁达紝浠ユ洿浼橀泤鍦板簲瀵规瀬绔礋杞姐€傚彲鑳借繕闇€瑕佽兘澶熸妸 RCU 鐨?kthread 鍜岃蒋涓柇澶勭悊绋嬪簭閫犳垚鐨?CPU 鍗犵敤锛屽綊鍜庡埌寮曞彂杩欎竴 CPU 鍗犵敤鐨勪唬鐮佸ご涓娿€備緥濡傦紝RCU 鍥炶皟寮€閿€鎴栬浼氳鍥炴函璁板埌鍙戣捣鐨?call_rcu() 瀹炰緥澶翠笂锛屽敖绠″湪鐢熶骇鍐呮牳涓ぇ姒備笉浼氳繖鏍峰仛銆?

鍙兘闇€瑕侀澶栫殑宸ヤ綔锛屼互鍦ㄩ噸璐熻浇涓嬩负瀹介檺鏈熷拰鍥炶皟璋冪敤鎻愪緵鍚堢悊鐨勫悜鍓嶆帹杩涗繚璇併€?

### 鎬荤粨


鏈枃妗ｅ憟鐜颁簡瓒呰繃浜屽崄骞寸殑 RCU 闇€姹傘€傞壌浜庤繖浜涢渶姹備竴鐩村湪鍙樺寲锛岃繖涓嶄細鏄湁鍏宠繖涓富棰樼殑鏈€鍚庡畾璁猴紝浣嗚嚦灏戝畠鏈夊姪浜庢妸涓€涓噸瑕佺殑闇€姹傚瓙闆嗛槓杩版竻妤氥€?

### 鑷磋阿


鎴戞劅璋?Steven Rostedt銆丩ai Jiangshan銆両ngo Molnar銆丱leg Nesterov銆丅orislav Petkov銆丳eter Zijlstra銆丅oqun Feng 鍜?Andy Lutomirski 鍦ㄦ妸杩欑瘒鏂囩珷鍙樺緱浜虹被鍙鏂归潰鎻愪緵鐨勫府鍔╋紝涔熸劅璋?Michelle Rankin 瀵硅繖椤瑰伐浣滅殑鏀寔銆傚叾浠栫殑璐＄尞鍦?Linux 鍐呮牳鐨?git 褰掓。涓緱鍒拌嚧璋€?
