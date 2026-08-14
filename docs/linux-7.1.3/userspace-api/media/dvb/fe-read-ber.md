######## FE_READ_BER


## 鍚嶇О


FE_READ_BER


## 鎽樿



`int ioctl(int fd, FE_READ_BER, uint32_t *ber)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`ber`
    璇爜鐜囪瀛樺叆 \*ber銆?

## 璇存槑


璇?ioctl 璋冪敤杩斿洖鍓嶇褰撳墠鎺ユ敹/瑙ｈ皟淇″彿鐨勮鐮佺巼銆傚浜庤鍛戒护锛屽璁惧鐨勫彧璇昏闂嵆宸茶冻澶熴€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
