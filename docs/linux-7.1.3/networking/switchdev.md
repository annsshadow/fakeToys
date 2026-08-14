
## 以太网交换设备驱动模型（switchdev）


Copyright |copy| 2014 Jiri Pirko <jiri@resnulli.us>

Copyright |copy| 2014-2015 Scott Feldman <sfeldma@gmail.com>


以太网交换设备驱动模型（switchdev）是一种内核内驱动模型，用于那些将
转发（数据）平面从内核卸载的交换设备。

图 1 是一个框图，展示了 switchdev 模型在使用数据中心级交换 ASIC 芯片的
示例配置中的各个组件。其他使用 SR-IOV 或软交换机（如 OVS）的配置也是
可能的。


```

			     User-space tools

       user space                   |
      +-------------------------------------------------------------------+
       kernel                       | Netlink
				    |
		     +--------------+-------------------------------+
		     |         Network stack                        |
		     |           (Linux)                            |
		     |                                              |
		     +----------------------------------------------+

			   sw1p2     sw1p4     sw1p6
		      sw1p1  +  sw1p3  +  sw1p5  +          eth1
			+    |    +    |    +    |            +
			|    |    |    |    |    |            |
		     +--+----+----+----+----+----+---+  +-----+-----+
		     |         Switch driver         |  |    mgmt   |
		     |        (this document)        |  |   driver  |
		     |                               |  |           |
		     +--------------+----------------+  +-----------+
				    |
       kernel                       | HW bus (eg PCI)
      +-------------------------------------------------------------------+
       hardware                     |
		     +--------------+----------------+
		     |         Switch device (sw1)   |
		     |  +----+                       +--------+
		     |  |    v offloaded data path   | mgmt port
		     |  |    |                       |
		     +--|----|----+----+----+----+---+
			|    |    |    |    |    |
			+    +    +    +    +    +
		       p1   p2   p3   p4   p5   p6

			     front-panel ports


				    Fig 1.


```

### 包含文件


```

    #include <linux/netdevice.h>
    #include <net/switchdev.h>


```

### 配置


在驱动的 Kconfig 中使用 "depends NET_SWITCHDEV"，以确保为驱动构建
switchdev 模型支持。


### 交换端口


在 switchdev 驱动初始化时，驱动会为每个枚举到的物理交换端口分配并注册
一个 struct net_device（使用 register_netdev()），称为端口 netdev。端口
netdev 是物理端口的软件表示，并提供了一个通道，用于在控制器（内核）与
网络之间传输控制流量，同时也是桥接、绑定（bond）、VLAN、隧道和 L3 路由器
等高层结构的锚点。使用标准的 netdev 工具（iproute2、ethtool 等），端口
netdev 还可以向用户提供对交换端口物理属性（如 PHY 链路状态和 I/O 统计
信息）的访问。

（目前）除了端口 netdev 之外，没有更高层的针对交换机的 kernel 对象。
所有的 switchdev 驱动操作都是 netdev 操作或 switchdev 操作。

交换机管理端口不在 switchdev 驱动模型的范围内。通常，管理端口不参与
卸载的数据平面，并在管理端口设备上加载一个不同的驱动（例如 NIC 驱动）。


##### 交换机 ID


switchdev 驱动必须为每个端口 netdev 实现 net_device 操作
ndo_get_port_parent_id，为交换机的每个端口返回相同的物理 ID。该 ID 在同一
系统上的不同交换机之间必须唯一。该 ID 在不同系统上的交换机之间无需唯一。

交换机 ID 用于定位交换机上的端口，并判断聚合端口是否属于同一台交换机。


##### 端口 Netdev 命名


应使用 udev 规则进行端口 netdev 命名，以端口的某个唯一属性作为键，例如
端口 MAC 地址或端口 PHYS 名称。不鼓励在驱动内对内核 netdev 名称进行硬编码；
让内核选择默认的 netdev 名称，并让 udev 基于端口属性设置最终名称。

