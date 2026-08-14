## Synopsys DesignWare Core SuperSpeed USB 3.0 控制器

本文档介绍 Synopsys DesignWare Core SuperSpeed USB 3.0（DWC3）控制器在 Linux 中的驱动实现，涵盖其外设/主机/双角色等配置模式、驱动设计与已知限制，供 USB 驱动开发者参考。



:Author: Felipe Balbi <felipe.balbi@linux.intel.com>
:Date: April 2017

## 简介


**Synopsys DesignWare Core SuperSpeed USB 3.0 控制器**
（以下简称 **DWC3**）是一个符合 USB SuperSpeed 规范的
控制器，可通过以下 4 种方式之一进行配置：

 1. 仅外设（Peripheral-only）配置
 2. 仅主机（Host-only）配置
 3. 双角色（Dual-Role）配置
 4. 集线器（Hub）配置

Linux 目前支持该控制器的多个版本。
你 SoC 中的版本极有可能已经受支持。在撰写本文时，
已知经过测试的版本范围从 2.02a 到 3.10a。
作为经验法则，高于 2.02a 的版本应该都能稳定工作。

目前，该驱动有许多已知用户。按字母
顺序排列如下：

 1. Cavium
 2. Intel Corporation
 3. Qualcomm
 4. Rockchip
 5. ST
 6. Samsung
 7. Texas Instruments
 8. Xilinx

## 特性概述


