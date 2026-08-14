


######## 视频输出叠加接口（Video Output Overlay Interface）


**也称为屏上显示（On-Screen Display，OSD）**

某些视频输出设备可以将帧缓冲（framebuffer）图像叠加到
传出的视频信号上。应用程序可以使用此接口设置这样的叠加，
该接口借用了视频叠加 <overlay> 接口的结构与 ioctl。

OSD 功能通过访问视频输出 <capture> 功能的同一个字符设备
特殊文件来访问。

   这样的 `/dev/video` 设备的默认功能是视频捕获或
   输出。OSD 功能只有在使用 VIDIOC_S_FMT <VIDIOC_G_FMT>
   ioctl 调用之后才可用。

## 查询能力


支持**视频输出叠加**接口的设备会在
VIDIOC_QUERYCAP ioctl 返回的 struct `v4l2_capability` 的
`capabilities` 字段中设置 `V4L2_CAP_VIDEO_OUTPUT_OVERLAY` 标志。

## 帧缓冲


与**视频叠加**接口相反，帧缓冲通常实现在电视卡上而非
显卡上。在 Linux 上，它作为帧缓冲设备（`/dev/fbN`）可访问。
给定一个 V4L2 设备，应用程序可以通过调用
VIDIOC_G_FBUF <VIDIOC_G_FBUF> ioctl 找到对应的帧缓冲设备。
除其他信息外，它返回帧缓冲在 struct `v4l2_framebuffer`
的 `base` 字段中的物理地址。
帧缓冲设备 ioctl `FBIOGET_FSCREENINFO` 在 struct
`fb_fix_screeninfo` 的 `smem_start` 字段中返回相同的地址。
`FBIOGET_FSCREENINFO` ioctl 与 struct `fb_fix_screeninfo`
定义在 `linux/fb.h` 头文件中。

帧缓冲的宽度与高度取决于当前的视频标准。V4L2 驱动可能会拒绝
更改视频标准（或任何其他意味着帧缓冲大小变化的 ioctl）的尝试，
返回 `EBUSY` 错误码，直到所有应用程序都关闭了帧缓冲设备。

### 示例：为 OSD 寻找帧缓冲设备



    #include <linux/fb.h>

    struct v4l2_framebuffer fbuf;
    unsigned int i;
    int fb_fd;

    if (-1 == ioctl(fd, VIDIOC_G_FBUF, &fbuf)) {
	perror("VIDIOC_G_FBUF");
	exit(EXIT_FAILURE);
    }

    for (i = 0; i < 30; i++) {
	char dev_name[^16^];
	struct fb_fix_screeninfo si;

	snprintf(dev_name, sizeof(dev_name), "/dev/fb%u", i);

	fb_fd = open(dev_name, O_RDWR);
	if (-1 == fb_fd) {
	    switch (errno) {
	    case ENOENT: /** 无此文件 **/
	    case ENXIO:  /** 无驱动 **/
		continue;

	    default:
		perror("open");
		exit(EXIT_FAILURE);
	    }
	}

	if (0 == ioctl(fb_fd, FBIOGET_FSCREENINFO, &si)) {
	    if (si.smem_start == (unsigned long)fbuf.base)
		break;
	} else {
	    /** 显然不是一个帧缓冲设备。 **/
	}

	close(fb_fd);
	fb_fd = -1;
    }

    /* fb_fd 是视频输出叠加的帧缓冲设备的文件描述符，
       如果未找到设备则为 -1。 */


## 叠加窗口与缩放


叠加由源矩形与目标矩形控制。源矩形选择要叠加的帧缓冲图像的
一个子区域，目标矩形选择图像将出现的传出视频信号中的一个区域。
驱动可能支持也可能不支持缩放，以及这些矩形的任意大小和位置。
此外，驱动可能支持（也可能不支持）为视频叠加 <overlay> 接口定义的
任何（或没有）裁剪/混合方法。

struct `v4l2_window` 定义源矩形的大小、它在帧缓冲中的位置，
以及用于叠加的裁剪/混合方法。要获取当前参数，应用程序将
struct `v4l2_format` 的 `type` 字段设置为
`V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY` 并调用
VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl。驱动填充名为 `win` 的
struct `v4l2_window` 子结构。无法取回先前编程的裁剪列表或位图。

要编程源矩形，应用程序将 struct `v4l2_format` 的 `type` 字段
设置为 `V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY`，初始化 `win`
子结构并调用 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl。
驱动根据硬件限制调整参数，并像 VIDIOC_G_FMT <VIDIOC_G_FMT> 那样
返回实际参数。与 VIDIOC_S_FMT <VIDIOC_G_FMT> 类似，
VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 可用于在不实际改变驱动状态的
情况下了解驱动能力。与 VIDIOC_S_FMT <VIDIOC_G_FMT> 不同，这也可以在
叠加启用之后工作。

struct `v4l2_crop` 定义目标矩形的大小与位置。叠加的缩放因子
由 struct `v4l2_window` 与 struct `v4l2_crop` 中给定的宽度和高度
隐含。裁剪 API 对**视频输出**与**视频输出叠加**设备的应用方式，
与对**视频捕获**与**视频叠加**设备相同，只是反转了
数据流的方向。更多信息请参见 crop。

## 启用叠加


没有用于启用或禁用叠加的 V4L2 ioctl，但驱动的帧缓冲接口
可能支持 `FBIOBLANK` ioctl。
