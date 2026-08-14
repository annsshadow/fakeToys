## Linux Switchtec 支持


Microsemi 的“Switchtec”系列 PCI 交换设备已经由内核通过标准 PCI 交换驱动支持。然而，Switchtec 设备通告了一个特殊的管理端点，用于启用一些额外功能。这包括：

- 数据包与字节计数器
- 固件升级
- 事件与错误日志
- 查询端口链路状态
- 自定义用户固件命令

switchtec 内核模块实现了这些功能。


## 接口


与 Switchtec 管理固件通信的主要方式是通过内存映射远程过程调用（MRPC）接口。
命令通过 4 字节的命令标识符和最多 1KB 的命令特定数据提交到该接口。固件将以 4 字节的返回码和最多 1KB 的命令特定数据作出响应。该接口一次只处理一个命令。


## 用户空间接口


MRPC 接口将通过简单的字符设备暴露给用户空间：/dev/switchtec#，系统中每个管理端点对应一个。

该字符设备具有以下语义：

- 一次写入必须至少包含 4 字节且不超过 1028 字节。前 4 字节将被解释为命令 ID，其余部分将用作输入数据。一次写入会将命令发送到固件以开始处理。

- 每次写入之后必须紧跟恰好一次读取。任何双重写入都会产生错误，任何不跟在写入之后的读取也会产生错误。

- 一次读取将阻塞，直到固件完成命令，并返回 4 字节的命令返回值以及最多 1024 字节的输出数据。（长度由读取调用的 size 参数指定——读取少于 4 字节会产生错误。）

- poll 调用也将被支持，用于需要在等待命令完成时做其他事情的用户空间应用程序。

该设备还支持以下 IOCTL：

- SWITCHTEC_IOCTL_FLASH_INFO - 检索固件长度和设备中的分区数量。

- SWITCHTEC_IOCTL_FLASH_PART_INFO - 检索闪存中任何指定分区的地址和长度。

- SWITCHTEC_IOCTL_EVENT_SUMMARY - 读取一个位图结构，指示所有未清除的事件。

- SWITCHTEC_IOCTL_EVENT_CTL - 获取当前计数、清除并设置任何事件的标志。该 ioctl 接收一个设置了 event_id、index 和 flags 的 switchtec_ioctl_event_ctl 结构（index 为分区或非全局事件的 PFF 编号）。它返回事件是否发生、发生的次数以及任何事件特定数据。这些标志可用于清除计数，或启用和禁用事件发生时采取的动作。通过使用 SWITCHTEC_IOCTL_EVENT_FLAG_EN_POLL 标志，你可以将某个事件设置为触发 poll 命令以 POLLPRI 返回。这样，用户空间可以等待事件发生。

- SWITCHTEC_IOCTL_PFF_TO_PORT 和 SWITCHTEC_IOCTL_PORT_TO_PFF 在 PCI Function Framework 编号（由事件系统使用）与 Switchtec 逻辑端口 ID 和分区编号（更友好）之间进行转换。


## 非透明桥（NTB）驱动


ntb_hw_switchtec 中为 Switchtec 硬件提供了一个 NTB 硬件驱动。目前，它仅支持配置了恰好 2 个 NT 分区以及零个或多个非 NT 分区的交换设备。它还要求以下配置设置：

- 两个 NT 分区必须能够访问彼此的 GAS 空间。因此，管理设置（Management Settings）下 GAS 访问向量中的位必须被设置以支持这一点。
- 内核配置必须包含对 NTB 的支持（需要设置 CONFIG_NTB）

NT EP BAR 2 将被动态配置为一个直接窗口（Direct Window），配置文件不需要显式配置它。

请参阅 Linux 源码树中的 Documentation/driver-api/ntb.rst 以从整体上了解 Linux NTB 栈。ntb_hw_switchtec 在此栈中作为 NTB 硬件驱动工作。
