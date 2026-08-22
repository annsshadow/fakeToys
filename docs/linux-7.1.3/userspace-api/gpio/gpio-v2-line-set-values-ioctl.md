
######## GPIO_V2_LINE_SET_VALUES_IOCTL


## 名称


GPIO_V2_LINE_SET_VALUES_IOCTL - 设置被请求输出线的数值
## 概要


`int ioctl(int req_fd, GPIO_V2_LINE_SET_VALUES_IOCTL, struct gpio_v2_line_values *values)`

## 参数


`req_fd`
    GPIO 字符设备的文件描述符，由
    gpio-v2-get-line-ioctl.rst `request.fd<gpio_v2_line_request>` 中返回
`values`
    要设置的 `line_values<gpio_v2_line_values>`，其`mask` 设为指示要设    的被请求线的子集，`bits` 设为指示新值
## 描述


设置被请求输出线的数值
设置的数值是逻辑值，表示线路是激活还是非激活。`GPIO_V2_LINE_FLAG_ACTIVE_LOW`
标志控制逻辑值（激非激活）与物理值（低）之间的映射。若未设`GPIO_V2_LINE_FLAG_ACTIVE_LOW`，则激活为高、非激活为低。若设置`GPIO_V2_LINE_FLAG_ACTIVE_LOW`，则激活为低、非激活为高
只能设置输出线的数值尝试设置输入线的数值是一个错误（**EPERM**）
## 杩斿洖鍊。

成功时返0
出错时返-1 并相应地设置 `errno` 变量。常见错误码error-codes.rst 中说明