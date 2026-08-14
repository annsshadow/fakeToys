## PARPORT 接口文档


:Time-stamp: <2000-02-24 13:30:20 twaugh>

这里描述以下函数：

```
  parport_register_driver
  parport_unregister_driver
  parport_enumerate
  parport_register_device
  parport_unregister_device
  parport_claim
  parport_claim_or_block
  parport_release
  parport_yield
  parport_yield_blocking
  parport_wait_peripheral
  parport_poll_peripheral
  parport_wait_event
  parport_negotiate
  parport_read
  parport_write
  parport_open
  parport_close
  parport_device_id
  parport_device_coords
  parport_find_class
  parport_find_device
  parport_set_timeout
```
端口函数（可被底层驱动覆盖）：

```
    port->ops->read_data
    port->ops->write_data
    port->ops->read_status
    port->ops->read_control
    port->ops->write_control
    port->ops->frob_control
    port->ops->enable_irq
    port->ops->disable_irq
    port->ops->data_forward
    port->ops->data_reverse

  EPP::

    port->ops->epp_write_data
    port->ops->epp_read_data
    port->ops->epp_write_addr
    port->ops->epp_read_addr

  ECP::

    port->ops->ecp_write_data
    port->ops->ecp_read_data
    port->ops->ecp_write_addr

  Other::

    port->ops->nibble_read_data
    port->ops->byte_read_data
    port->ops->compat_write_data
```
parport 子系统包含 `parport`（核心的端口共享代码），以及各种各样的底层驱动，它们
实际执行端口访问。每个底层驱动处理一种特定风格的端口（PC、Amiga 等）。

parport 面向设备驱动作者的接口可以分为全局函数和端口函数。

全局函数主要用于设备驱动与 parport 子系统之间的通信：获取可用端口列表、为独占使用
声明一个端口等等。它们还包括用于执行标准操作的 `generic` 函数，这些操作可在任何
支持 IEEE 1284 的架构上工作。

端口函数由底层驱动提供，尽管核心 parport 模块为某些例程提供了通用的 `defaults`。
端口函数可以分成三组：SPP、EPP 和 ECP。

SPP（标准并行端口）函数修改所谓的 `SPP` 寄存器：data、status 和 control。硬件未必
真的拥有完全那样的寄存器，但 PC 有，并且这个接口是仿照常见的 PC 实现建模的。其他
底层驱动可能能够模拟其中大部分功能。

EPP（增强并行端口）函数用于以 IEEE 1284 EPP 模式进行读写，而 ECP（扩展能力端口）函数
用于 IEEE 1284 ECP 模式。（那 BECP 呢？有人关心吗？）

用于 EPP 和/或 ECP 传输的硬件辅助可能可用，也可能不可用；如果可用，可能使用，也
可能不使用。如果没有使用硬件，传输将由软件驱动。为了应对那些只是勉强支持 IEEE 1284
的外设，提供了一个底层驱动特定的函数，用于调整 “fudge factors（微调因子）”。

## 全局函数


### parport_register_driver - 向 parport 注册一个设备驱动


##### 概要


```
	#include <linux/parport.h>

	struct parport_driver {
		const char *name;
		void (*attach) (struct parport *);
		void (*detach) (struct parport *);
		struct parport_driver *next;
	};
	int parport_register_driver (struct parport_driver *driver);
```
##### 描述


为了能在并行端口被检测到时收到通知，应该调用 parport_register_driver。你的驱动将
立即收到所有已经被检测到的端口的通知，并且在底层驱动被加载时，收到每个新端口的
通知。

一个 `struct parport_driver` 包含你的驱动的文本名称、一个指向用于处理新端口的函数的
指针，以及一个指向用于处理因底层驱动卸载而消失的端口的函数的指针。端口只有在未被
使用时（即上面没有注册任何设备）才会被分离。

