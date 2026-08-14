
## 并口游戏杆驱动

:Copyright: |copy| 1998-2000 Vojtech Pavlik <vojtech@ucw.cz>
:Copyright: |copy| 1998 Andree Borrmann <a.borrmann@tu-bs.de>

Sponsored by SuSE

## 免责声明

本文中的任何信息均按"原样"提供，不保证其准确性。因此，请自行承担使用风险。可能造成的损害包括烧毁你的并口、操纵杆和游戏杆，甚至可能更严重。就好比被闪电击中并不是我们的问题一样。

## 简介

并口游戏杆驱动用于那些原本并非为 PC 以及 Linux 所运行的其他计算机设计的游戏杆和游戏手柄。正因为如此，PC 通常缺少用于连接这些设备的合适端口。并口由于能够随意改变单个比特，并且同时提供输出和输入比特，因此成了 PC 上连接此类设备最合适的端口。

## 支持的设备

许多游戏主机和 8 位计算机的游戏手柄和游戏杆都受支持。以下小节分别讨论各自的使用方法。

### NES 与 SNES

Nintendo Entertainment System（任天堂娱乐系统）和 Super Nintendo Entertainment System（超级任天堂娱乐系统）的游戏手柄随处可得，很容易买到。而且它们连接到 PC 也相当容易，与它们通信不需要很高的处理速度（NES 为 108 微秒，SNES 为 165 微秒，而 PC 游戏手柄约为 1000 微秒）。

所有 NES 和 SNES 都使用相同的同步串行协议，由计算机一侧提供时钟（因此对时序不敏感）。为了允许最多 5 个 NES 和/或 SNES 游戏手柄和/或 SNES 鼠标同时连接到并口，并口的输出线是共享的，而 5 个可用输入线中的一个被分配给每个游戏手柄。

该协议由 gamecon.c 驱动处理，因此你将使用它来连接 NES、SNES 游戏手柄和 SNES 鼠标。

PC 并口的主要问题是没有一个引脚提供 +5V 电源。所以，如果你想要一个可靠的电源给手柄供电，请使用键盘端口或游戏杆端口，并制作一根转接电缆。你也可以直接从电源取电（红色线是 +5V）。

如果你只想使用并口，可以从某个数据引脚取电。对于大多数游戏手柄和并口实现来说只需要一个引脚，我推荐使用引脚 9（最高位数据位）来取电。另一方面，如果你不打算在并口上连接 NES/SNES 之外的其他设备，那么：

```
    (pin 9) -----> Power
```

遗憾的是，有些手柄需要多得多的电力，而有些并口无法通过数据引脚提供较大电流。如果你遇到这种情况，需要使用二极管（以防止烧毁你的并口）：

```
	      Diodes
    (pin 9) ----|>|-------+------> Power
			|
    (pin 8) ----|>|-------+
			|
    (pin 7) ----|>|-------+
			|
    <and so on>         :
			|
    (pin 4) ----|>|-------+
```

接地很简单。在 PC 并口上，接地位于以下任意一个引脚：

```
    (pin 18) -----> Ground
```

NES 和 SNES 手柄有两个输入比特，Clock（时钟）和 Latch（锁存），它们驱动串行传输。这两个信号连接到并口的引脚 2 和引脚 3：

```
    (pin 2) -----> Clock
    (pin 3) -----> Latch
```

最后一项是 NES/SNES 的数据线。只有这一路不是共享的，连接如下：

```
    (pin 10) -----> Pad 1 data
    (pin 11) -----> Pad 2 data
    (pin 12) -----> Pad 3 data
    (pin 13) -----> Pad 4 data
    (pin 15) -----> Pad 5 data
```

注意引脚 14 未被使用，因为它在并口上不是输入引脚。

这就是 PC 一侧连接所需的一切，接下来看看游戏手柄一侧。NES 和 SNES 使用不同的连接器。此外，有相当多的 NES 仿制品，由于任天堂在其机器上使用了专有连接器，仿制者无法使用，于是采用了标准的 D-Cannon 连接器。无论如何，如果你有一个游戏手柄，它有 A、B、Turbo A、Turbo B、Select 和 Start 按钮，并且通过 5 根线连接，那么它就是 NES 或 NES 仿制品，能够通过此连接工作。SNES 游戏手柄则：

