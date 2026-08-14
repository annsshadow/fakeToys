
## 铏氭嫙鍔犻€熷櫒浜ゆ崲鏈猴紙Virtual Accelerator Switchboard锛孷AS锛夌敤鎴风┖闂?API


## 绠€浠?

Power9 澶勭悊鍣ㄥ紩鍏ヤ簡铏氭嫙鍔犻€熷櫒浜ゆ崲鏈猴紙VAS锛夛紝瀹冨厑璁哥敤鎴风┖闂村拰鍐呮牳涓庤绉颁负
Nest Accelerator锛圢X锛夌殑鍗忓鐞嗗櫒锛堢‖浠跺姞閫熷櫒锛夎繘琛岄€氫俊銆侼X 鍗曞厓鐢变竴涓垨澶氫釜
纭欢寮曟搸鎴栧崗澶勭悊鍣ㄧ被鍨嬬粍鎴愶紝渚嬪 842 鍘嬬缉銆丟ZIP 鍘嬬缉鍜屽姞瀵嗐€傚湪 power9 涓婏紝
鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍙兘璁块棶鏀寔纭欢涓?ZLIB 鍜?GZIP 鍘嬬缉绠楁硶鐨?GZIP 鍘嬬缉寮曟搸銆?
涓轰簡涓?NX 閫氫俊锛屽唴鏍稿繀椤诲缓绔嬩竴涓€氶亾鎴栫獥鍙ｏ紝鐒跺悗璇锋眰灏卞彲浠ョ洿鎺ユ彁浜わ紝鑰屾棤闇€
鍐呮牳鍙備笌銆傚彂寰€ GZIP 寮曟搸鐨勮姹傚繀椤昏鏍煎紡鍖栦负鍗忓鐞嗗櫒璇锋眰鍧楋紙CRB锛夛紝骞朵笖杩欎簺
CRB 蹇呴』浣跨敤 COPY/PASTE 鎸囦护鎻愪氦缁?NX锛屾妸 CRB 绮樿创鍒颁笌璇ュ紩鎿庤姹傞槦鍒楃浉鍏宠仈鐨?纭欢鍦板潃涓娿€?
GZIP 寮曟搸鎻愪緵涓や釜浼樺厛绾х殑璇锋眰锛氭櫘閫氾紙Normal锛夊拰楂橈紙High锛夈€傜洰鍓嶄粠鐢ㄦ埛绌洪棿
鍙敮鎸佹櫘閫氳姹傘€?
鏈枃妗ｈВ閲婁簡鐢ㄤ簬涓庡唴鏍镐氦浜掍互寤虹珛閫氶亾/绐楀彛鐨勭敤鎴风┖闂?API锛岃绐楀彛鍙敤浜庣洿鎺ュ悜
NX 鍔犻€熷櫒鍙戦€佸帇缂╄姹傘€?

## 姒傝堪


閫氳繃鐢?VAS/NX 璁惧椹卞姩瀹炵幇鐨?/dev/crypto/nx-gzip 璁惧鑺傜偣锛屾彁渚涘 GZIP 寮曟搸
鐨勮闂€傚簲鐢ㄧ▼搴忓繀椤绘墦寮€ /dev/crypto/nx-gzip 璁惧浠ヨ幏寰椾竴涓枃浠舵弿杩扮锛坒d锛夈€?鐒跺悗搴斿綋鐢ㄨ繖涓?fd 鍙戝嚭 VAS_TX_WIN_OPEN ioctl 鏉ュ缓绔嬩笌寮曟搸鐨勮繛鎺ャ€傝繖鎰忓懗鐫€涓?璇ヨ繘绋嬪湪 GZIP 寮曟搸涓婃墦寮€浜嗕竴涓彂閫佺獥鍙ｃ€備竴鏃﹀缓绔嬭繛鎺ワ紝搴旂敤绋嬪簭灏卞簲褰撲娇鐢?mmap() 绯荤粺璋冪敤鎶婂紩鎿庤姹傞槦鍒楃殑纭欢鍦板潃鏄犲皠鍒板簲鐢ㄧ▼搴忕殑铏氭嫙鍦板潃绌洪棿銆?
鐒跺悗锛屽簲鐢ㄧ▼搴忓彲浠ラ€氳繃浣跨敤 copy/paste 鎸囦护鎶?CRB 绮樿创鍒?mmap() 杩斿洖鐨勮櫄鎷熷湴鍧€
锛堝嵆 paste_address锛夋潵鍚戝紩鎿庢彁浜や竴涓垨澶氫釜璇锋眰銆傜敤鎴风┖闂村彲浠ラ€氳繃鍏抽棴鏂囦欢鎻忚堪绗?锛坈lose(fd)锛夋垨鍦ㄨ繘绋嬮€€鍑烘椂鍏抽棴宸插缓绔嬬殑杩炴帴鎴栧彂閫佺獥鍙ｃ€?
娉ㄦ剰锛屽簲鐢ㄧ▼搴忓彲浠ョ敤鍚屼竴涓獥鍙ｅ彂閫佸涓姹傦紝涔熷彲浠ュ缓绔嬪涓獥鍙ｏ紝浣嗘瘡涓枃浠?鎻忚堪绗﹀搴斾竴涓獥鍙ｃ€?
浠ヤ笅鍚勮妭鎻愪緵鍏充簬鍚勪釜姝ラ鐨勬洿澶氱粏鑺傚拰鍙傝€冦€?

