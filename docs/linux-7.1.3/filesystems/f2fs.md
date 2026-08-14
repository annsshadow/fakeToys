
## 闈㈠悜闂瓨鐨勬枃浠剁郴缁燂紙Flash-Friendly File System锛孎2FS锛?


## 姒傝堪


鍩轰簬 NAND 闂瓨鐨勫瓨鍌ㄨ澶囷紝濡?SSD銆乪MMC 涓?SD 鍗★紝宸茶瑁呭鍦ㄤ粠绉诲姩璁惧鍒版湇鍔″櫒绯荤粺鐨勫悇绫荤郴缁熶笂銆傜敱浜庡畠浠凡鐭ュ叿鏈変笉鍚屼簬浼犵粺鏈烘纾佺洏鐨勭壒鎬э紝浣滀负瀛樺偍璁惧涓婂眰缁撴瀯鐨勬枃浠剁郴缁燂紝搴斿綋浠庤璁″眰闈㈣捣灏遍€傚簲杩欎簺鍙樺寲銆?

F2FS 鏄竴涓埄鐢ㄥ熀浜?NAND 闂瓨鐨勫瓨鍌ㄨ澶囩殑鏂囦欢绯荤粺锛屽畠鍩轰簬鏃ュ織缁撴瀯鏂囦欢绯荤粺锛圠og-structured File System锛孡FS锛夈€傚叾璁捐閲嶇偣鍦ㄤ簬瑙ｅ喅 LFS 涓殑鏍规湰闂锛屽嵆娓歌蛋鏍戯紙wandering tree锛夌殑婊氶洩鐞冩晥搴斾笌楂樻槀鐨勬竻鐞嗗紑閿€銆?

鐢变簬鍩轰簬 NAND 闂瓨鐨勫瓨鍌ㄨ澶囦細鍥犲叾鍐呴儴鍑犱綍缁撴瀯鎴栭棯瀛樼鐞嗘柟妗堬紙鍗?FTL锛夌殑涓嶅悓鑰岃〃鐜板嚭涓嶅悓鐗规€э紝F2FS 鍙婂叾宸ュ叿鏀寔鍚勭鍙傛暟锛屼笉浠呯敤浜庨厤缃鐩樹笂鐨勫竷灞€锛屼篃鐢ㄤ簬閫夋嫨鍒嗛厤涓庢竻鐞嗙畻娉曘€?

浠ヤ笅 git 鏍戞彁渚涗簡鏂囦欢绯荤粺鏍煎紡鍖栧伐鍏凤紙mkfs.f2fs锛夈€佷竴鑷存€ф鏌ュ伐鍏凤紙fsck.f2fs锛変互鍙婅皟璇曞伐鍏凤紙dump.f2fs锛夛細

- git://git.kernel.org/pub/scm/linux/kernel/git/jaegeuk/f2fs-tools.git

鎻愪氦琛ヤ竵璇蜂娇鐢ㄤ互涓嬮偖浠跺垪琛細

- linux-f2fs-devel@lists.sourceforge.net

鎶ュ憡 bug 璇蜂娇鐢ㄤ互涓?f2fs bug 璺熻釜閾炬帴锛?

- https://bugzilla.kernel.org/enter_bug.cgi?product=File%20System&component=f2fs

## 鑳屾櫙涓庤璁￠棶棰?


### 鏃ュ織缁撴瀯鏂囦欢绯荤粺锛圠FS锛?


鈥滄棩蹇楃粨鏋勬枃浠剁郴缁熶互绫绘棩蹇楃殑缁撴瀯灏嗘墍鏈変慨鏀归『搴忓啓鍏ョ鐩橈紝浠庤€屽悓鏃跺姞蹇枃浠跺啓鍏ヤ笌宕╂簝鎭㈠銆傛棩蹇楁槸纾佺洏涓婂敮涓€鐨勭粨鏋勶紱瀹冨寘鍚储寮曚俊鎭紝浠ヤ究鏂囦欢鑳藉楂樻晥鍦颁粠鏃ュ織涓鍥炪€備负浜嗙淮鎸佺鐩樹笂鐨勫ぇ鍧楃┖闂插尯鍩熶互鍔犲揩鍐欏叆锛屾垜浠皢鏃ュ織鍒掑垎涓烘锛屽苟浣跨敤娈垫竻鐞嗗櫒锛坰egment cleaner锛夊皢閲嶅害纰庣墖鍖栫殑娈典腑鐨勬湁鏁堜俊鎭帇缂┿€傗€?寮曡嚜 Rosenblum, M. 涓?Ousterhout, J. K., 1992锛屻€婃棩蹇楃粨鏋勬枃浠剁郴缁熺殑璁捐涓庡疄鐜般€嬶紝ACM Trans. Computer Systems 10, 1, 26鈥?2銆?

### 娓歌蛋鏍戦棶棰?


鍦?LFS 涓紝褰撴枃浠舵暟鎹鏇存柊骞跺啓鍏ユ棩蹇楁湯灏炬椂锛屽叾鐩存帴鎸囬拡鍧椾細鍥犱綅缃敼鍙樿€屾洿鏂般€傛帴鐫€闂存帴鎸囬拡鍧椾篃浼氬洜鐩存帴鎸囬拡鍧楃殑鏇存柊鑰屾洿鏂般€備緷姝ょ被鎺紝涓婂眰绱㈠紩缁撴瀯濡?inode銆乮node 鏄犲皠涓庢鏌ョ偣鍧椾篃浼氶€掑綊鍦版洿鏂般€傝繖涓棶棰樿绉颁负娓歌蛋鏍戦棶棰?[^1^]锛屼负浜嗘彁鍗囨€ц兘锛屽簲灏藉彲鑳芥秷闄ゆ垨鏀炬澗杩欑鏇存柊浼犳挱銆?

[^1^] Bityutskiy, A. 2005. JFFS3 design issues. http://www.linux-mtd.infradead.org/

### 娓呯悊寮€閿€


鐢变簬 LFS 鍩轰簬寮傚湴鍐欙紙out-of-place writes锛夛紝瀹冧細浜х敓澶ч噺鏁ｅ竷鍦ㄦ暣涓瓨鍌ㄤ腑鐨勫簾寮冨潡銆備负浜嗘彁渚涙柊鐨勭┖闂叉棩蹇楃┖闂达紝瀹冮渶瑕佹棤缂濆湴鍥炴敹杩欎簺搴熷純鍧椼€傝繖椤瑰伐浣滆绉颁负娓呯悊杩囩▼銆?

璇ヨ繃绋嬬敱濡備笅涓変釜鎿嶄綔缁勬垚銆?

1. 閫氳繃寮曠敤娈典娇鐢ㄨ〃閫夋嫨涓€涓彈瀹宠€呮銆?
2. 瀹冨姞杞藉彈瀹宠€呮涓墍鏈夋暟鎹敱娈垫憳瑕佸潡鏍囪瘑鍑虹殑鐖剁储寮曠粨鏋勩€?
3. 瀹冩鏌ユ暟鎹笌鐖剁储寮曠粨鏋勪箣闂寸殑浜ゅ弶寮曠敤銆?
4. 瀹冩湁閫夋嫨鍦扮Щ鍔ㄦ湁鏁堟暟鎹€?
杩欎竴娓呯悊宸ヤ綔鍙兘瀵艰嚧鎰忓鐨勯暱寤惰繜锛屽洜姝ゆ渶閲嶈鐨勭洰鏍囨槸鍚戠敤鎴烽殣钘忚繖浜涘欢杩熴€傚綋鐒讹紝瀹冭繕搴斿噺灏戦渶瑕佺Щ鍔ㄧ殑鏈夋晥鏁版嵁閲忥紝骞跺揩閫熷湴绉诲姩瀹冧滑銆?

## 鍏抽敭鐗规€?


### 闂瓨鎰熺煡


- 鎵╁ぇ闅忔満鍐欏叆鍖哄煙浠ヨ幏寰楁洿濂界殑鎬ц兘锛屽悓鏃舵彁渚涜緝楂樼殑绌洪棿灞€閮ㄦ€?
- 灏芥渶澶у姫鍔涘皢鏂囦欢绯荤粺鏁版嵁缁撴瀯瀵归綈鍒?FTL 涓殑鎿嶄綔鍗曞厓

### 娓歌蛋鏍戦棶棰?


- 浣跨敤涓€涓湳璇€渘ode锛堣妭鐐癸級鈥濇潵琛ㄧず inode 浠ュ強鍚勭鎸囬拡鍧?
- 寮曞叆鍖呭惈鎵€鏈夆€渘ode鈥濆潡浣嶇疆鐨勮妭鐐瑰湴鍧€琛紙Node Address Table锛孨AT锛夛紱杩欏皢鍒囨柇鏇存柊浼犳挱銆?

### 娓呯悊寮€閿€


- 鏀寔鍚庡彴娓呯悊杩囩▼
- 鏀寔璐績锛坓reedy锛変笌鎴愭湰鏀剁泭锛坈ost-benefit锛夌畻娉曠敤浜庡彈瀹宠€呴€夋嫨绛栫暐
- 鏀寔澶氱澶存棩蹇楃敤浜庨潤鎬?鍔ㄦ€佸喎鐑暟鎹垎绂?
- 寮曞叆鑷€傚簲鏃ュ織锛坅daptive logging锛変互瀹炵幇楂樻晥鍧楀垎閰?

