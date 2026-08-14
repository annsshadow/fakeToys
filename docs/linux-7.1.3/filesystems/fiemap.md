
## Fiemap Ioctl


fiemap ioctl 鏄竴绉嶄緵鐢ㄦ埛绌洪棿楂樻晥鑾峰彇鏂囦欢鍖洪棿锛坋xtent锛夋槧灏勭殑鏂规硶銆備笌閫愬潡鏄犲皠锛堝 bmap锛変笉鍚岋紝fiemap 杩斿洖鐨勬槸鍖洪棿鍒楄〃銆?
### 璇锋眰鍩虹


fiemap 璇锋眰缂栫爜鍦?struct fiemap 涓細

   :identifiers: fiemap

fm_start 鍜?fm_length 鎸囧畾浜嗚繘绋嬪笇鏈涜幏鍙栨槧灏勭殑鏂囦欢鍐呴€昏緫鑼冨洿銆傝繑鍥炵殑鍖洪棿涓庣鐩樹笂鐨勯暅鍍忎竴鑷粹€斺€斾篃灏辨槸璇达紝绗竴涓繑鍥炲尯闂寸殑閫昏緫鍋忕Щ鍙兘鏃╀簬 fm_start锛屾渶鍚庝竴涓繑鍥炲尯闂存墍瑕嗙洊鐨勮寖鍥村彲鑳芥櫄浜?fm_length銆傛墍鏈夊亸绉诲拰闀垮害閮戒互瀛楄妭涓哄崟浣嶃€?
鍙互閫氳繃璁剧疆 fm_flags 涓殑鏌愪簺鏍囧織鏉ヤ慨鏀规煡鎵炬槧灏勭殑鏂瑰紡銆傚鏋滃唴鏍镐笉鐞嗚В鏌愪簺鐗瑰畾鏍囧織锛屽畠浼氳繑鍥?EBADR锛屽苟涓?fm_flags 鐨勫唴瀹逛細鍖呭惈瀵艰嚧閿欒鐨勯偅缁勬爣蹇椼€傚鏋滃唴鏍镐笌浼犲叆鐨勬墍鏈夋爣蹇楅兘鍏煎锛屽垯 fm_flags 鐨勫唴瀹逛繚鎸佷笉鍙樸€傜敱鐢ㄦ埛绌洪棿鍐冲畾鎷掔粷鏌愪釜鐗瑰畾鏍囧織瀵瑰叾鎿嶄綔鏄惁鏄嚧鍛界殑銆傝鏂规鏃ㄥ湪璁?fiemap 鎺ュ彛鍦ㄦ湭鏉ヨ兘澶熸墿灞曪紝鍚屾椂鍙堜笉浼氫笌鏃ц蒋浠跺け鍘诲吋瀹规€с€?
fm_extent_count 鎸囧畾浜?fm_extents[] 鏁扮粍涓彲鐢ㄤ簬杩斿洖鍖洪棿鐨勫厓绱犱釜鏁般€傚鏋?fm_extent_count 涓洪浂锛屽垯蹇界暐 fm_extents[] 鏁扮粍锛堜笉浼氳繑鍥炰换浣曞尯闂达級锛屽苟涓?fm_mapped_extents 璁℃暟灏嗕繚瀛?fm_extents[] 涓负淇濆瓨鏂囦欢褰撳墠鏄犲皠鎵€闇€鐨勫尯闂存暟閲忋€傝娉ㄦ剰锛屾病鏈変换浣曟満鍒惰兘闃绘鏂囦欢鍦ㄤ袱娆?FIEMAP 璋冪敤涔嬮棿鍙戠敓鍙樺寲銆?
鍙互璁剧疆鍒?fm_flags 涓殑鏍囧織濡備笅锛?
FIEMAP_FLAG_SYNC
  濡傛灉璁剧疆浜嗚鏍囧織锛屽唴鏍镐細鍦ㄦ槧灏勫尯闂翠箣鍓嶅悓姝ヨ鏂囦欢銆?
FIEMAP_FLAG_XATTR
  濡傛灉璁剧疆浜嗚鏍囧織锛岃繑鍥炵殑鍖洪棿灏嗘弿杩?inode 鐨勬墿灞曞睘鎬ф煡鎵炬爲锛岃€屼笉鏄叾鏁版嵁鏍戙€?
FIEMAP_FLAG_CACHE
  璇ユ爣蹇楄姹傚鍖洪棿杩涜缂撳瓨銆?
### 鍖洪棿鏄犲皠


