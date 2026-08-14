# Linux 内核项目概览

> 由源码树 `D:\WORKSPACE\linux-7.1.3` 生成

---

# 目录结构

## arch/

特定于体系结构的代码（arm64、x86、riscv、m68k、powerpc 等）以及引导基础设施。

- `alpha/` — # alpha/Makefile #
- `arc/` — SPDX-License-Identifier: GPL-2.0-only # # Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
- `arm/` — # arch/arm/Makefile #
- `arm64/` — # arch/arm64/Makefile #
- `csky/` — SPDX-License-Identifier: GPL-2.0-only
- `hexagon/` — SPDX-License-Identifier: GPL-2.0#  用于Hexagon arch 的 Makefile# Do not use GP-relative jumps
- `loongarch/` — SPDX-License-Identifier: GPL-2.0 # # Author: Huacai Chen <chenhuacai@loongson.cn>
- `m68k/` — # m68k/Makefile #
- `microblaze/` — SPDX-License-Identifier: GPL-2.0# 我们正在为哪个 CPU 版本构建，并拆解它 # 形式为 major.minor.rev
- `mips/` — # This file is subject to the terms and conditions of the GNU General Public # License.  See the file "COPYING" in the main directory of this archive
- `nios2/` — # This file is subject to the terms and conditions of the GNU General Public # License.  See the file "COPYING" in the main directory of this archive
- `openrisc/` — BK Id: %F% %I% %G% %U% %#% # # This file is included by the global makefile so that you can add your own
- `parisc/` — # parisc/Makefile #
- `powerpc/` — 本文件由全局 makefile 包含，以便你可以添加自己的 # 体系结构特定标志和依赖项。#
- `riscv/` — 本文件由全局 makefile 包含，以便你可以添加自己的 # 体系结构特定标志和依赖项。#
- `s390/` — SPDX-License-Identifier: GPL-2.0# # s390/Makefile
- `sh/` — # arch/sh/Makefile #
- `sparc/` — SPDX-License-Identifier: GPL-2.0# # sparc/Makefile
- `um/` — # 本文件由全局 makefile 包含，以便你可以添加自己的 # 体系结构特定标志和依赖项。
- `x86/` — SPDX-License-Identifier: GPL-2.0# 用于 i386 和 x86_64 的统一 Makefile # 根据实际架构选择 defconfig
- `xtensa/` — # This file is subject to the terms and conditions of the GNU General Public # License.  See the file "COPYING" in the main directory of this archive

## crypto/

加密 API 与算法实现。

- `asymmetric_keys/` — SPDX-License-Identifier: GPL-2.0# # 用于非对称加密密钥 的 Makefile
- `async_tx/` — SPDX-License-Identifier: GPL-2.0
- `krb5/` — SPDX-License-Identifier: GPL-2.0# # 用于非对称加密密钥 的 Makefile

## drivers/

设备驱动（网络、块设备、字符设备、声卡、GPU、USB、PCI、infiniband 等）以及驱动核心。

