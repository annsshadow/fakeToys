
######## ioctl VIDIOC_CROPCAP


## 名称


VIDIOC_CROPCAP - 关于视频裁剪与缩放能力的信息

## 语法


`int ioctl(int fd, VIDIOC_CROPCAP, struct v4l2_cropcap *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_cropcap` 的指针
## 描述


应用程序使用此函数查询裁剪限制、图像像素宽高比并计算缩放因子。它们将 v4l2_cropcap 结构`type` 字段设为相应的缓冲区（流）类型，并以指向该结构的指针调用 VIDIOC_CROPCAP ioctl驱动填充结构的其余部分。除切换视频标准外，结果是恒定的。请记住，切换视频输入或输出可能会隐式发生这种切换
ioctl 必须由支持裁剪和/或缩放和/或具有非方形像素的视频捕获或输出设备，以及覆盖（overlay设备实现


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 数据流的类型，由应用程序设置。此处仅以下类型有效：`V4L2_BUF_TYPE_VIDEO_CAPTURE`	`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`、`V4L2_BUF_TYPE_VIDEO_OUTPUT`	`V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE` `V4L2_BUF_TYPE_VIDEO_OVERLAY`	参见 `v4l2_buf_type` 及下面的说明    - - struct v4l2_rect <v4l2-rect-crop>
      - `bounds`
      - 定义可进行捕获或输出的窗口，这可能排除例如水平和垂直消隐区。裁剪矩形不能超过这	限制。宽度与高度以像素定义，驱动编写者可自由选择在模拟域中坐标系的原点与单位    - - struct v4l2_rect <v4l2-rect-crop>
      - `defrect`
      - 默认裁剪矩形，它应覆盖“整幅画面”。假设像素宽高比1/1，对NTSC 可以是例	640 × 480 的矩形，对于 PAL SECAM 可以是居中于活动画面区域768 × 576 矩形	使用`bounds` 相同的坐标系    - - struct `v4l2_fract`
      - `pixelaspect`
      - 这是未应用缩放时的像素宽高比（y / x），即实际采样频率与获得方形像素所需频率之比
	当裁剪坐标指向方形像素时，驱动将 `pixelaspect` 设为 1/1。其他常见值为 PAL SECAM
	54/59，以及按 [itu601] 采样NTSC 11/10
   不幸的是，在多平面缓冲区类型（`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`    `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`）的情况下，关于应如何填`v4l2_cropcap`    `type` 字段，该 API 存在混乱。某些驱动只接受 `_MPLANE` 缓冲区类型，而其他驱动只接受
   非多平面缓冲区类型（即末尾不`_MPLANE`）
   从内4.13 起，两种变体都允许


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __s32
      - `left`
      - 矩形左上角的水平偏移，以像素计    - - __s32
      - `top`
      - 矩形左上角的垂直偏移，以像素计    - - __u32
      - `width`
      - 矩形的宽度，以像素计    - - __u32
      - `height`
      - 矩形的高度，以像素计
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
EINVAL
    struct `v4l2_cropcap` `type` 无效
ENODATA
    该输入或输出不支持裁剪