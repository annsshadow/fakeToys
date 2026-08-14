

######## 缂撳啿鍖?

缂撳啿鍖哄寘鍚敱搴旂敤绋嬪簭涓庨┍鍔ㄩ€氳繃鏌愪竴绉嶆祦寮?I/O锛圫treaming I/O锛夋柟娉曚氦鎹㈢殑鏁版嵁銆傚湪澶氬钩闈紙multi-planar锛堿PI 涓紝鏁版嵁淇濆瓨鍦ㄥ钩闈紙planes锛変腑锛岃€岀紦鍐插尯缁撴瀯浣撳垯鍏呭綋杩欎簺骞抽潰鐨勫鍣ㄣ€傚彧浜ゆ崲鎸囧悜缂撳啿鍖猴紙骞抽潰锛夌殑鎸囬拡锛屾暟鎹湰韬笉浼氳澶嶅埗銆傝繖浜涙寚閽堣繛鍚屾椂闂存埑鎴栧満濂囧伓鎬х瓑鍏冧俊鎭竴璧凤紝琚瓨鍌ㄥ湪缁撴瀯浣?`v4l2_buffer` 涓紝璇ョ粨鏋勪綋鏄?VIDIOC_QUERYBUF銆乂IDIOC_QBUF <VIDIOC_QBUF> 浠ュ強 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 鐨勫弬鏁般€傚湪澶氬钩闈?API 涓紝`v4l2_buffer` 缁撴瀯浣撻噷涓€浜涚壒瀹氫簬骞抽潰鐨勬垚鍛橈紙濡傛瘡涓钩闈㈢殑鎸囬拡鍜屽ぇ灏忥級鏀逛负瀛樺偍鍦ㄧ粨鏋勪綋 `v4l2_plane` 涓€傚湪杩欑鎯呭喌涓嬶紝`v4l2_buffer` 缁撴瀯浣撳寘鍚竴涓钩闈㈢粨鏋勪綋鏁扮粍銆?
鍑洪槦鐨勮棰戠紦鍐插尯甯︽湁鏃堕棿鎴炽€傜敱椹卞姩鍐冲畾鍦ㄥ抚鐨勫摢涓€閮ㄥ垎銆佷娇鐢ㄥ摢涓椂閽熸潵閲囬泦鏃堕棿鎴炽€傝鍙傞槄 buffer-flags 涓帺鐮?`V4L2_BUF_FLAG_TIMESTAMP_MASK` 涓?`V4L2_BUF_FLAG_TSTAMP_SRC_MASK` 閲岀殑鏍囧織浣嶃€傚湪鏁翠釜瑙嗛娴佹湡闂达紝杩欎簺鏍囧織浣嶅鎵€鏈夌紦鍐插尯濮嬬粓鏈夋晥涓斾繚鎸佷笉鍙樸€備笉杩囷紝浣滀负 VIDIOC_S_INPUT <VIDIOC_G_INPUT> 鎴?VIDIOC_S_OUTPUT <VIDIOC_G_OUTPUT> 鐨勫壇浣滅敤锛岃繖浜涙爣蹇椾綅鍙兘浼氬彂鐢熷彉鍖栥€傝鍒欑殑涓€涓緥澶栨槸 `V4L2_BUF_FLAG_TIMESTAMP_COPY` 鏃堕棿鎴崇被鍨嬶紙渚嬪鐢ㄤ簬 mem-to-mem 璁惧锛夛細鏃堕棿鎴虫簮鏍囧織浣嶄細浠?OUTPUT 瑙嗛缂撳啿鍖哄鍒跺埌 CAPTURE 瑙嗛缂撳啿鍖恒€?
## 鏍煎紡銆佹帶浠朵笌缂撳啿鍖轰箣闂寸殑浜や簰


