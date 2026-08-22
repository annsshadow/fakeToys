######## FE_READ_SIGNAL_STRENGTH


## 名称


FE_READ_SIGNAL_STRENGTH


## 摘要



`int ioctl(int fd, FE_READ_SIGNAL_STRENGTH, uint16_t *strength)`

## 参数


`fd`
    `open()` 返回的文件描述符

`strength`
    信号强度值被存入 \*strength

## 说明


ioctl 调用返回前端当前接收信号的信号强度值。对于该命令，对设备的只读访问即已足够

## 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
