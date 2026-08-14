
## Marvell OcteonTx2 RVU 内核驱动


Copyright (c) 2020 Marvell International Ltd.

## 目录


- `Overview`_
- `Drivers`_
- `Basic packet flow`_
- `Devlink health reporters`_
- `Quality of service`_
- `RVU representors`_

## 概述


Marvell 的 OcteonTX2 SOC 上的资源虚拟化单元（RVU）将来自网络、加密以及其他
功能块的硬件资源映射为 PCI 兼容的物理与虚拟功能。每个功能块又拥有多个本地
功能（LFs），供分配给 PCI 设备使用。RVU 支持多个 PCIe SRIOV 物理功能（PFs）与
虚拟功能（VFs）。PF0 被称为管理/管理功能（AF），并拥有将 RVU 功能块的 LFs 分配给
各个 PF/VF 的特权。

RVU 管理的网络功能块
 - 网络池或缓冲区分配器（NPA）
 - 网络接口控制器（NIX）
 - 网络解析器 CAM（NPC）
 - 调度/同步/排序单元（SSO）
 - 回环接口（LBK）

RVU 管理的非网络功能块
 - 加密加速器（CPT）
 - 调度定时器单元（TIM）
 - 调度/同步/排序单元（SSO）
   同时用于网络与非网络场景

资源分配示例
 - 带有 NIX-LF 与 NPA-LF 资源的 PF/VF 作为纯网络设备工作
 - 带有 CPT-LF 资源的 PF/VF 作为纯加密卸载设备工作

RVU 功能块可根据软件需求高度配置。

固件在内核启动前完成以下设置
 - 根据物理链路的数量启用所需数量的 RVU PF。
 - 每个 PF 的 VF 数量在编译时是静态或可调的。根据配置，固件将 VF 分配给各个
   PF。
 - 同时为每个 PF 与 VF 分配 MSIX 向量。
 - 这些在内核启动后不再改变。

## 驱动


Linux 内核将会有多个驱动注册到 RVU 的不同 PF 与 VF。就网络而言，将会有 3 种
风格的驱动。

### 管理功能驱动


如上所述，RVU PF0 被称为管理功能（AF），该驱动支持功能块的资源分配与配置。
它不处理任何 I/O。它设置少量基础事项，但大部分功能是通过来自 PF 与 VF 的配置
请求来实现的。

PF/VF 通过一段共享内存区域（邮箱）与 AF 通信。收到请求后，AF 进行资源分配以及
其他硬件配置。AF 始终挂接在主机内核上，但 PF 及其 VF 可能由主机内核自身使用，
或者被挂接到 VM 或 DPDK 等用户空间应用程序。因此 AF 必须处理来自任何域中任何
设备发送的资源分配/配置请求。

AF 驱动还与底层固件交互以
 - 管理物理以太网链路，即 CGX LMAC。
 - 获取速度、双工、自协商等信息
 - 获取 PHY EEPROM 与统计信息。
 - 配置 FEC、PAM 模式
 - 等等

从纯网络角度看，AF 驱动支持以下功能。
 - 将物理链路映射到注册了 netdev 的 RVU PF。
 - 将 NIX 与 NPA 块的 LFs 挂接到 RVU PF/VF，以提供用于常规网络功能的缓冲区池、
   RQ、SQ。
 - 流控（暂停帧）的启用/禁用/配置。
 - 与硬件 PTP 时间戳相关的配置。
 - NPC 解析器配置文件配置，即如何解析数据包以及提取什么信息。
 - NPC 提取配置文件配置，即从数据包中提取什么内容以匹配 MCAM 表项中的数据。
 - 管理 NPC MCAM 表项，在收到请求时可以为请求的包转发规则构建并安装。
 - 定义接收端缩放（RSS）算法。
 - 定义分段卸载算法（如 TSO）
 - VLAN 剥离、捕获与插入配置。
 - SSO 与 TIM 块配置，提供包调度支持。
 - Debugfs 支持，用于检查当前资源分配、NPA 池、NIX RQ、SQ 与 CQ 的当前状态、
   各种统计信息等，以帮助调试问题。
 - 以及更多。

### 物理功能驱动


该 RVU PF 处理 IO，被映射到一个物理以太网链路，并且该驱动注册一个 netdev。它
支持 SR-IOV。如上所述，该驱动通过邮箱与 AF 通信。为了从物理链路获取信息，该
驱动与 AF 交谈，AF 再从固件获取信息并回应回来，即它不能直接与固件交谈。

