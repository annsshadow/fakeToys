Parport
+++++++

`parport` 代码Linux 下提供并行端口支持。这包括在多个设备驱动之间共享一个端口的能力
您可以向 `parport` 代码传递参数以覆盖它对硬件的自动检测。如果您想使IRQ，这尤其有用，因为一般来说这IRQ 无法被自动探测成功。默认情况下，即IRQ **可以**被探测到，也不会使用它们。这是因为有很多人将同一IRQ 同时用于他们的并行端口和声卡或网卡
`parport` 代码分为两部分：通用部分（处理端口共享）和依赖于架构的部分（处理实际对端口的使用）

## Parport 作为模块


```

	# insmod parport

```
用于加载通用`parport` 代码。然后您必须加载

```

	# insmod parport_pc io=0x3bc,0x378,0x278 irq=none,7,auto

```
以告`parport` 代码您想要三PC 风格的端口，一个位0x3bc 且无 IRQ，一个位0x378 使用 IRQ 7，一个位0x278 使用自动检测的 IRQ。目前支PC 风格（`parport_pc`）、Sun `bpp`、Amiga、Atari MFC3 硬件
PCI 并行 I/O 卡的支持来自 `parport_pc`。对于受支持PCI 卡不应指定基地址 I/O，因为它们会被自动检测

### modprobe


如果您使modprobe，将如下面的行添加到

```

	alias parport_lowlevel parport_pc
	options parport_pc io=0x378,0x278 irq=7,auto

```
每当加载并行端口设备驱动（例`lp`）时，modprobe 都会加载 `parport_pc`（带有选项 `io=0x378,0x278 irq=7,auto`）
请注意，这些仅仅是示例行！一般来说，为了能够使用并行端口，您不应该需要为 `parport_pc` 指定任何选项

### Parport 探测 [可选]


2.2 内核中有一个名`parport_probe` 的模块，用于收集 IEEE 1284 设备 ID 信息。现在它已被增强，并IEEE 1284 支持放在一起。当检测到一个并行端口时，连接到它的设备会被分析
```

	parport0: Printer, BJC-210 (Canon)

```
探测信息可以`/proc/sys/dev/parport/` 中的文件获取

## Parport 静态链接进内核


如果您将 `parport` 代码编译进内核，那么您可以使用内核引导参数来获得相同的效果。添加类似下面的语句

```

	parport=0x3bc parport=0x378,7 parport=0x278,auto,nofifo

```
您可以有多个 `parport=...` 语句，每个要添加的端口对应一个。在内核命令行中添加 `parport=0` 将完全禁parport 支持。在内核命令行中添加 `parport=auto` 将使 `parport` 使用它自动检测到的任IRQ 线或 DMA 通道

## /proc 中的文件


如果您将 `/proc` 文件系统配置进内核，您会看到一个新的目录项：`/proc/sys/dev/parport`。其中会有一个目录项对应已配parport 的每个并行端口。在每个这样的目录中，都有一组描述该并行端口的文件
```

	parport
	|-- default
	|   |-- spintime
	|   `-- timeslice
	|-- parport0
	|   |-- autoprobe
	|   |-- autoprobe0
	|   |-- autoprobe1
	|   |-- autoprobe2
	|   |-- autoprobe3
	|   |-- devices
	|   |   |-- active
	|   |   `-- lp
	|   |       `-- timeslice
	|   |-- base-addr
	|   |-- irq
	|   |-- dma
	|   |-- modes
	|   `-- spintime
	`-- parport1
	|-- autoprobe
	|-- autoprobe0
	|-- autoprobe1
	|-- autoprobe2
	|-- autoprobe3
	|-- devices
	|   |-- active
	|   `-- ppa
	|       `-- timeslice
	|-- base-addr
	|-- irq
	|-- dma
	|-- modes
	`-- spintime

```

=======================	=======================================================
File			含义
=======================	=======================================================
`devices/active`	使用该端口的设备驱动列表。当前正在使用该端口的设			名称旁会出现一”（它可能不出现在任何名称旁）			字符“none表示没有设备驱动在使用该端口
`base-addr`		并行端口的基地址，如果有多个则列出多个地址，各地址
			之间用制表符分隔。这些值对某些端口可能没有实际意义
