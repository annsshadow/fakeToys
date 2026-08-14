


######## ioctl VIDIOC_SUBDEV_G_FRAME_INTERVAL銆乂IDIOC_SUBDEV_S_FRAME_INTERVAL


## 鍚嶇О


VIDIOC_SUBDEV_G_FRAME_INTERVAL - VIDIOC_SUBDEV_S_FRAME_INTERVAL - 鑾峰彇鎴栬缃瓙璁惧 pad 涓婄殑甯ч棿闅?
## 姒傝


`int ioctl(int fd, VIDIOC_SUBDEV_G_FRAME_INTERVAL, struct v4l2_subdev_frame_interval *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_FRAME_INTERVAL, struct v4l2_subdev_frame_interval *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_frame_interval` 鐨勬寚閽堛€?
## 鎻忚堪


杩欎簺 ioctl 鐢ㄤ簬鑾峰彇鍜岃缃浘鍍忔祦姘寸嚎涓壒瀹氬瓙璁惧 pad 涓婄殑甯ч棿闅斻€傚抚闂撮殧浠呭鑳藉鑷鎺у埗甯у懆鏈熺殑瀛愯澶囨墠鏈夋剰涔夈€傝繖鍖呮嫭锛屼緥濡傦紝鍥惧儚浼犳劅鍣ㄥ拰 TV 璋冭皭鍣ㄣ€備笉鏀寔甯ч棿闅旂殑瀛愯澶囦笉寰楀疄鐜拌繖浜?ioctl銆?
搴旂敤绋嬪簭涓鸿幏鍙栧綋鍓嶅抚闂撮殧锛岄渶灏?struct `v4l2_subdev_frame_interval` 鐨?`pad` 瀛楁璁句负 media controller API 鎶ュ憡鐨勬湡鏈?pad 缂栧彿銆傚綋瀹冧滑浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_SUBDEV_G_FRAME_INTERVAL` ioctl 鏃讹紝椹卞姩浼氬～鍏?`interval` 瀛楁鐨勫悇鎴愬憳銆?
涓烘敼鍙樺綋鍓嶅抚闂撮殧锛屽簲鐢ㄧ▼搴忛渶鍚屾椂璁剧疆 `pad` 瀛楁鍜?`interval` 瀛楁鐨勫叏閮ㄦ垚鍛樸€傚綋瀹冧滑浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_SUBDEV_S_FRAME_INTERVAL` ioctl 鏃讹紝椹卞姩浼氭牎楠屾墍璇锋眰鐨勯棿闅旓紝骞舵牴鎹‖浠惰兘鍔涘鍏惰繘琛岃皟鏁村悗閰嶇疆璁惧銆傝繑鍥炴椂锛宻truct `v4l2_subdev_frame_interval` 鍖呭惈褰撳墠甯ч棿闅旓紝绛変环浜?`VIDIOC_SUBDEV_G_FRAME_INTERVAL` 璋冪敤鎵€杩斿洖鐨勫€笺€?
濡傛灉瀛愯澶囪妭鐐逛互鍙妯″紡娉ㄥ唽锛屽垯瀵?`VIDIOC_SUBDEV_S_FRAME_INTERVAL` 鐨勮皟鐢ㄤ粎鍦?`which` 瀛楁璁句负 `V4L2_SUBDEV_FORMAT_TRY` 鏃舵湁鏁堬紝鍚﹀垯杩斿洖閿欒锛屼笖 errno 鍙橀噺琚涓?`-EPERM`銆?
椹卞姩缁濅笉鍙粎浠呭洜涓鸿姹傜殑闂撮殧涓庤澶囪兘鍔涗笉鍖归厤灏辫繑鍥為敊璇€傚畠浠繀椤绘敼涓哄闂撮殧杩涜淇敼浠ュ尮閰嶇‖浠舵墍鑳芥彁渚涚殑鍊笺€備慨鏀瑰悗鐨勯棿闅斿簲灏藉彲鑳芥帴杩戝師濮嬭姹傘€?
鏀瑰彉甯ч棿闅旂粷涓嶅彲鏀瑰彉鏍煎紡銆傚彟涓€鏂归潰锛屾敼鍙樻牸寮忓彲鑳戒細鏀瑰彉甯ч棿闅斻€?
鏀寔甯ч棿闅?ioctl 鐨勫瓙璁惧搴斾粎鍦ㄥ崟涓?pad 涓婂疄鐜板畠浠€傚綋鍚屼竴瀛愯澶囩殑澶氫釜 pad 閮芥敮鎸佹椂锛屽叾琛屼负鏈畾涔夈€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - media controller API 鎶ュ憡鐨?pad 缂栧彿銆?    - - struct `v4l2_fract`
      - `interval`
      - 杩炵画瑙嗛甯т箣闂寸殑鍛ㄦ湡锛屽崟浣嶄负绉掋€?    - - __u32
      - `stream`
      - 娴佹爣璇嗙銆?    - - __u32
      - `which`
      - 娲诲姩鎴栧皾璇曠殑甯ч棿闅旓紝鏉ヨ嚜鏋氫妇
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `reserved`\ [^7^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忎笌椹卞姩蹇呴』灏嗚鏁扮粍缃浂銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EBUSY
    甯ч棿闅旀棤娉曟敼鍙橈紝鍥犱负璇?pad 褰撳墠姝ｅ繖銆備緥濡傚彲鑳芥槸璇?pad 涓婃湁娲昏穬鐨?video 娴併€傚湪棣栧厛鎵ц鍏朵粬鎿嶄綔瑙ｅ喅闂涔嬪墠锛屼笉寰楅噸璇曡 ioctl銆備粎鐢?`VIDIOC_SUBDEV_S_FRAME_INTERVAL` 杩斿洖銆?
EINVAL
    struct `v4l2_subdev_frame_interval` 鐨?`pad` 寮曠敤浜嗕笉瀛樺湪鐨?pad锛宍which` 瀛楁鍙栧€间笉鍙楁敮鎸侊紝鎴栬€呰 pad 涓嶆敮鎸佸抚闂撮殧銆?
EPERM
    `VIDIOC_SUBDEV_S_FRAME_INTERVAL` ioctl 鍦ㄤ互鍙妯″紡杩愯鐨勫瓙璁惧涓婅璋冪敤锛屼笖 `which` 瀛楁琚涓?`V4L2_SUBDEV_FORMAT_ACTIVE`銆?