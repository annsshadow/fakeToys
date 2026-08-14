
## 閿佺被鍨嬪強鍏惰鍒?

## 寮曡█

鍐呮牳鎻愪緵浜嗗绉嶉攣鍘熻锛屽彲鍒嗕负涓夌被锛?
 - 鐫＄湢閿侊紙Sleeping locks锛? - CPU 鏈湴閿侊紙CPU local locks锛? - 鑷棆閿侊紙Spinning locks锛?
鏈枃妗ｄ粠姒傚康涓婃弿杩拌繖浜涢攣绫诲瀷锛屽苟鎻愪緵瀹冧滑鐨勫祵濂楄鍒欙紝鍖呮嫭鍦?PREEMPT_RT 涓嬩娇鐢ㄧ殑瑙勫垯銆?

## 閿佺被鍒?

### 鐫＄湢閿?
鐫＄湢閿佸彧鑳藉湪鍙姠鍗犵殑浠诲姟涓婁笅鏂囦腑鑾峰彇銆?
灏界瀹炵幇鍏佽浠庡叾瀹冧笂涓嬫枃杩涜 try_lock()锛屼絾鏈夊繀瑕佷粩缁嗚瘎浼?unlock() 浠ュ強 try_lock() 鐨勫畨鍏ㄦ€с€?姝ゅ锛屼篃鏈夊繀瑕佽瘎浼拌繖浜涘師璇殑璋冭瘯鐗堟湰銆傜畝鑰岃█涔嬶紝涓嶈浠庡叾瀹冧笂涓嬫枃鑾峰彇鐫＄湢閿侊紝闄ら潪鍒棤閫夋嫨銆?
鐫＄湢閿佺被鍨嬶細

 - mutex
 - rt_mutex
 - semaphore
 - rw_semaphore
 - ww_mutex
 - percpu_rw_semaphore

鍦?PREEMPT_RT 鍐呮牳涓婏紝浠ヤ笅閿佺被鍨嬭杞崲涓虹潯鐪犻攣锛?
 - local_lock
 - spinlock_t
 - rwlock_t


### CPU 鏈湴閿?
 - local_lock

鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝local_lock 鍑芥暟鏄鐢ㄦ姠鍗犲拰涓柇鍘熻鐨勫皝瑁呫€備笌鍏跺畠閿佹満鍒剁浉鍙嶏紝绂佺敤鎶㈠崰鎴?涓柇鏄函绮圭殑 CPU 鏈湴骞跺彂鎺у埗鏈哄埗锛屽苟涓嶉€傚悎鐢ㄤ簬 CPU 闂村苟鍙戞帶鍒躲€?

### 鑷棆閿?
 - raw_spinlock_t
 - 浣嶈嚜鏃嬮攣锛坆it spinlocks锛?
鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝浠ヤ笅閿佺被鍨嬩篃鏄嚜鏃嬮攣锛?
 - spinlock_t
 - rwlock_t

鑷棆閿侀殣寮忕鐢ㄦ姠鍗狅紝骞朵笖閿?瑙ｉ攣鍑芥暟鍙互甯︽湁鍚庣紑浠ュ簲鐢ㄨ繘涓€姝ョ殑淇濇姢锛?
 ===================  ====================================================
 _bh()                Disable / enable bottom halves锛堣蒋涓柇锛? _irq()               Disable / enable 涓柇
 _irqsave/restore()   淇濆瓨骞剁鐢?/ 鎭㈠涓柇绂佺敤鐘舵€? ===================  ====================================================


## 鎵€鏈夎€呰涔?
闄や簡淇″彿閲忎箣澶栵紝涓婅堪閿佺被鍨嬮兘鍏锋湁涓ユ牸鐨勬墍鏈夎€呰涔夛細

  鑾峰彇閿佺殑涓婁笅鏂囷紙浠诲姟锛夊繀椤婚噴鏀惧畠銆?
rw_semaphore 鏈変竴涓壒娈婃帴鍙ｏ紝鍏佽璇昏€呰繘琛岄潪鎵€鏈夎€呴噴鏀俱€?

## rtmutex

