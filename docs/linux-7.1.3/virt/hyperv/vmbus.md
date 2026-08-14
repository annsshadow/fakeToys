
## VMBus

VMBus 是 Hyper-V 提供给客户机 VM 的一个软件构造。它由一条控制路径以及
Hyper-V 呈现给客户机 VM 的合成设备所使用的一些公共设施组成。控制路径用于
向客户机 VM 提供合成设备，在某些情形下也用于撤销（rescind）这些设备。
公共设施包括在客户机 VM 中的设备驱动与作为 Hyper-V 一部分的合成设备实现之间
进行通信的软件通道，以及允许 Hyper-V 与客户机互相中断的信令原语。

在 Linux 中，VMBus 被建模为一条总线，在运行中的 Linux 客户机里表现为
预期的 `/sys/bus/vmbus` 入口。VMBus 驱动（drivers/hv/vmbus_drv.c）
与 Hyper-V 主机建立 VMBus 控制路径，然后把自己注册为一条 Linux 总线驱动。
它实现了用于在总线上添加和移除设备的标准总线函数。

Hyper-V 提供的大多数合成设备都有对应的 Linux 设备驱动。这些设备包括：

- SCSI 控制器
- NIC
- 图形帧缓冲
- 键盘
- 鼠标
- PCI 设备直通
- 心跳（Heartbeat）
- 时间同步（Time Sync）
- 关机（Shutdown）
- 内存气球（Memory balloon）
- 与 Hyper-V 的键值对（KVP）交换
- Hyper-V 在线备份（也称 VSS）

客户机 VM 可以拥有多个合成 SCSI 控制器、合成 NIC 和 PCI 直通设备的实例。
其他合成设备每个 VM 限制为单个实例。上面没有列出的是 Hyper-V 提供的少量
仅供 Windows 客户机使用、且 Linux 没有对应驱动的合成设备。

Hyper-V 在描述合成设备时使用了 "VSP" 和 "VSC" 这两个术语。"VSP" 指实现
某个特定合成设备的 Hyper-V 代码，而 "VSC" 指在客户机 VM 中该设备的驱动。
例如，合成 NIC 的 Linux 驱动被称为 "netvsc"，合成 SCSI 控制器的 Linux 驱动
是 "storvsc"。这些驱动包含像 "storvsc_connect_to_vsp" 这样命名的函数。

### VMBus 通道

一个合成设备的实例使用 VMBus 通道在 VSP 与 VSC 之间进行通信。通道是双向的，
用于传递消息。大多数合成设备使用单条通道，但合成 SCSI 控制器和合成 NIC
可能会使用多条通道以获得更高的性能和更强的并行度。

每个通道由两个环形缓冲区组成。这是大学数据结构教科书中的经典环形缓冲区。
如果读指针和写指针相等，环形缓冲区就被视为空，因此一个满的环形缓冲区总是
至少保留一个未使用的字节。"in" 环形缓冲区用于从 Hyper-V 主机发往客户机的
消息，"out" 环形缓冲区用于从客户机发往 Hyper-V 主机的消息。在 Linux 中，
"in" 和 "out" 的命名是从客户机一侧来看的。环形缓冲区是在客户机与主机之间
共享的内存，它们遵循标准范式：内存由客户机分配，构成环形缓冲区的 GPA 列表
被传递给主机。每个环形缓冲区由一个页头（4 Kbyte，含读写索引和一些控制标志）
以及其后的实际环形内存组成。环形缓冲区的大小由客户机中的 VSC 决定，并针对
每个合成设备而不同。构成环形的 GPA 列表通过 VMBus 控制路径作为 GPA 描述符
列表（GPADL）传递给 Hyper-V 主机。参见函数 vmbus_establish_gpadl()。

每个环形缓冲区被映射到连续的 Linux 内核虚拟空间，分为三部分：1）4 Kbyte 的
页头，2）构成环形本身的内存，3）构成环形本身的第二份映射。由于（2）和（3）
在内核虚拟空间中是连续的，负责在环形缓冲区中复制数据的代码无需关心环形缓冲区的
回绕（wrap-around）。一旦复制操作完成，读或写索引可能需要被重置以指回第一份
映射，但实际的数据复制不需要拆成两部分。这种方法还允许在环形中直接、轻松地处
理复杂的数据结构，而无需处理回绕。

