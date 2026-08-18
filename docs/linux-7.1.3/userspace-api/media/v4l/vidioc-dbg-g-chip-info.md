


######## ioctl VIDIOC_DBG_G_CHIP_INFO


## 鍚嶇О


VIDIOC_DBG_G_CHIP_INFO - 璇嗗埆鐢佃鍗′笂鐨勮姱鐗?
## 璇硶


`int ioctl(int fd, VIDIOC_DBG_G_CHIP_INFO, struct v4l2_dbg_chip_info *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_dbg_chip_info` 鐨勬寚閽堛€?
## 鎻忚堪



    杩欐槸涓€涓疄楠屾€ф帴鍙ｏ紝灏嗘潵鍙兘浼氬彂鐢熷彉鍖栥€?
鍑轰簬椹卞姩璋冭瘯鐩殑锛岃 ioctl 鍏佽娴嬭瘯绋嬪簭鍚戦┍鍔ㄦ煡璇㈢數瑙嗗崱涓婂瓨鍦ㄧ殑鑺墖淇℃伅銆傛櫘閫?搴旂敤绋嬪簭涓嶅緱浣跨敤璇ユ帴鍙ｃ€傚鏋滀綘鍙戠幇浜嗚姱鐗囩浉鍏崇殑 bug锛岃鑱旂郴 linux-media 閭欢鍒楄〃
锛坄https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__锛夛紝
浠ヤ究淇銆?
姝ゅ锛孡inux 鍐呮牳蹇呴』寮€鍚?`CONFIG_VIDEO_ADV_DEBUG` 閫夐」缂栬瘧锛屾墠鑳藉惎鐢ㄨ ioctl銆?
瑕佹煡璇㈤┍鍔紝搴旂敤绋嬪簭蹇呴』鍒濆鍖?struct `v4l2_dbg_chip_info` 鐨?`match.type` 涓?`match.addr` 鎴?`match.name` 瀛楁锛屽苟浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?VIDIOC_DBG_G_CHIP_INFO銆傛垚鍔熸椂锛岄┍鍔ㄥ皢鎵€閫夎姱鐗囩殑淇℃伅瀛樺叆 `name` 鍜?`flags` 瀛楁銆?
褰?`match.type` 涓?`V4L2_CHIP_MATCH_BRIDGE` 鏃讹紝`match.addr` 閫夋嫨鐢佃鍗′笂鐨勭 n
涓ˉ鎺モ€滆姱鐗団€濄€備綘鍙互浠?0 寮€濮嬶紝姣忔灏?`match.addr` 鍔?1锛岀洿鍒?VIDIOC_DBG_G_CHIP_INFO
浠?`EINVAL` 閿欒鐮佸け璐ワ紝浠庤€屾灇涓炬墍鏈夎姱鐗囥€傜紪鍙?0 鎬绘槸閫夋嫨妗ユ帴鑺墖鏈韩锛屼緥濡傝繛鎺ュ埌
PCI 鎴?USB 鎬荤嚎鐨勮姱鐗囥€傞潪闆剁紪鍙锋爣璇嗘ˉ鎺ヨ姱鐗囩殑鐗瑰畾閮ㄥ垎锛屼緥濡備竴涓?AC97 瀵勫瓨鍣ㄥ潡銆?
褰?`match.type` 涓?`V4L2_CHIP_MATCH_SUBDEV` 鏃讹紝`match.addr` 閫夋嫨绗?n 涓瓙璁惧銆?杩欏厑璁镐綘鏋氫妇鎵€鏈夊瓙璁惧銆?
鎴愬姛鏃讹紝`name` 瀛楁灏嗗寘鍚竴涓姱鐗囧悕绉帮紝`flags` 瀛楁鍦ㄩ┍鍔ㄦ敮鎸佷粠璁惧璇诲彇瀵勫瓨鍣ㄦ椂
鍖呭惈 `V4L2_CHIP_FL_READABLE`锛屾垨鍦ㄩ┍鍔ㄦ敮鎸佸悜璁惧鍐欏叆瀵勫瓨鍣ㄦ椂鍖呭惈
`V4L2_CHIP_FL_WRITABLE`銆?
鐩告瘮鐩存帴璋冪敤璇?ioctl锛屾垜浠帹鑽愪娇鐢?v4l2-dbg 宸ュ叿銆傚畠鍙粠 LinuxTV v4l-dvb 浠撳簱
鑾峰彇锛屽弬瑙?`https://linuxtv.org/repo/ <https://linuxtv.org/repo/>`__ 浠ヨ幏鍙?璁块棶璇存槑銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 鍙兘鐨勭被鍨嬪垪琛紝鍙傝 name-chip-match-types銆?    - - union {
      - (anonymous)
    - - __u32
      - `addr`
      - 鎸夋缂栧彿鍖归厤鑺墖锛屽叿浣撹В閲婂彇鍐充簬 `type` 瀛楁銆?    - - char
      - `name[^32^]`
      - 鎸夋鍚嶇О鍖归厤鑺墖锛屽叿浣撹В閲婂彇鍐充簬 `type` 瀛楁銆傚綋鍓嶆湭浣跨敤銆?    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct v4l2_dbg_match
      - `match`
      - 濡備綍鍖归厤鑺墖锛屽弬瑙?name-v4l2-dbg-match銆?    - - char
      - `name[^32^]`
      - 鑺墖鐨勫悕绉般€?    - - __u32
      - `flags`
      - 鐢遍┍鍔ㄨ缃€傝嫢璁剧疆浜?`V4L2_CHIP_FL_READABLE`锛屽垯椹卞姩鏀寔浠庤澶囪鍙栧瘎瀛樺櫒銆?	鑻ヨ缃簡 `V4L2_CHIP_FL_WRITABLE`锛屽垯鏀寔鍐欏叆瀵勫瓨鍣ㄣ€?    - - __u32
      - `reserved[^8^]`
      - 淇濈暀瀛楁锛屽簲鐢ㄧ▼搴忎笌椹卞姩閮藉繀椤诲皢鍏剁疆涓?0銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CHIP_MATCH_BRIDGE`
      - 0
      - 鍖归厤鍗′笂鐨勭 n 涓姱鐗囷紝0 琛ㄧず妗ユ帴鑺墖銆備笉鍖归厤瀛愯澶囥€?    - - `V4L2_CHIP_MATCH_SUBDEV`
      - 4
      - 鍖归厤绗?n 涓瓙璁惧銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    `match_type` 鏃犳晥锛屾垨鏃犳硶鍖归厤鍒颁换浣曡澶囥€?