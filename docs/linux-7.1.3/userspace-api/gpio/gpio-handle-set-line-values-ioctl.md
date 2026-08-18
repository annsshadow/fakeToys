
######## GPIO_HANDLE_SET_LINE_VALUES_IOCTL

    璇?ioctl 鏄?chardev_v1.rst 鐨勪竴閮ㄥ垎锛屽凡琚?    gpio-v2-line-set-values-ioctl.rst 搴熷純銆?
## 鍚嶇О


GPIO_HANDLE_SET_LINE_VALUES_IOCTL - 璁剧疆鎵€鏈夎璇锋眰杈撳嚭绾跨殑鏁板€笺€?
## 姒傝


`int ioctl(int handle_fd, GPIO_HANDLE_SET_LINE_VALUES_IOCTL, struct gpiohandle_data *values)`

## 鍙傛暟


`handle_fd`
    GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮锛岀敱
    gpio-get-linehandle-ioctl.rst 鍦?`request.fd<gpiohandle_request>` 涓繑鍥炪€?
`values`
    瑕佽缃殑 `line_values<gpiohandle_data>`銆?
## 鎻忚堪


璁剧疆鎵€鏈夎璇锋眰杈撳嚭绾跨殑鏁板€笺€?
璁剧疆鐨勬暟鍊兼槸閫昏緫鍊硷紝琛ㄧず绾胯矾鏄縺娲昏繕鏄潪婵€娲汇€俙GPIOHANDLE_REQUEST_ACTIVE_LOW`
鏍囧織鎺у埗閫昏緫鍊硷紙婵€娲?闈炴縺娲伙級涓庣墿鐞嗗€硷紙楂?浣庯級涔嬮棿鐨勬槧灏勩€傝嫢鏈缃?`GPIOHANDLE_REQUEST_ACTIVE_LOW`锛屽垯婵€娲讳负楂樸€侀潪婵€娲讳负浣庛€傝嫢璁剧疆浜?`GPIOHANDLE_REQUEST_ACTIVE_LOW`锛屽垯婵€娲讳负浣庛€侀潪婵€娲讳负楂樸€?
鍙兘璁剧疆杈撳嚭绾跨殑鏁板€笺€?灏濊瘯璁剧疆杈撳叆绾跨殑鏁板€兼槸涓€涓敊璇紙**EPERM**锛夈€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傚父瑙侀敊璇爜鍦?error-codes.rst 涓鏄庛€?