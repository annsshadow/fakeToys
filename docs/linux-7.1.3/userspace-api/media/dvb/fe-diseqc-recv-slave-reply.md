######## ioctl FE_DISEQC_RECV_SLAVE_REPLY


## 鍚嶇О


FE_DISEQC_RECV_SLAVE_REPLY - 鎺ユ敹鏉ヨ嚜 DiSEqC 2.0 鍛戒护鐨勫洖澶?

## 鎽樿



`int ioctl(int fd, FE_DISEQC_RECV_SLAVE_REPLY, struct dvb_diseqc_slave_reply *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜缁撴瀯浣?`dvb_diseqc_slave_reply` 鐨勬寚閽堛€?

## 璇存槑


鎺ユ敹鏉ヨ嚜 DiSEqC 2.0 鍛戒护鐨勫洖澶嶃€?

鎺ユ敹鍒扮殑娑堟伅瀛樺偍鍦?`argp` 鎸囧悜鐨勭紦鍐插尯涓€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
