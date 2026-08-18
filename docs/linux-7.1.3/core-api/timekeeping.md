## ktime 璁块棶鍣?

璁惧椹卞姩鍙互浣跨敤 ktime_get() 浠ュ強 linux/timekeeping.h 涓０鏄庣殑璁稿鐩稿叧鍑芥暟鏉ヨ鍙?褰撳墠鏃堕棿銆備綔涓虹粡楠屾硶鍒欙紝濡傛灉涓や釜璁块棶鍣ㄥ鏌愪釜鐗瑰畾鐢ㄤ緥鍚屾牱閫傜敤锛屽簲浼樺厛浣跨敤鍚嶅瓧鏇寸煭鐨?閭ｄ釜銆?
### 鍩轰簬鍩烘湰 ktime_t 鐨勬帴鍙?

鎺ㄨ崘鐨勬渶绠€鍗曞舰寮忚繑鍥炰竴涓笉閫忔槑鐨?ktime_t锛屽苟甯︽湁涓轰笉鍚屾椂閽熷弬鑰冭繑鍥炴椂闂寸殑鍙樹綋锛?

	CLOCK_MONOTONIC

	閫傜敤浜庡彲闈犵殑 timestamps 浠ュ強鍑嗙‘娴嬮噺鐭椂闂撮棿闅斻€備粠绯荤粺鍚姩鏃跺紑濮嬭鏃讹紝浣嗗湪
	鎸傝捣锛坰uspend锛夋湡闂村仠姝€?

	CLOCK_BOOTTIME

	绫讳技 ktime_get()锛屼絾鍦ㄦ寕璧锋椂涓嶄細鍋滄銆傝繖鍙敤浜庝緥濡傞渶瑕佷笌鍏跺畠鏈哄櫒璺ㄦ寕璧锋搷浣?	淇濇寔鍚屾鐨勫瘑閽ヨ繃鏈熸椂闂淬€?

	CLOCK_REALTIME

	杩斿洖鐩稿浜庡浜?1970 骞寸殑 UNIX 绾厓锛坋poch锛夌殑鏃堕棿锛屼娇鐢ㄥ崗璋冧笘鐣屾椂锛圲TC锛夛紝涓?	鐢ㄦ埛绌洪棿鐨?gettimeofday() 鐩稿悓銆傝繖鐢ㄤ簬鎵€鏈夐渶瑕佽法閲嶅惎淇濇寔鐨?timestamps锛屼緥濡?	inode 鏃堕棿锛屼絾搴旈伩鍏嶇敤浜庡唴閮ㄧ敤閫旓紝鍥犱负瀹冨彲鑳藉洜闂扮鏇存柊銆丯TP 璋冩暣鎴栨潵鑷敤鎴风┖闂寸殑
	settimeofday() 鎿嶄綔鑰屽悜鍚庤烦鍙樸€?

	 CLOCK_TAI

	绫讳技 ktime_get_real()锛屼絾浣跨敤鍥介檯鍘熷瓙鏃讹紙TAI锛夊弬鑰冭€岄潪 UTC锛屼互閬垮厤鍦ㄩ棸绉掓洿鏂版椂
	璺冲彉銆傝繖鍦ㄥ唴鏍镐腑寰堝皯鏈夌敤銆?

	CLOCK_MONOTONIC_RAW

	绫讳技 ktime_get()锛屼絾浠ヤ笌纭欢 clocksource 鐩稿悓鐨勯€熺巼杩愯锛屼笉鍋氾紙NTP锛夋椂閽熸紓绉?	璋冩暣銆傚湪鍐呮牳涓篃寰堝皯闇€瑕併€?
### 绾崇銆乼imespec64 鍜岀杈撳嚭


