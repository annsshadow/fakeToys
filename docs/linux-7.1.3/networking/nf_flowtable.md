
## Netfilter flowtable 基础设施


本文档描述了 Netfilter flowtable 基础设施，它允许你通过 flowtable 数据路径定义一快速路径（fastpath）。该基础设施也提供硬件卸载（offload）支持。flowtable 支持
3 层的 IPv4 IPv6 以及4 层的 TCP UDP 协议
### 概述


一旦数据流的首个数据包成功通过 IP 转发路径，从第二个数据包起，你就可以通过你的
规则集将这条流卸载到 flowtable。flowtable 基础设施提供了一种规则动作，允许你指何时flowtable 添加一条流
flowtable 中找到匹配条目（flowtable 命中）的数据包，会通过 neigh_xmit() 传送到输出网络设备，因此，数据包绕过了经典IP 转发路径（可见的效果是，你不会在
入口（ingress）之后任Netfilter 钩子中看到这些数据包）。如flowtable 中没匹配的条目（flowtable 未命中），数据包则沿着经典IP 转发路径行进
flowtable 使用一个可调整大小的哈希表。查找基于以n 元组选择器：2 层协议封（VLAN PPPoE）、第 3 层源和目的地址、第 4 层源和目的端口以及输入接口（在有多个
conntrack 区域（zone）时就很有用）
'flow add' 动作允许你填flowtable，由用户选择性地指定哪些流被放入 flowtable因此，除非用户通过策略显式指示某些流使用这条新的替代转发路径，否则数据包仍沿经IP 转发路径行进
flowtable 数据路径如图 1 所示，其中描述了包Netfilter 钩子flowtable 快速路绕行的经IP 转发路径
```

					 userspace process
					  ^              |
					  |              |
				     _____|____     ____\/___
				    /          \   /         \
				    |   input   |  |  output  |
				    \__________/   \_________/
					 ^               |
					 |               |
      _________      __________      ---------     _____\/_____
     /         \    /          \     |Routing |   /            \
  -->  ingress  ---> prerouting ---> |decision|   | postrouting |--> neigh_xmit
     \_________/    \__________/     ----------   \____________/          ^
       |      ^                          |               ^                |
   flowtable  |                     ____\/___            |                |
       |      |                    /         \           |                |
    __\/___   |                    | forward |------------                |
    |-----|   |                    \_________/                            |
    |-----|   |                 'flow offload' rule                       |
    |-----|   |                   adds entry to                           |
    |_____|   |                     flowtable                             |
       |      |                                                           |
      / \     |                                                           |
     /hit\_no_|                                                           |
     \ ? /                                                                |
      \ /                                                                 |
       |__yes_________________fastpath bypass ____________________________|

	       Fig.1 Netfilter hooks and flowtable interactions

```
flowtable 条目还存储了 NAT 配置，因此所有数据包都按照从经典 IP 转发路径指定NAT
策略被修改。在调用 neigh_xmit() 之前，TTL 会被减一。由于缺少传输层头，分片流量向上传递以沿经IP 转发路径行进，在这种情况下无法进flowtable 查找。TCP RST FIN 数据包也被向上传递到经典 IP 转发路径，以便优雅地释放流。超MTU 的数据包也被
向上传递到经典转发路径，以向发送方报告数据包过大的 ICMP 错误
### 配置示例


启用 flowtable 绕行相对容易，你只需创建一```

	table inet x {
		flowtable f {
			hook ingress priority 0; devices = { eth0, eth1 };
		}
		chain y {
			type filter hook forward priority 0; policy accept;
			ip protocol tcp flow add @f
			counter packets 0 bytes 0
		}
	}

```
此示例将 flowtable 'f' 添加eth0 eth1 网络设备ingress 钩子上。如果你需进行资源分区，可以创建任意数量的 flowtable。flowtable 优先级定义了管道中钩子运行的
顺序，这在你已经有一nftables ingress 链时会很方便（确flowtable 的优先级小于
nftables ingress 链，这样 flowtable 会在管道中先运行）
来自 forward 'y' 'flow offload' 动作，为来自回复方向TCP syn-ack 数据包向
flowtable 添加一条条目。一旦流被卸载，你会观察到上面示例中的计数规则不会为通过转发
绕行转发的那些数据包而更新
在列出连接跟踪表时，你可以通过 [OFFLOAD] 标签来识别被卸载的流
```

	# conntrack -L
	tcp      6 src=10.141.10.2 dst=192.168.10.2 sport=52728 dport=5201 src=192.168.10.2 dst=192.168.10.1 sport=5201 dport=52728 [OFFLOAD] mark=0 use=2


```
### 2 层封

