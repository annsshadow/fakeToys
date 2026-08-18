######## GPIO_GET_CHIPINFO_IOCTL


## 鍚嶇О


GPIO_GET_CHIPINFO_IOCTL - 鑾峰彇鑺墖鍏紑鍙敤鐨勪俊鎭€?

## 鎽樿



`int ioctl(int chip_fd, GPIO_GET_CHIPINFO_IOCTL, struct gpiochip_info *info)`

## 鍙傛暟


`chip_fd`
    GPIO 瀛楃璁惧鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`info`
    寰呭～鍏呯殑 `chip_info<gpiochip_info>`銆?

## 璇存槑


鑾峰彇鐗瑰畾 GPIO 鑺墖鍏紑鍙敤鐨勪俊鎭€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0 骞跺～鍏?`info` 鑺墖淇℃伅銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
甯歌閿欒鐮佸湪 error-codes.rst 涓弿杩般€?
