######## ioctl LIRC_SET_SEND_DUTY_CYCLE


## 鍚嶇О


LIRC_SET_SEND_DUTY_CYCLE - 璁剧疆绾㈠鍙戝皠杞芥尝淇″彿鐨勫崰绌烘瘮銆?

## 鎽樿



`int ioctl(int fd, LIRC_SET_SEND_DUTY_CYCLE, __u32 *duty_cycle)`

## 鍙傛暟


`fd`
    鐢?open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`duty_cycle`
    鍗犵┖姣旓紝浠ョ櫨鍒嗘瘮锛? 鍒?99锛夋弿杩版暣涓懆鏈熺殑鑴夊啿瀹藉害銆傚彇鍊?0 鍜?100 涓轰繚鐣欏€笺€?

## 璇存槑


鑾峰彇/璁剧疆绾㈠鍙戝皠杞芥尝淇″彿鐨勫崰绌烘瘮銆?

鐩墠锛? 鍜?100 娌℃湁瀹氫箟鐗规畩鍚箟锛屼絾灏嗘潵鍙兘鐢ㄤ簬鍏抽棴杞芥尝鐢熸垚锛屽洜姝ゅ簲淇濈暀杩欎簺鍊笺€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
