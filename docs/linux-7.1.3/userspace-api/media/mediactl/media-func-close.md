
######## media close()

## 名称


media-close - 关闭一个 media 设备

## 概要



    #include <unistd.h>


## 参数


`fd`
    由 `open()` 返回的文件描述符。

## 描述


关闭 media 设备。与文件描述符关联的资源被释放。设备配置保持不变。

## 返回值


`close()` 成功时返回 0。出错时返回 -1，并相应地设置 `errno`。可能的错误码包括：

EBADF
    `fd` 不是有效的已打开文件描述符。
