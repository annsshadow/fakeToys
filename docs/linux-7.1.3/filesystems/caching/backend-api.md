
## 缂撳瓨鍚庣 API


FS-Cache 绯荤粺鎻愪緵浜嗕竴绉?API锛屽疄闄呯殑缂撳瓨鍙互閫氳繃瀹冩彁渚涚粰 FS-Cache锛岀敱鍚庤€呰繘鑰屾湇鍔′簬缃戠粶鏂囦欢绯荤粺
鍙婂叾浠栨劅鍏磋叮鐨勬柟

```
	#include <linux/fscache-cache.h>.
```

## 姒傝堪


涓?API 鐨勪氦浜掑湪涓変釜灞傜骇涓婅繘琛岋細缂撳瓨锛坈ache锛夈€佸嵎锛坴olume锛変笌鏁版嵁瀛樺偍锛坉ata storage锛夛紝姣忎釜灞傜骇
閮芥湁鑷繁鐨?cookie 瀵硅薄绫诲瀷锛?
	=======================	=======================
	COOKIE			C TYPE
	=======================	=======================
	Cache cookie		struct fscache_cache
	Volume cookie		struct fscache_volume
	Data storage cookie	struct fscache_cookie
	=======================	=======================

Cookie 鐢ㄤ簬鍚戠紦瀛樻彁渚涗竴浜涙枃浠剁郴缁熸暟鎹€佺鐞嗙姸鎬佸苟鍦ㄨ闂湡闂村浐瀹氱紦瀛橈紝姝ゅ杩樺厖褰?API 鍑芥暟鐨?寮曠敤鐐广€傛瘡涓?cookie 閮芥湁涓€涓皟璇?ID锛屽畠琚寘鍚湪 tracepoint 涓紝浠ヤ究鏇村鏄撳湴鍏宠仈璺熻釜璁板綍銆備笉杩?璇锋敞鎰忥紝璋冭瘯 ID 鍙槸浠庨€掑璁℃暟鍣ㄤ腑鍒嗛厤鍑烘潵鐨勶紝鏈€缁堜細鍥炵粫銆?
缂撳瓨鍚庣涓庣綉缁滄枃浠剁郴缁熼兘鍙互璇锋眰缂撳瓨 cookie鈥斺€斿鏋滃畠浠姹備簡鍚屼竴涓悕绉帮紝灏变細寰楀埌鍚屼竴涓?cookie銆?鑰屽嵎涓庢暟鎹?cookie 鍒欎粎鐢辨枃浠剁郴缁熸寜闇€瑕佸垱寤恒€?

## 缂撳瓨 Cookie


缂撳瓨鍦?API 涓敱缂撳瓨 cookie 琛ㄧず銆傚畠浠槸濡備笅瀵硅薄

```
	struct fscache_cache {
		void		*cache_priv;
		unsigned int	debug_id;
		char		*name;
		...
	};
```

缂撳瓨鍚庣鍙兘浼氭劅鍏磋叮鐨勫瓧娈垫湁鍑犱釜銆俙debug_id` 鍙敤浜庤窡韪腑浠ュ尮閰嶅紩鐢ㄥ悓涓€缂撳瓨鐨勮锛宍name` 鏄
缂撳瓨娉ㄥ唽鏃朵娇鐢ㄧ殑鍚嶇О銆俙cache_priv` 鎴愬憳鏄紦瀛樹笂绾挎椂鐢辩紦瀛樻彁渚涚殑绉佹湁鏁版嵁銆傚叾浣欏瓧娈典緵鍐呴儴浣跨敤銆?

## 娉ㄥ唽涓€涓紦瀛?

褰撶紦瀛樺悗绔兂瑕佽涓€涓紦瀛樹笂绾挎椂锛屽畠搴斿綋鍏堟敞鍐?
```
	struct fscache_cache *fscache_acquire_cache(const char *name);
