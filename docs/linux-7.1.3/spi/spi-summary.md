## Linux 内核 SPI 支持概述


02-Feb-2012

### 什么是 SPI？

“Serial Peripheral Interface”（SPI，串行外设接口）是一种同步的四线串行
链路，用于将微控制器连接到传感器、存储器和外设。它是一个简单的“事实
标准”，还没有复杂到需要成立一个标准化组织。SPI 使用主机/目标（host/target）
配置。

三条信号线包含一条时钟线（SCK，通常约为 10 MHz），以及并行的数据线，
分别承载“主机输出、从机输入”（MOSI）或“主机输入、从机输出”（MISO）
信号。（也有使用其他名称的情况。）共有四种时钟模式用于交换数据；其中
mode-0 和 mode-3 最为常用。每个时钟周期移出和移入数据；时钟只在有数据
位需要移位时才会跳动。不过并非所有数据位都会被使用；并非每个协议都会
用到这些全双工能力。

SPI 主机使用第四条“片选”（chip select）线来激活某个给定的 SPI 目标
设备，因此那三条信号线可以并行连接到多个芯片。所有 SPI 目标都支持片选；
它们通常是低电平有效的信号，对目标 'x' 标记为 nCSx（例如 nCS0）。有些
设备还有其它信号，通常包含一个发往主机的中断。

与 USB 或 SMBus 之类的串行总线不同，即使是 SPI 目标功能的底层协议，
通常也在不同厂商之间互不兼容（商品化的 SPI 存储器芯片除外）。

  - SPI 可用于请求/响应式的设备协议，例如触摸屏传感器和存储器芯片。

  - 它也可以用于任意方向的数据流传输（半双工），或两个方向同时进行
    （全双工）。

  - 有些设备可能使用 8 位字。其它设备可能使用不同的字长，例如 12 位或
    20 位数字采样值的流。

  - 字通常以其最高有效位（MSB）先发送，但有时最低有效位（LSB）会
    先发送。

  - 有时 SPI 用于把设备像移位寄存器那样级联（daisy-chain）。

同样地，SPI 目标极少支持任何类型的自动发现/枚举协议。从给定 SPI 主机
控制器可访问的目标设备树通常要通过配置表手动建立。

SPI 只是此类四线协议使用的一个名称，大多数控制器处理“MicroWire”
（可视为半双工的 SPI，用于请求/响应协议）、SSP（“Synchronous Serial
Protocol”），PSP（“Programmable Serial Protocol”）以及其它相关协议都
没有问题。

有些芯片通过合并 MOSI 和 MISO、并在硬件层面将自己限制为半双工来减少
一条信号线。事实上有些 SPI 芯片就把这种信号模式作为一个 strapping 选项。
这些芯片可以使用与 SPI 相同的编程接口访问，但当然它们无法处理全双工
传输。你可能会发现这种芯片被描述为使用“三线”信号：SCK、data、nCSx。
（那条数据线有时被称为 MOMI 或 SISO。）

微控制器通常同时支持 SPI 协议的主机端和目标端。本文档（以及 Linux）同时
支持 SPI 交互的主机端与目标端。


### 谁在使用它？在哪些系统上？

使用 SPI 的 Linux 开发者大概是在为嵌入式系统板编写设备驱动。SPI 用于
控制外部芯片，它也是每张 MMC 或 SD 存储卡都支持的协议。（较早的“DataFlash”
卡早于 MMC 卡，但使用相同的连接器和卡形，只支持 SPI。）有些 PC 硬件使用
SPI flash 存放 BIOS 代码。

SPI 目标芯片种类繁多，从用于模拟传感器和编解码器的数/模转换器，到存储器，
再到像 USB 控制器或以太网适配器这样的外设，等等。

大多数使用 SPI 的系统会在主板上集成少量设备。有些通过扩展连接器提供 SPI
链路；在没有专用 SPI 控制器的情况下，可以使用 GPIO 引脚创建一个低速的
“bitbanging”适配器。极少有系统会对 SPI 控制器进行“热插拔”；使用 SPI 的
理由在于低成本和简单操作，而如果动态重配置很重要，USB 通常是更合适的
低引脚数外设总线。

