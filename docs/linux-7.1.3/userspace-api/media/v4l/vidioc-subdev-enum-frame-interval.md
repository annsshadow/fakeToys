


######## ioctl VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL


## 鍚嶇О


VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL - 鏋氫妇甯ч棿闅?
## 姒傝


`int ioctl(int fd, VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL, struct v4l2_subdev_frame_interval_enum * argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_frame_interval_enum` 鐨勬寚閽堛€?
## 鎻忚堪


璇?ioctl 璁╁簲鐢ㄧ▼搴忔灇涓剧粰瀹氬瓙璁惧 pad 涓婂彲鐢ㄧ殑甯ч棿闅斻€傚抚闂撮殧浠呭鑳藉鑷鎺у埗甯у懆鏈熺殑瀛愯澶囨湁鎰忎箟銆傝繖鍖呮嫭锛屼緥濡傦紝鍥惧儚浼犳劅鍣ㄥ拰 TV 璋冭皭鍣ㄣ€?
瀵逛簬鍥惧儚浼犳劅鍣ㄨ繖涓€甯歌鐢ㄤ緥锛屽瓙璁惧杈撳嚭 pad 涓婂彲鐢ㄧ殑甯ч棿闅斿彇鍐充簬鍚屼竴 pad 涓婄殑甯ф牸寮忓拰灏哄銆傚洜姝わ紝搴旂敤绋嬪簭鍦ㄦ灇涓惧抚闂撮殧鏃跺繀椤绘寚瀹氭湡鏈涚殑鏍煎紡鍜屽昂瀵搞€?
涓烘灇涓惧抚闂撮殧锛屽簲鐢ㄧ▼搴忓垵濮嬪寲 struct `v4l2_subdev_frame_interval_enum` 鐨?`index`銆乣pad`銆乣which`銆乣code`銆乣width` 鍜?`height` 瀛楁锛屽苟浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL ioctl銆傚鏋滄煇涓緭鍏ュ瓧娈垫棤鏁堬紝椹卞姩濉厖缁撴瀯鐨勫叾浣欓儴鍒嗘垨杩斿洖 `EINVAL` 閿欒鐮併€傛墍鏈夊抚闂撮殧閮藉彲閫氳繃浠庣储寮曢浂寮€濮嬮€掑涓€锛岀洿鍒拌繑鍥?`EINVAL` 鏉ユ灇涓俱€?
鍙敤甯ч棿闅斿彲鑳藉彇鍐充簬瀛愯澶囧叾浠?pad 涓婄殑褰撳墠鈥渢ry鈥濇牸寮忥紝浠ュ強褰撳墠鐨勬椿璺冮摼鎺ャ€傚叧浜?try 鏍煎紡鐨勬洿澶氫俊鎭紝璇峰弬瑙?VIDIOC_SUBDEV_G_FMT銆?
鏀寔甯ч棿闅旀灇涓?ioctl 鐨勫瓙璁惧搴斾粎鍦ㄥ崟涓?pad 涓婂疄鐜板畠銆傚綋瀹冨湪鍚屼竴瀛愯澶囩殑澶氫釜 pad 涓婂彈鏀寔鏃讹紝鍏惰涓烘湭瀹氫箟銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 鏋氫妇涓殑鏍煎紡缂栧彿锛岀敱搴旂敤绋嬪簭璁剧疆銆?    - - __u32
      - `pad`
      - media controller API 鎶ュ憡鐨?pad 缂栧彿銆?    - - __u32
      - `code`
      - media 鎬荤嚎鏍煎紡浠ｇ爜锛屽畾涔変簬 v4l2-mbus-format銆?    - - __u32
      - `width`
      - 甯у锛屽崟浣嶄负鍍忕礌銆?    - - __u32
      - `height`
      - 甯ч珮锛屽崟浣嶄负鍍忕礌銆?    - - struct `v4l2_fract`
      - `interval`
      - 杩炵画瑙嗛甯т箣闂寸殑鍛ㄦ湡锛屽崟浣嶄负绉掋€?    - - __u32
      - `which`
      - 瑕佹灇涓剧殑甯ч棿闅旓紝鏉ヨ嚜鏋氫妇
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `stream`
      - 娴佹爣璇嗙銆?    - - __u32
      - `reserved`\ [^7^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忎笌椹卞姩蹇呴』灏嗚鏁扮粍缃浂銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    struct `v4l2_subdev_frame_interval_enum` 鐨?`pad` 寮曠敤浜嗕笉瀛樺湪鐨?pad锛宍which` 瀛楁鍙栧€间笉鍙楁敮鎸侊紝缁欏畾鐨?`code`銆乣width` 鎴?`height` 瀛楁瀵规寚瀹?pad 鏃犳晥锛屾垨鑰?`index` 瀛楁瓒婄晫銆?