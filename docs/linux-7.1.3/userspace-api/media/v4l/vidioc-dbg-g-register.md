######## ioctl VIDIOC_DBG_G_REGISTER, VIDIOC_DBG_S_REGISTER


## 鍚嶇О


VIDIOC_DBG_G_REGISTER - VIDIOC_DBG_S_REGISTER - 璇绘垨鍐欑‖浠跺瘎瀛樺櫒

## 姒傝



`int ioctl(int fd, VIDIOC_DBG_G_REGISTER, struct v4l2_dbg_register *argp)`


`int ioctl(int fd, VIDIOC_DBG_S_REGISTER, const struct v4l2_dbg_register *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜 struct `v4l2_dbg_register` 鐨勬寚閽堛€?

## 鎻忚堪



    杩欐槸涓€涓疄楠屾€ф帴鍙ｏ紝鏈潵鍙兘浼氬彂鐢熷彉鍖栥€?

鍑轰簬椹卞姩璋冭瘯鐩殑锛岃繖浜?ioctl 鍏佽娴嬭瘯搴旂敤绋嬪簭鐩存帴璁块棶纭欢瀵勫瓨鍣ㄣ€傛櫘閫氬簲鐢ㄧ▼搴忎笉寰椾娇鐢ㄥ畠浠€?

鐢变簬鍐欏叆鐢氳嚦璇诲彇瀵勫瓨鍣ㄩ兘鍙兘鍗卞強绯荤粺瀹夊叏銆佺ǔ瀹氭€у苟鎹熷潖纭欢锛屼袱涓?ioctl 閮介渶瑕佽秴绾х敤鎴锋潈闄愩€傛澶栵紝Linux 鍐呮牳蹇呴』缂栬瘧鏃跺惎鐢?`CONFIG_VIDEO_ADV_DEBUG` 閫夐」浠ュ惎鐢ㄨ繖浜?ioctl銆?

瑕佸啓鍏ヤ竴涓瘎瀛樺櫒锛屽簲鐢ㄧ▼搴忓繀椤诲垵濮嬪寲 struct `v4l2_dbg_register` 鐨勬墍鏈夊瓧娈碉紙`size` 闄ゅ锛夛紝骞朵娇鐢ㄦ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_DBG_S_REGISTER`銆俙match.type` 鍜?`match.addr` 鎴?`match.name` 瀛楁閫夋嫨 TV 鍗′笂鐨勪竴涓姱鐗囷紝`reg` 瀛楁鎸囧畾瀵勫瓨鍣ㄧ紪鍙凤紝`val` 瀛楁涓鸿鍐欏叆瀵勫瓨鍣ㄧ殑鍊笺€?

瑕佽鍙栦竴涓瘎瀛樺櫒锛屽簲鐢ㄧ▼搴忓繀椤诲垵濮嬪寲 `match.type`銆乣match.addr` 鎴?`match.name` 浠ュ強 `reg` 瀛楁锛屽苟浣跨敤鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 `VIDIOC_DBG_G_REGISTER`銆傛垚鍔熸椂锛岄┍鍔ㄥ皢瀵勫瓨鍣ㄥ€煎瓨鍌ㄥ湪 `val` 瀛楁涓紝骞跺皢璇ュ€肩殑澶у皬锛堜互瀛楄妭涓哄崟浣嶏級瀛樺偍鍦?`size` 涓€?

褰?`match.type` 涓?`V4L2_CHIP_MATCH_BRIDGE` 鏃讹紝`match.addr` 閫夋嫨 TV 鍗′笂鐨勭 n 涓潪瀛愯澶囪姱鐗囥€傛暟瀛楅浂濮嬬粓閫夋嫨涓昏姱鐗囷紝渚嬪杩炴帴鍒?PCI 鎴?USB 鎬荤嚎鐨勮姱鐗囥€備綘鍙互閫氳繃 VIDIOC_DBG_G_CHIP_INFO ioctl 浜嗚В瀛樺湪鍝簺鑺墖銆?

褰?`match.type` 涓?`V4L2_CHIP_MATCH_SUBDEV` 鏃讹紝`match.addr` 閫夋嫨绗?n 涓瓙璁惧銆?

杩欎簺 ioctl 鏄彲閫夌殑锛屽苟闈炴墍鏈夐┍鍔ㄩ兘鍙兘鏀寔瀹冧滑銆傜劧鑰岋紝褰撻┍鍔ㄦ敮鎸佽繖浜?ioctl 鏃讹紝瀹冧篃蹇呴』鏀寔 VIDIOC_DBG_G_CHIP_INFO銆傚弽涔嬶紝瀹冨彲鑳芥敮鎸?`VIDIOC_DBG_G_CHIP_INFO` 浣嗕笉鏀寔杩欎簺 ioctl銆?

`VIDIOC_DBG_G_REGISTER` 鍜?`VIDIOC_DBG_S_REGISTER` 鏄湪 Linux 2.6.21 涓紩鍏ョ殑锛屼絾鍏?API 鍦ㄥ唴鏍?2.6.29 涓鏇存敼涓烘澶勬墍鎻忚堪鐨勭増鏈€?

鎴戜滑寤鸿浣跨敤 v4l2-dbg 宸ュ叿锛岃€屼笉鏄洿鎺ヨ皟鐢ㄨ繖浜?ioctl銆傚畠鍙粠 LinuxTV v4l-dvb 浠撳簱鑾峰彇锛涜闂鏄庤 `https://linuxtv.org/repo/ <https://linuxtv.org/repo/>`__銆?




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 鏈夊叧鍙兘鐨勭被鍨嬪垪琛紝璇峰弬瑙?chip-match-types銆?
    - - union {
      - (anonymous)
    - - __u32
      - `addr`
      - 鎸夋缂栧彿鍖归厤鑺墖锛屼緷鎹?`type` 瀛楁瑙ｉ噴銆?
    - - char
      - `name[^32^]`
      - 鎸夋鍚嶇О鍖归厤鑺墖锛屼緷鎹?`type` 瀛楁瑙ｉ噴銆傚綋鍓嶆湭浣跨敤銆?
    - - }
      -

    :header-rows:  0
    :stub-columns: 0

    - - struct v4l2_dbg_match
      - `match`
      - 濡備綍鍖归厤鑺墖锛岃鍙傝 `v4l2_dbg_match`銆?
    - - __u32
      - `size`
      - 瀵勫瓨鍣ㄥぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€?
    - - __u64
      - `reg`
      - 瀵勫瓨鍣ㄧ紪鍙枫€?
    - - __u64
      - `val`
      - 浠庡瘎瀛樺櫒璇诲彇鎴栬鍐欏叆瀵勫瓨鍣ㄧ殑鍊笺€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CHIP_MATCH_BRIDGE`
      - 0
      - 鍖归厤鍗′笂鐨勭 n 涓姱鐗囷紝妗ユ帴鑺墖涓洪浂銆備笉鍖归厤瀛愯澶囥€?
    - - `V4L2_CHIP_MATCH_SUBDEV`
      - 4
      - 鍖归厤绗?n 涓瓙璁惧銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

EPERM
    鏉冮檺涓嶈冻銆傛墽琛岃繖浜?ioctl 闇€瑕?root 鏉冮檺銆?
