## bnxt devlink 支持


本文档描`bnxt` 设备驱动实现devlink 特性
## 参数


   - - 名称
     - 模式
   - - `enable_sriov`
     - Permanent
   - - `ignore_ari`
     - Permanent
   - - `msix_vec_per_pf_max`
     - Permanent
   - - `msix_vec_per_pf_min`
     - Permanent
   - - `enable_remote_dev_reset`
     - Runtime
   - - `enable_roce`
     - Permanent

`bnxt` 驱动还实现了以下驱动专用参数
   :widths: 5 5 5 85

   - - 名称
     - 类型
     - 模式
     - 描述
   - - `gre_ver_check`
     - Boolean
     - Permanent
     - 将在设备中启用通用路由封装（GRE）版本检查。若禁用，设备将       入站数据包跳过版本检查
## 信息版本


`bnxt_en` 驱动报告以下版本

      :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   - - `board.id`
     - fixed
     - 标识板卡设计的部件号
   - - `asic.id`
     - fixed
     - ASIC 设计标识   - - `asic.rev`
     - fixed
     - ASIC 设计修订版本
   - - `fw.psid`
     - stored, running
     - 板卡的固件参数集版本
   - - `fw`
     - stored, running
     - 整体板卡固件版本
   - - `fw.mgmt`
     - stored, running
     - NIC 硬件资源管理固件版本
   - - `fw.mgmt.api`
     - running
     - 驱动与固件之间支持的最低固件接口规范版   - - `fw.nsci`
     - stored, running
     - 通用平台管理固件版本
   - - `fw.roce`
     - stored, running
     - RoCE 管理固件版本
