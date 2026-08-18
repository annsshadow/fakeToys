## VFAT


## 浣跨敤 VFAT


```
  mount -t vfat /dev/fd0 /mnt
```
涓嶉渶瑕佺壒娈婄殑鍒嗗尯鏍煎紡鍖栧伐鍏凤紝濡傛灉浣犳兂鍦?Linux 鍐呴儴鏍煎紡鍖栵紝`mkdosfs` 灏卞彲浠ュ緢濂藉湴宸ヤ綔銆?

## VFAT 鎸傝浇閫夐」


**uid=###**
	璁剧疆姝ゆ枃浠剁郴缁熶腑鎵€鏈夋枃浠剁殑鎷ユ湁鑰呫€?
	榛樿鍊兼槸褰撳墠杩涚▼鐨?uid銆?

**gid=###**
	璁剧疆姝ゆ枃浠剁郴缁熶腑鎵€鏈夋枃浠剁殑缁勩€?
	榛樿鍊兼槸褰撳墠杩涚▼鐨?gid銆?

**umask=###**
	鏉冮檺鎺╃爜锛堥拡瀵规枃浠朵笌鐩綍锛屽弬瑙?**umask(1)**锛夈€?
	榛樿鍊兼槸褰撳墠杩涚▼鐨?umask銆?

**dmask=###**
	閽堝鐩綍鐨勬潈闄愭帺鐮併€?
	榛樿鍊兼槸褰撳墠杩涚▼鐨?umask銆?

**fmask=###**
	閽堝鏂囦欢鐨勬潈闄愭帺鐮併€?
	榛樿鍊兼槸褰撳墠杩涚▼鐨?umask銆?

**allow_utime=###**
	璇ラ€夐」鎺у埗瀵?mtime/atime 鐨勬潈闄愭鏌ャ€?

		**-20**: 濡傛灉褰撳墠杩涚▼灞炰簬鏂囦欢缁?ID 鎵€鍦ㄧ殑缁勶紝
                浣犲氨鍙互鏇存敼鏃堕棿鎴炽€?

		**-2**: 鍏朵粬鐢ㄦ埛鍙互鏇存敼鏃堕棿鎴炽€?

	榛樿鐢?dmask 閫夐」璁剧疆銆傚鏋滅洰褰曞彲鍐欙紝涔熷厑璁?utime(2)銆傚嵆 ~dmask & 022銆?

	閫氬父 utime(2) 浼氭鏌ュ綋鍓嶈繘绋嬫槸鍚︿负鏂囦欢鎷ユ湁鑰咃紝鎴栬€呮槸鍚﹀叿鏈?CAP_FOWNER 鑳藉姏銆備絾 FAT 鏂囦欢绯荤粺纾佺洏涓婃病鏈?uid/gid锛屾墍浠ュ父瑙勬鏌ヨ繃浜庡兊鍖栥€傞€氳繃璇ラ€夐」鍙互鏀惧瀹冦€?

**codepage=###**
	璁剧疆鐢ㄤ簬鍦?FAT 鏂囦欢绯荤粺涓婅浆鎹负鐭枃浠跺悕瀛楃鐨?codepage 缂栧彿銆?
	榛樿浣跨敤 FAT_DEFAULT_CODEPAGE 璁剧疆銆?

**iocharset=<name>**
	鐢ㄤ簬杞崲鐢ㄦ埛鍙鏂囦欢鍚嶆墍鐢ㄧ紪鐮佷笌 16 浣?Unicode 瀛楃涔嬮棿
	鐨勫瓧绗﹂泦銆傞暱鏂囦欢鍚嶄互 Unicode 鏍煎紡瀛樺偍鍦ㄧ鐩樹笂锛屼絾 Unix 鍦?
	寰堝ぇ绋嬪害涓婁笉鐭ラ亾濡備綍澶勭悊 Unicode銆?
	榛樿浣跨敤 FAT_DEFAULT_IOCHARSET 璁剧疆銆?

	涔熸湁涓€涓娇鐢?utf8 閫夐」杩涜 UTF-8 杞崲鐨勯€夋嫨銆?

	  鏀逛负浣跨敤 utf8 閫夐」銆?

