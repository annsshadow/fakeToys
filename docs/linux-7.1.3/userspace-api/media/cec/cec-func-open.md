
######## cec open()


## 名称


cec-open - 打开一cec 设备

## 概要


    #include <fcntl.h>


## 参数


`device_name`
    要打开的设备

`flags`
    打开标志。访问模式必须为 `O_RDWR`

    当给`O_NONBLOCK` 标志时，在没有消息或事件可用的情况下，CEC_RECEIVE <CEC_RECEIVE> CEC_DQEVENT <CEC_DQEVENT> ioctl 将返`EAGAIN` 错误码，ioctl CEC_TRANSMIT <CEC_TRANSMIT>、CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 都返0

    其它标志没有效果

## 描述


要打开一cec 设备，应用程序调`open()` 并传入期望的设备名。该函数没有副作用；设备配置保持不变

当以只读模式打开设备时，尝试修改其配置将导致错误，并`errno` 将被设为 EBADF

## 杩斿洖鍊。


`open()` 成功时返回新的文件描述符。出错时返回 -1，并相应地设`errno`。可能的错误码包括：

`EACCES`
    不允许对文件进行请求的访问

`EMFILE`
    该进程已经打开了最大数量的文件

`ENFILE`
    系统对打开文件总数的限制已经达到

`ENOMEM`
    可用的内核内存不足

`ENODEV`
    未找到设备或已被移除
