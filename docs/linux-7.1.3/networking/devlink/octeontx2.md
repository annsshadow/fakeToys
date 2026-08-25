## octeontx2 devlink 支持


本文档描述了 `octeontx2 AF、PF VF` 设备驱动实现devlink 特性
## 参数


`octeontx2 PF VF` 驱动实现了以下驱动特定的参数
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `mcam_count`
     - u16
     - runtime
     - 选择为某个接口分配的匹配 CAM 条目数量       该数量同样用于该接口ntuple 过滤器。由 PF VF 驱动支持
`octeontx2 AF` 驱动实现了以下驱动特定的参数
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `dwrr_mtu`
     - u32
     - runtime
     - 用于设置硬件在传输队列之间调度时使用的量子（quantum）       硬件使用加权 DWRR 算法在所有传输队列之间进行调度   - - `npc_mcam_high_zone_percent`
     - u8
     - runtime
     - 用于设置用户可在 NPC MCAM 中分配的高优先级区域条目数量，从 high、mid low
       三个优先级区域类别中划分   - - `npc_def_rule_cntr`
     - bool
     - runtime
     - 用于启用或禁NPC MCAM 中默认规则的命中计数器       不能保证计数器会被启用并映射到所有默认规则，因为计数器稀缺，驱动采用尽力而为的方式       默认规则作为特定 PF VF 的主要数据包导向（steering）规则，基于其由 AF 驱动在初始化
       时安装的 DMAC 地址。从 debugfs 读取默认规则命中计数器的示例命令如下       cat /sys/kernel/debug/cn10k/npc/mcam_rules
   - - `nix_maxlf`
     - u16
     - runtime
     - 用于设置 NIX 硬件块中 LF 的最大数量。这有助于增加分配给已启LF（例MCAM 条目       的默认资源的可用性
`octeontx2 PF` 驱动实现了以下驱动特定的参数
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `unicast_filter_count`
     - u8
     - runtime
     - 设置可为该设备编程的单播过滤器的最大数量。这可用于实现更好的设备资源利用       避免过量消耗未使用MCAM 表条目