## 鎸傝浇閫夐」
======================== ============================================================
background_gc=%s	 寮€鍚?鍏抽棴鍚庡彴瑙﹀彂锛堝嵆褰?I/O 瀛愮郴缁熺┖闂叉椂锛夌殑娓呯悊鎿嶄綔锛屼篃灏辨槸鍨冨溇鍥炴敹锛坓arbage collection锛夈€傝嫢 background_gc=on锛屽垯寮€鍚瀮鍦惧洖鏀讹紱鑻?background_gc=off锛屽垯鍏抽棴鍨冨溇鍥炴敹銆傝嫢 background_gc=sync锛屽垯寮€鍚湪鍚庡彴杩愯鐨勫悓姝ュ瀮鍦惧洖鏀躲€傝閫夐」鐨勯粯璁ゅ€间负 on锛屽洜姝ら粯璁ゅ紑鍚瀮鍦惧洖鏀躲€?
gc_merge		 褰?background_gc 寮€鍚椂锛屽彲鍚敤姝ら€夐」锛岃鍚庡彴 GC 绾跨▼澶勭悊鍓嶅彴 GC 璇锋眰锛屼粠鑰屾秷闄ゅ綋 GC 鐢?I/O 涓?CPU 璧勬簮鍙楅檺鐨勮繘绋嬭Е鍙戞椂锛岀紦鎱㈢殑鍓嶅彴 GC 鎿嶄綔瀵艰嚧鐨勫崱椤块棶棰樸€?
nogc_merge		 绂佺敤 GC 鍚堝苟鐗规€с€?
disable_roll_forward	 绂佺敤鍓嶆粴锛坮oll-forward锛夋仮澶嶆祦绋嬨€?
norecovery		 绂佺敤鍓嶆粴鎭㈠娴佺▼锛屼互鍙鏂瑰紡鎸傝浇锛堝嵆 -o ro,disable_roll_forward锛夈€?
discard/nodiscard	 鍦?f2fs 涓惎鐢?绂佺敤瀹炴椂涓㈠純锛坉iscard锛夛紱鑻ュ惎鐢?discard锛宖2fs 浼氬湪娓呯悊涓€涓鏃跺彂鍑?discard/TRIM 鍛戒护銆?
heap/no_heap		 宸插簾寮冦€?
nouser_xattr		 绂佺敤鎵╁睍鐢ㄦ埛灞炴€э紙Extended User Attributes锛夈€傛敞鎰忥細鑻ラ€変腑浜?CONFIG_F2FS_FS_XATTR锛屽垯 xattr 榛樿鍚敤銆?
noacl			 绂佺敤 POSIX 璁块棶鎺у埗鍒楄〃锛圓ccess Control List锛夈€傛敞鎰忥細鑻ラ€変腑浜?CONFIG_F2FS_FS_POSIX_ACL锛屽垯 acl 榛樿鍚敤銆?
active_logs=%u		 鏀寔閰嶇疆娲诲姩鏃ュ織鐨勬暟閲忋€傚湪褰撳墠璁捐涓紝f2fs 浠呮敮鎸?2銆? 鍜?6 鏉℃棩蹇椼€傞粯璁ゆ暟閲忎负 6銆?
disable_ext_identify	 绂佺敤鐢?mkfs 閰嶇疆鐨勬墿灞曞悕鍒楄〃锛岃繖鏍?f2fs 灏变笉浼氭劅鐭ュ埌璇稿濯掍綋鏂囦欢涔嬬被鐨勫喎鏂囦欢銆?
inline_xattr		 鍚敤鍐呰仈 xattr锛坕nline xattrs锛夌壒鎬с€?
noinline_xattr		 绂佺敤鍐呰仈 xattr 鐗规€с€?
inline_xattr_size=%u	 鏀寔閰嶇疆鍐呰仈 xattr 澶у皬锛屽畠渚濊禆浜庣伒娲诲唴鑱?xattr 鐗规€с€?
inline_data		 鍚敤鍐呰仈鏁版嵁锛坕nline data锛夌壒鎬э細鏂板垱寤虹殑杈冨皬锛?~3.4k锛夋枃浠跺彲鐩存帴鍐欏叆 inode 鍧椼€?
inline_dentry		 鍚敤鍐呰仈鐩綍锛坕nline dir锛夌壒鎬э細鏂板缓鐩綍椤逛腑鐨勬暟鎹彲鍐欏叆 inode 鍧椼€傜敤浜庡瓨鍌ㄥ唴鑱旂洰褰曢」鐨?inode 鍧楃┖闂翠笂闄愮害涓?3.4k銆?
noinline_dentry		 绂佺敤鍐呰仈 dentry 鐗规€с€?
flush_merge		 灏藉彲鑳藉悎骞跺苟鍙戠殑 cache_flush 鍛戒护锛屼互娑堥櫎鍐椾綑鍛戒护鐨勫彂鍑恒€傚鏋滃簳灞傝澶囧鐞?cache_flush 鍛戒护鐩稿杈冩參锛屽缓璁惎鐢ㄦ閫夐」銆?
nobarrier		 褰撳簳灞傚瓨鍌ㄤ繚璇佸叾缂撳瓨鏁版嵁搴斿啓鍏ラ潪鏄撳け鎬у尯鍩熸椂锛屽彲浣跨敤姝ら€夐」銆傝嫢璁剧疆姝ら€夐」锛屽垯涓嶄細鍙戝嚭 cache_flush 鍛戒护锛屼絾 f2fs 浠嶄繚璇佹墍鏈夋暟鎹啓鍏ョ殑鍐欓『搴忋€?
barrier			 鑻ヨ缃閫夐」锛屽垯鍏佽鍙戝嚭 cache_flush 鍛戒护銆?
fastboot		 褰撶郴缁熷笇鏈涘敖鍙兘鍑忓皯鎸傝浇鏃堕棿銆佸嵆浣跨壓鐗叉甯告€ц兘鏃讹紝浣跨敤姝ら€夐」銆?
extent_cache		 鍚敤鍩轰簬 rb-tree 鐨?extent 缂撳瓨锛屽畠鍙互涓烘瘡涓?inode 缂撳瓨灏藉彲鑳藉鐨勩€佸湪杩炵画閫昏緫鍦板潃涓庣墿鐞嗗湴鍧€涔嬮棿鏄犲皠鐨?extent锛屼粠鑰屾彁楂樼紦瀛樺懡涓巼銆傞粯璁ゅ紑鍚€?
noextent_cache		 鏄惧紡绂佺敤鍩轰簬 rb-tree 鐨?extent 缂撳瓨锛屽弬瑙佷笂闈㈢殑 extent_cache 鎸傝浇閫夐」銆?
noinline_data		 绂佺敤鍐呰仈鏁版嵁鐗规€э紱鍐呰仈鏁版嵁鐗规€ч粯璁ゆ槸鍚敤鐨勩€?
data_flush		 鍦ㄦ鏌ョ偣涔嬪墠鍚敤鏁版嵁鍒锋柊锛屼互鎸佷箙鍖栨櫘閫氭枃浠朵笌绗﹀彿閾炬帴鐨勬暟鎹€?
reserve_root=%d	 鏀寔閰嶇疆淇濈暀绌洪棿锛屼緵鍏锋湁鎸囧畾 uid 鎴?gid 鐨勭壒鏉冪敤鎴疯繘琛屽垎閰嶏紝鍗曚綅锛?KB锛岄粯璁や笂闄愪负鐢ㄦ埛鍧楃殑 12.5%銆?
reserve_node=%d	 鏀寔閰嶇疆淇濈暀鑺傜偣锛屼緵鍏锋湁鎸囧畾 uid 鎴?gid 鐨勭壒鏉冪敤鎴疯繘琛屽垎閰嶏紝榛樿涓婇檺涓烘墍鏈夎妭鐐圭殑 12.5%銆?
resuid=%d		 鍙互浣跨敤淇濈暀鍧椾笌鑺傜偣鐨勭敤鎴?ID銆?
resgid=%d		 鍙互浣跨敤淇濈暀鍧椾笌鑺傜偣鐨勭粍 ID銆?
fault_injection=%d	 浠ユ寚瀹氱殑娉ㄥ叆閫熺巼锛屽湪鎵€鏈夋敮鎸佺殑绫诲瀷涓惎鐢ㄦ晠闅滄敞鍏ャ€?
fault_type=%d		 鏀寔閰嶇疆鏁呴殰娉ㄥ叆绫诲瀷锛屽簲涓?fault_injection 閫夐」涓€璧峰惎鐢紱鏁呴殰绫诲瀷鍊煎涓嬫墍绀猴紝鏀寔鍗曚竴鎴栫粍鍚堢被鍨嬨€?
			 .. code-block:: none
			     ===========================      ==========
			     Type_Name                        Type_Value
			     ===========================      ==========
			     FAULT_KMALLOC                    0x00000001
			     FAULT_KVMALLOC                   0x00000002
			     FAULT_PAGE_ALLOC                 0x00000004
			     FAULT_PAGE_GET                   0x00000008
			     FAULT_ALLOC_BIO                  0x00000010 (obsolete)
			     FAULT_ALLOC_NID                  0x00000020
			     FAULT_ORPHAN                     0x00000040
			     FAULT_BLOCK                       0x00000080
			     FAULT_DIR_DEPTH                  0x00000100
			     FAULT_EVICT_INODE                0x00000200
			     FAULT_TRUNCATE                   0x00000400
			     FAULT_READ_IO                    0x00000800
			     FAULT_CHECKPOINT                 0x00001000
			     FAULT_DISCARD                    0x00002000 (obsolete)
			     FAULT_WRITE_IO                   0x00004000
			     FAULT_SLAB_ALLOC                 0x00008000
			     FAULT_DQUOT_INIT                 0x00010000
			     FAULT_LOCK_OP                    0x00020000
			     FAULT_BLKADDR_VALIDITY           0x00040000
			     FAULT_BLKADDR_CONSISTENCE        0x00080000
			     FAULT_NO_SEGMENT                 0x00100000
			     FAULT_INCONSISTENT_FOOTER        0x00200000
			     FAULT_ATOMIC_TIMEOUT             0x00400000 (1000ms)
			     FAULT_VMALLOC                    0x00800000
			     FAULT_LOCK_TIMEOUT               0x01000000 (1000ms)
			     FAULT_SKIP_WRITE                 0x02000000
			     ===========================      ==========
