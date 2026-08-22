

######## 编解码器控制参


下面描述编解码器控制类中的所有控件。首先是通用控件，然后是特定于某些硬件的控件


   这些控件并非仅适用MPEG，而是适用于所有编解码器。这些定义以
   V4L2_CID_MPEG/V4L2_MPEG 为前缀，因为控件最初是MPEG 编解码器创建的，
   后来被扩展以涵盖所有编码格式


## 通用编解码器控件



### 编解码器控件 ID



`V4L2_CID_CODEC_CLASS (class)`
    编解码器（Codec）类的描述符。对该控件调VIDIOC_QUERYCTRL 将返
    该控件类的描述。例如，此描述可用作 GUI 中某个选项卡（Tab）页面的
    标题


`V4L2_CID_MPEG_STREAM_TYPE`
    (enum)

enum v4l2_mpeg_stream_type -
    MPEG-12 -4 输出流类型。这里不能做任何假设。每种硬MPEG 编码器往往支持可用 MPEG 流类型的不同子集。该控件专用于多路复用的 MPEG 流。当前已定义的流类型如下



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_STREAM_TYPE_MPEG2_PS`
      - MPEG-2 节目
    - - `V4L2_MPEG_STREAM_TYPE_MPEG2_TS`
      - MPEG-2 传输
    - - `V4L2_MPEG_STREAM_TYPE_MPEG1_SS`
      - MPEG-1 系统
    - - `V4L2_MPEG_STREAM_TYPE_MPEG2_DVD`
      - MPEG-2 DVD 兼容
    - - `V4L2_MPEG_STREAM_TYPE_MPEG1_VCD`
      - MPEG-1 VCD 兼容
    - - `V4L2_MPEG_STREAM_TYPE_MPEG2_SVCD`
      - MPEG-2 SVCD 兼容



`V4L2_CID_MPEG_STREAM_PID_PMT (integer)`
    MPEG 传输流的程序映射表（PMT）包 ID（默16

`V4L2_CID_MPEG_STREAM_PID_AUDIO (integer)`
    MPEG 传输流的音频ID（默256

`V4L2_CID_MPEG_STREAM_PID_VIDEO (integer)`
    MPEG 传输流的视频ID（默260

`V4L2_CID_MPEG_STREAM_PID_PCR (integer)`
    承载 PCR 字段MPEG 传输流包 ID（默259

`V4L2_CID_MPEG_STREAM_PES_ID_AUDIO (integer)`
    MPEG PES 的音ID

`V4L2_CID_MPEG_STREAM_PES_ID_VIDEO (integer)`
    MPEG PES 的视ID


`V4L2_CID_MPEG_STREAM_VBI_FMT`
    (enum)

enum v4l2_mpeg_stream_vbi_fmt -
    某些卡可以将 VBI 数据（例如字幕（Closed Caption）、图文电视（Teletext））嵌入MPEG 流中。该控件选择是否应嵌VBI 数据，以及如果嵌入，应采用何种嵌入方式。可能的 VBI 格式列表取决于驱动。当前已定义VBI 格式类型如下



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_STREAM_VBI_FMT_NONE`
      - MPEG 流中VBI
    - - `V4L2_MPEG_STREAM_VBI_FMT_IVTV`
      - 私有包中VBI，IVTV 格式（在内核源码文件
	`Documentation/userspace-api/media/drivers/cx2341x-uapi.rst` 中有文档说明



`V4L2_CID_MPEG_AUDIO_SAMPLING_FREQ`
    (enum)

enum v4l2_mpeg_audio_sampling_freq -
    MPEG 音频采样频率。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_SAMPLING_FREQ_44100`
      - 44.1 kHz
    - - `V4L2_MPEG_AUDIO_SAMPLING_FREQ_48000`
      - 48 kHz
    - - `V4L2_MPEG_AUDIO_SAMPLING_FREQ_32000`
      - 32 kHz



`V4L2_CID_MPEG_AUDIO_ENCODING`
    (enum)

enum v4l2_mpeg_audio_encoding -
    MPEG 音频编码。该控件专用于多路复用的 MPEG 流。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_ENCODING_LAYER_1`
      - MPEG-1/2 第一层（Layer I）编
    - - `V4L2_MPEG_AUDIO_ENCODING_LAYER_2`
      - MPEG-1/2 第二层（Layer II）编
    - - `V4L2_MPEG_AUDIO_ENCODING_LAYER_3`
      - MPEG-1/2 第三层（Layer III）编
    - - `V4L2_MPEG_AUDIO_ENCODING_AAC`
      - MPEG-2/4 AAC（高级音频编码，Advanced Audio Coding
    - - `V4L2_MPEG_AUDIO_ENCODING_AC3`
      - AC-3，即 ATSC A/52 编码



`V4L2_CID_MPEG_AUDIO_L1_BITRATE`
    (enum)

enum v4l2_mpeg_audio_l1_bitrate -
    MPEG-1/2 第一层比特率。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_L1_BITRATE_32K`
      - 32 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_64K`
      - 64 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_96K`
      - 96 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_128K`
      - 128 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_160K`
      - 160 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_192K`
      - 192 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_224K`
      - 224 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_256K`
      - 256 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_288K`
      - 288 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_320K`
      - 320 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_352K`
      - 352 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_384K`
      - 384 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_416K`
      - 416 kbit/s
    - - `V4L2_MPEG_AUDIO_L1_BITRATE_448K`
      - 448 kbit/s



`V4L2_CID_MPEG_AUDIO_L2_BITRATE`
    (enum)

enum v4l2_mpeg_audio_l2_bitrate -
    MPEG-1/2 第二层比特率。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_L2_BITRATE_32K`
      - 32 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_48K`
      - 48 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_56K`
      - 56 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_64K`
      - 64 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_80K`
      - 80 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_96K`
      - 96 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_112K`
      - 112 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_128K`
      - 128 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_160K`
      - 160 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_192K`
      - 192 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_224K`
      - 224 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_256K`
      - 256 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_320K`
      - 320 kbit/s
    - - `V4L2_MPEG_AUDIO_L2_BITRATE_384K`
      - 384 kbit/s



`V4L2_CID_MPEG_AUDIO_L3_BITRATE`
    (enum)

enum v4l2_mpeg_audio_l3_bitrate -
    MPEG-1/2 第三层比特率。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_L3_BITRATE_32K`
      - 32 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_40K`
      - 40 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_48K`
      - 48 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_56K`
      - 56 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_64K`
      - 64 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_80K`
      - 80 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_96K`
      - 96 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_112K`
      - 112 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_128K`
      - 128 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_160K`
      - 160 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_192K`
      - 192 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_224K`
      - 224 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_256K`
      - 256 kbit/s
    - - `V4L2_MPEG_AUDIO_L3_BITRATE_320K`
      - 320 kbit/s



`V4L2_CID_MPEG_AUDIO_AAC_BITRATE (integer)`
    AAC 比特率，单位为比特每秒


`V4L2_CID_MPEG_AUDIO_AC3_BITRATE`
    (enum)

enum v4l2_mpeg_audio_ac3_bitrate -
    AC-3 比特率。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_32K`
      - 32 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_40K`
      - 40 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_48K`
      - 48 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_56K`
      - 56 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_64K`
      - 64 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_80K`
      - 80 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_96K`
      - 96 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_112K`
      - 112 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_128K`
      - 128 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_160K`
      - 160 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_192K`
      - 192 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_224K`
      - 224 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_256K`
      - 256 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_320K`
      - 320 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_384K`
      - 384 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_448K`
      - 448 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_512K`
      - 512 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_576K`
      - 576 kbit/s
    - - `V4L2_MPEG_AUDIO_AC3_BITRATE_640K`
      - 640 kbit/s



`V4L2_CID_MPEG_AUDIO_MODE`
    (enum)

enum v4l2_mpeg_audio_mode -
    MPEG 音频模式。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_MODE_STEREO`
      - 绔嬩綋澹。
    - - `V4L2_MPEG_AUDIO_MODE_JOINT_STEREO`
      - 鑱斿悎绔嬩綋澹。
    - - `V4L2_MPEG_AUDIO_MODE_DUAL`
      - 双语
    - - `V4L2_MPEG_AUDIO_MODE_MONO`
      - 鍗曞０閬。

`V4L2_CID_MPEG_AUDIO_MODE_EXTENSION`
    (enum)

enum v4l2_mpeg_audio_mode_extension -
    联合立体声音频模式扩展。在第一层和第二层中，它们指示哪些子带采用强度立体声（intensity stereo）。其余子带以立体声编码。第三层尚不支持（或未支持）。可能的值如下：


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_4`
      - 子带 4-31 采用强度立体
    - - `V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_8`
      - 子带 8-31 采用强度立体
    - - `V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_12`
      - 子带 12-31 采用强度立体
    - - `V4L2_MPEG_AUDIO_MODE_EXTENSION_BOUND_16`
      - 子带 16-31 采用强度立体



`V4L2_CID_MPEG_AUDIO_EMPHASIS`
    (enum)

enum v4l2_mpeg_audio_emphasis -
    音频预加重（Emphasis）。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_EMPHASIS_NONE`
      - 鏃。
    - - `V4L2_MPEG_AUDIO_EMPHASIS_50_DIV_15_uS`
      - 50/15 微秒预加
    - - `V4L2_MPEG_AUDIO_EMPHASIS_CCITT_J17`
      - CCITT J.17



