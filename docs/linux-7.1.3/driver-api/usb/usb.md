
## Linux-USB 主机API


## Linux 上的 USB 简

通用串行总线（USB）用于将主机（例PC 或工作站）连接到若干外围设备。USB
使用树形结构，以主机为根（系统的主设备），以集线器（hub）为内部节点，以
外围设备为叶子（从设备）。现PC 支持若干这样USB 设备树，通常有几USB 3.0 GBit/s）或 USB 3.10 GBit/s）总线，以及一些遗留的 USB 2.0
80 MBit/s）总线以防万一
这种从非对称性是出于若干原因而设计的，其中一个是易用性。在物理上不可能
弄错上游和下游，或者（type C 插头的情况下）这无关紧要（或者它们内置于
外围设备中）。此外，主机软件不需要处理分布式自动配置，因为预先指定的主节管理着这一切
内核开发者在 2.2 内核系列早期就为 Linux 添加USB 支持，并自此不断开发它除了对每一代新 USB 的支持外，各种主机控制器也获得了支持，新增了用于外围
设备的驱动，并引入了用于延迟度量的高级特性和改进的电源管理
Linux 既可以在 USB 设备内部运行，也可以在控制这些设备的主机上运行。但是在
那些外围设备内部运行USB 设备驱动做的事情与在主机内部运行的那些不同，因此
它们被赋予了一个不同的名称*gadget 驱动**（小工具驱动）。本文档不涵gadget 驱动
## USB 主机API 模型


用于 USB 设备的主机端驱动"usbcore" API 通信。有两个。一个是面向
**通用** 驱动（通过驱动框架暴露）的，另一个是面向作为 *核心一部分* 驱动的。此类核心驱动包**hub** 驱动（管USB 设备的树）和几种不同*主机控制器驱，它们控制各自的总线
USB 驱动所看到的设备模型相对复杂
- USB 支持四种数据传输（控制、批量、中断和等时）。其中两种（控制和批量）
  在带宽可用时使用带宽，而另外两种（中断和等时）被调度以提供有保证的带宽
- 设备描述模型包括每个设备一个或多个“配置”（configuration），一次只  其中一个处于活动状态。设备应该能够以低于其最高速度的速度运行，并可以
  提供一BOS 描述符来显示它们仍完全可运行的最低速度
- USB 3.0 起，配置具有一个或多个“功能”（function），它们提供通用功能
  并为了电源管理的目的被组合在一起
- 配置或功能具有一个或多个“接口”（interface），每个接口可能具有“备  设置”（alternate setting）。接口可能由 USB “类”（Class）规范标准化  也可能是特定于某个供应商或设备的
  USB 设备驱动实际上绑定到接口，而非设备。可以把它们看作“接口驱动”，
  尽管你可能看不到许多这种区分很重要的设备大多USB 设备很简单，只有
  一个功能、一个配置、一个接口和一个备用设置

- 接口具有一个或多个“端点”（endpoint），每个端点支持一种类型和方向的数  传输，例如“批量输出”（bulk out）或“中断输入”（interrupt in）。整  配置在每个方向上最多可有十六个端点，在全部接口之间按需分配
- USB 上的数据传输是分组的（packetized）；每个端点有一个最大包大小。驱  通常必须意识到一些约定，例如使用“短”（包括零长度）包来标志批量传输  结束
- Linux USB API 支持控制消息和批量消息的同步调用。它也支持所有类型数据传  的异步调用，使用称为 “URB”（USB Request Block，USB 请求块）的请求结构
相应地，暴露给设备驱动的 USB 核心 API 涵盖了相当大的范围。你可能需要查USB
3.0 规范（可www.usb.org 免费在线获取）以及类或设备规范
唯一真正接触硬件（读写入寄存器、处IRQ 等）的主机端驱动HCD（主控制器驱动）。理论上，所HCD 都通过相同API 提供相同的功能。在实践中，
这正变得越来越真实，但仍存在差异，尤其是在较不常见控制器上的故障处理方面不同的控制器不一定报告故障的相同方面，并且从故障（包括软件引起的故障，例取消一URB 的链接）中恢复尚未完全一致。设备驱动作者应该特意对每个不同主机控制器驱动进行断开测试（在设备活跃时），以确保驱动自身没有 bug，并确保
它们不依赖于某些 HCD 特定的行为

## USB 标准类型


`include/uapi/linux/usb/ch9.h` 中，你可以找USB 规范9 章中定义USB 数据类型。这些数据类型在 USB 各处以及包括这个主机API、gadget API、usb
字符设备debugfs 接口在内API 中使用。该文件本身`include/linux/usb/ch9.h`
包含，后者还包含一些用于处理这些数据类型的工具例程的声明；其实现位`drivers/usb/common/common.c`
   :export:

此外，一些对创建调试输出有用的函数定义在 `drivers/usb/common/debug.c` 中

## 主机端数据类型与