**utf8=<bool>**
	UTF-8 鏄帶鍒跺彴鎵€浣跨敤鐨勩€佸鏂囦欢绯荤粺瀹夊叏鐨?Unicode 鐗堟湰銆?
	鍙互閫氳繃璇ラ€夐」涓烘枃浠剁郴缁熷惎鐢ㄦ垨绂佺敤瀹冦€?
	濡傛灉璁剧疆浜?'uni_xlate'锛孶TF-8 浼氳绂佺敤銆?
	榛樿浣跨敤 FAT_DEFAULT_UTF8 璁剧疆銆?

**uni_xlate=<bool>**
	灏嗘湭澶勭悊鐨?Unicode 瀛楃杞崲涓虹壒娈婄殑杞箟搴忓垪銆傝繖鍙互璁╀綘
	澶囦唤骞舵仮澶嶄娇鐢ㄤ换浣?Unicode 瀛楃鍒涘缓鐨勬枃浠跺悕銆傚湪 Linux 鐪熸
	鏀寔 Unicode 涔嬪墠锛岃繖缁欎簡浣犱竴绉嶆浛浠ｆ柟妗堛€傛病鏈夎閫夐」鏃讹紝鍦?
	鏃犳硶杩涜杞崲鏃朵細浣跨敤 '?'銆傝浆涔夊瓧绗︽槸 ':'锛屽洜涓鸿瀛楃鍦?
	vfat 鏂囦欢绯荤粺涓婃槸闈炴硶鐨勩€傛墍浣跨敤鐨勮浆涔夊簭鍒楁槸 ':' 鍔犲洓涓?
	鍗佸叚杩涘埗鏁板瓧琛ㄧず鐨?unicode銆?

**nonumtail=<bool>**
	鍦ㄥ垱寤?8.3 鍒悕鏃讹紝鍒悕閫氬父浠?'~1' 鎴栨尝娴彿鍚庤窡鏌愪釜鏁板瓧缁撳熬銆?
	濡傛灉璁剧疆浜嗚閫夐」锛岄偅涔堝綋鏂囦欢鍚嶄负 "longfilename.txt" 涓?
	"longfile.txt" 褰撳墠鍦ㄧ洰褰曚腑涓嶅瓨鍦ㄦ椂锛宻hort 鍒悕灏嗘槸 longfile.txt
	鑰岄潪 longfi~1.txt銆?

**usefree**
	浣跨敤瀛樺偍鍦?FSINFO 涓婄殑 鈥渇ree clusters鈥濓紙绌洪棽绨囷級鍊笺€傚畠灏?
	鐢ㄤ簬纭畾绌洪棽绨囩殑鏁伴噺鑰屾棤闇€鎵弿纾佺洏銆備絾榛樿涓嶄娇鐢紝鍥犱负
	鏈€杩戠殑 Windows 鍦ㄦ煇浜涙儏鍐典笅涓嶈兘姝ｇ‘鏇存柊瀹冦€傚鏋滀綘纭畾 FSINFO 涓婄殑
	鈥渇ree clusters鈥?鏄纭殑锛岄€氳繃璇ラ€夐」鍙互閬垮厤鎵弿纾佺洏銆?

**quiet**
	鍋滄鎵撳嵃鏌愪簺璀﹀憡娑堟伅銆?

**check=s|r|n**
	澶у皬鍐欐晱鎰熸鏌ヨ缃€?

	**s**: 涓ユ牸锛坰trict锛夛紝澶у皬鍐欐晱鎰?

	**r**: 瀹芥澗锛坮elaxed锛夛紝澶у皬鍐欎笉鏁忔劅

	**n**: 鏅€氾紙normal锛夛紝榛樿璁剧疆锛岀洰鍓嶅ぇ灏忓啓涓嶆晱鎰?

**nocase**
	璇ラ€夐」鍦?vfat 涓凡寮冪敤銆傝鏀圭敤 `shortname=win95`銆?