RT-mutex 鏄敮鎸佷紭鍏堢骇缁ф壙锛圥I锛夌殑 mutex銆?
鐢变簬鎶㈠崰鍜屼腑鏂鐢ㄦ鐨勫瓨鍦紝PI 鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婂彈鍒伴檺鍒躲€?
鍗充娇鍦?PREEMPT_RT 鍐呮牳涓婏紝PI 鏄剧劧涔熸棤娉曟姠鍗犵鐢ㄦ姠鍗犳垨绂佺敤涓柇鐨勪唬鐮佹銆傜浉鍙嶏紝PREEMPT_RT 鍐呮牳鍦?鍙姠鍗犵殑浠诲姟涓婁笅鏂囦腑鎵ц澶у鏁版绫讳唬鐮佹锛岀壒鍒槸涓柇澶勭悊绋嬪簭鍜岃蒋涓柇銆傝繖绉嶈浆鎹娇寰?spinlock_t 鍜?rwlock_t 鑳藉閫氳繃 RT-mutex 瀹炵幇銆?

## semaphore

semaphore 鏄竴涓鏁颁俊鍙烽噺鐨勫疄鐜般€?
淇″彿閲忓父甯告棦鐢ㄤ簬搴忓垪鍖栧張鐢ㄤ簬绛夊緟锛屼絾鏂扮殑鐢ㄤ緥搴斿綋鏀圭敤鐙珛鐨勫簭鍒楀寲鍜岀瓑寰呮満鍒讹紝渚嬪 mutex 鍜?completion銆?

### semaphore 涓?PREEMPT_RT

PREEMPT_RT 涓嶆敼鍙樹俊鍙烽噺鐨勫疄鐜帮紝鍥犱负璁℃暟淇″彿閲忔病鏈夋墍鏈夎€呯殑姒傚康锛屼粠鑰岄樆姝?PREEMPT_RT 涓轰俊鍙烽噺鎻愪緵
浼樺厛绾х户鎵裤€傛瘯绔燂紝鏈煡鐨勬墍鏈夎€呮棤娉曡鎻愬崌浼樺厛绾с€傚洜姝わ紝鍦ㄤ俊鍙烽噺涓婇樆濉炲彲鑳藉鑷翠紭鍏堢骇鍙嶈浆銆?

## rw_semaphore

rw_semaphore 鏄竴绉嶅璇昏€呭崟鍐欒€呴攣鏈哄埗銆?
鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝瀹炵幇鏄叕骞崇殑锛屼粠鑰岄槻姝㈠啓鑰呴ゥ楗裤€?
rw_semaphore 榛樿閬靛畧涓ユ牸鐨勬墍鏈夎€呰涔夛紝浣嗗瓨鍦ㄥ厑璁歌鑰呴潪鎵€鏈夎€呴噴鏀剧殑鐗规畩鐢ㄩ€旀帴鍙ｃ€傝繖浜涙帴鍙ｇ嫭绔嬩簬
鍐呮牳閰嶇疆宸ヤ綔銆?

### rw_semaphore 涓?PREEMPT_RT

PREEMPT_RT 鍐呮牳灏?rw_semaphore 鏄犲皠鍒板熀浜?rt_mutex 鐨勫崟鐙疄鐜帮紝浠庤€屾敼鍙樹簡鍏钩鎬э細

  鍥犱负 rw_semaphore 鍐欒€呮棤娉曞皢鍏朵紭鍏堢骇鎺堜簣澶氫釜璇昏€咃紝涓€涓鎶㈠崰鐨勪綆浼樺厛绾ц鑰呭皢缁х画鎸佹湁鍏堕攣锛屼粠鑰?  浣垮嵆浣挎槸楂樹紭鍏堢骇鐨勫啓鑰呬篃浼氶ゥ楗裤€傜浉鍙嶏紝鍥犱负璇昏€呭彲浠ュ皢鍏朵紭鍏堢骇鎺堜簣鍐欒€咃紝涓€涓鎶㈠崰鐨勪綆浼樺厛绾у啓鑰?  灏嗕娇鍏朵紭鍏堢骇琚彁鍗囷紝鐩村埌瀹冮噴鏀鹃攣锛屼粠鑰岄槻姝㈣鍐欒€呬娇璇昏€呴ゥ楗裤€?

