## 架构（Architecture）


本文档描述了 **分布式交换架构（Distributed Switch Architecture，DSA）** 子系统
的设计原则、局限性、与其他子系统的交互，以及如何为该子系统开发驱动，并
为有兴趣参与该工作的开发者提供一个 TODO（待办清单）。

## 设计原则（Design principles）


分布式交换架构（DSA）子系统最初主要用于在 Linux 下支持 Marvell 以太网交换机
（MV88E6xxx，即 Link Street 产品线），但后来已演进为也支持其他厂商。

这一设计背后的初衷是能够使用未修改的 Linux 工具（如 bridge、iproute2、
ifconfig）透明地工作，无论它们配置/查询的是交换机端口网络还是在普通的网络
设备。

一个以太网交换机通常由多个前面板端口以及一个或多个 CPU 或管理端口组成。DSA
子系统目前依赖于存在一个连接到以太网控制器的管理端口，该控制器能够接收来自
交换机的以太网帧。对于所有出现在小型家庭及办公产品（路由器、网关，甚至是
机柜顶部交换机）中的各类以太网交换机，这都是一种非常常见的配置。在 DSA 的
术语和代码中，这个主机以太网控制器之后被称为 "conduit"（导管）和 "cpu"。

DSA 中的 D 代表分布式（Distributed），因为该子系统在设计时就具备了在多个
交换机彼此级联的情况下进行配置和管理的能力，交换机之间通过上游和下游的以太网
链路互连。这些特定的端口在 DSA 术语和代码中被称为 "dsa" 端口。多个交换机
互相连接而成的集合被称为 "switch tree"（交换树）。

对于每个前面板端口，DSA 会创建专门的网络，将其用作 Linux 网络协议栈的
控制和数据流端点。这些专门的网络接口在 DSA 术语和代码中被称为 "user"
（用户）网络接口。

使用 DSA 的理想情况是：以太网交换机支持 "switch tag"（交换机标签），这是一种
硬件特性，使交换机为它接收/发送的每个以太网帧插入一个特定的标签，以帮助管理
接口判断：

- 这个帧来自哪个端口
- 这个帧被转发的原因是什么
- 如何将源自 CPU 的流量发送到特定端口

该子系统也支持那些无法插入/剥离标签的交换机，但在这种情况下功能可能会受到
轻微限制（流量隔离依赖于基于端口的 VLAN ID）。

注意，DSA 目前不为 "cpu" 和 "dsa" 端口创建网络接口，原因是：

- "cpu" 端口是管理控制器面向交换机的一侧，因此会造成功能重复，因为你会为
  同一个 conduit 得到两个接口：conduit 网络（netdev）和 "cpu" 网络（netdev）

- "dsa" 端口只是两个或多个交换机之间的导管，因此也不能真正用作合适的网络
  接口，只有下游接口或最顶层的上游接口在该模型下才有意义

注意：在过去 15 年里，DSA 子系统一直使用术语 "master"（而非 "conduit"）和
"slave"（而非 "user"）。这些术语已经从 DSA 代码库中移除，并已逐步退出 uAPI。

### 交换机标签协议（Switch tagging protocols）


DSA 支持许多厂商特定的标签协议、一个软件定义的标签协议，以及一种无标签模式
（`DSA_TAG_PROTO_NONE`）。

标签协议的具体格式是厂商特定的，但一般来说，它们都包含某些内容，用于：

- 标识以太网帧来自/应发送到哪个端口
- 提供该帧被转发到管理接口的原因

所有标签协议都位于 `net/dsa/tag_*.c` 文件中，并实现 `struct dsa_device_ops`
结构体的方法，这些方法将在下面详述。

标签协议通常可分为以下三类之一：

1. 交换机特定的帧头位于以太网头部之前，将 MAC DA、MAC SA、EtherType 以及
   整个 L2 载荷向右（从 DSA conduit 的帧解析器视角）移位。
2. 交换机特定的帧头位于 EtherType 之前，从 DSA conduit 的视角保持 MAC DA 和
   MAC SA 不动，但将“真正的”EtherType 和 L2 载荷向右移位。
3. 交换机特定的帧头位于报文的尾部，保持所有帧头不动，不改变 DSA conduit 帧
   解析器所看到的报文视图。

标签协议可能用相同长度的交换机标签标记所有报文，或者标签长度可能变化（例如
带有 PTP 时间戳的报文可能需要扩展的交换机标签，或者 TX 上使用一种标签长度而
RX 上使用另一种）。无论哪种情况，标签协议驱动都必须用交换机帧头/帧尾最长的
八位组（octet）长度来填充 `struct dsa_device_ops` 的 `needed_headroom`
和/或 `needed_tailroom`。DSA 框架会自动调整 conduit 接口的 MTU 以容纳这一额外
大小，从而使 DSA user 端口支持 1500 八位组的标准 MTU（L2 载荷长度）。
`needed_headroom` 和 `needed_tailroom` 属性还用于以尽力而为的方式，向网络协议栈
请求分配具有足够额外空间的报文，使得在发送报文时压入交换机标签不会因内存不足
而导致其重新分配。

尽管不期望应用程序解析 DSA 特定的帧头，但标签协议在线上的格式代表了内核向
用户空间暴露的应用二进制接口（ABI），供诸如 `libpcap` 之类的解码器使用。标签
协议驱动必须用能够唯一描述交换机硬件与数据路径驱动之间所需交互特征的值来填充
`struct dsa_device_ops` 的 `proto` 成员：帧头中每个位字段的偏移量，以及处理
这些帧所需的任何有状态处理（如 PTP 时间戳所需）。

从网络协议栈的视角来看，同一 DSA 交换树内的所有交换机都使用相同的标签协议。
在报文穿过包含多个交换机的 fabric 的情况下，交换机特定的帧头由报文所接收的
fabric 中的第一个交换机插入。该头部通常包含关于其类型的信息（它是必须陷入
（trap）到 CPU 的控制帧，还是要转发的数据帧）。控制帧应仅由软件数据路径解封装，
而数据帧也可能自主地转发到同一 fabric 中其他交换机的其他 user 端口，在这种
情况下，最外层的交换机端口必须对该报文进行解封装。

注意，在某些情况下，叶子交换机（未直接连接到 CPU）所使用的标签格式可能与
网络协议栈所看到的不同。这在 Marvell 交换树中可以看到，其中 CPU 端口可以配置
为使用 DSA 或 Ethertype DSA（EDSA）格式，但 DSA 链路被配置为使用较短的（不带
Ethertype）DSA 帧头，以减少自主报文转发的开销。如果 DSA 交换树配置为 EDSA 标签
协议，操作系统仍然会看到来自叶子交换机的、用较短 DSA 帧头标记的 EDSA 标记报文，
这一点依然成立。这是因为直接连接到 CPU 的 Marvell 交换机被配置为在 DSA 和 EDSA
之间执行标签转换（这只是添加或移除 `ETH_P_EDSA` EtherType 以及一些填充八位组
的操作）。

