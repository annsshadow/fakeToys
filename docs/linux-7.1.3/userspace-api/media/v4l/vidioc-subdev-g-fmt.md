


######## ioctl VIDIOC_SUBDEV_G_FMT, VIDIOC_SUBDEV_S_FMT


## 名称


VIDIOC_SUBDEV_G_FMT - VIDIOC_SUBDEV_S_FMT - 获取或设置子设备 pad 上的数据格式

## 概要



`int ioctl(int fd, VIDIOC_SUBDEV_G_FMT, struct v4l2_subdev_format *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_FMT, struct v4l2_subdev_format *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_subdev_format` 的指针。

## 描述


这些 ioctl 用于在图像流水线中特定的子设备 pad 上协商帧格式。

要获取当前格式，应用程序将 struct `v4l2_subdev_format` 的 `pad` 字段设为媒体
API 报告的期望 pad 号，并将 `which` 字段设为 `V4L2_SUBDEV_FORMAT_ACTIVE`。当
它们以指向该结构的指针调用 `VIDIOC_SUBDEV_G_FMT` ioctl 时，驱动会填充 `format`
字段的成员。

要更改当前格式，应用程序设置 `pad` 和 `which` 字段以及 `format` 字段的所有成员。
当它们以指向该结构的指针调用 `VIDIOC_SUBDEV_S_FMT` ioctl 时，驱动会校验请求的
格式，根据硬件能力对其进行调整，并配置设备。返回时 struct `v4l2_subdev_format`
包含当前格式，正如 `VIDIOC_SUBDEV_G_FMT` 调用所返回的那样。

应用程序可以通过将 `which` 设为 `V4L2_SUBDEV_FORMAT_TRY` 来查询设备能力。当设置
该值时，“try”格式不会被驱动应用到设备上，而是像活动格式一样被修改，并存储在
子设备文件句柄中。因此，两个查询同一子设备的应用程序不会相互影响。

例如，要在子设备的输出 pad 上尝试一种格式，应用程序会首先用 `VIDIOC_SUBDEV_S_FMT`
ioctl 在子设备输入处设置 try 格式。然后它们要么用 `VIDIOC_SUBDEV_G_FMT` ioctl
获取输出 pad 处的默认格式，要么用 `VIDIOC_SUBDEV_S_FMT` ioctl 设置期望的输出
pad 格式并检查返回值。

Try 格式不依赖于活动格式，但可能依赖于当前链路配置或子设备控制值。例如，一个
低通噪声滤波器可能会在帧边界处裁剪像素，从而修改其输出帧大小。

如果子设备节点已以只读模式注册，则对 `VIDIOC_SUBDEV_S_FMT` 的调用仅在 `which`
字段设为 `V4L2_SUBDEV_FORMAT_TRY` 时才有效，否则返回错误并将 errno 变量设为
`-EPERM`。

驱动绝不能仅仅因为请求的格式与设备能力不匹配就返回错误。它们必须改为修改格式
以匹配硬件所能提供的。修改后的格式应尽可能接近原始请求。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - 由媒体控制器 API 报告的 pad 号。
    - - __u32
      - `which`
      - 要修改的格式，来自枚举
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>。
    - - struct `v4l2_mbus_framefmt`
      - `format`
      - 图像格式定义，详见 `v4l2_mbus_framefmt`。
    - - __u32
      - `stream`
      - 流标识符。
    - - __u32
      - `reserved`\ [^7^]
      - 为将来扩展保留。应用程序和驱动必须将该数组置零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - V4L2_SUBDEV_FORMAT_TRY
      - 0
      - Try 格式，用于查询设备能力。
    - - V4L2_SUBDEV_FORMAT_ACTIVE
      - 1
      - 活动格式，应用到硬件。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述。

EBUSY
    格式无法更改，因为该 pad 当前正忙。例如，这可能是由于该 pad 上有活跃的
    视频流。必须先执行其它操作解决问题，才能重试该 ioctl。仅由
    `VIDIOC_SUBDEV_S_FMT` 返回。

EINVAL
    struct `v4l2_subdev_format` 的 `pad` 引用了一个不存在的 pad，或者 `which`
    字段的值不受支持。

EPERM
    `VIDIOC_SUBDEV_S_FMT` ioctl 在一个只读子设备上被调用，且 `which` 字段被设
    为 `V4L2_SUBDEV_FORMAT_ACTIVE`。

============

成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述。
