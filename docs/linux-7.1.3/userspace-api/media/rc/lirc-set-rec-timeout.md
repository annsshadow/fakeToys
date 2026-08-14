######## ioctl LIRC_GET_REC_TIMEOUT and LIRC_SET_REC_TIMEOUT


## 名称


LIRC_GET_REC_TIMEOUT/LIRC_SET_REC_TIMEOUT - 获取/设置 IR 非活动超时时间的整数值。

## 概要


`int ioctl(int fd, LIRC_GET_REC_TIMEOUT, __u32 *timeout)`


`int ioctl(int fd, LIRC_SET_REC_TIMEOUT, __u32 *timeout)`

## 参数


`fd`
    open() 返回的文件描述符。

`timeout`
    超时时间，单位为微秒。

## 描述


获取并设置 IR 非活动超时时间的整数值。

若硬件支持，将其设为 0 将禁用所有硬件超时，并应尽快上报数据。若无法设置精确值，则应将下一个大于给定值的可能值写入。

   支持的超时范围由 LIRC_GET_MIN_TIMEOUT 给出。

## 返回值


成功时返回 0，出错时返回 -1 并相应设置 `errno` 变量。通用错误码在“通用错误码 <gen-errors>”章节中描述。
