######## FE_GET_FRONTEND


## 鍚嶇О


FE_GET_FRONTEND


## 鎽樿



`int ioctl(int fd, FE_GET_FRONTEND, struct dvb_frontend_parameters *p)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`p`
    鎸囧悜璋冭皭鎿嶄綔鍙傛暟鐨勬寚閽堛€?

## 璇存槑


璇?ioctl 璋冪敤鏌ヨ褰撳墠鐢熸晥鐨勫墠绔弬鏁般€傚浜庤鍛戒护锛屽璁惧鐨勫彧璇昏闂嵆宸茶冻澶熴€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

    :header-rows:  0
    :stub-columns: 0

    - .. row 1

       - `EINVAL`

       - 宸茶揪鍒版敮鎸佺殑鏈€澶х鍙风巼銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
