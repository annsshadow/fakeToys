

######## ioctl VIDIOC_G_EXT_CTRLS, VIDIOC_S_EXT_CTRLS, VIDIOC_TRY_EXT_CTRLS


## 鍚嶇О


VIDIOC_G_EXT_CTRLS - VIDIOC_S_EXT_CTRLS - VIDIOC_TRY_EXT_CTRLS - 鑾峰彇鎴栬缃涓帶浠剁殑鍊硷紝灏濊瘯鎺т欢鍊?
## 姒傝



`int ioctl(int fd, VIDIOC_G_EXT_CTRLS, struct v4l2_ext_controls *argp)`


`int ioctl(int fd, VIDIOC_S_EXT_CTRLS, struct v4l2_ext_controls *argp)`


`int ioctl(int fd, VIDIOC_TRY_EXT_CTRLS, struct v4l2_ext_controls *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_ext_controls` 鐨勬寚閽堛€?
## 鎻忚堪


杩欎簺 ioctl 鍏佽璋冪敤鑰呭師瀛愬湴鑾峰彇鎴栬缃涓帶浠躲€傛帶浠?ID 琚垎缁勫埌鎺т欢绫伙紙瑙?ctrl-class锛変腑锛屽苟涓旀帶浠舵暟缁勪腑鐨勬墍鏈夋帶浠跺繀椤诲睘浜庡悓涓€涓帶浠剁被銆?
搴旂敤绋嬪簭蹇呴』濮嬬粓濉啓 struct `v4l2_ext_controls` 鐨?`count`銆乣which`銆乣controls`
鍜?`reserved` 瀛楁锛屽苟鍒濆鍖栫敱 `controls` 瀛楁鎸囧悜鐨?struct `v4l2_ext_control`
鏁扮粍銆?
瑕佽幏鍙栦竴缁勬帶浠剁殑褰撳墠鍊硷紝搴旂敤绋嬪簭鍒濆鍖栨瘡涓?struct `v4l2_ext_control` 鐨?`id`銆?`size` 鍜?`reserved2` 瀛楁锛屽苟璋冪敤 VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl銆?瀛楃涓叉帶浠惰繕蹇呴』璁剧疆 `string` 瀛楁銆傚鍚堢被鍨嬶紙`V4L2_CTRL_FLAG_HAS_PAYLOAD`
琚缃級鐨勬帶浠跺繀椤昏缃?`ptr` 瀛楁銆?
濡傛灉 `size` 澶皬浠ヨ嚦浜庢棤娉曟帴鏀舵帶浠剁粨鏋滐紙浠呬笌瀛楃涓茬瓑鎸囬拡绫诲瀷鎺т欢鐩稿叧锛夛紝閭ｄ箞
椹卞姩浼氬皢 `size` 璁剧疆涓轰竴涓湁鏁堝€煎苟杩斿洖 `ENOSPC` 閿欒鐮併€備綘搴旇灏嗗唴瀛橀噸鏂板垎閰嶄负
杩欎釜鏂板ぇ灏忓苟閲嶈瘯銆傚浜庡瓧绗︿覆绫诲瀷锛屽鏋滃瓧绗︿覆鍦ㄦ鏈熼棿鍙橀暱浜嗭紝鍚屾牱鐨勯棶棰樺彲鑳藉啀娆?鍙戠敓銆傚缓璁厛璋冪敤 VIDIOC_QUERYCTRL 骞朵娇鐢?`maximum`\ +1 浣滀负鏂扮殑 `size` 鍊笺€傝繖鑳戒繚璇?鍐呭瓨瓒冲銆?
N 缁存暟缁勯€愯璁剧疆鍜岃幏鍙栥€備綘涓嶈兘璁剧疆閮ㄥ垎鏁扮粍锛屽繀椤昏缃垨鑾峰彇鎵€鏈夊厓绱犮€傛€诲ぇ灏忚绠?涓?`elems` * `elem_size`銆傝繖浜涘€煎彲浠ラ€氳繃璋冪敤 VIDIOC_QUERY_EXT_CTRL <VIDIOC_QUERYCTRL>
鑾峰緱銆?
瑕佹洿鏀逛竴缁勬帶浠剁殑鍊硷紝搴旂敤绋嬪簭鍒濆鍖栨瘡涓?struct `v4l2_ext_control` 鐨?`id`銆乣size`銆?`reserved2` 鍜?`value/value64/string/ptr` 瀛楁锛屽苟璋冪敤
VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl銆傚彧鏈夊綋**鎵€鏈?*鎺т欢鍊奸兘鏈夋晥鏃讹紝鎺т欢鎵嶄細琚缃€?
瑕佹鏌ヤ竴缁勬帶浠舵槸鍚﹀叿鏈夋纭殑鍊硷紝搴旂敤绋嬪簭鍒濆鍖栨瘡涓?struct `v4l2_ext_control` 鐨?`id`銆乣size`銆乣reserved2` 鍜?`value/value64/string/ptr` 瀛楁锛屽苟璋冪敤
VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl銆傞敊璇€兼槸鑷姩璋冩暣鍒版湁鏁堝€艰繕鏄繑鍥為敊璇紝鍙栧喅浜庨┍鍔ㄣ€?
褰?`id` 鎴?`which` 鏃犳晥鏃讹紝椹卞姩杩斿洖 `EINVAL` 閿欒鐮併€傚綋鍊艰秺鐣屾椂锛岄┍鍔ㄥ彲浠ラ€夋嫨鍙?鏈€鎺ヨ繎鐨勫悎娉曞€兼垨杩斿洖 `ERANGE` 閿欒鐮侊紝 whichever 鐪嬭捣鏉ユ洿鍚堥€傘€傚湪绗竴绉嶆儏鍐典笅锛屾柊鍊?琚缃湪 struct `v4l2_ext_control` 涓€傚鏋滄柊鐨勬帶浠跺€间笉鍚堥€傦紙渚嬪缁欏畾鐨勮彍鍗曠储寮?涓嶈鑿滃崟鎺т欢鏀寔锛夛紝閭ｄ箞杩欎篃浼氬鑷翠竴涓?`EINVAL` 閿欒鐮侀敊璇€?
濡傛灉 `request_fd` 琚缃负涓€涓皻鏈帓闃熺殑 request <media-request-api> 鏂囦欢鎻忚堪绗︼紝
骞朵笖 `which` 琚缃负 `V4L2_CTRL_WHICH_REQUEST_VAL`锛岄偅涔堣繖浜涙帶浠朵笉浼氬湪璋冪敤
VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鏃剁珛鍗冲簲鐢紝鑰屾槸琚┍鍔ㄥ簲鐢ㄤ簬涓庡悓涓€璇锋眰鍏宠仈鐨勭紦鍐插尯銆?濡傛灉璁惧涓嶆敮鎸佽姹傦紝閭ｄ箞灏嗚繑鍥?`EACCES`銆傚鏋滄敮鎸佽姹備絾缁欏嚭浜嗘棤鏁堢殑璇锋眰鏂囦欢鎻忚堪绗︼紝
閭ｄ箞灏嗚繑鍥?`EINVAL`銆?
璇曞浘涓哄凡缁忔帓闃熺殑璇锋眰璋冪敤 VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 灏嗗鑷翠竴涓?`EBUSY` 閿欒銆?
濡傛灉鍦ㄨ皟鐢?VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鏈熼棿鎸囧畾浜?`request_fd` 骞朵笖 `which` 琚缃负
`V4L2_CTRL_WHICH_REQUEST_VAL`锛岄偅涔堝畠灏嗚繑鍥炶姹傚畬鎴愭椂鎺т欢鐨勫€笺€傚鏋滆姹傚皻鏈畬鎴愶紝
閭ｄ箞杩欏皢瀵艰嚧涓€涓?`EACCES` 閿欒銆?
椹卞姩鍙細鍦ㄦ墍鏈夋帶浠跺€奸兘姝ｇ‘鏃惰缃?鑾峰彇杩欎簺鎺т欢銆傝繖闃叉浜嗗彧鏈夐儴鍒嗘帶浠惰璁剧疆/鑾峰彇
鐨勬儏鍐点€傚彧鏈夊簳灞傞敊璇紙渚嬪澶辫触鐨?i2c 鍛戒护锛変粛鍙兘瀵艰嚧杩欑鎯呭喌銆?



   \footnotesize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 鏍囪瘑鎺т欢锛岀敱搴旂敤绋嬪簭璁剧疆銆?    - - __u32
      - `size`
      - 姝ゆ帶浠惰礋杞界殑鎬诲瓧鑺傚ぇ灏忋€?    - - `2` `size` 瀛楁閫氬父涓?0锛屼絾瀵逛簬鎸囬拡鎺т欢锛屽簲灏嗗叾璁剧疆涓哄寘鍚礋杞芥垨
	灏嗘帴鏀惰礋杞界殑鍐呭瓨鐨勫ぇ灏忋€?	濡傛灉 VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鍙戠幇璇ュ€煎皬浜庡瓨鍌ㄨ礋杞界粨鏋滄墍闇€鐨勫€硷紝
	閭ｄ箞瀹冧細琚缃负瓒冲瀛樺偍璐熻浇缁撴灉鐨勫€硷紝骞惰繑鍥?`ENOSPC`銆?
```

	   For string controls, this ``size`` field should
	   not be confused with the length of the string. This field refers
	   to the size of the memory that contains the string. The actual
	   *length* of the string may well be much smaller.
    * - __u32
      - ``reserved2``\ [1]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗘暟缁勮涓洪浂銆?    * - union {
      - (anonymous)
    * - __s32
      - ``value``
      - 鏂板€兼垨褰撳墠鍊笺€傚鏋滄鎺т欢涓嶆槸 `V4L2_CTRL_TYPE_INTEGER64` 绫诲瀷涓旀湭璁剧疆
	`V4L2_CTRL_FLAG_HAS_PAYLOAD`锛屽垯鏈夋晥銆?    * - __s64
      - ``value64``
      - 鏂板€兼垨褰撳墠鍊笺€傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_INTEGER64` 绫诲瀷涓旀湭璁剧疆
	`V4L2_CTRL_FLAG_HAS_PAYLOAD`锛屽垯鏈夋晥銆?    * - char *
      - ``string``
      - 鎸囧悜瀛楃涓茬殑鎸囬拡銆傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_STRING` 绫诲瀷鍒欐湁鏁堛€?    * - __u8 *
      - ``p_u8``
      - 鎸囧悜鏃犵鍙?8 浣嶅€肩煩闃垫帶浠剁殑鎸囬拡銆傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_U8` 绫诲瀷鍒欐湁鏁堛€?    * - __u16 *
      - ``p_u16``
      - 鎸囧悜鏃犵鍙?16 浣嶅€肩煩闃垫帶浠剁殑鎸囬拡銆傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_U16` 绫诲瀷鍒欐湁鏁堛€?    * - __u32 *
      - ``p_u32``
      - 鎸囧悜鏃犵鍙?32 浣嶅€肩煩闃垫帶浠剁殑鎸囬拡銆傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_U32` 绫诲瀷鍒欐湁鏁堛€?    * - __s32 *
      - ``p_s32``
      - 鎸囧悜鏈夌鍙?32 浣嶅€肩煩闃垫帶浠剁殑鎸囬拡銆傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_INTEGER` 绫诲瀷涓?        璁剧疆浜?`V4L2_CTRL_FLAG_HAS_PAYLOAD` 鍒欐湁鏁堛€?    * - __s64 *
      - ``p_s64``
      - 鎸囧悜鏈夌鍙?64 浣嶅€肩煩闃垫帶浠剁殑鎸囬拡銆傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_INTEGER64` 绫诲瀷涓?        璁剧疆浜?`V4L2_CTRL_FLAG_HAS_PAYLOAD` 鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_area` *
      - ``p_area``
      - 鎸囧悜 struct :c:type:`v4l2_area` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_AREA` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_rect` *
      - ``p_rect``
      - 鎸囧悜 struct :c:type:`v4l2_rect` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?`V4L2_CTRL_TYPE_RECT` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_h264_sps` *
      - ``p_h264_sps``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_h264_sps` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_H264_SPS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_h264_pps` *
      - ``p_h264_pps``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_h264_pps` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_H264_PPS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_h264_scaling_matrix` *
      - ``p_h264_scaling_matrix``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_h264_scaling_matrix` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_H264_SCALING_MATRIX` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_h264_pred_weights` *
      - ``p_h264_pred_weights``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_h264_pred_weights` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_H264_PRED_WEIGHTS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_h264_slice_params` *
      - ``p_h264_slice_params``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_h264_slice_params` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_H264_SLICE_PARAMS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_h264_decode_params` *
      - ``p_h264_decode_params``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_h264_decode_params` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_H264_DECODE_PARAMS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_fwht_params` *
      - ``p_fwht_params``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_fwht_params` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_FWHT_PARAMS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_vp8_frame` *
      - ``p_vp8_frame``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_vp8_frame` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_VP8_FRAME` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_mpeg2_sequence` *
      - ``p_mpeg2_sequence``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_mpeg2_sequence` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_MPEG2_SEQUENCE` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_mpeg2_picture` *
      - ``p_mpeg2_picture``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_mpeg2_picture` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_MPEG2_PICTURE` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_mpeg2_quantisation` *
      - ``p_mpeg2_quantisation``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_mpeg2_quantisation` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_MPEG2_QUANTISATION` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_vp9_compressed_hdr` *
      - ``p_vp9_compressed_hdr_probs``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_vp9_compressed_hdr` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_VP9_COMPRESSED_HDR` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_vp9_frame` *
      - ``p_vp9_frame``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_vp9_frame` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_VP9_FRAME` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hdr10_cll_info` *
      - ``p_hdr10_cll``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hdr10_cll_info` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HDR10_CLL_INFO` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hdr10_mastering_display` *
      - ``p_hdr10_mastering``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hdr10_mastering_display` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HDR10_MASTERING_DISPLAY` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hevc_sps` *
      - ``p_hevc_sps``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hevc_sps` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HEVC_SPS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hevc_pps` *
      - ``p_hevc_pps``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hevc_pps` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HEVC_PPS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hevc_slice_params` *
      - ``p_hevc_slice_params``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hevc_slice_params` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HEVC_SLICE_PARAMS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hevc_scaling_matrix` *
      - ``p_hevc_scaling_matrix``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hevc_scaling_matrix` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HEVC_SCALING_MATRIX` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hevc_decode_params` *
      - ``p_hevc_decode_params``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hevc_decode_params` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HEVC_DECODE_PARAMS` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_av1_sequence` *
      - ``p_av1_sequence``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_av1_sequence` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_AV1_SEQUENCE` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_av1_tile_group_entry` *
      - ``p_av1_tile_group_entry``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_av1_tile_group_entry` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_AV1_TILE_GROUP_ENTRY` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_av1_frame` *
      - ``p_av1_frame``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_av1_frame` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_AV1_FRAME` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_av1_film_grain` *
      - ``p_av1_film_grain``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_av1_film_grain` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_AV1_FILM_GRAIN` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hdr10_cll_info` *
      - ``p_hdr10_cll_info``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hdr10_cll_info` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HDR10_CLL_INFO` 绫诲瀷鍒欐湁鏁堛€?    * - struct :c:type:`v4l2_ctrl_hdr10_mastering_display` *
      - ``p_hdr10_mastering_display``
      - 鎸囧悜 struct :c:type:`v4l2_ctrl_hdr10_mastering_display` 鐨勬寚閽堛€傚鏋滄鎺т欢鏄?        `V4L2_CTRL_TYPE_HDR10_MASTERING_DISPLAY` 绫诲瀷鍒欐湁鏁堛€?    * - void *
      - ``ptr``
      - 鎸囧悜澶嶅悎绫诲瀷鐨勬寚閽堬紝璇ュ鍚堢被鍨嬪彲浠ユ槸涓€涓?N 缁存暟缁勫拰/鎴栧鍚堢被鍨嬶紙鎺т欢鐨勭被鍨?>=
	`V4L2_CTRL_COMPOUND_TYPES`锛夈€傚鏋滀负姝ゆ帶浠惰缃簡 `V4L2_CTRL_FLAG_HAS_PAYLOAD`
	鍒欐湁鏁堛€?    * - }
      -

