锘。
## ARCnet 硬件



:Author: Avery Pennarun <apenwarr@worldvisions.ca>


   1) 本文件是 arcnet.rst 的补充。有关通用的驱动配置帮助，请阅读该文件
   2) 本文件已不再特定Linux。它或许应该从内核源码中移出。有想法吗？

由于似乎有很多人（包括我）拿到了没有手册ARCnet 网卡，本文件包含了对 ARCnet 硬件的快速介绍、一些布线提示，以及我能找到的所有跳线设置列表。如果你有针对自己特定网卡的任何设置，和/或任何其他信息，请随时发邮件netdev <arcnet-netdev>


## ARCnet 简



ARCnet 是一种网络类型，其工作方式类似于流行Ethernet 网络，但也存在一些非常重要的差异

首先，你可以买到至少两种速度ARCnet 网卡.5 Mbps（比 Ethernet 慢）100 Mbps（比普Ethernet 快）。事实上还有其他速度，但不太常见。据我所知，不同的硬件类型之间互不兼容，因此你不能将 100 Mbps 网卡接到 2.5 Mbps 网卡上，依此类推。据我所闻，我的驱动确实可以配合 100 Mbps 网卡工作，但我自己无法验证这一点，因为我只2.5 Mbps 这一种。它大概无法让你100 Mbps 网卡跑满。别再抱怨了:)

你也无法ARCnet 网卡连接到任何种类的 Ethernet 网卡并指望它能工作

ARCnet 有两种“类型”——星型（STAR）拓扑和总线型（BUS）拓扑。这指的是网卡应该如何连接在一起。根据大多数可用文档，你只能STAR 网卡连接STAR 网卡，将 BUS 网卡连接BUS 网卡。这说得通，对吧？嗯，这并不完全正确；见下文“布线”一节

一旦跨过这些小障碍，ARCnet 实际上是一个相当精心设计的标准。它使用了一种称为“改进的令牌传递（modified token passing）”的机制，这使其与所谓的“令牌环（Token Ring）”网卡完全不兼容，但也使其传输比 Ethernet 可靠得多。事实上，ARCnet 会保证数据包安全到达目的地，即使它无法正确送达（例如由于电缆断裂，或目标计算机不存在），它也至少会通知发送方

由于“令牌”的动作经过精确定义，它总会在最大时长内绕“环”传递一圈。这使其对实时网络很有用

此外，所有已知的 ARCnet 网卡都有（几乎）相同的编程接口。这意味着用一ARCnet 驱动就能支持任何网卡，Ethernet 则每家制造商使用的有时是完全不同的编程接口，导致出现大量不同、有时又非常相似Ethernet 驱动。当然，始终使用相同的编程接口也意味着，当 PCI 总线主控 DMA 这类高性能硬件特性出现时，很难加以利用。这个我们就不展开讨论了

不过，使 ARCnet 网卡难以编程的一点是其数据包大小的限制；标准 ARCnet 只能发送长度不超过 508 字节的数据包。这小于 Internet“最低要求”的 576 字节，更不用Ethernet 1500 字节MTU 了。作为补偿，RFC1201 定义了额外的一层封装（我称之为“数据包拆分”），它允许“虚拟数据包”增大到每个最64K，尽管它们通常保持Ethernet 风格1500 字节

有关 ARCnet 网络的更多信息，请访问“ARCNET Resource Center”WWW 页面

	https://www.arcnet.cc


## ARCnet 网络布线



本节由以下人员重写：

	Vojtech Pavlik     <vojtech@suse.cz>

使用了多人的信息，包括：

 - Avery Pennraun     <apenwarr@worldvisions.ca>
 - Stephen A. Wood    <saw@hallc1.cebaf.gov>
 - John Paul Morrison <jmorriso@bogomips.ee.ubc.ca>
 - Joachim Koenig     <jojo@repas.de>

Vojtech 的请求，Avery 又对其做了一些润色

ARCnet（经典的 2.5 Mbps 版本）可以通过两种不同的电缆连接：同轴电缆和双绞线。其ARCnet 类型的网络（100 Mbps TCNS 以及 320 kbps - 32 Mbps ARCnet Plus）使用不同类型的电缆（Type1、光纤、C1、C4、C5）

对于同轴电缆网络，你“应该”使93 欧姆 RG-62 电缆。但其他电缆也能正常工作，因ARCnet 是一种非常稳定的网络。我个人使用的是 75 欧姆电视天线电缆

用于同轴电缆布线的网卡有两种变体：分别用BUS STAR 网络拓扑。它们大体相同。唯一的区别在于所安装的混合芯片。BUS 网卡使用高阻抗输出，STAR 使用低阻抗。低阻抗网卡（STAR）在电气上等同于接了终端电阻的高阻抗网卡

通常，ARCnet 网络STAR 网卡和集线器（hub）构成。集线器有两种类型——有源和无源。无源集线器是小盒子

```

	   |         | wires
	   R         + junction
	-R-+-R-      R 47 Ohm resistors
	   R
	   |

```

屏蔽层连接在一起。有源集线器要复杂得多；它们有电源，并包含用于放大信号并将其发送到网络其他网段的电子元件。它们通常有八个连接器。有源集线器有两种变体——哑（dumb）和智能（smart）。哑变体只是放大信号，而智能变体会将经过的所有数据包解码为数字再重新编码。如果你在网络中有多个集线器，这种方式要好得多，因为多个哑有源集线器可能会降低信号质量

现在来说说布线。你可以将以下设备连接在一起：

1. 网卡对网卡。这是组建双机网络最简单的方式

2. 网卡对无源集线器。记住，集线器上所有未使用的连接器都必须用 93 欧姆（如果没有合适的，也可以用其他阻值）的终端电阻正确端接

	（Avery 注：哎呀，我当时不知道这点。不过我的（电视电缆）照样能用。）

3. 网卡对有源集线器。这里无需端接未使用的连接器，除非出于某种美观考虑。但是，任意两台计算机之间不能有超过十一个的有源集线器。这当然不限制网络中有源集线器的总数

4. 有源集线器对另一个有源集线器

5. 有源集线器对无源集线器

记住，你不能将两个无源集线器连接在一起。这种连接导致的功率损耗过高，网络无法可靠运行


```

	   R                     S - STAR type card
    S------H--------A-------S    R - Terminator
	   |        |            H - Hub
	   |        |            A - Active hub
	   |   S----H----S
	   S        |
		    |
		    S

```

BUS 拓扑Ethernet 所使用的非常相似。唯一的区别在于电缆和终端电阻：它们应93 欧姆。Ethernet 使用 50 欧姆阻抗。你使用 T 型连接器将计算机接入单根电缆（即总线）。你必须在总线的两端都接上终端电阻

```

    RT----T------T------T------T------TR
     B    B      B      B      B      B

  B - BUS type card
  R - Terminator
  T - T connector

```

但这还不是全部！这两种类型可以连接在一起。根据官方文档，连接它们的唯一方法是使用一个有

```

	 A------T------T------TR
	 |      B      B      B
     S---H---S
	 |
	 S

```

官方文档还指出，你可以在……的末端使用 STAR 网卡

```

     S------T------T------S
	    B      B

```

不过，根据我自己的实验，你可以直接在 STAR 拓扑网络的电缆中间任意位置挂一BUS 类型网卡。更进一步——如果你使用终端电阻，还可以用总线网卡替代任何星型网卡。这样你就能构建出满足所有需求的非常复杂的网络！一

```

				  S
				  |
	   RT------T-------T------H------S
	    B      B       B      |
				  |       R
    S------A------T-------T-------A-------H------TR
	   |      B       B       |       |      B
	   |   S                 BT       |
	   |   |                  |  S----A-----S
    S------H---A----S             |       |
	   |   |      S------T----H---S   |
	   S   S             B    R       S

```

双绞线布线采用了一种基本不同的布线方案。每TP 网卡有两RJ（电话线风格）连接器。然后这些网卡通过连接相邻两张网卡的电缆首尾串联（daisy-chain）在一起。两端使RJ 93 欧姆终端电阻端接，它们插

```

	  ___________   ___________
      _R_|_         _|_|_         _|_R_
     |     |       |     |       |     |
     |Card |       |Card |       |Card |
     |_____|       |_____|       |_____|


```

TP 拓扑也有集线器。使用它们并不困难；你只需TP 链连接到集线器的任意一端，甚至两端都连。这样你就能创建几乎任意的网络配置。网络中任意两台计算机之间最11 个集线器的限制在此同样适用

```

    RP-------P--------P--------H-----P------P-----PR
			       |
      RP-----H--------P--------H-----P------PR
	     |                 |
	     PR                PR

    R - RJ Terminator
    P - TP Card
    H - TP Hub

```

与任何网络一样，ARCnet 的电缆长度有限。以下是两个有源端（有源端指有源集线器或 STAR 网卡）之间的最大电缆长度

		========== ======= ===========
		RG-62       93 Ohm up to 650 m
		RG-59/U     75 Ohm up to 457 m
		RG-11/U     75 Ohm up to 533 m
		IBM Type 1 150 Ohm up to 200 m
		IBM Type 3 100 Ohm up to 100 m
		========== ======= ===========

连接到无源集线器的所有电缆的最大长度对RG-62 布线限制65 米；其他电缆更短。你可以看到，在大型网络中使用无源集线器是个糟糕的主意。单根“BUS 干线（BUS Trunk）”的最大长度对RG-62 约为 300 米。网络中最远两点之间的最大距离限制为 3000 米。两张网集线器之TP 电缆的最大长度为 650 米


## 设置跳线



