
## 缃戠粶鏂囦欢绯荤粺鏈嶅姟搴擄紙Network Filesystem Services Library锛?

 - 姒傝堪銆?   - 璇锋眰涓庢祦銆?   - 瀛愯姹傘€?   - 缁撴灉鏀堕泦涓庨噸璇曘€?   - 鏈湴缂撳瓨銆?   - 鍐呭鍔犲瘑锛坒scrypt锛夈€? - 姣?inode 涓婁笅鏂囥€?   - inode 涓婁笅鏂囪緟鍔╁嚱鏁般€?   - inode 閿併€?   - inode 鍥炲啓銆? - 楂樺眰 VFS API銆?   - 鏈姞閿佺殑 read/write iter銆?   - 棰勫姞閿佺殑 read/write iter銆?   - 鏁翠綋鏂囦欢 API銆?   - 鍐呭瓨鏄犲皠 I/O API銆? - 楂樺眰 VM API銆?   - 宸插簾寮冪殑 PG_private_2 API銆? - I/O 璇锋眰 API銆?   - 璇锋眰缁撴瀯銆?   - 娴佺粨鏋勩€?   - 瀛愯姹傜粨鏋勩€?   - 鏂囦欢绯荤粺鏂规硶銆?   - 缁堟瀛愯姹傘€?   - 鏈湴缂撳瓨 API銆? - API 鍑芥暟鍙傝€冦€?

## 姒傝堪


缃戠粶鏂囦欢绯荤粺鏈嶅姟搴擄紙netfslib锛夋槸涓€缁勫嚱鏁帮紝鏃ㄥ湪甯姪缃戠粶鏂囦欢绯荤粺瀹炵幇 VM/VFS API 鎿嶄綔銆傚畠鎺ョ浜嗗父瑙勭殑缂撳啿璇汇€侀璇汇€佸啓鍜屽洖鍐欙紝鍚屾椂涔熷鐞嗛潪缂撳啿鍜岀洿鎺?I/O銆?
璇ュ簱鎻愪緵瀵?I/O 澶у皬锛堥噸鏂帮級鍗忓晢銆侀噸璇曞け璐ョ殑 I/O 浠ュ強鏈湴缂撳瓨鐨勬敮鎸侊紝骞朵笖鏈潵杩樺皢鎻愪緵鍐呭鍔犲瘑銆?
瀹冨敖鍙兘鍦板皢鏂囦欢绯荤粺涓?VM 鎺ュ彛鐨勫彉鍖栭殧绂诲紑鏉ワ紝骞跺鐞嗚濡傚ぇ鍨嬪椤?folio 涔嬬被鐨?VM 鐗规€с€傛枃浠剁郴缁熷熀鏈笂鍙渶瑕佹彁渚涗竴绉嶆墽琛岃鍐?RPC 璋冪敤鐨勬柟娉曘€?
netfslib 鍐呴儴缁勭粐 I/O 鐨勬柟寮忕敱鑻ュ共瀵硅薄鏋勬垚锛?
 - 涓€涓?*璇锋眰锛坮equest锛?*銆傝姹傜敤浜庤窡韪?I/O 鐨勬暣浣撹繘搴﹀苟鎸佹湁璧勬簮銆傜粨鏋滅殑鏀堕泦鍦ㄨ姹傚眰杩涜銆傝姹傚唴鐨?I/O 琚垝鍒嗕负鑻ュ共骞惰鐨勫瓙璇锋眰娴併€?
 - 涓€涓?*娴侊紙stream锛?*銆備竴缁勪簰涓嶉噸鍙犵殑瀛愯姹傚簭鍒椼€傛祦鍐呯殑瀛愯姹備笉蹇呮槸杩炵画鐨勩€?
 - 涓€涓?*瀛愯姹傦紙subrequest锛?*銆傝繖鏄?I/O 鐨勫熀鏈崟鍏冦€傚畠浠ｈ〃涓€娆″崟鐙殑 RPC 璋冪敤鎴栦竴娆″崟鐙殑缂撳瓨 I/O 鎿嶄綔銆傚簱灏嗚繖浜涗紶閫掔粰鏂囦欢绯荤粺鍜岀紦瀛樺幓鎵ц銆?
### 璇锋眰涓庢祦


褰撶湡姝ｆ墽琛?I/O 鏃讹紙涓庝粎浠呭鍒跺埌 pagecache 鐩稿锛夛紝netfslib 浼氬垱寤轰竴涓垨澶氫釜璇锋眰鏉ヨ窡韪?I/O 鐨勮繘搴﹀苟鎸佹湁璧勬簮銆?
璇绘搷浣滃皢鍙湁涓€涓祦锛岃娴佸唴鐨勫瓙璇锋眰鍙兘鏉ヨ嚜娣峰悎鐨勬潵婧愶紝渚嬪娣峰悎 RPC 瀛愯姹傚拰缂撳瓨瀛愯姹傘€?
鍙︿竴鏂归潰锛屽啓鎿嶄綔鍙兘鏈夊涓祦锛屽叾涓瘡涓祦闈㈠悜涓嶅悓鐨勭洰鏍囥€備緥濡傦紝鍙兘鏈変竴涓祦鍐欏叆鏈湴缂撳瓨锛屽彟涓€涓祦鍐欏叆鏈嶅姟鍣ㄣ€傜洰鍓嶅彧鍏佽涓や釜娴侊紝浣嗗鏋滈渶瑕佸澶氫釜鏈嶅姟鍣ㄨ繘琛屽苟琛屽啓鍏ワ紝鍙互澧炲姞銆?
鍐欐祦涓殑瀛愯姹備笉闇€瑕佷笌鍙︿竴涓啓娴佷腑鐨勫瓙璇锋眰瀵归綈鎴栧ぇ灏忎竴鑷达紝netfslib 浼氱嫭绔嬪湴灏嗘瘡涓祦涓殑瀛愯姹傚钩閾哄埌婧愮紦鍐插尯涓娿€傛澶栵紝姣忎釜娴佸彲鑳藉寘鍚笌鍙︿竴涓祦涓殑绌烘礊涓嶅搴旂殑绌烘礊銆?
鍙﹀锛屽瓙璇锋眰涓嶉渶瑕佷笌婧?鐩爣缂撳啿鍖轰腑 folio 鎴?vector 鐨勮竟鐣屽搴斻€傚簱璐熻矗缁撴灉鐨勬敹闆嗕互鍙?folio 鏍囧織涓庡紩鐢ㄧ殑澶勭悊銆?
### 瀛愯姹?

瀛愯姹傛槸 netfslib 涓庝娇鐢ㄥ畠鐨勬枃浠剁郴缁熶箣闂翠氦浜掔殑鏍稿績銆傛瘡涓瓙璇锋眰搴斿綋瀵瑰簲浜庝竴娆″崟鐙殑璇绘垨鍐?RPC 鎴栫紦瀛樻搷浣溿€傚簱浼氬皢涓€缁勫瓙璇锋眰鐨勭粨鏋滄嫾鎺ヨ捣鏉ワ紝浠ユ彁渚涙洿楂樺眰娆＄殑鎿嶄綔銆?
netfslib 鍦ㄥ缓绔嬪瓙璇锋眰鏃讹紝涓庢枃浠剁郴缁熸垨缂撳瓨鏈変袱娆′氦浜掋€傞鍏堬紝鏈変竴涓彲閫夌殑鍑嗗姝ラ锛屽厑璁告枃浠剁郴缁熷崗鍟嗗瓙璇锋眰鐨勯檺鍒讹紝鍖呮嫭鏈€澶у瓧鑺傛暟鍜屾渶澶?vector 鏁帮紙渚嬪鐢ㄤ簬 RDMA锛夈€傝繖鍙兘娑夊強涓庢湇鍔″櫒鍗忓晢锛堜緥濡?cifs 闇€瑕佽幏鍙栦俊鐢ㄩ搴︼級銆?
鍏舵锛屾槸鍒嗗彂姝ラ锛屽湪姝ゆ楠や腑瀛愯姹傝绉讳氦缁欐枃浠剁郴缁熸墽琛屻€?
娉ㄦ剰锛岃鍜屽啓涔嬮棿杩欎袱涓楠ょ殑鍋氭硶鐣ユ湁涓嶅悓锛?
 - 瀵逛簬璇伙紝VM/VFS 浼氶鍏堝憡鐭ユ垜浠璇锋眰澶氬皯鏁版嵁锛屽洜姝ゅ簱鍙互棰勮鏈€澶у€硷紝鐒跺悗缂撳瓨浠ュ強涔嬪悗鐨勬枃浠剁郴缁熷彲浠ラ€愭鍑忓皬璇ュ€笺€傜紦瀛樹篃浼氶鍏堣鍜ㄨ鏄惁鎯宠鎵ц璇伙紝鐒跺悗鎵嶅挩璇㈡枃浠剁郴缁熴€?
 - 瀵逛簬鍥炲啓锛屽湪閬嶅巻 pagecache 涔嬪墠锛屾棤娉曠煡閬撳皢瑕佸啓鍏ュ灏戞暟鎹紝鍥犳搴撲笉璁剧疆浠讳綍闄愬埗銆?
