######## ioctl LIRC_SET_REC_CARRIER_RANGE


## 鍚嶇О


LIRC_SET_REC_CARRIER_RANGE - 璁剧疆鐢ㄤ簬璋冨埗绾㈠鎺ユ敹鐨勮浇娉㈤鐜囦笅闄愩€?

## 鎽樿



`int ioctl(int fd, LIRC_SET_REC_CARRIER_RANGE, __u32 *frequency)`

## 鍙傛暟


`fd`
    鐢?open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`frequency`
    璋冨埗 PWM 鏁版嵁鐨勮浇娉㈤鐜囷紝鍗曚綅涓?Hz銆?

## 璇存槑


璇?ioctl 璁剧疆绾㈠鎺ユ敹鍣ㄨ兘澶熻瘑鍒殑杞芥尝棰戠巼鐨勪笂闄愯寖鍥淬€?


   瑕佽缃寖鍥达紝鍏堜娇鐢?:ref:`LIRC_SET_REC_CARRIER_RANGE
   <LIRC_SET_REC_CARRIER_RANGE>` 璁剧疆涓嬮檺锛岄殢鍚庡啀璋冪敤
   LIRC_SET_REC_CARRIER <LIRC_SET_REC_CARRIER> 璁剧疆涓婇檺銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
