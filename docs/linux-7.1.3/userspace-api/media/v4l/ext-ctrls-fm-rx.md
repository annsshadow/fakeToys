


######## FM 接收器（FM Receiver）控制参

FM 接收器（FM_RX）类包含用于具备 FM 接收能力设备的通用特性的控制

## FM_RX 控制 ID


`V4L2_CID_FM_RX_CLASS (class)`
    FM_RX 类描述符。对该控制调VIDIOC_QUERYCTRL 将返回该控制类的描述
`V4L2_CID_RDS_RECEPTION (boolean)`
    启用/禁用无线电调谐器RDS 接收

`V4L2_CID_RDS_RX_PTY (integer)`
    获取 RDS 节目类型（Programme Type）字段。它编码最31 种预定义的节目类型
`V4L2_CID_RDS_RX_PS_NAME (string)`
    获取节目服务名称（PS_NAME）。它用于在接收器上静态显示。它是听众识别与选择节目服务的主要辅助手段。在 RDS 规范 iec62106 的附E 中，有关于节目服务名称字符串正确字符编码的完整描述。同样根RDS 规范，PS 通常是一8 字符的文本。不过，也可以找到能够滚动显示大小为 8 x N 字符的字符串的接收器。因此，该控制必须以 8 个字符为步进配置。结果是它必须始终包含一个大小为 8 的倍数的字符串
`V4L2_CID_RDS_RX_RADIO_TEXT (string)`
    获取 Radio Text 信息。它是对正在广播内容的文本描述。当广播者希望传输更长的 PS 名称、节目相关信息或任何其他文本时，可以使用 RDS Radio Text。在这些情况下，RadioText 可与 `V4L2_CID_RDS_RX_PS_NAME` 配合使用。Radio Text 字符串的编码也在 iec62106 的附E 中完整描述。Radio Text 字符串的长度取决于用于传输它RDS 块，32A 块）64B 块）。不过，也可以找到能够滚动显示大小为 32 x N 64 x N 字符的字符串的接收器。因此，该控制必须以 32 64 个字符为步进配置。结果是它必须始终包含一个大小为 32 64 的倍数的字符串
`V4L2_CID_RDS_RX_TRAFFIC_ANNOUNCEMENT (boolean)`
    如果设置，则表示正在进行交通公告（traffic announcement）
`V4L2_CID_RDS_RX_TRAFFIC_PROGRAM (boolean)`
    如果设置，则表示所调谐的节目携带交通公告
`V4L2_CID_RDS_RX_MUSIC_SPEECH (boolean)`
    如果设置，则该频道广播音乐。如果清除，则广播语音。如果发射器不做此区分，则将被设置
`V4L2_CID_TUNE_DEEMPHASIS (enum)`
    配置接收的去加重（de-emphasis）值。去加重滤波器应用于广播以突出高频音频。根据地区，使用 50 75 微秒的时间常数。枚v4l2_deemphasis 定义了去加重的可能值。它们是
    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_DEEMPHASIS_DISABLED`
      - 不应用去加重    - - `V4L2_DEEMPHASIS_50_uS`
      - 使用 50 微秒的去加重    - - `V4L2_DEEMPHASIS_75_uS`
      - 使用 75 微秒的去加重