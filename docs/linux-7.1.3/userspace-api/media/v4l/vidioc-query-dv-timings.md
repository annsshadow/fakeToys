######## ioctl VIDIOC_QUERY_DV_TIMINGS


## 名称


VIDIOC_QUERY_DV_TIMINGS - VIDIOC_SUBDEV_QUERY_DV_TIMINGS - 检测当前输入收到的 DV 预设

## 摘要


`int ioctl(int fd, VIDIOC_QUERY_DV_TIMINGS, struct v4l2_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_QUERY_DV_TIMINGS, struct v4l2_dv_timings *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_dv_timings` 的指针。

## 描述


硬件可能能够自动检测当前的 DV 时序，类似于检测视频标准。为此，应用程序以指向
一个 struct `v4l2_dv_timings` 的指针调用 VIDIOC_QUERY_DV_TIMINGS。一旦硬件检测到
时序，它将填充该时序结构。


   驱动**不得**在检测到新时序时自动切换时序。相反，驱动应发送
   `V4L2_EVENT_SOURCE_CHANGE` 事件（如果它们支持），并期望用户空间通过调用
   VIDIOC_QUERY_DV_TIMINGS 来采取行动。原因是新时序通常也意味着不同的缓冲区大小，
   而你无法在运行时更改缓冲区大小。一般而言，接收到 Source Change 事件的应用程序
   必须调用 VIDIOC_QUERY_DV_TIMINGS，如果检测到的时序有效，则必须停止流传输、设置
   新时序、分配新缓冲区并重新启动流传输。

如果因为无信号而无法检测时序，则返回 ENOLINK。如果检测到信号，但它不稳定且接收器
无法锁定到该信号，则返回 `ENOLCK`。如果接收器能够锁定到信号，但格式不受支持（例如
因为像素时钟超出硬件能力范围），则驱动填充它所能找到的任意时序并返回 `ERANGE`。在
该情况下，应用程序可以调用 VIDIOC_DV_TIMINGS_CAP，将找到的时序与硬件能力进行比较，
以便向用户提供更具反馈。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

ENODATA
    该输入或输出不支持数字视频时序。

ENOLINK
    因为未找到信号，无法检测到任何时序。

ENOLCK
    信号不稳定，硬件无法锁定到它。

ERANGE
    找到了时序，但它们超出了硬件能力范围。