涓€鏃﹀瓙璇锋眰瀹屾垚锛屾枃浠剁郴缁熸垨缂撳瓨浼氶€氱煡搴撳畬鎴愶紝鐒跺悗璋冪敤缁撴灉鏀堕泦銆傛牴鎹姹傛槸鍚屾杩樻槸寮傛锛岀粨鏋滅殑鏀堕泦灏嗗湪搴旂敤绋嬪簭绾跨▼鎴栧伐浣滈槦鍒椾腑杩涜銆?
### 缁撴灉鏀堕泦涓庨噸璇?

闅忕潃瀛愯姹傜殑瀹屾垚锛屽簱浼氭敹闆嗗拰鏁寸悊缁撴灉锛屽苟閫愭鎵ц folio 瑙ｉ攣锛堝鏋滃悎閫傦級銆備竴鏃﹁姹傚畬鎴愶紝灏嗚皟鐢ㄥ紓姝ュ畬鎴愶紙鍚屾牱鍦帮紝濡傛灉鍚堥€傦級銆傛枃浠剁郴缁熷彲浠ュ悜搴撴彁渚涗复鏃剁殑杩涘害鎶ュ憡锛屼互渚垮湪鍙兘鐨勬儏鍐典笅浣?folio 瑙ｉ攣鏇存棭鍙戠敓銆?
濡傛灉鏈変换浣曞瓙璇锋眰澶辫触锛宯etfslib 鍙互閲嶈瘯瀹冧滑銆傚畠浼氱瓑寰呮墍鏈夊瓙璇锋眰瀹屾垚锛岀粰鏂囦欢绯荤粺鏈轰細鍘昏皟鏁磋姹傛寔鏈夌殑璧勬簮/鐘舵€侊紝骞跺湪閲嶆柊鍑嗗鍜岄噸鏂板垎鍙戝瓙璇锋眰涔嬪墠瀵瑰叾杩涜澶勭悊銆?
杩欏厑璁告敼鍙樻祦涓竴缁勮繛缁け璐ュ瓙璇锋眰鐨勫钩閾烘柟寮忥紝鏍规嵁闇€瑕佸鍔犲瓙璇锋眰鎴栦涪寮冨浣欑殑閮ㄥ垎锛堜緥濡傦紝濡傛灉缃戠粶澶у皬鍙戠敓鍙樺寲锛屾垨鑰呮湇鍔″櫒鍐冲畾闇€瑕佹洿灏忕殑鍧楋級銆?
姝ゅ锛屽鏋滀竴涓垨澶氫釜杩炵画鐨勭紦瀛樿瀛愯姹傚け璐ワ紝搴撲細灏嗗叾浜ょ粰鏂囦欢绯荤粺鎵ц锛屽苟鏍规嵁鏂囦欢绯荤粺鐨勫弬鏁帮紙鑰岄潪缂撳瓨鐨勫弬鏁帮級閲嶆柊鍗忓晢骞堕噸鏂板钩閾哄畠浠€?
### 鏈湴缂撳瓨


netfslib 閫氳繃 `fscache` 鎻愪緵鐨勬湇鍔′箣涓€锛屾槸閫夋嫨灏嗘潵鑷?鍐欏叆缃戠粶鏂囦欢绯荤粺鐨勬暟鎹壇鏈紦瀛樺湪鏈湴纾佺洏涓娿€傚鏋滄湁涓€涓?cookie 闄勫姞鍒?`netfs_inode` 涓婏紝搴撳皢鑷姩浠ｈ〃鏂囦欢绯荤粺绠＄悊鏁版嵁鐨勫瓨鍌ㄣ€佹绱㈠拰閮ㄥ垎澶辨晥銆?
娉ㄦ剰锛屾湰鍦扮紦瀛樿繃鍘讳娇鐢?PG_private_2锛堝埆鍚嶄负 PG_fscache锛夋潵璺熻釜姝ｅ湪鍐欏叆缂撳瓨鐨勯〉锛屼絾鐜板湪宸茬粡搴熷純锛屽洜涓?PG_private_2 灏嗚绉婚櫎銆?
鐩稿弽锛屼粠鏈嶅姟鍣ㄨ鍙栫殑銆佽€岀紦瀛樹腑娌℃湁鏁版嵁鐨?folio 灏嗚鏍囪涓鸿剰锛屽苟灏?`folio->private` 璁剧疆涓轰竴涓壒娈婂€硷紙`NETFS_FOLIO_COPY_TO_CACHE`锛夛紝骞剁暀寰呭洖鍐欏啓鍏ャ€傚鏋滃湪璇ユ搷浣滃彂鐢熶箣鍓?folio 琚慨鏀癸紝璇ョ壒娈婂€煎皢琚竻闄わ紝璇ュ啓鍏ュ皢鍙樹负鏅€氱殑鑴忕姸鎬併€?
褰撳洖鍐欏彂鐢熸椂锛屽姝ゆ爣璁扮殑 folio 灏嗗彧鍐欏叆缂撳瓨鑰屼笉鍐欏叆鏈嶅姟鍣ㄣ€傚洖鍐欓€氳繃浣跨敤涓や釜娴佹潵澶勭悊娣峰悎鐨勪粎缂撳瓨鍐欏叆鍜屾湇鍔″櫒涓庣紦瀛樺啓鍏ワ紝涓€涓彂寰€缂撳瓨锛屼竴涓彂寰€鏈嶅姟鍣ㄣ€傛湇鍔″櫒娴佷腑灏嗗寘鍚笌杩欎簺 folio 瀵瑰簲鐨勯棿闅欍€?
### 鍐呭鍔犲瘑锛坒scrypt锛?

灏界鐩墠杩樻病鏈夎繖鏍峰仛锛屼絾 netfslib 缁堝皢鑾峰緱浠ｈ〃缃戠粶鏂囦欢绯荤粺锛堜緥濡?Ceph锛夋墽琛屽鎴风鍐呭鍔犲瘑鐨勮兘鍔涖€傚鏋滃悎閫傦紙涔熷彲鑳戒笉鍚堥€傦紝渚嬪 cifs锛夛紝鍙互浣跨敤 fscrypt銆?
鏁版嵁灏嗕娇鐢ㄤ笌鍐欏叆鏈嶅姟鍣ㄧ殑鏁版嵁鐩稿悓鐨勫姞瀵嗘柟寮忓姞瀵嗗悗瀛樺偍鍦ㄦ湰鍦扮紦瀛樹腑锛屽簱灏嗗湪蹇呰鏃跺疄鏂藉弽寮圭紦鍐插拰 RMW 鍛ㄦ湡銆?

## 姣?inode 涓婁笅鏂?

