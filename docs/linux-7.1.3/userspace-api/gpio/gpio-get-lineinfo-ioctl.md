

######## GPIO_GET_LINEINFO_IOCTL


姝?ioctl 鏄?chardev_v1.rst 鐨勪竴閮ㄥ垎锛屽苟宸茶搴熷純
gpio-v2-get-lineinfo-ioctl.rst銆?

## 濮撳悕


GPIO_GET_LINEINFO_IOCTL - 鑾峰彇绾胯矾鐨勫叕寮€淇℃伅銆?

## 姒傝



`int ioctl(int chip_fd, GPIO_GET_LINEINFO_IOCTL, struct gpioline_info *info)`

## 璁虹偣


`chip_fd`
`open()`杩斿洖鐨凣PIO瀛楃璁惧鐨勬枃浠舵弿杩扮銆?

`info`
瑕佸～鍏呯殑`line_info<gpioline_info>`锛屽叾涓?
`offset`瀛楁璁剧疆涓烘寚绀鸿鏀堕泦鐨勮銆?

## 鎻忚堪


鑾峰彇绾胯矾鐨勫叕寮€淇℃伅銆?

璇ヤ俊鎭殑鍙敤鎬т笌绾胯矾鏄惁姝ｅ湪浣跨敤鏃犲叧銆?

绾胯矾淇℃伅涓嶅寘鎷嚎璺€笺€?

蹇呴』浣跨敤 gpio-get-linehandle-ioctl.rst 璇锋眰璇ョ嚎璺垨
gpio-get-lineevent-ioctl.rst 鏉ヨ闂叾鍊笺€?

## 杩斿洖鍊?


鎴愬姛鍚庯紝0 鍜?`info` 灏嗗～鍏呰姱鐗囦俊鎭€?

閿欒 -1 鏃讹紝`errno` 鍙橀噺宸叉纭缃€?
error-codes.rst 涓弿杩颁簡甯歌閿欒浠ｇ爜銆?