传给 `struct parport *` 参数的可见部分如下：
```
	struct parport
	{
		struct parport *next; /* next parport in list */
		const char *name;     /* port's name */
		unsigned int modes;   /* bitfield of hardware modes */
		struct parport_device_info probe_info;
				/* IEEE1284 info */
		int number;           /* parport index */
		struct parport_operations *ops;
		...
	};
```
结构中还有其他成员，但不应该去触碰它们。

`modes` 成员总结了底层硬件的能力。它由可以位或组合在一起的若干标志组成：

  ============================= ===============================================
  PARPORT_MODE_PCSPP		IBM PC 寄存器可用，即作用于 data、control 和
				status 寄存器的函数可能是在直接写入硬件。
  PARPORT_MODE_TRISTATE		数据驱动器可以被关闭。这允许数据线被用于
				反向（外设到主机）传输。
  PARPORT_MODE_COMPAT		硬件可以辅助兼容模式（打印机）传输，即
				compat_write_block。
  PARPORT_MODE_EPP		硬件可以辅助 EPP 传输。
  PARPORT_MODE_ECP		硬件可以辅助 ECP 传输。
  PARPORT_MODE_DMA		硬件可以使用 DMA，因此你可能想要把 ISA 可 DMA
				的内存（即使用 kmalloc 的 GFP_DMA 标志分配的内存）
				传给底层驱动，以利用它。
  ============================= ===============================================

`modes` 中可能还有其他标志。

`modes` 的内容仅供参考。例如，如果硬件能够使用 DMA，并且 PARPORT_MODE_DMA 在 `modes`
中，这并不必然意味着在可能时会总是使用 DMA。类似地，能够辅助 ECP 传输的硬件也不必然
会被使用。

##### 返回值


成功时为零，否则为一个错误码。

##### 错误


无。（它可能失败吗？为什么要返回 int？）

##### 示例


```
	static void lp_attach (struct parport *port)
	{
		...
		private = kmalloc (...);
		dev[count++] = parport_register_device (...);
		...
	}

	static void lp_detach (struct parport *port)
	{
		...
	}

	static struct parport_driver lp_driver = {
		"lp",
		lp_attach,
		lp_detach,
		NULL /* always put NULL here */
	};

	int lp_init (void)
	{
		...
		if (parport_register_driver (&lp_driver)) {
			/* Failed; nothing we can do. */
			return -EIO;
		}
		...
	}
```
##### 另请参阅


parport_unregister_driver, parport_register_device, parport_enumerate



### parport_unregister_driver - 告诉 parport 忘掉这个驱动


##### 概要


```
	#include <linux/parport.h>

	struct parport_driver {
		const char *name;
		void (*attach) (struct parport *);
		void (*detach) (struct parport *);
		struct parport_driver *next;
	};
	void parport_unregister_driver (struct parport_driver *driver);
```
##### 描述


这告诉 parport 不要再向设备驱动通知新端口或端口消失。属于该驱动的已注册设备不会被
注销：必须对每个设备使用 parport_unregister_device。

##### 示例


```
	void cleanup_module (void)
	{
		...
		/* Stop notifications. */
		parport_unregister_driver (&lp_driver);

		/* Unregister devices. */
		for (i = 0; i < NUM_DEVS; i++)
			parport_unregister_device (dev[i]);
		...
	}
```
##### 另请参阅


parport_register_driver, parport_enumerate



### parport_enumerate - 获取并行端口列表（已废弃）


##### 概要


```
	#include <linux/parport.h>

	struct parport *parport_enumerate (void);
```
##### 描述


获取本机器有效并行端口列表中的第一个。可以使用返回的 `struct parport **` 中的
``struct parport **next` 元素找到后续的并行端口。如果 `next`` 为 NULL，则列表中
没有更多并行端口了。列表中的端口数量不会超过 PARPORT_MAX。

##### 返回值


一个描述本机器有效并行端口的 `struct parport *`，如果没有则为 NULL。

##### 错误


这个函数可以返回 NULL，表示没有可用的并行端口。

##### 示例