mode=%s			 鎺у埗鍧楀垎閰嶆ā寮忥紝鏀寔鈥渁daptive鈥濅笌鈥渓fs鈥濄€傚湪鈥渓fs鈥濇ā寮忎笅锛屼笉搴旀湁鏈濆悜涓诲尯鍩熺殑闅忔満鍐欏叆銆?
			 鈥渇ragment:segment鈥濅笌鈥渇ragment:block鈥濇槸鏂板姞鍏ョ殑銆傝繖浜涙槸渚涘疄楠屾ā鎷熸枃浠剁郴缁熺鐗囧寲/GC 鍚庢儏褰㈡湰韬殑寮€鍙戣€呴€夐」銆傚紑鍙戣€呬娇鐢ㄨ繖浜涙ā寮忔潵鏇村ソ鍦扮悊瑙ｆ枃浠剁郴缁熺鐗囧寲/GC 鍚庣殑鐘跺喌锛屽苟鏈€缁堣幏寰楁洿濂界殑澶勭悊鎬濊矾銆?
			 鍦ㄢ€渇ragment:segment鈥濇ā寮忎笅锛宖2fs 浼氬湪闅忔満浣嶇疆鍒嗛厤涓€涓柊娈碉紝鍊熸鍙互妯℃嫙 GC 鍚庣殑鐘跺喌銆?
			 鍦ㄢ€渇ragment:block鈥濇ā寮忎笅锛屾垜浠彲浠ュ€熷姪鈥渕ax_fragment_chunk鈥濅笌鈥渕ax_fragment_hole鈥?sysfs 鑺傜偣鎵撴暎鍧楀垎閰嶃€傛垜浠 chunk 涓?hole 鐨勫ぇ灏忛兘鍔犲叆浜嗕竴浜涢殢鏈烘€э紝浣垮叾鎺ヨ繎鐪熷疄鐨?I/O 妯″紡銆傚洜姝ゅ湪姝ゆā寮忎笅锛宖2fs 浼氬湪涓€涓?chunk 涓垎閰?1..<max_fragment_chunk> 涓潡锛屽苟杞祦杞€屽埗閫犻暱搴︿负 1..<max_fragment_hole> 鐨勭┖娲炪€傝繖鏍凤紝鏂板垎閰嶇殑鍧楀皢鏁ｅ竷鍦ㄦ暣涓垎鍖轰腑銆傛敞鎰忥紝鈥渇ragment:block鈥濅細闅愬紡鍚敤鈥渇ragment:segment鈥濋€夐」浠ヨ幏寰楁洿澶氶殢鏈烘€с€?
			 璇峰皢杩欎簺閫夐」鐢ㄤ簬浣犵殑瀹為獙锛屽苟涓旀垜浠己鐑堝缓璁湪浣跨敤杩欎簺閫夐」鍚庨噸鏂版牸寮忓寲鏂囦欢绯荤粺銆?
usrquota		 鍚敤鏅€氱敤鎴风鐩橀厤棰濊璐︺€?
grpquota		 鍚敤鏅€氱粍纾佺洏閰嶉璁拌处銆?
prjquota		 鍚敤鏅€氶」鐩厤棰濊璐︺€?
usrjquota=<file>	 鍦ㄦ寕杞芥湡闂存寚瀹氱壒瀹氭枃浠朵笌绫诲瀷锛屼互渚块厤棰濅俊鎭兘鍦ㄦ仮澶嶆祦绋嬩腑姝ｇ‘鏇存柊锛?
grpjquota=<file>	 <quota file> 蹇呴』浣嶄簬鏍圭洰褰曪紱
prjjquota=<file>	 jqfmt=<quota type>: <quota type>: [vfsold,vfsv0,vfsv1]銆?
usrjquota=		 鍏抽棴鐢ㄦ埛鏃ュ織閰嶉銆?
grpjquota=		 鍏抽棴缁勬棩蹇楅厤棰濄€?
prjjquota=		 鍏抽棴椤圭洰鏃ュ織閰嶉銆?
quota			 鍚敤鏅€氱敤鎴风鐩橀厤棰濊璐︺€?
noquota			 绂佺敤鎵€鏈夋櫘閫氱鐩橀厤棰濋€夐」銆?
alloc_mode=%s		 璋冩暣鍧楀垎閰嶇瓥鐣ワ紝鏀寔鈥渞euse鈥濅笌鈥渄efault鈥濄€?
fsync_mode=%s		 鎺у埗 fsync 鐨勭瓥鐣ャ€傜洰鍓嶆敮鎸佲€減osix鈥濄€佲€渟trict鈥濅笌鈥渘obarrier鈥濄€傚湪榛樿鐨勨€減osix鈥濇ā寮忎笅锛宖sync 浼氶伒寰?POSIX 璇箟锛屽苟杩涜杞婚噺鎿嶄綔浠ユ彁鍗囨枃浠剁郴缁熸€ц兘銆傚湪鈥渟trict鈥濇ā寮忎笅锛宖sync 浼氳緝閲嶏紝琛屼负鍚?xfs銆乪xt4 涓?btrfs 鐪嬮綈锛寈fstest generic/342 浼氶€氳繃锛屼絾鎬ц兘浼氶€€鍖栥€傗€渘obarrier鈥濆熀浜庘€減osix鈥濓紝浣嗕笉鍍忊€渘obarrier鈥濇寕杞介€夐」閭ｆ牱涓洪潪鍘熷瓙鏂囦欢鍙戝嚭鍒锋柊鍛戒护銆?
test_dummy_encryption
test_dummy_encryption=%s
			 鍚敤铏氭嫙鍔犲瘑锛坉ummy encryption锛夛紝鎻愪緵浼€犵殑 fscrypt 涓婁笅鏂囥€備吉閫犵殑 fscrypt 涓婁笅鏂囦緵 xfstests 浣跨敤銆傚弬鏁板彲浠ユ槸鈥渧1鈥濇垨鈥渧2鈥濓紝浠ラ€夋嫨瀵瑰簲鐨?fscrypt 绛栫暐鐗堟湰銆?
checkpoint=%s[:%u[%]]	 璁句负鈥渄isable鈥濅互鍏抽棴妫€鏌ョ偣锛涜涓衡€渆nable鈥濅互閲嶆柊鍚敤妫€鏌ョ偣銆傞粯璁ゅ惎鐢ㄣ€傚湪绂佺敤鏈熼棿锛屼换浣曞嵏杞芥垨鎰忓鍏虫満閮戒細浣挎枃浠剁郴缁熷唴瀹硅〃鐜颁负鎸傝浇璇ラ€夐」鏃剁殑鏍峰瓙銆?
			 鍦ㄤ互 checkpoint=disable 鎸傝浇鏃讹紝鏂囦欢绯荤粺蹇呴』杩愯鍨冨溇鍥炴敹浠ョ‘淇濇墍鏈夊彲鐢ㄧ┖闂撮兘鑳借浣跨敤銆傚鏋滆繖鑰楁椂杩囬暱锛屾寕杞藉彲鑳借繑鍥?EAGAIN銆備綘鍙互閫夋嫨鎬у湴闄勫姞涓€涓€硷紝琛ㄧず浣犳効鎰忎复鏃舵斁寮冨灏戠鐩樼┖闂翠互閬垮厤棰濆鐨勫瀮鍦惧洖鏀躲€傝鍊煎彲缁欏畾涓哄潡鏁版垨鐧惧垎姣斻€備緥濡傦紝浠?checkpoint=disable:100% 鎸傝浇鎬讳細鎴愬姛锛屼絾鍙兘闅愯棌澶氳揪鍏ㄩ儴鍓╀綑绌洪棽绌洪棿銆傚疄闄呬笉鍙敤鐨勭┖闂村彲鍦?/sys/fs/f2fs/<disk>/unusable 鏌ョ湅銆備竴鏃?checkpoint=enable锛岃绌洪棿鍗宠鍥炴敹銆?
