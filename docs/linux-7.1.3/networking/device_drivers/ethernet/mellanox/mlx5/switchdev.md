
## Switchdev


:Copyright: |copy| 2023, NVIDIA CORPORATION & AFFILIATES. All rights reserved.


## 桥接卸载（Bridge offload

mlx5 驱动switchdev 模式下实现了对桥接规则卸载的支持。当 mlx5 switchdev
representor（代表端口）被挂接到桥（bridge）时，Linux 桥的 FDB 会被自动卸载
```

    $ devlink dev eswitch set pci/0000:06:00.0 mode switchdev

```
```

    $ ip link set enp8s0f0 master bridge1

```
### VLAN


mlx5 支持以下桥接 VLAN 功能
```

    $ ip link set bridge1 type bridge vlan_filtering 1
    $ bridge vlan add dev enp8s0f0 vid 2-3

```
```

    $ bridge vlan add dev enp8s0f0 vid 3 pvid

```
```

    $ bridge vlan add dev enp8s0f0 vid 3 untagged

```
## 子功能（Subfunction

通过 E-switch 生成的子功能（Subfunction）仅通过 devlink 设备创建，默认情况下所SF 辅助设备都是禁用的。这将允许用户在 SF 被完全探测（probe）之前对其进行配置，从节省时间
使用示例
```

    $ devlink port add pci/0000:08:00.0 flavour pcisf pfnum 0 sfnum 11
    $ devlink port function set pci/0000:08:00.0/32768 hw_addr 00:00:00:00:00:11 state active

```
```

    $ devlink dev param set auxiliary/mlx5_core.sf.1 name enable_eth value true cmode driverinit

```
```

    $ devlink dev reload auxiliary/mlx5_core.sf.1

```
mlx5 支持 ETH、rdma vdpa（vnet）辅助设备的 devlink 参数（参Documentation/networking/devlink/devlink-params.rst <devlink_params_generic>）
mlx5 支持使用 devlink port（参Documentation/networking/devlink/devlink-port.rst <devlink_port>）接口管理子功能
子功能拥有自己的功能能力以及自己的资源。这意味着子功能拥有自己的专用队列（txq、rxq、cq、eq）这些队列既不与父 PCI 功能（parent PCI function）共享，也不会从PCI 功能处窃取
当子功能具备 RDMA 能力时，它拥有自己的 QP1、GID 表，以及 RDMA 资源，既不与PCI 功能
共享，也不会从其窃取
子功能在 PCI BAR 空间中拥有一个专用的窗口，该窗口不与其它子功能或PCI 功能共享。这确保子功能的所有设备（netdev、rdma、vdpa 等）只访问被分配PCI BAR 空间
子功能支eswitch 表示（representation），并借此支持 tc 卸载。用户配eswitch 以向/子功能端口发接收数据包
子功能与PCI 功能及其它子功能共享 PCI 级别的资源，例如 PCI MSI-X IRQ
```

       _______
      | admin |
      | user  |----------
      |_______|         |
          |             |
      ____|____       __|______            _________________
     |         |     |         |          |                 |
     | devlink |     | tc tool |          |    user         |
     | tool    |     |_________|          | applications    |
     |_________|         |                |_________________|
           |             |                   |          |
           |             |                   |          |         Userspace
 +---------|-------------|-------------------|----------|--------------------+
           |             |           +----------+   +----------+   Kernel
           |             |           |  netdev  |   | rdma dev |
           |             |           +----------+   +----------+
   (devlink port add/del |              ^               ^
    port function set)   |              |               |
           |             |              +---------------|
      _____|___          |              |        _______|_______
     |         |         |              |       | mlx5 class    |
     | devlink |   +------------+       |       |   drivers     |
     | kernel  |   | rep netdev |       |       |(mlx5_core,ib) |
     |_________|   +------------+       |       |_______________|
           |             |              |               ^
   (devlink ops)         |              |          (probe/remove)
  _________|________     |              |           ____|________
 | subfunction      |    |     +---------------+   | subfunction |
 | management driver|-----     | subfunction   |---|  driver     |
 | (mlx5_core)      |          | auxiliary dev |   | (mlx5_core) |
 |__________________|          +---------------+   |_____________|
           |                                            ^
  (sf add/del, vhca events)                             |
           |                                      (device add/del)
      _____|____                                    ____|________
     |          |                                  | subfunction |
     |  PCI NIC |--- activate/deactivate events--->| host driver |
     |__________|                                  | (mlx5_core) |
                                                   |_____________|

```
子功能通过 devlink port 接口创建
```

    $ devlink dev eswitch set pci/0000:06:00.0 mode switchdev

```
```

    $ devlink port add pci/0000:06:00.0 flavour pcisf pfnum 0 sfnum 88
    pci/0000:06:00.0/32768: type eth netdev eth6 flavour pcisf controller 0 pfnum 0 sfnum 88 external false splittable false
      function:
        hw_addr 00:00:00:00:00:00 state inactive opstate detached

```
```

    $ devlink port show pci/0000:06:00.0/32768
    pci/0000:06:00.0/32768: type eth netdev enp6s0pf0sf88 flavour pcisf pfnum 0 sfnum 88
      function:
        hw_addr 00:00:00:00:00:00 state inactive opstate detached

```
```

    $ devlink port del pci/0000:06:00.0/32768

```
## 功能属

