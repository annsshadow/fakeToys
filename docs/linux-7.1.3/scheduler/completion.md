## Completions - 鈥滅瓑寰呭畬鎴愨€濆睆闅?API

### 绠€浠嬶細

濡傛灉浣犳湁涓€涓垨澶氫釜绾跨▼蹇呴』绛夊緟鏌愪簺鍐呮牳娲诲姩鍒拌揪鏌愪釜鐐规垨鐗瑰畾鐘舵€侊紝completions锛堝畬鎴愰噺锛夊彲浠ヤ负杩欎竴闂鎻愪緵涓€涓棤绔炴€侊紙race-free锛夌殑瑙ｅ喅鏂规銆備粠璇箟涓婅锛屽畠浠湁浜涚被浼?`pthread_barrier()`锛屽苟涓斿叿鏈夌浉浼肩殑鐢ㄤ緥銆?
Completions 鏄竴绉嶄唬鐮佸悓姝ユ満鍒讹紝姣斾换浣曞閿?淇″彿閲忕殑璇敤浠ュ強蹇欏惊鐜兘瑕佸彲鍙栥€備换浣曟椂鍊欏綋浣犳兂浣跨敤 `yield()` 鎴栨煇绉嶅彜鎬殑 `msleep(1)` 寰幆鏉ヨ鍏朵粬浜嬫儏缁х画鎺ㄨ繘鏃讹紝浣犲彲鑳藉簲褰撹€冭檻鏀圭敤鍏朵腑涓€涓?`wait_for_completion*()` 璋冪敤鍜?`complete()`銆?
浣跨敤 completions 鐨勫ソ澶勫湪浜庡畠浠叿鏈夊畾涔夎壇濂姐€佺洰鏍囦笓涓€鐨勭敤閫旓紝杩欎娇寰椾唬鐮佺殑鎰忓浘闈炲父瀹规槗鐪嬫竻锛岃€屼笖瀹冧滑杩樿兘浜х敓鏇撮珮鏁堢殑浠ｇ爜锛屽洜涓烘墍鏈夌嚎绋嬮兘鍙互涓€鐩存墽琛屼笅鍘伙紝鐩村埌鐪熸闇€瑕佺粨鏋滄椂涓烘锛屽苟涓旂瓑寰呬笌淇″彿閫氱煡閮介€氳繃搴曞眰鐨勮皟搴﹀櫒鐫＄湢/鍞ら啋璁炬柦瀹炵幇浜嗘瀬楂樼殑鏁堢巼銆?
Completions 鏋勫缓浜?Linux 璋冨害鍣ㄧ殑绛夊緟闃熷垪锛坵aitqueue锛夊拰鍞ら啋鍩虹璁炬柦涔嬩笂銆傜瓑寰呴槦鍒椾笂绾跨▼浠墍绛夊緟鐨勪簨浠惰鍖栫畝涓?`struct completion` 涓殑涓€涓畝鍗曟爣蹇楋紝鎭板綋鍦扮О涔嬩负鈥渄one鈥濄€?
鐢变簬 completions 涓庤皟搴︾浉鍏筹紝鐩稿叧浠ｇ爜鍙互鍦?kernel/sched/completion.c 涓壘鍒般€?
### 鐢ㄦ硶锛?
浣跨敤 completions 鏈変笁涓富瑕侀儴鍒嗭細

 - `struct completion` 鍚屾瀵硅薄鐨勫垵濮嬪寲
 - 閫氳繃璋冪敤 `wait_for_completion()` 鐨勬煇涓彉浣撹繘琛岀殑绛夊緟閮ㄥ垎
 - 閫氳繃璋冪敤 `complete()` 鎴?`complete_all()` 杩涜鐨勪俊鍙烽€氱煡閮ㄥ垎