主机API 向驱动暴露了若干层，其中一些比其他更必要。这些支持主机端驱动设备的生命周期模型，并支持通过 usbcore 将缓冲区传递给为设备驱动执I/O 某个 HCD
   :internal:

## USB 核心 API


USB API 中有两种基本I/O 模型。最基础的那个是异步的：驱动URB 的形提交请求，URB 的完成回调处理下一步。所USB 传输类型都支持该模型，尽控制 URB（总是setup status 阶段，但可能没有数据阶段）和等时 URB（允大数据包并包含每包故障报告）有特殊情况。构建于其上的是同步 API 支持，即
驱动调用一个例程，该例程分配一个或多个 URB、提交它们，并等待它们完成。有
用于单缓冲区控制和批量传输（在某些驱动断开场景中用起来较别扭）的同步包装，
以及用于基于 scatterlist 的流 I/O（批量或中断）的同步包装
USB 驱动需要提供可用于 DMA 的缓冲区，尽管它们不一定需要自己提DMA 映射有一些在分配 DMA 缓冲区时使用API，可以避免在某些系统上使用反弹缓冲区
（bounce buffer）。在某些情况下，驱动可能能够依赖 64 DMA 来消除另一种反缓冲区
   :export:

   :export:

   :export:

   :export:

   :export:

   :export:

   :export:

## 主机控制API


这些 API 仅供主机控制器驱动使用，其中大多数实现了标准的寄存器接口，例XHCI、EHCI、OHCI UHCI。UHCI 是最早的接口之一，由 Intel 设计并被 VIA 使用它在硬件方面做得不多。OHCI 设计得更晚，让硬件做更多工作（更大的传输、跟协议状态等）。EHCI 是伴USB 2.0 设计的；它的设计既有类似 OHCI 的特（硬件做更多工作）也有类UHCI 的特性（ISO 支持的某些部分、TD 列表处理）XHCI 是伴USB 3.0 设计的。它继续将功能支持转移到硬件中
除了“三巨头”之外还有其他主机控制器，尽管大多数基于 PCI 的控制器（以及少PCI 的）使用这些接口之一。并非所有主机控制器都使DMA；有些使PIO，还
有一个模拟器和一个虚拟主机控制器用于通过网路传输 USB
所有这些控制器的驱动都可以使用相同的基础 API。由于历史原因，它们分为两层:c:type:`struct usb_bus <usb_bus>` 是一个相当薄的一层，2.2 内核中变可用，`struct usb_hcd <usb_hcd>` 是一个特性更丰富的层，它HCD 共享
通用代码，从而缩小驱动大小并显著减少 hcd 特定的行为
   :export:

   :export:

   :internal:

## USB 字符设备节点


本章介绍 Linux 字符设备节点。你可能倾向于避免为你的 USB 驱动编写新的内核代码用户模式设备驱动通常被打包为应用程序或库，并可能通过包装它的编程库来使用
字符设备。此类库包括
 - `libusb <http://libusb.sourceforge.net>`__（用C/C++），以及
 - `jUSB <http://jUSB.sourceforge.net>`__（用Java）
关于它的一些旧信息可以USB 指南“USB Device Filesystem一节中看到。USB
指南的最新副本可http://www.linux-usb.org/ 找到

  - 它们过去是通过 **usbfs** 实现的，但这不属sysfs 调试接口的一部分
   - 这个特定的文档是不完整的，尤其是在异步模式方面。自内核 2.5.66 起，代码
     和这份（新的）文档需要交叉审阅
### "devtmpfs" 中有哪些文件

传统上挂载在 `/dev/bus/usb/`，usbfs 的特性包括：

- `/dev/bus/usb/BBB/DDD` …暴露每个设备的配置描述符、并支持一系列用于发出
   设备请求（包括对设备I/O）的 ioctl 的魔法文件。（纯粹供程序访问。）

每个总线被赋予一个编号（`BBB`），基于它被枚举的时间；在每个总线内，每个设备
被赋予一个类似的编号（`DDD`）。那`BBB/DDD` 路径不是“稳定”的标识符；
即使你总是把设备插在同一个集线器端口上，也预期它们会变化甚至不要想把它们
保存在应用程序的配置文件中 有稳定的标识符可供想要使用它们的用户模式
应用程序使用。HID 和网络设备暴露这些稳ID，因此例如你可以确定你告诉了正确UPS 去关闭它的第二个服务器。请注意，它（还）没有暴露那ID
### /dev/bus/usb/BBB/DDD


以以下基本方式之一使用这些文件
- **可以对它们进行读取，** 首先产生设备描述符（18 字节），然后是当前配置的
  描述符。有关这些二进制数据格式的详细信息，请参USB 2.0 规范。你将需要把
  大部分多字节值从 little endian 格式转换到你的原生主机字节序，尽管设备描述符
  中的少数字段（两BCD 编码字段，以及供应商和产ID）已经为你做了字节交换  注意配置描述符包含接口、备用设置、端点的描述符，以及可能额外的类描述符
