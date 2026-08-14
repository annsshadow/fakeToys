

######## GPIO_V2_GET_LINEINFO_WATCH_IOCTL


## 名称


GPIO_V2_GET_LINEINFO_WATCH_IOCTL - 启用对一条线的请求状态和配置信息变化的监视。

## 概要



`int ioctl(int chip_fd, GPIO_V2_GET_LINEINFO_WATCH_IOCTL, struct gpio_v2_line_info *info)`

## 参数


`chip_fd`
    `open()` 返回的 GPIO 字符设备的文件描述符。

`info`
    待填充的 `line_info<gpio_v2_line_info>` 结构体，其中 `offset` 被设为指示要监视的线

## 描述


启用对一条线的请求状态和配置信息变化的监视。线信息的变化包括一条线被请求、释放或重新配置。

   监视线信息通常不是必需的，一般只有系统监控组件才会使用。

   线信息不包含线的值。
   必须使用 gpio-v2-get-line-ioctl.rst 来请求该线以访问其值，并且该线请求可以使用 gpio-v2-line-event-read.rst 来监视线的事件。

默认情况下，当 GPIO 芯片被打开时所有线都未被监视。

可以通过为每条线添加监视来同时监视多条线。

一旦设置了监视，任何线信息的变化都会生成事件，可从 `chip_fd` 读取，如 gpio-v2-lineinfo-changed-read.rst 所述。

向一条已被监视的线添加监视会出错（**EBUSY**）。

监视是特定于 `chip_fd` 的，并且独立于使用单独的 `open()` 调用打开的同一 GPIO 芯片上的监视。

## 返回值


成功时返回 0，且 `info` 被填充为当前线信息。

出错时返回 -1，且 `errno` 变量被适当设置。常见的错误码在 error-codes.rst 中描述。
