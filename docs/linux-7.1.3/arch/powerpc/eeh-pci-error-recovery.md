## PCI 总线 EEH 错误恢复


Linas Vepstas <linas@austin.ibm.com>

2005 骞?1 鏈?12 鏃。

### 概述
基于 IBM POWER pSeries iSeries 计算机包PCI 总线控制器芯片，这些芯片具有
检测和报告各种 PCI 总线错误条件的扩展能力。这些特性被统称为“EEH”，即“Enhanced
Error Handling（增强错误处理）”。EEH 硬件特性允许清PCI 总线错误并“重启”PCI
卡，而无需重启操作系统
这与传统PCI 错误处理形成对比，在传统方式中，PCI 芯片直接连到 CPU，错误会导致
CPU 的机器检check-stop 状态，完全停止 CPU。另一种“传统”技术是忽略此类错误这可能导致用户数据或内核数据损坏、适配器挂无响应，或系统崩死锁。因此，EEH
背后的理念是：操作系统可以通过免受 PCI 错误的影响而变得更可靠、更健壮，并赋予 OS
“重启恢复单个 PCI 设备的能力
基于 PCI-E 规范的其他厂商的未来系统可能包含类似特性

### EEH 错误的成
EEH 最初设计用于防范硬件故障，例如 PCI 卡因热、湿度、灰尘、振动和不良电气连接
而损坏。在“现实生活”中看到的绝大多EEH 错误是由PCI 卡插接不良，或者（不幸相当常见）由于设备驱bug、设备固bug，以及有PCI 卡硬bug 造成的
最常见的软bug 是，导致设备尝试 DMA 到系统内存中未被预留给该卡进DMA 访问位置。这是一个强大的特性，因为它防止了原本会由错误 DMA 造成的静默内存损坏。过几年中，已通过此方式发现并修复了多个设备驱bug。EEH 错误的其他可能原因包括数据或
地址线奇偶校验错误（例如，由于卡插接不良导致的电气连接不良），以PCI-X
split-completion 错误（由于软件、设备固件或设备 PCI 硬件 bug）。绝大多数“真正的
硬件故障”可以通过物理拔出并重新插PCI 卡来治愈

### 检测与恢复

在接下来的讨论中，将给出如何检测和EEH 错误中恢复的一般概述。随后概Linux
内核中当前实现是如何做的。实际实现可能会发生变化，一些细节仍在讨论中。如果或当其架构实现类似功能时，这些也可能会受到影响
PCI 主桥（PHB，即连接 PCI 总线与系CPU 电子复合体的总线控制器）检测到 PCI
错误条件时，它会“隔离”受影响PCI 卡。隔离会阻塞所有写操作（无论是从系统到卡，
还是从卡到系统），并且会导致所有读操作返回ff（对8/16/32 位读，分别为
0xffxffffxffffffff）。选择这个值是因为它与设备被物理从插槽拔出时会得到的相同。这包括PCI 内存、I/O 空间PCI 配置空间的访问。但是，中断仍会继续投递
检测和恢复是在 ppc64 固件的辅助下进行的。Linux 内核中进入固件的编程接口被称RTAS（Run-Time Abstraction Services，运行时抽象服务）。Linux 内核不（不应）直访问 PCI 芯片组中EEH 功能，主要是因为市场上有许多不同的芯片组，各自具有不同的
接口和怪癖。固件提供了一个统一的抽象层，可与所pSeries iSeries 硬件配合工作
（并且向前兼容）
如果 OS 或设备驱动怀疑某PCI 插槽已被 EEH 隔离，它可以发起一个固件调用来确认是否
如此。如果是，那么设备驱动应使自己进入一致状态（鉴于它无法完成任何挂起的工作）并开恢复该卡。恢复通常包括复位 PCI 设备（将 PCI #RST 线拉高两秒），然后设置设备配置空（基地址寄存器（BAR）、延迟定时器、cache 行大小、中断线等）。随后是设备驱动的重初始化。在最坏情况下，可以切换卡的电源，至少在支持热插拔的插槽上可以。原则上，远设备驱动之上的层可能不需要知PCI 卡已通过这种方式“重启”；理想情况下，在卡被复期间，以太网/磁盘/USB I/O 最多应出现一次暂停
如果卡在三次或四次复位后仍无法恢复，内核/设备驱动应假定最坏情况，即卡已完全损坏，将此错误报告给系统管理员。此外，错误信息通过 RTAS 以及通过 syslogd
var/log/messages）报告，以提醒系统管理员关于 PCI 复位。处理故障适配器的正确方法
是使用标PCI 热插拔工具移除并更换损坏的卡

