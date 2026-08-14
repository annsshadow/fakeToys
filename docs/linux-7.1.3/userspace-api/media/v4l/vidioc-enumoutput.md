


######## ioctl VIDIOC_ENUMOUTPUT


## 名称


VIDIOC_ENUMOUTPUT - 枚举视频输出

## 概要


`int ioctl(int fd, VIDIOC_ENUMOUTPUT, struct v4l2_output *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_output` 的指针。

## 描述


为查询视频输出的属性，应用程序初始化 struct `v4l2_output` 的 `index` 字段，并以指向该结构的指针调用 VIDIOC_ENUMOUTPUT。当索引越界时，驱动填充结构的其余部分或返回 `EINVAL` 错误码。为枚举所有输出，应用程序应从索引零开始，每次递增一，直到驱动返回 `EINVAL`。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 标识输出，由应用程序设置。
    - - __u8
      - `name`\ [^32^]
      - 视频输出的名称，一个以 NUL 结尾的 ASCII 字符串，例如："Vout"。此信息面向用户，最好使用设备本身上的连接器标签。
    - - __u32
      - `type`
      - 输出的类型，参见 output-type。
    - - __u32
      - `audioset`
      - 驱动可以枚举多达 32 个视频和音频输出。如果这是当前选中的视频输出，该字段显示哪些音频输出可作为当前输出被选中。它是一个位掩码。LSB 对应音频输出 0，MSB 对应输出 31。可以设置任意数量的位，也可以不设置。

	当驱动不枚举音频输出时，不得设置任何位。应用程序不应将此解释为缺乏音频支持。驱动可以在不枚举的情况下自动选择音频输出。

	关于音频输出以及如何选择当前输出的细节，请参见 audio。
    - - __u32
      - `modulator`
      - 输出设备可以有零个或多个 RF 调制器。当 `type` 为 `V4L2_OUTPUT_TYPE_MODULATOR` 时，这是一个 RF 连接器，该字段标识调制器。它对应 struct `v4l2_modulator` 的 `index` 字段。关于调制器的细节，请参见 tuner。
    - - v4l2_std_id <v4l2-std-id>
      - `std`
      - 每个视频输出支持一种或多种不同的视频标准。该字段是所有支持标准的集合。关于视频标准及如何切换的细节，请参见 standard。
    - - __u32
      - `capabilities`
      - 该字段提供输出的能力。参见 output-capabilities 中的标志。
    - - __u32
      - `reserved`\ [^3^]
      - 为未来扩展保留。驱动必须将数组置零。





    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_OUTPUT_TYPE_MODULATOR`
      - 1
      - 此输出为模拟 TV 调制器。
    - - `V4L2_OUTPUT_TYPE_ANALOG`
      - 2
      - 任何非调制器的视频输出，例如 Composite Video、S-Video、HDMI。命名为 `_TYPE_ANALOG` 是历史原因，今天我们会称其为 `_TYPE_VIDEO`。
    - - `V4L2_OUTPUT_TYPE_ANALOGVGAOVERLAY`
      - 3
      - 视频输出将被复制到视频叠加 <overlay>。





    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_OUT_CAP_DV_TIMINGS`
      - 0x00000002
      - 此输出支持使用 `VIDIOC_S_DV_TIMINGS` 设置视频时序。
    - - `V4L2_OUT_CAP_STD`
      - 0x00000004
      - 此输出支持使用 `VIDIOC_S_STD` 设置 TV 标准。
    - - `V4L2_OUT_CAP_NATIVE_SIZE`
      - 0x00000008
      - 此输出支持使用 `V4L2_SEL_TGT_NATIVE_SIZE` 选择目标设置原生尺寸，请参见 v4l2-selections-common。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述。

EINVAL
    struct `v4l2_output` 的 `index` 越界。
