
######## ioctl VIDIOC_SUBDEV_ENUM_FRAME_SIZE


## 名称


VIDIOC_SUBDEV_ENUM_FRAME_SIZE - 枚举媒体总线帧尺寸

## 语法


`int ioctl(int fd, VIDIOC_SUBDEV_ENUM_FRAME_SIZE, struct v4l2_subdev_frame_size_enum * argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_subdev_frame_size_enum` 的指针。

## 描述


该 ioctl 允许应用程序访问子设备为指定 pad、指定媒体总线格式所支持的帧尺寸枚举。

支持的格式可通过 VIDIOC_SUBDEV_ENUM_MBUS_CODE ioctl 获取。

枚举由驱动定义，并使用 struct `v4l2_subdev_frame_size_enum` 的 `index` 字段进行索引。
每一对 `pad` 与 `code` 对应一个独立的枚举。每个枚举从 `index` 为 0 开始，最小的
无效 index 标志着枚举的结束。

因此，要枚举指定 pad 上、使用指定 mbus 格式所允许的帧尺寸，需将 `pad`、`which` 与
`code` 字段初始化为期望值，并将 `index` 置为 0。然后以指向该结构的指针调用
VIDIOC_SUBDEV_ENUM_FRAME_SIZE ioctl。

成功的调用会返回填充好的最小与最大帧尺寸。递增 `index` 重复调用，直到收到 `EINVAL`。
`EINVAL` 表示枚举中已无更多条目，或某个输入参数无效。

只支持离散帧尺寸的子设备（例如大多数传感器）会返回一个或多个最小与最大值相同的帧尺寸。

在给定 [minimum, maximum] 范围内并非所有可能的尺寸都受支持。例如，使用定点缩放比例的
缩放器可能无法生成最小与最大值之间的每一个帧尺寸。应用程序必须使用
VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 来向子设备请求一个确切受支持的
帧尺寸。

可用的帧尺寸可能取决于子设备其他 pad 上当前的 'try' 格式、当前的活跃链路以及当前
V4L2 控件的值。关于 try 格式的更多信息，请参见 VIDIOC_SUBDEV_G_FMT。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 枚举中属于给定 pad 与格式的帧尺寸索引。由应用程序填充。
    - - __u32
      - `pad`
      - 由媒体控制器 API 报告的 pad 编号。由应用程序填充。
    - - __u32
      - `code`
      - 媒体总线格式码，定义于 v4l2-mbus-format。由应用程序填充。
    - - __u32
      - `min_width`
      - 最小帧宽，单位像素。由驱动填充。
    - - __u32
      - `max_width`
      - 最大帧宽，单位像素。由驱动填充。
    - - __u32
      - `min_height`
      - 最小帧高，单位像素。由驱动填充。
    - - __u32
      - `max_height`
      - 最大帧高，单位像素。由驱动填充。
    - - __u32
      - `which`
      - 要枚举的帧尺寸，来自枚举 v4l2_subdev_format_whence <v4l2-subdev-format-whence>。
    - - __u32
      - `stream`
      - 流标识符。
    - - __u32
      - `reserved`\ [^7^]
      - 为将来扩展保留。应用程序与驱动都必须将数组置零。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    struct `v4l2_subdev_frame_size_enum` 的 `pad` 引用了一个不存在的 pad，`which`
    字段的值不受支持，`code` 对给定 pad 无效，或 `index` 字段越界。
