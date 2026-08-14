
## 虚拟视频测试驱动（vivid）


该驱动可模拟多种类型的 video4linux 硬件：视频采集、视频输出、VBI 采集与输出、元数据采集与输出、无线电接收与发射、触摸采集，以及软件定义无线电（SDR）接收器。此外，还提供一个简单的帧缓冲设备，用于测试采集与输出叠加（overlay）。

最多可创建 64 个 vivid 实例，每个实例最多有 16 个输入和 16 个输出。

每个输入可以是网络摄像头、TV 采集设备、S-Video 采集设备或 HDMI 采集设备。每个输出可以是 S-Video 输出设备或 HDMI 输出设备。

这些输入与输出表现得与真实硬件设备完全相同。这使你可以将该驱动作为应用程序开发的测试输入，因为你可以测试各种特性而无需特殊硬件。

本文档描述该驱动实现的特性：

- 支持 read()/write()、MMAP、USERPTR 与 DMABUF 流式 I/O。
- 大量测试图案及其变体
- 可用的亮度、对比度、饱和度与色调控制
- 支持 alpha 颜色分量
- 完整的色彩空间支持，包括受限/全范围 RGB
- 具备所有可能的控制类型
- 支持各种像素宽高比与视频宽高比
- 错误注入，用于测试发生错误时的行为
- 支持输入与输出的任意组合裁剪/合成/缩放
- 可模拟高达 4K 分辨率
- 支持所有 Field 设置，用于测试隔行采集
- 支持所有标准 YUV 与 RGB 格式，包括两种多平面 YUV 格式
- Raw 与 Sliced VBI 采集和输出支持
- 无线电接收与发射支持，包括 RDS 支持
- 软件定义无线电（SDR）支持
- 采集与输出叠加支持
- 元数据采集与输出支持
- 触摸采集支持

这些特性将在下文更详细地描述。

### 配置驱动


默认情况下，驱动会创建单个实例，包含一个带有 webcam、TV、S-Video 和 HDMI 输入的视频采集设备，一个带有 S-Video 与 HDMI 输出的视频输出设备，一个 VBI 采集设备，一个 VBI 输出设备，一个无线电接收设备，一个无线电发射设备以及一个 SDR 设备。

实例数量、设备数量、视频输入与输出及其类型，均可使用以下模块选项配置：

- n_devs:

	要创建的驱动实例数量。默认设为 1。最多可创建 64 个实例。

- node_types:

	每个驱动实例应创建哪些设备。一个十六进制值数组，每个实例一个。默认值为 0xe1d3d。
	每个值是一个位掩码，含义如下：

  - bit 0：视频采集节点
  - bit 2-3：VBI 采集节点：0 = 无，1 = raw vbi，2 = sliced vbi，3 = 两者
  - bit 4：无线电接收节点
  - bit 5：软件定义无线电接收节点
  - bit 8：视频输出节点
  - bit 10-11：VBI 输出节点：0 = 无，1 = raw vbi，2 = sliced vbi，3 = 两者
  - bit 12：无线电发射节点
  - bit 16：用于测试叠加的帧缓冲
  - bit 17：元数据采集节点
  - bit 18：元数据输出节点
  - bit 19：触摸采集节点

	因此，若要创建四个实例，其中前两个各只有一个视频采集设备、后两个各只有一个视频输出设备，可以向 vivid 传递以下模块选项：

	.. code-block:: none

		n_devs=4 node_types=0x1,0x1,0x100,0x100

- num_inputs:
	输入数量，每个实例一个。默认每个视频采集设备创建 4 个输入。最多可创建 16 个输入，且至少要有 1 个。

- input_types:

	每个实例的输入类型，默认值为 0xe4。它定义了为每个驱动实例创建输入时，每个输入的类型是什么。这是一个十六进制值，最多 16 对比特，每对给出一种类型，比特 0-1 映射到输入 0，比特 2-3 映射到输入 1，比特 30-31 映射到输入 15。每对比特含义如下：

  - 00：这是一个 webcam 输入
  - 01：这是一个 TV 调谐器输入
  - 10：这是一个 S-Video 输入
  - 11：这是一个 HDMI 输入

	因此，若要创建一个带有 8 个输入的视频采集设备，其中输入 0 为 TV 调谐器、输入 1-3 为 S-Video 输入、输入 4-7 为 HDMI 输入，可使用以下模块选项：

	.. code-block:: none

		num_inputs=8 input_types=0xffa9

- num_outputs:

	输出数量，每个实例一个。默认每个视频输出设备创建 2 个输出。最多可创建 16 个输出，且至少要有 1 个。

- output_types:

	每个实例的输出类型，默认值为 0x02。它定义了为每个驱动实例创建输出时，每个输出的类型是什么。这是一个十六进制值，最多 16 比特，每个比特给出一种类型，比特 0 映射到输出 0，比特 1 映射到输出 1，比特 15 映射到输出 15。每个比特的含义如下：

  - 0：这是一个 S-Video 输出
  - 1：这是一个 HDMI 输出

	因此，若要创建一个带有 8 个输出的视频输出设备，其中输出 0-3 为 S-Video 输出、输出 4-7 为 HDMI 输出，可使用以下模块选项：

	.. code-block:: none

		num_outputs=8 output_types=0xf0

- vid_cap_nr:

	给出每个视频采集设备期望的 videoX 起始编号。默认值为 -1，即直接取第一个空闲编号。这使你可以将采集视频节点映射到特定的 videoX 设备节点。例如：

	.. code-block:: none

		n_devs=4 vid_cap_nr=2,4,6,8

	这将尝试为第一个 vivid 实例的视频采集设备分配 /dev/video2，下一个为 video4，依此类推，最后一个实例为 video8。若无法成功，则会直接取下一个空闲编号。

