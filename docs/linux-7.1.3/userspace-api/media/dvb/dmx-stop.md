
## DMX_STOP


### 名称


DMX_STOP

### 概要


`int ioctl(int fd, DMX_STOP)`

### 参数


`fd`
    `open()` 返回的文件描述符

### 描述


ioctl 调用用于停止通过 DMX_SET_FILTER DMX_SET_PES_FILTER ioctl 调用
定义、并通过 DMX_START 命令启动的实际过滤操作

### 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

通用错误码在“通用错误码”（Generic Error Codesgen-errors> 章节中描述
