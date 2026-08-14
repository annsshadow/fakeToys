
######## GPIO_HANDLE_SET_LINE_VALUES_IOCTL

    该 ioctl 是 chardev_v1.rst 的一部分，已被
    gpio-v2-line-set-values-ioctl.rst 废弃。

## 名称


GPIO_HANDLE_SET_LINE_VALUES_IOCTL - 设置所有被请求输出线的数值。

## 概要


`int ioctl(int handle_fd, GPIO_HANDLE_SET_LINE_VALUES_IOCTL, struct gpiohandle_data *values)`

## 参数


`handle_fd`
    GPIO 字符设备的文件描述符，由
    gpio-get-linehandle-ioctl.rst 在 `request.fd<gpiohandle_request>` 中返回。

`values`
    要设置的 `line_values<gpiohandle_data>`。

## 描述


设置所有被请求输出线的数值。

设置的数值是逻辑值，表示线路是激活还是非激活。`GPIOHANDLE_REQUEST_ACTIVE_LOW`
标志控制逻辑值（激活/非激活）与物理值（高/低）之间的映射。若未设置
`GPIOHANDLE_REQUEST_ACTIVE_LOW`，则激活为高、非激活为低。若设置了
`GPIOHANDLE_REQUEST_ACTIVE_LOW`，则激活为低、非激活为高。

只能设置输出线的数值。
尝试设置输入线的数值是一个错误（**EPERM**）。

## 返回值


成功时返回 0。

出错时返回 -1 并相应地设置 `errno` 变量。常见错误码在 error-codes.rst 中说明。
