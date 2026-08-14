
## ARCnet


:Author: Avery Pennarun <apenwarr@worldvisions.ca>


   如果你和我们中许多人一样，碰巧没有拿到 ARCnet 卡的手册，请参见本目录下的
   arcnet-hardware.txt，以获取跳线设置和线缆连接信息。

既然似乎没人会听我的，也许一首诗能让你听进去
```

		This driver's getting fat and beefy,
		But my cat is still named Fifi.

```

嗯，我觉得我可以把那称作一首诗，尽管它只有两行。嘿，我是学计算机科学的，不是
学英语的。饶了我吧。

重点是：如果你测试了这个驱动并让它工作（或者没工作），或者其他任何情况，
我真的真的真的真的真的很想听到你的消息。

ARCnet 0.32 ALPHA 首次进入 Linux 内核 1.1.80 —— 这很好，但在那之后，甚至更
少的人开始给我写信，因为他们甚至不需要安装这个补丁了。<叹气>

来吧，做个够意思的人！给我发一份成功报告！

（嘿，这比我原来的诗还要好……这越来越糟了！）

----

以下是 Linux 的 ARCnet 驱动。

这个新版本（2.91）由 David Woodhouse <dwmw2@infradead.org> 整理，目的是在添加
对又一种芯片组的支持之后整理这个驱动。现在通用支持已经从各个芯片组驱动中分离
出来，源文件也不再塞满 #ifdefs 了！我对这个文件做了一点修改，但保留了 Avery 的
第一人称口吻，因为我不想完全重写它。

上一个版本来自我（Avery Pennarun）断断续续数月的努力、来自其他人的许多 bug 报告
/修复和建议，特别是来自 Tomasz Motylewski 的大量输入和代码。从 ARCnet 2.10 ALPHA
开始，Tomasz 全新改进的 RFC1051 支持被纳入，并且似乎工作正常！



### 我在哪里讨论这些驱动？


ARCnet 的讨论在 netdev 上进行。只需将你的邮件发送到 netdev@vger.kernel.org，
并确保抄送（Cc）Documentation/process/maintainers.rst 中“ARCNET NETWORK LAYER”
标题下列出的维护者。

### 其他驱动与信息


你可以访问我在万维网（World Wide Web）上的 ARCNET 页面：

	http://www.qis.net/~jschmitz/arcnet/

另外，SMC（生产 ARCnet 卡的公司之一）有一个你可能会感兴趣的 WWW 站点，其中包含
多个支持包括 ARCnet 在内的各种网卡的驱动。试试：

	http://www.smc.com/

Performance Technologies 出品各种支持 ARCnet 的网络软件：

	http://www.perftech.com/ 或通过 FTP 访问 ftp.perftech.com。

Novell 出品一个包含 ARCnet 驱动的 DOS 网络协议栈。试试 FTP 到 ftp.novell.com。

你可以从 oak.oakland.edu:/simtel/msdos/pktdrvr 获取 Crynwr 包驱动集合（包括
arcether.com，也就是你想配合 ARCnet 卡使用的那个）。不过它在未经打补丁的情况下
在 386+ 上无法完美工作，并且也不喜欢某些网卡。修正版本可以在我的 WWW 页面上获取，
或者如果你没有 WWW 访问权限，也可以通过电子邮件获取。


### 安装驱动


```

	make config
		(be sure to choose ARCnet in the network devices
		and at least one chipset driver.)
	make clean
	make zImage

```

如果你获得这个 ARCnet 软件包，是作为对你当前内核中 ARCnet 驱动的一个升级，你
需要先把 arcnet.c 复制到 linux/drivers/net 目录上覆盖原有的文件。

如果你在重新启动进入新的 Linux 内核时看到一些 ARCnet 消息，你就知道驱动已经
正确安装了。

有四种芯片组选项：

 1. 标准 ARCnet COM90xx 芯片组。

这是普通的 ARCnet 卡，你很可能用的就是它。这是唯一一个在没被告知卡的位置时会
自动探测的芯片组驱动。
```

 com90xx=[<io>[,<irq>[,<shmem>]]][,<name>] | <name>

```
```

 io=<io> irq=<irq> shmem=<shmem> device=<name>

```

要禁用自动探测，只需在内核命令行上指定 "com90xx="。要只指定名称，但允许自动
探测，只需写 "com90xx=<name>"

 2. ARCnet COM20020 芯片组。