```
  Pinout for NES gamepads                 Pinout for SNES gamepads and mice

             +----> Power                   +-----------------------\
             |                            7 | o  o  o  o |  x  x  o  | 1
   5 +---------+  7                         +-----------------------/
     | x  x  o   \                            |  |  |  |          |
     | o  o  o  o |                           |  |  |  |          +-> Ground
   4 +------------+ 1                         |  |  |  +------------> Data
       |  |  |  |                             |  |  +---------------> Latch
       |  |  |  +-> Ground                    |  +------------------> Clock
       |  |  +----> Clock                     +---------------------> Power
       |  +-------> Latch
       +----------> Data

  Pinout for NES clone (db9) gamepads     Pinout for NES clone (db15) gamepads

        +---------> Clock                    +-----------------> Data
        | +-------> Latch                    |             +---> Ground
        | | +-----> Data                     |             |
        | | |                              ___________________
    _____________                        8 \ o x x x x x x o / 1
  5 \ x o o o x / 1                         \ o x x o x x o /
     \ x o x o /                          15 `~~~~~~~~~~~~~' 9
    9 `~~~~~~~' 6                             |     |     |
         |   |                                |     |     +----> Clock
         |   +----> Power                     |     +----------> Latch
         +--------> Ground                    +----------------> Power
```

### Multisystem 游戏杆

在 8 位机器时代，游戏杆端口存在一种事实上的标准。它们都是数字的，并且都使用 D-Cannon 9 针连接器（db9）。因此，单个游戏杆可以在 Atari（130、800XE、800XL、2600、7200）、Amiga、Commodore C64、Amstrad CPC、Sinclair ZX Spectrum 以及许多其他机器上无碍使用。正因如此，这些游戏杆被称为"Multisystem"（多系统）。

```
        +---------> Right
        | +-------> Left
        | | +-----> Down
        | | | +---> Up
        | | | |
    _____________
  5 \ x o o o o / 1
     \ x o x o /
    9 `~~~~~~~' 6
         |   |
         |   +----> Button
         +--------> Ground
```

然而，随着时间的推移，针对这一标准的扩展出现了，这些扩展如下：

```
          Atari 130, 800/XL/XE                   MSX

                                           +-----------> Power
        +---------> Right                  | +---------> Right
        | +-------> Left                   | | +-------> Left
        | | +-----> Down                   | | | +-----> Down
        | | | +---> Up                     | | | | +---> Up
        | | | |                            | | | | |
    _____________                        _____________
  5 \ x o o o o / 1                    5 \ o o o o o / 1
     \ x o o o /                          \ o o o o /
    9 `~~~~~~~' 6                        9 `~~~~~~~' 6
         | | |                              | | | |
         | | +----> Button                  | | | +----> Button 1
         | +------> Power                   | | +------> Button 2
         +--------> Ground                  | +--------> Output 3
                                            +----------> Ground

          Amstrad CPC                           Commodore C64

                                           +-----------> Analog Y
        +---------> Right                  | +---------> Right
        | +-------> Left                   | | +-------> Left
        | | +-----> Down                   | | | +-----> Down
        | | | +---> Up                     | | | | +---> Up
        | | | |                            | | | | |
    _____________                        _____________
  5 \ x o o o o / 1                    5 \ o o o o o / 1
     \ x o o o /                          \ o o o o /
    9 `~~~~~~~' 6                        9 `~~~~~~~' 6
         | | |                              | | | |
         | | +----> Button 1                | | | +----> Button
         | +------> Button 2                | | +------> Power
         +--------> Ground                  | +--------> Ground
                                            +----------> Analog X

          Sinclair Spectrum +2A/+3           Amiga 1200

      +-----------> Up                     +-----------> Button 3
      | +---------> Fire                   | +---------> Right
      | |                                  | | +-------> Left
      | |   +-----> Ground                 | | | +-----> Down
      | |   |                              | | | | +---> Up
      | |   |                              | | | | |
    _____________                        _____________
  5 \ o o x o x / 1                    5 \ o o o o o / 1
     \ o o o o /                          \ o o o o /
    9 `~~~~~~~' 6                        9 `~~~~~~~' 6
       | | | |                              | | | |
       | | | +----> Right                   | | | +----> Button 1
       | | +------> Left                    | | +------> Power
       | +--------> Ground                  | +--------> Ground
       +----------> Down                    +----------> Button 2

  还有很多其他的。
