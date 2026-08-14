## Intel IXP4xx 网络处理器上的 Linux 发行说明


### 维护者：Deepak Saxena <dsaxena@plexity.net>


1. 概述

Intel 的 IXP4xx 网络处理器是一款高度集成的 SoC，面向网络应用，但由于其低成本与低
功耗，在工业控制及其他领域也颇受欢迎。IXP4xx 系列目前包含若干支持不同网络卸载功能
（如加密、路由、防火墙等）的处理器。IXP46x 系列是一个更新版本，支持更高的速度、新的
内存与闪存配置，以及更高的集成度（例如片内 I2C 控制器）。

关于该 CPU 各个版本的更多信息，请参阅：

   http://developer.intel.com/design/network/products/npfamily/ixp4xx.htm

Intel 还曾生产过 IXCP1100 CPU，它是去除了大量网络智能的 IXP4xx。

2. Linux 支持

Linux 目前在 IXP4xx 芯片上支持以下特性：

- 双串口
- PCI 接口
- 闪存访问（MTD/JFFS）
- IXP42x 上通过 GPIO 实现的 I2C
- 用于输入/输出/中断的 GPIO
  访问函数参见 arch/arm/mach-ixp4xx/include/mach/platform.h。
- 定时器（看门狗、操作系统）

以下芯片组件不受 Linux 支持，需要使用 Intel 专有的 CSR 软件：

- USB 设备接口
- 网络接口（HSS、Utopia、NPE 等）
- 网络卸载功能

如果你需要使用上述任何功能，需要从以下地址下载 Intel 的软件：

   http://developer.intel.com/design/network/products/npfamily/ixp425.htm

请勿就专有软件向 Linux 邮件列表提问。

有多个网站提供使用 Intel 软件的指引/线索：

   - http://sourceforge.net/projects/ixp4xx-osdg/
    使用 uClinux 与 Intel 库的开源开发者指南

   - http://gatewaymaker.sourceforge.net/
    使用 IXP425 与 Linux 构建网关的简单单页摘要

   - http://ixp425.sourceforge.net/
    依赖 Intel 库的 IXP425 ATM 设备驱动

3. 已知问题/限制

3a. 入站 PCI 窗口有限

IXP4xx 系列最多支持 256MB 内存，但 PCI 接口只能将其中 64MB 暴露给 PCI 总线。这意味着
如果你运行的内存大于 64MB，所有超出可访问范围的 PCI 缓冲区都将通过
arch/arm/common/dmabounce.c 中的例程进行反弹（bounce）。

3b. 出站 PCI 窗口有限

IXP4xx 提供两种访问 PCI 内存空间的方法：

1) 从 0x48000000 到 0x4bffffff 的直接映射窗口（64MB）。
   要通过此空间访问 PCI，我们只需将 BAR 使用 ioremap() 映射到内核中，即可使用
   标准的 read[bwl]/write[bwl] 宏。由于速度原因这是首选方法，但它将系统限制为仅
   64MB 的 PCI 内存。在使用显卡及其他高内存占用设备时，这可能会成为问题。

2) 如果需要大于 64MB 的内存空间，可将 IXP4xx 配置为使用间接寄存器来访问 PCI。这允许
   总线上最多 128MB（0x48000000 到 0x4fffffff）的内存。其缺点在于每次 PCI 访问都需要
   三次本地寄存器访问外加一把自旋锁，但在某些情况下性能损失是可以接受的。此外，由于
   PCI 窗口的间接特性，这种情况下无法对 PCI 设备进行 mmap()。

默认情况下，出于性能考虑使用直接方法。如果你需要更多 PCI 内存，请启用
IXP4XX_INDIRECT_PCI 配置选项。

3c. GPIO 作为中断

目前代码仅处理电平敏感（level-sensitive）的 GPIO 中断。

4. 支持的平台

ADI Engineering Coyote 网关参考平台
http://www.adiengineering.com/productsCoyote.html

   ADI Coyote 平台是为构建小型住宅/办公网关者提供的参考设计。一个 NPE 连接到
   10/100 接口，一个连接到 4 端口 10/100 交换机，第三个连接到 ADSL 接口。此外，
   它还支持通过 SLIC 连接的 POTs 接口。请注意这些不受 Linux ATM 支持。最后，该平台
   有两个用于 802.11[bga] 卡的 mini-PCI 插槽。此外，扩展总线上挂有一个 IDE 端口。

Gateworks Avila 网络平台
http://www.gateworks.com/support/overview.php

   Avila 平台基本上就是 IXDP425，只是将 4 个 PCI 插槽替换为 mini-PCI 插槽，并在
   扩展总线上挂了一个 CF IDE 接口。

Intel IXDP425 开发平台
http://www.intel.com/design/network/products/npfamily/ixdpg425.htm

   这是 Intel 针对 IXDP425 的标准参考平台，也被称为 Richfield 板。它包含 4 个
   PCI 插槽、16MB 闪存、两个 10/100 端口以及一个 ADSL 端口。

Intel IXDP465 开发平台
http://www.intel.com/design/network/products/npfamily/ixdp465.htm

   这基本上是带有 IXP465 以及 32MB 闪存（而非仅 16MB）的 IXDP425。

Intel IXDPG425 开发平台

   这基本上是带有新增 NEC EHCI 控制器的 ADI Coyote 板。该板的一个问题是 mini-PCI
   插槽仅连接了 3.3v 供电线，因此你无法使用带有 E100 卡的 PCI 转 mini-PCI 适配器。
   因此，为了以 NFS 作为根文件系统，你需要使用 CSR 或一张 WiFi 卡，以及一个执行
   BOOTP 然后 pivot_root 到 NFS 的 ramdisk。

Motorola PrPMC1100 处理器夹层卡
http://www.fountainsys.com

   PrPMC1100 基于 IXCP1100，用于插入 IXP2400/2800 系统以充当系统控制器。它板上仅
   包含一个 CPU 和 16MB 闪存，需要插入载板才能工作。目前 Linux 仅支持该平台的
   Motorola PrPMC 载板。

5. 待办列表

- 添加对 Coyote IDE 的支持
- 添加对边沿触发（edge-based）GPIO 中断的支持
- 添加对扩展总线上 CF IDE 的支持

6. 致谢

IXP4xx 的工作由 Intel Corp. 与 MontaVista Software, Inc. 资助。

以下人士提供了补丁/评论等：

- Lennerty Buytenhek
- Lutz Jaenicke
- Justin Mayfield
- Robert E. Ranslam

[我知道我遗漏了其他人，请发邮件给我以便补充]

-------------------------------------------------------------------------

最近更新：01/04/2005
