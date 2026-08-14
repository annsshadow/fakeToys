


######## ioctl VIDIOC_QUERYSTD, VIDIOC_SUBDEV_QUERYSTD


## 名称


VIDIOC_QUERYSTD - VIDIOC_SUBDEV_QUERYSTD - 检测当前输入所接收到的视频标准

## 概要


`int ioctl(int fd, VIDIOC_QUERYSTD, v4l2_std_id *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_QUERYSTD, v4l2_std_id *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 `v4l2_std_id` 的指针。

## 描述


硬件可能能够自动检测当前的视频标准。为此，应用程序调用 VIDIOC_QUERYSTD，并传入一个指向 v4l2_std_id <v4l2-std-id> 类型的指针。驱动在此处存储一组候选标准，这可以是一个单独的标志，也可以是一组受支持的标准（例如，如果硬件只能区分 50 Hz 和 60 Hz 系统）。如果没有检测到信号，驱动将返回 V4L2_STD_UNKNOWN。当无法检测或检测失败时，该集合必须包含当前视频输入或输出所支持的所有标准。


   驱动**不应当**在检测到新的视频标准时自动切换视频标准。相反，驱动应当发送 `V4L2_EVENT_SOURCE_CHANGE` 事件（如果它们支持的话），并期望用户空间通过调用 VIDIOC_QUERYSTD 来采取行动。原因是新的视频标准也可能意味着不同的缓冲区大小，而你无法在运行时更改缓冲区大小。一般来说，收到源变更（Source Change）事件的应用程序将不得不调用 VIDIOC_QUERYSTD，如果检测到的视频标准有效，它们将不得不停止流传输、设置新的标准、分配新的缓冲区并再次开始流传输。

## 返回值


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述。

ENODATA
    此输入或输出不支持标准视频时序。
