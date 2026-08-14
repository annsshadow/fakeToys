## OHCI


23-Aug-2002

"ohci-hcd" 驱动是一个 USB 主机控制器驱动（HCD），派生自 2.4 内核系列的 "usb-ohci" 驱动。"usb-ohci" 代码主要由 Roman Weissgaerber <weissg@vienna.at> 编写，但也有许多其他人的贡献（请阅读其版权/许可头部）。

它支持 "Open Host Controller Interface"（OHCI），该接口标准化了用于与 USB 1.1 主机控制器通信的硬件寄存器协议。与 Intel 较早的 "Universal Host Controller Interface"（UHCI）相比，它将更多智能推入硬件。除 Intel 和 VIA 之外的厂商的 USB 1.1 控制器通常使用 OHCI。

自 2.4 内核以来的变化包括

 - 改进的健壮性；bug 修复；以及更少的开销
 - 支持更新且简化的 usbcore API
 - 中断传输可以更大，并且可以排队
 - 通过使用上层的 "hcd" 框架减少了代码量
 - 支持 OHCI 的一些非 PCI 实现
 - ……更多

"ohci-hcd" 驱动处理所有 USB 1.1 传输类型。所有类型的传输都可以排队。在 "usb-ohci" 中也是如此，中断传输除外。以前，使用一个帧的周期会因 IRQ 处理中的开销而带来数据丢失的风险。当中断传输被排队时，通过确保硬件在操作系统处理相关 IRQ 期间始终有传输任务可执行，可以将这些风险降至最低。

- David Brownell
  <dbrownell@users.sourceforge.net>