所ARCnet 网卡总共应有四到五种不同设置

  - I/O 地址：这是你ARCnet 网卡所在的“端口”。Linux ARCnet 驱动中探测的值仅0x200 0x3F0 之间。（如果你的网卡还有其他值，这是可能的，请告诉我。）它不应与系统上任何其他设备相同。根据我Novell 拿到的一份文档，MS Windows 偏好 0x300 或更大的值，否则会吞掉我系统上的网络连接（至少是这样）。我猜这可能是因为，如果你的网卡位于 0x2E0，对 0x2E8 处串行端口的探测会重置该网卡，并很可能把事情搞得一团糟

 - Avery 的最爱：0x300

  - IRQ：在 8 位网卡上，它可能2 (9) 7
	     16 位网卡上，它可能2 (9) 10-15

    确保它与系统上任何其他网卡都不同。注意，Linux 而言，IRQ2 IRQ9 是相同的。你可以“cat /proc/interrupts”获得一份某时刻哪些 IRQ 正在使用的较为完整的列表。以下是 Vojtech Pavlik <vojtech@suse.cz> 提供的常见用途列表：

	（“Not on bus”表示网卡无法产生此中断

	======   =========================================================
	IRQ  0   Timer 0 (Not on bus)
	IRQ  1   Keyboard (Not on bus)
	IRQ  2   IRQ Controller 2 (Not on bus, nor does interrupt the CPU)
	IRQ  3   COM2
	IRQ  4   COM1
	IRQ  5   FREE (LPT2 if you have it; sometimes COM3; maybe PLIP)
	IRQ  6   Floppy disk controller
	IRQ  7   FREE (LPT1 if you don't use the polling driver; PLIP)
	IRQ  8   Realtime Clock Interrupt (Not on bus)
	IRQ  9   FREE (VGA vertical sync interrupt if enabled)
	IRQ 10   FREE
	IRQ 11   FREE
	IRQ 12   FREE
	IRQ 13   Numeric Coprocessor (Not on bus)
	IRQ 14   Fixed Disk Controller
	IRQ 15   FREE (Fixed Disk Controller 2 if you have it)
	======   =========================================================



```

	   IRQ 9 is used on some video cards for the "vertical retrace"
	   interrupt.  This interrupt would have been handy for things like
	   video games, as it occurs exactly once per screen refresh, but
	   unfortunately IBM cancelled this feature starting with the original
	   VGA and thus many VGA/SVGA cards do not support it.  For this
	   reason, no modern software uses this interrupt and it can almost
	   always be safely disabled, if your video card supports it at all.

	If your card for some reason CANNOT disable this IRQ (usually there
	is a jumper), one solution would be to clip the printed circuit
	contact on the board: it's the fourth contact from the left on the
	back side.  I take no responsibility if you try this.

	- Avery's favourite: IRQ2 (actually IRQ9).  Watch that VGA, though.

  - the memory address:  Unlike most cards, ARCnets use "shared memory" for
    copying buffers around.  Make SURE it doesn't conflict with any other
    used memory in your system!

    ::

	A0000		- VGA graphics memory (ok if you don't have VGA)
	B0000		- Monochrome text mode
	C0000		\  One of these is your VGA BIOS - usually C0000.
	E0000		/
	F0000		- System BIOS

    Anything less than 0xA0000 is, well, a BAD idea since it isn't above
    640k.

	- Avery's favourite: 0xD0000

  - the station address:  Every ARCnet card has its own "unique" network
    address from 0 to 255.  Unlike Ethernet, you can set this address
    yourself with a jumper or switch (or on some cards, with special
    software).  Since it's only 8 bits, you can only have 254 ARCnet cards
    on a network.  DON'T use 0 or 255, since these are reserved (although
    neat stuff will probably happen if you DO use them).  By the way, if you
    haven't already guessed, don't set this the same as any other ARCnet on
    your network!

	- Avery's favourite:  3 and 4.  Not that it matters.

  - There may be ETS1 and ETS2 settings.  These may or may not make a
    difference on your card (many manuals call them "reserved"), but are
    used to change the delays used when powering up a computer on the
    network.  This is only necessary when wiring VERY long range ARCnet
    networks, on the order of 4km or so; in any case, the only real
    requirement here is that all cards on the network with ETS1 and ETS2
    jumpers have them in the same position.  Chris Hindy <chrish@io.org>
    sent in a chart with actual values for this:

	======= ======= =============== ====================
	ET1	ET2	Response Time	Reconfiguration Time
	======= ======= =============== ====================
	open	open	74.7us		840us
	open	closed	283.4us		1680us
	closed	open	561.8us		1680us
	closed	closed	1118.6us	1680us
	======= ======= =============== ====================

    Make sure you set ETS1 and ETS2 to the SAME VALUE for all cards on your
    network.

```

此外，许多网卡上（虽然不是我的）有红色和绿色 LED。Vojtech Pavlik <vojtech@suse.cz> 告诉我它们的含义

	=============== =============== =====================================
	GREEN           RED             Status
	=============== =============== =====================================
	OFF             OFF             Power off
	OFF             Short flashes   Cabling problems (broken cable or not
					terminated)
	OFF (short)     ON              Card init
	ON              ON              Normal state - everything OK, nothing
					happens
	ON              Long flashes    Data transfer
	ON              OFF             Never happens (maybe when wrong ID)
	=============== =============== =====================================


以下是人们发给我的关于他们各自特ARCnet 网卡的全部具体信息。它简直是一团糟，包含大量重复信息。我没时间去整理它。如果你想整理，请务必动手！只需把你所做更改的“diff -u”发给我即可

型号 # 列在该网卡具体说明的正上方，因此你应该能够使用文本查看器的“search”功能找到你想要的条目。如果你不知道自己拥有何种网卡，试着翻看各种图示，看看能否辨认出来

如果你的型号没有列出或设置不同，请务必告诉我。我不得不在没有手册的情况下自己琢磨出来，那可一点都不好玩！

即使你的 ARCnet 型号没有列出，但跳线与另一个已列出的型号相同，也请发邮件告诉我

本文件中列出的网卡（大致按此顺序）：

	=============== ======================= ====
	Manufacturer	Model #			Bits
	=============== ======================= ====
	SMC		PC100			8
	SMC		PC110			8
	SMC		PC120			8
	SMC		PC130			8
	SMC		PC270E			8
	SMC		PC500			16
	SMC		PC500Longboard		16
	SMC		PC550Longboard		16
	SMC		PC600			16
	SMC		PC710			8
	SMC?		LCS-8830(-T)		8/16
	Puredata	PDI507			8
	CNet Tech	CN120-Series		8
	CNet Tech	CN160-Series		16
	Lantech?	UM9065L chipset		8
	Acer		5210-003		8
	Datapoint?	LAN-ARC-8		8
	Topware		TA-ARC/10		8
	Thomas-Conrad	500-6242-0097 REV A	8
	Waterloo?	(C)1985 Waterloo Micro. 8
	No Name		--			8/16
	No Name		Taiwan R.O.C?		8
	No Name		Model 9058		8
	Tiara		Tiara Lancard?		8
	=============== ======================= ====


- SMC = Standard Microsystems Corp（标准微系统公司）
- CNet Tech = CNet Technology, Inc.（CNet 科技公司）

## 未分类内



  - 请发送你能找到的任何其他信息


```

     From: root@ultraworld.xs4all.nl (Timo Hilbrink)
     To: apenwarr@foxnet.net (Avery Pennarun)
     Date: Wed, 26 Oct 1994 02:10:32 +0000 (GMT)
     Reply-To: timoh@xs4all.nl

     [...parts deleted...]

     About the jumpers: On my PC130 there is one more jumper, located near the
     cable-connector and it's for changing to star or bus topology;
     closed: star - open: bus
     On the PC500 are some more jumper-pins, one block labeled with RX,PDN,TXI
     and another with ALE,LA17,LA18,LA19 these are undocumented..

     [...more parts deleted...]

     --- CUT ---

```

## 标准微系统公司（SMC



### PC100、PC110、PC120、PC130 位网卡）以及 PC500、PC6006 位网卡）



  - 主要来自 Avery Pennarun <apenwarr@worldvisions.ca>。所示数值取Avery 的配置
  - 特别感谢 Timo Hilbrink <timoh@xs4all.nl> 指出 PC1203000 600 Avery PC100 具有相同的开关。不PC500/600 有几个额外的、未文档化的引脚。（)
  - PC110 的设置已Stephen A. Wood <saw@cebaf.gov> 验证
  - 另外，JP- S- 编号可能与你的网卡不完全对应。试着寻找具有同样数量设置的跳开关——这样可能更可靠


```

	     JP5		       [|]    :    :    :    :
	(IRQ Setting)		      IRQ2  IRQ3 IRQ4 IRQ5 IRQ7
			Put exactly one jumper on exactly one set of pins.


				  1  2   3  4  5  6   7  8  9 10
	     S1                /----------------------------------\
	(I/O and Memory        |  1  1 * 0  0  0  0 * 1  1  0  1  |
	 addresses)            \----------------------------------/
				  |--|   |--------|   |--------|
				  (a)       (b)           (m)

			WARNING.  It's very important when setting these which way
			you're holding the card, and which way you think is '1'!

			If you suspect that your settings are not being made
			correctly, try reversing the direction or inverting the
			switch positions.

			a: The first digit of the I/O address.
				Setting		Value
				-------		-----
				00		0
				01		1
				10		2
				11		3

			b: The second digit of the I/O address.
				Setting		Value
				-------		-----
				0000		0
				0001		1
				0010		2
				...		...
				1110		E
				1111		F

			The I/O address is in the form ab0.  For example, if
			a is 0x2 and b is 0xE, the address will be 0x2E0.

			DO NOT SET THIS LESS THAN 0x200!!!!!


			m: The first digit of the memory address.
				Setting		Value
				-------		-----
				0000		0
				0001		1
				0010		2
				...		...
				1110		E
				1111		F

			The memory address is in the form m0000.  For example, if
			m is D, the address will be 0xD0000.

			DO NOT SET THIS TO C0000, F0000, OR LESS THAN A0000!

				  1  2  3  4  5  6  7  8
	     S2                /--------------------------\
	(Station Address)      |  1  1  0  0  0  0  0  0  |
			       \--------------------------/

				Setting		Value
				-------		-----
				00000000	00
				10000000	01
				01000000	02
				...
				01111111	FE
				11111111	FF

			Note that this is binary with the digits reversed!

			DO NOT SET THIS TO 0 OR 255 (0xFF)!


```

### PC130E/PC270E 位网卡）



  - 来自 Juergen Seifert <seifert@htwm.de>

本描述由 Juergen Seifert <seifert@htwm.de> 根据以下原始 SMC 手册撰写

	     "Configuration Guide for ARCNET(R)-PC130E/PC270 Network
	     Controller Boards Pub. # 900.044A June, 1989"

ARCnet Datapoint Corporation 的注册商
SMC Standard Microsystems Corporation 的注册商

PC130E PC130 板卡的增强版本，配备标准BNC 母座连接器，用于连接 RG-62/U 同轴电缆。由于该板卡既设计用于星型网络中的点到点连接，也设计用于总线网络连接，因此它向下兼容所有为同轴网络设计的其他标准板卡（PC120、PC110 PC100 星型拓扑板卡，以PC220、PC210 PC200 总线拓扑板卡）

PC270E PC260 板卡的增强版本，配备两个模块化的 RJ11 型插孔，用于连接双绞线布线。它可用于星型网络或菊花链网络


```

	 8 7 6 5 4 3 2 1
    ________________________________________________________________
   |   |       S1        |                                          |
   |   |_________________|                                          |
   |    Offs|Base |I/O Addr                                         |
   |     RAM Addr |                                              ___|
   |         ___  ___                                       CR3 |___|
   |        |   \/   |                                      CR4 |___|
   |        |  PROM  |                                           ___|
   |        |        |                                        N |   | 8
   |        | SOCKET |                                        o |   | 7
   |        |________|                                        d |   | 6
   |                   ___________________                    e |   | 5
   |                  |                   |                   A | S | 4
   |       |oo| EXT2  |                   |                   d | 2 | 3
   |       |oo| EXT1  |       SMC         |                   d |   | 2
   |       |oo| ROM   |      90C63        |                   r |___| 1
   |       |oo| IRQ7  |                   |               |o|  _____|
   |       |oo| IRQ5  |                   |               |o| | J1  |
   |       |oo| IRQ4  |                   |              STAR |_____|
   |       |oo| IRQ3  |                   |                   | J2  |
   |       |oo| IRQ2  |___________________|                   |_____|
   |___                                               ______________|
       |                                             |
       |_____________________________________________|

```
```

  SMC 90C63	ARCNET Controller / Transceiver /Logic
  S1	1-3:	I/O Base Address Select
	4-6:	Memory Base Address Select
	7-8:	RAM Offset Select
  S2	1-8:	Node ID Select
  EXT		Extended Timeout Select
  ROM		ROM Enable Select
  STAR		Selected - Star Topology	(PC130E only)
		Deselected - Bus Topology	(PC130E only)
  CR3/CR4	Diagnostic LEDs
  J1		BNC RG62/U Connector		(PC130E only)
  J1		6-position Telephone Jack	(PC270E only)
  J2		6-position Telephone Jack	(PC270E only)

```

将某个开关设Off/Open 表示”，On/Closed 表示”


##### 设置节点 ID



S2 中的八个开关用于设置节ID。这些开关的工作方式PC100 系列网卡类似；更多信息请参阅该条目


##### 设置 I/O 基址



开关组 S1 的前三个开关用于选择其中之一

```


   Switch | Hex I/O
   1 2 3  | Address
   -------|--------
   0 0 0  |  260
   0 0 1  |  290
   0 1 0  |  2E0  (Manufacturer's default)
   0 1 1  |  2F0
   1 0 0  |  300
   1 0 1  |  350
   1 1 0  |  380
   1 1 1  |  3E0


```

##### 设置基址内存（RAM）缓冲区地址



内存缓冲区需16K RAM 块中2K。这16K 块的基址可以位于八个位置中的任意一个。开关组 S1 的开4-6 选择 16K 块的基址。在16K 地址空间内，缓冲区可被分配到四个位置中的任意一个，由偏移量（开关组 S1 的开7 8）决定


```

   Switch     | Hex RAM | Hex ROM
   4 5 6  7 8 | Address | Address *)
   -----------|---------|-----------
   0 0 0  0 0 |  C0000  |  C2000
   0 0 0  0 1 |  C0800  |  C2000
   0 0 0  1 0 |  C1000  |  C2000
   0 0 0  1 1 |  C1800  |  C2000
	      |         |
   0 0 1  0 0 |  C4000  |  C6000
   0 0 1  0 1 |  C4800  |  C6000
   0 0 1  1 0 |  C5000  |  C6000
   0 0 1  1 1 |  C5800  |  C6000
	      |         |
   0 1 0  0 0 |  CC000  |  CE000
   0 1 0  0 1 |  CC800  |  CE000
   0 1 0  1 0 |  CD000  |  CE000
   0 1 0  1 1 |  CD800  |  CE000
	      |         |
   0 1 1  0 0 |  D0000  |  D2000  (Manufacturer's default)
   0 1 1  0 1 |  D0800  |  D2000
   0 1 1  1 0 |  D1000  |  D2000
   0 1 1  1 1 |  D1800  |  D2000
	      |         |
   1 0 0  0 0 |  D4000  |  D6000
   1 0 0  0 1 |  D4800  |  D6000
   1 0 0  1 0 |  D5000  |  D6000
   1 0 0  1 1 |  D5800  |  D6000
	      |         |
   1 0 1  0 0 |  D8000  |  DA000
   1 0 1  0 1 |  D8800  |  DA000
   1 0 1  1 0 |  D9000  |  DA000
   1 0 1  1 1 |  D9800  |  DA000
	      |         |
   1 1 0  0 0 |  DC000  |  DE000
   1 1 0  0 1 |  DC800  |  DE000
   1 1 0  1 0 |  DD000  |  DE000
   1 1 0  1 1 |  DD800  |  DE000
	      |         |
   1 1 1  0 0 |  E0000  |  E2000
   1 1 1  0 1 |  E0800  |  E2000
   1 1 1  1 0 |  E1000  |  E2000
   1 1 1  1 1 |  E1800  |  E2000

  *) To enable the 8K Boot PROM install the jumper ROM.
     The default is jumper ROM not installed.


```

