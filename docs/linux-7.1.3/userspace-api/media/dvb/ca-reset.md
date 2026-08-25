
## CA_RESET


### 名称


CA_RESET

### 概要


`int ioctl(fd, CA_RESET)`

### 参数


`fd`
  由之前对 `open()` 的调用返回的文件描述符

### 描述


将条件接收（Conditional Access）硬件置于初始状态。应在开始使CA 硬件之前调用

### 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

通用错误码在“通用错误码”（Generic Error Codesgen-errors> 章节中描述
