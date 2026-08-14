## DPAA2（Data Path Acceleration Architecture Gen2，第二代数据通路加速架构）概述


:Copyright: |copy| 2015 Freescale Semiconductor Inc.
:Copyright: |copy| 2018 NXP

本文档概述了 Freescale DPAA2 架构，以及它如何集成到 Linux 内核中。

## 简介


DPAA2 是一种为高速网络数据包处理而设计的硬件架构。DPAA2 包含用于以太网数据包处理、队列管理、缓冲区管理、自主 L2 交换、虚拟以太网桥接，以及加速器（例如加密）共享的复杂机制。

一个名为管理复杂体（Management Complex，简称 MC）的 DPAA2 硬件组件负责管理 DPAA2 硬件资源。MC 为软件驱动提供基于对象的抽象，以便使用 DPAA2 硬件。MC 使用 DPAA2 硬件资源（例如队列、缓冲池和网络端口）来创建功能对象/设备，例如网络接口、L2 交换机或加速器实例。MC 提供内存映射 I/O 命令接口（MC portal，MC 端口），DPAA2 软件驱动通过这些接口来操作 DPAA2 对象。

下图展示了 DPAA2 资源管理的总体概况
```

	+--------------------------------------+
	|                  OS                  |
	|                        DPAA2 drivers |
	|                             |        |
	+-----------------------------|--------+
	                              |
	                              | (create,discover,connect
	                              |  config,use,destroy)
	                              |
	                 DPAA2        |
	+------------------------| mc portal |-+
	|                             |        |
	|   +- - - - - - - - - - - - -V- - -+  |
	|   |                               |  |
	|   |   Management Complex (MC)     |  |
	|   |                               |  |
	|   +- - - - - - - - - - - - - - - -+  |
	|                                      |
	| Hardware                  Hardware   |
	| Resources                 Objects    |
	| ---------                 -------    |
	| -queues                   -DPRC      |
	| -buffer pools             -DPMCP     |
	| -Eth MACs/ports           -DPIO      |
	| -network interface        -DPNI      |
	|  profiles                 -DPMAC     |
	| -queue portals            -DPBP      |
	| -MC portals                ...       |
	|  ...                                 |
	|                                      |
	+--------------------------------------+


```
MC 负责中介诸如创建、发现、连接、配置和销毁之类的操作。针对数据的快速路径（fast-path）操作（例如数据包的发送/接收）不由 MC 中介，而是直接使用 DPIO 对象中的内存映射区域完成。

## DPAA2 对象概述


本节简要概述一些关键的 DPAA2 对象。文中描述了一个简单场景，以说明创建网络接口所涉及的对象。

### DPRC（Datapath Resource Container，数据通路资源容器）

DPRC 是一个容器对象，持有所有其他类型的 DPAA2 对象。在下面的示例图中，容器内有 8 个对象，分属 5 种类型（DPMCP、DPIO、DPBP、DPNI 和 DPMAC）。
```

	+---------------------------------------------------------+
	| DPRC                                                    |
	|                                                         |
	|  +-------+  +-------+  +-------+  +-------+  +-------+  |
	|  | DPMCP |  | DPIO  |  | DPBP  |  | DPNI  |  | DPMAC |  |
	|  +-------+  +-------+  +-------+  +---+---+  +---+---+  |
	|  | DPMCP |  | DPIO  |                                   |
	|  +-------+  +-------+                                   |
	|  | DPMCP |                                              |
	|  +-------+                                              |
	|                                                         |
	+---------------------------------------------------------+

```
从操作系统的角度来看，DPRC 的行为类似于即插即用总线，例如 PCI。DPRC 命令可用于枚举 DPRC 的内容，发现存在的硬件对象（包括可映射区域和中断）。
```

	DPRC.1 (bus)
	   |
	   +--+--------+-------+-------+-------+
	      |        |       |       |       |
	    DPMCP.1  DPIO.1  DPBP.1  DPNI.1  DPMAC.1
	    DPMCP.2  DPIO.2
	    DPMCP.3

```
硬件对象可以动态创建和销毁，从而提供向 DPRC 热插拔/热拔除对象的能力。

DPRC 有一个可映射的 MMIO 区域（一个 MC portal），可用于发送 MC 命令。它有一个用于状态事件（例如热插拔）的中断。容器中的所有对象共享相同的硬件"隔离上下文"。这意味着，相对于 IOMMU 而言，隔离粒度是在 DPRC（容器）级别，而不是在单个对象级别。

DPRC 可以静态定义，并通过在固件启动 MC 时传入的配置文件来填充对象。

### 用于以太网网络接口的 DPAA2 对象


