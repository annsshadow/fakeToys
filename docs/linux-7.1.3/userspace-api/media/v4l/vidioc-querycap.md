


######## ioctl VIDIOC_QUERYCAP


## Name


VIDIOC_QUERYCAP - 查询设备能力


## Synopsis


`int ioctl(int fd, VIDIOC_QUERYCAP, struct v4l2_capability *argp)`


## Arguments


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_capability` 的指针
## Description


所V4L2 设备都支`VIDIOC_QUERYCAP` ioctl。它用于识别与本规范兼容的内设备，并获取有关驱动程序和硬件能力的信息。该 ioctl 接受一个指struct
`v4l2_capability` 的指针，该结构由驱动程序填充。当驱动程序与本规范不兼容时ioctl 返回 `EINVAL` 错误码


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 4 20

    - - __u8
      - `driver`\ [^16^]
      - 驱动程序的名称，一个唯一的、以 NUL 结尾ASCII 字符串。例如：
	"bttv"。特定于驱动程序的应用程序可以使用此信息来验证驱动程序的
	身份。它也有助于规避已知的缺陷，或在错误报告中识别驱动程序
	在固定大小的数组中存储字符串是一种不好的做法，但在这里不可避免	驱动程序和应用程序应当采取措施，绝不去读取或写入数组末尾之外，并
	确保字符串被正确 NUL 结尾    - - __u8
      - `card`\ [^32^]
      - 设备的名称，一个以 NUL 结尾UTF-8 字符串。例如："Yoyodyne TV/FM"	一个驱动程序可能支持不同品牌或型号的硬件。此信息面向用户，例	显示在可用设备的菜单中。由于可能安装了多个同一品牌的电视卡，且
	它们由同一个驱动程序支持，此名称应与字符设备文件名（例	`/dev/video2`）或 `bus_info` 字符串结合使用，以避免歧义    - - __u8
      - `bus_info`\ [^32^]
      - 设备在系统中的位置，一个以 NUL 结尾ASCII 字符串。例如：
	"PCI:0000:05:06.0"。此信息面向用户，用于区分多个相同的设备。如	没有此类信息可用，该字段必须简单地对由驱动程序控制的设备进行计	platform:vivid-000"）。对PCI 板卡，bus_info 必须"PCI:" 开头；
	对于 PCI Express 板卡"PCIe:" 开头；对于 USB 设备"usb-" 开头；
	对于 i2c 设备"I2C:" 开头；对于 ISA 设备"ISA:" 开头；对于并口
	设备"parport" 开头；对于平台设备"platform:" 开头    - - __u32
      - `version`
      - 驱动程序的版本号
	从内3.1 开始，报告的版本号V4L2 子系统按照内核编号方案提供	但是，如果例如一个稳定版或发行版修改过的内核使用了来自更新内核的
	V4L2 栈，它可能并不总是返回与内核相同的版本
	版本号使`KERNEL_VERSION()` 宏来格式化。例如，如果媒体栈对应于
	随内4.14 一起发布的 V4L2 版本，则它等价于    - - `2`

	`#define KERNEL_VERSION(a,b,c) (((a) << 16) + ((b) << 8) + (c))`

	`__u32 version = KERNEL_VERSION(4, 14, 0);`

	`printf ("Version: %u.%u.%u\\n",`

	`(version >> 16) & 0xFF, (version >> 8) & 0xFF, version & 0xFF);`
    - - __u32
      - `capabilities`
      - 整个物理设备可用的能力，device-capabilities。同一个物理设备可以在
	/dev 中导出多个设备（例如 /dev/videoXdev/vbiY /dev/radioZ）	`capabilities` 字段应包含导出到用户空间的所V4L2 设备周围可用能力	并集。对于所有这些设备，`capabilities` 字段返回相同的一组能力。这
	允许应用程序只打开其中一个设备（通常是视频设备），并发现是否也支	视频、vbi 或广播    - - __u32
      - `device_caps`
      - 所打开设备的能力，device-capabilities。应当包含该特定设备节点	可用能力。因此，例如，一个广播设备的 `device_caps` 将只包含与广	相关的能力，而不包含任何视频vbi 能力。仅`capabilities` 字段包含
	`V4L2_CAP_DEVICE_CAPS` 能力时，才会设置此字段。只`capabilities`
	字段可以`V4L2_CAP_DEVICE_CAPS` 能力，`device_caps` 永远不会设置
	`V4L2_CAP_DEVICE_CAPS`銆?    - - __u32
      - `reserved`\ [^3^]
      - 为将来的扩展保留。驱动程序必须将此数组置零



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_CAP_VIDEO_CAPTURE`
      - 0x00000001
      - 该设备通过 Video Capture <capture> 接口支持单平API    - - `V4L2_CAP_VIDEO_CAPTURE_MPLANE`
      - 0x00001000
      - 该设备通过 Video Capture <capture> 接口支持多平API <planar-apis>    - - `V4L2_CAP_VIDEO_OUTPUT`
      - 0x00000002
      - 该设备通过 Video Output <output> 接口支持单平API    - - `V4L2_CAP_VIDEO_OUTPUT_MPLANE`
      - 0x00002000
      - 该设备通过 Video Output <output> 接口支持多平API <planar-apis>    - - `V4L2_CAP_VIDEO_M2M`
      - 0x00008000
      - 该设备通过 Video Memory-To-Memory 接口支持单平API    - - `V4L2_CAP_VIDEO_M2M_MPLANE`
      - 0x00004000
      - 该设备通过 Video Memory-To-Memory 接口支持多平API <planar-apis>    - - `V4L2_CAP_VIDEO_OVERLAY`
      - 0x00000004
      - 该设备支Video Overlay <overlay> 接口。视频叠加设备通常将捕获的
	图像直接存入显卡的视频内存中，并带有硬件裁剪和缩放    - - `V4L2_CAP_VBI_CAPTURE`
      - 0x00000010
      - 该设备支Raw VBI Capture <raw-vbi> 接口，提供图文电视和隐藏字幕
	数据    - - `V4L2_CAP_VBI_OUTPUT`
      - 0x00000020
      - 该设备支Raw VBI Output <raw-vbi> 接口    - - `V4L2_CAP_SLICED_VBI_CAPTURE`
      - 0x00000040
      - 该设备支Sliced VBI Capture <sliced> 接口    - - `V4L2_CAP_SLICED_VBI_OUTPUT`
      - 0x00000080
      - 该设备支Sliced VBI Output <sliced> 接口    - - `V4L2_CAP_RDS_CAPTURE`
      - 0x00000100
      - 该设备支RDS <rds> 捕获接口    - - `V4L2_CAP_VIDEO_OUTPUT_OVERLAY`
      - 0x00000200
      - 该设备支Video Output Overlay <osd>（OSD）接口。与 **Video Overlay**
	接口不同，这是视频输出设备的次要功能，将一幅图像叠加到传出的视	信号上。当驱动程序设置此标志时，它必须清除 `V4L2_CAP_VIDEO_OVERLAY`
	标志，反之亦然。[#f1]_
    - - `V4L2_CAP_HW_FREQ_SEEK`
      - 0x00000400
      - 该设备支持用于硬件频率搜索的 VIDIOC_S_HW_FREQ_SEEK ioctl    - - `V4L2_CAP_RDS_OUTPUT`
      - 0x00000800
      - 该设备支RDS <rds> 输出接口    - - `V4L2_CAP_TUNER`
      - 0x00010000
      - 该设备带有某种用于接收射频调制视频信号的调谐器。有关调谐器编程	更多信息，见 tuner    - - `V4L2_CAP_AUDIO`
      - 0x00020000
      - 该设备具有音频输入或输出。它可能支持也可能不支持PCM 或压缩格	进行音频录制或播放。PCM 音频支持必须实现ALSA OSS 接口。有	音频输入和输出的更多信息，见 audio    - - `V4L2_CAP_RADIO`
      - 0x00040000
      - 这是一个广播接收器    - - `V4L2_CAP_MODULATOR`
      - 0x00080000
      - 该设备带有某种用于发射射频调制视音频信号的调制器。有关调制器
	编程的更多信息，tuner    - - `V4L2_CAP_SDR_CAPTURE`
      - 0x00100000
      - 该设备支SDR Capture <sdr> 接口    - - `V4L2_CAP_EXT_PIX_FORMAT`
      - 0x00200000
      - 该设备支struct `v4l2_pix_format` 的扩展字段    - - `V4L2_CAP_SDR_OUTPUT`
      - 0x00400000
      - 该设备支SDR Output <sdr> 接口    - - `V4L2_CAP_META_CAPTURE`
      - 0x00800000
      - 该设备支持元数据捕获接口    - - `V4L2_CAP_READWRITE`
      - 0x01000000
      - 该设备支`read()` `write()` I/O 方法    - - `V4L2_CAP_EDID`
      - 0x02000000
      - 该设备为视频输入存储 EDID，或为视频输出检EDID。它是一个独立的
	EDID 设备，因此不会发生视频流传输等操作
        对于视频输入，这通常是一个支VESA 增强型显示数据通道标准
	<vesaeddc> eeprom。它也可以是别的东西，例如一个微控制器
        对于视频输出，这通常从外部设备读取，例如通过串口访问HDMI 分配器    - - `V4L2_CAP_STREAMING`
      - 0x04000000
      - 该设备支streaming <mmap> I/O 方法    - - `V4L2_CAP_META_OUTPUT`
      - 0x08000000
      - 该设备支持元数据输出接口    - - `V4L2_CAP_TOUCH`
      - 0x10000000
      - 这是一个触摸设备    - - `V4L2_CAP_IO_MC`
      - 0x20000000
      - 从用户空间看到的只有一个输入和/或输出。整个视频拓扑配置，包括哪个
	I/O 实体被路由到输入/输出，由用户空间通过媒体控制器配置。见
	media_controller銆?    - - `V4L2_CAP_DEVICE_CAPS`
      - 0x80000000
      - 驱动程序填充 `device_caps` 字段。此能力只能出现`capabilities`
	字段中，而绝不会出现`device_caps` 字段中
## Return Value


成功时返0，出错时返回 -1 并设`errno` 变量。通用错误码在 Generic Error
Codes <gen-errors> 一章中描述
   struct `v4l2_framebuffer` 缺少一enum `v4l2_buf_type` 字段，因此叠加的
   类型由驱动程序能力隐含