##### 设置超时与中



标有 EXT1 EXT2 的跳线用于确定超时参数。这两个跳线通常保持断开（open）

要选择一个硬件中断级别，请设置跳IRQ2、IRQ3、IRQ4、IRQ5、IRQ7 中的一个（且只能一个！）。制造商默认值为 IRQ2


##### PC130E 配置星型或总线型拓



单个标有 STAR 的跳线用于为 PC130E 板卡配置星型或总线型拓扑。安装该跳线时，板卡可用于星型网络；移除该跳线时，板卡可用于总线型拓扑


##### 诊断 LED



板卡后挡板上可见两个诊断 LED。绿LED 监视网络活动；红LED 显示

```

 Green  | Status               Red      | Status
 -------|-------------------   ---------|-------------------
  on    | normal activity      flash/on | data transfer
  blink | reconfiguration      off      | no data transfer;
  off   | defective board or            | incorrect memory or
	| node ID is zero               | I/O address


```

### PC500/PC550 Longboard6 位网卡）



  - 来自 Juergen Seifert <seifert@htwm.de>



```

      There is another Version of the PC500 called Short Version, which
      is different in hard- and software! The most important differences
      are:

      - The long board has no Shared memory.
      - On the long board the selection of the interrupt is done by binary
	coded switch, on the short board directly by jumper.

```

[Avery 注：请特别留意这一点：长板没有共享内存。这意味着当前Linux-ARCnet 驱动无法使用这些网卡。我已经弄到一PC500Longboard，将来会对其做一些实验，但别太期待。再次感Juergen Seifert 的建议！]

本描述由 Juergen Seifert <seifert@htwm.de> 根据以下原始 SMC 手册撰写

	 "Configuration Guide for SMC ARCNET-PC500/PC550
	 Series Network Controller Boards Pub. # 900.033 Rev. A
	 November, 1989"

ARCnet Datapoint Corporation 的注册商
SMC Standard Microsystems Corporation 的注册商

PC500 配备标准BNC 母座连接器，用于连接 RG-62/U 同轴电缆。该板卡既设计用于星型网络中的点到点连接，也设计用于总线网络连接

PC550 配备两个模块化的 RJ11 型插孔，用于连接双绞线布线。它可用于星型网络或菊花链（BUS）网络


```

       1
       0 9 8 7 6 5 4 3 2 1     6 5 4 3 2 1
    ____________________________________________________________________
   < |         SW1         | |     SW2     |                            |
   > |_____________________| |_____________|                            |
   <   IRQ    |I/O Addr                                                 |
   >                                                                 ___|
   <                                                            CR4 |___|
   >                                                            CR3 |___|
   <                                                                 ___|
   >                                                              N |   | 8
   <                                                              o |   | 7
   >                                                              d | S | 6
   <                                                              e | W | 5
   >                                                              A | 3 | 4
   <                                                              d |   | 3
   >                                                              d |   | 2
   <                                                              r |___| 1
   >                                                        |o|    _____|
   <                                                        |o|   | J1  |
   >  3 1                                                   JP6   |_____|
   < |o|o| JP2                                                    | J2  |
   > |o|o|                                                        |_____|
   <  4 2__                                               ______________|
   >    |  |                                             |
   <____|  |_____________________________________________|

```
```

  SW1	1-6:	I/O Base Address Select
	7-10:	Interrupt Select
  SW2	1-6:	Reserved for Future Use
  SW3	1-8:	Node ID Select
  JP2	1-4:	Extended Timeout Select
  JP6		Selected - Star Topology	(PC500 only)
		Deselected - Bus Topology	(PC500 only)
  CR3	Green	Monitors Network Activity
  CR4	Red	Monitors Board Activity
  J1		BNC RG62/U Connector		(PC500 only)
  J1		6-position Telephone Jack	(PC550 only)
  J2		6-position Telephone Jack	(PC550 only)

```

将某个开关设Off/Open 表示”，On/Closed 表示”


##### 设置节点 ID



SW3 中的八个开关用于设置节ID。连接到网络的每个节点都必须有一个唯一的节ID，且必须不同0。开1 作为最低有效位（LSB）

节点 ID 是所有设为”的开关值之

