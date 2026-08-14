
######## ioctl VIDIOC_G_ENC_INDEX


## 鍚嶇О


VIDIOC_G_ENC_INDEX - 鑾峰彇鍘嬬缉瑙嗛娴佺殑鍏冩暟鎹?
## 鎽樿


`int ioctl(int fd, VIDIOC_G_ENC_INDEX, struct v4l2_enc_idx *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_enc_idx` 鐨勬寚閽堛€?
## 鎻忚堪


VIDIOC_G_ENC_INDEX <VIDIOC_G_ENC_INDEX> ioctl 鎻愪緵鍏充簬褰撳墠鐢辨湰椹卞姩鎴栧彟涓€搴旂敤绋嬪簭浠?椹卞姩璇诲彇鐨勫帇缂╄棰戞祦鐨勫厓鏁版嵁锛岃繖瀵逛簬鏃犻渶瑙ｇ爜鍗冲彲闅忔満璁块棶璇ユ祦闈炲父鏈夌敤銆?
涓轰簡璇诲彇鏁版嵁锛屽簲鐢ㄧ▼搴忓繀椤昏皟鐢?VIDIOC_G_ENC_INDEX <VIDIOC_G_ENC_INDEX>锛屽苟浼犲叆涓€涓?鎸囧悜 struct `v4l2_enc_idx` 鐨勬寚閽堛€傛垚鍔熸椂锛岄┍鍔ㄤ細濉厖 `entry` 鏁扮粍锛?灏嗗啓鍏ョ殑鍏冪礌涓暟瀛樺叆 `entries` 瀛楁锛屽苟鍒濆鍖?`entries_cap` 瀛楁銆?
`entry` 鏁扮粍鐨勬瘡涓厓绱犲寘鍚竴骞呭浘鍍忕殑鍏冩暟鎹€備竴娆?VIDIOC_G_ENC_INDEX <VIDIOC_G_ENC_INDEX>
璋冪敤浼氫粠椹卞姩缂撳啿鍖轰腑璇诲彇鏈€澶?`V4L2_ENC_IDX_ENTRIES` 涓潯鐩紝璇ョ紦鍐插尯鏈€澶氬彲瀹圭撼
`entries_cap` 涓潯鐩€傝鏁板瓧鍙互楂樹簬鎴栦綆浜?`V4L2_ENC_IDX_ENTRIES`锛屼絾涓嶈兘涓洪浂銆傚綋
搴旂敤绋嬪簭鏈兘鍙婃椂璇诲彇鍏冩暟鎹椂锛屾渶鏃х殑鏉＄洰灏嗕細涓㈠け銆傚綋缂撳啿鍖轰负绌烘垨娌℃湁杩涜鎹曡幏/缂栫爜鏃讹紝
`entries` 灏嗕负闆躲€?
鐩墠姝?ioctl 浠呴拡瀵?MPEG-2 program stream 涓?video elementary stream 瀹氫箟銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 3 8

    - - __u32
      - `entries`
      - 椹卞姩瀛樺叆 `entry` 鏁扮粍鐨勬潯鐩暟閲忋€?    - - __u32
      - `entries_cap`
      - 椹卞姩鍙紦鍐茬殑鏉＄洰鏁伴噺銆傚繀椤诲ぇ浜庨浂銆?    - - __u32
      - `reserved`\ [^4^]
      - 淇濈暀渚涘皢鏉ユ墿灞曘€傞┍鍔ㄥ繀椤诲皢鏁扮粍缃浂銆?    - - struct `v4l2_enc_idx_entry`
      - `entry`\ [`V4L2_ENC_IDX_ENTRIES`]
      - 鍏充簬鍘嬬缉瑙嗛娴佺殑鍏冩暟鎹€傛暟缁勭殑姣忎釜鍏冪礌瀵瑰簲涓€骞呭浘鍍忥紝鎸?	鍏?`offset` 鍗囧簭鎺掑垪銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u64
      - `offset`
      - 浠庡帇缂╄棰戞祦寮€澶村埌鏈箙鍥惧儚寮€澶寸殑瀛楄妭鍋忕Щ閲忥紝鍗?mpeg2part1
	涓畾涔夌殑 *PES 鍖呭ご閮?锛屾垨 mpeg2part2 涓畾涔夌殑 **鍥惧儚澶撮儴*銆?	褰撶紪鐮佸櫒鍋滄鏃讹紝椹卞姩灏嗗亸绉婚噺閲嶇疆涓洪浂銆?    - - __u64
      - `pts`
      - 鏈箙鍥惧儚鐨?33 浣?**鏄剧ず鏃堕棿鎴筹紙Presentation Time Stamp锛?*锛?	瀹氫箟瑙?mpeg2part1銆?    - - __u32
      - `length`
      - 鏈箙鍥惧儚鐨勫瓧鑺傞暱搴︺€?    - - __u32
      - `flags`
      - 鍖呭惈鏈箙鍥惧儚缂栫爜绫诲瀷鐨勬爣蹇椾綅锛屽弬瑙?enc-idx-flags銆?    - - __u32
      - `reserved`\ [^2^]
      - 淇濈暀渚涘皢鏉ユ墿灞曘€傞┍鍔ㄥ繀椤诲皢鏁扮粍缃浂銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_ENC_IDX_FRAME_I`
      - 0x00
      - 杩欐槸甯у唴缂栫爜鍥惧儚锛圛 甯э級銆?    - - `V4L2_ENC_IDX_FRAME_P`
      - 0x01
      - 杩欐槸鍓嶅悜棰勬祴缂栫爜鍥惧儚锛圥 甯э級銆?    - - `V4L2_ENC_IDX_FRAME_B`
      - 0x02
      - 杩欐槸鍙屽悜棰勬祴缂栫爜鍥惧儚锛圔 甯э級銆?    - - `V4L2_ENC_IDX_FRAME_MASK`
      - 0x0F
      - 灏?flags 瀛楁涓庢鎺╃爜鍋?**AND** 杩愮畻鍗冲彲寰楀埌鍥惧儚缂栫爜绫诲瀷銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞惰缃?`errno` 鍙橀噺銆?閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?