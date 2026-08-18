## DMX_ADD_PID


### Name


DMX_ADD_PID

### Synopsis



`int ioctl(fd, DMX_ADD_PID, __u16 *pid)`

### Arguments


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`pid`
   瑕佽繃婊ょ殑 PID 缂栧彿銆?

### Description


璇?ioctl 璋冪敤鍙皢澶氫釜 PID 娣诲姞鍒板厛鍓嶉€氳繃 DMX_SET_PES_FILTER 璁剧疆銆佷笖杈撳嚭绛変簬 `DMX_OUT_TSDEMUX_TAP <dmx_output>` 鐨勪紶杈撴祦杩囨护鍣ㄤ腑銆?

### Return Value


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