### 当前 PPC64 Linux EEH 实现

目前，已经实现了一个通用EEH 恢复机制，因此单个设备驱动无需修改即可支持 EEH
恢复。这个通用机制借助 PCI 热插拔基础设施，并通过 userspace/udev 基础设施将事件向传递。以下是其实现方式的详细描述
EEH 必须在引导过程早期、以及在 PCI 插槽被热插拔时，PHB 中启用。前者由
arch/powerpc/platforms/pseries/eeh.c 中的 eeh_init() 执行，后者由
drivers/pci/hotplug/pSeries_pci.c 调用 eeh.c 代码来执行。EEH 必须PCI 扫描设备
之前启用。当前的 Power5 硬件EEH 未启用时无法工作；尽管较旧的 Power4 可以在其
禁用时运行。实际上，EEH 已无法再关闭。PCI 设备**必须**EEH 代码中注册；EEH
代码需要知PCI 设备I/O 地址范围，以便检测错误。给定任意地址，例pci_get_device_by_addr() 将找到与该地址关联pci 设备（如果有）
默认arch/powerpc/include/asm/io.h readb()、inb()、insb() 等包含一项检查，
用于查看 i/o 读是否返回了0xff。如果是，它们会调用 eeh_dn_check_failure()，后再询问固件：ff 值是否是真正 EEH 错误的标志。如果不是，则像正常一样继续处理。这误报或“假阳性”的总数可以/proc/ppc64/eeh 中看到（可能会变更）。通常，几乎所有这些都
发生在引导期间扫PCI 总线时，此时大量 0xff 读是总线扫描过程的一部分
如果检测到冻结的插槽，arch/powerpc/platforms/pseries/eeh.c 中的代码会向 syslog
var/log/messages）打印一个栈跟踪。这个栈跟踪对设备驱动作者非常有用，用于找出何处检测到 EEH 错误，因为错误本身通常发生在稍早之前
接下来，它使Linux 内核notifier 工作队列机制，允许任何相关方了解该故障。设驱动或内核的其他部分可以使用 `eeh_register_notifier(struct notifier_block *)` 了解 EEH 事件。该事件将包含指pci 设备、设备节点和一些状态信息的指针。事件的接收可以“为所欲为”；默认处理程序将在本节进一步描述
为了协助设备恢复，eeh.c 导出了以下函数：

rtas_set_slot_reset()
   灏?PCI #RST 绾挎媺楂?1/8 绉?rtas_configure_bridge()
   请求固件配置位于 pci 插槽拓扑之下的任PCI 桥eeh_save_bars() eeh_restore_bars()   保存和恢复设备及其下任何设备PCI 配置空间信息

EEH notifier_block 事件的处理程序在 drivers/pci/hotplug/pSeries_pci.c 中实现，名为
handle_eeh_events()。它保存设备 BAR，然后调rpaphp_unconfig_pci_adapter()。最后一
次调用会导致该卡的的设备驱动停止，从而向用户空间发出 uevent。这会触发用户空间脚本，
可能发出诸如以太网卡的“ifdown eth0”之类的命令，等等。然后该处理程序休眠 5 秒，希望
给用户空间脚本足够的时间完成。接着它复PCI 卡，重新配置设备 BAR 以及其下的任何桥然后它调rpaphp_enable_pci_slot()，这会重新启动设备驱动并触发更多用户空间事件
（例如，对以太网卡调用“ifup eth0”）

