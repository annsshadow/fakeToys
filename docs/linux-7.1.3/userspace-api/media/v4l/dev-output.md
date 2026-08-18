
######## 瑙嗛杈撳嚭鎺ュ彛


瑙嗛杈撳嚭璁惧灏嗛潤鐗╂垨鍥惧儚搴忓垪缂栫爜涓烘ā鎷熻棰戜俊鍙枫€傞€氳繃璇ユ帴鍙ｏ紝搴旂敤绋嬪簭鍙互鎺у埗缂栫爜杩囩▼
骞跺皢鍥惧儚浠庣敤鎴风┖闂寸Щ鍔ㄥ埌椹卞姩涓€?
鎸夌収鎯緥锛孷4L2 瑙嗛杈撳嚭璁惧閫氳繃鍚嶄负 `/dev/video` 涓?`/dev/video0` 鍒?`/dev/video63` 鐨?瀛楃璁惧鐗规畩鏂囦欢璁块棶锛屼富璁惧鍙蜂负 81锛屾璁惧鍙蜂负 0 鍒?63銆俙/dev/video` 閫氬父鏄埌棣栭€夎棰?璁惧鐨勭鍙烽摼鎺ャ€?

## 鏌ヨ鑳藉姏


鏀寔瑙嗛杈撳嚭鎺ュ彛鐨勮澶囧湪 VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct `v4l2_capability` 鐨?`capabilities` 瀛楁涓缃?`V4L2_CAP_VIDEO_OUTPUT` 鎴?`V4L2_CAP_VIDEO_OUTPUT_MPLANE` 鏍囧織銆?浣滀负杈呭姪璁惧鍔熻兘锛屽畠浠篃鍙兘鏀寔鍘熷 VBI 杈撳嚭 <raw-vbi>锛坄V4L2_CAP_VBI_OUTPUT`锛夋帴鍙ｃ€?蹇呴』鑷冲皯鏀寔璇?鍐欐垨娴佸紡 I/O 鏂规硶涔嬩竴銆傝皟鍒跺櫒涓庨煶棰戣緭鍑烘槸鍙€夌殑銆?
## 琛ュ厖鍔熻兘


瑙嗛杈撳嚭璁惧搴旀牴鎹渶瑕佹敮鎸侀煶棰戣緭鍑?<audio>銆佽皟鍒跺櫒 <tuner>銆佹帶浠?<control>銆佽鍓笌
缂╂斁 <crop> 浠ュ強娴佸紡鍙傛暟 <streaming-par> ioctl銆傛墍鏈夎棰戣緭鍑鸿澶囬兘蹇呴』鏀寔瑙嗛杈撳嚭
<video> ioctl銆?
## 鍥惧儚鏍煎紡鍗忓晢


