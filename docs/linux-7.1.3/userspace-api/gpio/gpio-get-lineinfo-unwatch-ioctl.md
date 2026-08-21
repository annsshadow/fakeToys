######## GPIO_GET_LINEINFO_UNWATCH_IOCTL


## 名称


GPIO_GET_LINEINFO_UNWATCH_IOCTL - 取消对某一线路其请求状态和配置信息变化的监视

## 摘要



`int ioctl(int chip_fd, GPIO_GET_LINEINFO_UNWATCH_IOCTL, u32 *offset)`

## 参数


`chip_fd`
    GPIO 字符设备`open()` 返回的文件描述符

`offset`
    不再监视的线路偏移量

## 说明


将该线路从此 `chip_fd` 上正在监视的线路列表中移除

这是 gpio-v2-get-lineinfo-watch-ioctl.rst（v2）和
gpio-get-lineinfo-watch-ioctl.rst（v1）的逆操作

对一条未监视的线路取消监视是一个错误（**EBUSY**）

最初添加于 5.7

## 杩斿洖鍊。


成功时返0

出错时返-1，并相应地设`errno` 变量
常见错误码在 error-codes.rst 中描述
