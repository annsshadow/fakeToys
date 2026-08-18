..
        涓轰繚鎸佷綔鑰呯悊鏅虹殑绗ㄦ嫏椋庢牸璇存槑锛?        璇峰敖閲忓湪鍗曠嫭鐨勮涓婂紑濮嬪彞瀛愶紝浠ヤ究鍙ュ瓙鍙樻洿涓嶄細鍦?diff 涓覆鑹层€?        鏍囬瑁呴グ鍦?sphinx.rst 涓湁璇存槑銆?
## 鏀寔鐨勬枃浠舵搷浣?

   :local:

涓嬮潰璁ㄨ iomap 瀹炵幇鐨勯珮灞傛枃浠舵搷浣溿€?
## 缂撳啿 I/O


缂撳啿 I/O 鏄?Linux 涓粯璁ょ殑鏂囦欢 I/O 璺緞銆?鏂囦欢鍐呭琚紦瀛樺湪鍐呭瓨涓紙"pagecache"锛変互鍝嶅簲璇诲拰鍐欍€?鑴忕紦瀛樹細鍦ㄦ煇涓椂鍒诲啓鍥炵鐩橈紝涔熷彲閫氳繃 `fsync` 鍙婂叾鍙樹綋寮哄埗鍐欏洖銆?
iomap 瀹炵幇浜嗘枃浠剁郴缁熷湪浼犵粺 I/O 妯″瀷涓嬪繀椤昏嚜琛屽疄鐜扮殑鍑犱箮鎵€鏈?folio 涓?pagecache 绠＄悊宸ヤ綔銆?杩欐剰鍛崇潃鏂囦欢绯荤粺鏃犻渶浜嗚В鍒嗛厤銆佹槧灏勩€佺鐞?uptodate 涓?dirty 鐘舵€侊紝鎴?pagecache folio 鐨勫洖鍐欑瓑缁嗚妭銆?鍦ㄤ紶缁?I/O 妯″瀷涓嬶紝杩欎簺閮芥槸鐢?buffer head 閾捐〃鏉ヤ綆鏁堢鐞嗙殑锛岃€岄潪 iomap 鎵€浣跨敤鐨?per-folio 浣嶅浘銆?闄ら潪鏂囦欢绯荤粺鏄惧紡閫夋嫨浣跨敤 buffer head锛屽惁鍒欎笉浼氫娇鐢ㄥ畠浠紝杩欎娇寰楃紦鍐?I/O 楂樻晥寰楀锛屼篃璁?pagecache 缁存姢鑰呭紑蹇冨緱澶氥€?
### ``struct address_space_operations``


涓嬪垪 iomap 鍑芥暟鍙洿鎺ヤ粠鍦板潃绌洪棿鎿嶄綔缁撴瀯涓紩鐢細

 - `iomap_dirty_folio`
 - `iomap_release_folio`
 - `iomap_invalidate_folio`
 - `iomap_is_partially_uptodate`

涓嬪垪鍦板潃绌洪棿鎿嶄綔鍙交鏉惧皝瑁咃細

 - `read_folio`
 - `readahead`
 - `writepages`
 - `bmap`
 - `swap_activate`

### ``struct iomap_write_ops``


 struct iomap_write_ops {
     struct folio **(**get_folio)(struct iomap_iter *iter, loff_t pos,
                                unsigned len);
     void (**put_folio)(struct inode **inode, loff_t pos, unsigned copied,
                       struct folio *folio);
     bool (**iomap_valid)(struct inode **inode, const struct iomap *iomap);
     int (**read_folio_range)(const struct iomap_iter **iter,
     			struct folio *folio, loff_t pos, size_t len);
 };

iomap 璋冪敤浠ヤ笅鍑芥暟锛?
  - `get_folio`锛氬湪寮€濮嬪啓涔嬪墠璋冪敤锛岀敤浜庡垎閰嶅苟杩斿洖涓€涓凡閿佸畾 folio 鐨勬椿鍔ㄥ紩鐢ㄣ€?    鑻ユ湭鎻愪緵姝ゅ嚱鏁帮紝iomap 灏嗚皟鐢?`iomap_get_folio`銆?    杩欏彲鐢ㄤ簬 `涓轰竴娆″啓璁剧疆 per-folio 鏂囦欢绯荤粺鐘舵€?    <https://lore.kernel.org/all/20190429220934.10415-5-agruenba@redhat.com/>`_銆?
  - `put_folio`锛氬湪 pagecache 鎿嶄綔瀹屾垚鍚庤皟鐢紝鐢ㄤ簬瑙ｉ攣骞堕噴鏀句竴涓?folio銆?    鑻ユ湭鎻愪緵姝ゅ嚱鏁帮紝iomap 灏嗚嚜琛屾墽琛?`folio_unlock` 涓?`folio_put`銆?    杩欏彲鐢ㄤ簬 `鎻愪氦鐢?->get_folio 璁剧疆鐨?per-folio 鏂囦欢绯荤粺鐘舵€?    <https://lore.kernel.org/all/20180619164137.13720-6-hch@lst.de/>`_銆?
  - `iomap_valid`锛氭枃浠剁郴缁熶笉鑳藉湪 `->iomap_begin` 涓?`->iomap_end` 涔嬮棿鎸佹湁閿侊紝鍥犱负 pagecache 鎿嶄綔鍙兘鑾峰彇 folio 閿併€佸鐢ㄦ埛绌洪棿椤典骇鐢熺己椤点€佷负鍐呭瓨鍥炴敹鍙戣捣鍥炲啓锛屾垨杩涜鍏朵粬鑰楁椂鎿嶄綔銆?    濡傛灉鏂囦欢鐨勭┖闂存槧灏勬暟鎹槸鍙彉鐨勶紝閭ｄ箞鏌愪釜 pagecache folio 鐨勬槧灏勬湁鍙兘鍦ㄥ垎閰嶃€佸畨鏀惧苟閿佸畾璇?folio 鐨勮繖娈垫椂闂村唴鍙戠敓鍙樺寲銆?
    瀵逛簬 pagecache锛屽鏋滃洖鍐欎笉鑾峰彇 `i_rwsem` 鎴?`invalidate_lock` 骞舵洿鏂版槧灏勪俊鎭紝灏卞彲鑳藉彂鐢熺珵浜夈€?    濡傛灉鏂囦欢绯荤粺鍏佽骞跺彂鍐欙紝涔熷彲鑳藉彂鐢熺珵浜夈€?    瀵逛簬姝ょ被鏂囦欢锛屽繀椤诲湪鑾峰彇 folio 閿佷箣鍚庨噸鏂版牎楠屾槧灏勶紝浠ヤ究 iomap 鑳芥纭鐞嗚 folio銆?
    fsdax 涓嶉渶瑕佽繖绉嶉噸鏂版牎楠岋紝鍥犱负瀹冩病鏈夊洖鍐欙紝涔熶笉鏀寔 unwritten extent銆?
    鍙楁绫荤珵浜夊奖鍝嶇殑鏂囦欢绯荤粺蹇呴』鎻愪緵 `->iomap_valid` 鍑芥暟鏉ュ喅瀹氭槧灏勬槸鍚︿粛鐒舵湁鏁堛€?    濡傛灉鏄犲皠鏃犳晥锛屽皢閲嶆柊閲囨牱鏄犲皠銆?
    涓轰簡鏀寔鏈夋晥鎬у垽瀹氾紝鏂囦欢绯荤粺鐨?`->iomap_begin` 鍑芥暟鍦ㄥ～鍏呭叾浠?iomap 瀛楁鐨勫悓鏃讹紝鍙互璁剧疆 `struct iomap::validity_cookie`銆?    涓€涓畝鍗曠殑鏍￠獙 cookie 瀹炵幇鏄簭鍒楄鏁板櫒銆?    濡傛灉鏂囦欢绯荤粺鍦ㄦ瘡娆′慨鏀?inode 鐨?extent map 鏃堕兘閫掑搴忓垪璁℃暟鍣紝灏卞彲浠ュ湪 `->iomap_begin` 鏈熼棿灏嗗叾鏀惧叆 ``struct iomap::validity_cookie`` 涓€?    濡傛灉鍥炰紶缁?`->iomap_valid` 鏃?cookie 涓殑鍊艰鍙戠幇涓庢枃浠剁郴缁熸寔鏈夌殑鍊间笉鍚岋紝閭ｄ箞搴旇涓鸿 iomap 宸茶繃鏈燂紝鏍￠獙澶辫触銆?
  - `read_folio_range`锛氳皟鐢ㄤ互鍚屾璇诲叆灏嗚鍐欏叆鐨勮寖鍥淬€傝嫢鏈彁渚涙鍑芥暟锛宨omap 灏嗛粯璁ゆ彁浜や竴涓?bio 璇昏姹傘€?