许多可以运行 Linux 的微控制器都集成了一个或多个带有 SPI 模式的 I/O
接口。有了 SPI 支持，它们就可以使用 MMC 或 SD 卡，而无需专用的 MMC/SD/SDIO
控制器。


### 我有点困惑。这四种 SPI“时钟模式”是什么？

这里很容易混淆，而且你能找到的厂商文档未必有帮助。这四种模式组合了
两个模式位：

 - CPOL 表示初始时钟极性。CPOL=0 表示时钟起始为低电平，因此第一个
   （前沿）边沿是上升沿，第二个（后沿）边沿是下降沿。CPOL=1 表示时钟
   起始为高电平，因此第一个（前沿）边沿是下降沿。

 - CPHA 表示用于采样数据的时钟相位；CPHA=0 表示在前沿采样，CPHA=1
   表示在后沿采样。

   由于信号需要在采样前稳定，CPHA=0 意味着其数据在第一个时钟边沿之前
   半个时钟周期就被写入。片选可能使其变得可用。

芯片规格并不总是会用同样多的话说“使用 SPI 模式 X”，但它们的时序图会让
CPOL 和 CPHA 模式一目了然。

在 SPI 模式编号中，CPOL 是高位，CPHA 是低位。因此，当某个芯片的时序图
显示时钟起始为低电平（CPOL=0），且数据在后沿时钟边沿稳定以便采样
（CPHA=1）时，那就是 SPI 模式 1。

注意，时钟模式在片选变为有效的一刻就相关了。因此主机必须在选择目标之前
将时钟设为无效，而目标可以通过在其选通线变为有效时采样时钟电平来判断所选的
极性。这就是为什么许多设备同时支持例如模式 0 和模式 3：它们不关心极性，
并且总是在上升时钟边沿收发数据。


### 这些驱动编程接口是如何工作的？

<linux/spi/spi.h> 头文件包含 kerneldoc，主源代码也是如此，你当然应该
阅读内核 API 文档中的那一章。这里只是一个概述，以便你在了解细节之前
先建立起整体认知。

SPI 请求总是进入 I/O 队列。对给定 SPI 设备的请求总是按 FIFO 顺序执行，
并通过完成回调（completion callbacks）异步完成。也提供了一些简单的同步
封装来调用这些接口，包括用于常见事务类型的封装，例如先写一条命令再读取
其响应。

SPI 驱动有两种类型，这里称为：

  Controller drivers（控制器驱动） ...
        控制器可能内置于 System-On-Chip（片上系统）
	处理器中，并且通常同时支持 Controller 和目标角色。
	这些驱动会访问硬件寄存器并可能使用 DMA。
	或者它们可以是 PIO bitbanger，只需要 GPIO 引脚。

  Protocol drivers（协议驱动） ...
        这些驱动通过控制器驱动传递消息，
	以与位于 SPI 链路另一端的某个目标或 Controller 设备通信。

因此，例如某个协议驱动可能与 MTD 层对话，把数据导出到存储在 SPI flash
（如 DataFlash）上的文件系统；其它协议驱动可能控制音频接口、将触摸屏传感器
呈现为输入接口，或在工业处理过程中监视温度和电压水平。而这些可能都在
共享同一个控制器驱动。

“struct spi_device”封装了这两类驱动之间的控制器端接口。

