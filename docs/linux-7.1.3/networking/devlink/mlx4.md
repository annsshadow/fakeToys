
## mlx4 devlink 支持


本文档描述 `mlx4` 设备驱动实现的 devlink 特性。

## 参数


   - - 名称
     - 模式
   - - `internal_err_reset`
     - driverinit, runtime
   - - `max_macs`
     - driverinit
   - - `region_snapshot_enable`
     - driverinit, runtime

`mlx4` 驱动还实现以下驱动特定的参数。

   :widths: 5 5 5 85

   - - 名称
     - 类型
     - 模式
     - 描述
   - - `enable_64b_cqe_eqe`
     - Boolean
     - driverinit
     - 如果 FW 支持，启用 64 字节 CQEs/EQEs。
   - - `enable_4k_uar`
     - Boolean
     - driverinit
     - 启用使用 4k UAR。

`mlx4` 驱动支持通过 `DEVLINK_CMD_RELOAD` 重新加载。

## 区域


`mlx4` 驱动支持在出现严重固件问题时转储固件 PCI crspace 与健康缓冲区。

如果固件命令超时、固件卡住，或 catastrophic 缓冲区出现非零值，驱动将拍摄快照。

`cr-space` 区域将包含固件 PCI crspace 内容。`fw-health` 区域将包含设备固件的
健康缓冲区。这两个区域的快照都在相同的事件触发时拍摄。
