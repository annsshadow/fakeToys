# VGA 仲裁


图形设备通过 I/O 或内存空间中的地址范围进行访问。虽然大多数现代设备允许重定位这些范围，但一些在 PCI 上实现的“传统（Legacy）”VGA 设备通常具有与在 ISA 上相同的“硬解码（hard-decoded）”地址。更多细节请参见“PCI Bus Binding to IEEE Std 1275-1994 Standard for Boot (Initialization Configuration) Firmware Revision 2.1”第 7 节“Legacy Devices”

当同一台机器上共存多个传统设备时，X 服务[^0^] 内部的资源访问控制（RAC）模块（除其他总线管理任务外）负责传统VGA 仲裁任务。但当这些设备试图被不同的用户空间客户端（例如两个并行运行的服务器）访问时，问题就出现了：它们的地址分配会发生冲突。此外，理想情况下，作为用户空间应用程序，控制总线资源并不属于 X 服务器的职责。因此，需要在 X 服务器之外有一个仲裁方案来控制这些资源的共享。本文档介绍了为 Linux 内核实现VGA 仲裁器的运作方式

## vgaarb 内核/用户空间 ABI


vgaarb Linux 内核的一个模块。它在初始加载时会扫描所PCI 设备，并将其中的 VGA 设备加入仲裁。随后，仲裁器会在不同设备的 VGA 传统指令上启禁用解码。那些不不需要使用仲裁器的设备可以通过调用 vga_set_legacy_decoding() 显式告知它

内核向客户端导出了一个字符设备接口（/dev/vga_arbiter），其语义如下：

open
        打开仲裁器的一个用户实例。默认情况下，它附加到系统的默认 VGA 设备

close
        关闭一个用户实例。释放该用户持有的锁

read
        返回一个指示目标状态的字符串，例如

        "<card_ID>,decodes=<io_state>,owns=<io_state>,locks=<io_state> (ic,mc)"

        IO 状态字符串的形式为 {io,mem,io+mem,none}，mc ic 分别是内存和 IO 的锁计数（仅用于调试/诊断）。“decodes表示显卡当前解码的内容，“owns表示当前在其上启用的内容，“locks表示被此显卡锁定的内容。如果显卡被拔出，则 card_ID 处会返回 “invalid”，并且对于任何命令都会返回 -ENODEV 错误，直到有新的显卡成为目标


write
        向仲裁器写入一条命令。命令列表如下：

        target <card_ID>
                switch target to card <card_ID> (see below)
        lock <io_state>
                acquires locks on target ("none" is an invalid io_state)
        trylock <io_state>
                non-blocking acquire locks on target (returns EBUSY if
                unsuccessful)
        unlock <io_state>
                release locks on target
        unlock all
                release all locks on target held by this user (not implemented
                yet)
        decodes <io_state>
                set the legacy decoding attributes for the card

        poll
                当任何显卡（而不仅是目标）发生变化时产生事件

        card_ID 的形式为 “PCI:domain:bus:dev.fn”。可以将其设“default以回到系统默认显卡（TODO：尚未实现）。目前仅支持PCI 作为前缀，但即便当前内核实现不支持，用户API 未来也可能支持其他总线类型

## 关于锁的说明

驱动会跟踪哪个用户在哪个显卡上持有哪些锁。它支持嵌套（stacking），类似于内核的实现。这使实现稍微复杂了一些，但使仲裁器对用户空间问题更具容错性，并能在进程死亡的所有情况下正确清理。目前，对于给定的用户（文件描述符实例）而言，最多可以有 16 张显卡同时持有来自用户空间的锁

在设备热插拔（hot-{un,}plugged）的情况下，有一个钩子——pci_notify()——用于通知它们被加移出系统，并自动在仲裁器中加移除

如果 DRM、vgacon 或其他驱动希望使用仲裁器，还提供了一个内核内的仲裁器 API

## 鍐呮牳鍐呮帴鍙。


## :internal:

## :export:

## libpciaccess


为了使用 vgaarb 字符设备，在 libpciaccess 库中实现了一API。向 struct pci_device（每个设
```

    /* the type of resource decoded by the device */
    int vgaarb_rsrc;

```
```

    int vgaarb_fd;
    int vga_count;
    struct pci_device *vga_target;
    struct pci_device *vga_default_dev;

```
vga_count 用于跟踪正在被仲裁的显卡数量，因此例如，如果只有一张显卡，那么它就可以完全避开仲裁

下面这些函数会为给定显卡获取 VGA 资源，并将这些资源标记为已锁定。如果所请求的资源是“普通”（而非传统）资源，仲裁器将首先检查该显卡是否正在对该类型资源进行传统解码。如果是，则该锁会被“转换”为传统资源锁。仲裁器将首先查找所有可能冲突的 VGA 显卡，并禁用它们IO 或内存访问（必要时包P2P 桥上VGA 转发），以便所请求的资源可以使用。然后，该显卡被标记为锁定这些资源，并在其上启用 IO 或内存访问（如果有的话，包括P2P 桥上VGA 转发）。在 vga_arb_lock() 的情况下，如果某个冲突的显卡已经锁定了某个所需资源（或不同总线段上的任何资源，因为据我所P2P 桥不会区VGA 内存IO），该函数会阻塞。如果显卡已经拥有这些资源，则函数成功。vga_arb_trylock() 会返(-EBUSY) 而不是阻塞。支持嵌套调用（维护一个每资源的计数器）

```

    int  pci_device_vgaarb_set_target   (struct pci_device *dev);

```
例如，在 x86 上，如果同一总线上的两个设备想要锁定不同的资源，两者都会成功（锁定）。如果设备位于不同的总线上，
```

    int  pci_device_vgaarb_lock         (void);
    int  pci_device_vgaarb_trylock      (void);

```
```

    int  pci_device_vgaarb_unlock       (void);

```
向仲裁器指示该显卡是否解码传VGA IO、传VGA 内存、两者，还是都不解码。所有显卡默认两者都解码，显卡驱动（例如 fbdev）应告知仲裁器它是否已禁用传统解码，以便该显卡可以被排除在仲裁过程之外（并且可以安全地占
```

    int  pci_device_vgaarb_decodes      (int new_vgaarb_rsrc);

```
```

    int  pci_device_vgaarb_init         (void);

```
```

    void pci_device_vgaarb_fini         (void);

```
xf86VGAArbiter（X 服务器实现）


X 服务器基本上包装了所有以某种方式触及 VGA 寄存器的函数

## 参考资


Benjamin Herrenschmidt（IBM）在 2005 年与 Xorg 社区讨论这种设计时启动了这项工作 [1, 2]007 年底，Paulo Zanoni Tiago Vignatti（均来自 C3SL/巴拉那联邦大学）继续了他的工作，增强了内核代码以适配为一个内核模块，并完成了用户空间一侧的实现 [^3^]。如今（2009 年），Tiago Vignatti Dave Airlie 最终将这项工作整理成型，并排入 Jesse Barnes PCI 树中

0) https://cgit.freedesktop.org/xorg/xserver/commit/?id=4b42448a2388d40f257774fbffdccaea87bd0347
1) https://lists.freedesktop.org/archives/xorg/2005-March/006663.html
2) https://lists.freedesktop.org/archives/xorg/2005-March/006745.html
3) https://lists.freedesktop.org/archives/xorg/2007-October/029507.html