```
	int detect_device (void)
	{
		struct parport *port;

		for (port = parport_enumerate ();
		port != NULL;
		port = port->next) {
			/* Try to detect a device on the port... */
			...
		}
		}

		...
	}
```
##### 注意


parport_enumerate 已被废弃；应该使用 parport_register_driver 代替。

##### 另请参阅


parport_register_driver, parport_unregister_driver



### parport_register_device - 注册以使用一个端口


##### 概要


```
	#include <linux/parport.h>

	typedef int (*preempt_func) (void *handle);
	typedef void (*wakeup_func) (void *handle);
	typedef int (*irq_func) (int irq, void *handle, struct pt_regs *);

	struct pardevice *parport_register_device(struct parport *port,
						  const char *name,
						  preempt_func preempt,
						  wakeup_func wakeup,
						  irq_func irq,
						  int flags,
						  void *handle);
```
##### 描述


使用这个函数在并行端口（`port`）上注册你的设备驱动。一旦你这样做了，你将能够使用
parport_claim 和 parport_release 来使用该端口。

（`name`）参数是出现在 /proc 文件系统中的设备名称。该字符串必须在设备的整个生命周期
（直到调用 parport_unregister_device）内保持有效。

这个函数会向你的驱动注册三个回调：`preempt`、`wakeup` 和 `irq`。它们每一个都可以是
NULL，以表示你不想要该回调。

当 `preempt` 函数被调用时，是因为另一个驱动希望使用并行端口。`preempt` 函数如果返回
非零值，表示并行端口尚不能释放——如果返回零，该端口就丢失给了另一个驱动，并且在使用
之前必须重新声明该端口。

`wakeup` 函数在另一个驱动释放了端口、且还没有其他驱动声明它时被调用。你可以从 `wakeup`
函数内部声明并行端口（这种情况下声明保证会成功），或者如果你现在不需要也可以不声明。

如果在你的驱动已声明的并行端口上发生了中断，`irq` 函数将被调用。（在此写一些关于
共享中断的内容。）

`handle` 是一个指向驱动特定数据的指针，并被传给回调函数。

`flags` 可以是下列标志的位组合：

  ===================== =================================================
        Flag            Meaning
  ===================== =================================================
  PARPORT_DEV_EXCL	设备根本不能共享并行端口。仅在绝对必要时使用。
  ===================== =================================================

这些 typedef 实际上并未定义——它们只是为了让函数原型更具可读性而展示出来。

```
	struct pardevice {
		struct parport *port;	/* Associated port */
		void *private;		/* Device driver's 'handle' */
		...
	};
```
##### 返回值


一个 `struct pardevice *`：指向已注册并行端口设备的句柄，可用于 parport_claim、
parport_release 等。

##### 错误


返回值为 NULL 表示在该端口上注册设备时发生了问题。

##### 示例


```
	static int preempt (void *handle)
	{
		if (busy_right_now)
			return 1;

		must_reclaim_port = 1;
		return 0;
	}

	static void wakeup (void *handle)
	{
		struct toaster *private = handle;
		struct pardevice *dev = private->dev;
		if (!dev) return; /* avoid races */

		if (want_port)
			parport_claim (dev);
	}

	static int toaster_detect (struct toaster *private, struct parport *port)
	{
		private->dev = parport_register_device (port, "toaster", preempt,
							wakeup, NULL, 0,
							private);
		if (!private->dev)
			/* Couldn't register with parport. */
			return -EIO;

		must_reclaim_port = 0;
		busy_right_now = 1;
		parport_claim_or_block (private->dev);
		...
		/* Don't need the port while the toaster warms up. */
		busy_right_now = 0;
		...
		busy_right_now = 1;
		if (must_reclaim_port) {
			parport_claim_or_block (private->dev);
			must_reclaim_port = 0;
		}
		...
	}
```
##### 另请参阅


parport_unregister_device, parport_claim



### parport_unregister_device - 结束使用一个端口


SYNPOPSIS

```
	#include <linux/parport.h>

	void parport_unregister_device (struct pardevice *dev);
```
##### 描述


