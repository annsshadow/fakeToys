
## Flexcan CAN 控制器驱动


Authors: Marc Kleine-Budde <mkl@pengutronix.de>,
Dario Binacchi <dario.binacchi@amarulasolutions.com>

## RTR 帧接收的开启/关闭


对于大多数 flexcan IP 核，该驱动支持两种 RX 模式：

- FIFO
- mailbox

较旧的 flexcan 核（集成于 i.MX25、i.MX28、i.MX35
和 i.MX53 SoC）仅在控制器配置为 RX-FIFO 模式时
才能接收 RTR 帧。

RX FIFO 模式使用深度为 6 个 CAN 帧的硬件 FIFO，
而 mailbox 模式使用深度最高达 62 个
CAN 帧的软件 FIFO。借助更大的缓冲区，mailbox 模式
在高系统负载下表现更好。

由于接收 RTR 帧是 CAN 标准的一部分，所有 flexcan
核上电时处于可接收 RTR 帧的模式。

通过 "rx-rtr" 私有标志，可以放弃接收 RTR 帧的能力，
代价是失去接收 RTR
消息的能力。这种权衡在某些用例中是有利的。

"rx-rtr" on
  接收 RTR 帧。（默认）

  CAN 控制器能够并且将会接收 RTR 帧。

  在某些 IP 核上，控制器无法在性能更好的 "RX mailbox" 模式下
  接收 RTR 帧，而会使用 "RX FIFO" 模式
  代替。

"rx-rtr" off

  放弃接收 RTR 帧的能力。（并非所有 IP 核都支持）

  该模式会激活 "RX mailbox 模式" 以获得更好性能，在某些
  IP 核上则无法再接收 RTR 帧。

```

    ip link set dev can0 down
    ethtool --set-priv-flags can0 rx-rtr {off|on}
    ip link set dev can0 up

```
