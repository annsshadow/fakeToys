


######## cec ioctl()


## 名称


cec-ioctl - 控制 cec 设备

## 概要


    #include <sys/ioctl.h>

`int ioctl(int fd, int request, void *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符。

`request`
    在 cec.h 头文件中定义的 CEC ioctl 请求码，例如
    CEC_ADAP_G_CAPS <CEC_ADAP_G_CAPS>。

`argp`
    指向请求特定结构的指针。

## 描述


`ioctl()` 函数操纵 cec 设备参数。参数 `fd` 必须是一个已打开的文件描述符。

ioctl `request` 码指定要调用的 cec 函数。其中编码了参数是输入、输出还是
读写参数，以及参数 `argp` 的大小（字节数）。

指定 cec ioctl 请求及其参数的宏与结构定义位于 cec.h 头文件中。所有 cec
ioctl 请求及其各自的函数与参数在 cec-user-func 中说明。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

请求特定的错误码在各请求的描述中列出。

当采用输出或读写参数的 ioctl 失败时，该参数保持不变。
