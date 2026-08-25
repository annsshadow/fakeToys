
## 概述

Linux 内核包含多种代码，用于作为完开（enlightened）的客体运行Microsoft Hyper-V 虚拟机监控器上。Hyper-V 主要由一个裸机（bare-metal）虚拟机监控器以及在父分（parent partition，大致相当于 KVM QEMU）中运行的虚拟机管理服务组成。客VM 运行子分区（child partition）中。在本文档中，对 Hyper-V 的引用通常同时涵盖虚拟机监控器VMM 服务，而不区分某项功能由哪个组件提供
Hyper-V 运行x86/x64 arm64 架构上，两种架构上都支持 Linux 客体。除非另有说明，
Hyper-V 在这两种架构上的功能和行为基本一致
### Linux 客体Hyper-V 的通信

Linux 客体通过四种不同方式Hyper-V 通信
- 隐式陷入（Implicit traps）：x86/x64 arm64 架构所定义，某些客体操作会陷入  Hyper-V。Hyper-V 模拟该操作并将控制权返回给客体。这种行为对 Linux 内核通常是不可见的
- 显式超级调用（Explicit hypercalls）：Linux Hyper-V 进行显式的函数调用并传递参数  Hyper-V 执行所请求的操作并将控制权返回给调用者。参数通过处理器寄存器或在 Linux 客体
  Hyper-V 之间共享的内存中传递。在 x86/x64 上，超级调用使用 Hyper-V 特定的调用序列  arm64 上，超级调用使用 ARM 标准SMCCC 调用序列
- 合成寄存器访问（Synthetic register access）：Hyper-V 实现多种合成寄存器。在 x86/x64
  上，这些寄存器在客体中表现为 MSR，Linux 内核可以使用 x86/x64 架构定义的正常机制读  这些 MSR。在 arm64 上，这些合成寄存器必须使用显式的超级调用来访问
- VMBus：VMBus 是构建在其他 3 种机制之上的更高层软件构造。它Hyper-V 主机Linux
  客体之间的消息传递接口。它使用 Hyper-V 与客体之间共享的内存，以及各种信号机制
前三种通信机制记录`Hyper-V Top Level Functional Spec (TLFS)`_ 中。TLFS 描述Hyper-V
的一般功能，并提供了关于超级调用和合成寄存器的细节。TLFS 目前仅针x86/x64 架构编写
VMBus 没有文档。本文档提供VMBus 及其工作原理的高层概述，但其细节只能从代码中辨别
### 共享内存

Hyper-V Linux 之间通信的许多方面都基于共享内存。这种共享通常按如下方式完成：

- Linux 使用标准 Linux 机制从其物理地址空间分配内存
- Linux 告诉 Hyper-V 所分配内存的客体物理地址（GPA）。许多共享区域保持为 1 页，因此
  单个 GPA 就足够了。较大的共享区域需要一GPA 列表，这GPA 在客体物理地址空间  通常不需要连续。Hyper-V 如何被告GPA GPA 列表各有不同。在某些情况下，单个 GPA
  被写入一个合成寄存器。在其他情况下，GPA GPA 列表VMBus 消息中发送
- Hyper-V GPA 转换真实"的物理内存地址，并创建一个它可以用来访问该内存的虚拟映射
- Linux 之后可以通过告诉 Hyper-V 将共享的 GPA 设置为零，来撤销它先前建立的共享
Hyper-V 4 KB 的页大小运行。传达给 Hyper-V GPA 可以是页号的形式，并且总是描述 4 KB
的范围。由Linux 客体x86/x64 上的页大小也4 KB，因此从客体页到 Hyper-V 页的映射
1 1 的。在 arm64 上，Hyper-V 支持 arm64 架构定义4/16/64 KB 页的客体。如Linux
使用 16 64 KB 页，Linux 代码必须小心地只4 KB 页为单位Hyper-V 通信。HV_HYP_PAGE_SIZE
以及相关的宏用在Hyper-V 通信的代码中，以便它在所有配置下都能正确工作
TLFS 所述，Hyper-V Linux 客体之间共享的少数内存页覆盖"（overlay）页。对覆盖页，Linux 使用通常的方法分配客体内存，并告Hyper-V 所分配内存GPA。但 Hyper-V
随后会用它自己分配的一页替换掉该物理内存页，而原来的物理内存页在客体 VM 中不再可访问Linux 可以正常访问该内存，就像它是最初分配的那块内存一样覆盖"行为之所以可见，只是
因为页面内容（如 Linux 所见）Linux 最初建立共享、插入覆盖页时发生了改变。类似地如果 Linux 撤销共享，内容也会改变，此时 Hyper-V 移除覆盖页，Linux 最初分配的客体页再变得可见
Linux kdump 内核或任何其他内核执kexec 之前，应当撤销Hyper-V 共享的内存Hyper-V 可能在新内核将该页用于其他用途之后修改共享页或移除覆盖页，从而损坏新内核Hyper-V 不向客体 VM 提供单一全部设置"操作，因Linux 代码必须在执kexec 前逐个
撤销所有共享。参hv_kexec_handler() hv_crash_handler()。但崩溃/panic 路径在清理上
仍有漏洞，因为某些共享页是使用每 CPU 的合成寄存器设置的，并且没有机制为运panic 路径
CPU 之外的其CPU 撤销共享页
### CPU 管理

