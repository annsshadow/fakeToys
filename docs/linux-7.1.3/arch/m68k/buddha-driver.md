# Amiga Buddha Catweasel IDE 驱动


Amiga Buddha Catweasel IDE 驱动（ide.c 的一部分）由 Geert Uytterhoeven 基于以下规范编写

------------------------------------------------------------------------

Buddha IDE 控制器以Catweasel Zorro-II 版本Buddha 部分的寄存器映射

Autoconfiguration（自动配置）的实现与 Commodore 在其手册中描述的完全一致，没有使用任何技巧（例如将某些地址线排除在等式之外……）。如果你想自己配置这块板卡（例如Linux 内核来配置该卡），请参Commodore
```

  Vendor number: 4626 ($1212)
  product number: 0 (42 for Catweasel Z-II)
  Serial number: 0
  Rom-vector: $1000

```
该卡应为 Z-II 板，大小64K，不进入 freemem 列表，Rom 向量有效，同一卡上没有第二Autoconfig 板，无空间偏好，支持 “Shutup_forever”

设置基地址应分两步进行，正Amiga Kickstart 所做的那样 位地址的低半字节（nibble）被写入 $4a，然后将整个字节写入 $48，只要不触碰 $48，你写入 $4a 多少次都无关紧要。在写入 $48 之后，整块卡$e8 消失，并被映射到刚刚写入的新地址。务必先$4a 再写 $48，否则你找到该卡的机会只1:16 :-)

## 即使在映射到 $e8 时，本地内存映射也是激活的

==============  ===========================================
$0-$7e		自动配置空间，参Z-II 文档

## $80-$7fd	保留

$7fe		速度选择寄存器：可读可写
		（说明见下文

## $800-$8ff	IDE 选择 0（端0，寄存器0

## $900-$9ff	IDE 选择 1（端0，寄存器1

## $a00-$aff	IDE 选择 2（端1，寄存器0

## $b00-$bff	IDE 选择 3（端1，寄存器1

$c00-$cff	IDE 选择 4（端2，寄存器0
                Catweasel！）

$d00-$dff	IDE 选择 5（端3，寄存器1
		Catweasel！）

$e00-$eff	本地扩展端口，在 Catweasel Z-II 
		Catweasel 寄存器也映射于此
		切勿触碰，请使用 multidisk.device

$f00		只读，字节访问：7 显示
		IDE 端口 0 IRQ 线电平

## $f01-$f3f	$f00 的镜

$f40		只读，字节访问：7 显示
		IDE 端口 1 IRQ 线电平

## $f41-$f7f	$f40 的镜

$f80		只读，字节访问：7 显示
		IDE 端口 2 IRQ 线电平
		（仅 Catweasel！）

## $f81-$fbf	$f80 的镜

