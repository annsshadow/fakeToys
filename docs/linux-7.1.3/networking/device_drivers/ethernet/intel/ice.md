
## Intel(R) 以太网控制器 800 系列 Linux 基础驱动


Intel ice Linux 驱动版权所c) 2018-2021 英特尔公司
## 目录


- 概述
- 识别您的适配- 重要说明
- 附加功能与配- 性能优化


此驱动对应的虚拟功能（VF）驱动为 iavf
驱动信息可通过 ethtool lspci 获取
有关硬件要求的疑问，请参阅随 Intel 适配器提供的文档。列出的所有硬件要求均适用Linux 环境
本驱动支XDP（Express Data Path，快速数据路径）AF_XDP 零拷贝。注意对于帧大小超过 3KB 的情况，XDP 会被禁用

## 识别您的适配
有关如何识别适配器以及获取最Intel 网络驱动的信息，请参Intel 支持网站https://www.intel.com/support


## 重要说明


### 接收压力可能导致丢包

基于 Intel(R) 以太网控制器 800 系列的设备设计用于在 PCIe DMA 事务期间容忍有限的系统延迟。如果这些事务耗时超过可容忍的延迟，就会影响数据包在设备及相关内存中的缓冲时长，可能导致丢包。在标准工作负载下，这些丢包通常不会对吞吐量和性能产生明显影响
如果这些丢包似乎影响了您的工作负载，以下措施可能改善情况
1) 确保系统物理内存处于高性能配置，如平台供应商所建议。常见的建议是所有通道均插满单DIMM 模块2) 在系统的 BIOS/UEFI 设置中选择"性能"配置档3) 您的发行版可能提供诸"tuned" 之类的工具，可帮助调整内核设置，为不同工作负载获得更好的标准设置

### 配置 SR-IOV 以提升网络安全
在虚拟化环境中，在支SR-IOV Intel(R) 以太网网络适配器上，虚拟功能（VF）可能会遭受恶意行为。软件生成的二层帧，IEEE 802.3x（链路流控）、IEEE 802.1Qbb（基于优先级的流控）以及其他同类型帧，是预期之外的，并可能扼制主机与虚拟交换机之间的流量，降低性能。为解决这个问题并确保与意外流量流的隔离，请PF 的管理接口配置所有启SR-IOV 的端口进VLAN 标记。该配置可丢弃意外且可能恶意的帧
有关配置说明，请参阅本文档后文的"在启SR-IOV 的适配器端口上配置 VLAN 标记"

### 若绑定了活动虚拟机的 VF 绑定到端口，请勿卸载端口驱动

如果某虚拟功能（VF）绑定了活动虚拟机（VM），请勿卸载该端口的驱动。这样做会导致端口看似挂起。一VM 关闭或以其他方式释放VF，命令才会完成

## 附加功能与配

### ethtool

驱动使用 ethtool 接口进行驱动配置和诊断，并显示统计信息。此功能需要最新版本的 ethtool。下载地址https://kernel.org/pub/software/network/ethtool/

注意：由于设备会剥离 4 字节 CRC，ethtool rx_bytes 值与 Netdev rx_bytes 值不一致。两rx_bytes 值的差值为 Rx 数据包数量的 4 倍。例如，Rx 数据包为 10 个且 Netdev（软件统计）显示 rx_bytes "X"，则 ethtool（硬件统计）将显rx_bytes "X+40" 字节 CRC × 10 个数据包）
### ethtool 复位

驱动支持 3 种类型的复位
- PF 复位 - 仅复位与给定 PF 相关的组件，不影响其PF

- CORE 复位 - 整个适配器受影响，复位所PF

- GLOBAL 复位 - CORE 相同，但 mac phy 组件也会被重新初始化

这些对应ethtool 复位标志如下
- PF 复位
  # ethtool --reset <ethX> irq dma filter offload

- CORE 复位
  # ethtool --reset <ethX> irq-shared dma-shared filter-shared offload-shared \
  ram-shared

- GLOBAL 复位
  # ethtool --reset <ethX> irq-shared dma-shared filter-shared offload-shared \
  mac-shared phy-shared ram-shared

switchdev 模式下，您可以使用端口代表（port representor）复VF
  # ethtool --reset <repr> irq dma filter offload


### 查看链路消息

如果发行版限制了系统消息，链路消息将不会显示到控制台。要在网络上查看网络驱动链路消息

```

  # dmesg -n 8

```
注意：此设置不会在重启后保留

