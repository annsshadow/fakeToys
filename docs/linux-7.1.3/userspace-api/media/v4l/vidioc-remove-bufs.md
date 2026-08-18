


######## ioctl VIDIOC_REMOVE_BUFS


## 鍚嶇О


VIDIOC_REMOVE_BUFS - 浠庨槦鍒椾腑绉婚櫎缂撳啿鍖?
## 姒傝



`int ioctl(int fd, VIDIOC_REMOVE_BUFS, struct v4l2_remove_buffers *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_remove_buffers` 鐨勬寚閽堛€?
## 鎻忚堪


搴旂敤绋嬪簭鍙互鍙€夊湴璋冪敤 VIDIOC_REMOVE_BUFS ioctl 浠庨槦鍒椾腑绉婚櫎缂撳啿鍖恒€傝鍚敤 VIDIOC_REMOVE_BUFS锛屽繀椤绘敮鎸?VIDIOC_CREATE_BUFS ioctl銆傚綋璋冪敤 `VIDIOC_REQBUFS` 鎴?`VIDIOC_CREATE_BUFS` 鏃讹紝鑻ラ槦鍒椾笂璁剧疆浜?`V4L2_BUF_CAP_SUPPORTS_REMOVE_BUFS` 鑳藉姏锛屽垯璇?ioctl 鍙敤銆?



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 瑕佺Щ闄ょ殑璧峰缂撳啿鍖虹储寮曘€傝嫢 count == 0锛屾瀛楁琚拷鐣ャ€?    - - __u32
      - `count`
      - 瑕佺Щ闄ょ殑缂撳啿鍖烘暟閲忥紝绱㈠紩浠?'index' 鍒?'index + count - 1'銆?        姝よ寖鍥村唴鐨勬墍鏈夌紦鍐插尯蹇呴』鏈夋晥涓斿浜?DEQUEUED 鐘舵€併€?        VIDIOC_REMOVE_BUFS 鎬讳細妫€鏌?`type` 鐨勬湁鏁堟€э紝鑻ユ棤鏁堝垯杩斿洖 `EINVAL` 閿欒鐮併€?        鑻?count 璁句负 0锛孷IDIOC_REMOVE_BUFS 灏嗕笉鎵ц浠讳綍鎿嶄綔骞惰繑鍥?0銆?    - - __u32
      - `type`
      - 娴佹垨缂撳啿鍖虹殑绫诲瀷锛屼笌 struct `v4l2_format` 鐨?`type` 瀛楁鐩稿悓銆?	鏈夋晥鍊煎弬瑙?`v4l2_buf_type`銆?    - - __u32
      - `reserved`\ [^13^]
      - 涓烘湭鏉ユ墿灞曢鐣欑殑鍗犱綅绗︺€傞┍鍔ㄤ笌搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍缃浂銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€傝嫢鍙戠敓閿欒锛屼笉浼氶噴鏀句换浣曠紦鍐插尯锛屽苟杩斿洖浠ヤ笅閿欒鐮佷箣涓€锛?
EBUSY
    鏂囦欢 I/O 姝ｅ湪杩涜涓€?    `index` 鍒?`index + count - 1` 鑼冨洿涓殑涓€涓垨澶氫釜缂撳啿鍖轰笉澶勪簬 DEQUEUED 鐘舵€併€?
EINVAL
    `index` 鍒?`index + count - 1` 鑼冨洿涓殑涓€涓垨澶氫釜缂撳啿鍖轰笉鍦ㄩ槦鍒椾腑瀛樺湪銆?    缂撳啿鍖虹被鍨嬶紙`type` 瀛楁锛夋棤鏁堛€?