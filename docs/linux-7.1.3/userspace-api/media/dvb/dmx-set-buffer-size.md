## DMX_SET_BUFFER_SIZE


### 鍚嶇О


DMX_SET_BUFFER_SIZE

### 鎽樿



`int ioctl(int fd, DMX_SET_BUFFER_SIZE, unsigned long size)`

### 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`size`
    鏃犵鍙烽暱鏁村瀷鐨?size

### 璇存槑


璇?ioctl 璋冪敤鐢ㄤ簬璁剧疆鐢ㄤ簬杩囨护鏁版嵁鐨勭幆褰㈢紦鍐插尯鐨勫ぇ灏忋€傞粯璁ゅぇ灏忎负涓や釜鏈€澶у昂瀵哥殑娈碉紝鍗冲鏋滀笉璋冪敤姝ゅ嚱鏁帮紝灏嗕娇鐢?`2 * 4096` 瀛楄妭鐨勭紦鍐插尯澶у皬銆?

### 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
