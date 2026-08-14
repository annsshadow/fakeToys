
## 适用于 Linux 的 BusLogic MultiMaster 与 FlashPoint SCSI 驱动

			 Version 2.0.15 for Linux 2.0

			 Version 2.1.15 for Linux 2.1

			      PRODUCTION RELEASE

				17 August 1998

			       Leonard N. Zubkoff

			       Dandelion Digital

			       lnz@dandelion.com

	 Copyright 1995-1998 by Leonard N. Zubkoff <lnz@dandelion.com>


## 简介

BusLogic, Inc. 设计并制造了多种高性能 SCSI 主机适配器（host
adapter），它们借助 MultiMaster ASIC 技术，在种类繁多的总线架构上
共享一套通用的编程接口。BusLogic 于 1996 年 2 月被 Mylex Corporation
收购，但本驱动所支持的产品最初是以 BusLogic 名义推出的，因此该名称
被保留在源代码与文档中。

本驱动支持目前所有的 BusLogic MultiMaster 主机适配器，并且只需极少
甚至无需修改，就应能支持未来任何 MultiMaster 设计。较近时期，BusLogic
推出了 FlashPoint 主机适配器，它们成本更低，并且依赖主机 CPU，而非
板载处理器。尽管没有板载 CPU，FlashPoint 主机适配器仍表现非常出色，
命令延迟极低。BusLogic 近期向我提供了 FlashPoint Driver Developer's
Kit，其中包含 FlashPoint SCCB Manager 的文档与可自由再分发的源代码。
SCCB Manager 是一套运行在主机 CPU 上的代码库，执行的功能类似于
MultiMaster 主机适配器上的固件。得益于他们提供了 SCCB Manager，本驱动
现已同样支持 FlashPoint 主机适配器。

我为 Linux 编写这个全新 BusLogic 驱动的主要目标是：充分发挥 BusLogic
SCSI 主机适配器与现代 SCSI 外设所能达到的完整性能，并提供一个高度
健壮的驱动，可依赖其用于高性能、关键任务的场合。所有主要的性能特性
都可以从 Linux 内核命令行或模块初始化时配置，使各个安装能够针对其
特定需求调整驱动性能与错误恢复。

关于 BusLogic SCSI 主机适配器 Linux 支持的最新信息，以及本驱动的最新
发布版本和 BT-948/958/958D 的最新固件，将始终可以从我的 Linux 主页
URL "http://sourceforge.net/projects/dandelion/" 获取。

缺陷报告应通过电子邮件发送至 "lnz@dandelion.com"。请在缺陷报告中包含
驱动与 SCSI 子系统在启动时报告的完整配置信息，以及任何与 SCSI 操作
相关的后续系统消息，并详细描述你系统的硬件配置。

Mylex 是一家非常值得合作的公司，我向 Linux 社区大力推荐他们的产品。
1995 年 11 月，我有机会成为他们最新 MultiMaster 产品——BT-948 PCI
Ultra SCSI 主机适配器——的 beta 测试站点，随后在 1996 年 1 月又成为
BT-958 PCI Wide Ultra SCSI 主机适配器的测试站点。这是互利互惠的，因为
Mylex 获得了其自身测试团队难以轻易达成的一定程度的测试，而 Linux 社区
则得以拥有在上市前就已用 Linux 充分测试过的高性能主机适配器。这种关系
也让我有机会直接与他们的技术团队交流，更多地了解其产品的内部运作，并
反过来向他们说明 Linux 社区的需求与潜力。

较近时期，Mylex 重申了公司支持 Linux 社区的兴趣，而我目前正在为 DAC960
PCI RAID 控制器编写一个 Linux 驱动。Mylex 的兴趣与支持令人十分感激。

与某些其他厂商不同，如果你在使用 Linux 时联系 Mylex 技术支持寻求问题
帮助，他们不会告诉你使用其产品不受支持。他们最新的产品市场资料甚至
写明 "Mylex SCSI host adapters are compatible with all major operating
systems including: ... Linux ..."。

