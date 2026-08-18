######## ioctl VIDIOC_SUBDEV_G_SELECTION, VIDIOC_SUBDEV_S_SELECTION


## 鍚嶇О


VIDIOC_SUBDEV_G_SELECTION - VIDIOC_SUBDEV_S_SELECTION - 鑾峰彇鎴栬缃瓙璁惧 pad 涓婄殑閫夋嫨鐭╁舰

## 姒傝


`int ioctl(int fd, VIDIOC_SUBDEV_G_SELECTION, struct v4l2_subdev_selection *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_SELECTION, struct v4l2_subdev_selection *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_selection` 鐨勬寚閽堛€?
## 鎻忚堪


閫夋嫨鐭╁舰鐢ㄤ簬閰嶇疆瀛愯澶囨墽琛岀殑銆佸奖鍝嶅浘鍍忓昂瀵哥殑鍚勭鍥惧儚澶勭悊鍔熻兘銆?鐩墠杩欏寘鎷鍓€佺缉鏀句笌鍚堟垚銆?
閫夋嫨 API 鍙栦唬浜嗘棫鐨勫瓙璁惧瑁佸壀 API <VIDIOC_SUBDEV_G_CROP>銆傝鍓?API
鐨勬墍鏈夊姛鑳戒互鍙婃洿澶氬姛鑳介兘鐢遍€夋嫨 API 鏀寔銆?
鏈夊叧姣忎釜閫夋嫨鐩爣濡備綍褰卞搷瀛愯澶囧唴閮ㄧ殑鍥惧儚澶勭悊娴佹按绾匡紝璇峰弬闃呭瓙璁惧鐩稿叧鏂囨。銆?
濡傛灉瀛愯澶囪妭鐐规槸浠ュ彧璇绘ā寮忔敞鍐岀殑锛岄偅涔堝 `VIDIOC_SUBDEV_S_SELECTION`
鐨勮皟鐢ㄤ粎鍦?`which` 瀛楁琚涓?`V4L2_SUBDEV_FORMAT_TRY` 鏃舵墠鏈夋晥锛?鍚﹀垯灏嗚繑鍥為敊璇紝骞跺皢 errno 鍙橀噺璁句负 `-EPERM`銆?
### 閫夋嫨鐩爣鐨勭被鍨?

閫夋嫨鐩爣鏈変袱绉嶇被鍨嬶細瀹為檯鐩爣锛坅ctual锛変笌杈圭晫锛坆ounds锛夈€傚疄闄呯洰鏍?鏄敤浜庨厤缃‖浠剁殑鐩爣銆侭OUNDS 鐩爣灏嗚繑鍥炰竴涓寘鍚簡鎵€鏈夊彲鑳界殑瀹為檯
鐭╁舰鐨勭煩褰€?
### 鍙戠幇鍙楁敮鎸佺殑鐗规€?

瑕佸彂鐜板摢浜涚洰鏍囧彈鏀寔锛岀敤鎴峰彲浠ュ杩欎簺鐩爣鎵ц
`VIDIOC_SUBDEV_G_SELECTION`銆備换浣曚笉鍙楁敮鎸佺殑鐩爣閮戒細杩斿洖 `EINVAL`銆?
閫夋嫨鐩爣涓庢爣蹇楀湪 v4l2-selections-common 涓湁鏂囨。璇存槑銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `which`
      - 娲诲姩鎴栧皾璇曪紙try锛夐€夋嫨锛屾潵鑷灇涓?	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `pad`
      - 鐢卞獟浣撴鏋舵姤鍛婄殑 pad 缂栧彿銆?    - - __u32
      - `target`
      - 鐩爣閫夋嫨鐭╁舰銆傚弬瑙?v4l2-selections-common銆?    - - __u32
      - `flags`
      - 鏍囧織銆傚弬瑙?v4l2-selection-flags銆?    - - struct `v4l2_rect`
      - `r`
      - 閫夋嫨鐭╁舰锛屼互鍍忕礌涓哄崟浣嶃€?    - - __u32
      - `stream`
      - 娴佹爣璇嗙銆?    - - __u32
      - `reserved`\ [^7^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忎笌椹卞姩蹇呴』灏嗚鏁扮粍缃浂銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EBUSY
    閫夋嫨鐭╁舰鏃犳硶鏇存敼锛屽洜涓鸿 pad 褰撳墠澶勪簬蹇欑姸鎬併€備緥濡傦紝杩欏彲鑳芥槸鐢变簬
    pad 涓婂瓨鍦ㄤ竴涓椿鍔ㄧ殑瑙嗛娴併€傚湪棣栧厛鎵ц鍏朵粬鎿嶄綔鏉ヤ慨澶嶈闂涔嬪墠锛?    涓嶅緱閲嶈瘯璇?ioctl銆備粎鐢?`VIDIOC_SUBDEV_S_SELECTION` 杩斿洖銆?
EINVAL
    struct `v4l2_subdev_selection` 鐨?`pad` 寮曠敤浜嗕竴涓笉瀛樺湪鐨?pad锛?    `which` 瀛楁鍙栦簡涓嶅彈鏀寔鐨勫€硷紝鎴栬€呯粰瀹氱殑瀛愯澶?pad 涓嶆敮鎸佽閫夋嫨鐩爣銆?
EPERM
    `VIDIOC_SUBDEV_S_SELECTION` ioctl 鍦ㄥ彧璇诲瓙璁惧涓婅璋冪敤锛屼笖 `which`
    瀛楁琚涓?`V4L2_SUBDEV_FORMAT_ACTIVE`銆?