- vid_out_nr:

	给出每个视频输出设备期望的 videoX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- vbi_cap_nr:

	给出每个 VBI 采集设备期望的 vbiX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- vbi_out_nr:

	给出每个 VBI 输出设备期望的 vbiX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- radio_rx_nr:

	给出每个无线电接收设备期望的 radioX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- radio_tx_nr:

	给出每个无线电发射设备期望的 radioX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- sdr_cap_nr:

	给出每个 SDR 采集设备期望的 swradioX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- meta_cap_nr:

        给出每个元数据采集设备期望的 videoX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- meta_out_nr:
        给出每个元数据输出设备期望的 videoX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- touch_cap_nr:

        给出每个触摸采集设备期望的 v4l-touchX 起始编号。默认值为 -1，即直接取第一个空闲编号。

- ccs_cap_mode:

	指定每个驱动实例允许的组合：视频采集裁剪/合成/缩放。视频采集设备可以具有裁剪、合成与缩放能力的任意组合，这个选项会告诉 vivid 驱动应当模拟其中的哪些。默认情况下用户可通过控件选择。

	该值要么是 -1（由用户控制），要么是一组三个比特，每个比特启用（1）或禁用（0）其中一个特性：

 - bit 0:

		启用裁剪支持。裁剪将只取 incoming 画面的一部分。

 - bit 1:

		启用合成支持。合成会把 incoming 画面拷贝到一个更大的缓冲区中。

 - bit 2:

		启用缩放支持。缩放可以对 incoming 画面进行缩放。vivid 驱动的缩放器可将原始尺寸放大或缩小到最多四倍。该缩放器非常简单且质量较低，设计时优先考虑简单与速度，而非质量。

	注意，webcam 输入会忽略该值：它们枚举离散的帧尺寸，这与裁剪、合成或缩放不兼容。

- ccs_out_mode:

	指定每个驱动实例允许的组合：视频输出裁剪/合成/缩放。视频输出设备可以具有裁剪、合成与缩放能力的任意组合，这个选项会告诉 vivid 驱动应当模拟其中的哪些。默认情况下用户可通过控件选择。

	该值要么是 -1（由用户控制），要么是一组三个比特，每个比特启用（1）或禁用（0）其中一个特性：

 - bit 0:

		启用裁剪支持。裁剪将只取 outgoing 缓冲区的一部分。

 - bit 1:

		启用合成支持。合成会把 incoming 缓冲区拷贝到一个更大的画面帧中。

 - bit 2:

		启用缩放支持。缩放可以对 incoming 缓冲区进行缩放。vivid 驱动的缩放器可将原始尺寸放大或缩小到最多四倍。该缩放器非常简单且质量较低，设计时优先考虑简单与速度，而非质量。

- multiplanar:

	选择每个设备实例是否支持多平面格式，从而支持 V4L2 多平面 API。默认情况下设备实例为单平面。

	该模块选项可为每个实例覆盖此设置。取值为：

  - 1：这是一个单平面实例。
  - 2：这是一个多平面实例。

- vivid_debug:

	启用驱动调试信息

- no_error_inj:

	若设置则禁用错误注入控件。运行 v4l2-compliance 之类的工具时需要此选项。这类工具会遍历所有控件，包括像“Disconnect”这样的控件，它会模拟一次 USB 断开，使设备无法访问，从而导致此后 v4l2-compliance 执行的所有测试都会失败。

	也可能存在其他需要禁用 vivid 错误注入支持的情况。设置此选项后，选择裁剪、合成与缩放行为的控件也会被移除。除非被 ccs_cap_mode 和/或 ccs_out_mode 覆盖，否则将默认启用裁剪、合成与缩放。

- allocators:

	内存分配器选择，默认值为 0。它指定缓冲区的分配方式。

  - 0：vmalloc
  - 1：dma-contig
- cache_hints:

	指定设备是否应设置队列的用户空间缓存与内存一致性提示能力（V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS）。这些提示仅在使用 MMAP 流式 I/O 时有效。默认值为 0。

  - 0：禁止提示
  - 1：允许提示

- supports_requests:

	指定设备是否支持 Request API。有三种可能取值，默认值为 1：

  - 0：无请求
  - 1：支持请求
  - 2：需要请求

综上所述，所有这些模块选项使你能够精确自定义驱动行为，并用各种排列组合来测试你的应用程序。它也非常适合用于模拟尚不可用的硬件，例如为即将推出的新设备开发软件。


### 视频采集


这可能是最常被使用的特性。视频采集设备可以使用模块选项 num_inputs、input_types 与 ccs_cap_mode 进行配置（详见“配置驱动”），但默认会配置四个输入：一个 webcam、一个 TV 调谐器、一个 S-Video 与一个 HDMI 输入，每种输入类型各一个。这些将在下文更详细地描述。

对于新帧变为可用的速率，我们给予了特别关注。抖动大约在 1 个 jiffy（取决于你内核的 HZ 配置，通常是 1/100、1/250 或 1/1000 秒），但长期行为会精确遵循帧率。因此 59.94 Hz 的帧率与 60 Hz 确实有区别。如果帧率超过了你内核的 HZ 值，就会出现丢帧，但帧/场序列计数会跟踪这一情况，每当丢帧时序列计数就会跳过。

#### Webcam 输入


webcam 输入支持三种帧尺寸：320x180、640x360 与 1280x720。它支持 10、15、25、30、50 和 60 fps 的每秒帧数设置。具体哪些可用取决于所选帧尺寸：帧尺寸越大，最大每秒帧数越低。

切换到 webcam 输入时初始选择的色彩空间为 sRGB。

#### TV 与 S-Video 输入


TV 与 S-Video 输入之间唯一的区别是 TV 带有调谐器，除此之外它们行为完全一致。

