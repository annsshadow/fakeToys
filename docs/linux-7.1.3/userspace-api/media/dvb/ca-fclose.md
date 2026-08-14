
## 数字电视 CA 关闭（close()）


### 名称


Digital TV CA close()

### 概要



### 参数


`fd`
  由之前对 `open()` 的调用返回的文件描述符。

### 描述


该系统调用关闭之前打开的 CA 设备。

### 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在“通用错误码”（Generic Error Codes）<gen-errors> 章节中描述。
