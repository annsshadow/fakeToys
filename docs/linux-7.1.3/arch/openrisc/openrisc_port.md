## OpenRISC Linux

这是将 Linux 移植到 OpenRISC 系列微处理器的成果；具体而言，最初的目标架构是
32 位的 OpenRISC 1000 家族（or1k）。

有关 OpenRISC 处理器和持续开发的信息：

	=======		==============================
	website		https://openrisc.io
	email		linux-openrisc@vger.kernel.org
	=======		==============================

---------------------------------------------------------------------

## OpenRISC 工具链与 Linux 的构建说明

为了构建并运行 OpenRISC 上的 Linux，你至少需要一个基本的工具链，可能还需要
架构模拟器。此处概述了让这些组件就位所需的步骤。

1) 工具链

工具链二进制文件可以从 openrisc.io 或我们的 github releases 页面获取。构建不同
工具链的说明可以在 openrisc.io 或 Stafford 的工具链构建与发布脚本中找到。

	==========	==========================================================
	binaries	https://github.com/stffrdhrn/or1k-toolchain-build/releases
	toolchains	https://openrisc.io/software
	building	https://github.com/stffrdhrn/or1k-toolchain-build
	==========	==========================================================

2) 构建

```

	make ARCH=openrisc CROSS_COMPILE="or1k-linux-" defconfig
	make ARCH=openrisc CROSS_COMPILE="or1k-linux-"

```
```

	make ARCH=openrisc CROSS_COMPILE="or1k-linux-" CONFIG_INITRAMFS_SOURCE="path/to/rootfs path/to/devnodes"

```
关于此处的更多信息，请参阅 Documentation/filesystems/ramfs-rootfs-initramfs.rst。

3) 在 FPGA 上运行（可选）

OpenRISC 社区通常使用 FuseSoC 来管理将 SoC 构建并烧录到 FPGA 中。下面是将
OpenRISC SoC 烧录到 De0 Nano 开发板的示例。在构建过程中，FPGA RTL 代码会从
FuseSoC IP 核仓库下载，并使用 FPGA 厂商工具构建。二进制文件通过 openocd 加载到
板卡上。

```

	git clone https://github.com/olofk/fusesoc
	cd fusesoc
	sudo pip install -e .

	fusesoc init
	fusesoc build de0_nano
	fusesoc pgm de0_nano

	openocd -f interface/altera-usb-blaster.cfg \
		-f board/or1k_generic.cfg

	telnet localhost 4444
	> init
	> halt; load_image vmlinux ; reset

```
4) 在模拟器上运行（可选）

QEMU 是一个处理器模拟器，我们推荐用它来模拟 OpenRISC 平台。请按照 QEMU 网站上的
OpenRISC 说明来在 QEMU 上运行 Linux。你可以自己构建 QEMU，但你的 Linux 发行版很可能
提供了支持 OpenRISC 的二进制包。

	=============	======================================================
	qemu openrisc	https://wiki.qemu.org/Documentation/Platforms/OpenRISC
	=============	======================================================

---------------------------------------------------------------------

## 术语

在代码中，符号上使用以下“粒子（particle）”来将范围限定为或多或少特定的处理器
实现：

========= =======================================
openrisc:  OpenRISC 系列处理器
or1k:      OpenRISC 1000 家族处理器
or1200:    OpenRISC 1200 处理器
========= =======================================

---------------------------------------------------------------------

## 历史

18-11-2003	Matjaz Breskvar (phoenix@bsemi.com)
	将 linux 初步移植到 OpenRISC/or32 架构。
        所有核心部分都已实现，看起来可用。

08-12-2003	Matjaz Breskvar (phoenix@bsemi.com)
	完全改变了 TLB miss 的处理方式。
	重写了异常处理。
	默认 initrd 中具备完整可用的 sash-3.6。
	一个各方面都有很大改进的版本。

10-04-2004	Matjaz Breskvar (phoenix@bsemi.com)
	大量的 bug 修复。
	以太网支持，可用的 http 和 telnet 服务器。
	运行许多标准 linux 应用。

26-06-2004	Matjaz Breskvar (phoenix@bsemi.com)
	移植到 2.6.x。

30-11-2004	Matjaz Breskvar (phoenix@bsemi.com)
	大量 bug 修复与增强。
	添加了 opencores 帧缓冲驱动。

09-10-2010    Jonas Bonn (jonas@southpole.se)
	重大重写，以与上游 Linux 2.6.36 看齐
