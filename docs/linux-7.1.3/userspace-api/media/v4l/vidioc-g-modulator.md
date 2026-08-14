


######## ioctl VIDIOC_G_MODULATOR, VIDIOC_S_MODULATOR


## Name


VIDIOC_G_MODULATOR - VIDIOC_S_MODULATOR - 获取或设置调制器属性

## Synopsis


`int ioctl(int fd, VIDIOC_G_MODULATOR, struct v4l2_modulator *argp)`


`int ioctl(int fd, VIDIOC_S_MODULATOR, const struct v4l2_modulator *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_modulator` 的指针。

## Description


要查询调制器的属性，应用程序初始化 struct `v4l2_modulator` 的 `index` 字段并将
`reserved` 数组清零，然后以指向该结构的指针调用
VIDIOC_G_MODULATOR <VIDIOC_G_MODULATOR> ioctl。驱动填写结构的其余部分，或者当
index 越界时返回 `EINVAL` 错误码。要枚举所有调制器，应用程序应从 index 零开始，
每次加一，直到驱动返回 EINVAL。

调制器有两个可写属性：一个音频调制集和射频。要改变被调制的音频子节目，应用程序
初始化 `index` 和 `txsubchans` 字段以及 `reserved` 数组，然后调用
VIDIOC_S_MODULATOR <VIDIOC_G_MODULATOR> ioctl。如果请求无法被满足，驱动可以选择不同的
音频调制。然而这是一个只写 ioctl，它不会返回实际被选中的音频调制。

SDR <sdr> 特定的调制器类型是 `V4L2_TUNER_SDR` 和 `V4L2_TUNER_RF`。对于 SDR 设备
`txsubchans` 字段必须初始化为零。在此上下文中，术语 “modulator” 指 SDR 发射器。

要改变射频，可使用 VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2 1 1

    - - __u32
      - `index`
      - 标识调制器，由应用程序设置。
    - - __u8
      - `name`\ [^32^]
      - 调制器的名称，一个以 NUL 结尾的 ASCII 字符串。

	此信息面向用户。
    - - __u32
      - `capability`
      - 调制器能力标志。此字段没有定义标志，相应地使用 struct `v4l2_tuner` 中的
	tuner 标志。音频标志指示编码音频子节目的能力。例如它们**不会**随当前视频
	标准而改变。
    - - __u32
      - `rangelow`
      - 最低可调频率，单位为 62.5 KHz；或者如果 `capability` 标志
	`V4L2_TUNER_CAP_LOW` 被设置，单位为 62.5 Hz；或者如果 `capability` 标志
	`V4L2_TUNER_CAP_1HZ` 被设置，单位为 1 Hz。
    - - __u32
      - `rangehigh`
      - 最高可调频率，单位为 62.5 KHz；或者如果 `capability` 标志
	`V4L2_TUNER_CAP_LOW` 被设置，单位为 62.5 Hz；或者如果 `capability` 标志
	`V4L2_TUNER_CAP_1HZ` 被设置，单位为 1 Hz。
    - - __u32
      - `txsubchans`
      - 应用程序通过此字段确定音频副载波应如何被调制。它包含一组如
	modulator-txsubchans 中所定义的标志。

```

	   The tuner ``rxsubchans`` flags  are reused, but the
	   semantics are different. Video output devices
	   are assumed to have an analog or PCM audio input with 1-3
	   channels. The ``txsubchans`` flags select one or more channels
	   for modulation, together with some audio subprogram indicator,
	   for example, a stereo pilot tone.
    * - __u32
      - ``type``
      - :cspan:`2` Type of the modulator, see :c:type:`v4l2_tuner_type`.
    * - __u32
      - ``reserved``\ [3]
      - Reserved for future extensions.

	Drivers and applications must set the array to zero.

```




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TUNER_SUB_MONO`
      - 0x0001
      - 将通道 1 调制为单声道音频；当输入有更多通道时，为通道 1 和 2 的下混
	（down-mix）。此标志不与 `V4L2_TUNER_SUB_STEREO` 或
	`V4L2_TUNER_SUB_LANG1` 组合。
    - - `V4L2_TUNER_SUB_STEREO`
      - 0x0002
      - 将通道 1 和 2 调制为立体声音频信号的左声道和右声道。当输入只有一个通道，
	或有两个通道且同时设置了 `V4L2_TUNER_SUB_SAP` 时，通道 1 被编码为左声道和
	右声道。此标志不与 `V4L2_TUNER_SUB_MONO` 或 `V4L2_TUNER_SUB_LANG1` 组合。
	当驱动不支持立体声音频时，应回退到单声道。
    - - `V4L2_TUNER_SUB_LANG1`
      - 0x0008
      - 将通道 1 和 2 调制为双语音频信号的主要语言和次要语言。当输入只有一个通道
	时，它用于两种语言。无法仅编码主要或次要语言。此标志不与
	`V4L2_TUNER_SUB_MONO`、`V4L2_TUNER_SUB_STEREO` 或 `V4L2_TUNER_SUB_SAP`
	组合。如果硬件不支持相应的音频矩阵，或者当前视频标准不允许双语音频，则
	VIDIOC_S_MODULATOR <VIDIOC_G_MODULATOR> ioctl 应返回 `EINVAL` 错误码，驱动应
	回退到单声道或立体声模式。
    - - `V4L2_TUNER_SUB_LANG2`
      - 0x0004
      - 与 `V4L2_TUNER_SUB_SAP` 效果相同。
    - - `V4L2_TUNER_SUB_SAP`
      - 0x0004
      - 当与 `V4L2_TUNER_SUB_MONO` 组合时，第一个通道被编码为单声道音频，最后一个
	通道作为第二音频节目（Second Audio Program）。当输入只有一个通道时，它用于
	所有音轨。当输入有三个通道时，单声道音轨是通道 1 和 2 的下混。当与
	`V4L2_TUNER_SUB_STEREO` 组合时，通道 1 和 2 被编码为左右立体声音频，通道 3
	作为第二音频节目。当输入只有两个通道时，第一个被编码为左声道和右声道，第二
	个作为 SAP。当输入只有一个通道时，它用于所有音轨。无法仅编码第二音频节目。
	此标志必须与 `V4L2_TUNER_SUB_MONO` 或 `V4L2_TUNER_SUB_STEREO` 组合。如果
	硬件不支持相应的音频矩阵，或者当前视频标准不允许 SAP，则 VIDIOC_S_MODULATOR
	<VIDIOC_G_MODULATOR> ioctl 应返回 `EINVAL` 错误码，驱动应回退到单声道或立体声
	模式。
    - - `V4L2_TUNER_SUB_RDS`
      - 0x0010
      - 为 FM 收音机发射器启用 RDS 编码器。

## Return Value


成功时返回 0，出错时返回 -1 并且 `errno` 变量被相应地设置。通用错误码在
Generic Error Codes <gen-errors> 一章中描述。

EINVAL
    struct `v4l2_modulator` 的 `index` 越界。