- **执行 USB 操作** 使用 **ioctl()** 请求来发出端I/O 请求（同步或异步）或
  管理设备。这些请求需`CAP_SYS_RAWIO` 能力，以及文件系统访问权限。一次只  在这些设备文件之一上发出一ioctl 请求。这意味着如果你正从一个线程同步读  一个端点，在读取完成之前你将无法从另一个线程写入不同的端点。这**半双*
  （half duplex）协议有效，但在其他情况下你会使用异I/O 请求
每个连接USB 设备有一个文件。`BBB` 表示总线编号。`DDD` 表示该总线上的
设备地址。这两个数字都是顺序分配的，并且可以被重用，因此你不能依赖它们来
稳定地访问设备。例如，设备在仍连接时重新枚举（也许有人碰动了它们的电源、集线器
USB 电缆）是相对常见的，因此一个设备在你首次连接时可能`002/027`，稍后变成 `002/048`
这些文件可以作为二进制数据读取。二进制数据首先由设备描述符组成，然后是设备每个配置的描述符。设备描述符中的多字节字段由内核转换为宿主字节序配置描述符是总线字节序（bus endian）格式！配置描述符彼此相wTotalLength
字节。如果一个设备返回的的配置描述符数据少于 wTotalLength 指示的，文件缺失字节处将出现一个空洞。此信息也以文本形式显示`/sys/kernel/debug/usb/devices`
文件中，稍后描述
这些文件也可以用于为 USB 设备编写用户级驱动。你会以写方式打开
`/dev/bus/usb/BBB/DDD` 文件，读取它的描述符以确认它是你期望的设备，然后
使用一ioctl 调用绑定到一个（或可能几个）接口。你会向设备发出更多 ioctl
以使用控制、批量或其他种类USB 传输与之通信。这IOCTL 列在
`<linux/usbdevice_fs.h>` 文件中，在撰写本文时，源代码（`linux/drivers/usb/core/devio.c`是如何通过这些文件访问设备的主要参考
注意，由于默认情况下这些 `BBB/DDD` 文件只能root 写入，只root 可以编写
此类用户模式驱动。你可以通过使用 `chmod` 有选择地授予其他用户读/写权限此外，像 `devmode=0666` 这样usbfs 挂载选项可能有帮助

### 用户模式驱动的生命周

这样一个驱动首先需要为它知道如何处理的设备找到一个设备文件。也许它是因`/sbin/hotplug` 事件处理代理选择了该驱动来处理新设备而被告知的。或者它可能一个扫描所`/dev/bus/usb` 设备文件并忽略大多数设备的应用程序。无论哪情况，它都应`read()` 设备文件中的所有描述符，并将它们与它知道如何处理的
进行核对。它可能只是拒绝除特定供应商和产ID 之外的所有东西，或者需要更复杂策略
绝不要假设系统上一次只会有一个这样的设备！如果你的代码不能同时处理多个设备，
至少要在有多于一个时检测出来，并让你的用户选择使用哪个设备
一旦你的用户模式驱动知道要使用哪个设备，它就会以两种风格之一与之交互。简单的
风格是只发出控制请求；有些设备不需要比这更复杂的交互。（一个例子可能是软件
使用供应商特定的控制请求进行一些初始化或配置任务，其余部分使用内核驱动。）

更可能的是，你需要一个更复杂的风格驱动：一个使用非控制端点、读取或写入数据
并声明独占使用一个接口的驱动*批量** 传输最容易使用，但只有它们的兄**中断** 传输能与低速设备一起工作。中断和 **等时** 传输都提供服各保证，因为
它们的带宽是预留的。这种“周期性”传输通过 usbfs 使用起来很别扭，除非你使异步调用。然而，中断传输也可以以同步的“一次性”风格使用
你的用户模式驱动永远不需要操心在设备断开时清理请求状态，尽管它应该在一开看到 ENODEV 错误时尽快关闭它打开的文件描述符
### ioctl() 请求


要使用这ioctl，你需要在你的代码中包含以下头文件
```

    #include <linux/usb.h>
    #include <linux/usbdevice_fs.h>
    #include <asm/byteorder.h>

```
标准USB 设备模型请求，来USB 2.0 规范“Chapter 9”，会从 `<linux/usb/ch9.h>`
头文件自动包含
除非另有说明，这里描述的 ioctl 请求会更新它们所应用于的 usbfs 文件的修改时（除非它们失败）。返回零表示成功；否则，返回一个标USB 错误码（这些usb-error-codes 中有文档）
这些文件中的每一个都多路复用了到若干 I/O 流的访问，每个端点一个。每个设备有
一个控制端点（端点零），它支持有限RPC 风格访问。设备由 hub_wq（在内核中）
设置影响功耗和基本功能等事物的设备**配置** 来配置。端点是 USB **接口** 一部分，接口可能具有影响哪些端点可用等事物**备用设置**（altsetting）。许设备只有一个配置和一个接口，因此它们的驱动会忽略配置和备用设置
#### 管理/状态请