杩欎簺 `struct kiocb` 鏍囧織瀵?iomap 鐨勭紦鍐?I/O 寰堥噸瑕侊細

 - `IOCB_NOWAIT`锛氬紑鍚?`IOMAP_NOWAIT`銆?
 - `IOCB_DONTCACHE`锛氬紑鍚?`IOMAP_DONTCACHE`銆?
### ``struct iomap_read_ops``


 struct iomap_read_ops {
     int (**read_folio_range)(const struct iomap_iter **iter,
                             struct iomap_read_folio_ctx *ctx, size_t len);
     void (**submit_read)(struct iomap_read_folio_ctx **ctx);
 };

iomap 璋冪敤浠ヤ笅鍑芥暟锛?
  - `read_folio_range`锛氳皟鐢ㄤ互璇诲叆璇ヨ寖鍥淬€傝皟鐢ㄨ€呭繀椤绘彁渚涙鍑芥暟銆傝嫢鎴愬姛锛屾棤璁鸿鎴愬姛涓庡惁锛屽湪璇诲叆璇ヨ寖鍥村悗閮藉繀椤昏皟鐢?iomap_finish_folio_read()銆?
  - `submit_read`锛氭彁浜や换浣曟寕璧风殑璇昏姹傘€傛鍑芥暟涓哄彲閫夈€?
### 姣?Folio 鍐呴儴鐘舵€?

濡傛灉 fsblock 澶у皬涓?pagecache folio 澶у皬涓€鑷达紝鍒欏亣瀹氭墍鏈夌鐩?I/O 鎿嶄綔閮戒綔鐢ㄤ簬鏁翠釜 folio銆?瀵逛簬杩欑鎯呭喌锛屼粎闇€ folio 鐨?uptodate锛堝唴瀛樺唴瀹硅嚦灏戜笌纾佺洏涓婁竴鏍锋柊锛夊拰 dirty锛堝唴瀛樺唴瀹规瘮纾佺洏涓婃洿鏂帮級鐘舵€佸嵆鍙€?
濡傛灉 fsblock 澶у皬灏忎簬 pagecache folio 澶у皬锛宨omap 鑷璺熻釜姣忎釜 fsblock 鐨?uptodate 涓?dirty 鐘舵€併€?杩欎娇寰?iomap 鏃㈣兘澶勭悊 "bs < ps" `鏂囦欢绯荤粺
<https://lore.kernel.org/all/20230725122932.144426-1-ritesh.list@gmail.com/>`_锛屼篃鑳藉鐞?pagecache 涓殑澶?folio銆?
iomap 鍦ㄥ唴閮ㄤ负姣忎釜 fsblock 璺熻釜涓や釜鐘舵€佷綅锛?
 - `uptodate`锛歩omap 浼氬敖閲忎繚鎸?folio 瀹屽叏鏄渶鏂扮殑銆?   濡傛灉瀛樺湪璇伙紙棰勮锛夐敊璇紝閭ｄ簺 fsblock 涓嶄細琚爣璁颁负 uptodate銆?   褰?folio 鍐呮墍鏈?fsblock 閮芥槸 uptodate 鏃讹紝folio 鏈韩浼氳鏍囪涓?uptodate銆?
 - `dirty`锛氬綋绋嬪簭鍐欏叆鏂囦欢鏃讹紝iomap 浼氳缃?per-block 鐨?dirty 鐘舵€併€?   褰?folio 鍐呬换鎰?fsblock 涓?dirty 鏃讹紝folio 鏈韩浼氳鏍囪涓?dirty銆?
iomap 杩樿窡韪鍦ㄨ繘琛岀殑璇诲啓纾佺洏 I/O 鏁伴噺銆?璇ョ粨鏋勬瘮 `struct buffer_head` 杞婚噺寰楀锛屽洜涓烘瘡涓?folio 鍙湁涓€涓紝涓?per-fsblock 寮€閿€鏄袱涓綅瀵规瘮 104 瀛楄妭銆?
甯屾湜鍦?pagecache 涓紑鍚ぇ folio 鐨勬枃浠剁郴缁燂紝搴斿湪鍒濆鍖?incore inode 鏃惰皟鐢?`mapping_set_large_folios`銆?
### 缂撳啿棰勮涓庤鍙?