即使 DSA 交换机的标签协议彼此不兼容，也可以构建级联的 DSA 交换机配置。在这种
情况下，该 fabric 中没有 DSA 链路，每个交换机构成一个互不相交的 DSA 交换树。
DSA 链路被视为仅仅是一对 DSA conduit（上游 DSA 交换机的对外端口）和 CPU 端口
（下游 DSA 交换机的对内端口）。

所连接的 DSA 交换树的标签协议可以通过
```

    cat /sys/class/net/eth0/dsa/tagging

```
如果硬件和驱动支持，DSA 交换树的标签协议可以在运行时更改。这是通过将新的
标签协议名称写入与上面相同的 sysfs 设备属性来完成的（执行此操作时，DSA
conduit 和所有附着的交换机端口都必须处于 down 状态）。

理想情况下，所有标签协议都应可使用 `dsa_loop` 模拟驱动进行测试，该驱动可以
附着到任何网络接口。其目标是：任何网络接口都应能够以相同的方式传输相同的
报文，并且无论交换机控制路径使用的驱动是什么、DSA conduit 使用的驱动是什么，
tagger 都应以相同的方式解码收到的相同报文。

报文的发送要经过 tagger 的 `xmit` 函数。传入的 `struct sk_buff *skb` 的
`skb->data` 指向 `skb_mac_header(skb)`，即目的 MAC 地址；而传入的
`struct net_device *dev` 代表虚拟的 DSA user 网络接口，报文必须被导向其
对应的硬件（即 `swp0`）。该方法的工作是以一种让交换机能够理解该报文要发往
哪个出口端口（而不是将其投递到其他端口）的方式来准备 skb。这通常是通过压入一个
帧头来完成的。只要 `needed_headroom` 和 `needed_tailroom` 属性被正确填写，
就无需检查 skb 头空间（headroom）或尾空间（tailroom）是否不足，因为 DSA 在
调用该方法之前会确保有足够的空间。

报文的接收要经过 tagger 的 `rcv` 函数。传入的 `struct sk_buff *skb` 的
`skb->data` 指向 `skb_mac_header(skb) + ETH_ALEN` 个八位组，即如果该帧未被
标记，EtherType 之后的第一个八位组所在的位置。该方法的作用是消费帧头，调整
`skb->data` 使其真正指向 EtherType 之后的第一个八位组，并将 `skb->dev` 改为
指向该报文所接收的物理前面板交换机端口对应的虚拟 DSA user 网络接口。

由于第 1 类和第 2 类标签协议会破坏 DSA conduit 上的软件（通常也包括硬件）报文
解析，因此 DSA conduit 上的 RPS（接收报文转向，Receive Packet Steering）等
功能会失效。DSA 框架通过挂钩到流解析器（flow dissector）并移位 DSA conduit
所看到的标记帧中 IP 头所在的偏移量来处理这个问题。该行为基于标签协议的
`overhead` 值自动进行。如果并非所有报文大小都相等，tagger 可以实现
`struct dsa_device_ops` 的 `flow_dissect` 方法来覆盖这一默认行为，通过指定
每个单独 RX 报文所产生的正确偏移量。尾部（tail）tagger 不会对流解析器造成
问题。

当 DSA conduit 驱动在 vlan_features 中声明 NETIF_F_HW_CSUM 并查看 csum_start
和 csum_offset 时，校验和卸载应能与第 1 类和第 2 类 tagger 一起工作。对于这类
情况，DSA 会将校验和的起始和偏移按标签大小进行移位。如果 DSA conduit 驱动仍
在 vlan_features 中使用传统的 NETIF_F_IP_CSUM 或 NETIF_F_IPV6_CSUM，则只有当
卸载硬件已经预期该特定标签（可能是由于厂商匹配）时，卸载才可能生效。DSA user
端口从 conduit 继承这些标志，由驱动负责在 IP 头不在硬件预期位置时正确回退到
软件校验和。如果该检查无效，报文可能会在没有正确校验和的情况下进入网络（校验和
字段将带有伪 IP 头部校验和）。对于第 3 类，当卸载硬件尚未预期所使用的交换机
标签时，必须在插入任何标签之前（即在 tagger 内部）计算校验和。否则，DSA conduit
会将尾部标签包含在（软件或硬件）校验和计算中。然后，当标签在发送过程中被交换机
剥离时，会留下一个不正确的 IP 校验和。

由于各种原因（最常见的是第 1 类 tagger 与未感知 DSA 的 conduit 相关联，从而
篡改了 conduit 所认为的 MAC DA），标签协议可能要求 DSA conduit 以混杂（promiscuous）
模式运行，以接收所有帧而不管 MAC DA 的值。这可以通过设置 `struct dsa_device_ops`
的 `promisc_on_conduit` 属性来完成。注意，这假设的是一个未感知 DSA 的 conduit
驱动，这是常态。

### Conduit 网络设备


Conduit 网络设备是用于 CPU/管理以太网接口的常规、未修改的 Linux 网络设备驱动。
这样的驱动可能偶尔需要知道 DSA 是否已启用（例如：启用/禁用特定的卸载功能），
但 DSA 子系统已被证明可以与工业标准驱动（如 `e1000e,`、`mv643xx_eth` 等）一起
工作，而无需对这些驱动进行任何修改。此类网络设备也常被称为 conduit 网络设备，
因为它们充当主机处理器与硬件以太网交换机之间的管道。

### 网络协议栈挂钩（Networking stack hooks）


当 conduit 网络（netdev）与 DSA 一起使用时，会在网络协议栈中放置一个小挂钩，
以便 DSA 子系统处理以太网交换机特定的标签协议。DSA 通过向网络协议栈注册一个
特定的（且伪造的）以太网类型（之后成为 `skb->protocol`）来实现这一点，这也
被称为 `ptype` 或 `packet_type`。一个典型的以太网帧接收序列如下所示：

Conduit 网络设备（例如：e1000e）：

1. 接收中断触发：

        - 调用接收函数
        - 完成基本的报文处理：获取长度、状态等
        - 通过调用 `eth_type_trans` 将报文准备好交由以太网层处理
```

          eth_type_trans(skb, dev)
                  if (dev->dsa_ptr != NULL)
                          -> skb->protocol = ETH_P_XDSA

```
```

          netif_receive_skb(skb)
                  -> iterate over registered packet_type
                          -> invoke handler for ETH_P_XDSA, calls dsa_switch_rcv()

```
```

          -> dsa_switch_rcv()
                  -> invoke switch tag specific protocol handler in 'net/dsa/tag_*.c'

```
5. net/dsa/tag_*.c:

        - 检查并剥离交换机标签协议以确定来源端口
        - 定位每端口网络设备
        - 使用 DSA user 网络设备调用 `eth_type_trans()`
        - 调用 `netif_receive_skb()`

在此之后，DSA user 网络设备会收到可由网络协议栈处理的常规以太网帧。

### User 网络设备


