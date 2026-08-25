
######## ioctl VIDIOC_ENUM_FRAMEINTERVALS


## 名称


VIDIOC_ENUM_FRAMEINTERVALS - 鏋氫妇甯ч棿闅。
## 概要



`int ioctl(int fd, VIDIOC_ENUM_FRAMEINTERVALS, struct v4l2_frmivalenum *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向结构`v4l2_frmivalenum` 的指针，
    其中包含像素格式和尺寸，并接收一个帧间隔
## 描述


ioctl 允许应用程序枚举设备针对给定像素格式和帧尺寸所支持的所有帧间隔
支持的像素格式和帧尺寸可以通过使用 VIDIOC_ENUM_FMT VIDIOC_ENUM_FRAMESIZES 函数获得
返回值以`v4l2_frmivalenum.type` 字段的内容取决于设备支持的帧间隔类型。以下是函数在不同情况下的语义：

- **离散（Discrete）：** 如果给定的索引值（从零开始）有效，函数返回成功。应用程   应将索引每次加一进行调用，直到返`EINVAL`。`v4l2_frmivalenum.type` 字段由驱   设置`V4L2_FRMIVAL_TYPE_DISCRETE`。在联合体中，只`discrete` 成员有效
- **步进（Step-wise）：** 如果给定的索引值为零，函数返回成功；对于任何其他索引   则返`EINVAL`。`v4l2_frmivalenum.type` 字段由驱动设置为
   `V4L2_FRMIVAL_TYPE_STEPWISE`。在联合体中，只`stepwise` 成员有效
- **连续（Continuous）：** 这是上述步进类型的一种特殊情况。如果给定的索引值为零，
   函数返回成功；对于任何其他索引值则返回 `EINVAL`。`v4l2_frmivalenum.type` 字段   驱动设置`V4L2_FRMIVAL_TYPE_CONTINUOUS`。在联合体中，只`stepwise` 成员有效   `step` 值被设为 1
当应用程序以索引零调用该函数时，它必须检`type` 字段以确定设备支持的帧间隔枚类型。只有对`V4L2_FRMIVAL_TYPE_DISCRETE` 类型，递增索引值以接收更多帧间隔才意义

   帧间隔返回的顺序没有特殊含义。尤其它并不表示任何关于潜在默认帧间隔的信息
应用程序可以假定枚举数据不会在没有应用程序自身交互的情况下发生变化。这意味着如果
应用程序在运行帧间隔枚举时不执行任何其他 ioctl 调用，则枚举数据是一致的

   **帧间隔与帧率* V4L2 API 使用帧间隔而非帧率。给定帧间隔后，帧率可按如下方式
   计算
```
       frame_rate = 1 / frame_interval

```
## 结构

在下述结构体中，**IN** 表示必须由应用程序填入的值，**OUT** 表示由驱动填入的值应用程序应将**IN** 字段之外的所有成员置零

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct `v4l2_fract`
      - `min`
      - 最小帧间隔 [s]    - - struct `v4l2_fract`
      - `max`
      - 最大帧间隔 [s]    - - struct `v4l2_fract`
      - `step`
      - 帧间隔步[s]





    :header-rows:  0
    :stub-columns: 0

    - - __u32
      - `index`
      - IN：枚举中给定帧间隔的索引    - - __u32
      - `pixel_format`
      - IN：要枚举帧间隔的像素格式    - - __u32
      - `width`
      - IN：要枚举帧间隔的帧宽度    - - __u32
      - `height`
      - IN：要枚举帧间隔的帧高度    - - __u32
      - `type`
      - OUT：设备支持的帧间隔类型    - - union {
      - (anonymous)
      - OUT：具有给定索引的帧间隔    - - struct `v4l2_fract`
      - `discrete`
      - 帧间[s]    - - struct `v4l2_frmival_stepwise`
      - `stepwise`
      -
    - - }
#       -

    - - __u32
      - `reserved[^2^]`
      - 为未来使用保留的空间。驱动和应用程序必须将其置零

## 枚举



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FRMIVAL_TYPE_DISCRETE`
      - 1
      - 离散帧间隔    - - `V4L2_FRMIVAL_TYPE_CONTINUOUS`
      - 2
      - 连续帧间隔    - - `V4L2_FRMIVAL_TYPE_STEPWISE`
      - 3
      - 步进式定义的帧间隔
## 杩斿洖鍊。

成功时返0，出错时返回 -1，并相应地设`errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述