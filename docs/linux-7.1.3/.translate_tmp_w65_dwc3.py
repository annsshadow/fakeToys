import os, json

F = r"D:/WORKSPACE/linux-7.1.3/docs/系统文档/driver-api/usb/dwc3.md"

T = {
"## Synopsys DesignWare Core SuperSpeed USB 3.0 Controller":
"## Synopsys DesignWare Core SuperSpeed USB 3.0 控制器",
"## Introduction":
"## 简介",
'The **Synopsys DesignWare Core SuperSpeed USB 3.0 Controller**':
'**Synopsys DesignWare Core SuperSpeed USB 3.0 控制器**',
'(hereinafter referred to as **DWC3**) is a USB SuperSpeed compliant':
'（以下简称 **DWC3**）是一个符合 USB SuperSpeed 规范的',
'controller which can be configured in one of 4 ways:':
'控制器，可通过以下 4 种方式之一进行配置：',
' 1. Peripheral-only configuration':
' 1. 仅外设（Peripheral-only）配置',
' 2. Host-only configuration':
' 2. 仅主机（Host-only）配置',
' 3. Dual-Role configuration':
' 3. 双角色（Dual-Role）配置',
' 4. Hub configuration':
' 4. 集线器（Hub）配置',
'Linux currently supports several versions of this controller. In all':
'Linux 目前支持该控制器的多个版本。',
'likelihood, the version in your SoC is already supported. At the time':
'你 SoC 中的版本极有可能已经受支持。在撰写本文时，',
'of this writing, known tested versions range from 2.02a to 3.10a. As a':
'已知经过测试的版本范围从 2.02a 到 3.10a。',
'rule of thumb, anything above 2.02a should work reliably well.':
'作为经验法则，高于 2.02a 的版本应该都能稳定工作。',
'Currently, we have many known users for this driver. In alphabetical':
'目前，该驱动有许多已知用户。按字母',
'order:':
'顺序排列如下：',
" 1. Cavium":
" 1. Cavium",
" 2. Intel Corporation":
" 2. Intel Corporation",
" 3. Qualcomm":
" 3. Qualcomm",
" 4. Rockchip":
" 4. Rockchip",
" 5. ST":
" 5. ST",
" 6. Samsung":
" 6. Samsung",
" 7. Texas Instruments":
" 7. Texas Instruments",
" 8. Xilinx":
" 8. Xilinx",
"## Summary of Features":
"## 特性概述",
'For details about features supported by your version of DWC3, consult':
'有关你的 DWC3 版本所支持特性的详细信息，请咨询',
'your IP team and/or *Synopsys DesignWare Core SuperSpeed USB 3.0':
'你的 IP 团队和/或 *Synopsys DesignWare Core SuperSpeed USB 3.0',
'Controller Databook*. Following is a list of features supported by the':
'Controller Databook*。以下是撰写本文时驱动所支持的',
'driver at the time of this writing:':
'特性列表：',
' 1. Up to 16 bidirectional endpoints (including the control':
' 1. 最多 16 个双向端点（包括控制',
'	   pipe - ep0)':
'	   管道 - ep0）',
' 2. Flexible endpoint configuration':
' 2. 灵活的端点配置',
' 3. Simultaneous IN and OUT transfer support':
' 3. 同时支持 IN 和 OUT 传输',
' 4. Scatter-list support':
' 4. 散列表（Scatter-list）支持',
' 5. Up to 256 TRBs [#trb]_ per endpoint':
' 5. 每个端点最多 256 个 TRB [#trb]_',
' 6. Support for all transfer types (**Control**, **Bulk**,':
' 6. 支持所有传输类型（**Control**、**Bulk**、',
'	   **Interrupt**, and **Isochronous**)':
'	   **Interrupt** 和 **Isochronous**）',
' 7. SuperSpeed Bulk Streams':
' 7. SuperSpeed 批量流（Bulk Streams）',
' 8. Link Power Management':
' 8. 链路电源管理（Link Power Management）',
' 9. Trace Events for debugging':
' 9. 用于调试的 Trace Events',
' 10. DebugFS [#debugfs]_ interface':
' 10. DebugFS [#debugfs]_ 接口',
'These features have all been exercised with many of the **in-tree**':
'这些特性都已通过许多**树内**（in-tree）',
'gadget drivers. We have verified both **ConfigFS** [#configfs]_ and':
'gadget 驱动进行了验证。我们已验证 **ConfigFS** [#configfs]_ 和',
'legacy gadget drivers.':
'传统的 gadget 驱动。',
"## Driver Design":
"## 驱动设计",
'The DWC3 driver sits on the **drivers/usb/dwc3/** directory. All files':
'DWC3 驱动位于 **drivers/usb/dwc3/** 目录。所有文件',
'related to this driver are in this one directory. This makes it easy':
'都与此驱动相关并位于同一目录中。这使得',
'for new-comers to read the code and understand how it behaves.':
'新手能够轻松阅读代码并理解其行为。',
'Because of DWC3\'s configuration flexibility, the driver is a little':
'由于 DWC3 的配置灵活性，该驱动在某些地方',
'complex in some places but it should be rather straightforward to':
'略显复杂，但整体仍应相当',
'understand.':
'易于理解。',
'The biggest part of the driver refers to the Gadget API.':
'该驱动最主要的部分涉及 Gadget API。',
"## Known Limitations":
"## 已知限制",
'Like any other HW, DWC3 has its own set of limitations. To avoid':
'与任何其他硬件一样，DWC3 也有其自身的一组限制。为了',
'constant questions about such problems, we decided to document them':
'避免不断被问及此类问题，我们决定在此',
'here and have a single location to where we could point users.':
'记录它们，并提供一个统一的指引位置供用户参考。',
"### OUT Transfer Size Requirements":
"### OUT 传输大小要求",
'According to Synopsys Databook, all OUT transfer TRBs [#trb]_ must':
'根据 Synopsys Databook，所有 OUT 传输 TRB [#trb]_ 必须',
'have their **size** field set to a value which is integer divisible by':
'将其 **size** 字段设置为一个能被端点 **wMaxPacketSize**',
"the endpoint's **wMaxPacketSize**. This means that **e.g.** in order to":
'整除的值。这意味着，例如，为了',
'receive a Mass Storage **CBW** [#cbw]_, req->length must either be set':
'接收 Mass Storage 的 **CBW** [#cbw]_，req->length 必须设置为',
'to a value that\'s divisible by **wMaxPacketSize** (1024 on SuperSpeed,':
'一个能被 **wMaxPacketSize** 整除的值（SuperSpeed 下为 1024，',
'512 on HighSpeed, etc), or DWC3 driver must add a Chained TRB pointing':
'HighSpeed 下为 512 等），或者 DWC3 驱动必须添加一个指向',
'to a throw-away buffer for the remaining length. Without this, OUT':
'废弃缓冲区的链式 TRB 以处理剩余长度。否则，OUT',
'transfers will **NOT** start.':
'传输将**无法**启动。',
'Note that as of this writing, this won\'t be a problem because DWC3 is':
'请注意，截至撰写本文时，这不会成为问题，因为 DWC3',
'fully capable of appending a chained TRB for the remaining length and':
'完全能够为剩余长度追加一个链式 TRB，并',
'completely hide this detail from the gadget driver. It\'s still worth':
'向 gadget 驱动完全隐藏这一细节。但仍有必要',
'mentioning because this seems to be the largest source of queries':
'提及，因为这似乎是有关 DWC3 以及',
'about DWC3 and **non-working transfers**.':
'**传输无法工作**的最大疑问来源。',
"### TRB Ring Size Limitation":
"### TRB 环大小限制",
'We, currently, have a hard limit of 256 TRBs [#trb]_ per endpoint,':
'目前，我们对每个端点有 256 个 TRB [#trb]_ 的硬性限制，',
'with the last TRB being a Link TRB [#link_trb]_ pointing back to the':
'最后一个 TRB 是一个指回',
'first. This limit is arbitrary but it has the benefit of adding up to':
'第一个的 Link TRB [#link_trb]_。该限制是任意设定的，但其好处是',
'exactly 4096 bytes, or 1 Page.':
'总和恰好为 4096 字节，即 1 个页（Page）。',
'DWC3 driver will try its best to cope with more than 255 requests and,':
'DWC3 驱动会尽力处理超过 255 个请求的情况，并且',
'for the most part, it should work normally. However this is not':
'在大多数情况下应能正常工作。但这并不是',
'something that has been exercised very frequently. If you experience any':
'经常被验证的场景。如果你遇到任何',
'problems, see section **Reporting Bugs** below.':
'问题，请参阅下文“报告缺陷”一节。',
"## Reporting Bugs":
"## 报告缺陷",
'Whenever you encounter a problem with DWC3, first and foremost you':
'每当你遇到 DWC3 的问题时，首先应',
'should make sure that:':
'确保：',
" 1. You're running latest tag from `Linus' tree`_":
" 1. 你正在运行 `Linus' tree`_ 的最新标签",
' 2. You can reproduce the error without any out-of-tree changes':
' 2. 你能够在不对 DWC3 做任何树外（out-of-tree）修改的情况下',
'	   to DWC3':
'	   复现该错误',
' 3. You have checked that it is not a fault on the host machine':
' 3. 你已确认问题并非来自主机（host machine）端的故障',
'After all these are verified, then here is how to capture enough':
'在确认以上各项之后，下面介绍如何',
'information so we can be of any help to you.':
'收集足够的信息以便我们能为你提供帮助。',
"### Required Information":
"### 所需信息",
'DWC3 relies exclusively on Trace Events for debugging. Everything is':
'DWC3 完全依赖 Trace Events 进行调试。相关信息',
'exposed there, with some extra bits being exposed to DebugFS':
'都在其中暴露出来，另有部分额外信息暴露在 DebugFS',
'[#debugfs]_.':
'[#debugfs]_ 中。',
'In order to capture DWC3\'s Trace Events you should run the following':
'为了捕获 DWC3 的 Trace Events，你应在',
'commands **before** plugging the USB cable to a host machine:':
'将 USB 线缆插入主机**之前**运行以下命令：',
'After this is done, you can connect your USB cable and reproduce the':
'完成上述操作后，你可以连接 USB 线缆并复现问题。',
'problem. As soon as the fault is reproduced, make a copy of files':
'一旦复现了故障，请像下面这样复制',
'`trace` and `regdump`, like so:':
'`trace` 和 `regdump` 文件：',
'Make sure to compress `trace.txt` and `regdump.txt` in a tarball':
'请务必将 `trace.txt` 和 `regdump.txt` 压缩为一个 tar 包，',
'and email it to `me`_ with `linux-usb`_ in Cc. If you want to be extra':
'并通过电子邮件发送给 `me`_，同时抄送（Cc）`linux-usb`_。如果你想更',
'sure that I will help you, write your subject line in the following':
'确保我能帮到你，请按以下格式',
'format:':
'撰写邮件主题：',
'On the email body, make sure to detail what you doing, which gadget':
'在邮件正文中，请务必详细说明你在做什么、使用的是哪个 gadget',
'driver you were using, how to reproduce the problem, what SoC you are':
'驱动、如何复现问题、你使用的是哪个 SoC，以及',
'using, which OS (and its version) was running on the Host machine.':
'主机上运行的是哪个操作系统（及其版本）。',
'With all this information, we should be able to understand what is':
'有了这些信息，我们应该能够理解发生了什么并',
'going on and be helpful to you.':
'为你提供帮助。',
"## Debugging":
"## 调试",
'With that out of the way, let us carry on.':
'说完免责声明，我们继续。',
'If you are willing to debug your own problem, you deserve a round of':
'如果你愿意自己调试问题，值得为你',
'applause :-)':
'鼓掌 :-)',
'Anyway, there is not much to say here other than Trace Events will be':
'总之，这里没有什么更多可说的，除了 Trace Events 对',
'really helpful in figuring out issues with DWC3. Also, access to':
'排查 DWC3 的问题确实很有帮助。此外，能够',
'Synopsys Databook will be **really** valuable in this case.':
'查阅 Synopsys Databook 在这种情况下也**非常**有价值。',
'A USB Sniffer can be helpful at times but it is not entirely required,':
'USB 嗅探器（Sniffer）有时会有帮助，但并非完全必要，',
'there is a lot that can be understood without looking at the wire.':
'很多信息无需查看总线即可理解。',
'Feel free to email `me`_ and Cc `linux-usb`_ if you need any help.':
'如果你需要任何帮助，随时可以发送电子邮件给 `me`_ 并抄送 `linux-usb`_。',
'### ``DebugFS``':
'### ``DebugFS``',
'`DebugFS` is very good for gathering snapshots of what is going on':
'`DebugFS` 非常适合于获取 DWC3 和/或任意端点的',
'with DWC3 and/or any endpoint.':
'当前状态快照。',
'On DWC3\'s `DebugFS` directory, you will find the following files and':
'在 DWC3 的 `DebugFS` 目录中，你会找到以下',
'directories:':
'文件和目录：',
'When read, `link_state` will print out one of `U0`, `U1`,':
'读取时，`link_state` 将打印出 `U0`、`U1`、',
'`U2`, `U3`, `SS.Disabled`, `RX.Detect`, `SS.Inactive`,':
'`U2`、`U3`、`SS.Disabled`、`RX.Detect`、`SS.Inactive`、',
'`Polling`, `Recovery`, `Hot Reset`, `Compliance`,':
'`Polling`、`Recovery`、`Hot Reset`、`Compliance`、',
'`Loopback`, `Reset`, `Resume` or `UNKNOWN link state`.':
'`Loopback`、`Reset`、`Resume` 或 `UNKNOWN link state` 之一。',
'This file can also be written to in order to force link to one of the':
'该文件也可以被写入，以强制链路进入',
'states above.':
'上述某个状态。',
'File name is self-explanatory. When read, `regdump` will print out a':
'文件名不言自明。读取时，`regdump` 将打印出',
'register dump of DWC3. Note that this file can be grepped to find the':
'DWC3 的寄存器转储（register dump）。请注意，可以对该文件',
'information you want.':
'执行 grep 以查找所需信息。',
'When read, `testmode` will print out a name of one of the specified':
'读取时，`testmode` 将打印出指定',
'USB 2.0 Testmodes (`test_j`, `test_k`, `test_se0_nak`,':
'USB 2.0 测试模式之一（`test_j`、`test_k`、`test_se0_nak`、',
'`test_packet`, `test_force_enable`) or the string `no test` in':
'`test_packet`、`test_force_enable`），或者在没有测试正在执行时',
'case no tests are currently being executed.':
'打印字符串 `no test`。',
'In order to start any of these test modes, the same strings can be':
'要启动这些测试模式中的任意一个，可以将相同的字符串',
'written to the file and DWC3 will enter the requested test mode.':
'写入该文件，DWC3 将进入所请求的测试模式。',
'For each endpoint we expose one directory following the naming':
'对于每个端点，我们按照',
'convention `ep$num$dir` **(ep0in, ep0out, ep1in, ...)**. Inside each':
'`ep$num$dir`（ep0in、ep0out、ep1in……）的命名约定暴露一个目录。在这些',
'of these directories you will find the following files:':
'目录中，你会找到以下文件：',
'With access to Synopsys Databook, you can decode the information on':
'借助 Synopsys Databook，你可以解码其中的',
'them.':
'信息。',
'When read, `transfer_type` will print out one of `control`,':
'读取时，`transfer_type` 将根据端点描述符的内容打印出',
'`bulk`, `interrupt` or `isochronous` depending on what the':
'`control`、`bulk`、`interrupt` 或 `isochronous` 之一。',
'endpoint descriptor says. If the endpoint has not been enabled yet, it':
'如果端点尚未启用，它将',
'will print `--`.':
'打印 `--`。',
'When read, `trb_ring` will print out details about all TRBs on the':
'读取时，`trb_ring` 将打印出环上所有 TRB 的详细信息。',
'ring. It will also tell you where our enqueue and dequeue pointers are':
'它还会告诉你我们的入队（enqueue）和出队（dequeue）指针',
'located in the ring:':
'在环中的位置：',
"### Trace Events":
"### Trace Events",
'DWC3 also provides several trace events which help us gathering':
'DWC3 还提供了多个 trace events，帮助我们在运行时',
'information about the behavior of the driver during runtime.':
'收集关于驱动行为的信息。',
'In order to use these events, you must enable `CONFIG_FTRACE` in':
'要使用这些事件，你必须在',
'your kernel config.':
'内核配置中启用 `CONFIG_FTRACE`。',
'For details about how enable DWC3 events, see section **Reporting':
'关于如何启用 DWC3 事件的详细信息，请参阅',
'Bugs**.':
'“报告缺陷”一节。',
'The following subsections will give details about each Event Class and':
'以下小节将详细介绍 DWC3 定义的',
'each Event defined by DWC3.':
'每个事件类（Event Class）和每个事件。',
'MMIO':
'MMIO',
'It is sometimes useful to look at every MMIO access when looking for':
'在查找缺陷时，查看每一次 MMIO 访问有时很有用。',
'bugs. Because of that, DWC3 offers two Trace Events (one for':
'因此，DWC3 提供了两个 Trace Events（一个用于',
'Interrupt Events':
'Interrupt Events',
'Every IRQ event can be logged and decoded into a human readable':
'每个 IRQ 事件都可以被记录并解码为可读',
'string. Because every event will be different, we do not give an':
'字符串。由于每个事件都不同，我们没有给出',
'Control Request':
'Control Request',
'Every USB Control Request can be logged to the trace buffer. The':
'每个 USB 控制请求（Control Request）都可以被记录到 trace 缓冲区中。',
'Note that Standard Control Requests will be decoded into':
'注意，标准控制请求（Standard Control Requests）将被解码为',
'human-readable strings with their respective arguments. Class and':
'带各自参数的可读字符串。类（Class）和',
'Vendor requests will be printed out a sequence of 8 bytes in hex':
'厂商（Vendor）请求将以十六进制格式',
'format.':
'打印出 8 字节序列。',
'Lifetime of a `struct usb_request`':
'Lifetime of a `struct usb_request`',
'The entire lifetime of a `struct usb_request` can be tracked on the':
'`struct usb_request` 的整个生命周期都可以在 trace 缓冲区中被跟踪。',
'trace buffer. We have one event for each of allocation, free,':
'我们为分配（allocation）、释放（free）等',
'Generic Commands':
'Generic Commands',
'We can log and decode every Generic Command with its completion':
'我们可以记录并解码每个通用命令（Generic Command）及其完成',
'Endpoint Commands':
'Endpoint Commands',
'Endpoints commands can also be logged together with completion':
'端点命令（Endpoint Commands）也可以与完成状态一同被记录',
'Lifetime of a `TRB`':
'Lifetime of a `TRB`',
'A `TRB` Lifetime is simple. We are either preparing a `TRB` or':
'`TRB` 的生命周期很简单。我们要么在准备一个 `TRB`，要么',
'completing it. With these two events, we can see how a `TRB` changes':
'在完成它。通过这两个事件，我们可以看到 `TRB` 如何变化',
'Lifetime of an Endpoint':
'Lifetime of an Endpoint',
'And endpoint\'s lifetime is summarized with enable and disable':
'端点的生命周期通过启用（enable）和禁用（disable）来概括',
"## Structures, Methods and Definitions":
"## 结构体、方法与定义",
}

src = open(F, encoding='utf-8').read()
out_lines = []
for line in src.split('\n'):
    key = line.rstrip()
    if key in T:
        out_lines.append(T[key])
    else:
        out_lines.append(line)
new = '\n'.join(out_lines)

# self-check fences
n = new.count('```')
assert n % 2 == 0, "fence odd: %d" % n

tmp = F + '.tmp'
open(tmp, 'w', encoding='utf-8').write(new)
os.replace(tmp, F)
print("dwc3 done; fences:", n)
