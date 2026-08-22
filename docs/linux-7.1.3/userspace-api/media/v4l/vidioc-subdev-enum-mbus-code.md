


######## ioctl VIDIOC_SUBDEV_ENUM_MBUS_CODE


## 名称


VIDIOC_SUBDEV_ENUM_MBUS_CODE - 枚举媒体总线格式

## 概要



`int ioctl(int fd, VIDIOC_SUBDEV_ENUM_MBUS_CODE, struct v4l2_subdev_mbus_code_enum * argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向结构`v4l2_subdev_mbus_code_enum` 的指针
## 描述


应用程序使用此调用访问所pad 的媒体总线格式枚举
枚举由驱动定义，并使用结构体 `v4l2_subdev_mbus_code_enum` `index` 字段进行索引每次枚举`index` 0 开始，最低的非有效索引标记枚举的结束
因此，要枚举某个给定子设pad 上可用的媒体总线格式，请`pad` `which` 字段
初始化为期望值，并将 `index` 设为 0。然后以指向该结构体的指针调VIDIOC_SUBDEV_ENUM_MBUS_CODE ioctl
成功的调用将返回填充好的 `code` 字段，其中包含一mbus 代码值。递增 `index` 重复
调用，直到收`EINVAL`。`EINVAL` 表示 `pad` 无效，或者该 pad 上已没有更多代码可用
驱动不得为同一 pad 上不同的索引返回相同`code` 值
可用的媒体总线格式可能取决于子设备其他 pad 上当前的 'try' 格式，以及当前的活跃
链接。有try 格式的更多信息，请参VIDIOC_SUBDEV_G_FMT


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - Pad 编号，由媒体控制API 报告。由应用程序填写    - - __u32
      - `index`
      - 属于给定 pad 的枚举中mbus 代码索引。由应用程序填写    - - __u32
      - `code`
      - 媒体总线格式代码，定义于 v4l2-mbus-format。由驱动填写    - - __u32
      - `which`
      - 要枚举的媒体总线格式代码，来enum
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>銆?    - - __u32
      - `flags`
      - 参见 v4l2-subdev-mbus-code-flags
    - - __u32
      - `stream`
      - 流标识符    - - __u32
      - `reserved`\ [^6^]
      - 为未来扩展保留。应用程序和驱动必须将该数组置零



   \footnotesize



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - V4L2_SUBDEV_MBUS_CODE_CSC_COLORSPACE
      - 0x00000001
      - 驱动允许应用程序尝试更改默认colorspace 编码。应用程序可以在调用
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 并设	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 时，请求配置子设备的
	colorspace。关于如何操作，请参v4l2-mbus-format    - - V4L2_SUBDEV_MBUS_CODE_CSC_XFER_FUNC
      - 0x00000002
      - 驱动允许应用程序尝试更改默认的转换函数。应用程序可以在调用
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 并设	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 时，请求配置子设备的
	转换函数。关于如何操作，请参v4l2-mbus-format    - - V4L2_SUBDEV_MBUS_CODE_CSC_YCBCR_ENC
      - 0x00000004
      - 驱动允许应用程序尝试更改默认Y'CbCr 编码。应用程序可以在调用
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 并设	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 时，请求配置子设备的
	Y'CbCr 编码。关于如何操作，请参v4l2-mbus-format    - - V4L2_SUBDEV_MBUS_CODE_CSC_HSV_ENC
      - 0x00000004
      - 驱动允许应用程序尝试更改默认HSV 编码。应用程序可以在调用
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 并设	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 时，请求配置子设备的
	HSV 编码。关于如何操作，请参v4l2-mbus-format    - - V4L2_SUBDEV_MBUS_CODE_CSC_QUANTIZATION
      - 0x00000008
      - 驱动允许应用程序尝试更改默认的量化。应用程序可以在调用
	VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 并设	V4L2_MBUS_FRAMEFMT_SET_CSC <mbus-framefmt-set-csc> 时，请求配置子设备的
	量化。关于如何操作，请参v4l2-mbus-format

   \normalsize

## 杩斿洖鍊。

成功时返0，出错时返回 -1，并相应地设`errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
EINVAL
    结构`v4l2_subdev_mbus_code_enum` `pad` 引用了一个不存在pad    `which` 字段含有不支持的值，或`index` 字段越界