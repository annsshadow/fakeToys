## Devlink 端口


`devlink-port` 是设备上存在的端口。它具有逻辑上独立的设备入口/出口点。一个 devlink 端口可以是多种类型（flavour）中的任意一种。devlink 端口的类型（flavour）连同端口属性一起描述了该端口代表什么。

打算发布一个 devlink 端口的设备驱动会设置该 devlink 端口的属性，并注册该 devlink 端口。

Devlink 端口类型（flavour）描述如下。

   :widths: 33 90

   - - 类型
     - 描述
   - - `DEVLINK_PORT_FLAVOUR_PHYSICAL`
     - 任意种类的物理端口。可以是 eswitch 物理端口，或设备上的任何其他物理端口。
   - - `DEVLINK_PORT_FLAVOUR_DSA`
     - 表示一个 DSA 互连端口。
   - - `DEVLINK_PORT_FLAVOUR_CPU`
     - 表示一个仅适用于 DSA 的 CPU 端口。
   - - `DEVLINK_PORT_FLAVOUR_PCI_PF`
     - 表示一个代表 PCI 物理功能（PF）端口的 eswitch 端口。
   - - `DEVLINK_PORT_FLAVOUR_PCI_VF`
     - 表示一个代表 PCI 虚拟功能（VF）端口的 eswitch 端口。
   - - `DEVLINK_PORT_FLAVOUR_PCI_SF`
     - 表示一个代表 PCI 子功能（SF）端口的 eswitch 端口。
   - - `DEVLINK_PORT_FLAVOUR_VIRTUAL`
     - 表示一个用于 PCI 虚拟功能的虚拟端口。

Devlink 端口可以基于下述链路层拥有不同的类型。

   :widths: 23 90

   - - 类型
     - 描述
   - - `DEVLINK_PORT_TYPE_ETH`
     - 当端口的链路层为以太网时，驱动应设置此端口类型。
   - - `DEVLINK_PORT_TYPE_IB`
     - 当端口的链路层为 InfiniBand 时，驱动应设置此端口类型。
   - - `DEVLINK_PORT_TYPE_AUTO`
     - 当用户希望驱动自动检测端口类型时，指示此类型。

### PCI 控制器

在大多数情况下，一个 PCI 设备只有一个控制器。一个控制器由潜在多个物理功能、虚拟功能以及子功能组成。一个功能由一个或多个端口组成。该端口由 devlink eswitch 端口表示。

但是，连接到多个 CPU、或多个 PCI 根复合体、或一个 SmartNIC 的 PCI 设备，可能拥有多个控制器。对于具有多个控制器的设备，每个控制器通过一个唯一的控制器编号来区分。eswitch 位于支持多个控制器端口的 PCI 设备上。

```
                 ---------------------------------------------------------
                 |                                                       |
                 |           --------- ---------         ------- ------- |
    -----------  |           | vf(s) | | sf(s) |         |vf(s)| |sf(s)| |
    | server  |  | -------   ----/---- ---/----- ------- ---/--- ---/--- |
    | pci rc  |=== | pf0 |______/________/       | pf1 |___/_______/     |
    | connect |  | -------                       -------                 |
    -----------  |     | controller_num=1 (no eswitch)                   |
                 ------|--------------------------------------------------
                 (internal wire)
                       |
                 ---------------------------------------------------------
                 | devlink eswitch ports and reps                        |
                 | ----------------------------------------------------- |
                 | |ctrl-0 | ctrl-0 | ctrl-0 | ctrl-0 | ctrl-0 |ctrl-0 | |
                 | |pf0    | pf0vfN | pf0sfN | pf1    | pf1vfN |pf1sfN | |
                 | ----------------------------------------------------- |
                 | |ctrl-1 | ctrl-1 | ctrl-1 | ctrl-1 | ctrl-1 |ctrl-1 | |
                 | |pf0    | pf0vfN | pf0sfN | pf1    | pf1vfN |pf1sfN | |
                 | ----------------------------------------------------- |
                 |                                                       |
                 |                                                       |
    -----------  |           --------- ---------         ------- ------- |
    | smartNIC|  |           | vf(s) | | sf(s) |         |vf(s)| |sf(s)| |
    | pci rc  |==| -------   ----/---- ---/----- ------- ---/--- ---/--- |
    | connect |  | | pf0 |______/________/       | pf1 |___/_______/     |
    -----------  | -------                       -------                 |
                 |                                                       |
                 |  local controller_num=0 (eswitch)                     |
                 ---------------------------------------------------------
```

