
## Early Printk


在 x86 系统上使用 earlyprintk=dbgp 启动选项配合 USB2 Debug 端口密钥和调试线缆的 Mini-HOWTO。

你需要两台电脑、“USB debug key”专用小设备以及
```

  [host/target] <-------> [USB debug key] <-------> [client/console]

```
## Hardware requirements


  a) 主机/目标系统需要具备 USB debug 端口能力。

     你可以通过查看 'Debug port' 位来检查该能力，方法是运行
```

       # lspci -vvv
       ...
       00:1d.7 USB Controller: Intel Corporation 82801H (ICH8 Family) USB2 EHCI Controller #1 (rev 03) (prog-if 20 [EHCI])
               Subsystem: Lenovo ThinkPad T61
               Control: I/O- Mem+ BusMaster+ SpecCycle- MemWINV- VGASnoop- ParErr- Stepping- SERR+ FastB2B- DisINTx-
               Status: Cap+ 66MHz- UDF- FastB2B+ ParErr- DEVSEL=medium >TAbort- <TAbort- <MAbort- >SERR- <PERR- INTx-
               Latency: 0
               Interrupt: pin D routed to IRQ 19
               Region 0: Memory at fe227000 (32-bit, non-prefetchable) [size=1K]
               Capabilities: [50] Power Management version 2
                       Flags: PMEClk- DSI- D1- D2- AuxCurrent=375mA PME(D0+,D1-,D2-,D3hot+,D3cold+)
                       Status: D0 PME-Enable- DSel=0 DScale=0 PME+
               Capabilities: [58] Debug port: BAR=1 offset=00a0
                            ^^^^^^^^^^^ <==================== [ HERE ]
               Kernel driver in use: ehci_hcd
               Kernel modules: ehci-hcd
       ...

     .. note::
       如果你的系统没有列出 debug 端口能力，那么你可能无法使用 USB debug key。

  b) 你还需要一个 NetChip USB 调试线缆/密钥：

        http://www.plxtech.com/products/NET2000/NET20DC/default.asp

     这是一个带两个 USB 接口的小巧蓝色塑料连接器；它从 USB 接口取电。

  c) 你需要第二台带高速 USB 2.0 端口的 client/console 系统。

  d) NetChip 设备必须直接插入“主机/目标”系统上的物理 debug 端口。不能在物理 debug 端口与“主机/目标”系统之间使用 USB 集线器。

     EHCI 调试控制器绑定到特定的物理 USB 端口，NetChip 设备只能在该端口中作为 early printk 设备工作。EHCI 主机控制器的电气布线使得 EHCI 调试控制器连接到第一个物理端口，并且无法通过软件更改这一点。你可以通过逐个尝试系统上每个物理端口并重启来发现物理端口。或者你也可以尝试使用 lsusb，或查看把 USB 设备插入“主机/目标”系统各个端口时 usb 协议栈发出的内核信息消息。

     一些硬件厂商没有用物理连接器暴露 usb debug 端口，如果你发现这样的设备，请向硬件厂商投诉，因为没有理由不把该端口接到某个物理可访问的端口上。

  e) 同样重要的是，许多版本的 NetChip 设备要求“client/console”系统插入设备的右侧（产品 logo 朝上面向，从左到右可读）。原因是 5 伏电源只从该设备的一侧取电，且必须是不会被重启的那一侧。

```
## Software requirements


  a) 在主机/目标系统上：

```

      CONFIG_EARLY_PRINTK_DBGP=y

    并且你需要添加启动命令行："earlyprintk=dbgp"。

    .. note::
      如果你使用 Grub，请将其追加到 /etc/grub.conf 的 'kernel' 行。如果你在 BIOS 固件系统上使用 Grub2，请将其追加到 /boot/grub2/grub.cfg 的 'linux' 行。如果你在 EFI 固件系统上使用 Grub2，请将其追加到 /boot/grub2/grub.cfg 或 /boot/efi/EFI/<distro>/grub.cfg 的 'linux' 或 'linuxefi' 行。

    在有多个 EHCI 调试控制器的系统上，你必须指定正确的 EHCI 调试控制器编号。其顺序来自 EHCI 控制器的 PCI 总线枚举。不带编号参数的默认值是 "0"，即第一个 EHCI 调试控制器。要使用第二个 EHCI 调试控制器，你可以使用命令行："earlyprintk=dbgp1"

    .. note::
      通常 earlyprintk 控制台在常规控制台就绪后会被关闭——使用 "earlyprintk=dbgp,keep" 可以在早期启动之后保持该通道打开。这对调试 Xorg 下的崩溃等场景很有用。

  b) 在 client/console 系统上：

    你应该启用以下内核配置选项：：

      CONFIG_USB_SERIAL_DEBUG=y

    下一次使用修改后的内核启动时，你应该会得到一个 /dev/ttyUSBx 设备（或多个）。

    现在这个内核消息通道已准备好使用：启动你喜欢的终端仿真器（minicom 等）并将其配置为使用 /dev/ttyUSB0——或者使用原始的 'cat /dev/ttyUSBx' 来查看原始输出。

  c) 在基于 Nvidia 南桥的系统上：内核会尝试探测并找出哪个端口连接了调试设备。

```
## Testing


你可以通过使用 earlyprintk=dbgp,keep 并在主机/目标系统上触发内核消息来测试输出。你可以触发一个无害的
```

     echo h > /proc/sysrq-trigger

```

```

     SysRq : HELP : loglevel(0-9) reBoot Crashdump terminate-all-tasks(E) memory-full-oom-kill(F) kill-all-tasks(I) saK show-backtrace-all-active-cpus(L) show-memory-usage(M) nice-all-RT-tasks(N) powerOff show-registers(P) show-all-timers(Q) unRaw Sync show-task-states(T) Unmount show-blocked-tasks(W) dump-ftrace-buffer(Z)

```

```

       cat /dev/ttyUSB0

```
在你于主机系统上触发后，应该很快就能看到上面的帮助行。

如果它不工作，请在 linux-kernel@vger.kernel.org 邮件列表上询问，或联系 x86 维护者。