`V4L2_CID_MPEG_AUDIO_CRC`
    (enum)

enum v4l2_mpeg_audio_crc -
    CRC 方法。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_CRC_NONE`
      - 鏃。
    - - `V4L2_MPEG_AUDIO_CRC_CRC16`
      - 16 位奇偶校



`V4L2_CID_MPEG_AUDIO_MUTE (boolean)`
    采集时静音音频。这不是通过静音音频硬件来实现的（硬件静音仍可能产生轻微嘶嘶声），而是在编码器内部完成，从而保证固定的、可复现的音频码流 = 非静音，1 = 静音


`V4L2_CID_MPEG_AUDIO_DEC_PLAYBACK`
    (enum)

enum v4l2_mpeg_audio_dec_playback -
    决定单语（monolingual）音频应如何播放。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_AUTO`
      - 自动确定最佳播放模式
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_STEREO`
      - 立体声播放
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_LEFT`
      - 左声道播放
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_RIGHT`
      - 右声道播放
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_MONO`
      - 单声道播放
    - - `V4L2_MPEG_AUDIO_DEC_PLAYBACK_SWAPPED_STEREO`
      - 左右声道互换的立体声播放



`V4L2_CID_MPEG_AUDIO_DEC_MULTILINGUAL_PLAYBACK`
    (enum)

enum v4l2_mpeg_audio_dec_playback -
    决定多语言音频应如何播放


`V4L2_CID_MPEG_VIDEO_ENCODING`
    (enum)

enum v4l2_mpeg_video_encoding -
    MPEG 视频编码方法。该控件专用于多路复用的 MPEG 流。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_ENCODING_MPEG_1`
      - MPEG-1 视频编码
    - - `V4L2_MPEG_VIDEO_ENCODING_MPEG_2`
      - MPEG-2 视频编码
    - - `V4L2_MPEG_VIDEO_ENCODING_MPEG_4_AVC`
      - MPEG-4 AVC（H.264）视频编



`V4L2_CID_MPEG_VIDEO_ASPECT`
    (enum)

enum v4l2_mpeg_video_aspect -
    视频宽高比。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_ASPECT_1x1`
    - - `V4L2_MPEG_VIDEO_ASPECT_4x3`
    - - `V4L2_MPEG_VIDEO_ASPECT_16x9`
    - - `V4L2_MPEG_VIDEO_ASPECT_221x100`



`V4L2_CID_MPEG_VIDEO_B_FRAMES (integer)`
    B 帧数量（默认 2

`V4L2_CID_MPEG_VIDEO_GOP_SIZE (integer)`
    GOP 大小（默12

`V4L2_CID_MPEG_VIDEO_GOP_CLOSURE (boolean)`
    GOP 闭合（默1

`V4L2_CID_MPEG_VIDEO_PULLDOWN (boolean)`
    启用 3:2 下拉（pulldown）（默认 0


`V4L2_CID_MPEG_VIDEO_BITRATE_MODE`
    (enum)

enum v4l2_mpeg_video_bitrate_mode -
    视频比特率模式。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_BITRATE_MODE_VBR`
      - 可变比特
    - - `V4L2_MPEG_VIDEO_BITRATE_MODE_CBR`
      - 恒定比特
    - - `V4L2_MPEG_VIDEO_BITRATE_MODE_CQ`
      - 恒定质量



`V4L2_CID_MPEG_VIDEO_BITRATE (integer)`
    平均视频比特率，单位为比特每秒

`V4L2_CID_MPEG_VIDEO_BITRATE_PEAK (integer)`
    峰值视频比特率，单位为比特每秒。必须大于或等于平均视频比特率。如果视频比特率模式设置为恒定比特率，则该控件被忽略

`V4L2_CID_MPEG_VIDEO_CONSTANT_QUALITY (integer)`
    恒定质量等级控制。当 `V4L2_CID_MPEG_VIDEO_BITRATE_MODE` 的值为 `V4L2_MPEG_VIDEO_BITRATE_MODE_CQ` 时适用此控件。有效范围为 1 100，其1 表示最低质量，100 表示最高质量。编码器将决定适当的量化参数和比特率，以产生所请求的帧质量


`V4L2_CID_MPEG_VIDEO_FRAME_SKIP_MODE (enum)`

enum v4l2_mpeg_video_frame_skip_mode -
    指示编码器在何种条件下应跳过帧。如果编码某一帧会导致编码后的流大于所选的数据限制，则该帧将被跳过。可能的值如下：



    \small

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_DISABLED`
      - 帧跳过模式已禁用
    - - `V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_LEVEL_LIMIT`
      - 帧跳过模式已启用，缓冲区限制由所选等级设定，并由标准定义
    - - `V4L2_MPEG_VIDEO_FRAME_SKIP_MODE_BUF_LIMIT`
      - 帧跳过模式已启用，缓冲区限制VBV（MPEG1/2/4<v4l2-mpeg-video-vbv-size> CPB（H264）缓冲区大小 <v4l2-mpeg-video-h264-cpb-size> 控件设定


    \normalsize

`V4L2_CID_MPEG_VIDEO_TEMPORAL_DECIMATION (integer)`
    对于每一帧采集到的帧，跳过其后这么多帧（默认 0）

`V4L2_CID_MPEG_VIDEO_MUTE (boolean)`
    采集时将视频“静音”为固定颜色。这对于测试以产生固定的视频码流很有用 = 非静音，1 = 静音

`V4L2_CID_MPEG_VIDEO_MUTE_YUV (integer)`
    设置视频的“静音”颜色。所提供32 位整数按如下方式解释（bit 0 = 最低有效位）：



    :header-rows:  0
    :stub-columns: 0

    - - Bit 0:7
      - V 色度信息
    - - Bit 8:15
      - U 色度信息
    - - Bit 16:23
      - Y 亮度信息
    - - Bit 24:31
      - 必须为零



`V4L2_CID_MPEG_VIDEO_DEC_PTS (integer64)`
    这个只读控件返回当前显示帧的 33 位视频显示时间戳（Presentation Time Stamp），其定义见 ITU T-REC-H.222.0 ISO/IEC 13818-1。它VIDIOC_DECODER_CMD 中所用的 PTS 相同


`V4L2_CID_MPEG_VIDEO_DEC_FRAME (integer64)`
    这个只读控件返回当前显示（已解码）帧的帧计数器。每当解码器启动时，该值会被重置为 0

`V4L2_CID_MPEG_VIDEO_DEC_CONCEAL_COLOR (integer64)`
    此控件设YUV 色彩空间中的隐藏（conceal）颜色。它描述在参考帧缺失导致出错时，客户端对错误隐藏颜色的偏好。解码器应使用偏好颜色填充参考缓冲区，并将其用于后续解码。该控件每个通道使用 16 位。适用于解码器

    :header-rows:  0
    :stub-columns: 0

    - -
      - 8 位格
      - 10 位格
      - 12 位格
    - - Y 亮度
      - Bit 0:7
      - Bit 0:9
      - Bit 0:11
    - - Cb 色度
      - Bit 16:23
      - Bit 16:25
      - Bit 16:27
    - - Cr 色度
      - Bit 32:39
      - Bit 32:41
      - Bit 32:43
    - - 必须为零
      - Bit 48:63
      - Bit 48:63
      - Bit 48:63

`V4L2_CID_MPEG_VIDEO_DECODER_SLICE_INTERFACE (boolean)`
    如果启用，解码器期望每个缓冲区接收单个切片（slice）；否则解码器期望每个缓冲区接收单帧。适用于解码器，所有编解码器

`V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY_ENABLE (boolean)`
    如果启用了显示延迟，则解码器在处理一定数量的 OUTPUT 缓冲区后，被迫返回一CAPTURE 缓冲区（已解码帧）。该延迟可通过 `V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY` 设置。例如，此特性可用于生成视频缩略图。适用于解码器

`V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY (integer)`
    解码器的显示延迟值。解码器在设定的“显示延迟”帧数之后被迫返回一帧已解码帧。如果该数值较小，可能导致返回的帧乱序显示；此外硬件可能仍将该返回缓冲区用作后续帧的参考图像

`V4L2_CID_MPEG_VIDEO_AU_DELIMITER (boolean)`
    如果启用，将生成 AUD（访问单元定界符，Access Unit Delimiter）NALU。这在无需完全解析每个 NALU 即可找到帧起始时很有用。适用H264 HEVC 编码器

