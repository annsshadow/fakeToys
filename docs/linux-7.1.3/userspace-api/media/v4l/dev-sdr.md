######## 杞欢瀹氫箟鏃犵嚎鐢垫帴鍙ｏ紙SDR锛?

SDR 鏄?Software Defined Radio锛堣蒋浠跺畾涔夋棤绾跨數锛夌殑缂╁啓锛屽嵆浣跨敤搴旂敤杞欢杩涜
璋冨埗鎴栬В璋冪殑鏃犵嚎鐢佃澶囥€傝鎺ュ彛鐢ㄤ簬鎺у埗姝ょ被璁惧骞惰繘琛屾暟鎹祦浼犺緭銆?
SDR 璁惧閫氳繃鍚嶄负 `/dev/swradio0` 鍒?`/dev/swradio255` 鐨勫瓧绗﹁澶囩壒娈婃枃浠?璁块棶锛屼富璁惧鍙蜂负 81锛屾璁惧鍙峰湪 0 鍒?255 涔嬮棿鍔ㄦ€佸垎閰嶃€?
## 鏌ヨ鑳藉姏


鏀寔 SDR 鎺ユ敹鍣ㄦ帴鍙ｇ殑璁惧锛屼細鍦?VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct
`v4l2_capability` 鐨?`capabilities` 瀛楁涓缃?`V4L2_CAP_SDR_CAPTURE` 鍜?`V4L2_CAP_TUNER` 鏍囧織銆傝鏍囧織琛ㄧず璁惧鍏锋湁
妯℃暟杞崲鍣紙ADC锛夛紝瀹冩槸 SDR 鎺ユ敹鍣ㄧ殑蹇呴渶鍏冧欢銆?
鏀寔 SDR 鍙戦€佸櫒鎺ュ彛鐨勮澶囷紝浼氬湪 VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct
`v4l2_capability` 鐨?`capabilities` 瀛楁涓缃?`V4L2_CAP_SDR_OUTPUT` 鍜?`V4L2_CAP_MODULATOR` 鏍囧織銆傝鏍囧織琛ㄧず璁惧鍏锋湁
鏁版ā杞崲鍣紙DAC锛夛紝瀹冩槸 SDR 鍙戦€佸櫒鐨勫繀闇€鍏冧欢銆?
蹇呴』鑷冲皯鏀寔璇?鍐欐垨娴?I/O 鏂规硶涔嬩竴銆?
## 杈呭姪鍔熻兘


SDR 璁惧鍙互鏀寔 controls <control>锛屽苟涓斿繀椤绘敮鎸?tuner ioctls銆倀uner
ioctls 鐢ㄤ簬璁剧疆 ADC/DAC 閲囨牱鐜囷紙閲囨牱棰戠巼锛変互鍙婂彲鑳界殑鏃犵嚎鐢甸鐜囷紙RF锛夈€?
`V4L2_TUNER_SDR` tuner 绫诲瀷鐢ㄤ簬璁剧疆 SDR 璁惧鐨?ADC/DAC 棰戠巼锛宍V4L2_TUNER_RF`
tuner 绫诲瀷鐢ㄤ簬璁剧疆鏃犵嚎鐢甸鐜囥€俁F tuner锛堣嫢鏈夛級鐨?tuner 绱㈠紩蹇呴』濮嬬粓璺熼殢
SDR tuner 绱㈠紩銆傞€氬父 SDR tuner 涓?#0锛孯F tuner 涓?#1銆?
VIDIOC_S_HW_FREQ_SEEK ioctl 涓嶅彈鏀寔銆?
## 鏁版嵁鏍煎紡鍗忓晢


SDR 璁惧浣跨敤鏍煎紡 ioctls 鏉ラ€夋嫨鎹曡幏鍜岃緭鍑烘牸寮忋€傞噰鏍峰垎杈ㄧ巼鍜屾暟鎹祦寮忔牸寮忛兘
缁戝畾鍒拌鍙€夋嫨鐨勬牸寮忋€傞櫎鍩烘湰鐨勬牸寮?ioctls 澶栵紝杩樺繀椤绘敮鎸?VIDIOC_ENUM_FMT ioctl銆?
瑕佷娇鐢ㄦ牸寮?ioctls锛屽簲鐢ㄧ▼搴忓皢 struct `v4l2_format` 鐨?`type` 瀛楁璁句负
`V4L2_BUF_TYPE_SDR_CAPTURE` 鎴?`V4L2_BUF_TYPE_SDR_OUTPUT`锛屽苟鏍规嵁鎵€闇€
鎿嶄綔鎸夐渶浣跨敤 struct `v4l2_sdr_format` 鐨?`fmt` 鑱斿悎鐨?`sdr` 鎴愬憳銆?鐩墠浣跨敤浜?struct `v4l2_sdr_format` 鐨勪袱涓瓧娈碉細`pixelformat` 鍜?`buffersize`銆俙pixelformat` 鐨勫唴瀹规槸鏁版嵁鏍煎紡鐨?V4L2 FourCC 鐮併€俙buffersize`
瀛楁鏄暟鎹紶杈撴墍闇€鐨勬渶澶х紦鍐插尯瀛楄妭鏁帮紝鐢遍┍鍔ㄨ缃互鍛婄煡搴旂敤绋嬪簭銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pixelformat`
      - 鏁版嵁鏍煎紡鎴栧帇缂╃被鍨嬶紝鐢卞簲鐢ㄧ▼搴忚缃€傝繖鏄竴涓皬绔?	鍥涘瓧绗︾爜 <v4l2-fourcc>銆俈4L2 鍦?sdr-formats 涓畾涔変簡 SDR
	鏍煎紡銆?    - - __u32
      - `buffersize`
      - 鏁版嵁鎵€闇€鐨勬渶澶у瓧鑺傛暟銆傚€肩敱椹卞姩璁剧疆銆?    - - __u8
      - `reserved[^24^]`
      - 璇ユ暟缁勪负鏈潵鎵╁睍淇濈暀銆傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗗叾缃浂銆?

SDR 璁惧鍙兘鏀寔 read/write <rw> 鍜?鎴栨祦寮?锛堝唴瀛樻槧灏?<mmap> 鎴栫敤鎴锋寚閽?<userp>锛塈/O銆?