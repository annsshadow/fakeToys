## OpenCAPI（开放一致性加速器处理器接口，Open Coherent Accelerator Processor Interface）

OpenCAPI 是处理器与加速器之间的一种接口。它的目标是低延迟、高带宽。

该规范由 OpenCAPI 联盟制定，现在可以从 `Compute Express Link Consortium
<https://computeexpresslink.org/resource/opencapi-specification-archive/>`_ 获取。

它允许一个加速器（可以是 FPGA、ASIC 等）使用虚拟地址以一致性方式访问主机内存。一个 OpenCAPI 设备也可以承载自己的内存，该内存可以从主机访问。

在 Linux 中，OpenCAPI 被称为 'ocxl'，它是 'cxl'（用于 powerpc 的 IBM CAPI 接口的驱动）的开放式、与处理器无关的演进版本，之所以这样命名是为了避免与 ISDN CAPI 子系统混淆。

## 高层视图

OpenCAPI 定义了一个数据链路层（DL）和事务层（TL），实现在物理链路之上。任何实现了 DL 和 TL 的处理器或设备都可以开始共享内存。

```

  +-----------+                         +-------------+
  |           |                         |             |
  |           |                         | Accelerated |
  | Processor |                         |  Function   |
  |           |  +--------+             |    Unit     |  +--------+
  |           |--| Memory |             |    (AFU)    |--| Memory |
  |           |  +--------+             |             |  +--------+
  +-----------+                         +-------------+
       |                                       |
  +-----------+                         +-------------+
  |    TL     |                         |    TLX      |
  +-----------+                         +-------------+
       |                                       |
  +-----------+                         +-------------+
  |    DL     |                         |    DLX      |
  +-----------+                         +-------------+
       |                                       |
       |                   PHY                 |
       +---------------------------------------+

```
## 设备发现

OpenCAPI 依赖于在设备上实现的、类似 PCI 的配置空间。因此主机可以通过查询配置空间来发现 AFU。

Linux 中的 OpenCAPI 设备被当作 PCI 设备对待（有一些注意事项）。固件应当把硬件抽象得像一个 PCI 链路。大量现有的 PCI 基础设施被复用：设备被扫描，BAR 在标准 PCI 枚举期间被分配。因此像 'lspci' 这样的命令可以用来查看有哪些设备可用。

配置空间定义了可以在物理适配器上找到的 AFU，例如它的名称、它能处理多少个内存上下文、它的 MMIO 区域大小等。

## MMIO

OpenCAPI 为每个 AFU 定义了两个 MMIO 区域：

- 全局 MMIO 区域，包含与整个 AFU 相关的寄存器。
- 每进程 MMIO 区域，对于每个上下文具有固定大小。

## AFU 中断

OpenCAPI 包含让 AFU 向主机进程发送中断的可能性。这是通过事务层中定义的 'intrp_req' 完成的，它指定一个定义该中断的 64 位对象句柄。

驱动允许一个进程分配一个中断并获取其 64 位对象句柄，该句柄可以传递给 AFU。

## 字符设备

驱动为每个在物理设备上找到的 AFU 创建一个字符设备。一个物理设备可能有多个功能，每个功能可以有多个 AFU。不过在撰写本文时，仅使用导出单个 AFU 的设备测试过。

字符设备可以在 /dev/ocxl/ 下找到，命名为：
/dev/ocxl/<AFU name>.<location>.<index>

其中 <AFU name> 是最多 20 个字符的名称，如 AFU 配置空间中所示。
<location> 由驱动添加，当系统中存在多个相同 OpenCAPI 设备实例时，它有助于区分设备。
<index> 也是为了在设备携带多个相同 AFU 副本这种不太可能的情况下帮助区分 AFU。

## Sysfs 类

为表示 AFU 的设备添加了一个 ocxl 类。参见 /sys/class/ocxl。其布局在 Documentation/ABI/testing/sysfs-class-ocxl 中描述。

## 用户 API

### open

基于配置空间中找到的 AFU 定义，一个 AFU 可能支持与多个内存上下文协同工作，在这种情况下，相关联的字符设备可以被不同进程多次打开。

### ioctl

OCXL_IOCTL_ATTACH：

  把调用进程的内存上下文绑定到 AFU，以便 AFU 能够访问其内存。

OCXL_IOCTL_IRQ_ALLOC：

  分配一个 AFU 中断并返回一个标识符。

OCXL_IOCTL_IRQ_FREE：

  释放一个先前分配的 AFU 中断。

OCXL_IOCTL_IRQ_SET_FD：

  把一个事件 fd 关联到 AFU 中断，以便用户进程在 AFU 发送中断时收到通知。

OCXL_IOCTL_GET_METADATA：

  从卡上获取配置信息，例如 MMIO 区域大小、AFU 版本，以及当前上下文的 PASID。

OCXL_IOCTL_ENABLE_P9_WAIT：

  允许 AFU 唤醒正在执行 'wait' 的用户空间线程。向用户空间返回信息以允许它配置 AFU。注意，这仅在 POWER9 上可用。

OCXL_IOCTL_GET_FEATURES：

  报告哪些影响 OpenCAPI 的 CPU 特性可从用户空间使用。

### mmap

一个进程可以 mmap 每进程 MMIO 区域以与 AFU 交互。
