######## GPIOHANDLE_SET_CONFIG_IOCTL


    璇?ioctl 鏄?chardev_v1.rst 鐨勪竴閮ㄥ垎锛屽凡琚?gpio-v2-line-set-config-ioctl.rst
    鍙栦唬銆?
## 鍚嶇О


GPIOHANDLE_SET_CONFIG_IOCTL - 鏇存柊鍏堝墠璇锋眰鐨勭嚎璺厤缃€?
## 姒傝


`int ioctl(int handle_fd, GPIOHANDLE_SET_CONFIG_IOCTL, struct gpiohandle_config *config)`

## 鍙傛暟


`handle_fd`
    GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮锛岀敱 gpio-get-linehandle-ioctl.rst 鍦?    `request.fd<gpiohandle_request>` 涓繑鍥炪€?
`config`
    瑕佸簲鐢ㄥ埌璇锋眰绾胯矾鐨勬柊鐨?`configuration<gpiohandle_config>`銆?
## 鎻忚堪


鏇存柊鍏堝墠璇锋眰鐨勭嚎璺厤缃紝鑰屼笉閲婃斁绾胯矾鎴栧紩鍏ユ綔鍦ㄧ殑鏁呴殰銆?
璇ラ厤缃簲鐢ㄤ簬鎵€鏈夎姹傜殑绾胯矾銆?
璇锋眰绾胯矾鏃堕€傜敤鐨?gpio-get-linehandle-config-rules 鍜?gpio-get-linehandle-config-support 鍦ㄦ洿鏂扮嚎璺厤缃椂鍚屾牱閫傜敤锛岄檮鍔犻檺鍒舵槸蹇呴』
璁剧疆鏂瑰悜鏍囧織銆傝姹傛棤鏁堥厤缃紙鍖呮嫭鏈缃柟鍚戞爣蹇楋級鏄竴涓敊璇紙**EINVAL**锛夈€?
璇ュ懡浠ょ殑鍔ㄦ満鐢ㄤ緥鏄湪杈撳叆鍜岃緭鍑轰箣闂存敼鍙樺弻鍚戠嚎璺殑鏂瑰悜锛屼絾瀹冧篃鍙洿涓€鑸湴鐢ㄤ簬
灏嗙嚎璺粠涓€涓厤缃姸鎬佹棤缂濈Щ鍔ㄥ埌鍙︿竴涓€?
瑕佷粎鏇存敼杈撳嚭绾胯矾鐨勫€硷紝璇蜂娇鐢?gpio-handle-set-line-values-ioctl.rst銆?
棣栨娣诲姞浜?5.5銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟閫傚綋璁剧疆 `errno` 鍙橀噺銆傚父瑙侀敊璇爜鍦?error-codes.rst 涓弿杩般€?