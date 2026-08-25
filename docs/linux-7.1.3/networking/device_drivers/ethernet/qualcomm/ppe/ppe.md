
## 面向 Qualcomm IPQ SoC 系列PPE 以太网驱


版权所(c) Qualcomm Technologies, Inc. 及其子公司

作者：Lei Wei <quic_leiwei@quicinc.com>


## 目录


- `PPE Overview`_
- `PPE Driver Overview`_
- `PPE Driver Supported SoCs`_
- `Enabling the Driver`_
- `Debugging`_


## PPE 概述


IPQ（Qualcomm Internet Processor）SoC（System-on-Chip，系统级芯片）系列是 Qualcomm 面向
Wi-Fi 接入点的网络 SoC。PPE（Packet Process Engine，包处理引擎）是 IPQ SoC 中的以太
包处理引擎
```
下面是一IPQ9574 SoC 的简化硬件框图，其中包含 PPE 引擎以及
SoC 中但位于 PPE 引擎之外的其他模块。这些模块协同工
               |netdev| |netdev| |netdev| |netdev| |netdev|  |netdev|<------|PHYLINK|
               +------+ +------+ +------+ +------+ +------+  +------+ stop  +-+-+-+-+
                                             |                                | | ^
 +-------+     +-------------------------+--------+----------------------+    | | |
 | GCC   |     |                         |  EDMA  |                      |    | | |
 +---+---+     |  PPE                    +---+----+                      |    | | |
     | clk     |                             |                           |    | | |
     +-------->| +-----------------------+------+-----+---------------+  |    | | |
               | |   Switch Core         |Port0 |     |Port7(EIP FIFO)|  |    | | |
               | |                       +---+--+     +------+--------+  |    | | |
               | |                           |               |        |  |    | | |
 +-------+     | |                    +------+---------------+----+   |  |    | | |
 |CMN PLL|     | | +---+ +---+ +----+ | +--------+                |   |  |    | | |
 +---+---+     | | |BM | |QM | |SCH | | | L2/L3  |  .......       |   |  |    | | |
 |   |         | | +---+ +---+ +----+ | +--------+                |   |  |    | | |
 |   |         | |                    +------+--------------------+   |  |    | | |
 |   |         | |                           |                        |  |    | | |
 |   v         | | +-----+-+-----+-+-----+-+-+---+--+-----+-+-----+   |  |    | | |
 | +------+    | | |Port1| |Port2| |Port3| |Port4|  |Port5| |Port6|   |  | mac| | |
 | |NSSCC |    | | +-----+ +-----+ +-----+ +-----+  +-----+ +-----+   |  |<---+ | |
 | +-+-+--+    | | |MAC0 | |MAC1 | |MAC2 | |MAC3 |  |MAC4 | |MAC5 |   |  | ops  | |
 | ^ | |clk    | | +-----+-+-----+-+-----+-+-----+--+-----+-+-----+   |  |      | |
 | | | +------>| +----|------|-------|-------|---------|--------|-----+  |      | |
 | | |         +---------------------------------------------------------+      | |
 | | |                |      |       |       |         |        |               | |
 | | |   MII clk      |      QSGMII               USXGMII   USXGMII             | |
 | | +--------------->|      |       |       |         |        |               | |
 | |                +-------------------------+ +---------+ +---------+         | |
 | |125/312.5MHz clk|       (PCS0)            | | (PCS1)  | | (PCS2)  | pcs ops | |
 | +----------------+       UNIPHY0           | | UNIPHY1 | | UNIPHY2 |<--------+ |
 +----------------->|                         | |         | |         |           |
 | 31.25MHz ref clk +-------------------------+ +---------+ +---------+           |
 |                     |     |      |      |          |          |                |
 |                +-----------------------------------------------------+         |
 |25/50MHz ref clk| +-------------------------+    +------+   +------+  | link    |
 +--------------->| |      QUAD PHY           |    | PHY4 |   | PHY5 |  |---------+
                  | +-------------------------+    +------+   +------+  | change
                  |                                                     |
                  |                       MDIO bus                      |
                  +-----------------------------------------------------+

```
CMN（Common，通用）PLL、NSSCC（Networking Sub System Clock Controller，网络子系统时钟控制器）GCC（Global Clock Controller，全局时钟控制器）模块位于 SoC 中，充当时钟提供者

