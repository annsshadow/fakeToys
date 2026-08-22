
######## request ioctl()


## 名称


request-ioctl - 控制请求文件描述

## 概要


    #include <sys/ioctl.h>

`int ioctl(int fd, int cmd, void *argp)`

## 参数


`fd`
    MEDIA_IOC_REQUEST_ALLOC 返回的文件描述符

`cmd`
    请求 ioctl 命令代码，定义在 media.h 头文件中，例MEDIA_REQUEST_IOC_QUEUE

`argp`
    指向请求特定结构的指针

## 描述


ioctl() <request-func-ioctl> 函数操纵请求参数。参`fd` 必须是已打开的文件描述符

ioctl `cmd` 代码指定要调用的请求函数。它编码了参数是输入、输出还是读/写参数，以及参数 `argp` 的大小（以字节为单位）

指定请求 ioctl 命令及其参数的宏和结构定义位media.h 头文件中。所有请ioctl 命令、各自的函数和参数都media-user-func 中指定

## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述

特定于命令的错误码列在各个命令的描述中

当带有输出或写参数的 ioctl 失败时，该参数保持不变