这些输入也支持音频输入：一个 TV 和一个 Line-In。它们都支持所有 TV 标准。若查询标准，则由 Vivid 控件“Standard Signal Mode”与“Standard”决定结果。

这些输入支持 field 设置的所有组合。我们特别用心地忠实还原了不同 TV 标准下场的处理方式。当生成一幅水平移动的图像时，这一点尤为明显——使用隔行格式带来的时间效应会清晰可见。对于 50 Hz 标准，顶场在时间上最旧，底场最新；对于 60 Hz 标准则相反：底场最旧，顶场最新。

当你以 V4L2_FIELD_ALTERNATE 模式开始采集时，第一个缓冲区对于 50 Hz 标准将包含顶场，对于 60 Hz 标准将包含底场。真实采集硬件也是如此。

最后，对于 PAL/SECAM 标准，顶行的上半部分包含噪声，用于模拟通常放置在那里的宽屏信号（Wide Screen Signal）。

切换到 TV 或 S-Video 输入时初始选择的色彩空间为 SMPTE-170M。
像素宽高比取决于 TV 标准。视频宽高比可以通过“Standard Aspect Ratio”Vivid 控件选择。可选值为“4x3”、“16x9”（会得到加黑边的宽屏视频），以及“16x9 Anamorphic”（会得到全屏但被压扁的变形宽屏视频，需要相应缩放）。

TV“调谐器”支持 44-958 MHz 的频率范围。从 49.25 MHz 起，每 6 MHz 一个频道。对于每个频道，在其 +/- 0.25 MHz 范围内生成的画面为彩色，在 +/- 1 MHz 范围内为灰阶，范围之外则只有噪声。VIDIOC_G_TUNER ioctl 会在 +/- 0.25 MHz 时返回 100% 信号强度，在 +/- 1 MHz 时返回 50%。它还会返回正确的 afc 值，以表明频率是偏低还是偏高。

返回的音频子信道在有效频道频率的 +/- 1 MHz 范围内为 MONO。当频率在频道 +/- 0.25 MHz 范围内时，会返回 MONO、STEREO、MONO | SAP（NTSC）或 LANG1 | LANG2（其他标准），或 STEREO | SAP 之一。

具体返回哪一个取决于所选频道，每切换到一个新的有效频道就会循环经过可能的音频子信道组合。这使你可以仅通过切换频道来测试各种组合。

最后，对于这些输入，v4l2_timecode 结构体会被填入出队的 v4l2_buffer 结构体中。

#### HDMI 输入


HDMI 输入支持所有 CEA-861 与 DMT 时序，既支持逐行也支持隔行，像素时钟频率范围为 25 至 600 MHz。隔行格式的 field 模式始终为 V4L2_FIELD_ALTERNATE。对于 HDMI，场顺序始终顶场优先；当你开始采集隔行格式时，会先收到顶场。

切换到 HDMI 输入或选择一个 HDMI 时序时初始选择的色彩空间基于格式分辨率：对于分辨率小于或等于 720x576 的，色彩空间设为 SMPTE-170M；其他情况设为 REC-709（CEA-861 时序）或 sRGB（VESA DMT 时序）。

像素宽高比取决于 HDMI 时序：对于 720x480 按 NTSC TV 标准设置，对于 720x576 按 PAL TV 标准设置，其余情况返回 1:1 像素宽高比。

视频宽高比可以通过“DV Timings Aspect Ratio”Vivid 控件选择。可选值为“Source Width x Height”（使用与所选格式相同的比例）、“4x3”或“16x9”，后两者都可能产生左右加边或上下加边的视频。

对于 HDMI 输入，可以设置 EDID。默认提供一个简单的 EDID。你只能为 HDMI 输入设置 EDID。不过在内部，EDID 在所有 HDMI 输入之间共享。

除物理地址外，不会对 EDID 数据做任何解析。详见 CEC 章节。

HDMI 输入最多 15 个（如果更多，则会被缩减为 15 个），因为 EDID 物理地址存在这一上限。

### 视频输出


视频输出设备可以使用模块选项 num_outputs、output_types 与 ccs_out_mode 进行配置（详见“配置驱动”），但默认会配置两个输出：一个 S-Video 与一个 HDMI 输入，每种输出类型各一个。这些将在下文更详细地描述。

与视频采集类似，帧率在长期来看也是精确的。

#### S-Video 输出

该输出也支持音频输出：“Line-Out 1”与“Line-Out 2”。S-Video 输出支持所有 TV 标准。

该输出支持 field 设置的所有组合。

切换到 TV 或 S-Video 输入时初始选择的色彩空间为 SMPTE-170M。

#### HDMI 输出


HDMI 输出支持所有 CEA-861 与 DMT 时序，既支持逐行也支持隔行，像素时钟频率范围为 25 至 600 MHz。隔行格式的 field 模式始终为 V4L2_FIELD_ALTERNATE。

切换到 HDMI 输出或选择一个 HDMI 时序时初始选择的色彩空间基于格式分辨率：对于分辨率小于或等于 720x576 的，色彩空间设为 SMPTE-170M；其他情况设为 REC-709（CEA-861 时序）或 sRGB（VESA DMT 时序）。

像素宽高比取决于 HDMI 时序：对于 720x480 按 NTSC TV 标准设置，对于 720x576 按 PAL TV 标准设置，其余情况返回 1:1 像素宽高比。

HDMI 输出拥有一个有效的 EDID，可通过 VIDIOC_G_EDID 获取。

HDMI 输出最多 15 个（如果更多，则会被缩减为 15 个），因为 EDID 物理地址存在这一上限。另见 CEC 章节了解更多细节。

### VBI 采集


