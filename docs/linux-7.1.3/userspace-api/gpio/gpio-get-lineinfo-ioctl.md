

######## GPIO_GET_LINEINFO_IOCTL


此 ioctl 是 chardev_v1.rst 的一部分，并已被废弃
gpio-v2-get-lineinfo-ioctl.rst。

## 姓名


GPIO_GET_LINEINFO_IOCTL - 获取线路的公开信息。

## 概要



`int ioctl(int chip_fd, GPIO_GET_LINEINFO_IOCTL, struct gpioline_info *info)`

## 论点


`chip_fd`
`open()`返回的GPIO字符设备的文件描述符。

`info`
要填充的`line_info<gpioline_info>`，其中
`offset`字段设置为指示要收集的行。

## 描述


获取线路的公开信息。

该信息的可用性与线路是否正在使用无关。

线路信息不包括线路值。

必须使用 gpio-get-linehandle-ioctl.rst 请求该线路或
gpio-get-lineevent-ioctl.rst 来访问其值。

## 返回值


成功后，0 和 `info` 将填充芯片信息。

错误 -1 时，`errno` 变量已正确设置。
error-codes.rst 中描述了常见错误代码。
