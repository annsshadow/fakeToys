


######## GPIO_LINEINFO_CHANGED_READ


    ioctl chardev_v1.rst 的一部分，已gpio-v2-lineinfo-changed-read.rst
    取代
## 名称（Name

GPIO_LINEINFO_CHANGED_READ - 从芯片读取被监视线路的行信息变更事件
## 概要（Synopsis

`int read(int chip_fd, void *buf, size_t count)`

## 参数（Arguments

`chip_fd`
    GPIO 字符设备`open()` 返回的文件描述符
`buf`
    用于容纳 `events<gpioline_info_changed>` 的缓冲区
`count`
    `buf` 中可用的字节数，必须至少`gpioline_info_changed` 事件的大小
## 描述（Description

从芯片读取被监视线路的行信息变更事件
    监视行信息变更通常并非必需，一般只会由系统监视组件执行
    这些事件与线路请求状态或配置的变化有关，而非其数值。当线路数值变化时，请使用
    gpio-lineevent-data-read.rst 来接收事件
一条线路必须使gpio-get-lineinfo-watch-ioctl.rst 进行监视，才能生成信息变更事件随后，对该线路的请求、释放或重新配置将生成一个信息变更事件
内核在事件发生时为其打上时间戳，并将其存储在一个缓冲区中，用户空间可在方便时通过
`read()` 读取它们
内核事件缓冲区的大小固定为每`chip_fd` 32 个事件
如果用户空间读取事件的速度跟不上突发事件的到来，缓冲区可能会溢出。如果发生溢出，
则最新的事件会被丢弃。溢出无法从用户空间检测到
从缓冲区读取的事件顺序始终与内核检测到它们的顺序一致，包括当一`chip_fd` 监视
多条线路时
为了尽量减少将事件从内核复制到用户空间所需的调用次数，`read()` 支持一次复制多事件。复制的事件数量是内核缓冲区中可用事件数与能放入用户空间缓冲区（`buf`）的事件
数中的较小者
如果没有可用事件，且 `chip_fd` 未被设置**O_NONBLOCK**，则 `read()` 会阻塞
可以通过使用 `poll()` 或等价方法检`chip_fd` 是否可读，来测试是否存在事件
首次添加5.7
## 返回值（Return Value

成功时返回读取的字节数，该字节数将是 `gpioline_info_changed` 事件大小的整数倍
出错时返-1，并适当地设`errno` 变量。常见错误码error-codes.rst 中描述