这个函数与 parport_register_device 相反。使用 parport_unregister_device 之后，`dev`
不再是一个有效的设备句柄。

你不应注销一个当前已被声明的设备，尽管如果你这样做了它会被自动释放。

##### 示例


```
	...
	kfree (dev->private); /* before we lose the pointer */
	parport_unregister_device (dev);
	...
```
##### 另请参阅


parport_unregister_driver

### parport_claim, parport_claim_or_block - 为一个设备声明并行端口


##### 概要


```
	#include <linux/parport.h>

	int parport_claim (struct pardevice *dev);
	int parport_claim_or_block (struct pardevice *dev);
```
##### 描述


这些函数尝试获取 `dev` 所注册的并行端口的控制权。`parport_claim` 不阻塞，但
`parport_claim_or_block` 可能会阻塞。（在此写一些关于可中断或不可中断阻塞的内容。）

你不应尝试声明一个你已经声明过的端口。

##### 返回值


返回值为零表示端口被成功声明，调用者现在拥有了该并行端口。

如果 `parport_claim_or_block` 在成功返回之前阻塞了，返回值为正值。

##### 错误


========== ==========================================================
  -EAGAIN  端口当前不可用，但再次尝试声明它可能会成功。
========== ==========================================================

##### 另请参阅


parport_release

### parport_release - 释放并行端口


##### 概要


```
	#include <linux/parport.h>

	void parport_release (struct pardevice *dev);
```
##### 描述


一旦一个并行端口设备被声明，就可以使用 `parport_release` 释放它。它不会失败，但你
不应释放一个你并不拥有的设备。

##### 示例


```
	static size_t write (struct pardevice *dev, const void *buf,
			size_t len)
	{
		...
		written = dev->port->ops->write_ecp_data (dev->port, buf,
							len);
		parport_release (dev);
		...
	}
```
##### 另请参阅


change_mode, parport_claim, parport_claim_or_block, parport_yield



### parport_yield, parport_yield_blocking - 临时释放一个并行端口


##### 概要


```
	#include <linux/parport.h>

	int parport_yield (struct pardevice *dev)
	int parport_yield_blocking (struct pardevice *dev);
```
##### 描述


当一个驱动拥有并行端口的控制权时，它可以允许另一个驱动临时 `借用` 它。`parport_yield`
不阻塞；`parport_yield_blocking` 可能会阻塞。

##### 返回值


返回值为零表示调用者仍然拥有该端口，且调用没有阻塞。

来自 `parport_yield_blocking` 的正返回值表示调用者仍然拥有该端口，且调用发生了阻塞。

返回值为 -EAGAIN 表示调用者不再拥有该端口，并且在使用前必须重新声明它。

##### 错误


========= ==========================================================
  -EAGAIN  并行端口的所有权被让出了。
========= ==========================================================

##### 另请参阅


parport_release



### parport_wait_peripheral - 等待状态线，最多 35ms


##### 概要


```
	#include <linux/parport.h>

	int parport_wait_peripheral (struct parport *port,
				     unsigned char mask,
				     unsigned char val);
```
##### 描述


等待 mask 中的状态线匹配 val 中的值。

##### 返回值


======== ==========================================================
 -EINTR  有信号挂起
      0  mask 中的状态线的值与 val 中的一致
      1  等待超时（已过 35ms）
======== ==========================================================

##### 另请参阅


parport_poll_peripheral



### parport_poll_peripheral - 等待状态线，以微秒计


##### 概要


```
	#include <linux/parport.h>

	int parport_poll_peripheral (struct parport *port,
				     unsigned char mask,
				     unsigned char val,
				     int usec);
```
##### 描述


等待 mask 中的状态线匹配 val 中的值。

##### 返回值


======== ==========================================================
 -EINTR  有信号挂起
      0  mask 中的状态线的值与 val 中的一致
      1  等待超时（已过 usec 微秒）
======== ==========================================================

##### 另请参阅


parport_wait_peripheral



### parport_wait_event - 等待端口上的事件


##### 概要