checkpoint_merge	 褰撴鏌ョ偣鍚敤鏃讹紝姝ら€夐」鍙敤浜庡垱寤轰竴涓唴鏍稿畧鎶よ繘绋嬶紝骞朵娇鍏跺敖鍙兘鍚堝苟骞跺彂鐨勬鏌ョ偣璇锋眰锛屼互娑堥櫎鍐椾綑鐨勬鏌ョ偣鍙戝嚭銆傛澶栵紝褰撴鏌ョ偣鍦ㄥ叿鏈夎緝浣?I/O 棰勭畻涓?CPU 浠介鐨?cgroup 鐨勮繘绋嬩笂涓嬫枃涓畬鎴愭椂锛屾垜浠彲浠ユ秷闄ょ紦鎱㈡鏌ョ偣鎿嶄綔瀵艰嚧鐨勫崱椤裤€備负浜嗚瀹冭〃鐜版洿濂斤紝鎴戜滑灏嗚鍐呮牳瀹堟姢杩涚▼鐨勯粯璁?I/O 浼樺厛绾ц涓衡€?鈥濓紝浣垮叾浼樺厛绾ч珮浜庡叾浠栧唴鏍哥嚎绋嬨€傝繖涓庝负 ext4 鏂囦欢绯荤粺鐨?jbd2 鏃ュ織绾跨▼璧嬩簣 I/O 浼樺厛绾х殑鏂瑰紡鐩稿悓銆?
nocheckpoint_merge	 绂佺敤妫€鏌ョ偣鍚堝苟鐗规€с€?
compress_algorithm=%s	 鎺у埗鍘嬬缉绠楁硶锛岀洰鍓?f2fs 鏀寔鈥渓zo鈥濄€佲€渓z4鈥濄€佲€渮std鈥濅笌鈥渓zo-rle鈥濈畻娉曘€?
compress_algorithm=%s:%d 鎺у埗鍘嬬缉绠楁硶鍙婂叾鍘嬬缉绾у埆锛岀洰鍓嶄粎
```

				 =========      ===========
				 algorithm      level range
				 =========      ===========
				 lz4            3 - 16
				 zstd           1 - 22
				 =========      ===========

```
compress_log_size=%u	 鏀寔閰嶇疆鍘嬬缉绨囧ぇ灏忋€傚ぇ灏忎负 4KB * (1 << %u)銆傞粯璁や笌鏈€灏忓ぇ灏忓潎涓?16KB銆?
compress_extension=%s	 鏀寔娣诲姞鎸囧畾鎵╁睍鍚嶏紝浣?f2fs 鑳藉湪瀵瑰簲鐨勬枃浠朵笂鍚敤鍘嬬缉銆備緥濡傦紝濡傛灉鎵€鏈夊甫鈥?ext鈥濈殑鏂囦欢鍘嬬缉鐜囧緢楂橈紝鎴戜滑鍙互灏嗏€?ext鈥濆姞鍏ュ帇缂╂墿灞曞悕鍒楄〃锛屽苟榛樿瀵硅繖浜涙枃浠跺惎鐢ㄥ帇缂╋紝鑰屾棤闇€閫氳繃 ioctl 鍚敤銆傚浜庡叾浠栨枃浠讹紝鎴戜滑浠嶅彲閫氳繃 ioctl 鍚敤鍘嬬缉銆傛敞鎰忥紝鏈変竴涓繚鐣欑殑鐗规畩鎵╁睍鍚嶁€?鈥濓紝鍙皢鍏惰缃互瀵规墍鏈夋枃浠跺惎鐢ㄥ帇缂┿€?
nocompress_extension=%s	 鏀寔娣诲姞鎸囧畾鎵╁睍鍚嶏紝浣?f2fs 鑳藉湪瀵瑰簲鐨勬枃浠朵笂绂佺敤鍘嬬缉锛屾伆鎭颁笌鍘嬬缉鎵╁睍鍚嶇浉鍙嶃€傚鏋滀綘纭垏鐭ラ亾鍝簺鏂囦欢鏃犳硶鍘嬬缉锛屽彲浠ヤ娇鐢ㄦ椤广€傚悓涓€涓墿灞曞悕涓嶈兘鍚屾椂鍑虹幇鍦?compress 涓?nocompress 鎵╁睍鍚嶄腑銆傚鏋?compress 鎵╁睍鍚嶆寚瀹氫簡鎵€鏈夋枃浠讹紝鍒?nocompress 鎵╁睍鍚嶆寚瀹氱殑绫诲瀷灏嗚瑙嗕负鐗逛緥鑰屼笉琚帇缂┿€備笉鍏佽鍦?nocompress 鎵╁睍鍚嶄腑浣跨敤鈥?鈥濇潵鎸囧畾鎵€鏈夋枃浠躲€傛坊鍔?nocompress_extension 鍚庯紝浼樺厛绾у簲涓猴細dir_flag < comp_extension,nocompress_extension < comp_file_flag,no_comp_file_flag銆傝瑙佸帇缂╃珷鑺傘€?
compress_chksum		 鏀寔鏍￠獙鍘嬬缉绨囦腑鍘熷鏁版嵁鐨勬牎楠屽拰锛坈hksum锛夈€?
compress_mode=%s	 鎺у埗鏂囦欢鍘嬬缉妯″紡銆傛敮鎸佲€渇s鈥濅笌鈥渦ser鈥濇ā寮忋€傚湪鈥渇s鈥濇ā寮忥紙榛樿锛変笅锛宖2fs 浼氬鍚敤鍘嬬缉鐨勬枃浠惰繘琛岃嚜鍔ㄥ帇缂┿€傚湪鈥渦ser鈥濇ā寮忎笅锛宖2fs 绂佺敤鑷姩鍘嬬缉锛屽苟灏嗛€夋嫨鐩爣鏂囦欢涓庢椂鏈轰氦鐢辩敤鎴峰喅瀹氥€傜敤鎴峰彲浠ヤ娇鐢?ioctl 瀵瑰惎鐢ㄥ帇缂╃殑鏂囦欢杩涜鎵嬪姩鍘嬬缉/瑙ｅ帇缂┿€?
compress_cache		 鏀寔浣跨敤鏂囦欢绯荤粺绠＄悊鐨?inode 鐨勫湴鍧€绌洪棿鏉ョ紦瀛樺帇缂╁潡锛屼互鎻愰珮闅忔満璇荤殑缂撳瓨鍛戒腑鐜囥€?
inlinecrypt		 鍦ㄥ彲鑳芥椂锛屼娇鐢?blk-crypto 妗嗘灦鑰岄潪鏂囦欢绯荤粺灞傚姞瀵嗭紝瀵瑰姞瀵嗘枃浠剁殑鍐呭杩涜鍔犲瘑/瑙ｅ瘑銆傝繖鍏佽浣跨敤鍐呰仈鍔犲瘑纭欢銆傜鐩樹笂鐨勬牸寮忎笉鍙楀奖鍝嶃€傛洿澶氱粏鑺傝 Documentation/block/inline-encryption.rst銆?
atgc			 鍚敤鍩轰簬骞撮緞闃堝€硷紙age-threshold锛夌殑鍨冨溇鍥炴敹锛屽畠鍦ㄥ悗鍙?GC 涓婃彁渚涢珮鏈夋晥鎬т笌楂樻晥鐜囥€?
discard_unit=%s	 鎺у埗涓㈠純鍗曞厓锛屽弬鏁板彲浠ユ槸鈥渂lock鈥濄€佲€渟egment鈥濅笌鈥渟ection鈥濓紝鍙戝嚭鐨?discard 鍛戒护鐨勫亸绉?澶у皬灏嗗榻愬埌璇ュ崟鍏冦€傞粯璁よ缃负鈥渄iscard_unit=block鈥濓紝浠庤€屽惎鐢ㄥ皬鍧椾涪寮冨姛鑳姐€傚浜?blkzoned 璁惧锛岄粯璁や細璁剧疆涓衡€渄iscard_unit=section鈥濓紝杩欐湁鍔╀簬澶у瀷 SMR 鎴?ZNS 璁惧閫氳繃鎽嗚劚鏀寔灏忓潡涓㈠純鐨?fs 鍏冩暟鎹潵闄嶄綆鍐呭瓨寮€閿€銆?
memory=%s		 鎺у埗鍐呭瓨妯″紡銆傛敮鎸佲€渘ormal鈥濅笌鈥渓ow鈥濇ā寮忋€傗€渓ow鈥濇ā寮忔槸涓烘敮鎸佷綆鍐呭瓨璁惧鑰屽紩鍏ョ殑銆傜敱浜庝綆鍐呭瓨璁惧鐨勭壒鎬э紝鍦ㄦ妯″紡涓?f2fs 鏈夋椂浼氫互鐗虹壊鎬ц兘涓轰唬浠锋潵鑺傜渷鍐呭瓨銆傗€渘ormal鈥濇ā寮忔槸榛樿妯″紡锛屼笌涔嬪墠鐩稿悓銆?
age_extent_cache	 鍚敤鍩轰簬 rb-tree 鐨勫勾榫?extent 缂撳瓨銆傚畠璁板綍姣忎釜 inode 鐨?extent 鏁版嵁鍧楁洿鏂伴鐜囷紝浠ヤ究涓烘暟鎹潡鍒嗛厤鎻愪緵鏇村ソ鐨勬俯搴︽彁绀恒€?
errors=%s		 鎸囧畾 f2fs 鍦ㄤ弗閲嶉敊璇椂鐨勮涓恒€傛敮鎸佺殑妯″紡鏈夛細鈥減anic鈥濄€佲€渃ontinue鈥濅笌鈥渞emount-ro鈥濓紝鍒嗗埆鎰忎负绔嬪嵆瑙﹀彂 panic銆佷笉鍋氫换浣曞鐞嗙户缁繍琛屻€佷互鍙婁互鍙妯″紡閲嶆柊鎸傝浇鍒嗗尯銆傞粯璁や娇鐢ㄢ€渃ontinue鈥濇ā寮忋€?

			 .. code-block:: none

			     ====================== =============== =============== ========
			     mode                   continue        remount-ro      panic
			     ====================== =============== =============== ========
			     access ops             normal          normal          N/A
			     syscall errors         -EIO            -EROFS          N/A
			     mount option           rw              ro              N/A
			     pending dir write      keep            keep            N/A
			     pending non-dir write  drop            keep            N/A
			     pending node write     drop            keep            N/A
			     pending meta write     keep            keep            N/A
			     ====================== =============== =============== ========
nat_bits		 鍚敤 nat_bits 鐗规€т互澧炲己瀵规弧/绌?nat 鍧楃殑璁块棶锛岄粯璁ょ鐢ㄣ€?
lookup_mode=%s	 鎺у埗瀵瑰ぇ灏忓啓鎶樺彔锛坈asefolded锛夌洰褰曠殑鐩綍鏌ユ壘琛屼负銆傝閫夐」瀵规湭鍚敤 casefold 鐗规€х殑鐩綍鏃犳晥銆?

			 .. code-block:: none
			     ================== ========================================
			     Value              Description
			     ================== ========================================
			     perf               (Default) Enforces a hash-only lookup.
					        The linear search fallback is always
					        disabled, ignoring the on-disk flag.
			     compat             Enables the linear search fallback for
					        compatibility with directory entries
					        created by older kernel that used a
					        different case-folding algorithm.
					        This mode ignores the on-disk flag.
			     auto               F2FS determines the mode based on the
					        on-disk `SB_ENC_NO_COMPAT_FALLBACK_FL`
					        flag.
			     ================== ========================================
======================== ============================================================

## Debugfs 鏉＄洰


/sys/kernel/debug/f2fs/ 鍖呭惈鏈夊叧鎵€鏈変互 f2fs 鎸傝浇鐨勫垎鍖虹殑淇℃伅銆傛瘡涓枃浠跺睍绀哄畬鏁寸殑 f2fs 淇℃伅銆?

/sys/kernel/debug/f2fs/status 鍖呭惈锛?

 - f2fs 褰撳墠绠＄悊鐨勪富瑕佹枃浠剁郴缁熶俊鎭?
 - 鍏充簬鏁翠釜娈电殑 SIT 骞冲潎淇℃伅
 - f2fs 褰撳墠娑堣€楃殑鍐呭瓨鍗犵敤

## Sysfs 鏉＄洰


鏈夊叧宸叉寕杞?f2fs 鏂囦欢绯荤粺鐨勪俊鎭彲鍦?/sys/fs/f2fs 涓壘鍒般€傛瘡涓凡鎸傝浇鐨勬枃浠剁郴缁熼兘浼氬湪 /sys/fs/f2fs 涓嬫牴鎹叾璁惧鍚嶆嫢鏈変竴涓洰褰曪紙渚嬪 /sys/fs/f2fs/sda锛夈€傛瘡涓瘡璁惧鐩綍涓嬬殑鏂囦欢濡備笅琛ㄦ墍绀恒€?

/sys/fs/f2fs/<devname> 涓嬬殑鏂囦欢
锛堝彟瑙?Documentation/ABI/testing/sysfs-fs-f2fs锛?

## 鐢ㄦ硶


1. 涓嬭浇鐢ㄦ埛鎬佸伐鍏峰苟缂栬瘧瀹冧滑銆?

2. 濡傛灉 f2fs 宸茶闈欐€佺紪璇戣繘鍐呮牳锛屽垯璺宠繃姝ゆ銆?
```

	# insmod f2fs.ko

```
```

	# mkdir /mnt/f2fs

```
```

	# mkfs.f2fs -l label /dev/block_device
	# mount -t f2fs /dev/block_device /mnt/f2fs

```
### mkfs.f2fs


mkfs.f2fs 鐢ㄤ簬灏嗗垎鍖烘牸寮忓寲涓?f2fs 鏂囦欢绯荤粺锛屽畠浼氭瀯寤哄熀鏈殑纾佺洏甯冨眬銆?

蹇€熼€夐」鍖呮嫭锛?

===============    ===========================================================
`-l [label]`     鎸囧畾鍗锋爣锛屾渶澶?512 涓?unicode 鍚嶇О銆?
`-a [0 or 1]`    涓烘瘡涓尯鍩熺殑璧峰浣嶇疆鎷嗗垎锛岀敤浜庡熀浜庡爢鐨勫垎閰嶃€?

                   榛樿璁句负 1锛屽嵆鎵ц姝ゆ搷浣溿€?
`-o [int]`       璁剧疆瓒呴閰嶇疆锛坥verprovision锛夋瘮渚嬶紝浠ュ嵎澶у皬鐨勭櫨鍒嗘瘮璁°€?

                   榛樿璁句负 5銆?