瀵逛簬涓婅堪鎵€鏈夋帴鍙ｏ紝閮芥湁鏍规嵁璋冪敤鑰呴渶姹備互涓嶅悓鏍煎紡杩斿洖鏃堕棿鐨勫彉浣擄細

		u64 ktime_get_boottime_ns( void )
		u64 ktime_get_real_ns( void )
		u64 ktime_get_clocktai_ns( void )
		u64 ktime_get_raw_ns( void )

	涓庝笂杩版櫘閫氱殑 ktime_get 鍑芥暟鐩稿悓锛屼絾杩斿洖鐩稿簲鏃堕棿鍙傝€冧笅鐨?u64 绾崇鏁帮紝瀵规煇浜?	璋冪敤鑰呭彲鑳芥洿鏂逛究銆?
		void ktime_get_boottime_ts64( struct timespec64 * )
		void ktime_get_real_ts64( struct timespec64 * )
		void ktime_get_clocktai_ts64( struct timespec64 * )
		void ktime_get_raw_ts64( struct timespec64 * )

	涓庝笂杩扮浉鍚岋紝浣嗕互 鈥榮truct timespec64鈥?褰㈠紡杩斿洖鏃堕棿锛屾媶鍒嗕负绉掑拰绾崇銆傝繖鍙互鍦?	鎵撳嵃鏃堕棿锛屾垨灏嗘椂闂翠紶鍏ユ湡鏈?鈥榯imespec鈥?鎴?鈥榯imeval鈥?缁撴瀯鐨勫閮ㄦ帴鍙ｆ椂閬垮厤涓€娆￠澶?	鐨勯櫎娉曘€?
		time64_t ktime_get_boottime_seconds( void )
		time64_t ktime_get_real_seconds( void )
		time64_t ktime_get_clocktai_seconds( void )
		time64_t ktime_get_raw_seconds( void )

	浠ユ爣閲?time64_t 褰㈠紡杩斿洖涓€涓矖绮掑害锛坈oarse-grained锛夌殑鏃堕棿鐗堟湰銆傝繖閬垮厤浜嗚闂椂閽?	纭欢锛屽苟浣跨敤鐩稿簲鍙傝€冨皢绉掑悜涓嬪彇鏁村埌涓婁竴涓畾鏃跺櫒鑺傛媿锛坱imer tick锛夌殑瀹屾暣绉掓暟銆?
### 绮楃矑搴︿笌 fast_ns 璁块棶


杩樻湁涓€浜涚敤浜庢洿涓撻棬鍦烘櫙鐨勫彉浣擄細

		ktime_t ktime_get_coarse_boottime( void )
		ktime_t ktime_get_coarse_real( void )
		ktime_t ktime_get_coarse_clocktai( void )

		u64 ktime_get_coarse_boottime_ns( void )
		u64 ktime_get_coarse_real_ns( void )
		u64 ktime_get_coarse_clocktai_ns( void )

		void ktime_get_coarse_boottime_ts64( struct timespec64 * )
		void ktime_get_coarse_real_ts64( struct timespec64 * )
		void ktime_get_coarse_clocktai_ts64( struct timespec64 * )

	杩欎簺姣旈潪绮楃矑搴︾増鏈洿蹇紝浣嗙簿搴﹁緝浣庯紝瀵瑰簲浜庣敤鎴风┖闂翠腑鐨?CLOCK_MONOTONIC_COARSE
	鍜?CLOCK_REALTIME_COARSE锛屼互鍙婄敤鎴风┖闂翠腑涓嶅彲鐢ㄧ殑绛夋晥 boottime/tai/raw 鏃跺熀銆?
	杩欓噷杩斿洖鐨勬椂闂村搴斾簬涓婁竴涓畾鏃跺櫒鑺傛媿锛屽湪杩囧幓鍙兘闀胯揪 10ms锛堝浜?CONFIG_HZ=100锛夛紝
	涓庤鍙?鈥榡iffies鈥?鍙橀噺鐩稿悓銆傝繖浜涗粎鍦ㄥ鏃舵晥鎬ц姹傞珮锛坒ast path锛変笖浠嶆湡鏈涗紭浜庣绾?	绮惧害銆佷絾鍙堟棤娉曡交鏉句娇鐢?鈥榡iffies鈥?鐨勬儏鍐典笅鏈夌敤锛屼緥濡傜敤浜?inode 鏃堕棿鎴炽€傝烦杩囩‖浠?	鏃堕挓璁块棶鍦ㄧ幇浠ｅぇ澶氭暟甯︽湁鍙潬鍛ㄦ湡璁℃暟鍣ㄧ殑鏈哄櫒涓婂彲鑺傜渷绾?100 涓?CPU 鍛ㄦ湡锛屼絾鍦ㄥ甫鏈?	澶栭儴 clocksource 鐨勮緝鏃х‖浠朵笂鏈€澶氬彲杈炬暟寰銆?
		u64 ktime_get_raw_fast_ns( void )
		u64 ktime_get_boot_fast_ns( void )
		u64 ktime_get_tai_fast_ns( void )
		u64 ktime_get_real_fast_ns( void )

	杩欎簺鍙樹綋鍙互瀹夊叏鍦颁粠浠讳綍涓婁笅鏂囦腑璋冪敤锛屽寘鎷湪 timekeeper 鏇存柊鏈熼棿鐨勪笉鍙睆钄戒腑鏂?	锛圢MI锛変腑锛屼互鍙婂湪鎴戜滑杩涘叆鎸傝捣涓?clocksource 鏂數鏃躲€傝繖鍦ㄤ竴浜涜窡韪垨璋冭瘯浠ｇ爜浠ュ強
	鏈哄櫒妫€鏌ワ紙machine check锛夋姤鍛婁腑寰堟湁鐢紝浣嗗ぇ澶氭暟椹卞姩缁濅笉搴旇皟鐢ㄥ畠浠紝鍥犱负璇ユ椂闂?	鍦ㄦ煇浜涙潯浠朵笅鍏佽璺冲彉銆?