**shortname=lower|win95|winnt|mixed**
	鐭枃浠跺悕鏄剧ず/鍒涘缓璁剧疆銆?

	**lower**: 鏄剧ず鏃惰浆鎹负灏忓啓锛屽垱寤烘椂妯℃嫙 Windows 95 瑙勫垯銆?

	**win95**: 鏄剧ず/鍒涘缓鏃舵ā鎷?Windows 95 瑙勫垯銆?

	**winnt**: 鏄剧ず/鍒涘缓鏃舵ā鎷?Windows NT 瑙勫垯銆?

	**mixed**: 鏄剧ず鏃舵ā鎷?Windows NT 瑙勫垯锛屽垱寤烘椂妯℃嫙 Windows 95 瑙勫垯銆?

	榛樿璁剧疆涓?`mixed`銆?

**tz=UTC**
	灏嗘椂闂存埑瑙ｉ噴涓?UTC 鑰岄潪鏈湴鏃堕棿銆?
	璇ラ€夐」绂佺敤浜嗘湰鍦版椂闂达紙Windows 鍦?FAT 涓婁娇鐢ㄧ殑锛変笌 UTC
	锛圠inux 鍐呴儴浣跨敤鐨勶級涔嬮棿鐨勬椂闂存埑杞崲銆傝繖鍦ㄦ寕杞借璁剧疆涓?UTC 鐨?
	璁惧锛堝鏁扮爜鐩告満锛夋椂鐗瑰埆鏈夌敤锛屽彲浠ラ伩鍏嶆湰鍦版椂闂村甫鏉ョ殑闄烽槺銆?

**time_offset=minutes**
	璁剧疆浠?FAT 浣跨敤鐨勬湰鍦版椂闂磋浆鎹㈠埌 UTC 鐨勫亸绉婚噺銆傚嵆姣忎釜鏃堕棿鎴充細
	鍑忓幓 <minutes> 鍒嗛挓锛岃浆鎹负 Linux 鍐呴儴浣跨敤鐨?UTC銆傚綋 `sys_tz` 涓?
	璁剧疆鐨勬椂鍖轰笉鏄枃浠剁郴缁熸墍浣跨敤鐨勬椂鍖烘椂杩欏緢鏈夌敤銆傛敞鎰忥紝鍦ㄥ瓨鍦?
	DST锛堝浠ゆ椂锛夌殑鎯呭喌涓嬶紝璇ラ€夐」浠嶇劧涓嶈兘鍦ㄦ墍鏈夋儏鍐典笅鎻愪緵姝ｇ‘鐨?
	鏃堕棿鎴斥€斺€斿浜庝笉鍚?DST 璁剧疆涓嬬殑鏃堕棿鎴充細鍋忓樊涓€灏忔椂銆?

**showexec**
	濡傛灉璁剧疆锛屽垯鍙湁褰撳悕绉扮殑鎵╁睍閮ㄥ垎涓?.EXE銆?COM 鎴?.BAT 鏃讹紝
	鏂囦欢鐨勬墽琛屾潈闄愪綅鎵嶈鍏佽銆傞粯璁や笉璁剧疆銆?

**debug**
	鍙互璁剧疆锛屼絾褰撳墠瀹炵幇涓湭浣跨敤銆?

**sys_immutable**
	濡傛灉璁剧疆锛孎AT 涓婄殑 ATTR_SYS 灞炴€т細琚綋浣?Linux 涓婄殑 IMMUTABLE 鏍囧織澶勭悊銆傞粯璁や笉璁剧疆銆?

**flush**
	濡傛灉璁剧疆锛屾枃浠剁郴缁熶細姣旀甯告儏鍐垫洿鏃╁皾璇曞埛鏂板埌纾佺洏銆傞粯璁や笉璁剧疆銆?

**rodir**
	FAT 鍏锋湁 ATTR_RO锛堝彧璇伙級灞炴€с€傚湪 Windows 涓婏紝鐩綍鐨?ATTR_RO
	浼氳蹇界暐锛屼粎琚簲鐢ㄧ▼搴忕敤浣滀竴涓爣蹇楋紙渚嬪锛屽畠涓鸿嚜瀹氫箟鏂囦欢澶硅€岃缃級銆?

	濡傛灉浣犳兂鎶?ATTR_RO 浣滀负鍙鏍囧織鐢ㄤ簬鐩綍锛岃璁剧疆璇ラ€夐」銆?