Hyper-V 没有从运行中VM 热添加或热移CPU 的能力。然而，Windows Server 2019 Hyper-V
及更早版本可能向客体提供 ACPI 表，指示CPU 数量多于 VM 中实际存在的 CPU。如常，Linux
将这些额外的 CPU 视为潜在的热添加 CPU，并如此报告，尽Hyper-V 实际上永远不会热添加它们Windows Server 2022 Hyper-V 开始，ACPI 表只反映 VM 中实际存在的 CPU，因Linux 不会
报告任何热添CPU
只要没有 VMBus 通道中断分配给该 CPU，就可以使用正常Linux 机制Linux 客体 CPU 下线关于如何VMBus 通道中断重新分配以允许将 CPU 下线的更多细节，请参VMBus 中断一节
### 32 位与 64 
x86/x64 上，Hyper-V 支持 32 位和 64 位客体，Linux 在任一版本下都能构建并运行。虽32
位版本预期可以工作，但它很少被使用，并且可能存在未检测到的回归
arm64 上，Hyper-V 仅支64 位客体
### 字节序（Endian-ness
Hyper-V 与客VM 之间的所有通信x86/x64 arm64 上都使用小端（Little-Endian）格式Hyper-V 不支arm64 上的大端格式，并Linux 代码在访问与 Hyper-V 共享的数据时也不使用
字节序宏
### 版本管理

当前Linux 内核能与早至 Windows Server 2012 Hyper-V 的旧版本 Hyper-V 正确配合工作。对
Windows Server 2008/2008 R2 中的原始 Hyper-V 版本上运行的支持已被移除
运行Hyper-V 上的 Linux 客体会在 dmesg 中输出其所运行Hyper-V 版本。该版本形式Windows 内部版本号，仅用于显示。Linux 代码不会在运行时测试这个版本号来确定可用的特性和
功能。Hyper-V 通过提供给客体的合成 MSR 中的标志指示特功能的可用性，客体代码测试这些
标志
VMBus 有它自己的协议版本，该版本在客体Hyper-V 的初VMBus 连接期间协商确定。该版本也会在启动期间输出到 dmesg。该版本号在代码中的少数地方被检查，以确定是否存在特定功能
此外，VMBus 上的每个合成设备也都有一个独立于 VMBus 协议版本的设备协议版本。这些合成设备的
设备驱动通常会协商设备协议版本，并可能测试该协议版本以确定是否存在特定的设备功能
### 代码打包

Hyper-V 相关的代码出现在 Linux 内核代码树中的三个主要区域：

1. drivers/hv

2. arch/x86/hyperv 鍜?arch/arm64/hyperv

3. 单独的设备驱动区域，例如 drivers/scsi、drivers/net、drivers/clocksource 等
少数杂项文件出现在其他地方。请参阅 MAINTAINERS 文件Hyper-V/Azure CORE AND DRIVERS""DRM DRIVER FOR HYPERV SYNTHETIC VIDEO DEVICE"下的完整列表
#1 #2 中的代码仅在设置CONFIG_HYPERV 时构建。类似地，大多数Hyper-V 相关的驱代码也仅在设置了 CONFIG_HYPERV 时构建
#1 #3 中与 Hyper-V 相关的大部分代码可以构建为模块2 中特定于架构的代码必须内（built-in）。此外，drivers/hv/hv_common.c 是跨架构通用的底层代码，必须内置