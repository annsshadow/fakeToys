
######## ioctl VIDIOC_SUBDEV_ENUM_FRAME_SIZE


## 鍚嶇О


VIDIOC_SUBDEV_ENUM_FRAME_SIZE - 鏋氫妇濯掍綋鎬荤嚎甯у昂瀵?
## 璇硶


`int ioctl(int fd, VIDIOC_SUBDEV_ENUM_FRAME_SIZE, struct v4l2_subdev_frame_size_enum * argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_frame_size_enum` 鐨勬寚閽堛€?
## 鎻忚堪


璇?ioctl 鍏佽搴旂敤绋嬪簭璁块棶瀛愯澶囦负鎸囧畾 pad銆佹寚瀹氬獟浣撴€荤嚎鏍煎紡鎵€鏀寔鐨勫抚灏哄鏋氫妇銆?
鏀寔鐨勬牸寮忓彲閫氳繃 VIDIOC_SUBDEV_ENUM_MBUS_CODE ioctl 鑾峰彇銆?
鏋氫妇鐢遍┍鍔ㄥ畾涔夛紝骞朵娇鐢?struct `v4l2_subdev_frame_size_enum` 鐨?`index` 瀛楁杩涜绱㈠紩銆?姣忎竴瀵?`pad` 涓?`code` 瀵瑰簲涓€涓嫭绔嬬殑鏋氫妇銆傛瘡涓灇涓句粠 `index` 涓?0 寮€濮嬶紝鏈€灏忕殑
鏃犳晥 index 鏍囧織鐫€鏋氫妇鐨勭粨鏉熴€?
鍥犳锛岃鏋氫妇鎸囧畾 pad 涓娿€佷娇鐢ㄦ寚瀹?mbus 鏍煎紡鎵€鍏佽鐨勫抚灏哄锛岄渶灏?`pad`銆乣which` 涓?`code` 瀛楁鍒濆鍖栦负鏈熸湜鍊硷紝骞跺皢 `index` 缃负 0銆傜劧鍚庝互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤
VIDIOC_SUBDEV_ENUM_FRAME_SIZE ioctl銆?
鎴愬姛鐨勮皟鐢ㄤ細杩斿洖濉厖濂界殑鏈€灏忎笌鏈€澶у抚灏哄銆傞€掑 `index` 閲嶅璋冪敤锛岀洿鍒版敹鍒?`EINVAL`銆?`EINVAL` 琛ㄧず鏋氫妇涓凡鏃犳洿澶氭潯鐩紝鎴栨煇涓緭鍏ュ弬鏁版棤鏁堛€?
鍙敮鎸佺鏁ｅ抚灏哄鐨勫瓙璁惧锛堜緥濡傚ぇ澶氭暟浼犳劅鍣級浼氳繑鍥炰竴涓垨澶氫釜鏈€灏忎笌鏈€澶у€肩浉鍚岀殑甯у昂瀵搞€?
鍦ㄧ粰瀹?[minimum, maximum] 鑼冨洿鍐呭苟闈炴墍鏈夊彲鑳界殑灏哄閮藉彈鏀寔銆備緥濡傦紝浣跨敤瀹氱偣缂╂斁姣斾緥鐨?缂╂斁鍣ㄥ彲鑳芥棤娉曠敓鎴愭渶灏忎笌鏈€澶у€间箣闂寸殑姣忎竴涓抚灏哄銆傚簲鐢ㄧ▼搴忓繀椤讳娇鐢?VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 鏉ュ悜瀛愯澶囪姹備竴涓‘鍒囧彈鏀寔鐨?甯у昂瀵搞€?
鍙敤鐨勫抚灏哄鍙兘鍙栧喅浜庡瓙璁惧鍏朵粬 pad 涓婂綋鍓嶇殑 'try' 鏍煎紡銆佸綋鍓嶇殑娲昏穬閾捐矾浠ュ強褰撳墠
V4L2 鎺т欢鐨勫€笺€傚叧浜?try 鏍煎紡鐨勬洿澶氫俊鎭紝璇峰弬瑙?VIDIOC_SUBDEV_G_FMT銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 鏋氫妇涓睘浜庣粰瀹?pad 涓庢牸寮忕殑甯у昂瀵哥储寮曘€傜敱搴旂敤绋嬪簭濉厖銆?    - - __u32
      - `pad`
      - 鐢卞獟浣撴帶鍒跺櫒 API 鎶ュ憡鐨?pad 缂栧彿銆傜敱搴旂敤绋嬪簭濉厖銆?    - - __u32
      - `code`
      - 濯掍綋鎬荤嚎鏍煎紡鐮侊紝瀹氫箟浜?v4l2-mbus-format銆傜敱搴旂敤绋嬪簭濉厖銆?    - - __u32
      - `min_width`
      - 鏈€灏忓抚瀹斤紝鍗曚綅鍍忕礌銆傜敱椹卞姩濉厖銆?    - - __u32
      - `max_width`
      - 鏈€澶у抚瀹斤紝鍗曚綅鍍忕礌銆傜敱椹卞姩濉厖銆?    - - __u32
      - `min_height`
      - 鏈€灏忓抚楂橈紝鍗曚綅鍍忕礌銆傜敱椹卞姩濉厖銆?    - - __u32
      - `max_height`
      - 鏈€澶у抚楂橈紝鍗曚綅鍍忕礌銆傜敱椹卞姩濉厖銆?    - - __u32
      - `which`
      - 瑕佹灇涓剧殑甯у昂瀵革紝鏉ヨ嚜鏋氫妇 v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `stream`
      - 娴佹爣璇嗙銆?    - - __u32
      - `reserved`\ [^7^]
      - 涓哄皢鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忎笌椹卞姩閮藉繀椤诲皢鏁扮粍缃浂銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    struct `v4l2_subdev_frame_size_enum` 鐨?`pad` 寮曠敤浜嗕竴涓笉瀛樺湪鐨?pad锛宍which`
    瀛楁鐨勫€间笉鍙楁敮鎸侊紝`code` 瀵圭粰瀹?pad 鏃犳晥锛屾垨 `index` 瀛楁瓒婄晫銆?