**errors=panic|continue|remount-ro**
	鎸囧畾 FAT 鍦ㄩ亣鍒颁弗閲嶉敊璇椂鐨勮涓猴細panic锛堟亹鎱岋級銆乧ontinue锛堜笉鍋氫换浣曞鐞嗙户缁級
	杩樻槸浠ュ彧璇绘ā寮忛噸鏂版寕杞藉垎鍖猴紙榛樿琛屼负锛夈€?

**discard**
	濡傛灉璁剧疆锛屽綋鍧楄閲婃斁鏃跺悜鍧楄澶囧彂鍑?discard/TRIM 鍛戒护銆傝繖瀵?SSD 璁惧
	浠ュ強绋€鐤?绮剧畝閰嶇疆鐨?LUN 寰堟湁鐢ㄣ€?

**nfs=stale_rw|nostale_ro**
	浠呭綋浣犳兂瑕侀€氳繃 NFS 瀵煎嚭 FAT 鏂囦欢绯荤粺鏃跺惎鐢ㄦ閫夐」銆?

		**stale_rw**: 璇ラ€夐」缁存姢涓€涓寜 **i_logstart** 绱㈠紩锛堢紦瀛橈級鐨勭洰褰?
		**inode**锛孨FS 鐩稿叧浠ｇ爜鐢ㄥ畠鏉ユ敼鍠勬煡鎵俱€傛敮鎸侀€氳繃 NFS 鐨勫畬鏁存枃浠?
		鎿嶄綔锛堣/鍐欙級锛屼絾鐢变簬 NFS 鏈嶅姟鍣ㄤ笂鐨勭紦瀛橀┍閫愶紝杩欏彲鑳藉鑷?ESTALE 闂銆?

		**nostale_ro**: 璇ラ€夐」灏?**inode** 鍙蜂笌鏂囦欢鍙ユ焺寤虹珛鍦?MS-DOS 鐩綍椤?
		涓枃浠跺湪纾佺洏涓婄殑浣嶇疆涔嬩笂銆傝繖纭繚浜嗘枃浠朵粠 inode 缂撳瓨涓椹遍€愬悗
		涓嶄細杩斿洖 ESTALE銆傜劧鑰岋紝杩欐剰鍛崇潃 rename銆乧reate 涓?unlink 绛夋搷浣?
		鍙兘瀵艰嚧鍏堝墠鎸囧悜鏌愪釜鏂囦欢鐨勬枃浠跺彞鏌勬寚鍚戝彟涓€涓枃浠讹紝娼滃湪鍦伴€犳垚鏁版嵁鎹熷潖銆?
		鍥犳锛岃閫夐」涔熶細浠ュ彧璇绘柟寮忔寕杞芥枃浠剁郴缁熴€?

	涓轰簡淇濇寔鍚戝悗鍏煎锛宍'-o nfs'` 涔熻鎺ュ彈锛岄粯璁や负 "stale_rw"銆?

**dos1xfloppy  <bool>: 0,1,yes,no,true,false**
	濡傛灉璁剧疆锛屼娇鐢ㄧ敱鍚庡璁惧澶у皬鍐冲畾鐨勫洖閫€榛樿 BIOS 鍙傛暟鍧?
	閰嶇疆銆傝繖浜涢潤鎬佸弬鏁板尮閰?DOS 1.x 涓?160 kiB銆?80 kiB銆?20 kiB
	涓?360 kiB 杞洏鍙婅蒋鐩橀暅鍍忔墍鍋囪鐨勯粯璁ゅ€笺€?



## 闄愬埗


鍦ㄤ娇鐢ㄥ甫鏈?FALLOC_FL_KEEP_SIZE 鐨?fallocate 鏃讹紝鏂囦欢鐨?fallocated 鍖哄煙浼氬湪
umount/evict锛堝嵏杞?鍥炴敹锛夋椂琚涪寮冦€傚洜姝わ紝鐢ㄦ埛搴斿綋鍋囪鍦ㄦ湁鍐呭瓨鍘嬪姏瀵艰嚧 inode
浠庡唴瀛樹腑琚洖鏀舵椂锛宖allocated 鍖哄煙鍙兘鍦ㄦ渶鍚庝竴娆″叧闂椂琚涪寮冦€傚洜姝わ紝瀵逛簬浠讳綍
瀵?fallocated 鍖哄煙鐨勪緷璧栵紝鐢ㄦ埛搴斿綋纭繚鍦ㄩ噸鏂版墦寮€鏂囦欢鍚庨噸鏂版鏌?fallocate銆?

## TODO


闇€瑕佸幓鎺夊師濮嬬殑鎵弿浠ｇ爜銆傛敼涓哄缁堜娇鐢ㄨ幏鍙栦笅涓€涓洰褰曢」鐨勬柟寮忋€傜洰鍓嶄粛鍦ㄤ娇鐢?
鍘熷鎵弿鐨勫彧鍓╀笅鐩綍閲嶅懡鍚嶄唬鐮併€?


## 鍙兘瀛樺湪鐨勯棶棰?


- vfat_valid_longname 娌℃湁姝ｇ‘妫€鏌ヤ繚鐣欏悕銆?
- 褰撳嵎鍚嶄笌鏂囦欢绯荤粺鏍圭洰褰曚腑鐨勬煇涓洰褰曞悕鐩稿悓鏃讹紝璇ョ洰褰曞悕鏈夋椂鏄剧ず涓?
  涓€涓┖鏂囦欢銆?
- autoconv 閫夐」涓嶈兘姝ｇ‘宸ヤ綔銆?


## 娴嬭瘯濂椾欢


濡傛灉浣犳墦绠楀 vfat 鏂囦欢绯荤粺鍋氫换浣曚慨鏀癸紝璇疯幏鍙栭殢 vfat 鍙戣鐗堜竴璧锋彁渚涚殑娴嬭瘯濂椾欢锛屽湴鍧€涓?

`<http://web.archive.org/web/*/http://bmrc.berkeley.edu/people/chaffee/vfat.html>`_

