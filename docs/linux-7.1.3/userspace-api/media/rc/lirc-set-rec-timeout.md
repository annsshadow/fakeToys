######## ioctl LIRC_GET_REC_TIMEOUT and LIRC_SET_REC_TIMEOUT


## 鍚嶇О


LIRC_GET_REC_TIMEOUT/LIRC_SET_REC_TIMEOUT - 鑾峰彇/璁剧疆 IR 闈炴椿鍔ㄨ秴鏃舵椂闂寸殑鏁存暟鍊笺€?
## 姒傝


`int ioctl(int fd, LIRC_GET_REC_TIMEOUT, __u32 *timeout)`


`int ioctl(int fd, LIRC_SET_REC_TIMEOUT, __u32 *timeout)`

## 鍙傛暟


`fd`
    open() 杩斿洖鐨勬枃浠舵弿杩扮銆?
`timeout`
    瓒呮椂鏃堕棿锛屽崟浣嶄负寰銆?
## 鎻忚堪


鑾峰彇骞惰缃?IR 闈炴椿鍔ㄨ秴鏃舵椂闂寸殑鏁存暟鍊笺€?
鑻ョ‖浠舵敮鎸侊紝灏嗗叾璁句负 0 灏嗙鐢ㄦ墍鏈夌‖浠惰秴鏃讹紝骞跺簲灏藉揩涓婃姤鏁版嵁銆傝嫢鏃犳硶璁剧疆绮剧‘鍊硷紝鍒欏簲灏嗕笅涓€涓ぇ浜庣粰瀹氬€肩殑鍙兘鍊煎啓鍏ャ€?
   鏀寔鐨勮秴鏃惰寖鍥寸敱 LIRC_GET_MIN_TIMEOUT 缁欏嚭銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴旇缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪鈥滈€氱敤閿欒鐮?<gen-errors>鈥濈珷鑺備腑鎻忚堪銆?