

######## ioctl VIDIOC_G_CROP, VIDIOC_S_CROP


## 名称


VIDIOC_G_CROP - VIDIOC_S_CROP - 获取或设置当前的裁剪矩形

## 语法


`int ioctl(int fd, VIDIOC_G_CROP, struct v4l2_crop *argp)`


`int ioctl(int fd, VIDIOC_S_CROP, const struct v4l2_crop *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向结构`v4l2_crop` 的指针
## 描述


为了查询裁剪矩形的大小和位置，应用程序把结构`v4l2_crop` `type`
字段设置为相应的缓冲区（流）类型，并以指向该结构体的指针调用 VIDIOC_G_CROP <VIDIOC_G_CROP> ioctl驱动会填充结构体的其余部分，如果不支持裁剪则返回 `EINVAL` 错误码
为了改变裁剪矩形，应用程序初始化 v4l2_crop 结构体中`type` 字段
以及名为 `c` `v4l2_rect` 子结构体，并以指向该结构体的指针调用
VIDIOC_S_CROP <VIDIOC_G_CROP> ioctl銆。
驱动首先根据硬件限制（即由捕输出窗口给出的边界）调整请求的尺寸，
并把水平和垂直偏移、宽度和高度舍入到最接近的可能值。特别是，驱动必须把裁剪
矩形的垂直偏移舍入为帧行数模二，以避免字段顺序被混淆
其次，驱动在保持当前水平和垂直缩放因子的前提下，把图像大小（缩放过程相对的矩形，源或目标取决于数据方向）调整为最接近的可能大小
最后，驱动用实际的裁剪和图像参数对硬件进行编程。VIDIOC_S_CROP <VIDIOC_G_CROP>
是一个只ioctl，它不返回实际参数。要查询这些参数，应用程序必须调VIDIOC_G_CROP <VIDIOC_G_CROP> VIDIOC_G_FMT。当参数不合适时，应用程序可修改裁剪或图像参数并重复该循环，直到协商出满意的参数
当不支持裁剪时，不会修改任何参数，VIDIOC_S_CROP <VIDIOC_G_CROP> 返回 `EINVAL` 错误码


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 数据类型，由应用程序设置。此处仅以下类型有效：`V4L2_BUF_TYPE_VIDEO_CAPTURE`、`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`	`V4L2_BUF_TYPE_VIDEO_OUTPUT`、`V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE` 以及
	`V4L2_BUF_TYPE_VIDEO_OVERLAY`。参`v4l2_buf_type` 以及下面的说明    - - struct `v4l2_rect`
      - `c`
      - 裁剪矩形。使用的坐标系与结构`v4l2_cropcap` `bounds` 相同
   遗憾的是，在多平面缓冲区类型（`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE` `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`   的情况下，关于应如何填写 `v4l2_crop` `type` 字段，该 API 一度混乱。某些驱   只接`_MPLANE` 缓冲区类型，而其他驱动只接受非多平面缓冲区类型（即不带末尾的
   `_MPLANE`）
   从内4.13 起，两种写法都被允许
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
通用错误<gen-errors> 章节中描述
ENODATA
    该输入或输出不支持裁剪
