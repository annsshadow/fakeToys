
## Squashfs 4.0 鏂囦欢绯荤粺


Squashfs 鏄?Linux 鐨勪竴绉嶅帇缂╃殑鍙鏂囦欢绯荤粺銆?
瀹冧娇鐢?zlib銆乴z4銆乴zo銆亁z 鎴?zstd 鍘嬬缉鏉ュ帇缂╂枃浠躲€乮node 涓庣洰褰曘€傜郴缁熶腑鐨?inode
闈炲父灏忥紝骞朵笖鎵€鏈夊潡閮借鎵撳寘浠ユ渶灏忓寲鏁版嵁寮€閿€銆傛敮鎸佸ぇ浜?4K 鐨勫潡澶у皬锛屾渶澶у彲杈?1
鍏嗗瓧鑺傦紙榛樿鍧楀ぇ灏?128K锛夈€?
Squashfs 鏃ㄥ湪鐢ㄤ簬閫氱敤鐨勫彧璇绘枃浠剁郴缁熴€佸綊妗ｇ敤閫旓紙鍗冲師鏈彲鑳戒娇鐢?.tar.gz 鏂囦欢鐨?鍦烘櫙锛夛紝浠ュ強闇€瑕佷綆寮€閿€鐨勫彈闄愬埗鍧楄澶?鍐呭瓨绯荤粺锛堜緥濡傚祵鍏ュ紡绯荤粺锛夈€?
閭欢鍒楄〃锛堝唴鏍镐唬鐮侊級锛歭inux-fsdevel@vger.kernel.org
缃戠珯锛歡ithub.com/plougher/squashfs-tools

### 1. 鏂囦欢绯荤粺鐗规€?

Squashfs 鏂囦欢绯荤粺鐗规€т笌 Cramfs 鐨勫姣旓細

============================== 	=========		==========
				Squashfs		Cramfs
============================== 	=========		==========
Max filesystem size		2^64			256 MiB
Max file size			~ 2 TiB			16 MiB
Max files			unlimited		unlimited
Max directories			unlimited		unlimited
Max entries per directory	unlimited		unlimited
Max block size			1 MiB			4 KiB
Metadata compression		yes			no
Directory indexes		yes			no
Sparse file support		yes			no
Tail-end packing (fragments)	yes			no
Exportable (NFS etc.)		yes			no
Hard link support		yes			no
"." and ".." in readdir		yes			no
Real inode numbers		yes			no
32-bit uids/gids		yes			no
File creation time		yes			no
Xattr support			yes			no
ACL support			no			no
============================== 	=========		==========

Squashfs 鍘嬬缉鏁版嵁銆乮node 涓庣洰褰曘€傛澶栵紝inode 涓庣洰褰曟暟鎹楂樺害鍘嬬缉锛屽苟鎸夊瓧鑺傝竟鐣?鎵撳寘銆傛瘡涓鍘嬬缉鐨?inode 骞冲潎闀垮害涓?8 瀛楄妭锛堢‘鍒囬暱搴﹂殢鏂囦欢绫诲瀷鑰屽彉锛屽嵆甯歌鏂囦欢銆?鐩綍銆佺鍙烽摼鎺ヤ互鍙婂潡/瀛楃璁惧 inode 鐨勫ぇ灏忓悇涓嶇浉鍚岋級銆?
### 2. 浣跨敤 Squashfs


鐢变簬 squashfs 鏄彧璇绘枃浠剁郴缁燂紝蹇呴』浣跨敤 mksquashfs 绋嬪簭鏉ュ垱寤哄凡濉厖鍐呭鐨?squashfs 鏂囦欢绯荤粺銆傝绋嬪簭鍙婂叾浠?squashfs 宸ュ叿寰堝彲鑳藉凡琚綘鐨?Linux 鍙戣鐗堟墦鍖?锛堝悕涓?squashfs-tools锛夈€傛簮浠ｇ爜鍙粠 github.com/plougher/squashfs-tools 鑾峰彇銆?浣跨敤璇存槑涔熷彲浠庡悓涓€绔欑偣鑾峰彇銆?
### 2.1 鎸傝浇閫夐」


