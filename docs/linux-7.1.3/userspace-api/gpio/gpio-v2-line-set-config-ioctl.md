


######## GPIO_V2_LINE_SET_CONFIG_IOCTL


## 鍚嶇О


GPIO_V2_LINE_SET_CONFIG_IOCTL - 鏇存柊鍏堝墠璇锋眰鐨勭嚎璺厤缃€?

## 姒傝



`int ioctl(int req_fd, GPIO_V2_LINE_SET_CONFIG_IOCTL, struct gpio_v2_line_config *config)`

## 鍙傛暟


`req_fd`
    GPIO 瀛楃璁惧鐨勬枃浠舵弿杩扮锛屽
    `request.fd<gpio_v2_line_request>` 涓?gpio-v2-get-line-ioctl.rst 鎵€杩斿洖鐨勯偅鏍枫€?

`config`
    瑕佸簲鐢ㄥ埌
    璇锋眰绾胯矾涓婄殑鏂?`configuration<gpio_v2_line_config>`銆?

## 鎻忚堪


鏇存柊鍏堝墠璇锋眰鐨勭嚎璺厤缃紝鏃犻渶閲婃斁
绾胯矾鎴栧紩鍏ユ綔鍦ㄧ殑鏁呴殰銆?

鏂伴厤缃繀椤讳负鎵€鏈夎姹傜殑绾胯矾鎸囧畾閰嶇疆銆?

璇锋眰绾胯矾鏃堕€傜敤鐨勭浉鍚?gpio-v2-get-line-config-rules 鍜?
gpio-v2-get-line-config-support 鍦ㄦ洿鏂扮嚎璺厤缃椂鍚屾牱閫傜敤锛屽彟鍔?
涓€鏉￠檺鍒讹細蹇呴』璁剧疆鏂瑰悜鏍囧織浠ュ惎鐢ㄩ噸鏂伴厤缃€?

濡傛灉鏌愭潯绾胯矾鍦ㄩ厤缃腑鏈缃柟鍚戞爣蹇楋紝鍒欒
绾胯矾閰嶇疆淇濇寔涓嶅彉銆?

璇ュ懡浠ょ殑涓昏鐢ㄤ緥鏄湪
杈撳叆鍜岃緭鍑轰箣闂存敼鍙樺弻鍚戠嚎璺殑鏂瑰悜锛屼絾涔熷彲鐢ㄤ簬
鍔ㄦ€佹帶鍒惰竟娌挎娴嬶紝鎴栨洿涓€鑸湴璁╃嚎璺湪
涓嶅悓閰嶇疆鐘舵€佷箣闂存棤缂濆垏鎹€?

濡傛灉鍙兂鏀瑰彉杈撳嚭绾胯矾鐨勫€硷紝璇蜂娇鐢?
gpio-v2-line-set-values-ioctl.rst銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
甯歌閿欒鐮佸湪 error-codes.rst 涓弿杩般€?
