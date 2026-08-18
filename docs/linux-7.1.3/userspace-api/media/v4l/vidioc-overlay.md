


######## ioctl VIDIOC_OVERLAY


## 鍚嶇О


VIDIOC_OVERLAY - 鍚姩鎴栧仠姝㈣棰戝彔鍔?
## 姒傝



`int ioctl(int fd, VIDIOC_OVERLAY, const int *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜涓€涓暣鏁扮殑鎸囬拡銆?
## 鎻忚堪


姝?ioctl 鏄棰戝彔鍔?<overlay> I/O 鏂规硶鐨勪竴閮ㄥ垎銆傚簲鐢ㄧ▼搴忚皟鐢?VIDIOC_OVERLAY
浠ュ惎鍔ㄦ垨鍋滄鍙犲姞銆傚畠鎺ュ彈涓€涓寚鍚戞暣鏁扮殑鎸囬拡锛屽簲鐢ㄧ▼搴忓繀椤诲皢鍏惰缃负闆朵互
鍋滄鍙犲姞锛岃缃负 1 浠ュ惎鍔ㄣ€?
椹卞姩绋嬪簭涓嶆敮鎸佸皢 VIDIOC_STREAMON 鎴?VIDIOC_STREAMOFF <VIDIOC_STREAMON>
涓?`V4L2_BUF_TYPE_VIDEO_OVERLAY` 涓€璧蜂娇鐢ㄣ€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佺殑鎻忚堪
瑙侀€氱敤閿欒鐮?<gen-errors> 绔犺妭銆?
EINVAL
    鍙犲姞鍙傛暟灏氭湭璁剧疆銆傛湁鍏冲繀瑕佹楠わ紝璇峰弬闃?overlay銆?