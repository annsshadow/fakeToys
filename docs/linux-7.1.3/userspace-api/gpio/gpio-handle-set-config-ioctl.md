######## GPIOHANDLE_SET_CONFIG_IOCTL


    ioctl chardev_v1.rst 的一部分，已gpio-v2-line-set-config-ioctl.rst
    取代
## 名称


GPIOHANDLE_SET_CONFIG_IOCTL - 更新先前请求的线路配置
## 概要


`int ioctl(int handle_fd, GPIOHANDLE_SET_CONFIG_IOCTL, struct gpiohandle_config *config)`

## 参数


`handle_fd`
    GPIO 字符设备的文件描述符，由 gpio-get-linehandle-ioctl.rst     `request.fd<gpiohandle_request>` 中返回
`config`
    要应用到请求线路的新`configuration<gpiohandle_config>`
## 描述


更新先前请求的线路配置，而不释放线路或引入潜在的故障
该配置应用于所有请求的线路
请求线路时适用gpio-get-linehandle-config-rules gpio-get-linehandle-config-support 在更新线路配置时同样适用，附加限制是必须
设置方向标志。请求无效配置（包括未设置方向标志）是一个错误（**EINVAL**）
该命令的动机用例是在输入和输出之间改变双向线路的方向，但它也可更一般地用于
将线路从一个配置状态无缝移动到另一个
要仅更改输出线路的值，请使gpio-handle-set-line-values-ioctl.rst
首次添加5.5
## 杩斿洖鍊。

成功时返0
出错时返-1，并适当设置 `errno` 变量。常见错误码error-codes.rst 中描述