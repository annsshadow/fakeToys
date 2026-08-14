

######## GPIO_V2_LINE_EVENT_READ


## 名称


GPIO_V2_LINE_EVENT_READ - 从请求中读取线路的边沿检测事件。

## 概要


`int read(int req_fd, void *buf, size_t count)`

## 参数


`req_fd`
    GPIO 字符设备的文件描述符，由 gpio-v2-get-line-ioctl.rst 在
    `request.fd<gpio_v2_line_request>` 中返回。

`buf`
    用于容纳 `events<gpio_v2_line_event>` 的缓冲区。

`count`
    `buf` 中可用的字节数，必须至少为
    一个 `gpio_v2_line_event` 的大小。

## 描述


从请求中读取线路的边沿检测事件。

必须为输入线路启用边沿检测，使用 `GPIO_V2_LINE_FLAG_EDGE_RISING` 或 `GPIO_V2_LINE_FLAG_EDGE_FALLING`，或两者都使用。随后，只要在该输入线路上检测到边沿中断，就会生成边沿事件。

边沿是根据逻辑线路值的变化来定义的，因此从无效（inactive）到有效（active）的跳变是上升沿。如果设置了 `GPIO_V2_LINE_FLAG_ACTIVE_LOW`，那么逻辑极性与物理极性相反，此时 `GPIO_V2_LINE_FLAG_EDGE_RISING` 对应的就是物理上的下降沿。

内核会在边沿事件发生时尽可能接近地捕获并打上时间戳，将其存入一个缓冲区，用户空间可在方便时通过 `read()` 读取。

从缓冲区读取的事件，其顺序始终与内核检测到的顺序一致，包括当一个请求监控多条线路时也是如此。

内核事件缓冲区的大小在线路请求创建时固定，并可由
`request.event_buffer_size<gpio_v2_line_request>` 影响。
默认大小为所请求线路数量的 16 倍。

如果用户空间读取事件的速度赶不上事件突发的速度，缓冲区可能会溢出。如果发生溢出，则最旧的缓冲事件会被丢弃。用户空间可通过监控事件序列号来检测溢出。

为了尽量减少从内核向用户空间复制事件所需的调用次数，`read()` 支持一次复制多个事件。复制的事件数量是内核缓冲区中可用事件数量与用户空间缓冲区（`buf`）所能容纳事件数量中的较小者。

使用 gpio-v2-line-set-config-ioctl.rst 更改边沿检测标志，不会移除或修改内核事件缓冲区中已有的事件。

如果 `req_fd` 未设置为 **O_NONBLOCK**，且没有可用事件，则 `read()` 会阻塞。

可通过使用 `poll()` 或等价方式检查 `req_fd` 是否可读，来测试是否存在事件。

## 返回值


成功时返回读取的字节数，该字节数将是 `gpio_v2_line_event` 事件大小的整数倍。

出错时返回 -1，并相应地设置 `errno` 变量。
常见错误码在 error-codes.rst 中描述。