在页大小大于 4 Kbyte 的 arm64 上，页头仍然必须以一个 4 Kbyte 的区域传递给
Hyper-V。但组成实际环形的内存必须与 PAGE_SIZE 对齐，且大小必须是 PAGE_SIZE
的整数倍，以便完成那份重复映射的技巧。因此页头的一部分未被使用，也不会传递给
Hyper-V。这种情况由 vmbus_establish_gpadl() 处理。

Hyper-V 对可以通过 GPADL 与主机共享的客户机内存总量施加了限制。该限制确保
一个恶意客户机无法强迫主机消耗过多的资源。对于 Windows Server 2019 及更高
版本，该限制约为 1280 Mbyte。对于 Windows Server 2019 之前的版本，该限制约为
384 Mbyte。

### VMBus 通道消息

在 VMBus 通道中发送的所有消息都有一个标准头部，包括消息长度、消息载荷的偏移、
一些标志以及一个 transactionID。头部之后的消息部分对每个 VSP/VSC 对都是
唯一的。

消息遵循两种模式之一：

- 单向（Unidirectional）：任一方发送一条消息，且不期望收到响应消息
- 请求/响应（Request/response）：一方（通常是客户机）发送一条消息并期望收到响应

transactionID（也称 "requestID"）用于匹配请求与响应。某些合成设备允许同时
有多个请求在途（in-flight），因此客户机在发送请求时指定一个 transactionID。
Hyper-V 在匹配的响应中回送相同的 transactionID。

在 VSP 与 VSC 之间传递的消息是控制消息。例如，来自 storvsc 驱动的消息可能是
"执行这条 SCSI 命令"。如果一条消息还意味着客户机与 Hyper-V 主机之间要进行
某些数据传输，待传输的实际数据可以嵌入到控制消息中，也可以指定为一个独立的
数据缓冲区，由 Hyper-V 主机作为 DMA 操作来访问。前一种情况用于数据量较小、
在环形缓冲区中复制数据的开销可忽略时。例如，从 Hyper-V 主机发往客户机的时间
同步消息就包含实际的时间值。当数据较大时，则使用一个独立的数据缓冲区。在这种
情况下，控制消息包含描述该数据缓冲区的 GPA 列表。例如，storvsc 驱动就用这种
方式指定进行磁盘 I/O 的数据缓冲区。

存在三个用于发送 VMBus 通道消息的函数：

1. vmbus_sendpacket()：纯控制消息以及带有内嵌数据的消息 —— 不含 GPA
2. vmbus_sendpacket_pagebuffer()：带有 GPA 列表、标识待传输数据的消息。
   每个 GPA 关联一个偏移和长度，因此可以定位客户机内存中多个不连续的区域。
3. vmbus_sendpacket_mpb_desc()：带有 GPA 列表、标识待传输数据的消息。
   一个偏移和长度关联一组 GPA。这些 GPA 必须描述一块单一的、逻辑上连续的
   客户机内存区域。

历史上，Linux 客户机信任 Hyper-V 会发送格式良好且有效的消息，合成设备的 Linux
驱动并没有充分校验消息。随着能够完全加密客户机内存、并允许客户机不信任虚拟机
监控器（hypervisor）的处理器技术（AMD SEV-SNP、Intel TDX）的出现，信任
Hyper-V 主机不再是一个有效的假设。VMBus 合成设备的驱动正在被更新，以充分校验
从与 Hyper-V 共享的内存（包括来自 VMBus 设备的消息）中读取的任何值。为便于
此类校验，客户机从 "in" 环形缓冲区读出的消息会被复制到一个不与 Hyper-V 共享
的临时缓冲区。校验就在这个临时缓冲区中进行，从而避免 Hyper-V 在消息被校验之后、
被使用之前恶意修改它的风险。

### 合成中断控制器（synic）

Hyper-V 为每个客户机 CPU 提供一个合成中断控制器，VMBus 用它进行主机-客户机
通信。虽然每个 synic 定义了 16 个合成中断（SINT），但 Linux 只使用其中 1 个
（VMBUS_MESSAGE_SINT）。所有与 Hyper-V 主机和客户机 CPU 之间通信相关的中断
都使用这个 SINT。

