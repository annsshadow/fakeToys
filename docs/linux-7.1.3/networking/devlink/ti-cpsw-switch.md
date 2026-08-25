
## ti-cpsw-switch devlink 支持


本文档描述了 `ti-cpsw-switch` 设备驱动实现devlink 特性

## 参数


`ti-cpsw-switch` 驱动实现了以下驱动特定的参数

   :widths: 5 5 5 85

   - - 名称
     - 类型
     - 模式
     - 描述
   - - `ale_bypass`
     - Boolean
     - runtime
     - 启用 ALE_CONTROL(4).BYPASS 模式用于调试。在此模式下，所有数据包将仅
       发送至主机端口
   - - `switch_mode`
     - Boolean
     - runtime
     - 启用交换模式