### 动态设备个性化（DDP
动态设备个性化（DDP）允许您在运行时通过向设备应用配置文件包来更改设备的数据包处理流水线。例如，配置文件可用于添加对新协议的支持、更改现有协议或更改默认设置。DDP 配置文件也可以在不重启系统的情况下回滚
DDP 包在设备初始化期间加载。驱动会在固件根目录（通常`/lib/firmware/` `/lib/firmware/updates/`）中查找 `intel/ice/ddp/ice.pkg`，并检查其是否包含有效DDP 包文件
注意：您的发行版很可能已提供最新的 DDP 文件，但如果缺少 ice.pkg，您可以linux-firmware 仓库或从 intel.com 找到它
如果驱动无法加载 DDP 包，设备将进入安全模式（Safe Mode）。安全模式会禁用高级和性能特性，仅支持基本流量和最小功能，例如更新 NVM 或下载新驱动DDP 包。安全模式仅适用于受影响的物理功能，不会影响任何其他 PF。有DDP 和安全模式的更多详细信息，请参阅"Intel(R) 以太网适配器和设备用户指南"
注意
- 如果您遇DDP 包文件的问题，可能需要下载更新的驱动DDP 包文件。有关更多信息，请参阅日志消息
- ice.pkg 文件是指向默DDP 包文件的符号链接
- 如果任何 PF 驱动已加载，您无法更DDP 包。要覆盖某个包，请卸载所PF，然后使用新包重新加载驱动
- 每个设备只有第一个加载的 PF 才能下载该设备的包
您可以在同一系统中为不同的物理设备安装特定的 DDP 包文件。要安装特定DDP 包文件：

1. 下载您设备所需DDP 包文件
2. 将文件重命名ice-xxxxxxxxxxxxxxxx.pkg，其'xxxxxxxxxxxxxxxx' 是要下载该包的设备唯一64 PCI Express 设备序列号（十六进制）。文件名必须包含完整的序列号（包括前导零）且全部小写。例如，64 位序列号b887a3ffffca0568，则文件名应ice-b887a3ffffca0568.pkg
   要从 PCI 总线地址查找序列号，可以使用以下命令
```

     # lspci -vv -s af:00.0 | grep -i Serial
     Capabilities: [150 v1] Device Serial Number b8-87-a3-ff-ff-ca-05-68

   您可以使用以下命令将序列号格式化（去掉短横线）：

     # lspci -vv -s af:00.0 | grep -i Serial | awk '{print $7}' | sed s/-//g
     b887a3ffffca0568

```
3. 将重命名后的 DDP 包文件复制到 `/lib/firmware/updates/intel/ice/ddp/`。如果该目录尚不存在，请在复制文件前创建它
4. 卸载设备上所有的 PF
5. 使用新包重新加载驱动
注意：设备特定的 DDP 包文件的存在会覆盖默DDP 包文件（ice.pkg）的加载

### Intel(R) 以太网流导向
Intel 以太网流导向器执行以下任务：

- 根据数据流将接收数据包导向不同队- 实现对平台中数据流路由的紧密控制
- 将流CPU 核心匹配以实现流亲和

注意：本驱动支持以下流类型：

- IPv4
- TCPv4
- UDPv4
- SCTPv4
- IPv6
- TCPv6
- UDPv6
- SCTPv6

每种流类型支IP 地址（源或目的）UDP/TCP/SCTP 端口（源和目的）的有效组合。您可以提供仅源 IP 地址、源 IP 地址加目的端口，或这四个参数中任意一个或多个的组合
注意：本驱动允许您使ethtool user-def mask 字段，基于用户定义的双字节模式和偏移来过滤流量。用户定义的灵活过滤器仅支持 L3 L4 流类型。对于给定的流类型，在更改输入集（针对该流类型）之前，必须先清除所Intel 以太网流导向器过滤器

### 流导向器过滤
流导向器过滤器用于导向与指定特征匹配的数据流。它们通过 ethtool ntuple 接口启用。要启用

