


######## GPIO_GET_LINEEVENT_IOCTL


    This ioctl is part of chardev_v1.rst and is obsoleted by
    gpio-v2-get-line-ioctl.rst.

## 鍚嶇О


GPIO_GET_LINEEVENT_IOCTL - 浠庡唴鏍歌姹備竴鏉″甫杈规部妫€娴嬬殑绾胯矾銆?
## 姒傝



`int ioctl(int chip_fd, GPIO_GET_LINEEVENT_IOCTL, struct gpioevent_request *request)`

## 鍙傛暟


`chip_fd`
    GPIO 瀛楃璁惧鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`request`
    鎸囧畾瑕佽姹傜殑绾胯矾鍙婂叾閰嶇疆鐨?`event_request<gpioevent_request>`銆?
## 鎻忚堪


浠庡唴鏍歌姹備竴鏉″甫杈规部妫€娴嬬殑绾胯矾銆?
鎴愬姛鏃讹紝璇锋眰杩涚▼琚巿浜堝璇ョ嚎璺€肩殑鐙崰璁块棶鏉冮檺锛屽苟鍙湪绾胯矾涓婃娴嬪埌杈规部鏃舵帴鏀朵簨浠讹紝濡?gpio-lineevent-data-read.rst 鎵€杩般€?
绾胯矾鐨勭姸鎬佷繚璇佷繚鎸佷负鎵€璇锋眰鐨勭姸鎬侊紝鐩村埌杩斿洖鐨勬枃浠舵弿杩扮琚叧闂€備竴鏃︽枃浠舵弿杩扮琚叧闂紝浠庣敤鎴风┖闂寸殑瑙掑害鐪嬶紝绾胯矾鐨勭姸鎬佸彉寰椾笉鍙楁帶鍒讹紝骞跺彲鑳芥仮澶嶅埌鍏堕粯璁ょ姸鎬併€?
璇锋眰涓€鏉″凡缁忚浣跨敤鐨勭嚎璺槸涓€涓敊璇紙**EBUSY**锛夈€?
璇锋眰涓€鏉′笉鏀寔涓柇鐨勭嚎璺殑杈规部妫€娴嬫槸涓€涓敊璇紙**ENXIO**锛夈€?
涓?line handle<gpio-get-linehandle-config-support> 涓€鏍凤紝鍋忕疆锛坆ias锛夐厤缃槸灏藉姏鑰屼负鐨勩€?
鍏抽棴 `chip_fd` 瀵瑰凡鏈夌殑绾胯矾浜嬩欢娌℃湁褰卞搷銆?
### 閰嶇疆瑙勫垯


浠ヤ笅閰嶇疆瑙勫垯閫傜敤锛?
绾胯矾浜嬩欢琚綔涓鸿緭鍏ョ璇锋眰锛屽洜姝や笉鑳借缃换浣曚笓鐢ㄤ簬杈撳嚭绾胯矾鐨勬爣蹇楋紝鍗?`GPIOHANDLE_REQUEST_OUTPUT`銆乣GPIOHANDLE_REQUEST_OPEN_DRAIN` 鎴?`GPIOHANDLE_REQUEST_OPEN_SOURCE`銆?
鍙兘璁剧疆涓€涓亸缃爣蹇?`GPIOHANDLE_REQUEST_BIAS_xxx`銆傝嫢鏈缃换浣曞亸缃爣蹇楋紝鍒欏亸缃厤缃笉浼氳鏀瑰彉銆?
杈规部鏍囧織 `GPIOEVENT_REQUEST_RISING_EDGE` 涓?`GPIOEVENT_REQUEST_FALLING_EDGE` 鍙互缁勫悎锛屼互鍚屾椂妫€娴嬩笂鍗囨部涓庝笅闄嶆部銆?
璇锋眰鏃犳晥鐨勯厤缃槸涓€涓敊璇紙**EINVAL**锛夈€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屼笖 `request.fd<gpioevent_request>` 鍖呭惈璇ヨ姹傜殑鏂囦欢鎻忚堪绗︺€?
鍑洪敊鏃惰繑鍥?-1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傚父瑙侀敊璇爜鍦?error-codes.rst 涓弿杩般€?