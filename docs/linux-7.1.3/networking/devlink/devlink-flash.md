

## Devlink Flash


`devlink-flash` API 允许更新设备固件。它取代了较旧的 `ethtool-flash` 机制，并且不需要取下任何
```

  $ devlink dev flash pci/0000:05:00.0 file flash-boot.bin

```
注意，文件名是相对于固件加载路径（通常是 `/lib/firmware/`）的路径。驱动可能发送状态更新，以通知用户空间
更新操作的进度。

## Overwrite Mask（覆盖掩码）


`devlink-flash` 命令允许可选地指定一个掩码，指示设备在更新时应如何处理闪存组件的子段。此掩码指示允许
被覆盖的段集合。

   :widths: 5 95

   - - 名称
     - 描述
   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS`
     - 指示设备应使用所提供映像中的设置覆盖正在更新的组件中的设置。
   - - `DEVLINK_FLASH_OVERWRITE_IDENTIFIERS`
     - 指示设备应使用所提供映像中的标识符覆盖正在更新的组件中的标识符。这包括 MAC 地址、序列 ID 以及
       类似的设备标识符。

可以组合并一起请求多个覆盖位。如果未提供任何位，则期望设备只更新正在更新的组件中的固件二进制。设置和
标识符应跨更新被保留。设备可能不支持每种组合，此类设备的驱动必须拒绝任何无法忠实实现的组合。

## 固件加载


需要固件才能运行的设备通常将其存储在板上的非易失性存储器中，例如闪存。有些设备只在板上存储基本固件，
驱动在探测期间从磁盘加载其余部分。`devlink-info` 允许用户查询固件信息（已加载的组件和版本）。

在其他情况下，设备既可以将映像存储在板上、从磁盘加载，也可以自动从磁盘刷写新映像。`fw_load_policy`
devlink 参数可用于控制此行为
（Documentation/networking/devlink/devlink-params.rst <devlink_params_generic>）。

磁盘上的固件文件通常存储在 `/lib/firmware/`。

## 固件版本管理


期望驱动实现 `devlink-flash` 和 `devlink-info` 功能，它们共同允许实现与供应商无关的自动化固件更新设施。

`devlink-info` 暴露 `driver` 名称以及三个版本组（`fixed`、`running`、`stored`）。

`driver` 属性和 `fixed` 组标识特定的设备设计，例如用于查找适用的固件更新。这就是为什么 `serial_number`
不是 `fixed` 版本的一部分（即使它是固定的）——`fixed` 版本应标识设计，而非单个设备。

`running` 和 `stored` 固件版本标识设备上运行的固件，以及将在重启或设备重置后激活的固件。

固件更新代理应该能够遵循这个简单算法来更新固件内容，而与设备供应商无关：


  # 获取唯一的硬件设计标识符
  $hw_id = devlink-dev-info['fixed']

  # 查明我们想为此 NIC 使用哪个 FW 闪存
  $want_flash_vers = some-db-backed.lookup($hw_id, 'flash')

  # 必要时更新闪存
  if $want_flash_vers != devlink-dev-info['stored']:
      $file = some-db-backed.download($hw_id, 'flash')
      devlink-dev-flash($file)

  # 查明预期的整体固件版本
  $want_fw_vers = some-db-backed.lookup($hw_id, 'all')

  # 必要时更新磁盘上的文件
  if $want_fw_vers != devlink-dev-info['running']:
      $file = some-db-backed.download($hw_id, 'disk')
      write($file, '/lib/firmware/')

  # 尝试设备重置（如果可用）
  if $want_fw_vers != devlink-dev-info['running']:
     devlink-reset()

  # 重启（如果重置不够）
  if $want_fw_vers != devlink-dev-info['running']:
     reboot()

注意，此伪代码中每次对 `devlink-dev-info` 的引用都期望从内核获取最新信息。

为方便识别固件文件，一些供应商在固件版本中添加了 `bundle_id` 信息。此元版本覆盖多个逐组件版本，可用于
例如固件文件名中（所有组件版本可能会相当长）。
