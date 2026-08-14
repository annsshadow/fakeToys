## DMX_SET_FILTER


### Name


DMX_SET_FILTER

### Synopsis


`int ioctl(int fd, DMX_SET_FILTER, struct dmx_sct_filter_params *params)`

### Arguments


`fd`
    `open()` 返回的文件描述符。

`params`
    指向包含过滤参数的结构体的指针。

### Description


该 ioctl 调用根据所提供的过滤器和掩码参数设置一个过滤器。可以定义一个超时时间，表示等待某个段（section）被加载的秒数。值为 0 表示不应用超时。最后还有一个标志字段，可用于指明某个段是否应进行 CRC 校验、该过滤器是否应为“一次性（one-shot）”过滤器（即是否在接收到第一个段后停止过滤操作），以及过滤操作是否应立即开始（无需等待 DMX_START ioctl 调用）。如果之前已经设置了一个过滤器，则该过滤器将被取消，接收缓冲区也会被清空。

### Return Value


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
