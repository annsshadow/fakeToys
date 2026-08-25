


######## ioctl VIDIOC_ENUM_DV_TIMINGS, VIDIOC_SUBDEV_ENUM_DV_TIMINGS


## 名称


VIDIOC_ENUM_DV_TIMINGS - VIDIOC_SUBDEV_ENUM_DV_TIMINGS - 枚举受支持的数字视频（Digital Video）时
## 概要



`int ioctl(int fd, VIDIOC_ENUM_DV_TIMINGS, struct v4l2_enum_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_ENUM_DV_TIMINGS, struct v4l2_enum_dv_timings *argp)`

## 参数



`fd`
    `open()` 返回的文件描述符
`argp`
    指向结构`v4l2_enum_dv_timings` 的指针
## 描述


虽然某些 DV 接收端或发送端支持很宽范围的时序，但另一些只支持数量有限的时序。应用程序可通过ioctl 枚举一份已知受支持时序的列表。若还需确认它是否支持本列表之外的其他标准甚至自定义时序，可调用 VIDIOC_DV_TIMINGS_CAP 进行检查
要查询可用时序，应用程序先初始化 `index` 字段，将 `pad` 字段设为 0，把结构`v4l2_enum_dv_timings` reserved 数组清零，然后在该视频节点上以指向该结构的指针调`VIDIOC_ENUM_DV_TIMINGS` ioctl。驱动会填充结构的其余部分；当索引越界时返回 `EINVAL` 错误码。要枚举所有受支持DV 时序，应用程序应从索0 开始，每次1，直到驱动返`EINVAL`

   驱动在切换视频输入或输出后，可能会枚举出一组不同的 DV 时序
当由驱动实现时，子设备的 DV 时序可通过在子设备节点上直接调`VIDIOC_SUBDEV_ENUM_DV_TIMINGS` ioctl 来查询。DV 时序针对输入（DV 接收端）或输出（DV 发送端）而特定，应用程序必须在结构体 `v4l2_enum_dv_timings` `pad` 字段中指定所需pad 编号。尝试在不支持的 pad 上枚举时序将返回 `EINVAL` 错误码


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - DV 时序的编号，由应用程序设置    - - __u32
      - `pad`
      - 由媒体控制器 API 报告pad 编号。该字段仅在对子设备节点操作时使用。在对视频节点操作时，应用程序必须将该字段设0    - - __u32
      - `reserved`\ [^2^]
      - 保留以备将来扩展。驱动和应用程序都必须将该数组置零    - - struct `v4l2_dv_timings`
      - `timings`
      - 时序
## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误<gen-errors> 一章中描述
EINVAL
    结构`v4l2_enum_dv_timings` `index` 越界，或 `pad` 编号无效
ENODATA
    该输入或输出不支持数字视频预设