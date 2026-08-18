

######## ioctl LIRC_SET_TRANSMITTER_MASK


## 濮撳悕


LIRC_SET_TRANSMITTER_MASK - 鍦ㄧ粰瀹氱殑涓€缁勫彂灏勫櫒涓婂惎鐢ㄥ彂閫佷唬鐮?

## 姒傝



`int ioctl(int fd, LIRC_SET_TRANSMITTER_MASK, __u32 *mask)`

## 璁虹偣


`fd`
open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`mask`
甯︽湁閫氶亾鐨勬帺鐮佷互鍚敤 tx銆傞€氶亾 0 鏄渶浣庢湁鏁堜綅銆?

## 鎻忚堪


鏈変簺 IR TX 璁惧鏈夊涓緭鍑洪€氶亾锛屽湪杩欑鎯呭喌涓嬶紝
LIRC_CAN_SET_TRANSMITTER_MASK <LIRC-CAN-SET-TRANSMITTER-MASK> 鏄?
閫氳繃 LIRC_GET_FEATURES 杩斿洖锛屾 ioctl 璁剧疆鍝簺閫氶亾灏?
鍙戦€佺孩澶栦唬鐮併€?

璇?ioctl 鍚敤缁欏畾鐨勪竴缁勫彂灏勫櫒銆傜涓€涓彂灏勫櫒鏄?
鐢辨渶浣庢湁鏁堜綅缂栫爜绛夌瓑銆?

褰撶粰鍑烘棤鏁堢殑浣嶆帺鐮佹椂锛屽嵆璁剧疆浜嗕竴涓綅锛屽嵆浣胯澶?
娌℃湁閭ｄ箞澶氫腑杞櫒锛岄偅涔堣繖涓?ioctl 杩斿洖鐨勬暟閲?
鍙敤鐨勪紶杈撳櫒锛屽苟涓斾笉鎵ц浠讳綍鍏朵粬鎿嶄綔銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛岄敊璇椂杩斿洖 -1 骞朵笖璁剧疆 `errno` 鍙橀噺
閫傚綋鍦般€傞€氱敤閿欒浠ｇ爜鐨勬弿杩拌
閫氱敤閿欒浠ｇ爜 <gen-errors> 绔犺妭銆?