Mylex Corporation 位于 34551 Ardenwood Blvd., Fremont, California 94555,
USA，可通过 510/796-6100 联系，或在万维网上通过 http://www.mylex.com
联系。Mylex HBA 技术支持可通过电子邮件 techsup@mylex.com、语音 510/608-2400
或传真 510/745-7715 联系。欧洲与日本办事处的联系信息可在网站上获取。


## 驱动特性


### 配置报告与测试

  在系统初始化期间，驱动会广泛地报告主机适配器的硬件配置，包括与每个
  目标设备请求并协商的同步传输参数。会针对每个目标设备报告同步协商、
   Wide 协商以及断开/重连（Disconnect/Reconnect）的 AutoSCSI 设置，以及
  标记队列（Tagged Queuing）的状态。如果所有目标设备都采用同一设置，则
  使用一个单词或短语表示；否则，会为每个目标设备提供一个字母以表示其
  各自的状态。以下示例应能阐明这种报告格式：

    Synchronous Negotiation: Ultra

      已对所有目标设备启用同步协商，主机适配器将尝试协商 20.0 兆传输/秒。

    Synchronous Negotiation: Fast

      已对所有目标设备启用同步协商，主机适配器将尝试协商 10.0 兆传输/秒。

    Synchronous Negotiation: Slow

      已对所有目标设备启用同步协商，主机适配器将尝试协商 5.0 兆传输/秒。

    Synchronous Negotiation: Disabled

      已禁用同步协商，所有目标设备被限制为异步操作。

    Synchronous Negotiation: UFSNUUU#UUUUUUUU

      已对目标设备 0 以及 4 到 15 启用 Ultra 速度的同步协商，对目标设备 1
      启用 Fast 速度，对目标设备 2 启用 Slow 速度，且不允许目标设备 3 使用。
      主机适配器的 SCSI ID 由 "#" 表示。

    Wide 协商、断开/重连以及标记队列的状态会被报告为 "Enabled"、
    "Disabled"，或一串 "Y" 和 "N" 字母。

### 性能特性

  BusLogic SCSI 主机适配器直接实现了 SCSI-2 标记队列，因此驱动中包含了
  对报告具备标记队列能力的任何目标设备使用标记队列的支持。标记队列允许
  向每个目标设备或逻辑单元发出多个 outstanding 命令，并可显著提升 I/O
  性能。此外，使用了 BusLogic 的严格轮询（Strict Round Robin）模式来优化
  主机适配器性能，并且分散/聚集（scatter/gather）I/O 能够支持 Linux I/O
  子系统可有效利用的任意多个段。通过内核命令行或模块初始化时提供的驱动
  选项，可以控制每个目标设备对标记队列的使用，以及单独选择标记队列深度。
  默认情况下，队列深度会根据主机适配器的总队列深度以及所发现目标设备的
  数量、类型、速度和能力自动确定。此外，只要已知主机适配器固件版本未正确
  实现标记队列，或者一旦选择了队列深度为 1，标记队列就会自动禁用。对于
  已禁用断开/重连的目标设备，其标记队列也会被禁用。

### 健壮性特性

  驱动实现了广泛的错误恢复流程。当 SCSI 子系统较高层请求重置一个超时的
  命令时，会根据 SCSI 子系统的建议，在完整的主机适配器硬复位与 SCSI 总线
  复位之间，以及向各个目标设备发送总线设备复位消息之间进行选择。错误恢复
  策略可通过驱动选项为每个目标设备单独选择，也包括向与正在被重置的命令
  相关联的特定目标设备发送总线设备复位消息，以及完全抑制错误恢复以避免
  干扰运行不正常的设备。如果选择了总线设备复位错误恢复策略，而发送总线
  设备复位未能恢复正确操作，则下一个被重置的命令将强制进行一次完整的主机
  适配器硬复位与 SCSI 总线复位。由其他设备引起并被主机适配器检测到的 SCSI
  总线复位，也会通过向主机适配器发出软复位并重新初始化来处理。最后，如果
  标记队列处于活跃状态，且在 10 分钟间隔内发生了多次命令重置，或者如果在
  运行的前 10 分钟内发生了命令重置，则会禁用该目标设备的标记队列。这些
  错误恢复选项通过防止个别出错设备导致整个系统锁定或崩溃，从而提升整体
  系统的健壮性，并由此在移除违规部件后允许进行干净的关机与重启。