这是 SMC 出品的新型芯片组，支持混杂模式（数据包嗅探）、额外的诊断信息等。
不幸的是，没有合理的方法可以自动探测这些卡。你必须在内核命令行上指定 I/O 地址。

```

 com20020=<io>[,<irq>[,<node_ID>[,backplane[,CKP[,timeout]]]]][,name]

```
```

 io=<io> irq=<irq> node=<node_ID> backplane=<backplane> clock=<CKP>
 timeout=<timeout> device=<name>

```

COM20020 芯片组允许你通过软件设置节点 ID，覆盖默认（仍然由卡上的 DIP 开关设置）
的值。如果你没有 COM20020 的数据手册，并且你不知道其他三个选项指的是什么，那
它们不会让你感兴趣 —— 忘了它们吧。

 3. IO 映射模式下的 ARCnet COM90xx 芯片组。

这也能用于普通的 ARCnet 卡，但不使用共享内存。它的性能不如上面的驱动，但提供它
是考虑到你有一张不支持共享内存的卡，或者（奇怪地）考虑到你机器里的 ARCnet 卡
太多而导致共享内存插槽用完了。如果你不在内核命令行上给出 I/O 地址，那么驱动将
找不到这张卡。

```

 com90io=<io>[,<irq>][,<name>]

```

如果你把芯片组支持作为模块加载，选项是：
 io=<io> irq=<irq> device=<name>

 4. ARCnet RIM I 卡。

这些是*完全*内存映射的 COM90xx 芯片。对这些卡的支持未经测试。如果你有这种卡，
请给作者发邮件并附上成功报告。除设备名外，所有选项都必须指定。
```

 arcrimi=<shmem>,<irq>,<node_ID>[,<name>]

```
```

 shmem=<shmem> irq=<irq> node=<node_ID> device=<name>


```

### 可加载模块支持


配置并重新编译 Linux。当被问到时，若你想使用可加载模块，对“Generic ARCnet
support”以及对你的 ARCnet 芯片组的支持回答 'm'。你也可以对“Generic ARCnet
support”回答 'y'，而对芯片组支持回答 'm'，随你愿意。

```

	make config
	make clean
	make zImage
	make modules

```

如果你使用可加载模块，你需要用 insmod 来加载它，并且可以在命令行上指定你卡的
各种特性。（在驱动的较新版本中，自动探测可靠得多，并且作为模块也能工作，所以
这些现在大多不必要了。）

```

	cd /usr/src/linux/modules
	insmod arcnet.o
	insmod com90xx.o
	insmod com20020.o io=0x2e0 device=eth1


```

### 使用驱动


如果你编译内核时包含了 ARCnet COM90xx 支持，它应该在你启动时自动探测你的卡。
如果你使用编译进内核的其他芯片组驱动，你必须如上所述在内核命令行上给出必要的
选项。

去读 Linux 的 NET-2-HOWTO 和 ETHERNET-HOWTO；它们应该和你拿到这个驱动的同一处
可以获取到。把你的 ARCnet 当作一块加强版（或弱化版，视情况而定）的以太网卡。

顺便说一句，一定要在 HOWTO 中把所有对 "eth0" 的引用改为 "arc0"。记住 ARCnet 并
不是“真正的”以太网，设备名是*不同*的。


### 一台计算机中多张卡


Linux 现在对此有相当好的支持，但由于我一直很忙，ARCnet 驱动在这方面多少有些
落后。如果编译进内核，COM90xx 支持会（尝试）自动探测所有已安装的卡。

如果你有其他卡，并且其支持编译进了内核，那么你可以
```

	LILO: linux com20020=0x2e0 com20020=0x380 com90io=0x260

```

如果你把芯片组支持构建为可加载模块，那么你需要
```

	insmod -o arc0 com90xx
	insmod -o arc1 com20020 io=0x2e0
	insmod -o arc2 com90xx

```

ARCnet 驱动现在会自动整理它们的名称。


### 我如何让它与……一起工作？


NFS：
	linux 到 linux 应该没问题，就当自己在使用以太网卡。
	oak.oakland.edu:/simtel/msdos/nfs 有一些不错的 DOS 客户端。还有
	一个名为 SOSS 的、基于 DOS 的 NFS 服务器。它的多任务方式和 Linux
	不太一样（实际上，它根本不 multitask），但你永远不知道你会需要什么。

	对于 AmiTCP（可能还有其他），你可能需要在你的 Amiga nfstab 中
	设置以下选项：MD 1024 MR 1024 MW 1024
	（感谢 Christian Gottschling <ferksy@indigo.tng.oche.de>
	提供此信息。）

	大概这些指的是最大 NFS 数据/读/写块大小。我不知道为什么 Amiga 上的
	默认值不行；如果你知道更多，请写信给我。

