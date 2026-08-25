


######## FM Transmitter Control Reference


FM 发射器（FM_TX）类包含具备 FM 发射能力设备的常见功能控件。目前该类包含音压缩、导频音生成、音频频偏限制器、RDS 发射与调谐功率等功能的参数

## FM_TX 控制 ID


`V4L2_CID_FM_TX_CLASS (class)`
    FM_TX 类描述符。对该控件调VIDIOC_QUERYCTRL 将返回此控件类的描述
`V4L2_CID_RDS_TX_DEVIATION (integer)`
    Hz 为单位配RDS 信号频偏电平。范围与步进由驱动决定
`V4L2_CID_RDS_TX_PI (integer)`
    设置用于发射RDS 节目识别（Programme Identification）字段
`V4L2_CID_RDS_TX_PTY (integer)`
    设置用于发射RDS 节目类型（Programme Type）字段。其编码最31     预定义节目类型
`V4L2_CID_RDS_TX_PS_NAME (string)`
    设置用于发射的节目服务名称（PS_NAME）。它用于接收机上的静态显示，    听众识别和选择节目服务的主要辅助。在 RDS 规范 iec62106 的附E 中，    节目服务名称字符串的正确字符编码有完整说明。同样根RDS 规范，PS 通常
    是一8 个字符的文本。但也可能找到能够滚动显8×N 个字符字符串的接收机    因此，此控件必须8 个字符为步进配置，结果它必须始终包含一个大小为 8     整数倍的字符串
`V4L2_CID_RDS_TX_RADIO_TEXT (string)`
    设置用于发射Radio Text 信息。它是对正在广播内容的文字描述。当广播    希望传输更长PS 名称、与节目相关的信息或任何其他文本时，可以使用 RDS
    Radio Text。在这些情况下，RadioText 应与 `V4L2_CID_RDS_TX_PS_NAME` 配合
    使用。Radio Text 字符串的编码同样iec62106 的附E 中有完整说明。Radio
    Text 字符串的长度取决于用于传输它RDS 块，32A 块）64B 块）    但也可能找到能够滚动显示 32×N 64×N 个字符字符串的接收机。因此，此控    必须32 64 个字符为步进配置，结果它必须始终包含一个大小为 32 64     整数倍的字符串
`V4L2_CID_RDS_TX_MONO_STEREO (boolean)`
    设置解码器识别码（Decoder Identification code）的 Mono/Stereo 位。若设置    则表示音频以立体声录制
`V4L2_CID_RDS_TX_ARTIFICIAL_HEAD (boolean)`
    设置解码器识别码`Artificial Head <http://en.wikipedia.org/wiki/Artificial_head>`__
    位。若设置，则表示音频使用人工头（artificial head）录制
`V4L2_CID_RDS_TX_COMPRESSED (boolean)`
    设置解码器识别码Compressed 位。若设置，则表示音频经过压缩
`V4L2_CID_RDS_TX_DYNAMIC_PTY (boolean)`
    设置解码器识别码Dynamic PTY 位。若设置，则表示 PTY 码被动态切换
`V4L2_CID_RDS_TX_TRAFFIC_ANNOUNCEMENT (boolean)`
    若设置，则表示正在进行交通公告
`V4L2_CID_RDS_TX_TRAFFIC_PROGRAM (boolean)`
    若设置，则表示当前调谐的节目携带交通公告
`V4L2_CID_RDS_TX_MUSIC_SPEECH (boolean)`
    若设置，则表示该频道广播音乐；若清除，则表示广播语音。如果发射器不做    区分，则应该设置它
`V4L2_CID_RDS_TX_ALT_FREQS_ENABLE (boolean)`
    若设置，则表示发射备用频率
`V4L2_CID_RDS_TX_ALT_FREQS (__u32 array)`
    kHz 为单位的备用频率。RDS 标准允许定义最25 个频率。驱动可能支    更少频率，因此请检查数组大小
`V4L2_CID_AUDIO_LIMITER_ENABLED (boolean)`
    启用或禁用音频频偏限制器功能。当试图最大化音频音量、最小化接收机产生的
    失真并防止过调制时，限制器很有用
`V4L2_CID_AUDIO_LIMITER_RELEASE_TIME (integer)`
    设置音频频偏限制器功能的释放时间。单位为微秒。步进与范围由驱动决定
`V4L2_CID_AUDIO_LIMITER_DEVIATION (integer)`
    Hz 为单位配置音频频偏电平。范围与步进由驱动决定
`V4L2_CID_AUDIO_COMPRESSION_ENABLED (boolean)`
    启用或禁用音频压缩功能。该功能以固定增益放大低于阈值的信号，并    Threshold/(Gain + Threshold) 的比率压缩高于阈值的音频信号
`V4L2_CID_AUDIO_COMPRESSION_GAIN (integer)`
    设置音频压缩功能的增益。为 dB 值。范围与步进由驱动决定
`V4L2_CID_AUDIO_COMPRESSION_THRESHOLD (integer)`
    设置音频压缩功能的阈值电平。为 dB 值。范围与步进由驱动决定
`V4L2_CID_AUDIO_COMPRESSION_ATTACK_TIME (integer)`
    设置音频压缩功能的启动时间。为微秒值。范围与步进由驱动决定
`V4L2_CID_AUDIO_COMPRESSION_RELEASE_TIME (integer)`
    设置音频压缩功能的释放时间。为微秒值。范围与步进由驱动决定
`V4L2_CID_PILOT_TONE_ENABLED (boolean)`
    启用或禁用导频音生成功能
`V4L2_CID_PILOT_TONE_DEVIATION (integer)`
    配置导频音频偏电平。单位为 Hz。范围与步进由驱动决定
`V4L2_CID_PILOT_TONE_FREQUENCY (integer)`
    配置导频音频率值。单位为 Hz。范围与步进由驱动决定
`V4L2_CID_TUNE_PREEMPHASIS (enum)`
    配置用于广播的预加重值。对广播应用预加重滤波器以突出高频音频。根据地    不同，使50 75 微秒的时间常数。枚v4l2_preemphasis 定义了预加重    可能取值，如下
    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_PREEMPHASIS_DISABLED`
      - 不应用预加重    - - `V4L2_PREEMPHASIS_50_uS`
      - 使用 50 微秒的预加重    - - `V4L2_PREEMPHASIS_75_uS`
      - 使用 75 微秒的预加重
`V4L2_CID_TUNE_POWER_LEVEL (integer)`
    设置信号发射的输出功率电平。单位为 dBuV。范围与步进由驱动决定
`V4L2_CID_TUNE_ANTENNA_CAPACITOR (integer)`
    手动或（若设0）自动选择天线调谐电容的值。单位、范围与步进由驱动决定
有关 RDS 规范的更多细节，请参CENELEC iec62106 文档