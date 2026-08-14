######## ioctl LIRC_SET_REC_CARRIER_RANGE


## 名称


LIRC_SET_REC_CARRIER_RANGE - 设置用于调制红外接收的载波频率下限。

## 摘要



`int ioctl(int fd, LIRC_SET_REC_CARRIER_RANGE, __u32 *frequency)`

## 参数


`fd`
    由 open() 返回的文件描述符。

`frequency`
    调制 PWM 数据的载波频率，单位为 Hz。

## 说明


该 ioctl 设置红外接收器能够识别的载波频率的上限范围。


   要设置范围，先使用 :ref:`LIRC_SET_REC_CARRIER_RANGE
   <LIRC_SET_REC_CARRIER_RANGE>` 设置下限，随后再调用
   LIRC_SET_REC_CARRIER <LIRC_SET_REC_CARRIER> 设置上限。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。
