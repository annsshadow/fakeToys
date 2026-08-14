
## 缃戠粶鏂囦欢绯荤粺缂撳瓨 API

fscache 鎻愪緵涓€涓?API锛岀綉缁滄枃浠剁郴缁熷彲鍊熸鍒╃敤鏈湴缂撳瓨璁炬柦銆傝 API 鍥寸粫鑻ュ共鍘熷垯缁勭粐锛?
 (1) 涓€涓紦瀛樹粠閫昏緫涓婅缁勭粐涓鸿嫢骞插嵎锛坴olume锛変互鍙婅繖浜涘嵎鍐呯殑鏁版嵁瀛樺偍瀵硅薄銆?
 (2) 鍗峰拰鏁版嵁瀛樺偍瀵硅薄鐢卞悇绉嶇被鍨嬬殑 cookie 琛ㄧず銆?
 (3) cookie 鎷ユ湁灏嗗叾涓庡悓绫诲尯鍒嗗紑鏉ョ殑閿紙key锛夈€?
 (4) cookie 鎷ユ湁涓€鑷存€ф暟鎹紝浣跨紦瀛樿兘澶熷垽鏂紦瀛樼殑鏁版嵁鏄惁浠嶇劧鏈夋晥銆?
 (5) 鍦ㄥ彲鑳界殑鎯呭喌涓嬶紝I/O 鏄紓姝ヨ繘琛岀殑銆?
```

	#include <linux/fscache.h>.

```

	 (1) 姒傝堪
	 (2) 鍗锋敞鍐?	 (3) 鏁版嵁鏂囦欢娉ㄥ唽
	 (4) 澹版槑涓€涓?cookie 鍦ㄤ娇鐢ㄤ腑
	 (5) 璋冩暣鏁版嵁鏂囦欢澶у皬锛堟埅鏂級
	 (6) 鏁版嵁 I/O API
	 (7) 鏁版嵁鏂囦欢涓€鑷存€?	 (8) 鏁版嵁鏂囦欢澶辨晥
	 (9) 鍥炲啓璧勬簮绠＄悊
	(10) 鏈湴淇敼鐨勭紦瀛?	(11) 椤甸噴鏀句笌澶辨晥


## 姒傝堪

