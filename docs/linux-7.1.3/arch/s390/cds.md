## Linux for S/390 涓?zSeries


通用设备支持（CDS设备驱动 I/O 支持例程

作者：
 - Ingo Adlung
 - Cornelia Huck

版权所有，IBM Corp. 1999-2002

## 简

本文档描述了 Linux/390 的通用设备支持例程。与其他硬件架构不同，ESA/390
定义了一种统一I/O 访问方法。这减轻了设备驱动的负担，因为它们不必再去处不同的总线类型、轮询与中断处理、共享与非共享中断处理、DMA 与端I/O（PIO以及其他更多的硬件特性。然而，这意味着要么每个设备驱动都需要自行实现硬I/O 挂接功能，要么操作系统提供一种统一的访问硬件的方法，提供每个设备驱本应自行提供的全部功能
本文档无意逐条解释 ESA/390 硬件架构的细节。这些信息可ESA/390 操作原理手册
（IBM 表单SA22-7201）获取
为了ESA/390 I/O 接口构建通用设备支持，引入了一个功能层，它向硬件提通用I/O 访问方法
通用设备支持层包含了下文定义I/O 支持例程。其中一些实现了通用Linux
设备驱动接口，另一些则ESA/390 平台特有的
注意  要为 S/390 编写驱动，你还需要查Documentation/arch/s390/driver-model.rst
  中描述的接口
2.4 移植驱动的注意事项：

主要变更如下
- 函数使用 ccw_device 而非 irq（子通道）- 所有驱动都必须定义一ccw_driver（见 driver-model.txt）以及相关的函数- request_irq() free_irq() 不再由驱动完成- oper_handler 被（某种程度上）ccw_driver probe() set_online() 函数替代- not_oper_handler 被（某种程度上）ccw_driver remove() set_offline() 函数替代- 通道设备层已移除- 中断处理函数必须改写为以 ccw_device 作为参数。此外，它们不再返回 devstat  而是返回 irb- 在发起一io 之前，必须通过 ccw_device_set_options() 设置选项- 取代调用 read_dev_chars()/read_conf_data()，驱动改为发出通道程序并自行处  中断
ccw_device_get_ciw()
   从扩sense 数据中获取命令
ccw_device_start(), ccw_device_start_timeout(), ccw_device_start_key(), ccw_device_start_key_timeout()
   发起一I/O 请求
ccw_device_resume()
   恢复通道程序的执行
ccw_device_halt()
   终止设备上正在处理的当前 I/O 请求
do_IRQ()
   通用中断例程。每当系统收到一I/O 中断时，该函数由中断入口例程调用   do_IRQ() 例程确定中断状态，并根据发I/O 请求时（通过 do_IO()）定义的规则
   （标志）调用设备特定的中断处理函数
后续章节会更详细地描述除 do_IRQ() 之外的函数。do_IRQ() 接口未被描述，因为它
仅由 Linux/390 的一级中断处理程序调用，并不包含可被子设备驱动调用的接口相反，do_IO() 的功能描述同时也描述了提供给设备特定中断处理函数的输入
注意	以下所有说明同样适用64 位架s390x

## Linux/390 设备驱动的通用设备支持（CDS

### 概述


以下章节描述Linux/390 通用设备支持（CDS）提供给设备特定驱动实现所使用I/O 相关接口例程，这些实现运行在 IBM ESA/390 硬件平台上。这些接口旨在提每种设备驱动实现所需的、用于在 ESA/390 平台上驱动特定硬件设备的功能。其一些接口例程是 Linux/390 特有的，另一些也能在其他 Linux 平台实现中找到各种函数原型、数据声明和宏定义可以在架构特定C 头文linux/arch/s390/include/asm/irq.h 中找到
### CDS 接口概念概览