VBI 采集设备有三种类型：仅支持 raw（未解码）VBI 的、仅支持 sliced（已解码）VBI 的，以及两者都支持的。这由 node_types 模块选项决定。在所有情况下，驱动都会生成有效的 VBI 数据：对于 60 Hz 标准，会生成 Closed Caption 与 XDS 数据。字幕流会每秒在“Hello world!”与“Closed captions test”之间切换。XDS 流每分钟给出一次当前时间。对于 50 Hz 标准，会生成基于实际视频宽高比控件设置的宽屏信号（Wide Screen Signal）以及图文电视（teletext）第 100-159 页，每帧一页。

VBI 设备仅对 S-Video 与 TV 输入有效，如果当前输入是 webcam 或 HDMI，则会返回错误。

### VBI 输出


VBI 输出设备有三种类型：仅支持 raw（未解码）VBI 的、仅支持 sliced（已解码）VBI 的，以及两者都支持的。这由 node_types 模块选项决定。

sliced VBI 输出对于 50 Hz 标准支持宽屏信号与图文电视信号，对于 60 Hz 标准支持 Closed Captioning + XDS。

VBI 设备仅对 S-Video 输出有效，如果当前输出是 HDMI，则会返回错误。

### 无线电接收器


无线电接收器模拟 FM/AM/SW 接收器。FM 频段还支持 RDS。频率范围如下：

 - FM：64 MHz - 108 MHz
 - AM：520 kHz - 1710 kHz
 - SW：2300 kHz - 26.1 MHz

FM 每 1 MHz、AM 与 SW 每 100 kHz 模拟一个有效频道。信号强度随频率偏离有效频率越远而越弱，直到在距理想频率 +/- 50 kHz（FM）或 5 kHz（AM/SW）处降为 0%。驱动加载时的初始频率设为 95 MHz。

FM 接收器也支持 RDS，既可使用“Block I/O”模式，也可使用“Controls”模式。在“Controls”模式下，RDS 信息存储在只读控件中，这些控件会在每次频率改变或请求调谐器状态时更新。“Block I/O”方法使用 read() 接口将 RDS 块传递给应用程序进行解码。
RDS 信号在频道频率 +/- 12.5 kHz 范围内被“检测到”，频率偏离有效频率越远，Block I/O 流中会被随机引入越多的 RDS 错误，最多可达所有块的 50%（当你距频道频率 +/- 12.5 kHz 时）。四种错误会以相等比例出现：标记为“CORRECTED”的块、标记为“ERROR”的块、标记为“INVALID”的块，以及被丢弃的块。

生成的 RDS 流包含 0B 组中的所有标准字段，以及电台文本与当前时间。

接收器支持 HW 频率搜索，可在 Bounded 模式、Wrap Around 模式或两者下进行，这可通过“Radio HW Seek Mode”控件配置。

### 无线电发射器


无线电发射器模拟 FM/AM/SW 发射器。FM 频段也支持 RDS。频率范围如下：

 - FM：64 MHz - 108 MHz
 - AM：520 kHz - 1710 kHz
 - SW：2300 kHz - 26.1 MHz

驱动加载时的初始频率为 95.5 MHz。

FM 发射器也支持 RDS，既可使用“Block I/O”模式，也可使用“Controls”模式。在“Controls”模式下，发送的 RDS 信息通过控件配置；在“Block I/O”模式下，块通过 write() 传递给驱动。

### 软件定义无线电接收器


SDR 接收器的 ADC 调谐器有三个频段：

 - 300 kHz
 - 900 kHz - 2800 kHz
 - 3200 kHz

RF 调谐器支持 50 MHz - 2000 MHz。

生成的数据包含振幅为 sqrt(2) 的 1 kHz 单音的同相（In-phase）与正交（Quadrature）分量。

### 元数据采集


元数据采集生成 UVC 格式的元数据。PTS 与 SCR 基于 vivid 控件中设置的值进行传输。

元数据设备仅对 Webcam 输入有效，对所有其他输入会返回错误。

### 元数据输出


元数据输出可用于设置亮度、对比度、饱和度与色调。

元数据设备仅对 Webcam 输出有效，对所有其他输出会返回错误。

### 触摸采集


触摸采集生成触摸图案，模拟单点触摸、双点触摸、三点触摸、从左到右移动、放大、缩小、手掌按压（模拟触摸板上被大面积按压），以及 16 个同时触摸点。

### 控件


不同设备支持的控件不同。以下各节将描述每个控件以及哪些设备支持它们。

#### 用户控件 - 测试控件


Button、Boolean、Integer 32 Bits、Integer 64 Bits、Menu、String、Bitmask 与 Integer Menu 是代表所有可能控件类型的控件。Menu 控件与 Integer Menu 控件在其菜单列表中都有“空洞”，即调用 VIDIOC_QUERYMENU 时，一个或多个菜单项会返回 EINVAL。这两个菜单控件还具有非零的最小控件值。这些特性使你可以检查应用程序是否能正确处理此类情况。这些控件被所有设备类型支持。

#### 用户控件 - 视频采集


以下控件特定于视频采集。
亮度、对比度、饱和度与色调控件是标准控件，并且确实有效。亮度控件有一个特殊之处：每个视频输入都有自己的亮度值，因此切换输入时会恢复该输入的亮度。此外，每个视频输入使用的亮度范围（最小与最大控件值）不同。切换输入会触发一个控件事件，并设置 V4L2_EVENT_CTRL_CH_RANGE 标志。这使你可以测试能够改变范围的控件。

“Gain, Automatic”与 Gain 控件可用于测试易失（volatile）控件：如果设置了“Gain, Automatic”，则 Gain 控件是易失的，会不断变化；如果清除了“Gain, Automatic”，则 Gain 控件是普通控件。

“Horizontal Flip”与“Vertical Flip”控件可用于翻转图像。它们与“Sensor Flipped Horizontally/Vertically”Vivid 控件组合使用。

