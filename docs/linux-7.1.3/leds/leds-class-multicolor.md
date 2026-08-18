
## Linux 涓嬬殑澶氳壊 LED 澶勭悊


## 鎻忚堪

澶氳壊 LED 绫诲皢鍗曡壊 LED 褰掍负涓€缁勶紝骞跺厑璁告帶鍒舵渶缁堝悎鎴愰鑹茬殑涓や釜鏂归潰锛氳壊鐩革紙hue锛夊拰浜害锛坙ightness锛夈€傚墠鑰呴€氳繃 multi_intensity 鏁扮粍鏂囦欢鎺у埗锛屽悗鑰呴€氳繃 brightness 鏂囦欢鎺у埗銆?
## 澶氳壊绫绘帶鍒?
澶氳壊绫讳互鏁扮粍绱㈠紩鐨勬柟寮忓皢棰滆壊褰掍负涓€缁勫苟鎻愪緵鐩稿簲鏂囦欢銆傝繖浜涙枃浠舵槸 led_class 妗嗘灦鍒涘缓鐨?LED 鐖惰妭鐐逛笅鐨勫瓙椤广€俵ed_class 妗嗘灦鐨勬枃妗ｈ鏈枃妗ｇ洰褰曚腑鐨?led-class.rst銆?
姣忎釜褰╄壊 LED 閮戒細鍦?`multi_*` 鏂囦欢涓嬪缓绔嬬储寮曘€傞鑹茬殑椤哄簭鏄换鎰忕殑銆傚彲浠ヨ鍙?`multi_index` 鏂囦欢浠ョ‘瀹氶鑹插悕绉板搴旂殑绱㈠紩鍊笺€?
`multi_index` 鏂囦欢鏄竴涓暟缁勶紝鍖呭惈鍦ㄦ瘡涓?`multi_*` 鏁扮粍鏂囦欢涓畾涔夌殑棰滆壊瀛楃涓插垪琛ㄣ€?
`multi_intensity` 鏄竴涓彲璇诲啓鐨勬暟缁勶紝鐢ㄤ簬璁剧疆鍚勪釜棰滆壊寮哄害銆傚繀椤绘寜椤哄簭鍐欏叆璇ユ暟缁勭殑鎵€鏈夊厓绱狅紝棰滆壊 LED 寮哄害鎵嶄細鏇存柊銆?
## 鐩綍甯冨眬绀轰緥


    root:/sys/class/leds/multicolor:status# ls -lR
    -rw-r--r--    1 root     root          4096 Oct 19 16:16 brightness
    -r--r--r--    1 root     root          4096 Oct 19 16:16 max_brightness
    -r--r--r--    1 root     root          4096 Oct 19 16:16 multi_index
    -rw-r--r--    1 root     root          4096 Oct 19 16:16 multi_intensity

..

## 澶氳壊绫讳寒搴︽帶鍒?
姣忎釜 LED 鐨勪寒搴︾骇鍒牴鎹€滈鑹?LED 寮哄害璁剧疆 梅 鍏ㄥ眬 max_brightness 璁剧疆 脳 璇锋眰鐨勪寒搴︹€濊绠椼€?
`led_brightness = brightness * multi_intensity/max_brightness`

绀轰緥锛?鐢ㄦ埛棣栧厛鍚?multi_intensity 鏂囦欢鍐欏叆鍚勪釜 LED 鐨勪寒搴︾骇鍒紝杩欎簺绾у埆鏄疄鐜版煇涓鑹?LED 缁勭壒瀹氶鑹茶緭鍑烘墍蹇呴渶鐨勩€?
    # cat /sys/class/leds/multicolor:status/multi_index
    green blue red

    # echo 43 226 138 > /sys/class/leds/multicolor:status/multi_intensity

    red -
    	intensity = 138
    	max_brightness = 255
    green -
    	intensity = 43
    	max_brightness = 255
    blue -
    	intensity = 226
    	max_brightness = 255

..

鐢ㄦ埛鍙互閫氳繃鍐欏叆鍏ㄥ眬 'brightness' 鎺у埗椤规潵鎺у埗璇ュ鑹?LED 缁勭殑浜害銆傚亣瀹?max_brightness 涓?255锛岀敤鎴峰彲鑳藉笇鏈涘皢璇ラ鑹茬粍璋冩殫涓€鍗娿€傜敤鎴峰簲鍚戝叏灞€ brightness 鏂囦欢鍐欏叆鍊?128锛岄殢鍚庡啓鍏ユ瘡涓?LED 鐨勫€间細鍩轰簬璇ュ€艰繘琛岃皟鏁淬€?
    # cat /sys/class/leds/multicolor:status/max_brightness
    255
    # echo 128 > /sys/class/leds/multicolor:status/brightness

..

    adjusted_red_value = 128 * 138/255 = 69
    adjusted_green_value = 128 * 43/255 = 21
    adjusted_blue_value = 128 * 226/255 = 113

..

璇诲彇鍏ㄥ眬 brightness 鏂囦欢灏嗚繑鍥炶棰滆壊 LED 缁勭殑褰撳墠浜害鍊笺€?
    # cat /sys/class/leds/multicolor:status/brightness
    128

..
