


######## ioctl VIDIOC_SUBDEV_G_ROUTING, VIDIOC_SUBDEV_S_ROUTING


## 鍚嶇О


VIDIOC_SUBDEV_G_ROUTING - VIDIOC_SUBDEV_S_ROUTING - 鑾峰彇鎴栬缃獟浣撳疄浣撲腑濯掍綋 pad 涔嬮棿鐨勬祦璺敱銆?
## 姒傝


`int ioctl(int fd, VIDIOC_SUBDEV_G_ROUTING, struct v4l2_subdev_routing *argp)`

`int ioctl(int fd, VIDIOC_SUBDEV_S_ROUTING, struct v4l2_subdev_routing *argp)`

## 鍙傛暟


`fd`
    鐢?open() <func-open> 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_routing` 鐨勬寚閽堛€?
## 鎻忚堪


杩欎簺 ioctl 鐢ㄤ簬鑾峰彇鍜岃缃獟浣撳疄浣撲腑鐨勮矾鐢便€?璺敱閰嶇疆鍐冲畾浜嗗疄浣撳唴閮ㄧ殑鏁版嵁娴併€?
椹卞姩浣跨敤 `VIDIOC_SUBDEV_G_ROUTING` ioctl 鎶ュ憡鍏跺綋鍓嶈矾鐢辫〃锛?鑰屽簲鐢ㄧ▼搴忓彲浠ラ€氳繃娣诲姞鎴栫Щ闄よ矾鐢便€佷互鍙婅缃垨娓呴櫎
struct `v4l2_subdev_route` 鐨?`flags` 瀛楁涓殑鏍囧織锛?浣跨敤 `VIDIOC_SUBDEV_S_ROUTING` ioctl 鏉ュ惎鐢ㄦ垨绂佺敤璺敱銆?涓?`VIDIOC_SUBDEV_G_ROUTING` 绫讳技锛宍VIDIOC_SUBDEV_S_ROUTING`
涔熶細灏嗚矾鐢辫繑鍥炵粰鐢ㄦ埛銆?
褰撹皟鐢?`VIDIOC_SUBDEV_S_ROUTING` 鏃讹紝鎵€鏈夋祦閰嶇疆閮戒細琚噸缃€?杩欐剰鍛崇潃鐢ㄦ埛绌洪棿蹇呴』鍦ㄨ皟鐢ㄨ ioctl 涔嬪悗锛屼緥濡備娇鐢?`VIDIOC_SUBDEV_S_FMT` 閲嶆柊閰嶇疆鎵€鏈夌殑娴佹牸寮忎笌閫夋嫨锛坰elections锛夈€?
鍙湁鍚屾椂鍏锋湁 sink 涓?source pad 鐨勫瓙璁惧鎵嶈兘鏀寔璺敱銆?
`len_routes` 瀛楁琛ㄧず鐢ㄦ埛绌洪棿鍒嗛厤鐨?`routes` 鏁扮粍涓?鑳藉瀹圭撼鐨勮矾鐢辨暟閲忋€傚畠鐢卞簲鐢ㄧ▼搴忎负涓や釜 ioctl 璁剧疆锛?浠ユ寚绀哄唴鏍稿彲浠ヨ繑鍥炲灏戞潯璺敱锛屽苟涓斿喅涓嶄細琚唴鏍镐慨鏀广€?
`num_routes` 瀛楁琛ㄧず璺敱琛ㄤ腑鐨勮矾鐢辨暟閲忋€?瀵逛簬 `VIDIOC_SUBDEV_S_ROUTING`锛屽畠鐢辩敤鎴风┖闂磋缃负
搴旂敤绋嬪簭瀛樺偍鍦?`routes` 鏁扮粍涓殑璺敱鏁伴噺銆傚浜庝袱涓?ioctl锛?瀹冮兘鐢卞唴鏍歌繑鍥烇紝骞舵寚绀哄瓙璁惧璺敱琛ㄤ腑瀛樺偍浜嗗灏戞潯璺敱銆?杩欏彲鑳藉皬浜庢垨澶т簬搴旂敤绋嬪簭涓?`VIDIOC_SUBDEV_S_ROUTING`
璁剧疆鐨?`num_routes` 鍊硷紝鍥犱负椹卞姩鍙兘浼氳皟鏁存墍璇锋眰鐨勮矾鐢辫〃銆?
鍐呮牳鍙互浠庝袱涓?ioctl 杩斿洖姣?`len_routes` 鏇村ぇ鐨?`num_routes` 鍊笺€?杩欒〃绀鸿矾鐢辫〃涓殑璺敱鏁伴噺澶氫簬 `routes` 鏁扮粍鎵€鑳藉绾崇殑銆?鍦ㄨ繖绉嶆儏鍐典笅锛屽唴鏍镐細鐢ㄥ瓙璁惧璺敱琛ㄧ殑鍓?`len_routes` 涓?鏉＄洰濉厖 `routes` 鏁扮粍銆傝繖涓嶈瑙嗕负閿欒锛宨octl 璋冪敤浼氭垚鍔熴€?濡傛灉搴旂敤绋嬪簭甯屾湜鍙栧洖缂哄け鐨勮矾鐢憋紝瀹冨彲浠ュ彂鍑轰竴涓柊鐨?`VIDIOC_SUBDEV_G_ROUTING` 璋冪敤锛屽苟鎻愪緵涓€涓冻澶熷ぇ鐨?`routes` 鏁扮粍銆?
`VIDIOC_SUBDEV_S_ROUTING` 鍙兘浼氳繑鍥炴瘮鐢ㄦ埛鍦?`num_routes`
瀛楁涓彁渚涚殑鏇村璺敱锛屼緥濡傜敱浜庣‖浠剁壒鎬с€?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `which`
      - 瑕佽闂殑璺敱琛紝鏉ヨ嚜 enum
        v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `len_routes`
      - 鏁扮粍鐨勯暱搴︼紙鍗虫暟缁勬墍鍗犵敤鐨勫唴瀛橈級銆?    - - struct `v4l2_subdev_route`
      - `routes[]`
      - struct `v4l2_subdev_route` 鏉＄洰缁勬垚鐨勬暟缁勩€?    - - __u32
      - `num_routes`
      - routes 鏁扮粍鐨勬潯鐩暟閲忋€?    - - __u32
      - `reserved`\ [^11^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忎笌椹卞姩蹇呴』灏嗚鏁扮粍
	璁剧疆涓洪浂銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `sink_pad`
      - Sink pad 缂栧彿銆?    - - __u32
      - `sink_stream`
      - Sink pad 娴佺紪鍙枫€?    - - __u32
      - `source_pad`
      - Source pad 缂栧彿銆?    - - __u32
      - `source_stream`
      - Source pad 娴佺紪鍙枫€?    - - __u32
      - `flags`
      - 璺敱鍚敤/绂佺敤鏍囧織
	v4l2_subdev_routing_flags <v4l2-subdev-routing-flags>銆?    - - __u32
      - `reserved`\ [^5^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忎笌椹卞姩蹇呴』灏嗚鏁扮粍
	璁剧疆涓洪浂銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - V4L2_SUBDEV_ROUTE_FL_ACTIVE
      - 0x0001
      - 璇ヨ矾鐢卞凡鍚敤銆傜敱搴旂敤绋嬪簭璁剧疆銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?閫氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 涓€绔犱腑鎻忚堪銆?
EINVAL
    sink 鎴?source pad 鏍囪瘑绗﹀紩鐢ㄤ簡涓嶅瓨鍦ㄧ殑 pad锛屾垨鑰呭紩鐢ㄤ簡
    涓嶅悓绫诲瀷鐨?pad锛堝嵆 sink_pad 鏍囪瘑绗﹀紩鐢ㄤ簡涓€涓?source pad锛夛紝
    `which` 瀛楁鐨勫€间笉鍙楁敮鎸侊紝鎴栬€呭浜?`VIDIOC_SUBDEV_S_ROUTING`锛?    搴旂敤绋嬪簭璁剧疆鐨?num_routes 瀛楁澶т簬 len_routes 瀛楁鐨勫€笺€?
ENXIO
    搴旂敤绋嬪簭璇锋眰鐨勮矾鐢辨棤娉曞垱寤猴紝鎴栬€呮寚瀹氳矾鐢辩殑鐘舵€?    鏃犳硶淇敼銆備粎閽堝 `VIDIOC_SUBDEV_S_ROUTING` 杩斿洖銆?
E2BIG
    搴旂敤绋嬪簭涓?`VIDIOC_SUBDEV_S_ROUTING` 鎻愪緵鐨?`num_routes`
    澶т簬椹卞姩鎵€鑳藉鐞嗙殑璺敱鏁伴噺銆?