DOS：
	如果你使用的是免费软件 arcether.com，你可能想安装来自我网页上的
	驱动补丁。它对 PC/TCP 有帮助，并且也能让 arcether 在初始化时
	超时太快的情况下加载。事实上，如果你在 386+ 上使用它，你确实
	真的需要这个补丁。

Windows：
	参见 DOS :) Trumpet Winsock 配合 Novell 或 Arcether 客户端都能
	正常工作，当然前提是你记得加载 winpkt。

LAN Manager 和 Windows for Workgroups：
	这些程序使用的协议与 Internet 标准不兼容。它们试图假装这些卡是
	以太网，并把网络上其他所有人都搞糊涂。

	不过，v2.00 及更高版本的 Linux ARCnet 驱动通过 'arc0e' 设备支持
	这个协议。更多信息参见“多协议支持”一节。

	使用免费的 Linux Samba 服务器和客户端，你现在可以与基于 TCP/IP 的
	WfWg 或 Lan Manager 网络相当友好地互联。

Windows 95：
	Win95 自带工具，让你使用 LANMAN 风格的网络驱动（NDIS）或 Novell
	驱动（ODI）来处理你的 ARCnet 数据包。如果你使用 ODI，你需要对
	Linux 使用 'arc0' 设备。如果你使用 NDIS，那么试试 'arc0e' 设备。
	如果你需要 arc0e，你完全疯了，和/或你需要构建某种同时使用两种
	封装类型的混合网络，请参见下面的“多协议支持”一节。

OS/2：
	有人告诉我它在 Warp Connect 下配合来自 SMC 的 ARCnet 驱动可以工作。
	为此你需要使用 'arc0e' 接口。如果你让 SMC 驱动配合“普通”Warp
	Bonus Pack 中包含的 TCP/IP 部分工作，请告诉我。

	ftp.microsoft.com 上还有一个免费的“Lan Manager for OS/2”客户端，
	它应该使用和 WfWg 相同的协议。不过我在 Warp 下安装它没成功。
	如有任何结果请告诉我。

NetBSD/AmiTCP：
	它们使用旧版本的 Internet 标准 ARCnet 协议（RFC1051），该协议与
	Linux 驱动 v2.10 ALPHA 及更高版本使用 arc0s 设备兼容。（参见下面
	的“Multiprotocol ARCnet”。）** 较新版本的 NetBSD 显然支持 RFC1201。


### 使用多协议 ARCnet


ARCnet 驱动 v2.10 ALPHA 支持三种协议，每种都在其自己的“虚拟网络设备”上：

	======  ===============================================================
	arc0	RFC1201 协议，是官方的 Internet 标准，恰好与 Novell 的 TRXNET
		驱动 100% 兼容。ARCnet 驱动的 1.00 版本*只*支持这一协议。
		arc0 是三种协议中速度最快的（不管什么原因），并且允许使用
		更大的数据包，因为它支持 RFC1201 的“数据包拆分”操作。除非
		你有特定需要使用不同的协议，我强烈建议你坚持使用这一种。

	arc0e	“以太网封装（Ethernet-Encapsulation）”，通过 ARCnet 发送
		实际上非常像以太网数据包的数据包，包括 6 字节的硬件地址。
		该协议与 Microsoft 的 NDIS ARCnet 驱动兼容，如 WfWg 和 LANMAN
		中的那个。由于 493 的 MTU 实际上比 TCP/IP“要求”的（576）更小，
		某些网络操作有可能无法正常工作。不过，Linux 的 TCP/IP 层在
		大多数情况下可以通过自动分片 TCP/IP 数据包来使它们适应。arc0e
		也比 arc0 稍慢一些，原因尚未确定。（大概就是更小的 MTU 造成的。）

	arc0s	“[s]imple” RFC1051 协议是新标准完全不兼容的“旧” Internet
		标准。不过，今天有些软件继续支持（且只支持）旧标准，包括
		NetBSD 和 AmiTCP。RFC1051 也不支持 RFC1201 的数据包拆分，而
		507 的 MTU 仍然小于 Internet“要求”，所以你很可能会遇到问题。
		出于和 arc0e 相同的原因，它也比 RFC1201 慢约 25%。

		arc0s 支持由 Tomasz Motylewski 贡献，并由我做了一些修改。bug
		大概是我的错。
	======  ===============================================================

