

## 简

Linux 的摇杆驱动为多种摇杆及类似设备提供支持。它基于一个更大的项目，该项目旨在支持 Linux 中的所有输入设备
该项目的邮件列表为：

	linux-input@vger.kernel.org

majordomo@vger.kernel.org 发"subscribe linux-input" 即可订阅
## 使用


对于基本使用，你只需在内核配置中选择正确的选项即可
### 工具


出于测试及其他目的（例如串行设备），有一组工具，`jstest`、`jscal` `evtest`，通常被打包为 `joystick`、`input-utils`、`evtest` 等
如果你的摇杆连接到串口，则需`inputattach` 工具
### 设备节点


为了让应用程序能够使用摇杆，应在 /dev 中创建设备节点。通常由系统自动完成，```

    cd /dev
    rm js*
    mkdir input
    mknod input/js0 c 13 0
    mknod input/js1 c 13 1
    mknod input/js2 c 13 2
    mknod input/js3 c 13 3
    ln -s input/js0 js0
    ln -s input/js1 js1
    ln -s input/js2 js2
    ln -s input/js3 js3

```
```

    mknod input/event0 c 13 64
    mknod input/event1 c 13 65
    mknod input/event2 c 13 66
    mknod input/event3 c 13 67

```
### 所需模块


为使所有摇杆驱动正常工作，你需要用户态接```

	modprobe joydev

```
```

	modprobe ns558

```
而对于串口摇杆，你需要串行输入线```

	modprobe serport
	inputattach -xxx /dev/tts/X &

```
除此之外，你还需要摇杆驱动模块本身，通常```

	modprobe analog

```
为了实现模块自动加载，类似下面的配置可能有效——请根据实际情况调整
```

	alias tty-ldisc-2 serport
	alias char-major-13 input
	above input joydev ns558 analog
	options analog map=gamepad,none,2btn

```
### 验证是否工作


为了测试摇杆驱动功能，可以使jstest
```

	jstest /dev/input/js0

```
它应显示一行摇杆数值，当你移动摇杆并按下按钮时这些数值会更新。当摇杆处于中心位置时，所有轴都应为零。它们不应自行跳变到其他接近的值，并且在摇杆的任何其他位置都应保持稳定。它们应具有-32767 32767 的完整范围。如果满足所有这些条件，那就一切正常，你可以玩游戏了)

如果不是，则可能存在问题。尝试校准摇杆，如果仍然不工作，请阅读本文件的驱动一节、排障一节以FAQ
### 校准


对于大多数摇杆，你不需要任何手动校准，因为摇杆应由驱动自动（automagically）自动校准。然而，对于某些模拟摇杆，它们要么不使用线性电阻，要么当你
```

	jscal -c /dev/input/js0

```
包含joystick 包中，用于设置比驱动自身选择更好的校正系数
校准摇杆后，你可以用 jstest 命令验证是否喜欢新的校准，如果喜欢，你可以保```

	jscal -p /dev/input/js0 > /etc/joystick.cal

```
```

	source /etc/joystick.cal

```
这样，在下一次重启后你的摇杆将保持已校准状态。你也可以把 `jscal -p` 这一行加入你的关机脚本
## 硬件特定驱动信息


本节描述各个独立的硬件特定驱动
### 模拟摇杆


analog.c 驱动使用游戏口的模拟标准输入，因此支持所有标准摇杆与游戏手柄。它为此使用了非常先进的例程，达到了其他任何系统都无法企及的数据精度
它还支持诸如CH Flightstick Pro、ThrustMaster FCS 6 键及 8 键游戏手柄兼容的额外帽键和按钮等扩展。Saitek Cyborg 'digital' 摇杆也受此驱动支持，因为它们本质上是加强CHF 摇杆
但唯一可以自动检测的类型是：

- 2 轴 键摇- 3 轴 键摇- 4 轴 键摇- Saitek Cyborg 'digital' 摇杆

