
## PF 网络设备


## 目录


- `背景`_
- `概述`_
- `mlx5 实现`_
- `通道分配`_
- `可观测性`_
- `流导向`_
- `互斥功能`_

## 背景


PF NIC 技术使多路服务器中的多CPU 能够通过各自的专PCIe 接口直接连接网络，既可以通过在两张卡之间分割 PCIe 通道的连接线束，也可以通过为单张卡分叉
PCIe 插槽来实现。这消除了跨插槽内部总线传输的网络流量，显著降低了开销和延迟，
同时减少CPU 利用率并提高了网络吞吐量
## 概述


该特性支持在PF 环境中将同一端口的多PF 合并到一netdev 实例之下。它实现netdev 层。底层实例（pci 功能、sysfs 条目devlink）保持独立通过不同的设备（属于不同NUMA 插槽）传输流量，可以节省NUMA 流量，并允许
运行于同一 netdev 上、来自不NUMA 的应用程序仍然感受到与设备的邻近性，从而获改善的性能
## mlx5 实现


mlx5 中的PF Socket-direct 是通过将属于同一 NIC 且启用了 socket-direct 属性的
PF 分组在一起来实现的，一旦所PF 都被探测到，我们就创建一个单一netdev 代表它们全部；对称地，当任何一PF 被移除时，我们销毁该 netdev
netdev 的网络通道被分配到所有设备之间，正确的配置会在处理某个应CPU 时使正确的邻NUMA 节点
我们选择一PF 作为PF（领导者），它承担特殊角色。其它设备（从属）在芯片层面
与网络断开（设为静默模式）。在静默模式下，从属 PF 之间没有<-> 北流量直接流过它需要在领导PF（东 <-> 西流量）的协助下才能工作。所Rx/Tx 流量都通过PF
向从PF 导向或从中流出
目前，我们将支持限制为仅 PF，且最多两PF（插槽）
## 通道分配


我们在不同的 PF 之间分配通道，以在多NUMA 节点上实现本NUMA 节点性能
每个组合通道针对一个特定的 PF 工作，针对它创建其所有的数据路径队列。我们以
轮询策略将通道分配PF
```

        Example for 2 PFs and 5 channels:
        +--------+--------+
        | ch idx | PF idx |
        +--------+--------+
        |    0   |    0   |
        |    1   |    1   |
        |    2   |    0   |
        |    3   |    1   |
        |    4   |    0   |
        +--------+--------+


```
我们倾向于轮询的原因是，它较少受到通道数量变化的影响。通道索引PF 之间的映是固定的，无论用户配置了多少通道。由于通道统计在通道关闭期间是持久的，每次都
改变映射会使累积统计不能很好地代表通道的历史
这是通过在每个通道中使用正确的核心设备实例（mdev），而不是全部使"priv->mdev"
下的同一实例来实现的
## 可观测```

  $ ./tools/net/ynl/pyynl/cli.py --spec Documentation/netlink/specs/netdev.yaml --dump queue-get --json='{"ifindex": 13}'
  [{'id': 0, 'ifindex': 13, 'napi-id': 539, 'type': 'rx'},
   {'id': 1, 'ifindex': 13, 'napi-id': 540, 'type': 'rx'},
   {'id': 2, 'ifindex': 13, 'napi-id': 541, 'type': 'rx'},
   {'id': 3, 'ifindex': 13, 'napi-id': 542, 'type': 'rx'},
   {'id': 4, 'ifindex': 13, 'napi-id': 543, 'type': 'rx'},
   {'id': 0, 'ifindex': 13, 'napi-id': 539, 'type': 'tx'},
   {'id': 1, 'ifindex': 13, 'napi-id': 540, 'type': 'tx'},
   {'id': 2, 'ifindex': 13, 'napi-id': 541, 'type': 'tx'},
   {'id': 3, 'ifindex': 13, 'napi-id': 542, 'type': 'tx'},
   {'id': 4, 'ifindex': 13, 'napi-id': 543, 'type': 'tx'}]

  $ ./tools/net/ynl/pyynl/cli.py --spec Documentation/netlink/specs/netdev.yaml --dump napi-get --json='{"ifindex": 13}'
  [{'id': 543, 'ifindex': 13, 'irq': 42},
   {'id': 542, 'ifindex': 13, 'irq': 41},
   {'id': 541, 'ifindex': 13, 'irq': 40},
   {'id': 540, 'ifindex': 13, 'irq': 39},
   {'id': 539, 'ifindex': 13, 'irq': 36}]

