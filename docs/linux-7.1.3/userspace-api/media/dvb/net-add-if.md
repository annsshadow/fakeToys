


######## ioctl NET_ADD_IF


## 鍚嶇О


NET_ADD_IF - 涓虹粰瀹氱殑鍖?ID 鍒涘缓鏂扮殑缃戠粶鎺ュ彛銆?
## 姒傝



`int ioctl(int fd, NET_ADD_IF, struct dvb_net_if *net_if)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`net_if`
    鎸囧悜 struct `dvb_net_if` 鐨勬寚閽?
## 鎻忚堪


NET_ADD_IF ioctl 绯荤粺璋冪敤閫夋嫨鍖呭惈 TCP/IP 娴侀噺鐨勫寘 ID (PID)銆佽浣跨敤鐨勫皝瑁?绫诲瀷锛圡PE 鎴?ULE锛変互鍙婅鍒涘缓鐨勬柊鎺ュ彛鐨勬帴鍙ｅ彿銆傚綋绯荤粺璋冪敤鎴愬姛杩斿洖鏃讹紝浼?鍒涘缓涓€涓柊鐨勮櫄鎷熺綉缁滄帴鍙ｃ€?
**struct `dvb_net_if`**
锛歩fnum 瀛楁灏嗚濉厖涓烘墍鍒涘缓鎺ュ彛鐨勭紪鍙枫€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽苟濉厖 `ca_slot_info`銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
閫氱敤閿欒鐮佺殑鎻忚堪瑙侀€氱敤閿欒鐮?<gen-errors> 绔犺妭銆?