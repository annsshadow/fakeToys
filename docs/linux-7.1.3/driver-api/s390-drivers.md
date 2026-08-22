## 编写 s390 通道设备驱动


:作 Cornelia Huck

## 简

本文档描述可用于驱动基于 s390 的通道附接（channel-attached）I/O 设备的设备驱动的接口。这包括与硬件交互的接口以及与通用驱动核心（driver core）交互的接口。这些接口由 s390 通用 I/O 层提供
本文档假设读者熟悉与 s390 通道 I/O 架构相关的技术术语。关于该架构的描述，请参IBM 出版SA22-7832《z/Architecture: Principles of Operation》
虽然 s390 系统上的大多I/O 设备通常通过此处描述的通道 I/O 机制驱动，但也存在各种其他方法（diag 接口）。它们不在本文档讨论范围内
s390 通用 I/O 层也提供对某些并不严格视I/O 的设备的访问。它们在此一并考虑，尽管并非本文档重点
一些额外信息也可在内核源码Documentation/arch/s390/driver-model.rst 中找到
## css 总线


css 总线包含系统上可用的子通道（subchannel）。它们分为几类：

- 标准 I/O 子通道，供系统使用。它们在 ccw 总线上有一个子设备，如下文描述- 绑定vfio-ccw 驱动I/O 子通道。请参见 Documentation/arch/s390/vfio-ccw.rst- 消息子通道。目前不存在 Linux 驱动- CHSC 子通道（最多一个）。chsc 子通道驱动可用于发送异chsc 命令- eADM 子通道。用于与存储级内存（storage class memory）通信
## ccw 总线


ccw 总线通常包含 s390 系统可用的大多数设备。它以用于寻址其设备的基本命令结构——通道命令字（ccw）命名，ccw 总线包含所谓的通道附接设备。它们通过 css 总线上可见的 I/O 子通道寻址。然而，通道附接设备的设备驱动绝不会直接与子通道交互，而只能通过 ccw 总线上的 I/O 设备（即 ccw 设备）间接交互
### 通道附接设备I/O 函数


一些硬件结构已被转换为 C 结构，供通用 I/O 层和设备驱动使用。关于此处表示的硬件结构的更多信息，请查Principles of Operation
   :internal:

### ccw 设备


想要发起通道 I/O 的设备需要附接到 ccw 总线。与驱动核心的交互通过通用 I/O 层完成，后者提ccw 设备ccw 设备驱动的抽象
所有发起或终止通道 I/O 的函数都作用ccw 设备结构。设备驱动不得绕过这些函数，否则可能会发生奇怪的副作用
   :internal:

   :export:

   :export:

### 通道测量设施


通道测量（channel-measurement）设施提供了一种收集测量数据的手段，这些数据由通道子系统为每个通道附接设备提供
   :internal:

   :export:

## ccwgroup 总线


ccwgroup 总线只包含用户创建的人工设备。许多网络设备（qeth）实际上由若ccw 设备（如 qeth 的读、写和数据通道）组成。ccwgroup 总线提供了一种机制来创建一个元设备（meta-device），将这ccw 设备作为从设备包含在内，并可netdevice 关联
### ccw 组设

   :internal:

   :export:

## 通用接口


以下章节包含的接口不仅被处理 ccw 设备的驱动使用，也被各种其他 s390 硬件的驱动使用
### 适配器中

通用 I/O 层提供用于处理适配器中断和中断向量的辅助函数
   :export:
