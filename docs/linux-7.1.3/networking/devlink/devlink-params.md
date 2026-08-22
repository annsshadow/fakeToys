
## Devlink 参数


`devlink` 提供了让驱动暴露设备参数以控制底层设备功能的能力。由devlink 可以在设备级（device-wide）运作，因此它可以用于提供可能影响单个设备上多个端口的配置

本文档描述多个驱动共同支持的一些通用参数。每个驱动也可以自由添加自己的参数。每个驱动都必须记录它们所支持的具体参数，无论是否通用

## 配置模式


参数可以在不同的配置模式下设置

   :widths: 5 90

   - - Name
     - Description
   - - `runtime`
     - 在驱动运行时设置，立即生效。不需要复位
   - - `driverinit`
     - 在驱动初始化时应用。需要用户使`devlink` reload 命令重启驱动
   - - `permanent`
     - 写入设备的非易失性存储器。需要硬复位才能生效

### 重新加载


为了`driverinit` 参数生效，驱动必须支持通过 `devlink-reload` 命令重新加载。该命令会请求重新加载设备驱动

## 默认参数


驱动可以可选地导出 `runtime` `permanent` 模式参数的默认值。对`driverinit` 参数，驱动设置的最后一个值将用作默认值。驱动还可以支持`runtime` `permanent` 模式的参数重置为其默认值。重`driverinit` 参数devlink 核心支持，无需额外的驱动支持

## 通用配置参数


以下是驱动可以添加的通用配置参数列表。优先使用通用参数，而不是让每个驱动创建自己的名称

   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `enable_sriov`
     - Boolean
     - 在设备中启用单根 I/O 虚拟化（SRIOV）
   - - `ignore_ari`
     - Boolean
     - 忽略替代路由 ID 解释（ARI）能力。如果启用，即使平台已支ARI，适配器也会忽ARI 能力。设备将创建与平台不支持 ARI 时相同数量的分区
   - - `msix_vec_per_pf_max`
     - u32
     - 提供设备可以创建的最MSI-X 中断数量。该值对设备中所有物理功能（PF）相同
   - - `msix_vec_per_pf_min`
     - u32
     - 提供设备初始化所需的最MSI-X 中断数量。该值对设备中所有物理功能（PF）相同
   - - `fw_load_policy`
     - u8
     - 控制设备的固件加载策略
        - `DEVLINK_PARAM_FW_LOAD_POLICY_VALUE_DRIVER` (0)
          加载驱动偏好的固件版本
        - `DEVLINK_PARAM_FW_LOAD_POLICY_VALUE_FLASH` (1)
          加载当前存储在闪存中的固件
        - `DEVLINK_PARAM_FW_LOAD_POLICY_VALUE_DISK` (2)
          加载当前在主机磁盘上可用的固件
   - - `reset_dev_on_drv_probe`
     - u8
     - 控制驱动探测（probe）时设备的复位策略
        - `DEVLINK_PARAM_RESET_DEV_ON_DRV_PROBE_VALUE_UNKNOWN` (0)
          未知或无效值
        - `DEVLINK_PARAM_RESET_DEV_ON_DRV_PROBE_VALUE_ALWAYS` (1)
          驱动探测时总是复位设备
        - `DEVLINK_PARAM_RESET_DEV_ON_DRV_PROBE_VALUE_NEVER` (2)
          驱动探测时从不复位设备
        - `DEVLINK_PARAM_RESET_DEV_ON_DRV_PROBE_VALUE_DISK` (3)
          仅当能在文件系统中找到固件时才复位设备
   - - `enable_roce`
     - Boolean
     - 在设备中启用 RoCE 流量处理
   - - `enable_eth`
     - Boolean
     - 启用时，设备驱动将实例化devlink 设备的以太网特定辅助设备
   - - `enable_rdma`
     - Boolean
     - 启用时，设备驱动将实例化devlink 设备RDMA 特定辅助设备
   - - `enable_vnet`
     - Boolean
     - 启用时，设备驱动将实例化devlink 设备VDPA 网络特定辅助设备
   - - `enable_iwarp`
     - Boolean
     - 在设备中启用 iWARP 流量处理
   - - `internal_err_reset`
     - Boolean
     - 启用时，设备驱动将在内部错误时复位设备
   - - `max_macs`
     - u32
     - 通常 macvlan、vlan 网络设备mac 也会编程到其父网络设备的 Function rx 过滤器中。该参数限制每个以太网端口可从该设备接收流量的最大单mac 地址过滤器数量
   - - `region_snapshot_enable`
     - Boolean
     - 启用 `devlink-region` 快照的捕获
   - - `enable_remote_dev_reset`
     - Boolean
     - 启用由远程主机进行的设备复位。清除时，设备驱动将拒绝（NACK）其它主机复位设备的任何尝试。该参数对于设备被不同主机共享（如多主机设置）的场景很有用
   - - `io_eq_size`
     - u32
     - 控制 I/O 完成 EQ 的大小
   - - `event_eq_size`
     - u32
     - 控制异步控制事件 EQ 的大小
   - - `enable_phc`
     - Boolean
     - 在设备中启用 PHC（PTP 硬件时钟）功能
   - - `clock_id`
     - u64
     - 设备用于注册 DPLL 设备和引脚的时钟 ID
   - - `total_vfs`
     - u32
     - PF 暴露的虚拟功能（VF）的最大数量。在重启/PCI 复位后，设备 sysfs 目录下的 'sriov_totalvfs' 条目将报告此值
   - - `num_doorbells`
     - u32
     - 控制设备使用的门铃（doorbell）数量
   - - `max_mac_per_vf`
     - u32
     - 控制可以分配给虚拟功能（VF）的最MAC 地址过滤器数量