```

杩欎細鏌ユ壘骞跺彲鑳藉垱寤轰竴涓紦瀛?cookie銆傝缂撳瓨 cookie 鍙兘宸茬粡琚煇涓鍦ㄥ鎵惧畠鐨勭綉缁滄枃浠剁郴缁熷垱寤猴紝
鍦ㄨ繖绉嶆儏褰笅灏变細浣跨敤閭ｄ釜缂撳瓨 cookie銆傚鏋滆缂撳瓨 cookie 娌℃湁琚彟涓€涓紦瀛樹娇鐢紝瀹冨皢琚Щ鍏?preparing锛堝噯澶囦腑锛夌姸鎬侊紝鍚﹀垯浼氳繑鍥?busy锛堝繖锛夈€?
濡傛灉鎴愬姛锛岀紦瀛樺悗绔殢鍚庡氨鍙互寮€濮嬫惌寤虹紦瀛樸€傚湪

```
	void fscache_relinquish_cache(struct fscache_cache *cache);
```

涓彲浠ラ噸缃苟涓㈠純璇?cookie銆?

## 浣跨紦瀛樹笂绾?

```
	int fscache_add_cache(struct fscache_cache *cache,
			      const struct fscache_cache_ops *ops,
			      void *cache_priv);
```

杩欏皢鎶婄紦瀛樻搷浣滆〃鎸囬拡涓庣紦瀛樼鏈夋暟鎹瓨鍌ㄨ繘缂撳瓨 cookie锛屽苟灏嗙紦瀛樼Щ鍏?active锛堟椿鍔級鐘舵€侊紝浠庤€?鍏佽璁块棶鍙戠敓銆?

## 灏嗙紦瀛樻挙鍑烘湇鍔?

```
	void fscache_withdraw_cache(struct fscache_cache *cache);
```

杩欏皢鎶婄紦瀛樼Щ鍏?withdrawn锛堝凡鎾ゅ嚭锛夌姸鎬侊紝浠ラ樆姝㈡柊鐨勭紦瀛樼骇涓庡嵎绾ц闂惎鍔紝鐒跺悗绛夊緟鏈畬鎴愮殑缂撳瓨绾?璁块棶瀹屾垚銆?
闅忓悗缂撳瓨蹇呴』閬嶅巻瀹冩墍鎷ユ湁鐨勬暟鎹瓨鍌ㄥ璞★紝骞跺姣忎釜瀵硅薄鎵€灞炵殑 cookie 璋冪敤

```
	void fscache_withdraw_cookie(struct fscache_cookie *cookie);
```

杩欎細灏嗚鎸囧畾 cookie 瀹夋帓鎾ゅ嚭銆傚畠琚嵏杞藉埌涓€涓伐浣滈槦鍒椾笂銆傚湪

```
	void fscache_wait_for_objects(struct fscache_cache *cache);
```

涔嬪悗锛岀紦瀛樺悗绔彲浠ユ挙鍑烘墍鏈夌殑

```
	void fscache_withdraw_volume(struct fscache_volume *volume);
```

浠ュ憡鐭?fscache 鏌愪釜鍗峰凡琚挙鍑恒€傚畠浼氬湪杩斿洖涔嬪墠绛夊緟璇ュ嵎涓婃墍鏈夋湭瀹屾垚鐨勮闂畬鎴愩€?
褰撶紦瀛樿瀹屽叏鎾ゅ嚭鏃讹紝搴斿綋閫氳繃

```
	void fscache_relinquish_cache(struct fscache_cache *cache);
```

閫氱煡 fscache锛屼互娓呴櫎 cookie 涓殑瀛楁骞朵涪寮冭皟鐢ㄦ柟瀵瑰叾鐨勫紩鐢ㄣ€?

## 鍗?Cookie


鍦ㄤ竴涓紦瀛樺唴閮紝鏁版嵁瀛樺偍瀵硅薄琚粍缁囨垚閫昏緫鍗枫€?
```
	struct fscache_volume {
		struct fscache_cache		*cache;
		void				*cache_priv;
		unsigned int			debug_id;
		char				*key;
		unsigned int			key_hash;
		...
		u8				coherency_len;
		u8				coherency[];
	};
