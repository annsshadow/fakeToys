## DMX_SET_BUFFER_SIZE


### 名称


DMX_SET_BUFFER_SIZE

### 摘要



`int ioctl(int fd, DMX_SET_BUFFER_SIZE, unsigned long size)`

### 参数


`fd`
    `open()` 返回的文件描述符

`size`
    无符号长整型size

### 说明


ioctl 调用用于设置用于过滤数据的环形缓冲区的大小。默认大小为两个最大尺寸的段，即如果不调用此函数，将使`2 * 4096` 字节的缓冲区大小

### 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
