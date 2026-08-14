######## ioctl LIRC_SET_SEND_CARRIER


## Name


LIRC_SET_SEND_CARRIER - 璁剧疆鐢ㄤ簬璋冨埗 IR 鍙戝皠鐨勫彂閫佽浇娉€?

## Synopsis



`int ioctl(int fd, LIRC_SET_SEND_CARRIER, __u32 *frequency)`

## Arguments


`fd`
    鐢?open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`frequency`
    寰呰皟鍒惰浇娉㈢殑棰戠巼锛屽崟浣嶄负 Hz銆?

## Description


璁剧疆鐢ㄤ簬璋冨埗 IR PWM 鑴夊啿涓庨棿闅旂殑鍙戦€佽浇娉€?

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
