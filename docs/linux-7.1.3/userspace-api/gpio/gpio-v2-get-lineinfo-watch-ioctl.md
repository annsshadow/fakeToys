

######## GPIO_V2_GET_LINEINFO_WATCH_IOCTL


## 鍚嶇О


GPIO_V2_GET_LINEINFO_WATCH_IOCTL - 鍚敤瀵逛竴鏉＄嚎鐨勮姹傜姸鎬佸拰閰嶇疆淇℃伅鍙樺寲鐨勭洃瑙嗐€?
## 姒傝



`int ioctl(int chip_fd, GPIO_V2_GET_LINEINFO_WATCH_IOCTL, struct gpio_v2_line_info *info)`

## 鍙傛暟


`chip_fd`
    `open()` 杩斿洖鐨?GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮銆?
`info`
    寰呭～鍏呯殑 `line_info<gpio_v2_line_info>` 缁撴瀯浣擄紝鍏朵腑 `offset` 琚涓烘寚绀鸿鐩戣鐨勭嚎

## 鎻忚堪


鍚敤瀵逛竴鏉＄嚎鐨勮姹傜姸鎬佸拰閰嶇疆淇℃伅鍙樺寲鐨勭洃瑙嗐€傜嚎淇℃伅鐨勫彉鍖栧寘鎷竴鏉＄嚎琚姹傘€侀噴鏀炬垨閲嶆柊閰嶇疆銆?
   鐩戣绾夸俊鎭€氬父涓嶆槸蹇呴渶鐨勶紝涓€鑸彧鏈夌郴缁熺洃鎺х粍浠舵墠浼氫娇鐢ㄣ€?
   绾夸俊鎭笉鍖呭惈绾跨殑鍊笺€?   蹇呴』浣跨敤 gpio-v2-get-line-ioctl.rst 鏉ヨ姹傝绾夸互璁块棶鍏跺€硷紝骞朵笖璇ョ嚎璇锋眰鍙互浣跨敤 gpio-v2-line-event-read.rst 鏉ョ洃瑙嗙嚎鐨勪簨浠躲€?
榛樿鎯呭喌涓嬶紝褰?GPIO 鑺墖琚墦寮€鏃舵墍鏈夌嚎閮芥湭琚洃瑙嗐€?
鍙互閫氳繃涓烘瘡鏉＄嚎娣诲姞鐩戣鏉ュ悓鏃剁洃瑙嗗鏉＄嚎銆?
涓€鏃﹁缃簡鐩戣锛屼换浣曠嚎淇℃伅鐨勫彉鍖栭兘浼氱敓鎴愪簨浠讹紝鍙粠 `chip_fd` 璇诲彇锛屽 gpio-v2-lineinfo-changed-read.rst 鎵€杩般€?
鍚戜竴鏉″凡琚洃瑙嗙殑绾挎坊鍔犵洃瑙嗕細鍑洪敊锛?*EBUSY**锛夈€?
鐩戣鏄壒瀹氫簬 `chip_fd` 鐨勶紝骞朵笖鐙珛浜庝娇鐢ㄥ崟鐙殑 `open()` 璋冪敤鎵撳紑鐨勫悓涓€ GPIO 鑺墖涓婄殑鐩戣銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屼笖 `info` 琚～鍏呬负褰撳墠绾夸俊鎭€?
鍑洪敊鏃惰繑鍥?-1锛屼笖 `errno` 鍙橀噺琚€傚綋璁剧疆銆傚父瑙佺殑閿欒鐮佸湪 error-codes.rst 涓弿杩般€?