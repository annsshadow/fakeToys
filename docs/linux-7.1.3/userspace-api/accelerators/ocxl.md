## OpenCAPI（开放一致性加速器处理器接口，Open Coherent Accelerator Processor Interface
OpenCAPI 是处理器与加速器之间的一种接口。它的目标是低延迟、高带宽
该规范由 OpenCAPI 联盟制定，现在可以从 `Compute Express Link Consortium
<https://computeexpresslink.org/resource/opencapi-specification-archive/>`_ 获取
它允许一个加速器（可以是 FPGA、ASIC 等）使用虚拟地址以一致性方式访问主机内存。一OpenCAPI 设备也可以承载自己的内存，该内存可以从主机访问
Linux 中，OpenCAPI 被称'ocxl'，它'cxl'（用powerpc IBM CAPI 接口的驱动）的开放式、与处理器无关的演进版本，之所以这样命名是为了避免ISDN CAPI 子系统混淆
## 高层视图

OpenCAPI 定义了一个数据链路层（DL）和事务层（TL），实现在物理链路之上。任何实现了 DL TL 的处理器或设备都可以开始共享内存
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

OpenCAPI 依赖于在设备上实现的、类PCI 的配置空间。因此主机可以通过查询配置空间来发AFU
Linux 中的 OpenCAPI 设备被当PCI 设备对待（有一些注意事项）。固件应当把硬件抽象得像一PCI 链路。大量现有的 PCI 基础设施被复用：设备被扫描，BAR 在标PCI 枚举期间被分配。因此像 'lspci' 这样的命令可以用来查看有哪些设备可用
配置空间定义了可以在物理适配器上找到AFU，例如它的名称、它能处理多少个内存上下文、它MMIO 区域大小等
## MMIO

OpenCAPI 为每AFU 定义了两MMIO 区域
- 全局 MMIO 区域，包含与整个 AFU 相关的寄存器- 每进MMIO 区域，对于每个上下文具有固定大小
## AFU 中断

OpenCAPI 包含AFU 向主机进程发送中断的可能性。这是通过事务层中定义'intrp_req' 完成的，它指定一个定义该中断64 位对象句柄
驱动允许一个进程分配一个中断并获取64 位对象句柄，该句柄可以传递给 AFU
## 字符设备

驱动为每个在物理设备上找到的 AFU 创建一个字符设备。一个物理设备可能有多个功能，每个功能可以有多个 AFU。不过在撰写本文时，仅使用导出单AFU 的设备测试过
字符设备可以/dev/ocxl/ 下找到，命名为：
/dev/ocxl/<AFU name>.<location>.<index>

其中 <AFU name> 是最20 个字符的名称，如 AFU 配置空间中所示<location> 由驱动添加，当系统中存在多个相同 OpenCAPI 设备实例时，它有助于区分设备<index> 也是为了在设备携带多个相AFU 副本这种不太可能的情况下帮助区分 AFU
## Sysfs 绫。
为表AFU 的设备添加了一ocxl 类。参/sys/class/ocxl。其布局Documentation/ABI/testing/sysfs-class-ocxl 中描述
## 用户 API

### open

基于配置空间中找到的 AFU 定义，一AFU 可能支持与多个内存上下文协同工作，在这种情况下，相关联的字符设备可以被不同进程多次打开
### ioctl

OCXL_IOCTL_ATTACH锛。
  把调用进程的内存上下文绑定到 AFU，以AFU 能够访问其内存
OCXL_IOCTL_IRQ_ALLOC锛。
  分配一AFU 中断并返回一个标识符
OCXL_IOCTL_IRQ_FREE锛。
  释放一个先前分配的 AFU 中断
OCXL_IOCTL_IRQ_SET_FD锛。
  把一个事fd 关联AFU 中断，以便用户进程在 AFU 发送中断时收到通知
OCXL_IOCTL_GET_METADATA锛。
  从卡上获取配置信息，例如 MMIO 区域大小、AFU 版本，以及当前上下文PASID
OCXL_IOCTL_ENABLE_P9_WAIT锛。
  允许 AFU 唤醒正在执行 'wait' 的用户空间线程。向用户空间返回信息以允许它配置 AFU。注意，这仅POWER9 上可用
OCXL_IOCTL_GET_FEATURES锛。
  报告哪些影响 OpenCAPI CPU 特性可从用户空间使用
### mmap

一个进程可mmap 每进MMIO 区域以与 AFU 交互