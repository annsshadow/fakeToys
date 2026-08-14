
######## ioctl VIDIOC_G_INPUT, VIDIOC_S_INPUT


## 鍚嶇О


VIDIOC_G_INPUT - VIDIOC_S_INPUT - 鏌ヨ鎴栭€夋嫨褰撳墠鐨勮棰戣緭鍏?

## 姒傝


`int ioctl(int fd, VIDIOC_G_INPUT, int *argp)`


`int ioctl(int fd, VIDIOC_S_INPUT, int *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜鍖呭惈杈撳叆绱㈠紩鐨勬暣鏁般€?

## 鎻忚堪


瑕佹煡璇㈠綋鍓嶇殑瑙嗛杈撳叆锛屽簲鐢ㄧ▼搴忛渶璋冪敤 VIDIOC_G_INPUT <VIDIOC_G_INPUT> ioctl锛屽苟浼犲叆涓€涓寚鍚戞暣鏁扮殑鎸囬拡锛岄┍鍔ㄤ細灏嗚杈撳叆鐨勭紪鍙峰瓨鍏ュ叾涓紝濡傚悓缁撴瀯浣?`v4l2_input` 鐨?`index` 瀛楁涓€鏍枫€傝 ioctl 浠呭湪娌℃湁瑙嗛杈撳叆鏃舵墠浼氬け璐ワ紝骞惰繑鍥?`EINVAL`銆?

瑕侀€夋嫨鏌愪釜瑙嗛杈撳叆锛屽簲鐢ㄧ▼搴忛渶灏嗚閫夋嫨鐨勮緭鍏ョ紪鍙峰瓨鍏ヤ竴涓暣鏁帮紝骞惰皟鐢?VIDIOC_S_INPUT <VIDIOC_G_INPUT> ioctl锛屼紶鍏ユ寚鍚戣鏁存暟鐨勬寚閽堛€傝繖鍙兘浼氫骇鐢熷壇浣滅敤銆備緥濡傦紝杈撳叆鍙兘鏀寔涓嶅悓鐨勮棰戞爣鍑嗭紝鍥犳椹卞姩鍙兘浼氶殣寮忓湴鍒囨崲褰撳墠鏍囧噯銆傜敱浜庤繖浜涘彲鑳界殑鍓綔鐢紝搴旂敤绋嬪簭蹇呴』鍏堥€夋嫨涓€涓緭鍏ワ紝鐒跺悗鍐嶆煡璇㈡垨鍗忓晢浠讳綍鍏朵粬鍙傛暟銆?

鏈夊叧瑙嗛杈撳叆鐨勪俊鎭彲閫氳繃 VIDIOC_ENUMINPUT ioctl 鑾峰彇銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

EINVAL
    瑙嗛杈撳叆鐨勭紪鍙疯秺鐣屻€?