璇ュ浠舵祴璇曚簡 vfat 鏂囦欢绯荤粺鐨勭浉褰撳閮ㄥ垎锛屾杩庝负鏂扮殑鎴栨湭缁忔祴璇曠殑鐗规€ф彁渚涢澶栫殑娴嬭瘯銆?

## 鍏充簬 VFAT 鏂囦欢绯荤粺缁撴瀯鐨勮鏄?


鏈枃妗ｇ敱 Galen C. Hunt gchunt@cs.rochester.edu 鎻愪緵锛屽苟缁?Gordon Chaffee 鐣ヤ綔娉ㄨВ銆?

鏈枃妗ｉ潪甯哥矖鐣ャ€佹妧鏈€у湴姒傝堪浜嗘垜瀵?Windows NT 3.5 涓?Windows 95 涓娇鐢ㄧ殑鎵╁睍 FAT
鏂囦欢绯荤粺鐨勪簡瑙ｃ€傛垜涓嶄繚璇佷互涓嬪唴瀹规湁浠讳綍姝ｇ‘鎬э紝浣嗙湅璧锋潵纭疄濡傛銆?

鎵╁睍 FAT 鏂囦欢绯荤粺鍑犱箮涓?DOS锛堝惈 **6.223410239847** 鐗堟湰锛夊強鏇存棭鐗堟湰涓娇鐢ㄧ殑 FAT
鏂囦欢绯荤粺瀹屽叏鐩稿悓 :-)銆傛樉钁楃殑鍙樺寲鏄鍔犱簡闀挎枃浠跺悕銆傝繖浜涘悕瀛楁敮鎸佹渶澶?255 涓瓧绗︼紝
鍖呮嫭绌烘牸涓庡皬鍐欏瓧绗︼紝鑰屼紶缁熺殑 8.3 鐭悕鍒欎笉鐒躲€?

