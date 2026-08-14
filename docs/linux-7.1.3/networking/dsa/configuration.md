
## 从用户空间配置 DSA 交换机


目前，DSA 交换机配置尚未集成到主流的用户空间网络配置套件中，必须手动进行。


### 配置示例


要配置一个 DSA 交换机，需要执行若干命令。本文档将一些常见配置场景作为示例进行讲解：

**单端口（single port）**
  每个交换机端口都作为一个可配置的独立以太网端口

**桥接（bridge）**
  每个交换机端口都是一个可配置以太网桥的一部分

**网关（gateway）**
  除一个上游端口外的每个交换机端口都是可配置以太网桥的一部分。
  上游端口作为一个可配置的独立以太网端口。

所有配置都使用来自 iproute2 的工具完成，iproute2 可在
https://www.kernel.org/pub/linux/utils/net/iproute2/ 获取。

通过 DSA，交换机的每个端口都像普通的 Linux 以太网接口一样被处理。CPU 端口是连接到以太网 MAC
芯片的交换机端口。相应的 Linux 以太网接口称为 conduit 接口（导管接口）。所有其他相应的 Linux
接口称为用户接口。

用户接口依赖于 conduit 接口处于 up 状态才能发送或接收流量。在内核 v5.12 之前，conduit 接口的
状态必须由用户显式管理。从内核 v5.12 开始，行为如下：

- 当一个 DSA 用户接口被拉起（up）时，conduit 接口会被自动拉起。
- 当 conduit 接口被关闭（down）时，所有 DSA 用户接口会被自动关闭。

本文档中使用以下以太网接口：

**eth0**
  conduit 接口

**eth1**
  另一个 conduit 接口

**lan1**
  一个用户接口

**lan2**
  另一个用户接口

**lan3**
  第三个用户接口

**wan**
  专用于上游流量的用户接口

可以进一步以类似方式配置其他以太网接口。配置的 IP 与网络如下：

**单端口**
  - lan1: 192.0.2.1/30 (192.0.2.0 - 192.0.2.3)
  - lan2: 192.0.2.5/30 (192.0.2.4 - 192.0.2.7)
  - lan3: 192.0.2.9/30 (192.0.2.8 - 192.0.2.11)

**桥接**
  - br0: 192.0.2.129/25 (192.0.2.128 - 192.0.2.255)

**网关**
  - br0: 192.0.2.129/25 (192.0.2.128 - 192.0.2.255)
  - wan: 192.0.2.1/30 (192.0.2.0 - 192.0.2.3)


### 带标记支持的配置


基于标记（tagging）的配置是大多数 DSA 交换机所期望并支持的。这些交换机能够在不使用基于 VLAN
配置的情况下，对 incoming 和 outgoing 流量进行标记。

**单端口**
  .. code-block:: sh

    # configure each interface
    ip addr add 192.0.2.1/30 dev lan1
    ip addr add 192.0.2.5/30 dev lan2
    ip addr add 192.0.2.9/30 dev lan3

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up

    # bring up the user interfaces
    ip link set lan1 up
    ip link set lan2 up
    ip link set lan3 up

**桥接**
  .. code-block:: sh

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up

    # bring up the user interfaces
    ip link set lan1 up
    ip link set lan2 up
    ip link set lan3 up

    # create bridge
    ip link add name br0 type bridge

    # add ports to bridge
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0
    ip link set dev lan3 master br0

    # configure the bridge
    ip addr add 192.0.2.129/25 dev br0

    # bring up the bridge
    ip link set dev br0 up

**网关**
  .. code-block:: sh

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up

    # bring up the user interfaces
    ip link set wan up
    ip link set lan1 up
    ip link set lan2 up

    # configure the upstream port
    ip addr add 192.0.2.1/30 dev wan

    # create bridge
    ip link add name br0 type bridge

    # add ports to bridge
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0

    # configure the bridge
    ip addr add 192.0.2.129/25 dev br0

    # bring up the bridge
    ip link set dev br0 up


### 不带标记支持的配置


少数交换机无法使用标记协议（DSA_TAG_PROTO_NONE）。这些交换机可以通过基于 VLAN 的配置进行配置。

**单端口**
  该配置只能通过 VLAN 标记和桥接设置来建立。

  .. code-block:: sh

    # tag traffic on CPU port
    ip link add link eth0 name eth0.1 type vlan id 1
    ip link add link eth0 name eth0.2 type vlan id 2
    ip link add link eth0 name eth0.3 type vlan id 3

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up
    ip link set eth0.1 up
    ip link set eth0.2 up
    ip link set eth0.3 up

    # bring up the user interfaces
    ip link set lan1 up
    ip link set lan2 up
    ip link set lan3 up

    # create bridge
    ip link add name br0 type bridge

    # activate VLAN filtering
    ip link set dev br0 type bridge vlan_filtering 1

    # add ports to bridges
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0
    ip link set dev lan3 master br0

    # tag traffic on ports
    bridge vlan add dev lan1 vid 1 pvid untagged
    bridge vlan add dev lan2 vid 2 pvid untagged
    bridge vlan add dev lan3 vid 3 pvid untagged

    # configure the VLANs
    ip addr add 192.0.2.1/30 dev eth0.1
    ip addr add 192.0.2.5/30 dev eth0.2
    ip addr add 192.0.2.9/30 dev eth0.3

    # bring up the bridge devices
    ip link set br0 up


