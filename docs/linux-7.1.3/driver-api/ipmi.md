## Linux IPMI 驱动


:Author: Corey Minyard <minyard@mvista.com> / <minyard@acm.org>

IPMI（Intelligent Platform Management Interface，智能平台管理接口）是一种
用于控制监控系统的智能设备的标准。它支持系统中传感器的动态发现，以及
监视传感器并在传感器数值变化或超出某些边界时得到通知的能力。它还拥有
用于现场可更换单元（FRU）的标准化数据库以及一个看门狗定时器。

要使用它，你需要在系统中有一个到 IPMI 控制器的接口（称为基板管理控制器，
即 BMC），以及能够使用 IPMI 系统的管理软件。

本文档描述如何在 Linux 下使用 IPMI 驱动。如果你本身不熟悉 IPMI，请参阅
网站 https://www.intel.com/design/servers/ipmi/index.htm。IPMI 是个很大的
主题，我无法在这里全部覆盖！

### 配置

Linux IPMI 驱动是模块化的，这意味着你需要根据你的硬件选取若干项才能让它
正常工作。其中大多数位于 'Character Devices' 菜单下的 IPMI 菜单中。

无论如何，你必须选取 'IPMI top-level message handler' 才能使用 IPMI。在此
之外做什么取决于你的需求和硬件。

消息处理程序不提供任何用户级接口。内核代码（如看门狗）仍可以使用它。如果
你需要从用户空间访问，并且想通过设备驱动访问，则需要选取 'Device interface
for IPMI'。

驱动接口取决于你的硬件。如果系统正确提供了 IPMI 的 SMBIOS 信息，驱动会
检测到它并直接工作。如果你有一块带有标准接口的板子（通常这些要么是 "KCS"、
"SMIC" 或 "BT"，请查阅你的硬件手册），选择 'IPMI SI handler' 选项。也存在一个
用于直接 I2C 访问 IPMI 管理控制器的驱动。有些板子支持这种访问，但不知道它
是否能在每块板子上都工作。为此，选择 'IPMI SMBus handler'，但如果 SMBIOS/ACPI
信息错误或不存在，你要准备好自己去摸索它能否在你的系统上工作。同时启用这两个
通常是安全的，让驱动自动探测存在哪些接口。

你通常应该在系统上启用 ACPI，因为带有 IPMI 的系统可能拥有描述它们的 ACPI
表。

如果你有标准接口且板子制造商正确地完成了他们的工作，IPMI 控制器应当被
自动检测到（通过 ACPI 或 SMBIOS 表）并直接工作。遗憾的是，许多板子没有
这些信息。驱动会尝试标准默认值，但它们可能不工作。如果你遇到这种情况，
你需要阅读下面名为 'The SI Driver' 或 "The SMBus Driver" 的小节，了解如何
手动配置你的系统。

IPMI 定义了一个标准看门狗定时器。你可以用 'IPMI Watchdog Timer' 配置选项
来启用它。如果你把驱动编译进内核，那么通过内核命令行选项，可以让看门狗
定时器在初始化后立即启动。它还有很多其他选项，详见下文的 'Watchdog' 小节。
注意，你也可以让看门狗在关闭时继续运行（默认在关闭时禁用）。进入 'Watchdog
Cards' 菜单，启用 'Watchdog Timer Support'，并启用选项 'Disable watchdog
shutdown on close'。

IPMI 系统通常可以使用 IPMI 命令关机。选择 'IPMI Poweroff' 来做到这一点。
驱动会自动探测系统是否能被 IPMI 关机。即使你的系统不支持此选项，启用它也
是安全的。这在 ATCA 系统、Radisys CPI1 卡，以及任何支持标准机箱管理命令的
IPMI 系统上有效。