===================    =========================================================
errors=%s              Specify whether squashfs errors trigger a kernel panic
                       or not

		       ==========  =============================================
                         continue  errors don't trigger a panic (default)
                            panic  trigger a panic when errors are encountered,
                                   similar to several other filesystems (e.g.
                                   btrfs, ext4, f2fs, GFS2, jfs, ntfs, ubifs)

                                   This allows a kernel dump to be saved,
                                   useful for analyzing and debugging the
                                   corruption.
                       ==========  =============================================
threads=%s             Select the decompression mode or the number of threads

                       If SQUASHFS_CHOICE_DECOMP_BY_MOUNT is set:

		       ==========  =============================================
                           single  use single-threaded decompression (default)

                                   Only one block (data or metadata) can be
                                   decompressed at any one time. This limits
                                   CPU and memory usage to a minimum, but it
                                   also gives poor performance on parallel I/O
                                   workloads when using multiple CPU machines
                                   due to waiting on decompressor availability.
                            multi  use up to two parallel decompressors per core

                                   If you have a parallel I/O workload and your
                                   system has enough memory, using this option
                                   may improve overall I/O performance. It
                                   dynamically allocates decompressors on a
                                   demand basis.
                           percpu  use a maximum of one decompressor per core

                                   It uses percpu variables to ensure
                                   decompression is load-balanced across the
                                   cores.
                        1|2|3|...  configure the number of threads used for
                                   decompression

                                   The upper limit is num_online_cpus() * 2.
                       ==========  =============================================

                       If SQUASHFS_CHOICE_DECOMP_BY_MOUNT is **not** set and
                       SQUASHFS_DECOMP_MULTI, SQUASHFS_MOUNT_DECOMP_THREADS are
                       both set:

		       ==========  =============================================
                          2|3|...  configure the number of threads used for
                                   decompression

                                   The upper limit is num_online_cpus() * 2.
                       ==========  =============================================

===================    =========================================================

### 3. Squashfs 鏂囦欢绯荤粺璁捐


涓€涓?squashfs 鏂囦欢绯荤粺鏈€澶氱敱涔濅釜閮ㄥ垎缁勬垚锛屼竴璧锋墦鍖呭湪

```
	 ---------------
	|  superblock 	|
	|---------------|
	|  compression  |
	|    options    |
	|---------------|
	|  datablocks   |
	|  & fragments  |
	|---------------|
	|  inode table	|
	|---------------|
	|   directory	|
	|     table     |
	|---------------|
	|   fragment	|
	|    table      |
	|---------------|
	|    export     |
	|    table      |
	|---------------|
	|    uid/gid	|
	|  lookup table	|
	|---------------|
	|     xattr     |
	|     table	|
	 ---------------
```

鍘嬬缉鏁版嵁鍧楀湪浠庢簮鐩綍璇诲彇鏂囦欢鏃惰鍐欏叆鏂囦欢绯荤粺锛屽苟妫€鏌ラ噸澶嶉」銆備竴鏃︽墍鏈夋枃浠舵暟鎹?鍐欏叆瀹屾瘯锛屽氨浼氬啓鍏ュ凡瀹屾垚鐨?inode銆佺洰褰曘€乫ragment銆乪xport銆乽id/gid 鏌ユ壘浠ュ強 xattr
琛ㄣ€?
### 3.1 鍘嬬缉閫夐」


鍘嬬缉鍣ㄥ彲浠ラ€夋嫨鎬у湴鏀寔鐗瑰畾浜庡帇缂╃殑閫夐」锛堜緥濡傚瓧鍏稿ぇ灏忥級銆傚鏋滀娇鐢ㄤ簡闈為粯璁ょ殑
鍘嬬缉閫夐」锛屽垯杩欎簺閫夐」瀛樺偍浜庢銆?
### 3.2 Inodes


