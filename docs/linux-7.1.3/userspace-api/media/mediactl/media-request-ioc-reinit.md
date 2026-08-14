


######## ioctl MEDIA_REQUEST_IOC_REINIT


## 名称


MEDIA_REQUEST_IOC_REINIT - 重新初始化一个请求

## 概要



`int ioctl(int request_fd, MEDIA_REQUEST_IOC_REINIT)`

## 参数


`request_fd`
    由 MEDIA_IOC_REQUEST_ALLOC 返回的文件描述符。

## 描述


如果媒体设备支持请求 <media-request-api>，则可以使用此请求 ioctl 来
重新初始化先前已分配的请求。

重新初始化请求将清除请求中的任何现有数据。这样就无需 `close()` 一个
已完成的请求并分配一个新请求。相反，已完成的请求只需重新初始化即可
再次使用。

只有在请求尚未排队，或者已排队且已完成的情况下，才能重新初始化请求。
否则它会将 `errno` 设置为 `EBUSY`。不会返回其他错误码。

## 返回值


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。

EBUSY
    请求已排队但尚未完成。
