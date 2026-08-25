## 编写 MUSB Glue Layer（粘合层

:Author: Apelete Seketeli

## 简

Linux MUSB 子系统是更大Linux USB 子系统的一部分。它为那些不
使用 Universal Host Controller Interface (UHCI) Open Host
Controller Interface (OHCI) 的嵌入式 USB Device Controller (UDC)
提供支持

相反，这些嵌入式 UDC 依赖USB On-the-Go (OTG) 规范，并且至
部分地实现了该规范。大多数情况下使用的硅片参考设计是 Mentor
Graphics Inventra设计中的 Multipoint USB Highspeed
Dual-Role Controller (MUSB HDRC)銆。

作为一次自学练习，我为 Ingenic JZ4740 SoC 编写了一MUSB glue
layer，其模型参照了内核源码树中的众多 MUSB glue layer。该层位
`drivers/usb/musb/jz4740.c`。在本文档中，我将逐步讲解 `jz4740.c`
这个 glue layer 的基础知识，解释其中各个组成部分，以及编写一
属于你自己的设备 glue layer 需要做些什么

## Linux MUSB 基础

要开始了解这个主题，请阅USB On-the-Go Basics（参
Resources），它从硬件层面介绍USB OTG 的操作。Texas Instruments
Analog Devices 的几wiki 页面也概述了 Linux 内核 MUSB 的配置，
尽管它们侧重于这两家公司提供的某些特定设备。最后，通过 USB home
page 来熟USB 规范可能会有所帮助，并且可以通过 Writing USB
Device Drivers 文档（同样参Resources）获得实用的实例

Linux USB 协议栈是一个分层架构，MUSB 控制器硬件位于最底层。MUSB
控制器驱动对以下部分进行了抽象：

```
	  ------------------------
	  |                      | <------- drivers/usb/gadget
	  | Linux USB Core Stack | <------- drivers/usb/host
	  |                      | <------- drivers/usb/core
	  ------------------------
		     猬?
	 --------------------------
	 |                        | <------ drivers/usb/musb/musb_gadget.c
	 | MUSB Controller driver | <------ drivers/usb/musb/musb_host.c
	 |                        | <------ drivers/usb/musb/musb_core.c
	 --------------------------
		     猬?
      ---------------------------------
      | MUSB Platform Specific Driver |
      |                               | <-- drivers/usb/musb/jz4740.c
      |       aka "Glue Layer"        |
      ---------------------------------
		     猬?
      ---------------------------------
      |   MUSB Controller Hardware    |
      ---------------------------------
```

如上所述，glue layer 实际上是位于控制器驱动与控制器硬件之间的
平台相关代码

就像 Linux USB 驱动需要向 Linux USB 子系统注册自己一样，MUSB glue
layer 需要先MUSB 控制器驱动注册自己。这样控制器驱动就能知道
glue layer 支持哪些设备，以及在检测到或释放受支持的设备时
调用哪些函数；请记住，我们这里讨论的是一个嵌入式控制器芯片，
因此不存在运行时的插入或移除

所有这些相关信息都通过以下方式传递给 MUSB 控制器驱动：

```
    static struct platform_driver jz4740_driver = {
	.probe      = jz4740_probe,
	.remove     = jz4740_remove,
	.driver     = {
	    .name   = "musb-jz4740",
	},
    };
```

probe remove 函数指针分别在检测到匹配的设备和（相应地）释
设备时被调用。name 字符串描述了glue layer 所支持的设备。在当前
情况下，它与 `arch/mips/jz4740/platform.c` 中声明的 platform_device
结构相匹配。注意，我们这里没有使用 device tree bindings

为了向控制器驱动完成注册，glue layer 要经历几个步骤，基本上是
分配控制器硬件资源并初始化若干模块。为此，它需要跟踪这些步骤中
所使用的相关信息。这是通过以下结构完成的：

```
    struct jz4740_glue {
	struct device           *dev;
	struct platform_device  *musb;
	struct clk      *clk;
    };
```

dev musb 成员都是 device 结构变量。第一个成员保存关于该设备
通用信息，因为它是最基础的设备结构；而后者保存与设备所注册到的
子系统更密切相关的信息。clk 变量保存与设备时钟操作相关的信息

