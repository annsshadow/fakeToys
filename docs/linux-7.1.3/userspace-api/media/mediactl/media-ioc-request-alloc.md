
######## ioctl MEDIA_IOC_REQUEST_ALLOC


## 名称


MEDIA_IOC_REQUEST_ALLOC - 分配一个请

## 概要


`int ioctl(int fd, MEDIA_IOC_REQUEST_ALLOC, int *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符

`argp`
    指向一个整数的指针

## 描述


如果媒体设备支持请求 <media-request-api>，则
ioctl 可用于分配一个请求。如果不支持，则
`errno` 被设`ENOTTY`。请求通过一个文件描述符访问，该描述
`*argp` 中返回

如果请求成功分配，则该请求文件描述符可以被传递给
VIDIOC_QBUF <VIDIOC_QBUF>、VIDIOC_G_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>、VIDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 以及
VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> ioctl銆。

此外，可通过调用 MEDIA_REQUEST_IOC_QUEUE 将该请求入队，并通过调用
MEDIA_REQUEST_IOC_REINIT 重新初始化

最后，可以对该文件描述符执poll <request-func-poll> 以等
请求完成

该请求将一直保持分配状态，直到与之关联的所有文件描述符都被 `close()` 关闭，且驱动内部
不再使用该请求。更多信息请参见
此处 <media-request-life-time>

## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述

ENOTTY
    驱动不支持请求
