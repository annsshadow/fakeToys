## qed 的 devlink 支持


本文档描述了 `qed` 核心设备驱动实现的 devlink 特性。

## 参数


`qed` 驱动实现了以下驱动特定的参数。

   :widths: 5 5 5 85

   - - 名称
     - 类型
     - 模式
     - 描述
   - - `iwarp_cmt`
     - Boolean
     - runtime
     - 为 100g 设备启用 iWARP 功能。请注意这会影响 L2 性能，因此默认不启用。
