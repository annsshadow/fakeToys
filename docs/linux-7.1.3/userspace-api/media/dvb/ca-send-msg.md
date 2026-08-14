## CA_SEND_MSG


### Name


CA_SEND_MSG

### Synopsis



`int ioctl(fd, CA_SEND_MSG, struct ca_msg *msg)`

### Arguments


`fd`
  由先前 `open()` 调用返回的文件描述符。

`msg`
  指向 struct `ca_msg` 的指针。

### Description


通过 CI CA 模块发送一条消息。


   请注意，在大多数驱动上，这是通过写入 /dev/adapter?/ca? 设备节点完成的。

### Return Value


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
