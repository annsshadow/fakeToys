## Futex Requeue PI


灏嗕换鍔′粠闈?PI futex 閲嶆柊鎺掗槦锛坮equeue锛夊埌 PI futex 闇€瑕佺壒娈婂鐞嗭紝浠ョ‘淇濆簳灞傜殑
rt_mutex 鍦ㄦ湁绛夊緟鑰呮椂姘歌繙涓嶄細娌℃湁鎷ユ湁鑰咃紱鍚﹀垯浼氱牬鍧?PI 鎻愬崌閫昏緫 [see rt-mutex-design.rst]銆?
涓虹畝娲佽捣瑙侊紝鏈枃妗ｄ腑灏嗚鎿嶄綔缁熶竴绉颁负 "requeue_pi"銆備紭鍏堢骇缁ф壙鍦ㄥ叏鏂囩缉鍐欎负 "PI"銆?

### Motivation


濡傛灉娌℃湁 requeue_pi锛宲thread_cond_broadcast() 鐨?glibc 瀹炵幇灏卞繀椤诲敜閱掓墍鏈夌瓑寰呭湪鏌愪釜
pthread_condvar 涓婄殑浠诲姟锛屽啀璁╁畠浠嚜琛屼簤鎶㈣皝鍏堣繍琛岋紝褰㈡垚缁忓吀鐨勬儕缇わ紙thundering-herd锛?
灞€闈€傜悊鎯崇殑瀹炵幇搴斿綋鍞ら啋浼樺厛绾ф渶楂樼殑绛夊緟鑰咃紝鑰屽叾浣欑殑鍒欎氦鐢变笌 condvar 鐩稿叧鑱旂殑浜掓枼浣?
瑙ｉ攣鏃跺浐鏈夌殑鑷劧鍞ら啋鏉ュ鐞嗐€?

```

	/* caller must lock mutex */
	pthread_cond_wait(cond, mutex)
	{
		lock(cond->__data.__lock);
		unlock(mutex);
		do {
		unlock(cond->__data.__lock);
		futex_wait(cond->__data.__futex);
		lock(cond->__data.__lock);
		} while(...)
		unlock(cond->__data.__lock);
		lock(mutex);
	}

	pthread_cond_broadcast(cond)
	{
		lock(cond->__data.__lock);
		unlock(cond->__data.__lock);
		futex_requeue(cond->data.__futex, cond->mutex);
	}

```
涓€鏃?pthread_cond_broadcast() 閲嶆柊鎺掗槦浜嗚繖浜涗换鍔★紝cond->mutex 灏辨湁浜嗙瓑寰呰€呫€傛敞鎰?
pthread_cond_wait() 鍙湁鍦ㄨ繑鍥炲埌鐢ㄦ埛绌洪棿涔嬪悗鎵嶄細灏濊瘯閿佸畾璇ヤ簰鏂ヤ綋銆傝繖灏嗕娇搴曞眰鐨?rt_mutex
澶勪簬鏈夌瓑寰呰€呭嵈娌℃湁鎷ユ湁鑰呯殑鐘舵€侊紝浠庤€岀牬鍧忎簡鍓嶉潰鎻愬埌鐨?PI 鎻愬崌绠楁硶銆?

涓轰簡鏀寔鎰熺煡 PI 鐨?pthread_condvar锛屽唴鏍搁渶瑕佽兘澶熸妸浠诲姟閲嶆柊鎺掗槦鍒?PI futex銆傝繖绉嶆敮鎸?
鎰忓懗鐫€锛屽湪涓€娆℃垚鍔熺殑 futex_wait 绯荤粺璋冪敤涔嬪悗锛岃皟鐢ㄨ€呰繑鍥炵敤鎴风┖闂存椂宸茬粡鎸佹湁浜嗚 PI futex銆?
glibc 鐨勫疄鐜?
```


	/* caller must lock mutex */
	pthread_cond_wait_pi(cond, mutex)
	{
		lock(cond->__data.__lock);
		unlock(mutex);
		do {
		unlock(cond->__data.__lock);
		futex_wait_requeue_pi(cond->__data.__futex);
		lock(cond->__data.__lock);
		} while(...)
		unlock(cond->__data.__lock);
		/* the kernel acquired the mutex for us */
	}

	pthread_cond_broadcast_pi(cond)
	{
		lock(cond->__data.__lock);
		unlock(cond->__data.__lock);
		futex_requeue_pi(cond->data.__futex, cond->mutex);
	}

```
瀹為檯鐨?glibc 瀹炵幇寰堝彲鑳戒細瀵?PI 杩涜娴嬭瘯锛屽苟鍦ㄧ幇鏈夎皟鐢ㄥ唴閮ㄥ仛蹇呰鐨勪慨鏀癸紝鑰屼笉鏄负 PI 鍦烘櫙
鏂板缓璋冪敤銆俻thread_cond_timedwait() 鍜?pthread_cond_signal() 涔熼渶瑕佺被浼肩殑淇敼銆?

### Implementation


