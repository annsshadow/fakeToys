## mv88e6xxx devlink 支持


本文档描述了 `mv88e6xxx` 设备驱动实现的 devlink 特性。

## Parameters


`mv88e6xxx` 驱动实现了以下驱动特有的参数。

   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `ATU_hash`
     - u8
     - runtime
     - 为地址转换单元（ATU）中的 MAC 地址选择四种可能哈希算法之一。当许多 MAC 地址具有相同 OUI 时，取值 3 可能比默认值 1 效果更好。该参数仅允许 0 到 3 的值。