对于其他摇杆类型（更更少轴、帽键和按钮）的支持，你需要在analog 插入内核时，在内核命令行或模块命令行上指定类型```

	analog.map=<type1>,<type2>,<type3>,....

```
'ttype'（类型）是下表中摇杆的类型，定义系统中游戏口上存在的摇杆，从 gameport0 开始，第二'type' 条目定义 gameport1 上的摇杆，依此类推
	========= =====================================================
	Type      Meaning
	========= =====================================================
	none      该端口上无模拟摇	auto      自动检测摇	2btn      2 n 轴摇	y-joy     一Y 线上两个 2 2 轴摇	y-pad     一Y 线上两个 2 2 轴游戏手	fcs       Thrustmaster FCS 兼容摇杆
	chf       CH Flightstick 兼容帽键的摇	fullchf   CH Flightstick 兼容，带两个帽键6 个按	gamepad   4/6 n 轴游戏手	gamepad8  8 2 轴游戏手	========= =====================================================

如果你的摇杆不属于上述任何类别，你可以将类型指定为一个数字，方法是组合下表中的位。除非你确实清楚自己在做什么，否则不建议这样做。这并不危险，但也不简单
	==== =========================
	Bit  Meaning
	==== =========================
	 0   Axis X1
	 1   Axis Y1
	 2   Axis X2
	 3   Axis Y2
	 4   Button A
	 5   Button B
	 6   Button C
	 7   Button D
	 8   CHF Buttons X and Y
	 9   CHF Hat 1
	10   CHF Hat 2
	11   FCS Hat
	12   Pad Button X
	13   Pad Button Y
	14   Pad Button U
	15   Pad Button V
	16   Saitek F1-F4 Buttons
	17   Saitek Digital Mode
	19   GamePad
	20   Joy2 Axis X1
	21   Joy2 Axis Y1
	22   Joy2 Axis X2
	23   Joy2 Axis Y2
	24   Joy2 Button A
	25   Joy2 Button B
	26   Joy2 Button C
	27   Joy2 Button D
	31   Joy2 GamePad
	==== =========================

### Microsoft SideWinder 摇杆


sidewinder.c 模块支持 Microsoft 'Digital Overdrive' 协议。所有当前支持的摇杆
- Microsoft SideWinder 3D Pro
- Microsoft SideWinder Force Feedback Pro
- Microsoft SideWinder Force Feedback Wheel
- Microsoft SideWinder FreeStyle Pro
- Microsoft SideWinder GamePad（最多四个，链式连接- Microsoft SideWinder Precision Pro
- Microsoft SideWinder Precision Pro USB

均可自动检测，因此不需要模块参数
3D Pro 有一个需要注意之处。它会报9 个按钮，尽管摇杆只有 8 个。第 9 个按钮是摇杆后侧的模式开关。不过，移动它会使摇杆复位，并使其在约三分之一秒内无响应。此外，摇杆还会重新居中，将这段时间内的位置作为新的中心位置。想用就用，但先想清楚
SideWinder Standard 不是数字摇杆，因此由上文描述的模拟驱动支持
### Logitech ADI 设备


adi.c 模块支持 Logitech ADI 协议。它应支持任何使用该协议Logitech 设备。这包括但不限于
- Logitech CyberMan 2
- Logitech ThunderPad Digital
- Logitech WingMan Extreme Digital
- Logitech WingMan Formula
- Logitech WingMan Interceptor
- Logitech WingMan GamePad
- Logitech WingMan GamePad USB
- Logitech WingMan GamePad Extreme
- Logitech WingMan Extreme Digital 3D

ADI 设备是自动检测的，该驱动在使Y 线或链式连接的情况下，支持单个游戏口上最多两个（任意组合）设备
Logitech WingMan Joystick、Logitech WingMan Attack、Logitech WingMan Extreme 以及 Logitech WingMan ThunderPad 不是数字摇杆，由上文描述的模拟驱动处理。Logitech WingMan Warrior Logitech Magellan 由下文描述的串行驱动支持。Logitech WingMan Force Logitech WingMan Formula Force 由下文描述的 I-Force 驱动支持。Logitech CyberMan 尚不支持
### Gravis GrIP


grip.c 模块支持 Gravis GrIP 协议。它目前支持
- Gravis GamePad Pro
- Gravis BlackHawk Digital
- Gravis Xterminator
- Gravis Xterminator DualControl

所有这些设备都是自动检测的，你甚至可以在单个游戏口上以任意组合使用最多两个这样的手柄，无论是链式连接还是使用 Y 线
GrIP MultiPort 尚不支持。Gravis Stinger 是串行设备，stinger 驱动支持。其Gravis 摇杆由模拟驱动支持
### FPGaming A3D 涓?MadCatz A3D


