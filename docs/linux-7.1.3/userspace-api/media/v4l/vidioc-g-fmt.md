


######## ioctl VIDIOC_G_FMT, VIDIOC_S_FMT, VIDIOC_TRY_FMT


## Name


VIDIOC_G_FMT - VIDIOC_S_FMT - VIDIOC_TRY_FMT - 鑾峰彇鎴栬缃暟鎹牸寮忥紝灏濊瘯涓€绉嶆牸寮?
## Synopsis



`int ioctl(int fd, VIDIOC_G_FMT, struct v4l2_format *argp)`


`int ioctl(int fd, VIDIOC_S_FMT, struct v4l2_format *argp)`


`int ioctl(int fd, VIDIOC_TRY_FMT, struct v4l2_format *argp)`

## Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_format` 鐨勬寚閽堛€?
## Description


杩欎簺 ioctl 鐢ㄤ簬鍗忓晢椹卞姩涓庡簲鐢ㄧ▼搴忎箣闂翠氦鎹㈢殑鏁版嵁锛堥€氬父鏄浘鍍忥級鏍煎紡銆?
瑕佹煡璇㈠綋鍓嶅弬鏁帮紝搴旂敤绋嬪簭灏?struct `v4l2_format` 鐨?`type` 瀛楁璁剧疆涓虹浉搴旂殑缂撳啿鍖猴紙娴侊級绫诲瀷銆備緥濡傝棰戦噰闆嗚澶囦娇鐢?`V4L2_BUF_TYPE_VIDEO_CAPTURE` 鎴?`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`銆傚綋搴旂敤绋嬪簭璋冪敤甯︽湁鎸囧悜璇ョ粨鏋勭殑鎸囬拡鐨?VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl 鏃讹紝椹卞姩浼氬～鍏?`fmt` 鑱斿悎鐨勭浉搴旀垚鍛樸€傚浜庤棰戦噰闆嗚澶囷紝璇ユ垚鍛樻槸 struct `v4l2_pix_format` 鐨?`pix` 鎴?struct `v4l2_pix_format_mplane` 鐨?`pix_mp` 鎴愬憳銆傚綋鎵€璇锋眰鐨勭紦鍐插尯绫诲瀷涓嶅彈鏀寔鏃讹紝椹卞姩杩斿洖 `EINVAL` 閿欒鐮併€?
瑕佹洿鏀瑰綋鍓嶆牸寮忓弬鏁帮紝搴旂敤绋嬪簭鍒濆鍖?`type` 瀛楁浠ュ強鐩稿簲 `fmt` 鑱斿悎鎴愬憳鐨勬墍鏈夊瓧娈点€傜粏鑺傝鍙傞槄 devices 涓悇绉嶈澶囩被鍨嬬殑鏂囨。銆傚ソ鐨勫仛娉曟槸浠ュ厛鏌ヨ褰撳墠鍙傛暟锛岀劧鍚庡彧淇敼閭ｄ簺涓嶉€傚悎搴旂敤绋嬪簭鐨勫弬鏁颁负鍑嗐€傚綋搴旂敤绋嬪簭璋冪敤甯︽湁鎸囧悜 struct `v4l2_format` 缁撴瀯鐨勬寚閽堢殑 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鏃讹紝椹卞姩浼氭牴鎹‖浠惰兘鍔涙鏌ュ苟璋冩暣鍙傛暟銆傞櫎闈?`type` 瀛楁鏃犳晥锛屽惁鍒欓┍鍔ㄤ笉搴旇繑鍥為敊璇爜锛岃繖鏄竴绉嶆帰娴嬭澶囪兘鍔涘苟鎺ヨ繎搴旂敤绋嬪簭鍜岄┍鍔ㄩ兘鍙帴鍙楀弬鏁扮殑鏈哄埗銆傛垚鍔熸椂锛岄┍鍔ㄥ彲浠ョ紪绋嬬‖浠躲€佸垎閰嶈祫婧愶紝骞堕€氬父涓烘暟鎹氦鎹㈠仛鍑嗗銆傛渶鍚庯紝VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鍍?VIDIOC_G_FMT <VIDIOC_G_FMT> 閭ｆ牱杩斿洖褰撳墠鏍煎紡鍙傛暟銆傞潪甯哥畝鍗曠殑銆佷笉鐏垫椿鐨勮澶囩敋鑷冲彲鑳藉拷鐣ユ墍鏈夎緭鍏ワ紝骞舵€绘槸杩斿洖榛樿鍙傛暟銆傜劧鑰岋紝鎵€鏈変笌搴旂敤绋嬪簭浜ゆ崲鏁版嵁鐨?V4L2 璁惧閮藉繀椤诲疄鐜?VIDIOC_G_FMT <VIDIOC_G_FMT> 鍜?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl銆傚綋鎵€璇锋眰鐨勭紦鍐插尯绫诲瀷涓嶅彈鏀寔鏃讹紝椹卞姩鍦?VIDIOC_S_FMT <VIDIOC_G_FMT> 灏濊瘯鏃惰繑鍥?`EINVAL` 閿欒鐮併€傚綋 I/O 宸插湪杩涜涓紝鎴栧洜鍏朵粬鍘熷洜璧勬簮涓嶅彲鐢ㄦ椂锛岄┍鍔ㄨ繑鍥?`EBUSY` 閿欒鐮併€?
VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 绛夊悓浜?VIDIOC_S_FMT <VIDIOC_G_FMT>锛屽彧鏈変竴涓緥澶栵細瀹冧笉鏀瑰彉椹卞姩鐘舵€併€傚畠涔熷彲浠ュ湪浠讳綍鏃跺€欒皟鐢紝缁濅笉浼氳繑鍥?`EBUSY`銆傛彁渚涙鍑芥暟鏄负浜嗗湪涓嶇鐢?I/O 鎴栧彲鑳借€楁椂鐨勭‖浠跺噯澶囩殑鎯呭喌涓嬶紝鍗忓晢鍙傛暟銆佷簡瑙ｇ‖浠堕檺鍒躲€傚敖绠″己鐑堟帹鑽愶紝椹卞姩骞朵笉瑕佹眰瀹炵幇姝?ioctl銆?
VIDIOC_TRY_FMT <VIDIOC_G_FMT> 杩斿洖鐨勬牸寮忓繀椤讳笌 VIDIOC_S_FMT <VIDIOC_G_FMT> 瀵圭浉鍚岃緭鍏ユ垨杈撳嚭杩斿洖鐨勬牸寮忓畬鍏ㄧ浉鍚屻€?


    :header-rows:  0
    :stub-columns: 0

    - - __u32
      - `type`
      - 鏁版嵁娴佺殑绫诲瀷锛屽弬瑙?`v4l2_buf_type`銆?    - - union {
      - `fmt`
    - - struct `v4l2_pix_format`
      - `pix`
      - 鍥惧儚鏍煎紡鐨勫畾涔夛紝鍙傝 pixfmt锛岀敤浜庤棰戦噰闆嗗拰杈撳嚭璁惧銆?    - - struct `v4l2_pix_format_mplane`
      - `pix_mp`
      - 鍥惧儚鏍煎紡鐨勫畾涔夛紝鍙傝 pixfmt锛岀敤浜庢敮鎸?        澶氬钩闈㈢増鏈?API <planar-apis> 鐨勮棰戦噰闆嗗拰杈撳嚭璁惧銆?    - - struct `v4l2_window`
      - `win`
      - 鍙犲姞鍥惧儚鐨勫畾涔夛紝鍙傝 overlay锛岀敤浜庤棰戝彔鍔犺澶囥€?    - - struct `v4l2_vbi_format`
      - `vbi`
      - 鍘熷 VBI 閲囬泦鎴栬緭鍑哄弬鏁般€傝繖鍦?raw-vbi 涓湁鏇磋缁嗙殑璁ㄨ銆傜敤浜庡師濮?VBI 閲囬泦鍜岃緭鍑鸿澶囥€?    - - struct `v4l2_sliced_vbi_format`
      - `sliced`
      - 鍒囩墖 VBI 閲囬泦鎴栬緭鍑哄弬鏁般€傜粏鑺傚弬瑙?sliced銆傜敤浜庡垏鐗?VBI 閲囬泦鍜岃緭鍑鸿澶囥€?    - - struct `v4l2_sdr_format`
      - `sdr`
      - 鏁版嵁鏍煎紡鐨勫畾涔夛紝鍙傝 pixfmt锛岀敤浜?SDR 閲囬泦鍜岃緭鍑鸿澶囥€?    - - struct `v4l2_meta_format`
      - `meta`
      - 鍏冩暟鎹牸寮忕殑瀹氫箟锛屽弬瑙?meta-formats锛岀敤浜庡厓鏁版嵁閲囬泦璁惧銆?    - - __u8
      - `raw_data`\ [^200^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欑殑鍗犱綅绗︺€?    - - }
      -

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟涓?`errno` 鍙橀噺浼氳鐩稿簲鍦拌缃€傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
EINVAL
    struct `v4l2_format` 鐨?`type` 瀛楁鏃犳晥锛屾垨鎵€璇锋眰鐨勭紦鍐插尯绫诲瀷涓嶅彈鏀寔銆?
EBUSY
    璁惧姝ｅ繖锛屾棤娉曟洿鏀规牸寮忋€傝繖鍙兘鏄洜涓鸿澶囨鍦ㄦ祦寮忎紶杈擄紝鎴栬€呯紦鍐插尯宸插垎閰嶆垨宸插叆闃熷埌椹卞姩銆備粎涓?:ref:`VIDIOC_S_FMT <VIDIOC_G_FMT>` 鐩稿叧銆?