鍖洪棿淇℃伅鍦ㄥ祵鍏ョ殑 fm_extents 鏁扮粍涓繑鍥烇紝璇ユ暟缁勫繀椤荤敱鐢ㄦ埛绌洪棿涓?fiemap 缁撴瀯浣撲竴璧峰垎閰嶃€俧iemap_extents[] 鏁扮粍涓殑鍏冪礌涓暟搴旈€氳繃 fm_extent_count 浼犲叆銆傚唴鏍告槧灏勭殑鍖洪棿鏁伴噺灏嗛€氳繃 fm_mapped_extents 杩斿洖銆傚鏋滃垎閰嶇殑 fiemap_extents 鏁伴噺灏戜簬鏄犲皠鎵€璇锋眰鑼冨洿鎵€闇€鐨勬暟閲忥紝鍒欎細杩斿洖 fm_extent[] 鏁扮粍涓兘澶熸槧灏勭殑鏈€澶у尯闂存暟閲忥紝涓?fm_mapped_extents 浼氱瓑浜?fm_extent_count銆傚湪杩欑鎯呭喌涓嬶紝鏁扮粍涓殑鏈€鍚庝竴涓尯闂翠笉浼氬畬鎴愭墍璇锋眰鐨勮寖鍥达紝涔熶笉浼氳缃?FIEMAP_EXTENT_LAST 鏍囧織锛堣涓嬩竴鑺傚叧浜庡尯闂存爣蹇楃殑鍐呭锛夈€?
姣忎釜鍖洪棿鐢?fm_extents 涓繑鍥炵殑涓€涓?fiemap_extent 缁撴瀯浣撴弿杩帮細

    :identifiers: fiemap_extent

鎵€鏈夊亸绉诲拰闀垮害閮戒互瀛楄妭涓哄崟浣嶏紝骞朵笌纾佺洏涓婄殑淇濇寔涓€鑷淬€傚尯闂寸殑閫昏緫鍋忕Щ鏃╀簬璇锋眰鎴栧叾閫昏緫闀垮害瓒呭嚭璇锋眰閮芥槸鏈夋晥鐨勩€傞櫎闈炶繑鍥炰簡 FIEMAP_EXTENT_NOT_ALIGNED锛屽惁鍒?fe_logical銆乫e_physical 鍜?fe_length 閮戒細涓庢枃浠剁郴缁熺殑鍧楀ぇ灏忓榻愩€傞櫎浜嗚鏍囪涓?FIEMAP_EXTENT_MERGED 鐨勫尯闂村锛岀浉閭荤殑鍖洪棿涓嶄細琚悎骞躲€?
fe_flags 瀛楁鍖呭惈鎻忚堪鎵€杩斿洖鍖洪棿鐨勬爣蹇椼€備竴涓壒娈婃爣蹇?FIEMAP_EXTENT_LAST 鎬绘槸璁剧疆鍦ㄦ枃浠朵腑鏈€鍚庝竴涓尯闂翠笂锛屼互渚垮彂璧?fiemap 璋冪敤鐨勮繘绋嬭兘澶熷垽鏂綍鏃舵病鏈夋洿澶氬尯闂村彲鐢紝鑰屾棤闇€鍐嶆璋冪敤璇?ioctl銆?
鏌愪簺鏍囧織鏄湁鎰忓惈绯婄殑锛屽苟涓斿彧瑕佸瓨鍦ㄥ叾浠栨洿鍏蜂綋鐨勬爣蹇楀氨浼氬缁堣璁剧疆銆傝繖鏍凤紝瀵绘壘涓€鑸睘鎬х殑绋嬪簭灏变笉蹇呯煡閬撴墍鏈夋殫绀鸿灞炴€х殑鐜版湁鍙婃湭鏉ユ爣蹇椼€?
渚嬪锛屽鏋滆缃簡 FIEMAP_EXTENT_DATA_INLINE 鎴?FIEMAP_EXTENT_DATA_TAIL锛屼篃浼氳缃?FIEMAP_EXTENT_NOT_ALIGNED銆傚鎵惧唴鑱旀垨灏鹃儴鎵撳寘鏁版嵁鐨勭▼搴忓彲浠ヤ緷鎹鍏蜂綋鏍囧織銆傜劧鑰岋紝浠呬粎鍏冲績涓嶈鍘绘搷浣滄湭瀵归綈鍖洪棿鐨勮蒋浠跺彲浠ュ彧渚濇嵁 FIEMAP_EXTENT_NOT_ALIGNED锛岃€屼笉蹇呮媴蹇冩墍鏈夊綋鍓嶅拰鏈潵鐨勩€佸彲鑳芥殫绀烘湭瀵归綈鏁版嵁鐨勬爣蹇椼€傛敞鎰忓弽涔嬩笉鎴愮珛鈥斺€擣IEMAP_EXTENT_NOT_ALIGNED 鍗曠嫭鍑虹幇鏄湁鏁堢殑銆?
FIEMAP_EXTENT_LAST
  杩欓€氬父鏄枃浠朵腑鐨勬渶鍚庝竴涓尯闂淬€傝秺杩囪鍖洪棿鐨勬槧灏勫皾璇曞彲鑳借繑鍥炵┖銆傛煇浜涘疄鐜拌缃鏍囧織浠ヨ〃绀烘鍖洪棿鏄敤鎴凤紙閫氳繃 fiemap->fm_length锛夋煡璇㈣寖鍥村唴鐨勬渶鍚庝竴涓尯闂淬€?
