## CA_SET_DESCR


### Name


CA_SET_DESCR

### Synopsis



`int ioctl(fd, CA_SET_DESCR, struct ca_descr *desc)`

### Arguments


`fd`
  由先前 `open()` 调用返回的文件描述符。

`msg`
  指向 struct `ca_descr` 的指针。

### Description


CA_SET_DESCR 用于向解扰器 CA 插槽提供解扰密钥（称为控制字）。

### Return Value


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