`V4L2_CID_MPEG_VIDEO_H264_VUI_SAR_ENABLE (boolean)`
    启用在视频可用性信息（Video Usability Information）中写入采样宽高比。适用H264 编码器


`V4L2_CID_MPEG_VIDEO_H264_VUI_SAR_IDC`
    (enum)

enum v4l2_mpeg_video_h264_vui_sar_idc -
    用于 H.264 编码VUI 采样宽高比指示符。该值在标准E-1 中定义。适用H264 编码器



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_UNSPECIFIED`
      - 未指
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_1x1`
      - 1x1
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_12x11`
      - 12x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_10x11`
      - 10x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_16x11`
      - 16x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_40x33`
      - 40x33
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_24x11`
      - 24x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_20x11`
      - 20x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_32x11`
      - 32x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_80x33`
      - 80x33
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_18x11`
      - 18x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_15x11`
      - 15x11
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_64x33`
      - 64x33
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_160x99`
      - 160x99
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_4x3`
      - 4x3
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_3x2`
      - 3x2
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_2x1`
      - 2x1
    - - `V4L2_MPEG_VIDEO_H264_VUI_SAR_IDC_EXTENDED`
      - 扩展 SAR


`V4L2_CID_MPEG_VIDEO_H264_VUI_EXT_SAR_WIDTH (integer)`
    用于 H.264 VUI 编码的扩展采样宽高比宽度。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_VUI_EXT_SAR_HEIGHT (integer)`
    用于 H.264 VUI 编码的扩展采样宽高比高度。适用H264 编码器


`V4L2_CID_MPEG_VIDEO_H264_LEVEL`
    (enum)

enum v4l2_mpeg_video_h264_level -
    H264 视频基本流的等级（level）信息。适用H264 编码器。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1_0`
      - Level 1.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1B`
      - Level 1B
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1_1`
      - Level 1.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1_2`
      - Level 1.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_1_3`
      - Level 1.3
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_2_0`
      - Level 2.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_2_1`
      - Level 2.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_2_2`
      - Level 2.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_3_0`
      - Level 3.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_3_1`
      - Level 3.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_3_2`
      - Level 3.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_4_0`
      - Level 4.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_4_1`
      - Level 4.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_4_2`
      - Level 4.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_5_0`
      - Level 5.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_5_1`
      - Level 5.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_5_2`
      - Level 5.2
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_6_0`
      - Level 6.0
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_6_1`
      - Level 6.1
    - - `V4L2_MPEG_VIDEO_H264_LEVEL_6_2`
      - Level 6.2



`V4L2_CID_MPEG_VIDEO_MPEG2_LEVEL`
    (enum)

enum v4l2_mpeg_video_mpeg2_level -
    MPEG2 基本流的等级信息。适用MPEG2 编解码器。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MPEG2_LEVEL_LOW`
      - 低等级（LL
    - - `V4L2_MPEG_VIDEO_MPEG2_LEVEL_MAIN`
      - 主等级（ML
    - - `V4L2_MPEG_VIDEO_MPEG2_LEVEL_HIGH_1440`
      - 1440 等级（H-14
    - - `V4L2_MPEG_VIDEO_MPEG2_LEVEL_HIGH`
      - 高等级（HL

`V4L2_CID_MPEG_VIDEO_MPEG4_LEVEL`
    (enum)

enum v4l2_mpeg_video_mpeg4_level -
    MPEG4 基本流的等级信息。适用MPEG4 编码器。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_0`
      - Level 0
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_0B`
      - Level 0b
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_1`
      - Level 1
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_2`
      - Level 2
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_3`
      - Level 3
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_3B`
      - Level 3b
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_4`
      - Level 4
    - - `V4L2_MPEG_VIDEO_MPEG4_LEVEL_5`
      - Level 5



`V4L2_CID_MPEG_VIDEO_H264_PROFILE`
    (enum)

enum v4l2_mpeg_video_h264_profile -
    H264 的档次（profile）信息。适用H264 编码器。可能的值如下：



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_PROFILE_BASELINE`
      - 基线档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_CONSTRAINED_BASELINE`
      - 受限基线档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_MAIN`
      - 主档
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_EXTENDED`
      - 扩展档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH`
      - 高档
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_10`
      - 10 档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_422`
      - 422 档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_444_PREDICTIVE`
      - 444 预测档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_10_INTRA`
      - 10 Intra 档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_422_INTRA`
      - 422 Intra 档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_HIGH_444_INTRA`
      - 444 Intra 档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_CAVLC_444_INTRA`
      - CAVLC 444 Intra 档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_BASELINE`
      - 可伸缩基线档
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_HIGH`
      - 可伸缩高档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_SCALABLE_HIGH_INTRA`
      - 可伸缩高 Intra 档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_STEREO_HIGH`
      - 立体声高档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_MULTIVIEW_HIGH`
      - 多视点高档次
    - - `V4L2_MPEG_VIDEO_H264_PROFILE_CONSTRAINED_HIGH`
      - 受限高档


    \normalsize


`V4L2_CID_MPEG_VIDEO_MPEG2_PROFILE`
    (enum)

enum v4l2_mpeg_video_mpeg2_profile -
    MPEG2 的档次信息。适用MPEG2 编解码器。可能的值如下：



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_SIMPLE`
      - 简单档次（SP
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_MAIN`
      - 主档次（MP
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_SNR_SCALABLE`
      - SNR 可伸缩档次（SNR
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_SPATIALLY_SCALABLE`
      - 空间可伸缩档次（Spt
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_HIGH`
      - 高档次（HP
    - - `V4L2_MPEG_VIDEO_MPEG2_PROFILE_MULTIVIEW`
      - 多视点档次（MVP



    \normalsize


`V4L2_CID_MPEG_VIDEO_MPEG4_PROFILE`
    (enum)

enum v4l2_mpeg_video_mpeg4_profile -
    MPEG4 的档次信息。适用MPEG4 编码器。可能的值如下：



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_SIMPLE`
      - 简单档
    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_ADVANCED_SIMPLE`
      - 高级简单档
    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_CORE`
      - 核心档次
    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_SIMPLE_SCALABLE`
      - 简单可伸缩档次
    - - `V4L2_MPEG_VIDEO_MPEG4_PROFILE_ADVANCED_CODING_EFFICIENCY`
      - 高级编码效率档次


    \normalsize

`V4L2_CID_MPEG_VIDEO_MAX_REF_PIC (integer)`
    用于编码的参考图像的最大数量。适用于编码器


`V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MODE`
    (enum)

enum v4l2_mpeg_video_multi_slice_mode -
    决定编码器应如何将帧划分为切片（slice）。适用于编码器。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_SINGLE`
      - 每帧单个切片
    - - `V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_MB`
      - 多个切片，每个切片设定最大宏块数
    - - `V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_BYTES`
      - 多个切片，每个切片设定最大字节数



`V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MAX_MB (integer)`
    一个切片中的最大宏块数。当 `V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MODE` 设置`V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_MB` 时使用。适用于编码器

`V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MAX_BYTES (integer)`
    一个切片的最大字节数。当 `V4L2_CID_MPEG_VIDEO_MULTI_SLICE_MODE` 设置`V4L2_MPEG_VIDEO_MULTI_SLICE_MODE_MAX_BYTES` 时使用。适用于编码器


`V4L2_CID_MPEG_VIDEO_H264_LOOP_FILTER_MODE`
    (enum)

enum v4l2_mpeg_video_h264_loop_filter_mode -
    H264 编码器的环内滤波器模式。可能的值如下：



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_ENABLED`
      - 环内滤波器已启用
    - - `V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_DISABLED`
      - 环内滤波器已禁用
    - - `V4L2_MPEG_VIDEO_H264_LOOP_FILTER_MODE_DISABLED_AT_SLICE_BOUNDARY`
      - 在切片边界处禁用环内滤波器


    \normalsize


`V4L2_CID_MPEG_VIDEO_H264_LOOP_FILTER_ALPHA (integer)`
    环内滤波alpha 系数，定义于 H264 标准。该值对应于 slice header 字段 slice_alpha_c0_offset_div2，取值范围应-6 +6（含）。实际的 alpha 偏移 FilterOffsetA 是此值的两倍。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_LOOP_FILTER_BETA (integer)`
    环内滤波beta 系数，定义于 H264 标准。该值对应于 slice header 字段 slice_beta_offset_div2，取值范围应-6 +6（含）。实际的 beta 偏移 FilterOffsetB 是此值的两倍。适用H264 编码器


`V4L2_CID_MPEG_VIDEO_H264_ENTROPY_MODE`
    (enum)

