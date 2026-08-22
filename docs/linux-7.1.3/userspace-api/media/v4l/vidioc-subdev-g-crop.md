


######## ioctl VIDIOC_SUBDEV_G_CROP、VIDIOC_SUBDEV_S_CROP


## 名称


VIDIOC_SUBDEV_G_CROP - VIDIOC_SUBDEV_S_CROP - 获取或设置子设备 pad 上的裁剪矩形

## 概要


`int ioctl(int fd, VIDIOC_SUBDEV_G_CROP, struct v4l2_subdev_crop *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_CROP, const struct v4l2_subdev_crop *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_subdev_crop` 的指针
## 描述



    这是一个已废弃的接口，未来可能会被移除。它已被选择（selection）API <VIDIOC_SUBDEV_G_SELECTION> 取代。不再接受对 `v4l2_subdev_crop` 结构的任何新扩展
为获取当前裁剪矩形，应用程序struct `v4l2_subdev_crop` `pad` 字段设为 media API 报告的期pad 编号，并`which` 字段设为 `V4L2_SUBDEV_FORMAT_ACTIVE`。然后它们以指向该结构的指针调用 `VIDIOC_SUBDEV_G_CROP` ioctl。如果输入参数无效，或者给pad 不支持裁剪，驱动会填`rect` 字段的成员或返回 `EINVAL` 错误码
为改变当前裁剪矩形，应用程序需同时设置 `pad` `which` 字段以及 `rect` 字段的所有成员。然后它们以指向该结构的指针调用 `VIDIOC_SUBDEV_S_CROP` ioctl。驱动会校验所请求的裁剪矩形，根据硬件能力对其进行调整并配置设备。返回时，struct `v4l2_subdev_crop` 包含当前格式，等价于 `VIDIOC_SUBDEV_G_CROP` 调用所返回的值
应用程序可以通过`which` 设为 `V4L2_SUBDEV_FORMAT_TRY` 来查询设备能力。当设置时，“try”裁剪矩形不会被驱动应用到设备，而是像活动裁剪矩形一样被处理并存储在子设备文件句柄中。因此查询同一子设备的两个应用程序不会相互干扰
如果子设备节点以只读模式注册，则`VIDIOC_SUBDEV_S_CROP` 的调用仅`which` 字段设为 `V4L2_SUBDEV_FORMAT_TRY` 时有效，否则返回错误并将 errno 变量设为 `-EPERM`
驱动绝不可仅仅因为所请求的裁剪矩形与设备能力不匹配就返回错误。它们必须改为修改该矩形以匹配硬件所能提供的值。修改后的格式应尽可能接近原始请求


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - media 框架报告pad 编号    - - __u32
      - `which`
      - 要获取或设置的裁剪矩形，来自枚举
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - struct `v4l2_rect`
      - `rect`
      - 裁剪矩形的边界，单位为像素    - - __u32
      - `stream`
      - 流标识符    - - __u32
      - `reserved`\ [^7^]
      - 为未来扩展保留。应用程序与驱动必须将该数组置零
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EBUSY
    裁剪矩形无法改变，因为该 pad 当前正忙。例如可能是pad 上有活跃video 流。在首先执行其他操作解决问题之前，不得重试该 ioctl。仅`VIDIOC_SUBDEV_S_CROP` 返回
EINVAL
    struct `v4l2_subdev_crop` `pad` 引用了不存在pad，`which` 字段取值不受支持，或者给定子设备 pad 不支持裁剪
EPERM
    `VIDIOC_SUBDEV_S_CROP` ioctl 在以只读模式运行的子设备上被调用，且 `which` 字段被设`V4L2_SUBDEV_FORMAT_ACTIVE`