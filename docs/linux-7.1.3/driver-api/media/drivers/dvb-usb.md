
## dvb-usb 框架背后的理念


   #) 本文档已经过时。请查阅 DVB wiki（位于 https://linuxtv.org/wiki）以获取更新的信息。

   #) **已废弃：** 较新的 DVB USB 驱动应当使用 dvb-usb-v2 框架。

2005 年 3 月，我拿到了新的 Twinhan USB2.0 DVB-T 设备。他们提供了规格说明和固件。

我非常急切地想要把这个驱动（当然带一些怪异之处）放进 dibusb。在读了些规格说明、做了
一些 USB 嗅探之后，我意识到，那样做的话 dibusb 驱动之后会变得一团糟。所以我决定用
一种不同的方式来做：借助一个 dvb-usb 框架。

该框架提供通用函数（大多是内核 API 调用），例如：

- 与 dvb-demux-feed-control 配合的传输流（Transport Stream）URB 处理
  （支持 bulk 和 isoc）
- 为设备注册 DVB-API
- 在适用时注册一个 I2C 适配器
- 遥控器/输入设备处理
- 固件请求与加载（目前仅针对 Cypress USB 控制器）
- 其他可以被多个驱动共享的函数/方法（例如用于 bulk 控制命令的函数）
- TODO：一个 I2C 分块器（chunker）。它根据寄存器长度和可多写、多读的数值个数，创建
  设备特定的寄存器访问块。

特定 DVB USB 设备的源代码只负责通过总线与设备进行通信。与 DVB-API 功能之间的连接
是通过回调完成的，这些回调在每个设备驱动都必须拥有的静态设备描述（struct
dvb_usb_device）中赋值。

作为示例，可以查看 drivers/media/usb/dvb-usb/vp7045*。

目标是把所有 usb 设备（dibusb、cinergyT2，也许还有 ttusb；flexcop-usb 已经受益于
通用的 flexcop 设备）迁移到使用 dvb-usb-lib。

TODO：根据所请求的 feed 数量，动态启用和禁用 pid 过滤器。

### 受支持的设备


关于网卡/驱动/固件的完整列表，请参阅 LinuxTV DVB Wiki，位于 https://linuxtv.org：
https://linuxtv.org/wiki/index.php/DVB_USB

0. 历史与新闻：

  2005-06-30

  - 新增对 WideView WT-220U 的支持（感谢 Steve Chang）

  2005-05-30

  - 为 dvb-usb 框架添加了基本的等时（isochronous）支持
  - 新增对 Conexant 混合参考设计和 Nebula DigiTV USB 的支持

  2005-04-17

  - 所有 dibusb 设备已被移植以使用 dvb-usb 框架

  2005-04-02

  - 重新启用并改进了遥控器代码。

  2005-03-31

  - 将 Yakumo/Hama/Typhoon DVB-T USB2.0 设备移植到 dvb-usb。

  2005-03-30

  - 基于 dibusb 源码的 dvb-usb 模块首次提交。
    第一个设备是针对 TwinhanDTV Alpha / MagicBox II 仅 USB2.0 的 DVB-T 设备的新驱动。
  - （从 dvb-dibusb 变更为 dvb-usb）

  2005-03-28

  - 新增对 AVerMedia AverTV DVB-T USB2.0 设备的支持
    （感谢 Glen Harris 和 Jiun-Kuei Jung，AVerMedia）

  2005-03-14

  - 新增对 Typhoon/Yakumo/HAMA DVB-T mobile USB2.0 的支持

  2005-02-11

  - 新增对 KWorld/ADSTech Instant DVB-T USB2.0 的支持。
    非常感谢 Joachim von Caron

  2005-02-02
  - 新增对 Hauppauge Win-TV Nova-T USB2 的支持

  2005-01-31
  - USB1.1 设备的失真流问题已解决

  2005-01-13

  - 把镜像的 pid_filter_table 移回 dvb-dibusb
    第一个几乎可用的 HanfTek UMT-010 版本
    发现 Yakumo/HAMA/Typhoon 是 HanfTek UMT-010 的前身

  2005-01-10

  - 重构完成，现在一切都令人非常满意

  - 一些怪异设备的调谐器怪癖（Artec T1 AN2235 设备有时会装配 Panasonic 调谐器）。
    实现了调谐器探测。非常感谢 Gunnar Wittich。

  2004-12-29

  - 经过几天与“URB 不返回”这个 bug 的搏斗，终于修复。

  2004-12-26

  - 重构了 dibusb 驱动，拆分为独立的文件
  - 启用了 i2c 探测

  2004-12-06

  - 增加了对 demod i2c 地址探测的可能性
  - 新的 USB ID（Compro、Artec）

  2004-11-23

  - 合并了来自 DiB3000MC_ver2.1 的更改
  - 修订了调试
  - 可以为 USB2.0 提供完整的 TS

  2004-11-21

  - dib3000mc/p 前端驱动的第一个可用版本。

  2004-11-12

  - 增加了额外的遥控器按键。感谢 Uwe Hanke。

  2004-11-07

  - 增加了遥控器支持。感谢 David Matthews。

  2004-11-05

  - 新增对一个新设备的支持（Grandtec/Avermedia/Artec）
  - 把我的更改（针对 dib3000mb/dibusb）合并到 FE_REFACTORING，因为它成了 HEAD
  - 把传输控制（pid 过滤器、fifo 控制）从 usb 驱动移到了前端，看起来放在那里更好
    （增加了 xfer_ops 结构体）
  - 为前端（mc/p/mb）创建了公共文件

  2004-09-28

  - 新增对一个新设备的支持（Unknown，vendor ID 是 Hyper-Paltek）

  2004-09-20

  - 新增对一个新设备的支持（Compro DVB-U2000），感谢 Amaury Demol 的报告
  - 改变了 usb TS 传输方式（多个 urb，在设置新的 pid 之前停止传输）

  2004-09-13

  - 新增对一个新设备的支持（Artec T1 USB TVBOX），感谢 Christian Motschke 的报告

  2004-09-05

  - 发布了 dibusb 设备和 dib3000mb 前端驱动
    （vp7041.c 的旧消息）

  2004-07-15

  - 偶然发现，该设备为 PLL 使用了 TUA6010XS

  2004-07-12

  - 弄清楚该驱动也应当能配合 CTS Portable（中华电视系统）工作

  2004-07-08

  - 固件提取 2.422 问题已解决，驱动现在能正确使用从 2.422 提取的固件正常工作
  - 针对 2.6.4（dvb）的 #if，编译问题
  - 改变了固件处理方式，见 vp7041.txt 第 1.1 节

  2004-07-02

  - 一些调谐器修改，v0.1，清理，首次公开

  2004-06-28

  - 现在使用 dvb_dmx_swfilter_packets，一切都运行良好

  2004-06-27

  - 能够观看并切换频道（pre-alpha）
  - 还没有 section 过滤

  2004-06-06

  - 收到了第一个 TS，但内核 oops :/

  2004-05-14

  - 固件加载器工作正常

  2004-05-11

  - 开始编写驱动

