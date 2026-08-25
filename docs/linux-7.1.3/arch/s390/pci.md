
## S/390 PCI


Authors:
        - Pierre Morel
        - Niklas Schnelle

Copyright, IBM Corp. 2020


## 命令行参数与 debugfs 条目


### 命令行参

- nomio

  不使PCI 映射 I/O（MIO）指令
- norid

  忽略 RID 字段，并强制每个 PCI 功能使用一PCI 域
### debugfs 条目


S/390 调试功能（s390dbf）会生成视图，以如下形式sysfs 目录保存各种调试结果
 - /sys/kernel/debug/s390dbf/pci_*/

例如
  - /sys/kernel/debug/s390dbf/pci_msg/sprintf

    保存来自 PCI 事件处理的消息，例如机器检查处理和全局功能（如 UID 检查）的设置
  通过将一0 6 之间的数字通过管道写入 /sys/kernel/debug/s390dbf/pci_*/level  可以调整日志级别的高低。详情请参阅 Documentation/arch/s390/s390dbf.rst 中有  S/390 调试功能的文档
## Sysfs 条目


特定zPCI 功能的条目，以及保存 zPCI 信息的条目
- /sys/bus/pci/slots/XXXXXXXX:

  槽位条目使用 PCI 功能的函数标识符（FID）作为槽位名来建立。上面以 XXXXXXXX
  表示的格式是 8 位十六进制数字，0 填充并使用小写十六进制数字
  - /sys/bus/pci/slots/XXXXXXXX/power

  除了FID 用作槽位名称外，槽位目录还包含以s390 特定的槽位属性
  - uid:
    该功能可由本槽位配置的用户定义标识符（UID）。另见设备的相应属性
  当前支持某个虚拟功能的物理功能，在通过以下命令移除所有虚拟功能之前无法断电：
  echo 0 > /sys/bus/pci/devices/DDDD:BB:dd.f/sriov_numvf

- /sys/bus/pci/devices/DDDD:BB:dd.f/:

  - function_id:
    zPCI 函数标识符（FID）是一32 位十六进制值，用于唯一标识PCI 功能    除非 hypervisor 提供了虚FID（例如在 KVM 上），否则该标识符在整个机器
    范围内（即使跨不同分区）都是唯一的
  - function_handle:
    32 位十六进制值是一个用PCI 功能的底层标识符。注意，函数句柄可能    发生变化，并PCI 事件发生时以及启禁用 PCI 功能时变为无效
  - pchid:
    16 位十六进制值对 PCI 功能与机型相关的位置进行了编码
  - pfgid:
    PCI 功能ID；共享相同功能的功能使用同一个标识符。一PCI 组定义了中断    IOMMU、IOTLB DMA 的细节
  - vfn:
    虚拟功能编号，虚拟功能为 1 N，物理功能为 0
  - pft:
    PCI 功能类型是一s390 特定的类型属性。它指示PCI 规范中的 class/vendor/
    device 标识符更通用、更以用途为导向的类型。也就是说，具有相同 pft 值的 PCI
    功能可能由不同的硬件实现提供支持。同时，除未分类功能（pft 0x00）外，相    pft 值通常意味着相似的使用模型。同时，同一PCI 硬件设备在不同使用模    下可能以不同pft 值出现。例NETD NETH VF 可能由同一PCI 硬件
    设备实现，但NETD 中父物理功能由用户管理，而在 NETH 中由平台管理
    目前定义了以PFT 值：

    - 0x00 (UNC): Unclassified
    - 0x02 (ROCE): RoCE Express
    - 0x05 (ISM): Internal Shared Memory
    - 0x0a (ROC2): RoCE Express 2
    - 0x0b (NVMe): NVMe
    - 0x0c (NETH): Network Express hybrid
    - 0x0d (CNW): Cloud Network Adapter
    - 0x0f (NETD): Network Express direct

  - port:
    端口是一个十进制值，对应于该功能所连接到的物理端口。虚拟功能（VF）与其父
    物理功能（PF）共享端口。值为 0 表示该端口属性不适用于该 PCI 功能类型
  - uid:
    PCI 功能的用户定义标识符（UID）是一32 位十六进制值。它按实例定义，作为
    分区、KVM 客户机或 z/VM 客户机配置的一部分。如果启用了 UID 检查，平台会确    UID 在该实例内唯一，并且该实例不会看到两个具有相同 UID PCI 功能
    与这一保证无关，并且与函数 ID（FID）不同，UID 在同一机器的不同分区中可能
    相同。这使得在多个分区中创建PCI 配置UID 命名空间中可以保持一致
  - uid_is_unique:
    一0 1 的标志，指示用户定义标识符（UID）是否保证在Linux 实例内保    唯一。该平台特性称UID 检查
  - pfip/segmentX:
    段决定了功能的隔离程度。它们对应于通往该功能的物理路径。段之间的差异越大，
    功能之间的隔离程度越高
  - fidparm:
    包含一个由平台提供的、以十六进制表示的每 PCI 功能 8 位参数字段。该字段    含义取决PCI 功能类型。对NETH VF，0x01 表示该功能支持混杂模式
- /sys/firmware/clp/uid_checking:

  除了每个设备uid_is_unique 属性外，这还提供一个全局指示，表明是否启用了
  UID 检查。这使得用户即使在未配置任何 PCI 功能时也能检UID 检查状态
## 枚举与热插拔


PCI 地址由四部分组成：域（domain）、总线（bus）、设备（device）和功能（function），
形式如下：DDDD:BB:dd.f
- 对于平台未暴RID PCI 功能，使pci=norid 内核参数；或者是一种所谓的
  隔离虚拟功能（isolated Virtual Function），它具RID 信息，但其父物理功能
  并非同一 PCI 配置的一部分
  - 每个域只有一个功能
  - 如果 UID 检查开启，则域zPCI 功能UID 设置；否则域 ID 是动态生成的，并    在重启或热插拔之间不稳定
- 对于平台暴露 RID 且不是隔离虚拟功能的 PCI 功能
  - 每个域仍然只有一个总线
  - 每条总线上最多可256 PCI 功能
  - 同一拓扑内所有功能的地址的域部分，是该拓扑中 devfn 最低的配置 PCI 功能的域
  - 由支SR-IOV 的物理功能生成的虚拟功能，只有在启用 SR-IOV 后才可见