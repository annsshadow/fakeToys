## DMX_REMOVE_PID


### 名称


DMX_REMOVE_PID

### 摘要



`int ioctl(fd, DMX_REMOVE_PID, __u16 *pid)`

### 参数


`fd`
    `open()` 返回的文件描述符

`pid`
    要移除的 PES 过滤器的 PID

### 说明


当一个传输流过滤器上设置了多PID 时，ioctl 调用允许移除某个 PID，例如之前通过 DMX_SET_PES_FILTER DMX_ADD_PID 创建、且输出等于 `DMX_OUT_TSDEMUX_TAP <dmx_output>` 的过滤器

### 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