```

#### 使用 db9.c 的 Multisystem 游戏杆

针对 Multisystem 游戏杆及其衍生型号，编写了 db9.c 驱动。每个并口只允许一个游戏杆/游戏手柄，但接口易于制作，并且几乎能与任何设备配合工作。

对于基本的单按钮 Multisystem 游戏杆，将其引线连接到：

```
    (pin  1) -----> Power
    (pin 18) -----> Ground

    (pin  2) -----> Up
    (pin  3) -----> Down
    (pin  4) -----> Left
    (pin  5) -----> Right
    (pin  6) -----> Button 1
```

不过，如果游戏杆是基于开关的（例如移动时会有咔哒声），你可能（也可能不，取决于你的并口）需要 10 kOhm 上拉电阻：

```
    (pin 2) ------------+------> Up
              Resistor  |
    (pin 1) --[10kOhm]--+
```

先不使用试试，如果不工作再加上。对于基于 TTL 的游戏杆/游戏手柄则不需要上拉电阻。

对于带有两个按钮的游戏杆，将第二个按钮连接到引脚 7：

```
    (pin 7) -----> Button 2
```

就这样。

附带说明一下，如果你已经为数字游戏杆驱动 0.8.0.2 制作了不同的适配器，db9.c 驱动也支持它，作为设备类型 8。（见第 3.2 节）

#### 使用 gamecon.c 的 Multisystem 游戏杆

对有些人来说，每个并口只接一个游戏杆是不够用的，而且/或者希望将它们与 NES/SNES/PSX 手柄共用一个并口。这通过 gamecon.c 是可以实现的。它支持上述类型的最多 5 个设备，包括单按钮和双按钮的 Multisystem 游戏杆。

然而，天下没有免费的午餐。为了允许同时使用更多游戏杆，你需要这些游戏杆是纯开关型的（即非 TTL），并且不需要供电。仅仅是内部简单的六个开关即可。如果你的游戏杆能做更多（例如连发），在使用 gamecon.c 之前你需要先将其完全禁用。

此外，连接要稍微复杂一些。你需要一堆二极管和一个上拉电阻。首先，将方向和按钮连接到：

```
                Diodes
    (pin 2) -----|<|----> Up
    (pin 3) -----|<|----> Down
    (pin 4) -----|<|----> Left
    (pin 5) -----|<|----> Right
    (pin 6) -----|<|----> Button 1
```

```
    (pin 7) -----|<|----> Button 2
```

最后，将游戏杆的接地引线，按照本文件第 2.1 节中为 NES/SNES 手柄所描述的那样，连接到并口的 Power（电源）和 Data（数据）上——也就是说，使用一个数据引脚：

```
    Data    ------------+-----> Ground
              Resistor  |
    Power   --[10kOhm]--+
```

这就是全部，开始吧！

#### 使用 turbografx.c 的 Multisystem 游戏杆

TurboGraFX 接口由

	Steffen Schwenke <schwenke@burg-halle.de>

设计，允许最多 7 个 Multisystem 游戏杆连接到并口。在 Steffen 的版本中，每个游戏杆最多支持 5 个按钮。然而，由于这在所有并口上都不能可靠工作，turbografx.c 驱动每个游戏杆仅支持一个按钮。关于如何制作该接口的更多信息，请参阅：

	http://www2.burg-halle.de/~schwenke/parport.html

### Sony Playstation

PSX 控制器由 gamecon.c 支持。PSX 的引脚定义如下：

```
    +---------+---------+---------+
  9 | o  o  o | o  o  o | o  o  o | 1               parallel
     \________|_________|________/                  port pins
      |  |      |  |  |   |
      |  |      |  |  |   +-------->  Clock    ---  (4)
      |  |      |  |  +------------>  Select   ---  (3)
      |  |      |  +--------------->  Power    ---  (5-9)
      |  |      +------------------>  Ground   ---  (18-25)
      |  +------------------------->  Command  ---  (2)
      +---------------------------->  Data     ---  (one of 10,11,12,13,15)
