

######## ioctl VIDIOC_ENUM_FMT


## Name


VIDIOC_ENUM_FMT - 枚举图像格式

## Synopsis



`int ioctl(int fd, VIDIOC_ENUM_FMT, struct v4l2_fmtdesc *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符

`argp`
    指向 struct `v4l2_fmtdesc` 的指针

## Description


为了枚举图像格式，应用程序初始化 struct `v4l2_fmtdesc` `type`、`mbus_code` `index`
字段，并使用指向该结构的指针调用 VIDIOC_ENUM_FMT ioctl。驱动填充结构的其余部分，或返回
`EINVAL` 错误码。所有格式都可通过index 0 开始每次加 1 直到返回 `EINVAL` 来枚举。如
适用，驱动应按优先顺序返回格式，其中优先格式在（即使用更小的 `index` 值）非优先格式之
返回

根据 `V4L2_CAP_IO_MC` 能力 <device-capabilities>，`mbus_code` 字段的处理方式不同：

1) 未设`V4L2_CAP_IO_MC`（也称为“video-node-centric”驱动）

   应用程序应将 `mbus_code` 字段初始化为零，驱动应忽略该字段的值

   驱动应枚举所有图像格式

```

      After switching the input or output the list of enumerated image
      formats may be different.

```
2) 设置`V4L2_CAP_IO_MC`（也称为“MC-centric”驱动）

   如果 `mbus_code` 字段为零，则应枚举所有图像格式

   如果 `mbus_code` 字段被初始化为一个有效的（非零）媒体总线格式
   <v4l2-mbus-pixelcode>，则驱动应将枚举限制为只能生成（对于视频输出设备）或只能由（对于
   视频捕获设备）该媒体总线码产被产生的图像格式。如果驱动不支持`mbus_code`，则
   返回 `EINVAL`

   无论 `mbus_code` 字段的值如何，枚举出的图像格式不应依赖于视频设备或设备流水线的活动
   配置




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 枚举中格式的编号，由应用程序设置。这`pixelformat` 字段毫无关系。当 index 
        `V4L2_FMTDESC_FLAG_ENUM_ALL` 进行 OR 运算时，驱动清除该标志并枚举所有可能的格式
        忽略当前配置带来的任何限制。不支持该标志的驱动总是返回 `EINVAL` 错误码，且不清除
        该标志。使`V4L2_FMTDESC_FLAG_ENUM_ALL` 标志枚举的格式不应在调用
        `VIDIOC_ENUM_FRAMESIZES` `VIDIOC_ENUM_FRAMEINTERVALS` 时使用。`V4L2_FMTDESC_FLAG_ENUM_ALL`
        只应由能根据该标志返回不同格式列表的驱动使用
    - - __u32
      - `type`
      - 数据流的类型，由应用程序设置。此处只有以下类型是有效的：`V4L2_BUF_TYPE_VIDEO_CAPTURE`
        `V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`、`V4L2_BUF_TYPE_VIDEO_OUTPUT`
        `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`、`V4L2_BUF_TYPE_VIDEO_OVERLAY`
        `V4L2_BUF_TYPE_SDR_CAPTURE`、`V4L2_BUF_TYPE_SDR_OUTPUT`、`V4L2_BUF_TYPE_META_CAPTURE`
        `V4L2_BUF_TYPE_META_OUTPUT`。参`v4l2_buf_type`
    - - __u32
      - `flags`
      - 参见 fmtdesc-flags
    - - __u8
      - `description`\ [^32^]
      - 格式的描述，一个以 NUL 结尾ASCII 字符串。此信息供用户使用，例如：“YUV 4:2:2”
    - - __u32
      - `pixelformat`
      - 图像格式标识符。这是一个由 v4l2_fourcc() 宏计算出的四字符码：
    - - `2`

	.. _v4l2-fourcc:

	`#define v4l2_fourcc(a,b,c,d)`

	`(((__u32)(a)<<0)|((__u32)(b)<<8)|((__u32)(c)<<16)|((__u32)(d)<<24))`

	本规范已pixfmt 中定义了若干图像格式

