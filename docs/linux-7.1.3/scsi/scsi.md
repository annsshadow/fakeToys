
## SCSI 瀛愮郴缁熸枃妗。

Linux 文档项目（LDP）维护着一份描Linux 内核（lk.4 系列SCSI 子系统的文档。参见：
https://www.tldp.org/HOWTO/SCSI-2.4-HOWTO 。LDP 提供单页和多页的 HTML 版本，以postscript pdf 版本
## SCSI 子系统中使用模块的说
Linux 内核中的 SCSI 支持可以根据最终用户的需要以多种不同的方式进行模块化。要理解你的选项，我们应该先定义几个术语
scsi-core（也称为"中间，mid level）包SCSI 支持的核心。没有它，你无法使用任何其他 SCSI 驱动做任何事情。SCSI 核心支持可以是一个模块（scsi_mod.o），也可以编译进内核。如果核心是模块，它必须是第一个被加载SCSI 模块，而如果你要卸载这些模块，它必须是最后一个被卸载的。在实践中，modprobe rmmod 命令会强制保SCSI 子系统中模块的加载和卸载顺序正确
一SCSI 核心存在于内核中（无论是编译进内核还是作为模块加载），各个上层和下层驱动可以以任意顺序加载。磁盘驱动（sd_mod.o）、CD-ROM 驱动（sr_mod.o）、磁带驱[^1^]_（st.o）和 SCSI 通用驱动（sg.o）代表了用于支持各种可控制设备的上层驱动。例如，你可以加载磁带驱动来使用磁带机，然后在不再需要该驱动时将其卸载（并释放相关内存）
下层驱动是那些支持你所运行硬件平台上各类受支持板卡的驱动。这些单独的板卡通常被称为主机总线适配器（Host Bus Adapters，HBAs）。例aic7xxx.o 驱动用于控制 Adaptec 最近所有的 SCSI 控制器卡。几乎所有的下层驱动都可以构建为模块或编译进内核
       devices. Its module name is osst.o .
