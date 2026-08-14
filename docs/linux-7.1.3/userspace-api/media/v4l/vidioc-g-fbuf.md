


######## ioctl VIDIOC_G_FBUF, VIDIOC_S_FBUF


## Name


VIDIOC_G_FBUF - VIDIOC_S_FBUF - Get or set frame buffer overlay parameters

## Synopsis



`int ioctl(int fd, VIDIOC_G_FBUF, struct v4l2_framebuffer *argp)`


`int ioctl(int fd, VIDIOC_S_FBUF, const struct v4l2_framebuffer *argp)`

## Arguments


`fd`
    File descriptor returned by `open()`.

`argp`
    Pointer to struct `v4l2_framebuffer`.

## Description


Applications can use the VIDIOC_G_FBUF <VIDIOC_G_FBUF> and VIDIOC_S_FBUF <VIDIOC_G_FBUF> ioctl
to get and set the framebuffer parameters for a
Video Overlay <overlay> or Video Output Overlay <osd>
(OSD). The type of overlay is implied by the device type (capture or
output device) and can be determined with the
VIDIOC_QUERYCAP ioctl. One `/dev/videoN`
device must not support both kinds of overlay.

The V4L2 API distinguishes destructive and non-destructive overlays. A
destructive overlay copies captured video images into the video memory
of a graphics card. A non-destructive overlay blends video images into a
VGA signal or graphics into a video signal. **Video Output Overlays** are
always non-destructive.

Destructive overlay support has been removed: with modern GPUs and CPUs
this is no longer needed, and it was always a very dangerous feature.

To get the current parameters applications call the VIDIOC_G_FBUF <VIDIOC_G_FBUF>
ioctl with a pointer to a struct `v4l2_framebuffer`
structure. The driver fills all fields of the structure or returns an
EINVAL error code when overlays are not supported.

To set the parameters for a **Video Output Overlay**, applications must
initialize the `flags` field of a struct
`v4l2_framebuffer`. Since the framebuffer is
implemented on the TV card all other parameters are determined by the
driver. When an application calls VIDIOC_S_FBUF <VIDIOC_G_FBUF> with a pointer to
this structure, the driver prepares for the overlay and returns the
framebuffer parameters as VIDIOC_G_FBUF <VIDIOC_G_FBUF> does, or it returns an error
code.