如果你希望驱动在 panic 时向事件日志写入一个事件，启用 'Generate a panic event
to all BMCs on a panic' 选项。如果你希望用 OEM 事件把整个 panic 字符串写入事件
日志，启用 'Generate OEM events containing the panic string' 选项。你也可以通过将
ipmi_msghandler 模块中名为 "panic_op" 的模块参数设为 "event" 或 "string" 来动态
启用它们。将该参数设为 "none" 则禁用此功能。

### 基本设计

Linux IPMI 驱动设计得非常模块化和灵活，你只需取用你需要的部分，就可以用多种
不同方式使用它。正因如此，它被拆分成了许多代码块。这些代码块（按模块名）是：

ipmi_msghandler - 这是 IPMI 系统的核心软件部分。它处理所有消息、消息时序和
响应。IPMI 用户接入这里，IPMI 物理接口（称为系统管理接口，即 SMI）也接入这里。
它提供 IPMI 的内核态接口，但不提供供应用程序进程使用的接口。

ipmi_devintf - 这为 IPMI 驱动提供一个用户态 IOCTL 接口，此设备的每次打开文件
都作为一个 IPMI 用户接入消息处理程序。

ipmi_si - 一个用于各种系统接口的驱动。它支持 KCS、SMIC 和 BT 接口。除非你有
SMBus 接口或自己的定制接口，否则你很可能需要使用它。

ipmi_ssif - 一个用于访问 SMBus 上 BMC 的驱动。它使用 I2C 内核驱动的 SMBus 接口
来通过 SMBus 收发 IPMI 消息。

ipmi_powernv - 一个用于访问 POWERNV 系统上 BMC 的驱动。

ipmi_watchdog - IPMI 要求系统具备一个非常强大的看门狗定时器。此驱动在 IPMI 消息
处理程序之上实现了标准的 Linux 看门狗定时器接口。

ipmi_poweroff - 某些系统支持通过 IPMI 命令关机。

bt-bmc - 这不是主驱动的一部分，而是一个用于访问 BT 接口的 BMC 侧接口的驱动。
它用于运行 Linux 的 BMC，以向主机提供接口。

这些都可以通过配置选项单独选取。

接口的很多文档在头文件中。IPMI 头文件有：

linux/ipmi.h - 包含 IPMI 的用户接口和 IOCTL 接口。

linux/ipmi_smi.h - 包含供系统管理接口（对接 IPMI 控制器的那些东西）使用的接口。

linux/ipmi_msgdefs.h - 基础 IPMI 消息传递的通用定义。


### 寻址

IPMI 寻址工作起来很像 IP 地址，你有一个覆盖层
```

  struct ipmi_addr
  {
	int   addr_type;
	short channel;
	char  data[IPMI_MAX_ADDR_SIZE];
  };

```
addr_type 决定了地址究竟是什么。驱动目前理解两种不同类型的地址。

```

  struct ipmi_system_interface_addr
  {
	int   addr_type;
	short channel;
  };

```
类型是 IPMI_SYSTEM_INTERFACE_ADDR_TYPE。这用于直接与当前卡上的 BMC 通信。channel
必须是 IPMI_BMC_CHANNEL。

发往 IPMB 总线、经由
```

  struct ipmi_ipmb_addr
  {
	int           addr_type;
	short         channel;
	unsigned char slave_addr;
	unsigned char lun;
  };

```
的 address 的消息。"channel" 这里通常为零，但有些设备支持多于一个通道，它对应
IPMI 规范中定义的通道。

还有一种 IPMB 直连地址，用于发送者直接位于 IPMB 总线上、无需经过 BMC 的情况。
你可以向
```

  struct ipmi_ipmb_direct_addr
  {
	int           addr_type;
	short         channel;
	unsigned char slave_addr;
	unsigned char rq_lun;
	unsigned char rs_lun;
  };

```
上的特定管理控制器（MC）发送消息。channel 始终为零。你也可以接收来自你已注册
处理并响应的其他 MC 的命令，因此你可以用它来实现总线上的一个管理控制器。

### 消息

