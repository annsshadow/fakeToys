


######## GPIO_V2_LINE_SET_CONFIG_IOCTL


## 名称


GPIO_V2_LINE_SET_CONFIG_IOCTL - 更新先前请求的线路配置

## 概要



`int ioctl(int req_fd, GPIO_V2_LINE_SET_CONFIG_IOCTL, struct gpio_v2_line_config *config)`

## 参数


`req_fd`
    GPIO 字符设备的文件描述符，如
    `request.fd<gpio_v2_line_request>` gpio-v2-get-line-ioctl.rst 所返回的那样

`config`
    要应用到
    请求线路上的`configuration<gpio_v2_line_config>`

## 描述


更新先前请求的线路配置，无需释放
线路或引入潜在的故障

新配置必须为所有请求的线路指定配置

请求线路时适用的相gpio-v2-get-line-config-rules 
gpio-v2-get-line-config-support 在更新线路配置时同样适用，另
一条限制：必须设置方向标志以启用重新配置

如果某条线路在配置中未设置方向标志，则该
线路配置保持不变

该命令的主要用例是在
输入和输出之间改变双向线路的方向，但也可用于
动态控制边沿检测，或更一般地让线路在
不同配置状态之间无缝切换

如果只想改变输出线路的值，请使
gpio-v2-line-set-values-ioctl.rst銆。

## 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量
常见错误码在 error-codes.rst 中描述
