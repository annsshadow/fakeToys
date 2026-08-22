
## Marvell(Aquantia) AQtion 驱动


适用aQuantia 多千兆位 PCI Express 系列以太网适配

    - 识别您的适配    - 配置
    - 受支持的 ethtool 选项
    - 命令行参    - 配置文件参数
    - 支持
    - 许可
## 识别您的适配

此版本驱动与基于 AQC-100、AQC-107、AQC-108 的以太网适配器兼容

### SFP+ 设备（适用于基AQC-100 的适配器）


此版本使用无源直连铜缆（DAC）和 SFP+/LC 光收发器进行了测试
## 配置


### 查看链路消息

  如果发行版限制了系统消息，链路消息将不会显示到控制台。为了在控制台上
  看到网络驱动链路消息，请使用
```

       dmesg -n 8

  .. note::

     此设置不会在重启后保留
```
### Jumbo Frames（巨型帧
  该驱动对所有适配器支Jumbo Frames。通过MTU 更改为大于默认1500
  的值来启用 Jumbo Frames 支持。MTU 的最大值为 16000。使`ip` 命令```

	ip link set mtu 16000 dev enp1s0

```
### ethtool

  驱动利用 ethtool 接口进行驱动配置和诊断，以及显示统计信息。此功能需  最新版本的 ethtool
### NAPI

  atlantic 驱动支持 NAPI（Rx 轮询模式）
## 受支持的 ethtool 选项


### 查看适配器设

```

    ethtool <ethX>

 Output example::

  Settings for enp1s0:
    Supported ports: [ TP ]
    Supported link modes:   100baseT/Full
			    1000baseT/Full
			    10000baseT/Full
			    2500baseT/Full
			    5000baseT/Full
    Supported pause frame use: Symmetric
    Supports auto-negotiation: Yes
    Supported FEC modes: Not reported
    Advertised link modes:  100baseT/Full
			    1000baseT/Full
			    10000baseT/Full
			    2500baseT/Full
			    5000baseT/Full
    Advertised pause frame use: Symmetric
    Advertised auto-negotiation: Yes
    Advertised FEC modes: Not reported
    Speed: 10000Mb/s
    Duplex: Full
    Port: Twisted Pair
    PHYAD: 0
    Transceiver: internal
    Auto-negotiation: on
    MDI-X: Unknown
    Supports Wake-on: g
    Wake-on: d
    Link detected: yes


 .. note::

    AQrate 速率.5/5 Gb/s）仅会在 linux 内核 > 4.10 时显示    但您仍可使用这些速率::

	ethtool -s eth0 autoneg off speed 2500

```
### 查看适配器信

```

  ethtool -i <ethX>

 Output example::

  driver: atlantic
  version: 5.2.0-050200rc5-generic-kern
  firmware-version: 3.1.78
  expansion-rom-version:
  bus-info: 0000:01:00.0
  supports-statistics: yes
  supports-test: no
  supports-eeprom-access: no
  supports-register-dump: yes
  supports-priv-flags: no


```
### 查看以太网适配器统计信

```

    ethtool -S <ethX>

 Output example::

  NIC statistics:
     InPackets: 13238607
     InUCast: 13293852
     InMCast: 52
     InBCast: 3
     InErrors: 0
     OutPackets: 23703019
     OutUCast: 23704941
     OutMCast: 67
     OutBCast: 11
     InUCastOctects: 213182760
     OutUCastOctects: 22698443
     InMCastOctects: 6600
     OutMCastOctects: 8776
     InBCastOctects: 192
     OutBCastOctects: 704
     InOctects: 2131839552
     OutOctects: 226938073
     InPacketsDma: 95532300
     OutPacketsDma: 59503397
     InOctetsDma: 1137102462
     OutOctetsDma: 2394339518
     InDroppedDma: 0
     Queue[0] InPackets: 23567131
     Queue[0] OutPackets: 20070028
     Queue[0] InJumboPackets: 0
     Queue[0] InLroPackets: 0
     Queue[0] InErrors: 0
     Queue[1] InPackets: 45428967
     Queue[1] OutPackets: 11306178
     Queue[1] InJumboPackets: 0
     Queue[1] InLroPackets: 0
     Queue[1] InErrors: 0
     Queue[2] InPackets: 3187011
     Queue[2] OutPackets: 13080381
     Queue[2] InJumboPackets: 0
     Queue[2] InLroPackets: 0
     Queue[2] InErrors: 0
     Queue[3] InPackets: 23349136
     Queue[3] OutPackets: 15046810
     Queue[3] InJumboPackets: 0
     Queue[3] InLroPackets: 0
     Queue[3] InErrors: 0

```
### 中断合并支持


