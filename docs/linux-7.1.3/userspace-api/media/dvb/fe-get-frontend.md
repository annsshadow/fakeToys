######## FE_GET_FRONTEND


## 名称


FE_GET_FRONTEND


## 摘要



`int ioctl(int fd, FE_GET_FRONTEND, struct dvb_frontend_parameters *p)`

## 参数


`fd`
    `open()` 返回的文件描述符

`p`
    指向调谐操作参数的指针

## 说明


ioctl 调用查询当前生效的前端参数。对于该命令，对设备的只读访问即已足够

## 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量

    :header-rows:  0
    :stub-columns: 0

    - .. row 1

       - `EINVAL`

       - 已达到支持的最大符号率

通用错误码在 Generic Error Codes <gen-errors> 章节中描述
