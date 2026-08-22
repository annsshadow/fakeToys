## Xen 的虚TPM 接口


Authors: Matthew Fioravante (JHUAPL), Daniel De Graaf (NSA)

本文档描述用Xen 的虚拟可信平台模块（vTPM）子系统。假定读者熟悉构建和安装 Xen、Linux，并TPM vTPM 概念有基本理解
### 简

这项工作的目标是向虚拟客户机操作系统（在 Xen 术语中即 DomU）提TPM 功能。这让程序能够与虚拟系统中的
TPM 交互，就像它们与物理系统上的 TPM 交互一样。每个客户机获得自己独有的、被仿真的、软件实现的 TPM。然而，
每个 vTPM 的秘密（密钥、NVRAM 等）由一vTPM 管理器域管理，该域将这些秘密密封（seal）到物理 TPM 上如果创建这些域（管理器、vTPM 和客户机）的过程是可信的，那vTPM 子系统将根植于硬TPM 的信任链延伸Xen 中的虚拟机。vTPM 的每个主要组件都实现为一个独立的域，提供由虚拟机监视器保证的安全隔离。vTPM 域在
mini-os 中实现，以减少内存和处理器开销
mini-os vTPM 子系统建立在 IBM Intel 公司先前完成vTPM 工作之上

### 设计概述


```

  +------------------+
  |    Linux DomU    | ...
  |       |  ^       |
  |       v  |       |
  |   xen-tpmfront   |
  +------------------+
          |  ^
          v  |
  +------------------+
  | mini-os/tpmback  |
  |       |  ^       |
  |       v  |       |
  |  vtpm-stubdom    | ...
  |       |  ^       |
  |       v  |       |
  | mini-os/tpmfront |
  +------------------+
          |  ^
          v  |
  +------------------+
  | mini-os/tpmback  |
  |       |  ^       |
  |       v  |       |
  | vtpmmgr-stubdom  |
  |       |  ^       |
  |       v  |       |
  | mini-os/tpm_tis  |
  +------------------+
          |  ^
          v  |
  +------------------+
  |   Hardware TPM   |
  +------------------+

```
- Linux DomU:
	       希望使用 vTPM 的、基Linux 的客户机。可能存在多个这样的客户机
- xen-tpmfront.ko:
		    Linux 内核虚拟 TPM 前端驱动。该驱动为基Linux DomU 提供 vTPM 访问
- mini-os/tpmback:
		    Mini-os TPM 后端驱动。Linux 前端驱动连接到此后端驱动，以促进
		    Linux DomU 与其 vTPM 之间的通信。vtpmmgr-stubdom 也使用此驱动		    vtpm-stubdom 通信
- vtpm-stubdom:
		 一个实vTPM mini-os 桩域（stub domain）。运行中vtpm-stubdom
		 实例与系统上的逻辑 vtpms 之间存在一一映射。vTPM 的平台配置寄存器（PCR）通常
		 全部初始化为零
- mini-os/tpmfront:
		     Mini-os TPM 前端驱动。vTPM mini-os vtpm-stubdom 使用此驱动与
		     vtpmmgr-stubdom 通信。此驱动也用于与 vTPM 域通信mini-os 域（pv-grub）中
- vtpmmgr-stubdom:
		一个实vTPM 管理器的 mini-os 域。只有一vTPM 管理器，并且它应该在整个
		机器的生命周期内运行。该域调节对系统上物TPM 的访问，并保护每vTPM 的持久状态
- mini-os/tpm_tis:
		    Mini-os TPM 1.2 TPM 接口规范（TIS）驱动。vtpmmgr-stubdom 使用此驱		    直接与硬TPM 对话。通信通过将硬件内存页映射vtpmmgr-stubdom 来实现
- Hardware TPM:
		焊接到主板上的物TPM

### Xen 的集

vTPM 驱动的支持在 Xen 4.3 中通过 libxl toolstack 加入 Xen。关于设vTPM vTPM 管理器桩域的细节请参Xen 文档（docs/misc/vtpm.txt）。一旦桩域运行起来，vTPM 设备的设置方式与域配置文件中的磁盘或
网络设备相同
为了使用诸如 IMA 这样需要在 initrd 之前加载 TPM 的特性，xen-tpmfront 驱动必须编译进内核。如果不使用
这类特性，该驱动可以编译为模块，并像往常一样被加载