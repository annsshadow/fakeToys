
######## GPIO_V2_LINE_SET_VALUES_IOCTL


## 鍚嶇О


GPIO_V2_LINE_SET_VALUES_IOCTL - 璁剧疆琚姹傝緭鍑虹嚎鐨勬暟鍊笺€?
## 姒傝


`int ioctl(int req_fd, GPIO_V2_LINE_SET_VALUES_IOCTL, struct gpio_v2_line_values *values)`

## 鍙傛暟


`req_fd`
    GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮锛岀敱
    gpio-v2-get-line-ioctl.rst 鍦?`request.fd<gpio_v2_line_request>` 涓繑鍥炪€?
`values`
    瑕佽缃殑 `line_values<gpio_v2_line_values>`锛屽叾涓?`mask` 璁句负鎸囩ず瑕佽缃?    鐨勮璇锋眰绾跨殑瀛愰泦锛宍bits` 璁句负鎸囩ず鏂板€笺€?
## 鎻忚堪


璁剧疆琚姹傝緭鍑虹嚎鐨勬暟鍊笺€?
璁剧疆鐨勬暟鍊兼槸閫昏緫鍊硷紝琛ㄧず绾胯矾鏄縺娲昏繕鏄潪婵€娲汇€俙GPIO_V2_LINE_FLAG_ACTIVE_LOW`
鏍囧織鎺у埗閫昏緫鍊硷紙婵€娲?闈炴縺娲伙級涓庣墿鐞嗗€硷紙楂?浣庯級涔嬮棿鐨勬槧灏勩€傝嫢鏈缃?`GPIO_V2_LINE_FLAG_ACTIVE_LOW`锛屽垯婵€娲讳负楂樸€侀潪婵€娲讳负浣庛€傝嫢璁剧疆浜?`GPIO_V2_LINE_FLAG_ACTIVE_LOW`锛屽垯婵€娲讳负浣庛€侀潪婵€娲讳负楂樸€?
鍙兘璁剧疆杈撳嚭绾跨殑鏁板€笺€?灏濊瘯璁剧疆杈撳叆绾跨殑鏁板€兼槸涓€涓敊璇紙**EPERM**锛夈€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傚父瑙侀敊璇爜鍦?error-codes.rst 涓鏄庛€?