```

该驱动支持以下控制器：

 - 标准 PSX 手柄
 - NegCon PSX 手柄
 - 模拟 PSX 手柄（red 模式）
 - 模拟 PSX 手柄（green 模式）
 - PSX Rumble 手柄
 - PSX DDR 手柄

### Sega

所有 Sega 控制器或多或少都基于标准的双按钮 Multisystem 游戏杆。然而，由于它们不使用开关而使用 TTL 逻辑，唯一可用于它们的驱动是 db9.c。

#### Sega Master System

SMS 游戏手柄几乎与普通的双按钮 Multisystem 游戏杆完全相同。将驱动设置为 Multi2 模式，使用相应的引脚定义：

```
      +-----------> Power
      | +---------> Right
      | | +-------> Left
      | | | +-----> Down
      | | | | +---> Up
      | | | | |
    _____________
  5 \ o o o o o / 1
     \ o o x o /
    9 `~~~~~~~' 6
       | |   |
       | |   +----> Button 1
       | +--------> Ground
       +----------> Button 2
```

#### Sega Genesis（又名 MegaDrive）

Sega Genesis（在欧洲销售时称为 Sega MegaDrive）手柄是对 Sega Master System 手柄的扩展。它们使用更多按钮（3+1、5+1、6+1）。使用：

```
        +-----------> Power
        | +---------> Right
        | | +-------> Left
        | | | +-----> Down
        | | | | +---> Up
        | | | | |
      _____________
    5 \ o o o o o / 1
       \ o o o o /
      9 `~~~~~~~' 6
        | | | |
        | | | +----> Button 1
        | | +------> Select
        | +--------> Ground
        +----------> Button 2
```

```
    (pin 14) -----> Select
```

其余部分与使用 db9.c 的 Multi2 游戏杆相同。

#### Sega Saturn

Sega Saturn 有八个按钮，为了传输这些数据，不像 Genesis 6 手柄那样使用 hack，它还需要一个额外的选择引脚。无论如何，它仍然由 db9.c 驱动处理。它的引脚定义与任何现有设备都大不相同：

```
      +-----------> Select 1
      | +---------> Power
      | | +-------> Up
      | | | +-----> Down
      | | | | +---> Ground
      | | | | |
    _____________
  5 \ o o o o o / 1
     \ o o o o /
    9 `~~~~~~~' 6
       | | | |
       | | | +----> Select 2
       | | +------> Right
       | +--------> Left
       +----------> Power
```

Select 1 在并口上是引脚 14，Select 2 在并口上是引脚 16：

```
    (pin 14) -----> Select 1
    (pin 16) -----> Select 2
```

其他引脚（Up、Down、Right、Left、Power、Ground）与使用 db9.c 的 Multi 游戏杆相同。

### Amiga CD32

```
        +-----------> Button 3
        | +---------> Right
        | | +-------> Left
        | | | +-----> Down
        | | | | +---> Up
        | | | | |
      _____________
    5 \ o o o o o / 1
       \ o o o o /
    9 `~~~~~~~' 6
        | | | |
        | | | +----> Button 1
        | | +------> Power
        | +--------> Ground
        +----------> Button 2
```

它可以连接到并口并由 db9.c 驱动。它需要的接线如下：

	============    =============
	CD32 pad        Parallel port
	============    =============
	1 (Up)           2 (D0)
	2 (Down)         3 (D1)
	3 (Left)         4 (D2)
	4 (Right)        5 (D3)
	5 (Button 3)    14 (AUTOFD)
	6 (Button 1)    17 (SELIN)
	7 (+5V)          1 (STROBE)
	8 (Gnd)         18 (Gnd)
	9 (Button 2)     7 (D5)
	============    =============

## 驱动

并口接口共有三个驱动。如上所述，每个驱动可以连接不同组的游戏杆和手柄。下面描述它们的命令行：

### gamecon.c

使用 gamecon.c，你可以将最多五个设备连接到一个并口。它：

```
	gamecon.map=port,pad1,pad2,pad3,pad4,pad5