**桥接**
  .. code-block:: sh

    # tag traffic on CPU port
    ip link add link eth0 name eth0.1 type vlan id 1

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up
    ip link set eth0.1 up

    # bring up the user interfaces
    ip link set lan1 up
    ip link set lan2 up
    ip link set lan3 up

    # create bridge
    ip link add name br0 type bridge

    # activate VLAN filtering
    ip link set dev br0 type bridge vlan_filtering 1

    # add ports to bridge
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0
    ip link set dev lan3 master br0
    ip link set eth0.1 master br0

    # tag traffic on ports
    bridge vlan add dev lan1 vid 1 pvid untagged
    bridge vlan add dev lan2 vid 1 pvid untagged
    bridge vlan add dev lan3 vid 1 pvid untagged

    # configure the bridge
    ip addr add 192.0.2.129/25 dev br0

    # bring up the bridge
    ip link set dev br0 up

**网关**
  .. code-block:: sh

    # tag traffic on CPU port
    ip link add link eth0 name eth0.1 type vlan id 1
    ip link add link eth0 name eth0.2 type vlan id 2

    # For kernels earlier than v5.12, the conduit interface needs to be
    # brought up manually before the user ports.
    ip link set eth0 up
    ip link set eth0.1 up
    ip link set eth0.2 up

    # bring up the user interfaces
    ip link set wan up
    ip link set lan1 up
    ip link set lan2 up

    # create bridge
    ip link add name br0 type bridge

    # activate VLAN filtering
    ip link set dev br0 type bridge vlan_filtering 1

    # add ports to bridges
    ip link set dev wan master br0
    ip link set eth0.1 master br0
    ip link set dev lan1 master br0
    ip link set dev lan2 master br0

    # tag traffic on ports
    bridge vlan add dev lan1 vid 1 pvid untagged
    bridge vlan add dev lan2 vid 1 pvid untagged
    bridge vlan add dev wan vid 2 pvid untagged

    # configure the VLANs
    ip addr add 192.0.2.1/30 dev eth0.2
    ip addr add 192.0.2.129/25 dev br0

    # bring up the bridge devices
    ip link set br0 up

### 转发数据库（FDB）管理