enum v4l2_mpeg_video_h264_entropy_mode -
    H264 的熵编码模式 - CABAC/CAVLC。适用H264 编码器。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_ENTROPY_MODE_CAVLC`
      - 使用 CAVLC 熵编码
    - - `V4L2_MPEG_VIDEO_H264_ENTROPY_MODE_CABAC`
      - 使用 CABAC 熵编码


`V4L2_CID_MPEG_VIDEO_H264_8X8_TRANSFORM (boolean)`
    H264 启用 8X8 变换。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_CONSTRAINED_INTRA_PREDICTION (boolean)`
    H264 启用受限帧内预测。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_CHROMA_QP_INDEX_OFFSET (integer)`
    指定应加到亮度量化参数上以确定色度量化参数的偏移量。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_CYCLIC_INTRA_REFRESH_MB (integer)`
    循环帧内宏块刷新。这是每帧刷新的连续宏块数量。每一帧依次刷新一组宏块，直到整个循环完成并从帧顶部重新开始。将此控件设为零表示不刷新宏块。注意，`V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD` 控件被设为非零值时，此控件将不起作用。适用H264、H263 MPEG4 编码器

`V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD_TYPE (enum)`

enum v4l2_mpeg_video_intra_refresh_period_type -
    设置帧内刷新的类型。刷新整个帧的周期由 V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD 指定。注意，如果不存在此控件，则所使用的刷新类型未定义，由驱动决定。适用H264 HEVC 编码器。可能的值如下：


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_INTRA_REFRESH_PERIOD_TYPE_RANDOM`
      - 在指定周期后，整个帧被随机地完全刷新
    - - `V4L2_MPEG_VIDEO_INTRA_REFRESH_PERIOD_TYPE_CYCLIC`
      - 在指定周期后，整个帧的宏块按循环顺序被完全刷新

`V4L2_CID_MPEG_VIDEO_INTRA_REFRESH_PERIOD (integer)`
    帧内宏块刷新周期。它设置刷新整个帧的周期。换句话说，它定义了整个帧将被帧内刷新的帧数。例如：将周期设1 表示整个帧将被刷新；设为 2 表示一半宏块在 frameX 上进行帧内刷新，另一半宏块在 frameX + 1 上刷新，依此类推。将周期设为零表示未指定周期。注意，如果客户端将此控件设为非零值，`V4L2_CID_MPEG_VIDEO_CYCLIC_INTRA_REFRESH_MB` 控件应被忽略。适用H264 HEVC 编码器

`V4L2_CID_MPEG_VIDEO_FRAME_RC_ENABLE (boolean)`
    帧级码率控制使能。如果禁用此控件，则每种帧类型的量化参数为常量，并通过相应控件设置（例`V4L2_CID_MPEG_VIDEO_H263_I_FRAME_QP`）。如果启用帧码率控制，则量化参数会被调整以满足所选比特率。量化参数的最小值和最大值可通过相应控件设置（例`V4L2_CID_MPEG_VIDEO_H263_MIN_QP`）。适用于编码器

`V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE (boolean)`
    宏块级码率控制使能。适用MPEG4 H264 编码器

`V4L2_CID_MPEG_VIDEO_MPEG4_QPEL (boolean)`
    MPEG4 1/4 像素运动估计。适用MPEG4 编码器

`V4L2_CID_MPEG_VIDEO_H263_I_FRAME_QP (integer)`
    H263 I 帧量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_H263_MIN_QP (integer)`
    H263 的最小量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_H263_MAX_QP (integer)`
    H263 的最大量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_H263_P_FRAME_QP (integer)`
    H263 P 帧量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_H263_B_FRAME_QP (integer)`
    H263 B 帧量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_H264_I_FRAME_QP (integer)`
    H264 I 帧量化参数。有效范围：0 51

`V4L2_CID_MPEG_VIDEO_H264_MIN_QP (integer)`
    H264 的最小量化参数。有效范围：0 51

`V4L2_CID_MPEG_VIDEO_H264_MAX_QP (integer)`
    H264 的最大量化参数。有效范围：0 51

`V4L2_CID_MPEG_VIDEO_H264_P_FRAME_QP (integer)`
    H264 P 帧量化参数。有效范围：0 51

`V4L2_CID_MPEG_VIDEO_H264_B_FRAME_QP (integer)`
    H264 B 帧量化参数。有效范围：0 51

`V4L2_CID_MPEG_VIDEO_H264_I_FRAME_MIN_QP (integer)`
    用于限制 H264 I 帧质量范围的 H264 I 帧最小量化参数。有效范围：0 51。如果同时设置了 V4L2_CID_MPEG_VIDEO_H264_MIN_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_H264_I_FRAME_MAX_QP (integer)`
    用于限制 H264 I 帧质量范围的 H264 I 帧最大量化参数。有效范围：0 51。如果同时设置了 V4L2_CID_MPEG_VIDEO_H264_MAX_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_H264_P_FRAME_MIN_QP (integer)`
    用于限制 H264 P 帧质量范围的 H264 P 帧最小量化参数。有效范围：0 51。如果同时设置了 V4L2_CID_MPEG_VIDEO_H264_MIN_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_H264_P_FRAME_MAX_QP (integer)`
    用于限制 H264 P 帧质量范围的 H264 P 帧最大量化参数。有效范围：0 51。如果同时设置了 V4L2_CID_MPEG_VIDEO_H264_MAX_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_H264_B_FRAME_MIN_QP (integer)`
    用于限制 H264 B 帧质量范围的 H264 B 帧最小量化参数。有效范围：0 51。如果同时设置了 V4L2_CID_MPEG_VIDEO_H264_MIN_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_H264_B_FRAME_MAX_QP (integer)`
    用于限制 H264 B 帧质量范围的 H264 B 帧最大量化参数。有效范围：0 51。如果同时设置了 V4L2_CID_MPEG_VIDEO_H264_MAX_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_MPEG4_I_FRAME_QP (integer)`
    MPEG4 I 帧量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_MPEG4_MIN_QP (integer)`
    MPEG4 的最小量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_MPEG4_MAX_QP (integer)`
    MPEG4 的最大量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_MPEG4_P_FRAME_QP (integer)`
    MPEG4 P 帧量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_MPEG4_B_FRAME_QP (integer)`
    MPEG4 B 帧量化参数。有效范围：1 31


`V4L2_CID_MPEG_VIDEO_VBV_SIZE (integer)`
    视频缓冲校验器（Video Buffer Verifier）大小，单位为千字节，用作帧跳过的限制。VBV 在标准中被定义为一种验证所产生码流能否被成功解码的手段。标准将其描述为“一个假设解码器的一部分，在概念上连接到编码器的输出。其目的是对编码器或编辑过程可能产生的数据速率的变化性施加约束。”。适用MPEG1、MPEG2、MPEG4 编码器


`V4L2_CID_MPEG_VIDEO_VBV_DELAY (integer)`
    VBV 缓冲区控制设置初始延迟，单位为毫秒


`V4L2_CID_MPEG_VIDEO_MV_H_SEARCH_RANGE (integer)`
    水平搜索范围定义了在当前宏块（MB）于参考图像中搜索和匹配时的最大水平搜索区域（以像素计）。此 V4L2 控件宏用于设置视频编码器中运动估计模块的水平搜索范围


`V4L2_CID_MPEG_VIDEO_MV_V_SEARCH_RANGE (integer)`
    垂直搜索范围定义了在当前宏块（MB）于参考图像中搜索和匹配时的最大垂直搜索区域（以像素计）。此 V4L2 控件宏用于设置视频编码器中运动估计模块的垂直搜索范围


`V4L2_CID_MPEG_VIDEO_FORCE_KEY_FRAME (button)`
    为下一个排队的缓冲区强制生成关键帧。适用于编码器。这是一个通用的、与编解码器无关的强制关键帧控件


`V4L2_CID_MPEG_VIDEO_H264_CPB_SIZE (integer)`
    编码图像缓冲区（Coded Picture Buffer）大小，单位为千字节，用作帧跳过的限制。CPB H264 标准中被定义为一种验证所产生码流能否被成功解码的手段。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_I_PERIOD (integer)`
    H264 在开GOP I 帧之间的周期。对于开GOP，这是两I 帧之间的周期。IDR（Instantaneous Decoding Refresh，即时解码刷新）帧之间的周期取自 GOP_SIZE 控件。IDR 帧代表即时解码刷新，是一种在其之后不再引用任何先前帧I 帧。这意味着可以IDR 帧重新开始码流，而无需存储或解码任何先前帧。适用H264 编码器


`V4L2_CID_MPEG_VIDEO_HEADER_MODE`
    (enum)

enum v4l2_mpeg_video_header_mode -
    决定头部是作为第一个缓冲区返回，还是与第一帧一起返回。适用于编码器。可能的值如下：


    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEADER_MODE_SEPARATE`
      - 流头部在第一个缓冲区中单独返回
    - - `V4L2_MPEG_VIDEO_HEADER_MODE_JOINED_WITH_1ST_FRAME`
      - 流头部与第一帧编码帧一起返回


    \normalsize


`V4L2_CID_MPEG_VIDEO_REPEAT_SEQ_HEADER (boolean)`
    重复视频序列头。重复这些头部使对视频流的随机访问更容易。适用MPEG1 4 编码器