```

    ethtool -c <ethX>

 and changed with::

    ethtool -C <ethX> tx-usecs <usecs> rx-usecs <usecs>

 To disable coalescing::

    ethtool -C <ethX> tx-usecs 0 rx-usecs 0 tx-max-frames 1 tx-max-frames 1

```
### Wake on LAN 支持


```

    ethtool -s <ethX> wol g

 To disable WOL::

    ethtool -s <ethX> wol d

```
### 设置并检查驱动消息级

 设置消息级别

```

    ethtool -s <ethX> msglvl <level>

 级别值：

 ======   =============================
 0x0001   通用驱动状态 0x0002   硬件探测 0x0004   链路状态 0x0008   周期性状态检查 0x0010   接口被关闭 0x0020   接口被启用 0x0040   接收错误 0x0080   发送错误 0x0200   中断处理 0x0400   发送完成 0x0800   接收完成 0x1000   数据包内容 0x2000   硬件状态 0x4000   Wake-on-LAN 状态 ======   =============================

 默认情况下，调试消息级别设为 0x0001（通用驱动状态）
 检查消息级
 ::

    ethtool <ethX> | grep "Current message level"

 如果您想禁用消息输出::

    ethtool -s <ethX> msglvl 0

```
### RX 流规则（ntuple 过滤器）


 支持以下独立的规则，按该顺序应用
 1. 16 VLAN ID 规则
 2. 16 L2 EtherType 规则
 3. 8 L3/L4 5 元组规则


 驱动利用 ethtool 接口通过 `ethtool -N <device> <filter>` 配置 ntuple 过滤器
