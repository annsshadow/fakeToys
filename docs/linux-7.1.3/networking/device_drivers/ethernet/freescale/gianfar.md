
## Gianfar 以太网驱


:Author: Andy Fleming <afleming@freescale.com>
:Updated: 2005-07-28


## 鏍￠獙鍜屽嵏杞。


eTSEC 控制器（首次包含2005 年末的部件如 8548 中）能够在硬件中执行 TCP、UDP IP 校验和。Linux 内核只卸TCP UDP 校验和（并总是执行伪头部校验和），因此该驱动只支持 TCP/IP UDP/IP 数据包的校验和。使ethtool RX TX 启用或禁用此特性

## VLAN


为了使用 VLAN，请查阅 Linux 关于配置 VLAN 的文档。gianfar 驱动支持硬件插入和提VLAN 头部，但不支持过滤。过滤将由内核完成

## 多播


gianfar 驱动支持TSEC（以eTSEC 上的扩展哈希表）上使用组哈希表进行多播过滤。在 eTSEC 上，在哈希表之前会使用精确匹MAC 寄存器。有关如何加入多播组，请参阅 Linux 文档

## 填充


gianfar 驱动支持在硬件支持时，用 2 字节填充接收到的帧，以将 IP 头部对齐16 字节边界

## Ethtool


gianfar 驱动支持使用 ethtool 进行许多配置选项。你只能在当前已打开的接口上运行 ethtool。详ethtool 文档
