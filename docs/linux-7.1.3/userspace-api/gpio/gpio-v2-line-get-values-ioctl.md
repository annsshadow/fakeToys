
######## GPIO_V2_LINE_GET_VALUES_IOCTL


## 名称


GPIO_V2_LINE_GET_VALUES_IOCTL - 获取所请求线路的值。

## 概要


`int ioctl(int req_fd, GPIO_V2_LINE_GET_VALUES_IOCTL, struct gpio_v2_line_values *values)`

## 参数


`req_fd`
    GPIO 字符设备的文件描述符，由 gpio-v2-get-line-ioctl.rst 在
    `request.fd<gpio_v2_line_request>` 中返回。

`values`
    要获取的 `line_values<gpio_v2_line_values>`，其中 `mask` 被
    设置以指示要获取的所请求线路的子集。

## 描述


获取所请求线路的值。

返回的值是逻辑值，表示线路是激活还是非激活。`GPIO_V2_LINE_FLAG_ACTIVE_LOW`
标志控制物理值（高/低）与逻辑值（激活/非激活）之间的映射。若未设置
`GPIO_V2_LINE_FLAG_ACTIVE_LOW`，则高电平为激活、低电平为非激活；若设置了
`GPIO_V2_LINE_FLAG_ACTIVE_LOW`，则低电平为激活、高电平为非激活。

输入线路和输出线路的值均可读取。

对于输出线路，返回的值取决于驱动和配置，可能是输出缓冲区（最后设置的请求值）或输入缓冲区（线路的实际电平），并且根据硬件和配置的不同，二者可能不一致。

## 返回值


成功时返回 0，且相应的 `values.bits<gpio_v2_line_values>` 包含读取到的值。

出错时返回 -1，并相应地设置 `errno` 变量。常见错误码在 error-codes.rst 中描述。
