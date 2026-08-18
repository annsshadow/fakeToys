


######## ioctl VIDIOC_G_FREQUENCY, VIDIOC_S_FREQUENCY


## 鍚嶇О


VIDIOC_G_FREQUENCY - VIDIOC_S_FREQUENCY - 鑾峰彇鎴栬缃皟璋愬櫒锛坱uner锛夋垨璋冨埗鍣紙modulator锛夌殑鏃犵嚎鐢甸鐜?
## 姒傝



`int ioctl(int fd, VIDIOC_G_FREQUENCY, struct v4l2_frequency *argp)`


`int ioctl(int fd, VIDIOC_S_FREQUENCY, const struct v4l2_frequency *argp)`

## 鍙傛暟



`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜缁撴瀯浣?`v4l2_frequency` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佽幏鍙栧綋鍓嶇殑璋冭皭鍣ㄦ垨璋冨埗鍣ㄦ棤绾跨數棰戠巼锛屽簲鐢ㄧ▼搴忓皢缁撴瀯浣?`v4l2_frequency` 鐨?`tuner` 瀛楁璁句负瀵瑰簲鐨勮皟璋愬櫒鎴栬皟鍒跺櫒缂栧彿锛堝彧鏈夎緭鍏ヨ澶囨墠鏈夎皟璋愬櫒锛屽彧鏈夎緭鍑鸿澶囨墠鏈夎皟鍒跺櫒锛夛紝灏?`reserved` 鏁扮粍娓呴浂锛岀劧鍚庝互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl銆傞┍鍔ㄥ皢褰撳墠棰戠巼瀛樺叆 `frequency` 瀛楁銆?
瑕佹洿鏀瑰綋鍓嶇殑璋冭皭鍣ㄦ垨璋冨埗鍣ㄦ棤绾跨數棰戠巼锛屽簲鐢ㄧ▼搴忓垵濮嬪寲缁撴瀯浣?`v4l2_frequency` 鐨?`tuner`銆乣type` 鍜?`frequency` 瀛楁浠ュ強 `reserved` 鏁扮粍锛岀劧鍚庝互鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl銆傚綋鎵€璇锋眰鐨勯鐜囦笉鍙疄鐜版椂锛岄┍鍔ㄤ細鍙栨渶鎺ヨ繎鐨勫彲琛屽€笺€備笉杩?VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> 鏄竴涓彧鍐?ioctl锛屽畠骞朵笉杩斿洖瀹為檯鐨勬柊棰戠巼銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `tuner`
      - 璋冭皭鍣ㄦ垨璋冨埗鍣ㄧ储寮曠紪鍙枫€傝鍊间笌缁撴瀯浣?`v4l2_input` 鐨?`tuner` 瀛楁銆佺粨鏋勪綋 `v4l2_tuner` 鐨?`index` 瀛楁锛屾垨缁撴瀯浣?`v4l2_output` 鐨?`modulator` 瀛楁銆佺粨鏋勪綋 `v4l2_modulator` 鐨?`index` 瀛楁鐩稿悓銆?    - - __u32
      - `type`
      - 璋冭皭鍣ㄧ被鍨嬨€傝鍊间笌缁撴瀯浣?`v4l2_tuner` 鐨?`type` 瀛楁鐩稿悓銆傚浜?`/dev/radioX` 璁惧鑺傜偣锛岃绫诲瀷蹇呴』璁句负 `V4L2_TUNER_RADIO`锛涘浜庢墍鏈夊叾浠栬妭鐐瑰垯璁句负 `V4L2_TUNER_ANALOG_TV`銆傚璋冨埗鍣ㄥ簲璁句负 `V4L2_TUNER_RADIO`锛堢洰鍓嶄粎鏀寔鏃犵嚎鐢佃皟鍒跺櫒锛夈€傚弬瑙?`v4l2_tuner_type`
    - - __u32
      - `frequency`
      - 璋冭皭棰戠巼锛屽崟浣嶄负 62.5 kHz锛涜嫢璁剧疆浜嗙粨鏋勪綋 `v4l2_tuner` 鎴栫粨鏋勪綋 `v4l2_modulator` 鐨?`capability` 鏍囧織 `V4L2_TUNER_CAP_LOW`锛屽垯鍗曚綅涓?62.5 Hz銆傚綋璁剧疆浜?`capability` 鏍囧織 `V4L2_TUNER_CAP_1HZ` 鏃讹紝浣跨敤 1 Hz 涓哄崟浣嶃€?    - - __u32
      - `reserved`\ [^8^]
      - 淇濈暀浠ュ灏嗘潵鎵╁睍銆傞┍鍔ㄥ拰搴旂敤绋嬪簭閮藉繀椤诲皢璇ユ暟缁勭疆闆躲€?
## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 涓€绔犱腑鎻忚堪銆?
EINVAL
    `tuner` 绱㈠紩瓒婄晫锛屾垨 `type` 瀛楁涓殑鍊奸敊璇€?
EBUSY
    纭欢鎼滅储锛坰eek锛夋鍦ㄨ繘琛屼腑銆?