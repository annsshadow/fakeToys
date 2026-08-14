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

# Kconfig Summary

## Other

| Config | Type | Description |
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
| BOOT_PRINTK_DELAY | bool | This build option allows you to read kernel boot messages by inserting a short delay after each one.  The delay is specified in milliseconds on the kernel command line, using "boot_delay=N". It is ... |
| BRIDGE_NETFILTER | tristate | Enabling this option will let arptables resp. iptables see bridged ARP resp. IP traffic. If you want a bridging firewall, you probably want this option enabled. Enabling or disabling this option do... |
| BROKEN | bool | This option allows you to choose whether you want to try to compile (and fix) old drivers that haven't been updated to new infrastructure. |
| BROKEN_ON_SMP | bool | Maximum of each of the number of arguments and environment variables passed to init from the kernel command line. |
| BSD_PROCESS_ACCT | bool | If you say Y here, a user level program will be able to instruct the kernel (via a special system call) to write process accounting information to a file: whenever a process exits, information abou... |
| BSD_PROCESS_ACCT_V3 | bool | If you say Y here, the process accounting information is written in a new file format that also logs the process IDs of each process and its parent. Note that this file format is incompatible with ... |
| BUG | bool | Disabling this option eliminates support for BUG and WARN, reducing the size of your kernel image and potentially quietly ignoring numerous fatal conditions. You should only consider disabling this... |
| BUILD_SALT | string | The build ID is used to link binaries and their debug info. Setting this option will use the value in the calculation of the build id. This is mostly useful for distributions which want to ensure t... |
| BUILTIN_MODULE_RANGES | bool | When modules are built into the kernel, there will be no module name associated with its symbols in /proc/kallsyms.  Tracers may want to identify symbols by module name and symbol name regardless o... |
| CACHESTAT_SYSCALL | bool | Enable the cachestat system call, which queries the page cache statistics of a file (number of cached pages, dirty pages, pages marked for writeback, (recently) evicted pages). If unsure say Y here. |
| CC_IS_GCC | def_bool | It does not depend on `RUST` since that one may need to use the version in a `depends on`. |
| CC_OPTIMIZE_FOR_PERFORMANCE | bool | This is the default optimization level for the kernel, building with the "-O2" compiler flag for best performance and most helpful compile-time warnings. |
| CC_OPTIMIZE_FOR_SIZE | bool | Choosing this option will pass "-Os" to your compiler resulting in a smaller kernel. |
| CC_VERSION_TEXT | string | This is used in unclear ways: - Re-run Kconfig when the compiler is updated The 'default' property references the environment variable, CC_VERSION_TEXT so it is recorded in include/config/auto.conf... |
| CGROUP_BPF | bool | Allow attaching eBPF programs to a cgroup using the bpf(2) syscall command BPF_PROG_ATTACH. In which context these programs are accessed depends on the type of attachment. For instance, programs th... |
| CGROUP_CPUACCT | bool | Provides a simple controller for monitoring the total CPU consumed by the tasks in a cgroup. |
| CGROUP_DEBUG | bool | This option enables a simple controller that exports debugging information about the cgroups framework. This controller is for control cgroup debugging only. Its interfaces are not stable. Say N. |
| CGROUP_DMEM | bool | The DMEM controller allows compatible devices to restrict device memory usage based on the cgroup hierarchy. As an example, it allows you to restrict VRAM usage for applications in the DRM subsystem. |
| CGROUP_FREEZER | bool | Provides a way to freeze and unfreeze all tasks in a cgroup. This option affects the ORIGINAL cgroup interface. The cgroup2 memory controller includes important in-kernel memory consumers per defau... |
| CGROUP_HUGETLB | bool | Provides a cgroup controller for HugeTLB pages. When you enable this, you can put a per cgroup limit on HugeTLB usage. The limit is enforced during page fault. Since HugeTLB doesn't support page re... |
| CGROUP_MISC | bool | Provides a controller for miscellaneous resources on a host. Miscellaneous scalar resources are the resources on the host system which cannot be abstracted like the other cgroups. This controller t... |
| CGROUP_NET_CLASSID | bool | Cgroup subsystem for use as general purpose socket classid marker that is being used in cls_cgroup and for netfilter matching. |
| CGROUP_PERF | bool | This option extends the perf per-cpu mode to restrict monitoring to threads which belong to the cgroup specified and run on the designated cpu.  Or this can be used to have cgroup ID in samples so ... |
| CGROUP_PIDS | bool | Provides enforcement of process number limits in the scope of a cgroup. Any attempt to fork more processes than is allowed in the cgroup will fail. PIDs are fundamentally a global resource because ... |
| CGROUP_RDMA | bool | Provides enforcement of RDMA resources defined by IB stack. It is fairly easy for consumers to exhaust RDMA resources, which can result into resource unavailability to other consumers. RDMA control... |
| CGROUP_WRITEBACK | bool | This feature lets CPU scheduler recognize task groups and control CPU bandwidth allocation to such task groups. It uses cgroups to group tasks. if CGROUP_SCHED |
| CHECKPOINT_RESTORE | bool | Enables additional kernel features in a sake of checkpoint/restore. In particular it adds auxiliary prctl codes to setup process text, data and heap segment sizes, and a few additional /proc filesy... |
| CHECKSUM_KUNIT | tristate | Enable this option to test the checksum functions at boot. KUnit tests run during boot and output the results to the debug log in TAP format (http://testanything.org/). Only useful for kernel devs ... |
| CLOSURES | bool | Use dynamic allocation for cpumask_var_t, instead of putting them on the stack.  This is a bit more expensive, but avoids stack overflow. |
| CMA_AREAS | int | CMA allows to create CMA areas for particular purpose, mainly, used as device private area. This parameter sets the maximum number of CMA area in the system. If unsure, leave the default value "8" ... |
| CMA_DEBUGFS | bool | Turns on the DebugFS interface for CMA. |
| CMA_SYSFS | bool | This option exposes some sysfs attributes to get information from CMA. |
| CMDLINE_KUNIT_TEST | tristate | This builds the cmdline API unit test. Tests the logic of API provided by cmdline.c. For more information on KUnit and unit tests in general please refer to the KUnit documentation in Documentation... |
| CMDLINE_LOG_WRAP_IDEAL_LEN | int | At boot time, the kernel command line is logged to the console. The log message will start with the prefix "Kernel command line: ". The log message will attempt to be wrapped (split into multiple l... |
| CODE_TAGGING | bool | Track allocation source code and record total allocation size initiated at that code location. The mechanism can be used to track memory leaks with a low performance and memory impact. |
| COMPACTION | bool | Compaction is the only memory management component to form high order (larger physically contiguous) memory blocks reliably. The page allocator relies on compaction heavily and the lack of the feat... |
| COMPACT_UNEVICTABLE_DEFAULT | int | Free page reporting allows for the incremental acquisition of free pages from the buddy allocator for the purpose of reporting those pages to another entity, such as a hypervisor, so that the memor... |
| COMPAT_BINFMT_ELF | def_bool | ELF FDPIC binaries are based on ELF, but allow the individual load segments of a binary to be located in memory independently of each other. This makes this format ideal for use in environments whe... |
| COMPAT_BRK | bool | Randomizing heap placement makes heap exploits harder, but it also breaks ancient binaries (including anything libc5 based). This option changes the bootup default to heap randomization disabled, a... |
| COMPAT_NETLINK_MESSAGES | def_bool | This option makes it possible to send different netlink messages to tasks depending on whether the task is a compat task or not. To achieve this, you need to set skb_shinfo(skb)->frag_list to the c... |
| COMPILE_TEST | bool | Some drivers can be compiled on a different platform than they are intended to be run on. Despite they cannot be loaded there (or even when they load they cannot be used due to missing HW support),... |
| CONSOLE_LOGLEVEL_DEFAULT | int | Default loglevel to determine what will be printed on the console. Setting a default here is equivalent to passing in loglevel=<x> in the kernel bootargs. loglevel=<x> continues to override whateve... |
| CONSOLE_LOGLEVEL_QUIET | int | loglevel to use when "quiet" is passed on the kernel commandline. When "quiet" is passed on the kernel commandline this loglevel will be used as the loglevel. IOW passing "quiet" will be the equiva... |
| CONTEXT_ANALYSIS_TEST | bool | This builds the test for compiler-based context analysis. The test does not add executable code to the kernel, but is meant to test that common patterns supported by the analysis do not result in f... |
| CONTIG_ALLOC | def_bool | In page allocator, PCP (Per-CPU pageset) is refilled and drained in batches.  The batch number is scaled automatically to improve page allocation/free throughput.  But too large scale factor may hu... |
| COREDUMP | bool | This option enables support for performing core dumps. You almost certainly want to say Y here. Not necessary on systems that never need debugging or only ever run flawless code. |
| CORE_DUMP_DEFAULT_ELF_HEADERS | bool | ELF core dump files describe each memory mapping of the crashed process, and can contain or omit the memory contents of each one. The contents of an unmodified text mapping are omitted by default. ... |
| CPUMASK_KUNIT_TEST | tristate | Enable to turn on cpumask tests, running at boot or module load time. For more information on KUnit and unit tests in general, please refer to the KUnit documentation in Documentation/dev-tools/kun... |
| CPUSETS | bool | This option will let you create and manage CPUSETs which allow dynamically partitioning a system into sets of CPUs and Memory Nodes and assigning tasks to run only within those sets. This is primar... |
| CPUSETS_V1 | bool | Legacy cgroup v1 cpusets controller which has been deprecated by cgroup v2 implementation. The v1 is there for legacy applications which haven't migrated to the new cgroup v2 interface yet. Legacy ... |
| CPU_HOTPLUG_STATE_CONTROL | bool | Allows to write steps between "offline" and "online" to the CPUs sysfs target file so states can be stepped granular. This is a debug option for now as the hotplug machinery cannot be stopped and r... |
| CPU_ISOLATION | bool | Make sure that CPUs running critical tasks are not disturbed by any source of "noise" such as unbound workqueues, timers, kthreads... Unbound jobs get offloaded to housekeeping CPUs. This is driven... |
| CROSS_MEMORY_ATTACH | bool | Enabling this option adds the system calls process_vm_readv and process_vm_writev which allow a process with the correct privileges to directly read from or write to another process' address space.... |
| CRYPTO | tristate | This option provides the core Cryptographic API. if CRYPTO |
| CRYPTO_842 | tristate | 842 compression algorithm by IBM See https://github.com/plauth/lib842 for further information. |
| CRYPTO_ADIANTUM | tristate | Adiantum tweakable, length-preserving encryption mode Designed for fast and secure disk encryption, especially on CPUs without dedicated crypto instructions.  It encrypts each sector using the XCha... |
| CRYPTO_AEGIS128 | tristate | AEGIS-128 AEAD algorithm |
| CRYPTO_AEGIS128_SIMD | bool | AEGIS-128 AEAD algorithm Architecture: arm or arm64 using: - NEON (Advanced SIMD) extension |
| CRYPTO_AES | tristate | AES cipher algorithms (Rijndael)(FIPS-197, ISO/IEC 18033-3) Rijndael appears to be consistently a very good performer in both hardware and software across a wide range of computing environments reg... |
| CRYPTO_ALGAPI | tristate | This option provides the API for cryptographic algorithms. |
| CRYPTO_ALGAPI2 | tristate | This provides the support for instantiating templates such as cbc(aes), and the support for the crypto self-tests. |
| CRYPTO_ANUBIS | tristate | Anubis cipher algorithm Anubis is a variable key length cipher which can use keys from 128 bits to 320 bits in length.  It was evaluated as a entrant in the NESSIE competition. See https://web.arch... |
| CRYPTO_ARC4 | tristate | ARC4 cipher algorithm ARC4 is a stream cipher using keys ranging from 8 bits to 2048 bits in length.  This algorithm is required for driver-based WEP, but it should not be for other purposes becaus... |
| CRYPTO_ARIA | tristate | ARIA cipher algorithm (RFC5794) ARIA is a standard encryption algorithm of the Republic of Korea. The ARIA specifies three key sizes and rounds. 128-bit: 12 rounds. 192-bit: 14 rounds. 256-bit: 16 ... |
| CRYPTO_AUTHENC | tristate | Authenc: Combined mode wrapper for IPsec. This is required for IPSec ESP (XFRM_ESP). |
| CRYPTO_BENCHMARK | tristate | Quick & dirty crypto benchmarking module. This is mainly intended for use by people developing cryptographic algorithms in the kernel.  It should not be enabled in production kernels. |
| CRYPTO_BLAKE2B | tristate | BLAKE2b cryptographic hash function (RFC 7693) BLAKE2b is optimized for 64-bit platforms and can produce digests of any size between 1 and 64 bytes. The keyed hash is also implemented. This module ... |
| CRYPTO_BLOWFISH | tristate | Blowfish cipher algorithm, by Bruce Schneier This is a variable key length cipher which can use keys from 32 bits to 448 bits in length.  It's fast, simple and specifically designed for use on "lar... |
| CRYPTO_BLOWFISH_COMMON | tristate | Common parts of the Blowfish cipher algorithm shared by the generic c and the assembler implementations. |
| CRYPTO_CAMELLIA | tristate | Camellia cipher algorithms (ISO/IEC 18033-3) Camellia is a symmetric key block cipher developed jointly at NTT and Mitsubishi Electric Corporation. The Camellia specifies three key sizes: 128, 192 ... |
| CRYPTO_CAST5 | tristate | CAST5 (CAST-128) cipher algorithm (RFC2144, ISO/IEC 18033-3) |
| CRYPTO_CAST6 | tristate | CAST6 (CAST-256) encryption algorithm (RFC2612) |
| CRYPTO_CAST_COMMON | tristate | Common parts of the CAST cipher algorithms shared by the generic c and the assembler implementations. |
| CRYPTO_CBC | tristate | CBC (Cipher Block Chaining) mode (NIST SP800-38A) This block cipher mode is required for IPSec ESP (XFRM_ESP). |
| CRYPTO_CCM | tristate | CCM (Counter with Cipher Block Chaining-Message Authentication Code) authenticated encryption mode (NIST SP800-38C) |
| CRYPTO_CHACHA20 | tristate | The ChaCha20, XChaCha20, and XChaCha12 stream cipher algorithms ChaCha20 is a 256-bit high-speed stream cipher designed by Daniel J. Bernstein and further specified in RFC7539 for use in IETF proto... |
| CRYPTO_CHACHA20POLY1305 | tristate | ChaCha20 stream cipher and Poly1305 authenticator combined mode (RFC8439) |
| CRYPTO_CMAC | tristate | CMAC (Cipher-based Message Authentication Code) authentication mode (NIST SP800-38B and IETF RFC4493) |
| CRYPTO_CRC32 | tristate | CRC32 CRC algorithm (IEEE 802.3) |
| CRYPTO_CRC32C | tristate | CRC32c CRC algorithm with the iSCSI polynomial (RFC 3385 and RFC 3720) A 32-bit CRC (cyclic redundancy check) with a polynomial defined by G. Castagnoli, S. Braeuer and M. Herrman in "Optimization ... |
| CRYPTO_CRYPTD | tristate | This is a generic software asynchronous crypto daemon that converts an arbitrary synchronous software crypto algorithm into an asynchronous algorithm that executes in a kernel thread. |
| CRYPTO_CTR | tristate | CTR (Counter) mode (NIST SP800-38A) |
| CRYPTO_CTS | tristate | CBC-CS3 variant of CTS (Cipher Text Stealing) (NIST Addendum to SP800-38A (October 2010)) This mode is required for Kerberos gss mechanism support for AES encryption. |
| CRYPTO_DEFLATE | tristate | Deflate compression algorithm (RFC1951) Used by IPSec with the IPCOMP protocol (RFC3173, RFC2394) |
| CRYPTO_DES | tristate | DES (Data Encryption Standard)(FIPS 46-2, ISO/IEC 18033-3) and Triple DES EDE (Encrypt/Decrypt/Encrypt) (FIPS 46-3, ISO/IEC 18033-3) cipher algorithms |
| CRYPTO_DH | tristate | DH (Diffie-Hellman) key exchange algorithm |
| CRYPTO_DH_RFC7919_GROUPS | bool | FFDHE (Finite-Field-based Diffie-Hellman Ephemeral) groups defined in RFC7919. Support these finite-field groups in DH key exchanges: - ffdhe2048, ffdhe3072, ffdhe4096, ffdhe6144, ffdhe8192 If unsu... |
| CRYPTO_DRBG | tristate | CPU Jitter RNG (Random Number Generator) from the Jitterentropy library A non-physical non-deterministic ("true") RNG (e.g., an entropy source compliant with NIST SP800-90B) intended to provide a s... |
| CRYPTO_DRBG_CTR | bool | CTR_DRBG variant as defined in NIST SP800-90A. This uses the AES cipher algorithm with the counter block mode. |
| CRYPTO_DRBG_HMAC | bool | Hash_DRBG variant as defined in NIST SP800-90A. This uses the SHA-1, SHA-256, SHA-384, or SHA-512 hash algorithms. |
| CRYPTO_DRBG_MENU | tristate | DRBG (Deterministic Random Bit Generator) (NIST SP800-90A) In the following submenu, one or more of the DRBG types must be selected. if CRYPTO_DRBG_MENU |
| CRYPTO_ECB | tristate | ECB (Electronic Codebook) mode (NIST SP800-38A) |
| CRYPTO_ECC | tristate | ECDH (Elliptic Curve Diffie-Hellman) key exchange algorithm using curves P-192, P-256, and P-384 (FIPS 186) |
| CRYPTO_ECDSA | tristate | ECDSA (Elliptic Curve Digital Signature Algorithm) (FIPS 186, ISO/IEC 14888-3) using curves P-192, P-256, P-384 and P-521 Only signature verification is implemented. |
| CRYPTO_ECHAINIV | tristate | Encrypted Chain IV generator This IV generator generates an IV based on the encryption of a sequence number xored with a salt.  This is the default algorithm for CBC. |
| CRYPTO_ECRDSA | tristate | Elliptic Curve Russian Digital Signature Algorithm (GOST R 34.10-2012, RFC 7091, ISO/IEC 14888-3) One of the Russian cryptographic standard algorithms (called GOST algorithms). Only signature verif... |
| CRYPTO_ESSIV | tristate | Encrypted Salt-Sector IV generator This IV generator is used in some cases by fscrypt and/or dm-crypt. It uses the hash of the block encryption key as the symmetric key for a block encryption pass ... |
| CRYPTO_FCRYPT | tristate | FCrypt algorithm used by RxRPC See https://ota.polyonymo.us/fcrypt-paper.txt |
| CRYPTO_FIPS | bool | This option enables the fips boot option which is required if you want the system to operate in a FIPS 200 certification.  You should say no unless you know what this is. |
| CRYPTO_FIPS_CUSTOM_VERSION | bool | This option provides the ability to override the FIPS Module Version. By default the KERNELRELEASE value is used. |
| CRYPTO_FIPS_NAME | string | This option sets the FIPS Module name reported by the Crypto API via the /proc/sys/crypto/fips_name file. |
| CRYPTO_GCM | tristate | GCM (Galois/Counter Mode) authenticated encryption mode and GMAC (GCM Message Authentication Code) (NIST SP800-38D) This is required for IPSec ESP (XFRM_ESP). |
| CRYPTO_GENIV | tristate | Sequence Number IV generator This IV generator generates an IV based on a sequence number by xoring it with a salt.  This algorithm is mainly useful for CTR. This is required for IPsec ESP (XFRM_ESP). |
| CRYPTO_HCTR2 | tristate | HCTR2 length-preserving encryption mode A mode for storage encryption that is efficient on processors with instructions to accelerate AES and carryless multiplication, e.g. x86 processors with AES-... |
| CRYPTO_HMAC | tristate | HMAC (Keyed-Hash Message Authentication Code) (FIPS 198 and RFC2104) This is required for IPsec AH (XFRM_AH) and IPsec ESP (XFRM_ESP). |
| CRYPTO_JITTERENTROPY_MEMORY_BLOCKS | int | Enable the userspace interface for hash algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_JITTERENTROPY_MEMSIZE_2 | bool | The Jitter RNG allows the specification of an oversampling rate (OSR). The Jitter RNG operation requires a fixed amount of timing measurements to produce one output block of random numbers. The OSR... |
| CRYPTO_JITTERENTROPY_TESTINTERFACE | bool | The test interface allows a privileged process to capture the raw unconditioned high resolution time stamp noise that is collected by the Jitter RNG for statistical analysis. As this data is used a... |
| CRYPTO_KHAZAD | tristate | Khazad cipher algorithm Khazad was a finalist in the initial NESSIE competition.  It is an algorithm optimized for 64-bit processors with good performance on 32-bit processors.  Khazad uses an 128 ... |
| CRYPTO_KRB5ENC | tristate | Combined hash and cipher support for Kerberos 5 RFC3961 simplified profile.  This is required for Kerberos 5-style encryption, used by sunrpc/NFS and rxrpc/AFS. |
| CRYPTO_LRW | tristate | LRW (Liskov Rivest Wagner) mode A tweakable, non malleable, non movable narrow block cipher mode for dm-crypt.  Use it with cipher specification string aes-lrw-benbi, the key must be 256, 320 or 38... |
| CRYPTO_LZ4 | tristate | LZ4 compression algorithm See https://github.com/lz4/lz4 for further information. |
| CRYPTO_LZ4HC | tristate | LZ4 high compression mode algorithm See https://github.com/lz4/lz4 for further information. |
| CRYPTO_LZO | tristate | LZO compression algorithm See https://www.oberhumer.com/opensource/lzo/ for further information. |
| CRYPTO_MANAGER2 | def_tristate | Userspace configuration for cryptographic instantiations such as cbc(aes). |
| CRYPTO_MD4 | tristate | MD4 message digest algorithm (RFC1320) |
| CRYPTO_MD5 | tristate | MD5 message digest algorithm (RFC1321), including HMAC support. |
| CRYPTO_MLDSA | tristate | ML-DSA (Module-Lattice-Based Digital Signature Algorithm) (FIPS-204). Only signature verification is implemented. |
| CRYPTO_NULL | tristate | These are 'Null' algorithms, used by IPsec, which do nothing. |
| CRYPTO_PCBC | tristate | PCBC (Propagating Cipher Block Chaining) mode This block cipher mode is required for RxRPC. |
| CRYPTO_PCRYPT | tristate | This converts an arbitrary crypto algorithm into a parallel algorithm that executes in kernel threads. |
| CRYPTO_RMD160 | tristate | RIPEMD-160 hash function (ISO/IEC 10118-3) RIPEMD-160 is a 160-bit cryptographic hash function. It is intended to be used as a secure replacement for the 128-bit hash functions MD4, MD5 and its pre... |
| CRYPTO_SEED | tristate | SEED cipher algorithm (RFC4269, ISO/IEC 18033-3) SEED is a 128-bit symmetric key block cipher that has been developed by KISA (Korea Information Security Agency) as a national standard encryption a... |
| CRYPTO_SELFTESTS | bool | Enable the cryptographic self-tests. The cryptographic self-tests run at boot time, or at algorithm registration time if algorithms are dynamically loaded later. There are two main use cases for th... |
| CRYPTO_SELFTESTS_FULL | bool | Enable the full set of cryptographic self-tests for each algorithm. The full set of tests should be enabled for development and pre-release testing, but not in production kernels. All crypto code i... |
| CRYPTO_SERPENT | tristate | Serpent cipher algorithm, by Anderson, Biham & Knudsen Keys are allowed to be from 0 to 256 bits in length, in steps of 8 bits. See https://www.cl.cam.ac.uk/~rja14/serpent.html for further informat... |
| CRYPTO_SHA1 | tristate | SHA-1 secure hash algorithm (FIPS 180, ISO/IEC 10118-3), including HMAC support. |
| CRYPTO_SHA256 | tristate | SHA-224 and SHA-256 secure hash algorithms (FIPS 180, ISO/IEC 10118-3), including HMAC support. This is required for IPsec AH (XFRM_AH) and IPsec ESP (XFRM_ESP). |
| CRYPTO_SHA3 | tristate | SHA-3 secure hash algorithms (FIPS 202, ISO/IEC 10118-3) |
| CRYPTO_SHA512 | tristate | SHA-384 and SHA-512 secure hash algorithms (FIPS 180, ISO/IEC 10118-3), including HMAC support. |
| CRYPTO_SIMD | tristate | RSA (Rivest-Shamir-Adleman) public key algorithm (RFC8017) |
| CRYPTO_SM3 | tristate | SM3 (ShangMi 3) secure hash function (OSCCA GM/T 0004-2012, ISO/IEC 10118-3) This is part of the Chinese Commercial Cryptography suite. References: http://www.oscca.gov.cn/UpFile/20101222141857786.... |
| CRYPTO_SM4 | tristate | SM4 cipher algorithms (OSCCA GB/T 32907-2016, ISO/IEC 18033-3:2010/Amd 1:2021) SM4 (GBT.32907-2016) is a cryptographic standard issued by the Organization of State Commercial Administration of Chin... |
| CRYPTO_STREEBOG | tristate | Streebog Hash Function (GOST R 34.11-2012, RFC 6986, ISO/IEC 10118-3) This is one of the Russian cryptographic standard algorithms (called GOST algorithms). This setting enables two hash algorithms... |
| CRYPTO_TEA | tristate | TEA (Tiny Encryption Algorithm) cipher algorithms Tiny Encryption Algorithm is a simple cipher that uses many rounds for security.  It is very fast and uses little memory. Xtendend Tiny Encryption ... |
| CRYPTO_TWOFISH | tristate | Twofish cipher algorithm Twofish was submitted as an AES (Advanced Encryption Standard) candidate cipher by researchers at CounterPane Systems.  It is a 16 round block cipher supporting key sizes o... |
| CRYPTO_TWOFISH_COMMON | tristate | Common parts of the Twofish cipher algorithm shared by the generic c and the assembler implementations. |
| CRYPTO_USER_API_AEAD | tristate | Enable the userspace interface for AEAD cipher algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_ENABLE_OBSOLETE | bool | Allow obsolete cryptographic algorithms to be selected that have already been phased out from internal use by the kernel, and are only useful for userspace clients that still rely on them. |
| CRYPTO_USER_API_RNG | tristate | Enable the userspace interface for RNG (random number generator) algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_RNG_CAVP | bool | Enable extra APIs in the userspace interface for NIST CAVP (Cryptographic Algorithm Validation Program) testing: - resetting DRBG entropy - providing Additional Data This should only be enabled for... |
| CRYPTO_USER_API_SKCIPHER | tristate | Enable the userspace interface for symmetric key cipher algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_WP512 | tristate | Whirlpool hash function (ISO/IEC 10118-3) 512, 384 and 256-bit hashes. Whirlpool-512 is part of the NESSIE cryptographic primitives. See https://web.archive.org/web/20171129084214/http://www.larc.u... |
| CRYPTO_XCBC | tristate | XCBC-MAC (Extended Cipher Block Chaining Message Authentication Code) (RFC3566) |
| CRYPTO_XCTR | tristate | XCTR (XOR Counter) mode for HCTR2 This blockcipher mode is a variant of CTR mode using XORs and little-endian addition rather than big-endian arithmetic. XCTR mode is used to implement HCTR2. |
| CRYPTO_XTS | tristate | XTS (XOR Encrypt XOR with ciphertext stealing) mode (NIST SP800-38E and IEEE 1619) Use with aes-xts-plain, key size 256, 384 or 512 bits. This implementation currently can't handle a sectorsize whi... |
| CRYPTO_XXHASH | tristate | xxHash non-cryptographic hash algorithm Extremely fast, working at speeds close to RAM limits. |
| CRYPTO_ZSTD | tristate | zstd compression algorithm See https://github.com/facebook/zstd for further information. |
| CSD_LOCK_WAIT_DEBUG | bool | This option enables debug prints when CPUs are slow to respond to the smp_call_function*() IPI wrappers.  These debug prints include the IPI handler function currently executing (if any) and releva... |
| CSD_LOCK_WAIT_DEBUG_DEFAULT | bool | This option causes the csdlock_debug= kernel boot parameter to default to 1 (basic debugging) instead of 0 (no debugging). |
| DCACHE_WORD_ACCESS | bool | Enable this to perform validation of the parameter description for a filesystem when it is registered. |
| DEBUG_ATOMIC | bool | If you say Y here then the kernel will add a runtime alignment check to atomic accesses. Useful for architectures that do not have trap on mis-aligned access. This option has potentially significan... |
| DEBUG_ATOMIC_LARGEST_ALIGN | bool | If you say Y here then the check for natural alignment of atomic accesses will be constrained to the compiler's largest alignment for scalar types. |
| DEBUG_ATOMIC_SLEEP | bool | If you say Y here, various routines which may sleep will become very noisy if they are called inside atomic sections: when a spinlock is held, inside an rcu read side critical section, inside preem... |
| DEBUG_BUGVERBOSE | bool | Say Y here to make BUG() panics output the file name and line number of the BUG call as well as the EIP and oops trace.  This aids debugging but costs about 70-100K of memory. |
| DEBUG_BUGVERBOSE_DETAILED | bool | Say Y here to make WARN_ON_ONCE() output the condition string of the warning, in addition to the file name and line number. This helps debugging, but costs about 100K of memory. Say N if unsure. |
| DEBUG_CGROUP_REF | bool | Force cgroup css reference count functions to not be inlined so that they can be kprobed for debugging. |
| DEBUG_CLOSURES | bool | Keeps all active closures in a linked list and provides a debugfs interface to list them, which makes it possible to see asynchronous operations that get stuck. |
| DEBUG_FORCE_FUNCTION_ALIGN_64B | bool | There are cases that a commit from one domain changes the function address alignment of other domains, and cause magic performance bump (regression or improvement). Enable this option will help to ... |
| DEBUG_FORCE_WEAK_PER_CPU | bool | s390 and alpha require percpu variables in modules to be defined weak to work around addressing range issue which puts the following two restrictions on percpu variable definitions. 1. percpu symbo... |
| DEBUG_FS | bool | debugfs is a virtual file system that kernel developers use to put debugging files into.  Enable this option to be able to read and write to these files. For detailed documentation on the debugfs A... |
| DEBUG_FS_ALLOW_ALL | bool | No restrictions apply. Both API and filesystem registration is on. This is the normal default operation. |
| DEBUG_FS_ALLOW_NONE | bool | Access is off. Clients get -PERM when trying to create nodes in debugfs tree and debugfs is not registered as a filesystem. Client can then back-off or continue without debugfs access. |
| DEBUG_HIGHMEM | bool | This option enables additional error checking for high memory systems.  Disable for production systems. |
| DEBUG_INFO | bool | A kernel debug info option other than "None" has been selected in the "Debug information" choice below, indicating that debug information will be generated for build targets. # Clang generates .ule... |
| DEBUG_INFO_BTF | bool | Generate deduplicated BTF type information from DWARF debug info. Turning this on requires pahole v1.22 or later, which will convert DWARF type info into equivalent deduplicated BTF type info. |
| DEBUG_INFO_BTF_MODULES | bool | Generate compact split BTF type information for kernel modules. |
| DEBUG_INFO_COMPRESSED_NONE | bool | Don't compress debug info sections. |
| DEBUG_INFO_COMPRESSED_ZLIB | bool | Compress the debug information using zlib. Users of dpkg-deb via debian/rules may find an increase in size of their debug .deb packages with this config set, due to the debug info being compressed ... |
| DEBUG_INFO_COMPRESSED_ZSTD | bool | Compress the debug information using zstd.  This may provide better compression than zlib, for about the same time costs, but requires newer toolchain support.  Requires GCC 13.0+ or Clang 16.0+, b... |
| DEBUG_INFO_DWARF4 | bool | Generate DWARF v4 debug info. This requires gcc 4.5+, binutils 2.35.2 if using clang without clang's integrated assembler, and gdb 7.0+. If you have consumers of DWARF debug info that are not ready... |
| DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT | bool | The implicit default version of DWARF debug info produced by a toolchain changes over time. This can break consumers of the debug info that haven't upgraded to support newer revisions, and prevent ... |
| DEBUG_INFO_NONE | bool | Do not build the kernel with debugging information, which will result in a faster and smaller build. |
| DEBUG_INFO_SPLIT | bool | Generate debug info into separate .dwo files. This significantly reduces the build directory size for builds with DEBUG_INFO, because it stores the information only once on disk in .dwo files inste... |
| DEBUG_IRQFLAGS | bool | Enables checks for potentially unsafe enabling or disabling of interrupts, such as calling raw_local_irq_restore() when interrupts are enabled. |
| DEBUG_KERNEL | bool | Say Y here if you are developing drivers or trying to debug and identify kernel problems. |
| DEBUG_KMAP_LOCAL | bool | This option enables additional error checking for the kmap_local infrastructure.  Disable for production use. |
| DEBUG_KOBJECT | bool | If you say Y here, some extra kobject debugging messages will be sent to the syslog. |
| DEBUG_KOBJECT_RELEASE | bool | kobjects are reference counted objects.  This means that their last reference count put is not predictable, and the kobject can live on past the point at which a driver decides to drop its initial ... |
| DEBUG_LOCKDEP | bool | If you say Y here, the lock dependency engine will do additional runtime checks to debug itself, at the price of more runtime overhead. |
| DEBUG_LOCKING_API_SELFTESTS | bool | Say Y here if you want the kernel to run a short self-test during bootup. The self-test checks whether common types of locking bugs are detected by debugging mechanisms or not. (if you disable lock... |
| DEBUG_LOCK_ALLOC | bool | This feature will check whether any held lock (spinlock, rwlock, mutex or rwsem) is incorrectly freed by the kernel, via any of the memory-freeing routines (kfree(), kmem_cache_free(), free_pages()... |
| DEBUG_MAPLE_TREE | bool | Enable maple tree debugging information and extra validations. If unsure, say N. |
| DEBUG_MEMORY_INIT | bool | Enable this for additional checks during memory initialisation. The sanity checks verify aspects of the VM such as the memory model and other information provided by the architecture. Verbose infor... |
| DEBUG_MISC | bool | Say Y here if you need to enable miscellaneous debug code that should be under a more specific debug option but isn't. |
| DEBUG_MUTEXES | bool | This feature allows mutex semantics violations to be detected and reported. |
| DEBUG_NOMMU_REGIONS | bool | This option causes the global tree of anonymous and private mapping regions to be regularly checked for invalid topology. |
| DEBUG_NOTIFIERS | bool | Enable this to turn on sanity checking for notifier call chains. This is most useful for kernel developers to make sure that modules properly unregister themselves from notifier chains. This is a r... |
| DEBUG_OBJECTS | bool | If you say Y here, additional code will be inserted into the kernel to track the life time of various objects and validate the operations on those objects. |
| DEBUG_OBJECTS_ENABLE_DEFAULT | int | Debug objects boot parameter default value |
| DEBUG_OBJECTS_FREE | bool | This enables checks whether a k/v free operation frees an area which contains an object which has not been deactivated properly. This can make kmalloc/kfree-intensive workloads much slower. |
| DEBUG_OBJECTS_PERCPU_COUNTER | bool | If you say Y here, additional code will be inserted into the percpu counter routines to track the life time of percpu counter objects and validate the percpu counter operations. |
| DEBUG_OBJECTS_RCU_HEAD | bool | Enable this to turn on debugging of RCU list heads (call_rcu() usage). |
| DEBUG_OBJECTS_SELFTEST | bool | This enables the selftest of the object debug code. |
| DEBUG_OBJECTS_TIMERS | bool | If you say Y here, additional code will be inserted into the timer routines to track the life time of timer objects and validate the timer operations. |
| DEBUG_OBJECTS_WORK | bool | If you say Y here, additional code will be inserted into the work queue routines to track the life time of work objects and validate the work operations. |
| DEBUG_PERF_USE_VMALLOC | bool | Use vmalloc memory to back perf mmap() buffers. Mostly useful for debugging the vmalloc code on platforms that don't require it. Say N if unsure. |
| DEBUG_PER_CPU_MAPS | bool | Say Y to verify that the per_cpu map being accessed has been set up. This adds a fair amount of code to kernel memory and decreases performance. Say N if unsure. |
| DEBUG_PLIST | bool | Enable this to turn on extended checks in the priority-ordered linked-list (plist) walking routines.  This checks the entire list multiple times during each manipulation. If unsure, say N. |
| DEBUG_PREEMPT | bool | If you say Y here then the kernel will use a debug variant of the commonly used smp_processor_id() function and will print warnings if kernel code uses it in a preemption-unsafe way. Also, the kern... |
| DEBUG_RSEQ | bool | Enable extra debugging checks for the rseq system call. If unsure, say N. |
| DEBUG_RT_MUTEXES | bool | This allows rt mutex semantics violations and rt mutex related deadlocks (lockups) to be detected and reported automatically. |
| DEBUG_RWSEMS | bool | This debugging feature allows mismatched rw semaphore locks and unlocks to be detected and reported. |
| DEBUG_SECTION_MISMATCH | bool | The section mismatch analysis checks if there are illegal references from one section to another. During linktime or runtime, some sections are dropped; any use of code/data previously in these sec... |
| DEBUG_SG | bool | Enable this to turn on checks on scatter-gather tables. This can help find problems with drivers that do not properly initialize their sg tables. If unsure, say N. |
| DEBUG_SHIRQ | bool | Enable this to generate a spurious interrupt just before a shared interrupt handler is deregistered (generating one when registering is currently disabled). Drivers need to handle this correctly. S... |
| DEBUG_SPINLOCK | bool | Say Y here and build SMP to catch missing spinlock initialization and certain other kinds of spinlock errors commonly made.  This is best used in conjunction with the NMI watchdog so that spinlock ... |
| DEBUG_STACK_USAGE | bool | Enables the display of the minimum amount of free stack which each task has ever had available in the sysrq-T and sysrq-P debug output. Also emits a message to dmesg when a process exits if that pr... |
| DEBUG_VFS | bool | Enable this to turn on extended checks in the VFS layer that may impact performance. If unsure, say N. |
| DEBUG_VM_IRQSOFF | def_bool | Enable this to turn on extended checks in the virtual-memory system that may impact performance. If unsure, say N. |
| DEBUG_VM_MAPLE_TREE | bool | Enable VM maple tree debugging information and extra validations. If unsure, say N. |
| DEBUG_VM_PGFLAGS | bool | Enables extra validation on page flags operations. If unsure, say N. |
| DEBUG_VM_PGTABLE | bool | This option provides a debug method which can be used to test architecture page table helper functions on various platforms in verifying if they comply with expected generic MM semantics. This will... |
| DEBUG_VM_RB | bool | Enable VM red-black tree debugging information and extra validations. If unsure, say N. |
| DEBUG_VM_SHOOT_LAZIES | bool | Enable additional IPIs that ensure lazy tlb mm references are removed before the mm is freed. If unsure, say N. |
| DEBUG_WQ_FORCE_RR_CPU | bool | Workqueue used to implicitly guarantee that work items queued without explicit CPU specified are put on the local CPU.  This guarantee is no longer true and while local CPU is still preferred work ... |
| DEBUG_WW_MUTEX_SLOWPATH | bool | This feature enables slowpath testing for w/w mutex users by injecting additional -EDEADLK wound/backoff cases. Together with the full mutex checks enabled with (CONFIG_PROVE_LOCKING) this will tes... |
| DEFAULT_HOSTNAME | string | This option determines the default system hostname before userspace calls sethostname(2). The kernel traditionally uses "(none)" here, but you may wish to use a different default here to make a min... |
| DEFAULT_HUNG_TASK_TIMEOUT | int | This option controls the default timeout (in seconds) used to determine when a task has become non-responsive and should be considered hung. It can be adjusted at runtime via the kernel.hung_task_t... |
| DEFAULT_INIT | string | This option determines the default init for the system if no init= option is passed on the kernel command line. If the requested path is not present, we will still then move on to attempting furthe... |
| DEFAULT_MMAP_MIN_ADDR | int | This is the portion of low virtual memory which should be protected from userspace allocation.  Keeping a user from writing to low pages can help reduce the impact of kernel NULL pointer bugs. For ... |
| DEFAULT_SECURITY_SELINUX | bool | A comma-separated list of LSMs, in initialization order. Any LSMs left off this list, except for those with order LSM_ORDER_FIRST and LSM_ORDER_LAST, which are always enabled if selected in the ker... |
| DEFERRED_STRUCT_PAGE_INIT | bool | Ordinarily all struct pages are initialised during early boot in a single thread. On very large machines this can take a considerable amount of time. If this option is set, large machines will brin... |
| DETECT_HUNG_TASK | bool | Say Y here to enable the kernel to detect "hung tasks", which are bugs that cause the task to be stuck in uninterruptible "D" state indefinitely. When a hung task is detected, the kernel will print... |
| DETECT_HUNG_TASK_BLOCKER | bool | Say Y here to show the blocker task's stacktrace who acquires the mutex lock which "hung tasks" are waiting. This will add overhead a bit but shows suspicious tasks and call trace if it comes from ... |
| DIMLIB | tristate | Dynamic Interrupt Moderation library. Implements an algorithm for dynamically changing CQ moderation values according to run time performance. # # libfdt files, only selected if needed. # |
| DST_CACHE | bool | The NET_SOCK_MSG provides a framework for plain sockets (e.g. TCP) or ULPs (upper layer modules, e.g. TLS) to process L7 application data with the help of BPF programs. |
| DYNAMIC_DEBUG | bool | Compiles debug level messages into the kernel, which would not otherwise be available at runtime. These messages can then be enabled/disabled based on various levels of scope - per source file, fun... |
| DYNAMIC_DEBUG_CORE | bool | Enable core functional support of dynamic debug. It is useful when you want to tie dynamic debug to your kernel modules with DYNAMIC_DEBUG_MODULE defined for each of them, especially for the case o... |
| ELFCORE | bool | This option enables kernel/elfcore.o. |
| ELF_CORE | bool | Enable support for generating core dumps. Disabling saves about 4k. |
| ETHTOOL_NETLINK | bool | An alternative userspace interface for ethtool based on generic netlink. It provides better extensibility and some new features, e.g. notification messages. |
| EVENTFD | bool | Enable the eventfd() system call that allows to receive both kernel notification (ie. KAIO) or userspace notifications. If unsure, say Y. |
| EXEC_KUNIT_TEST | bool | This builds the exec KUnit tests, which tests boundary conditions of various aspects of the exec internals. |
| EXT_GROUP_SCHED | bool | This feature enables the scheduler to track the clamped utilization of each CPU based on RUNNABLE tasks currently scheduled on that CPU. When this option is enabled, the user can specify a min and ... |
| FAILOVER | tristate | The failover module provides a generic interface for paravirtual drivers to register a netdev and a set of ops with a failover instance. The ops are used as event handlers that get called to handle... |
| FAILSLAB | bool | Provide fault-injection capability for kmalloc. |
| FAIL_FUNCTION | bool | Provide function-based fault-injection capability. This will allow you to override a specific function with a return with given return value. As a result, function caller will see an error value an... |
| FAIL_FUTEX | bool | Provide fault-injection capability for futexes. |
| FAIL_IO_TIMEOUT | bool | Provide fault-injection capability on end IO handling. This will make the block layer "forget" an interrupt as configured, thus exercising the error handling. Only works with drivers that use the g... |
| FAIL_MAKE_REQUEST | bool | Provide fault-injection capability for disk IO. |
| FAIL_MMC_REQUEST | bool | Provide fault-injection capability for MMC IO. This will make the mmc core return data errors. This is useful to test the error handling in the mmc block device and to test how the mmc host driver ... |
| FAIL_PAGE_ALLOC | bool | Provide fault-injection capability for alloc_pages(). |
| FAIL_SKB_REALLOC | bool | Provide fault-injection capability that forces the skb to be reallocated, catching possible invalid pointers to the skb. For more information, check Documentation/fault-injection/fault-injection.rst |
| FAIL_SUNRPC | bool | Provide fault-injection capability for SunRPC and its consumers. |
| FAULT_INJECTION | bool | Provide fault-injection framework. For more details, see Documentation/fault-injection/. |
| FAULT_INJECTION_CONFIGFS | bool | This option allows configfs-based drivers to dynamically configure fault-injection via configfs.  Each parameter for driver-specific fault-injection can be made visible as a configfs attribute in a... |
| FAULT_INJECTION_DEBUG_FS | bool | Enable configuration of fault-injection capabilities via debugfs. |
| FAULT_INJECTION_STACKTRACE_FILTER | bool | Provide stacktrace filter for fault-injection capabilities |
| FAULT_INJECTION_USERCOPY | bool | Provides fault-injection capability to inject failures in usercopy functions (copy_from_user(), get_user(), ...). |
| FFS_KUNIT_TEST | tristate | This builds KUnit tests for ffs-family bit manipulation functions including ffs(), __ffs(), fls(), __fls(), fls64(), and __ffs64(). These tests validate mathematical correctness, edge case handling... |
| FHANDLE | bool | If you say Y here, a user level program will be able to map file names to handle and then later use the handle for different file system operations. This is useful in implementing userspace file se... |
| FIB_RULES | bool | This feature provides an infrastructure to support light weight tunnels like mpls. There is no netdevice associated with a light weight tunnel endpoint. Tunnel encapsulation parameters are stored w... |
| FILE_LOCKING | bool | This option enables standard file locking support, required for filesystems like NFS and for the flock() system call. Disabling this option saves about 11k. |
| FIND_BIT_BENCHMARK | tristate | This builds the "test_find_bit" module that measure find_*_bit() functions performance. If unsure, say N. |
| FIND_BIT_BENCHMARK_RUST | tristate | This builds the "find_bit_benchmark_rust" module. It is a micro benchmark that measures the performance of Rust functions that correspond to the find_*_bit() operations in C. It follows the FIND_BI... |
| FIND_NORMAL_PAGE | def_bool | The architecture uses the lazy MMU mode. This allows changes to MMU-related architectural state to be deferred until the mode is exited. See <linux/pgtable.h> for details. |
| FLATMEM_MANUAL | bool | This option is best suited for non-NUMA systems with flat address space. The FLATMEM is the most efficient system in terms of performance and resource consumption and it is the best option for smal... |
| FORCE_NR_CPUS | def_bool | This option provides a glob_match function for performing simple text pattern matching.  It originated in the ATA code to blacklist particular drive models, but other 设备驱动程序 may need simila... |
| FORTIFY_KUNIT_TEST | tristate | Builds unit tests for checking internals of FORTIFY_SOURCE as used by the str*() and mem*() family of functions. For testing runtime traps of FORTIFY_SOURCE, see LKDTM's "FORTIFY_*" tests. |
| FPROBE_SANITY_TEST | bool | This option will enable testing the fprobe when the system boot. A series of tests are made to verify that the fprobe is functioning properly. Say N if you are unsure. |
| FRAME_WARN | int | Tell the compiler to warn at build time for stack frames larger than this. Setting this too low will cause a lot of warnings. Setting it to 0 disables the warning. |
| FREEZER | def_bool |  |
| FS_DAX_PMD | bool | This option enables the export operations for a filesystem to support external block IO. |
| FS_IOMAP | bool | Direct Access (DAX) can be used on memory-backed block devices. If the block device supports DAX and the filesystem supports DAX, then you can avoid using the pagecache to buffer I/Os.  Turning on ... |
| FUNCTION_ERROR_INJECTION | bool | Add fault injections into various functions that are annotated with ALLOW_ERROR_INJECTION() in the kernel. BPF may also modify the return value of these functions. This is useful to test error path... |
| FUTEX | bool | Disabling this option will cause the kernel to be built without support for "fast userspace mutexes".  The resulting kernel may not run glibc-based applications correctly. |
| FUTEX_PI | bool | Disabling this option will cause the kernel to be built without support for epoll family of system calls. |
| GCD_KUNIT_TEST | tristate | This option enables the KUnit test suite for the gcd() function, which computes the greatest common divisor of two numbers. This test suite verifies the correctness of gcd() across various scenario... |
| GCOV_PROFILE_URING | bool | Enable GCOV profiling on the io_uring subsystem, to facilitate code coverage testing. If unsure, say N. Note that this will have a negative impact on the performance of the io_uring subsystem, henc... |
| GDB_SCRIPTS | bool | This creates the required links to GDB helper scripts in the build directory. If you load vmlinux into gdb, the helper scripts will be automatically imported by gdb as well, and additional function... |
| GENERIC_EARLY_IOREMAP | bool | This is the maximum stack size in Megabytes in the VM layout of 32-bit user processes when the stack grows upwards (currently only on parisc arch) when the RLIMIT_STACK hard limit is unlimited. A s... |
| GENERIC_IOREMAP | bool |  |
| GLOB_KUNIT_TEST | tristate | Enable this option to test the glob functions at runtime. This test suite verifies the correctness of glob_match() across various scenarios, including edge cases. If unsure, say N |
| GRACE_PERIOD | tristate | Some NFS servers support an auxiliary NFS LOCALIO protocol that is not an official part of the NFS protocol. This option enables support for the LOCALIO protocol in the kernel's NFS server and clie... |
| GROUP_SCHED_WEIGHT | def_bool | This option allows users to define CPU bandwidth rates (limits) for tasks running within the fair group scheduler.  Groups with no limit set are considered to be unconstrained and will run with no ... |
| GUEST_PERF_EVENTS | bool | See tools/perf/design.txt for details |
| GUP_GET_PXX_LOW_HIGH | bool | Provides a test module that will allocate and free many blocks of various sizes and report how long it takes. This is intended to provide a consistent way to measure how changes to the dma_pool_all... |
| GUP_TEST | bool | Provides /sys/kernel/debug/gup_test, which in turn provides a way to make ioctl calls that can launch kernel-based unit tests for the get_user_pages*() and pin_user_pages*() family of API calls. Th... |
| HARDLOCKUP_DETECTOR_COUNTS_HRTIMER | bool | Say Y here to enable the kernel to panic on "hard lockups", which are bugs that cause the kernel to loop in kernel mode with interrupts disabled for more than 10 seconds (configurable using the wat... |
| HARDLOCKUP_DETECTOR_PERF | bool | The arch-specific implementation of the hardlockup detector will be used. # # Both the "perf" and "buddy" hardlockup detectors count hrtimer # interrupts. This config enables functions managing thi... |
| HARDLOCKUP_DETECTOR_PREFER_BUDDY | bool | Say Y here to prefer the buddy hardlockup detector over the perf one. With the buddy detector, each CPU uses its softlockup hrtimer to check that the next CPU is processing hrtimer interrupts by ve... |
| HASHTABLE_KUNIT_TEST | tristate | This builds the hashtable KUnit test suite. It tests the basic functionality of the API defined in include/linux/hashtable.h. For more information on KUnit and unit tests in general please refer to... |
| HASH_KUNIT_TEST | tristate | Enable this option to test the kernel's string (<linux/stringhash.h>), and integer (<linux/hash.h>) hash functions on boot. KUnit tests run during boot and output the results to the debug log in TA... |
| HAS_SECURITY_AUDIT | def_bool | This will build the securityfs filesystem.  It is currently used by various security modules (AppArmor, IMA, SafeSetID, TOMOYO, TPM). If you are unsure how to answer this question, answer N. |
| HAVE_ARCH_AUDITSYSCALL | bool | This is the basic tick based cputime accounting that maintains statistics about user, system and idle time spent on per jiffies granularity. If unsure, say Y. |
| HAVE_ARCH_TLB_REMOVE_TABLE | def_bool | Try to reclaim empty user page table pages in paths other than munmap and exit_mmap path. Note: now only empty user PTE page table pages will be reclaimed. |
| HAVE_ARCH_USERFAULTFD_MINOR | bool | Arch has userfaultfd minor fault support |
| HAVE_ARCH_USERFAULTFD_WP | bool | Arch has userfaultfd write protection support |
| HAVE_DEBUG_BUGVERBOSE | bool | Enable this to turn on extended checks in the linked-list walking routines. This option trades better quality error reports for performance, and is more suitable for kernel debugging. If you care a... |
| HAVE_DEBUG_STACKOVERFLOW | bool | Say Y here if you want to check for overflows of kernel, IRQ and exception stacks (if your architecture uses them). This option will show detailed messages if free stack space drops below a certain... |
| HAVE_HARDLOCKUP_DETECTOR_BUDDY | bool | Say Y here to enable the kernel to act as a watchdog to detect hard lockups. Hardlockups are bugs that cause the CPU to loop in kernel mode for more than 10 seconds, without letting other interrupt... |
| HAVE_KERNEL_GZIP | bool | The linux kernel is a kind of self-extracting executable. Several compression algorithms are available, which differ in efficiency, compression and decompression speed. Compression speed is only re... |
| HAVE_LD_DEAD_CODE_DATA_ELIMINATION | bool | This requires that the arch annotates or otherwise protects its external entry points from being discarded. Linker scripts must also merge .text.*, .data.*, and .bss.* correctly into output section... |
| HAVE_PCSPKR_PLATFORM | bool | This option allows certain base kernel options and settings to be disabled or tweaked. This is for specialized environments which can tolerate a "non-standard" kernel. Only use this if you really k... |
| HAVE_PERF_EVENTS | bool | See tools/perf/design.txt for details. |
| HAVE_SCHED_AVG_IRQ | def_bool | Select this option to enable HW pressure accounting in the scheduler. HW pressure is the value conveyed to the scheduler that reflects the reduction in CPU compute capacity resulted from HW throttl... |
| HAVE_UNSTABLE_SCHED_CLOCK | bool | This feature enables the scheduler to track the clamped utilization of each CPU based on RUNNABLE tasks scheduled on that CPU. With this option, the user can specify the min and max CPU utilization... |
| HEADERS_INSTALL | bool | This option will install uapi headers (headers exported to user-space) into the usr/include directory for use during the kernel build. This is unneeded for building the kernel itself, but needed fo... |
| HMM_MIRROR | bool | Allows creation of struct pages to represent unaddressable device memory; i.e., memory that is only accessible from the device (or group of devices). You likely also want to select HMM_MIRROR. |
| HUGETLB_PAGE | def_bool | Say Y here to get to see options for various miscellaneous filesystems, such as filesystems that came from other operating systems. This option alone does not add any kernel code. If you say N, all... |
| HUGETLB_PAGE_OPTIMIZE_VMEMMAP_DEFAULT_ON | bool | The HugeTLB Vmemmap Optimization (HVO) defaults to off. Say Y here to enable HVO by default. It can be disabled via hugetlb_free_vmemmap=off (boot command line) or hugetlb_optimize_vmemmap (sysctl). |
| HWPOISON_INJECT | tristate | The NOMMU mmap() frequently needs to allocate large contiguous chunks of memory on which to store mappings, but it can only ask the system allocator for chunks in 2^N*PAGE_SIZE amounts - which is f... |
| HW_BREAKPOINT_KUNIT_TEST | bool | Tests for hw_breakpoint constraints accounting. If unsure, say N. |
| HYPERV_TESTING | bool | Select this option to enable Hyper-V vmbus testing. |
| IDLE_PAGE_TRACKING | bool | This feature allows to estimate the amount of user pages that have not been touched during a given period of time. This information can be useful to tune memory cgroup limits and/or for job placeme... |
| IKCONFIG | tristate | This option enables the complete Linux kernel ".config" file contents to be saved in the kernel. It provides documentation of which kernel options are used in a running kernel or in an on-disk kern... |
| IKCONFIG_PROC | bool | This option enables access to the kernel configuration file through /proc/config.gz. |
| IKHEADERS | tristate | This option enables access to the in-kernel headers that are generated during the build process. These can be used to build eBPF tracing programs, or similar programs.  If you build the headers as ... |
| INDIRECT_IOMEM | bool | This is selected by other options/architectures to provide the emulated iomem accessors. |
| INDIRECT_IOMEM_FALLBACK | bool | If INDIRECT_IOMEM is selected, this enables falling back to plain mmio accesses when the IO memory address is not a registered emulated region. |
| INET | bool | These are the protocols used on the Internet and on most local Ethernets. It is highly recommended to say Y here (this will enlarge your kernel by about 400 KB), since some programs (e.g. the X win... |
| INITRAMFS_PRESERVE_MTIME | bool | Each entry in an initramfs cpio archive carries an mtime value. When enabled, extracted cpio items take this mtime, with directory mtime setting deferred until after creation of any child entries. ... |
| INITRAMFS_TEST | bool | Build KUnit tests for initramfs. See Documentation/dev-tools/kunit |
| INTEL_TXT | bool | This option enables support for booting the kernel with the Trusted Boot (tboot) module. This will utilize Intel(R) Trusted Execution Technology to perform a measured launch of the kernel. If the s... |
| INTERVAL_TREE_SPAN_ITER | bool | Support entries which occupy multiple consecutive indices in the XArray. |
| INTERVAL_TREE_TEST | tristate | A benchmark measuring the performance of the interval tree library |
| INT_LOG_KUNIT_TEST | tristate | This option enables the KUnit test suite for the int_log library, which provides two functions to compute the integer logarithm in base 2 and base 10, called respectively as intlog2 and intlog10. I... |
| INT_POW_KUNIT_TEST | tristate | This option enables the KUnit test suite for the int_pow function, which performs integer exponentiation. The test suite is designed to verify that the implementation of int_pow correctly computes ... |
| INT_SQRT_KUNIT_TEST | tristate | This option enables the KUnit test suite for the int_sqrt() function, which performs square root calculation. The test suite checks various scenarios, including edge cases, to ensure correctness. E... |
| IO_STRICT_DEVMEM | bool | If this option is disabled, you allow userspace (root) access to all io-memory regardless of whether a driver is actively using that range.  Accidental access to this is obviously disastrous, but s... |
| IO_URING | bool | This option enables support for the io_uring interface, enabling applications to submit and complete IO through submission and completion rings that are shared between the kernel and application. |
| IO_URING_MOCK_FILE | tristate | Enable mock files for io_uring subsystem testing. The ABI might still change, so it's still experimental and should only be enabled for specific test purposes. If unsure, say N. |
| IO_URING_ZCRX | def_bool |  |
| IRQ_TIME_ACCOUNTING | bool | Select this option to enable fine granularity task irq time accounting. This is done by reading a timestamp on each transitions between softirq and hardirq state, so there can be a small performanc... |
| IS_SIGNED_TYPE_KUNIT_TEST | tristate | Builds unit tests for the is_signed_type() macro. For more information on KUnit and unit tests in general please refer to the KUnit documentation in Documentation/dev-tools/kunit/. If unsure, say N. |
| KALLSYMS | bool | Say Y here to let the kernel print out symbolic crash information and symbolic stack backtraces. This increases the size of the kernel somewhat, as all symbols have to be loaded into the kernel image. |
| KALLSYMS_ALL | bool | Normally kallsyms only contains the symbols of functions for nicer OOPS messages and backtraces (i.e., symbols from the text and inittext sections). This is sufficient for most cases. And only if y... |
| KALLSYMS_SELFTEST | bool | Test the basic functions and performance of some interfaces, such as kallsyms_lookup_name. It also calculates the compression rate of the kallsyms compression algorithm for the current symbol set. ... |
| KCMP | bool | Enable the kernel resource comparison system call. It provides user-space with the ability to compare two processes to see if they share a common resource, such as a file descriptor or even virtual... |
| KCOV | bool | KCOV exposes kernel code coverage information in a form suitable for coverage-guided fuzzing (randomized testing). For more details, see Documentation/dev-tools/kcov.rst. |
| KCOV_ENABLE_COMPARISONS | bool | KCOV also exposes operands of every comparison in the instrumented code along with operand sizes and PCs of the comparison instructions. These operands can be used by fuzzing engines to improve the... |
| KCOV_INSTRUMENT_ALL | bool | If you are doing generic system call fuzzing (like e.g. syzkaller), then you will want to instrument the whole kernel and you should say y here. If you are doing more targeted fuzzing (like e.g. fi... |
| KCOV_IRQ_AREA_SIZE | hex | KCOV uses preallocated per-cpu areas to collect coverage from soft interrupts. This specifies the size of those areas in the number of unsigned long words. |
| KCOV_SELFTEST | bool | Run short KCOV coverage collection selftests on boot. On test failure, causes the kernel to panic. Recommended to be enabled, ensuring critical functionality works as intended. |
| KERNEL_BZIP2 | bool | Its compression ratio and speed is intermediate. Decompression speed is slowest among the choices.  The kernel size is about 10% smaller with bzip2, in comparison to gzip. Bzip2 uses a large amount... |
| KERNEL_GZIP | bool | The old and tried gzip compression. It provides a good balance between compression ratio and decompression speed. |
| KERNEL_LZ4 | bool | LZ4 is an LZ77-type compressor with a fixed, byte-oriented encoding. A preliminary version of LZ4 de/compression tool is available at <https://code.google.com/p/lz4/>. Its compression ratio is wors... |
| KERNEL_LZMA | bool | This compression algorithm's ratio is best.  Decompression speed is between gzip and bzip2.  Compression is slowest. The kernel size is about 33% smaller with LZMA in comparison to gzip. |
| KERNEL_LZO | bool | Its compression ratio is the poorest among the choices. The kernel size is about 10% bigger than gzip; however its speed (both compression and decompression) is the fastest. |
| KERNEL_UNCOMPRESSED | bool | Produce uncompressed kernel image. This option is usually not what you want. It is useful for debugging the kernel in slow simulation environments, where decompressing and moving the kernel is awfu... |
| KERNEL_XZ | bool | XZ uses the LZMA2 algorithm and instruction set specific BCJ filters which can improve compression ratio of executable code. The size of the kernel is about 30% smaller with XZ in comparison to gzi... |
| KERNEL_ZSTD | bool | ZSTD is a compression algorithm targeting intermediate compression with fast decompression speed. It will compress better than GZIP and decompress around the same speed as LZO, but slower than LZ4.... |
| KFIFO_KUNIT_TEST | tristate | This builds the generic FIFO implementation KUnit test suite. It tests that the API and basic functionality of the kfifo type and associated macros. For more information on KUnit and unit tests in ... |
| KPROBES_SANITY_TEST | tristate | This option provides for testing basic kprobes functionality on boot. Samples of kprobe and kretprobe are inserted and verified for functionality. Say N if you are unsure. |
| LATENCYTOP | bool | Enable this option if you want to use the LatencyTOP tool to find out which userspace is blocking on what kernel operations. |
| LAZY_MMU_MODE_KUNIT_TEST | tristate | Enable this option to check that the lazy MMU mode interface behaves as expected. Only tests for the generic interface are included (not architecture-specific behaviours). If unsure, say N. |
| LD_DEAD_CODE_DATA_ELIMINATION | bool | Enable this if you want to do dead code and data elimination with the linker by compiling with -ffunction-sections -fdata-sections, and linking with --gc-sections. This can reduce on disk and in-me... |
| LD_ORPHAN_WARN | def_bool | Enable support for /proc/sys/debug/exception-trace. |
| LIBFDT | bool | Enable fast lookup object identifier registry. |
| LINEAR_RANGES | tristate | This option provides the packing() helper function, which permits converting bitfields between a CPU-usable representation and a memory representation that can have any combination of these quirks:... |
| LINEAR_RANGES_TEST | tristate | This builds the linear_ranges unit test, which runs on boot. Tests the linear_ranges logic correctness. For more information on KUnit and unit tests in general please refer to the KUnit documentati... |
| LIST_KUNIT_TEST | tristate | This builds the linked list KUnit test suite. It tests that the API and basic functionality of the list_head type and associated macros. KUnit tests run during boot and output the results to the de... |
| LIST_PRIVATE_KUNIT_TEST | tristate | This builds the KUnit test for the private linked-list primitives defined in include/linux/list_private.h. These primitives allow manipulation of list_head members that are marked as private and re... |
| LIVEUPDATE_TEST | bool | Enable a built-in kernel test module for the Live Update Orchestrator. This module validates the File-Lifecycle-Bound subsystem by registering a set of mock FLB objects with any real file handlers ... |
| LKDTM | tristate | This module enables testing of the different dumping mechanisms by inducing system failures at predefined crash points. If you don't need it: say N Choose M here to compile this code as a module. T... |
| LOCALVERSION | string | Append an extra string to the end of your kernel version. This will show up when you type uname, for example. The string you set here will be appended after the contents of any files with a filenam... |
| LOCALVERSION_AUTO | bool | This will try to automatically determine if the current tree is a release tree by looking for git tags that belong to the current top of tree revision. A string of the format -gxxxxxxxx will be add... |
| LOCKDEP | bool | Try increasing this value if you hit "BUG: MAX_LOCKDEP_ENTRIES too low!" message. |
| LOCKDEP_CHAINS_BITS | int | Try increasing this value if you hit "BUG: MAX_LOCKDEP_CHAINS too low!" message. |
| LOCKDEP_CIRCULAR_QUEUE_BITS | int | Try increasing this value if you hit "lockdep bfs error:-1" warning due to __cq_enqueue() failure. |
| LOCKDEP_STACK_TRACE_BITS | int | Try increasing this value if you hit "BUG: MAX_STACK_TRACE_ENTRIES too low!" message. KASAN significantly increases stack trace consumption because its slab tracking interacts with lockdep's depend... |
| LOCKDEP_STACK_TRACE_HASH_BITS | int | Try increasing this value if you need large STACK_TRACE_HASH_SIZE. |
| LOCKUP_DETECTOR | bool | Say Y here to enable the kernel to act as a watchdog to detect soft lockups. Softlockups are bugs that cause the kernel to loop in kernel mode for more than 20 seconds, without giving other tasks a... |
| LOCK_DEBUGGING_SUPPORT | bool | This feature enables the kernel to prove that all locking that occurs in the kernel runtime is mathematically correct: that under no circumstance could an arbitrary (and not yet triggered) combinat... |
| LOCK_MM_AND_FIND_VMA | bool | Enable NUMA emulation. A flat machine will be split into virtual nodes when booted with "numa=fake=N", where N is the number of nodes. This is only useful for debugging. |
| LOCK_STAT | bool | This feature enables tracking lock contention points For more details, see Documentation/locking/lockstat.rst This also enables lock events required by "perf lock", subcommand of perf. If you want ... |
| LOCK_TORTURE_TEST | tristate | This option provides a kernel module that runs torture tests on kernel locking primitives.  The kernel module may be built after the fact on the running kernel to be tested, if desired. Say Y here ... |
| LOG_BUF_SHIFT | int | Select the minimal kernel log buffer size as a power of 2. The final size is affected by LOG_CPU_MAX_BUF_SHIFT config parameter, see below. Any higher size also might be forced by "log_buf_len" boo... |
| LOG_CPU_MAX_BUF_SHIFT | int | This option allows to increase the default ring buffer size according to the number of CPUs. The value defines the contribution of each CPU as a power of 2. The used space is typically only few lin... |
| LONGEST_SYM_KUNIT_TEST | tristate | Tests the longest symbol possible If unsure, say N. |
| LRU_GEN | bool | A high performance LRU implementation to overcommit memory. See Documentation/admin-guide/mm/multigen_lru.rst for details. |
| LRU_GEN_ENABLED | bool | This option enables the multi-gen LRU by default. |
| LRU_GEN_STATS | bool | Do not enable this option unless you plan to look at historical stats from evicted generations for debugging purpose. This option has a per-memcg and per-node memory overhead. |
| LRU_GEN_WALKS_MMU | def_bool | Allow per-vma locking during page fault handling. This feature allows locking each virtual memory area separately when handling page faults instead of taking mmap_lock. |
| LSM_MMAP_MIN_ADDR | int | This is the portion of low virtual memory which should be protected from userspace allocation.  Keeping a user from writing to low pages can help reduce the impact of kernel NULL pointer bugs. For ... |
| LWTUNNEL_BPF | bool | Allows to run BPF programs as a nexthop action following a route lookup for incoming and outgoing packets. |
| LZO_COMPRESS | tristate | Drivers may select this option to force specific constant values for parameters 'm' (Galois field order) and 't' (error correction capability). Those specific values must be set by declaring defaul... |
| MAGIC_SYSRQ | bool | If you say Y here, you will have some control over the system even if the system crashes for example during kernel debugging (e.g., you will be able to flush the buffer cache to disk, reboot the sy... |
| MAGIC_SYSRQ_DEFAULT_ENABLE | hex | Specifies which SysRq key functions are enabled by default. This may be set to 1 or 0 to enable or disable them all, or to a bitmask as described in Documentation/admin-guide/sysrq.rst. |
| MAGIC_SYSRQ_SERIAL | bool | Many embedded boards have a disconnected TTL level serial which can generate some garbage that can lead to spurious false sysrq detects. This option allows you to decide whether you want to enable ... |
| MAGIC_SYSRQ_SERIAL_SEQUENCE | string | Specifies a sequence of characters that can follow BREAK to enable SysRq on a serial console. If unsure, leave an empty string and the option will not be enabled. |
| MAX_SKB_FRAGS | int | Having more fragments per skb_shared_info can help GRO efficiency. This helps BIG TCP workloads, but might expose bugs in some legacy drivers. This also increases memory overhead of small packets, ... |
| MEMBARRIER | bool | Enable the membarrier() system call that allows issuing memory barriers across all running threads, which can be used to distribute the cost of user-space memory barriers asymmetrically by transfor... |
| MEMCG | bool | Provides control over the memory footprint of tasks in a cgroup. |
| MEMCG_NMI_UNSAFE | bool | Legacy cgroup v1 memory controller which has been deprecated by cgroup v2 implementation. The v1 is there for legacy applications which haven't migrated to the new cgroup v2 interface yet. If you d... |
| MEMCPY_KUNIT_TEST | tristate | Builds unit tests for memcpy(), memmove(), and memset() functions. For more information on KUnit and unit tests in general please refer to the KUnit documentation in Documentation/dev-tools/kunit/.... |
| MEMORY_HOTREMOVE | bool | Allow for migration of pages inflated in a memory balloon such that they can be allocated from memory areas only available for movable allocations (e.g., ZONE_MOVABLE, CMA) and such that they can b... |
| MEMORY_NOTIFIER_ERROR_INJECT | tristate | This option provides the ability to inject artificial errors to memory hotplug notifier chain callbacks.  It is controlled through debugfs interface under /sys/kernel/debug/notifier-error-inject/me... |
| MEMTEST | bool | This option adds a kernel parameter 'memtest', which allows memtest to be set and executed. memtest=0, mean disabled; -- default memtest=1, mean do 1 test pattern; ... memtest=17, mean do 17 test p... |
| MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT | bool | Adds warnings with helpful error messages for memory allocation profiling. |
| MEM_SOFT_DIRTY | bool | This option enables memory changes tracking by introducing a soft-dirty bit on pte-s. This bit it set when someone writes into a page just as regular dirty bit, but unlike the latter it can be clea... |
| MESSAGE_LOGLEVEL_DEFAULT | int | Default log level for printk statements with no specified priority. This was hard-coded to KERN_WARNING since at least 2.6.10 but folks that are auditing their logs closely may want to set it to a ... |
| MHP_DEFAULT_ONLINE_TYPE_OFFLINE | bool | Hotplugged memory will not be onlined by default. Choose this for systems with drivers and user policy that handle onlining of hotplug memory policy. |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_AUTO | bool | Select this if you want the kernel to automatically online hotplugged memory into the zone it thinks is reasonable. This memory may be utilized for kernel data. |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_KERNEL | bool | Select this if you want the kernel to automatically online hotplugged memory into a zone capable of being used for kernel data. This typically means ZONE_NORMAL. |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_MOVABLE | bool | Select this if you want the kernel to automatically online hotplug memory into ZONE_MOVABLE. This memory will generally not be utilized for kernel data. This should only be used when the admin know... |
| MIGRATION | bool | Allows the pageblock_order value to be dynamic instead of just standard HUGETLB_PAGE_ORDER when there are multiple HugeTLB page sizes available on a platform. Note that the pageblock_order cannot e... |
| MIN_HEAP_KUNIT_TEST | tristate | This option enables the KUnit test suite for the min heap library which provides functions for creating and managing min heaps. The test suite checks the functionality of the min heap library. If u... |
| MMAP_ALLOW_UNINITIALIZED | bool | Normally, and according to the Linux spec, anonymous memory obtained from mmap() has its contents cleared before it is passed to userspace.  Enabling this config option allows you to request that m... |
| MM_ID | def_bool | Transparent Hugepages allows the kernel to use huge pages and huge tlb transparently to the applications whenever possible. This feature can improve computing performance to certain applications by... |
| MODULE_ALLOW_BTF_MISMATCH | bool | For modules whose split BTF does not match vmlinux, load without BTF rather than refusing to load. The default behavior with module BTF enabled is to reject modules with such mismatches; this optio... |
| MPILIB | tristate | Multiprecision maths library from GnuPG. It is used to implement RSA digital signature verification, which is used by IMA/EVM digital signature extension. |
| MSEAL_SYSTEM_MAPPINGS | bool | Apply mseal on system mappings. The system mappings includes vdso, vvar, vvar_vclock, vectors (arm compat-mode), sigpage (arm compat-mode), uprobes. A 64-bit kernel is required for the memory seali... |
| MULTIUSER | bool | This option enables support for non-root users, groups and capabilities. If you say N here, all processes will run with UID 0, GID 0, and all possible capabilities.  Saying N here also compiles out... |
| NET | bool | Unless you really know what you are doing, you should say Y here. The reason is that some programs need kernel networking support even when running on a stand-alone machine that isn't connected to ... |
| NETDEV_ADDR_LIST_TEST | tristate | KUnit tests covering core networking infra, such as sk_buff. If unsure, say N. |
| NETDEV_NOTIFIER_ERROR_INJECT | tristate | This option provides the ability to inject artificial errors to netdevice notifier chain callbacks.  It is controlled through debugfs interface /sys/kernel/debug/notifier-error-inject/netdev If the... |
| NETFILTER | bool | Netfilter is a framework for filtering and mangling network packets that pass through your Linux box. The most common use of packet filtering is to run your Linux box as a firewall protecting a loc... |
| NETFILTER_ADVANCED | bool | If you say Y here you can select between all the netfilter modules. If you say N the more unusual ones will not be shown and the basic ones needed by most people will default to 'M'. If unsure, say Y. |
| NETWORK_FILESYSTEMS | bool | Say Y here to get to see options for network filesystems and filesystem-related networking code, such as NFS daemon and RPCSEC security modules. This option alone does not add any kernel code. If y... |
| NETWORK_SECMARK | bool | This enables security marking of network packets, similar to nfmark, but designated for security purposes. If you are unsure how to answer this question, answer N. |
| NET_DEVLINK | bool | Enable page pool statistics to track page allocation and recycling in page pools. This option incurs additional CPU cost in allocation and recycle paths and additional memory cost to store the stat... |
| NET_DROP_MONITOR | tristate | This feature provides an alerting service to userspace in the event that packets are discarded in the network stack.  Alerts are broadcast via netlink socket to any listening user space process. Th... |
| NET_FLOW_LIMIT | bool | The network stack has to drop packets when a receive processing CPU's backlog reaches netdev_max_backlog. If a few out of many active flows generate the vast majority of load, drop their traffic ea... |
| NET_INGRESS | bool | This builds the KUnit tests for the handshake upcall mechanism. KUnit tests run during boot and output the results to the debug log in TAP format (https://testanything.org/). Only useful for kernel... |
| NET_NS | bool | Allow user space to create what appear to be multiple instances of the network stack. |
| NET_PKTGEN | tristate | This module will inject preconfigured packets, at a configurable rate, out of a given interface.  It is used for network interface stress testing and performance analysis.  If you don't understand ... |
| NET_PTP_CLASSIFY | def_bool | This allows timestamping of network packets by PHYs (or other MII bus snooping devices) with hardware timestamping capabilities. This option adds some overhead in the transmit and receive paths. If... |
| NET_RX_BUSY_POLL | bool | Enabling this allows a TCP stream parser to be used with BPF_MAP_TYPE_SOCKMAP. |
| NFS_V4_2_SSC_HELPER | bool |  |
| NLATTR | bool | Helper library to poll interrupt mitigation using polling. |
| NOINSTR_VALIDATION | bool | Selecting this option will pass "-Map=vmlinux.map" to ld when linking vmlinux. That file can be useful for verifying and debugging magic section games, and for seeing which pieces of code get elimi... |
| NOTIFIER_ERROR_INJECTION | tristate | This option provides the ability to inject artificial errors to specified notifier chain callbacks. It is useful to test the error handling of notifier call chain failures. Say N if unsure. |
| NO_PAGE_MAPCOUNT | bool | Do not maintain per-page mapcounts for pages part of larger allocations, such as transparent huge pages. When this config option is enabled, some interfaces that relied on this information will rel... |
| NUMA_BALANCING_DEFAULT_ENABLED | bool | If set, automatic NUMA balancing will be enabled if running on a NUMA machine. |
| NUMA_MIGRATION | bool | Support the migration of pages to other NUMA nodes, available to user space through interfaces like migrate_pages(), move_pages(), and mbind(). Selecting this option also enables support for page d... |
| OBJTOOL | bool | Fail the build on objtool warnings. Objtool warnings can indicate kernel instability, including boot failures.  This option is highly recommended. If unsure, say Y. |
| OF_RECONFIG_NOTIFIER_ERROR_INJECT | tristate | This option provides the ability to inject artificial errors to OF reconfig notifier chain callbacks.  It is controlled through debugfs interface under /sys/kernel/debug/notifier-error-inject/OF-re... |
| OVERFLOW_KUNIT_TEST | tristate | Builds unit tests for the check_*_overflow(), size_*(), allocation, and related functions. For more information on KUnit and unit tests in general please refer to the KUnit documentation in Documen... |
| PACKING_KUNIT_TEST | tristate | This builds KUnit tests for the packing library. For more information on KUnit and unit tests in general, please refer to the KUnit documentation in Documentation/dev-tools/kunit/. When in doubt, s... |
| PAGE_COUNTER | bool | This option enables the "favordynmods" mount option by default which reduces the latencies of dynamic cgroup modifications such as task migrations and controller on/offs at the cost of making hot p... |
| PAGE_IDLE_FLAG | bool | This adds PG_idle and PG_young flags to 'struct page'.  PTE Accessed bit writers can set the state of the bit in the flags so that PTE Accessed bit readers may avoid disturbance. |
| PAGE_MAPCOUNT | def_bool | This enables the Contiguous Memory Allocator which allows other subsystems to allocate big physically-contiguous blocks of memory. CMA reserves a region of memory and allows only movable pages to b... |
| PAHOLE_HAS_BTF_TAG | def_bool | Decide whether pahole emits btf_tag attributes (btf_type_tag and btf_decl_tag) or not. Currently only clang compiler implements these attributes, so make the config depend on CC_IS_CLANG. |
| PAHOLE_HAS_LANG_EXCLUDE | def_bool | Support for the --lang_exclude flag which makes pahole exclude compilation units from the supplied language. Used in Kbuild to omit Rust CUs which are not supported in version 1.24 of pahole, other... |
| PANIC_ON_OOPS | bool | Say Y here to enable the kernel to panic when it oopses. This has the same effect as setting oops=panic on the kernel command line. This feature is useful to ensure that the kernel does not do anyt... |
| PANIC_TIMEOUT | int | Set the timeout value (in seconds) until a reboot occurs when the kernel panics. If n = 0, then we wait forever. A timeout value n > 0 will wait n seconds before rebooting, while a timeout value n ... |
| PC104 | bool | Expose PC/104 form factor 设备驱动程序 and options available for selection and configuration. Enable this option if your target machine has a PC/104 bus. |
| PCPU_DEV_REFCNT | bool | network device refcount are using per cpu variables if this option is set. This can be forced to N to detect underflows (with a performance drop). |
| PCSPKR_PLATFORM | bool | This option allows to disable the internal PC-Speaker support, saving some memory. |
| PERCPU_STATS | bool | This feature collects and exposes statistics via debugfs. The information includes global and per chunk statistics, which can be used to help understand percpu memory usage. |
| PERCPU_TEST | tristate | Enable this option to build test module which validates per-cpu operations. If unsure, say N. |
| PERF_EVENTS | bool | Enable kernel support for various performance events provided by software and hardware. Software events are supported either built-in or via the use of generic tracepoints. Most modern CPUs support... |
| PHYS_ADDR_T_64BIT | def_bool | Enable Kernel Samepage Merging: KSM periodically scans those areas of an application's address space that an app has advised may be mergeable.  When it finds pages of identical content, it replaces... |
| PID_NS | bool | Support process id namespaces.  This allows having multiple processes with the same pid as long as they are in different pid namespaces.  This is a building block of containers. |
| PM_NOTIFIER_ERROR_INJECT | tristate | This option provides the ability to inject artificial errors to PM notifier chain callbacks.  It is controlled through debugfs interface /sys/kernel/debug/notifier-error-inject/pm If the notifier c... |
| POSIX_MQUEUE_SYSCTL | bool | This is a general notification queue for the kernel to pass events to userspace by splicing them into pipes.  It can be used in conjunction with watches for key/keyring change notifications and dev... |
| POSIX_TIMERS | bool | This includes native support for POSIX timers to the kernel. Some embedded systems have no use for them and therefore they can be configured out to reduce the size of the kernel image. When this op... |
| PREEMPT_NOTIFIERS | bool | Build a simple ASN.1 grammar compiler that produces a bytecode output that can be interpreted by the ASN.1 stream decoder and used to inform it as to what tags are to be expected in a stream and wh... |
| PRIME_NUMBERS_KUNIT_TEST | tristate | This option enables the KUnit test suite for the {is,next}_prime_number functions. Enabling this option will include tests that compare the prime number generator functions against a brute force im... |
| PRINTK | bool | This option enables normal printk support. Removing it eliminates most of the message strings from the kernel image and makes the kernel more or less silent. As this makes it very difficult to diag... |
| PRINTK_CALLER | bool | Selecting this option causes printk() to add a caller "thread id" (if in task context) or a caller "processor id" (if not in task context) to every message. This option is intended for environments... |
| PRINTK_EXECUTION_CTX | bool | This option extends struct printk_info to include extra execution context in printk, such as task name and CPU number from where the message originated. This is useful for correlating printk messag... |
| PRINTK_INDEX | bool | Add support for indexing of all printk formats known at compile time at <debugfs>/printk/index/<module>. This can be used as part of maintaining daemons which monitor /dev/kmsg, as it permits audit... |
| PRINTK_RINGBUFFER_KUNIT_TEST | tristate | This builds the printk ringbuffer KUnit test suite. For more information on KUnit and unit tests in general, please refer to the KUnit documentation. If unsure, say N. |
| PRINTK_TIME | bool | Selecting this option causes time stamps of the printk() messages to be added to the output of the syslog() system call and at the console. The timestamp is always recorded internally, and exported... |
| PROC_MEM_ALWAYS_FORCE | bool | This allows /proc/pid/mem accesses to override memory mapping permissions if you have ptrace access rights. |
| PROC_MEM_FORCE_PTRACE | bool | This allows /proc/pid/mem accesses to override memory mapping permissions for active ptracers like gdb. |
| PROC_MEM_NO_FORCE | bool | Never override memory mapping permissions |
| PROC_PID_CPUSET | bool | Provides a cgroup controller implementing whitelists for devices which a process in the cgroup can mknod or open. |
| PROFILING | bool | Say Y here to enable the extended profiling support mechanisms used by profilers. |
| PROVE_RAW_LOCK_NESTING | bool | Enable the raw_spinlock vs. spinlock nesting checks which ensure that the lock nesting rules for PREEMPT_RT enabled kernels are not violated. |
| PROVIDE_OHCI1394_DMA_INIT | bool | If you want to debug problems which hang or crash the kernel early on boot and the crashing machine has a FireWire port, you can use this feature to remotely access the memory of the crashed machin... |
| PSI | bool | Collect metrics that indicate how overcommitted the CPU, memory, and IO capacity are in the system. If you say Y here, the kernel will create /proc/pressure/ with the pressure statistics files cpu,... |
| PSI_DEFAULT_DISABLED | bool | If set, pressure stall information tracking will be disabled per default but can be enabled through passing psi=1 on the kernel commandline during boot. This feature adds some code to the task wake... |
| PTE_MARKER_UFFD_WP | bool | Allows to create marker PTEs for userfaultfd write protection purposes.  It is required to enable userfaultfd write protection on file-backed memory types like shmem and hugetlbfs. |
| RANDOM_KMALLOC_CACHES | bool | A hardening feature that creates multiple copies of slab caches for normal kmalloc allocation and makes kmalloc randomly pick one based on code address, which makes the attackers more difficult to ... |
| RANDSTRUCT_KUNIT_TEST | tristate | Builds unit tests for the checking CONFIG_RANDSTRUCT=y, which randomizes structure layouts. |
| RATELIMIT_KUNIT_TEST | tristate | This builds the "test_ratelimit" module that should be used for correctness verification and concurrent testings of rate limiting. If unsure, say N. |
| RATIONAL_KUNIT_TEST | tristate | This builds the rational math unit test. For more information on KUnit and unit tests in general please refer to the KUnit documentation in Documentation/dev-tools/kunit/. If unsure, say N. |
| RBTREE_TEST | tristate | A benchmark measuring the performance of the rbtree library. Also includes rbtree invariant checks. |
| READABLE_ASM | bool | Disable some compiler optimizations that tend to generate human unreadable assembler output. This may make the kernel slightly slower, but it helps to keep kernel developers who have to stare a lot... |
| READ_ONLY_THP_FOR_FS | bool | Allow khugepaged to put read-only file-backed pages in THP. This is marked experimental because it is a new feature. Write support of file THPs will be developed in the next few release cycles. |
| REED_SOLOMON_TEST | tristate | This option enables the self-test function of rslib at boot, or at module load time. If unsure, say N. |
| RELAY | bool | This option enables support for relay interface support in certain file systems (such as debugfs). It is designed to provide an efficient mechanism for tools and facilities to relay large amounts o... |
| RESOURCE_KUNIT_TEST | tristate | This builds the resource API unit test. Tests the logic of API provided by resource.c and ioport.h. For more information on KUnit and unit tests in general please refer to the KUnit documentation i... |
| RFS_ACCEL | bool | Allowing drivers for multiqueue hardware with flow filter tables to accelerate RFS. |
| RPS | bool | Software receive side packet steering (RPS) distributes the load of received packet processing across multiple CPUs. |
| RSEQ | bool | Enable the restartable sequences system call. It provides a user-space cache for the current CPU number value, which speeds up getting the current CPU number from user-space, as well as an ABI to s... |
| RSEQ_DEBUG_DEFAULT_ENABLE | bool | This enables the static branch for debug mode of restartable sequences. This also can be controlled on the kernel command line via the command line parameter "rseq_debug=0/1" and through debugfs. I... |
| RSEQ_SLICE_EXTENSION | bool | Allows userspace to request a limited time slice extension when returning from an interrupt to user space via the RSEQ shared data ABI. If granted, that allows to complete a critical section, so th... |
| RSEQ_STATS | bool | Enable lightweight counters which expose information about the frequency of RSEQ operations via debugfs. Mostly interesting for kernel debugging or performance analysis. While lightweight it's stil... |
| RT_GROUP_SCHED | bool | This feature lets you explicitly allocate real CPU bandwidth to task groups. If enabled, it will also make it impossible to schedule realtime tasks for non-root users until you allocate realtime ba... |
| RT_GROUP_SCHED_DEFAULT_DISABLED | bool | When set, the RT group scheduling is disabled by default. The option is in inverted form so that mere RT_GROUP_SCHED enables the group scheduling. Say N if unsure. |
| RUNTIME_TESTING_MENU | bool | Enable this to include the Dhrystone 2.1 benchmark.  This test calculates the number of Dhrystones per second, and the number of DMIPS (Dhrystone MIPS) obtained when the Dhrystone score is divided ... |
| RUST | bool | Enables Rust support in the kernel. This allows other Rust-related options, like drivers written in Rust, to be selected. It is also required to be able to load external kernel modules written in R... |
| RUSTC_LLVM_VERSION | int | This indicates whether Rust and Clang use LLVM of the same major version. Operations involving handling LLVM IR or bitcode (e.g. cross-language LTO) require the same LLVM major version to work prop... |
| RUSTC_VERSION_TEXT | string | See `CC_VERSION_TEXT`. |
| RUST_BUILD_ASSERT_ALLOW | bool | Controls how `build_error!` and `build_assert!` are handled during the build. If calls to them exist in the binary, it may indicate a violated invariant or that the optimizer failed to verify the i... |
| RUST_DEBUG_ASSERTIONS | bool | Enables rustc's `-Cdebug-assertions` codegen option. This flag lets you turn `cfg(debug_assertions)` conditional compilation on or off. This can be used to enable extra debugging code in developmen... |
| RUST_INLINE_HELPERS | bool | Inlines C helpers into Rust code using Link Time Optimization. If this option is enabled, C helper functions declared in rust/helpers/ are inlined into Rust code, which is helpful for performance o... |
| RUST_IS_AVAILABLE | def_bool | This shows whether a suitable Rust toolchain is available (found). Please see Documentation/rust/quick-start.rst for instructions on how to satisfy the build requirements of Rust support. In partic... |
| RUST_KERNEL_DOCTESTS | bool | This builds the documentation tests of the `kernel` crate as KUnit tests. For more information on KUnit and unit tests in general, please refer to the KUnit documentation in Documentation/dev-tools... |
| RUST_OVERFLOW_CHECKS | bool | Enables rustc's `-Coverflow-checks` codegen option. This flag allows you to control the behavior of runtime integer overflow. When overflow-checks are enabled, a Rust panic will occur on overflow. ... |
| SCANF_KUNIT_TEST | tristate | Enable this option to test the scanf functions at runtime. If unsure, say N. |
| SCF_TORTURE_TEST | tristate | This option provides a kernel module that runs torture tests on the smp_call_function() family of primitives.  The kernel module may be built after the fact on the running kernel to be tested, if d... |
| SCHED_AUTOGROUP | bool | This option optimizes the scheduler for common desktop workloads by automatically creating and populating task groups.  This separation of workloads isolates aggressive CPU burners (like build jobs... |
| SCHED_INFO | bool | If you say Y here, additional code will be inserted into the scheduler and related routines to collect statistics about scheduler behavior and provide them in /proc/schedstat.  These stats may be u... |
| SCHED_PROXY_EXEC | bool | This option enables proxy execution, a mechanism for mutex-owning tasks to inherit the scheduling context of higher priority waiters. |
| SCHED_STACK_END_CHECK | bool | This option checks for a stack overrun on calls to schedule(). If the stack end location is found to be over written always panic as the content of the corrupted region can no longer be trusted. Th... |
| SECTION_MISMATCH_WARN_ONLY | bool | If you say N here, the build process will fail if there are any section mismatch, instead of just throwing warnings. If unsure, say Y. |
| SECURITY | bool | This allows you to choose different security modules to be configured into your kernel. If this option is not selected, the default Linux security model will be used. If you are unsure how to answe... |
| SECURITY_COMMONCAP_KUNIT_TEST | bool | This builds the commoncap KUnit tests. KUnit tests run during boot and output the results to the debug log in TAP format (https://testanything.org/). Only useful for kernel devs running KUnit test ... |
| SECURITY_DMESG_RESTRICT | bool | This enforces restrictions on unprivileged users reading the kernel syslog via dmesg(8). If this option is not selected, no restrictions will be enforced unless the dmesg_restrict sysctl is explici... |
| SECURITY_INFINIBAND | bool | This enables the Infiniband security hooks. If enabled, a security module can use these hooks to implement Infiniband access controls. If you are unsure how to answer this question, answer N. |
| SECURITY_NETWORK | bool | This enables the socket and networking security hooks. If enabled, a security module can use these hooks to implement socket and networking access controls. If you are unsure how to answer this que... |
| SECURITY_NETWORK_XFRM | bool | This enables the XFRM (IPSec) networking security hooks. If enabled, a security module can use these hooks to implement per-packet access controls based on labels derived from IPSec policy.  Non-IP... |
| SECURITY_PATH | bool | This enables the security hooks for pathname based access control. If enabled, a security module can use these hooks to implement pathname based access controls. If you are unsure how to answer thi... |
| SELECT_MEMORY_MODEL | def_bool | This option allows you to change some of the ways that Linux manages its memory internally. Most users will only have one option here selected by the architecture configuration. This is normal. |
| SEQ_BUF_KUNIT_TEST | tristate | This builds unit tests for the seq_buf library. If unsure, say N. |
| SGETMASK_SYSCALL | bool | sys_sgetmask and sys_ssetmask are obsolete system calls no longer supported in libc but still enabled by default in some architectures. If unsure, leave the default option here. |
| SG_POOL | def_bool | Provides a helper to allocate chained scatterlists. This should be selected by a driver or an API which whishes to allocate chained scatterlist. # # sg chaining option # |
| SHMEM | bool | The shmem is an internal filesystem used to manage shared memory. It is backed by swap and manages resource limits. It is also exported to userspace as tmpfs if TMPFS is enabled. Disabling this opt... |
| SHRINKER_DEBUG | bool | Say Y to enable the shrinker debugfs interface which provides visibility into the kernel memory shrinkers subsystem. Disable it to avoid an extra memory footprint. |
| SHUFFLE_PAGE_ALLOCATOR | bool | Randomization of the page allocator improves the average utilization of a direct-mapped memory-side-cache. See section 5.2.27 Heterogeneous Memory Attribute Table (HMAT) in the ACPI 6.2a specificat... |
| SIGNALFD | bool | Enable the signalfd() system call that allows to receive signals on a file descriptor. If unsure, say Y. |
| SIGNATURE | tristate | Digital signature verification. Currently only RSA is supported. Implementation is done using GnuPG MPI library |
| SIPHASH_KUNIT_TEST | tristate | Enable this option to test the kernel's siphash (<linux/siphash.h>) hash functions on boot (or module load). This is intended to help people writing architecture-specific optimized versions.  If un... |
| SLAB_BUCKETS | bool | Kernel heap attacks frequently depend on being able to create specifically-sized allocations with user-controlled contents that will be allocated into the same kmalloc bucket as a target object. To... |
| SLAB_FREELIST_HARDENED | bool | Many kernel heap attacks try to target slab cache metadata and other infrastructure. This options makes minor performance sacrifices to harden the kernel slab allocator against common freelist expl... |
| SLAB_FREELIST_RANDOM | bool | Randomizes the freelist order used on creating new pages. This security feature reduces the predictability of the kernel slab allocator against heap overflows. |
| SLAB_MERGE_DEFAULT | bool | For reduced kernel memory fragmentation, slab caches can be merged when they share the same size and other characteristics. This carries a risk of kernel heap overflows being able to overwrite obje... |
| SLAB_OBJ_EXT | bool | This option adds support for grouping sets of processes together, for use with process control subsystems such as Cpusets, CFS, memory controls or device isolation. See - Documentation/scheduler/sc... |
| SLUB | def_bool | Configures the slab allocator in a way to achieve minimal memory footprint, sacrificing scalability, debugging and other features. This is intended only for the smallest system that had used the SL... |
| SLUB_KUNIT_TEST | tristate | This builds SLUB allocator unit test. Tests SLUB cache debugging functionality. For more information on KUnit and unit tests in general please refer to the KUnit documentation in Documentation/dev-... |
| SLUB_STATS | bool | The statistics are useful to debug slab allocation behavior in order find ways to optimize the allocator. This should never be enabled for production use since keeping statistics slows down the all... |
| SOCK_CGROUP_DATA | bool | Provides the way to make tasks work with different objects using the same id. For example same IPC id may refer to different objects or same user id or pid may refer to different tasks when used in... |
| SOCK_RX_QUEUE_MAPPING | bool | Cgroup subsystem for use in assigning processes to network priorities on a per-interface basis. |
| SOFTLOCKUP_DETECTOR_INTR_STORM | bool | Say Y here to enable the kernel to detect interrupt storm during "soft lockups". "soft lockups" can be caused by a variety of reasons. If one is caused by an interrupt storm, then the storming inte... |
| SPARSEMEM | def_bool | SPARSEMEM_VMEMMAP uses a virtually mapped memmap to optimise pfn_to_page and page_to_pfn operations.  This is the most efficient option when sufficient kernel resources are available. |
| SPARSEMEM_MANUAL | bool | This will be the only option for some systems, including memory hot-plug systems.  This is normal. This option provides efficient support for systems with holes is their physical address space and ... |
| SPARSEMEM_VMEMMAP_PREINIT | bool | Default memory type for hotplugged memory. This option sets the default policy setting for memory hotplug onlining policy (/sys/devices/system/memory/auto_online_blocks) which determines what happe... |
| STACKDEPOT_ALWAYS_INIT | bool | Always initialize stack depot during early boot |
| STACKDEPOT_MAX_FRAMES | int | Run boot-time test of light-weight queuing. |
| STACKINIT_KUNIT_TEST | tristate | Test if the kernel is zero-initializing stack variables and padding. Coverage is controlled by compiler flags, CONFIG_INIT_STACK_ALL_PATTERN or CONFIG_INIT_STACK_ALL_ZERO. |
| STACKTRACE | bool | This option causes the kernel to create a /proc/pid/stack for every process, showing its current stack trace. It is also used by various kernel debugging features that require stack trace generation. |
| STACKTRACE_BUILD_ID | bool | Selecting this option adds build ID information for symbols in stacktraces printed with the printk format '%p[SR]b'. This option is intended for distros where debuginfo is not easily accessible but... |
| STACK_VALIDATION | bool | Validate frame pointer rules at compile-time.  This helps ensure that runtime stack traces are more reliable. For more information, see tools/objtool/Documentation/objtool.txt. |
| STATIC_USERMODEHELPER | bool | By default, the kernel can call many different userspace binary programs through the "usermode helper" kernel interface.  Some of these binaries are statically defined either in the kernel code its... |
| STATIC_USERMODEHELPER_PATH | string | The binary called by the kernel when any usermode helper program is wish to be run.  The "real" application's name will be in the first argument passed to this program on the command line. If you w... |
| STRING_KUNIT_TEST | tristate | Enable performance measurement for string functions. This measures the execution efficiency of string functions during the KUnit test run. If unsure, say N. |
| STRIP_ASM_SYMS | bool | Strip internal assembler-generated symbols during a link (symbols that look like '.Lxxx') so they don't pollute the output of get_wchan() and suchlike. |
| SYMBOLIC_ERRNAME | bool | If you say Y here, the kernel's printf implementation will be able to print symbolic error names such as ENOSPC instead of the number 28. It makes the kernel image slightly larger (about 3KB), but ... |
| SYSCTL_ARCH_UNALIGN_ALLOW | bool | Enable support for /proc/sys/kernel/unaligned-trap Allows arches to define/use @unaligned_enabled to runtime toggle the unaligned access emulation. see arch/parisc/kernel/unaligned.c for reference |
| SYSCTL_ARCH_UNALIGN_NO_WARN | bool | Enable support for /proc/sys/kernel/ignore-unaligned-usertrap Allows arch to define/use @no_unaligned_warning to possibly warn about unaligned access emulation going on under the hood. |
| SYSCTL_KUNIT_TEST | tristate | This builds the proc sysctl unit test, which runs on boot. Tests the API contract and implementation correctness of sysctl. For more information on KUnit and unit tests in general please refer to t... |
| SYSFS_SYSCALL | bool | sys_sysfs is an obsolete system call no longer supported in libc. Note that disabling this option is more secure but might break compatibility with some systems. If unsure say N here. |
| SYSTEM_DATA_VERIFICATION | def_bool | Provide PKCS#7 message verification using the contents of the system trusted keyring to provide public keys.  This then can be used for module verification, kexec image verification and firmware bl... |
| SYSVIPC | bool | Inter Process Communication is a suite of library functions and system calls which let processes (running programs) synchronize and exchange information. It is generally considered to be a good thi... |
| SYSVIPC_SYSCTL | bool | POSIX variant of message queues is a part of IPC. In POSIX message queues every message has a priority which decides about succession of receiving it by a process. If you want to compile and run pr... |
| TASKSTATS | bool | Export selected statistics for tasks/processes through the generic netlink interface. Unlike BSD process accounting, the statistics are available during the lifetime of tasks/processes as responses... |
| TASK_DELAY_ACCT | bool | Collect information on time spent by a task waiting for system resources like cpu, synchronous block I/O completion and swapping in pages. Such statistics can help in setting a task's priorities re... |
| TASK_IO_ACCOUNTING | bool | Collect information on the number of bytes of storage I/O which this task has caused. Say N if unsure. |
| TASK_XACCT | bool | Collect extended task accounting data and send the data to userland for processing over the taskstats interface. Say N if unsure. |
| TEST_BITOPS | tristate | This builds the "test_bitops" module that is much like the TEST_LKM module except that it does a basic exercise of the set/clear_bit macros and get_count_order/long to make sure there are no compil... |
| TEST_BPF | tristate | This builds the "test_bpf" module that runs various test vectors against the BPF interpreter or BPF JIT compiler depending on the current setting. This is in particular useful for BPF JIT compiler ... |
| TEST_CLOCKSOURCE_WATCHDOG | tristate | Enable this option to create a kernel module that will trigger a test of the clocksource watchdog.  This module may be loaded via modprobe or insmod in which case it will run upon being loaded, or ... |
| TEST_DEBUG_VIRTUAL | tristate | Test the kernel's ability to detect incorrect calls to virt_to_phys() done against the non-linear part of the kernel's virtual address map. If unsure, say N. |
| TEST_DIV64 | tristate | Enable this to turn on 'do_div()' function test. This test is executed only once during system boot (so affects only boot time), or at module load time. If unsure, say N. |
| TEST_DYNAMIC_DEBUG | tristate | This module registers a tracer callback to count enabled pr_debugs in a 'do_debugging' function, then alters their enablements, calls the function, and compares counts. If unsure, say N. |
| TEST_FIRMWARE | tristate | This builds the "test_firmware" module that creates a userspace interface for testing firmware loading. This can be used to control the triggering of firmware loading without needing an actual firm... |
| TEST_FPU | tristate | Enable this option to add /sys/kernel/debug/selftest_helpers/test_fpu which will trigger a sequence of floating point operations. This is used for self-testing floating point control register setti... |
| TEST_FREE_PAGES | tristate | Test that a memory leak does not occur due to a race between freeing a block of pages and a speculative page reference. Loading this module is safe if your kernel has the bug fixed. If the bug is n... |
| TEST_HEXDUMP | tristate | Enable this option to test the printf functions at runtime. If unsure, say N. |
| TEST_HMM | tristate | This is a pseudo 设备驱动 solely for testing HMM. Say M here if you want to build the HMM test module. Doing so will allow you to run tools/testing/selftest/vm/hmm-tests. If unsure, say N. |
| TEST_IDA | tristate | Kunit test for miscdevice API, specially its behavior in respect to static and dynamic minor numbers. KUnit tests run during boot and output the results to the debug log in TAP format (https://test... |
| TEST_IOV_ITER | tristate | Enable this to turn on testing of the operation of the I/O iterator (iov_iter). This test is executed only once during system boot (so affects only boot time), or at module load time. If unsure, sa... |
| TEST_KALLSYMS_A | tristate | Selecting something other than "Fast" will enable tests which slow down the build and may crash your build. |
| TEST_KALLSYMS_FAST | bool | You won't really be testing kallsysms, so this just helps fast builds when allmodconfig is used.. |
| TEST_KALLSYMS_LARGE | bool | This will enable larger number of symbols. This will slow down your build considerably. |
| TEST_KALLSYMS_MAX | bool | This will enable exports to the point we know we'll start crashing builds. |
| TEST_KALLSYMS_NUMSYMS | int | The number of symbols to create on TEST_KALLSYMS_A, only one of which module TEST_KALLSYMS_B will use. This also will be used for how many symbols TEST_KALLSYMS_C will have, scaled up by TEST_KALLS... |
| TEST_KALLSYMS_SCALE_FACTOR | int | How many more unusued symbols will TEST_KALLSYSMS_C have than TEST_KALLSYMS_A. If 8, then module C will have 8 * syms than module A. Then TEST_KALLSYMS_D will have double the amount of symbols than... |
| TEST_KEXEC_HANDOVER | bool | This option enables test for Kexec HandOver (KHO). The test consists of two parts: saving kernel data before kexec and restoring the data after kexec and verifying that it was properly handed over.... |
| TEST_KMOD | tristate | Test the kernel's module loading mechanism: kmod. kmod implements support to load modules using the Linux kernel's usermode helper. This test provides a series of tests against kmod. Although techn... |
| TEST_KSTRTOX | tristate | Enable this option to test the bitmap functions at boot. If unsure, say N. |
| TEST_LIST_SORT | tristate | Enable this to turn on 'list_sort()' function test. This test is executed only once during system boot (so affects only boot time), or at module load time. If unsure, say N. |
| TEST_LKM | tristate | This builds the "test_module" module that emits "Hello, world" on printk when loaded. It is designed to be used for basic evaluation of the module loading subsystem (for example when validating mod... |
| TEST_LOCKUP | tristate | This builds the "test_lockup" module that helps to make sure that watchdogs and lockup detectors are working properly. Depending on module parameters it could emulate soft or hard lockup, "hung tas... |
| TEST_MEMCAT_P | tristate | Test the memcat_p() helper for correctly merging two pointer arrays together. If unsure, say N. |
| TEST_MEMINIT | tristate | Test if the kernel is zero-initializing heap and page allocations. This can be useful to test init_on_alloc and init_on_free features. If unsure, say N. |
| TEST_MULDIV64 | tristate | Enable this to turn on 'mul_u64_u64_div_u64()' function test. This test is executed only once during system boot (so affects only boot time), or at module load time. If unsure, say N. |
| TEST_OBJAGG | tristate | Enable this option to test object aggregation manager on boot (or module load). |
| TEST_OBJPOOL | tristate | This builds the "test_objpool" module that should be used for correctness verification and concurrent testings of objects allocation and reclamation. If unsure, say N. |
| TEST_PARMAN | tristate | Enable this option to test priority array manager on boot (or module load). If unsure, say N. |
| TEST_REF_TRACKER | tristate | This option provides a kernel module performing tests using reference tracker infrastructure. Say N if you are unsure. |
| TEST_RHASHTABLE | tristate | Enable this option to test the rhashtable functions at boot. If unsure, say N. |
| TEST_RUNTIME | bool | This allows us to stress test find_symbol() through the kallsyms used to place symbols on the kernel ELF kallsyms and modules kallsyms where we place kernel symbols such as exported symbols. We hav... |
| TEST_SORT | tristate | This option enables the self-test function of 'sort()' at boot, or at module load time. If unsure, say N. |
| TEST_STATIC_KEYS | tristate | Test the static key interfaces. If unsure, say N. |
| TEST_SYSCTL | tristate | This builds the "test_sysctl" module. This driver enables to test the proc sysctl interfaces available to drivers safely without affecting production knobs which might alter system functionality. I... |
| TEST_UDELAY | tristate | This builds the "udelay_test" module that helps to make sure that udelay() is working properly. If unsure, say N. |
| TEST_VMALLOC | tristate | This builds the "test_vmalloc" module that should be used for stress and performance analysis. So, any new change for vmalloc subsystem can be evaluated from performance and stability point of view... |
| TEST_WORKQUEUE | tristate | This builds the "test_workqueue" module for benchmarking workqueue throughput under contention. Useful for evaluating affinity scope changes (e.g., cache_shard vs cache). If unsure, say N. |
| TEST_XARRAY | tristate | Enable this option to test the maple tree code functions at boot, or when the module is loaded. Enable "Debug Maple Trees" will enable more verbose output on failures. If unsure, say N. |
| TEXTSEARCH | bool | Simple, embeddable, interval-tree. Can find the start of an overlapping range in log(n) time and then iterate over all overlapping nodes. The algorithm is implemented as an augmented rbtree. See: D... |
| THP_SWAP | def_bool | Swap transparent huge pages in one piece, without splitting. XXX: For now, swap cluster backing transparent huge page will be split after swapout. For selection by architectures with reasonable THP... |
| TIMERFD | bool | Enable the timerfd() system call that allows to receive timer events on a file descriptor. If unsure, say Y. |
| TIME_NS | bool | In this namespace boottime and monotonic clocks can be set. The time will keep going with the same pace. |
| TIME_NS_VDSO | def_bool | In this namespace tasks work with IPC ids which correspond to different IPC objects in different namespaces. |
| TMPFS | bool | Tmpfs is a file system which keeps all files in virtual memory. Everything in tmpfs is temporary in the sense that no files will be created on your hard drive. The files live in memory and swap spa... |
| TMPFS_INODE64 | bool | tmpfs has historically used only inode numbers as wide as an unsigned int. In some cases this can cause wraparound, potentially resulting in multiple files with the same inode number on a single de... |
| TMPFS_POSIX_ACL | bool | POSIX Access Control Lists (ACLs) support additional access rights for users and groups beyond the standard owner/group/world scheme, and this option selects support for ACLs specifically for tmpfs... |
| TMPFS_QUOTA | bool | Quota support allows to set per user and group limits for tmpfs usage.  Say Y to enable quota support. Once enabled you can control user and group quota enforcement with quota, usrquota and grpquot... |
| TMPFS_XATTR | bool | Extended attributes are name:value pairs associated with inodes by the kernel or by users (see the attr(5) manual page for details). This enables support for the trusted.*, security.* and user.* na... |
| TRACE_IRQFLAGS | bool | Enables hooks to interrupt enabling and disabling for either tracing or lock debugging. |
| TRACE_IRQFLAGS_NMI | def_bool | Enables debug prints when a CPU fails to respond to a given backtrace NMI.  These prints provide some reasons why a CPU might legitimately be failing to respond, for example, if it is offline of if... |
| TRACE_MMIO_ACCESS | bool | Create tracepoints for MMIO read/write operations. These trace events can be used for logging all MMIO read/write operations. |
| TRANSPARENT_HUGEPAGE_ALWAYS | bool | Enabling Transparent Hugepage always, can increase the memory footprint of applications without a guaranteed benefit but it will work automatically for all applications. |
| TRANSPARENT_HUGEPAGE_MADVISE | bool | Enabling Transparent Hugepage madvise, will only provide a performance improvement benefit to the applications using madvise(MADV_HUGEPAGE) but it won't risk to increase the memory footprint of app... |
| TRANSPARENT_HUGEPAGE_NEVER | bool | Disable Transparent Hugepage by default. It can still be enabled at runtime via sysfs. |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ADVISE | bool | Enable hugepage allocation for the shmem mount exclusively when applications supply the madvise(MADV_HUGEPAGE) hint. This ensures that hugepages are used only in response to explicit requests from ... |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ALWAYS | bool | Always attempt to allocate hugepage for shmem mount, can increase the memory footprint of applications without a guaranteed benefit but it will work automatically for all applications. |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_NEVER | bool | Disable hugepage allocation for shmem mount by default. It can still be enabled with the kernel command line 'transparent_hugepage_shmem=' option or at runtime via sysfs knob. Note that madvise(MAD... |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_WITHIN_SIZE | bool | Enable hugepage allocation for shmem mount if the allocation will be fully within the i_size. This configuration also takes into account any madvise(MADV_HUGEPAGE) hints that may be provided by the... |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ADVISE | bool | Enable hugepage allocation for the tmpfs mount exclusively when applications supply the madvise(MADV_HUGEPAGE) hint. This ensures that hugepages are used only in response to explicit requests from ... |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ALWAYS | bool | Always attempt to allocate hugepage for tmpfs mount, can increase the memory footprint of applications without a guaranteed benefit but it will work automatically for all applications. |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_NEVER | bool | Disable hugepage allocation for tmpfs mount by default. It can still be enabled with the kernel command line 'transparent_hugepage_tmpfs=' option. Note that madvise(MADV_COLLAPSE) can still cause t... |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_WITHIN_SIZE | bool | Enable hugepage allocation for tmpfs mount if the allocation will be fully within the i_size. This configuration also takes into account any madvise(MADV_HUGEPAGE) hints that may be provided by the... |
| UAPI_HEADER_TEST | bool | Compile test headers exported to user-space to ensure they are self-contained, i.e. compilable as standalone units. If you are a developer or tester and want to ensure the exported headers are self... |
| UCLAMP_BUCKETS_COUNT | int | Defines the number of clamp buckets to use. The range of each bucket will be SCHED_CAPACITY_SCALE/UCLAMP_BUCKETS_COUNT. The higher the number of clamp buckets the finer their granularity and the hi... |
| UCS2_STRING | tristate | Provides a helper to split scatterlists into chunks, each chunk being a scatterlist. This should be selected by a driver or an API which whishes to split a scatterlist amongst multiple DMA channels. |
| UID16 | bool | This enables the legacy 16-bit UID syscall wrappers. |
| USERCOPY_KUNIT_TEST | tristate | This builds the "usercopy_kunit" module that runs sanity checks on the copy_to/from_user infrastructure, making sure basic user/kernel boundary testing is working. |
| USERFAULTFD | bool | Enable the userfaultfd() system call that allows to intercept and handle page faults in userland. if USERFAULTFD |
| USER_NS | bool | This allows containers, i.e. vservers, to use user namespaces to provide different user info for different servers. When user namespaces are enabled in the kernel it is recommended that the MEMCG o... |
| UTIL_MACROS_KUNIT | tristate | Enable this option to test the util_macros.h function at boot. KUnit tests run during boot and output the results to the debug log in TAP format (http://testanything.org/). Only useful for kernel d... |
| UTS_NS | bool | In this namespace tasks see different info provided with the uname() system call |
| UUID_KUNIT_TEST | tristate | This option enables the KUnit test suite for the uuid library, which provides functions for generating and parsing UUID and GUID. The test suite checks parsing of UUID and GUID strings. If unsure, ... |
| VIRT_CPU_ACCOUNTING_GEN | bool | Select this option to enable task and CPU time accounting on full dynticks systems. This accounting is implemented by watching every kernel-user boundaries using the context tracking subsystem. The... |
| VIRT_CPU_ACCOUNTING_NATIVE | bool | Select this option to enable more accurate task and CPU time accounting.  This is done by reading a CPU counter on each kernel entry and exit and on transitions within the kernel between system, so... |
| VMAP_PFN | bool | VM event counters are needed for event counts to be shown. This option allows the disabling of the VM event counters on EXPERT systems.  /proc/vmstat will only show page counts if VM event counters... |
| WANT_COMPAT_NETLINK_MESSAGES | bool | This option can be selected by other options that need compat netlink messages. |
| WARN_ABI_ERRORS | bool | The files under Documentation/ABI should follow what's described at Documentation/ABI/README. Yet, as they're manually written, it would be possible that some of those files would have errors that ... |
| WARN_CONTEXT_ANALYSIS | bool | Context Analysis is a language extension, which enables statically checking that required contexts are active (or inactive) by acquiring and releasing user-definable "context locks". Clang's name o... |
| WARN_CONTEXT_ANALYSIS_ALL | bool | Enable tree-wide context analysis. This is likely to produce a large number of false positives - enable at your own risk. If unsure, say N. |
| WARN_MISSING_DOCUMENTS | bool | It is not uncommon that a document gets renamed. This option makes the Kernel to check for missing dependencies, warning when something is missing. Works only if the Kernel is built from a git tree... |
| WERROR | bool | A kernel build should not cause any compiler warnings, and this enables the '-Werror' (for C) and '-Dwarnings' (for Rust) flags to enforce that rule by default. Certain warnings from other tools su... |
| WQ_CPU_INTENSIVE_REPORT | bool | Say Y here to enable reporting of concurrency-managed per-cpu work items that hog CPUs for longer than workqueue.cpu_intensive_thresh_us. Workqueue automatically detects and excludes them from conc... |
| WQ_WATCHDOG | bool | Say Y here to enable stall detection on workqueues.  If a worker pool doesn't make forward progress on a pending work item for over a given amount of time, 30s by default, a warning message is prin... |
| WW_MUTEX_SELFTEST | tristate | This option provides a kernel module that runs tests on the on the struct ww_mutex locking API. It is recommended to enable DEBUG_WW_MUTEX_SLOWPATH in conjunction with this test harness. Say M if y... |
| XXHASH | tristate | This option enables the 32 bit PRNG library functions to perform a self test on initialization. # # compression support is select'ed if needed # |
| ZSMALLOC_CHAIN_SIZE | int | This option sets the upper limit on the number of physical pages that a zmalloc page (zspage) can consist of. The optimal zspage chain size is calculated for each size class during the initializati... |
| ZSWAP | bool | A lightweight compressed cache for swap pages.  It takes pages that are in the process of being swapped out and attempts to compress them into a dynamically allocated RAM-based memory pool. This ca... |
| ZSWAP_COMPRESSOR_DEFAULT | string | This option enables code in the zsmalloc to collect various statistics about what's happening in zsmalloc and exports that information to userspace via debugfs. If unsure, say N. |
| ZSWAP_COMPRESSOR_DEFAULT_842 | bool | Use the 842 algorithm as the default compression algorithm. |
| ZSWAP_COMPRESSOR_DEFAULT_DEFLATE | bool | Use the Deflate algorithm as the default compression algorithm. |
| ZSWAP_COMPRESSOR_DEFAULT_LZ4 | bool | Use the LZ4 algorithm as the default compression algorithm. |
| ZSWAP_COMPRESSOR_DEFAULT_LZ4HC | bool | Use the LZ4HC algorithm as the default compression algorithm. |
| ZSWAP_COMPRESSOR_DEFAULT_LZO | bool | Use the LZO algorithm as the default compression algorithm. |
| ZSWAP_COMPRESSOR_DEFAULT_ZSTD | bool | Use the zstd algorithm as the default compression algorithm. |
| ZSWAP_DEFAULT_ON | bool | If selected, the compressed cache for swap pages will be enabled at boot, otherwise it will be disabled. The selection made here can be overridden by using the kernel command line 'zswap.enabled=' ... |
| ZSWAP_SHRINKER_DEFAULT_ON | bool | If selected, the zswap shrinker will be enabled, and the pages stored in the zswap pool will become available for reclaim (i.e written back to the backing swap device) on memory pressure. This mean... |
| if | bool | If you say Y here gcc is instructed to generate less debugging information for structure types. This means that tools that need full debugging information (like kgdb or systemtap) won't be happy. B... |
| select | bool | Generate DWARF v5 debug info. Requires binutils 2.35.2, gcc 5.0+ (gcc 5.0+ accepts the -gdwarf-5 flag but only had partial support for some draft features until 7.0), and gdb 8.0+. Changes to the s... |

---

# Makefile Targets

## Build targets

| Target | Description | Source |
|--------|-------------|--------|
| all | If building an external module we do not care about the all: rule but instead __all depend on modules | Makefile |
| dtbs_install |  | Makefile |
| headers |  | Makefile |
| headers_install |  | Makefile |
| modules | Build all loadable kernel modules | Makefile |
| modules_install |  | Makefile |
| vmlinux |  | Makefile |

## Configuration targets

| Target | Description | Source |
|--------|-------------|--------|
| config |  | Makefile |

## Clean targets

| Target | Description | Source |
|--------|-------------|--------|
| clean | clean - Delete most, but leave enough to build external modules  | Makefile |
| distclean | distclean  | Makefile |
| mrproper | mrproper - Delete all generated files, including .config  | Makefile |

## Documentation targets

| Target | Description | Source |
|--------|-------------|--------|
| cleandocs | Remove all generated documentation files | Makefile |
| htmldocs-redirects |  | Makefile |
| markdowndocs | Build Markdown documentation via Pandoc post-processing | Makefile |
| refcheckdocs | Check for broken file references in docs | Makefile |

## Other targets

| Target | Description | Source |
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
| dtbs_prepare | include/config/kernel.release is actually needed when installing DTBs because INSTALL_DTBS_PATH contains $(KERNELRELEASE). However, we do not want to make dtbs_install depend on it as dtbs_install may run as root. | Makefile |
| headerdep |  | Makefile |
| help | Show available make targets | Makefile |
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
| rust-analyzer | Generate rust-project.json (a file that describes the structure of non-Cargo Rust projects) for rust-analyzer (an implementation of the Language Server Protocol). | Makefile |
| rustavailable | "Is Rust available?" target | Makefile |
| rustdoc | Documentation target  Using the singular to avoid running afoul of `no-dot-config-targets`. | Makefile |
| rustfmt | Formatting targets  Generated files as well as vendored crates are skipped. | Makefile |
| rustfmtcheck |  | Makefile |
| rusttest | Testing target | Makefile |
| scripts | Additional helpers built in scripts/ Carefully list dependencies so we do not try to build scripts twice in parallel | Makefile |
| scripts_basic | Basic helpers built in scripts/basic/ | Makefile |
| scripts_dtc |  | Makefile |
| scripts_gdb |  | Makefile |
| scripts_gen_packed_field_checks |  | Makefile |
| scripts_unifdef |  | Makefile |
| uapi-asm-generic |  | Makefile |
| usr_gen_init_cpio |  | Makefile |
| versioncheck |  | Makefile |

---

# Subsystem Descriptions

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
