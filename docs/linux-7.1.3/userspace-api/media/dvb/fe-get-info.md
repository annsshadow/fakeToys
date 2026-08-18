######## ioctl FE_GET_INFO


## Name


FE_GET_INFO - 鏌ヨ鏁板瓧鐢佃鍓嶇锛坒ront-end锛夌殑鑳藉姏骞惰繑鍥炴湁鍏冲墠绔殑淇℃伅銆傝璋冪敤浠呴渶瑕佸璁惧鍏锋湁鍙璁块棶鏉冮檺銆?
## Synopsis


`int ioctl(int fd, FE_GET_INFO, struct dvb_frontend_info *argp)`

## Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `dvb_frontend_info` 鐨勬寚閽堛€?
## Description


鎵€鏈夋暟瀛楃數瑙嗭紙Digital TV锛夊墠绔澶囬兘鏀寔 FE_GET_INFO ioctl銆傚畠鐢ㄤ簬璇嗗埆涓庢瑙勮寖鍏煎鐨勫唴鏍歌澶囷紝骞惰幏鍙栨湁鍏抽┍鍔ㄧ▼搴忓拰纭欢鑳藉姏鐨勪俊鎭€傝 ioctl 鎺ユ敹涓€涓寚鍚?dvb_frontend_info 鐨勬寚閽堬紝鐢遍┍鍔ㄧ▼搴忓～鍏呫€傚綋椹卞姩绋嬪簭涓庢瑙勮寖涓嶅吋瀹规椂锛岃 ioctl 杩斿洖閿欒銆?
## frontend capabilities


鑳藉姏锛坈apabilities锛夋弿杩颁簡鍓嶇鑳藉鎵ц鐨勬搷浣溿€傛煇浜涜兘鍔涗粎鍦ㄧ壒瀹氱被鍨嬬殑鍓嶇涓婂彈鏀寔銆?
鍓嶇鑳藉姏鍦?`fe_caps` 涓弿杩般€?
## Return Value


鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?