```

  # ethtool -K <ethX> ntuple <off|on>

```
注意：当您禁ntuple 过滤器时，所有用户编程的过滤器都会从驱动缓存和硬件中清除。重新启ntuple 时，必须重新添加所有需要的过滤器
```

  # ethtool -u <ethX>

```
```

  # ethtool -U <ethX> flow-type <type> src-ip <ip> [m <ip_mask>] dst-ip <ip>
  [m <ip_mask>] src-port <port> [m <port_mask>] dst-port <port> [m <port_mask>]
  action <queue>

  Where:
    <ethX> - the Ethernet device to program
    <type> - can be ip4, tcp4, udp4, sctp4, ip6, tcp6, udp6, sctp6
    <ip> - the IP address to match on
    <ip_mask> - the IPv4 address to mask on
              NOTE: These filters use inverted masks.
    <port> - the port number to match on
    <port_mask> - the 16-bit integer for masking
              NOTE: These filters use inverted masks.
    <queue> - the queue to direct traffic toward (-1 discards the
              matched traffic)

```
```

  # ethtool -U <ethX> delete <N>

  Where <N> is the filter ID displayed when printing all the active filters,
  and may also have been specified using "loc <N>" when adding the filter.

```
示例
```

  # ethtool -U <ethX> flow-type tcp4 src-ip 192.168.10.1 dst-ip \
  192.168.10.2 src-port 2000 dst-port 2001 action 2 [loc 1]

```
```

  # ethtool -U <ethX> flow-type tcp4 src-ip 192.168.10.1 dst-ip \
  192.168.10.2 action 2 [loc 1]

```
```

  # ethtool -U <ethX> flow-type tcp4 src-ip 192.168.10.1 dst-ip \
  192.168.10.2 user-def 0x4FFFF action 2 [loc 1]

  where the value of the user-def field contains the offset (4 bytes) and
  the pattern (0xffff).

```
要匹配从 192.168.0.1、端5300 发出、定向到 192.168.0.5 TCP 流量
```

  # ethtool -U enp130s0 flow-type tcp4 src-ip 192.168.0.1 dst-ip 192.168.0.5
  src-port 5300 dst-port 80 action 7

```
```

  # ethtool -U <ethX> flow-type tcp4 src-ip 192.168.0.0 m 0.255.255.255 dst-ip
  192.168.5.12 src-port 12600 dst-port 31 action 12

```
注意
对于每个流类型，编程的过滤器必须全部具有相同的匹
```

  # ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.1 src-port 5300 action 7
  # ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.5 src-port 55 action 10

```
然而，发出接下来的两条命令是不可接受的，因为第一
```

  # ethtool -U enp130s0 flow-type ip4 src-ip 192.168.0.1 src-port 5300 action 7
  # ethtool -U enp130s0 flow-type ip4 dst-ip 192.168.0.5 src-port 55 action 10

```
第二条命令将失败并报错。您可以使用相同字段、不同值编程多个过滤器，但在一个设备上，您不能编程两个具有不同匹配字段tcp4 过滤器
ice 驱动不支持对字段的子部分进行匹配，因此不支持部分掩码字段

### 灵活字节流导向器过滤
驱动还支持匹配数据包载荷中的用户定义数据。此灵活数据通过 ethtool 命令"user-def" 字段以下列方式指定：


    ============================== ============================
    `31    28    24    20    16` `15    12    8    4    0`
    `offset into packet payload` `2 bytes of flexible data`
    ============================== ============================

例如
```

  ... user-def 0x4FFFF ...

```
指示过滤器在载荷中查4 字节，并将该值与 0xFFFF 匹配。偏移基于载荷的起始位置，而非数据包的起始位置。因
```

  flow-type tcp4 ... user-def 0x8BEAF ...

```
将匹TCP/IPv4 载荷中第 8 字节处值为 0xBEAF TCP/IPv4 数据包
注意 ICMP 头部被解析为 4 字节头部4 字节载荷。因此要匹配载荷的第一个字节，实际上必须给偏移4 字节。另请注意，ip4 过滤器同时匹ICMP 帧以及原始（未知）ip4 帧，其载荷将IP4 帧的 L3 载荷
最大偏移为 64。硬件只会从载荷中读取最64 字节数据。偏移必须为偶数，因为灵活数据为 2 字节长，且必须与数据包载荷的字节 0 对齐
用户定义的灵活偏移也被视为输入集的一部分，不能针对同一类型的多个过滤器单独编程。但是，灵活数据不属于输入集，多个过滤器可以使用相同偏移但匹配不同数据

### RSS 哈希
允许您为每个流类型设置哈希字节，以及接收端缩放（RSS）哈希字节配置的一个或多个选项组合
```

  # ethtool -N <ethX> rx-flow-hash <type> <option>

  Where <type> is:
    tcp4    signifying TCP over IPv4
    udp4    signifying UDP over IPv4
    gtpc4   signifying GTP-C over IPv4
    gtpc4t  signifying GTP-C (include TEID) over IPv4
    gtpu4   signifying GTP-U over IPV4
    gtpu4e  signifying GTP-U and Extension Header over IPV4
    gtpu4u  signifying GTP-U PSC Uplink over IPV4
    gtpu4d  signifying GTP-U PSC Downlink over IPV4
    tcp6    signifying TCP over IPv6
    udp6    signifying UDP over IPv6
    gtpc6   signifying GTP-C over IPv6
    gtpc6t  signifying GTP-C (include TEID) over IPv6
    gtpu6   signifying GTP-U over IPV6
    gtpu6e  signifying GTP-U and Extension Header over IPV6
    gtpu6u  signifying GTP-U PSC Uplink over IPV6
    gtpu6d  signifying GTP-U PSC Downlink over IPV6
  And <option> is one or more of:
    s     Hash on the IP source address of the Rx packet.
    d     Hash on the IP destination address of the Rx packet.
    f     Hash on bytes 0 and 1 of the Layer 4 header of the Rx packet.
    n     Hash on bytes 2 and 3 of the Layer 4 header of the Rx packet.
    e     Hash on GTP Packet on TEID (4bytes) of the Rx packet.


```
### 加速接收流导向（aRFS
基于 Intel(R) 以太网控制器 800 系列的设备在 PF 上支持加速接收流导向（aRFS）。aRFS 是一种负载均衡机制，允许您将数据包导向运行或消费该流中数据包的同一 CPU
注意
- aRFS 需要通过 ethtool 启用 ntuple 过滤- aRFS 支持仅限于以下数据包类型
    - IPv4 IPv6 上的 TCP
    - IPv4 IPv6 上的 UDP
    - 非分片数据包