UNIPHY 模块位于 SoC 中，提供 PCS（Physical Coding Sublayer，物理编码子层）XPCS0-Gigabit Physical Coding Sublayer0 千兆物理编码子层）功能，以支PPE MAC 与外PHY 之间的不同接口模式
CMN（Common）PLL、NSSCC（Networking Sub System Clock Controller，网络子系统时钟控制器）GCC（Global
Clock Controller，全局时钟控制器）模块位于 SoC 中，充当时钟提供者

UNIPHY 模块位于 SoC 中，提供 PCS（Physical Coding Sublayer，物理编码子层）
XPCS0-Gigabit Physical Coding Sublayer0 千兆物理编码子层）功能，以支PPE MAC 与外PHY 之间
不同接口模式

本文档重点描PPE 引擎PPE 驱动

PPE（Packet Process Engine）中的以太网功能由三
部分组成：交换核心（switch core）、端口封装（port wrapper）和以太DMA
下面列出将由PPE 驱动驱动的主要模块：
IPQ9574 PPE 中的交换核心最多具6 个前面板端口2 FIFO
接口。两FIFO 接口中的一个用于以太网端口与主CPU 之间
通信（使用以太网 DMA）。另一个用于与
EIP 引擎通信，该引擎用于 IPsec 卸载。在 IPQ9574 上，PPE 包含 6 GMAC/XGMAC
，可与外部以太网 PHY 连接。交换核心还包括 BM（Buffer
Management，缓冲区管理）、QM（Queue Management，队列管理）SCH（Scheduler，调度器）模块，用于支持
数据包处理
- L2
端口封装提供6 GMAC/XGMAC UNIPHY（PCS）的连接，支
SGMII/QSGMII/PSGMII/USXGMII/10G-BASER 等多种模式。IPQ9574 上支3 UNIPHY（PCS
实例
- EDMA（Ethernet DMA，以太网 DMA
以太DMA 用于在以太网子系
ARM 主机 CPU 之间收发数据包
PPE MAC 端口上接收到的数据包可以被转发到另一PPE MAC 端口。它也可以被转发到内部交换端0，从而可以使用以太网 DMA（EDMA）引擎将数据包传送给 ARM 核。以太网 DMA 驱动会将数据包传送到相应“netdevice接口
下面列出将由
PPE 驱动驱动PPE 引擎主要模块

- BM
## PPE 驱动概述
- QM

- SCH

- L2

- Makefile
- ppe.c
- MAC
- ppe_config.c
- ppe_config.h
- ppe_debugfs.c
- ppe_debugfs.h
- ppe_regs.h
- EDMA（以太网 DMA
ppe.c 文件包含主要PPE 平台驱动，并承担 PPE 交换核心模块（如 QM、BM L2）的初始化。这些硬件模块的配置 API ppe_config.c 文件中提供

ppe.h 定义PPE 设备数据结构，供 PPE 驱动函数使用
PPE MAC 端口上接收到的数据包可以转发到另一PPE MAC 端口。它也可
转发到内部交换端0，以便通过以太DMA（EDMA）引擎将数据
送达 ARM 核心。以太网 DMA 驱动会将
数据包投递到相应'netdevice' 接口
## PPE 驱动支持SoC
PPE MAC（netdevice）、PCS 与外PHY 的软件实例与
Linux PHYLINK 框架交互，以管理 PPE 端口
所连接 PHY 之间的连接以及端口链路状态。上图也展示了这一点

- IPQ9574
## PPE 驱动概述

PPE 驱动是面Qualcomm IPQ SoC 的以太网驱动。它是一个单一的平台驱动，
包含 PPE 部分和以太网 DMA 部分。PPE 部分初始化并驱动
PPE 交换核心中的各种模块（如 BM/QM/L2 模块PPE MAC）。EDMA 部分
驱动以太DMA，用于在 PPE 端口ARM 核心之间传输数据包，并启
面向 PPE 端口netdevice 驱动
  -> Device Drivers
drivers/net/ethernet/qualcomm/ppe/ 下的 PPE 驱动文件如下所列：
      -> Ethernet driver support
- Makefile
- ppe.c
- ppe.h
- ppe_config.c
- ppe_config.h
- ppe_debugfs.c
- ppe_debugfs.h
- ppe_regs.h

ppe.c 文件包含主要PPE 平台驱动，并负责初始
PPE 交换核心模块（如 QM、BM L2）。这些硬件模块的
配置 API ppe_config.c 文件中提供
PPE 硬件计数器可以通过 debugfs 接口`/sys/kernel/debug/ppe/` 目录访问
ppe.h 定义PPE 设备数据结构，供 PPE 驱动函数使用