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

| Config | 类型 | 描述 |
|--------|------|-------------|
| 842_COMPRESS | tristate | 启用内核中 s390x 对 zlib 的硬件支持。 |
| ADVISE_SYSCALLS | bool | 该选项启用 madvise 与 fadvise 系统调用，应用程序借此向内核建议其未来的内存或文件使用方式，从而提升性能。若构建…… |
| AIO | bool | 该选项启用 POSIX 异步 I/O，部分高性能多线程应用可能会用到。禁用此选项可节省约 7k。 |
| ANON_VMA_NAME | bool | 允许为匿名虚拟内存区域命名。该功能可为虚拟内存区域指定名称，所指定名称随后可从 /proc/pid/maps 与 /proc/pid/smaps 中读取，有助于识…… |
| ARCH_FORCE_MAX_ORDER | int | 页块阶（page block order）指物理连续、可关联迁移类型的页面数量的 2 的幂。页块阶的最大尺寸至少为…… |
| ARCH_HAS_BINFMT_FLAT | bool | 支持 uClinux FLAT 格式二进制文件。 |
| ARCH_HAS_CC_CAN_LINK | bool | 选择此项可将 thread_info 从栈上移入 task_struct。为使此功能生效，体系结构需移除除 flags 外的所有 thread_info 字段并修复相关运行时缺陷。其中一个细微改动…… |
| ARCH_HAS_CPU_CACHE_ALIASING | bool | 为支持 HARDENED_USERCOPY 进行栈变量生命周期检查，需要一种与体系结构无关的方式来获取栈指针。一旦某体系结构定义了 unsigned long 全局变量 r…… |
| ARCH_HAS_DEBUG_VIRTUAL | bool | 在虚拟地址到页的转换代码中启用一些代价较高的健全性检查。可捕获 virt_to_page() 等函数的误用。若不确定，选 N。 |
| ARCH_HAS_DEBUG_VM_PGTABLE | bool | 当某体系结构能成功构建并运行 DEBUG_VM_PGTABLE 时，应选择此项。 |
| ARCH_HAS_DEVMEM_IS_ALLOWED | bool | 若禁用此选项，则允许用户空间（root）访问全部内存，包括内核与用户空间内存。意外访问显然后果严重，但特定访问可能…… |
| ARCH_HAS_ELF_CORE_EFLAGS | bool | 若体系结构利用 ELF 头中的 e_flags 字段来存放应在核心转储中保留的 ABI 或其他体系结构相关信息，请选择此项。 |
| ARCH_HAS_KCOV | bool | 当某体系结构能成功在 CONFIG_KCOV 下构建并运行时，应选择此项。这通常需对某些早期引导代码禁用插桩。 |
| ARCH_HAS_MEMBARRIER_CALLBACKS | bool | 基于体系结构控制 MSEAL_SYSTEM_MAPPINGS 的访问。内存密封特性需要 64 位内核。无需 CPU 提供特定硬件特性。要启用此特性…… |
| ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE | bool |  |
| ARCH_HAS_PTE_SPECIAL | bool | 启用 memfd_secret() 系统调用，可创建仅在所属进程上下文中可见、且不映射到其他进程及其他内核页表的内存区域。 |
| ARCH_HAS_STRNCPY_FROM_USER | bool | 在某些不存在独立 I/O 空间的平台上，部分 I/O 主机无法以 MMIO 模式访问。借助逻辑 PIO 机制，主机本地 I/O 资源可被映射到系统…… |
| ARCH_HAS_USER_SHADOW_STACK | bool | 该体系结构提供对用户空间影子调用栈（shadow call stack）的硬件支持（例如 x86 CET、arm64 GCS 或 RISC-V Zicfiss）。 |
| ARCH_HAS_ZONE_DMA_SET | bool | 设备内存热插拔支持允许在 memmap 中建立 pmem 或其他由设备驱动发现的内存区域。这使得可对原本“设备物理”地址进行 pfn_to_page() 查找…… |
| ARCH_NO_SG_CHAIN | def_bool | 栈仓库（stack depot）：避免重复的栈跟踪存储 |
| ARCH_NO_SWAP | bool | 此选项让你选择内核是否支持所谓的交换设备（swap device）或交换文件（swap file），用于提供比实际物理 RAM 更多的虚拟内存…… |
| ARCH_SUPPORTS_HUGETLBFS | def_bool | hugetlbfs 是基于 ramfs 的 HugeTLB 页文件系统后端。支持的体系结构请在此选 Y，并阅读 <file:Documentation/admin-guide/mm/hugetlbpage.rst> 了解细节。若不确定…… |
| ARCH_SUPPORTS_KMAP_LOCAL_FORCE_MAP | bool | 此选项在非高端内存页及非高端内存系统上，强制通过 kmap_local 机制建立临时映射。生产系统请禁用！ |
| ARCH_SUPPORTS_MEMORY_FAILURE | bool | 在具备 MCA 恢复能力的系统上启用从部分内存故障中恢复的代码。即使部分内存存在未纠正错误，系统仍可继续运行。这需要特殊的硬…… |
| ARCH_SUPPORTS_NUMA_BALANCING | bool | 此选项添加对自动感知 NUMA 的内存/任务放置的支持。该机制较为原始，基于在内存引用到任务所运行的节点时进行迁移…… |
| ARCH_USE_MEMTEST | bool | 当某体系结构在引导过程中使用 early_memtest() 时，应选择此项。 |
| ARCH_WANT_FRAME_POINTERS | bool | 若选 Y，生成的内核镜像会稍大且稍慢，但在内核出错时可提供非常有用的调试信息（精确的 oops、栈跟踪、警告）。 |
| ARCH_WANT_GENERAL_HUGETLB | bool | 启用此选项可降低大零 folio（huge zero folio）的运行时引用计数开销，并扩展内核中可使用大零 folio 的位置。例如块 I/O 可从中受益…… |
| ASSOCIATIVE_ARRAY | bool | 通用关联数组。可在被修改的同时进行查找与遍历。其查找与修改也相当迅速。算法为非递归式，树结构较…… |
| ASYNC_RAID6_TEST | tristate | 这是一次性自检测试，会遍历 N 盘阵列所有可能的双盘故障场景进行恢复。恢复使用异步 raid6 恢复例程…… |
| AS_HAS_NON_CONST_ULEB128 | def_bool | 选择“None”以外的值会导致内核镜像包含调试信息，从而增大镜像体积。它会向内核与模块添加调试符号（gcc -g），并…… |
| ATOMIC64_SELFTEST | tristate | 启用此选项可在引导时或模块加载时测试 atomic64_t 函数。若不确定，选 N。 |
| AUDIT | bool | 启用审计基础设施，可与其他内核子系统（如 SELinux，其记录 avc 消息输出需要它）配合使用。系统调用审计包含于体系结构…… |
| BACKTRACE_SELF_TEST | tristate | 该选项提供一个内核模块，用于测试内核栈回溯代码。此选项对发行版或通用内核无用，仅对内核开发者…… |
| BASE64_KUNIT | tristate | 构建 base64 单元测试。测试覆盖内核中 Base64 函数的编码与解码逻辑。除正确性检查外，还对两种编码都进行了简单的性能基准测试…… |
| BASE_SMALL | bool | 启用此选项可缩减各类核心内核数据结构的大小。这在小型机器上节省内存，但可能降低性能。 |
| BCH_CONST_M | int | Galois 域阶数“m”的常数值。若“k”为要保护的位数，则“m”应满足 (k + m*t) <= 2**m - 1。驱动应为该符号声明默认值…… |
| BCH_CONST_T | int | 纠错能力（以比特为单位的“t”）的常数值。若驱动选择了 BCH_CONST_PARAMS 选项，则应为该符号声明默认值。# # 如需…… |
| BINARY_PRINTF | def_bool | 在初始化时对全部可用的 RAID6 PQ 函数进行基准测试，并选择最快的一个。 |
| BINDGEN_VERSION_TEXT | string | 回溯到每个体系结构各自定义 cpu_online_mask 与 cpu_possible_mask 的时代，其中一些将其初始化为全 1，另一些为全 0。当它们被集中化…… |
| BINFMT_ELF | bool | ELF（Executable and Linkable Format，可执行与可链接格式）是一种跨不同体系结构与操作系统使用的库与可执行文件格式。在此选 Y 将让你的内核能够运行 ELF 二进制文件…… |
| BINFMT_ELF_KUNIT_TEST | bool | 构建 ELF 加载器 KUnit 测试，尝试将以往的错误修复收集为回归测试集。这通常仅用于调试。注意在 CONFIG_COMPAT=y 时，compat_b…… |
| BINFMT_FLAT_ARGVP_ENVP_ON_STACK | bool | 支持十年前的 uClinux FLAT 格式二进制文件。除非你确定拥有此类文件，否则在此选 N。 |
| BINFMT_MISC | tristate | 若在此选 Y，便可向内核插入由包装器驱动的二进制格式。当你使用需要解释器才能运行的程序（如 Java、Python……）时会特别有用。 |
| BINFMT_SCRIPT | tristate | 若希望执行以 #! 开头并后跟解释器路径的脚本，请在此选 Y。你可以将其构建为模块；但在该模块加载之前，你无…… |
| BINFMT_ZFLAT | bool | 支持 FLAT 格式压缩二进制文件 |
| BITFIELD_KUNIT | tristate | 启用此选项可在引导时测试位域函数。KUnit 测试在引导期间运行，并以 TAP 格式（http://testanything.org/）将结果输出到调试日志。仅供内核开发者…… |
| BITOPS_KUNIT | tristate | 该选项启用 bitops 库的 KUnit 测试，提供位操作函数。注意它源自原始的 test_bitops 模块。用于微基准测试与编译…… |
| BITREVERSE | tristate | 该选项在某些支持此类操作的体系结构上启用硬件位反转指令。 |
| BITS_TEST | tristate | 构建 bits 单元测试。测试 bits.h 中定义的宏的逻辑。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-tools 中的 KUnit 文档…… |
| BLACKHOLE_DEV_KUNIT_TEST | tristate | 构建“blackhole_dev_kunit”模块，用于验证通过该黑洞网络设备的的数据路径。若不确定，选 N。 |
| BLK_CGROUP | bool | 通用块 I/O 控制器 cgroup 接口。这是各类 I/O 控制策略应使用的通用 cgroup 接口。当前 CFQ I/O 调度器用它来识别任务组…… |
| BLK_DEV_INITRD | bool | 初始 RAM 文件系统是由引导加载程序（loadlin 或 lilo）加载的 ramfs，并在正常引导流程之前挂载为根文件系统。它通常用于加载所需模块…… |
| BOOTPARAM_HUNG_TASK_PANIC | int | 当设为非零值时，若在单次扫描中发现的挂起任务数量达到该值，将触发内核 panic。该 panic 可与 panic_timeout 配合使用，以…… |
| BOOTPARAM_SOFTLOCKUP_PANIC | int | 设为非零值 N，使内核在出现“软锁死（soft lockup）”时 panic；软锁死是指导致内核在内核模式下循环超过 (N * 20 秒)（可使用 watchdo……配置）的缺陷。 |
| BOOTPARAM_WQ_STALL_PANIC | int | 设置触发内核 panic 的工作队列停滞次数。当工作线程池在超过 30 秒（可使用……配置）内对某个待处理工作项没有进展时，即发生工作队列停滞。 |
| BOOT_CONFIG | bool | 额外的引导配置允许系统管理员在内核引导时，将一份配置文件作为内核命令行参数的补充扩展传入。该引导配置文件必须以校验和形式附加在 initramfs 末尾，因…… |
| BOOT_CONFIG_EMBED | bool | 将 BOOT_CONFIG_EMBED_FILE 指定的 bootconfig 文件嵌入内核。通常 bootconfig 文件随 initrd 镜像加载。但若系统不支持 initrd，此选项会有所帮助…… |
| BOOT_CONFIG_EMBED_FILE | string | 指定将要嵌入内核的 bootconfig 文件。当 initrd 中没有，或 initrd 中没有其他 bootconfig 时，将使用此 bootconfig。 |
| BOOT_CONFIG_FORCE | bool | 设置此 Kconfig 选项后，即使省略“bootconfig”内核引导参数，也会执行 BOOT_CONFIG 处理。事实上，设置此选项后，无法使内核…… |
| BOOT_PRINTK_DELAY | bool | 该编译选项允许你在每条内核启动消息后插入一个短暂延迟来阅读它们。延迟在内核命令行上以毫秒为单位指定，使用 "boot_delay=N"。其... |
| BRIDGE_NETFILTER | tristate | 启用该选项将让 arptables 与 iptables 分别看到桥接的 ARP 与 IP 流量。如果你想要一个桥接防火墙，大概会希望启用该选项。启用或禁用该选项会... |
| BROKEN | bool | 该选项允许你选择是否尝试编译（并修复）尚未更新到新基础设施的旧驱动。 |
| BROKEN_ON_SMP | bool | 从内核命令行传递给 init 的参数数量与环境变量数量各自的最大值。 |
| BSD_PROCESS_ACCT | bool | 如果你在此选择 Y，用户级程序就能（通过特殊的系统调用）指示内核将进程记账信息写入一个文件：每当进程退出时，关于该进程... |
| BSD_PROCESS_ACCT_V3 | bool | 如果你在此选择 Y，进程记账信息将以一种新的文件格式写入，该格式还会记录每个进程及其父进程的进程 ID。注意该文件格式与... |
| BUG | bool | 禁用该选项会移除对 BUG 和 WARN 的支持，减小内核镜像体积，并可能悄然忽略大量致命状况。你只应在确有必要的情况下考虑禁用... |
| BUILD_SALT | string | 构建 ID 用于链接二进制文件及其调试信息。设置该选项将在构建 ID 的计算中使用该值。这对于希望确保... |
| BUILTIN_MODULE_RANGES | bool | 当模块被构建进内核时，其在 /proc/kallsyms 中的符号不会关联模块名。跟踪器可能希望不论... |
| CACHESTAT_SYSCALL | bool | 启用 cachestat 系统调用，它查询文件的页缓存统计信息（已缓存页数、脏页数、标记为回写的页、以及（最近）被逐出的页）。如果不确定，在此选择 Y。 |
| CC_IS_GCC | def_bool | 它不依赖于 `RUST`，因为后者可能需要在 `depends on` 中使用该版本。 |
| CC_OPTIMIZE_FOR_PERFORMANCE | bool | 这是内核的默认优化级别，使用 "-O2" 编译器标志进行构建，以获得最佳性能和最有帮助的编译期警告。 |
| CC_OPTIMIZE_FOR_SIZE | bool | 选择该选项会向编译器传递 "-Os"，从而生成更小的内核。 |
| CC_VERSION_TEXT | string | 其用途并不明确： - 当编译器更新时重新运行 Kconfig 'default' 属性引用环境变量 CC_VERSION_TEXT，因此它会被记录在 include/config/auto.conf... |
| CGROUP_BPF | bool | 允许使用 bpf(2) 系统调用命令 BPF_PROG_ATTACH 将 eBPF 程序附加到 cgroup。这些程序在何种上下文中被访问取决于附加的类型。例如，程序... |
| CGROUP_CPUACCT | bool | 提供一个简单的控制器，用于监控 cgroup 中任务消耗的总 CPU。 |
| CGROUP_DEBUG | bool | 该选项启用一个简单的控制器，导出关于 cgroups 框架的调试信息。该控制器仅用于 cgroup 调试。其接口不稳定。选择 N。 |
| CGROUP_DMEM | bool | DMEM 控制器允许兼容设备基于 cgroup 层级限制设备内存使用。例如，它允许你限制 DRM 子系统中应用程序的 VRAM 使用。 |
| CGROUP_FREEZER | bool | 提供一种冻结和解冻 cgroup 中所有任务的方法。该选项影响原始的 cgroup 接口。cgroup2 内存控制器默认包含重要的内核内内存消耗者... |
| CGROUP_HUGETLB | bool | 提供一个用于 HugeTLB 页的 cgroup 控制器。启用后，你可以对 HugeTLB 使用设置每个 cgroup 的限制。该限制在缺页时强制实施。由于 HugeTLB 不支持页重... |
| CGROUP_MISC | bool | 为主机上的杂项资源提供控制器。杂项标量资源是主机系统上无法像其他 cgroup 那样被抽象的资源。该控制器... |
| CGROUP_NET_CLASSID | bool | 用作通用套接字 classid 标记的 cgroup 子系统，用于 cls_cgroup 和 netfilter 匹配。 |
| CGROUP_PERF | bool | 该选项扩展 perf 的每 CPU 模式，将监控限制为属于指定 cgroup 并在指定 CPU 上运行的线程。或者可用于在采样中携带 cgroup ID，从而... |
| CGROUP_PIDS | bool | 在 cgroup 范围内强制实施进程数量限制。任何超出 cgroup 允许数量而 fork 更多进程的尝试都会失败。PID 从根本上是一种全局资源，因为... |
| CGROUP_RDMA | bool | 强制实施由 IB 栈定义的 RDMA 资源。使用者很容易耗尽 RDMA 资源，从而导致其他使用者无法获得资源。RDMA 控制... |
| CGROUP_WRITEBACK | bool | 该特性让 CPU 调度器识别任务组，并控制向这些任务组的 CPU 带宽分配。它使用 cgroup 将任务分组。依赖于 CGROUP_SCHED |
| CHECKPOINT_RESTORE | bool | 为检查点/恢复启用额外的内核特性。特别是它添加了辅助的 prctl 代码来设置进程文本、数据和堆段大小，以及少量额外的 /proc 文件... |
| CHECKSUM_KUNIT | tristate | Enable this option to test the checksum functions at boot. KUnit tests run during boot and output the results to the debug log in TAP format (http://testanything.org/). Only useful for kernel devs ... |
| CLOSURES | bool | 对 cpumask_var_t 使用动态分配，而不是将其放在栈上。这样开销略大，但可避免栈溢出。 |
| CMA_AREAS | int | CMA 允许为特定用途创建 CMA 区域，主要用作设备私有区域。该参数设置系统中 CMA 区域的最大数量。如果不确定，保留默认值 "8"... |
| CMA_DEBUGFS | bool | 开启 CMA 的 DebugFS 接口。 |
| CMA_SYSFS | bool | 该选项暴露一些 sysfs 属性，以便从 CMA 获取信息。 |
| CMDLINE_KUNIT_TEST | tristate | 该选项构建 cmdline API 单元测试，测试 cmdline.c 所提供的 API 逻辑。有关 KUnit 及单元测试的更多信息，请参阅 Documentation 中的 KUnit 文档... |
| CMDLINE_LOG_WRAP_IDEAL_LEN | int | 在启动时，内核命令行会被记录到控制台。日志消息以前缀 "Kernel command line: " 开头。该日志消息会尝试换行（拆分为多行... |
| CODE_TAGGING | bool | 跟踪分配源代码并记录在该代码位置发起的分配总大小。该机制可用于以较低的性能和内存开销跟踪内存泄漏。 |
| COMPACTION | bool | 内存规整（compaction）是唯一能可靠地形成高阶（更大的物理连续）内存块的内存管理组件。页分配器严重依赖内存规整，而该特性的缺失... |
| COMPACT_UNEVICTABLE_DEFAULT | int | 空闲页报告允许从伙伴分配器增量获取空闲页，以便将这些页报告给另一个实体（例如虚拟机监控器），从而使内存... |
| COMPAT_BINFMT_ELF | def_bool | ELF FDPIC 二进制基于 ELF，但允许二进制各个加载段彼此独立地位于内存中。这使得该格式非常适用于... |
| COMPAT_BRK | bool | 随机化堆布局使堆漏洞利用更困难，但也会破坏古老二进制（包括任何基于 libc5 的程序）。该选项将启动默认值改为禁用堆随机化，从... |
| COMPAT_NETLINK_MESSAGES | def_bool | 该选项使得能够根据任务是否为兼容（compat）任务，向任务发送不同的 netlink 消息。为此，你需要将 skb_shinfo(skb)->frag_list 设置为... |
| COMPILE_TEST | bool | 某些驱动可以在与其预期运行平台不同的平台上编译。尽管它们无法在那里加载（或者即使加载也会因缺少硬件支持而无法使用），... |
| CONSOLE_LOGLEVEL_DEFAULT | int | 用于决定在控制台上打印内容的默认日志级别。在此设置默认值等价于在内核启动参数中传入 loglevel=<x>。loglevel=<x> 仍会覆盖... |
| CONSOLE_LOGLEVEL_QUIET | int | 当内核命令行传入 "quiet" 时使用的日志级别。当内核命令行传入 "quiet" 时，将使用该日志级别。换言之，传入 "quiet" 等价于... |
| CONTEXT_ANALYSIS_TEST | bool | 该选项构建针对基于编译器的上下文分析的测试。该测试不会向内核添加可执行代码，而是用于测试分析所支持的通用模式不会导致... |
| CONTIG_ALLOC | def_bool | 在页分配器中，PCP（每 CPU 页集）以批方式补充和清空。批数量会自动缩放以提高页分配/释放吞吐。但过大的缩放因子可能损... |
| COREDUMP | bool | 该选项启用对生成核心转储的支持。你几乎肯定应该在此选择 Y。对于从不需要调试或只运行无瑕代码的系统则非必需。 |
| CORE_DUMP_DEFAULT_ELF_HEADERS | bool | ELF 核心转储文件描述崩溃进程的每个内存映射，并可包含或省略每个映射的内存内容。未修改的文本映射内容默认被省略。... |
| CPUMASK_KUNIT_TEST | tristate | 启用 cpumask 测试，在启动或模块加载时运行。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-tools/kunit 中的 KUnit 文档... |
| CPUSETS | bool | 该选项允许你创建和管理 CPUSET，从而将系统动态划分为 CPU 和内存节点的集合，并指派任务仅在这些集合内运行。这主要... |
| CPUSETS_V1 | bool | 已被 cgroup v2 实现弃用的传统 cgroup v1 cpusets 控制器。v1 保留用于尚未迁移到新 cgroup v2 接口的传统应用。传统... |
| CPU_HOTPLUG_STATE_CONTROL | bool | 允许向 CPU 的 sysfs 目标文件写入 "offline" 与 "online" 之间的阶梯步骤，从而可以细粒度地逐步切换状态。目前这是一个调试选项，因为热插拔机制无法停止和重... |
| CPU_ISOLATION | bool | 确保运行关键任务的 CPU 不受任何"噪声"源干扰，例如未绑定的工作队列、定时器、内核线程等。未绑定的任务会被卸载到管家（housekeeping）CPU 上。这由... |
| CROSS_MEMORY_ATTACH | bool | 启用该选项会添加 process_vm_readv 与 process_vm_writev 系统调用，允许具有相应权限的进程直接读取或写入另一个进程的地址空间。... |
| CRYPTO | tristate | 该选项提供核心加密 API。依赖于 CRYPTO |
| CRYPTO_842 | tristate | 842 compression algorithm by IBM See https://github.com/plauth/lib842 for further information. |
| CRYPTO_ADIANTUM | tristate | Adiantum 可调整、保长的加密模式。设计用于快速且安全的磁盘加密，尤其适用于没有专用加密指令的 CPU。它使用 XCha... |
| CRYPTO_AEGIS128 | tristate | AEGIS-128 AEAD 算法 |
| CRYPTO_AEGIS128_SIMD | bool | AEGIS-128 AEAD 算法。架构：arm 或 arm64，使用： - NEON（高级 SIMD）扩展 |
| CRYPTO_AES | tristate | AES 加密算法（Rijndael）（FIPS-197、ISO/IEC 18033-3）。Rijndael 在广泛的硬件与软件计算环境中都始终表现优异... |
| CRYPTO_ALGAPI | tristate | 该选项提供加密算法的 API。 |
| CRYPTO_ALGAPI2 | tristate | 这提供了实例化诸如 cbc(aes) 等模板的支持，以及加密自测的支持。 |
| CRYPTO_ANUBIS | tristate | Anubis cipher algorithm Anubis is a variable key length cipher which can use keys from 128 bits to 320 bits in length.  It was evaluated as a entrant in the NESSIE competition. See https://web.arch... |
| CRYPTO_ARC4 | tristate | ARC4 加密算法。ARC4 是一种流密码，使用长度从 8 位到 2048 位的密钥。该算法是基于驱动的 WEP 所必需的，但不应被用于其他目的，因为... |
| CRYPTO_ARIA | tristate | ARIA 加密算法（RFC5794）。ARIA 是韩国标准加密算法。ARIA 规定了三种密钥长度与轮数：128 位 12 轮，192 位 14 轮，256 位 16... |
| CRYPTO_AUTHENC | tristate | Authenc：用于 IPsec 的组合模式封装。这是 IPSec ESP（XFRM_ESP）所必需的。 |
| CRYPTO_BENCHMARK | tristate | 粗略的加密基准测试模块。主要供在内核中开发加密算法的人员使用。不应在生产内核中启用。 |
| CRYPTO_BLAKE2B | tristate | BLAKE2b 加密哈希函数（RFC 7693）。BLAKE2b 针对 64 位平台优化，可生成 1 到 64 字节之间任意大小的摘要。密钥化哈希也已实现。该模块... |
| CRYPTO_BLOWFISH | tristate | Blowfish 加密算法，由 Bruce Schneier 设计。这是一种可变密钥长度的密码，可使用 32 位到 448 位的密钥。它快速、简单，专为在"大型... |
| CRYPTO_BLOWFISH_COMMON | tristate | Blowfish 加密算法由通用 C 实现与汇编实现共享的通用部分。 |
| CRYPTO_CAMELLIA | tristate | Camellia 加密算法（ISO/IEC 18033-3）。Camellia 是由 NTT 与三菱电机联合开发的对称密钥分组密码。Camellia 规定了三种密钥长度：128、192... |
| CRYPTO_CAST5 | tristate | CAST5（CAST-128）加密算法（RFC2144、ISO/IEC 18033-3） |
| CRYPTO_CAST6 | tristate | CAST6（CAST-256）加密算法（RFC2612） |
| CRYPTO_CAST_COMMON | tristate | CAST 加密算法由通用 C 实现与汇编实现共享的通用部分。 |
| CRYPTO_CBC | tristate | CBC（密码块链接）模式（NIST SP800-38A）。该分组密码模式是 IPSec ESP（XFRM_ESP）所必需的。 |
| CRYPTO_CCM | tristate | CCM（计数器与密码块链接-消息认证码）认证加密模式（NIST SP800-38C） |
| CRYPTO_CHACHA20 | tristate | ChaCha20、XChaCha20 与 XChaCha12 流密码算法。ChaCha20 是由 Daniel J. Bernstein 设计的 256 位高速流密码，并在 RFC7539 中进一步规定用于 IETF 协议... |
| CRYPTO_CHACHA20POLY1305 | tristate | ChaCha20 流密码与 Poly1305 认证器组合模式（RFC8439） |
| CRYPTO_CMAC | tristate | CMAC（基于密码的消息认证码）认证模式（NIST SP800-38B 与 IETF RFC4493） |
| CRYPTO_CRC32 | tristate | CRC32 CRC 算法（IEEE 802.3） |
| CRYPTO_CRC32C | tristate | CRC32c CRC 算法，使用 iSCSI 多项式（RFC 3385 与 RFC 3720）。这是一种 32 位 CRC（循环冗余校验），其多项式由 G. Castagnoli、S. Braeuer 与 M. Herrman 在"Optimization... |
| CRYPTO_CRYPTD | tristate | 这是一个通用的软件异步加密守护进程，可将任意同步软件加密算法转换为在内核线程中执行的异步算法。 |
| CRYPTO_CTR | tristate | CTR（计数器）模式（NIST SP800-38A） |
| CRYPTO_CTS | tristate | CTS（密文窃取）的 CBC-CS3 变体（NIST 对 SP800-38A 的附录（2010 年 10 月））。该模式是 Kerberos gss 机制支持 AES 加密所必需的。 |
| CRYPTO_DEFLATE | tristate | Deflate 压缩算法（RFC1951）。由 IPSec 配合 IPCOMP 协议（RFC3173、RFC2394）使用。 |
| CRYPTO_DES | tristate | DES（数据加密标准）（FIPS 46-2、ISO/IEC 18033-3）与三重 DES EDE（加密/解密/加密）（FIPS 46-3、ISO/IEC 18033-3）加密算法。 |
| CRYPTO_DH | tristate | DH（Diffie-Hellman）密钥交换算法。 |
| CRYPTO_DH_RFC7919_GROUPS | bool | RFC7919 中定义的 FFDHE（基于有限域的 Diffie-Hellman 临时）组。在 DH 密钥交换中支持这些有限域组： - ffdhe2048、ffdhe3072、ffdhe4096、ffdhe6144、ffdhe8192。如果不确定... |
| CRYPTO_DRBG | tristate | 来自 Jitterentropy 库的 CPU 抖动 RNG（随机数生成器）。一种非物理、非确定性的（"真"）RNG（例如符合 NIST SP800-90B 的熵源），旨在提供... |
| CRYPTO_DRBG_CTR | bool | NIST SP800-90A 中定义的 CTR_DRBG 变体。它使用 AES 加密算法与计数器分组模式。 |
| CRYPTO_DRBG_HMAC | bool | NIST SP800-90A 中定义的 Hash_DRBG 变体。它使用 SHA-1、SHA-256、SHA-384 或 SHA-512 哈希算法。 |
| CRYPTO_DRBG_MENU | tristate | DRBG（确定性随机比特生成器）（NIST SP800-90A）。在下面的子菜单中，必须选择一种或多种 DRBG 类型。依赖于 CRYPTO_DRBG_MENU |
| CRYPTO_ECB | tristate | ECB（电子密码本）模式（NIST SP800-38A）。 |
| CRYPTO_ECC | tristate | 使用 P-192、P-256 与 P-384 曲线（FIPS 186）的 ECDH（椭圆曲线 Diffie-Hellman）密钥交换算法。 |
| CRYPTO_ECDSA | tristate | ECDSA（椭圆曲线数字签名算法）（FIPS 186、ISO/IEC 14888-3），使用 P-192、P-256、P-384 与 P-521 曲线。仅实现签名验证。 |
| CRYPTO_ECHAINIV | tristate | 加密链 IV 生成器。该 IV 生成器基于对序列号与盐异或后再加密来生成 IV。这是 CBC 的默认算法。 |
| CRYPTO_ECRDSA | tristate | 椭圆曲线俄罗斯数字签名算法（GOST R 34.10-2012、RFC 7091、ISO/IEC 14888-3）。俄罗斯加密标准算法之一（称为 GOST 算法）。仅实现签名验证... |
| CRYPTO_ESSIV | tristate | 加密盐-扇区 IV 生成器。该 IV 生成器在某些场景下被 fscrypt 和/或 dm-crypt 使用。它使用块加密密钥的哈希作为块加密遍的对称密钥... |
| CRYPTO_FCRYPT | tristate | FCrypt algorithm used by RxRPC See https://ota.polyonymo.us/fcrypt-paper.txt |
| CRYPTO_FIPS | bool | 该选项启用 fips 启动选项，如果你希望系统在 FIPS 200 认证下运行则需要它。除非你知道它的含义，否则应选择否。 |
| CRYPTO_FIPS_CUSTOM_VERSION | bool | 该选项提供覆盖 FIPS 模块版本的能力。默认使用 KERNELRELEASE 值。 |
| CRYPTO_FIPS_NAME | string | 该选项设置由 Crypto API 通过 /proc/sys/crypto/fips_name 文件报告的 FIPS 模块名称。 |
| CRYPTO_GCM | tristate | GCM（Galois/计数器模式）认证加密模式与 GMAC（GCM 消息认证码）（NIST SP800-38D）。这是 IPSec ESP（XFRM_ESP）所必需的。 |
| CRYPTO_GENIV | tristate | 序列号 IV 生成器。该 IV 生成器通过把序列号与盐异或来生成 IV。该算法主要用于 CTR。这是 IPsec ESP（XFRM_ESP）所必需的。 |
| CRYPTO_HCTR2 | tristate | HCTR2 保长加密模式。一种用于存储加密的模式，在带有加速 AES 与无进位乘法指令的处理器（例如带有 AES-...的 x86 处理器）上高效。 |
| CRYPTO_HMAC | tristate | HMAC（带密钥的哈希消息认证码）（FIPS 198 与 RFC2104）。这是 IPsec AH（XFRM_AH）与 IPsec ESP（XFRM_ESP）所必需的。 |
| CRYPTO_JITTERENTROPY_MEMORY_BLOCKS | int | Enable the userspace interface for hash algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_JITTERENTROPY_MEMSIZE_2 | bool | Jitter RNG 允许指定过采样率（OSR）。Jitter RNG 的运行需要固定数量的定时测量来产生一个输出随机区块。OSR... |
| CRYPTO_JITTERENTROPY_TESTINTERFACE | bool | 测试接口允许特权进程捕获 Jitter RNG 收集的、用于统计分析的原始未调理高分辨率时间戳噪声。由于该数据被用于... |
| CRYPTO_KHAZAD | tristate | Khazad 加密算法。Khazad 是初始 NESSIE 竞赛的决赛算法。它是一种为 64 位处理器优化、在 32 位处理器上也有良好表现的算法。Khazad 使用 128... |
| CRYPTO_KRB5ENC | tristate | 用于 Kerberos 5 RFC3961 简化配置文件的哈希与加密组合支持。这是 sunrpc/NFS 与 rxrpc/AFS 所使用的 Kerberos 5 风格加密所必需的。 |
| CRYPTO_LRW | tristate | LRW（Liskov Rivest Wagner）模式。一种用于 dm-crypt 的可调整、不可塑、不可移动的窄块密码模式。与密码规格字符串 aes-lrw-benbi 一起使用，密钥必须为 256、320 或 38... |
| CRYPTO_LZ4 | tristate | LZ4 compression algorithm See https://github.com/lz4/lz4 for further information. |
| CRYPTO_LZ4HC | tristate | LZ4 high compression mode algorithm See https://github.com/lz4/lz4 for further information. |
| CRYPTO_LZO | tristate | LZO compression algorithm See https://www.oberhumer.com/opensource/lzo/ for further information. |
| CRYPTO_MANAGER2 | def_tristate | 针对 cbc(aes) 等加密实例的用户空间配置。 |
| CRYPTO_MD4 | tristate | MD4 消息摘要算法（RFC1320）。 |
| CRYPTO_MD5 | tristate | MD5 消息摘要算法（RFC1321），包含 HMAC 支持。 |
| CRYPTO_MLDSA | tristate | ML-DSA（基于模块格的数字签名算法）（FIPS-204）。仅实现签名验证。 |
| CRYPTO_NULL | tristate | 这些是 IPsec 使用的"空"算法，它们不做任何事情。 |
| CRYPTO_PCBC | tristate | PCBC（传播密码块链接）模式。该分组密码模式是 RxRPC 所必需的。 |
| CRYPTO_PCRYPT | tristate | 该选项将任意加密算法转换为在内核线程中执行的并行算法。 |
| CRYPTO_RMD160 | tristate | RIPEMD-160 哈希函数（ISO/IEC 10118-3）。RIPEMD-160 是一种 160 位加密哈希函数。它旨在作为 128 位哈希函数 MD4、MD5 及其前身的安全替代... |
| CRYPTO_SEED | tristate | SEED 加密算法（RFC4269、ISO/IEC 18033-3）。SEED 是一种 128 位对称密钥分组密码，由 KISA（韩国信息安全院）作为国家加密标准算法开发... |
| CRYPTO_SELFTESTS | bool | 启用加密自测。加密自测在启动时运行，或在算法稍后动态加载时于算法注册时运行。这有两个主要用例... |
| CRYPTO_SELFTESTS_FULL | bool | 启用每种算法的完整加密自测集。完整测试集应启用于开发与发布前测试，而不应在生产内核中启用。所有加密代码... |
| CRYPTO_SERPENT | tristate | Serpent cipher algorithm, by Anderson, Biham & Knudsen Keys are allowed to be from 0 to 256 bits in length, in steps of 8 bits. See https://www.cl.cam.ac.uk/~rja14/serpent.html for further informat... |
| CRYPTO_SHA1 | tristate | SHA-1 安全哈希算法（FIPS 180、ISO/IEC 10118-3），包含 HMAC 支持。 |
| CRYPTO_SHA256 | tristate | SHA-224 与 SHA-256 安全哈希算法（FIPS 180、ISO/IEC 10118-3），包含 HMAC 支持。这是 IPsec AH（XFRM_AH）与 IPsec ESP（XFRM_ESP）所必需的。 |
| CRYPTO_SHA3 | tristate | SHA-3 安全哈希算法（FIPS 202、ISO/IEC 10118-3）。 |
| CRYPTO_SHA512 | tristate | SHA-384 与 SHA-512 安全哈希算法（FIPS 180、ISO/IEC 10118-3），包含 HMAC 支持。 |
| CRYPTO_SIMD | tristate | RSA（Rivest-Shamir-Adleman）公钥算法（RFC8017）。 |
| CRYPTO_SM3 | tristate | SM3 (ShangMi 3) secure hash function (OSCCA GM/T 0004-2012, ISO/IEC 10118-3) This is part of the Chinese Commercial Cryptography suite. References: http://www.oscca.gov.cn/UpFile/20101222141857786.... |
| CRYPTO_SM4 | tristate | SM4 加密算法（OSCCA GB/T 32907-2016、ISO/IEC 18033-3:2010/Amd 1:2021）。SM4（GBT.32907-2016）是由中国国家商用密码管理办公室发布的密码标准... |
| CRYPTO_STREEBOG | tristate | Streebog 哈希函数（GOST R 34.11-2012、RFC 6986、ISO/IEC 10118-3）。这是俄罗斯加密标准算法之一（称为 GOST 算法）。该设置启用两种哈希算法... |
| CRYPTO_TEA | tristate | TEA（微型加密算法）加密算法。微型加密算法是一种使用多轮以保证安全性的简单密码。它非常快且占用内存少。扩展微型加密... |
| CRYPTO_TWOFISH | tristate | Twofish 加密算法。Twofish 由 CounterPane Systems 的研究人员作为 AES（高级加密标准）候选密码提交。它是一种 16 轮分组密码，支持...的密钥长度。 |
| CRYPTO_TWOFISH_COMMON | tristate | Twofish 加密算法由通用 C 实现与汇编实现共享的通用部分。 |
| CRYPTO_USER_API_AEAD | tristate | Enable the userspace interface for AEAD cipher algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_ENABLE_OBSOLETE | bool | 允许选择已被内核内部使用逐步淘汰、仅对仍依赖它们的用户空间客户端有用的过时加密算法。 |
| CRYPTO_USER_API_RNG | tristate | Enable the userspace interface for RNG (random number generator) algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_RNG_CAVP | bool | 在用户空间接口中为 NIST CAVP（加密算法验证程序）测试启用额外 API： - 重置 DRBG 熵 - 提供附加数据。这只应... |
| CRYPTO_USER_API_SKCIPHER | tristate | Enable the userspace interface for symmetric key cipher algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_WP512 | tristate | Whirlpool hash function (ISO/IEC 10118-3) 512, 384 and 256-bit hashes. Whirlpool-512 is part of the NESSIE cryptographic primitives. See https://web.archive.org/web/20171129084214/http://www.larc.u... |
| CRYPTO_XCBC | tristate | XCBC-MAC（扩展密码块链接消息认证码）（RFC3566）。 |
| CRYPTO_XCTR | tristate | 用于 HCTR2 的 XCTR（异或计数器）模式。该分组密码模式是 CTR 模式的变体，使用异或与小端加法而非大端算术。XCTR 模式用于实现 HCTR2。 |
| CRYPTO_XTS | tristate | XTS（异或加密异或并窃取密文）模式（NIST SP800-38E 与 IEEE 1619）。与 aes-xts-plain 一起使用，密钥长度 256、384 或 512 位。该实现目前无法处理...的扇区大小。 |
| CRYPTO_XXHASH | tristate | xxHash 非加密哈希算法。极其快速，速度接近 RAM 极限。 |
| CRYPTO_ZSTD | tristate | zstd compression algorithm See https://github.com/facebook/zstd for further information. |
| CSD_LOCK_WAIT_DEBUG | bool | 该选项在 CPU 对 smp_call_function*() IPI 包装函数响应缓慢时启用调试打印。这些调试打印包括当前正在执行的 IPI 处理函数（如果有）以及相关的... |
| CSD_LOCK_WAIT_DEBUG_DEFAULT | bool | 该选项使 csdlock_debug= 内核启动参数默认为 1（基本调试）而非 0（无调试）。 |
| DCACHE_WORD_ACCESS | bool | 启用该选项可在文件系统注册时对其参数描述进行校验。 |
| DEBUG_ATOMIC | bool | 如果你在此选择 Y，内核将为原子访问添加运行时对齐检查。对没有未对齐访问陷阱的体系结构很有用。该选项可能有显著的... |
| DEBUG_ATOMIC_LARGEST_ALIGN | bool | 如果你在此选择 Y，对原子访问自然对齐的检查将被限制为编译器对标量类型的最大对齐。 |
| DEBUG_ATOMIC_SLEEP | bool | 如果你在此选择 Y，各种可能休眠的例程如果在原子区间内被调用会变得非常嘈杂：当持有自旋锁时、在 rcu 读侧临界区内、在抢占禁用时... |
| DEBUG_BUGVERBOSE | bool | 在此选择 Y 可使 BUG() 恐慌同时输出 BUG 调用的文件名与行号，以及 EIP 与 oops 跟踪。这有助于调试，但耗费约 70-100K 内存。 |
| DEBUG_BUGVERBOSE_DETAILED | bool | 在此选择 Y 可使 WARN_ON_ONCE() 除文件名与行号外，还输出警告的条件字符串。这有助于调试，但耗费约 100K 内存。如果不确定选择 N。 |
| DEBUG_CGROUP_REF | bool | 强制 cgroup css 引用计数函数不被内联，以便它们可以被 kprobe 用于调试。 |
| DEBUG_CLOSURES | bool | 将所有活动的 closure 保留在一个链表中，并提供 debugfs 接口来列出它们，从而可以查看卡住的异步操作。 |
| DEBUG_FORCE_FUNCTION_ALIGN_64B | bool | 存在这样的情况：一个域的提交会改变其他域的函数地址对齐，并导致神奇的性能突变（回归或提升）。启用该选项将有助于... |
| DEBUG_FORCE_WEAK_PER_CPU | bool | s390 与 alpha 要求模块中的 percpu 变量被定义为弱符号，以规避寻址范围问题，这对 percpu 变量定义施加了以下两条限制。1. percpu 符号... |
| DEBUG_FS | bool | debugfs 是内核开发者用来放置调试文件的虚拟文件系统。启用该选项以便能够读写这些文件。有关 debugfs 的详细文档... |
| DEBUG_FS_ALLOW_ALL | bool | 无限制。API 与文件系统注册均开启。这是正常的默认操作。 |
| DEBUG_FS_ALLOW_NONE | bool | 访问关闭。客户端在尝试在 debugfs 树中创建节点时得到 -PERM，且 debugfs 未注册为文件系统。客户端随后可退避或在没有 debugfs 访问的情况下继续。 |
| DEBUG_HIGHMEM | bool | 该选项启用针对高内存系统的额外错误检查。在生产系统上禁用。 |
| DEBUG_INFO | bool | 在下面的"调试信息"选项中选择了"无"以外的内核调试信息选项，表明将为构建目标生成调试信息。# Clang 生成 .ule... |
| DEBUG_INFO_BTF | bool | 从 DWARF 调试信息生成去重后的 BTF 类型信息。开启它需要 pahole v1.22 或更高版本，它将把 DWARF 类型信息转换为等价的去重 BTF 类型信息。 |
| DEBUG_INFO_BTF_MODULES | bool | 为内核模块生成紧凑的拆分 BTF 类型信息。 |
| DEBUG_INFO_COMPRESSED_NONE | bool | 不压缩调试信息节。 |
| DEBUG_INFO_COMPRESSED_ZLIB | bool | 使用 zlib 压缩调试信息。通过 debian/rules 使用 dpkg-deb 的用户可能会发现，由于调试信息被压缩，其调试 .deb 包的体积会增大... |
| DEBUG_INFO_COMPRESSED_ZSTD | bool | 使用 zstd 压缩调试信息。这可能提供比 zlib 更好的压缩率，耗时大致相当，但需要较新的工具链支持。需要 GCC 13.0+ 或 Clang 16.0+，但... |
| DEBUG_INFO_DWARF4 | bool | 生成 DWARF v4 调试信息。这需要 gcc 4.5+、若使用不带 clang 集成汇编器的 clang 则需要 binutils 2.35.2，以及 gdb 7.0+。如果你有尚未准备好...的 DWARF 调试信息使用者。 |
| DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT | bool | 工具链产生的 DWARF 调试信息的隐式默认版本会随时间变化。这可能破坏尚未升级以支持更新版本的调试信息使用者，并阻止... |
| DEBUG_INFO_NONE | bool | 不以内核调试信息构建，这将产生更快、更小的构建。 |
| DEBUG_INFO_SPLIT | bool | 将调试信息生成到独立的 .dwo 文件中。这显著减小了带 DEBUG_INFO 构建的构建目录体积，因为它只在磁盘上的 .dwo 文件中存储一次信息，而非... |
| DEBUG_IRQFLAGS | bool | 启用对可能不安全的开/关中断的检查，例如在中断已启用时调用 raw_local_irq_restore()。 |
| DEBUG_KERNEL | bool | 如果你在开发驱动或试图调试并定位内核问题，在此选择 Y。 |
| DEBUG_KMAP_LOCAL | bool | 该选项为 kmap_local 基础设施启用额外的错误检查。生产环境请禁用。 |
| DEBUG_KOBJECT | bool | 如果你在此选择 Y，一些额外的 kobject 调试消息将被发送到 syslog。 |
| DEBUG_KOBJECT_RELEASE | bool | kobject 是引用计数的对象。这意味着它们的最后一次引用计数释放是不可预测的，并且 kobject 可能在驱动决定丢弃其初始... |
| DEBUG_LOCKDEP | bool | 如果你在此选择 Y，锁依赖引擎将做额外的运行时检查以自我调试，代价是更多的运行时开销。 |
| DEBUG_LOCKING_API_SELFTESTS | bool | 如果你希望内核在启动期间运行一个简短的自测，在此选择 Y。该自测检查常见的各类锁 Bug 是否被调试机制检测到（如果你禁用锁...）。 |
| DEBUG_LOCK_ALLOC | bool | 该特性将通过任何内存释放例程（kfree()、kmem_cache_free()、free_pages()...）检查是否有任何被持有的锁（自旋锁、rwlock、mutex 或 rwsem）被内核错误地释放。 |
| DEBUG_MAPLE_TREE | bool | 启用 maple tree 调试信息与额外校验。如果不确定，选择 N。 |
| DEBUG_MEMORY_INIT | bool | 启用该选项以在内存初始化期间进行额外检查。健全性检查验证 VM 的各个方面，例如内存模型以及体系结构提供的其他信息。详细信息... |
| DEBUG_MISC | bool | 如果你需要启用本应归入更具体的调试选项但并非如此的杂项调试代码，在此选择 Y。 |
| DEBUG_MUTEXES | bool | 该特性允许检测并报告 mutex 语义违规。 |
| DEBUG_NOMMU_REGIONS | bool | 该特性使匿名与私有映射区域的全局树被定期检查是否存在无效拓扑。 |
| DEBUG_NOTIFIERS | bool | 启用该选项以开启对通知链（notifier call chain）的健全性检查。这对内核开发者确保模块正确地从通知链注销最为有用。这是一个... |
| DEBUG_OBJECTS | bool | 如果你在此选择 Y，内核中将插入额外代码来跟踪各类对象的生命周期，并校验对这些对象的操作。 |
| DEBUG_OBJECTS_ENABLE_DEFAULT | int | 调试对象的启动参数默认值。 |
| DEBUG_OBJECTS_FREE | bool | 该选项启用检查：k/v 释放操作是否释放了一个包含尚未被正确停用的对象的区域。这可能使 kmalloc/kfree 密集的工作负载明显变慢。 |
| DEBUG_OBJECTS_PERCPU_COUNTER | bool | 如果你在此选择 Y，percpu 计数器例程中将插入额外代码，以跟踪 percpu 计数器对象的生命周期并校验 percpu 计数器操作。 |
| DEBUG_OBJECTS_RCU_HEAD | bool | 启用该选项以开启对 RCU 链表头（call_rcu() 用法）的调试。 |
| DEBUG_OBJECTS_SELFTEST | bool | 该选项启用对象调试代码的自测。 |
| DEBUG_OBJECTS_TIMERS | bool | 如果你在此选择 Y，定时器例程中将插入额外代码，以跟踪定时器对象的生命周期并校验定时器操作。 |
| DEBUG_OBJECTS_WORK | bool | 如果你在此选择 Y，工作队列例程中将插入额外代码，以跟踪工作对象的生命周期并校验工作操作。 |
| DEBUG_PERF_USE_VMALLOC | bool | 使用 vmalloc 内存作为 perf mmap() 缓冲区的后备。主要用于在不需要它的平台上调试 vmalloc 代码。如果不确定选择 N。 |
| DEBUG_PER_CPU_MAPS | bool | 选择 Y 以验证正在访问的 per_cpu 映射已被建立。这会向内核内存添加相当多的代码并降低性能。如果不确定选择 N。 |
| DEBUG_PLIST | bool | 启用该选项以开启在按优先级排序的链表（plist）遍历例程中的扩展检查。它会在每次操作时多次检查整个链表。如果不确定，选择 N。 |
| DEBUG_PREEMPT | bool | 如果你在此选择 Y，内核将使用常用 smp_processor_id() 函数的调试变体，并在内核代码以不安全抢占的方式使用它时打印警告。此外，内核... |
| DEBUG_RSEQ | bool | 为 rseq 系统调用启用额外的调试检查。如果不确定，选择 N。 |
| DEBUG_RT_MUTEXES | bool | 该选项允许自动检测并报告 rt mutex 语义违规以及 rt mutex 相关的死锁（lockup）。 |
| DEBUG_RWSEMS | bool | 该调试特性允许检测并报告不匹配的读写信号量加锁与解锁。 |
| DEBUG_SECTION_MISMATCH | bool | 节区不匹配分析检查是否存在从一个节区到另一个节区的非法引用。在链接时或运行时，某些节区会被丢弃；任何对这些节区中原有代码/数据的使用... |
| DEBUG_SG | bool | 启用该选项以开启对分散-聚集（scatter-gather）表的检查。这有助于发现未正确初始化其 sg 表的驱动问题。如果不确定，选择 N。 |
| DEBUG_SHIRQ | bool | 启用该选项以在共享中断处理程序注销前生成一个伪中断（注册时生成当前被禁用）。驱动需要正确处理它。如果不确定... |
| DEBUG_SPINLOCK | bool | 在此选择 Y 并构建 SMP 以捕获缺失的自旋锁初始化以及常犯的其他自旋锁错误。最好与 NMI 看门狗配合使用，以便自旋锁... |
| DEBUG_STACK_USAGE | bool | 启用在 sysrq-T 与 sysrq-P 调试输出中显示每个任务曾经可用的最小空闲栈量。当进程退出时，如果该进程...也会向 dmesg 发送一条消息。 |
| DEBUG_VFS | bool | 启用该选项以开启 VFS 层中可能影响性能的扩展检查。如果不确定，选择 N。 |
| DEBUG_VM_IRQSOFF | def_bool | 启用该选项以开启虚拟内存系统中可能影响性能的扩展检查。如果不确定，选择 N。 |
| DEBUG_VM_MAPLE_TREE | bool | 启用 VM maple tree 调试信息与额外校验。如果不确定，选择 N。 |
| DEBUG_VM_PGFLAGS | bool | 在页标志（page flags）操作上启用额外校验。如果不确定，选择 N。 |
| DEBUG_VM_PGTABLE | bool | 该选项提供一种调试方法，可用于在各种平台上测试体系结构的页表辅助函数是否符合预期的通用 MM 语义。这将... |
| DEBUG_VM_RB | bool | 启用 VM 红黑树调试信息与额外校验。如果不确定，选择 N。 |
| DEBUG_VM_SHOOT_LAZIES | bool | 启用额外的 IPI，以确保在使用完 mm 之前移除惰性 tlb mm 引用。如果不确定，选择 N。 |
| DEBUG_WQ_FORCE_RR_CPU | bool | 工作队列过去隐式保证：未指定具体 CPU 入队的工作项会被放到本地 CPU 上。该保证已不再成立，虽然本地 CPU 仍被优先... |
| DEBUG_WW_MUTEX_SLOWPATH | bool | 该特性通过注入额外的 -EDEADLK 伤害/退避情形，为 w/w mutex 使用者启用慢速路径测试。配合由 (CONFIG_PROVE_LOCKING) 启用的完整 mutex 检查，这将测试... |
| DEFAULT_HOSTNAME | string | 该选项决定在用户空间调用 sethostname(2) 之前系统的默认主机名。内核传统上使用 "(none)"，但你可能希望使用不同的默认值，以构成一个最小... |
| DEFAULT_HUNG_TASK_TIMEOUT | int | 该选项控制在判定任务变为无响应并应被视为挂起时所用的默认超时（秒）。它可以在运行时通过 kernel.hung_task_t... 调整。 |
| DEFAULT_INIT | string | 该选项决定当内核命令行未传入 init= 选项时系统的默认 init。如果请求的路径不存在，我们仍将继续尝试进一步的... |
| DEFAULT_MMAP_MIN_ADDR | int | 这是应当免受用户空间分配影响的低虚拟内存部分。阻止用户写入低地址页有助于减少内核空指针 Bug 的影响。对于... |
| DEFAULT_SECURITY_SELINUX | bool | 一个以逗号分隔的 LSM 列表，按初始化顺序排列。任何未列入此列表的 LSM，除顺序为 LSM_ORDER_FIRST 与 LSM_ORDER_LAST 者外（若在内核中选中的话总是启用），... |
| DEFERRED_STRUCT_PAGE_INIT | bool | 通常所有 struct page 都在早期启动期间由单线程初始化。在非常大的机器上这可能耗费相当多的时间。若设置该选项，大型机器将带... |
| DETECT_HUNG_TASK | bool | 在此选择 Y 以让内核检测"挂起任务"，即导致任务无限期卡在不可中断的 "D" 状态的 Bug。当检测到挂起任务时，内核将打印... |
| DETECT_HUNG_TASK_BLOCKER | bool | 在此选择 Y 以显示获取了"挂起任务"所等待的 mutex 锁的阻塞任务栈跟踪。这会增加少量开销，但若其来自...会显示可疑任务与调用栈。 |
| DIMLIB | tristate | 动态中断调节库。实现一种根据运行时性能动态改变 CQ 调节值的算法。# # libfdt 文件，仅在需要时选中。# |
| DST_CACHE | bool | NET_SOCK_MSG 为普通套接字（例如 TCP）或 ULP（上层模块，例如 TLS）提供一个借助 BPF 程序处理 L7 应用数据的框架。 |
| DYNAMIC_DEBUG | bool | 将调试级消息编译进内核，否则在运行时将不可用。随后这些消息可根据不同范围层级启用/禁用——每个源文件、函数... |
| DYNAMIC_DEBUG_CORE | bool | 启用动态调试的核心功能支持。当你希望将动态调试通过为每个内核模块定义的 DYNAMIC_DEBUG_MODULE 绑定到它们时很有用，尤其是... |
| ELFCORE | bool | 该选项启用 kernel/elfcore.o。 |
| ELF_CORE | bool | 启用对生成核心转储的支持。禁用可节省约 4k。 |
| ETHTOOL_NETLINK | bool | 一个基于 generic netlink 的、用于 ethtool 的替代用户空间接口。它提供更好的可扩展性以及一些新特性，例如通知消息。 |
| EVENTFD | bool | 启用 eventfd() 系统调用，允许接收内核通知（即 KAIO）或用户空间通知。如果不确定，选择 Y。 |
| EXEC_KUNIT_TEST | bool | 该选项构建 exec 的 KUnit 测试，测试 exec 内部各方面边界条件。 |
| EXT_GROUP_SCHED | bool | 该特性使调度器能够基于当前在该 CPU 上调度的 RUNNABLE 任务跟踪每个 CPU 的钳制利用率。启用该选项后，用户可以指定一个最小值与... |
| FAILOVER | tristate | failover 模块为半虚拟化驱动提供一个通用接口，以向 failover 实例注册一个 netdev 与一组操作。这些操作用作事件处理程序，在...时被调用。 |
| FAILSLAB | bool | 为 kmalloc 提供故障注入能力。 |
| FAIL_FUNCTION | bool | 提供基于函数的故障注入能力。这将允许你用给定返回值的返回语句覆盖特定函数。结果，函数调用者将看到一个错误值... |
| FAIL_FUTEX | bool | 为 futex 提供故障注入能力。 |
| FAIL_IO_TIMEOUT | bool | 在端 IO 处理上提供故障注入能力。这将使块层按配置"遗忘"一个中断，从而演练错误处理。仅对使用通用...的驱动有效。 |
| FAIL_MAKE_REQUEST | bool | 为磁盘 IO 提供故障注入能力。 |
| FAIL_MMC_REQUEST | bool | 为 MMC IO 提供故障注入能力。这将使 mmc 核心返回数据错误。这对于测试 mmc 块设备中的错误处理，以及测试 mmc 主机驱动...很有用。 |
| FAIL_PAGE_ALLOC | bool | 为 alloc_pages() 提供故障注入能力。 |
| FAIL_SKB_REALLOC | bool | 提供故障注入能力，强制 skb 被重新分配，以捕获指向 skb 的可能无效指针。更多信息请参阅 Documentation/fault-injection/fault-injection.rst |
| FAIL_SUNRPC | bool | 为 SunRPC 及其使用者提供故障注入能力。 |
| FAULT_INJECTION | bool | 提供故障注入框架。更多细节请参阅 Documentation/fault-injection/。 |
| FAULT_INJECTION_CONFIGFS | bool | 该选项允许基于 configfs 的驱动通过 configfs 动态配置故障注入。驱动特定的每个故障注入参数都可以作为 configfs 属性在...中可见。 |
| FAULT_INJECTION_DEBUG_FS | bool | 通过 debugfs 启用故障注入能力的配置。 |
| FAULT_INJECTION_STACKTRACE_FILTER | bool | 为故障注入能力提供栈跟踪过滤器。 |
| FAULT_INJECTION_USERCOPY | bool | 为 usercopy 函数（copy_from_user()、get_user()...）提供注入失败的故障注入能力。 |
| FFS_KUNIT_TEST | tristate | 该选项构建 ffs 系列位操作函数（包括 ffs()、__ffs()、fls()、__fls()、fls64() 与 __ffs64()）的 KUnit 测试。这些测试验证数学正确性、边界情况处理... |
| FHANDLE | bool | 如果你在此选择 Y，用户级程序将能够将文件名映射为句柄，随后将该句柄用于不同的文件系统操作。这在实现用户空间文件服务...时很有用。 |
| FIB_RULES | bool | 该特性提供一个支持轻量级隧道（如 mpls）的基础设施。轻量级隧道端点没有关联的网络设备。隧道封装参数存储于...。 |
| FILE_LOCKING | bool | 该选项启用标准文件锁支持，这是 NFS 等文件系统以及 flock() 系统调用所必需的。禁用该选项可节省约 11k。 |
| FIND_BIT_BENCHMARK | tristate | 该选项构建 "test_find_bit" 模块，用于测量 find_*_bit() 函数的性能。如果不确定，选择 N。 |
| FIND_BIT_BENCHMARK_RUST | tristate | 该选项构建 "find_bit_benchmark_rust" 模块。它是一个微基准测试，测量与 C 中 find_*_bit() 操作对应的 Rust 函数性能。它遵循 FIND_BI... |
| FIND_NORMAL_PAGE | def_bool | 该体系结构使用惰性 MMU 模式。这允许将与 MMU 相关的体系结构状态变更推迟到退出该模式时才进行。详情请参阅 <linux/pgtable.h>。 |
| FLATMEM_MANUAL | bool | 该选项最适合具有扁平地址空间的非 NUMA 系统。FLATMEM 在性能与资源消耗方面是最高效的系统，也是小...的最佳选项。 |
| FORCE_NR_CPUS | def_bool | This option provides a glob_match function for performing simple text pattern matching.  It originated in the ATA code to blacklist particular drive models, but other 设备驱动程序 may need simila... |
| FORTIFY_KUNIT_TEST | tristate | 构建用于检查 FORTIFY_SOURCE 内部机制的单元测试，FORTIFY_SOURCE 由 str*() 与 mem*() 系列函数使用。有关 FORTIFY_SOURCE 运行时陷阱的测试，请参阅 LKDTM 的 "FORTIFY_*" 测试。 |
| FPROBE_SANITY_TEST | bool | 该选项将在系统启动时启用对 fprobe 的测试。会进行一系列测试以验证 fprobe 是否正常工作。如果不确定，选择 N。 |
| FRAME_WARN | int | 告诉编译器在构建时对大于此值的栈帧发出警告。设置过低会导致大量警告。设置为 0 则禁用该警告。 |
| FREEZER | def_bool |  |
| FS_DAX_PMD | bool | 该选项启用文件系统的导出操作，以支持外部块 IO。 |
| FS_IOMAP | bool | 直接访问（DAX）可用于内存后备的块设备。如果块设备支持 DAX 且文件系统支持 DAX，你就可以避免用页缓存来缓冲 I/O。开启... |
| FUNCTION_ERROR_INJECTION | bool | 向内核中用 ALLOW_ERROR_INJECTION() 注解的各种函数注入故障。BPF 也可能修改这些函数的返回值。这有助于测试错误路径... |
| FUTEX | bool | 禁用该选项将导致构建出的内核不支持"快速用户空间互斥体"。所得内核可能无法正确运行基于 glibc 的应用程序。 |
| FUTEX_PI | bool | 禁用该选项将导致构建出的内核不支持 epoll 系列系统调用。 |
| GCD_KUNIT_TEST | tristate | 该选项启用针对 gcd() 函数的 KUnit 测试套件，gcd() 计算两个数的最大公约数。该测试套件在各种场景下验证 gcd() 的正确性... |
| GCOV_PROFILE_URING | bool | 在 io_uring 子系统上启用 GCOV 性能分析，以方便代码覆盖率测试。如果不确定，选择 N。注意这会对 io_uring 子系统的性能产生负面影响，因此... |
| GDB_SCRIPTS | bool | 这会在构建目录中创建指向 GDB 辅助脚本所需的链接。如果你将 vmlinux 加载到 gdb 中，辅助脚本也会被 gdb 自动导入，并提供额外的功能... |
| GENERIC_EARLY_IOREMAP | bool | 这是 32 位用户进程在栈向上增长时（目前仅在 parisc 架构上）的 VM 布局中栈的最大大小（以 MB 为单位），当 RLIMIT_STACK 硬限制为无限制时。一个... |
| GENERIC_IOREMAP | bool |  |
| GLOB_KUNIT_TEST | tristate | 启用该选项以在运行时测试 glob 函数。该测试套件在各种场景（包括边界情况）下验证 glob_match() 的正确性。如果不确定，选择 N |
| GRACE_PERIOD | tristate | 一些 NFS 服务器支持一个辅助的 NFS LOCALIO 协议，它不是 NFS 协议的正式部分。该选项在内核的 NFS 服务器与客户端中启用对 LOCALIO 协议的支持... |
| GROUP_SCHED_WEIGHT | def_bool | 该选项允许用户在公平组调度器内为任务定义 CPU 带宽速率（限制）。未设置限制的组被视为不受约束，将以无...的方式运行。 |
| GUEST_PERF_EVENTS | bool | 详情请参阅 tools/perf/design.txt |
| GUP_GET_PXX_LOW_HIGH | bool | 提供一个测试模块，它会分配并释放许多不同大小的块，并报告耗时。旨在提供一种一致的方法来衡量对 dma_pool_all...的修改效果。 |
| GUP_TEST | bool | 提供 /sys/kernel/debug/gup_test，进而提供一种发起 ioctl 调用的方式，用于启动针对 get_user_pages*() 与 pin_user_pages*() 系列 API 的内核单元测试。这... |
| HARDLOCKUP_DETECTOR_COUNTS_HRTIMER | bool | 在此选择 Y 以让内核在"硬锁死"时恐慌，硬锁死是指导致内核在中断禁用状态下于内核模式循环超过 10 秒（可通过 watchdog...配置）的 Bug。 |
| HARDLOCKUP_DETECTOR_PERF | bool | 将使用特定于体系结构的硬锁死检测器实现。# # "perf" 与 "buddy" 硬锁死检测器都对 hrtimer 中断计数。该配置启用管理这些...的函数。 |
| HARDLOCKUP_DETECTOR_PREFER_BUDDY | bool | 在此选择 Y 以优先使用 buddy 硬锁死检测器而非 perf 检测器。使用 buddy 检测器时，每个 CPU 利用其 softlockup hrtimer 通过检查下一个 CPU 是否正在处理 hrtimer 中断来... |
| HASHTABLE_KUNIT_TEST | tristate | 该选项构建 hashtable 的 KUnit 测试套件。它测试 include/linux/hashtable.h 中定义的 API 的基本功能。有关 KUnit 及单元测试的更多信息，请参阅... |
| HASH_KUNIT_TEST | tristate | 启用该选项以在启动时测试内核的字符串（<linux/stringhash.h>）与整数（<linux/hash.h>）哈希函数。KUnit 测试在启动时运行，并以 TA...格式将结果输出到调试日志。 |
| HAS_SECURITY_AUDIT | def_bool | 这将构建 securityfs 文件系统。它目前被各种安全模块（AppArmor、IMA、SafeSetID、TOMOYO、TPM）使用。如果你不确定如何回答，选择 N。 |
| HAVE_ARCH_AUDITSYSCALL | bool | 这是基本的基于 tick 的 cputime 记账，按每次 jiffy 的粒度维护关于用户、系统与空闲时间消耗的统计。如果不确定，选择 Y。 |
| HAVE_ARCH_TLB_REMOVE_TABLE | def_bool | 尝试在 munmap 与 exit_mmap 路径之外的路径中回收空的用户页表页。注意：目前只会回收空的用户 PTE 页表页。 |
| HAVE_ARCH_USERFAULTFD_MINOR | bool | 体系结构具有 userfaultfd 次要缺页支持。 |
| HAVE_ARCH_USERFAULTFD_WP | bool | 体系结构具有 userfaultfd 写保护支持。 |
| HAVE_DEBUG_BUGVERBOSE | bool | 启用该选项以开启链表遍历例程中的扩展检查。该选项以性能换取更高质量的错误报告，更适合内核调试。如果你在意... |
| HAVE_DEBUG_STACKOVERFLOW | bool | 如果你想检查内核、IRQ 与异常栈（若你的体系结构使用它们）的溢出，在此选择 Y。如果空闲栈空间低于某个...，该选项将显示详细消息。 |
| HAVE_HARDLOCKUP_DETECTOR_BUDDY | bool | 在此选择 Y 以让内核充当看门狗来检测硬锁死。硬锁死是指导致 CPU 在内核模式循环超过 10 秒、且不让其他中断...的 Bug。 |
| HAVE_KERNEL_GZIP | bool | Linux 内核是一种自解压可执行文件。有多种压缩算法可用，它们在效率、压缩与解压缩速度上有所不同。压缩速度仅在...时有意义。 |
| HAVE_LD_DEAD_CODE_DATA_ELIMINATION | bool | 这要求体系结构对其外部入口点进行注解或以其他方式保护，使其不被丢弃。链接器脚本还必须正确地将 .text.*、.data.* 与 .bss.* 合并到输出节...。 |
| HAVE_PCSPKR_PLATFORM | bool | 该选项允许禁用或调整某些基础内核选项与设置。这用于能够容忍"非标准"内核的专用环境。只有当你确实...时才使用。 |
| HAVE_PERF_EVENTS | bool | 详情请参阅 tools/perf/design.txt。 |
| HAVE_SCHED_AVG_IRQ | def_bool | 选择该选项以在调度器中启用硬件压力记账。硬件压力是传达给调度器的值，反映由硬件节流导致的 CPU 计算能力降低... |
| HAVE_UNSTABLE_SCHED_CLOCK | bool | 该特性使调度器能够基于当前在该 CPU 上调度的 RUNNABLE 任务跟踪每个 CPU 的钳制利用率。通过该选项，用户可以指定 CPU 利用率的最小值与最大值... |
| HEADERS_INSTALL | bool | 该选项将安装 uapi 头文件（导出到用户空间的头文件）到 usr/include 目录，供内核构建期间使用。构建内核本身不需要它，但...需要它。 |
| HMM_MIRROR | bool | 允许创建 struct page 来表示不可寻址的设备内存；即只能从设备（或设备组）访问的内存。你可能还希望选择 HMM_MIRROR。 |
| HUGETLB_PAGE | def_bool | 在此选择 Y 以查看各种杂项文件系统的选项，例如来自其他操作系统的文件系统。该选项本身不添加任何内核代码。如果你选择 N，所有... |
| HUGETLB_PAGE_OPTIMIZE_VMEMMAP_DEFAULT_ON | bool | HugeTLB Vmemmap 优化（HVO）默认关闭。在此选择 Y 以默认启用 HVO。它可通过 hugetlb_free_vmemmap=off（启动命令行）或 hugetlb_optimize_vmemmap（sysctl）禁用。 |
| HWPOISON_INJECT | tristate | NOMMU 的 mmap() 经常需要分配大块连续内存来存储映射，但它只能向系统分配器请求 2^N*PAGE_SIZE 大小的块——这... |
| HW_BREAKPOINT_KUNIT_TEST | bool | hw_breakpoint 约束记账测试。如果不确定，选择 N。 |
| HYPERV_TESTING | bool | 选择该选项以启用 Hyper-V vmbus 测试。 |
| IDLE_PAGE_TRACKING | bool | 该特性允许估计在给定时间段内未被访问的用户页数量。该信息可用于调优内存 cgroup 限制和/或为作业放置... |
| IKCONFIG | tristate | 该选项启用将完整的 Linux 内核 ".config" 文件内容保存到内核中。它记录了运行内核或磁盘内核中使用了哪些内核选项... |
| IKCONFIG_PROC | bool | 该选项启用通过 /proc/config.gz 访问内核配置文件。 |
| IKHEADERS | tristate | 该选项启用访问构建过程中生成的内核头文件。这些可用于构建 eBPF 跟踪程序或类似程序。如果你将头文件构建为... |
| INDIRECT_IOMEM | bool | 该选项由其他选项/体系结构选中，以提供模拟的 iomem 访问器。 |
| INDIRECT_IOMEM_FALLBACK | bool | 如果选中 INDIRECT_IOMEM，该选项在 IO 内存地址不是已注册的模拟区域时，启用回退到普通 mmio 访问。 |
| INET | bool | 这些是用于互联网与大多数本地以太网上的协议。强烈建议在此选择 Y（这会使内核增大约 400 KB），因为某些程序（例如 X 窗口...）需要它。 |
| INITRAMFS_PRESERVE_MTIME | bool | initramfs cpio 归档中的每个条目都带有一个 mtime 值。启用后，提取的 cpio 项采用该 mtime，目录 mtime 的设置推迟到其任何子项创建之后。... |
| INITRAMFS_TEST | bool | 为 initramfs 构建 KUnit 测试。请参阅 Documentation/dev-tools/kunit |
| INTEL_TXT | bool | 该选项启用在内核中使用可信启动（tboot）模块启动的支持。这将利用 Intel(R) 可信执行技术对内核进行度量启动。如果... |
| INTERVAL_TREE_SPAN_ITER | bool | 支持在 XArray 中占据多个连续索引的条目。 |
| INTERVAL_TREE_TEST | tristate | 一个测量区间树库性能的基准测试。 |
| INT_LOG_KUNIT_TEST | tristate | 该选项启用针对 int_log 库的 KUnit 测试套件，该库提供两个分别称为 intlog2 与 intlog10 的函数，用于计算以 2 为底和以 10 为底的整数对数。该... |
| INT_POW_KUNIT_TEST | tristate | 该选项启用针对 int_pow 函数（执行整数幂运算）的 KUnit 测试套件。该测试套件旨在验证 int_pow 的实现能正确计算... |
| INT_SQRT_KUNIT_TEST | tristate | 该选项启用针对 int_sqrt() 函数（执行平方根计算）的 KUnit 测试套件。该测试套件检查各种场景（包括边界情况）以确保正确性。如果... |
| IO_STRICT_DEVMEM | bool | 如果禁用该选项，你将允许用户空间（root）访问所有 io 内存，无论驱动是否正在使用该范围。意外访问这显然是灾难性的，但... |
| IO_URING | bool | 该选项启用对 io_uring 接口的支持，使应用程序能够通过内核与应用程序之间共享的提交与完成环来提交并完成 IO。 |
| IO_URING_MOCK_FILE | tristate | 为 io_uring 子系统测试启用模拟文件。ABI 仍可能变化，因此它仍是实验性的，只应为特定测试目的启用。如果不确定，选择 N。 |
| IO_URING_ZCRX | def_bool |  |
| IRQ_TIME_ACCOUNTING | bool | 选择该选项以启用细粒度的任务 IRQ 时间记账。这通过在软中断与硬中断状态之间的每次转换读取时间戳来实现，因此可能会有小的性能... |
| IS_SIGNED_TYPE_KUNIT_TEST | tristate | 构建针对 is_signed_type() 宏的单元测试。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-tools/kunit/ 中的 KUnit 文档。如果不确定，选择 N。 |
| KALLSYMS | bool | 在此选择 Y 以让内核打印符号化的崩溃信息与符号化栈回溯。这会使内核体积略有增大，因为所有符号都必须加载到内核镜像中。 |
| KALLSYMS_ALL | bool | 通常 kallsyms 只包含函数的符号，以便得到更好的 OOPS 消息与回溯（即来自 text 与 inittext 节的符号）。这对大多数情况已足够。只有在你... |
| KALLSYMS_SELFTEST | bool | 测试一些接口（例如 kallsyms_lookup_name）的基本功能与性能。它还计算当前符号集的 kallsyms 压缩算法的压缩率。... |
| KCMP | bool | 启用内核资源比较系统调用。它为用户空间提供比较两个进程是否共享公共资源（例如文件描述符甚至虚拟...）的能力。 |
| KCOV | bool | KCOV 以适合覆盖率引导模糊测试（随机化测试）的形式导出内核代码覆盖率信息。更多细节请参阅 Documentation/dev-tools/kcov.rst。 |
| KCOV_ENABLE_COMPARISONS | bool | KCOV 还导出被插桩代码中每次比较的操作数，以及操作数大小与比较指令的 PC。这些操作数可被模糊测试引擎用来改进... |
| KCOV_INSTRUMENT_ALL | bool | 如果你在进行通用的系统调用模糊测试（例如 syzkaller），你会希望插桩整个内核，并应在此选择 y。如果你在进行更有针对性的模糊测试（例如...）， |
| KCOV_IRQ_AREA_SIZE | hex | KCOV 使用预分配的每 CPU 区域来从软中断收集覆盖率。这指定了这些区域的大小，以 unsigned long 字数计。 |
| KCOV_SELFTEST | bool | 在启动时运行简短的 KCOV 覆盖率收集自测。测试失败时导致内核恐慌。建议启用，以确保关键功能按预期工作。 |
| KERNEL_BZIP2 | bool | 它的压缩率与速度居中。解压缩速度在可选方案中是最慢的。与 gzip 相比，使用 bzip2 的内核体积小约 10%。bzip2 占用大量... |
| KERNEL_GZIP | bool | 久经考验的旧式 gzip 压缩。它在压缩率与解压缩速度之间提供了良好的平衡。 |
| KERNEL_LZ4 | bool | LZ4 is an LZ77-type compressor with a fixed, byte-oriented encoding. A preliminary version of LZ4 de/compression tool is available at <https://code.google.com/p/lz4/>. Its compression ratio is wors... |
| KERNEL_LZMA | bool | 该压缩算法的压缩率最佳。解压缩速度介于 gzip 与 bzip2 之间。压缩最慢。与 gzip 相比，使用 LZMA 的内核体积小约 33%。 |
| KERNEL_LZO | bool | 它的压缩率在可选方案中最差。内核体积比 gzip 大约 10%；但其速度（压缩与解压缩）最快。 |
| KERNEL_UNCOMPRESSED | bool | 生成未压缩的内核镜像。该选项通常不是你想要的。它在缓慢的仿真环境中调试内核时有用，在那里解压和移动内核非常... |
| KERNEL_XZ | bool | XZ 使用 LZMA2 算法以及特定于指令集的 BCJ 过滤器，可改善可执行代码的压缩率。与 gzip 相比，使用 XZ 的内核体积小约 30%... |
| KERNEL_ZSTD | bool | ZSTD 是一种面向中等压缩率与快速解压缩速度的压缩算法。它的压缩比 GZIP 更好，解压速度与 LZO 大致相当，但比 LZ4 慢... |
| KFIFO_KUNIT_TEST | tristate | 该选项构建通用 FIFO 实现的 KUnit 测试套件。它测试 kfifo 类型及相关宏的 API 与基本功能。有关 KUnit 及单元测试的更多信息请参阅... |
| KPROBES_SANITY_TEST | tristate | 该选项提供在启动时测试基本 kprobes 功能的能力。插入 kprobe 与 kretprobe 样本并验证其功能。如果不确定，选择 N。 |
| LATENCYTOP | bool | 如果你想使用 LatencyTOP 工具找出哪个用户空间进程阻塞在哪个内核操作上，启用该选项。 |
| LAZY_MMU_MODE_KUNIT_TEST | tristate | 启用该选项以检查惰性 MMU 模式接口是否如预期般工作。只包含对通用接口的测试（不包括体系结构特定的行为）。如果不确定，选择 N。 |
| LD_DEAD_CODE_DATA_ELIMINATION | bool | 启用对 -ffunction-sections -fdata-sections 编译并使用 --gc-sections 链接，以通过链接器进行死代码与死数据消除。这可以减少磁盘上和内存中... |
| LD_ORPHAN_WARN | def_bool | 启用对 /proc/sys/debug/exception-trace 的支持。 |
| LIBFDT | bool | 启用快速查找对象标识符注册表。 |
| LINEAR_RANGES | tristate | 该选项提供 packing() 辅助函数，它允许在 CPU 可用表示与可能具有任意以下特性的内存表示之间转换位域：... |
| LINEAR_RANGES_TEST | tristate | 该选项构建 linear_ranges 单元测试，在启动时运行。测试 linear_ranges 逻辑的正确性。有关 KUnit 及单元测试的更多信息，请参阅 KUnit 文档... |
| LIST_KUNIT_TEST | tristate | 该选项构建链表 KUnit 测试套件。它测试 list_head 类型及相关宏的 API 与基本功能。KUnit 测试在启动时运行，并将结果输出到调试... |
| LIST_PRIVATE_KUNIT_TEST | tristate | 该选项构建针对 include/linux/list_private.h 中定义的私有链表原语的 KUnit 测试。这些原语允许操作被标记为私有且...的 list_head 成员。 |
| LIVEUPDATE_TEST | bool | 为 Live Update Orchestrator 启用一个内建内核测试模块。该模块通过向任意真实文件处理程序注册一组模拟 FLB 对象来验证 File-Lifecycle-Bound 子系统... |
| LKDTM | tristate | 该模块通过在预定义的崩溃点诱发系统故障来测试不同的转储机制。如果不需要：选择 N。在此选择 M 将本代码编译为模块。该... |
| LOCALVERSION | string | 在内核版本末尾追加一个额外的字符串。例如，这会在你输入 uname 时显示。你在此设置的字符串将被追加到任何以该文件名...的文件内容之后。 |
| LOCALVERSION_AUTO | bool | 这将尝试通过查找属于当前树顶修订的 git 标签，自动确定当前树是否为发布树。格式为 -gxxxxxxxx 的字符串将被添加... |
| LOCKDEP | bool | 如果你遇到 "BUG: MAX_LOCKDEP_ENTRIES too low!" 消息，尝试增大该值。 |
| LOCKDEP_CHAINS_BITS | int | 如果你遇到 "BUG: MAX_LOCKDEP_CHAINS too low!" 消息，尝试增大该值。 |
| LOCKDEP_CIRCULAR_QUEUE_BITS | int | 如果你因 __cq_enqueue() 失败而遇到 "lockdep bfs error:-1" 警告，尝试增大该值。 |
| LOCKDEP_STACK_TRACE_BITS | int | 如果你遇到 "BUG: MAX_STACK_TRACE_ENTRIES too low!" 消息，尝试增大该值。KASAN 会显著增加栈跟踪消耗，因为其 slab 跟踪与 lockdep 的依赖... |
| LOCKDEP_STACK_TRACE_HASH_BITS | int | 如果你需要较大的 STACK_TRACE_HASH_SIZE，尝试增大该值。 |
| LOCKUP_DETECTOR | bool | 在此选择 Y 以让内核充当看门狗检测软锁死。软锁死是指导致内核在内核模式循环超过 20 秒、且不给其他任务...的 Bug。 |
| LOCK_DEBUGGING_SUPPORT | bool | 该特性使内核能够证明内核运行时发生的所有加锁在数学上是正确的：即在任何情况下，任意（且尚未触发的）组合...都不可能导致死锁。 |
| LOCK_MM_AND_FIND_VMA | bool | 启用 NUMA 仿真。当以 "numa=fake=N" 启动时（N 为节点数），扁平机器将被拆分为虚拟节点。这仅用于调试。 |
| LOCK_STAT | bool | 该特性启用对锁竞争点的跟踪。更多细节请参阅 Documentation/locking/lockstat.rst。这也启用了 "perf lock"（perf 的子命令）所需的锁事件。如果你希望... |
| LOCK_TORTURE_TEST | tristate | 该选项提供一个内核模块，对内核锁原语运行 torture 测试。如果需要，该内核模块可以在被测的正在运行的内核上事后构建。在此选择 Y... |
| LOG_BUF_SHIFT | int | 选择最小内核日志缓冲区大小（以 2 的幂计）。最终大小受 LOG_CPU_MAX_BUF_SHIFT 配置参数影响，见下。任何更大的大小也可能被 "log_buf_len" 启动...强制。 |
| LOG_CPU_MAX_BUF_SHIFT | int | 该选项允许根据 CPU 数量增大默认环形缓冲区大小。该值定义每个 CPU 的贡献（以 2 的幂计）。使用的空间通常只有几行... |
| LONGEST_SYM_KUNIT_TEST | tristate | 测试可能的最长符号。如果不确定，选择 N。 |
| LRU_GEN | bool | 一个用于内存过量提交的高性能 LRU 实现。详情请参阅 Documentation/admin-guide/mm/multigen_lru.rst。 |
| LRU_GEN_ENABLED | bool | 该选项默认启用多代 LRU。 |
| LRU_GEN_STATS | bool | 除非你计划为调试目的查看被逐出代际的历史统计，否则不要启用该选项。该选项有每个 memcg 与每个节点的内存开销。 |
| LRU_GEN_WALKS_MMU | def_bool | 允许在缺页处理期间进行每 vma 加锁。该特性在处理缺页时分别锁定每个虚拟内存区域，而不是获取 mmap_lock。 |
| LSM_MMAP_MIN_ADDR | int | 这是应当免受用户空间分配影响的低虚拟内存部分。阻止用户写入低地址页有助于减少内核空指针 Bug 的影响。对于... |
| LWTUNNEL_BPF | bool | 允许在路由查找之后，将 BPF 程序作为下一跳动作运行于传入与传出的数据包。 |
| LZO_COMPRESS | tristate | 驱动可以选择该选项以强制为参数 'm'（伽罗瓦域阶数）与 't'（纠错能力）指定特定常量值。那些特定值必须通过声明默认值...来设置。 |
| MAGIC_SYSRQ | bool | 如果你在此选择 Y，即使系统崩溃（例如在内核调试期间），你也能对系统有一定控制（例如，你将能够将缓冲区缓存刷新到磁盘、重启系统...）。 |
| MAGIC_SYSRQ_DEFAULT_ENABLE | hex | 指定默认启用哪些 SysRq 键功能。可设置为 1 或 0 以全部启用或禁用，或设置为 Documentation/admin-guide/sysrq.rst 中描述的位掩码。 |
| MAGIC_SYSRQ_SERIAL | bool | 许多嵌入式板卡有断开的 TTL 电平串口，可能产生一些垃圾数据，导致虚假的 sysrq 误检测。该选项允许你决定是否要启用... |
| MAGIC_SYSRQ_SERIAL_SEQUENCE | string | 指定可跟在 BREAK 之后以在串口控制台上启用 SysRq 的字符序列。如果不确定，留空字符串，该选项将不被启用。 |
| MAX_SKB_FRAGS | int | 每个 skb_shared_info 拥有更多分片有助于提高 GRO 效率。这有助于 BIG TCP 工作负载，但可能暴露某些遗留驱动中的 Bug。这也会增加小包的内存开销，... |
| MEMBARRIER | bool | 启用 membarrier() 系统调用，它允许跨所有运行线程发出内存屏障，可用于通过将用户空间内存屏障的成本非对称地转移来...。 |
| MEMCG | bool | 提供对 cgroup 中任务内存占用的控制。 |
| MEMCG_NMI_UNSAFE | bool | 已被 cgroup v2 实现弃用的传统 cgroup v1 内存控制器。v1 保留用于尚未迁移到新 cgroup v2 接口的传统应用。如果你... |
| MEMCPY_KUNIT_TEST | tristate | 构建针对 memcpy()、memmove() 与 memset() 函数的单元测试。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-tools/kunit/ 中的 KUnit 文档... |
| MEMORY_HOTREMOVE | bool | 允许迁移在内存气球中膨胀的页，使它们能从仅可用于可移动分配（例如 ZONE_MOVABLE、CMA）的内存区域分配，并且... |
| MEMORY_NOTIFIER_ERROR_INJECT | tristate | 该选项提供向内存热插拔通知链回调注入人为错误的能力。它通过 /sys/kernel/debug/notifier-error-inject/me...下的 debugfs 接口控制。 |
| MEMTEST | bool | 该选项添加内核参数 'memtest'，允许设置并执行 memtest。memtest=0 表示禁用；-- 默认 memtest=1 表示执行 1 种测试模式；...memtest=17 表示执行 17 种测试模式... |
| MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT | bool | 为内存分配性能分析添加带有帮助性错误消息的警告。 |
| MEM_SOFT_DIRTY | bool | 该选项通过在 pte 上引入一个软脏位来启用内存变更跟踪。当有人写入一个页时，该位被设置，如同普通的脏位，但不同于后者，它可以被清... |
| MESSAGE_LOGLEVEL_DEFAULT | int | 没有指定优先级的 printk 语句的默认日志级别。至少从 2.6.10 起它被硬编码为 KERN_WARNING，但紧密审计其日志的人员可能希望将其设置为... |
| MHP_DEFAULT_ONLINE_TYPE_OFFLINE | bool | 热插拔内存默认不会被上线。为具有处理热插拔内存上线策略的驱动与用户策略的系统选择此项。 |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_AUTO | bool | 如果你希望内核自动将热插拔内存上线到它认为合理的区域，选择此项。该内存可能被用于内核数据。 |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_KERNEL | bool | 如果你希望内核自动将热插拔内存上线到可用于内核数据的区域，选择此项。这通常意味着 ZONE_NORMAL。 |
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_MOVABLE | bool | 如果你希望内核自动将热插拔内存上线到 ZONE_MOVABLE，选择此项。该内存通常不会被用于内核数据。这应仅在管理员知道...时使用。 |
| MIGRATION | bool | 当平台上存在多种 HugeTLB 页大小时，允许 pageblock_order 值为动态值，而非仅标准 HUGETLB_PAGE_ORDER。注意 pageblock_order 不能... |
| MIN_HEAP_KUNIT_TEST | tristate | 该选项启用针对最小堆（min heap）库的 KUnit 测试套件，该库提供创建与管理最小堆的函数。该测试套件检查最小堆库的功能。如果不确定... |
| MMAP_ALLOW_UNINITIALIZED | bool | 通常，按照 Linux 规范，从 mmap() 获得的匿名内存在传递给用户空间之前其内容会被清除。启用该配置选项允许你请求... |
| MM_ID | def_bool | 透明大页允许内核在可能时透明地对应用程序使用大页与大页 TLB。该特性可通过...对某些应用程序提升计算性能。 |
| MODULE_ALLOW_BTF_MISMATCH | bool | 对于拆分 BTF 与 vmlinux 不匹配的模块，不拒绝加载而是不带 BTF 加载。启用模块 BTF 时的默认行为是拒绝此类不匹配的模块；该选项... |
| MPILIB | tristate | 来自 GnuPG 的多精度数学库。它用于实现 RSA 数字签名验证，该验证被 IMA/EVM 数字签名扩展使用。 |
| MSEAL_SYSTEM_MAPPINGS | bool | 对系统映射应用 mseal。系统映射包括 vdso、vvar、vvar_vclock、vectors（arm 兼容模式）、sigpage（arm 兼容模式）、uprobes。内存密封需要 64 位内核... |
| MULTIUSER | bool | 该选项启用对非 root 用户、组与能力的支持。如果你在此选择 N，所有进程将以 UID 0、GID 0 以及所有可能的 capability 运行。在此选择 N 还会编译掉... |
| NET | bool | 除非你确实知道自己在做什么，否则应在此选择 Y。原因是某些程序即使在不连接网络的独立机器上运行也需要内核网络支持... |
| NETDEV_ADDR_LIST_TEST | tristate | 覆盖核心网络基础设施（例如 sk_buff）的 KUnit 测试。如果不确定，选择 N。 |
| NETDEV_NOTIFIER_ERROR_INJECT | tristate | 该选项提供向 netdevice 通知链回调注入人为错误的能力。它通过 /sys/kernel/debug/notifier-error-inject/netdev 下的 debugfs 接口控制。如果... |
| NETFILTER | bool | Netfilter 是一个用于过滤与篡改经过你的 Linux 主机的网络数据包的框架。包过滤最常见的用途是将你的 Linux 主机作为保护本地...的防火墙。 |
| NETFILTER_ADVANCED | bool | 如果你在此选择 Y，你可以在所有 netfilter 模块之间选择。如果选择 N，则不常见的模块将不显示，而大多数人需要的基本模块将默认为 'M'。如果不确定，选择 Y。 |
| NETWORK_FILESYSTEMS | bool | 在此选择 Y 以查看网络文件系统与文件系统相关网络代码（例如 NFS 守护进程与 RPCSEC 安全模块）的选项。该选项本身不添加任何内核代码。如果... |
| NETWORK_SECMARK | bool | 该选项启用网络数据包的安全标记，类似于 nfmark，但专用于安全目的。如果你不确定如何回答，选择 N。 |
| NET_DEVLINK | bool | 启用页池统计以跟踪页池中的页分配与回收。该选项在分配与回收路径上产生额外的 CPU 开销，以及存储统计信息的额外内存开销... |
| NET_DROP_MONITOR | tristate | 该特性在网络栈中丢弃数据包时向用户空间提供告警服务。告警通过 netlink 套接字广播给任何监听的用户空间进程。该... |
| NET_FLOW_LIMIT | bool | 当接收处理 CPU 的 backlog 达到 netdev_max_backlog 时，网络栈必须丢弃数据包。如果许多活动流中只有少数几个产生了绝大多数负载，就提前丢弃它们的流量... |
| NET_INGRESS | bool | This builds the KUnit tests for the handshake upcall mechanism. KUnit tests run during boot and output the results to the debug log in TAP format (https://testanything.org/). Only useful for kernel... |
| NET_NS | bool | 允许用户空间创建看似网络栈多个实例的东西。 |
| NET_PKTGEN | tristate | 该模块将以可配置的速率，从给定接口注入预配置的数据包。它用于网络接口压力测试与性能分析。如果你不理解... |
| NET_PTP_CLASSIFY | def_bool | 该选项允许具有硬件时间戳能力的 PHY（或其他 MII 总线嗅探设备）对网络数据包进行时间戳标记。该选项在发送与接收路径上增加一些开销。如果... |
| NET_RX_BUSY_POLL | bool | 启用该选项允许将 TCP 流解析器与 BPF_MAP_TYPE_SOCKMAP 一起使用。 |
| NFS_V4_2_SSC_HELPER | bool |  |
| NLATTR | bool | 用于使用轮询进行中断缓解轮询的辅助库。 |
| NOINSTR_VALIDATION | bool | 选择该选项将在链接 vmlinux 时向 ld 传递 "-Map=vmlinux.map"。该文件对于验证与调试神奇的节区处理，以及查看哪些代码段被消除...很有用。 |
| NOTIFIER_ERROR_INJECTION | tristate | 该选项提供向指定的通知链回调注入人为错误的能力。它可用于测试通知链失败时的错误处理。如果不确定，选择 N。 |
| NO_PAGE_MAPCOUNT | bool | 对于属于较大分配（例如透明大页）一部分的页，不维护每页 mapcount。启用该配置选项后，一些依赖此信息的接口将... |
| NUMA_BALANCING_DEFAULT_ENABLED | bool | 若设置，在 NUMA 机器上运行时将启用自动 NUMA 平衡。 |
| NUMA_MIGRATION | bool | 支持将页迁移到其他 NUMA 节点，通过 migrate_pages()、move_pages() 与 mbind() 等接口对用户空间可用。选择该选项也启用对页...的支持。 |
| OBJTOOL | bool | 在 objtool 警告时使构建失败。objtool 警告可能指示内核不稳定，包括启动失败。强烈建议启用该选项。如果不确定，选择 Y。 |
| OF_RECONFIG_NOTIFIER_ERROR_INJECT | tristate | 该选项提供向 OF 重配置通知链回调注入人为错误的能力。它通过 /sys/kernel/debug/notifier-error-inject/OF-re...下的 debugfs 接口控制。 |
| OVERFLOW_KUNIT_TEST | tristate | 构建针对 check_*_overflow()、size_*()、分配及相关函数的单元测试。有关 KUnit 及单元测试的更多信息，请参阅 Documentation 中的 KUnit 文档... |
| PACKING_KUNIT_TEST | tristate | 该选项构建 packing 库的 KUnit 测试。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-tools/kunit/ 中的 KUnit 文档。如有疑问，选择 N。 |
| PAGE_COUNTER | bool | 该选项默认启用 "favordynmods" 挂载选项，它降低了任务迁移与控制器开关等动态 cgroup 修改的延迟，代价是使热... |
| PAGE_IDLE_FLAG | bool | 这向 'struct page' 添加 PG_idle 与 PG_young 标志。PTE Accessed 位的写入者可以设置标志中该位的状态，使 PTE Accessed 位的读取者避免干扰。 |
| PAGE_MAPCOUNT | def_bool | 该选项启用连续内存分配器（CMA），它允许其他子系统分配大的物理连续内存块。CMA 保留一块内存区域，并只允许可移动页... |
| PAHOLE_HAS_BTF_TAG | def_bool | 决定 pahole 是否发出 btf_tag 属性（btf_type_tag 与 btf_decl_tag）。目前只有 clang 编译器实现了这些属性，因此使该配置依赖于 CC_IS_CLANG。 |
| PAHOLE_HAS_LANG_EXCLUDE | def_bool | 支持 --lang_exclude 标志，使 pahole 排除所提供语言的编译单元。在 Kbuild 中用于省略 pahole 1.24 版本不支持的 Rust 编译单元，此外... |
| PANIC_ON_OOPS | bool | 在此选择 Y 以让内核在 oops 时恐慌。其效果等同于在内核命令行设置 oops=panic。该特性有助于确保内核不做任何... |
| PANIC_TIMEOUT | int | 设置内核恐慌后直到发生重启的超时值（秒）。若 n = 0，则永远等待。超时值 n > 0 将在重启前等待 n 秒，而超时值 n... |
| PC104 | bool | Expose PC/104 form factor 设备驱动程序 and options available for selection and configuration. Enable this option if your target machine has a PC/104 bus. |
| PCPU_DEV_REFCNT | bool | 若设置该选项，网络设备引用计数使用每 CPU 变量。可强制设为 N 以检测下溢（代价是性能下降）。 |
| PCSPKR_PLATFORM | bool | 该选项允许禁用内部 PC 扬声器支持，节省一些内存。 |
| PERCPU_STATS | bool | 该特性通过 debugfs 收集并暴露统计信息。信息包括全局与每块统计，可用于帮助理解 percpu 内存使用。 |
| PERCPU_TEST | tristate | 启用该选项以构建验证 per-cpu 操作的测试模块。如果不确定，选择 N。 |
| PERF_EVENTS | bool | 启用内核对软件与硬件提供的各种性能事件的内核支持。软件事件要么内建支持，要么通过通用跟踪点支持。大多数现代 CPU 支持... |
| PHYS_ADDR_T_64BIT | def_bool | 启用内核同页合并（KSM）：KSM 定期扫描应用程序地址空间中那些应用建议可能可合并的区域。当它找到内容相同的页时，会将其替换... |
| PID_NS | bool | 支持进程 ID 命名空间。只要处于不同的 pid 命名空间中，这就允许存在多个具有相同 pid 的进程。这是容器的基本构件。 |
| PM_NOTIFIER_ERROR_INJECT | tristate | 该选项提供向 PM 通知链回调注入人为错误的能力。它通过 /sys/kernel/debug/notifier-error-inject/pm 下的 debugfs 接口控制。如果通知... |
| POSIX_MQUEUE_SYSCTL | bool | 这是内核通过将其拼接进管道来向用户空间传递事件的通用通知队列。它可以与用于密钥/密钥环变更通知与设备...的监视器配合使用。 |
| POSIX_TIMERS | bool | 这包含对 POSIX 定时器的内核原生支持。一些嵌入式系统用不到它们，因此可以配置掉以减小内核镜像体积。当该选项... |
| PREEMPT_NOTIFIERS | bool | 构建一个简单的 ASN.1 语法编译器，它产生可被 ASN.1 流解码器解释的字节码输出，并用于告知解码器在流中预期哪些标签以及... |
| PRIME_NUMBERS_KUNIT_TEST | tristate | 该选项启用针对 {is,next}_prime_number 函数的 KUnit 测试套件。启用该选项将包含将素数生成器函数与暴力实现进行比较的测试... |
| PRINTK | bool | 该选项启用正常的 printk 支持。移除它会消除内核镜像中大部分消息字符串，使内核或多或少沉默。由于这使得诊断...非常困难。 |
| PRINTK_CALLER | bool | 选择该选项会使 printk() 向每条消息添加调用者"线程 ID"（若处于任务上下文）或调用者"处理器 ID"（若不在任务上下文）。该选项用于...环境。 |
| PRINTK_EXECUTION_CTX | bool | 该选项扩展 struct printk_info 以在 printk 中包含额外的执行上下文，例如消息来源的进程名与 CPU 编号。这对于关联 printk 消息...很有用。 |
| PRINTK_INDEX | bool | 添加对编译时已知的全部 printk 格式在 <debugfs>/printk/index/<module> 处建立索引的支持。这可作为维护监视 /dev/kmsg 的守护进程的一部分，因为它允许审计... |
| PRINTK_RINGBUFFER_KUNIT_TEST | tristate | 该选项构建 printk 环形缓冲区 KUnit 测试套件。有关 KUnit 及单元测试的更多信息，请参阅 KUnit 文档。如果不确定，选择 N。 |
| PRINTK_TIME | bool | 选择该选项会使 printk() 消息的时间戳被添加到 syslog() 系统调用的输出与控制台上。时间戳总是在内部记录，并导出... |
| PROC_MEM_ALWAYS_FORCE | bool | 该选项允许 /proc/pid/mem 访问在拥有 ptrace 访问权限时覆盖内存映射权限。 |
| PROC_MEM_FORCE_PTRACE | bool | 该选项允许 /proc/pid/mem 访问为像 gdb 这样的活动 ptracer 覆盖内存映射权限。 |
| PROC_MEM_NO_FORCE | bool | 永远不要覆盖内存映射权限。 |
| PROC_PID_CPUSET | bool | 提供一个 cgroup 控制器，实现 cgroup 中进程可以 mknod 或打开的设备的白名单。 |
| PROFILING | bool | 在此选择 Y 以启用分析器使用的扩展性能分析支持机制。 |
| PROVE_RAW_LOCK_NESTING | bool | 启用 raw_spinlock 与 spinlock 嵌套检查，以确保不违反为 PREEMPT_RT 启用的内核的锁嵌套规则。 |
| PROVIDE_OHCI1394_DMA_INIT | bool | 如果你想调试在启动早期挂起或崩溃内核的问题，且崩溃的机器有 FireWire 端口，你可以使用此特性远程访问崩溃机器的内存... |
| PSI | bool | 收集指示系统中 CPU、内存与 IO 容量过量提交程度的指标。如果你在此选择 Y，内核将创建 /proc/pressure/，其中包含压力统计文件 cpu... |
| PSI_DEFAULT_DISABLED | bool | 若设置，压力停顿信息跟踪默认禁用，但可在启动时通过内核命令行传入 psi=1 启用。该特性向任务唤醒...添加一些代码。 |
| PTE_MARKER_UFFD_WP | bool | 允许为 userfaultfd 写保护目的创建标记 PTE。在基于文件的 shmem 与 hugetlbfs 等内存类型上启用 userfaultfd 写保护时需要它。 |
| RANDOM_KMALLOC_CACHES | bool | 一种加固特性，为正常的 kmalloc 分配创建 slab 缓存的多个副本，并使 kmalloc 基于代码地址随机选取其中之一，使攻击者更难... |
| RANDSTRUCT_KUNIT_TEST | tristate | 构建用于检查 CONFIG_RANDSTRUCT=y（随机化结构体布局）的单元测试。 |
| RATELIMIT_KUNIT_TEST | tristate | 该选项构建 "test_ratelimit" 模块，应用于速率限制的并发测试与正确性验证。如果不确定，选择 N。 |
| RATIONAL_KUNIT_TEST | tristate | 该选项构建有理数学单元测试。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-tools/kunit/ 中的 KUnit 文档。如果不确定，选择 N。 |
| RBTREE_TEST | tristate | 一个测量 rbtree 库性能的基准测试。还包括 rbtree 不变式检查。 |
| READABLE_ASM | bool | 禁用某些倾向于生成人类难以阅读的汇编输出的编译器优化。这可能使内核稍慢，但有助于需要经常盯着...的内核开发者。 |
| READ_ONLY_THP_FOR_FS | bool | 允许 khugepaged 将只读的基于文件的页放入 THP。它被标记为实验性，因为这是一个新特性。文件 THP 的写支持将在接下来的几个发布周期中开发。 |
| REED_SOLOMON_TEST | tristate | 该选项在启动时或模块加载时启用 rslib 的自测函数。如果不确定，选择 N。 |
| RELAY | bool | 该选项在某些文件系统（例如 debugfs）中启用对 relay 接口的支持。它旨在为工具与设施提供高效的机制来中转大量... |
| RESOURCE_KUNIT_TEST | tristate | 该选项构建资源 API 单元测试。测试 resource.c 与 ioport.h 提供的 API 逻辑。有关 KUnit 及单元测试的更多信息，请参阅 KUnit 文档... |
| RFS_ACCEL | bool | 允许具有流过滤表的多个队列硬件的驱动加速 RFS。 |
| RPS | bool | 软件接收侧数据包导向（RPS）将接收数据包处理的负载分布到多个 CPU 上。 |
| RSEQ | bool | 启用可重启序列系统调用。它提供一个用户空间当前 CPU 编号值的缓存，加速从用户空间获取当前 CPU 编号，以及一个...的 ABI。 |
| RSEQ_DEBUG_DEFAULT_ENABLE | bool | 该选项为可重启序列的调试模式启用静态分支。它也可以通过内核命令行参数 "rseq_debug=0/1" 与 debugfs 控制。如果... |
| RSEQ_SLICE_EXTENSION | bool | 允许用户空间在通过 RSEQ 共享数据 ABI 从中断返回到用户空间时请求有限的时隙扩展。若被授予，这允许完成一个临界区，从而... |
| RSEQ_STATS | bool | 启用轻量级计数器，通过 debugfs 暴露关于 RSEQ 操作频率的信息。主要用于内核调试或性能分析。虽然是轻量级的，但它仍... |
| RT_GROUP_SCHED | bool | 该特性让你显式地为任务组分配真实的 CPU 带宽。若启用，在你为普通用户分配实时带宽之前，将无法调度实时任务... |
| RT_GROUP_SCHED_DEFAULT_DISABLED | bool | 设置时，RT 组调度默认禁用。该选项以反相形式存在，以便单纯的 RT_GROUP_SCHED 即启用组调度。如果不确定，选择 N。 |
| RUNTIME_TESTING_MENU | bool | 启用该选项以包含 Dhrystone 2.1 基准测试。该测试计算每秒 Dhrystone 数，以及当 Dhrystone 分数除以...时获得的 DMIPS（Dhrystone MIPS）数。 |
| RUST | bool | 在内核中启用 Rust 支持。这允许选择其他 Rust 相关选项，例如用 Rust 编写的驱动。要能够加载用 Rust 编写的外部内核模块也需要它... |
| RUSTC_LLVM_VERSION | int | 这表明 Rust 与 Clang 是否使用相同主版本的 LLVM。涉及处理 LLVM IR 或位码（例如跨语言 LTO）的操作需要相同的 LLVM 主版本才能正常工作... |
| RUSTC_VERSION_TEXT | string | 请参阅 `CC_VERSION_TEXT`。 |
| RUST_BUILD_ASSERT_ALLOW | bool | 控制构建期间如何处理 `build_error!` 与 `build_assert!`。如果二进制中存在对它们的调用，可能表明一个被违反的不变量，或优化器未能验证该... |
| RUST_DEBUG_ASSERTIONS | bool | 启用 rustc 的 `-Cdebug-assertions` 代码生成选项。该标志让你开启或关闭 `cfg(debug_assertions)` 条件编译。这可用于在开发时启用额外的调试代码... |
| RUST_INLINE_HELPERS | bool | 使用链接时优化（LTO）将 C 辅助函数内联到 Rust 代码中。若启用该选项，rust/helpers/ 中声明的 C 辅助函数会被内联到 Rust 代码中，这有助于...的性能。 |
| RUST_IS_AVAILABLE | def_bool | 这表明是否有合适的 Rust 工具链可用（已找到）。满足 Rust 支持的构建要求的方法请参阅 Documentation/rust/quick-start.rst。特别... |
| RUST_KERNEL_DOCTESTS | bool | 该选项将 `kernel` crate 的文档测试构建为 KUnit 测试。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-tools/...中的 KUnit 文档。 |
| RUST_OVERFLOW_CHECKS | bool | 启用 rustc 的 `-Coverflow-checks` 代码生成选项。该标志允许你控制运行时整数溢出的行为。当启用溢出检查时，溢出将发生 Rust 恐慌。... |
| SCANF_KUNIT_TEST | tristate | 启用该选项以在运行时测试 scanf 函数。如果不确定，选择 N。 |
| SCF_TORTURE_TEST | tristate | 该选项提供一个内核模块，对 smp_call_function() 系列原语运行 torture 测试。如果需要，该内核模块可以在被测的正在运行的内核上事后构建... |
| SCHED_AUTOGROUP | bool | 该选项通过自动创建并填充任务组来为常见桌面工作负载优化调度器。这种工作负载的分离隔离了激进的 CPU 消耗者（例如构建任务...）。 |
| SCHED_INFO | bool | 如果你在此选择 Y，调度器及相关例程中将插入额外代码，以收集调度器行为统计并提供在 /proc/schedstat 中。这些统计可被... |
| SCHED_PROXY_EXEC | bool | 该选项启用代理执行（proxy execution），一种让持有 mutex 的任务继承更高优先级等待者调度上下文的机制。 |
| SCHED_STACK_END_CHECK | bool | 该选项检查对 schedule() 调用时的栈溢出。如果发现栈末尾位置被覆写，则总是恐慌，因为被损坏区域的内容不再可信。该... |
| SECTION_MISMATCH_WARN_ONLY | bool | 如果你在此选择 N，构建过程将在存在任何节区不匹配时失败，而不仅仅是抛出警告。如果不确定，选择 Y。 |
| SECURITY | bool | 该选项允许你选择不同的安全模块配置进内核。如果未选择该选项，将使用默认 Linux 安全模型。如果你不确定如何回答... |
| SECURITY_COMMONCAP_KUNIT_TEST | bool | This builds the commoncap KUnit tests. KUnit tests run during boot and output the results to the debug log in TAP format (https://testanything.org/). Only useful for kernel devs running KUnit test ... |
| SECURITY_DMESG_RESTRICT | bool | 该选项强制对未特权用户通过 dmesg(8) 读取内核 syslog 的限制。如果未选择该选项，除非 dmesg_restrict sysctl 被显式...，否则不会实施限制。 |
| SECURITY_INFINIBAND | bool | 该选项启用 Infiniband 安全钩子。若启用，安全模块可以使用这些钩子实现 Infiniband 访问控制。如果你不确定如何回答，选择 N。 |
| SECURITY_NETWORK | bool | 该选项启用套接字与网络的安全钩子。若启用，安全模块可以使用这些钩子实现套接字与网络的访问控制。如果你不确定如何回答... |
| SECURITY_NETWORK_XFRM | bool | 该选项启用 XFRM（IPSec）网络的安全钩子。若启用，安全模块可以使用这些钩子基于从 IPSec 策略派生的标签实现逐包访问控制。非 IP... |
| SECURITY_PATH | bool | 该选项启用基于路径名访问控制的安全钩子。若启用，安全模块可以使用这些钩子实现基于路径名的访问控制。如果你不确定如何回答... |
| SELECT_MEMORY_MODEL | def_bool | 该选项允许你更改 Linux 内部管理其内存的一些方式。大多数用户只会由体系结构配置选中其中一个选项。这是正常的。 |
| SEQ_BUF_KUNIT_TEST | tristate | 该选项构建 seq_buf 库的单元测试。如果不确定，选择 N。 |
| SGETMASK_SYSCALL | bool | sys_sgetmask 与 sys_ssetmask 是已废弃的系统调用，libc 不再支持，但在某些体系结构上默认仍启用。如果不确定，保留这里的默认选项。 |
| SG_POOL | def_bool | 提供一个分配链式分散列表（scatterlist）的辅助函数。应由希望分配链式 scatterlist 的驱动或 API 选中。# # sg 链式选项 # |
| SHMEM | bool | shmem 是一个用于管理共享内存的内部文件系统。它由交换区后备并管理资源限制。若启用 TMPFS，它也会作为 tmpfs 导出到用户空间。禁用该选项... |
| SHRINKER_DEBUG | bool | 在此选择 Y 以启用 shrinker 的 debugfs 接口，它提供对内核内存 shrinker 子系统的可见性。禁用它以避免额外的内存占用。 |
| SHUFFLE_PAGE_ALLOCATOR | bool | 页分配器的随机化提高了直接映射的内存侧缓存的平均利用率。请参阅 ACPI 6.2a 规范中第 5.2.27 节异构内存属性表（HMAT）... |
| SIGNALFD | bool | 启用 signalfd() 系统调用，允许在文件描述符上接收信号。如果不确定，选择 Y。 |
| SIGNATURE | tristate | 数字签名验证。目前仅支持 RSA。实现使用 GnuPG MPI 库。 |
| SIPHASH_KUNIT_TEST | tristate | 启用该选项以在启动时（或模块加载时）测试内核的 siphash（<linux/siphash.h>）哈希函数。旨在帮助编写体系结构特定的优化版本。如果不确定... |
| SLAB_BUCKETS | bool | 内核堆攻击常常依赖于能够创建由用户控制内容、特定大小的分配，使其与目标对象分配到同一个 kmalloc 桶中。为... |
| SLAB_FREELIST_HARDENED | bool | 许多内核堆攻击试图针对 slab 缓存元数据与其他基础设施。该选项做出微小的性能牺牲，以加固内核 slab 分配器抵御常见的空闲链表利用... |
| SLAB_FREELIST_RANDOM | bool | 随机化创建新页时使用的空闲链表顺序。该安全特性降低内核 slab 分配器对堆溢出的可预测性。 |
| SLAB_MERGE_DEFAULT | bool | 为减少内核内存碎片，当 slab 缓存共享相同大小与其他特性时可被合并。这带来内核堆溢出能够覆写对象...的风险。 |
| SLAB_OBJ_EXT | bool | 该选项增加对一组进程进行分组管理的支持，用于配合 Cpusets、CFS、内存控制或设备隔离等进程控制子系统。请参阅 - Documentation/scheduler/sc... |
| SLUB | def_bool | 以最小化内存占用方式配置 slab 分配器，牺牲可扩展性、调试与其他特性。这仅用于曾使用 SL...的最小系统。 |
| SLUB_KUNIT_TEST | tristate | 该选项构建 SLUB 分配器单元测试。测试 SLUB 缓存调试功能。有关 KUnit 及单元测试的更多信息，请参阅 Documentation/dev-...中的 KUnit 文档。 |
| SLUB_STATS | bool | 这些统计信息有助于调试 slab 分配行为，以寻找优化分配器的方法。由于保持统计会拖慢整体...，绝不应用于生产环境。 |
| SOCK_CGROUP_DATA | bool | 提供一种让任务使用相同 id 操作不同对象的方式。例如，当在...中使用时，相同的 IPC id 可能引用不同对象，或相同的用户 id 或 pid 可能引用不同任务。 |
| SOCK_RX_QUEUE_MAPPING | bool | 用于在每接口基础上为进程分配网络优先级的 cgroup 子系统。 |
| SOFTLOCKUP_DETECTOR_INTR_STORM | bool | 在此选择 Y 以让内核检测"软锁死"期间的Interrupt Storm（中断风暴）。"软锁死"可由多种原因引起。若由中断风暴引起，则风暴的中断... |
| SPARSEMEM | def_bool | SPARSEMEM_VMEMMAP 使用虚拟映射的 memmap 来优化 pfn_to_page 与 page_to_pfn 操作。在拥有充足内核资源时这是最高效的选项。 |
| SPARSEMEM_MANUAL | bool | 这将是某些系统（包括内存热插拔系统）的唯一选项。这是正常的。该选项为物理地址空间存在空洞的系统提供高效支持，并... |
| SPARSEMEM_VMEMMAP_PREINIT | bool | 热插拔内存的默认内存类型。该选项设置内存热插拔上线策略（/sys/devices/system/memory/auto_online_blocks）的默认策略，决定发生...时的情况。 |
| STACKDEPOT_ALWAYS_INIT | bool | 在启动早期始终初始化栈仓库（stack depot）。 |
| STACKDEPOT_MAX_FRAMES | int | 运行轻量级排队的启动期测试。 |
| STACKINIT_KUNIT_TEST | tristate | 测试内核是否对栈变量与填充进行零初始化。覆盖率由编译器标志 CONFIG_INIT_STACK_ALL_PATTERN 或 CONFIG_INIT_STACK_ALL_ZERO 控制。 |
| STACKTRACE | bool | 该选项使内核为每个进程创建 /proc/pid/stack，显示其当前栈跟踪。它也被各种需要栈跟踪生成的内核调试特性使用。 |
| STACKTRACE_BUILD_ID | bool | 选择该选项会为以 printk 格式 '%p[SR]b' 打印的栈跟踪中的符号添加构建 ID 信息。该选项用于不易获取 debuginfo 的发行版，但... |
| STACK_VALIDATION | bool | 在编译时校验帧指针规则。这有助于确保运行时栈跟踪更可靠。更多信息请参阅 tools/objtool/Documentation/objtool.txt。 |
| STATIC_USERMODEHELPER | bool | 默认情况下，内核可以通过"用户态辅助程序"内核接口调用许多不同的用户空间二进制程序。其中一些二进制在代码或...中被静态定义。 |
| STATIC_USERMODEHELPER_PATH | string | 当任何用户态辅助程序希望运行时，内核调用的二进制。"真正"应用程序的名称将作为第一个参数在命令行上传给该程序。如果你希望... |
| STRING_KUNIT_TEST | tristate | 启用字符串函数的性能测量。它在 KUnit 测试运行期间测量字符串函数的执行效率。如果不确定，选择 N。 |
| STRIP_ASM_SYMS | bool | 在链接期间剥离汇编器内部生成的符号（形如 '.Lxxx' 的符号），使它们不会污染 get_wchan() 等输出的结果。 |
| SYMBOLIC_ERRNAME | bool | 如果你在此选择 Y，内核的 printf 实现将能够打印符号化的错误名（例如 ENOSPC）而非数字 28。它使内核镜像略大（约 3KB），但... |
| SYSCTL_ARCH_UNALIGN_ALLOW | bool | 启用对 /proc/sys/kernel/unaligned-trap 的支持。允许体系结构定义/使用 @unaligned_enabled 来在运行时切换未对齐访问模拟。参考 arch/parisc/kernel/unaligned.c |
| SYSCTL_ARCH_UNALIGN_NO_WARN | bool | 启用对 /proc/sys/kernel/ignore-unaligned-usertrap 的支持。允许体系结构定义/使用 @no_unaligned_warning 以就可能警告底层正在进行的未对齐访问模拟。 |
| SYSCTL_KUNIT_TEST | tristate | 该选项构建 proc sysctl 单元测试，在启动时运行。测试 sysctl 的 API 契约与实现正确性。有关 KUnit 及单元测试的更多信息，请参阅... |
| SYSFS_SYSCALL | bool | sys_sysfs 是一个已废弃的系统调用，libc 不再支持。注意禁用该选项更安全，但可能破坏某些系统的兼容性。如果不确定，在此选择 N。 |
| SYSTEM_DATA_VERIFICATION | def_bool | 使用系统可信密钥环的内容提供公钥，进行 PKCS#7 消息验证。这随后可用于模块验证、kexec 镜像验证与固件...。 |
| SYSVIPC | bool | 进程间通信（IPC）是一套库函数与系统调用，让进程（运行中的程序）同步并交换信息。这通常被认为是一件好事... |
| SYSVIPC_SYSCTL | bool | POSIX 消息队列是 IPC 的一部分。在 POSIX 消息队列中，每个消息都有一个优先级，决定进程接收它的顺序。如果你想编译并运行... |
| TASKSTATS | bool | 通过通用 netlink 接口导出针对任务/进程的选定统计信息。与 BSD 进程记账不同，这些统计在任务/进程的生命周期内作为响应...可用。 |
| TASK_DELAY_ACCT | bool | 收集任务等待系统资源（例如 CPU、同步块 I/O 完成、以及页换入）所花费时间的信息。此类统计有助于设置任务的优先级... |
| TASK_IO_ACCOUNTING | bool | 收集该任务引起的存储 I/O 字节数的信息。如果不确定，选择 N。 |
| TASK_XACCT | bool | 收集扩展的任务记账数据并通过 taskstats 接口将数据发送到用户空间进行处理。如果不确定，选择 N。 |
| TEST_BITOPS | tristate | 该选项构建 "test_bitops" 模块，与 TEST_LKM 模块类似，只是它对 set/clear_bit 宏与 get_count_order/long 做基本演练，以确保没有编译... |
| TEST_BPF | tristate | 该选项构建 "test_bpf" 模块，根据当前设置对 BPF 解释器或 BPF JIT 编译器运行各种测试向量。这对 BPF JIT 编译器...尤其有用。 |
| TEST_CLOCKSOURCE_WATCHDOG | tristate | 启用该选项以创建一个内核模块，触发对时钟源看门狗的测试。该模块可通过 modprobe 或 insmod 加载，加载时即运行，或... |
| TEST_DEBUG_VIRTUAL | tristate | 测试内核检测针对内核虚拟地址映射非线性部分错误调用 virt_to_phys() 的能力。如果不确定，选择 N。 |
| TEST_DIV64 | tristate | 启用该选项以开启 'do_div()' 函数测试。该测试仅在系统启动期间执行一次（因此只影响启动时间），或在模块加载时执行。如果不确定，选择 N。 |
| TEST_DYNAMIC_DEBUG | tristate | 该模块注册一个跟踪器回调，统计 'do_debugging' 函数中已启用的 pr_debug 数量，然后改变其启用状态，调用该函数并比较计数。如果不确定，选择 N。 |
| TEST_FIRMWARE | tristate | 该选项构建 "test_firmware" 模块，创建用于测试固件加载的用户空间接口。这可用于控制固件加载的触发，而无需实际的固件... |
| TEST_FPU | tristate | 启用该选项以添加 /sys/kernel/debug/selftest_helpers/test_fpu，它将触发一系列浮点运算。这用于自测浮点控制寄存器设置... |
| TEST_FREE_PAGES | tristate | 测试是否不会因释放一块页与投机性页引用之间的竞争而发生内存泄漏。如果你的内核已修复该 Bug，加载此模块是安全的。如果 Bug 未... |
| TEST_HEXDUMP | tristate | 启用该选项以在运行时测试 printf 函数。如果不确定，选择 N。 |
| TEST_HMM | tristate | This is a pseudo 设备驱动 solely for testing HMM. Say M here if you want to build the HMM test module. Doing so will allow you to run tools/testing/selftest/vm/hmm-tests. If unsure, say N. |
| TEST_IDA | tristate | Kunit test for miscdevice API, specially its behavior in respect to static and dynamic minor numbers. KUnit tests run during boot and output the results to the debug log in TAP format (https://test... |
| TEST_IOV_ITER | tristate | 启用该选项以开启对 I/O 迭代器（iov_iter）操作的测试。该测试仅在系统启动期间执行一次（因此只影响启动时间），或在模块加载时执行。如果不确定，选择... |
| TEST_KALLSYMS_A | tristate | 选择"快速"以外的选项将启用会拖慢构建并可能使构建崩溃的测试。 |
| TEST_KALLSYMS_FAST | bool | 你并不会真正测试 kallsyms，所以这只是在使用 allmodconfig 时帮助快速构建。 |
| TEST_KALLSYMS_LARGE | bool | 这将启用更多数量的符号。这将显著拖慢你的构建。 |
| TEST_KALLSYMS_MAX | bool | 这将启用导出到我们知道会开始使构建崩溃的程度。 |
| TEST_KALLSYMS_NUMSYMS | int | 在 TEST_KALLSYMS_A 上创建的符号数量，其中只有 TEST_KALLSYMS_B 模块会使用一个。这也用于 TEST_KALLSYMS_C 将拥有的符号数量，按 TEST_KALLS...缩放。 |
| TEST_KALLSYMS_SCALE_FACTOR | int | TEST_KALLSYSMS_C 比 TEST_KALLSYMS_A 多出的未使用符号数量。若为 8，则模块 C 将比模块 A 多 8 * syms 个符号。然后 TEST_KALLSYMS_D 将拥有比...多一倍的符号。 |
| TEST_KEXEC_HANDOVER | bool | 该选项启用对 Kexec HandOver（KHO）的测试。测试包含两部分：在 kexec 前保存内核数据，并在 kexec 后恢复数据并验证其被正确移交...。 |
| TEST_KMOD | tristate | 测试内核的模块加载机制：kmod。kmod 实现使用 Linux 内核的用户态辅助程序加载模块的支持。该测试提供一系列针对 kmod 的测试。尽管技术上... |
| TEST_KSTRTOX | tristate | 启用该选项以在启动时测试位图函数。如果不确定，选择 N。 |
| TEST_LIST_SORT | tristate | 启用该选项以开启 'list_sort()' 函数测试。该测试仅在系统启动期间执行一次（因此只影响启动时间），或在模块加载时执行。如果不确定，选择 N。 |
| TEST_LKM | tristate | 该选项构建 "test_module" 模块，加载时通过 printk 输出 "Hello, world"。它旨在用于模块加载子系统（例如验证模块...）的基本评估。 |
| TEST_LOCKUP | tristate | 该选项构建 "test_lockup" 模块，有助于确保看门狗与锁死检测器正常工作。根据模块参数，它可以模拟软锁死或硬锁死、"挂起任务...。 |
| TEST_MEMCAT_P | tristate | 测试 memcat_p() 辅助函数是否正确合并两个指针数组。如果不确定，选择 N。 |
| TEST_MEMINIT | tristate | 测试内核是否对堆与页分配进行零初始化。这可用于测试 init_on_alloc 与 init_on_free 特性。如果不确定，选择 N。 |
| TEST_MULDIV64 | tristate | 启用该选项以开启 'mul_u64_u64_div_u64()' 函数测试。该测试仅在系统启动期间执行一次（因此只影响启动时间），或在模块加载时执行。如果不确定，选择 N。 |
| TEST_OBJAGG | tristate | 启用该选项以在启动时（或模块加载时）测试对象聚合管理器。 |
| TEST_OBJPOOL | tristate | 该选项构建 "test_objpool" 模块，应用于对象分配与回收的正确性与并发测试。如果不确定，选择 N。 |
| TEST_PARMAN | tristate | 启用该选项以在启动时（或模块加载时）测试优先级数组管理器。如果不确定，选择 N。 |
| TEST_REF_TRACKER | tristate | 该选项提供一个使用引用跟踪器基础设施执行测试的内核模块。如果不确定，选择 N。 |
| TEST_RHASHTABLE | tristate | 启用该选项以在启动时测试 rhashtable 函数。如果不确定，选择 N。 |
| TEST_RUNTIME | bool | 这允许我们通过用于将符号放置在内核 ELF kallsyms 与模块 kallsyms 上的 kallsyms 来对 find_symbol() 进行压力测试，我们在其中放置导出的符号等内核符号。我们已... |
| TEST_SORT | tristate | 该选项在启动时（或模块加载时）启用 'sort()' 的自测函数。如果不确定，选择 N。 |
| TEST_STATIC_KEYS | tristate | 测试静态键（static key）接口。如果不确定，选择 N。 |
| TEST_SYSCTL | tristate | 该选项构建 "test_sysctl" 模块。该驱动能够在不影响可能改变系统功能的生产开关的情况下，安全地测试驱动可用的 proc sysctl 接口。如果... |
| TEST_UDELAY | tristate | 该选项构建 "udelay_test" 模块，有助于确保 udelay() 正常工作。如果不确定，选择 N。 |
| TEST_VMALLOC | tristate | 该选项构建 "test_vmalloc" 模块，应用于压力与性能分析。因此，可以从性能与稳定性角度评估 vmalloc 子系统的任何新变更... |
| TEST_WORKQUEUE | tristate | 该选项构建 "test_workqueue" 模块，用于在有竞争时基准测试工作队列吞吐。有助于评估亲和性范围变更（例如 cache_shard 与 cache）。如果不确定，选择 N。 |
| TEST_XARRAY | tristate | 启用该选项以在启动时或模块加载时测试 maple tree 代码函数。启用 "Debug Maple Trees" 将在失败时输出更详细的错误信息。如果不确定，选择 N。 |
| TEXTSEARCH | bool | 简单、可嵌入的区间树。可以在 log(n) 时间内找到重叠范围的起点，然后遍历所有重叠节点。该算法实现为一个增强的 rbtree。请参阅：D... |
| THP_SWAP | def_bool | 整体交换透明大页，不拆分。XXX：目前，后备透明大页的交换簇将在换出后拆分。供具有合理 THP...支持的体系结构选择。 |
| TIMERFD | bool | 启用 timerfd() 系统调用，允许在文件描述符上接收定时器事件。如果不确定，选择 Y。 |
| TIME_NS | bool | 在该命名空间中，启动时间与单调时钟可以被设置。时间将以相同的节奏继续推进。 |
| TIME_NS_VDSO | def_bool | 在该命名空间中，任务使用与不同命名空间中不同 IPC 对象相对应的 IPC id。 |
| TMPFS | bool | Tmpfs 是一个将所有文件保存在虚拟内存中的文件系统。tmpfs 中的一切都是临时的，即不会在你的硬盘上创建文件。文件存在于内存与交换空间...。 |
| TMPFS_INODE64 | bool | tmpfs 历史上只使用与 unsigned int 一样宽的 inode 号。在某些情况下这可能导致回绕，潜在地导致单个设备上出现多个具有相同 inode 号的文件... |
| TMPFS_POSIX_ACL | bool | POSIX 访问控制列表（ACL）在标准所有者/组/其他方案之外，为用户与组提供额外的访问权限，该选项专门选择对 tmpfs 的 ACL 支持... |
| TMPFS_QUOTA | bool | 配额支持允许设置每个用户与组的 tmpfs 使用限制。选择 Y 以启用配额支持。一旦启用，你可以通过 quota、usrquota 与 grpquot...控制用户与组的配额实施。 |
| TMPFS_XATTR | bool | 扩展属性是由内核或用户与 inode 关联的 名称:值 对（详情请参阅 attr(5) 手册页）。这启用对 trusted.*、security.* 与 user.* 名称...的支持。 |
| TRACE_IRQFLAGS | bool | 启用钩子以中断跟踪或锁调试的启用与禁用。 |
| TRACE_IRQFLAGS_NMI | def_bool | 当 CPU 未能响应给定的 backtrace NMI 时启用调试打印。这些打印提供一些 CPU 可能合理未能响应的原因，例如它处于离线状态或... |
| TRACE_MMIO_ACCESS | bool | 为 MMIO 读/写操作创建跟踪点。这些跟踪事件可用于记录所有 MMIO 读/写操作。 |
| TRANSPARENT_HUGEPAGE_ALWAYS | bool | 始终启用透明大页会增加应用程序的内存占用，却没有保证的收益，但它会对所有应用程序自动生效。 |
| TRANSPARENT_HUGEPAGE_MADVISE | bool | 启用透明大页的 madvise 模式，只会为使用 madvise(MADV_HUGEPAGE) 的应用程序带来性能提升收益，但不会增加应用程序内存占用的风险... |
| TRANSPARENT_HUGEPAGE_NEVER | bool | 默认禁用透明大页。仍可在运行时通过 sysfs 启用。 |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ADVISE | bool | 仅当应用程序提供 madvise(MADV_HUGEPAGE) 提示时，才为 shmem 挂载点独占分配大页。这确保大页仅用于响应来自...的显式请求。 |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ALWAYS | bool | 始终尝试为 shmem 挂载点分配大页，会增加应用程序的内存占用而没有保证的收益，但它会对所有应用程序自动生效。 |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_NEVER | bool | 默认禁用 shmem 挂载点的大页分配。仍可通过内核命令行 'transparent_hugepage_shmem=' 选项或运行时的 sysfs 开关启用。注意 madvise(MADV_COLLAPSE)... |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_WITHIN_SIZE | bool | 如果分配完全在 i_size 之内，则为 shmem 挂载点启用大页分配。该配置也考虑应用程序可能提供的任何 madvise(MADV_HUGEPAGE) 提示...。 |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ADVISE | bool | 仅当应用程序提供 madvise(MADV_HUGEPAGE) 提示时，才为 tmpfs 挂载点独占分配大页。这确保大页仅用于响应来自...的显式请求。 |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ALWAYS | bool | 始终尝试为 tmpfs 挂载点分配大页，会增加应用程序的内存占用而没有保证的收益，但它会对所有应用程序自动生效。 |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_NEVER | bool | 默认禁用 tmpfs 挂载点的大页分配。仍可通过内核命令行 'transparent_hugepage_tmpfs=' 选项启用。注意 madvise(MADV_COLLAPSE) 仍可导致... |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_WITHIN_SIZE | bool | 如果分配完全在 i_size 之内，则为 tmpfs 挂载点启用大页分配。该配置也考虑应用程序可能提供的任何 madvise(MADV_HUGEPAGE) 提示...。 |
| UAPI_HEADER_TEST | bool | 编译导出到用户空间的测试头文件，以确保它们是自包含的（即可作为独立单元编译）。如果你是开发者或测试者，并希望确保导出的头文件是自包含的... |
| UCLAMP_BUCKETS_COUNT | int | 定义要使用的钳制桶（clamp bucket）数量。每个桶的范围为 SCHED_CAPACITY_SCALE/UCLAMP_BUCKETS_COUNT。钳制桶数量越多，其粒度越细，越高... |
| UCS2_STRING | tristate | 提供一个将分散列表拆分为多个块（每块为一个分散列表）的辅助函数。应由希望将分散列表拆分到多个 DMA 通道的驱动或 API 选中。 |
| UID16 | bool | 该选项启用传统的 16 位 UID 系统调用包装器。 |
| USERCOPY_KUNIT_TEST | tristate | 该选项构建 "usercopy_kunit" 模块，对 copy_to/from_user 基础设施运行健全性检查，确保基本的用户/内核边界测试正常工作。 |
| USERFAULTFD | bool | 启用 userfaultfd() 系统调用，允许在用户空间拦截并处理页错误。依赖于 USERFAULTFD |
| USER_NS | bool | 这允许容器（即 vservers）使用用户命名空间为不同服务器提供不同的用户信息。当内核中启用了用户命名空间时，建议同时启用 MEMCG 或... |
| UTIL_MACROS_KUNIT | tristate | Enable this option to test the util_macros.h function at boot. KUnit tests run during boot and output the results to the debug log in TAP format (http://testanything.org/). Only useful for kernel d... |
| UTS_NS | bool | 在该命名空间中，任务看到 uname() 系统调用提供的不同信息。 |
| UUID_KUNIT_TEST | tristate | 该选项启用针对 uuid 库的 KUnit 测试套件，该库提供生成与解析 UUID 和 GUID 的函数。该测试套件检查 UUID 与 GUID 字符串的解析。如果不确定，选择... |
| VIRT_CPU_ACCOUNTING_GEN | bool | 选择该选项以在全动态 tick（dynticks）系统上启用任务与 CPU 时间记账。该记账通过利用上下文跟踪子系统监视每个内核-用户边界来实现。该... |
| VIRT_CPU_ACCOUNTING_NATIVE | bool | 选择该选项以启用更精确的任务与 CPU 时间记账。这通过在每次内核进入与退出以及内核内系统...之间的转换时读取 CPU 计数器来实现。 |
| VMAP_PFN | bool | VM 事件计数器是显示事件计数所必需的。该选项允许在 EXPERT 系统上禁用 VM 事件计数器。如果禁用了 VM 事件计数器，/proc/vmstat 将只显示页计数... |
| WANT_COMPAT_NETLINK_MESSAGES | bool | 该选项可由需要兼容 netlink 消息的其他选项选中。 |
| WARN_ABI_ERRORS | bool | Documentation/ABI 下的文件应遵循 Documentation/ABI/README 中的描述。然而，由于它们是手工编写的，某些文件可能存在一些错误... |
| WARN_CONTEXT_ANALYSIS | bool | 上下文分析（Context Analysis）是一种语言扩展，通过获取与释放用户可定义的"上下文锁"，静态检查所需的上下文是处于活动（或非活动）状态。Clang 将其称为... |
| WARN_CONTEXT_ANALYSIS_ALL | bool | 启用全树范围的上下文分析。这可能会产生大量误报——风险自负启用。如果不确定，选择 N。 |
| WARN_MISSING_DOCUMENTS | bool | 文档被重命名并不罕见。该选项使内核检查缺失的依赖，并在缺失时发出警告。仅在从 git 树构建内核时有效... |
| WERROR | bool | 内核构建不应产生任何编译器警告，该选项启用 '-Werror'（针对 C）与 '-Dwarnings'（针对 Rust）标志以默认强制执行该规则。来自其他工具的某些警告... |
| WQ_CPU_INTENSIVE_REPORT | bool | 在此选择 Y 以启用对占用 CPU 超过 workqueue.cpu_intensive_thresh_us 的并发管理的每 CPU 工作项的报告。工作队列会自动检测并将其排除出并发... |
| WQ_WATCHDOG | bool | 在此选择 Y 以启用工作队列上的停顿（stall）检测。如果一个工作池在超过给定时间（默认 30 秒）内对挂起的工作项没有取得进展，则会打印警告消息... |
| WW_MUTEX_SELFTEST | tristate | 该选项提供一个在内核 struct ww_mutex 锁 API 上运行测试的内核模块。建议配合 DEBUG_WW_MUTEX_SLOWPATH 启用此测试工具。如果...选择 M。 |
| XXHASH | tristate | 该选项启用 32 位 PRNG 库函数的初始化自测。# # 压缩支持在需要时被 select # |
| ZSMALLOC_CHAIN_SIZE | int | 该选项设置 zmalloc 页（zspage）可由其组成的物理页数量上限。最优的 zspage 链大小在初始化期间为每个大小类计算... |
| ZSWAP | bool | 一个用于交换页的轻量级压缩缓存。它获取正在被换出的页，并尝试将它们压缩到动态分配的基于 RAM 的内存池中。这可... |
| ZSWAP_COMPRESSOR_DEFAULT | string | 该选项在 zsmalloc 中启用代码以收集关于 zsmalloc 内部发生情况的各类统计，并通过 debugfs 将信息导出到用户空间。如果不确定，选择 N。 |
| ZSWAP_COMPRESSOR_DEFAULT_842 | bool | 使用 842 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_DEFLATE | bool | 使用 Deflate 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_LZ4 | bool | 使用 LZ4 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_LZ4HC | bool | 使用 LZ4HC 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_LZO | bool | 使用 LZO 算法作为默认压缩算法。 |
| ZSWAP_COMPRESSOR_DEFAULT_ZSTD | bool | 使用 zstd 算法作为默认压缩算法。 |
| ZSWAP_DEFAULT_ON | bool | 若选中，交换页的压缩缓存将在启动时启用，否则禁用。此处的选择可通过内核命令行 'zswap.enabled='...覆盖。 |
| ZSWAP_SHRINKER_DEFAULT_ON | bool | 若选中，将启用 zswap shrinker，存储在 zswap 池中的页将在内存压力下可用于回收（即写回后备交换设备）。这意味着... |
| if | bool | 如果你在此选择 Y，gcc 会被指示为结构体类型生成较少的调试信息。这意味着需要完整调试信息的工具（如 kgdb 或 systemtap）会不满意。但... |
| select | bool | 生成 DWARF v5 调试信息。需要 binutils 2.35.2、gcc 5.0+（gcc 5.0+ 接受 -gdwarf-5 标志，但对某些草案特性直到 7.0 才仅有部分支持）以及 gdb 8.0+。对...的更改。 |

---

# Makefile Targets

## Build targets

| Target | 描述 | 来源 |
|--------|-------------|--------|
| all | 如果构建外部模块，我们不关心 all: 规则，而是让 __all 依赖于 modules | Makefile |
| dtbs_install |  | Makefile |
| headers |  | Makefile |
| headers_install |  | Makefile |
| modules | 构建所有可加载的内核模块 | Makefile |
| modules_install |  | Makefile |
| vmlinux |  | Makefile |

## Configuration targets

| Target | 描述 | 来源 |
|--------|-------------|--------|
| config |  | Makefile |

## Clean targets

| Target | 描述 | 来源 |
|--------|-------------|--------|
| clean | clean - 删除大部分内容，但保留足够内容以构建外部模块 | Makefile |
| distclean | distclean | Makefile |
| mrproper | mrproper - 删除所有生成的文件，包括 .config | Makefile |

## Documentation targets

| Target | 描述 | 来源 |
|--------|-------------|--------|
| cleandocs | 删除所有生成的文档文件 | Makefile |
| htmldocs-redirects |  | Makefile |
| markdowndocs | 通过 Pandoc 后处理构建 Markdown 文档 | Makefile |
| refcheckdocs | 检查文档中损坏的文件引用 | Makefile |

## Other targets

| Target | 描述 | 来源 |
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
| rust-analyzer | 为 rust-analyzer（语言服务器协议的一种实现）生成 rust-project.json（描述非 Cargo Rust 项目结构的文件）。 | Makefile |
| rustavailable | "Rust 是否可用？" 目标 | Makefile |
| rustdoc | 文档目标  使用单数形式以避免违反 `no-dot-config-targets`。 | Makefile |
| rustfmt | 格式化目标  生成的文件以及 vendored crate 会被跳过。 | Makefile |
| rustfmtcheck |  | Makefile |
| rusttest | 测试目标 | Makefile |
| scripts | 构建于 scripts/ 中的额外辅助程序。仔细列出依赖，以免我们尝试并行构建 scripts 两次 | Makefile |
| scripts_basic | 构建于 scripts/basic/ 中的基本辅助程序 | Makefile |
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