- aRFS 仅支持流导向器过滤器，其由源/目的 IP 地址和源/目的端口组成- aRFS ethtool ntuple 接口都使用设备的流导向器。aRFS ntuple 特性可以共存，但如aRFS ntuple 请求之间存在冲突，可能会遇到意外结果。有关更多信息，请参Intel(R) 以太网流导向
设置 aRFS
1. 使用 ethtool 启用 Intel 以太网流导向器和 ntuple 过滤器
```

   # ethtool -K <ethX> ntuple on

```
2. 设置全局流表中的条目数。例如：

```

   # NUM_RPS_ENTRIES=16384
   # echo $NUM_RPS_ENTRIES > /proc/sys/net/core/rps_sock_flow_entries

```
3. 设置每队列流表中的条目数。例如：

```

   # NUM_RX_QUEUES=64
   # for file in /sys/class/net/$IFACE/queues/rx-*/rps_flow_cnt; do
   # echo $(($NUM_RPS_ENTRIES/$NUM_RX_QUEUES)) > $file;
   # done

```
4. 禁用 IRQ 均衡守护进程（这只是服务的临时停止，直到下次重启）
```

   # systemctl stop irqbalance

```
5. 配置中断亲和性
   参见 `/Documentation/core-api/irq/irq-affinity.rst`


```

  # ethtool -K <ethX> ntuple off

```
注意：此命令将禁ntuple 过滤器，并清除软件与硬件中的任何 aRFS 过滤器
用例示例
1. 将服务器应用程序设置在所需 CPU 上（例如 CPU 4）
```

   # taskset -c 4 netserver

```
2. 使用 netperf 在已配置 aRFS 的情况下，将来自客户端的流量路由到服务器上的 CPU 4。本例使IPv4 上的 TCP
```

   # netperf -H <Host IPv4 Address> -t TCP_STREAM


```
### 启用虚拟功能（VF
使用 sysfs 启用虚拟功能（VF）
```

  # echo 4 > /sys/class/net/<ethX>/device/sriov_numvfs

```
```

  # echo 0 > /sys/class/net/<ethX>/device/sriov_numvfs

```
ice 驱动支持VF 最大总数256（所有端口）。要检
```

  # cat /sys/class/net/<ethX>/device/sriov_totalvfs

```
注意：当链路聚合（LAGbonding 处于活动状态时，您不能使用 SR-IOV，反之亦然。为强制执行此规则，驱动会检查这种互斥关系

### PF 上显VF 统计信息

```

  # ip -s link show dev <ethX>

```
注意：由于可能的 VF 数量很大，此命令的输出可能非常庞大
PF 驱动将显PF 以及所有已配置 VF 的部分统计信息。PF 将始终为每个可能VF 打印一个统计块，对于未配置VF 则显示零

### 在启SR-IOV 的适配器端口上配置 VLAN 标记

要为启用 SR-IOV 的适配器上的端口配VLAN 标记，请使用以下命令。VLAN 配置应在加载 VF 驱动或启VM 之前完成。VF 不会感知在发送时插入、接收时移除VLAN 标记（有时称端口 VLAN"模式）
```

  # ip link set dev <ethX> vf <id> vlan <vlan id>

```
```

  # ip link set dev eth0 vf 0 vlan 10


```
### 若端口断开则启VF 链路

如果物理功能（PF）链路断开，您可以从主PF 强制任何绑定到该 PF 的虚拟功能（VF）链up
```

  # ip link set eth0 vf 0 state enable

```
注意：如果命令不起作用，可能是您的系统不支持

### 设置 VF MAC 地址

```

  # ip link set <ethX> vf 0 mac <address>

```
```

  # ip link set <ethX> vf 0 mac 00:01:02:03:04:05

```
此设置持续到 PF 重新加载
注意：从主机VF 分配 MAC 地址将禁用来VM 内部的任何后续更MAC 地址的请求。这是一项安全特性。VM 不会感知此限制，因此如果VM 中尝试，将触MDD 事件

### 受信VF VF 混杂模式