许多 usbfs 请求并不直接处理设备 I/O。它们主要与设备管理和状态相关。这些都同步请求
USBDEVFS_CLAIMINTERFACE
    这用于强usbfs 声明一个特定接口，该接口之前未usbfs 或任何其他内    驱动声明。ioctl 参数是一个保存接口编号（来自描述符的 bInterfaceNumber    的整数
    注意，如果你的驱动在尝试使用某个接口的一个端点之前没有声明它，并且没有其    驱动绑定到它，那么该接口会被 usbfs 自动声明
    此声明会RELEASEINTERFACE ioctl 释放，或在关闭文件描述符时释放。文    修改时间不会因为此请求而更新
USBDEVFS_CONNECTINFO
    说明设备是否为低速。ioctl 参数指向一
```

	struct usbdevfs_connectinfo {
		unsigned int   devnum;
		unsigned char  slow;
	};

    文件修改时间不会因为此请求而更新
    *你无法分辨“非慢速”设备是以高速（480 MBit/sec）还是全速（12 MBit/sec    连接的 你应该已经知devnum 值，它就是设备文件名DDD 值
```
USBDEVFS_GET_SPEED
    返回设备的速度。该速度作为一个数值返回，依据 enum usb_device_speed
    文件修改时间不会因为此请求而更新
USBDEVFS_GETDRIVER
    返回绑定到给定接口（一个字符串）的内核驱动的名称。参数是一个指向此结构    指针，该结构
```

	struct usbdevfs_getdriver {
		unsigned int  interface;
		char          driver[USBDEVFS_MAXDRIVERNAME + 1];
	};

    文件修改时间不会因为此请求而更新
```
USBDEVFS_IOCTL
    将来自用户空间的请求向下传递到一个已绑定的内核驱
```

	struct usbdevfs_ioctl {
		int     ifno;
		int     ioctl_code;
		void    *data;
	};

	/* user mode call looks like this.
	 * 'request' becomes the driver->ioctl() 'code' parameter.
	 * the size of 'param' is encoded in 'request', and that data
	 * is copied to or from the driver->ioctl() 'buf' parameter.
	 */
	static int
	usbdev_ioctl (int fd, int ifno, unsigned request, void *param)
	{
		struct usbdevfs_ioctl   wrapper;

		wrapper.ifno = ifno;
		wrapper.ioctl_code = request;
		wrapper.data = param;

		return ioctl (fd, USBDEVFS_IOCTL, &wrapper);
	}

    文件修改时间不会因为此请求而更新
    此请求让内核驱动通过文件系统操作与用户模式代码对话，即使它们没有创建字符    块特殊设备。它也被用来做诸如询问设备应使用哪个设备特殊文件之类的事情。两    预定义的 ioctl 用于断开和重新连接内核驱动，以便用户模式代码可以完全管理设备
    的绑定和配置
```
USBDEVFS_RELEASEINTERFACE
    这用于在关闭文件描述符之前，释放 usbfs 对接口所做的声明（无论是隐式的还    由于 USBDEVFS_CLAIMINTERFACE 调用）。ioctl 参数是一个保存接口编号（来自
    描述符的 bInterfaceNumber）的整数；文件修改时间不会因为此请求而更新
```

	*不进行检查以确保发起声明的任务就是释放它的那个任务。这意味着用户模式
	驱动可能会干扰其他驱动

```
USBDEVFS_RESETEP
    将端点（批量或中断）的数据切换值重置为 DATA0。ioctl 参数是一个整数的端点
    编号 15，如端点描述符中所标识），如果设备的端点向主机发送数据，    加上 USB_DIR_IN
```

	*避免使用此请求。它可能应该被移除 使用它通常意味着设备和驱动将失去
	切换同步。如果你真的失去了同步，你可能需要使用像 CLEAR_HALT 	SET_INTERFACE 这样的请求与设备完全握手
```
USBDEVFS_DROP_PRIVILEGES
    这用于放弃在 usbfs 文件描述符上执行某些被认为是特权操作的能力。这包括声明
    任意接口、重置一个当前有其他用户声明了接口的设备，以及发USBDEVFS_IOCTL
    调用。ioctl 参数是一32 位掩码，表示用户被允许在此文件描述符上声明的接口    你可以多次发出此 ioctl 以收窄该掩码
#### 同步 I/O 支持


同步请求涉及内核阻塞，直到用户模式请求完成，要么成功完成，要么报告错误。在
大多数情况下这是使用 usbfs 的最简单方式，尽管如上所述，它确实阻止了同时向多端点执行 I/O
USBDEVFS_BULK
    向设备发出一个批量读或写请求。ioctl

