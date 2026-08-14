## MMC 工具简介


有一个名为 mmc-utils 的 MMC 测试工具，由 Ulf Hansson 维护，你可以在以下公共 git 仓库中找到它：

	https://git.kernel.org/pub/scm/utils/mmc/mmc-utils.git

## 功能


mmc-utils 工具可以完成以下操作：

 - 打印并解析 extcsd 数据。
 - 确定 eMMC 写保护（writeprotect）状态。
 - 设置 eMMC 写保护状态。
 - 通过禁用仿真（emulation）将 eMMC 数据扇区大小设置为 4KB。
 - 创建通用分区（general purpose partition）。
 - 启用增强用户区（enhanced user area）。
 - 按分区启用写可靠性（write reliability）。
 - 打印对 STATUS_SEND（CMD13）的响应。
 - 启用启动分区（boot partition）。
 - 设置启动总线条件（Boot Bus Conditions）。
 - 启用 eMMC BKOPS 功能。
 - 永久启用 eMMC 硬件复位（H/W Reset）功能。
 - 永久禁用 eMMC 硬件复位功能。
 - 发送 Sanitize 命令。
 - 为设备编程认证密钥（authentication key）。
 - 将 rpmb 设备的计数值读取到 stdout。
 - 从 rpmb 设备读取到输出。
 - 从数据文件写入 rpmb 设备。
 - 启用 eMMC 缓存（cache）功能。
 - 禁用 eMMC 缓存功能。
 - 打印并解析 CID 数据。
 - 打印并解析 CSD 数据。
 - 打印并解析 SCR 数据。
