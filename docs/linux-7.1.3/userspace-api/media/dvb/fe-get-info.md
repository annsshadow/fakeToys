######## ioctl FE_GET_INFO


## Name


FE_GET_INFO - 查询数字电视前端（front-end）的能力并返回有关前端的信息。该调用仅需要对设备具有只读访问权限
## Synopsis


`int ioctl(int fd, FE_GET_INFO, struct dvb_frontend_info *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `dvb_frontend_info` 的指针
## Description


所有数字电视（Digital TV）前端设备都支持 FE_GET_INFO ioctl。它用于识别与此规范兼容的内核设备，并获取有关驱动程序和硬件能力的信息。该 ioctl 接收一个指dvb_frontend_info 的指针，由驱动程序填充。当驱动程序与此规范不兼容时，该 ioctl 返回错误
## frontend capabilities


能力（capabilities）描述了前端能够执行的操作。某些能力仅在特定类型的前端上受支持
前端能力`fe_caps` 中描述
## Return Value


成功时返0
出错时返-1，并相应地设`errno` 变量
通用错误码在 Generic Error Codes <gen-errors> 章节中描述