
## mlxsw devlink 支持


本文档描述了 `mlxsw` 设备驱动实现devlink 特性
## 参数


   - - 名称
     - 模式
   - - `fw_load_policy`
     - driverinit

`mlxsw` 驱动还实现了以下驱动特定的参数
   :widths: 5 5 5 85

   - - 名称
     - 类型
     - 模式
     - 描述
   - - `acl_region_rehash_interval`
     - u32
     - runtime
     - 设置 ACL 区域定期重哈希（rehash）的间隔。以毫秒为单位，最小为
       `3000`。值为 `0` 表示完全禁用定期工作。第一次重哈希将在值被设置       立即运行
`mlxsw` 驱动支持通过 `DEVLINK_CMD_RELOAD` 重新加载

## 版本信息


`mlxsw` 驱动报告以下版本

   :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   - - `hw.revision`
     - fixed
     - 该板的硬件修订号
   - - `fw.psid`
     - fixed
     - 固件 PSID
   - - `fw.version`
     - running
     - 三位固件版本
## 线卡辅助设备版本信息


`mlxsw` 驱动为线卡辅助设备报告以下版
   :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   - - `hw.revision`
     - fixed
     - 该线卡的硬件修订   - - `ini.version`
     - running
     - 已加载的线卡 INI 版本
   - - `fw.psid`
     - fixed
     - 线卡设备 PSID
   - - `fw.version`
     - running
     - 线卡设备的三位固件版本号

## 驱动特定陷阱


   :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   - - `irif_disabled`
     - `drop`
     - 捕获设备决定丢弃的数据包，因为它们需要从已禁用的路由器接口（RIF       路由。这可能发生RIF 拆除期间，当 RIF 在被彻底移除前先被禁用时
   - - `erif_disabled`
     - `drop`
     - 捕获设备决定丢弃的数据包，因为它们需要通过已禁用的路由器接口（RIF       路由。这可能发生RIF 拆除期间，当 RIF 在被彻底移除前先被禁用时