浠庣綉缁滄枃浠剁郴缁熺殑瑙掑害鐪嬶紝fscache 鐨勫眰绾у垎涓轰袱涓骇鍒粍缁囥€備笂绾т唬琛ㄢ€滃嵎鈥濓紝涓嬬骇浠ｈ〃鈥滄暟鎹瓨鍌ㄥ璞♀€濄€?瀹冧滑鐢变袱绫?cookie 琛ㄧず锛屼互涓嬪垎鍒О涓衡€滃嵎 cookie鈥濆拰鈥渃ookie鈥濄€?
缃戠粶鏂囦欢绯荤粺浣跨敤鍗烽敭锛坴olume key锛変负鏌愪釜鍗疯幏鍙栦竴涓嵎 cookie锛屽嵎閿唬琛ㄥ畾涔夎鍗风殑鍏ㄩ儴淇℃伅
锛堜緥濡?cell 鍚嶇О鎴栨湇鍔″櫒鍦板潃銆佸嵎 ID 鎴栧叡浜悕锛夈€傚畠蹇呴』琚覆鏌撲负涓€涓彲鐢ㄤ綔鐩綍鍚嶇殑鍙墦鍗板瓧绗︿覆
锛堝嵆涓嶅惈 '/' 瀛楃锛屼笖涓嶅簲浠?'.' 寮€澶达級銆傛渶澶у悕绉伴暱搴︽瘮鏂囦欢鍚嶅垎閲忕殑鏈€澶уぇ灏忓皬涓€锛堜负缂撳瓨鍚庣鐣欏嚭涓€涓?瀛楃渚涘叾鑷敤锛夈€?
涓€涓枃浠剁郴缁熼€氬父浼氫负姣忎釜瓒呯骇鍧楁嫢鏈変竴涓嵎 cookie銆?
闅忓悗锛屾枃浠剁郴缁熶娇鐢ㄨ鍗峰唴鐨勫璞￠敭锛坥bject key锛変负璇ュ嵎涓殑姣忎釜鏂囦欢鑾峰彇涓€涓?cookie銆傚璞￠敭鏄簩杩涘埗
blob锛屽彧闇€瑕佸湪鍏剁埗鍗峰唴鍞竴銆傜紦瀛樺悗绔礋璐ｅ皢浜岃繘鍒?blob 娓叉煋涓哄畠鍙敤鐨勫舰寮忥紝骞跺彲鑳介噰鐢ㄥ搱甯岃〃銆佹爲鎴?浠讳綍鍏跺畠缁撴瀯鏉ユ彁鍗囧叾鏌ユ壘瀵硅薄鐨勮兘鍔涖€傝繖瀵圭綉缁滄枃浠剁郴缁熸槸閫忔槑鐨勩€?
涓€涓枃浠剁郴缁熼€氬父浼氫负姣忎釜 inode 鎷ユ湁涓€涓?cookie锛屽苟鍦?iget 涓幏鍙栧畠锛屽湪椹遍€愯 cookie 鏃?relinquish銆?
涓€鏃︽嫢鏈?cookie锛屾枃浠剁郴缁熼渶瑕佸皢 cookie 鏍囪涓哄湪浣跨敤涓€傝繖浼氬鑷?fscache 鍦ㄥ悗鍙版淳缂撳瓨鍚庣鍘绘煡鎵?鍒涘缓
璇?cookie 鐨勮祫婧愶紝妫€鏌ュ叾涓€鑷存€э紝骞跺湪蹇呰鏃跺皢璇ュ璞℃爣璁颁负澶勪簬淇敼涓€?
鏂囦欢绯荤粺閫氬父浼氬湪鍏舵枃浠舵墦寮€渚嬬▼涓€滀娇鐢ㄢ€濊 cookie锛屽苟鍦ㄦ枃浠堕噴鏀炬椂鍙栨秷浣跨敤锛屽苟涓斿畠闇€瑕佸湪瀵?cookie
杩涜鏈湴鎴柇鐨勮皟鐢ㄥ墠鍚庝娇鐢ㄨ cookie銆傚畠**杩?*闇€瑕佸湪 pagecache 鍙樿剰鏃朵娇鐢ㄨ cookie锛屽苟鍦ㄥ洖鍐欏畬鎴愭椂
鍙栨秷浣跨敤銆傝繖鏈変簺妫樻墜锛屼絾鎴戜滑涓烘鍋氫簡鐩稿簲瀹夋帓銆?
鍦ㄥ cookie 鎵ц璇汇€佸啓鎴栬皟鏁村ぇ灏忔搷浣滄椂锛屾枃浠剁郴缁熷繀椤婚鍏堝紑濮嬩竴涓搷浣溿€傝繖浼氬皢璧勬簮澶嶅埗鍒颁竴涓寔鏈夌粨鏋?锛坔olding struct锛変腑锛屽苟瀵圭紦瀛樺姞棰濆鐨?pin锛屼互闃绘缂撳瓨鎾ら攢鎷嗘瘉姝ｅ湪浣跨敤鐨勭粨鏋勩€傞殢鍚庡彲浠ュ彂璧峰疄闄呮搷浣滐紝
骞跺湪瀹屾垚鏃舵娴嬪埌鍐茬獊鐨勫け鏁堛€?
鏂囦欢绯荤粺搴斿綋浣跨敤 netfslib 鏉ヨ闂紦瀛橈紝浣嗚繖骞堕潪鐪熸寮哄埗锛屽畠涔熷彲浠ョ洿鎺ヤ娇鐢?fscache I/O API銆?

