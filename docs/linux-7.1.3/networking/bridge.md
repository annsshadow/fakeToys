
## 以太网桥
## 简
IEEE 802.1Q-2022（Bridges and Bridged Networks，桥接与桥接网络）标准定义了桥接在计算机网络中的运作方式。在该标准的语境下，桥（bridge）是一种连接两个或多个网段、并运行OSI（Open Systems Interconnection，开放系统互连）模型的数据链路层（Layer 2，第二层）的设备。桥的作用是依据目的 MAC（Media Access Control，介质访问控制）地址在不同网段之间过滤并转发帧
## 桥接 kAPI

下面是桥接代码的一些核心结构体。请注意，kAPI *不稳*的，随时可能被修改
   :identifiers: net_bridge_vlan

## 桥接 uAPI

现代 Linux 桥接 uAPI 通过 Netlink 接口访问。你可以在下面的文件中找到桥接以及桥接端口的 netlink 属性定义
### 桥接 netlink 属
   :doc: Bridge enum definition

### 桥接端口 netlink 属
   :doc: Bridge port enum definition

### 桥接 sysfs

sysfs 接口已被弃用，若新增选项则不应再扩展它
## STP（生成树协议
Linux 桥接驱动中的 STP（Spanning Tree Protocol，生成树协议）实现是一个关键特性，它通过识别并禁用冗余链路，帮助防止以太网网络中的环路与广播风暴。在 Linux 桥接的语境下，STP 对网络的稳定性与可用性至关重要
STP 是一个运行在 OSI 模型数据链路层的 Layer 2 协议。它最初作IEEE 802.1D 开发，此后演进出了多个版本，包Rapid Spanning Tree Protocol（RSTP，快速生成树协议）以`Multiple Spanning Tree Protocol (MSTP)
<https://lore.kernel.org/netdev/20220316150857.2442916-1-tobias@waldekranz.com/>`_銆。
802.1D-2004 移除了最初的 Spanning Tree Protocol，转而纳入了 Rapid Spanning Tree Protocol（RSTP）。到 2014 年，IEEE 802.1D 定义的全部功能都已被合并IEEE 802.1Q（Bridges and Bridged Networks，桥接与桥接网络）或 IEEE 802.1AC（MAC Service Definition，MAC 服务定义）之中02.1D 已于 2022 年正式撤销
### 桥接端口STP 状
STP 的语境下，桥接端口可处于以下状态之一  - Blocking（阻塞）：端口被禁止数据流量，只侦听来自其他设备BPDU（Bridge Protocol Data Units，桥协议数据单元），以确定网络拓扑  - Listening（侦听）：端口开始参STP 过程并侦BPDU  - Learning（学习）：端口继续侦BPDU，并开始从 incoming 帧中学习 MAC 地址，但不转发数据帧  - Forwarding（转发）：端口完全可用，同时转发 BPDU 与数据帧  - Disabled（禁用）：端口被管理性禁用，不参STP 过程，数据帧转发也被禁用
### 根桥与收
Linux 网络与以太网桥接的语境下，根桥（root bridge）是桥接网络中一个被指定的交换机，它作为生成树算法的参考点，用于创建无环拓扑
以下STP 的工作原理以及根桥的选举方式  1. Bridge Priority（桥优先级）：每个运行生成树协议的桥都有一个可配置Bridge Priority 值。值越小，优先级越高。默认情况下，Bridge Priority 被设置为一个标准值（例如 32768）  2. Bridge ID（桥 ID）：Bridge ID 由两部分组成：Bridge Priority 与桥MAC 地址。它在网络中唯一标识每个桥。Bridge ID 用于比较不同桥的优先级  3. Bridge Election（桥选举）：网络启动时，所有桥最初都假定自己是根桥。它们开始向邻居通告 Bridge Protocol Data Units（BPDU，桥协议数据单元），其中包含自身Bridge ID 及其他信息  4. BPDU Comparison（BPDU 比较）：桥之间相互交BPDU 以确定根桥。每个桥检查收到的 BPDU（包Bridge Priority Bridge ID），来判断是否应调整自身的优先级。Bridge ID 最小的桥将成为根桥  5. Root Bridge Announcement（根桥通告）：一旦确定了根桥，它就会向网络中所有其他桥发送包含根桥信息的 BPDU。其他桥利用这些信息计算出到根桥的最短路径，从而创建无环拓扑  6. Forwarding Ports（转发端口）：根桥选定、生成树拓扑建立之后，每个桥都会确定其哪些端口应处于转发状态（用于数据流量）、哪些应处于阻塞状态（用于防止环路）。根桥的所有端口都处于转发状态，而其他桥则有一些端口处于阻塞状态以避免环路  7. Root Ports（根端口）：根桥选定、生成树拓扑建立之后，每个非根桥处理收到BPDU，并根据其中信息确定哪个端口提供了到根桥的最短路径。该端口被指定为根端口，且处Forwarding（转发）状态，可主动转发网络流量  8. Designated ports（指定端口）：指定端口是非根桥用来向指定网段转发流量的端口。指定端口被置于 Forwarding（转发）状态。非根桥上所有未被指定给特定网段的端口都被置Blocking（阻塞）状态，以防止网络环路
STP 通过计算最短路径并禁用冗余链路来保障网络收敛。当网络拓扑发生变化（例如链路故障）时，STP 会重新计算网络拓扑，在避免环路的同时恢复连通性
STP 参数（例bridge priority，桥优先级）的正确配置，会影响网络性能、路径选择以及哪个桥成为根桥（Root Bridge）
### 用户空间 STP 辅助程序