mlx5 驱动提供了一种机制，以统一的方式为 SmartNIC 与非 SmartNIC 设置 PCI VF/SF 功能属性
这仅eswitch 模式设置switchdev 时才受支持。PCI VF/SF 的端口功能配置通过 devlink
eswitch port 支持
端口功能属性应PCI VF/SF 被驱动枚举之前设置
### MAC 地址设置


mlx5 驱动支持 devlink port function attr 机制来设MAC 地址。（参见 Documentation/networking/devlink/devlink-port.rst
#### RoCE 能力设置


并非所mlx5 PCI 设备/SF 都需RoCE 能力
RoCE 能力被禁用时，每PCI 设备/SF 可节1 Mbytes 的系统内存
mlx5 驱动支持 devlink port function attr 机制来设RoCE 能力。（参见 Documentation/networking/devlink/devlink-port.rst
#### 可迁移（migratable）能力设

希望 mlx5 PCI VF 能够进行实时迁移（live migration）的用户，需要显式地启用 VF 的可迁移能力
mlx5 驱动支持 devlink port function attr 机制来设置可迁移能力。（参见 Documentation/networking/devlink/devlink-port.rst
#### IPsec crypto 能力设置


希望 mlx5 PCI VF 能够进行 IPsec crypto 卸载的用户，需要显式地启用 VF ipsec_crypto 能力ConnectX6dx 及以上设备开始支持为 VF 启用 IPsec 能力。当 VF 启用IPsec 能力时，PF 上的任何
IPsec 卸载都会被阻塞
mlx5 驱动支持 devlink port function attr 机制来设ipsec_crypto 能力。（参见 Documentation/networking/devlink/devlink-port.rst
#### IPsec packet 能力设置


希望 mlx5 PCI VF 能够进行 IPsec packet 卸载的用户，需要显式地启用 VF ipsec_packet 能力ConnectX6dx 及以上设备开始支持为 VF 启用 IPsec 能力。当 VF 启用IPsec 能力时，PF 上的任何
IPsec 卸载都会被阻塞
mlx5 驱动支持 devlink port function attr 机制来设ipsec_packet 能力。（参见 Documentation/networking/devlink/devlink-port.rst
### SF 状态设

要使SF，用户必须通过 SF 功能状态（function state）属性来激SF
```

   $ devlink port show ens2f0npf0sf88
   pci/0000:06:00.0/32768: type eth netdev ens2f0npf0sf88 flavour pcisf controller 0 pfnum 0 sfnum 88 external false splittable false
     function:
       hw_addr 00:00:00:00:88:88 state inactive opstate detached

```
```

   $ devlink port function set ens2f0npf0sf88 state active

   $ devlink port show ens2f0npf0sf88
   pci/0000:06:00.0/32768: type eth netdev ens2f0npf0sf88 flavour pcisf controller 0 pfnum 0 sfnum 88 external false splittable false
     function:
       hw_addr 00:00:00:00:88:88 state active opstate detached

```
功能激活后，PF 驱动实例会从设备收到某个特定 SF 已被激活的事件。这是将该设备放到总线上对其进行探测（probe）并为其实例devlink 实例以及类特定的辅助设备的信号
```

    $ devlink dev show
    devlink dev show auxiliary/mlx5_core.sf.4

    $ devlink port show auxiliary/mlx5_core.sf.4/1
    auxiliary/mlx5_core.sf.4/1: type eth netdev p0sf88 flavour virtual port 0 splittable false

    $ rdma link show mlx5_0/1
    link mlx5_0/1 state ACTIVE physical_state LINK_UP netdev p0sf88

    $ rdma dev show
    8: rocep6s0f1: node_type ca fw 16.29.0550 node_guid 248a:0703:00b3:d113 sys_image_guid 248a:0703:00b3:d112
    13: mlx5_0: node_type ca fw 16.29.0550 node_guid 0000:00ff:fe00:8888 sys_image_guid 248a:0703:00b3:d112

```
```

                 mlx5_core.sf.4
          (subfunction auxiliary device)
                       /\
                      /  \
                     /    \
                    /      \
                   /        \
      mlx5_core.eth.4     mlx5_core.rdma.4
     (sf eth aux dev)     (sf rdma aux dev)
         |                      |
         |                      |
      p0sf88                  mlx5_0
     (sf netdev)          (sf rdma device)

```
此外，当驱动挂接到子功能的辅助设备时，SF 端口也会收到该事件。这会改变功能的操作（operational状态。这让用户能够判断何时可以安全地删除 SF 端口，以实现子功能的优雅终止
```

    $ devlink port show ens2f0npf0sf88
    pci/0000:06:00.0/32768: type eth netdev ens2f0npf0sf88 flavour pcisf controller 0 pfnum 0 sfnum 88 external false splittable false
      function:
        hw_addr 00:00:00:00:88:88 state active opstate attached

```
