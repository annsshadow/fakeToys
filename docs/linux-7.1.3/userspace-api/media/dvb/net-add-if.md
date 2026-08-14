


######## ioctl NET_ADD_IF


## 名称


NET_ADD_IF - 为给定的包 ID 创建新的网络接口。

## 概要



`int ioctl(int fd, NET_ADD_IF, struct dvb_net_if *net_if)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`net_if`
    指向 struct `dvb_net_if` 的指针

## 描述


NET_ADD_IF ioctl 系统调用选择包含 TCP/IP 流量的包 ID (PID)、要使用的封装
类型（MPE 或 ULE）以及要创建的新接口的接口号。当系统调用成功返回时，会
创建一个新的虚拟网络接口。

**struct `dvb_net_if`**
：ifnum 字段将被填充为所创建接口的编号。

## 返回值


成功时返回 0，并填充 `ca_slot_info`。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码的描述见通用错误码 <gen-errors> 章节。
