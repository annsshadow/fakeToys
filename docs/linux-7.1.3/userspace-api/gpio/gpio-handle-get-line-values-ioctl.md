

######## GPIOHANDLE_GET_LINE_VALUES_IOCTL

    此 ioctl 是 chardev_v1.rst 的一部分，并已被 gpio-v2-line-get-values-ioctl.rst 取代。

## 名称

GPIOHANDLE_GET_LINE_VALUES_IOCTL - 获取所有已请求线的值。

## 概要

`int ioctl(int handle_fd, GPIOHANDLE_GET_LINE_VALUES_IOCTL, struct gpiohandle_data *values)`

## 参数

`handle_fd`
    GPIO 字符设备的文件描述符，由 gpio-get-linehandle-ioctl.rst 在 `request.fd<gpiohandle_request>` 中返回。

`values`
    待填充的 `line_values<gpiohandle_data>`。

## 描述

获取所有已请求线的值。

返回的值是逻辑值，指示该线是激活还是非激活。`GPIOHANDLE_REQUEST_ACTIVE_LOW` 标志控制物理值（高/低）与逻辑值（激活/非激活）之间的映射。如果未设置 `GPIOHANDLE_REQUEST_ACTIVE_LOW`，则高为激活、低为非激活。如果设置了 `GPIOHANDLE_REQUEST_ACTIVE_LOW`，则低为激活、高为非激活。

输入线和输出线的值均可被读取。

对于输出线，返回的值取决于驱动和配置，可能是输出缓冲区（最后设置的请求值）或输入缓冲区（线的实际电平），并且根据硬件和配置的不同，二者可能存在差异。

此 ioctl 也可用于读取线事件的线值，将 `event_fd` 替换为 `handle_fd`。由于这种情况下只请求了一条线，因此 `values` 中只返回一个值。

## 返回值

成功时返回 0，且 `values` 被填充为读取到的值。

出错时返回 -1，并适当设置 `errno` 变量。常见的错误码在 error-codes.rst 中描述。