让我们来看看 probe 函数中那些使 glue layer 向控制器驱动完成注册
的各个步骤

   出于可读性考虑，每个函数将被拆分成逻辑上的若干部分，每一部分
   都像彼此独立一样展示

    :emphasize-lines: 8,12,18

    static int jz4740_probe(struct platform_device *pdev)
    {
	struct platform_device      *musb;
	struct jz4740_glue      *glue;
	struct clk                      *clk;
	int             ret;

	glue = devm_kzalloc(&pdev->dev, sizeof(*glue), GFP_KERNEL);
	if (!glue)
	    return -ENOMEM;

	musb = platform_device_alloc("musb-hdrc", PLATFORM_DEVID_AUTO);
	if (!musb) {
	    dev_err(&pdev->dev, "failed to allocate musb device\n");
	    return -ENOMEM;
	}

	clk = devm_clk_get(&pdev->dev, "udc");
	if (IS_ERR(clk)) {
	    dev_err(&pdev->dev, "failed to get clock\n");
	    ret = PTR_ERR(clk);
	    goto err_platform_device_put;
	}

	ret = clk_prepare_enable(clk);
	if (ret) {
	    dev_err(&pdev->dev, "failed to enable clock\n");
	    goto err_platform_device_put;
	}

	musb->dev.parent        = &pdev->dev;

	glue->dev           = &pdev->dev;
	glue->musb          = musb;
	glue->clk           = clk;

	return 0;

    err_platform_device_put:
	platform_device_put(musb);
	return ret;
    }

probe 函数的前几行分配并赋glue、musb clk 变量。`GFP_KERNEL`
标志（第 8 行）允许分配过程睡眠并等待内存，因此可用于加锁的
情形。`PLATFORM_DEVID_AUTO` 标志（第 12 行）允许自动分配和管
设备 ID，以避免与显ID 产生设备命名空间冲突。通过
`devm_clk_get`（第 18 行），glue layer 分配时钟——`devm_` 前缀表示
`clk_get` 是受管理的：当设备被释放时它会自动释放所分配的时
资源数据——并启用它

接下来是注册步骤

    :emphasize-lines: 3,5,7,9,16

    static int jz4740_probe(struct platform_device *pdev)
    {
	struct musb_hdrc_platform_data  *pdata = &jz4740_musb_platform_data;

	pdata->platform_ops     = &jz4740_musb_ops;

	platform_set_drvdata(pdev, glue);

	ret = platform_device_add_resources(musb, pdev->resource,
			    pdev->num_resources);
	if (ret) {
	    dev_err(&pdev->dev, "failed to add resources\n");
	    goto err_clk_disable;
	}

	ret = platform_device_add_data(musb, pdata, sizeof(*pdata));
	if (ret) {
	    dev_err(&pdev->dev, "failed to add platform_data\n");
	    goto err_clk_disable;
	}

	return 0;

    err_clk_disable:
	clk_disable_unprepare(clk);
    err_platform_device_put:
	platform_device_put(musb);
	return ret;
    }

第一步是通过 `platform_set_drvdata`（第 7 行）glue layer 私有
持有的设备数据传递给控制器驱动。接下来是通过
`platform_device_add_resources`（第 9 行）传递设备资源信息，此时
这些信息同样为私有持有

最后是向控制器驱动传递平台相关数据（16 行）。Platform data
将在 musb-dev-platform-data 中讨论，但这里我们要看的
`musb_hdrc_platform_data` 结构（第 3 行）中的 `platform_ops` 函数
指针（第 5 行）。这个函数指针允MUSB 控制器驱动在需要时调用
以下函数

```
    static const struct musb_platform_ops jz4740_musb_ops = {
	.init       = jz4740_musb_init,
	.exit       = jz4740_musb_exit,
    };
```

这里是最精简的情况，控制器驱动仅在需要时调用 init exit 函数
事实JZ4740 MUSB 控制器是一个基础型控制器，缺少其他控制器
具备的一些特性，否则我们可能还需要指向其他一些函数的指针，例
电源管理函数，或OTG 与非 OTG 模式之间切换的函数等等

在注册的那个时刻，控制器驱动会实际调init 函数

   .. code-block:: c
    :emphasize-lines: 12,14

    static int jz4740_musb_init(struct musb *musb)
    {
	musb->xceiv = usb_get_phy(USB_PHY_TYPE_USB2);
	if (!musb->xceiv) {
	    pr_err("HS UDC: no transceiver configured\n");
	    return -ENODEV;
	}

	/* Silicon does not implement ConfigData register.
  - Set dyn_fifo to avoid reading EP config from hardware.
	 */
	musb->dyn_fifo = true;

	musb->isr = jz4740_musb_interrupt;

	return 0;
    }