```

	struct usbdevfs_bulktransfer {
		unsigned int  ep;
		unsigned int  len;
		unsigned int  timeout; /* in milliseconds */
		void          *data;
	};

    ``ep`` 值标识一个批量端点编号（1 15，如端点描述符中所标识），当引用一    从设备向主机发送数据的端点时，USB_DIR_IN 掩码。数据缓冲区的长度由 ``len``
    标识；近期的内核支持高达128K 字节的请求FIXME 说明如何返回读取长度    以及如何处理短读
```
USBDEVFS_CLEAR_HALT
    清除端点暂停（halt/stall）并重置端点切换。这仅对标量或中断端点有意义。ioctl
    参数是一个整数的端点编号 15，如端点描述符中所标识），当引用一个从设备
    向主机发送数据的端点时，USB_DIR_IN 掩码
    在已经停止（stall）、向数据传输请求返回 `-EPIPE` 状态的批量或中断端点上使用
    它。不要直接发出控制请求，因为那会使主机的数据切换记录失效
USBDEVFS_CONTROL
    向设备发出一个控制请求。ioctl 参数指向

```

	struct usbdevfs_ctrltransfer {
		__u8   bRequestType;
		__u8   bRequest;
		__u16  wValue;
		__u16  wIndex;
		__u16  wLength;
		__u32  timeout;  /* in milliseconds */
		void   *data;
	};

    此结构的前八个字节是要发送给设备SETUP 包的内容；详USB 2.0 规范    bRequestType 值是通过组合一``USB_TYPE_*`` 值、一``USB_DIR_*`` 值和一    ``USB_RECIP_*`` 值（来自 ``linux/usb.h``）构成的。如wLength 非零，它描述
    数据缓冲区长度，该缓冲区或者被写入设备（USB_DIR_OUT），或者从设备读取
    （USB_DIR_IN）
    在撰写本文时，你不能在设备和主机之间传输超过 4 KB 的数据；usbfs 有一个限制，
    一些主机控制器驱动也有一个限制。（这通常不是问题。）*另外*，没有办法说    设备获得短读回是不可以的
```
USBDEVFS_RESET
    执行一USB 级的设备复位。ioctl 参数被忽略。复位之后，这会重新绑定所    设备接口。文件修改时间不会因为此请求而更新

	**避免使用此调* 直到某些 usbcore bug 被修复，因为它没有完全同步设备	接口和驱动（不仅仅是 usbfs）状态
USBDEVFS_SETINTERFACE
    设置接口的备用设置。ioctl 参数

```

	struct usbdevfs_setinterface {
		unsigned int  interface;
		unsigned int  altsetting;
	};

    文件修改时间不会因为此请求而更新
    那些结构成员来自应用于当前配置的某个接口描述符。接口编号是 bInterfaceNumber
    值，备用设置编号bAlternateSetting 值。（这会重置接口中的每个端点。）

```
USBDEVFS_SETCONFIGURATION
    为设备发`usb_set_configuration()` 调用。参数是一个保存配置编号（来自
    描述符的 bConfigurationValue）的整数。文件修改时间不会因为此请求而更新

	**避免使用此调* 直到某些 usbcore bug 被修复，因为它没有完全同步设备	接口和驱动（不仅仅是 usbfs）状态
#### 异步 I/O 支持


如上所述，在某些情况下，从用户模式代码发起并发操作可能很重要。这对周期性传（中断和等时）尤其重要，但它也可以用于其他种类的 USB 请求。在这种情况下，这里
描述的异步请求是必不可少的。不是提交一个请求并让内核阻塞直到它完成，而是将阻分离开来
这些请求被打包到一个类似于内核设备驱动使用URB 的结构中。（这里没有 POSIX
异步 I/O 支持，抱歉。）它标识端点类型（`USBDEVFS_URB_TYPE_*`）、端点（编号酌情USB_DIR_IN 掩码）、缓冲区和长度，以及一个用于唯一标识每个请求的用“上下文”值。（它通常是指向每请求数据的指针。）标志可以修改请求（没有内核驱支持的那么多）
每个请求可以指定一个实时信号编号（SIGRTMIN SIGRTMAX 之间，含边界），请求在请求完成时发送一个信号
usbfs 返回这些 urb 时，状态值被更新，并且缓冲区可能已被修改。除了等时传之外，actual_length 被更新以说明传输了多少字节；如果设置USBDEVFS_URB_DISABLE_SPD 标志（“短包不可以”），如
```

    struct usbdevfs_iso_packet_desc {
	    unsigned int                     length;
	    unsigned int                     actual_length;
	    unsigned int                     status;
    };

    struct usbdevfs_urb {
	    unsigned char                    type;
	    unsigned char                    endpoint;
	    int                              status;
	    unsigned int                     flags;
	    void                             *buffer;
	    int                              buffer_length;
	    int                              actual_length;
	    int                              start_frame;
	    int                              number_of_packets;
	    int                              error_count;
	    unsigned int                     signr;
	    void                             *usercontext;
	    struct usbdevfs_iso_packet_desc  iso_frame_desc[];
    };

```
对于这些异步请求，文件修改时间反映请求被发起的时间。这与它们在同步请求中的
使用形成对比，在同步请求中它反映请求完成的时间
USBDEVFS_DISCARDURB
    **TBS** 文件修改时间不会因为此请求而更新
