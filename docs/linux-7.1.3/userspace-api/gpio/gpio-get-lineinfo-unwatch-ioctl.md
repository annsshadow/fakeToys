######## GPIO_GET_LINEINFO_UNWATCH_IOCTL


## 鍚嶇О


GPIO_GET_LINEINFO_UNWATCH_IOCTL - 鍙栨秷瀵规煇涓€绾胯矾鍏惰姹傜姸鎬佸拰閰嶇疆淇℃伅鍙樺寲鐨勭洃瑙嗐€?

## 鎽樿



`int ioctl(int chip_fd, GPIO_GET_LINEINFO_UNWATCH_IOCTL, u32 *offset)`

## 鍙傛暟


`chip_fd`
    GPIO 瀛楃璁惧鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`offset`
    涓嶅啀鐩戣鐨勭嚎璺亸绉婚噺銆?

## 璇存槑


灏嗚绾胯矾浠庢 `chip_fd` 涓婃鍦ㄧ洃瑙嗙殑绾胯矾鍒楄〃涓Щ闄ゃ€?

杩欐槸 gpio-v2-get-lineinfo-watch-ioctl.rst锛坴2锛夊拰
gpio-get-lineinfo-watch-ioctl.rst锛坴1锛夌殑閫嗘搷浣溿€?

瀵逛竴鏉℃湭鐩戣鐨勭嚎璺彇娑堢洃瑙嗘槸涓€涓敊璇紙**EBUSY**锛夈€?

鏈€鍒濇坊鍔犱簬 5.7銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
甯歌閿欒鐮佸湪 error-codes.rst 涓弿杩般€?
