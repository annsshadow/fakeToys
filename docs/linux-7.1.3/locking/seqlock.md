
## 椤哄簭璁℃暟鍣ㄤ笌椤哄簭閿?

## 绠€浠?

椤哄簭璁℃暟鍣ㄦ槸涓€绉嶈鍐欎竴鑷存€ф満鍒讹紝鍏锋湁鏃犻攣鐨勮鑰咃紙鍙閲嶈瘯寰幆锛夛紝涓斾笉浼氬嚭鐜?鍐欒€呴ゥ楗裤€傚畠浠敤浜庡緢灏戣鍐欏叆鐨勬暟鎹紙渚嬪绯荤粺鏃堕棿锛夛紝姝ゆ椂璇昏€呭笇鏈涜幏寰椾竴缁?涓€鑷寸殑淇℃伅锛屽苟鎰挎剰鍦ㄨ淇℃伅鍙戠敓鍙樺寲鏃堕噸璇曘€?
褰撹渚т复鐣屽尯寮€濮嬫椂鐨勯『搴忚鏁颁负鍋舵暟銆佷笖涓寸晫鍖虹粨鏉熸椂璇诲埌鐨勯『搴忚鏁板€肩浉鍚屾椂锛?鏁版嵁闆嗘槸涓€鑷寸殑銆傞泦鍚堜腑鐨勬暟鎹繀椤诲湪璇讳晶涓寸晫鍖哄唴琚鍒跺嚭鏉ャ€傚鏋滈『搴忚鏁板湪
涓寸晫鍖哄紑濮嬪拰缁撴潫涔嬮棿鍙戠敓浜嗗彉鍖栵紝璇昏€呭繀椤婚噸璇曘€?
鍐欒€呭湪鍐欎晶涓寸晫鍖虹殑寮€濮嬪拰缁撴潫澶勯€掑椤哄簭璁℃暟銆傝繘鍏ヤ复鐣屽尯鍚庨『搴忚鏁颁负濂囨暟锛?鍚戣鑰呰〃鏄庢湁鏇存柊姝ｅ湪杩涜銆傚湪鍐欎晶涓寸晫鍖虹粨鏉熸椂椤哄簭璁℃暟鍐嶆鍙樹负鍋舵暟锛屼娇璇昏€?寰椾互缁х画鎺ㄨ繘銆?
椤哄簭璁℃暟鍣ㄧ殑鍐欎晶涓寸晫鍖虹粷涓嶈兘琚渚т复鐣屽尯鎶㈠崰鎴栨墦鏂€傚惁鍒欙紝鐢变簬濂囨暟鐨勯『搴?璁℃暟鍊煎拰琚墦鏂殑鍐欒€咃紝璇昏€呬細鍦ㄦ暣涓皟搴﹁妭鎷嶅唴鑷棆銆傚鏋滆璇昏€呭睘浜庡疄鏃惰皟搴?绫伙紝瀹冨彲鑳芥案杩滆嚜鏃嬶紝鍐呮牳灏嗗彂鐢熸椿閿併€?
濡傛灉鍙椾繚鎶ょ殑鏁版嵁鍖呭惈鎸囬拡锛屽垯涓嶈兘浣跨敤姝ゆ満鍒讹紝鍥犱负鍐欒€呭彲鑳戒娇璇昏€呮鍦ㄨ窡闅忕殑
鎸囬拡澶辨晥銆?


## 椤哄簭璁℃暟鍣紙``seqcount_t``锛?

杩欐槸鍘熷鐨勮鏁版満鍒讹紝涓嶉槻姝㈠涓啓鑰呫€傚洜姝ゅ啓渚т复鐣屽尯蹇呴』鐢卞閮ㄩ攣杩涜涓茶鍖栥€?
濡傛灉鍐欎覆琛屽寲鍘熻娌℃湁闅愬紡鍦扮鐢ㄦ姠鍗狅紝鍒欏繀椤诲湪杩涘叆鍐欎晶涓寸晫鍖轰箣鍓嶆樉寮忕鐢?鎶㈠崰銆傚鏋滆渚т复鐣屽尯鍙互浠?hardirq 鎴?softirq 涓婁笅鏂囪皟鐢紝鍒欏湪杩涘叆鍐欎晶涓寸晫鍖?涔嬪墠杩樺繀椤诲垎鍒鐢ㄤ腑鏂垨搴曞崐閮ㄣ€?
濡傛灉甯屾湜鑷姩澶勭悊鍐欒€呬覆琛屽寲鍜屼笉鍙姠鍗犳€х殑椤哄簭璁℃暟鍣ㄨ姹傦紝璇锋敼鐢?seqlock_t銆?
```

	/* dynamic */
	seqcount_t foo_seqcount;
	seqcount_init(&foo_seqcount);

	/* static */
	static seqcount_t foo_seqcount = SEQCNT_ZERO(foo_seqcount);

	/* C99 struct init */
	struct {
		.seq   = SEQCNT_ZERO(foo.seq),
	} foo;

```
```

	/* Serialized context with disabled preemption */

	write_seqcount_begin(&foo_seqcount);

	/* ... [[write-side critical section]] ... */

	write_seqcount_end(&foo_seqcount);

```
```

	do {
		seq = read_seqcount_begin(&foo_seqcount);

		/* ... [[read-side critical section]] ... */

	} while (read_seqcount_retry(&foo_seqcount, seq));


```

