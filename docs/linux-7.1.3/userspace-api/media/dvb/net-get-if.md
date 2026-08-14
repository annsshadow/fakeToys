######## ioctl NET_GET_IF


## 名称


NET_GET_IF - 读取通过 NET_ADD_IF <net> 创建的接口的配置数据。

## 概要


`int ioctl(int fd, NET_GET_IF, struct dvb_net_if *net_if)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`net_if`
    指向 struct `dvb_net_if` 的指针

## 描述


NET_GET_IF ioctl 使用 struct **`dvb_net_if`** 的 : ifnum 字段给定的接口号，并用该接口所使用的包 ID 与封装类型填充 struct `dvb_net_if` 的内容。如果尚未通过 NET_ADD_IF <net> 创建该接口，它将返回 -1 并将 `errno` 设为 `EINVAL` 错误码。

## 返回值


成功时返回 0，并填充 `ca_slot_info`。

出错时返回 -1，并相应设置 `errno` 变量。

通用错误码在“通用错误码 <gen-errors>”章节中描述。
