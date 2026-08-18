


######## GPIO_GET_LINEINFO_WATCH_IOCTL


    璇?ioctl 鏄?chardev_v1.rst 鐨勪竴閮ㄥ垎锛屽凡琚?    gpio-v2-get-lineinfo-watch-ioctl.rst 搴熷純銆?
## 鍚嶇О


GPIO_GET_LINEINFO_WATCH_IOCTL - 鍚敤瀵逛竴鏉＄嚎鐨勮姹傜姸鎬佸拰閰嶇疆淇℃伅鐨勫彉鏇寸洃瑙嗐€?
## 姒傝


`int ioctl(int chip_fd, GPIO_GET_LINEINFO_WATCH_IOCTL, struct gpioline_info *info)`

## 鍙傛暟


`chip_fd`
    `open()` 杩斿洖鐨?GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮銆?
`info`
    瑕佸～鍏呯殑 `line_info<gpioline_info>` 缁撴瀯浣擄紝鍏朵腑
    `offset` 琚缃负鎸囩ず瑕佺洃瑙嗙殑绾?
## 鎻忚堪


鍚敤瀵逛竴鏉＄嚎鐨勮姹傜姸鎬佸拰閰嶇疆淇℃伅鐨勫彉鏇寸洃瑙嗐€傜嚎淇℃伅鐨勫彉鏇村寘鎷竴鏉＄嚎琚姹傘€侀噴鏀炬垨閲嶆柊閰嶇疆銆?
    鐩戣绾夸俊鎭€氬父骞朵笉鏄繀闇€鐨勶紝涓€鑸彧浼氳绯荤粺鐩戞帶缁勪欢浣跨敤銆?
    绾夸俊鎭笉鍖呭惈绾跨殑鍊硷紙value锛夈€?
    蹇呴』浣跨敤 gpio-get-linehandle-ioctl.rst 鎴?gpio-get-lineevent-ioctl.rst 鏉ヨ姹備竴鏉＄嚎浠ヨ闂叾鍊硷紝骞朵笖鍙互浣跨敤 gpio-lineevent-data-read.rst 閫氳繃绾夸簨浠剁洃瑙嗕竴鏉＄嚎銆?
榛樿鎯呭喌涓嬶紝褰?GPIO 鑺墖琚墦寮€鏃讹紝鎵€鏈夌嚎閮芥湭琚洃瑙嗐€?
鍙互閫氳繃涓烘瘡鏉＄嚎娣诲姞鐩戣鏉ュ悓鏃剁洃瑙嗗鏉＄嚎銆?
涓€鏃﹁缃簡鐩戣锛屼换浣曠嚎淇℃伅鐨勫彉鏇撮兘浼氱敓鎴愪簨浠讹紝鍙互浠?`chip_fd` 璇诲彇锛屽 gpio-lineinfo-changed-read.rst 鎵€杩般€?
鍚戜竴鏉″凡缁忚鐩戣鐨勭嚎娣诲姞鐩戣鏄竴涓敊璇紙**EBUSY**锛夈€?
鐩戣鏄壒瀹氫簬 `chip_fd` 鐨勶紝骞朵笖鐙珛浜庨€氳繃瀵?`open()` 鐨勫崟鐙皟鐢ㄦ墦寮€鐨勫悓涓€涓?GPIO 鑺墖涓婄殑鐩戣銆?
棣栨娣诲姞浜?5.7銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽苟涓?`info` 琚～鍏呬负褰撳墠鐨勭嚎淇℃伅銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?甯歌閿欒鐮佸湪 error-codes.rst 涓弿杩般€?