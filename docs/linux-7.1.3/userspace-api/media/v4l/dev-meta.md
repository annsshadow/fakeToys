


######## 鍏冩暟鎹帴鍙ｏ紙Metadata Interface锛?

鍏冩暟鎹槸鎸囦换浣曡ˉ鍏呰棰戝抚鐨勯澶栦俊鎭殑闈炲浘鍍忔暟鎹€傝繖鍙兘鍖呮嫭鍩轰簬鍥惧儚璁＄畻鐨勭粺璁￠噺銆佸浘鍍忔簮鎻愪緵鐨勫抚鎹曡幏鍙傛暟锛屾垨鐢ㄤ簬鎸囧畾璁惧濡備綍澶勭悊鍥惧儚鐨勭壒瀹氳澶囧弬鏁般€傝鎺ュ彛鐢ㄤ簬鍦ㄧ敤鎴风┖闂村拰纭欢涔嬮棿浼犺緭鍏冩暟鎹苟鎺у埗璇ユ搷浣溿€?
鍏冩暟鎹帴鍙ｅ湪瑙嗛璁惧鑺傜偣涓婂疄鐜般€傝澶囧彲浠ヤ笓鐢ㄤ簬鍏冩暟鎹紝涔熷彲浠ユ牴鎹叾鎶ュ憡鐨勮兘鍔涘悓鏃舵敮鎸佽棰戝拰鍏冩暟鎹€?
## 鏌ヨ鑳藉姏


鏀寔鍏冩暟鎹崟鑾锋帴鍙ｇ殑璁惧鑺傜偣浼氬湪 `VIDIOC_QUERYCAP` ioctl 杩斿洖鐨?`v4l2_capability` 缁撴瀯鐨?`device_caps` 瀛楁涓缃?`V4L2_CAP_META_CAPTURE` 鏍囧織銆傝鏍囧織琛ㄧず璁惧鍙互灏嗗厓鏁版嵁鎹曡幏鍒板唴瀛樸€傜被浼煎湴锛屾敮鎸佸厓鏁版嵁杈撳嚭鎺ュ彛鐨勮澶囪妭鐐瑰湪 `v4l2_capability` 缁撴瀯鐨?`device_caps` 瀛楁涓缃?`V4L2_CAP_META_OUTPUT` 鏍囧織銆傝鏍囧織琛ㄧず璁惧鍙互浠庡唴瀛樿鍙栧厓鏁版嵁銆?
蹇呴』鑷冲皯鏀寔璇?鍐欐垨娴佸紡 I/O 鏂规硶涔嬩竴銆?

## 鏁版嵁鏍煎紡鍗忓晢


鍏冩暟鎹澶囦娇鐢?format ioctl 鏉ラ€夋嫨鎹曡幏鏍煎紡銆傚厓鏁版嵁缂撳啿鍖虹殑鍏у鏍煎紡缁戝畾鍒版墍閫夋牸寮忋€傞櫎浜嗗熀鏈殑 format ioctl锛宍VIDIOC_ENUM_FMT` ioctl 涔熷繀椤绘敮鎸併€?
涓轰娇鐢?format ioctl锛屽簲鐢ㄧ▼搴忓皢 `v4l2_format` 缁撴瀯鐨?`type` 瀛楁璁句负 `V4L2_BUF_TYPE_META_CAPTURE` 鎴?`V4L2_BUF_TYPE_META_OUTPUT`锛屽苟鏍规嵁鎵€闇€鎿嶄綔鎸夐渶浣跨敤 `fmt` 鑱斿悎鐨?`v4l2_meta_format` 鐨?`meta` 鎴愬憳銆傞┍鍔ㄥ拰搴旂敤绋嬪簭閮藉繀椤诲皢 `v4l2_format` 缁撴瀯鐨勫叾浣欓儴鍒嗚涓?0銆?
鎸夎鎹曡幏鍏冩暟鎹殑璁惧鍦?`VIDIOC_ENUM_FMT` 鏃惰缃簡 struct v4l2_fmtdesc 鐨?`V4L2_FMT_FLAG_META_LINE_BASED` 鏍囧織銆傛绫昏澶囬€氬父涔熷彲浠ユ崟鑾峰浘鍍忔暟鎹?<capture>銆傝繖涓昏娑夊強浠庡叾浠栬澶囷紙濡傜浉鏈轰紶鎰熷櫒锛夋帴鏀舵暟鎹殑璁惧銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `dataformat`
      - 鏁版嵁鏍煎紡锛岀敱搴旂敤绋嬪簭璁剧疆銆傝繖鏄竴涓皬绔簭鐨勫洓瀛楃鐮?<v4l2-fourcc>銆俈4L2 鍦?meta-formats 涓畾涔変簡鍏冩暟鎹牸寮忋€?    - - __u32
      - `buffersize`
      - 鏁版嵁鎵€闇€鐨勬渶澶х紦鍐插尯澶у皬锛堝瓧鑺傦級銆傝鍊肩敱椹卞姩璁剧疆銆?    - - __u32
      - `width`
      - 涓€琛屽厓鏁版嵁鍦ㄢ€滄暟鎹崟鍏冣€濅腑鐨勫搴︺€傚綋 :c:type`v4l2_fmtdesc` 鏍囧織 `V4L2_FMT_FLAG_META_LINE_BASED` 琚缃椂鏈夋晥锛屽惁鍒欎负闆躲€傚弬瑙?`VIDIOC_ENUM_FMT`銆?    - - __u32
      - `height`
      - 鍏冩暟鎹鏁般€傚綋 :c:type`v4l2_fmtdesc` 鏍囧織 `V4L2_FMT_FLAG_META_LINE_BASED` 琚缃椂鏈夋晥锛屽惁鍒欎负闆躲€傚弬瑙?`VIDIOC_ENUM_FMT`銆?    - - __u32
      - `bytesperline`
      - 涓や釜杩炵画琛岀殑璧峰涔嬮棿鐨勫瓧鑺傚亸绉汇€傚綋 :c:type`v4l2_fmtdesc` 鏍囧織 `V4L2_FMT_FLAG_META_LINE_BASED` 琚缃椂鏈夋晥锛屽惁鍒欎负闆躲€傚弬瑙?`VIDIOC_ENUM_FMT`銆?