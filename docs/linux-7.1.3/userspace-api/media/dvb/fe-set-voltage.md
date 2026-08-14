


######## ioctl FE_SET_VOLTAGE


## 名称


FE_SET_VOLTAGE - 允许设置发送到天线子系统的直流电平。

## 概要


`int ioctl(int fd, FE_SET_VOLTAGE, enum fe_sec_voltage voltage)`

## 参数


`fd`
    `open()` 返回的文件描述符。

`voltage`
    在 `fe_sec_voltage` 中描述的整型枚举值。

## 描述


该 ioctl 允许设置通过天线电缆发送的直流电压电平为 13V、18V 或关闭。

通常，卫星天线子系统要求数字电视设备发送直流电压以为 LNBf 供电。根据
LNBf 的类型，其极化或中频（IF）可由电压电平控制。其他设备（例如实现
DISEqC 与多点 LNBf 的设备）不需要控制电压电平，只要发送 13V 或 18V 为
LNBf 上电即可。

  设置电压电平可能会干扰其他设备，因为它们可能丧失设置极化或
  IF 的能力。因此，在这些情况下，建议在不使用设备时将电压
  设置为 SEC_VOLTAGE_OFF。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