```

   \normalsize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - union {
      - (anonymous)
    - - __u32
      - `which`
      - 瑕佽幏鍙?璁剧疆/灏濊瘯鐨勬帶浠剁殑鍊笺€?    - - `2` `V4L2_CTRL_WHICH_CUR_VAL` 灏嗚繑鍥炴帶浠剁殑褰撳墠鍊硷紝
	`V4L2_CTRL_WHICH_DEF_VAL` 灏嗚繑鍥炴帶浠剁殑榛樿鍊硷紝`V4L2_CTRL_WHICH_MIN_VAL` 灏嗚繑鍥?	鎺т欢鐨勬渶灏忓€硷紝鑰?`V4L2_CTRL_WHICH_MAX_VAL` 灏嗚繑鍥炴帶浠剁殑鏈€澶у€笺€?	`V4L2_CTRL_WHICH_REQUEST_VAL` 琛ㄧず鎺т欢鍊煎繀椤讳粠璇锋眰涓幏鍙栵紝鎴栭拡瀵硅姹傚皾璇?璁剧疆銆?	鍦ㄨ繖绉嶆儏鍐典笅锛宍request_fd` 瀛楁鍖呭惈搴斾娇鐢ㄧ殑璇锋眰鐨勬枃浠舵弿杩扮銆傚鏋滆澶囦笉鏀寔
	璇锋眰锛岄偅涔堝皢杩斿洖 `EACCES`銆?
	浣跨敤 `V4L2_CTRL_WHICH_DEF_VAL`銆乣V4L2_CTRL_WHICH_MIN_VAL` 鎴?	`V4L2_CTRL_WHICH_MAX_VAL` 鏃惰娉ㄦ剰锛屼綘鍙兘鑾峰彇鎺т欢鐨勯粯璁?鏈€灏?鏈€澶у€硷紝涓嶈兘
	璁剧疆鎴栧皾璇曞畠銆?
	鎺т欢鏄惁鏀寔浣跨敤 `V4L2_CTRL_WHICH_MIN_VAL` 鍜?`V4L2_CTRL_WHICH_MAX_VAL` 鏌ヨ
	鏈€灏忓€煎拰鏈€澶у€硷紝鐢?`V4L2_CTRL_FLAG_HAS_WHICH_MIN_MAX` 鏍囧織鎸囩ず銆傚ぇ澶氭暟闈炲鍚?	鎺т欢绫诲瀷閮芥敮鎸佽繖涓€鐐广€傚浜庡叿鏈夊鍚堢被鍨嬬殑鎺т欢锛屾渶灏?鏈€澶у€肩殑瀹氫箟鐢辨帶浠舵枃妗?	鎻愪緵銆傚鏋滀竴涓鍚堟帶浠舵病鏈夎褰曟渶灏?鏈€澶у€肩殑鍚箟锛岄偅涔堟煡璇㈡渶灏忓€兼垨鏈€澶у€煎皢瀵艰嚧
	閿欒鐮?-EINVAL銆?
	涓轰簡鍚戝悗鍏煎锛屼綘涔熷彲浠ュ湪杩欓噷浣跨敤鎺т欢绫伙紙瑙?ctrl-class锛夈€傚湪杩欑鎯呭喌涓嬶紝鎵€鏈?	鎺т欢蹇呴』灞炰簬璇ユ帶浠剁被銆傝繖绉嶇敤娉曞凡琚純鐢紝璇锋敼鐢?`V4L2_CTRL_WHICH_CUR_VAL`銆?	鏈変竴浜涢潪甯歌€佺殑椹卞姩灏氫笉鏀寔 `V4L2_CTRL_WHICH_CUR_VAL`锛岄渶瑕佸湪閭ｉ噷鎸囧畾鎺т欢绫汇€?	浣犲彲浠ラ€氳繃灏?`which` 璁句负 `V4L2_CTRL_WHICH_CUR_VAL` 骞朵互 count 涓?0 璋冪敤
	VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鏉ユ祴璇曟绫婚┍鍔ㄣ€傚鏋滃け璐ワ紝鍒欒椹卞姩涓嶆敮鎸?	`V4L2_CTRL_WHICH_CUR_VAL`銆?    - - __u32
      - `ctrl_class`
      - 涓哄悜鍚庡吋瀹逛繚鐣欑殑寮冪敤鍚嶇О銆傝鏀圭敤 `which`銆?    - - }
      -
    - - __u32
      - `count`
      - controls 鏁扮粍涓殑鎺т欢鏁伴噺銆備篃鍙互涓洪浂銆?    - - __u32
      - `error_idx`
      - 澶辫触鎺т欢鐨勭储寮曘€傚嚭閿欐椂鐢遍┍鍔ㄨ缃€?    - - `2` 濡傛灉閿欒涓庢煇涓壒瀹氭帶浠剁浉鍏宠仈锛岄偅涔?`error_idx` 琚缃负璇ユ帶浠剁殑绱㈠紩銆?	濡傛灉閿欒涓庣壒瀹氭帶浠舵棤鍏筹紝鎴栬€呴獙璇佹楠ゅけ璐ワ紙瑙佷笅鏂囷級锛岄偅涔?`error_idx` 琚缃负
	`count`銆傚鏋?ioctl 杩斿洖 0锛堟垚鍔燂級锛岃鍊兼湭瀹氫箟銆?
	鍦ㄤ粠纭欢璇诲彇/鍐欏叆纭欢涔嬪墠浼氳繘琛屼竴涓獙璇佹楠わ細杩欎細妫€鏌ュ垪琛ㄤ腑鐨勬墍鏈夋帶浠舵槸鍚﹂兘鏄?	鏈夋晥鐨勬帶浠讹紝鏄惁娌℃湁灏濊瘯鍐欏叆鍙鎺т欢鎴栦粠鍙啓鎺т欢璇诲彇锛屼互鍙婁换浣曞叾浠栧彲浠ュ湪涓嶈闂?	纭欢鐨勬儏鍐典笅瀹屾垚鐨勪簨鍓嶆鏌ャ€傛姝ラ鎵€鍋氱殑纭垏楠岃瘉鏄┍鍔ㄧ浉鍏崇殑锛屽洜涓烘煇浜涙鏌ュ彲鑳?	闇€瑕佽闂煇浜涜澶囩殑纭欢锛屼粠鑰屾棤娉曚簨鍓嶅畬鎴愩€傜劧鑰岋紝椹卞姩搴斿敖鏈€澶у姫鍔涜繘琛屽敖鍙兘澶氱殑
	浜嬪墠妫€鏌ャ€?
	杩欐牱鍋氭槸涓轰簡閬垮厤鍥犲鏄撻伩鍏嶇殑闂鑰屼娇纭欢澶勪簬涓嶄竴鑷寸姸鎬併€備絾瀹冨鑷翠簡鍙︿竴涓棶棰橈細
	搴旂敤绋嬪簭闇€瑕佺煡閬撻敊璇槸鏉ヨ嚜楠岃瘉姝ラ锛堟剰鍛崇潃鏈Е鍙婄‖浠讹級杩樻槸鍦ㄥ疄闄呬粠纭欢璇诲彇/鍐欏叆
	纭欢鏈熼棿鍙戠敓鐨勯敊璇€?
	浜嬪悗鐪嬫潵鐩稿綋绯熺硶鐨勮В鍐虫柟妗堟槸灏嗛獙璇佸け璐ユ椂鐨?`error_idx` 璁句负 `count`銆傝繖鏈変竴涓?	涓嶅垢鐨勫壇浣滅敤锛屽嵆鏃犳硶鐪嬪埌鍝釜鎺т欢鏈€氳繃楠岃瘉銆傚鏋滈獙璇佹垚鍔熷苟涓旈敊璇彂鐢熷湪璁块棶纭欢
	鏈熼棿锛岄偅涔?`error_idx` 灏忎簬 `count`锛屽苟涓斿彧鏈夊埌 `error_idx-1` 鐨勬帶浠惰姝ｇ‘鍦?	璇诲彇鎴栧啓鍏ワ紝鍓╀綑鎺т欢鐨勭姸鎬佹湭瀹氫箟銆?
	鐢变簬 VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 涓嶈闂‖浠讹紝鍥犳涔熶笉闇€瑕佷互杩欑鐗规畩鏂瑰紡澶勭悊
	楠岃瘉姝ラ锛屾墍浠?`error_idx` 灏嗚璁句负鏈€氳繃楠岃瘉姝ラ鐨勬帶浠讹紝鑰屼笉鏄?`count`銆傝繖鎰忓懗鐫€
	濡傛灉 VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 浠?`error_idx` 璁句负 `count` 澶辫触锛岄偅涔堜綘鍙互璋冪敤
	VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鏉ュ皾璇曞彂鐜板疄闄呮湭閫氳繃楠岃瘉姝ラ鐨勬帶浠躲€備笉骞哥殑鏄紝
	VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 娌℃湁瀵瑰簲鐨?`TRY`銆?    - - __s32
      - `request_fd`
      - 姝ゆ搷浣滆浣跨敤鐨勮姹傜殑鏂囦欢鎻忚堪绗︺€備粎褰?`which` 琚涓?	`V4L2_CTRL_WHICH_REQUEST_VAL` 鏃舵湁鏁堛€傚鏋滆澶囦笉鏀寔璇锋眰锛岄偅涔堝皢杩斿洖 `EACCES`銆?	濡傛灉鏀寔璇锋眰浣嗙粰鍑轰簡鏃犳晥鐨勮姹傛枃浠舵弿杩扮锛岄偅涔堝皢杩斿洖 `EINVAL`銆?    - - __u32
      - `reserved`\ [^1^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€?
	椹卞姩鍜屽簲鐢ㄧ▼搴忓繀椤诲皢鏁扮粍璁句负闆躲€?    - - struct `v4l2_ext_control` *
      - `controls`
      - 鎸囧悜 `count` 涓?v4l2_ext_control 缁撴瀯鏁扮粍鐨勬寚閽堛€?
	濡傛灉 `count` 绛変簬闆跺垯蹇界暐銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CTRL_CLASS_USER`
      - 0x980000
      - 鍖呭惈鐢ㄦ埛鎺т欢鐨勭被銆傝繖浜涙帶浠跺湪 control 涓弿杩般€傛墍鏈夊彲浠ヤ娇鐢?	VIDIOC_S_CTRL <VIDIOC_G_CTRL> 鍜?VIDIOC_G_CTRL <VIDIOC_G_CTRL> ioctl 璁剧疆鐨勬帶浠堕兘灞炰簬姝ょ被銆?    - - `V4L2_CTRL_CLASS_CODEC`
      - 0x990000
      - 鍖呭惈鏈夌姸鎬佺紪瑙ｇ爜鍣ㄦ帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 codec-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_CAMERA`
      - 0x9a0000
      - 鍖呭惈鎽勫儚澶存帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 camera-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_FM_TX`
      - 0x9b0000
      - 鍖呭惈 FM 鍙戝皠鍣紙FM TX锛夋帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 fm-tx-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_FLASH`
      - 0x9c0000
      - 鍖呭惈闂厜鐏澶囨帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 flash-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_JPEG`
      - 0x9d0000
      - 鍖呭惈 JPEG 鍘嬬缉鎺т欢鐨勭被銆傝繖浜涙帶浠跺湪 jpeg-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_IMAGE_SOURCE`
      - 0x9e0000
      - 鍖呭惈鍥惧儚婧愭帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 image-source-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_IMAGE_PROC`
      - 0x9f0000
      - 鍖呭惈鍥惧儚澶勭悊鎺т欢鐨勭被銆傝繖浜涙帶浠跺湪 image-process-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_FM_RX`
      - 0xa10000
      - 鍖呭惈 FM 鎺ユ敹鍣紙FM RX锛夋帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 fm-rx-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_RF_TUNER`
      - 0xa20000
      - 鍖呭惈 RF 璋冭皭鍣ㄦ帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 rf-tuner-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_DETECT`
      - 0xa30000
      - 鍖呭惈杩愬姩鎴栫墿浣撴娴嬫帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 detect-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_CODEC_STATELESS`
      - 0xa40000
      - 鍖呭惈鏃犵姸鎬佺紪瑙ｇ爜鍣ㄦ帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 codec-stateless-controls 涓弿杩般€?    - - `V4L2_CTRL_CLASS_COLORIMETRY`
      - 0xa50000
      - 鍖呭惈鑹插害瀛︽帶浠剁殑绫汇€傝繖浜涙帶浠跺湪 colorimetry-controls 涓弿杩般€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺琚€傚綋璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    struct `v4l2_ext_control` 鐨?`id` 鏃犳晥锛屾垨 struct `v4l2_ext_controls` 鐨?    `which` 鏃犳晥锛屾垨 struct `v4l2_ext_control` 鐨?`value` 涓嶅悎閫傦紙渚嬪缁欏畾鐨勮彍鍗?    绱㈠紩涓嶈椹卞姩鏀寔锛夛紝鎴?`which` 瀛楁琚涓?`V4L2_CTRL_WHICH_REQUEST_VAL` 浣嗙粰瀹氱殑
    `request_fd` 鏃犳晥鎴?`V4L2_CTRL_WHICH_REQUEST_VAL` 涓嶈鍐呮牳鏀寔銆?    濡傛灉涓や釜鎴栨洿澶氭帶浠跺€煎啿绐侊紝VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鍜?    VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl 涔熶細杩斿洖姝ら敊璇爜銆?
