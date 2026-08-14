
## 接口统计


## 概述


本文档是 Linux 网络接口统计的指南。

Linux 中有三个主要的接口统计来源：

 - 基于 `struct rtnl_link_stats64 <rtnl_link_stats64>` 的标准接口统计；
 - 协议特定的统计；以及
 - 通过 ethtool 可用的驱动定义统计。

### 标准接口统计


有多种接口可以访问标准统计。
```

  $ ip -s -s link show dev ens4u1u1
  6: ens4u1u1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000
    link/ether 48:2a:e3:4c:b1:d1 brd ff:ff:ff:ff:ff:ff
    RX: bytes  packets  errors  dropped overrun mcast
    74327665117 69016965 0       0       0       0
    RX errors: length   crc     frame   fifo    missed
               0        0       0       0       0
    TX: bytes  packets  errors  dropped carrier collsns
    21405556176 44608960 0       0       0       0
    TX errors: aborted  fifo   window heartbeat transns
               0        0       0       0       128
    altname enp58s0u1u1

```
注意，`-s` 被指定了两次，以查看 `struct rtnl_link_stats64 <rtnl_link_stats64>`
的所有成员。如果只指定一次 `-s`，则不会显示详细的错误。

`ip` 支持通过 `-j` 选项进行 JSON 格式化。

#### 队列统计


队列统计可以通过 netdev netlink 系列访问。

目前没有广泛分发的 CLI 来访问这些统计。内核开发工具（ynl）可用于试验它们，参见
`Documentation/userspace-api/netlink/intro-specs.rst`。

### 协议特定的统计


协议特定的统计通过相关接口暴露，这些接口与用于配置它们的接口相同。

#### ethtool


Ethtool 暴露常见的底层统计。所有标准统计都预期由设备（而非驱动）维护（与下一节
描述的驱动定义统计不同，后者混合了软件和硬件统计）。对于包含非托管交换机（例如
传统 SR-IOV 或多主机 NIC）的设备，所计数的事件可能并非专门对应于发往本地主机
接口的数据包。换句话说，事件可能在网络端口（MAC/PHY 模块）处被计数，而不区分
不同的主机侧（PCIe）设备。当内部交换机由 Linux 管理时（即 NIC 的所谓 switchdev
模式），不得存在这种歧义。

标准 ethtool 统计可以通过用于配置的接口访问。例如使用的 ethtool 接口
```

  $ ethtool --include-statistics -a eth0
  Pause parameters for eth0:
  Autonegotiate:	on
  RX:			on
  TX:			on
  Statistics:
    tx_pause_frames: 1
    rx_pause_frames: 1

```
与任何特定功能无关的通用以太网统计通过 `ethtool -S $ifc` 暴露，通过指定
```

  $ ethtool -S eth0 --groups eth-phy eth-mac eth-ctrl rmon
  Stats for eth0:
  eth-phy-SymbolErrorDuringCarrier: 0
  eth-mac-FramesTransmittedOK: 1
  eth-mac-FrameTooLongErrors: 1
  eth-ctrl-MACControlFramesTransmitted: 1
  eth-ctrl-MACControlFramesReceived: 0
  eth-ctrl-UnsupportedOpcodesReceived: 1
  rmon-etherStatsUndersizePkts: 1
  rmon-etherStatsJabbers: 0
  rmon-rx-etherStatsPkts64Octets: 1
  rmon-rx-etherStatsPkts65to127Octets: 0
  rmon-rx-etherStatsPkts128to255Octets: 0
  rmon-tx-etherStatsPkts64Octets: 2
  rmon-tx-etherStatsPkts65to127Octets: 3
  rmon-tx-etherStatsPkts128to255Octets: 0

```
### 驱动定义的统计


```

  $ ethtool -S ens4u1u1
  NIC statistics:
     tx_single_collisions: 0
     tx_multi_collisions: 0

```
## uAPI


### procfs


历史性的 `/proc/net/dev` 文本接口提供了对接口列表及其统计的访问。

