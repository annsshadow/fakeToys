
######## GPIO_HANDLE_SET_LINE_VALUES_IOCTL

    ioctl chardev_v1.rst 的一部分，已    gpio-v2-line-set-values-ioctl.rst 废弃
## 名称


GPIO_HANDLE_SET_LINE_VALUES_IOCTL - 设置所有被请求输出线的数值
## 概要


`int ioctl(int handle_fd, GPIO_HANDLE_SET_LINE_VALUES_IOCTL, struct gpiohandle_data *values)`

## 参数


`handle_fd`
    GPIO 字符设备的文件描述符，由
    gpio-get-linehandle-ioctl.rst `request.fd<gpiohandle_request>` 中返回
`values`
    要设置的 `line_values<gpiohandle_data>`
## 描述


设置所有被请求输出线的数值
设置的数值是逻辑值，表示线路是激活还是非激活。`GPIOHANDLE_REQUEST_ACTIVE_LOW`
标志控制逻辑值（激非激活）与物理值（低）之间的映射。若未设`GPIOHANDLE_REQUEST_ACTIVE_LOW`，则激活为高、非激活为低。若设置`GPIOHANDLE_REQUEST_ACTIVE_LOW`，则激活为低、非激活为高
只能设置输出线的数值尝试设置输入线的数值是一个错误（**EPERM**）
## 杩斿洖鍊。

成功时返0
出错时返-1 并相应地设置 `errno` 变量。常见错误码error-codes.rst 中说明