FPGaming 创建Assassin 3D 协议，既FPGaming 自己使用，也被授权给 MadCatz。A3D 设备a3d.c 模块支持。它目前支持
- FPGaming Assassin 3D
- MadCatz Panther
- MadCatz Panther XL

所有这些设备都是自动检测的。由Assassin 3D Panther 允许连接模拟摇杆，你还需要加载模拟驱动来处理所连接的摇杆
轨迹球应作为普通鼠标配USB mousedev 模块工作。有关如何设USB 鼠标，请参见 USB 文档
### ThrustMaster DirectConnect (BSP)


tmdc.c 模块支持 TM DirectConnect (BSP) 协议。这包括但不限于
- ThrustMaster Millennium 3D Interceptor
- ThrustMaster 3D Rage Pad
- ThrustMaster Fusion Digital Game Pad

未直接支持但有望工作的设备：

- ThrustMaster FragMaster
- ThrustMaster Attack Throttle

如果你拥有其中之一，请联系我
TMDC 设备是自动检测的，因此不需要给模块传参数。使Y 线，最多可将两TMDC 设备连接到单个游戏口
### Creative Labs Blaster


cobra.c 模块支持 Blaster 协议。它仅支持：

- Creative Blaster GamePad Cobra

使用 Y 线，最多可在单个游戏口上使用两个这样的设备
### Genius Digital 摇杆


gf2k.c 模块支持 Genius 数字通信摇杆。这包括
- Genius Flight2000 F-23 摇杆
- Genius Flight2000 F-31 摇杆
- Genius G-09D 游戏手柄

其他 Genius 数字摇杆尚不支持，但相当容易添加支持
### InterAct Digital 摇杆


interact.c 模块支持 InterAct 数字通信摇杆。这包括
- InterAct HammerHead/FX 游戏手柄
- InterAct ProPad8 游戏手柄

其他 InterAct 数字摇杆尚不支持，但相当容易添加支持
### PDPI Lightning 4 游戏

lightning.c 模块支持 PDPI Lightning 4 游戏卡。模块加载后，可用模拟驱动来处理摇杆。数字通信摇杆只能在端0 上工作，而使Y 线，你可以将最8 个模拟摇杆连接到单个 L4 卡上；如果你的系统中有两张卡，则16 个
### Trident 4DWave / Aureal Vortex


带有 Trident 4DWave DX/NX Aureal Vortex/Vortex2 芯片组的声卡提供"增强游戏口（Enhanced Game Port模式，由声卡负责轮询摇杆。pcigame.c 模块支持此模式。加载后，模拟驱动即可使用这些游戏口的增强特性
### Crystal SoundFusion


带有 Crystal SoundFusion 芯片组的声卡提供"增强游戏口（Enhanced Game Port，与上文4DWave Vortex 非常相似。这一点，以及 SoundFusion 端口的普通模式，都由 cs461x.c 模块支持
### SoundBlaster Live!


Live! 有一个特殊的 PCI 游戏口，尽管它不4DWave 及其同类那样提供任何"增强"功能，但比其 ISA 同类要快得多。它也需要特殊支持，因此使用 emu10k1-gp.c 模块，而不是普通的 ns558.c
### SoundBlaster 64 128 - ES1370 ES1371、ESS Solo1 S3 SonicVibes


这些 PCI 声卡有特定的游戏口。它们由声卡驱动自身处理。请确保为你的相应声卡在摇杆菜单中选择游戏口支持，并在声音菜单中选择声卡支持
### Amiga


连接Amiga Amiga 摇杆amijoy.c 驱动支持。由于它们无法被自动检测，该驱动有一个命令行
	amijoy.map=<a>,<b>

a b 定义连接Amiga JOY0DAT JOY1DAT 端口的摇杆
	====== ===========================
	Value  Joystick type
	====== ===========================
	  0    None
	  1    1-button digital joystick
	====== ===========================

目前不支持更多摇杆类型，但如果我手边能拿到一Amiga，未来这应该会改变
### 游戏主机8 位手柄及摇杆