使用端口 PHYS 名称（ndo_get_phys_port_name）作为键，对于基于外部配置命名
其端口的动态命名端口特别有用。例如，如果一个物理 40G 端口在逻辑上被拆分为
4 个 10G 端口，从而产生 4 个端口 netdev，设备可以给出一个唯一的
```

    SUBSYSTEM=="net", ACTION=="add", ATTR{phys_switch_id}=="<phys_switch_id>", \
	    ATTR{phys_port_name}!="", NAME="swX$attr{phys_port_name}"

```

建议的命名约定是 "swXpYsZ"，其中 X 是交换机名称或 ID，Y 是端口名称或 ID，
Z 是子端口名称或 ID。例如，sw1p1s0 将是交换机 1 上端口 1 的子端口 0。


##### 端口特性


dev->netns_immutable

如果 switchdev 驱动（和设备）仅支持卸载默认网络命名空间（netns），则驱动
应设置此私有标志，以防止端口 netdev 被移出默认 netns。一个感知 netns 的
驱动/设备将不会设置此标志，并负责划分硬件以保留 netns 隔离。这意味着硬件
不能将流量从一个命名空间中的端口转发到另一个命名空间中的另一个端口。


##### 端口拓扑


表示物理交换端口的端口 netdev 可以被组织成更高层的交换结构。默认结构是
独立的路由器端口，用于卸载 L3 转发。两个或更多端口可以绑定在一起形成 LAG。
两个或更多端口（或 LAG）可以被桥接以桥接 L2 网络。可以应用 VLAN 来细分
L2 网络。可以在端口上构建 L2-over-L3 隧道。这些结构是使用标准的 Linux 工具
（如 bridge 驱动、bonding/team 驱动）以及基于 netlink 的工具（如 iproute2）
构建的。

switchdev 驱动可以通过监控 NETDEV_CHANGEUPPER 通知来获知特定端口在拓扑中的
位置。例如，一个端口被移入一个 bond，会看到它的上层主设备发生变化。如果
该 bond 被移入一个桥，bond 的上层主设备会发生变化。依此类推。驱动将通过
注册 netdevice 事件并对 NETDEV_CHANGEUPPER 做出响应，来跟踪此类移动以了解
端口在整个拓扑中的位置。


### L2 转发卸载


其思路是通过将桥的 FDB 条目镜像到设备，将 L2 数据转发（交换）路径从内核
卸载到 switchdev 设备。一个 FDB 条目是 {port, MAC, VLAN} 元组形式的转发
目的地。

要卸载 L2 桥接，switchdev 驱动/设备应支持：

 - 安装在桥端口上的静态 FDB 条目
 - 来自设备的已学习/已遗忘的源 mac/vlan 的通知
 - 端口上的 STP 状态变更
 - 组播/广播和未知单播报文的 VLAN 泛洪


##### 静态 FDB 条目


实现了 `ndo_fdb_add`、`ndo_fdb_del` 和 `ndo_fdb_dump` 操作的驱动能够支持
以下命令，该命令添加一个
```

        bridge fdb add dev DEV ADDRESS [vlan VID] [self] static

```

（“static”关键字是非可选的：如果未指定，该条目默认为“local”，这意味着
不应被转发）

“self”关键字（可选，因为它是隐式的）的作用是指导内核通过 `DEV` 设备自身的
`ndo_fdb_add` 实现来完成该操作。如果 `DEV` 是一个桥端口，这将绕过桥，从而
使软件数据库与硬件数据库失去同步。

```

        bridge fdb add dev DEV ADDRESS [vlan VID] master static

```

上述命令指导内核搜索 `DEV` 的主接口，并通过它的 `ndo_fdb_add` 方法来完成
该操作。这一次，桥会生成一个 `SWITCHDEV_FDB_ADD_TO_DEVICE` 通知，端口驱动
可以处理该通知并用它来编程其硬件表。这样，软件和硬件数据库都将包含这个
静态 FDB 条目。

