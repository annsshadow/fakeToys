
######## ioctl MEDIA_IOC_REQUEST_ALLOC


## 鍚嶇О


MEDIA_IOC_REQUEST_ALLOC - 鍒嗛厤涓€涓姹?

## 姒傝


`int ioctl(int fd, MEDIA_IOC_REQUEST_ALLOC, int *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜涓€涓暣鏁扮殑鎸囬拡銆?

## 鎻忚堪


濡傛灉濯掍綋璁惧鏀寔璇锋眰 <media-request-api>锛屽垯
璇?ioctl 鍙敤浜庡垎閰嶄竴涓姹傘€傚鏋滀笉鏀寔锛屽垯
`errno` 琚涓?`ENOTTY`銆傝姹傞€氳繃涓€涓枃浠舵弿杩扮璁块棶锛岃鎻忚堪绗?
鍦?`*argp` 涓繑鍥炪€?

濡傛灉璇锋眰鎴愬姛鍒嗛厤锛屽垯璇ヨ姹傛枃浠舵弿杩扮鍙互琚紶閫掔粰
VIDIOC_QBUF <VIDIOC_QBUF>銆乂IDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>銆乂IDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 浠ュ強
VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl銆?

姝ゅ锛屽彲閫氳繃璋冪敤 MEDIA_REQUEST_IOC_QUEUE 灏嗚璇锋眰鍏ラ槦锛屽苟閫氳繃璋冪敤
MEDIA_REQUEST_IOC_REINIT 閲嶆柊鍒濆鍖栥€?

鏈€鍚庯紝鍙互瀵硅鏂囦欢鎻忚堪绗︽墽琛?poll <request-func-poll> 浠ョ瓑寰?
璇锋眰瀹屾垚銆?

璇ヨ姹傚皢涓€鐩翠繚鎸佸垎閰嶇姸鎬侊紝鐩村埌涓庝箣鍏宠仈鐨勬墍鏈夋枃浠舵弿杩扮閮借 `close()` 鍏抽棴锛屼笖椹卞姩鍐呴儴
涓嶅啀浣跨敤璇ヨ姹傘€傛洿澶氫俊鎭鍙傝
姝ゅ <media-request-life-time>銆?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?

ENOTTY
    椹卞姩涓嶆敮鎸佽姹傘€?
