

######## ioctl FE_SET_TONE


## 姓名


FE_SET_TONE - 设置/重置连续 22kHz 音调的生成。

## 概要



`int ioctl(int fd, FE_SET_TONE, enum fe_sec_tone_mode tone)`

## 论点


`fd`
`open()`返回的文件描述符。

`tone`
`fe_sec_tone_mode`描述的整数枚举值

## 描述


该 ioctl 用于设置连续 22kHz 音调的生成。
此调用需要读/写权限。

通常，卫星天线子系统要求数字电视设备
发送 22kHz 音调，以便在某些设备上选择高/低频段
双频 LNBf。它还用于向 DiSEqC 设备发送信号，但是
这是使用 DiSEqC ioctl 完成的。

设置音调可能会干扰其他设备，因为它们可能会丢失
选择频段的能力。所以，建议申请
当设备不使用时，将更改为 SEC_TONE_OFF。

## 返回值


成功时返回 0。

错误时返回-1，并设置`errno`变量
适当地。

通用错误代码的描述见
通用错误代码 <gen-errors> 章节。