`iomap_readahead` 鍑芥暟鍚?pagecache 鍙戣捣棰勮銆?`iomap_read_folio` 鍑芥暟灏嗕竴浠?folio 澶у皬鐨勬暟鎹鍏?pagecache銆?浼犵粰 `->iomap_begin` 鐨?`flags` 鍙傛暟灏嗚璁句负闆躲€?pagecache 鍦ㄨ皟鐢ㄦ枃浠剁郴缁熶箣鍓嶄細鑾峰彇鎵€闇€鐨勪换浣曢攣銆?
`iomap_readahead` 涓?`iomap_read_folio` 閮戒紶鍏ヤ竴涓?``struct iomap_read_folio_ctx``锛?

 struct iomap_read_folio_ctx {
    const struct iomap_read_ops *ops;
    struct folio *cur_folio;
    struct readahead_control *rac;
    void *read_ctx;
 };

`iomap_readahead` 蹇呴』璁剧疆锛? - `ops->read_folio_range()` 涓?`rac`

`iomap_read_folio` 蹇呴』璁剧疆锛? - `ops->read_folio_range()` 涓?`cur_folio`

`ops->submit_read()` 涓?`read_ctx` 涓哄彲閫夈€俙read_ctx` 鐢ㄤ簬鍦?ops 鍥炶皟涓紶閫掕皟鐢ㄨ€呴渶瑕佽闂殑鑷畾涔夋暟鎹紝浠ユ弧瓒宠鍙栭渶姹傘€?
### 缂撳啿鍐?

`iomap_file_buffered_write` 鍑芥暟灏嗕竴涓?`iocb` 鍐欏叆 pagecache銆?`IOMAP_WRITE` 鎴?`IOMAP_WRITE` | `IOMAP_NOWAIT` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鍏变韩鎴栫嫭鍗犳ā寮忚幏鍙?`i_rwsem`銆?
#### mmap 鍐欑己椤?

`iomap_page_mkwrite` 鍑芥暟澶勭悊瀵?pagecache 涓煇 folio 鐨勫啓缂洪〉銆?`IOMAP_WRITE | IOMAP_FAULT` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鍏变韩鎴栫嫭鍗犳ā寮忚幏鍙?mmap 鐨?`invalidate_lock`銆?
#### 缂撳啿鍐欏け璐?

瀵?pagecache 鐨勭煭鍐欎箣鍚庯紝鏈鍐欏叆鐨勫尯鍩熶笉浼氳鏍囪涓?dirty銆?鏂囦欢绯荤粺蹇呴』瀹夋帓 `鍙栨秷
<https://lore.kernel.org/all/20221123055812.747923-6-david@fromorbit.com/>`_ 杩欑被 `棰勭暀
<https://lore.kernel.org/linux-xfs/20220817093627.GZ3600936@dread.disaster.area/>`_锛屽洜涓哄洖鍐欎笉浼氭秷鑰楄棰勭暀銆?`iomap_write_delalloc_release` 鍙粠 `->iomap_end` 鍑芥暟璋冪敤锛屼互鏌ユ壘缂撳瓨浜嗗叏鏂帮紙`IOMAP_F_NEW`锛塪elalloc 鏄犲皠鐨?folio 鐨勬墍鏈夊共鍑€鍖哄煙銆?瀹冭幏鍙?`invalidate_lock`銆?
鏂囦欢绯荤粺蹇呴』鎻愪緵涓€涓?`punch` 鍑芥暟锛屽澶勪簬姝ょ姸鎬佺殑姣忎釜鏂囦欢鑼冨洿璋冪敤銆?姝ゅ嚱鏁板繀椤?*浠?*绉婚櫎寤惰繜鍒嗛厤棰勭暀锛屼互闃蹭笌褰撳墠绾跨▼绔炰簤鐨勫彟涓€涓嚎绋嬫垚鍔熷啓鍏ュ悓涓€鍖哄煙骞惰Е鍙戝洖鍐欏皢鑴忔暟鎹埛鍒扮鐩樸€?
#### 鏂囦欢鎿嶄綔鐨勯浂濉厖


鏂囦欢绯荤粺鍙互璋冪敤 `iomap_zero_range` 鏉ュ鏈笌 fsblock 澶у皬瀵归綈鐨勯潪鎴柇鏂囦欢鎿嶄綔鎵ц pagecache 鐨勯浂濉厖銆?`IOMAP_ZERO` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鐙崰妯″紡鎸佹湁 `i_rwsem` 涓?`invalidate_lock`銆?
#### 鍙栨秷鍏变韩 Reflinked 鏂囦欢鏁版嵁


鏂囦欢绯荤粺鍙互璋冪敤 `iomap_file_unshare` 寮哄埗涓€涓笌鍙︿竴鏂囦欢鍏变韩瀛樺偍鐨勬枃浠讹紝棰勫厛灏嗗叡浜暟鎹鍒跺埌鏂板垎閰嶇殑瀛樺偍涓€?`IOMAP_WRITE | IOMAP_UNSHARE` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鐙崰妯″紡鎸佹湁 `i_rwsem` 涓?`invalidate_lock`銆?
### 鎴柇


鏂囦欢绯荤粺鍙互璋冪敤 `iomap_truncate_page` 鍦ㄦ枃浠舵埅鏂搷浣滄湡闂达紝灏?pagecache 涓粠 EOF 鍒?fsblock 鏈熬鐨勫瓧鑺傛竻闆躲€?`truncate_setsize` 鎴?`truncate_pagecache` 灏嗗鐞?EOF 鍧椾箣鍚庣殑鎵€鏈夊唴瀹广€?`IOMAP_ZERO` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鐙崰妯″紡鎸佹湁 `i_rwsem` 涓?`invalidate_lock`銆?
### Pagecache 鍥炲啓


鏂囦欢绯荤粺鍙互璋冪敤 `iomap_writepages` 鏉ュ搷搴斿皢鑴?pagecache folio 鍐欏洖纾佺洏鐨勮姹傘€?`mapping` 涓?`wbc` 鍙傛暟搴斿師鏍蜂紶閫掋€?`wpc` 鎸囬拡搴旂敱鏂囦欢绯荤粺鍒嗛厤锛屼笖蹇呴』鍒濆鍖栦负闆躲€?
pagecache 鍦ㄥ皾璇曡皟搴︽煇涓?folio 杩涜鍥炲啓涔嬪墠浼氶攣瀹氬畠銆?瀹冧笉浼氶攣瀹?`i_rwsem` 鎴?`invalidate_lock`銆?
鍗充娇鍥炲啓澶辫触锛岀粡杩囦笅杩?`->writeback_range` 鏈哄埗鐨?folio 鐨?dirty 浣嶄篃浼氳娓呴櫎銆?杩欐槸涓轰簡闃叉瀛樺偍璁惧鏁呴殰鏃跺嚭鐜拌剰 folio 缁撳潡锛涗細璁板綍涓€涓?`-EIO` 渚涚敤鎴风┖闂撮€氳繃 `fsync` 鏀堕泦銆?
`ops` 缁撴瀯蹇呴』鎸囧畾锛屽涓嬫墍绀猴細