```

    ethtool -K ethX ntuple <on|off>

 禁用 ntuple 过滤器时，所有用户编程的过滤器都会从驱动缓存和硬件中被刷新 重新启用 ntuple 后必须重新添加所有需要的过滤器
 由于规则的固定顺序，过滤器的位置也是固定的：

 - 位置 0 - 15 用于 VLAN ID 过滤 - 位置 16 - 31 用于 L2 EtherType 过滤 - 位置 32 - 39 用于 L3/L4 5 元组过滤器（位置 326 用于 IPv6
 L3/L4 5 元组（协议、源和目IP 地址、源和目TCP/UDP/SCTP 端口）与 8  过滤器进行比较。对IPv4，最多可匹配 8 个源地址和目的地址。对IPv6 最多支2 对地址。源端口和目的端口仅TCP/UDP/SCTP 数据包进行比较
 要添加一条将数据包导向队5 的过滤器，使 ``<-N|-U|--config-nfc|--config-ntuple>`` 开:

    ethtool -N <ethX> flow-type udp4 src-ip 10.0.0.1 dst-ip 10.0.0.2 src-port 2000 dst-port 2001 action 5 <loc 32>

 - action 为队列号 - loc 为规则号
 对于 ``flow-type ip4|udp4|tcp4|sctp4|ip6|udp6|tcp6|sctp6``，必须将 loc 编号
 设在 32 - 39 之间 对于 ``flow-type ip4|udp4|tcp4|sctp4|ip6|udp6|tcp6|sctp6``，您可以IPv4
 流量设置 8 条规则，或为 IPv6 流量设置 2 条规则。IPv6 流量loc 编号32
 36 目前您不能同时使IPv4 IPv6 过滤器
 IPv6 过滤流量的示:

    sudo ethtool -N <ethX> flow-type tcp6 src-ip 2001:db8:0:f101::1 dst-ip 2001:db8:0:f101::2 action 1 loc 32
    sudo ethtool -N <ethX> flow-type ip6 src-ip 2001:db8:0:f101::2 dst-ip 2001:db8:0:f101::5 action -1 loc 36

 IPv4 过滤流量的示:

    sudo ethtool -N <ethX> flow-type udp4 src-ip 10.0.0.4 dst-ip 10.0.0.7 src-port 2000 dst-port 2001 loc 32
    sudo ethtool -N <ethX> flow-type tcp4 src-ip 10.0.0.3 dst-ip 10.0.0.9 src-port 2000 dst-port 2001 loc 33
    sudo ethtool -N <ethX> flow-type ip4 src-ip 10.0.0.6 dst-ip 10.0.0.4 loc 34

 如果设置 action -1，则所有匹配该过滤器的流量都会被丢弃
 action 的最大值为 31

 VLAN 过滤器（VLAN id）与 16 个过滤器进行比较 VLAN id 必须伴随掩码 0xF000。这是为VLAN 过滤器与带有 UserPriority  L2 EtherType 过滤器区分开，因User Priority VLAN ID 都通过同一 'vlan' 参数传入
 要添加一条将来自 VLAN 2001 的数据包导向队列 5 的过滤器::

    ethtool -N <ethX> flow-type ip4 vlan 2001 m 0xF000 action 1 loc 0


 L2 EtherType 过滤器允许按 EtherType 字段，或同时802.1Q EtherType  User Priority（PCP）字段过滤数据包 UserPriority（vlan）参数必须伴随掩0x1FFF。这是为VLAN 过滤器与带有
 UserPriority L2 Ethertype 过滤器区分开，因User Priority VLAN ID
 都通过同一'vlan' 参数传入
 要添加一条将优先3 IP4 数据包导向队3 的过滤器::

    ethtool -N <ethX> flow-type ether proto 0x800 vlan 0x600 m 0x1FFF action 3 loc 16

 要查看当前存在的过滤器列:

    ethtool <-u|-n|--show-nfc|--show-ntuple> <ethX>

 规则可以从表本身删除。使用如下命令完:

    sudo ethtool <-N|-U|--config-nfc|--config-ntuple> <ethX> delete <loc>

 - loc 为要删除的规则号
 Rx 过滤器是一个将过滤表加载的接口，除非使“action指定替代队列，否 它将所有流汇入队列 0。在这种情况下，任何匹配过滤器条件的流都会被导向相应 队列。RX 过滤器在所2.6.30 及以后版本的内核上受支持
```
### UDP 鐨?RSS

 目前，NIC 不支持对分片 IP 数据包的 RSS，这会导致对分片 UDP 流量RSS
 工作不正确。要禁用 UDP RSS，可以使RX Flow L3/L4 规则
```

    ethtool -N eth0 flow-type udp4 action 0 loc 32

```
### UDP GSO 硬件卸载

 UDP GSO 通过UDP 头部分配卸载到硬件，来提UDP 发送速率。为此需要特殊的
 用户空间 socket 选项```

    udpgso_bench_tx -u -4 -D 10.0.1.1 -s 6300 -S 100

 将导致从单个 6300 字节的用户缓冲区发出 100 字节大小UDP 数据包
 UDP GSO 通过如下方式配置::

    ethtool -K eth0 tx-udp-segmentation on

```
### 私有标志（测试用