支持 ethtool 用于配置链路、RSS、队列数量、队列大小、流控、ntuple 过滤器、转储
PHY EEPROM、配置 FEC 等。

### 虚拟功能驱动


有两种类型的 VF，与其父 SR-IOV PF 共享物理链路的 VF，以及使用内部硬件回环通道
（LBK）成对工作的 VF。

类型 1：
 - 这些 VF 及其父 PF 共享一条物理链路，用于与外部通信。
 - VF 不能直接与 AF 通信，它们将 mbox 消息发送给 PF，PF 再将其转发给 AF。AF 处理
   之后，将回应返回给 PF，PF 再将回复转发给 VF。
 - 从功能角度看，PF 与 VF 之间没有区别，因为相同的硬件资源被挂接到两者。但用户
   只能从 PF 配置少量内容，因为 PF 被视为链路的所有者/管理员。

类型 2：
 - RVU PF0，即管理功能，创建这些 VF 并将它们映射到回环块的通道。
 - 一组两个 VF（VF0 与 VF1、VF2 与 VF3……依此类推）成对工作，即从 VF0 发出的包
   会被 VF1 接收，反之亦然。
 - 这些 VF 可被应用程序或虚拟机用来在它们之间通信而无需将流量发往外部。硬件中
   不存在交换机，因此提供了对回环 VF 的支持。
 - 这些 VF 通过 mbox 直接与 AF（PF0）通信。

除了用于包收发所用的 IO 通道或链路之外，这些 VF 类型之间没有其他区别。AF 驱动
负责 IO 通道映射，因此同一个 VF 驱动对两类设备都能工作。

## 基本包流


### 入向


1. CGX LMAC 接收数据包。
2. 将数据包转发给 NIX 块。
3. 随后提交给 NPC 块进行解析，再进行 MCAM 查找以获得目标 RVU 设备。
4. 挂接到目标 RVU 设备的 NIX LF 从 NPA 块 LF 的 RQ 映射缓冲区池中分配一个缓冲区。
5. RQ 可由 RSS 选择，或通过配置带 RQ 号的 MCAM 规则来选择。
6. 数据包被 DMA，并通知驱动。

### 出向


1. 驱动准备一个发送描述符并提交给 SQ 以进行传输。
2. 该 SQ 已被（AF）配置为在特定链路/通道上传输。
3. SQ 描述符环由从 NPA 块 LF 的 SQ 映射池中分配的缓冲区维护。
4. NIX 块在指定通道上传输该包。
5. 可以安装 NPC MCAM 表项以将包改道到不同的通道。

## Devlink 健康报告器


### NPA 报告器


NPA 报告器负责报告并恢复以下一组错误：

1. GENERAL 事件

   - 因未映射 PF 的操作导致的错误。
   - 因其他 HW 块（NIX、SSO、TIM、DPI 与 AURA）的分配/释放被禁用导致的错误。

2. ERROR 事件

   - 因 NPA_AQ_INST_S 读或 NPA_AQ_RES_S 写导致的故障。
   - AQ Doorbell 错误。

3. RAS 事件

   - 针对 NPA_AQ_INST_S/NPA_AQ_RES_S 的 RAS 错误报告。

4. RVU 事件

   - 因未映射槽位导致的错误。

```

	~# devlink health
	pci/0002:01:00.0:
	  reporter hw_npa_intr
	      state healthy error 2872 recover 2872 last_dump_date 2020-12-10 last_dump_time 09:39:09 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_gen
	      state healthy error 2872 recover 2872 last_dump_date 2020-12-11 last_dump_time 04:43:04 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_err
	      state healthy error 2871 recover 2871 last_dump_date 2020-12-10 last_dump_time 09:39:17 grace_period 0 auto_recover true auto_dump true
	   reporter hw_npa_ras
	      state healthy error 0 recover 0 last_dump_date 2020-12-10 last_dump_time 09:32:40 grace_period 0 auto_recover true auto_dump true

```
每个报告器转储出

 - 错误类型
 - 错误寄存器值
 - 文字形式的缘由

