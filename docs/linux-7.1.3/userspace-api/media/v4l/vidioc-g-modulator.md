


######## ioctl VIDIOC_G_MODULATOR, VIDIOC_S_MODULATOR


## Name


VIDIOC_G_MODULATOR - VIDIOC_S_MODULATOR - 鑾峰彇鎴栬缃皟鍒跺櫒灞炴€?
## Synopsis


`int ioctl(int fd, VIDIOC_G_MODULATOR, struct v4l2_modulator *argp)`


`int ioctl(int fd, VIDIOC_S_MODULATOR, const struct v4l2_modulator *argp)`

## Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_modulator` 鐨勬寚閽堛€?
## Description


瑕佹煡璇㈣皟鍒跺櫒鐨勫睘鎬э紝搴旂敤绋嬪簭鍒濆鍖?struct `v4l2_modulator` 鐨?`index` 瀛楁骞跺皢
`reserved` 鏁扮粍娓呴浂锛岀劧鍚庝互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤
VIDIOC_G_MODULATOR <VIDIOC_G_MODULATOR> ioctl銆傞┍鍔ㄥ～鍐欑粨鏋勭殑鍏朵綑閮ㄥ垎锛屾垨鑰呭綋
index 瓒婄晫鏃惰繑鍥?`EINVAL` 閿欒鐮併€傝鏋氫妇鎵€鏈夎皟鍒跺櫒锛屽簲鐢ㄧ▼搴忓簲浠?index 闆跺紑濮嬶紝
姣忔鍔犱竴锛岀洿鍒伴┍鍔ㄨ繑鍥?EINVAL銆?
璋冨埗鍣ㄦ湁涓や釜鍙啓灞炴€э細涓€涓煶棰戣皟鍒堕泦鍜屽皠棰戙€傝鏀瑰彉琚皟鍒剁殑闊抽瀛愯妭鐩紝搴旂敤绋嬪簭
鍒濆鍖?`index` 鍜?`txsubchans` 瀛楁浠ュ強 `reserved` 鏁扮粍锛岀劧鍚庤皟鐢?VIDIOC_S_MODULATOR <VIDIOC_G_MODULATOR> ioctl銆傚鏋滆姹傛棤娉曡婊¤冻锛岄┍鍔ㄥ彲浠ラ€夋嫨涓嶅悓鐨?闊抽璋冨埗銆傜劧鑰岃繖鏄竴涓彧鍐?ioctl锛屽畠涓嶄細杩斿洖瀹為檯琚€変腑鐨勯煶棰戣皟鍒躲€?
SDR <sdr> 鐗瑰畾鐨勮皟鍒跺櫒绫诲瀷鏄?`V4L2_TUNER_SDR` 鍜?`V4L2_TUNER_RF`銆傚浜?SDR 璁惧
`txsubchans` 瀛楁蹇呴』鍒濆鍖栦负闆躲€傚湪姝や笂涓嬫枃涓紝鏈 鈥渕odulator鈥?鎸?SDR 鍙戝皠鍣ㄣ€?
瑕佹敼鍙樺皠棰戯紝鍙娇鐢?VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2 1 1

    - - __u32
      - `index`
      - 鏍囪瘑璋冨埗鍣紝鐢卞簲鐢ㄧ▼搴忚缃€?    - - __u8
      - `name`\ [^32^]
      - 璋冨埗鍣ㄧ殑鍚嶇О锛屼竴涓互 NUL 缁撳熬鐨?ASCII 瀛楃涓层€?
	姝や俊鎭潰鍚戠敤鎴枫€?    - - __u32
      - `capability`
      - 璋冨埗鍣ㄨ兘鍔涙爣蹇椼€傛瀛楁娌℃湁瀹氫箟鏍囧織锛岀浉搴斿湴浣跨敤 struct `v4l2_tuner` 涓殑
	tuner 鏍囧織銆傞煶棰戞爣蹇楁寚绀虹紪鐮侀煶棰戝瓙鑺傜洰鐨勮兘鍔涖€備緥濡傚畠浠?*涓嶄細**闅忓綋鍓嶈棰?	鏍囧噯鑰屾敼鍙樸€?    - - __u32
      - `rangelow`
      - 鏈€浣庡彲璋冮鐜囷紝鍗曚綅涓?62.5 KHz锛涙垨鑰呭鏋?`capability` 鏍囧織
	`V4L2_TUNER_CAP_LOW` 琚缃紝鍗曚綅涓?62.5 Hz锛涙垨鑰呭鏋?`capability` 鏍囧織
	`V4L2_TUNER_CAP_1HZ` 琚缃紝鍗曚綅涓?1 Hz銆?    - - __u32
      - `rangehigh`
      - 鏈€楂樺彲璋冮鐜囷紝鍗曚綅涓?62.5 KHz锛涙垨鑰呭鏋?`capability` 鏍囧織
	`V4L2_TUNER_CAP_LOW` 琚缃紝鍗曚綅涓?62.5 Hz锛涙垨鑰呭鏋?`capability` 鏍囧織
	`V4L2_TUNER_CAP_1HZ` 琚缃紝鍗曚綅涓?1 Hz銆?    - - __u32
      - `txsubchans`
      - 搴旂敤绋嬪簭閫氳繃姝ゅ瓧娈电‘瀹氶煶棰戝壇杞芥尝搴斿浣曡璋冨埗銆傚畠鍖呭惈涓€缁勫
	modulator-txsubchans 涓墍瀹氫箟鐨勬爣蹇椼€?
