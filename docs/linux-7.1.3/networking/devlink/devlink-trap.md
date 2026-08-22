
## Devlink 陷阱


## 背景


能够卸载内核数据通路并执行桥接与路由等功能的设备，还必须能够将特数据包发送到内核（即 CPU）进行处理
例如，充当支持组播的桥接设备的设备，必须能够IGMP 成员报告发送到
内核，由桥接模块处理。如果不处理此类数据包，桥接模块将永远无法填充其
MDB銆。
再举一个例子，考虑一个充当路由器的设备收到了一TTL 1 IP 数据包在路由该数据包时，设备必须将其发送到内核，以便内核也对其进行路由并生ICMP Time Exceeded 错误数据报。如果不让内核自己路由此类数据包，诸`traceroute` 这样的工具将无法工作
将某些数据包发送到内核进行处理的基本能力被称为“packet trapping”（数据陷阱）
## 概述


`devlink-trap` 机制允许具备能力的设备驱动向 `devlink` 注册其支持的数据陷阱，并`devlink` 报告被陷阱捕获的数据包以进行进一步分析
在接收到被陷阱捕获的数据包后，`devlink` 会按陷阱进行数据包与字节计数并可能通过 netlink 事件将所有提供的元数据（如陷阱原因、时间戳、输入端口）
一并报告给用户空间。这对于丢弃类陷阱（see Trap-Types）尤其有用，因为它让
用户能够进一步了解原本不可见的丢包情况
```

                                    Netlink event: Packet w/ metadata
                                                   Or a summary of recent drops
                                  ^
                                  |
         Userspace                |
        +---------------------------------------------------+
         Kernel                   |
                                  |
                          +-------+--------+
                          |                |
                          |  drop_monitor  |
                          |                |
                          +-------^--------+
                                  |
                                  | Non-control traps
                                  |
                             +----+----+
                             |         |      Kernel's Rx path
                             | devlink |      (non-drop traps)
                             |         |
                             +----^----+      ^
                                  |           |
                                  +-----------+
                                  |
                          +-------+-------+
                          |               |
                          | Device driver |
                          |               |
                          +-------^-------+
         Kernel                   |
        +---------------------------------------------------+
         Hardware                 |
                                  | Trapped packet
                                  |
                               +--+---+
                               |      |
                               | ASIC |
                               |      |
                               +------+

```

## 陷阱类型


`devlink-trap` 机制支持以下数据包陷阱类型：

  - `drop`：被陷阱捕获的数据包已被底层设备丢弃。数据包仅由 `devlink` 处理    不会注入到内核的接收路径。陷阱动作（see Trap-Actions）可以更改  - `exception`：被陷阱捕获的数据包由于异常（如 TTL 错误、缺少邻居表项）    未被底层设备按预期转发，并被陷阱捕获到控制平面以进行解析。数据包    `devlink` 处理并注入到内核的接收路径。不允许更改此类陷阱的动作，因为
    这很容易破坏控制平面  - `control`：被陷阱捕获的数据包被设备捕获，因为它们是控制平面正确运    所需的控制数据包。例ARP 请求IGMP 查询数据包。数据包被注入到
    内核的接收路径，但不会报告给内核的丢包监视器。不允许更改此类陷阱    动作，因为这很容易破坏控制平面

## 陷阱动作


`devlink-trap` 机制支持以下数据包陷阱动作：

  - `trap`：数据包的唯一副本被发送到 CPU  - `drop`：数据包被底层设备丢弃，且不发送副本到 CPU  - `mirror`：数据包被底层设备转发，并发送一份副本到 CPU
## 通用数据包陷