`V4L2_CID_MPEG_VIDEO_DECODER_MPEG4_DEBLOCK_FILTER (boolean)`
    MPEG4 解码器启用去块后处理滤波器。适用MPEG4 解码器

`V4L2_CID_MPEG_VIDEO_MPEG4_VOP_TIME_RES (integer)`
    MPEG4 vop_time_increment_resolution 值。适用MPEG4 编码器

`V4L2_CID_MPEG_VIDEO_MPEG4_VOP_TIME_INC (integer)`
    MPEG4 vop_time_increment 值。适用MPEG4 编码器

`V4L2_CID_MPEG_VIDEO_H264_SEI_FRAME_PACKING (boolean)`
    在编码码流中启用生成帧封装补充增强信息（frame packing SEI）。帧封装 SEI 消息包含用于 3D 观看L R 平面的排列方式。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_SEI_FP_CURRENT_FRAME_0 (boolean)`
    在帧封装 SEI 中将当前帧设frame0。适用H264 编码器


`V4L2_CID_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE`
    (enum)

enum v4l2_mpeg_video_h264_sei_fp_arrangement_type -
    H264 SEI 的帧封装排列方式类型。适用H264 编码器。可能的值如下：



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_CHEKERBOARD`
      - 像素交替来自 L R
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_COLUMN`
      - L R 按列隔行排列
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_ROW`
      - L R 按行隔行排列
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_SIDE_BY_SIDE`
      - L 在左，R 在右
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_TOP_BOTTOM`
      - L 在上，R 在下
    - - `V4L2_MPEG_VIDEO_H264_SEI_FP_ARRANGEMENT_TYPE_TEMPORAL`
      - 每帧一个视点


    \normalsize


`V4L2_CID_MPEG_VIDEO_H264_FMO (boolean)`
    在编码码流中启用灵活宏块排序（FMO）。这是一种用于重组图像中宏块顺序的技术。适用H264 编码器


`V4L2_CID_MPEG_VIDEO_H264_FMO_MAP_TYPE`
   (enum)

enum v4l2_mpeg_video_h264_fmo_map_type -
    使用 FMO 时，映射类型将图像划分为不同的宏块扫描模式。适用H264 编码器。可能的值如下：



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_INTERLEAVED_SLICES`
      - 切片按游程长度顺序彼此交织排列宏块
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_SCATTERED_SLICES`
      - 基于编码器和解码器双方均已知的数学函数分散宏块
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_FOREGROUND_WITH_LEFT_OVER`
      - 宏块排列在矩形区域或感兴趣区域内
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_BOX_OUT`
      - 切片组从中心向外以循环方式增长
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_RASTER_SCAN`
      - 切片组按光栅扫描模式从左到右增长
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_WIPE_SCAN`
      - 切片组按擦除扫描模式从上到下增长
    - - `V4L2_MPEG_VIDEO_H264_FMO_MAP_TYPE_EXPLICIT`
      - 用户自定义映射类型


    \normalsize


`V4L2_CID_MPEG_VIDEO_H264_FMO_SLICE_GROUP (integer)`
    FMO 中切片组的数量。适用H264 编码器


`V4L2_CID_MPEG_VIDEO_H264_FMO_CHANGE_DIRECTION`
    (enum)

enum v4l2_mpeg_video_h264_fmo_change_dir -
    指定光栅和擦除映射的切片组变化方向。适用H264 编码器。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_FMO_CHANGE_DIR_RIGHT`
      - 光栅扫描或向右擦除
    - - `V4L2_MPEG_VIDEO_H264_FMO_CHANGE_DIR_LEFT`
      - 反向光栅扫描或向左擦除



`V4L2_CID_MPEG_VIDEO_H264_FMO_CHANGE_RATE (integer)`
    指定光栅和擦除映射中第一个切片组的大小。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_FMO_RUN_LENGTH (integer)`
    指定交织映射中连续宏块的数量。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_ASO (boolean)`
    在编码码流中启用任意切片排序（ASO）。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_ASO_SLICE_ORDER (integer)`
    指定 ASO 中的切片顺序。适用H264 编码器。所提供32 位整数按如下方式解释（bit 0 = 最低有效位）：



    :header-rows:  0
    :stub-columns: 0

    - - Bit 0:15
      - 切片 ID
    - - Bit 16:32
      - 切片位置或顺



`V4L2_CID_MPEG_VIDEO_H264_HIERARCHICAL_CODING (boolean)`
    启用 H264 分层编码。适用H264 编码器


`V4L2_CID_MPEG_VIDEO_H264_HIERARCHICAL_CODING_TYPE`
    (enum)

enum v4l2_mpeg_video_h264_hierarchical_coding_type -
    指定分层编码类型。适用H264 编码器。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_H264_HIERARCHICAL_CODING_B`
      - 分层 B 编码
    - - `V4L2_MPEG_VIDEO_H264_HIERARCHICAL_CODING_P`
      - 分层 P 编码



`V4L2_CID_MPEG_VIDEO_H264_HIERARCHICAL_CODING_LAYER (integer)`
    指定分层编码层的数量。适用H264 编码器

`V4L2_CID_MPEG_VIDEO_H264_HIERARCHICAL_CODING_LAYER_QP (integer)`
    为每一层指定用户自定义QP。适用H264 编码器。所提供32 位整数按如下方式解释（bit 0 = 最低有效位）：



    :header-rows:  0
    :stub-columns: 0

    - - Bit 0:15
      - QP 鍊。
    - - Bit 16:32
      - 层编

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L0_BR (integer)`
    指示 H264 编码器分层编码层 0 的比特率（bps）

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L1_BR (integer)`
    指示 H264 编码器分层编码层 1 的比特率（bps）

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L2_BR (integer)`
    指示 H264 编码器分层编码层 2 的比特率（bps）

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L3_BR (integer)`
    指示 H264 编码器分层编码层 3 的比特率（bps）

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L4_BR (integer)`
    指示 H264 编码器分层编码层 4 的比特率（bps）

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L5_BR (integer)`
    指示 H264 编码器分层编码层 5 的比特率（bps）

`V4L2_CID_MPEG_VIDEO_H264_HIER_CODING_L6_BR (integer)`
    指示 H264 编码器分层编码层 6 的比特率（bps）

`V4L2_CID_FWHT_I_FRAME_QP (integer)`
    FWHT I 帧量化参数。有效范围：1 31

`V4L2_CID_FWHT_P_FRAME_QP (integer)`
    FWHT P 帧量化参数。有效范围：1 31

`V4L2_CID_MPEG_VIDEO_AVERAGE_QP (integer)`
    这个只读控件返回当前已编码帧的平QP 值。该值适用于最后一个出队的捕获缓冲区（VIDIOC_DQBUF）。其有效范围取决于编码格式和参数。对H264，有效范围为 0 51。对HEVC 位时0 510 位时0 63。对H263 MPEG4，有效范围为 1 31。对VP8，有效范围为 0 127。对VP9，有效范围为 0 255。如果编解码器的 MIN_QP MAX_QP 已设置，QP 将满足两者的要求。编解码器需要始终使用指定的范围，而不是硬件自定义范围。适用于编码器


    \normalsize

## MFC 5.1 MPEG 控件


以下 MPEG 类控件涉及特定于三星 S5P 系列 SoC Multi Format Codec 5.1 设备MPEG 解码与编码设置



### MFC 5.1 控件 ID


`V4L2_CID_MPEG_MFC51_VIDEO_DECODER_H264_DISPLAY_DELAY_ENABLE (boolean)`
    如果启用了显示延迟，则解码器在处理一定数量的 OUTPUT 缓冲区后，被迫返回一CAPTURE 缓冲区（已解码帧）。该延迟可通过 `V4L2_CID_MPEG_MFC51_VIDEO_DECODER_H264_DISPLAY_DELAY` 设置。例如，此特性可用于生成视频缩略图。适用H264 解码器

```

       This control is deprecated. Use the standard
       ``V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY_ENABLE`` control instead.

```
`V4L2_CID_MPEG_MFC51_VIDEO_DECODER_H264_DISPLAY_DELAY (integer)`
    显示延迟值，用于 H264 解码器。解码器在设定的“显示延迟”帧数之后被迫返回一帧已解码帧。如果该数值较小，可能导致返回的帧乱序显示；此外硬件可能仍将该返回缓冲区用作后续帧的参考图像

```

       This control is deprecated. Use the standard
       ``V4L2_CID_MPEG_VIDEO_DEC_DISPLAY_DELAY`` control instead.