```

  struct ipmi_msg
  {
	unsigned char netfn;
	unsigned char lun;
	unsigned char cmd;
	unsigned char *data;
	int           data_len;
  };

```
驱动负责添加/剥离头部信息。data 部分只是要发送的数据（不要把寻址信息放在这里）
或响应。注意，响应的完成码（completion code）是 "data" 中的第一项，它没有被剥离
出来，因为这就是规范中所有消息的定义方式（这也让偏移计数稍微容易一些 :-）。

从用户态使用 IOCTL 接口时，即使是在接收消息时，你也必须为 "data" 提供一块数据、
填充它，并将 data_len 设为该数据块的长度。否则驱动无处放置消息。

从内核态的消息处理程序上来的消息会以
```

  struct ipmi_recv_msg
  {
	struct list_head link;

	/* The type of message as defined in the "Receive Types"
           defines above. */
	int         recv_type;

	ipmi_user_t      *user;
	struct ipmi_addr addr;
	long             msgid;
	struct ipmi_msg  msg;

	/* Call this when done with the message.  It will presumably free
	   the message and do any other necessary cleanup. */
	void (*done)(struct ipmi_recv_msg *msg);

	/* Place-holder for the data, don't make any assumptions about
	   the size or existence of this, since it may change. */
	unsigned char   msg_data[IPMI_MAX_MSG_LENGTH];
  };

```
的形式到来。你应该查看接收类型并适当地处理消息。


### 上层接口（消息处理程序）

上层接口为用户提供对 IPMI 接口的一致视图。它允许多个 SMI 接口被寻址（因为某些
板子上实际上有多个 BMC 在它们之上），而用户无需关心它们下面是什么类型的 SMI。


##### 监视接口

当你的代码启动时，IPMI 驱动可能已检测到也可能尚未检测到 IPMI 设备是否存在。因此
你可能需要推迟你的设置，直到设备被检测到，或者你可能能够立即进行。为了处理这种
情况，并支持发现，你可以用 ipmi_smi_watcher_register() 注册一个 SMI 监视器（watcher），
以遍历接口并在它们出现和消失时通知你。


##### 创建用户

要使用消息处理程序，你必须先用 ipmi_create_user 创建一个用户。接口号指定你想
连接到的 SMI，并且你必须提供在数据到来时被调用的回调函数。这也允许你传入一块
数据 handler_data，它会在所有调用中回传给你。

一旦完成，调用 ipmi_destroy_user() 来移除该用户。

在用户态，打开设备会自动创建一个用户，关闭设备会自动销毁该用户。


##### 消息传递

要从内核态发送消息，ipmi_request_settime() 调用几乎完成了所有消息处理。大多数
参数不言自明。但它接受一个 "msgid" 参数。这 **不是** 消息的序列号。它只是一个
长整型值，在消息的响应返回时被回传。你可以随意使用它。

响应会在你传给 ipmi_create_user() 的 "handler" 的 ipmi_recv_hndl 字段所指向的
函数中返回。也记得查看接收类型。

在用户态，你填充一个 ipmi_req_t 结构并使用 IPMICTL_SEND_COMMAND ioctl。对于传入
的内容，你可以使用 select() 或 poll() 等待消息到来。但是，你不能使用 read() 获取
它们，你必须调用带 ipmi_recv_t 结构的 IPMICTL_RECEIVE_MSG 来真正获取消息。记住你
必须在 msg.data 字段提供一个指向数据块的指针，并且必须在 msg.data_len 字段填入
数据的大小。这给接收者一个实际放置消息的地方。

如果消息无法放入你提供的数据中，你将得到一个 EMSGSIZE 错误，并且驱动会把数据留在
接收队列中。如果你想获取它并让消息被截断，请使用 IPMICTL_RECEIVE_MSG_TRUNC ioctl。