通用数据包陷阱用于描述捕获定义明确的数据包、或因定义明确的条件（如 TTL
错误）而被捕获的数据包的陷阱。此类陷阱可由多个设备驱动共享，其描述必添加到下表中
   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `source_mac_is_multicast`
     - `drop`
     - 捕获设备因组播源 MAC 而决定丢弃的入站数据   - - `vlan_tag_mismatch`
     - `drop`
     - 捕获设备VLAN 标签不匹配而决定丢弃的入站数据包：入桥端口未配       PVID，且数据包未带标签或仅带优先级标   - - `ingress_vlan_filter`
     - `drop`
     - 捕获设备因数据包带有入桥端口上未配置VLAN 而决定丢弃的入站数据   - - `ingress_spanning_tree_filter`
     - `drop`
     - 捕获设备因入桥端口的 STP 状态不"forwarding" 而决定丢弃的入站数据   - - `port_list_is_empty`
     - `drop`
     - 捕获设备因需要泛洪（如未知单播、未注册组播）但没有可泛洪的端口       决定丢弃的数据包
   - - `port_loopback_filter`
     - `drop`
     - 捕获设备因二层转发后唯一应发送出去的端口就是接收端口而决定丢弃的
       数据   - - `blackhole_route`
     - `drop`
     - 捕获设备因命中黑洞路由而决定丢弃的数据   - - `ttl_value_is_too_small`
     - `exception`
     - 捕获设备本应转发、但 TTL 被递减0 或更小的单播数据   - - `tail_drop`
     - `drop`
     - 捕获设备因无法入队到已满的发送队列而决定丢弃的数据   - - `non_ip`
     - `drop`
     - 捕获设备因需要执行三层查找但不是 IP MPLS 数据包而决定丢弃的数据   - - `uc_dip_over_mc_dmac`
     - `drop`
     - 捕获设备因需要路由、且目的 IP 为单播而目MAC 为组播而决定丢弃的
       数据   - - `dip_is_loopback_address`
     - `drop`
     - 捕获设备因需要路由、且目的 IP 为环回地址（即 127.0.0.0/8 ::1/128       而决定丢弃的数据   - - `sip_is_mc`
     - `drop`
     - 捕获设备因需要路由、且IP 为组播（224.0.0.0/8 ff::/8）而决       丢弃的数据包
   - - `sip_is_loopback_address`
     - `drop`
     - 捕获设备因需要路由、且IP 为环回地址（即 127.0.0.0/8 ::1/128       而决定丢弃的数据   - - `ip_header_corrupted`
     - `drop`
     - 捕获设备因需要路由、且 IP 头部损坏（校验和错误、IP 版本错误或过       Internet Header Length（IHL））而决定丢弃的数据   - - `ipv4_sip_is_limited_bc`
     - `drop`
     - 捕获设备因需要路由、且IP 为受限广播（255.255.255.255/32）       决定丢弃的数据包
   - - `ipv6_mc_dip_reserved_scope`
     - `drop`
     - 捕获设备因需要路由、且 IPv6 组播目的 IP 具有保留范围（即 ffx0::/16       而决定丢弃的 IPv6 数据   - - `ipv6_mc_dip_interface_local_scope`
     - `drop`
     - 捕获设备因需要路由、且 IPv6 组播目的 IP 具有接口本地范围（即
       ffx1::/16）而决定丢弃的 IPv6 数据   - - `mtu_value_is_too_small`
     - `exception`
     - 捕获本应由设备路由、但大于出口接口 MTU 的数据包
   - - `unresolved_neigh`
     - `exception`
     - 捕获路由后没有匹IP 邻居的数据包
   - - `mc_reverse_path_forwarding`
     - `exception`
     - 捕获组播路由中未通过反向路径转发（RPF）检查的组播 IP 数据   - - `reject_route`
     - `exception`
     - 捕获命中拒绝路由（即 "unreachable"prohibit"）的数据   - - `ipv4_lpm_miss`
     - `exception`
     - 捕获未匹配任何路由的单播 IPv4 数据   - - `ipv6_lpm_miss`
     - `exception`
     - 捕获未匹配任何路由的单播 IPv6 数据   - - `non_routable_packet`
     - `drop`
     - 捕获设备因不应被路由而决定丢弃的数据包。例如，IGMP 查询可被设备       二层泛洪并到达路由器，此类数据包不应被路由而应当丢   - - `decap_error`
     - `exception`
     - 捕获设备因解封装失败（如数据包过短、VXLAN 头部中设置了保留比特）       决定丢弃NVE IPinIP 数据   - - `overlay_smac_is_mc`
     - `drop`
     - 捕获设备因叠加网络源 MAC 为组播而决定丢弃的 NVE 数据   - - `ingress_flow_action_drop`
     - `drop`
     - 捕获在处理入向流动作 drop 时丢弃的数据   - - `egress_flow_action_drop`
     - `drop`
     - 捕获在处理出向流动作 drop 时丢弃的数据   - - `stp`
     - `control`
     - 捕获 STP 数据   - - `lacp`
     - `control`
     - 捕获 LACP 数据   - - `lldp`
     - `control`
     - 捕获 LLDP 数据   - - `igmp_query`
     - `control`
     - 捕获 IGMP 成员查询数据   - - `igmp_v1_report`
     - `control`
     - 捕获 IGMP 版本 1 成员报告数据   - - `igmp_v2_report`
     - `control`
     - 捕获 IGMP 版本 2 成员报告数据   - - `igmp_v3_report`
     - `control`
     - 捕获 IGMP 版本 3 成员报告数据   - - `igmp_v2_leave`
     - `control`
     - 捕获 IGMP 版本 2 离组数据   - - `mld_query`
     - `control`
     - 捕获 MLD 组播侦听者查询数据包
   - - `mld_v1_report`
     - `control`
     - 捕获 MLD 版本 1 组播侦听者报告数据包
   - - `mld_v2_report`
     - `control`
     - 捕获 MLD 版本 2 组播侦听者报告数据包
   - - `mld_v1_done`
     - `control`
     - 捕获 MLD 版本 1 组播侦听者完成数据包
   - - `ipv4_dhcp`
     - `control`
     - 捕获 IPv4 DHCP 数据   - - `ipv6_dhcp`
     - `control`
     - 捕获 IPv6 DHCP 数据   - - `arp_request`
     - `control`
     - 捕获 ARP 请求数据   - - `arp_response`
     - `control`
     - 捕获 ARP 应答数据   - - `arp_overlay`
     - `control`
     - 捕获到达叠加网络、经 NVE 解封装的 ARP 数据包。例如，当需要解析的
       地址是本地地址时就需要此陷阱
   - - `ipv6_neigh_solicit`
     - `control`
     - 捕获 IPv6 邻居请求数据   - - `ipv6_neigh_advert`
     - `control`
     - 捕获 IPv6 邻居通告数据   - - `ipv4_bfd`
     - `control`
     - 捕获 IPv4 BFD 数据   - - `ipv6_bfd`
     - `control`
     - 捕获 IPv6 BFD 数据   - - `ipv4_ospf`
     - `control`
     - 捕获 IPv4 OSPF 数据   - - `ipv6_ospf`
     - `control`
     - 捕获 IPv6 OSPF 数据   - - `ipv4_bgp`
     - `control`
     - 捕获 IPv4 BGP 数据   - - `ipv6_bgp`
     - `control`
     - 捕获 IPv6 BGP 数据   - - `ipv4_vrrp`
     - `control`
     - 捕获 IPv4 VRRP 数据   - - `ipv6_vrrp`
     - `control`
     - 捕获 IPv6 VRRP 数据   - - `ipv4_pim`
     - `control`
     - 捕获 IPv4 PIM 数据   - - `ipv6_pim`
     - `control`
     - 捕获 IPv6 PIM 数据   - - `uc_loopback`
     - `control`
     - 捕获需要通过接收该包的同一三层接口进行路由的单播数据包。此类数据包
       由内核路由，但也可能使其生成 ICMP 重定向数据包
   - - `local_route`
     - `control`
     - 捕获命中本地路由、需要本地投递的单播数据   - - `external_route`
     - `control`
     - 捕获应通过不属于同一设备（如交换 ASIC）的外部接口（如管理接口）进       路由的数据包
   - - `ipv6_uc_dip_link_local_scope`
     - `control`
     - 捕获需要路由、且目的 IP 地址具有链路本地范围（即 fe80::/10）的单播
       IPv6 数据包。该陷阱允许设备驱动避免编程链路本地路由，但仍能接收用于
       本地投递的数据   - - `ipv6_dip_all_nodes`
     - `control`
     - 捕获目的 IP 地址"All Nodes Address"（即 ff02::1）的 IPv6 数据   - - `ipv6_dip_all_routers`
     - `control`
     - 捕获目的 IP 地址"All Routers Address"（即 ff02::2）的 IPv6 数据   - - `ipv6_router_solicit`
     - `control`
     - 捕获 IPv6 路由器请求数据包
   - - `ipv6_router_advert`
     - `control`
     - 捕获 IPv6 路由器通告数据   - - `ipv6_redirect`
     - `control`
     - 捕获 IPv6 重定向消息数据包
   - - `ipv4_router_alert`
     - `control`
     - 捕获需要路由、且包含 Router Alert 选项IPv4 数据包。此类数据包需       本地投递到设置IP_ROUTER_ALERT socket 选项的原始套接字
   - - `ipv6_router_alert`
     - `control`
     - 捕获需要路由、且在其逐跳扩展头部中包Router Alert 选项IPv6
       数据包。此类数据包需要本地投递到设置IPV6_ROUTER_ALERT socket 选项
       的原始套接字
   - - `ptp_event`
     - `control`
     - 捕获 PTP 时间关键事件消息（Sync、Delay_req、Pdelay_Req Pdelay_Resp   - - `ptp_general`
     - `control`
     - 捕获 PTP 通用消息（Announce、Follow_Up、Delay_Resp       Pdelay_Resp_Follow_Up、管理与信令   - - `flow_action_sample`
     - `control`
     - 捕获在处理流动作 sample（如通过 tc sample 动作）时采样的数据包
   - - `flow_action_trap`
     - `control`
     - 捕获在处理流动作 trap（如通过 tc trap 动作）时记录的数据包
   - - `early_drop`
     - `drop`
     - 捕获RED（随机早期检测）算法（即早期丢弃）而丢弃的数据   - - `vxlan_parsing`
     - `drop`
     - 捕获VXLAN 头部解析错误（可能是数据包截断或 I 标志未设置）而丢       的数据包
   - - `llc_snap_parsing`
     - `drop`
     - 捕获LLC+SNAP 头部解析错误而丢弃的数据   - - `vlan_parsing`
     - `drop`
     - 捕获VLAN 头部解析错误而丢弃的数据包。可能包括意外的数据包截   - - `pppoe_ppp_parsing`
     - `drop`
     - 捕获PPPoE+PPP 头部解析错误而丢弃的数据包。可能包括发现会ID        0xFFFF（保留不可用）、PPPoE 长度大于接收到的帧，或此类头部上的任       常见错误
   - - `mpls_parsing`
     - `drop`
     - 捕获MPLS 头部解析错误（可能包括意外的头部截断）而丢弃的数据   - - `arp_parsing`
     - `drop`
     - 捕获ARP 头部解析错误而丢弃的数据   - - `ip_1_parsing`
     - `drop`
     - 捕获因第一IP 头部解析错误而丢弃的数据包。该数据包陷阱可能包括未
       通过 IP 校验和检查、头部长度检查（最20 字节）、可能因数据包截断导       总长度字段超过接收包长度等的数据   - - `ip_n_parsing`
     - `drop`
     - 捕获因最后一IP 头部（IP over IP 隧道情况下的内层头部）解析错误       丢弃的数据包。此处执行与 ip_1_parsing 陷阱相同的常见错误检   - - `gre_parsing`
     - `drop`
     - 捕获GRE 头部解析错误而丢弃的数据   - - `udp_parsing`
     - `drop`
     - 捕获UDP 头部解析错误而丢弃的数据包。该数据包陷阱可能包括校验和
       错误、检测到不合适的 UDP 长度（小8 字节）或检测到头部截断
   - - `tcp_parsing`
     - `drop`
     - 捕获TCP 头部解析错误而丢弃的数据包。可能包TCP 校验和错误、SYN       FIN RESET 的不当组合等
   - - `ipsec_parsing`
     - `drop`
     - 捕获IPSEC 头部解析错误而丢弃的数据   - - `sctp_parsing`
     - `drop`
     - 捕获SCTP 头部解析错误而丢弃的数据包。这意味着使用了端口号 0        头部被截   - - `dccp_parsing`
     - `drop`
     - 捕获DCCP 头部解析错误而丢弃的数据   - - `gtp_parsing`
     - `drop`
     - 捕获GTP 头部解析错误而丢弃的数据   - - `esp_parsing`
     - `drop`
     - 捕获ESP 头部解析错误而丢弃的数据   - - `blackhole_nexthop`
     - `drop`
     - 捕获设备因命中黑洞下一跳而决定丢弃的数据   - - `dmac_filter`
     - `drop`
     - 捕获设备因目MAC 未在 MAC 表中配置、且接口不处于混杂模式而决       丢弃的入站数据包
   - - `eapol`
     - `control`
     - 捕获 IEEE 802.1X 中规定的 "Extensible Authentication Protocol over LAN"
       （EAPOL）数据包
   - - `locked_port`
     - `drop`
     - 捕获设备因未通过锁定桥端口检查而决定丢弃的数据包。即，通过锁定端口
       接收、且{SMAC, VID} 不对应于指向该端口的 FDB 条目的数据包

