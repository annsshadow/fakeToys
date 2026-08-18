
######## ioctl VIDIOC_G_OUTPUT, VIDIOC_S_OUTPUT


## Name


VIDIOC_G_OUTPUT - VIDIOC_S_OUTPUT - 鏌ヨ鎴栭€夋嫨褰撳墠鐨勮棰戣緭鍑?

## Synopsis


`int ioctl(int fd, VIDIOC_G_OUTPUT, int *argp)`


`int ioctl(int fd, VIDIOC_S_OUTPUT, int *argp)`

## Arguments


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜鍖呭惈杈撳嚭绱㈠紩鐨勬暣鏁扮殑鎸囬拡銆?

## Description


瑕佹煡璇㈠綋鍓嶇殑瑙嗛杈撳嚭锛屽簲鐢ㄧ▼搴忛渶璋冪敤
VIDIOC_G_OUTPUT <VIDIOC_G_OUTPUT> ioctl锛屽苟浼犲叆涓€涓寚鍚戞暣鏁扮殑鎸囬拡锛岄┍鍔ㄤ細
灏嗚緭鍑虹紪鍙峰瓨鍏ュ叾涓紝濡傚悓缁撴瀯浣?
`v4l2_output` 鐨?`index` 瀛楁涓€鏍枫€傝 ioctl 浠呭湪娌℃湁瑙嗛杈撳嚭鏃舵墠浼?
澶辫触锛屽苟杩斿洖 `EINVAL` 閿欒鐮併€?

瑕侀€夋嫨鏌愪釜瑙嗛杈撳嚭锛屽簲鐢ㄧ▼搴忛渶灏嗚閫夋嫨鐨勮緭鍑虹紪鍙峰瓨鍏ヤ竴涓暣鏁帮紝骞惰皟鐢?VIDIOC_S_OUTPUT <VIDIOC_G_OUTPUT> ioctl锛屼紶鍏?
鎸囧悜璇ユ暣鏁扮殑鎸囬拡銆傝繖鍙兘浼氫骇鐢熷壇浣滅敤銆備緥濡傦紝涓嶅悓鐨勮緭鍑哄彲鑳芥敮鎸佷笉鍚岀殑瑙嗛鏍囧噯锛屽洜姝ら┍鍔ㄥ彲鑳戒細闅愬紡鍦?
鍒囨崲褰撳墠鏍囧噯銆傜敱浜庤繖浜涘彲鑳界殑鍓綔鐢紝搴旂敤绋嬪簭蹇呴』鍏堥€夋嫨涓€涓緭鍑猴紝鐒跺悗鍐嶆煡璇㈡垨
鍗忓晢浠讳綍鍏朵粬鍙傛暟銆?

鏈夊叧瑙嗛杈撳嚭鐨勪俊鎭彲閫氳繃
VIDIOC_ENUMOUTPUT ioctl 鑾峰彇銆?

## Return Value


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

EINVAL
    瑙嗛杈撳嚭鐨勭紪鍙疯秺鐣岋紝鎴栬€呮牴鏈病鏈変换浣曡棰戣緭鍑恒€?
