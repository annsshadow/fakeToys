
######## GPIO_V2_LINE_GET_VALUES_IOCTL


## 鍚嶇О


GPIO_V2_LINE_GET_VALUES_IOCTL - 鑾峰彇鎵€璇锋眰绾胯矾鐨勫€笺€?

## 姒傝


`int ioctl(int req_fd, GPIO_V2_LINE_GET_VALUES_IOCTL, struct gpio_v2_line_values *values)`

## 鍙傛暟


`req_fd`
    GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮锛岀敱 gpio-v2-get-line-ioctl.rst 鍦?
    `request.fd<gpio_v2_line_request>` 涓繑鍥炪€?

`values`
    瑕佽幏鍙栫殑 `line_values<gpio_v2_line_values>`锛屽叾涓?`mask` 琚?
    璁剧疆浠ユ寚绀鸿鑾峰彇鐨勬墍璇锋眰绾胯矾鐨勫瓙闆嗐€?

## 鎻忚堪


鑾峰彇鎵€璇锋眰绾胯矾鐨勫€笺€?

杩斿洖鐨勫€兼槸閫昏緫鍊硷紝琛ㄧず绾胯矾鏄縺娲昏繕鏄潪婵€娲汇€俙GPIO_V2_LINE_FLAG_ACTIVE_LOW`
鏍囧織鎺у埗鐗╃悊鍊硷紙楂?浣庯級涓庨€昏緫鍊硷紙婵€娲?闈炴縺娲伙級涔嬮棿鐨勬槧灏勩€傝嫢鏈缃?
`GPIO_V2_LINE_FLAG_ACTIVE_LOW`锛屽垯楂樼數骞充负婵€娲汇€佷綆鐢靛钩涓洪潪婵€娲伙紱鑻ヨ缃簡
`GPIO_V2_LINE_FLAG_ACTIVE_LOW`锛屽垯浣庣數骞充负婵€娲汇€侀珮鐢靛钩涓洪潪婵€娲汇€?

杈撳叆绾胯矾鍜岃緭鍑虹嚎璺殑鍊煎潎鍙鍙栥€?

瀵逛簬杈撳嚭绾胯矾锛岃繑鍥炵殑鍊煎彇鍐充簬椹卞姩鍜岄厤缃紝鍙兘鏄緭鍑虹紦鍐插尯锛堟渶鍚庤缃殑璇锋眰鍊硷級鎴栬緭鍏ョ紦鍐插尯锛堢嚎璺殑瀹為檯鐢靛钩锛夛紝骞朵笖鏍规嵁纭欢鍜岄厤缃殑涓嶅悓锛屼簩鑰呭彲鑳戒笉涓€鑷淬€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屼笖鐩稿簲鐨?`values.bits<gpio_v2_line_values>` 鍖呭惈璇诲彇鍒扮殑鍊笺€?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傚父瑙侀敊璇爜鍦?error-codes.rst 涓弿杩般€?