当你在 IPMB 总线上发送一条命令（按 IPMI 规范由 netfn 的最低位定义）时，驱动会自动
为命令分配序列号并保存该命令。如果在 IPMI 规定的 5 秒内没有收到响应，它会自动生成
一个响应，表示命令超时。如果收到一个未经请求（unsolicited）的响应（例如它在 5 秒后
才到），该响应将被忽略。

在内核态，收到一条消息并处理好之后，你必须对它调用 ipmi_free_recv_msg()，否则会
泄漏消息。注意你绝不应该动消息的 "done" 字段，那是正确清理消息所必需的。

注意，发送时有一个 ipmi_request_supply_msgs() 调用，允许你提供 smi 和接收消息。这对
即使系统缓冲区耗尽也需要工作的代码很有用（例如看门狗定时器就用了这个）。你提供自己的
缓冲区和自己的释放例程。不过不建议正常使用，因为管理自己的缓冲区很棘手。


##### 事件与传入命令

驱动负责轮询 IPMI 事件和接收命令（命令是并非响应的消息，它们是 IPMB 总线上其他东西
发给你的命令）。要接收这些，你必须为它们注册，它们不会被自动发送给你。

要接收事件，你必须调用 ipmi_set_gets_events() 并将 "val" 设为非零。自启动以来驱动已
收到的任何事件都会立即投递给第一个为事件注册的用户。之后，如果多个用户注册了事件，
它们都会收到所有进来的事件。

对于接收命令，你必须为你想接收的命令逐个注册。调用 ipmi_register_for_cmd() 并为每个
你想接收的命令提供 netfn 和命令名。你还指定一个你希望从中接收命令的通道位掩码（如果
不在乎，可以使用 IPMI_CHAN_ALL 表示所有通道）。对于每个 netfn/cmd/channel 只能注册一个
用户，但不同的用户可以注册不同的命令，或者如果通道位掩码不重叠，则可以注册相同的命令。

要响应收到的命令，在返回的 netfn 中设置响应位，使用收到消息的地址，并使用你在收到
消息中得到的相同 msgid。

在用户态，提供了等价的 IOCTL 来执行这些功能。


### 下层（SMI）接口

如前所述，多个 SMI 接口可以注册到消息处理程序，它们各自在注册时获得一个接口号。
它们通常按注册顺序分配，不过如果一个 SMI 注销后另一个再注册，那就一切难说了。

ipmi_smi.h 定义了管理接口的接口，详见该文件。


### SI 驱动

SI 驱动允许在系统中配置 KCS、BT 和 SMIC 接口。它根据系统，通过多种不同的方法发现
接口。

你可以在模块加载行上最多指定四个接口，以及
```

  modprobe ipmi_si.o type=<type1>,<type2>....
       ports=<port1>,<port2>... addrs=<addr1>,<addr2>...
       irqs=<irq1>,<irq2>...
       regspacings=<sp1>,<sp2>,... regsizes=<size1>,<size2>,...
       regshifts=<shift1>,<shift2>,...
       slave_addrs=<addr1>,<addr2>,...
       force_kipmid=<enable1>,<enable2>,...
       kipmid_max_busy_us=<ustime1>,<ustime2>,...
       unload_when_empty=[0|1]
       trydmi=[0|1] tryacpi=[0|1]
       tryplatform=[0|1] trypci=[0|1]

```
除了 try... 项以外，这些每一项都是一个列表，第一项对应第一个接口，第二项对应第二个
接口，以此类推。

si_type 可以是 "kcs"、"smic" 或 "bt"。如果留空，默认为 "kcs"。

如果你为某个接口指定了非零的 addrs，驱动将把给定的内存地址用作设备地址。这会覆盖
si_ports。

如果你为某个接口指定了非零的 ports，驱动将把给定的 I/O 端口用作设备地址。

如果你为某个接口指定了非零的 irqs，驱动将尝试把给定的中断用于该设备。

其他 try... 项通过其对应名称禁用发现。这些默认全部启用，设为 0 以禁用它们。tryplatform
禁用 openfirmware。