DSA 创建的 user 网络设备堆叠在它们的 conduit 网络设备之上，这些网络接口中的每一个
都将负责充当交换机每个前面板端口的控制和数据流端点。这些接口经过专门化，以便：

- 在向特定交换机端口发送/从特定交换机端口接收流量时，插入/移除交换机标签协议
  （如果存在）
- 查询交换机以进行 ethtool 操作：统计信息、链路状态、Wake-on-LAN、寄存器转储...
- 管理外部/内部 PHY：链路、自协商等

这些 user 网络设备具有自定义的 net_device_ops 和 ethtool_ops 函数指针，使 DSA
能够在网络协议栈/ethtool 与交换机驱动实现之间引入一层分层。

当从这些 user 网络设备发送帧时，DSA 会查找当前注册到这些网络设备的交换机标签
协议，并调用一个特定的发送例程，该例程负责在以太网帧中添加相关的交换机标签。

随后这些帧使用 conduit 网络设备的 `ndo_start_xmit()` 函数排队等待发送。由于它们
包含适当的交换机标签，以太网交换机将能够处理来自管理接口的这些传入帧，并将其
投递到物理交换机端口。

当使用多个 CPU 端口时，可以在 DSA user 设备与物理 DSA conduit 之间堆叠一个 LAG
（bonding/team）设备。因此该 LAG 设备也是一个 DSA conduit，但 LAG 从设备也继续
作为 DSA conduit（只是没有分配 user 端口；这用于在 LAG DSA conduit 消失时进行
恢复）。因此，LAG DSA conduit 的数据路径被非对称地使用。在 RX 上，调用
`dsa_switch_rcv()` 的 `ETH_P_XDSA` 处理程序被提前调用（在物理 DSA conduit 上；
即 LAG 从设备）。因此，LAG DSA conduit 的 RX 数据路径不会被使用。另一方面，TX
线性进行：`dsa_user_xmit` 调用 `dsa_enqueue_skb`，后者向 LAG DSA conduit 调用
`dev_queue_xmit`。后者再向其中一个物理 DSA conduit 调用 `dev_queue_xmit`，在两种
情况下，报文都通过一条硬件路径退出系统、前往交换机。

### 图形表示（Graphical representation）


概括来说，从网络设备视角看，DSA 大致如下所示
```

                Unaware application
              opens and binds socket
                       |  ^
                       |  |
           +-----------v--|--------------------+
           |+------+ +------+ +------+ +------+|
           || swp0 | | swp1 | | swp2 | | swp3 ||
           |+------+-+------+-+------+-+------+|
           |          DSA switch driver        |
           +-----------------------------------+
                         |        ^
            Tag added by |        | Tag consumed by
           switch driver |        | switch driver
                         v        |
           +-----------------------------------+
           | Unmodified host interface driver  | Software
   --------+-----------------------------------+------------
           |       Host interface (eth0)       | Hardware
           +-----------------------------------+
                         |        ^
         Tag consumed by |        | Tag added by
         switch hardware |        | switch hardware
                         v        |
           +-----------------------------------+
           |               Switch              |
           |+------+ +------+ +------+ +------+|
           || swp0 | | swp1 | | swp2 | | swp3 ||
           ++------+-+------+-+------+-+------++

```
### User MDIO 总线（User MDIO bus）


为了能够读取内置在交换机中的交换机 PHY，DSA 创建了一个 user MDIO 总线，允许
特定的交换机驱动转移并拦截针对特定 PHY 地址的 MDIO 读/写。在大多数通过 MDIO
连接的交换机中，这些函数会利用直接或间接的 PHY 寻址模式，从交换机内置的 PHY
返回标准的 MII 寄存器，从而允许 PHY 库和/或返回链路状态、链路对端页、自协商
结果等。

对于同时具有外部和内部 MDIO 总线的以太网交换机，user MII 总线可用于对连接到
该交换机的内部或外部 MDIO 设备（内部 PHY、外部 PHY，甚至外部交换机）的 MDIO
读/写进行复用/解复用。

### 数据结构（Data structures）


DSA 数据结构定义在 `include/net/dsa.h` 以及 `net/dsa/dsa_priv.h` 中：

- `dsa_chip_data`：给定交换机设备的平台数据配置，该结构体描述交换机设备的
  父设备、其地址，以及其端口的各种属性：名称/标签，最后是路由表指示（在
  级联交换机时）

- `dsa_switch_tree`：在 `dsa_ptr` 下分配给 conduit 网络设备的结构体，该结构体
  引用一个 dsa_platform_data 结构体，以及交换树所支持的标签协议、应调用哪个
  接收/发送函数挂钩，还提供有关直接附着的交换机的信息：CPU 端口。最后，引用
  一组 dsa_switch 以寻址树中的各个交换机。

- `dsa_switch`：描述树中交换机设备的结构体，引用一个 `dsa_switch_tree` 作为
  反向指针，引用 user 网络设备、conduit 网络设备，并引用后台的 `dsa_switch_ops`

- `dsa_switch_ops`：引用函数指针的结构体，完整说明见下文。

## 设计局限性（Design limitations）


### 缺少 CPU/DSA 网络设备


如前所述，DSA 目前不为 CPU 或 DSA 端口创建 user 网络设备。在以下情况下这可能会
成为问题：

- 无法使用 ethtool 获取交换机 CPU 端口统计计数器，这可能使调试通过 xMII
  接口连接的 MDIO 交换机更加困难

- 无法基于附着到它的以太网控制器能力来配置 CPU 端口链路参数：
  http://patchwork.ozlabs.org/patch/509806/

- 在使用级联配置时，无法在交换机之间配置特定的 VLAN ID / 中继 VLAN

### 使用 DSA 配置时的常见陷阱（Common pitfalls）


一旦 conduit 网络设备被配置为使用 DSA（`dev->dsa_ptr` 变为非 NULL），并且其背后的
交换机期望一个标签协议，该网络接口就只能专门用作 conduit 接口。直接通过该接口
发送报文（例如：使用该接口打开一个 socket）将不会使我们经过交换机标签协议的
发送函数，因此另一端的以太网交换机由于期望一个标签，通常会丢弃该帧。

## 与其他子系统的交互（Interactions with other subsystems）


DSA 目前利用了以下子系统：

- MDIO/PHY 库：`drivers/net/phy/phy.c`、`mdio_bus.c`
- Switchdev：`net/switchdev/*`
- 用于各种 of_* 函数的 Device Tree
- Devlink：`net/core/devlink.c`

### MDIO/PHY 库


DSA 暴露的 user 网络设备可能会也可能不会与 PHY 设备（`include/linux/phy.h` 中
定义的 `struct phy_device`）接口，但 DSA 子系统处理所有可能的组合：

- 内置在以太网交换机硬件中的内部 PHY 设备
- 通过内部或外部 MDIO 总线连接的外部 PHY 设备
- 通过内部 MDIO 总线连接的内部 PHY 设备
- 特殊的、非自协商或非 MDIO 管理的 PHY 设备：SFP、MoCA；即所谓固定 PHY

