
######## ioctl VIDIOC_CROPCAP


## 鍚嶇О


VIDIOC_CROPCAP - 鍏充簬瑙嗛瑁佸壀涓庣缉鏀捐兘鍔涚殑淇℃伅

## 璇硶


`int ioctl(int fd, VIDIOC_CROPCAP, struct v4l2_cropcap *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_cropcap` 鐨勬寚閽堛€?
## 鎻忚堪


搴旂敤绋嬪簭浣跨敤姝ゅ嚱鏁版煡璇㈣鍓檺鍒躲€佸浘鍍忓儚绱犲楂樻瘮骞惰绠楃缉鏀惧洜瀛愩€傚畠浠皢 v4l2_cropcap 缁撴瀯鐨?`type` 瀛楁璁句负鐩稿簲鐨勭紦鍐插尯锛堟祦锛夌被鍨嬶紝骞朵互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 VIDIOC_CROPCAP ioctl銆?椹卞姩濉厖缁撴瀯鐨勫叾浣欓儴鍒嗐€傞櫎鍒囨崲瑙嗛鏍囧噯澶栵紝缁撴灉鏄亽瀹氱殑銆傝璁颁綇锛屽垏鎹㈣棰戣緭鍏ユ垨杈撳嚭鏃?鍙兘浼氶殣寮忓彂鐢熻繖绉嶅垏鎹€?
璇?ioctl 蹇呴』鐢辨敮鎸佽鍓拰/鎴栫缉鏀惧拰/鎴栧叿鏈夐潪鏂瑰舰鍍忕礌鐨勮棰戞崟鑾锋垨杈撳嚭璁惧锛屼互鍙婅鐩栵紙overlay锛?璁惧瀹炵幇銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 鏁版嵁娴佺殑绫诲瀷锛岀敱搴旂敤绋嬪簭璁剧疆銆傛澶勪粎浠ヤ笅绫诲瀷鏈夋晥锛歚V4L2_BUF_TYPE_VIDEO_CAPTURE`銆?	`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`銆乣V4L2_BUF_TYPE_VIDEO_OUTPUT`銆?	`V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE` 涓?`V4L2_BUF_TYPE_VIDEO_OVERLAY`銆?	鍙傝 `v4l2_buf_type` 鍙婁笅闈㈢殑璇存槑銆?    - - struct v4l2_rect <v4l2-rect-crop>
      - `bounds`
      - 瀹氫箟鍙繘琛屾崟鑾锋垨杈撳嚭鐨勭獥鍙ｏ紝杩欏彲鑳芥帓闄や緥濡傛按骞冲拰鍨傜洿娑堥殣鍖恒€傝鍓煩褰笉鑳借秴杩囪繖浜?	闄愬埗銆傚搴︿笌楂樺害浠ュ儚绱犲畾涔夛紝椹卞姩缂栧啓鑰呭彲鑷敱閫夋嫨鍦ㄦā鎷熷煙涓潗鏍囩郴鐨勫師鐐逛笌鍗曚綅銆?    - - struct v4l2_rect <v4l2-rect-crop>
      - `defrect`
      - 榛樿瑁佸壀鐭╁舰锛屽畠搴旇鐩栤€滄暣骞呯敾闈⑩€濄€傚亣璁惧儚绱犲楂樻瘮涓?1/1锛屽浜?NTSC 鍙互鏄緥濡?	640 脳 480 鐨勭煩褰紝瀵逛簬 PAL 涓?SECAM 鍙互鏄眳涓簬娲诲姩鐢婚潰鍖哄煙鐨?768 脳 576 鐭╁舰銆?	浣跨敤涓?`bounds` 鐩稿悓鐨勫潗鏍囩郴銆?    - - struct `v4l2_fract`
      - `pixelaspect`
      - 杩欐槸鏈簲鐢ㄧ缉鏀炬椂鐨勫儚绱犲楂樻瘮锛坹 / x锛夛紝鍗冲疄闄呴噰鏍烽鐜囦笌鑾峰緱鏂瑰舰鍍忕礌鎵€闇€棰戠巼涔嬫瘮銆?
	褰撹鍓潗鏍囨寚鍚戞柟褰㈠儚绱犳椂锛岄┍鍔ㄥ皢 `pixelaspect` 璁句负 1/1銆傚叾浠栧父瑙佸€间负 PAL 涓?SECAM
	鐨?54/59锛屼互鍙婃寜 [itu601] 閲囨牱鐨?NTSC 鐨?11/10銆?
   涓嶅垢鐨勬槸锛屽湪澶氬钩闈㈢紦鍐插尯绫诲瀷锛坄V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE` 涓?   `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`锛夌殑鎯呭喌涓嬶紝鍏充簬搴斿浣曞～鍐?`v4l2_cropcap` 鐨?   `type` 瀛楁锛岃 API 瀛樺湪娣蜂贡銆傛煇浜涢┍鍔ㄥ彧鎺ュ彈 `_MPLANE` 缂撳啿鍖虹被鍨嬶紝鑰屽叾浠栭┍鍔ㄥ彧鎺ュ彈
   闈炲骞抽潰缂撳啿鍖虹被鍨嬶紙鍗虫湯灏句笉甯?`_MPLANE`锛夈€?
   浠庡唴鏍?4.13 璧凤紝涓ょ鍙樹綋閮藉厑璁搞€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s32
      - `left`
      - 鐭╁舰宸︿笂瑙掔殑姘村钩鍋忕Щ锛屼互鍍忕礌璁°€?    - - __s32
      - `top`
      - 鐭╁舰宸︿笂瑙掔殑鍨傜洿鍋忕Щ锛屼互鍍忕礌璁°€?    - - __u32
      - `width`
      - 鐭╁舰鐨勫搴︼紝浠ュ儚绱犺銆?    - - __u32
      - `height`
      - 鐭╁舰鐨勯珮搴︼紝浠ュ儚绱犺銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    struct `v4l2_cropcap` 鐨?`type` 鏃犳晥銆?
ENODATA
    璇ヨ緭鍏ユ垨杈撳嚭涓嶆敮鎸佽鍓€?