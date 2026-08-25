## CA_GET_DESCR_INFO


### Name


CA_GET_DESCR_INFO

### Synopsis



`int ioctl(fd, CA_GET_DESCR_INFO, struct ca_descr_info *desc)`

### Arguments


`fd`
  由先`open()` 调用返回的文件描述符

`desc`
  指向 struct `ca_descr_info` 的指针

### Description


返回有关所有解扰器插槽的信息

### Return Value


成功时返0，并填充 `ca_descr_info`

出错时返-1，并相应地设`errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
