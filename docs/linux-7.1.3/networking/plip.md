## PLIP：并行口 Internet 协议（Parallel Line Internet Protocol）设

作者：Donald Becker <becker@super.org>
机构：Supercomputing Research Center, Bowie MD 20715
文本贡献：Tommy Thorn <tthorn@daimi.aau.dk>

### PLIP 简

本文档描述并行口数据包推送器（Net/LGX）。该设备接口允许将两个并行口以点对点方式连接，并呈现为一IP 网络接口

## PLIP 是什么？

PLIP Parallel Line IP，它通过并行口传IP 数据包。对PC 而言，打印机端口显然是最佳选择。PLIP 并非标准协议；它使用标准LapLink 零调制解调器（null-printer）电缆，并可turbo 模式下工作（而非 PLIP 专用电缆）。该协议使用打包IP 数据包，Crynwr 发起，实现简单

## PLIP 的优

它便宜、随处可得、且易于使用。连接两Linux 机器只需要一PLIP 电缆，其制作成本仅为几美元。将两台 Linux 机器连起来只需片刻决定和几分钟的工夫，无需去寻找受支持的网卡。这一点对笔记本电脑尤其重要，因为网卡并非随手可得。不依赖网卡意味着：除了连接电缆之外，其余一切都只是软件配置，原则上可以做得非常简单

## PLIP 的缺

它不能像 SLIP PPP 那样通过调制解调器工作。传输距离有限，约为 15 米。用它来连接三台（？）Linux 机器。它无法接入已有的以太网。它也不是一个标准（就连 SLIP 那样的事实上标准都算不上）

## 性能

PLIP 轻易便能胜过以太网卡……（哎呀，我在做梦吧，时间不早了。EOB

### PLIP 驱动细节

Linux PLIP 驱动实现了最初的 Crynwr 协议，并使用内核中的并行口子系统，以正确地在各项 PLIP 服务之间共享并行口

## IRQ 触发超时

PLIP 驱动所使用的并行口配置IRQ 后，每当有数据通过电缆发送、或有数据可用、且驱动并未在使用该端口时，PLIP 驱动就会收到通知。然而，在某些机器上，为特定并行口配IRQ 很困难甚至不可能（主要是那些使用了该设备的机器）。在这类机器上，PLIP 驱动会采用无 IRQ 模式：它会不断地轮询并行口以等待数据，进程数据可用。这种模式比 IRQ 模式的效率要低，因为驱动每秒要检查并行口很多次，即便并没有数据在发送。粗略的测量表明，在所涉及的数据传输速度下，使用IRQ 模式相比 IRQ 模式并没有明显的性能下降。性能下降体现在承载该驱动的机器上

PLIP 驱动使用 IRQ 模式时，在数据传送（即驱动允许对端宣告超时、并尝试通过握手完成数据传输的最长时间）时会使用一个超时值，默认500 微秒。由IRQ 的递送或多或少是即时的，这个超时值已相当充裕

在无 IRQ 模式下，PLIP 驱动每秒轮询并行HZ 次（在大多数平台HZ 通常100，在 Alpha 上为 1024，撰写时如此）。两次轮询之间相10^6/HZ 微秒。例如在 i386 上，10^6/100 = 10000 微秒。很容易看出，触发超时完全有可能在两次轮询之间到期，此时 500 微秒的超时值就太长了。因此，必须PLIP 连接一端的触发超时改为 10^6/HZ 微秒。PLIP 连接的两端若都使用无 IRQ 模式，则两端都必须使用这个超时值

实践中，触发超时往往比计算值更短。除非线缆有故障，否则这不是一个重要问题；若线缆故障，过长的超时值会使机器停滞，无论何种原因导致位丢失

执行此修改的工具Linux 上的 plipconfig，它属于 net-tools 软件包（其位置可doc/Changes 文件中找到）。示例命令为 'plipconfig plipX trigger 10000'，其plipX 是相应的 PLIP 设备

### PLIP 硬件互联

PLIP 使用几种不同的数据传输方式。第一种（在代码早期版本中实现）使用标准的打印机“零调制解调器”电缆，通过数据位输出连接到状态位输入，每次传4 位数据

第二种数据传输方式依赖机器拥有双向并行口，而非仅输出的“打印机”端口。这允许以字节为单位传输，避免了把半字节（nibble）重新拼装成字节，从而带来快得多的传输速度

## 并行传输模式 0 电缆

第一种传输模式的电缆即标准的打印机“零调制解调器”电缆，它使用第一个端口（机器 T）的数据位输出连接到第二个端口（机器 R）的状态位输入，每次传4 位数据。在五个状态输入中，使用了四个数据输入和一个时钟（数据选通，strobe）输入，并安排得使数据输入位在标准状态寄存器的实现中呈现为连续的位

该电缆实现了市面上可买到的“Null Printer”或“Turbo Laplink”电缆协议。其连接方式如下
```

    STROBE output	1*
    D0->ERROR	2 - 15		15 - 2
    D1->SLCT	3 - 13		13 - 3
    D2->PAPOUT	4 - 12		12 - 4
    D3->ACK	5 - 10		10 - 5
    D4->BUSY	6 - 11		11 - 6
    D5,D6,D7 are   7*, 8*, 9*
    AUTOFD output 14*
    INIT   output 16*
    SLCTIN	17 - 17
    extra grounds are 18*,19*,20*,21*,22*,23*,24*
    GROUND	25 - 25

    * Do not connect these pins on either end

```
该电缆使用金属屏蔽层，连接到一端金DB-25 外壳上

## 并行传输模式 1

第二种数据传输方式依赖机器拥有双向并行口，而非仅输出的打印机端口。这允许以字节为单位传输，避免了把半字节重新拼装成字节。当机器未配置为 PLIP 时，该电缆使用单向的打印机（而非并行）端口，否则输出驱动会产生冲突（虽不太可能），甚至有可能造成损坏

其连接方式如下：
```

    STROBE->BUSY 1 - 11
    D0->D0	2 - 2
    D1->D1	3 - 3
    D2->D2	4 - 4
    D3->D3	5 - 5
    D4->D4	6 - 6
    D5->D5	7 - 7
    D6->D6	8 - 8
    D7->D7	9 - 9
    INIT -> ACK  16 - 10
    AUTOFD->PAPOUT 14 - 12
    SLCT->SLCTIN 13 - 17
    GND->ERROR	18 - 15
    extra grounds are 19*,20*,21*,22*,23*,24*
    GROUND	25 - 25

    * Do not connect these pins on either end

```
同样，该电缆使用金属屏蔽层，连接到一端金DB-25 外壳上

## PLIP 模式 0 传输协议

PLIP 驱动兼容“Crynwr”并行口传输协议
```

   send header nibble '0x8'
   count-low octet
   count-high octet
   ... data octets
   checksum octet

```

```

	<wait for rx. '0x1?'>	<send 0x10+(octet&0x0F)>
	<wait for rx. '0x0?'>	<send 0x00+((octet>>4)&0x0F)>

```
传输开始时，发送方机器输出半字0x08，并拉高 ACK 线，从而触发接收方机器的中断。接收方机器禁用中断，并拉高自己ACK 线
```

  (OUT is bit 0-4, OUT.j is bit j from OUT. IN likewise)
  Send_Byte:
     OUT := low nibble, OUT.4 := 1
     WAIT FOR IN.4 = 1
     OUT := high nibble, OUT.4 := 0
     WAIT FOR IN.4 = 0

```