`jz4740_musb_init()` 的目标是获取 MUSB 控制器硬件的 transceiver
驱动数据，并像往常一样将其传递给 MUSB 控制器驱动。transceiver 
控制器硬件内部负责发接收 USB 数据的电路。由于它OSI 模型
物理层的实现，transceiver 也常被称PHY

获取 `MUSB PHY` 驱动数据是通过 `usb_get_phy()` 完成的，它返回指
包含驱动实例数据的结构的指针。接下来的几条指令（12 行和14 行）
分别用作一quirk 以及用于设置 IRQ 处理。Quirks IRQ 处理
musb-dev-quirks 中稍后讨论

```
    static int jz4740_musb_exit(struct musb *musb)
    {
	usb_put_phy(musb->xceiv);

	return 0;
    }
```

作为 init 的对应部分，exit 函数在控制器硬件本身即将被释放时
释放 MUSB PHY 驱动

再次注意，由JZ4740 控制器硬件的特性集较为基础，init exit
在此处相当简单。在为更复杂的控制器硬件编写 musb glue layer 时，
你可能需要在这两个函数中处理更多事务

init 函数返回后，MUSB 控制器驱动跳回到

```
    static int jz4740_probe(struct platform_device *pdev)
    {
	ret = platform_device_add(musb);
	if (ret) {
	    dev_err(&pdev->dev, "failed to register musb device\n");
	    goto err_clk_disable;
	}

	return 0;

    err_clk_disable:
	clk_disable_unprepare(clk);
    err_platform_device_put:
	platform_device_put(musb);
	return ret;
    }
```

这是设备注册过程的最后一部分，glue layer 将控制器硬件设备添加
Linux 内核设备层级结构中：在此阶段，所有已知的关于该设备的信息
都被传递给 Linux USB core 协议栈：

   .. code-block:: c
    :emphasize-lines: 5,6

    static int jz4740_remove(struct platform_device *pdev)
    {
	struct jz4740_glue  *glue = platform_get_drvdata(pdev);

	platform_device_unregister(glue->musb);
	clk_disable_unprepare(glue->clk);

	return 0;
    }

作为 probe 的对应部分，remove 函数注销 MUSB 控制器硬件（5 行）
并禁用时钟（6 行），使其可以被门控关闭

## 处理 IRQ

除了 MUSB 控制器硬件的基本设置和注册之外，glue layer 还负责处
IRQ锛。

   .. code-block:: c
    :emphasize-lines: 7,9-11,14,24

    static irqreturn_t jz4740_musb_interrupt(int irq, void *__hci)
    {
	unsigned long   flags;
	irqreturn_t     retval = IRQ_NONE;
	struct musb     *musb = __hci;

	spin_lock_irqsave(&musb->lock, flags);

	musb->int_usb = musb_readb(musb->mregs, MUSB_INTRUSB);
	musb->int_tx = musb_readw(musb->mregs, MUSB_INTRTX);
	musb->int_rx = musb_readw(musb->mregs, MUSB_INTRRX);

	/*
  - The controller is gadget only, the state of the host mode IRQ bits is
  - undefined. Mask them to make sure that the musb driver core will
  - never see them set
	 */
	musb->int_usb &= MUSB_INTR_SUSPEND | MUSB_INTR_RESUME |
	    MUSB_INTR_RESET | MUSB_INTR_SOF;

	if (musb->int_usb || musb->int_tx || musb->int_rx)
	    retval = musb_interrupt(musb);

	spin_unlock_irqrestore(&musb->lock, flags);

	return retval;
    }

这里 glue layer 主要需要读取相关的硬件寄存器，并将其值传递给
控制器驱动，由控制器驱动来处理实际触发该 IRQ 的事件

中断处理程序的关键区段由 `spin_lock_irqsave` 及其对应函数
`spin_unlock_irqrestore`（分别是7 行和24 行）保护，它
防止中断处理程序代码被两个不同的线程同时运行

随后读取相关的中断寄存器（第 9 11 行）

- `MUSB_INTRUSB`：指示当前哪USB 中断处于激活状态，

- `MUSB_INTRTX`：指示当TX 端点中哪些中断处于激活状态，

