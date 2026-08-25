


## SysKonnect driver - SKFP


|copy| Copyright 1998-2000 SysKonnect,

skfp.txt created 11-May-2000

Readme File for skfp.o v2.06



   (1) OVERVIEW
   (2) SUPPORTED ADAPTERS
   (3) GENERAL INFORMATION
   (4) INSTALLATION
   (5) INCLUSION OF THE ADAPTER IN SYSTEM START
   (6) TROUBLESHOOTING
   (7) FUNCTION OF THE ADAPTER LEDS
   (8) HISTORY



## 1. Overview


README 介绍如何在你的网络适配器上使用 Linux 驱动 'skfp'
2 章：列出本驱动支持的所有网络适配器
绗?3 绔狅細
	   提供一些通用信息
4 章：描述常见问题及其解决方案
5 章：展示适配LED 功能的变更
6 章：开发历史

## 2. Supported adapters


网络驱动 'skfp' 支持以下网络适配器：
SysKonnect 适配器：

  - SK-5521 (SK-NET FDDI-UP)
  - SK-5522 (SK-NET FDDI-UP DAS)
  - SK-5541 (SK-NET FDDI-FP)
  - SK-5543 (SK-NET FDDI-LP)
  - SK-5544 (SK-NET FDDI-LP DAS)
  - SK-5821 (SK-NET FDDI-UP64)
  - SK-5822 (SK-NET FDDI-UP64 DAS)
  - SK-5841 (SK-NET FDDI-FP64)
  - SK-5843 (SK-NET FDDI-LP64)
  - SK-5844 (SK-NET FDDI-LP64 DAS)

Compaq 适配器（未测试）
  - Netelligent 100 FDDI DAS Fibre SC
  - Netelligent 100 FDDI SAS Fibre SC
  - Netelligent 100 FDDI DAS UTP
  - Netelligent 100 FDDI SAS UTP
  - Netelligent 100 FDDI SAS Fibre MIC


## 3. General Information


v2.01 起，该驱动已集成linux 内核源码中。因此，安装方式与内核支持的任何其他适配器相同
关于网络适配器的安装，请参阅你发行版的说明书
这让我的工作轻松多了 :-)

## 4. Troubleshooting


如果在安装过程中遇到问题，请检查以下各项：

Problem:
	  驱动找不FDDI 适配器
Reason:
	  /proc/pci 中查找以下条目：

	     'FDDI network controller: SysKonnect SK-FDDI-PCI ...'

	  如果该条目存在，FDDI 适配器已被系统找到，应当可以使用
	  如果该条目不存在，或文件 '/proc/pci' 不存在，则你可能有硬件问题，或者内核未启用 PCI 支持
	  可以使用 SysKonnect 网站上提供的诊断程序来检查适配器：

	      www.syskonnect.de

	  一COMPAQ 机器Linux 下存PCI 相关问题。这'PCI howto' 文档（包含在某些发行版中，或可从 www 获取，例'www.linux.org'）中有描述，目前没有解决办法
Problem:
	  你想把你的电脑用作多IP 子网（使用多个适配器）之间的路由器，但你无法访问其他子网中的计算机
Reason:
	  要么是路由器的内核未配置 IP 转发，要么是至少一个计算机上的路由表与网关配置有问题
如果你的问题未列于此，请联系我们的技术支持以获取帮助
你可以发送邮件至：linux@syskonnect.de

联系我们的技术支持时，请确保提供以下信息
- System Manufacturer and Model
- Boards in your system
- Distribution
- Kernel version


## 5. Function of the Adapter LEDs


	FDDI 网络适配器上 LED 的功能在 SMT 版本 v2.82 中做了变更。在这个新的 SMT 版本中，黄色 LED 用作环运行指示。黄LED 点亮表示环已断开。适配器上的绿LED 现在用作链路指示，绿LED 点亮表示该端口有物理连接
	v2.82 之前SMT 版本中，黄色 LED 熄灭表示环正常，而绿LED 显示适配器的连接状态。环断开时绿LED 熄灭而黄LED 点亮
	所有实现都表明，如果所LED 都熄灭，则表示驱动未加载

## 6. History


v2.06 (20000511) (In-Kernel version)
    New features:

 - 64 bit support
 - new pci dma interface
 - in kernel 2.3.99

v2.05 (20000217) (In-Kernel version)
    New features:

 - Changes for 2.3.45 kernel

v2.04 (20000207) (Standalone version)
    New features:

 - Added rx/tx byte counter

v2.03 (20000111) (Standalone version)
    Problems fixed:

 - Fixed printk statements from v2.02

v2.02 (991215) (Standalone version)
    Problems fixed:

 - Removed unnecessary output
 - Fixed path for "printver.sh" in makefile

v2.01 (991122) (In-Kernel version)
    New features:

 - Integration in Linux kernel sources
 - Support for memory mapped I/O.

v2.00 (991112)
    New features:

 - Full source released under GPL

v1.05 (991023)
    Problems fixed:

 - Compilation with kernel version 2.2.13 failed

v1.04 (990427)
    Changes:

 - New SMT module included, changing LED functionality

    Problems fixed:

 - Synchronization on SMP machines was buggy

v1.03 (990325)
    Problems fixed:

 - Interrupt routing on SMP machines could be incorrect

v1.02 (990310)
    New features:

 - Support for kernel versions 2.2.x added
 - Kernel patch instead of private duplicate of kernel functions

v1.01 (980812)
    Problems fixed:

	Connection hangup with telnet
	Slow telnet connection

v1.00 beta 01 (980507)
    New features:

	None.

    Problems fixed:

	None.

    Known limitations:

 - tar archive instead of standard package format (rpm).
 - FDDI statistic is empty.
 - not tested with 2.1.xx kernels
 - integration in kernel not tested
 - not tested simultaneously with FDDI adapters from other vendors.
 - only X86 processors supported.
 - SBA (Synchronous Bandwidth Allocator) parameters can
	  not be configured.
 - does not work on some COMPAQ machines. See the PCI howto
	  document for details about this problem.
 - data corruption with kernel versions below 2.0.33.