SPI 编程接口有一个最小核心，侧重于使用驱动模型、借助板级特定初始化代码
提供的设备表来连接控制器驱动和协议驱动。SPI
```

   /sys/devices/.../CTLR ... physical node for a given SPI controller

   /sys/devices/.../CTLR/spiB.C ... spi_device on bus "B",
	chipselect C, accessed through CTLR.

   /sys/bus/spi/devices/spiB.C ... symlink to that physical
	.../CTLR/spiB.C device

   /sys/devices/.../CTLR/spiB.C/modalias ... identifies the driver
	that should be used with this device (for hotplug/coldplug)

   /sys/bus/spi/drivers/D ... driver for one or more spi*.* devices

   /sys/class/spi_master/spiB ... symlink to a logical node which could hold
	class related state for the SPI host controller managing bus "B".
	All spiB.* devices share one physical SPI bus segment, with SCLK,
	MOSI, and MISO.

   /sys/devices/.../CTLR/slave ... virtual file for (un)registering the
	target device for an SPI target controller.
	Writing the driver name of an SPI target handler to this file
	registers the target device; writing "(null)" unregisters the target
	device.
	Reading from this file shows the name of the target device ("(null)"
	if not registered).

   /sys/class/spi_slave/spiB ... symlink to a logical node which could hold
	class related state for the SPI target controller on bus "B".  When
	registered, a single spiB.* device is present here, possible sharing
	the physical SPI bus segment with other SPI target devices.

```
目前，唯一的类特定状态就是总线号（“spiB”中的“B”），因此那些
/sys/class 条目仅用于快速识别总线。


### 板级特定的初始化代码如何声明 SPI 设备？

Linux 需要若干类信息才能正确配置 SPI 设备。即便对于支持部分自动
发现/枚举的芯片，这些信息通常也由板级特定代码提供。

##### 声明控制器


第一类信息是一个列表，列出存在哪些 SPI 控制器。对于基于 System-on-Chip
（SOC）的板，这些通常是 platform 设备，并且控制器可能需要一些 platform_data
才能正常运作。“struct platform_device”会包含诸如控制器第一个寄存器的物理
地址及其 IRQ 等资源。

平台通常会抽象出“注册 SPI 控制器”这一操作，也许将其与初始化引脚配置的
代码耦合在一起，以便多个板的 arch/.../mach-**/board-**.c 文件都能共享
相同的基本控制器设置代码。这是因为大多数 SOC 都有多个支持 SPI 的控制器，
而通常只应设置并注册在某个给定板上真正可用的那些。
```

	#include <mach/spi.h>	/* for mysoc_spi_data */

	/* if your mach-* infrastructure doesn't support kernels that can
	 * run on multiple boards, pdata wouldn't benefit from "__init".
	 */
	static struct mysoc_spi_data pdata __initdata = { ... };

	static __init board_init(void)
	{
		...
		/* this board only uses SPI controller #2 */
		mysoc_register_spi(2, &pdata);
		...
	}

```
```

	#include <mach/spi.h>

	static struct platform_device spi2 = { ... };

	void mysoc_register_spi(unsigned n, struct mysoc_spi_data *pdata)
	{
		struct mysoc_spi_data *pdata2;

		pdata2 = kmalloc(sizeof *pdata2, GFP_KERNEL);
		*pdata2 = pdata;
		...
		if (n == 2) {
			spi2->dev.platform_data = pdata2;
			register_platform_device(&spi2);

			/* also: set up pin modes so the spi2 signals are
			 * visible on the relevant pins ... bootloaders on
			 * production boards may already have done this, but
			 * developer boards will often need Linux to do it.
			 */
		}
		...
	}

```
注意，即使使用相同的 SOC 控制器，不同板的 platform_data 也可能不同。例如，
在一块板上 SPI 可能使用外部时钟，而另一块板则从某个主时钟的当前设置推导出
SPI 时钟。

##### 声明目标设备


第二类信息是一个列表，列出目标板上存在哪些 SPI 目标设备，通常还带有驱动
正确工作所需的某些板级特定数据。

通常你的 arch/.../mach-**/board-**.c 文件会提供一个小表格，列出每块板上的
SPI 设备。（这通常只
```

	static struct ads7846_platform_data ads_info = {
		.vref_delay_usecs	= 100,
		.x_plate_ohms		= 580,
		.y_plate_ohms		= 410,
	};

	static struct spi_board_info spi_board_info[] __initdata = {
	{
		.modalias	= "ads7846",
		.platform_data	= &ads_info,
		.mode		= SPI_MODE_0,
		.irq		= GPIO_IRQ(31),
		.max_speed_hz	= 120000 /* max sample rate at 3V */ * 16,
		.bus_num	= 1,
		.chip_select	= 0,
	},
	};

```
同样，注意板级特定信息是如何提供的；每个芯片可能需要若干种类型。这个例子
展示了通用约束，例如允许的最快 SPI 时钟（在本例中是板电压的函数），或者
IRQ 引脚是如何接线的，以及芯片特定的约束，例如某个引脚的电容所导致的重要
延迟变化。