- `MUSB_INTRRX`：指示当TX 端点中哪些中断处于激活状态

注意，`musb_readb` 最多用于读8 位寄存器，`musb_readw` 允许
我们读取最16 位的寄存器。根据设备寄存器大小的不同，还可以使
其他函数。更多信息请参见 `musb_io.h`

18 行的指令JZ4740 USB 设备控制器特有的另一quirk，将
musb-dev-quirks 中稍后讨论

不过，glue layer 仍然需要注册该 IRQ 处理程序。还记得

```
    static int jz4740_musb_init(struct musb *musb)
    {
	musb->isr = jz4740_musb_interrupt;

	return 0;
    }
```

该指令设置了一个指glue layer IRQ 处理函数的指针，以便
控制器硬件产IRQ 时控制器硬件能够回调该处理程序。中断处理程
现已实现并注册完成

## 设备 Platform Data

为了编写一MUSB glue layer，你需要有一些描述控制器硬件能力
数据，这被称platform data

Platform data 是特定于你的硬件的，尽管它可能会覆盖一大类设备
并且通常位于 `arch/` 目录中的某个位置，具体取决于你的设备架构

例如，JZ4740 SoC platform data 位于 `arch/mips/jz4740/platform.c`
`platform.c` 文件中，JZ4740 SoC 的每个设备都通过一组结构来描述

以下`arch/mips/jz4740/platform.c` 中覆USB Device Controller (UDC)
的部分：

   .. code-block:: c
    :emphasize-lines: 2,7,14-17,21,22,25,26,28,29

    /** USB Device Controller **/
    struct platform_device jz4740_udc_xceiv_device = {
	.name = "usb_phy_gen_xceiv",
	.id   = 0,
    };

    static struct resource jz4740_udc_resources[] = {
	[^0^] = {
	    .start = JZ4740_UDC_BASE_ADDR,
	    .end   = JZ4740_UDC_BASE_ADDR + 0x10000 - 1,
	    .flags = IORESOURCE_MEM,
	},
	[^1^] = {
	    .start = JZ4740_IRQ_UDC,
	    .end   = JZ4740_IRQ_UDC,
	    .flags = IORESOURCE_IRQ,
	    .name  = "mc",
	},
    };

    struct platform_device jz4740_udc_device = {
	.name = "musb-jz4740",
	.id   = -1,
	.dev  = {
	    .dma_mask          = &jz4740_udc_device.dev.coherent_dma_mask,
	    .coherent_dma_mask = DMA_BIT_MASK(32),
	},
	.num_resources = ARRAY_SIZE(jz4740_udc_resources),
	.resource      = jz4740_udc_resources,
    };

`jz4740_udc_xceiv_device` platform device 结构（第 2 行）通过名称
id 号描述了 UDC transceiver

在撰写本文时，请注意 `usb_phy_gen_xceiv` 是用于所有内置在参USB
IP 中、或自主且不需要任PHY 编程transceiver 的专用名称。你需
在内核配置中设置 `CONFIG_NOP_USB_XCEIV=y` 才能使用相应transceiver
驱动。id 字段可以设置-1（相当于 `PLATFORM_DEVID_NONE`）2（相当于
`PLATFORM_DEVID_AUTO`），或者如果要指定 id 号，则从 0 开始作为此
设备的第一个设备

`jz4740_udc_resources` 资源结构（第 7 行）定义UDC 寄存器基地址

第一个数组（9 11 行）定义UDC 寄存器基地址内存地址：start
指向第一个寄存器内存地址，end 指向最后一个寄存器内存地址，flags
成员定义了我们所处理资源的类型。因`IORESOURCE_MEM` 用于定义
寄存器内存地址。第二个数组（第 14 17 行）定义UDC IRQ 寄存
地址。由JZ4740 UDC 只有一个可用的 IRQ 寄存器，start end 指向
相同的地址。`IORESOURCE_IRQ` 标志表明我们处理的是 IRQ 资源，而名
`mc` 实际上是硬编码在 MUSB core 中的，以便控制器驱动能够通过按名
查询来获取这IRQ 资源

最后，`jz4740_udc_device` platform device 结构（第 21 行）描述
UDC 本身