FIEMAP_EXTENT_UNKNOWN
  璇ュ尯闂寸殑浣嶇疆鐩墠鏈煡銆傝繖鍙兘琛ㄧず鏁版嵁瀛樺偍鍦ㄤ笉鍙闂殑鍗蜂笂锛屾垨鑰呭皻鏈负璇ユ枃浠跺垎閰嶅瓨鍌ㄣ€?
FIEMAP_EXTENT_DELALLOC
  杩欎篃浼氳缃?FIEMAP_EXTENT_UNKNOWN銆?
  寤惰繜鍒嗛厤鈥斺€旇櫧鐒惰鍖洪棿宸叉湁鏁版嵁锛屼絾鍏剁墿鐞嗕綅缃皻鏈垎閰嶃€?
FIEMAP_EXTENT_ENCODED
  璇ュ尯闂村苟闈炵敱鏅€氱殑鏂囦欢绯荤粺鍧楃粍鎴愶紝鑰屾槸缁忚繃缂栫爜锛堜緥濡傚姞瀵嗘垨鍘嬬缉锛夈€傞€氳繃璇ュ潡璁惧杩涜 I/O 鏉ヨ鍙栨鍖洪棿涓殑鏁版嵁灏嗕骇鐢熸湭瀹氫箟鐨勭粨鏋溿€?
娉ㄦ剰锛屽湪鏂囦欢绯荤粺鐨勫崗鍔╀笅锛岃瘯鍥鹃€氳繃鍐欏叆鎵€鎸囩ず浣嶇疆鏉ュ氨鍦版洿鏂版暟鎹紝鎴栧湪鏂囦欢绯荤粺宸叉寕杞芥椂閫氳繃 FIEMAP 鎺ュ彛杩斿洖鐨勪俊鎭潵璁块棶鏁版嵁锛岃繖**鎬绘槸**鏈畾涔夌殑銆傛崲瑷€涔嬶紝鐢ㄦ埛搴旂敤绋嬪簭鍙兘鍦ㄦ枃浠剁郴缁熸湭鎸傝浇鏃堕€氳繃鍧楄澶囪繘琛?I/O 璇诲彇鍖洪棿鏁版嵁锛屽苟涓斾粎褰?FIEMAP_EXTENT_ENCODED 鏍囧織鏈缃椂鎵嶅彲浠ワ紱鍦ㄤ换浣曞叾浠栨儏鍐典笅锛岀敤鎴峰簲鐢ㄧ▼搴忛兘涓嶅緱璇曞浘閫氳繃鍧楄澶囪鍙栨垨鍐欏叆鏂囦欢绯荤粺銆?
FIEMAP_EXTENT_DATA_ENCRYPTED
  杩欎篃浼氳缃?FIEMAP_EXTENT_ENCODED
  璇ュ尯闂翠腑鐨勬暟鎹凡琚枃浠剁郴缁熷姞瀵嗐€?
FIEMAP_EXTENT_NOT_ALIGNED
  鍖洪棿鍋忕Щ鍜岄暱搴︿笉淇濊瘉鎸夊潡瀵归綈銆?
FIEMAP_EXTENT_DATA_INLINE
  杩欎篃浼氳缃?FIEMAP_EXTENT_NOT_ALIGNED
  鏁版嵁浣嶄簬涓€涓厓鏁版嵁鍧椾腑銆?
FIEMAP_EXTENT_DATA_TAIL
  杩欎篃浼氳缃?FIEMAP_EXTENT_NOT_ALIGNED
  鏁版嵁琚墦鍖呰繘涓€涓笌鍏朵粬鏂囦欢鏁版嵁鍏辩敤鐨勫潡涓€?
FIEMAP_EXTENT_UNWRITTEN
  鏈啓鍏ュ尯闂粹€斺€旇鍖洪棿宸插垎閰嶄絾鍏舵暟鎹皻鏈垵濮嬪寲銆傝繖琛ㄧず濡傛灉閫氳繃鏂囦欢绯荤粺璇诲彇锛岃鍖洪棿鐨勬暟鎹皢鍏ㄤ负闆讹紱濡傛灉鐩存帴浠庤澶囪鍙栵紝鍏跺唴瀹瑰垯鏄湭瀹氫箟鐨勩€?
