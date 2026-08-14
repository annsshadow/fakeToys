######## FE_READ_UNCORRECTED_BLOCKS


## 名称


FE_READ_UNCORRECTED_BLOCKS


## 摘要



`int ioctl(int fd, FE_READ_UNCORRECTED_BLOCKS, uint32_t *ublocks)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`ublocks`
    驱动迄今为止看到的总未校正块数。

## 说明


该 ioctl 调用返回设备驱动在其生命周期内检测到的未校正块数量。为了获得有意义的测量值，应计算在特定时间间隔内块计数的增量。对于该命令，对设备的只读访问即已足够。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