## local_lock

local_lock 涓洪€氳繃绂佺敤鎶㈠崰鎴栦腑鏂繚鎶ょ殑涓寸晫鍖烘彁渚涗竴涓叿鍚嶄綔鐢ㄥ煙銆?
鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝local_lock 鎿嶄綔鏄犲皠鍒扮鐢ㄥ拰鍚敤鎶㈠崰鍙婁腑鏂殑鍘熻锛?
 ===============================  ======================
 local_lock(&llock)               preempt_disable()
 local_unlock(&llock)             preempt_enable()
 local_lock_irq(&llock)           local_irq_disable()
 local_unlock_irq(&llock)         local_irq_enable()
 local_lock_irqsave(&llock)       local_irq_save()
 local_unlock_irqrestore(&llock)  local_irq_restore()
 ===============================  ======================

local_lock 鐨勫叿鍚嶄綔鐢ㄥ煙鐩稿浜庡父瑙勫師璇湁涓や釜浼樼偣锛?
  - 閿佸悕鍏佽闈欐€佸垎鏋愶紝鍚屾椂涔熸槸瀵逛繚鎶よ寖鍥寸殑娓呮櫚鏂囨。璇存槑锛岃€屽父瑙勫師璇槸鏃犱綔鐢ㄥ煙涓斾笉閫忔槑鐨勩€?
  - 濡傛灉鍚敤浜?lockdep锛宭ocal_lock 浼氳幏寰椾竴涓?lockmap锛屽彲鐢ㄤ簬楠岃瘉淇濇姢鐨勬纭€с€傝繖鍙互妫€娴嬩緥濡備娇鐢?    preempt_disable() 浣滀负淇濇姢鏈哄埗鐨勫嚱鏁颁粠涓柇鎴栬蒋涓柇涓婁笅鏂囪璋冪敤鐨勬儏鍐点€傞櫎姝や箣澶栵紝
    lockdep_assert_held(&llock) 涓庝换浣曞叾瀹冮攣鍘熻涓€鏍峰伐浣溿€?

### local_lock 涓?PREEMPT_RT

PREEMPT_RT 鍐呮牳灏?local_lock 鏄犲皠鍒版瘡-CPU 鐨?spinlock_t锛屼粠鑰屾敼鍙樿涔夛細

  - spinlock_t 鐨勬墍鏈夊彉鏇村悓鏍烽€傜敤浜?local_lock銆?

### local_lock 鐨勭敤娉?
local_lock 搴斿綋鐢ㄤ簬杩欐牱鐨勬儏褰細鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝绂佺敤鎶㈠崰鎴栦腑鏂槸淇濇姢姣?CPU 鏁版嵁缁撴瀯鐨勫悎閫?骞跺彂鎺у埗褰㈠紡銆?
鐢变簬 PREEMPT_RT 鐗瑰畾鐨?spinlock_t 璇箟锛宭ocal_lock 涓嶉€傚悎鍦?PREEMPT_RT 鍐呮牳涓婄敤浜庨槻鑼冩姠鍗犳垨涓柇銆?

### CPU 鏈湴浣滅敤鍩熶笌 bottom-half