（还有“controller_data”，即对控制器驱动可能有用的信息。例如特定于外设的
DMA 调优数据或片选回调。它之后会存储在 spi_device 中。）

board_info 应提供足够的信息，以便系统在该芯片的驱动加载之前就能工作。其
中最麻烦的方面可能是 spi_device.mode 字段中的 SPI_CS_HIGH 位，因为在一个
把片选“反向”解释的设备共享同一条总线之前，基础设施无法知道如何取消选中它。

然后，你的板初始化代码会向 SPI 基础设施注册该表，这样稍后 SPI 主机控制器
```

	spi_register_board_info(spi_board_info, ARRAY_SIZE(spi_board_info));

```
像其它静态板级特定设置一样，你不会注销它们。

广泛使用的“card”式计算机会把存储器、cpu 以及少量其它元件集成到一块可能
只有三十平方厘米的卡上。在这样的系统上，你的 `arch/.../mach-.../board-*.c`
文件主要提供关于此类卡所插入的主板上的设备信息。这当然包括通过卡连接器
接上的 SPI 设备！


##### 非静态配置


当 Linux 包含通过 SPI 对 MMC/SD/SDIO/DataFlash 卡的支持时，这些配置也将是
动态的。幸运的是，这类设备都支持基本的设备识别探测，因此它们应当能正常
热插拔。


### 如何编写“SPI 协议驱动”？

目前大多数 SPI 驱动都是内核驱动，但也支持用户空间驱动。这里我们只讨论
内核驱动。
```

	static struct spi_driver CHIP_driver = {
		.driver = {
			.name		= "CHIP",
			.pm		= &CHIP_pm_ops,
		},

		.probe		= CHIP_probe,
		.remove		= CHIP_remove,
	};

```
驱动核心会自动尝试将此驱动绑定到任何 board_info 给出 modalias 为“CHIP”的
SPI 设备。除非你正在创建一个管理总线的设备（出现在 /sys/class/spi_master
下），否则你的 probe() 代码可能像这样：
```

	static int CHIP_probe(struct spi_device *spi)
	{
		struct CHIP			*chip;
		struct CHIP_platform_data	*pdata;

		/* assuming the driver requires board-specific data: */
		pdata = &spi->dev.platform_data;
		if (!pdata)
			return -ENODEV;

		/* get memory for driver's per-chip state */
		chip = kzalloc(sizeof *chip, GFP_KERNEL);
		if (!chip)
			return -ENOMEM;
		spi_set_drvdata(spi, chip);

		... etc
		return 0;
	}

```
一旦进入 probe()，驱动就可以使用“struct spi_message”向 SPI 设备发起 I/O
请求。当 remove() 返回，或 probe() 失败之后，驱动保证不会再提交任何此类
消息。

  - 一个 spi_message 是一串协议操作序列，作为一个原子序列执行。SPI 驱动
    控制包括：

      - 双向读写何时开始……由其 spi_transfer 请求序列的排列方式决定；

      - 使用哪些 I/O 缓冲区……每个 spi_transfer 为每个传输方向包装一个
        缓冲区，支持全双工（两个指针，两种情况可能相同）和半双工
        （一个指针为 NULL）传输；

      - 可选地在传输之后定义短延时……使用 spi_transfer.delay.value 设置
        （如果缓冲区长度为零，该延时可以是唯一的协议效果）……指定此延时时
        默认的 spi_transfer.delay.unit 是微秒，但如有需要可以调整为时钟
        周期或纳秒；

      - 传输之后片选是否变为无效以及是否带延时……使用 spi_transfer.cs_change
        标志；

      - 提示下一条消息是否可能发往同一设备……使用那个原子组中最后一次
        传输上的 spi_transfer.cs_change 标志，并可能节省片选取消选中和
        选中的开销。

  - 遵循标准内核规则，并在你的消息中提供 DMA 安全的缓冲区。这样使用 DMA 的
    控制器驱动就不必做额外的拷贝，除非硬件有此要求（例如绕开强制使用
    反弹缓冲（bounce buffering）的硬件 errata）。

  - 基本的 I/O 原语是 spi_async()。异步请求可以在任何上下文（irq 处理程序、
    任务等）中发起，完成通过消息附带的回调报告。在检测到任何错误之后，芯片
    被取消选中，并且该 spi_message 的处理被中止。

  - 还有像 spi_sync() 这样的同步封装，以及像 spi_read()、spi_write() 和
    spi_write_then_read() 这样的封装。这些只能在可能睡眠的上下文中发起，
    而且它们都是位于 spi_async() 之上的干净（且小巧、“可选”）层。

  - spi_write_then_read() 调用以及围绕它的便捷封装，只应在数据量较小、可以
    忽略一次额外拷贝开销的情况下使用。它旨在支持常见的 RPC 式请求，例如
    写一个 8 位命令并读一个 16 位响应——spi_w8r16() 就是其封装之一，做的
    正是这件事。

