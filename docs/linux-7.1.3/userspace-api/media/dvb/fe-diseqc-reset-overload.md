######## ioctl FE_DISEQC_RESET_OVERLOAD


## 名称


FE_DISEQC_RESET_OVERLOAD - 如果总线因过载断电，则恢复天线子系统的供电。

## 摘要



`int ioctl(int fd, FE_DISEQC_RESET_OVERLOAD, NULL)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

## 说明


如果总线因功率过载被自动断电，该 ioctl 调用会恢复总线的供电。该调用需要对设备的读写访问权限。若设备被手动断电，则该调用无效。并非所有数字电视（Digital TV）适配器都支持该 ioctl。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
