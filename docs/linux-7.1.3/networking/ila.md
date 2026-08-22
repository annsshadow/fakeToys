
## 标识位置寻址（ILA
## 简
标识位置寻址（Identifier-locator addressing，ILA）是一种配IPv6 使用的技术，用于区分网络节点的位置与身份。地址的一部分表达该节点不可变的身份（identity），另一部分指示节点的位置（location），而位置可以是动态的。标识符-位置寻址可用于高效地实现用于网络虚拟化的overlay网络，以及用于移动性场景的各类解决方案
ILA 可以被视为一种无需封装即可实现 overlay 网络的方法。这是通过在一个数据包穿越网络时对其目的地址进行网络地址转换（NAT）来完成的。对网络而言，经ILA 转换的数据包与任何其IPv6 数据包并无不同。例如，如果传输协议TCP，那么经ILA 转换的数据包看起来就只是另一TCP/IPv6 数据包。这样做的好处是 ILA 对网络是透明的，从而使网络中的各项优化（如 ECMP、RSS、GRO、GSO 等）都能照常工作
ILA 协议Internet-Draft draft-herbert-intarea-ila 中描述
## ILA 术语

  - Identifier（标识符		一个用于标识网络中可寻址节点、与其位置无关的数字		ILA 标识符为六十四位值
  - Locator（位置符		一条路由到物理主机的网络前缀。位置符提供了被寻址节点的拓扑位置		ILA 位置符为六十四位前缀
  - ILA mapping（ILA 映射		将一ILA 标识符映射到位置符（或位置符加元数据）的映射关系		一ILA 域维护一个数据库，其中包含该域中所有目的地的映射
  - SIR address（SIR 地址		一个由 SIR 前缀（高六十四位）和标识符（低六十四位）组成IPv6 地址		SIR 地址对应用程序可见，并为它们提供了一种与节点位置无关的寻址手段
  - ILA address（ILA 地址		一个由位置符（高六十四位）和标识符（低六十四位）组成的 IPv6 地址		ILA 地址绝不对应用程序可见
  - ILA host（ILA 主机		能够在发送或接收时执ILA 转换的端主机
  - ILA router（ILA 路由器）
		执行 ILA 转换并转发已转换数据包的网络节点
  - ILA forwarding cache（ILA 转发缓存		一种仅维护映射的工作集缓存ILA 路由器
  - ILA node（ILA 节点		能够执行 ILA 转换的网络节点。可以是 ILA 路由器、ILA 转发缓存ILA 主机
## 运作

ILA 有两个基本操作：

  - SIR 地址转换ILA 地址。这在进ILA overlay 时执行
  - ILA 地址转换SIR 地址。这在离开 ILA overlay 时执行
ILA 可以部署在端主机或网络中的中间设备上；分别由“ILA 主机”和“ILA 路由器”提供。这两类部署点的配置与数据路径有所不同
下图展示了数据包流经 ILA 的过```

    +--------+                                                +--------+
    | Host A +-+                                         +--->| Host B |
    |        | |              (2) ILA                   (')   |        |
    +--------+ |            ...addressed....           (   )  +--------+
	       V  +---+--+  .  packet      .  +---+--+  (_)
   (1) SIR     |  | ILA  |----->-------->---->| ILA  |   |   (3) SIR
    addressed  +->|router|  .              .  |router|->-+    addressed
    packet        +---+--+  .     IPv6     .  +---+--+        packet
		   /        .    Network   .
		  /         .              .   +--+-++--------+
    +--------+   /          .              .   |ILA ||  Host  |
    |  Host  +--+           .              .- -|host||        |
    |        |              .              .   +--+-++--------+
    +--------+              ................


```
## 传输层校验和处理

当地址ILA 转换时，伪首部中包含被转换地址的封装传输层校验和，在线缆上可能变得不正确。这对包NIC 中校验和卸载在内的中间设备构成了问题，因为它们需要处理校验和。有三种方法来处理它
- no action（不采取措施允许校验和在线缆上不正确。在接收方验证校验和之前，必须完ILA SIR 地址的转换
- adjust transport checksum（调整传输层校验和）
		执行 ILA 转换时解析数据包，如果发现传输层校验和，则对其进行调整以反映依据被转换地址计算出的正确校验和