鍏冩暟鎹紙inode 涓庣洰褰曪級浠?8K 瀛楄妭鍧椾负鍗曚綅鍘嬬缉銆傛瘡涓帇缂╁潡鍓嶉潰鏈変竴涓袱瀛楄妭鐨勯暱搴︼紝
濡傛灉璇ュ潡鏈鍘嬬缉鍒欐渶楂樹綅缃綅銆傚鏋滆缃簡 -noI 閫夐」锛屾垨鑰呭帇缂╁悗鐨勫潡澶т簬鏈帇缂╃殑
鍧楋紝鍒欒鍧椾笉浼氳鍘嬬缉銆?
inode 琚墦鍖呰繘鍏冩暟鎹潡涓紝骞朵笖涓嶄笌鍧楄竟鐣屽榻愶紝鍥犳 inode 浼氶噸鍙犲湪鍘嬬缉鍧椾笂銆俰node
鐢变竴涓?48 浣嶆暟瀛楁爣璇嗭紝璇ユ暟瀛楃紪鐮佷簡鍖呭惈璇?inode 鐨勫帇缂╁厓鏁版嵁鍧楃殑浣嶇疆锛屼互鍙婅
inode 鍦ㄨ鍧椾腑鐨勫瓧鑺傚亸绉伙紙<block, offset>锛夈€?
涓轰簡鏈€澶у寲鍘嬬缉锛岄拡瀵规瘡绉嶆枃浠剁被鍨嬶紙甯歌鏂囦欢銆佺洰褰曘€佽澶囩瓑锛夋湁涓嶅悓鐨?inode锛屽叾
鍐呭涓庨暱搴﹂殢绫诲瀷鑰屽彉銆?
涓轰簡杩涗竴姝ユ渶澶у寲鍘嬬缉锛屽畾涔変簡涓ょ被甯歌鏂囦欢 inode 鍜岀洰褰?inode锛氶拡瀵归绻佸嚭鐜扮殑
甯歌鏂囦欢鍜岀洰褰曚紭鍖栫殑 inode锛屼互鍙婇渶瑕佸瓨鍌ㄩ澶栦俊鎭殑鎵╁睍绫诲瀷銆?
### 3.3 鐩綍


涓?inode 绫讳技锛岀洰褰曡鎵撳寘杩涘帇缂╃殑鍏冩暟鎹潡涓紝瀛樺偍鍦ㄧ洰褰曡〃閲屻€傜洰褰曢€氳繃鍖呭惈璇?鐩綍鐨?metablock 鐨勮捣濮嬪湴鍧€浠ュ強杩涘叆瑙ｅ帇鍚庡潡鐨勫亸绉绘潵璁块棶锛?block, offset>锛夈€?
鐩綍鐨勭粍缁囨柟寮忕暐寰鏉傦紝骞堕潪绠€鍗曠殑鏂囦欢鍚嶅垪琛ㄣ€傝繖绉嶇粍缁囨柟寮忓埄鐢ㄤ簡浠ヤ笅浜嬪疄锛?锛堝湪澶у鏁版儏鍐典笅锛夋枃浠剁殑 inode 浼氫綅浜庡悓涓€涓帇缂╁厓鏁版嵁鍧椾腑锛屽洜姝ゅ彲鍏变韩璧峰鍧椼€?浜庢槸鐩綍浠ヤ袱绾у垪琛ㄧ粍缁囷細涓€涓洰褰曞ご鍖呭惈鍏变韩鐨勮捣濮嬪潡鍊硷紝鍚庤窡涓€绯诲垪鐩綍椤癸紝姣忛」
鍏变韩璇ヨ捣濮嬪潡銆備竴鏃?inode 璧峰鍧楀彂鐢熷彉鍖栵紝灏变細鍐欏叆涓€涓柊鐨勭洰褰曞ご銆傜洰褰曞ご/鐩綍椤?鍒楄〃鎸夐渶閲嶅澶氭銆?
鐩綍鏄湁搴忕殑锛屽苟涓斿彲浠ュ寘鍚洰褰曠储寮曚互鍔犻€熸枃浠舵煡鎵俱€傜洰褰曠储寮曚负姣忎釜 metablock 瀛樺偍
涓€涓潯鐩紝姣忎釜鏉＄洰瀛樺偍璇ュ厓鏁版嵁鍧椾腑绗竴涓洰褰曞ご鐨勭储寮?鏂囦欢鍚嶆槧灏勩€傜洰褰曟寜瀛楁瘝椤哄簭
鎺掑簭锛屾煡鎵炬椂绾挎€ф壂鎻忕储寮曪紝瀵绘壘绗竴涓瓧姣嶉『搴忓ぇ浜庤鏌ユ壘鏂囦欢鍚嶇殑鏂囦欢鍚嶃€傛鏃跺氨
鎵惧埌浜嗘枃浠跺悕鎵€鍦ㄥ厓鏁版嵁鍧楃殑浣嶇疆銆傜储寮曠殑鎬讳綋鎬濊矾鏄細鏃犺鐩綍澶氶暱锛屾煡鎵鹃兘鍙渶瑙ｅ帇
涓€涓厓鏁版嵁鍧椼€傝鏂规鐨勪紭鍔垮湪浜庝笉闇€瑕侀澶栫殑鍐呭瓨寮€閿€锛屼篃涓嶉渶瑕佺鐩樹笂杩囧鐨勯澶?瀛樺偍銆?
### 3.4 鏂囦欢鏁版嵁