```

杩欓噷鏈変竴浜涘缂撳瓨鍚庣鑰岃█鎰熷叴瓒ｇ殑瀛楁锛?
   - `cache` - 鐖剁紦瀛?cookie銆?
   - `cache_priv` - 缂撳瓨鐢ㄦ潵瀛樻斁绉佹湁鏁版嵁鐨勫湴鏂广€?
   - `debug_id` - 鐢ㄤ簬 tracepoint 鏃ュ織璁板綍鐨勮皟璇?ID銆?
   - `key` - 涓€涓彲鎵撳嵃瀛楃涓诧紝鍏朵腑涓嶅寘鍚换浣?'/' 瀛楃锛岃〃绀哄嵎鐨勭储寮曢敭銆傝閿互 NUL 缁撳熬锛屽苟
     琚～鍏呭埌 4 瀛楄妭鐨勫€嶆暟銆?
   - `key_hash` - 绱㈠紩閿殑鍝堝笇銆傛棤璁?CPU 鏋舵瀯涓庡瓧鑺傚簭濡備綍锛屽畠閮藉簲褰撴槸涓€鏍风殑銆?
   - `coherency` - 涓€娈典竴鑷存€ф暟鎹紝鍦ㄥ嵎琚粦瀹氬埌缂撳瓨涓椂搴斿綋琚鏌ャ€?
   - `coherency_len` - 涓€鑷存€ф暟鎹紦鍐插尯涓殑鏁版嵁閲忋€?

## 鏁版嵁瀛樺偍 Cookie


涓€涓嵎鏄暟鎹瓨鍌ㄥ璞＄殑閫昏緫鍒嗙粍锛屽叾涓瘡涓璞￠兘鐢变竴涓?cookie 鍚戠綉缁滄枃浠剁郴缁熻〃绀恒€侰ookie 鍦?
```
	struct fscache_cookie {
		struct fscache_volume		*volume;
		void				*cache_priv;
		unsigned long			flags;
		unsigned int			debug_id;
		unsigned int			inval_counter;
		loff_t				object_size;
		u8				advice;
		u32				key_hash;
		u8				key_len;
		u8				aux_len;
		...
	};
```

涓〃绀恒€?
cookie 涓缂撳瓨鍚庣鑰岃█鎰熷叴瓒ｇ殑瀛楁鏈夛細

   - `volume` - 鐖跺嵎 cookie銆?
   - `cache_priv` - 缂撳瓨鐢ㄦ潵瀛樻斁绉佹湁鏁版嵁鐨勫湴鏂广€?
   - `flags` - 涓€缁勪綅鏍囧織锛屽寘鎷細

      - FSCACHE_COOKIE_NO_DATA_TO_READ - 缂撳瓨涓病鏈夊彲渚涜鍙栫殑鏁版嵁锛屽洜涓鸿 cookie 宸茶鍒涘缓鎴?	澶辨晥銆?
      - FSCACHE_COOKIE_NEEDS_UPDATE - 涓€鑷存€ф暟鎹拰/鎴栧璞″ぇ灏忓凡琚洿鏀癸紝闇€瑕佹彁浜ゃ€?
      - FSCACHE_COOKIE_LOCAL_WRITE - netfs 鐨勬暟鎹凡琚湰鍦颁慨鏀癸紝鍥犳缂撳瓨瀵硅薄鐩稿浜庢湇鍔″櫒鍙兘澶勪簬
	涓嶄竴鑷寸姸鎬併€?
      - FSCACHE_COOKIE_HAVE_DATA - 濡傛灉鍚庣鎴愬姛灏嗘暟鎹瓨鍏ョ紦瀛橈紝鍒欏簲褰撹缃鏍囧織銆?
      - FSCACHE_COOKIE_RETIRED - 璇?cookie 鍦ㄨ鏀惧純鏃跺凡琚け鏁堬紝缂撳瓨鏁版嵁搴斿綋琚涪寮冦€?
   - `debug_id` - 鐢ㄤ簬 tracepoint 鏃ュ織璁板綍鐨勮皟璇?ID銆?
   - `inval_counter` - 瀵硅 cookie 鎵ц鐨勫け鏁堟鏁般€?
   - `advice` - 鍏充簬璇?cookie 灏嗗浣曡浣跨敤鐨勪俊鎭€?
   - `key_hash` - 绱㈠紩閿殑鍝堝笇銆傛棤璁?CPU 鏋舵瀯涓庡瓧鑺傚簭濡備綍锛屽畠閮藉簲褰撴槸涓€鏍风殑銆?
   - `key_len` - 绱㈠紩閿殑闀垮害銆?
   - `aux_len` - 涓€鑷存€ф暟鎹紦鍐插尯鐨勯暱搴︺€?
姣忎釜 cookie 閮芥湁涓€涓储寮曢敭锛屽畠鍙互鍐呰仈瀛樺偍鍦?cookie 涓紝涔熷彲浠ラ€氳繃

```
	void *fscache_get_key(struct fscache_cookie *cookie);