接下来的三个参数与寄存器布局有关。接口使用的寄存器可能不出现在连续的位置，也可能
不在 8 位寄存器中。这些参数允许更精确地指定寄存器中数据的布局。

regspacings 参数给出连续寄存器起始地址之间的字节数。例如，如果 regspacing 设为 4，
起始地址为 0xca2，那么第二个寄存器的地址将是 0xca6。默认为 1。

regsizes 参数给出寄存器的大小（字节）。IPMI 使用的数据是 8 位宽，但它可能在更大的
寄存器内部。此参数允许指定读写类型。它可以是 1、2、4 或 8。默认为 1。

由于寄存器大小可能大于 32 位，IPMI 数据可能不在低 8 位。regshifts 参数给出为了得到
实际 IPMI 数据所需的移位量。

slave_addrs 指定本地 BMC 的 IPMI 地址。它通常是 0x20，驱动默认如此，但若不是，可以在
驱动启动时指定。

force_ipmid 参数强制启用（设为 1）或禁用（设为 0）内核 IPMI 守护进程。通常这由驱动自动
探测，但中断损坏的系统可能需要启用，或者不想用守护进程（不需要性能、不想占用 CPU）的
用户可以禁用它。

如果 unload_when_empty 设为 1，当驱动找不到任何接口或所有接口都失效时，驱动将被卸载。
默认为 1。设为 0 配合 hotmod 时有用，但显然只对模块有意义。

当编译进内核时，参数可以在
```

  ipmi_si.type=<type1>,<type2>...
       ipmi_si.ports=<port1>,<port2>... ipmi_si.addrs=<addr1>,<addr2>...
       ipmi_si.irqs=<irq1>,<irq2>...
       ipmi_si.regspacings=<sp1>,<sp2>,...
       ipmi_si.regsizes=<size1>,<size2>,...
       ipmi_si.regshifts=<shift1>,<shift2>,...
       ipmi_si.slave_addrs=<addr1>,<addr2>,...
       ipmi_si.force_kipmid=<enable1>,<enable2>,...
       ipmi_si.kipmid_max_busy_us=<ustime1>,<ustime2>,...

```
上指定。它与同名的模块参数工作方式相同。

如果你的 IPMI 接口不支持中断，并且是 KCS 或 SMIC 接口，IPMI 驱动会为该接口启动一个
内核线程以加快速度。这是一个低优先级内核线程，在 IPMI 操作进行期间不断轮询 IPMI 驱动。
force_kipmid 模块参数允许用户强制开启或关闭此线程。如果你强制关闭它且没有中断，驱动
将运行得非常慢。别怪我，这些接口太烂了。

遗憾的是，这个线程可能会占用大量 CPU，取决于接口的性能。这会浪费很多 CPU 并引发各种
检测空闲 CPU 和使用额外功耗的问题。为避免此问题，kipmid_max_busy_us 设置 kipmid 在
休眠一个 tick 之前自旋的最长时间（微秒）。这个值在性能和 CPU 浪费之间设定了一个平衡，
需要根据你的需求调整。也许有一天会加入自动调优，但这不是件简单的事，即便自动调优也
需要根据用户期望的性能来调整。

驱动支持接口的热添加和移除。这样，可以在内核启动并运行之后添加或移除接口。这是通过
/sys/modules/ipmi_si/parameters/hotmod 完成的，它是一个只写参数。你向该接口写入一个
字符串。该字符串
```

   <op1>[:op2[:op3...]]

```
```

   add|remove,kcs|bt|smic,mem|i/o,<address>[,<opt1>[,<opt2>[,...]]]

```
```

   rsp=<regspacing>
   rsi=<regsize>
   rsh=<regshift>
   irq=<irq>
   ipmb=<ipmb slave addr>

```
它们的含义与上文讨论的相同。注意你也可以在内核命令行上使用它，以获得更紧凑的指定
接口的格式。注意，当移除一个接口时，只有前三个参数（si 类型、地址类型和地址）用于
比较。任何选项在移除时都会被忽略。

