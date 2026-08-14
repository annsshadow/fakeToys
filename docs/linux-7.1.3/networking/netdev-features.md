
## Netdev 特性乱象与脱困指南


Author:
	Michał Mirosław <mirq-linux@rere.qmqm.pl>



## 第一部分：特性集合


网卡只是原封不动地收发包的日子早已一去不返。如今的設備添加了多种“特性”与“缺陷”（读懂了：即 offload 卸载），把生成与校验校验和、拆分数据包、对数据包分类等各种任务从操作系统身上卸下。这些能力及其状态在 Linux 内核中通常被称为 netdev 特性。

目前与驱动相关的特性集合有三组，另有网络核心内部使用的一组：

 1. netdev->hw_features 集合包含那些状态可能应某个设备的用户请求而改变（启用或禁用）的特性。该集合应在 ndo_init 回调中初始化，之后不可更改。

 2. netdev->features 集合包含当前为某设备启用的特性。它只应由网络核心或在 ndo_set_features 回调的出错路径中修改。

 3. netdev->vlan_features 集合包含其状态会被子 VLAN 设备继承的特性（受 netdev->features 集合限制）。目前它用于所有 VLAN 设备，无论标签是在硬件还是软件中剥离或插入。

 4. netdev->wanted_features 集合包含用户请求的特性集合。每当本集合或某些设备特定条件发生变化时，它都会被 ndo_fix_features 回调过滤。该集合是网络核心内部使用的，驱动中不应引用。



## 第二部分：控制已启用的特性


当要改变当前特性集合（netdev->features）时，会调用 ndo_fix_features 回调与 netdev_fix_features() 计算出新的集合并对其进行过滤。若结果集合与当前集合不同，则将其传入 ndo_set_features 回调，并在（该回调返回成功后）替换 netdev->features 中存储的值。之后只要当前集合可能发生变化，就会发出 NETDEV_FEAT_CHANGE 通知。

以下事件会触发重新计算：
 1. 设备注册后，ndo_init 返回成功
 2. 用户请求改变特性状态
 3. 调用了 netdev_update_features()

ndo_*_features 回调在持有 rtnl_lock 的情况下被调用。缺失的回调被视为总是返回成功。

想要触发重新计算的驱动必须通过持有 rtnl_lock 时调用 netdev_update_features() 来实现。不应从 ndo_*_features 回调中执行此操作。除通过 ndo_fix_features 回调外，驱动不应修改 netdev->features。



## 第三部分：实现提示


 - ndo_fix_features:

特性之间的所有依赖关系都应在此处解决。结果集合还可能被网络核心施加的限制进一步缩减（如 netdev_fix_features() 中所编写）。因此，当某特性的依赖未满足时，禁用该特性比强制开启其依赖更安全。

该回调不应修改硬件或驱动状态（应是无状态的）。在连续的 ndo_set_features 调用之间，它可能被多次调用。

回调不得更改 NETIF_F_SOFT_FEATURES 或 NETIF_F_NEVER_CHANGE 集合中包含的特性。唯一的例外是 NETIF_F_VLAN_CHALLENGED，但需谨慎，因为这种更改不会影响已配置的 VLAN。

 - ndo_set_features:

应重新配置硬件以匹配传入的特性集合。除非出现无法在 ndo_fix_features 中可靠检测的错误情况，否则不应更改该集合。在这种情况下，回调应将 netdev->features 更新为与最终硬件状态一致。返回的错误不会（也无法）被传播到 dmesg 以外的任何地方。（注：成功返回为零，>0 表示静默错误。）



## 第四部分：特性


有关特性的当前列表，请参阅 include/linux/netdev_features.h。本节描述其中部分特性的语义。

 - Transmit checksumming（发送校验和卸载）

完整说明请参阅 include/linux/skbuff.h 顶部的注释。

注意：NETIF_F_HW_CSUM 是 NETIF_F_IP_CSUM + NETIF_F_IPV6_CSUM 的超集。这意味着设备可以在数据包的任何位置（无论存在何种头部）填写类似 TCP/UDP 的校验和。

 - Transmit TCP segmentation offload（发送 TCP 分段卸载）

NETIF_F_TSO_ECN 表示硬件能够正确地拆分设置了 CWR 位的数据包，无论是 TCPv4（启用 NETIF_F_TSO 时）还是 TCPv6（NETIF_F_TSO6）。

 - Transmit UDP segmentation offload（发送 UDP 分段卸载）

NETIF_F_GSO_UDP_L4 接受一个带有超过 gso_size 的负载的单个 UDP 头部。在分段时，它会按 gso_size 边界对负载进行分段，并复制网络与 UDP 头部（若最后一段小于 gso_size 则进行修正）。

 - Transmit DMA from high memory（从高端内存发送 DMA）

在相关的平台上，NETIF_F_HIGHDMA 表示 ndo_start_xmit 能够处理分片（frags）位于高端内存的 skb。

 - Transmit scatter-gather（发送分散/聚集）

这些特性表示 ndo_start_xmit 能够处理分段的 skb：NETIF_F_SG —— 分页 skb（skb_shinfo()->frags），NETIF_F_FRAGLIST —— 链表式 skb（skb->next/prev 链表）。

 - Software features（软件特性）

NETIF_F_SOFT_FEATURES 中包含的特性属于网络栈的特性。驱动不应基于这些特性改变行为。

 - VLAN challenged（受 VLAN 限制）

NETIF_F_VLAN_CHALLENGED 应设置于那些无法处理 VLAN 头部的设备。某些驱动设置它是因为网卡无法处理更大的 MTU。[FIXME：这些情况可在 VLAN 代码中通过只允许减小 MTU 的 VLAN 来修复。不过这可能用处不大。]

- rx-fcs

该特性请求 NIC 将以太网帧校验和（FCS）附加到 skb 数据的末尾。这样嗅探器及其他工具就能读取 NIC 在收到数据包时记录的 CRC。

- rx-all

该特性请求 NIC 接收所有可能的帧，包括出错的帧（如错误的 FCS 等）。在嗅探存在坏包的链路时会很有帮助。某些 NIC 在同时进入普通 PROMISC（混杂）模式时可能会收到更多数据包。

- rx-gro-hw

该特性请求 NIC 启用硬件 GRO（通用接收卸载）。硬件 GRO 基本上是 TSO 的逆向操作，且通常比硬件 LRO 更严格。由硬件 GRO 合并的数据包流必须能被 GSO 或 TSO 重新分段回完全原始的包流。硬件 GRO 依赖 RXCSUM，因为硬件成功合并的每个数据包也必须由硬件完成校验和验证。

- hsr-tag-ins-offload

应在那些能自动插入 HSR（高可用无缝冗余）或 PRP（并行冗余协议）标签的设备上设置此特性。

- hsr-tag-rm-offload

应在那些能自动移除 HSR（高可用无缝冗余）或 PRP（并行冗余协议）标签的设备上设置此特性。

- hsr-fwd-offload

应在那些能在硬件中将 HSR（高可用无缝冗余）帧从一个端口转发到另一个端口的设备上设置此特性。

- hsr-dup-offload

应在那些能在硬件中自动复制外发的 HSR（高可用无缝冗余）或 PRP（并行冗余协议）标签帧的设备上设置此特性。

- netmem-tx

应在支持 netmem TX 的设备上设置此特性。请参阅 Documentation/networking/netmem.rst