```

鑾峰彇銆傜储寮曢敭鏄竴娈典簩杩涘埗鏁版嵁鍧楋紝鍏跺瓨鍌ㄤ細琚～鍏呭埌 4 瀛楄妭鐨勫€嶆暟銆?
姣忎釜 cookie 杩樻湁涓€涓敤浜庝竴鑷存€ф暟鎹殑缂撳啿鍖恒€傚畠涔熷彲浠ユ槸鍐呰仈鐨勶紝鎴栭€氳繃

```
	void *fscache_get_aux(struct fscache_cookie *cookie);
```

鑾峰彇銆?

## Cookie 璁拌处


鏁版嵁瀛樺偍 cookie 浼氳璁℃暟锛岃繖鐢ㄤ簬鍦ㄦ墍鏈夊璞￠兘琚攢姣佷箣鍓嶉樆濉炵紦瀛樻挙鍑哄畬鎴愩€備互涓嬪嚱鏁?
```
	void fscache_count_object(struct fscache_cache *cache);
	void fscache_uncount_object(struct fscache_cache *cache);
	void fscache_wait_for_objects(struct fscache_cache *cache);
```

count 鍑芥暟璁板綍缂撳瓨涓竴涓璞＄殑鍒嗛厤锛寀ncount 鍑芥暟璁板綍鍏堕攢姣併€傝鍛婏細鍦?uncount 鍑芥暟杩斿洖鏃讹紝缂撳瓨
鍙兘宸茬粡琚攢姣併€?
wait 鍑芥暟鍙湪鎾ゅ嚭杩囩▼涓娇鐢紝浠ョ瓑寰?fscache 瀹屾垚鎾ゅ嚭缂撳瓨涓殑鎵€鏈夊璞°€傚綋瀹冮€氳繃鏃讹紝灏嗕笉鍐嶆湁寮曠敤
璇ョ紦瀛樺璞℃垨浠讳綍鍗峰璞＄殑鍓╀綑瀵硅薄銆?

## 缂撳瓨绠＄悊 API


缂撳瓨鍚庣閫氳繃鎻愪緵涓€涓搷浣滆〃鏉ュ疄鐜扮紦瀛樼鐞?API锛宖scache 鍙互鍒╃敤杩欎簺鎿嶄綔鏉ョ鐞嗙紦瀛樼殑鍚勪釜鏂归潰銆?璇ヨ〃鐢?`struct fscache_cache_ops` 琛ㄧず锛?
```
	struct fscache_cache_ops {
		const char *name;
		...
	};
```

瀹冨寘鍚竴涓緵缂撳瓨鍚庣椹卞姩鎵撳嵃鐨勫悕绉帮紝浠ュ強鑻ュ共鎸囧悜鏂规硶鐨勬寚閽堬紝浣?fscache 鑳藉璇锋眰瀵圭紦瀛樼殑绠＄悊锛?
```
	void (*acquire_volume)(struct fscache_volume *volume);
```

     璇ユ柟娉曞湪涓€涓嵎 cookie 姝ｅ湪鍒涘缓鏃惰璋冪敤銆傝皟鐢ㄦ柟鎸佹湁涓€涓紦瀛樼骇鍒殑璁块棶寮曡剼锛坅ccess pin锛夛紝
     浠ラ槻姝㈢紦瀛樺湪姝ゆ湡闂磋閿€姣併€傝鏂规硶搴斿綋寤虹珛璁块棶缂撳瓨涓煇涓嵎鎵€闇€鐨勮祫婧愶紝骞朵笖鍦ㄥ畬鎴愪箣鍓嶄笉搴旇繑鍥炪€?
     濡傛灉鎴愬姛锛屽畠鍙互灏?``cache_priv`` 璁句负瀹冭嚜宸辩殑鏁版嵁銆?
   * 娓呯悊鍗?cookie [鍙€塢锛?
```
	void (*free_volume)(struct fscache_volume *volume);
