## octeontx2 devlink 支持


本文档描述了 `octeontx2 CPT` 设备驱动实现devlink 特性

## 参数


`octeontx2` 驱动实现了以下驱动特定的参数

   :widths: 5 5 5 85

   - - 名称
     - 类型
     - 模式
     - 描述
   - - `t106_mode`
     - u8
     - runtime
     - 用于CN10KA B0/CN10KB CPT 配置为以 CN10KA A0/A1 方式工作
