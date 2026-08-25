## kvaser_usb devlink 支持


本文档描述了 `kvaser_usb` 设备驱动实现devlink 特性

## Info versions


`kvaser_usb` 驱动报告以下版本

   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `fw`
     - running
     - 设备上运行的固件版本。也可通过 `ethtool -i` `firmware-version` 获取
   - - `board.rev`
     - fixed
     - 设备硬件修订版本
   - - `board.id`
     - fixed
     - 设备EAN（产品编号）
   - - `serial_number`
     - fixed
     - 设备序列号