缃戠粶鏂囦欢绯荤粺杈呭姪搴撻渶瑕佷负姣忎釜鐢卞叾甯姪绠＄悊鐨?netfs inode 瀛樺偍涓€浜涚姸鎬併€備负姝わ紝鎻愪緵浜嗕竴涓笂涓嬫枃
```

	struct netfs_inode {
		struct inode inode;
		const struct netfs_request_ops *ops;
		struct fscache_cookie * cache;
		loff_t remote_i_size;
		unsigned long flags;
		...
	};

```
鎯宠浣跨敤 netfslib 鐨勭綉缁滄枃浠剁郴缁熷繀椤诲皢姝ょ粨鏋勬斁鍏ュ叾 inode 灏佽缁撴瀯浣撲腑锛屼互鏇夸唬 VFS 鐨?`struct inode`銆傝繖鍙互閫氳繃浠ヤ笅鏂瑰紡瀹屾垚
```

	struct my_inode {
		struct netfs_inode netfs; /* Netfslib context and vfs inode */
		...
	};

```
杩欎娇寰?netfslib 鑳藉閫氳繃 `container_of()` 浠?inode 鎸囬拡鎵惧埌鍏剁姸鎬侊紝浠庤€屽厑璁?netfslib 杈呭姪鍑芥暟琚?VFS/VM 鎿嶄綔琛ㄧ洿鎺ユ寚鍚戙€?
璇ョ粨鏋勪腑鍖呭惈鏂囦欢绯荤粺鎰熷叴瓒ｇ殑浠ヤ笅瀛楁锛?
 - `inode`

   VFS inode 缁撴瀯銆?
 - `ops`

   缃戠粶鏂囦欢绯荤粺鎻愪緵缁?netfslib 鐨勪竴缁勬搷浣溿€?
 - `cache`

   鏈湴缂撳瓨 cookie锛屽鏋滄湭鍚敤缂撳瓨鍒欎负 NULL銆傚鏋滅鐢ㄤ簡 fscache锛岃瀛楁涓嶅瓨鍦ㄣ€?
 - `remote_i_size`

   鏈嶅姟鍣ㄤ笂鏂囦欢鐨勫ぇ灏忋€傚鏋滃凡杩涜浜嗘湰鍦颁慨鏀逛絾灏氭湭鍐欏洖锛屽垯璇ュ€间笌 inode->i_size 涓嶅悓銆?
 - `flags`

   涓€缁勬爣蹇楋紝鍏朵腑涓€浜涙枃浠剁郴缁熷彲鑳芥劅鍏磋叮锛?
   - `NETFS_ICTX_MODIFIED_ATTR`

     濡傛灉 netfslib 淇敼浜?mtime/ctime 鍒欒缃€傛枃浠剁郴缁熷彲浠ヨ嚜鐢卞拷鐣ユ垨娓呴櫎瀹冦€?
   - `NETFS_ICTX_UNBUFFERED`

     瀵硅鏂囦欢鎵ц闈炵紦鍐?I/O銆傜被浼间簬鐩存帴 I/O锛屼絾娌℃湁瀵归綈闄愬埗銆傚繀瑕佹椂灏嗘墽琛?RMW銆傞櫎闈炲悓鏃朵娇鐢?mmap()锛屽惁鍒欎笉浼氫娇鐢?pagecache銆?
   - `NETFS_ICTX_WRITETHROUGH`

     瀵硅鏂囦欢鎵ц鐩村啓锛坵ritethrough锛夌紦瀛樸€傚綋鍚戦〉缂撳瓨杩涜缂撳啿鍐欐椂锛孖/O 灏嗚寤虹珛鍜屽垎鍙戙€俶map() 鎵ц姝ｅ父鐨勫洖鍐欐搷浣溿€?
   - `NETFS_ICTX_SINGLE_NO_UPLOAD`

     濡傛灉璇ユ枃浠剁殑鍐呭鏄暣鍧楋紙monolithic锛夌殑锛屽繀椤讳竴娆℃€ф暣浣撹鍙栦笖涓嶅緱鍐欏洖鏈嶅姟鍣紝鍒欒缃鏍囧織锛屼絾鍙互缂撳瓨锛堜緥濡?AFS 鐩綍锛夈€?
### inode 涓婁笅鏂囪緟鍔╁嚱鏁?

涓轰簡甯姪澶勭悊姣?inode 涓婁笅鏂囷紝鎻愪緵浜嗕竴缁勮緟鍔╁嚱鏁般€傞鍏堬紝涓€涓敤浜庡涓婁笅鏂囨墽琛屽熀鏈垵濮嬪寲鐨勫嚱鏁?```

	void netfs_inode_init(struct netfs_inode *ctx,
			      const struct netfs_request_ops *ops);

```
```

	struct netfs_inode *netfs_inode(struct inode *inode);

```
鏈€鍚庯紝涓€涓敤浜庝粠涓婁笅鏂囪幏鍙栫紦瀛?cookie 鎸囬拡鐨勫嚱鏁?```

	struct fscache_cookie *netfs_i_cookie(struct netfs_inode *ctx);

```
### inode 閿?

鎻愪緵浜嗕竴缁勫嚱鏁扮敤浜庣鐞?I/O 鍜?mmap 鐨?i_rwsem 閿?```

	int netfs_start_io_read(struct inode *inode);
	void netfs_end_io_read(struct inode *inode);
	int netfs_start_io_write(struct inode *inode);
	void netfs_end_io_write(struct inode *inode);
	int netfs_start_io_direct(struct inode *inode);
	void netfs_end_io_direct(struct inode *inode);

```
鎺掍粬鎬у垎涓哄洓涓嫭绔嬬殑绫诲埆锛?
 1) 缂撳啿璇诲拰鍐欍€?
    缂撳啿璇诲彲浠ュ郊姝ゅ苟鍙戣繍琛岋紝涔熷彲浠ヤ笌缂撳啿鍐欏苟鍙戣繍琛岋紝浣嗙紦鍐插啓褰兼涔嬮棿涓嶈兘骞跺彂杩愯銆?
 2) 鐩存帴璇诲拰鍐欍€?
    鐩存帴锛堜笌闈炵紦鍐诧級璇诲拰鍐欏彲浠ュ苟鍙戣繍琛岋紝鍥犱负瀹冧滑涓嶅叡浜湰鍦扮紦鍐诧紙鍗?pagecache锛夛紝骞朵笖鍦ㄧ綉缁滄枃浠剁郴缁熶腑锛岄鏈熸帓浠栨€х敱鏈嶅姟鍣ㄧ鐞嗭紙灏界瀵逛簬 Ceph 绛夋儏鍐靛彲鑳藉苟闈炲姝わ級銆?
 3) 鍏朵粬涓昏鐨?inode 淇敼鎿嶄綔锛堜緥濡?truncate銆乫allocate锛夈€?
    杩欎簺搴旂洿鎺ヨ闂?i_rwsem銆?
 4) mmap()銆?
    mmap 鏄犲皠鐨勮闂彲鑳戒笌鍏朵粬浠讳綍绫诲埆骞跺彂杩愯銆傚畠浠彲鑳芥瀯鎴愭枃浠跺唴鐜洖 DIO 璇?鍐欑殑缂撳啿鍖恒€傚畠浠彲鑳借鍏佽鍑虹幇鍦ㄩ潪缂撳啿鏂囦欢涓娿€?
### inode 鍥炲啓


褰?inode 琚紕鑴忔椂锛宯etfslib 浼氫负鏈潵鐨勫洖鍐欏浐瀹?inode 涓婄殑璧勬簮锛堜緥濡傚浐瀹?fscache cookie 鐨勪娇鐢級銆傜劧鑰岋紝杩欑鍥哄畾闇€瑕佽皑鎱庣鐞嗐€備负浜嗙鐞嗗浐瀹氾紝浼氬彂鐢熶互涓嬪簭鍒楋細

 1) 褰撳浐瀹氬紑濮嬫椂锛堜緥濡傚綋鏌愪釜 folio 琚紕鑴忔椂锛夛紝濡傛灉缂撳瓨澶勪簬娲诲姩鐘舵€侊紝netfslib 浼氳缃竴涓?inode 鐘舵€佹爣蹇?`I_PINNING_NETFS_WB`锛屼互闃绘缂撳瓨缁撴瀯琚涪寮冧互鍙婄紦瀛樼┖闂磋鍥炴敹銆傚鏋滆鏍囧織宸茶缃紝杩欎篃鍙互闃叉閲嶆柊鑾峰彇缂撳瓨璧勬簮銆?
 2) 璇ユ爣蹇楅殢鍚庡湪 VM 涓殑 inode 閿佸唴銆乮node 鍥炲啓鏈熼棿琚竻闄も€斺€斿苟涓斿叾宸茶璁剧疆鐨勪簨瀹炶杞Щ鍒?`struct writeback_control` 涓殑 `->unpinned_netfs_wb`銆?
 3) 濡傛灉鐜板湪璁剧疆浜?`->unpinned_netfs_wb`锛屽垯寮哄埗璋冪敤 write_inode 杩囩▼銆?
 4) 璋冪敤鏂囦欢绯荤粺鐨?`->write_inode()` 鍑芥暟杩涜娓呯悊銆?
 5) 鏂囦欢绯荤粺璋冪敤 netfs 杩涜娓呯悊銆?```

	int netfs_unpin_writeback(struct inode *inode, struct writeback_control *wbc);

```
濡傛灉鏂囦欢绯荤粺涓嶉渶瑕佸仛鍏朵粬浜嬫儏锛屽彲浠ュ皢鍏惰缃负瀹冪殑 `.write_inode` 鏂规硶銆?
姝ゅ锛屽鏋滀竴涓?inode 琚垹闄わ紝鏂囦欢绯荤粺鐨?write_inode 鏂规硶鍙兘涓嶄細
```

	void netfs_clear_inode_writeback(struct inode *inode, const void *aux);

```
蹇呴』鍦?`->evict_inode()` 涓€佸湪璋冪敤 `clear_inode()` **涔嬪墠**璋冪敤銆?

## 楂樺眰 VFS API


netfslib 鎻愪緵澶氱粍 API 璋冪敤锛屼緵鏂囦欢绯荤粺灏?VFS 鎿嶄綔濮旀墭缁欏畠銆俷etfslib 鍙嶈繃鏉ヤ細璋冪敤鏂囦欢绯荤粺鍜岀紦瀛樻潵鍗忓晢 I/O 澶у皬銆佸彂鍑?RPC锛屽苟鍦ㄤ笉鍚屾椂鏈烘彁渚涘叾浠嬪叆鐨勪綅缃€?
### 鏈姞閿佺殑 Read/Write Iter