V4L2 鏆撮湶浜嗕竴浜涗細褰卞搷缂撳啿鍖哄ぇ灏忔垨鏁版嵁鍦ㄧ紦鍐插尯涓竷灞€鏂瑰紡鐨勫弬鏁般€傝繖浜涘弬鏁版棦閫氳繃鏍煎紡涔熼€氳繃鎺т欢鏉ユ毚闇层€傛绫绘帶浠剁殑涓€涓緥瀛愭槸 `V4L2_CID_ROTATE` 鎺т欢锛屽畠浼氫慨鏀瑰儚绱犲湪缂撳啿鍖轰腑瀛樺偍鐨勬柟鍚戯紝骞跺湪鎵€閫夋牸寮忓湪琛屽熬鍖呭惈濉厖鏃跺悓鏃朵慨鏀圭紦鍐插尯澶у皬銆?
瑙ｉ噴缂撳啿鍖哄唴瀹规墍闇€鐨勪竴缁勪俊鎭紙渚嬪鍍忕礌鏍煎紡銆佽姝ラ暱銆佸钩閾烘柟鍚戞垨鏃嬭浆锛夊湪鏈妭鍏朵綑閮ㄥ垎缁熺О涓虹紦鍐插尯甯冨眬锛坆uffer layout锛夈€?
鍙互淇敼缂撳啿鍖哄竷灞€鐨勬帶浠跺簲褰撹缃?`V4L2_CTRL_FLAG_MODIFY_LAYOUT` 鏍囧織銆?
淇敼浼氬奖鍝嶇紦鍐插尯澶у皬鎴栧竷灞€鐨勬牸寮忔垨鎺т欢瑕佹眰鍏堝仠姝㈡祦銆備换浣曞湪娴佸浜庢椿鍔ㄧ姸鎬佹椂灏濊瘯鍋氭绫讳慨鏀圭殑琛屼负锛岄兘搴斾娇璁剧疆鏍煎紡鎴栨帶浠剁殑 ioctl 杩斿洖 `EBUSY` 閿欒鐮併€傚湪杩欑鎯呭喌涓嬶紝褰撴祦澶勪簬娲诲姩鐘舵€佹椂椹卞姩閽堝姝ょ被鎺т欢璋冪敤 `VIDIOC_QUERYCTRL` 鎴?`VIDIOC_QUERY_EXT_CTRL` 杩樺簲褰撹缃?`V4L2_CTRL_FLAG_GRABBED` 鏍囧織銆?

   `VIDIOC_S_SELECTION` ioctl 鍙兘锛堝彇鍐充簬纭欢锛屼緥濡傝澶囦笉鍖呭惈缂╂斁鍣ㄦ椂锛夊湪淇敼閫夋嫨鐭╁舰鐨勫悓鏃朵慨鏀规牸寮忋€傜被浼煎湴锛宍VIDIOC_S_INPUT`銆乣VIDIOC_S_OUTPUT`銆乣VIDIOC_S_STD` 鍜?`VIDIOC_S_DV_TIMINGS` ioctl 涔熷彲浠ヤ慨鏀规牸寮忓拰閫夋嫨鐭╁舰銆傚綋杩欎簺 ioctl 瀵艰嚧缂撳啿鍖哄ぇ灏忔垨甯冨眬鍙戠敓鍙樺寲鏃讹紝椹卞姩搴斿綋鎸夌収鏈妭鎵€鎻忚堪鐨勫悇绉嶆儏鍐典腑澶勭悊 `VIDIOC_S_FMT` ioctl 鐨勬柟寮忔潵搴斿璇ョ姸鍐点€?
鍙奖鍝嶇紦鍐插尯甯冨眬鐨勬帶浠跺彲浠ュ湪娴佸仠姝㈠悗鐨勪换鎰忔椂鍒讳慨鏀广€傜敱浜庡畠浠笉褰卞搷缂撳啿鍖哄ぇ灏忥紝鍥犳涓嶉渶瑕佷换浣曠壒娈婄殑澶勭悊鏉ュ皢杩欎簺鎺т欢涓庣紦鍐插尯鍒嗛厤鍚屾锛屽苟涓斾竴鏃︽祦鍋滄锛宍V4L2_CTRL_FLAG_GRABBED` 鏍囧織鍗宠娓呴櫎銆?
褰卞搷缂撳啿鍖哄ぇ灏忕殑鏍煎紡鍜屾帶浠朵細涓庣紦鍐插尯鍒嗛厤鐩镐簰浣滅敤銆傛渶绠€鍗曠殑澶勭悊鏂瑰紡鏄┍鍔ㄥ缁堣姹傞噸鏂板垎閰嶇紦鍐插尯锛屼互渚挎洿鏀硅繖浜涙牸寮忔垨鎺т欢銆傚湪杩欑鎯呭喌涓嬶紝瑕佽繘琛屾绫绘洿鏀癸紝鐢ㄦ埛绌洪棿搴旂敤绋嬪簭搴斿厛鍦ㄦ祦杩愯鏃剁敤 `VIDIOC_STREAMOFF` ioctl 鍋滄瑙嗛娴侊紝骞跺湪缂撳啿鍖哄凡鍒嗛厤鏃剁敤 `VIDIOC_REQBUFS` ioctl 閲婃斁鎵€鏈夌紦鍐插尯銆傞噴鏀炬墍鏈夌紦鍐插尯鍚庯紝鎺т欢鐨?`V4L2_CTRL_FLAG_GRABBED` 鏍囧織琚竻闄ゃ€傜劧鍚庡彲浠ヤ慨鏀规牸寮忔垨鎺т欢锛岄殢鍚庡簲閲嶆柊鍒嗛厤缂撳啿鍖哄苟閲嶆柊鍚姩娴併€備竴涓吀鍨嬬殑 ioctl 搴忓垪涓?
 #. VIDIOC_STREAMOFF
 #. VIDIOC_REQBUFS(0)
 #. VIDIOC_S_EXT_CTRLS
 #. VIDIOC_S_FMT
 #. VIDIOC_REQBUFS(n)
 #. VIDIOC_QBUF
 #. VIDIOC_STREAMON

绗簩娆?`VIDIOC_REQBUFS` 璋冪敤浼氬皢鏂扮殑鏍煎紡鍜屾帶浠跺€艰€冭檻鍦ㄥ唴锛屼互璁＄畻瑕佸垎閰嶇殑缂撳啿鍖哄ぇ灏忋€傚鏈夐渶瑕侊紝搴旂敤绋嬪簭涔熷彲浠ラ€氳繃璋冪敤 `VIDIOC_G_FMT` ioctl 鏉ヨ幏鍙栬澶у皬銆?

   璇?API 骞舵湭寮哄埗瑙勫畾涓婅堪鎺т欢锛?.锛変笌鏍煎紡锛?.锛夋洿鏀圭殑椤哄簭銆傛牸寮忓拰鎺т欢鍙互鎸変笉鍚岄『搴忚缃紝鐢氳嚦鍙互浜ら敊璁剧疆锛屽叿浣撳彇鍐充簬璁惧鍜岀敤渚嬨€備緥濡傛煇浜涙帶浠跺浜庝笉鍚岀殑鍍忕礌鏍煎紡鍙兘琛ㄧ幇涓嶅悓锛屽湪杩欑鎯呭喌涓嬪彲鑳介渶瑕佸厛璁剧疆鏍煎紡銆?
褰撻渶瑕侀噸鏂板垎閰嶆椂锛屼换浣曞湪缂撳啿鍖哄凡鍒嗛厤鐨勬儏鍐典笅灏濊瘯淇敼褰卞搷缂撳啿鍖哄ぇ灏忕殑鏍煎紡鎴栨帶浠剁殑琛屼负锛岄兘搴斾娇璁剧疆鏍煎紡鎴栨帶浠剁殑 ioctl 杩斿洖 `EBUSY` 閿欒銆備换浣曞皾璇曞皢瀵逛簬褰撳墠鏍煎紡鎴栨帶浠惰€岃█澶皬鐨勭紦鍐插尯鍏ラ槦鐨勮涓猴紝閮藉簲浣?`VIDIOC_QBUF` ioctl 杩斿洖 `EINVAL` 閿欒銆?
缂撳啿鍖洪噸鏂板垎閰嶆槸涓€椤瑰紑閿€杈冨ぇ鐨勬搷浣溿€備负閬垮厤璇ュ紑閿€锛岄┍鍔ㄥ彲浠ワ紙骞朵笖琚紦鍔憋級鍏佽鍦ㄧ紦鍐插尯宸插垎閰嶇殑鎯呭喌涓嬫洿鏀瑰奖鍝嶇紦鍐插尯澶у皬鐨勬牸寮忔垨鎺т欢銆傚湪杩欑鎯呭喌涓嬶紝淇敼鏍煎紡鍜屾帶浠剁殑鍏稿瀷 ioctl 搴忓垪涓?
 #. VIDIOC_STREAMOFF
 #. VIDIOC_S_EXT_CTRLS
 #. VIDIOC_S_FMT
 #. VIDIOC_QBUF
 #. VIDIOC_STREAMON

