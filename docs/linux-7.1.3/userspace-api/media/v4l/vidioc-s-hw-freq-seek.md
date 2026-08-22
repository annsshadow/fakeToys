######## ioctl VIDIOC_S_HW_FREQ_SEEK


## 名称


VIDIOC_S_HW_FREQ_SEEK - 执行硬件频率搜索

## 概要


`int ioctl(int fd, VIDIOC_S_HW_FREQ_SEEK, struct v4l2_hw_freq_seek *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符

`argp`
    指向 struct `v4l2_hw_freq_seek` 的指针

## 描述


从当前频率开始进行硬件频率搜索。为此，应用程序初始`tuner`、`type`、`seek_upward`、`wrap_around`、`spacing`、`rangelow` `rangehigh` 字段，将 `reserved` 数组清零，并使用指向该结构的指针调用 `VIDIOC_S_HW_FREQ_SEEK` ioctl

`rangelow` `rangehigh` 字段可以设置为非默认值，以告知驱动搜索特定频段。如struct `v4l2_tuner` `capability` 字段设置`V4L2_TUNER_CAP_HWSEEK_PROG_LIM` 标志，这些值必须落VIDIOC_ENUM_FREQ_BANDS 返回的某个频段之内。如果未设置 `V4L2_TUNER_CAP_HWSEEK_PROG_LIM` 标志，则这些值必须精确匹VIDIOC_ENUM_FREQ_BANDS 返回的某个频段。如果调谐器的当前频率不在所选频段内，在开始搜索之前它将被限制（clamp）到该频段内

如果返回错误，则将恢复原始频率

如果设置`V4L2_CAP_HW_FREQ_SEEK` 能力，则支持ioctl

如果ioctl 从非阻塞文件句柄调用，则返回 `EAGAIN` 错误码，且不进行搜索




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `tuner`
      - 调谐器索引号。这struct `v4l2_input` `tuner` 字段以及 struct `v4l2_tuner` `index` 字段中的值相同
    - - __u32
      - `type`
      - 调谐器类型。这struct `v4l2_tuner` `type` 字段中的值相同。请参见 `v4l2_tuner_type`
    - - __u32
      - `seek_upward`
      - 如果非零，则从当前频率向上搜索，否则向下搜索
    - - __u32
      - `wrap_around`
      - 如果非零，在到达频率范围末端时回绕，否则停止搜索。struct `v4l2_tuner` `capability` 字段会告诉你硬件支持什么
    - - __u32
      - `spacing`
      - 如果非零，定义硬件搜索分辨率（以 Hz 为单位）。驱动选择设备支持的最接近的值。如spacing 为零，则使用合理的默认值
    - - __u32
      - `rangelow`
      - 如果非零，要搜索频段的以 62.5 kHz 为单位的 tunable 最低频率；如果 struct `v4l2_tuner` `capability` 字段设置`V4L2_TUNER_CAP_LOW` 标志，则62.5 Hz 为单位；如果 struct `v4l2_tuner` `capability` 字段设置`V4L2_TUNER_CAP_1HZ` 标志，则1 Hz 为单位。如`rangelow` 为零，则使用合理的默认值
    - - __u32
      - `rangehigh`
      - 如果非零，要搜索频段的以 62.5 kHz 为单位的 tunable 最高频率；如果 struct `v4l2_tuner` `capability` 字段设置`V4L2_TUNER_CAP_LOW` 标志，则62.5 Hz 为单位；如果 struct `v4l2_tuner` `capability` 字段设置`V4L2_TUNER_CAP_1HZ` 标志，则1 Hz 为单位。如`rangehigh` 为零，则使用合理的默认值
    - - __u32
      - `reserved`\ [^5^]
      - 为未来扩展保留。应用程序必须将数组设置为零

## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

EINVAL
    `tuner` 索引越界，`wrap_around` 值不受支持，`type`、`rangelow` `rangehigh` 字段中的某个值有误

EAGAIN
    尝试以非阻塞模式调用 `VIDIOC_S_HW_FREQ_SEEK`

ENODATA
    硬件搜索未找到任何频道

EBUSY
    另一个硬件搜索已在进行中
