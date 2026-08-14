######## Digital TV frontend close()


## Name


fe-close - 关闭一个前端设备

## Synopsis



    #include <unistd.h>


## Arguments


`fd`
    由 `open()` 返回的文件描述符。

## Description


该系统调用关闭一个先前已打开的前端设备。关闭前端设备后，其对应硬件可能会自动断电。

## Return Value


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