#### ``struct iomap_writeback_ops``


 struct iomap_writeback_ops {
    int (**writeback_range)(struct iomap_writepage_ctx **wpc,
        struct folio *folio, u64 pos, unsigned int len, u64 end_pos);
    int (**writeback_submit)(struct iomap_writepage_ctx **wpc, int error);
 };

瀛楁濡備笅锛?
  - `writeback_range`锛氬皢 `wpc->iomap` 璁剧疆涓虹敱 `offset` 涓?`len` 缁欏嚭鐨勬枃浠惰寖鍥达紙瀛楄妭锛夌殑绌洪棿鏄犲皠銆?    iomap 瀵规瘡涓剰 folio 涓殑姣忎釜鑴?fs 鍧楄皟鐢ㄦ鍑芥暟锛屼笉杩囧浜?folio 鍐呰繛缁剰 fsblock 鐨勮繍琛屼細 `澶嶇敤鏄犲皠
    <https://lore.kernel.org/all/20231207072710.176093-15-hch@lst.de/>`_銆?    涓嶈鍦ㄦ杩斿洖 `IOMAP_INLINE` 鏄犲皠锛沗->iomap_end` 鍑芥暟蹇呴』澶勭悊宸插啓鏁版嵁鐨勬寔涔呭寲銆?    涓嶈鍦ㄦ杩斿洖 `IOMAP_DELALLOC` 鏄犲皠锛沬omap 褰撳墠瑕佹眰鏄犲皠鍒板凡鍒嗛厤鐨勭┖闂淬€?    濡傛灉鏄犲皠鏈敼鍙橈紝鏂囦欢绯荤粺鍙互璺宠繃鍙兘鏄傝吹鐨勬槧灏勬煡鎵俱€?    杩欑閲嶆柊鏍￠獙蹇呴』鐢辨枃浠剁郴缁熸樉寮忕紪鐮佸疄鐜帮紱灏氫笉娓呮 `iomap::validity_cookie` 鑳藉惁澶嶇敤浜庢鐩殑銆?
    濡傛灉璇ユ柟娉曟湭鑳戒负鏌愪釜鑴?folio 鐨勪换浣曢儴鍒嗚皟搴?I/O锛屽畠搴斾涪寮冨彲鑳戒负璇ュ啓鎵€鍋氱殑浠讳綍棰勭暀銆?    folio 灏嗚鏍囪涓哄共鍑€锛屽苟鍦?pagecache 涓褰曚竴涓?`-EIO`銆?    鏂囦欢绯荤粺鍙互浣跨敤姝ゅ洖璋?`绉婚櫎
    <https://lore.kernel.org/all/20201029163313.1766967-1-bfoster@redhat.com/>`_ delalloc 棰勭暀锛屼互閬垮厤涓哄共鍑€鐨?pagecache 淇濈暀 delalloc 棰勭暀銆?    姝ゅ嚱鏁板繀椤荤敱鏂囦欢绯荤粺鎻愪緵銆?    濡傛灉鎴愬姛锛屾棤璁哄洖鍐欐垚鍔熶笌鍚︼紝鍦ㄨ寖鍥村洖鍐欏畬鎴愬悗閮藉繀椤昏皟鐢ㄤ竴娆?iomap_finish_folio_write()銆?
  - `writeback_submit`锛氭彁浜や箣鍓嶆瀯寤虹殑鍥炲啓涓婁笅鏂囥€?    鍩轰簬鍧楃殑鏂囦欢绯荤粺搴斾娇鐢?iomap_ioend_writeback_submit 杈呭姪鍑芥暟锛屽叾浠栨枃浠剁郴缁熷彲瀹炵幇鑷繁鐨勩€?    鏂囦欢绯荤粺鍙互閫夋嫨鎬у湴鎸傛帴鍒板洖鍐?bio 鎻愪氦銆?    杩欏彲鑳藉寘鎷啓鍓嶇殑绌洪棿璁拌处鏇存柊锛屾垨涓哄唴閮ㄧ洰鐨勫畨瑁呰嚜瀹氫箟鐨?`->bi_end_io` 鍑芥暟锛屼緥濡傚皢 ioend 瀹屾垚寤惰繜鍒?workqueue锛屼互渚垮湪鎻愪氦 bio 涔嬪墠浠庤繘绋嬩笂涓嬫枃杩愯鍏冩暟鎹洿鏂颁簨鍔°€?    姝ゅ嚱鏁板繀椤荤敱鏂囦欢绯荤粺鎻愪緵銆?
#### Pagecache 鍥炲啓瀹屾垚


