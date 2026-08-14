
######## ioctl VIDIOC_DQEVENT


## 名称


VIDIOC_DQEVENT - 出队（Dequeue）事件

## 概要


`int ioctl(int fd, VIDIOC_DQEVENT, struct v4l2_event *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_event` 的指针。

## 描述


从一个视频设备出队一个事件。这个 ioctl 不需要输入。struct `v4l2_event` 的所有字段
都由驱动填充。文件句柄还会收到异常，应用程序可以通过例如使用 select 系统调用来获取
这些异常。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 事件的类型，参见 event-type。
    - - union {
      - `u`
    - - struct `v4l2_event_vsync`
      - `vsync`
      - 事件 `V4L2_EVENT_VSYNC` 的事件数据。
    - - struct `v4l2_event_ctrl`
      - `ctrl`
      - 事件 `V4L2_EVENT_CTRL` 的事件数据。
    - - struct `v4l2_event_frame_sync`
      - `frame_sync`
      - 事件 `V4L2_EVENT_FRAME_SYNC` 的事件数据。
    - - struct `v4l2_event_motion_det`
      - `motion_det`
      - 事件 V4L2_EVENT_MOTION_DET 的事件数据。
    - - struct `v4l2_event_src_change`
      - `src_change`
      - 事件 V4L2_EVENT_SOURCE_CHANGE 的事件数据。
    - - __u8
      - `data`\ [^64^]
      - 事件数据。由事件类型定义。应当使用该联合体为事件定义易于访问的类型。
    - - }
      -
    - - __u32
      - `pending`
      - 除本事件外待处理事件的数量。
    - - __u32
      - `sequence`
      - 事件序列号。每发生一个已订阅的事件，序列号就递增。如果序列号不连续，意味着
	事件已经丢失。
    - - struct timespec
      - `timestamp`
      - 事件时间戳。时间戳取自 `CLOCK_MONOTONIC` 时钟。要在 V4L2 之外访问同一个时钟，
	请使用 `clock_gettime`。
    - - u32
      - `id`
      - 与事件源关联的 ID。如果事件没有关联的 ID（这取决于事件类型），那么这里是 0。
    - - __u32
      - `reserved`\ [^8^]
      - 为未来的扩展保留。驱动必须把该数组置为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_EVENT_ALL`
      - 0
      - 所有事件。V4L2_EVENT_ALL 仅对 VIDIOC_UNSUBSCRIBE_EVENT 有效，用于一次性退订
	所有事件。
    - - `V4L2_EVENT_VSYNC`
      - 1
      - 该事件在垂直同步（vertical sync）时触发。该事件关联了一个 struct
	`v4l2_event_vsync`。
    - - `V4L2_EVENT_EOS`
      - 2
      - 当到达流的末尾时触发该事件。这通常配合 MPEG 解码器使用，用来向应用程序报告
	MPEG 流的最后一部分已经被解码。
    - - `V4L2_EVENT_CTRL`
      - 3
      - 该事件要求 `id` 与你想要接收事件的控件的 ID 匹配。当控件的值改变、按钮控件
	被按下，或者控件的标志改变时，触发该事件。该事件关联了一个 struct
	`v4l2_event_ctrl`。该结构体包含与 struct
	v4l2_queryctrl <v4l2-queryctrl> 和 struct
	`v4l2_control` 基本相同的信息。

	如果该事件是由于调用 VIDIOC_S_CTRL <VIDIOC_G_CTRL> 或
	VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 而产生的，那么该事件将**不会**发送给
	调用该 ioctl 函数的文件句柄。这避免了恼人的反馈循环。如果你**确实**想要收到
	该事件，则设置 `V4L2_EVENT_SUB_FL_ALLOW_FEEDBACK` 标志。

	这种事件类型可以确保在内部空间不足、产生的事件多于可容纳数量时不会丢失信息。
	在那种情况下，第二旧事件的 struct `v4l2_event_ctrl` 会被保留，但其 `changes`
	字段会与最旧事件的 `changes` 字段做按位或运算。
    - - `V4L2_EVENT_FRAME_SYNC`
      - 4
      - 在帧的接收一开始时立即触发。该事件关联了一个 struct
	`v4l2_event_frame_sync`。

	如果硬件在缓冲区欠载（underrun）的情况下需要被停止，它可能就无法生成该事件。在
	这种情况下，struct `v4l2_event_frame_sync` 中的 `frame_sequence` 字段不会被递增。
	这会导致两个连续的帧序列号之间有 n 倍的帧间隔。
    - - `V4L2_EVENT_SOURCE_CHANGE`
      - 5
      - 当视频设备在运行时检测到源参数变化时触发该事件。它可以是视频解码器触发的
	运行时分辨率变化，或者是发生在某个输入连接器上的格式变化。该事件要求 `id` 与
	你想要接收事件的输入索引（用于视频设备节点时）或 pad 索引（用于子设备节点时）
	匹配。

	该事件关联了一个 struct
	`v4l2_event_src_change`。`changes` 位域表示所订阅的 pad 上发生了什么变化。如果
	在应用程序能够出队之前发生了多个事件，那么 changes 将具有所有已生成事件的按位
	或值。
    - - `V4L2_EVENT_MOTION_DET`
      - 6
      - 当一个或多个区域的运动检测状态发生变化时触发。该事件关联了一个 struct
	`v4l2_event_motion_det`。
    - - `V4L2_EVENT_PRIVATE_START`
      - 0x08000000
      - 驱动私有事件的基准事件号。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `field`
      - 即将到来的场。参见 enum `v4l2_field`。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `changes`
      - 一个位掩码，表示发生了什么变化。参见 ctrl-changes-flags。
    - - __u32
      - `type`
      - 控件的类型。参见 enum `v4l2_ctrl_type`。
    - - union {
      - (anonymous)
    - - __s32
      - `value`
      - 32 位控件类型的控件的 32 位值。对字符串控件这是 0，因为字符串的值无法通过
	VIDIOC_DQEVENT 传递。
    - - __s64
      - `value64`
      - 64 位控件类型的控件的 64 位值。
    - - }
      -
    - - __u32
      - `flags`
      - 控件标志。参见 control-flags。
    - - __s32
      - `minimum`
      - 控件的最小值。参见 struct v4l2_queryctrl <v4l2-queryctrl>。
    - - __s32
      - `maximum`
      - 控件的最大值。参见 struct v4l2_queryctrl <v4l2-queryctrl>。
    - - __s32
      - `step`
      - 控件的步进值。参见 struct v4l2_queryctrl <v4l2-queryctrl>。
    - - __s32
      - `default_value`
      - 控件的默认值。参见 struct v4l2_queryctrl <v4l2-queryctrl>。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `frame_sequence`
      - 正在接收的帧的序列号。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `changes`
      - 一个位掩码，表示发生了什么变化。参见 src-changes-flags。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `flags`
      - 目前只有一个标志可用：如果设置了 `V4L2_EVENT_MD_FL_HAVE_FRAME_SEQ`，那么
	`frame_sequence` 字段有效，否则应当忽略该字段。
    - - __u32
      - `frame_sequence`
      - 正在接收的帧的序列号。仅当 `V4L2_EVENT_MD_FL_HAVE_FRAME_SEQ` 标志被设置时有效。
    - - __u32
      - `region_mask`
      - 报告了运动的区域的位掩码。至少有一个区域。如果该字段为 0，则根本未检测到
	运动。如果没有 `V4L2_CID_DETECT_MD_REGION_GRID` 控件（见 detect-controls）来为
	运动检测网格中的每个单元分配不同的区域，那么所有单元都会自动被分配到默认
	区域 0。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_EVENT_CTRL_CH_VALUE`
      - 0x0001
      - 该控件事件是因控件的值改变而触发的。特殊情况：易变（Volatile）控件不会产生
	该事件；如果一个控件设置了 `V4L2_CTRL_FLAG_EXECUTE_ON_WRITE` 标志，那么无论其值
	如何，也会发送该事件。
    - - `V4L2_EVENT_CTRL_CH_FLAGS`
      - 0x0002
      - 该控件事件是因控件标志改变而触发的。
    - - `V4L2_EVENT_CTRL_CH_RANGE`
      - 0x0004
      - 该控件事件是因控件的最小值、最大值、步进或默认值改变而触发的。
    - - `V4L2_EVENT_CTRL_CH_DIMENSIONS`
      - 0x0008
      - 该控件事件是因控件的维度改变而触发的。注意维度的数量保持不变。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_EVENT_SRC_CH_RESOLUTION`
      - 0x0001
      - 当在输入上检测到分辨率变化时触发该事件。这可以来自输入连接器，也可以来自
	视频解码器。应用程序将不得不查询新的分辨率（如果有的话；信号也可能已经丢失）。

	对于有状态（stateful）解码器，请遵循 decoder 中的指南。视频采集设备必须使用
	VIDIOC_QUERY_DV_TIMINGS 或
	VIDIOC_QUERYSTD <VIDIOC_QUERYSTD> 查询新的时序。

	**重要**：即使新的视频时序看起来与旧的相同，收到该事件也表明视频信号出现过问题，
	你必须停止并重新启动流（先 VIDIOC_STREAMOFF <VIDIOC_STREAMON>，再
	VIDIOC_STREAMON <VIDIOC_STREAMON>）。原因是许多视频采集设备无法从信号的临时
	丢失中恢复，因此为了硬件能与视频信号重新同步，需要重启流 I/O。

## 返回值


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中有描述。
