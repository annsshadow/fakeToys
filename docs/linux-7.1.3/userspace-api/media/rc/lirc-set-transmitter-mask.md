

######## ioctl LIRC_SET_TRANSMITTER_MASK


## 姓名


LIRC_SET_TRANSMITTER_MASK - 在给定的一组发射器上启用发送代

## 概要



`int ioctl(int fd, LIRC_SET_TRANSMITTER_MASK, __u32 *mask)`

## 论点


`fd`
open() 返回的文件描述符

`mask`
带有通道的掩码以启用 tx。通道 0 是最低有效位

## 描述


有些 IR TX 设备有多个输出通道，在这种情况下，
LIRC_CAN_SET_TRANSMITTER_MASK <LIRC-CAN-SET-TRANSMITTER-MASK> 鏄。
通过 LIRC_GET_FEATURES 返回，此 ioctl 设置哪些通道
发送红外代码

ioctl 启用给定的一组发射器。第一个发射器
由最低有效位编码等等

当给出无效的位掩码时，即设置了一个位，即使设
没有那么多中转器，那么这ioctl 返回的数
可用的传输器，并且不执行任何其他操作

## 杩斿洖鍊。


成功时返0，错误时返回 -1 并且设置 `errno` 变量
适当地。通用错误代码的描述见
通用错误代码 <gen-errors> 章节
