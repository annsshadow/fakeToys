######## ioctl FE_DISEQC_RECV_SLAVE_REPLY


## 名称


FE_DISEQC_RECV_SLAVE_REPLY - 接收来自 DiSEqC 2.0 命令的回

## 摘要



`int ioctl(int fd, FE_DISEQC_RECV_SLAVE_REPLY, struct dvb_diseqc_slave_reply *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符

`argp`
    指向结构`dvb_diseqc_slave_reply` 的指针

## 说明


接收来自 DiSEqC 2.0 命令的回复

接收到的消息存储`argp` 指向的缓冲区中

## 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
