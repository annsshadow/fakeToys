


######## ioctl DMX_QBUF, DMX_DQBUF


## 鍚嶇О


DMX_QBUF - DMX_DQBUF - 涓庨┍鍔ㄤ氦鎹㈢紦鍐插尯


## 姒傝



`int ioctl(int fd, DMX_QBUF, struct dmx_buffer *argp)`


`int ioctl(int fd, DMX_DQBUF, struct dmx_buffer *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `dmx_buffer` 鐨勬寚閽堛€?
## 鎻忚堪


搴旂敤绋嬪簭璋冪敤 `DMX_QBUF` ioctl 灏嗙┖鐨勶紙鎹曡幏鐢級鎴栧凡濉厖鐨勶紙杈撳嚭鐢級缂撳啿鍖哄叆闃熷埌椹卞姩鐨勪紶鍏ラ槦鍒椼€傚叾璇箟鍙栧喅浜庢墍閫夋嫨鐨?I/O 鏂规硶銆?
瑕佸叆闃熺紦鍐插尯锛屽簲鐢ㄧ▼搴忚缃?`index` 瀛楁銆傛湁鏁堢殑绱㈠紩缂栧彿鑼冨洿浠庨浂鍒扮敤 DMX_REQBUFS锛坰truct `dmx_requestbuffers` 鐨?`count`锛夊垎閰嶇殑缂撳啿鍖烘暟閲忓噺涓€銆傜敱 DMX_QUERYBUF ioctl 杩斿洖鐨?struct `dmx_buffer` 鍐呭涔熷悓鏍烽€傜敤銆?
褰撲娇鐢ㄤ竴涓寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?`DMX_QBUF` 鏃讹紝瀹冧細灏嗙紦鍐插尯鐨勭墿鐞嗗唴瀛橀〉閿佸畾锛屼娇鍏朵笉鑳借鎹㈠嚭鍒扮鐩樸€傜紦鍐插尯浼氫竴鐩翠繚鎸侀攣瀹氾紝鐩村埌鍑洪槦鎴栬澶囪鍏抽棴銆?
搴旂敤绋嬪簭璋冪敤 `DMX_DQBUF` ioctl 浠庨┍鍔ㄧ殑浼犲嚭闃熷垪涓彇鍑轰竴涓凡濉厖鐨勶紙鎹曡幏鐢級缂撳啿鍖恒€傚畠浠彧闇€鐢ㄨ鍏ラ槦鐨勭紦鍐插尯 ID 璁剧疆 `index` 瀛楁銆傚綋浣跨敤鎸囧悜 struct `dmx_buffer` 鐨勬寚閽堣皟鐢?`DMX_DQBUF` 鏃讹紝椹卞姩浼氬～鍏呭叾浣欏瓧娈垫垨杩斿洖閿欒鐮併€?
榛樿鎯呭喌涓嬶紝褰撲紶鍑洪槦鍒椾腑娌℃湁缂撳啿鍖烘椂 `DMX_DQBUF` 浼氶樆濉炪€傚綋鍚?`open()` 鍑芥暟浼犲叆 `O_NONBLOCK` 鏍囧織鏃讹紝鑻ユ病鏈夊彲鐢ㄧ紦鍐插尯锛宍DMX_DQBUF` 浼氱珛鍗宠繑鍥?`EAGAIN` 閿欒鐮併€?
struct `dmx_buffer` 缁撴瀯鍦?buffer 涓畾涔夈€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EAGAIN
    宸蹭娇鐢?`O_NONBLOCK` 閫夋嫨浜嗛潪闃诲 I/O锛屼笖浼犲嚭闃熷垪涓病鏈夌紦鍐插尯銆?
EINVAL
    `index` 瓒呭嚭鑼冨洿锛屾垨灏氭湭鍒嗛厤浠讳綍缂撳啿鍖恒€?
EIO
    `DMX_DQBUF` 鐢变簬鍐呴儴閿欒鑰屽け璐ャ€備篃鍙兘鎸囩ず涓存椂鎬ч棶棰橈紝濡備俊鍙蜂涪澶辨垨 CRC 閿欒銆?