## NX-GZIP 璁惧鑺傜偣


绯荤粺涓湁涓€涓?/dev/crypto/nx-gzip 鑺傜偣锛屽畠鎻愪緵瀵圭郴缁熶腑鎵€鏈?GZIP 寮曟搸鐨勮闂€?瀵?/dev/crypto/nx-gzip 鍞竴鏈夋晥鐨勬搷浣滄槸锛?
 - 浠ヨ鍐欐柟寮?open() 璇ヨ澶囥€? - 鍙戝嚭 VAS_TX_WIN_OPEN ioctl
 - 鎶婂紩鎿庣殑璇锋眰闃熷垪 mmap() 鍒板簲鐢ㄧ▼搴忕殑铏氭嫙鍦板潃绌洪棿锛堝嵆鑾峰緱鍗忓鐞嗗櫒寮曟搸鐨?   paste_address锛夈€? - 鍏抽棴璇ヨ澶囪妭鐐广€?
璇ヨ澶囪妭鐐逛笂鐨勫叾浠栨枃浠舵搷浣滄槸鏈畾涔夌殑銆?
娉ㄦ剰锛宑opy 鍜?paste 鎿嶄綔鐩存帴鍙戝線纭欢锛屽苟涓嶇粡杩囪璁惧銆傛洿澶氱粏鑺傝鍙傝€?COPY/PASTE 鏂囨。銆?
灏界涓€涓郴缁熷彲鑳芥嫢鏈夊涓?NX 鍗忓鐞嗗櫒寮曟搸鐨勫疄渚嬶紙閫氬父姣忎釜 P9 鑺墖涓€涓級锛屼絾
绯荤粺涓彧鏈変竴涓?/dev/crypto/nx-gzip 璁惧鑺傜偣銆傚綋鎵撳紑 nx-gzip 璁惧鑺傜偣鏃讹紝鍐呮牳
鍦ㄤ竴涓悎閫傜殑 NX 鍔犻€熷櫒瀹炰緥涓婃墦寮€鍙戦€佺獥鍙ｃ€傚畠浼氭壘鍒扮敤鎴疯繘绋嬫鍦ㄥ叾涓婃墽琛岀殑 CPU锛?骞剁‘瀹氳 CPU 鎵€灞炵殑鐩稿簲鑺墖涓婄殑 NX 瀹炰緥銆?
搴旂敤绋嬪簭鍙互浣跨敤 VAS_TX_WIN_OPEN ioctl 涓殑 vas_id 瀛楁鏉ラ€夋嫨鐗瑰畾鐨?NX 鍗忓鐞嗗櫒
瀹炰緥锛岃瑙佷笅鏂囥€?
涓€涓悕涓?libnxz 鐨勭敤鎴风┖闂村簱鍙湪姝よ幏鍙栵紝浣嗕粛鍦ㄥ紑鍙戜腑锛?
	 https://github.com/abalib/power-gzip

浣跨敤 inflate / deflate 璋冪敤鐨勫簲鐢ㄧ▼搴忓彲浠ラ摼鎺?libnxz 鑰岄潪 libz锛屼粠鑰屾棤闇€浠讳綍
淇敼鍗冲彲浣跨敤 NX GZIP 鍘嬬缉銆?

## 鎵撳紑 /dev/crypto/nx-gzip