## 鍗锋敞鍐?
缃戠粶鏂囦欢绯荤粺鐨勭涓€姝ユ槸涓哄嵎鑾峰彇涓€涓嵎 cookie锛?
```

	struct fscache_volume *
	fscache_acquire_volume(const char *volume_key,
			       const char *cache_name,
			       const void *coherency_data,
			       size_t coherency_len);

```
姝ゅ嚱鏁板垱寤轰竴涓互鎸囧畾鍗烽敭浣滀负鍚嶇О鐨勫嵎 cookie锛屽苟璁板綍涓€鑷存€ф暟鎹€?
鍗烽敭蹇呴』鏄彲鎵撳嵃瀛楃涓诧紝涓斿叾涓笉鍚?'/' 瀛楃銆傚畠搴斾互鏂囦欢绯荤粺鐨勫悕绉板紑澶达紝涓旈暱搴︿笉瓒呰繃 254 涓瓧绗︺€?瀹冨簲褰撳敮涓€鍦颁唬琛ㄨ鍗凤紝骞跺皢涓庣紦瀛樹腑瀛樺偍鐨勫唴瀹硅繘琛屽尮閰嶃€?
璋冪敤鑰呰繕鍙互鎸囧畾瑕佷娇鐢ㄧ殑缂撳瓨鐨勫悕绉般€傚鏋滄寚瀹氫簡锛宖scache 灏嗘煡鎵炬垨鍒涘缓涓€涓叿鏈夎鍚嶇О鐨勭紦瀛?cookie锛?骞跺湪璇ュ悕绉扮殑缂撳瓨涓婄嚎鏃朵娇鐢ㄥ畠銆傚鏋滄湭鎸囧畾缂撳瓨鍚嶇О锛屽畠灏嗕娇鐢ㄦ墜杈圭涓€涓紦瀛橈紝骞跺皢鍚嶇О璁句负璇ョ紦瀛樸€?
鎸囧畾鐨勪竴鑷存€ф暟鎹瓨鍌ㄥ湪 cookie 涓紝骞跺皢涓庣鐩樹笂瀛樺偍鐨勪竴鑷存€ф暟鎹尮閰嶃€傚鏋滄病鏈夋彁渚涙暟鎹紝鏁版嵁鎸囬拡鍙互鏄?NULL銆傚鏋滀竴鑷存€ф暟鎹笉鍖归厤锛屾暣涓紦瀛樺嵎灏嗚澶辨晥銆?
姝ゅ嚱鏁板彲鑳借繑鍥炶濡?EBUSY锛堝鏋滃嵎閿凡琚竴涓凡鑾峰彇鐨勫嵎浣跨敤锛夋垨 ENOMEM锛堝鏋滃彂鐢熷垎閰嶅け璐ワ級涔嬬被鐨勯敊璇€?濡傛灉 fscache 鏈惎鐢紝瀹冧篃鍙兘杩斿洖 NULL 鍗?cookie銆傚皢 NULL cookie 浼犻€掔粰浠讳綍鎺ュ彈鍗?cookie 鐨勫嚱鏁版槸瀹夊叏鐨勶紝
杩欏皢瀵艰嚧璇ュ嚱鏁颁粈涔堥兘涓嶅仛銆?

褰撶綉缁滄枃浠剁郴缁熺敤瀹屼竴涓嵎鏃讹紝瀹冨簲褰?relinquish 瀹冿細

```

	void fscache_relinquish_volume(struct fscache_volume *volume,
				       const void *coherency_data,
				       bool invalidate);

```
杩欏皢瀵艰嚧璇ュ嵎琚彁浜ゆ垨绉婚櫎锛屽苟涓斿鏋滆 seal锛屼竴鑷存€ф暟鎹皢琚涓烘彁渚涚殑鍊笺€備竴鑷存€ф暟鎹殑澶у皬蹇呴』涓庤幏鍙栬鍗锋椂
鎸囧畾鐨勯暱搴﹀尮閰嶃€傛敞鎰忥紝鍦ㄨ鍗疯 relinquish 涔嬪墠锛屽繀椤?relinquish 鍦ㄨ鍗蜂腑鑾峰彇鐨勬墍鏈夋暟鎹?cookie銆?

## 鏁版嵁鏂囦欢娉ㄥ唽