浠ヤ笅鏄綋鍓嶄紶缁?FAT 椤圭殑鎻忚堪锛?
```
        struct directory { // Short 8.3 names
                unsigned char name[8];          // file name
                unsigned char ext[3];           // file extension
                unsigned char attr;             // attribute byte
		unsigned char lcase;		// Case for base and extension
		unsigned char ctime_ms;		// Creation time, milliseconds
		unsigned char ctime[2];		// Creation time
		unsigned char cdate[2];		// Creation date
		unsigned char adate[2];		// Last access date
		unsigned char reserved[2];	// reserved values (ignored)
                unsigned char time[2];          // time stamp
                unsigned char date[2];          // date stamp
                unsigned char start[2];         // starting cluster number
                unsigned char size[4];          // size of the file
        };
```
lcase 瀛楁鎸囧畾 8.3 鍚嶅瓧鐨勫熀鍚嶅拰/鎴栨墿灞曞悕鏄惁搴斿ぇ鍐欍€傝瀛楁浼间箮涓嶈 Windows 95 浣跨敤锛屼絾琚?Windows NT 浣跨敤銆傛枃浠跺悕鐨勫ぇ灏忓啓鍦?Windows NT 鍒?Windows 95 涔嬮棿骞朵笉瀹屽叏鍏煎銆傚弽杩囨潵鏂瑰悜鍚屾牱涓嶅畬鍏ㄥ吋瀹广€傞€傚悎 8.3 鍛藉悕绌洪棿銆佷笖鍦?Windows NT 涓婁互灏忓啓鍐欏叆鐨勬枃浠跺悕锛屽湪 Windows 95 涓婁細鏄剧ず涓哄ぇ鍐欍€?

          瀛楄妭搴忥紙endian锛夋暣鏁板€笺€傝缁撴瀯涓悇瀛楁鐨勬弿杩版槸鍏紑鐭ヨ瘑锛屽彲浠ュ湪鍒鎵惧埌銆?

閫氳繃鎵╁睍 FAT 绯荤粺锛孧icrosoft 涓轰换浣曞叿鏈夋墿灞曞悕鐨勬枃浠舵彃鍏ヤ簡棰濆鐨勭洰褰曢」銆?
锛堜换浣曞悎娉曞湴绗﹀悎鏃?8.3 缂栫爜鏂规鐨勫悕绉版病鏈夐澶栭」銆傦級鎴戠О杩欎簺棰濆椤逛负妲斤紙slot锛夈€?
鍩烘湰涓婏紝涓€涓Ы鏄竴涓壒娈婃牸寮忕殑鐩綍椤癸紝鎸佹湁鏂囦欢鍚嶆墿灞曞悕涓渶澶?13 涓瓧绗︺€傚皢妲?
瑙嗕负涓庡叾瀵瑰簲鐨勬枃浠剁洰褰曢」鐨勯檮鍔犳爣绛俱€侻icrosoft 鍊惧悜浜庡皢鏂囦欢鐨?8.3 椤圭О涓哄叾鍒悕锛坅lias锛夛紝
灏嗘墿灞曟Ы鐩綍椤圭О涓烘枃浠跺悕銆?
```
        struct slot { // Up to 13 characters of a long name
                unsigned char id;               // sequence number for slot
                unsigned char name0_4[10];      // first 5 characters in name
                unsigned char attr;             // attribute byte
                unsigned char reserved;         // always 0
                unsigned char alias_checksum;   // checksum for 8.3 alias
                unsigned char name5_10[12];     // 6 more characters in name
                unsigned char start[2];         // starting cluster number
                unsigned char name11_12[4];     // last 2 characters in name
        };
```
濡傛灉妲界殑甯冨眬鐪嬭捣鏉ユ湁鐐瑰鎬紝閭ｅ彧鏄洜涓?Microsoft 鍔姏淇濇寔涓庢棫杞欢鐨勫吋瀹规€с€傛Ы蹇呴』琚吉瑁呬互闃叉鏃ц蒋浠舵亹鎱屻€備负姝わ紝閲囧彇浜嗕竴浜涙帾鏂斤細

        1) 妲界洰褰曢」鐨勫睘鎬у瓧鑺傛€绘槸璁剧疆涓?0x0f銆傝繖瀵瑰簲浜庝竴涓叿鏈?鈥渉idden鈥濓紙闅愯棌锛夈€?
           鈥渟ystem鈥濓紙绯荤粺锛夈€佲€渞ead-only鈥濓紙鍙锛変笌 鈥渧olume label鈥濓紙鍗锋爣锛夊睘鎬х殑鏃х洰褰曢」銆?
           澶у鏁版棫杞欢浼氬拷鐣ヤ换浣曡缃簡 鈥渧olume label鈥?浣嶇殑鐩綍椤广€傜湡姝ｇ殑鍗锋爣椤?
           涓嶄細璁剧疆鍏朵粬涓変釜浣嶃€?

        2) 璧峰绨囨€绘槸璁剧疆涓?0锛岃繖瀵逛竴涓?DOS 鏂囦欢鏉ヨ鏄笉鍙兘鐨勫彇鍊笺€?