鍙﹀杩樻湁涓€浜涜緟鍔╁嚱鏁扮敤浜庢鏌?completions 鐨勭姸鎬併€傛敞鎰忥紝铏界劧鍒濆鍖栧繀椤绘渶鍏堝彂鐢燂紝浣嗙瓑寰呭拰淇″彿閫氱煡閮ㄥ垎鍙互浠ヤ换鎰忛『搴忓彂鐢熴€傚嵆锛屽湪鍙︿竴涓嚎绋嬫鏌ュ畠鏄惁闇€瑕佺瓑寰呬箣鍓嶏紝鏌愪釜绾跨▼灏卞凡缁忓皢涓€涓?completion 鏍囪涓衡€渄one鈥濇槸瀹屽叏姝ｅ父鐨勩€?
瑕佷娇鐢?completions锛屼綘闇€瑕?`#include <linux/completion.h>` 骞跺垱寤轰竴涓?`struct completion` 绫诲瀷鐨勯潤鎬佹垨鍔ㄦ€佸彉閲忥紝
```

	struct completion {
		unsigned int done;
		struct swait_queue_head wait;
	};

```
杩欎负绛夊緟锛堝鏈夛級鐨勪换鍔℃彁渚涗簡 ->wait 绛夊緟闃熷垪锛屼互鍙婄敤浜庢寚绀烘槸鍚﹀畬鎴愮殑 ->done 瀹屾垚鏍囧織銆?
Completions 鐨勫懡鍚嶅簲褰撴寚鍚戞鍦ㄨ鍚屾鐨勪簨浠躲€?```

	wait_for_completion(&early_console_added);

	complete(&early_console_added);

```
濂界殑銆佺洿瑙傜殑鍛藉悕锛堜竴濡傛棦寰€锛夋湁鍔╀簬浠ｇ爜鍙鎬с€傚皢涓€涓?completion 鍛藉悕涓?'complete' 鏄病鏈夊府鍔╃殑锛岄櫎闈炲叾鐢ㄩ€旀瀬鍏舵槑鏄锯€︹€?
### 鍒濆鍖?completions锛?
鍔ㄦ€佸垎閰嶇殑 completion 瀵硅薄鏈€濂藉唴宓屽湪鑳藉淇濊瘉鍦ㄥ叾鍑芥暟/椹卞姩鐢熷懡鍛ㄦ湡鍐呭瓨娲荤殑鏁版嵁缁撴瀯涓紝浠ラ槻姝笌寮傛鐨?`complete()` 璋冪敤鍙戠敓绔炴€併€?
鍦ㄤ娇鐢?`wait_for_completion()` 鐨?`_timeout()` 鎴?`_killable()`/`_interruptible()` 鍙樹綋鏃跺簲褰撶壒鍒皬蹇冿紝鍥犱负蹇呴』淇濊瘉鍐呭瓨閲婃斁涓嶄細鍙戠敓鍦ㄦ墍鏈夌浉鍏虫椿鍔紙`complete()` 鎴?`reinit_completion()`锛夊畬鎴愪箣鍓嶏紝鍗充娇杩欎簺绛夊緟鍑芥暟鐢变簬瓒呮椂鎴栦俊鍙疯Е鍙戣€屾彁鍓嶈繑鍥炪€?
鍔ㄦ€佸垎閰嶇殑 completion 瀵硅薄鐨勫垵濮嬪寲閫氳繃璋冪敤浠ヤ笅鍑芥暟瀹屾垚
```

	init_completion(&dynamic_object->done);

```
鍦ㄦ璋冪敤涓紝鎴戜滑鍒濆鍖栫瓑寰呴槦鍒楀苟灏?->done 璁剧疆涓?0锛屽嵆鈥滄湭瀹屾垚鈥濇垨鈥滄湭瀹屾垚鈥濄€?
閲嶆柊鍒濆鍖栧嚱鏁?`reinit_completion()` 鍙槸灏?->done 瀛楁閲嶇疆涓?0锛堚€滄湭瀹屾垚鈥濓級锛岃€屼笉瑙︾绛夊緟闃熷垪銆傛鍑芥暟鐨勮皟鐢ㄨ€呭繀椤荤‘淇濇病鏈夊苟鍙戠殑 `wait_for_completion()` 璋冪敤鍦ㄥ苟琛岃繘琛屻€?
瀵瑰悓涓€涓?completion 瀵硅薄璋冪敤 `init_completion()` 涓ゆ鏋佹湁鍙兘鏄竴涓?bug锛屽洜涓哄畠灏嗛槦鍒楅噸鏂板垵濮嬪寲涓虹┖闃熷垪锛岃€屽凡鍏ラ槦鐨勪换鍔″彲鑳戒細鈥滀涪澶扁€濃€斺€斿湪杩欑鎯呭喌涓嬪簲浣跨敤 `reinit_completion()`锛屼絾涔熻娉ㄦ剰鍏朵粬绔炴€併€?
瀵逛簬闈欐€佸０鏄庡拰鍒濆鍖栵紝鎻愪緵浜嗗畯銆?
瀵逛簬鏂囦欢浣滅敤鍩熶腑鐨勯潤鎬侊紙鎴栧叏灞€锛夊０鏄庯紝浣犲彲浠ヤ娇鐢?```

	static DECLARE_COMPLETION(setup_done);
	DECLARE_COMPLETION(setup_done);

```
娉ㄦ剰锛屽湪杩欑鎯呭喌涓嬶紝completion 鍦ㄥ惎鍔ㄦ椂闂达紙鎴栨ā鍧楀姞杞芥椂闂达級琚垵濮嬪寲涓衡€滄湭瀹屾垚鈥濓紝涓嶉渶瑕?`init_completion()` 璋冪敤銆?
褰撲竴涓?completion 琚０鏄庝负鍑芥暟鍐呯殑灞€閮ㄥ彉閲忔椂锛岄偅涔堝垵濮嬪寲搴斿綋濮嬬粓鏄惧紡浣跨敤 `DECLARE_COMPLETION_ONSTACK()`锛岃繖涓嶄粎鏄负浜嗚 lockdep 婊℃剰锛屼篃鏄负浜嗘槑纭?```

	DECLARE_COMPLETION_ONSTACK(setup_done)

```
娉ㄦ剰锛屽綋灏?completion 瀵硅薄鐢ㄤ綔灞€閮ㄥ彉閲忔椂锛屼綘蹇呴』鏁忛攼鍦版剰璇嗗埌鍑芥暟鏍堢殑鐭殏鐢熷懡鍛ㄦ湡锛氬湪鎵€鏈夋椿鍔紙渚嬪绛夊緟绾跨▼锛夊仠姝笖 completion 瀵硅薄瀹屽叏涓嶅啀琚娇鐢ㄤ箣鍓嶏紝鍑芥暟涓嶅緱杩斿洖鍒拌皟鐢ㄤ笂涓嬫枃銆?
鍐嶆寮鸿皟杩欎竴鐐癸細鐗瑰埆鏄綋浣跨敤涓€浜涘叿鏈夋洿澶嶆潅缁撴灉鐨勭瓑寰?API 鍙樹綋鏃讹紝渚嬪瓒呮椂鎴栦俊鍙烽€氱煡锛坄_timeout()`銆乣_killable()` 鍜?`_interruptible()`锛夊彉浣擄紝绛夊緟鍙兘浼氭彁鍓嶅畬鎴愶紝鑰岃瀵硅薄鍙兘浠嶈鍙︿竴涓嚎绋嬩娇鐢ㄢ€斺€斿鏋滄煇涓叾浠栫嚎绋嬩腑鎵ц浜?`complete()`锛岄偅涔?`wait_on_completion*()` 璋冪敤鏂瑰嚱鏁扮殑杩斿洖灏嗛噴鏀惧嚱鏁版爤骞跺鑷撮毦浠ュ療瑙夌殑鏁版嵁鎹熷潖銆傜畝鍗曠殑娴嬭瘯鍙兘鏃犳硶瑙﹀彂杩欑被绔炴€併€?
濡傛灉涓嶇‘瀹氾紝璇蜂娇鐢ㄥ姩鎬佸垎閰嶇殑 completion 瀵硅薄锛屾渶濂藉唴宓屽湪鏌愪釜鍏朵粬闀跨敓鍛藉懆鏈熺殑瀵硅薄涓紝鍏剁敓鍛藉懆鏈熶箣闀胯秴杩囦换浣曚娇鐢?completion 瀵硅薄鐨勮緟鍔╃嚎绋嬶紝鎴栬€呭叿鏈夐攣鎴栧叾浠栧悓姝ユ満鍒朵互纭繚涓嶄細瀵瑰凡閲婃斁鐨勫璞¤皟鐢?`complete()`銆?
鍦ㄦ爤涓婃湸绱犵殑 `DECLARE_COMPLETION()` 浼氳Е鍙?lockdep 璀﹀憡銆?
### 绛夊緟 completions锛?
瀵逛簬涓€涓嚎绋嬭绛夊緟鏌愪簺骞跺彂娲诲姩瀹屾垚锛屽畠
```

	void wait_for_completion(struct completion *done)

```
```

	CPU#1					CPU#2

	struct completion setup_done;

	init_completion(&setup_done);
	initialize_work(...,&setup_done,...);

	/* run non-dependent code */		/* do setup */

	wait_for_completion(&setup_done);	complete(&setup_done);

```
杩欏苟涓嶆殫绀?`wait_for_completion()` 涓庡 `complete()` 鐨勮皟鐢ㄤ箣闂村瓨鍦ㄤ换浣曠壒瀹氶『搴忊€斺€斿鏋滃 `complete()` 鐨勮皟鐢ㄥ彂鐢熷湪瀵?`wait_for_completion()` 鐨勮皟鐢ㄤ箣鍓嶏紝閭ｄ箞绛夊緟鏂瑰皢绔嬪嵆缁х画锛屽洜涓烘墍鏈変緷璧栭兘宸叉弧瓒筹紱鍚﹀垯瀹冨皢闃诲锛岀洿鍒拌 `complete()` 淇″彿閫氱煡瀹屾垚銆?
娉ㄦ剰 `wait_for_completion()` 璋冪敤浜?`spin_lock_irq()`/`spin_unlock_irq()`锛屽洜姝ゅ彧鏈夊湪浣犵‘瀹氫腑鏂凡浣胯兘鏃舵墠鑳藉畨鍏ㄨ皟鐢ㄣ€傚湪涓柇鍏抽棴锛圛RQs-off锛夌殑鍘熷瓙涓婁笅鏂囦腑璋冪敤瀹冧細瀵艰嚧闅句互妫€娴嬬殑浼腑鏂娇鑳姐€?
榛樿琛屼负鏄笉甯﹁秴鏃跺湴绛夊緟锛屽苟灏嗕换鍔℃爣璁颁负涓嶅彲涓柇锛坲ninterruptible锛夈€俙wait_for_completion()` 鍙婂叾鍙樹綋浠呭湪杩涚▼涓婁笅鏂囷紙鍥犱负瀹冧滑浼氱潯鐪狅級涓畨鍏紝鑰屽湪鍘熷瓙涓婁笅鏂囥€佷腑鏂笂涓嬫枃銆佷腑鏂绂佺敤鎴栨姠鍗犺绂佺敤鏃跺垯涓嶅畨鍏ㄢ€斺€斿叧浜庡湪鍘熷瓙/涓柇涓婁笅鏂囦腑澶勭悊 completion锛屽彟璇峰弬闃呬笅闈㈢殑 `try_wait_for_completion()`銆?
鐢变簬 `wait_for_completion()` 鐨勬墍鏈夊彉浣撻兘鍙兘锛堟樉鐒讹級鏍规嵁鎵€绛夊緟娲诲姩鐨勬€ц川闃诲寰堥暱鏃堕棿锛屽洜姝ゅ湪澶у鏁版儏鍐典笅浣犲彲鑳戒笉甯屾湜鎸佹湁鐫€浜掓枼浣擄紙mutex锛夋椂璋冪敤瀹冦€?
### 鍙敤鐨?wait_for_completion*() 鍙樹綋锛?
浠ヤ笅鍙樹綋閮借繑鍥炵姸鎬侊紝骞朵笖鍦ㄥぇ澶氭暟锛?鎵€鏈夛級鎯呭喌涓嬮兘搴旀鏌ヨ鐘舵€佲€斺€斿湪鏁呮剰涓嶆鏌ョ姸鎬佺殑鎯呭喌涓嬶紝浣犲彲鑳芥兂瑕佸啓涓€鏉℃敞閲婃潵瑙ｉ噴鍘熷洜锛堜緥濡傚弬瑙?arch/arm/kernel/smp.c:__cpu_up()锛夈€?
涓€涓父瑙佺殑鍑虹幇鐨勯棶棰樻槸杩斿洖绫诲瀷璧嬪€间笉骞插噣锛屽洜姝よ灏忓績灏嗚繑鍥炲€艰祴缁欓€傚綋绫诲瀷鐨勫彉閲忋€?
妫€鏌ヨ繑鍥炲€肩殑鐗瑰畾鍚箟涔熸浘琚彂鐜?```

	if (!wait_for_completion_interruptible_timeout(...))