如果你愿意，你可以选择不把 arc0e 和 arc0s 编译进驱动 —— 这会省下一点内存，并
避免例如在使用近期 Linux 内核中的“NFS-root”功能时的混乱。

当你第一次 ifconfig arc0 设备时，arc0e 和 arc0s 设备会自动创建。但要真正使用
它们，你还需要 ifconfig 你需要的其他虚拟设备。然后你可以用多种方式设置你的
网络：


1. 单一协议。

   这是配置网络最简单的方式：只使用两种可用协议之一。如上所述，除非你有充分
   理由（比如某些其他软件，即 WfWg，只与 arc0e 一起工作），否则只使用 arc0 是
   个好主意。

```

	ifconfig arc0 MY.IP.ADD.RESS
	route add MY.IP.ADD.RESS arc0
	route add -net SUB.NET.ADD.RESS arc0
	[add other local routes here]

   If you need arc0e (and only arc0e), it's a little different::

	ifconfig arc0 MY.IP.ADD.RESS
	ifconfig arc0e MY.IP.ADD.RESS
	route add MY.IP.ADD.RESS arc0e
	route add -net SUB.NET.ADD.RESS arc0e

   arc0s works much the same way as arc0e.


```

2. 同一根线上使用多个协议。

   现在事情开始变得混乱了。要尝试它，你可能得有点疯。这是我（**我**）的做法。
   :) 注意我的家庭网络中没有包含 arc0s；我没有 NetBSD 或 AmiTCP 计算机，所以我
   只在有限的测试中使用 arc0s。

   我的家庭网络上有三台计算机：两台 Linux 机器（由于上面列出的原因偏好
   RFC1201 协议）和一台不能运行 Linux、但运行免费的 Microsoft LANMAN 客户端的
   XT。

   更糟的是，其中一台 Linux 计算机（freedom）还有一个调制解调器，并充当到我
   Internet 提供商的路由器。另一台 Linux 机器（insight）也有自己的 IP 地址，
   并需要使用 freedom 作为其默认网关。而 XT（patience）没有自己的 Internet IP
   地址，所以我在一个“私有子网”上（如 RFC1597 定义的）给它分配了一个。

   先从一个只有 insight 和 freedom 的简单网络开始。insight 需要：

 - 通过 RFC1201（arc0）协议与 freedom 通信，因为我更喜欢它，而且它更快。
 - 使用 freedom 作为其 Internet 网关。

```

	ifconfig arc0 insight
	route add insight arc0
	route add freedom arc0	/* I would use the subnet here (like I said
					to in "single protocol" above),
					but the rest of the subnet
					unfortunately lies across the PPP
					link on freedom, which confuses
					things. */
	route add default gw freedom

   And freedom gets configured like so::

	ifconfig arc0 freedom
	route add freedom arc0
	route add insight arc0
	/* and default gateway is configured by pppd */

   Great, now insight talks to freedom directly on arc0, and sends packets
   to the Internet through freedom.  If you didn't know how to do the above,
   you should probably stop reading this section now because it only gets
   worse.

   Now, how do I add patience into the network?  It will be using LANMAN
   Client, which means I need the arc0e device.  It needs to be able to talk
   to both insight and freedom, and also use freedom as a gateway to the
   Internet.  (Recall that patience has a "private IP address" which won't
   work on the Internet; that's okay, I configured Linux IP masquerading on
   freedom for this subnet).

   So patience (necessarily; I don't have another IP number from my
   provider) has an IP address on a different subnet than freedom and
   insight, but needs to use freedom as an Internet gateway.  Worse, most
   DOS networking programs, including LANMAN, have braindead networking
   schemes that rely completely on the netmask and a 'default gateway' to
   determine how to route packets.  This means that to get to freedom or
   insight, patience WILL send through its default gateway, regardless of
   the fact that both freedom and insight (courtesy of the arc0e device)
   could understand a direct transmission.

   I compensate by giving freedom an extra IP address - aliased 'gatekeeper' -
   that is on my private subnet, the same subnet that patience is on.  I
   then define gatekeeper to be the default gateway for patience.

   To configure freedom (in addition to the commands above)::

	ifconfig arc0e gatekeeper
	route add gatekeeper arc0e
	route add patience arc0e

   This way, freedom will send all packets for patience through arc0e,
   giving its IP address as gatekeeper (on the private subnet).  When it
   talks to insight or the Internet, it will use its "freedom" Internet IP
   address.

   You will notice that we haven't configured the arc0e device on insight.
   This would work, but is not really necessary, and would require me to
   assign insight another special IP number from my private subnet.  Since
   both insight and patience are using freedom as their default gateway, the
   two can already talk to each other.

   It's quite fortunate that I set things up like this the first time (cough
   cough) because it's really handy when I boot insight into DOS.  There, it
   runs the Novell ODI protocol stack, which only works with RFC1201 ARCnet.
   In this mode it would be impossible for insight to communicate directly
   with patience, since the Novell stack is incompatible with Microsoft's
   Ethernet-Encap.  Without changing any settings on freedom or patience, I
   simply set freedom as the default gateway for insight (now in DOS,
   remember) and all the forwarding happens "automagically" between the two
   hosts that would normally not be able to communicate at all.

   For those who like diagrams, I have created two "virtual subnets" on the
   same physical ARCnet wire.  You can picture it like this::


	  [RFC1201 NETWORK]                   [ETHER-ENCAP NETWORK]
      (registered Internet subnet)           (RFC1597 private subnet)

			     (IP Masquerade)
	  /---------------\         *            /---------------\
	  |               |         *            |               |
	  |               +-Freedom-*-Gatekeeper-+               |
	  |               |    |    *            |               |
	  \-------+-------/    |    *            \-------+-------/
		  |            |                         |
	       Insight         |                      Patience
			   (Internet)


```

