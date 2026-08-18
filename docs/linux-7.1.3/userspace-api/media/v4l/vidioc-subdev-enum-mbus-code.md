


######## ioctl VIDIOC_SUBDEV_ENUM_MBUS_CODE


## 鍚嶇О


VIDIOC_SUBDEV_ENUM_MBUS_CODE - 鏋氫妇濯掍綋鎬荤嚎鏍煎紡

## 姒傝



`int ioctl(int fd, VIDIOC_SUBDEV_ENUM_MBUS_CODE, struct v4l2_subdev_mbus_code_enum * argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜缁撴瀯浣?`v4l2_subdev_mbus_code_enum` 鐨勬寚閽堛€?
## 鎻忚堪


搴旂敤绋嬪簭浣跨敤姝よ皟鐢ㄨ闂墍閫?pad 鐨勫獟浣撴€荤嚎鏍煎紡鏋氫妇銆?
鏋氫妇鐢遍┍鍔ㄥ畾涔夛紝骞朵娇鐢ㄧ粨鏋勪綋 `v4l2_subdev_mbus_code_enum` 鐨?`index` 瀛楁杩涜绱㈠紩銆?姣忔鏋氫妇浠?`index` 涓?0 寮€濮嬶紝鏈€浣庣殑闈炴湁鏁堢储寮曟爣璁版灇涓剧殑缁撴潫銆?
鍥犳锛岃鏋氫妇鏌愪釜缁欏畾瀛愯澶?pad 涓婂彲鐢ㄧ殑濯掍綋鎬荤嚎鏍煎紡锛岃灏?`pad` 鍜?`which` 瀛楁
鍒濆鍖栦负鏈熸湜鍊硷紝骞跺皢 `index` 璁句负 0銆傜劧鍚庝互鎸囧悜璇ョ粨鏋勪綋鐨勬寚閽堣皟鐢?VIDIOC_SUBDEV_ENUM_MBUS_CODE ioctl銆?
鎴愬姛鐨勮皟鐢ㄥ皢杩斿洖濉厖濂界殑 `code` 瀛楁锛屽叾涓寘鍚竴涓?mbus 浠ｇ爜鍊笺€傞€掑 `index` 閲嶅
璋冪敤锛岀洿鍒版敹鍒?`EINVAL`銆俙EINVAL` 琛ㄧず `pad` 鏃犳晥锛屾垨鑰呰 pad 涓婂凡娌℃湁鏇村浠ｇ爜鍙敤銆?
椹卞姩涓嶅緱涓哄悓涓€ pad 涓婁笉鍚岀殑绱㈠紩杩斿洖鐩稿悓鐨?`code` 鍊笺€?
鍙敤鐨勫獟浣撴€荤嚎鏍煎紡鍙兘鍙栧喅浜庡瓙璁惧鍏朵粬 pad 涓婂綋鍓嶇殑 'try' 鏍煎紡锛屼互鍙婂綋鍓嶇殑娲昏穬
閾炬帴銆傛湁鍏?try 鏍煎紡鐨勬洿澶氫俊鎭紝璇峰弬闃?VIDIOC_SUBDEV_G_FMT銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - Pad 缂栧彿锛岀敱濯掍綋鎺у埗鍣?API 鎶ュ憡銆傜敱搴旂敤绋嬪簭濉啓銆?    - - __u32
      - `index`
      - 灞炰簬缁欏畾 pad 鐨勬灇涓句腑鐨?mbus 浠ｇ爜绱㈠紩銆傜敱搴旂敤绋嬪簭濉啓銆?    - - __u32
      - `code`
      - 濯掍綋鎬荤嚎鏍煎紡浠ｇ爜锛屽畾涔変簬 v4l2-mbus-format銆傜敱椹卞姩濉啓銆?    - - __u32
      - `which`
      - 瑕佹灇涓剧殑濯掍綋鎬荤嚎鏍煎紡浠ｇ爜锛屾潵鑷?enum
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `flags`
      - 鍙傝 v4l2-subdev-mbus-code-flags
    - - __u32
      - `stream`
      - 娴佹爣璇嗙銆?    - - __u32
      - `reserved`\ [^6^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忓拰椹卞姩蹇呴』灏嗚鏁扮粍缃浂銆?



   \footnotesize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - V4L2_SUBDEV_MBUS_CODE_CSC_COLORSPACE
      - 0x00000001
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鐨?colorspace 缂栫爜銆傚簲鐢ㄧ▼搴忓彲浠ュ湪璋冪敤
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 骞惰缃?	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 鏃讹紝璇锋眰閰嶇疆瀛愯澶囩殑
	colorspace銆傚叧浜庡浣曟搷浣滐紝璇峰弬闃?v4l2-mbus-format銆?    - - V4L2_SUBDEV_MBUS_CODE_CSC_XFER_FUNC
      - 0x00000002
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鐨勮浆鎹㈠嚱鏁般€傚簲鐢ㄧ▼搴忓彲浠ュ湪璋冪敤
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 骞惰缃?	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 鏃讹紝璇锋眰閰嶇疆瀛愯澶囩殑
	杞崲鍑芥暟銆傚叧浜庡浣曟搷浣滐紝璇峰弬闃?v4l2-mbus-format銆?    - - V4L2_SUBDEV_MBUS_CODE_CSC_YCBCR_ENC
      - 0x00000004
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鐨?Y'CbCr 缂栫爜銆傚簲鐢ㄧ▼搴忓彲浠ュ湪璋冪敤
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 骞惰缃?	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 鏃讹紝璇锋眰閰嶇疆瀛愯澶囩殑
	Y'CbCr 缂栫爜銆傚叧浜庡浣曟搷浣滐紝璇峰弬闃?v4l2-mbus-format銆?    - - V4L2_SUBDEV_MBUS_CODE_CSC_HSV_ENC
      - 0x00000004
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鐨?HSV 缂栫爜銆傚簲鐢ㄧ▼搴忓彲浠ュ湪璋冪敤
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 骞惰缃?	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 鏃讹紝璇锋眰閰嶇疆瀛愯澶囩殑
	HSV 缂栫爜銆傚叧浜庡浣曟搷浣滐紝璇峰弬闃?v4l2-mbus-format銆?    - - V4L2_SUBDEV_MBUS_CODE_CSC_QUANTIZATION
      - 0x00000008
      - 椹卞姩鍏佽搴旂敤绋嬪簭灏濊瘯鏇存敼榛樿鐨勯噺鍖栥€傚簲鐢ㄧ▼搴忓彲浠ュ湪璋冪敤
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 骞惰缃?	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 鏃讹紝璇锋眰閰嶇疆瀛愯澶囩殑
	閲忓寲銆傚叧浜庡浣曟搷浣滐紝璇峰弬闃?v4l2-mbus-format銆?

   \normalsize

## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    缁撴瀯浣?`v4l2_subdev_mbus_code_enum` 鐨?`pad` 寮曠敤浜嗕竴涓笉瀛樺湪鐨?pad锛?    `which` 瀛楁鍚湁涓嶆敮鎸佺殑鍊硷紝鎴栬€?`index` 瀛楁瓒婄晫銆?