```

	   These codes are not the same as those used
	   in the Windows world.
    * - __u32
      - ``mbus_code``
      - Media bus code restricting the enumerated formats, set by the
        application. Only applicable to drivers that advertise the
        ``V4L2_CAP_IO_MC`` :ref:`capability <device-capabilities>`, shall be 0
        otherwise.
    * - __u32
      - ``reserved``\ [3]
      - Reserved for future extensions. Drivers must set the array to
	zero.


```



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FMT_FLAG_COMPRESSED`
      - 0x0001
      - 这是一个压缩格式
    - - `V4L2_FMT_FLAG_EMULATED`
      - 0x0002
      - 该格式并非设备的原生格式，而是通过软件（通常libv4l2）模拟的，在可能的情况下
        尽量使用原生格式以获得更好性能
    - - `V4L2_FMT_FLAG_CONTINUOUS_BYTESTREAM`
      - 0x0004
      - 该压缩字节流格式（又coded 格式）的硬件解码器能够解析连续的字节流。应用程序无需
        自行解析字节流来查找场之间的边界

	该标志只能与 `V4L2_FMT_FLAG_COMPRESSED` 标志组合使用，因为它仅适用于压缩格式。该
	标志仅对 stateful 解码器有效
    - - `V4L2_FMT_FLAG_DYN_RESOLUTION`
      - 0x0008
      - 设备支持该压缩字节流格式（又coded 格式）的动态分辨率切换。当检测到视频参数变化时，
        它会通过事件 `V4L2_EVENT_SOURCE_CHANGE` 通知用户

	该标志只能与 `V4L2_FMT_FLAG_COMPRESSED` 标志组合使用，因为它仅适用于压缩格式。该
	标志仅对 stateful 编解码器有效
    - - `V4L2_FMT_FLAG_ENC_CAP_FRAME_INTERVAL`
      - 0x0010
      - 硬件编码器支持将 `CAPTURE` coded 帧间隔与 `OUTPUT` 原始帧间隔分开设置。使
        VIDIOC_S_PARM <VIDIOC_G_PARM> 设置 `OUTPUT` 原始帧间隔也会将 `CAPTURE` coded 帧间
        设为相同的值。如果设置了该标志，则之后可以将 `CAPTURE` coded 帧间隔设为不同的值。这
        通常用于离线编码，其`OUTPUT` 原始帧间隔用作保留硬件编码器资源的提示，
        `CAPTURE` coded 帧间隔是嵌入在编码视频流中的实际帧率

	该标志只能与 `V4L2_FMT_FLAG_COMPRESSED` 标志组合使用，因为它仅适用于压缩格式。该
        标志仅对 stateful 编码器有效
    - - `V4L2_FMT_FLAG_CSC_COLORSPACE`
      - 0x0020
      - 驱动允许应用程序尝试更改默认色彩空间。该标志仅与捕获设备相关。应用程序可以在调用
        VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 并设V4L2_PIX_FMT_FLAG_SET_CSC
        <v4l2-pix-fmt-flag-set-csc> 时，请求配置捕获设备的色彩空间
    - - `V4L2_FMT_FLAG_CSC_XFER_FUNC`
      - 0x0040
      - 驱动允许应用程序尝试更改默认传递函数（transfer function）。该标志仅与捕获设备相关
        应用程序可以在调VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 并设
        V4L2_PIX_FMT_FLAG_SET_CSC <v4l2-pix-fmt-flag-set-csc> 时，请求配置捕获设备的传递函数
    - - `V4L2_FMT_FLAG_CSC_YCBCR_ENC`
      - 0x0080
      - 驱动允许应用程序尝试更改默认Y'CbCr 编码。该标志仅与捕获设备相关。应用程序可以在
        调用 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 并设V4L2_PIX_FMT_FLAG_SET_CSC
        <v4l2-pix-fmt-flag-set-csc> 时，请求配置捕获设备Y'CbCr 编码
    - - `V4L2_FMT_FLAG_CSC_HSV_ENC`
      - 0x0080
      - 驱动允许应用程序尝试更改默认HSV 编码。该标志仅与捕获设备相关。应用程序可以在调用
        VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 并设V4L2_PIX_FMT_FLAG_SET_CSC
        <v4l2-pix-fmt-flag-set-csc> 时，请求配置捕获设备HSV 编码
    - - `V4L2_FMT_FLAG_CSC_QUANTIZATION`
      - 0x0100
      - 驱动允许应用程序尝试更改默认的量化。该标志仅与捕获设备相关。应用程序可以在调用
        VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 并设V4L2_PIX_FMT_FLAG_SET_CSC
        <v4l2-pix-fmt-flag-set-csc> 时，请求配置捕获设备的量化
    - - `V4L2_FMT_FLAG_META_LINE_BASED`
      - 0x0200
      - 元数据格式是基于行的。在这种情况`v4l2_meta_format` `width`、`height` 
        `bytesperline` 字段是有效的。缓冲区`height` 行组成，每行`width` 个数据单元，
        每两个连续行之间的偏移量（字节）`bytesperline`
    - - `V4L2_FMTDESC_FLAG_ENUM_ALL`
      - 0x80000000
      - 当应用程序将 `index` `V4L2_FMTDESC_FLAG_ENUM_ALL` 标志进行 OR 运算时，驱动枚举
        所有可能的像素格式，而不考虑任何已设置的配置。不支持该标志的驱动总是返回 `EINVAL`
        且不清除该标志

## Return Value


成功时返0，出错时返回 -1 并设`errno`。通用错误码在 Generic Error Codes
<gen-errors> 一章中描述

EINVAL
    struct `v4l2_fmtdesc` `type` 不被支持，或 `index` 越界

    如果设置`V4L2_CAP_IO_MC` 且指定的 `mbus_code` 不被支持，则也返回此错误码
