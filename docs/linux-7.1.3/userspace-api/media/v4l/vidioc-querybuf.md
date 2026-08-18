


######## ioctl VIDIOC_QUERYBUF


## 鍚嶇О


VIDIOC_QUERYBUF - 鏌ヨ缂撳啿鍖虹殑鐘舵€?
## 姒傝



`int ioctl(int fd, VIDIOC_QUERYBUF, struct v4l2_buffer *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_buffer` 鐨勬寚閽堛€?
## 鎻忚堪


璇?ioctl 灞炰簬娴佸紡 <mmap> I/O 鏂规硶鐨勪竴閮ㄥ垎銆傚畠鍙互鍦ㄤ娇鐢?VIDIOC_REQBUFS ioctl 鍒嗛厤缂撳啿鍖轰箣鍚庣殑浠绘剰鏃跺埢锛岀敤浜庢煡璇㈢紦鍐插尯鐨勭姸鎬併€?
搴旂敤绋嬪簭灏?struct `v4l2_buffer` 鐨?`type` 瀛楁璁剧疆涓哄厛鍓嶄笌 struct `v4l2_format` 鐨?`type` 浠ュ強 struct `v4l2_requestbuffers` 鐨?`type` 鎵€鐢ㄨ繃鐨勭浉鍚岀紦鍐插尯绫诲瀷锛屽苟璁剧疆 `index` 瀛楁銆傛湁鏁堢殑绱㈠紩缂栧彿鑼冨洿浠庨浂鍒扮敤 VIDIOC_REQBUFS锛坰truct `v4l2_requestbuffers` 鐨?`count`锛夊垎閰嶇殑缂撳啿鍖烘暟閲忓噺涓€銆俙reserved` 涓?`reserved2` 瀛楁蹇呴』璁剧疆涓?0銆備娇鐢ㄥ骞抽潰 API <planar-apis> 鏃讹紝`m.planes` 瀛楁蹇呴』鍖呭惈涓€涓寚鍚?struct `v4l2_plane` 鏁扮粍鐨勭敤鎴风┖闂存寚閽堬紝涓?`length` 瀛楁蹇呴』璁剧疆涓鸿鏁扮粍鐨勫厓绱犱釜鏁般€傚湪鐢ㄤ竴涓寚鍚戣缁撴瀯鐨勬寚閽堣皟鐢?VIDIOC_QUERYBUF 鍚庯紝椹卞姩杩斿洖閿欒鐮佹垨濉厖缁撴瀯鐨勫叾浣欓儴鍒嗐€?
鍦?`flags` 瀛楁涓紝`V4L2_BUF_FLAG_MAPPED`銆乣V4L2_BUF_FLAG_PREPARED`銆乣V4L2_BUF_FLAG_QUEUED` 涓?`V4L2_BUF_FLAG_DONE` 鏍囧織灏嗘槸鏈夋晥鐨勩€俙memory` 瀛楁灏嗚璁剧疆涓哄綋鍓嶇殑 I/O 鏂规硶銆傚浜庡崟骞抽潰 API锛宍m.offset` 鍖呭惈缂撳啿鍖虹浉瀵硅澶囧唴瀛樿捣濮嬩綅缃殑鍋忕Щ閲忥紝`length` 瀛楁涓哄叾澶у皬銆傚浜庡骞抽潰 API锛屽皢鏀圭敤 `m.planes` 鏁扮粍鍏冪礌涓殑 `m.mem_offset` 涓?`length` 瀛楁锛屼笖 struct `v4l2_buffer` 鐨?`length` 瀛楁琚缃负宸插～鍏呯殑鏁扮粍鍏冪礌涓暟銆傞┍鍔ㄥ彲鑳戒細涔熷彲鑳戒笉浼氳缃叾浣欏瓧娈典笌鏍囧織锛屽湪姝や笂涓嬫枃涓畠浠病鏈夋剰涔夈€?
struct `v4l2_buffer` 缁撴瀯鍦?buffer 涓畾涔夈€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    缂撳啿鍖?`type` 涓嶅彈鏀寔锛屾垨 `index` 瓒呭嚭鑼冨洿銆?