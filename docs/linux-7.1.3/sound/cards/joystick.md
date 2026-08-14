## ALSA 驱动上的模拟游戏杆支持


2003 年 10 月 14 日

Takashi Iwai <tiwai@suse.de>

### 概述


首先，要在 ALSA 驱动下使用游戏杆，需要在 Linux 内核中启用 GAMEPORT 支持。有关 gameport 支持的详细信息，请参阅 Documentation/input/joydev/joystick.rst。

ALSA 驱动的游戏杆支持在 ISA 与 PCI 卡之间有所不同。对于 ISA（PnP）卡，通常由独立模块（ns558）来处理。而 ALSA 的 PCI 驱动则内置了 gameport 支持。因此，当 ALSA 的 PCI 驱动编译进内核时，`CONFIG_GAMEPORT` 也必须为 'y'。否则，该卡上的 gameport 支持将被（静默地）禁用。

某些适配模块会在加载时探测设备的物理连接。在加载模块之前插入游戏杆设备会更稳妥。


### PCI 卡


对于 PCI 卡，在指定了相应的模块选项后游戏杆即被启用。有些驱动不需要选项，游戏杆支持始终处于启用状态。在较早的 ALSA 版本中，曾有一个用于游戏杆激活的动态控制 API。不过，出于系统稳定性与资源管理的考虑，它已被改为静态模块选项。

以下 PCI 驱动原生支持游戏杆。

==============	=============	============================================
驱动	模块选项	可用值
==============	=============	============================================
als4000		joystick_port	0 = 禁用（默认），1 = 自动检测，
				手动：任意地址（如 0x200）
au88x0		N/A		N/A
azf3328		joystick	0 = 禁用，1 = 启用，-1 = 自动（默认）
ens1370		joystick	0 = 禁用（默认），1 = 启用
ens1371		joystick_port	0 = 禁用（默认），1 = 自动检测，
				手动：0x200, 0x208, 0x210, 0x218
cmipci		joystick_port	0 = 禁用（默认），1 = 自动检测，
				手动：任意地址（如 0x200）
cs4281		N/A		N/A
cs46xx		N/A		N/A
es1938		N/A		N/A
es1968		joystick	0 = 禁用（默认），1 = 启用
sonicvibes	N/A		N/A
trident		N/A		N/A
via82xx [#f1]_	joystick	0 = 禁用（默认），1 = 启用
ymfpci		joystick_port	0 = 禁用（默认），1 = 自动检测，
				手动：0x201, 0x202, 0x204, 0x205 [#f2]_
==============	=============	============================================


以下驱动并不原生支持 gameport，但有额外的模块可用。加载对应的模块即可添加 gameport 支持。

=======	=================
驱动	附加模块
=======	=================
emu10k1	emu10k1-gp
fm801	fm801-gp
=======	=================

注意：“pcigame” 与 “cs461x” 模块仅用于 OSS 驱动。这些 ALSA 驱动（cs46xx、trident 与 au88x0）均内置了 gameport 支持。

如上所述，ALSA 的 PCI 驱动内置了 gameport 支持，因此无需加载 ns558 模块。只需加载 “joydev” 及相应的适配模块（如 “analog”）即可。


### ISA 卡


ALSA 的 ISA 驱动没有内置的 gameport 支持。相反，除了 “joydev” 和适配模块（如 “analog”）之外，你还需要加载 “ns558” 模块。
