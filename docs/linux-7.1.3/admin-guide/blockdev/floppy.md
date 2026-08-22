## Floppy Driver（软驱驱动）


## FAQ list（常见问题列表）:


常见问题列表可以fdutils 软件包（见下文）中找到，也可以访<https://fdutils.linux.lu/faq.html>

## LILO 配置选项（Thinkpad 用户请阅读本节）


软驱驱动通过 lilo 中的 'floppy=' 选项进行配置。该选项既可以在启动提示符下
输入，也可以写入 lilo 配置文件
示例：如果你的内核名linux-2.6.9，请输入以下一```

 linux-2.6.9 floppy=thinkpad

```
你也可以把下面这行写/etc/lilo.conf 的配置描述中
```

 append = "floppy=thinkpad"

```
```

 linux-2.6.9 floppy=daring floppy=two_fdc
 append = "floppy=daring floppy=two_fdc"

```
如果你同时在 lilo 配置文件和启动提示符下都给出了选项，那么两处的选项字符会被拼接在一起，启动提示符下的选项排在最后。这也是为什么还提供了一些用恢复默认行为的选项

## Module configuration options（模块配置选项

```

	modprobe floppy floppy="<options>"

```
```

	modprobe floppy floppy="omnibook messages"

```
如果你每次加载软驱驱动时都需要启用某些选项，可```

	options floppy floppy="omnibook messages"

```
写入 /etc/modprobe.d/ 目录下的某个配置文件中

软驱驱动相关的选项如下
 floppy=asus_pci
	设置位掩码，只允0 号和 1 号设备。（默认
 floppy=daring
	告诉软驱驱动你拥有一块行为良好的软盘控制器	这样可以获得更高效、更平滑的操作，但在某些控制器上可能会失败	这有可能加快某些操作的速度
 floppy=0,daring
	告诉软驱驱动你的软盘控制器应当谨慎使用
 floppy=one_fdc
	告诉软驱驱动你只有一个软盘控制器	（默认）

 floppy=two_fdc / floppy=<address>,two_fdc
	告诉软驱驱动你有两个软盘控制器	第二个软盘控制器假定位于 <address>	如果第二个控制器位于地址 0x370，并且你使用'cmos' 选项	则不需要此选项
 floppy=thinkpad
	告诉软驱驱动你使用的Thinkpad。Thinkpad 对磁盘更换线
	使用了反转的约定
 floppy=0,thinkpad
	告诉软驱驱动你没有使Thinkpad
 floppy=omnibook / floppy=nodma
	告诉软驱驱动不要使用 Dma 进行数据传输	HP Omnibook 需要使用此选项，因为它没有可用的软DMA 通道	如果你频繁收"Unable to allocate DMA memory" 消息，此选项也很有用	事实上，dma 内存需要在物理内存中连续，因此更难找到，而非 dma 的缓冲区
	可以在虚拟内存中分配。不过，如果你的 FDC 没有 FIFO272A 82072），
	我建议不要使用此选项2072A 及以后的型号都可以。使nodma 至少需486	如果使用 nodma 模式，建议你同时FIFO 阈值设10 或更低，
	以限制数据传输中断的次数
	如果你拥有支FIFO FDC，当找不到可DMA 内存时，软驱驱动会自	回退到非 DMA 模式。如果你想避免这种情况，可以显式地请'yesdma'
 floppy=yesdma
	告诉软驱驱动存在可用DMA 通道	（默认）

 floppy=nofifo
	完全禁用 FIFO。当你在访问软驱时，网卡（或其他设备）报	"Bus master arbitration error" 消息时需要使用此选项
 floppy=usefifo
	启用 FIFO。（默认
 floppy=<threshold>,fifo_depth
	设置 FIFO 阈值。这DMA 模式下最为相关。如果阈值较高，
	软驱驱动可以容忍更多的中断延迟，但会触发更多的中断（即给系统其余部分
	带来更多负载）。如果阈值较低，中断延迟也应该更低（处理器更快）	较低阈值的好处是中断更少
	要调fifo 阈值，可以使用 'floppycontrol --messages' 打开
	over/underrun 消息。然后访问一张软盘。如果你收到大量
	"Over/Underrun - retrying" 消息，说fifo 阈值过低。尝试使用更高的值，
	直到只偶尔出Over/Underrun 为止。在进行此项调优时，最好将软驱驱动
	编译为模块。因为这样就可以在不重启机器的情况下尝试不同fifo 值	注意每次重新插入模块时都需要执'floppycontrol --messages'
	通常不需要调fifo 阈值，因为默认值（0xa）已经比较合理
 floppy=<drive>,<type>,cmos
	<drive> CMOS 类型设为 <type>。如果你拥有超过两个软驱
	（物CMOS 只能描述两个），或者你BIOS 使用了非标准CMOS 类型	则此项为必填。CMOS 类型如下
	       ==  ==================================
		0  Use the value of the physical CMOS
		1  5 1/4 DD
		2  5 1/4 HD
		3  3 1/2 DD
		4  3 1/2 HD
		5  3 1/2 ED
		6  3 1/2 ED
	       16  unknown or not installed
	       ==  ==================================

	（注：ED 驱动器有两个有效类型。这是因为最初选择 5 来表示软*磁带**	6 表示 ED 驱动器。AMI 忽略了这一点，5 用于 ED 驱动器	这就是为什么软驱驱动同时处理两者。）

 floppy=unexpected_interrupts
	当收到意外中断时打印警告消息	（默认）

 floppy=no_unexpected_interrupts / floppy=L40SX
	当收到意外中断时不打印消息。在 IBM L40SX 笔记本电脑的某些视频模式	需要使用此选项。（视频与软驱之间似乎存在相互作用。意外中断只影响性能	可以安全地忽略。）

 floppy=broken_dcl
	不使用磁盘更换线，而是假设每次重新打开设备节点时磁盘都已更换	某些磁盘更换线损坏或不被支持的机器上需要使用此选项	这应被视为一种临时应对措施，因为它会因不必要的缓存刷	而降低软驱操作的效率，并且略微更不可靠。如果你遇到任何 DCL 问题	请检查你的线缆、连接和跳线设置。不过，一些较旧的驱动器，以及部分
	笔记本电脑，已知没有 DCL
 floppy=debug
	打印调试消息
 floppy=messages
	为某些操作打印信息性消息（磁盘更换通知、关over/underrun 的警告，
	以及关于自动检测的消息）
 floppy=silent_dcl_clear
	使用一种更安静的方式清除磁盘更换线（不涉及寻道）daring' 选项隐含此项
 floppy=<nr>,irq
	将软IRQ 设为 <nr>，而不6
 floppy=<nr>,dma
	将软DMA 通道设为 <nr>，而不2
 floppy=slow