此特性允许您将特VF 指定为受信任，并允许该受信任VF 在物理功能（PF）上请求选择性混杂模式
要将 VF 设置为受信任或不受信任，请输入以下命
```

  # ip link set dev <ethX> vf 1 trust [on|off]

```
注意：在设置混杂模式之前，先VF 设置为受信任非常重要。如VM 不受信任，PF 将忽略来VF 的混杂模式请求。如果在 VF 驱动加载VM 变为受信任，您必须重新发出请求以VF 设置为混杂模式
一VF 被指定为受信任，请使VM 中的以下命令VF 设置为混杂模式
```

  # ip link set <ethX> promisc on
  Where <ethX> is a VF interface in the VM

```
```

  # ip link set <ethX> allmulticast on
  Where <ethX> is a VF interface in the VM

```
注意：默认情况下，ethtool 私有标志 vf-true-promisc-support 设置"off"，意味着 VF 的混杂模式将受限。要VF 的混杂模式设置为真正的混杂模式并允许 VF 看到所
```

  # ethtool --set-priv-flags <ethX> vf-true-promisc-support on

```
vf-true-promisc-support 私有标志并不启用混杂模式；相反，它指定当您使用上ip link 命令启用混杂模式时，将获得哪种类型的混杂模式（受限或真正）。请注意这是一个影响整个设备的全局设置。但是，vf-true-promisc-support 私有标志仅暴露给设备的第一PF。无vf-true-promisc-support 设置如何，PF 始终保持受限混杂模式
```

  # ip link add link eth2 name eth2.100 type vlan id 100

```
请注意，您将 VF 设置为混杂模式与添加 VLAN 接口的顺序无关（可任意先做其一）。本例的结果VF 将获得所有标记了 VLAN 100 的流量

### 针对 VF 的恶意驱动检测（MDD
一Intel 以太网设备使用恶意驱动检测（MDD）来检测来VF 的恶意流量，并在 VF 驱动复位发生前禁Tx/Rx 队列或丢弃违规数据包。您可以使用 dmesg 命令PF 的系统日志中查看 MDD 消息
- 如果 PF 驱动记录了来VF MDD 事件，请确认已安装正确的 VF 驱动- 要恢复功能，您可以手动重新加VF VM，或启用自动 VF 复位- 启用自动 VF 复位后，PF 驱动在接收路径上检测到 MDD 事件时会立即复位 VF 并重新启用队列- 如果禁用自动 VF 复位，PF 在检测到 MDD 事件时不会自动复VF
```

  # ethtool --set-priv-flags <ethX> mdd-auto-reset-vf on|off


```
### 针对 VF MAC VLAN 防欺骗特
当虚拟功能（VF）接口上的恶意驱动尝试发送欺骗数据包时，硬件会将其丢弃而不传输
```

  # ip link set <ethX> vf <vf id> spoofchk {off|on}


```
### 巨型
通过将最大传输单元（MTU）更改为大于默认1500 的值来启用巨型帧支持
使用 ifconfig 命令增大 MTU 大小。例如，输入

```

  # ifconfig <ethX> mtu 9000 up

```
```

  # ip link set mtu 9000 dev <ethX>
  # ip link set up dev <ethX>

```
此设置不会在重启后保留

注意：巨型帧的最MTU 设置9702。这对应9728 字节的最大巨型帧大小
注意：本驱动将尝试使用多个页大小的缓冲区来接收每个巨型数据包。这有助于避免在分配接收数据包时出现缓冲区耗尽问题
注意：使用巨型帧时，丢包可能对吞吐量产生更大影响。如果在启用巨型帧后观察到性能下降，启用流控可能会缓解该问题

### 速率与双工配
在解决速率和双工配置问题时，您需要区分基于铜缆的适配器和基于光纤的适配器
在默认模式下，使用铜缆连接的 Intel(R) 以太网网络适配器将尝试与其链路伙伴自动协商以确定最佳设置。如果适配器无法通过自动协商与链路伙伴建立链路，您可能需要手动将适配器和链路伙伴配置为相同设置，以建立链路并传输数据包。这仅在尝试与不支持自动协商的老旧交换机连接，或已被强制为特定速率或双工模式时才需要。您的链路伙伴必须与您选择的设置匹配 Gbps 及更高速率无法被强制。使用自动协商通告设置手动1 Gbps 及更高速率设置设备
速率、双工和自动协商通告通过 ethtool 工具配置。有关最新版本，请从以下网站下载并安ethtool
   https://kernel.org/pub/software/network/ethtool/

```

  # ethtool <ethX>

```
警告：只有经验丰富的网络管理员才应强制设置速率和双工，或手动更改自动协商通告。交换机上的设置必须始终与适配器设置匹配。如果您将适配器配置得与交换机不同，适配器性能可能会下降或无法工作

### 数据中心桥接（DCB
注意：内核假TC0 可用，如TC0 不可用，将在设备上禁用优先级流控（PFC）。要解决此问题，请在交换机上设置 DCB 时确保启TC0
DCB 是硬件中的服务质量（QoS）配置实现。它使用 VLAN 优先级标签（802.1p）来过滤流量。这意味着流量可以被过滤到 8 个不同的优先级。它还启用了优先级流控（802.1Qbb），可以在网络压力期间限制或消除丢包数量。可以为这些优先级中的每一个分配带宽，该分配在硬件级别强制执行02.1Qaz）
DCB 通常使用 DCBX 协议02.1Qaz，LLDP02.1AB）的特化版）在网络上配置。ice 驱动支持以下互斥DCBX 支持变体
1) 基于固件LLDP 代理
2) 基于软件LLDP 代理

在基于固件模式下，固件拦截所LLDP 流量并透明地为用户处理 DCBX 协商。在此模式下，适配器以"willing" DCBX 模式运行，从链路伙伴（通常是交换机）接DCB 设置。本地用户只能查询协商后DCB 配置。有关在交换机上配置 DCBX 参数的信息，请查阅交换机制造商的文档
在基于软件模式下，LLDP 流量被转发到网络栈和用户空间，由软件代理处理。在此模式下，适配器可以以"willing"nonwilling" DCBX 模式运行，并DCB 配置既可以被查询也可以在本地设置。此模式需要禁用基FW LLDP 代理
注意
- 您可以使ethtool 私有标志启用和禁用基于固件的 LLDP 代理。有关更多信息，请参阅本文档中的"FW-LLDP（固件链路层发现协议一节- 在基于软件的 DCBX 模式下，您可以使用与 Linux 内核DCB Netlink API 接口的软LLDP/DCBX 代理配置 DCB 参数。我们建议在软件模式下运行时使用 OpenLLDP 作为 DCBX 代理。更多信息，请参OpenLLDP 的手册页https://github.com/intel/openlldp- 驱动实现DCB netlink 接口层，以允许用户空间与驱动通信并查询端口的 DCB 配置- 不支持带 DCB iSCSI

### FW-LLDP（固件链路层发现协议
使用 ethtool 更改 FW-LLDP 设置。FW-LLDP 设置为每端口设置，并在重启后保留
```

  # ethtool --set-priv-flags <ethX> fw-lldp-agent on

```
```

  # ethtool --set-priv-flags <ethX> fw-lldp-agent off

```
```

  # ethtool --show-priv-flags <ethX>

```
注意：您必须启用 UEFI HII "LLDP Agent" 属性，此设置才能生效。如"LLDP AGENT" 被设置为禁用，您无法从操作系统启用它

### 流控

以太网流控（IEEE 802.3x）可通过 ethtool 配置，以启用 ice 的接收和发送暂停帧。启用发送时，当接收数据包缓冲区越过预定义阈值时会生成暂停帧。启用接收时，发送单元将在收到暂停帧时指定的延迟时间内暂停
注意：您必须有一个支持流控的链路伙伴
流控默认禁用
使用 ethtool 更改流控设置
```

  # ethtool -A <ethX> rx <on|off> tx <on|off>

```
注意：此命令仅在禁用自动协商时才启用或禁用流控。如果启用了自动协商，此命令会更改与链路伙伴进行自动协商所用的参数
注意：流控自动协商是链路自动协商的一部分。根据您的设备，您可能无法更改自动协商设置
注意
- ice 驱动要求端口和链路伙伴两端都启用流控。如果其中一端禁用了流控，端口在重流量下可能看似挂起- 禁用 DCB 后，您可能会遇到链路级流控（LFC）问题。LFC 状态可能显示为已启用，但流量并未被暂停。要解决

```

   # ethtool -A <ethX> rx off tx off
   # ethtool -A <ethX> rx on tx on


```
### NAPI


本驱动支NAPI（Rx 轮询模式）
参见 Documentation/networking/napi.rst <napi> 获取更多信息
### MACVLAN

本驱动支MACVLAN。可以通过检查是否已加载 MACVLAN 驱动来测试内核是否支MACVLAN。您可以运行 'lsmod | grep macvlan' 查看是否已加MACVLAN 驱动，或运行 'modprobe macvlan' 尝试加载 MACVLAN 驱动
注意
- passthru 模式下，您只能设置一MACVLAN 设备。它将继承底PF（物理功能）设备MAC 地址

### IEEE 802.1ad（QinQ）支
IEEE 802.1ad 标准，通常称为 QinQ，允许在单个以太网帧中包含多VLAN ID。VLAN ID 有时称为"标签"，因此多VLAN ID 称为"标签。标签栈允许 L2 隧道以及在特VLAN ID 内隔离流量等用途
注意
- 802.1ad（QinQ）数据包不支持接收校验和卸载VLAN 加速
- 除非通过以下方式禁用 VLAN 剥离，否则不会接0x88A8 流量

