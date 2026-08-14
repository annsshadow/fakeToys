
# 分段卸载



## 简介


本文档描述了一系列 Linux 网络栈中的技术，用于利用各种 NIC 的分段卸载能力。

本文描述了以下技术：
 - TCP 分段卸载 - TSO
 - UDP 分片卸载 - UFO
 - IPIP、SIT、GRE 与 UDP 隧道卸载
 - 通用分段卸载 - GSO
 - 通用接收卸载 - GRO
 - 部分通用分段卸载 - GSO_PARTIAL
 - 基于 GSO 的 SCTP 加速 - GSO_BY_FRAGS


## TCP 分段卸载


TCP 分段允许设备将单个帧分割为多个帧，其数据负载大小由 skb_shinfo()->gso_size 指定。当请求 TCP 分段时，应在 skb_shinfo()->gso_type 中设置 SKB_GSO_TCPV4 或 SKB_GSO_TCPV6 标志位，并且 skb_shinfo()->gso_size 应被设为非零值。

TCP 分段依赖于对部分校验和卸载的支持。因此，如果某个设备的 Tx 校验和卸载被禁用，TSO 通常也会被禁用。

为了支持 TCP 分段卸载，需要填充 skbuff 的网络层与传输层头部偏移，以便设备驱动能够确定 IP 或 IPv6 头部以及 TCP 头部的偏移。此外，由于需要 CHECKSUM_PARTIAL，csum_start 也应指向数据包的 TCP 头部。



对于 IPv4 分段，我们在 IP ID 方面支持两种类型之一。默认行为是随着每个分段递增 IP ID。如果指定了 GSO 类型 SKB_GSO_TCP_FIXEDID，则不会递增 IP ID，所有分段将使用相同的 IP ID。

如果设备设置了 NETIF_F_TSO_MANGLEID，则执行 TSO 时可以忽略 IP ID，我们将对所有帧递增 IP ID，或者根据驱动偏好将其保持为静态值。对于封装的数据包，NETIF_F_TSO_MANGLEID 同时适用于外层与内层头部，除非外层头部未设置 DF 位；在此情况下，设备驱动必须保证外层头部 IP ID 字段随每个分段递增。


## UDP 分片卸载


UDP 分片卸载允许设备将一个超大的 UDP 数据报分片为多个 IPv4 分片。UDP 分片卸载的许多要求与 TSO 相同。但是，当一个 IPv4 数据报被分片时，各分片的 IPv4 ID 不应递增。

UFO 已被弃用：现代内核不再生成 UFO 类型的 skb，但仍可以从 tuntap 及类似设备接收它们。基于 UDP 的隧道协议的卸载仍然受支持。


## IPIP、SIT、GRE、UDP 隧道与远程校验和卸载


除了上述卸载之外，一个帧也可能包含额外的头部，例如外层隧道。为了应对这类情况，引入了一组额外的分段卸载类型，包括 SKB_GSO_IPXIP4、SKB_GSO_IPXIP6、SKB_GSO_GRE 和 SKB_GSO_UDP_TUNNEL。这些额外的分段类型用于标识存在多组头部的情况。例如在 IPIP 和 SIT 场景中，应将网络层和传输层头部从标准头部列表移动到“内层”头部偏移处。

目前仅支持两级头部。约定将隧道头部称为外层头部，而封装的数据通常称为内层头部。以下是可以用来访问对应头部的调用列表：

```

             Outer                  Inner
  MAC        skb_mac_header
  Network    skb_network_header     skb_inner_network_header
  Transport  skb_transport_header

```
```

             Outer                  Inner
  MAC        skb_mac_header         skb_inner_mac_header
  Network    skb_network_header     skb_inner_network_header
  Transport  skb_transport_header   skb_inner_transport_header

```
除上述隧道类型外，还有 SKB_GSO_GRE_CSUM 和 SKB_GSO_UDP_TUNNEL_CSUM。这两种额外的隧道类型表明外层头部也要求在外层头部中包含非零校验和。

最后是 SKB_GSO_TUNNEL_REMCSUM，它表示一个给定的隧道头部请求了远程校验和卸载。在这种情况下，内层头部将保留部分校验和，仅计算外层头部校验和。


## 通用分段卸载


通用分段卸载是一种纯软件卸载，用于处理设备驱动无法执行上述卸载的情况。GSO 中所做的是：将一个给定的 skbuff 的数据拆分到多个已调整大小以匹配通过 skb_shinfo()->gso_size 提供的 MSS 的 skbuff 上。

在启用任何硬件分段卸载之前，GSO 中需要存在对应的软件卸载。否则，一个帧可能在设备之间被重新路由，最终导致无法发送。


## 通用接收卸载


通用接收卸载是 GSO 的补充。理想情况下，任何由 GRO 组装的帧都应能被分段，以使用 GSO 生成完全相同的帧序列；而任何由 GSO 分段的帧序列都应能被 GRO 重新组装回原始帧。


## 部分通用分段卸载


部分通用分段卸载是 TSO 与 GSO 的混合体。它的实际作用是利用 TCP 和隧道的某些特性，使得不必为每个分段重写数据包头部，而只需更新最内层传输层头部以及可能的最外层网络层头部。这使得不支持隧道卸载或带校验和的隧道卸载的设备仍能利用分段。

在部分卸载中，除内层传输层头部外的所有头部都会被更新，以包含该头部被简单复制时所需的正确值。唯一的例外是外层 IPv4 ID 字段。设备驱动有责任保证：当某个头部未设置 DF 位时，外层 IPv4 ID 字段随每个分段递增。


## 基于 GSO 的 SCTP 加速


SCTP——尽管缺少硬件支持——仍然可以利用 GSO 让一个大数据包通过网络栈，而不是多个小数据包。

这需要与其他卸载不同的处理方式，因为 SCTP 数据包不能简单地按（P）MTU 分段。相反，数据块必须包含在 IP 分段中，并遵循填充规则。因此，与常规 GSO 不同，SCTP 不能只是生成一个大的 skb、将 gso_size 设为分片点并交付给 IP 层。

相反，SCTP 协议层构建一个 skb，其各分段已被正确填充并存储为链式 skb，skb_segment() 据此进行拆分。为了表明这一点，gso_size 被设为特殊值 GSO_BY_FRAGS。

因此，核心网络栈中的任何代码都必须意识到 gso_size 可能为 GSO_BY_FRAGS，并恰当处理该情况。

## 有一些辅助函数可以简化这一点：

- skb_is_gso(skb) && skb_is_gso_sctp(skb) 是判断一个 skb 是否为 SCTP GSO skb 的最佳方式。

- 对于大小检查，skb_gso_validate_*_len 系列辅助函数会正确处理 GSO_BY_FRAGS。

- 对于数据包的操作，skb_increase_gso_size 与 skb_decrease_gso_size 会检查 GSO_BY_FRAGS，并在被要求操作这些 skb 时发出 WARN 警告。

这也会影响设置了 NETIF_F_FRAGLIST 与 NETIF_F_GSO_SCTP 标志位的驱动。还需注意，NETIF_F_GSO_SCTP 包含在 NETIF_F_GSO_SOFTWARE 中。