鐢变簬鎵╁睍 FAT 绯荤粺鏄悜鍚庡吋瀹圭殑锛屾棫杞欢鍙兘淇敼鐩綍椤广€傚繀椤婚噰鍙栨帾鏂界‘淇濇Ы鐨勬湁鏁堟€с€傛墿灞?FAT 绯荤粺鍙互閫氳繃濡備笅鏂瑰紡楠岃瘉涓€涓Ы纭疄灞炰簬鏌愪釜 8.3 鐩綍椤癸細

        1) 浣嶇疆銆備竴涓枃浠剁殑妲芥€绘槸绱ф帴鍦ㄥ叾瀵瑰簲鐨?8.3 鐩綍椤逛箣鍓嶃€傛澶栵紝姣忎釜妲?
           閮芥湁涓€涓?id锛屾爣璁板叾鍦ㄦ墿灞曟枃浠跺悕涓殑椤哄簭銆備笅闈㈡槸涓€涓?8.3 鐩綍椤瑰強鍏?
           瀵瑰簲闀垮悕妲介潪甯哥畝鐣ョ殑瑙嗗浘锛岄拡瀵规枃浠?
```
                <proceeding files...>
                <slot #3, id = 0x43, characters = "h is long">
                <slot #2, id = 0x02, characters = "xtension whic">
                <slot #1, id = 0x01, characters = "My Big File.E">
                <directory entry, name = "MYBIGFIL.EXT">
```
           .. note:: 娉ㄦ剰妲芥槸浠庢渶鍚庡埌鏈€鍓嶅瓨鍌ㄧ殑銆傛Ы浠?1 缂栧彿鍒?N銆傜 N 涓Ы涓?
		     0x40 杩涜 ``or`` 杩愮畻浠ユ爣璁颁负鏈€鍚庝竴涓€?

        2) 鏍￠獙鍜屻€傛瘡涓Ы閮芥湁涓€涓?alias_checksum 鍊笺€傛牎楠屽拰浣跨敤濡備笅绠楁硶浠?8.3 鍚嶅瓧璁＄畻鑰屾潵::
```
                for (sum = i = 0; i < 11; i++) {
                        sum = (((sum&1)<<7)|((sum&0xfe)>>1)) + name[i]
                }
```
	3) 濡傛灉鏈€鍚庝竴涓Ы涓湁绌洪棽绌洪棿锛屽湪鏈€鍚庝竴涓瓧绗︿箣鍚庡瓨鍌ㄤ竴涓?Unicode ``NULL (0x0000)``銆?
	   涔嬪悗锛屾渶鍚庝竴涓Ы涓墍鏈夋湭浣跨敤鐨勫瓧绗﹁璁剧疆涓?Unicode 0xFFFF銆?

```
鏈€鍚庯紝娉ㄦ剰鎵╁睍鍚嶆槸浠?Unicode 瀛樺偍鐨勩€傛瘡涓?Unicode 瀛楃鍗犵敤涓や釜鎴栧洓涓瓧鑺傦紝浠?UTF-16LE 缂栫爜銆?
```
