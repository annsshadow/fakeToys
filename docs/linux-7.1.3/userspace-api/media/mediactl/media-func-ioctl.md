
######## media ioctl()


## 名称


media-ioctl - 控制媒体设备

## 概要


    #include <sys/ioctl.h>

`int ioctl(int fd, int request, void *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`request`
    媒体 ioctl 请求代码，定义在 media.h 头文件中，例如 MEDIA_IOC_SETUP_LINK。

`argp`
    指向请求特定结构的指针。

## 描述


ioctl() <media-func-ioctl> 函数操纵媒体设备参数。参数 `fd` 必须是已打开的文件描述符。

ioctl `request` 代码指定要调用的媒体函数。它编码了参数是输入、输出还是读/写参数，以及参数 `argp` 的大小（以字节为单位）。

指定媒体 ioctl 请求及其参数的宏和结构定义位于 media.h 头文件中。所有媒体 ioctl 请求、各自的函数和参数都在 media-user-func 中指定。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述。

特定于请求的错误码列在各个请求的描述中。

当带有输出或读/写参数的 ioctl 失败时，该参数保持不变。
