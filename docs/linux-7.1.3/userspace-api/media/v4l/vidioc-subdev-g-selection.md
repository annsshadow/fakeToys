######## ioctl VIDIOC_SUBDEV_G_SELECTION, VIDIOC_SUBDEV_S_SELECTION


## 名称


VIDIOC_SUBDEV_G_SELECTION - VIDIOC_SUBDEV_S_SELECTION - 获取或设置子设备 pad 上的选择矩形

## 概要


`int ioctl(int fd, VIDIOC_SUBDEV_G_SELECTION, struct v4l2_subdev_selection *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_SELECTION, struct v4l2_subdev_selection *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_subdev_selection` 的指针。

## 描述


选择矩形用于配置子设备执行的、影响图像尺寸的各种图像处理功能。
目前这包括裁剪、缩放与合成。

选择 API 取代了旧的子设备裁剪 API <VIDIOC_SUBDEV_G_CROP>。裁剪 API
的所有功能以及更多功能都由选择 API 支持。

有关每个选择目标如何影响子设备内部的图像处理流水线，请参阅子设备相关文档。

如果子设备节点是以只读模式注册的，那么对 `VIDIOC_SUBDEV_S_SELECTION`
的调用仅在 `which` 字段被设为 `V4L2_SUBDEV_FORMAT_TRY` 时才有效，
否则将返回错误，并将 errno 变量设为 `-EPERM`。

### 选择目标的类型


选择目标有两种类型：实际目标（actual）与边界（bounds）。实际目标
是用于配置硬件的目标。BOUNDS 目标将返回一个包含了所有可能的实际
矩形的矩形。

### 发现受支持的特性


要发现哪些目标受支持，用户可以对这些目标执行
`VIDIOC_SUBDEV_G_SELECTION`。任何不受支持的目标都会返回 `EINVAL`。

选择目标与标志在 v4l2-selections-common 中有文档说明。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `which`
      - 活动或尝试（try）选择，来自枚举
	v4l2_subdev_format_whence <v4l2-subdev-format-whence>。
    - - __u32
      - `pad`
      - 由媒体框架报告的 pad 编号。
    - - __u32
      - `target`
      - 目标选择矩形。参见 v4l2-selections-common。
    - - __u32
      - `flags`
      - 标志。参见 v4l2-selection-flags。
    - - struct `v4l2_rect`
      - `r`
      - 选择矩形，以像素为单位。
    - - __u32
      - `stream`
      - 流标识符。
    - - __u32
      - `reserved`\ [^7^]
      - 为未来扩展保留。应用程序与驱动必须将该数组置零。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

EBUSY
    选择矩形无法更改，因为该 pad 当前处于忙状态。例如，这可能是由于
    pad 上存在一个活动的视频流。在首先执行其他操作来修复该问题之前，
    不得重试该 ioctl。仅由 `VIDIOC_SUBDEV_S_SELECTION` 返回。

EINVAL
    struct `v4l2_subdev_selection` 的 `pad` 引用了一个不存在的 pad，
    `which` 字段取了不受支持的值，或者给定的子设备 pad 不支持该选择目标。

EPERM
    `VIDIOC_SUBDEV_S_SELECTION` ioctl 在只读子设备上被调用，且 `which`
    字段被设为 `V4L2_SUBDEV_FORMAT_ACTIVE`。