有些驱动可能需要修改 spi_device 的特性，例如传输模式、字长或时钟速率。这
通过 spi_setup() 完成，它通常应在第一次对设备做 I/O 之前从 probe() 调用。
不过，也可以在该设备没有任何消息挂起时的任何时刻调用。

虽然“spi_device”是驱动的下边界，上边界可能包括 sysfs（尤其是传感器读数）、
输入层、ALSA、网络、MTD、字符设备框架，或其它 Linux 子系统。

注意，作为与 SPI 设备交互的一部分，你的驱动必须管理两类内存。

  - I/O 缓冲区使用通常的 Linux 规则，并且必须是 DMA 安全的。你通常应从
    堆或空闲页池中分配它们。不要使用栈，或任何被声明为“static”的东西。

  - 用于将那些 I/O 缓冲区粘合为一组协议事务的 spi_message 和 spi_transfer
    元数据。这些可以在方便的任何地方分配，包括作为其它一次性分配的驱动
    数据结构的一部分。将这些清零初始化。

如果你愿意，可以使用 spi_message_alloc() 和 spi_message_free() 便捷例程来
分配并零初始化一个带有多个传输的 spi_message。


### 如何编写“SPI 控制器驱动”？

一个 SPI 控制器大概会注册在 platform_bus 上；编写一个驱动来绑定到该设备，
无论涉及的是哪条总线。

这类驱动的主要任务是提供一个“spi_controller”。使用 spi_alloc_host() 分配
主机控制器，使用 spi_controller_get_devdata() 获取为该设备分配的驱动私有
数据。
```

	struct spi_controller	*ctlr;
	struct CONTROLLER	*c;

	ctlr = spi_alloc_host(dev, sizeof *c);
	if (!ctlr)
		return -ENODEV;

	c = spi_controller_get_devdata(ctlr);

```
驱动将初始化该 spi_controller 的字段，包括总线号（也许与 platform 设备 ID
相同）以及与 SPI 核心和 SPI 协议驱动交互的三个方法。它还会初始化自己的内部
状态。（关于总线编号和那些方法，见下文。）

在你初始化 spi_controller 之后，使用 spi_register_controller() 将其发布到
系统的其余部分。此时，控制器和任何预先声明的 spi 设备的设备节点都将可用，
驱动模型核心会负责将它们绑定到驱动。

如果你需要移除你的 SPI 控制器驱动，spi_unregister_controller() 将逆转
spi_register_controller() 的效果。


##### 总线编号


总线编号很重要，因为 Linux 正是用它来识别给定 SPI 总线（共享 SCK、MOSI、
MISO）的。有效的总线号从零开始。在 SOC 系统上，总线号应与芯片制造商定义的
编号相匹配。例如，硬件控制器 SPI2 将是总线号 2，连接到它的设备的
spi_board_info 将使用该编号。

如果你没有这样的硬件分配的总线号，又因为某种原因无法自行分配，那么提供一个
负的总线号。随后它将被一个动态分配的编号替换。这时你需要将其视为非静态
配置（见上文）。