涓轰簡澶勭悊鍥炲啓纾佺洏 I/O 瀹屾垚鍚庡繀椤昏繘琛岀殑绨胯锛宨omap 鍒涘缓浜?`struct iomap_ioend` 瀵硅薄閾撅紝杩欎簺瀵硅薄灏佽浜嗙敤浜庡皢 pagecache 鏁版嵁鍐欏叆纾佺洏鐨?`bio`銆?榛樿鎯呭喌涓嬶紝iomap 閫氳繃娓呴櫎闄勫姞鍒?`ioend` 鐨?folio 涓婄殑 writeback 浣嶆潵瀹屾垚鍥炲啓 ioend銆?濡傛灉鍐欏け璐ワ紝瀹冭繕浼氬湪 folio 鍜屽湴鍧€绌洪棿涓婅缃敊璇綅銆?杩欏彲鑳藉彂鐢熷湪涓柇鎴栬繘绋嬩笂涓嬫枃涓紝鍙栧喅浜庡瓨鍌ㄨ澶囥€?闇€瑕佹洿鏂板唴閮ㄧ翱璁帮紙渚嬪 unwritten extent 杞崲锛夌殑鏂囦欢绯荤粺搴斿湪 `->submit_writeback` 鎻愪氦鐨?bio 涓婅缃嚜宸辩殑 bi_end_io銆?姝ゅ嚱鏁板簲鍦ㄥ畬鎴愯嚜韬伐浣滐紙渚嬪 unwritten extent 杞崲锛夊悗璋冪敤 `iomap_finish_ioends`銆?
鏌愪簺鏂囦欢绯荤粺鍙兘甯屾湜 `鍒嗘憡杩愯鍏冩暟鎹簨鍔?<https://lore.kernel.org/all/20220120034733.221737-1-david@fromorbit.com/>`_ 鐨勬垚鏈紝浠ュ鍐欏悗鏇存柊杩涜鎵瑰鐞嗐€?瀹冧滑鍙兘杩樿姹備簨鍔′粠杩涚▼涓婁笅鏂囪繍琛岋紝杩欐剰鍛崇潃灏嗘壒娆℃帹閫佸埌 workqueue銆?iomap ioend 鍖呭惈涓€涓?`list_head` 浠ユ敮鎸佹壒澶勭悊銆?
缁欏畾涓€鎵?ioend锛宨omap 鏈夊嚑涓緟鍔╁嚱鏁板崗鍔╁垎鎽婏細

 - `iomap_sort_ioends`锛氭寜鏂囦欢鍋忕Щ瀵瑰垪琛ㄤ腑鐨勬墍鏈?ioend 鎺掑簭銆?
 - `iomap_ioend_try_merge`锛氱粰瀹氫竴涓笉鍦ㄤ换浣曞垪琛ㄤ腑鐨?ioend 浠ュ強鍙︿竴涓凡鎺掑簭鐨?ioend 鍒楄〃锛屽皢鍒楄〃涓敖鍙兘澶氱殑 ioend 浠庡ご閮ㄥ悎骞跺埌缁欏畾 ioend 涓€?   鍙湁褰撴枃浠惰寖鍥村拰瀛樺偍鍦板潃杩炵画銆乽nwritten 涓?shared 鐘舵€佺浉鍚屻€佷笖鍐?I/O 缁撴灉鐩稿悓鏃讹紝ioend 鎵嶈兘鍚堝苟銆?   鍚堝苟鍚庣殑 ioend 鑷垚涓€浣撴垚涓轰竴涓垪琛ㄣ€?
 - `iomap_finish_ioends`锛氬畬鎴愪竴涓彲鑳介摼鎺ヤ簡鍏朵粬 ioend 鐨?ioend銆?
## 鐩存帴 I/O


鍦?Linux 涓紝鐩存帴 I/O 瀹氫箟涓虹洿鎺ュ彂寰€瀛樺偍銆佺粫杩?pagecache 鐨勬枃浠?I/O銆?`iomap_dio_rw` 鍑芥暟瀹炵幇浜嗘枃浠剁殑 O_DIRECT锛堢洿鎺?I/O锛夎鍜屽啓銆?

 ssize_t iomap_dio_rw(struct kiocb **iocb, struct iov_iter **iter,
                      const struct iomap_ops *ops,
                      const struct iomap_dio_ops *dops,
                      unsigned int dio_flags, void *private,
                      size_t done_before);

鏂囦欢绯荤粺鍙互鎻愪緵 `dops` 鍙傛暟锛屽鏋滃畠闇€瑕佸湪 I/O 鍙戝線瀛樺偍鍓嶅悗鎵ц棰濆宸ヤ綔銆?`done_before` 鍙傛暟鍛婄煡宸茬粡浼犺緭浜嗗灏戣姹傘€?瀹冪敤浜庡湪 `璇锋眰鐨勪竴閮ㄥ垎
<https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=c03098d4b9ad76bca2966a8769dcfe59f7f85103>`_ 宸茬粡鍚屾瀹屾垚鏃讹紝寮傛鍦扮户缁竴涓姹傘€?
濡傛灉璋冪敤鍓嶅凡涓?`iocb` 鍚姩浜嗗啓锛屽垯搴旇缃?`done_before` 鍙傛暟銆?I/O 鐨勬柟鍚戠敱浼犲叆鐨?`iocb` 鍐冲畾銆?
`dio_flags` 鍙傛暟鍙缃负涓嬪垪鍊肩殑浠绘剰缁勫悎锛?
 - `IOMAP_DIO_FORCE_WAIT`锛氬嵆浣?kiocb 涓嶆槸鍚屾鐨勶紝涔熺瓑寰?I/O 瀹屾垚銆?
 - `IOMAP_DIO_OVERWRITE_ONLY`锛氬姝よ寖鍥存墽琛岀函瑕嗙洊鍐欙紝鍚﹀垯浠?`-EAGAIN` 澶辫触銆?   杩欏彲琚叿鏈夊鏉傛湭瀵归綈 I/O 鍐欒矾寰勭殑鏂囦欢绯荤粺鐢ㄦ潵涓烘湭瀵归綈鍐欐彁渚涗紭鍖栫殑蹇€熻矾寰勩€?   濡傛灉鑳芥墽琛岀函瑕嗙洊鍐欙紝鍒欐棤闇€閽堝鍚屼竴鏂囦欢绯荤粺鍧楃殑鍏朵粬 I/O 杩涜涓茶鍖栵紝鍥犱负娌℃湁鏆撮湶闄堟棫鏁版嵁鎴栨暟鎹涪澶辩殑椋庨櫓銆?   濡傛灉鏃犳硶鎵ц绾鐩栧啓锛屽垯鏂囦欢绯荤粺鍙互鎵ц鎵€闇€鐨勪覆琛屽寲姝ラ锛屼互鎻愪緵瀵规湭瀵归綈 I/O 鑼冨洿鐨勭嫭鍗犺闂紝浠庤€屽畨鍏ㄥ湴鎵ц鍒嗛厤鍜屽瓙鍧楅浂濉厖銆?   鏂囦欢绯荤粺鍙娇鐢ㄦ鏍囧織灏濊瘯鍑忓皯閿佺珵浜夛紝浣嗚 `姝ｇ‘
   <https://lore.kernel.org/linux-ext4/20230314130759.642710-1-bfoster@redhat.com/>`_ 鍋氬埌闇€瑕佸ぇ閲?`缁嗚嚧妫€鏌?   <https://lore.kernel.org/linux-ext4/20230810165559.946222-1-bfoster@redhat.com/>`_銆?
 - `IOMAP_DIO_PARTIAL`锛氬鏋滃彂鐢熺己椤碉紝杩斿洖宸插畬鎴愮殑浠讳綍杩涘害銆?   璋冪敤鑰呭彲浠ュ鐞嗙己椤靛苟閲嶈瘯璇ユ搷浣溿€?   濡傛灉璋冪敤鑰呭喅瀹氶噸璇曡鎿嶄綔锛屽簲灏嗕箣鍓嶆墍鏈夎皟鐢ㄧ殑绱杩斿洖鍊间綔涓?`done_before` 鍙傛暟浼犵粰涓嬩竴娆¤皟鐢ㄣ€?
