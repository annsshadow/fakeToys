######## FE_READ_UNCORRECTED_BLOCKS


## 鍚嶇О


FE_READ_UNCORRECTED_BLOCKS


## 鎽樿



`int ioctl(int fd, FE_READ_UNCORRECTED_BLOCKS, uint32_t *ublocks)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`ublocks`
    椹卞姩杩勪粖涓烘鐪嬪埌鐨勬€绘湭鏍℃鍧楁暟銆?

## 璇存槑


璇?ioctl 璋冪敤杩斿洖璁惧椹卞姩鍦ㄥ叾鐢熷懡鍛ㄦ湡鍐呮娴嬪埌鐨勬湭鏍℃鍧楁暟閲忋€備负浜嗚幏寰楁湁鎰忎箟鐨勬祴閲忓€硷紝搴旇绠楀湪鐗瑰畾鏃堕棿闂撮殧鍐呭潡璁℃暟鐨勫閲忋€傚浜庤鍛戒护锛屽璁惧鐨勫彧璇昏闂嵆宸茶冻澶熴€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
