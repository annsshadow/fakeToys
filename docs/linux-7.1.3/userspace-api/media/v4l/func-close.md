######## V4L2 close()


## Name


v4l2-close - 关闭一个 V4L2 设备

## Synopsis



    #include <unistd.h>


## Arguments


`fd`
    由 `open()` 返回的文件描述符。

## Description


关闭设备。任何正在进行的 I/O 都会终止，并与该文件描述符关联的资源被释放。但数据格式参数、当前输入或输出、控制值或其他属性保持不变。

## Return Value


函数成功时返回 0，失败时返回 -1，并相应地设置 `errno`。可能的错误码：

EBADF
    `fd` 不是一个有效的已打开文件描述符。
