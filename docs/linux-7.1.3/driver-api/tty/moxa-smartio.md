## MOXA Smartio/Industio Family Device Driver Installation Guide


Copyright (C) 2008, Moxa Inc.
Copyright (C) 2021, Jiri Slaby


   1. Introduction
   2. System Requirement
   3. Installation
      3.1 Hardware installation
      3.2 Device naming convention
   4. Utilities
   5. Setserial
   6. Troubleshooting

##### 1. Introduction


   Smartio/Industio/UPCI 系列 Linux 驱动支持以下多端口板卡：

    - 2 ports multiport board
	CP-102U, CP-102UL, CP-102UF
	CP-132U-I, CP-132UL,
	CP-132, CP-132I, CP132S, CP-132IS,
	(CP-102, CP-102S)

    - 4 ports multiport board
	CP-104EL,
	CP-104UL, CP-104JU,
	CP-134U, CP-134U-I,
	C104H/PCI, C104HS/PCI,
	CP-114, CP-114I, CP-114S, CP-114IS, CP-114UL,
	(C114HI, CT-114I),
	POS-104UL,
	CB-114,
	CB-134I

    - 8 ports multiport board
	CP-118EL, CP-168EL,
	CP-118U, CP-168U,
	C168H/PCI,
	CB-108

   如果发生兼容性问题，请联Moxa：support@moxa.com.tw
   除设备驱动外，本版本还提供了一些有用的工具。它们是
    - msdiag
		 用于显示已安装的 Moxa Smartio/Industio 板卡的诊断程序    - msmon
		 用于观察数据计数和线路状态信号的监视程序    - msterm     一个用于测试串口的简单终端程序
   本版本中所有的驱动和工具都以源代码形式GNU General Public License 下发布。详情请参阅各源代码文件中的 GNU General Public License 声明
   Moxa 的网站上，你总能找到最新驱动：https://www.moxa.com/
   本版本驱动可以安装为可加载模块（Module driver）或内建到内核中（Static driver）。安装驱动前，请参考用户手册中的硬件安装步骤
   我们假设用户应当熟悉以下文档
   - Serial-HOWTO
   - Kernel-HOWTO

##### 2. System Requirement


   - 最多可组合安装 4 块板
##### 3. Installation


## 3.1 Hardware installation


### PCI/UPCI board


   你可能需要在 BIOS 中调IRQ 使用以避免与其他 ISA 设备发生 IRQ 冲突。请提前参考用户手册中的硬件安装步骤
### PCI IRQ Sharing


   同一块多端口板卡内的每个端口共享同一IRQ。最多可4 Moxa Smartio/Industio PCI 系列多端口板卡安装在同一系统中，并且它们可以共享同一IRQ


## 3.2 Device naming convention


   设备节点命名"ttyMxx"
### Device naming when more than 2 boards installed


   Smartio/Industio 每块多端口板卡的命名约定预定义如下
   ============ ===============
   Board Num.	Device node
   1st board	ttyM0  - ttyM7
   2nd board	ttyM8  - ttyM15
   3rd board	ttyM16 - ttyM23
   4th board	ttyM24 - ttyM31
   ============ ===============

##### 4. Utilities


   本驱动包3 个工具，msdiag、msmon msterm。这 3 个工具以源代码形式发布。它们应当被编译为可执行文件并复制到 /usr/bin
## msdiag - Diagnostic


   该工具提供显示系统中驱动所找到Moxa Smartio/Industio 板卡的功能
## msmon - Port Monitoring


   该工具让用户快速查看所MOXA 端口的活动。可以轻松了解每个端口自监视开始以来的接收/发送（Rx/Tx）字符总数
   每秒Rx/Tx 吞吐量既按间隔（例如最5 秒）报告，也按平均（自监视开始以来）报告。你可以<HOME> 键重置所有端口计数。按 <+> <->（加/减）键更改显示的时间间隔。在光标所在端口上<ENTER> 可查看该端口的通信参数、信号状态以及输输出队列
## msterm - Terminal Emulation


   该工具提供所tty 端口（尤其是 MOXA 端口）的数据收发能力。它对测试简单应用很有用，例如向连接到该端口的调制解调器发AT 命令，或作为登录用的终端。注意，这只是一个哑终端仿真，不处理全屏操作
##### 5. Setserial


   支持Setserial 参数如下
   ============== =============================================================
   uart		  set UART type(16450 --> disable FIFO, 16550A --> enable FIFO)
   close_delay	  set the amount of time (in 1/100 of a second) that DTR
		  should be kept low while being closed.
   closing_wait   set the amount of time (in 1/100 of a second) that the
		  serial port should wait for data to be drained while
		  being closed, before the receiver is disabled.
   spd_hi	  Use 57.6kb when the application requests 38.4kb.
   spd_vhi	  Use 115.2kb when the application requests 38.4kb.
   spd_shi	  Use 230.4kb when the application requests 38.4kb.
   spd_warp	  Use 460.8kb when the application requests 38.4kb.
   spd_normal	  Use 38.4kb when the application requests 38.4kb.
   spd_cust	  Use the custom divisor to set the speed when the
		  application requests 38.4kb.
   divisor	  This option sets the custom division.
   baud_base	  This option sets the base baud rate.
   ============== =============================================================

##### 6. Troubleshooting


   启动时的错误消息及解决方案已尽可能清晰地说明。如果所有可能的解决方案都失败，请联系我们的技术支持团队以获取更多帮助

   Error msg:
	      More than 4 Moxa Smartio/Industio family boards found. Fifth board
              and after are ignored.

   Solution:
   为避免此问题，请拔下第五块及之后的板卡，因为 Moxa 驱动最多支4 块板卡