```
鈥︹€︿細瀵规垚鍔熷畬鎴愬拰浠ヤ笅鎯呭喌鎵ц鐩稿悓鐨勪唬鐮佽矾寰?```

	int wait_for_completion_interruptible(struct completion *done)

```
姝ゅ嚱鏁板湪绛夊緟鏃跺皢浠诲姟鏍囪涓?TASK_INTERRUPTIBLE銆?```

	unsigned long wait_for_completion_timeout(struct completion *done, unsigned long timeout)

```
浠诲姟琚爣璁颁负 TASK_UNINTERRUPTIBLE锛屽苟涓旀渶澶氱瓑寰呪€渢imeout鈥濅釜 jiffies銆傚鏋滃彂鐢熻秴鏃讹紝瀹冭繑鍥?0锛屽惁鍒欒繑鍥炲墿浣欑殑 jiffies锛堜絾鑷冲皯涓?1锛夈€?
瓒呮椂鏈€濂戒娇鐢?`msecs_to_jiffies()` 鎴?`usecs_to_jiffies()` 璁＄畻锛屼互浣夸唬鐮佸湪寰堝ぇ绋嬪害涓婁笌 HZ 鏃犲叧銆?
濡傛灉鏁呮剰蹇界暐杩斿洖鐨勮秴鏃跺€硷紝澶ф搴斿綋鍐欎竴鏉℃敞閲婃潵瑙ｉ噴
```

	long wait_for_completion_interruptible_timeout(struct completion *done, unsigned long timeout)

