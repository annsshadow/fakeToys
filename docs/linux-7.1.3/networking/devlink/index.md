## Linux Devlink 文档


devlink 是一API，用于暴露与任何设备类没有直接关系的设备信息与资源，例如芯片交换ASIC 级的配置
### 閿。

驱动面向API 目前正在过渡到允许更显式的加锁。驱动可以使用现有的 `devlink_*` 一API，或
`devl_*` 为前缀的新 API。较旧的 API devlink 核心中处理所有加锁，但不允许在主 devlink 对象
自身注册后注册大多数子对象。较新的 `devl_*` API 假定 devlink 实例锁已经被持有。驱动可以通过调用
`devl_lock()` 获取实例锁。在所devlink netlink 命令的回调中也会持有它
鼓励驱动为自己的需求使devlink 实例锁
驱动在同时获devlink 实例锁与获取 RTNL 锁时需要谨慎。需要先获取 devlink 实例锁，只有在此之后才能
获取 RTNL 锁
### 嵌套实例


某些对象，如线卡（linecard）或端口功能（port function），其下可能会创建另一devlink 实例。在那种
情况下，驱动应确保遵守以下规则：

 - 应保持加锁顺序。如果驱动需要同时获取嵌套实例与父母实例的实例锁，应先获取父母实例的 devlink 实例锁，
   然后才能获取嵌套实例的实例锁 - 驱动应使用对象特定的辅助函数来建立嵌套关系：

   - `devl_nested_devlink_set()` - 调用以建devlink -> 嵌套 devlink 关系（可用于多个嵌套实例）   - `devl_port_fn_devlink_set()` - 调用以建立端口功-> 嵌套 devlink 关系   - `devlink_linecard_nested_dl_set()` - 调用以建立线-> 嵌套 devlink 关系
嵌套 devlink 信息通过 devlink netlink 的对象特定属性暴露给用户空间
### 接口文档


以下页面一般地描述了通过 devlink 可用的各种接口
- [devlink-dpipe](devlink-dpipe)
- [devlink-eswitch-attr](devlink-eswitch-attr)
- [devlink-flash](devlink-flash)
- [devlink-health](devlink-health)
- [devlink-info](devlink-info)
- [devlink-linecard](devlink-linecard)
- [devlink-params](devlink-params)
- [devlink-port](devlink-port)
- [devlink-region](devlink-region)
- [devlink-reload](devlink-reload)
- [devlink-resource](devlink-resource)
- [devlink-selftests](devlink-selftests)
- [devlink-trap](devlink-trap)
- [devlink-shared](devlink-shared)

### 驱动专有文档


每个实现`devlink` 的驱动都应记录它支持的参数、信息版本以及其他特性
- [am65-nuss-cpsw-switch](am65-nuss-cpsw-switch)
- [bnxt](bnxt)
- [etas_es58x](etas_es58x)
- [hns3](hns3)
- [i40e](i40e)
- [ice](ice)
- [ionic](ionic)
- [iosm](iosm)
- [ixgbe](ixgbe)
- [kvaser_pciefd](kvaser_pciefd)
- [kvaser_usb](kvaser_usb)
- [mlx4](mlx4)
- [mlx5](mlx5)
- [mlxsw](mlxsw)
- [mv88e6xxx](mv88e6xxx)
- [netdevsim](netdevsim)
- [nfp](nfp)
- [octeontx2](octeontx2)
- [prestera](prestera)
- [qed](qed)
- [sfc](sfc)
- [stmmac](stmmac)
- [ti-cpsw-switch](ti-cpsw-switch)
- [zl3073x](zl3073x)