### PCI 配置支持

  在运行启用了 PCI BIOS 支持的内核的 PCI 系统上，本驱动将查询 PCI 配置
  空间，并使用由系统 BIOS 分配的 I/O 端口地址，而非 ISA 兼容的 I/O 端口
  地址。随后驱动会禁用 ISA 兼容的 I/O 端口。在 PCI 系统上，还建议使用
  AutoSCSI 工具完全禁用 ISA 兼容 I/O 端口，因为它并无必要。在 BT-948/958/958D
  上，ISA 兼容 I/O 端口默认是禁用的。

### /proc 文件系统支持

  主机适配器配置信息的副本，连同更新的数据传输与错误恢复统计信息，可
  通过 /proc/scsi/BusLogic/<N> 接口获取。

### 共享中断支持

  在支持共享中断的系统上，任意数量的 BusLogic 主机适配器可共享同一个
  中断请求通道。


## 受支持的主机适配器

以下列表包含截至本文档日期所支持的 BusLogic SCSI 主机适配器。建议任何
打算购买下列表中未列出的 BusLogic 主机适配器的人事先联系作者，以确认其
当前或将来的支持情况。

FlashPoint 系列 PCI 主机适配器：

=======================	=============================================
FlashPoint LT (BT-930)	Ultra SCSI-3
FlashPoint LT (BT-930R)	Ultra SCSI-3 with RAIDPlus
FlashPoint LT (BT-920)	Ultra SCSI-3 (BT-930 without BIOS)
FlashPoint DL (BT-932)	Dual Channel Ultra SCSI-3
FlashPoint DL (BT-932R)	Dual Channel Ultra SCSI-3 with RAIDPlus
FlashPoint LW (BT-950)	Wide Ultra SCSI-3
FlashPoint LW (BT-950R)	Wide Ultra SCSI-3 with RAIDPlus
FlashPoint DW (BT-952)	Dual Channel Wide Ultra SCSI-3
FlashPoint DW (BT-952R)	Dual Channel Wide Ultra SCSI-3 with RAIDPlus
=======================	=============================================

MultiMaster "W" 系列主机适配器：

=======     ===		==============================
BT-948	    PCI		Ultra SCSI-3
BT-958	    PCI		Wide Ultra SCSI-3
BT-958D	    PCI		Wide Differential Ultra SCSI-3
=======     ===		==============================

MultiMaster "C" 系列主机适配器：

========    ====	==============================
BT-946C	    PCI		Fast SCSI-2
BT-956C	    PCI		Wide Fast SCSI-2
BT-956CD    PCI		Wide Differential Fast SCSI-2
BT-445C	    VLB		Fast SCSI-2
BT-747C	    EISA	Fast SCSI-2
BT-757C	    EISA	Wide Fast SCSI-2
BT-757CD    EISA	Wide Differential Fast SCSI-2
========    ====	==============================

MultiMaster "S" 系列主机适配器：

=======     ====	==============================
BT-445S	    VLB		Fast SCSI-2
BT-747S	    EISA	Fast SCSI-2
BT-747D	    EISA	Differential Fast SCSI-2
BT-757S	    EISA	Wide Fast SCSI-2
BT-757D	    EISA	Wide Differential Fast SCSI-2
BT-742A	    EISA	SCSI-2 (742A revision H)
=======     ====	==============================

MultiMaster "A" 系列主机适配器：

=======     ====	==============================
BT-742A	    EISA	SCSI-2 (742A revisions A - G)
=======     ====	==============================

真正属于 BusLogic MultiMaster 克隆的 AMI FastDisk 主机适配器也受本驱动
支持。

BusLogic SCSI 主机适配器既有裸板形式，也有零售套装形式。上表中的 BT-
型号指的是裸板包装。零售套装的型号可通过将上表中的 BT- 替换为 KT- 得到。
零售套装包含裸板与手册，以及裸板所不提供的线缆、驱动介质与文档。


## FlashPoint 安装说明


