
## PHY 链路拓扑（PHY link topology

## 概述


网络栈中PHY 链路拓扑表示旨在表示任何给定以太网链路的硬件布局
从用户空间的角度来看，一个以太网接口不过是一`struct net_device <net_device>`它通过传统ioctl ethtool netlink 命令暴露配置选项。基本的假设
```

  +-----------------------+        +----------+      +--------------+
  | Ethernet Controller / |        | Ethernet |      | Connector /  |
  |       MAC             | ------ |   PHY    | ---- |    Port      | ---... to LP
  +-----------------------+        +----------+      +--------------+
  struct net_device               struct phy_device

```
需要配PHY 的命令将通过 net_device.phydev 字段到达 PHY 并执行相关配置
当出现更复杂的拓扑时，这一假设就会失效，例如使SFP 收发器时
（尽管并非只有这一种特定情况）
这里我们有两种基本场景。要MAC 能够输出串行接口，可以直接馈SFP 笼（cage），
例如 SGMII000BaseX0GBaseR 等
```

  +-----+  SGMII  +------------+
  | MAC | ------- | SFP Module |
  +-----+         +------------+

```
```

  +-----+  SGMII   +--------------+
  | MAC | -------- | PHY (on SFP) |
  +-----+          +--------------+

```
在这种情况下，SFP PHY phylib 处理，并通过SFP 上游 ops phylink 注册
现在一些以太网控制器无法输出串行接口，因此我们不能直接将它们连接到 SFP 笼然而，一PHY 可以用作媒体转换器（media-converter），将非串行MAC MII 接口
转换```

  +-----+  RGMII  +-----------------------+  SGMII  +--------------+
  | MAC | ------- | PHY (media converter) | ------- | PHY (on SFP) |
  +-----+         +-----------------------+         +--------------+

```
这正是单一 net_device.phydev 指针模型显露其局限性的地方，因为现在链路上
有两PHY
phy_link 拓扑框架旨在提供一种方式来跟踪链路上的每个 PHY，供内核驱动和子系统
使用，同时也向用户空间报告拓扑，从而允许在配置命令中针对单PHY
## API


`struct phy_link_topology <phy_link_topology>` 是一per-netdevice
资源，在网络设备创建时初始化。一旦初始化，就可以通过
`phy_link_topo_add_phy` PHY 注册到拓扑中
除了PHY 注册到拓扑之外，该调用还会为 PHY 分配一个唯一索引，该索引随后
可以报告给用户空间以引用PHY（类似于 ifindex）。该索引是一u32，范围从
1 U32_MAX。0 被保留用于表PHY 尚不属于任何拓扑
然后可以通过 `phy_link_topo_del_phy` PHY 从拓扑中移除
这些函数已经挂接phylib 子系统中，因此所有通过 `phy_attach_direct` 链接net_device PHY 将自动加入该 netdev 的拓扑
位于 SFP 模块上的 PHY 也会SFP 上游phylink（即没有媒体转换器）自动注册
可用SFP 上游PHY 驱动需要调`phy_sfp_attach_phy` `phy_sfp_detach_phy`它们可以用作 `struct sfp_upstream_ops <sfp_upstream_ops>` .attach_phy / .detach_phy 实现
## UAPI


存在一netlink 命令用于从用户空间查询链路拓扑，请参`Documentation/networking/ethtool-netlink.rst`
拥有拓扑表示的全部意义在于为 `struct phy_device <phy_device>` 中的
phyindex 字段赋值。该索引使用 `ETHTOOL_MSG_PHY_GET` ethtnl 命令报告用户空间。执DUMP 操作将导致列出所net_device 的所PHY。DUMP 命令
接受 `ETHTOOL_A_HEADER_DEV_INDEX` `ETHTOOL_A_HEADER_DEV_NAME`
作为请求中传入的参数，以DUMP 过滤到单net_device
检索到的索引随后可以作为请求参数使`ETHTOOL_A_HEADER_PHY_INDEX` 字段
传入以下 ethnl 命令
- `ETHTOOL_MSG_STRSET_GET` 用于获取给定 PHY 的统计字符串- `ETHTOOL_MSG_CABLE_TEST_ACT` `ETHTOOL_MSG_CABLE_TEST_ACT`，用于在链路上的
  给定 PHY（最可能是最外层PHY）上执行电缆测试
- `ETHTOOL_MSG_PSE_SET` `ETHTOOL_MSG_PSE_GET` 用于 PHY 控制PoE PSE 设置
- `ETHTOOL_MSG_PLCA_GET_CFG`、`ETHTOOL_MSG_PLCA_SET_CFG`   `ETHTOOL_MSG_PLCA_GET_STATUS` 用于设置 PLCA（物理层冲突避免）参
注意，PHY 索引可以传递给其他请求，如果存在且不相关，它们会静默忽略它