涓轰娇璇ュ簭鍒楁纭繍琛岋紝宸插叆闃熺殑缂撳啿鍖哄繀椤昏冻澶熷ぇ浠ュ绾虫柊鏍煎紡鎴栨帶浠躲€傚鏋滃綋鍓嶅凡鍏ラ槦鐨勭紦鍐插尯瀵逛簬鏂版牸寮忚€岃█澶皬锛岄┍鍔ㄥ簲褰撳湪鍝嶅簲鏍煎紡鏇存敼锛坄VIDIOC_S_FMT`锛夋垨鎺т欢鏇存敼锛坄VIDIOC_S_CTRL` 鎴?`VIDIOC_S_EXT_CTRLS`锛夋椂杩斿洖 `ENOSPC` 閿欒銆備綔涓虹畝鍖栵紝椹卞姩濡傛灉褰撳墠鏈変换浣曠紦鍐插尯宸插叆闃燂紝涔熷彲浠ヤ笉妫€鏌ュ凡鍏ラ槦缂撳啿鍖虹殑澶у皬鑰岀洿鎺ヤ粠杩欎簺 ioctl 杩斿洖 `EBUSY` 閿欒銆?
姝ゅ锛屽鏋滄鍦ㄥ叆闃熺殑缂撳啿鍖哄浜庡綋鍓嶆牸寮忔垨鎺т欢鑰岃█澶皬锛岄┍鍔ㄥ簲浠?`VIDIOC_QBUF` ioctl 杩斿洖 `EINVAL` 閿欒銆傝繖浜涜姹傚叡鍚岀‘淇濆凡鍏ラ槦鐨勭紦鍐插尯濮嬬粓瓒冲澶т互瀹圭撼鎵€閰嶇疆鐨勬牸寮忓拰鎺т欢銆?
鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍙互閫氳繃灏嗘墍闇€鎺т欢鍊煎厛璁剧疆濂斤紝鐒跺悗灏濊瘯鎵€闇€鏍煎紡锛屾潵鏌ヨ缁欏畾鏍煎紡鍜屾帶浠舵墍闇€鐨勭紦鍐插尯澶у皬銆俙VIDIOC_TRY_FMT` ioctl 灏嗚繑鍥炴墍闇€鐨勭紦鍐插尯澶у皬銆?
 #. VIDIOC_S_EXT_CTRLS(x)
 #. VIDIOC_TRY_FMT()
 #. VIDIOC_S_EXT_CTRLS(y)
 #. VIDIOC_TRY_FMT()

闅忓悗鍙互浣跨敤 `VIDIOC_CREATE_BUFS` ioctl 鍩轰簬鏌ヨ鍒扮殑灏哄鏉ュ垎閰嶇紦鍐插尯锛堜緥濡傚垎閰嶄竴缁勫鎵€鏈夋墍闇€鏍煎紡鍜屾帶浠堕兘瓒冲澶х殑缂撳啿鍖猴紝鎴栬€呴拡瀵规瘡涓敤渚嬪垎閰嶄竴缁勫昂瀵稿悎閫傜殑鍗曠嫭缂撳啿鍖猴級銆?

## struct v4l2_buffer



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 2 10

    - - __u32
      - `index`
      - 缂撳啿鍖虹殑缂栧彿锛岀敱搴旂敤绋嬪簭璁剧疆锛屼絾鍦ㄨ皟鐢?	VIDIOC_DQBUF <VIDIOC_QBUF> 鏃剁敱椹卞姩璁剧疆銆傝瀛楁鐨勫彇鍊艰寖鍥翠粠闆跺埌閫氳繃
	VIDIOC_REQBUFS ioctl 鍒嗛厤鐨勭紦鍐插尯鏁伴噺锛堢粨鏋勪綋 `v4l2_requestbuffers`
	鐨?`count` 瀛楁锛夛紝鍐嶅姞涓婇€氳繃
	VIDIOC_CREATE_BUFS 鍒嗛厤鐨勭紦鍐插尯鏁伴噺鍑忎竴銆?    - - __u32
      - `type`
      - 缂撳啿鍖虹殑绫诲瀷锛屼笌缁撴瀯浣?	`v4l2_format` 鐨?`type` 瀛楁鎴栫粨鏋勪綋
	`v4l2_requestbuffers` 鐨?`type` 瀛楁鐩稿悓锛岀敱搴旂敤绋嬪簭璁剧疆銆傚弬瑙?`v4l2_buf_type`
    - - __u32
      - `bytesused`
      - 缂撳啿鍖轰腑鏁版嵁鎵€鍗犵敤鐨勫瓧鑺傛暟銆傚畠鍙栧喅浜庡崗鍟嗗緱鍒扮殑鏁版嵁鏍煎紡锛屽浜?JPEG
	杩欑被鍘嬬缉鐨勫彲鍙樺ぇ灏忔暟鎹紝姣忎釜缂撳啿鍖虹殑鍊煎彲鑳戒笉鍚屻€傚綋 `type` 鎸囧悜閲囬泦锛坈apture锛夋祦鏃堕┍鍔ㄥ繀椤昏缃瀛楁锛屽綋瀹冩寚鍚戣緭鍑猴紙output锛夋祦鏃剁敱搴旂敤绋嬪簭璁剧疆銆傚浜庡骞抽潰鏍煎紡锛屾瀛楁琚拷鐣ワ紝鏀圭敤
	`planes` 鎸囬拡銆?    - - __u32
      - `flags`
      - 鐢卞簲鐢ㄧ▼搴忔垨椹卞姩璁剧疆鐨勬爣蹇椾綅锛屽弬瑙?buffer-flags銆?    - - __u32
      - `field`
      - 鎸囩ず缂撳啿鍖轰腑鍥惧儚鐨勫満椤哄簭锛屽弬瑙?	`v4l2_field`銆傚綋缂撳啿鍖哄寘鍚?VBI 鏁版嵁鏃舵瀛楁涓嶄娇鐢ㄣ€傚綋 `type`
	鎸囧悜閲囬泦娴佹椂椹卞姩蹇呴』璁剧疆瀹冿紝褰撳畠鎸囧悜杈撳嚭娴佹椂鐢卞簲鐢ㄧ▼搴忚缃€?    - - struct timeval
      - `timestamp`
      - 瀵逛簬閲囬泦娴侊紝杩欐槸鎹曡幏绗竴涓暟鎹瓧鑺傜殑鏃堕棿锛岀敱
	`clock_gettime()` 鍑芥暟閽堝鐩稿簲鏃堕挓 id 杩斿洖锛涘弬瑙?buffer-flags 涓殑
	`V4L2_BUF_FLAG_TIMESTAMP_*`銆傚浜庤緭鍑烘祦锛岄┍鍔ㄥ皢鏈€鍚庝竴涓暟鎹瓧鑺傚疄闄呭彂閫佸嚭鍘荤殑鏃堕棿瀛樺叆
	`timestamp` 瀛楁銆傝繖浣垮簲鐢ㄧ▼搴忚兘澶熺洃娴嬭棰戞椂閽熶笌绯荤粺鏃堕挓涔嬮棿鐨勬紓绉汇€傚浜庝娇鐢?	`V4L2_BUF_FLAG_TIMESTAMP_COPY` 鐨勮緭鍑烘祦锛屽簲鐢ㄧ▼搴忓繀椤诲～鍏ユ椂闂存埑锛岄┍鍔ㄤ細灏嗗叾澶嶅埗鍒伴噰闆嗘祦銆?    - - struct `v4l2_timecode`
      - `timecode`
      - 褰?`flags` 涓缃簡 `V4L2_BUF_FLAG_TIMECODE` 鏍囧織鏃讹紝璇ョ粨鏋勪綋鍖呭惈涓€涓抚鏃堕棿鐮併€傚湪
	`V4L2_FIELD_ALTERNATE <v4l2_field>` 妯″紡涓嬶紝椤跺満鍜屽簳鍦哄寘鍚浉鍚岀殑鏃堕棿鐮併€傛椂闂寸爜鏃ㄥ湪杈呭姪瑙嗛缂栬緫锛岄€氬父璁板綍鍦ㄥ綍鍍忓甫涓婏紝浣嗕篃鍙祵鍏ュ埌 MPEG 绛夊帇缂╂牸寮忎腑銆傛瀛楁鐙珛浜?	`timestamp` 鍜?`sequence` 瀛楁銆?    - - __u32
      - `sequence`
      - 鐢遍┍鍔ㄨ缃紝瀵瑰抚锛堣€岄潪鍦猴紒锛夐『搴忚鏁般€傝瀛楁瀵硅緭鍏ヨ澶囧拰杈撳嚭璁惧閮戒細璁剧疆銆?    - - `2`

	鍦?`V4L2_FIELD_ALTERNATE <v4l2_field>` 妯″紡涓嬶紝椤跺満鍜屽簳鍦哄叿鏈夌浉鍚岀殑搴忓垪鍙枫€傝鏁颁粠闆跺紑濮嬶紝骞跺寘鍚涪寮冩垨閲嶅鐨勫抚銆備涪寮冪殑甯ф槸杈撳叆璁惧宸叉帴鏀跺埌浣嗗洜缂哄皯绌洪棽缂撳啿鍖虹┖闂磋€屾棤娉曞瓨鍌ㄧ殑甯с€傞噸澶嶇殑甯ф槸杈撳嚭璁惧鍥犲簲鐢ㄧ▼搴忔湭鑳藉強鏃朵紶閫佹柊鏁版嵁鑰屽啀娆℃樉绀虹殑甯с€?
```

	   This may count the frames received e.g. over USB, without
	   taking into account the frames dropped by the remote hardware due
	   to limited compression throughput or bus bandwidth. These devices
	   identify by not enumerating any video standards, see
	   :ref:`standard`.

    * - __u32
      - ``memory``
      - This field must be set by applications and/or drivers in
	accordance with the selected I/O method. See :c:type:`v4l2_memory`
    * - union {
      - ``m``
    * - __u32
      - ``offset``
      - For the single-planar API and when ``memory`` is
	``V4L2_MEMORY_MMAP`` this is the offset of the buffer from the
	start of the device memory. The value is returned by the driver
	and apart of serving as parameter to the
	:c:func:`mmap()` function not useful for applications.
	See :ref:`mmap` for details
    * - unsigned long
      - ``userptr``
      - For the single-planar API and when ``memory`` is
	``V4L2_MEMORY_USERPTR`` this is a pointer to the buffer (casted to
	unsigned long type) in virtual memory, set by the application. See
	:ref:`userp` for details.
    * - struct v4l2_plane
      - ``*planes``
      - When using the multi-planar API, contains a userspace pointer to
	an array of struct :c:type:`v4l2_plane`. The size of
	the array should be put in the ``length`` field of this
	struct :c:type:`v4l2_buffer` structure.
    * - int
      - ``fd``
      - For the single-plane API and when ``memory`` is
	``V4L2_MEMORY_DMABUF`` this is the file descriptor associated with
	a DMABUF buffer.
    * - }
      -
    * - __u32
      - ``length``
      - Size of the buffer (not the payload) in bytes for the
	single-planar API. This is set by the driver based on the calls to
	:ref:`VIDIOC_REQBUFS` and/or
	:ref:`VIDIOC_CREATE_BUFS`. For the
	multi-planar API the application sets this to the number of
	elements in the ``planes`` array. The driver will fill in the
	actual number of valid elements in that array.
    * - __u32
      - ``reserved2``
      - A place holder for future extensions. Drivers and applications
	must set this to 0.
    * - __u32
      - ``request_fd``
      - The file descriptor of the request to queue the buffer to. If the flag
        ``V4L2_BUF_FLAG_REQUEST_FD`` is set, then the buffer will be
	queued to this request. If the flag is not set, then this field will
	be ignored.

	The ``V4L2_BUF_FLAG_REQUEST_FD`` flag and this field are only used by
	:ref:`ioctl VIDIOC_QBUF <VIDIOC_QBUF>` and ignored by other ioctls that
	take a :c:type:`v4l2_buffer` as argument.

	Applications should not set ``V4L2_BUF_FLAG_REQUEST_FD`` for any ioctls
	other than :ref:`VIDIOC_QBUF <VIDIOC_QBUF>`.

	If the device does not support requests, then ``EBADR`` will be returned.
	If requests are supported but an invalid request file descriptor is
	given, then ``EINVAL`` will be returned.


