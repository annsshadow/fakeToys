


######## ioctl VIDIOC_G_TUNER, VIDIOC_S_TUNER


## 名称


VIDIOC_G_TUNER - VIDIOC_S_TUNER - 获取或设置调谐器属性

## 概要



`int ioctl(int fd, VIDIOC_G_TUNER, struct v4l2_tuner *argp)`


`int ioctl(int fd, VIDIOC_S_TUNER, const struct v4l2_tuner *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_tuner` 的指针。

## 描述


要查询某个调谐器的属性，应用程序初始化 struct `v4l2_tuner` 的 `index` 字段并将 `reserved` 数组清零，然后以指向该结构的指针调用 `VIDIOC_G_TUNER` ioctl。当索引越界时，驱动填充结构的其余部分或返回 `EINVAL` 错误码。要枚举所有调谐器，应用程序应从索引 0 开始，每次递增 1，直到驱动返回 `EINVAL`。

调谐器有两个可写属性：音频模式和无线电频率。要更改音频模式，应用程序初始化 `index`、`audmode` 和 `reserved` 字段并调用 `VIDIOC_S_TUNER` ioctl。这 **不会** 改变当前的调谐器，当前调谐器由当前视频输入决定。如果所请求的模式无效或不受支持，驱动可以选择一个不同的音频模式。由于这是一个只写 ioctl，它不会返回实际被选中的音频模式。

SDR <sdr> 特定的调谐器类型是 `V4L2_TUNER_SDR` 和 `V4L2_TUNER_RF`。对于 SDR 设备，`audmode` 字段必须初始化为零。在此上下文中，"tuner" 一词指的是 SDR 接收器。

要更改无线电频率，可使用 VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl。

 .. tabularcolumns:: |p{1.3cm}|p{3.0cm}|p{7.0cm}|p{5.8cm}|



    :header-rows:  0
    :stub-columns: 0

    - - __u32
      - `index`
      - `1` 标识调谐器，由应用程序设置。
    - - __u8
      - `name`\ [^32^]
      - `1`

	调谐器的名称，一个以 NUL 结尾的 ASCII 字符串。

	该信息供用户使用。
    - - __u32
      - `type`
      - `1` 调谐器的类型，参见 `v4l2_tuner_type`。
    - - __u32
      - `capability`
      - `1`

	调谐器能力标志，参见 tuner-capability。音频标志表示解码音频子节目（subprogram）的能力。它们 **不会** 改变，例如不会随当前视频标准而改变。

	当该结构引用一个无线电调谐器时，`V4L2_TUNER_CAP_LANG1`、`V4L2_TUNER_CAP_LANG2` 和 `V4L2_TUNER_CAP_NORM` 标志不能使用。

	如果支持多个频带，则 `capability` 是每个 struct `v4l2_frequency_band` 的所有 `capability` 字段的并集。
    - - __u32
      - `rangelow`
      - `1` 最低可调频率，单位为 62.5 kHz；如果设置了 `capability` 标志 `V4L2_TUNER_CAP_LOW`，则单位为 62.5 Hz；如果设置了 `capability` 标志 `V4L2_TUNER_CAP_1HZ`，则单位为 1 Hz。如果支持多个频带，则 `rangelow` 是所有频带中最低的频率。
    - - __u32
      - `rangehigh`
      - `1` 最高可调频率，单位为 62.5 kHz；如果设置了 `capability` 标志 `V4L2_TUNER_CAP_LOW`，则单位为 62.5 Hz；如果设置了 `capability` 标志 `V4L2_TUNER_CAP_1HZ`，则单位为 1 Hz。如果支持多个频带，则 `rangehigh` 是所有频带中最高的频率。
    - - __u32
      - `rxsubchans`
      - `1`

	某些调谐器或音频解码器可以通过分析音频载波、导频音或其他指示器来确定接收到的音频子节目。为了传递该信息，驱动在本字段中设置 tuner-rxsubchans 中定义的标志。例如：
#     * -

      - `V4L2_TUNER_SUB_MONO`
      - 接收单声道音频
#     * -

      - `STEREO | SAP`
      - 接收立体声音频和一个辅助音频节目
#     * -

      - `MONO | STEREO`
      - 接收单声道或立体声音频，硬件无法区分
#     * -

      - `LANG1 | LANG2`
      - 接收双语音频
#     * -

      - `MONO | STEREO | LANG1 | LANG2`
      - 接收单声道、立体声或双语音频
