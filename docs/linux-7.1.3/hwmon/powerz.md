## 内核驱动 POWERZ


支持的芯片：

  - ChargerLAB POWER-Z KM003C

    Prefix: 'powerz'

    Addresses scanned: -

作者：

  - Thomas Weißschuh <linux@weissschuh.net>

### Description


该驱动实现了对 ChargerLAB POWER-Z USB-C 电源测试系列的支持。

设备通过 USB 上的自定义协议进行通信。

通过 hwmon 暴露的通道标签与设备上显示屏以及官方 POWER-Z PC 软件使用的标签一致。

由于电流可双向流经测试仪，通道 "curr1_input"（标签 "IBUS"）的符号表示方向。