### SMBus 驱动（SSIF）

SMBus 驱动允许在系统中配置最多 4 个 SMBus 设备。默认情况下，驱动只会在它在 DMI 或
ACPI 表中发现的东西上注册。你可以这样更改
```

  modprobe ipmi_ssif.o
	addr=<i2caddr1>[,<i2caddr2>[,...]]
	adapter=<adapter1>[,<adapter2>[...]]
	dbg=<flags1>,<flags2>...
	slave_addrs=<addr1>,<addr2>,...
	tryacpi=[0|1] trydmi=[0|1]
	[dbg_probe=1]
	alerts_broken

```
这些地址是普通的 I2C 地址。adapter 是适配器的字符串名称，如
/sys/bus/i2c/devices/i2c-<n>/name 所示。它 **不是** i2c-<n> 本身。此外，比较时忽略
空格，所以如果名称是 "This is an I2C chip"，你可以说 adapter_name=ThisisanI2cchip。
这是因为在内核参数中很难传入空格。

调试标志是对应每个发现的 BMC 的位标志，它们是：
IPMI 消息：1，驱动状态：2，时序：4，I2C 探测：8

tryxxx 参数可用于禁用从各种来源检测接口。

将 dbg_probe 设为 1 会启用对 SMBus 上 BMC 探测和检测过程的调试。

slave_addrs 指定本地 BMC 的 IPMI 地址。它通常是 0x20，驱动默认如此，但若不是，可以
在驱动启动时指定。

alerts_broken 不为 SSIF 启用 SMBus alert。否则 SMBus alert 会在受支持的硬件上被启用。

在 SMBus 上发现符合 IPMI 的 BMC 可能导致 I2C 总线上的设备失败。SMBus 驱动向 I2C 总线
以块写方式写入一条 "Get Device ID" IPMI 消息并等待响应。此动作对某些 I2C 设备是有害的。
强烈建议将已知 I2C 地址通过 smb_addr 参数提供给 SMBus 驱动，除非你有 DMI 或 ACPI 数据
告诉驱动该用什么。

当编译进内核时，地址可以在
```

  ipmb_ssif.addr=<i2caddr1>[,<i2caddr2>[...]]
	ipmi_ssif.adapter=<adapter1>[,<adapter2>[...]]
	ipmi_ssif.dbg=<flags1>[,<flags2>[...]]
	ipmi_ssif.dbg_probe=1
	ipmi_ssif.slave_addrs=<addr1>[,<addr2>[...]]
	ipmi_ssif.tryacpi=[0|1] ipmi_ssif.trydmi=[0|1]

```
上指定。这些选项与模块命令行上的相同。

I2C 驱动不支持非阻塞访问或轮询，因此如果没有特殊的内核补丁和驱动修改，此驱动无法做
IPMI panic 事件、在 panic 时延长看门狗或其他与 panic 相关的 IPMI 功能。你可以在 openipmi
网页上获取它们。

驱动通过 I2C sysfs 接口支持接口的热添加和移除。

### IPMI IPMB 驱动

此驱动用于支持位于 IPMB 总线上的系统；它让该接口看起来像一个普通的 IPMI 接口。向它
发送系统接口寻址的消息会导致消息发往系统上已注册的 BMC（默认在 IPMI 地址 0x20）。

它还允许你使用 ipmb 直连寻址直接寻址总线上的其他 MC。你可以接收来自总线上其他 MC 的
命令，它们会通过上文描述的普通接收命令机制处理。