##### SPI 主机控制器方法


`ctlr->setup(struct spi_device *spi)`
	这会设置设备时钟速率、SPI 模式和字长。驱动可以修改 board_info 提供的
	默认值，然后调用 spi_setup(spi) 来调用此例程。它可能会睡眠。

	除非每个 SPI 目标都有自己的配置寄存器，否则不要立即修改它们……否则
	驱动可能会破坏正在为其它 SPI 设备进行的 I/O。
```

		BUG ALERT:  for some reason the first version of
		many spi_controller drivers seems to get this wrong.
		When you code setup(), ASSUME that the controller
		is actively processing transfers for another device.

```
`ctlr->cleanup(struct spi_device *spi)`
	你的控制器驱动可以使用 spi_device.controller_state 来保存它动态关联到
	该设备的状态。如果你这样做，务必提供 cleanup() 方法来释放该状态。

`ctlr->prepare_transfer_hardware(struct spi_controller *ctlr)`
	队列机制会调用它，向驱动发出信号表示一条消息即将到来，于是子系统请求
	驱动通过发起此调用来准备传输硬件。它可能会睡眠。

`ctlr->unprepare_transfer_hardware(struct spi_controller *ctlr)`
	队列机制会调用它，向驱动发出信号表示队列中没有更多挂起的消息，它可以
	放松硬件（例如通过电源管理调用）。它可能会睡眠。

`ctlr->transfer_one_message(struct spi_controller **ctlr, struct spi_message **mesg)`
	子系统调用驱动来传输单条消息，同时把期间到达的传输排入队列。当驱动
	完成此消息时，它必须调用 spi_finalize_current_message()，以便子系统
	可以发出下一条消息。它可能会睡眠。

`ctrl->transfer_one(struct spi_controller **ctlr, struct spi_device **spi, struct spi_transfer *transfer)`
	子系统调用驱动来传输单个传输，同时把期间到达的传输排入队列。当驱动
	完成此传输时，它必须调用 spi_finalize_current_transfer()，以便子系统
	可以发出下一个传输。它可能会睡眠。注意：transfer_one 和 transfer_one_message
	是互斥的；当两者都被设置时，通用子系统不会调用你的 transfer_one 回调。

	返回值：

 - 负的 errno：错误
 - 0：传输已完成
 - 1：传输仍在进行中

`ctrl->set_cs_timing(struct spi_device *spi, u8 setup_clk_cycles, u8 hold_clk_cycles, u8 inactive_clk_cycles)`
	此方法允许 SPI 客户端驱动请求 SPI 主机控制器配置设备特定的 CS 建立、
	保持和无效时序要求。

##### 已弃用的方法


`ctrl->transfer(struct spi_device **spi, struct spi_message **message)`
	这绝不能睡眠。它的职责是安排传输发生，并发出其 complete() 回调。这两件事
	通常稍后发生，在其它传输完成之后，而如果控制器空闲，则需要被 kickstart。
	此方法不用于排队式控制器，并且在实现了 transfer_one_message() 和
	(un)prepare_transfer_hardware() 时必须为空。


##### SPI 消息队列


如果你对 SPI 子系统提供的标准排队机制感到满意，只需实现上面指定的排队方法
即可。使用消息队列的好处是可以集中大量代码，并提供方法的纯进程上下文执行。
在高优先级 SPI 流量下，消息队列也可以提升到实时优先级。

除非选择了 SPI 子系统内的排队机制，否则驱动的大部分工作将是管理由现已弃用
的 transfer() 函数所喂入的 I/O 队列。

那个队列可以是纯概念上的。例如，一个仅用于低频传感器访问的驱动，使用同步
PIO 可能就足够了。

但那个队列很可能是非常真实的，使用 message->queue、PIO，经常 DMA（特别是
如果根文件系统位于 SPI flash 中），以及像 IRQ 处理程序、tasklet 或工作队列
（如 keventd）这样的执行上下文。你的驱动可以如你需要那般花哨，或那般简单。
这样的 transfer() 方法通常只是把消息加入队列，然后启动某个异步传输引擎
（除非它已经在运行）。