浠呭湪鍐呮牳杞腑鏂紙softirq锛変笂涓嬫枃涓闂殑姣?CPU 鍙橀噺锛屼笉搴斾緷璧栦簬鈥滆涓婁笅鏂囧洜涓嶅彲鎶㈠崰鑰屽彈鍒伴殣寮忎繚鎶も€?杩欎竴鍋囪銆傚湪 PREEMPT_RT 鍐呮牳涓婏紝杞腑鏂笂涓嬫枃鏄彲鎶㈠崰鐨勶紝閫氳繃闅愬紡涓婁笅鏂囧悓姝ユ瘡涓鐢?bottom-half 鐨?娈典細瀵艰嚧涓€涓殣寮忕殑姣?CPU鈥滃ぇ鍐呮牳閿佲€濄€?
local_lock_t 閰嶅悎 local_lock_nested_bh() 鍜?local_unlock_nested_bh() 鐢ㄤ簬鍔犻攣鎿嶄綔锛屾湁鍔╀簬
鏍囪瘑鍔犻攣浣滅敤鍩熴€?
褰撳惎鐢?lockdep 鏃讹紝杩欎簺鍑芥暟楠岃瘉瀵规暟鎹粨鏋勭殑璁块棶鍙戠敓鍦ㄨ蒋涓柇涓婁笅鏂囦腑銆備笌 local_lock() 涓嶅悓锛?local_unlock_nested_bh() 涓嶇鐢ㄦ姠鍗狅紝骞朵笖鍦ㄤ笉浣跨敤 lockdep 鏃朵笉澧炲姞寮€閿€銆?
鍦?PREEMPT_RT 鍐呮牳涓婏紝local_lock_t 琛ㄧ幇涓轰竴鎶婄湡瀹炵殑閿侊紝local_unlock_nested_bh() 瀵规暟鎹粨鏋勮闂?杩涜搴忓垪鍖栵紝浠庤€屽彲浠ョЩ闄ら€氳繃 local_bh_disable() 杩涜鐨勫簭鍒楀寲銆?

## raw_spinlock_t 涓?spinlock_t


### raw_spinlock_t

raw_spinlock_t 鍦ㄦ墍鏈夊唴鏍革紙鍖呮嫭 PREEMPT_RT 鍐呮牳锛変腑閮芥槸涓ユ牸鐨勮嚜鏃嬮攣瀹炵幇銆備粎鍦ㄧ湡姝ｇ殑涓寸晫鏍稿績浠ｇ爜銆?搴曞眰涓柇澶勭悊浠ュ強闇€瑕佺鐢ㄦ姠鍗犳垨涓柇鐨勫湴鏂癸紙渚嬪锛屼负瀹夊叏璁块棶纭欢鐘舵€侊級浣跨敤 raw_spinlock_t銆傚綋涓寸晫鍖?闈炲父灏忔椂锛屾湁鏃朵篃鍙互浣跨敤 raw_spinlock_t锛屼粠鑰岄伩鍏?RT-mutex 鐨勫紑閿€銆?

### spinlock_t

spinlock_t 鐨勮涔夐殢 PREEMPT_RT 鐨勭姸鎬佽€屽彉鍖栥€?
鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝spinlock_t 琚槧灏勫埌 raw_spinlock_t锛屽苟鍏锋湁瀹屽叏鐩稿悓鐨勮涔夈€?

### spinlock_t 涓?PREEMPT_RT

鍦?PREEMPT_RT 鍐呮牳涓婏紝spinlock_t 琚槧灏勫埌鍩轰簬 rt_mutex 鐨勫崟鐙疄鐜帮紝浠庤€屾敼鍙樿涔夛細

 - 涓嶇鐢ㄦ姠鍗犮€?
 - 涓庣‖涓柇鐩稿叧鐨勫悗缂€锛坰pin_lock / spin_unlock 鎿嶄綔鐨?_irq銆乢irqsave / _irqrestore锛変笉褰卞搷 CPU 鐨?   涓柇绂佺敤鐘舵€併€?
 - 涓庤蒋涓柇鐩稿叧鐨勫悗缂€锛坃bh()锛変粛绂佺敤杞腑鏂鐞嗙▼搴忋€?
   闈?PREEMPT_RT 鍐呮牳閫氳繃绂佺敤鎶㈠崰鏉ヨ幏寰楁鏁堟灉銆?
   PREEMPT_RT 鍐呮牳浣跨敤姣?CPU 閿佽繘琛屽簭鍒楀寲锛屽悓鏃朵繚鎸佹姠鍗犲惎鐢ㄣ€傝閿佺鐢ㄨ蒋涓柇澶勭悊绋嬪簭锛屽苟闃叉鐢变簬
   浠诲姟鎶㈠崰鑰屽鑷寸殑閲嶅叆銆?