```

## struct v4l2_plane



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `bytesused`
      - 璇ュ钩闈腑鏁版嵁锛堝嵆鍏舵湁鏁堣浇鑽凤級鎵€鍗犵敤鐨勫瓧鑺傛暟銆傚綋 `type`
	鎸囧悜閲囬泦娴佹椂椹卞姩蹇呴』璁剧疆姝ゅ瓧娈碉紝褰撳畠鎸囧悜杈撳嚭娴佹椂鐢卞簲鐢ㄧ▼搴忚缃€?
```

	   Note that the actual image data starts at ``data_offset``
	   which may not be 0.
    * - __u32
      - ``length``
      - Size in bytes of the plane (not its payload). This is set by the
	driver based on the calls to
	:ref:`VIDIOC_REQBUFS` and/or
	:ref:`VIDIOC_CREATE_BUFS`.
    * - union {
      - ``m``
    * - __u32
      - ``mem_offset``
      - When the memory type in the containing struct
	:c:type:`v4l2_buffer` is ``V4L2_MEMORY_MMAP``, this
	is the value that should be passed to :c:func:`mmap()`,
	similar to the ``offset`` field in struct
	:c:type:`v4l2_buffer`.
    * - unsigned long
      - ``userptr``
      - When the memory type in the containing struct
	:c:type:`v4l2_buffer` is ``V4L2_MEMORY_USERPTR``,
	this is a userspace pointer to the memory allocated for this plane
	by an application.
    * - int
      - ``fd``
      - When the memory type in the containing struct
	:c:type:`v4l2_buffer` is ``V4L2_MEMORY_DMABUF``,
	this is a file descriptor associated with a DMABUF buffer, similar
	to the ``fd`` field in struct :c:type:`v4l2_buffer`.
    * - }
      -
    * - __u32
      - ``data_offset``
      - Offset in bytes to video data in the plane. Drivers must set this
	field when ``type`` refers to a capture stream, applications when
	it refers to an output stream.

	.. note::

	   That data_offset is included  in ``bytesused``. So the
	   size of the image in the plane is ``bytesused``-``data_offset``
	   at offset ``data_offset`` from the start of the plane.
    * - __u32
      - ``reserved[11]``
      - Reserved for future use. Should be zeroed by drivers and
	applications.


