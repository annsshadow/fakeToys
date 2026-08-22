

######## GPIO_LINEEVENT_DATA_READ


    ioctl 属于 chardev_v1.rst，已    gpio-v2-line-event-read.rst 废弃
## 名称


GPIO_LINEEVENT_DATA_READ - 从线路事件读取边沿检测事件
## 概要


`int read(int event_fd, void *buf, size_t count)`

## 参数


`event_fd`
    GPIO 字符设备的文件描述符，由 gpio-get-lineevent-ioctl.rst     `request.fd<gpioevent_request>` 中返回
`buf`
    用于容纳 `events<gpioevent_data>` 的缓冲区
`count`
    `buf` 中可用的字节数，必须至少    一`gpioevent_data` 的大小
## 描述


从线路事件中读取线路的边沿检测事件
必须为输入线路启用边沿检测，使用 `GPIOEVENT_REQUEST_RISING_EDGE` `GPIOEVENT_REQUEST_FALLING_EDGE`，或两者都使用。随后，只要在该输入线路上检测到边沿中断，就会生成边沿事件
边沿是根据逻辑线路值的变化来定义的，因此从无效到有效的跳变是上升沿。如果设置了 `GPIOHANDLE_REQUEST_ACTIVE_LOW`，那么逻辑极性与物理极性相反，此时 `GPIOEVENT_REQUEST_RISING_EDGE` 对应的就是物理上的下降沿
内核会在边沿事件发生时尽可能接近地捕获并打上时间戳，将其存入一个缓冲区，用户空间可在方便时通过 `read()` 读取
`event.timestamp<gpioevent_data>` 的时钟源`CLOCK_MONOTONIC`，但Linux 5.7 之前的版本中`CLOCK_REALTIME`。`gpioevent_data` 中并未指明使用的是哪个时钟源，必须根据内核版本或对时间戳本身进行合理性检查来确定
从缓冲区读取的事件，其顺序始终与内核检测到的顺序一致
内核事件缓冲区的大小固定16 个事件
如果用户空间读取事件的速度赶不上事件突发的速度，缓冲区可能会溢出。如果发生溢出，则最新的事件会被丢弃。溢出无法从用户空间检测
为了尽量减少从内核向用户空间复制事件所需的调用次数，`read()` 支持一次复制多个事件。复制的事件数量是内核缓冲区中可用事件数量与用户空间缓冲区（`buf`）所能容纳事件数量中的较小者
如果 `event_fd` 未设置为 **O_NONBLOCK**，且没有可用事件，则 `read()` 会阻塞
可通过使用 `poll()` 或等价方式检`event_fd` 是否可读，来测试是否存在事件
## 杩斿洖鍊。

成功时返回读取的字节数，该字节数将是 `gpio_lineevent_data` 事件大小的整数倍
出错时返-1，并相应地设`errno` 变量常见错误码在 error-codes.rst 中描述