该 SINT 被映射到一个每-CPU 的体系结构中断（即 8 位的 x86/x64 中断向量，或
arm64 的 PPI INTID）。由于客户机中的每个 CPU 都有一个 synic 并可能收到 VMBus
中断，在 Linux 中最好将它们建模为每-CPU 中断。这个模型在 arm64 上工作良好，
会为 VMBUS_MESSAGE_SINT 分配一个每-CPU 的 Linux IRQ。该 IRQ 在 `/proc/interrupts`
中表现为一个标记为 "Hyper-V VMbus" 的 IRQ。由于 x86/x64 缺乏对每-CPU IRQ 的
支持，一个 x86 中断向量被静态分配（HYPERVISOR_CALLBACK_VECTOR）到所有 CPU，
并显式编码以调用 vmbus_isr()。在这种情况下没有 Linux IRQ，这些中断在
`/proc/interrupts` 的 "HYP" 行中汇总可见。

synic 提供了将体系结构中断解复用（demultiplex）为一个或多个逻辑中断、并将该
逻辑中断路由到 Linux 中正确的 VMBus 处理程序的手段。这个解复用由 vmbus_isr()
以及访问 synic 数据结构的相关函数完成。

synic 在 Linux 中既没有被建模为 irq chip 或 irq domain，这些被解复用的逻辑
中断也不是 Linux IRQ。因此，它们不会出现在 `/proc/interrupts` 或 `/proc/irq`
中。这些逻辑中断的 CPU 亲和性通过 `/sys/bus/vmbus` 下的一个入口来控制，如下
所述。

### VMBus 中断

VMBus 提供了一种机制，使得当客户机在环形缓冲区中排队了新消息时，可以中断主机。
主机期望客户机仅在 "out" 环形缓冲区从空变为非空时才发送中断。如果客户机在其他
时候发送中断，主机会认为这类中断是不必要的。如果某个客户机发送了过多不必要的
中断，主机可能会通过暂停该客户机数秒来限制它，以防止拒绝服务攻击。

类似地，当主机在 VMBus 控制路径上发送新消息，或者某个 VMBus 通道的 "in" 环形
缓冲区因主机插入新的 VMBus 通道消息而从空变为非空时，主机会通过 synic 中断
客户机。控制消息流和每个 VMBus 通道的 "in" 环形缓冲区是独立的逻辑中断，由
vmbus_isr() 进行解复用。它先通过调用 vmbus_chan_sched() 检查通道中断来解复用，
后者查看一个 synic 位图以确定哪些通道在此 CPU 上有待处理的中断。如果多个通道
在此 CPU 上有待处理中断，它们会被顺序处理。当所有通道中断都处理完毕后，
vmbus_isr() 检查并处理在 VMBus 控制路径上收到的任何消息。

一个 VMBus 通道将中断哪个客户机 CPU，是在通道创建时由客户机选定的，主机也会
被告知这个选择。VMBus 设备大致分为两类：

1. "慢速"设备，只需要一个 VMBus 通道。这类设备（如键盘、鼠标、心跳和时间同步）
   产生的中断相对较少。它们的 VMBus 通道都被分配去中断 VMBUS_CONNECT_CPU，即
   总是 CPU 0。

2. "高速"设备，可能会使用多个 VMBus 通道以获得更高的并行度和性能。这些设备
   包括合成 SCSI 控制器和合成 NIC。它们的 VMBus 通道中断被分配到在 VM 可用 CPU
   中分散开的 CPU 上，以便多个通道上的中断可以并行处理。

VMBus 通道中断到 CPU 的分配在 init_vp_index() 函数中完成。这个分配是在正常的
Linux 中断亲和性机制之外完成的，因此这些中断既不是 "unmanaged" 也不是
"managed" 中断。

一个 VMBus 通道将中断的 CPU 可以在
`/sys/bus/vmbus/devices/<deviceGUID>/ channels/<channelRelID>/cpu` 中看到。
在较新版本的 Hyper-V 上运行时，可以通过向这个 sysfs 入口写入一个新值来更改
CPU。由于 VMBus 通道中断不是 Linux IRQ，在 `/proc/interrupts` 或 `/proc/irq`
中没有与单个 VMBus 通道中断对应的入口。

Linux 客户机中一个在线的 CPU，如果上面分配有 VMBus 通道中断，就不能被离线。
从内核 v6.15 开始，任何此类中断都会在离线时自动重新分配到其他某个 CPU。那个
"其他" CPU 由实现自行选择，并非负载均衡或其他智能决定。如果该 CPU 再次上线，
先前分配给它的通道中断不会被移回。因此，在多个 CPU 被离线、或许又再次上线之后，
中断到 CPU 的映射可能会变得混乱且非最优。在这种情况下，必须手动重新建立最优的
分配。对于 v6.14 及更早的内核，必须首先按上述方法手动将任何冲突的通道中断重新
分配到另一个 CPU。然后，当没有通道中断分配给该 CPU 时，它才能被离线。

