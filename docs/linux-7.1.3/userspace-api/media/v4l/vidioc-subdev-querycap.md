


######## ioctl VIDIOC_SUBDEV_QUERYCAP


## 鍚嶇О


VIDIOC_SUBDEV_QUERYCAP - 鏌ヨ瀛愯澶囪兘鍔?
## 姒傝


`int ioctl(int fd, VIDIOC_SUBDEV_QUERYCAP, struct v4l2_subdev_capability *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_subdev_capability` 鐨勬寚閽堛€?
## 鎻忚堪


鎵€鏈?V4L2 瀛愯澶囬兘鏀寔 `VIDIOC_SUBDEV_QUERYCAP` ioctl銆傚畠鐢ㄤ簬璇嗗埆涓庢湰
瑙勮寖鍏煎鐨勫唴鏍歌澶囷紝骞惰幏鍙栨湁鍏抽┍鍔ㄤ笌纭欢鑳藉姏鐨勪俊鎭€傝 ioctl 鎺ュ彈涓€涓?鎸囧悜 struct `v4l2_subdev_capability` 鐨勬寚閽堬紝鐢遍┍鍔ㄥ～鍏呫€傚綋椹卞姩涓庢湰
瑙勮寖涓嶅吋瀹规椂锛岃 ioctl 杩斿洖 `ENOTTY` 閿欒鐮併€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 4 20

    - - __u32
      - `version`
      - 椹卞姩鐨勭増鏈彿銆?
	鎶ュ憡鐨勭増鏈敱 V4L2 瀛愮郴缁熸寜鐓у唴鏍哥紪鍙锋柟妗堟彁渚涖€備笉杩囷紝瀹冨彲鑳藉苟闈?	鎬绘槸杩斿洖涓庡唴鏍哥浉鍚岀殑鐗堟湰锛屼緥濡傦紝褰撴煇涓ǔ瀹氱増鎴栦慨鏀硅繃鐨勫彂琛岀増
	鍐呮牳浣跨敤浜嗘潵鑷洿鏂板唴鏍哥殑 V4L2 鏍堟椂銆?
	鐗堟湰鍙蜂娇鐢?`KERNEL_VERSION()` 瀹忔牸寮忓寲锛?    - - `2`

	`#define KERNEL_VERSION(a,b,c) (((a) << 16) + ((b) << 8) + (c))`

	`__u32 version = KERNEL_VERSION(0, 8, 1);`

	`printf ("Version: %u.%u.%u\\n",`

	`(version >> 16) & 0xFF, (version >> 8) & 0xFF, version & 0xFF);`
    - - __u32
      - `capabilities`
      - 鎵€鎵撳紑璁惧鐨勫瓙璁惧鑳藉姏锛岃鍙傞槄
	subdevice-capabilities銆?    - - __u32
      - `reserved`\ [^14^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傜敱 V4L2 鏍稿績璁剧疆涓?0銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - V4L2_SUBDEV_CAP_RO_SUBDEV
      - 0x00000001
      - 瀛愯澶囪澶囪妭鐐逛互鍙妯″紡娉ㄥ唽銆?	瀵逛慨鏀硅澶囩姸鎬佺殑瀛愯澶?ioctl 鐨勮闂彈鍒伴檺鍒躲€傚叧浜庡摢浜涢檺鍒堕€傜敤浜?	鍙瀛愯澶囷紝璇峰弬闃呭悇鑷殑瀛愯澶?ioctl 鏂囨。銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
ENOTTY
    璇ヨ澶囪妭鐐逛笉鏄?V4L2 瀛愯澶囥€?