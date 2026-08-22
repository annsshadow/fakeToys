######## ioctl LIRC_GET_REC_RESOLUTION


## 名称


LIRC_GET_REC_RESOLUTION - 获取接收分辨率的值，单位为微秒

## 概要


`int ioctl(int fd, LIRC_GET_REC_RESOLUTION, __u32 *microseconds)`

## 参数


`fd`
    open() 返回的文件描述符

`microseconds`
    分辨率，单位为微秒

## 描述


部分接收器具有由内部采样率或数据格式限制决定的最大分辨率。例如，
信号通常只能50 微秒的步长上报

ioctl 返回具有该分辨率的整数值，可被 lircd 等用户空间应用程
用于自动调整容差（tolerance）值

## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量
通用错误码在 Generic Error Codes <gen-errors> 章节中描述