与其他硬件平台不同，ESA/390 架构并没有定义由特定中断控制器管理的中断线，
也没有定义可能允许或不允许共享中断、DMA 处理等的总线系统。相反，ESA/390
架构实现了一个所谓的通道子系统，它对物理连接到系统的设备提供统一的视图尽管 ESA/390 硬件平台支持种类繁多的外围附件，例如磁盘设备（即 DASD）、磁带通信控制器等，但它们都可以通过一个定义良好的访问方法来访问，并且都以统一
的方式呈I/O 完成：I/O 中断。每个设备都由所谓的子通道唯一标识，ESA/390
架构允许挂载多达 64k 个设备
然而，Linux 最初构建在 Intel PC 架构之上，该架构带有两级级联8259 可编中断控制器（PIC），最多允15 条不同的中断线。挂载到此类系统的所有设备共15 个中断级别。挂载到 ISA 总线系统的设备不得共享中断级别（IRQ），因为
ISA 总线基于边沿触发中断。MCA、EISA、PCI 以及其他总线系统基于电平触发中断因此允许共享 IRQ。不过，如果多个设备通过相同（共享）IRQ 呈现其硬件状态，
操作系统必须调用注册在该 IRQ 上的每一个设备驱动，以确定拥有引发中断的设备那个驱动
2.4 内核及之前，Linux/390 通过 IRQ（子通道）提供接口。对于通用 I/O 层的
内部使用，这些接口仍然存在。但是，设备驱动应当只使用通过 ccw_device 的新
调用接口
在启动期间，Linux/390 系统会检测外围设备。其中每个设备由 ESA/390 通道子系通过所谓的子通道唯一确定。虽然子通道号是系统生成的，但每个子通道还带有一用户定义的属性，即所谓的设备号。子通道号和设备号都不能超过 65535。在 sysfs
初始化期间，会收集控制部件类型和设备类型的信息，这些信息意味着操作设备所需特定 I/O 命令（通道命令字——CCW）。设备驱动可以在其初始化步骤中获取这组硬信息，利用提供给它们struct ccw_device 中保存的信息来识别其支持的设备这种方法意味着 Linux/390 不需要探测空闲（未占用）的中断请求线（IRQ）来驱动设备。在适用的情况下，设备驱动可以在online 例程中发READ DEVICE
CHARACTERISTICS ccw 来获取设备特性
为了便于发起 I/O，CDS 层提供了 ccw_device_start() 接口，它以设备特定的通道
程序（一个或多个 CCW）作为输入，建立所需的架构特定控制块，并代表设备驱动发起
一I/O 请求。ccw_device_start() 例程允许指定是希CDS 层对观察到的每一中断都通知设备驱动，还是仅在最终状态时通知。详ccw_device_start()。设备驱绝不可自行发ESA/390 I/O 命令，而必须使Linux/390 CDS 接口
对于需要取消的长时间运I/O 请求，CDS 层提供了 ccw_device_halt() 函数。有设备需要先发出一HALT SUBCHANNEL（HSCH）命令，而不必挂I/O 请求。该功能
也由 ccw_device_halt() 覆盖
get_ciw() - 获取命令信息
该调用使设备驱动能够从扩SenseID 数据中获取有关所支持命令的信息
```

  struct ciw *
  ccw_device_get_ciw(struct ccw_device *cdev, __u32 cmd);

```
====  ========================================================
cdev  要获取其命令ccw_devicecmd   要获取的命令类型====  ========================================================

ccw_device_get_ciw() 返回
=====  ================================================================
 NULL  无扩展数据可用、设备无效或命令未找到!NULL  所请求的命令=====  ================================================================