`musb-jz4740` 名称（第 22 行）定义了用于该设备MUSB 驱动；请记住
这实际上正是我们musb-basics `jz4740_driver` platform driver
结构里使用的名称。id 字段（第 23 行）设为 -1（相当于
`PLATFORM_DEVID_NONE`），因为我们不需要为设备指定 id：MUSB 控制
驱动已经musb-basics 中设为分配自id 了。在 dev 字段中，我们
在此关注 DMA 相关信息。`dma_mask` 字段（第 25 行）定义了将要使用的
DMA 掩码宽度，`coherent_dma_mask`（第 26 行）用途相同，但针
`alloc_coherent` DMA 映射：在这两种情况下我们都使用一32 位的掩码
然后 resource 字段（第 29 行）只是一个指向之前定义的资源结构的指针，
`num_resources` 字段（第 28 行）记录了资源结构中定义的数组数
（本例中有两个资源数组被定义）

`arch/` 层面UDC platform data 的简要概览到此结束，让我们回
`drivers/usb/musb/jz4740.c` MUSB glue layer 特定platform data

   .. code-block:: c
    :emphasize-lines: 3,5,7-9,11

    static struct musb_hdrc_config jz4740_musb_config = {
	/** Silicon does not implement USB OTG. **/
	.multipoint = 0,
	/** Max EPs scanned, driver will decide which EP can be used. **/
	.num_eps    = 4,
	/** RAMbits needed to configure EPs from table **/
	.ram_bits   = 9,
	.fifo_cfg = jz4740_musb_fifo_cfg,
	.fifo_cfg_size = ARRAY_SIZE(jz4740_musb_fifo_cfg),
    };

    static struct musb_hdrc_platform_data jz4740_musb_platform_data = {
	.mode   = MUSB_PERIPHERAL,
	.config = &jz4740_musb_config,
    };

首先，glue layer 配置控制器驱动操作中与控制器硬件特定相关的一
方面。这是通过 `jz4740_musb_config` `musb_hdrc_config` 结构完成的

定义控制器硬件的 OTG 能力时，multipoint 成员（第 3 行）设为 0
（相当于 false），因为 JZ4740 UDC 不兼OTG。接着 `num_eps`（第 5 行）
定义了控制器硬件USB 端点数量，包括端0：这里我们有 3 个端点加
端点 0。接下来`ram_bits`（第 7 行），它MUSB 控制器硬件的 RAM
地址总线宽度。当控制器驱动无法通过读取相关控制器硬件寄存器来自
配置端点时，就需要这个信息。这个问题将在我们在 musb-dev-quirks 
讨论设备 quirks 时提及。最后两个字段（8 行和9 行）也关乎设
quirks：`fifo_cfg` 指向 USB 端点配置表，`fifo_cfg_size` 记录该配置表
中的条目数量。更多内容将musb-dev-quirks 中介绍

随后该配置被嵌入`jz4740_musb_platform_data` `musb_hdrc_platform_data`
结构（第 11 行）中：config 是指向配置结构本身的指针，mode 告诉
控制器驱动该控制器硬件是仅可用作 `MUSB_HOST`、仅可用
`MUSB_PERIPHERAL`，还是可用作双模式的 `MUSB_OTG`

请记住，`jz4740_musb_platform_data` 随后被用来传platform data 信息
正如我们musb-basics probe 函数中所看到的那样

## 设备 Quirks

在完善特定于你设备的 platform data 时，你可能还需要在 glue layer 
编写一些代码，以规避某些设备特定的限制。这quirks 可能是由某些
硬件缺陷引起的，或者仅仅是 USB On-the-Go 规范实现不完整的后果

JZ4740 UDC 就表现出这样quirks，其中一些我们将在此讨论，以增进
了解，尽管这些可能在你正在使用的控制器硬件中并不存在

让我们先回到 init 函数

   .. code-block:: c
    :emphasize-lines: 12

    static int jz4740_musb_init(struct musb *musb)
    {
	musb->xceiv = usb_get_phy(USB_PHY_TYPE_USB2);
	if (!musb->xceiv) {
	    pr_err("HS UDC: no transceiver configured\n");
	    return -ENODEV;
	}

	/* Silicon does not implement ConfigData register.
  - Set dyn_fifo to avoid reading EP config from hardware.
	 */
	musb->dyn_fifo = true;

	musb->isr = jz4740_musb_interrupt;

	return 0;
    }

12 行的指令帮助 MUSB 控制器驱动规避了这样一个事实：控制器硬
缺少用于 USB 端点配置的寄存器

