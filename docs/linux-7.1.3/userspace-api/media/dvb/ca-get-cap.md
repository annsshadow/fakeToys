## CA_GET_CAP


### Name


CA_GET_CAP

### Synopsis



`int ioctl(fd, CA_GET_CAP, struct ca_caps *caps)`

### Arguments


`fd`
  由先前 `open()` 调用返回的文件描述符。

`caps`
  指向 struct `ca_caps` 的指针。

### Description


向内核查询有关可用 CA 和解扰器插槽及其类型的信息。

### Return Value


成功时返回 0 并填充 `ca_caps`。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
