######## ioctl LIRC_GET_REC_RESOLUTION


## 鍚嶇О


LIRC_GET_REC_RESOLUTION - 鑾峰彇鎺ユ敹鍒嗚鲸鐜囩殑鍊硷紝鍗曚綅涓哄井绉掋€?

## 姒傝


`int ioctl(int fd, LIRC_GET_REC_RESOLUTION, __u32 *microseconds)`

## 鍙傛暟


`fd`
    open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`microseconds`
    鍒嗚鲸鐜囷紝鍗曚綅涓哄井绉掋€?

## 鎻忚堪


閮ㄥ垎鎺ユ敹鍣ㄥ叿鏈夌敱鍐呴儴閲囨牱鐜囨垨鏁版嵁鏍煎紡闄愬埗鍐冲畾鐨勬渶澶у垎杈ㄧ巼銆備緥濡傦紝
淇″彿閫氬父鍙兘浠?50 寰鐨勬闀夸笂鎶ャ€?

鏈?ioctl 杩斿洖鍏锋湁璇ュ垎杈ㄧ巼鐨勬暣鏁板€硷紝鍙 lircd 绛夌敤鎴风┖闂村簲鐢ㄧ▼搴?
鐢ㄤ簬鑷姩璋冩暣瀹瑰樊锛坱olerance锛夊€笺€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆?
閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
