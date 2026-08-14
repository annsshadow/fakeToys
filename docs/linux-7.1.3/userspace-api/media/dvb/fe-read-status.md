
######## ioctl FE_READ_STATUS


## Name


FE_READ_STATUS - 返回前端（front-end）的状态信息。该调用仅需要对设备具有只读访问权限。

## Synopsis


`int ioctl(int fd, FE_READ_STATUS, unsigned int *status)`

## Arguments


`fd`
    `open()` 返回的文件描述符。

`status`
    指向一个位掩码整数的指针，由 enum `fe_status` 中定义的值填充。

## Description


所有数字电视（Digital TV）前端设备都支持 `FE_READ_STATUS` ioctl。它用于在调谐（tune）之后检查前端的锁定（locking）状态。该 ioctl 接收一个指向整数的指针，状态信息将被写入其中。


   status 的实际大小为 sizeof(enum fe_status)，其值随体系结构而不同。这一点需要在将来修复。

## int fe_status


fe_status 参数用于指示前端硬件的当前状态和/或状态变化。它是使用 enum `fe_status` 的值按位掩码（bitmask）组合而成的。

## Return Value


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