### RAIDPlus 支持

  FlashPoint 主机适配器现在包含 RAIDPlus——Mylex 的可引导软件 RAID。
  RAIDPlus 在 Linux 上不受支持，也没有计划支持它。Linux 2.0 中的 MD 驱动
  提供串接（LINEAR）与条带化（RAID-0），而对镜像（RAID-1）、固定奇偶校验
  （RAID-4）和分布式奇偶校验（RAID-5）的支持可另行获取。内建的 Linux RAID
  支持通常更灵活，预计性能也会优于 RAIDPlus，因此将 RAIDPlus 支持纳入
  BusLogic 驱动的动机很小。

### 启用 UltraSCSI 传输

  FlashPoint 主机适配器出厂时配置为 "Factory Default"（出厂默认）设置，
  这些设置较为保守，不允许协商 UltraSCSI 速度。这样在将这些主机适配器
  安装到布线或终端电阻不足以支持 UltraSCSI 操作的系统中时，或现有 SCSI
  设备未正确响应 UltraSCSI 速度的同步传输协商时，可减少问题。可使用
  AutoSCSI 载入 "Optimum Performance"（最佳性能）设置，以允许与所有设备
  协商 UltraSCSI 速度，也可以逐个设备地启用 UltraSCSI 速度。建议在载入
  "Optimum Performance" 设置后手动禁用 SCAM。


## BT-948/958/958D 安装说明

BT-948/958/958D PCI Ultra SCSI 主机适配器有一些特性，在安装 Linux 时
在某些情况下可能需要留意。

### PCI I/O 端口分配

  配置为出厂默认设置时，BT-948/958/958D 只会识别由主板 PCI BIOS 做出的
  PCI I/O 端口分配。BT-948/958/958D 不会响应此前 BusLogic SCSI 主机适配器
  所响应的任何 ISA 兼容 I/O 端口。本驱动支持 PCI I/O 端口分配，因此这是
  首选配置。然而，如果由于某种原因必须使用已过时的 BusLogic 驱动（例如
  某个 Linux 发行版其引导内核尚未使用本驱动），BusLogic 提供了一个 AutoSCSI
  配置选项以启用一个传统的 ISA 兼容 I/O 端口。

  要启用这个向后兼容选项，可在系统启动时通过 Ctrl-B 调用 AutoSCSI 工具，
  选择 "Adapter Configuration"、"View/Modify Configuration"，然后将
  "ISA Compatible Port" 设置从 "Disable" 改为 "Primary" 或 "Alternate"。
  一旦本驱动安装完毕，应将 "ISA Compatible Port" 选项设回 "Disable"，以
  避免将来可能出现的 I/O 端口冲突。较老的 BT-946C/956C/956CD 也有此配置
  选项，但其出厂默认设置为 "Primary"。

### PCI 插槽扫描顺序

  在配有多个 BusLogic PCI 主机适配器的系统中，与 BT-946C/956C/956CD 相比，
  BT-948/958/958D 扫描 PCI 插槽的顺序可能看似相反。要使从 SCSI 磁盘引导
  正确工作，主机适配器的 BIOS 与内核必须就哪个磁盘是引导设备达成一致，
  这要求它们以相同顺序识别 PCI 主机适配器。主板 PCI BIOS 提供了一种枚举
  PCI 主机适配器的标准方式，Linux 内核就使用这种方式。某些 PCI BIOS 实现
  按总线号和设备号递增的顺序枚举 PCI 插槽，而另一些则按相反方向枚举。

  遗憾的是，Microsoft 决定 Windows 95 将始终按总线号和设备号递增的顺序
  枚举 PCI 插槽，而不管 PCI BIOS 的枚举顺序，并且要求主机适配器的 BIOS
  支持其方案以获得 Windows 95 认证。因此，BT-948/958/958D 的出厂默认设置
  按总线号和设备号递增的顺序枚举主机适配器。要禁用此特性，可在系统启动时
  通过 Ctrl-B 调用 AutoSCSI 工具，选择 "Adapter Configuration"、"View/Modify
  Configuration"，按 Ctrl-F10，然后将 "Use Bus And Device # For PCI Scanning
  Seq." 选项改为 OFF。

  本驱动将查询 PCI 扫描顺序（Scanning Sequence）选项的设置，以便以与主机
  适配器 BIOS 枚举相同的顺序识别主机适配器。

