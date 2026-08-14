######## ioctl FE_DISEQC_SEND_MASTER_CMD


## 名称


FE_DISEQC_SEND_MASTER_CMD - 发送 DiSEqC 命令

## 摘要



`int ioctl(int fd, FE_DISEQC_SEND_MASTER_CMD, struct dvb_diseqc_master_cmd *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向结构体 `dvb_diseqc_master_cmd` 的指针

## 说明


将 `dvb_diseqc_master_cmd` 指向的 DiSEqC 命令发送到天线子系统。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