杩欎簺 `struct kiocb` 鏍囧織瀵?iomap 鐨勭洿鎺?I/O 寰堥噸瑕侊細

 - `IOCB_NOWAIT`锛氬紑鍚?`IOMAP_NOWAIT`銆?
 - `IOCB_SYNC`锛氱‘淇濆湪瀹屾垚璋冪敤涔嬪墠璁惧宸插皢鎸佷箙鍖栨暟鎹啓鍏ョ鐩樸€?   鍦ㄧ函瑕嗙洊鍐欑殑鎯呭喌涓嬶紝I/O 鍙兘浠ュ惎鐢?FUA 鐨勬柟寮忓彂鍑恒€?
 - `IOCB_HIPRI`锛氳疆璇?I/O 瀹屾垚锛岃€屼笉鏄瓑寰呬腑鏂€?   浠呭寮傛 I/O 鏈夋剰涔夛紝涓斾粎褰撴暣涓?I/O 鍙互浣滀负鍗曚釜 `struct bio` 鍙戝嚭鏃躲€?
鏂囦欢绯荤粺搴斾粠 `->read_iter` 鍜?`->write_iter` 璋冪敤 `iomap_dio_rw`锛屽苟鍦ㄦ枃浠剁殑 `->open` 鍑芥暟涓缃?`FMODE_CAN_ODIRECT`銆?瀹冧滑涓嶅簲璁剧疆 `->direct_IO`锛岃瀛楁宸插簾寮冦€?
濡傛灉鏂囦欢绯荤粺甯屾湜鍦ㄧ洿鎺?I/O 瀹屾垚鍓嶆墽琛岃嚜韬伐浣滐紝瀹冨簲璋冪敤 `__iomap_dio_rw`銆?濡傛灉鍏惰繑鍥炲€间笉鏄敊璇寚閽堟垨 NULL 鎸囬拡锛屾枃浠剁郴缁熷簲鍦ㄥ畬鎴愬唴閮ㄥ伐浣滃悗灏嗚繑鍥炲€间紶缁?`iomap_dio_complete`銆?
### 杩斿洖鍊?

`iomap_dio_rw` 鍙繑鍥炰互涓嬩箣涓€锛?
 - 涓€涓潪璐熷瓧鑺傛暟锛岃〃绀哄凡浼犺緭鐨勫瓧鑺傘€?
 - `-ENOTBLK`锛氬洖閫€鍒扮紦鍐?I/O銆?   濡傛灉 iomap 鏃犳硶鍦ㄥ皢 I/O 鍙戝線瀛樺偍鍓嶄娇 page cache 澶辨晥锛屽畠鑷韩浼氳繑鍥炴鍊笺€?   `->iomap_begin` 鎴?`->iomap_end` 鍑芥暟涔熷彲鑳借繑鍥炴鍊笺€?
 - `-EIOCBQUEUED`锛氬紓姝ョ洿鎺?I/O 璇锋眰宸插叆闃燂紝灏嗗崟鐙畬鎴愩€?
 - 浠讳綍鍏朵粬璐熼敊璇爜銆?
### 鐩存帴璇?

鐩存帴 I/O 璇诲彂璧蜂粠瀛樺偍璁惧鍒拌皟鐢ㄨ€呯紦鍐插尯鐨勮 I/O銆?鍦ㄥ彂璧疯 I/O 涔嬪墠锛宲agecache 鐨勮剰閮ㄥ垎浼氳鍒峰洖瀛樺偍銆?`->iomap_begin` 鐨?`flags` 鍊煎皢鏄?`IOMAP_DIRECT`锛屽彲闄勫姞涓嬪垪澧炲己鐨勭粍鍚堬細

 - `IOMAP_NOWAIT`锛屽鍓嶆墍杩般€?
璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鍏变韩妯″紡鎸佹湁 `i_rwsem`銆?
### 鐩存帴鍐?

鐩存帴 I/O 鍐欏彂璧蜂粠璋冪敤鑰呯紦鍐插尯鍒板瓨鍌ㄨ澶囩殑鍐?I/O銆?鍦ㄥ彂璧峰啓 I/O 涔嬪墠锛宲agecache 鐨勮剰閮ㄥ垎浼氳鍒峰洖瀛樺偍銆?鍦ㄥ啓 I/O 鍓嶅悗閮戒細浣?pagecache 澶辨晥銆?`->iomap_begin` 鐨?`flags` 鍊煎皢鏄?``IOMAP_DIRECT | IOMAP_WRITE``锛屽彲闄勫姞涓嬪垪澧炲己鐨勭粍鍚堬細

 - `IOMAP_NOWAIT`锛屽鍓嶆墍杩般€?
 - `IOMAP_OVERWRITE_ONLY`锛氫笉鍏佽鍒嗛厤鍧楀拰闆跺～鍏呴儴鍒嗗潡銆?   鏁翠釜鏂囦欢鑼冨洿蹇呴』鏄犲皠鍒板崟涓凡鍐欐垨 unwritten extent銆?   濡傛灉鏄犲皠鏄?unwritten 鐨勶紝涓旀枃浠剁郴缁熸棤娉曞湪涓嶆毚闇查檲鏃у唴瀹圭殑鎯呭喌涓嬪鐞嗘湭瀵归綈鍖哄煙鐨勯浂濉厖锛屽垯鏂囦欢 I/O 鑼冨洿蹇呴』瀵归綈鍒版枃浠剁郴缁熷潡澶у皬銆?
 - `IOMAP_ATOMIC`锛氭鍐欏甫鏈夋挄瑁傚啓淇濇姢銆?   鎾曡鍐欎繚鎶ゅ彲鍩轰簬纭欢鍗歌浇鎻愪緵锛屾垨鐢辨枃浠剁郴缁熸彁渚涚殑杞欢鏈哄埗鎻愪緵銆?
   瀵逛簬鍩轰簬纭欢鍗歌浇鐨勬敮鎸侊紝鍐欏彧鑳藉垱寤轰竴涓?bio锛屼笖鍐欎笉寰楁媶鍒嗕负澶氫釜 I/O 璇锋眰锛屽嵆蹇呴』璁剧疆 REQ_ATOMIC 鏍囧織銆?   瑕佸啓鍏ョ殑鏂囦欢鑼冨洿蹇呴』瀵归綈锛屼互婊¤冻鏂囦欢绯荤粺鍜屽簳灞傚潡璁惧鍘熷瓙鎻愪氦鑳藉姏鐨勮姹傘€?   濡傛灉闇€瑕佹枃浠剁郴缁熷厓鏁版嵁鏇存柊锛堜緥濡?unwritten extent 杞崲鎴栧啓鏃跺鍒讹級锛屾暣涓枃浠惰寖鍥寸殑鎵€鏈夋洿鏂颁篃蹇呴』鍘熷瓙鎻愪氦銆?   鏃犳崯鍐欏彲鑳芥瘮鍗曚釜鏂囦欢鍧楁洿闀裤€傚湪鎵€鏈夋儏鍐典笅锛屾槧灏勮捣濮嬬殑纾佺洏鍧楀繀椤昏嚦灏戜笌鍐欏亸绉诲叿鏈夌浉鍚岀殑瀵归綈銆?   鏂囦欢绯荤粺蹇呴』璁剧疆 IOMAP_F_ATOMIC_BIO 浠ュ憡鐭?iomap 鏍稿績鍩轰簬纭欢鍗歌浇鐨勬棤鎹熷啓銆?
   瀵逛簬鍩轰簬鏂囦欢绯荤粺鎻愪緵鐨勮蒋浠舵満鍒剁殑鏃犳崯鍐欙紝閫傜敤浜庡熀浜庣‖浠跺嵏杞界殑鏃犳崯鍐欑殑纾佺洏鍧楀榻愬拰鍗?bio 闄愬埗鍧囦笉閫傜敤銆?   璇ユ満鍒堕€氬父鐢ㄤ綔鍩轰簬纭欢鍗歌浇鐨勬棤鎹熷啓鍙兘鏃犳硶鍙戝嚭鏃剁殑鍥為€€锛屼緥濡傚啓鍏ヨ寖鍥磋鐩栧涓?extent锛屾剰鍛崇潃鏃犳硶鍙戝嚭鍗曚釜 bio銆?   鏁翠釜鏂囦欢鑼冨洿鐨勬墍鏈夋枃浠剁郴缁熷厓鏁版嵁鏇存柊涔熷繀椤诲師瀛愭彁浜ゃ€?
璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鍏变韩鎴栫嫭鍗犳ā寮忔寔鏈?`i_rwsem`銆?
### ``struct iomap_dio_ops:``


 struct iomap_dio_ops {
     void (**submit_io)(const struct iomap_iter **iter, struct bio *bio,
                       loff_t file_offset);
     int (**end_io)(struct kiocb **iocb, ssize_t size, int error,
                   unsigned flags);
     struct bio_set *bio_set;
 };

姝ょ粨鏋勭殑瀛楁濡備笅锛?
  - `submit_io`锛氬綋 iomap 鏋勯€犲ソ鎵€璇锋眰 I/O 鐨?`struct bio` 瀵硅薄骞跺笇鏈涘皢鍏舵彁浜ょ粰鍧楄澶囨椂锛屼細璋冪敤姝ゅ嚱鏁般€?   濡傛灉鏈彁渚涘嚱鏁帮紝`submit_bio` 灏嗚鐩存帴璋冪敤銆?   甯屾湜鍦ㄤ箣鍓嶆墽琛岄澶栧伐浣滐紙渚嬪 btrfs 鐨勬暟鎹鍒讹級鐨勬枃浠剁郴缁熷簲瀹炵幇姝ゅ嚱鏁般€?
  - `end_io`锛氬湪 `struct bio` 瀹屾垚鍚庤皟鐢ㄣ€?   姝ゅ嚱鏁板簲鎵ц unwritten extent 鏄犲皠鐨勫啓鍚庤浆鎹€佸鐞嗗啓澶辫触绛夈€?   `flags` 鍙傛暟鍙缃负涓嬪垪缁勫悎锛?
    - `IOMAP_DIO_UNWRITTEN`锛氭槧灏勬槸 unwritten 鐨勶紝鍥犳 ioend 搴斿皢 extent 鏍囪涓哄凡鍐欍€?
    - `IOMAP_DIO_COW`锛氬啓鍏ユ槧灏勪腑鐨勭┖闂撮渶瑕佸啓鏃跺鍒舵搷浣滐紝鍥犳 ioend 搴斿垏鎹㈡槧灏勩€?
  - `bio_set`锛氳繖鍏佽鏂囦欢绯荤粺鎻愪緵鑷畾涔夌殑 bio_set 鐢ㄤ簬鍒嗛厤鐩存帴 I/O 鐨?bio銆?    杩欎娇寰楁枃浠剁郴缁熻兘澶?`瀛樻斁棰濆鐨?per-bio 淇℃伅
    <https://lore.kernel.org/all/20220505201115.937837-3-hch@lst.de/>`_ 渚涚鏈変娇鐢ㄣ€?    濡傛灉姝ゅ瓧娈典负 NULL锛屽皢浣跨敤閫氱敤鐨?`struct bio` 瀵硅薄銆?
甯屾湜鍦?I/O 瀹屾垚鍚庢墽琛岄澶栧伐浣滅殑鏂囦欢绯荤粺搴旈€氳繃 `->submit_io` 璁剧疆鑷畾涔夌殑 `->bi_end_io` 鍑芥暟銆?涔嬪悗锛岃嚜瀹氫箟鐨?endio 鍑芥暟蹇呴』璋冪敤 `iomap_dio_bio_end_io` 鏉ュ畬鎴愮洿鎺?I/O銆?
## DAX I/O


鏌愪簺瀛樺偍璁惧鍙洿鎺ユ槧灏勪负鍐呭瓨銆?杩欎簺璁惧鏀寔涓€绉嶇О涓?"fsdax" 鐨勬柊璁块棶妯″紡锛屽厑璁搁€氳繃 CPU 鍜屽唴瀛樻帶鍒跺櫒杩涜鍔犺浇鍜屽瓨鍌ㄣ€?
### fsdax 璇?

fsdax 璇绘墽琛屼粠瀛樺偍璁惧鍒拌皟鐢ㄨ€呯紦鍐插尯鐨?memcpy銆?`->iomap_begin` 鐨?`flags` 鍊煎皢鏄?`IOMAP_DAX`锛屽彲闄勫姞涓嬪垪澧炲己鐨勭粍鍚堬細

 - `IOMAP_NOWAIT`锛屽鍓嶆墍杩般€?
璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鍏变韩妯″紡鎸佹湁 `i_rwsem`銆?
### fsdax 鍐?

