


######## ioctl VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL


## 名称


VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL - 鏋氫妇甯ч棿闅。
## 概要


`int ioctl(int fd, VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL, struct v4l2_subdev_frame_interval_enum * argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_subdev_frame_interval_enum` 的指针
## 描述


ioctl 让应用程序枚举给定子设备 pad 上可用的帧间隔。帧间隔仅对能够自行控制帧周期的子设备有意义。这包括，例如，图像传感器和 TV 调谐器
对于图像传感器这一常见用例，子设备输出 pad 上可用的帧间隔取决于同一 pad 上的帧格式和尺寸。因此，应用程序在枚举帧间隔时必须指定期望的格式和尺寸
为枚举帧间隔，应用程序初始化 struct `v4l2_subdev_frame_interval_enum` `index`、`pad`、`which`、`code`、`width` `height` 字段，并以指向该结构的指针调VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL ioctl。如果某个输入字段无效，驱动填充结构的其余部分或返回 `EINVAL` 错误码。所有帧间隔都可通过从索引零开始递增一，直到返`EINVAL` 来枚举
可用帧间隔可能取决于子设备其pad 上的当前“try”格式，以及当前的活跃链接。关try 格式的更多信息，请参VIDIOC_SUBDEV_G_FMT
支持帧间隔枚ioctl 的子设备应仅在单pad 上实现它。当它在同一子设备的多个 pad 上受支持时，其行为未定义


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 枚举中的格式编号，由应用程序设置    - - __u32
      - `pad`
      - media controller API 报告pad 编号    - - __u32
      - `code`
      - media 总线格式代码，定义于 v4l2-mbus-format    - - __u32
      - `width`
      - 帧宽，单位为像素    - - __u32
      - `height`
      - 帧高，单位为像素    - - struct `v4l2_fract`
      - `interval`
      - 连续视频帧之间的周期，单位为秒    - - __u32
      - `which`
      - 要枚举的帧间隔，来自枚举
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `stream`
      - 流标识符    - - __u32
      - `reserved`\ [^7^]
      - 为未来扩展保留。应用程序与驱动必须将该数组置零
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EINVAL
    struct `v4l2_subdev_frame_interval_enum` `pad` 引用了不存在pad，`which` 字段取值不受支持，给定`code`、`width` `height` 字段对指pad 无效，或`index` 字段越界