### 设备关闭与用户空间事
本节记录pci 插槽被取消配置时发生的事情，重点关注设备驱动如何被关闭，以及事件
如何投递给用户空间脚本
以下是导致在 EEH 复位第一阶段调用设备驱动 close 函数的一系列事件示例```

    rpa_php_unconfig_pci_adapter (struct slot *)  // in rpaphp_pci.c
    {
      calls
      pci_remove_bus_device (struct pci_dev *) // in /drivers/pci/remove.c
      {
        calls
        pci_destroy_dev (struct pci_dev *)
        {
          calls
          device_unregister (&dev->dev) // in /drivers/base/core.c
          {
            calls
            device_del (struct device *)
            {
              calls
              bus_remove_device() // in /drivers/base/bus.c
              {
                calls
                device_release_driver()
                {
                  calls
                  struct device_driver->remove() which is just
                  pci_device_remove()  // in /drivers/pci/pci_driver.c
                  {
                    calls
                    struct pci_driver->remove() which is just
                    pcnet32_remove_one() // in /drivers/net/pcnet32.c
                    {
                      calls
                      unregister_netdev() // in /net/core/dev.c
                      {
                        calls
                        dev_close()  // in /net/core/dev.c
                        {
                           calls dev->stop();
                           which is just pcnet32_close() // in pcnet32.c
                           {
                             which does what you wanted
                             to stop the device
                           }
                        }
                     }
                   which
                   frees pcnet32 device driver memory
                }
     }}}}}}

```
drivers/pci/pci_driver.c 中，struct device_driver->remove() 就是
pci_device_remove()，它调用 struct pci_driver->remove()，即
pcnet32_remove_one()，后者调unregister_netdev()（在 net/core/dev.c），后者调dev_close()（在 net/core/dev.c），后者调dev->stop()，即 pcnet32_close()，然执行适当的关闭操作
---

以下是发送给用户空间的事件的类似栈跟```

  rpa_php_unconfig_pci_adapter() {             // in rpaphp_pci.c
    calls
    pci_remove_bus_device (struct pci_dev *) { // in /drivers/pci/remove.c
      calls
      pci_destroy_dev (struct pci_dev *) {
        calls
        device_unregister (&dev->dev) {        // in /drivers/base/core.c
          calls
          device_del(struct device * dev) {    // in /drivers/base/core.c
            calls
            kobject_del() {                    //in /libs/kobject.c
              calls
              kobject_uevent() {               // in /libs/kobject.c
                calls
                kset_uevent() {                // in /lib/kobject.c
                  calls
                  kset->uevent_ops->uevent()   // which is really just
                  a call to
                  dev_uevent() {               // in /drivers/base/core.c
                    calls
                    dev->bus->uevent() which is really just a call to
                    pci_uevent () {            // in drivers/pci/hotplug.c
                      which prints device name, etc....
                   }
                 }
                 then kobject_uevent() sends a netlink uevent to userspace
                 --> userspace uevent
                 (during early boot, nobody listens to netlink events and
                 kobject_uevent() executes uevent_helper[], which runs the
                 event process /sbin/hotplug)
             }
           }
           kobject_del() then calls sysfs_remove_dir(), which would
           trigger any user-space daemon that was watching /sysfs,
           and notice the delete event.


```
### 当前设计的优点与缺点

当前EEH 软件恢复设计存在若干问题，可能会在未来的修订中解决。但首先要注意，当前
设计的一大优点是无需对单个设备驱动做任何修改，因此当前设计的覆盖面很广。该设计最大的
缺点是它可能打扰那些本不需要被打扰的网络守护进程和文件系统
- 一个小抱怨是，复位网卡会导致用户空间背靠背的 ifdown/ifup 嗝嗝声，可能打扰那些
   本不需要知pci 卡正在重启的网络守护进程
- 一个更严重的担忧是，同样的复位对于 SCSI 设备会导致挂载的文件系统陷入混乱。脚   无法在事后卸载文件系统而不刷新挂起的缓冲区，但这是不可能的，因I/O 已经停止   因此，理想情况下，复位应该发生在块层或更低层，这样文件系统就不会被打扰
   Ext3fs 似乎具有容忍性，会重试读/写直到成功。两者在此场景下都只经过了轻度测试
   SCSI-generic 子系统已经内置了执行 SCSI 设备复位、SCSI 总线复位SCSI 主总线
   适配器（HBA）复位的代码。如SCSI 命令失败，这些会被级联成一系列尝试的复位。这   完全对块层隐藏。将 EEH 复位添加到这一系列事件中是十分自然的
- 如果根设备发SCSI 错误，一切都将丢失，除非系统管理员有先见之明/binsbin   /etcvar 等放ramdisk/tmpfs 中运行

### 结论

正在取得进展…