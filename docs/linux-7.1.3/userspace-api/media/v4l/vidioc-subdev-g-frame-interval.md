


######## ioctl VIDIOC_SUBDEV_G_FRAME_INTERVAL、VIDIOC_SUBDEV_S_FRAME_INTERVAL


## 名称


VIDIOC_SUBDEV_G_FRAME_INTERVAL - VIDIOC_SUBDEV_S_FRAME_INTERVAL - 获取或设置子设备 pad 上的帧间
## 概要


`int ioctl(int fd, VIDIOC_SUBDEV_G_FRAME_INTERVAL, struct v4l2_subdev_frame_interval *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_FRAME_INTERVAL, struct v4l2_subdev_frame_interval *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_subdev_frame_interval` 的指针
## 描述


这些 ioctl 用于获取和设置图像流水线中特定子设备 pad 上的帧间隔。帧间隔仅对能够自行控制帧周期的子设备才有意义。这包括，例如，图像传感器和 TV 调谐器。不支持帧间隔的子设备不得实现这ioctl
应用程序为获取当前帧间隔，需struct `v4l2_subdev_frame_interval` `pad` 字段设为 media controller API 报告的期pad 编号。当它们以指向该结构的指针调`VIDIOC_SUBDEV_G_FRAME_INTERVAL` ioctl 时，驱动会填`interval` 字段的各成员
为改变当前帧间隔，应用程序需同时设置 `pad` 字段`interval` 字段的全部成员。当它们以指向该结构的指针调`VIDIOC_SUBDEV_S_FRAME_INTERVAL` ioctl 时，驱动会校验所请求的间隔，并根据硬件能力对其进行调整后配置设备。返回时，struct `v4l2_subdev_frame_interval` 包含当前帧间隔，等价`VIDIOC_SUBDEV_G_FRAME_INTERVAL` 调用所返回的值
如果子设备节点以只读模式注册，则`VIDIOC_SUBDEV_S_FRAME_INTERVAL` 的调用仅`which` 字段设为 `V4L2_SUBDEV_FORMAT_TRY` 时有效，否则返回错误，且 errno 变量被设`-EPERM`
驱动绝不可仅仅因为请求的间隔与设备能力不匹配就返回错误。它们必须改为对间隔进行修改以匹配硬件所能提供的值。修改后的间隔应尽可能接近原始请求
改变帧间隔绝不可改变格式。另一方面，改变格式可能会改变帧间隔
支持帧间ioctl 的子设备应仅在单pad 上实现它们。当同一子设备的多个 pad 都支持时，其行为未定义


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - media controller API 报告pad 编号    - - struct `v4l2_fract`
      - `interval`
      - 连续视频帧之间的周期，单位为秒    - - __u32
      - `stream`
      - 流标识符    - - __u32
      - `which`
      - 活动或尝试的帧间隔，来自枚举
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `reserved`\ [^7^]
      - 为未来扩展保留。应用程序与驱动必须将该数组置零
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
EBUSY
    帧间隔无法改变，因为pad 当前正忙。例如可能是pad 上有活跃video 流。在首先执行其他操作解决问题之前，不得重试该 ioctl。仅`VIDIOC_SUBDEV_S_FRAME_INTERVAL` 返回
EINVAL
    struct `v4l2_subdev_frame_interval` `pad` 引用了不存在pad，`which` 字段取值不受支持，或者该 pad 不支持帧间隔
EPERM
    `VIDIOC_SUBDEV_S_FRAME_INTERVAL` ioctl 在以只读模式运行的子设备上被调用，且 `which` 字段被设`V4L2_SUBDEV_FORMAT_ACTIVE`