涓€鏃︽嫢鏈変簡鍗?cookie锛岀綉缁滄枃浠剁郴缁熷氨鍙互鐢ㄥ畠鏉ヨ幏鍙栦竴涓?cookie锛?
```

	struct fscache_cookie *
	fscache_acquire_cookie(struct fscache_volume *volume,
			       u8 advice,
			       const void *index_key,
			       size_t index_key_len,
			       const void *aux_data,
			       size_t aux_data_len,
			       loff_t object_size)

```
杩欎娇鐢ㄦ寚瀹氱殑绱㈠紩閿湪鍗蜂腑鍒涘缓 cookie銆傜储寮曢敭鏄粰瀹氶暱搴︾殑浜岃繘鍒?blob锛屼笖瀵硅鍗峰繀椤诲敮涓€銆傚畠琚繚瀛樺埌 cookie 涓€?鍏跺唴瀹规病鏈夐檺鍒讹紝浣嗗叾闀垮害涓嶅簲瓒呰繃鏈€澶ф枃浠跺悕闀垮害鐨勫ぇ绾﹀洓鍒嗕箣涓夛紝浠ヤ究杩涜缂栫爜銆?
璋冪敤鑰呰繕搴斾紶鍏ヤ竴娈典綅浜?aux_data 涓殑涓€鑷存€ф暟鎹€傚皢鍒嗛厤涓€涓ぇ灏忎负 aux_data_len 鐨勭紦鍐插尯骞跺鍒朵竴鑷存€ф暟鎹€?鍋囧畾鍏跺ぇ灏忛殢鏃堕棿涓嶅彉銆備竴鑷存€ф暟鎹敤浜庢鏌ョ紦瀛樹腑鏁版嵁鐨勬湁鏁堟€с€傛彁渚涗簡鍙洿鏂颁竴鑷存€ф暟鎹殑鍑芥暟銆?
杩樺簲鎻愪緵琚紦瀛樺璞＄殑澶у皬銆傝繖鍙兘鐢ㄤ簬瑁佸壀鏁版嵁锛屽苟灏嗕笌涓€鑷存€ф暟鎹竴鍚屽瓨鍌ㄣ€?
姝ゅ嚱鏁颁粠涓嶈繑鍥為敊璇紝灏界瀹冨彲鑳藉湪鍒嗛厤澶辫触鎴?fscache 鏈惎鐢ㄦ椂杩斿洖 NULL cookie銆備紶鍏?NULL 鍗?cookie 骞跺皢杩斿洖鐨?NULL cookie 浼犻€掔粰浠讳綍鎺ュ彈瀹冪殑鍑芥暟閮芥槸瀹夊叏鐨勩€傝繖灏嗗鑷磋鍑芥暟浠€涔堥兘涓嶅仛銆?

褰撶綉缁滄枃浠剁郴缁熺敤瀹屼竴涓?cookie 鏃讹紝瀹冨簲褰?relinquish 瀹冿細

```

	void fscache_relinquish_cookie(struct fscache_cookie *cookie,
				       bool retire);

```
杩欏皢瀵艰嚧 fscache 鎻愪氦鎴栧垹闄ゆ敮鎾戣 cookie 鐨勫瓨鍌ㄣ€?

## 鏍囪涓€涓?Cookie 鍦ㄤ娇鐢ㄤ腑

涓€鏃︾綉缁滄枃浠剁郴缁熻幏鍙栦簡 cookie锛屾枃浠剁郴缁熷簲鍦ㄥ畠鎵撶畻浣跨敤璇?cookie 鏃讹紙閫氬父鍦ㄦ枃浠舵墦寮€鏃讹級鍛婄煡 fscache锛?
```

	void fscache_use_cookie(struct fscache_cookie *cookie,
				bool will_modify);
	void fscache_unuse_cookie(struct fscache_cookie *cookie,
				  const void *aux_data,
				  const loff_t *object_size);

```
**use** 鍑芥暟鍛婄煡 fscache 瀹冨皢浣跨敤璇?cookie锛屽苟棰濆鎸囩ず鐢ㄦ埛鏄惁鎵撶畻鍦ㄦ湰鍦颁慨鏀瑰唴瀹广€傚鏋滃皻鏈畬鎴愶紝杩欏皢瑙﹀彂
缂撳瓨鍚庣鍘绘敹闆嗗畠璁块棶/瀛樺偍缂撳瓨涓暟鎹墍闇€鐨勮祫婧愩€傝繖鏄湪鍚庡彴瀹屾垚鐨勶紝鍥犳鍦ㄥ嚱鏁拌繑鍥炴椂鍙兘灏氭湭瀹屾垚銆?
**unuse** 鍑芥暟鎸囩ず鏂囦欢绯荤粺宸茬敤瀹屼竴涓?cookie銆傚畠鍙€夊湴鏇存柊瀛樺偍鐨勪竴鑷存€ф暟鎹拰瀵硅薄澶у皬锛岀劧鍚庨€掑噺浣跨敤涓鏁般€?褰撴渶鍚庝竴涓敤鎴峰彇娑堜娇鐢ㄨ cookie 鏃讹紝瀹冨皢琚畨鎺掕繘琛屽瀮鍦惧洖鏀躲€傚鏋滃湪鐭椂闂村唴鏈澶嶇敤锛岃祫婧愬皢琚噴鏀句互鍑忓皯绯荤粺
璧勬簮娑堣€椼€?
鍦ㄨ兘澶熻闂?cookie 杩涜璇汇€佸啓鎴栬皟鏁村ぇ灏忎箣鍓嶏紝蹇呴』灏嗚 cookie 鏍囪涓哄湪浣跨敤涓€斺€斿苟涓斿湪 pagecache 涓瓨鍦ㄨ剰鏁版嵁鏈熼棿
蹇呴』淇濇寔鍦ㄤ娇鐢ㄤ腑鏍囪锛屼互閬垮厤鍦ㄨ繘绋嬮€€鍑烘湡闂村皾璇曟墦寮€鏂囦欢鑰屽鑷?oops銆?
娉ㄦ剰锛屼娇鐢ㄤ腑鏍囪鏄疮绉殑銆傛瘡灏?cookie 鏍囪涓哄湪浣跨敤涓竴娆★紝灏卞繀椤诲彇娑堜娇鐢ㄤ竴娆°€?

