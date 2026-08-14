

######## V4L 与 V4L2 的区别


Video For Linux（V4L）API 最早在 Linux 2.1 中引入，用于统一并取代早些年由驱动开发者各自开发的多种电视和收音机设备相关接口。从 Linux 2.5 开始，改进的 V4L2 API 取代了 V4L API。内核中已移除对旧 V4L 调用的支持，但 libv4l 库支持将 V4L API 系统调用转换为 V4L2 调用。

## 设备的打开与关闭


出于兼容性考虑，V4L2 建议用于视频捕获、叠加、收音机和原始 VBI 捕获设备的字符设备文件名，与 V4L 使用的文件名保持一致。它们列在 devices 以及下文 v4l-dev 中。

图文电视（teletext）设备（次设备号范围 192-223）在 V4L2 中已被移除，不再存在。目前已无处理纯图文电视的硬件，取而代之使用原始或切片 VBI。

V4L 的 `videodev` 模块会根据注册的设备类型，按加载顺序自动为驱动分配次设备号。我们建议 V4L2 驱动默认使用相同的编号注册设备，但系统管理员可以通过驱动模块选项分配任意次设备号。主设备号仍为 81。


    :header-rows:  1
    :stub-columns: 0

    - - 设备类型
      - 文件名
      - 次设备号
    - - 视频捕获与叠加
      - `/dev/video` and `/dev/bttv0`\  [#f1]_, `/dev/video0` to
	`/dev/video63`
      - 0-63
    - - 收音机接收器
      - `/dev/radio`\  [#f2]_, `/dev/radio0` to `/dev/radio63`
      - 64-127
    - - 原始 VBI 捕获
      - `/dev/vbi`, `/dev/vbi0` to `/dev/vbi31`
      - 224-255

V4L 禁止（或曾经禁止）多次打开同一设备文件。V4L2 驱动**可能**支持多次打开，详见 open 了解细节与后果。

V4L 驱动会以 `EINVAL` 错误码响应 V4L2 的 ioctl。

## 查询能力


V4L 的 `VIDIOCGCAP` ioctl 等价于 V4L2 的 VIDIOC_QUERYCAP。

struct `video_capability` 中的 `name` 字段在 struct `v4l2_capability` 中变为 `card`，`type` 被 `capabilities` 取代。注意 V4L2 并不会如此区分设备类型，更准确地说，应将其视为支持一组相关功能（如视频捕获、视频叠加和 VBI 捕获）的基本视频输入、视频输出和收音机设备。参见 open 了解介绍。


   \small



    :header-rows:  1
    :stub-columns: 0

    - - struct `video_capability` `type`
      - struct `v4l2_capability`
	`capabilities` flags
      - 用途
    - - `VID_TYPE_CAPTURE`
      - `V4L2_CAP_VIDEO_CAPTURE`
      - 支持视频捕获（video capture）接口。
    - - `VID_TYPE_TUNER`
      - `V4L2_CAP_TUNER`
      - 设备带有调谐器或调制器（tuner/modulator）。
    - - `VID_TYPE_TELETEXT`
      - `V4L2_CAP_VBI_CAPTURE`
      - 支持原始 VBI 捕获（raw VBI）接口。
    - - `VID_TYPE_OVERLAY`
      - `V4L2_CAP_VIDEO_OVERLAY`
      - 支持视频叠加（video overlay）接口。
    - - `VID_TYPE_CHROMAKEY`
      - `V4L2_FBUF_CAP_CHROMAKEY` in field `capability` of struct
	`v4l2_framebuffer`
      - 是否支持色键（chromakey）叠加。关于叠加的更多信息请参见 overlay。
    - - `VID_TYPE_CLIPPING`
      - `V4L2_FBUF_CAP_LIST_CLIPPING` and
	`V4L2_FBUF_CAP_BITMAP_CLIPPING` in field `capability` of
	struct `v4l2_framebuffer`
      - 是否支持对叠加图像进行裁剪（clipping），参见 overlay。
    - - `VID_TYPE_FRAMERAM`
      - `V4L2_FBUF_CAP_EXTERNOVERLAY` **not set** in field `capability`
	of struct `v4l2_framebuffer`
      - 叠加是否覆盖帧缓冲内存，参见 overlay。
    - - `VID_TYPE_SCALES`
      - `-`
      - 该标志表示硬件是否能够缩放图像。V4L2 API 通过分别使用 VIDIOC_S_CROP <VIDIOC_G_CROP> 和 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 设置裁剪尺寸和图像大小来隐含缩放系数。驱动会返回尽可能接近的尺寸。关于裁剪和缩放的更多信息请参见 crop。
    - - `VID_TYPE_MONOCHROME`
      - `-`
      - 应用程序可以通过 VIDIOC_ENUM_FMT ioctl 枚举支持的的图像格式，以确定设备是否仅支持灰度捕获。关于图像格式的更多信息请参见 pixfmt。
    - - `VID_TYPE_SUBCAPTURE`
      - `-`
      - 应用程序可以调用 VIDIOC_G_CROP <VIDIOC_G_CROP> ioctl 来确定设备是否支持捕获完整画面的一部分（即 V4L2 中的“cropping”）。如果不支持，该 ioctl 会返回 `EINVAL` 错误码。关于裁剪和缩放的更多信息请参见 crop。
    - - `VID_TYPE_MPEG_DECODER`
      - `-`
      - 应用程序可以通过 VIDIOC_ENUM_FMT ioctl 枚举支持的的图像格式，以确定设备是否支持 MPEG 流。
    - - `VID_TYPE_MPEG_ENCODER`
      - `-`
      - 参见上文。
    - - `VID_TYPE_MJPEG_DECODER`
      - `-`
      - 参见上文。
    - - `VID_TYPE_MJPEG_ENCODER`
      - `-`
      - 参见上文。


   \normalsize

`audios` 字段被 `capabilities` 标志 `V4L2_CAP_AUDIO` 取代，用于指示设备**是否**具有任何音频输入或输出。要确定其数量，应用程序可以使用 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> ioctl 枚举音频输入。音频相关的 ioctl 在 audio 中有说明。

`maxwidth`、`maxheight`、`minwidth` 和 `minheight` 字段已被移除。使用期望的尺寸调用 VIDIOC_S_FMT <VIDIOC_G_FMT> 或 VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 时，会综合考虑当前视频标准、裁剪和缩放限制，返回尽可能接近的尺寸。

## 视频源


V4L 使用 struct `video_channel` 提供 `VIDIOCGCHAN` 和 `VIDIOCSCHAN` ioctl，用于枚举 V4L 设备的视频输入。等价的 V4L2 ioctl 是 VIDIOC_ENUMINPUT、VIDIOC_G_INPUT <VIDIOC_G_INPUT> 和 VIDIOC_S_INPUT <VIDIOC_G_INPUT>，它们使用 struct `v4l2_input`，正如 video 中所述。

用于计数输入的 `channel` 字段被重命名为 `index`，视频输入类型的重命名如下：


    :header-rows:  1
    :stub-columns: 0

    - - struct `video_channel` `type`
      - struct `v4l2_input` `type`
    - - `VIDEO_TYPE_TV`
      - `V4L2_INPUT_TYPE_TUNER`
    - - `VIDEO_TYPE_CAMERA`
      - `V4L2_INPUT_TYPE_CAMERA`

与表示此输入调谐器数量的 `tuners` 字段不同，V4L2 假设每个视频输入最多连接一个调谐器。但一个调谐器可以有多个输入（即 RF 连接器），且一个设备可以有多个调谐器。与该输入关联的调谐器（如有）的索引号存储在 struct `v4l2_input` 的 `tuner` 字段中。调谐器的枚举在 tuner 中讨论。

冗余的 `VIDEO_VC_TUNER` 标志被移除。与调谐器关联的视频输入类型为 `V4L2_INPUT_TYPE_TUNER`。`VIDEO_VC_AUDIO` 标志被 `audioset` 字段取代。V4L2 支持最多 32 个音频输入的设备。`audioset` 字段中每个被置位的位代表该视频输入所组合的一个音频输入。关于音频输入及其切换方式的信息请参见 audio。

描述所支持视频标准的 `norm` 字段被 `std` 取代。V4L 规范提到过 `VIDEO_VC_NORM` 标志，用于表示标准是否可更改。该标志与 `norm` 字段是后来一起加入的，现已被移除。V4L2 对视频标准采用了类似但更全面的方案，详见 standard。

## 调谐


V4L 的 `VIDIOCGTUNER` 和 `VIDIOCSTUNER` ioctl 以及 struct `video_tuner` 可用于枚举 V4L 电视或收音机设备的调谐器。等价的 V4L2 ioctl 是 VIDIOC_G_TUNER <VIDIOC_G_TUNER> 和 VIDIOC_S_TUNER <VIDIOC_G_TUNER>，使用 struct `v4l2_tuner`。调谐器相关说明见 tuner。

用于计数调谐器的 `tuner` 字段被重命名为 `index`。`name`、`rangelow` 和 `rangehigh` 字段保持不变。

表示所支持视频标准的 `VIDEO_TUNER_PAL`、`VIDEO_TUNER_NTSC` 和 `VIDEO_TUNER_SECAM` 标志已被移除。该信息现在包含在关联的 struct `v4l2_input` 中。对于表示视频标准是否可切换的 `VIDEO_TUNER_NORM` 标志，目前没有替代项。用于选择不同视频标准的 `mode` 字段被一整套新的 ioctl 和结构取代，详见 standard。值得一提的是，由于 BTTV 驱动应用广泛，除常规的 `VIDEO_MODE_PAL` (0)、`VIDEO_MODE_NTSC`、`VIDEO_MODE_SECAM` 和 `VIDEO_MODE_AUTO` (3) 外，它还支持 N/PAL Argentina、M/PAL、N/PAL 和 NTSC Japan（编号为 3-6，原文如此）。

表示立体声接收的 `VIDEO_TUNER_STEREO_ON` 标志在 `rxsubchans` 字段中变为 `V4L2_TUNER_SUB_STEREO`。该字段还允许检测单声道和双语音频，详见 struct `v4l2_tuner` 的定义。目前对于 `VIDEO_TUNER_RDS_ON` 和 `VIDEO_TUNER_MBS_ON` 标志尚无替代项。

`VIDEO_TUNER_LOW` 标志在 struct `v4l2_tuner` 的 `capability` 字段中被重命名为 `V4L2_TUNER_CAP_LOW`。

用于更改调谐器频率的 `VIDIOCGFREQ` 和 `VIDIOCSFREQ` ioctl 被重命名为 VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> 和 VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY>。它们接受指向 struct `v4l2_frequency` 的指针，而非 unsigned long 整数。


## 图像属性


V4L2 没有与 `VIDIOCGPICT` 和 `VIDIOCSPICT` ioctl 以及 struct `video_picture` 等价的内容。以下字段被可通过 VIDIOC_QUERYCTRL、VIDIOC_G_CTRL <VIDIOC_G_CTRL> 和 VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl 访问的 V4L2 控件取代：


    :header-rows:  1
    :stub-columns: 0

    - - struct `video_picture`
      - V4L2 Control ID
    - - `brightness`
      - `V4L2_CID_BRIGHTNESS`
    - - `hue`
      - `V4L2_CID_HUE`
    - - `colour`
      - `V4L2_CID_SATURATION`
    - - `contrast`
      - `V4L2_CID_CONTRAST`
    - - `whiteness`
      - `V4L2_CID_WHITENESS`

V4L 的图像控件假定取值范围为 0 到 65535，没有特定的复位值。V4L2 API 允许任意的限制和默认值，可通过 VIDIOC_QUERYCTRL ioctl 查询。关于控件的一般信息请参见 control。

视频图像的 `depth`（每像素平均位数）由所选图像格式隐含。V4L2 不显式提供此类信息，它假设能识别该格式的应用程序了解图像深度，而其他应用程序则无需知道。`palette` 字段移入了 struct `v4l2_pix_format`：


    :header-rows:  1
    :stub-columns: 0

    - - struct `video_picture` `palette`
      - struct `v4l2_pix_format` `pixfmt`
    - - `VIDEO_PALETTE_GREY`
      - V4L2_PIX_FMT_GREY <V4L2-PIX-FMT-GREY>
    - - `VIDEO_PALETTE_HI240`
      - V4L2_PIX_FMT_HI240 <pixfmt-reserved> [#f3]_
    - - `VIDEO_PALETTE_RGB565`
      - V4L2_PIX_FMT_RGB565 <pixfmt-rgb>
    - - `VIDEO_PALETTE_RGB555`
      - V4L2_PIX_FMT_RGB555 <pixfmt-rgb>
    - - `VIDEO_PALETTE_RGB24`
      - V4L2_PIX_FMT_BGR24 <pixfmt-rgb>
    - - `VIDEO_PALETTE_RGB32`
      - V4L2_PIX_FMT_BGR32 <pixfmt-rgb> [#f4]_
    - - `VIDEO_PALETTE_YUV422`
      - V4L2_PIX_FMT_YUYV <V4L2-PIX-FMT-YUYV>
    - - `VIDEO_PALETTE_YUYV`\  [#f5]_
      - V4L2_PIX_FMT_YUYV <V4L2-PIX-FMT-YUYV>
    - - `VIDEO_PALETTE_UYVY`
      - V4L2_PIX_FMT_UYVY <V4L2-PIX-FMT-UYVY>
    - - `VIDEO_PALETTE_YUV420`
      - None
    - - `VIDEO_PALETTE_YUV411`
      - V4L2_PIX_FMT_Y41P <V4L2-PIX-FMT-Y41P> [#f6]_
    - - `VIDEO_PALETTE_RAW`
      - None [#f7]_
    - - `VIDEO_PALETTE_YUV422P`
      - V4L2_PIX_FMT_YUV422P <V4L2-PIX-FMT-YUV422P>
    - - `VIDEO_PALETTE_YUV411P`
      - V4L2_PIX_FMT_YUV411P <V4L2-PIX-FMT-YUV411P> [#f8]_
    - - `VIDEO_PALETTE_YUV420P`
      - V4L2_PIX_FMT_YVU420 <V4L2-PIX-FMT-YVU420>
    - - `VIDEO_PALETTE_YUV410P`
      - V4L2_PIX_FMT_YVU410 <V4L2-PIX-FMT-YVU410>

V4L2 图像格式定义于 pixfmt。图像格式可通过 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 选择。

## 音频


`VIDIOCGAUDIO` 和 `VIDIOCSAUDIO` ioctl 以及 struct `video_audio` 用于枚举 V4L 设备的音频输入。等价的 V4L2 ioctl 是 VIDIOC_G_AUDIO <VIDIOC_G_AUDIO> 和 VIDIOC_S_AUDIO <VIDIOC_G_AUDIO>，使用 struct `v4l2_audio`，如 audio 中所述。

用于计数音频输入的 `audio` “channel number” 字段被重命名为 `index`。

在 `VIDIOCSAUDIO` 中，`mode` 字段选择 `VIDEO_SOUND_MONO`、`VIDEO_SOUND_STEREO`、`VIDEO_SOUND_LANG1` 或 `VIDEO_SOUND_LANG2` 音频解调模式中的**一种**。当当前音频标准为 BTSC 时，`VIDEO_SOUND_LANG2` 指 SAP，而 `VIDEO_SOUND_LANG1` 没有意义。V4L 规范中也未记录，没有办法查询所选模式。在 `VIDIOCGAUDIO` 中，驱动在该字段返回**实际接收**到的音频节目。在 V4L2 API 中，该信息分别存储在 struct `v4l2_tuner` 的 `rxsubchans` 和 `audmode` 字段中。有关调谐器的更多信息请参见 tuner。与音频模式相关，struct `v4l2_audio` 还会报告这是单声道还是立体声输入，无论其来源是否为调谐器。

以下字段被可通过 VIDIOC_QUERYCTRL、VIDIOC_G_CTRL <VIDIOC_G_CTRL> 和 VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl 访问的 V4L2 控件取代：


    :header-rows:  1
    :stub-columns: 0

    - - struct `video_audio`
      - V4L2 Control ID
    - - `volume`
      - `V4L2_CID_AUDIO_VOLUME`
    - - `bass`
      - `V4L2_CID_AUDIO_BASS`
    - - `treble`
      - `V4L2_CID_AUDIO_TREBLE`
    - - `balance`
      - `V4L2_CID_AUDIO_BALANCE`

为了确定驱动支持其中哪些控件，V4L 提供了 `flags` `VIDEO_AUDIO_VOLUME`、`VIDEO_AUDIO_BASS`、`VIDEO_AUDIO_TREBLE` 和 `VIDEO_AUDIO_BALANCE`。在 V4L2 API 中，VIDIOC_QUERYCTRL ioctl 会报告相应控件是否被支持。相应地，`VIDEO_AUDIO_MUTABLE` 和 `VIDEO_AUDIO_MUTE` 标志被布尔型 `V4L2_CID_AUDIO_MUTE` 控件取代。

所有 V4L2 控件都有一个 `step` 属性，取代了 struct `video_audio` 的 `step` 字段。V4L 音频控件假定取值范围为 0 到 65535，没有特定的复位值。V4L2 API 允许任意的限制和默认值，可通过 VIDIOC_QUERYCTRL ioctl 查询。关于控件的一般信息请参见 control。

## 帧缓冲叠加


与 `VIDIOCGFBUF` 和 `VIDIOCSFBUF` 等价的 V4L2 ioctl 是 VIDIOC_G_FBUF <VIDIOC_G_FBUF> 和 VIDIOC_S_FBUF <VIDIOC_G_FBUF>。struct `video_buffer` 的 `base` 字段保持不变，但 V4L2 定义了一个标志来表示非破坏性的叠加，而非使用 `NULL` 指针。所有其他字段都移入了 struct `v4l2_framebuffer` 的 struct `v4l2_pix_format` `fmt` 子结构。`depth` 字段被 `pixelformat` 取代。关于 RGB 格式及其各自颜色深度的列表，请参见 pixfmt-rgb。

V4L2 使用通用的数据格式协商 ioctl VIDIOC_G_FMT <VIDIOC_G_FMT> 和 VIDIOC_S_FMT <VIDIOC_G_FMT>，而非特殊的 `VIDIOCGWIN` 和 `VIDIOCSWIN` ioctl。它们接受一个指向 struct `v4l2_format` 的指针作为参数。这里使用 `fmt` 联合的 `win` 成员，即 struct `v4l2_window`。

struct `video_window` 的 `x`、`y`、`width` 和 `height` 字段移入了 struct `v4l2_window` 的 struct `v4l2_rect` 子结构 `w`。`chromakey`、`clips` 和 `clipcount` 字段保持不变。struct `video_clip` 被重命名为 struct `v4l2_clip`，同样包含一个 struct `v4l2_rect`，但语义仍然相同。

`VIDEO_WINDOW_INTERLACE` 标志被移除。相反，应用程序必须将 `field` 字段设置为 `V4L2_FIELD_ANY` 或 `V4L2_FIELD_INTERLACED`。`VIDEO_WINDOW_CHROMAKEY` 标志移入了 struct `v4l2_framebuffer`，新名称为 `V4L2_FBUF_FLAG_CHROMAKEY`。

在 V4L 中，将位图指针存入 `clips` 并将 `clipcount` 设为 `VIDEO_CLIP_BITMAP` (-1) 即请求位图裁剪，使用固定大小为 1024 × 625 位的位图。struct `v4l2_window` 为此提供了一个独立的 `bitmap` 指针字段，位图大小由 `w.width` 和 `w.height` 决定。

用于启用或禁用叠加的 `VIDIOCCAPTURE` ioctl 被重命名为 VIDIOC_OVERLAY。

## 裁剪


为了仅捕获完整画面的一部分，V4L 定义了使用 struct `video_capture` 的 `VIDIOCGCAPTURE` 和 `VIDIOCSCAPTURE` ioctls。等价的 V4L2 ioctl 是使用 struct `v4l2_crop` 的 VIDIOC_G_CROP <VIDIOC_G_CROP> 和 VIDIOC_S_CROP <VIDIOC_G_CROP>，以及相关的 VIDIOC_CROPCAP ioctl。这是相当复杂的问题，详见 crop。

`x`、`y`、`width` 和 `height` 字段移入了 struct `v4l2_crop` 的 struct `v4l2_rect` 子结构 `c`。`decimation` 字段被移除。在 V4L2 API 中，缩放系数由裁剪矩形的大小以及所捕获或叠加图像的大小隐含。

用于仅捕获奇数场或偶数场的 `VIDEO_CAPTURE_ODD` 和 `VIDEO_CAPTURE_EVEN` 标志，在 struct `v4l2_pix_format` 和 struct `v4l2_window` 中名为 `field` 的字段里被 `V4L2_FIELD_TOP` 和 `V4L2_FIELD_BOTTOM` 取代。这些结构用于通过 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 选择捕获或叠加格式。

## 读取图像、内存映射


### 使用 read 方法捕获


使用 `read()` 函数从 V4L 或 V4L2 设备读取图像在本质上没有区别，但 V4L2 驱动并不要求支持这种 I/O 方法。应用程序可以通过 VIDIOC_QUERYCAP ioctl 确定该函数是否可用。所有与应用程序交换数据的 V4L2 设备都必须支持 `select()` 和 `poll()` 函数。

要选择图像格式和大小，V4L 提供 `VIDIOCSPICT` 和 `VIDIOCSWIN` ioctls。V4L2 使用通用的数据格式协商 ioctl VIDIOC_G_FMT <VIDIOC_G_FMT> 和 VIDIOC_S_FMT <VIDIOC_G_FMT>。它们接受一个指向 struct `v4l2_format` 的指针作为参数，这里使用其 `fmt` 联合中名为 `pix` 的 struct `v4l2_pix_format`。

关于 V4L2 read 接口的更多信息请参见 rw。

### 使用内存映射捕获


应用程序可以通过将设备内存中的缓冲区（或更常见地，仅在可 DMA 的系统内存中分配的缓冲区）映射到其地址空间，来从 V4L 设备读取数据。这避免了 read 方法的数据拷贝开销。V4L2 同样支持内存映射，但有一些区别。


    :header-rows:  1
    :stub-columns: 0

    - - V4L
      - V4L2
    - -
      - 在分配缓冲区之前必须选择图像格式，使用 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl。若未选择格式，驱动可能会使用上一次（可能由另一应用程序请求的）格式。
    - - 应用程序无法更改缓冲区的数量。数量内置于驱动中，除非驱动模块在加载时提供了用于更改数量的模块选项。
      - VIDIOC_REQBUFS ioctl 分配所需数量的缓冲区，这是初始化序列中必需的步骤。
    - - 驱动将所有缓冲区作为一个连续的内存范围进行映射。可使用 `VIDIOCGMBUF` ioctl 查询缓冲区数量、每个缓冲区相对于虚拟文件起始位置的偏移量，以及所用的总内存量，这些可作为 `mmap()` 函数的参数。
      - 缓冲区被单独映射。每个缓冲区的偏移量和大小可通过 VIDIOC_QUERYBUF ioctl 确定。
    - - `VIDIOCMCAPTURE` ioctl 准备一个缓冲区用于捕获，同时确定该缓冲区的图像格式。该 ioctl 立即返回，若未检测到视频信号，最终可能返回 `EAGAIN` 错误码。当驱动支持多个缓冲区时，应用程序可以多次调用该 ioctl，从而拥有多个未完成的捕获请求。

	`VIDIOCSYNC` ioctl 会挂起执行，直到特定缓冲区被填充完毕。
      - 驱动维护一个传入队列和一个传出队列。VIDIOC_QBUF 将任意空缓冲区加入传入队列。已填充的缓冲区通过 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 从传出队列取出。要等待已填充缓冲区变为可用，可以使用该函数、`select()` 或 `poll()`。在入队一个或多个缓冲区后，必须调用一次 VIDIOC_STREAMON ioctl 以开始捕获。其对应项 VIDIOC_STREAMOFF <VIDIOC_STREAMON> 会停止捕获，并从两个队列中取出所有缓冲区。若已知信号状态，应用程序可以通过 VIDIOC_ENUMINPUT ioctl 查询。

关于内存映射及示例的更深入讨论，请参见 mmap。

## 读取原始 VBI 数据


最初 V4L API 并未规定原始 VBI 捕获接口，仅为该用途保留了设备文件 `/dev/vbi`。唯一支持该接口的驱动是 BTTV 驱动，它实际上定义了 V4L VBI 接口。从该设备读取会得到一个具有以下参数的原始 VBI 图像：


    :header-rows:  1
    :stub-columns: 0

    - - struct `v4l2_vbi_format`
      - V4L, BTTV driver
    - - sampling_rate
      - 28636363 Hz NTSC (or any other 525-line standard); 35468950 Hz PAL
	and SECAM (625-line standards)
    - - offset
      - ?
    - - samples_per_line
      - 2048
    - - sample_format
      - V4L2_PIX_FMT_GREY。最后四个字节（一个机器字节序整数）包含一个帧计数器。
    - - start[]
      - 10, 273 NTSC; 22, 335 PAL and SECAM
    - - count[]
      - 16, 16 [#f9]_
    - - flags
      - 0

V4L 规范中未记录，在 Linux 2.3 中加入了使用 struct `vbi_format` 的 `VIDIOCGVBIFMT` 和 `VIDIOCSVBIFMT` ioctls，用于确定 VBI 图像参数。这些 ioctl 仅与 raw-vbi 中规定的 V4L2 VBI 接口部分兼容。

不存在 `offset` 字段，`sample_format` 应为 `VIDEO_PALETTE_RAW`，等价于 `V4L2_PIX_FMT_GREY`。其余字段可能等价于 struct `v4l2_vbi_format`。

显然只有 Zoran（ZR 36120）驱动实现了这些 ioctl。其语义与 V4L2 的规定有两处不同：参数在 `open()` 时重置，且当参数无效时 `VIDIOCSVBIFMT` 总是返回 `EINVAL` 错误码。

## 杂项


V4L2 没有与 `VIDIOCGUNIT` ioctl 等价的内容。应用程序可以通过重新打开设备并请求 VBI 数据，来找到与视频捕获设备关联的 VBI 设备（反之亦然）。详情请参见 open。

对于 `VIDIOCKEY` 以及 V4L 的微码编程函数，目前没有替代项。关于 MPEG 压缩和播放设备的新接口记录在 extended-controls 中。

   根据 Documentation/admin-guide/devices.rst，这些应该是指向 `/dev/video0` 的符号链接。注意原始的 bttv 接口与 V4L 或 V4L2 均不兼容。

   根据 `Documentation/admin-guide/devices.rst`，这是一个指向 `/dev/radio0` 的符号链接。

   这是 BTTV 驱动使用的自定义格式，并非 V4L2 标准格式之一。

   推测所有 V4L RGB 格式都是小端序，尽管某些驱动可能按机器字节序来解释它们。V4L2 定义了小端序、大端序以及红/蓝交换的变体。详情请参见 pixfmt-rgb。

   `VIDEO_PALETTE_YUV422` 和 `VIDEO_PALETTE_YUYV` 是同一格式。某些 V4L 驱动响应其中一种，某些响应另一种。

   不要与 `V4L2_PIX_FMT_YUV411P` 混淆，后者是平面（planar）格式。

   V4L 将其解释为："RAW capture (BT848)"

   不要与 `V4L2_PIX_FMT_Y41P` 混淆，后者是打包（packed）格式。

   旧的驱动版本使用了不同的值，最终加入了自定义的 `BTTV_VBISIZE` ioctl 来查询正确的值。
