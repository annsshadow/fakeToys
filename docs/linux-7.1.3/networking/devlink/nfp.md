
## nfp devlink 支持


本文档描述 `nfp` 设备驱动实现的 devlink 特性。

## 参数


   - - 名称
     - 模式
   - - `fw_load_policy`
     - permanent
   - - `reset_dev_on_drv_probe`
     - permanent

## Info versions


`nfp` 驱动上报以下版本

   :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   - - `board.id`
     - fixed
     - 板卡设计标识符
   - - `board.rev`
     - fixed
     - 板卡设计修订号
   - - `board.manufacture`
     - fixed
     - 板卡设计厂商
   - - `board.model`
     - fixed
     - 板卡设计型号名称
   - - `board.part_number`
     - fixed
     - 板卡及其组件的部件号
   - - `fw.bundle_id`
     - stored, running
     - 固件 bundle id
   - - `fw.mgmt`
     - stored, running
     - 管理固件版本
   - - `fw.cpld`
     - stored, running
     - CPLD 固件组件版本
   - - `fw.app`
     - stored, running
     - APP 固件组件版本
   - - `fw.undi`
     - stored, running
     - UNDI 固件组件版本
   - - `fw.ncsi`
     - stored, running
     - NSCI 固件组件版本
   - - `chip.init`
     - stored, running
     - CFGR 固件组件版本