现有的 DSA 交换机没有必要的硬件支持来使桥接的软件 FDB 与硬件表保持同步，因此这两个表是分开
管理的（`bridge fdb show` 会查询两者，并且根据使用的是 `self` 还是 `master` 标志，``bridge fdb
add`` 或 `bridge fdb del`` 命令作用于其中一个或两个表里的条目）。

直到内核 v4.14，DSA 仅支持使用桥接旁路操作（这些操作不更新软件 FDB，只更新硬件 FDB）来由用户
空间管理桥接 FDB 条目，使用 `self` 标志（该标志是可选的，可以省略）。

  .. code-block:: sh

    bridge fdb add dev swp0 00:01:02:03:04:05 self static
    # or shorthand
    bridge fdb add dev swp0 00:01:02:03:04:05 static

由于一个 bug，DSA 提供的桥接旁路 FDB 实现没有区分 `static` 与 `local` FDB 条目（`static` 旨在
被转发，而 `local` 旨在被本地终结，即发往主机端口）。相反，所有带有 `self` 标志（隐式或显式）
的 FDB 条目都被 DSA 当作 `static` 处理，即使它们实际上是 `local`。

  .. code-block:: sh

    # This command:
    bridge fdb add dev swp0 00:01:02:03:04:05 static
    # behaves the same for DSA as this command:
    bridge fdb add dev swp0 00:01:02:03:04:05 local
    # or shorthand, because the 'local' flag is implicit if 'static' is not
    # specified, it also behaves the same as:
    bridge fdb add dev swp0 00:01:02:03:04:05

最后一条命令是使用桥接旁路操作向 DSA 交换机添加静态桥接 FDB 条目的不正确方式，它碰巧能够工作。
其他驱动会将同一命令添加的 FDB 条目当作 `local` 处理，因此不会转发它，这与 DSA 不同。

在内核 v4.14 到 v5.14 之间，DSA 并行支持两种向交换机添加桥接 FDB 条目的模式：上文讨论的桥接
旁路，以及一种使用 `master` 标志的新模式，该模式也会将 FDB 条目安装进软件桥。

  .. code-block:: sh

    bridge fdb add dev swp0 00:01:02:03:04:05 master static

自内核 v5.14 起，DSA 获得了与桥接软件 FDB 更强的集成，并且对桥接旁路 FDB 实现（使用 `self`
标志）的支持已被移除。这导致了以下变化：

  .. code-block:: sh

    # This is the only valid way of adding an FDB entry that is supported,
    # compatible with v4.14 kernels and later:
    bridge fdb add dev swp0 00:01:02:03:04:05 master static
    # This command is no longer buggy and the entry is properly treated as
    # 'local' instead of being forwarded:
    bridge fdb add dev swp0 00:01:02:03:04:05
    # This command no longer installs a static FDB entry to hardware:
    bridge fdb add dev swp0 00:01:02:03:04:05 static

因此，脚本编写者在处理 DSA 交换机接口上的桥接 FDB 条目时，鼓励使用 `master static` 这组标志。

### 用户端口到 CPU 端口的亲和性


通常，DSA 交换机通过单个以太网接口连接到主机，但在交换机芯片是分立（discrete）的情况下，硬件
设计可能允许多达 2 个或更多端口连接到主机，以提高终结吞吐量。

DSA 可以通过两种方式利用多个 CPU 端口。首先，可以静态地将与某个用户端口相关联的终结流量分配
给某个特定的 CPU 端口处理。这样，用户空间可以通过根据可用的 CPU 端口来分散亲和性，实现用户端口
之间静态负载均衡的自定义策略。

其次，可以在每个数据包的基础上而不是静态地将用户端口分配给 CPU 端口，从而在 CPU 端口之间执行
负载均衡。这可以通过将 DSA conduit 置于一个 LAG 接口（bonding 或 team）下来实现。DSA 监控此
操作，并在构成 LAG 从设备的、面向物理 DSA conduit 的 CPU 端口上创建该软件 LAG 的镜像。

为了利用多个 CPU 端口，交换机的固件（设备树）描述必须使用 `ethernet` 引用/phandle 标记所有
CPU 端口与其 DSA conduit 之间的链接。在启动时，只会使用一个单一的 CPU 端口和 DSA conduit ——
即固件描述中数值上第一个带有 `ethernet` 属性的端口。由用户来配置系统以使交换机使用其他 conduit。

DSA 使用 `rtnl_link_ops` 机制（带有 "dsa" `kind`）来允许更改用户端口的 DSA conduit。`IFLA_DSA_CONDUIT`
u32 netlink 属性包含处理每个用户设备的 conduit 设备的 ifindex。DSA conduit 必须是一个基于固件
节点信息的有效候选，或者一个只包含有效候选作为从设备的 LAG 接口。

使用 iproute2，可以进行以下操作：

  .. code-block:: sh

    # See the DSA conduit in current use
    ip -d link show dev swp0
        (...)
        dsa master eth0

    # Static CPU port distribution
    ip link set swp0 type dsa master eth1
    ip link set swp1 type dsa master eth0
    ip link set swp2 type dsa master eth1
    ip link set swp3 type dsa master eth0

    # CPU ports in LAG, using explicit assignment of the DSA conduit
    ip link add bond0 type bond mode balance-xor && ip link set bond0 up
    ip link set eth1 down && ip link set eth1 master bond0
    ip link set swp0 type dsa master bond0
    ip link set swp1 type dsa master bond0
    ip link set swp2 type dsa master bond0
    ip link set swp3 type dsa master bond0
    ip -d link show dev swp0
        (...)
        dsa master bond0

    # CPU ports in LAG, relying on implicit migration of the DSA conduit
    ip link add bond0 type bond mode balance-xor && ip link set bond0 up
    ip link set eth0 down && ip link set eth0 master bond0
    ip link set eth1 down && ip link set eth1 master bond0
    ip -d link show dev swp0
        (...)
        dsa master bond0

注意，在 CPU 端口位于 LAG 之下的情况下，使用 `IFLA_DSA_CONDUIT` netlink 属性并非严格需要，相反，
DSA 会对其当前 conduit（`eth0`）的 `IFLA_MASTER` 属性变更做出反应，并将所有用户端口迁移到 `eth0`
的新上层 `bond0`。类似地，当使用 `RTM_DELLINK` 销毁 `bond0` 时，DSA 会将其分配的用户端口迁移到
基于固件描述符合条件的第一个物理 DSA conduit（它实际上会回退到启动配置）。

因此，在具有超过 2 个物理 CPU 端口的设置中，可以将静态的用户到 CPU 端口分配与 DSA conduit 之间的
LAG 混合使用。不可能将用户端口静态分配给具有任何上层接口（这包括 LAG 设备——此时 conduit 必须
始终是该 LAG）的 DSA conduit。

允许在运行时更改用户端口的 DSA conduit（以及 CPU 端口）亲和性，以允许根据流量进行动态重新分配。

物理 DSA conduit 可以随时加入和离开用作 DSA conduit 的 LAG 接口；但是，除非该 LAG 接口至少有一个
物理 DSA conduit 作为从设备，否则 DSA 会拒绝将其作为 DSA conduit 的有效候选。