PREEMPT_RT 鍐呮牳淇濈暀鎵€鏈夊叾瀹?spinlock_t 璇箟锛?
 - 鎸佹湁 spinlock_t 鐨勪换鍔′笉浼氳縼绉汇€傞潪 PREEMPT_RT 鍐呮牳閫氳繃绂佺敤鎶㈠崰鏉ラ伩鍏嶈縼绉汇€侾REEMPT_RT 鍐呮牳鍒欐敼涓?   绂佺敤杩佺Щ锛岃繖纭繚鍗充娇浠诲姟琚姠鍗狅紝鎸囧悜姣?CPU 鍙橀噺鐨勬寚閽堜粛鐒舵湁鏁堛€?
 - 浠诲姟鐘舵€佸湪鑾峰彇 spinlock_t 鏈熼棿琚繚鐣欙紝纭繚浠诲姟鐘舵€佽鍒欓€傜敤浜庢墍鏈夊唴鏍搁厤缃€傞潪 PREEMPT_RT 鍐呮牳
   淇濇寔浠诲姟鐘舵€佷笉鍙樸€傜劧鑰岋紝濡傛灉浠诲姟鍦ㄨ幏鍙栨湡闂撮樆濉烇紝PREEMPT_RT 蹇呴』鏀瑰彉浠诲姟鐘舵€併€傚洜姝わ紝瀹冨湪闃诲鍓?   淇濆瓨褰撳墠浠诲姟鐘舵€侊紝鐩稿簲鐨勯攣鍞ら啋
```

    task->state = TASK_INTERRUPTIBLE
     lock()
       block()
         task->saved_state = task->state
	 task->state = TASK_UNINTERRUPTIBLE
	 schedule()
					lock wakeup
					  task->state = task->saved_state

   鍏跺畠绫诲瀷鐨勫敜閱掗€氬父浼氭棤鏉′欢灏嗕换鍔＄姸鎬佽涓?RUNNING锛屼絾杩欏湪杩欓噷涓嶈捣浣滅敤锛屽洜涓轰换鍔″繀椤讳繚鎸侀樆濉炵洿鍒伴攣
   鍙敤銆傚洜姝わ紝褰撲竴娆￠潪閿佸敜閱掑皾璇曞敜閱掍竴涓樆濉炵瓑寰呰嚜鏃嬮攣鐨勪换鍔℃椂锛屽畠鏀逛负灏嗕繚瀛樼殑鐘舵€佽涓?RUNNING銆?   鐒跺悗锛屽綋閿佽幏鍙栧畬鎴愭椂锛岄攣鍞ら啋灏嗕换鍔＄姸鎬佽涓轰繚瀛樼殑鐘舵€侊紝鍦ㄦ渚嬩腑灏嗗叾璁句负 RUNNING::

    task->state = TASK_INTERRUPTIBLE
     lock()
       block()
         task->saved_state = task->state
	 task->state = TASK_UNINTERRUPTIBLE
	 schedule()
					non lock wakeup
					  task->saved_state = TASK_RUNNING

					lock wakeup
					  task->state = task->saved_state

   杩欑‘淇濈湡姝ｇ殑鍞ら啋涓嶄細涓㈠け銆?

```
## rwlock_t

rwlock_t 鏄竴绉嶅璇昏€呭崟鍐欒€呴攣鏈哄埗銆?
闈?PREEMPT_RT 鍐呮牳灏?rwlock_t 瀹炵幇涓鸿嚜鏃嬮攣锛宻pinlock_t 鐨勫悗缂€瑙勫垯鐩稿簲閫傜敤銆傚疄鐜版槸鍏钩鐨勶紝浠庤€岄槻姝?鍐欒€呴ゥ楗裤€?

### rwlock_t 涓?PREEMPT_RT