### 宸插簾寮冪殑鏃堕棿鎺ュ彛


杈冩棫鐨勫唴鏍镐娇鐢ㄤ簡涓€浜涘叾瀹冩帴鍙ｏ紝鐜板湪姝ｅ湪閫愭娣樻卑锛屼絾鍙兘鍑虹幇鍦ㄨ绉绘鍒拌繖閲岀殑涓夋柟椹卞姩
涓€傜壒鍒槸锛屾墍鏈夎繑鍥?鈥榮truct timeval鈥?鎴?鈥榮truct timespec鈥?鐨勬帴鍙ｉ兘宸茶鏇挎崲锛屽洜涓?鍦?32 浣嶄綋绯荤粨鏋勪笂 tv_sec 鎴愬憳浼氬湪 2038 骞存孩鍑恒€備互涓嬫槸鎺ㄨ崘鐨勬浛鎹細


	浣跨敤 ktime_get() 鎴?ktime_get_ts64() 浠ｆ浛銆?
		void getnstimeofday( struct timespec * )
		void getnstimeofday64( struct timespec64 * )
		void ktime_get_real_ts( struct timespec * )

	ktime_get_real_ts64() 鏄洿鎺ユ浛鎹紝浣嗚€冭檻浣跨敤鍗曡皟鏃堕棿锛坘time_get_ts64()锛夊拰/鎴栧熀浜?	ktime_t 鐨勬帴鍙ｏ紙ktime_get()/ktime_get_real()锛夈€?
		struct timespec64 current_kernel_time64( void )
		struct timespec get_monotonic_coarse( void )
		struct timespec64 get_monotonic_coarse64( void )

	杩欎簺琚?ktime_get_coarse_real_ts64() 鍜?ktime_get_coarse_ts64() 鏇挎崲銆傜劧鑰岋紝璁稿
	闇€瑕佺矖绮掑害鏃堕棿鐨勪唬鐮佸彲浠ユ敼鐢ㄧ畝鍗曠殑 鈥榡iffies鈥欙紝鑰屽浠婁竴浜涢┍鍔ㄥ彲鑳藉疄闄呬笂鎯宠鏇撮珮
	鍒嗚鲸鐜囩殑璁块棶鍣ㄣ€?
		struct timespec64 getrawmonotonic64( void )
		struct timespec timekeeping_clocktai( void )
		struct timespec64 timekeeping_clocktai64( void )
		struct timespec get_monotonic_boottime( void )
		struct timespec64 get_monotonic_boottime64( void )

	杩欎簺琚?ktime_get_raw()/ktime_get_raw_ts64()銆乲time_get_clocktai()/
	ktime_get_clocktai_ts64() 浠ュ強 ktime_get_boottime()/ktime_get_boottime_ts64() 鏇挎崲銆?	鐒惰€岋紝濡傛灉鐢ㄦ埛骞朵笉鍦ㄦ剰鏃堕挓婧愮殑鍏蜂綋閫夋嫨锛屼负浜嗕竴鑷存€ц€冭檻鏀圭敤 ktime_get()/
	ktime_get_ts64()銆?