### 甯﹀叧鑱旈攣鐨勯『搴忚鏁板櫒锛坄`seqcount_LOCKNAME_t``锛?

姝ｅ鍦?seqcount_t 涓墍杩帮紝椤哄簭璁℃暟鐨勫啓渚т复鐣屽尯蹇呴』琚覆琛屽寲涓斾笉鍙姠鍗犮€傝繖绉?椤哄簭璁℃暟鍣ㄧ殑鍙樹綋鍦ㄥ垵濮嬪寲鏃跺皢鐢ㄤ簬鍐欒€呬覆琛屽寲鐨勯攣鍏宠仈璧锋潵锛屼粠鑰屼娇 lockdep 鑳藉
楠岃瘉鍐欎晶涓寸晫鍖烘槸鍚﹁姝ｇ‘涓茶鍖栥€?
濡傛灉绂佺敤 lockdep锛屾閿佸叧鑱旀槸涓€涓┖鎿嶄綔锛屾棦娌℃湁瀛樺偍寮€閿€涔熸病鏈夎繍琛屾椂寮€閿€銆?濡傛灉鍚敤 lockdep锛岄攣鎸囬拡琚瓨鍌ㄥ湪 struct seqcount 涓紝骞跺湪鍐欎晶涓寸晫鍖哄紑濮嬫椂
娉ㄥ叆 lockdep 鐨?閿佸凡琚寔鏈?鏂█锛屼互楠岃瘉鍏跺彈鍒版纭繚鎶ゃ€?
瀵逛簬涓嶄細闅愬紡绂佺敤鎶㈠崰鐨勯攣绫诲瀷锛屽啓渚у嚱鏁颁腑浼氬己鍒跺疄鏂芥姠鍗犱繚鎶ゃ€?
瀹氫箟浜嗕互涓嬪甫鍏宠仈閿佺殑椤哄簭璁℃暟鍣細

  - `seqcount_spinlock_t`
  - `seqcount_raw_spinlock_t`
  - `seqcount_rwlock_t`
  - `seqcount_mutex_t`
  - `seqcount_ww_mutex_t`

椤哄簭璁℃暟鐨勮鍐?API 鏃㈠彲浠ユ帴鍙楁櫘閫氱殑 seqcount_t锛屼篃鍙互鎺ュ彈涓婅堪浠绘剰
seqcount_LOCKNAME_t 鍙樹綋銆?
```

	/* dynamic */
	seqcount_LOCKNAME_t foo_seqcount;
	seqcount_LOCKNAME_init(&foo_seqcount, &lock);

	/* static */
	static seqcount_LOCKNAME_t foo_seqcount =
		SEQCNT_LOCKNAME_ZERO(foo_seqcount, &lock);

	/* C99 struct init */
	struct {
		.seq   = SEQCNT_LOCKNAME_ZERO(foo.seq, &lock),
	} foo;

```
鍐欒矾寰勶細涓?seqcount_t 鐩稿悓锛屼絾杩愯鍦ㄥ凡鑾峰彇鍏宠仈鍐欎覆琛屽寲閿佺殑涓婁笅鏂囦腑銆?
璇昏矾寰勶細涓?seqcount_t 鐩稿悓銆?


### 閿佸瓨椤哄簭璁℃暟鍣紙``seqcount_latch_t``锛?