nx-gzip 璁惧搴斿綋浠ヨ鍐欐柟寮忔墦寮€銆傛墦寮€璇ヨ澶囦笉闇€瑕佺壒娈婃潈闄愩€傛瘡涓獥鍙ｅ搴斾竴涓枃浠?鎻忚堪绗︺€傛墍浠ュ鏋滅敤鎴风┖闂磋繘绋嬮渶瑕佸涓獥鍙ｏ紝灏卞繀椤诲彂鍑哄娆?open 璋冪敤銆?
鍏充簬杩斿洖鍊笺€侀敊璇爜鍜岄檺鍒剁瓑鍏朵粬缁嗚妭锛岃鍙傞槄 open(2) 绯荤粺璋冪敤鎵嬪唽椤点€?

## VAS_TX_WIN_OPEN ioctl


搴旂敤绋嬪簭搴斿綋濡備笅浣跨敤 VAS_TX_WIN_OPEN ioctl 鏉ヤ笌 NX 鍗忓鐞嗗櫒寮曟搸寤虹珛杩炴帴锛?
```
		struct vas_tx_win_open_attr {
			__u32   version;
			__s16   vas_id; /* specific instance of vas or -1
						for default */
			__u16   reserved1;
			__u64   flags;	/* For future use */
			__u64   reserved2[6];
		};

	version:
		version 瀛楁鐩墠蹇呴』璁剧疆涓?1銆?	vas_id:
		濡傛灉浼犲叆 '-1'锛屽唴鏍稿皢灏芥渶澶у姫鍔涗负杩涚▼鍒嗛厤涓€涓渶浼樼殑 NX
		瀹炰緥銆傝閫夋嫨鐗瑰畾鐨?VAS 瀹炰緥锛岃鍙傝€冧笅鏂圭殑鈥滃彲鐢?VAS 寮曟搸鐨?		鍙戠幇鈥濅竴鑺傘€?
	flags銆乺eserved1 鍜?reserved2[6] 瀛楁鐢ㄤ簬鏈潵鐨勬墿灞曪紝蹇呴』璁剧疆涓?0銆?
	VAS_TX_WIN_OPEN ioctl 鐨勫睘鎬?attr 瀹氫箟濡備笅::

		#define VAS_MAGIC 'v'
		#define VAS_TX_WIN_OPEN _IOW(VAS_MAGIC, 1,
						struct vas_tx_win_open_attr)

		struct vas_tx_win_open_attr attr;
		rc = ioctl(fd, VAS_TX_WIN_OPEN, &attr);

	VAS_TX_WIN_OPEN ioctl 鎴愬姛鏃惰繑鍥?0銆傚嚭閿欐椂锛岃繑鍥?-1 骞惰缃?errno
	鍙橀噺浠ユ寚绀洪敊璇€?
	閿欒鏉′欢锛?
		======	================================================
		EINVAL	fd 涓嶆寚鍚戜竴涓湁鏁堢殑 VAS 璁惧銆?		EINVAL	鏃犳晥鐨?vas ID
		EINVAL	version 鏈缃负姝ｇ‘鐨勫€?		EEXIST	缁欏畾鐨?fd 宸茬粡鎵撳紑浜嗙獥鍙?		ENOMEM	娌℃湁鍙敤鍐呭瓨鏉ュ垎閰嶇獥鍙?		ENOSPC	绯荤粺宸叉墦寮€鐨勬椿璺冪獥鍙ｏ紙杩炴帴锛夎繃澶?		EINVAL	淇濈暀瀛楁鏈璁剧疆涓?0銆?		======	================================================

	鍏充簬鏇村缁嗚妭銆侀敊璇爜鍜岄檺鍒讹紝璇峰弬闃?ioctl(2) 鎵嬪唽椤点€?
```
## mmap() NX-GZIP 璁惧


