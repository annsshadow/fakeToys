


######## 图像裁剪、插入与缩放 CROP API


   CROP API 大多已被更新:ref:`SELECTION API
   <selection-api>` 取代。在大多数情况下应优先使用新 API   唯一的例外是像素宽高比（pixel aspect ratio）检测，它由
   VIDIOC_CROPCAP <VIDIOC_CROPCAP> 实现，在 SELECTION API 中没   对应的功能。参selection-vs-crop 以了解这两个 API 的对比
有些视频采集设备可以采样图像的一个子区域，并将其缩小或放大到任意
尺寸的图像。我们称这些能力为裁剪（cropping）与缩放（scaling）。有视频输出设备可以将图像放大或缩小，并插入到视频信号中任意的扫描行水平偏移处
应用程序可以使用以下 API 来选择视频信号中的一个区域，并查询默认区以及硬件限制

   CROP API 的名称虽然如此，VIDIOC_CROPCAP <VIDIOC_CROPCAP>   VIDIOC_G_CROP <VIDIOC_G_CROP> :ref:`VIDIOC_S_CROP
   <VIDIOC_G_CROP>` ioctl 既适用于输入设备，也适用于输出设备
缩放需要源与目标。在视频采集或叠加（overlay）设备上，源是视频信号，
裁剪 ioctl 决定实际被采样的区域。目标则是应用程序读取的图像，或叠加到图形屏幕上的图像。其尺寸（对于叠加还包括位置）由
VIDIOC_G_FMT <VIDIOC_G_FMT> 涓?VIDIOC_S_FMT <VIDIOC_G_FMT>
ioctl 协商确定
在视频输出设备上，源是应用程序传入的图像，其尺寸同样VIDIOC_G_FMT <VIDIOC_G_FMT> VIDIOC_S_FMT <VIDIOC_G_FMT>
ioctl 协商，或者可能已经编码在压缩视频流中。目标是视频信号，裁ioctl 决定图像被插入的区域
即使设备不支持缩放或 VIDIOC_G_CROP <VIDIOC_G_CROP> VIDIOC_S_CROP <VIDIOC_G_CROP> ioctl，源矩形与目标矩形也是有定义的在这种情况下，其尺寸（以及适用的位置）将是固定的

   所有支CROP SELECTION API 的采集与输出设备，也都支   VIDIOC_CROPCAP <VIDIOC_CROPCAP> ioctl
## 裁剪结构

   :alt:    crop.svg
   :align:  center

   图像裁剪、插入与缩放

   裁剪、插入与缩放的过


对于采集设备，可被采样的区域的左上角坐标、宽度与高度VIDIOC_CROPCAP <VIDIOC_CROPCAP> ioctl 返回struct
`v4l2_cropcap` `bounds` 子结构给出。为了支持广泛的
硬件，本规范并未定义原点或单位。但按照惯例，驱动应相对0H（水同步脉冲的前沿，参见 vbi-hsync）水平地统计未缩放的采样点。在垂直
方向上，使用第一个场（field）的 ITU-R 行号（参525 行的
ITU R-525 行编<vbi-525> 625 行的 <vbi-625>），如果驱动
能够采集两个场，则乘2
源矩形（即实际被采样的区域）的左上角、宽度与高度struct
`v4l2_crop` 给出，使用与 struct `v4l2_cropcap` 相同坐标系。应用程序可以使VIDIOC_G_CROP <VIDIOC_G_CROP> VIDIOC_S_CROP <VIDIOC_G_CROP> ioctl 来获取和设置这个矩形。它必须
完全落在采集边界之内，并且驱动可能会根据硬件限制进一步调整所请求尺寸或位置
每个采集设备都有一个默认的源矩形，struct `v4l2_cropcap` `defrect` 子结构给出。该矩形的中心应与视频信号有效图像区域的中心
对齐，并覆盖驱动编写者所认为的完整图像。驱动应在首次加载时将源矩形
重置为默认值，但之后不应再重置
对于输出设备，这些结构体ioctl 以类似的方式使用，定义图像将被插视频信号中的**目标**矩形

## 缩放调整


视频硬件可能具有各种各样的裁剪、插入与缩放限制。它可能只能放大或只缩小，只支持离散的缩放系数，或者在水平与垂直方向上具有不同的缩能力。也可能根本不支持缩放。与此同时，struct `v4l2_crop` 矩形可能
必须对齐，而且源矩形与目标矩形都可能有任意的上限与下限尺寸限制。特是，struct `v4l2_crop` 中最大的 `width` `height` 可能小于
struct `v4l2_cropcap` `bounds` 区域。因此，像往常一样，驱动
应调整所请求的参数并返回实际选定的值
应用程序可以先改变源矩形或目标矩形，取决于它更倾向于特定的图像尺寸
还是视频信号中的某个区域。如果驱动必须同时调整两者以满足硬件限制，则
最后请求的矩形应优先，并且驱动最好去调整另一个相反的矩形。不VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 不应改变驱动状态，因此只调所请求的矩形
假设视频采集设备上的缩放被限制为任一方向1:1 2:1 的系数，且目图像尺寸必须16 × 16 像素的倍数。源裁剪矩形被设置为默认值（在本中也是上限），即在偏0, 0 处的 640 × 400 像素。一个应用程序请300 × 225 像素的图像尺寸，假定视频会据此从“完整图像”缩小。驱动将
图像尺寸设置为最接近的可用304 × 224，然后选择最接近请求尺寸裁剪矩形，即 608 × 22424 × 2:1 会超400 的上限）。偏0, 0 仍然
有效，因此保持不变。给VIDIOC_CROPCAP <VIDIOC_CROPCAP> 报告的默裁剪矩形，应用程序可以很容易地提出另一个偏移来使裁剪矩形居中
现在应用程序可能坚持要覆盖一个使用更接近原始请求的图像宽高比的区域，
因此它请求一608 × 456 像素的裁剪矩形。当前的缩放系数将裁剪限制为
640 × 384，因此驱动返608 × 384 的裁剪尺寸，并将图像尺寸调整为最接近可用304 × 192