典型的以太网 NIC 是单体的——NIC 设备包含 TX/RX 排队机制、配置机制、缓冲区管理、物理端口和中断。DPAA2 采用了更细粒度的方法，利用多个硬件对象。每个对象提供专门的功能。软件使用这些对象的组合来提供以太网网络接口功能。这种方法能高效利用有限的硬件资源，并提供灵活性与性能优势。

下图展示了在一个包含 2 个 CPU 的系统上，用于简单网络接口配置所需的对象。
```

	+---+---+ +---+---+
	   CPU0     CPU1
	+---+---+ +---+---+
	    |         |
	+---+---+ +---+---+
	   DPIO     DPIO
	+---+---+ +---+---+
	    \     /
	     \   /
	      \ /
	   +---+---+
	      DPNI  --- DPBP,DPMCP
	   +---+---+
	       |
	       |
	   +---+---+
	     DPMAC
	   +---+---+
	       |
	   port/PHY

```
下面描述了这些对象。对于每个对象，都会提供简要描述，以及该对象所支持的操作种类和其关键资源（MMIO 区域和 IRQ）的摘要。

#### DPMAC（Datapath Ethernet MAC，数据通路以太网 MAC）

表示一个以太网 MAC，即连接到以太网 PHY 并允许以太网帧进行物理收发的一种硬件设备。

- MMIO 区域：无
- IRQ：DPNI 链路状态变化（link change）
- 命令：设置链路 up/down、链路配置、获取统计信息、IRQ 配置、使能、复位

#### DPNI（Datapath Network Interface，数据通路网络接口）

包含 TX/RX 队列、网络接口配置，以及 RX 缓冲池配置机制。TX/RX 队列位于内存中，由队列号标识。

- MMIO 区域：无
- IRQ：链路状态（link state）
- 命令：端口配置、卸载（offload）配置、队列配置、解析/分类配置、IRQ 配置、使能、复位

#### DPIO（Datapath I/O，数据通路 I/O）

提供入队（enqueue）和出队（dequeue）数据包，以及执行硬件缓冲池管理操作的接口。DPAA2 架构将访问队列的机制（DPIO 对象）与队列本身分离开来。DPIO 提供用于入队/出队数据包的 MMIO 接口。要入队某个内容，需要向 DPIO MMIO 区域写入一个描述符，其中包含目标队列号。通常会为每个 CPU 分配一个 DPIO。这使得所有 CPU 都能同时执行入队/出队操作。DPIO 预期会被不同的 DPAA2 驱动所共享。

- MMIO 区域：队列操作、缓冲管理
- IRQ：数据可用性（data availability）、拥塞通知（congestion notification）、缓冲池耗尽（buffer pool depletion）
- 命令：IRQ 配置、使能、复位

#### DPBP（Datapath Buffer Pool，数据通路缓冲池）

表示一个硬件缓冲池。

- MMIO 区域：无
- IRQ：无
- 命令：使能、复位

#### DPMCP（Datapath MC Portal，数据通路 MC 端口）

提供 MC 命令端口（MC command portal）。供驱动向 MC 发送命令以管理对象。

- MMIO 区域：MC 命令端口
- IRQ：命令完成（command completion）
- 命令：IRQ 配置、使能、复位

## 对象连接

某些对象之间存在必须配置的显式关系：

- DPNI <--> DPMAC
- DPNI <--> DPNI
- DPNI <--> L2-switch-port

    DPNI 必须连接到某个对象，例如 DPMAC、另一个 DPNI，或 L2 交换机端口。DPNI 连接是通过 DPRC 命令建立的。
```

              +-------+  +-------+
              | DPNI  |  | DPMAC |
              +---+---+  +---+---+
                  |          |
                  +==========+

```
- DPNI <--> DPBP

    网络接口需要一个"缓冲池"（DPBP 对象），它提供一个指向内存的指针列表，接收到的以太网数据将被复制到这些内存中。以太网驱动配置与该网络接口相关联的 DPBP。

## 中断

DPAA2 对象产生的所有中断都是消息中断（message interrupt）。在硬件层面，设备产生的消息中断通常包含 3 个组成部分——1) 在硬件总线上表达的不可伪造的"device-id"，2) 一个地址，3) 一个数据值。

对于 DPAA2 设备/对象，同一容器/DPRC 中的所有对象共享相同的"device-id"。对于基于 ARM 的 SoC，这与流 ID（stream ID）相同。


## DPAA2 Linux 驱动概述


本节概述了用于 DPAA2 的 Linux 内核驱动——1) 总线驱动及相关的"DPAA2 基础设施"驱动，以及 2) 功能对象驱动（例如以太网）。