PHY 配置由 `dsa_user_phy_setup()` 函数完成，其逻辑大致如下：

- 如果使用 Device Tree，则使用标准的 "phy-handle" 属性查找 PHY 设备，如果找到，
  则使用 `of_phy_connect()` 创建并注册该 PHY 设备

- 如果使用 Device Tree 且 PHY 设备是 "fixed"（即符合
  `Documentation/devicetree/bindings/net/fixed-link.txt` 中定义的、非 MDIO 管理
  的 PHY 的定义），则使用特殊的固定 MDIO 总线驱动透明地注册并连接该 PHY

- 最后，如果 PHY 内置在交换机中（对于独立交换机封装非常常见），则使用 DSA 创建的
  user MII 总线探测该 PHY


### SWITCHDEV


DSA 在与桥接（bridge）层交互时直接使用 SWITCHDEV，更具体地说，是在基于每端口
user 网络设备配置 VLAN 时使用其 VLAN 过滤部分。截至目前，DSA 支持的 SWITCHDEV
对象只有 FDB 和 VLAN 对象。

### Devlink


DSA 为 fabric 中的每个物理交换机注册一个 devlink 设备。对于每个 devlink 设备，
每个物理端口（即 user 端口、CPU 端口、DSA 链路或未使用的端口）都作为 devlink
端口暴露。

DSA 驱动可以利用以下 devlink 特性：

- Regions（区域）：一种调试特性，允许用户空间以底层二进制格式转储驱动定义的
  硬件信息区域。既支持全局区域，也支持每端口区域。即使某些数据已经以某种方式
  暴露给标准的 iproute2 用户空间程序（ip-link、bridge），如地址表和 VLAN 表，
  也可以导出 devlink 区域。例如，如果表中包含 iproute2 抽象层看不到的额外的硬件
  特定细节，或者需要在非 user 端口上也检查这些表（由于未为它们注册网络接口，
  iproute2 看不到它们），这可能很有用。
- Params（参数）：一种允许用户配置与设备相关的某些底层可调旋钮的特性。驱动可以
  实现适用的通用 devlink 参数，也可以添加新的设备特定 devlink 参数。
- Resources（资源）：一种监控特性，使用户能够查看设备中某些硬件表（如 FDB、VLAN
  等）的利用程度。
- Shared buffers（共享缓冲区）：一种 QoS 特性，用于在每个端口和每个流量类上、在
  入向和出向方向上调整和划分内存及帧预留，使得低优先级的大流量不会阻碍高优先级
  关键流量的处理。

更多细节，请参阅 `Documentation/networking/devlink/`。

### Device Tree


DSA 具有一个标准化的绑定（binding），记录在
`Documentation/devicetree/bindings/net/dsa/dsa.txt` 中。PHY/MDIO 库辅助函数，如
`of_get_phy_mode()`、`of_phy_connect()`，也用于查询每端口 PHY 的特定细节：接口
连接、MDIO 总线位置等。

## 驱动开发（Driver development）


DSA 交换机驱动需要实现 `dsa_switch_ops` 结构体，其中包含下文描述的各个成员。

### 探测、注册与设备生命周期（Probing, registration and device lifetime）


DSA 交换机是总线上常规的 `device` 结构体（无论是 platform、SPI、I2C、MDIO 还是
其他）。DSA 框架不参与设备核心对它们的探测。

从驱动的角度看，交换机注册意味着将一个有效的 `struct dsa_switch` 指针传递给
`dsa_register_switch()`，通常从交换机驱动的探测函数中进行。所提供的结构体中
以下成员必须有效：

- `ds->dev`：将用于解析交换机的 OF 节点或平台数据。

- `ds->num_ports`：将用于创建该交换机的端口列表，并验证 OF 节点中提供的端口
  索引。

- `ds->ops`：指向持有 DSA 方法实现的 `dsa_switch_ops` 结构体的指针。

- `ds->priv`：指向驱动私有数据结构的反向指针，可在所有后续的 DSA 方法回调中
  检索。

此外，`dsa_switch` 结构体中的以下标志可以选择性地进行配置，以从 DSA 核心获取
驱动特定的行为。设置时它们的行为通过 `include/net/dsa.h` 中的注释进行了说明。

- `ds->vlan_filtering_is_global`

- `ds->needs_standalone_vlan_filtering`

- `ds->configure_vlan_while_not_filtering`

- `ds->untag_bridge_pvid`

- `ds->assisted_learning_on_cpu_port`

- `ds->mtu_enforcement_ingress`

- `ds->fdb_isolation`

在内部，DSA 维护一个对内核全局的交换树（一组交换机）数组，并在注册时将
`dsa_switch` 结构体附加到一个树。交换机所附着的树 ID 由交换机 OF 节点的
`dsa,member` 属性的第一个 u32 数字决定（缺失则为 0）。树中的交换机 ID 由同一
OF 属性的第二个 u32 数字决定（缺失则为 0）。注册具有相同交换机 ID 和树 ID 的
多个交换机会导致非法并引发错误。使用平台数据时，允许一个交换机和一个交换树。

对于包含多个交换机的树，探测是非对称进行的。前 N-1 个 `dsa_register_switch()`
调用者只将其端口添加到树的端口列表（`dst->ports`）中，每个端口都有一个指向其
关联交换机的反向指针（`dp->ds`）。然后，这些交换机提前退出其 `dsa_register_switch()`
调用，因为 `dsa_tree_setup_routing_table()` 已判定该树尚未完整（树端口列表中尚
不存在 DSA 链路引用的所有端口）。当最后一个交换机调用 `dsa_register_switch()`
时，该树变为完整，这会触发对该树中所有交换机初始化工作（包括调用
`ds->ops->setup()`）的实际继续，所有这些都作为最后一个交换机探测函数调用上下文
的一部分。

调用 `dsa_unregister_switch()` 会执行注册的反向操作，它将一个交换机的端口从树的
端口列表中移除。当第一个交换机注销时，整个树被拆除。

DSA 交换机驱动必须实现其各自总线的 `shutdown()` 回调，并从中调用
`dsa_switch_shutdown()`（`dsa_unregister_switch()` 所执行完整拆除的一个最小版本）。
原因是 DSA 持有对 conduit 网络设备的引用，如果 conduit 设备的驱动决定在关闭
（shutdown）时解绑（unbind），DSA 的引用将阻止该操作完成。

`dsa_switch_shutdown()` 或 `dsa_unregister_switch()` 必须被调用，但二者不能同时
调用，并且设备驱动模型允许在 `shutdown()` 已经被调用的情况下仍调用总线的
`remove()` 方法。因此，驱动应通过在其中任何一个运行后将 drvdata 设为 NULL，并在
采取任何行动前检查 drvdata 是否为 NULL，来实现 `remove()` 与 `shutdown()` 之间的
互斥方法。