### SPI 协议的扩展

SPI 没有正式的规范或标准，这一事实使得芯片制造商可以以略有不同的方式实现
SPI 协议。在大多数情况下，来自不同厂商的 SPI 协议实现彼此兼容。例如，在
SPI 模式 0（CPOL=0，CPHA=0）下，总线信号线可能表现如下：
```

  nCSx ___                                                                   ___
          \_________________________________________________________________/
          •                                                                 •
          •                                                                 •
  SCLK         ___     ___     ___     ___     ___     ___     ___     ___
       _______/   \___/   \___/   \___/   \___/   \___/   \___/   \___/   \_____
          •   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ; •
          •   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ; •
  MOSI XXX__________         _______                 _______         ________XXX
  0xA5 XXX__/ 1     \_0_____/ 1     \_0_______0_____/ 1     \_0_____/ 1    \_XXX
          •       ;       ;       ;       ;       ;       ;       ;       ; •
          •       ;       ;       ;       ;       ;       ;       ;       ; •
  MISO XXX__________         _______________________          _______        XXX
  0xBA XXX__/     1 \_____0_/     1       1       1 \_____0__/    1  \____0__XXX

```
```

  • marks the start/end of transmission;
  : marks when data is clocked into the peripheral;
  ; marks when data is clocked into the controller;
  X marks when line states are not specified.

```
在少数情况下，芯片通过指定其它 SPI 协议不使用的信号线行为（例如 CS 未断言时
的数据线状态）来扩展 SPI 协议。那些不同的 SPI 协议、模式和配置由不同的 SPI
模式标志支持。

##### MOSI 空闲状态配置


常见的 SPI 协议实现没有为控制器未时钟输出数据时的 MOSI 线指定任何状态或
行为。然而，确实存在一些外设，要求在未时钟输出数据时 MOSI 线处于特定状态。
例如，如果外设期望在控制器未时钟输出数据时 MOSI 线为高电平（`SPI_MOSI_IDLE_HIGH`），
那么 SPI 模式 0 下的传输看起来如下：
```

  nCSx ___                                                                   ___
          \_________________________________________________________________/
          •                                                                 •
          •                                                                 •
  SCLK         ___     ___     ___     ___     ___     ___     ___     ___
       _______/   \___/   \___/   \___/   \___/   \___/   \___/   \___/   \_____
          •   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ; •
          •   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ;   :   ; •
  MOSI _____         _______         _______         _______________         ___
  0x56      \_0_____/ 1     \_0_____/ 1     \_0_____/ 1       1     \_0_____/
          •       ;       ;       ;       ;       ;       ;       ;       ; •
          •       ;       ;       ;       ;       ;       ;       ;       ; •
  MISO XXX__________         _______________________          _______        XXX
  0xBA XXX__/     1 \_____0_/     1       1       1 \_____0__/    1  \____0__XXX

```
```

  • marks the start/end of transmission;
  : marks when data is clocked into the peripheral;
  ; marks when data is clocked into the controller;
  X marks when line states are not specified.

```
在对通常 SPI 协议的这个扩展中，MOSI 线状态被指定为：在 CS 断言但控制器未
时钟输出数据给外设时，以及在 CS 未断言时，都保持为高电平。

需要此扩展的外设必须通过在其 ``struct spi_device`` 的 mode 属性中设置
`SPI_MOSI_IDLE_HIGH` 位并调用 spi_setup() 来请求它。支持此扩展的控制器应
通过在其 `struct spi_controller` 的 mode_bits 属性中设置 `SPI_MOSI_IDLE_HIGH`
来表明这一点。将 MOSI 空闲为低电平的配置与之类似，但使用 `SPI_MOSI_IDLE_LOW`
模式位。


### 致谢

对 Linux-SPI 讨论做出贡献的人包括（按姓氏字母顺序）：

- Mark Brown
- David Brownell
- Russell King
- Grant Likely
- Dmitry Pervushin
- Stephen Street
- Mark Underwood
- Andrew Victor
- Linus Walleij
- Vitaly Wool