“Alpha Component”控件可用于设置包含 alpha 通道的格式的 alpha 分量。

#### 用户控件 - 音频


以下控件特定于视频采集与输出，以及无线电接收与发射。

“Volume”与“Mute”音频控件是此类设备用于控制音量与静音的典型控件。它们在 vivid 驱动中实际上不做任何事情。

#### Vivid 控件


这些 vivid 自定义控件用于控制图像生成、错误注入等。

##### 测试图案控件


测试图案控件全部特定于视频采集。

- Test Pattern:

	选择要使用的测试图案。可使用 CSC Colorbar 来测试色彩空间转换：该测试图案中使用的颜色可映射到所有色彩空间中的有效颜色。对于其他测试图案，色彩空间转换被禁用。

- OSD Text Mode:

	选择是否显示叠加在测试图案上的文字；若显示，是只显示计数器还是显示完整文字。

- Horizontal Movement:

	选择测试图案是否应向左或向右移动，以及以何种速度移动。

- Vertical Movement:

	对垂直方向执行相同功能。

- Show Border:

	在真实图像边缘显示两像素宽的边框（不含上下/左右加边区域）。

- Show Square:

	在图像中央显示一个正方形。如果图像以正确的像素与图像宽高比校正显示，则显示器上该正方形的宽高应相同。

- Insert SAV Code in Image:

	向图像添加 SAV（Start of Active Video，有效视频开始）码。这可用于检查图像中的此类代码是否会被无意地解释而非被忽略。

- Insert EAV Code in Image:

	对 EAV（End of Active Video，有效视频结束）码执行相同功能。

- Insert Video Guard Band
在图像左侧添加 4 列带有 HDMI Video Guard Band 码的像素。这只对 3 或 4 字节的 RGB 像素格式有效。RGB 像素值 0xab/0x55/0xab 恰好等同于每条有效视频行之前的 HDMI Video Guard Band 码（见 HDMI 1.3 规范第 5.2.2.1 节）。若要测试视频接收器是否具有正确的 HDMI Video Guard Band 处理能力，可启用此控件，然后将图像移到屏幕左侧。这将导致视频行以多个与前置 Video Guard Band 值相同的像素开始。那些只是不断跳过 Video Guard Band 值的接收器现在会失败，要么失去同步，要么这些视频行会发生偏移。

##### 采集特性选择控件


这些控件全部特定于视频采集。

- Sensor Flipped Horizontally:

	图像被水平翻转，并设置 V4L2_IN_ST_HFLIP 输入状态标志。这模拟了例如传感器倒置安装的情况。

- Sensor Flipped Vertically:

	图像被垂直翻转，并设置 V4L2_IN_ST_VFLIP 输入状态标志。这模拟了例如传感器倒置安装的情况。

- Standard Aspect Ratio:

	选择用于 TV 或 S-Video 输入的画面宽高比应为 4x3、16x9 还是变形宽屏。这可能会引入上下加边。

- DV Timings Aspect Ratio:

	选择用于 HDMI 输入的画面宽高比应与其源宽高比相同，还是应为 4x3 或 16x9。这可能会引入上下或左右加边。

- Timestamp Source:

	选择每个缓冲区的时间戳取自何时。

- Colorspace:

	选择生成图像时应使用的色彩空间。这仅在选择 CSC Colorbar 测试图案时生效，否则测试图案会原样通过而不转换。这也是你所期望的行为，因为 75% 的彩条确实应具有 75% 的信号强度，并且不应受色彩空间转换影响。

	改变色彩空间会触发 V4L2_EVENT_SOURCE_CHANGE 事件，因为它模拟了检测到的色彩空间变化。

- Transfer Function:

	选择生成图像时应使用的色彩空间传递函数。这仅在选择 CSC Colorbar 测试图案时生效，否则测试图案会原样通过而不转换。这也是你所期望的行为，因为 75% 的彩条确实应具有 75% 的信号强度，并且不应受色彩空间转换影响。

	改变传递函数会触发 V4L2_EVENT_SOURCE_CHANGE 事件，因为它模拟了检测到的色彩空间变化。

- Y'CbCr Encoding:

	选择生成 Y'CbCr 图像时应使用的 Y'CbCr 编码。这仅在选择 Y'CbCr 格式（而非 RGB 格式）时生效。

	改变 Y'CbCr 编码会触发 V4L2_EVENT_SOURCE_CHANGE 事件，因为它模拟了检测到的色彩空间变化。

- Quantization:

	选择生成测试图案时用于 RGB 或 Y'CbCr 编码的量化方式。

	改变量化会触发 V4L2_EVENT_SOURCE_CHANGE 事件，因为它模拟了检测到的色彩空间变化。

- Limited RGB Range (16-235):
	选择 HDMI 源的 RGB 范围应为受限范围还是全范围。它与 Digital Video 的“Rx RGB Quantization Range”控件组合使用，可用于测试当源提供错误的量化范围信息时会发生的情形。详见该控件的描述。

- Apply Alpha To Red Only:

	将“Alpha Component”用户控件设置的 alpha 通道仅应用到测试图案的红色上。

- Enable Capture Cropping:

	启用裁剪支持。该控件仅在 ccs_cap_mode 模块选项设为默认值 -1，且 no_error_inj 模块选项设为 0（默认）时存在。

- Enable Capture Composing:

	启用合成支持。该控件仅在 ccs_cap_mode 模块选项设为默认值 -1，且 no_error_inj 模块选项设为 0（默认）时存在。

- Enable Capture Scaler:

	启用缩放器支持（最多 4 倍放大与缩小）。该控件仅在 ccs_cap_mode 模块选项设为默认值 -1，且 no_error_inj 模块选项设为 0（默认）时存在。