`-s [int]`       璁剧疆姣忎釜 section 鐨勬鏁伴噺銆?

                   榛樿璁句负 1銆?
`-z [int]`       璁剧疆姣忎釜 zone 鐨?section 鏁伴噺銆?

                   榛樿璁句负 1銆?
`-e [str]`       璁剧疆鍩烘湰鎵╁睍鍚嶅垪琛ㄣ€備緥濡?"mp3,gif,mov"銆?
`-t [0 or 1]`    鏄惁绂佺敤 discard 鍛戒护銆?

                   榛樿璁句负 1锛屽嵆鎵ц discard銆?
===============    ===========================================================

娉ㄦ剰锛氳鍙傝€?mkfs.f2fs(8) 鐨勬墜鍐岄〉鑾峰彇瀹屾暣閫夐」鍒楄〃銆?

### fsck.f2fs


fsck.f2fs 鏄敤浜庢鏌?f2fs 鏍煎紡鍖栧垎鍖轰竴鑷存€х殑宸ュ叿锛屽畠浼氭鏌ユ枃浠剁郴缁熷厓鏁版嵁涓庣敤鎴锋暟鎹槸鍚﹁姝ｇ‘浜ゅ弶寮曠敤銆傛敞鎰忥紝璇ュ伐鍏风殑鏃╂湡鐗堟湰涓嶄細淇浠讳綍涓嶄竴鑷淬€?
```

  -d debug level [default:0]

```
娉ㄦ剰锛氳鍙傝€?fsck.f2fs(8) 鐨勬墜鍐岄〉鑾峰彇瀹屾暣閫夐」鍒楄〃銆?

### dump.f2fs


dump.f2fs 鏄剧ず鐗瑰畾 inode 鐨勪俊鎭紝骞跺皢 SSA 涓?SIT 杞偍鍒版枃浠躲€傛瘡涓枃浠跺垎鍒负 dump_ssa 涓?dump_sit銆?

dump.f2fs 鐢ㄤ簬璋冭瘯 f2fs 鏂囦欢绯荤粺鐨勭鐩樻暟鎹粨鏋勩€傚畠鏄剧ず鐢辩粰瀹?inode 鍙疯瘑鍒殑纾佺洏 inode 淇℃伅锛屽苟鑳藉灏嗘墍鏈夌殑 SSA 涓?SIT 鏉＄洰杞偍鍒伴瀹氫箟鏂囦欢 ./dump_ssa 涓?./dump_sit 涓€?
```

  -d debug level [default:0]
  -i inode no (hex)
  -s [SIT dump segno from #1~#2 (decimal), for all 0~-1]
  -a [SSA dump segno from #1~#2 (decimal), for all 0~-1]

```
```

    # dump.f2fs -i [ino] /dev/sdx
    # dump.f2fs -s 0~-1 /dev/sdx (SIT dump)
    # dump.f2fs -a 0~-1 /dev/sdx (SSA dump)

```
娉ㄦ剰锛氳鍙傝€?dump.f2fs(8) 鐨勬墜鍐岄〉鑾峰彇瀹屾暣閫夐」鍒楄〃銆?

### sload.f2fs


sload.f2fs 鎻愪緵浜嗕竴绉嶅湪鐜版湁纾佺洏闀滃儚涓彃鍏ユ枃浠朵笌鐩綍鐨勬柟寮忋€傝宸ュ叿鍦ㄥ熀浜庡凡缂栬瘧鏂囦欢鏋勫缓 f2fs 闀滃儚鏃跺緢鏈夌敤銆?

娉ㄦ剰锛氳鍙傝€?sload.f2fs(8) 鐨勬墜鍐岄〉鑾峰彇瀹屾暣閫夐」鍒楄〃銆?

### resize.f2fs


resize.f2fs 璁╃敤鎴峰彲浠ヨ皟鏁?f2fs 鏍煎紡鍖栫鐩橀暅鍍忕殑澶у皬锛屽悓鏃朵繚鐣欓暅鍍忎腑瀛樺偍鐨勬墍鏈夋枃浠朵笌鐩綍銆?

娉ㄦ剰锛氳鍙傝€?resize.f2fs(8) 鐨勬墜鍐岄〉鑾峰彇瀹屾暣閫夐」鍒楄〃銆?

### defrag.f2fs


defrag.f2fs 鍙敤浜庡鏁ｅ竷鍐欏叆鐨勬暟鎹互鍙婅法纾佺洏鐨勬枃浠剁郴缁熷厓鏁版嵁杩涜纰庣墖鏁寸悊銆傞€氳繃鎻愪緵鏇村鐨勮繛缁┖闂茬┖闂达紝杩欏彲浠ユ彁鍗囧啓鍏ラ€熷害銆?

娉ㄦ剰锛氳鍙傝€?defrag.f2fs(8) 鐨勬墜鍐岄〉鑾峰彇瀹屾暣閫夐」鍒楄〃銆?

### f2fs_io


f2fs_io 鏄竴涓畝鍗曞伐鍏凤紝鐢ㄤ簬鍙戝嚭鍚勭鏂囦欢绯荤粺 API 浠ュ強 f2fs 鐗瑰畾鐨?API锛屽 QA 娴嬭瘯闈炲父鏈夌敤銆?

娉ㄦ剰锛氳鍙傝€?f2fs_io(8) 鐨勬墜鍐岄〉鑾峰彇瀹屾暣閫夐」鍒楄〃銆?

## 璁捐


### 纾佺洏甯冨眬


F2FS 灏嗘暣涓嵎鍒掑垎涓鸿嫢骞蹭釜娈碉紙segment锛夛紝姣忎釜娈靛浐瀹氫负 2MB 澶у皬銆備竴涓?section 鐢辫繛缁殑娈电粍鎴愶紝涓€涓?zone 鐢变竴缁?section 缁勬垚銆傞粯璁ゆ儏鍐典笅锛宻ection 涓?zone 鐨勫ぇ灏忛兘琚涓轰笌涓€涓澶у皬鐩稿悓锛屼絾鐢ㄦ埛鍙互閫氳繃 mkfs 杞绘澗淇敼杩欎簺澶у皬銆?

F2FS 灏嗘暣涓嵎鍒掑垎涓哄叚涓尯鍩燂紝闄よ秴绾у潡锛坰uperblock锛夊鎵€鏈夊尯鍩?
```

                                            align with the zone size <-|
                 |-> align with the segment size
     _________________________________________________________________________
    |            |            |   Segment   |    Node     |   Segment  |      |
    | Superblock | Checkpoint |    Info.    |   Address   |   Summary  | Main |
    |    (SB)    |   (CP)     | Table (SIT) | Table (NAT) | Area (SSA) |      |
    |____________|_____2______|______N______|______N______|______N_____|__N___|
                                                                       .      .
                                                             .                .
                                                 .                            .
                                    ._________________________________________.
                                    |_Segment_|_..._|_Segment_|_..._|_Segment_|
                                    .           .
                                    ._________._________
                                    |_section_|__...__|_
                                    .            .
		                    .________.
	                            |__zone__|

```
- Superblock (SB)
   瀹冧綅浜庡垎鍖虹殑寮€澶达紝瀛樺湪涓や唤鍓湰浠ラ伩鍏嶆枃浠剁郴缁熷穿婧冦€傚畠鍖呭惈鍩烘湰鐨勫垎鍖轰俊鎭互鍙?f2fs 鐨勪竴浜涢粯璁ゅ弬鏁般€?

- Checkpoint (CP)
   瀹冨寘鍚枃浠剁郴缁熶俊鎭€佹湁鏁?NAT/SIT 闆嗗悎鐨勪綅鍥俱€佸鍎匡紙orphan锛塱node 鍒楄〃锛屼互鍙婂綋鍓嶆椿鍔ㄦ鐨勬憳瑕佹潯鐩€?

- Segment Information Table (SIT)
   瀹冨寘鍚淇℃伅锛屼緥濡傛湁鏁堝潡璁℃暟锛屼互鍙婃墍鏈夊潡鏈夋晥鎬х殑浣嶅浘銆?

- Node Address Table (NAT)
   瀹冪敱瀛樺偍浜庝富鍖哄煙涓墍鏈夎妭鐐瑰潡鐨勫潡鍦板潃琛ㄧ粍鎴愩€?

- Segment Summary Area (SSA)
   瀹冨寘鍚憳瑕佹潯鐩紝杩欎簺鏉＄洰淇濆瓨浜嗗瓨鍌ㄤ簬涓诲尯鍩熶腑鎵€鏈夋暟鎹笌鑺傜偣鍧楃殑鎵€鏈夎€呬俊鎭€?

- Main Area
   瀹冨寘鍚枃浠朵笌鐩綍鏁版嵁锛屽寘鎷畠浠殑绱㈠紩銆?

涓轰簡閬垮厤鏂囦欢绯荤粺涓庡熀浜庨棯瀛樼殑瀛樺偍涔嬮棿鍑虹幇鏈榻愶紝F2FS 灏?CP 鐨勮捣濮嬪潡鍦板潃涓庢澶у皬瀵归綈銆傚悓鏃讹紝瀹冮€氳繃鍦?SSA 鍖哄煙涓繚鐣欎竴浜涙锛屽皢涓诲尯鍩熺殑璧峰鍧楀湴鍧€涓?zone 澶у皬瀵归綈銆?

鏇村鎶€鏈粏鑺傝鍙傝€冧互涓嬭皟鏌ャ€?
https://wiki.linaro.org/WorkingGroups/Kernel/Projects/FlashCardSurvey

### 鏂囦欢绯荤粺鍏冩暟鎹粨鏋?


F2FS 閲囩敤妫€鏌ョ偣锛坈heckpointing锛夋柟妗堟潵缁存姢鏂囦欢绯荤粺涓€鑷存€с€傚湪鎸傝浇鏃讹紝F2FS 棣栧厛灏濊瘯閫氳繃鎵弿 CP 鍖哄煙鏉ユ壘鍒版渶鍚庝竴涓湁鏁堢殑妫€鏌ョ偣鏁版嵁銆備负浜嗗噺灏戞壂鎻忔椂闂达紝F2FS 鍙娇鐢ㄤ袱浠?CP 鍓湰銆傚叾涓竴浠藉缁堟寚鍚戞渶鍚庝竴涓湁鏁堟暟鎹紝杩欒绉颁负褰卞瓙鍓湰锛坰hadow copy锛夋満鍒躲€傞櫎浜?CP 涔嬪锛孨AT 涓?SIT 涔熼噰鐢ㄤ簡褰卞瓙鍓湰鏈哄埗銆?