### 它工作了：现在做什么？


按照 arcnet-netdev 发邮件。描述你的设置，最好包括驱动版本、内核版本、ARCnet 卡
型号、CPU 类型、网络上系统数量，以及正在使用的软件列表。

### 它不工作：现在做什么？


做和上面一样的事，但还要在邮件中附上 ifconfig 和 route 命令的输出，以及任何
相关的日志条目（即从上次重启以来出现的、以 "arcnet:" 开头的任何内容）。

如果你想尝试自己修复它（我强烈建议你先就这个问题给我发邮件，因为它可能已经被
解决了），你也许想尝试一些可用的调试级别。对于 D_DURING 或更高级别的重度测试，
先杀掉你的 klogd 守护进程会是个*非常*好的主意！D_DURING 为每个发送或接收的数据
包显示 4-5 行。D_TX、D_RX 和 D_SKB 实际上会显示每个发送或接收的数据包，这显然
相当大。

从 v2.40 ALPHA 开始，自动探测例程有了重大改变。特别是，除非你打开 D_INIT_REASONS
调试标志，否则它们不会告诉你为什么没找到卡。

一旦驱动运行起来，你可以作为 root 随时运行 arcdump shell 脚本（可以从我这里，或
在你有的完整 ARCnet 软件包中获取）来列出 arcnet 缓冲区的内容。要从中看出任何
意义，你应该获取相关的 RFC。（有些列在 arcnet.c 顶部附近。）arcdump 假定你的卡
在 0xD0000。如果不是，请编辑该脚本。

缓冲区 0 和 1 用于接收，缓冲区 2 和 3 用于发送。乒乓缓冲（ping-pong buffers）
在两个方向上都实现了。

如果你的调试级别包含 D_DURING 并且你没有定义 SLOW_XMIT_COPY，那么每次卡被复位
时（这只应该发生在你做 ifconfig up 时，或者当 Linux 判定驱动已损坏时），缓冲区
都会被清成一个常量值 0x42。在发送过程中，缓冲区的未使用部分也会被清成 0x42。
这是为了更容易弄清楚一个数据包使用了哪些字节。

```

	ifconfig arc0 down metric 1xxx
	/etc/rc.d/rc.inet1

```

其中 "xxx" 是你想要的调试级别。例如，"metric 1015" 会把你置于调试级别 15。调试
级别 7 目前是默认值。

注意（从 v1.90 ALPHA 开始）调试级别是不同调试标志的二进制组合；所以调试级别 7
实际上是 1+2+4，即 D_NORMAL+D_EXTRA+D_INIT。要包含 D_DURING，你需要再加上 16，
得到调试级别 23。

如果你不明白这个，你可能反正也不想知道。就你所遇到的问题给我发邮件吧。


### 我想寄钱：现在做什么？


去睡个午觉或做点别的。你早上起来会感觉好些。
