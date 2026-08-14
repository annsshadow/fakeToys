


######## ioctl VIDIOC_ENCODER_CMD, VIDIOC_TRY_ENCODER_CMD


## 名称


VIDIOC_ENCODER_CMD - VIDIOC_TRY_ENCODER_CMD - 执行一条编码器命令

## 概要



`int ioctl(int fd, VIDIOC_ENCODER_CMD, struct v4l2_encoder_cmd *argp)`


`int ioctl(int fd, VIDIOC_TRY_ENCODER_CMD, struct v4l2_encoder_cmd *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_encoder_cmd` 的指针。

## 描述


这些 ioctl 控制一个音频/视频（通常是 MPEG-）编码器。`VIDIOC_ENCODER_CMD`
向编码器发送一条命令，`VIDIOC_TRY_ENCODER_CMD` 可用于在不实际执行的情况下
尝试一条命令。

要发送一条命令，应用程序必须初始化 struct `v4l2_encoder_cmd` 的所有字段，并
以指向该结构的指针调用 `VIDIOC_ENCODER_CMD` 或 `VIDIOC_TRY_ENCODER_CMD`。

`cmd` 字段必须包含命令码。某些命令使用 `flags` 字段来携带附加信息。

在 STOP 命令之后，`read()` 调用会读取驱动缓冲的剩余数据。当缓冲区为空时，
`read()` 将返回零，而下一次 `read()` 调用会重新启动编码器。

如果编码器尚未启动，一次 `read()` 或 VIDIOC_STREAMON <VIDIOC_STREAMON>
调用会向编码器发送一个隐式的 START 命令。适用于 mem2mem 编码器的两个队列。

对正在流式传输的文件描述符的一次 `close()` 或 VIDIOC_STREAMOFF <VIDIOC_STREAMON>
调用会向编码器发送一个隐式的立即 STOP，所有缓冲数据被丢弃。适用于 mem2mem
编码器的两个队列。

这些 ioctl 是可选的，并非所有驱动都可能支持它们。它们于 Linux 2.6.21 引入。
不过，对于有状态（stateful）mem2mem 编码器而言它们是强制的（如 encoder 中
进一步说明）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `cmd`
      - 编码器命令，参见 encoder-cmds。
    - - __u32
      - `flags`
      - 与命令配套的标志，参见 encoder-flags。如果未为该命令定义任何标志，
	驱动和应用程序必须将该字段置为零。
    - - __u32
      - `data`\ [^8^]
      - 为将来扩展保留。驱动和应用程序必须将该数组置为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_ENC_CMD_START`
      - 0
      - 启动编码器。当编码器已经在运行或已暂停时，该命令不执行任何操作。
	未为该命令定义任何标志。

	对于实现了编码器的设备，一旦通过 `V4L2_ENC_CMD_STOP` 命令启动了 drain
	序列，在该命令被调用之前必须将其驱动至完成。在 drain 序列进行期间任何
	调用该命令的尝试都会触发 `EBUSY` 错误码。详见 encoder。
    - - `V4L2_ENC_CMD_STOP`
      - 1
      - 停止编码器。当设置了 `V4L2_ENC_CMD_STOP_AT_GOP_END` 标志时，编码将
	持续到当前 *Group Of Pictures*（图像组）结束，否则编码将立即停止。当
	编码器已经停止时，该命令不执行任何操作。

	对于实现了编码器的设备，该命令将启动 encoder 中所述的 drain 序列。此时
	不接受任何标志或其它参数。在序列完成之前任何再次调用该命令的尝试都会
	触发 `EBUSY` 错误码。
    - - `V4L2_ENC_CMD_PAUSE`
      - 2
      - 暂停编码器。当编码器尚未启动时，驱动将返回 `EPERM` 错误码。当编码器
	已经暂停时，该命令不执行任何操作。未为该命令定义任何标志。
    - - `V4L2_ENC_CMD_RESUME`
      - 3
      - 在 PAUSE 命令之后恢复编码。当编码器尚未启动时，驱动将返回 `EPERM`
	错误码。当编码器已经在运行时，该命令不执行任何操作。未为该命令定义
	任何标志。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_ENC_CMD_STOP_AT_GOP_END`
      - 0x0001
      - 在当前 **Group Of Pictures**（图像组）结束时停止编码，而不是立即
	停止。

        不适用于编码器。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述。

EBUSY
    实现了编码器的设备的 drain 序列仍在进行中。在完成之前不允许发出另一条
    编码器命令。

EINVAL
    `cmd` 字段无效。

EPERM
    应用程序在编码器未运行时发送了 PAUSE 或 RESUME 命令。
