
## ixgbe devlink support


本文档描述了 `ixgbe` 设备驱动实现devlink 特性
## Info versions


`devlink-info` 呈现的与安全性相关的任何版本都纯粹是信息性的。Devlink 不使用安全通道与设备通信
`ixgbe` 驱动报告以下版本

    :widths: 5 5 5 90

    - - Name
      - Type
      - Example
      - Description
    - - `board.id`
      - fixed
      - H49289-000
      - 板卡的产品板组件（PBA）标识符    - - `fw.undi`
      - running
      - 1.1937.0
      - 包含 UEFI 驱动Option ROM 版本。版本以 `major.minor.patch` 格式报告。主版本在任何重大不兼容变更发生时递增，或在次版本将溢出时递增。次版本在非破坏性变更时递增，并在主版本递增时重置为 1。补丁版本通常0，但当修复作为针对较旧基础 Option ROM 的补丁交付时递增    - - `fw.undi.srev`
      - running
      - 4
      - 指示 Option ROM 安全修订号的编号    - - `fw.bundle_id`
      - running
      - 0x80000d0d
      - 加载到设备上的固件映像文件的唯一标识符。也称为 NVM EETRACK 标识符    - - `fw.mgmt.api`
      - running
      - 1.5.1
      - 由管理固件通过 AdminQ 导出API 3 位版本号（major.minor.patch）。驱动用它来识别支持哪些命令。历史版本的内核只显2 位版本号（major.minor）    - - `fw.mgmt.build`
      - running
      - 0x305d955f
      - 管理固件来源的唯一标识符    - - `fw.mgmt.srev`
      - running
      - 3
      - 指示固件安全修订号的编号    - - `fw.psid.api`
      - running
      - 0.80
      - 定义闪存内容格式的版本    - - `fw.netlist`
      - running
      - 1.1.2000-6.7.0
      - netlist 模块的版本。该模块定义设备的以太网能力和默认设置，并被管理固件用作管理链路和设备连接的一部分    - - `fw.netlist.build`
      - running
      - 0xee16ced7
      - netlist 模块内容哈希的前 4 个字节
## Flash Update


`ixgbe` 驱动实现了使`devlink-flash` 接口的闪存更新支持。它支持使用包含 `fw.mgmt`、`fw.undi` `fw.netlist` 组件的合并闪存映像来更新设备闪存
   :widths: 5 95

   - - Bits
     - Behavior
   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS`
     - 不保留正在更新的闪存组件中存储的设置。这包括覆盖决定设备将初始化为多少物理功能的端口配置   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS` and `DEVLINK_FLASH_OVERWRITE_IDENTIFIERS`
     - 既不保留设置也不保留标识符。用所提供映像的内容覆盖闪存中的一切，不进行任何保留。这包括覆盖设备标识字段，例MAC 地址、重要产品数据（VPD）区域和设备序列号。此组合预期用于针对特定设备定制的映像
## Reload


`ixgbe` 驱动支持在闪存更新后使用带有 `DEVLINK_RELOAD_ACTION_FW_ACTIVATE` 动作`DEVLINK_CMD_RELOAD` 来激活新固件

    $ devlink dev reload pci/0000:01:00.0 reload action fw_activate

新固件通过发出设备特定的嵌入式管理处理器（Embedded Management Processor）重置来激活，该重置请求设备重置并重新加载 EMP 固件映像
驱动当前不支持通过 `DEVLINK_RELOAD_ACTION_DRIVER_REINIT` 重新加载驱动
## Regions


`ixgbe` 驱动实现了以下用于访问内部设备数据的区域
    :widths: 15 85

    - - Name
      - Description
    - - `nvm-flash`
      - 整个闪存芯片的内容，有时被称为设备的非易失性存储器（Non Volatile Memory）    - - `shadow-ram`
      - Shadow RAM 的内容，它从闪存开头加载。尽管内容主要来自闪存，但该区域还包含设备启动期间生成、未存储在闪存中的数据    - - `device-caps`
      - 设备固件能力缓冲区的内容。有助于确定设备的当前状态和配置
`nvm-flash` `shadow-ram` 区域都可以在不快照的情况下访问。`device-caps` 区域需要快照，因为内容由固件发送且无法拆分为单独的读取
用户可以通过 `DEVLINK_CMD_REGION_NEW` 命令请求立即为所有三个区域捕获快照

    $ devlink region show
    pci/0000:01:00.0/nvm-flash: size 10485760 snapshot [] max 1
    pci/0000:01:00.0/device-caps: size 4096 snapshot [] max 10

    $ devlink region new pci/0000:01:00.0/nvm-flash snapshot 1

    $ devlink region dump pci/0000:01:00.0/nvm-flash snapshot 1
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30
    0000000000000010 0000 0000 ffff ff04 0029 8c00 0028 8cc8
    0000000000000020 0016 0bb8 0016 1720 0000 0000 c00f 3ffc
    0000000000000030 bada cce5 bada cce5 bada cce5 bada cce5

    $ devlink region read pci/0000:01:00.0/nvm-flash snapshot 1 address 0 length 16
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30

    $ devlink region delete pci/0000:01:00.0/device-caps snapshot 1
