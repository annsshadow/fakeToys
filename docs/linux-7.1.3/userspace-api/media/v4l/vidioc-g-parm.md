

######## ioctl VIDIOC_G_PARM, VIDIOC_S_PARM


## 名称


VIDIOC_G_PARM - VIDIOC_S_PARM - 获取或设置流参数

## 概要


`int ioctl(int fd, VIDIOC_G_PARM, v4l2_streamparm *argp)`


`int ioctl(int fd, VIDIOC_S_PARM, v4l2_streamparm *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_streamparm` 的指针。

## 描述


应用程序可以请求一个不同的帧间隔。如果可能，捕获或输出设备将被重新配置以支持
所请求的帧间隔。驱动也可以选择性地跳过或重复帧，以实现所请求的帧间隔。

对于有状态编码器（参见 encoder），这表示通常嵌入在编码视频流中的帧间隔。

更改帧间隔绝不应更改格式。另一方面，更改格式可能会更改帧间隔。

此外，这些 ioctl 还可用于确定驱动在 read/write 模式下内部使用的缓冲区数量。其
影响参见讨论 `read()` 函数的章节。

为了获取和设置流参数，应用程序分别调用 VIDIOC_G_PARM <VIDIOC_G_PARM> 和
VIDIOC_S_PARM <VIDIOC_G_PARM> ioctl。它们接受一个指向 struct `v4l2_streamparm`
的指针，该结构体包含一个联合体，持有用于输入和输出设备的独立参数。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 缓冲区（流）类型，与 struct
	`v4l2_format` `type` 相同，由应用程序设置。参见 `v4l2_buf_type`。
    - - union {
      - `parm`
    - - struct `v4l2_captureparm`
      - `capture`
      - 捕获设备的参数，在 `type` 为
	`V4L2_BUF_TYPE_VIDEO_CAPTURE` 或
	`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE` 时使用。
    - - struct `v4l2_outputparm`
      - `output`
      - 输出设备的参数，在 `type` 为
	`V4L2_BUF_TYPE_VIDEO_OUTPUT` 或 `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE` 时使用。
    - - __u8
      - `raw_data`\ [^200^]
      - 用于未来扩展的占位符。
    - - }



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `capability`
      - 参见 parm-caps。
    - - __u32
      - `capturemode`
      - 由驱动和应用程序设置，参见 parm-flags。
    - - struct `v4l2_fract`
      - `timeperframe`
      - 这是驱动捕获连续帧之间的期望周期，以秒为单位。
    - - `2`

	这将配置视频源（例如传感器）生成视频帧的速度。如果速度是固定的，那么驱动
	可能会选择跳过或重复帧，以实现所请求的帧率。

	对于有状态编码器（参见 encoder），这表示通常嵌入在编码视频流中的帧间隔。

	应用程序在此存储期望的帧周期，驱动返回实际的帧周期。

	更改视频标准（也通过切换视频输入隐式地）可能会将此参数重置为标称帧周期。
	若要手动重置，应用程序只需将此字段设为零。

	驱动只有在它们在 `capability` 字段中设置了 `V4L2_CAP_TIMEPERFRAME` 标志时
	才支持此功能。
    - - __u32
      - `extendedmode`
      - 自定义（驱动特定的）流参数。未使用时，应用程序和驱动必须将此字段设为零。
	使用此字段的应用程序应检查驱动名称和版本，参见 querycap。
    - - __u32
      - `readbuffers`
      - 应用程序将此字段设置为驱动在 `read()` 模式下内部使用的期望缓冲区数量。
	驱动返回实际的缓冲区数量。当应用程序请求零个缓冲区时，驱动应该只返回
	当前设置，而不是最小值或错误码。详见 rw。
    - - __u32
      - `reserved`\ [^4^]
      - 为未来扩展保留。驱动和应用程序必须将数组设为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `capability`
      - 参见 parm-caps。
    - - __u32
      - `outputmode`
      - 由驱动和应用程序设置，参见 parm-flags。
    - - struct `v4l2_fract`
      - `timeperframe`
      - 这是驱动输出连续帧之间的期望周期，以秒为单位。
    - - `2`

	该字段旨在 `write()` 模式下在驱动侧重复帧（在流模式下可以使用时间戳来调节
	输出），以节省 I/O 带宽。

	对于有状态编码器（参见 encoder），这表示通常嵌入在编码视频流中的帧间隔，
	并为编码器提供原始帧排队进入编码器的速度提示。

	应用程序在此存储期望的帧周期，驱动返回实际的帧周期。

	更改视频标准（也通过切换视频输出隐式地）可能会将此参数重置为标称帧周期。
	若要手动重置，应用程序只需将此字段设为零。

	驱动只有在它们在 `capability` 字段中设置了 `V4L2_CAP_TIMEPERFRAME` 标志时
	才支持此功能。
    - - __u32
      - `extendedmode`
      - 自定义（驱动特定的）流参数。未使用时，应用程序和驱动必须将此字段设为零。
	使用此字段的应用程序应检查驱动名称和版本，参见 querycap。
    - - __u32
      - `writebuffers`
      - 应用程序将此字段设置为驱动在 `write()` 模式下内部使用的期望缓冲区数量。
	驱动返回实际的缓冲区数量。当应用程序请求零个缓冲区时，驱动应该只返回当前
	设置，而不是最小值或错误码。详见 rw。
    - - __u32
      - `reserved`\ [^4^]
      - 为未来扩展保留。驱动和应用程序必须将数组设为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CAP_TIMEPERFRAME`
      - 0x1000
      - 帧周期可以通过设置 `timeperframe` 字段来修改。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_MODE_HIGHQUALITY`
      - 0x0001
      - 高质量成像模式。高质量模式用于静态成像应用。其想法是获得硬件能够提供的
	尽可能最好的图像质量。驱动作者如何实现这一点并未定义；它将取决于硬件以及
	驱动作者的巧思。高质量模式是与常规运动视频捕获模式不同的模式。在高质量
	模式下：

 - 驱动可能能够捕获比运动捕获更高的分辨率。

 - 驱动可能支持比运动捕获更少的像素格式（例如；真彩色）。

 - 驱动可能捕获并算术地组合多个连续的场或帧，以消除彩色边缘伪影并减少视频数据
	   中的噪声。

 - 驱动可能像扫描仪一样分片捕获图像，以处理原本不可能处理的更大格式图像。

 - 图像捕获操作可能比运动捕获显著更慢。

 - 图像中的运动物体可能会有过度的运动模糊。

 - 捕获可能只能通过 `read()` 调用来工作。

## 返回值


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用错误码在通用错误码
<gen-errors> 章节中描述。