$fc0		只写：向该寄存器写入任意
		会启用从 IDE 端口Zorro 总线IRQ 传递。此机制
		的实现是为了兼容那些存在缺陷或固件有 bug、会
		IRQ 线拉高的硬盘
		while starting up. If interrupts would
		always be passed to the bus, the computer
		might not start up. Once enabled, this flag
		can not be disabled again. The level of the
		flag can not be determined by software
		(what for? Write to me if it's necessary!).

## $fc1-$fff	$fc0 的镜

$1000-$ffff	偏移量为 $1000 Buddha ROM
		chip. The addresses $0 to $fff of the rom
		chip cannot be read. Rom is Byte-wide and
		mapped to even addresses.
==============  ===========================================

IDE 端口会产生一INT2。你可以通过读取三个（仅 Buddha 为两个）寄存$f00f40 $f80 来读IDE 端口IRQ 线电平。这样，可以处理多于一个的 I/O 请求，并且你可以轻松确定哪个驱动必须INT2 提供服务。Buddha Catweasel 扩展板可以产INT6。I/O 模块sysop I/O 模块有单独的内存映射可用

IDE 端口由地址A2 A4 驱动，正Amiga 1200 Amiga 4000 IDE 端口一样。这样，现有的驱动可以轻松移植到 Buddha。一move.l 会从 IDE 端口的同一地址轮询出两个字，因为每个字都会被镜像一次。movem 不可用，但也没有必要，因为你只能用这种技术为 68000 系统提速。带fastmem 68020 系统在使move.l 时更快

如果你使IDE 端口的镜像寄存器A6=1，Buddha 不会理会你在速度寄存器（见下文）中选择的速度。当 A6=1 时（例如端口 0、寄存器0 对应 $840），会进行一780ns 的访问。这些寄存器应用于对硬盘/光盘的命令访问，因为命令访问是字节宽度，必须根据 ATA-X3T9 手册放慢速度

现在说说速度寄存器：该寄存器为字节宽度，仅使用高三位（位 7 5）。位 4 必须始终设为 1，以兼容后续版本Buddha（如果我以后更新这个的话）。我假定永远不会使用低四位，但它们按定义必须设为 1

此表中的值必须左5 位，并与 $1f 进行或运算（这会设置5 位）

所有时序的共同点：Select IOR/IOW 同时上升。IOR IOW Zorro 总线上的时钟约有 30ns 的传播延迟，这就是为什么这些值不71 的整数倍。一个时钟周期为 71ns（在 PAL 系统 14.18 Mhz 下恰好为 70.5ns）

0（复位后的默认值）
  497ns Select 个时钟周期），IOR/IOW 172ns 后（2 个时钟周期）
  （与 Amiga 1200 在其 IDE 端口上、无加速卡时相同）

鍊?1
  639ns Select 个时钟周期），IOR/IOW 243ns 后（3 个时钟周期）

鍊?2
  781ns Select1 个时钟周期），IOR/IOW 314ns 后（4 个时钟周期）

鍊?3
  355ns Select 个时钟周期），IOR/IOW 101ns 后（1 个时钟周期）

鍊?4
  355ns Select 个时钟周期），IOR/IOW 172ns 后（2 个时钟周期）

鍊?5
  355ns Select 个时钟周期），IOR/IOW 243ns 后（3 个时钟周期）

鍊?6
  1065ns Select5 个时钟周期），IOR/IOW 314ns 后（4 个时钟周期）

鍊?7
  355ns Select 个时钟周期），IOR/IOW 101ns 后（1 个时钟周期）

当以 A6=1 访问 IDE 寄存器（例如 $84x）时，无论你在速度寄存器中选择什么，时序将始终是模式 0 8 位兼容：

781ns select，IOR/IOW 4 个时钟周期（=314ns）后激活

所有具有极select 信号55ns 的快速访问）的时序都取决于系统中使用的加速卡：有时总线接口会插入两个额外的时钟周期，使整个访问长达 497ns。这不会影响控制器的可靠性，也不会影响卡的性能，因为这种情况不常发生

所有时序都是计算得出的，并且仅通过了允许我统计时钟周期的测量得以确认。如果系统由28.37516 Mhz 以外的振荡器提供时钟（例NTSC 频率 28.63636 Mhz），每个时钟周期会缩短到略小70ns（不值一提）。你可能会想到通过超频系统来获得小幅性能提升，但那样你将需要一台多频显示器或一块显卡，而且你的内部软驱会变得不正常，因此你不应该以这种方式调优你的 Amiga

为了让你可以编写同时兼容 Buddha Catweasel Z-II 的软件，Buddha 表现得就像一块没有设备连接到第三IDE 端口Catweasel Z-II。IRQ 寄存$f80 Buddha 上始终显“此处无 IRQ”，而对第三IDE 端口的访问在 Buddha 上会进入数据的虚无

## Jens Sch枚nfeld 1997 骞?2 鏈?19 鏃。

## 鏇存柊浜?1997 骞?5 鏈?27 鏃。

## 电子邮件：sysop@nostlgic.tng.oche.de
