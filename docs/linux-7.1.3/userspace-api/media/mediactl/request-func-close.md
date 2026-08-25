######## request close()


## 名称


request-close - 关闭请求文件描述

## 摘要



    #include <unistd.h>


## 参数


`fd`
    MEDIA_IOC_REQUEST_ALLOC 返回的文件描述符

## 说明


关闭请求文件描述符。一旦与该请求关联的所有文件描述符都已关闭，且驱动已完成该请求，与该请求关联的资源即被释放
更多信息请参<media-request-life-time>

## 杩斿洖鍊。


`close()` 成功时返0。出错时返回 -1，并相应地设`errno`。可能的错误码如下：

EBADF
    `fd` 不是有效的已打开文件描述符
