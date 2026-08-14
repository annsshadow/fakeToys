
## 网络功能代表设备（Network Function Representors）

本文档描述了代表设备（representor netdevice）的语义与用法，它们用于控制 SmartNIC 上的内部交换。对于物理（多端口）交换机上密切相关的端口代表设备，请参见 Documentation/networking/switchdev.rst <switchdev>。

### Motivation（动机）

自 2010 年代中期以来，网卡开始提供比传统 SR-IOV 方法（及其简单的基于 MAC/VLAN 的交换模型）所能支持的、更为复杂的虚拟化能力。这催生了将软件定义网络（如 OpenVSwitch）卸载到这些 NIC 上、以指定每个函数的网络连接的需求。由此产生的设计被 variously 称为 SmartNIC 或 DPU。

网络功能代表设备将标准 Linux 网络栈引入虚拟交换机和 IOV 设备。正如 Linux 控制的交换机每个物理端口都有一个独立的 netdev，虚拟交换机的每个虚拟端口也是如此。当系统启动、且尚未配置任何卸载时，来自虚拟函数的所有数据包都会经由代表设备出现在 PF 的网络栈中。因此 PF 始终可以与虚拟函数自由通信。PF 可以配置代表设备之间、上行链路（uplink）或任何其他 netdev（路由、桥接、TC 分类器）之间的标准 Linux 转发。

因此，一个代表设备既是一个控制平面对象（在管理命令中代表该函数），也是一个数据平面对象（一根虚拟管道的一端）。作为一个虚拟链路端点，代表设备可以像任何其他 netdevice 一样被配置；在某些情况下（例如链路状态），被代表对象（representee）会遵循代表设备的配置，而在另一些情况下则有单独的 API 来配置被代表对象。

### Definitions（定义）

本文档使用术语 “switchdev function” 来指代对设备上的虚拟交换机拥有管理控制权的 PCIe 函数。通常这会是一个 PF，但理论上也可以将 NIC 配置为把这些管理特权授予某个 VF 或 SF（子函数，subfunction）。取决于 NIC 的设计，一个多端口 NIC 可能对整个设备只有一个 switchdev function，也可能对每个物理网络端口都有独立的虚拟交换机、从而有独立的 switchdev function。如果 NIC 支持嵌套交换，每个嵌套交换机可能都有独立的 switchdev function，这种情况下每个 switchdev function 只应为其直接管理的（子）交换机上的端口创建代表设备。

“representee” 是代表设备所代表的对象。例如在 VF 代表设备的情况下，representee 就是对应的 VF。

### What does a representor do?（代表设备做什么？）

一个代表设备有三个主要角色。

1. 它用于配置被代表对象所看到的网络连接，例如链路 up/down、MTU 等。例如，将代表设备管理性地置为 UP 应当导致被代表对象看到 link up / carrier on 事件。
2. 它为那些没有命中虚拟交换机中任何已卸载快速路径规则的数据流提供了慢速路径。在代表设备 netdevice 上发送的数据包应当被投递给被代表对象；由被代表对象发送、且未能匹配任何交换规则的数据包，应当在代表设备 netdevice 上被接收。（也就是说，存在一根连接代表设备与被代表对象的虚拟管道，概念上类似于一个 veth 对。）
   这使得软件交换机实现（如 OpenVSwitch 或 Linux 网桥）能够在被代表对象与网络其余部分之间转发数据包。
3. 它充当了一个句柄，交换规则（如 TC 过滤器）可以通过它来引用被代表对象，从而允许这些规则被卸载。

2) 和 3) 的结合意味着，无论 TC 过滤器是否被卸载，其行为（性能除外）应当一致。例如，VF 代表设备上的一个 TC 规则在软件中应用于在该代表设备 netdevice 上接收的数据包，而在硬件卸载中它会应用于由被代表 VF 发送的数据包。反过来，一个 mirred egress 重定向到 VF 代表设备，在硬件中对应于直接投递到被代表 VF。

### What functions should have a representor?（哪些函数应当拥有代表设备？）

本质上，对于设备内部交换机上的每个虚拟端口，都应当有一个代表设备。一些厂商选择省略上行链路和物理网络端口的代表设备，这可以简化使用（上行链路 netdev 实际上成为物理端口的代表设备），但无法推广到具有多个端口或上行链路的设备。