涓轰簡淇濊瘉鏂囦欢绯荤粺涓€鑷存€э紝姣忎釜 CP 鎸囧悜鍝簺 NAT 涓?SIT 鍓湰鏄?
```

  +--------+----------+---------+
  |   CP   |    SIT   |   NAT   |
  +--------+----------+---------+
  .         .          .          .
  .            .              .              .
  .               .                 .                 .
  +-------+-------+--------+--------+--------+--------+
  | CP #0 | CP #1 | SIT #0 | SIT #1 | NAT #0 | NAT #1 |
  +-------+-------+--------+--------+--------+--------+
     |             ^                          ^
     |             |                          |
     `----------------------------------------'

```
### 绱㈠紩缁撴瀯


绠＄悊鏁版嵁浣嶇疆鐨勫叧閿暟鎹粨鏋勬槸鈥渘ode锛堣妭鐐癸級鈥濄€備笌浼犵粺鏂囦欢缁撴瀯绫讳技锛孎2FS 鏈変笁绉嶇被鍨嬬殑鑺傜偣锛歩node銆佺洿鎺ヨ妭鐐癸紙direct node锛夈€侀棿鎺ヨ妭鐐癸紙indirect node锛夈€侳2FS 涓?inode 鍧楀垎閰?4KB锛屽叾涓寘鍚?923 涓暟鎹潡绱㈠紩銆佷袱涓洿鎺ヨ妭鐐规寚閽堛€佷袱涓棿鎺ヨ妭鐐规寚閽堬紝浠ュ強涓€涓弻闂存帴鑺傜偣鎸囬拡锛屽涓嬫墍绀恒€備竴涓洿鎺ヨ妭鐐瑰潡鍖呭惈 1018 涓暟鎹潡锛屼竴涓棿鎺ヨ妭鐐瑰潡涔熷寘鍚?1018 涓妭鐐瑰潡銆傚洜姝わ紝
```

  4KB * (923 + 2 * 1018 + 2 * 1018 * 1018 + 1018 * 1018 * 1018) := 3.94TB.

   Inode block (4KB)
     |- data (923)
     |- direct node (2)
     |          `- data (1018)
     |- indirect node (2)
     |            `- direct node (1018)
     |                       `- data (1018)
     `- double indirect node (1)
                         `- indirect node (1018)
			              `- direct node (1018)
	                                         `- data (1018)

```
娉ㄦ剰锛屾墍鏈夎妭鐐瑰潡閮界敱 NAT 鏄犲皠锛岃繖鎰忓懗鐫€姣忎釜鑺傜偣鐨勪綅缃兘閫氳繃 NAT 琛ㄨ繘琛岃浆鎹€傝€冭檻鍒版父璧版爲闂锛孎2FS 鑳藉鍒囨柇鐢卞彾瀛愭暟鎹啓鍏ュ紩璧风殑鑺傜偣鏇存柊浼犳挱銆?

### 鐩綍缁撴瀯


涓€涓洰褰曢」锛坉irectory entry锛夊崰鐢?11 瀛楄妭锛岀敱浠ヤ笅灞炴€х粍鎴愩€?

- hash		鏂囦欢鍚嶇殑鍝堝笇鍊?
- ino		inode 鍙?
- len		鏂囦欢鍚嶇殑闀垮害
- type		鏂囦欢绫诲瀷锛屽鐩綍銆佺鍙烽摼鎺ョ瓑

涓€涓?dentry 鍧楃敱 214 涓?dentry 妲戒笌鏂囦欢鍚嶇粍鎴愩€傚叾涓娇鐢ㄤ竴涓綅鍥炬潵琛ㄧず姣忎釜 dentry 鏄惁鏈夋晥銆備竴涓?dentry 鍧楀崰鐢?4KB锛屽叾缁勬垚濡備笅銆?
```

  Dentry Block(4 K) = bitmap (27 bytes) + reserved (3 bytes) +
	              dentries(11 * 214 bytes) + file name (8 * 214 bytes)

                         [Bucket]
             +--------------------------------+
             |dentry block 1 | dentry block 2 |
             +--------------------------------+
             .               .
       .                             .
  .       [Dentry Block Structure: 4KB]       .
  +--------+----------+----------+------------+
  | bitmap | reserved | dentries | file names |
  +--------+----------+----------+------------+
  [Dentry Block: 4KB] .   .
		 .               .
            .                          .
            +------+------+-----+------+
            | hash | ino  | len | type |
            +------+------+-----+------+
            [Dentry Structure: 11 bytes]

```
F2FS 涓虹洰褰曠粨鏋勫疄鐜颁簡澶氱骇鍝堝笇琛ㄣ€傛瘡涓€绾ч兘鏈変竴涓叿鏈変笓闂ㄦ暟閲忓搱甯屾《锛坔ash bucket锛夌殑鍝堝笇琛紝濡備笅鎵€绀恒€傛敞鎰忥紝鈥淎(2B)鈥濊〃绀轰竴涓《鍖呭惈 2 涓暟鎹潡銆?
```

    ----------------------
    A : bucket
    B : block
    N : MAX_DIR_HASH_DEPTH
    ----------------------

    level #0   | A(2B)
	    |
    level #1   | A(2B) - A(2B)
	    |
    level #2   | A(2B) - A(2B) - A(2B) - A(2B)
	.     |   .       .       .       .
    level #N/2 | A(2B) - A(2B) - A(2B) - A(2B) - A(2B) - ... - A(2B)
	.     |   .       .       .       .
    level #N   | A(4B) - A(4B) - A(4B) - A(4B) - A(4B) - ... - A(4B)

```
```

                            ,- 2, if n < MAX_DIR_HASH_DEPTH / 2,
  # of blocks in level #n = |
                            `- 4, Otherwise

                             ,- 2^(n + dir_level),
			     |        if n + dir_level < MAX_DIR_HASH_DEPTH / 2,
  # of buckets in level #n = |
                             `- 2^((MAX_DIR_HASH_DEPTH / 2) - 1),
			              Otherwise

```
褰?F2FS 鍦ㄧ洰褰曚腑鏌ユ壘鏂囦欢鍚嶆椂锛岄鍏堣绠楁枃浠跺悕鐨勫搱甯屽€笺€傜劧鍚庯紝F2FS 鎵弿绾у埆 #0 鐨勫搱甯岃〃锛屼互鏌ユ壘鐢辨枃浠跺悕鍙婂叾 inode 鍙风粍鎴愮殑 dentry銆傚鏋滄湭鎵惧埌锛孎2FS 浼氭壂鎻忕骇鍒?#1 鐨勪笅涓€涓搱甯岃〃銆備緷姝ょ被鎺紝F2FS 浠?1 鍒?N 閫愮骇閫掑鍦版壂鎻忓搱甯岃〃銆傚湪姣忎竴绾т腑锛孎2FS 鍙渶鎵弿鐢变笅寮忕‘瀹氱殑涓€涓《锛屽叾澶嶆潅搴︿负 O(log(鏂囦欢鏁?)
```

  bucket number to scan in level #n = (hash value) % (# of buckets in level #n)

```
鍦ㄥ垱寤烘枃浠舵椂锛孎2FS 浼氭煡鎵捐鐩栬鏂囦欢鍚嶇殑杩炵画绌烘Ы銆侳2FS 浠ヤ笌鏌ユ壘鎿嶄綔鐩稿悓鐨勬柟寮忥紝浠?1 鍒?N 鍦ㄦ墍鏈夌骇鍒殑鍝堝笇琛ㄤ腑鎼滅储绌烘Ы銆?
```

       --------------> Dir <--------------
       |                                 |
    child                             child

    child - child                     [hole] - child

    child - child - child             [hole] - [hole] - child

   Case 1:                           Case 2:
   Number of children = 6,           Number of children = 3,
   File size = 7                     File size = 7

```
### 榛樿鍧楀垎閰?


鍦ㄨ繍琛屾椂锛孎2FS 鍦ㄢ€淢ain鈥濆尯鍩熷唴绠＄悊鍏釜娲诲姩鏃ュ織锛氱儹/娓?鍐疯妭鐐癸紙Hot/Warm/Cold node锛変笌鐑?娓?鍐锋暟鎹紙Hot/Warm/Cold data锛夈€?

- Hot node	鍖呭惈鐩綍鐨勭洿鎺ヨ妭鐐瑰潡銆?
- Warm node	鍖呭惈闄ょ儹鑺傜偣鍧椾箣澶栫殑鐩存帴鑺傜偣鍧椼€?
- Cold node	鍖呭惈闂存帴鑺傜偣鍧椼€?
- Hot data	鍖呭惈 dentry 鍧椼€?
- Warm data	鍖呭惈闄ょ儹鏁版嵁涓庡喎鏁版嵁鍧椾箣澶栫殑鏁版嵁鍧椼€?
- Cold data	鍖呭惈澶氬獟浣撴暟鎹垨杩佺Щ鐨勬暟鎹潡銆?

LFS 鏈変袱绉嶇┖闂茬┖闂寸鐞嗘柟妗堬細绾跨▼鍖栨棩蹇楋紙threaded log锛変笌澶嶅埗-鍘嬬缉锛坈opy-and-compaction锛夈€傝绉颁负娓呯悊锛坈leaning锛夌殑澶嶅埗-鍘嬬缉鏂规闈炲父閫傚悎椤哄簭鍐欏叆鎬ц兘闈炲父濂界殑璁惧锛屽洜涓哄缁堟湁绌洪棽娈靛彲鐢ㄤ簬鍐欏叆鏂版暟鎹€傜劧鑰岋紝鍦ㄩ珮鍒╃敤鐜囦笅瀹冧細鍙楀埌娓呯悊寮€閿€鐨勫洶鎵般€傜浉鍙嶏紝绾跨▼鍖栨棩蹇楁柟妗堜細鍙楅殢鏈哄啓鍏ヤ箣鑻︼紝浣嗕笉闇€瑕佹竻鐞嗚繃绋嬨€侳2FS 閲囩敤娣峰悎鏂规锛氶粯璁ら噰鐢ㄥ鍒?鍘嬬缉鏂规锛屼絾浼氭牴鎹枃浠剁郴缁熺姸鎬佸姩鎬佸垏鎹㈠埌绾跨▼鍖栨棩蹇楁柟妗堛€?

