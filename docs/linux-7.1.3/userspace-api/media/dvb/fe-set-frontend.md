######## FE_SET_FRONTEND


## Name


FE_SET_FRONTEND

## Synopsis


`int ioctl(int fd, FE_SET_FRONTEND, struct dvb_frontend_parameters *p)`

## Arguments


`fd`
    `open()` 返回的文件描述符
`p`
    指向调谐（tuning）操作所需参数的指针
## Description


ioctl 调用使用指定的参数启动一次调谐操作。如果参数有效且能够启动调谐，则该调用的结果将成功。然而，调谐操作本身的结果将作为事件异步到达（参FE_GET_EVENT FrontendEvent 的文档）。如果在前一个操作完成之前发起了新的 FE_SET_FRONTEND 操作，则前一个操作将被中止，以便执行新的操作。该命令需要对设备具有读写访问权限
## Return Value


成功时返0
出错时返-1，并相应地设`errno` 变量

    :header-rows:  0
    :stub-columns: 0
    :widths: 1 16

    - .. row 1

       - `EINVAL`

       - 达到所支持的最大符号率（symbol rate）
Generic error codes are described at the
Generic Error Codes <gen-errors> chapter.