因此，以下各项都应当拥有代表设备：

 - 属于 switchdev function 的 VF。
 - 本地 PCIe 控制器上的其他 PF，以及属于它们的任何 VF。
 - 设备上外部 PCIe 控制器上的 PF 和 VF（例如 SmartNIC 内任何嵌入式片上系统）。
 - 具有其他 “身份”（personality）、包括网络块设备（如由远程/分布式存储支持的 vDPA virtio-blk PF）的 PF 和 VF，当且仅当它们的网络访问是通过一个虚拟交换机端口实现时。[#]_
   注意，即便被代表对象没有 netdev，这类函数也可能需要一个代表设备。
 - 属于上述任何 PF 或 VF 的子函数（SF），前提是它们在交换机上有自己的端口（而不是使用其父 PF 的端口）。
 - 设备上任何通过虚拟交换机端口接入网络的加速器或插件，即便它们没有对应的 PCIe PF 或 VF。

这使得 NIC 的全部交换行为都能通过代表设备的 TC 规则来控制。

将虚拟端口与 PCIe 虚拟函数或其 netdev 混为一谈是一个常见的误解。虽然在简单情况下 VF netdevice 与 VF 代表设备之间会有一一对应关系，但更先进的设备配置可能并非如此。一个不通过内部交换机获得网络访问（哪怕是间接地通过其函数所提供的任何服务的硬件实现）的 PCIe 函数，应当 **不** 拥有代表设备（即便它有一个 netdev）。这样的函数没有可供代表设备配置、或作为虚拟管道另一端的交换机虚拟端口。代表设备代表的是虚拟端口，而不是 PCIe 函数，也不是 “终端用户” netdevice。

   在块 DMA 请求与网络数据包之间的翻译，使得只有网络数据包通过虚拟端口抵达交换机。IP 栈 “看到” 的网络访问随后就可通过 tc 规则来配置；例如它的流量可能全部被封装进某个特定的 VLAN 或 VxLAN。然而，作为块设备（而非网络实体）所需的任何配置，都不适合代表设备，因此会使用其他通道（如 devlink）。
   与之对比的是这样一种 virtio-blk 实现：它将 DMA 请求原样转发给另一个 PF，后者的驱动随后在软件中发起并终结 IP 流量；在这种情况下，DMA 流量 **不会** 经过虚拟交换机，因此该 virtio-blk PF 应当 **不** 拥有代表设备。

### How are representors created?（代表设备如何创建？）

附加到 switchdev function 的驱动实例，应当为交换机上的每个虚拟端口创建一个纯软件的 netdevice，该 netdevice 以某种形式在内核中引用 switchdev function 自身的 netdevice 或驱动私有数据（`netdev_priv()`）。这可以通过在探测（probe）时枚举端口、在运行时动态响应端口的创建与销毁，或两者结合来实现。

代表设备 netdevice 的操作通常会涉及通过 switchdev function 来执行。例如，`ndo_start_xmit()` 可能会通过附加到 switchdev function 的硬件 TX 队列来发送数据包，并通过数据包元数据或队列配置将其标记为投递给被代表对象。

### How are representors identified?（代表设备如何被识别？）

代表设备 netdevice 应当 **不** 直接引用某个 PCIe 设备（例如通过 `net_dev->dev.parent` / `SET_NETDEV_DEV()`），无论是被代表对象的还是 switchdev function 的。相反，驱动应当在注册 netdevice 之前使用 `SET_NETDEV_DEVLINK_PORT` 宏为该 netdevice 指派一个 devlink 端口实例；内核使用该 devlink 端口来提供 `phys_switch_id` 和 `phys_port_name` sysfs 节点。（一些遗留驱动直接实现 `ndo_get_port_parent_id()` 和 `ndo_get_phys_port_name()`，但这种方式已被弃用。）关于该 API 的细节，请参见 Documentation/networking/devlink/devlink-port.rst <devlink_port>。

用户态应当会利用这些信息（例如通过 udev 规则）为 netdevice 构造一个恰当且具信息性的名称或别名。例如，如果 switchdev function 是 `eth4`，那么一个 `phys_port_name` 为 `p0pf1vf2` 的代表设备可能会被重命名为 `eth4pf1vf2rep`。

对于不对应 PCIe 函数的代表设备（例如加速器和插件），目前尚无既定的命名约定。

### How do representors interact with TC rules?（代表设备如何与 TC 规则交互？）

代表设备上的任何 TC 规则（在软件 TC 中）都应用于该代表设备 netdevice 所接收的数据包。因此，如果规则的投递部分对应于虚拟交换机上的另一个端口，驱动可以选择将其卸载到硬件，将其应用于由被代表对象发送的数据包。

类似地，由于一个以代表设备为目标的 TC mirred egress 动作（在软件中）会经由代表设备发送数据包（从而间接地投递给被代表对象），硬件卸载应当将其解读为投递给被代表对象。

作为一个简单示例，如果 `PORT_DEV` 是物理端口代表设备，而
```

    tc filter add dev $REP_DEV parent ffff: protocol ipv4 flower \
        action mirred egress redirect dev $PORT_DEV
    tc filter add dev $PORT_DEV parent ffff: protocol ipv4 flower skip_sw \
        action mirred egress mirror dev $REP_DEV

```
意味着来自 VF 的所有 IPv4 数据包都被从物理端口发出，而在物理端口上接收到的所有 IPv4 数据包都被投递给 VF（此外还有 `PORT_DEV`）。（注意，若第二条规则没有 `skip_sw`，VF 会收到两份拷贝，因为 `PORT_DEV` 上的数据包接收会再次触发该 TC 规则，并将数据包镜像到 `REP_DEV`。）

在没有独立端口和上行链路代表设备的设备上，`PORT_DEV` 会是 switchdev function 自身的上行链路 netdevice。

当然，规则可以（如果 NIC 支持）包含修改数据包的动作（例如 VLAN push/pop），这些应由虚拟交换机执行。

隧道封装与解封装要复杂得多，因为它们涉及第三个 netdevice（一个以 metadata 模式运行的隧道 netdev，例如用 `ip link add vxlan0 type vxlan external` 创建的 VxLAN 设备），并且要求将一个 IP 地址绑定到底层（underlay）设备（例如 switchdev
```

    tc filter add dev $REP_DEV parent ffff: flower \
        action tunnel_key set id $VNI src_ip $LOCAL_IP dst_ip $REMOTE_IP \
                              dst_port 4789 \
        action mirred egress redirect dev vxlan0
    tc filter add dev vxlan0 parent ffff: flower enc_src_ip $REMOTE_IP \
        enc_dst_ip $LOCAL_IP enc_key_id $VNI enc_dst_port 4789 \
        action tunnel_key unset action mirred egress redirect dev $REP_DEV

```
其中 `LOCAL_IP` 是绑定到 `PORT_DEV` 的一个 IP 地址，`REMOTE_IP` 是同一子网上的另一个 IP 地址；这意味着由 VF 发送的数据包应当被 VxLAN 封装并从物理端口发出（驱动必须通过 `LOCAL_IP` 到 `PORT_DEV` 的路由查找来推断这一点，并且还要执行 ARP/邻居表查找以找到外层以太网帧要使用的 MAC 地址），而在物理端口上接收到的、UDP 端口为 4789 的 UDP 数据包应当被解析为 VxLAN，并且如果其 VSID 匹配 `$VNI`，则被解封装并转发给 VF。

如果这一切看起来很复杂，只需记住 TC 卸载的 “黄金法则”：硬件应当确保与数据包经由慢速路径、遍历软件 TC（除了忽略任何 `skip_hw` 规则、并应用任何 `skip_sw` 规则）并通过代表设备 netdevice 发送或接收时所得到的相同最终结果。

### Configuring the representee's MAC（配置被代表对象的 MAC）

被代表对象的链路状态通过代表设备来控制。将代表设备管理性地置为 UP 或 DOWN 应当导致被代表对象的 carrier ON 或 OFF。

在代表设备上设置 MTU 应当导致向被代表对象报告相同的 MTU。（在允许配置独立且不同的 MTU 与 MRU 值的硬件上，代表设备的 MTU 应对应于被代表对象的 MRU，反之亦然。）

目前还没有办法使用代表设备来设置被代表对象的站点永久（station permanent）MAC 地址；可用于做到这一点的其他方法包括：

 - 传统 SR-IOV（`ip link set DEVICE vf NUM mac LLADDR`）
 - devlink 端口函数（参见 **devlink-port(8)** 以及 Documentation/networking/devlink/devlink-port.rst <devlink_port>）
