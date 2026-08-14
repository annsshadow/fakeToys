

######## ioctl VIDEO_PREPARE_BUF


## 濮撳悕


VIDIOC_PREPARE_BUF - 涓?I/O 鍑嗗缂撳啿鍖?

## 姒傝



`int ioctl(int fd, VIDIOC_PREPARE_BUF, struct v4l2_buffer *argp)`

## 璁虹偣


`fd`
`open()`杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
鎸囧悜缁撴瀯浣揱v4l2_buffer`鐨勬寚閽堛€?

## 鎻忚堪


搴旂敤绋嬪簭鍙互閫夋嫨璋冪敤 VIDIOC_PREPARE_BUF ioctl
鍦ㄥ疄闄呭皢缂撳啿鍖烘帓闃熶箣鍓嶅皢缂撳啿鍖虹殑鎵€鏈夋潈浼犻€掔粰椹卞姩绋嬪簭锛?
浣跨敤 VIDIOC_QBUF <VIDIOC_QBUF> ioctl锛屽苟涓哄皢鏉ョ殑 I/O 鍋氬ソ鍑嗗銆傝繖鏍风殑
鍑嗗宸ヤ綔鍙兘鍖呮嫭缂撳瓨澶辨晥鎴栨竻鐞嗐€傝〃婕斿畠浠?
鎻愬墠鑺傜渷瀹為檯 I/O 鏈熼棿鐨勬椂闂淬€?

struct `v4l2_buffer` 缁撴瀯浣撳湪涓寚瀹?
缂撳啿銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛岄敊璇椂杩斿洖 -1 骞朵笖璁剧疆 `errno` 鍙橀噺
閫傚綋鍦般€傞€氱敤閿欒浠ｇ爜鐨勬弿杩拌
閫氱敤閿欒浠ｇ爜 <gen-errors> 绔犺妭銆?

蹇欑鐨?
鏂囦欢 I/O 姝ｅ湪杩涜涓€?

鍗曢」閫夋嫨
缂撳啿鍖篳type`涓嶅彈鏀寔锛屾垨鑰卄index`瓒呭嚭鑼冨洿
杈圭晫锛屾垨鑰呭皻鏈垎閰嶇紦鍐插尯锛屾垨鑰?`userptr` 鎴?
`length`鏃犳晥銆?