VMBus 通道中断处理代码被设计为即使中断到达的 CPU 不是分配给该通道的 CPU，也
能正确工作。具体来说，该代码不使用基于 CPU 的互斥来保证正确性。在正常操作中，
Hyper-V 会中断被分配的 CPU。但是，当分配给某个通道的 CPU 正通过 sysfs 更改时，
客户机并不知道 Hyper-V 何时会完成这一转换。即使 Hyper-V 开始中断新 CPU 之前
存在时间滞后，代码也必须正确工作。参见 target_cpu_store() 中的注释。

### VMBus 设备创建/删除

Hyper-V 和 Linux 客户机有一条独立的消息传递路径，用于合成设备的创建和删除。
这条路径不使用 VMBus 通道。参见 vmbus_post_msg() 和 vmbus_on_msg_dpc()。

第一步是客户机连接到通用的 Hyper-V VMBus 机制。作为建立此连接的一部分，客户机
和 Hyper-V 就它们将使用的 VMBus 协议版本达成一致。这种协商允许较新的 Linux
内核运行在较旧的 Hyper-V 版本上，反之亦然。

然后客户机告诉 Hyper-V "发送 offer（提供）"。Hyper-V 为 VM 被配置拥有的每个
合成设备向客户机发送一条 offer 消息。每种 VMBus 设备类型都有一个固定的 GUID，
称为 "class ID（类 ID）"，每个 VMBus 设备实例也由一个 GUID 标识。来自 Hyper-V
的 offer 消息包含两个 GUID，以在 VM 内唯一标识该设备。每个设备实例对应一条
offer 消息，因此一个拥有两个合成 NIC 的 VM 会收到两条带有 NIC 类 ID 的 offer
消息。offer 消息的顺序可能因启动而异，Linux 代码中不可假设它是一致的。offer
消息也可能在 Linux 初始启动很久之后才到达，因为 Hyper-V 支持向运行中的 VM 添加
设备（如合成 NIC）。一条新的 offer 消息由 vmbus_process_offer() 处理，它间接
调用 vmbus_add_channel_work()。

收到 offer 消息后，客户机根据类 ID 识别设备类型，并调用正确的驱动来建立该设备。
驱动/设备的匹配使用标准的 Linux 机制完成。

设备驱动的 probe 函数打开到对应 VSP 的主 VMBus 通道。它为客户机通道环形缓冲区
分配内存，并通过向主机提供环形缓冲区内存的 GPA 列表来与 Hyper-V 主机共享该环形
缓冲区。参见 vmbus_establish_gpadl()。

环形缓冲区建立后，设备驱动和 VSP 通过主通道交换建立消息。这些消息可能包括协商
将在 Linux VSC 与 Hyper-V 主机上的 VSP 之间使用的设备协议版本。建立消息也可能
包括创建额外的 VMBus 通道，这些通道被有些不太恰当地称为 "sub-channels（子通道）"，
因为一旦创建，它们在功能上就等同于主通道。

最后，设备驱动可以像任何设备驱动一样在 `/dev` 中创建入口。

Hyper-V 主机可以向客户机发送一条 "rescind（撤销）" 消息，以移除先前被提供的
设备。Linux 驱动必须随时处理这样的 rescind 消息。撤销一个设备会调用设备驱动的
"remove（移除）" 函数，以干净地关闭并移除该设备。一旦一个合成设备被撤销，
Hyper-V 和 Linux 都不会保留任何关于它先前存在的状态。这样的设备之后可能会被
重新添加，在这种情况下它被视为一个全新的设备。参见 vmbus_onoffer_rescind()。

对于某些设备，例如 KVP 设备，在关闭主通道时（很可能是解绑设备与其驱动的结果），
Hyper-V 会自动发送一条 rescind 消息。该 rescind 导致 Linux 移除该设备。但随后
Hyper-V 立即向客户机重新提供该设备，导致 Linux 中创建一个新的设备实例。对于其他
设备，例如合成 SCSI 和 NIC 设备，关闭主通道**不会**导致 Hyper-V 发送 rescind
消息。该设备继续存在于 Linux 的 VMBus 上，但没有任何驱动与之绑定。随后可以将
同一个驱动或一个新的驱动绑定到该设备的现有实例上。
