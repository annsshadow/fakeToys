######## ioctl NET_GET_IF


## 鍚嶇О


NET_GET_IF - 璇诲彇閫氳繃 NET_ADD_IF <net> 鍒涘缓鐨勬帴鍙ｇ殑閰嶇疆鏁版嵁銆?
## 姒傝


`int ioctl(int fd, NET_GET_IF, struct dvb_net_if *net_if)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`net_if`
    鎸囧悜 struct `dvb_net_if` 鐨勬寚閽?
## 鎻忚堪


NET_GET_IF ioctl 浣跨敤 struct **`dvb_net_if`** 鐨?: ifnum 瀛楁缁欏畾鐨勬帴鍙ｅ彿锛屽苟鐢ㄨ鎺ュ彛鎵€浣跨敤鐨勫寘 ID 涓庡皝瑁呯被鍨嬪～鍏?struct `dvb_net_if` 鐨勫唴瀹广€傚鏋滃皻鏈€氳繃 NET_ADD_IF <net> 鍒涘缓璇ユ帴鍙ｏ紝瀹冨皢杩斿洖 -1 骞跺皢 `errno` 璁句负 `EINVAL` 閿欒鐮併€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽苟濉厖 `ca_slot_info`銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲璁剧疆 `errno` 鍙橀噺銆?
閫氱敤閿欒鐮佸湪鈥滈€氱敤閿欒鐮?<gen-errors>鈥濈珷鑺備腑鎻忚堪銆?