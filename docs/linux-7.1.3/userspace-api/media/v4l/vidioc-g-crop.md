

######## ioctl VIDIOC_G_CROP, VIDIOC_S_CROP


## 鍚嶇О


VIDIOC_G_CROP - VIDIOC_S_CROP - 鑾峰彇鎴栬缃綋鍓嶇殑瑁佸壀鐭╁舰

## 璇硶


`int ioctl(int fd, VIDIOC_G_CROP, struct v4l2_crop *argp)`


`int ioctl(int fd, VIDIOC_S_CROP, const struct v4l2_crop *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜缁撴瀯浣?`v4l2_crop` 鐨勬寚閽堛€?
## 鎻忚堪


涓轰簡鏌ヨ瑁佸壀鐭╁舰鐨勫ぇ灏忓拰浣嶇疆锛屽簲鐢ㄧ▼搴忔妸缁撴瀯浣?`v4l2_crop` 鐨?`type`
瀛楁璁剧疆涓虹浉搴旂殑缂撳啿鍖猴紙娴侊級绫诲瀷锛屽苟浠ユ寚鍚戣缁撴瀯浣撶殑鎸囬拡璋冪敤 VIDIOC_G_CROP <VIDIOC_G_CROP> ioctl銆?椹卞姩浼氬～鍏呯粨鏋勪綋鐨勫叾浣欓儴鍒嗭紝濡傛灉涓嶆敮鎸佽鍓垯杩斿洖 `EINVAL` 閿欒鐮併€?
涓轰簡鏀瑰彉瑁佸壀鐭╁舰锛屽簲鐢ㄧ▼搴忓垵濮嬪寲 v4l2_crop 缁撴瀯浣撲腑鐨?`type` 瀛楁
浠ュ強鍚嶄负 `c` 鐨?`v4l2_rect` 瀛愮粨鏋勪綋锛屽苟浠ユ寚鍚戣缁撴瀯浣撶殑鎸囬拡璋冪敤
VIDIOC_S_CROP <VIDIOC_G_CROP> ioctl銆?
椹卞姩棣栧厛鏍规嵁纭欢闄愬埗锛堝嵆鐢辨崟鑾?杈撳嚭绐楀彛缁欏嚭鐨勮竟鐣岋級璋冩暣璇锋眰鐨勫昂瀵革紝
骞舵妸姘村钩鍜屽瀭鐩村亸绉汇€佸搴﹀拰楂樺害鑸嶅叆鍒版渶鎺ヨ繎鐨勫彲鑳藉€笺€傜壒鍒槸锛岄┍鍔ㄥ繀椤绘妸瑁佸壀
鐭╁舰鐨勫瀭鐩村亸绉昏垗鍏ヤ负甯ц鏁版ā浜岋紝浠ラ伩鍏嶅瓧娈甸『搴忚娣锋穯銆?
鍏舵锛岄┍鍔ㄥ湪淇濇寔褰撳墠姘村钩鍜屽瀭鐩寸缉鏀惧洜瀛愮殑鍓嶆彁涓嬶紝鎶婂浘鍍忓ぇ灏忥紙缂╂斁杩囩▼涓?鐩稿鐨勭煩褰紝婧愭垨鐩爣鍙栧喅浜庢暟鎹柟鍚戯級璋冩暣涓烘渶鎺ヨ繎鐨勫彲鑳藉ぇ灏忋€?
鏈€鍚庯紝椹卞姩鐢ㄥ疄闄呯殑瑁佸壀鍜屽浘鍍忓弬鏁板纭欢杩涜缂栫▼銆俈IDIOC_S_CROP <VIDIOC_G_CROP>
鏄竴涓彧鍐?ioctl锛屽畠涓嶈繑鍥炲疄闄呭弬鏁般€傝鏌ヨ杩欎簺鍙傛暟锛屽簲鐢ㄧ▼搴忓繀椤昏皟鐢?VIDIOC_G_CROP <VIDIOC_G_CROP> 鍜?VIDIOC_G_FMT銆傚綋鍙傛暟涓嶅悎閫傛椂锛屽簲鐢ㄧ▼搴忓彲浠?淇敼瑁佸壀鎴栧浘鍍忓弬鏁板苟閲嶅璇ュ惊鐜紝鐩村埌鍗忓晢鍑烘弧鎰忕殑鍙傛暟銆?
褰撲笉鏀寔瑁佸壀鏃讹紝涓嶄細淇敼浠讳綍鍙傛暟锛孷IDIOC_S_CROP <VIDIOC_G_CROP> 杩斿洖 `EINVAL` 閿欒鐮併€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 鏁版嵁绫诲瀷锛岀敱搴旂敤绋嬪簭璁剧疆銆傛澶勪粎浠ヤ笅绫诲瀷鏈夋晥锛歚V4L2_BUF_TYPE_VIDEO_CAPTURE`銆乣V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`銆?	`V4L2_BUF_TYPE_VIDEO_OUTPUT`銆乣V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE` 浠ュ強
	`V4L2_BUF_TYPE_VIDEO_OVERLAY`銆傚弬瑙?`v4l2_buf_type` 浠ュ強涓嬮潰鐨勮鏄庛€?    - - struct `v4l2_rect`
      - `c`
      - 瑁佸壀鐭╁舰銆備娇鐢ㄧ殑鍧愭爣绯讳笌缁撴瀯浣?`v4l2_cropcap` 鐨?`bounds` 鐩稿悓銆?
   閬楁喚鐨勬槸锛屽湪澶氬钩闈㈢紦鍐插尯绫诲瀷锛坄V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE` 鍜?`V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`锛?   鐨勬儏鍐典笅锛屽叧浜庡簲濡備綍濉啓 `v4l2_crop` 鐨?`type` 瀛楁锛岃 API 涓€搴︽贩涔便€傛煇浜涢┍鍔?   鍙帴鍙?`_MPLANE` 缂撳啿鍖虹被鍨嬶紝鑰屽叾浠栭┍鍔ㄥ彧鎺ュ彈闈炲骞抽潰缂撳啿鍖虹被鍨嬶紙鍗充笉甯︽湯灏剧殑
   `_MPLANE`锛夈€?
   浠庡唴鏍?4.13 璧凤紝涓ょ鍐欐硶閮借鍏佽銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
閫氱敤閿欒鐮?<gen-errors> 绔犺妭涓弿杩般€?
ENODATA
    璇ヨ緭鍏ユ垨杈撳嚭涓嶆敮鎸佽鍓€?