在调用 `dsa_switch_shutdown()` 或 `dsa_unregister_switch()` 之后，不得再通过所提供
的 `dsa_switch_ops` 进行任何回调，驱动可以释放与 `dsa_switch` 相关的数据结构。

### 交换机配置（Switch configuration）


- `get_tag_protocol`：用于指示支持哪种标签协议，应为 `dsa_tag_protocol` 枚举中
  的一个有效值。返回的信息不必是静态的；驱动会传入 CPU 端口号，以及可能级联的
  上游交换机的标签协议，以防在支持的标签格式方面存在硬件限制。

- `change_tag_protocol`：当默认标签协议与 conduit 存在兼容性问题或其他问题时，
  驱动可以支持在运行时更改它，无论是通过设备树属性还是通过 sysfs。在这种情况下，
  后续对 `get_tag_protocol` 的调用应报告当前使用的协议。

- `setup`：交换机的设置函数，该函数负责用所需的一切来设置 `dsa_switch_ops` 私有
  结构体：寄存器映射、中断、互斥体、锁等。该函数还应正确地将交换机配置为将所有
  网络接口彼此隔离，即它们应由交换机硬件本身隔离，通常是通过为每个端口创建一个
  基于端口的 VLAN ID，并仅允许 CPU 端口和该特定端口处于转发向量中。平台未使用的
  端口应被禁用。在此函数之后，交换机应已完全配置好并准备好服务任何类型的请求。
  建议在此设置函数期间对交换机发出软件复位，以避免依赖先前软件代理（如
  bootloader/固件）可能已经配置的内容。负责撤销此处所做的任何适用分配或操作的
  方法是 `teardown`。

- `port_setup` 和 `port_teardown`：用于初始化和销毁每端口数据结构的方法。某些
  操作（如注册和注销 devlink 端口区域）必须通过这些方法完成，否则它们是可选的。
  只有当一个端口先前已被设置时，才会被拆除。一个端口有可能在探测期间被设置，随后
  立即被拆除，例如在其 PHY 无法找到的情况下。在这种情况下，DSA 交换机的探测会在
  没有该特定端口的情况下继续。

- `port_change_conduit`：一种方法，通过它可以更改 user 端口与 CPU 端口之间的
  亲和性（用于流量终结目的的关联）。默认情况下，树中的所有 user 端口都被分配给
  对它们而言第一个可用的 CPU 端口（大多数情况下这意味着树中的 user 端口都被
  分配给同一个 CPU 端口，H 型拓扑除外，如提交 2c0b03258b8b 所述）。`port` 参数
  代表 user 端口的索引，`conduit` 参数代表新的 DSA conduit `net_device`。与新的
  conduit 关联的 CPU 端口可以通过查看 ``struct dsa_port *cpu_dp =
  conduit->dsa_ptr`` 来获取。此外，conduit 也可以是一个 LAG 设备，其中所有从设备
  都是物理 DSA conduit。LAG DSA 也具有一个有效的 `conduit->dsa_ptr` 指针，但这并非
  唯一，而是第一个物理 DSA conduit（LAG 从设备）的 `dsa_ptr` 的副本。在 LAG DSA
  conduit 的情况下，将针对与物理 DSA conduit 关联的物理 CPU 端口单独发出对
  `port_lag_join` 的进一步调用，请求它们创建与 LAG 接口关联的硬件 LAG。

### PHY 设备与链路管理（PHY devices and link management）


- `get_phy_flags`：某些交换机与各种以太网 PHY 接口，如果 PHY 库中的 PHY 驱动
  需要了解它自身无法获取的信息（例如：来自交换机内存映射寄存器），该函数应返回
  一个 32 位位掩码（"flags"），该位掩码在交换机驱动与 `drivers/net/phy/*` 中的
  以太网 PHY 驱动之间是私有的。

- `phy_read`：当 DSA user MDIO 总线尝试读取交换机端口 MDIO 寄存器时调用的函数。
  如果不可用，每次读取返回 0xffff。对于内置的交换机以太网 PHY，该函数应允许读取
  链路状态、自协商结果、链路对端页等。

- `phy_write`：当 DSA user MDIO 总线尝试写入交换机端口 MDIO 寄存器时调用的函数。
  如果不可用，返回负的错误码。

- `adjust_link`：当 user 网络设备附着到 PHY 设备时由 PHY 库调用的函数。该函数负责
  根据 `phy_device` 所提供的内容，适当地配置交换机端口链路参数：速度、双工、基于
  暂停（pause）。

- `fixed_link_update`：由 PHY 库，特别是由固定 PHY 驱动调用的函数，向交换机驱动
  询问无法通过自协商获得、或通过 MDIO 读取 PHY 寄存器获得的链路参数。这对于特定
  种类的硬件（如 QSGMII、MoCA 或其他种类的非 MDIO 管理的 PHY）特别有用，这些硬件
  可获取带外链路信息。

### Ethtool 操作


- `get_strings`：用于查询驱动字符串的 ethtool 函数，通常返回统计字符串、私有标志
  字符串等。

- `get_ethtool_stats`：用于查询每端口统计信息并返回其值的 ethtool 函数。DSA 叠加
  了 user 网络设备的一般统计信息：来自网络设备的 RX/TX 计数器，以及每端口的交换机
  驱动特定统计信息。

- `get_sset_count`：用于查询统计项数量的 ethtool 函数。

- `get_wol`：用于获取每端口 Wake-on-LAN 设置的 ethtool 函数，对于某些实现，如果该
  接口需要参与 Wake-on-LAN，该函数也可能查询 conduit 网络设备的 Wake-on-LAN 设置。

- `set_wol`：用于配置每端口 Wake-on-LAN 设置的 ethtool 函数，是 set_wol 的直接
  对应物，具有类似的限制。

- `set_eee`：用于配置交换机端口 EEE（绿色以太网，Green Ethernet）设置的 ethtool
  函数，如果相关，可以可选地调用 PHY 库以在 PHY 层启用 EEE。该函数应在交换机端口
  MAC 控制器和数据处理逻辑处启用 EEE。

- `get_eee`：用于查询交换机端口 EEE 设置的 ethtool 函数，该函数应返回交换机端口
  MAC 控制器和数据处理逻辑的 EEE 状态，以及查询 PHY 当前配置的 EEE 设置。

- `get_eeprom_len`：返回给定交换机 EEPROM 长度/大小（字节）的 ethtool 函数。

- `get_eeprom`：返回给定交换机 EEPROM 内容的 ethtool 函数。

- `set_eeprom`：将指定数据写入给定交换机 EEPROM 的 ethtool 函数。

- `get_regs_len`：返回给定交换机寄存器长度的 ethtool 函数。

- `get_regs`：返回以太网交换机内部寄存器内容的 ethtool 函数。该函数可能需要 ethtool
  中的用户态代码来美化打印寄存器值和寄存器。

### 电源管理（Power management）