注意：对于卸载 Linux 桥的新 switchdev 驱动，强烈不建议实现 `ndo_fdb_add`
和 `ndo_fdb_del` 桥绕过方法：所有静态 FDB 条目都应使用“master”标志添加到
桥端口上。`ndo_fdb_dump` 是个例外，如果设备没有用于通知操作系统新学习/已
遗忘的动态 FDB 地址的中断，则可以实现它来可视化硬件表。在这种情况下，硬件
FDB 可能最终拥有软件 FDB 所没有的条目，而实现 `ndo_fdb_dump` 是查看它们的
唯一方法。

注意：默认情况下，桥不对 VLAN 进行过滤，只桥接不带标签的
```

        echo 1 >/sys/class/net/<bridge>/bridge/vlan_filtering

```


##### 已学习/已遗忘源 MAC/VLAN 的通知


交换设备会在入口报文上学习/遗忘源 MAC 地址/VLAN，并通知交换驱动 mac/vlan/
port 元组。交换驱动会
```

	err = call_switchdev_notifiers(val, dev, info, extack);

```

其中 val 在学习时为 SWITCHDEV_FDB_ADD，在遗忘时为 SWITCHDEV_FDB_DEL，info
指向一个 struct switchdev_notifier_fdb_info。在 SWITCHDEV_FDB_ADD 时，桥驱动
会将 FDB 条目安装到桥的 FDB 中，并将该条目标记为 NTF_EXT_LEARNED。iproute2
bridge
```

	$ bridge fdb
	52:54:00:12:35:01 dev sw1p1 master br0 permanent
	00:02:00:00:02:00 dev sw1p1 master br0 offload
	00:02:00:00:02:00 dev sw1p1 self
	52:54:00:12:35:02 dev sw1p2 master br0 permanent
	00:02:00:00:03:00 dev sw1p2 master br0 offload
	00:02:00:00:03:00 dev sw1p2 self
	33:33:00:00:00:01 dev eth0 self permanent
	01:00:5e:00:00:01 dev eth0 self permanent
	33:33:ff:00:00:00 dev eth0 self permanent
	01:80:c2:00:00:0e dev eth0 self permanent
	33:33:00:00:00:01 dev br0 self permanent
	01:00:5e:00:00:01 dev br0 self permanent
	33:33:ff:12:35:01 dev br0 self permanent

```

```

	bridge link set dev DEV learning off

```

```

	bridge link set dev DEV learning on self
	bridge link set dev DEV learning_sync on self

```

learning_sync 属性用于启用将已学习/已遗忘的 FDB 条目同步到桥的 FDB。可以
（但不是最优地）在设备端口和桥端口上都启用学习，并禁用 learning_sync。

为了支持学习，驱动实现 switchdev 操作 switchdev_port_attr_set 以处理
SWITCHDEV_ATTR_PORT_ID_{PRE}_BRIDGE_FLAGS。


##### FDB 老化


桥会跳过对标记为 NTF_EXT_LEARNED 的 FDB 条目的老化，由端口驱动/设备负责
将这些条目老化掉。如果端口设备支持老化，当 FDB 条目过期时，它会通知驱动，
驱动进而使用 SWITCHDEV_FDB_DEL 通知桥。如果设备不支持老化，驱动可以使用
垃圾回收定时器来模拟老化以监控 FDB 条目。过期的条目将使用 SWITCHDEV_FDB_DEL
通知给桥。参见 rocker 驱动中运行老化定时器的示例。

为了使一个 NTF_EXT_LEARNED 条目保持“存活”，驱动应通过调用
call_switchdev_notifiers(SWITCHDEV_FDB_ADD, ...) 来刷新该 FDB 条目。该通知
会将 FDB 条目的最后使用时间重置为当前时间。驱动应限制刷新通知的速率，例如
每秒不超过一次。（最后使用时间可通过 bridge -s fdb 选项查看）。


##### 端口上的 STP 状态变更


在内部或通过第三方 STP 协议实现（例如 mstpd），桥驱动维护端口的 STP 状态，
并将使用 switchdev 操作 switchdev_attr_port_set 针对
SWITCHDEV_ATTR_PORT_ID_STP_UPDATE 通知交换驱动端口的 STP 状态变更。

