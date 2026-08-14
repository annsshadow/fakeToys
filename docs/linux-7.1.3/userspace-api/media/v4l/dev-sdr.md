######## 软件定义无线电接口（SDR）


SDR 是 Software Defined Radio（软件定义无线电）的缩写，即使用应用软件进行
调制或解调的无线电设备。该接口用于控制此类设备并进行数据流传输。

SDR 设备通过名为 `/dev/swradio0` 到 `/dev/swradio255` 的字符设备特殊文件
访问，主设备号为 81，次设备号在 0 到 255 之间动态分配。

## 查询能力


支持 SDR 接收器接口的设备，会在 VIDIOC_QUERYCAP ioctl 返回的 struct
`v4l2_capability` 的 `capabilities` 字段中设置
`V4L2_CAP_SDR_CAPTURE` 和 `V4L2_CAP_TUNER` 标志。该标志表示设备具有
模数转换器（ADC），它是 SDR 接收器的必需元件。

支持 SDR 发送器接口的设备，会在 VIDIOC_QUERYCAP ioctl 返回的 struct
`v4l2_capability` 的 `capabilities` 字段中设置
`V4L2_CAP_SDR_OUTPUT` 和 `V4L2_CAP_MODULATOR` 标志。该标志表示设备具有
数模转换器（DAC），它是 SDR 发送器的必需元件。

必须至少支持读/写或流 I/O 方法之一。

## 辅助功能


SDR 设备可以支持 controls <control>，并且必须支持 tuner ioctls。tuner
ioctls 用于设置 ADC/DAC 采样率（采样频率）以及可能的无线电频率（RF）。

`V4L2_TUNER_SDR` tuner 类型用于设置 SDR 设备的 ADC/DAC 频率，`V4L2_TUNER_RF`
tuner 类型用于设置无线电频率。RF tuner（若有）的 tuner 索引必须始终跟随
SDR tuner 索引。通常 SDR tuner 为 #0，RF tuner 为 #1。

VIDIOC_S_HW_FREQ_SEEK ioctl 不受支持。

## 数据格式协商


SDR 设备使用格式 ioctls 来选择捕获和输出格式。采样分辨率和数据流式格式都
绑定到该可选择的格式。除基本的格式 ioctls 外，还必须支持
VIDIOC_ENUM_FMT ioctl。

要使用格式 ioctls，应用程序将 struct `v4l2_format` 的 `type` 字段设为
`V4L2_BUF_TYPE_SDR_CAPTURE` 或 `V4L2_BUF_TYPE_SDR_OUTPUT`，并根据所需
操作按需使用 struct `v4l2_sdr_format` 的 `fmt` 联合的 `sdr` 成员。
目前使用了 struct `v4l2_sdr_format` 的两个字段：`pixelformat` 和
`buffersize`。`pixelformat` 的内容是数据格式的 V4L2 FourCC 码。`buffersize`
字段是数据传输所需的最大缓冲区字节数，由驱动设置以告知应用程序。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pixelformat`
      - 数据格式或压缩类型，由应用程序设置。这是一个小端
	四字符码 <v4l2-fourcc>。V4L2 在 sdr-formats 中定义了 SDR
	格式。
    - - __u32
      - `buffersize`
      - 数据所需的最大字节数。值由驱动设置。
    - - __u8
      - `reserved[^24^]`
      - 该数组为未来扩展保留。驱动和应用程序必须将其置零。


SDR 设备可能支持 read/write <rw> 和/或流式
（内存映射 <mmap> 或用户指针 <userp>）I/O。