```

	~# devlink health dump show  pci/0002:01:00.0 reporter hw_npa_gen
	 NPA_AF_GENERAL:
	         NPA General Interrupt Reg : 1
	         NIX0: free disabled RX
	~# devlink health dump show  pci/0002:01:00.0 reporter hw_npa_intr
	 NPA_AF_RVU:
	         NPA RVU Interrupt Reg : 1
	         Unmap Slot Error
	~# devlink health dump show  pci/0002:01:00.0 reporter hw_npa_err
	 NPA_AF_ERR:
	        NPA Error Interrupt Reg : 4096
	        AQ Doorbell Error


```
### NIX 报告器


NIX 报告器负责报告并恢复以下一组错误：

1. GENERAL 事件

   - 因缓冲区不足导致的接收镜像/组播包丢弃。
   - SMQ Flush 操作。

2. ERROR 事件

   - 因从组播/镜像缓冲区读写 WQE 导致的内存错误。
   - 接收组播/镜像复制列表错误。
   - 在未映射的 PF 上接收数据包。
   - 因 NIX_AQ_INST_S 读或 NIX_AQ_RES_S 写导致的故障。
   - AQ Doorbell 错误。

3. RAS 事件

   - 针对 NIX 接收组播/镜像条目结构的 RAS 错误报告。
   - 针对从组播/镜像缓冲区读出的 WQE/包数据的 RAS 错误报告。
   - 针对 NIX_AQ_INST_S/NIX_AQ_RES_S 的 RAS 错误报告。

4. RVU 事件

   - 因未映射槽位导致的错误。

```

	~# ./devlink health
	pci/0002:01:00.0:
	  reporter hw_npa_intr
	    state healthy error 0 recover 0 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_gen
	    state healthy error 0 recover 0 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_err
	    state healthy error 0 recover 0 grace_period 0 auto_recover true auto_dump true
	  reporter hw_npa_ras
	    state healthy error 0 recover 0 grace_period 0 auto_recover true auto_dump true
	  reporter hw_nix_intr
	    state healthy error 1121 recover 1121 last_dump_date 2021-01-19 last_dump_time 05:42:26 grace_period 0 auto_recover true auto_dump true
	  reporter hw_nix_gen
	    state healthy error 949 recover 949 last_dump_date 2021-01-19 last_dump_time 05:42:43 grace_period 0 auto_recover true auto_dump true
	  reporter hw_nix_err
	    state healthy error 1147 recover 1147 last_dump_date 2021-01-19 last_dump_time 05:42:59 grace_period 0 auto_recover true auto_dump true
	  reporter hw_nix_ras
	    state healthy error 409 recover 409 last_dump_date 2021-01-19 last_dump_time 05:43:16 grace_period 0 auto_recover true auto_dump true

```
每个报告器转储出

 - 错误类型
 - 错误寄存器值
 - 文字形式的缘由

```

	~# devlink health dump show pci/0002:01:00.0 reporter hw_nix_intr
	 NIX_AF_RVU:
	        NIX RVU Interrupt Reg : 1
	        Unmap Slot Error
	~# devlink health dump show pci/0002:01:00.0 reporter hw_nix_gen
	 NIX_AF_GENERAL:
	        NIX General Interrupt Reg : 1
	        Rx multicast pkt drop
	~# devlink health dump show pci/0002:01:00.0 reporter hw_nix_err
	 NIX_AF_ERR:
	        NIX Error Interrupt Reg : 64
	        Rx on unmapped PF_FUNC


```
## 服务质量


### 调度中使用的硬件算法


octeontx2 硅片与 CN10K 传输接口由五个传输层级组成，从 SMQ/MDQ、TL4 到 TL1。每个
数据包会遍历 MDQ、TL4 到 TL1 各层级。每个层级包含一个队列数组以支持调度与整形。
硬件根据调度器队列的优先级使用下述算法。一旦用户创建了具有不同优先级的 tc 类，
驱动就用指定的优先级以及速率限制配置来配置分配给该类的调度器。

1. 严格优先级

      - 一旦包被提交给 MDQ，硬件会使用严格优先级选取所有具有不同优先级的活跃 MDQ。

2. 轮询（Round Robin）

      - 具有相同优先级的活跃 MDQ 使用轮询方式选取。


### 配置 HTB 卸载


```

        # ethtool -K <interface> hw-tc-offload on

```
```

        # tc qdisc add dev <interface> clsact
        # tc qdisc replace dev <interface> root handle 1: htb offload

```
```

        # tc class add dev <interface> parent 1: classid 1:1 htb rate 10Gbit prio 1

        # tc class add dev <interface> parent 1: classid 1:2 htb rate 10Gbit prio 7

```
```

        # tc class add dev <interface> parent 1: classid 1:1 htb rate 10Gbit prio 2 quantum 409600

        # tc class add dev <interface> parent 1: classid 1:2 htb rate 10Gbit prio 2 quantum 188416

        # tc class add dev <interface> parent 1: classid 1:3 htb rate 10Gbit prio 2 quantum 32768


```
## RVU Representors


