


######## cec ioctl()


## 鍚嶇О


cec-ioctl - 鎺у埗 cec 璁惧

## 姒傝


    #include <sys/ioctl.h>

`int ioctl(int fd, int request, void *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`request`
    鍦?cec.h 澶存枃浠朵腑瀹氫箟鐨?CEC ioctl 璇锋眰鐮侊紝渚嬪
    CEC_ADAP_G_CAPS <CEC_ADAP_G_CAPS>銆?
`argp`
    鎸囧悜璇锋眰鐗瑰畾缁撴瀯鐨勬寚閽堛€?
## 鎻忚堪


`ioctl()` 鍑芥暟鎿嶇旱 cec 璁惧鍙傛暟銆傚弬鏁?`fd` 蹇呴』鏄竴涓凡鎵撳紑鐨勬枃浠舵弿杩扮銆?
ioctl `request` 鐮佹寚瀹氳璋冪敤鐨?cec 鍑芥暟銆傚叾涓紪鐮佷簡鍙傛暟鏄緭鍏ャ€佽緭鍑鸿繕鏄?璇诲啓鍙傛暟锛屼互鍙婂弬鏁?`argp` 鐨勫ぇ灏忥紙瀛楄妭鏁帮級銆?
鎸囧畾 cec ioctl 璇锋眰鍙婂叾鍙傛暟鐨勫畯涓庣粨鏋勫畾涔変綅浜?cec.h 澶存枃浠朵腑銆傛墍鏈?cec
ioctl 璇锋眰鍙婂叾鍚勮嚜鐨勫嚱鏁颁笌鍙傛暟鍦?cec-user-func 涓鏄庛€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
璇锋眰鐗瑰畾鐨勯敊璇爜鍦ㄥ悇璇锋眰鐨勬弿杩颁腑鍒楀嚭銆?
褰撻噰鐢ㄨ緭鍑烘垨璇诲啓鍙傛暟鐨?ioctl 澶辫触鏃讹紝璇ュ弬鏁颁繚鎸佷笉鍙樸€?