```
`V4L2_CID_MPEG_MFC51_VIDEO_H264_NUM_REF_PIC_FOR_P (integer)`
    用于编码 P 帧的参考图像数量。适用H264 编码器

`V4L2_CID_MPEG_MFC51_VIDEO_PADDING (boolean)`
    在编码器中启用填充——使用颜色而不是重复边界像素。适用于编码器

`V4L2_CID_MPEG_MFC51_VIDEO_PADDING_YUV (integer)`
    编码器中的填充颜色。适用于编码器。所提供32 位整数按如下方式解释（bit 0 = 最低有效位）：



    :header-rows:  0
    :stub-columns: 0

    - - Bit 0:7
      - V 色度信息
    - - Bit 8:15
      - U 色度信息
    - - Bit 16:23
      - Y 亮度信息
    - - Bit 24:31
      - 必须为零



`V4L2_CID_MPEG_MFC51_VIDEO_RC_REACTION_COEFF (integer)`
    MFC 码率控制的反应系数。适用于编码器

```

       #. Valid only when the frame level RC is enabled.

       #. For tight CBR, this field must be small (ex. 2 ~ 10). For
	  VBR, this field must be large (ex. 100 ~ 1000).

       #. It is not recommended to use the greater number than
	  FRAME_RATE * (10^9 / BIT_RATE).

```
`V4L2_CID_MPEG_MFC51_VIDEO_H264_ADAPTIVE_RC_DARK (boolean)`
    针对暗区域的自适应码率控制。仅当启H.264 和宏块级码率控制（`V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE`）时有效。适用H264 编码器

`V4L2_CID_MPEG_MFC51_VIDEO_H264_ADAPTIVE_RC_SMOOTH (boolean)`
    针对平滑区域的自适应码率控制。仅当启H.264 和宏块级码率控制（`V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE`）时有效。适用H264 编码器

`V4L2_CID_MPEG_MFC51_VIDEO_H264_ADAPTIVE_RC_STATIC (boolean)`
    针对静态区域的自适应码率控制。仅当启H.264 和宏块级码率控制（`V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE`）时有效。适用H264 编码器

`V4L2_CID_MPEG_MFC51_VIDEO_H264_ADAPTIVE_RC_ACTIVITY (boolean)`
    针对活动区域的自适应码率控制。仅当启H.264 和宏块级码率控制（`V4L2_CID_MPEG_VIDEO_MB_RC_ENABLE`）时有效。适用H264 编码器


`V4L2_CID_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE`
    (enum)

```

       This control is deprecated. Use the standard
       ``V4L2_CID_MPEG_VIDEO_FRAME_SKIP_MODE`` control instead.

```
enum v4l2_mpeg_mfc51_video_frame_skip_mode -
    指示编码器在何种条件下应跳过帧。如果编码某一帧会导致编码后的流大于所选的数据限制，则该帧将被跳过。可能的值如下：



    \small

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_DISABLED`
      - 帧跳过模式已禁用
    - - `V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_LEVEL_LIMIT`
      - 帧跳过模式已启用，缓冲区限制由所选等级设定，并由标准定义
    - - `V4L2_MPEG_MFC51_VIDEO_FRAME_SKIP_MODE_BUF_LIMIT`
      - 帧跳过模式已启用，缓冲区限制VBV（MPEG1/2/4）或 CPB（H264）缓冲区大小控件设定


    \normalsize

`V4L2_CID_MPEG_MFC51_VIDEO_RC_FIXED_TARGET_BIT (integer)`
    启用具有固定目标比特的码率控制。如果启用此设置，编码器的码率控制逻辑将为 GOP 计算平均比特率，并使其低于或等于设定的比特率目标。否则，码率控制逻辑计算整个码流的总体平均比特率，并使其低于或等于设定比特率。在第一种情况下，整个码流的平均比特率将小于设定比特率。这是因为平均值是基于较少的帧数计算的；另一方面，启用此设置可确保码流满足严格的带宽约束。适用于编码器


`V4L2_CID_MPEG_MFC51_VIDEO_FORCE_FRAME_TYPE`
    (enum)

enum v4l2_mpeg_mfc51_video_force_frame_type -
    为下一个排队的缓冲区强制帧类型。适用于编码器。可能的值如下：


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_MFC51_FORCE_FRAME_TYPE_DISABLED`
      - 禁用强制特定帧类型
    - - `V4L2_MPEG_MFC51_FORCE_FRAME_TYPE_I_FRAME`
      - 强制 I 帧
    - - `V4L2_MPEG_MFC51_FORCE_FRAME_TYPE_NOT_CODED`
      - 强制非编码帧


## CX2341x MPEG 控件


以下 MPEG 类控件涉及特定于 Conexant CX23415 CX23416 MPEG 编码芯片MPEG 编码设置



### CX2341x 控件 ID



`V4L2_CID_MPEG_CX2341X_VIDEO_SPATIAL_FILTER_MODE`
    (enum)

enum v4l2_mpeg_cx2341x_video_spatial_filter_mode -
    设置空间滤波器模式（默认 `MANUAL`）。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_SPATIAL_FILTER_MODE_MANUAL`
      - 鎵嬪姩閫夋嫨婊ゆ尝鍣。
    - - `V4L2_MPEG_CX2341X_VIDEO_SPATIAL_FILTER_MODE_AUTO`
      - 自动选择滤波



`V4L2_CID_MPEG_CX2341X_VIDEO_SPATIAL_FILTER (integer (0-15))`
    空间滤波器的设置 = 关闭5 = 最大。（默认 0。）


`V4L2_CID_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE`
    (enum)

enum v4l2_mpeg_cx2341x_video_luma_spatial_filter_type -
    选择用于亮度空间滤波器的算法（默`1D_HOR`）。可能的值：



    \footnotesize

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_OFF`
      - 无滤波器
    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_1D_HOR`
      - 一维水
    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_1D_VERT`
      - 一维垂
    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_2D_HV_SEPARABLE`
      - 二维可分
    - - `V4L2_MPEG_CX2341X_VIDEO_LUMA_SPATIAL_FILTER_TYPE_2D_SYM_NON_SEPARABLE`
      - 二维对称不可分离


    \normalsize


`V4L2_CID_MPEG_CX2341X_VIDEO_CHROMA_SPATIAL_FILTER_TYPE`
    (enum)

enum v4l2_mpeg_cx2341x_video_chroma_spatial_filter_type -
    选择用于色度空间滤波器的算法（默`1D_HOR`）。可能的值如下：


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_CHROMA_SPATIAL_FILTER_TYPE_OFF`
      - 无滤波器
    - - `V4L2_MPEG_CX2341X_VIDEO_CHROMA_SPATIAL_FILTER_TYPE_1D_HOR`
      - 一维水


    \normalsize


`V4L2_CID_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER_MODE`
    (enum)

enum v4l2_mpeg_cx2341x_video_temporal_filter_mode -
    设置时间滤波器模式（默认 `MANUAL`）。可能的值如下：


    \footnotesize

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER_MODE_MANUAL`
      - 鎵嬪姩閫夋嫨婊ゆ尝鍣。
    - - `V4L2_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER_MODE_AUTO`
      - 自动选择滤波


    \normalsize

`V4L2_CID_MPEG_CX2341X_VIDEO_TEMPORAL_FILTER (integer (0-31))`
    时间滤波器的设置 = 关闭1 = 最大。（全分辨率采集时默8，缩放采集时默认 0。）

`V4L2_CID_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE`
    (enum)

enum v4l2_mpeg_cx2341x_video_median_filter_type -
    中值滤波器类型（默`OFF`）。可能的值如下：



    \small


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_OFF`
      - 无滤波器
    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_HOR`
      - 姘村钩婊ゆ尝鍣。
    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_VERT`
      - 鍨傜洿婊ゆ尝鍣。
    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_HOR_VERT`
      - 水平和垂直滤波器
    - - `V4L2_MPEG_CX2341X_VIDEO_MEDIAN_FILTER_TYPE_DIAG`
      - 对角滤波


    \normalsize

`V4L2_CID_MPEG_CX2341X_VIDEO_LUMA_MEDIAN_FILTER_BOTTOM (integer (0-255))`
    启用亮度中值滤波器的阈值上限（默认 0

`V4L2_CID_MPEG_CX2341X_VIDEO_LUMA_MEDIAN_FILTER_TOP (integer (0-255))`
    启用亮度中值滤波器的阈值下限（默认 255

`V4L2_CID_MPEG_CX2341X_VIDEO_CHROMA_MEDIAN_FILTER_BOTTOM (integer (0-255))`
    启用色度中值滤波器的阈值上限（默认 0

`V4L2_CID_MPEG_CX2341X_VIDEO_CHROMA_MEDIAN_FILTER_TOP (integer (0-255))`
    启用色度中值滤波器的阈值下限（默认 255

`V4L2_CID_MPEG_CX2341X_STREAM_INSERT_NAV_PACKETS (boolean)`
    CX2341X MPEG 编码器可以在每四个视频帧之间向码流中插入一个空MPEG-2 PES 包。包大小2048 字节，包packet_start_code_prefix stream_id 字段。stream_id 0xBF（私有流 2）。载荷由 0x00 字节组成，由应用程序填充 = 不插入，1 = 插入包

