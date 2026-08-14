
######## ioctl VIDIOC_ENUM_FREQ_BANDS


## 鍚嶇О


VIDIOC_ENUM_FREQ_BANDS - 鏋氫妇鏀寔鐨勯娈?
## 璇硶


`int ioctl(int fd, VIDIOC_ENUM_FREQ_BANDS, struct v4l2_frequency_band *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_frequency_band` 鐨勬寚閽堛€?
## 鎻忚堪


鏋氫妇璋冭皭鍣ㄦ垨璋冨埗鍣ㄦ敮鎸佺殑棰戞銆備负姝わ紝搴旂敤绋嬪簭鍒濆鍖?struct `v4l2_frequency_band` 鐨?`tuner`銆乣type` 涓?`index` 瀛楁锛屽苟灏?`reserved` 鏁扮粍娓呴浂锛岀劧鍚庝互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤
VIDIOC_ENUM_FREQ_BANDS ioctl銆?
濡傛灉鐩稿簲璋冭皭鍣?璋冨埗鍣ㄧ殑 `V4L2_TUNER_CAP_FREQ_BANDS` 鑳藉姏琚缃紝鍒欒 ioctl 鍙楁敮鎸併€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2 1 1

    - - __u32
      - `tuner`
      - 璋冭皭鍣ㄦ垨璋冨埗鍣ㄧ储寮曞彿銆傝鍊间笌 struct `v4l2_input` 鐨?`tuner` 瀛楁銆乻truct
	`v4l2_tuner` 鐨?`index` 瀛楁銆乻truct `v4l2_output` 鐨?`modulator` 瀛楁浠ュ強
	struct `v4l2_modulator` 鐨?`index` 瀛楁鐩稿悓銆?    - - __u32
      - `type`
      - 璋冭皭鍣ㄧ被鍨嬨€傝鍊间笌 struct `v4l2_tuner` 鐨?`type` 瀛楁鐩稿悓銆傚浜?`/dev/radioX`
	璁惧鑺傜偣锛岃绫诲瀷蹇呴』璁句负 `V4L2_TUNER_RADIO`锛涘浜庢墍鏈夊叾浠栬妭鐐癸紝璁句负
	`V4L2_TUNER_ANALOG_TV`銆傚浜庤皟鍒跺櫒锛屽皢璇ュ瓧娈佃涓?`V4L2_TUNER_RADIO`锛堢洰鍓嶅彧鏀寔
	鏃犵嚎鐢佃皟鍒跺櫒锛夈€傚弬瑙?`v4l2_tuner_type`
    - - __u32
      - `index`
      - 鏍囪瘑棰戞锛岀敱搴旂敤绋嬪簭璁剧疆銆?    - - __u32
      - `capability`
      - `2` 璇ラ娈靛搴旂殑璋冭皭鍣?璋冨埗鍣ㄨ兘鍔涙爣蹇楋紝鍙傝 tuner-capability銆傛墍閫?	璋冭皭鍣?璋冨埗鍣ㄧ殑鎵€鏈夐娈靛繀椤讳竴鑷村湴璁剧疆 `V4L2_TUNER_CAP_LOW` 鎴?	`V4L2_TUNER_CAP_1HZ` 鑳藉姏銆備篃灏辨槸璇达紝瑕佷箞鎵€鏈夐娈甸兘璁剧疆璇ヨ兘鍔涳紝瑕佷箞閮戒笉璁剧疆銆?    - - __u32
      - `rangelow`
      - `2` 璇ラ娈垫渶浣庡彲璋冭妭棰戠巼锛屽崟浣嶄负 62.5 kHz锛涜嫢璁剧疆浜?`capability` 鏍囧織
	`V4L2_TUNER_CAP_LOW`锛屽垯鍗曚綅涓?62.5 Hz銆傚綋璁剧疆浜?`capability` 鏍囧織
	`V4L2_TUNER_CAP_1HZ` 鏃讹紝浣跨敤 1 Hz 鍗曚綅銆?    - - __u32
      - `rangehigh`
      - `2` 璇ラ娈垫渶楂樺彲璋冭妭棰戠巼锛屽崟浣嶄负 62.5 kHz锛涜嫢璁剧疆浜?`capability` 鏍囧織
	`V4L2_TUNER_CAP_LOW`锛屽垯鍗曚綅涓?62.5 Hz銆傚綋璁剧疆浜?`capability` 鏍囧織
	`V4L2_TUNER_CAP_1HZ` 鏃讹紝浣跨敤 1 Hz 鍗曚綅銆?    - - __u32
      - `modulation`
      - `2` 璇ラ娈垫敮鎸佺殑璋冨埗绯荤粺锛屽弬瑙?band-modulation銆?
```

	  鐩墠姣忎釜棰戞鍙敮鎸佷竴绉嶈皟鍒剁郴缁熴€傝嫢闇€瑕佹敮鎸佸绉嶈皟鍒剁郴缁燂紝杩橀渶瑕佸仛鏇村宸ヤ綔銆?	  濡傛灉浣犻渶瑕佹绫诲姛鑳斤紝璇疯仈绯?linux-media 閭欢鍒楄〃
	  (`https://linuxtv.org/lists.php <https://linuxtv.org/lists.php>`__)銆?    * - __u32
      - ``reserved``\ [9]
      - 涓哄皢鏉ユ墿灞曚繚鐣欍€?
	搴旂敤绋嬪簭涓庨┍鍔ㄩ兘蹇呴』灏嗘暟缁勭疆闆躲€?

```

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_BAND_MODULATION_VSB`
      - 0x02
      - 娈嬬暀杈瑰甫锛圴estigial Sideband锛夎皟鍒讹紝鐢ㄤ簬妯℃嫙鐢佃銆?    - - `V4L2_BAND_MODULATION_FM`
      - 0x04
      - 璋冮锛團requency Modulation锛夛紝甯哥敤浜庢ā鎷熸棤绾跨數銆?    - - `V4L2_BAND_MODULATION_AM`
      - 0x08
      - 璋冨箙锛圓mplitude Modulation锛夛紝甯哥敤浜庢ā鎷熸棤绾跨數銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    `tuner` 鎴?`index` 瓒婄晫锛屾垨 `type` 瀛楁閿欒銆?