## DMX_REMOVE_PID


### 鍚嶇О


DMX_REMOVE_PID

### 鎽樿



`int ioctl(fd, DMX_REMOVE_PID, __u16 *pid)`

### 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`pid`
    瑕佺Щ闄ょ殑 PES 杩囨护鍣ㄧ殑 PID銆?

### 璇存槑


褰撲竴涓紶杈撴祦杩囨护鍣ㄤ笂璁剧疆浜嗗涓?PID 鏃讹紝璇?ioctl 璋冪敤鍏佽绉婚櫎鏌愪釜 PID锛屼緥濡備箣鍓嶉€氳繃 DMX_SET_PES_FILTER 鎴?DMX_ADD_PID 鍒涘缓銆佷笖杈撳嚭绛変簬 `DMX_OUT_TSDEMUX_TAP <dmx_output>` 鐨勮繃婊ゅ櫒銆?

### 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
