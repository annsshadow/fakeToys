
## Boot Interrupts


:Author: - Sean V Kelley <sean.v.kelley@linux.intel.com>

## Overview


PCI Express 上，中断MSI 或入站中断消息（Assert_INTx/Deassert_INTx）表示。给Core IO 中的集成 IO-APIC 将来PCI Express 的传统中断消息转换为 MSI 中断。如IO-APIC 被禁用（通过 IO-APIC 表项中的掩码位），这些消息会被路由到传统PCH。这种带内（in-band）中断机制传统上对于不支IO-APIC 的系统以及启动（boot）是必要的。Intel 过去使用术语“boot interrupts”来描述这种机制。此外，PCI Express 协议描述了这种带内传统线缆中INTx 机制，供 I/O 设备发出 PCI 风格的电平中断。后续段落描述了 Core IO 在处INTx 消息路由PCH 时的问题，以BIOS 和操作系统内的缓解措施

## Issue


当带内传INTx 消息被转发到 PCH 时，它们反过来会触发一个新的中断，而操作系统很可能缺少对应的处理程序。当中断长时间未被处理时，Linux 内核会将其作为伪中断（Spurious Interrupt）跟踪。当IRQ 达到特定计数时，Linux 内核会以 "nobody cared" 错误禁用它。这个被禁用IRQ 现在阻止了恰好共享该
```

  irq 19: nobody cared (try booting with the "irqpoll" option)
  CPU: 0 PID: 2988 Comm: irq/34-nipalk Tainted: 4.14.87-rt49-02410-g4a640ec-dirty #1
  Hardware name: National Instruments NI PXIe-8880/NI PXIe-8880, BIOS 2.1.5f1 01/09/2020
  Call Trace:

  <IRQ>
   ? dump_stack+0x46/0x5e
   ? __report_bad_irq+0x2e/0xb0
   ? note_interrupt+0x242/0x290
   ? nNIKAL100_memoryRead16+0x8/0x10 [nikal]
   ? handle_irq_event_percpu+0x55/0x70
   ? handle_irq_event+0x4f/0x80
   ? handle_fasteoi_irq+0x81/0x180
   ? handle_irq+0x1c/0x30
   ? do_IRQ+0x41/0xd0
   ? common_interrupt+0x84/0x84
  </IRQ>

  handlers:
  irq_default_primary_handler threaded usb_hcd_irq
  Disabling IRQ #19


```
## Conditions


使用线程化中断（threaded interrupts）是当今最有可能触发此问题的条件。线程化中断IRQ 处理程序唤醒后可能不会被重新启用。这些“一次性”（one shot）条件意味着线程化中断需要在线程处理程序运行之前一直保持中断线被屏蔽。特别是在处理高数据速率中断时，线程需要运行到完成；否则一些处理程序最终会导致栈溢出，因为发出设备的中断仍处于活动状态
## Affected Chipsets


传统的终端中断转发机制如今存在于许多设备中，包括但不限于来自 AMD/ATI、Broadcom Intel 的芯片组。通过下面缓解措施所做的更改已应用到 drivers/pci/quirks.c
ICX 开始，Core IO 的设备中不再有任IO-APIC。IO-APIC 仅在 PCH 中。连接到 Core IO PCIe Root Port 的设备将使用原生MSI/MSI-X 机制
## Mitigations


缓解措施采取 PCI quirks 的形式。优先做法是首先识别并利用一种禁用到 PCH 路由的方法。在这种情况下，可以添加一个禁boot 中断生成quirk。[^1^]_

Intel庐 6300ESB I/O Controller Hub
  Alternate Base Address Register锛?   BIE: Boot Interrupt Enable

	  ==  ===========================
	  0   Boot interrupt is enabled.
	  1   Boot interrupt is disabled.
	  ==  ===========================

Intel® Sandy Bridge Sky Lake Xeon 服务器：
  Coherent Interface Protocol Interrupt Control
   dis_intx_route2pch/dis_intx_route2ich/dis_intx_route2dmi2	  当该位被设置时。从 Intel® Quick Data DMA/PCI Express 端口收到的本INTx 消息不会被路由到传统
	  PCH——它们要么通过集成IO-APIC 转换MSI（如果相应表项中IO-APIC 掩码位为清除），
	  要么不引发进一步动作（当掩码位被设置时
在无法直接禁用路由的情况下，另一种方法是利用 PCI 中断引脚INTx 的路由表，以便默认将中断处理程序重定向到重新路由的中断线。因此，在无法禁用此 INTx 路由的芯片组上，Linux 内核会把有效的中断重新路由到其传统中断。这种处理程序的重定向将防止出现伪中断检测，否则该检测会因过多的未处理计数而禁IRQ 线。[^2^]_

配置选项 X86_REROUTE_FOR_BROKEN_BOOT_IRQS 用于启用（或禁用）将中断处理程序重定向到 PCH 中断线。该选项可以pci=ioapicreroute pci=noioapicreroute 覆盖。[^3^]_


## More Documentation


在一些数据手册（下面6300ESB 6700PXH）中有关于传统中断处理的概述。虽然大体相同，但它揭示了其处理随芯片组的演进
### Example of disabling of the boot interrupt


      - Intel庐 6300ESB I/O Controller Hub (Document # 300641-004US)
	5.7.3 Boot Interrupt
	https://www.intel.com/content/dam/doc/datasheet/6300esb-io-controller-hub-datasheet.pdf

      - Intel庐 Xeon庐 Processor E5-1600/2400/2600/4600 v3 Product Families
	Datasheet - Volume 2: Registers (Document # 330784-003)
	6.6.41 cipintrc Coherent Interface Protocol Interrupt Control
	https://www.intel.com/content/dam/www/public/us/en/documents/datasheets/xeon-e5-v3-datasheet-vol-2.pdf

### Example of handler rerouting


      - Intel庐 6700PXH 64-bit PCI Hub (Document # 302628)
	2.15.2 PCI Express Legacy INTx Support and Boot Interrupt
	https://www.intel.com/content/dam/doc/datasheet/6700pxh-64-bit-pci-hub-datasheet.pdf


如果你有任何未解答的传统 PCI 中断问题，请发邮件给我
Cheers,
    Sean V Kelley
    sean.v.kelley@linux.intel.com