```

     褰撴煇涓嵎 cookie 琚噴鏀炬椂锛屽鏋?``cache_priv`` 宸茶璁剧疆锛屽垯浼氳皟鐢ㄦ鏂规硶銆?
   * 鍦ㄧ紦瀛樹腑鏌ユ壘涓€涓?cookie [寮哄埗]锛?
```
	bool (*lookup_cookie)(struct fscache_cookie *cookie);
```

     璋冪敤姝ゆ柟娉曚互鏌ユ壘/鍒涘缓璁块棶鏌愪釜 cookie 鐨勬暟鎹瓨鍌ㄦ墍闇€鐨勮祫婧愩€傚畠浠庝竴涓伐浣滅嚎绋嬩腑璋冪敤锛屽苟甯︽湁
     缂撳瓨涓殑涓€涓嵎绾ц闂紩鑴氾紝浠ラ槻姝㈣鍗疯鎾ゅ嚭銆?
     鎴愬姛鏃跺簲褰撹繑鍥?true锛屽惁鍒欒繑鍥?false銆傚鏋滆繑鍥?false锛屽垯浼氳皟鐢?withdraw_cookie 鎿嶄綔锛堣涓嬫枃锛夈€?
     濡傛灉鏌ユ壘澶辫触锛屼絾璇ュ璞′粛鍙鍒涘缓锛堜緥濡傛鍓嶅皻鏈缂撳瓨锛夛紝鍒欏彲浠ヨ皟鐢?
```
	void fscache_cookie_lookup_negative(
			struct fscache_cookie *cookie);
```

     浠ヨ缃戠粶鏂囦欢绯荤粺缁х画杩愯锛屽苟鍦ㄧ紦瀛樺悗绔潃鎵嬪垱寤虹浉鍏宠祫婧愮殑鍚屾椂寮€濮嬩笅杞藉唴瀹广€?
     濡傛灉鎴愬姛锛屽彲浠ヨ缃?``cookie->cache_priv``銆?
   * 鍦ㄦ病鏈夋寔鏈変换浣?cookie 璁块棶璁℃暟鍦版儏鍐典笅鎾ゅ嚭涓€涓璞?[寮哄埗]锛?
```
	void (*withdraw_cookie)(struct fscache_cookie *cookie);
```

     璋冪敤姝ゆ柟娉曚互灏嗕竴涓?cookie 鎾ゅ嚭鏈嶅姟銆傚綋璇?cookie 琚?netfs 鏀惧純銆佽缂撳瓨鍚庣鎾ゅ嚭鎴栧墧闄わ紝鎴栬
     fscache 鍦ㄩ潪浣跨敤涓€娈垫椂闂村悗鍏抽棴鏃讹紝閮戒細璋冪敤瀹冦€?
     璋冪敤鏂逛笉鎸佹湁浠讳綍璁块棶寮曡剼锛屼絾瀹冧粠涓€涓笉鍙噸鍏ョ殑宸ヤ綔椤逛腑琚皟鐢紝浠ョ鐞嗘挙鍑哄彲鑳藉彂鐢熺殑鍚勭鏂瑰紡
     涔嬮棿鐨勭珵浜夈€?
     濡傛灉鐩稿叧鑱旂殑鏁版嵁瑕佷粠缂撳瓨涓Щ闄わ紝璇?cookie 涓婁細璁剧疆 ``FSCACHE_COOKIE_RETIRED`` 鏍囧織銆?
   * 鏀瑰彉涓€涓暟鎹瓨鍌ㄥ璞＄殑澶у皬 [寮哄埗]锛?
```
	void (*resize_cookie)(struct netfs_cache_resources *cres,
			      loff_t new_size);
```

     璋冪敤姝ゆ柟娉曚互鍛婄煡缂撳瓨鍚庣锛岀敱浜庢湰鍦版埅鏂紝netfs 鏂囦欢鐨勫ぇ灏忓彂鐢熶簡鍙樺寲銆傜紦瀛樺悗绔簲褰撳湪杩斿洖涔嬪墠
     瀹屾垚瀹冮渶瑕佸仛鐨勬墍鏈夋敼鍔紝鍥犱负杩欏彂鐢熷湪 netfs inode 浜掓枼閿佷箣涓嬨€?
     璋冪敤鏂规寔鏈変竴涓?cookie 绾у埆鐨勮闂紩鑴氾紝浠ラ槻姝笌鎾ゅ嚭鍙戠敓绔炰簤锛屽苟涓?netfs 蹇呴』宸插皢璇?cookie
     鏍囪涓轰娇鐢ㄤ腑锛屼互闃叉鍨冨溇鍥炴敹鎴栧墧闄ょЩ闄や换浣曡祫婧愩€?
   * 浣夸竴涓暟鎹瓨鍌ㄥ璞″け鏁?[寮哄埗]锛?
```
	bool (*invalidate_cookie)(struct fscache_cookie *cookie);