在上述示例中，外部控制器（由 controller number = 1 标识）没有 eswitch。本地控制器（由 controller number = 0 标识）拥有 eswitch。本地控制器上的 Devlink 实例为两个控制器都提供了 eswitch devlink 端口。

## 功能配置

用户可以在枚举 PCI 功能之前配置一个或多个功能属性。通常这意味着，用户应当在为该功能创建特定于总线的设备之前配置功能属性。但是，当启用 SRIOV 时，虚拟功能设备会在 PCI 总线上创建。因此，应当在将虚拟功能设备绑定到驱动之前配置功能属性。对于子功能，这意味着用户应当在激活端口功能之前配置端口功能属性。

用户可以使用 `devlink port function set hw_addr` 命令设置该功能的硬件地址。对于以太网端口功能，这表示 MAC 地址。

用户也可以使用 `devlink port function set roce` 命令设置该功能的 RoCE 能力。

用户也可以使用 `devlink port function set migratable` 命令将该功能设置为可迁移的。

用户也可以使用 `devlink port function set ipsec_crypto` 命令设置该功能的 IPsec crypto 能力。

用户也可以使用 `devlink port function set ipsec_packet` 命令设置该功能的 IPsec packet 能力。

用户也可以使用 `devlink port function set max_io_eqs` 命令设置该功能的最大 IO 事件队列数。

## 功能属性

### MAC 地址设置

为 PCI VF/SF 配置的 MAC 地址将被为该 PCI VF/SF 创建的 netdevice 和 rdma 设备使用。

```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
      function:
        hw_addr 00:00:00:00:00:00

```
```
    $ devlink port function set pci/0000:06:00.0/2 hw_addr 00:11:22:33:44:55

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
      function:
        hw_addr 00:11:22:33:44:55

```
```
    $ devlink port show pci/0000:06:00.0/32768
    pci/0000:06:00.0/32768: type eth netdev enp6s0pf0sf88 flavour pcisf pfnum 0 sfnum 88
      function:
        hw_addr 00:00:00:00:00:00

```
```
    $ devlink port function set pci/0000:06:00.0/32768 hw_addr 00:00:00:00:88:88

    $ devlink port show pci/0000:06:00.0/32768
    pci/0000:06:00.0/32768: type eth netdev enp6s0pf0sf88 flavour pcisf pfnum 0 sfnum 88
      function:
        hw_addr 00:00:00:00:88:88

```
### RoCE 能力设置

并非所有 PCI VF/SF 都需要 RoCE 能力。

当禁用 RoCE 能力时，会为每个 PCI VF/SF 节省系统内存。

当用户为某个 VF/SF 禁用 RoCE 能力时，用户应用程序无法通过该 VF/SF 发送或接收任何 RoCE 数据包，并且该 PCI 的 RoCE GID 表将为空。

当使用端口功能属性在设备中禁用 RoCE 能力时，VF/SF 驱动无法覆盖它。

```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 roce enable

```
```
    $ devlink port function set pci/0000:06:00.0/2 roce disable

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 roce disable

```
### 可迁移能力设置

实时迁移（Live migration）是在不中断其正常运行的情况下，将正在运行的虚拟机从一台物理主机转移到另一台物理主机的过程。

希望 PCI VF 能够执行实时迁移的用户，需要显式地启用 VF 的可迁移能力。

当用户为 VF 启用可迁移能力，并且 HV 将 VF 绑定到支持迁移的 VFIO 驱动时，用户可以将带有该 VF 的虚拟机从一台 HV 迁移到另一台 HV。

但是，当启用可迁移能力时，设备会禁用那些无法迁移的特性。因此可迁移能力会对 VF 施加限制，由用户自行决定。

