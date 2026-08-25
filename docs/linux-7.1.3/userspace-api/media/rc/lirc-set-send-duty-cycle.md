######## ioctl LIRC_SET_SEND_DUTY_CYCLE


## 名称


LIRC_SET_SEND_DUTY_CYCLE - 设置红外发射载波信号的占空比

## 摘要



`int ioctl(int fd, LIRC_SET_SEND_DUTY_CYCLE, __u32 *duty_cycle)`

## 参数


`fd`
    open() 返回的文件描述符

`duty_cycle`
    占空比，以百分比 99）描述整个周期的脉冲宽度。取0 100 为保留值

## 说明


获取/设置红外发射载波信号的占空比

目前 100 没有定义特殊含义，但将来可能用于关闭载波生成，因此应保留这些值

## 杩斿洖鍊。


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
