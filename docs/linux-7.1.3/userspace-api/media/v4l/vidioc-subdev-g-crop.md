


######## ioctl VIDIOC_SUBDEV_G_CROP銆乂IDIOC_SUBDEV_S_CROP


## 鍚嶇О


VIDIOC_SUBDEV_G_CROP - VIDIOC_SUBDEV_S_CROP - 鑾峰彇鎴栬缃瓙璁惧 pad 涓婄殑瑁佸壀鐭╁舰

## 姒傝


`int ioctl(int fd, VIDIOC_SUBDEV_G_CROP, struct v4l2_subdev_crop *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_CROP, const struct v4l2_subdev_crop *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_crop` 鐨勬寚閽堛€?
## 鎻忚堪



    杩欐槸涓€涓凡搴熷純鐨勬帴鍙ｏ紝鏈潵鍙兘浼氳绉婚櫎銆傚畠宸茶閫夋嫨锛坰election锛堿PI <VIDIOC_SUBDEV_G_SELECTION> 鍙栦唬銆備笉鍐嶆帴鍙楀 `v4l2_subdev_crop` 缁撴瀯鐨勪换浣曟柊鎵╁睍銆?
涓鸿幏鍙栧綋鍓嶈鍓煩褰紝搴旂敤绋嬪簭灏?struct `v4l2_subdev_crop` 鐨?`pad` 瀛楁璁句负 media API 鎶ュ憡鐨勬湡鏈?pad 缂栧彿锛屽苟灏?`which` 瀛楁璁句负 `V4L2_SUBDEV_FORMAT_ACTIVE`銆傜劧鍚庡畠浠互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 `VIDIOC_SUBDEV_G_CROP` ioctl銆傚鏋滆緭鍏ュ弬鏁版棤鏁堬紝鎴栬€呯粰瀹?pad 涓嶆敮鎸佽鍓紝椹卞姩浼氬～鍏?`rect` 瀛楁鐨勬垚鍛樻垨杩斿洖 `EINVAL` 閿欒鐮併€?
涓烘敼鍙樺綋鍓嶈鍓煩褰紝搴旂敤绋嬪簭闇€鍚屾椂璁剧疆 `pad` 鍜?`which` 瀛楁浠ュ強 `rect` 瀛楁鐨勬墍鏈夋垚鍛樸€傜劧鍚庡畠浠互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 `VIDIOC_SUBDEV_S_CROP` ioctl銆傞┍鍔ㄤ細鏍￠獙鎵€璇锋眰鐨勮鍓煩褰紝鏍规嵁纭欢鑳藉姏瀵瑰叾杩涜璋冩暣骞堕厤缃澶囥€傝繑鍥炴椂锛宻truct `v4l2_subdev_crop` 鍖呭惈褰撳墠鏍煎紡锛岀瓑浠蜂簬 `VIDIOC_SUBDEV_G_CROP` 璋冪敤鎵€杩斿洖鐨勫€笺€?
搴旂敤绋嬪簭鍙互閫氳繃灏?`which` 璁句负 `V4L2_SUBDEV_FORMAT_TRY` 鏉ユ煡璇㈣澶囪兘鍔涖€傚綋璁剧疆鏃讹紝鈥渢ry鈥濊鍓煩褰笉浼氳椹卞姩搴旂敤鍒拌澶囷紝鑰屾槸鍍忔椿鍔ㄨ鍓煩褰竴鏍疯澶勭悊骞跺瓨鍌ㄥ湪瀛愯澶囨枃浠跺彞鏌勪腑銆傚洜姝ゆ煡璇㈠悓涓€瀛愯澶囩殑涓や釜搴旂敤绋嬪簭涓嶄細鐩镐簰骞叉壈銆?
濡傛灉瀛愯澶囪妭鐐逛互鍙妯″紡娉ㄥ唽锛屽垯瀵?`VIDIOC_SUBDEV_S_CROP` 鐨勮皟鐢ㄤ粎鍦?`which` 瀛楁璁句负 `V4L2_SUBDEV_FORMAT_TRY` 鏃舵湁鏁堬紝鍚﹀垯杩斿洖閿欒骞跺皢 errno 鍙橀噺璁句负 `-EPERM`銆?
椹卞姩缁濅笉鍙粎浠呭洜涓烘墍璇锋眰鐨勮鍓煩褰笌璁惧鑳藉姏涓嶅尮閰嶅氨杩斿洖閿欒銆傚畠浠繀椤绘敼涓轰慨鏀硅鐭╁舰浠ュ尮閰嶇‖浠舵墍鑳芥彁渚涚殑鍊笺€備慨鏀瑰悗鐨勬牸寮忓簲灏藉彲鑳芥帴杩戝師濮嬭姹傘€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - media 妗嗘灦鎶ュ憡鐨?pad 缂栧彿銆?    - - __u32
      - `which`
      - 瑕佽幏鍙栨垨璁剧疆鐨勮鍓煩褰紝鏉ヨ嚜鏋氫妇
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - struct `v4l2_rect`
      - `rect`
      - 瑁佸壀鐭╁舰鐨勮竟鐣岋紝鍗曚綅涓哄儚绱犮€?    - - __u32
      - `stream`
      - 娴佹爣璇嗙銆?    - - __u32
      - `reserved`\ [^7^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忎笌椹卞姩蹇呴』灏嗚鏁扮粍缃浂銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EBUSY
    瑁佸壀鐭╁舰鏃犳硶鏀瑰彉锛屽洜涓鸿 pad 褰撳墠姝ｅ繖銆備緥濡傚彲鑳芥槸璇?pad 涓婃湁娲昏穬鐨?video 娴併€傚湪棣栧厛鎵ц鍏朵粬鎿嶄綔瑙ｅ喅闂涔嬪墠锛屼笉寰楅噸璇曡 ioctl銆備粎鐢?`VIDIOC_SUBDEV_S_CROP` 杩斿洖銆?
EINVAL
    struct `v4l2_subdev_crop` 鐨?`pad` 寮曠敤浜嗕笉瀛樺湪鐨?pad锛宍which` 瀛楁鍙栧€间笉鍙楁敮鎸侊紝鎴栬€呯粰瀹氬瓙璁惧 pad 涓嶆敮鎸佽鍓€?
EPERM
    `VIDIOC_SUBDEV_S_CROP` ioctl 鍦ㄤ互鍙妯″紡杩愯鐨勫瓙璁惧涓婅璋冪敤锛屼笖 `which` 瀛楁琚涓?`V4L2_SUBDEV_FORMAT_ACTIVE`銆?