用户空间STP 辅助程序 **bridge-stp** 是一个用于控制是否使用用户模式生成树（spanning tree）的程序。当桥上启用/禁用 STP 时（通过 `brctl stp <bridge> <on|off>` ``ip link set <bridge> type bridge
stp_state <0|1>``），内核会调`/sbin/bridge-stp <bridge> <start|stop>`。若该命令返0，内核启user_stp 模式；若返回其他值，则启kernel_stp 模式
### STP 模式选择

`IFLA_BR_STP_MODE` 桥接属性允许在 STP 启用时显式控制其运作方式，对`user` `kernel` 模式可完全绕`/sbin/bridge-stp` 辅助程序
   :doc: Bridge STP mode values

默认模式`BR_STP_MODE_AUTO`，保留了调用 `/sbin/bridge-stp` 辅助程序的传统行为。`user` `kernel` 模式helper 机制不可用的网络命名空间环境中尤其有用，因为 `call_usermodehelper()` 被限制在初始网络命名空间中
```

  ip link set dev br0 type bridge stp_mode user stp_state 1

```

该模式只能在 STP 被禁用时修改
## VLAN（虚拟局域网
LAN（Local Area Network，局域网）是覆盖较小地理区域的网络，通常位于一栋建筑或一个园区内。LAN 用于连接 localized 区域内的计算机、服务器、打印机及其他联网设备。LAN 可以是有线的（使用以太网电缆）或无线的（使用 Wi-Fi）
VLAN（Virtual Local Area Network，虚拟局域网）是对物理网络的逻辑分割，形成多个相互隔离的广播域。VLAN 用于将一个物LAN 划分为多个虚LAN，使不同组的设备可以像身处独立的物理网络一样相互通信
通常有两VLAN 实现：IEEE 802.1Q IEEE 802.1ad（也QinQ）。IEEE 802.1Q 是以太网VLAN 标记（tagging）的标准。它允许网络管理员在物理网络上创建逻辑 VLAN，并用以 VLAN 信息标记以太网帧，这被称*VLAN 标记帧（VLAN-tagged frames*。IEEE 802.1ad 通常称为 QinQ Double VLAN，是 IEEE 802.1Q 标准的扩展。QinQ 允许在单个以太网帧内堆叠多个 VLAN 标记。Linux 桥同时支IEEE 802.1Q 以及 `802.1AD
<https://lore.kernel.org/netdev/1402401565-15423-1-git-send-email-makita.toshiaki@lab.ntt.co.jp/>`_
这两种用VLAN 标记的协议
`VLAN filtering <https://lore.kernel.org/netdev/1360792820-14116-1-git-send-email-vyasevic@redhat.com/>`_
在桥上默认是禁用的。在桥上启用 VLAN filtering 后，它将依据目的 MAC 地址VLAN 标记（两者都必须匹配）把帧转发到合适的目标
## 组播（Multicast
Linux 桥接驱动支持组播，使其能够处Internet Group Management Protocol（IGMP，因特网组管理协议）Multicast Listener Discovery（MLD，组播侦听者发现）消息，并高效地转发组播数据包。该桥接驱动支持 IGMPv2/IGMPv3 MLDv1/MLDv2
### 组播侦听（Multicast snooping
Multicast snooping 是一项网络技术，它使网络交换机能够在局域网（LAN）内智能地管理组播流量
交换机会维护一张组播组表，记录组播组地址与主机已加入这些组的端口之间的关联。该组表根据收到IGMP/MLD 消息动态更新。借助通过 snooping 收集的组播组信息，交换机优化组播流量的转发。它不会盲目地将组播流量广播到所有端口，而是仅根据目MAC 地址将组播流量发送到已订阅相应目的组播组的端口
Linux 桥接设备在创建时默认启用 multicast snooping。它会维护一Multicast forwarding database（MDB，组播转发表），用于记录端口与组之间的关系
### IGMPv3/MLDv2 EHT 支持