绗竴缁?API 鐢ㄤ簬鍦ㄦ枃浠剁郴缁熼€氳繃鏍囧噯 VFS 鏂规硶琚皟鐢ㄣ€佷絾闇€瑕佸厛鎴栧悗鍋氬叾浠栦簨鎯呫€佸悓鏃朵粛澶勪簬鍔犻攣鍖烘鍐呮椂锛屽皢鎿嶄綔濮旀墭缁?netfslib
```

	ssize_t netfs_file_read_iter(struct kiocb *iocb, struct iov_iter *iter);
	ssize_t netfs_file_write_iter(struct kiocb *iocb, struct iov_iter *from);
	ssize_t netfs_buffered_read_iter(struct kiocb *iocb, struct iov_iter *iter);
	ssize_t netfs_unbuffered_read_iter(struct kiocb *iocb, struct iov_iter *iter);
	ssize_t netfs_unbuffered_write_iter(struct kiocb *iocb, struct iov_iter *from);

```
瀹冧滑鍙互鐩存帴璧嬬粰 `.read_iter` 鍜?`.write_iter`銆傚畠浠嚜宸辨墽琛?inode 閿侊紝鍓嶄袱涓細鍦ㄧ紦鍐?I/O 鍜?DIO 涔嬮棿鎸夐渶鍒囨崲銆?
### 棰勫姞閿佺殑 Read/Write Iter


绗簩缁?API 鐢ㄤ簬鍦ㄦ枃浠剁郴缁熼€氳繃鏍囧噯 VFS 鏂规硶琚皟鐢ㄣ€佷絾闇€瑕佸厛鎴栧悗鍋氬叾浠栦簨鎯呫€佸悓鏃朵粛澶勪簬鍔犻攣鍖烘鍐呮椂锛屽皢鎿嶄綔濮旀墭缁?netfslib
```

	ssize_t netfs_unbuffered_read_iter_locked(struct kiocb *iocb, struct iov_iter *iter);

```
瀹冧笉鑳界洿鎺ヨ祴缁?`.read_iter`锛屾枃浠剁郴缁熻礋璐ｅ湪璋冪敤瀹冧箣鍓嶆墽琛?inode 閿併€傚浜庣紦鍐茶锛屾枃浠剁郴缁熷簲浣跨敤 `filemap_read()`銆?```

	ssize_t netfs_buffered_write_iter_locked(struct kiocb *iocb, struct iov_iter *from,
					 struct netfs_group *netfs_group);
	ssize_t netfs_perform_write(struct kiocb *iocb, struct iov_iter *iter,
				    struct netfs_group *netfs_group);
	ssize_t netfs_unbuffered_write_iter_locked(struct kiocb *iocb, struct iov_iter *iter,
						   struct netfs_group *netfs_group);

```
杩欎簺涓嶈兘鐩存帴璧嬬粰 `.write_iter`锛屾枃浠剁郴缁熻礋璐ｅ湪璋冪敤瀹冧滑涔嬪墠鎵ц inode 閿併€?
鍓嶄袱涓嚱鏁扮敤浜庣紦鍐插啓锛涚涓€涓彧鏄坊鍔犱竴浜涙爣鍑嗗啓妫€鏌ュ苟璺宠浆鍒扮浜屼釜锛屼絾濡傛灉鏂囦欢绯荤粺鎯宠鑷繁鍋氭鏌ワ紝瀹冨彲浠ョ洿鎺ヤ娇鐢ㄧ浜屼釜銆傜涓変釜鍑芥暟鐢ㄤ簬闈炵紦鍐叉垨 DIO 鍐欍€?
鍦ㄨ繖涓変釜鍐欏嚱鏁颁笂锛岄兘鏈変竴涓洖鍐欑粍鎸囬拡锛堝鏋滄枃浠剁郴缁熶笉浣跨敤鍒欎负 NULL锛夈€傚洖鍐欑粍鍦?folio 琚慨鏀规椂璁剧疆鍦?folio 涓娿€傚鏋滆淇敼鐨?folio 宸茬粡鏍囪浜嗕笉鍚岀殑缁勶紝鍒欏厛灏嗗叾鍒峰嚭銆傚洖鍐?API 鍏佽鍐欏洖鐗瑰畾鐨勭粍銆?
### 鍐呭瓨鏄犲皠 I/O API


```

	vm_fault_t netfs_page_mkwrite(struct vm_fault *vmf, struct netfs_group *netfs_group);

```
杩欎娇寰楁枃浠剁郴缁熷彲浠ュ皢 `.page_mkwrite` 濮旀墭缁?netfslib銆傛枃浠剁郴缁熶笉搴斿湪璋冪敤瀹冧箣鍓嶈幏鍙?inode 閿侊紝浣嗕笌涓婇潰鐨勫姞閿佸啓鍑芥暟涓€鏍凤紝瀹冪‘瀹炲甫鏈変竴涓洖鍐欑粍鎸囬拡銆傚鏋滆鍙樹负鍙啓鐨勯〉灞炰簬涓嶅悓鐨勭粍锛屽垯浼氬厛灏嗗叾鍒峰嚭銆?
### 鏁翠綋鏂囦欢 API


杩樻湁涓€缁勭壒娈婄殑 API锛岀敤浜庨偅浜涘唴瀹瑰繀椤婚€氳繃鍗曟 RPC 璇诲彇锛堜笖涓嶅啓鍥烇級銆佸苟浣滀负鏁翠綋鍧楃淮鎶ょ殑鏂囦欢
```

	ssize_t netfs_read_single(struct inode *inode, struct file *file, struct iov_iter *iter);
	void netfs_single_mark_inode_dirty(struct inode *inode);
	int netfs_writeback_single(struct address_space *mapping,
				   struct writeback_control *wbc,
				   struct iov_iter *iter);

```
绗竴涓嚱鏁颁粠鏂囦欢璇诲彇鍒扮粰瀹氱紦鍐插尯锛屽鏋滄暟鎹凡鍦ㄧ紦瀛樹腑鍒欎紭鍏堜粠缂撳瓨璇诲彇锛涚浜屼釜鍑芥暟鍏佽灏?inode 鏍囪涓鸿剰锛屼粠鑰屽紩鍙戝悗缁殑鍥炲啓锛涚涓変釜鍑芥暟鍙敱鍥炲啓浠ｇ爜璋冪敤锛屼互灏嗘暟鎹啓鍏ョ紦瀛橈紙濡傛灉瀛樺湪锛夈€?
濡傛灉浣跨敤姝?API锛宨node 搴旀爣璁颁负 `NETFS_ICTX_SINGLE_NO_UPLOAD`銆傚洖鍐欏嚱鏁拌姹傜紦鍐插尯涓?ITER_FOLIOQ 绫诲瀷銆?
## 楂樺眰 VM API


netfslib 杩樻彁渚涘缁?API 璋冪敤锛屼緵鏂囦欢绯荤粺灏?VM 鎿嶄綔濮旀墭缁欏畠銆傚悓鏍峰湴锛宯etfslib 鍙嶈繃鏉ヤ細璋冪敤鏂囦欢绯荤粺鍜岀紦瀛樻潵鍗忓晢 I/O 澶у皬銆佸彂鍑?RPC 骞舵彁渚涗粙鍏ヤ綅缃?```

	void netfs_readahead(struct readahead_control *);
	int netfs_read_folio(struct file *, struct folio *);
	int netfs_writepages(struct address_space *mapping,
			     struct writeback_control *wbc);
	bool netfs_dirty_folio(struct address_space *mapping, struct folio *folio);
	void netfs_invalidate_folio(struct folio *folio, size_t offset, size_t length);
	bool netfs_release_folio(struct folio *folio, gfp_t gfp);

```
杩欎簺鏄?`address_space_operations` 鏂规硶锛屽彲浠ョ洿鎺ヨ缃湪鎿嶄綔琛ㄤ腑銆?
### 宸插簾寮冪殑 PG_private_2 API


杩樻湁涓€涓敤浜庝粛浣跨敤宸插簾寮?PG_private_2 鏍囧織鐨勬枃浠剁郴缁熺殑搴熷純鍑芥暟
```

	int netfs_write_begin(struct netfs_inode *inode, struct file *file,
			      struct address_space *mapping, loff_t pos, unsigned int len,
			      struct folio **_folio, void **_fsdata);

```
瀹冧娇鐢ㄤ簡宸插簾寮冪殑 PG_private_2 鏍囧織锛屽洜姝や笉搴旇浣跨敤銆?

## I/O 璇锋眰 API


I/O 璇锋眰 API 鍖呭惈鑻ュ共缁撴瀯浠ュ強鏂囦欢绯荤粺鍙兘闇€瑕佷娇鐢ㄧ殑鑻ュ共鍑芥暟銆?
### 璇锋眰缁撴瀯


