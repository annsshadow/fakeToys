


######## ioctl VIDIOC_OVERLAY


## 名称


VIDIOC_OVERLAY - 启动或停止视频叠加

## 概要



`int ioctl(int fd, VIDIOC_OVERLAY, const int *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向一个整数的指针。

## 描述


此 ioctl 是视频叠加 <overlay> I/O 方法的一部分。应用程序调用 VIDIOC_OVERLAY
以启动或停止叠加。它接受一个指向整数的指针，应用程序必须将其设置为零以
停止叠加，设置为 1 以启动。

驱动程序不支持将 VIDIOC_STREAMON 或 VIDIOC_STREAMOFF <VIDIOC_STREAMON>
与 `V4L2_BUF_TYPE_VIDEO_OVERLAY` 一起使用。

## 返回值


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用错误码的描述
见通用错误码 <gen-errors> 章节。

EINVAL
    叠加参数尚未设置。有关必要步骤，请参阅 overlay。