閽堝 NX-GZIP 璁惧 fd 鐨?mmap() 绯荤粺璋冪敤杩斿洖涓€涓?paste_address锛屽簲鐢ㄧ▼搴忓彲
鐢ㄥ畠鎶?CRB 澶嶅埗/绮樿创鍒扮‖浠跺紩鎿庛€?
```
		paste_addr = mmap(addr, size, prot, flags, fd, offset);

	瀵?NX-GZIP 璁惧 fd 杩涜 mmap 鐨勫敮涓€闄愬埗鏄細

		* size 搴斾负 PAGE_SIZE
		* offset 鍙傛暟搴斾负 0ULL

	鍏充簬鏇村缁嗚妭/闄愬埗锛岃鍙傞槄 mmap(2) 鎵嬪唽椤点€傞櫎浜?mmap(2) 鎵嬪唽椤典笂
	鍒楀嚭鐨勯敊璇潯浠朵箣澶栵紝涔熷彲鑳藉洜浠ヤ笅鏌愪釜閿欒鐮佽€屽け璐ワ細

		======	=============================================
		EINVAL	fd 娌℃湁鍏宠仈涓€涓凡鎵撳紑鐨勭獥鍙?			锛堝嵆 mmap() 娌℃湁璺熷湪鎴愬姛鐨?VAS_TX_WIN_OPEN
			ioctl 璋冪敤涔嬪悗锛夈€?		EINVAL	offset 瀛楁涓嶆槸 0ULL銆?		======	=============================================

```
## 鍙敤 VAS 寮曟搸鐨勫彂鐜?

绯荤粺涓殑姣忎釜鍙敤 VAS 瀹炰緥閮戒細鏈変竴涓澶囨爲鑺傜偣锛屼緥濡?/proc/device-tree/vas@** 鎴?/proc/device-tree/xscom@**/vas@*銆傜‘瀹氳姱鐗囨垨 VAS
瀹炰緥锛屽苟浣跨敤璇ヨ妭鐐逛腑鐨?ibm,vas-id 灞炴€у€兼潵閫夋嫨鐗瑰畾鐨?VAS 瀹炰緥銆?

## Copy/Paste 鎿嶄綔


搴旂敤绋嬪簭搴斿綋浣跨敤 copy 鍜?paste 鎸囦护鏉ュ悜 NX 鍙戦€?CRB銆傚叧浜?Copy/Paste 鎸囦护锛?璇峰弬闃?PowerISA 鐨勭 4.4 鑺傦細
https://openpowerfoundation.org/?resource_lib=power-isa-version-3-0


## CRB 瑙勮寖涓庝娇鐢?NX


搴旂敤绋嬪簭搴斿綋浣跨敤鍗忓鐞嗗櫒璇锋眰鍧楋紙CRB锛夋潵鏍煎紡鍖栧彂寰€鍗忓鐞嗗櫒鐨勮姹傘€傚叧浜?CRB 鐨?鏍煎紡浠ュ強浠庣敤鎴风┖闂翠娇鐢?NX锛堜緥濡傚彂閫佽姹傚拰妫€鏌ヨ姹傜姸鎬侊級锛岃鍙傞槄 NX-GZIP 鐢ㄦ埛
鎵嬪唽銆?

## NX 閿欒澶勭悊


