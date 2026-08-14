######## ioctl FE_ENABLE_HIGH_LNB_VOLTAGE


## 名称


FE_ENABLE_HIGH_LNB_VOLTAGE - 在正常 LNBf 电压与更高的 LNBf 电压之间选择输出直流电平。

## 摘要



`int ioctl(int fd, FE_ENABLE_HIGH_LNB_VOLTAGE, unsigned int high)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`high`
    有效标志：

    - 0 - 正常的 13V 和 18V。

    - >0 - 启用略高的电压以替代 13/18V，用于补偿过长的天线电缆。

## 说明


在正常 LNBf 电压与更高 LNBf 电压之间选择输出直流电平：0（正常）或大于 0 的值（更高电压）。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
