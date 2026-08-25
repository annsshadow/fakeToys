######## ioctl LIRC_SET_REC_CARRIER


## 名称


LIRC_SET_REC_CARRIER - 设置用于调制红外接收的载波频率

## 摘要



`int ioctl(int fd, LIRC_SET_REC_CARRIER, __u32 *frequency)`

## 参数


`fd`
    open() 返回的文件描述符

`frequency`
    调制 PWM 数据的载波频率，单位Hz

## 说明


设置用于调制红外 PWM 脉冲与间隔（spaces）的接收载波

   若与 LIRC_SET_REC_CARRIER_RANGE 一同调用，ioctl 设置设备能够识别的上限频率

## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