```
	#include <linux/parport.h>

	int parport_wait_event (struct parport *port, signed long timeout)
```
##### 描述


等待端口上的事件（例如中断）。超时以 jiffies 计。

##### 返回值


======= ==========================================================
      0  成功
     <0  错误（尽快退出）
     >0  超时
======= ==========================================================

### parport_negotiate - 执行 IEEE 1284 协商


##### 概要


```
	#include <linux/parport.h>

	int parport_negotiate (struct parport *, int mode);
```
##### 描述


执行 IEEE 1284 协商。

##### 返回值


======= ==========================================================
     0  握手成功；IEEE 1284 外设和模式可用
    -1  握手失败；外设不兼容（或不存在）
     1  握手成功；存在 IEEE 1284 外设但模式不可用
======= ==========================================================

##### 另请参阅


parport_read, parport_write



### parport_read - 从设备读取数据


##### 概要


```
	#include <linux/parport.h>

	ssize_t parport_read (struct parport *, void *buf, size_t len);
```
##### 描述


以当前 IEEE 1284 传输模式从设备读取数据。这仅对支持反向数据传输的模式有效。

##### 返回值


如果为负，则为错误码；否则为传输的字节数。

##### 另请参阅


parport_write, parport_negotiate



### parport_write - 向设备写入数据


##### 概要


```
	#include <linux/parport.h>

	ssize_t parport_write (struct parport *, const void *buf, size_t len);
```
##### 描述


以当前 IEEE 1284 传输模式向设备写入数据。这仅对支持正向数据传输的模式有效。

##### 返回值


如果为负，则为错误码；否则为传输的字节数。

##### 另请参阅


parport_read, parport_negotiate



### parport_open - 为特定设备号注册设备


##### 概要


```
	#include <linux/parport.h>

	struct pardevice *parport_open (int devnum, const char *name,
				        int (*pf) (void *),
					void (*kf) (void *),
					void (*irqf) (int, void *,
						      struct pt_regs *),
					int flags, void *handle);
```
##### 描述


这类似于 parport_register_device，但接受一个设备号而不是一个指向 struct parport 的
指针。

##### 返回值


参见 parport_register_device。如果没有与 devnum 关联的设备，返回 NULL。

##### 另请参阅


parport_register_device



### parport_close - 为特定设备号注销设备


##### 概要


```
	#include <linux/parport.h>

	void parport_close (struct pardevice *dev);
```
##### 描述


这是 parport_open 对应的 parport_unregister_device。

##### 另请参阅


parport_unregister_device, parport_open



### parport_device_id - 获取 IEEE 1284 设备 ID


##### 概要


```
	#include <linux/parport.h>

	ssize_t parport_device_id (int devnum, char *buffer, size_t len);
```
##### 描述


获取与给定设备关联的 IEEE 1284 设备 ID。

##### 返回值


如果为负，则为错误码；否则为包含设备 ID 的 buffer 的字节数。设备 ID 的格式如下：
```
	[length][ID]
```
前两个字节表示整个设备 ID 的包含性长度，且以大端字节序排列。ID 是一系列这样的
配对：
```
	key:value;
```
##### 注意


许多设备有格式不正确的 IEEE 1284 设备 ID。

##### 另请参阅


parport_find_class, parport_find_device



### parport_device_coords - 将设备号转换为设备坐标


##### 概要


```
	#include <linux/parport.h>

	int parport_device_coords (int devnum, int *parport, int *mux,
				   int *daisy);
```
##### 描述


在设备号（从零开始）与设备坐标（端口、多路复用器、菊花链地址）之间转换。

##### 返回值


成功时为零，此时坐标为 (`**parport`, `**mux`, `*daisy`)。

##### 另请参阅


parport_open, parport_device_id



### parport_find_class - 按类别查找设备


##### 概要