- Maximum EDID Blocks:

	决定驱动支持的 EDID 块数量。注意 vivid 驱动实际上不会解析新的 EDID 数据，只是存储它。它最多支持 256 个 EDID 块，这也是标准所支持的最大值。

- Fill Percentage of Frame:

	可用于只绘制图像顶部的 X%。由于每一帧都必须由驱动绘制，这会占用大量 CPU。对于高分辨率，这会成为问题。通过只绘制图像的一部分，可以降低这一 CPU 负载。

##### 输出特性选择控件


这些控件全部特定于视频输出。

- Enable Output Cropping:

	启用裁剪支持。该控件仅在 ccs_out_mode 模块选项设为默认值 -1，且 no_error_inj 模块选项设为 0（默认）时存在。

- Enable Output Composing:

	启用合成支持。该控件仅在 ccs_out_mode 模块选项设为默认值 -1，且 no_error_inj 模块选项设为 0（默认）时存在。

- Enable Output Scaler:

	启用缩放器支持（最多 4 倍放大与缩小）。该控件仅在 ccs_out_mode 模块选项设为默认值 -1，且 no_error_inj 模块选项设为 0（默认）时存在。

##### 错误注入控件


以下两个控件仅对视频与 VBI 采集有效。

- Standard Signal Mode:

	选择 VIDIOC_QUERYSTD 的行为：它应返回什么？

	改变此控件会触发 V4L2_EVENT_SOURCE_CHANGE 事件，因为它模拟了输入条件的变化（例如插拔了线缆）。

- Standard:

	选择当上一控件设为“Selected Standard”时，VIDIOC_QUERYSTD 应返回的标准。

	改变此控件会触发 V4L2_EVENT_SOURCE_CHANGE 事件，因为它模拟了输入标准的变化。

以下两个控件仅对视频采集有效。

- DV Timings Signal Mode:

	选择 VIDIOC_QUERY_DV_TIMINGS 的行为：它应返回什么？

	改变此控件会触发 V4L2_EVENT_SOURCE_CHANGE 事件，因为它模拟了输入条件的变化（例如插拔了线缆）。

- DV Timings:
	选择当上一控件设为“Selected DV Timings”时，VIDIOC_QUERY_DV_TIMINGS 应返回的时序。

	改变此控件会触发 V4L2_EVENT_SOURCE_CHANGE 事件，因为它模拟了输入时序的变化。

以下控件仅在 no_error_inj 模块选项设为 0（默认）时存在。这些控件对视频与 VBI 采集和输出流，以及 SDR 采集设备有效，但 Disconnect 控件对所有设备均有效。

- Wrap Sequence Number:

	测试当 struct v4l2_buffer 中的序列号回绕时会发生什么。

- Wrap Timestamp:

	测试当 struct v4l2_buffer 中的时间戳回绕时会发生什么。

- Percentage of Dropped Buffers:

	设置永远不被驱动返回（即被丢弃）的缓冲区百分比。

- Disconnect:

	模拟一次 USB 断开。设备将表现得像是已被断开。只有在关闭对该设备节点的所有打开的文件句柄后，设备才会重新“连接”。

- Inject V4L2_BUF_FLAG_ERROR:

	按下时，驱动返回的下一帧将设置错误标志（即该帧被标记为损坏）。

- Inject VIDIOC_REQBUFS Error:

	按下时，下一次 REQBUFS 或 CREATE_BUFS ioctl 调用将以错误失败。准确地说：videobuf2 的 queue_setup() 操作将返回 -EINVAL。

- Inject VIDIOC_QBUF Error:

	按下时，下一次 VIDIOC_QBUF 或 VIDIOC_PREPARE_BUFFER ioctl 调用将以错误失败。准确地说：videobuf2 的 buf_prepare() 操作将返回 -EINVAL。

- Inject VIDIOC_STREAMON Error:

	按下时，下一次 VIDIOC_STREAMON ioctl 调用将以错误失败。准确地说：videobuf2 的 start_streaming() 操作将返回 -EINVAL。

- Inject Fatal Streaming Error:

	按下时，流式核心将被标记为发生了致命错误，恢复的唯一方法是停止流式传输。准确地说：会调用 videobuf2 的 vb2_queue_error() 函数。

##### VBI Raw 采集控件


- Interlaced VBI Format:

	若设置，则 raw VBI 数据将被隔行化，而不是按场分组提供。

#### 数字视频控件


- Rx RGB Quantization Range:

	设置 HDMI 输入的 RGB 量化检测结果。它与 Vivid 的“Limited RGB Range (16-235)”控件组合使用，可用于测试当源提供错误的量化范围信息时会发生的情形。可通过选择 HDMI 输入、将此控件设为 Full 或 Limited 范围，并在“Limited RGB Range (16-235)”控件中选择相反的值来进行测试。如果选择了“Gray Ramp”测试图案，效果很容易看出。

- Tx RGB Quantization Range:

	设置 HDMI 输出的 RGB 量化检测结果。目前在 vivid 中它没有实际用途，但大多数 HDMI 发射器通常都会有此控件。

- Transmit Mode:

	将 HDMI 输出的发送模式设为 HDMI 或 DVI-D。这会影响报告的色彩空间，因为 DVI_D 输出始终使用 sRGB。

#### FM 无线电接收器控件


- RDS Reception:

	设置是否启用 RDS 接收器。

- RDS Program Type:


- RDS PS Name:


- RDS Radio Text:


- RDS Traffic Announcement:


- RDS Traffic Program:


- RDS Music:

	这些都是只读控件。如果 RDS Rx I/O Mode 设为“Block I/O”，则它们也处于非活动状态。如果 RDS Rx I/O Mode 设为“Controls”，则这些控件报告接收到的 RDS 数据。
	vivid 对此的实现相当基础：它们只在你设置新频率或获取调谐器状态（VIDIOC_G_TUNER）时更新。