```

  ipmi_ipmb.bmcaddr=<address to use for system interface addresses messages>
	ipmi_ipmb.retry_time_ms=<Time between retries on IPMB>
	ipmi_ipmb.max_retries=<Number of times to retry a message>

```
加载模块不会使驱动自动启动，除非有设备树信息来设置它。如果
```

  echo ipmi-ipmb <addr> > /sys/class/i2c-dev/i2c-<n>/device/new_device

```
注意你在这里给出的地址是 I2C 地址，不是 IPMI 地址。所以如果你希望你的 MC 地址是 0x60，
你在这里放 0x30。详见 I2C 驱动信息。

通过此接口向其他 IPMB 总线桥接命令不起作用。接收消息队列按设计未实现。BMC 上只有一个
接收消息队列，那是给主机驱动用的，而不是给 IPMB 总线上的东西用的。

BMC 可能有多个 IPMB 总线，你的设备位于哪条总线取决于系统的接线方式。你可以用
"ipmitool channel info <n>" 获取通道，其中 <n> 是通道，通道为 0-7，试试 IPMB 通道。

### 其他部分

### 获取与 IPMI 设备相关的详细信息

有些用户需要关于设备的更详细信息，比如地址从何而来，或 IPMI 接口的原始基础设备。
你可以使用 IPMI smi_watcher 在 IPMI 接口出现或消失时捕捉它们，并为了获取信息，你可以
使用函数
```

  struct ipmi_smi_info {
	enum ipmi_addr_src addr_src;
	struct device *dev;
	union {
		struct {
			void *acpi_handle;
		} acpi_info;
	} addr_info;
  };

```
目前仅返回 SI_ACPI 地址源的特殊信息。必要时可能会添加其他信息。

注意上述结构中包含了 dev 指针，假设 ipmi_smi_get_info 返回成功，你必须对 dev 指针调用
put_device。

### 看门狗

提供了一个实现了 Linux 标准看门狗定时器接口的看门狗定时器。它有三个模块参数可以
```

  modprobe ipmi_watchdog timeout=<t> pretimeout=<t> action=<action type>
      preaction=<preaction type> preop=<preop type> start_now=x
      nowayout=x ifnum_to_use=n panic_wdt_timeout=<t>

```
ifnum_to_use 指定看门狗定时器应使用哪个接口。默认为 -1，表示选取第一个注册的接口。

timeout 是到动作发生的秒数，pretimeout 是在重置之前多少秒发生预超时 panic（如果
pretimeout 为零，则不会启用 pretimeout）。注意 pretimeout 是最终超时之前的时间。因此如果
timeout 是 50 秒、pretimeout 是 10 秒，那么 pretimeout 将在 40 秒时发生（超时前 10 秒）。
panic_wdt_timeout 是在内核 panic 时设置的 timeout 值，以便让诸如 kdump 之类的动作在 panic
期间发生。

action 可以是 "reset"、"power_cycle" 或 "power_off"，指定定时器超时时做什么，默认为
"reset"。

preaction 可以是 "pre_smi"（通过 SMI 接口指示）、"pre_int"（通过 SMI 带中断指示），或
"pre_nmi"（preaction 上的 NMI）。这就是驱动被告知 pretimeout 的方式。

preop 可以设为 "preop_none"（pretimeout 时不操作）、"preop_panic"（将预操作设为 panic），
或 "preop_give_data"（在 pretimeout 发生时提供可从看门狗设备读取的数据）。"pre_nmi" 设置
**不能** 与 "preop_give_data" 一起使用，因为你无法从 NMI 做数据操作。

当 preop 设为 "preop_give_data" 时，在 pretimeout 发生时设备会有一个字节就绪可供读取。
select 和 fasync 在设备上也有效。

如果 start_now 设为 1，看门狗定时器将在驱动加载后立即开始运行。

如果 nowayout 设为 1，看门狗定时器在关闭看门狗设备时不会停止。如果启用了
CONFIG_WATCHDOG_NOWAYOUT 选项，nowayout 的默认值为真，否则为假。

