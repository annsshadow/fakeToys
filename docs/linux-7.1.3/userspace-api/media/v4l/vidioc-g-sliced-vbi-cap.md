

######## ioctl VIDIOC_G_SLICED_VBI_CAP


## 鍚嶇О


VIDIOC_G_SLICED_VBI_CAP - 鏌ヨ鍒囩墖锛坰liced锛塚BI 鑳藉姏

## 鎽樿


`int ioctl(int fd, VIDIOC_G_SLICED_VBI_CAP, struct v4l2_sliced_vbi_cap *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_sliced_vbi_cap` 鐨勬寚閽堛€?
## 鎻忚堪


涓轰簡鏌ユ槑鍒囩墖 VBI 鎹曡幏鎴栬緭鍑鸿澶囨敮鎸佸摢浜涙暟鎹湇鍔★紝搴旂敤绋嬪簭鍒濆鍖?struct
`v4l2_sliced_vbi_cap` 鐨?`type` 瀛楁锛屾竻闄?`reserved` 鏁扮粍锛屽苟璋冪敤
VIDIOC_G_SLICED_VBI_CAP <VIDIOC_G_SLICED_VBI_CAP> ioctl銆傞┍鍔ㄥ～鍏呭叾浣欏瓧娈碉紝
濡傛灉鍒囩墖 VBI API 涓嶅彈鏀寔鎴?`type` 鏃犳晥锛屽垯杩斿洖 `EINVAL` 閿欒鐮併€?

    `type` 瀛楁鏄湪 Linux 2.6.19 涓坊鍔犵殑锛屽苟涓旇 ioctl 浠庡彧璇诲彉鏇翠负浜?    璇诲啓銆?

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 3 2 2 2

    - - __u16
      - `service_set`
      - `2` 椹卞姩鏀寔鐨勬墍鏈夋暟鎹湇鍔＄殑闆嗗悎銆?
	绛変簬 `service_lines` 鏁扮粍鎵€鏈夊厓绱犵殑骞堕泦銆?    - - __u16
      - `service_lines`\ [^2^][^24^]
      - `2` 姝ゆ暟缁勭殑姣忎釜鍏冪礌鍖呭惈涓€涓暟鎹湇鍔￠泦鍚堬紝纭欢鍙互鍦ㄧ壒瀹氭壂鎻?	琛屼笂鏌ユ壘鎴栨彃鍏ヨ繖浜涙湇鍔°€傛暟鎹湇鍔″湪 vbi-services 涓畾涔夈€?	鏁扮粍绱㈠紩鏄犲皠鍒?ITU-R 琛屽彿\ [#f1]_锛屽涓嬫墍绀猴細
#     * -

      - 鍏冪礌
      - 525 琛岀郴缁?      - 625 琛岀郴缁?#     * -

      - `service_lines`\ [^0^][^1^]
      - 1
      - 1
#     * -

      - `service_lines`\ [^0^][^23^]
      - 23
      - 23
#     * -

      - `service_lines`\ [^1^][^1^]
      - 264
      - 314
#     * -

      - `service_lines`\ [^1^][^23^]
      - 286
      - 336
    - -
#     * -

      - `2` 纭欢姣忓抚鍙互鎹曡幏鎴栬緭鍑虹殑 VBI 琛屾暟锛屾垨鑰呭畠鑳藉湪缁欏畾琛屼笂璇嗗埆鐨?	鏈嶅姟鏁伴噺鍙兘鏄彈闄愮殑銆備緥濡傦紝鍦?PAL 绗?16 琛屼笂锛岀‖浠跺彲鑳借兘澶熸煡鎵?	VPS 鎴?Teletext 淇″彿锛屼絾涓嶈兘鍚屾椂鏌ユ壘涓よ€呫€傚簲鐢ㄧ▼搴忓彲浠ヤ娇鐢?	VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl锛堝 sliced 涓墍杩帮級鏉ヤ簡瑙ｈ繖浜涢檺鍒躲€?    - -
#     * -

      - `2` 椹卞姩蹇呴』灏?`service_lines` [^0^][^0^] 鍜?	`service_lines`\ [^1^][^0^] 璁句负闆躲€?    - - __u32
      - `type`
      - 鏁版嵁娴佺殑绫诲瀷锛岃 `v4l2_buf_type`銆傚簲涓?	`V4L2_BUF_TYPE_SLICED_VBI_CAPTURE` 鎴?	`V4L2_BUF_TYPE_SLICED_VBI_OUTPUT`銆?    - - __u32
      - `reserved`\ [^3^]
      - `2` 姝ゆ暟缁勪负灏嗘潵鎵╁睍淇濈暀銆?
	搴旂敤绋嬪簭鍜岄┍鍔ㄥ繀椤诲皢鍏惰涓洪浂銆?

   鍙﹁ vbi-525 鍜?vbi-625銆?

    \scriptsize



    :header-rows:  1
    :stub-columns: 0
    :widths:       2 1 1 2 2

    - - 绗﹀彿
      - 鍊?      - 鍙傝€?      - 琛岋紝閫氬父
      - 杞借嵎
    - - `V4L2_SLICED_TELETEXT_B`锛圱eletext System B锛?      - 0x0001
      - ets300706銆?
	itu653
      - PAL/SECAM 绗?7-22 琛屻€?20-335 琛岋紙绗簩鍦?7-22 琛岋級
      - 45 瀛楄妭 Teletext 鍖呯殑鏈€鍚?42 瀛楄妭锛屽嵆涓嶅甫鏃堕挓
	寮曞鍜屾垚甯х爜锛屾渶浣庝綅锛坙sb锛夊厛浼犺緭銆?    - - `V4L2_SLICED_VPS`
      - 0x0400
      - ets300231
      - PAL 绗?16 琛?      - 鏍规嵁 ETS 300 231 鍥?9 鐨勭 3 鍒?15 瀛楄妭锛屾渶浣庝綅鍏堜紶杈撱€?    - - `V4L2_SLICED_CAPTION_525`
      - 0x1000
      - cea608
      - NTSC 绗?21銆?84 琛岋紙绗簩鍦?21 琛岋級
      - 浼犺緭椤哄簭鐨勪袱涓瓧鑺傦紝鍖呮嫭濂囧伓鏍￠獙浣嶏紝鏈€浣庝綅鍏堜紶杈撱€?    - - `V4L2_SLICED_WSS_625`
      - 0x4000
      - en300294銆?
	itu1119
      - PAL/SECAM 绗?23 琛?      - 瑙佷笅鏂圭殑 v4l2-sliced-vbi-cap-wss-625-payload銆?    - - `V4L2_SLICED_VBI_525`
      - 0x1000
      - `2` 閫傜敤浜?525 琛岀郴缁熺殑鏈嶅姟闆嗗悎銆?    - - `V4L2_SLICED_VBI_625`
      - 0x4401
      - `2` 閫傜敤浜?625 琛岀郴缁熺殑鏈嶅姟闆嗗悎銆?


    \normalsize


#### V4L2_SLICED_VBI_CAP WSS_625 杞借嵎


`V4L2_SLICED_WSS_625` 鐨勮浇鑽蜂负锛?
	    +-----+------------------+-----------------------+
	    |瀛楄妭 |        0         |           1           |
	    +-----+--------+---------+-----------+-----------+
	    |     | msb    | lsb     | msb       | lsb       |
	    |     +-+-+-+--+--+-+-+--+--+-+--+---+---+--+-+--+
	    | 浣?|7|6|5|4 | 3|2|1|0 | x|x|13|12 | 11|10|9|8 |
	    +-----+-+-+-+--+--+-+-+--+--+-+--+---+---+--+-+--+


## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤鐨勯敊璇爜
鍦ㄩ€氱敤閿欒鐮?<gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    `type` 瀛楁涓殑鍊奸敊璇€?