USBDEVFS_DISCSIGNAL
    **TBS** 文件修改时间不会因为此请求而更新
USBDEVFS_REAPURB
    **TBS** 文件修改时间不会因为此请求而更新
USBDEVFS_REAPURBNDELAY
    **TBS** 文件修改时间不会因为此请求而更新
USBDEVFS_SUBMITURB
    **TBS**

## USB 设备


USB 设备现在通过 debugfs 导出
- `/sys/kernel/debug/usb/devices` …一个文本文件，显示内核已知的每USB 设备
   及其配置描述符。你也可poll() 它来了解新设备
### /sys/kernel/debug/usb/devices


此文件对用户模式中的状态查看工具很方便，这些工具可以扫描文本格式并忽略大部内容。更详细的设备状态（包括类和供应商状态）可从设备特定的文件中获取。关于此
文件当前格式的信息，请见下文
此文件与 poll() 系统调用结合，也可以用于

```

    int fd;
    struct pollfd pfd;

    fd = open("/sys/kernel/debug/usb/devices", O_RDONLY);
    pfd = { fd, POLLIN, 0 };
    for (;;) {
	/* The first time through, this call will return immediately. */
	poll(&pfd, 1, -1);

	/* To see what's changed, compare the file's previous and current
	   contents or scan the filesystem.  (Scanning is more precise.) */
    }

```
注意，这种行为旨在用于信息和调试目的。例如，使用udev HAL 这样的程序来
初始化设备或启动用户模式辅助程序会更合适
在此文件中，每个设备的输出有多行 ASCII 输出
我特意把它做ASCII 而非二进制，以便有人无需使用辅助程序就能从中获取一有用数据。但是，借助辅助程序，每`T:` 行（拓扑信息：Lev、Prnt、Port、Cnt的前 4 列中的数字可用于构建 USB 拓扑图
```

	T = Topology (etc.)
	B = Bandwidth (applies only to USB host controllers, which are
	virtualized as root hubs)
	D = Device descriptor info.
	P = Product ID info. (from Device descriptor, but they won't fit
	together on one line)
	S = String descriptors.
	C = Configuration descriptor info. (* = active configuration)
	I = Interface descriptor info.
	E = Endpoint descriptor info.

```
#### /sys/kernel/debug/usb/devices 输出格式


```

  d = decimal number (may have leading spaces or 0's)
  x = hexadecimal number (may have leading spaces or 0's)
  s = string



```
##### 拓扑信息


```

	T:  Bus=dd Lev=dd Prnt=dd Port=dd Cnt=dd Dev#=ddd Spd=dddd MxCh=dd
	|   |      |      |       |       |      |        |        |__MaxChildren
	|   |      |      |       |       |      |        |__Device Speed in Mbps
	|   |      |      |       |       |      |__DeviceNumber
	|   |      |      |       |       |__Count of devices at this level
	|   |      |      |       |__Connector/Port on Parent for this device
	|   |      |      |__Parent DeviceNumber
	|   |      |__Level in topology for this bus
	|   |__Bus number
	|__Topology info tag

```
速度可能是：

	======= ======================================================
	1.5	Mbit/s for low speed USB
	12	Mbit/s for full speed USB
	480	Mbit/s for high speed USB (added for USB 2.0)
	5000	Mbit/s for SuperSpeed USB (added for USB 3.0)
	======= ======================================================

由于迷失在时间的迷雾中原因，端口号总是比实际小 1。例如，插入端口 4 的设将显示为 `Port=03`
##### 带宽信息


```

	B:  Alloc=ddd/ddd us (xx%), #Int=ddd, #Iso=ddd
	|   |                       |         |__Number of isochronous requests
	|   |                       |__Number of interrupt requests
	|   |__Total Bandwidth allocated to this bus
	|__Bandwidth info tag

```
带宽分配是对一帧（毫秒）中有多少被使用的一个近似。它只反映周期性传输，这是
唯一预留带宽的传输。控制和批量传输使用所有其他带宽，包括未被用于传输（例用于短包）的预留带宽
该百分比是那些传输调度了多少“预留”带宽。对于低速或全速总线（粗略地说是
“USB 1.1”），预留了 90% 的总线带宽。对于高速总线（粗略地说是“USB 2.0”）
预留80%

##### 设备描述符信息与产品 ID 信息


```

	D:  Ver=x.xx Cls=xx(s) Sub=xx Prot=xx MxPS=dd #Cfgs=dd
	P:  Vendor=xxxx ProdID=xxxx Rev=xx.xx

```
```

	D:  Ver=x.xx Cls=xx(sssss) Sub=xx Prot=xx MxPS=dd #Cfgs=dd
	|   |        |             |      |       |       |__NumberConfigurations
	|   |        |             |      |       |__MaxPacketSize of Default Endpoint
	|   |        |             |      |__DeviceProtocol
	|   |        |             |__DeviceSubClass
	|   |        |__DeviceClass
	|   |__Device USB version
	|__Device info tag #1

```
```

	P:  Vendor=xxxx ProdID=xxxx Rev=xx.xx
	|   |           |           |__Product revision number
	|   |           |__Product ID code
	|   |__Vendor ID code
	|__Device info tag #2


```
##### 字符串描述符信息


