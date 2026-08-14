
## 面向 Intel(R) 以太网自适应虚拟功能（Adaptive Virtual Function）的 Linux 基础驱动

Intel 以太网自适应虚拟功能 Linux 驱动。
Copyright(c) 2013-2018 Intel Corporation.

## 目录

- 概述
- 识别你的适配器
- 附加配置
- 已知问题/故障排查
- 支持

## 概述

本文件描述了 iavf Linux 基础驱动。该驱动以前称为 i40evf。

iavf 驱动支持下述虚拟功能设备，并且只能在运行编译了 CONFIG_PCI_IOV 的 i40e 或更新版本物理功能（PF）驱动的内核上被激活。iavf 驱动要求启用 `CONFIG_PCI_MSI`。

加载 iavf 驱动的客机操作系统必须支持 MSI-X 中断。

## 识别你的适配器

本内核中的驱动与基于以下设备的产品兼容：
 - Intel(R) XL710 X710 虚拟功能
 - Intel(R) X722 虚拟功能
 - Intel(R) XXV710 虚拟功能
 - Intel(R) 以太网自适应虚拟功能

为了获得最佳性能，请确保设备上安装了最新的 NVM/FW。

有关如何识别你的适配器，以及最新的 NVM/FW 镜像和 Intel 网络驱动的信息，请参阅 Intel 支持网站：
https://www.intel.com/support


## 附加特性与配置

### 查看链路消息

如果发行版限制了系统消息，链路消息将不会显示到控制台。为了看到网络驱动链路消息，请使用

```
    # dmesg -n 8

```
注意：
  该设置不会在重启后保留。

### ethtool

该驱动利用 ethtool 接口进行驱动配置和诊断，以及显示统计信息。此功能需要最新版本的 ethtool。在此处下载：
https://www.kernel.org/pub/software/network/ethtool/

### 设置 VLAN 标签剥离

如果你有要求虚拟功能（VF）接收带有 VLAN 标签的数据包的应用，你可以为 VF 禁用 VLAN 标签剥离。物理功能（PF）处理来自 VF 的启用或禁用 VLAN 标签剥离的请求。注意，如果 PF 已经为某个 VF 分配了 VLAN，那么来自该 VF 的设置 VLAN 标签剥离的请求将被忽略。

要启用/禁用某个 VF 的 VLAN 标签剥离，发出以下命令

```
    # ethtool -K <if_name> rxvlan on/off

```
```
    # ethtool --offload <if_name> rxvlan on/off

```
### 自适应虚拟功能（Adaptive Virtual Function）

自适应虚拟功能（AVF）允许虚拟功能驱动（即 VF）适应与其关联的物理功能驱动（PF）不断变化的能力集合。这使得系统管理员可以在不必更新与其关联的所有 VF 的情况下更新 PF。所有 AVF 都有单一的通用设备 ID 和品牌字符串。

AVF 具有一组被称为 “base mode（基本模式）” 的最小特性集，但可能根据与其关联的 PF 中可用的特性提供额外特性。以下是基本模式特性：

- 4 个队列对（QP）以及用于 Tx/Rx 的关联配置状态寄存器（CSR）
- i40e 描述符和环格式
- 描述符写回完成
- 1 个控制队列，使用 i40e 描述符、CSR 和环格式
- 5 个 MSI-X 中断向量及相应的 i40e CSR
- 1 个中断节流率（ITR）索引
- 每个 VF 1 个虚拟站点接口（VSI）
- 1 个流量类别（TC），即 TC0
- 接收端缩放（RSS），带有 64 表项的间接表和密钥，通过 PF 配置
- 每个 VF 保留 1 个单播 MAC 地址
- 每个 VF 16 个 MAC 地址过滤器
- 无状态卸载——非隧道校验和
- AVF 设备 ID
- HW 邮箱用于 VF 到 PF 的通信（包括 Windows 上）

### IEEE 802.1ad（QinQ）支持

IEEE 802.1ad 标准（非正式称为 QinQ）允许在单个以太网帧中包含多个 VLAN ID。VLAN ID 有时被称为 “tags（标签）”，因此多个 VLAN ID 被称为 “tag stack（标签栈）”。标签栈允许 L2 隧道以及在特定 VLAN ID 内隔离流量等用途。