fsdax 鍐欏彂璧蜂粠璋冪敤鑰呯紦鍐插尯鍒板瓨鍌ㄨ澶囩殑 memcpy銆?`->iomap_begin` 鐨?`flags` 鍊煎皢鏄?``IOMAP_DAX | IOMAP_WRITE``锛屽彲闄勫姞涓嬪垪澧炲己鐨勭粍鍚堬細

 - `IOMAP_NOWAIT`锛屽鍓嶆墍杩般€?
 - `IOMAP_OVERWRITE_ONLY`锛氳皟鐢ㄨ€呰姹備粠姝ゆ槧灏勬墽琛岀函瑕嗙洊鍐欍€?   杩欒姹傛枃浠剁郴缁?extent 鏄犲皠宸茬粡浠?`IOMAP_MAPPED` 绫诲瀷瀛樺湪锛屽苟璺ㄨ秺鏁翠釜鍐?I/O 璇锋眰鐨勮寖鍥淬€?   濡傛灉鏂囦欢绯荤粺鏃犳硶浠ュ厑璁?iomap 鍩虹璁炬柦鎵ц绾鐩栧啓鐨勬柟寮忔槧灏勬璇锋眰锛屽垯蹇呴』浠?`-EAGAIN` 浣挎槧灏勬搷浣滃け璐ャ€?
璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鐙崰妯″紡鎸佹湁 `i_rwsem`銆?
#### fsdax mmap 缂洪〉


`dax_iomap_fault` 鍑芥暟澶勭悊瀵?fsdax 瀛樺偍鐨勮鍜屽啓缂洪〉銆?瀵逛簬璇荤己椤碉紝`IOMAP_DAX | IOMAP_FAULT` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?瀵逛簬鍐欑己椤碉紝`IOMAP_DAX | IOMAP_FAULT | IOMAP_WRITE` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?
璋冪敤鑰呴€氬父鎸佹湁涓庡叾璋冪敤 iomap pagecache 瀵瑰簲鍑芥暟鐩稿悓鐨勯攣銆?
### fsdax 鎴柇銆乫allocate 涓庡彇娑堝叡浜?

瀵逛簬 fsdax 鏂囦欢锛屾彁渚涗互涓嬪嚱鏁颁互鏇挎崲鍏?iomap pagecache I/O 瀵瑰簲鍑芥暟銆?浼犵粰 `->iomap_begin` 鐨?`flags` 鍙傛暟涓?pagecache 瀵瑰簲鍑芥暟鐩稿悓锛屽彧鏄鍔犱簡 `IOMAP_DAX`銆?
 - `dax_file_unshare`
 - `dax_zero_range`
 - `dax_truncate_page`

璋冪敤鑰呴€氬父鎸佹湁涓庡叾璋冪敤 iomap pagecache 瀵瑰簲鍑芥暟鐩稿悓鐨勯攣銆?
### fsdax 鍘婚噸


瀹炵幇 `FIDEDUPERANGE` ioctl 鐨勬枃浠剁郴缁熷繀椤荤敤鍏惰嚜韬殑 iomap 璇?ops 璋冪敤 `dax_remap_file_range_prep` 鍑芥暟銆?
## 鏂囦欢瀹氫綅


iomap 瀹炵幇浜?`llseek` 绯荤粺璋冪敤鐨勪袱绉嶈凯浠?whence 妯″紡銆?
### SEEK_DATA


`iomap_seek_data` 鍑芥暟瀹炵幇浜?llseek 鐨?SEEK_DATA "whence" 鍊笺€?`IOMAP_REPORT` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?
瀵逛簬 unwritten 鏄犲皠锛屽皢鎼滅储 pagecache銆?pagecache 涓槧灏勪簡 folio 涓旇繖浜?folio 鍐呮湁 uptodate fsblock 鐨勫尯鍩熷皢琚姤鍛婁负鏁版嵁鍖哄煙銆?
璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鍏变韩妯″紡鎸佹湁 `i_rwsem`銆?
### SEEK_HOLE


`iomap_seek_hole` 鍑芥暟瀹炵幇浜?llseek 鐨?SEEK_HOLE "whence" 鍊笺€?`IOMAP_REPORT` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?
瀵逛簬 unwritten 鏄犲皠锛屽皢鎼滅储 pagecache銆?pagecache 涓病鏈夋槧灏?folio锛屾垨 folio 鍐呮湁闈?uptodate fsblock 鐨勫尯鍩熷皢琚姤鍛婁负绋€鐤忕┖娲炲尯鍩熴€?
璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鍏变韩妯″紡鎸佹湁 `i_rwsem`銆?
## 浜ゆ崲鏂囦欢婵€娲?

`iomap_swapfile_activate` 鍑芥暟鏌ユ壘鏂囦欢涓殑鎵€鏈夋寜鍩洪〉瀵归綈鐨勫尯鍩燂紝骞跺皢鍏惰缃负浜ゆ崲绌洪棿銆?鏂囦欢鍦ㄦ縺娲诲墠浼氳 `fsync()`銆?`IOMAP_REPORT` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?鎵€鏈夋槧灏勫繀椤绘槸宸叉槧灏勬垨 unwritten 鐨勶紱涓嶈兘鏄?dirty 鎴?shared 鐨勶紝涓斾笉鑳借法瓒婂涓潡璁惧銆?璋冪敤鑰呭繀椤讳互鐙崰妯″紡鎸佹湁 `i_rwsem`锛涜繖宸茬敱 `swapon` 鎻愪緵銆?
## 鏂囦欢绌洪棿鏄犲皠鎶ュ憡


iomap 瀹炵幇浜嗕袱涓枃浠剁┖闂存槧灏勭郴缁熻皟鐢ㄣ€?
### FS_IOC_FIEMAP


`iomap_fiemap` 鍑芥暟浠?`FS_IOC_FIEMAP` ioctl 鎸囧畾鐨勬牸寮忓皢鏂囦欢 extent 鏄犲皠瀵煎嚭鍒扮敤鎴风┖闂淬€?`IOMAP_REPORT` 灏嗕綔涓?`flags` 鍙傛暟浼犵粰 `->iomap_begin`銆?璋冪敤鑰呴€氬父鍦ㄨ皟鐢ㄦ鍑芥暟鍓嶄互鍏变韩妯″紡鎸佹湁 `i_rwsem`銆?
### FIBMAP锛堝凡搴熷純锛?

`iomap_bmap` 瀹炵幇 FIBMAP銆?璋冪敤绾﹀畾涓?FIEMAP 鐩稿悓銆?姝ゅ嚱鏁颁粎涓轰笌杞崲鍓嶅凡瀹炵幇 FIBMAP 鐨勬枃浠剁郴缁熶繚鎸佸吋瀹硅€屾彁渚涖€?姝?ioctl 宸插簾寮冿紱涓嶈涓烘病鏈夊畠鐨勬枃浠剁郴缁熸坊鍔?FIBMAP 瀹炵幇銆?璋冪敤鑰呭彲鑳藉湪璋冪敤姝ゅ嚱鏁板墠搴旀寔鏈?`i_rwsem` 鐨勫叡浜ā寮忥紝浣嗚繖骞朵笉鏄庣‘銆?