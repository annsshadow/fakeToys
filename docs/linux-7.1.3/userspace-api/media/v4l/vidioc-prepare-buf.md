

######## ioctl VIDEO_PREPARE_BUF


## 姓名


VIDIOC_PREPARE_BUF - 为 I/O 准备缓冲区

## 概要



`int ioctl(int fd, VIDIOC_PREPARE_BUF, struct v4l2_buffer *argp)`

## 论点


`fd`
`open()`返回的文件描述符。

`argp`
指向结构体`v4l2_buffer`的指针。

## 描述


应用程序可以选择调用 VIDIOC_PREPARE_BUF ioctl
在实际将缓冲区排队之前将缓冲区的所有权传递给驱动程序，
使用 VIDIOC_QBUF <VIDIOC_QBUF> ioctl，并为将来的 I/O 做好准备。这样的
准备工作可能包括缓存失效或清理。表演它们
提前节省实际 I/O 期间的时间。

struct `v4l2_buffer` 结构体在中指定
缓冲。

## 返回值


成功时返回 0，错误时返回 -1 并且设置 `errno` 变量
适当地。通用错误代码的描述见
通用错误代码 <gen-errors> 章节。

忙碌的
文件 I/O 正在进行中。

单项选择
缓冲区`type`不受支持，或者`index`超出范围
边界，或者尚未分配缓冲区，或者 `userptr` 或
`length`无效。