```
姝ゅ嚱鏁颁紶鍏ヤ互 jiffies 涓哄崟浣嶇殑瓒呮椂锛屽苟灏嗕换鍔℃爣璁颁负 TASK_INTERRUPTIBLE銆傚鏋滄敹鍒颁俊鍙凤紝瀹冨皢杩斿洖 -ERESTARTSYS锛涘惁鍒欙紝濡傛灉 completion 瓒呮椂鍒欒繑鍥?0锛屽鏋滃彂鐢?completion 鍒欒繑鍥炲墿浣欑殑 jiffies銆?
杩涗竴姝ョ殑鍙樹綋鍖呮嫭 `_killable`锛屽畠浣跨敤 TASK_KILLABLE 浣滀负鎸囧畾鐨勪换鍔＄姸鎬侊紝濡傛灉琚腑鏂皢杩斿洖 -ERESTARTSYS锛?```

	long wait_for_completion_killable(struct completion *done)
	long wait_for_completion_killable_timeout(struct completion *done, unsigned long timeout)

```
`_io` 鍙樹綋 `wait_for_completion_io()` 鐨勮涓轰笌闈?`_io` 鍙樹綋鐩稿悓锛屽彧鏄皢绛夊緟鏃堕棿璁″叆鈥滅瓑寰?IO鈥濓紝杩欐湁
```

	void wait_for_completion_io(struct completion *done)
	unsigned long wait_for_completion_io_timeout(struct completion *done, unsigned long timeout)


