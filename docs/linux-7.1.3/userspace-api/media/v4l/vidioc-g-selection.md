

######## ioctl VIDIOC_G_SELECTION, VIDIOC_S_SELECTION


## Name


VIDIOC_G_SELECTION - VIDIOC_S_SELECTION - 获取或设置其中一个选择矩形

## Synopsis


`int ioctl(int fd, VIDIOC_G_SELECTION, struct v4l2_selection *argp)`


`int ioctl(int fd, VIDIOC_S_SELECTION, struct v4l2_selection *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符

`argp`
    指向 struct `v4l2_selection` 的指针

## Description


这些 ioctl 用于查询和配置选择矩形

要查询裁剪（组合）矩形，请将 struct `v4l2_selection` `type` 字段设置为相应的缓冲区类型。下一步是struct `v4l2_selection` `target` 字段设置`V4L2_SEL_TGT_CROP`（`V4L2_SEL_TGT_COMPOSE`）。更多目标请参考表 v4l2-selections-common selection-api。struct `v4l2_selection` `flags` `reserved` 字段会被忽略，必须填零。驱动会填充结构的其余部分，如果使用了不正确的缓冲区类型或目标则返回 EINVAL 错误码。如果不支持裁剪（组合），则活动矩形不可变，且始终等于边界矩形。最后，struct `v4l2_rect` `r` 矩形会被填入当前的裁剪（组合）坐标。坐标以驱动相关的单位表示。唯一的例外是原始格式图像的矩形，其坐标始终以像素表示

要修改裁剪（组合）矩形，请将 struct `v4l2_selection` `type` 字段设置为相应的缓冲区类型。下一步是struct `v4l2_selection` `target` 设置`V4L2_SEL_TGT_CROP`（`V4L2_SEL_TGT_COMPOSE`）。更多目标请参考表 v4l2-selections-common selection-api。struct `v4l2_rect` `r` 矩形需要被设置为期望的活动区域。struct `v4l2_selection` `reserved` 字段会被忽略，必须填零。驱动可能会调整所请求矩形的坐标。应用程序可以引入约束来控制舍入行为。struct `v4l2_selection` `flags` 字段必须设置为下列之一

- `0` - 驱动可以自由调整矩形大小，并应选择尽可能接近所请求矩形的裁组合矩形

- `V4L2_SEL_FLAG_GE` - 不允许驱动缩小矩形。原始矩形必须位于调整后的矩形内部

- `V4L2_SEL_FLAG_LE` - 不允许驱动放大矩形。调整后的矩形必须位于原始矩形内部

- `V4L2_SEL_FLAG_GE | V4L2_SEL_FLAG_LE` - 驱动必须选择大小与所请求矩形完全相同的矩形

请参sel-const-adjust

驱动可能必须根据硬件限制以及流水线的其他部分（即捕获/输出窗口或电视显示所给出的边界）来调整所请求的尺寸。按照以下优先级选择尽可能接近的水平与垂直偏移及大小

1. 满足来自 struct `v4l2_selection` `flags` 的约束

2. 根据硬件限制和对齐要求调整宽度、高度、左边和上边

3. 使调整后矩形的中心尽可能接近原始矩形

4. 使宽度和高度尽可能接近原始值

5. 使水平和垂直偏移尽可能接近原始值

成功时，struct `v4l2_rect` `r` 字段包含调整后的矩形。当参数不合适时，应用程序可以修改裁剪（组合）或图像参数并重复该循环，直到协商出满意的参数。如果必须违反约束标志，则返`ERANGE`。该错误表明**不存*满足约束的矩形

选择目标与标志在 v4l2-selections-common 中有文档说明


    :alt:    constraints.svg
    :align:  center

    带约束标志的尺寸调整

    不同约束标志下矩形调整的行为



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 缓冲区的类型（来enum `v4l2_buf_type`）
    - - __u32
      - `target`
      - 用于在裁剪矩形和组合矩形之间进行选择 <v4l2-selections-common>
    - - __u32
      - `flags`
      - 控制选择矩形调整的标志，请参selection flags <v4l2-selection-flags>
    - - struct `v4l2_rect`
      - `r`
      - 选择矩形
    - - __u32
      - `reserved[^9^]`
      - 供将来使用的保留字段。驱动和应用程序必须将本数组清零

   遗憾的是，对于多平面缓冲区类型（`V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE` `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`），关于应如何填`v4l2_selection` `type` 字段，该 API 是混乱的。某些驱动只接受 `_MPLANE` 缓冲区类型，而另一些驱动只接受非多平面缓冲区类型（即末尾不`_MPLANE`）

   从内4.13 开始，两种写法都被允许

## Return Value


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述

EINVAL
    给定的缓冲区类型 `type` 或选择目标 `target` 不受支持，或`flags` 参数无效

ERANGE
    无法调整 struct `v4l2_rect` `r` 矩形以满`flags` 参数中给出的所有约束

ENODATA
    该输入或输出不支持选择

EBUSY
    当前无法应用选择矩形的修改。通常是因为正在进行流传输
