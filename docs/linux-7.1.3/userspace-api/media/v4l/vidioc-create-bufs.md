


######## ioctl VIDIOC_CREATE_BUFS


## 鍚嶇О


VIDIOC_CREATE_BUFS - 涓哄唴瀛樻槧灏勩€佺敤鎴锋寚閽堟垨 DMA 缂撳啿鍖?I/O 鍒涘缓缂撳啿鍖?
## 姒傝



`int ioctl(int fd, VIDIOC_CREATE_BUFS, struct v4l2_create_buffers *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_create_buffers` 鐨勬寚閽堛€?
## 鎻忚堪


璇?ioctl 鐢ㄤ簬涓哄唴瀛樻槧灏?<mmap>銆佺敤鎴锋寚閽?<userp> 鎴?DMA 缂撳啿鍖?<dmabuf>
I/O 鍒涘缓缂撳啿鍖恒€傚綋闇€瑕佸缂撳啿鍖鸿繘琛屾洿涓ユ牸鐨勬帶鍒舵椂锛屽畠鍙互浣滀负 VIDIOC_REQBUFS
ioctl 鐨勬浛浠ｆ垨琛ュ厖鏉ヤ娇鐢ㄣ€傝 ioctl 鍙互澶氭璋冪敤锛屼互鍒涘缓涓嶅悓澶у皬鐨勭紦鍐插尯銆?
涓轰簡鍒嗛厤璁惧缂撳啿鍖猴紝搴旂敤绋嬪簭蹇呴』鍒濆鍖?struct `v4l2_create_buffers` 缁撴瀯鐨?鐩稿叧瀛楁銆俙count` 瀛楁蹇呴』璁句负璇锋眰鐨勭紦鍐插尯鏁伴噺锛宍memory` 瀛楁鎸囧畾璇锋眰鐨?I/O
鏂规硶锛岃€?`reserved` 鏁扮粍蹇呴』娓呴浂銆?
`format` 瀛楁鎸囧畾缂撳啿鍖哄繀椤昏兘澶熷鐞嗙殑鍥惧儚鏍煎紡銆傚簲鐢ㄧ▼搴忓繀椤诲～鍐?struct
`v4l2_format`銆傞€氬父杩欎細閫氳繃 VIDIOC_TRY_FMT <VIDIOC_G_FMT> 鎴?VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl 鏉ュ畬鎴愶紝浠ョ‘淇濊姹傜殑鏍煎紡鍙楅┍鍔ㄦ敮鎸併€?鏍规嵁鏍煎紡鐨?`type` 瀛楁锛屽垎閰嶇紦鍐插尯鏃跺皢浣跨敤璇锋眰鐨勭紦鍐插尯澶у皬锛堝浜庡崟骞抽潰锛夋垨
骞抽潰澶у皬锛堝浜庡骞抽潰鏍煎紡锛夈€傚鏋滃ぇ灏忎笉鍙楃‖浠舵敮鎸侊紙閫氬父鏄洜涓哄お灏忥級锛岄┍鍔?鍙兘杩斿洖閿欒銆?
璇?ioctl 鍒涘缓鐨勭紦鍐插尯鐨勬渶灏忓ぇ灏忎负 `format.pix.sizeimage` 瀛楁锛堟垨鍏跺畠鏍煎紡绫诲瀷
鐨勫搴斿瓧娈碉級鎵€瀹氫箟鐨勫ぇ灏忋€傞€氬父锛屽鏋?`format.pix.sizeimage` 瀛楁灏忎簬缁欏畾鏍煎紡
鎵€闇€鐨勬渶灏忓€硷紝鍒欎細杩斿洖閿欒锛屽洜涓洪┍鍔ㄩ€氬父涓嶅厑璁歌繖鏍峰仛銆傚鏋滃畠鏇村ぇ锛屽垯璇ュ€煎皢
鍘熸牱浣跨敤銆傛崲鍙ヨ瘽璇达紝椹卞姩鍙兘鎷掔粷璇锋眰鐨勫ぇ灏忥紝浣嗗鏋滆鎺ュ彈锛岄┍鍔ㄥ皢涓嶅姞淇敼鍦?浣跨敤瀹冦€?
褰撲互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤璇?ioctl 鏃讹紝椹卞姩灏嗗皾璇曞垎閰嶅杈捐姹傛暟閲忕殑缂撳啿鍖猴紝骞?鍒嗗埆鎶婂疄闄呭垎閰嶇殑鏁伴噺鍜岃捣濮嬬储寮曞瓨鍏?`count` 鍜?`index` 瀛楁銆傝繑鍥炴椂 `count` 鍙兘
灏忎簬璇锋眰鐨勬暟閲忋€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 璧峰缂撳啿鍖虹储寮曪紝鐢遍┍鍔ㄨ繑鍥炪€?    - - __u32
      - `count`
      - 璇锋眰鎴栨巿浜堢殑缂撳啿鍖烘暟閲忋€傚鏋?count == 0锛屽垯 VIDIOC_CREATE_BUFS 浼氬皢
	`index` 璁句负褰撳墠宸插垱寤虹紦鍐插尯鐨勬暟閲忥紝骞舵鏌?`memory` 鍜?`format.type`
	鐨勬湁鏁堟€с€傚鏋滃畠浠棤鏁堝垯杩斿洖 -1 骞跺皢 errno 璁句负 `EINVAL` 閿欒鐮侊紝鍚﹀垯
	VIDIOC_CREATE_BUFS 杩斿洖 0銆傚湪杩欑鐗瑰畾鎯呭喌涓嬪畠缁濅笉浼氬皢 errno 璁句负
	`EBUSY` 閿欒鐮併€?    - - __u32
      - `memory`
      - 搴旂敤绋嬪簭灏嗚瀛楁璁句负 `V4L2_MEMORY_MMAP`銆乣V4L2_MEMORY_DMABUF` 鎴?	`V4L2_MEMORY_USERPTR`銆傚弬瑙?`v4l2_memory`
    - - struct `v4l2_format`
      - `format`
      - 鐢卞簲鐢ㄧ▼搴忓～鍐欙紝鐢遍┍鍔ㄤ繚鐣欍€?    - - __u32
      - `capabilities`
      - 鐢遍┍鍔ㄨ缃€傚鏋滀负 0锛岃〃绀洪┍鍔ㄤ笉鏀寔 capabilities銆傚湪杩欑鎯呭喌涓嬶紝浣犳墍鐭ラ亾鐨?	鍙槸椹卞姩淇濊瘉鏀寔 `V4L2_MEMORY_MMAP`锛屽苟涓?*鍙兘**鏀寔鍏跺畠 `v4l2_memory`
	绫诲瀷銆傚畠涓嶆敮鎸佷换浣曞叾瀹?capabilities銆傛湁鍏?capabilities 鍒楄〃锛岃鍙傝
	姝ゅ <v4l2-buf-capabilities>銆?
	濡傛灉浣犲彧鎯虫煡璇?capabilities 鑰屼笉鍋氫换浣曞叾瀹冩敼鍔紝鍒欏皢 `count` 璁句负 0锛?	`memory` 璁句负 `V4L2_MEMORY_MMAP`锛屽苟灏?`format.type` 璁句负缂撳啿鍖虹被鍨嬨€?
    - - __u32
      - `flags`
      - 鎸囧畾棰濆鐨勭紦鍐插尯绠＄悊灞炴€с€傚弬瑙?memory-flags銆?    - - __u32
      - `max_num_buffers`
      - 濡傛灉璁剧疆浜?V4L2_BUF_CAP_SUPPORTS_MAX_NUM_BUFFERS capability 鏍囧織锛屽垯璇?	瀛楁鎸囩ず姝ら槦鍒楀彲鑳界殑鏈€澶х紦鍐插尯鏁伴噺銆?    - - __u32
      - `reserved`\ [^5^]
      - 涓哄皢鏉ユ墿灞曚繚鐣欑殑鍗犱綅绗︺€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃负闆躲€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
ENOMEM
    娌℃湁鍐呭瓨涓哄唴瀛樻槧灏?<mmap> I/O 鍒嗛厤缂撳啿鍖恒€?
EINVAL
    缂撳啿鍖虹被鍨嬶紙`format.type` 瀛楁锛夈€佽姹傜殑 I/O 鏂规硶锛坄memory`锛夋垨鏍煎紡
    锛坄format` 瀛楁锛夋棤鏁堛€?