## 璋冩暣鏁版嵁鏂囦欢澶у皬锛堟埅鏂級

濡傛灉缃戠粶鏂囦欢绯荤粺鏂囦欢閫氳繃鎴柇鍦ㄦ湰鍦拌璋冩暣澶у皬锛屽垯浣跨敤浠ヤ笅鍑芥暟锛?
```

	void fscache_resize_cookie(struct fscache_cookie *cookie,
				   loff_t new_size);

```
璋冪敤鑰呭繀椤婚鍏堝皢璇?cookie 鏍囪涓哄湪浣跨敤涓€俢ookie 鍜屾柊鐨勫ぇ灏忚浼犲叆锛岀紦瀛樿鍚屾鍦拌皟鏁村ぇ灏忋€傝繖棰勬湡鍦?inode 閿佷笅
浠?`->setattr()` inode 鎿嶄綔涓皟鐢ㄣ€?

## 鏁版嵁 I/O API

瑕佺洿鎺ラ€氳繃涓€涓?cookie 鎵ц鏁版嵁 I/O 鎿嶄綔锛屼娇鐢ㄤ互涓嬪嚱鏁帮細

```

	int fscache_begin_read_operation(struct netfs_cache_resources *cres,
					 struct fscache_cookie *cookie);
	int fscache_read(struct netfs_cache_resources *cres,
			 loff_t start_pos,
			 struct iov_iter *iter,
			 enum netfs_read_from_hole read_hole,
			 netfs_io_terminated_t term_func,
			 void *term_func_priv);
	int fscache_write(struct netfs_cache_resources *cres,
			  loff_t start_pos,
			  struct iov_iter *iter,
			  netfs_io_terminated_t term_func,
			  void *term_func_priv);

```
**begin** 鍑芥暟璁剧疆涓€涓搷浣滐紝灏嗚闂紦瀛樻墍闇€璧勬簮闄勫埌 cookie 鐨勭紦瀛樿祫婧愬潡涓娿€傚亣璁惧畠鏈繑鍥為敊璇紙渚嬪锛屽鏋滅粰瀹?NULL
cookie 瀹冨皢杩斿洖 -ENOBUFS锛屽惁鍒欎粈涔堥兘涓嶅仛锛夛紝閭ｄ箞鍙互鍙戣捣鍙﹀涓や釜鍑芥暟涔嬩竴銆?
**read** 鍜?**write** 鍑芥暟鍙戣捣涓€涓洿鎺?I/O锛坉irect-IO锛夋搷浣溿€備袱鑰呴兘鎺ュ彈鍏堝墠璁剧疆濂界殑缂撳瓨璧勬簮鍧椼€佽捣濮嬫枃浠朵綅缃殑鎸囩ず锛?浠ュ強涓€涓弿杩扮紦鍐插尯骞舵寚鏄庢暟鎹噺鐨?I/O 杩唬鍣ㄣ€?
read 鍑芥暟杩樻帴鍙椾竴涓弬鏁帮紝鎸囩ず瀹冨簲濡備綍澶勭悊纾佺洏鍐呭涓儴鍒嗗～鍏呯殑鍖哄煙锛堢┖娲烇紝hole锛夈€傝繖鍙互鏄拷鐣ュ畠銆佽烦杩囧垵濮嬬┖娲?骞跺湪缂撳啿鍖轰腑濉叆闆讹紝鎴栬€呯粰鍑洪敊璇€?
read 鍜?write 鍑芥暟鍙互缁欏畾涓€涓彲閫夌殑缁堟鍑芥暟锛?
```

	typedef
	void (*netfs_io_terminated_t)(void *priv, ssize_t transferred_or_error,
				      bool was_async);

```
濡傛灉缁欏畾浜嗙粓姝㈠嚱鏁帮紝鎿嶄綔灏嗗紓姝ヨ繍琛岋紝骞跺湪瀹屾垚鏃惰皟鐢ㄧ粓姝㈠嚱鏁般€傚鏋滄湭缁欏畾锛屾搷浣滃皢鍚屾杩愯銆傛敞鎰忥紝鍦ㄥ紓姝ユ儏鍐典笅锛?鎿嶄綔鏈夊彲鑳藉湪鍑芥暟杩斿洖涔嬪墠灏卞凡瀹屾垚銆?
read 鍜?write 鍑芥暟閮戒細鍦ㄥ畬鎴愭椂缁撴潫鎿嶄綔锛宒etach 浠讳綍琚?pin 鐨勮祫婧愩€?
濡傛灉鎿嶄綔杩涜鏈熼棿鍙戠敓浜嗗け鏁堬紝read 鎿嶄綔灏嗕互 ESTALE 澶辫触銆?