涓轰簡浣?F2FS 涓庡簳灞傚熀浜庨棯瀛樼殑瀛樺偍瀵归綈锛孎2FS 浠?section 涓哄崟浣嶅垎閰嶆銆侳2FS 鏈熸湜 section 澶у皬涓?FTL 涓瀮鍦惧洖鏀剁殑鍗曚綅澶у皬鐩稿悓銆傛澶栵紝鍏充簬 FTL 涓殑鏄犲皠绮掑害锛孎2FS 灏藉彲鑳戒粠涓嶅悓 zone 鍒嗛厤娲诲姩鏃ュ織鐨勬瘡涓?section锛屽洜涓?FTL 鍙互鎸夊叾鏄犲皠绮掑害灏嗘椿鍔ㄦ棩蹇椾腑鐨勬暟鎹啓鍏ヤ竴涓垎閰嶅崟鍏冦€?

### 娓呯悊杩囩▼


F2FS 鏃㈡寜闇€娓呯悊锛屼篃鍦ㄥ悗鍙版竻鐞嗐€傛寜闇€娓呯悊鍦ㄦ病鏈夎冻澶熺殑绌洪棽娈垫潵鏈嶅姟 VFS 璋冪敤鏃惰Е鍙戙€傚悗鍙版竻鐞嗗櫒鐢变竴涓唴鏍哥嚎绋嬭繍琛岋紝骞跺湪绯荤粺绌洪棽鏃惰Е鍙戞竻鐞嗗伐浣溿€?

F2FS 鏀寔涓ょ鍙楀鑰呴€夋嫨绛栫暐锛氳椽蹇冿紙greedy锛変笌鎴愭湰鏀剁泭锛坈ost-benefit锛夌畻娉曘€傚湪璐績绠楁硶涓紝F2FS 閫夋嫨鏈夋晥鍧楁暟閲忔渶灏戠殑鍙楀鑰呮銆傚湪鎴愭湰鏀剁泭绠楁硶涓紝F2FS 鏍规嵁娈靛勾榫勪笌鏈夋晥鍧楁暟閲忛€夋嫨鍙楀鑰呮锛屼互瑙ｅ喅璐績绠楁硶涓殑鏃ュ織鍧楁姈鍔紙log block thrashing锛夐棶棰樸€侳2FS 瀵规寜闇€娓呯悊鍣ㄩ噰鐢ㄨ椽蹇冪畻娉曪紝鑰屽悗鍙版竻鐞嗗櫒閲囩敤鎴愭湰鏀剁泭绠楁硶銆?

涓轰簡璇嗗埆鍙楀鑰呮涓殑鏁版嵁鏄惁鏈夋晥锛孎2FS 绠＄悊涓€涓綅鍥俱€傛瘡涓€浣嶄唬琛ㄤ竴涓潡鐨勬湁鏁堟€э紝璇ヤ綅鍥剧敱瑕嗙洊涓诲尯鍩熸墍鏈夊潡鐨勪綅娴佺粍鎴愩€?

### 鍐欐彁绀猴紙Write-hint锛夌瓥鐣?


F2FS 濮嬬粓鎸夌収浠ヤ笅绛栫暐璁剧疆鍐欐彁绀猴紙whint锛夈€?

===================== ======================== ===================
User                  F2FS                     Block
===================== ======================== ===================
N/A                   META                     WRITE_LIFE_NONE|REQ_META
N/A                   HOT_NODE                 WRITE_LIFE_NONE
N/A                   WARM_NODE                WRITE_LIFE_MEDIUM
N/A                   COLD_NODE                WRITE_LIFE_LONG
ioctl(COLD)           COLD_DATA                WRITE_LIFE_EXTREME
extension list        "                        "

### -- buffered io

N/A                   COLD_DATA                WRITE_LIFE_EXTREME
N/A                   HOT_DATA                 WRITE_LIFE_SHORT
N/A                   WARM_DATA                WRITE_LIFE_NOT_SET

### -- direct io
WRITE_LIFE_EXTREME    COLD_DATA                WRITE_LIFE_EXTREME
WRITE_LIFE_SHORT      HOT_DATA                 WRITE_LIFE_SHORT
WRITE_LIFE_NOT_SET    WARM_DATA                WRITE_LIFE_NOT_SET
WRITE_LIFE_NONE       "                        WRITE_LIFE_NONE
WRITE_LIFE_MEDIUM     "                        WRITE_LIFE_MEDIUM
WRITE_LIFE_LONG       "                        WRITE_LIFE_LONG
===================== ======================== ===================

### Fallocate(2) 绛栫暐


榛樿绛栫暐閬靛惊浠ヤ笅 POSIX 瑙勫垯銆?

鍒嗛厤纾佺洏绌洪棿
     fallocate() 鐨勯粯璁ゆ搷浣滐紙鍗?mode 涓洪浂锛夊湪 offset 涓?len 鎸囧畾鐨勮寖鍥村唴鍒嗛厤纾佺洏绌洪棿銆傚鏋?offset+len 澶т簬鏂囦欢澶у皬锛屽垯鏂囦欢澶у皬锛堢敱 stat(2) 鎶ュ憡锛変細琚敼鍙樸€傚湪璋冪敤鍓?offset 涓?len 鎸囧畾鑼冨洿鍐呭師鏈笉鍚暟鎹殑浠讳綍瀛愬尯鍩燂紝灏嗚鍒濆鍖栦负闆躲€傝繖涓€榛樿琛屼负闈炲父绫讳技浜?posix_fallocate(3) 搴撳嚱鏁扮殑琛屼负锛屽苟琚璁′负鏈€浼樺疄鐜拌鍑芥暟鐨勬柟娉曘€?

鐒惰€岋紝涓€鏃?F2FS 鍦?fallocate(fd, DEFAULT_MODE) 涔嬪墠鏀跺埌 ioctl(fd, F2FS_IOC_SET_PIN_FILE)锛屽畠灏变細鍒嗛厤鍏锋湁闆舵垨闅忔満鏁版嵁鐨勭鐩樺潡鍦板潃锛岃繖瀵逛簬浠ヤ笅鍦烘櫙寰堟湁鐢細

 1. create(fd)
 2. ioctl(fd, F2FS_IOC_SET_PIN_FILE)
 3. fallocate(fd, 0, 0, size)
 4. address = fibmap(fd, offset)
 5. open(blkdev)
 6. write(blkdev, address)

### 鍘嬬缉瀹炵幇


- 鏂版湳璇€渃luster锛堢皣锛夆€濊瀹氫箟涓哄帇缂╃殑鍩烘湰鍗曚綅锛屾枃浠跺彲浠ュ湪閫昏緫涓婂垝鍒嗕负澶氫釜绨囥€備竴涓皣鍖呭惈 4 << n锛坣 >= 0锛変釜閫昏緫椤碉紝鍘嬬缉澶у皬涔熷氨鏄皣澶у皬锛屾瘡涓皣鍙互琚帇缂╂垨涓嶅帇缂┿€?

- 鍦ㄧ皣鍏冩暟鎹竷灞€涓紝浣跨敤涓€涓壒娈婂潡鍦板潃鏉ユ寚绀轰竴涓皣鏄帇缂╃皣杩樻槸鏅€氱皣锛涘浜庡帇缂╃皣锛屽叾鍚庣殑鍏冩暟鎹皢绨囨槧灏勫埌 [1, 4 << n - 1] 涓墿鐞嗗潡锛宖2fs 鍦ㄥ叾涓瓨鍌ㄥ寘鍚帇缂╁ご涓庡帇缂╂暟鎹殑鏁版嵁銆?

- 涓轰簡鍦ㄨ鐩栧啓鍏ユ湡闂存秷闄ゅ啓鏀惧ぇ锛孎2FS 浠呮敮鎸佸鍙啓涓€娆★紙write-once锛夌殑鏂囦欢杩涜鍘嬬缉锛涘彧鏈夊綋绨囦腑鎵€鏈夐€昏緫鍧楅兘鍖呭惈鏈夋晥鏁版嵁銆佷笖绨囨暟鎹殑鍘嬬缉姣斾綆浜庢寚瀹氶槇鍊兼椂锛屾暟鎹墠鑳借鍘嬬缉銆?

- 瑕佸鏅€?inode 鍚敤鍘嬬缉锛屾湁鍥涚鏂瑰紡锛?

  - chattr +c file
  - chattr +c dir; touch dir/file
  - mount w/ -o compress_extension=ext; touch file.ext
  - mount w/ -o compress_extension=*; touch any_file

- 瑕佸鏅€?inode 绂佺敤鍘嬬缉锛屾湁涓ょ鏂瑰紡锛?

  - chattr -c file
  - mount w/ -o nocompress_extension=ext; touch file.ext

- FS_COMPR_FL銆丗S_NOCOMP_FS 涓庢墿灞曞悕涔嬮棿鐨勪紭鍏堢骇锛?

  - compress_extension=so; nocompress_extension=zip; chattr +c dir; touch
    dir/foo.so; touch dir/bar.zip; touch dir/baz.txt; 鍒?foo.so 涓?baz.txt
    搴旇鍘嬬缉锛宐ar.zip 搴斾笉琚帇缂┿€俢hattr +c dir/bar.zip 鍙湪 bar.zip 涓婂惎鐢ㄥ帇缂┿€?
  - compress_extension=so; nocompress_extension=zip; chattr -c dir; touch
    dir/foo.so; touch dir/bar.zip; touch dir/baz.txt; 鍒?foo.so 搴旇
    鍘嬬缉锛宐ar.zip 涓?baz.txt 搴斾笉琚帇缂┿€?
    chattr+c dir/bar.zip; chattr+c dir/baz.txt; 鍙湪 bar.zip 涓?baz.txt 涓婂惎鐢ㄥ帇缂┿€?
