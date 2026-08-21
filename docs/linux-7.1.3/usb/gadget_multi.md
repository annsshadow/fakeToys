## 多功能复Gadget

## 概述

多功能复Gadget（即 g_multi）是一个复gadget，它大量使用复合框架来提供……一个多功能 gadget

在标准配置下，它提供一个单独的 USB 配置，包RNDIS[^1^]（即以太网）、USB CDC[^2^] ACM（即串口）以USB 大容量存储功能

可通过 Kconfig 选项开启一CDC ECM（以太网）功能，并关RNDIS。如果两者都启用，该 gadget 将具有两个配置——一个带 RNDIS，另一个带 CDC ECM[^3^]

请注意，如果你使用非标准配置（即启用 CDC ECM），可能需要更改厂商和/或产ID

## 主机驱动

要使用这gadget，需要让它在主机侧也能工作——否则这gadget 就毫无用处。正如人们所料，在不同系统上需要做的事情各不相同

### Linux 主机驱动

由于gadget 使用标准复合框架，在 Linux 主机看来就是一个复合设备，因此 Linux 主机侧无需任何额外的驱动。所有功能都由为它们各自开发的相应驱动来处理

对于RNDIS 的双配置设置（RNDIS 配置作为第一个）也是如此。Linux 主机会使用第二个CDC ECM 的配置，该配置在 Linux 下应该表现更好

### Windows 主机驱动

要让gadget Windows 下工作，必须满足两个条件

##### 作为复合 gadget 被识

首先，Windows 需要将gadget 识别USB 复合 gadget，而这一点本身有一些条件[^4^]。如果条件满足，Windows 会让 USB 通用父驱动[^5^]来处理该设备，然后尝试为每个独立的接口匹配驱动（大致如此，这里就不展开太多细节了）

好消息是：大部分条件你都不用操心

唯一需要操心的是，gadget 必须只有一个配置，所以双 RNDIS CDC ECM gadget 无法工作，除非你创建一个合适的 INF——当然，前提是你确实提交了它

##### 为每个功能安装驱

另一件更棘手的事，是Windows 为每个独立的功能安装驱动

对于大容量存储来说很简单，因为 Windows 识别出它是一个实现了 USB 大容量存储类的接口，于是选择相应的驱动

RNDIS CDC ACM 则更麻烦

RNDIS
.....

要让 Windows gadget 中的第一个功能选择 RNDIS 驱动，需要使用本文档提供[[file:linux.inf]] 文件。它会把 Windows RNDIS 驱动“绑定”到 gadget 的第一个接口

请注意，在测试时我们发现RNDIS 不是第一个接口时会出现一些问题[^6^]。除非你正在尝试开发自己的 gadget，否则不必为此担心；如果是开发自己的 gadget，则要留意这bug

CDC ACM
.......

类似地，提供[[file:linux-cdc-acm.inf]] 用于 CDC ACM

自定gadget
............

如果你打算修g_multi gadget，请注意：重新排列功能显然会改变每个功能对应的接口号。这样一来，所提供INF 将不再生效，因为其中硬编码了接口号（不过改起来并不难[^7^]）

这也意味着，在试验 g_multi 并更改所提供的功能之后，应当更改 gadget 的厂商和/或产ID，以免与其他定制gadget 或原gadget 发生冲突

如果不这样做，你可能会“脑损伤”——在花了数小时纳闷为什么一切都不按预期工作之后，才意识Windows 缓存了某些驱动信息（换个 USB 口有时会有帮助，你也可以试试USBDeview[^8^] 来移除那个幽灵设备）

INF 测试
.......

所提供INF 文件已经Windows XP SP3、Windows Vista Windows 7 上测试过，均32 位版本。它64 位版本上也应该可以工作。在 Windows XP SP2 之前的版本上多半无法工作

### 其他系统

目前尚未对其他任何系统的驱动进行测试。鉴MacOS 基于 BSD，BSD 是开源的，相信它应该（读作：“我完全不知道它到底行不行”）开箱即用

对于更冷门的系统，我更是无从说起…

非常欢迎任何测试和驱动贡献！

## 作

本文档由 Michal Nazarewicz（[[mailto:mina86@mina86.com]]）编写。INF 文件Marek Szyprowski（[[mailto:m.szyprowski@samsung.com]]）和 Xiaofan Chen（[[mailto:xiaofanc@gmail.com]]）的帮助下完成，基于 MS RNDIS 模板[^9^]、Microchip CDC ACM INF 文件以及 David Brownell（[[mailto:dbrownell@users.sourceforge.net]]）的原始 INF 文件

## 脚注

[^1^] 远程网络驱动接口规范（Remote Network Driver Interface Specification），[[https://msdn.microsoft.com/en-us/library/ee484414.aspx]]

[^2^] 通信设备类抽象控制模型（Communications Device Class Abstract Control Model），该类及其USB 类的规范可参[[http://www.usb.org/developers/devclass_docs/]]

[^3^] CDC 以太网控制模型（CDC Ethernet Control Model）

[^4^] [[https://msdn.microsoft.com/en-us/library/ff537109(v=VS.85).aspx]]

[^5^] [[https://msdn.microsoft.com/en-us/library/ff539234(v=VS.85).aspx]]

[^6^] 换句好听点的话说，就Windows 对用户的任何输入都没有响应

[^7^] 你可能会发现 [[http://www.cygnal.org/ubb/Forum9/HTML/001050.html]] 有用

[^8^] https://www.nirsoft.net/utils/usb_devices_view.html

[^9^] [[https://msdn.microsoft.com/en-us/library/ff570620.aspx]]
