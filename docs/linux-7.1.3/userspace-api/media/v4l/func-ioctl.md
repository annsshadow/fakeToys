
######## V4L2 ioctl()


## 名称


v4l2-ioctl - 编程 V4L2 设备

## 概要


    #include <sys/ioctl.h>

`int ioctl(int fd, int request, void *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符。

`request`
    在 `videodev2.h` 头文件中定义的 V4L2 ioctl 请求码，例如
    VIDIOC_QUERYCAP。

`argp`
    指向函数参数的指针，通常是一个结构。

## 描述


ioctl() <func-ioctl> 函数用于编程 V4L2 设备。参数 `fd` 必须是一个已打开的
文件描述符。ioctl `request` 中编码了参数是输入、输出还是读写参数，以及
参数 `argp` 的大小（字节数）。指定 V4L2 ioctl 请求的宏与 define 位于
`videodev2.h` 头文件中。应用程序应使用自己的副本，而非包含其编译所在系统
内核源码中的版本。所有 V4L2 ioctl 请求及其各自的函数与参数在 user-func 中
说明。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

当采用输出或读写参数的 ioctl 失败时，该参数保持不变。
