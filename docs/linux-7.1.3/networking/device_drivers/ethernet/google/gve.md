
## Linux kernel driver for Compute Engine Virtual Ethernet (gve):


## Supported Hardware

GVE 驱动绑定到某些 Compute Engine 虚拟机中虚拟以太网设备使用的一个 PCI 设备 id。

+--------------+----------+---------+
|Field         | Value    | Comments|
+==============+==========+=========+
|Vendor ID     | `0x1AE0` | Google  |
+--------------+----------+---------+
|Device ID     | `0x0042` |         |
+--------------+----------+---------+
|Sub-vendor ID | `0x1AE0` | Google  |
+--------------+----------+---------+
|Sub-device ID | `0x0058` |         |
+--------------+----------+---------+
|Revision ID   | `0x0`    |         |
+--------------+----------+---------+
|Device Class  | `0x200`  | Ethernet|
+--------------+----------+---------+

## PCI Bars

gVNIC PCI 设备暴露三个 32 位 memory BAR：
- Bar0 - 设备配置和状态寄存器。
- Bar1 - MSI-X 向量表
- Bar2 - IRQ、RX 和 TX 门铃（doorbell）

## Device Interactions

驱动通过以下方式与设备交互：
 - Registers
    - 一块 MMIO 寄存器
    - 详见 gve_register.h
 - Admin Queue
    - 见下面描述
 - Reset
    - 设备随时可以被重置
 - Interrupts
    - 见下面支持的下列中断
 - Transmit and Receive Queues
    - 见下面描述

### Descriptor Formats

GVE 支持两种描述符格式：GQI 和 DQO。这两种格式有完全不同的描述符，将在下面描述。

### Addressing Mode

GVE 支持两种寻址模式：QPL 和 RDA。
QPL（“queue-page-list”，队列页列表）模式通过一组预注册的页来传递数据。

对于 RDA（“raw DMA addressing”，原始 DMA 寻址）模式，页集合是动态的。因此，包缓冲区可以位于客户机内存的任何位置。

### Registers

所有寄存器都是 MMIO。

这些寄存器用于初始化和配置设备，以及响应管理中断查询设备状态。

### Endianness

- Admin Queue 消息和寄存器全都是大端（Big Endian）。
- GQI 描述符和数据路径寄存器是大端。
- DQO 描述符和数据路径寄存器是小端（Little Endian）。

### Admin Queue (AQ)

Admin Queue 是一个 PAGE_SIZE 大小的内存块，被视为 AQ 命令的数组，驱动用它向设备发出命令并建立资源。驱动和设备维护一个计数，记录已提交和已执行的命令数。要发出 AQ 命令，驱动必须执行以下操作（需适当的加锁）：

1) 把新命令复制到 AQ 数组中下一个可用的槽位
2) 把计数增加新命令的数量
3) 把计数写入 GVE_ADMIN_QUEUE_DOORBELL 寄存器
4) 轮询 ADMIN_QUEUE_EVENT_COUNTER 寄存器，直到它等于写入门铃的值，或直到超时。

设备会更新通过 ADMIN_QUEUE_EVENT_COUNTER 寄存器报告为已执行的每个 AQ 命令中的 status 字段。

### Device Resets

设备重置通过向 AQ PFN 寄存器写入 0x0 触发。这会导致设备释放驱动分配的所有资源，包括 AQ 本身。

### Interrupts

驱动支持以下中断：

#### Management Interrupt

管理中断由设备用来通知驱动去查看 GVE_DEVICE_STATUS 寄存器。

管理 irq 的处理程序只是把服务任务排入工作队列以检查寄存器并确认（ack）该 irq。

#### Notification Block Interrupts

通知块（notification block）中断用于通知驱动去轮询与该中断关联的队列。

这些 irq 的处理程序会调度该块的 napi 运行并轮询队列。

### GQI Traffic Queues

GQI 队列由一个描述符环和一个缓冲区组成，并被分配给一个通知块。

描述符环是大小为 2 的幂的环形缓冲区，由固定大小的描述符组成。它们使用一个位于 Bar2 的 __be32 门铃推进头指针。尾指针通过按顺序消费描述符并更新一个 __be32 计数器来推进。门铃和计数器都会溢出回零。

每个队列的缓冲区必须事先作为队列页列表向设备注册，包数据只能放在那些页中。

#### Transmit

gve 将发送环的缓冲区映射到一个 FIFO 中，并在发送到 NIC 之前把包复制到 FIFO 中。

#### Receive

接收环的缓冲区被放入一个与描述符环长度相同的数据环中，头、尾指针一起在环上推进。

### DQO Traffic Queues

- 每个 TX 和 RX 队列都被分配一个通知块。

- 向设备发送描述符的 TX 和 RX 缓冲区队列，使用 MMIO 门铃通知设备有新描述符。

- 从设备接收描述符的 RX 和 TX 完成队列，使用“代（generation）位”来判断设备何时填充了描述符。驱动用“当前代”初始化所有位。设备会用与当前代相反的“下一代”填充接收到的描述符。当环回绕时，当前/下一代被交换。

- 确保 RX 和 TX 完成队列不被溢出是驱动的责任。这可以通过限制向 HW 提交的描述符数量来实现。

- TX 包有一个 16 位的 completion_tag，RX 缓冲区有一个 16 位的 buffer_id。它们会分别在 TX 完成和 RX 队列上返回，让驱动知道哪个包/缓冲区已完成。

#### Transmit

包的缓冲区在传输前被 DMA 映射以供设备访问。在包成功传输后，缓冲区被解除映射。

#### Receive

驱动在 RX 缓冲区队列上向 HW 提交固定大小的缓冲区。在关联 RX 队列上接收到的包可能跨多个描述符。