Linux 桥支IGMPv3/MLDv2 EHT（Explicit Host Tracking，显式主机跟踪），它`474ddb37fa3a ("net: bridge: multicast: add EHT allow/block handling")
<https://lore.kernel.org/netdev/20210120145203.1109140-1-razor@blackwall.org/>`_
加入
显式主机跟踪使设备能够记录加入某个特定组或通道的每一台独立主机。IGMP 中显式主机跟踪的主要好处，是能够在主机离开某个组播组或通道时实现最小的离开延迟（leave latency）
从主机想要离开到设备停止转发流量之间的时间间隔称为 IGMP leave latency（离开延迟）。配置了 IGMPv3 MLDv2 并开启显式跟踪的设备，在最后请求接收该设备流量的主机表示不再希望接收流量时，可立即停止转发流量。因此，离开延迟仅受多路访问网络中的数据包传输延迟以及设备处理时间的限制
### 其他组播特
Linux 桥还支持 `per-VLAN multicast snooping
<https://lore.kernel.org/netdev/20210719170637.435541-1-razor@blackwall.org/>`_
（默认禁用但可启用），以`Multicast Router Discovery
<https://lore.kernel.org/netdev/20190121062628.2710-1-linus.luessing@c0d3.blue/>`_
（组播路由器发现），后者用于帮助识别组播路由器的位置
## Switchdev

Linux Bridge Switchdev Linux 内核中的一项特性，它扩展了传统 Linux 桥的能力，使其能与支switchdev 的硬件交换机更高效地协同工作。借助 Linux Bridge Switchdev，转发、过滤、学习以太网帧等某些网络功能可被卸载（offload）到硬件交换机上。这种卸载减轻了 Linux 内核CPU 的负担，从而提升网络性能并降低延迟
要使Linux Bridge Switchdev，你需要支switchdev 接口的硬件交换机。这意味着交换机硬件必须具备必要的驱动与功能，才能Linux 内核协同工作
更多细节请参switchdev 文档
## Netfilter

bridge netfilter 模块是一项遗留特性，它允许使iptables ip6tables 过滤被桥接的数据包。不推荐使用它，用户应考虑使用 nftables 进行包过滤
较老的 ebtables 工具相比 nftables 功能更为有限，但nftables 一样，它也不需要此模块即可工作
br_netfilter 模块会拦截进入桥的数据包，对 ipv4 ipv6 数据包执行最基本的健全性检查，然后假装这些数据包正在被路由而非桥接。随br_netfilter 从桥接层调用 ip ipv6 netfilter 钩子，也就是ip(6)tables 规则集也会看到这些数据包
br_netfilter 也是 iptables **physdev** 匹配存在的原因：iptables 规则集中，此匹配是可靠区分路由包与桥接包的唯一方式
注意，ebtables nftables 在没br_netfilter 模块时也能正常工作。iptables/ip6tables/arptables 对桥接流量不起作用，因为它们插入了路由栈。ip/ip6/inet/arp 族的 nftables 规则同样看不到由桥转发的数据包，但这本来就是应有的行为
历史ebtables 的功能集非常有限（至今仍是如此），加入此模块是为了假装数据包被路由，并从桥接层调ipv4/ipv6 netfilter 钩子，使用户得以使用功能更丰富的 iptables 匹配能力（包conntrack）。nftables 没有这种限制，几乎全部特性都不受协议族影响而正常工作
因此，只有在用户出于某些原因需要使ip(6)tables 来过滤由桥转发的数据包，或对桥接流量NAT 时，才需br_netfilter。对于纯链路层过滤，则不需要此模块
## 其他特
Linux 桥还支持 `IEEE 802.11 Proxy ARP
<https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=958501163ddd6ea22a98f94fa0e7ce6d4734e5c4>`_銆?`Media Redundancy Protocol (MRP)
<https://lore.kernel.org/netdev/20200426132208.3232-1-horatiu.vultur@microchip.com/>`_銆?`Media Redundancy Protocol (MRP) LC mode
<https://lore.kernel.org/r/20201124082525.273820-1-horatiu.vultur@microchip.com>`_銆?`IEEE 802.1X port authentication
<https://lore.kernel.org/netdev/20220218155148.2329797-1-schultz.hans+netdev@gmail.com/>`_以及 `MAC Authentication Bypass (MAB)
<https://lore.kernel.org/netdev/20221101193922.2125323-2-idosch@nvidia.com/>`_銆。
## 常见问题（FAQ
### 桥的作用是什么？

桥在多个网络接口之间透明地转发流量。通俗地说，这意味着桥将两个或多个物理以太网网络连接在一起，形成一个更大的（逻辑上的）以太网网络
### 它是否与 L3 协议无关
是的。桥会看到所有帧，但*仅使* L2 头部/信息。因此，桥接功能与协议无关，转发 IPX、NetBEUI、IP、IPv6 等都不会有问题
## 联系信息

该代码目前由 Roopa Prabhu <roopa@nvidia.com> Nikolay Aleksandrov <razor@blackwall.org> 维护。桥的缺陷与增强linux-netdev 邮件列表 netdev@vger.kernel.org 以及 bridge@lists.linux.dev 上讨论
该列表对任何感兴趣的人开放：http://vger.kernel.org/vger-lists.html#netdev

## 外部链接

Linux 桥接的旧版文档位于：
https://wiki.linuxfoundation.org/networking/bridge