To set the parameters for a **Video Capture Overlay**
applications must initialize the `flags` field, the `fmt`
substructure, and call VIDIOC_S_FBUF <VIDIOC_G_FBUF>. Again the driver prepares for
the overlay and returns the framebuffer parameters as VIDIOC_G_FBUF <VIDIOC_G_FBUF>
does, or it returns an error code.





    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 1 2

    - - __u32
      - `capability`
      -
      - 鐢遍┍鍔ㄨ缃殑 overlay 鑳藉姏鏍囧織锛屽弬瑙?framebuffer-cap銆?    - - __u32
      - `flags`
      -
      - 鐢卞簲鐢ㄧ▼搴忓拰椹卞姩璁剧疆鐨?overlay 鎺у埗鏍囧織锛屽弬瑙?framebuffer-flags
    - - void *
      - `base`
      -
      - 甯х紦鍐插尯鐨勭墿鐞嗗熀鍦板潃锛屽嵆甯х紦鍐插尯宸︿笂瑙掑儚绱犵殑鍦板潃銆?	瀵逛簬 VIDIOC_S_FBUF <VIDIOC_G_FBUF> 姝ゅ瓧娈典笉鍐嶅彈鏀寔锛?	鍐呮牳灏嗗缁堝皢鍏惰涓?NULL銆?	瀵逛簬 **Video Output Overlays**锛?	椹卞姩灏嗚繑鍥炰竴涓湁鏁堢殑鍩哄湴鍧€锛屼互渚垮簲鐢ㄧ▼搴忓彲浠ユ壘鍒板搴旂殑
	Linux 甯х紦鍐茶澶囷紙鍙傝 osd锛夈€傚浜?**Video Capture Overlays**
	姝ゅ瓧娈靛皢濮嬬粓涓?NULL銆?    - - struct
      - `fmt`
      -
      - 甯х紦鍐插尯鐨勫竷灞€銆?    - -
      - __u32
      - `width`
      - 甯х紦鍐插尯鐨勫搴︼紝浠ュ儚绱犺銆?    - -
      - __u32
      - `height`
      - 甯х紦鍐插尯鐨勯珮搴︼紝浠ュ儚绱犺銆?    - -
      - __u32
      - `pixelformat`
      - 甯х紦鍐插尯鐨勫儚绱犳牸寮忋€?#     * -

      -
      - 瀵逛簬 **non-destructive Video Overlays**锛屾瀛楁浠呬负
	struct `v4l2_window` 鐨?`chromakey`
	瀛楁瀹氫箟涓€涓牸寮忋€?#     * -

      -
      - 瀵逛簬 **Video Output Overlays**锛岄┍鍔ㄥ繀椤昏繑鍥炰竴涓湁鏁堢殑鏍煎紡銆?#     * -

      -
      - 閫氬父杩欐槸涓€涓?RGB 鏍煎紡锛堜緥濡?	V4L2_PIX_FMT_RGB565 <V4L2-PIX-FMT-RGB565>锛夛紝浣?YUV
	鏍煎紡锛堜粎褰撲娇鐢ㄨ壊搴﹂敭鎺ф椂涓?packed YUV 鏍煎紡锛屼笉鍖呮嫭
	`V4L2_PIX_FMT_YUYV` 鍜?`V4L2_PIX_FMT_UYVY`锛変互鍙?	`V4L2_PIX_FMT_PAL8` 鏍煎紡涔熷厑璁镐娇鐢ㄣ€傚綋搴旂敤绋嬪簭璇锋眰鍘嬬缉鏍煎紡鏃?	椹卞姩鐨勮涓烘槸鏈畾涔夌殑銆傚叧浜庡儚绱犳牸寮忕殑淇℃伅鍙傝 pixfmt銆?    - -
      - enum `v4l2_field`
      - `field`
      - 椹卞姩鍜屽簲鐢ㄧ▼搴忓簲蹇界暐姝ゅ瓧娈点€傚閫傜敤锛屽瓧娈甸『搴忕敱
	VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 浣跨敤 struct `v4l2_window`
	鐨?`field` 瀛楁閫夋嫨銆?    - -
      - __u32
      - `bytesperline`
      - 涓ゆ潯鐩搁偦鎵弿绾挎渶宸︿晶鍍忕礌涔嬮棿鐨勮窛绂伙紝浠ュ瓧鑺傝銆?    - - `3`

	This field is irrelevant to **non-destructive Video Overlays**.

	For **Video Output Overlays** the driver must return a valid value.

	Video hardware may access padding bytes, therefore they must
	reside in accessible memory. Consider for example the case where
	padding bytes after the last line of an image cross a system page
	boundary. Capture devices may write padding bytes, the value is
	undefined. Output devices ignore the contents of padding bytes.

	When the image format is planar the `bytesperline` value applies
	to the first plane and is divided by the same factor as the
	`width` field for the other planes. For example the Cb and Cr
	planes of a YUV 4:2:0 image have half as many padding bytes
	following each line as the Y plane. To avoid ambiguities drivers
	must return a `bytesperline` value rounded up to a multiple of
	the scale factor.
    - -
      - __u32
      - `sizeimage`
      - This field is irrelevant to **non-destructive Video Overlays**.
	For **Video Output Overlays** the driver must return a valid
	format.

	Together with `base` it defines the framebuffer memory
	accessible by the driver.
    - -
      - enum `v4l2_colorspace`
      - `colorspace`
      - 璇ヤ俊鎭ˉ鍏?`pixelformat`锛屽繀椤荤敱椹卞姩璁剧疆锛屽弬瑙?colorspaces銆?    - -
      - __u32
      - `priv`
      - 淇濈暀銆傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗘瀛楁璁句负闆躲€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FBUF_CAP_EXTERNOVERLAY`
      - 0x0001
      - 璁惧鏀寔闈炵牬鍧忔€?overlay銆傚綋椹卞姩娓呴櫎姝ゆ爣蹇楁椂锛屼粎鏀寔鐮村潖鎬?	overlay銆傜洰鍓嶈繕娌℃湁鍚屾椂鏀寔鐮村潖鎬?overlay 鍜岄潪鐮村潖鎬?overlay 鐨?	椹卞姩銆傚疄闄呬笂 Video Output Overlays 鎬绘槸闈炵牬鍧忔€х殑銆?    - - `V4L2_FBUF_CAP_CHROMAKEY`
      - 0x0002
      - 璁惧鏀寔閫氳繃鑹插害閿帶瀵瑰浘鍍忚繘琛岃鍓€傚嵆锛屼粎鍦ㄥ悗鑰呭憟鐜版煇绉嶇壒瀹?	棰滆壊鐨勪綅缃紝鍥惧儚鍍忕礌鎵嶆浛鎹?VGA 鎴栬棰戜俊鍙蜂腑鐨勫儚绱犮€傝壊搴﹂敭鎺?	瀵圭牬鍧忔€?overlay 娌℃湁鎰忎箟銆?    - - `V4L2_FBUF_CAP_LIST_CLIPPING`
      - 0x0004
      - 璁惧鏀寔浣跨敤瑁佸壀鐭╁舰鍒楄〃杩涜瑁佸壀銆?        娉ㄦ剰锛屾鍔熻兘涓嶅啀鍙楁敮鎸併€?    - - `V4L2_FBUF_CAP_BITMAP_CLIPPING`
      - 0x0008
      - 璁惧鏀寔浣跨敤浣嶆帺鐮佽繘琛岃鍓€?        娉ㄦ剰锛屾鍔熻兘涓嶅啀鍙楁敮鎸併€?    - - `V4L2_FBUF_CAP_LOCAL_ALPHA`
      - 0x0010
      - 璁惧鏀寔浣跨敤甯х紦鍐插尯鎴?VGA 淇″彿鐨?alpha 閫氶亾杩涜瑁佸壀/娣峰悎銆?	alpha 娣峰悎瀵圭牬鍧忔€?overlay 娌℃湁鎰忎箟銆?    - - `V4L2_FBUF_CAP_GLOBAL_ALPHA`
      - 0x0020
      - 璁惧鏀寔浣跨敤鍏ㄥ眬 alpha 鍊艰繘琛?alpha 娣峰悎銆?	alpha 娣峰悎瀵圭牬鍧忔€?overlay 娌℃湁鎰忎箟銆?    - - `V4L2_FBUF_CAP_LOCAL_INV_ALPHA`
      - 0x0040
      - 璁惧鏀寔浣跨敤甯х紦鍐插尯鎴?VGA 淇″彿鐨勫彇鍙?alpha 閫氶亾杩涜瑁佸壀/娣峰悎銆?	alpha 娣峰悎瀵圭牬鍧忔€?overlay 娌℃湁鎰忎箟銆?    - - `V4L2_FBUF_CAP_SRC_CHROMAKEY`
      - 0x0080
      - 璁惧鏀寔婧愯壊搴﹂敭鎺с€傚叿鏈夎壊搴﹂敭鎺ч鑹茬殑瑙嗛鍍忕礌琚抚缂撳啿鍖哄儚绱?	鏇挎崲锛岃繖涓?`V4L2_FBUF_CAP_CHROMAKEY` 姝ｅソ鐩稿弽銆?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FBUF_FLAG_PRIMARY`
      - 0x0001
      - 甯х紦鍐插尯鏄富鍥惧舰琛ㄩ潰銆傛崲鍙ヨ瘽璇达紝璇?overlay 鏄牬鍧忔€х殑銆傛鏍囧織
	閫氬父鐢变换浣曟病鏈?`V4L2_FBUF_CAP_EXTERNOVERLAY` 鑳藉姏鐨勯┍鍔ㄨ缃紝
	鍚﹀垯瀹冭娓呴櫎銆?    - - `V4L2_FBUF_FLAG_OVERLAY`
      - 0x0002
      - 濡傛灉涓鸿棰戞崟鑾疯澶囪缃簡姝ゆ爣蹇楋紝鍒欓┍鍔ㄤ細灏嗗垵濮?overlay 澶у皬璁句负
	瑕嗙洊鏁翠釜甯х紦鍐插尯澶у皬锛屽惁鍒欏皢浣跨敤鐜版湁鐨?overlay 澶у皬锛堢敱
	VIDIOC_S_FMT <VIDIOC_G_FMT> 璁剧疆锛夈€傚彧鏈変竴涓棰戞崟鑾烽┍鍔紙bttv锛?	鏀寔姝ゆ爣蹇椼€傚湪鎹曡幏璁惧涓婁娇鐢ㄦ鏍囧織宸茶寮冪敤銆傛病鏈夊姙娉曟娴嬪摢浜?	椹卞姩鏀寔姝ゆ爣蹇楋紝鍥犳璁剧疆 overlay 澶у皬鍞竴鍙潬鐨勬柟娉曟槸閫氳繃
	VIDIOC_S_FMT <VIDIOC_G_FMT>銆傚鏋滀负瑙嗛杈撳嚭璁惧璁剧疆浜嗘鏍囧織锛?	鍒欒棰戣緭鍑?overlay 绐楀彛鐩稿浜庡抚缂撳啿鍖虹殑宸︿笂瑙掞紝骞堕檺鍒朵负甯х紦鍐插尯
	鐨勫ぇ灏忋€傚鏋滄竻闄や簡瀹冿紝鍒欒棰戣緭鍑?overlay 绐楀彛鐩稿浜庤棰戣緭鍑烘樉绀恒€?    - - `V4L2_FBUF_FLAG_CHROMAKEY`
      - 0x0004
      - 浣跨敤鑹插害閿帶銆傝壊搴﹂敭鎺ч鑹茬敱 struct `v4l2_window` 鐨?	`chromakey` 瀛楁纭畾锛屽苟閫氳繃 VIDIOC_S_FMT <VIDIOC_G_FMT>
	ioctl 鍗忓晢锛屽弬瑙?overlay 鍜?osd銆?    - - `2` 娌℃湁鐢ㄤ簬閫氳繃瑁佸壀鐭╁舰鍒楄〃鎴栦綅鍥惧惎鐢ㄨ鍓殑鏍囧織銆傝繖浜涙柟娉?	閫氳繃 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鍗忓晢锛屽弬瑙?overlay
	鍜?osd銆?    - - `V4L2_FBUF_FLAG_LOCAL_ALPHA`
      - 0x0008
      - 浣跨敤甯х紦鍐插尯鐨?alpha 閫氶亾鏉ヨ鍓垨娣峰悎甯х紦鍐插尯鍍忕礌涓庤棰戝浘鍍忋€?	娣峰悎鍑芥暟涓猴細output = framebuffer pixel ** alpha + video pixel **
	(1 - alpha)銆傚疄闄呯殑 alpha 娣卞害鍙栧喅浜庡抚缂撳啿鍖哄儚绱犳牸寮忋€?    - - `V4L2_FBUF_FLAG_GLOBAL_ALPHA`
      - 0x0010
      - 浣跨敤鍏ㄥ眬 alpha 鍊煎皢甯х紦鍐插尯涓庤棰戝浘鍍忔贩鍚堛€傛贩鍚堝嚱鏁颁负锛?	output = (framebuffer pixel * alpha - video pixel * (255 - alpha)) / 255銆?	alpha 鍊肩敱 struct `v4l2_window` 鐨?`global_alpha` 瀛楁纭畾锛?	骞堕€氳繃 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鍗忓晢锛屽弬瑙?overlay
	鍜?osd銆?    - - `V4L2_FBUF_FLAG_LOCAL_INV_ALPHA`
      - 0x0020
      - 涓?`V4L2_FBUF_FLAG_LOCAL_ALPHA` 绫讳技锛屼娇鐢ㄥ抚缂撳啿鍖虹殑 alpha 閫氶亾
	鏉ヨ鍓垨娣峰悎甯х紦鍐插尯鍍忕礌涓庤棰戝浘鍍忥紝浣嗕娇鐢ㄥ彇鍙嶇殑 alpha 鍊笺€?	娣峰悎鍑芥暟涓猴細output = framebuffer pixel ** (1 - alpha) + video pixel
	** alpha銆傚疄闄呯殑 alpha 娣卞害鍙栧喅浜庡抚缂撳啿鍖哄儚绱犳牸寮忋€?    - - `V4L2_FBUF_FLAG_SRC_CHROMAKEY`
      - 0x0040
      - 浣跨敤婧愯壊搴﹂敭鎺с€傛簮鑹插害閿帶棰滆壊鐢?struct `v4l2_window` 鐨?	`chromakey` 瀛楁纭畾锛屽苟閫氳繃 VIDIOC_S_FMT <VIDIOC_G_FMT>
	ioctl 鍗忓晢锛屽弬瑙?overlay 鍜?osd銆備袱绉嶈壊搴﹂敭鎺у郊姝や簰鏂ワ紝
	鍥犳浣跨敤鐨勬槸 struct `v4l2_window` 鐨勫悓涓€涓?`chromakey` 瀛楁銆?
## Return Value


On success 0 is returned, on error -1 and the `errno` variable is set
appropriately. The generic error codes are described at the
Generic Error Codes <gen-errors> chapter.

EPERM
    VIDIOC_S_FBUF <VIDIOC_G_FBUF> 鍙兘鐢辩壒鏉冪敤鎴疯皟鐢紝浠ュ崗鍟嗙牬鍧忔€?overlay 鐨勫弬鏁般€?
EINVAL
    VIDIOC_S_FBUF <VIDIOC_G_FBUF> 鐨勫弬鏁颁笉鍚堥€傘€?