- `suspend`：当系统挂起时由 DSA 平台设备调用的函数，应静默所有以太网交换机活动，
  但保持参与 Wake-on-LAN 的端口以及（如果支持）额外的唤醒逻辑处于活动状态。

- `resume`：当系统恢复时由 DSA 平台设备调用的函数，应恢复所有以太网交换机活动，
  并将交换机重新配置为完全活动状态。

- `port_enable`：当端口被管理员性地启用（up）时，由 DSA user 网络设备的 ndo_open
  函数调用的函数，该函数应完全启用给定的交换机端口。如果该端口是桥接成员，DSA
  负责将其标记为 `BR_STATE_BLOCKING`，否则标记为 `BR_STATE_FORWARDING`，并将这些更改
  传播到硬件。

- `port_disable`：当端口被管理员性地关闭（down）时，由 DSA user 网络设备的 ndo_close
  函数调用的函数，该函数应完全禁用给定的交换机端口。如果该端口是桥接成员且在被
  禁用时，DSA 负责将其标记为 `BR_STATE_DISABLED` 并将更改传播到硬件（如果该端口是
  桥接成员）。

### 地址数据库（Address databases）


交换硬件预期有一个用于 FDB 条目的表，但并非所有条目都同时处于活动状态。地址数据库
（address database）是 FDB 条目中活跃（可以在 RX 上通过地址学习匹配，或在 TX 上
进行 FDB 查找）的子集，具体取决于端口的状态。地址数据库在本文档中有时被称为 "FID"
（Filtering ID，过滤 ID），尽管底层实现可以选择硬件可用的任何方式。

例如，属于 VLAN 不感知桥接（**当前** VLAN 不感知）的所有端口，预期在该桥接关联的
数据库（而非其他 VLAN 不感知桥接）中学习源地址。在转发和 FDB 查找期间，在 VLAN
不感知桥接端口上收到的报文，应能够找到具有相同 MAC DA 的 VLAN 不感知 FDB 条目，该
条目位于同一桥接的另一个端口成员上。同时，FDB 查找过程必须能够不找到具有相同 MAC
DA 的 FDB 条目，如果该条目指向属于不同 VLAN 不感知桥接的端口（因此与不同的地址
数据库关联）。

类似地，每个卸载的 VLAN 感知桥接的每个 VLAN 都应有一个关联的地址数据库，它由作为
该 VLAN 成员的所有端口共享，但不由属于同一 VID 的不同桥接的成员端口共享。

在此上下文中，VLAN 不感知数据库意味着所有报文都预期匹配它而不管 VLAN ID（仅 MAC
地址查找），而 VLAN 感知数据库意味着报文应基于分类后的 802.1Q 头部中的 VLAN ID
（或 pvid，如果未标记）进行匹配。

在桥接层，VLAN 不感知 FDB 条目具有特殊的 VID 值 0，而 VLAN 感知 FDB 条目具有非零
VID 值。注意，VLAN 不感知桥接可能具有 VLAN 感知（非零 VID）FDB 条目，而 VLAN 感知
桥接也可能具有 VLAN 不感知 FDB 条目。与硬件一样，软件桥接保持独立的地址数据库，并
通过 switchdev 将属于这些数据库的 FDB 条目异步地卸载到硬件，相对于数据库变为活动或
不活动的时间。

当 user 端口以独立（standalone）模式运行时，其驱动应将其配置为使用一个称为端口私有
数据库（port private database）的单独数据库。这与上述数据库不同，并且应尽可能少地
妨碍作为独立端口的运行（报文进、报文出到 CPU 端口）。例如，在入向上，它不应尝试
学习入向流量的 MAC SA，因为学习是桥接层的服务，而这是一个独立端口，因此会消耗无用
的空间。在没有地址学习的情况下，在朴素实现中端口私有数据库应为空，这种情况下，所有
收到的报文都应直接泛洪到 CPU 端口。

DSA（级联）和 CPU 端口也被称为 "shared"（共享）端口，因为它们服务于多个地址数据库，
而报文应关联的数据库通常嵌入在 DSA 标签中。这意味着 CPU 端口可能同时传输来自独立
端口（由硬件分类到一个地址数据库）和来自桥接端口（被分类到不同的地址数据库）的报文。

满足特定条件的交换机驱动能够通过将 CPU 端口从交换机的泛洪域中移除来优化朴素配置，
并仅为已知软件感兴趣的那些 MAC 地址，用指向 CPU 端口的 FDB 条目对硬件进行编程。
不匹配任何已知 FDB 条目的报文将不会被投递到 CPU，从而节省为创建一个 skb 仅为了丢弃
它所需的 CPU 周期。

DSA 能够对以下种类的地址执行主机地址过滤：

- 端口的主用单播 MAC 地址（`dev->dev_addr`）。这些与相应 user 端口的端口私有数据库
  关联，并通过 `port_fdb_add` 通知驱动将其安装到 CPU 端口。

- 端口的辅助单播和组播 MAC 地址（通过 `dev_uc_add()` 和 `dev_mc_add()` 添加的
  地址）。这些也与相应 user 端口的端口私有数据库关联。

- 本地/永久桥接 FDB 条目（`BR_FDB_LOCAL`）。这些是桥接端口的 MAC 地址，报文必须在
  本地终结而不是被转发。它们与该桥接的地址数据库关联。

- 安装在与某些 DSA 交换机端口处于同一桥接中的外部（非 DSA）接口上的静态桥接 FDB
  条目。这些也与该桥接的地址数据库关联。

- 与某些 DSA 交换机端口处于同一桥接中的外部接口上动态学习的 FDB 条目，仅当
  `ds->assisted_learning_on_cpu_port` 被驱动设为 true 时。这些与该桥接的地址数据库
  关联。

对于下面详述的各种操作，DSA 提供一个 `dsa_db` 结构体，它可以是以下类型：

- `DSA_DB_PORT`：要安装或删除的 FDB（或 MDB）条目属于 user 端口 `db->dp` 的端口
  私有数据库。
- `DSA_DB_BRIDGE`：该条目属于桥接 `db->bridge` 的某个地址数据库。驱动应负责区分该
  桥接的 VLAN 不感知数据库和每 VID 数据库。
- `DSA_DB_LAG`：该条目属于 LAG `db->lag` 的地址数据库。注意：`DSA_DB_LAG` 目前未被
  使用，未来可能会被移除。

在 `port_fdb_add`、`port_mdb_add` 等中处理 `dsa_db` 参数的驱动应将 `ds->fdb_isolation`
声明为 true。

DSA 将每个卸载的桥接和每个卸载的 LAG 与一个基于 1 的 ID（**（``struct dsa_bridge **
: num`、`struct dsa_lag :: id``）相关联，用于共享端口上的地址引用计数。驱动可以借用
DSA 的编号方案（该 ID 可通过 `db->bridge.num` 和 `db->lag.id` 读取），也可以实现自己的
方案。