閿佸瓨椤哄簭璁℃暟鍣ㄦ槸涓€绉嶅鐗堟湰骞跺彂鎺у埗鏈哄埗锛屽叾涓祵鍏ョ殑 seqcount_t 璁℃暟鍣?鍋舵暟/濂囨暟鍊肩敤浜庡湪鍙椾繚鎶ゆ暟鎹殑涓や唤鍓湰涔嬮棿鍒囨崲銆傝繖浣垮緱椤哄簭璁℃暟鍣ㄧ殑璇昏矾寰?鑳藉瀹夊叏鍦版墦鏂叾鑷韩鐨勫啓渚т复鐣屽尯銆?
褰撳啓渚т复鐣屽尯鏃犳硶琚鑰呮墦鏂繚鎶ゆ椂浣跨敤 seqcount_latch_t銆傚綋璇讳晶鍙互浠?NMI
澶勭悊绋嬪簭璋冪敤鏃堕€氬父灏辨槸杩欑鎯呭喌銆?
璇峰弬闃?`write_seqcount_latch()` 浜嗚В鏇村淇℃伅銆?


## 椤哄簭閿侊紙``seqlock_t``锛?

杩欏寘鍚墠闈㈣璁虹殑 seqcount_t 鏈哄埗锛屽鍔犱竴涓敤浜庡啓鑰呬覆琛屽寲鍜屼笉鍙姠鍗犳€х殑
宓屽叆寮忚嚜鏃嬮攣銆?
濡傛灉璇讳晶涓寸晫鍖哄彲浠ヤ粠 hardirq 鎴?softirq 涓婁笅鏂囪皟鐢紝璇蜂娇鐢ㄥ垎鍒鐢ㄤ腑鏂垨
搴曞崐閮ㄧ殑鍐欎晶鍑芥暟鍙樹綋銆?
```

	/* dynamic */
	seqlock_t foo_seqlock;
	seqlock_init(&foo_seqlock);

	/* static */
	static DEFINE_SEQLOCK(foo_seqlock);

	/* C99 struct init */
	struct {
		.seql   = __SEQLOCK_UNLOCKED(foo.seql)
	} foo;

```
```

	write_seqlock(&foo_seqlock);

	/* ... [[write-side critical section]] ... */

	write_sequnlock(&foo_seqlock);

```
璇昏矾寰勶紝涓夌被锛?
1. 鏅€氶『搴忚鑰咃紝浠庝笉闃诲鍐欒€咃紝浣嗗鏋滄娴嬪埌椤哄簭鍙樺寲銆佹湁鍐欒€呮鍦ㄨ繘琛岋紝鍒欏繀椤?   閲嶈瘯
```

	do {
		seq = read_seqbegin(&foo_seqlock);

		/* ... [[read-side critical section]] ... */

	} while (read_seqretry(&foo_seqlock, seq));

```
2. 閿佸畾璇昏€咃紝濡傛灉鍐欒€呮垨鍙︿竴涓攣瀹氳鑰呮鍦ㄨ繘琛岋紝鍒欎細绛夊緟銆傝繘琛屼腑鐨勯攣瀹氳鑰?   涔熶細闃绘鍐欒€呰繘鍏ュ叾涓寸晫鍖恒€傝繖涓閿佷负
```

	read_seqlock_excl(&foo_seqlock);

	/* ... [[read-side critical section]] ... */

	read_sequnlock_excl(&foo_seqlock);

```
3. 鏉′欢鏃犻攣璇昏€咃紙濡?1锛夋垨閿佸畾璇昏€咃紙濡?2锛夛紝鍙栧喅浜庝紶鍏ョ殑鏍囪銆傝繖鐢ㄤ簬閬垮厤
   鏃犻攣璇昏€呭湪鍐欐椿鍔ㄦ€ュ墽椋欏崌鏃跺嚭鐜伴ゥ楗匡紙杩囧閲嶈瘯寰幆锛夈€傞鍏堝皾璇曟棤閿佽
   锛堜紶鍏ュ伓鏁版爣璁帮級銆傚鏋滆灏濊瘯澶辫触锛堥『搴忚鏁板櫒涓嶅尮閰嶏級锛屽垯灏嗘爣璁板彉涓哄鏁?   鐢ㄤ簬涓嬩竴娆¤凯浠ｏ紝鏃犻攣璇昏杞崲涓?```

	/* marker; even initialization */
	int seq = 1;
	do {
		seq++; /* 绗?1 娆?鏃犻攣璺緞涓?2锛屽惁鍒欎负濂囨暟 */
		read_seqbegin_or_lock(&foo_seqlock, &seq);

		/* ... [[read-side critical section]] ... */

	} while (need_seqretry(&foo_seqlock, seq));
	done_seqretry(&foo_seqlock, seq);


```
## API 鏂囨。

