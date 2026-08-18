


######## ioctl VIDIOC_G_STD, VIDIOC_S_STD, VIDIOC_SUBDEV_G_STD, VIDIOC_SUBDEV_S_STD


## 鍚嶇О


VIDIOC_G_STD - VIDIOC_S_STD - VIDIOC_SUBDEV_G_STD - VIDIOC_SUBDEV_S_STD - 鏌ヨ鎴栭€夋嫨褰撳墠杈撳叆鐨勮棰戞爣鍑?
## 姒傝



`int ioctl(int fd, VIDIOC_G_STD, v4l2_std_id *argp)`


`int ioctl(int fd, VIDIOC_S_STD, const v4l2_std_id *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_G_STD, v4l2_std_id *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_STD, const v4l2_std_id *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 `v4l2_std_id` 鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇㈠拰閫夋嫨褰撳墠鐨勮棰戞爣鍑嗭紝搴旂敤绋嬪簭浣跨敤 VIDIOC_G_STD <VIDIOC_G_STD> 鍜?VIDIOC_S_STD <VIDIOC_G_STD> ioctl锛屽畠浠皢鎸囧悜 v4l2_std_id <v4l2-std-id> 绫诲瀷鐨勬寚閽堜綔涓哄弬鏁般€俈IDIOC_G_STD <VIDIOC_G_STD> 鍙互杩斿洖鍗曚釜鏍囧織鎴栦竴缁勬爣蹇楋紝濡傚悓 struct `v4l2_standard` 鐨?`id` 瀛楁閭ｆ牱銆傝繖浜涙爣蹇楀繀椤绘槑纭棤璇紝鍗冲畠浠彧鍑虹幇鍦ㄥ敮涓€鐨勬煇涓鏋氫妇鐨?struct `v4l2_standard` 缁撴瀯涓€?
VIDIOC_S_STD <VIDIOC_G_STD> 鎺ュ彈涓€涓垨澶氫釜鏍囧織锛屼綔涓轰竴涓彧鍐?ioctl锛屽畠涓嶄細鍍?VIDIOC_G_STD <VIDIOC_G_STD> 閭ｆ牱杩斿洖瀹為檯鐨勬柊鏍囧噯銆傚綋娌℃湁缁欏嚭浠讳綍鏍囧織锛屾垨鑰呭綋鍓嶈緭鍏ヤ笉鏀寔鎵€璇锋眰鐨勬爣鍑嗘椂锛岄┍鍔ㄨ繑鍥?`EINVAL` 閿欒鐮併€傚綋鏍囧噯闆嗗悎瀛樺湪姝т箟鏃讹紝椹卞姩鍙兘杩斿洖 `EINVAL` 鎴栭€夋嫨浠绘剰涓€涓墍璇锋眰鐨勬爣鍑嗐€傚鏋滃綋鍓嶈緭鍏ユ垨杈撳嚭涓嶆敮鎸佹爣鍑嗚棰戞椂搴忥紙渚嬪锛岃嫢 VIDIOC_ENUMINPUT 娌℃湁璁剧疆 `V4L2_IN_CAP_STD` 鏍囧織锛夛紝鍒欒繑鍥?`ENODATA` 閿欒鐮併€?
鍦ㄥ浠ュ彧璇绘ā寮忔敞鍐岀殑瀛愯澶囷紙subdev锛夎妭鐐逛笂璋冪敤 `VIDIOC_SUBDEV_S_STD` 鏄笉鍏佽鐨勩€傚皢杩斿洖閿欒骞舵妸 errno 鍙橀噺璁剧疆涓?`-EPERM`銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    VIDIOC_S_STD <VIDIOC_G_STD> 鍙傛暟涓嶅悎閫傘€?
ENODATA
    璇ヨ緭鍏ユ垨杈撳嚭涓嶆敮鎸佹爣鍑嗚棰戞椂搴忋€?
EPERM
    `VIDIOC_SUBDEV_S_STD` 鍦ㄨ璋冪敤鍦ㄤ竴涓彧璇诲瓙璁惧涓娿€?