```

	$ ethtool --show-priv-flags ethX

	Private flags for ethX:
	DMASystemLoopback  : off
	PKTSystemLoopback  : off
	DMANetworkLoopback : off
	PHYInternalLoopback: off
	PHYExternalLoopback: off

 Example::

	$ ethtool --set-priv-flags ethX DMASystemLoopback on

 DMASystemLoopback:   DMA 主机回环 PKTSystemLoopback:   数据包缓冲区主机回环 DMANetworkLoopback:  DMA 块上的网络侧回环 PHYInternalLoopback: Phy 上的内部回环 PHYExternalLoopback: Phy 上的外部回环（使用回环以太网线缆）

```
## 命令行参
atlantic 驱动提供以下命令行参数：

### aq_itr - 中断节流模式

可接受值：0, 1, 0xFFFF

默认值：0xFFFF

======   ==============================================================
0        禁用中断节流1        启用中断节流并使用指定的 tx rx 速率0xFFFF   自动节流模式。驱动将根据链路速率选择最佳的 RX TX
	 中断节流设置======   ==============================================================

### aq_itr_tx - TX 中断节流速率


可接受值：0 - 0x1FF

默认值：0

以微秒计TX 侧节流。适配器会将最大中断延迟设置为此值。最小中断延迟为
此值的一半
### aq_itr_rx - RX 中断节流速率


可接受值：0 - 0x1FF

默认值：0

以微秒计RX 侧节流。适配器会将最大中断延迟设置为此值。最小中断延迟为
此值的一半

   ITR 设置可在运行时通过 ethtool -c 方式更改（见下文
## 配置文件参数


为了一些微调与性能优化，某些参数可以在 {source_dir}/aq_cfg.h 文件中更改
### AQ_CFG_RX_PAGEORDER


默认值：0

RX 页阶覆盖。这是为每个描述符分配的 RX 页数量的 2 的幂次。接收描述符大小
仍受 AQ_CFG_RX_FRAME_MAX 限制
增大页阶可以改善页复用（在启iommu 的系统上尤为明显）
### AQ_CFG_RX_REFILL_THRES


默认值：32

RX 填充阈值。RX 路径在观察到指定数量的空闲描述符之前不会填充已释放的描述符较大的值可能有助于更好地复用页，但也可能导致丢包
### AQ_CFG_VECS_DEF


队列数量

有效范围 - 8（最大到 AQ_CFG_VECS_MAX
默认值：8

注意此值会被系统中可用的核心数所限制
### AQ_CFG_IS_RSS_DEF


启用/禁用 Receive Side Scaling（接收端缩放
此特性允许适配器将接收处理分布到多CPU 核心上，以防止单CPU 核心过载
鏈夋晥鍊。
==  ========
0   禁用
1   启用
==  ========

默认值：1

### AQ_CFG_NUM_RSS_QUEUES_DEF


Receive Side Scaling 的队列数
有效范围 - 8（最大到 AQ_CFG_VECS_DEF
默认值：AQ_CFG_VECS_DEF

### AQ_CFG_IS_LRO_DEF


启用/禁用 Large Receive Offload（大型接收卸载）

此卸载使适配器能够将多个 TCP 段合并，并将其作为单个合并单元指示给操作系统
网络子系统
系统消耗更少的能量，但也引入更多的数据包处理延迟
鏈夋晥鍊。
==  ========
0   禁用
1   启用
==  ========

默认值：1

### AQ_CFG_TX_CLEAN_BUDGET


单次 TX 清理的最大描述符数量
默认值：256

修改 aq_cfg.h 文件后，必须重新构建驱动才能生效
## 支持


如果在受支持的内核上使用受支持的适配器，发现已发布源代码存在问题，请与该问题相关的具体信息通过电子邮件发送至 aqn_support@marvell.com

## 许可

aQuantia Corporation 网络驱动

Copyright |copy| 2014 - 2019 aQuantia Corporation.

本程序是自由软件；您可以在自由软件基金会发布GNU 通用公共许可证第 2 的条款和条件下重新分发和/或修改它