```

    Switch | Value
    -------|-------
      1    |   1
      2    |   2
      3    |   4
      4    |   8
      5    |  16
      6    |  32
      7    |  64
      8    | 128

```
```

    Switch         | Hex     | Decimal
   8 7 6 5 4 3 2 1 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 设置 I/O 基址



开关组 SW1 的前六个开关用于选择其中之一

```

   Switch       | Hex I/O
   6 5  4 3 2 1 | Address
   -------------|--------
   0 1  0 0 0 0 |  200
   0 1  0 0 0 1 |  210
   0 1  0 0 1 0 |  220
   0 1  0 0 1 1 |  230
   0 1  0 1 0 0 |  240
   0 1  0 1 0 1 |  250
   0 1  0 1 1 0 |  260
   0 1  0 1 1 1 |  270
   0 1  1 0 0 0 |  280
   0 1  1 0 0 1 |  290
   0 1  1 0 1 0 |  2A0
   0 1  1 0 1 1 |  2B0
   0 1  1 1 0 0 |  2C0
   0 1  1 1 0 1 |  2D0
   0 1  1 1 1 0 |  2E0 (Manufacturer's default)
   0 1  1 1 1 1 |  2F0
   1 1  0 0 0 0 |  300
   1 1  0 0 0 1 |  310
   1 1  0 0 1 0 |  320
   1 1  0 0 1 1 |  330
   1 1  0 1 0 0 |  340
   1 1  0 1 0 1 |  350
   1 1  0 1 1 0 |  360
   1 1  0 1 1 1 |  370
   1 1  1 0 0 0 |  380
   1 1  1 0 0 1 |  390
   1 1  1 0 1 0 |  3A0
   1 1  1 0 1 1 |  3B0
   1 1  1 1 0 0 |  3C0
   1 1  1 1 0 1 |  3D0
   1 1  1 1 1 0 |  3E0
   1 1  1 1 1 1 |  3F0


```

##### 设置中断



开关组 SW1 的开关七到十用于选择中断级别。中断级别为二进制编码，因此理论上可选择 0 15，但只支持以下八个值：3012


```

   Switch   | IRQ
   10 9 8 7 |
   ---------|--------
    0 0 1 1 |  3
    0 1 0 0 |  4
    0 1 0 1 |  5
    0 1 1 1 |  7
    1 0 0 1 |  9 (=2) (default)
    1 0 1 0 | 10
    1 0 1 1 | 11
    1 1 0 0 | 12


```

##### 设置超时



两个跳线 JP2-4）用于确定超时参数。这两个跳线通常保持断开（open）。有关替代配置，请参COM9026 数据手册


##### PC500 配置星型或总线型拓



单个标有 JP6 的跳线用于为 PC500 板卡配置星型或总线型拓扑。安装该跳线时，板卡可用于星型网络；移除该跳线时，板卡可用于总线型拓扑


##### 诊断 LED



板卡后挡板上可见两个诊断 LED。绿LED 监视网络活动；红LED 显示

```

 Green  | Status               Red      | Status
 -------|-------------------   ---------|-------------------
  on    | normal activity      flash/on | data transfer
  blink | reconfiguration      off      | no data transfer;
  off   | defective board or            | incorrect memory or
	| node ID is zero               | I/O address


```

### PC710 位网卡）



  - 来自 J.S. van Oosten <jvoosten@compiler.tdcnet.nl>

注意：这些数据是通过实验并参考其他网卡的信息收集的。不过，我确信我搞对99% 的设置

SMC710 网卡类似PC270 网卡，但要基础得多（即没有

```

    _______________________________________
   | +---------+  +---------+              |____
   | |   S2    |  |   S1    |              |
   | +---------+  +---------+              |
   |                                       |
   |  +===+    __                          |
   |  | R |   |  | X-tal                 ###___
   |  | O |   |__|                      ####__'|
   |  | M |    ||                        ###
   |  +===+                                |
   |                                       |
   |   .. JP1   +----------+               |
   |   ..       | big chip |               |
   |   ..       |  90C63   |               |
   |   ..       |          |               |
   |   ..       +----------+               |
    -------                     -----------
	   |||||||||||||||||||||

```

JP1 处的跳线排实际上8 个跳线组成，（有时标注为）与 PC270 上相同的，从上到下依次为：EXT2、EXT1、ROM、IRQ7、IRQ5、IRQ4、IRQ3、IRQ2（嘿，猜猜它们是干嘛用的:-) 

S1 S2 的功能与 PC270 上相同，只是编号互换了（S1 是节点地址，S2 设置 IO RAM 地址）

我知道它连接PC110 类型ARCnet 板卡时可以正常工作


*****************************************************************************

## 可能SMC



### LCS-8830(-T) 位与 16 位网卡）



  - 来自 Mathias Katzer <mkatzer@HRZ.Uni-Bielefeld.DE>
  - Marek Michalkiewicz <marekm@i17linuxb.ists.pwr.wroc.pl> 指出
    LCS-8830 LCS-8830-T 略有不同。它们是 8 位、仅总线型（JP0 跳线为硬连线），且仅 BNC

这是我认为是 SMC 制造的 LCS-8830-TSMC' 只出现在一PLCC 上，别处都没有，连手册里那几张复印纸中也没有）


```

     ------------------------------------
    |                                    |
    |              JP3 88  8 JP2         |
    |       #####      | \               |
    |       #####    ET1 ET2          ###|
    |                              8  ###|
    |  U3   SW 1                  JP0 ###|  Phone Jacks
    |  --                             ###|
    | |  |                               |
    | |  |   SW2                         |
    | |  |                               |
    | |  |  #####                        |
    |  --   #####                       ####  BNC Connector
    |                                   ####
    |   888888 JP1                       |
    |   234567                           |
     --                           -------
       |||||||||||||||||||||||||||
	--------------------------


  SW1: DIP-Switches for Station Address
  SW2: DIP-Switches for Memory Base and I/O Base addresses

  JP0: If closed, internal termination on (default open)
  JP1: IRQ Jumpers
  JP2: Boot-ROM enabled if closed
  JP3: Jumpers for response timeout

  U3: Boot-ROM Socket


  ET1 ET2     Response Time     Idle Time    Reconfiguration Time

		 78                86               840
   X            285               316              1680
       X        563               624              1680
   X   X       1130              1237              1680

  (X means closed jumper)

  (DIP-Switch downwards means "0")

```

站地址SW1 以二进制编码

I/O 基址SW2 DIP 开6 编码

========	========
Switches        Base
678             Address
========	========
000		260-26f
100		290-29f
010		2e0-2ef
110		2f0-2ff
001		300-30f
101		350-35f
011		380-38f
111 		3e0-3ef
========	========


SW2 DIP 开1-5 编码 RAM ROM 地址范围

========        ============= ================
Switches        RAM           ROM
12345           Address Range  Address Range
========        ============= ================
00000		C:0000-C:07ff	C:2000-C:3fff
10000		C:0800-C:0fff
01000		C:1000-C:17ff
11000		C:1800-C:1fff
00100		C:4000-C:47ff	C:6000-C:7fff
10100		C:4800-C:4fff
01100		C:5000-C:57ff
11100		C:5800-C:5fff
00010		C:C000-C:C7ff	C:E000-C:ffff
10010		C:C800-C:Cfff
01010		C:D000-C:D7ff
11010		C:D800-C:Dfff
00110		D:0000-D:07ff	D:2000-D:3fff
10110		D:0800-D:0fff
01110		D:1000-D:17ff
11110		D:1800-D:1fff
00001		D:4000-D:47ff	D:6000-D:7fff
10001		D:4800-D:4fff
01001		D:5000-D:57ff
11001		D:5800-D:5fff
00101		D:8000-D:87ff	D:A000-D:bfff
10101		D:8800-D:8fff
01101		D:9000-D:97ff
11101		D:9800-D:9fff
00011		D:C000-D:c7ff	D:E000-D:ffff
10011		D:C800-D:cfff
01011		D:D000-D:d7ff
11011		D:D800-D:dfff
00111		E:0000-E:07ff	E:2000-E:3fff
10111		E:0800-E:0fff
01111		E:1000-E:17ff
11111		E:1800-E:1fff
========        ============= ================


## PureData Corp



### PDI507 位网卡）



  - 来自 Mark Rejhon <mdrejhon@magi.com>（Avery 略有修改
  - Avery 注：我认PDI508 网卡（但肯定不是 PDI508Plus 网卡）与此基本相同的。PDI508Plus 网卡似乎主要是软件配置的

跳线

	网卡底部、靠近边缘连接器处有一组跳线阵列。该阵列标注J1。它们控IRQ 和其他某些功能。只IRQ 引脚上放一个跳线

	ETS1、ETS2 用于远距离网络的时序。请参阅本文件顶部附近的更通用信息

	J2 是一个两引脚的跳线。应该在上面放一个跳线，因为我拿到卡时上面就已经有了。不过我不知道这个跳线是做什么的

	J3 是一个两跳线的阵列。我不知道它是做什么的，但我拿到卡时上面已经有两个跳线了。它是一个两行三列、共六个引脚的栅格。这些跳线是

```

	   .-------.
	 o | o   o |
	   :-------:    ------> Accessible end of card with connectors
	 o | o   o |             in this direction ------->
	   `-------'

```

Carl de Billy <CARL@carainfo.com> 解释J3 J4


```

	   .-------.
	 o | o   o |
	   :-------:    TWIST Technology
	 o | o   o |
	   `-------'
	   .-------.
	   | o   o | o
	   :-------:    COAX Technology
	   | o   o | o
	   `-------'

  - If using coax cable in a bus topology the J4 jumper must be removed;
    place it on one pin.

  - If using bus topology with twisted pair wiring move the J3
    jumpers so they connect the middle pin and the pins closest to the RJ11
    Connectors.  Also the J4 jumper must be removed; place it on one pin of
    J4 jumper for storage.

  - If using  star topology with twisted pair wiring move the J3
    jumpers so they connect the middle pin and the pins closest to the RJ11
    connectors.


```

DIP 开关：

	安装网卡时，在网卡可触及的一端可访问DIP 开关用于设ARCnet 地址。共8 个开关。请使用 1 254 之间的地址

	==========      =========================
	Switch No.	ARCnet address
	12345678
	==========      =========================
	00000000	FF  	(Don't use this!)
	00000001	FE
	00000010	FD
	...
	11111101	2
	11111110	1
	11111111	0	(Don't use this!)
	==========      =========================

	网卡顶部还有另一组八DIP 开关。其中有五个标注MS0-MS4，似乎控制内存地址；另外三个标注为 IO0-IO2，似乎控制网卡的 I/O 基址

	通过试错来测试这一点很困难，而且 I/O 地址的顺序很奇怪。测试方法为：设DIP 开关，重启计算机，并尝试以各种地址（主要在 0x200 0x400 之间）加ARCETHER。导致红色发LED 闪烁的地址，就是我认为可用的地址

	另外，地址 0x3D0 似乎有特殊含义，因为 ARCETHER 包驱动可以正常加载，但红LED 不闪烁。不过我不知0x3D0 是做什么的。我建议使用 0x300 地址，因Windows 可能不喜欢低0x300 的地址

	=============   ===========
	IO Switch No.   I/O address
	210
	=============   ===========
	111             0x260
	110             0x290
	101             0x2E0
	100             0x2F0
	011             0x300
	010             0x350
	001             0x380
	000             0x3E0
	=============   ===========

	内存开关设置一0x1000 字节x100 段单位，4k）的保留地址空间。例如，如果我设置地址 0xD000，它将使0xD000 0xD100 的地址

	内存开关是通过使用 QEMM386 stealth 启动，并LOADHI 查看哪些地址自动从上位内存区域中被排除，然后尝试用这些地址加载 ARCETHER 来测试的

	我建议使ARCnet 内存地址 0xD000，并QEMM stealth 模式下将 EMS 页帧放在 0xC000，这样你就能0xD100 开始获得几乎一直到兆字节末端的连续高位内存

	内存开0（MS0）在我的卡上设为 OFF 时似乎不能正常工作。它可能是我的卡上出了故障。先试着将其设为 ON，如果不行，再设OFF。（它可能是 0x200 位的修饰位？

	=============   ============================================
	MS Switch No.
	43210           Memory address
	=============   ============================================
	00001           0xE100  (guessed - was not detected by QEMM)
	00011           0xE000  (guessed - was not detected by QEMM)
	00101           0xDD00
	00111           0xDC00
	01001           0xD900
	01011           0xD800
	01101           0xD500
	01111           0xD400
	10001           0xD100
	10011           0xD000
	10101           0xCD00
	10111           0xCC00
	11001           0xC900 (guessed - crashes tested system)
	11011           0xC800 (guessed - crashes tested system)
	11101           0xC500 (guessed - crashes tested system)
	11111           0xC400 (guessed - crashes tested system)
	=============   ============================================


## CNet Technology Inc.位网卡）


### 120 系列位网卡）


  - 来自 Juergen Seifert <seifert@htwm.de>

本说明由 Juergen Seifert <seifert@htwm.de> 根据以下 CNet 原版手册编写

	      "ARCNET USER'S MANUAL for
	      CN120A
	      CN120AB
	      CN120TP
	      CN120ST
	      CN120SBT
	      P/N:12-01-0007
	      Revision 3.00"

ARCNET Datapoint Corporation 的注册商

- P/N 120A   ARCNET 8XT/AT 星型
- P/N 120AB  ARCNET 8XT/AT 总线
- P/N 120TP  ARCNET 8XT/AT 双绞
- P/N 120ST  ARCNET 8XT/AT 星型、双绞线
- P/N 120SBT ARCNET 8XT/AT 星型、总线、双绞线


```

    __________________________________________________________________
   |                                                                  |
   |                                                               ___|
   |                                                          LED |___|
   |                                                               ___|
   |                                                            N |   | ID7
   |                                                            o |   | ID6
   |                                                            d | S | ID5
   |                                                            e | W | ID4
   |                     ___________________                    A | 2 | ID3
   |                    |                   |                   d |   | ID2
   |                    |                   |  1 2 3 4 5 6 7 8  d |   | ID1
   |                    |                   | _________________ r |___| ID0
   |                    |      90C65        ||       SW1       |  ____|
   |  JP 8 7            |                   ||_________________| |    |
   |    |o|o|  JP1      |                   |                    | J2 |
   |    |o|o|  |oo|     |                   |         JP 1 1 1   |    |
   |   ______________   |                   |            0 1 2   |____|
   |  |  PROM        |  |___________________|           |o|o|o|  _____|
   |  >  SOCKET      |  JP 6 5 4 3 2                    |o|o|o| | J1  |
   |  |______________|    |o|o|o|o|o|                   |o|o|o| |_____|
   |_____                 |o|o|o|o|o|                   ______________|
	 |                                             |
	 |_____________________________________________|

```
```

  90C65       ARCNET Probe
  S1  1-5:    Base Memory Address Select
      6-8:    Base I/O Address Select
  S2  1-8:    Node ID Select (ID0-ID7)
  JP1     ROM Enable Select
  JP2     IRQ2
  JP3     IRQ3
  JP4     IRQ4
  JP5     IRQ5
  JP6     IRQ7
  JP7/JP8     ET1, ET2 Timeout Parameters
  JP10/JP11   Coax / Twisted Pair Select  (CN120ST/SBT only)
  JP12        Terminator Select       (CN120AB/ST/SBT only)
  J1      BNC RG62/U Connector        (all except CN120TP)
  J2      Two 6-position Telephone Jack   (CN120TP/ST/SBT only)

```

将某个开关拨Off（关）表"1"，拨On（开）表"0"


##### 设置节点 ID


SW2 中的八个开关用于设置节ID。连接到网络的每个节点必须具有唯一的节ID，且必须不同0 号开关（ID0）作为最低有效位（LSB）

节点 ID 是所有拨"1" 的开关取值之
这些取值为

```

    Switch         | Hex     | Decimal
   8 7 6 5 4 3 2 1 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 设置 I/O 基地址


开关块 SW1 中的最后三个开关用于选择一


```


   Switch      | Hex I/O
    6   7   8  | Address
   ------------|--------
   ON  ON  ON  |  260
   OFF ON  ON  |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   OFF OFF ON  |  2F0
   ON  ON  OFF |  300
   OFF ON  OFF |  350
   ON  OFF OFF |  380
   OFF OFF OFF |  3E0


```

##### 设置基址内存（RAM）缓冲区地址


内存缓冲区（RAM）需2K。该缓冲区的基址可位于八个位置中的任意一个。Boot Prom 的地址为内存基址 + 8K 或内存基址 + 0x2000
开关块 SW1 1-5 号开关用于选择内存基址


```

   Switch              | Hex RAM | Hex ROM
    1   2   3   4   5  | Address | Address *)
   --------------------|---------|-----------
   ON  ON  ON  ON  ON  |  C0000  |  C2000
   ON  ON  OFF ON  ON  |  C4000  |  C6000
   ON  ON  ON  OFF ON  |  CC000  |  CE000
   ON  ON  OFF OFF ON  |  D0000  |  D2000  (Manufacturer's default)
   ON  ON  ON  ON  OFF |  D4000  |  D6000
   ON  ON  OFF ON  OFF |  D8000  |  DA000
   ON  ON  ON  OFF OFF |  DC000  |  DE000
   ON  ON  OFF OFF OFF |  E0000  |  E2000

  *) To enable the Boot ROM install the jumper JP1

```


      Since the switches 1 and 2 are always set to ON it may be possible
      that they can be used to add an offset of 2K, 4K or 6K to the base
      address, but this feature is not documented in the manual and I
      haven't tested it yet.


##### 设置中断


要选择一个硬件中断级别，请安装其中一个（且只能安装一个！）跳


```

   Jumper | IRQ
   -------|-----
     2    |  2
     3    |  3
     4    |  4
     5    |  5
     6    |  7


```

##### CN120AB/TP/SBT 上设置内部终端电




```

			 -----
       0                |  0  |
     -----   ON         |     |  ON
    |  0  |             |  0  |
    |     |  OFF         -----   OFF
    |  0  |                0
     -----
   Terminator          Terminator
    disabled            enabled


```

##### CN120ST/SBT 上选择连接器类




```

     JP10    JP11        JP10    JP11
			 -----   -----
       0       0        |  0  | |  0  |
     -----   -----      |     | |     |
    |  0  | |  0  |     |  0  | |  0  |
    |     | |     |      -----   -----
    |  0  | |  0  |        0       0
     -----   -----
     Coaxial Cable       Twisted Pair Cable
       (Default)


```

##### 设置超时参数


标有 EXT1 EXT2 的跳线用于确定超时参数。这两个跳线通常保持断开（开路）


## CNet Technology Inc.6位网卡）


### 160 系列6位网卡）


  - 来自 Juergen Seifert <seifert@htwm.de>

本说明由 Juergen Seifert <seifert@htwm.de> 根据以下 CNet 原版手册编写

	      "ARCNET USER'S MANUAL for
	      CN160A CN160AB CN160TP
	      P/N:12-01-0006 Revision 3.00"

ARCNET Datapoint Corporation 的注册商

- P/N 160A   ARCNET 16XT/AT 星型
- P/N 160AB  ARCNET 16XT/AT 总线
- P/N 160TP  ARCNET 16XT/AT 双绞


```

   ___________________________________________________________________
  <                             _________________________          ___|
  >               |oo| JP2     |                         |    LED |___|
  <               |oo| JP1     |        9026             |    LED |___|
  >                            |_________________________|         ___|
  <                                                             N |   | ID7
  >                                                      1      o |   | ID6
  <                                    1 2 3 4 5 6 7 8 9 0      d | S | ID5
  >         _______________           _____________________     e | W | ID4
  <        |     PROM      |         |         SW1         |    A | 2 | ID3
  >        >    SOCKET     |         |_____________________|    d |   | ID2
  <        |_______________|          | IO-Base   | MEM   |     d |   | ID1
  >                                                             r |___| ID0
  <                                                               ____|
  >                                                              |    |
  <                                                              | J1 |
  >                                                              |    |
  <                                                              |____|
  >                            1 1 1 1                                |
  <  3 4 5 6 7      JP     8 9 0 1 2 3                                |
  > |o|o|o|o|o|           |o|o|o|o|o|o|                               |
  < |o|o|o|o|o| __        |o|o|o|o|o|o|                    ___________|
  >            |  |                                       |
  <____________|  |_______________________________________|

```
```

  9026            ARCNET Probe
  SW1 1-6:    Base I/O Address Select
      7-10:   Base Memory Address Select
  SW2 1-8:    Node ID Select (ID0-ID7)
  JP1/JP2     ET1, ET2 Timeout Parameters
  JP3-JP13    Interrupt Select
  J1      BNC RG62/U Connector        (CN160A/AB only)
  J1      Two 6-position Telephone Jack   (CN160TP only)
  LED

```

将某个开关拨Off（关）表"1"，拨On（开）表"0"


##### 设置节点 ID


SW2 中的八个开关用于设置节ID。连接到网络的每个节点必须具有唯一的节ID，且必须不同0
1 号开关（ID0）作为最低有效位（LSB）

节点 ID 是所有拨"1" 的开关取值之


```

   Switch | Label | Value
   -------|-------|-------
     1    | ID0   |   1
     2    | ID1   |   2
     3    | ID2   |   4
     4    | ID3   |   8
     5    | ID4   |  16
     6    | ID5   |  32
     7    | ID6   |  64
     8    | ID7   | 128

```
```

    Switch         | Hex     | Decimal
   8 7 6 5 4 3 2 1 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 设置 I/O 基地址


开关块 SW1 中的前六个开关用于选择 I/O 基地址


```

	     Switch        | Hex I/O
    1   2   3   4   5   6  | Address
   ------------------------|--------
   OFF ON  ON  OFF OFF ON  |  260
   OFF ON  OFF ON  ON  OFF |  290
   OFF ON  OFF OFF OFF ON  |  2E0  (Manufacturer's default)
   OFF ON  OFF OFF OFF OFF |  2F0
   OFF OFF ON  ON  ON  ON  |  300
   OFF OFF ON  OFF ON  OFF |  350
   OFF OFF OFF ON  ON  ON  |  380
   OFF OFF OFF OFF OFF ON  |  3E0

```

注意：似乎还可以选择其他 I/O 基地址，但手册中仅记录了上述组合


##### 设置基址内存（RAM）缓冲区地址


开关块 SW1 7-10 号开关用于选择内存


```

   Switch          | Hex RAM | Hex ROM
    7   8   9  10  | Address | Address
   ----------------|---------|-----------
   OFF OFF ON  ON  |  C0000  |  C8000
   OFF OFF ON  OFF |  D0000  |  D8000 (Default)
   OFF OFF OFF ON  |  E0000  |  E8000

```


      Other MEM-Base addresses seem to be selectable, but only the above
      combinations are documented.


##### 设置中断


要选择一个硬件中断级别，请安装其中一个（且只能安装一个！）跳


```

   Jumper | IRQ
   -------|-----------------
     3    |  14
     4    |  15
     5    |  12
     6    |  11
     7    |  10
     8    |   3
     9    |   4
    10    |   5
    11    |   6
    12    |   7
    13    |   2 (=9) Default!

```


       - 不要使用 JP11=IRQ6，它可能与你的软盘控制器冲突
	 Controller
       - 仅当没有 IDE、MFM RLL 硬盘时才使用 JP3=IRQ14，否则它可能与这些硬盘控制器冲突


### 设置超时参数


标有 JP1 JP2 的跳线用于确定超时参数。这两个跳线通常保持断开（开路）


## Lantech


### 8位网卡，型号未知


  - 来自 Vlad Lungu <vlungu@ugal.ro> —我尝试联系他时，他的电子邮件地址似乎已失效。Vlad，如果你没有收到我的回复，抱歉


```

   ________________________________________________________________
   |   1         8                                                 |
   |   ___________                                               __|
   |   |   SW1    |                                         LED |__|
   |   |__________|                                                |
   |                                                            ___|
   |                _____________________                       |S | 8
   |                |                   |                       |W |
   |                |                   |                       |2 |
   |                |                   |                       |__| 1
   |                |      UM9065L      |     |o|  JP4         ____|____
   |                |                   |     |o|              |  CN    |
   |                |                   |                      |________|
   |                |                   |                          |
   |                |___________________|                          |
   |                                                               |
   |                                                               |
   |      _____________                                            |
   |      |            |                                           |
   |      |    PROM    |        |ooooo|  JP6                       |
   |      |____________|        |ooooo|                            |
   |_____________                                             _   _|
		|____________________________________________| |__|


```

UM9065L：ARCnet 控制

SW 1    ：共享内存地址I/O 基地址



```

	ON=0

	12345|Memory Address
	-----|--------------
	00001|  D4000
	00010|  CC000
	00110|  D0000
	01110|  D1000
	01101|  D9000
	10010|  CC800
	10011|  DC800
	11110|  D1800

```

位似乎是按相反顺序解释的。此外，你必须注意其中某些地址并不常见，我也未对它们进行探测；我是DOS 下通过内存转储来识别它们的。对00000 配置以及我未在此写出的其他一些配置，该网卡似乎会与显卡（一S3 GENDAC）冲突。这些地址的完整解码就留给你了


```

	678| I/O Address
	---|------------
	000|    260
	001|    failed probe
	010|    2E0
	011|    380
	100|    290
	101|    350
	110|    failed probe
	111|    3E0

  SW 2  : Node ID (binary coded)

  JP 4  : Boot PROM enable   CLOSE - enabled
			     OPEN  - disabled

  JP 6  : IRQ set (ONLY ONE jumper on 1-5 for IRQ 2-6)


```

## Acer


### 8位网卡，型号 5210-003



  - 来自 Vojtech Pavlik <vojtech@suse.cz>，使用了现有 arcnet-hardware 文件的部分内容

这是一块基90C26 的网卡。其配置似乎SMC PC100 类似，但有一些我不知其含义的额外跳线


```

	       __
	      |  |
   ___________|__|_________________________
  |         |      |                       |
  |         | BNC  |                       |
  |         |______|                    ___|
  |  _____________________             |___
  | |                     |                |
  | | Hybrid IC           |                |
  | |                     |       o|o J1   |
  | |_____________________|       8|8      |
  |                               8|8 J5   |
  |                               o|o      |
  |                               8|8      |
  |__                             8|8      |
 (|__| LED                        o|o      |
  |                               8|8      |
  |                               8|8 J15  |
  |                                        |
  |                    _____               |
  |                   |     |   _____      |
  |                   |     |  |     |  ___|
  |                   |     |  |     | |
  |  _____            | ROM |  | UFS | |
  | |     |           |     |  |     | |
  | |     |     ___   |     |  |     | |
  | |     |    |   |  |__.__|  |__.__| |
  | | NCR |    |XTL|   _____    _____  |
  | |     |    |___|  |     |  |     | |
  | |90C26|           |     |  |     | |
  | |     |           | RAM |  | UFS | |
  | |     | J17 o|o   |     |  |     | |
  | |     | J16 o|o   |     |  |     | |
  | |__.__|           |__.__|  |__.__| |
  |  ___                               |
  | |   |8                             |
  | |SW2|                              |
  | |   |                              |
  | |___|1                             |
  |  ___                               |
  | |   |10           J18 o|o          |
  | |   |                 o|o          |
  | |SW1|                 o|o          |
  | |   |             J21 o|o          |
  | |___|1                             |
  |                                    |
  |____________________________________|


```
```

  90C26       ARCNET Chip
  XTL         20 MHz Crystal
  SW1 1-6     Base I/O Address Select
      7-10    Memory Address Select
  SW2 1-8     Node ID Select (ID0-ID7)
  J1-J5       IRQ Select
  J6-J21      Unknown (Probably extra timeouts & ROM enable ...)
  LED1        Activity LED
  BNC         Coax connector (STAR ARCnet)
  RAM         2k of SRAM
  ROM         Boot ROM socket
  UFS         Unidentified Flying Sockets


```

##### 设置节点 ID


SW2 中的八个开关用于设置节ID。连接到网络的每个节点必须具有唯一的节ID，且不能0
1 号开关（ID0）作为最低有效位（LSB）

将某个开关拨OFF（关）表"1"，拨ON（开）表"0"

节点 ID 是所有拨"1" 的开关取值之


```

   Switch | Value
   -------|-------
     1    |   1
     2    |   2
     3    |   4
     4    |   8
     5    |  16
     6    |  32
     7    |  64
     8    | 128

```

不要将其设为 0 255；这两个值是保留值


##### 设置 I/O 基地址


开关块 SW1 1 6 号开关用于选择一


```

	  | Hex
   Switch | Value
   -------|-------
     1    | 200
     2    | 100
     3    |  80
     4    |  40
     5    |  20
     6    |  10

```

I/O 地址是所有拨"1" 的开关的取值之和。请注意x200 以下I/O 地址空间是为主板保留的，因此 1 号开关应始终拨到 OFF（关）


##### 设置基址内存（RAM）缓冲区地址


内存缓冲区（RAM）需2K。该缓冲区的基址可位于十六个位置中的任意一个。不过，A0000 以下的地址可能会因为存在主内存而导致系统挂起


```

   Switch          | Hex RAM
    7   8   9  10  | Address
   ----------------|---------
   OFF OFF OFF OFF |  F0000 (conflicts with main BIOS)
   OFF OFF OFF ON  |  E0000
   OFF OFF ON  OFF |  D0000
   OFF OFF ON  ON  |  C0000 (conflicts with video BIOS)
   OFF ON  OFF OFF |  B0000 (conflicts with mono video)
   OFF ON  OFF ON  |  A0000 (conflicts with graphics)


```

##### 设置中断


跳线J1 1-5 号跳线控IRQ 级别。ON（开）表


```

    Jumper              |  IRQ
    1   2   3   4   5   |
   ----------------------------
    ON  OFF OFF OFF OFF |  7
    OFF ON  OFF OFF OFF |  5
    OFF OFF ON  OFF OFF |  4
    OFF OFF OFF ON  OFF |  3
    OFF OFF OFF OFF ON  |  2


```

##### 未知跳线与插


我对这些一无所知。我猜测 J16 J17 是超时跳线，也许 J18-J21 中有一个用于选择 ROM。此外，J6-J10 J11-J15 IRQ2-7 连接UFS 上的某些引脚。我猜不出其用途

## Datapoint？（厂商未知


### LAN-ARC-8，一8位网


  - 来自 Vojtech Pavlik <vojtech@suse.cz>

这是另一块基SMC 90C65 ARCnet 网卡。我无法确定其制造商，但它可能是 DataPoint，因为该网卡右上角带有原始的 arcNet 标志


```

	  _______________________________________________________
	 |                         _________                     |
	 |                        |   SW2   | ON      arcNet     |
	 |                        |_________| OFF             ___|
	 |  _____________         1 ______  8                |   | 8
	 | |             | SW1     | XTAL | ____________     | S |
	 | > RAM (2k)    |         |______||            |    | W |
	 | |_____________|                 |      H     |    | 3 |
	 |                        _________|_____ y     |    |___| 1
	 |  _________            |         |     |b     |        |
	 | |_________|           |         |     |r     |        |
	 |                       |     SMC |     |i     |        |
	 |                       |    90C65|     |d     |        |
	 |  _________            |         |     |      |        |
	 | |   SW1   | ON        |         |     |I     |        |
	 | |_________| OFF       |_________|_____/C     |   _____|
	 |  1       8                      |            |  |     |___
	 |  ______________                 |            |  | BNC |___|
	 | |              |                |____________|  |_____|
	 | > EPROM SOCKET |              _____________           |
	 | |______________|             |_____________|          |
	 |                                         ______________|
	 |                                        |
	 |________________________________________|

```
```

  90C65       ARCNET Chip
  SW1 1-5:    Base Memory Address Select
      6-8:    Base I/O Address Select
  SW2 1-8:    Node ID Select
  SW3 1-5:    IRQ Select
      6-7:    Extra Timeout
      8  :    ROM Enable
  BNC         Coax connector
  XTAL        20 MHz Crystal


```

##### 设置节点 ID


SW3 中的八个开关用于设置节ID。连接到网络的每个节点必须具有唯一的节ID，且不能0
1 号开关作为最低有效位（LSB）

将某个开关拨Off（关）表"1"，拨On（开）表"0"

节点 ID 是所有拨"1" 的开关取值之


```

   Switch | Value
   -------|-------
     1    |   1
     2    |   2
     3    |   4
     4    |   8
     5    |  16
     6    |  32
     7    |  64
     8    | 128


```

##### 设置 I/O 基地址


开关块 SW1 中的最后三个开关用于选择一


```


   Switch      | Hex I/O
    6   7   8  | Address
   ------------|--------
   ON  ON  ON  |  260
   OFF ON  ON  |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   OFF OFF ON  |  2F0
   ON  ON  OFF |  300
   OFF ON  OFF |  350
   ON  OFF OFF |  380
   OFF OFF OFF |  3E0


```

##### 设置基址内存（RAM）缓冲区地址


内存缓冲区（RAM）需2K。该缓冲区的基址可位于八个位置中的任意一个。Boot Prom 的地址为内存基址 + 0x2000

开关块 SW1 3-5 号跳线用于选择内存基址


```

   Switch              | Hex RAM | Hex ROM
    1   2   3   4   5  | Address | Address *)
   --------------------|---------|-----------
   ON  ON  ON  ON  ON  |  C0000  |  C2000
   ON  ON  OFF ON  ON  |  C4000  |  C6000
   ON  ON  ON  OFF ON  |  CC000  |  CE000
   ON  ON  OFF OFF ON  |  D0000  |  D2000  (Manufacturer's default)
   ON  ON  ON  ON  OFF |  D4000  |  D6000
   ON  ON  OFF ON  OFF |  D8000  |  DA000
   ON  ON  ON  OFF OFF |  DC000  |  DE000
   ON  ON  OFF OFF OFF |  E0000  |  E2000

  *) To enable the Boot ROM set the switch 8 of switch block SW3 to position ON.

```

1 号和 2 号开关可能会RAM 基址增加 0x0800 0x1000


##### 设置中断


要选择一个硬件中断级别，请安装其中一个（且只能安装一个！）跳


```

    Jumper              |  IRQ
    1   2   3   4   5   |
   ----------------------------
    ON  OFF OFF OFF OFF |  3
    OFF ON  OFF OFF OFF |  4
    OFF OFF ON  OFF OFF |  5
    OFF OFF OFF ON  OFF |  7
    OFF OFF OFF OFF ON  |  2


```

##### 设置超时参数


开关块 SW3 6-7 号开关用于确定超时参数。这两个开关通常保持OFF（关）位置


## Topware


### 8位网卡，TA-ARC/10


  - 来自 Vojtech Pavlik <vojtech@suse.cz>

这是另一块非常相似的 90C65 网卡。其大部分开关和跳线与其他兼容卡相同


```

   _____________________________________________________________________
  |  ___________   |                         |            ______        |
  | |SW2 NODE ID|  |                         |           | XTAL |       |
  | |___________|  |  Hybrid IC              |           |______|       |
  |  ___________   |                         |                        __|
  | |SW1 MEM+I/O|  |_________________________|                   LED1|__|)
  | |___________|           1 2                                         |
  |                     J3 |o|o| TIMEOUT                          ______|
  |     ______________     |o|o|                                 |      |
  |    |              |  ___________________                     | RJ   |
  |    > EPROM SOCKET | |                   \                    |------|
  |J2  |______________| |                    |                   |      |
  ||o|                  |                    |                   |______|
  ||o| ROM ENABLE       |        SMC         |    _________             |
  |     _____________   |       90C65        |   |_________|       _____|
  |    |             |  |                    |                    |     |___
  |    > RAM (2k)    |  |                    |                    | BNC |___|
  |    |_____________|  |                    |                    |_____|
  |                     |____________________|                          |
  | ________ IRQ 2 3 4 5 7                  ___________                 |
  ||________|   |o|o|o|o|o|                |___________|                |
  |________   J1|o|o|o|o|o|                               ______________|
	   |                                             |
	   |_____________________________________________|

```
```

  90C65       ARCNET Chip
  XTAL        20 MHz Crystal
  SW1 1-5     Base Memory Address Select
      6-8     Base I/O Address Select
  SW2 1-8     Node ID Select (ID0-ID7)
  J1          IRQ Select
  J2          ROM Enable
  J3          Extra Timeout
  LED1        Activity LED
  BNC         Coax connector (BUS ARCnet)
  RJ          Twisted Pair Connector (daisy chain)


```

##### 设置节点 ID


SW2 中的八个开关用于设置节ID。连接到网络的每个节点必须具有唯一的节ID，且不能0 号开关（ID0）作为最低有效位（LSB）

将某个开关拨Off（关）表"1"，拨On（开）表"0"

节点 ID 是所有拨"1" 的开关取值之


```

   Switch | Label | Value
   -------|-------|-------
     1    | ID0   |   1
     2    | ID1   |   2
     3    | ID2   |   4
     4    | ID3   |   8
     5    | ID4   |  16
     6    | ID5   |  32
     7    | ID6   |  64
     8    | ID7   | 128

```

##### 设置 I/O 基地址


开关块 SW1 中的最后三个开关用于选择一


```


   Switch      | Hex I/O
    6   7   8  | Address
   ------------|--------
   ON  ON  ON  |  260  (Manufacturer's default)
   OFF ON  ON  |  290
   ON  OFF ON  |  2E0
   OFF OFF ON  |  2F0
   ON  ON  OFF |  300
   OFF ON  OFF |  350
   ON  OFF OFF |  380
   OFF OFF OFF |  3E0


```

##### 设置基址内存（RAM）缓冲区地址


内存缓冲区（RAM）需2K。该缓冲区的基址可位于八个位置中的任意一个。Boot Prom 的地址为内存基址 + 0x2000

开关块 SW1 3-5 号跳线用于选择内存基址


```

   Switch              | Hex RAM | Hex ROM
    1   2   3   4   5  | Address | Address *)
   --------------------|---------|-----------
   ON  ON  ON  ON  ON  |  C0000  |  C2000
   ON  ON  OFF ON  ON  |  C4000  |  C6000  (Manufacturer's default)
   ON  ON  ON  OFF ON  |  CC000  |  CE000
   ON  ON  OFF OFF ON  |  D0000  |  D2000
   ON  ON  ON  ON  OFF |  D4000  |  D6000
   ON  ON  OFF ON  OFF |  D8000  |  DA000
   ON  ON  ON  OFF OFF |  DC000  |  DE000
   ON  ON  OFF OFF OFF |  E0000  |  E2000

   *) To enable the Boot ROM short the jumper J2.

```

1 号和 2 号跳线可能会RAM 地址增加 0x0800 0x1000


##### 设置中断


跳线J1 1-5 号跳线控IRQ 级别。ON（开）表


```

    Jumper              |  IRQ
    1   2   3   4   5   |
   ----------------------------
    ON  OFF OFF OFF OFF |  2
    OFF ON  OFF OFF OFF |  3
    OFF OFF ON  OFF OFF |  4
    OFF OFF OFF ON  OFF |  5
    OFF OFF OFF OFF ON  |  7


```

##### 设置超时参数


跳线 J3 用于设置超时参数。这两个跳线通常保持断开（开路）

## Thomas-Conrad


### 型号 #500-6242-0097 REV A位网卡）


  - 来自 Lars Karlsson <100617.3473@compuserve.com>


```

     ________________________________________________________
   |          ________   ________                           |_____
   |         |........| |........|                            |
   |         |________| |________|                         ___|
   |            SW 3       SW 1                           |   |
   |         Base I/O   Base Addr.                Station |   |
   |                                              address |   |
   |    ______                                    switch  |   |
   |   |      |                                           |   |
   |   |      |                                           |___|
   |   |      |                                 ______        |___._
   |   |______|                                |______|         ____| BNC
   |                                            Jumper-        _____| Connector
   |   Main chip                                block  _    __|   '
   |                                                  | |  |    RJ Connector
   |                                                  |_|  |    with 110 Ohm
   |                                                       |__  Terminator
   |    ___________                                         __|
   |   |...........|                                       |    RJ-jack
   |   |...........|    _____                              |    (unused)
   |   |___________|   |_____|                             |__
   |  Boot PROM socket IRQ-jumpers                            |_  Diagnostic
   |________                                       __          _| LED (red)
	    | | | | | | | | | | | | | | | | | | | |  |        |
	    | | | | | | | | | | | | | | | | | | | |  |________|
							      |
							      |

```

以下是该网卡上部分开关和跳线的设置


```

	    I/O

	   1 2 3 4 5 6 7 8

  2E0----- 0 0 0 1 0 0 0 1
  2F0----- 0 0 0 1 0 0 0 0
  300----- 0 0 0 0 1 1 1 1
  350----- 0 0 0 0 1 1 1 0

```

上述示例中的 "0" 表示开关为关（off），"1" 表示开关为开（on）


```

      ShMem address.

	1 2 3 4 5 6 7 8

  CX00--0 0 1 1 | |   |
  DX00--0 0 1 0       |
  X000--------- 1 1   |
  X400--------- 1 0   |
  X800--------- 0 1   |
  XC00--------- 0 0
  ENHANCED----------- 1
  COMPATIBLE--------- 0

```
```

	 IRQ


     3 4 5 7 2
     . . . . .
     . . . . .


```

有一个带 8 个开关的 DIP 开关，用于设置要使用的共享内存地址。前 6 个开关设置地址，第 7 个没有任何功能，8 个开关用于选择 "compatible"（兼容）"enhanced"（增强）。我拿到两块网卡时，其中一块的这个开关设在了 "enhanced"（增强）。那块网卡根本无法工作，驱动程序甚至无法识别它。另一块网卡该开关设在了 "compatible"（兼容），表现完全正常。我猜测其中一块网卡的开关在从原主机取出时一定被意外改动过enhanced"（增强）位置的用途到底是什么，这个问题仍未得到解答

[Avery 的注释："enhanced"（增强）可能禁用共享内存（改I/O 端口），也可能禁I/O 端口（改用内存地址）。具体因网卡类型而异。我实在看不出这两种方式有何"增强"之处。如果对此模式有更详细的信息，请发给我，否则直接使用 "compatible"（兼容）模式即可。]

## Waterloo Microsystems Inc.？（厂商未知


### 8位网卡（C985


  - 来自 Robert Michael Best <rmb117@cs.usask.ca>

[Avery 的注释：出于某种原因，这些网卡无法与我的驱动程序配合工作。这些网卡的设置似乎PDI508Plus 类似，而后者是软件配置的，也无法与我的驱动程序配合工作Waterloo 芯片"是一块启PROM，很可能是专门为滑铁卢大学设计的。如果你有关于此网卡的更多信息，请发电子邮件给我。]

探测程序无法在任J2 设置下检测到该网卡，即便我取"Waterloo" 芯片后再次尝试也是如此


```

   _____________________________________________________________________
  | \/  \/              ___  __ __                                      |
  | C4  C4     |^|     | M ||  ^  ||^|                                  |
  | --  --     |_|     | 5 ||     || | C3                               |
  | \/  \/      C10    |___||     ||_|                                  |
  | C4  C4             _  _ |     |                 ??                  |
  | --  --            | \/ ||     |                                     |
  |                   |    ||     |                                     |
  |                   |    ||  C1 |                                     |
  |                   |    ||     |  \/                            _____|
  |                   | C6 ||     |  C9                           |     |___
  |                   |    ||     |  --                           | BNC |___|
  |                   |    ||     |          >C7|                 |_____|
  |                   |    ||     |                                     |
  | __ __             |____||_____|       1 2 3     6                   |
  ||  ^  |     >C4|                      |o|o|o|o|o|o| J2    >C4|       |
  ||     |                               |o|o|o|o|o|o|                  |
  || C2  |     >C4|                                          >C4|       |
  ||     |                                   >C8|                       |
  ||     |       2 3 4 5 6 7  IRQ                            >C4|       |
  ||_____|      |o|o|o|o|o|o| J3                                        |
  |_______      |o|o|o|o|o|o|                            _______________|
	  |                                             |
	  |_____________________________________________|

  C1 -- "COM9026
	 SMC 8638"
	In a chip socket.

  C2 -- "@Copyright
	 Waterloo Microsystems Inc.
	 1985"
	In a chip Socket with info printed on a label covering a round window
	showing the circuit inside. (The window indicates it is an EPROM chip.)

  C3 -- "COM9032
	 SMC 8643"
	In a chip socket.

  C4 -- "74LS"
	9 total no sockets.

  M5 -- "50006-136
	 20.000000 MHZ
	 MTQ-T1-S3
	 0 M-TRON 86-40"
	Metallic case with 4 pins, no socket.

  C6 -- "MOSTEK@TC8643
	 MK6116N-20
	 MALAYSIA"
	No socket.

  C7 -- No stamp or label but in a 20 pin chip socket.

  C8 -- "PAL10L8CN
	 8623"
	In a 20 pin socket.

  C9 -- "PAl16R4A-2CN
	 8641"
	In a 20 pin socket.

  C10 -- "M8640
	    NMC
	  9306N"
	 In an 8 pin socket.

  ?? -- Some components on a smaller board and attached with 20 pins all
	along the side closest to the BNC connector.  The are coated in a dark
	resin.

```

电路板上有两组标J2 J3 的跳线排。制造商没有在板上放J1。我手上的两块电路板都各带有一个对应跳线排的跳线盒


```

  J2 -- Numbered 1 2 3 4 5 6.
	4 and 5 are not stamped due to solder points.

  J3 -- IRQ 2 3 4 5 6 7

```

电路板本身在 IRQ 跳线上方压印有一片枫叶，C2 旁边印有 "-2 46-86"。在 C1 C6 之间压印"ASS 'Y 300163"，BNC 连接器正下方压印"@1986 CORMAN CUSTOM ELECTRONICS CORP."。其下方"MADE IN CANADA"（加拿大制造）

## 无名网卡（No Name


### 8位网卡6位网


  - 来自 Juergen Seifert <seifert@htwm.de>

我将这块 ARCnet 网卡命名"NONAME"（无名），因为在安装手册和包装盒上都找不到任何制造商名称。唯一暗示存在制造商的痕迹是以铜箔印出的 "Made in Taiwan"（台湾制造）

本说明由 Juergen Seifert <seifert@htwm.de> 根据原版编写

		    "ARCnet Installation Manual"


```

    ________________________________________________________________
   | |STAR| BUS| T/P|                                               |
   | |____|____|____|                                               |
   |                            _____________________               |
   |                           |                     |              |
   |                           |                     |              |
   |                           |                     |              |
   |                           |        SMC          |              |
   |                           |                     |              |
   |                           |       COM90C65      |              |
   |                           |                     |              |
   |                           |                     |              |
   |                           |__________-__________|              |
   |                                                           _____|
   |      _______________                                     |  CN |
   |     | PROM          |                                    |_____|
   |     > SOCKET        |                                          |
   |     |_______________|         1 2 3 4 5 6 7 8  1 2 3 4 5 6 7 8 |
   |                               _______________  _______________ |
   |           |o|o|o|o|o|o|o|o|  |      SW1      ||      SW2      ||
   |           |o|o|o|o|o|o|o|o|  |_______________||_______________||
   |___         2 3 4 5 7 E E R        Node ID       IOB__|__MEM____|
       |        \ IRQ   / T T O                      |
       |__________________1_2_M______________________|

```
```

  COM90C65:       ARCnet Probe
  S1  1-8:    Node ID Select
  S2  1-3:    I/O Base Address Select
      4-6:    Memory Base Address Select
      7-8:    RAM Offset Select
  ET1, ET2    Extended Timeout Select
  ROM     ROM Enable Select
  CN              RG62 Coax Connector
  STAR| BUS | T/P Three fields for placing a sign (colored circle)
		  indicating the topology of the card

```

将某个开关拨Off（关）表"1"，拨On（开）表"0"


##### 设置节点 ID


SW1 中的八个开关用于设置节ID
连接到网络的每个节点必须具有唯一的节ID，且必须不同0
8 号开关作为最低有效位（LSB）

节点 ID 是所有拨"1" 的开关取值之


```

    Switch | Value
    -------|-------
      8    |   1
      7    |   2
      6    |   4
      5    |   8
      4    |  16
      3    |  32
      2    |  64
      1    | 128

```
```

    Switch         | Hex     | Decimal
   1 2 3 4 5 6 7 8 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 设置 I/O 基地址


开关组 SW2 中的前三个开关用于选择一


```

   Switch      | Hex I/O
    1   2   3  | Address
   ------------|--------
   ON  ON  ON  |  260
   ON  ON  OFF |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   ON  OFF OFF |  2F0
   OFF ON  ON  |  300
   OFF ON  OFF |  350
   OFF OFF ON  |  380
   OFF OFF OFF |  3E0


```


##### 设置基址内存（RAM）缓冲区地址


内存缓冲区需16K RAM 块中2K。该 16K 块的基址可位于八个位置中的任意一个。开关组 SW2 4-6 号开关选择 16K 块的基址。在16K 地址空间内，缓冲区可被分配到四个位置中的任意一个，具体由偏移量（即SW2 7 号和 8 号开关）决定


```

   Switch     | Hex RAM | Hex ROM
   4 5 6  7 8 | Address | Address *)
   -----------|---------|-----------
   0 0 0  0 0 |  C0000  |  C2000
   0 0 0  0 1 |  C0800  |  C2000
   0 0 0  1 0 |  C1000  |  C2000
   0 0 0  1 1 |  C1800  |  C2000
	      |         |
   0 0 1  0 0 |  C4000  |  C6000
   0 0 1  0 1 |  C4800  |  C6000
   0 0 1  1 0 |  C5000  |  C6000
   0 0 1  1 1 |  C5800  |  C6000
	      |         |
   0 1 0  0 0 |  CC000  |  CE000
   0 1 0  0 1 |  CC800  |  CE000
   0 1 0  1 0 |  CD000  |  CE000
   0 1 0  1 1 |  CD800  |  CE000
	      |         |
   0 1 1  0 0 |  D0000  |  D2000  (Manufacturer's default)
   0 1 1  0 1 |  D0800  |  D2000
   0 1 1  1 0 |  D1000  |  D2000
   0 1 1  1 1 |  D1800  |  D2000
	      |         |
   1 0 0  0 0 |  D4000  |  D6000
   1 0 0  0 1 |  D4800  |  D6000
   1 0 0  1 0 |  D5000  |  D6000
   1 0 0  1 1 |  D5800  |  D6000
	      |         |
   1 0 1  0 0 |  D8000  |  DA000
   1 0 1  0 1 |  D8800  |  DA000
   1 0 1  1 0 |  D9000  |  DA000
   1 0 1  1 1 |  D9800  |  DA000
	      |         |
   1 1 0  0 0 |  DC000  |  DE000
   1 1 0  0 1 |  DC800  |  DE000
   1 1 0  1 0 |  DD000  |  DE000
   1 1 0  1 1 |  DD800  |  DE000
	      |         |
   1 1 1  0 0 |  E0000  |  E2000
   1 1 1  0 1 |  E0800  |  E2000
   1 1 1  1 0 |  E1000  |  E2000
   1 1 1  1 1 |  E1800  |  E2000

   *) To enable the 8K Boot PROM install the jumper ROM.
      The default is jumper ROM not installed.


```

##### 设置中断请求线（IRQ


要选择一个硬件中断级别，请设置跳IRQ2、IRQ3、IRQ4、IRQ5 IRQ7 中的一个（且只能设置一个！）。厂商默认值为 IRQ2


##### 设置超时


标有 ET1 ET2 的两个跳线用于确定超时参数（响应时间与重配置时间）。网络中的每个节点都必须设置为相同的超时值


```

   ET1 ET2 | Response Time (us) | Reconfiguration Time (ms)
   --------|--------------------|--------------------------
   Off Off |        78          |          840   (Default)
   Off On  |       285          |         1680
   On  Off |       563          |         1680
   On  On  |      1130          |         1680

```

On（开）表示已安装跳线，Off（关）表示未安装跳线


### 16浣?ARCNET


我的 8NONAME ARCnet 网卡手册中还包含了对一16同轴电缆/双绞网卡的描述。该描述不完整，因为手册小册子中缺了两页。（目录中列出了页码…-9-11-12-1……，但小册子内部的页码编排方式不同…-9-10、A-1、（空白页）-1……-18、A-1（再次出现）、A-2）。此外，电路板布局图的质量不如 8网卡那张，因为图上没有标注类"SW1" 的字样

如果有人拥有这样一块电路板，请随时补充此描述或给我发邮件！

本说明由 Juergen Seifert <seifert@htwm.de> 根据原版编写

		    "ARCnet Installation Manual"


```

   ___________________________________________________________________
  <                    _________________  _________________           |
  >                   |       SW?       ||      SW?        |          |
  <                   |_________________||_________________|          |
  >                       ____________________                        |
  <                      |                    |                       |
  >                      |                    |                       |
  <                      |                    |                       |
  >                      |                    |                       |
  <                      |                    |                       |
  >                      |                    |                       |
  <                      |                    |                       |
  >                      |____________________|                       |
  <                                                               ____|
  >                       ____________________                   |    |
  <                      |                    |                  | J1 |
  >                      |                    <                  |    |
  <                      |____________________|  ? ? ? ? ? ?     |____|
  >                                             |o|o|o|o|o|o|         |
  <                                             |o|o|o|o|o|o|         |
  >                                                                   |
  <             __                                         ___________|
  >            |  |                                       |
  <____________|  |_______________________________________|


```

将某个开关拨Off（关）表"1"，拨On（开）表"0"


##### 设置节点 ID


SW2 中的八个开关用于设置节ID。连接到网络的每个节点必须具有唯一的节ID，且必须不同0
8 号开关作为最低有效位（LSB）

节点 ID 是所有拨"1" 的开关取值之

```

    Switch | Value
    -------|-------
      8    |   1
      7    |   2
      6    |   4
      5    |   8
      4    |  16
      3    |  32
      2    |  64
      1    | 128

```
```

    Switch         | Hex     | Decimal
   1 2 3 4 5 6 7 8 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 设置 I/O 基地址


开关组 SW1 中的前三个开关用于选择一

```

   Switch      | Hex I/O
    3   2   1  | Address
   ------------|--------
   ON  ON  ON  |  260
   ON  ON  OFF |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   ON  OFF OFF |  2F0
   OFF ON  ON  |  300
   OFF ON  OFF |  350
   OFF OFF ON  |  380
   OFF OFF OFF |  3E0


```

##### 设置基址内存（RAM）缓冲区地址


内存缓冲区需16K RAM 块中2K。该 16K 块的基址可位于八个位置中的任意一个
开关组 SW1 6-8 号开关选择 16K 块的基址
在该 16K 地址空间内，缓冲区可被分配到四个位置中的任意一

```

   Switch     | Hex RAM | Hex ROM
   8 7 6  5 4 | Address | Address
   -----------|---------|-----------
   0 0 0  0 0 |  C0000  |  C2000
   0 0 0  0 1 |  C0800  |  C2000
   0 0 0  1 0 |  C1000  |  C2000
   0 0 0  1 1 |  C1800  |  C2000
	      |         |
   0 0 1  0 0 |  C4000  |  C6000
   0 0 1  0 1 |  C4800  |  C6000
   0 0 1  1 0 |  C5000  |  C6000
   0 0 1  1 1 |  C5800  |  C6000
	      |         |
   0 1 0  0 0 |  CC000  |  CE000
   0 1 0  0 1 |  CC800  |  CE000
   0 1 0  1 0 |  CD000  |  CE000
   0 1 0  1 1 |  CD800  |  CE000
	      |         |
   0 1 1  0 0 |  D0000  |  D2000  (Manufacturer's default)
   0 1 1  0 1 |  D0800  |  D2000
   0 1 1  1 0 |  D1000  |  D2000
   0 1 1  1 1 |  D1800  |  D2000
	      |         |
   1 0 0  0 0 |  D4000  |  D6000
   1 0 0  0 1 |  D4800  |  D6000
   1 0 0  1 0 |  D5000  |  D6000
   1 0 0  1 1 |  D5800  |  D6000
	      |         |
   1 0 1  0 0 |  D8000  |  DA000
   1 0 1  0 1 |  D8800  |  DA000
   1 0 1  1 0 |  D9000  |  DA000
   1 0 1  1 1 |  D9800  |  DA000
	      |         |
   1 1 0  0 0 |  DC000  |  DE000
   1 1 0  0 1 |  DC800  |  DE000
   1 1 0  1 0 |  DD000  |  DE000
   1 1 0  1 1 |  DD800  |  DE000
	      |         |
   1 1 1  0 0 |  E0000  |  E2000
   1 1 1  0 1 |  E0800  |  E2000
   1 1 1  1 0 |  E1000  |  E2000
   1 1 1  1 1 |  E1800  |  E2000


```

##### 设置中断请求线（IRQ


??????????????????????????????????????


##### 设置超时


??????????????????????????????????????


### 8位网卡（"Made in Taiwan R.O.C."


  - 来自 Vojtech Pavlik <vojtech@suse.cz>

我将这块 ARCnet 网卡命名"NONAME"（无名），因为我只拿到了这张卡，没有任何手册，而唯一能标识制造商的文字是印在卡上"MADE IN TAIWAN R.O.C"


```

	  ____________________________________________________________
	 |                 1 2 3 4 5 6 7 8                            |
	 | |o|o| JP1       o|o|o|o|o|o|o|o| ON                        |
	 |  +              o|o|o|o|o|o|o|o|                        ___|
	 |  _____________  o|o|o|o|o|o|o|o| OFF         _____     |   | ID7
	 | |             | SW1                         |     |    |   | ID6
	 | > RAM (2k)    |        ____________________ |  H  |    | S | ID5
	 | |_____________|       |                    ||  y  |    | W | ID4
	 |                       |                    ||  b  |    | 2 | ID3
	 |                       |                    ||  r  |    |   | ID2
	 |                       |                    ||  i  |    |   | ID1
	 |                       |       90C65        ||  d  |    |___| ID0
	 |      SW3              |                    ||     |        |
	 | |o|o|o|o|o|o|o|o| ON  |                    ||  I  |        |
	 | |o|o|o|o|o|o|o|o|     |                    ||  C  |        |
	 | |o|o|o|o|o|o|o|o| OFF |____________________||     |   _____|
	 |  1 2 3 4 5 6 7 8                            |     |  |     |___
	 |  ______________                             |     |  | BNC |___|
	 | |              |                            |_____|  |_____|
	 | > EPROM SOCKET |                                           |
	 | |______________|                                           |
	 |                                              ______________|
	 |                                             |
	 |_____________________________________________|

```
```

  90C65       ARCNET Chip
  SW1 1-5:    Base Memory Address Select
      6-8:    Base I/O Address Select
  SW2 1-8:    Node ID Select (ID0-ID7)
  SW3 1-5:    IRQ Select
      6-7:    Extra Timeout
      8  :    ROM Enable
  JP1         Led connector
  BNC         Coax connector

```

尽管 SW1 SW3 标注SW 而非 JP，但它们是跳线，不是开关

将跳线设ON（开）表示连接上方的两个引脚，设off 表示连接下方两个引脚——或者——在 IRQ 设置的情况下，表示完全不连接任何引脚

##### 设置节点 ID


SW2 中的八个开关用于设置节ID。连接到网络的每个节点必须具有唯一的节ID，且不能0
1 号开关（ID0）作为最低有效位（LSB）

将某个开关拨Off（关）表"1"，拨On（开）表"0"

节点 ID 是所有拨"1" 的开关取值之

```

   Switch | Label | Value
   -------|-------|-------
     1    | ID0   |   1
     2    | ID1   |   2
     3    | ID2   |   4
     4    | ID3   |   8
     5    | ID4   |  16
     6    | ID5   |  32
     7    | ID6   |  64
     8    | ID7   | 128

```
```

    Switch         | Hex     | Decimal
   8 7 6 5 4 3 2 1 | Node ID | Node ID
   ----------------|---------|---------
   0 0 0 0 0 0 0 0 |    not allowed
   0 0 0 0 0 0 0 1 |    1    |    1
   0 0 0 0 0 0 1 0 |    2    |    2
   0 0 0 0 0 0 1 1 |    3    |    3
       . . .       |         |
   0 1 0 1 0 1 0 1 |   55    |   85
       . . .       |         |
   1 0 1 0 1 0 1 0 |   AA    |  170
       . . .       |         |
   1 1 1 1 1 1 0 1 |   FD    |  253
   1 1 1 1 1 1 1 0 |   FE    |  254
   1 1 1 1 1 1 1 1 |   FF    |  255


```

##### 设置 I/O 基地址


开关块 SW1 中的最后三个开关用于选择一

```


   Switch      | Hex I/O
    6   7   8  | Address
   ------------|--------
   ON  ON  ON  |  260
   OFF ON  ON  |  290
   ON  OFF ON  |  2E0  (Manufacturer's default)
   OFF OFF ON  |  2F0
   ON  ON  OFF |  300
   OFF ON  OFF |  350
   ON  OFF OFF |  380
   OFF OFF OFF |  3E0


```

##### 设置基址内存（RAM）缓冲区地址


内存缓冲区（RAM）需2K。该缓冲区的基址可位于八个位置中的任意一个。Boot Prom 的地址为内存基址 + 0x2000

跳线SW1 3-5 号跳线用于选择内存基址


```

   Switch              | Hex RAM | Hex ROM
    1   2   3   4   5  | Address | Address *)
   --------------------|---------|-----------
   ON  ON  ON  ON  ON  |  C0000  |  C2000
   ON  ON  OFF ON  ON  |  C4000  |  C6000
   ON  ON  ON  OFF ON  |  CC000  |  CE000
   ON  ON  OFF OFF ON  |  D0000  |  D2000  (Manufacturer's default)
   ON  ON  ON  ON  OFF |  D4000  |  D6000
   ON  ON  OFF ON  OFF |  D8000  |  DA000
   ON  ON  ON  OFF OFF |  DC000  |  DE000
   ON  ON  OFF OFF OFF |  E0000  |  E2000

  *) To enable the Boot ROM set the jumper 8 of jumper block SW3 to position ON.

```

1 号和 2 号跳线可能会RAM 地址增加 0x0800x1000 0x1800


##### 设置中断



```

    Jumper              |  IRQ
    1   2   3   4   5   |
   ----------------------------
    ON  OFF OFF OFF OFF |  2
    OFF ON  OFF OFF OFF |  3
    OFF OFF ON  OFF OFF |  4
    OFF OFF OFF ON  OFF |  5
    OFF OFF OFF OFF ON  |  7


```

##### 设置超时参数


跳线SW3 6-7 号跳线用于确定超时参数。这两个跳线通常保持OFF（关）位置



### （通用型号 9058


  - 来自 Andrew J. Kroll <ag784@freenet.buffalo.edu>
  - 抱歉这份资料在我的待办箱里搁置了这么久，Andrew！（哎呀——超过一年了！）


```

								      _____
								     |    <
								     | .---'
    ________________________________________________________________ | |
   |                           |     SW2     |                      |  |
   |   ___________             |_____________|                      |  |
   |  |           |              1 2 3 4 5 6                     ___|  |
   |  >  6116 RAM |         _________                         8 |   |  |
   |  |___________|        |20MHzXtal|                        7 |   |  |
   |                       |_________|       __________       6 | S |  |
   |    74LS373                             |          |-     5 | W |  |
   |   _________                            |      E   |-     4 |   |  |
   |   >_______|              ______________|..... P   |-     3 | 3 |  |
   |                         |              |    : O   |-     2 |   |  |
   |                         |              |    : X   |-     1 |___|  |
   |   ________________      |              |    : Y   |-           |  |
   |  |      SW1       |     |      SL90C65 |    :     |-           |  |
   |  |________________|     |              |    : B   |-           |  |
   |    1 2 3 4 5 6 7 8      |              |    : O   |-           |  |
   |                         |_________o____|..../ A   |-    _______|  |
   |    ____________________                |      R   |-   |       |------,
   |   |                    |               |      D   |-   |  BNC  |   #  |
   |   > 2764 PROM SOCKET   |               |__________|-   |_______|------'
   |   |____________________|              _________                |  |
   |                                       >________| <- 74LS245    |  |
   |                                                                |  |
   |___                                               ______________|  |
       |H H H H H H H H H H H H H H H H H H H H H H H|               | |
       |U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U_U|               | |
								      \|

```
```

  SL90C65 	ARCNET Controller / Transceiver /Logic
  SW1	1-5:	IRQ Select
	  6:	ET1
	  7:	ET2
	  8:	ROM ENABLE
  SW2	1-3:    Memory Buffer/PROM Address
	3-6:	I/O Address Map
  SW3	1-8:	Node ID Select
  BNC		BNC RG62/U Connection
		*I* have had success using RG59B/U with *NO* terminators!
		What gives?!

```

##### SW1：超时、中断与 ROM


要选择一个硬件中断级别，请将 SW1 上（位于 1-5 号开关处）的 DIP 开关中的一个拨up（on）：IRQ3、IRQ4、IRQ5、IRQ7、IRQ2。厂商默认值为 IRQ2

SW1 上标EXT1 号开关）EXT2 号开关）的开关用于确定超时参数。这两个 DIP 开关通常保持off（down）位置

   要启8K Boot PROM，请SW1 上标ROM 8 号开关拨on（UP）  默认情况ROM 跳线未安装


##### 设置 I/O 基地址


开关组 SW2 中的最后三个开关用于选择一

```


   Switch | Hex I/O
   4 5 6  | Address
   -------|--------
   0 0 0  |  260
   0 0 1  |  290
   0 1 0  |  2E0  (Manufacturer's default)
   0 1 1  |  2F0
   1 0 0  |  300
   1 0 1  |  350
   1 1 0  |  380
   1 1 1  |  3E0


```

##### 设置基址内存地址（RAM ROM


内存缓冲区需16K RAM 块中2K。该 16K 块的基址可位于八个位置中的任意一个
开关组 SW2 1-3 号开关选择 16K 块的基址
锛? = DOWN锛? = UP锛。
不过，我只能验证其中两种设置…



```

   Switch| Hex RAM | Hex ROM
   1 2 3 | Address | Address
   ------|---------|-----------
   0 0 0 |  E0000  |  E2000
   0 0 1 |  D0000  |  D2000  (Manufacturer's default)
   0 1 0 |  ?????  |  ?????
   0 1 1 |  ?????  |  ?????
   1 0 0 |  ?????  |  ?????
   1 0 1 |  ?????  |  ?????
   1 1 0 |  ?????  |  ?????
   1 1 1 |  ?????  |  ?????


```

##### 设置节点 ID


SW3 中的八个开关用于设置节ID。连接到网络的每个节点必须具有唯一的节ID，且必须不同0
1 号开关作为最低有效位（LSB）
处于 DOWN 位置的开关为 OFF），处于 UP 位置的开关为 ON）

节点 ID 是所有拨"1" 的开关取值之

```

    Switch | Value
    -------|-------
      1    |   1
      2    |   2
      3    |   4
      4    |   8
      5    |  16
      6    |  32
      7    |  64
      8    | 128

```
```

      Switch#     |   Hex   | Decimal
  8 7 6 5 4 3 2 1 | Node ID | Node ID
  ----------------|---------|---------
  0 0 0 0 0 0 0 0 |    not allowed  <-.
  0 0 0 0 0 0 0 1 |    1    |    1    |
  0 0 0 0 0 0 1 0 |    2    |    2    |
  0 0 0 0 0 0 1 1 |    3    |    3    |
      . . .       |         |         |
  0 1 0 1 0 1 0 1 |   55    |   85    |
      . . .       |         |         + Don't use 0 or 255!
  1 0 1 0 1 0 1 0 |   AA    |  170    |
      . . .       |         |         |
  1 1 1 1 1 1 0 1 |   FD    |  253    |
  1 1 1 1 1 1 1 0 |   FE    |  254    |
  1 1 1 1 1 1 1 1 |   FF    |  255  <-'


```

## Tiara


### （型号未知）


  - 来自 Christoph Lameter <cl@gentwo.org>



```


  ----------------------------------------------- tiara
  Tiara LanCard of Tiara Computer Systems.

  +----------------------------------------------+
  !           ! Transmitter Unit !               !
  !           +------------------+             -------
  !          MEM                              Coax Connector
  !  ROM    7654321 <- I/O                     -------
  !  :  :   +--------+                           !
  !  :  :   ! 90C66LJ!                         +++
  !  :  :   !        !                         !D  Switch to set
  !  :  :   !        !                         !I  the Nodenumber
  !  :  :   +--------+                         !P
  !                                            !++
  !         234567 <- IRQ                      !
  +------------!!!!!!!!!!!!!!!!!!!!!!!!--------+
	       !!!!!!!!!!!!!!!!!!!!!!!!

```

- 0 = 宸插畨瑁呰烦绾。
- 1 = 断开（开路）

顶部跳线7 = ROM 使能54 = 内存位置21 = I/O

内存位置设置（顶部跳线排

===     ================
456     Address selected
===     ================
000	C0000
001     C4000
010     CC000
011     D0000
100     D4000
101     D8000
110     DC000
111     E0000
===     ================

I/O 地址设置（顶部跳线排

===     ====
123     Port
===     ====
000	260
001	290
010	2E0
011	2F0
100	300
101	350
110	380
111	3E0
===     ====

IRQ 选择设置（底部跳线排

====== =====
234567
====== =====
011111 IRQ 2
101111 IRQ 3
110111 IRQ 4
111011 IRQ 5
111110 IRQ 7
====== =====

## 其他网卡


目前我没有任何关于其他型ARCnet 网卡的信息

感谢