```

	S:  Manufacturer=ssss
	|   |__Manufacturer of this device as read from the device.
	|      For USB host controller drivers (virtual root hubs) this may
	|      be omitted, or (for newer drivers) will identify the kernel
	|      version and the driver which provides this hub emulation.
	|__String info tag

	S:  Product=ssss
	|   |__Product description of this device as read from the device.
	|      For older USB host controller drivers (virtual root hubs) this
	|      indicates the driver; for newer ones, it's a product (and vendor)
	|      description that often comes from the kernel's PCI ID database.
	|__String info tag

	S:  SerialNumber=ssss
	|   |__Serial Number of this device as read from the device.
	|      For USB host controller drivers (virtual root hubs) this is
	|      some unique ID, normally a bus ID (address or slot name) that
	|      can't be shared with any other device.
	|__String info tag



```
##### 閰嶇疆鎻忚堪绗︿俊鎭。

```

	C:* #Ifs=dd Cfg#=dd Atr=xx MPwr=dddmA
	| | |       |       |      |__MaxPower in mA
	| | |       |       |__Attributes
	| | |       |__ConfiguratioNumber
	| | |__NumberOfInterfaces
	| |__ "*" indicates the active configuration (others are " ")
	|__Config info tag

```
USB 设备可能有多个配置，每个的行为大不相同。例如，一个总线供电的配置可能比
自供电的配置能力弱得多。一次只能有一个设备配置处于活动状态；大多数设备只一个配置
每个配置由一个或多个接口组成。每个接口服务于一个不同的“功能”，通常绑定不同USB 设备驱动。一个常见的例子是一个带有用于播放的音频接口和用于软件音控制HID 接口USB 扬声器
##### 接口描述符信息（每个配置可有多个

```

	I:* If#=dd Alt=dd #EPs=dd Cls=xx(sssss) Sub=xx Prot=xx Driver=ssss
	| | |      |      |       |             |      |       |__Driver name
	| | |      |      |       |             |      |          or "(none)"
	| | |      |      |       |             |      |__InterfaceProtocol
	| | |      |      |       |             |__InterfaceSubClass
	| | |      |      |       |__InterfaceClass
	| | |      |      |__NumberOfEndpoints
	| | |      |__AlternateSettingNumber
	| | |__InterfaceNumber
	| |__ "*" indicates the active altsetting (others are " ")
	|__Interface info tag

```
一个给定的接口可能有一个或多个“备用”设置。例如，默认设置可能不使用超过少量的
周期性带宽。要使用总线带宽的显著部分，驱动必须选择一个非默认的备用设置
一个接口一次只能有一个设置处于活动状态，并且一次只能有一个驱动绑定到一接口。大多数设备每个接口只有一个备用设置

##### 端点描述符信息（每个接口可有多个

```

	E:  Ad=xx(s) Atr=xx(ssss) MxPS=dddd Ivl=dddss
	|   |        |            |         |__Interval (max) between transfers
	|   |        |            |__EndpointMaxPacketSize
	|   |        |__Attributes(EndpointType)
	|   |__EndpointAddress(I=In,O=Out)
	|__Endpoint info tag

```
对于所有周期性（中断或等时）端点，间隔都是非零的。对于高速端点，传输间隔可能
以微秒而非毫秒来度量
对于高速周期性端点，`EndpointMaxPacketSize` 反映每微帧的数据传输大小。对“高带宽”端点，那可以反映每个端点两个或三个包（125 微秒最3KB）
使用 Linux-USB 协议栈，周期性带宽预留使URB 提供的传输间隔和大小，它们可小于端点描述符中找到的那些
#### 使用示例


如果用户或脚本只对标量信息感兴趣，例如，使用类似
`grep ^T: /sys/kernel/debug/usb/devices` 的命令只获取拓扑行。像
`grep -i ^[tdp]: /sys/kernel/debug/usb/devices` 这样的命令可用于只列出以方括号中
字符开头的行，其中有效字符TDPCIE。借助稍强一点的脚本，它可以显示任何选定行（例如，只T、D P 行）并更改它们的输出格式。（`procusb` Perl 脚本这个想法的开端。它将只列出TBDPSCIE 中选定的行，或来自 `/sys/kernel/debug/usb/devices`
的“所有”行。）

拓扑行可用于生成系统根集线器USB 设备的图图示。（关于如何执行此操作，参见下文更多内容。）

接口行可用于确定每个设备正在使用什么驱动，以及它激活了哪个备用设置
配置行可用于列出系统 USB 设备正在使用的最大功率（以毫安为单位）。例如，
`grep ^C: /sys/kernel/debug/usb/devices`銆。

