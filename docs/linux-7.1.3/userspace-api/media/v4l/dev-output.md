
######## 视频输出接口


视频输出设备将静物或图像序列编码为模拟视频信号。通过该接口，应用程序可以控制编码过程
并将图像从用户空间移动到驱动中
按照惯例，V4L2 视频输出设备通过名为 `/dev/video` `/dev/video0` `/dev/video63` 字符设备特殊文件访问，主设备号为 81，次设备号为 0 63。`/dev/video` 通常是到首选视设备的符号链接

## 查询能力


支持视频输出接口的设备在 VIDIOC_QUERYCAP ioctl 返回struct `v4l2_capability` `capabilities` 字段中设`V4L2_CAP_VIDEO_OUTPUT` `V4L2_CAP_VIDEO_OUTPUT_MPLANE` 标志作为辅助设备功能，它们也可能支持原始 VBI 输出 <raw-vbi>（`V4L2_CAP_VBI_OUTPUT`）接口必须至少支持写或流式 I/O 方法之一。调制器与音频输出是可选的
## 补充功能


视频输出设备应根据需要支持音频输<audio>、调制器 <tuner>、控<control>、裁剪与
缩放 <crop> 以及流式参数 <streaming-par> ioctl。所有视频输出设备都必须支持视频输出
<video> ioctl銆。
## 图像格式协商


输出由裁剪与图像格式参数决定。前者选择图像将出现的视频画面区域，后者决定图像如何存储于
内存中，即以 RGB 还是 YUV 格式、每像素位数或宽高。它们共同也定义了图像在处理过程中如缩放
像往常一样，这些参数`open()` *不会**被重置，以允Unix 工具链将设备编程后像写入
普通文件一样写入它。编写良好的 V4L2 应用程序会确保它们真正得到想要的结果，包括裁剪与缩放
裁剪初始化至少需要将参数重置为默认值。crop 中给出了一个示例
要查询当前图像格式，应用程序struct `v4l2_format` `type` 字段设为
`V4L2_BUF_TYPE_VIDEO_OUTPUT` `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`，并以指向该结构指针调用 VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl。驱动填struct `v4l2_pix_format` `pix`
成员struct `v4l2_pix_format_mplane` `pix_mp` 成员（属`fmt` 联合体）
要请求不同的参数，应用程序像上面那样设置 struct `v4l2_format` `type` 字段，并初始`fmt` 联合体的 struct `v4l2_pix_format` `vbi` 成员的所有字段，或者更好的做法是只修改
VIDIOC_G_FMT <VIDIOC_G_FMT> 的结果，然后以指向该结构的指针调VIDIOC_S_FMT <VIDIOC_G_FMT>
ioctl。驱动可能会调整参数，并最终像 VIDIOC_G_FMT <VIDIOC_G_FMT> 那样返回实际参数
VIDIOC_S_FMT <VIDIOC_G_FMT> 类似，VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 可用于了解硬限制，而无需禁用 I/O 或可能耗时的硬件准备
struct `v4l2_pix_format` struct `v4l2_pix_format_mplane` 的内容在 pixfmt 中讨论。有细节另请参见 VIDIOC_G_FMT <VIDIOC_G_FMT>、VIDIOC_S_FMT <VIDIOC_G_FMT> VIDIOC_TRY_FMT
<VIDIOC_G_FMT> ioctl 的规范。视频输出设备必须实VIDIOC_G_FMT <VIDIOC_G_FMT> VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl，即VIDIOC_S_FMT <VIDIOC_G_FMT> 忽略所有请求并总是
VIDIOC_G_FMT <VIDIOC_G_FMT> 那样返回默认参数。VIDIOC_TRY_FMT <VIDIOC_G_FMT> 是可选的
## 写入图像


视频输出设备可能支持 write() 函数 <rw> 或流式（内存映射 <mmap> 或用户指<userp>）I/O。详io