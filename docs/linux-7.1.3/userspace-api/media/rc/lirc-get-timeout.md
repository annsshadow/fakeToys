

######## ioctl LIRC_GET_MIN_TIMEOUT 和 LIRC_GET_MAX_TIMEOUT


## 姓名


LIRC_GET_MIN_TIMEOUT / LIRC_GET_MAX_TIMEOUT - 获取可能的超时时间
红外接收范围。

## 概要



`int ioctl(int fd, LIRC_GET_MIN_TIMEOUT, __u32 *timeout)`


`int ioctl(int fd, LIRC_GET_MAX_TIMEOUT, __u32 *timeout)`

## 论点


`fd`
open() 返回的文件描述符。

`timeout`
超时，以微秒为单位。

## 描述


某些设备具有内部定时器，可用于检测何时
很长一段时间没有IR活动。这可以帮助 lircd
检测IR信号完成，可以加快解码速度
过程。返回具有最小/最大超时的整数值
可以设置。


有些设备有固定的超时时间，在这种情况下
即使超时，两个 ioctl 也会返回相同的值
无法通过 LIRC_SET_REC_TIMEOUT 更改。

## 返回值


成功时返回 0，错误时返回 -1 并且设置 `errno` 变量
适当地。通用错误代码的描述见
通用错误代码 <gen-errors> 章节。
