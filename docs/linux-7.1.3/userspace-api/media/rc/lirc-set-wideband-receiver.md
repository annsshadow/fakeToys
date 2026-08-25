
######## ioctl LIRC_SET_WIDEBAND_RECEIVER


## 名称


LIRC_SET_WIDEBAND_RECEIVER - 启用宽带接收器
## 概要


`int ioctl(int fd, LIRC_SET_WIDEBAND_RECEIVER, __u32 *enable)`

## 参数


`fd`
    open() 返回的文件描述符
`enable`
    enable = 1 表示启用宽带接收器，enable = 0 表示禁用宽带接收器
## 描述


某些接收器配备了特殊的宽带接收器，旨在用于学习现有遥控器的输出。该 ioctl
允许启用或禁用它
对于本来具有窄带接收器、从而无法与某些遥控器配合使用的接收器，这可能很有用宽带接收器也可能更精确。另一方面，其缺点是接收范围通常减小

    如果你启用了载波报告，宽带接收器可能会被隐式启用。在这种情况下，一旦你
    禁用载波报告，它就会被禁用。在载波报告处于活动状态时尝试禁用宽带接收    将不起作用
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述