## VPX 控件参


VPX 控件包含用于 VPx 视频编解码器编码参数的控件



### VPX 控件 ID



`V4L2_CID_MPEG_VIDEO_VPX_NUM_PARTITIONS`
    (enum)

enum v4l2_vp8_num_partitions -
    VP8 编码器中使用的标记（token）分区数量。可能的值如下：



    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CID_MPEG_VIDEO_VPX_1_PARTITION`
      - 1 个系数分
    - - `V4L2_CID_MPEG_VIDEO_VPX_2_PARTITIONS`
      - 2 个系数分
    - - `V4L2_CID_MPEG_VIDEO_VPX_4_PARTITIONS`
      - 4 个系数分
    - - `V4L2_CID_MPEG_VIDEO_VPX_8_PARTITIONS`
      - 8 个系数分



`V4L2_CID_MPEG_VIDEO_VPX_IMD_DISABLE_4X4 (boolean)`
    设置此项可防止在帧内模式决策中使用帧4x4 模式


`V4L2_CID_MPEG_VIDEO_VPX_NUM_REF_FRAMES`
    (enum)

enum v4l2_vp8_num_ref_frames -
    用于编码 P 帧的参考图像数量。可能的值如下：



    \small

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CID_MPEG_VIDEO_VPX_1_REF_FRAME`
      - 将搜索最后一帧已编码
    - - `V4L2_CID_MPEG_VIDEO_VPX_2_REF_FRAME`
      - 将在最后一帧已编码帧、黄金帧（golden frame）和备用参考（altref）帧中搜索两帧。编码器实现将决定选择哪两帧
    - - `V4L2_CID_MPEG_VIDEO_VPX_3_REF_FRAME`
      - 将搜索最后一帧已编码帧、黄金帧altref 帧


    \normalsize



`V4L2_CID_MPEG_VIDEO_VPX_FILTER_LEVEL (integer)`
    指示环路滤波器等级。环路滤波器等级的调整是通过相对于基准环路滤波器值的增量值来完成的

`V4L2_CID_MPEG_VIDEO_VPX_FILTER_SHARPNESS (integer)`
    此参数影响环路滤波器。任何大于零的值都会减弱环路滤波器的去块效应

`V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_REF_PERIOD (integer)`
    设置黄金帧的刷新周期。该周期以帧数定义。对于'n'，从第一个关键帧开始，每第 n 帧将被视为黄金帧。例如，对于编码序列 0，若黄金帧刷新周期设4，则0 等将被视为黄金帧，因为帧 0 始终是关键帧


`V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_SEL`
    (enum)

enum v4l2_vp8_golden_frame_sel -
    选择用于编码的黄金帧。可能的值如下：


    \scriptsize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_USE_PREV`
      - 使用(n-2) 帧作为黄金帧，当前帧索引'n'
    - - `V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_USE_REF_PERIOD`
      - 使用`V4L2_CID_MPEG_VIDEO_VPX_GOLDEN_FRAME_REF_PERIOD` 指示的前一个特定帧作为黄金帧


    \normalsize


`V4L2_CID_MPEG_VIDEO_VPX_MIN_QP (integer)`
    VP8 的最小量化参数

`V4L2_CID_MPEG_VIDEO_VPX_MAX_QP (integer)`
    VP8 的最大量化参数

`V4L2_CID_MPEG_VIDEO_VPX_I_FRAME_QP (integer)`
    VP8 I 帧量化参数

`V4L2_CID_MPEG_VIDEO_VPX_P_FRAME_QP (integer)`
    VP8 P 帧量化参数


`V4L2_CID_MPEG_VIDEO_VP8_PROFILE`
    (enum)

enum v4l2_mpeg_video_vp8_profile -
    此控件用于选择 VP8 编码器的档次。它也用于枚VP8 编码器或解码器支持的档次。可能的值如下：

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_VP8_PROFILE_0`
      - Profile 0
    - - `V4L2_MPEG_VIDEO_VP8_PROFILE_1`
      - Profile 1
    - - `V4L2_MPEG_VIDEO_VP8_PROFILE_2`
      - Profile 2
    - - `V4L2_MPEG_VIDEO_VP8_PROFILE_3`
      - Profile 3


`V4L2_CID_MPEG_VIDEO_VP9_PROFILE`
    (enum)

enum v4l2_mpeg_video_vp9_profile -
    此控件用于选择 VP9 编码器的档次。它也用于枚VP9 编码器或解码器支持的档次。可能的值如下：

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_VP9_PROFILE_0`
      - Profile 0
    - - `V4L2_MPEG_VIDEO_VP9_PROFILE_1`
      - Profile 1
    - - `V4L2_MPEG_VIDEO_VP9_PROFILE_2`
      - Profile 2
    - - `V4L2_MPEG_VIDEO_VP9_PROFILE_3`
      - Profile 3


`V4L2_CID_MPEG_VIDEO_VP9_LEVEL (enum)`

enum v4l2_mpeg_video_vp9_level -
    此控件用于选择 VP9 编码器的等级。它也用于枚VP9 编码器或解码器支持的等级。更多信息可参阅 `webmproject <https://www.webmproject.org/vp9/levels/>`__。可能的值如下：

    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_1_0`
      - Level 1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_1_1`
      - Level 1.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_2_0`
      - Level 2
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_2_1`
      - Level 2.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_3_0`
      - Level 3
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_3_1`
      - Level 3.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_4_0`
      - Level 4
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_4_1`
      - Level 4.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_5_0`
      - Level 5
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_5_1`
      - Level 5.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_5_2`
      - Level 5.2
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_6_0`
      - Level 6
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_6_1`
      - Level 6.1
    - - `V4L2_MPEG_VIDEO_VP9_LEVEL_6_2`
      - Level 6.2


## 高效视频编码（HEVC/H.265）控件参


HEVC/H.265 控件包含用于 HEVC/H.265 视频编解码器编码参数的控件



### HEVC/H.265 控件 ID



`V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP (integer)`
    HEVC 的最小量化参数。有效范围：8 位时0 510 位时0 63

`V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP (integer)`
    HEVC 的最大量化参数。有效范围：8 位时0 510 位时0 63

`V4L2_CID_MPEG_VIDEO_HEVC_I_FRAME_QP (integer)`
    HEVC I 帧量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_P_FRAME_QP (integer)`
    HEVC P 帧量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_B_FRAME_QP (integer)`
    HEVC B 帧量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_I_FRAME_MIN_QP (integer)`
    用于限制 HEVC I 帧质量范围的 HEVC I 帧最小量化参数。有效范围：8 位时0 510 位时0 63。如果同时设置了 V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_HEVC_I_FRAME_MAX_QP (integer)`
    用于限制 HEVC I 帧质量范围的 HEVC I 帧最大量化参数。有效范围：8 位时0 510 位时0 63。如果同时设置了 V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_HEVC_P_FRAME_MIN_QP (integer)`
    用于限制 HEVC P 帧质量范围的 HEVC P 帧最小量化参数。有效范围：8 位时0 510 位时0 63。如果同时设置了 V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_HEVC_P_FRAME_MAX_QP (integer)`
    用于限制 HEVC P 帧质量范围的 HEVC P 帧最大量化参数。有效范围：8 位时0 510 位时0 63。如果同时设置了 V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_HEVC_B_FRAME_MIN_QP (integer)`
    用于限制 HEVC B 帧质量范围的 HEVC B 帧最小量化参数。有效范围：8 位时0 510 位时0 63。如果同时设置了 V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_HEVC_B_FRAME_MAX_QP (integer)`
    用于限制 HEVC B 帧质量范围的 HEVC B 帧最大量化参数。有效范围：8 位时0 510 位时0 63。如果同时设置了 V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP，则量化参数应满足两者的要求

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_QP (boolean)`
    HIERARCHICAL_QP 允许主机通过 HIERARCHICAL_QP_LAYER 为每一时间层指定量化参数值。仅HIERARCHICAL_CODING_LAYER 大于 1 时有效。将此控件值设1 可启用各层的 QP 值设置


`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_TYPE`
    (enum)

enum v4l2_mpeg_video_hevc_hier_coding_type -
    选择用于编码的分层编码类型。可能的值如下：


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_HIERARCHICAL_CODING_B`
      - 使用 B 帧进行分层编码
    - - `V4L2_MPEG_VIDEO_HEVC_HIERARCHICAL_CODING_P`
      - 使用 P 帧进行分层编码


    \normalsize


`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_LAYER (integer)`
    选择分层编码层。在普通编码（非分层编码）中，应设为零。可能的值为 [0, 6] 表示分层编码0 表示分层编码1，依此类推

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L0_QP (integer)`
    指示分层编码0 的量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L1_QP (integer)`
    指示分层编码1 的量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L2_QP (integer)`
    指示分层编码2 的量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L3_QP (integer)`
    指示分层编码3 的量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L4_QP (integer)`
    指示分层编码4 的量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L5_QP (integer)`
    指示分层编码5 的量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L6_QP (integer)`
    指示分层编码6 的量化参数。有效范围：[V4L2_CID_MPEG_VIDEO_HEVC_MIN_QP, V4L2_CID_MPEG_VIDEO_HEVC_MAX_QP]


