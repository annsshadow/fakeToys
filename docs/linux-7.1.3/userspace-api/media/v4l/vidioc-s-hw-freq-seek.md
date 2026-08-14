######## ioctl VIDIOC_S_HW_FREQ_SEEK


## 鍚嶇О


VIDIOC_S_HW_FREQ_SEEK - 鎵ц纭欢棰戠巼鎼滅储

## 姒傝


`int ioctl(int fd, VIDIOC_S_HW_FREQ_SEEK, struct v4l2_hw_freq_seek *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜 struct `v4l2_hw_freq_seek` 鐨勬寚閽堛€?

## 鎻忚堪


浠庡綋鍓嶉鐜囧紑濮嬭繘琛岀‖浠堕鐜囨悳绱€備负姝わ紝搴旂敤绋嬪簭鍒濆鍖?`tuner`銆乣type`銆乣seek_upward`銆乣wrap_around`銆乣spacing`銆乣rangelow` 鍜?`rangehigh` 瀛楁锛屽皢 `reserved` 鏁扮粍娓呴浂锛屽苟浣跨敤鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 `VIDIOC_S_HW_FREQ_SEEK` ioctl銆?

`rangelow` 鍜?`rangehigh` 瀛楁鍙互璁剧疆涓洪潪榛樿鍊硷紝浠ュ憡鐭ラ┍鍔ㄦ悳绱㈢壒瀹氶娈点€傚鏋?struct `v4l2_tuner` 鐨?`capability` 瀛楁璁剧疆浜?`V4L2_TUNER_CAP_HWSEEK_PROG_LIM` 鏍囧織锛岃繖浜涘€煎繀椤昏惤鍦?VIDIOC_ENUM_FREQ_BANDS 杩斿洖鐨勬煇涓娈典箣鍐呫€傚鏋滄湭璁剧疆 `V4L2_TUNER_CAP_HWSEEK_PROG_LIM` 鏍囧織锛屽垯杩欎簺鍊煎繀椤荤簿纭尮閰?VIDIOC_ENUM_FREQ_BANDS 杩斿洖鐨勬煇涓娈点€傚鏋滆皟璋愬櫒鐨勫綋鍓嶉鐜囦笉鍦ㄦ墍閫夐娈靛唴锛屽湪寮€濮嬫悳绱箣鍓嶅畠灏嗚闄愬埗锛坈lamp锛夊埌璇ラ娈靛唴銆?

濡傛灉杩斿洖閿欒锛屽垯灏嗘仮澶嶅師濮嬮鐜囥€?

濡傛灉璁剧疆浜?`V4L2_CAP_HW_FREQ_SEEK` 鑳藉姏锛屽垯鏀寔姝?ioctl銆?

濡傛灉姝?ioctl 浠庨潪闃诲鏂囦欢鍙ユ焺璋冪敤锛屽垯杩斿洖 `EAGAIN` 閿欒鐮侊紝涓斾笉杩涜鎼滅储銆?




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `tuner`
      - 璋冭皭鍣ㄧ储寮曞彿銆傝繖涓?struct `v4l2_input` 鐨?`tuner` 瀛楁浠ュ強 struct `v4l2_tuner` 鐨?`index` 瀛楁涓殑鍊肩浉鍚屻€?
    - - __u32
      - `type`
      - 璋冭皭鍣ㄧ被鍨嬨€傝繖涓?struct `v4l2_tuner` 鐨?`type` 瀛楁涓殑鍊肩浉鍚屻€傝鍙傝 `v4l2_tuner_type`
    - - __u32
      - `seek_upward`
      - 濡傛灉闈為浂锛屽垯浠庡綋鍓嶉鐜囧悜涓婃悳绱紝鍚﹀垯鍚戜笅鎼滅储銆?
    - - __u32
      - `wrap_around`
      - 濡傛灉闈為浂锛屽湪鍒拌揪棰戠巼鑼冨洿鏈鏃跺洖缁曪紝鍚﹀垯鍋滄鎼滅储銆俿truct `v4l2_tuner` 鐨?`capability` 瀛楁浼氬憡璇変綘纭欢鏀寔浠€涔堛€?
    - - __u32
      - `spacing`
      - 濡傛灉闈為浂锛屽畾涔夌‖浠舵悳绱㈠垎杈ㄧ巼锛堜互 Hz 涓哄崟浣嶏級銆傞┍鍔ㄩ€夋嫨璁惧鏀寔鐨勬渶鎺ヨ繎鐨勫€笺€傚鏋?spacing 涓洪浂锛屽垯浣跨敤鍚堢悊鐨勯粯璁ゅ€笺€?
    - - __u32
      - `rangelow`
      - 濡傛灉闈為浂锛岃鎼滅储棰戞鐨勪互 62.5 kHz 涓哄崟浣嶇殑 tunable 鏈€浣庨鐜囷紱濡傛灉 struct `v4l2_tuner` 鐨?`capability` 瀛楁璁剧疆浜?`V4L2_TUNER_CAP_LOW` 鏍囧織锛屽垯浠?62.5 Hz 涓哄崟浣嶏紱濡傛灉 struct `v4l2_tuner` 鐨?`capability` 瀛楁璁剧疆浜?`V4L2_TUNER_CAP_1HZ` 鏍囧織锛屽垯浠?1 Hz 涓哄崟浣嶃€傚鏋?`rangelow` 涓洪浂锛屽垯浣跨敤鍚堢悊鐨勯粯璁ゅ€笺€?
    - - __u32
      - `rangehigh`
      - 濡傛灉闈為浂锛岃鎼滅储棰戞鐨勪互 62.5 kHz 涓哄崟浣嶇殑 tunable 鏈€楂橀鐜囷紱濡傛灉 struct `v4l2_tuner` 鐨?`capability` 瀛楁璁剧疆浜?`V4L2_TUNER_CAP_LOW` 鏍囧織锛屽垯浠?62.5 Hz 涓哄崟浣嶏紱濡傛灉 struct `v4l2_tuner` 鐨?`capability` 瀛楁璁剧疆浜?`V4L2_TUNER_CAP_1HZ` 鏍囧織锛屽垯浠?1 Hz 涓哄崟浣嶃€傚鏋?`rangehigh` 涓洪浂锛屽垯浣跨敤鍚堢悊鐨勯粯璁ゅ€笺€?
    - - __u32
      - `reserved`\ [^5^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忓繀椤诲皢鏁扮粍璁剧疆涓洪浂銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

EINVAL
    `tuner` 绱㈠紩瓒婄晫锛宍wrap_around` 鍊间笉鍙楁敮鎸侊紝鎴?`type`銆乣rangelow` 鎴?`rangehigh` 瀛楁涓殑鏌愪釜鍊兼湁璇€?

EAGAIN
    灏濊瘯浠ラ潪闃诲妯″紡璋冪敤 `VIDIOC_S_HW_FREQ_SEEK`銆?

ENODATA
    纭欢鎼滅储鏈壘鍒颁换浣曢閬撱€?

EBUSY
    鍙︿竴涓‖浠舵悳绱㈠凡鍦ㄨ繘琛屼腑銆?
