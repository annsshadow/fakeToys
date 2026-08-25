


######## ioctl VIDIOC_DECODER_CMD, VIDIOC_TRY_DECODER_CMD


## Name


VIDIOC_DECODER_CMD - VIDIOC_TRY_DECODER_CMD - 执行一条解码器命令

## Synopsis



`int ioctl(int fd, VIDIOC_DECODER_CMD, struct v4l2_decoder_cmd *argp)`


`int ioctl(int fd, VIDIOC_TRY_DECODER_CMD, struct v4l2_decoder_cmd *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_decoder_cmd` 的指针
## Description


这些 ioctl 用于控制音频/视频（通常MPEG-）解码器。`VIDIOC_DECODER_CMD` 向解码器发送一条命令，`VIDIOC_TRY_DECODER_CMD` 可用于在不实际执行的情况下尝试一条命令。要发送命令，应用程序必须初始struct `v4l2_decoder_cmd` 的所有字段，并以指向该结构的指针调用 `VIDIOC_DECODER_CMD` `VIDIOC_TRY_DECODER_CMD`
`cmd` 字段必须包含命令码。某些命令使`flags` 字段携带附加信息
如果解码器尚未启动，`write()` VIDIOC_STREAMON 调用会向解码器发送一个隐式的 START 命令。适用mem2mem 解码器的两个队列
对正在流式传输的文件描述符调`close()` VIDIOC_STREAMOFF 会向解码器发送一个隐式的立即 STOP 命令，并且所有缓冲数据会被丢弃。适用mem2mem 解码器的两个队列
原则上，这些 ioctl 是可选的，并非所有驱动都可能支持它们。它们是Linux 3.3 中引入的。然而，对于有状态（stateful）的 mem2mem 解码器，它们是强制要求的（详decoder 文档）


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 1 1 3

    - - __u32
      - `cmd`
      -
      - 解码器命令，参见 decoder-cmds    - - __u32
      - `flags`
      -
      - 与该命令一同使用的标志。如果没有为该命令定义标志，驱动和应用程序必须将本字段设0    - - union {
      - (anonymous)
    - - struct
      - `start`
      -
      - 包含 `V4L2_DEC_CMD_START` 命令附加数据的结构    - -
      - __s32
      - `speed`
      - 播放速度和方向。播放速度定义为正常速度`speed`/1000。因1000 为正常播放。负数表示反向播放，因此 -1000 表示以正常速度反向播放。速度 -1 1 有特殊含义：速度 0 1000（正常播放）的简写。速度 1 表示仅前进一帧，速度 -1 表示仅后退一帧    - -
      - __u32
      - `format`
      - 格式限制。本字段由驱动设置，而非应用程序。如果没有格式限制，可能的值为 `V4L2_DEC_START_FMT_NONE`；如果解码器以完GOP*Group Of Pictures**，图像组）为单位操作，则`V4L2_DEC_START_FMT_GOP`。反向播放通常就是这种情况：解码器需要完整的 GOP，然后才能按相反顺序播放。因此要实现反向播放，应用程序必须向解码器喂入视频文件中的最后一GOP，然后是前一GOP，依此类推    - - struct
      - `stop`
      -
      - 包含 `V4L2_DEC_CMD_STOP` 命令附加数据的结构    - -
      - __u64
      - `pts`
      - 在该 `pts` 处停止播放，或者如果播放已经超过该时间戳则立即停止。如果希望在解码完最后一帧后停止，则保留0    - - struct
      - `raw`
    - -
      - __u32
      - `data`\ [^16^]
      - 保留供将来扩展使用。驱动和应用程序必须将该数组设为 0    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths: 56 6 113

    - - `V4L2_DEC_CMD_START`
      - 0
      - 启动解码器。当解码器已经在运行或暂停时，此命令只会改变播放速度。这意味着当解码器处于暂停状态时调用 `V4L2_DEC_CMD_START` **不会**恢复解码器。为此你必须显式调用 `V4L2_DEC_CMD_RESUME`。此命令有一个标志：`V4L2_DEC_CMD_START_MUTE_AUDIO`。如果设置该标志，则在不以标准速度播放时会静音。对于实现了该解码器的设备，一旦通过 `V4L2_DEC_CMD_STOP` 命令启动drain 序列，就必须驱动其完成，然后才能调用此命令。在 drain 序列进行期间任何调用此命令的尝试都会触发 `EBUSY` 错误码。如果解码器自身发起了隐式停止（未显式调`V4L2_DEC_CMD_STOP`），此命令也可用于重启解码器。详decoder    - - `V4L2_DEC_CMD_STOP`
      - 1
      - 停止解码器。当解码器已经停止时，此命令不执行任何操作。此命令有两个标志：如果设置`V4L2_DEC_CMD_STOP_TO_BLACK`，则解码器在停止解码后会将画面置为黑色。否则最后一帧图像会重复显示。如果设置了 `V4L2_DEC_CMD_STOP_IMMEDIATELY`，则解码器立即停止（忽略 `pts` 值），否则它会持续解码，直到时间>= pts，或者直到其内部缓冲区中待处理数据的最后一帧被解码完毕。对于实现了该解码器的设备，此命令会启动decoder 文档中所描述drain 序列。这种情况下不接受任何标志或其他参数。在序列完成前任何再次调用此命令的尝试都会触`EBUSY` 错误码    - - `V4L2_DEC_CMD_PAUSE`
      - 2
      - 暂停解码器。当解码器尚未启动时，驱动将返回 `EPERM` 错误码。当解码器已经暂停时，此命令不执行任何操作。此命令有一个标志：如果设置`V4L2_DEC_CMD_PAUSE_TO_BLACK`，则在暂停时将解码器输出置为黑色    - - `V4L2_DEC_CMD_RESUME`
      - 3
      - PAUSE 命令之后恢复解码。当解码器尚未启动时，驱动将返回 `EPERM` 错误码。当解码器已经在运行时，此命令不执行任何操作。此命令未定义任何标志    - - `V4L2_DEC_CMD_FLUSH`
      - 4
      - 刷新所有被持有的捕获缓冲区。仅对无状态（stateless）解码器有效。当应用程序到达流末尾，且最后一个输出缓冲区设置`V4L2_BUF_FLAG_M2M_HOLD_CAPTURE_BUF` 标志时，通常会使用此命令。这会阻止取出包含最后一帧已解码画面的捕获缓冲区。因此此命令可用于显式刷新那一帧最终的已解码画面。如果没有被持有的捕获缓冲区，此命令不执行任何操作
## Return Value


成功时返0，出错时返回 -1 并适当地设`errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EBUSY
    实现了该解码器的设备drain 序列仍在进行中。在它完成之前不允许发出另一条解码器命令
EINVAL
    `cmd` 字段无效
EPERM
    应用程序在解码器未运行时发送了 PAUSE RESUME 命令