`V4L2_CID_MPEG_VIDEO_HEVC_PROFILE`
    (enum)

enum v4l2_mpeg_video_hevc_profile -
    HEVC 编码器选择所需的档次


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN`
      - 主档次
    - - `V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN_STILL_PICTURE`
      - 主静态图像档次
    - - `V4L2_MPEG_VIDEO_HEVC_PROFILE_MAIN_10`
      - 10 档次


    \normalsize



`V4L2_CID_MPEG_VIDEO_HEVC_LEVEL`
    (enum)

enum v4l2_mpeg_video_hevc_level -
    HEVC 编码器选择所需的等级

==================================	=========
`V4L2_MPEG_VIDEO_HEVC_LEVEL_1`	Level 1.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_2`	Level 2.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_2_1`	Level 2.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_3`	Level 3.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_3_1`	Level 3.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_4`	Level 4.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_4_1`	Level 4.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_5`	Level 5.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_5_1`	Level 5.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_5_2`	Level 5.2
`V4L2_MPEG_VIDEO_HEVC_LEVEL_6`	Level 6.0
`V4L2_MPEG_VIDEO_HEVC_LEVEL_6_1`	Level 6.1
`V4L2_MPEG_VIDEO_HEVC_LEVEL_6_2`	Level 6.2
==================================	=========

`V4L2_CID_MPEG_VIDEO_HEVC_FRAME_RATE_RESOLUTION (integer)`
    指示一秒内的均匀间隔子区间（称为 ticks）数量。这是一16 位无符号整数，最大值为 0xffff，最小值为 1


`V4L2_CID_MPEG_VIDEO_HEVC_TIER`
    (enum)

enum v4l2_mpeg_video_hevc_tier -
    TIER_FLAG 指定 HEVC 编码图像的层级（tier）信息。层级是为了处理最大比特率不同的应用而设立的。将该标志设0 选择 HEVC Main 层级，设1 表示 High 层级。High 层级用于需要高比特率的应用

==================================	==========
`V4L2_MPEG_VIDEO_HEVC_TIER_MAIN`	主层级
`V4L2_MPEG_VIDEO_HEVC_TIER_HIGH`	高层级
==================================	==========


`V4L2_CID_MPEG_VIDEO_HEVC_MAX_PARTITION_DEPTH (integer)`
    选择 HEVC 最大编码单元深度


`V4L2_CID_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE`
    (enum)

enum v4l2_mpeg_video_hevc_loop_filter_mode -
    HEVC 编码器的环路滤波器模式。可能的值如下：


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_DISABLED`
      - 环路滤波器已禁用
    - - `V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_ENABLED`
      - 环路滤波器已启用
    - - `V4L2_MPEG_VIDEO_HEVC_LOOP_FILTER_MODE_DISABLED_AT_SLICE_BOUNDARY`
      - 在切片边界处禁用环路滤波器


    \normalsize


`V4L2_CID_MPEG_VIDEO_HEVC_LF_BETA_OFFSET_DIV2 (integer)`
    选择 HEVC 环路滤波beta 偏移。有效范围为 [-6, +6]

`V4L2_CID_MPEG_VIDEO_HEVC_LF_TC_OFFSET_DIV2 (integer)`
    选择 HEVC 环路滤波tc 偏移。有效范围为 [-6, +6]


`V4L2_CID_MPEG_VIDEO_HEVC_REFRESH_TYPE`
    (enum)

enum v4l2_mpeg_video_hevc_hier_refresh_type -
    选择 HEVC 编码器的刷新类型。主机必须将周期指定V4L2_CID_MPEG_VIDEO_HEVC_REFRESH_PERIOD


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_REFRESH_NONE`
      - 使用 B 帧进行分层编码
    - - `V4L2_MPEG_VIDEO_HEVC_REFRESH_CRA`
      - 使用 CRA（Clean Random Access Unit）图像编码
    - - `V4L2_MPEG_VIDEO_HEVC_REFRESH_IDR`
      - 使用 IDR（Instantaneous Decoding Refresh）图像编码


    \normalsize


`V4L2_CID_MPEG_VIDEO_HEVC_REFRESH_PERIOD (integer)`
    选择 HEVC 编码器的刷新周期。它指定两个 CRA/IDR 图像之间I 图像数量。仅REFRESH_TYPE 不为 0 时有效

`V4L2_CID_MPEG_VIDEO_HEVC_LOSSLESS_CU (boolean)`
    指示 HEVC 无损编码。设0 禁用无损编码，设1 启用无损编码

`V4L2_CID_MPEG_VIDEO_HEVC_CONST_INTRA_PRED (boolean)`
    指示 HEVC 编码器的恒定帧内预测。指定受限帧内预测，其中帧内最大编码单元（LCU）的预测仅使用相邻帧LCU 的残差数据和已解码样本来进行。将该值设1 启用恒定帧内预测，设0 禁用恒定帧内预测

`V4L2_CID_MPEG_VIDEO_HEVC_WAVEFRONT (boolean)`
    指示 HEVC 编码器的波前并行处理（wavefront parallel processing）。设0 禁用该特性，设为 1 启用波前并行处理

`V4L2_CID_MPEG_VIDEO_HEVC_GENERAL_PB (boolean)`
    将该值设1 可为 HEVC 编码器启P 帧和 B 帧的组合

`V4L2_CID_MPEG_VIDEO_HEVC_TEMPORAL_ID (boolean)`
    指示 HEVC 编码器的时间标识符，通过将值设1 来启用

`V4L2_CID_MPEG_VIDEO_HEVC_STRONG_SMOOTHING (boolean)`
    指示当设1 时，CVS 的帧内预测滤波过程中有条件地使用双线性插值。指示当设为 0 时，CVS 中不使用双线性插值

`V4L2_CID_MPEG_VIDEO_HEVC_MAX_NUM_MERGE_MV_MINUS1 (integer)`
    指示合并候选运动矢量的最大数量。取值范围为 0 4

`V4L2_CID_MPEG_VIDEO_HEVC_TMV_PREDICTION (boolean)`
    指示 HEVC 编码器的时间运动矢量预测。设1 启用预测，设0 禁用预测

`V4L2_CID_MPEG_VIDEO_HEVC_WITHOUT_STARTCODE (boolean)`
    指定 HEVC 是否生成以长度字段大小代替起始码模式的码流。长度字段的大小可通过 V4L2_CID_MPEG_VIDEO_HEVC_SIZE_OF_LENGTH_FIELD 控件配置。将该值设0 禁用无起始码模式的编码。将该值设1 将启用无起始码模式的编码


`V4L2_CID_MPEG_VIDEO_HEVC_SIZE_OF_LENGTH_FIELD`
(enum)

enum v4l2_mpeg_video_hevc_size_of_length_field -
    指示长度字段的大小。当启用 WITHOUT_STARTCODE_ENABLE 编码时有效


    \footnotesize


    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_MPEG_VIDEO_HEVC_SIZE_0`
      - 生成起始码模式（普通）
    - - `V4L2_MPEG_VIDEO_HEVC_SIZE_1`
      - 生成长度字段的大小代替起始码模式，长度为 1
    - - `V4L2_MPEG_VIDEO_HEVC_SIZE_2`
      - 生成长度字段的大小代替起始码模式，长度为 2
    - - `V4L2_MPEG_VIDEO_HEVC_SIZE_4`
      - 生成长度字段的大小代替起始码模式，长度为 4


    \normalsize

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L0_BR (integer)`
    指示 HEVC 编码器分层编码层 0 的比特率

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L1_BR (integer)`
    指示 HEVC 编码器分层编码层 1 的比特率

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L2_BR (integer)`
    指示 HEVC 编码器分层编码层 2 的比特率

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L3_BR (integer)`
    指示 HEVC 编码器分层编码层 3 的比特率

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L4_BR (integer)`
    指示 HEVC 编码器分层编码层 4 的比特率

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L5_BR (integer)`
    指示 HEVC 编码器分层编码层 5 的比特率

`V4L2_CID_MPEG_VIDEO_HEVC_HIER_CODING_L6_BR (integer)`
    指示 HEVC 编码器分层编码层 6 的比特率

`V4L2_CID_MPEG_VIDEO_REF_NUMBER_FOR_PFRAMES (integer)`
    选择 HEVC 编码器所需P 参考图像数量。P 帧可使用 1 2 帧作为参考

`V4L2_CID_MPEG_VIDEO_PREPEND_SPSPPS_TO_IDR (integer)`
    指示是否在每IDR 处生SPS PPS。设0 禁用在每IDR 处生SPS PPS。设1 启用在每IDR 处生SPS PPS