```
	#include <linux/parport.h>

	typedef enum {
		PARPORT_CLASS_LEGACY = 0,       /* Non-IEEE1284 device */
		PARPORT_CLASS_PRINTER,
		PARPORT_CLASS_MODEM,
		PARPORT_CLASS_NET,
		PARPORT_CLASS_HDC,              /* Hard disk controller */
		PARPORT_CLASS_PCMCIA,
		PARPORT_CLASS_MEDIA,            /* Multimedia device */
		PARPORT_CLASS_FDC,              /* Floppy disk controller */
		PARPORT_CLASS_PORTS,
		PARPORT_CLASS_SCANNER,
		PARPORT_CLASS_DIGCAM,
		PARPORT_CLASS_OTHER,            /* Anything else */
		PARPORT_CLASS_UNSPEC,           /* No CLS field in ID */
		PARPORT_CLASS_SCSIADAPTER
	} parport_device_class;

	int parport_find_class (parport_device_class cls, int from);
```
##### 描述


按类别查找设备。搜索从设备号 from+1 开始。

##### 返回值


该类别中下一个设备的设备号，如果不存在这样的设备则为 -1。

##### 注意


```
	int devnum = -1;
	while ((devnum = parport_find_class (PARPORT_CLASS_DIGCAM, devnum)) != -1) {
		struct pardevice *dev = parport_open (devnum, ...);
		...
	}
```
##### 另请参阅


parport_find_device, parport_open, parport_device_id



### parport_find_device - 按类别查找设备


##### 概要


```
	#include <linux/parport.h>

	int parport_find_device (const char *mfg, const char *mdl, int from);
```
##### 描述


按厂商和型号查找设备。搜索从设备号 from+1 开始。

##### 返回值


下一个匹配规格的设备的设备号，如果不存在这样的设备则为 -1。

##### 注意


```
	int devnum = -1;
	while ((devnum = parport_find_device ("IOMEGA", "ZIP+", devnum)) != -1) {
		struct pardevice *dev = parport_open (devnum, ...);
		...
	}
```
##### 另请参阅


parport_find_class, parport_open, parport_device_id



### parport_set_timeout - 设置不活动超时


##### 概要


```
	#include <linux/parport.h>

	long parport_set_timeout (struct pardevice *dev, long inactivity);
```
##### 描述


为已注册的设备设置不活动超时，以 jiffies 计。返回先前的超时值。

##### 返回值


先前的超时值，以 jiffies 计。

##### 注意


由于外设的延迟，某个端口的 port->ops 函数可能会耗时。在外设超过 `inactivity` 个
jiffies 没有响应之后，将发生超时，并且阻塞的函数将返回。

0 个 jiffies 的超时是一个特例：函数必须尽可能多地完成工作，而不阻塞或将硬件留在
未知状态。例如，如果端口操作是在中断处理程序内部执行的，则应该使用 0 个 jiffies
的超时。

一旦为已注册设备设置，超时将保持在所设置的值，直到再次被设置。

##### 另请参阅


port->ops->xxx_read/write_yyy



## 端口函数


port->ops 结构（struct parport_operations）中的函数由负责该端口的底层驱动提供。

### port->ops->read_data - 读取数据寄存器


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		unsigned char (*read_data) (struct parport *port);
		...
	};
```
##### 描述


如果 port->modes 包含 PARPORT_MODE_TRISTATE 标志，且 control 寄存器中的
PARPORT_CONTROL_DIRECTION 位被设置，则返回数据引脚上的值。如果 port->modes 包含
PARPORT_MODE_TRISTATE 标志，而 PARPORT_CONTROL_DIRECTION 位未被设置，则返回
值*可能*是写入数据寄存器的最后一个值。否则返回值是未定义的。

##### 另请参阅


write_data, read_status, write_control



### port->ops->write_data - 写入数据寄存器


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*write_data) (struct parport *port, unsigned char d);
		...
	};
```
##### 描述


写入数据寄存器。可能会有副作用（例如一个 STROBE 脉冲）。

##### 另请参阅


read_data, read_status, write_control



### port->ops->read_status - 读取状态寄存器


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		unsigned char (*read_status) (struct parport *port);
		...
	};