- 姝ゆ椂锛屽帇缂╃壒鎬т笉浼氱洿鎺ュ悜鐢ㄦ埛鏆撮湶鍘嬬缉鍚庣殑绌洪棿锛屼互淇濊瘉璇ョ┖闂村悗缁綔鍦ㄧ殑鏁版嵁鏇存柊銆傜浉鍙嶏紝鍏朵富瑕佺洰鏍囨槸灏藉彲鑳藉噺灏戝啓鍏ラ棯瀛樼洏鐨勬暟鎹紝浠庤€屽欢闀跨鐩樺鍛藉苟缂撹В I/O 鎷ュ銆傛澶栵紝鎴戜滑娣诲姞浜?ioctl(F2FS_IOC_RELEASE_COMPRESS_BLOCKS) 鎺ュ彛锛岀敤浜庡湪涓?inode 璁剧疆鐗规畩鏍囧織鍚庡洖鏀跺帇缂╃┖闂村苟灞曠ず缁欑敤鎴枫€備竴鏃﹀帇缂╃┖闂磋閲婃斁锛岃鏍囧織灏嗛樆姝㈠悜鏂囦欢鍐欏叆鏁版嵁锛岀洿鍒伴€氳繃 ioctl(F2FS_IOC_RESERVE_COMPRESS_BLOCKS) 棰勭暀鍘嬬缉绌洪棿锛屾垨灏嗘枃浠跺ぇ灏忔埅鏂负闆躲€?
```

				[Dnode Structure]
		+-----------------------------------------------+
		| cluster 1 | cluster 2 | ......... | cluster N |
		+-----------------------------------------------+
		.           .                       .           .
	  .                      .                .                      .
    .         Compressed Cluster       .        .        Normal Cluster            .
    +----------+---------+---------+---------+  +---------+---------+---------+---------+
    |compr flag| block 1 | block 2 | block 3 |  | block 1 | block 2 | block 3 | block 4 |
    +----------+---------+---------+---------+  +---------+---------+---------+---------+
	       .                             .
	    .                                           .
	.                                                           .
	+-------------+-------------+----------+----------------------------+
	| data length | data chksum | reserved |      compressed data       |
	+-------------+-------------+----------+----------------------------+

```
### 鍘嬬缉妯″紡


f2fs 閫氳繃鈥渃ompression_mode鈥濇寕杞介€夐」鏀寔鈥渇s鈥濅笌鈥渦ser鈥濅袱绉嶅帇缂╂ā寮忋€備娇鐢ㄨ閫夐」锛宖2fs 鎻愪緵浜嗛€夋嫨濡備綍鍘嬬缉鍚敤鍘嬬缉鐨勬枃浠剁殑鏂瑰紡锛堝浣曞鏅€?inode 鍚敤鍘嬬缉锛岃鍙傝€冣€淐ompression implementation鈥濈珷鑺傦級銆?

1) compress_mode=fs

   杩欐槸榛樿閫夐」銆俧2fs 鍦ㄥ鍚敤鍘嬬缉鐨勬枃浠舵墽琛屽洖鍐欙紙writeback锛夋椂鑷姩杩涜鍘嬬缉銆?

2) compress_mode=user

   杩欎細绂佺敤鑷姩鍘嬬缉锛屽苟灏嗛€夋嫨鐩爣鏂囦欢涓庢椂鏈虹殑鍐冲畾鏉冧氦缁欑敤鎴枫€傜敤鎴峰彲浠ヤ娇鐢?F2FS_IOC_DECOMPRESS_FILE 涓?F2FS_IOC_COMPRESS_FILE ioctl锛屽鍚敤鍘嬬缉鐨勬枃浠惰繘琛屾墜鍔ㄥ帇缂?瑙ｅ帇缂╋紝濡備笅鎵€绀恒€?
```

  fd = open(filename, O_WRONLY, 0);
  ret = ioctl(fd, F2FS_IOC_DECOMPRESS_FILE);

```
```

  fd = open(filename, O_WRONLY, 0);
  ret = ioctl(fd, F2FS_IOC_COMPRESS_FILE);

```
### NVMe 鍒嗗尯鍛藉悕绌洪棿锛圸oned Namespace锛夎澶?


- ZNS 瀹氫箟浜嗕竴涓瘡鍖哄煙锛坧er-zone锛夊閲忥紝瀹冨彲浠ョ瓑浜庢垨灏忎簬鍖哄煙澶у皬锛坺one-size锛夈€俍one-capacity 鏄鍖哄煙涓彲鐢ㄥ潡鐨勬暟閲忋€侳2FS 浼氭鏌?zone-capacity 鏄惁灏忎簬 zone-size锛屽鏋滄槸锛屽垯鍦ㄥ垵濮嬫寕杞芥椂锛屼换浣曡捣濮嬩綅缃湪 zone-capacity 涔嬪悗鐨勬浼氬湪绌洪棽娈典綅鍥句腑琚爣璁颁负涓嶅彲鐢ㄣ€傝繖浜涙琚爣璁颁负姘镐箙浣跨敤锛屽洜姝や笉浼氳鍒嗛厤鐢ㄤ簬鍐欏叆锛屼篃灏辨棤闇€杩涜鍨冨溇鍥炴敹銆傚鏋?zone-capacity 鏈笌榛樿娈靛ぇ灏忥紙2MB锛夊榻愶紝閭ｄ箞涓€涓鍙互鍦?zone-capacity 涔嬪墠寮€濮嬪苟璺ㄨ秺 zone-capacity 杈圭晫銆傝繖绉嶈法杈圭晫鐨勬涔熻瑙嗕负鍙敤娈点€傝繖浜涙涓綅浜?zone-capacity 涔嬪悗鐨勬墍鏈夊潡閮借瑙嗕负涓嶅彲鐢ㄣ€?

### 璁惧鍒悕锛坉evice aliasing锛夌壒鎬?


f2fs 鍙互鍒╃敤涓€涓О涓衡€滆澶囧埆鍚嶆枃浠讹紙device aliasing file锛夆€濈殑鐗规畩鏂囦欢銆傝鏂囦欢鍏佽鐢ㄥ崟涓ぇ extent 鏄犲皠鏁翠釜瀛樺偍璁惧锛岃€屼笉浣跨敤甯歌鐨?f2fs 鑺傜偣缁撴瀯銆傝繖鍧楄鏄犲皠鐨勫尯鍩熻鍥哄畾锛坧inned锛夛紝涓昏鐢ㄤ簬鍗犳嵁绌洪棿銆?

鏈川涓婏紝杩欎竴鏈哄埗鍏佽 f2fs 鍖哄煙鐨勪竴閮ㄥ垎琚复鏃朵繚鐣欏苟渚涘彟涓€涓枃浠剁郴缁熸垨鍏朵粬鐢ㄩ€斾娇鐢ㄣ€備竴鏃﹁澶栭儴浣跨敤瀹屾垚锛岃澶囧埆鍚嶆枃浠跺嵆鍙鍒犻櫎锛屽皢淇濈暀鐨勭┖闂撮噴鏀惧洖 F2FS 渚涘叾鑷韩浣跨敤銆?


   # ls /dev/vd*
   /dev/vdb (32GB) /dev/vdc (32GB)
   # mkfs.ext4 /dev/vdc
   # mkfs.f2fs -c /dev/vdc@vdc.file /dev/vdb
   # mount /dev/vdb /mnt/f2fs
   # ls -l /mnt/f2fs
   vdc.file
   # df -h
   /dev/vdb                            64G   33G   32G  52% /mnt/f2fs

   # mount -o loop /dev/vdc /mnt/ext4
   # df -h
   /dev/vdb                            64G   33G   32G  52% /mnt/f2fs
   /dev/loop7                          32G   24K   30G   1% /mnt/ext4
   # umount /mnt/ext4

   # f2fs_io getflags /mnt/f2fs/vdc.file
   get a flag on /mnt/f2fs/vdc.file ret=0, flags=nocow(pinned),immutable
   # f2fs_io setflags noimmutable /mnt/f2fs/vdc.file
   get a flag on noimmutable ret=0, flags=800010
   set a flag on /mnt/f2fs/vdc.file ret=0, flags=noimmutable
   # rm /mnt/f2fs/vdc.file
   # df -h
   /dev/vdb                            64G  753M   64G   2% /mnt/f2fs

鍥犳锛屽叾鏍稿績鎬濇兂鏄細鐢ㄦ埛鍙互鍦?/dev/vdc 涓婃墽琛屼换浣曟枃浠舵搷浣滐紝骞跺湪浣跨敤瀹屽悗鍥炴敹绌洪棿锛岃€岃繖浜涚┖闂磋璁″叆 /data銆傝繖涓嶉渶瑕佷慨鏀瑰垎鍖哄ぇ灏忎笌鏂囦欢绯荤粺鏍煎紡銆?

### 姣忔枃浠跺彧璇诲ぇ Folio 鏀寔


F2FS 鍦ㄨ鍙栬矾寰勪笂瀹炵幇浜嗗ぇ folio锛坙arge folio锛夋敮鎸侊紝浠ュ埄鐢ㄩ珮闃堕〉鍒嗛厤鑾峰緱鏄捐憲鐨勬€ц兘鎻愬崌銆備负浜嗘渶灏忓寲浠ｇ爜澶嶆潅搴︼紝璇ユ敮鎸佺洰鍓嶈鎺掗櫎鍦ㄥ啓鍏ヨ矾寰勪箣澶栵紝鍥犱负鍐欏叆璺緞闇€瑕佸鐞嗗帇缂╀笌鍧楀垎閰嶆ā寮忕瓑澶嶆潅浼樺寲銆?

杩欎竴鍙€夌壒鎬т粎鍦ㄦ枃浠剁殑 immutable 浣嶈璁剧疆鏃惰Е鍙戙€傚洜姝わ紝鍗充娇鐢ㄦ埛鍦ㄦ竻闄よ浣嶅悗绔嬪嵆灏濊瘯浠ュ啓鏉冮檺鎵撳紑涓€涓凡缂撳瓨鐨勬枃浠讹紝F2FS 涔熶細杩斿洖 EOPNOTSUPP銆傚彧鏈夊湪宸茬紦瀛樼殑 inode 琚涪寮冨悗锛屽啓璁块棶鎵嶄細鎭㈠銆傚叾浣跨敤娴佺▼濡備笅鎵€绀猴細


   # f2fs_io setflags immutable /data/testfile_read_seq

   /** flush and reload the inode to enable the large folio **/
   # sync && echo 3 > /proc/sys/vm/drop_caches

   /** mmap(MAP_POPULATE) + mlock() **/
   # f2fs_io read 128 0 1024 mmap 1 0 /data/testfile_read_seq
   /** mmap() + fadvise(POSIX_FADV_WILLNEED) + mlock() **/
   # f2fs_io read 128 0 1024 fadvise 1 0 /data/testfile_read_seq

   /** mmap() + mlock2(MLOCK_ONFAULT) + madvise(MADV_POPULATE_READ) **/
   # f2fs_io read 128 0 1024 madvise 1 0 /data/testfile_read_seq

   # f2fs_io clearflags immutable /data/testfile_read_seq

   # f2fs_io write 1 0 1 zero buffered /data/testfile_read_seq
   Failed to open /mnt/test/test: Operation not supported

   /** flush and reload the inode to disable the large folio **/
   # sync && echo 3 > /proc/sys/vm/drop_caches

   # f2fs_io write 1 0 1 zero buffered /data/testfile_read_seq
   Written 4096 bytes with pattern = zero, total_time = 29 us, max_latency = 28 us

   # rm /data/testfile_read_seq