璇锋眰缁撴瀯绠＄悊鏁翠釜璇锋眰锛屾寔鏈変竴浜涜祫婧?```

	struct netfs_io_request {
		enum netfs_io_origin	origin;
		struct inode		*inode;
		struct address_space	*mapping;
		struct netfs_group	*group;
		struct netfs_io_stream	io_streams[];
		void			*netfs_priv;
		void			*netfs_priv2;
		unsigned long long	start;
		unsigned long long	len;
		unsigned long long	i_size;
		unsigned int		debug_id;
		unsigned long		flags;
		...
	};

```
璁稿瀛楁渚涘唴閮ㄤ娇鐢紝浣嗘澶勬樉绀虹殑瀛楁鏄枃浠剁郴缁熸劅鍏磋叮鐨勶細

 - `origin`

   璇锋眰鐨勬潵婧愶紙棰勮銆乺ead_folio銆丏IO 璇汇€佸洖鍐欑瓑锛夈€?
 - `inode`
 - `mapping`

   琚鍙栨枃浠剁殑 inode 鍜屽湴鍧€绌洪棿銆俶apping 鍙兘鎸囧悜涔熷彲鑳戒笉鎸囧悜 inode->i_data銆?
 - `group`

   姝よ姹傛鍦ㄥ鐞嗙殑鍥炲啓缁勶紝鎴?NULL銆傝繖鎸佹湁瀵硅缁勭殑涓€涓紩鐢ㄣ€?
 - `io_streams`

   璇锋眰鍙敤鐨勫苟琛屽瓙璇锋眰娴併€傜洰鍓嶆湁涓や釜鍙敤锛屼絾鏈潵鍙兘鍋氭垚鍙墿灞曠殑銆俙NR_IO_STREAMS` 鎸囩ず璇ユ暟缁勭殑澶у皬銆?
 - `netfs_priv`
 - `netfs_priv2`

   缃戠粶鏂囦欢绯荤粺鐨勭鏈夋暟鎹€傝鍊煎彲浠ュ湪璋冪敤杈呭姪鍑芥暟鏃朵紶鍏ワ紝涔熷彲浠ュ湪璇锋眰鏈熼棿璁剧疆銆?
 - `start`
 - `len`

   璇昏姹傝捣濮嬩綅缃殑鏂囦欢鍋忕Щ鍜岄暱搴︺€傝繖浜涘彲鑳借 ->expand_readahead() 鎿嶄綔淇敼銆?
 - `i_size`

   璇锋眰寮€濮嬫椂鏂囦欢鐨勫ぇ灏忋€?
 - `debug_id`

   涓烘鎿嶄綔鍒嗛厤鐨勪竴涓紪鍙凤紝鍙湪 trace 琛屼腑鏄剧ず浠ヤ緵鍙傝€冦€?
 - `flags`

   鐢ㄤ簬绠＄悊鍜屾帶鍒惰姹傛搷浣滅殑鏍囧織銆傚叾涓竴浜涘彲鑳藉紩璧锋枃浠剁郴缁熺殑鍏磋叮锛?
   - `NETFS_RREQ_RETRYING`

     netfslib 鍦ㄧ敓鎴愰噸璇曟椂璁剧疆姝ゆ爣蹇椼€?
   - `NETFS_RREQ_PAUSE`

     鏂囦欢绯荤粺鍙互璁剧疆姝ゆ爣蹇椾互璇锋眰鏆傚仠搴撶殑鍒嗗彂瀛愯姹傚惊鐜€斺€斾絾闇€瑕佹敞鎰忥紝鍥犱负 netfslib 涔熷彲鑳借缃畠銆?
   - `NETFS_RREQ_NONBLOCK`
   - `NETFS_RREQ_BLOCKED`

     netfslib 璁剧疆绗竴涓互鎸囩ず璋冪敤鑰呰缃簡闈為樆濉炴ā寮忥紝鏂囦欢绯荤粺鍙互璁剧疆绗簩涓互鎸囩ず瀹冩湰搴旈樆濉炪€?
   - `NETFS_RREQ_USE_PGPRIV2`

     濡傛灉鏂囦欢绯荤粺鎯宠浣跨敤 PG_private_2 鏉ヨ窡韪煇涓?folio 鏄惁姝ｅ湪鍐欏叆缂撳瓨锛屽垯鍙互璁剧疆姝ゆ爣蹇椼€傝繖宸茶搴熷純锛屽洜涓?PG_private_2 鍗冲皢娑堝け銆?
濡傛灉鏂囦欢绯荤粺闇€瑕佹瘮姝ょ粨鏋勬彁渚涚殑鏇村鐨勭鏈夋暟鎹紝鍒欏簲璇ュ皝瑁呭畠骞舵彁渚涜嚜宸辩殑鍒嗛厤鍣ㄣ€?
### 娴佺粨鏋?

涓€涓姹傜敱涓€涓垨澶氫釜骞惰娴佺粍鎴愶紝姣忎釜娴佸彲鑳介潰鍚戜笉鍚岀殑鐩爣銆?
瀵逛簬璇昏姹傦紝鍙娇鐢ㄦ祦 0銆傚畠鍙互鍖呭惈闈㈠悜涓嶅悓鏉ユ簮鐨勩€佹贩鍚堢殑瀛愯姹傘€傚浜庡啓璇锋眰锛屾祦 0 鐢ㄤ簬鏈嶅姟鍣紝娴?1 鐢ㄤ簬缂撳瓨銆傚浜庣紦鍐插洖鍐欙紝闄ら潪閬囧埌姝ｅ父鐨勮剰 folio锛屽惁鍒欐祦 0 涓嶄細鍚敤锛屾鏃跺皢璋冪敤 ->begin_writeback()锛屾枃浠剁郴缁熷彲浠ュ皢璇ユ祦鏍囪涓哄彲鐢ㄣ€?```

	struct netfs_io_stream {
		unsigned char		stream_nr;
		bool			avail;
		size_t			sreq_max_len;
		unsigned int		sreq_max_segs;
		unsigned int		submit_extendable_to;
		...
	};

```
鏂囦欢绯荤粺鍙互璁块棶/浣跨敤鑻ュ共鎴愬憳锛?
 - `stream_nr`

   璇锋眰鍐呮祦鐨勭紪鍙枫€?
 - `avail`

   濡傛灉娴佸彲鐢ㄥ垯涓?true銆傛枃浠剁郴缁熷簲鍦ㄦ祦闆朵笂銆佸湪 ->begin_writeback() 涓缃畠銆?
 - `sreq_max_len`
 - `sreq_max_segs`

   杩欎簺鐢辨枃浠剁郴缁熸垨缂撳瓨鍦?->prepare_read() 鎴?->prepare_write() 涓负姣忎釜瀛愯姹傝缃紝浠ユ寚绀鸿瀛愯姹傛敮鎸佺殑鏈€澶у瓧鑺傛暟锛屼互鍙婂彲閫夌殑鏈€澶ф鏁帮紙濡傛灉涓嶄负 0锛夈€?
 - `submit_extendable_to`

   鍦ㄧ粰瀹氬彲鐢ㄧ紦鍐插尯鐨勬儏鍐典笅锛屽瓙璇锋眰鍙互鍚戜笂鑸嶅叆瓒呭嚭 EOF 鐨勫ぇ灏忋€傝繖浣垮緱缂撳瓨鑳藉鍒ゆ柇瀹冩槸鍚﹁兘鎵ц璺ㄨ秺 EOF 鏍囪鐨?DIO 璇绘垨鍐欍€?
### 瀛愯姹傜粨鏋?

鍗曚釜 I/O 鍗曞厓鐢卞瓙璇锋眰缁撴瀯绠＄悊銆傝繖浜?```

	struct netfs_io_subrequest {
		struct netfs_io_request *rreq;
		struct iov_iter		io_iter;
		unsigned long long	start;
		size_t			len;
		size_t			transferred;
		unsigned long		flags;
		short			error;
		unsigned short		debug_index;
		unsigned char		stream_nr;
		...
	};