注意，即使此接口内部使用 `struct rtnl_link_stats64 <rtnl_link_stats64>`，它也
合并了其中一些字段。

### sysfs


sysfs 中每个设备目录都包含一个 `statistics` 目录（例如
`/sys/class/net/lo/statistics/`），其中的文件对应于 `struct rtnl_link_stats64
<rtnl_link_stats64>` 的成员。

这个简单的接口在没有工具可用的受限/嵌入式环境中尤其方便。然而，当读取多个统计时
它效率低下，因为它内部执行了一次 `struct rtnl_link_stats64 <rtnl_link_stats64>`
的完整转储，并只报告与所访问文件对应的统计。

Sysfs 文件记录在 Documentation/ABI/testing/sysfs-class-net-statistics。


### netlink


`rtnetlink`（`NETLINK_ROUTE`）是访问 `struct rtnl_link_stats64
<rtnl_link_stats64>` 统计的首选方法。

统计在链路信息请求（`RTM_GETLINK`）和统计请求（`RTM_GETSTATS`，当请求的
`.filter_mask` 中设置了 `IFLA_STATS_LINK_64` 位时）的响应中都会被报告。

#### netdev（netlink）


`netdev` 通用 netlink 系列允许访问页池和每队列统计。

### ethtool


Ethtool IOCTL 接口允许驱动报告实现特定的统计。历史上它也被用于报告其它 API 不存在
的统计，例如每设备队列统计，或基于标准的统计（例如 RFC 2863）。

统计及其字符串标识符是分别获取的。标识符通过 `ETHTOOL_GSTRINGS`（将 `string_set`
设为 `ETH_SS_STATS`）获取，值通过 `ETHTOOL_GSTATS` 获取。用户空间应使用
`ETHTOOL_GDRVINFO` 检索统计的数量（`.n_stats`）。

### ethtool-netlink


Ethtool netlink 是对较旧 IOCTL 接口的替代。

协议相关的统计可以在 get 命令中通过设置 `ETHTOOL_A_HEADER_FLAGS` 中的
`ETHTOOL_FLAG_STATS` 标志来请求。目前以下命令支持统计：

  - `ETHTOOL_MSG_FEC_GET`
  - `ETHTOOL_MSG_LINKSTATE_GET`
  - `ETHTOOL_MSG_MM_GET`
  - `ETHTOOL_MSG_PAUSE_GET`
  - `ETHTOOL_MSG_TSINFO_GET`

### debugfs


一些驱动通过 `debugfs` 暴露额外的统计。

## struct rtnl_link_stats64


    :identifiers: rtnl_link_stats64

## 给驱动作者的注意事项


驱动应当报告所有在 `struct rtnl_link_stats64 <rtnl_link_stats64>` 中有对应成员的
统计，且只能通过 `.ndo_get_stats64` 报告。通过 ethtool 或 debugfs 报告此类标准
统计将不被接受。

驱动必须确保与 `struct rtnl_link_stats64 <rtnl_link_stats64>` 尽可能兼容。请注意
例如，详细的错误统计必须被加入通用的 `rx_error` / `tx_error` 计数器中。

`.ndo_get_stats64` 回调不能睡眠，因为会通过 `/proc/net/dev` 访问。如果驱动在从
设备检索统计时可能会睡眠，它应当定期异步地执行，并且只从 `.ndo_get_stats64` 返回
最近的副本。如有需要，ethtool 中断聚合接口允许设置刷新统计的频率。

检索 ethtool 统计是一个多系统调用的过程，建议驱动保持统计数量恒定，以避免与试图
读取它们的用户空间发生竞态条件。

统计必须跨常规操作（例如将接口关闭再开启）持续存在。

### 内核内部数据结构


以下结构是内核内部的，它们在被转储时被转换为 netlink 属性。驱动绝不能用 0 覆盖
它们未报告的统计。

- ethtool_pause_stats()
- ethtool_fec_stats()