这些手柄和摇杆并非为 PC 以及运行 Linux 的其他计算机设计，通常需要通过并口连接的特殊连接器
更多资讯请参joystick-parport
### SpaceTec/LabTec 设备


SpaceTec 串行设备使用 SpaceWare 协议通信。spaceorb.c spaceball.c 驱动支持该协议。spaceorb.c 当前支持的设备：

- SpaceTec SpaceBall Avenger
- SpaceTec SpaceOrb 360

spaceball.c 当前支持的设备：

- SpaceTec SpaceBall 4000 FLX

除了在内核中拥有 spaceorb/spaceball serport 模块外，你还需要将一个串口连接到它。为此，运行
```

	inputattach --spaceorb /dev/tts/x &

```
```

	inputattach --spaceball /dev/tts/x &

```
其中 /dev/tts/x 是设备所连接的串口。完成此操作后，设备将被报告并开始工作
SpaceOrb 有一个需要注意之处。第 6 个按钮，即球体底部的那个，尽管被报告为普通按钮，但会导致 spaceorb 内部重新居中，将零点移动到按下按钮时球所在的位置。因此，在将它绑定到其他功能之前请先想清楚
SpaceTec SpaceBall 2003 FLX 3003 FLX 尚不支持
### Logitech SWIFT 设备


warrior.c 模块支持 SWIFT 串行协议。它目前仅支持：

- Logitech WingMan Warrior

但未来，Logitech CyberMan（原始版本，而非 CM2）也可能得到支持。要使用模块，你需要在之后运行 inputattach
```

	inputattach --warrior /dev/tts/x &

```
/dev/tts/x 是你Warrior 所连接的串口
### Magellan / Space Mouse


LogiCad3d（前Space Systems）为许多其他公司（Logitech、HP 等）制造的 Magellan（或 Space Mouse），joy-magellan 模块支持。它目前仅支持：

- Magellan 3D
- Space Mouse

型号Plus' 版本的额外按钮尚不支持```

	inputattach --magellan /dev/tts/x &

```
命令。之Magellan 将被检测、初始化、发出蜂鸣，并且 /dev/input/jsX 设备应变得可用
### I-Force 设备


所I-Force 设备都由 iforce 模块支持。这包括
- AVB Mag Turbo Force
- AVB Top Shot Pegasus
- AVB Top Shot Force Feedback Racing Wheel
- Boeder Force Feedback Wheel
- Logitech WingMan Force
- Logitech WingMan Force Wheel
- Guillemot Race Leader Force Feedback
- Guillemot Force Feedback Racing Wheel
- Thrustmaster Motor Sport GT

```

	inputattach --iforce /dev/tts/x &

```
命令。之I-Force 设备将被检测，并且 /dev/input/jsX 设备应变得可用
如果你通过 USB 端口使用设备，则不需inputattach 命令
I-Force 驱动现在支持通过 event 接口进行力反馈
请注意，Logitech WingMan 3D 设备_不_受此模块支持，而是hid 支持。这些设备不支持力反馈。Logitech 游戏手柄也是 hid 设备
### Gravis Stinger 游戏手柄


为配合笔记本电脑使用而设计的 Gravis Stinger 串口游戏手柄，由 stinger.c 模块支持。要使用它，连接
```

	inputattach --stinger /dev/tty/x &

```
其中 x 是串口编号
## 排障


你遇到一些问题有相当高的概率。要测试驱动是否工作，如有疑问，可使jstest 工具的某些模式。最有用的模式是 "normal"——针1.x
```

	jstest --normal /dev/input/js0
	jstest --old    /dev/input/js0

```
```

	evtest /dev/input/event0

```
哦，还要阅读 FAQ)

## FAQ


:Q: 运行 'jstest /dev/input/js0' 出现 "File not found" 错误。原因是什么？
:A: 设备文件不存在。创建它们（见第 2.2 节）
:Q: 能否将我旧的 Atari/Commodore/Amiga/游戏主机摇杆或手柄（使用 9 D Cannon 连接器）连接到我 PC 的串口？
:A: 可以，但会烧毁你的串口或手柄。当然，它不会工作
:Q: 我的摇杆Quake / Quake 2 中不起作用。原因是什么？
:A: Quake / Quake 2 不支持摇杆。使joy2key 为它们模拟按键