


######## Raw VBI 数据接口


VBI 是 Vertical Blanking Interval（场消隐间隔）的缩写，是模拟视频信号行序列中的一个间隙。在 VBI 期间不传输图像信息，为阴极射线管电视的电子束返回屏幕顶部留出一些时间。使用示波器，你会在这里发现垂直同步脉冲以及 ASK 调制 [#f1]_ 到视频信号上的短数据包。这些是 Teletext 或 Closed Caption 等服务的传输。

此接口类型的主题是原始 VBI 数据，即从视频信号上采样得到、或将被添加到信号中用于输出的数据。数据格式类似于未压缩的视频图像，即若干行乘以每行若干采样，我们称之为 VBI 图像。

按照惯例，V4L2 VBI 设备通过主设备号 81、次设备号 224 到 255 的字符设备特殊文件访问，命名为 `/dev/vbi` 以及 `/dev/vbi0` 到 `/dev/vbi31`。`/dev/vbi` 通常是指向首选 VBI 设备的符号链接。此约定同时适用于输入和输出设备。

为了解决寻找相关视频和 VBI 设备的问题，VBI 采集和输出也作为设备功能在 `/dev/video` 下提供。要使用这些设备采集或输出原始 VBI 数据，应用程序必须调用 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl。作为 `/dev/vbi` 访问时，原始 VBI 采集或输出是默认的设备功能。

## 查询能力


支持原始 VBI 采集或输出 API 的设备，分别在 VIDIOC_QUERYCAP ioctl 返回的 struct
`v4l2_capability` 的 `capabilities` 字段中设置 `V4L2_CAP_VBI_CAPTURE` 或 `V4L2_CAP_VBI_OUTPUT` 标志。必须至少支持 read/write 或流式 I/O 方法中的一种。VBI 设备可能有也可能没有调谐器或调制器。

## 辅助功能


VBI 设备应当根据需要支持 video 输入或输出 <video>、tuner 或 modulator <tuner> 以及 controls <control> ioctl。video standard <standard> ioctl 提供了对编程一个 VBI 设备至关重要的信息，因此必须支持。

## 原始 VBI 格式协商


原始 VBI 采样能力可能不同，尤其是采样频率。为了正确解释数据，V4L2 规定了一个 ioctl 来查询采样参数。此外，为了提供一定的灵活性，应用程序也可以建议不同的参数。

像往常一样，这些参数在 `open()` 时**不会**被重置，以允许 Unix 工具链将设备编程后像普通文件一样从中读取。编写良好的 V4L2 应用程序应当始终确保它们真正得到了想要的结果，即请求合理的参数，然后检查实际参数是否合适。

要查询当前的原始 VBI 采集参数，应用程序将 struct `v4l2_format` 的 `type` 字段设为 `V4L2_BUF_TYPE_VBI_CAPTURE` 或 `V4L2_BUF_TYPE_VBI_OUTPUT`，并以指向该结构的指针调用 VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl。驱动程序填充 `fmt` 联合体中的 struct
`v4l2_vbi_format` `vbi` 成员。

要请求不同的参数，应用程序像上面那样设置 struct `v4l2_format` 的 `type` 字段，并初始化 `fmt` 联合体中 struct
`v4l2_vbi_format` `vbi` 成员的所有字段，或者更好地只修改 VIDIOC_G_FMT <VIDIOC_G_FMT> 的结果，然后以指向该结构的指针调用 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl。只有当给定参数不明确时，驱动程序才返回 `EINVAL` 错误码，否则它们会根据硬件能力修改参数并返回实际参数。当驱动程序在此时分配资源时，它可能返回 `EBUSY` 错误码，表示返回的参数有效，但所需资源当前不可用。例如，当要采集的视频和 VBI 区域会重叠时，或者当驱动程序支持多次打开而另一个进程已经请求了 VBI 采集或输出时，就可能发生这种情况。无论如何，应用程序必须预料到其他可能返回 `EBUSY` 的资源分配点，例如在 VIDIOC_STREAMON ioctl 以及首次 `read()`、`write()` 和 `select()` 调用时。

VBI 设备必须实现 VIDIOC_G_FMT <VIDIOC_G_FMT> 和 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl，即使 VIDIOC_S_FMT <VIDIOC_G_FMT> 忽略所有请求并始终像 VIDIOC_G_FMT <VIDIOC_G_FMT> 那样返回默认参数。VIDIOC_TRY_FMT <VIDIOC_G_FMT> 是可选的。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `sampling_rate`
      - 每秒采样数，即单位 1 Hz。
    - - __u32
      - `offset`
      - VBI 图像的水平偏移，相对于行同步脉冲的前沿，以采样计数：VBI 图像中的第一个采样将位于前沿之后 `offset` /
	`sampling_rate` 秒处。另见 vbi-hsync。
    - - __u32
      - `samples_per_line`
      -
    - - __u32
      - `sample_format`
      - 定义像素格式中的采样格式，一个四字符码 [#f2]_。通常是 `V4L2_PIX_FMT_GREY`，
	即每个采样由 8 位组成，较低的值朝向黑电平。不要假设值与信号电平有任何其他关联。例如，MSB 不一定指示信号是 “高” 还是 “低”，因为 128 可能不是信号的平均值。驱动程序不应通过软件转换采样格式。
    - - __u32
      - `start`\ [#f2]_
      - 这是与 VBI 图像第一行相关联的扫描系统行号，分别是第一场和第二场。有效值见 vbi-525 和 vbi-625。`V4L2_VBI_ITU_525_F1_START`、
	`V4L2_VBI_ITU_525_F2_START`、`V4L2_VBI_ITU_625_F1_START` 和
	`V4L2_VBI_ITU_625_F2_START` 定义提供了作为便利的每种 525 或 625 行格式每场的起始行号。
	不要忘记 ITU 行编号从 1 开始，而不是 0。VBI 输入驱动可以在硬件无法可靠识别扫描行时返回 start 值 0，VBI 采集可能不需要此信息。
    - - __u32
      - `count`\ [#f2]_
      - 分别是第一场和第二场图像中的行数。
    - - `2`

	驱动程序应尽可能灵活。例如，可以将 VBI 采集窗口扩展或下移到图像区域，实现 “全场的模式” 来采集嵌入在图像中的数据服务传输。

	如果相应场不需要数据，应用程序可以将第一个或第二个 `count` 值设为零；如果扫描系统是逐行的（即非隔行），则设 `count`\ [^1^]。相应的 start 值应由应用程序和驱动程序忽略。无论如何，驱动程序可能不支持单场采集，而返回两个非零的 count 值。

	两个 `count` 值都为零，或者行号超出所描绘的边界 [#f4]_，或者覆盖两场行的场图像，都是无效的，驱动程序不应返回。

	要初始化 `start` 和 `count` 字段，应用程序必须首先确定当前的视频标准选择。为此可以评估 v4l2_std_id <v4l2-std-id> 或 struct
	`v4l2_standard` 的 `framelines` 字段。
    - - __u32
      - `flags`
      - 见下文 vbifmt-flags。目前只有驱动程序设置 flags，应用程序必须将此字段设为零。
    - - __u32
      - `reserved`\ [#f2]_
      - 此数组保留供将来扩展。驱动程序和应用程序必须将其设为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_VBI_UNSYNC`
      - 0x0001
      - 此标志表示硬件不能正确区分两场。通常 VBI 图像先在内存中存储第一场（较低的扫描行号）。根据视频标准的不同，这可能是顶场或底场。设置此标志时，第一场或第二场可能先存储，但两场在时间顺序上仍然正确，较旧的场先存入内存 [#f3]_。
    - - `V4L2_VBI_INTERLACED`
      - 0x0002
      - 默认情况下，两场图像将顺序传递；第一场的所有行之后跟着第二场的所有行（比较场顺序 `V4L2_FIELD_SEQ_TB` 和
	`V4L2_FIELD_SEQ_BT`，内存中顶场还是底场先存取决于视频标准）。设置此标志时，两场是隔行的（参见 `V4L2_FIELD_INTERLACED`）。第一场的第一行之后跟着第二场的第一行，然后是两个第二行，依此类推。当硬件被编程为采集或输出隔行视频图像，并且无法同时为 VBI 采集分离两场时，可能需要这样的布局。为简单起见，设置此标志意味着两个 `count` 值相等且非零。



    :alt:   vbi_hsync.svg
    :align: center

    行同步


    :alt:   vbi_525.svg
    :align: center

    ITU-R 525 行编号（M/NTSC 和 M/PAL）


    :alt:   vbi_625.svg
    :align: center

    ITU-R 625 行编号

请记住，VBI 图像格式取决于所选的视频标准，因此应用程序必须先选择新的标准或查询当前标准。在格式协商之前，或者在可能使协商好的 VBI 参数失效的视频标准切换之后，尝试读取或写入数据，应当被驱动程序拒绝。在活动的 I/O 期间不允许更改格式。

## 读取和写入 VBI 图像


为了保证与场号同步并便于实现，每次传递的最小数据单位是一帧，由内存中紧跟着的两场 VBI 图像组成。

一帧的总大小计算如下：


    (count[^0^] + count[^1^]) ** samples_per_line ** 字节中的采样大小

采样大小很可能始终是一字节，不过应用程序必须检查 `sample_format` 字段，以与其他驱动程序正确配合。

VBI 设备可能支持 read/write <rw> 和/或流式（memory mapping <mmap> 或 user pointer <userp>）I/O。后者具有通过使用缓冲区时间戳来同步视频和 VBI 数据的可能性。

请记住，VIDIOC_STREAMON <VIDIOC_STREAMON> ioctl 以及首次 `read()`、`write()` 和
`select()` 调用可能是资源分配点，当所需的硬件资源暂时不可用时（例如设备已被另一个进程使用）会返回 `EBUSY` 错误码。

   ASK：Amplitude-Shift Keying（幅移键控）。高信号电平表示 “1” 位，低电平表示 “0” 位。

   少数设备可能根本无法采样 VBI 数据，但可以将视频采集窗口扩展到 VBI 区域。

   大多数 VBI 服务在两场上都传输，但有些根据场号具有不同的语义。当设置了 `V4L2_VBI_UNSYNC` 时，它们无法被可靠地解码或编码。

   有效值见 vbi-525 和 vbi-625。
