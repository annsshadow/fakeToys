


######## ioctl VIDIOC_ENCODER_CMD, VIDIOC_TRY_ENCODER_CMD


## 鍚嶇О


VIDIOC_ENCODER_CMD - VIDIOC_TRY_ENCODER_CMD - 鎵ц涓€鏉＄紪鐮佸櫒鍛戒护

## 姒傝



`int ioctl(int fd, VIDIOC_ENCODER_CMD, struct v4l2_encoder_cmd *argp)`


`int ioctl(int fd, VIDIOC_TRY_ENCODER_CMD, struct v4l2_encoder_cmd *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_encoder_cmd` 鐨勬寚閽堛€?
## 鎻忚堪


杩欎簺 ioctl 鎺у埗涓€涓煶棰?瑙嗛锛堥€氬父鏄?MPEG-锛夌紪鐮佸櫒銆俙VIDIOC_ENCODER_CMD`
鍚戠紪鐮佸櫒鍙戦€佷竴鏉″懡浠わ紝`VIDIOC_TRY_ENCODER_CMD` 鍙敤浜庡湪涓嶅疄闄呮墽琛岀殑鎯呭喌涓?灏濊瘯涓€鏉″懡浠ゃ€?
瑕佸彂閫佷竴鏉″懡浠わ紝搴旂敤绋嬪簭蹇呴』鍒濆鍖?struct `v4l2_encoder_cmd` 鐨勬墍鏈夊瓧娈碉紝骞?浠ユ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`VIDIOC_ENCODER_CMD` 鎴?`VIDIOC_TRY_ENCODER_CMD`銆?
`cmd` 瀛楁蹇呴』鍖呭惈鍛戒护鐮併€傛煇浜涘懡浠や娇鐢?`flags` 瀛楁鏉ユ惡甯﹂檮鍔犱俊鎭€?
鍦?STOP 鍛戒护涔嬪悗锛宍read()` 璋冪敤浼氳鍙栭┍鍔ㄧ紦鍐茬殑鍓╀綑鏁版嵁銆傚綋缂撳啿鍖轰负绌烘椂锛?`read()` 灏嗚繑鍥為浂锛岃€屼笅涓€娆?`read()` 璋冪敤浼氶噸鏂板惎鍔ㄧ紪鐮佸櫒銆?
濡傛灉缂栫爜鍣ㄥ皻鏈惎鍔紝涓€娆?`read()` 鎴?VIDIOC_STREAMON <VIDIOC_STREAMON>
璋冪敤浼氬悜缂栫爜鍣ㄥ彂閫佷竴涓殣寮忕殑 START 鍛戒护銆傞€傜敤浜?mem2mem 缂栫爜鍣ㄧ殑涓や釜闃熷垪銆?
瀵规鍦ㄦ祦寮忎紶杈撶殑鏂囦欢鎻忚堪绗︾殑涓€娆?`close()` 鎴?VIDIOC_STREAMOFF <VIDIOC_STREAMON>
璋冪敤浼氬悜缂栫爜鍣ㄥ彂閫佷竴涓殣寮忕殑绔嬪嵆 STOP锛屾墍鏈夌紦鍐叉暟鎹涓㈠純銆傞€傜敤浜?mem2mem
缂栫爜鍣ㄧ殑涓や釜闃熷垪銆?
杩欎簺 ioctl 鏄彲閫夌殑锛屽苟闈炴墍鏈夐┍鍔ㄩ兘鍙兘鏀寔瀹冧滑銆傚畠浠簬 Linux 2.6.21 寮曞叆銆?涓嶈繃锛屽浜庢湁鐘舵€侊紙stateful锛塵em2mem 缂栫爜鍣ㄨ€岃█瀹冧滑鏄己鍒剁殑锛堝 encoder 涓?杩涗竴姝ヨ鏄庯級銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `cmd`
      - 缂栫爜鍣ㄥ懡浠わ紝鍙傝 encoder-cmds銆?    - - __u32
      - `flags`
      - 涓庡懡浠ら厤濂楃殑鏍囧織锛屽弬瑙?encoder-flags銆傚鏋滄湭涓鸿鍛戒护瀹氫箟浠讳綍鏍囧織锛?	椹卞姩鍜屽簲鐢ㄧ▼搴忓繀椤诲皢璇ュ瓧娈电疆涓洪浂銆?    - - __u32
      - `data`\ [^8^]
      - 涓哄皢鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃负闆躲€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_ENC_CMD_START`
      - 0
      - 鍚姩缂栫爜鍣ㄣ€傚綋缂栫爜鍣ㄥ凡缁忓湪杩愯鎴栧凡鏆傚仠鏃讹紝璇ュ懡浠や笉鎵ц浠讳綍鎿嶄綔銆?	鏈负璇ュ懡浠ゅ畾涔変换浣曟爣蹇椼€?
	瀵逛簬瀹炵幇浜嗙紪鐮佸櫒鐨勮澶囷紝涓€鏃﹂€氳繃 `V4L2_ENC_CMD_STOP` 鍛戒护鍚姩浜?drain
	搴忓垪锛屽湪璇ュ懡浠よ璋冪敤涔嬪墠蹇呴』灏嗗叾椹卞姩鑷冲畬鎴愩€傚湪 drain 搴忓垪杩涜鏈熼棿浠讳綍
	璋冪敤璇ュ懡浠ょ殑灏濊瘯閮戒細瑙﹀彂 `EBUSY` 閿欒鐮併€傝瑙?encoder銆?    - - `V4L2_ENC_CMD_STOP`
      - 1
      - 鍋滄缂栫爜鍣ㄣ€傚綋璁剧疆浜?`V4L2_ENC_CMD_STOP_AT_GOP_END` 鏍囧織鏃讹紝缂栫爜灏?	鎸佺画鍒板綋鍓?*Group Of Pictures*锛堝浘鍍忕粍锛夌粨鏉燂紝鍚﹀垯缂栫爜灏嗙珛鍗冲仠姝€傚綋
	缂栫爜鍣ㄥ凡缁忓仠姝㈡椂锛岃鍛戒护涓嶆墽琛屼换浣曟搷浣溿€?
	瀵逛簬瀹炵幇浜嗙紪鐮佸櫒鐨勮澶囷紝璇ュ懡浠ゅ皢鍚姩 encoder 涓墍杩扮殑 drain 搴忓垪銆傛鏃?	涓嶆帴鍙椾换浣曟爣蹇楁垨鍏跺畠鍙傛暟銆傚湪搴忓垪瀹屾垚涔嬪墠浠讳綍鍐嶆璋冪敤璇ュ懡浠ょ殑灏濊瘯閮戒細
	瑙﹀彂 `EBUSY` 閿欒鐮併€?    - - `V4L2_ENC_CMD_PAUSE`
      - 2
      - 鏆傚仠缂栫爜鍣ㄣ€傚綋缂栫爜鍣ㄥ皻鏈惎鍔ㄦ椂锛岄┍鍔ㄥ皢杩斿洖 `EPERM` 閿欒鐮併€傚綋缂栫爜鍣?	宸茬粡鏆傚仠鏃讹紝璇ュ懡浠や笉鎵ц浠讳綍鎿嶄綔銆傛湭涓鸿鍛戒护瀹氫箟浠讳綍鏍囧織銆?    - - `V4L2_ENC_CMD_RESUME`
      - 3
      - 鍦?PAUSE 鍛戒护涔嬪悗鎭㈠缂栫爜銆傚綋缂栫爜鍣ㄥ皻鏈惎鍔ㄦ椂锛岄┍鍔ㄥ皢杩斿洖 `EPERM`
	閿欒鐮併€傚綋缂栫爜鍣ㄥ凡缁忓湪杩愯鏃讹紝璇ュ懡浠や笉鎵ц浠讳綍鎿嶄綔銆傛湭涓鸿鍛戒护瀹氫箟
	浠讳綍鏍囧織銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_ENC_CMD_STOP_AT_GOP_END`
      - 0x0001
      - 鍦ㄥ綋鍓?**Group Of Pictures**锛堝浘鍍忕粍锛夌粨鏉熸椂鍋滄缂栫爜锛岃€屼笉鏄珛鍗?	鍋滄銆?
        涓嶉€傜敤浜庣紪鐮佸櫒銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
EBUSY
    瀹炵幇浜嗙紪鐮佸櫒鐨勮澶囩殑 drain 搴忓垪浠嶅湪杩涜涓€傚湪瀹屾垚涔嬪墠涓嶅厑璁稿彂鍑哄彟涓€鏉?    缂栫爜鍣ㄥ懡浠ゃ€?
EINVAL
    `cmd` 瀛楁鏃犳晥銆?
EPERM
    搴旂敤绋嬪簭鍦ㄧ紪鐮佸櫒鏈繍琛屾椂鍙戦€佷簡 PAUSE 鎴?RESUME 鍛戒护銆?