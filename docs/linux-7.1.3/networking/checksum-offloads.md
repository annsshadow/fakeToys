
## 鏍￠獙鍜屽嵏杞。

## 简

本文档描Linux 网络协议栈中一组利用各NIC 校验和卸载能力的技术
描述了以下技术：

- TX 校验和卸- LCO：本地校验和卸载（Local Checksum Offload- RCO：远程校验和卸载（Remote Checksum Offload
应当在此文档中但尚未记录的内容：

- RX 校验和卸- CHECKSUM_UNNECESSARY 转换


## TX 鏍￠獙鍜屽嵏杞。

将发送校验和卸载到设备的相关接口include/linux/skbuff.h 顶部的注释中有详细说明
简而言之，它允许请求设备根sk_buff 字段 skb->csum_start skb->csum_offset 填充
一个单一的二进制反码（ones-complement）校验和。设备应计算csum_start 到数据包末尾
16 位二进制反码校验和（即“IP 风格”的校验和），并将结果填(csum_start +
csum_offset)銆。
由于 csum_offset 不能为负，这确保了校验和字段的先前值被包含在校验和计算中，因此用它来提供对校验和的任何必要修正（例UDP TCP 的伪首部之和）
该接口只允许卸载单一的一个校验和。在使用封装时，数据包可能在不同头部层中具有多个
校验和字段，其余的将不得不由 LCO RCO 等其它机制处理
也可以通过该接口卸CRC32c：方法是按上述方式填skb->csum_start skb->csum_offset，并设置 skb->csum_not_inet：详skbuff.h 的注释（‘D节）
不对 IP 首部校验和执行卸载；它总是由软件完成。这没问题，因为当我们构IP 首部时，
显然它已在缓存中，因此对其求和并不昂贵。而且它也相当短
GSO 的要求更为复杂，因为在分段一个被封装的数据包时，内部与外部校验和可能都需要为
每个结果段进行编辑或重新计算。详skbuff.h 的注释（‘E节）
驱动netdev->hw_features 中声明其卸载能力；详Documentation/networking/netdev-features.rst。注意，一个仅声明NETIF_F_IP[V6]_CSUM 的设备仍必须遵守 SKB 中给出的 csum_start csum_offset；如它试图在硬件中自行推导这些（正如某些 NIC 所做的那样），驱动应检SKB 中的值与硬件
将推导出的值是否匹配，若不匹配，则退回到在软件中进行校验和（使用
skb_csum_hwoffload_help() skbuff.h 中提到的 skb_checksum_help() /
skb_crc32c_csum_help 函数之一）
协议栈在大多数情况下应当假定底层设备支持校验和卸载。唯一应当检查的地方validate_xmit_skb() 以及它直接或间接调用的函数。该函数比较 SKB 所请求的卸载特（其中可能包含除 TX 校验和卸载之外的其它卸载），如果设备（由 netdev->features 决定不支持或未启用这些特性，则在软件中执行相应的卸载。在 TX 校验和卸载的情况下，这意味着
调用 skb_csum_hwoffload_help(skb, features)

## LCO：本地校验和卸载


LCO 是一种在内部校验和即将被卸载时，高效计算被封装数据报外部校验和的技术
一个被正确计算校验和的 TCP UDP 数据包的二进制反码之和等于其伪首部之和的补码因为其它所有部分都被校验和字段“抵消”了。这是因为该和在写入校验和字段之前被取了补
更一般地说，只要使用“IP 风格”的二进制反码校验和，这一结论就成立，因此任何 TX 校验卸载所支持的校验和也都成立
也就是说，如果我们用一start/offset 设置TX 校验和卸载，我们就知道在设备填入校验和后，从 csum_start 到数据包末尾的二进制反码之和将等于我们事先填入校验和字段那个值的补码。这使我们无需查看载荷即可计算外部校验和：我们只需在到csum_start 停止求和，然后加上位(csum_start + csum_offset) 处的 16 位字的补码
然后，当真正的内部校验和被填入时（无论由硬件还是skb_checksum_help()），外部
校验和会因该运算而变得正确
LCO 由协议栈在为一VXLAN GENEVE 之类的封装构造外UDP 首部时，udp_set_csum() 中执行。IPv6 的对应情况类似，udp6_set_csum() 中
在构IPv4 GRE 首部时（net/ipv4/ip_gre.c:build_header()）也会执行。目前在构IPv6
GRE 首部**执行；GRE 校验和是net/ipv6/ip6_gre.c:ip6gre_xmit2() 中对整个数据计算的，但应当可以在这里使用 LCO，因IPv6 GRE 仍然使用 IP 风格的校验和
所有的 LCO 实现都使include/linux/skbuff.h 中的一个辅助函lco_csum()
LCO 可以安全地用于嵌套封装；在这种情况下，外部封装层将对其自身首部与“中间”首部两求和。这确实意味着“中间”首部会被求和多次，但似乎没有办法在不付出更大代价（例如SKB
膨胀）的情况下避免这一点

## RCO：远程校验和卸载


RCO 是一种省略被封装数据报内部校验和、从而允许外部校验和被卸载的技术。然而，它涉对封装协议的改动，接收方也必须支持该改动。因此，默认它是禁用的
RCO 在以Internet-Draft 中有详细描述
- https://tools.ietf.org/html/draft-herbert-remotecsumoffload-00
- https://tools.ietf.org/html/draft-herbert-vxlan-rco-00

Linux 中，RCO 在每个封装协议中分别实现，并且大多数隧道类型都有控制其使用的标志例如，VXLAN 具有标志 VXLAN_F_REMCSUM_TX（位struct vxlan_rdst 中），用于指示在
向给定远程目的地发送时应使RCO