######## ioctl LIRC_SET_MEASURE_CARRIER_MODE


## 鍚嶇О


LIRC_SET_MEASURE_CARRIER_MODE - 鍚敤鎴栫鐢ㄦ祴閲忔ā寮?

## 鎽樿



`int ioctl(int fd, LIRC_SET_MEASURE_CARRIER_MODE, __u32 *enable)`

## 鍙傛暟


`fd`
    鐢?open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`enable`
    enable = 1 琛ㄧず鍚敤娴嬮噺妯″紡锛宔nable = 0 琛ㄧず绂佺敤娴嬮噺妯″紡銆?

## 璇存槑


鍚敤鎴栫鐢ㄦ祴閲忔ā寮忋€傝嫢鍚敤锛屼粠涓嬩竴娆℃寜閿捣锛岄┍鍔ㄥ皢鍙戦€?`LIRC_MODE2_FREQUENCY` 鏁版嵁鍖呫€傞粯璁ゆ儏鍐典笅璇ユā寮忓簲澶勪簬鍏抽棴鐘舵€併€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
