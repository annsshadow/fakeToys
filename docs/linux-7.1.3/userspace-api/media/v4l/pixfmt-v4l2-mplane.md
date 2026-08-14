######## 澶氬钩闈㈡牸寮忕粨鏋勪綋


struct `v4l2_plane_pix_format` 缁撴瀯浣撳畾涔変簡澶氬钩闈㈡牸寮忎腑姣忎釜骞抽潰鐨?澶у皬鍜屽竷灞€銆俿truct `v4l2_pix_format_mplane` 缁撴瀯浣撳寘鍚?鎵€鏈夊钩闈㈠叡鏈夌殑淇℃伅锛堝鍥惧儚瀹藉害鍜岄珮搴︼級锛屼互鍙婁竴涓?struct
`v4l2_plane_pix_format` 缁撴瀯浣撴暟缁勶紝鎻忚堪璇ユ牸寮忕殑鎵€鏈夊钩闈€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `sizeimage`
      - 璇ュ钩闈腑鍥惧儚鏁版嵁鎵€闇€鐨勬渶澶у瓧鑺傛暟锛岀敱椹卞姩璁剧疆銆傚綋鍥惧儚鐢卞彉闀?	鍘嬬缉鏁版嵁缁勬垚鏃讹紝杩欐槸缂栬В鐮佸櫒鏀寔鏈€鍧忔儏鍐靛帇缂╁満鏅墍闇€鐨勫瓧鑺傛暟銆?
	椹卞姩灏嗕负姝ょ被鏈帇缂╁浘鍍忚缃鍊笺€?
	瀹㈡埛绔厑璁镐负鍦?VIDIOC_ENUM_FMT 澶勮
	`V4L2_FMT_FLAG_COMPRESSED` 鏍囪鐨勫彉闀垮帇缂╂暟鎹缃?	sizeimage 瀛楁锛屼絾椹卞姩鍙互蹇界暐瀹冨苟鑷璁剧疆璇ュ€硷紝鎴栬€呮牴鎹?	瀵归綈瑕佹眰鎴栨渶灏?鏈€澶у昂瀵歌姹備慨鏀规墍鎻愪緵鐨勫€笺€傚鏋滃鎴风甯屾湜
	灏嗘浜ょ敱椹卞姩澶勭悊锛屽垯搴旀妸 sizeimage 璁句负 0銆?    - - __u32
      - `bytesperline`
      - 涓ゆ潯鐩搁偦琛屼腑鏈€宸︿晶鍍忕礌涔嬮棿鐨勫瓧鑺傝窛绂汇€傝 struct
	`v4l2_pix_format`銆?    - - __u16
      - `reserved[^6^]`
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭搴斿皢鍏剁疆闆躲€?


    \small



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `width`
      - 鍥惧儚瀹藉害锛堝儚绱狅級銆傝 struct
	`v4l2_pix_format`銆?    - - __u32
      - `height`
      - 鍥惧儚楂樺害锛堝儚绱狅級銆傝 struct
	`v4l2_pix_format`銆?    - - __u32
      - `pixelformat`
      - 鍍忕礌鏍煎紡銆傚彲浠ヤ娇鐢ㄥ崟骞抽潰鍜屽骞抽潰鍥涘瓧绗︾爜銆?    - - __u32
      - `field`
      - 鍦洪『搴忥紝鏉ヨ嚜鏋氫妇 `v4l2_field`銆?        瑙?struct `v4l2_pix_format`銆?    - - __u32
      - `colorspace`
      - 鑹插僵绌洪棿缂栫爜锛屾潵鑷灇涓?`v4l2_colorspace`銆?        瑙?struct `v4l2_pix_format`銆?    - - struct `v4l2_plane_pix_format`
      - `plane_fmt[VIDEO_MAX_PLANES]`
      - 鎻忚堪姝ゅ儚绱犳牸寮忔墍鍖呭惈鐨勬瘡涓钩闈㈢殑缁撴瀯浣撴暟缁勩€傝鏁扮粍涓湁鏁?	鏉＄洰鐨勬暟閲忓繀椤绘斁鍏?`num_planes` 瀛楁銆?    - - __u8
      - `num_planes`
      - 璇ユ牸寮忕殑骞抽潰鏁帮紙鍗崇嫭绔嬬殑鍐呭瓨缂撳啿鍖猴級锛屼互鍙?`plane_fmt`
	鏁扮粍涓湁鏁堟潯鐩殑鏁伴噺銆?    - - __u8
      - `flags`
      - 鐢卞簲鐢ㄧ▼搴忔垨椹卞姩璁剧疆鐨勬爣蹇楋紝瑙?format-flags銆?    - - union {
      - (鍖垮悕)
    - - __u8
      - `ycbcr_enc`
      - Y'CbCr 缂栫爜锛屾潵鑷灇涓?`v4l2_ycbcr_encoding`銆?	瑙?struct `v4l2_pix_format`銆?    - - __u8
      - `hsv_enc`
      - HSV 缂栫爜锛屾潵鑷灇涓?`v4l2_hsv_encoding`銆?	瑙?struct `v4l2_pix_format`銆?    - - }
      -
    - - __u8
      - `quantization`
      - 閲忓寲鑼冨洿锛屾潵鑷灇涓?`v4l2_quantization`銆?	瑙?struct `v4l2_pix_format`銆?    - - __u8
      - `xfer_func`
      - 浼犺緭鍑芥暟锛屾潵鑷灇涓?`v4l2_xfer_func`銆?	瑙?struct `v4l2_pix_format`銆?    - - __u8
      - `reserved[^7^]`
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭搴斿皢鍏剁疆闆躲€?

    \normalsize
