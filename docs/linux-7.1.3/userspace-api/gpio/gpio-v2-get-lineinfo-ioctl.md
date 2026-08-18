


######## GPIO_V2_GET_LINEINFO_IOCTL


## 鍚嶇О


GPIO_V2_GET_LINEINFO_IOCTL - 鑾峰彇鏌愭潯绾跨殑鍏紑鍙敤淇℃伅銆?
## 姒傝



`int ioctl(int chip_fd, GPIO_V2_GET_LINEINFO_IOCTL, struct gpio_v2_line_info *info)`

## 鍙傛暟


`chip_fd`
    鐢?`open()` 杩斿洖鐨?GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮銆?
`info`
    瑕佸～鍏呯殑 `line_info<gpio_v2_line_info>`锛屽叾涓?`offset` 瀛楁
    璁剧疆涓烘寚绀鸿鏀堕泦鐨勭嚎璺€?
## 鎻忚堪


鑾峰彇鏌愭潯绾跨殑鍏紑鍙敤淇℃伅銆?
鏃犺璇ョ嚎璺槸鍚︽鍦ㄤ娇鐢紝姝や俊鎭兘鍙敤銆?
    绾胯矾淇℃伅涓嶅寘鎷嚎璺€笺€?
    蹇呴』浣跨敤 gpio-v2-get-line-ioctl.rst 璇锋眰璇ョ嚎璺墠鑳借闂叾鍊笺€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽苟濉厖 `info` 鐨勮姱鐗囦俊鎭€?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傚父瑙侀敊璇爜鍦?error-codes.rst
涓弿杩般€?