


######## ioctl VIDIOC_SUBDEV_G_CLIENT_CAP, VIDIOC_SUBDEV_S_CLIENT_CAP


## 鍚嶇О


VIDIOC_SUBDEV_G_CLIENT_CAP - VIDIOC_SUBDEV_S_CLIENT_CAP - 鑾峰彇鎴栬缃鎴风
鑳藉姏銆?
## 姒傝


`int ioctl(int fd, VIDIOC_SUBDEV_G_CLIENT_CAP, struct v4l2_subdev_client_capability *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_CLIENT_CAP, struct v4l2_subdev_client_capability *argp)`

## 鍙傛暟


`fd`
    open() <func-open> 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_client_capability` 鐨勬寚閽堛€?
## 鎻忚堪


杩欎簺 ioctl 鐢ㄤ簬鑾峰彇鍜岃缃鎴风锛堜娇鐢?subdevice ioctl 鐨勫簲鐢ㄧ▼搴忥級鑳藉姏銆傚鎴风
鑳藉姏瀛樺偍鍦ㄦ墦寮€鐨?subdev 璁惧鑺傜偣鐨勬枃浠跺彞鏌勪腑锛屽鎴风蹇呴』鍒嗗埆涓烘瘡涓墦寮€鐨?subdev 璁剧疆鑳藉姏銆?
榛樿鎯呭喌涓嬶紝鎵撳紑 subdev 璁惧鑺傜偣鏃朵笉璁剧疆浠讳綍瀹㈡埛绔兘鍔涖€?
瀹㈡埛绔兘鍔涚殑鐢ㄩ€旀槸鍛婄煡鍐呮牳璇ュ鎴风鐨勮涓猴紝涓昏涓庝繚鎸佷笉鍚屽唴鏍镐笌鐢ㄦ埛绌洪棿鐗堟湰
涔嬮棿鐨勫吋瀹规€ф湁鍏炽€?
`VIDIOC_SUBDEV_G_CLIENT_CAP` ioctl 杩斿洖涓庢枃浠跺彞鏌?`fd` 鍏宠仈鐨勫綋鍓嶅鎴风鑳藉姏銆?
`VIDIOC_SUBDEV_S_CLIENT_CAP` ioctl 璁剧疆鏂囦欢鍙ユ焺 `fd` 鐨勫鎴风鑳藉姏銆傛柊鐨勮兘鍔涗細
瀹屽叏鏇挎崲褰撳墠鑳藉姏锛屽洜姝よ ioctl 涔熷彲鐢ㄤ簬绉婚櫎鍏堝墠宸茶缃殑鑳藉姏銆?
`VIDIOC_SUBDEV_S_CLIENT_CAP` 浼氫慨鏀?struct `v4l2_subdev_client_capability` 浠?鍙嶆槧宸茶鎺ュ彈鐨勮兘鍔涖€傚唴鏍镐笉鎺ュ彈鏌愯兘鍔涚殑涓€绉嶅父瑙佹儏鍐垫槸锛屽唴鏍告瘮鐢ㄦ埛绌洪棿浣跨敤鐨?澶存枃浠舵洿鏃э紝鍥犳璇ヨ兘鍔涘鍐呮牳鑰岃█鏄湭鐭ョ殑銆?

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 4 20

    - - __u64
      - `capabilities`
      - 鎵€鎵撳紑璁惧鐨勫瓙璁惧瀹㈡埛绔兘鍔涖€?
    :header-rows:  1

    - - 鑳藉姏
      - 鎻忚堪
    - - `V4L2_SUBDEV_CLIENT_CAP_STREAMS`
      - 瀹㈡埛绔簡瑙ｆ祦锛坰tream锛夈€傝缃鏍囧織鍙惎鐢ㄥ悇绉?ioctl 涓?'stream' 瀛楁
        锛堟寚娴佺紪鍙凤級鐨勪娇鐢ㄣ€傝嫢鏈缃紙榛樿濡傛锛夛紝'stream' 瀛楁灏嗚鍐呮牳寮哄埗
        涓?0銆?    - - `V4L2_SUBDEV_CLIENT_CAP_INTERVAL_USES_WHICH`
      - 瀹㈡埛绔簡瑙?`v4l2_subdev_frame_interval` 鐨?`which` 瀛楁銆傝嫢鏈缃?        锛堥粯璁ゅ姝わ級锛宍which` 瀛楁灏嗚鍐呮牳寮哄埗涓?`V4L2_SUBDEV_FORMAT_ACTIVE`銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
閫氱敤閿欒鐮?<gen-errors> 绔犺妭涓弿杩般€?
ENOIOCTLCMD
   鍐呮牳涓嶆敮鎸佹 ioctl銆?