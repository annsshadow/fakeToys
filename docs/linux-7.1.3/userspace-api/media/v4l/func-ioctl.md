
######## V4L2 ioctl()


## 名称


v4l2-ioctl - 编程 V4L2 设备

## 概要


    #include <sys/ioctl.h>

`int ioctl(int fd, int request, void *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`request`
    `videodev2.h` 头文件中定义V4L2 ioctl 请求码，例如
    VIDIOC_QUERYCAP銆。
`argp`
    指向函数参数的指针，通常是一个结构
## 描述


ioctl() <func-ioctl> 函数用于编程 V4L2 设备。参`fd` 必须是一个已打开文件描述符。ioctl `request` 中编码了参数是输入、输出还是读写参数，以及
参数 `argp` 的大小（字节数）。指V4L2 ioctl 请求的宏define 位于
`videodev2.h` 头文件中。应用程序应使用自己的副本，而非包含其编译所在系内核源码中的版本。所V4L2 ioctl 请求及其各自的函数与参数user-func 说明
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
当采用输出或读写参数ioctl 失败时，该参数保持不变