```
    # ip link add link eth0 eth0.24 type vlan proto 802.1ad id 24
    # ip link add link eth0.24 eth0.24.371 type vlan proto 802.1Q id 371

```
其中 “24” 和 “371” 是示例 VLAN ID。

注意：
  对于 802.1ad（QinQ）数据包，不支持接收校验和卸载、云过滤器（cloud filters）和 VLAN 加速。

### 应用设备队列（ADq，Application Device Queues）

应用设备队列（ADq）允许你将一个或多个队列专用于特定应用。这可以减少指定应用的延迟，并允许按应用对 Tx 流量进行限速。按照以下步骤设置 ADq。

要求：

- 必须加载 sch_mqprio、act_mirred 和 cls_flower 模块
- 最新版本的 iproute2
- 如果另一个驱动（例如 DPDK）已经设置了云过滤器，则无法启用 ADQ
- 根据底层 PF 设备，当启用以下特性时无法启用 ADQ：

  - 数据中心桥接（DCB）
  - 每端口多功能（MFP）
  - 边带过滤器（Sideband Filters）

1. 创建流量类别（TC）。每个接口最多可创建 8 个 TC。shaper bw_rlimit 参数是可选的。

示例：建立两个 TC，tc0 和 tc1，每个 16 个队列，tc0 最大 tx 速率设为 1Gbit，tc1 设为 3Gbit。

```
    tc qdisc add dev <interface> root mqprio num_tc 2 map 0 0 0 0 1 1 1 1
    queues 16@0 16@16 hw 1 mode channel shaper bw_rlimit min_rate 1Gbit 2Gbit
    max_rate 1Gbit 3Gbit

```
map：将最多 16 个优先级映射到 TC（例如 map 0 0 0 0 1 1 1 1 将优先级 0-3 设为使用 tc0，4-7 设为使用 tc1）

queues：对于每个 TC，<队列数>@<偏移>（例如 queues 16@0 16@16 将 16 个队列分配给偏移 0 处的 tc0，将 16 个队列分配给偏移 16 处的 tc1。所有 TC 的队列总数最多为 64 或核心数，取两者中较小者。）

hw 1 mode channel：在 mqprio 中，将 ‘hw’ 设为 1 的 ‘channel’ 是一种新的硬件卸载模式，它充分利用了 mqprio 选项、TC、队列配置和 QoS 参数。

shaper bw_rlimit：对于每个 TC，设置最小和最大带宽速率。总和必须小于或等于端口速率。

例如：min_rate 1Gbit 3Gbit：使用网络监控工具（如 `ifstat` 或 `sar -n DEV [interval] [number of samples]`）验证带宽限制

注意：
  当 TC 使用 mqprio 配置时，通过 ethtool（ethtool -L）设置通道不受支持。

```
    # ethtool -K <interface> hw-tc-offload on

```
```
    # tc qdisc add dev <interface> ingress

```
注意：
 - 从 iproute2 <pathtoiproute2>/tc/ 目录运行所有 tc 命令
 - ADq 与云过滤器不兼容
 - 当 TC 使用 mqprio 配置时，通过 ethtool（ethtool -L）设置通道不受支持
 - 你必须拥有最新版本的 iproute2
 - 需要 NVM 版本 6.01 或更高
 - 当启用以下任何特性时，无法启用 ADQ：数据中心桥接（DCB）、每端口多功能（MFP）或边带过滤器
 - 如果另一个驱动（例如 DPDK）已经设置了云过滤器，则无法启用 ADq
 - ADq 不支持隧道过滤器。如果封装的数据包以非隧道模式到达，过滤将在内部头部上进行。例如，对于非隧道模式下的 VXLAN 流量，PCTYPE 被识别为 VXLAN 封装的数据包，外层头部被忽略。因此，匹配的是内部头部。
 - 如果 PF 上的 TC 过滤器匹配了经过 VF（在 PF 上）的流量，该流量将被路由到 PF 的相应队列，而不会传递给 VF。由于该流量与 PF 地址数据不匹配，它将在 TCP/IP 协议栈更上层被丢弃。
 - 如果流量匹配了指向不同 TC 的多个 TC 过滤器，该流量将被复制并发送到所有匹配的 TC 队列。当匹配多个过滤器时，硬件交换机会将数据包镜像到 VSI 列表。


## 已知问题/故障排查

