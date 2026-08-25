


######## GPIO_GET_LINEEVENT_IOCTL


    This ioctl is part of chardev_v1.rst and is obsoleted by
    gpio-v2-get-line-ioctl.rst.

## 名称


GPIO_GET_LINEEVENT_IOCTL - 从内核请求一条带边沿检测的线路
## 概要



`int ioctl(int chip_fd, GPIO_GET_LINEEVENT_IOCTL, struct gpioevent_request *request)`

## 参数


`chip_fd`
    GPIO 字符设备`open()` 返回的文件描述符
`request`
    指定要请求的线路及其配置`event_request<gpioevent_request>`
## 描述


从内核请求一条带边沿检测的线路
成功时，请求进程被授予对该线路值的独占访问权限，并可在线路上检测到边沿时接收事件，gpio-lineevent-data-read.rst 所述
线路的状态保证保持为所请求的状态，直到返回的文件描述符被关闭。一旦文件描述符被关闭，从用户空间的角度看，线路的状态变得不受控制，并可能恢复到其默认状态
请求一条已经被使用的线路是一个错误（**EBUSY**）
请求一条不支持中断的线路的边沿检测是一个错误（**ENXIO**）
line handle<gpio-get-linehandle-config-support> 一样，偏置（bias）配置是尽力而为的
关闭 `chip_fd` 对已有的线路事件没有影响
### 配置规则


以下配置规则适用
线路事件被作为输入端请求，因此不能设置任何专用于输出线路的标志，`GPIOHANDLE_REQUEST_OUTPUT`、`GPIOHANDLE_REQUEST_OPEN_DRAIN` `GPIOHANDLE_REQUEST_OPEN_SOURCE`
只能设置一个偏置标`GPIOHANDLE_REQUEST_BIAS_xxx`。若未设置任何偏置标志，则偏置配置不会被改变
边沿标志 `GPIOEVENT_REQUEST_RISING_EDGE` `GPIOEVENT_REQUEST_FALLING_EDGE` 可以组合，以同时检测上升沿与下降沿
请求无效的配置是一个错误（**EINVAL**）
## 杩斿洖鍊。

成功时返0，且 `request.fd<gpioevent_request>` 包含该请求的文件描述符
出错时返-1 并相应地设置 `errno` 变量。常见错误码error-codes.rst 中描述