
######## ioctl DMX_QUERYBUF


## 鍚嶇О


DMX_QUERYBUF - 鏌ヨ缂撳啿鍖虹殑鐘舵€?


## 姒傝


`int ioctl(int fd, DMX_QUERYBUF, struct dvb_buffer *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜缁撴瀯浣?`dvb_buffer` 鐨勬寚閽堛€?

## 鎻忚堪


璇?ioctl 鏄?mmap 娴佸紡 I/O 鏂规硶鐨勪竴閮ㄥ垎銆傚畠鍙互鍦ㄤ娇鐢?DMX_REQBUFS ioctl 鍒嗛厤缂撳啿鍖轰箣鍚庣殑浠讳綍鏃堕棿鐢ㄤ簬鏌ヨ缂撳啿鍖虹殑鐘舵€併€?

搴旂敤绋嬪簭璁剧疆 `index` 瀛楁銆傛湁鏁堢殑绱㈠紩缂栧彿鑼冨洿浠庨浂鍒颁娇鐢?DMX_REQBUFS 鍒嗛厤鐨勭紦鍐插尯鏁伴噺锛堢粨鏋勪綋 `dvb_requestbuffers` 鐨?`count`锛夊噺涓€銆?

鍦ㄤ娇鐢ㄦ寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?DMX_QUERYBUF 鍚庯紝椹卞姩杩斿洖閿欒鐮佹垨濉厖缁撴瀯鐨勫叾浣欓儴鍒嗐€?

鎴愬姛鏃讹紝`offset` 灏嗗寘鍚紦鍐插尯璺濊澶囧唴瀛樿捣濮嬩綅缃殑鍋忕Щ锛宍length` 瀛楁涓哄叾澶у皬锛宍bytesused` 涓虹紦鍐插尯涓暟鎹紙鏈夋晥杞借嵎锛夊崰鐢ㄧ殑瀛楄妭鏁般€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛宍offset` 灏嗗寘鍚紦鍐插尯璺濊澶囧唴瀛樿捣濮嬩綅缃殑鍋忕Щ锛宍length` 瀛楁涓哄叾澶у皬锛宍bytesused` 涓虹紦鍐插尯涓暟鎹紙鏈夋晥杞借嵎锛夊崰鐢ㄧ殑瀛楄妭鏁般€?

鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

EINVAL
    `index` 瓒婄晫銆?
