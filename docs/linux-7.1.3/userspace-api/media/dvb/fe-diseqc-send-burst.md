


######## ioctl FE_DISEQC_SEND_BURST


## 鍚嶇О


FE_DISEQC_SEND_BURST - 涓?2x1 mini DiSEqC 鍗槦閫夋嫨鍙戦€?22KHz 闊宠皟绐佸彂銆?
## 姒傝



`int ioctl(int fd, FE_DISEQC_SEND_BURST, enum fe_sec_mini_cmd tone)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`tone`
    鍦?`fe_sec_mini_cmd` 涓弿杩扮殑鏁存暟鏋氫妇鍊笺€?
## 鎻忚堪


姝?ioctl 鐢ㄤ簬涓?2x1 寮€鍏崇殑 mini DiSEqC 鍗槦閫夋嫨璁剧疆 22kHz 闊宠皟绐佸彂鐨?鐢熸垚銆傛璋冪敤闇€瑕佽/鍐欐潈闄愩€?
瀹冩敮鎸?`Digital Satellite Equipment Control (DiSEqC) - Simple "ToneBurst" Detection Circuit specification. <http://www.eutelsat.com/files/contributed/satellites/pdf/Diseqc/associated%20docs/simple_tone_burst_detec.pdf>`__
涓瀹氱殑鍐呭銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
閫氱敤閿欒鐮佺殑鎻忚堪瑙侀€氱敤閿欒鐮?<gen-errors> 绔犺妭銆?