如前所述，DPRC 是一个持有其他类型 DPAA2 对象的容器。它在功能上类似于即插即用总线控制器。DPRC 中的每个对象都是一个 Linux"设备"，并被绑定到一个驱动。下图展示了网络场景中涉及的 Linux 驱动，以及绑定到每个驱动的对象。随后是对每个驱动的简要描述。
```

	                                     +------------+
	                                     | OS Network |
	                                     |   Stack    |
	         +------------+              +------------+
	         | Allocator  |. . . . . . . |  Ethernet  |
	         |(DPMCP,DPBP)|              |   (DPNI)   |
	         +-.----------+              +---+---+----+
	          .          .                   ^   |
	         .            .     <data avail, |   | <enqueue,
	        .              .     tx confirm> |   | dequeue>
	+-------------+         .                |   |
	| DPRC driver |          .           +---+---V----+     +---------+
	|   (DPRC)    |           . . . . . .| DPIO driver|     |   MAC   |
	+----------+--+                      |  (DPIO)    |     | (DPMAC) |
	           |                         +------+-----+     +-----+---+
	           |<dev add/remove>                |                 |
	           |                                |                 |
	  +--------+----------+                     |              +--+---+
	  |   MC-bus driver   |                     |              | PHY  |
	  |                   |                     |              |driver|
	  |   /bus/fsl-mc     |                     |              +--+---+
	  +-------------------+                     |                 |
	                                            |                 |
	========================= HARDWARE =========|=================|======
	                                          DPIO                |
	                                            |                 |
	                                          DPNI---DPBP         |
	                                            |                 |
	                                          DPMAC               |
	                                            |                 |
	                                           PHY ---------------+
	============================================|========================

```
下面简要描述每个驱动。

### MC-bus 驱动

MC-bus 驱动是一个平台驱动（platform driver），由启动固件传入的设备树节点（compatible 为 "fsl,qoriq-mc"）探测到。它负责引导 DPAA2 内核基础设施。关键功能包括：

- 在内核中注册一个名为 "fsl-mc" 的新总线类型，并实现总线回调（例如 match/uevent/dev_groups）
- 实现用于 DPAA2 驱动注册以及设备 add/remove 的 API
- 创建一个 MSI IRQ 域
- 执行 "device add" 以暴露"根"DPRC，进而触发将根 DPRC 绑定到 DPRC 驱动

MC-bus 设备树节点的绑定可参考 **Documentation/devicetree/bindings/misc/fsl,qoriq-mc.yaml**。MC-bus 的 sysfs bind/unbind 接口可参考 **Documentation/ABI/testing/sysfs-bus-fsl-mc**。

### DPRC 驱动

DPRC 驱动被绑定到 DPRC 对象，并对总线实例进行运行时管理。它执行 DPRC 的初始总线扫描，并处理容器事件（例如热插拔）的中断，方法是重新扫描 DPRC。

### Allocator（分配器）

某些对象（例如 DPMCP 和 DPBP）是通用且可互换的，旨在供其他驱动使用。例如，DPAA2 以太网驱动需要：

- DPMCP 用于发送 MC 命令，以配置网络接口
- DPBP 用于网络缓冲池

分配器驱动为这些可分配对象类型进行注册，当总线被探测时，这些对象被绑定到分配器。分配器维护一个可供其他 DPAA2 驱动分配的对象池。

### DPIO 驱动

DPIO 驱动被绑定到 DPIO 对象，并提供服务，使得其他驱动（例如以太网驱动）能够为其各自的对象入队和出队数据。关键服务包括：

- 数据可用性通知（data availability notifications）
- 硬件排队操作（数据的入队和出队）
- 硬件缓冲池管理

要发送一个数据包，以太网驱动将数据放到一个队列上，并调用一个 DPIO API。对于接收，以太网驱动注册一个数据可用性通知回调。要出队一个数据包，则使用一个 DPIO API。

为了获得最佳性能，通常每个物理 CPU 有一个 DPIO 对象，允许不同的 CPU 同时入队和出队数据。

DPIO 驱动代表内核中所有活跃的 DPAA2 驱动——以太网、加密、压缩等——进行工作。

### 以太网驱动

以太网驱动被绑定到一个 DPNI，并实现将 DPAA2 网络接口连接到网络栈所需的内核接口。每个 DPNI 对应一个 Linux 网络接口。

### MAC 驱动

以太网 PHY 是一个片外、与板卡相关的组件，由相应的 PHY 驱动通过 mdio 总线管理。MAC 驱动充当 PHY 驱动与 MC 之间的代理（proxy）。它借助发往 DPMAC 对象的 MC 命令来完成这种代理。如果 PHY 驱动发出链路变化的信号，MAC 驱动会通过 DPMAC 命令通知 MC。如果某个网络接口被启用或禁用，MC 会通过中断通知 DPMAC 驱动，驱动可以采取相应动作。
