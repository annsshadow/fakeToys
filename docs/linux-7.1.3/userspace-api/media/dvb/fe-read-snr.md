######## FE_READ_SNR


## 名称


FE_READ_SNR


## 摘要



`int ioctl(int fd, FE_READ_SNR, int16_t *snr)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`snr`
    信噪比被存入 \*snr。

## 说明


该 ioctl 调用返回前端当前接收信号的信噪比。对于该命令，对设备的只读访问即已足够。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