## 鏁版嵁鏂囦欢涓€鑷存€?
瑕佽姹傛洿鏂?cookie 涓婄殑涓€鑷存€ф暟鎹拰鏂囦欢澶у皬锛屼娇鐢細

```

	void fscache_update_cookie(struct fscache_cookie *cookie,
				   const void *aux_data,
				   const loff_t *object_size);

```
杩欏皢鏇存柊 cookie 鐨勪竴鑷存€ф暟鎹拰/鎴栨枃浠跺ぇ灏忋€?

## 鏁版嵁鏂囦欢澶辨晥

鏈夋椂鏈夊繀瑕佷娇鍖呭惈鏁版嵁鐨勫璞″け鏁堛€傞€氬父锛屽綋鏈嶅姟鍣ㄩ€氱煡缃戠粶鏂囦欢绯荤粺鍙戠敓浜嗚繙绋嬬涓夋柟鏇存敼鏃惰繖鏄繀瑕佺殑鈥斺€旀鏃舵枃浠?绯荤粺蹇呴』涓㈠純瀹冧负璇ユ枃浠舵寔鏈夌殑鐘舵€佸拰缂撳瓨鏁版嵁锛屽苟浠庢湇鍔″櫒閲嶆柊鍔犺浇銆?
瑕佹寚绀轰竴涓紦瀛樺璞″簲褰撳け鏁堬紝搴斾娇鐢ㄤ互涓嬪嚱鏁帮細

```

	void fscache_invalidate(struct fscache_cookie *cookie,
				const void *aux_data,
				loff_t size,
				unsigned int flags);

```
杩欎細澧炲姞 cookie 涓殑澶辨晥璁℃暟鍣紝瀵艰嚧鏈畬鎴愮殑璇绘搷浣滀互 -ESTALE 澶辫触锛屼粠鎻愪緵鐨勪俊鎭缃竴鑷存€ф暟鎹拰鏂囦欢澶у皬锛岄樆姝?瀵硅 cookie 鐨勬柊 I/O锛屽苟娲剧紦瀛樺幓娓呴櫎鏃ф暟鎹€?
澶辨晥鍦ㄤ竴涓伐浣滅嚎绋嬩腑寮傛杩愯锛屼互鍏嶉樆濉炶繃澶氥€?

## 鍥炲啓璧勬簮绠＄悊