甯歌鏂囦欢鐢变竴涓茶繛缁殑鍘嬬缉鍧楀拰/鎴栦竴涓帇缂╃殑 fragment 鍧楋紙灏鹃儴鎵撳寘鍧楋級缁勬垚銆傛瘡涓?鏁版嵁鍧楃殑鍘嬬缉澶у皬瀛樺偍鍦ㄦ枃浠?inode 鍐呯殑鍧楀垪琛ㄤ腑銆?
涓轰簡鍦ㄨ鍙栤€滃ぇ鈥濇枃浠讹紙256 鍏嗗瓧鑺傛垨鏇村ぇ锛夋椂鍔犻€熷鏁版嵁鍧楃殑璁块棶锛屼唬鐮佸疄鐜颁簡涓€涓储寮?缂撳瓨锛岀紦瀛樹粠鍧楃储寮曞埌纾佺洏涓婃暟鎹潡浣嶇疆鐨勬槧灏勩€?
璇ョ储寮曠紦瀛樹娇 Squashfs 鑳藉澶勭悊澶ф枃浠讹紙鏈€澶?1.75 TiB锛夛紝鍚屾椂鍦ㄧ鐩樹笂淇濈暀绠€鍗曚笖
鑺傜渷绌洪棿鐨勫潡鍒楄〃銆傜紦瀛樿鍒掑垎涓哄涓Ы浣嶏紝鏈€澶氬彲缂撳瓨 8 涓?224 GiB 鐨勬枃浠讹紙128 KiB
鍧楋級銆傛洿澶х殑鏂囦欢浣跨敤澶氫釜妲戒綅锛?.75 TiB 鐨勬枃浠朵細浣跨敤鍏ㄩ儴 8 涓Ы浣嶃€傜储寮曠紦瀛樿璁捐
涓哄唴瀛橀珮鏁堬紝榛樿浣跨敤 16 KiB銆?
### 3.5 Fragment 鏌ユ壘琛?