- checksum neutral mapping（校验和中立映射		当地址被转换时，差值可以在数据包中受校验和覆盖的其他位置进行抵消。使用标识符的低十六位。这种方法更受青睐，因为它不需要解IP 首部之外的数据包，且多数情况下调整值可以预先计算并与映射一同保存
注意，校验和中立调整会影响标识符的低十六位。当在出口处执行 ILA SIR 地址转换时，低十六位被恢复为原始值，从而将标识符恢复为最初发送时的样子
## 鏍囪瘑绗︾被鍨?
ILA 为不同用例定义了不同类型的标识符
已定义的类型有：

      0: 接口标识符（interface identifier
      1: 本地唯一标识符（locally unique identifier
      2: 用于 IPv4 地址的虚拟网络标识符

      3: 用于 IPv6 单播地址的虚拟网络标识符

      4: 用于 IPv6 组播地址的虚拟网络标识符

      5: 非本地地址标识符（non-local address identifier
在内ILA 的当前实现中，仅支持本地唯一标识符（LUID）。LUID 允许一个通用的、无格式64 位标识符
## 标识符格
内核 ILA 在标识符中支持两个用于格式化的可选字段：“C-bit”和“标识符类型（identifier type）”。这些字段是否存在由配置决定，如下所述
如果标识符类型存在，它占用标识符最高的三位。可能的值见上面的列表
如果 C-bit 存在，则用它作为已执行校验和中立映射的标识。C-bit 只能设置ILA 地址中，绝不能出现在 SIR 地址中
在最简单的格式中，标识符类型、C-bit 和校验和调整值均不存在，因此标识符被视为
```

     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                            Identifier                         |
     +                                                               +
     |                                                               |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

```
校验和中立调整可通过 neutral-map-auto 配置为始终存在。在这种情况下没C-bit，但校验和调整位于低 16 位。标识符```

     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                            Identifier                         |
     |                               +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                               |  Checksum-neutral adjustment  |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

```
C-bit 可用于显式指示校验和中立
```

     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |     |C|                    Identifier                         |
     |     +-+                       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                               |  Checksum-neutral adjustment  |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

```
标识符类型字段可能存在，用以指示标识符类型。如果它不存在，则类型根据映射配置推断。校验和中立调整可能自动
```

     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     | Type|                      Identifier                         |
     +-+-+-+                         +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                               |  Checksum-neutral adjustment  |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

```
如果标识符类型和 C-bit 可以同时存在，则
```

     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     | Type|C|                    Identifier                         |
     +-+-+-+-+                       +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                               |  Checksum-neutral adjustment  |
     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+


```
## 配置

配置 ILA 映射有两种方法。一种是使用 LWT 路由，另一种是 ila_xlat（从 NFHOOK PREROUTING 钩子调用）。ila_xlat 旨在用于 ILA 主机的接收路径
ILA 路由器也已在 XDP 中实现。其描述超出了本文档范围
ILA LWT 路由的用法为
ip route add DEST/128 encap ila LOC csum-mode MODE ident-type TYPE via ADDR

目的地址（DEST）可以是 SIR 地址（用ILA 主机或入ILA 路由器），也可以ILA 地址（出ILA 路由器）。LOC 是覆盖目的地址高六十四位的六十四位位置符（格式W:X:Y:Z）。校验和 MODE 为“no-action”、“adj-transport”、“neutral-map”和“neutral-map-auto”之一。如果设置了 neutral-map，则 C-bit 将存在。标识符 TYPE 为“luid”或“use-format”之一。在 use-format 的情况下，标识符类型字段存在，有效类型取自该字段
ila_xlat 的用法为
ip ila add loc_match MATCH loc LOC csum-mode MODE ident-type TYPE

MATCH 指示必须被匹配以应用转换的入向位置符。LOC 是覆盖目的地址高六十四位的位置符。MODE TYPE 的含义如上所述
## 一些示
```

     # 配置一条同时使用校验和中立映射以及类型字段ILA 路由     # 注意类型字段设置SIR 地址中（2000 表示类型1，即 LUID）     ip route add 3333:0:0:1:2000:0:1:87/128 encap ila 2001:0:87:0 \
	  csum-mode neutral-map ident-type use-format

     # 配置一条使用自动校验和中立映射（无 C-bit）的 ILA LWT 路由     # 并将标识符类型配置为 LUID，从而标识符类型字段将不存在     ip route add 3333:0:0:1:2000:0:2:87/128 encap ila 2001:0:87:1 \
	  csum-mode neutral-map-auto ident-type luid

     ila_xlat 配置

     # 配置一条匹配位置符并将其覆盖为 SIR 地址（本例中3333:0:0:1）的
     # ILA SIR 映射。使C-bit 和标识符字段     ip ila add loc_match 2001:0:119:0 loc 3333:0:0:1 \
	 csum-mode neutral-map-auto ident-type use-format

     # 配置一条自动设置校验和中立（无 C-bit）且标识符类型配置为 LUID      # ILA SIR 映射，从而标识符类型字段不存在     ip ila add loc_match 2001:0:119:0 loc 3333:0:0:1 \
	 csum-mode neutral-map-auto ident-type use-format


```
