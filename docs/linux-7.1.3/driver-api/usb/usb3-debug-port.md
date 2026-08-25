## USB3 调试端口


:作 Lu Baolu <baolu.lu@linux.intel.com>
:鏃ユ湡: 2017 骞?3 鏈。
## 概述（GENERAL

本文介绍如何x86 系统上使USB3 调试端口
在使用任何基USB3 的调试功能之

```
	1) check whether any USB3 debug port is available in
	   your system;
	2) check which port is used for debugging purposes;
	3) have a USB 3.0 super-speed A-to-A debugging cable.

```

## 介绍（INTRODUCTION

xHCI 调试能力（DbC）是 xHCI 主机控制器提供的一个可选但独立功能。xHCI 规范7.6 节中描述DbC
DbC 被初始化并启用后，它会通过调试端口（通常是第一USB3
超高速端口）呈现一个调试设备。该调试设备完全符合
USB 框架，并在调试目标（被调试的系统）与调试主机（debug host）之提供相当于一条非常高性能的、全双工串行链路
## 早期打印（EARLY PRINTK

DbC 被设计用来记early printk 消息。该特性的一个用途是内核调试例如，当你的机器在常规控制台代码初始化之前就非常早地崩溃时其他用途包括更简单、无锁（lockless）的日志，而非完整printk 控制台驱动与 klogd
在调试目标系统上，你需要定制一个启用了
CONFIG_EARLY_PRINTK_USB_XDBC 的调试内核。并添加如下
内核参数


```
	"earlyprintk=xdbc"

```

如果你的系统中有多个 xHCI 控制器，你可在该内核参数后附加一个主机控制器索引。该索引0 开始
当前设计不支DbC 运行时挂恢复。因此，你最好为
以下参数禁用运行时电源管

```
	"usbcore.autosuspend=-1"

```

在启动调试目标之前，你应该将调试端口连接到调试主机上一USB 端口（根端口或任何外部集线器的端口）。用于连接这两个端口线缆应当是一USB 3.0 超高A-to-A 调试线缆
在调试目标早期启动过程中，DbC 会被检测到并初始化初始化完成后，调试主机应当能够枚举调试目标中的调试设备随后调试主机会将调试设备usb_debug 驱动模块绑定并创/dev/ttyUSB 设备
如果调试设备的枚举顺利进行，你应该能看到


```
	# tail -f /var/log/kern.log
	[ 1815.983374] usb 4-3: new SuperSpeed USB device number 4 using xhci_hcd
	[ 1815.999595] usb 4-3: LPM exit latency is zeroed, disabling LPM.
	[ 1815.999899] usb 4-3: New USB device found, idVendor=1d6b, idProduct=0004
	[ 1815.999902] usb 4-3: New USB device strings: Mfr=1, Product=2, SerialNumber=3
	[ 1815.999903] usb 4-3: Product: Remote GDB
	[ 1815.999904] usb 4-3: Manufacturer: Linux
	[ 1815.999905] usb 4-3: SerialNumber: 0001
	[ 1816.000240] usb_debug 4-3:1.0: xhci_dbc converter detected
	[ 1816.000360] usb 4-3: xhci_dbc converter now attached to ttyUSB0

```

你可以使用任何通信程序（例minicom）来读取并查看这些消息下面这个简单的 bash 脚本可以帮助你检查设置是否正确

	===== start of bash scripts =============
	#!/bin/bash

	while true ; do
		while [ ! -d /sys/class/tty/ttyUSB0 ] ; do
			:
		done
	cat /dev/ttyUSB0
	done
	===== end of bash scripts ===============

## 串行 TTY（Serial TTY

DbC 支持已被添加xHCI 驱动中。你可以在运行时获得
DbC 提供的调试设备
为了使用此功能，你需要确保内核已配置为支USB_XHCI_DBGCAP。xHCI 设备节点下的一sysfs 属用于启用或禁DbC。默认情况下

```
	root@target:/sys/bus/pci/devices/0000:00:14.0# cat dbc
	disabled

```

```
	root@target:/sys/bus/pci/devices/0000:00:14.0# echo enable > dbc

```

```
	root@target:/sys/bus/pci/devices/0000:00:14.0# cat dbc
	enabled

```

使用一USB 3.0 超高A-to-A 调试线缆将调试目标连接到
调试主机。你会看/dev/ttyDBC0 被创建，


```
	root@target: tail -f /var/log/kern.log
	[  182.730103] xhci_hcd 0000:00:14.0: DbC connected
	[  191.169420] xhci_hcd 0000:00:14.0: DbC configured
	[  191.169597] xhci_hcd 0000:00:14.0: DbC now attached to /dev/ttyDBC0

```

```
	root@target:/sys/bus/pci/devices/0000:00:14.0# cat dbc
	configured

```

在调试主机上，你会看到调试设备已被枚举

```
	root@host: tail -f /var/log/kern.log
	[   79.454780] usb 2-2.1: new SuperSpeed USB device number 3 using xhci_hcd
	[   79.475003] usb 2-2.1: LPM exit latency is zeroed, disabling LPM.
	[   79.475389] usb 2-2.1: New USB device found, idVendor=1d6b, idProduct=0010
	[   79.475390] usb 2-2.1: New USB device strings: Mfr=1, Product=2, SerialNumber=3
	[   79.475391] usb 2-2.1: Product: Linux USB Debug Target
	[   79.475392] usb 2-2.1: Manufacturer: Linux Foundation
	[   79.475393] usb 2-2.1: SerialNumber: 0001

```

调试设备现在已工作。你可以使用任何通信或调程序在主机与目标之间进行通信