FIEMAP_EXTENT_MERGED
  褰撴枃浠朵笉鏀寔鍖洪棿锛堝嵆浣跨敤鍩轰簬鍧楃殑瀵诲潃鏂规锛夋椂浼氳缃€傜敱浜庡皢姣忎釜鍧楃殑鍖洪棿杩斿洖缁欑敤鎴风┖闂存晥鐜囨瀬浣庯紝鍐呮牳浼氬皾璇曞皢澶у鏁扮浉閭诲潡鍚堝苟涓衡€滃尯闂粹€濄€?
FIEMAP_EXTENT_SHARED
  璁剧疆璇ユ爣蹇椾互璇锋眰绌洪棿涓庡叾浠栨枃浠跺叡浜€?
### VFS -> 鏂囦欢绯荤粺瀹炵幇


甯屾湜鏀寔 fiemap 鐨勬枃浠剁郴缁熷繀椤诲湪鍏?inode_operations 缁撴瀯浣撲笂瀹炵幇 ->fiemap 鍥炶皟銆傝 fs ->fiemap 璋冪敤璐熻矗瀹氫箟鍏舵敮鎸佺殑涓€缁?fiemap 鏍囧織锛屽苟璋冪敤涓€涓緟鍔╁嚱鏁帮紝鍏蜂綋瑙?
```
  struct inode_operations {
       ...

       int (*fiemap)(struct inode *, struct fiemap_extent_info *, u64 start,
                     u64 len);
```

->fiemap 浼氫紶鍏ユ弿杩?fiemap 璇锋眰鐨?struct fiemap_extent_info锛?
    :identifiers: fiemap_extent_info

鏂囦欢绯荤粺鐨勬湰鎰忔槸涓嶉渶瑕佺洿鎺ヨ闂缁撴瀯浣撶殑浠讳綍鎴愬憳銆傛枃浠剁郴缁熷鐞嗙▼搴忓簲璇ュ淇″彿瀹藉锛屽苟鍦ㄦ敹鍒拌嚧鍛戒俊鍙锋椂杩斿洖 EINTR銆?
鏍囧織妫€鏌ュ簲鍦?->fiemap 鍥炶皟寮€濮嬫椂閫氳繃

```
  int fiemap_prep(struct inode *inode, struct fiemap_extent_info *fieinfo,
		  u64 start, u64 *len, u32 supported_flags);
```

瀹屾垚銆俿truct fieinfo 搴旀寜浠?ioctl_fiemap() 鏀跺埌鏃剁殑鏍峰瓙浼犲叆銆傛枃浠剁郴缁熺悊瑙ｇ殑 fiemap 鏍囧織闆嗗悎搴旈€氳繃 fs_flags 浼犲叆銆傚鏋?fiemap_prep 鍙戠幇鏃犳晥鐨勭敤鎴锋爣蹇楋紝瀹冧細灏嗛敊璇€兼斁鍏?fieinfo->fi_flags 骞惰繑鍥?-EBADR銆傚鏋滄枃浠剁郴缁熶粠 fiemap_prep() 寰楀埌 -EBADR锛屽畠搴旂珛鍗抽€€鍑猴紝灏嗚閿欒杩斿洖缁?ioctl_fiemap()銆傛澶栵紝鑼冨洿浼氭牴鎹墍鏀寔鐨勬渶澶ф枃浠跺ぇ灏忚繘琛屾牎楠屻€?
瀵逛簬璇锋眰鑼冨洿鍐呯殑姣忎釜鍖洪棿锛屾枃浠剁郴缁熷簲璋冪敤

```
  int fiemap_fill_next_extent(struct fiemap_extent_info *info, u64 logical,
			      u64 phys, u64 len, u32 flags, u32 dev);
```

fiemap_fill_next_extent() 灏嗕娇鐢ㄤ紶鍏ョ殑鍊兼潵濉厖 fm_extents 鏁扮粍涓殑涓嬩竴涓┖闂插尯闂淬€傞€氱敤鐨勫尯闂存爣蹇椾細鏍规嵁鍏蜂綋鏍囧織鑷姩鐢辫皟鐢ㄦ枃浠剁郴缁熻缃紝浠庤€屼笉浼氱牬鍧忕敤鎴风┖闂?API銆?
fiemap_fill_next_extent() 鎴愬姛鏃惰繑鍥?0锛屽綋鐢ㄦ埛鎻愪緵鐨?fm_extents 鏁扮粍宸叉弧鏃惰繑鍥?1銆傚鏋滃湪灏嗗尯闂村鍒跺埌鐢ㄦ埛鍐呭瓨鏃堕亣鍒伴敊璇紝鍒欒繑鍥?-EFAULT銆?