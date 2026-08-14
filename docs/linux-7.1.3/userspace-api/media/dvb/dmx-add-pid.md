## DMX_ADD_PID


### Name


DMX_ADD_PID

### Synopsis



`int ioctl(fd, DMX_ADD_PID, __u16 *pid)`

### Arguments


`fd`
    由 `open()` 返回的文件描述符。

`pid`
   要过滤的 PID 编号。

### Description


该 ioctl 调用可将多个 PID 添加到先前通过 DMX_SET_PES_FILTER 设置、且输出等于 `DMX_OUT_TSDEMUX_TAP <dmx_output>` 的传输流过滤器中。

### Return Value


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