### 启用 UltraSCSI 传输

  BT-948/958/958D 出厂时配置为 "Factory Default"（出厂默认）设置，这些设置
  较为保守，不允许协商 UltraSCSI 速度。这样在将这些主机适配器安装到布线或
  终端电阻不足以支持 UltraSCSI 操作的系统中时，或现有 SCSI 设备未正确响应
  UltraSCSI 速度的同步传输协商时，可减少问题。可使用 AutoSCSI 载入
  "Optimum Performance"（最佳性能）设置，以允许与所有设备协商 UltraSCSI
  速度，也可以逐个设备地启用 UltraSCSI 速度。建议在载入 "Optimum Performance"
  设置后手动禁用 SCAM。


## 驱动选项

BusLogic 驱动选项可通过 Linux 内核命令行，或通过可加载内核模块安装工具
（Loadable Kernel Module Installation Facility）指定。多个主机适配器的
驱动选项可以通过分号分隔选项字符串来指定，也可以在命令行上指定多个
"BusLogic=" 字符串。单个主机适配器的各个选项规范以逗号分隔。探测与调试
选项适用于所有主机适配器，而其余选项仅单独适用于所选的主机适配器。

BusLogic 驱动的探测选项包含如下内容：

NoProbe

  "NoProbe" 选项禁用所有探测，因此不会检测到任何 BusLogic 主机适配器。

NoProbePCI

  "NoProbePCI" 选项禁用对 PCI 配置空间（PCI Configuration Space）的查询，
  因此只会检测到 ISA MultiMaster 主机适配器，以及 ISA 兼容 I/O 端口设为
  "Primary" 或 "Alternate" 的 PCI MultiMaster 主机适配器。

NoSortPCI

  "NoSortPCI" 选项强制 PCI MultiMaster 主机适配器按 PCI BIOS 提供的顺序
  枚举，忽略 AutoSCSI "Use Bus And Device # For PCI Scanning Seq." 选项的
  任何设置。

MultiMasterFirst

  "MultiMasterFirst" 选项强制先探测 MultiMaster 主机适配器，再探测 FlashPoint
  主机适配器。默认情况下，如果同时存在 FlashPoint 和 PCI MultiMaster 主机
  适配器，本驱动会先探测 FlashPoint 主机适配器，除非 BIOS 主磁盘由第一个
  PCI MultiMaster 主机适配器控制，在此情况下会先探测 MultiMaster 主机适配器。

FlashPointFirst

  "FlashPointFirst" 选项强制先探测 FlashPoint 主机适配器，再探测 MultiMaster
  主机适配器。

BusLogic 驱动的标记队列选项允许显式指定队列深度，以及是否为每个目标设备
（前提是该目标设备支持标记队列）允许标记队列。队列深度是允许同时提交执行
（无论是提交给主机适配器还是目标设备）的 SCSI 命令数量。请注意，显式启用
标记队列可能导致问题；启用或禁用标记队列的选项主要是为了让那些未正确实现
标记队列的目标设备能够禁用它。可用选项如下：

QueueDepth:<integer>

  "QueueDepth:" 或 "QD:" 选项指定用于所有支持标记队列的目标设备的队列深度，
  以及用于不支持标记队列的设备的队列深度上限。如果未提供队列深度选项，
  队列深度将根据主机适配器的总队列深度以及所检测到的目标设备的数量、类型、
  速度和能力自动确定。不支持标记队列的目标设备其队列深度始终被设为
  BusLogic_UntaggedQueueDepth 或 BusLogic_UntaggedQueueDepthBB，除非提供了
  更低的队列深度选项。队列深度为 1 会自动禁用标记队列。

QueueDepth:[<integer>,<integer>...]

  "QueueDepth:[...]" 或 "QD:[...]" 选项为每个目标设备单独指定队列深度。如果
  省略某个 <integer>，相应的目标设备将自动选择其队列深度。

TaggedQueuing:Default

  "TaggedQueuing:Default" 或 "TQ:Default" 选项根据 BusLogic 主机适配器的固件
  版本，以及队列深度是否允许排队多个命令，来决定是否允许标记队列。