## 示例


源矩形与目标矩形在关闭并重新打开设备后应保持不变，这样向设备输入从设备输出数据无需特殊准备即可工作。更先进的应用程序应在开I/O 之前
确保参数是合适的

   在接下来的两个示例中，假定是一个视频采集设备；对于其他类型的设备，
   请将 `V4L2_BUF_TYPE_VIDEO_CAPTURE` 改为相应类型
## 示例：重置裁剪参

    struct v4l2_cropcap cropcap;
    struct v4l2_crop crop;

    memset (&cropcap, 0, sizeof (cropcap));
    cropcap.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    if (-1 == ioctl (fd, VIDIOC_CROPCAP, &cropcap)) {
	perror ("VIDIOC_CROPCAP");
	exit (EXIT_FAILURE);
    }

    memset (&crop, 0, sizeof (crop));
    crop.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    crop.c = cropcap.defrect;

    /** Ignore if cropping is not supported (EINVAL). **/

    if (-1 == ioctl (fd, VIDIOC_S_CROP, &crop)
	&& errno != EINVAL) {
	perror ("VIDIOC_S_CROP");
	exit (EXIT_FAILURE);
    }


## 示例：简单下缩放


    struct v4l2_cropcap cropcap;
    struct v4l2_format format;

    reset_cropping_parameters ();

    /** Scale down to 1/4 size of full picture. **/

    memset (&format, 0, sizeof (format)); /** defaults **/

    format.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    format.fmt.pix.width = cropcap.defrect.width >> 1;
    format.fmt.pix.height = cropcap.defrect.height >> 1;
    format.fmt.pix.pixelformat = V4L2_PIX_FMT_YUYV;

    if (-1 == ioctl (fd, VIDIOC_S_FMT, &format)) {
	perror ("VIDIOC_S_FORMAT");
	exit (EXIT_FAILURE);
    }

    /* We could check the actual image size now, the actual scaling factor
       or if the driver can scale at all. */

## 示例：选择一个输出区

    struct v4l2_cropcap cropcap;
    struct v4l2_crop crop;

    memset (&cropcap, 0, sizeof (cropcap));
    cropcap.type = V4L2_BUF_TYPE_VIDEO_OUTPUT;

    if (-1 == ioctl (fd, VIDIOC_CROPCAP;, &cropcap)) {
	perror ("VIDIOC_CROPCAP");
	exit (EXIT_FAILURE);
    }

    memset (&crop, 0, sizeof (crop));

    crop.type = V4L2_BUF_TYPE_VIDEO_OUTPUT;
    crop.c = cropcap.defrect;

    /* Scale the width and height to 50 % of their original size
       and center the output. */

    crop.c.width /= 2;
    crop.c.height /= 2;
    crop.c.left += crop.c.width / 2;
    crop.c.top += crop.c.height / 2;

    /** Ignore if cropping is not supported (EINVAL). **/

    if (-1 == ioctl (fd, VIDIOC_S_CROP, &crop)
	&& errno != EINVAL) {
	perror ("VIDIOC_S_CROP");
	exit (EXIT_FAILURE);
    }

## 示例：当前缩放系数与像素宽高

    struct v4l2_cropcap cropcap;
    struct v4l2_crop crop;
    struct v4l2_format format;
    double hscale, vscale;
    double aspect;
    int dwidth, dheight;

    memset (&cropcap, 0, sizeof (cropcap));
    cropcap.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    if (-1 == ioctl (fd, VIDIOC_CROPCAP, &cropcap)) {
	perror ("VIDIOC_CROPCAP");
	exit (EXIT_FAILURE);
    }

    memset (&crop, 0, sizeof (crop));
    crop.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    if (-1 == ioctl (fd, VIDIOC_G_CROP, &crop)) {
	if (errno != EINVAL) {
	    perror ("VIDIOC_G_CROP");
	    exit (EXIT_FAILURE);
	}

	/** Cropping not supported. **/

	crop.c = cropcap.defrect;
    }

    memset (&format, 0, sizeof (format));
    format.fmt.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;

    if (-1 == ioctl (fd, VIDIOC_G_FMT, &format)) {
	perror ("VIDIOC_G_FMT");
	exit (EXIT_FAILURE);
    }

    /** The scaling applied by the driver. **/

    hscale = format.fmt.pix.width / (double) crop.c.width;
    vscale = format.fmt.pix.height / (double) crop.c.height;

    aspect = cropcap.pixelaspect.numerator /
	 (double) cropcap.pixelaspect.denominator;
    aspect = aspect * hscale / vscale;

    /* Devices following ITU-R BT.601 do not capture
       square pixels. For playback on a computer monitor
       we should scale the images to this size. */

    dwidth = format.fmt.pix.width / aspect;
    dheight = format.fmt.pix.height;
