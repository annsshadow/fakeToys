
## PCI 测试功能（PCI Test Function

:Author: Kishon Vijay Abraham I <kishon@ti.com>

传统上，PCI RC（Root Complex）一直通过使用标准PCI 卡（如以太网 PCI 卡、USB PCI 卡或 SATA PCI 卡）来验证。不过，随着 Linux 内核中加EP-core，可以将一个可运行EP 模式PCI 控制器配置为作为测试设备工作
PCI 端点测试设备是一个虚拟设备（在软件中定义），用于测试端点功能，并作为其他 PCI 端点设备（使EP 框架）的示例驱动
PCI 端点测试设备具有以下寄存器：

 1) PCI_ENDPOINT_TEST_MAGIC
 2) PCI_ENDPOINT_TEST_COMMAND
 3) PCI_ENDPOINT_TEST_STATUS
 4) PCI_ENDPOINT_TEST_SRC_ADDR
 5) PCI_ENDPOINT_TEST_DST_ADDR
 6) PCI_ENDPOINT_TEST_SIZE
 7) PCI_ENDPOINT_TEST_CHECKSUM
 8) PCI_ENDPOINT_TEST_IRQ_TYPE
 9) PCI_ENDPOINT_TEST_IRQ_NUMBER

- PCI_ENDPOINT_TEST_MAGIC

该寄存器将用于测BAR0。会写入一个已知模式并MAGIC 寄存器读回，以验BAR0
- PCI_ENDPOINT_TEST_COMMAND

该寄存器由主机驱动用来指示端点设备必须执行的功能
========	================================================================
Bitfield	Description
========	================================================================
Bit 0		触发传统（legacy）IRQ
Bit 1		触发 MSI IRQ
Bit 2		触发 MSI-X IRQ
Bit 3		读命令（RC 缓冲区读取数据）
Bit 4		写命令（RC 缓冲区写入数据）
Bit 5		复制命令（将一RC 缓冲区的数据复制到另一RC 缓冲区）
========	================================================================

- PCI_ENDPOINT_TEST_STATUS

该寄存器反映 PCI 端点设备的状态
========	==============================
Bitfield	Description
========	==============================
Bit 0		读成Bit 1		读失Bit 2		写成Bit 3		写失Bit 4		复制成功
Bit 5		复制失败
Bit 6		宸茶Е鍙?IRQ
Bit 7		源地址无效
Bit 8		目的地址无效
========	==============================

- PCI_ENDPOINT_TEST_SRC_ADDR

该寄存器包含 COPY/READ 命令的源地址（RC 缓冲区地址）
- PCI_ENDPOINT_TEST_DST_ADDR

该寄存器包含 COPY/WRITE 命令的目的地址（RC 缓冲区地址）
- PCI_ENDPOINT_TEST_IRQ_TYPE

该寄存器包含READ/WRITE/COPY 以及触发 IRQ（Legacy/MSI）命令所触发的中断类型
可选类型：

======	==
Legacy	0
MSI	1
MSI-X	2
======	==

- PCI_ENDPOINT_TEST_IRQ_NUMBER

该寄存器包含被触发的中断 ID
可取的值：

======	===========
Legacy	0
MSI	[1 .. 32]
MSI-X	[1 .. 2048]
======	===========