```
##### 描述


从状态寄存器读取。这是一个位掩码：

- PARPORT_STATUS_ERROR (打印机故障, "nFault")
- PARPORT_STATUS_SELECT (在线, "Select")
- PARPORT_STATUS_PAPEROUT (无纸, "PError")
- PARPORT_STATUS_ACK (握手, "nAck")
- PARPORT_STATUS_BUSY (忙, "Busy")

可能还有其他位被设置。

##### 另请参阅


read_data, write_data, write_control



### port->ops->read_control - 读取控制寄存器


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		unsigned char (*read_control) (struct parport *port);
		...
	};
```
##### 描述


返回写入控制寄存器的最后一个值（来自 write_control 或 frob_control）。不执行端口访问。

##### 另请参阅


read_data, write_data, read_status, write_control



### port->ops->write_control - 写入控制寄存器


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*write_control) (struct parport *port, unsigned char s);
		...
	};
```
##### 描述


```
				  _______
	- PARPORT_CONTROL_STROBE (nStrobe)
				  _______
	- PARPORT_CONTROL_AUTOFD (nAutoFd)
				_____
	- PARPORT_CONTROL_INIT (nInit)
				  _________
	- PARPORT_CONTROL_SELECT (nSelectIn)
```
##### 另请参阅


read_data, write_data, read_status, frob_control



### port->ops->frob_control - 写入控制寄存器位


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		unsigned char (*frob_control) (struct parport *port,
					unsigned char mask,
					unsigned char val);
		...
	};
```
##### 描述


这等价于从控制寄存器读取、掩掉 mask 中的位、与 val 中的位做异或，然后将结果写入
控制寄存器。

由于某些端口不允许从控制端口读取，会维护其内容的软件副本，因此 frob_control 实际上
只进行一次端口访问。

##### 另请参阅


read_data, write_data, read_status, write_control



### port->ops->enable_irq - 启用中断生成


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*enable_irq) (struct parport *port);
		...
	};
```
##### 描述


并行端口硬件被指示在适当时刻生成中断，尽管那些时刻是架构特定的。对于 PC 架构，中断
通常在 nAck 的上升沿生成。

##### 另请参阅


disable_irq



### port->ops->disable_irq - 禁用中断生成


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*disable_irq) (struct parport *port);
		...
	};
```
##### 描述


并行端口硬件被指示不要生成中断。中断本身并未被屏蔽。

##### 另请参阅


enable_irq



### port->ops->data_forward - 启用数据驱动器


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*data_forward) (struct parport *port);
		...
	};
```
##### 描述


启用数据线驱动器，用于 8 位主机到外设的通信。

##### 另请参阅


data_reverse



### port->ops->data_reverse - 将缓冲器置为三态


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		void (*data_reverse) (struct parport *port);
		...
	};
```
##### 描述


如果 port->modes 设置了 PARPORT_MODE_TRISTATE 位，将数据总线置于高阻抗状态。

##### 另请参阅


data_forward



### port->ops->epp_write_data - 写入 EPP 数据


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*epp_write_data) (struct parport *port, const void *buf,
					size_t len, int flags);
		...
	};
```
##### 描述


以 EPP 模式写入数据，并返回写入的字节数。

`flags` 参数可以是一个或多个下列标志的位或组合：

======================= =================================================
PARPORT_EPP_FAST	使用快速传输。某些芯片提供 16 位和 32 位寄存器。
			但是，如果一次传输超时，返回值可能不可靠。
======================= =================================================

##### 另请参阅


epp_read_data, epp_write_addr, epp_read_addr



### port->ops->epp_read_data - 读取 EPP 数据


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*epp_read_data) (struct parport *port, void *buf,
					size_t len, int flags);
		...
	};
```
##### 描述


以 EPP 模式读取数据，并返回读取的字节数。

`flags` 参数可以是一个或多个下列标志的位或组合：

======================= =================================================
PARPORT_EPP_FAST	使用快速传输。某些芯片提供 16 位和 32 位寄存器。
			但是，如果一次传输超时，返回值可能不可靠。