```

  ccw_device_start() - 发起 I/O 请求

```
ccw_device_start() 例程I/O 请求的前端处理器。所有设备驱动的 I/O 请求都必通过该例程发出。设备驱动不得自行发ESA/390 I/O 命令。相反，ccw_device_start()
例程提供了驱动任意设备所需的全部接口
本描述也涵盖了传递给设备驱动中断处理函数的状态信息，因为这关联到调用
ccw_device_start() 时与相应 I/O 请求一起定义的规则（标志）
```

  int ccw_device_start(struct ccw_device *cdev,
		       struct ccw1 *cpa,
		       unsigned long intparm,
		       __u8 lpm,
		       unsigned long flags);
  int ccw_device_start_timeout(struct ccw_device *cdev,
			       struct ccw1 *cpa,
			       unsigned long intparm,
			       __u8 lpm,
			       unsigned long flags,
			       int expires);
  int ccw_device_start_key(struct ccw_device *cdev,
			   struct ccw1 *cpa,
			   unsigned long intparm,
			   __u8 lpm,
			   __u8 key,
			   unsigned long flags);
  int ccw_device_start_key_timeout(struct ccw_device *cdev,
				   struct ccw1 *cpa,
				   unsigned long intparm,
				   __u8 lpm,
				   __u8 key,
				   unsigned long flags,
				   int expires);

```
============= =============================================================
cdev           I/O 所发往ccw_device
cpa           通道程序的逻辑起始地址
user_intparm  用户特定的中断信息；将被回传给设备驱动的中断处理函数。使得设	      驱动能够将中断与特定I/O 请求关联起来lpm           定义用于特定 I/O 请求的通道路径。值为 0 会让 cio 使用 opmkey           用于I/O 的存储键（用于在存储!= 默认键的存储上操作时很有用）
flag          定义 I/O 处理要执行的动作
expires       jiffies 计的超时值。通用 I/O 层将在此之后终止正在运行的程序，
	      并以 ERR_PTR(-ETIMEDOUT) 作为 irb 调用中断处理函数============= =============================================================

可能flag 值有
========================= =============================================
DOIO_ALLOW_SUSPEND        通道程序可能进入挂起状DOIO_DENY_PREFETCH        不允CCW 预取；通常这意味着通道程序可能会被
			  修改
DOIO_SUPPRESS_INTER       不在中间状态时调用处理函数
========================= =============================================

```

  struct ccw1 {
	__u8  cmd_code;/* command code */
	__u8  flags;   /* flags, like IDA addressing, etc. */
	__u16 count;   /* byte count */
	__u32 cda;     /* data address */
  } __attribute__ ((packed,aligned(8)));

```
具有以下定义CCW 标志值：

=================== =========================
CCW_FLAG_DC         data chaining
CCW_FLAG_CC         command chaining
CCW_FLAG_SLI        suppress incorrect length
CCW_FLAG_SKIP       skip
CCW_FLAG_PCI        PCI
CCW_FLAG_IDA        indirect addressing
CCW_FLAG_SUSPEND    suspend
=================== =========================


通过 ccw_device_set_options()，设备驱动可以为设备指定以下选项
========================= ======================================
DOIO_EARLY_NOTIFICATION   允许早期中断通知
DOIO_REPORT_ALL           报告所有中断条========================= ======================================


ccw_device_start() 函数返回
======== ======================================================================
      0  成功完成或请求已成功发起
 -EBUSY  设备当前正在处理先前I/O 请求，或设备上有状态挂起-ENODEV  cdev 无效，设备未运行ccw_device 未上线======== ======================================================================

I/O 请求完成时，CDS 一级中断处理程序会先在 struct irb 中累积状态，然后
调用设备中断处理函数。intparm 字段将包含设备驱动与特定 I/O 请求关联的值如果识别到一个挂起的设备状态，intparm 将被设为 0（零）。这可能发生I/O
发起期间，也可能由告警状态通知延迟触发。无论如何，该状态与当前（最后一个）
I/O 请求无关。在延迟状态通知的情况下，不会呈现特殊的中断来表I/O 完成，因I/O 请求从未被启动，即使 ccw_device_start() 返回了成功完成
irb 可能包含一个错误值，设备驱动应当先检查这一点：