### 与绑定到 Intel(R) 以太网控制器 700 系列设备的 VF 绑定时失败

如果你将虚拟功能（VF）绑定到基于 Intel(R) 以太网控制器 700 系列的设备，当这些 VF 从设备变成活动从设备（active slave）时可能会失败。如果 VF 的 MAC 地址由设备的物理功能（PF）设置，当你添加从设备，或更改主备（active-backup）从设备时，Linux bonding 会尝试将备份从设备的 MAC 地址同步为与活动从设备相同的 MAC 地址。Linux bonding 此时将失败。如果 VF 的 MAC 地址不是由 PF 设置的，则不会发生此问题。

### 流量未在 VM 与客户端之间传递

如果虚拟功能（VF，或虚拟 NIC）不处于可信模式且启用了伪造检查（spoof checking），你可能无法在客户端系统和运行于独立主机上的虚拟机（VM）之间传递流量。注意，这种情况可能发生在客户端、主机和客机操作系统的任意组合中。有关如何将 VF 设置为可信模式的信息，请参阅本 readme 文档中的 “VLAN Tag Packet Steering” 一节。有关设置伪造检查的信息，请参阅本 readme 文档中的 “MAC and VLAN anti-spoofing feature” 一节。

### 如果绑定了带活动 VM 的 VF，不要卸载端口驱动

如果虚拟功能（VF）带有活动的虚拟机（VM）并绑定到某个端口，不要卸载该端口的驱动。这样做会导致该端口看起来像挂起。一旦 VM 关闭，或以其他方式释放了 VF，命令就会完成。

### 使用四个流量类别会失败

不要在 iavf 驱动中尝试保留超过三个流量类别。这样做将无法设置任何流量类别，并会导致驱动将错误写入 stdout。最多使用三个队列以避免此问题。

### 移除 iavf 驱动时出现多条日志错误消息

如果你有多个 VF 并移除 iavf 驱动，会出现若干如下实例：

```
    Unable to send opcode 2 to PF, err I40E_ERR_QUEUE_EMPTY, aq_err ok
    Unable to send the message to VF 2 aq_err 12
    ARQ Overflow Error detected

```
### 虚拟机未获得链路

如果虚拟机分配了多个虚拟端口，并且这些虚拟端口绑定到不同的物理端口，你可能在某些虚拟端口上无法获得链路。

```
    # ethtool -r <PF>

```
其中 <PF> 是主机中的 PF 接口，例如：p5p1。你可能需要多次运行该命令才能在所有虚拟端口上获得链路。

### 虚拟功能的 MAC 地址意外改变

如果虚拟功能的 MAC 地址未在主机中分配，则 VF（虚拟功能）驱动将使用随机 MAC 地址。该随机 MAC 地址可能在每次重新加载 VF 驱动时改变。你可以在主机机器中分配一个静态 MAC 地址。该静态 MAC 地址将在 VF 驱动重新加载后仍然存在。

### 驱动缓冲区溢出修复

解决 CVE-2016-8105 的修复（在 Intel SA-00069 中引用 https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00069.html）已包含在本版本及未来版本的驱动中。

### 同一以太网广播网络上的多个接口

由于 Linux 上默认的 ARP 行为，不可能让同一以太网广播域（未分区的交换机）中两个 IP 网络上的单个系统表现如预期。所有以太网接口都会响应分配给该系统的任何 IP 地址的 IP 流量。这导致接收流量不平衡。

如果你在服务器上有多个接口，要么通过以下方式开启 ARP 过滤：

```
    # echo 1 > /proc/sys/net/ipv4/conf/all/arp_filter

```
注意：
  该设置不会在重启后保留。配置更改可以通过

```
    net.ipv4.conf.all.arp_filter = 1

```
另一种替代方案是将接口安装在独立的广播域中（在不同的交换机中，或在分区为 VLAN 的交换机中）。

### Rx 页分配错误

在压力下可能会出现 ‘Page allocation failure. order:0’ 错误。这是由 Linux 内核报告这种压力状况的方式引起的。


## 支持

有关一般信息，请访问 Intel 支持网站：
https://support.intel.com

如果在受支持的内核上，使用受支持的适配器，已发布的源代码中被发现存在问题时，请将与该问题相关的具体信息通过电子邮件发送至 intel-wired-lan@lists.osuosl.org。