```

    # ethtool -K <ethX> rxvlan off

```
- 同一端口上配置了 0x8100 VLAN 时，不能使用 0x88A8/0x8100 VLAN 0x8100 0x8100/0x8100 VLAN。如果配置了 0x8100 VLAN，将不会接收 0x88a8/0x8100 流量
- VF 仅在下述条件下才能发0x88A8/0x8100（即 802.1ad/802.1Q）流量：

    1) VF 未被分配端口 VLAN    2) PF 禁用spoofchk。如果启spoofchk，VF 将不会发0x88A8/0x8100 流量
- SR-IOV 模式下启VF 真正混杂模式（vf-true-promisc-support）和VLAN 时，VF 可能无法根据内部 VLAN 头接收所有网络流量
```

  # ip link add link eth0 eth0.24 type vlan proto 802.1ad id 24
  # ip link add link eth0.24 eth0.24.371 type vlan proto 802.1Q id 371

  Where "24" and "371" are example VLAN IDs.


```
### 闅ч亾/鍙犲姞鏃犵姸鎬佸嵏杞。
支持的隧道和叠加包括 VXLAN、GENEVE 以及取决于硬件和软件配置的其他类型。无状态卸载默认启用
```

  # ethtool -k <ethX>


```
### UDP 分段卸载

允许适配器将有效载荷最64K UDP 数据包的发送分段卸载到有效的以太网帧中。由于适配器硬件能够比操作系统软件更快地完成数据分段，此特性可改善传输性能此外，适配器可能使用更少的 CPU 资源
注意
- 发UDP 数据包的应用程序必须支持 UDP 分段卸载
```

  # ethtool -K <ethX> tx-udp-segmentation [off|on]

```
### PTP 引脚接口

所有适配器都支持标准 PTP 引脚接口。SDP（Software Definable Pin，软件可定义引脚）是支持周期输出和外部时间戳的单端引脚。还有特定的差分输入/输出引脚（TIME_SYNCPPS），每种仅支持其中一种功能
有些适配器带DPLL，其引脚连接DPLL 而非暴露在板卡上。您需要注意，在这些配置中，仅暴露 SDP 引脚，且每个引脚有其固定的方向。要在这PTP 引脚上看到输入信号，您必须正确配DPLL。输出信号仅DPLL 上可见，要将其发送到板卡SMA/U.FL 引脚，必须手动配DPLL 输出引脚
### GNSS 模块

需要内核以 CONFIG_GNSS=y CONFIG_GNSS=m 编译。允许用户从 GNSS 硬件模块读取消息并写入受支持的命令。如果模块物理存在，将生成一GNSS 设备：`/dev/gnss<id>`。写入命令的协议取决GNSS 硬件模块，因为驱动通过 i2c 将原始字节由 GNSS 对象写入接收器。有关配置详情，请参阅硬GNSS 模块文档

### 固件（FW）日
驱动仅通过 PF 0 上的 debugfs 接口支持 FW 日志。运行在 NIC 上的 FW 必须支持 FW 日志；如FW 不支FW 日志，则不会ice debugfs 目录中创'fwlog' 文件
#### 模块配置

固件日志按模块进行配置。每个模块可以设置为独立于其他模块的值（除非指定模块 'all'）。这些模块将'fwlog/modules' 目录下实例化
用户可以通过写入模块文件来设置模块的日志级别，如

```

  # echo <log_level> > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/<module>

```
其中

- log_level 是如下所述的名称。每个级别包含前一更低级别的消
      - none
      - error
      - warning
      - normal
      - verbose

- module 是表示要接收事件的模块的名称。模块名称为

      - general
      - ctrl
      - link
      - link_topo
      - dnl
      - i2c
      - sdp
      - mdio
      - adminq
      - hdma
      - lldp
      - dcbx
      - dcb
      - xlr
      - nvm
      - auth
      - vpd
      - iosf
      - parser
      - sw
      - scheduler
      - txq
      - rsvd
      - post
      - watchdog
      - task_dispatch
      - mng
      - synce
      - health
      - tsdrv
      - pfreg
      - mdlver
      - all

名称 'all' 是特殊的，允许用户将所有模块设置为指定log_level，或读取所有模块的 log_level
##### 配置模块的示例用

```

  # echo verbose > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/link

```
```

  # echo verbose > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/link
  # echo warning > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/ctrl
  # echo none > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/dcb

```
```

  # echo normal > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/all

```
```

  # cat /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/general

```
```

  # cat /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/modules/all

```
#### 启用 FW 日志

配置模块会通知 FW，配置的模块应生成驱动感兴趣的事件，但在FW 发enable 消息之前，它**不会**将这些事件发送给驱动。为此，用户可以'fwlog/enable' 写入 1（启用）0（禁用）。示
```

  # echo 1 > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/enable

```
#### 获取 FW 日志数据