如果没有这些寄存器，控制器驱动就无法从硬件读取端点配置，因此我们
使用12 行的指令绕过从硅片读取配置，转而依赖一个配置表

```
    static const struct musb_fifo_cfg jz4740_musb_fifo_cfg[] = {
	{ .hw_ep_num = 1, .style = FIFO_TX, .maxpacket = 512, },
	{ .hw_ep_num = 1, .style = FIFO_RX, .maxpacket = 512, },
	{ .hw_ep_num = 2, .style = FIFO_TX, .maxpacket = 64, },
    };
```

查看上面的配置表，我们看到每个端点由三个字段描述：`hw_ep_num` 
端点号，style 是其方向（要么是 `FIFO_TX`，表示由控制器驱动向控制
硬件发送数据包；要么是 `FIFO_RX`，表示从硬件接收数据包），
maxpacket 定义了该端点上可传输的每个数据包的最大尺寸。从表中读取
可知，端1 可用于一次性发送和接收 512 字节USB 数据包（这实际上
是一个批in/out 端点），端点 2 可用于一次性发64 字节的数据包
（这实际上是一个中断端点）

注意，这里没有关于端0 的信息：端点 0 在每个硅片设计中都是默认
实现的，并按USB 规范具有预定义的配置。更多端点配置表的示例请
参见 `musb_core.c`

现在让我们回到中断处理函数：

   .. code-block:: c
    :emphasize-lines: 18-19

    static irqreturn_t jz4740_musb_interrupt(int irq, void *__hci)
    {
	unsigned long   flags;
	irqreturn_t     retval = IRQ_NONE;
	struct musb     *musb = __hci;

	spin_lock_irqsave(&musb->lock, flags);

	musb->int_usb = musb_readb(musb->mregs, MUSB_INTRUSB);
	musb->int_tx = musb_readw(musb->mregs, MUSB_INTRTX);
	musb->int_rx = musb_readw(musb->mregs, MUSB_INTRRX);

	/*
  - The controller is gadget only, the state of the host mode IRQ bits is
  - undefined. Mask them to make sure that the musb driver core will
  - never see them set
	 */
	musb->int_usb &= MUSB_INTR_SUSPEND | MUSB_INTR_RESUME |
	    MUSB_INTR_RESET | MUSB_INTR_SOF;

	if (musb->int_usb || musb->int_tx || musb->int_rx)
	    retval = musb_interrupt(musb);

	spin_unlock_irqrestore(&musb->lock, flags);

	return retval;
    }

上面18 行的指令是控制器驱动规避这样一个事实的一种方式：用于 USB
主机模式操作的某些中断位`MUSB_INTRUSB` 寄存器中缺失，因此处
未定义的硬件状态，因为MUSB 控制器硬件仅用于外设模式。因此，glue
layer 通过对从 `MUSB_INTRUSB` 读取的值与寄存器中实际实现的位进行
逻辑 AND 操作，将这些缺失的位屏蔽掉，以避免产生寄生中断

这些只是 JZ4740 USB 设备控制器中发现的少数几quirks。其他一
则直接在 MUSB core 中得到处理，因为那些修复足够通用，能够为最
其他控制器硬件更好地处理问题

## 结语

编写 Linux MUSB glue layer 应当是一项更易上手的任务是，因为本文
试图展示这项练习的来龙去脉

JZ4740 USB 设备控制器相当简单，我希望它glue layer 能作为一
好例子供好奇者参考。结合当前的 MUSB glue layer 一起使用，本文
应当能提供足够的入门指导；万一事情失控，linux-usb 邮件列表归档
是另一个可供查阅的有用资源

## 致谢

非常感谢 Lars-Peter Clausen Maarten ter Huurne，他们在撰写
JZ4740 glue layer 期间回答了我的问题，并帮助我将代码整理得井井有条

我还要感谢整Qi-Hardware 社区所给予的愉快指导与支持

## 资源

USB Home Page: https://www.usb.org

linux-usb Mailing List Archives: https://lore.kernel.org/linux-usb

USB On-the-Go Basics:
https://www.maximintegrated.com/app-notes/index.mvp/id/1822

Writing USB Device Drivers <writing-usb-driver>

Texas Instruments USB Configuration Wiki Page:
https://web.archive.org/web/20201215135015/http://processors.wiki.ti.com/index.php/Usbgeneralpage