状态是 BR_STATE_* 之一。交换驱动可以使用 STP 状态更新来更新端口的入口报文
过滤列表。例如，如果端口是 DISABLED，则不应让任何报文通过；但如果端口移动到
BLOCKED，则 STP BPDU 和其他 IEEE 01:80:c2:xx:xx:xx 链路本地组播报文可以通过。

注意，STP BPDU 是不带标签的，且 STP 状态适用于端口上的所有 VLAN，因此报文
过滤器应在端口上不带标签和带标签的 VLAN 之间一致地应用。


##### 泛洪 L2 域


对于给定的 L2 VLAN 域，如果端口当前的 STP 状态允许，交换设备应将组播/广播
和未知单播报文泛洪到域内的所有端口。交换驱动知道哪些端口位于哪个 VLAN L2
域内，可以对交换设备进行泛洪编程。该报文可能被发送到端口 netdev 由桥驱动
处理。桥不应将该报文重新泛洪到设备已泛洪的相同端口，否则线上会出现重复报文。

为避免重复报文，交换驱动应通过设置 skb->offload_fwd_mark 位将报文标记为已被
转发。桥驱动会使用入口桥端口的标记来标记 skb，并阻止其通过任何具有相同标记的
桥端口转发。

交换设备也有可能不处理泛洪，而是将报文上送到桥驱动进行泛洪。这并不理想，
因为随着 L2 域中端口数量的增加，设备在进行报文泛洪时比软件高效得多。

如果设备支持，可以将泛洪控制卸载到设备上，防止某些 netdev 泛洪那些没有 FDB
条目的单播流量。


##### IGMP 嗅探


为了支持 IGMP 嗅探，端口 netdev 应将所有 IGMP 加入和离开消息陷阱到桥驱动。
桥组播模块会在每个组播组发生变化时通知端口 netdev，无论它是静态配置的还是
动态加入/离开的。硬件实现应当仅将已注册的所有组播流量组转发到配置的端口。


### L3 路由卸载


卸载 L3 路由要求使用来自内核的 FIB 条目对设备进行编程，由设备执行 FIB 查找
和转发。设备对匹配路由前缀的 FIB 条目执行最长前缀匹配（LPM），并将报文转发
到匹配 FIB 条目的下一跳（nexthop）出口端口。

为了对设备进行编程，驱动必须使用 register_fib_notifier 注册一个 FIB 通知
处理程序。可用事件如下：

===================  ===================================================
FIB_EVENT_ENTRY_ADD  used for both adding a new FIB entry to the device,
		     or modifying an existing entry on the device.
FIB_EVENT_ENTRY_DEL  used for removing a FIB entry
FIB_EVENT_RULE_ADD,
FIB_EVENT_RULE_DEL   used to propagate FIB rule changes
===================  ===================================================


```

	struct fib_entry_notifier_info {
		struct fib_notifier_info info; /* must be first */
		u32 dst;
		int dst_len;
		struct fib_info *fi;
		u8 tos;
		u8 type;
		u32 tb_id;
		u32 nlflags;
	};

```

用于在表 tb_id 上添加/修改/删除 IPv4 dst/dest_len 前缀。`*fi` 结构保存了
路由及其下一跳的详细信息。`*dev` 是路由下一跳列表中提到的端口 netdev 之一。

卸载到设备的路由在 ip route 中标记为 "offload"
```

	$ ip route show
	default via 192.168.0.2 dev eth0
	11.0.0.0/30 dev sw1p1  proto kernel  scope link  src 11.0.0.2 offload
	11.0.0.4/30 via 11.0.0.1 dev sw1p1  proto zebra  metric 20 offload
	11.0.0.8/30 dev sw1p2  proto kernel  scope link  src 11.0.0.10 offload
	11.0.0.12/30 via 11.0.0.9 dev sw1p2  proto zebra  metric 20 offload
	12.0.0.2  proto zebra  metric 30 offload
		nexthop via 11.0.0.1  dev sw1p1 weight 1
		nexthop via 11.0.0.9  dev sw1p2 weight 1
	12.0.0.3 via 11.0.0.1 dev sw1p1  proto zebra  metric 20 offload
	12.0.0.4 via 11.0.0.9 dev sw1p2  proto zebra  metric 20 offload
	192.168.0.0/24 dev eth0  proto kernel  scope link  src 192.168.0.15

```