使用可迁移功能配置的 LM 示例：
```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 migratable disable

```
```
    $ devlink port function set pci/0000:06:00.0/2 migratable enable

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 migratable enable

```
```
    $ echo <pci_id> > /sys/bus/pci/devices/0000:08:00.0/driver/unbind
    $ echo mlx5_vfio_pci > /sys/bus/pci/devices/0000:08:00.0/driver_override
    $ echo <pci_id> > /sys/bus/pci/devices/0000:08:00.0/driver/bind

```
将 VF 附加到虚拟机。
启动虚拟机。
执行实时迁移。

### IPsec crypto 能力设置

当用户为 VF 启用 IPsec crypto 能力时，用户应用程序可以将 XFRM 状态 crypto 操作（加密/解密）卸载到该 VF。

当 VF 的 IPsec crypto 能力被禁用（默认）时，XFRM 状态由内核在软件中处理。

```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_crypto disabled

```
```
    $ devlink port function set pci/0000:06:00.0/2 ipsec_crypto enable

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_crypto enabled

```
### IPsec packet 能力设置

当用户为 VF 启用 IPsec packet 能力时，用户应用程序可以将 XFRM 状态和策略 crypto 操作（加密/解密）以及 IPsec 封装卸载到该 VF。

当 VF 的 IPsec packet 能力被禁用（默认）时，XFRM 状态和策略由内核在软件中处理。

```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_packet disabled

```
```
    $ devlink port function set pci/0000:06:00.0/2 ipsec_packet enable

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_packet enabled

```
### 最大 IO 事件队列设置

当用户为 SF 或 VF 设置最大 IO 事件队列数时，该功能驱动被限制为只能消耗所强制规定的 IO 事件队列数。

IO 事件队列传递与 IO 队列相关的事件，包括网络设备发送和接收队列（txq 和 rxq）以及 RDMA 队列对（QP）。例如，netdevice 通道数和 RDMA 设备完成向量的数量都派生自该功能的 IO 事件队列。通常，驱动消耗的终端向量数量受每个设备的 IO 事件队列数量限制，因为每个 IO 事件队列都连接到一个中断向量。

```
    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_packet disabled max_io_eqs 10

```
```
    $ devlink port function set pci/0000:06:00.0/2 max_io_eqs 32

    $ devlink port show pci/0000:06:00.0/2
    pci/0000:06:00.0/2: type eth netdev enp6s0pf0vf1 flavour pcivf pfnum 0 vfnum 1
        function:
            hw_addr 00:00:00:00:00:00 ipsec_packet disabled max_io_eqs 32

```
## 子功能（Subfunction）


子功能（Subfunction）是一种轻量级功能，它部署在一个父 PCI 功能之上。子功能以 1 为单位创建和部署。与 SRIOV VF 不同，子功能不需要自己的 PCI 虚拟功能。子功能通过父 PCI 功能与硬件通信。

要使用子功能，需要遵循 3 步设置流程：

1) 创建（create）——创建一个子功能；
2) 配置（configure）——配置子功能属性；
3) 部署（deploy）——部署该子功能；

子功能管理通过 devlink 端口用户界面完成。用户在子功能管理设备上执行设置。

### (1) 创建

子功能通过 devlink 端口接口创建。用户通过添加一个子功能类型的 devlink 端口来添加子功能。devlink 内核代码向下调用子功能管理驱动（devlink ops），并要求它创建一个子功能 devlink 端口。然后驱动实例化该子功能端口以及任何关联的对象，例如健康报告器（health reporter）和代表（representor） netdevice。

### (2) 配置

已创建子功能 devlink 端口，但它尚未激活。这意味着实体已在 devlink 一侧创建，e-switch 端口代表（representor）也已创建，但子功能设备本身尚未创建。用户可以使用 e-switch 端口代表进行设置、将其加入网桥、添加 TC 规则等。用户也可以在子功能处于非活动状态时配置其硬件地址（例如 MAC 地址）。

### (3) 部署

一旦子功能配置完成，用户必须激活它才能使用它。激活时，子功能管理驱动会要求子功能管理设备在特定 PCI 功能上实例化子功能设备。子功能设备在 Documentation/driver-api/auxiliary_bus.rst <auxiliary_bus> 上创建。此时，一个匹配的子功能驱动会绑定到该子功能的辅助设备。

