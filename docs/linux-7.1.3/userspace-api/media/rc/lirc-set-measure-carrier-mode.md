######## ioctl LIRC_SET_MEASURE_CARRIER_MODE


## 名称


LIRC_SET_MEASURE_CARRIER_MODE - 启用或禁用测量模式

## 摘要



`int ioctl(int fd, LIRC_SET_MEASURE_CARRIER_MODE, __u32 *enable)`

## 参数


`fd`
    由 open() 返回的文件描述符。

`enable`
    enable = 1 表示启用测量模式，enable = 0 表示禁用测量模式。

## 说明


启用或禁用测量模式。若启用，从下一次按键起，驱动将发送 `LIRC_MODE2_FREQUENCY` 数据包。默认情况下该模式应处于关闭状态。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。