```

	   PS/2 软驱的步进速率比普通软驱慢得多。在某些更极端的情形下，
	   建议将速度降到默认值的1/4

```
## Supporting utilities and additional documentation（支持工具与附加文档


软驱驱动的额外参数可以在运行时配置。完成此功能的工具可以在 fdutils 软件包中找到该软件包还包含一个新版本mtools，允许访问大容量磁盘（在高密3 1/2 软盘最高可1992K！）。它还包含关于软驱驱动的附加文档
最新版本可以在 fdutils 主页找到
 https://fdutils.linux.lu

fdutils 发布版本可以在以下地址找到
 https://fdutils.linux.lu/download.html

 http://www.tux.org/pub/knaff/fdutils/

 ftp://metalab.unc.edu/pub/Linux/utils/disk-management/

## Reporting problems about the floppy driver（报告软驱驱动的问题

如果你有关于软驱驱动的问题或缺陷报告，请发邮件给我：Alain.Knaff@poboxes.com如果你在 Usenet 上发帖，最好使comp.os.linux.hardware。由于这些新闻组
流量相当大，请务必在主题行中包含 "floppy"（或 "FLOPPY"）字样如果报告的问题发生在挂载软盘时，请务必在主题行中同时提及文件系统的类型
在发邮件或发帖报告任何缺陷之前，请务必先阅读 FAQ
Alain

## Changelog（变更日志）


10-30-2004 :
		Cleanup, updating, add reference to module configuration.
		James Nelson <james4765@gmail.com>

6-3-2000 :
		Original Document
