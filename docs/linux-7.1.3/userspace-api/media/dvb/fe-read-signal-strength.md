######## FE_READ_SIGNAL_STRENGTH


## 鍚嶇О


FE_READ_SIGNAL_STRENGTH


## 鎽樿



`int ioctl(int fd, FE_READ_SIGNAL_STRENGTH, uint16_t *strength)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`strength`
    淇″彿寮哄害鍊艰瀛樺叆 \*strength銆?

## 璇存槑


璇?ioctl 璋冪敤杩斿洖鍓嶇褰撳墠鎺ユ敹淇″彿鐨勪俊鍙峰己搴﹀€笺€傚浜庤鍛戒护锛屽璁惧鐨勫彧璇昏闂嵆宸茶冻澶熴€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
