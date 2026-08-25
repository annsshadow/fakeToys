######## ioctl FE_DISEQC_SEND_MASTER_CMD


## 名称


FE_DISEQC_SEND_MASTER_CMD - 发DiSEqC 命令

## 摘要



`int ioctl(int fd, FE_DISEQC_SEND_MASTER_CMD, struct dvb_diseqc_master_cmd *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符

`argp`
    指向结构`dvb_diseqc_master_cmd` 的指

## 说明


`dvb_diseqc_master_cmd` 指向DiSEqC 命令发送到天线子系统

## 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