PREEMPT_RT 鍐呮牳灏?rwlock_t 鏄犲皠鍒板熀浜?rt_mutex 鐨勫崟鐙疄鐜帮紝浠庤€屾敼鍙樿涔夛細

 - spinlock_t 鐨勬墍鏈夊彉鏇村悓鏍烽€傜敤浜?rwlock_t銆?
 - 鍥犱负 rwlock_t 鍐欒€呮棤娉曞皢鍏朵紭鍏堢骇鎺堜簣澶氫釜璇昏€咃紝涓€涓鎶㈠崰鐨勪綆浼樺厛绾ц鑰呭皢缁х画鎸佹湁鍏堕攣锛屼粠鑰屼娇鍗充娇鏄?   楂樹紭鍏堢骇鐨勫啓鑰呬篃浼氶ゥ楗裤€傜浉鍙嶏紝鍥犱负璇昏€呭彲浠ュ皢鍏朵紭鍏堢骇鎺堜簣鍐欒€咃紝涓€涓鎶㈠崰鐨勪綆浼樺厛绾у啓鑰呭皢浣垮叾浼樺厛绾?   琚彁鍗囷紝鐩村埌瀹冮噴鏀鹃攣锛屼粠鑰岄槻姝㈣鍐欒€呬娇璇昏€呴ゥ楗裤€?

## PREEMPT_RT 娉ㄦ剰浜嬮」


### RT 涓婄殑 local_lock

local_lock 鍦?PREEMPT_RT 鍐呮牳涓婃槧灏勫埌 spinlock_t 鏈変竴浜涘奖鍝嶃€備緥濡傦紝鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝浠ヤ笅浠ｇ爜
```

  local_lock_irq(&local_lock);
  raw_spin_lock(&lock);

```
```

   raw_spin_lock_irq(&lock);

```
鍦?PREEMPT_RT 鍐呮牳涓婏紝杩欐搴忓垪浼氬嚭閿欙紝鍥犱负 local_lock_irq() 琚槧灏勫埌姣?CPU 鐨?spinlock_t锛屽畠鏃笉绂佺敤
涓柇涔熶笉绂佺敤鎶㈠崰銆備互涓嬩唬鐮佸簭鍒楀湪涓よ€呬笂閮借兘瀹屽叏姝ｇ‘鍦板伐浣?```

  local_lock_irq(&local_lock);
  spin_lock(&lock);

```
local_lock 鐨勫彟涓€涓敞鎰忎簨椤规槸锛屾瘡涓?local_lock 閮芥湁涓€涓壒瀹氱殑
```

  func1()
  {
    local_irq_save(flags);    -> local_lock_irqsave(&local_lock_1, flags);
    func3();
    local_irq_restore(flags); -> local_unlock_irqrestore(&local_lock_1, flags);
  }

  func2()
  {
    local_irq_save(flags);    -> local_lock_irqsave(&local_lock_2, flags);
    func3();
    local_irq_restore(flags); -> local_unlock_irqrestore(&local_lock_2, flags);
  }

  func3()
  {
    lockdep_assert_irqs_disabled();
    access_protected_data();
  }

```
鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婅繖鑳芥纭伐浣滐紝浣嗗湪 PREEMPT_RT 鍐呮牳涓?local_lock_1 鍜?local_lock_2 鏄簰涓嶇浉鍚岀殑锛?鏃犳硶瀵?func3() 鐨勮皟鐢ㄨ€呰繘琛屽簭鍒楀寲銆傚苟涓旂敱浜?local_lock_irqsave() 涓嶇鐢ㄤ腑鏂紝lockdep 鏂█涔熶細鍦?PREEMPT_RT 鍐呮牳涓婅Е鍙戯紝鍥犱负
```

  func1()
  {
    local_irq_save(flags);    -> local_lock_irqsave(&local_lock, flags);
    func3();
    local_irq_restore(flags); -> local_unlock_irqrestore(&local_lock, flags);
  }

  func2()
  {
    local_irq_save(flags);    -> local_lock_irqsave(&local_lock, flags);
    func3();
    local_irq_restore(flags); -> local_unlock_irqrestore(&local_lock, flags);
  }

  func3()
  {
    lockdep_assert_held(&local_lock);
    access_protected_data();
  }


```
### spinlock_t 涓?rwlock_t

