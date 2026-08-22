


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
      - 由驱动设置的 overlay 能力标志，参framebuffer-cap    - - __u32
      - `flags`
      -
      - 由应用程序和驱动设置overlay 控制标志，参framebuffer-flags
    - - void *
      - `base`
      -
      - 帧缓冲区的物理基地址，即帧缓冲区左上角像素的地址	对于 VIDIOC_S_FBUF <VIDIOC_G_FBUF> 此字段不再受支持	内核将始终将其设NULL	对于 **Video Output Overlays**	驱动将返回一个有效的基地址，以便应用程序可以找到对应的
	Linux 帧缓冲设备（参见 osd）。对**Video Capture Overlays**
	此字段将始终NULL    - - struct
      - `fmt`
      -
      - 帧缓冲区的布局    - -
      - __u32
      - `width`
      - 帧缓冲区的宽度，以像素计    - -
      - __u32
      - `height`
      - 帧缓冲区的高度，以像素计    - -
      - __u32
      - `pixelformat`
      - 帧缓冲区的像素格式#     * -

      -
      - 对于 **non-destructive Video Overlays**，此字段仅为
	struct `v4l2_window` 鐨?`chromakey`
	字段定义一个格式#     * -

      -
      - 对于 **Video Output Overlays**，驱动必须返回一个有效的格式#     * -

      -
      - 通常这是一RGB 格式（例	V4L2_PIX_FMT_RGB565 <V4L2-PIX-FMT-RGB565>），YUV
	格式（仅当使用色度键控时packed YUV 格式，不包括
	`V4L2_PIX_FMT_YUYV` `V4L2_PIX_FMT_UYVY`）以	`V4L2_PIX_FMT_PAL8` 格式也允许使用。当应用程序请求压缩格式	驱动的行为是未定义的。关于像素格式的信息参见 pixfmt    - -
      - enum `v4l2_field`
      - `field`
      - 驱动和应用程序应忽略此字段。如适用，字段顺序由
	VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 使用 struct `v4l2_window`
	`field` 字段选择    - -
      - __u32
      - `bytesperline`
      - 两条相邻扫描线最左侧像素之间的距离，以字节计    - - `3`

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
      - 该信息补`pixelformat`，必须由驱动设置，参colorspaces    - -
      - __u32
      - `priv`
      - 保留。驱动和应用程序必须将此字段设为零


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FBUF_CAP_EXTERNOVERLAY`
      - 0x0001
      - 设备支持非破坏overlay。当驱动清除此标志时，仅支持破坏	overlay。目前还没有同时支持破坏overlay 和非破坏overlay 	驱动。实际上 Video Output Overlays 总是非破坏性的    - - `V4L2_FBUF_CAP_CHROMAKEY`
      - 0x0002
      - 设备支持通过色度键控对图像进行裁剪。即，仅在后者呈现某种特	颜色的位置，图像像素才替VGA 或视频信号中的像素。色度键	对破坏overlay 没有意义    - - `V4L2_FBUF_CAP_LIST_CLIPPING`
      - 0x0004
      - 设备支持使用裁剪矩形列表进行裁剪        注意，此功能不再受支持    - - `V4L2_FBUF_CAP_BITMAP_CLIPPING`
      - 0x0008
      - 设备支持使用位掩码进行裁剪        注意，此功能不再受支持    - - `V4L2_FBUF_CAP_LOCAL_ALPHA`
      - 0x0010
      - 设备支持使用帧缓冲区VGA 信号alpha 通道进行裁剪/混合	alpha 混合对破坏overlay 没有意义    - - `V4L2_FBUF_CAP_GLOBAL_ALPHA`
      - 0x0020
      - 设备支持使用全局 alpha 值进alpha 混合	alpha 混合对破坏overlay 没有意义    - - `V4L2_FBUF_CAP_LOCAL_INV_ALPHA`
      - 0x0040
      - 设备支持使用帧缓冲区VGA 信号的取alpha 通道进行裁剪/混合	alpha 混合对破坏overlay 没有意义    - - `V4L2_FBUF_CAP_SRC_CHROMAKEY`
      - 0x0080
      - 设备支持源色度键控。具有色度键控颜色的视频像素被帧缓冲区像	替换，这`V4L2_FBUF_CAP_CHROMAKEY` 正好相反


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FBUF_FLAG_PRIMARY`
      - 0x0001
      - 帧缓冲区是主图形表面。换句话说，overlay 是破坏性的。此标志
	通常由任何没`V4L2_FBUF_CAP_EXTERNOVERLAY` 能力的驱动设置，
	否则它被清除    - - `V4L2_FBUF_FLAG_OVERLAY`
      - 0x0002
      - 如果为视频捕获设备设置了此标志，则驱动会将初overlay 大小设为
	覆盖整个帧缓冲区大小，否则将使用现有overlay 大小（由
	VIDIOC_S_FMT <VIDIOC_G_FMT> 设置）。只有一个视频捕获驱动（bttv	支持此标志。在捕获设备上使用此标志已被弃用。没有办法检测哪	驱动支持此标志，因此设置 overlay 大小唯一可靠的方法是通过
	VIDIOC_S_FMT <VIDIOC_G_FMT>。如果为视频输出设备设置了此标志	则视频输overlay 窗口相对于帧缓冲区的左上角，并限制为帧缓冲区
	的大小。如果清除了它，则视频输overlay 窗口相对于视频输出显示    - - `V4L2_FBUF_FLAG_CHROMAKEY`
      - 0x0004
      - 使用色度键控。色度键控颜色由 struct `v4l2_window` 	`chromakey` 字段确定，并通过 VIDIOC_S_FMT <VIDIOC_G_FMT>
	ioctl 协商，参overlay osd    - - `2` 没有用于通过裁剪矩形列表或位图启用裁剪的标志。这些方	通过 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 协商，参overlay
	鍜?osd銆?    - - `V4L2_FBUF_FLAG_LOCAL_ALPHA`
      - 0x0008
      - 使用帧缓冲区alpha 通道来裁剪或混合帧缓冲区像素与视频图像	混合函数为：output = framebuffer pixel ** alpha + video pixel **
	(1 - alpha)。实际的 alpha 深度取决于帧缓冲区像素格式    - - `V4L2_FBUF_FLAG_GLOBAL_ALPHA`
      - 0x0010
      - 使用全局 alpha 值将帧缓冲区与视频图像混合。混合函数为	output = (framebuffer pixel * alpha - video pixel * (255 - alpha)) / 255	alpha 值由 struct `v4l2_window` `global_alpha` 字段确定	并通过 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 协商，参overlay
	鍜?osd銆?    - - `V4L2_FBUF_FLAG_LOCAL_INV_ALPHA`
      - 0x0020
      - `V4L2_FBUF_FLAG_LOCAL_ALPHA` 类似，使用帧缓冲区的 alpha 通道
	来裁剪或混合帧缓冲区像素与视频图像，但使用取反的 alpha 值	混合函数为：output = framebuffer pixel ** (1 - alpha) + video pixel
	** alpha。实际的 alpha 深度取决于帧缓冲区像素格式    - - `V4L2_FBUF_FLAG_SRC_CHROMAKEY`
      - 0x0040
      - 使用源色度键控。源色度键控颜色struct `v4l2_window` 	`chromakey` 字段确定，并通过 VIDIOC_S_FMT <VIDIOC_G_FMT>
	ioctl 协商，参overlay osd。两种色度键控彼此互斥，
	因此使用的是 struct `v4l2_window` 的同一`chromakey` 字段
## Return Value


On success 0 is returned, on error -1 and the `errno` variable is set
appropriately. The generic error codes are described at the
Generic Error Codes <gen-errors> chapter.

EPERM
    VIDIOC_S_FBUF <VIDIOC_G_FBUF> 只能由特权用户调用，以协商破坏overlay 的参数
EINVAL
    VIDIOC_S_FBUF <VIDIOC_G_FBUF> 的参数不合适