Linux 内核 5.13 起，flowtable 基础设施会发VLAN PPPoE 网络设备背后的真网络设备。flowtable 软件数据路径会解VLAN PPPoE 2 层头部，以提取用flowtable
查找ethertype VLAN ID / PPPoE 会话 ID。flowtable 数据路径也处理第 2 层解封装
你无需PPPoE VLAN 设备添加到你flowtable，真实设备就足以flowtable 跟踪
你的流
### 桥接IP 转发


Linux 内核 5.13 起，你可以将网桥端口添加flowtable。flowtable 基础设施会发网桥设备背后的拓扑。这允许 flowtable 在你的网桥端口（在下图示例中表示eth1 eth2）与网关设备（表示为 eth0）之间定义一条快速路径绕行，位于你的交换路由器中
```

                      fastpath bypass
               .-------------------------.
              /                           \
              |           IP forwarding   |
              |          /             \ \/
              |       br0               eth0 ..... eth0
              .       / \                          *host B*
               -> eth1  eth2
                   .           *switch/router*
                   .
                   .
                 eth0
               *host A*

```
flowtable 基础设施也支持网VLAN 过滤动作，例PVID untagged。你也可以在你的
网桥端口之上堆叠一个经典的 VLAN 设备
如果你希望你flowtable 在网桥端口与 IP 转发路径之间定义一条快速路径，则必须将你的
网桥端口（由真实网络设备表示）添加到你的 flowtable 定义中
### 计数

flowtable 可以通过在你flowtable 定义中指counter 语句，将数据包和字节计数器与
现有的连接跟踪条目同步，例如
```

	table inet x {
		flowtable f {
			hook ingress priority 0; devices = { eth0, eth1 };
			counter
		}
	}

```
计数器支持自 Linux 内核 5.7 起可用
### 硬件卸载


如果你的网络设备提供硬件卸载支持，你可以通过在你flowtable 定义中使'offload'
标志来开启它，例如：

```

	table inet x {
		flowtable f {
			hook ingress priority 0; devices = { eth0, eth1 };
			flags offload;
		}
	}

```
有一个工作队列（workqueue）将流添加到硬件。注意，在工作队列有机会将流卸载到网络设之前，少数数据包可能仍会运行flowtable 软件路径上
在列出连接跟踪表时，你可以通过 [HW_OFFLOAD] 标签来识别硬件卸载的流。请注意，[OFFLOAD]
标签指的是软件卸载模式，因此 [OFFLOAD]（指软件 flowtable 快速路径）[HW_OFFLOAD]
（指该流所使用的硬件卸载数据路径）之间是有区别的
flowtable 硬件卸载基础设施也支DSA（Distributed Switch Architecture）
### 局限

flowtable 的行为类似于缓存。如果用于传输的目的 MAC 地址或出口网络设备发生变化，
flowtable 条目可能会变得陈旧（stale）
在以下情况下这可能是个问题：

- 你在软件模式下运flowtable，并且在你的配置中同时组合了桥接IP 转发- 启用了硬件卸载
### 延伸阅读


本文档基LWN.net 的文[^1^]_\ [^2^]_。Rafal Milecki 也撰写了一篇非常完整而全面的
总结，名"A state of network acceleration"，描述了在此基础设施被合并入主线之前
的情[^3^]_，并对此项工作做了一个粗略的总结 [^4^]_