只有声明支持 FDB 隔离的驱动才会收到属于 `DSA_DB_PORT` 数据库的 CPU 端口上的 FDB 条目
通知。出于兼容/遗留原因，即使驱动不支持 FDB 隔离，也会向驱动通知 `DSA_DB_BRIDGE` 地址。
但在这种情况下，`db->bridge.num` 和 `db->lag.id` 始终被设为 0（表示缺乏隔离，用于引用
计数目的）。

注意，交换机驱动不必为每个独立 user 端口实现物理上独立的地址数据库。由于端口私有
数据库中的 FDB 条目将始终指向 CPU 端口，不存在错误转发决策的风险。在这种情况下，所有
独立端口可以共享同一个数据库，但主机过滤地址的引用计数（如果端口的 MAC 地址仍被另一个
端口使用，则不删除其 FDB 条目）成为驱动的责任，因为 DSA 并不知道端口数据库实际上是
共享的。这可以通过调用 `dsa_fdb_present_in_other_db()` 和 `dsa_mdb_present_in_other_db()`
来实现。缺点是每个 user 端口的 RX 过滤列表实际上是共享的，这意味着 user 端口 A 可能会
接收到一个它本不应有的、具有某 MAC DA 的报文，仅仅因为该 MAC 地址在 user 端口 B 的 RX
过滤列表中。不过，这些报文仍会在软件中被丢弃。

### 桥接层（Bridge layer）


卸载桥接转发平面是可选的，由下述方法处理。它们可以缺失、返回 -EOPNOTSUPP，或者
`ds->max_num_bridges` 可能非零且被超出，在这种情况下，加入桥接端口仍然可能，但报文
转发将在软件中进行，并且软件桥接下的端口必须保持与独立运行相同的配置，即禁用所有桥接
服务函数（地址学习等），并且仅将所有收到的报文发送到 CPU 端口。

具体来说，一旦端口对 `port_bridge_join` 方法返回成功，它就开卸载桥接的转发平面；在调用
`port_bridge_leave` 之后停止。卸载桥接意味着根据软件桥接端口的状态自主地学习 FDB 条目，
并自主地转发（或泛洪）收到的报文而无需 CPU 干预。即使是在卸载桥接端口时，这也是可选的。
标签协议驱动应为那些已经在入向交换机端口的转发域中被自主转发的报文调用
`dsa_default_offload_fwd_mark(skb)`。DSA 通过 `dsa_port_devlink_setup()` 将具有相同树 ID
的所有交换机端口视为同一桥接转发域的一部分（能够自主地相互转发）。

卸载桥接的 TX 转发过程是一个与简单地卸载其转发平面不同的概念，指的是某些驱动和标签协议
组合能够将从桥接设备发送函数出来的单个 skb 传输到潜在多个出口端口（从而避免在软件中
克隆它）。

桥接请求此行为的报文称为数据平面（data plane）报文，并且在标签协议驱动的 `xmit` 函数中
`skb->offload_fwd_mark` 被设为 true。数据平面报文受 FDB 查找、CPU 端口上的硬件学习约束，
并且不覆盖端口的 STP 状态。此外，数据平面报文的复制（组播、泛洪）在硬件中处理，桥接驱动
将为每个可能需要或不需要复制的报文传输一个 skb。

当启用 TX 转发卸载时，标签协议驱动负责将报文注入硬件的数据平面，导向该端口所属的正确桥接
域（FID）。该端口可能是 VLAN 不感知的，在这种情况下，FID 必须等于驱动用于其与该桥接关联
的 VLAN 不感知地址数据库的 FID。或者，桥接可能是 VLAN 感知的，在这种情况下，保证该报文
也用桥接处理此报文时所用的 VLAN ID 进行了 VLAN 标记。由硬件负责在未标记出向的端口上去掉
VID，或在带标记出向的端口上保留标签。

- `port_bridge_join`：当给定交换机端口被添加到桥接时调用的桥接层函数，该函数应在交换机层面
  做必要的事情，以允许加入的端口被添加到相关的逻辑域，从而使其能够与其他桥接成员进/出流量。
  通过将 `tx_fwd_offload` 参数设为 true，也将卸载此桥接的 TX 转发过程。

- `port_bridge_leave`：当给定交换机端口从桥接中移除时调用的桥接层函数，该函数应在交换机层面
  做必要的事情，以拒绝离开的端口与剩余桥接成员之间的进/出流量。

- `port_stp_state_set`：当桥接层计算出给定交换机端口的 STP 状态并应传播到交换机硬件以
  转发/阻止/学习流量时调用的桥接层函数。

- `port_bridge_flags`：当端口必须配置其设置（例如未知流量泛洪或源地址学习）时调用的桥接层
  函数。交换机驱动负责以禁用地址学习、启用所有类型流量的出向泛洪来初始设置独立端口，然后
  DSA 核心在端口加入和离开桥接时通知对桥接端口标志的任何更改。DSA 目前不为 CPU 端口管理桥接
  端口标志。假设是地址学习应在 CPU 端口上静态启用（如果硬件支持），并且由于 DSA 核心缺乏
  显式的地址过滤机制，向 CPU 端口的泛洪也应启用。

- `port_fast_age`：当有必要刷新端口上动态学习的 FDB 条目时调用的桥接层函数。当从应当进行
  学习的 STP 状态转换到不应进行学习的 STP 状态、或离开桥接、或通过 `port_bridge_flags` 关闭
  地址学习时调用。

### 桥接 VLAN 过滤（Bridge VLAN filtering）


- `port_vlan_filtering`：当桥接被配置为开启或关闭 VLAN 过滤时调用的桥接层函数。如果在硬件层面
  不需要做特定处理，则无需实现此回调。开启 VLAN 过滤时，必须用拒绝具有超出已编程允许的 VLAN
  ID 映射/规则的 802.1Q 帧来编程硬件。如果交换机端口没有编程 PVID，则也必须拒绝未标记的帧。
  关闭时，交换机必须接受任何 802.1Q 帧而不管其 VLAN ID，并且允许未标记的帧。

- `port_vlan_add`：当为给定交换机端口配置 VLAN（带标记或未标记）时调用的桥接层函数。CPU 端口
  成为 VLAN 的成员，仅当外部桥接端口也是其成员（并且需要在软件中进行转发）时，或者该 VLAN 被
  安装到桥接设备自身的 VLAN 组以用于终结目的（`bridge vlan add dev br0 vid 100 self`）。共享
  端口上的 VLAN 是引用计数的，当没有用户时会被移除。驱动不需要手动在 CPU 端口上安装 VLAN。

- `port_vlan_del`：当从给定交换机端口移除 VLAN 时调用的桥接层函数。

- `port_fdb_add`：当桥接想要安装一个转发数据库（Forwarding Database）条目时调用的桥接层函数，
  应用指定地址在与此 VLAN ID 关联的转发数据库中的指定 VLAN ID 对交换机硬件进行编程。

- `port_fdb_del`：当桥接想要删除一个转发数据库条目时调用的桥接层函数，如果指定 MAC 地址已映射到
  该端口转发数据库，则应用对交换机硬件编程以从该指定 VLAN ID 中删除它。