当编译进内核时，内核命令行可用
```

  ipmi_watchdog.timeout=<t> ipmi_watchdog.pretimeout=<t>
	ipmi_watchdog.action=<action type>
	ipmi_watchdog.preaction=<preaction type>
	ipmi_watchdog.preop=<preop type>
	ipmi_watchdog.start_now=x
	ipmi_watchdog.nowayout=x
	ipmi_watchdog.panic_wdt_timeout=<t>

```
选项与模块参数选项相同。

看门狗在收到预动作时会 panic 并启动一个 120 秒的重置超时。在 panic 或重启期间，如果看门狗
正在运行，它会启动一个 120 秒定时器以确保重启发生。

注意，如果你对看门狗使用 NMI preaction，你 **绝不能** 使用 nmi 看门狗。没有合理的方法判断
NMI 是否来自 IPMI 控制器，因此必须假设如果它收到一个本应未被处理的 NMI，它必定来自 IPMI，
并会立即 panic。

一旦你打开了看门狗定时器，你必须向设备写入一个 'V' 字符来关闭它，否则定时器不会停止。这对
驱动是一个新的语义，但让它和 Linux 中其他看门狗驱动保持一致。


### Panic 超时

OpenIPMI 驱动支持在发生 panic 时将半定制和自定义事件放入系统事件日志的能力。如果你启用
'Generate a panic event to all BMCs on a panic' 选项，你会在 panic 时得到一个标准 IPMI 事件
格式的事件。如果你启用 'Generate OEM events containing the panic string' 选项，你还会得到
一批持有 panic 字符串的 OEM 事件。


事件的字段设置如下：

- Generator ID: 0x21（内核）
- EvM Rev: 0x03（此事件以 IPMI 1.0 格式格式化）
- Sensor Type: 0x20（OS critical stop sensor）
- Sensor #: panic 字符串的第一个字节（若无 panic 字符串则为 0）
- Event Dir | Event Type: 0x6f（Assertion，sensor-specific event info）
- Event Data 1: 0xa1（Runtime stop in OEM bytes 2 and 3）
- Event data 2: panic 字符串的第二个字节
- Event data 3: panic 字符串的第三个字节

详见 IPMI 规范了解事件布局的细节。此事件总是发往本地管理控制器。它会负责把消息路由到
正确的地方

其他 OEM 事件具有以下格式：

- Record ID（字节 0-1）：由 SEL 设置。
- Record type（字节 2）：0xf0（OEM non-timestamped）
- byte 3: 保存 panic 的卡的从地址（slave address）
- byte 4: 一个序列号（从零开始）
  其余字节（11 字节）是 panic 字符串。如果 panic 字符串超过 11 字节，将发送多条消息，
  序列号递增。

因为你无法使用标准接口发送 OEM 事件，此功能会尝试找一个 SEL 并把事件加进去。它会首先
查询本地管理控制器的能力。如果它有一个 SEL，那么它们会被存储在本地管理控制器的 SEL 中。
如果没有，且本地管理控制器是一个事件生成器，则会查询本地管理控制器的事件接收者，并将
事件发往该设备上的 SEL。否则，事件无处可去，因为没有地方发给它们。


### 关机

如果选择了关机能力，IPMI 驱动会向标准关机函数指针安装一个关机函数。这在 ipmi_poweroff
模块中。当系统请求断电时，它会发送正确的 IPMI 命令来完成。这在多个平台上受支持。

有一个名为 "poweroff_powercycle" 的模块参数，可以为零（执行断电）或非零（执行电源循环，
即先给系统断电，然后在几秒内重新上电）。在内核命令行上设置 ipmi_poweroff.poweroff_control=x
会做同样的事。该参数也可通过 proc 文件系统在 /proc/sys/dev/ipmi/poweroff_powercycle 中获取。
注意，如果系统不支持电源循环，它总是会执行断电。

"ifnum_to_use" 参数指定关机代码应使用哪个接口。默认为 -1，表示选取第一个注册的接口。

注意，如果你启用了 ACPI，系统会优先使用 ACPI 关机。
