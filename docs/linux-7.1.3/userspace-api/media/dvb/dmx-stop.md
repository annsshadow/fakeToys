
## DMX_STOP


### 名称


DMX_STOP

### 概要


`int ioctl(int fd, DMX_STOP)`

### 参数


`fd`
    由 `open()` 返回的文件描述符。

### 描述


该 ioctl 调用用于停止通过 DMX_SET_FILTER 或 DMX_SET_PES_FILTER ioctl 调用
定义、并通过 DMX_START 命令启动的实际过滤操作。

### 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在“通用错误码”（Generic Error Codes）<gen-errors> 章节中描述。