只要有至少一个设备卸载了该 FIB 条目，“offload”标志就会被设置。

XXX：add/mod/del IPv6 FIB API


##### 下一跳解析


FIB 条目的下一跳列表包含下一跳元组（gateway、dev），但为了让交换设备以正确
的 dst mac 地址转发报文，必须将下一跳网关解析为邻居的 mac 地址。邻居 mac
地址的发现通过 ARP（或 ND）过程完成，并可通过 arp_tbl 邻居表获取。为了解析
路由的下一跳网关，驱动应触发内核的邻居解析过程。参见 rocker 驱动的
rocker_port_ipv4_resolve() 作为示例。

驱动可以使用 netevent 通知 NETEVENT_NEIGH_UPDATE 监控 arp_tbl 的更新。随着
arp_tbl 的更新，可以为路由编程已解析的下一跳到设备中。驱动实现 ndo_neigh_destroy
以获知 arp_tbl 邻居条目何时从端口中被清除。


### 设备驱动预期行为


以下是一组 switchdev 启用的网络设备必须遵守的已定义行为。


##### 无配置状态


在驱动启动（bring up）时，网络设备必须完全可运行，并且底层驱动必须配置网络设备，
使其有可能向该网络设备发送和接收流量，并且它与其他网络设备/端口适当地隔离
（例如：在交换 ASIC 中经常如此）。如何实现这一点在很大程度上取决于硬件，但一个
简单的解决方案是使用每端口的 VLAN 标识符，除非有更好的机制可用（例如每个网络
端口的专有元数据）。

网络设备必须能够运行完整的 IP 协议栈，包括组播、DHCP、IPv4/6 等。如有必要，它
应编程适当的 VLAN、组播、单播等过滤器。底层设备驱动必须以类似于为这些 switchdev
网络设备启用 IGMP 嗅探时的方式进行有效配置，并且非请求组播必须尽可能早地在硬件
中被过滤。

在配置网络设备上方的 VLAN 时，所有 VLAN 都必须正常工作，而不受其他网络设备状态
的影响（例如：作为 VLAN 感知桥一部分、进行入口 VID 检查的其他端口）。详见下文。

如果设备实现了例如 VLAN 过滤，将接口置于混杂（promiscuous）模式应允许接收所有
VLAN 标签（包括过滤器中不存在的那些）。


##### 已桥接的交换端口


当将一个 switchdev 启用的网络设备添加为桥成员时，它不应干扰任何非桥接网络设备
的功能，并且它们应继续表现为正常的网络设备。根据下文的桥配置开关，预期行为如下
所述。


##### 桥接 VLAN 过滤


Linux 桥允许配置 VLAN 过滤模式（静态地，在设备创建时；或动态地，在运行时），这
必须被底层 switchdev 网络设备/硬件遵守：

- 关闭 VLAN 过滤时：桥严格地对 VLAN 无感知，其数据路径会将所有以太网帧当作不带
  VLAN 标签的帧处理。桥的 VLAN 数据库仍可被修改，但在 VLAN 过滤关闭期间这些修改
  应不起作用。以未编程到桥/交换机的 VLAN 表中的 VID 进入设备的帧必须被转发，并且
  可以使用 VLAN 设备进行处理（见下文）。

- 打开 VLAN 过滤时：桥对 VLAN 有感知，以未编程到桥/交换机的 VLAN 表中的 VID 进入
  设备的帧必须被丢弃（严格的 VID 检查）。

当在作为桥端口成员的 switchdev 网络设备上配置有 VLAN 设备（例如：sw0p1.100）时，
必须保留软件网络栈的行为，如果不可能则必须拒绝该配置。