spinlock_t 鍜?rwlock_t 鍦?PREEMPT_RT 鍐呮牳涓婅涔夌殑鍙樺寲鏈変竴浜涘奖鍝嶃€備緥濡傦紝鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓?```

   local_irq_disable();
   spin_lock(&lock);

```
```

   spin_lock_irq(&lock);

```
鍚屾牱鐨勯亾鐞嗛€傜敤浜?rwlock_t 鍜?_irqsave() 鍚庣紑鍙樹綋銆?
鍦?PREEMPT_RT 鍐呮牳涓婏紝杩欐搴忓垪浼氬嚭閿欙紝鍥犱负 RT-mutex 闇€瑕佷竴涓畬鍏ㄥ彲鎶㈠崰鐨勪笂涓嬫枃銆傜浉鍙嶏紝搴斾娇鐢?spin_lock_irq() 鎴?spin_lock_irqsave() 鍙婂叾瀵瑰簲鐨勮В閿佸嚱鏁般€傚湪涓柇绂佺敤鍜屽姞閿佸繀椤讳繚鎸佸垎绂荤殑鎯呭喌涓嬶紝
PREEMPT_RT 鎻愪緵浜嗕竴绉?local_lock 鏈哄埗銆傝幏鍙?local_lock 灏嗕换鍔″浐瀹氬埌鏌愪釜 CPU锛屼粠鑰屽厑璁歌幏鍙栨瘡-CPU
鐨勭鐢ㄤ腑鏂攣绛夈€傜劧鑰岋紝杩欑鏂规硶鍙簲鍦ㄧ粷瀵瑰繀瑕佹椂浣跨敤銆?
```

  struct foo *p = get_cpu_ptr(&var1);

  spin_lock(&p->lock);
  p->count += this_cpu_read(var2);

```
鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婅繖鏄纭殑浠ｇ爜锛屼絾鍦?PREEMPT_RT 鍐呮牳涓婅繖浼氬嚭閿欍€俿pinlock_t 鐨?PREEMPT_RT 鐗瑰畾
璇箟鍙樺寲涓嶅厑璁歌幏鍙?p->lock锛屽洜涓?get_cpu_ptr() 闅愬紡鍦扮鐢?```

  struct foo *p;

  migrate_disable();
  p = this_cpu_ptr(&var1);
  spin_lock(&p->lock);
  p->count += this_cpu_read(var2);

```
migrate_disable() 纭繚浠诲姟琚浐瀹氬埌褰撳墠 CPU锛岃繘鑰屼繚璇佸彧瑕佷换鍔′繚鎸佸彲鎶㈠崰锛屽 var1 鍜?var2 鐨勬瘡-CPU 璁块棶
灏卞仠鐣欏湪鍚屼竴涓?CPU 涓娿€?
migrate_disable() 鏇挎崲瀵逛簬浠ヤ笅鎯呭喌鏃犳晥
```

  func()
  {
    struct foo *p;

    migrate_disable();
    p = this_cpu_ptr(&var1);
    p->val = func2();

```
杩欎細鍑洪敊锛屽洜涓?migrate_disable() 鏃犳硶闃茶寖鏉ヨ嚜
```

  func()
  {
    struct foo *p;

    local_lock(&foo_lock);
    p = this_cpu_ptr(&var1);
    p->val = func2();

```
鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝杩欓€氳繃绂佺敤鎶㈠崰鏉ラ槻鑼冮噸鍏ャ€傚湪 PREEMPT_RT 鍐呮牳涓婏紝杩欓€氳繃鑾峰彇搴曞眰鐨勬瘡-CPU
鑷棆閿佹潵瀹炵幇銆?

### RT 涓婄殑 raw_spinlock_t

鑾峰彇 raw_spinlock_t 浼氱鐢ㄦ姠鍗狅紝鍙兘杩樹細绂佺敤涓柇锛屽洜姝や复鐣屽尯蹇呴』閬垮厤鑾峰彇甯歌鐨?spinlock_t 鎴?rwlock_t锛屼緥濡傦紝涓寸晫鍖哄繀椤婚伩鍏嶅垎閰嶅唴瀛樸€傚洜姝わ紝鍦ㄩ潪 PREEMPT_RT 鍐呮牳涓婏紝浠ヤ笅浠ｇ爜
```

  raw_spin_lock(&lock);
  p = kmalloc(sizeof(*p), GFP_ATOMIC);

```
浣嗗湪 PREEMPT_RT 鍐呮牳涓婅繖娈典唬鐮佷細澶辫触锛屽洜涓哄唴瀛樺垎閰嶅櫒鏄畬鍏ㄥ彲鎶㈠崰鐨勶紝鍥犳鏃犳硶浠庣湡姝ｇ殑鍘熷瓙涓婁笅鏂囪皟鐢ㄣ€?鐒惰€岋紝鍦ㄦ寔鏈夊父瑙勭殑闈?raw 鑷棆閿佹椂璋冪敤鍐呭瓨鍒嗛厤鍣ㄦ槸瀹屽叏娌￠棶棰樼殑锛屽洜涓哄畠浠笉浼氱鐢?```

  spin_lock(&lock);
  p = kmalloc(sizeof(*p), GFP_ATOMIC);


```
### 浣嶈嚜鏃嬮攣

