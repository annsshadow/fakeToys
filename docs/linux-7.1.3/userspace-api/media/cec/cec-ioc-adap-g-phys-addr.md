

######## ioctls CEC_ADAP_G_PHYS_ADDR 涓?CEC_ADAP_S_PHYS_ADDR


## 鍚嶇О


CEC_ADAP_G_PHYS_ADDR, CEC_ADAP_S_PHYS_ADDR - 鑾峰彇鎴栬缃墿鐞嗗湴鍧€

## 姒傝


`int ioctl(int fd, CEC_ADAP_G_PHYS_ADDR, __u16 *argp)`


`int ioctl(int fd, CEC_ADAP_S_PHYS_ADDR, __u16 *argp)`

## 鍙傛暟


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 CEC 鍦板潃鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇㈠綋鍓嶇墿鐞嗗湴鍧€锛屽簲鐢ㄧ▼搴忎互鎸囧悜涓€涓?__u16 鐨勬寚閽堣皟鐢?ioctl CEC_ADAP_G_PHYS_ADDR <CEC_ADAP_G_PHYS_ADDR>锛岄┍鍔ㄤ細灏嗙墿鐞嗗湴鍧€瀛樺偍鍦ㄥ叾涓€?
瑕佽缃柊鐨勭墿鐞嗗湴鍧€锛屽簲鐢ㄧ▼搴忓皢涓€涓?__u16 涓殑鐗╃悊鍦板潃瀛樺偍濂斤紝骞朵互鎸囧悜璇ユ暣鏁扮殑鎸囬拡璋冪敤 ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR>銆俰octl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 浠呭湪璁剧疆浜?`CEC_CAP_PHYS_ADDR` 鏃跺彲鐢紙鍚﹀垯灏嗚繑鍥?`ENOTTY` 閿欒鐮侊級銆俰octl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 鍙兘鐢卞浜庡彂璧疯€咃紙initiator锛夋ā寮忕殑鏂囦欢鎻忚堪绗﹁皟鐢紙鍙傝 CEC_S_MODE锛夛紝鍚﹀垯灏嗚繑鍥?`EBUSY` 閿欒鐮併€?
瑕佹竻闄ゅ凡鏈夌殑鐗╃悊鍦板潃锛岃浣跨敤 `CEC_PHYS_ADDR_INVALID`銆傞€傞厤鍣ㄥ皢杩涘叆鏈厤缃姸鎬併€?
濡傛灉宸插畾涔変簡閫昏緫鍦板潃绫诲瀷锛堝弬瑙?ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>锛夛紝閭ｄ箞璇?ioctl 浼氶樆濉烇紝鐩村埌鎵€鏈夎姹傜殑閫昏緫鍦板潃閮借璁ら銆傚鏋滄枃浠舵弿杩扮澶勪簬闈為樆濉炴ā寮忥紝鍒欎笉浼氱瓑寰呴€昏緫鍦板潃琚棰嗭紝鑰屾槸鐩存帴杩斿洖 0銆?
褰撶墿鐞嗗湴鍧€鍙戠敓鍙樺寲鏃讹紝浼氬彂閫佷竴涓?CEC_EVENT_STATE_CHANGE <CEC-EVENT-STATE-CHANGE> 浜嬩欢銆?
鐗╃悊鍦板潃鏄竴涓?16 浣嶇殑鏁板瓧锛屽叾涓瘡 4 浣嶄竴缁勪唬琛ㄧ墿鐞嗗湴鍧€ a.b.c.d 鐨勪竴浣嶆暟瀛楋紝鏈€楂?4 浣嶄唬琛?'a'銆侰EC 鏍硅澶囷紙閫氬父鏄數瑙嗭級鐨勫湴鍧€涓?0.0.0.0銆傛瘡涓繛鎺ュ埌鐢佃杈撳叆绔瓙鐨勮澶囧湴鍧€涓?a.0.0.0锛堝叾涓?'a' 鈮?1锛夛紝渚濇杩炴帴鍦ㄨ繖浜涜澶囦笂鐨勮澶囧湴鍧€涓?a.b.0.0锛屼緷姝ょ被鎺ㄣ€傚洜姝ゆ敮鎸佹渶澶?5 灞傛繁鐨勮澶囨嫇鎵戙€傝澶囧簲浣跨敤鐨勭墿鐞嗗湴鍧€瀛樺偍浜庢帴鏀剁锛坰ink锛夌殑 EDID 涓€?
渚嬪锛岀數瑙嗘瘡涓?HDMI 杈撳叆绔殑 EDID 閮戒細鏈変竴涓舰濡?a.0.0.0 鐨勪笉鍚岀墿鐞嗗湴鍧€锛屼俊鍙锋簮浼氳鍑哄苟灏嗗叾鐢ㄤ綔鑷繁鐨勭墿鐞嗗湴鍧€銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 绔犺妭涓弿杩般€?
ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 鍙繑鍥炰互涓嬮敊璇爜锛?
ENOTTY
    鏈缃?`CEC_CAP_PHYS_ADDR` 鑳藉姏锛屽洜姝や笉鏀寔姝?ioctl銆?
EBUSY
    鍙︿竴涓枃浠跺彞鏌勫浜庣嫭鍗犵殑 follower 鎴?initiator 妯″紡锛屾垨鑰呰鏂囦欢鍙ユ焺澶勪簬 `CEC_MODE_NO_INITIATOR` 妯″紡銆?
EINVAL
    鐗╃悊鍦板潃鏍煎紡閿欒銆?