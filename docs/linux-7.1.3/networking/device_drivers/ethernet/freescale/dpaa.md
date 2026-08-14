
## QorIQ DPAA 以太网驱动


作者：
- Madalin Bucur <madalin.bucur@nxp.com>
- Camelia Groza <camelia.groza@nxp.com>


 - DPAA 以太网概述
 - DPAA 以太网支持的 SoC
 - 在你的内核中配置 DPAA 以太网
 - DPAA 以太网帧处理
 - DPAA 以太网特性
 - DPAA 中断亲和性与接收端缩放（RSS）
 - 调试

## DPAA 以太网概述


DPAA 代表 Data Path Acceleration Architecture（数据通路加速架构），它是一组网络加速
IP，在 PowerPC 和 ARM64 多个代际的 SoC 上都可用。

Freescale 的 DPAA 架构由一系列支持以太网连接的硬件模块组成。该以太网驱动依赖于
Linux 内核中的以下驱动：

 - 外设访问存储单元（PAMU）（*仅 PPC 平台需要）
    drivers/iommu/fsl_*
 - 帧管理器（FMan）
    drivers/net/ethernet/freescale/fman
 - 队列管理器（QMan）、缓冲区管理器（BMan）
    drivers/soc/fsl/qbman

```

  dpaa_eth       /eth0\     ...       /ethN\
  driver        |      |             |      |
  -------------   ----   -----------   ----   -------------
       -Ports  / Tx  Rx \    ...    / Tx  Rx \
  FMan        |          |         |          |
       -MACs  |   MAC0   |         |   MACN   |
	     /   dtsec0   \  ...  /   dtsecN   \ (or tgec)
	    /              \     /              \(or memac)
  ---------  --------------  ---  --------------  ---------
      FMan, FMan Port, FMan SP, FMan MURAM drivers
  ---------------------------------------------------------
      FMan HW blocks: MURAM, MACs, Ports, SP
  ---------------------------------------------------------

```
```

	      ________________________________
  dpaa_eth   /            eth0                \
  driver    /                                  \
  ---------   -^-   -^-   -^-   ---    ---------
  QMan driver / \   / \   / \  \   /  | BMan    |
	     |Rx | |Rx | |Tx | |Tx |  | driver  |
  ---------  |Dfl| |Err| |Cnf| |FQs|  |         |
  QMan HW    |FQ | |FQ | |FQs| |   |  |         |
	     /   \ /   \ /   \  \ /   |         |
  ---------   ---   ---   ---   -v-    ---------
	    |        FMan QMI         |         |
	    | FMan HW       FMan BMI  | BMan HW |
	      -----------------------   --------

```
其中上面（以及代码中）使用的缩写为：

=============== ===========================================================
DPAA 		Data Path Acceleration Architecture（数据通路加速架构）
FMan 		DPAA 帧管理器
QMan 		DPAA 队列管理器
BMan 		DPAA 缓冲区管理器
QMI 		FMan 中的 QMan 接口
BMI 		FMan 中的 BMan 接口
FMan SP 	FMan 存储配置文件
MURAM 		FMan 中的多用户 RAM
FQ 		QMan 帧队列
Rx Dfl FQ 	默认接收 FQ
Rx Err FQ 	Rx 错误帧 FQ
Tx Cnf FQ 	Tx 确认 FQ
Tx FQs 		发送帧队列
dtsec 		datapath 三速以太网控制器（10/100/1000 Mbps）
tgec 		十千兆以太网控制器（10 Gbps）
memac 		多速率以太网 MAC（10/100/1000/10000）
=============== ===========================================================

## DPAA 以太网支持的 SoC


DPAA 驱动启用了以下 SoC 上存在的以太网控制器：

PPC
- P1023
- P2041
- P3041
- P4080
- P5020
- P5040
- T1023
- T1024
- T1040
- T1042
- T2080
- T4240
- B4860

ARM
- LS1043A
- LS1046A

## 在你的内核中配置 DPAA 以太网


```

  # arch/arm64 和 arch/powerpc 平台通用
  CONFIG_FSL_DPAA=y
  CONFIG_FSL_FMAN=y
  CONFIG_FSL_DPAA_ETH=y
  CONFIG_FSL_XGMAC_MDIO=y

  # 仅 arch/powerpc
  CONFIG_FSL_PAMU=y

  # RDB 上所用 PHY 需要的通用选项
  CONFIG_VITESSE_PHY=y
  CONFIG_REALTEK_PHY=y
  CONFIG_AQUANTIA_PHY=y

```
## DPAA 以太网帧处理


在接收（Rx）侧，传入帧的缓冲区是从专用接口缓冲区池中的缓冲区获取的。驱动初始化并
用一页大小的缓冲区填充这些池。

在发送（Tx）侧，所有被发送的帧都通过 Tx 确认帧队列返回给驱动。然后驱动负责释放这些
缓冲区。为了正确地做到这一点，在发送之前会向缓冲区添加一个指回 skb 的回指针。当缓冲
区在确认 FQ 上返回给驱动时，skb 就能被正确消费。

## DPAA 以太网特性


目前 DPAA 以太网驱动启用了 Linux 以太网驱动所需的基本特性。对高级特性的支持将逐步
添加。

