######## ioctl FE_DISEQC_RESET_OVERLOAD


## 鍚嶇О


FE_DISEQC_RESET_OVERLOAD - 濡傛灉鎬荤嚎鍥犺繃杞芥柇鐢碉紝鍒欐仮澶嶅ぉ绾垮瓙绯荤粺鐨勪緵鐢点€?

## 鎽樿



`int ioctl(int fd, FE_DISEQC_RESET_OVERLOAD, NULL)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

## 璇存槑


濡傛灉鎬荤嚎鍥犲姛鐜囪繃杞借鑷姩鏂數锛岃 ioctl 璋冪敤浼氭仮澶嶆€荤嚎鐨勪緵鐢点€傝璋冪敤闇€瑕佸璁惧鐨勮鍐欒闂潈闄愩€傝嫢璁惧琚墜鍔ㄦ柇鐢碉紝鍒欒璋冪敤鏃犳晥銆傚苟闈炴墍鏈夋暟瀛楃數瑙嗭紙Digital TV锛夐€傞厤鍣ㄩ兘鏀寔璇?ioctl銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0銆?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
