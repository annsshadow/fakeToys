######## ioctl NET_REMOVE_IF


## Name


NET_REMOVE_IF - 移除一个网络接口。

## Synopsis



`int ioctl(int fd, NET_REMOVE_IF, int ifnum)`

## Arguments


`fd`
    由 `open()` 返回的文件描述符。

`net_if`
    要移除的接口编号

## Description


NET_REMOVE_IF ioctl 删除之前通过 NET_ADD_IF <net> 创建的接口。

## Return Value


成功时返回 0，并填充 `ca_slot_info`。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