========== =================================================================
-ETIMEDOUT 通用 I/O 层在指定的超时值之后终止了请求
-EIO       通用 I/O 层因错误状态而终止了请求
========== =================================================================

如果 irb 中扩展状态字（esw）里的并sense 标志被置位，esw 中的字段 erw.scnt
描述了扩展控制字 irb->scsw.ecw[] 中可用的设备特定 sense 字节数。设备驱动自无需做设sensing
设备中断处理函数可以使用以下定义来查看编码在 sense 字节 0 中的主要单元检来源
======================= ====
SNS0_CMD_REJECT         0x80
SNS0_INTERVENTION_REQ   0x40
SNS0_BUS_OUT_CHECK      0x20
SNS0_EQUIPMENT_CHECK    0x10
SNS0_DATA_CHECK         0x08
SNS0_OVERRUN            0x04
SNS0_INCOMPL_DOMAIN     0x01
======================= ====

这些值中的多个可能会根据设备状态一同被置位。详情请参阅设备特定的文档
irb->scsw.cstat 字段提供（累积的）子通道状态：

========================= ============================
SCHN_STAT_PCI             program controlled interrupt
SCHN_STAT_INCORR_LEN      incorrect length
SCHN_STAT_PROG_CHECK      program check
SCHN_STAT_PROT_CHECK      protection check
SCHN_STAT_CHN_DATA_CHK    channel data check
SCHN_STAT_CHN_CTRL_CHK    channel control check
SCHN_STAT_INTF_CTRL_CHK   interface control check
SCHN_STAT_CHAIN_CHECK     chaining check
========================= ============================

irb->scsw.dstat 字段提供（累积的）设备状态：

===================== =================
DEV_STAT_ATTENTION    attention
DEV_STAT_STAT_MOD     status modifier
DEV_STAT_CU_END       control unit end
DEV_STAT_BUSY         busy
DEV_STAT_CHN_END      channel end
DEV_STAT_DEV_END      device end
DEV_STAT_UNIT_CHECK   unit check
DEV_STAT_UNIT_EXCEP   unit exception
===================== =================

有关各个标志含义的详情，请参ESA/390 操作原理手册
使用说明
ccw_device_start() 必须在关中断并持ccw 设备锁的情况下调用
设备驱动允许在其中断处理函数内部直接发出下一ccw_device_start() 调用。除需要调度一个非确定性的长时错误恢复过程或类似操作，否则不需要调bottom-halfI/O 处理期间，Linux/390 通用 I/O 设备驱动支持已经获取IRQ 锁，即处理函在调ccw_device_start() 时绝不能再次尝试获取它，否则会导致死锁！

如果设备驱动依赖某次 I/O 请求在其开始下一次之前完成，它可以通过NoOp I/O
命令 CCW_CMD_NOOP 链接到所提交CCW 链末尾来减少 I/O 处理开销。这会强Channel-End Device-End 状态通过单次中断一起呈现。不过应谨慎使用，因为这
意味着通道将保持忙状态，无法处理同一通道上其他设备的 I/O 请求。因此，例如命令绝不应使用此技术，因为结果无论如何都会通过单次中断呈现
为了最小化 I/O 开销，设备驱动应当仅在设备能够在 device-end 之前报告设备驱动
迫切需要的中间中断信息时，才使DOIO_REPORT_ALL。在这种情况下，所I/O
中断都会呈现给设备驱动，直到识别到最终状态
如果设备能够从异步呈现的 I/O 错误中恢复，它可以使DOIO_EARLY_NOTIFICATION
标志执行重叠 I/O。虽然某些设备总是通过单次中断channel-end device-end
一起报告，但其他设备会在通道准备好接收下一I/O 请求时呈现主状态（channel-end），
并在设备处数据传输完成时呈现次状态（device-end）
上述标志允许利用此特性，例如用于能在网络上处理丢失数据的通信设备，以实现增强
I/O 处理
除非通道子系统在任何时刻都呈现次状态中断，利用此特性将使重I/O 执行期间只向
设备驱动呈现主状态中断。当呈现一个无错误的次状态（告警状态）时，这表示自上次
次状态（最终）以来发出的所有重ccw_device_start() 请求都已成功完成
意图在通道命令字（CCW）上设置挂起标志的通道程序，必须以 DOIO_ALLOW_SUSPEND
选项启动 I/O 操作，否则挂起标志将导致通道程序检查错误。在通道程序进入挂起时，
通道子系统会生成一个中间中断
ccw_device_resume() - 恢复通道程序执行

