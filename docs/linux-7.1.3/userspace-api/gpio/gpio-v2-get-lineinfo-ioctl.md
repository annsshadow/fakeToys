


######## GPIO_V2_GET_LINEINFO_IOCTL


## 名称


GPIO_V2_GET_LINEINFO_IOCTL - 获取某条线的公开可用信息。

## 概要



`int ioctl(int chip_fd, GPIO_V2_GET_LINEINFO_IOCTL, struct gpio_v2_line_info *info)`

## 参数


`chip_fd`
    由 `open()` 返回的 GPIO 字符设备的文件描述符。

`info`
    要填充的 `line_info<gpio_v2_line_info>`，其中 `offset` 字段
    设置为指示要收集的线路。

## 描述


获取某条线的公开可用信息。

无论该线路是否正在使用，此信息都可用。

    线路信息不包括线路值。

    必须使用 gpio-v2-get-line-ioctl.rst 请求该线路才能访问其值。

## 返回值


成功时返回 0，并填充 `info` 的芯片信息。

出错时返回 -1，并相应地设置 `errno` 变量。常见错误码在 error-codes.rst
中描述。
