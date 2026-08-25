


######## 视频捕获接口（Video Capture Interface

视频捕获设备对模拟视频信号进行采样，并将数字化后的图像存储在内存中。如今几乎所有设备都能以完整25 30 秒进行捕获。通过该接口，应用程序可以控制捕获过程并将图像从驱动移动到用户空间
按照惯例，V4L2 视频捕获设备通过名为 `/dev/video` 以及 `/dev/video0` `/dev/video63` 的字符设备特殊文件访问，主设备号81，次设备号为 0 63。`/dev/video` 通常是指向首选视频设备的符号链接

## 查询能力


支持视频捕获接口的设备会VIDIOC_QUERYCAP ioctl 返回struct `v4l2_capability` `capabilities` 字段中设`V4L2_CAP_VIDEO_CAPTURE` `V4L2_CAP_VIDEO_CAPTURE_MPLANE` 标志。作为次要设备功能，它们也可能支持视频叠<overlay>（`V4L2_CAP_VIDEO_OVERLAY`）和原始 VBI 捕获 <raw-vbi>（`V4L2_CAP_VBI_CAPTURE`）接口。必须至少支持读/写或流式 I/O 方法之一。调谐器（tuner）和音频输入是可选的
## 辅助功能


视频捕获设备应根据需要支持音频输<audio>、tuner、控<control>、裁剪与缩放 <crop> 以及流参<streaming-par> ioctls。所有视频捕获设备都必须支持视频输入 <video> ioctls
## 图像格式协商


捕获操作的结果由裁剪和图像格式参数决定。前者选择要捕获的视频画面区域，后者决定图像如何存储在内存中，RGB YUV 格式、每像素位数或宽和高。它们一起还定义了在过程中图像如何被缩放
像往常一样，这些参数`open()` *不会**被重置，以允许多 Unix 工具链：先对设备编程，然后像读取普通文件一样读取它。编写良好的 V4L2 应用程序会确保它们真正得到想要的结果，包括裁剪和缩放
裁剪初始化至少需要将参数重置为默认值。示例见 crop
为查询当前图像格式，应用程序struct `v4l2_format` `type` 字段设为 `V4L2_BUF_TYPE_VIDEO_CAPTURE` `V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`，并以指向该结构的指针调VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl。驱动填`fmt` 联合struct `v4l2_pix_format` `pix` 成员struct `v4l2_pix_format_mplane` `pix_mp` 成员
为请求不同的参数，应用程序像上面一样设struct `v4l2_format` `type` 字段，并初始`fmt` 联合struct `v4l2_pix_format` `vbi` 成员的所有字段，或者更好地仅修VIDIOC_G_FMT <VIDIOC_G_FMT> 的结果，然后以指向该结构的指针调VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl。驱动可以调整参数，并最终像 VIDIOC_G_FMT <VIDIOC_G_FMT> 那样返回实际参数
VIDIOC_S_FMT <VIDIOC_G_FMT> 类似，VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 可用于在不禁I/O 或可能耗时的硬件准备的情况下了解硬件限制
struct `v4l2_pix_format` struct `v4l2_pix_format_mplane` 的内容在 pixfmt 中讨论。细节另VIDIOC_G_FMT <VIDIOC_G_FMT>、VIDIOC_S_FMT <VIDIOC_G_FMT> VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 的规范。视频捕获设备必须实VIDIOC_G_FMT <VIDIOC_G_FMT> VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl，即VIDIOC_S_FMT <VIDIOC_G_FMT> 忽略所有请求并总是VIDIOC_G_FMT <VIDIOC_G_FMT> 那样返回默认参数。VIDIOC_TRY_FMT <VIDIOC_G_FMT> 是可选的
## 读取图像


视频捕获设备可以支持 read() 函数 <func-read> 或流式（内存映射 <func-mmap> 或用户指<userp>）I/O。详情见 io