## 驱动特定的数据包陷阱


设备驱动可以注册驱动特定的数据包陷阱，但必须清楚地记录。此类陷阱可对应设备特定的异常，并有助于调试由这些异常引起的丢包。以下列表包含指向各设备驱动注册的驱动特定陷阱描述的链接
  - Documentation/networking/devlink/netdevsim.rst
  - Documentation/networking/devlink/mlxsw.rst
  - Documentation/networking/devlink/prestera.rst


## 通用数据包陷阱组


通用数据包陷阱组用于聚合逻辑上相关的数据包陷阱。这些组允许用户批量操作例如设置所有成员陷阱的陷阱动作。此外，在各陷阱统计过于狭窄的情况下`devlink-trap` 可以报告按组聚合的数据包与字节统计。这些组的描述必须添到下表中
   :widths: 10 90

   - - Name
     - Description
   - - `l2_drops`
     - 包含设备在二层转发（即桥接）期间丢弃的数据包的陷   - - `l3_drops`
     - 包含设备在三层转发期间丢弃的数据包的陷阱
   - - `l3_exceptions`
     - 包含设备在三层转发期间命中异常（TTL 错误）的数据包的陷阱
   - - `buffer_drops`
     - 包含设备因入队决策而丢弃的数据包的陷阱
   - - `tunnel_drops`
     - 包含设备在隧道封解封装期间丢弃的数据包的陷阱
   - - `acl_drops`
     - 包含设备ACL 处理期间丢弃的数据包的陷   - - `stp`
     - 包含 STP 数据包的陷阱
   - - `lacp`
     - 包含 LACP 数据包的陷阱
   - - `lldp`
     - 包含 LLDP 数据包的陷阱
   - - `mc_snooping`
     - 包含组播侦听所需IGMP MLD 数据包的陷阱
   - - `dhcp`
     - 包含 DHCP 数据包的陷阱
   - - `neigh_discovery`
     - 包含邻居发现数据包（ARP、IPv6 ND）的陷阱
   - - `bfd`
     - 包含 BFD 数据包的陷阱
   - - `ospf`
     - 包含 OSPF 数据包的陷阱
   - - `bgp`
     - 包含 BGP 数据包的陷阱
   - - `vrrp`
     - 包含 VRRP 数据包的陷阱
   - - `pim`
     - 包含 PIM 数据包的陷阱
   - - `uc_loopback`
     - 包含单播环回数据包（`uc_loopback`）的陷阱。将该陷阱单独列出，       因为在诸如单臂路由器的情况下它会持续触发。为限制CPU 使用率的
       影响，可以将低速率的数据包陷阱限速器绑定到该组，而不影响其他陷阱
   - - `local_delivery`
     - 包含路由后应本地投递、但不匹配更具体的数据包陷阱（如 `ipv4_bgp`）的
       数据包的陷阱
   - - `external_delivery`
     - 包含应通过不属于同一设备（如交换 ASIC）的外部接口（如管理接口）进       路由的数据包的陷   - - `ipv6`
     - 包含各种 IPv6 控制数据包（如路由器通告）的陷阱
   - - `ptp_event`
     - 包含 PTP 时间关键事件消息（Sync、Delay_req、Pdelay_Req Pdelay_Resp       的陷   - - `ptp_general`
     - 包含 PTP 通用消息（Announce、Follow_Up、Delay_Resp       Pdelay_Resp_Follow_Up、管理与信令）的陷阱
   - - `acl_sample`
     - 包含设备ACL 处理期间采样的数据包的陷   - - `acl_trap`
     - 包含设备ACL 处理期间被陷阱捕获（记录）的数据包的陷阱
   - - `parser_error_drops`
     - 包含设备在解析期间标记为错误的包的陷   - - `eapol`
     - 包含 IEEE 802.1X 中规定的 "Extensible Authentication Protocol over LAN"
       （EAPOL）数据包的陷
## 数据包陷阱限速器


如前所述，底层设备可以将某些数据包陷阱捕获CPU 进行处理。在大多数情下，底层设备能够处理的数据包速率CPU 能处理的速率高几个数量级
因此，为了防止底层设备压CPU，设备通常包含数据包陷阱限速器，能够将陷阱
捕获的数据包限制CPU 可处理的速率
`devlink-trap` 机制允许具备能力的设备驱动向 `devlink` 注册其支持的数据陷阱限速器。设备驱动可以在初始化期间选择将这些限速器与受支持的数据包陷阱
组（see Generic-Packet-Trap-Groups）关联，从而将其默认控制平面策略暴露给
用户空间
设备驱动应通过实现相关回调函数，允许用户空间更改限速器的参数（如速率突发大小）以及限速器与陷阱组之间的关联
如果可能，设备驱动应实现一个回调函数，允许用户空间获取因违反配置的限策略而被限速器丢弃的数据包数量
## 测试


有关覆盖核心基础设施的测试，请参`tools/testing/selftests/drivers/net/netdevsim/devlink_trap.sh`。应为任新功能添加测试用例
设备驱动应将其测试重点放在设备特定的功能上，例如所支持的数据包陷阱触发