- `accel/` — SPDX-License-Identifier: GPL-2.0-only
- `accessibility/` — SPDX-License-Identifier: GPL-2.0-only
- `acpi/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux ACPI 解释器 的 Makefile
- `amba/` — SPDX-License-Identifier: GPL-2.0-only
- `android/` — SPDX-License-Identifier: GPL-2.0-only
- `ata/` — SPDX-License-Identifier: GPL-2.0# 非 SFF 接口
- `atm/` — SPDX-License-Identifier: GPL-2.0
- `auxdisplay/` — SPDX-License-Identifier: GPL-2.0# # 用于内核辅助显示设备驱动程序 的 Makefile.
- `base/` — SPDX-License-Identifier: GPL-2.0# 用于Linux 设备树 的 Makefile
- `bcma/` — Broadcom 推出了一种新的总线来替代较旧的 SSB。它基于 AMBA，但从编程角度看，我们并没有使用任何 AMBA 特定的内容。标准的 AMBA 驱动是平台特定的，具有硬编码地址，并使用诸如 CID 和 PID 之类的 AMBA 标准字段。在 Broadcom 的网卡中，每个设备由以下部分组成：1) Broadcom 特定的 AMBA 设备。它被放在 AMBA 总线上，但不能作为标准 AMBA 设备处理。读取其 CID 或 PID 可能导致机器死锁。2) AMBA s...
- `block/` — SPDX-License-Identifier: GPL-2.0# # 用于内核块设备驱动程序 的 Makefile.
- `bluetooth/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux Bluetooth HCI 设备驱动程序 的 Makefile.
- `bus/` — SPDX-License-Identifier: GPL-2.0# # 用于总线驱动程序 的 Makefile.
- `cache/` — SPDX-License-Identifier: GPL-2.0
- `cdrom/` — SPDX-License-Identifier: GPL-2.0
- `cdx/` — SPDX-License-Identifier: GPL-2.0# # 用于CDX 的 Makefile
- `char/` — SPDX-License-Identifier: GPL-2.0# # 用于内核字符设备驱动程序 的 Makefile.
- `clk/` — SPDX-License-Identifier: GPL-2.0# 通用时钟类型
- `clocksource/` — SPDX-License-Identifier: GPL-2.0
- `comedi/` — SPDX-License-Identifier: GPL-2.0
- `connector/` — SPDX-License-Identifier: GPL-2.0-only
- `counter/` — SPDX-License-Identifier: GPL-2.0-only# # 用于计数器设备 的 Makefile
- `cpufreq/` — SPDX-License-Identifier: GPL-2.0# CPUfreq 核心 # CPUfreq 统计
- `cpuidle/` — SPDX-License-Identifier: GPL-2.0# # 用于cpuidle 的 Makefile.
- `crypto/` — SPDX-License-Identifier: GPL-2.0# __init ordering requires atmel-i2c being before atmel-ecc and atmel-sha204a.
- `cxl/` — SPDX-License-Identifier: GPL-2.0# Order is important here for the built-in case: # - 'core' first for fundamental init
- `dax/` — SPDX-License-Identifier: GPL-2.0
- `dca/` — SPDX-License-Identifier: GPL-2.0-only
- `devfreq/` — SPDX-License-Identifier: GPL-2.0# DEVFREQ 驱动 # DEVFREQ 事件驱动
- `dibs/` — SPDX-License-Identifier: GPL-2.0# # DIBS class module
- `dio/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux 内核 的 Makefile.
- `dma/` — SPDX-License-Identifier: GPL-2.0#dmaengine 调试标志 #core
- `dma-buf/` — SPDX-License-Identifier: GPL-2.0-only
- `dpll/` — SPDX-License-Identifier: GPL-2.0# # 用于DPLL drivers 的 Makefile.
- `edac/` — # 用于 Linux 内核 EDAC 驱动程序的 Makefile。#
- `eisa/` — SPDX-License-Identifier: GPL-2.0# 用于Linux 设备树 的 Makefile# virtual_root.o should be the last EISA root device to initialize,
- `extcon/` — SPDX-License-Identifier: GPL-2.0# 用于external connector class (extcon) devices 的 Makefile#
- `firewire/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux IEEE 1394 实现 的 Makefile
- `firmware/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux 内核 的 Makefile.
- `fpga/` — SPDX-License-Identifier: GPL-2.0# # 用于fpga 框架和 fpga 管理器驱动程序 的 Makefile.
- `fsi/` — SPDX-License-Identifier: GPL-2.0-only
- `fwctl/` — SPDX-License-Identifier: GPL-2.0
- `gnss/` — SPDX-License-Identifier: GPL-2.0# # 用于GNSS 子系统 的 Makefile.
- `gpib/`
- `gpio/` — SPDX-License-Identifier: GPL-2.0# 通用 gpio 支持：平台驱动、专用扩展器芯片等 # 设备驱动。通常保持列表按字母排序
- `gpu/` — SPDX-License-Identifier: GPL-2.0-only# drm/tegra depends on host1x, so if both drivers are built-in care must be # taken to initialize them in the correct order. Link order is the only way
- `greybus/` — SPDX-License-Identifier: GPL-2.0# Greybus 核心 # 跟踪事件所需
- `hid/` — SPDX-License-Identifier: GPL-2.0# # 用于HID 驱动 的 Makefile
- `hsi/` — SPDX-License-Identifier: GPL-2.0# # 用于HSI 的 Makefile
- `hte/`
- `hv/` — SPDX-License-Identifier: GPL-2.0
- `hwmon/` — SPDX-License-Identifier: GPL-2.0# # 用于sensor chip drivers 的 Makefile.
- `hwspinlock/` — SPDX-License-Identifier: GPL-2.0# # Generic Hardware Spinlock framework
- `hwtracing/`
- `i2c/` — SPDX-License-Identifier: GPL-2.0# # 用于i2c 核心 的 Makefile.
- `i3c/` — SPDX-License-Identifier: GPL-2.0
- `idle/` — SPDX-License-Identifier: GPL-2.0-only# Branch profiling isn't noinstr-safe
- `iio/` — SPDX-License-Identifier: GPL-2.0# # 用于工业 I/O 核心 的 Makefile.
- `infiniband/` — SPDX-License-Identifier: GPL-2.0-only
- `input/` — SPDX-License-Identifier: GPL-2.0# # 用于输入核心驱动 的 Makefile.
- `interconnect/` — SPDX-License-Identifier: GPL-2.0
- `iommu/` — SPDX-License-Identifier: GPL-2.0
- `ipack/` — SPDX-License-Identifier: GPL-2.0-only# # 用于IPACK 桥接设备驱动程序 的 Makefile.
- `irqchip/` — SPDX-License-Identifier: GPL-2.0
- `leds/` — SPDX-License-Identifier: GPL-2.0# LED 核心 # LED 平台驱动（保持排序，M-| sort）
- `macintosh/` — SPDX-License-Identifier: GPL-2.0# # 用于Macintosh 特定设备驱动程序 的 Makefile.
- `mailbox/` — SPDX-License-Identifier: GPL-2.0# 通用 MAILBOX API
- `mcb/` — SPDX-License-Identifier: GPL-2.0
- `md/` — SPDX-License-Identifier: GPL-2.0# # 用于内核软件 RAID 和 LVM 驱动程序 的 Makefile.
- `media/` — SPDX-License-Identifier: GPL-2.0# # 用于内核多媒体设备驱动程序 的 Makefile.
- `memory/` — SPDX-License-Identifier: GPL-2.0# # 用于内存设备 的 Makefile
- `memstick/` — SPDX-License-Identifier: GPL-2.0-only# # 用于内核 MemoryStick 设备驱动程序 的 Makefile.
- `message/` — SPDX-License-Identifier: GPL-2.0-only# # 用于基于 MPT 的块设备 的 Makefile
- `mfd/` — SPDX-License-Identifier: GPL-2.0# # 用于多功能杂项设备 的 Makefile
- `misc/` — SPDX-License-Identifier: GPL-2.0# # 用于确实无处安放的杂项设备 的 Makefile.
- `mmc/` — SPDX-License-Identifier: GPL-2.0-only# # 用于内核 mmc 设备驱动程序 的 Makefile.
- `most/` — SPDX-License-Identifier: GPL-2.0
- `mtd/` — SPDX-License-Identifier: GPL-2.0# # 用于存储器技术设备驱动程序 的 Makefile.
- `mux/` — SPDX-License-Identifier: GPL-2.0# # 用于多路复用器设备 的 Makefile.
- `net/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux 网络设备驱动程序 的 Makefile.
- `nfc/` — SPDX-License-Identifier: GPL-2.0# # 用于nfc 设备 的 Makefile
- `ntb/` — SPDX-License-Identifier: GPL-2.0-only
- `nubus/` — SPDX-License-Identifier: GPL-2.0-only# # 用于nubus 特定驱动程序 的 Makefile.
- `nvdimm/` — SPDX-License-Identifier: GPL-2.0
- `nvme/` — SPDX-License-Identifier: GPL-2.0-only
- `nvmem/` — SPDX-License-Identifier: GPL-2.0# # 用于nvmem drivers 的 Makefile.
- `of/` — SPDX-License-Identifier: GPL-2.0
- `opp/` — SPDX-License-Identifier: GPL-2.0-only
- `parisc/` — /* ** HP VISUALIZE 工作站 PCI 总线缺陷 ** ** “HP 发现了一个潜在的系统缺陷，可能影响 ** 五款 HP VISUALIZE 工作站机型在配备 ** 第三方或客户自行安装的 PCI I/O 扩展卡时的行为。 ** 该缺陷仅限于 HP C180、C160、C160L、B160L 和 ** B132L VISUALIZE 工作站，且只会在 ** 通过 PCI 总线上的 PCI I/O 扩展卡传输数据时出现。 ** HP 提供的显卡如果...
- `parport/` — SPDX-License-Identifier: GPL-2.0# # 用于内核并行端口设备驱动程序 的 Makefile.
- `pci/` — SPDX-License-Identifier: GPL-2.0# # 用于PCI 总线特定驱动程序 的 Makefile.
- `pcmcia/` — SPDX-License-Identifier: GPL-2.0# # 用于内核 pcmcia 子系统（由 David Hinds 维护） 的 Makefile
- `peci/` — SPDX-License-Identifier: GPL-2.0-only# Core functionality # Hardware specific bus drivers
- `perf/` — SPDX-License-Identifier: GPL-2.0
- `phy/` — SPDX-License-Identifier: GPL-2.0# # 用于phy 驱动 的 Makefile.
- `pinctrl/` — SPDX-License-Identifier: GPL-2.0# 通用 pinmux 支持
- `platform/` — SPDX-License-Identifier: GPL-2.0# # 用于linux/drivers/platform 的 Makefile
- `pmdomain/` — SPDX-License-Identifier: GPL-2.0-only
- `pnp/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux 即插即用支持 的 Makefile.
- `power/` — SPDX-License-Identifier: GPL-2.0-only
- `powercap/` — SPDX-License-Identifier: GPL-2.0-only
- `pps/` — SPDX-License-Identifier: GPL-2.0-only# # 用于PPS 核心 的 Makefile.
- `ps3/` — SPDX-License-Identifier: GPL-2.0-only
- `ptp/` — SPDX-License-Identifier: GPL-2.0# # 用于PTP 1588 时钟支持 的 Makefile.
- `pwm/` — SPDX-License-Identifier: GPL-2.0
- `rapidio/` — SPDX-License-Identifier: GPL-2.0# # 用于RapidIO interconnect services 的 Makefile
- `ras/` — SPDX-License-Identifier: GPL-2.0-only
- `regulator/` — SPDX-License-Identifier: GPL-2.0# # 用于调节器驱动程序 的 Makefile.
- `remoteproc/` — SPDX-License-Identifier: GPL-2.0# # Generic framework for controlling remote processors
- `resctrl/`
- `reset/` — SPDX-License-Identifier: GPL-2.0
- `rpmsg/` — SPDX-License-Identifier: GPL-2.0
- `rtc/` — SPDX-License-Identifier: GPL-2.0# # 用于RTC 类/驱动程序 的 Makefile.
- `s390/` — SPDX-License-Identifier: GPL-2.0# # 用于S/390 特定设备驱动程序 的 Makefile
- `sbus/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux 内核 的 Makefile.
- `scsi/` — SPDX-License-Identifier: GPL-2.0# # 用于linux/drivers/scsi 的 Makefile
- `sh/` — SPDX-License-Identifier: GPL-2.0# # 用于SuperH 特定驱动程序 的 Makefile.
- `siox/` — SPDX-License-Identifier: GPL-2.0-only
- `slimbus/` — SPDX-License-Identifier: GPL-2.0# # 用于内核 SLIMbus 框架 的 Makefile.
- `soc/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux 内核 SOC 特定设备驱动程序 的 Makefile.
- `soundwire/` — SPDX-License-Identifier: GPL-2.0-only# # 用于soundwire 核心 的 Makefile
- `spi/` — SPDX-License-Identifier: GPL-2.0# # 用于内核 SPI 驱动程序 的 Makefile.
- `spmi/` — SPDX-License-Identifier: GPL-2.0-only# # 用于内核 SPMI 框架 的 Makefile.
- `ssb/` — SPDX-License-Identifier: GPL-2.0# 核心 # 主机支持
- `staging/` — SPDX-License-Identifier: GPL-2.0# 用于staging directory 的 Makefile
- `target/` — SPDX-License-Identifier: GPL-2.0
- `tc/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux 内核 的 Makefile.
- `tee/` — SPDX-License-Identifier: GPL-2.0
- `thermal/` — SPDX-License-Identifier: GPL-2.0# # 用于sensor chip drivers 的 Makefile.
- `thunderbolt/` — SPDX-License-Identifier: GPL-2.0-only
- `tty/` — SPDX-License-Identifier: GPL-2.0# tty 驱动
- `ufs/` — SPDX-License-Identifier: GPL-2.0# The link order is important here. ufshcd-core must initialize # before vendor drivers.
- `uio/` — SPDX-License-Identifier: GPL-2.0
- `usb/` — SPDX-License-Identifier: GPL-2.0# # 用于内核 USB 设备驱动程序 的 Makefile.
- `vdpa/` — SPDX-License-Identifier: GPL-2.0
- `vfio/` — SPDX-License-Identifier: GPL-2.0
- `vhost/` — SPDX-License-Identifier: GPL-2.0
- `video/` — SPDX-License-Identifier: GPL-2.0
- `virt/` — SPDX-License-Identifier: GPL-2.0-only# # 用于支持虚拟化的驱动程序 的 Makefile
- `virtio/` — SPDX-License-Identifier: GPL-2.0
- `w1/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Dallas 1-wire 总线 的 Makefile.
- `watchdog/` — SPDX-License-Identifier: GPL-2.0# # 用于WatchDog 设备驱动程序 的 Makefile.
- `xen/` — SPDX-License-Identifier: GPL-2.0
- `zorro/` — SPDX-License-Identifier: GPL-2.0# # 用于Zorro 总线特定驱动程序 的 Makefile.

## fs/

文件系统（ext4、btrfs、xfs、fuse、overlayfs、nfs、jffs2、cramfs 等）。

- `9p/` — SPDX-License-Identifier: GPL-2.0
- `adfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux adfs 文件系统例程 的 Makefile.
- `affs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux affs 文件系统例程 的 Makefile.
- `afs/` — SPDX-License-Identifier: GPL-2.0# # 用于Red Hat Linux AFS client 的 Makefile.
- `autofs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux autofs 文件系统例程 的 Makefile.
- `befs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux BeOS 文件系统例程 的 Makefile.
- `bfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于BFS 文件系统 的 Makefile.
- `btrfs/` — SPDX-License-Identifier: GPL-2.0# W=1 警告的子集 # 以下关闭由 -Wextra 启用的警告
- `cachefiles/` — SPDX-License-Identifier: GPL-2.0# # 用于在已挂载文件系统中进行缓存 的 Makefile
- `ceph/` — SPDX-License-Identifier: GPL-2.0# # 用于CEPH filesystem 的 Makefile.
- `coda/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux Coda 文件系统例程 的 Makefile.
- `configfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于configfs 虚拟文件系统 的 Makefile
- `cramfs/` — 文件系统布局说明 --------------------------  这些说明描述了 mkcramfs 生成的内容。内核的要求稍微宽松一些，例如它不关心 <file_data> 项是否被交换了位置（但它确实要求给定目录中的目录项（inode）是连续的，因为 readdir 会用到这一点）。目前所有数据都采用主机字节序格式；mkcramfs 和内核都不会进行字节交换。（详见下文的 “Block Size” 小节。）
- `crypto/` — SPDX-License-Identifier: GPL-2.0-only
- `debugfs/` — SPDX-License-Identifier: GPL-2.0-only
- `devpts/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux /dev/pts 虚拟文件系统 的 Makefile.
- `dlm/` — SPDX-License-Identifier: GPL-2.0
- `ecryptfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux eCryptfs 的 Makefile
- `efivarfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于efivarfs 文件系统 的 Makefile
- `efs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux efs 文件系统例程 的 Makefile.
- `erofs/` — SPDX-License-Identifier: GPL-2.0-only
- `exfat/` — SPDX-License-Identifier: GPL-2.0-or-later# # 用于linux exFAT filesystem support 的 Makefile.
- `exportfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于文件系统导出支持例程 的 Makefile.
- `ext2/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux ext2 文件系统例程 的 Makefile.
- `ext4/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux ext4 文件系统例程 的 Makefile.
- `f2fs/` — SPDX-License-Identifier: GPL-2.0
- `fat/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux fat 文件系统支持 的 Makefile.
- `freevxfs/` — SPDX-License-Identifier: GPL-2.0-only# # VxFS Makefile
- `fuse/` — SPDX-License-Identifier: GPL-2.0-only# # 用于FUSE filesystem 的 Makefile.
- `gfs2/` — SPDX-License-Identifier: GPL-2.0
- `hfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux hfs 文件系统例程 的 Makefile.
- `hfsplus/` — SPDX-License-Identifier: GPL-2.0# ## 用于linux hfsplus filesystem routines 的 Makefile.
- `hostfs/` — # Copyright (C) 2000 Jeff Dike (jdike@karaya.com) # Licensed under the GPL
- `hpfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux hpfs 文件系统例程 的 Makefile.
- `hugetlbfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux ramfs 例程 的 Makefile.
- `iomap/` — SPDX-License-Identifier: GPL-2.0-or-later # # Copyright (c) 2019 Oracle.
- `isofs/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux isofs 文件系统例程 的 Makefile.
- `jbd2/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux 日志例程 的 Makefile.
- `jffs2/` — JFFS2 加锁文档 	---------------------------  本文档试图描述 JFFS2 现有的加锁规则。它并不保证始终完全最新，但应当相当接近。   	alloc_sem
- `jfs/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux JFS 文件系统例程 的 Makefile.
- `kernfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于kernfs 伪文件系统 的 Makefile
- `lockd/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux 锁管理器相关内容 的 Makefile
- `minix/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux minix 文件系统例程 的 Makefile.
- `netfs/` — SPDX-License-Identifier: GPL-2.0
- `nfs/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux nfs 文件系统例程 的 Makefile.
- `nfs_common/` — SPDX-License-Identifier: GPL-2.0-only# # 用于客户端与服务器共享的 Linux 文件系统例程 的 Makefile.
- `nfsd/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux nfs 服务器 的 Makefile
- `nilfs2/` — SPDX-License-Identifier: GPL-2.0
- `nls/` — SPDX-License-Identifier: GPL-2.0# # 用于本地语言支持 的 Makefile
- `notify/` — SPDX-License-Identifier: GPL-2.0
- `ntfs/` — SPDX-License-Identifier: GPL-2.0
- `ntfs3/` — SPDX-License-Identifier: GPL-2.0# # 用于ntfs3 文件系统支持 的 Makefile.
- `ocfs2/` — SPDX-License-Identifier: GPL-2.0
- `omfs/` — SPDX-License-Identifier: GPL-2.0-only
- `openpromfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux Sun Openprom 文件系统例程 的 Makefile.
- `orangefs/` — SPDX-License-Identifier: GPL-2.0# # 用于ORANGEFS 文件系统 的 Makefile.
- `overlayfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于overlay 文件系统 的 Makefile.
- `proc/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux proc 文件系统例程 的 Makefile.
- `pstore/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux pstorefs 例程 的 Makefile.
- `qnx4/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux qnx4 文件系统例程 的 Makefile.
- `qnx6/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux qnx4 文件系统例程 的 Makefile.
- `quota/` — SPDX-License-Identifier: GPL-2.0
- `ramfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux ramfs 例程 的 Makefile.
- `resctrl/` — SPDX-License-Identifier: GPL-2.0# To allow define_trace.h's recursive include:
- `romfs/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux RomFS 文件系统例程 的 Makefile.
- `smb/` — SPDX-License-Identifier: GPL-2.0
- `squashfs/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux squashfs 例程 的 Makefile.
- `sysfs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于sysfs 虚拟文件系统 的 Makefile
- `tests/`
- `tracefs/` — SPDX-License-Identifier: GPL-2.0-only
- `ubifs/` — SPDX-License-Identifier: GPL-2.0
- `udf/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux udf 文件系统例程 的 Makefile.
- `ufs/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux ufs 文件系统例程 的 Makefile.
- `unicode/` — 本目录中的 utf8data.c 文件由 Unicode 字符数据库生成，对应 Unicode 标准的 12.1.0 版本。完整的文件集可在此处找到：    http://www.unicode.org/Public/12.1.0/ucd/  各个源文件链接：    https://www.unicode.org/Public/12.1.0/ucd/CaseFolding.txt
- `vboxsf/` — SPDX-License-Identifier: MIT
- `verity/` — SPDX-License-Identifier: GPL-2.0
- `xfs/` — SPDX-License-Identifier: GPL-2.0 # # Copyright (c) 2000-2005 Silicon Graphics, Inc.
- `zonefs/` — SPDX-License-Identifier: GPL-2.0

## include/

内核公共头文件（linux/、asm-generic/、uapi/）。

- `acpi/`
- `asm-generic/`
- `clocksource/`
- `crypto/` — 加密 API 与算法实现。
- `cxl/`
- `drm/` — SPDX-License-Identifier: GPL-2.0# Ensure drm headers are self-contained and pass kernel-doc # Include the header twice to detect missing include guard.
- `dt-bindings/`
- `hyperv/`
- `keys/`
- `kunit/`
- `kvm/`
- `linux/`
- `math-emu/`
- `media/`
- `memory/`
- `misc/`
- `net/` — 网络协议栈（ipv4、ipv6、netfilter、BPF、核心、以太网、无线等）。
- `pcmcia/`
- `ras/`
- `rdma/`
- `rv/`
- `scsi/`
- `soc/`
- `sound/` — ALSA 声音子系统与音频驱动。
- `target/`
- `trace/`
- `uapi/`
- `ufs/`
- `vdso/`
- `video/`
- `xen/`

## io_uring/

io_uring 异步 I/O 子系统。


## ipc/

进程间通信（msg、sem、shm）。


## kernel/

核心内核子系统（调度器、printk、irq、时间、加锁、RCU、BPF 等）。

- `bpf/` — SPDX-License-Identifier: GPL-2.0# ___bpf_prog_run() needs GCSE disabled on x86; see 3193c0836f203 for details
- `cgroup/` — SPDX-License-Identifier: GPL-2.0
- `configs/`
- `debug/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux 内核调试器 的 Makefile
- `dma/` — SPDX-License-Identifier: GPL-2.0
- `entry/` — SPDX-License-Identifier: GPL-2.0# Prevent the noinstr section from being pestered by sanitizer and other goodies # as long as these things cannot be disabled per function.
- `events/` — SPDX-License-Identifier: GPL-2.0
- `futex/` — SPDX-License-Identifier: GPL-2.0
- `gcov/` — SPDX-License-Identifier: GPL-2.0
- `irq/` — SPDX-License-Identifier: GPL-2.0
- `kcsan/` — SPDX-License-Identifier: GPL-2.0
- `livepatch/` — SPDX-License-Identifier: GPL-2.0-only
- `liveupdate/` — SPDX-License-Identifier: GPL-2.0
- `locking/` — SPDX-License-Identifier: GPL-2.0# Any varying coverage in these files is non-deterministic # and is generally not a function of system call inputs.
- `module/` — SPDX-License-Identifier: GPL-2.0-only# # 用于linux kernel module support 的 Makefile
- `power/` — SPDX-License-Identifier: GPL-2.0
- `printk/` — SPDX-License-Identifier: GPL-2.0-only
- `rcu/` — SPDX-License-Identifier: GPL-2.0# Any varying coverage in these files is non-deterministic # and is generally not a function of system call inputs.
- `sched/` — SPDX-License-Identifier: GPL-2.0# The compilers are complaining about unused variables inside an if(0) scope # block. This is daft, shut them up.
- `time/` — SPDX-License-Identifier: GPL-2.0# Branch profiling isn't noinstr-safe
- `trace/` — SPDX-License-Identifier: GPL-2.0# Do not instrument the tracer itself: # Avoid recursion due to instrumentation.
- `unwind/`

## lib/

内核通用库（位图、rbtree、radix-tree、crc、kunit 等）。

- `842/` — SPDX-License-Identifier: GPL-2.0-only
- `crc/` — SPDX-License-Identifier: GPL-2.0-only# 用于内核循环冗余校验（CRC）库代码 的 Makefile
- `crypto/` — SPDX-License-Identifier: GPL-2.0
- `dim/` — # DIM 动态中断调节库 #
- `fonts/` — SPDX-License-Identifier: GPL-2.0# 字体处理 # 内建字体；按 Family-Size 升序排序
- `kunit/` — KUnit 的 “hooks” 即便在 KUnit 作为模块构建时也是内建的。
- `lz4/` — SPDX-License-Identifier: GPL-2.0-only
- `lzo/` — SPDX-License-Identifier: GPL-2.0-only
- `math/` — SPDX-License-Identifier: GPL-2.0-only
- `pldmfw/` — SPDX-License-Identifier: GPL-2.0-only
- `raid/` — SPDX-License-Identifier: GPL-2.0
- `raid6/` — SPDX-License-Identifier: GPL-2.0# Enable <altivec.h>
- `reed_solomon/` — SPDX-License-Identifier: GPL-2.0-only# # This is a modified version of reed solomon lib,
- `test_fortify/` — SPDX-License-Identifier: GPL-2.0
- `tests/` — SPDX-License-Identifier: GPL-2.0# # 用于tests of kernel library functions 的 Makefile.
- `vdso/` — SPDX-License-Identifier: GPL-2.0-only
- `xz/` — SPDX-License-Identifier: GPL-2.0-only
- `zlib_deflate/` — SPDX-License-Identifier: GPL-2.0-only# # This is a modified version of zlib, which does all memory
- `zlib_dfltcc/` — SPDX-License-Identifier: GPL-2.0-only# # This is a modified version of zlib, which does all memory
- `zlib_inflate/` — SPDX-License-Identifier: GPL-2.0-only# # This is a modified version of zlib, which does all memory
- `zstd/` — SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause # ################################################################ # Copyright (c) Meta Platforms, Inc. and affiliates.

## mm/

内存管理（页分配器、slab、vmalloc、hugetlb、swap、mmap 等）。

- `damon/` — SPDX-License-Identifier: GPL-2.0
- `kasan/` — SPDX-License-Identifier: GPL-2.0# Disable ftrace to avoid recursion. # Function splitter causes unnecessary splits in __asan_load1/__asan_store1
- `kfence/` — SPDX-License-Identifier: GPL-2.0
- `kmsan/` — SPDX-License-Identifier: GPL-2.0# # 用于KernelMemorySanitizer (KMSAN) 的 Makefile.
- `tests/`

## net/

网络协议栈（ipv4、ipv6、netfilter、BPF、核心、以太网、无线等）。

- `6lowpan/` — SPDX-License-Identifier: GPL-2.0#rfc6282 nhcs #rfc7400 ghcs
- `802/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux 802 的 Makefile.x protocol layers.
- `8021q/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux VLAN 层 的 Makefile.
- `9p/` — SPDX-License-Identifier: GPL-2.0
- `appletalk/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux AppleTalk 层 的 Makefile.
- `atm/` — SPDX-License-Identifier: GPL-2.0# # 用于ATM 协议族 的 Makefile.
- `batman-adv/` — SPDX-License-Identifier: GPL-2.0 # Copyright (C) B.A.T.M.A.N. contributors: #
- `bluetooth/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux Bluetooth subsystem 的 Makefile.
- `bpf/` — SPDX-License-Identifier: GPL-2.0-only
- `bridge/` — SPDX-License-Identifier: GPL-2.0# # 用于IEEE 802 的 Makefile.1d ethernet bridging layer.
- `can/` — SPDX-License-Identifier: GPL-2.0# #  用于Linux Controller Area Network core 的 Makefile.
- `ceph/` — SPDX-License-Identifier: GPL-2.0# # 用于CEPH filesystem 的 Makefile.
- `core/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux networking core 的 Makefile.
- `dcb/` — SPDX-License-Identifier: GPL-2.0-only
- `devlink/` — SPDX-License-Identifier: GPL-2.0
- `dns_resolver/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux DNS 解析器 的 Makefile.
- `dsa/` — SPDX-License-Identifier: GPL-2.0# 只要 DSA 内建或作为模块构建，这些桩就是内建的 # 核心
- `ethernet/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux 以太网层 的 Makefile.
- `ethtool/` — SPDX-License-Identifier: GPL-2.0-only
- `handshake/` — SPDX-License-Identifier: GPL-2.0-only# # 用于通用 HANDSHAKE 服务 的 Makefile
- `hsr/` — SPDX-License-Identifier: GPL-2.0-only# # 用于HSR 的 Makefile
- `ieee802154/` — SPDX-License-Identifier: GPL-2.0
- `ife/` — SPDX-License-Identifier: GPL-2.0-only# # 用于IFE 封装协议 的 Makefile
- `ipv4/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux TCP/IP (INET) layer 的 Makefile.
- `ipv6/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux TCP/IP (INET6) layer 的 Makefile.
- `iucv/` — SPDX-License-Identifier: GPL-2.0-only# # 用于IUCV 的 Makefile
- `kcm/` — SPDX-License-Identifier: GPL-2.0-only
- `key/` — SPDX-License-Identifier: GPL-2.0-only# # 用于key AF 的 Makefile.
- `l2tp/` — SPDX-License-Identifier: GPL-2.0# # 用于L2TP 的 Makefile.
- `l3mdev/` — SPDX-License-Identifier: GPL-2.0-only# # 用于L3 设备 API 的 Makefile
- `lapb/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Linux LAPB 层 的 Makefile.
- `llc/` — # 用于 Linux 802.2 LLC（功能完整）层的 Makefile。#
- `mac80211/` — SPDX-License-Identifier: GPL-2.0# mac80211 对象
- `mac802154/` — SPDX-License-Identifier: GPL-2.0-only
- `mctp/` — SPDX-License-Identifier: GPL-2.0# 测试
- `mpls/` — SPDX-License-Identifier: GPL-2.0-only# # 用于MPLS 的 Makefile.
- `mptcp/` — SPDX-License-Identifier: GPL-2.0
- `ncsi/` — SPDX-License-Identifier: GPL-2.0-only# # 用于NCSI API 的 Makefile
- `netfilter/` — SPDX-License-Identifier: GPL-2.0
- `netlabel/` — SPDX-License-Identifier: GPL-2.0# # 用于NetLabel 子系统 的 Makefile.
- `netlink/` — SPDX-License-Identifier: GPL-2.0-only# # 用于netlink 驱动 的 Makefile.
- `nfc/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux NFC subsystem 的 Makefile.
- `nsh/` — SPDX-License-Identifier: GPL-2.0-only
- `openvswitch/` — SPDX-License-Identifier: GPL-2.0# # 用于Open vSwitch 的 Makefile.
- `packet/` — SPDX-License-Identifier: GPL-2.0-only# # 用于packet AF 的 Makefile.
- `phonet/` — SPDX-License-Identifier: GPL-2.0
- `psample/` — SPDX-License-Identifier: GPL-2.0-only# # 用于psample netlink 通道 的 Makefile
- `psp/` — SPDX-License-Identifier: GPL-2.0-only
- `qrtr/` — SPDX-License-Identifier: GPL-2.0-only
- `rds/` — SPDX-License-Identifier: GPL-2.0# 用于 GCOV 覆盖率分析
- `rfkill/` — SPDX-License-Identifier: GPL-2.0-only# # 用于RF 开关子系统 的 Makefile.
- `rxrpc/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux kernel RxRPC 的 Makefile
- `sched/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux 流量控制单元 的 Makefile.
- `sctp/` — SPDX-License-Identifier: GPL-2.0# # 用于SCTP support code 的 Makefile.
- `shaper/` — SPDX-License-Identifier: GPL-2.0-only# # 用于网络整形基础设施 的 Makefile.
- `smc/` — SPDX-License-Identifier: GPL-2.0-only
- `strparser/` — SPDX-License-Identifier: GPL-2.0-only
- `sunrpc/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux kernel SUN RPC 的 Makefile
- `switchdev/` — SPDX-License-Identifier: GPL-2.0-only# # 用于Switch 设备 API 的 Makefile
- `tipc/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux TIPC 层 的 Makefile
- `tls/` — SPDX-License-Identifier: GPL-2.0-only# # 用于TLS 子系统 的 Makefile.
- `unix/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux unix 域套接字层 的 Makefile.
- `vmw_vsock/` — SPDX-License-Identifier: GPL-2.0
- `wireless/` — SPDX-License-Identifier: GPL-2.0
- `x25/` — SPDX-License-Identifier: GPL-2.0# # 用于Linux X 的 Makefile.25 Packet layer.
- `xdp/` — SPDX-License-Identifier: GPL-2.0-only
- `xfrm/` — SPDX-License-Identifier: GPL-2.0# # 用于XFRM subsystem 的 Makefile.

## rust/

Rust 内核支持（bindings、核心、helpers、vendored crates）。

- `bindings/`
- `helpers/`
- `kernel/` — 核心内核子系统（调度器、printk、irq、时间、加锁、RCU、BPF 等）。
- `macros/`
- `pin-init/` — [![Crates.io](https://img.shields.io/crates/v/pin-init.svg)](https://crates.io/crates/pin-init) [![Documentation](https://docs.rs/pin-init/badge.svg)](https://docs.rs/pin-init/) [![Dependency status](https://deps.rs/repo/github/Rust-for-Linux/pin-init/status.svg)](https://deps.rs/repo/github/Rust-for-Linux/pin-init) ![License](https://img.shields.io/crates/l/pin-init) [![Toolchain](https://img.shields.io/badge/toolchain-nightly-red)](#nightly-only) ![GitHub Workflow Status](https://img.shield...
- `proc-macro2/` — # `proc-macro2`  这些源文件来自 Rust `proc-macro2` crate，版本 1.0.101（发布于 2025-08-16），托管于 <https://github.com/dtolnay/proc-macro2> 仓库，采用 “Apache-2.0 OR MIT” 许可，仅做了修改以添加 SPDX 许可证标识符并移除 `unicode-ident` 依赖。  版权详情请参阅：
- `quote/` — # `quote`  这些源文件来自 Rust `quote` crate，版本 1.0.40（发布于 2025-03-12），托管于 <https://github.com/dtolnay/quote> 仓库，采用 “Apache-2.0 OR MIT” 许可，仅做了修改以添加 SPDX 许可证标识符。  版权详情请参阅：      https://github.com/dtolnay/quote/blob/1.0.40/README.md#license
- `syn/` — # `syn`  这些源文件来自 Rust `syn` crate，版本 2.0.106（发布于 2025-08-16），托管于 <https://github.com/dtolnay/syn> 仓库，采用 “Apache-2.0 OR MIT” 许可，仅做了修改以添加 SPDX 许可证标识符并移除 `unicode-ident` 依赖。  版权详情请参阅：
- `uapi/`

## samples/

示例与教程代码（BPF、vfio-mdev、pktgen）。

- `acrn/` — SPDX-License-Identifier: GPL-2.0
- `auxdisplay/` — SPDX-License-Identifier: GPL-2.0
- `binderfs/` — SPDX-License-Identifier: GPL-2.0-only
- `bpf/` — eBPF 示例程序 ====================  本目录包含使用 eBPF 的测试桩、验证器测试套件和示例。示例使用了来自 tools/lib/bpf 的 libbpf。请注意，特定于 XDP 的示例已从本目录移除，并移至 xdp-tools 仓库： https://github.com/xdp-project/xdp-tools 有关如何将旧示例中的特定命令调用转换为新工具的说明，请参阅从本目录移除每个工具的提交信息
- `cgroup/` — SPDX-License-Identifier: GPL-2.0
- `check-exec/` — SPDX-License-Identifier: BSD-3-Clause
- `configfs/` — SPDX-License-Identifier: GPL-2.0-only
- `connector/` — SPDX-License-Identifier: GPL-2.0
- `coresight/` — SPDX-License-Identifier: GPL-2.0-only
- `damon/` — SPDX-License-Identifier: GPL-2.0
- `fanotify/` — SPDX-License-Identifier: GPL-2.0-only
- `fprobe/` — SPDX-License-Identifier: GPL-2.0-only
- `ftrace/` — SPDX-License-Identifier: GPL-2.0-only
- `hid/` — SPDX-License-Identifier: GPL-2.0# 要构建的程序列表 # Libbpf 依赖
- `hidraw/` — SPDX-License-Identifier: GPL-2.0
- `hung_task/` — SPDX-License-Identifier: GPL-2.0-only
- `hw_breakpoint/` — SPDX-License-Identifier: GPL-2.0-only
- `kdb/` — SPDX-License-Identifier: GPL-2.0-only
- `kfifo/` — SPDX-License-Identifier: GPL-2.0-only
- `kmemleak/` — SPDX-License-Identifier: GPL-2.0-only
- `kobject/` — SPDX-License-Identifier: GPL-2.0-only
- `kprobes/` — SPDX-License-Identifier: GPL-2.0-only# builds the kprobes example kernel modules; # then to use one (as root):  insmod <module_name.ko>
- `landlock/` — SPDX-License-Identifier: BSD-3-Clause
- `livepatch/` — SPDX-License-Identifier: GPL-2.0-only
- `mei/` — SPDX-License-Identifier: GPL-2.0 # Copyright (c) 2012-2019, Intel Corporation. All rights reserved.
- `nitro_enclaves/` — SPDX-License-Identifier: GPL-2.0 # # Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
- `pfsm/` — SPDX-License-Identifier: GPL-2.0
- `pidfd/` — SPDX-License-Identifier: GPL-2.0
- `pktgen/` — pktgen（数据包生成器）的示例与基准脚本 ========================================================== 本目录包含一些 pktgen 示例和基准脚本，可轻松复制并针对你的用例进行调整。  通用文档位于内核中：Documentation/networking/pktgen.rst  辅助包含文件 ==================== 本目录包含两个可“包含”的辅助 shell 文件
- `qmi/` — SPDX-License-Identifier: GPL-2.0-only
- `rpmsg/` — SPDX-License-Identifier: GPL-2.0-only
- `rust/` — SPDX-License-Identifier: GPL-2.0
- `seccomp/` — SPDX-License-Identifier: GPL-2.0
- `timers/` — SPDX-License-Identifier: GPL-2.0
- `trace_events/` — SPDX-License-Identifier: GPL-2.0-only# builds the trace events example kernel modules; # then to use one (as root):  insmod <module_name.ko>
- `trace_printk/` — SPDX-License-Identifier: GPL-2.0-only# builds a module that calls various trace_printk routines # then to use one (as root):  insmod <module_name.ko>
- `tsm-mr/` — SPDX-License-Identifier: GPL-2.0-only
- `uhid/` — SPDX-License-Identifier: GPL-2.0-only
- `user_events/` — SPDX-License-Identifier: GPL-2.0
- `v4l/` — SPDX-License-Identifier: GPL-2.0-only
- `vfio-mdev/` — 使用 mtty vfio-mdev 示例代码 ====================================  mtty 是一个示例 vfio-mdev 驱动，演示了如何使用中介设备（mediated device）框架。该示例驱动创建一个 mdev 设备，模拟通过 PCI 卡提供的串口。  1. 构建并加载 mtty.ko 模块。
- `vfs/` — SPDX-License-Identifier: GPL-2.0-only
- `watch_queue/` — SPDX-License-Identifier: GPL-2.0-only
- `watchdog/` — SPDX-License-Identifier: GPL-2.0
- `workqueue/`

## scripts/

构建脚本、checkpatch、coccinelle 补丁、kconfig、modpost 等。

- `atomic/`
- `bash-completion/`
- `basic/` — SPDX-License-Identifier: GPL-2.0-only# # fixdep: used to generate dependency information during build process
- `clang-tools/`
- `coccinelle/`
- `crypto/` — 加密 API 与算法实现。
- `dtc/` — SPDX-License-Identifier: GPL-2.0# scripts/dtc makefile # *** Also keep .gitignore in sync when changing ***
- `dummy-tools/`
- `gcc-plugins/` — SPDX-License-Identifier: GPL-2.0# Build rules for plugins #
- `gdb/` — SPDX-License-Identifier: GPL-2.0-only
- `gendwarfksyms/` — SPDX-License-Identifier: GPL-2.0
- `genksyms/` — SPDX-License-Identifier: GPL-2.0# -I needed for generated C source to include headers in source tree # dependencies on generated files need to be listed explicitly
- `include/` — 内核公共头文件（linux/、asm-generic/、uapi/）。
- `ipe/` — SPDX-License-Identifier: GPL-2.0-only
- `kconfig/` — SPDX-License-Identifier: GPL-2.0# =========================================================================== # Kernel configuration targets
- `ksymoops/` — ksymoops 已从内核中移除。它一直是一个独立的工具，不链接到任何特定的内核版本。最新版本可在 https://www.kernel.org/pub/linux/utils/kernel/ksymoops 找到，同时还有对其他工具的补丁，以便提供更准确的 Oops 调试信息。  Keith Owens <kaos@ocs.com.au> Sat Jun 19 10:30:34 EST 1999
- `livepatch/` — SPDX-License-Identifier: GPL-2.0# 用于开发者工具的独立 Makefile（不属于 kbuild）。
- `mod/` — SPDX-License-Identifier: GPL-2.0# dependencies on generated files need to be listed explicitly
- `package/`
- `selinux/` — 有关安装虚拟 SELinux 策略的信息，请参阅 Documentation/admin-guide/LSM/SELinux.rst。
- `tracing/`

## security/

安全模块（selinux、apparmor、landlock、smack 等）。

- `apparmor/` — SPDX-License-Identifier: GPL-2.0# 用于AppArmor Linux Security Module 的 Makefile#
- `bpf/` — SPDX-License-Identifier: GPL-2.0 # # Copyright (C) 2020 Google LLC.
- `integrity/` — SPDX-License-Identifier: GPL-2.0# # 用于缓存 inode 完整性数据（iint） 的 Makefile
- `ipe/` — SPDX-License-Identifier: GPL-2.0 # # Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
- `keys/` — SPDX-License-Identifier: GPL-2.0# # 用于密钥管理 的 Makefile
- `landlock/`
- `loadpin/` — SPDX-License-Identifier: GPL-2.0-only
- `lockdown/`
- `safesetid/` — SPDX-License-Identifier: GPL-2.0# # 用于safesetid LSM 的 Makefile.
- `selinux/` — SPDX-License-Identifier: GPL-2.0# # 用于将 SELinux 模块作为内核树的一部分构建 的 Makefile.
- `smack/` — SPDX-License-Identifier: GPL-2.0-only# # 用于SMACK LSM 的 Makefile
- `tomoyo/` — SPDX-License-Identifier: GPL-2.0
- `yama/` — SPDX-License-Identifier: GPL-2.0-only

## sound/

ALSA 声音子系统与音频驱动。

- `ac97/` — SPDX-License-Identifier: GPL-2.0-only# # make for AC97 bus drivers
- `aoa/` — SPDX-License-Identifier: GPL-2.0-only
- `arm/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `atmel/` — SPDX-License-Identifier: GPL-2.0-only
- `core/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `drivers/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `firewire/` — SPDX-License-Identifier: GPL-2.0# To find a header included by define_trace.h.
- `hda/` — SPDX-License-Identifier: GPL-2.0# this must be the last entry after codec drivers; # otherwise the codec drivers won't be hooked before the PCI probe
- `i2c/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `isa/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `mips/` — SPDX-License-Identifier: GPL-2.0-only# # 用于ALSA 的 Makefile
- `oss/`
- `parisc/` — SPDX-License-Identifier: GPL-2.0-only# # 用于ALSA 的 Makefile
- `pci/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `pcmcia/` — SPDX-License-Identifier: GPL-2.0-only# # 用于ALSA 的 Makefile
- `ppc/` — SPDX-License-Identifier: GPL-2.0-only# # 用于ALSA 的 Makefile
- `sh/` — SPDX-License-Identifier: GPL-2.0-only# # 用于ALSA 的 Makefile
- `soc/` — SPDX-License-Identifier: GPL-2.0# snd-soc-test-y := soc-topology-test.o # snd-soc-test-y := soc-utils-test.o
- `sparc/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `spi/` — SPDX-License-Identifier: GPL-2.0# 用于SPI drivers 的 Makefile
- `synth/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `usb/` — SPDX-License-Identifier: GPL-2.0# # 用于ALSA 的 Makefile
- `virtio/` — SPDX-License-Identifier: GPL-2.0+
- `x86/` — SPDX-License-Identifier: GPL-2.0-only
- `xen/` — SPDX-License-Identifier: GPL-2.0 OR MIT

## tools/

用户空间工具（perf、bpftool、selftests、kunit、cpupower 等）。

- `accounting/` — SPDX-License-Identifier: GPL-2.0
- `arch/` — 特定于体系结构的代码（arm64、x86、riscv、m68k、powerpc 等）以及引导基础设施。
- `bootconfig/` — SPDX-License-Identifier: GPL-2.0# 用于bootconfig command 的 Makefile
- `bpf/` — SPDX-License-Identifier: GPL-2.0# This will work when bpf is built in tools env. where srctree # isn't set and when invoked from selftests build, where srctree
- `build/` — SPDX-License-Identifier: GPL-2.0
- `certs/`
- `cgroup/`
- `counter/` — SPDX-License-Identifier: GPL-2.0# Do not use make's built-in rules # (this improves performance and avoids hard-to-debug behaviour);
- `crypto/` — 加密 API 与算法实现。
- `debugging/` — SPDX-License-Identifier: GPL-2.0# 用于debugging tools 的 Makefile
- `dma/` — SPDX-License-Identifier: GPL-2.0# This will work when dma is built in tools env. where srctree # isn't set and when invoked from selftests build, where srctree
- `docs/`
- `firewire/` — SPDX-License-Identifier: GPL-2.0
- `firmware/` — SPDX-License-Identifier: GPL-2.0# 用于firmware tools 的 Makefile
- `gpio/` — SPDX-License-Identifier: GPL-2.0# This will work when gpio is built in tools env. where srctree # isn't set and when invoked from selftests build, where srctree
- `hv/` — SPDX-License-Identifier: GPL-2.0# 用于Hyper-V tools 的 Makefile# Do not use make's built-in rules
- `iio/` — SPDX-License-Identifier: GPL-2.0# Do not use make's built-in rules # (this improves performance and avoids hard-to-debug behaviour);
- `include/` — 内核公共头文件（linux/、asm-generic/、uapi/）。
- `kvm/`
- `laptop/`
- `leds/` — SPDX-License-Identifier: GPL-2.0# 用于LEDs tools 的 Makefile
- `lib/` — 内核通用库（位图、rbtree、radix-tree、crc、kunit 等）。
- `memory-model/` — ===================================== 		LINUX 内核内存一致性模型 		=====================================  ============ 简介 ============  本目录包含 Linux 内核的内存一致性模型（简称内存模型），使用 “cat” 语言编写并可执行
- `mm/` — SPDX-License-Identifier: GPL-2.0# 用于vm tools 的 Makefile#
- `net/` — 网络协议栈（ipv4、ipv6、netfilter、BPF、核心、以太网、无线等）。
- `objtool/` — SPDX-License-Identifier: GPL-2.0
- `pcmcia/` — SPDX-License-Identifier: GPL-2.0
- `perf/` — SPDX-License-Identifier: GPL-2.0# # This is a simple wrapper Makefile that calls the main Makefile.perf
- `power/`
- `rcu/`
- `sched/`
- `sched_ext/` — SCHED_EXT 示例调度器 ============================  # 简介  本目录包含若干 sched_ext 示例调度器。这些调度器旨在提供使用 sched_ext 可构建的不同类型调度器的示例，并演示 sched_ext 的各项特性如何被使用。
- `scripts/` — 构建脚本、checkpatch、coccinelle 补丁、kconfig、modpost 等。
- `sound/` — ALSA 声音子系统与音频驱动。
- `spi/` — SPDX-License-Identifier: GPL-2.0-only# Do not use make's built-in rules # (this improves performance and avoids hard-to-debug behaviour);
- `testing/`
- `thermal/`
- `time/`
- `tracing/` — SPDX-License-Identifier: GPL-2.0
- `unittests/`
- `usb/` — SPDX-License-Identifier: GPL-2.0# 用于USB tools 的 Makefile# Do not use make's built-in rules
- `verification/`
- `virtio/` — SPDX-License-Identifier: GPL-2.0
- `wmi/` — SPDX-License-Identifier: GPL-2.0-only
- `workqueue/`
- `writeback/`

## virt/

虚拟化（KVM、UML、Xen 等）。

- `kvm/`
- `lib/` — SPDX-License-Identifier: GPL-2.0-only

---

# Kconfig 摘要

## 其他

| 配置 | 类型 | 描述 |
|--------|------|-------------|
| 842_COMPRESS | tristate | 启用内核中 s390x 对 zlib 的硬件支持。  |
| ADVISE_SYSCALLS | bool | 该选项启用 madvise 与 fadvise 系统调用，应用程序借此向内核建议其未来的内存或文件使用方式，从而提升性能。若构建……  |
| AIO | bool | 该选项启用 POSIX 异步 I/O，部分高性能多线程应用可能会用到。禁用此选项可节省约 7k。  |
| ANON_VMA_NAME | bool | 允许为匿名虚拟内存区域命名。该功能可为虚拟内存区域指定名称，所指定名称随后可从 /proc/pid/maps 与 /proc/pid/smaps 中读取，有助于识……  |
| ARCH_FORCE_MAX_ORDER | int | 页块阶（page block order）指物理连续、可关联迁移类型的页面数量的 2 的幂。页块阶的最大尺寸至少为……  |
| ARCH_HAS_BINFMT_FLAT | bool | 支持 uClinux FLAT 格式二进制文件。  |
| ARCH_HAS_CC_CAN_LINK | bool | 选择此项可将 thread_info 从栈上移入 task_struct。为使此功能生效，体系结构需移除除 flags 外的所有 thread_info 字段并修复相关运行时缺陷。其中一个细微改动……  |
| ARCH_HAS_CPU_CACHE_ALIASING | bool | 为支持 HARDENED_USERCOPY 进行栈变量生命周期检查，需要一种与体系结构无关的方式来获取栈指针。一旦某体系结构定义了 unsigned long 全局变量 r……  |
| ARCH_HAS_DEBUG_VIRTUAL | bool | 在虚拟地址到页的转换代码中启用一些代价较高的健全性检查。可捕获 virt_to_page() 等函数的误用。若不确定，选 N。  |
| ARCH_HAS_DEBUG_VM_PGTABLE | bool | 当某体系结构能成功构建并运行 DEBUG_VM_PGTABLE 时，应选择此项。  |
| ARCH_HAS_DEVMEM_IS_ALLOWED | bool | 若禁用此选项，则允许用户空间（root）访问全部内存，包括内核与用户空间内存。意外访问显然后果严重，但特定访问可能……  |
| ARCH_HAS_ELF_CORE_EFLAGS | bool | 若体系结构利用 ELF 头中的 e_flags 字段来存放应在核心转储中保留的 ABI 或其他体系结构相关信息，请选择此项。  |
| ARCH_HAS_KCOV | bool | 当某体系结构能成功在 CONFIG_KCOV 下构建并运行时，应选择此项。这通常需对某些早期引导代码禁用插桩。  |
| ARCH_HAS_MEMBARRIER_CALLBACKS | bool | 基于体系结构控制 MSEAL_SYSTEM_MAPPINGS 的访问。内存密封特性需要 64 位内核。无需 CPU 提供特定硬件特性。要启用此特性……  |
| ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE | bool |  |
| ARCH_HAS_PTE_SPECIAL | bool | 启用 memfd_secret() 系统调用，可创建仅在所属进程上下文中可见、且不映射到其他进程及其他内核页表的内存区域。  |
| ARCH_HAS_STRNCPY_FROM_USER | bool | 在某些不存在独立 I/O 空间的平台上，部分 I/O 主机无法以 MMIO 模式访问。借助逻辑 PIO 机制，主机本地 I/O 资源可被映射到系统……  |
| ARCH_HAS_USER_SHADOW_STACK | bool | 该体系结构提供对用户空间影子调用栈（shadow call stack）的硬件支持（例如 x86 CET、arm64 GCS 或 RISC-V Zicfiss）。  |
| ARCH_HAS_ZONE_DMA_SET | bool | 设备内存热插拔支持允许在 memmap 中建立 pmem 或其他由设备驱动发现的内存区域。这使得可对原本“设备物理”地址进行 pfn_to_page() 查找……  |
| ARCH_NO_SG_CHAIN | def_bool | 栈仓库（stack depot）：避免重复的栈跟踪存储  |
| ARCH_NO_SWAP | bool | 此选项让你选择内核是否支持所谓的交换设备（swap device）或交换文件（swap file），用于提供比实际物理 RAM 更多的虚拟内存……  |
| ARCH_SUPPORTS_HUGETLBFS | def_bool | hugetlbfs 是基于 ramfs 的 HugeTLB 页文件系统后端。支持的体系结构请在此选 Y，并阅读 <file:Documentation/admin-guide/mm/hugetlbpage.rst> 了解细节。若不确定……  |
| ARCH_SUPPORTS_KMAP_LOCAL_FORCE_MAP | bool | 此选项在非高端内存页及非高端内存系统上，强制通过 kmap_local 机制建立临时映射。生产系统请禁用！  |
| ARCH_SUPPORTS_MEMORY_FAILURE | bool | 在具备 MCA 恢复能力的系统上启用从部分内存故障中恢复的代码。即使部分内存存在未纠正错误，系统仍可继续运行。这需要特殊的硬……  |
| ARCH_SUPPORTS_NUMA_BALANCING | bool | 此选项添加对自动感知 NUMA 的内存/任务放置的支持。该机制较为原始，基于在内存引用到任务所运行的节点时进行迁移……  |
| ARCH_USE_MEMTEST | bool | 当某体系结构在引导过程中使用 early_memtest() 时，应选择此项。  |
| ARCH_WANT_FRAME_POINTERS | bool | 若选 Y，生成的内核镜像会稍大且稍慢，但在内核出错时可提供非常有用的调试信息（精确的 oops、栈跟踪、警告）。  |
| ARCH_WANT_GENERAL_HUGETLB | bool | 启用此选项可降低大零 folio（huge zero folio）的运行时引用计数开销，并扩展内核中可使用大零 folio 的位置。例如块 I/O 可从中受益……  |
| ASSOCIATIVE_ARRAY | bool | 通用关联数组。可在被修改的同时进行查找与遍历。其查找与修改也相当迅速。算法为非递归式，树结构较……  |
| ASYNC_RAID6_TEST | tristate | 这是一次性自检测试，会遍历 N 盘阵列所有可能的双盘故障场景进行恢复。恢复使用异步 raid6 恢复例程……  |
| AS_HAS_NON_CONST_ULEB128 | def_bool | 选择“None”以外的值会导致内核镜像包含调试信息，从而增大镜像体积。它会向内核与模块添加调试符号（gcc -g），并……  |
| ATOMIC64_SELFTEST | tristate | 启用此选项可在引导时或模块加载时测试 atomic64_t 函数。若不确定，选 N。  |
| AUDIT | bool | 启用审计基础设施，可与其他内核子系统（如 SELinux，其记录 avc 消息输出需要它）配合使用。系统调用审计包含于体系结构……  |
| BACKTRACE_SELF_TEST | tristate | 该选项提供一个内核模块，用于测试内核栈回溯代码。此选项对发行版或通用内核无用，仅对内核开发者……  |
| BASE64_KUNIT | tristate | 构建 base64 单元测试。测试覆盖内核中 Base64 函数的编码与解码逻辑。除正确性检查外，还对两种编码都进行了简单的性能基准测试……  |
| BASE_SMALL | bool | 启用此选项可缩减各类核心内核数据结构的大小。这在小型机器上节省内存，但可能降低性能。  |
| BCH_CONST_M | int | Galois 域阶数“m”的常数值。若“k”为要保护的位数，则“m”应满足 (k + m*t) <= 2**m - 1。驱动应为该符号声明默认值……  |
| BCH_CONST_T | int | 纠错能力（以比特为单位的“t”）的常数值。若驱动选择了 BCH_CONST_PARAMS 选项，则应为该符号声明默认值。# # 如需……  |
| BINARY_PRINTF | def_bool | 在初始化时对全部可用的 RAID6 PQ 函数进行基准测试，并选择最快的一个。  |
| BINDGEN_VERSION_TEXT | string | 回溯到每个体系结构各自定义 cpu_online_mask 与 cpu_possible_mask 的时代，其中一些将其初始化为全 1，另一些为全 0。当它们被集中化……  |
| BINFMT_ELF | bool | ELF（Executable and Linkable Format，可执行与可链接格式）是一种跨不同体系结构与操作系统使用的库与可执行文件格式。在此选 Y 将让你的内核能够运行 ELF 二进制文件……  |
| BINFMT_ELF_KUNIT_TEST | bool | 构建 ELF 加载器 KUnit 测试，尝试将以往的错误修复收集为回归测试集。这通常仅用于调试。注意在 CONFIG_COMPAT=y 时，compat_b……  |
| BINFMT_FLAT_ARGVP_ENVP_ON_STACK | bool | 支持十年前的 uClinux FLAT 格式二进制文件。除非你确定拥有此类文件，否则在此选 N。  |
| BINFMT_MISC | tristate | 若在此选 Y，便可向内核插入由包装器驱动的二进制格式。当你使用需要解释器才能运行的程序（如 Java、Python……）时会特别有用。  |
| BINFMT_SCRIPT | tristate | 若希望执行以 #! 开头并后跟解释器路径的脚本，请在此选 Y。你可以将其构建为模块；但在该模块加载之前，你无……  |
| BINFMT_ZFLAT | bool | 支持 FLAT 格式压缩二进制文件  |
| BITFIELD_KUNIT | tristate | 启用此选项可在引导时测试位域函数。KUnit 测试在引导期间运行，并以 TAP 格式（http://testanything.org/）将结果输出到调试日志。仅供内核开发者……  |
| BITOPS_KUNIT | tristate | 该选项启用 bitops 库的 KUnit 测试，提供位操作函数。注意它源自原始的 test_bitops 模块。用于微基准测试与编译……  |
| BITREVERSE | tristate | 该选项在某些支持此类操作的体系结构上启用硬件位反转指令。  |
| BITS_TEST | tristate | 构建 bits 单元测试。测试 bits.h 中定义的宏的逻辑。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-tools 中的 KUnit 文档……  |
| BLACKHOLE_DEV_KUNIT_TEST | tristate | 构建“blackhole_dev_kunit”模块，用于验证通过该黑洞网络设备的的数据路径。若不确定，选 N。  |
| BLK_CGROUP | bool | 通用块 I/O 控制器 cgroup 接口。这是各类 I/O 控制策略应使用的通用 cgroup 接口。当前 CFQ I/O 调度器用它来识别任务组……  |
| BLK_DEV_INITRD | bool | 初始 RAM 文件系统是由引导加载程序（loadlin 或 lilo）加载的 ramfs，并在正常引导流程之前挂载为根文件系统。它通常用于加载所需模块……  |
| BOOTPARAM_HUNG_TASK_PANIC | int | 当设为非零值时，若在单次扫描中发现的挂起任务数量达到该值，将触发内核 panic。该 panic 可与 panic_timeout 配合使用，以……  |
| BOOTPARAM_SOFTLOCKUP_PANIC | int | 设为非零值 N，使内核在出现“软锁死（soft lockup）”时 panic；软锁死是指导致内核在内核模式下循环超过 (N * 20 秒)（可使用 watchdo……配置）的缺陷。  |
| BOOTPARAM_WQ_STALL_PANIC | int | 设置触发内核 panic 的工作队列停滞次数。当工作线程池在超过 30 秒（可使用……配置）内对某个待处理工作项没有进展时，即发生工作队列停滞。  |
| BOOT_CONFIG | bool | 额外的引导配置允许系统管理员在内核引导时，将一份配置文件作为内核命令行参数的补充扩展传入。该引导配置文件必须以校验和形式附加在 initramfs 末尾，因……  |
| BOOT_CONFIG_EMBED | bool | 将 BOOT_CONFIG_EMBED_FILE 指定的 bootconfig 文件嵌入内核。通常 bootconfig 文件随 initrd 镜像加载。但若系统不支持 initrd，此选项会有所帮助……  |
| BOOT_CONFIG_EMBED_FILE | string | 指定将要嵌入内核的 bootconfig 文件。当 initrd 中没有，或 initrd 中没有其他 bootconfig 时，将使用此 bootconfig。  |
| BOOT_CONFIG_FORCE | bool | 设置此 Kconfig 选项后，即使省略“bootconfig”内核引导参数，也会执行 BOOT_CONFIG 处理。事实上，设置此选项后，无法使内核……  |
| BOOT_PRINTK_DELAY | bool | 该编译选项通过在每条内核引导消息后插入短暂延迟，使你能更轻松地阅读这些消息。延迟值以毫秒为单位，通过在命令行上使用 "boot_delay=N" 指定。若不确定，选 N。 |
| BRIDGE_NETFILTER | tristate | 启用此选项后，arptables 与 iptables 将能看到经过桥接的 ARP 或 IP 流量。若你想要一个桥接防火墙，很可能应当启用此选项。 |
| BROKEN | bool | 此选项允许你选择是否尝试编译（并修复）尚未更新到新基础设施的旧驱动。 |
| BROKEN_ON_SMP | bool | 经由内核命令行传递给 init 的参数与环境变量数量各自的最大值。 |
| BSD_PROCESS_ACCT | bool | 若在此选 Y，一个用户态程序便能指示内核（通过特殊的系统调用）将进程记账信息写入文件：每当一个进程退出时，有关该进程的信息便会被记录。 |
| BSD_PROCESS_ACCT_V3 | bool | 若在此选 Y，进程记账信息将以一种新的文件格式写入，该格式同时记录每个进程及其父进程的进程 ID。注意此文件格式与旧格式不兼容。 |
| BUG | bool | 禁用此选项会移除对 BUG 与 WARN 的支持，缩小内核镜像体积，但也可能在后台静默忽略大量致命状况。你只应在明确需要时考虑禁用它。 |
| BUILD_SALT | string | 构建 ID 用于将二进制文件与其调试信息相关联。设置此选项将在构建 ID 的计算中使用该值。这对于希望确保…… |
| BUILTIN_MODULE_RANGES | bool | 当模块被编译进内核时，/proc/kallsyms 中的符号将不再关联模块名。追踪器可能希望无论……都按模块名与符号名识别符号。 |
| CACHESTAT_SYSCALL | bool | 启用 cachestat 系统调用，它可查询文件的页缓存统计信息（已缓存页数、脏页数、标记为回写的页数、（近期）被回收的页数）。若不确定，在此选 Y。 |
| CC_IS_GCC | def_bool | 它不依赖于 `RUST`，因为后者可能需要在 `depends on` 中使用该版本。 |
| CC_OPTIMIZE_FOR_PERFORMANCE | bool | 这是内核的默认优化级别，使用 "-O2" 编译器标志构建，以获得最佳性能与最有用的编译期警告。 |
| CC_OPTIMIZE_FOR_SIZE | bool | 选择此选项会向编译器传递 "-Os"，从而生成更小的内核。 |
| CC_VERSION_TEXT | string | 它的用途不太明确：- 当编译器更新时重新运行 Kconfig。'default' 属性引用环境变量 CC_VERSION_TEXT，因此它会被记录到 include/config/auto.conf…… |
| CGROUP_BPF | bool | 允许使用 bpf(2) 系统调用的 BPF_PROG_ATTACH 命令将 eBPF 程序附加到 cgroup。这些程序在何种上下文中被访问取决于附加的类型。 |
| CGROUP_CPUACCT | bool | 提供一个简单的控制器，用于监控 cgroup 中任务消耗的总 CPU 时间。 |
| CGROUP_DEBUG | bool | 此选项启用一个简单的控制器，导出关于 cgroups 框架的调试信息。该控制器仅用于控制 cgroup 调试，其接口不稳定。建议选 N。 |
| CGROUP_DMEM | bool | DMEM 控制器允许兼容的设备基于 cgroup 层级限制设备内存使用。例如，它允许你限制 DRM 子系统中应用的 VRAM 用量。 |
| CGROUP_FREEZER | bool | 提供一种冻结与解冻 cgroup 中所有任务的方式。此选项影响原始的 cgroup 接口。cgroup2 内存控制器默认包含重要的内核内内存消费者…… |
| CGROUP_HUGETLB | bool | 为 HugeTLB 页提供一个 cgroup 控制器。启用后，你可以对每个 cgroup 设置 HugeTLB 使用上限。该限制在缺页时强制执行。由于 HugeTLB 不支持页回收…… |
| CGROUP_MISC | bool | 为主机上的杂项资源提供控制器。杂项标量资源是主机系统中无法像其他 cgroup 那样抽象的资源。该控制器…… |
| CGROUP_NET_CLASSID | bool | 用作通用套接字 classid 标记的 cgroup 子系统，用于 cls_cgroup 与 netfilter 匹配。 |
| CGROUP_PERF | bool | 此选项扩展 perf 的每 CPU 模式，将监控限制到属于指定 cgroup 并在指定 CPU 上运行的线程。也可用于在采样中带上 cgroup ID…… |
| CGROUP_PIDS | bool | 在 cgroup 范围内强制进程数量上限。任何超出 cgroup 允许数量而尝试 fork 更多进程的操作都将失败。PID 本质上是一种全局资源，因为…… |
| CGROUP_RDMA | bool | 强制实施由 IB 协议栈定义的 RDMA 资源限制。消费者很容易耗尽 RDMA 资源，从而导致其他消费者无法获得资源。RDMA 控制…… |
| CGROUP_WRITEBACK | bool | 该特性让 CPU 调度器识别任务组，并对这些任务组控制 CPU 带宽分配。它使用 cgroups 对任务分组（若 CGROUP_SCHED）。 |
| CHECKPOINT_RESTORE | bool | 为检查点/恢复之目的启用额外的内核特性。特别是它添加了辅助的 prctl 代码以设置进程代码段、数据段与堆段的大小，以及少量额外的 /proc 文件…… |
| CHECKSUM_KUNIT | tristate | 启用此选项以在引导时测试校验和函数。KUnit 测试在引导期间运行，并以 TAP 格式（http://testanything.org/）将结果输出到调试日志。仅对内核开发者有用。 |
| CLOSURES | bool | 对 cpumask_var_t 使用动态分配，而非将其放在栈上。这样开销略大，但可避免栈溢出。 |
| CMA_AREAS | int | CMA 可针对特定用途创建 CMA 区域，主要用作设备私有区域。此参数设置系统中 CMA 区域的最大数量。若不确定，保留默认值 "8"。 |
| CMA_DEBUGFS | bool | 开启 CMA 的 DebugFS 接口。 |
| CMA_SYSFS | bool | 此选项暴露一些 sysfs 属性，以从 CMA 获取信息。 |
| CMDLINE_KUNIT_TEST | tristate | 构建 cmdline API 单元测试，测试 cmdline.c 提供的 API 逻辑。有关 KUnit 与单元测试的更多信息，请参阅 Documentation 中的 KUnit 文档…… |
| CMDLINE_LOG_WRAP_IDEAL_LEN | int | 引导时，内核命令行会被记录到控制台。日志消息以前缀 "Kernel command line: " 开头。该日志消息会尝试自动换行（拆分为多行…… |
| CODE_TAGGING | bool | 跟踪分配源代码并记录在该代码位置发起的分配总大小。该机制可用于以较低的性能与内存开销跟踪内存泄漏。 |
| COMPACTION | bool | 内存规整是唯一能可靠形成高阶（更大物理连续）内存块的内存管理组件。页分配器高度依赖内存规整，缺乏该特性…… |
| COMPACT_UNEVICTABLE_DEFAULT | int | 空闲页汇报允许从伙伴分配器增量获取空闲页，以便将这些页汇报给另一实体（如 hypervisor），从而让内存…… |
| COMPAT_BINFMT_ELF | def_bool | ELF FDPIC 二进制基于 ELF，但允许二进制文件的各个加载段在内存中彼此独立地定位。这使得该格式非常适合用于……环境 |
| COMPAT_BRK | bool | 随机化堆布局使堆利用攻击更困难，但也会破坏古老的二进制（包括任何基于 libc5 的程序）。此选项将引导默认改为禁用堆随机化…… |
| COMPAT_NETLINK_MESSAGES | def_bool | 此选项使得可以根据任务是否为 compat 任务，向任务发送不同的 netlink 消息。为此，你需要将 skb_shinfo(skb)->frag_list 设置为…… |
| COMPILE_TEST | bool | 某些驱动可以在与其运行平台不同的平台上编译。尽管它们无法在那里加载（或即便加载也因缺少硬件支持而无法使用）…… |
| CONSOLE_LOGLEVEL_DEFAULT | int | 决定控制台将打印哪些内容的默认日志级别。在此设置默认值等同于在内核引导参数中传入 loglevel=<x>。loglevel=<x> 仍会覆盖此处设置…… |
| CONSOLE_LOGLEVEL_QUIET | int | 当内核命令行传入 "quiet" 时使用的日志级别。当命令行传入 "quiet" 时，该日志级别将作为日志级别使用。换言之，传入 "quiet" 等效于…… |
| CONTEXT_ANALYSIS_TEST | bool | 构建用于基于编译器的上下文分析的测试。该测试不会向内核添加可执行代码，而是用于验证分析所支持的常见模式不会导致…… |
| CONTIG_ALLOC | def_bool | 在页分配器中，PCP（每 CPU 页集）以批处理方式补充与清空。批次数会自动缩放以改善页分配/释放吞吐。但过大的缩放因子可能损害…… |
| COREDUMP | bool | 此选项启用对执行核心转储的支持。你几乎肯定应当在此选 Y。对于从不需要调试或只运行无瑕疵代码的系统则非必需。 |
| CORE_DUMP_DEFAULT_ELF_HEADERS | bool | ELF 核心转储文件描述崩溃进程的每个内存映射，并可包含或省略其中每一个的内存内容。未修改的代码段映射内容默认被省略。 |
| CPUMASK_KUNIT_TEST | tristate | 启用 cpumask 测试，在引导或模块加载时运行。有关 KUnit 与单元测试的更多信息，请参阅 Documentation/dev-tools/kunit 中的 KUnit 文档…… |
| CPUSETS | bool | 此选项允许你创建与管理 CPUSET，从而将系统动态划分为若干 CPU 与内存节点集合，并将任务限定为只能在这些集合内运行。这主要用于…… |
| CPUSETS_V1 | bool | 已被 cgroup v2 实现废弃的传统 cgroup v1 cpusets 控制器。v1 保留给尚未迁移到新 cgroup v2 接口的遗留应用使用。遗留…… |
| CPU_HOTPLUG_STATE_CONTROL | bool | 允许将 "offline" 与 "online" 之间的各个步骤写入 CPU 的 sysfs 目标文件，从而可以逐步精细地切换状态。目前这仍是一个调试选项，因为热插拔机制无法被停止…… |
| CPU_ISOLATION | bool | 确保运行关键任务的 CPU 不受任何“噪声”源（如未绑定的工作队列、定时器、内核线程……）干扰。未绑定的任务会被转移到管家 CPU 上。该特性由…… |
| CROSS_MEMORY_ATTACH | bool | 启用此选项会添加 process_vm_readv 与 process_vm_writev 系统调用，允许拥有相应权限的进程直接读取或写入另一进程的地址空间。 |
| CRYPTO | tristate | 此选项提供核心加密 API（若 CRYPTO）。 |
| CRYPTO_842 | tristate | IBM 的 842 压缩算法。更多信息请参阅 https://github.com/plauth/lib842。 |
| CRYPTO_ADIANTUM | tristate | Adiantum 可调整、保持长度的加密模式。专为快速且安全的磁盘加密而设计，尤其适用于没有专用加密指令的 CPU。它使用 XCha……对每个扇区进行加密 |
| CRYPTO_AEGIS128 | tristate | AEGIS-128 AEAD 算法 |
| CRYPTO_AEGIS128_SIMD | bool | AEGIS-128 AEAD 算法。体系结构：arm 或 arm64，使用：- NEON（Advanced SIMD）扩展 |
| CRYPTO_AES | tristate | AES 密码算法（Rijndael）（FIPS-197, ISO/IEC 18033-3）。Rijndael 在广泛的软硬件计算环境中都表现稳定且优异…… |
| CRYPTO_ALGAPI | tristate | 此选项提供加密算法的 API。 |
| CRYPTO_ALGAPI2 | tristate | 提供实例化 cbc(aes) 等模板的支持，以及加密自检测试的支持。 |
| CRYPTO_ANUBIS | tristate | Anubis 密码算法。Anubis 是一种可变密钥长度密码，可使用 128 位至 320 位的密钥。它曾作为 NESSIE 竞赛的候选算法参评。 |
| CRYPTO_ARC4 | tristate | ARC4 密码算法。ARC4 是一种流密码，使用 8 位至 2048 位长度的密钥。该算法是基于驱动的 WEP 所必需的，但不应将其用于其他目的…… |
| CRYPTO_ARIA | tristate | ARIA 密码算法（RFC5794）。ARIA 是大韩民国的标准加密算法，规定了三种密钥长度与轮数：128 位 12 轮、192 位 14 轮、256 位 16 轮…… |
| CRYPTO_AUTHENC | tristate | Authenc：IPsec 的组合模式封装。IPSec ESP（XFRM_ESP）需要它。 |
| CRYPTO_BENCHMARK | tristate | 一个简单粗暴的加密基准测试模块，主要供在内核中开发加密算法的人使用。生产内核不应启用它。 |
| CRYPTO_BLAKE2B | tristate | BLAKE2b 加密哈希函数（RFC 7693）。BLAKE2b 针对 64 位平台优化，可生成 1 至 64 字节任意长度的摘要。也实现了带密钥的哈希。该模块…… |
| CRYPTO_BLOWFISH | tristate | Blowfish 密码算法，由 Bruce Schneier 设计。这是一种可变密钥长度的密码，可使用 32 位至 448 位的密钥。它快速、简单，专为“大型”……设计 |
| CRYPTO_BLOWFISH_COMMON | tristate | 由通用 C 实现与汇编实现共享的 Blowfish 密码算法公共部分。 |
| CRYPTO_CAMELLIA | tristate | Camellia 密码算法（ISO/IEC 18033-3）。Camellia 是由 NTT 与三菱电机联合开发的对称密钥分组密码，规定了三种密钥长度：128、192、256 位…… |
| CRYPTO_CAST5 | tristate | CAST5（CAST-128）密码算法（RFC2144, ISO/IEC 18033-3） |
| CRYPTO_CAST6 | tristate | CAST6（CAST-256）加密算法（RFC2612） |
| CRYPTO_CAST_COMMON | tristate | 由通用 C 实现与汇编实现共享的 CAST 密码算法公共部分。 |
| CRYPTO_CBC | tristate | CBC（密码分组链接）模式（NIST SP800-38A）。IPSec ESP（XFRM_ESP）需要此分组密码模式。 |
| CRYPTO_CCM | tristate | CCM（计数器与密码分组链接-消息认证码）认证加密模式（NIST SP800-38C） |
| CRYPTO_CHACHA20 | tristate | ChaCha20、XChaCha20 与 XChaCha12 流密码算法。ChaCha20 是由 Daniel J. Bernstein 设计的 256 位高速流密码，并在 RFC7539 中进一步规范，用于 IETF 协议…… |
| CRYPTO_CHACHA20POLY1305 | tristate | ChaCha20 流密码与 Poly1305 认证器的组合模式（RFC8439） |
| CRYPTO_CMAC | tristate | CMAC（基于密码的消息认证码）认证模式（NIST SP800-38B 与 IETF RFC4493） |
| CRYPTO_CRC32 | tristate | CRC32 CRC 算法（IEEE 802.3） |
| CRYPTO_CRC32C | tristate | 采用 iSCSI 多项式的 CRC32c CRC 算法（RFC 3385 与 RFC 3720）。一种 32 位 CRC（循环冗余校验），其多项式由 G. Castagnoli、S. Braeuer 与 M. Herrman 在《Optimization……》中定义 |
| CRYPTO_CRYPTD | tristate | 这是一个通用的软件异步加密守护进程，可将任意同步软件加密算法转换为在内核线程中执行的异步算法。 |
| CRYPTO_CTR | tristate | CTR（计数器）模式（NIST SP800-38A） |
| CRYPTO_CTS | tristate | CTS（密文窃取）的 CBC-CS3 变体（NIST SP800-38A 增补（2010 年 10 月））。AES 加密的 Kerberos gss 机制支持需要此模式。 |
| CRYPTO_DEFLATE | tristate | Deflate 压缩算法（RFC1951）。由 IPSec 配合 IPCOMP 协议使用（RFC3173, RFC2394） |
| CRYPTO_DES | tristate | DES（数据加密标准）（FIPS 46-2, ISO/IEC 18033-3）与三重 DES EDE（加密/解密/加密）（FIPS 46-3, ISO/IEC 18033-3）密码算法 |
| CRYPTO_DH | tristate | DH（Diffie-Hellman）密钥交换算法 |
| CRYPTO_DH_RFC7919_GROUPS | bool | RFC7919 中定义的 FFDHE（基于有限域的临时 Diffie-Hellman）组。在 DH 密钥交换中支持这些有限域组：- ffdhe2048、ffdhe3072、ffdhe4096、ffdhe6144、ffdhe8192。若不确定…… |
| CRYPTO_DRBG | tristate | 来自 Jitterentropy 库的 CPU Jitter RNG（随机数生成器）。一种非物理、非确定性的（“真”）RNG（例如符合 NIST SP800-90B 的熵源），旨在提供…… |
| CRYPTO_DRBG_CTR | bool | NIST SP800-90A 定义的 CTR_DRBG 变体。它使用 AES 密码算法配合计数器分组模式。 |
| CRYPTO_DRBG_HMAC | bool | NIST SP800-90A 定义的 Hash_DRBG 变体。它使用 SHA-1、SHA-256、SHA-384 或 SHA-512 哈希算法。 |
| CRYPTO_DRBG_MENU | tristate | DRBG（确定性随机比特生成器）（NIST SP800-90A）。在随后的子菜单中，必须选择一种或多种 DRBG 类型（若 CRYPTO_DRBG_MENU）。 |
| CRYPTO_ECB | tristate | ECB（电子密码本）模式（NIST SP800-38A） |
| CRYPTO_ECC | tristate | 使用 P-192、P-256 与 P-384 曲线的 ECDH（椭圆曲线 Diffie-Hellman）密钥交换算法（FIPS 186） |
| CRYPTO_ECDSA | tristate | 使用 P-192、P-256、P-384 与 P-521 曲线的 ECDSA（椭圆曲线数字签名算法）（FIPS 186, ISO/IEC 14888-3）。目前仅实现签名验证。 |
| CRYPTO_ECHAINIV | tristate | 加密链式 IV 生成器。该 IV 生成器基于序列号与盐异或后再加密来生成 IV。这是 CBC 的默认算法。 |
| CRYPTO_ECRDSA | tristate | 椭圆曲线俄罗斯数字签名算法（GOST R 34.10-2012, RFC 7091, ISO/IEC 14888-3）。俄罗斯密码标准算法之一（称为 GOST 算法）。目前仅实现签名验证…… |
| CRYPTO_ESSIV | tristate | 加密盐-扇区 IV 生成器。该 IV 生成器在某些情况下由 fscrypt 和/或 dm-crypt 使用。它使用块加密密钥的哈希作为块加密遍的对称密钥…… |
| CRYPTO_FCRYPT | tristate | RxRPC 使用的 FCrypt 算法。参见 https://ota.polyonymo.us/fcrypt-paper.txt |
| CRYPTO_FIPS | bool | 此选项启用 fips 引导参数，若希望系统在 FIPS 200 认证下运行则需要它。除非你知道它的用途，否则应选否。 |
| CRYPTO_FIPS_CUSTOM_VERSION | bool | 此选项提供覆盖 FIPS 模块版本的能力。默认使用 KERNELRELEASE 值。 |
| CRYPTO_FIPS_NAME | string | 此选项设置由 Crypto API 通过 /proc/sys/crypto/fips_name 文件报告的 FIPS 模块名称。 |
| CRYPTO_GCM | tristate | GCM（Galois/计数器模式）认证加密模式与 GMAC（GCM 消息认证码）（NIST SP800-38D）。IPSec ESP（XFRM_ESP）需要它。 |
| CRYPTO_GENIV | tristate | 序列号 IV 生成器。该 IV 生成器通过将序列号与盐异或来生成 IV。该算法主要对 CTR 有用。IPsec ESP（XFRM_ESP）需要它。 |
| CRYPTO_HCTR2 | tristate | HCTR2 保持长度的加密模式。一种用于存储加密的模式，在带有加速 AES 与无进位乘法的指令的处理器（如带有 AES-……的 x86 处理器）上效率很高 |
| CRYPTO_HMAC | tristate | HMAC（带密钥的哈希消息认证码）（FIPS 198 与 RFC2104）。IPsec AH（XFRM_AH）与 IPsec ESP（XFRM_ESP）需要它。 |
| CRYPTO_JITTERENTROPY_MEMORY_BLOCKS | int | 启用哈希算法的用户空间接口。请参阅 Documentation/crypto/userspace-if.rst 与 https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_JITTERENTROPY_MEMSIZE_2 | bool | Jitter RNG 允许指定过采样率（OSR）。Jitter RNG 的运行需要固定数量的定时测量才能产生一个随机数输出块。OSR…… |
| CRYPTO_JITTERENTROPY_TESTINTERFACE | bool | 该测试接口允许特权进程捕获 Jitter RNG 收集的原始、未调节的高分辨率时间戳噪声，以供统计分析。由于这些数据被用作…… |
| CRYPTO_KHAZAD | tristate | Khazad 密码算法。Khazad 是首届 NESSIE 竞赛的决赛算法。它是一种针对 64 位处理器优化、在 32 位处理器上也有良好表现的算法。Khazad 使用 128 位…… |
| CRYPTO_KRB5ENC | tristate | 针对 Kerberos 5 RFC3961 简化配置文件的组合哈希与密码支持。sunrpc/NFS 与 rxrpc/AFS 所使用的 Kerberos 5 风格加密需要它。 |
| CRYPTO_LRW | tristate | LRW（Liskov Rivest Wagner）模式。一种可调整、不可塑、不可移动的窄分组密码模式，用于 dm-crypt。配合密码规格字符串 aes-lrw-benbi 使用，密钥必须为 256、320 或 384…… |
| CRYPTO_LZ4 | tristate | LZ4 压缩算法。更多信息请参阅 https://github.com/lz4/lz4。 |
| CRYPTO_LZ4HC | tristate | LZ4 高压缩模式算法。更多信息请参阅 https://github.com/lz4/lz4。 |
| CRYPTO_LZO | tristate | LZO 压缩算法。更多信息请参阅 https://www.oberhumer.com/opensource/lzo/。 |
| CRYPTO_MANAGER2 | def_tristate | 针对 cbc(aes) 等加密实例的用户空间配置。 |
| CRYPTO_MD4 | tristate | MD4 消息摘要算法（RFC1320） |
| CRYPTO_MD5 | tristate | MD5 消息摘要算法（RFC1321），包含 HMAC 支持。 |
| CRYPTO_MLDSA | tristate | ML-DSA（基于模块格的数字签名算法）（FIPS-204）。目前仅实现签名验证。 |
| CRYPTO_NULL | tristate | 这些是 IPsec 使用的“Null”算法，它们不做任何事。 |
| CRYPTO_PCBC | tristate | PCBC（传播式密码分组链接）模式。RxRPC 需要此分组密码模式。 |
| CRYPTO_PCRYPT | tristate | 这将任意加密算法转换为在内核线程中执行的并行算法。 |
| CRYPTO_RMD160 | tristate | RIPEMD-160 哈希函数（ISO/IEC 10118-3）。RIPEMD-160 是一种 160 位加密哈希函数，旨在作为 128 位哈希函数 MD4、MD5 及其前身的安全替代品…… |
| CRYPTO_SEED | tristate | SEED 密码算法（RFC4269, ISO/IEC 18033-3）。SEED 是一种 128 位对称密钥分组密码，由 KISA（韩国互联网与安全局）作为国家标准加密算法开发…… |
| CRYPTO_SELFTESTS | bool | 启用加密自检测试。加密自检测试在引导时运行，或在算法注册时（若算法稍后动态加载）运行。主要有两种使用场景…… |
| CRYPTO_SELFTESTS_FULL | bool | 为每个算法启用完整的加密自检测试集。完整测试集应在开发与发布前测试时启用，但不应在生产内核中启用。所有加密代码…… |
| CRYPTO_SERPENT | tristate | Serpent 密码算法，由 Anderson、Biham 与 Knudsen 设计。密钥长度允许为 0 至 256 位，以 8 位为步长。更多信息请参阅 https://www.cl.cam.ac.uk/~rja14/serpent.html…… |
| CRYPTO_SHA1 | tristate | SHA-1 安全哈希算法（FIPS 180, ISO/IEC 10118-3），包含 HMAC 支持。 |
| CRYPTO_SHA256 | tristate | SHA-224 与 SHA-256 安全哈希算法（FIPS 180, ISO/IEC 10118-3），包含 HMAC 支持。IPsec AH（XFRM_AH）与 IPsec ESP（XFRM_ESP）需要它。 |
| CRYPTO_SHA3 | tristate | SHA-3 安全哈希算法（FIPS 202, ISO/IEC 10118-3） |
| CRYPTO_SHA512 | tristate | SHA-384 与 SHA-512 安全哈希算法（FIPS 180, ISO/IEC 10118-3），包含 HMAC 支持。 |
| CRYPTO_SIMD | tristate | RSA（Rivest-Shamir-Adleman）公钥算法（RFC8017） |
| CRYPTO_SM3 | tristate | SM3（商密 3）安全哈希函数（OSCCA GM/T 0004-2012, ISO/IEC 10118-3）。这是中国商用密码体系的一部分。参考：http://www.oscca.gov.cn/UpFile/20101222141857786…… |
| CRYPTO_SM4 | tristate | SM4 密码算法（OSCCA GB/T 32907-2016, ISO/IEC 18033-3:2010/Amd 1:2021）。SM4（GBT.32907-2016）是由中国国家商用密码管理办公室发布的密码标准…… |
| CRYPTO_STREEBOG | tristate | Streebog 哈希函数（GOST R 34.11-2012, RFC 6986, ISO/IEC 10118-3）。这是俄罗斯密码标准算法之一（称为 GOST 算法）。此设置启用两种哈希算法…… |
| CRYPTO_TEA | tristate | TEA（微型加密算法）密码算法。Tiny Encryption Algorithm 是一种使用多轮以保证安全的简单密码，速度极快且占用内存很少。扩展微型加密算法…… |
| CRYPTO_TWOFISH | tristate | Twofish 密码算法。Twofish 由 CounterPane Systems 的研究人员作为 AES（高级加密标准）候选密码提交。它是一种 16 轮分组密码，支持……的密钥长度 |
| CRYPTO_TWOFISH_COMMON | tristate | 由通用 C 实现与汇编实现共享的 Twofish 密码算法公共部分。 |
| CRYPTO_USER_API_AEAD | tristate | 启用 AEAD 密码算法的用户空间接口。请参阅 Documentation/crypto/userspace-if.rst 与 https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_ENABLE_OBSOLETE | bool | 允许选择那些已从内核内部使用中淘汰、仅对仍依赖它们的用户空间客户端有用的过时加密算法。 |
| CRYPTO_USER_API_RNG | tristate | 启用 RNG（随机数生成器）算法的用户空间接口。请参阅 Documentation/crypto/userspace-if.rst 与 https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_RNG_CAVP | bool | 在用户空间接口中启用额外的 API，用于 NIST CAVP（加密算法验证程序）测试：- 重置 DRBG 熵 - 提供附加数据。此选项只应…… |
| CRYPTO_USER_API_SKCIPHER | tristate | 启用对称密钥密码算法的用户空间接口。请参阅 Documentation/crypto/userspace-if.rst 与 https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_WP512 | tristate | Whirlpool 哈希函数（ISO/IEC 10118-3），支持 512、384 与 256 位哈希。Whirlpool-512 是 NESSIE 密码原语的一部分。参见 https://web.archive.org/web/20171129084214/http://www.larc.u…… |
| CRYPTO_XCBC | tristate | XCBC-MAC（扩展密码分组链接消息认证码）（RFC3566） |
| CRYPTO_XCTR | tristate | 用于 HCTR2 的 XCTR（XOR 计数器）模式。该分组密码模式是 CTR 模式的一种变体，使用 XOR 与小端加法而非大端运算。XCTR 模式用于实现 HCTR2。 |
| CRYPTO_XTS | tristate | XTS（带密文窃取的 XOR 加密 XOR）模式（NIST SP800-38E 与 IEEE 1619）。配合 aes-xts-plain 使用，密钥长度为 256、384 或 512 位。此实现当前无法处理……的扇区大小 |
| CRYPTO_XXHASH | tristate | xxHash 非加密哈希算法。速度极快，接近 RAM 极限。 |
| CRYPTO_ZSTD | tristate | zstd 压缩算法。更多信息请参阅 https://github.com/facebook/zstd。 |
| CSD_LOCK_WAIT_DEBUG | bool | 当 CPU 对 smp_call_function*() IPI 封装响应缓慢时，此选项启用调试打印。这些调试打印包含当前正在执行的 IPI 处理函数（若有）及相关的…… |
| CSD_LOCK_WAIT_DEBUG_DEFAULT | bool | 此选项使 csdlock_debug= 内核引导参数默认为 1（基本调试）而非 0（无调试）。 |
| DCACHE_WORD_ACCESS | bool | 启用此项可在文件系统注册时对其参数描述进行校验。 |
| DEBUG_ATOMIC | bool | 若在此选 Y，内核将为原子访问添加运行时对齐检查。对于不对未对齐访问产生陷阱的体系结构很有用。此选项可能带来显著的…… |
| DEBUG_ATOMIC_LARGEST_ALIGN | bool | 若在此选 Y，则对原子访问自然对齐的检查将被限制为编译器对标量类型的最大对齐。 |
| DEBUG_ATOMIC_SLEEP | bool | 若在此选 Y，各类可能休眠的例程若在内核原子段内被调用（持有自旋锁时、处于 rcu 读端临界区时、处于抢占……时）将产生大量告警。 |
| DEBUG_BUGVERBOSE | bool | 在此选 Y 可使 BUG() panic 输出 BUG 调用的文件名与行号，以及 EIP 与 oops 跟踪。这有助于调试，但会占用约 70-100K 内存。 |
| DEBUG_BUGVERBOSE_DETAILED | bool | 在此选 Y 可使 WARN_ON_ONCE() 除文件名与行号外，还输出警告的条件字符串。这有助于调试，但占用约 100K 内存。若不确定，选 N。 |
| DEBUG_CGROUP_REF | bool | 强制 cgroup css 引用计数函数不被内联，以便可通过 kprobe 进行调试。 |
| DEBUG_CLOSURES | bool | 将所有活跃的 closure 保存在链表中，并提供 debugfs 接口列出它们，从而可以查看卡住的异步操作。 |
| DEBUG_FORCE_FUNCTION_ALIGN_64B | bool | 存在这样的情况：来自某个领域的提交改变了其他领域的函数地址对齐，导致神奇的性能波动（回归或提升）。启用此选项有助于…… |
| DEBUG_FORCE_WEAK_PER_CPU | bool | s390 与 alpha 要求模块中的 percpu 变量定义为弱符号，以规避寻址范围问题，这给 percpu 变量定义带来以下两条限制：1. percpu 符号…… |
| DEBUG_FS | bool | debugfs 是内核开发者用来放置调试文件的虚拟文件系统。启用此选项以便能够读写这些文件。有关 debugfs 的详细文档…… |
| DEBUG_FS_ALLOW_ALL | bool | 不施加任何限制。API 与文件系统注册均开启。这是正常的默认操作。 |
| DEBUG_FS_ALLOW_NONE | bool | 禁用访问。客户端尝试在 debugfs 树中创建节点时会收到 -PERM，且 debugfs 不会被注册为文件系统。客户端随后可退避或在没有 debugfs 访问的情况下继续。 |
| DEBUG_HIGHMEM | bool | 此选项为高内存系统启用额外的错误检查。生产系统应禁用。 |
| DEBUG_INFO | bool | 在下面的“调试信息”选择中已选中“None”以外的内核调试信息选项，表示将为构建目标生成调试信息。# Clang 生成 .ule…… |
| DEBUG_INFO_BTF | bool | 从 DWARF 调试信息生成去重后的 BTF 类型信息。开启它需要 pahole v1.22 或更高版本，它会将 DWARF 类型信息转换为等效的去重 BTF 类型信息。 |
| DEBUG_INFO_BTF_MODULES | bool | 为内核模块生成紧凑的拆分 BTF 类型信息。 |
| DEBUG_INFO_COMPRESSED_NONE | bool | 不压缩调试信息段。 |
| DEBUG_INFO_COMPRESSED_ZLIB | bool | 使用 zlib 压缩调试信息。通过 debian/rules 使用 dpkg-deb 的用户可能会发现其调试 .deb 包体积因调试信息被压缩而增大…… |
| DEBUG_INFO_COMPRESSED_ZSTD | bool | 使用 zstd 压缩调试信息。在相近的时间开销下，它比 zlib 提供更好的压缩率，但需要较新的工具链支持。需要 GCC 13.0+ 或 Clang 16.0+…… |
| DEBUG_INFO_DWARF4 | bool | 生成 DWARF v4 调试信息。这需要 gcc 4.5+、若使用不带 clang 集成汇编器的 clang 则需要 binutils 2.35.2、以及 gdb 7.0+。若你有尚未准备好……的 DWARF 调试信息消费者 |
| DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT | bool | 工具链生成的 DWARF 调试信息的隐式默认版本会随时间变化。这可能破坏尚未升级以支持新版本的调试信息消费者，并阻止…… |
| DEBUG_INFO_NONE | bool | 构建内核时不包含调试信息，从而生成更快且更小的构建。 |
| DEBUG_INFO_SPLIT | bool | 将调试信息生成到独立的 .dwo 文件中。这显著减小了带 DEBUG_INFO 构建的构建目录体积，因为它只在磁盘上的 .dwo 文件中存储一次信息，而非…… |
| DEBUG_IRQFLAGS | bool | 启用对可能不安全的中断启用/禁用操作的检查，例如在中断已启用时调用 raw_local_irq_restore()。 |
| DEBUG_KERNEL | bool | 若你正在开发驱动或尝试调试并定位内核问题，在此选 Y。 |
| DEBUG_KMAP_LOCAL | bool | 此选项为 kmap_local 基础设施启用额外的错误检查。生产环境应禁用。 |
| DEBUG_KOBJECT | bool | 若在此选 Y，一些额外的 kobject 调试消息将被发送到 syslog。 |
| DEBUG_KOBJECT_RELEASE | bool | kobject 是引用计数的对象。这意味着它们的最后一次引用计数释放是不可预测的，且 kobject 可能在驱动决定丢弃其初始……之后继续存活 |
| DEBUG_LOCKDEP | bool | 若在此选 Y，锁依赖引擎将执行额外的运行时检查以自我调试，代价是更多运行时开销。 |
| DEBUG_LOCKING_API_SELFTESTS | bool | 若希望内核在引导时运行一段简短的自检测试，在此选 Y。该自检测试会检查常见类型的加锁缺陷是否能被调试机制检测到（若你禁用锁…… |
| DEBUG_LOCK_ALLOC | bool | 该特性将检查任何被持有的锁（自旋锁、rwlock、互斥体或 rwsem）是否被内核通过任一内存释放例程（kfree()、kmem_cache_free()、free_pages()……）错误地释放 |
| DEBUG_MAPLE_TREE | bool | 启用 maple tree 调试信息与额外验证。若不确定，选 N。 |
| DEBUG_MEMORY_INIT | bool | 启用此项以在内存初始化期间进行额外检查。健全性检查会校验 VM 的各个方面，例如内存模型以及体系结构提供的其他信息。详细信…… |
| DEBUG_MISC | bool | 若你需要启用本应归属某个更具体的调试选项、但实际上并未归类的杂项调试代码，在此选 Y。 |
| DEBUG_MUTEXES | bool | 该特性允许检测并报告对互斥体语义的违反。 |
| DEBUG_NOMMU_REGIONS | bool | 此选项使匿名与私有映射区域的全局树被定期检查是否存在无效拓扑。 |
| DEBUG_NOTIFIERS | bool | 启用此项以开启对通知链（notifier call chain）的健全性检查。这对内核开发者确保模块正确地从通知链注销最为有用。这是…… |
| DEBUG_OBJECTS | bool | 若在此选 Y，内核中将插入额外代码以跟踪各类对象的生命周期，并验证针对这些对象的操作。 |
| DEBUG_OBJECTS_ENABLE_DEFAULT | int | 调试对象的引导参数默认值 |
| DEBUG_OBJECTS_FREE | bool | 这启用检查：k/v 释放操作是否释放了包含尚未正确去激活对象的区域。这可能使 kmalloc/kfree 密集型工作负载明显变慢。 |
| DEBUG_OBJECTS_PERCPU_COUNTER | bool | 若在此选 Y，将在 percpu 计数器例程中插入额外代码，以跟踪 percpu 计数器对象的生命周期并验证 percpu 计数器操作。 |
| DEBUG_OBJECTS_RCU_HEAD | bool | 启用此项以开启对 RCU 链表头（call_rcu() 用法）的调试。 |
| DEBUG_OBJECTS_SELFTEST | bool | 这启用对象调试代码的自检测试。 |
| DEBUG_OBJECTS_TIMERS | bool | 若在此选 Y，将在定时器例程中插入额外代码，以跟踪定时器对象的生命周期并验证定时器操作。 |
| DEBUG_OBJECTS_WORK | bool | 若在此选 Y，将在工作队列例程中插入额外代码，以跟踪工作对象的生命周期并验证工作操作。 |
| DEBUG_PERF_USE_VMALLOC | bool | 使用 vmalloc 内存作为 perf mmap() 缓冲区的后备。主要用于在不要求它的平台上调试 vmalloc 代码。若不确定，选 N。 |
| DEBUG_PER_CPU_MAPS | bool | 选 Y 以验证被访问的 per_cpu 映射已建立。这会给内核内存增加相当多的代码并降低性能。若不确定，选 N。 |
| DEBUG_PLIST | bool | 启用此项以开启对按优先级排序的链表（plist）遍历例程的扩展检查。它会在每次操作时多次检查整个链表。若不确定，选 N。 |
| DEBUG_PREEMPT | bool | 若在此选 Y，内核将使用常用 smp_processor_id() 函数的调试变体，并在内核代码以不安全于抢占的方式使用它时打印警告。此外，内核…… |
| DEBUG_RSEQ | bool | 为 rseq 系统调用启用额外的调试检查。若不确定，选 N。 |
| DEBUG_RT_MUTEXES | bool | 这允许自动检测并报告对 rt 互斥体语义的违反，以及 rt 互斥体相关的死锁（lockup）。 |
| DEBUG_RWSEMS | bool | 该调试特性允许检测并报告不匹配的读写信号量加锁与解锁。 |
| DEBUG_SECTION_MISMATCH | bool | 段不匹配分析检查是否存在从一个段到另一个段的非法引用。在链接时或运行时，某些段会被丢弃；任何对这些段中先前代码/数据的使用…… |
| DEBUG_SG | bool | 启用此项以开启对散列聚集（scatter-gather）表的检查。这有助于发现未正确初始化其 sg 表的驱动问题。若不确定，选 N。 |
| DEBUG_SHIRQ | bool | 启用此项以在共享中断处理程序注销前生成一个伪中断（注册时生成目前被禁用）。驱动需要正确处理它。若…… |
| DEBUG_SPINLOCK | bool | 在此选 Y 并构建 SMP，可捕获缺失的自旋锁初始化以及某些其他常见的自旋锁错误。最好与 NMI 看门狗配合使用，以便自旋锁…… |
| DEBUG_STACK_USAGE | bool | 启用在 sysrq-T 与 sysrq-P 调试输出中显示每个任务曾经可用的最小空闲栈空间。当进程退出时，若该进程……还会向 dmesg 发送一条消息 |
| DEBUG_VFS | bool | 启用此项以开启 VFS 层中可能影响性能的扩展检查。若不确定，选 N。 |
| DEBUG_VM_IRQSOFF | def_bool | 启用此项以开启虚拟内存系统中可能影响性能的扩展检查。若不确定，选 N。 |
| DEBUG_VM_MAPLE_TREE | bool | 启用 VM maple tree 调试信息与额外验证。若不确定，选 N。 |
| DEBUG_VM_PGFLAGS | bool | 启用对页标志操作的额外验证。若不确定，选 N。 |
| DEBUG_VM_PGTABLE | bool | 此选项提供一种调试方法，可用于在各种平台上测试体系结构的页表辅助函数，验证其是否符合预期的通用 MM 语义。这将…… |
| DEBUG_VM_RB | bool | 启用 VM 红黑树调试信息与额外验证。若不确定，选 N。 |
| DEBUG_VM_SHOOT_LAZIES | bool | 启用额外的 IPI，确保 lazy tlb mm 引用在 mm 被释放前被移除。若不确定，选 N。 |
| DEBUG_WQ_FORCE_RR_CPU | bool | 工作队列曾经隐式保证：未显式指定 CPU 的排队工作项会被放到本地 CPU 上。这一保证已不再成立，虽然本地 CPU 仍是首选，但工作…… |
| DEBUG_WW_MUTEX_SLOWPATH | bool | 该特性通过注入额外的 -EDEADLK 回退（wound/backoff）用例，为 w/w 互斥体使用者启用慢速路径测试。配合（CONFIG_PROVE_LOCKING）启用的完整互斥体检查，这将测试…… |
| DEFAULT_HOSTNAME | string | 此选项决定在用户空间调用 sethostname(2) 之前的默认系统主机名。内核传统上在此使用“(none)”，但你可能希望在此使用一个不同默认值以生成最小…… |
| DEFAULT_HUNG_TASK_TIMEOUT | int | 此选项控制用于判断任务何时变得无响应并应被视为挂起的默认超时（秒）。它可在运行时通过 kernel.hung_task_t……调整 |
| DEFAULT_INIT | string | 若内核命令行未传入 init= 选项，此选项决定系统的默认 init。若所请求的路径不存在，我们仍会继续尝试进一步的…… |
| DEFAULT_MMAP_MIN_ADDR | int | 这是应当被保护以免受用户空间分配的低虚拟内存部分。阻止用户写入低地址页有助于降低内核空指针缺陷的影响。有关…… |
| DEFAULT_SECURITY_SELINUX | bool | 以逗号分隔的 LSM 列表，按初始化顺序。任何未列入此列表的 LSM（除那些具有 LSM_ORDER_FIRST 与 LSM_ORDER_LAST 顺序、在……选中时总是启用的除外）…… |
| DEFERRED_STRUCT_PAGE_INIT | bool | 通常所有 struct page 都在早期引导期间以单线程方式初始化。在非常大的机器上，这可能耗费相当多时间。若设置此选项，大型机器将…… |
| DETECT_HUNG_TASK | bool | 在此选 Y 以让内核检测“hung tasks（挂起任务）”，即导致任务无限期卡在不可中断“D”状态的缺陷。当检测到挂起任务时，内核将打印…… |
| DETECT_HUNG_TASK_BLOCKER | bool | 在此选 Y 以显示获取了“hung tasks”正在等待的互斥锁的阻塞任务的栈跟踪。这会增加少量开销，但能显示可疑任务及其调用跟踪（若其来自……） |
| DIMLIB | tristate | 动态中断调节库。实现一种根据运行时性能动态改变 CQ 调节值的算法。# # libfdt 文件，仅在需要时选中。# |
| DST_CACHE | bool | NET_SOCK_MSG 为普通套接字（如 TCP）或 ULP（上层模块，如 TLS）提供一个借助 BPF 程序处理 L7 应用数据的框架。 |
| DYNAMIC_DEBUG | bool | 将调试级别消息编译进内核，否则这些消息在运行时不可用。随后可基于不同作用域级别（按源文件、函数……）启用/禁用这些消息。 |
| DYNAMIC_DEBUG_CORE | bool | 启用 dynamic debug 的核心功能支持。当你希望将 dynamic debug 与你为每个模块定义的 DYNAMIC_DEBUG_MODULE 关联到内核模块时很有用，尤其适用于……的情况 |
| ELFCORE | bool | 此选项启用 kernel/elfcore.o。 |
| ELF_CORE | bool | 启用对生成核心转储的支持。禁用可节省约 4k。 |
| ETHTOOL_NETLINK | bool | 基于通用 netlink 的 ethtool 替代性用户空间接口。它提供更好的可扩展性以及一些新特性，如通知消息。 |
| EVENTFD | bool | 启用 eventfd() 系统调用，它允许接收内核通知（如 KAIO）或用户空间通知。若不确定，选 Y。 |
| EXEC_KUNIT_TEST | bool | 构建 exec 的 KUnit 测试，测试 exec 内部各方面边界条件。 |
| EXT_GROUP_SCHED | bool | 该特性让调度器基于当前在该 CPU 上可调度的 RUNNABLE 任务跟踪每个 CPU 的钳位利用率。启用此选项后，用户可指定最小与…… |
| FAILOVER | tristate | failover 模块为半虚拟化驱动提供一个通用接口，用于向 failover 实例注册一个 netdev 与一组操作。这些操作作为事件处理程序被调用来处理…… |
| FAILSLAB | bool | 为 kmalloc 提供故障注入能力。 |
| FAIL_FUNCTION | bool | 提供基于函数的故障注入能力。这将允许你用给定返回值的返回来覆盖特定函数。结果，函数调用者将看到一个错误值…… |
| FAIL_FUTEX | bool | 为 futex 提供故障注入能力。 |
| FAIL_IO_TIMEOUT | bool | 在端 IO 处理上提供故障注入能力。这将使块层“遗忘”一个按配置设定的中断，从而演练错误处理。仅适用于使用 g……的驱动 |
| FAIL_MAKE_REQUEST | bool | 为磁盘 IO 提供故障注入能力。 |
| FAIL_MMC_REQUEST | bool | 为 MMC IO 提供故障注入能力。这将使 mmc 核心返回数据错误。这有助于测试 mmc 块设备中的错误处理，以及测试 mmc 主机驱动…… |
| FAIL_PAGE_ALLOC | bool | 为 alloc_pages() 提供故障注入能力。 |
| FAIL_SKB_REALLOC | bool | 提供强制重新分配 skb 的故障注入能力，以捕获可能的 skb 无效指针。更多信息请参阅 Documentation/fault-injection/fault-injection.rst |
| FAIL_SUNRPC | bool | 为 SunRPC 及其消费者提供故障注入能力。 |
| FAULT_INJECTION | bool | 提供故障注入框架。更多细节请参阅 Documentation/fault-injection/。 |
| FAULT_INJECTION_CONFIGFS | bool | 此选项允许基于 configfs 的驱动通过 configfs 动态配置故障注入。每个驱动特定的故障注入参数可作为 configfs 属性显示在一个…… |
| FAULT_INJECTION_DEBUG_FS | bool | 通过 debugfs 启用故障注入能力的配置。 |
| FAULT_INJECTION_STACKTRACE_FILTER | bool | 为故障注入能力提供栈跟踪过滤器 |
| FAULT_INJECTION_USERCOPY | bool | 提供在 usercopy 函数（copy_from_user()、get_user()……）中注入失败的故障注入能力。 |
| FFS_KUNIT_TEST | tristate | 构建针对 ffs 系列位操作函数（包括 ffs()、__ffs()、fls()、__fls()、fls64() 与 __ffs64()）的 KUnit 测试。这些测试验证数学正确性、边界情况处理…… |
| FHANDLE | bool | 若在此选 Y，用户态程序便能将文件名映射为句柄，随后将该句柄用于不同的文件系统操作。这在实现用户空间文件服务……时很有用 |
| FIB_RULES | bool | 该特性提供一个支持 mpls 等轻量级隧道的基础设施。轻量级隧道端点不关联任何 netdevice。隧道封装参数存储于…… |
| FILE_LOCKING | bool | 此选项启用标准文件锁支持，这是 NFS 等文件系统以及 flock() 系统调用所必需的。禁用此选项可节省约 11k。 |
| FIND_BIT_BENCHMARK | tristate | 构建“test_find_bit”模块，用于测量 find_*_bit() 函数的性能。若不确定，选 N。 |
| FIND_BIT_BENCHMARK_RUST | tristate | 构建“find_bit_benchmark_rust”模块。它是一个微基准测试，测量与 C 中 find_*_bit() 操作对应的 Rust 函数的性能。它遵循 FIND_BI…… |
| FIND_NORMAL_PAGE | def_bool | 该体系结构使用 lazy MMU 模式。这允许对 MMU 相关体系结构状态的更改被推迟到退出该模式时进行。详见 <linux/pgtable.h>。 |
| FLATMEM_MANUAL | bool | 此选项最适合具有平坦地址空间的非 NUMA 系统。FLATMEM 在性能与资源消耗方面是最有效的系统，对于小……也是最佳选项 |
| FORCE_NR_CPUS | def_bool | 此选项提供用于简单文本模式匹配的 glob_match 函数。它起源于 ATA 代码以将特定驱动器型号列入黑名单，但其他设备驱动程序可能也需要类似…… |
| FORTIFY_KUNIT_TEST | tristate | 构建用于检查 FORTIFY_SOURCE 内部机制的单元测试，FORTIFY_SOURCE 被 str*() 与 mem*() 系列函数使用。要测试 FORTIFY_SOURCE 的运行时陷阱，请参阅 LKDTM 的“FORTIFY_*”测试。 |
| FPROBE_SANITY_TEST | bool | 此选项将在系统引导时启用对 fprobe 的测试。会执行一系列测试以验证 fprobe 工作正常。若不确定，选 N。 |
| FRAME_WARN | int | 告知编译器在构建时对此大小以上的栈帧发出警告。设置过低会导致大量警告。设为 0 则禁用该警告。 |
| FREEZER | def_bool |  |
| FS_DAX_PMD | bool | 此选项启用文件系统的导出操作，以支持外部块 IO。 |
| FS_IOMAP | bool | 直接访问（DAX）可用于内存后备的块设备。若块设备支持 DAX 且文件系统支持 DAX，你便可避免使用页缓存来缓冲 I/O。开启…… |
| FUNCTION_ERROR_INJECTION | bool | 将故障注入到内核中用 ALLOW_ERROR_INJECTION() 标注的各种函数中。BPF 也可能修改这些函数的返回值。这有助于测试错误路径…… |
| FUTEX | bool | 禁用此选项将导致内核在构建时不包含对“fast userspace mutexes（快速用户空间互斥体）”的支持。生成的内核可能无法正确运行基于 glibc 的应用。 |
| FUTEX_PI | bool | 禁用此选项将导致内核在构建时不包含对 epoll 系列系统调用的支持。 |
| GCD_KUNIT_TEST | tristate | 此选项启用针对 gcd() 函数（计算两个数的最大公约数）的 KUnit 测试套件。该测试套件在各种场景下验证 gcd() 的正确性…… |
| GCOV_PROFILE_URING | bool | 在 io_uring 子系统上启用 GCOV 性能分析，以便进行代码覆盖率测试。若不确定，选 N。注意这将对 io_uring 子系统性能产生负面影响…… |
| GDB_SCRIPTS | bool | 这会在构建目录中创建到 GDB 辅助脚本所需的链接。若你将 vmlinux 加载到 gdb 中，这些辅助脚本也会被 gdb 自动导入，并提供额外的函数…… |
| GENERIC_EARLY_IOREMAP | bool | 这是 32 位用户进程栈向上增长时（目前仅在 parisc 体系结构上）其 VM 布局中栈的最大大小（以兆字节计），当 RLIMIT_STACK 硬限制为无限制时。若…… |
| GENERIC_IOREMAP | bool |  |
| GLOB_KUNIT_TEST | tristate | 启用此选项以在运行时测试 glob 函数。该测试套件在各种场景（包括边界情况）下验证 glob_match() 的正确性。若不确定，选 N。 |
| GRACE_PERIOD | tristate | 某些 NFS 服务器支持一个辅助性的 NFS LOCALIO 协议，它并非 NFS 协议的官方组成部分。此选项在内核的 NFS 服务器与客户端中启用对 LOCALIO 协议的支持…… |
| GROUP_SCHED_WEIGHT | def_bool | 此选项允许用户在公平组调度器中为运行的任务定义 CPU 带宽速率（限制）。未设置限制的组被视为无约束，将以无……的方式运行 |
| GUEST_PERF_EVENTS | bool | 详见 tools/perf/design.txt |
| GUP_GET_PXX_LOW_HIGH | bool | 提供一个测试模块，分配并释放许多不同大小的块并报告耗时。旨在提供一种一致的方式来度量对 dma_pool_all……的更改所产生的影响 |
| GUP_TEST | bool | 提供 /sys/kernel/debug/gup_test，进而提供一种发起 ioctl 调用的方法，这些调用可启动针对 get_user_pages*() 与 pin_user_pages*() 系列 API 调用的基于内核的单元测试。该…… |
| HARDLOCKUP_DETECTOR_COUNTS_HRTIMER | bool | 在此选 Y 以让内核在“hard lockups（硬锁死）”时 panic，硬锁死是指导致内核在中断被禁用的情况下于内核模式循环超过 10 秒（可通过 wat……配置）的缺陷 |
| HARDLOCKUP_DETECTOR_PERF | bool | 将使用特定于体系结构的硬锁死检测器实现。# # “perf”与“buddy”两种硬锁死检测器都会计数 hrtimer 中断。此配置启用管理这些……的函数 |
| HARDLOCKUP_DETECTOR_PREFER_BUDDY | bool | 在此选 Y 以优先使用 buddy 硬锁死检测器而非 perf 版。使用 buddy 检测器时，每个 CPU 利用自身的 softlockup hrtimer 来检查下一个 CPU 是否通过 ve……在处理 hrtimer 中断 |
| HASHTABLE_KUNIT_TEST | tristate | 构建 hashtable 的 KUnit 测试套件。它测试 include/linux/hashtable.h 中定义的 API 的基本功能。有关 KUnit 与单元测试的更多信息，请参阅…… |
| HASH_KUNIT_TEST | tristate | 启用此选项以在引导时测试内核的字符串（<linux/stringhash.h>）与整数（<linux/hash.h>）哈希函数。KUnit 测试在引导期间运行，并以 TA……格式将结果输出到调试日志 |
| HAS_SECURITY_AUDIT | def_bool | 这将构建 securityfs 文件系统。它目前被各种安全模块（AppArmor、IMA、SafeSetID、TOMOYO、TPM）使用。若你不确定如何回答，选 N。 |
| HAVE_ARCH_AUDITSYSCALL | bool | 这是基于 tick 的基本 cputime 记账，维护关于用户、系统与空闲时间（以每 jiffies 粒度）的统计数据。若不确定，选 Y。 |
| HAVE_ARCH_TLB_REMOVE_TABLE | def_bool | 尝试在 munmap 与 exit_mmap 之外的路径中回收空的用户页表页。注意：目前仅回收空的用户 PTE 页表页。 |
| HAVE_ARCH_USERFAULTFD_MINOR | bool | 体系结构具有 userfaultfd 次要缺页支持 |
| HAVE_ARCH_USERFAULTFD_WP | bool | 体系结构具有 userfaultfd 写保护支持 |
| HAVE_DEBUG_BUGVERBOSE | bool | 启用此项以开启对链表遍历例程的扩展检查。此选项以性能换取更高质量的错误报告，更适合内核调试。若你关心…… |
| HAVE_DEBUG_STACKOVERFLOW | bool | 若你希望检查内核、IRQ 与异常栈（若你的体系结构使用它们）的溢出，在此选 Y。若空闲栈空间降到某一下限以下，此选项将显示详细消息…… |
| HAVE_HARDLOCKUP_DETECTOR_BUDDY | bool | 在此选 Y 以让内核充当看门狗检测硬锁死。硬锁死是指导致 CPU 在内核模式循环超过 10 秒、且不让其他中断……的缺陷 |
| HAVE_KERNEL_GZIP | bool | Linux 内核是一种自解压可执行文件。有多种压缩算法可用，它们在效率、压缩与解压速度上各不相同。压缩速度仅在……时才相关 |
| HAVE_LD_DEAD_CODE_DATA_ELIMINATION | bool | 这要求体系结构标注或以其他方式保护其外部入口点不被丢弃。链接脚本还必须将 .text.*、.data.* 与 .bss.* 正确合并到输出段…… |
| HAVE_PCSPKR_PLATFORM | bool | 此选项允许禁用或调整某些基础内核选项与设置。这适用于能够容忍“非标准”内核的专门环境。仅当你确实……时才使用 |
| HAVE_PERF_EVENTS | bool | 详见 tools/perf/design.txt。 |
| HAVE_SCHED_AVG_IRQ | def_bool | 选择此选项以在调度器中启用 HW 压力记账。HW 压力是传递给调度器的一个值，反映了由 HW 节流……导致的 CPU 计算能力降低 |
| HAVE_UNSTABLE_SCHED_CLOCK | bool | 该特性让调度器基于当前在该 CPU 上可调度的 RUNNABLE 任务跟踪每个 CPU 的钳位利用率。通过此选项，用户可指定每个 CPU 的最小与最大利用率…… |
| HEADERS_INSTALL | bool | 此选项将把 uapi 头文件（导出到用户空间的头文件）安装到 usr/include 目录，供内核构建期间使用。构建内核本身并不需要它，但……需要 |
| HMM_MIRROR | bool | 允许创建 struct page 来表示不可寻址的设备内存，即只能从设备（或设备组）访问的内存。你可能还希望选中 HMM_MIRROR。 |
| HUGETLB_PAGE | def_bool | 在此选 Y 以查看各种杂项文件系统的选项，例如来自其他操作系统的文件系统。此选项本身不添加任何内核代码。若你选 N，所有…… |
| HUGETLB_PAGE_OPTIMIZE_VMEMMAP_DEFAULT_ON | bool | HugeTLB Vmemmap 优化（HVO）默认关闭。在此选 Y 以默认启用 HVO。它可通过 hugetlb_free_vmemmap=off（引导命令行）或 hugetlb_optimize_vmemmap（sysctl）禁用。 |
| HWPOISON_INJECT | tristate | NOMMU 的 mmap() 经常需要分配大块连续内存来存储映射，但它只能向系统分配器请求 2^N*PAGE_SIZE 大小的块——这…… |
| HW_BREAKPOINT_KUNIT_TEST | bool | hw_breakpoint 约束记账的测试。若不确定，选 N。 |
| HYPERV_TESTING | bool | 选择此选项以启用 Hyper-V vmbus 测试。 |
| IDLE_PAGE_TRACKING | bool | 该特性允许估算在给定时间段内未被访问的用户页数量。该信息可用于调优内存 cgroup 限制和/或用于作业放置…… |
| IKCONFIG | tristate | 此选项将完整的 Linux 内核“.config”文件内容保存到内核中。它记录了运行中的内核或磁盘上的内核……使用了哪些内核选项 |
| IKCONFIG_PROC | bool | 此选项通过 /proc/config.gz 启用对内核配置文件的访问。 |
| IKHEADERS | tristate | 此选项启用对构建过程中生成的、内核内头文件的访问。这些头文件可用于构建 eBPF 跟踪程序或类似程序。若你将头文件构建为…… |
| INDIRECT_IOMEM | bool | 此选项由其他选项/体系结构选中，以提供模拟的 iomem 访问器。 |
| INDIRECT_IOMEM_FALLBACK | bool | 若选中 INDIRECT_IOMEM，此选项在 IO 内存地址不是已注册的模拟区域时，启用回退到普通 mmio 访问。 |
| INET | bool | 这些是互联网与大多数本地以太网上使用的协议。强烈建议在此选 Y（这将使内核增大约 400 KB），因为某些程序（如 X 窗口…… |
| INITRAMFS_PRESERVE_MTIME | bool | initramfs cpio 归档中的每个条目都带有 mtime 值。启用后，解出的 cpio 项采用该 mtime，目录 mtime 设置延迟到其任何子条目创建之后…… |
| INITRAMFS_TEST | bool | 构建 initramfs 的 KUnit 测试。请参阅 Documentation/dev-tools/kunit |
| INTEL_TXT | bool | 此选项启用配合 Trusted Boot（tboot）模块引导内核的支持。它将利用 Intel(R) 可信执行技术对内核执行可度量的启动。若…… |
| INTERVAL_TREE_SPAN_ITER | bool | 支持在 XArray 中占据多个连续索引的条目。 |
| INTERVAL_TREE_TEST | tristate | 一个测量区间树库性能的基准测试 |
| INT_LOG_KUNIT_TEST | tristate | 此选项启用针对 int_log 库的 KUnit 测试套件，该库提供两个函数以分别计算以 2 和 10 为底的整数对数，分别称为 intlog2 与 intlog10。该…… |
| INT_POW_KUNIT_TEST | tristate | 此选项启用针对 int_pow 函数（执行整数幂运算）的 KUnit 测试套件。该测试套件旨在验证 int_pow 的实现能正确计算…… |
| INT_SQRT_KUNIT_TEST | tristate | 此选项启用针对 int_sqrt() 函数（执行平方根计算）的 KUnit 测试套件。该测试套件检查各种场景（包括边界情况）以确保正确性。若…… |
| IO_STRICT_DEVMEM | bool | 若禁用此选项，你便允许用户空间（root）访问所有 io-memory，无论是否有驱动正在使用该范围。意外访问显然是灾难性的，但…… |
| IO_URING | bool | 此选项启用对 io_uring 接口的支持，使应用能够通过内核与应用之间共享的提交环与完成环来提交并完成 IO。 |
| IO_URING_MOCK_FILE | tristate | 为 io_uring 子系统测试启用模拟文件。ABI 仍可能变化，因此它仍是实验性的，只应为特定的测试目的启用。若不确定，选 N。 |
| IO_URING_ZCRX | def_bool |  |
| IRQ_TIME_ACCOUNTING | bool | 选择此选项以启用细粒度任务 irq 时间记账。这通过读取 softirq 与 hardirq 状态每次转换时的时间戳来实现，因此可能带来少量性能…… |
| IS_SIGNED_TYPE_KUNIT_TEST | tristate | 构建针对 is_signed_type() 宏的单元测试。有关 KUnit 与单元测试的更多信息，请参阅 Documentation/dev-tools/kunit/ 中的 KUnit 文档。若不确定，选 N。 |
| KALLSYMS | bool | 在此选 Y 以让内核打印符号化的崩溃信息与符号化栈回溯。这会一定程度上增大内核体积，因为所有符号都必须加载进内核镜像。 |
| KALLSYMS_ALL | bool | 通常 kallsyms 只包含函数的符号以提供更好的 OOPS 消息与回溯（即来自 text 与 inittext 段的符号）。这对大多数情况已足够。仅在…… |
| KALLSYMS_SELFTEST | bool | 测试某些接口（如 kallsyms_lookup_name）的基本功能与性能。它还会计算当前符号集下 kallsyms 压缩算法的压缩率…… |
| KCMP | bool | 启用内核资源比较系统调用。它为用户空间提供比较两个进程是否共享公共资源（如文件描述符乃至虚拟……）的能力 |
| KCOV | bool | KCOV 以一种适合覆盖率引导模糊测试（随机化测试）的形式暴露内核代码覆盖率信息。更多细节请参阅 Documentation/dev-tools/kcov.rst。 |
| KCOV_ENABLE_COMPARISONS | bool | KCOV 还会暴露插桩代码中每次比较的操作数，以及操作数大小与比较指令的 PC。这些操作数可被模糊测试引擎用来改进…… |
| KCOV_INSTRUMENT_ALL | bool | 若你在进行通用的系统调用模糊测试（如 syzkaller），你会希望对整个内核插桩，并应在此选 y。若你进行更有针对性的模糊测试（如…… |
| KCOV_IRQ_AREA_SIZE | hex | KCOV 使用预分配的每 CPU 区域来从软中断收集覆盖率。这指定了这些区域的大小（以 unsigned long 字数计）。 |
| KCOV_SELFTEST | bool | 在引导时运行简短的 KCOV 覆盖率收集自检测试。测试失败时会导致内核 panic。建议启用，以确保关键功能按预期工作。 |
| KERNEL_BZIP2 | bool | 其压缩率与速度居中。解压速度在各项选择中最慢。与 gzip 相比，使用 bzip2 内核体积约减小 10%。bzip2 会占用大量…… |
| KERNEL_GZIP | bool | 久经考验的 gzip 压缩。它在压缩率与解压速度之间提供了良好的平衡。 |
| KERNEL_LZ4 | bool | LZ4 是一种固定、面向字节编码的 LZ77 类压缩器。LZ4 解/压缩工具的初步版本可在 <https://code.google.com/p/lz4/> 获取。其压缩率较…… |
| KERNEL_LZMA | bool | 该压缩算法的压缩率最佳。解压速度介于 gzip 与 bzip2 之间。压缩最慢。与 gzip 相比，使用 LZMA 内核体积约减小 33%。 |
| KERNEL_LZO | bool | 其压缩率在各项选择中最差。内核体积比 gzip 大约 10%；但其速度（压缩与解压）最快。 |
| KERNEL_UNCOMPRESSED | bool | 生成未压缩的内核镜像。这通常不是你想要的。它可用于在慢速仿真环境中调试内核，因为在这些环境中解压与移动内核…… |
| KERNEL_XZ | bool | XZ 使用 LZMA2 算法与特定于指令集的 BCJ 过滤器，可改善可执行代码的压缩率。与 gzip 相比，使用 XZ 内核体积约减小 30%…… |
| KERNEL_ZSTD | bool | ZSTD 是一种面向中等压缩率、解压速度快速的压缩算法。它的压缩效果优于 GZIP，解压速度与 LZO 相近，但慢于 LZ4…… |
| KFIFO_KUNIT_TEST | tristate | 构建通用 FIFO 实现的 KUnit 测试套件。它测试 kfifo 类型及相关宏的 API 与基本功能。有关 KUnit 与单元测试的更多信息，请参阅…… |
| KPROBES_SANITY_TEST | tristate | 此选项提供在引导时测试基本 kprobes 功能的能力。会插入 kprobe 与 kretprobe 样例并验证其功能。若不确定，选 N。 |
| LATENCYTOP | bool | 若你希望使用 LatencyTOP 工具来查明哪些用户空间因哪些内核操作而阻塞，启用此选项。 |
| LAZY_MMU_MODE_KUNIT_TEST | tristate | 启用此选项以检查 lazy MMU 模式接口是否按预期工作。仅包含对通用接口的测试（不含体系结构相关行为）。若不确定，选 N。 |
| LD_DEAD_CODE_DATA_ELIMINATION | bool | 若你希望通过链接器进行死代码与数据消除（以 -ffunction-sections -fdata-sections 编译，并以 --gc-sections 链接），启用此项。这可以减少磁盘与内存中…… |
| LD_ORPHAN_WARN | def_bool | 启用对 /proc/sys/debug/exception-trace 的支持。 |
| LIBFDT | bool | 启用快速查找对象标识符注册表。 |
| LINEAR_RANGES | tristate | 此选项提供 packing() 辅助函数，它允许在 CPU 可用表示与可能具有这些任意组合特性的内存表示之间转换位域…… |
| LINEAR_RANGES_TEST | tristate | 构建 linear_ranges 单元测试，在引导时运行。测试 linear_ranges 逻辑的正确性。有关 KUnit 与单元测试的更多信息，请参阅 KUnit 文档…… |
| LIST_KUNIT_TEST | tristate | 构建链表 KUnit 测试套件。它测试 list_head 类型及相关宏的 API 与基本功能。KUnit 测试在引导期间运行，并将结果输出到调试日志…… |
| LIST_PRIVATE_KUNIT_TEST | tristate | 构建针对 include/linux/list_private.h 中定义的私有链表原语的 KUnit 测试。这些原语允许操作被标记为私有并重新……的 list_head 成员 |
| LIVEUPDATE_TEST | bool | 为 Live Update Orchestrator 启用一个内置内核测试模块。该模块通过注册一组模拟 FLB 对象（与任何真实文件处理程序……）来验证 File-Lifecycle-Bound 子系统 |
| LKDTM | tristate | 该模块通过在预定义崩溃点引发系统故障，来测试不同的转储机制。若你不需要它：选 N；在此选 M 将本代码编译为模块。该…… |
| LOCALVERSION | string | 在你的内核版本末尾追加一个额外字符串。例如，它会在你输入 uname 时显示。你在此设置的字符串将被追加到任何文件名为……的文件内容之后 |
| LOCALVERSION_AUTO | bool | 这将尝试通过查找属于当前树顶修订版的 git 标签，自动判断当前树是否为发布树。格式为 -gxxxxxxxx 的字符串将被添加…… |
| LOCKDEP | bool | 若遇到“BUG: MAX_LOCKDEP_ENTRIES too low!”消息，尝试增大此值。 |
| LOCKDEP_CHAINS_BITS | int | 若遇到“BUG: MAX_LOCKDEP_CHAINS too low!”消息，尝试增大此值。 |
| LOCKDEP_CIRCULAR_QUEUE_BITS | int | 若因 __cq_enqueue() 失败而遇到“lockdep bfs error:-1”警告，尝试增大此值。 |
| LOCKDEP_STACK_TRACE_BITS | int | 若遇到“BUG: MAX_STACK_TRACE_ENTRIES too low!”消息，尝试增大此值。KASAN 会显著增加栈跟踪消耗，因为其 slab 跟踪与 lockdep 的依赖…… |
| LOCKDEP_STACK_TRACE_HASH_BITS | int | 若你需要较大的 STACK_TRACE_HASH_SIZE，尝试增大此值。 |
| LOCKUP_DETECTOR | bool | 在此选 Y 以让内核充当看门狗检测软锁死。软锁死是指导致内核在内核模式循环超过 20 秒、且不给其他任务……的缺陷 |
| LOCK_DEBUGGING_SUPPORT | bool | 该特性让内核能够证明内核运行时发生的所有加锁在 mathematically 上是正确的：即任何情况下都不可能出现任意（尚未触发的）组合…… |
| LOCK_MM_AND_FIND_VMA | bool | 启用 NUMA 模拟。使用“numa=fake=N”引导时（N 为节点数），平坦机器将被拆分为虚拟节点。这仅对调试有用。 |
| LOCK_STAT | bool | 该特性启用对锁竞争点的跟踪。更多细节请参阅 Documentation/locking/lockstat.rst。它还启用“perf lock”（perf 的子命令）所需的锁事件。若你希望…… |
| LOCK_TORTURE_TEST | tristate | 此选项提供一个内核模块，对内核锁原语运行 torture 测试。若需要，该模块可在被测的运行中内核上事后构建。在此选 Y…… |
| LOG_BUF_SHIFT | int | 以 2 的幂选择最小内核日志缓冲区大小。最终大小受 LOG_CPU_MAX_BUF_SHIFT 配置参数影响（见下）。任何更大的大小也可能被“log_buf_len”引导…… |
| LOG_CPU_MAX_BUF_SHIFT | int | 此选项允许根据 CPU 数量增大默认环形缓冲区大小。该值定义了每个 CPU 以 2 的幂计的贡献。已用空间通常只有几行…… |
| LONGEST_SYM_KUNIT_TEST | tristate | 测试可能的最长符号。若不确定，选 N。 |
| LRU_GEN | bool | 一种用于内存超配的高性能 LRU 实现。详见 Documentation/admin-guide/mm/multigen_lru.rst。 |
| LRU_GEN_ENABLED | bool | 此选项默认启用多代 LRU。 |
| LRU_GEN_STATS | bool | 除非你打算为了调试而查看被驱逐代的历史统计，否则不要启用此选项。此选项有 per-memcg 与 per-node 的内存开销。 |
| LRU_GEN_WALKS_MMU | def_bool | 允许在缺页处理期间进行 per-vma 加锁。该特性在处理缺页时允许分别锁定每个虚拟内存区域，而非获取 mmap_lock。 |
| LSM_MMAP_MIN_ADDR | int | 这是应当被保护以免受用户空间分配的低虚拟内存部分。阻止用户写入低地址页有助于降低内核空指针缺陷的影响。有关…… |
| LWTUNNEL_BPF | bool | 允许在路由查找之后，将 BPF 程序作为入向与出向数据包的下一跳动作运行。 |
| LZO_COMPRESS | tristate | 驱动可选择此选项，为参数 'm'（伽罗瓦域阶）与 't'（纠错能力）强制特定常数值。这些特定值必须通过声明默认……来设置 |
| MAGIC_SYSRQ | bool | 若在此选 Y，即使系统在例如内核调试期间崩溃，你仍对系统有一定控制权（例如，你能够将缓冲区缓存刷写到磁盘、重启系统……） |
| MAGIC_SYSRQ_DEFAULT_ENABLE | hex | 指定默认启用哪些 SysRq 键功能。可设为 1 或 0 以全部启用或禁用，或设为 Documentation/admin-guide/sysrq.rst 中描述的位掩码。 |
| MAGIC_SYSRQ_SERIAL | bool | 许多嵌入式板卡具有未连接的 TTL 电平串口，可能产生导致误报 sysrq 检测的垃圾数据。此选项允许你决定是否……启用 |
| MAGIC_SYSRQ_SERIAL_SEQUENCE | string | 指定可跟在 BREAK 之后以在串口控制台上启用 SysRq 的字符序列。若不确定，留空字符串，该选项将不被启用。 |
| MAX_SKB_FRAGS | int | 每个 skb_shared_info 拥有更多分片有助于 GRO 效率。这有助于 BIG TCP 工作负载，但可能暴露某些遗留驱动中的缺陷。这也会增加小数据包的内存开销…… |
| MEMBARRIER | bool | 启用 membarrier() 系统调用，它允许在所有运行中的线程间发布内存屏障，可用于通过变换……非对称地分摊用户空间内存屏障的代价 |
| MEMCG | bool | 提供对 cgroup 中任务内存占用的控制。 |
| MEMCG_NMI_UNSAFE | bool | 已被 cgroup v2 实现废弃的传统 cgroup v1 内存控制器。v1 保留给尚未迁移到新 cgroup v2 接口的遗留应用。若你…… |
| MEMCPY_KUNIT_TEST | tristate | 构建针对 memcpy()、memmove() 与 memset() 函数的单元测试。有关 KUnit 与单元测试的更多信息，请参阅 Documentation/dev-tools/kunit/ 中的 KUnit 文档…… |
| MEMORY_HOTREMOVE | bool | 允许迁移在内存气球中膨胀的页，使它们能从仅可用于可移动分配（如 ZONE_MOVABLE、CMA）的内存区域分配，并且可以…… |
| MEMORY_NOTIFIER_ERROR_INJECT | tristate | 此选项提供向内存热插拔通知链回调注入人为错误的能力。它通过 /sys/kernel/debug/notifier-error-inject/me……下的 debugfs 接口控制 |
| MEMTEST | bool | 此选项添加内核参数 'memtest'，允许设置并执行 memtest。memtest=0 表示禁用；默认 memtest=1 表示执行 1 种测试模式；…… memtest=17 表示执行 17 种测试模式…… |
| MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT | bool | 为内存分配性能分析添加带有有用错误信息的警告。 |
| MEM_SOFT_DIRTY | bool | 此选项通过在 PTE 上引入软脏位来启用内存变更跟踪。当有人写入某页时设置该位，如同普通脏位，但它与普通脏位不同，可被清除…… |
| MESSAGE_LOGLEVEL_DEFAULT | int | 未指定优先级的 printk 语句的默认日志级别。自至少 2.6.10 起它被硬编码为 KERN_WARNING，但密切审计日志的人可能希望将其设为…… |
| MHP_DEFAULT_ONLINE_TYPE_OFFLINE | bool | 热插拔内存默认不会上线。对于由驱动与用户策略处理热插拔内存上线的系统，选择此项。 |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_AUTO | bool | 若你希望内核自动将热插拔内存上线到它认为合理的 zone，选择此项。该内存可能被用于内核数据。 |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_KERNEL | bool | 若你希望内核自动将热插拔内存上线到可用于内核数据的 zone，选择此项。这通常指 ZONE_NORMAL。 |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_MOVABLE | bool | 若你希望内核自动将热插拔内存上线到 ZONE_MOVABLE，选择此项。该内存通常不被用于内核数据。仅当管理员知道……时才应使用。 |
| MIGRATION | bool | 当平台上存在多种 HugeTLB 页大小时，允许 pageblock_order 值为动态而非仅为标准 HUGETLB_PAGE_ORDER。注意 pageblock_order 无法…… |
| MIN_HEAP_KUNIT_TEST | tristate | 此选项启用针对 min heap 库（提供创建与管理最小堆的函数）的 KUnit 测试套件。该测试套件检查最小堆库的功能。若…… |
| MMAP_ALLOW_UNINITIALIZED | bool | 通常，根据 Linux 规范，从 mmap() 获得的匿名内存在传递给用户空间之前其内容会被清零。启用此配置选项允许你请求…… |
| MM_ID | def_bool | 透明大页允许内核在可能时透明地对应用使用大页与 huge tlb。该特性可通过……提升某些应用的计算性能 |
| MODULE_ALLOW_BTF_MISMATCH | bool | 对于拆分 BTF 与 vmlinux 不匹配的模块，不使用 BTF 加载而不拒绝加载。启用模块 BTF 时的默认行为是拒绝此类不匹配的模块；此选项…… |
| MPILIB | tristate | 来自 GnuPG 的多精度数学库。它用于实现 RSA 数字签名验证，IMA/EVM 数字签名扩展会用到它。 |
| MSEAL_SYSTEM_MAPPINGS | bool | 对系统映射应用 mseal。系统映射包括 vdso、vvar、vvar_vclock、vectors（arm 兼容模式）、sigpage（arm 兼容模式）、uprobes。内存密封特性需要 64 位内核…… |
| MULTIUSER | bool | 此选项启用对普通用户、组与能力的支持。若在此选 N，所有进程将以 UID 0、GID 0 及所有可能的能力运行。在此选 N 还会编译掉…… |
| NET | bool | 除非你确实知道自己在做什么，否则应在此选 Y。原因是某些程序即使在未连接……的独立机器上运行，也需要内核网络支持 |
| NETDEV_ADDR_LIST_TEST | tristate | 覆盖核心网络基础设施（如 sk_buff）的 KUnit 测试。若不确定，选 N。 |
| NETDEV_NOTIFIER_ERROR_INJECT | tristate | 此选项提供向 netdevice 通知链回调注入人为错误的能力。它通过 /sys/kernel/debug/notifier-error-inject/netdev 下的 debugfs 接口控制。若…… |
| NETFILTER | bool | Netfilter 是一个用于过滤与篡改流经 Linux 主机的网络数据包的框架。数据包过滤最常见的用途是将你的 Linux 主机作为防火墙保护本地…… |
| NETFILTER_ADVANCED | bool | 若在此选 Y，你可以在所有 netfilter 模块之间选择。若选 N，较不常见的模块将不显示，而大多数人需要的基本模块将默认为 'M'。若不确定，选 Y。 |
| NETWORK_FILESYSTEMS | bool | 在此选 Y 以查看网络文件系统与文件系统相关网络代码（如 NFS 守护进程与 RPCSEC 安全模块）的选项。此选项本身不添加任何内核代码。若…… |
| NETWORK_SECMARK | bool | 这启用对网络数据包的安全标记，类似于 nfmark，但专用于安全目的。若你不确定如何回答，选 N。 |
| NET_DEVLINK | bool | 启用页池统计，以跟踪页池中的页分配与回收。此选项在分配与回收路径上带来额外的 CPU 开销，以及存储统计……的额外内存开销 |
| NET_DROP_MONITOR | tristate | 该特性在网络栈中丢弃数据包时，向用户空间提供告警服务。告警通过 netlink 套接字广播给任何监听的用户空间进程。该…… |
| NET_FLOW_LIMIT | bool | 当接收处理 CPU 的 backlog 达到 netdev_max_backlog 时，网络栈必须丢弃数据包。若众多活跃流中仅有少数产生了绝大多数负载，丢弃它们的流量…… |
| NET_INGRESS | bool | 这构建针对 handshake upcall 机制的 KUnit 测试。KUnit 测试在引导期间运行，并以 TAP 格式（https://testanything.org/）将结果输出到调试日志。仅对内核……有用 |
| NET_NS | bool | 允许用户空间创建看似多个网络栈实例的对象。 |
| NET_PKTGEN | tristate | 该模块将以可配置速率从给定接口注入预配置的数据包。它用于网络接口压力测试与性能分析。如果你不理解…… |
| NET_PTP_CLASSIFY | def_bool | 这允许具有硬件时间戳能力的 PHY（或其他 MII 总线嗅探设备）对网络数据包进行时间戳标记。此选项在发送与接收路径上增加一些开销。若…… |
| NET_RX_BUSY_POLL | bool | 启用此项允许将 TCP 流解析器与 BPF_MAP_TYPE_SOCKMAP 一起使用。 |
| NFS_V4_2_SSC_HELPER | bool |  |
| NLATTR | bool | 用于通过轮询进行中断缓解的辅助库。 |
| NOINSTR_VALIDATION | bool | 选择此选项将在链接 vmlinux 时向 ld 传递 "-Map=vmlinux.map"。该文件可用于验证与调试神奇的段操作，以及查看哪些代码段被消除…… |
| NOTIFIER_ERROR_INJECTION | tristate | 此选项提供向指定通知链回调注入人为错误的能力。它有助于测试通知链失败的错误处理。若不确定，选 N。 |
| NO_PAGE_MAPCOUNT | bool | 不为属于较大分配（如透明大页）的页维护每页 mapcount。启用此配置选项后，一些依赖此信息的接口将…… |
| NUMA_BALANCING_DEFAULT_ENABLED | bool | 若设置，在 NUMA 机器上运行时将启用自动 NUMA 平衡。 |
| NUMA_MIGRATION | bool | 支持将页迁移到其他 NUMA 节点，用户空间可通过 migrate_pages()、move_pages() 与 mbind() 等接口使用。选择此选项还启用对页……的支持 |
| OBJTOOL | bool | 在遇到 objtool 警告时使构建失败。objtool 警告可能指示内核不稳定，包括引导失败。强烈建议此选项。若不确定，选 Y。 |
| OF_RECONFIG_NOTIFIER_ERROR_INJECT | tristate | 此选项提供向 OF 重配置通知链回调注入人为错误的能力。它通过 /sys/kernel/debug/notifier-error-inject/OF-re……下的 debugfs 接口控制 |
| OVERFLOW_KUNIT_TEST | tristate | 构建针对 check_*_overflow()、size_*()、分配及相关函数的单元测试。有关 KUnit 与单元测试的更多信息，请参阅 KUnit 文档…… |
| PACKING_KUNIT_TEST | tristate | 构建针对 packing 库的 KUnit 测试。有关 KUnit 与单元测试的更多信息，请参阅 Documentation/dev-tools/kunit/ 中的 KUnit 文档。若有疑问，选…… |
| PAGE_COUNTER | bool | 此选项默认启用 "favordynmods" 挂载选项，它以降低热路径……为代价，减少了任务迁移与控制器开关等动态 cgroup 修改的延迟 |
| PAGE_IDLE_FLAG | bool | 这向 'struct page' 添加 PG_idle 与 PG_young 标志。PTE Accessed 位的写入者可以设置标志中位的状态，使 PTE Accessed 位的读取者可以避免干扰。 |
| PAGE_MAPCOUNT | def_bool | 这启用连续内存分配器（Contiguous Memory Allocator），允许其他子系统分配大的物理连续内存块。CMA 保留一块内存区域，并只允许可移动页…… |
| PAHOLE_HAS_BTF_TAG | def_bool | 决定 pahole 是否发出 btf_tag 属性（btf_type_tag 与 btf_decl_tag）。目前只有 clang 编译器实现了这些属性，因此使该配置依赖于 CC_IS_CLANG。 |
| PAHOLE_HAS_LANG_EXCLUDE | def_bool | 支持 --lang_exclude 标志，使 pahole 排除来自所提供语言的编译单元。在 Kbuild 中用于省略 pahole 1.24 版本不支持的 Rust CU…… |
| PANIC_ON_OOPS | bool | 在此选 Y 以让内核在 oops 时 panic。这与在内核命令行设置 oops=panic 效果相同。该特性有助于确保内核不执行任何…… |
| PANIC_TIMEOUT | int | 设置内核 panic 后到发生重启的超时值（秒）。若 n = 0，则永远等待。n > 0 的超时值将等待 n 秒后重启，而 n…… |
| PC104 | bool | 暴露可供选择与配置的 PC/104 规格设备驱动程序与选项。若你的目标机器具有 PC/104 总线，启用此选项。 |
| PCPU_DEV_REFCNT | bool | 若设置此选项，网络设备引用计数将使用 per cpu 变量。可强制为 N 以检测下溢（伴随性能下降）。 |
| PCSPKR_PLATFORM | bool | 此选项允许禁用内置 PC 扬声器支持，节省一些内存。 |
| PERCPU_STATS | bool | 该特性通过 debugfs 收集并暴露统计信息。这些信息包括全局与每块统计，可用于帮助理解 percpu 内存使用。 |
| PERCPU_TEST | tristate | 启用此选项以构建验证 per-cpu 操作的测试模块。若不确定，选 N。 |
| PERF_EVENTS | bool | 启用内核对各种由软件与硬件提供的性能事件的支持。软件事件以内置方式或通过通用跟踪点支持。大多数现代 CPU 支持…… |
| PHYS_ADDR_T_64BIT | def_bool | 启用内核同页合并（Kernel Samepage Merging）：KSM 周期性扫描应用地址空间中应用建议可能可合并的区域。当它找到内容相同的页时，会将其替换…… |
| PID_NS | bool | 支持进程 ID 命名空间。只要进程处于不同的 pid 命名空间中，就允许存在多个具有相同 pid 的进程。这是容器的构建模块。 |
| PM_NOTIFIER_ERROR_INJECT | tristate | 此选项提供向 PM 通知链回调注入人为错误的能力。它通过 /sys/kernel/debug/notifier-error-inject/pm 下的 debugfs 接口控制。若通知链…… |
| POSIX_MQUEUE_SYSCTL | bool | 这是一个通用通知队列，供内核通过将事件拼接进管道来传递给用户空间。它可与用于密钥/密钥环变更通知的监视……配合使用 |
| POSIX_TIMERS | bool | 这为内核包含对 POSIX 定时器的原生支持。某些嵌入式系统用不到它们，因此可以将其配置掉以减小内核镜像体积。当此选项…… |
| PREEMPT_NOTIFIERS | bool | 构建一个简单的 ASN.1 语法编译器，生成可由 ASN.1 流解码器解释的字节码输出，并用于告知它在流中预期出现哪些标签以及…… |
| PRIME_NUMBERS_KUNIT_TEST | tristate | 此选项启用针对 {is,next}_prime_number 函数的 KUnit 测试套件。启用此选项将包含将这些素数生成函数与暴力实现……进行对比的测试 |
| PRINTK | bool | 此选项启用常规 printk 支持。移除它会从内核镜像中消除大部分消息字符串，使内核或多或少保持静默。由于这会使诊断……非常困难 |
| PRINTK_CALLER | bool | 选择此选项会使 printk() 为每条消息添加调用者“thread id”（若在任务上下文中）或调用者“processor id”（若不在任务上下文中）。此选项面向……的环境 |
| PRINTK_EXECUTION_CTX | bool | 此选项扩展 struct printk_info，以在 printk 中包含额外的执行上下文，例如消息来源的任务名与 CPU 编号。这有助于关联 printk 消息…… |
| PRINTK_INDEX | bool | 添加对编译时已知的所有 printk 格式在 <debugfs>/printk/index/<module> 处建立索引的支持。这可用于维护监控 /dev/kmsg 的守护进程，因为它允许审计…… |
| PRINTK_RINGBUFFER_KUNIT_TEST | tristate | 构建 printk 环形缓冲区 KUnit 测试套件。有关 KUnit 与单元测试的更多信息，请参阅 KUnit 文档。若不确定，选 N。 |
| PRINTK_TIME | bool | 选择此选项会使 printk() 消息的时间戳被添加到 syslog() 系统调用的输出以及控制台输出中。时间戳始终在内部记录，并导出…… |
| PROC_MEM_ALWAYS_FORCE | bool | 若你具有 ptrace 访问权限，这允许 /proc/pid/mem 访问覆盖内存映射权限。 |
| PROC_MEM_FORCE_PTRACE | bool | 这允许 /proc/pid/mem 访问为 gdb 等活跃的 ptracer 覆盖内存映射权限。 |
| PROC_MEM_NO_FORCE | bool | 永远不覆盖内存映射权限 |
| PROC_PID_CPUSET | bool | 提供一个 cgroup 控制器，为 cgroup 中的进程可以 mknod 或打开的设备实现白名单。 |
| PROFILING | bool | 在此选 Y 以启用性能分析器所使用的扩展性能分析支持机制。 |
| PROVE_RAW_LOCK_NESTING | bool | 启用 raw_spinlock 与 spinlock 嵌套检查，以确保不违反为 PREEMPT_RT 启用内核的锁嵌套规则。 |
| PROVIDE_OHCI1394_DMA_INIT | bool | 若你想调试在引导早期挂起或崩溃内核的问题，且崩溃的机器具有 FireWire 端口，你可以使用此特性远程访问崩溃机器的内存…… |
| PSI | bool | 收集指示系统中 CPU、内存与 IO 容量超配程度的指标。若在此选 Y，内核将创建 /proc/pressure/，其中包含压力统计文件 cpu、…… |
| PSI_DEFAULT_DISABLED | bool | 若设置，压力停顿信息跟踪默认禁用，但可通过在引导时于内核命令行传递 psi=1 来启用。该特性向任务唤醒……添加一些代码 |
| PTE_MARKER_UFFD_WP | bool | 允许为 userfaultfd 写保护目的创建标记 PTE。要在 shmem 与 hugetlbfs 等文件后备内存类型上启用 userfaultfd 写保护，需要它。 |
| RANDOM_KMALLOC_CACHES | bool | 一项加固特性，为普通 kmalloc 分配创建 slab 缓存的多个副本，并让 kmalloc 基于代码地址随机选取一个，使攻击者更难…… |
| RANDSTRUCT_KUNIT_TEST | tristate | 构建用于检查 CONFIG_RANDSTRUCT=y（随机化结构体布局）的单元测试。 |
| RATELIMIT_KUNIT_TEST | tristate | 构建“test_ratelimit”模块，用于速率限制的正确性验证与并发测试。若不确定，选 N。 |
| RATIONAL_KUNIT_TEST | tristate | 构建有理数数学单元测试。有关 KUnit 与单元测试的更多信息，请参阅 Documentation/dev-tools/kunit/ 中的 KUnit 文档。若不确定，选 N。 |
| RBTREE_TEST | tristate | 一个测量 rbtree 库性能的基准测试。也包括 rbtree 不变式检查。 |
| READABLE_ASM | bool | 禁用一些倾向于生成人类不可读汇编输出的编译器优化。这可能使内核稍慢，但有助于让需要大量……的内核开发者 |
| READ_ONLY_THP_FOR_FS | bool | 允许 khugepaged 将只读的文件后备页放入 THP。这被标记为实验性，因为它是一项新特性。文件 THP 的写入支持将在接下来的几个发布周期中开发。 |
| REED_SOLOMON_TEST | tristate | 此选项在引导时或模块加载时启用 rslib 的自检测试函数。若不确定，选 N。 |
| RELAY | bool | 此选项在某些文件系统（如 debugfs）中启用 relay 接口支持。它旨在为工具与设施提供一种高效机制，以传输大量…… |
| RESOURCE_KUNIT_TEST | tristate | 构建资源 API 单元测试。测试 resource.c 与 ioport.h 提供的 API 逻辑。有关 KUnit 与单元测试的更多信息，请参阅 KUnit 文档…… |
| RFS_ACCEL | bool | 允许具有流过滤表的 multiqueue 硬件的驱动加速 RFS。 |
| RPS | bool | 软件接收端数据包导向（RPS）将接收数据包处理的负载分布到多个 CPU 上。 |
| RSEQ | bool | 启用可重启序列（restartable sequences）系统调用。它提供一个用户空间缓存以存放当前 CPU 编号值，加速从用户空间获取当前 CPU 编号，并提供一个 ABI 以…… |
| RSEQ_DEBUG_DEFAULT_ENABLE | bool | 这启用可重启序列调试模式的静态分支。它也可通过内核命令行参数 "rseq_debug=0/1" 以及通过 debugfs 控制。若…… |
| RSEQ_SLICE_EXTENSION | bool | 允许用户空间通过 RSEQ 共享数据 ABI，在从中断返回用户空间时请求有限的时间片延长。若获准，即可完成一个临界区，从而…… |
| RSEQ_STATS | bool | 启用轻量级计数器，通过 debugfs 暴露关于 RSEQ 操作频率的信息。主要用于内核调试或性能分析。虽然轻量，它仍…… |
| RT_GROUP_SCHED | bool | 该特性让你可以显式地将真实 CPU 带宽分配给任务组。若启用，在为非 root 用户分配实时带宽……之前，将无法为非 root 用户调度实时任务 |
| RT_GROUP_SCHED_DEFAULT_DISABLED | bool | 当设置时，RT 组调度默认禁用。该选项采用反向形式，因此单纯的 RT_GROUP_SCHED 即启用组调度。若不确定，选 N。 |
| RUNTIME_TESTING_MENU | bool | 启用此项以包含 Dhrystone 2.1 基准测试。该测试计算每秒的 Dhrystones 数量，以及将 Dhrystone 分数除以……后得到的 DMIPS（Dhrystone MIPS）数量 |
| RUST | bool | 启用内核中的 Rust 支持。这允许选择其他 Rust 相关选项，如用 Rust 编写的驱动。要能够加载用 Rust 编写外部内核模块也需要它…… |
| RUSTC_LLVM_VERSION | int | 这指示 Rust 与 Clang 是否使用相同主版本的 LLVM。涉及处理 LLVM IR 或 bitcode（如跨语言 LTO）的操作需要相同主版本的 LLVM 才能正常工作…… |
| RUSTC_VERSION_TEXT | string | 参见 `CC_VERSION_TEXT`。 |
| RUST_BUILD_ASSERT_ALLOW | bool | 控制在构建期间如何处理 `build_error!` 与 `build_assert!`。如果二进制中存在对它们的调用，可能指示被违反的不变量或优化器未能验证该不变量…… |
| RUST_DEBUG_ASSERTIONS | bool | 启用 rustc 的 `-Cdebug-assertions` codegen 选项。该标志可让你开启或关闭 `cfg(debug_assertions)` 条件编译。这可用于在开发……中启用额外的调试代码 |
| RUST_INLINE_HELPERS | bool | 使用链接时优化（LTO）将 C 辅助函数内联到 Rust 代码中。若启用此选项，rust/helpers/ 中声明的 C 辅助函数将被内联到 Rust 代码中，这有助于性能…… |
| RUST_IS_AVAILABLE | def_bool | 这显示是否有合适的 Rust 工具链可用（已找到）。有关如何满足 Rust 支持的构建要求的说明，请参阅 Documentation/rust/quick-start.rst。特别…… |
| RUST_KERNEL_DOCTESTS | bool | 这将 `kernel` crate 的文档测试构建为 KUnit 测试。有关 KUnit 与单元测试的更多信息，请参阅 Documentation/dev-tools……中的 KUnit 文档 |
| RUST_OVERFLOW_CHECKS | bool | 启用 rustc 的 `-Coverflow-checks` codegen 选项。该标志允许你控制运行时整数溢出的行为。当启用溢出检查时，溢出将引发 Rust panic…… |
| SCANF_KUNIT_TEST | tristate | 启用此选项以在运行时测试 scanf 函数。若不确定，选 N。 |
| SCF_TORTURE_TEST | tristate | 此选项提供一个内核模块，对 smp_call_function() 系列原语运行 torture 测试。若需要，该模块可在被测的运行中内核上事后构建。若…… |
| SCHED_AUTOGROUP | bool | 此选项通过自动创建并填充任务组，为常见桌面工作负载优化调度器。这种工作负载的分离隔离了激进的 CPU 消耗者（如构建作业……） |
| SCHED_INFO | bool | 若在此选 Y，将在调度器及相关例程中插入额外代码，以收集调度器行为的统计信息并通过 /proc/schedstat 提供。这些统计可能…… |
| SCHED_PROXY_EXEC | bool | 此选项启用代理执行（proxy execution），一种让持有互斥体的任务继承更高优先级等待者调度上下文的机制。 |
| SCHED_STACK_END_CHECK | bool | 此选项检查对 schedule() 调用时的栈溢出。如果发现栈末尾位置被覆盖，则总是 panic，因为被破坏区域的内容已不再可信。该…… |
| SECTION_MISMATCH_WARN_ONLY | bool | 若在此选 N，构建过程将在出现任何段不匹配时失败，而非仅仅抛出警告。若不确定，选 Y。 |
| SECURITY | bool | 这允许你选择将不同的安全模块配置进内核。若未选择此选项，将使用默认 Linux 安全模型。若你不确定如何回答…… |
| SECURITY_COMMONCAP_KUNIT_TEST | bool | 构建 commoncap KUnit 测试。KUnit 测试在引导期间运行，并以 TAP 格式（https://testanything.org/）将结果输出到调试日志。仅对运行 KUnit 测试的内核开发者有用…… |
| SECURITY_DMESG_RESTRICT | bool | 这强制限制非特权用户通过 dmesg(8) 读取内核 syslog。若未选择此选项，除非 dmesg_restrict sysctl 被显式……否则不强制任何限制 |
| SECURITY_INFINIBAND | bool | 这启用 Infiniband 安全钩子。若启用，安全模块可使用这些钩子实现 Infiniband 访问控制。若你不确定如何回答，选 N。 |
| SECURITY_NETWORK | bool | 这启用套接字与网络安全的钩子。若启用，安全模块可使用这些钩子实现套接字与网络访问控制。若你不确定如何回答…… |
| SECURITY_NETWORK_XFRM | bool | 这启用 XFRM（IPSec）网络安全钩子。若启用，安全模块可使用这些钩子实现基于从 IPSec 策略派生的标签的每包访问控制。非 IP…… |
| SECURITY_PATH | bool | 这启用基于路径名的访问控制的安全钩子。若启用，安全模块可使用这些钩子实现基于路径名的访问控制。若你不确定如何回答…… |
| SELECT_MEMORY_MODEL | def_bool | 此选项允许你更改 Linux 在内部管理内存的某些方式。大多数用户只会由体系结构配置选中其中一个选项。这是正常的。 |
| SEQ_BUF_KUNIT_TEST | tristate | 构建针对 seq_buf 库的单元测试。若不确定，选 N。 |
| SGETMASK_SYSCALL | bool | sys_sgetmask 与 sys_ssetmask 是已废弃的系统调用，libc 不再支持，但在某些体系结构上默认仍启用。若不确定，保留此处的默认选项。 |
| SG_POOL | def_bool | 提供一个分配链式散列表的辅助函数。应由希望分配链式散列表的驱动或 API 选中。# # sg 链式选项 # |
| SHMEM | bool | shmem 是一个用于管理共享内存的内部文件系统。它由 swap 后备并管理资源限制。若启用 TMPFS，它也被作为 tmpfs 导出到用户空间。禁用此选项…… |
| SHRINKER_DEBUG | bool | 选 Y 以启用 shrinker 的 debugfs 接口，它提供对内核内存 shrinker 子系统的可见性。禁用它以避免额外的内存占用。 |
| SHUFFLE_PAGE_ALLOCATOR | bool | 页分配器的随机化改善了直接映射的内存侧缓存的平均利用率。参见 ACPI 6.2a 规范中 5.2.27 节异构内存属性表（HMAT）…… |
| SIGNALFD | bool | 启用 signalfd() 系统调用，它允许在文件描述符上接收信号。若不确定，选 Y。 |
| SIGNATURE | tristate | 数字签名验证。目前仅支持 RSA。实现使用 GnuPG MPI 库。 |
| SIPHASH_KUNIT_TEST | tristate | 启用此选项以在引导时（或模块加载时）测试内核的 siphash（<linux/siphash.h>）哈希函数。它旨在帮助编写特定于体系结构的优化版本的人。若…… |
| SLAB_BUCKETS | bool | 内核堆攻击常常依赖于能够创建具有用户可控内容、且会被分配到与目标对象相同 kmalloc bucket 的特定大小分配。为…… |
| SLAB_FREELIST_HARDENED | bool | 许多内核堆攻击试图针对 slab 缓存元数据与其他基础设施。此选项做出少量性能牺牲，以加固内核 slab 分配器抵御常见的 freelist 利用…… |
| SLAB_FREELIST_RANDOM | bool | 随机化创建新页时使用的 freelist 顺序。该安全特性降低了内核 slab 分配器对堆溢出的可预测性。 |
| SLAB_MERGE_DEFAULT | bool | 为减少内核内存碎片，当 slab 缓存具有相同大小与其他特征时可被合并。这带来内核堆溢出能够覆盖对象……的风险 |
| SLAB_OBJ_EXT | bool | 此选项添加将进程集合分组在一起的支持，供 Cpusets、CFS、内存控制或设备隔离等进程控制子系统使用。参见 Documentation/scheduler/sc…… |
| SLUB | def_bool | 以一种实现最小内存占用的方式配置 slab 分配器，牺牲可扩展性、调试与其他特性。这仅面向曾使用 SL……的最小系统 |
| SLUB_KUNIT_TEST | tristate | 构建 SLUB 分配器单元测试。测试 SLUB 缓存调试功能。有关 KUnit 与单元测试的更多信息，请参阅 Documentation/dev-……中的 KUnit 文档 |
| SLUB_STATS | bool | 这些统计有助于调试 slab 分配行为，以找到优化分配器的方法。绝不应在生产环境中启用，因为保存统计会拖慢整个…… |
| SOCK_CGROUP_DATA | bool | 提供让任务使用相同 id 与不同对象协作的方式。例如，相同的 IPC id 在不同……中可能指向不同对象，或相同的用户 id 或 pid 可能指向不同任务 |
| SOCK_RX_QUEUE_MAPPING | bool | 用于将进程按每个接口分配到网络优先级的 cgroup 子系统。 |
| SOFTLOCKUP_DETECTOR_INTR_STORM | bool | 在此选 Y 以让内核在“soft lockups（软锁死）”期间检测中断风暴。“soft lockups”可由多种原因引起。若其中之一由中断风暴导致，则风暴中的中断…… |
| SPARSEMEM | def_bool | SPARSEMEM_VMEMMAP 使用虚拟映射的 memmap 来优化 pfn_to_page 与 page_to_pfn 操作。当内核资源充足时，这是最高效的选项。 |
| SPARSEMEM_MANUAL | bool | 对某些系统（包括内存热插拔系统）而言，这将是唯一选项。这是正常的。此选项为物理地址空间中存在空洞……的系统提供高效支持 |
| SPARSEMEM_VMEMMAP_PREINIT | bool | 热插拔内存的默认内存类型。此选项设置内存热插拔上线策略（/sys/devices/system/memory/auto_online_blocks）的默认策略，该策略决定…… |
| STACKDEPOT_ALWAYS_INIT | bool | 在早期引导期间始终初始化 stack depot。 |
| STACKDEPOT_MAX_FRAMES | int | 运行轻量级排队的引导时测试。 |
| STACKINIT_KUNIT_TEST | tristate | 测试内核是否对栈变量与填充进行零初始化。覆盖范围由编译器标志 CONFIG_INIT_STACK_ALL_PATTERN 或 CONFIG_INIT_STACK_ALL_ZERO 控制。 |
| STACKTRACE | bool | 此选项使内核为每个进程创建 /proc/pid/stack，显示其当前栈跟踪。它也被需要生成栈跟踪的各种内核调试特性使用。 |
| STACKTRACE_BUILD_ID | bool | 选择此选项会为以 printk 格式 '%p[SR]b' 打印的栈跟踪中的符号添加 build ID 信息。此选项面向 debuginfo 不易获取……的发行版 |
| STACK_VALIDATION | bool | 在编译时验证帧指针规则。这有助于确保运行时栈跟踪更可靠。更多信息请参阅 tools/objtool/Documentation/objtool.txt。 |
| STATIC_USERMODEHELPER | bool | 默认情况下，内核可通过“usermode helper（用户态辅助）”内核接口调用许多不同的用户空间二进制程序。其中一些二进制要么在……中静态定义 |
| STATIC_USERMODEHELPER_PATH | string | 当任何 usermode helper 程序希望运行时，内核调用的二进制。被传递的“真实”应用名将在命令行传给该程序的第一个参数中。若你…… |
| STRING_KUNIT_TEST | tristate | 启用对字符串函数的性能测量。它在 KUnit 测试运行期间测量字符串函数的执行效率。若不确定，选 N。 |
| STRIP_ASM_SYMS | bool | 在链接期间剥离汇编器生成的内部符号（形如 '.Lxxx' 的符号），以免它们污染 get_wchan() 等的输出。 |
| SYMBOLIC_ERRNAME | bool | 若在此选 Y，内核的 printf 实现将能够打印符号化的错误名（如 ENOSPC）而非数字 28。这会使内核镜像略大（约 3KB），但…… |
| SYSCTL_ARCH_UNALIGN_ALLOW | bool | 启用对 /proc/sys/kernel/unaligned-trap 的支持。允许体系结构定义/使用 @unaligned_enabled 在运行时切换未对齐访问模拟。参考 arch/parisc/kernel/unaligned.c |
| SYSCTL_ARCH_UNALIGN_NO_WARN | bool | 启用对 /proc/sys/kernel/ignore-unaligned-usertrap 的支持。允许体系结构定义/使用 @no_unaligned_warning 以就可能发生的未对齐访问模拟发出警告。 |
| SYSCTL_KUNIT_TEST | tristate | 构建 proc sysctl 单元测试，在引导时运行。测试 sysctl 的 API 契约与实现正确性。有关 KUnit 与单元测试的更多信息，请参阅…… |
| SYSFS_SYSCALL | bool | sys_sysfs 是一个已废弃的系统调用，libc 不再支持。注意禁用此选项更安全，但可能破坏与某些系统的兼容性。若不确定，在此选 N。 |
| SYSTEM_DATA_VERIFICATION | def_bool | 使用系统可信密钥环的内容提供 PKCS#7 消息验证以供给公钥。这随后可用于模块验证、kexec 镜像验证与固件…… |
| SYSVIPC | bool | 进程间通信（Inter Process Communication）是一套库函数与系统调用，让进程（运行中的程序）同步并交换信息。它通常被认为是一件好事…… |
| SYSVIPC_SYSCTL | bool | POSIX 消息队列是 IPC 的一部分。在 POSIX 消息队列中，每条消息都有一个优先级，决定进程接收它的顺序。若你想编译并运行…… |
| TASKSTATS | bool | 通过通用 netlink 接口导出任务/进程的选定统计信息。与 BSD 进程记账不同，这些统计在任务/进程的生命周期内可用，作为响应…… |
| TASK_DELAY_ACCT | bool | 收集任务等待系统资源（如 cpu、同步块 I/O 完成与页换入）所花时间的信息。此类统计有助于设置任务的优先级…… |
| TASK_IO_ACCOUNTING | bool | 收集此任务引起的存储 I/O 字节数信息。若不确定，选 N。 |
| TASK_XACCT | bool | 收集扩展的任务记账数据，并通过 taskstats 接口将数据发送到用户空间处理。若不确定，选 N。 |
| TEST_BITOPS | tristate | 构建“test_bitops”模块，它与 TEST_LKM 模块非常相似，只是它对 set/clear_bit 宏与 get_count_order/long 做基本演练，以确保没有编译…… |
| TEST_BPF | tristate | 构建“test_bpf”模块，根据当前设置对 BPF 解释器或 BPF JIT 编译器运行各种测试向量。这对 BPF JIT 编译器……特别有用 |
| TEST_CLOCKSOURCE_WATCHDOG | tristate | 启用此选项以创建一个内核模块，触发 clocksource 看门狗的测试。该模块可通过 modprobe 或 insmod 加载，加载时即运行，或…… |
| TEST_DEBUG_VIRTUAL | tristate | 测试内核检测对内核虚拟地址映射非线性部分错误调用 virt_to_phys() 的能力。若不确定，选 N。 |
| TEST_DIV64 | tristate | 启用此项以开启 'do_div()' 函数测试。该测试仅在系统引导期间执行一次（因此只影响引导时间），或在模块加载时执行。若不确定，选 N。 |
| TEST_DYNAMIC_DEBUG | tristate | 该模块注册一个跟踪回调，统计 'do_debugging' 函数中已启用的 pr_debug，然后改变其启用状态、调用该函数并比较计数。若不确定，选 N。 |
| TEST_FIRMWARE | tristate | 构建“test_firmware”模块，它创建一个用于测试固件加载的用户空间接口。这可用于在不需真实固件……的情况下控制固件加载的触发 |
| TEST_FPU | tristate | 启用此选项以添加 /sys/kernel/debug/selftest_helpers/test_fpu，它将触发一系列浮点运算。这用于浮点控制寄存器设置的自我测试…… |
| TEST_FREE_PAGES | tristate | 测试是否因释放一块页与推测性页引用之间的竞争而不发生内存泄漏。若你的内核已修复该缺陷，加载此模块是安全的。若缺陷…… |
| TEST_HEXDUMP | tristate | 启用此选项以在运行时测试 printf 函数。若不确定，选 N。 |
| TEST_HMM | tristate | 这是一个仅用于测试 HMM 的伪设备驱动。若你想构建 HMM 测试模块，在此选 M。这样做将允许你运行 tools/testing/selftest/vm/hmm-tests。若不确定，选 N。 |
| TEST_IDA | tristate | 针对 miscdevice API 的 Kunit 测试，特别是其关于静态与动态次设备号的行为。KUnit 测试在引导期间运行，并以 TAP 格式（https://test……）将结果输出到调试日志 |
| TEST_IOV_ITER | tristate | 启用此项以开启对 I/O 迭代器（iov_iter）操作的测试。该测试仅在系统引导期间执行一次（因此只影响引导时间），或在模块加载时执行。若不确定，选…… |
| TEST_KALLSYMS_A | tristate | 选择“Fast”以外的内容将启用会拖慢构建并可能使构建崩溃的测试。 |
| TEST_KALLSYMS_FAST | bool | 你实际上不会测试 kallsysms，因此这只是在使用 allmodconfig 时帮助快速构建。 |
| TEST_KALLSYMS_LARGE | bool | 这将启用更大数量的符号。这会显著拖慢你的构建。 |
| TEST_KALLSYMS_MAX | bool | 这将启用导出，直到我们知道将开始使构建崩溃的程度。 |
| TEST_KALLSYMS_NUMSYMS | int | 在 TEST_KALLSYMS_A 上创建的符号数量，其中只有 TEST_KALLSYMS_B 模块会使用其一。这也用于确定 TEST_KALLSYMS_C 将拥有的符号数量，由 TEST_KALLS……放大 |
| TEST_KALLSYMS_SCALE_FACTOR | int | TEST_KALLSYSMS_C 比 TEST_KALLSYMS_A 多出的未使用符号数量。若为 8，则模块 C 拥有的符号是模块 A 的 8 倍。然后 TEST_KALLSYMS_D 拥有的符号数量是……的两倍 |
| TEST_KEXEC_HANDOVER | bool | 此选项启用 Kexec HandOver（KHO）测试。测试由两部分组成：在 kexec 之前保存内核数据，并在 kexec 之后恢复数据并验证其被正确移交…… |
| TEST_KMOD | tristate | 测试内核的模块加载机制 kmod。kmod 实现使用 Linux 内核 usermode helper 加载模块的支持。该测试提供一系列针对 kmod 的测试。尽管技术…… |
| TEST_KSTRTOX | tristate | 启用此选项以在引导时测试 bitmap 函数。若不确定，选 N。 |
| TEST_LIST_SORT | tristate | 启用此项以开启 'list_sort()' 函数测试。该测试仅在系统引导期间执行一次（因此只影响引导时间），或在模块加载时执行。若不确定，选 N。 |
| TEST_LKM | tristate | 构建“test_module”模块，加载时通过 printk 输出“Hello, world”。它设计用于模块加载子系统的基本评估（例如验证模块……） |
| TEST_LOCKUP | tristate | 构建“test_lockup”模块，帮助确保看门狗与锁死检测器正常工作。根据模块参数，它可以模拟软锁死或硬锁死、“hung tas……” |
| TEST_MEMCAT_P | tristate | 测试 memcat_p() 辅助函数是否正确合并两个指针数组。若不确定，选 N。 |
| TEST_MEMINIT | tristate | 测试内核是否对堆与页分配进行零初始化。这有助于测试 init_on_alloc 与 init_on_free 特性。若不确定，选 N。 |
| TEST_MULDIV64 | tristate | 启用此项以开启 'mul_u64_u64_div_u64()' 函数测试。该测试仅在系统引导期间执行一次（因此只影响引导时间），或在模块加载时执行。若不确定，选 N。 |
| TEST_OBJAGG | tristate | 启用此选项以在引导时（或模块加载时）测试对象聚合管理器。 |
| TEST_OBJPOOL | tristate | 构建“test_objpool”模块，用于对象分配与回收的正确性与并发测试。若不确定，选 N。 |
| TEST_PARMAN | tristate | 启用此选项以在引导时（或模块加载时）测试优先级数组管理器。若不确定，选 N。 |
| TEST_REF_TRACKER | tristate | 此选项提供一个使用引用跟踪器基础设施执行测试的内核模块。若不确定，选 N。 |
| TEST_RHASHTABLE | tristate | 启用此选项以在引导时测试 rhashtable 函数。若不确定，选 N。 |
| TEST_RUNTIME | bool | 这允许我们通过用于在 kallsyms 上放置符号（如导出符号）的 kallsyms 来对 find_symbol() 进行压力测试。我们…… |
| TEST_SORT | tristate | 此选项在引导时或模块加载时启用 'sort()' 的自检测试函数。若不确定，选 N。 |
| TEST_STATIC_KEYS | tristate | 测试静态键接口。若不确定，选 N。 |
| TEST_SYSCTL | tristate | 构建“test_sysctl”模块。该驱动可安全测试驱动可用的 proc sysctl 接口，而不影响可能改变系统功能的生成旋钮。若…… |
| TEST_UDELAY | tristate | 构建“udelay_test”模块，帮助确保 udelay() 工作正常。若不确定，选 N。 |
| TEST_VMALLOC | tristate | 构建“test_vmalloc”模块，用于压力与性能分析。因此，对 vmalloc 子系统的任何新改动都可从性能与稳定性角度评估…… |
| TEST_WORKQUEUE | tristate | 构建“test_workqueue”模块，用于在争用下对工作队列吞吐量进行基准测试。有助于评估亲和性范围变化（如 cache_shard 与 cache）。若不确定，选 N。 |
| TEST_XARRAY | tristate | 启用此选项以在引导时或模块加载时测试 maple tree 代码函数。启用“Debug Maple Trees”将在失败时输出更详细的日志。若不确定，选 N。 |
| TEXTSEARCH | bool | 简单、可嵌入的区间树。可在 log(n) 时间内找到重叠范围的起点，然后遍历所有重叠节点。该算法实现为增强 rbtree。参见：D…… |
| THP_SWAP | def_bool | 以整体方式交换透明大页，无需拆分。XXX：目前，支撑透明大页的交换簇将在换出后被拆分。供具有合理 THP……的体系结构选择 |
| TIMERFD | bool | 启用 timerfd() 系统调用，它允许在文件描述符上接收定时事件。若不确定，选 Y。 |
| TIME_NS | bool | 在该命名空间中，boottime 与单调时钟可被设置。时间将以相同节奏继续走。 |
| TIME_NS_VDSO | def_bool | 在该命名空间中，任务使用对应于不同命名空间中不同 IPC 对象的 IPC id。 |
| TMPFS | bool | Tmpfs 是一种将所有文件保存在虚拟内存中的文件系统。Tmpfs 中的一切都是临时的，意味着不会在你的硬盘上创建文件。文件存在于内存与交换空间…… |
| TMPFS_INODE64 | bool | tmpfs 历史上只使用与 unsigned int 等宽的文件号。在某些情况下这会导致回绕，可能使单个设备……上出现多个具有相同文件号的文件 |
| TMPFS_POSIX_ACL | bool | POSIX 访问控制列表（ACL）为标准的所有者/组/其他方案之外的用户与组提供额外的访问权限，此选项选择对 tmpfs 的 ACL 支持…… |
| TMPFS_QUOTA | bool | 配额支持允许为 tmpfs 使用设置每用户与每组的限制。选 Y 以启用配额支持。启用后，你可通过 quota、usrquota 与 grpquot……控制用户与组配额的强制 |
| TMPFS_XATTR | bool | 扩展属性是由内核或用户与 inode 关联的 名称:值 对（详见 attr(5) 手册页）。这启用对 trusted.*、security.* 与 user.* 名称……的支持 |
| TRACE_IRQFLAGS | bool | 启用用于跟踪或锁调试的中断启用/禁用钩子。 |
| TRACE_IRQFLAGS_NMI | def_bool | 当 CPU 未能响应给定的 backtrace NMI 时启用调试打印。这些打印提供一些 CPU 可能合理地未能响应的原因，例如它处于离线状态或…… |
| TRACE_MMIO_ACCESS | bool | 为 MMIO 读/写操作创建跟踪点。这些跟踪事件可用于记录所有 MMIO 读/写操作。 |
| TRANSPARENT_HUGEPAGE_ALWAYS | bool | 总是启用透明大页，可能会增加应用的内存占用而没有保证的收益，但它会对所有应用自动生效。 |
| TRANSPARENT_HUGEPAGE_MADVISE | bool | 启用透明大页 madvise，只会为使用 madvise(MADV_HUGEPAGE) 的应用带来性能提升，但不会冒增加应用内存占用的风险…… |
| TRANSPARENT_HUGEPAGE_NEVER | bool | 默认禁用透明大页。仍可在运行时通过 sysfs 启用。 |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ADVISE | bool | 仅当应用提供 madvise(MADV_HUGEPAGE) 提示时，才为 shmem 挂载启用大页分配。这确保大页仅在响应来自……的显式请求时使用 |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ALWAYS | bool | 总是尝试为 shmem 挂载分配大页，可能会增加应用内存占用而没有保证的收益，但它会对所有应用自动生效。 |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_NEVER | bool | 默认禁用 shmem 挂载的大页分配。仍可通过内核命令行 'transparent_hugepage_shmem=' 选项或运行时 sysfs 旋钮启用。注意 madvise(MAD…… |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_WITHIN_SIZE | bool | 若分配将完全位于 i_size 之内，则为 shmem 挂载启用大页分配。此配置还考虑……可能提供的任何 madvise(MADV_HUGEPAGE) 提示 |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ADVISE | bool | 仅当应用提供 madvise(MADV_HUGEPAGE) 提示时，才为 tmpfs 挂载启用大页分配。这确保大页仅在响应来自……的显式请求时使用 |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ALWAYS | bool | 总是尝试为 tmpfs 挂载分配大页，可能会增加应用内存占用而没有保证的收益，但它会对所有应用自动生效。 |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_NEVER | bool | 默认禁用 tmpfs 挂载的大页分配。仍可通过内核命令行 'transparent_hugepage_tmpfs=' 选项启用。注意 madvise(MADV_COLLAPSE) 仍可能导致…… |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_WITHIN_SIZE | bool | 若分配将完全位于 i_size 之内，则为 tmpfs 挂载启用大页分配。此配置还考虑……可能提供的任何 madvise(MADV_HUGEPAGE) 提示 |
| UAPI_HEADER_TEST | bool | 编译测试导出到用户空间的头文件，以确保它们是自包含的（即可作为独立单元编译）。若你是开发者或测试者并希望确保导出的头文件是自包含…… |
| UCLAMP_BUCKETS_COUNT | int | 定义要使用的钳位桶数量。每个桶的范围将是 SCHED_CAPACITY_SCALE/UCLAMP_BUCKETS_COUNT。钳位桶数量越多，粒度越细，且…… |
| UCS2_STRING | tristate | 提供一个将散列表拆分为多个块（每块是一个散列表）的辅助函数。应由希望在多个 DMA 通道间拆分散列表的驱动或 API 选中。 |
| UID16 | bool | 这启用传统的 16 位 UID 系统调用封装。 |
| USERCOPY_KUNIT_TEST | tristate | 构建“usercopy_kunit”模块，对 copy_to/from_user 基础设施运行健全性检查，确保基本的用户/内核边界测试正常工作。 |
| USERFAULTFD | bool | 启用 userfaultfd() 系统调用，它允许在用户空间拦截并处理缺页。若 USERFAULTFD |
| USER_NS | bool | 这允许容器（即 vservers）使用用户命名空间为不同服务器提供不同的用户信息。当内核中启用了用户命名空间时，建议将 MEMCG 或…… |
| UTIL_MACROS_KUNIT | tristate | 启用此选项以在引导时测试 util_macros.h 函数。KUnit 测试在引导期间运行，并以 TAP 格式（http://testanything.org/）将结果输出到调试日志。仅对内核……有用 |
| UTS_NS | bool | 在该命名空间中，任务看到 uname() 系统调用提供的不同信息 |
| UUID_KUNIT_TEST | tristate | 此选项启用针对 uuid 库（提供生成与解析 UUID 和 GUID 的函数）的 KUnit 测试套件。该测试套件检查 UUID 与 GUID 字符串的解析。若不确定…… |
| VIRT_CPU_ACCOUNTING_GEN | bool | 选择此选项以在完全 dynticks 系统上启用任务与 CPU 时间记账。该记账通过 context tracking 子系统监视每个内核-用户边界来实现。该…… |
| VIRT_CPU_ACCOUNTING_NATIVE | bool | 选择此选项以启用更精确的任务与 CPU 时间记账。这通过在每次内核进入与退出以及内核内系统态……之间转换时读取 CPU 计数器来实现 |
| VMAP_PFN | bool | 显示事件计数需要 VM 事件计数器。此选项允许在 EXPERT 系统上禁用 VM 事件计数器。/proc/vmstat 仅在 VM 事件计数器……存在时才显示页计数 |
| WANT_COMPAT_NETLINK_MESSAGES | bool | 此选项可由需要 compat netlink 消息的其他选项选中。 |
| WARN_ABI_ERRORS | bool | Documentation/ABI 下的文件应遵循 Documentation/ABI/README 中的描述。然而，由于它们是手动编写的，某些文件可能存在错误…… |
| WARN_CONTEXT_ANALYSIS | bool | 上下文分析是一种语言扩展，它通过在获取与释放用户可定义的“context locks”时静态检查所需上下文是否处于活动（或非活动）状态。Clang 的名称…… |
| WARN_CONTEXT_ANALYSIS_ALL | bool | 启用全树范围的上下文分析。这很可能产生大量误报——风险自负。若不确定，选 N。 |
| WARN_MISSING_DOCUMENTS | bool | 文档被重命名并不罕见。此选项让内核检查缺失的依赖项，并在缺失时发出警告。仅当内核从 git 树……构建时才有效 |
| WERROR | bool | 内核构建不应产生任何编译器警告，此选项默认启用 '-Werror'（用于 C）与 '-Dwarnings'（用于 Rust）标志以强制该规则。来自其他工具的某些警告…… |
| WQ_CPU_INTENSIVE_REPORT | bool | 在此选 Y 以启用对占用 CPU 超过 workqueue.cpu_intensive_thresh_us 的并发管理 per-cpu 工作项的报告。工作队列会自动检测并将它们排除出并发…… |
| WQ_WATCHDOG | bool | 在此选 Y 以启用对工作队列的停顿检测。若一个工作池在超过给定时间（默认 30 秒）内未在待处理工作项上取得进展，会打印一条警告消息…… |
| WW_MUTEX_SELFTEST | tristate | 此选项提供一个内核模块，对 struct ww_mutex 锁 API 运行测试。建议配合此测试工具启用 DEBUG_WW_MUTEX_SLOWPATH。若……选 M |
| XXHASH | tristate | 此选项启用 32 位 PRNG 库函数，在初始化时执行自检测试。# # 压缩支持在需要时被选中 # |
| ZSMALLOC_CHAIN_SIZE | int | 此选项设置 zmalloc 页（zspage）可包含的物理页数量上限。最优的 zspage 链大小在初始化期间为每个大小类计算…… |
| ZSWAP | bool | 一个用于交换页的轻量级压缩缓存。它接收正在被换出的页，并尝试将其压缩到动态分配的基于 RAM 的内存池中。这可…… |
| ZSWAP_COMPRESSOR_DEFAULT | string | 此选项启用 zsmalloc 中的代码，以收集 zsmalloc 内部发生情况的各类统计，并通过 debugfs 将其导出到用户空间。若不确定，选 N。 |
| ZSWAP_COMPRESSOR_DEFAULT_842 | bool | 使用 842 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_DEFLATE | bool | 使用 Deflate 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_LZ4 | bool | 使用 LZ4 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_LZ4HC | bool | 使用 LZ4HC 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_LZO | bool | 使用 LZO 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_ZSTD | bool | 使用 zstd 算法作为默认压缩算法。 |
| ZSWAP_DEFAULT_ON | bool | 若选中，交换页的压缩缓存将在引导时启用，否则禁用。此处所做的选择可通过内核命令行 'zswap.enabled='……覆盖 |
| ZSWAP_SHRINKER_DEFAULT_ON | bool | 若选中，zswap shrinker 将被启用，存储在 zswap 池中的页将在内存压力下可用于回收（即写回后备交换设备）。这意味…… |
| if | bool | 若在此选 Y，gcc 将被指示为结构体类型生成更少的调试信息。这意味着需要完整调试信息（如 kgdb 或 systemtap）的工具将不满意。但…… |
| select | bool | 生成 DWARF v5 调试信息。需要 binutils 2.35.2、gcc 5.0+（gcc 5.0+ 接受 -gdwarf-5 标志，但在 7.0 之前对某些草案特性仅部分支持）以及 gdb 8.0+。对 s……的更改 |

---

# Makefile 目标

## 构建目标

| 目标 | 描述 | 来源 |
|--------|-------------|--------|
| all | 如果构建外部模块，我们并不关心 all: 规则，而是让 __all 依赖于 modules | Makefile |
| dtbs_install |  | Makefile |
| headers |  | Makefile |
| headers_install |  | Makefile |
| modules | 构建所有可加载的内核模块 | Makefile |
| modules_install |  | Makefile |
| vmlinux |  | Makefile |

## 配置目标

| 目标 | 描述 | 来源 |
|--------|-------------|--------|
| config |  | Makefile |

## 清理目标

| 目标 | 描述 | 来源 |
|--------|-------------|--------|
| clean | clean - 删除大部分文件，但保留足够内容以构建外部模块 | Makefile |
| distclean | distclean  | Makefile |
| mrproper | mrproper - 删除所有生成的文件，包括 .config | Makefile |

## 文档目标

| 目标 | 描述 | 来源 |
|--------|-------------|--------|
| cleandocs | 删除所有生成的文档文件 | Makefile |
| htmldocs-redirects |  | Makefile |
| markdowndocs | 通过 Pandoc 后处理构建 Markdown 文档 | Makefile |
| refcheckdocs | 检查文档中损坏的文件引用 | Makefile |

## 其他目标

| 目标 | 描述 | 来源 |
|--------|-------------|--------|
| FORCE |  | Makefile |
| archprepare |  | Makefile |
| asm-generic |  | Makefile |
| checkstack |  | Makefile |
| clang-tidy |  | Makefile |
| coccicheck |  | Makefile |
| dochelp |  | Makefile |
| dt_binding_check |  | Makefile |
| dt_binding_schemas |  | Makefile |
| dt_compatible_check |  | Makefile |
| dtbs |  | Makefile |
| dtbs_check |  | Makefile |
| dtbs_prepare | 安装 DTB 时实际确实需要 include/config/kernel.release，因为 INSTALL_DTBS_PATH 包含 $(KERNELRELEASE)。但我们不希望让 dtbs_install 依赖于它，因为 dtbs_install 可能以 root 身份运行。 | Makefile |
| headerdep |  | Makefile |
| help | 显示可用的 make 目标 | Makefile |
| help-boards |  | Makefile |
| image_name |  | Makefile |
| includecheck |  | Makefile |
| kernelrelease |  | Makefile |
| kernelversion |  | Makefile |
| kselftest |  | Makefile |
| kselftest-merge |  | Makefile |
| misc-check |  | Makefile |
| modpost |  | Makefile |
| nsdeps |  | Makefile |
| outputmakefile |  | Makefile |
| prepare |  | Makefile |
| prepare0 |  | Makefile |
| remove-stale-files |  | Makefile |
| run-command |  | Makefile |
| rust-analyzer | 为 rust-analyzer（Language Server Protocol 的一种实现）生成 rust-project.json（描述非 Cargo Rust 项目结构的文件）。 | Makefile |
| rustavailable | "Rust 是否可用？" 目标 | Makefile |
| rustdoc | 文档目标  使用单数形式以避免触犯 `no-dot-config-targets`。 | Makefile |
| rustfmt | 格式化目标  生成的文件以及 vendored crates 将被跳过。 | Makefile |
| rustfmtcheck |  | Makefile |
| rusttest | 测试目标 | Makefile |
| scripts | scripts/ 中构建的额外辅助工具  仔细列出依赖关系，以免并行构建时重复构建 scripts。 | Makefile |
| scripts_basic | 在 scripts/basic/ 中构建的基础辅助工具 | Makefile |
| scripts_dtc |  | Makefile |
| scripts_gdb |  | Makefile |
| scripts_gen_packed_field_checks |  | Makefile |
| scripts_unifdef |  | Makefile |
| uapi-asm-generic |  | Makefile |
| usr_gen_init_cpio |  | Makefile |
| versioncheck |  | Makefile |

---

# 子系统说明

## arch/

特定于体系结构的代码（arm64、x86、riscv、m68k、powerpc 等）以及引导基础设施。

## crypto/

加密 API 与算法实现。

## drivers/

设备驱动（网络、块设备、字符设备、声卡、GPU、USB、PCI、infiniband 等）以及驱动核心。

## fs/

文件系统（ext4、btrfs、xfs、fuse、overlayfs、nfs、jffs2、cramfs 等）。

## include/

内核公共头文件（linux/、asm-generic/、uapi/）。

## io_uring/

io_uring 异步 I/O 子系统。

## ipc/

进程间通信（msg、sem、shm）。

## kernel/

核心内核子系统（调度器、printk、irq、时间、加锁、RCU、BPF 等）。

## lib/

内核通用库（位图、rbtree、radix-tree、crc、kunit 等）。

## mm/

内存管理（页分配器、slab、vmalloc、hugetlb、swap、mmap 等）。

## net/

网络协议栈（ipv4、ipv6、netfilter、BPF、核心、以太网、无线等）。

## rust/

Rust 内核支持（bindings、核心、helpers、vendored crates）。

## samples/

示例与教程代码（BPF、vfio-mdev、pktgen）。

## scripts/

构建脚本、checkpatch、coccinelle 补丁、kconfig、modpost 等。

## security/

安全模块（selinux、apparmor、landlock、smack 等）。

## sound/

ALSA 声音子系统与音频驱动。

## tools/

用户空间工具（perf、bpftool、selftests、kunit、cpupower 等）。

## virt/

虚拟化（KVM、UML、Xen 等）。