```

     褰撶綉缁滄枃浠剁郴缁熸娴嬪埌绗笁鏂逛慨鏀癸紝鎴栬繘琛屼簡涓€娆℃湰鍦?O_DIRECT 鍐欏叆鏃讹紝浼氳皟鐢ㄦ鏂规硶銆傚畠璇锋眰缂撳瓨
     鍚庣涓㈠純璇ュ璞″湪缂撳瓨涓殑鎵€鏈夋暟鎹苟閲嶆柊寮€濮嬨€傛垚鍔熸椂搴斿綋杩斿洖 true锛屽惁鍒欒繑鍥?false銆?
     鍦ㄨ繘鍏ユ椂锛屾柊鐨?I/O 鎿嶄綔浼氳闃诲銆備竴鏃︾紦瀛樺浜庡彲浠ュ啀娆℃帴鍙?I/O 鐨勭姸鎬侊紝鍚庣搴斿綋閫氳繃璋冪敤

```
	void fscache_resume_after_invalidation(struct fscache_cookie *cookie);
```

     鏉ラ噴鏀捐闃诲銆?
     濡傛灉璇ユ柟娉曡繑鍥?false锛屽垯浼氶拡瀵规 cookie 鎾ゅ嚭缂撳瓨銆?
   * 鍑嗗瀵圭紦瀛樿繘琛屾湰鍦颁慨鏀?[寮哄埗]锛?
```
	void (*prepare_to_write)(struct fscache_cookie *cookie);
```

     褰撶綉缁滄枃浠剁郴缁熷彂鐜板畠灏嗛渶瑕佸洜鏈湴鍐欏叆鎴栨埅鏂€屼慨鏀圭紦瀛樼殑鍐呭鏃讹紝浼氳皟鐢ㄦ鏂规硶銆傝繖缁欑紦瀛樹竴涓?     鏈轰細锛岃涓嬫煇涓紦瀛樺璞＄浉瀵逛簬鏈嶅姟鍣ㄥ彲鑳藉浜庝笉涓€鑷寸姸鎬侊紝骞跺彲鑳介渶瑕佸湪绋嶅悗鍐欏洖銆傚鏋滄湭鑳芥纭?     鎻愪氦锛岃繖涔熷彲鑳藉鑷寸紦瀛樻暟鎹湪鍚庣画閲嶆柊缁戝畾鏃惰涓㈠純銆?
   * 涓?netfs 搴撳紑濮嬩竴涓搷浣?[寮哄埗]锛?
```
	bool (*begin_operation)(struct netfs_cache_resources *cres,
				enum fscache_want_state want_state);
