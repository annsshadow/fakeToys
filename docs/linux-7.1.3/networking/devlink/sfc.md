
## sfc devlink 支持


本文档描`sfc` 设备驱动ef10 ef100 设备实现devlink 特性
## 信息版本


`sfc` 驱动报告以下版本

   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `fw.bundle_id`
     - stored
     - 上次用于更新多个组件的固件“bundle”镜像的版本   - - `fw.mgmt.suc`
     - running
     - 对于管理功能被拆分到多个控制单元的板卡，这是 SUC 控制单元的固件版本   - - `fw.mgmt.cmc`
     - running
     - 对于管理功能被拆分到多个控制单元的板卡，这是 CMC 控制单元的固件版本   - - `fpga.rev`
     - running
     - FPGA 设计修订版   - - `fpga.app`
     - running
     - 数据通路可编程逻辑版本   - - `fw.app`
     - running
     - 数据通路软件/微码/固件版本   - - `coproc.boot`
     - running
     - SmartNIC 应用协处理器（APU）第一阶段引导加载器版本   - - `coproc.uboot`
     - running
     - SmartNIC 应用协处理器（APU）协同操作系统加载器版本   - - `coproc.main`
     - running
     - SmartNIC 应用协处理器（APU）主操作系统版本   - - `coproc.recovery`
     - running
     - SmartNIC 应用协处理器（APU）恢复操作系统版本   - - `fw.exprom`
     - running
     - 扩展 ROM 版本。对于扩ROM 被拆分到多个镜像（如 PXE UEFI）的板卡，这专门PXE 引导 ROM 版本   - - `fw.uefi`
     - running
     - UEFI 驱动版本（无 UNDI 支持）
## 闪存更新


`sfc` 驱动实现对使`devlink-flash` 接口进行闪存更新的支持。它支持使用包含多个组件的组合闪存镜像（“bundle”）更新设备闪存（在 ef10 上，通常`fw.mgmt`、`fw.app`、`fw.exprom` `fw.uefi`）
该驱动不支持任何覆写掩码（overwrite mask）标志