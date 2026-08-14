
######## ioctl VIDIOC_DV_TIMINGS_CAP, VIDIOC_SUBDEV_DV_TIMINGS_CAP


## 名称


VIDIOC_DV_TIMINGS_CAP - VIDIOC_SUBDEV_DV_TIMINGS_CAP - 数字视频接收/发送器的能力

## 语法


`int ioctl(int fd, VIDIOC_DV_TIMINGS_CAP, struct v4l2_dv_timings_cap *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_DV_TIMINGS_CAP, struct v4l2_dv_timings_cap *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_dv_timings_cap` 的指针。

## 描述


要查询 DV 接收/发送器的能力，应用程序将 struct `v4l2_dv_timings_cap` 的 `pad`
字段初始化为 0，将 reserved 数组清零，并在视频节点上调用 `VIDIOC_DV_TIMINGS_CAP`
ioctl，驱动随后会填充该结构。


   驱动在切换视频输入或输出后，可能返回不同的值。

当由驱动实现时，子设备的 DV 能力可通过在子设备节点上直接调用
`VIDIOC_SUBDEV_DV_TIMINGS_CAP` ioctl 来查询。这些能力特定于输入（对于 DV 接收器）
或输出（对于 DV 发送器），应用程序必须在 struct `v4l2_dv_timings_cap` 的 `pad`
字段中指定所需的 pad 编号，并将 `reserved` 数组清零。尝试查询不支持该能力的 pad
将返回 `EINVAL` 错误码。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `min_width`
      - 活动视频的最小宽度（像素）。
    - - __u32
      - `max_width`
      - 活动视频的最大宽度（像素）。
    - - __u32
      - `min_height`
      - 活动视频的最小高度（行数）。
    - - __u32
      - `max_height`
      - 活动视频的最大高度（行数）。
    - - __u64
      - `min_pixelclock`
      - 最小像素时钟频率（Hz）。
    - - __u64
      - `max_pixelclock`
      - 最大像素时钟频率（Hz）。
    - - __u32
      - `standards`
      - 硬件支持的视频标准。标准列表参见 dv-bt-standards。
    - - __u32
      - `capabilities`
      - 提供关于这些能力的更多信息的一些标志。标志说明参见 dv-bt-cap-capabilities。
    - - __u32
      - `reserved`\ [^16^]
      - 为将来扩展保留。驱动必须将数组置零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - DV 时序类型，列出于 dv-timing-types。
    - - __u32
      - `pad`
      - 由媒体控制器 API 报告的 pad 编号。该字段仅在对子设备节点操作时使用。
	在对视频节点操作时，应用程序必须将该字段置为零。
    - - __u32
      - `reserved`\ [^2^]
      - 为将来扩展保留。

	驱动与应用程序都必须将数组置零。
    - - union {
      - (anonymous)
    - - struct `v4l2_bt_timings_cap`
      - `bt`
      - 硬件的 BT.656/1120 时序能力。
    - - __u32
      - `raw_data`\ [^32^]
    - - }
      -



    :header-rows:  0
    :stub-columns: 0

    - - 标志
      - 描述
#     * -

    - - `V4L2_DV_BT_CAP_INTERLACED`
      - 支持隔行（interlaced）格式。
    - - `V4L2_DV_BT_CAP_PROGRESSIVE`
      - 支持逐行（progressive）格式。
    - - `V4L2_DV_BT_CAP_REDUCED_BLANKING`
      - CVT/GTF 专用：时序可利用缩减消隐（CVT）或“Secondary GTF”曲线（GTF）。
    - - `V4L2_DV_BT_CAP_CUSTOM`
      - 支持非标准时序，即不属于 `standards` 字段所设标准的时序。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。