#     * -

      - `1`

	当 `capability` 字段中的 `V4L2_TUNER_CAP_STEREO`、`_LANG1`、`_LANG2` 或 `_SAP` 标志被清除时，此处不得设置相应的 `V4L2_TUNER_SUB_` 标志。

	本字段仅在它是当前视频输入的调谐器，或者该结构引用一个无线电调谐器时才有效。
    - - __u32
      - `audmode`
      - `1`

	所选的音频模式，有效取值参见 tuner-audmode。音频模式不影响音频子节目的检测，并且像控制一样，除非所请求的模式无效或不受支持，否则不会自动改变。关于所选音频节目与接收到的音频节目不匹配时可能的结果，参见 tuner-matrix。

	目前这是应用程序能够更改的 struct `v4l2_tuner` 的唯一字段。
    - - __u32
      - `signal`
      - `1` 信号强度（如果已知）。

	取值范围为 0 到 65535。数值越大表示信号越好。
    - - __s32
      - `afc`
      - `1` 自动频率控制。

	当 `afc` 值为负时，频率偏低；为正时，频率偏高。
    - - __u32
      - `reserved`\ [^4^]
      - `1` 为未来扩展保留。

	驱动和应用程序都必须将该数组置零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 6

    - - `V4L2_TUNER_RADIO`
      - 1
      - 调谐器支持无线电
    - - `V4L2_TUNER_ANALOG_TV`
      - 2
      - 调谐器支持模拟电视
    - - `V4L2_TUNER_SDR`
      - 4
      - 调谐器控制软件数字无线电（SDR）的 A/D 和/或 D/A 模块
    - - `V4L2_TUNER_RF`
      - 5
      - 调谐器控制软件数字无线电（SDR）的射频部分



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TUNER_CAP_LOW`
      - 0x0001
      - 设置时，调谐频率以 62.5 Hz 为单位，而非 62.5 kHz。
    - - `V4L2_TUNER_CAP_NORM`
      - 0x0002
      - 这是一个多标准调谐器；视频标准可以或必须被切换。（例如 B/G PAL 调谐器通常不被视为多标准，因为视频标准是根据频带自动确定的。）所支持的视频标准集合可从指向该调谐器的 struct `v4l2_input` 获取，详见 ioctl VIDIOC_ENUMINPUT 的描述。只有 `V4L2_TUNER_ANALOG_TV` 调谐器可以具有此能力。
    - - `V4L2_TUNER_CAP_HWSEEK_BOUNDED`
      - 0x0004
      - 如果设置，则该调谐器支持硬件搜索功能，当搜索到达频率范围末端时停止。
    - - `V4L2_TUNER_CAP_HWSEEK_WRAP`
      - 0x0008
      - 如果设置，则该调谐器支持硬件搜索功能，当搜索到达频率范围末端时回绕。
    - - `V4L2_TUNER_CAP_STEREO`
      - 0x0010
      - 支持立体声音频接收。
    - - `V4L2_TUNER_CAP_LANG1`
      - 0x0040
      - 支持接收双语音频节目的主要语言。双语音频是双通道系统的特性，在主音频载波上单声道传输主要语言，在第二个载波上单声道传输次要语言。只有 `V4L2_TUNER_ANALOG_TV` 调谐器可以具有此能力。
    - - `V4L2_TUNER_CAP_LANG2`
      - 0x0020
      - 支持接收双语音频节目的次要语言。只有 `V4L2_TUNER_ANALOG_TV` 调谐器可以具有此能力。
    - - `V4L2_TUNER_CAP_SAP`
      - 0x0020
      - 支持接收辅助音频节目。这是伴随 NTSC 视频标准的 BTSC 系统的特性。主要语言的单声道或立体声传输有两个音频载波可用，此外还有一个独立的第三载波用于单声道次要语言。只有 `V4L2_TUNER_ANALOG_TV` 调谐器可以具有此能力。

```

	   ``V4L2_TUNER_CAP_LANG2`` 和 ``V4L2_TUNER_CAP_SAP``
	   标志是同义词。``V4L2_TUNER_CAP_SAP`` 适用于支持
	   ``V4L2_STD_NTSC_M`` 视频标准的调谐器。
    * - ``V4L2_TUNER_CAP_RDS``
      - 0x0080
      - 支持 RDS 捕获。此能力仅对无线电调谐器有效。
    * - ``V4L2_TUNER_CAP_RDS_BLOCK_IO``
      - 0x0100
      - RDS 数据以未解析的 RDS 块形式传递。
    * - ``V4L2_TUNER_CAP_RDS_CONTROLS``
      - 0x0200
      - RDS 数据由硬件解析并通过控制设置。
    * - ``V4L2_TUNER_CAP_FREQ_BANDS``
      - 0x0400
      - 可以使用 :ref:`VIDIOC_ENUM_FREQ_BANDS`
	ioctl 来枚举可用的频带。
    * - ``V4L2_TUNER_CAP_HWSEEK_PROG_LIM``
      - 0x0800
      - 使用硬件搜索功能时搜索的范围是可编程的，详见
	:ref:`VIDIOC_S_HW_FREQ_SEEK`。
    * - ``V4L2_TUNER_CAP_1HZ``
      - 0x1000
      - 设置时，调谐频率以 1 Hz 为单位，而非 62.5 kHz。


