


######## 单平面与多平面 API


某些设备要求将每个输入或输出视频帧的数据放置在不连续的内存缓冲区中。
在这种情况下，一个视频帧必须使用多个内存地址来寻址，即每个“平面”
（plane）一个指针。一个平面是当前帧的一个子缓冲区。此类格式的示例
请参阅 pixfmt。

最初，V4L2 API 并不支持多平面缓冲区，后来引入了一组扩展来处理它们。
这些扩展构成了所谓的“多平面 API”。

V4L2 API 的部分调用与结构体，其解释取决于使用的是单平面 API 还是
多平面 API。应用程序可以通过向其 ioctl 调用传入相应的缓冲区类型来选择
使用哪一种。多平面版本的缓冲区类型以 `_MPLANE` 字符串作为后缀。可用
的多平面缓冲区类型列表请参阅枚举 `v4l2_buf_type`。


## 多平面格式


多平面 API 引入了新的多平面格式。这些格式使用一组独立的 FourCC 代码。
区分多平面 API 与多平面格式非常重要。多平面 API 调用也可以处理所有
单平面格式（只要它们被传入多平面 API 结构体中），而单平面 API 则
无法处理多平面格式。


## 区分单平面与多平面 API 的调用


VIDIOC_QUERYCAP <VIDIOC_QUERYCAP>
    新增了两个多平面能力。对于同时处理单平面与多平面格式的设备，它们
    可以与非多平面能力一起设置。

VIDIOC_G_FMT <VIDIOC_G_FMT>, VIDIOC_S_FMT <VIDIOC_S_FMT>, VIDIOC_TRY_FMT <VIDIOC_TRY_FMT>
    新增了用于描述多平面格式的结构体：struct
    `v4l2_pix_format_mplane` 与
    struct `v4l2_plane_pix_format`。
    驱动可以定义新的多平面格式，它们具有与现有单平面格式不同的
    FourCC 代码。

VIDIOC_QBUF <VIDIOC_QBUF>, VIDIOC_DQBUF <VIDIOC_DQBUF>, VIDIOC_QUERYBUF <VIDIOC_QUERYBUF>
    新增了用于描述平面的 struct `v4l2_plane` 结构体。
    该结构体的数组通过 struct `v4l2_buffer` 新的
    `m.planes` 字段传入。

VIDIOC_REQBUFS <VIDIOC_REQBUFS>
    会按需分配多平面缓冲区。
