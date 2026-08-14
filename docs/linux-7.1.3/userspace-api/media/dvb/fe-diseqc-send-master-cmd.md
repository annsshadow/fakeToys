######## ioctl FE_DISEQC_SEND_MASTER_CMD


## 鍚嶇О


FE_DISEQC_SEND_MASTER_CMD - 鍙戦€?DiSEqC 鍛戒护

## 鎽樿



`int ioctl(int fd, FE_DISEQC_SEND_MASTER_CMD, struct dvb_diseqc_master_cmd *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜缁撴瀯浣?`dvb_diseqc_master_cmd` 鐨勬寚閽?

## 璇存槑


灏?`dvb_diseqc_master_cmd` 鎸囧悜鐨?DiSEqC 鍛戒护鍙戦€佸埌澶╃嚎瀛愮郴缁熴€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
