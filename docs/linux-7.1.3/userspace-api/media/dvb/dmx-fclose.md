
## 数字电视 demux 关闭（close()


### 名称


Digital TV demux close()

### 概要



### 参数


`fd`
  由之前对 `open()` 的调用返回的文件描述符

### 描述


该系统调用停用并释放之前通过 `open()` 调用分配的过滤器

### 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

通用错误码在“通用错误码”（Generic Error Codesgen-errors> 章节中描述