该驱动对 UDP 和 TCP 具有 Rx 和 Tx 校验和卸载。目前 Rx 校验和卸载特性默认启用，且无法
通过 ethtool 控制。此外，还添加了 rx-flow-hash 和 rx-hashing。RSS 的加入为转发场景
带来了巨大的性能提升，允许由一个接口接收的不同流量流被不同的 CPU 并行处理。

该驱动支持多个带优先级的 Tx 流量类别。优先级范围从 0（最低）到 3（最高）。它们被
映射到具有严格优先级级别的硬件工作队列。每个流量类别包含 NR_CPU 个 Tx 队列。默认
情况下，仅启用一个流量类别，并使用最低优先级的 Tx 队列。可以通过 mqprio qdisc 启用
更高优先级的流量类别。例如，使用以下命令在某个接口上启用全部四个流量类别。此外，
skb 优先级级别到流量类别的映射如下：

 - 优先级 0 到 3 - 流量类别 0（低优先级）
 - 优先级 4 到 7 - 流量类别 1（中低优先级）
 - 优先级 8 到 11 - 流量类别 2（中高优先级）
 - 优先级 12 到 15 - 流量类别 3（高优先级）

```

  tc qdisc add dev <int> root handle 1: \
	 mqprio num_tc 4 map 0 0 0 0 1 1 1 1 2 2 2 2 3 3 3 3 hw 1

```
## DPAA 中断亲和性与接收端缩放


到达 DPAA Rx 队列或 DPAA Tx 确认队列的流量，在 CPU 看来是某个特定 portal 上的入口
（ingress）流量。DPAA QMan portal 中断各自亲和到某个特定 CPU。同一个 portal 中断
服务于所有 QMan portal 消费者。

默认情况下，DPAA 以太网驱动启用 RSS，利用 DPAA FMan 的 Parser 和 Keygen 模块，基于所
接收帧中存在的 IPv4/IPv6 源和目的地址以及 L4 源和目的端口的哈希，将流量分布到 128 个
硬件帧队列上。当 RSS 被禁用时，某个特定接口接收的所有流量都在默认 Rx 帧队列上接收。
默认的 DPAA Rx 帧队列被配置为将接收到的流量放入一个池通道（pool channel），允许任何
可用的 CPU portal 出队该入口流量。默认帧队列设置了 HOLDACTIVE 选项，确保来自某个队列
的流量突发由同一个 CPU 提供服务。这保证了极低的帧乱序率。其缺点是，在 RSS 未启用时，
某个特定接口接收到的流量一次只能由一个 CPU 提供服务。

为了实现 RSS，DPAA 以太网驱动额外分配一组 128 个 Rx 帧队列，这些队列以轮询方式配置到
专用通道。帧队列到 CPU 的映射现在是硬编码的，没有间接表来将某个 FQ（哈希结果）的流量
移动到另一个 CPU。到达这些帧队列之一的入口流量将到达同一个 portal，并总是由同一个 CPU
处理。这保证了流内顺序的保持以及多个流量流之间的工作负载分布。

```

	# ethtool -N fm1-mac9 rx-flow-hash tcp4 ""

```
```

	# ethtool -N fm1-mac9 rx-flow-hash udp4 sfdn

```
无法对各个协议进行独立控制，针对 tcp4|udp4|ah4|esp4|sctp4|tcp6|udp6|ah6|esp6|sctp6
中任意一个运行的命令，都会控制该接口上所有协议的 rx-flow-hashing。

除了使用 FMan Keygen 计算的哈希将流量分散到 128 个 Rx FQ 之外，DPAA 以太网驱动还会在
NETIF_F_RXHASH 特性开启（默认激活）时设置 skb 哈希值。这可以通过以下方式关闭
```

	# ethtool -K fm1-mac9 rx-hashing off
	# ethtool -k fm1-mac9 | grep hash
	receive-hashing: off
	# ethtool -K fm1-mac9 rx-hashing on
	Actual changes:
	receive-hashing: on
	# ethtool -k fm1-mac9 | grep hash
	receive-hashing: on

```
请注意，Rx 哈希依赖于该接口的 rx-flow-hashing 处于开启状态——关闭 rx-flow-hashing 也
会禁用 rx-hashing（ethtool 不会将其报告为 off，因为这取决于 NETIF_F_RXHASH 特性标志）。

## 调试


以下统计信息通过 ethtool 为每个接口导出：

 - 每个 CPU 的中断计数
 - 每个 CPU 的 Rx 数据包计数
 - 每个 CPU 的 Tx 数据包计数
 - 每个 CPU 的 Tx 确认数据包计数
 - 每个 CPU 的 Tx S/G 帧计数
 - 每个 CPU 的 Tx 错误计数
 - 每个 CPU 的 Rx 错误计数
 - 每个类型的 Rx 错误计数
 - 与拥塞相关的统计：

  - 拥塞状态
  - 处于拥塞状态的时间
  - 设备进入拥塞状态的次数
  - 按原因的丢包计数

该驱动还会在 sysfs 中导出以下信息：

 - 每种 FQ 类型的 FQ ID
	  /sys/devices/platform/soc/<addr>.fman/<addr>.ethernet/dpaa-ethernet.<id>/net/fm<nr>-mac<nr>/fqids

 - 所用缓冲区池的 ID
	  /sys/devices/platform/soc/<addr>.fman/<addr>.ethernet/dpaa-ethernet.<id>/net/fm<nr>-mac<nr>/bpids