- `port_fdb_dump`：由物理 DSA 端口接口上的 `ndo_fdb_dump` 调用的桥接绕过函数。由于 DSA 不尝试
  将其硬件 FDB 条目与软件桥接保持同步，此方法被实现为一种查看 user 端口在硬件数据库中可见条目的
  手段。此函数报告的条目在 `bridge fdb show` 命令的输出中具有 `self` 标志。

- `port_mdb_add`：当桥接想要安装一个组播数据库条目时调用的桥接层函数。应用指定地址在与此 VLAN
  ID 关联的转发数据库中的指定 VLAN ID 对交换机硬件进行编程。

- `port_mdb_del`：当桥接想要删除一个组播数据库条目时调用的桥接层函数，如果指定 MAC 地址已映射到
  该端口转发数据库，则应用对交换机硬件编程以从该指定 VLAN ID 中删除它。

### 链路聚合（Link aggregation）


链路聚合（Link aggregation）在 Linux 网络协议栈中由 bonding 和 team 驱动实现，它们被建模为
虚拟的、可堆叠的网络接口。DSA 能够将链路聚合组（LAG）卸载到支持该特性的硬件，并支持物理端口
与 LAG 之间以及 LAG 之间的桥接。持有多个物理端口的 bonding/team 接口构成一个逻辑端口，尽管 DSA
目前没有逻辑端口的明确概念。因此，LAG 加入/离开桥接的事件被视为该 LAG 的所有单个物理端口成员
加入/离开桥接。卸载到 LAG 作为桥接端口的 Switchdev 端口属性（VLAN 过滤、STP 状态等）和对象（VLAN、
MDB 条目）也类似处理：DSA 在该 LAG 的所有成员上卸载相同的 switchdev 对象/端口属性。LAG 上的静态
桥接 FDB 条目尚不支持，因为 DSA 驱动 API 没有逻辑端口 ID 的概念。

- `port_lag_join`：当给定交换机端口被添加到 LAG 时调用的函数。驱动可以返回 `-EOPNOTSUPP`，在
  这种情况下，DSA 会回退到软件实现，将该端口的所有流量发送到 CPU。
- `port_lag_leave`：当给定交换机端口离开 LAG 并返回作为独立端口运行时调用的函数。
- `port_lag_change`：当 LAG 的任何成员的链路状态发生变化、并且哈希函数需要重新平衡以仅使用处于
  up 状态的物理 LAG 成员端口子集时调用的函数。

受益于为每个卸载的 LAG 关联一个 ID 的驱动**可以可选地通过 ``dsa_switch_ops** : setup`` 方法
填充 `ds->num_lag_ids`。然后，bonding/team 接口关联的 LAG ID 可以由 DSA 交换机驱动使用
`dsa_lag_id` 函数获取。

### IEC 62439-2 (MRP)


媒体冗余协议（Media Redundancy Protocol，MRP）是一种针对环形网络快速故障恢复时间优化的拓扑
管理协议，其部分组件作为桥接驱动的一个函数实现。MRP 使用在 01:15:4e:00:00:0x 多播目的 MAC
地址范围发送、且 EtherType 为 0x88e3 的管理 PDU（Test、Topology、LinkDown/Up、Option）。根据节点
在环中的角色（MRM：Media Redundancy Manager，MRC：Media Redundancy Client，MRA：Media Redundancy
Automanager），某些 MRP PDU 可能需要在本地终结，而其他的可能需要被转发。MRM 也可能受益于将某些
MRP PDU（Test）的创建和传输卸载到硬件。

通常可以在任何网络接口之上创建 MRP 实例，但在具有如 DSA 这样卸载数据路径的设备情况下，即使硬件
不感知 MRP，也需要硬件能够在驱动进行软件实现之前从 fabric 中提取 MRP PDU。DSA 目前没有感知 MRP 的
驱动，因此它只监听软件辅助正常工作所需的最少 switchdev 对象。操作详述如下。

- `port_mrp_add` 和 `port_mrp_del`：当创建/删除具有特定环 ID、优先级、主端口和次端口的 MRP 实例时
  通知驱动。
- `port_mrp_add_ring_role` 和 `port_mrp_del_ring_role`：当 MRP 实例在 MRM 或 MRC 之间改变环角色时
  调用的函数。这影响哪些 MRP PDU 应陷入（trap）到软件，哪些应被自主转发。

### IEC 62439-3 (HSR/PRP)


并行冗余协议（Parallel Redundancy Protocol，PRP）是一种网络冗余协议，它通过两条独立的 L2 网络
（不感知报文中携带的 PRP 尾部标签）复制并对报文进行序列编号，并在接收端消除重复。高可用性无缝
冗余（High-availability Seamless Redundancy，HSR）协议在概念上类似，只是所有承载冗余流量的节点都
意识到它是 HSR 标记的（因为 HSR 使用 EtherType 为 0x892f 的头部）并且物理上连接成环形拓扑。HSR 和
PRP 都使用监管帧来监控网络健康状况并发现其他节点。

在 Linux 中，HSR 和 PRP 都在 hsr 驱动中实现，该驱动实例化一个具有两个成员端口的虚拟、可堆叠网络
接口。该驱动仅实现 DANH（实现 HSR 的双连接节点，Doubly Attached Node）、DANP（实现 PRP 的双连接节点）
和 RedBox（允许非 HSR 设备通过 Interlink 端口连接到环）的基本角色。

能够卸载某些功能的驱动应声明相应的 netdev 特性，如 `Documentation/networking/netdev-features.rst`
中的文档所示。此外，必须实现以下方法：

- `port_hsr_join`：当给定交换机端口被添加到 DANP/DANH 时调用的函数。驱动可以返回 `-EOPNOTSUPP`，在
  这种情况下，DSA 会回退到软件实现，将该端口的所有流量发送到 CPU。
- `port_hsr_leave`：当给定交换机端口离开 DANP/DANH 并返回作为独立端口正常运行时调用的函数。

注意，`NETIF_F_HW_HSR_DUP` 特性依赖于向多个端口的传输，只要标签协议使用 `dsa_xmit_port_mask()` 辅助
函数，该特性通常可用。如果使用了该辅助函数，也应设置 HSR 卸载特性。`dsa_port_simple_hsr_join()` 和
`dsa_port_simple_hsr_leave()` 方法可用作 `port_hsr_join` 和 `port_hsr_leave` 的通用实现，如果这是唯一
支持的卸载特性。

## TODO


### 使 SWITCHDEV 与 DSA 走向统一的代码库（Making SWITCHDEV and DSA converge towards an unified codebase）


SWITCHDEV 妥善地负责用具有卸载能力的硬件来抽象网络协议栈，但不强制严格的交换机设备驱动模型。另一方面，
DSA 强制相当严格的设备驱动模型，并处理大部分交换机特定的内容。在某个时候，我们应该设想这两个子系统
之间的合并，以兼得两者之长。