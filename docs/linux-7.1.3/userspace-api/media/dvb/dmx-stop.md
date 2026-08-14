
## DMX_STOP


### 鍚嶇О


DMX_STOP

### 姒傝


`int ioctl(int fd, DMX_STOP)`

### 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

### 鎻忚堪


璇?ioctl 璋冪敤鐢ㄤ簬鍋滄閫氳繃 DMX_SET_FILTER 鎴?DMX_SET_PES_FILTER ioctl 璋冪敤
瀹氫箟銆佸苟閫氳繃 DMX_START 鍛戒护鍚姩鐨勫疄闄呰繃婊ゆ搷浣溿€?

### 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪鈥滈€氱敤閿欒鐮佲€濓紙Generic Error Codes锛?gen-errors> 绔犺妭涓弿杩般€?