```
### 淇″彿閫氱煡 completions锛?
鎯宠鍙戝嚭鈥滅户缁潯浠跺凡杈炬垚鈥濅俊鍙风殑涓€涓嚎绋嬭皟鐢?`complete()` 鏉ョ簿纭湴閫氱煡鍏朵腑涓€涓瓑寰呰€呭畠鍙互
```

	void complete(struct completion *done)

```
```

	void complete_all(struct completion *done)

```
鍗充娇 completions 鍦ㄦ煇涓€绾跨▼寮€濮嬬瓑寰呬箣鍓嶅氨琚俊鍙烽€氱煡锛屼俊鍙烽€氱煡涔熶細濡傞鏈熻埇宸ヤ綔銆傝繖鏄€氳繃绛夊緟鑰呪€滄秷璐光€濓紙閫掑噺锛塦struct completion` 鐨?done 瀛楁瀹炵幇鐨勩€傜瓑寰呯嚎绋嬬殑鍞ら啋椤哄簭涓庡畠浠叆闃熼『搴忕浉鍚岋紙FIFO 椤哄簭锛夈€?
濡傛灉澶氭璋冪敤 `complete()`锛岄偅涔堣繖灏嗗厑璁哥浉搴旀暟閲忕殑绛夊緟鑰呯户缁€斺€旀瘡娆″ `complete()` 鐨勮皟鐢ㄥ彧鏄皢 done 瀛楁鍔犱竴銆備笉杩囧娆¤皟鐢?`complete_all()` 鏄竴涓?bug銆傛棤璁烘槸 `complete()` 杩樻槸 `complete_all()` 閮藉彲浠ュ湪 IRQ/鍘熷瓙涓婁笅鏂囦腑瀹夊叏璋冪敤銆?
浠讳綍鏃跺埢鍙兘鏈変竴涓嚎绋嬭皟鐢?`complete()` 鎴?`complete_all()` 浣滅敤浜庣壒瀹氱殑 `struct completion`鈥斺€旈€氳繃绛夊緟闃熷垪鑷棆閿佷覆琛屽寲銆備换浣曟绫诲 `complete()` 鎴?`complete_all()` 鐨勫苟鍙戣皟鐢ㄩ兘鍙兘鏄璁′笂鐨?bug銆?
浠?IRQ 涓婁笅鏂囦俊鍙烽€氱煡 completion 鏄病闂鐨勶紝鍥犱负瀹冧細閫傚綋鍦颁娇鐢?`spin_lock_irqsave()`/`spin_unlock_irqrestore()` 鍔犻攣锛屽苟涓旀案杩滀笉浼氱潯鐪犮€?
### try_wait_for_completion()/completion_done()锛?
`try_wait_for_completion()` 鍑芥暟涓嶄細灏嗙嚎绋嬫斁鍒扮瓑寰呴槦鍒椾笂锛岃€屾槸濡傛灉闇€瑕佸皢绾跨▼鍏ラ槦锛堥樆濉烇級鍒欒繑鍥?false锛?```

	bool try_wait_for_completion(struct completion *done)

```
鏈€鍚庯紝瑕佸湪涓嶆敼鍙?completion 浠讳綍鐘舵€佺殑鎯呭喌涓嬫鏌ュ叾鐘舵€侊紝璋冪敤 `completion_done()`锛屽鏋滄病鏈夎绛夊緟鑰呮秷璐圭殑宸叉彁浜?completion锛堟剰鍛崇潃
```

	bool completion_done(struct completion *done)

```
`try_wait_for_completion()` 鍜?`completion_done()` 閮藉彲浠ュ湪 IRQ 鎴栧師瀛愪笂涓嬫枃涓畨鍏ㄨ皟鐢ㄣ€?