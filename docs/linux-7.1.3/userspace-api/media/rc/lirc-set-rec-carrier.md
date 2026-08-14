######## ioctl LIRC_SET_REC_CARRIER


## 鍚嶇О


LIRC_SET_REC_CARRIER - 璁剧疆鐢ㄤ簬璋冨埗绾㈠鎺ユ敹鐨勮浇娉㈤鐜囥€?

## 鎽樿



`int ioctl(int fd, LIRC_SET_REC_CARRIER, __u32 *frequency)`

## 鍙傛暟


`fd`
    鐢?open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`frequency`
    璋冨埗 PWM 鏁版嵁鐨勮浇娉㈤鐜囷紝鍗曚綅涓?Hz銆?

## 璇存槑


璁剧疆鐢ㄤ簬璋冨埗绾㈠ PWM 鑴夊啿涓庨棿闅旓紙spaces锛夌殑鎺ユ敹杞芥尝銆?

   鑻ヤ笌 LIRC_SET_REC_CARRIER_RANGE 涓€鍚岃皟鐢紝璇?ioctl 璁剧疆璁惧鑳藉璇嗗埆鐨勪笂闄愰鐜囥€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