```
姣忎釜瀛愯姹傚簲褰撹闂崟涓€鏉ユ簮锛屼笉杩囧簱浼氬鐞嗕粠涓€绉嶆潵婧愮被鍨嬪洖閫€鍒板彟涓€绉嶆潵婧愮被鍨嬨€傚悇鎴愬憳濡備笅锛?
 - `rreq`

   鎸囧悜璇昏姹傜殑鎸囬拡銆?
 - `io_iter`

   涓€涓?I/O 杩唬鍣紝琛ㄧず瑕佽鍏ユ垨鍐欏嚭鐨勭紦鍐插尯鐗囨銆?
 - `start`
 - `len`

   姝よ璇锋眰鐗囨璧峰浣嶇疆鐨勬枃浠跺亸绉诲拰闀垮害銆?
 - `transferred`

   姝ゅ瓙璇锋眰鍒扮洰鍓嶄负姝㈠凡浼犺緭鐨勬暟鎹噺銆傚簲鍦ㄦ湰娆″垎鍙戝瓙璇锋眰鎵€瀹屾垚鐨勪紶杈撻暱搴︿笂绱姞銆傚鏋滄鍊煎皬浜?`len`锛屽垯瀛愯姹傚彲鑳戒細琚噸鏂板垎鍙戜互缁х画銆?
 - `flags`

   鐢ㄤ簬绠＄悊瀛愯姹傜殑鑻ュ共鏍囧織銆傛枃浠剁郴缁熸垨缂撳瓨瀵瑰叾涓竴浜涙劅鍏磋叮锛?
   - `NETFS_SREQ_MADE_PROGRESS`

     鐢辨枃浠剁郴缁熻缃紝琛ㄧず涓€涓垨澶氫釜瀛楄妭鐨勬暟鎹凡琚鍙栨垨鍐欏叆銆?
   - `NETFS_SREQ_HIT_EOF`

     濡傛灉璇诲懡涓簡鏂囦欢 EOF锛屾枃浠剁郴缁熷簲璁剧疆姝ゆ爣蹇楋紙鍦ㄨ繖绉嶆儏鍐典笅 `transferred` 搴斿仠鍦?EOF 澶勶級銆俷etfslib 鍙兘浼氬皢瀛愯姹傛墿灞曞埌鍖呭惈 EOF 鐨?folio 鐨勫ぇ灏忥紝浠ラ槻绗笁鏂瑰彂鐢熶簡鏀瑰彉锛屾垨鑰?DIO 璇诲彲鑳借姹備簡姣斿彲鐢ㄦ暟鎹洿澶氱殑鏁版嵁銆傚簱灏嗘竻闄や换浣曞浣欑殑 pagecache銆?
   - `NETFS_SREQ_CLEAR_TAIL`

     鏂囦欢绯荤粺鍙互璁剧疆姝ゆ爣蹇楋紝浠ユ寚绀轰粠 transferred 鍒?len 鐨勭墖娈靛墿浣欓儴鍒嗗簲琚竻闆躲€傚鏋滆缃簡 HIT_EOF锛岃鍕胯缃€?
   - `NETFS_SREQ_NEED_RETRY`

     鏂囦欢绯荤粺鍙互璁剧疆姝ゆ爣蹇楋紝浠ュ憡璇?netfslib 閲嶈瘯璇ュ瓙璇锋眰銆?
   - `NETFS_SREQ_BOUNDARY`

     鏂囦欢绯荤粺鍙互鍦ㄥ瓙璇锋眰涓婅缃鏍囧織锛屼互鎸囩ず瀹冨湪鏂囦欢绯荤粺缁撴瀯鐨勮竟鐣屽缁撴潫锛堜緥濡傚湪涓€涓?Ceph 瀵硅薄鐨勬湯灏撅級銆傚畠鍛婅瘔 netfslib 涓嶈璺ㄥ畠閲嶆柊骞抽摵瀛愯姹傘€?
 - `error`

   渚涙枃浠剁郴缁熷瓨鍌ㄥ瓙璇锋眰鐨勭粨鏋溿€傛垚鍔熸椂璁句负 0锛屽惁鍒欒涓鸿礋鐨勯敊璇爜銆?
 - `debug_index`
 - `stream_nr`

   涓烘鐗囨鍒嗛厤鐨勩€佸彲鍦?trace 琛屼腑鏄剧ず浠ヤ緵鍙傝€冪殑缂栧彿锛屼互鍙婂畠鎵€灞炵殑璇锋眰娴佺殑缂栧彿銆?
濡傛湁蹇呰锛屾枃浠剁郴缁熷彲浠ュ瀹冩鍦ㄤ娇鐢ㄧ殑瀛愯姹傝幏鍙栧拰閲婃斁棰濆鐨勫紩鐢?```

	void netfs_get_subrequest(struct netfs_io_subrequest *subreq,
				  enum netfs_sreq_ref_trace what);
	void netfs_put_subrequest(struct netfs_io_subrequest *subreq,
				  enum netfs_sreq_ref_trace what);

```
浣跨敤 netfs trace 鐮佹潵鎸囩ず鍘熷洜銆備絾蹇呴』灏忓績锛屽洜涓轰竴鏃﹀瓙璇锋眰鐨勬帶鍒舵潈杩斿洖缁?netfslib锛屽悓涓€涓瓙璇锋眰鍙兘浼氳閲嶆柊鍒嗗彂/閲嶈瘯銆?
### 鏂囦欢绯荤粺鏂规硶


鏂囦欢绯荤粺鍦?`netfs_inode` 涓缃竴涓搷浣滆〃渚?netfslib
```

	struct netfs_request_ops {
		mempool_t *request_pool;
		mempool_t *subrequest_pool;
		int (*init_request)(struct netfs_io_request *rreq, struct file *file);
		void (*free_request)(struct netfs_io_request *rreq);
		void (*free_subrequest)(struct netfs_io_subrequest *rreq);
		void (*expand_readahead)(struct netfs_io_request *rreq);
		int (*prepare_read)(struct netfs_io_subrequest *subreq);
		void (*issue_read)(struct netfs_io_subrequest *subreq);
		void (*done)(struct netfs_io_request *rreq);
		void (*update_i_size)(struct inode *inode, loff_t i_size);
		void (*post_modify)(struct inode *inode);
		void (*begin_writeback)(struct netfs_io_request *wreq);
		void (*prepare_write)(struct netfs_io_subrequest *subreq);
		void (*issue_write)(struct netfs_io_subrequest *subreq);
		void (*retry_request)(struct netfs_io_request *wreq,
				      struct netfs_io_stream *stream);
		void (*invalidate_cache)(struct netfs_io_request *wreq);
	};