### 如何使用？


#### 固件


大多数 USB 驱动在开始工作之前，都需要向设备下载一个固件。

查看 DVB-USB 驱动的 Wiki 页面，以了解你的设备需要哪个固件：

https://linuxtv.org/wiki/index.php/DVB_USB

#### 编译


由于该驱动位于 Linux 内核中，在你喜欢的配置环境中启用该驱动就应当足够了。我建议
把驱动编译为模块。剩下的由 Hotplug 完成。

如果你使用 dvb-kernel，进入 build-2.6 目录，然后运行 'make'，之后运行 'insmod.sh
load'。

#### 加载驱动


Hotplug 能够在需要时（因为你插入了设备）加载驱动。

如果你想要启用调试输出，你必须手动加载驱动，并且是在 dvb-kernel cvs 仓库内部。

首先看一下有哪些调试级别可用：


	# modinfo dvb-usb
	# modinfo dvb-usb-vp7045

	etc.


	modprobe dvb-usb debug=<level>
	modprobe dvb-usb-vp7045 debug=<level>
	etc.

应该就能解决问题。

当驱动加载成功、固件文件位置正确、且设备已连接时，“Power” LED 应当亮起。

到这一步，你就应当能够启动一个支持 DVB 的应用程序了。我使用 (t|s)zap、mplayer 和
dvbscan 来测试基本功能。VDR-xine 提供了长期测试场景。

### 已知问题与缺陷


- 不要在运行 DVB 应用程序时拔除 USB 设备，你的系统很可能会发疯或者死机。

#### 为设备添加支持


TODO

#### USB1.1 带宽限制


目前受支持的许多设备都是 USB1.1 的，因此当连接到 USB2.0 集线器时，它们的最大带宽
约为 5-6 MBit/s。这对于接收一个 DVB-T 频道（约 16 MBit/s）的完整传输流来说是不够
的。通常这不成问题，如果你只想看电视（这不适用于 HDTV），但是在同一频率上边看一个
频道边录另一个频道就工作得不太好。这适用于所有 USB1.1 DVB-T 设备，而不仅仅是
dvb-usb 设备。

那个因重度使用设备而导致 TS 失真的 bug 已经彻底消失了。我用过的所有 dvb-usb 设备
（Twinhan、Kworld、DiBcom）现在配合 VDR 工作得像魔法一样。有时我甚至能够录一个频道
同时看另一个。

#### 评论


非常非常欢迎补丁、评论和建议。

### 3. 致谢


   Amaury Demol (Amaury.Demol@parrot.com) 和来自 DiBcom 的 Francois Kanounnikoff，
   提供了规格说明、代码和帮助，dvb-dibusb、dib3000mb 和 dib3000mc 就是基于它们的。

   David Matthews，他识别了一种新的设备类型（带 AN2235 的 Artec T1），并为 dibusb
   扩展了遥控器事件处理。谢谢你。

   Alex Woods，他频繁地回答关于 usb 和 dvb 的问题，非常感谢。

   Bernd Wagner，他提供了大量 bug 报告和讨论方面的帮助。

   Gunnar Wittich 和 Joachim von Caron，他们信任我，在自己的机器上提供 root shell
   来实现对新设备的支持。

   Allan Third 和 Michael Hutchinson，他们帮助编写了 Nebula digitv 驱动。

   Glen Harris，他提出了存在新的 dibusb 设备，以及来自 AVerMedia 的 Jiun-Kuei Jung，
   他善意地提供了一个特殊固件，使该设备在 Linux 中能够启动运行。

   Jennifer Chen、Jeff 和来自 Twinhan 的 Jack，他们通过编写 vp7045 驱动给予了善意的
   支持。

   Steve Chang，来自 WideView，他提供了新设备和固件文件的信息。

   Michael Paxton，他提交了遥控器键位映射。

   linux-dvb 邮件列表上的一些朋友，他们鼓励了我。

   Peter Schildmann <peter.schildmann-nospam-at-web.de>，他提供了用户态固件加载器，
   节省了大量时间（在编写 vp7041 驱动时）

   Ulf Hermenau，他帮我处理繁体中文。

   André Smoktun 和 Christian Frömmel，他们为我提供硬件，并非常耐心地听我倾诉问题。