```

## enum v4l2_buf_type



    :header-rows:  0
    :stub-columns: 0
    :widths:       4 1 9

    - - `V4L2_BUF_TYPE_VIDEO_CAPTURE`
      - 1
      - 鍗曞钩闈㈣棰戦噰闆嗘祦鐨勭紦鍐插尯锛屽弬瑙?	capture銆?    - - `V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`
      - 9
      - 澶氬钩闈㈣棰戦噰闆嗘祦鐨勭紦鍐插尯锛屽弬瑙?	capture銆?    - - `V4L2_BUF_TYPE_VIDEO_OUTPUT`
      - 2
      - 鍗曞钩闈㈣棰戣緭鍑烘祦鐨勭紦鍐插尯锛屽弬瑙?	output銆?    - - `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`
      - 10
      - 澶氬钩闈㈣棰戣緭鍑烘祦鐨勭紦鍐插尯锛屽弬瑙?output銆?    - - `V4L2_BUF_TYPE_VIDEO_OVERLAY`
      - 3
      - 瑙嗛鍙犲姞锛坥verlay锛夌殑缂撳啿鍖猴紝鍙傝 overlay銆?    - - `V4L2_BUF_TYPE_VBI_CAPTURE`
      - 4
      - 鍘熷 VBI 閲囬泦娴佺殑缂撳啿鍖猴紝鍙傝 raw-vbi銆?    - - `V4L2_BUF_TYPE_VBI_OUTPUT`
      - 5
      - 鍘熷 VBI 杈撳嚭娴佺殑缂撳啿鍖猴紝鍙傝 raw-vbi銆?    - - `V4L2_BUF_TYPE_SLICED_VBI_CAPTURE`
      - 6
      - 鍒囩墖 VBI 閲囬泦娴佺殑缂撳啿鍖猴紝鍙傝 sliced銆?    - - `V4L2_BUF_TYPE_SLICED_VBI_OUTPUT`
      - 7
      - 鍒囩墖 VBI 杈撳嚭娴佺殑缂撳啿鍖猴紝鍙傝 sliced銆?    - - `V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY`
      - 8
      - 瑙嗛杈撳嚭鍙犲姞锛圤SD锛夌殑缂撳啿鍖猴紝鍙傝 osd銆?    - - `V4L2_BUF_TYPE_SDR_CAPTURE`
      - 11
      - 杞欢瀹氫箟鏃犵嚎鐢碉紙SDR锛夐噰闆嗘祦鐨勭紦鍐插尯锛屽弬瑙?	sdr銆?    - - `V4L2_BUF_TYPE_SDR_OUTPUT`
      - 12
      - 杞欢瀹氫箟鏃犵嚎鐢碉紙SDR锛夎緭鍑烘祦鐨勭紦鍐插尯锛屽弬瑙?sdr銆?    - - `V4L2_BUF_TYPE_META_CAPTURE`
      - 13
      - 鍏冩暟鎹噰闆嗙殑缂撳啿鍖猴紝鍙傝 metadata銆?    - - `V4L2_BUF_TYPE_META_OUTPUT`
      - 14
      - 鍏冩暟鎹緭鍑虹殑缂撳啿鍖猴紝鍙傝 metadata銆?


## 缂撳啿鍖烘爣蹇?


    \footnotesize



    :header-rows:  0
    :stub-columns: 0
    :widths:       65 18 70

    - .. _`V4L2-BUF-FLAG-MAPPED`:

      - `V4L2_BUF_FLAG_MAPPED`
      - 0x00000001
      - 缂撳啿鍖轰綅浜庤澶囧唴瀛樹腑锛屽苟宸叉槧灏勫埌搴旂敤绋嬪簭鐨勫湴鍧€绌洪棿锛岃瑙?mmap銆傞┍鍔ㄥ湪璋冪敤
	VIDIOC_QUERYBUF銆?	VIDIOC_QBUF 鎴?	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 鏃惰缃垨娓呴櫎姝ゆ爣蹇椼€傜敱椹卞姩璁剧疆銆?    - .. _`V4L2-BUF-FLAG-QUEUED`:

      - `V4L2_BUF_FLAG_QUEUED`
      - 0x00000002
      - 椹卞姩鍐呴儴缁存姢涓や釜缂撳啿鍖洪槦鍒楋細鍏ラ槦闃熷垪鍜屽嚭闃熼槦鍒椼€傚綋璁剧疆姝ゆ爣蹇楁椂锛岀紦鍐插尯褰撳墠浣嶄簬鍏ラ槦闃熷垪涓€傚湪缂撳啿鍖鸿濉厖锛堥噰闆嗚澶囷級鎴栨樉绀猴紙杈撳嚭璁惧锛夊悗锛屽畠浼氳嚜鍔ㄧЩ鍔ㄥ埌鍑洪槦闃熷垪銆傞┍鍔ㄥ湪璋冪敤
	`VIDIOC_QUERYBUF` ioctl 鏃惰缃垨娓呴櫎姝ゆ爣蹇椼€傛垚鍔熻皟鐢?	`VIDIOC_QBUF`\ ioctl 鍚庡畠鎬绘槸琚缃紝璋冪敤 `VIDIOC_DQBUF` 鍚庢€绘槸琚竻闄ゃ€?    - .. _`V4L2-BUF-FLAG-DONE`:

      - `V4L2_BUF_FLAG_DONE`
      - 0x00000004
      - 褰撹缃鏍囧織鏃讹紝缂撳啿鍖哄綋鍓嶄綅浜庡嚭闃熼槦鍒椾腑锛屽凡鍑嗗濂戒粠椹卞姩涓嚭闃熴€傞┍鍔ㄥ湪璋冪敤
	`VIDIOC_QUERYBUF` ioctl 鏃惰缃垨娓呴櫎姝ゆ爣蹇椼€傝皟鐢?`VIDIOC_QBUF` 鎴?	`VIDIOC_DQBUF` 鍚庡畠鎬绘槸琚竻闄ゃ€傚綋鐒讹紝缂撳啿鍖轰笉鍙兘鍚屾椂浣嶄簬涓や釜闃熷垪涓紝`V4L2_BUF_FLAG_QUEUED` 鍜?	`V4L2_BUF_FLAG_DONE` 鏍囧織鏄簰鏂ョ殑銆備笉杩囧畠浠篃鍙互閮借娓呴櫎锛屾鏃剁紦鍐插尯澶勪簬鈥滃凡鍑洪槦锛坉equeued锛夆€濈姸鎬侊紝鍗冲湪搴旂敤绋嬪簭鐨勭杈栬寖鍥村唴銆?    - .. _`V4L2-BUF-FLAG-ERROR`:

      - `V4L2_BUF_FLAG_ERROR`
      - 0x00000040
      - 褰撹缃鏍囧織鏃讹紝缂撳啿鍖哄凡鎴愬姛鍑洪槦锛屽敖绠℃暟鎹彲鑳藉凡鎹熷潖銆傝繖鏄彲鎭㈠鐨勶紝娴佷紶杈撳彲浠ョ収甯哥户缁紝缂撳啿鍖轰篃鍙互鐓у父閲嶇敤銆傞┍鍔ㄥ湪璋冪敤
	`VIDIOC_DQBUF` ioctl 鏃惰缃鏍囧織銆?    - .. _`V4L2-BUF-FLAG-IN-REQUEST`:

      - `V4L2_BUF_FLAG_IN_REQUEST`
      - 0x00000080
      - 璇ョ紦鍐插尯鏄竴涓皻鏈叆闃熺殑璇锋眰鐨勪竴閮ㄥ垎銆?    - .. _`V4L2-BUF-FLAG-KEYFRAME`:

      - `V4L2_BUF_FLAG_KEYFRAME`
      - 0x00000008
      - 椹卞姩鍦ㄨ皟鐢?`VIDIOC_DQBUF` ioctl 鏃惰缃垨娓呴櫎姝ゆ爣蹇椼€傚綋缂撳啿鍖哄寘鍚彲浣滀负鍏抽敭甯э紙鎴栧満锛夌嫭绔嬭В鍘嬬缉鐨勫帇缂╁浘鍍忔椂锛岃棰戦噰闆嗚澶囧彲鑳戒細璁剧疆瀹冿紝涔熺О涓?I 甯э紙I-frame锛夈€傚綋
	`type` 鎸囧悜杈撳嚭娴佹椂锛屽簲鐢ㄧ▼搴忓彲浠ヨ缃浣嶃€?    - .. _`V4L2-BUF-FLAG-PFRAME`:

      - `V4L2_BUF_FLAG_PFRAME`
      - 0x00000010
      - 涓?`V4L2_BUF_FLAG_KEYFRAME` 绫讳技锛屾鏍囧織鏍囪浠呭寘鍚笌鍓嶄竴鍏抽敭甯у樊寮傜殑棰勬祴甯ф垨鍦恒€傚綋
	`type` 鎸囧悜杈撳嚭娴佹椂锛屽簲鐢ㄧ▼搴忓彲浠ヨ缃浣嶃€?    - .. _`V4L2-BUF-FLAG-BFRAME`:

      - `V4L2_BUF_FLAG_BFRAME`
      - 0x00000020
      - 涓?`V4L2_BUF_FLAG_KEYFRAME` 绫讳技锛屾鏍囧織鏍囪鍙屽悜棰勬祴甯ф垨鍦猴紝鍏跺唴瀹逛粎鐢卞綋鍓嶅抚涓庡墠涓€鍏抽敭甯у拰鍚庝竴鍏抽敭甯т箣闂寸殑宸紓鏉ユ寚瀹氥€傚綋
	`type` 鎸囧悜杈撳嚭娴佹椂锛屽簲鐢ㄧ▼搴忓彲浠ヨ缃浣嶃€?    - .. _`V4L2-BUF-FLAG-TIMECODE`:

      - `V4L2_BUF_FLAG_TIMECODE`
      - 0x00000100
      - `timecode` 瀛楁鏈夋晥銆傞┍鍔ㄥ湪璋冪敤 `VIDIOC_DQBUF`
	ioctl 鏃惰缃垨娓呴櫎姝ゆ爣蹇椼€傚綋 `type` 鎸囧悜杈撳嚭娴佹椂锛屽簲鐢ㄧ▼搴忓彲浠ヨ缃浣嶄互鍙婄浉搴旂殑
	`timecode` 缁撴瀯浣撱€?    - .. _`V4L2-BUF-FLAG-PREPARED`:

      - `V4L2_BUF_FLAG_PREPARED`
      - 0x00000400
      - 缂撳啿鍖哄凡涓?I/O 鍋氬ソ鍑嗗锛屽彲鐢卞簲鐢ㄧ▼搴忓叆闃熴€傞┍鍔ㄥ湪璋冪敤
	VIDIOC_QUERYBUF <VIDIOC_QUERYBUF>銆?	VIDIOC_PREPARE_BUF <VIDIOC_QBUF>銆?	VIDIOC_QBUF <VIDIOC_QBUF> 鎴?	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 鏃惰缃垨娓呴櫎姝ゆ爣蹇椼€?    - .. _`V4L2-BUF-FLAG-NO-CACHE-INVALIDATE`:

      - `V4L2_BUF_FLAG_NO_CACHE_INVALIDATE`
      - 0x00000800
      - 涓嶅繀浣胯缂撳啿鍖虹殑缂撳瓨澶辨晥銆傞€氬父锛屽鏋滅紦鍐插尯涓崟鑾风殑鏁版嵁涓嶄細琚?CPU 瑙︾锛岃€屾槸寰堝彲鑳借浼犻€掔粰鏀寔 DMA 鐨勭‖浠跺崟鍏冨仛杩涗竴姝ュ鐞嗘垨杈撳嚭锛屽簲鐢ㄧ▼搴忓簲浣跨敤姝ゆ爣蹇椼€傞櫎闈為槦鍒楃敤浜庡唴瀛樻槧灏勶紙memory mapping <mmap>锛夋祦寮?I/O 骞朵笖鎶ュ憡 :ref:`V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS
	<V4L2-BUF-CAP-SUPPORTS-MMAP-CACHE-HINTS>` 鑳藉姏锛屽惁鍒欐鏍囧織琚拷鐣ャ€?    - .. _`V4L2-BUF-FLAG-NO-CACHE-CLEAN`:

      - `V4L2_BUF_FLAG_NO_CACHE_CLEAN`
      - 0x00001000
      - 涓嶅繀娓呯悊璇ョ紦鍐插尯鐨勭紦瀛樸€傞€氬父锛屽鏋滆缂撳啿鍖轰腑鐨勬暟鎹笉鏄敱 CPU 鑰屾槸鐢辨煇涓敮鎸?DMA 鐨勫崟鍏冨垱寤虹殑锛堣繖绉嶆儏鍐典笅骞舵湭浣跨敤缂撳瓨锛夛紝搴旂敤绋嬪簭搴斿杈撳嚭缂撳啿鍖轰娇鐢ㄦ鏍囧織銆傞櫎闈為槦鍒楃敤浜庡唴瀛樻槧灏勶紙memory mapping <mmap>锛夋祦寮?I/O 骞朵笖鎶ュ憡 :ref:`V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS
	<V4L2-BUF-CAP-SUPPORTS-MMAP-CACHE-HINTS>` 鑳藉姏锛屽惁鍒欐鏍囧織琚拷鐣ャ€?    - .. _`V4L2-BUF-FLAG-M2M-HOLD-CAPTURE-BUF`:

      - `V4L2_BUF_FLAG_M2M_HOLD_CAPTURE_BUF`
      - 0x00000200
      - 浠呭綋缁撴瀯浣?`v4l2_requestbuffers` 鐨?`V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF` 鏍囧織琚缃椂鎵嶆湁鏁堛€傚畠閫氬父涓庢棤鐘舵€佽В鐮佸櫒涓€璧蜂娇鐢紝鍏朵腑澶氫釜杈撳嚭缂撳啿鍖哄悇鑷В鐮佷负瑙ｇ爜鍚庡抚鐨勪竴涓垏鐗囥€傚簲鐢ㄧ▼搴忓湪鍏ラ槦杈撳嚭缂撳啿鍖烘椂鍙互璁剧疆姝ゆ爣蹇楋紝浠ラ槻姝㈤┍鍔ㄥ湪杈撳嚭缂撳啿鍖鸿В鐮佸畬鎴愬悗灏嗛噰闆嗙紦鍐插尯鍑洪槦锛堝嵆鈥滀繚鎸佲€濋噰闆嗙紦鍐插尯锛夈€傚鏋滆杈撳嚭缂撳啿鍖虹殑鏃堕棿鎴充笌鍓嶄竴涓緭鍑虹紦鍐插尯鐨勬椂闂存埑涓嶅悓锛屽垯琛ㄦ槑涓€涓柊甯у紑濮嬶紝涔嬪墠淇濇寔鐨勯噰闆嗙紦鍐插尯琚嚭闃熴€?    - .. _`V4L2-BUF-FLAG-LAST`:

      - `V4L2_BUF_FLAG_LAST`
      - 0x00100000
      - 纭欢浜х敓鐨勬渶鍚庝竴涓紦鍐插尯銆傚綋璋冪敤 VIDIOC_QUERYBUF 鎴?	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 鏃讹紝mem2mem 缂栬В鐮佸櫒椹卞姩浼氬湪閲囬泦闃熷垪鐨勬渶鍚庝竴涓紦鍐插尯涓婅缃鏍囧織銆傚彈纭欢闄愬埗锛屾渶鍚庝竴涓紦鍐插尯鍙兘涓虹┖銆傛鏃堕┍鍔ㄤ細灏?	`bytesused` 瀛楁璁句负 0锛屼笌鏍煎紡鏃犲叧銆備箣鍚庝换浣曞
	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 鐨勮皟鐢ㄩ兘涓嶅啀闃诲锛岃€屾槸杩斿洖 `EPIPE` 閿欒鐮併€?    - .. _`V4L2-BUF-FLAG-REQUEST-FD`:

      - `V4L2_BUF_FLAG_REQUEST_FD`
      - 0x00800000
      - `request_fd` 瀛楁鍖呭惈涓€涓湁鏁堢殑鏂囦欢鎻忚堪绗︺€?    - .. _`V4L2-BUF-FLAG-TIMESTAMP-MASK`:

      - `V4L2_BUF_FLAG_TIMESTAMP_MASK`
      - 0x0000e000
      - 涓嬫柟鏃堕棿鎴崇被鍨嬬殑鎺╃爜銆傝娴嬭瘯鏃堕棿鎴崇被鍨嬶紝鍙€氳繃瀵圭紦鍐插尯鏍囧織浣嶅拰鏃堕棿鎴虫帺鐮佹墽琛岄€昏緫涓庢搷浣滐紝灏嗕笉灞炰簬鏃堕棿鎴崇被鍨嬬殑浣嶅睆钄芥帀銆?    - .. _`V4L2-BUF-FLAG-TIMESTAMP-UNKNOWN`:

      - `V4L2_BUF_FLAG_TIMESTAMP_UNKNOWN`
      - 0x00000000
      - 鏈煡鐨勬椂闂存埑绫诲瀷銆侺inux 3.9 涔嬪墠鐨勯┍鍔ㄤ娇鐢ㄦ绫诲瀷锛屽畠鍙兘鏄崟璋冩椂閽燂紙瑙佷笅鏂囷級鎴栧疄鏃舵椂閽燂紙澧欎笂鏃堕挓锛夈€傚祵鍏ュ紡绯荤粺涓€惧悜浜庝娇鐢ㄥ崟璋冩椂閽燂紝鑰屽ぇ澶氭暟椹卞姩浣跨敤瀹炴椂鏃堕挓銆傝繖涓ょ鏃堕棿鎴抽兘鍙€氳繃
	`clock_gettime` 鍒嗗埆浣跨敤鏃堕挓 ID `CLOCK_MONOTONIC`
	鍜?`CLOCK_REALTIME` 鍦ㄧ敤鎴风┖闂磋幏寰椼€?    - .. _`V4L2-BUF-FLAG-TIMESTAMP-MONOTONIC`:

      - `V4L2_BUF_FLAG_TIMESTAMP_MONOTONIC`
      - 0x00002000
      - 缂撳啿鍖烘椂闂存埑鍙栬嚜 `CLOCK_MONOTONIC` 鏃堕挓銆傝鍦?V4L2 涔嬪璁块棶鍚屼竴鏃堕挓锛岃浣跨敤
	`clock_gettime`銆?    - .. _`V4L2-BUF-FLAG-TIMESTAMP-COPY`:

      - `V4L2_BUF_FLAG_TIMESTAMP_COPY`
      - 0x00004000
      - CAPTURE 缂撳啿鍖虹殑鏃堕棿鎴冲彇鑷搴旂殑 OUTPUT 缂撳啿鍖恒€傛鏍囧織浠呴€傜敤浜?mem2mem 璁惧銆?    - .. _`V4L2-BUF-FLAG-TSTAMP-SRC-MASK`:

      - `V4L2_BUF_FLAG_TSTAMP_SRC_MASK`
      - 0x00070000
      - 涓嬫柟鏃堕棿鎴虫簮鐨勬帺鐮併€傛椂闂存埑婧愬畾涔夌浉瀵逛簬甯ц€岃█閲囬泦鏃堕棿鎴崇殑鏃堕棿鐐广€傚 `flags` 瀛楁鍜?	`V4L2_BUF_FLAG_TSTAMP_SRC_MASK` 鎵ц閫昏緫涓庢搷浣滃彲寰楀埌鏃堕棿鎴虫簮鐨勫€笺€傚綋
	`type` 鎸囧悜杈撳嚭娴佷笖璁剧疆浜?`V4L2_BUF_FLAG_TIMESTAMP_COPY` 鏃讹紝搴旂敤绋嬪簭蹇呴』璁剧疆鏃堕棿鎴虫簮銆?    - .. _`V4L2-BUF-FLAG-TSTAMP-SRC-EOF`:

      - `V4L2_BUF_FLAG_TSTAMP_SRC_EOF`
      - 0x00000000
      - 甯х粨鏉燂紙End Of Frame锛夈€傛椂闂存埑鍦ㄥ抚鐨勬渶鍚庝竴涓儚绱犺鎺ユ敹鎴栧抚鐨勬渶鍚庝竴涓儚绱犺鍙戦€佹椂閲囬泦銆傚疄闄呬笂锛岃蒋浠剁敓鎴愮殑鏃堕棿鎴抽€氬父浼氬湪鏈€鍚庝竴涓儚绱犺鎺ユ敹鎴栧彂閫佸悗鐨勭煭鏆傚仠椤垮悗浠庢椂閽熻鍙栵紝鍏蜂綋鍙栧喅浜庣郴缁熷強鍏朵腑鐨勫叾浠栨椿鍔ㄣ€?    - .. _`V4L2-BUF-FLAG-TSTAMP-SRC-SOE`:

      - `V4L2_BUF_FLAG_TSTAMP_SRC_SOE`
      - 0x00010000
      - 鏇濆厜寮€濮嬶紙Start Of Exposure锛夈€傛椂闂存埑鍦ㄥ抚鐨勬洕鍏夊紑濮嬫椂閲囬泦銆傝繖浠呭
	`V4L2_BUF_TYPE_VIDEO_CAPTURE` 缂撳啿鍖虹被鍨嬫湁鏁堛€?


    \normalsize

## enum v4l2_memory



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_MEMORY_MMAP`
      - 1
      - 缂撳啿鍖虹敤浜庡唴瀛樻槧灏勶紙memory mapping <mmap>锛塈/O銆?    - - `V4L2_MEMORY_USERPTR`
      - 2
      - 缂撳啿鍖虹敤浜庣敤鎴锋寚閽堬紙user pointer <userp>锛塈/O銆?    - - `V4L2_MEMORY_OVERLAY`
      - 3
      - [to do]
    - - `V4L2_MEMORY_DMABUF`
      - 4
      - 缂撳啿鍖虹敤浜?DMA 鍏变韩缂撳啿鍖猴紙DMA shared buffer <dmabuf>锛塈/O銆?

    \normalsize

