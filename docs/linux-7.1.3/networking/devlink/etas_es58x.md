## etas_es58x devlink 支持


本文档描`etas_es58x` 设备驱动实现devlink 特性

## 信息版本（Info versions


`etas_es58x` 驱动报告以下版本

   :widths: 5 5 90

   - - 名称
     - 类型
     - 说明
   - - `fw`
     - running
     - 设备上运行的固件版本。也可通过 `ethtool -i` 作为 `firmware-version` 的第一个成员获取
   - - `fw.bootloader`
     - running
     - 设备上运行的 bootloader 版本。也可通过 `ethtool -i` 作为 `firmware-version` 的第二个成员获取
   - - `board.rev`
     - fixed
     - 设备的硬件版本
   - - `serial_number`
     - fixed
     - USB 序列号。也可通过 `lsusb -v` 获取
