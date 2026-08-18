
## OCFS2 鏂囦欢绯荤粺


OCFS2 鏄竴涓€氱敤鐨勩€佸熀浜?extent 鐨勫叡浜鐩橀泦缇ゆ枃浠剁郴缁燂紝涓?ext3 鏈夎澶?鐩镐技涔嬪銆傚畠鏀寔 64 浣?inode 鍙凤紝骞朵笖鍏锋湁鑷姩鎵╁睍鐨勫厓鏁版嵁缁勶紝杩欎篃浣垮畠瀵?闈為泦缇や娇鐢ㄩ鍏峰惛寮曞姏銆?
浣犻渶瑕佸畨瑁?ocfs2-tools 杞欢鍖咃紝浠ヤ究鑷冲皯鑾峰緱 "mount.ocfs2" 鍜?"ocfs2_hb_ctl"銆?
Project web page:    http://ocfs2.wiki.kernel.org
Tools git tree:      https://github.com/markfasheh/ocfs2-tools
OCFS2 mailing lists: https://subspace.kernel.org/lists.linux.dev.html

闄ゅ彟鏈夎鏄庡锛屾墍鏈変唬鐮佺増鏉冨綊 2005 Oracle 鎵€鏈夈€?
## 鑷磋阿


澶ч噺浠ｇ爜鍙栬嚜 ext3 鍙婂叾瀹冮」鐩€?
鎸夊瓧姣嶉『搴忔帓鍒楃殑浣滆€咃細

- Joel Becker   <joel.becker@oracle.com>
- Zach Brown    <zach.brown@oracle.com>
- Mark Fasheh   <mfasheh@suse.com>
- Kurt Hackel   <kurt.hackel@oracle.com>
- Tao Ma        <tao.ma@oracle.com>
- Sunil Mushran <sunil.mushran@oracle.com>
- Manish Singh  <manish.singh@oracle.com>
- Tiger Yang    <tiger.yang@oracle.com>

## 娉ㄦ剰浜嬮」

OCFS2 灏氫笉鏀寔鐨勭壒鎬э細

 - 鐩綍鍙樻洿閫氱煡锛團_NOTIFY锛? - 鍒嗗竷寮忕紦瀛橈紙F_SETLEASE/F_GETLEASE/break_lease锛?
## 鎸傝浇閫夐」