甯歌鏂囦欢鍙互鍖呭惈涓€涓?fragment 绱㈠紩锛岃绱㈠紩閫氳繃 fragment 鏌ユ壘琛ㄦ槧灏勫埌纾佺洏涓婄殑
fragment 浣嶇疆涓庡帇缂╁ぇ灏忋€傝 fragment 鏌ユ壘琛ㄦ湰韬互鍘嬬缉褰㈠紡瀛樺偍鍦ㄥ厓鏁版嵁鍧椾腑銆備娇鐢?绗簩涓储寮曡〃鏉ュ畾浣嶅畠浠€傚嚭浜庤闂€熷害锛堜笖鍥犲叾杈冨皬锛夎€冭檻锛岃繖绗簩涓储寮曡〃鍦ㄦ寕杞芥椂
琚鍙栧苟缂撳瓨鍦ㄥ唴瀛樹腑銆?
### 3.6 Uid/gid 鏌ユ壘琛?

涓轰簡鑺傜渷绌洪棿锛屽父瑙勬枃浠跺瓨鍌?uid 鍜?gid 绱㈠紩锛岃繖浜涚储寮曢€氳繃涓€涓?id 鏌ユ壘琛ㄨ浆鎹负
32 浣?uid/gid銆傝琛ㄤ互鍘嬬缉褰㈠紡瀛樺偍鍦ㄥ厓鏁版嵁鍧椾腑銆備娇鐢ㄧ浜屼釜绱㈠紩琛ㄦ潵瀹氫綅瀹冧滑銆傚嚭浜?璁块棶閫熷害锛堜笖鍥犲叾杈冨皬锛夎€冭檻锛岃繖绗簩涓储寮曡〃鍦ㄦ寕杞芥椂琚鍙栧苟缂撳瓨鍦ㄥ唴瀛樹腑銆?
### 3.7 Export 琛?

涓轰簡浣?Squashfs 鏂囦欢绯荤粺鍙鍑猴紙閫氳繃 NFS 绛夛級锛屾枃浠剁郴缁熷彲浠ュ彲閫夊湴锛堥€氳繃 -no-exports
Mksquashfs 閫夐」绂佺敤锛夊寘鍚竴涓?inode 鍙峰埌 inode 纾佺洏浣嶇疆鐨勬煡鎵捐〃銆傝繖鏄负浜嗕娇
Squashfs 鑳藉灏嗘枃浠跺彞鏌勪腑浼犲叆鐨?inode 鍙锋槧灏勫埌纾佺洏涓婄殑 inode 浣嶇疆锛岃€岃繖鍦ㄥ鍑轰唬鐮?閲嶆柊瀹炰緥鍖栬繃鏈?琚埛鍑虹殑 inode 鏃舵槸蹇呴渶鐨勩€?
璇ヨ〃浠ュ帇缂╁舰寮忓瓨鍌ㄥ湪鍏冩暟鎹潡涓€備娇鐢ㄧ浜屼釜绱㈠紩琛ㄦ潵瀹氫綅瀹冧滑銆傚嚭浜庤闂€熷害锛堜笖鍥犲叾
杈冨皬锛夎€冭檻锛岃繖绗簩涓储寮曡〃鍦ㄦ寕杞芥椂琚鍙栧苟缂撳瓨鍦ㄥ唴瀛樹腑銆?
### 3.8 Xattr 琛?

