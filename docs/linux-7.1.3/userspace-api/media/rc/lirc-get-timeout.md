

######## ioctl LIRC_GET_MIN_TIMEOUT 鍜?LIRC_GET_MAX_TIMEOUT


## 濮撳悕


LIRC_GET_MIN_TIMEOUT / LIRC_GET_MAX_TIMEOUT - 鑾峰彇鍙兘鐨勮秴鏃舵椂闂?
绾㈠鎺ユ敹鑼冨洿銆?

## 姒傝



`int ioctl(int fd, LIRC_GET_MIN_TIMEOUT, __u32 *timeout)`


`int ioctl(int fd, LIRC_GET_MAX_TIMEOUT, __u32 *timeout)`

## 璁虹偣


`fd`
open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`timeout`
瓒呮椂锛屼互寰涓哄崟浣嶃€?

## 鎻忚堪


鏌愪簺璁惧鍏锋湁鍐呴儴瀹氭椂鍣紝鍙敤浜庢娴嬩綍鏃?
寰堥暱涓€娈垫椂闂存病鏈塈R娲诲姩銆傝繖鍙互甯姪 lircd
妫€娴婭R淇″彿瀹屾垚锛屽彲浠ュ姞蹇В鐮侀€熷害
杩囩▼銆傝繑鍥炲叿鏈夋渶灏?鏈€澶ц秴鏃剁殑鏁存暟鍊?
鍙互璁剧疆銆?


鏈変簺璁惧鏈夊浐瀹氱殑瓒呮椂鏃堕棿锛屽湪杩欑鎯呭喌涓?
鍗充娇瓒呮椂锛屼袱涓?ioctl 涔熶細杩斿洖鐩稿悓鐨勫€?
鏃犳硶閫氳繃 LIRC_SET_REC_TIMEOUT 鏇存敼銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛岄敊璇椂杩斿洖 -1 骞朵笖璁剧疆 `errno` 鍙橀噺
閫傚綋鍦般€傞€氱敤閿欒浠ｇ爜鐨勬弿杩拌
閫氱敤閿欒浠ｇ爜 <gen-errors> 绔犺妭銆?