- 关闭 VLAN 过滤时，桥将处理该端口的所有入口流量，但发往 VLAN 上层的、带有某 VLAN
  ID 标签的流量除外。VLAN 上层接口（它消费 VLAN 标签）甚至可以被添加到第二个桥中，
  该桥包含其他交换端口或软件接口。一些确保属于 VLAN 上层接口的流量转发域被正确管理
  的方法：

    - 如果转发目的地可以逐 VLAN 管理，则可以将硬件配置为将所有流量（标记为属于某个
      VLAN 上层接口的 VID 的报文除外）映射到一个对应于不带标签报文的内部 VID。这个
      内部 VID 跨越 VLAN 无感知桥的所有端口。对应于 VLAN 上层接口的 VID 跨越该 VLAN
      接口的物理端口，以及可能与它桥接的其他端口。
    - 将带有 VLAN 上层接口的桥端口视为独立的，并让转发在软件数据路径中处理。

- 打开 VLAN 过滤时，只要桥在任何桥端口上不存在具有相同 VID 的现有 VLAN 条目，就可以
  创建这些 VLAN 设备。这些 VLAN 设备不能被 enslave 到桥中，因为它们与桥的 VLAN 数据
  路径处理在功能/用例上重复。

同一交换 fabric 的非桥接网络端口绝不应因在桥设备上启用 VLAN 过滤而受到任何干扰。如果
VLAN 过滤设置对整个芯片是全局的，那么独立端口应通过在其 ethtool 特性中设置
'rx-vlan-filter: on [fixed]' 来向网络栈表明需要 VLAN 过滤。

由于 VLAN 过滤可以在运行时打开/关闭，switchdev 驱动必须能够即时重新配置底层硬件以
遵从该选项的切换并表现得当。如果不可能，switchdev 驱动也可以拒绝支持运行时 VLAN
过滤开关的动态切换，而要求销毁桥设备并创建具有不同 VLAN 过滤值的新桥设备，以确保
将 VLAN 感知下推到硬件。

即使桥中的 VLAN 过滤被关闭，底层交换硬件和驱动仍可以将自身配置为 VLAN 感知模式，
前提是遵守上述描述的行为。

桥的 VLAN 协议在决定一个报文是否被视为带标签时起作用：使用 802.1ad 协议的桥必须将
不带 VLAN 标签的报文以及带有 802.1Q 头部的报文都视为不带标签的。

设备必须以与不带标签报文相同的方式处理 802.1p（VID 0）带标签的报文，因为桥设备不
允许在其数据库中操作 VID 0。

当桥启用了 VLAN 过滤且在入口端口上未配置 PVID 时，不带标签和 802.1p 带标签的报文
必须被丢弃。当桥启用了 VLAN 过滤且在入口端口上存在 PVID 时，不带标签和带优先级标签的
报文必须被接受，并根据桥对该 PVID VLAN 的端口成员关系进行转发。当桥禁用了 VLAN 过滤
时，PVID 的存在/缺失不应影响报文转发决策。


##### 桥接 IGMP 嗅探


Linux 桥允许配置 IGMP 嗅探（静态地，在接口创建时；或动态地，在运行时），这必须被
底层 switchdev 网络设备/硬件以下列方式遵守：

- 当 IGMP 嗅探关闭时，组播流量必须被泛洪到同一桥内所有 mcast_flood=true 的端口。
  CPU/管理端口理想情况下不应被泛洪（除非入口接口具有 IFF_ALLMULTI 或 IFF_PROMISC），
  并继续通过网络栈通知学习组播流量。如果硬件无法做到这一点，那么 CPU/管理端口也必须
  被泛洪，组播过滤在软件中进行。

- 当 IGMP 嗅探打开时，组播流量必须有选择地流向适当的网络端口（包括 CPU/管理端口）。
  未知组播的泛洪应仅朝向连接到组播路由器的端口（本地设备也可以充当组播路由器）。

交换机必须遵守 RFC 4541 并相应地泛洪组播流量，因为这正是 Linux 桥实现所做的。

由于 IGMP 嗅探可以在运行时打开/关闭，switchdev 驱动必须能够即时重新配置底层硬件以
遵从该选项的切换并表现得当。

switchdev 驱动也可以拒绝支持运行时组播嗅探开关的动态切换，而要求销毁桥设备并创建
具有不同组播嗅探值的新桥设备。