有关你的 DWC3 版本所支持特性的详细信息，请咨询
你的 IP 团队和/或 *Synopsys DesignWare Core SuperSpeed USB 3.0
Controller Databook*。以下是撰写本文时驱动所支持的
特性列表：

 1. 最多 16 个双向端点（包括控制
	   管道 - ep0）
 2. 灵活的端点配置
 3. 同时支持 IN 和 OUT 传输
 4. 散列表（Scatter-list）支持
 5. 每个端点最多 256 个 TRB [#trb]_
 6. 支持所有传输类型（**Control**、**Bulk**、
	   **Interrupt** 和 **Isochronous**）
 7. SuperSpeed 批量流（Bulk Streams）
 8. 链路电源管理（Link Power Management）
 9. 用于调试的 Trace Events
 10. DebugFS [#debugfs]_ 接口

这些特性都已通过许多**树内**（in-tree）
gadget 驱动进行了验证。我们已验证 **ConfigFS** [#configfs]_ 和
传统的 gadget 驱动。

## 驱动设计


DWC3 驱动位于 **drivers/usb/dwc3/** 目录。所有文件
都与此驱动相关并位于同一目录中。这使得
新手能够轻松阅读代码并理解其行为。

由于 DWC3 的配置灵活性，该驱动在某些地方
略显复杂，但整体仍应相当
易于理解。

该驱动最主要的部分涉及 Gadget API。

## 已知限制


与任何其他硬件一样，DWC3 也有其自身的一组限制。为了
避免不断被问及此类问题，我们决定在此
记录它们，并提供一个统一的指引位置供用户参考。

### OUT 传输大小要求


根据 Synopsys Databook，所有 OUT 传输 TRB [#trb]_ 必须
将其 **size** 字段设置为一个能被端点 **wMaxPacketSize**
整除的值。这意味着，例如，为了
接收 Mass Storage 的 **CBW** [#cbw]_，req->length 必须设置为
一个能被 **wMaxPacketSize** 整除的值（SuperSpeed 下为 1024，
HighSpeed 下为 512 等），或者 DWC3 驱动必须添加一个指向
废弃缓冲区的链式 TRB 以处理剩余长度。否则，OUT
传输将**无法**启动。

请注意，截至撰写本文时，这不会成为问题，因为 DWC3
完全能够为剩余长度追加一个链式 TRB，并
向 gadget 驱动完全隐藏这一细节。但仍有必要
提及，因为这似乎是有关 DWC3 以及
**传输无法工作**的最大疑问来源。

### TRB 环大小限制


目前，我们对每个端点有 256 个 TRB [#trb]_ 的硬性限制，
最后一个 TRB 是一个指回
第一个的 Link TRB [#link_trb]_。该限制是任意设定的，但其好处是
总和恰好为 4096 字节，即 1 个页（Page）。

DWC3 驱动会尽力处理超过 255 个请求的情况，并且
在大多数情况下应能正常工作。但这并不是
经常被频繁测试验证的部分。如果你遇到任何问题，请参阅下文 **报告缺陷** 一节。

## 报告缺陷


每当你遇到 DWC3 的问题时，首先应
确保：

 1. 你正在运行 `Linus' tree`_ 的最新标签
 2. 你能够在不对 DWC3 做任何树外（out-of-tree）修改的情况下
	   复现该错误
 3. 你已确认该问题并非主机（host）端的故障

在以上各项都确认之后，下面介绍如何收集足够的信息以便我们能为你提供帮助。

### 所需信息


DWC3 完全依赖 Trace Events 进行调试。相关信息
都在其中暴露出来，另有部分额外信息暴露在 DebugFS
[#debugfs]_ 中。

为了捕获 DWC3 的 Trace Events，你应在
将 USB 线缆插入主机**之前**运行以下命令：


		 # mkdir -p /d
		 # mkdir -p /t
		 # mount -t debugfs none /d
		 # mount -t tracefs none /t
		 # echo 81920 > /t/buffer_size_kb
		 # echo 1 > /t/events/dwc3/enable

完成上述操作后，你可以连接 USB 线缆并复现问题。
一旦复现了故障，请像下面这样复制
`trace` 和 `regdump` 文件：


		# cp /t/trace /root/trace.txt
		# cat /d/**dwc3**/regdump > /root/regdump.txt

请务必将 `trace.txt` 和 `regdump.txt` 压缩为一个 tar 包，
并通过电子邮件发送给 `me`_，同时抄送（Cc）`linux-usb`_。如果你想更
为了确保我能帮助你，请按以下格式撰写邮件主题：

	**[BUG REPORT] usb: dwc3: Bug while doing XYZ**

在邮件正文中，请务必详细说明你在做什么、使用的是哪个 gadget
驱动、如何复现问题、使用的是哪个 SoC、以及主机上运行的是哪个操作系统（及其版本）。

有了以上全部信息，我们应该能够理解问题所在并为你提供帮助。

## 调试


```

  DISCLAIMER: The information available on DebugFS and/or TraceFS can
  change at any time at any Major Linux Kernel Release. If writing
  scripts, do **NOT** assume information to be available in the
  current format.

```
说完上述免责声明，我们继续。

如果你愿意自己调试问题，那值得为你鼓掌 :-)

总之，除了 Trace Events 对排查 DWC3 的问题确实很有帮助之外，这里没什么可多说的。此外，能够
查阅 Synopsys Databook 在这种情况下也**非常**有价值。

USB 抓包工具（Sniffer）有时会有帮助，但并非完全必需，很多信息无需查看线路即可理解。

如果你需要任何帮助，随时可以发送电子邮件给 `me`_ 并抄送 `linux-usb`_。

### ``DebugFS``


`DebugFS` 非常适合用于获取当前运行状态的快照。

在 DWC3 的 `DebugFS` 目录中，你会找到以下
文件和目录：

`ep[0..15]{in,out}/`
`link_state`
`regdump`
`testmode`

`link_state`
``````````````

读取时，`link_state` 将打印出 `U0`、`U1`、
`U2`、`U3`、`SS.Disabled`、`RX.Detect`、`SS.Inactive`、
`Polling`、`Recovery`、`Hot Reset`、`Compliance`、
`Loopback`、`Reset`、`Resume` 或 `UNKNOWN link state` 之一。

该文件也可以被写入，以强制链路进入
上述某个状态。

`regdump`
`````````````

文件名不言自明。读取时，`regdump` 将打印出
DWC3 的寄存器转储（register dump）。请注意，可以对该文件
执行 grep 以查找所需信息。

`testmode`
``````````````

读取时，`testmode` 将打印出指定
USB 2.0 测试模式之一（`test_j`、`test_k`、`test_se0_nak`、
`test_packet`、`test_force_enable`），或者在没有测试正在执行时
打印字符串 `no test`。

要启动这些测试模式中的任意一个，可以将相同的字符串
写入该文件，DWC3 将进入所请求的测试模式。


`ep[0..15]{in,out}`
``````````````````````

对于每个端点，我们按照
`ep$num$dir`（ep0in、ep0out、ep1in……）的命名约定暴露一个目录。在这些
目录中，你会找到以下文件：

`descriptor_fetch_queue`
`event_queue`
`rx_fifo_queue`
`rx_info_queue`
`rx_request_queue`
`transfer_type`
`trb_ring`
`tx_fifo_queue`
`tx_request_queue`

借助 Synopsys Databook，你可以解码其中的
信息。

#### ``transfer_type``


读取时，`transfer_type` 将根据端点描述符的内容打印出
`control`、`bulk`、`interrupt` 或 `isochronous` 之一。
如果端点尚未被启用，则打印 `--`。

#### ``trb_ring``


读取时，`trb_ring` 将打印出环上所有 TRB 的详细信息。
它还会告诉你我们的入队（enqueue）和出队（dequeue）指针
在环中的位置：


		buffer_addr,size,type,ioc,isp_imi,csp,chn,lst,hwo
		000000002c754000,481,normal,1,0,1,0,0,0         
		000000002c75c000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c754000,481,normal,1,0,1,0,0,0         
		000000002c75c000,481,normal,1,0,1,0,0,0         
		000000002c784000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c784000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c754000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c784000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c784000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c754000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c75c000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c754000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c754000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c754000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c75c000,481,normal,1,0,1,0,0,0         
		000000002c780000,481,normal,1,0,1,0,0,0         
		000000002c784000,481,normal,1,0,1,0,0,0         
		000000002c788000,481,normal,1,0,1,0,0,0         
		000000002c78c000,481,normal,1,0,1,0,0,0         
		000000002c790000,481,normal,1,0,1,0,0,0         
		000000002c754000,481,normal,1,0,1,0,0,0         
		000000002c758000,481,normal,1,0,1,0,0,0         
		000000002c75c000,512,normal,1,0,1,0,0,1        D
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0       E 
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		0000000000000000,0,UNKNOWN,0,0,0,0,0,0         
		00000000381ab000,0,link,0,0,0,0,0,1


### Trace Events


DWC3 还提供了多个 trace events，帮助我们在运行时
收集关于驱动行为的信息。

要使用这些事件，你必须在
内核配置中启用 `CONFIG_FTRACE`。

关于如何启用 DWC3 事件的详细信息，请参阅
“报告缺陷”一节。

以下小节将详细介绍 DWC3 定义的
每个事件类（Event Class）和每个事件。

MMIO
```````

在查找缺陷时，查看每一次 MMIO 访问有时很有用。
因此，DWC3 提供了两个 Trace Events（一个用于读操作，一个用于写操作）：
```

  TP_printk("addr %p value %08x", __entry->base + __entry->offset,
  		__entry->value)

```
Interrupt Events
````````````````

每个 IRQ 事件都可以被记录并解码为可读
字符串。由于每个事件都不同，我们这里不给出示例。
```

  TP_printk("event (%08x): %s", __entry->event,
  		dwc3_decode_event(__entry->event, __entry->ep0state))

```
Control Request
`````````````````

每个 USB 控制请求（Control Request）都可以被记录到 trace 缓冲区中。
```

  TP_printk("%s", dwc3_decode_ctrl(__entry->bRequestType,
  				__entry->bRequest, __entry->wValue,
  				__entry->wIndex, __entry->wLength)
  )

```
注意，标准控制请求（Standard Control Requests）将被解码为
带各自参数的可读字符串。类（Class）和
厂商（Vendor）请求将以十六进制格式
打印出 8 字节序列。

Lifetime of a `struct usb_request`
```````````````````````````````````````

`struct usb_request` 的整个生命周期都可以在 trace 缓冲区中被跟踪。
我们为分配（allocation）、释放（free）等操作提供了 trace 事件。
```

  TP_printk("%s: req %p length %u/%u %s%s%s ==> %d",
  	__get_str(name), __entry->req, __entry->actual, __entry->length,
  	__entry->zero ? "Z" : "z",
  	__entry->short_not_ok ? "S" : "s",
  	__entry->no_interrupt ? "i" : "I",
  	__entry->status
  )

```
Generic Commands
````````````````````

我们可以记录并解码每个通用命令（Generic Command）及其完成
```

  TP_printk("cmd '%s' [%x] param %08x --> status: %s",
  	dwc3_gadget_generic_cmd_string(__entry->cmd),
  	__entry->cmd, __entry->param,
  	dwc3_gadget_generic_cmd_status_string(__entry->status)
  )

```
Endpoint Commands
````````````````````

端点命令（Endpoint Commands）也可以与完成状态一同被记录
```

  TP_printk("%s: cmd '%s' [%d] params %08x %08x %08x --> status: %s",
  	__get_str(name), dwc3_gadget_ep_cmd_string(__entry->cmd),
  	__entry->cmd, __entry->param0,
  	__entry->param1, __entry->param2,
  	dwc3_ep_cmd_status_string(__entry->cmd_status)
  )

```
Lifetime of a `TRB`
``````````````````````

`TRB` 的生命周期很简单。我们要么在准备一个 `TRB`，要么
在完成它。通过这两个事件，我们可以看到 `TRB` 如何变化
```

  TP_printk("%s: %d/%d trb %p buf %08x%08x size %s%d ctrl %08x (%c%c%c%c:%c%c:%s)",
  	__get_str(name), __entry->queued, __entry->allocated,
  	__entry->trb, __entry->bph, __entry->bpl,
  	({char *s;
  	int pcm = ((__entry->size >> 24) & 3) + 1;
  	switch (__entry->type) {
  	case USB_ENDPOINT_XFER_INT:
  	case USB_ENDPOINT_XFER_ISOC:
  		switch (pcm) {
  		case 1:
  			s = "1x ";
  			break;
  		case 2:
  			s = "2x ";
  			break;
  		case 3:
  			s = "3x ";
  			break;
  		}
  	default:
  		s = "";
  	} s; }),
  	DWC3_TRB_SIZE_LENGTH(__entry->size), __entry->ctrl,
  	__entry->ctrl & DWC3_TRB_CTRL_HWO ? 'H' : 'h',
  	__entry->ctrl & DWC3_TRB_CTRL_LST ? 'L' : 'l',
  	__entry->ctrl & DWC3_TRB_CTRL_CHN ? 'C' : 'c',
  	__entry->ctrl & DWC3_TRB_CTRL_CSP ? 'S' : 's',
  	__entry->ctrl & DWC3_TRB_CTRL_ISP_IMI ? 'S' : 's',
  	__entry->ctrl & DWC3_TRB_CTRL_IOC ? 'C' : 'c',
      dwc3_trb_type_string(DWC3_TRBCTL_TYPE(__entry->ctrl))
  )  

```
Lifetime of an Endpoint
```````````````````````

端点的生命周期通过启用（enable）和禁用（disable）来概括
```

  TP_printk("%s: mps %d/%d streams %d burst %d ring %d/%d flags %c:%c%c%c%c%c:%c:%c",
  	__get_str(name), __entry->maxpacket,
  	__entry->maxpacket_limit, __entry->max_streams,
  	__entry->maxburst, __entry->trb_enqueue,
  	__entry->trb_dequeue,
  	__entry->flags & DWC3_EP_ENABLED ? 'E' : 'e',
  	__entry->flags & DWC3_EP_STALL ? 'S' : 's',
  	__entry->flags & DWC3_EP_WEDGE ? 'W' : 'w',
  	__entry->flags & DWC3_EP_TRANSFER_STARTED ? 'B' : 'b',
  	__entry->flags & DWC3_EP_PENDING_REQUEST ? 'P' : 'p',
  	__entry->flags & DWC3_EP_END_TRANSFER_PENDING ? 'E' : 'e',
  	__entry->direction ? '<' : '>'
  )


```
## 结构体、方法与定义


   :doc: main data structures
   :internal:

   :doc: gadget-only helpers
   :internal:

   :doc: gadget-side implementation
   :internal:

   :doc: core driver (probe, PM, etc)
   :internal:

	       Request Block.