TaggedQueuing:Enable

  "TaggedQueuing:Enable" 或 "TQ:Enable" 选项对本主机适配器上的所有目标设备
  启用标记队列，覆盖任何原本会基于主机适配器固件版本施加的限制。

TaggedQueuing:Disable

  "TaggedQueuing:Disable" 或 "TQ:Disable" 选项对本主机适配器上的所有目标设备
  禁用标记队列。

TaggedQueuing:<Target-Spec>

  "TaggedQueuing:<Target-Spec>" 或 "TQ:<Target-Spec>" 选项为每个目标设备单独
  控制标记队列。<Target-Spec> 是一串 "Y"、"N" 和 "X" 字符。"Y" 启用标记队列，
  "N" 禁用标记队列，"X" 接受基于固件版本的默认值。第一个字符指目标设备 0，
  第二个指目标设备 1，以此类推；如果 "Y"、"N"、"X" 字符序列未覆盖所有目标
  设备，未指定的字符假定为 "X"。

BusLogic 驱动的杂项选项包含如下内容：

BusSettleTime:<seconds>

  "BusSettleTime:" 或 "BST:" 选项以秒为单位指定总线稳定时间（Bus Settle
  Time）。总线稳定时间是指在上一次发起 SCSI 总线复位的主机适配器硬复位，与
  发出任何 SCSI 命令之间需要等待的时间量。如果未指定，默认为
  BusLogic_DefaultBusSettleTime。

InhibitTargetInquiry

  "InhibitTargetInquiry" 选项禁止在 MultiMaster 主机适配器上执行查询目标设备
  （Inquire Target Devices）或查询已安装设备（Inquire Installed Devices）
  命令。当某些较老的目标设备在寻址逻辑单元 0 以上时不做出正确响应时，这可能
  是必要的。

BusLogic 驱动的调试选项包含如下内容：

TraceProbe

  "TraceProbe" 选项启用对主机适配器探测的跟踪。

TraceHardwareReset

  "TraceHardwareReset" 选项启用对主机适配器硬件复位的跟踪。

TraceConfiguration

  "TraceConfiguration" 选项启用对主机适配器配置的跟踪。

TraceErrors

  "TraceErrors" 选项启用对从目标设备返回错误的 SCSI 命令的跟踪。对于每个
  失败的 SCSI 命令，将打印其 CDB 与 Sense Data。

Debug

  "Debug" 选项启用所有调试选项。

以下示例演示将第一个主机适配器上目标设备 1 和 2 的队列深度设为 7 和 15，
将第二个主机适配器上所有目标设备的队列深度设为 31，并将第二个主机适配器的
总线稳定时间设为 30 秒。

```

  linux BusLogic=QueueDepth:[,7,15];QueueDepth:31,BusSettleTime:30

```
```
  append = "BusLogic=QueueDepth:[,7,15];QueueDepth:31,BusSettleTime:30"

```
```
  insmod BusLogic.o \
      'BusLogic="QueueDepth:[,7,15];QueueDepth:31,BusSettleTime:30"'


```

      Module Utilities 2.1.71 或更高版本是正确解析包含逗号的驱动选项所必需的。


## 驱动安装

本发行版是为 Linux 内核版本 2.0.35 准备的，但应与 2.0.4 或任何更晚的 2.0
系列内核兼容。

要安装新的 BusLogic SCSI 驱动，你可以使用以下命令：
```

  cd /usr/src
  tar -xvzf BusLogic-2.0.15.tar.gz
  mv README.* LICENSE.* BusLogic.[ch] FlashPoint.c linux/drivers/scsi
  patch -p0 < BusLogic.patch (only for 2.0.33 and below)
  cd linux
  make config
  make zImage

```
然后将 "arch/x86/boot/zImage" 安装为你的标准内核，如适用则运行 lilo，并
重启。


## BusLogic 公告邮件列表

BusLogic 公告邮件列表提供了一个论坛，用于向 Linux 用户通告新的驱动发布以及
有关 BusLogic SCSI 主机适配器 Linux 支持的其他公告。要加入邮件列表，请发送
一封邮件到 "buslogic-announce-request@dandelion.com"，并在邮件正文中写上
"subscribe" 一行。