```
璇ヨ〃浠ヤ竴瀵瑰彲閫夌殑鍐呭瓨姹犳寚閽堝紑澶达紝璇锋眰鍜屽瓙璇锋眰鍙粠涓垎閰嶃€傚鏋滄湭鎻愪緵锛宯etfslib 鏈夐粯璁ょ殑姹犳潵鏇夸唬浣跨敤銆傚鏋滄枃浠剁郴缁熷皢鑷繁鐨勬洿澶х粨鏋勪綋灏佽鍦?netfs 缁撴瀯浣撲箣澶栵紝鍒欓渶瑕佷娇鐢ㄨ嚜宸辩殑姹犮€俷etfslib 灏嗙洿鎺ヤ粠姹犱腑鍒嗛厤銆?
琛ㄤ腑瀹氫箟鐨勬柟娉曟湁锛?
 - `init_request()`
 - `free_request()`
 - `free_subrequest()`

   [鍙€塢 鏂囦欢绯荤粺鍙互瀹炵幇杩欎簺鏂规硶鏉ュ垵濮嬪寲鎴栨竻鐞嗗叾闄勫姞鍒拌姹傛垨瀛愯姹備笂鐨勪换浣曡祫婧愩€?
 - `expand_readahead()`

   [鍙€塢 璋冪敤姝ゆ柟娉曚互鍏佽鏂囦欢绯荤粺鎵╁睍棰勮璇锋眰鐨勫ぇ灏忋€傛枃浠剁郴缁熷彲浠ュ湪涓や釜鏂瑰悜涓婃墿灞曡姹傦紝浣嗗繀椤讳繚鐣欏垵濮嬪尯鍩燂紝鍥犱负瀹冨彲鑳戒唬琛ㄥ凡缁忓畬鎴愮殑鍒嗛厤銆傚鏋滃惎鐢ㄤ簡鏈湴缂撳瓨锛屽垯瀹冪巼鍏堟墿灞曡姹傘€?
   鎵╁睍閫氳繃淇敼璇锋眰缁撴瀯涓殑 ->start 鍜?->len 鏉ヤ紶杈俱€傛敞鎰忥紝濡傛灉杩涜浜嗕换浣曚慨鏀癸紝->len 鐨勫鍔犻噺鑷冲皯搴斾笌 ->start 鐨勫噺灏戦噺涓€鏍峰銆?
 - `prepare_read()`

   [鍙€塢 璋冪敤姝ゆ柟娉曚互鍏佽鏂囦欢绯荤粺闄愬埗瀛愯姹傜殑澶у皬銆傚畠涔熷彲浠ラ檺鍒惰凯浠ｅ櫒涓崟鐙尯鍩熺殑鏁伴噺锛?```

	rreq->io_streams[0].sreq_max_len
	rreq->io_streams[0].sreq_max_segs

   鏂囦欢绯荤粺鍙互鍒╃敤瀹冿紝渚嬪锛屽皢涓€涓繀椤昏法澶氫釜鏈嶅姟鍣ㄦ媶鍒嗙殑璇锋眰鍒囩墖锛屾垨鑰呭皢澶氫釜璇绘搷浣滃悓鏃舵淳鍙戙€?
   鎴愬姛鏃惰繑鍥?0锛屽惁鍒欒繑鍥為敊璇爜銆?
 * ``issue_read()``

   [蹇呴渶] netfslib 璋冪敤姝ゅ嚱鏁板皢瀛愯姹傚垎娲惧埌鏈嶅姟鍣ㄨ繘琛岃鍙栥€傚湪瀛愯姹備腑锛?>start銆?>len 鍜?->transferred 鎸囩ず搴斾粠鏈嶅姟鍣ㄨ鍙栧摢浜涙暟鎹紝->io_iter 鎸囩ず瑕佷娇鐢ㄧ殑缂撳啿鍖恒€?
   娌℃湁杩斿洖鍊硷紱搴旇皟鐢?``netfs_read_subreq_terminated()`` 鍑芥暟鏉ユ寚绀哄瓙璇锋眰宸插畬鎴愶紙鏃犺鍝缁撴灉锛夈€?>error銆?>transferred 鍜?->flags 搴斿湪瀹屾垚鍓嶆洿鏂般€傜粓姝㈠彲浠ユ槸寮傛鐨勩€?
   娉ㄦ剰锛氭枃浠剁郴缁熶笉寰楀鐞嗚缃?folio 涓?uptodate銆佽В閿佸畠浠垨涓㈠純瀹冧滑鐨勫紩鐢ㄢ€斺€斿簱浼氬鐞嗚繖浜涳紝鍥犱负瀹冨彲鑳介渶瑕佸皢澶氫釜瀛愯姹傜殑缁撴灉鎷兼帴璧锋潵锛岃繖浜涘瓙璇锋眰浠ュ悇绉嶆柟寮忛噸鍙犱簬涓€缁?folio銆?
 * ``done()``

   [鍙€塢 鍦ㄨ璇锋眰涓殑 folio 鍏ㄩ儴瑙ｉ攣锛堝苟鍦ㄩ€傜敤鏃舵爣璁颁负 uptodate锛変箣鍚庤皟鐢ㄣ€?
 * ``update_i_size()``

   [鍙€塢 鍦ㄥ啓璺緞鐨勫悇涓椂鏈虹敱 netfslib 璋冪敤锛屼互璇锋眰鏂囦欢绯荤粺鏇存柊鍏跺鏂囦欢澶у皬鐨勮鐭ャ€傚鏋滄湭鎻愪緵锛宯etfslib 灏嗚缃?i_size 鍜?i_blocks 骞舵洿鏂版湰鍦扮紦瀛?cookie銆?
 * ``post_modify()``

   [鍙€塢 鍦?netfslib 鍐欏叆 pagecache 鏃讹紝鎴栧綋瀹冨厑璁镐竴涓?mmap 鏄犲皠鐨勯〉琚爣璁颁负鍙啓鏃惰皟鐢ㄣ€?
 * ``begin_writeback()``

   [鍙€塢 netfslib 鍦ㄥ鐞嗗洖鍐欒姹傛椂锛屽鏋滃彂鐜颁竴涓笉浠呬粎鏄爣璁颁负 NETFS_FOLIO_COPY_TO_CACHE 鐨勮剰椤碉紝琛ㄦ槑瀹冨繀椤诲啓鍏ユ湇鍔″櫒锛屽垯璋冪敤姝ゅ嚱鏁般€傝繖浣垮緱鏂囦欢绯荤粺鍙湁鍦ㄧ煡閬撹嚜宸卞皢瑕佹墽琛屼竴娆″啓鎿嶄綔鏃讹紝鎵嶅缓绔嬪洖鍐欒祫婧愩€?
 * ``prepare_write()``

   [鍙€塢 璋冪敤姝ゆ柟娉曚互鍏佽鏂囦欢绯荤粺闄愬埗瀛愯姹傜殑澶у皬銆傚畠涔熷彲浠ラ檺鍒惰凯浠ｅ櫒涓崟鐙尯鍩熺殑鏁伴噺锛屼緥濡?RDMA 鎵€瑕佹眰鐨勩€傛淇℃伅搴旇缃湪瀛愯姹傛墍灞炴祦涓?:

	rreq->io_streams[subreq->stream_nr].sreq_max_len
	rreq->io_streams[subreq->stream_nr].sreq_max_segs

   鏂囦欢绯荤粺鍙互鍒╃敤瀹冿紝渚嬪锛屽皢涓€涓繀椤昏法澶氫釜鏈嶅姟鍣ㄦ媶鍒嗙殑璇锋眰鍒囩墖锛屾垨鑰呭皢澶氫釜鍐欐搷浣滃悓鏃舵淳鍙戙€?
   涓嶅厑璁歌繑鍥為敊璇€傜浉鍙嶏紝鍦ㄥけ璐ョ殑鎯呭喌涓嬶紝蹇呴』璋冪敤 ``netfs_prepare_write_failed()``銆?
 * ``issue_write()``

   [蹇呴渶] 鐢ㄤ簬灏嗗瓙璇锋眰鍒嗘淳鍒版湇鍔″櫒杩涜鍐欏叆銆傚湪瀛愯姹備腑锛?>start銆?>len 鍜?->transferred 鎸囩ず搴斿啓鍏ユ湇鍔″櫒鐨勬暟鎹紝->io_iter 鎸囩ず瑕佷娇鐢ㄧ殑缂撳啿鍖恒€?
   娌℃湁杩斿洖鍊硷紱搴旇皟鐢?``netfs_write_subreq_terminated()`` 鍑芥暟鏉ユ寚绀哄瓙璇锋眰宸插畬鎴愶紙鏃犺鍝缁撴灉锛夈€?>error銆?>transferred 鍜?->flags 搴斿湪瀹屾垚鍓嶆洿鏂般€傜粓姝㈠彲浠ユ槸寮傛鐨勩€?
   娉ㄦ剰锛氭枃浠剁郴缁熶笉寰楀鐞嗘竻闄ゆ搷浣滀腑娑夊強鐨?folio 涓婄殑鑴忔垨鍥炲啓鏍囪锛屼篃涓嶅簲瀵瑰畠浠幏鍙栧紩鐢ㄦ垨鍥哄畾锛岃€屽簲灏嗕繚鐣欎氦缁?netfslib銆?
 * ``retry_request()``

   [鍙€塢 netfslib 鍦ㄩ噸璇曞懆鏈熷紑濮嬫椂璋冪敤姝ゅ嚱鏁般€傝繖浣垮緱鏂囦欢绯荤粺鑳藉妫€鏌ヨ姹傜殑鐘舵€併€佹寚瀹氭祦涓殑瀛愯姹備互鍙婂叾鑷韩鏁版嵁鐨勭姸鎬侊紝骞惰繘琛岃皟鏁存垨閲嶆柊鍗忓晢璧勬簮銆?
 * ``invalidate_cache()``

   [鍙€塢 褰撳啓鍏ユ湰鍦扮紦瀛樺け璐ユ椂锛宯etfslib 璋冪敤姝ゅ嚱鏁颁互浣垮瓨鍌ㄥ湪鏈湴缂撳瓨涓殑鏁版嵁澶辨晥锛屾彁渚?netfs 鏃犳硶鎻愪緵鐨勬洿鏂颁竴鑷存€ф暟鎹€?
```
### 缁堟瀛愯姹?

褰撳瓙璇锋眰瀹屾垚鏃讹紝缂撳瓨鎴栧瓙璇锋眰鍙互璋冪敤鑻ュ共鍑芥暟鏉ラ€氱煡 netfslib 鐘舵€佸彉鍖栥€傛彁渚涗竴涓嚱鏁板湪鍑嗗闃舵鍚屾鍦扮粓姝竴涓啓瀛愯姹傦細

 - `void netfs_prepare_write_failed(struct netfs_io_subrequest *subreq);`

   鎸囩ず ->prepare_write() 璋冪敤澶辫触銆俙error` 瀛楁搴斿凡鏇存柊銆?
娉ㄦ剰锛?>prepare_read() 鍙互杩斿洖閿欒锛屽洜涓鸿鍙互绠€鍗曞湴涓銆傚鐞嗗洖鍐欏け璐ュ垯鏇存鎵嬨€?
鍏朵粬鍑芥暟鐢ㄤ簬宸茬粡鍒嗗彂鍒版墽琛岄樁娈电殑瀛愯姹傦細

 - `void netfs_read_subreq_terminated(struct netfs_io_subrequest *subreq);`

   鍛婅瘔 netfslib 涓€涓瀛愯姹傚凡缁堟銆俙error`銆乣flags` 鍜?`transferred` 瀛楁搴斿凡鏇存柊銆?
 - `void netfs_write_subreq_terminated(void *_op, ssize_t transferred_or_error);`

   鍛婅瘔 netfslib 涓€涓啓瀛愯姹傚凡缁堟銆傚彲浠ヤ紶鍏ュ凡澶勭悊鐨勬暟鎹噺鎴栬礋鐨勯敊璇爜銆傝繖鍙互鐢ㄤ綔 kiocb 瀹屾垚鍑芥暟銆?
 - `void netfs_read_subreq_progress(struct netfs_io_subrequest *subreq);`

   鎻愪緵姝ゅ嚱鏁颁互鍙€夊湴鍚?netfslib 鏇存柊璇荤殑澧為噺杩涘害锛屽厑璁告煇浜?folio 鎻愬墠瑙ｉ攣锛屼絾瀹為檯涓婂苟涓嶇粓姝㈠瓙璇锋眰銆俙transferred` 瀛楁搴斿凡鏇存柊銆?
### 鏈湴缂撳瓨 API


netfslib 鎻愪緵浜嗕竴涓嫭绔嬬殑 API 渚涙湰鍦扮紦瀛樺疄鐜帮紝灏界瀹冩彁渚涗簡涓€浜涗笌鏂囦欢绯荤粺璇锋眰 API 鐩稿綋绫讳技鐨勮繃绋嬨€?
棣栧厛锛宯etfs_io_request 瀵硅薄鍖呭惈涓€涓緵缂撳瓨鎸傝浇鍏?```

	struct netfs_cache_resources {
		const struct netfs_cache_ops	*ops;
		void				*cache_priv;
		void				*cache_priv2;
		unsigned int			debug_id;
		unsigned int			inval_counter;
	};

```
杩欏寘鍚竴涓搷浣滆〃鎸囬拡鍜屼袱涓鏈夋寚閽堬紝鍔犱笂鐢ㄤ簬杩借釜鐨?fscache cookie 鐨勮皟璇?ID锛屼互鍙婁竴涓敱 `fscache_invalidate()` 璋冪敤閫掑鐨勫け鏁堣鏁板櫒锛屽厑璁哥紦瀛樺瓙璇锋眰鍦ㄥ畬鎴愬悗琚け鏁堛€?```

	struct netfs_cache_ops {
		void (*end_operation)(struct netfs_cache_resources *cres);
		void (*expand_readahead)(struct netfs_cache_resources *cres,
					 loff_t *_start, size_t *_len, loff_t i_size);
		enum netfs_io_source (*prepare_read)(struct netfs_io_subrequest *subreq,
						     loff_t i_size);
		int (*read)(struct netfs_cache_resources *cres,
			    loff_t start_pos,
			    struct iov_iter *iter,
			    bool seek_data,
			    netfs_io_terminated_t term_func,
			    void *term_func_priv);
		void (*prepare_write_subreq)(struct netfs_io_subrequest *subreq);
		void (*issue_write)(struct netfs_io_subrequest *subreq);
	};

```
```

	typedef void (*netfs_io_terminated_t)(void *priv,
					      ssize_t transferred_or_error,
					      bool was_async);

```
琛ㄤ腑瀹氫箟鐨勬柟娉曟湁锛?
 - `end_operation()`

   [蹇呴渶] 鍦ㄨ鍙栬姹傜粨鏉熸椂璋冪敤锛屼互娓呯悊璧勬簮銆?
 - `expand_readahead()`

   [鍙€塢 鍦ㄩ璇绘搷浣滃紑濮嬫椂璋冪敤锛屼互鍏佽缂撳瓨鍚戜换涓€鏂瑰悜鎵╁睍璇锋眰銆傝繖浣垮緱缂撳瓨鑳藉閽堝鍏剁矑搴﹀璇锋眰杩涜閫傚綋鐨勫ぇ灏忚皟鏁淬€?
 - `prepare_read()`

   [蹇呴渶] 璋冪敤浠ラ厤缃姹傜殑涓嬩竴涓墖娈点€傚瓙璇锋眰涓殑 ->start 鍜?->len 鎸囩ず涓嬩竴涓墖娈电殑浣嶇疆鍜屽ぇ灏忥紱缂撳瓨鍙互灏嗛暱搴﹀噺灏忎互鍖归厤鍏剁矑搴﹁姹傘€?
   璇ュ嚱鏁板湪鍏跺弬鏁颁腑浼犲叆鎸囧悜璧峰浣嶇疆鍜岄暱搴︾殑鎸囬拡锛屽姞涓婃枃浠跺ぇ灏忎緵鍙傝€冿紝骞堕€傚綋鍦拌皟鏁磋捣濮嬩綅缃拰闀垮害銆傚畠搴旇繑鍥炰互涓嬩箣涓€锛?
   - `NETFS_FILL_WITH_ZEROES`
   - `NETFS_DOWNLOAD_FROM_SERVER`
   - `NETFS_READ_FROM_CACHE`
   - `NETFS_INVALID_READ`

   浠ユ寚绀鸿鐗囨搴斾粎琚竻闆讹紝杩樻槸搴斾粠鏈嶅姟鍣ㄤ笅杞芥垨浠庣紦瀛樿鍙栤€斺€旀垨鑰呮槸鍚﹀簲鍦ㄥ綋鍓嶄綅缃斁寮冨垏鐗囥€?
 - `read()`

   [蹇呴渶] 璋冪敤浠ヤ粠缂撳瓨璇诲彇銆傜粰瀹氳捣濮嬫枃浠跺亸绉婚噺浠ュ強涓€涓璇诲叆鐨勮凯浠ｅ櫒锛堝畠涔熺粰鍑洪暱搴︼級銆傚彲浠ョ粰瀹氫竴涓彁绀猴紝璇锋眰浠庨偅涓捣濮嬩綅缃悜鍓嶆煡鎵炬暟鎹€?
   杩樻彁渚涗簡涓€涓寚鍚戠粓姝㈠鐞嗗嚱鏁扮殑鎸囬拡浠ュ強瑕佷紶閫掔粰璇ュ嚱鏁扮殑绉佹湁鏁版嵁銆傚簲浠ヤ紶杈撶殑瀛楄妭鏁版垨閿欒鐮侊紝鍔犱笂涓€涓寚绀虹粓姝㈡槸鍚﹁偗瀹氬彂鐢熷湪璋冪敤鑰呬笂涓嬫枃涓殑鏍囧織鏉ヨ皟鐢ㄨ缁堟鍑芥暟銆?
 - `prepare_write_subreq()`

   [蹇呴渶] 璋冪敤浠ュ厑璁哥紦瀛橀檺鍒跺瓙璇锋眰鐨勫ぇ灏忋€傚畠涔熷彲浠ラ檺鍒惰凯浠ｅ櫒涓崟鐙尯鍩熺殑鏁伴噺锛屼緥濡?DIO/DMA 鎵€瑕佹眰鐨勩€傛淇℃伅搴旇缃湪瀛愯姹傛墍灞炵殑娴佷笂
```

	rreq->io_streams[subreq->stream_nr].sreq_max_len
	rreq->io_streams[subreq->stream_nr].sreq_max_segs

   鏂囦欢绯荤粺鍙互鍒╃敤瀹冿紝渚嬪锛屽皢涓€涓繀椤昏法澶氫釜鏈嶅姟鍣ㄦ媶鍒嗙殑璇锋眰鍒囩墖锛屾垨鑰呭皢澶氫釜鍐欐搷浣滃悓鏃舵淳鍙戙€?
   涓嶅厑璁歌繑鍥為敊璇€傚湪澶辫触鐨勬儏鍐典笅锛屽繀椤昏皟鐢?``netfs_prepare_write_failed()``銆?
 * ``issue_write()``

   [蹇呴渶] 鐢ㄤ簬灏嗗瓙璇锋眰鍒嗘淳鍒扮紦瀛樿繘琛屽啓鍏ャ€傚湪瀛愯姹備腑锛?>start銆?>len 鍜?->transferred 鎸囩ず搴斿啓鍏ョ紦瀛樼殑鏁版嵁锛?>io_iter 鎸囩ず瑕佷娇鐢ㄧ殑缂撳啿鍖恒€?
   娌℃湁杩斿洖鍊硷紱搴旇皟鐢?``netfs_write_subreq_terminated()`` 鍑芥暟鏉ユ寚绀哄瓙璇锋眰宸插畬鎴愶紙鏃犺鍝缁撴灉锛夈€?>error銆?>transferred 鍜?->flags 搴斿湪瀹屾垚鍓嶆洿鏂般€傜粓姝㈠彲浠ユ槸寮傛鐨勩€?

```
## API 鍑芥暟鍙傝€?