如果设备驱动选择通过在某个特CCW 上设CCW 挂起标志来挂起当前通道程序执行，通道程序的执行便被挂起。为了恢复通道程序的执行，CIO 层提供了
ccw_device_resume() 例程
```

  int ccw_device_resume(struct ccw_device *cdev);

```
====  ================================================
cdev  请求其恢复操作的 ccw_device
====  ================================================

ccw_device_resume() 函数返回
=========   ==============================================
	0   挂起的通道程序已恢   -EBUSY   状态挂  -ENODEV   cdev 无效或未运行的子通道
  -EINVAL   恢复函数不适用
-ENOTCONN   没有待完成的 I/O 请求
=========   ==============================================

使用说明
请参ccw_device_start() 的使用说明以了解有关挂起通道程序的更多细节
ccw_device_halt() - 终止 I/O 请求处理

有时设备驱动可能需要一种停止长时运行通道程序处理的方法，或者设备可能需先发出一halt subchannel（HSCH）I/O 命令。为此提供了 ccw_device_halt() 命令
ccw_device_halt() 必须在关中断并持ccw 设备锁的情况下调用
```

  int ccw_device_halt(struct ccw_device *cdev,
		      unsigned long intparm);

```
=======  =====================================================
cdev    请求其终止操作的 ccw_device
intparm 中断参数；仅在没I/O 未完成时使用该值，否则返回I/O 请求关联intparm
=======  =====================================================

ccw_device_halt() 函数返回
=======  ==============================================================
      0  请求已成功发-EBUSY   设备当前忙，或状态挂起-ENODEV  cdev 无效-EINVAL  设备未运行或 ccw 设备未上线=======  ==============================================================

使用说明
设备驱动可以通过编写一个在末端通过通道内转移（TIC）命令（CCW_CMD_TIC）回环到
开头的通道程序，来编写一个永不结束的通道程序。通常网络设备驱动通过设置 PCI
CCW 标志（CCW_FLAG_PCI）来执行此操作。一旦该 CCW 被执行，就会生成一个程序控中断（PCI）。设备驱动随后可以执行适当的动作。在中断一个未完成的对网络设备的读
（无论带不带 PCI 标志）之前，需要先调用 ccw_device_halt() 来结束挂起的操作
```

  ccw_device_clear() - 终止 I/O 请求处理

```
为了终止子通道上的所I/O 处理，使clear subchannel（CSCH）命令。它可以通过
ccw_device_clear() 发出
ccw_device_clear() 必须在关中断并持ccw 设备锁的情况下调用
```

  int ccw_device_clear(struct ccw_device *cdev, unsigned long intparm);

```
======= ===============================================
cdev    ccw_device the clear operation is requested for
intparm interruption parameter (see ccw_device_halt())
======= ===============================================

ccw_device_clear() 函数返回
=======  ==============================================================
      0  请求已成功发-ENODEV  cdev 无效
-EINVAL  设备未运行或 ccw 设备未上线=======  ==============================================================

### 其他支持例程


本章描述Linux/390 设备驱动编程环境中使用的各种例程
get_ccwdev_lock()

获取设备特定锁的地址。随后用spin_lock() / spin_unlock() 调用
```

  __u8 ccw_device_get_path_mask(struct ccw_device *cdev);

```
获取 cdev 当前可用路径的掩码