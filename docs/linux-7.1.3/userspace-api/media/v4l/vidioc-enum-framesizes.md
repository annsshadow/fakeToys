


######## ioctl VIDIOC_ENUM_FRAMESIZES


## 名称


VIDIOC_ENUM_FRAMESIZES - 鏋氫妇甯уぇ灏。
## 概要



`int ioctl(int fd, VIDIOC_ENUM_FRAMESIZES, struct v4l2_frmsizeenum *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_frmsizeenum` 的指针，其中包含一个索引和像素格式    并接收帧的宽度和高度
## 描述


ioctl 允许应用程序枚举设备针对给定像素格式所支持的所有帧大小（即宽度
和高度，单位为像素）
支持的像素格式可以通过 VIDIOC_ENUM_FMT 函数获得
`v4l2_frmsizeenum.type` 字段的返回值及其内容取决于设备所支持的帧大小类型以下是该函数在不同情况下的语义：

- **Discrete（离散）* 若给定的索引值（从零开始）有效，函数返回成功  应用程序应将索引每次加一后重复调用，直到返回 `EINVAL`。驱动将
  `v4l2_frmsizeenum.type` 字段设为 `V4L2_FRMSIZE_TYPE_DISCRETE`。联合体  只有 `discrete` 成员有效
- **Step-wise（步进）* 若给定的索引值为零则函数返回成功，其它任何索引  都返`EINVAL`。驱动将 `v4l2_frmsizeenum.type` 字段设为
  `V4L2_FRMSIZE_TYPE_STEPWISE`。联合体中只`stepwise` 成员有效
- **Continuous（连续）* 这是上述步进类型的一种特殊情况。若给定的索引  为零则函数返回成功，其它任何索引值都返回 `EINVAL`。驱动将
  `v4l2_frmsizeenum.type` 字段设为 `V4L2_FRMSIZE_TYPE_CONTINUOUS`。联合体  只有 `stepwise` 成员有效，且 `step_width` `step_height` 的值被设为 1
当应用程序以索引零调用该函数时，它必须检`type` 字段以确定设备支持的
帧大小枚举类型。只有对`V4L2_FRMSIZE_TYPE_DISCRETE` 类型，递增索引值以
获取更多帧大小才有意义

   帧大小返回的顺序没有特殊含义。特别地，它并不表示任何潜在的默认格式大小
应用程序可以假定，在没有应用程序自身任何交互的情况下，枚举数据不会发生变化这意味着如果应用程序在运行帧大小枚举期间不执行任何其ioctl 调用，枚举数就是一致的
## 结构


In the structs below, **IN** denotes a value that has to be filled in by
the application, **OUT** denotes values that the driver fills in. The
application should zero out all members except for the **IN** fields.


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `width`
      - 帧宽[像素]    - - __u32
      - `height`
      - 帧高[像素]

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `min_width`
      - 最小帧宽度 [像素]    - - __u32
      - `max_width`
      - 最大帧宽度 [像素]    - - __u32
      - `step_width`
      - 帧宽度步[像素]    - - __u32
      - `min_height`
      - 最小帧高度 [像素]    - - __u32
      - `max_height`
      - 最大帧高度 [像素]    - - __u32
      - `step_height`
      - 帧高度步[像素]

    :header-rows:  0
    :stub-columns: 0

    - - __u32
      - `index`
      - IN：枚举中给定帧大小的索引    - - __u32
      - `pixel_format`
      - IN：要枚举帧大小的像素格式    - - __u32
      - `type`
      - OUT：设备支持的帧大小类型    - - union {
      - (anonymous)
      - OUT：具有给定索引的帧大小    - - struct `v4l2_frmsize_discrete`
      - `discrete`
      -
    - - struct `v4l2_frmsize_stepwise`
      - `stepwise`
      -
    - - }
#       -

    - - __u32
      - `reserved[^2^]`
      - 为将来使用保留的空间。驱动和应用程序必须将其清零

## 枚举



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_FRMSIZE_TYPE_DISCRETE`
      - 1
      - 离散帧大小    - - `V4L2_FRMSIZE_TYPE_CONTINUOUS`
      - 2
      - 连续帧大小    - - `V4L2_FRMSIZE_TYPE_STEPWISE`
      - 3
      - 步进式定义的帧大小
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述