## 鏃堕棿鐮?

`v4l2_buffer_timecode` 缁撴瀯浣撹璁＄敤浜庝繚瀛?smpte12m 鎴栫被浼肩殑鏃堕棿鐮併€?锛堢粨鏋勪綋 `timeval` 鏃堕棿鎴冲瓨鍌ㄥ湪缁撴瀯浣?`v4l2_buffer` 鐨?`timestamp` 瀛楁涓€傦級


### struct v4l2_timecode



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 鏃堕棿鐮佹墍鍩轰簬鐨勫抚鐜囷紝鍙傝 timecode-type銆?    - - __u32
      - `flags`
      - 鏃堕棿鐮佹爣蹇楋紝鍙傝 timecode-flags銆?    - - __u8
      - `frames`
      - 甯ц鏁帮紝0 ... 23/24/29/49/59锛屽彇鍐充簬鏃堕棿鐮佺殑绫诲瀷銆?    - - __u8
      - `seconds`
      - 绉掕鏁帮紝0 ... 59銆傝繖鏄簩杩涘埗鏁帮紝涓嶆槸 BCD 鐮併€?    - - __u8
      - `minutes`
      - 鍒嗚鏁帮紝0 ... 59銆傝繖鏄簩杩涘埗鏁帮紝涓嶆槸 BCD 鐮併€?    - - __u8
      - `hours`
      - 灏忔椂璁℃暟锛? ... 29銆傝繖鏄簩杩涘埗鏁帮紝涓嶆槸 BCD 鐮併€?    - - __u8
      - `userbits`\ [^4^]
      - 鏃堕棿鐮佷腑鐨勨€滅敤鎴风粍锛坲ser group锛夆€濅綅銆?


