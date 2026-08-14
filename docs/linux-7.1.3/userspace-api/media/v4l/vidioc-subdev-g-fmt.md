


######## ioctl VIDIOC_SUBDEV_G_FMT, VIDIOC_SUBDEV_S_FMT


## 鍚嶇О


VIDIOC_SUBDEV_G_FMT - VIDIOC_SUBDEV_S_FMT - 鑾峰彇鎴栬缃瓙璁惧 pad 涓婄殑鏁版嵁鏍煎紡

## 姒傝



`int ioctl(int fd, VIDIOC_SUBDEV_G_FMT, struct v4l2_subdev_format *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_FMT, struct v4l2_subdev_format *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_format` 鐨勬寚閽堛€?
## 鎻忚堪


杩欎簺 ioctl 鐢ㄤ簬鍦ㄥ浘鍍忔祦姘寸嚎涓壒瀹氱殑瀛愯澶?pad 涓婂崗鍟嗗抚鏍煎紡銆?
瑕佽幏鍙栧綋鍓嶆牸寮忥紝搴旂敤绋嬪簭灏?struct `v4l2_subdev_format` 鐨?`pad` 瀛楁璁句负濯掍綋
API 鎶ュ憡鐨勬湡鏈?pad 鍙凤紝骞跺皢 `which` 瀛楁璁句负 `V4L2_SUBDEV_FORMAT_ACTIVE`銆傚綋
瀹冧滑浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_SUBDEV_G_FMT` ioctl 鏃讹紝椹卞姩浼氬～鍏?`format`
瀛楁鐨勬垚鍛樸€?
瑕佹洿鏀瑰綋鍓嶆牸寮忥紝搴旂敤绋嬪簭璁剧疆 `pad` 鍜?`which` 瀛楁浠ュ強 `format` 瀛楁鐨勬墍鏈夋垚鍛樸€?褰撳畠浠互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 `VIDIOC_SUBDEV_S_FMT` ioctl 鏃讹紝椹卞姩浼氭牎楠岃姹傜殑
鏍煎紡锛屾牴鎹‖浠惰兘鍔涘鍏惰繘琛岃皟鏁达紝骞堕厤缃澶囥€傝繑鍥炴椂 struct `v4l2_subdev_format`
鍖呭惈褰撳墠鏍煎紡锛屾濡?`VIDIOC_SUBDEV_G_FMT` 璋冪敤鎵€杩斿洖鐨勯偅鏍枫€?
搴旂敤绋嬪簭鍙互閫氳繃灏?`which` 璁句负 `V4L2_SUBDEV_FORMAT_TRY` 鏉ユ煡璇㈣澶囪兘鍔涖€傚綋璁剧疆
璇ュ€兼椂锛屸€渢ry鈥濇牸寮忎笉浼氳椹卞姩搴旂敤鍒拌澶囦笂锛岃€屾槸鍍忔椿鍔ㄦ牸寮忎竴鏍疯淇敼锛屽苟瀛樺偍鍦?瀛愯澶囨枃浠跺彞鏌勪腑銆傚洜姝わ紝涓や釜鏌ヨ鍚屼竴瀛愯澶囩殑搴旂敤绋嬪簭涓嶄細鐩镐簰褰卞搷銆?
渚嬪锛岃鍦ㄥ瓙璁惧鐨勮緭鍑?pad 涓婂皾璇曚竴绉嶆牸寮忥紝搴旂敤绋嬪簭浼氶鍏堢敤 `VIDIOC_SUBDEV_S_FMT`
ioctl 鍦ㄥ瓙璁惧杈撳叆澶勮缃?try 鏍煎紡銆傜劧鍚庡畠浠涔堢敤 `VIDIOC_SUBDEV_G_FMT` ioctl
鑾峰彇杈撳嚭 pad 澶勭殑榛樿鏍煎紡锛岃涔堢敤 `VIDIOC_SUBDEV_S_FMT` ioctl 璁剧疆鏈熸湜鐨勮緭鍑?pad 鏍煎紡骞舵鏌ヨ繑鍥炲€笺€?
Try 鏍煎紡涓嶄緷璧栦簬娲诲姩鏍煎紡锛屼絾鍙兘渚濊禆浜庡綋鍓嶉摼璺厤缃垨瀛愯澶囨帶鍒跺€笺€備緥濡傦紝涓€涓?浣庨€氬櫔澹版护娉㈠櫒鍙兘浼氬湪甯ц竟鐣屽瑁佸壀鍍忕礌锛屼粠鑰屼慨鏀瑰叾杈撳嚭甯уぇ灏忋€?
濡傛灉瀛愯澶囪妭鐐瑰凡浠ュ彧璇绘ā寮忔敞鍐岋紝鍒欏 `VIDIOC_SUBDEV_S_FMT` 鐨勮皟鐢ㄤ粎鍦?`which`
瀛楁璁句负 `V4L2_SUBDEV_FORMAT_TRY` 鏃舵墠鏈夋晥锛屽惁鍒欒繑鍥為敊璇苟灏?errno 鍙橀噺璁句负
`-EPERM`銆?
椹卞姩缁濅笉鑳戒粎浠呭洜涓鸿姹傜殑鏍煎紡涓庤澶囪兘鍔涗笉鍖归厤灏辫繑鍥為敊璇€傚畠浠繀椤绘敼涓轰慨鏀规牸寮?浠ュ尮閰嶇‖浠舵墍鑳芥彁渚涚殑銆備慨鏀瑰悗鐨勬牸寮忓簲灏藉彲鑳芥帴杩戝師濮嬭姹傘€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - 鐢卞獟浣撴帶鍒跺櫒 API 鎶ュ憡鐨?pad 鍙枫€?    - - __u32
      - `which`
      - 瑕佷慨鏀圭殑鏍煎紡锛屾潵鑷灇涓?	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - struct `v4l2_mbus_framefmt`
      - `format`
      - 鍥惧儚鏍煎紡瀹氫箟锛岃瑙?`v4l2_mbus_framefmt`銆?    - - __u32
      - `stream`
      - 娴佹爣璇嗙銆?    - - __u32
      - `reserved`\ [^7^]
      - 涓哄皢鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忓拰椹卞姩蹇呴』灏嗚鏁扮粍缃浂銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - V4L2_SUBDEV_FORMAT_TRY
      - 0
      - Try 鏍煎紡锛岀敤浜庢煡璇㈣澶囪兘鍔涖€?    - - V4L2_SUBDEV_FORMAT_ACTIVE
      - 1
      - 娲诲姩鏍煎紡锛屽簲鐢ㄥ埌纭欢銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
EBUSY
    鏍煎紡鏃犳硶鏇存敼锛屽洜涓鸿 pad 褰撳墠姝ｅ繖銆備緥濡傦紝杩欏彲鑳芥槸鐢变簬璇?pad 涓婃湁娲昏穬鐨?    瑙嗛娴併€傚繀椤诲厛鎵ц鍏跺畠鎿嶄綔瑙ｅ喅闂锛屾墠鑳介噸璇曡 ioctl銆備粎鐢?    `VIDIOC_SUBDEV_S_FMT` 杩斿洖銆?
EINVAL
    struct `v4l2_subdev_format` 鐨?`pad` 寮曠敤浜嗕竴涓笉瀛樺湪鐨?pad锛屾垨鑰?`which`
    瀛楁鐨勫€间笉鍙楁敮鎸併€?
EPERM
    `VIDIOC_SUBDEV_S_FMT` ioctl 鍦ㄤ竴涓彧璇诲瓙璁惧涓婅璋冪敤锛屼笖 `which` 瀛楁琚
    涓?`V4L2_SUBDEV_FORMAT_ACTIVE`銆?
============

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?