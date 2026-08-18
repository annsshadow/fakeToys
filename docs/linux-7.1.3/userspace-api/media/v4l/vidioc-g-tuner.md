


######## ioctl VIDIOC_G_TUNER, VIDIOC_S_TUNER


## 鍚嶇О


VIDIOC_G_TUNER - VIDIOC_S_TUNER - 鑾峰彇鎴栬缃皟璋愬櫒灞炴€?
## 姒傝



`int ioctl(int fd, VIDIOC_G_TUNER, struct v4l2_tuner *argp)`


`int ioctl(int fd, VIDIOC_S_TUNER, const struct v4l2_tuner *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_tuner` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇㈡煇涓皟璋愬櫒鐨勫睘鎬э紝搴旂敤绋嬪簭鍒濆鍖?struct `v4l2_tuner` 鐨?`index` 瀛楁骞跺皢 `reserved` 鏁扮粍娓呴浂锛岀劧鍚庝互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 `VIDIOC_G_TUNER` ioctl銆傚綋绱㈠紩瓒婄晫鏃讹紝椹卞姩濉厖缁撴瀯鐨勫叾浣欓儴鍒嗘垨杩斿洖 `EINVAL` 閿欒鐮併€傝鏋氫妇鎵€鏈夎皟璋愬櫒锛屽簲鐢ㄧ▼搴忓簲浠庣储寮?0 寮€濮嬶紝姣忔閫掑 1锛岀洿鍒伴┍鍔ㄨ繑鍥?`EINVAL`銆?
璋冭皭鍣ㄦ湁涓や釜鍙啓灞炴€э細闊抽妯″紡鍜屾棤绾跨數棰戠巼銆傝鏇存敼闊抽妯″紡锛屽簲鐢ㄧ▼搴忓垵濮嬪寲 `index`銆乣audmode` 鍜?`reserved` 瀛楁骞惰皟鐢?`VIDIOC_S_TUNER` ioctl銆傝繖 **涓嶄細** 鏀瑰彉褰撳墠鐨勮皟璋愬櫒锛屽綋鍓嶈皟璋愬櫒鐢卞綋鍓嶈棰戣緭鍏ュ喅瀹氥€傚鏋滄墍璇锋眰鐨勬ā寮忔棤鏁堟垨涓嶅彈鏀寔锛岄┍鍔ㄥ彲浠ラ€夋嫨涓€涓笉鍚岀殑闊抽妯″紡銆傜敱浜庤繖鏄竴涓彧鍐?ioctl锛屽畠涓嶄細杩斿洖瀹為檯琚€変腑鐨勯煶棰戞ā寮忋€?
SDR <sdr> 鐗瑰畾鐨勮皟璋愬櫒绫诲瀷鏄?`V4L2_TUNER_SDR` 鍜?`V4L2_TUNER_RF`銆傚浜?SDR 璁惧锛宍audmode` 瀛楁蹇呴』鍒濆鍖栦负闆躲€傚湪姝や笂涓嬫枃涓紝"tuner" 涓€璇嶆寚鐨勬槸 SDR 鎺ユ敹鍣ㄣ€?
瑕佹洿鏀规棤绾跨數棰戠巼锛屽彲浣跨敤 VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl銆?
 .. tabularcolumns:: |p{1.3cm}|p{3.0cm}|p{7.0cm}|p{5.8cm}|



    :header-rows:  0
    :stub-columns: 0

    - - __u32
      - `index`
      - `1` 鏍囪瘑璋冭皭鍣紝鐢卞簲鐢ㄧ▼搴忚缃€?    - - __u8
      - `name`\ [^32^]
      - `1`

	璋冭皭鍣ㄧ殑鍚嶇О锛屼竴涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓层€?
	璇ヤ俊鎭緵鐢ㄦ埛浣跨敤銆?    - - __u32
      - `type`
      - `1` 璋冭皭鍣ㄧ殑绫诲瀷锛屽弬瑙?`v4l2_tuner_type`銆?    - - __u32
      - `capability`
      - `1`

	璋冭皭鍣ㄨ兘鍔涙爣蹇楋紝鍙傝 tuner-capability銆傞煶棰戞爣蹇楄〃绀鸿В鐮侀煶棰戝瓙鑺傜洰锛坰ubprogram锛夌殑鑳藉姏銆傚畠浠?**涓嶄細** 鏀瑰彉锛屼緥濡備笉浼氶殢褰撳墠瑙嗛鏍囧噯鑰屾敼鍙樸€?
	褰撹缁撴瀯寮曠敤涓€涓棤绾跨數璋冭皭鍣ㄦ椂锛宍V4L2_TUNER_CAP_LANG1`銆乣V4L2_TUNER_CAP_LANG2` 鍜?`V4L2_TUNER_CAP_NORM` 鏍囧織涓嶈兘浣跨敤銆?
	濡傛灉鏀寔澶氫釜棰戝甫锛屽垯 `capability` 鏄瘡涓?struct `v4l2_frequency_band` 鐨勬墍鏈?`capability` 瀛楁鐨勫苟闆嗐€?    - - __u32
      - `rangelow`
      - `1` 鏈€浣庡彲璋冮鐜囷紝鍗曚綅涓?62.5 kHz锛涘鏋滆缃簡 `capability` 鏍囧織 `V4L2_TUNER_CAP_LOW`锛屽垯鍗曚綅涓?62.5 Hz锛涘鏋滆缃簡 `capability` 鏍囧織 `V4L2_TUNER_CAP_1HZ`锛屽垯鍗曚綅涓?1 Hz銆傚鏋滄敮鎸佸涓甯︼紝鍒?`rangelow` 鏄墍鏈夐甯︿腑鏈€浣庣殑棰戠巼銆?    - - __u32
      - `rangehigh`
      - `1` 鏈€楂樺彲璋冮鐜囷紝鍗曚綅涓?62.5 kHz锛涘鏋滆缃簡 `capability` 鏍囧織 `V4L2_TUNER_CAP_LOW`锛屽垯鍗曚綅涓?62.5 Hz锛涘鏋滆缃簡 `capability` 鏍囧織 `V4L2_TUNER_CAP_1HZ`锛屽垯鍗曚綅涓?1 Hz銆傚鏋滄敮鎸佸涓甯︼紝鍒?`rangehigh` 鏄墍鏈夐甯︿腑鏈€楂樼殑棰戠巼銆?    - - __u32
      - `rxsubchans`
      - `1`

	鏌愪簺璋冭皭鍣ㄦ垨闊抽瑙ｇ爜鍣ㄥ彲浠ラ€氳繃鍒嗘瀽闊抽杞芥尝銆佸棰戦煶鎴栧叾浠栨寚绀哄櫒鏉ョ‘瀹氭帴鏀跺埌鐨勯煶棰戝瓙鑺傜洰銆備负浜嗕紶閫掕淇℃伅锛岄┍鍔ㄥ湪鏈瓧娈典腑璁剧疆 tuner-rxsubchans 涓畾涔夌殑鏍囧織銆備緥濡傦細