ERANGE
    struct `v4l2_ext_control` 鐨?`value` 瓒婄晫銆?
EBUSY
    鎺т欢鏆傛椂涓嶅彲鏇存敼锛屽彲鑳芥槸鍥犱负鍙︿竴涓簲鐢ㄧ▼搴忔帴绠′簡姝ゆ帶浠舵墍灞炵殑璁惧鍔熻兘锛屾垨锛堝鏋?    `which` 瀛楁琚涓?`V4L2_CTRL_WHICH_REQUEST_VAL`锛夎姹傚凡鎺掗槦浣嗗皻鏈畬鎴愩€?
ENOSPC
    涓烘帶浠惰礋杞戒繚鐣欑殑绌洪棿涓嶈冻銆俙size` 瀛楁琚涓轰竴涓冻澶熷瓨鍌ㄨ礋杞界殑鍊硷紝骞惰繑鍥炴閿欒鐮併€?
EACCES
    璇曞浘灏濊瘯鎴栬缃彧璇绘帶浠讹紝鎴栬幏鍙栧彧鍐欐帶浠讹紝鎴栦粠灏氭湭瀹屾垚鐨勮姹備腑鑾峰彇鎺т欢銆?
    鎴栬€?`which` 瀛楁琚涓?`V4L2_CTRL_WHICH_REQUEST_VAL` 浣嗚澶囦笉鏀寔璇锋眰銆?
    鎴栬€呭鏋滄湁璇曞浘璁剧疆涓€涓潪娲诲姩鎺т欢鐨勬搷浣滐紝涓旈┍鍔ㄦ棤娉曠紦瀛樻柊鍊肩洿鍒拌鎺т欢鍐嶆娲诲姩銆?