```

其中 `port` 是 parport 接口的编号（例如 parport0 为 0）。

而 `pad1` 到 `pad5` 是连接到不同数据输入引脚（10、11、12、13、15）的手柄类型，如本文件第 2.1 节所述。

类型如下：

	===== =============================
	Type  Joystick/Pad
	===== =============================
	  0   None
	  1   SNES pad
	  2   NES pad
	  4   Multisystem 1-button joystick
	  5   Multisystem 2-button joystick
	  6   N64 pad
	  7   Sony PSX controller
	  8   Sony PSX DDR controller
	  9   SNES mouse
	===== =============================

PSX 控制器类型的确切类型在使用时会被自动探测，因此热插拔应该可以工作（但不推荐）。

如果你希望同时使用多个并口，可以使用 gamecon.map2 和 gamecon.map3 作为另外两个并口的附加命令行参数。

有两个专门用于 PSX 驱动部分的选项。gamecon.psx_delay 设置在与控制器通信时的命令延迟。默认值 25 应该可以工作，但你可以尝试降低它以获得更好的性能。如果你的手柄没有响应，请尝试提高它直到正常工作。将类型设置为 8 可让驱动用于 Dance Dance Revolution 或类似的游戏。方向键被注册为按键按下，而不是 X 和 Y 轴。

### db9.c

除了制作接口之外，使用 db9.c 没有难度：

```
	db9.dev=port,type
```

其中 `port` 是 parport 接口的编号（例如 parport0 为 0）。

这里的注意事项：此驱动仅适用于双向并口。如果你的并口足够新，应该不会有问题。老式并口可能没有这一特性。

`Type` 是所连接的游戏杆或手柄的类型：

	===== ======================================================
	Type  Joystick/Pad
	===== ======================================================
	  0   None
	  1   Multisystem 1-button joystick
	  2   Multisystem 2-button joystick
	  3   Genesis pad (3+1 buttons)
	  5   Genesis pad (5+1 buttons)
	  6   Genesis pad (6+2 buttons)
	  7   Saturn pad (8 buttons)
	  8   Multisystem 1-button joystick (v0.8.0.2 pin-out)
	  9   Two Multisystem 1-button joysticks (v0.8.0.2 pin-out)
	 10   Amiga CD32 pad
	===== ======================================================

如果你希望同时使用多个此类游戏杆/手柄，可以使用 db9.dev2 和 db9.dev3 作为另外两个游戏杆/手柄的附加命令行参数。

### turbografx.c

```
	turbografx.map=port,js1,js2,js3,js4,js5,js6,js7
```

其中 `port` 是 parport 接口的编号（例如 parport0 为 0）。

`jsX` 是连接到接口端口 1-7 的 Multisystem 游戏杆的按钮数。对于标准 Multisystem 游戏杆，此值为 1。

如果你希望同时使用多个此类接口，可以使用 turbografx.map2 和 turbografx.map3 作为另外两个接口的附加命令行参数。

## PC 并口引脚定义

```
		  .----------------------------------------.
   At the PC:     \ 13 12 11 10  9  8  7  6  5  4  3  2  1 /
                   \  25 24 23 22 21 20 19 18 17 16 15 14 /
                     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
```

======  =======  =============
   Pin  Name     Description
======  =======  =============
     1  /STROBE  Strobe
   2-9  D0-D7    Data Bit 0-7
    10  /ACK     Acknowledge
    11  BUSY     Busy
    12  PE       Paper End
    13  SELIN    Select In
    14  /AUTOFD  Autofeed
    15  /ERROR   Error
    16  /INIT    Initialize
    17  /SEL     Select
 18-25  GND      Signal Ground
======  =======  =============

就是这样，朋友们！玩得开心！
