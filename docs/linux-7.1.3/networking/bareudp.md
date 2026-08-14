
## Bare UDP 隧道模块文档


当前有多种基于 UDP 的 L3 封装标准正在被讨论，以利用不同网络基于 UDP 的负载均衡能力。MPLSoUDP (https://tools.ietf.org/html/rfc7510) 就是其中之一。

Bareudp 隧道模块为在 UDP 隧道内封装 MPLS、IP、NSH 等不同 L3 协议提供了通用的 L3 封装支持。

### 特殊处理


bareudp 设备对 MPLS 与 IP 提供特殊处理，因为它们可以拥有多种 ethertype（以太类型）。MPLS 协议可以拥有 ethertype ETH_P_MPLS_UC（单播）与 ETH_P_MPLS_MC（组播）。IP 协议可以拥有 ethertype ETH_P_IP（v4）与 ETH_P_IPV6（v6）。这种特殊处理只能针对 ethertype ETH_P_IP 与 ETH_P_MPLS_UC 启用，通过一个称为 multiproto 模式的标志来实现。

### 用法


1) 设备创建与删除

    a) ip link add dev bareudp0 type bareudp dstport 6635 ethertype mpls_uc

       这将创建一个 bareudp 隧道设备，用于封装 ethertype 为 0x8847（MPLS 流量）的 L3 流量。UDP 头的目的端口将被设置为 6635。该设备将在 UDP 端口 6635 上监听以接收流量。

    b) ip link delete bareudp0

2) 启用 multiproto 模式创建设备

multiproto 模式允许 bareudp 隧道处理同一族的多种协议。目前仅可用于 IP 与 MPLS。该模式必须通过“multiproto”标志显式启用。

    a) ip link add dev bareudp0 type bareudp dstport 6635 ethertype ipv4 multiproto

       对于 IPv4 隧道，multiproto 模式允许该隧道同时处理 IPv6。

    b) ip link add dev bareudp0 type bareudp dstport 6635 ethertype mpls_uc multiproto

       对于 MPLS，multiproto 模式允许该隧道同时处理单播与组播 MPLS 报文。

3) 设备使用

bareudp 设备可与 OVS 或 TC 中的 flower 过滤器一起使用。OVS 或 TC flower 层必须在将报文缓冲区发送给 bareudp 设备进行发送之前，在 SKB 的 dst 字段中设置隧道信息。在接收时，bareUDP 设备提取隧道信息并存储在 SKB 的 dst 字段中，再将报文缓冲区传递给网络协议栈。