搴旂敤绋嬪簭鍚?NX 鍙戦€佽姹傦紝骞堕€氳繃杞鍗忓鐞嗗櫒鐘舵€佸潡锛圕SB锛夋爣蹇楁潵绛夊緟鐘舵€併€侼X 鍦?姣忎釜璇锋眰澶勭悊瀹屾垚鍚庢洿鏂?CSB 涓殑鐘舵€併€傚叧浜?CSB 鐨勬牸寮忓拰鐘舵€佹爣蹇楋紝璇峰弬闃?NX-GZIP
鐢ㄦ埛鎵嬪唽銆?
濡傛灉 NX 鍦?CSB 鍦板潃鎴栦换浣曡姹傜紦鍐插尯涓婇亣鍒拌浆鎹㈤敊璇紙绉颁负 NX 椤甸敊璇級锛屽氨浼氬湪
CPU 涓婂紩鍙戜竴涓腑鏂潵澶勭悊璇ラ敊璇€傚鏋滃簲鐢ㄧ▼搴忎紶鍏ヤ簡鏃犳晥鍦板潃锛屾垨鑰呰姹傜紦鍐插尯
涓嶅湪鍐呭瓨涓紝灏卞彲鑳藉彂鐢熼〉閿欒銆傛搷浣滅郴缁熼€氳繃浠ヤ笅鏂瑰紡澶勭悊璇ラ敊璇細
```
	csb.flags = CSB_V;
	csb.cc = CSB_CC_FAULT_ADDRESS;
	csb.ce = CSB_CE_TERMINATION;
	csb.address = fault_address;
```
褰撳簲鐢ㄧ▼搴忔敹鍒拌浆鎹㈤敊璇椂锛屽畠鍙互瑙﹀強鎴栬闂甫鏈夐敊璇湴鍧€鐨勯〉锛屼娇鍏朵綅浜庡唴瀛樹腑銆?鐒跺悗搴旂敤绋嬪簭鍙互閲嶆柊鍚?NX 鍙戦€佽璇锋眰銆?
濡傛灉鎿嶄綔绯荤粺鐢变簬鏃犳晥鐨?CSB 鍦板潃鑰屾棤娉曟洿鏂?CSB锛屽氨浼氬悜鎵撳紑鍙戦€佺獥鍙ｇ殑杩涚▼鍙戦€?SEGV 淇″彿锛屽師濮嬭姹傛鏄€氳繃璇ョ獥鍙ｅ彂鍑虹殑锛?```
	siginfo.si_signo = SIGSEGV;
	siginfo.si_errno = EFAULT;
	siginfo.si_code = SEGV_MAPERR;
	siginfo.si_addr = CSB address;
```
瀵逛簬澶氱嚎绋嬪簲鐢ㄧ▼搴忥紝NX 鍙戦€佺獥鍙ｅ彲浠ュ湪鎵€鏈夌嚎绋嬩箣闂村叡浜€備緥濡傦紝涓€涓瓙绾跨▼鍙互
鎵撳紑涓€涓彂閫佺獥鍙ｏ紝浣嗗叾浠栫嚎绋嬪彲浠ヤ娇鐢ㄨ繖涓獥鍙ｅ悜 NX 鍙戦€佽姹傘€傚彧瑕?CSB 鍦板潃鏈夋晥锛?杩欎簺璇锋眰鍗充娇鍦ㄦ搷浣滅郴缁熷鐞嗛敊璇殑鎯呭舰涓嬩篃浼氭垚鍔熴€傚鏋?NX 璇锋眰鍖呭惈鏃犳晥鐨?CSB 鍦板潃锛?淇″彿灏嗚鍙戦€佺粰鎵撳紑璇ョ獥鍙ｇ殑瀛愮嚎绋嬨€備絾濡傛灉璇ョ嚎绋嬪湪娌℃湁鍏抽棴绐楀彛鐨勬儏鍐典笅閫€鍑猴紝骞朵笖
璇锋眰鏄娇鐢ㄨ繖涓獥鍙ｅ彂鍑虹殑锛屼俊鍙峰皢琚彂缁欑嚎绋嬬粍缁勯暱锛坱gid锛夈€傚簲鐢ㄧ▼搴忓彲浠ュ拷鐣ユ垨
澶勭悊杩欎簺淇″彿锛岀敱搴旂敤绋嬪簭鑷鍐冲畾銆?
NX-GZIP 鐢ㄦ埛鎵嬪唽锛?https://github.com/libnxz/power-gzip/blob/master/doc/power_nx_gzip_um.pdf


## 绠€鍗曠ず渚?

```
		int use_nx_gzip()
		{
			int rc, fd;
			void *addr;
			struct vas_setup_attr txattr;

			fd = open("/dev/crypto/nx-gzip", O_RDWR);
			if (fd < 0) {
				fprintf(stderr, "open nx-gzip failed\n");
				return -1;
			}
			memset(&txattr, 0, sizeof(txattr));
			txattr.version = 1;
			txattr.vas_id = -1
			rc = ioctl(fd, VAS_TX_WIN_OPEN,
					(unsigned long)&txattr);
			if (rc < 0) {
				fprintf(stderr, "ioctl() n %d, error %d\n",
						rc, errno);
				return rc;
			}
			addr = mmap(NULL, 4096, PROT_READ|PROT_WRITE,
					MAP_SHARED, fd, 0ULL);
			if (addr == MAP_FAILED) {
				fprintf(stderr, "mmap() failed, errno %d\n",
						errno);
				return -errno;
			}
			do {
				//Format CRB request with compression or
				//uncompression
				// Refer tests for vas_copy/vas_paste
				vas_copy((&crb, 0, 1);
				vas_paste(addr, 0, 1);
				// Poll on csb.flags with timeout
				// csb address is listed in CRB
			} while (true)
			close(fd) or window can be closed upon process exit
		}

	Refer https://github.com/libnxz/power-gzip for tests or more
	use cases.

```