```

	   The tuner ``rxsubchans`` flags  are reused, but the
	   semantics are different. Video output devices
	   are assumed to have an analog or PCM audio input with 1-3
	   channels. The ``txsubchans`` flags select one or more channels
	   for modulation, together with some audio subprogram indicator,
	   for example, a stereo pilot tone.
    * - __u32
      - ``type``
      - :cspan:`2` Type of the modulator, see :c:type:`v4l2_tuner_type`.
    * - __u32
      - ``reserved``\ [3]
      - Reserved for future extensions.

	Drivers and applications must set the array to zero.

```




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TUNER_SUB_MONO`
      - 0x0001
      - 灏嗛€氶亾 1 璋冨埗涓哄崟澹伴亾闊抽锛涘綋杈撳叆鏈夋洿澶氶€氶亾鏃讹紝涓洪€氶亾 1 鍜?2 鐨勪笅娣?	锛坉own-mix锛夈€傛鏍囧織涓嶄笌 `V4L2_TUNER_SUB_STEREO` 鎴?	`V4L2_TUNER_SUB_LANG1` 缁勫悎銆?    - - `V4L2_TUNER_SUB_STEREO`
      - 0x0002
      - 灏嗛€氶亾 1 鍜?2 璋冨埗涓虹珛浣撳０闊抽淇″彿鐨勫乏澹伴亾鍜屽彸澹伴亾銆傚綋杈撳叆鍙湁涓€涓€氶亾锛?	鎴栨湁涓や釜閫氶亾涓斿悓鏃惰缃簡 `V4L2_TUNER_SUB_SAP` 鏃讹紝閫氶亾 1 琚紪鐮佷负宸﹀０閬撳拰
	鍙冲０閬撱€傛鏍囧織涓嶄笌 `V4L2_TUNER_SUB_MONO` 鎴?`V4L2_TUNER_SUB_LANG1` 缁勫悎銆?	褰撻┍鍔ㄤ笉鏀寔绔嬩綋澹伴煶棰戞椂锛屽簲鍥為€€鍒板崟澹伴亾銆?    - - `V4L2_TUNER_SUB_LANG1`
      - 0x0008
      - 灏嗛€氶亾 1 鍜?2 璋冨埗涓哄弻璇煶棰戜俊鍙风殑涓昏璇█鍜屾瑕佽瑷€銆傚綋杈撳叆鍙湁涓€涓€氶亾
	鏃讹紝瀹冪敤浜庝袱绉嶈瑷€銆傛棤娉曚粎缂栫爜涓昏鎴栨瑕佽瑷€銆傛鏍囧織涓嶄笌
	`V4L2_TUNER_SUB_MONO`銆乣V4L2_TUNER_SUB_STEREO` 鎴?`V4L2_TUNER_SUB_SAP`
	缁勫悎銆傚鏋滅‖浠朵笉鏀寔鐩稿簲鐨勯煶棰戠煩闃碉紝鎴栬€呭綋鍓嶈棰戞爣鍑嗕笉鍏佽鍙岃闊抽锛屽垯
	VIDIOC_S_MODULATOR <VIDIOC_G_MODULATOR> ioctl 搴旇繑鍥?`EINVAL` 閿欒鐮侊紝椹卞姩搴?	鍥為€€鍒板崟澹伴亾鎴栫珛浣撳０妯″紡銆?    - - `V4L2_TUNER_SUB_LANG2`
      - 0x0004
      - 涓?`V4L2_TUNER_SUB_SAP` 鏁堟灉鐩稿悓銆?    - - `V4L2_TUNER_SUB_SAP`
      - 0x0004
      - 褰撲笌 `V4L2_TUNER_SUB_MONO` 缁勫悎鏃讹紝绗竴涓€氶亾琚紪鐮佷负鍗曞０閬撻煶棰戯紝鏈€鍚庝竴涓?	閫氶亾浣滀负绗簩闊抽鑺傜洰锛圫econd Audio Program锛夈€傚綋杈撳叆鍙湁涓€涓€氶亾鏃讹紝瀹冪敤浜?	鎵€鏈夐煶杞ㄣ€傚綋杈撳叆鏈変笁涓€氶亾鏃讹紝鍗曞０閬撻煶杞ㄦ槸閫氶亾 1 鍜?2 鐨勪笅娣枫€傚綋涓?	`V4L2_TUNER_SUB_STEREO` 缁勫悎鏃讹紝閫氶亾 1 鍜?2 琚紪鐮佷负宸﹀彸绔嬩綋澹伴煶棰戯紝閫氶亾 3
	浣滀负绗簩闊抽鑺傜洰銆傚綋杈撳叆鍙湁涓や釜閫氶亾鏃讹紝绗竴涓缂栫爜涓哄乏澹伴亾鍜屽彸澹伴亾锛岀浜?	涓綔涓?SAP銆傚綋杈撳叆鍙湁涓€涓€氶亾鏃讹紝瀹冪敤浜庢墍鏈夐煶杞ㄣ€傛棤娉曚粎缂栫爜绗簩闊抽鑺傜洰銆?	姝ゆ爣蹇楀繀椤讳笌 `V4L2_TUNER_SUB_MONO` 鎴?`V4L2_TUNER_SUB_STEREO` 缁勫悎銆傚鏋?	纭欢涓嶆敮鎸佺浉搴旂殑闊抽鐭╅樀锛屾垨鑰呭綋鍓嶈棰戞爣鍑嗕笉鍏佽 SAP锛屽垯 VIDIOC_S_MODULATOR
	<VIDIOC_G_MODULATOR> ioctl 搴旇繑鍥?`EINVAL` 閿欒鐮侊紝椹卞姩搴斿洖閫€鍒板崟澹伴亾鎴栫珛浣撳０
	妯″紡銆?    - - `V4L2_TUNER_SUB_RDS`
      - 0x0010
      - 涓?FM 鏀堕煶鏈哄彂灏勫櫒鍚敤 RDS 缂栫爜鍣ㄣ€?
## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞朵笖 `errno` 鍙橀噺琚浉搴斿湴璁剧疆銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
EINVAL
    struct `v4l2_modulator` 鐨?`index` 瓒婄晫銆?