瑕佸皢鏁版嵁浠庣綉缁滄枃浠剁郴缁熷洖鍐欏啓鍏ョ紦瀛橈紝鎵€闇€鐨勭紦瀛樿祫婧愰渶瑕佸湪淇敼鍙戠敓涔嬫椂锛堜緥濡傚綋椤佃鏍囪涓鸿剰鏃讹級琚?pin 浣忥紝鍥犱负鍦?姝ｅ湪閫€鍑虹殑绾跨▼涓棤娉曟墦寮€鏂囦欢銆?
鎻愪緵浜嗕互涓嬭鏂芥潵绠＄悊杩欎竴鐐癸細

 - 鎻愪緵浜嗕竴涓?inode 鏍囧織 `I_PINNING_FSCACHE_WB`锛岀敤浜庢寚绀鸿 inode 鐨?cookie 涓婃寔鏈変竴涓娇鐢ㄤ腑鏍囪銆?   鍙湁鍦ㄦ寔鏈?inode 閿佹椂鎵嶈兘鏇存敼瀹冦€?
 - 涓€涓爣蹇?`unpinned_fscache_wb` 琚斁鍏?`writeback_control` 缁撴瀯锛屽綋 `__writeback_single_inode()`
   鍥犱负鎵€鏈夎剰椤甸兘宸叉竻闄よ€屾竻闄?`I_PINNING_FSCACHE_WB` 鏃惰缃畠銆?
```

	bool fscache_dirty_folio(struct address_space *mapping,
				 struct folio *folio,
				 struct fscache_cookie *cookie);
	void fscache_unpin_writeback(struct writeback_control *wbc,
				     struct fscache_cookie *cookie);
	void fscache_clear_inode_writeback(struct fscache_cookie *cookie,
					   struct inode *inode,
					   const void *aux);

```
**set** 鍑芥暟鏃ㄥ湪浠庢枃浠剁郴缁熺殑 `dirty_folio` 鍦板潃绌洪棿鎿嶄綔璋冪敤銆傚鏋?`I_PINNING_FSCACHE_WB` 鏈缃紝瀹冭缃鏍囧織
骞堕€掑 cookie 鐨勪娇鐢ㄨ鏁帮紙璋冪敤鑰呭繀椤诲凡缁忚皟鐢ㄨ繃 `fscache_use_cookie()`锛夈€?
**unpin** 鍑芥暟鏃ㄥ湪浠庢枃浠剁郴缁熺殑 `write_inode` 瓒呯骇鍧楁搷浣滆皟鐢ㄣ€傚鏋滃湪 writeback_control 缁撴瀯涓缃簡
unpinned_fscache_wb锛屽畠閫氳繃鍙栨秷浣跨敤璇?cookie 鏉ヨ繘琛屽啓鍚庣殑娓呯悊銆?
**clear** 鍑芥暟鏃ㄥ湪浠?netfs 鐨?`evict_inode` 瓒呯骇鍧楁搷浣滆皟鐢ㄣ€傚畠蹇呴』鍦?`truncate_inode_pages_final()`
**涔嬪悗**銆佷絾鍦?`clear_inode()` **涔嬪墠**璋冪敤銆傝繖浼氭竻鐞嗕换浣曟偓鎸傜殑 `I_PINNING_FSCACHE_WB`銆傚畠涔熷厑璁告洿鏂颁竴鑷存€ф暟鎹€?