可以通过读取 'fwlog/data' 获取 FW 日志数据。用户可以向 'fwlog/data' 写入任意值以清除数据。数据只能在禁用 FW 日志时清除。FW 日志数据是发送给 Intel 并用于帮助调试用户问题的二进制文件
```

  # cat /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/data > fwlog.bin

```
```

  # echo 0 > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/data

```
#### 更改日志事件发送到驱动的频
驱动从管理接收队列（ARQ）接FW 日志数据。FW 发ARQ 事件的频率可以通过写入 'fwlog/nr_messages' 来配置。范围是 1-128 表示推送每条日志消息，128 表示仅在最AQ 命令缓冲区满时推送）。建议值为 10。用户可以通过读取

```

  # echo 50 > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/nr_messages

```
#### 配置用于存储 FW 日志数据的内存量

驱动在驱动内部存FW 日志数据。用于存储数据的默认内存大小1MB。某些用例可能需要更多或更少数据，因此用户可以更改为 FW 日志数据分配的内存量。要更改内存量，请写'fwlog/log_size'。值必须为以下之一28K56K12KM 2M。必须禁FW 日志才能更改

```

  # echo 128K > /sys/kernel/debug/ice/0000\:18\:00.0/fwlog/log_size


```
## 性能优化

驱动默认值旨在适应各种工作负载，但如果需要进一步优化，我们建议尝试以下设置

### Rx 描述符环大小

要减Rx 数据包丢弃的数量，请使用 ethtool 增加每个 Rx 环的 Rx 描述符数量
  检查接口是否因缓冲区满而丢Rx 数据
```

    # ethtool -S <ethX> | grep "rx_dropped"

  如果上一条命令显示队列上有丢弃，使用 'ethtool -G' 增加描述符数量可能会有帮助：

    # ethtool -G <ethX> rx <N>
    Where <N> is the desired number of ring entries/descriptors

  这可以为 CPU 处理描述符时产生延迟的问题提供临时缓冲

```
### 中断速率限制

本驱动支持针对通用工作负载调优的自适应中断节流率（ITR）机制。用户可以通过 ethtool 针对特定工作负载自定义中断速率控制，调整中断之间的微秒数
```

  # ethtool -C <ethX> adaptive-rx off adaptive-tx off

```
为降CPU 利用率：

  禁用自适应 ITR 并降Rx Tx 中断。以下示例影响指定接口的每个队列
  rx-usecs tx-usecs 设置80 会将中断限制在大
```

    # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs 80 tx-usecs 80

```
为降低延迟：

  通过rx-usecs tx-usecs 设置0 来禁用自适应 ITR ITR

```

    # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs 0 tx-usecs 0

```
每队列中断速率设置
  以下示例针对队列 1 3，但您可以调整其他队列
  要禁Rx 自适应 ITR 并将静Rx ITR 设置10 微秒
```

    # ethtool --per-queue <ethX> queue_mask 0xa --coalesce adaptive-rx off
    rx-usecs 10

  要显示队1 3 的当前合并设置：

    # ethtool --per-queue <ethX> queue_mask 0xa --show-coalesce

```
使用 rx-usecs-high 限制中断速率
  :有效范围-236=无限制）

   0-236 微秒的范围提供每4,237 250,000 次中断的有效范围。rx-usecs-high 的值可以在同一 ethtool 命令中独立于 rx-usecs tx-usecs 设置，并且也独立于自适应中断调节算法。底层硬件支4 微秒间隔的粒度，因此相邻值可能导致相同的中断速率
  以下命令将禁用自适应中断调节，并允许在指示接收或发送完成之前最5 微秒。然而，它不会像可能产生多达每秒 200,000 次中断那样，而是通过 rx-usecs-high 参数将总中断限制为每秒 50,000 次
```

    # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs-high 20
    rx-usecs 5 tx-usecs 5


```
### 虚拟化环
除本节的其他建议外，以下建议可能有助于优VM 中的性能
  VM 中使用适当的机制（vcpupin），CPU 固定到各LCPU，确保使用包含在设备 local_cpulist 中的一CPU：`/sys/class/net/<ethX>/device/local_cpulist`
  VM 中配置尽可能多的可用 Rx/Tx 队列。（有关 iavf 驱动

```

    # ethtool -L <virt_interface> rx <max> tx <max>


```
## 支持

有关一般信息，请访Intel 支持网站https://www.intel.com/support/

如果在受支持的内核上使用受支持的适配器，发现已发布源码存在问题时，请将与该问题相关的具体信息发送至 intel-wired-lan@lists.osuosl.org

## 商标

Intel Intel 公司或其子公司在美国或其他国地区的商标或注册商标
- 其他名称和品牌可能被声称为其他方的财产