```
```

  $ ls /proc/irq/{36,39,40,41,42}/mlx5* -d -1
  /proc/irq/36/mlx5_comp0@pci:0000:08:00.0
  /proc/irq/39/mlx5_comp0@pci:0000:09:00.0
  /proc/irq/40/mlx5_comp1@pci:0000:08:00.0
  /proc/irq/41/mlx5_comp1@pci:0000:09:00.0
  /proc/irq/42/mlx5_comp2@pci:0000:08:00.0

```
## 流导

从属 PF 被设静默"模式，意味着它们与网络断开
Rx 中，流导向表仅属于主 PF，由其负责通过vhca 流导向能力将 incoming 流量
分发到其PF。仍然维护一个单一的默RSS 表，它能够指向不PF 的接收队列
Tx 中，PF 创建一个新Tx 流表，由从属 PF 别名引用，以便它们可以通过出去到网络
此外，我们设置默认的 XPS 配置，它基于 CPU 选择属于与该 CPU 同一节点PF SQ
XPS 默认配置示例
NUMA node(s):          2
NUMA node0 CPU(s):     0-11
NUMA node1 CPU(s):     12-23

PF0 node0 上，PF1 node1 上
- /sys/class/net/eth2/queues/tx-0/xps_cpus:000001
- /sys/class/net/eth2/queues/tx-1/xps_cpus:001000
- /sys/class/net/eth2/queues/tx-2/xps_cpus:000002
- /sys/class/net/eth2/queues/tx-3/xps_cpus:002000
- /sys/class/net/eth2/queues/tx-4/xps_cpus:000004
- /sys/class/net/eth2/queues/tx-5/xps_cpus:004000
- /sys/class/net/eth2/queues/tx-6/xps_cpus:000008
- /sys/class/net/eth2/queues/tx-7/xps_cpus:008000
- /sys/class/net/eth2/queues/tx-8/xps_cpus:000010
- /sys/class/net/eth2/queues/tx-9/xps_cpus:010000
- /sys/class/net/eth2/queues/tx-10/xps_cpus:000020
- /sys/class/net/eth2/queues/tx-11/xps_cpus:020000
- /sys/class/net/eth2/queues/tx-12/xps_cpus:000040
- /sys/class/net/eth2/queues/tx-13/xps_cpus:040000
- /sys/class/net/eth2/queues/tx-14/xps_cpus:000080
- /sys/class/net/eth2/queues/tx-15/xps_cpus:080000
- /sys/class/net/eth2/queues/tx-16/xps_cpus:000100
- /sys/class/net/eth2/queues/tx-17/xps_cpus:100000
- /sys/class/net/eth2/queues/tx-18/xps_cpus:000200
- /sys/class/net/eth2/queues/tx-19/xps_cpus:200000
- /sys/class/net/eth2/queues/tx-20/xps_cpus:000400
- /sys/class/net/eth2/queues/tx-21/xps_cpus:400000
- /sys/class/net/eth2/queues/tx-22/xps_cpus:000800
- /sys/class/net/eth2/queues/tx-23/xps_cpus:800000

## 互斥功能


PF 的本质是不同通道与不PF 配合工作，这与状态维护在其中一PF 中的有状功能相冲突。例如，TLS 设备卸载功能中，会为每个连接创建特殊的上下文对象并维PF 中。在不同 RQ/SQ 之间切换会破坏该功能。因此，我们暂时禁用了这种组合