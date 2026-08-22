# xpad —用于 Xbox 兼容手柄Linux USB 驱动


该驱动支持所有第一方与第三Xbox 兼容手柄。它有很长的历史，并被广泛使用，这是因为 Windows xinput 库使得大多数 PC 游戏都侧重于 Xbox 手柄兼容性

出于向后兼容的考虑，所有按键都以数字形式上报。这仅影响初Xbox 手柄。所有后续手柄型号的面键均为数字按键

部分 Xbox 360 手柄支持震动，但初代 Xbox 手柄Xbox One 手柄不支持。截至本文撰写时，Xbox One 的震动协议尚未被逆向工程，但未来可能会得到支持


## 注意事项


## 上报的按轴数量取决于以下三件事：

- 是否在使用已知的手柄
 - 是否在使用已知的跳舞
 - 若使用的是未知设备（下列未列出的），则取决于你在模块配置中为“将未知跳舞毯的 D-PAD 映射为按键而非轴”（模块选项 dpad_to_buttons）所设置的

如果你将 dpad_to_buttons 设为 N，且使用的是未知设备，驱动会将方向键映射为轴（X/Y）。若设为 Y，则会将方向键映射为按键，这是跳舞类游戏正常运行所必需的。默认值为 Y

dpad_to_buttons 对已知跳舞毯无效。曾有错误的提交说明声称 dpad_to_buttons 可用于强制已知设备的行为，事实并非如此。dpad_to_buttons triggers_to_buttons 仅影响未知手柄


## 普通手


对于普通手柄，方向键被映射为独立的 X/Y 轴。来joystick-1.2.15（jstest 版本 2.1.0）的 jstest 程序将报8 个轴10 个按键

全部 8 个轴都可用，不过它们的范围相同（-32768..32767），且扳机的归零设置并不正确（我不确定这jstest 的某种限制，因为输入设备的设置应当没问题。我尚未查看 jstest 本身）

全部 10 个按键都可用（数字模式）。右侧的六个按键（A、B、X、Y、black、white）据称是“模拟”的，并8 位无符号值上报，不清楚这有何用途

我用 quake3 测试了该手柄，配置与游戏内功能均正常。不过，我发现用手柄玩第一人称射击游戏相当困难。具体体验可能因人而异


## Xbox 跳舞


使用已知的跳舞毯时，jstest 将报6 个轴14 个按键

针对跳舞类跳舞毯（如 redoctane 跳舞毯）做了若干改动。旧驱动会将方向键映射为轴，导致当用户同时按下左+右或下时驱动无法上报，使DDR 类游戏无法游玩

已知的跳舞毯会自动将方向键映射为按键，开箱即可正常工作

如果你的跳舞毯已被驱动识别，但使用的是轴而非按键，请参阅0.3 节——未知手柄

我用 Stepmania 测试过，效果相当好


## 未知手柄


如果你有一个未知的 Xbox 手柄，使用默认设置应当就能正常工作

但是，如果你有一个下列未列出的未知跳舞毯，它将无法工作，除非你在模块配置中将 “dpad_to_buttons设为 1


## USB 閫傞厤鍣。


各代 Xbox 手柄都通过线缆使用 USB 通信

- 初代 Xbox 手柄使用专有接口，需要适配器
 - 无线 Xbox 360 手柄需'Xbox 360 Wireless Gaming Receiver for Windows'
 - 有线 Xbox 360 手柄使用标准 USB 接口
 - Xbox One 手柄可以无线工作，但使用 Wi-Fi Direct，目前尚不支持
 - Xbox One 手柄可以是有线的，并使用标准 Micro-USB 接口



## 初代 Xbox USB 适配


将该驱动用于初代 Xbox 手柄需要一根转接线缆，把专有接口的引脚引出USB。你可以在网上以相当便宜的价格买到，或者自己制作

这样的线缆相当容易制作。手柄本身是一USB 复合设备（一个带有三个端口的集线器，用于两个扩展槽和手柄设备），唯一的区别在于其使用了非标准接口 针，而标USB 1.0 接口4 针）

你只需USB 接口焊接到线缆上，并使黄线保持断开。其余引脚在两种接口上的顺序相同，因此没有什么诀窍。关于这些内容的详细信息可以在网上找到（[^1^]_、[^2^]_、[^3^]_）