PREEMPT_RT 鏃犳硶鏇挎崲浣嶈嚜鏃嬮攣锛屽洜涓哄崟涓綅澶皬锛屽绾充笉涓?RT-mutex銆傚洜姝わ紝浣嶈嚜鏃嬮攣鐨勮涔夊湪 PREEMPT_RT
鍐呮牳涓婅淇濈暀锛屼粠鑰屼娇寰?raw_spinlock_t 鐨勬敞鎰忎簨椤瑰悓鏍烽€傜敤浜庝綅鑷棆閿併€?
涓€浜涗綅鑷棆閿佸湪 PREEMPT_RT 涓嬭鏇挎崲涓哄父瑙勭殑 spinlock_t锛岃繖閫氳繃鍦ㄨ皟鐢ㄧ偣浣跨敤鏉′欢锛?ifdef锛変唬鐮佸彉鏇?鏉ュ疄鐜般€傜浉姣斾箣涓嬶紝spinlock_t 鐨勬浛鎹笉闇€瑕佽皟鐢ㄧ偣鍙樻洿銆傜浉鍙嶏紝澶存枃浠朵腑鐨勬潯浠跺垽鏂拰鏍稿績閿佸疄鐜颁娇寰楃紪璇戝櫒
鑳藉閫忔槑鍦板畬鎴愭浛鎹€?

## 閿佺被鍨嬪祵濂楄鍒?
鏈€鍩烘湰鐨勮鍒欐槸锛?
  - 鍚屼竴閿佺被鍒紙鐫＄湢銆丆PU 鏈湴銆佽嚜鏃嬶級鐨勯攣绫诲瀷鍙互浠绘剰宓屽锛屽彧瑕佸畠浠伒瀹堥€氱敤鐨勯攣鎺掑簭瑙勫垯浠ラ槻姝㈡閿併€?
  - 鐫＄湢閿佺被鍨嬩笉鑳藉祵濂楀湪 CPU 鏈湴閿佸拰鑷棆閿佺被鍨嬪唴閮ㄣ€?
  - CPU 鏈湴閿佸拰鑷棆閿佺被鍨嬪彲浠ュ祵濂楀湪鐫＄湢閿佺被鍨嬪唴閮ㄣ€?
  - 鑷棆閿佺被鍨嬪彲浠ュ祵濂楀湪鎵€鏈夐攣绫诲瀷鍐呴儴

杩欎簺绾︽潫鍦?PREEMPT_RT 鍜屽叾瀹冩儏鍐典笅閮介€傜敤銆?
PREEMPT_RT 灏?spinlock_t 鍜?rwlock_t 鐨勯攣绫诲埆浠庤嚜鏃嬫敼涓虹潯鐪狅紝骞跺皢 local_lock 鏇挎崲涓烘瘡-CPU 鐨?spinlock_t锛岃繖鎰忓懗鐫€瀹冧滑涓嶈兘鍦ㄦ寔鏈?raw spinlock 鏃惰幏鍙栥€傝繖瀵艰嚧浠ヤ笅宓屽椤哄簭锛?
  1) 鐫＄湢閿?  2) spinlock_t銆乺wlock_t銆乴ocal_lock
  3) raw_spinlock_t 鍜屼綅鑷棆閿?
濡傛灉杩濆弽杩欎簺绾︽潫锛宭ockdep 浼氬湪 PREEMPT_RT 鍜屽叾瀹冩儏鍐典笅閮藉彂鍑哄憡璀︺€?