- Radio HW Seek Mode:

	可以是“Bounded”、“Wrap Around”或“Both”。这决定了 VIDIOC_S_HW_FREQ_SEEK 是受频率范围限制、回绕，还是可由用户选择。

- Radio Programmable HW Seek:

	若设置，则用户可以提供 HW Seek 的下界与上界。否则将使用频率范围的边界。

- Generate RBDS Instead of RDS:

	若设置，则生成 RBDS（RDS 的美式变体）数据，而非 RDS（欧式 RDS）。这仅影响 PICODE 与 PTY 码。

- RDS Rx I/O Mode:

	可以是“Block I/O”，即 RDS 块必须由应用程序 read()；也可以是“Controls”，即 RDS 数据由上述 RDS 控件提供。

#### FM 无线电调制器控件


- RDS Program ID:


- RDS Program Type:


- RDS PS Name:


- RDS Radio Text:


- RDS Stereo:


- RDS Artificial Head:


- RDS Compressed:


- RDS Dynamic PTY:


- RDS Traffic Announcement:


- RDS Traffic Program:


- RDS Music:

	这些都是用于设置 FM 调制器所发送 RDS 数据的控件。

- RDS Tx I/O Mode:

	可以是“Block I/O”，即应用程序必须使用 write() 将 RDS 块传递给驱动；也可以是“Controls”，即 RDS 数据由上述 RDS 控件提供。

#### 元数据采集控件


- Generate PTS

        若设置，则生成的元数据流包含呈现时间戳（Presentation timestamp）。

- Generate SCR

        若设置，则生成的元数据流包含源时钟（Source Clock）信息。

### 视频、Sliced VBI 与 HDMI CEC 环回


视频环回功能在由同一 vivid 驱动实例创建的设备之间，以及跨多个 vivid 驱动实例之间都受支持。vivid 驱动支持在 S-Video 输出与 S-Video 输入之间环回视频与 Sliced VBI 数据，也支持在 HDMI 输出与 HDMI 输入之间环回视频与 HDMI CEC 数据。

要启用环回，请设置“HDMI/S-Video XXX-N Is Connected To”控件，以选择输入是使用测试图案生成器、断开连接，还是连接到某个输出。一个输入可以连接到任意 vivid 实例的输出。输入与输出编号格式为 XXX-N，其中 XXX 是 vivid 实例编号（见模块选项 n_devs）。如果只有一个 vivid 实例（默认），则 XXX 为 000；N 是该实例的第 N 个 S-Video/HDMI 输入或输出。如果 vivid 不带模块选项加载，则你可以将 S-Video 000-0 输入连接到 S-Video 000-0 输出，或将 HDMI 000-0 输入连接到 HDMI 000-0 输出。这等同于在物理设备的输入与输出之间连接或断开一条线缆。

如果“HDMI/S-Video XXX-N Is Connected To”控件选择了一个输出，则视频输出会被环回至视频输入，前提是：

- 当前选择的输入与控件名称所指的输入相匹配。

- 在输出连接器的 vivid 实例中，当前选择的输出与控件值所指的输出相匹配。

- 视频输入的视频分辨率必须与视频输出的分辨率相匹配。因此，无法将 50 Hz（720x576）的 S-Video 输出环回到 60 Hz（720x480）的 S-Video 输入，也无法将 720p60 的 HDMI 输出环回到 1080p30 的输入。
- 两侧的像素格式必须完全相同。否则驱动还需要做像素格式转换，那就太过复杂了。

- 两侧的 field 设置必须完全相同。原因同上：要求驱动在一种 field 格式与另一种之间转换会使事情过于复杂。这也禁止在输出视频设为“Field Alternate”时使用“Field Top”或“Field Bottom”采集。这种组合虽然合法，但支持起来过于复杂。要让它工作，两侧都必须是“Field Alternate”。另请注意，在这种特定情况下，采集侧 struct v4l2_buffer 中的序列与场计数可能并非 100% 准确。

- 不支持 field 设置 V4L2_FIELD_SEQ_TB/BT。虽然实现它是可能的，但要做对需要大量工作。由于这些 field 值很少使用，目前决定不实现。

- 在输入侧，应配置 S-Video 输入的“Standard Signal Mode”或 HDMI 输入的“DV Timings Signal Mode”，以便向视频输入传递有效信号。

如果任何条件不成立，则显示“Noise”测试图案。

帧率不必匹配，尽管这一点未来可能会改变。

默认情况下，你会看到叠加在环回视频之上的 OSD 文字。可以通过更改视频采集设备的“OSD Text Mode”控件来关闭它。

要使 VBI 环回工作，上述所有条件都必须成立，此外 VBI 输出必须配置为 sliced VBI。VBI 采集侧可配置为 raw 或 sliced VBI。请注意，目前只环回 CC/XDS（60 Hz 格式）与 WSS（50 Hz 格式）的 VBI 数据，图文电视（Teletext）VBI 数据不被环回。

### 无线电与 RDS 环回


vivid 驱动支持将 RDS 输出环回到 RDS 输入。

由于无线电是无线传输，只要无线电接收频率接近无线电发射频率，这种环回就总会发生。此时无线电发射器会“覆盖”被模拟的广播电台。

RDS 环回目前仅支持由同一 vivid 驱动实例创建的设备之间。

如“无线电接收器”章节所述，无线电接收器在规则的频间隔处模拟电台。根据无线电接收器的频率会计算一个信号强度值（由 VIDIOC_G_TUNER 返回）。不过，它也会查看无线电发射器设置的频率，如果该频率产生的信号强度更高，则会使用无线电发射器的设置，就像它是一个有效电台一样。这也包括发射器“发射”的 RDS 数据（如果有）。这些数据在接收侧被如实接收。请注意，驱动加载时无线电接收器与发射器的频率并不相同，因此初始时不会发生环回。