涓轰簡纭繚 rt_mutex 鍦ㄦ湁绛夊緟鑰呮椂鎷ユ湁鎷ユ湁鑰咃紝閲嶆柊鎺掗槦浠ｇ爜浠ュ強绛夊緟浠ｇ爜閮藉繀椤昏兘澶熷湪杩斿洖鐢ㄦ埛
绌洪棿涔嬪墠鑾峰彇璇?rt_mutex銆傞噸鏂版帓闃熶唬鐮佷笉鑳界畝鍗曞湴鍞ら啋绛夊緟鑰咃紝鐒跺悗浠荤敱鍏跺幓鑾峰彇 rt_mutex锛?
鍥犱负閭ｆ牱浼氬湪閲嶆柊鎺掗槦璋冪敤杩斿洖鐢ㄦ埛绌洪棿涓庣瓑寰呰€呰鍞ら啋骞跺紑濮嬭繍琛屼箣闂存墦寮€涓€涓珵鎬佺獥鍙ｃ€傚湪
鏃犵珵浜夌殑鎯呭喌涓嬪挨鍏跺姝ゃ€?

瑙ｅ喅鏂规寮曞叆浜嗕袱涓柊鐨?rt_mutex 杈呭姪渚嬬▼锛宺t_mutex_start_proxy_lock() 鍜?
rt_mutex_finish_proxy_lock()锛屽畠浠厑璁搁噸鏂版帓闃熶唬鐮佷唬琛ㄧ瓑寰呰€呰幏鍙栦竴涓棤绔炰簤鐨?rt_mutex锛?
骞舵妸绛夊緟鑰呮帓鍏ヤ竴涓湁绔炰簤鐨?rt_mutex 鐨勭瓑寰呴槦鍒椼€備袱涓柊鐨勭郴缁熻皟鐢ㄦ彁渚涗簡鍐呮牳涓庣敤鎴风┖闂翠箣闂?
鐢ㄤ簬 requeue_pi 鐨勬帴鍙ｏ細FUTEX_WAIT_REQUEUE_PI 鍜?FUTEX_CMP_REQUEUE_PI銆?

FUTEX_WAIT_REQUEUE_PI 鐢辩瓑寰呰€咃紙pthread_cond_wait() 鍜?pthread_cond_timedwait()锛夎皟鐢紝
鐢ㄤ簬闃诲鍦ㄥ垵濮?futex 涓婂苟绛夊緟琚噸鏂版帓闃熷埌涓€涓劅鐭?PI 鐨?futex銆傚叾瀹炵幇鏄?futex_wait() 涓?
futex_lock_pi() 楂橀€熺鎾炵殑缁撴灉锛屽苟鍔犲叆浜嗕竴浜涢澶栫殑閫昏緫鏉ュ鐞嗛偅浜涢澶栫殑鍞ら啋鍦烘櫙銆?

FUTEX_CMP_REQUEUE_PI 鐢卞敜閱掕€咃紙pthread_cond_broadcast() 鍜?pthread_cond_signal()锛夎皟鐢紝
鐢ㄤ簬閲嶆柊鎺掗槦骞跺彲鑳藉敜閱掔瓑寰呯殑浠诲姟銆傚湪鍐呴儴锛岃绯荤粺璋冪敤浠嶇劧鐢?futex_requeue 澶勭悊锛堥€氳繃浼犲叆
requeue_pi=1锛夈€傚湪閲嶆柊鎺掗槦涔嬪墠锛宖utex_requeue() 浼氫唬琛ㄦ渶椤剁鐨勭瓑寰呰€呭皾璇曡幏鍙栭噸鏂版帓闃熺洰鏍囩殑
PI futex銆傚鏋滄垚鍔燂紝璇ョ瓑寰呰€呰鍞ら啋銆傞殢鍚?futex_requeue() 缁х画鎶婂墿涓嬬殑 nr_wake+nr_requeue
涓换鍔￠噸鏂版帓闃熷埌 PI futex锛屽湪姣忔閲嶆柊鎺掗槦鍓嶈皟鐢?rt_mutex_start_proxy_lock() 浠ュ皢璇ヤ换鍔″噯澶?
涓哄簳灞?rt_mutex 涓婄殑涓€涓瓑寰呰€呫€傚湪杩欎竴闃舵涔熸湁鍙兘鑾峰彇鍒伴攣锛屽鏋滄槸杩欐牱锛屼笅涓€涓瓑寰呰€呬細琚?
鍞ら啋浠ュ畬鎴愰攣鐨勮幏鍙栥€?

FUTEX_CMP_REQUEUE_PI 鎺ュ彈 nr_wake 鍜?nr_requeue 浣滀负鍙傛暟锛屼絾鐪熸閲嶈鐨勫彧鏄畠浠殑鍜屻€?
futex_requeue() 浼氬敜閱掓垨閲嶆柊鎺掗槦鏈€澶?nr_wake + nr_requeue 涓换鍔°€傚畠鍙細鍞ら啋瀹冭兘澶熶负鍏惰幏鍙栭攣
鐨勯偅涔堝浠诲姟锛岃€屽湪澶у鏁版儏鍐典笅锛岃繖涓暟瀛楀簲褰撴槸 0锛屽洜涓鸿壇濂界殑缂栫▼瀹炶返瑕佹眰 pthread_cond_broadcast()
鎴?pthread_cond_signal() 鐨勮皟鐢ㄨ€呭湪鍙戣捣璋冪敤涔嬪墠鍏堣幏鍙栦簰鏂ヤ綋銆侳UTEX_CMP_REQUEUE_PI 瑕佹眰 nr_wake=1銆?
瀵逛簬 broadcast锛宯r_requeue 搴斾负 INT_MAX锛涘浜?signal锛屽簲涓?0銆?