OCFS2 鏀寔浠ヤ笅鎸傝浇閫夐」锛?
(*) == 榛樿鍊?
======================= ========================================================
barrier=1		璇ラ€夐」鍚敤/绂佺敤灞忛殰銆俠arrier=0 绂佺敤灞忛殰锛?			barrier=1 鍚敤灞忛殰銆?errors=remount-ro(*)	鍑洪敊鏃跺皢鏂囦欢绯荤粺浠ュ彧璇绘柟寮忛噸鏂版寕杞姐€?errors=panic		鍑洪敊鏃惰Е鍙?panic 骞跺仠鏈恒€?intr		(*)	鍏佽淇″彿涓柇闆嗙兢鎿嶄綔銆?nointr			涓嶅厑璁镐俊鍙蜂腑鏂泦缇ゆ搷浣溿€?noatime			涓嶆洿鏂拌闂椂闂淬€?relatime(*)		鑻ュ厛鍓嶇殑 atime 鏃╀簬 mtime 鎴?ctime 鍒欐洿鏂?atime銆?strictatime		鎬绘槸鏇存柊 atime锛屼絾鏈€灏忔洿鏂伴棿闅旂敱 atime_quantum 鎸囧畾銆?atime_quantum=60(*)	鍦ㄨ绉掓暟杩囧幓涔嬪墠锛孫CFS2 涓嶄細鏇存柊 atime銆?			璁句负 0 鍒欐€绘槸鏇存柊 atime銆傛閫夐」闇€涓?strictatime 閰嶅悎浣跨敤銆?data=ordered	(*)	鍦ㄥ叾鍏冩暟鎹彁浜ゅ埌鏃ュ織涔嬪墠锛屾墍鏈夋暟鎹寮哄埗鐩存帴鍐欏嚭
			鍒颁富鏂囦欢绯荤粺銆?data=writeback		涓嶄繚鐣欐暟鎹『搴忥紝鏁版嵁鍙兘鍦ㄥ叾鍏冩暟鎹彁浜ゅ埌鏃ュ織涔嬪悗
			鎵嶅啓鍏ヤ富鏂囦欢绯荤粺銆?preferred_slot=0(*)	鎸傝浇鏃堕鍏堝皾璇曚娇鐢ㄨ鏂囦欢绯荤粺妲戒綅銆傝嫢瀹冩琚叾瀹冭妭鐐?			浣跨敤锛屽垯閫夋嫨鎵惧埌鐨勭涓€涓┖妲姐€傛棤鏁堝€煎皢琚拷鐣ャ€?commit=nrsec	(*)	鍙互鎸囩ず Ocfs2 姣?'nrsec' 绉掑悓姝ュ叾鎵€鏈夋暟鎹拰鍏冩暟鎹€?			榛樿鍊间负 5 绉掋€傝繖鎰忓懗鐫€鑻ユ帀鐢碉紝浣犳渶澶氬彲鑳戒涪澶辨渶杩?			5 绉掔殑宸ヤ綔锛堜笉杩囨枃浠剁郴缁熶笉浼氭崯鍧忥紝杩欏緱鐩婁簬鏃ュ織锛夈€?			璇ラ粯璁ゅ€硷紙鎴栦换浣曚綆鍊硷級浼氭崯瀹虫€ц兘锛屼絾鏈夊埄浜庢暟鎹?			瀹夊叏銆傚皢鍏惰涓?0 鐨勬晥鏋滀笌淇濇寔榛樿锛? 绉掞級鐩稿悓銆?			灏嗗叾璁句负闈炲父澶х殑鍊间細鎻愬崌鎬ц兘銆?localalloc=8(*)		鍏佽浠?MB 涓哄崟浣嶈嚜瀹氫箟 localalloc 鐨勫ぇ灏忋€傝嫢鍊艰繃澶э紝
			鏂囦欢绯荤粺浼氶潤榛樺湴灏嗗叾鎭㈠涓洪粯璁ゅ€笺€?localflocks		绂佺敤闆嗙兢鎰熺煡鐨?flock銆?inode64			琛ㄧず鍏佽 Ocfs2 鍦ㄦ枃浠剁郴缁熺殑浠绘剰浣嶇疆鍒涘缓 inode锛屽寘鎷?			閭ｄ簺浼氬鑷?inode 鍙峰崰鐢ㄨ秴杩?32 浣嶆湁鏁堜綅鐨勬儏褰€?user_xattr	(*)	鍚敤鎵╁睍鐢ㄦ埛灞炴€с€?nouser_xattr		绂佺敤鎵╁睍鐢ㄦ埛灞炴€с€?acl			鍚敤 POSIX 璁块棶鎺у埗鍒楄〃鏀寔銆?noacl		(*)	绂佺敤 POSIX 璁块棶鎺у埗鍒楄〃鏀寔銆?resv_level=2	(*)	璁剧疆鍒嗛厤棰勭暀鐨勬縺杩涚▼搴︺€傛湁鏁堝€间负 0锛堝叧闂鐣欙級鍒?8
			锛堜负棰勭暀淇濈暀鏈€澶х┖闂达級銆?dir_resv_level=	(*)	榛樿鎯呭喌涓嬶紝鐩綍棰勭暀浼氶殢鏂囦欢棰勭暀缂╂斁鈥斺€旂敤鎴峰緢灏戦渶瑕?			鏇存敼姝ゅ€笺€傝嫢鍒嗛厤棰勭暀琚叧闂紝姝ら€夐」灏嗕笉璧蜂綔鐢ㄣ€?coherency=full  (*)	绂佹骞跺彂鐨?O_DIRECT 鍐欏叆锛屽皢鑾峰彇闆嗙兢 inode 閿佷互寮哄埗
			鍏跺畠鑺傜偣涓㈠純缂撳瓨锛屽洜姝ゅ嵆浣垮浜?O_DIRECT 鍐欏叆涔熻兘
			淇濊瘉瀹屾暣鐨勯泦缇や竴鑷存€с€?coherency=buffered	鍏佽鑺傜偣闂存棤闇€ EX 閿佺殑骞跺彂 O_DIRECT 鍐欏叆锛屼互杈冮珮鎬ц兘
			涓轰唬浠凤紝浣嗗彲鑳藉湪鍏跺畠鑺傜偣涓婅鍒伴檲鏃ф暟鎹€?journal_async_commit	鎻愪氦鍧楀彲浠ュ湪涓嶇瓑寰呮弿杩扮鍧楃殑鎯呭喌涓嬪啓鍏ョ鐩樸€傝嫢鍚敤锛?			鏃у唴鏍稿皢鏃犳硶鎸傝浇璇ヨ澶囥€傝繖浼氬湪鍐呴儴鍚敤 'journal_checksum'銆?======================= ========================================================