```

     褰撴鍦ㄥ缓绔嬩竴涓?I/O 鎿嶄綔锛堣銆佸啓鎴栬皟鏁村ぇ灏忥級鏃讹紝浼氳皟鐢ㄦ鏂规硶銆傝皟鐢ㄦ柟鎸佹湁璇?cookie 涓婄殑涓€涓?     璁块棶寮曡剼锛屽苟涓斿繀椤诲凡灏嗚 cookie 鏍囪涓轰娇鐢ㄤ腑銆?
     濡傛灉鍙互锛屽悗绔簲褰撴妸闇€瑕佷繚鐣欑殑浠讳綍璧勬簮闄勫姞鍒?netfs_cache_resources 瀵硅薄涓婏紝骞惰繑鍥?true銆?
     濡傛灉瀹冩棤娉曞畬鎴愯缃紝鍒欏簲褰撹繑鍥?false銆?
     want_state 鍙傛暟鎸囩ず璋冪敤鏂归渶瑕佺紦瀛樺璞″浜庝粈涔堢姸鎬侊紝浠ュ強瀹冩兂鍦ㄨ鎿嶄綔鏈熼棿鍋氫粈涔堬細

	* ``FSCACHE_WANT_PARAMS`` - 璋冪敤鏂瑰彧鏄兂璁块棶缂撳瓨瀵硅薄鐨勫弬鏁帮紱瀹冭繕涓嶉渶瑕佽繘琛屾暟鎹?I/O銆?
	* ``FSCACHE_WANT_READ`` - 璋冪敤鏂规兂瑕佽鍙栨暟鎹€?
	* ``FSCACHE_WANT_WRITE`` - 璋冪敤鏂规兂瑕佸啓鍏ユ垨璋冩暣缂撳瓨瀵硅薄鐨勫ぇ灏忋€?
     娉ㄦ剰锛屽鏋?cookie 浠嶅湪鍒涘缓涓紝鍏?``cache_priv`` 涓婃湭蹇呭凡缁忛檮鍔犱簡浠讳綍鍐呭銆?

## 鏁版嵁 I/O API


缂撳瓨鍚庣閫氳繃 netfs 搴撶殑 ``struct netfs_cache_ops`` 鎻愪緵鏁版嵁 I/O API锛岃缁撴瀯鐢变笂杩?`begin_operation` 鏂规硶闄勫姞鍒颁竴涓?`struct netfs_cache_resources` 涓娿€?
鐩稿叧璇存槑璇峰弬闃?Documentation/filesystems/netfs_library.rst銆?

## 鏉傞」鍑芥暟


FS-Cache 鎻愪緵浜嗕竴浜涚紦瀛樺悗绔彲浠ヤ娇鐢ㄧ殑瀹炵敤鍑芥暟锛?
```
	void fscache_io_error(struct fscache_cache *cache);
```

     杩欏憡鐭?FS-Cache 缂撳瓨涓彂鐢熶簡涓€涓?I/O 閿欒銆傝繖浼氶樆姝㈠湪璇ョ紦瀛樹笂鍚姩浠讳綍鏂扮殑 I/O銆?
     杩欏苟涓嶄細瀹為檯鎾ゅ嚭缂撳瓨銆傞偅蹇呴』鍗曠嫭杩涜銆?
   * 璁板綍鍥犲け璐ヨ€屽仠姝㈠湪鏌愪釜 cookie 涓婄殑缂撳瓨锛?
```
	void fscache_caching_failed(struct fscache_cookie *cookie);
```

     杩欒褰曞湪鏌愪釜 cookie 涓婅繘琛岀殑缂撳瓨浠ユ煇绉嶆柟寮忓け璐ヤ簡锛屼緥濡傚悗澶囧瓨鍌ㄥ垱寤哄け璐ユ垨澶辨晥澶辫触锛屽苟涓斿湪
     缂撳瓨琚噸缃箣鍓嶄笉搴斿湪鍏朵笂杩涜杩涗竴姝ョ殑 I/O 鎿嶄綔銆?
   * 缁熻 I/O 璇锋眰锛?
```
	void fscache_count_read(void);
	void fscache_count_write(void);
```

     杩欎簺璁板綍瀵圭紦瀛樼殑璇讳笌鍐欍€傝繖浜涙暟瀛楁樉绀哄湪 /proc/fs/fscache/stats 涓€?
   * 缁熻绌洪棿涓嶈冻閿欒锛?
```
	void fscache_count_no_write_space(void);
	void fscache_count_no_create_space(void);
```

     杩欎簺璁板綍缂撳瓨涓殑 ENOSPC 閿欒锛屽垎涓烘暟鎹啓鍏ュけ璐ヤ笌鏂囦欢绯荤粺瀵硅薄鍒涘缓澶辫触锛堜緥濡?mkdir锛夈€?
   * 缁熻琚墧闄ょ殑瀵硅薄锛?
```
	void fscache_count_culled(void);
```

     杩欒褰曚竴涓璞¤鍓旈櫎銆?
   * 浠庝竴缁勭紦瀛樿祫婧愪腑鍙栧嚭 cookie锛?
```
	struct fscache_cookie *fscache_cres_cookie(struct netfs_cache_resources *cres)
```

     浠庣紦瀛樿祫婧愪腑鎷夊嚭涓€涓寚鍚?cookie 鐨勬寚閽堛€傚鏋滄病鏈夎缃?cookie锛屽垯鍙兘杩斿洖涓€涓?NULL cookie銆?

## API 鍑芥暟鍙傝€?