### 裁剪、合成、缩放


该驱动支持裁剪、合成与缩放的任意组合。通常哪些特性受支持可通过 Vivid 控件选择，也可以在模块加载时通过 ccs_cap_mode 与 ccs_out_mode 模块选项硬编码。关于这些模块选项的详情，见“配置驱动”。

这使你可以针对所有这些变体测试你的应用程序。
注意，webcam 输入从不支持裁剪、合成或缩放。这仅适用于 TV/S-Video/HDMI 的输入与输出。原因是 webcam（包括这个虚拟实现）通常使用 VIDIOC_ENUM_FRAMESIZES 列出一组它支持的离散帧尺寸，而这与裁剪、合成或缩放不兼容。这主要是 V4L2 API 的一个限制，此处被刻意复现。

缩放器所能达到的最小与最大分辨率分别为 16x16 和 (4096 * 4) x (2160 x 4)，但它只能以 4 倍或更小的倍数进行放大或缩小。因此，对于 1280x720 的源分辨率，缩放器最小能做到 320x180，最大为 5120x2880。你可以用 qv4l2 测试工具试验这一点，就能看到这些依赖关系。

该驱动也支持更大的“bytesperline”设置，这是 VIDIOC_S_FMT 允许但很少驱动实现的特性。

缩放器是一个简单的缩放器，使用粗略的 Bresenham 算法。它专为速度与简单性而设计，而非质量。

如果裁剪、合成与缩放的组合允许，则可以实时改变裁剪与合成矩形。

### 格式


该驱动支持所有常规的交错（packed）与平面（planar）4:4:4、4:2:2 与 4:2:0 YUYV 格式、8/16/24/32 位 RGB 交错格式，以及各种多平面格式。

对于那些支持的格式，alpha 分量可通过“Alpha Component”用户控件设置。如果设置了“Apply Alpha To Red Only”控件，则 alpha 分量仅用于红色，其他情况设为 0。

驱动必须配置为支持多平面格式。默认情况下设备实例为单平面。这可通过设置 multiplanar 模块选项来改变，详见“配置驱动”了解该选项的更多信息。

如果设备实例使用多平面格式/API，则第一个单平面格式（YUYV）以及多平面 NV16M 与 NV61M 格式会拥有一个 data_offset 为 128 字节（非零）的平面。data_offset 非零的情况很罕见，因此这对测试应用程序很有用。

视频输出也会遵循应用程序设置的任何 data_offset。

### 输出叠加


注意：实现输出叠加主要是为了测试现有的 V4L2 输出叠加 API。新驱动是否应该使用此 API 是存疑的。

该驱动支持输出叠加，并能够实现：

 - 位图裁剪（bitmap clipping）
 - 列表裁剪（list clipping，最多 16 个矩形）
 - 色度键控（chromakey）
 - 源色度键控（source chromakey）
 - 全局 alpha（global alpha）
 - 局部 alpha（local alpha）
 - 局部反相 alpha（local inverse alpha）

输出叠加不支持多平面格式。此外，叠加要生效，采集格式与帧缓冲的 pixelformat 必须相同，否则 VIDIOC_OVERLAY 会返回错误。

输出叠加仅在驱动通过将 node_types 模块选项中的标志 0x10000 设置为创建帧缓冲时才有效。创建的帧缓冲大小为 720x576，支持 ARGB 1:5:5:5 与 RGB 5:6:5。

要查看各种裁剪、色度键控或 alpha 处理的效果，需要开启视频环回，并在采集侧查看结果。使用裁剪、色度键控或 alpha 处理能力会显著拖慢视频环回，因为需要对每个像素做大量检查。

### CEC（消费电子控制，Consumer Electronics Control）

如果存在 HDMI 输入，则会创建一个具有相同数量输入端口的 CEC 适配器。这等同于例如一台具有该数量输入的电视。每个 HDMI 输出也会创建一个连接到对应输入端口的 CEC 适配器，或者（如果输出多于输入）完全不连接。换句话说，这等同于将每个输出设备连接到电视的一个输入端口。任何剩余的输出现备保持未连接。

每个输出读取的 EDID 会报告一个唯一的 CEC 物理地址，该地址基于输入 EDID 的物理地址。因此，如果接收器的 EDID 物理地址为 A.B.0.0，则每个输出会看到包含物理地址 A.B.C.0 的 EDID，其中 C 从 1 到输入数量。如果输出多于输入，则剩余的输出拥有被禁用并报告无效物理地址的 CEC 适配器。

### 一些未来的改进


仅作提醒，不分先后顺序：

- 添加一个虚拟 alsa 驱动以测试音频
- 添加虚拟子设备
- 一些用于测试压缩视频的支持
- 添加将 raw VBI 输出环回到 raw VBI 输入的支持
- 添加将图文电视 sliced VBI 输出环回到 VBI 输入的支持
- 修复环回带交替场的视频时的序列/场编号
- 为视频输出添加 V4L2_CID_BG_COLOR 支持
- 添加 ARGB888 叠加支持：更好地测试 alpha 通道
- 通过传递真实的 v4l2_fract 来改进 tpg 代码中的像素宽高比支持
- 使用每队列锁和/或每设备锁来提高吞吐量
- SDR 无线电应与普通无线电接收器对电台使用相同的“频率”，并在频率与电台频率不匹配时返回噪声
- 为 RDS 生成创建一个线程，这对“Controls”RDS Rx I/O Mode 尤其有帮助，因为只读的 RDS 控件可以实时更新
- 更改 EDID 时不必等待 100 ms 再设置 HPD 信号