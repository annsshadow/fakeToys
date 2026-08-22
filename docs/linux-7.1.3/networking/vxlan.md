
## 虚拟可扩展局域网（VXLAN）文

VXLAN 协议是一种隧道协议，旨在解决 IEEE 802.1q VLAN ID 数量有限096 个）的问题。借助 VXLAN，标识符的长度扩展到 24 位（16777216 个）
VXLAN IETF RFC 7348 描述，并已被多家厂商实现。该协议运行UDP 之上，使用单一目的端口。本文档描述的是 Linux 内核的隧道设备，此外 Openvswitch 也有一份独立的 VXLAN 实现
与大多数隧道不同，VXLAN 是一1 N 的网络，而不仅仅是一对一点对点。VXLAN 设备既可以像学习桥那样动态地学习对端 IP 地址，也可以使用静态配置的转发表项
vxlan 的管理方式与其两个最接近的邻GRE VLAN 类似。配VXLAN 需要使用与 VXLAN 首次合入上游时内核版本相匹配iproute2 版本
```

    # ip link add vxlan0 type vxlan id 42 group 239.1.1.1 dev eth1 dstport 4789

```
这会创建一个名vxlan0 的新设备。该设备通过 eth1 上的组播239.1.1.1 来处理转发表中没有表项对应的流量。目的端口号被设IANA 分配4789。Linux VXLAN 实现早于 IANA 选定标准目的端口号，为保持向后兼容，默认使用 Linux 选定的值
```

    # ip link delete vxlan0

```
```

    # ip -d link show vxlan0

```
可以使用新的 bridge 命令来创建、销毁和显示 vxlan 转发表
```

    # bridge fdb add to 00:17:42:8a:b4:05 dst 192.19.0.2 dev vxlan0

```
```

    # bridge fdb delete 00:17:42:8a:b4:05 dev vxlan0

```
```

    # bridge fdb show dev vxlan0

```
以下 NIC 特性可能意味着UDP 隧道相关卸载的支持（最常见的是 VXLAN 特性，但对特定封装协议的支持取决于具体 NIC）：

 - `tx-udp_tnl-segmentation`
 - `tx-udp_tnl-csum-segmentation`
    UDP 封装帧执TCP 分段卸载的能
 - `rx-udp_tunnel-port-offload`
    UDP 封装帧的接收端解析，NIC 能够执行协议感知的卸载，例如内层帧的校验和验证卸载（仅在没有协议无关卸载NIC 上才需要）

对于支持 `rx-udp_tunnel-port-offload` 的设备，当前列表可以
```

  $ ethtool --show-tunnels eth0
  Tunnel information for eth0:
    UDP port table 0:
      Size: 4
      Types: vxlan
      No entries
    UDP port table 1:
      Size: 4
      Types: geneve, vxlan-gpe
      Entries (1):
          port 1230, vxlan-gpe

```