xattr 琛ㄥ寘鍚瘡涓?inode 鐨勬墿灞曞睘鎬с€傛瘡涓?inode 鐨?xattr 瀛樺偍鍦ㄤ竴涓垪琛ㄤ腑锛屾瘡涓垪琛?鏉＄洰鍖呭惈绫诲瀷銆佸悕绉板拰鍊煎瓧娈点€傜被鍨嬪瓧娈电紪鐮佷簡 xattr 鍓嶇紑锛堚€渦ser.鈥濄€佲€渢rusted.鈥?绛夛級锛?鍚屾椂涔熺紪鐮佷簡鍚嶇О/鍊煎瓧娈靛簲濡備綍瑙ｉ噴銆傜洰鍓嶈绫诲瀷鎸囩ず鍊兼槸鍐呰仈瀛樺偍鐨勶紙姝ゆ椂鍊煎瓧娈靛寘鍚?xattr 鍊硷級锛岃繕鏄鑱斿瓨鍌ㄧ殑锛堟鏃跺€煎瓧娈靛瓨鍌ㄥ瀹為檯鍊煎瓨鍌ㄤ綅缃殑寮曠敤锛夈€傝繖浣垮緱澶у€煎彲浠?澶栬仈瀛樺偍锛屼粠鑰屾彁鍗囨壂鎻忎笌鏌ユ壘鎬ц兘锛屽苟涓斾篃鍏佽鍊艰鍘婚噸鈥斺€斿€煎彧瀛樺偍涓€娆★紝鎵€鏈夊叾浠栧嚭鐜?澶勬寔鏈夊璇ュ€肩殑澶栬仈寮曠敤銆?
xattr 鍒楄〃琚墦鍖呰繘鍘嬬缉鐨?8K 鍏冩暟鎹潡涓€備负浜嗗噺灏?inode 涓殑寮€閿€锛宨node 鍐呭苟涓嶅瓨鍌?xattr 鍒楄〃鐨勭鐩樹綅缃紝鑰屾槸瀛樺偍涓€涓?32 浣嶇殑 xattr id銆傝 xattr id 閫氳繃绗簩涓?xattr id
鏌ユ壘琛ㄦ槧灏勫埌 xattr 鍒楄〃鐨勪綅缃€?
### 4. TODO 涓庢湭鍐抽棶棰?

### 4.1 TODO 鍒楄〃


瀹炵幇 ACL 鏀寔銆?
### 4.2 Squashfs 鍐呴儴缂撳瓨


Squashfs 涓殑鍧楁槸鍘嬬缉鐨勩€備负浜嗛伩鍏嶅弽澶嶈В鍘嬫渶杩戣闂殑鏁版嵁锛孲quashfs 浣跨敤浜嗕袱涓皬鍨?鐨勫厓鏁版嵁涓?fragment 缂撳瓨銆?
璇ョ紦瀛樹笉鐢ㄤ簬鏂囦欢鏁版嵁鍧楋紝鏂囦欢鏁版嵁鍧椾互甯歌鏂瑰紡瑙ｅ帇骞剁紦瀛樺湪椤电紦瀛橈紙page-cache锛変腑銆?璇ョ紦瀛樼敤浜庝复鏃剁紦瀛樼敱浜庡厓鏁版嵁锛堝嵆 inode 鎴栫洰褰曪級鎴?fragment 璁块棶鑰岃璇诲彇鐨?fragment
涓庡厓鏁版嵁鍧椼€傜敱浜庡厓鏁版嵁涓?fragment 琚墦鍖呭湪涓€璧峰舰鎴愬潡锛堜互鑾峰緱鏇撮珮鐨勫帇缂╃巼锛夛紝璇诲彇
鏌愪竴鐗瑰畾鍏冩暟鎹垨 fragment 鏃朵細涓€骞跺彇鍥炰笌涔嬫墦鍖呭湪涓€璧风殑鍏跺畠鍏冩暟鎹?fragment锛岃€屽熀浜?灞€閮ㄦ€у師鐞嗭紝杩欎簺鍙兘鍦ㄤ笉涔呯殑灏嗘潵琚鍙栥€備复鏃剁紦瀛樺畠浠‘淇濆叾鍦ㄨ繎鏈熻闂椂鍙敤锛岃€?鏃犻渶棰濆鐨勮鍙栦笌瑙ｅ帇銆?
鏈潵杩欎釜鍐呴儴缂撳瓨鍙兘浼氳浣跨敤鍐呮牳椤电紦瀛樼殑瀹炵幇鎵€鍙栦唬銆傜敱浜庨〉缂撳瓨浠ラ〉澶у皬涓烘搷浣滃崟浣嶏紝
杩欏彲鑳戒細鍦ㄩ攣鏈哄埗鍙婄浉鍏崇珵鎬佹潯浠舵柟闈㈠紩鍏ラ澶栫殑澶嶆潅鎬с€?