得益于线缆上trip 分线器，你甚至无需剪断原装线缆。你可以买一根延长线并剪断它来代替。这样，如果你有 Xbox，仍然可以将手柄用于其上 ;)



## 驱动安装


一旦（如有必要）接好转接线缆并连接手柄，xpad 模块应当会被自动加载。为确认这一点，你可cat /sys/kernel/debug/usb/devices。应当会出现类似下面的条目：

   :caption: dump from InterAct PowerPad Pro (Germany)

    T:  Bus=01 Lev=03 Prnt=04 Port=00 Cnt=01 Dev#=  5 Spd=12  MxCh= 0
    D:  Ver= 1.10 Cls=00(>ifc ) Sub=00 Prot=00 MxPS=32 #Cfgs=  1
    P:  Vendor=05fd ProdID=107a Rev= 1.00
    C:* #Ifs= 1 Cfg#= 1 Atr=80 MxPwr=100mA
    I:  If#= 0 Alt= 0 #EPs= 2 Cls=58(unk. ) Sub=42 Prot=00 Driver=(none)
    E:  Ad=81(I) Atr=03(Int.) MxPS=  32 Ivl= 10ms
    E:  Ad=02(O) Atr=03(Int.) MxPS=  32 Ivl= 10ms

   :caption: dump from Redoctane Xbox Dance Pad (US)

    T:  Bus=01 Lev=02 Prnt=09 Port=00 Cnt=01 Dev#= 10 Spd=12  MxCh= 0
    D:  Ver= 1.10 Cls=00(>ifc ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
    P:  Vendor=0c12 ProdID=8809 Rev= 0.01
    S:  Product=XBOX DDR
    C:* #Ifs= 1 Cfg#= 1 Atr=80 MxPwr=100mA
    I:  If#= 0 Alt= 0 #EPs= 2 Cls=58(unk. ) Sub=42 Prot=00 Driver=xpad
    E:  Ad=82(I) Atr=03(Int.) MxPS=  32 Ivl=4ms
    E:  Ad=02(O) Atr=03(Int.) MxPS=  32 Ivl=4ms


## 支持的手


有关支持的手柄及其对应的厂商与产IDs 的完整列表，请参xpad_device[] 数组\ [^4^]_

自历史版0.0.6006-10-10）起，下列设
```

 original Microsoft XBOX controller (US),    vendor=0x045e, product=0x0202
 smaller  Microsoft XBOX controller (US),    vendor=0x045e, product=0x0289
 original Microsoft XBOX controller (Japan), vendor=0x045e, product=0x0285
 InterAct PowerPad Pro (Germany),            vendor=0x05fd, product=0x107a
 RedOctane Xbox Dance Pad (US),              vendor=0x0c12, product=0x8809

```
无法识别Xbox 手柄型号应当可以作为通用 Xbox 手柄工作。无法识别的跳舞毯手柄需要设置模块选项 'dpad_to_buttons'

如果你有无法识别的手柄，请参0.3——未知手柄


## 手动测试


要测试该驱动的功能，你可以使'jstest'

```

    > modprobe xpad
    > modprobe joydev
    > jstest /dev/js0

```
如果你使用的是普通手柄，应当有一行显18 个输入（8 个轴0 个按键），当你移动摇杆或按下按键时其值应当变化。如果你使用的是跳舞毯，则应当显20 个输入（6 个轴4 个按键）

能用吗？那就大功告成;)



## 致谢


我要感谢 ITO Takayuki 在其网站上提供的详细信息：http://euc.jp/periphs/xbox-controller.ja.html

他提供的有用信息，以usb-skeleton iforce 输入驱动（Greg Kroah-Hartman；Vojtech Pavlik）都为快速原型化基本功能提供了很大帮助



## 参考资




## 历史修订


2002-07-16 - Marko Friedemann <mfr@bmx-chemnitz.de>
 - 原始文档

2005-03-19 - Dominic Cerquetti <binary1230@yahoo.com>
 - 新增跳舞毯相关内容，以及新的方向键→轴映

后续修改可通过 'git log --follow Documentation/input/devices/xpad.rst' 查看
