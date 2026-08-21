######## GPIO_V2_LINEINFO_CHANGED_READ


## 名称


GPIO_V2_LINEINFO_CHANGED_READ - 从芯片读取被监视线路line info 变更事件
## 摘要


`int read(int chip_fd, void *buf, size_t count)`

## 参数


`chip_fd`
    GPIO 字符设备的文件描述符，由 `open()` 返回
`buf`
    用于容纳 `events<gpio_v2_line_info_changed>` 的缓冲区
`count`
    `buf` 中可用的字节数，必须至少`gpio_v2_line_info_changed` 事件的大小
## 描述


从芯片读取被监视线路line info 变更事件
    监视线路信息变更通常并不需要，一般只由系统监视组件执行
    这些事件与线路的请求状态或配置的变化有关，而非其数值。当线路数值变化时，使    gpio-v2-line-event-read.rst 接收事件
一条线路必须使gpio-v2-get-lineinfo-watch-ioctl.rst 进行监视以生成信息变更事件随后，线路的请求、释放或重新配置将生成一个信息变更事件
内核在事件发生时为其打上时间戳，并将其存储在一个缓冲区中，用户空间可在方便时通过
`read()` 读取
内核事件缓冲区的大小固定为每`chip_fd` 32 个事件
如果用户空间读取事件的速度慢于事件突发的速度，缓冲区可能溢出。如果发生溢出，最新的事件会被丢弃。溢出无法从用户空间检测
从缓冲区读取的事件顺序始终与内核检测到的顺序相同，包括在单`chip_fd` 监视多条线路情况下
为了尽量减少将事件从内核复制到用户空间所需的调用次数，`read()` 支持复制多个事件。复制的
事件数量是内核缓冲区中可用事件数量与能够放入用户空间缓冲区（`buf`）的事件数量中的较小者
如果没有可用事件`chip_fd` 未被设置**O_NONBLOCK**，则 `read()` 会阻塞
可以通过使用 `poll()` 或等效方法检`chip_fd` 是否可读来测试是否存在事件
## 杩斿洖鍊。

成功时返回读取的字节数，该字节数`gpio_v2_line_info_changed` 事件大小的整数倍
出错时返-1，并相应地设`errno` 变量。常见错误码error-codes.rst 中描述