这是一个例子，来自一个具UHCI 根集线器、连接到根集线器的外部集线器，以连接到外部集线器的鼠标和串行转换器的系统
```

	T:  Bus=00 Lev=00 Prnt=00 Port=00 Cnt=00 Dev#=  1 Spd=12   MxCh= 2
	B:  Alloc= 28/900 us ( 3%), #Int=  2, #Iso=  0
	D:  Ver= 1.00 Cls=09(hub  ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
	P:  Vendor=0000 ProdID=0000 Rev= 0.00
	S:  Product=USB UHCI Root Hub
	S:  SerialNumber=dce0
	C:* #Ifs= 1 Cfg#= 1 Atr=40 MxPwr=  0mA
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=09(hub  ) Sub=00 Prot=00 Driver=hub
	E:  Ad=81(I) Atr=03(Int.) MxPS=   8 Ivl=255ms

	T:  Bus=00 Lev=01 Prnt=01 Port=00 Cnt=01 Dev#=  2 Spd=12   MxCh= 4
	D:  Ver= 1.00 Cls=09(hub  ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
	P:  Vendor=0451 ProdID=1446 Rev= 1.00
	C:* #Ifs= 1 Cfg#= 1 Atr=e0 MxPwr=100mA
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=09(hub  ) Sub=00 Prot=00 Driver=hub
	E:  Ad=81(I) Atr=03(Int.) MxPS=   1 Ivl=255ms

	T:  Bus=00 Lev=02 Prnt=02 Port=00 Cnt=01 Dev#=  3 Spd=1.5  MxCh= 0
	D:  Ver= 1.00 Cls=00(>ifc ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
	P:  Vendor=04b4 ProdID=0001 Rev= 0.00
	C:* #Ifs= 1 Cfg#= 1 Atr=80 MxPwr=100mA
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=03(HID  ) Sub=01 Prot=02 Driver=mouse
	E:  Ad=81(I) Atr=03(Int.) MxPS=   3 Ivl= 10ms

	T:  Bus=00 Lev=02 Prnt=02 Port=02 Cnt=02 Dev#=  4 Spd=12   MxCh= 0
	D:  Ver= 1.00 Cls=00(>ifc ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
	P:  Vendor=0565 ProdID=0001 Rev= 1.08
	S:  Manufacturer=Peracom Networks, Inc.
	S:  Product=Peracom USB to Serial Converter
	C:* #Ifs= 1 Cfg#= 1 Atr=a0 MxPwr=100mA
	I:  If#= 0 Alt= 0 #EPs= 3 Cls=00(>ifc ) Sub=00 Prot=00 Driver=serial
	E:  Ad=81(I) Atr=02(Bulk) MxPS=  64 Ivl= 16ms
	E:  Ad=01(O) Atr=02(Bulk) MxPS=  16 Ivl= 16ms
	E:  Ad=82(I) Atr=03(Int.) MxPS=   8 Ivl=  8ms


```
只从此中选择 `T:` `I:` 行（例如，使`procusb ti`），我们
```

	T:  Bus=00 Lev=00 Prnt=00 Port=00 Cnt=00 Dev#=  1 Spd=12   MxCh= 2
	T:  Bus=00 Lev=01 Prnt=01 Port=00 Cnt=01 Dev#=  2 Spd=12   MxCh= 4
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=09(hub  ) Sub=00 Prot=00 Driver=hub
	T:  Bus=00 Lev=02 Prnt=02 Port=00 Cnt=01 Dev#=  3 Spd=1.5  MxCh= 0
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=03(HID  ) Sub=01 Prot=02 Driver=mouse
	T:  Bus=00 Lev=02 Prnt=02 Port=02 Cnt=02 Dev#=  4 Spd=12   MxCh= 0
	I:  If#= 0 Alt= 0 #EPs= 3 Cls=00(>ifc ) Sub=00 Prot=00 Driver=serial


```
```

                      +------------------+
                      |  PC/root_hub (12)|   Dev# = 1
                      +------------------+   (nn) is Mbps.
    Level 0           |  CN.0   |  CN.1  |   [CN = connector/port #]
                      +------------------+
                          /
                         /
            +-----------------------+
  Level 1   | Dev#2: 4-port hub (12)|
            +-----------------------+
            |CN.0 |CN.1 |CN.2 |CN.3 |
            +-----------------------+
                \           \____________________
                 \_____                          \
                       \                          \
               +--------------------+      +--------------------+
  Level 2      | Dev# 3: mouse (1.5)|      | Dev# 4: serial (12)|
               +--------------------+      +--------------------+



```
或者，以更像树的结构（不带端口 [连接器] 

```

	PC:  Dev# 1, root hub, 2 ports, 12 Mbps
	|_ CN.0:  Dev# 2, hub, 4 ports, 12 Mbps
	     |_ CN.0:  Dev #3, mouse, 1.5 Mbps
	     |_ CN.1:
	     |_ CN.2:  Dev #4, serial, 12 Mbps
	     |_ CN.3:
	|_ CN.1:

```