`irq`			并行端口IRQ，如果未使用则为 -1
`dma`			并行端口DMA 通道，如果未使用则为 -1
`modes`		并行端口的硬件模式，以逗号分隔，含义如下：

   - PCSPP
				PC 风格SPP 寄存器可用
   - TRISTATE
				端口是双向的
   - COMPAT
				可用于打印机的硬件加速可用且将被使用
   - EPP
				EPP 协议的硬件加速可用且将被使用
   - ECP
				ECP 协议的硬件加速可用且将被使用
   - DMA
				DMA 可用且将被使用
			注意，当前的实现只有在有一条可用的 IRQ 线时			才会利用 COMPAT ECP 模式
`autoprobe`		从（IEEE 1284.3）设备获取的任意 IEEE-1284 设备 ID 信息
`autoprobe[0-3]`	从符IEEE 1284.3 的菊花链设备检索到IEEE 1284 设备 ID 信息
`spintime`		等待外设响应时忙等待的微秒数。您可能会发现，调整			可以改善性能，具体取决于您的外设。这是一个端口范围的设置			即它适用于特定端口上的所有设备
`timeslice`		允许设备驱动保持占用一个端口的毫秒数。这是建议性的			驱动如果必须，可以忽略它
`default/*`		spintime timeslice 的默认值。当注册一个新端口时，
			它会采用默认spintime。当注册一个新设备时，它会采用
			默认timeslice=======================	=======================================================

## 设备驱动


一parport 代码被初始化，您就可以将设备驱动附加到特定的端口。通常这是自动发生的；如果加载lp 驱动，它会为找到的每个端口创建一lp 设备。不过，您可以通过在加lp 时使用参数来覆盖此行
```

	# insmod lp parport=0,2

```

```

	lp=parport0 lp=parport2

```
上面两个示例都会告诉 lp，您希望 `/dev/lp0` 是第一个并行端口，`/dev/lp1` *第三*并行端口，第二个端口（parport1）不关联任何 lp 设备。请注意，这不同于较旧内核的工作方式；以I/O 端口地址和设备名称之间存在静态关联，因此 `/dev/lp0` 总是位于 0x3bc 的端口。现在不再是这样了——如果您只有一个端口，无论基地址如何，它都将默认`/dev/lp0`
另外
 - 如果您在编译时选择IEEE 1284 支持，可以在内核命令行上`lp=auto`，lp 将只为那些似乎连接了打印机的端口创建设备
 - 如果您给 PLIP 提供 `timid` 参数，无论是通过命令行上`plip=timid`，还是使用模块时`insmod plip timid=1`，它都会避开那些似乎正被其他设备使用的端口
 - 目前 IRQ 自动探测只对少数几种端口类型有效
## 报告 parport 的打印问

如果您在打印时遇到问题，请通过以下步骤尝试缩小问题区域
在报parport 问题时，您确实需要给`parport_pc` 在初始化时输出的所有消息。有几种代码路径
- polling（轮询）
- interrupt-driven，协议在软件- interrupt-driven，协议在硬件中，使用 PIO
- interrupt-driven，协议在硬件中，使用 DMA

`parport_pc` 记录的内核消息表明了正在使用哪条代码路径。（实际上它们本可以做得更好..
对于普通的打印机协议，是否启用 IEEE 1284 模式应该没有影响
要关“协议在硬件中的代码路径，请禁`CONFIG_PARPORT_PC_FIFO`。请注意，当它们被启用时，不一定被**使用**；这取决于硬件是否可用、是否被 BIOS 启用、以及是否被驱动检测到
因此，首先要禁用 `CONFIG_PARPORT_PC_FIFO`，并`irq=none` 加载 `parport_pc`。看看那时打印是否工作。它确实应该工作，因为这是最简单的代码路径
如果那工作正常，尝试`io=0x378 irq=7`（根据您的硬件调整），使它使用中断驱动、软件中的协议
如果***也工作正常，那么某个硬件模式工作不正常。启`CONFIG_FIFO`（不，它不是模块选项，而且是的，它应该是），在 BIOS 中将端口设为 ECP 模式，并注意

```

    io=0x378 irq=7 dma=none (用于 PIO)
    io=0x378 irq=7 dma=3 (用于 DMA)

```
----------

philb@gnu.org
tim@cyberelk.net
