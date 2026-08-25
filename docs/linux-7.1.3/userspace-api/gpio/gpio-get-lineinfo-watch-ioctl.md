


######## GPIO_GET_LINEINFO_WATCH_IOCTL


    ioctl chardev_v1.rst 的一部分，已    gpio-v2-get-lineinfo-watch-ioctl.rst 废弃
## 名称


GPIO_GET_LINEINFO_WATCH_IOCTL - 启用对一条线的请求状态和配置信息的变更监视
## 概要


`int ioctl(int chip_fd, GPIO_GET_LINEINFO_WATCH_IOCTL, struct gpioline_info *info)`

## 参数


`chip_fd`
    `open()` 返回GPIO 字符设备的文件描述符
`info`
    要填充的 `line_info<gpioline_info>` 结构体，其中
    `offset` 被设置为指示要监视的
## 描述


启用对一条线的请求状态和配置信息的变更监视。线信息的变更包括一条线被请求、释放或重新配置
    监视线信息通常并不是必需的，一般只会被系统监控组件使用
    线信息不包含线的值（value）
    必须使用 gpio-get-linehandle-ioctl.rst gpio-get-lineevent-ioctl.rst 来请求一条线以访问其值，并且可以使用 gpio-lineevent-data-read.rst 通过线事件监视一条线
默认情况下，GPIO 芯片被打开时，所有线都未被监视
可以通过为每条线添加监视来同时监视多条线
一旦设置了监视，任何线信息的变更都会生成事件，可以`chip_fd` 读取，如 gpio-lineinfo-changed-read.rst 所述
向一条已经被监视的线添加监视是一个错误（**EBUSY**）
监视是特定于 `chip_fd` 的，并且独立于通过`open()` 的单独调用打开的同一GPIO 芯片上的监视
首次添加5.7
## 杩斿洖鍊。

成功时返0，并`info` 被填充为当前的线信息
出错时返-1，并相应地设`errno` 变量常见错误码在 error-codes.rst 中描述