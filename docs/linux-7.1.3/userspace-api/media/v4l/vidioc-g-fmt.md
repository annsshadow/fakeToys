


######## ioctl VIDIOC_G_FMT, VIDIOC_S_FMT, VIDIOC_TRY_FMT


## Name


VIDIOC_G_FMT - VIDIOC_S_FMT - VIDIOC_TRY_FMT - 获取或设置数据格式，尝试一种格
## Synopsis



`int ioctl(int fd, VIDIOC_G_FMT, struct v4l2_format *argp)`


`int ioctl(int fd, VIDIOC_S_FMT, struct v4l2_format *argp)`


`int ioctl(int fd, VIDIOC_TRY_FMT, struct v4l2_format *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_format` 的指针
## Description


这些 ioctl 用于协商驱动与应用程序之间交换的数据（通常是图像）格式
要查询当前参数，应用程序struct `v4l2_format` `type` 字段设置为相应的缓冲区（流）类型。例如视频采集设备使`V4L2_BUF_TYPE_VIDEO_CAPTURE` `V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`。当应用程序调用带有指向该结构的指针VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl 时，驱动会填`fmt` 联合的相应成员。对于视频采集设备，该成员是 struct `v4l2_pix_format` `pix` struct `v4l2_pix_format_mplane` `pix_mp` 成员。当所请求的缓冲区类型不受支持时，驱动返回 `EINVAL` 错误码
要更改当前格式参数，应用程序初始`type` 字段以及相应 `fmt` 联合成员的所有字段。细节请参阅 devices 中各种设备类型的文档。好的做法是以先查询当前参数，然后只修改那些不适合应用程序的参数为准。当应用程序调用带有指向 struct `v4l2_format` 结构的指针的 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 时，驱动会根据硬件能力检查并调整参数。除`type` 字段无效，否则驱动不应返回错误码，这是一种探测设备能力并接近应用程序和驱动都可接受参数的机制。成功时，驱动可以编程硬件、分配资源，并通常为数据交换做准备。最后，VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl VIDIOC_G_FMT <VIDIOC_G_FMT> 那样返回当前格式参数。非常简单的、不灵活的设备甚至可能忽略所有输入，并总是返回默认参数。然而，所有与应用程序交换数据V4L2 设备都必须实VIDIOC_G_FMT <VIDIOC_G_FMT> VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl。当所请求的缓冲区类型不受支持时，驱动VIDIOC_S_FMT <VIDIOC_G_FMT> 尝试时返`EINVAL` 错误码。当 I/O 已在进行中，或因其他原因资源不可用时，驱动返`EBUSY` 错误码
VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 等同VIDIOC_S_FMT <VIDIOC_G_FMT>，只有一个例外：它不改变驱动状态。它也可以在任何时候调用，绝不会返`EBUSY`。提供此函数是为了在不禁I/O 或可能耗时的硬件准备的情况下，协商参数、了解硬件限制。尽管强烈推荐，驱动并不要求实现ioctl
VIDIOC_TRY_FMT <VIDIOC_G_FMT> 返回的格式必须与 VIDIOC_S_FMT <VIDIOC_G_FMT> 对相同输入或输出返回的格式完全相同


    :header-rows:  0
    :stub-columns: 0

    - - __u32
      - `type`
      - 数据流的类型，参`v4l2_buf_type`    - - union {
      - `fmt`
    - - struct `v4l2_pix_format`
      - `pix`
      - 图像格式的定义，参见 pixfmt，用于视频采集和输出设备    - - struct `v4l2_pix_format_mplane`
      - `pix_mp`
      - 图像格式的定义，参见 pixfmt，用于支        多平面版API <planar-apis> 的视频采集和输出设备    - - struct `v4l2_window`
      - `win`
      - 叠加图像的定义，参见 overlay，用于视频叠加设备    - - struct `v4l2_vbi_format`
      - `vbi`
      - 原始 VBI 采集或输出参数。这raw-vbi 中有更详细的讨论。用于原VBI 采集和输出设备    - - struct `v4l2_sliced_vbi_format`
      - `sliced`
      - 切片 VBI 采集或输出参数。细节参sliced。用于切VBI 采集和输出设备    - - struct `v4l2_sdr_format`
      - `sdr`
      - 数据格式的定义，参见 pixfmt，用SDR 采集和输出设备    - - struct `v4l2_meta_format`
      - `meta`
      - 元数据格式的定义，参meta-formats，用于元数据采集设备    - - __u8
      - `raw_data`\ [^200^]
      - 为未来扩展保留的占位符    - - }
      -

## Return Value


成功时返0，出错时返回 -1，并`errno` 变量会被相应地设置。通用错误码在 Generic Error Codes <gen-errors> 一章中描述
EINVAL
    struct `v4l2_format` `type` 字段无效，或所请求的缓冲区类型不受支持
EBUSY
    设备正忙，无法更改格式。这可能是因为设备正在流式传输，或者缓冲区已分配或已入队到驱动。仅:ref:`VIDIOC_S_FMT <VIDIOC_G_FMT>` 相关