杈撳嚭鐢辫鍓笌鍥惧儚鏍煎紡鍙傛暟鍐冲畾銆傚墠鑰呴€夋嫨鍥惧儚灏嗗嚭鐜扮殑瑙嗛鐢婚潰鍖哄煙锛屽悗鑰呭喅瀹氬浘鍍忓浣曞瓨鍌ㄤ簬
鍐呭瓨涓紝鍗充互 RGB 杩樻槸 YUV 鏍煎紡銆佹瘡鍍忕礌浣嶆暟鎴栧楂樸€傚畠浠叡鍚屼篃瀹氫箟浜嗗浘鍍忓湪澶勭悊杩囩▼涓浣?缂╂斁銆?
鍍忓線甯镐竴鏍凤紝杩欎簺鍙傛暟鍦?`open()` 鏃?*涓嶄細**琚噸缃紝浠ュ厑璁?Unix 宸ュ叿閾惧皢璁惧缂栫▼鍚庡儚鍐欏叆
鏅€氭枃浠朵竴鏍峰啓鍏ュ畠銆傜紪鍐欒壇濂界殑 V4L2 搴旂敤绋嬪簭浼氱‘淇濆畠浠湡姝ｅ緱鍒版兂瑕佺殑缁撴灉锛屽寘鎷鍓笌缂╂斁銆?
瑁佸壀鍒濆鍖栬嚦灏戦渶瑕佸皢鍙傛暟閲嶇疆涓洪粯璁ゅ€笺€俢rop 涓粰鍑轰簡涓€涓ず渚嬨€?
瑕佹煡璇㈠綋鍓嶅浘鍍忔牸寮忥紝搴旂敤绋嬪簭灏?struct `v4l2_format` 鐨?`type` 瀛楁璁句负
`V4L2_BUF_TYPE_VIDEO_OUTPUT` 鎴?`V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`锛屽苟浠ユ寚鍚戣缁撴瀯鐨?鎸囬拡璋冪敤 VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl銆傞┍鍔ㄥ～鍏?struct `v4l2_pix_format` 鐨?`pix`
鎴愬憳鎴?struct `v4l2_pix_format_mplane` 鐨?`pix_mp` 鎴愬憳锛堝睘浜?`fmt` 鑱斿悎浣擄級銆?
瑕佽姹備笉鍚岀殑鍙傛暟锛屽簲鐢ㄧ▼搴忓儚涓婇潰閭ｆ牱璁剧疆 struct `v4l2_format` 鐨?`type` 瀛楁锛屽苟鍒濆鍖?`fmt` 鑱斿悎浣撶殑 struct `v4l2_pix_format` 鐨?`vbi` 鎴愬憳鐨勬墍鏈夊瓧娈碉紝鎴栬€呮洿濂界殑鍋氭硶鏄彧淇敼
VIDIOC_G_FMT <VIDIOC_G_FMT> 鐨勭粨鏋滐紝鐒跺悗浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?VIDIOC_S_FMT <VIDIOC_G_FMT>
ioctl銆傞┍鍔ㄥ彲鑳戒細璋冩暣鍙傛暟锛屽苟鏈€缁堝儚 VIDIOC_G_FMT <VIDIOC_G_FMT> 閭ｆ牱杩斿洖瀹為檯鍙傛暟銆?
涓?VIDIOC_S_FMT <VIDIOC_G_FMT> 绫讳技锛孷IDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 鍙敤浜庝簡瑙ｇ‖浠?闄愬埗锛岃€屾棤闇€绂佺敤 I/O 鎴栧彲鑳借€楁椂鐨勭‖浠跺噯澶囥€?
struct `v4l2_pix_format` 涓?struct `v4l2_pix_format_mplane` 鐨勫唴瀹瑰湪 pixfmt 涓璁恒€傛湁鍏?缁嗚妭鍙﹁鍙傝 VIDIOC_G_FMT <VIDIOC_G_FMT>銆乂IDIOC_S_FMT <VIDIOC_G_FMT> 涓?VIDIOC_TRY_FMT
<VIDIOC_G_FMT> ioctl 鐨勮鑼冦€傝棰戣緭鍑鸿澶囧繀椤诲疄鐜?VIDIOC_G_FMT <VIDIOC_G_FMT> 涓?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl锛屽嵆浣?VIDIOC_S_FMT <VIDIOC_G_FMT> 蹇界暐鎵€鏈夎姹傚苟鎬绘槸
鍍?VIDIOC_G_FMT <VIDIOC_G_FMT> 閭ｆ牱杩斿洖榛樿鍙傛暟銆俈IDIOC_TRY_FMT <VIDIOC_G_FMT> 鏄彲閫夌殑銆?
## 鍐欏叆鍥惧儚


瑙嗛杈撳嚭璁惧鍙兘鏀寔 write() 鍑芥暟 <rw> 鍜?鎴栨祦寮忥紙鍐呭瓨鏄犲皠 <mmap> 鎴栫敤鎴锋寚閽?<userp>锛塈/O銆傝瑙?io銆?