```

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TUNER_SUB_MONO`
      - 0x0001
      - 调谐器接收单声道音频信号。
    - - `V4L2_TUNER_SUB_STEREO`
      - 0x0002
      - 调谐器接收立体声音频信号。
    - - `V4L2_TUNER_SUB_LANG1`
      - 0x0008
      - 调谐器接收双语音频信号的主要语言。当当前视频标准为 `V4L2_STD_NTSC_M` 时，驱动必须清除此标志。
    - - `V4L2_TUNER_SUB_LANG2`
      - 0x0004
      - 调谐器接收双语音频信号（或第二个音频节目）的次要语言。
    - - `V4L2_TUNER_SUB_SAP`
      - 0x0004
      - 调谐器接收辅助音频节目。

```

	   ``V4L2_TUNER_SUB_LANG2`` 和 ``V4L2_TUNER_SUB_SAP``
	   标志是同义词。``V4L2_TUNER_SUB_SAP`` 标志适用于当前视频标准为
	   ``V4L2_STD_NTSC_M`` 的情况。
    * - ``V4L2_TUNER_SUB_RDS``
      - 0x0010
      - 调谐器接收 RDS 信道。


```

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TUNER_MODE_MONO`
      - 0
      - 播放单声道音频。当调谐器接收立体声信号时，这是左右声道的下混。当调谐器接收双语或 SAP 信号时，此模式选择主要语言。
    - - `V4L2_TUNER_MODE_STEREO`
      - 1
      - 播放立体声音频。当调谐器接收双语音频时，它可能在左、右声道播放不同语言，或在两个声道播放主要语言。

	在此模式下播放不同语言的做法已被弃用。新的驱动只应在 `MODE_LANG1_LANG2` 中这样做。

	当调谐器未接收到立体声信号或不支持立体声接收时，驱动应回退到 `MODE_MONO`。
    - - `V4L2_TUNER_MODE_LANG1`
      - 3
      - 播放主要语言，单声道或立体声。只有 `V4L2_TUNER_ANALOG_TV` 调谐器支持此模式。
    - - `V4L2_TUNER_MODE_LANG2`
      - 2
      - 播放次要语言，单声道。当调谐器未接收到双语音频或 SAP，或其接收不受支持时，驱动应回退到单声道或立体声模式。只有 `V4L2_TUNER_ANALOG_TV` 调谐器支持此模式。
    - - `V4L2_TUNER_MODE_SAP`
      - 2
      - 播放辅助音频节目。当调谐器未接收到双语音频或 SAP，或其接收不受支持时，驱动应回退到单声道或立体声模式。只有 `V4L2_TUNER_ANALOG_TV` 调谐器支持此模式。

	.. note:: `V4L2_TUNER_MODE_LANG2` 和 `V4L2_TUNER_MODE_SAP` 是同义词。
    - - `V4L2_TUNER_MODE_LANG1_LANG2`
      - 4
      - 在左声道播放主要语言，在右声道播放次要语言。当调谐器未接收到双语音频或 SAP 时，它应回退到 `MODE_LANG1` 或 `MODE_MONO`。只有 `V4L2_TUNER_ANALOG_TV` 调谐器支持此模式。


    \scriptsize



    :header-rows:  2
    :stub-columns: 0
    :widths: 7 7 14 14 14 14

    - -
      - `4` 所选 `V4L2_TUNER_MODE_`
    - - 接收到的 `V4L2_TUNER_SUB_`
      - `MONO`
      - `STEREO`
      - `LANG1`
      - `LANG2 = SAP`
      - `LANG1_LANG2`\ [#f1]_
    - - `MONO`
      - 单声道
      - 单声道/单声道
      - 单声道
      - 单声道
      - 单声道/单声道
    - - `MONO | SAP`
      - 单声道
      - 单声道/单声道
      - 单声道
      - SAP
      - 单声道/SAP（优先）或单声道/单声道
    - - `STEREO`
      - L+R
      - L/R
      - 立体声 L/R（优先）或单声道 L+R
      - 立体声 L/R（优先）或单声道 L+R
      - L/R（优先）或 L+R/L+R
    - - `STEREO | SAP`
      - L+R
      - L/R
      - 立体声 L/R（优先）或单声道 L+R
      - SAP
      - L+R/SAP（优先）或 L/R 或 L+R/L+R
    - - `LANG1 | LANG2`
      - 语言 1
      - Lang1/Lang2（已弃用\ [#f2]_）或 Lang1/Lang1
      - 语言 1
      - 语言 2
      - Lang1/Lang2（优先）或 Lang1/Lang1


    \normalsize

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误码 <gen-errors> 一章中描述。

EINVAL
    struct `v4l2_tuner` 的 `index` 越界。

   该模式是在 Linux 2.6.17 中加入的，较旧的驱动可能不支持。

   在 `MODE_STEREO` 中播放两种语言的做法已被弃用。将来驱动在此模式下应只产生主要语言。应用程序应当请求 `MODE_LANG1_LANG2` 以录制两种语言或立体声信号。
