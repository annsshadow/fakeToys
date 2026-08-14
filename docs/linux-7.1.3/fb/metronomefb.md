## Metronomefb


Maintained by Jaya Kumar <jayakumar.lkml.gmail.com>

Last revised: Mar 10, 2008

Metronomefb 是一个用于 Metronome 显示控制器的驱动。该控制器来自 E-Ink Corporation。它旨在用于驱动 E-Ink Vizplex 显示媒体。E-Ink 在其网站上托管了该控制器和显示媒体的一些细节：http://www.e-ink.com/products/matrix/metronome.html 。

Metronome 通过 AMLCD 接口与主机 CPU 连接。主机 CPU 生成控制信息和图像，放在一个 framebuffer 中，随后通过某种主机特定的方式传送到 AMLCD 接口。显示状态和错误状态各自通过独立的 GPIO 拉取。

Metronomefb 是与平台无关的，依赖于一个板级特定的驱动来完成所有物理 IO 工作。目前，针对 AM-200 EPD 开发套件中使用的 PXA 板实现了一个示例。该示例是 am200epd.c。

Metronomefb 需要波形信息，该信息通过 AMLCD 接口传送给 metronome 控制器。波形信息预期通过固件类（firmware class）接口从用户空间传送。只要你的 udev 或 hotplug 脚本知道在传送之前需要解压缩，波形文件就可以被压缩。metronomefb 会请求 metronome.wbf，它通常会根据 udev/hotplug 配置放入 /lib/firmware/metronome.wbf。我只用过一个最初标记为 23P01201_60_WT0107_MTC 的波形文件测试过。我不知道它代表什么含义。操作波形时应谨慎，因为它可能对显示媒体产生某些永久性的影响。我既无法访问也不确切知道该波形对于物理媒体具体起什么作用。

Metronomefb 使用 deferred IO 接口，以便提供一个可内存映射的帧缓冲。它已用 tinyx（Xfbdev）测试过。目前已知它可与 xeyes、xclock、xloadimage、xpdf 一起工作。