### 鏃堕棿鐮佺被鍨?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TC_TYPE_24FPS`
      - 1
      - 姣忕 24 甯э紝鍗?film锛堢數褰憋級銆?    - - `V4L2_TC_TYPE_25FPS`
      - 2
      - 姣忕 25 甯э紝鍗?PAL 鎴?SECAM 瑙嗛銆?    - - `V4L2_TC_TYPE_30FPS`
      - 3
      - 姣忕 30 甯э紝鍗?NTSC 瑙嗛銆?    - - `V4L2_TC_TYPE_50FPS`
      - 4
      -
    - - `V4L2_TC_TYPE_60FPS`
      - 5
      -



### 鏃堕棿鐮佹爣蹇?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TC_FLAG_DROPFRAME`
      - 0x0001
      - 鎸囩ず鐢ㄤ簬 29.97 fps 绱犳潗璁″抚鐨勨€滀涪甯э紙drop frame锛夆€濊涔夈€傝缃悗锛岄櫎绗?0銆?0銆?0銆?0銆?0銆?0 鍒嗛挓澶栵紝姣忓垎閽熷紑濮嬫椂甯у彿 0 鍜?1 琚粠璁℃暟涓渷鐣ャ€?    - - `V4L2_TC_FLAG_COLORFRAME`
      - 0x0002
      - 鈥滃僵鑹插抚锛坈olor frame锛夆€濇爣蹇椼€?    - - `V4L2_TC_USERBITS_field`
      - 0x000C
      - 鈥滀簩杩涘埗缁勬爣蹇楋紙binary group flags锛夆€濈殑瀛楁鎺╃爜銆?    - - `V4L2_TC_USERBITS_USERDEFINED`
      - 0x0000
      - 鏈寚瀹氭牸寮忋€?    - - `V4L2_TC_USERBITS_8BITCHARS`
      - 0x0008
      - 8 浣?ISO 瀛楃銆?