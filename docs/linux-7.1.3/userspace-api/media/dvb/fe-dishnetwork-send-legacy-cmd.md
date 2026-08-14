######## FE_DISHNETWORK_SEND_LEGACY_CMD


## 名称


FE_DISHNETWORK_SEND_LEGACY_CMD

## 摘要



`int ioctl(int fd, FE_DISHNETWORK_SEND_LEGACY_CMD, unsigned long cmd)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`cmd`
    通过 DISEqC 向天线（dish）发送指定的原始命令。

## 说明


   这是一个非常冷门（obscure）的遗留命令，仅用于 stv0299 驱动。不应在新驱动中使用。

它为前端（frontend）提供了一种非标准方法，用于为 Dish Network 遗留切换选择 Diseqc 电压。

由于对该 ioctl 的支持是在 2004 年加入的，这意味着此类天线在 2004 年时就已经是遗留设备了。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
