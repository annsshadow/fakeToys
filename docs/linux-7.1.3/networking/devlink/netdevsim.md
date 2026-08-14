## netdevsim devlink 支持


本文档描述了 `netdevsim` 设备驱动支持的 `devlink` 特性。

## 参数


   - - Name
     - Mode
   - - `max_macs`
     - driverinit

`netdevsim` 驱动还实现了以下驱动特定的参数。

   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `test1`
     - Boolean
     - driverinit
     - 用于展示驱动特定的 devlink 参数如何实现的测试参数。

`netdevsim` 驱动支持通过 `DEVLINK_CMD_RELOAD` 重新加载。

## 区域（Regions）


`netdevsim` 驱动导出一个 `dummy` 区域，作为 devlink-region 接口如何工作的示例。每当向
`take_snapshot` debugfs 文件写入时，就会获取一次快照。

## 资源


`netdevsim` 驱动导出资源以控制驱动将允许的 FIB 条目、FIB 规则条目和 nexthops 的数量。


    $ devlink resource set netdevsim/netdevsim0 path /IPv4/fib size 96
    $ devlink resource set netdevsim/netdevsim0 path /IPv4/fib-rules size 16
    $ devlink resource set netdevsim/netdevsim0 path /IPv6/fib size 64
    $ devlink resource set netdevsim/netdevsim0 path /IPv6/fib-rules size 16
    $ devlink resource set netdevsim/netdevsim0 path /nexthops size 16
    $ devlink dev reload netdevsim/netdevsim0

## 速率对象


`netdevsim` 驱动支持速率对象管理，包括：

- 为每个 VF devlink 端口注册/注销叶子（leaf）速率对象；
- 创建/删除节点速率对象；
- 为任意速率对象类型设置 tx_share 和 tx_max 速率值；
- 为任意速率对象类型设置父节点。

速率节点及其参数在 `netdevsim` debugfs 中以只读（RO）模式导出。例如创建的名为 `some_group`
的速率节点：


    $ ls /sys/kernel/debug/netdevsim/netdevsim0/rate_groups/some_group
    rate_parent  tx_max  tx_share

相同的参数在相应端口目录下为叶子对象导出。例如：


    $ ls /sys/kernel/debug/netdevsim/netdevsim0/ports/1
    dev  ethtool  rate_parent  tx_max  tx_share

## 驱动特定的 Traps


   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `fid_miss`
     - `exception`
     - 当数据包进入设备时，会基于入端口和 VLAN 将其分类到一个过滤标识符（FID）。
       该 trap 用于捕获无法找到 FID 的数据包