======================= =================================================

##### 另请参阅


epp_write_data, epp_write_addr, epp_read_addr



### port->ops->epp_write_addr - 写入 EPP 地址


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*epp_write_addr) (struct parport *port,
					const void *buf, size_t len, int flags);
		...
	};
```
##### 描述


写入 EPP 地址（每个 8 位），并返回写入的数量。

`flags` 参数可以是一个或多个下列标志的位或组合：

======================= =================================================
PARPORT_EPP_FAST	使用快速传输。某些芯片提供 16 位和 32 位寄存器。
			但是，如果一次传输超时，返回值可能不可靠。
======================= =================================================

（PARPORT_EPP_FAST 对这个函数有意义吗？）

##### 另请参阅


epp_write_data, epp_read_data, epp_read_addr



### port->ops->epp_read_addr - 读取 EPP 地址


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*epp_read_addr) (struct parport *port, void *buf,
					size_t len, int flags);
		...
	};
```
##### 描述


读取 EPP 地址（每个 8 位），并返回读取的数量。

`flags` 参数可以是一个或多个下列标志的位或组合：

======================= =================================================
PARPORT_EPP_FAST	使用快速传输。某些芯片提供 16 位和 32 位寄存器。
			但是，如果一次传输超时，返回值可能不可靠。
======================= =================================================

（PARPORT_EPP_FAST 对这个函数有意义吗？）

##### 另请参阅


epp_write_data, epp_read_data, epp_write_addr



### port->ops->ecp_write_data - 写入一块 ECP 数据


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*ecp_write_data) (struct parport *port,
					const void *buf, size_t len, int flags);
		...
	};
```
##### 描述


写入一块 ECP 数据。`flags` 参数被忽略。

##### 返回值


写入的字节数。

##### 另请参阅


ecp_read_data, ecp_write_addr



### port->ops->ecp_read_data - 读取一块 ECP 数据


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*ecp_read_data) (struct parport *port,
					void *buf, size_t len, int flags);
		...
	};
```
##### 描述


读取一块 ECP 数据。`flags` 参数被忽略。

##### 返回值


读取的字节数。注意：FIFO 中可能还有更多未读数据。有没有办法让 FIFO 暂停以防止这种
情况？

##### 另请参阅


ecp_write_block, ecp_write_addr



### port->ops->ecp_write_addr - 写入一块 ECP 地址


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*ecp_write_addr) (struct parport *port,
					const void *buf, size_t len, int flags);
		...
	};
```
##### 描述


写入一块 ECP 地址。`flags` 参数被忽略。

##### 返回值


写入的字节数。

##### 注意


这可能使用一个 FIFO，如果是这样，在 FIFO 清空之前不应返回。

##### 另请参阅


ecp_read_data, ecp_write_data



### port->ops->nibble_read_data - 以 nibble 模式读取一块数据


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*nibble_read_data) (struct parport *port,
					void *buf, size_t len, int flags);
		...
	};
```
##### 描述


以 nibble 模式读取一块数据。`flags` 参数被忽略。

##### 返回值


读取的完整字节数。

##### 另请参阅


byte_read_data, compat_write_data



### port->ops->byte_read_data - 以字节模式读取一块数据


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*byte_read_data) (struct parport *port,
					void *buf, size_t len, int flags);
		...
	};
```
##### 描述


以字节模式读取一块数据。`flags` 参数被忽略。

##### 返回值


读取的字节数。

##### 另请参阅


nibble_read_data, compat_write_data



### port->ops->compat_write_data - 以兼容模式写入一块数据


##### 概要


```
	#include <linux/parport.h>

	struct parport_operations {
		...
		size_t (*compat_write_data) (struct parport *port,
					const void *buf, size_t len, int flags);
		...
	};
```
##### 描述


以兼容模式写入一块数据。`flags` 参数被忽略。

##### 返回值


写入的字节数。

##### 另请参阅


nibble_read_data, byte_read_data