RVU representor 驱动添加了对系统中为 RVU PF 的 VF 创建 representor 设备的支持。
当用户启用 switchdev 模式时，会创建 representor 设备。在设置 SRIOV numVFs 之前或
之后都可以启用 switchdev 模式。所有 representor 设备共享单个 NIXLF，但每个都拥有
专用的 Rx/Tx 队列。RVU PF representor 驱动为每个 Rx/Tx 队列对注册一个独立的 netdev。

当前硬件不支持能够进行 L2 学习与在 representee 与 representor 之间转发数据包的
内置交换机。因此，representee 与其 representor 之间的包路径是通过设置合适的 NPC
MCAM 过滤器实现的。匹配这些过滤器的传输数据包会通过硬件回环通道/接口（即，而非
从 MAC 接口发往外部）被回环。这会再次匹配已安装的过滤器并被转发。以此方式实现
representee => representor 以及 representor => representee 的包路径。这些规则在
representor 被创建时安装，并根据 representor/representee 接口状态而激活/停用。

使用示例：

```

	# devlink dev eswitch set pci/0002:1c:00.0 mode switchdev

 - List of representor devices on the system::

	# ip link show
	Rpf1vf0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether f6:43:83:ee:26:21 brd ff:ff:ff:ff:ff:ff
	Rpf1vf1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether 12:b2:54:0e:24:54 brd ff:ff:ff:ff:ff:ff
	Rpf1vf2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether 4a:12:c4:4c:32:62 brd ff:ff:ff:ff:ff:ff
	Rpf1vf3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether ca:cb:68:0e:e2:6e brd ff:ff:ff:ff:ff:ff
	Rpf2vf0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc pfifo_fast state DOWN mode DEFAULT group default qlen 1000 link/ether 06:cc:ad:b4:f0:93 brd ff:ff:ff:ff:ff:ff


```
要从系统中删除 representor 设备，将设备切换为 legacy 模式。

```

	# devlink dev eswitch set pci/0002:1c:00.0 mode legacy

```
RVU representors 可以使用 devlink 端口
（参见 Documentation/networking/devlink/devlink-port.rst <devlink_port>）接口进行管理。

```

	# devlink port
	pci/0002:1c:00.0/0: type eth netdev Rpf1vf0 flavour physical port 0 splittable false
	pci/0002:1c:00.0/1: type eth netdev Rpf1vf1 flavour pcivf controller 0 pfnum 1 vfnum 1 external false splittable false
	pci/0002:1c:00.0/2: type eth netdev Rpf1vf2 flavour pcivf controller 0 pfnum 1 vfnum 2 external false splittable false
	pci/0002:1c:00.0/3: type eth netdev Rpf1vf3 flavour pcivf controller 0 pfnum 1 vfnum 3 external false splittable false

```
## 功能属性


RVU representor 支持 representor 的功能属性。representor 的端口功能配置通过 devlink
eswitch 端口支持。

### MAC 地址配置


RVU representor 驱动支持通过 devlink 端口功能属性机制来配置 MAC 地址。（参见
Documentation/networking/devlink/devlink-port.rst）

```

	# devlink port function set pci/0002:1c:00.0/2 hw_addr 5c:a1:1b:5e:43:11
	# devlink port show pci/0002:1c:00.0/2
	pci/0002:1c:00.0/2: type eth netdev Rpf1vf2 flavour pcivf controller 0 pfnum 1 vfnum 2 external false splittable false
	function:
		hw_addr 5c:a1:1b:5e:43:11


```
## TC 卸载


rvu representor 驱动实现了使用端口 representor 卸载 tc 规则的支持。

```

	# tc filter add dev Rpf1vf0 protocol 802.1Q parent ffff: flower vlan_id 3 vlan_ethtype ipv4 skip_sw action drop

 - Redirect packets with vlan id 5 and IPv4 packets to eth1, after stripping vlan header.::

	# tc filter add dev Rpf1vf0 ingress protocol 802.1Q flower vlan_id 5 vlan_ethtype ipv4 skip_sw action vlan pop action mirred ingress redirect dev eth1

```
