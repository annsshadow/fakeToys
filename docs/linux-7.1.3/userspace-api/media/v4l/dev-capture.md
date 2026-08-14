


######## 瑙嗛鎹曡幏鎺ュ彛锛圴ideo Capture Interface锛?

瑙嗛鎹曡幏璁惧瀵规ā鎷熻棰戜俊鍙疯繘琛岄噰鏍凤紝骞跺皢鏁板瓧鍖栧悗鐨勫浘鍍忓瓨鍌ㄥ湪鍐呭瓨涓€傚浠婂嚑涔庢墍鏈夎澶囬兘鑳戒互瀹屾暣鐨?25 鎴?30 甯?绉掕繘琛屾崟鑾枫€傞€氳繃璇ユ帴鍙ｏ紝搴旂敤绋嬪簭鍙互鎺у埗鎹曡幏杩囩▼骞跺皢鍥惧儚浠庨┍鍔ㄧЩ鍔ㄥ埌鐢ㄦ埛绌洪棿銆?
鎸夌収鎯緥锛孷4L2 瑙嗛鎹曡幏璁惧閫氳繃鍚嶄负 `/dev/video` 浠ュ強 `/dev/video0` 鍒?`/dev/video63` 鐨勫瓧绗﹁澶囩壒娈婃枃浠惰闂紝涓昏澶囧彿涓?81锛屾璁惧鍙蜂负 0 鍒?63銆俙/dev/video` 閫氬父鏄寚鍚戦閫夎棰戣澶囩殑绗﹀彿閾炬帴銆?

## 鏌ヨ鑳藉姏


鏀寔瑙嗛鎹曡幏鎺ュ彛鐨勮澶囦細鍦?VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct `v4l2_capability` 鐨?`capabilities` 瀛楁涓缃?`V4L2_CAP_VIDEO_CAPTURE` 鎴?`V4L2_CAP_VIDEO_CAPTURE_MPLANE` 鏍囧織銆備綔涓烘瑕佽澶囧姛鑳斤紝瀹冧滑涔熷彲鑳芥敮鎸佽棰戝彔鍔?<overlay>锛坄V4L2_CAP_VIDEO_OVERLAY`锛夊拰鍘熷 VBI 鎹曡幏 <raw-vbi>锛坄V4L2_CAP_VBI_CAPTURE`锛夋帴鍙ｃ€傚繀椤昏嚦灏戞敮鎸佽/鍐欐垨娴佸紡 I/O 鏂规硶涔嬩竴銆傝皟璋愬櫒锛坱uner锛夊拰闊抽杈撳叆鏄彲閫夌殑銆?
## 杈呭姪鍔熻兘


瑙嗛鎹曡幏璁惧搴旀牴鎹渶瑕佹敮鎸侀煶棰戣緭鍏?<audio>銆乼uner銆佹帶鍒?<control>銆佽鍓笌缂╂斁 <crop> 浠ュ強娴佸弬鏁?<streaming-par> ioctls銆傛墍鏈夎棰戞崟鑾疯澶囬兘蹇呴』鏀寔瑙嗛杈撳叆 <video> ioctls銆?
## 鍥惧儚鏍煎紡鍗忓晢


鎹曡幏鎿嶄綔鐨勭粨鏋滅敱瑁佸壀鍜屽浘鍍忔牸寮忓弬鏁板喅瀹氥€傚墠鑰呴€夋嫨瑕佹崟鑾风殑瑙嗛鐢婚潰鍖哄煙锛屽悗鑰呭喅瀹氬浘鍍忓浣曞瓨鍌ㄥ湪鍐呭瓨涓紝鍗?RGB 鎴?YUV 鏍煎紡銆佹瘡鍍忕礌浣嶆暟鎴栧鍜岄珮銆傚畠浠竴璧疯繕瀹氫箟浜嗗湪杩囩▼涓浘鍍忓浣曡缂╂斁銆?
鍍忓線甯镐竴鏍凤紝杩欎簺鍙傛暟鍦?`open()` 鏃?*涓嶄細**琚噸缃紝浠ュ厑璁稿 Unix 宸ュ叿閾撅細鍏堝璁惧缂栫▼锛岀劧鍚庡儚璇诲彇鏅€氭枃浠朵竴鏍疯鍙栧畠銆傜紪鍐欒壇濂界殑 V4L2 搴旂敤绋嬪簭浼氱‘淇濆畠浠湡姝ｅ緱鍒版兂瑕佺殑缁撴灉锛屽寘鎷鍓拰缂╂斁銆?
瑁佸壀鍒濆鍖栬嚦灏戦渶瑕佸皢鍙傛暟閲嶇疆涓洪粯璁ゅ€笺€傜ず渚嬭 crop銆?
涓烘煡璇㈠綋鍓嶅浘鍍忔牸寮忥紝搴旂敤绋嬪簭灏?struct `v4l2_format` 鐨?`type` 瀛楁璁句负 `V4L2_BUF_TYPE_VIDEO_CAPTURE` 鎴?`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`锛屽苟浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl銆傞┍鍔ㄥ～鍏?`fmt` 鑱斿悎涓?struct `v4l2_pix_format` 鐨?`pix` 鎴愬憳鎴?struct `v4l2_pix_format_mplane` 鐨?`pix_mp` 鎴愬憳銆?
涓鸿姹備笉鍚岀殑鍙傛暟锛屽簲鐢ㄧ▼搴忓儚涓婇潰涓€鏍疯缃?struct `v4l2_format` 鐨?`type` 瀛楁锛屽苟鍒濆鍖?`fmt` 鑱斿悎涓?struct `v4l2_pix_format` 鐨?`vbi` 鎴愬憳鐨勬墍鏈夊瓧娈碉紝鎴栬€呮洿濂藉湴浠呬慨鏀?VIDIOC_G_FMT <VIDIOC_G_FMT> 鐨勭粨鏋滐紝鐒跺悗浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl銆傞┍鍔ㄥ彲浠ヨ皟鏁村弬鏁帮紝骞舵渶缁堝儚 VIDIOC_G_FMT <VIDIOC_G_FMT> 閭ｆ牱杩斿洖瀹為檯鍙傛暟銆?
涓?VIDIOC_S_FMT <VIDIOC_G_FMT> 绫讳技锛孷IDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 鍙敤浜庡湪涓嶇鐢?I/O 鎴栧彲鑳借€楁椂鐨勭‖浠跺噯澶囩殑鎯呭喌涓嬩簡瑙ｇ‖浠堕檺鍒躲€?
struct `v4l2_pix_format` 鍜?struct `v4l2_pix_format_mplane` 鐨勫唴瀹瑰湪 pixfmt 涓璁恒€傜粏鑺傚彟瑙?VIDIOC_G_FMT <VIDIOC_G_FMT>銆乂IDIOC_S_FMT <VIDIOC_G_FMT> 鍜?VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 鐨勮鑼冦€傝棰戞崟鑾疯澶囧繀椤诲疄鐜?VIDIOC_G_FMT <VIDIOC_G_FMT> 鍜?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl锛屽嵆浣?VIDIOC_S_FMT <VIDIOC_G_FMT> 蹇界暐鎵€鏈夎姹傚苟鎬绘槸鍍?VIDIOC_G_FMT <VIDIOC_G_FMT> 閭ｆ牱杩斿洖榛樿鍙傛暟銆俈IDIOC_TRY_FMT <VIDIOC_G_FMT> 鏄彲閫夌殑銆?
## 璇诲彇鍥惧儚


瑙嗛鎹曡幏璁惧鍙互鏀寔 read() 鍑芥暟 <func-read> 鍜?鎴栨祦寮忥紙鍐呭瓨鏄犲皠 <func-mmap> 鎴栫敤鎴锋寚閽?<userp>锛塈/O銆傝鎯呰 io銆?