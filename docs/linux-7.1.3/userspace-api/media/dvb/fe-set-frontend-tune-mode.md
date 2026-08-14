

######## ioctl FE_SET_FRONTEND_TUNE_MODE


## 姓名


FE_SET_FRONTEND_TUNE_MODE - 允许为前端设置调谐器模式标志。

## 概要



`int ioctl(int fd, FE_SET_FRONTEND_TUNE_MODE, unsigned int flags)`

## 论点


`fd`
`open()`返回的文件描述符。

`flags`
有效标志：

    - 0 - 正常调谐模式

    - `FE_TUNE_MODE_ONESHOT` - 设置后，该标志将禁用任何
之字形或其他“正常”调谐行为。此外，
不会自动监控锁定状态，并且
因此不会生成任何前端事件。如果前端设备
关闭时，该标志将自动关闭
设备重新以读写方式打开。

## 描述


允许将调谐器模式标志设置为前端，介于 0（正常）或
`FE_TUNE_MODE_ONESHOT`模式

## 返回值


成功时返回 0。

错误时返回-1，并设置`errno`变量
适当地。

通用错误代码的描述见
通用错误代码 <gen-errors> 章节。
