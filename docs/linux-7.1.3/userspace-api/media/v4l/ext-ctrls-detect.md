


######## 妫€娴嬫帶鍒跺弬鑰冿紙Detect Control Reference锛?

Detect 绫诲寘鍚敤浜庡悇绉嶅叿澶囪繍鍔ㄦ垨鐗╀綋妫€娴嬭兘鍔涚殑璁惧鐨勯€氱敤鐗规€ф帶鍒躲€?


## 妫€娴嬫帶鍒?IDs


`V4L2_CID_DETECT_CLASS (class)`
    Detect 绫绘弿杩扮銆傚璇ユ帶鍒惰皟鐢?VIDIOC_QUERYCTRL 灏嗚繑鍥炶鎺у埗绫荤殑鎻忚堪銆?
`V4L2_CID_DETECT_MD_MODE (menu)`
    璁剧疆杩愬姩妫€娴嬫ā寮忋€?

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_DETECT_MD_MODE_DISABLED`
      - 绂佺敤杩愬姩妫€娴嬨€?    - - `V4L2_DETECT_MD_MODE_GLOBAL`
      - 浣跨敤鍗曚竴鐨勮繍鍔ㄦ娴嬮槇鍊笺€?    - - `V4L2_DETECT_MD_MODE_THRESHOLD_GRID`
      - 灏嗗浘鍍忓垝鍒嗕负缃戞牸锛屾瘡涓崟鍏冩牸鏈夎嚜宸辩殑杩愬姩妫€娴嬮槇鍊笺€傝繖浜涢槇鍊奸€氳繃
	`V4L2_CID_DETECT_MD_THRESHOLD_GRID` 鐭╅樀鎺у埗璁剧疆銆?    - - `V4L2_DETECT_MD_MODE_REGION_GRID`
      - 灏嗗浘鍍忓垝鍒嗕负缃戞牸锛屾瘡涓崟鍏冩牸鏈夎嚜宸辩殑鍖哄煙鍊硷紝鐢ㄤ簬鎸囧畾搴斿綋浣跨敤鍝釜
	姣忓尯鍩熺殑杩愬姩妫€娴嬮槇鍊笺€傛瘡涓尯鍩熼兘鏈夎嚜宸辩殑闃堝€笺€傝繖浜涙瘡鍖哄煙闃堝€肩殑璁剧疆鏂瑰紡
	鏄┍鍔ㄧ浉鍏崇殑銆傜綉鏍肩殑鍖哄煙鍊奸€氳繃 `V4L2_CID_DETECT_MD_REGION_GRID` 鐭╅樀鎺у埗璁剧疆銆?


`V4L2_CID_DETECT_MD_GLOBAL_THRESHOLD (integer)`
    璁剧疆涓?`V4L2_DETECT_MD_MODE_GLOBAL` 杩愬姩妫€娴嬫ā寮忎竴璧蜂娇鐢ㄧ殑鍏ㄥ眬杩愬姩妫€娴嬮槇鍊笺€?
`V4L2_CID_DETECT_MD_THRESHOLD_GRID (__u16 matrix)`
    璁剧疆缃戞牸涓瘡涓崟鍏冩牸鐨勮繍鍔ㄦ娴嬮槇鍊笺€傞渶涓?`V4L2_DETECT_MD_MODE_THRESHOLD_GRID`
    杩愬姩妫€娴嬫ā寮忎竴璧蜂娇鐢ㄣ€傜煩闃靛厓绱?(0, 0) 琛ㄧず缃戞牸宸︿笂瑙掔殑鍗曞厓鏍笺€?
`V4L2_CID_DETECT_MD_REGION_GRID (__u8 matrix)`
    璁剧疆缃戞牸涓瘡涓崟鍏冩牸鐨勮繍鍔ㄦ娴嬪尯鍩熷€笺€傞渶涓?`V4L2_DETECT_MD_MODE_REGION_GRID`
    杩愬姩妫€娴嬫ā寮忎竴璧蜂娇鐢ㄣ€傜煩闃靛厓绱?(0, 0) 琛ㄧず缃戞牸宸︿笂瑙掔殑鍗曞厓鏍笺€?