#     * -

      - `V4L2_TUNER_SUB_MONO`
      - 鎺ユ敹鍗曞０閬撻煶棰?#     * -

      - `STEREO | SAP`
      - 鎺ユ敹绔嬩綋澹伴煶棰戝拰涓€涓緟鍔╅煶棰戣妭鐩?#     * -

      - `MONO | STEREO`
      - 鎺ユ敹鍗曞０閬撴垨绔嬩綋澹伴煶棰戯紝纭欢鏃犳硶鍖哄垎
#     * -

      - `LANG1 | LANG2`
      - 鎺ユ敹鍙岃闊抽
#     * -

      - `MONO | STEREO | LANG1 | LANG2`
      - 鎺ユ敹鍗曞０閬撱€佺珛浣撳０鎴栧弻璇煶棰?#     * -

      - `1`

	褰?`capability` 瀛楁涓殑 `V4L2_TUNER_CAP_STEREO`銆乣_LANG1`銆乣_LANG2` 鎴?`_SAP` 鏍囧織琚竻闄ゆ椂锛屾澶勪笉寰楄缃浉搴旂殑 `V4L2_TUNER_SUB_` 鏍囧織銆?
	鏈瓧娈典粎鍦ㄥ畠鏄綋鍓嶈棰戣緭鍏ョ殑璋冭皭鍣紝鎴栬€呰缁撴瀯寮曠敤涓€涓棤绾跨數璋冭皭鍣ㄦ椂鎵嶆湁鏁堛€?    - - __u32
      - `audmode`
      - `1`

	鎵€閫夌殑闊抽妯″紡锛屾湁鏁堝彇鍊煎弬瑙?tuner-audmode銆傞煶棰戞ā寮忎笉褰卞搷闊抽瀛愯妭鐩殑妫€娴嬶紝骞朵笖鍍忔帶鍒朵竴鏍凤紝闄ら潪鎵€璇锋眰鐨勬ā寮忔棤鏁堟垨涓嶅彈鏀寔锛屽惁鍒欎笉浼氳嚜鍔ㄦ敼鍙樸€傚叧浜庢墍閫夐煶棰戣妭鐩笌鎺ユ敹鍒扮殑闊抽鑺傜洰涓嶅尮閰嶆椂鍙兘鐨勭粨鏋滐紝鍙傝 tuner-matrix銆?
	鐩墠杩欐槸搴旂敤绋嬪簭鑳藉鏇存敼鐨?struct `v4l2_tuner` 鐨勫敮涓€瀛楁銆?    - - __u32
      - `signal`
      - `1` 淇″彿寮哄害锛堝鏋滃凡鐭ワ級銆?
	鍙栧€艰寖鍥翠负 0 鍒?65535銆傛暟鍊艰秺澶ц〃绀轰俊鍙疯秺濂姐€?    - - __s32
      - `afc`
      - `1` 鑷姩棰戠巼鎺у埗銆?
	褰?`afc` 鍊间负璐熸椂锛岄鐜囧亸浣庯紱涓烘鏃讹紝棰戠巼鍋忛珮銆?    - - __u32
      - `reserved`\ [^4^]
      - `1` 涓烘湭鏉ユ墿灞曚繚鐣欍€?
	椹卞姩鍜屽簲鐢ㄧ▼搴忛兘蹇呴』灏嗚鏁扮粍缃浂銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 6

    - - `V4L2_TUNER_RADIO`
      - 1
      - 璋冭皭鍣ㄦ敮鎸佹棤绾跨數
    - - `V4L2_TUNER_ANALOG_TV`
      - 2
      - 璋冭皭鍣ㄦ敮鎸佹ā鎷熺數瑙?    - - `V4L2_TUNER_SDR`
      - 4
      - 璋冭皭鍣ㄦ帶鍒惰蒋浠舵暟瀛楁棤绾跨數锛圫DR锛夌殑 A/D 鍜?鎴?D/A 妯″潡
    - - `V4L2_TUNER_RF`
      - 5
      - 璋冭皭鍣ㄦ帶鍒惰蒋浠舵暟瀛楁棤绾跨數锛圫DR锛夌殑灏勯閮ㄥ垎



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TUNER_CAP_LOW`
      - 0x0001
      - 璁剧疆鏃讹紝璋冭皭棰戠巼浠?62.5 Hz 涓哄崟浣嶏紝鑰岄潪 62.5 kHz銆?    - - `V4L2_TUNER_CAP_NORM`
      - 0x0002
      - 杩欐槸涓€涓鏍囧噯璋冭皭鍣紱瑙嗛鏍囧噯鍙互鎴栧繀椤昏鍒囨崲銆傦紙渚嬪 B/G PAL 璋冭皭鍣ㄩ€氬父涓嶈瑙嗕负澶氭爣鍑嗭紝鍥犱负瑙嗛鏍囧噯鏄牴鎹甯﹁嚜鍔ㄧ‘瀹氱殑銆傦級鎵€鏀寔鐨勮棰戞爣鍑嗛泦鍚堝彲浠庢寚鍚戣璋冭皭鍣ㄧ殑 struct `v4l2_input` 鑾峰彇锛岃瑙?ioctl VIDIOC_ENUMINPUT 鐨勬弿杩般€傚彧鏈?`V4L2_TUNER_ANALOG_TV` 璋冭皭鍣ㄥ彲浠ュ叿鏈夋鑳藉姏銆?    - - `V4L2_TUNER_CAP_HWSEEK_BOUNDED`
      - 0x0004
      - 濡傛灉璁剧疆锛屽垯璇ヨ皟璋愬櫒鏀寔纭欢鎼滅储鍔熻兘锛屽綋鎼滅储鍒拌揪棰戠巼鑼冨洿鏈鏃跺仠姝€?    - - `V4L2_TUNER_CAP_HWSEEK_WRAP`
      - 0x0008
      - 濡傛灉璁剧疆锛屽垯璇ヨ皟璋愬櫒鏀寔纭欢鎼滅储鍔熻兘锛屽綋鎼滅储鍒拌揪棰戠巼鑼冨洿鏈鏃跺洖缁曘€?    - - `V4L2_TUNER_CAP_STEREO`
      - 0x0010
      - 鏀寔绔嬩綋澹伴煶棰戞帴鏀躲€?    - - `V4L2_TUNER_CAP_LANG1`
      - 0x0040
      - 鏀寔鎺ユ敹鍙岃闊抽鑺傜洰鐨勪富瑕佽瑷€銆傚弻璇煶棰戞槸鍙岄€氶亾绯荤粺鐨勭壒鎬э紝鍦ㄤ富闊抽杞芥尝涓婂崟澹伴亾浼犺緭涓昏璇█锛屽湪绗簩涓浇娉笂鍗曞０閬撲紶杈撴瑕佽瑷€銆傚彧鏈?`V4L2_TUNER_ANALOG_TV` 璋冭皭鍣ㄥ彲浠ュ叿鏈夋鑳藉姏銆?    - - `V4L2_TUNER_CAP_LANG2`
      - 0x0020
      - 鏀寔鎺ユ敹鍙岃闊抽鑺傜洰鐨勬瑕佽瑷€銆傚彧鏈?`V4L2_TUNER_ANALOG_TV` 璋冭皭鍣ㄥ彲浠ュ叿鏈夋鑳藉姏銆?    - - `V4L2_TUNER_CAP_SAP`
      - 0x0020
      - 鏀寔鎺ユ敹杈呭姪闊抽鑺傜洰銆傝繖鏄即闅?NTSC 瑙嗛鏍囧噯鐨?BTSC 绯荤粺鐨勭壒鎬с€備富瑕佽瑷€鐨勫崟澹伴亾鎴栫珛浣撳０浼犺緭鏈変袱涓煶棰戣浇娉㈠彲鐢紝姝ゅ杩樻湁涓€涓嫭绔嬬殑绗笁杞芥尝鐢ㄤ簬鍗曞０閬撴瑕佽瑷€銆傚彧鏈?`V4L2_TUNER_ANALOG_TV` 璋冭皭鍣ㄥ彲浠ュ叿鏈夋鑳藉姏銆?
```

	   ``V4L2_TUNER_CAP_LANG2`` 鍜?``V4L2_TUNER_CAP_SAP``
	   鏍囧織鏄悓涔夎瘝銆俙`V4L2_TUNER_CAP_SAP`` 閫傜敤浜庢敮鎸?	   ``V4L2_STD_NTSC_M`` 瑙嗛鏍囧噯鐨勮皟璋愬櫒銆?    * - ``V4L2_TUNER_CAP_RDS``
      - 0x0080
      - 鏀寔 RDS 鎹曡幏銆傛鑳藉姏浠呭鏃犵嚎鐢佃皟璋愬櫒鏈夋晥銆?    * - ``V4L2_TUNER_CAP_RDS_BLOCK_IO``
      - 0x0100
      - RDS 鏁版嵁浠ユ湭瑙ｆ瀽鐨?RDS 鍧楀舰寮忎紶閫掋€?    * - ``V4L2_TUNER_CAP_RDS_CONTROLS``
      - 0x0200
      - RDS 鏁版嵁鐢辩‖浠惰В鏋愬苟閫氳繃鎺у埗璁剧疆銆?    * - ``V4L2_TUNER_CAP_FREQ_BANDS``
      - 0x0400
      - 鍙互浣跨敤 :ref:`VIDIOC_ENUM_FREQ_BANDS`
	ioctl 鏉ユ灇涓惧彲鐢ㄧ殑棰戝甫銆?    * - ``V4L2_TUNER_CAP_HWSEEK_PROG_LIM``
      - 0x0800
      - 浣跨敤纭欢鎼滅储鍔熻兘鏃舵悳绱㈢殑鑼冨洿鏄彲缂栫▼鐨勶紝璇﹁
	:ref:`VIDIOC_S_HW_FREQ_SEEK`銆?    * - ``V4L2_TUNER_CAP_1HZ``
      - 0x1000
      - 璁剧疆鏃讹紝璋冭皭棰戠巼浠?1 Hz 涓哄崟浣嶏紝鑰岄潪 62.5 kHz銆?

```

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TUNER_SUB_MONO`
      - 0x0001
      - 璋冭皭鍣ㄦ帴鏀跺崟澹伴亾闊抽淇″彿銆?    - - `V4L2_TUNER_SUB_STEREO`
      - 0x0002
      - 璋冭皭鍣ㄦ帴鏀剁珛浣撳０闊抽淇″彿銆?    - - `V4L2_TUNER_SUB_LANG1`
      - 0x0008
      - 璋冭皭鍣ㄦ帴鏀跺弻璇煶棰戜俊鍙风殑涓昏璇█銆傚綋褰撳墠瑙嗛鏍囧噯涓?`V4L2_STD_NTSC_M` 鏃讹紝椹卞姩蹇呴』娓呴櫎姝ゆ爣蹇椼€?    - - `V4L2_TUNER_SUB_LANG2`
      - 0x0004
      - 璋冭皭鍣ㄦ帴鏀跺弻璇煶棰戜俊鍙凤紙鎴栫浜屼釜闊抽鑺傜洰锛夌殑娆¤璇█銆?    - - `V4L2_TUNER_SUB_SAP`
      - 0x0004
      - 璋冭皭鍣ㄦ帴鏀惰緟鍔╅煶棰戣妭鐩€?
```

	   ``V4L2_TUNER_SUB_LANG2`` 鍜?``V4L2_TUNER_SUB_SAP``
	   鏍囧織鏄悓涔夎瘝銆俙`V4L2_TUNER_SUB_SAP`` 鏍囧織閫傜敤浜庡綋鍓嶈棰戞爣鍑嗕负
	   ``V4L2_STD_NTSC_M`` 鐨勬儏鍐点€?    * - ``V4L2_TUNER_SUB_RDS``
      - 0x0010
      - 璋冭皭鍣ㄦ帴鏀?RDS 淇￠亾銆?

```

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TUNER_MODE_MONO`
      - 0
      - 鎾斁鍗曞０閬撻煶棰戙€傚綋璋冭皭鍣ㄦ帴鏀剁珛浣撳０淇″彿鏃讹紝杩欐槸宸﹀彸澹伴亾鐨勪笅娣枫€傚綋璋冭皭鍣ㄦ帴鏀跺弻璇垨 SAP 淇″彿鏃讹紝姝ゆā寮忛€夋嫨涓昏璇█銆?    - - `V4L2_TUNER_MODE_STEREO`
      - 1
      - 鎾斁绔嬩綋澹伴煶棰戙€傚綋璋冭皭鍣ㄦ帴鏀跺弻璇煶棰戞椂锛屽畠鍙兘鍦ㄥ乏銆佸彸澹伴亾鎾斁涓嶅悓璇█锛屾垨鍦ㄤ袱涓０閬撴挱鏀句富瑕佽瑷€銆?
	鍦ㄦ妯″紡涓嬫挱鏀句笉鍚岃瑷€鐨勫仛娉曞凡琚純鐢ㄣ€傛柊鐨勯┍鍔ㄥ彧搴斿湪 `MODE_LANG1_LANG2` 涓繖鏍峰仛銆?
	褰撹皟璋愬櫒鏈帴鏀跺埌绔嬩綋澹颁俊鍙锋垨涓嶆敮鎸佺珛浣撳０鎺ユ敹鏃讹紝椹卞姩搴斿洖閫€鍒?`MODE_MONO`銆?    - - `V4L2_TUNER_MODE_LANG1`
      - 3
      - 鎾斁涓昏璇█锛屽崟澹伴亾鎴栫珛浣撳０銆傚彧鏈?`V4L2_TUNER_ANALOG_TV` 璋冭皭鍣ㄦ敮鎸佹妯″紡銆?    - - `V4L2_TUNER_MODE_LANG2`
      - 2
      - 鎾斁娆¤璇█锛屽崟澹伴亾銆傚綋璋冭皭鍣ㄦ湭鎺ユ敹鍒板弻璇煶棰戞垨 SAP锛屾垨鍏舵帴鏀朵笉鍙楁敮鎸佹椂锛岄┍鍔ㄥ簲鍥為€€鍒板崟澹伴亾鎴栫珛浣撳０妯″紡銆傚彧鏈?`V4L2_TUNER_ANALOG_TV` 璋冭皭鍣ㄦ敮鎸佹妯″紡銆?    - - `V4L2_TUNER_MODE_SAP`
      - 2
      - 鎾斁杈呭姪闊抽鑺傜洰銆傚綋璋冭皭鍣ㄦ湭鎺ユ敹鍒板弻璇煶棰戞垨 SAP锛屾垨鍏舵帴鏀朵笉鍙楁敮鎸佹椂锛岄┍鍔ㄥ簲鍥為€€鍒板崟澹伴亾鎴栫珛浣撳０妯″紡銆傚彧鏈?`V4L2_TUNER_ANALOG_TV` 璋冭皭鍣ㄦ敮鎸佹妯″紡銆?
	.. note:: `V4L2_TUNER_MODE_LANG2` 鍜?`V4L2_TUNER_MODE_SAP` 鏄悓涔夎瘝銆?    - - `V4L2_TUNER_MODE_LANG1_LANG2`
      - 4
      - 鍦ㄥ乏澹伴亾鎾斁涓昏璇█锛屽湪鍙冲０閬撴挱鏀炬瑕佽瑷€銆傚綋璋冭皭鍣ㄦ湭鎺ユ敹鍒板弻璇煶棰戞垨 SAP 鏃讹紝瀹冨簲鍥為€€鍒?`MODE_LANG1` 鎴?`MODE_MONO`銆傚彧鏈?`V4L2_TUNER_ANALOG_TV` 璋冭皭鍣ㄦ敮鎸佹妯″紡銆?

    \scriptsize



    :header-rows:  2
    :stub-columns: 0
    :widths: 7 7 14 14 14 14

    - -
      - `4` 鎵€閫?`V4L2_TUNER_MODE_`
    - - 鎺ユ敹鍒扮殑 `V4L2_TUNER_SUB_`
      - `MONO`
      - `STEREO`
      - `LANG1`
      - `LANG2 = SAP`
      - `LANG1_LANG2`\ [#f1]_
    - - `MONO`
      - 鍗曞０閬?      - 鍗曞０閬?鍗曞０閬?      - 鍗曞０閬?      - 鍗曞０閬?      - 鍗曞０閬?鍗曞０閬?    - - `MONO | SAP`
      - 鍗曞０閬?      - 鍗曞０閬?鍗曞０閬?      - 鍗曞０閬?      - SAP
      - 鍗曞０閬?SAP锛堜紭鍏堬級鎴栧崟澹伴亾/鍗曞０閬?    - - `STEREO`
      - L+R
      - L/R
      - 绔嬩綋澹?L/R锛堜紭鍏堬級鎴栧崟澹伴亾 L+R
      - 绔嬩綋澹?L/R锛堜紭鍏堬級鎴栧崟澹伴亾 L+R
      - L/R锛堜紭鍏堬級鎴?L+R/L+R
    - - `STEREO | SAP`
      - L+R
      - L/R
      - 绔嬩綋澹?L/R锛堜紭鍏堬級鎴栧崟澹伴亾 L+R
      - SAP
      - L+R/SAP锛堜紭鍏堬級鎴?L/R 鎴?L+R/L+R
    - - `LANG1 | LANG2`
      - 璇█ 1
      - Lang1/Lang2锛堝凡寮冪敤\ [#f2]_锛夋垨 Lang1/Lang1
      - 璇█ 1
      - 璇█ 2
      - Lang1/Lang2锛堜紭鍏堬級鎴?Lang1/Lang1


    \normalsize

## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 涓€绔犱腑鎻忚堪銆?
EINVAL
    struct `v4l2_tuner` 鐨?`index` 瓒婄晫銆?
   璇ユā寮忔槸鍦?Linux 2.6.17 涓姞鍏ョ殑锛岃緝鏃х殑椹卞姩鍙兘涓嶆敮鎸併€?
   鍦?`MODE_STEREO` 涓挱鏀句袱绉嶈瑷€鐨勫仛娉曞凡琚純鐢ㄣ€傚皢鏉ラ┍鍔ㄥ湪姝ゆā寮忎笅搴斿彧浜х敓涓昏璇█銆傚簲鐢ㄧ▼搴忓簲褰撹姹?`MODE_LANG1_LANG2` 浠ュ綍鍒朵袱绉嶈瑷€鎴栫珛浣撳０淇″彿銆?