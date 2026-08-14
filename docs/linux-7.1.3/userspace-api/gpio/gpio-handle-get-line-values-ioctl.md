

######## GPIOHANDLE_GET_LINE_VALUES_IOCTL

    姝?ioctl 鏄?chardev_v1.rst 鐨勪竴閮ㄥ垎锛屽苟宸茶 gpio-v2-line-get-values-ioctl.rst 鍙栦唬銆?
## 鍚嶇О

GPIOHANDLE_GET_LINE_VALUES_IOCTL - 鑾峰彇鎵€鏈夊凡璇锋眰绾跨殑鍊笺€?
## 姒傝

`int ioctl(int handle_fd, GPIOHANDLE_GET_LINE_VALUES_IOCTL, struct gpiohandle_data *values)`

## 鍙傛暟

`handle_fd`
    GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮锛岀敱 gpio-get-linehandle-ioctl.rst 鍦?`request.fd<gpiohandle_request>` 涓繑鍥炪€?
`values`
    寰呭～鍏呯殑 `line_values<gpiohandle_data>`銆?
## 鎻忚堪

鑾峰彇鎵€鏈夊凡璇锋眰绾跨殑鍊笺€?
杩斿洖鐨勫€兼槸閫昏緫鍊硷紝鎸囩ず璇ョ嚎鏄縺娲昏繕鏄潪婵€娲汇€俙GPIOHANDLE_REQUEST_ACTIVE_LOW` 鏍囧織鎺у埗鐗╃悊鍊硷紙楂?浣庯級涓庨€昏緫鍊硷紙婵€娲?闈炴縺娲伙級涔嬮棿鐨勬槧灏勩€傚鏋滄湭璁剧疆 `GPIOHANDLE_REQUEST_ACTIVE_LOW`锛屽垯楂樹负婵€娲汇€佷綆涓洪潪婵€娲汇€傚鏋滆缃簡 `GPIOHANDLE_REQUEST_ACTIVE_LOW`锛屽垯浣庝负婵€娲汇€侀珮涓洪潪婵€娲汇€?
杈撳叆绾垮拰杈撳嚭绾跨殑鍊煎潎鍙璇诲彇銆?
瀵逛簬杈撳嚭绾匡紝杩斿洖鐨勫€煎彇鍐充簬椹卞姩鍜岄厤缃紝鍙兘鏄緭鍑虹紦鍐插尯锛堟渶鍚庤缃殑璇锋眰鍊硷級鎴栬緭鍏ョ紦鍐插尯锛堢嚎鐨勫疄闄呯數骞筹級锛屽苟涓旀牴鎹‖浠跺拰閰嶇疆鐨勪笉鍚岋紝浜岃€呭彲鑳藉瓨鍦ㄥ樊寮傘€?
姝?ioctl 涔熷彲鐢ㄤ簬璇诲彇绾夸簨浠剁殑绾垮€硷紝灏?`event_fd` 鏇挎崲涓?`handle_fd`銆傜敱浜庤繖绉嶆儏鍐典笅鍙姹備簡涓€鏉＄嚎锛屽洜姝?`values` 涓彧杩斿洖涓€涓€笺€?
## 杩斿洖鍊?
鎴愬姛鏃惰繑鍥?0锛屼笖 `values` 琚～鍏呬负璇诲彇鍒扮殑鍊笺€?
鍑洪敊鏃惰繑鍥?-1锛屽苟閫傚綋璁剧疆 `errno` 鍙橀噺銆傚父瑙佺殑閿欒鐮佸湪 error-codes.rst 涓弿杩般€?