## 速率对象管理

Devlink 提供用于管理单个 devlink 端口或一组端口的 tx 速率的 API。这是通过速率对象完成的，速率对象可以是以下两种类型之一：

`leaf`
  代表单个 devlink 端口；由驱动创建/销毁。由于 leaf 与其 devlink 端口是 1 对 1 映射，在用户空间中它被称为 `pci/<bus_addr>/<port_index>`；

`node`
  代表一组速率对象（leaf 和/或 node）；由来自用户空间的请求创建/删除；最初为空（未添加任何速率对象）。在用户空间中它被称为 `pci/<bus_addr>/<node_name>`，其中 `node_name` 可以是除十进制数字之外的任何标识符，以避免与 leaf 冲突。

API 允许配置以下速率对象参数：

`tx_share`
  在所有其他速率对象之间共享的最小 TX 速率值；如果它是同一组的一部分，则是父组的速率对象的一部分。

`tx_max`
  最大 TX 速率值。

`tx_priority`
  允许在兄弟节点之间使用严格优先级仲裁器。该仲裁方案尝试根据节点的优先级来调度节点，只要节点仍在其带宽限制内。优先级越高，该节点被选中进行调度的概率就越大。

`tx_weight`
  允许在兄弟节点之间使用权重公平排队（Weighted Fair Queuing）仲裁方案。该仲裁方案可以与严格优先级同时使用。当节点配置有更高的速率时，它相对于其兄弟节点获得更多的带宽（BW）。值就像百分比点一样是相对的，它们基本上告诉节点相对于其兄弟节点应获取多少带宽。

`parent`
  父节点名称。父节点速率限制被视为对其所有子节点限制的额外限制。`tx_max` 是子节点的上限。`tx_share` 是在子节点之间分配的总带宽。

`tc_bw`
  允许用户设置速率对象上每个流量类的带宽分配。这通过对每个流量类分配一个相对份额值，实现了细粒度的 QoS 配置。带宽按照每个类的份额值相对于所有份额之和的比例进行分配。当应用于非叶子节点时，tc_bw 决定了其各个子元素之间如何共享带宽。

`tx_priority` 和 `tx_weight` 可以同时使用。在这种情况下，具有相同优先级的节点在兄弟组中形成一个 WFQ 子组，它们之间的仲裁基于所分配的权重。

从高层次看，仲裁流程如下：

#. 选择一个优先级最高、处于带宽限制内且未被阻塞的节点或节点组。使用 `tx_priority` 作为此仲裁的参数。

#. 如果一组节点具有相同的优先级，则在该子组上执行 WFQ 仲裁。使用 `tx_weight` 作为此仲裁的参数。

#. 选出获胜节点，并继续在其子节点间进行仲裁，直到到达叶子节点，从而确定获胜者。

#. 如果最高优先级子组中的所有节点都已满足或超出其分配的带宽，则转向较低优先级的节点。

驱动实现允许支持两种速率对象类型之一或两者，以及它们参数的设置方法。此外，驱动实现可以导出 node/leaf 及其父子关系。

## 术语与定义


   :widths: 22 90

   - - 术语
     - 定义
   - - `PCI device`
     - 一个物理 PCI 设备，具有一个或多个由 PCI 总线组成的 PCI 控制器。
   - - `PCI controller`
     - 一个控制器由潜在多个物理功能、虚拟功能以及子功能组成。
   - - `Port function`
     - 用于管理端口功能的对象。
   - - `Subfunction`
     - 一种轻量级功能，部署在父 PCI 功能之上。
   - - `Subfunction device`
     - 子功能的总线设备，通常位于辅助总线上。
   - - `Subfunction driver`
     - 子功能辅助设备的设备驱动。
   - - `Subfunction management device`
     - 支持子功能管理的 PCI 物理功能。
   - - `Subfunction management driver`
     - 支持使用 devlink 端口接口进行子功能管理的 PCI 物理功能的设备驱动。
   - - `Subfunction host driver`
     - 承载子功能设备的 PCI 物理功能的设备驱动。在大多数情况下它与子功能管理驱动相同。当子功能用于外部控制器时，子功能管理和宿主驱动是不同的。