## 鏈湴淇敼鐨勭紦瀛?
濡傛灉缃戠粶鏂囦欢绯荤粺鏈夋兂瑕佸啓鍏ョ紦瀛樼殑鏈湴淇敼鏁版嵁锛屽畠闇€瑕佹爣璁拌繖浜涢〉浠ユ寚绀哄啓鎿嶄綔姝ｅ湪杩涜锛屽苟涓斿鏋滄爣璁板凡缁忓瓨鍦紝
瀹冮渶瑕佸厛绛夊緟鍏惰绉婚櫎锛堝ぇ姒傛槸鐢变簬宸茬粡鍦ㄨ繘琛屼腑鐨勬搷浣滐級銆傝繖闃叉浜嗗缂撳瓨涓悓涓€瀛樺偍鐨勫涓簰鐩哥珵浜夌殑 DIO 鍐欍€?
棣栧厛锛宯etfs 搴旈€氳繃浠ヤ笅鏂瑰紡纭畾缂撳瓨鏄惁鍙敤锛?
```

	bool caching = fscache_cookie_enabled(cookie);

```
濡傛灉瑕佸皾璇曠紦瀛橈紝搴旂瓑寰呴〉锛岀劧鍚庣敤浠ヤ笅鏂瑰紡鏍囪锛?
```

	void set_page_fscache(struct page *page);
	void wait_on_page_fscache(struct page *page);
	int wait_on_page_fscache_killable(struct page *page);

```
涓€鏃﹁法搴﹀唴鐨勬墍鏈夐〉閮借鏍囪锛宯etfs 灏卞彲浠ヨ姹?fscache锛?
```

	void fscache_write_to_cache(struct fscache_cookie *cookie,
				    struct address_space *mapping,
				    loff_t start, size_t len, loff_t i_size,
				    netfs_io_terminated_t term_func,
				    void *term_func_priv,
				    bool caching)

```
濡傛灉鍦ㄥ埌杈捐鐐逛箣鍓嶅彂鐢熼敊璇紝鍙互绉婚櫎鏍囪锛?
```

	void fscache_clear_page_bits(struct address_space *mapping,
				     loff_t start, size_t len,
				     bool caching)

```
鍦ㄨ繖浜涘嚱鏁颁腑锛屼紶鍏ユ寚鍚戞簮椤垫墍闄勫姞鏄犲皠鐨勬寚閽堬紝start 鍜?len 鎸囩ず灏嗚鍐欏叆鐨勫尯鍩熷ぇ灏忥紙瀹冧笉涓€瀹氶渶瑕佸榻愬埌椤佃竟鐣岋紝
浣嗗繀椤诲鍚庣鏂囦欢绯荤粺涓殑 DIO 杈圭晫瀵归綈锛夈€俢aching 鍙傛暟鎸囩ず鏄惁搴旇烦杩囩紦瀛橈紝鑻ヤ负 false锛岃繖浜涘嚱鏁颁粈涔堥兘涓嶅仛銆?
write 鍑芥暟鎺ュ彈涓€浜涢檮鍔犲弬鏁帮細浠ｈ〃瑕佸啓鍏ョ殑缂撳瓨瀵硅薄鐨?cookie銆乮_size 鎸囩ず netfs 鏂囦欢鐨勫ぇ灏忥紝term_func 鎸囩ず涓€涓?鍙€夌殑瀹屾垚鍑芥暟锛宼erm_func_priv 灏嗕笌閿欒鎴栧啓鍏ラ噺涓€鍚屼紶缁欏畠銆?
娉ㄦ剰锛寃rite 鍑芥暟灏嗗缁堝紓姝ヨ繍琛岋紝骞跺湪瀹屾垚鏃惰皟鐢?term_func 涔嬪墠鍙栨秷瀵规墍鏈夐〉鐨勬爣璁般€?

## 椤甸噴鏀句笌澶辨晥

fscache 璺熻釜鎴戜滑鍒氬垰鍒涘缓鐨勭紦瀛樺璞″湪缂撳瓨涓槸鍚﹀凡鏈変换浣曟暟鎹€傚畠鐭ラ亾鍦ㄥ啓瀹屼笖鍐欏叆鎵€鏉ユ簮鐨勯〉琚?VM 閲婃斁涔嬪墠鏃犻渶杩涜
浠讳綍璇诲彇锛屽湪閭ｄ箣鍚庡畠**蹇呴』**鍘荤紦瀛樹腑鏌ユ壘銆?
瑕佸憡鐭?fscache 涓€涓〉鐜板湪鍙兘宸插湪缂撳瓨涓紝浣跨敤浠ヤ笅鍑芥暟锛?
```

	void fscache_note_page_release(struct fscache_cookie *cookie);

```
濡傛灉椤靛凡琚噴鏀撅紙鍗?release_folio 杩斿洖 true锛夈€?
椤甸噴鏀惧拰椤靛け鏁堜篃搴旂瓑寰呯暀鍦ㄩ〉涓婄殑浠讳綍鏍囪锛?
```

	void wait_on_page_fscache(struct page *page);
	int wait_on_page_fscache_killable(struct page *page);


```
## API 鍑芥暟鍙傝€?