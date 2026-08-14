# Linux 鍐呮牳椤圭洰姒傝

> 鐢辨簮鐮佹爲 `D:\WORKSPACE\linux-7.1.3` 鐢熸垚

---

# 鐩綍缁撴瀯

## arch/

鐗瑰畾浜庝綋绯荤粨鏋勭殑浠ｇ爜锛坅rm64銆亁86銆乺iscv銆乵68k銆乸owerpc 绛夛級浠ュ強寮曞鍩虹璁炬柦銆?

- `alpha/` 鈥?# alpha/Makefile #
- `arc/` 鈥?SPDX-License-Identifier: GPL-2.0-only # # Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
- `arm/` 鈥?# arch/arm/Makefile #
- `arm64/` 鈥?# arch/arm64/Makefile #
- `csky/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `hexagon/` 鈥?SPDX-License-Identifier: GPL-2.0#  鐢ㄤ簬Hexagon arch 鐨?Makefile# Do not use GP-relative jumps
- `loongarch/` 鈥?SPDX-License-Identifier: GPL-2.0 # # Author: Huacai Chen <chenhuacai@loongson.cn>
- `m68k/` 鈥?# m68k/Makefile #
- `microblaze/` 鈥?SPDX-License-Identifier: GPL-2.0# 鎴戜滑姝ｅ湪涓哄摢涓?CPU 鐗堟湰鏋勫缓锛屽苟鎷嗚В瀹?# 褰㈠紡涓?major.minor.rev
- `mips/` 鈥?# This file is subject to the terms and conditions of the GNU General Public # License.  See the file "COPYING" in the main directory of this archive
- `nios2/` 鈥?# This file is subject to the terms and conditions of the GNU General Public # License.  See the file "COPYING" in the main directory of this archive
- `openrisc/` 鈥?BK Id: %F% %I% %G% %U% %#% # # This file is included by the global makefile so that you can add your own
- `parisc/` 鈥?# parisc/Makefile #
- `powerpc/` 鈥?鏈枃浠剁敱鍏ㄥ眬 makefile 鍖呭惈锛屼互渚夸綘鍙互娣诲姞鑷繁鐨?# 浣撶郴缁撴瀯鐗瑰畾鏍囧織鍜屼緷璧栭」銆?
- `riscv/` 鈥?鏈枃浠剁敱鍏ㄥ眬 makefile 鍖呭惈锛屼互渚夸綘鍙互娣诲姞鑷繁鐨?# 浣撶郴缁撴瀯鐗瑰畾鏍囧織鍜屼緷璧栭」銆?
- `s390/` 鈥?SPDX-License-Identifier: GPL-2.0# # s390/Makefile
- `sh/` 鈥?# arch/sh/Makefile #
- `sparc/` 鈥?SPDX-License-Identifier: GPL-2.0# # sparc/Makefile
- `um/` 鈥?# 鏈枃浠剁敱鍏ㄥ眬 makefile 鍖呭惈锛屼互渚夸綘鍙互娣诲姞鑷繁鐨?# 浣撶郴缁撴瀯鐗瑰畾鏍囧織鍜屼緷璧栭」銆?
- `x86/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬 i386 鍜?x86_64 鐨勭粺涓€ Makefile # 鏍规嵁瀹為檯鏋舵瀯閫夋嫨 defconfig
- `xtensa/` 鈥?# This file is subject to the terms and conditions of the GNU General Public # License.  See the file "COPYING" in the main directory of this archive

## crypto/

鍔犲瘑 API 涓庣畻娉曞疄鐜般€?

- `asymmetric_keys/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬闈炲绉板姞瀵嗗瘑閽?鐨?Makefile
- `async_tx/` 鈥?SPDX-License-Identifier: GPL-2.0
- `krb5/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬闈炲绉板姞瀵嗗瘑閽?鐨?Makefile

## drivers/

璁惧椹卞姩锛堢綉缁溿€佸潡璁惧銆佸瓧绗﹁澶囥€佸０鍗°€丟PU銆乁SB銆丳CI銆乮nfiniband 绛夛級浠ュ強椹卞姩鏍稿績銆?

- `accel/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `accessibility/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `acpi/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux ACPI 瑙ｉ噴鍣?鐨?Makefile
- `amba/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `android/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `ata/` 鈥?SPDX-License-Identifier: GPL-2.0# 闈?SFF 鎺ュ彛
- `atm/` 鈥?SPDX-License-Identifier: GPL-2.0
- `auxdisplay/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳杈呭姪鏄剧ず璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `base/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬Linux 璁惧鏍?鐨?Makefile
- `bcma/` 鈥?Broadcom 鎺ㄥ嚭浜嗕竴绉嶆柊鐨勬€荤嚎鏉ユ浛浠ｈ緝鏃х殑 SSB銆傚畠鍩轰簬 AMBA锛屼絾浠庣紪绋嬭搴︾湅锛屾垜浠苟娌℃湁浣跨敤浠讳綍 AMBA 鐗瑰畾鐨勫唴瀹广€傛爣鍑嗙殑 AMBA 椹卞姩鏄钩鍙扮壒瀹氱殑锛屽叿鏈夌‖缂栫爜鍦板潃锛屽苟浣跨敤璇稿 CID 鍜?PID 涔嬬被鐨?AMBA 鏍囧噯瀛楁銆傚湪 Broadcom 鐨勭綉鍗′腑锛屾瘡涓澶囩敱浠ヤ笅閮ㄥ垎缁勬垚锛?) Broadcom 鐗瑰畾鐨?AMBA 璁惧銆傚畠琚斁鍦?AMBA 鎬荤嚎涓婏紝浣嗕笉鑳戒綔涓烘爣鍑?AMBA 璁惧澶勭悊銆傝鍙栧叾 CID 鎴?PID 鍙兘瀵艰嚧鏈哄櫒姝婚攣銆?) AMBA s...
- `block/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳鍧楄澶囬┍鍔ㄧ▼搴?鐨?Makefile.
- `bluetooth/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux Bluetooth HCI 璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `bus/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鎬荤嚎椹卞姩绋嬪簭 鐨?Makefile.
- `cache/` 鈥?SPDX-License-Identifier: GPL-2.0
- `cdrom/` 鈥?SPDX-License-Identifier: GPL-2.0
- `cdx/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬CDX 鐨?Makefile
- `char/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳瀛楃璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `clk/` 鈥?SPDX-License-Identifier: GPL-2.0# 閫氱敤鏃堕挓绫诲瀷
- `clocksource/` 鈥?SPDX-License-Identifier: GPL-2.0
- `comedi/` 鈥?SPDX-License-Identifier: GPL-2.0
- `connector/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `counter/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬璁℃暟鍣ㄨ澶?鐨?Makefile
- `cpufreq/` 鈥?SPDX-License-Identifier: GPL-2.0# CPUfreq 鏍稿績 # CPUfreq 缁熻
- `cpuidle/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬cpuidle 鐨?Makefile.
- `crypto/` 鈥?SPDX-License-Identifier: GPL-2.0# __init ordering requires atmel-i2c being before atmel-ecc and atmel-sha204a.
- `cxl/` 鈥?SPDX-License-Identifier: GPL-2.0# Order is important here for the built-in case: # - 'core' first for fundamental init
- `dax/` 鈥?SPDX-License-Identifier: GPL-2.0
- `dca/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `devfreq/` 鈥?SPDX-License-Identifier: GPL-2.0# DEVFREQ 椹卞姩 # DEVFREQ 浜嬩欢椹卞姩
- `dibs/` 鈥?SPDX-License-Identifier: GPL-2.0# # DIBS class module
- `dio/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux 鍐呮牳 鐨?Makefile.
- `dma/` 鈥?SPDX-License-Identifier: GPL-2.0#dmaengine 璋冭瘯鏍囧織 #core
- `dma-buf/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `dpll/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬DPLL drivers 鐨?Makefile.
- `edac/` 鈥?# 鐢ㄤ簬 Linux 鍐呮牳 EDAC 椹卞姩绋嬪簭鐨?Makefile銆?
- `eisa/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬Linux 璁惧鏍?鐨?Makefile# virtual_root.o should be the last EISA root device to initialize,
- `extcon/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬external connector class (extcon) devices 鐨?Makefile#
- `firewire/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux IEEE 1394 瀹炵幇 鐨?Makefile
- `firmware/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux 鍐呮牳 鐨?Makefile.
- `fpga/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬fpga 妗嗘灦鍜?fpga 绠＄悊鍣ㄩ┍鍔ㄧ▼搴?鐨?Makefile.
- `fsi/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `fwctl/` 鈥?SPDX-License-Identifier: GPL-2.0
- `gnss/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬GNSS 瀛愮郴缁?鐨?Makefile.
- `gpib/`
- `gpio/` 鈥?SPDX-License-Identifier: GPL-2.0# 閫氱敤 gpio 鏀寔锛氬钩鍙伴┍鍔ㄣ€佷笓鐢ㄦ墿灞曞櫒鑺墖绛?# 璁惧椹卞姩銆傞€氬父淇濇寔鍒楄〃鎸夊瓧姣嶆帓搴?
- `gpu/` 鈥?SPDX-License-Identifier: GPL-2.0-only# drm/tegra depends on host1x, so if both drivers are built-in care must be # taken to initialize them in the correct order. Link order is the only way
- `greybus/` 鈥?SPDX-License-Identifier: GPL-2.0# Greybus 鏍稿績 # 璺熻釜浜嬩欢鎵€闇€
- `hid/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬HID 椹卞姩 鐨?Makefile
- `hsi/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬HSI 鐨?Makefile
- `hte/`
- `hv/` 鈥?SPDX-License-Identifier: GPL-2.0
- `hwmon/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬sensor chip drivers 鐨?Makefile.
- `hwspinlock/` 鈥?SPDX-License-Identifier: GPL-2.0# # Generic Hardware Spinlock framework
- `hwtracing/`
- `i2c/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬i2c 鏍稿績 鐨?Makefile.
- `i3c/` 鈥?SPDX-License-Identifier: GPL-2.0
- `idle/` 鈥?SPDX-License-Identifier: GPL-2.0-only# Branch profiling isn't noinstr-safe
- `iio/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬宸ヤ笟 I/O 鏍稿績 鐨?Makefile.
- `infiniband/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `input/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬杈撳叆鏍稿績椹卞姩 鐨?Makefile.
- `interconnect/` 鈥?SPDX-License-Identifier: GPL-2.0
- `iommu/` 鈥?SPDX-License-Identifier: GPL-2.0
- `ipack/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬IPACK 妗ユ帴璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `irqchip/` 鈥?SPDX-License-Identifier: GPL-2.0
- `leds/` 鈥?SPDX-License-Identifier: GPL-2.0# LED 鏍稿績 # LED 骞冲彴椹卞姩锛堜繚鎸佹帓搴忥紝M-| sort锛?
- `macintosh/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Macintosh 鐗瑰畾璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `mailbox/` 鈥?SPDX-License-Identifier: GPL-2.0# 閫氱敤 MAILBOX API
- `mcb/` 鈥?SPDX-License-Identifier: GPL-2.0
- `md/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳杞欢 RAID 鍜?LVM 椹卞姩绋嬪簭 鐨?Makefile.
- `media/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳澶氬獟浣撹澶囬┍鍔ㄧ▼搴?鐨?Makefile.
- `memory/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呭瓨璁惧 鐨?Makefile
- `memstick/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬鍐呮牳 MemoryStick 璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `message/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬鍩轰簬 MPT 鐨勫潡璁惧 鐨?Makefile
- `mfd/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬澶氬姛鑳芥潅椤硅澶?鐨?Makefile
- `misc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬纭疄鏃犲瀹夋斁鐨勬潅椤硅澶?鐨?Makefile.
- `mmc/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬鍐呮牳 mmc 璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `most/` 鈥?SPDX-License-Identifier: GPL-2.0
- `mtd/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬瀛樺偍鍣ㄦ妧鏈澶囬┍鍔ㄧ▼搴?鐨?Makefile.
- `mux/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬澶氳矾澶嶇敤鍣ㄨ澶?鐨?Makefile.
- `net/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux 缃戠粶璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `nfc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬nfc 璁惧 鐨?Makefile
- `ntb/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `nubus/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬nubus 鐗瑰畾椹卞姩绋嬪簭 鐨?Makefile.
- `nvdimm/` 鈥?SPDX-License-Identifier: GPL-2.0
- `nvme/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `nvmem/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬nvmem drivers 鐨?Makefile.
- `of/` 鈥?SPDX-License-Identifier: GPL-2.0
- `opp/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `parisc/` 鈥?/* ** HP VISUALIZE 宸ヤ綔绔?PCI 鎬荤嚎缂洪櫡 ** ** 鈥淗P 鍙戠幇浜嗕竴涓綔鍦ㄧ殑绯荤粺缂洪櫡锛屽彲鑳藉奖鍝?** 浜旀 HP VISUALIZE 宸ヤ綔绔欐満鍨嬪湪閰嶅 ** 绗笁鏂规垨瀹㈡埛鑷瀹夎鐨?PCI I/O 鎵╁睍鍗℃椂鐨勮涓恒€?** 璇ョ己闄蜂粎闄愪簬 HP C180銆丆160銆丆160L銆丅160L 鍜?** B132L VISUALIZE 宸ヤ綔绔欙紝涓斿彧浼氬湪 ** 閫氳繃 PCI 鎬荤嚎涓婄殑 PCI I/O 鎵╁睍鍗′紶杈撴暟鎹椂鍑虹幇銆?** HP 鎻愪緵鐨勬樉鍗″鏋?..
- `parport/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳骞惰绔彛璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `pci/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬PCI 鎬荤嚎鐗瑰畾椹卞姩绋嬪簭 鐨?Makefile.
- `pcmcia/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳 pcmcia 瀛愮郴缁燂紙鐢?David Hinds 缁存姢锛?鐨?Makefile
- `peci/` 鈥?SPDX-License-Identifier: GPL-2.0-only# Core functionality # Hardware specific bus drivers
- `perf/` 鈥?SPDX-License-Identifier: GPL-2.0
- `phy/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬phy 椹卞姩 鐨?Makefile.
- `pinctrl/` 鈥?SPDX-License-Identifier: GPL-2.0# 閫氱敤 pinmux 鏀寔
- `platform/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬linux/drivers/platform 鐨?Makefile
- `pmdomain/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `pnp/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux 鍗虫彃鍗崇敤鏀寔 鐨?Makefile.
- `power/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `powercap/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `pps/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬PPS 鏍稿績 鐨?Makefile.
- `ps3/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `ptp/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬PTP 1588 鏃堕挓鏀寔 鐨?Makefile.
- `pwm/` 鈥?SPDX-License-Identifier: GPL-2.0
- `rapidio/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬RapidIO interconnect services 鐨?Makefile
- `ras/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `regulator/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬璋冭妭鍣ㄩ┍鍔ㄧ▼搴?鐨?Makefile.
- `remoteproc/` 鈥?SPDX-License-Identifier: GPL-2.0# # Generic framework for controlling remote processors
- `resctrl/`
- `reset/` 鈥?SPDX-License-Identifier: GPL-2.0
- `rpmsg/` 鈥?SPDX-License-Identifier: GPL-2.0
- `rtc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬RTC 绫?椹卞姩绋嬪簭 鐨?Makefile.
- `s390/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬S/390 鐗瑰畾璁惧椹卞姩绋嬪簭 鐨?Makefile
- `sbus/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux 鍐呮牳 鐨?Makefile.
- `scsi/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬linux/drivers/scsi 鐨?Makefile
- `sh/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬SuperH 鐗瑰畾椹卞姩绋嬪簭 鐨?Makefile.
- `siox/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `slimbus/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳 SLIMbus 妗嗘灦 鐨?Makefile.
- `soc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux 鍐呮牳 SOC 鐗瑰畾璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `soundwire/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬soundwire 鏍稿績 鐨?Makefile
- `spi/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳 SPI 椹卞姩绋嬪簭 鐨?Makefile.
- `spmi/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬鍐呮牳 SPMI 妗嗘灦 鐨?Makefile.
- `ssb/` 鈥?SPDX-License-Identifier: GPL-2.0# 鏍稿績 # 涓绘満鏀寔
- `staging/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬staging directory 鐨?Makefile
- `target/` 鈥?SPDX-License-Identifier: GPL-2.0
- `tc/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux 鍐呮牳 鐨?Makefile.
- `tee/` 鈥?SPDX-License-Identifier: GPL-2.0
- `thermal/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬sensor chip drivers 鐨?Makefile.
- `thunderbolt/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `tty/` 鈥?SPDX-License-Identifier: GPL-2.0# tty 椹卞姩
- `ufs/` 鈥?SPDX-License-Identifier: GPL-2.0# The link order is important here. ufshcd-core must initialize # before vendor drivers.
- `uio/` 鈥?SPDX-License-Identifier: GPL-2.0
- `usb/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍐呮牳 USB 璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `vdpa/` 鈥?SPDX-License-Identifier: GPL-2.0
- `vfio/` 鈥?SPDX-License-Identifier: GPL-2.0
- `vhost/` 鈥?SPDX-License-Identifier: GPL-2.0
- `video/` 鈥?SPDX-License-Identifier: GPL-2.0
- `virt/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬鏀寔铏氭嫙鍖栫殑椹卞姩绋嬪簭 鐨?Makefile
- `virtio/` 鈥?SPDX-License-Identifier: GPL-2.0
- `w1/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Dallas 1-wire 鎬荤嚎 鐨?Makefile.
- `watchdog/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬WatchDog 璁惧椹卞姩绋嬪簭 鐨?Makefile.
- `xen/` 鈥?SPDX-License-Identifier: GPL-2.0
- `zorro/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Zorro 鎬荤嚎鐗瑰畾椹卞姩绋嬪簭 鐨?Makefile.

## fs/

鏂囦欢绯荤粺锛坋xt4銆乥trfs銆亁fs銆乫use銆乷verlayfs銆乶fs銆乯ffs2銆乧ramfs 绛夛級銆?

- `9p/` 鈥?SPDX-License-Identifier: GPL-2.0
- `adfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux adfs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `affs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux affs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `afs/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Red Hat Linux AFS client 鐨?Makefile.
- `autofs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux autofs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `befs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux BeOS 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `bfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬BFS 鏂囦欢绯荤粺 鐨?Makefile.
- `btrfs/` 鈥?SPDX-License-Identifier: GPL-2.0# W=1 璀﹀憡鐨勫瓙闆?# 浠ヤ笅鍏抽棴鐢?-Wextra 鍚敤鐨勮鍛?
- `cachefiles/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鍦ㄥ凡鎸傝浇鏂囦欢绯荤粺涓繘琛岀紦瀛?鐨?Makefile
- `ceph/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬CEPH filesystem 鐨?Makefile.
- `coda/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux Coda 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `configfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬configfs 铏氭嫙鏂囦欢绯荤粺 鐨?Makefile
- `cramfs/` 鈥?鏂囦欢绯荤粺甯冨眬璇存槑 --------------------------  杩欎簺璇存槑鎻忚堪浜?mkcramfs 鐢熸垚鐨勫唴瀹广€傚唴鏍哥殑瑕佹眰绋嶅井瀹芥澗涓€浜涳紝渚嬪瀹冧笉鍏冲績 <file_data> 椤规槸鍚﹁浜ゆ崲浜嗕綅缃紙浣嗗畠纭疄瑕佹眰缁欏畾鐩綍涓殑鐩綍椤癸紙inode锛夋槸杩炵画鐨勶紝鍥犱负 readdir 浼氱敤鍒拌繖涓€鐐癸級銆傜洰鍓嶆墍鏈夋暟鎹兘閲囩敤涓绘満瀛楄妭搴忔牸寮忥紱mkcramfs 鍜屽唴鏍搁兘涓嶄細杩涜瀛楄妭浜ゆ崲銆傦紙璇﹁涓嬫枃鐨?鈥淏lock Size鈥?灏忚妭銆傦級
- `crypto/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `debugfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `devpts/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux /dev/pts 铏氭嫙鏂囦欢绯荤粺 鐨?Makefile.
- `dlm/` 鈥?SPDX-License-Identifier: GPL-2.0
- `ecryptfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux eCryptfs 鐨?Makefile
- `efivarfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬efivarfs 鏂囦欢绯荤粺 鐨?Makefile
- `efs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux efs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `erofs/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `exfat/` 鈥?SPDX-License-Identifier: GPL-2.0-or-later# # 鐢ㄤ簬linux exFAT filesystem support 鐨?Makefile.
- `exportfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬鏂囦欢绯荤粺瀵煎嚭鏀寔渚嬬▼ 鐨?Makefile.
- `ext2/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux ext2 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `ext4/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux ext4 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `f2fs/` 鈥?SPDX-License-Identifier: GPL-2.0
- `fat/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux fat 鏂囦欢绯荤粺鏀寔 鐨?Makefile.
- `freevxfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # VxFS Makefile
- `fuse/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬FUSE filesystem 鐨?Makefile.
- `gfs2/` 鈥?SPDX-License-Identifier: GPL-2.0
- `hfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux hfs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `hfsplus/` 鈥?SPDX-License-Identifier: GPL-2.0# ## 鐢ㄤ簬linux hfsplus filesystem routines 鐨?Makefile.
- `hostfs/` 鈥?# Copyright (C) 2000 Jeff Dike (jdike@karaya.com) # Licensed under the GPL
- `hpfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux hpfs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `hugetlbfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux ramfs 渚嬬▼ 鐨?Makefile.
- `iomap/` 鈥?SPDX-License-Identifier: GPL-2.0-or-later # # Copyright (c) 2019 Oracle.
- `isofs/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux isofs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `jbd2/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux 鏃ュ織渚嬬▼ 鐨?Makefile.
- `jffs2/` 鈥?JFFS2 鍔犻攣鏂囨。 	---------------------------  鏈枃妗ｈ瘯鍥炬弿杩?JFFS2 鐜版湁鐨勫姞閿佽鍒欍€傚畠骞朵笉淇濊瘉濮嬬粓瀹屽叏鏈€鏂帮紝浣嗗簲褰撶浉褰撴帴杩戙€?  	alloc_sem
- `jfs/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux JFS 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `kernfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬kernfs 浼枃浠剁郴缁?鐨?Makefile
- `lockd/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux 閿佺鐞嗗櫒鐩稿叧鍐呭 鐨?Makefile
- `minix/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux minix 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `netfs/` 鈥?SPDX-License-Identifier: GPL-2.0
- `nfs/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux nfs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `nfs_common/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬瀹㈡埛绔笌鏈嶅姟鍣ㄥ叡浜殑 Linux 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `nfsd/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux nfs 鏈嶅姟鍣?鐨?Makefile
- `nilfs2/` 鈥?SPDX-License-Identifier: GPL-2.0
- `nls/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬鏈湴璇█鏀寔 鐨?Makefile
- `notify/` 鈥?SPDX-License-Identifier: GPL-2.0
- `ntfs/` 鈥?SPDX-License-Identifier: GPL-2.0
- `ntfs3/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ntfs3 鏂囦欢绯荤粺鏀寔 鐨?Makefile.
- `ocfs2/` 鈥?SPDX-License-Identifier: GPL-2.0
- `omfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `openpromfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux Sun Openprom 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `orangefs/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ORANGEFS 鏂囦欢绯荤粺 鐨?Makefile.
- `overlayfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬overlay 鏂囦欢绯荤粺 鐨?Makefile.
- `proc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux proc 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `pstore/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux pstorefs 渚嬬▼ 鐨?Makefile.
- `qnx4/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux qnx4 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `qnx6/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux qnx4 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `quota/` 鈥?SPDX-License-Identifier: GPL-2.0
- `ramfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux ramfs 渚嬬▼ 鐨?Makefile.
- `resctrl/` 鈥?SPDX-License-Identifier: GPL-2.0# To allow define_trace.h's recursive include:
- `romfs/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux RomFS 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `smb/` 鈥?SPDX-License-Identifier: GPL-2.0
- `squashfs/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux squashfs 渚嬬▼ 鐨?Makefile.
- `sysfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬sysfs 铏氭嫙鏂囦欢绯荤粺 鐨?Makefile
- `tests/`
- `tracefs/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `ubifs/` 鈥?SPDX-License-Identifier: GPL-2.0
- `udf/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux udf 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `ufs/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux ufs 鏂囦欢绯荤粺渚嬬▼ 鐨?Makefile.
- `unicode/` 鈥?鏈洰褰曚腑鐨?utf8data.c 鏂囦欢鐢?Unicode 瀛楃鏁版嵁搴撶敓鎴愶紝瀵瑰簲 Unicode 鏍囧噯鐨?12.1.0 鐗堟湰銆傚畬鏁寸殑鏂囦欢闆嗗彲鍦ㄦ澶勬壘鍒帮細    http://www.unicode.org/Public/12.1.0/ucd/  鍚勪釜婧愭枃浠堕摼鎺ワ細    https://www.unicode.org/Public/12.1.0/ucd/CaseFolding.txt
- `vboxsf/` 鈥?SPDX-License-Identifier: MIT
- `verity/` 鈥?SPDX-License-Identifier: GPL-2.0
- `xfs/` 鈥?SPDX-License-Identifier: GPL-2.0 # # Copyright (c) 2000-2005 Silicon Graphics, Inc.
- `zonefs/` 鈥?SPDX-License-Identifier: GPL-2.0

## include/

鍐呮牳鍏叡澶存枃浠讹紙linux/銆乤sm-generic/銆乽api/锛夈€?

- `acpi/`
- `asm-generic/`
- `clocksource/`
- `crypto/` 鈥?鍔犲瘑 API 涓庣畻娉曞疄鐜般€?
- `cxl/`
- `drm/` 鈥?SPDX-License-Identifier: GPL-2.0# Ensure drm headers are self-contained and pass kernel-doc # Include the header twice to detect missing include guard.
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
- `net/` 鈥?缃戠粶鍗忚鏍堬紙ipv4銆乮pv6銆乶etfilter銆丅PF銆佹牳蹇冦€佷互澶綉銆佹棤绾跨瓑锛夈€?
- `pcmcia/`
- `ras/`
- `rdma/`
- `rv/`
- `scsi/`
- `soc/`
- `sound/` 鈥?ALSA 澹伴煶瀛愮郴缁熶笌闊抽椹卞姩銆?
- `target/`
- `trace/`
- `uapi/`
- `ufs/`
- `vdso/`
- `video/`
- `xen/`

## io_uring/

io_uring 寮傛 I/O 瀛愮郴缁熴€?


## ipc/

杩涚▼闂撮€氫俊锛坢sg銆乻em銆乻hm锛夈€?


## kernel/

鏍稿績鍐呮牳瀛愮郴缁燂紙璋冨害鍣ㄣ€乸rintk銆乮rq銆佹椂闂淬€佸姞閿併€丷CU銆丅PF 绛夛級銆?

- `bpf/` 鈥?SPDX-License-Identifier: GPL-2.0# ___bpf_prog_run() needs GCSE disabled on x86; see 3193c0836f203 for details
- `cgroup/` 鈥?SPDX-License-Identifier: GPL-2.0
- `configs/`
- `debug/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux 鍐呮牳璋冭瘯鍣?鐨?Makefile
- `dma/` 鈥?SPDX-License-Identifier: GPL-2.0
- `entry/` 鈥?SPDX-License-Identifier: GPL-2.0# Prevent the noinstr section from being pestered by sanitizer and other goodies # as long as these things cannot be disabled per function.
- `events/` 鈥?SPDX-License-Identifier: GPL-2.0
- `futex/` 鈥?SPDX-License-Identifier: GPL-2.0
- `gcov/` 鈥?SPDX-License-Identifier: GPL-2.0
- `irq/` 鈥?SPDX-License-Identifier: GPL-2.0
- `kcsan/` 鈥?SPDX-License-Identifier: GPL-2.0
- `livepatch/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `liveupdate/` 鈥?SPDX-License-Identifier: GPL-2.0
- `locking/` 鈥?SPDX-License-Identifier: GPL-2.0# Any varying coverage in these files is non-deterministic # and is generally not a function of system call inputs.
- `module/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬linux kernel module support 鐨?Makefile
- `power/` 鈥?SPDX-License-Identifier: GPL-2.0
- `printk/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `rcu/` 鈥?SPDX-License-Identifier: GPL-2.0# Any varying coverage in these files is non-deterministic # and is generally not a function of system call inputs.
- `sched/` 鈥?SPDX-License-Identifier: GPL-2.0# The compilers are complaining about unused variables inside an if(0) scope # block. This is daft, shut them up.
- `time/` 鈥?SPDX-License-Identifier: GPL-2.0# Branch profiling isn't noinstr-safe
- `trace/` 鈥?SPDX-License-Identifier: GPL-2.0# Do not instrument the tracer itself: # Avoid recursion due to instrumentation.
- `unwind/`

## lib/

鍐呮牳閫氱敤搴擄紙浣嶅浘銆乺btree銆乺adix-tree銆乧rc銆乲unit 绛夛級銆?

- `842/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `crc/` 鈥?SPDX-License-Identifier: GPL-2.0-only# 鐢ㄤ簬鍐呮牳寰幆鍐椾綑鏍￠獙锛圕RC锛夊簱浠ｇ爜 鐨?Makefile
- `crypto/` 鈥?SPDX-License-Identifier: GPL-2.0
- `dim/` 鈥?# DIM 鍔ㄦ€佷腑鏂皟鑺傚簱 #
- `fonts/` 鈥?SPDX-License-Identifier: GPL-2.0# 瀛椾綋澶勭悊 # 鍐呭缓瀛椾綋锛涙寜 Family-Size 鍗囧簭鎺掑簭
- `kunit/` 鈥?KUnit 鐨?鈥渉ooks鈥?鍗充究鍦?KUnit 浣滀负妯″潡鏋勫缓鏃朵篃鏄唴寤虹殑銆?
- `lz4/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `lzo/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `math/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `pldmfw/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `raid/` 鈥?SPDX-License-Identifier: GPL-2.0
- `raid6/` 鈥?SPDX-License-Identifier: GPL-2.0# Enable <altivec.h>
- `reed_solomon/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # This is a modified version of reed solomon lib,
- `test_fortify/` 鈥?SPDX-License-Identifier: GPL-2.0
- `tests/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬tests of kernel library functions 鐨?Makefile.
- `vdso/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `xz/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `zlib_deflate/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # This is a modified version of zlib, which does all memory
- `zlib_dfltcc/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # This is a modified version of zlib, which does all memory
- `zlib_inflate/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # This is a modified version of zlib, which does all memory
- `zstd/` 鈥?SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause # ################################################################ # Copyright (c) Meta Platforms, Inc. and affiliates.

## mm/

鍐呭瓨绠＄悊锛堥〉鍒嗛厤鍣ㄣ€乻lab銆乿malloc銆乭ugetlb銆乻wap銆乵map 绛夛級銆?

- `damon/` 鈥?SPDX-License-Identifier: GPL-2.0
- `kasan/` 鈥?SPDX-License-Identifier: GPL-2.0# Disable ftrace to avoid recursion. # Function splitter causes unnecessary splits in __asan_load1/__asan_store1
- `kfence/` 鈥?SPDX-License-Identifier: GPL-2.0
- `kmsan/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬KernelMemorySanitizer (KMSAN) 鐨?Makefile.
- `tests/`

## net/

缃戠粶鍗忚鏍堬紙ipv4銆乮pv6銆乶etfilter銆丅PF銆佹牳蹇冦€佷互澶綉銆佹棤绾跨瓑锛夈€?

- `6lowpan/` 鈥?SPDX-License-Identifier: GPL-2.0#rfc6282 nhcs #rfc7400 ghcs
- `802/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux 802 鐨?Makefile.x protocol layers.
- `8021q/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux VLAN 灞?鐨?Makefile.
- `9p/` 鈥?SPDX-License-Identifier: GPL-2.0
- `appletalk/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux AppleTalk 灞?鐨?Makefile.
- `atm/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ATM 鍗忚鏃?鐨?Makefile.
- `batman-adv/` 鈥?SPDX-License-Identifier: GPL-2.0 # Copyright (C) B.A.T.M.A.N. contributors: #
- `bluetooth/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux Bluetooth subsystem 鐨?Makefile.
- `bpf/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `bridge/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬IEEE 802 鐨?Makefile.1d ethernet bridging layer.
- `can/` 鈥?SPDX-License-Identifier: GPL-2.0# #  鐢ㄤ簬Linux Controller Area Network core 鐨?Makefile.
- `ceph/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬CEPH filesystem 鐨?Makefile.
- `core/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux networking core 鐨?Makefile.
- `dcb/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `devlink/` 鈥?SPDX-License-Identifier: GPL-2.0
- `dns_resolver/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux DNS 瑙ｆ瀽鍣?鐨?Makefile.
- `dsa/` 鈥?SPDX-License-Identifier: GPL-2.0# 鍙 DSA 鍐呭缓鎴栦綔涓烘ā鍧楁瀯寤猴紝杩欎簺妗╁氨鏄唴寤虹殑 # 鏍稿績
- `ethernet/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux 浠ュお缃戝眰 鐨?Makefile.
- `ethtool/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `handshake/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬閫氱敤 HANDSHAKE 鏈嶅姟 鐨?Makefile
- `hsr/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬HSR 鐨?Makefile
- `ieee802154/` 鈥?SPDX-License-Identifier: GPL-2.0
- `ife/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬IFE 灏佽鍗忚 鐨?Makefile
- `ipv4/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux TCP/IP (INET) layer 鐨?Makefile.
- `ipv6/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux TCP/IP (INET6) layer 鐨?Makefile.
- `iucv/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬IUCV 鐨?Makefile
- `kcm/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `key/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬key AF 鐨?Makefile.
- `l2tp/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬L2TP 鐨?Makefile.
- `l3mdev/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬L3 璁惧 API 鐨?Makefile
- `lapb/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Linux LAPB 灞?鐨?Makefile.
- `llc/` 鈥?# 鐢ㄤ簬 Linux 802.2 LLC锛堝姛鑳藉畬鏁达級灞傜殑 Makefile銆?
- `mac80211/` 鈥?SPDX-License-Identifier: GPL-2.0# mac80211 瀵硅薄
- `mac802154/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `mctp/` 鈥?SPDX-License-Identifier: GPL-2.0# 娴嬭瘯
- `mpls/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬MPLS 鐨?Makefile.
- `mptcp/` 鈥?SPDX-License-Identifier: GPL-2.0
- `ncsi/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬NCSI API 鐨?Makefile
- `netfilter/` 鈥?SPDX-License-Identifier: GPL-2.0
- `netlabel/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬NetLabel 瀛愮郴缁?鐨?Makefile.
- `netlink/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬netlink 椹卞姩 鐨?Makefile.
- `nfc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux NFC subsystem 鐨?Makefile.
- `nsh/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `openvswitch/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Open vSwitch 鐨?Makefile.
- `packet/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬packet AF 鐨?Makefile.
- `phonet/` 鈥?SPDX-License-Identifier: GPL-2.0
- `psample/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬psample netlink 閫氶亾 鐨?Makefile
- `psp/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `qrtr/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `rds/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬 GCOV 瑕嗙洊鐜囧垎鏋?
- `rfkill/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬RF 寮€鍏冲瓙绯荤粺 鐨?Makefile.
- `rxrpc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux kernel RxRPC 鐨?Makefile
- `sched/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux 娴侀噺鎺у埗鍗曞厓 鐨?Makefile.
- `sctp/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬SCTP support code 鐨?Makefile.
- `shaper/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬缃戠粶鏁村舰鍩虹璁炬柦 鐨?Makefile.
- `smc/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `strparser/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `sunrpc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux kernel SUN RPC 鐨?Makefile
- `switchdev/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬Switch 璁惧 API 鐨?Makefile
- `tipc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux TIPC 灞?鐨?Makefile
- `tls/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬TLS 瀛愮郴缁?鐨?Makefile.
- `unix/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux unix 鍩熷鎺ュ瓧灞?鐨?Makefile.
- `vmw_vsock/` 鈥?SPDX-License-Identifier: GPL-2.0
- `wireless/` 鈥?SPDX-License-Identifier: GPL-2.0
- `x25/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬Linux X 鐨?Makefile.25 Packet layer.
- `xdp/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `xfrm/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬XFRM subsystem 鐨?Makefile.

## rust/

Rust 鍐呮牳鏀寔锛坆indings銆佹牳蹇冦€乭elpers銆乿endored crates锛夈€?

- `bindings/`
- `helpers/`
- `kernel/` 鈥?鏍稿績鍐呮牳瀛愮郴缁燂紙璋冨害鍣ㄣ€乸rintk銆乮rq銆佹椂闂淬€佸姞閿併€丷CU銆丅PF 绛夛級銆?
- `macros/`
- `pin-init/` 鈥?[![Crates.io](https://img.shields.io/crates/v/pin-init.svg)](https://crates.io/crates/pin-init) [![Documentation](https://docs.rs/pin-init/badge.svg)](https://docs.rs/pin-init/) [![Dependency status](https://deps.rs/repo/github/Rust-for-Linux/pin-init/status.svg)](https://deps.rs/repo/github/Rust-for-Linux/pin-init) ![License](https://img.shields.io/crates/l/pin-init) [![Toolchain](https://img.shields.io/badge/toolchain-nightly-red)](#nightly-only) ![GitHub Workflow Status](https://img.shield...
- `proc-macro2/` 鈥?# `proc-macro2`  杩欎簺婧愭枃浠舵潵鑷?Rust `proc-macro2` crate锛岀増鏈?1.0.101锛堝彂甯冧簬 2025-08-16锛夛紝鎵樼浜?<https://github.com/dtolnay/proc-macro2> 浠撳簱锛岄噰鐢?鈥淎pache-2.0 OR MIT鈥?璁稿彲锛屼粎鍋氫簡淇敼浠ユ坊鍔?SPDX 璁稿彲璇佹爣璇嗙骞剁Щ闄?`unicode-ident` 渚濊禆銆? 鐗堟潈璇︽儏璇峰弬闃咃細
- `quote/` 鈥?# `quote`  杩欎簺婧愭枃浠舵潵鑷?Rust `quote` crate锛岀増鏈?1.0.40锛堝彂甯冧簬 2025-03-12锛夛紝鎵樼浜?<https://github.com/dtolnay/quote> 浠撳簱锛岄噰鐢?鈥淎pache-2.0 OR MIT鈥?璁稿彲锛屼粎鍋氫簡淇敼浠ユ坊鍔?SPDX 璁稿彲璇佹爣璇嗙銆? 鐗堟潈璇︽儏璇峰弬闃咃細      https://github.com/dtolnay/quote/blob/1.0.40/README.md#license
- `syn/` 鈥?# `syn`  杩欎簺婧愭枃浠舵潵鑷?Rust `syn` crate锛岀増鏈?2.0.106锛堝彂甯冧簬 2025-08-16锛夛紝鎵樼浜?<https://github.com/dtolnay/syn> 浠撳簱锛岄噰鐢?鈥淎pache-2.0 OR MIT鈥?璁稿彲锛屼粎鍋氫簡淇敼浠ユ坊鍔?SPDX 璁稿彲璇佹爣璇嗙骞剁Щ闄?`unicode-ident` 渚濊禆銆? 鐗堟潈璇︽儏璇峰弬闃咃細
- `uapi/`

## samples/

绀轰緥涓庢暀绋嬩唬鐮侊紙BPF銆乿fio-mdev銆乸ktgen锛夈€?

- `acrn/` 鈥?SPDX-License-Identifier: GPL-2.0
- `auxdisplay/` 鈥?SPDX-License-Identifier: GPL-2.0
- `binderfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `bpf/` 鈥?eBPF 绀轰緥绋嬪簭 ====================  鏈洰褰曞寘鍚娇鐢?eBPF 鐨勬祴璇曟々銆侀獙璇佸櫒娴嬭瘯濂椾欢鍜岀ず渚嬨€傜ず渚嬩娇鐢ㄤ簡鏉ヨ嚜 tools/lib/bpf 鐨?libbpf銆傝娉ㄦ剰锛岀壒瀹氫簬 XDP 鐨勭ず渚嬪凡浠庢湰鐩綍绉婚櫎锛屽苟绉昏嚦 xdp-tools 浠撳簱锛?https://github.com/xdp-project/xdp-tools 鏈夊叧濡備綍灏嗘棫绀轰緥涓殑鐗瑰畾鍛戒护璋冪敤杞崲涓烘柊宸ュ叿鐨勮鏄庯紝璇峰弬闃呬粠鏈洰褰曠Щ闄ゆ瘡涓伐鍏风殑鎻愪氦淇℃伅
- `cgroup/` 鈥?SPDX-License-Identifier: GPL-2.0
- `check-exec/` 鈥?SPDX-License-Identifier: BSD-3-Clause
- `configfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `connector/` 鈥?SPDX-License-Identifier: GPL-2.0
- `coresight/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `damon/` 鈥?SPDX-License-Identifier: GPL-2.0
- `fanotify/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `fprobe/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `ftrace/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `hid/` 鈥?SPDX-License-Identifier: GPL-2.0# 瑕佹瀯寤虹殑绋嬪簭鍒楄〃 # Libbpf 渚濊禆
- `hidraw/` 鈥?SPDX-License-Identifier: GPL-2.0
- `hung_task/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `hw_breakpoint/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `kdb/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `kfifo/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `kmemleak/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `kobject/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `kprobes/` 鈥?SPDX-License-Identifier: GPL-2.0-only# builds the kprobes example kernel modules; # then to use one (as root):  insmod <module_name.ko>
- `landlock/` 鈥?SPDX-License-Identifier: BSD-3-Clause
- `livepatch/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `mei/` 鈥?SPDX-License-Identifier: GPL-2.0 # Copyright (c) 2012-2019, Intel Corporation. All rights reserved.
- `nitro_enclaves/` 鈥?SPDX-License-Identifier: GPL-2.0 # # Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
- `pfsm/` 鈥?SPDX-License-Identifier: GPL-2.0
- `pidfd/` 鈥?SPDX-License-Identifier: GPL-2.0
- `pktgen/` 鈥?pktgen锛堟暟鎹寘鐢熸垚鍣級鐨勭ず渚嬩笌鍩哄噯鑴氭湰 ========================================================== 鏈洰褰曞寘鍚竴浜?pktgen 绀轰緥鍜屽熀鍑嗚剼鏈紝鍙交鏉惧鍒跺苟閽堝浣犵殑鐢ㄤ緥杩涜璋冩暣銆? 閫氱敤鏂囨。浣嶄簬鍐呮牳涓細Documentation/networking/pktgen.rst  杈呭姪鍖呭惈鏂囦欢 ==================== 鏈洰褰曞寘鍚袱涓彲鈥滃寘鍚€濈殑杈呭姪 shell 鏂囦欢
- `qmi/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `rpmsg/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `rust/` 鈥?SPDX-License-Identifier: GPL-2.0
- `seccomp/` 鈥?SPDX-License-Identifier: GPL-2.0
- `timers/` 鈥?SPDX-License-Identifier: GPL-2.0
- `trace_events/` 鈥?SPDX-License-Identifier: GPL-2.0-only# builds the trace events example kernel modules; # then to use one (as root):  insmod <module_name.ko>
- `trace_printk/` 鈥?SPDX-License-Identifier: GPL-2.0-only# builds a module that calls various trace_printk routines # then to use one (as root):  insmod <module_name.ko>
- `tsm-mr/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `uhid/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `user_events/` 鈥?SPDX-License-Identifier: GPL-2.0
- `v4l/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `vfio-mdev/` 鈥?浣跨敤 mtty vfio-mdev 绀轰緥浠ｇ爜 ====================================  mtty 鏄竴涓ず渚?vfio-mdev 椹卞姩锛屾紨绀轰簡濡備綍浣跨敤涓粙璁惧锛坢ediated device锛夋鏋躲€傝绀轰緥椹卞姩鍒涘缓涓€涓?mdev 璁惧锛屾ā鎷熼€氳繃 PCI 鍗℃彁渚涚殑涓插彛銆? 1. 鏋勫缓骞跺姞杞?mtty.ko 妯″潡銆?
- `vfs/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `watch_queue/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `watchdog/` 鈥?SPDX-License-Identifier: GPL-2.0
- `workqueue/`

## scripts/

鏋勫缓鑴氭湰銆乧heckpatch銆乧occinelle 琛ヤ竵銆乲config銆乵odpost 绛夈€?

- `atomic/`
- `bash-completion/`
- `basic/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # fixdep: used to generate dependency information during build process
- `clang-tools/`
- `coccinelle/`
- `crypto/` 鈥?鍔犲瘑 API 涓庣畻娉曞疄鐜般€?
- `dtc/` 鈥?SPDX-License-Identifier: GPL-2.0# scripts/dtc makefile # *** Also keep .gitignore in sync when changing ***
- `dummy-tools/`
- `gcc-plugins/` 鈥?SPDX-License-Identifier: GPL-2.0# Build rules for plugins #
- `gdb/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `gendwarfksyms/` 鈥?SPDX-License-Identifier: GPL-2.0
- `genksyms/` 鈥?SPDX-License-Identifier: GPL-2.0# -I needed for generated C source to include headers in source tree # dependencies on generated files need to be listed explicitly
- `include/` 鈥?鍐呮牳鍏叡澶存枃浠讹紙linux/銆乤sm-generic/銆乽api/锛夈€?
- `ipe/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `kconfig/` 鈥?SPDX-License-Identifier: GPL-2.0# =========================================================================== # Kernel configuration targets
- `ksymoops/` 鈥?ksymoops 宸蹭粠鍐呮牳涓Щ闄ゃ€傚畠涓€鐩存槸涓€涓嫭绔嬬殑宸ュ叿锛屼笉閾炬帴鍒颁换浣曠壒瀹氱殑鍐呮牳鐗堟湰銆傛渶鏂扮増鏈彲鍦?https://www.kernel.org/pub/linux/utils/kernel/ksymoops 鎵惧埌锛屽悓鏃惰繕鏈夊鍏朵粬宸ュ叿鐨勮ˉ涓侊紝浠ヤ究鎻愪緵鏇村噯纭殑 Oops 璋冭瘯淇℃伅銆? Keith Owens <kaos@ocs.com.au> Sat Jun 19 10:30:34 EST 1999
- `livepatch/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬寮€鍙戣€呭伐鍏风殑鐙珛 Makefile锛堜笉灞炰簬 kbuild锛夈€?
- `mod/` 鈥?SPDX-License-Identifier: GPL-2.0# dependencies on generated files need to be listed explicitly
- `package/`
- `selinux/` 鈥?鏈夊叧瀹夎铏氭嫙 SELinux 绛栫暐鐨勪俊鎭紝璇峰弬闃?Documentation/admin-guide/LSM/SELinux.rst銆?
- `tracing/`

## security/

瀹夊叏妯″潡锛坰elinux銆乤pparmor銆乴andlock銆乻mack 绛夛級銆?

- `apparmor/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬AppArmor Linux Security Module 鐨?Makefile#
- `bpf/` 鈥?SPDX-License-Identifier: GPL-2.0 # # Copyright (C) 2020 Google LLC.
- `integrity/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬缂撳瓨 inode 瀹屾暣鎬ф暟鎹紙iint锛?鐨?Makefile
- `ipe/` 鈥?SPDX-License-Identifier: GPL-2.0 # # Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
- `keys/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬瀵嗛挜绠＄悊 鐨?Makefile
- `landlock/`
- `loadpin/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `lockdown/`
- `safesetid/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬safesetid LSM 鐨?Makefile.
- `selinux/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬灏?SELinux 妯″潡浣滀负鍐呮牳鏍戠殑涓€閮ㄥ垎鏋勫缓 鐨?Makefile.
- `smack/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬SMACK LSM 鐨?Makefile
- `tomoyo/` 鈥?SPDX-License-Identifier: GPL-2.0
- `yama/` 鈥?SPDX-License-Identifier: GPL-2.0-only

## sound/

ALSA 澹伴煶瀛愮郴缁熶笌闊抽椹卞姩銆?

- `ac97/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # make for AC97 bus drivers
- `aoa/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `arm/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `atmel/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `core/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `drivers/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `firewire/` 鈥?SPDX-License-Identifier: GPL-2.0# To find a header included by define_trace.h.
- `hda/` 鈥?SPDX-License-Identifier: GPL-2.0# this must be the last entry after codec drivers; # otherwise the codec drivers won't be hooked before the PCI probe
- `i2c/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `isa/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `mips/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬ALSA 鐨?Makefile
- `oss/`
- `parisc/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬ALSA 鐨?Makefile
- `pci/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `pcmcia/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬ALSA 鐨?Makefile
- `ppc/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬ALSA 鐨?Makefile
- `sh/` 鈥?SPDX-License-Identifier: GPL-2.0-only# # 鐢ㄤ簬ALSA 鐨?Makefile
- `soc/` 鈥?SPDX-License-Identifier: GPL-2.0# snd-soc-test-y := soc-topology-test.o # snd-soc-test-y := soc-utils-test.o
- `sparc/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `spi/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬SPI drivers 鐨?Makefile
- `synth/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `usb/` 鈥?SPDX-License-Identifier: GPL-2.0# # 鐢ㄤ簬ALSA 鐨?Makefile
- `virtio/` 鈥?SPDX-License-Identifier: GPL-2.0+
- `x86/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `xen/` 鈥?SPDX-License-Identifier: GPL-2.0 OR MIT

## tools/

鐢ㄦ埛绌洪棿宸ュ叿锛坧erf銆乥pftool銆乻elftests銆乲unit銆乧pupower 绛夛級銆?

- `accounting/` 鈥?SPDX-License-Identifier: GPL-2.0
- `arch/` 鈥?鐗瑰畾浜庝綋绯荤粨鏋勭殑浠ｇ爜锛坅rm64銆亁86銆乺iscv銆乵68k銆乸owerpc 绛夛級浠ュ強寮曞鍩虹璁炬柦銆?
- `bootconfig/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬bootconfig command 鐨?Makefile
- `bpf/` 鈥?SPDX-License-Identifier: GPL-2.0# This will work when bpf is built in tools env. where srctree # isn't set and when invoked from selftests build, where srctree
- `build/` 鈥?SPDX-License-Identifier: GPL-2.0
- `certs/`
- `cgroup/`
- `counter/` 鈥?SPDX-License-Identifier: GPL-2.0# Do not use make's built-in rules # (this improves performance and avoids hard-to-debug behaviour);
- `crypto/` 鈥?鍔犲瘑 API 涓庣畻娉曞疄鐜般€?
- `debugging/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬debugging tools 鐨?Makefile
- `dma/` 鈥?SPDX-License-Identifier: GPL-2.0# This will work when dma is built in tools env. where srctree # isn't set and when invoked from selftests build, where srctree
- `docs/`
- `firewire/` 鈥?SPDX-License-Identifier: GPL-2.0
- `firmware/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬firmware tools 鐨?Makefile
- `gpio/` 鈥?SPDX-License-Identifier: GPL-2.0# This will work when gpio is built in tools env. where srctree # isn't set and when invoked from selftests build, where srctree
- `hv/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬Hyper-V tools 鐨?Makefile# Do not use make's built-in rules
- `iio/` 鈥?SPDX-License-Identifier: GPL-2.0# Do not use make's built-in rules # (this improves performance and avoids hard-to-debug behaviour);
- `include/` 鈥?鍐呮牳鍏叡澶存枃浠讹紙linux/銆乤sm-generic/銆乽api/锛夈€?
- `kvm/`
- `laptop/`
- `leds/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬LEDs tools 鐨?Makefile
- `lib/` 鈥?鍐呮牳閫氱敤搴擄紙浣嶅浘銆乺btree銆乺adix-tree銆乧rc銆乲unit 绛夛級銆?
- `memory-model/` 鈥?===================================== 		LINUX 鍐呮牳鍐呭瓨涓€鑷存€фā鍨?		=====================================  ============ 绠€浠?============  鏈洰褰曞寘鍚?Linux 鍐呮牳鐨勫唴瀛樹竴鑷存€фā鍨嬶紙绠€绉板唴瀛樻ā鍨嬶級锛屼娇鐢?鈥渃at鈥?璇█缂栧啓骞跺彲鎵ц
- `mm/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬vm tools 鐨?Makefile#
- `net/` 鈥?缃戠粶鍗忚鏍堬紙ipv4銆乮pv6銆乶etfilter銆丅PF銆佹牳蹇冦€佷互澶綉銆佹棤绾跨瓑锛夈€?
- `objtool/` 鈥?SPDX-License-Identifier: GPL-2.0
- `pcmcia/` 鈥?SPDX-License-Identifier: GPL-2.0
- `perf/` 鈥?SPDX-License-Identifier: GPL-2.0# # This is a simple wrapper Makefile that calls the main Makefile.perf
- `power/`
- `rcu/`
- `sched/`
- `sched_ext/` 鈥?SCHED_EXT 绀轰緥璋冨害鍣?============================  # 绠€浠? 鏈洰褰曞寘鍚嫢骞?sched_ext 绀轰緥璋冨害鍣ㄣ€傝繖浜涜皟搴﹀櫒鏃ㄥ湪鎻愪緵浣跨敤 sched_ext 鍙瀯寤虹殑涓嶅悓绫诲瀷璋冨害鍣ㄧ殑绀轰緥锛屽苟婕旂ず sched_ext 鐨勫悇椤圭壒鎬у浣曡浣跨敤銆?
- `scripts/` 鈥?鏋勫缓鑴氭湰銆乧heckpatch銆乧occinelle 琛ヤ竵銆乲config銆乵odpost 绛夈€?
- `sound/` 鈥?ALSA 澹伴煶瀛愮郴缁熶笌闊抽椹卞姩銆?
- `spi/` 鈥?SPDX-License-Identifier: GPL-2.0-only# Do not use make's built-in rules # (this improves performance and avoids hard-to-debug behaviour);
- `testing/`
- `thermal/`
- `time/`
- `tracing/` 鈥?SPDX-License-Identifier: GPL-2.0
- `unittests/`
- `usb/` 鈥?SPDX-License-Identifier: GPL-2.0# 鐢ㄤ簬USB tools 鐨?Makefile# Do not use make's built-in rules
- `verification/`
- `virtio/` 鈥?SPDX-License-Identifier: GPL-2.0
- `wmi/` 鈥?SPDX-License-Identifier: GPL-2.0-only
- `workqueue/`
- `writeback/`

## virt/

铏氭嫙鍖栵紙KVM銆乁ML銆乆en 绛夛級銆?

- `kvm/`
- `lib/` 鈥?SPDX-License-Identifier: GPL-2.0-only

---

# Kconfig 鎽樿

## 鍏朵粬

| 閰嶇疆 | 绫诲瀷 | 鎻忚堪 |
|--------|------|-------------|
| 842_COMPRESS | tristate | 鍚敤鍐呮牳涓?s390x 瀵?zlib 鐨勭‖浠舵敮鎸併€? |
| ADVISE_SYSCALLS | bool | 璇ラ€夐」鍚敤 madvise 涓?fadvise 绯荤粺璋冪敤锛屽簲鐢ㄧ▼搴忓€熸鍚戝唴鏍稿缓璁叾鏈潵鐨勫唴瀛樻垨鏂囦欢浣跨敤鏂瑰紡锛屼粠鑰屾彁鍗囨€ц兘銆傝嫢鏋勫缓鈥︹€? |
| AIO | bool | 璇ラ€夐」鍚敤 POSIX 寮傛 I/O锛岄儴鍒嗛珮鎬ц兘澶氱嚎绋嬪簲鐢ㄥ彲鑳戒細鐢ㄥ埌銆傜鐢ㄦ閫夐」鍙妭鐪佺害 7k銆? |
| ANON_VMA_NAME | bool | 鍏佽涓哄尶鍚嶈櫄鎷熷唴瀛樺尯鍩熷懡鍚嶃€傝鍔熻兘鍙负铏氭嫙鍐呭瓨鍖哄煙鎸囧畾鍚嶇О锛屾墍鎸囧畾鍚嶇О闅忓悗鍙粠 /proc/pid/maps 涓?/proc/pid/smaps 涓鍙栵紝鏈夊姪浜庤瘑鈥︹€? |
| ARCH_FORCE_MAX_ORDER | int | 椤靛潡闃讹紙page block order锛夋寚鐗╃悊杩炵画銆佸彲鍏宠仈杩佺Щ绫诲瀷鐨勯〉闈㈡暟閲忕殑 2 鐨勫箓銆傞〉鍧楅樁鐨勬渶澶у昂瀵歌嚦灏戜负鈥︹€? |
| ARCH_HAS_BINFMT_FLAT | bool | 鏀寔 uClinux FLAT 鏍煎紡浜岃繘鍒舵枃浠躲€? |
| ARCH_HAS_CC_CAN_LINK | bool | 閫夋嫨姝ら」鍙皢 thread_info 浠庢爤涓婄Щ鍏?task_struct銆備负浣挎鍔熻兘鐢熸晥锛屼綋绯荤粨鏋勯渶绉婚櫎闄?flags 澶栫殑鎵€鏈?thread_info 瀛楁骞朵慨澶嶇浉鍏宠繍琛屾椂缂洪櫡銆傚叾涓竴涓粏寰敼鍔ㄢ€︹€? |
| ARCH_HAS_CPU_CACHE_ALIASING | bool | 涓烘敮鎸?HARDENED_USERCOPY 杩涜鏍堝彉閲忕敓鍛藉懆鏈熸鏌ワ紝闇€瑕佷竴绉嶄笌浣撶郴缁撴瀯鏃犲叧鐨勬柟寮忔潵鑾峰彇鏍堟寚閽堛€備竴鏃︽煇浣撶郴缁撴瀯瀹氫箟浜?unsigned long 鍏ㄥ眬鍙橀噺 r鈥︹€? |
| ARCH_HAS_DEBUG_VIRTUAL | bool | 鍦ㄨ櫄鎷熷湴鍧€鍒伴〉鐨勮浆鎹唬鐮佷腑鍚敤涓€浜涗唬浠疯緝楂樼殑鍋ュ叏鎬ф鏌ャ€傚彲鎹曡幏 virt_to_page() 绛夊嚱鏁扮殑璇敤銆傝嫢涓嶇‘瀹氾紝閫?N銆? |
| ARCH_HAS_DEBUG_VM_PGTABLE | bool | 褰撴煇浣撶郴缁撴瀯鑳芥垚鍔熸瀯寤哄苟杩愯 DEBUG_VM_PGTABLE 鏃讹紝搴旈€夋嫨姝ら」銆? |
| ARCH_HAS_DEVMEM_IS_ALLOWED | bool | 鑻ョ鐢ㄦ閫夐」锛屽垯鍏佽鐢ㄦ埛绌洪棿锛坮oot锛夎闂叏閮ㄥ唴瀛橈紝鍖呮嫭鍐呮牳涓庣敤鎴风┖闂村唴瀛樸€傛剰澶栬闂樉鐒跺悗鏋滀弗閲嶏紝浣嗙壒瀹氳闂彲鑳解€︹€? |
| ARCH_HAS_ELF_CORE_EFLAGS | bool | 鑻ヤ綋绯荤粨鏋勫埄鐢?ELF 澶翠腑鐨?e_flags 瀛楁鏉ュ瓨鏀惧簲鍦ㄦ牳蹇冭浆鍌ㄤ腑淇濈暀鐨?ABI 鎴栧叾浠栦綋绯荤粨鏋勭浉鍏充俊鎭紝璇烽€夋嫨姝ら」銆? |
| ARCH_HAS_KCOV | bool | 褰撴煇浣撶郴缁撴瀯鑳芥垚鍔熷湪 CONFIG_KCOV 涓嬫瀯寤哄苟杩愯鏃讹紝搴旈€夋嫨姝ら」銆傝繖閫氬父闇€瀵规煇浜涙棭鏈熷紩瀵间唬鐮佺鐢ㄦ彃妗┿€? |
| ARCH_HAS_MEMBARRIER_CALLBACKS | bool | 鍩轰簬浣撶郴缁撴瀯鎺у埗 MSEAL_SYSTEM_MAPPINGS 鐨勮闂€傚唴瀛樺瘑灏佺壒鎬ч渶瑕?64 浣嶅唴鏍搞€傛棤闇€ CPU 鎻愪緵鐗瑰畾纭欢鐗规€с€傝鍚敤姝ょ壒鎬р€︹€? |
| ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE | bool |  |
| ARCH_HAS_PTE_SPECIAL | bool | 鍚敤 memfd_secret() 绯荤粺璋冪敤锛屽彲鍒涘缓浠呭湪鎵€灞炶繘绋嬩笂涓嬫枃涓彲瑙併€佷笖涓嶆槧灏勫埌鍏朵粬杩涚▼鍙婂叾浠栧唴鏍搁〉琛ㄧ殑鍐呭瓨鍖哄煙銆? |
| ARCH_HAS_STRNCPY_FROM_USER | bool | 鍦ㄦ煇浜涗笉瀛樺湪鐙珛 I/O 绌洪棿鐨勫钩鍙颁笂锛岄儴鍒?I/O 涓绘満鏃犳硶浠?MMIO 妯″紡璁块棶銆傚€熷姪閫昏緫 PIO 鏈哄埗锛屼富鏈烘湰鍦?I/O 璧勬簮鍙鏄犲皠鍒扮郴缁熲€︹€? |
| ARCH_HAS_USER_SHADOW_STACK | bool | 璇ヤ綋绯荤粨鏋勬彁渚涘鐢ㄦ埛绌洪棿褰卞瓙璋冪敤鏍堬紙shadow call stack锛夌殑纭欢鏀寔锛堜緥濡?x86 CET銆乤rm64 GCS 鎴?RISC-V Zicfiss锛夈€? |
| ARCH_HAS_ZONE_DMA_SET | bool | 璁惧鍐呭瓨鐑彃鎷旀敮鎸佸厑璁稿湪 memmap 涓缓绔?pmem 鎴栧叾浠栫敱璁惧椹卞姩鍙戠幇鐨勫唴瀛樺尯鍩熴€傝繖浣垮緱鍙鍘熸湰鈥滆澶囩墿鐞嗏€濆湴鍧€杩涜 pfn_to_page() 鏌ユ壘鈥︹€? |
| ARCH_NO_SG_CHAIN | def_bool | 鏍堜粨搴擄紙stack depot锛夛細閬垮厤閲嶅鐨勬爤璺熻釜瀛樺偍  |
| ARCH_NO_SWAP | bool | 姝ら€夐」璁╀綘閫夋嫨鍐呮牳鏄惁鏀寔鎵€璋撶殑浜ゆ崲璁惧锛坰wap device锛夋垨浜ゆ崲鏂囦欢锛坰wap file锛夛紝鐢ㄤ簬鎻愪緵姣斿疄闄呯墿鐞?RAM 鏇村鐨勮櫄鎷熷唴瀛樷€︹€? |
| ARCH_SUPPORTS_HUGETLBFS | def_bool | hugetlbfs 鏄熀浜?ramfs 鐨?HugeTLB 椤垫枃浠剁郴缁熷悗绔€傛敮鎸佺殑浣撶郴缁撴瀯璇峰湪姝ら€?Y锛屽苟闃呰 <file:Documentation/admin-guide/mm/hugetlbpage.rst> 浜嗚В缁嗚妭銆傝嫢涓嶇‘瀹氣€︹€? |
| ARCH_SUPPORTS_KMAP_LOCAL_FORCE_MAP | bool | 姝ら€夐」鍦ㄩ潪楂樼鍐呭瓨椤靛強闈為珮绔唴瀛樼郴缁熶笂锛屽己鍒堕€氳繃 kmap_local 鏈哄埗寤虹珛涓存椂鏄犲皠銆傜敓浜х郴缁熻绂佺敤锛? |
| ARCH_SUPPORTS_MEMORY_FAILURE | bool | 鍦ㄥ叿澶?MCA 鎭㈠鑳藉姏鐨勭郴缁熶笂鍚敤浠庨儴鍒嗗唴瀛樻晠闅滀腑鎭㈠鐨勪唬鐮併€傚嵆浣块儴鍒嗗唴瀛樺瓨鍦ㄦ湭绾犳閿欒锛岀郴缁熶粛鍙户缁繍琛屻€傝繖闇€瑕佺壒娈婄殑纭€︹€? |
| ARCH_SUPPORTS_NUMA_BALANCING | bool | 姝ら€夐」娣诲姞瀵硅嚜鍔ㄦ劅鐭?NUMA 鐨勫唴瀛?浠诲姟鏀剧疆鐨勬敮鎸併€傝鏈哄埗杈冧负鍘熷锛屽熀浜庡湪鍐呭瓨寮曠敤鍒颁换鍔℃墍杩愯鐨勮妭鐐规椂杩涜杩佺Щ鈥︹€? |
| ARCH_USE_MEMTEST | bool | 褰撴煇浣撶郴缁撴瀯鍦ㄥ紩瀵艰繃绋嬩腑浣跨敤 early_memtest() 鏃讹紝搴旈€夋嫨姝ら」銆? |
| ARCH_WANT_FRAME_POINTERS | bool | 鑻ラ€?Y锛岀敓鎴愮殑鍐呮牳闀滃儚浼氱◢澶т笖绋嶆參锛屼絾鍦ㄥ唴鏍稿嚭閿欐椂鍙彁渚涢潪甯告湁鐢ㄧ殑璋冭瘯淇℃伅锛堢簿纭殑 oops銆佹爤璺熻釜銆佽鍛婏級銆? |
| ARCH_WANT_GENERAL_HUGETLB | bool | 鍚敤姝ら€夐」鍙檷浣庡ぇ闆?folio锛坔uge zero folio锛夌殑杩愯鏃跺紩鐢ㄨ鏁板紑閿€锛屽苟鎵╁睍鍐呮牳涓彲浣跨敤澶ч浂 folio 鐨勪綅缃€備緥濡傚潡 I/O 鍙粠涓彈鐩娾€︹€? |
| ASSOCIATIVE_ARRAY | bool | 閫氱敤鍏宠仈鏁扮粍銆傚彲鍦ㄨ淇敼鐨勫悓鏃惰繘琛屾煡鎵句笌閬嶅巻銆傚叾鏌ユ壘涓庝慨鏀逛篃鐩稿綋杩呴€熴€傜畻娉曚负闈為€掑綊寮忥紝鏍戠粨鏋勮緝鈥︹€? |
| ASYNC_RAID6_TEST | tristate | 杩欐槸涓€娆℃€ц嚜妫€娴嬭瘯锛屼細閬嶅巻 N 鐩橀樀鍒楁墍鏈夊彲鑳界殑鍙岀洏鏁呴殰鍦烘櫙杩涜鎭㈠銆傛仮澶嶄娇鐢ㄥ紓姝?raid6 鎭㈠渚嬬▼鈥︹€? |
| AS_HAS_NON_CONST_ULEB128 | def_bool | 閫夋嫨鈥淣one鈥濅互澶栫殑鍊间細瀵艰嚧鍐呮牳闀滃儚鍖呭惈璋冭瘯淇℃伅锛屼粠鑰屽澶ч暅鍍忎綋绉€傚畠浼氬悜鍐呮牳涓庢ā鍧楁坊鍔犺皟璇曠鍙凤紙gcc -g锛夛紝骞垛€︹€? |
| ATOMIC64_SELFTEST | tristate | 鍚敤姝ら€夐」鍙湪寮曞鏃舵垨妯″潡鍔犺浇鏃舵祴璇?atomic64_t 鍑芥暟銆傝嫢涓嶇‘瀹氾紝閫?N銆? |
| AUDIT | bool | 鍚敤瀹¤鍩虹璁炬柦锛屽彲涓庡叾浠栧唴鏍稿瓙绯荤粺锛堝 SELinux锛屽叾璁板綍 avc 娑堟伅杈撳嚭闇€瑕佸畠锛夐厤鍚堜娇鐢ㄣ€傜郴缁熻皟鐢ㄥ璁″寘鍚簬浣撶郴缁撴瀯鈥︹€? |
| BACKTRACE_SELF_TEST | tristate | 璇ラ€夐」鎻愪緵涓€涓唴鏍告ā鍧楋紝鐢ㄤ簬娴嬭瘯鍐呮牳鏍堝洖婧唬鐮併€傛閫夐」瀵瑰彂琛岀増鎴栭€氱敤鍐呮牳鏃犵敤锛屼粎瀵瑰唴鏍稿紑鍙戣€呪€︹€? |
| BASE64_KUNIT | tristate | 鏋勫缓 base64 鍗曞厓娴嬭瘯銆傛祴璇曡鐩栧唴鏍镐腑 Base64 鍑芥暟鐨勭紪鐮佷笌瑙ｇ爜閫昏緫銆傞櫎姝ｇ‘鎬ф鏌ュ锛岃繕瀵逛袱绉嶇紪鐮侀兘杩涜浜嗙畝鍗曠殑鎬ц兘鍩哄噯娴嬭瘯鈥︹€? |
| BASE_SMALL | bool | 鍚敤姝ら€夐」鍙缉鍑忓悇绫绘牳蹇冨唴鏍告暟鎹粨鏋勭殑澶у皬銆傝繖鍦ㄥ皬鍨嬫満鍣ㄤ笂鑺傜渷鍐呭瓨锛屼絾鍙兘闄嶄綆鎬ц兘銆? |
| BCH_CONST_M | int | Galois 鍩熼樁鏁扳€渕鈥濈殑甯告暟鍊笺€傝嫢鈥渒鈥濅负瑕佷繚鎶ょ殑浣嶆暟锛屽垯鈥渕鈥濆簲婊¤冻 (k + m*t) <= 2**m - 1銆傞┍鍔ㄥ簲涓鸿绗﹀彿澹版槑榛樿鍊尖€︹€? |
| BCH_CONST_T | int | 绾犻敊鑳藉姏锛堜互姣旂壒涓哄崟浣嶇殑鈥渢鈥濓級鐨勫父鏁板€笺€傝嫢椹卞姩閫夋嫨浜?BCH_CONST_PARAMS 閫夐」锛屽垯搴斾负璇ョ鍙峰０鏄庨粯璁ゅ€笺€? # 濡傞渶鈥︹€? |
| BINARY_PRINTF | def_bool | 鍦ㄥ垵濮嬪寲鏃跺鍏ㄩ儴鍙敤鐨?RAID6 PQ 鍑芥暟杩涜鍩哄噯娴嬭瘯锛屽苟閫夋嫨鏈€蹇殑涓€涓€? |
| BINDGEN_VERSION_TEXT | string | 鍥炴函鍒版瘡涓綋绯荤粨鏋勫悇鑷畾涔?cpu_online_mask 涓?cpu_possible_mask 鐨勬椂浠ｏ紝鍏朵腑涓€浜涘皢鍏跺垵濮嬪寲涓哄叏 1锛屽彟涓€浜涗负鍏?0銆傚綋瀹冧滑琚泦涓寲鈥︹€? |
| BINFMT_ELF | bool | ELF锛圗xecutable and Linkable Format锛屽彲鎵ц涓庡彲閾炬帴鏍煎紡锛夋槸涓€绉嶈法涓嶅悓浣撶郴缁撴瀯涓庢搷浣滅郴缁熶娇鐢ㄧ殑搴撲笌鍙墽琛屾枃浠舵牸寮忋€傚湪姝ら€?Y 灏嗚浣犵殑鍐呮牳鑳藉杩愯 ELF 浜岃繘鍒舵枃浠垛€︹€? |
| BINFMT_ELF_KUNIT_TEST | bool | 鏋勫缓 ELF 鍔犺浇鍣?KUnit 娴嬭瘯锛屽皾璇曞皢浠ュ線鐨勯敊璇慨澶嶆敹闆嗕负鍥炲綊娴嬭瘯闆嗐€傝繖閫氬父浠呯敤浜庤皟璇曘€傛敞鎰忓湪 CONFIG_COMPAT=y 鏃讹紝compat_b鈥︹€? |
| BINFMT_FLAT_ARGVP_ENVP_ON_STACK | bool | 鏀寔鍗佸勾鍓嶇殑 uClinux FLAT 鏍煎紡浜岃繘鍒舵枃浠躲€傞櫎闈炰綘纭畾鎷ユ湁姝ょ被鏂囦欢锛屽惁鍒欏湪姝ら€?N銆? |
| BINFMT_MISC | tristate | 鑻ュ湪姝ら€?Y锛屼究鍙悜鍐呮牳鎻掑叆鐢卞寘瑁呭櫒椹卞姩鐨勪簩杩涘埗鏍煎紡銆傚綋浣犱娇鐢ㄩ渶瑕佽В閲婂櫒鎵嶈兘杩愯鐨勭▼搴忥紙濡?Java銆丳ython鈥︹€︼級鏃朵細鐗瑰埆鏈夌敤銆? |
| BINFMT_SCRIPT | tristate | 鑻ュ笇鏈涙墽琛屼互 #! 寮€澶村苟鍚庤窡瑙ｉ噴鍣ㄨ矾寰勭殑鑴氭湰锛岃鍦ㄦ閫?Y銆備綘鍙互灏嗗叾鏋勫缓涓烘ā鍧楋紱浣嗗湪璇ユā鍧楀姞杞戒箣鍓嶏紝浣犳棤鈥︹€? |
| BINFMT_ZFLAT | bool | 鏀寔 FLAT 鏍煎紡鍘嬬缉浜岃繘鍒舵枃浠? |
| BITFIELD_KUNIT | tristate | 鍚敤姝ら€夐」鍙湪寮曞鏃舵祴璇曚綅鍩熷嚱鏁般€侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞朵互 TAP 鏍煎紡锛坔ttp://testanything.org/锛夊皢缁撴灉杈撳嚭鍒拌皟璇曟棩蹇椼€備粎渚涘唴鏍稿紑鍙戣€呪€︹€? |
| BITOPS_KUNIT | tristate | 璇ラ€夐」鍚敤 bitops 搴撶殑 KUnit 娴嬭瘯锛屾彁渚涗綅鎿嶄綔鍑芥暟銆傛敞鎰忓畠婧愯嚜鍘熷鐨?test_bitops 妯″潡銆傜敤浜庡井鍩哄噯娴嬭瘯涓庣紪璇戔€︹€? |
| BITREVERSE | tristate | 璇ラ€夐」鍦ㄦ煇浜涙敮鎸佹绫绘搷浣滅殑浣撶郴缁撴瀯涓婂惎鐢ㄧ‖浠朵綅鍙嶈浆鎸囦护銆? |
| BITS_TEST | tristate | 鏋勫缓 bits 鍗曞厓娴嬭瘯銆傛祴璇?bits.h 涓畾涔夌殑瀹忕殑閫昏緫銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools 涓殑 KUnit 鏂囨。鈥︹€? |
| BLACKHOLE_DEV_KUNIT_TEST | tristate | 鏋勫缓鈥渂lackhole_dev_kunit鈥濇ā鍧楋紝鐢ㄤ簬楠岃瘉閫氳繃璇ラ粦娲炵綉缁滆澶囩殑鐨勬暟鎹矾寰勩€傝嫢涓嶇‘瀹氾紝閫?N銆? |
| BLK_CGROUP | bool | 閫氱敤鍧?I/O 鎺у埗鍣?cgroup 鎺ュ彛銆傝繖鏄悇绫?I/O 鎺у埗绛栫暐搴斾娇鐢ㄧ殑閫氱敤 cgroup 鎺ュ彛銆傚綋鍓?CFQ I/O 璋冨害鍣ㄧ敤瀹冩潵璇嗗埆浠诲姟缁勨€︹€? |
| BLK_DEV_INITRD | bool | 鍒濆 RAM 鏂囦欢绯荤粺鏄敱寮曞鍔犺浇绋嬪簭锛坙oadlin 鎴?lilo锛夊姞杞界殑 ramfs锛屽苟鍦ㄦ甯稿紩瀵兼祦绋嬩箣鍓嶆寕杞戒负鏍规枃浠剁郴缁熴€傚畠閫氬父鐢ㄤ簬鍔犺浇鎵€闇€妯″潡鈥︹€? |
| BOOTPARAM_HUNG_TASK_PANIC | int | 褰撹涓洪潪闆跺€兼椂锛岃嫢鍦ㄥ崟娆℃壂鎻忎腑鍙戠幇鐨勬寕璧蜂换鍔℃暟閲忚揪鍒拌鍊硷紝灏嗚Е鍙戝唴鏍?panic銆傝 panic 鍙笌 panic_timeout 閰嶅悎浣跨敤锛屼互鈥︹€? |
| BOOTPARAM_SOFTLOCKUP_PANIC | int | 璁句负闈為浂鍊?N锛屼娇鍐呮牳鍦ㄥ嚭鐜扳€滆蒋閿佹锛坰oft lockup锛夆€濇椂 panic锛涜蒋閿佹鏄寚瀵艰嚧鍐呮牳鍦ㄥ唴鏍告ā寮忎笅寰幆瓒呰繃 (N * 20 绉?锛堝彲浣跨敤 watchdo鈥︹€﹂厤缃級鐨勭己闄枫€? |
| BOOTPARAM_WQ_STALL_PANIC | int | 璁剧疆瑙﹀彂鍐呮牳 panic 鐨勫伐浣滈槦鍒楀仠婊炴鏁般€傚綋宸ヤ綔绾跨▼姹犲湪瓒呰繃 30 绉掞紙鍙娇鐢ㄢ€︹€﹂厤缃級鍐呭鏌愪釜寰呭鐞嗗伐浣滈」娌℃湁杩涘睍鏃讹紝鍗冲彂鐢熷伐浣滈槦鍒楀仠婊炪€? |
| BOOT_CONFIG | bool | 棰濆鐨勫紩瀵奸厤缃厑璁哥郴缁熺鐞嗗憳鍦ㄥ唴鏍稿紩瀵兼椂锛屽皢涓€浠介厤缃枃浠朵綔涓哄唴鏍稿懡浠よ鍙傛暟鐨勮ˉ鍏呮墿灞曚紶鍏ャ€傝寮曞閰嶇疆鏂囦欢蹇呴』浠ユ牎楠屽拰褰㈠紡闄勫姞鍦?initramfs 鏈熬锛屽洜鈥︹€? |
| BOOT_CONFIG_EMBED | bool | 灏?BOOT_CONFIG_EMBED_FILE 鎸囧畾鐨?bootconfig 鏂囦欢宓屽叆鍐呮牳銆傞€氬父 bootconfig 鏂囦欢闅?initrd 闀滃儚鍔犺浇銆備絾鑻ョ郴缁熶笉鏀寔 initrd锛屾閫夐」浼氭湁鎵€甯姪鈥︹€? |
| BOOT_CONFIG_EMBED_FILE | string | 鎸囧畾灏嗚宓屽叆鍐呮牳鐨?bootconfig 鏂囦欢銆傚綋 initrd 涓病鏈夛紝鎴?initrd 涓病鏈夊叾浠?bootconfig 鏃讹紝灏嗕娇鐢ㄦ bootconfig銆? |
| BOOT_CONFIG_FORCE | bool | 璁剧疆姝?Kconfig 閫夐」鍚庯紝鍗充娇鐪佺暐鈥渂ootconfig鈥濆唴鏍稿紩瀵煎弬鏁帮紝涔熶細鎵ц BOOT_CONFIG 澶勭悊銆備簨瀹炰笂锛岃缃閫夐」鍚庯紝鏃犳硶浣垮唴鏍糕€︹€? |
| BOOT_PRINTK_DELAY | bool | 璇ョ紪璇戦€夐」閫氳繃鍦ㄦ瘡鏉″唴鏍稿紩瀵兼秷鎭悗鎻掑叆鐭殏寤惰繜锛屼娇浣犺兘鏇磋交鏉惧湴闃呰杩欎簺娑堟伅銆傚欢杩熷€间互姣涓哄崟浣嶏紝閫氳繃鍦ㄥ懡浠よ涓婁娇鐢?"boot_delay=N" 鎸囧畾銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| BRIDGE_NETFILTER | tristate | 鍚敤姝ら€夐」鍚庯紝arptables 涓?iptables 灏嗚兘鐪嬪埌缁忚繃妗ユ帴鐨?ARP 鎴?IP 娴侀噺銆傝嫢浣犳兂瑕佷竴涓ˉ鎺ラ槻鐏锛屽緢鍙兘搴斿綋鍚敤姝ら€夐」銆?|
| BROKEN | bool | 姝ら€夐」鍏佽浣犻€夋嫨鏄惁灏濊瘯缂栬瘧锛堝苟淇锛夊皻鏈洿鏂板埌鏂板熀纭€璁炬柦鐨勬棫椹卞姩銆?|
| BROKEN_ON_SMP | bool | 缁忕敱鍐呮牳鍛戒护琛屼紶閫掔粰 init 鐨勫弬鏁颁笌鐜鍙橀噺鏁伴噺鍚勮嚜鐨勬渶澶у€笺€?|
| BSD_PROCESS_ACCT | bool | 鑻ュ湪姝ら€?Y锛屼竴涓敤鎴锋€佺▼搴忎究鑳芥寚绀哄唴鏍革紙閫氳繃鐗规畩鐨勭郴缁熻皟鐢級灏嗚繘绋嬭璐︿俊鎭啓鍏ユ枃浠讹細姣忓綋涓€涓繘绋嬮€€鍑烘椂锛屾湁鍏宠杩涚▼鐨勪俊鎭究浼氳璁板綍銆?|
| BSD_PROCESS_ACCT_V3 | bool | 鑻ュ湪姝ら€?Y锛岃繘绋嬭璐︿俊鎭皢浠ヤ竴绉嶆柊鐨勬枃浠舵牸寮忓啓鍏ワ紝璇ユ牸寮忓悓鏃惰褰曟瘡涓繘绋嬪強鍏剁埗杩涚▼鐨勮繘绋?ID銆傛敞鎰忔鏂囦欢鏍煎紡涓庢棫鏍煎紡涓嶅吋瀹广€?|
| BUG | bool | 绂佺敤姝ら€夐」浼氱Щ闄ゅ BUG 涓?WARN 鐨勬敮鎸侊紝缂╁皬鍐呮牳闀滃儚浣撶Н锛屼絾涔熷彲鑳藉湪鍚庡彴闈欓粯蹇界暐澶ч噺鑷村懡鐘跺喌銆備綘鍙簲鍦ㄦ槑纭渶瑕佹椂鑰冭檻绂佺敤瀹冦€?|
| BUILD_SALT | string | 鏋勫缓 ID 鐢ㄤ簬灏嗕簩杩涘埗鏂囦欢涓庡叾璋冭瘯淇℃伅鐩稿叧鑱斻€傝缃閫夐」灏嗗湪鏋勫缓 ID 鐨勮绠椾腑浣跨敤璇ュ€笺€傝繖瀵逛簬甯屾湜纭繚鈥︹€?|
| BUILTIN_MODULE_RANGES | bool | 褰撴ā鍧楄缂栬瘧杩涘唴鏍告椂锛?proc/kallsyms 涓殑绗﹀彿灏嗕笉鍐嶅叧鑱旀ā鍧楀悕銆傝拷韪櫒鍙兘甯屾湜鏃犺鈥︹€﹂兘鎸夋ā鍧楀悕涓庣鍙峰悕璇嗗埆绗﹀彿銆?|
| CACHESTAT_SYSCALL | bool | 鍚敤 cachestat 绯荤粺璋冪敤锛屽畠鍙煡璇㈡枃浠剁殑椤电紦瀛樼粺璁′俊鎭紙宸茬紦瀛橀〉鏁般€佽剰椤垫暟銆佹爣璁颁负鍥炲啓鐨勯〉鏁般€侊紙杩戞湡锛夎鍥炴敹鐨勯〉鏁帮級銆傝嫢涓嶇‘瀹氾紝鍦ㄦ閫?Y銆?|
| CC_IS_GCC | def_bool | 瀹冧笉渚濊禆浜?`RUST`锛屽洜涓哄悗鑰呭彲鑳介渶瑕佸湪 `depends on` 涓娇鐢ㄨ鐗堟湰銆?|
| CC_OPTIMIZE_FOR_PERFORMANCE | bool | 杩欐槸鍐呮牳鐨勯粯璁や紭鍖栫骇鍒紝浣跨敤 "-O2" 缂栬瘧鍣ㄦ爣蹇楁瀯寤猴紝浠ヨ幏寰楁渶浣虫€ц兘涓庢渶鏈夌敤鐨勭紪璇戞湡璀﹀憡銆?|
| CC_OPTIMIZE_FOR_SIZE | bool | 閫夋嫨姝ら€夐」浼氬悜缂栬瘧鍣ㄤ紶閫?"-Os"锛屼粠鑰岀敓鎴愭洿灏忕殑鍐呮牳銆?|
| CC_VERSION_TEXT | string | 瀹冪殑鐢ㄩ€斾笉澶槑纭細- 褰撶紪璇戝櫒鏇存柊鏃堕噸鏂拌繍琛?Kconfig銆?default' 灞炴€у紩鐢ㄧ幆澧冨彉閲?CC_VERSION_TEXT锛屽洜姝ゅ畠浼氳璁板綍鍒?include/config/auto.conf鈥︹€?|
| CGROUP_BPF | bool | 鍏佽浣跨敤 bpf(2) 绯荤粺璋冪敤鐨?BPF_PROG_ATTACH 鍛戒护灏?eBPF 绋嬪簭闄勫姞鍒?cgroup銆傝繖浜涚▼搴忓湪浣曠涓婁笅鏂囦腑琚闂彇鍐充簬闄勫姞鐨勭被鍨嬨€?|
| CGROUP_CPUACCT | bool | 鎻愪緵涓€涓畝鍗曠殑鎺у埗鍣紝鐢ㄤ簬鐩戞帶 cgroup 涓换鍔℃秷鑰楃殑鎬?CPU 鏃堕棿銆?|
| CGROUP_DEBUG | bool | 姝ら€夐」鍚敤涓€涓畝鍗曠殑鎺у埗鍣紝瀵煎嚭鍏充簬 cgroups 妗嗘灦鐨勮皟璇曚俊鎭€傝鎺у埗鍣ㄤ粎鐢ㄤ簬鎺у埗 cgroup 璋冭瘯锛屽叾鎺ュ彛涓嶇ǔ瀹氥€傚缓璁€?N銆?|
| CGROUP_DMEM | bool | DMEM 鎺у埗鍣ㄥ厑璁稿吋瀹圭殑璁惧鍩轰簬 cgroup 灞傜骇闄愬埗璁惧鍐呭瓨浣跨敤銆備緥濡傦紝瀹冨厑璁镐綘闄愬埗 DRM 瀛愮郴缁熶腑搴旂敤鐨?VRAM 鐢ㄩ噺銆?|
| CGROUP_FREEZER | bool | 鎻愪緵涓€绉嶅喕缁撲笌瑙ｅ喕 cgroup 涓墍鏈変换鍔＄殑鏂瑰紡銆傛閫夐」褰卞搷鍘熷鐨?cgroup 鎺ュ彛銆俢group2 鍐呭瓨鎺у埗鍣ㄩ粯璁ゅ寘鍚噸瑕佺殑鍐呮牳鍐呭唴瀛樻秷璐硅€呪€︹€?|
| CGROUP_HUGETLB | bool | 涓?HugeTLB 椤垫彁渚涗竴涓?cgroup 鎺у埗鍣ㄣ€傚惎鐢ㄥ悗锛屼綘鍙互瀵规瘡涓?cgroup 璁剧疆 HugeTLB 浣跨敤涓婇檺銆傝闄愬埗鍦ㄧ己椤垫椂寮哄埗鎵ц銆傜敱浜?HugeTLB 涓嶆敮鎸侀〉鍥炴敹鈥︹€?|
| CGROUP_MISC | bool | 涓轰富鏈轰笂鐨勬潅椤硅祫婧愭彁渚涙帶鍒跺櫒銆傛潅椤规爣閲忚祫婧愭槸涓绘満绯荤粺涓棤娉曞儚鍏朵粬 cgroup 閭ｆ牱鎶借薄鐨勮祫婧愩€傝鎺у埗鍣ㄢ€︹€?|
| CGROUP_NET_CLASSID | bool | 鐢ㄤ綔閫氱敤濂楁帴瀛?classid 鏍囪鐨?cgroup 瀛愮郴缁燂紝鐢ㄤ簬 cls_cgroup 涓?netfilter 鍖归厤銆?|
| CGROUP_PERF | bool | 姝ら€夐」鎵╁睍 perf 鐨勬瘡 CPU 妯″紡锛屽皢鐩戞帶闄愬埗鍒板睘浜庢寚瀹?cgroup 骞跺湪鎸囧畾 CPU 涓婅繍琛岀殑绾跨▼銆備篃鍙敤浜庡湪閲囨牱涓甫涓?cgroup ID鈥︹€?|
| CGROUP_PIDS | bool | 鍦?cgroup 鑼冨洿鍐呭己鍒惰繘绋嬫暟閲忎笂闄愩€備换浣曡秴鍑?cgroup 鍏佽鏁伴噺鑰屽皾璇?fork 鏇村杩涚▼鐨勬搷浣滈兘灏嗗け璐ャ€侾ID 鏈川涓婃槸涓€绉嶅叏灞€璧勬簮锛屽洜涓衡€︹€?|
| CGROUP_RDMA | bool | 寮哄埗瀹炴柦鐢?IB 鍗忚鏍堝畾涔夌殑 RDMA 璧勬簮闄愬埗銆傛秷璐硅€呭緢瀹规槗鑰楀敖 RDMA 璧勬簮锛屼粠鑰屽鑷村叾浠栨秷璐硅€呮棤娉曡幏寰楄祫婧愩€俁DMA 鎺у埗鈥︹€?|
| CGROUP_WRITEBACK | bool | 璇ョ壒鎬ц CPU 璋冨害鍣ㄨ瘑鍒换鍔＄粍锛屽苟瀵硅繖浜涗换鍔＄粍鎺у埗 CPU 甯﹀鍒嗛厤銆傚畠浣跨敤 cgroups 瀵逛换鍔″垎缁勶紙鑻?CGROUP_SCHED锛夈€?|
| CHECKPOINT_RESTORE | bool | 涓烘鏌ョ偣/鎭㈠涔嬬洰鐨勫惎鐢ㄩ澶栫殑鍐呮牳鐗规€с€傜壒鍒槸瀹冩坊鍔犱簡杈呭姪鐨?prctl 浠ｇ爜浠ヨ缃繘绋嬩唬鐮佹銆佹暟鎹涓庡爢娈电殑澶у皬锛屼互鍙婂皯閲忛澶栫殑 /proc 鏂囦欢鈥︹€?|
| CHECKSUM_KUNIT | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃舵祴璇曟牎楠屽拰鍑芥暟銆侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞朵互 TAP 鏍煎紡锛坔ttp://testanything.org/锛夊皢缁撴灉杈撳嚭鍒拌皟璇曟棩蹇椼€備粎瀵瑰唴鏍稿紑鍙戣€呮湁鐢ㄣ€?|
| CLOSURES | bool | 瀵?cpumask_var_t 浣跨敤鍔ㄦ€佸垎閰嶏紝鑰岄潪灏嗗叾鏀惧湪鏍堜笂銆傝繖鏍峰紑閿€鐣ュぇ锛屼絾鍙伩鍏嶆爤婧㈠嚭銆?|
| CMA_AREAS | int | CMA 鍙拡瀵圭壒瀹氱敤閫斿垱寤?CMA 鍖哄煙锛屼富瑕佺敤浣滆澶囩鏈夊尯鍩熴€傛鍙傛暟璁剧疆绯荤粺涓?CMA 鍖哄煙鐨勬渶澶ф暟閲忋€傝嫢涓嶇‘瀹氾紝淇濈暀榛樿鍊?"8"銆?|
| CMA_DEBUGFS | bool | 寮€鍚?CMA 鐨?DebugFS 鎺ュ彛銆?|
| CMA_SYSFS | bool | 姝ら€夐」鏆撮湶涓€浜?sysfs 灞炴€э紝浠ヤ粠 CMA 鑾峰彇淇℃伅銆?|
| CMDLINE_KUNIT_TEST | tristate | 鏋勫缓 cmdline API 鍗曞厓娴嬭瘯锛屾祴璇?cmdline.c 鎻愪緵鐨?API 閫昏緫銆傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation 涓殑 KUnit 鏂囨。鈥︹€?|
| CMDLINE_LOG_WRAP_IDEAL_LEN | int | 寮曞鏃讹紝鍐呮牳鍛戒护琛屼細琚褰曞埌鎺у埗鍙般€傛棩蹇楁秷鎭互鍓嶇紑 "Kernel command line: " 寮€澶淬€傝鏃ュ織娑堟伅浼氬皾璇曡嚜鍔ㄦ崲琛岋紙鎷嗗垎涓哄琛屸€︹€?|
| CODE_TAGGING | bool | 璺熻釜鍒嗛厤婧愪唬鐮佸苟璁板綍鍦ㄨ浠ｇ爜浣嶇疆鍙戣捣鐨勫垎閰嶆€诲ぇ灏忋€傝鏈哄埗鍙敤浜庝互杈冧綆鐨勬€ц兘涓庡唴瀛樺紑閿€璺熻釜鍐呭瓨娉勬紡銆?|
| COMPACTION | bool | 鍐呭瓨瑙勬暣鏄敮涓€鑳藉彲闈犲舰鎴愰珮闃讹紙鏇村ぇ鐗╃悊杩炵画锛夊唴瀛樺潡鐨勫唴瀛樼鐞嗙粍浠躲€傞〉鍒嗛厤鍣ㄩ珮搴︿緷璧栧唴瀛樿鏁达紝缂轰箯璇ョ壒鎬р€︹€?|
| COMPACT_UNEVICTABLE_DEFAULT | int | 绌洪棽椤垫眹鎶ュ厑璁镐粠浼欎即鍒嗛厤鍣ㄥ閲忚幏鍙栫┖闂查〉锛屼互渚垮皢杩欎簺椤垫眹鎶ョ粰鍙︿竴瀹炰綋锛堝 hypervisor锛夛紝浠庤€岃鍐呭瓨鈥︹€?|
| COMPAT_BINFMT_ELF | def_bool | ELF FDPIC 浜岃繘鍒跺熀浜?ELF锛屼絾鍏佽浜岃繘鍒舵枃浠剁殑鍚勪釜鍔犺浇娈靛湪鍐呭瓨涓郊姝ょ嫭绔嬪湴瀹氫綅銆傝繖浣垮緱璇ユ牸寮忛潪甯搁€傚悎鐢ㄤ簬鈥︹€︾幆澧?|
| COMPAT_BRK | bool | 闅忔満鍖栧爢甯冨眬浣垮爢鍒╃敤鏀诲嚮鏇村洶闅撅紝浣嗕篃浼氱牬鍧忓彜鑰佺殑浜岃繘鍒讹紙鍖呮嫭浠讳綍鍩轰簬 libc5 鐨勭▼搴忥級銆傛閫夐」灏嗗紩瀵奸粯璁ゆ敼涓虹鐢ㄥ爢闅忔満鍖栤€︹€?|
| COMPAT_NETLINK_MESSAGES | def_bool | 姝ら€夐」浣垮緱鍙互鏍规嵁浠诲姟鏄惁涓?compat 浠诲姟锛屽悜浠诲姟鍙戦€佷笉鍚岀殑 netlink 娑堟伅銆備负姝わ紝浣犻渶瑕佸皢 skb_shinfo(skb)->frag_list 璁剧疆涓衡€︹€?|
| COMPILE_TEST | bool | 鏌愪簺椹卞姩鍙互鍦ㄤ笌鍏惰繍琛屽钩鍙颁笉鍚岀殑骞冲彴涓婄紪璇戙€傚敖绠″畠浠棤娉曞湪閭ｉ噷鍔犺浇锛堟垨鍗充究鍔犺浇涔熷洜缂哄皯纭欢鏀寔鑰屾棤娉曚娇鐢級鈥︹€?|
| CONSOLE_LOGLEVEL_DEFAULT | int | 鍐冲畾鎺у埗鍙板皢鎵撳嵃鍝簺鍐呭鐨勯粯璁ゆ棩蹇楃骇鍒€傚湪姝よ缃粯璁ゅ€肩瓑鍚屼簬鍦ㄥ唴鏍稿紩瀵煎弬鏁颁腑浼犲叆 loglevel=<x>銆俵oglevel=<x> 浠嶄細瑕嗙洊姝ゅ璁剧疆鈥︹€?|
| CONSOLE_LOGLEVEL_QUIET | int | 褰撳唴鏍稿懡浠よ浼犲叆 "quiet" 鏃朵娇鐢ㄧ殑鏃ュ織绾у埆銆傚綋鍛戒护琛屼紶鍏?"quiet" 鏃讹紝璇ユ棩蹇楃骇鍒皢浣滀负鏃ュ織绾у埆浣跨敤銆傛崲瑷€涔嬶紝浼犲叆 "quiet" 绛夋晥浜庘€︹€?|
| CONTEXT_ANALYSIS_TEST | bool | 鏋勫缓鐢ㄤ簬鍩轰簬缂栬瘧鍣ㄧ殑涓婁笅鏂囧垎鏋愮殑娴嬭瘯銆傝娴嬭瘯涓嶄細鍚戝唴鏍告坊鍔犲彲鎵ц浠ｇ爜锛岃€屾槸鐢ㄤ簬楠岃瘉鍒嗘瀽鎵€鏀寔鐨勫父瑙佹ā寮忎笉浼氬鑷粹€︹€?|
| CONTIG_ALLOC | def_bool | 鍦ㄩ〉鍒嗛厤鍣ㄤ腑锛孭CP锛堟瘡 CPU 椤甸泦锛変互鎵瑰鐞嗘柟寮忚ˉ鍏呬笌娓呯┖銆傛壒娆℃暟浼氳嚜鍔ㄧ缉鏀句互鏀瑰杽椤靛垎閰?閲婃斁鍚炲悙銆備絾杩囧ぇ鐨勭缉鏀惧洜瀛愬彲鑳芥崯瀹斥€︹€?|
| COREDUMP | bool | 姝ら€夐」鍚敤瀵规墽琛屾牳蹇冭浆鍌ㄧ殑鏀寔銆備綘鍑犱箮鑲畾搴斿綋鍦ㄦ閫?Y銆傚浜庝粠涓嶉渶瑕佽皟璇曟垨鍙繍琛屾棤鐟曠柕浠ｇ爜鐨勭郴缁熷垯闈炲繀闇€銆?|
| CORE_DUMP_DEFAULT_ELF_HEADERS | bool | ELF 鏍稿績杞偍鏂囦欢鎻忚堪宕╂簝杩涚▼鐨勬瘡涓唴瀛樻槧灏勶紝骞跺彲鍖呭惈鎴栫渷鐣ュ叾涓瘡涓€涓殑鍐呭瓨鍐呭銆傛湭淇敼鐨勪唬鐮佹鏄犲皠鍐呭榛樿琚渷鐣ャ€?|
| CPUMASK_KUNIT_TEST | tristate | 鍚敤 cpumask 娴嬭瘯锛屽湪寮曞鎴栨ā鍧楀姞杞芥椂杩愯銆傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit 涓殑 KUnit 鏂囨。鈥︹€?|
| CPUSETS | bool | 姝ら€夐」鍏佽浣犲垱寤轰笌绠＄悊 CPUSET锛屼粠鑰屽皢绯荤粺鍔ㄦ€佸垝鍒嗕负鑻ュ共 CPU 涓庡唴瀛樿妭鐐归泦鍚堬紝骞跺皢浠诲姟闄愬畾涓哄彧鑳藉湪杩欎簺闆嗗悎鍐呰繍琛屻€傝繖涓昏鐢ㄤ簬鈥︹€?|
| CPUSETS_V1 | bool | 宸茶 cgroup v2 瀹炵幇搴熷純鐨勪紶缁?cgroup v1 cpusets 鎺у埗鍣ㄣ€倂1 淇濈暀缁欏皻鏈縼绉诲埌鏂?cgroup v2 鎺ュ彛鐨勯仐鐣欏簲鐢ㄤ娇鐢ㄣ€傞仐鐣欌€︹€?|
| CPU_HOTPLUG_STATE_CONTROL | bool | 鍏佽灏?"offline" 涓?"online" 涔嬮棿鐨勫悇涓楠ゅ啓鍏?CPU 鐨?sysfs 鐩爣鏂囦欢锛屼粠鑰屽彲浠ラ€愭绮剧粏鍦板垏鎹㈢姸鎬併€傜洰鍓嶈繖浠嶆槸涓€涓皟璇曢€夐」锛屽洜涓虹儹鎻掓嫈鏈哄埗鏃犳硶琚仠姝⑩€︹€?|
| CPU_ISOLATION | bool | 纭繚杩愯鍏抽敭浠诲姟鐨?CPU 涓嶅彈浠讳綍鈥滃櫔澹扳€濇簮锛堝鏈粦瀹氱殑宸ヤ綔闃熷垪銆佸畾鏃跺櫒銆佸唴鏍哥嚎绋嬧€︹€︼級骞叉壈銆傛湭缁戝畾鐨勪换鍔′細琚浆绉诲埌绠″ CPU 涓娿€傝鐗规€х敱鈥︹€?|
| CROSS_MEMORY_ATTACH | bool | 鍚敤姝ら€夐」浼氭坊鍔?process_vm_readv 涓?process_vm_writev 绯荤粺璋冪敤锛屽厑璁告嫢鏈夌浉搴旀潈闄愮殑杩涚▼鐩存帴璇诲彇鎴栧啓鍏ュ彟涓€杩涚▼鐨勫湴鍧€绌洪棿銆?|
| CRYPTO | tristate | 姝ら€夐」鎻愪緵鏍稿績鍔犲瘑 API锛堣嫢 CRYPTO锛夈€?|
| CRYPTO_842 | tristate | IBM 鐨?842 鍘嬬缉绠楁硶銆傛洿澶氫俊鎭鍙傞槄 https://github.com/plauth/lib842銆?|
| CRYPTO_ADIANTUM | tristate | Adiantum 鍙皟鏁淬€佷繚鎸侀暱搴︾殑鍔犲瘑妯″紡銆備笓涓哄揩閫熶笖瀹夊叏鐨勭鐩樺姞瀵嗚€岃璁★紝灏ゅ叾閫傜敤浜庢病鏈変笓鐢ㄥ姞瀵嗘寚浠ょ殑 CPU銆傚畠浣跨敤 XCha鈥︹€﹀姣忎釜鎵囧尯杩涜鍔犲瘑 |
| CRYPTO_AEGIS128 | tristate | AEGIS-128 AEAD 绠楁硶 |
| CRYPTO_AEGIS128_SIMD | bool | AEGIS-128 AEAD 绠楁硶銆備綋绯荤粨鏋勶細arm 鎴?arm64锛屼娇鐢細- NEON锛圓dvanced SIMD锛夋墿灞?|
| CRYPTO_AES | tristate | AES 瀵嗙爜绠楁硶锛圧ijndael锛夛紙FIPS-197, ISO/IEC 18033-3锛夈€俁ijndael 鍦ㄥ箍娉涚殑杞‖浠惰绠楃幆澧冧腑閮借〃鐜扮ǔ瀹氫笖浼樺紓鈥︹€?|
| CRYPTO_ALGAPI | tristate | 姝ら€夐」鎻愪緵鍔犲瘑绠楁硶鐨?API銆?|
| CRYPTO_ALGAPI2 | tristate | 鎻愪緵瀹炰緥鍖?cbc(aes) 绛夋ā鏉跨殑鏀寔锛屼互鍙婂姞瀵嗚嚜妫€娴嬭瘯鐨勬敮鎸併€?|
| CRYPTO_ANUBIS | tristate | Anubis 瀵嗙爜绠楁硶銆侫nubis 鏄竴绉嶅彲鍙樺瘑閽ラ暱搴﹀瘑鐮侊紝鍙娇鐢?128 浣嶈嚦 320 浣嶇殑瀵嗛挜銆傚畠鏇句綔涓?NESSIE 绔炶禌鐨勫€欓€夌畻娉曞弬璇勩€?|
| CRYPTO_ARC4 | tristate | ARC4 瀵嗙爜绠楁硶銆侫RC4 鏄竴绉嶆祦瀵嗙爜锛屼娇鐢?8 浣嶈嚦 2048 浣嶉暱搴︾殑瀵嗛挜銆傝绠楁硶鏄熀浜庨┍鍔ㄧ殑 WEP 鎵€蹇呴渶鐨勶紝浣嗕笉搴斿皢鍏剁敤浜庡叾浠栫洰鐨勨€︹€?|
| CRYPTO_ARIA | tristate | ARIA 瀵嗙爜绠楁硶锛圧FC5794锛夈€侫RIA 鏄ぇ闊╂皯鍥界殑鏍囧噯鍔犲瘑绠楁硶锛岃瀹氫簡涓夌瀵嗛挜闀垮害涓庤疆鏁帮細128 浣?12 杞€?92 浣?14 杞€?56 浣?16 杞€︹€?|
| CRYPTO_AUTHENC | tristate | Authenc锛欼Psec 鐨勭粍鍚堟ā寮忓皝瑁呫€侷PSec ESP锛圶FRM_ESP锛夐渶瑕佸畠銆?|
| CRYPTO_BENCHMARK | tristate | 涓€涓畝鍗曠矖鏆寸殑鍔犲瘑鍩哄噯娴嬭瘯妯″潡锛屼富瑕佷緵鍦ㄥ唴鏍镐腑寮€鍙戝姞瀵嗙畻娉曠殑浜轰娇鐢ㄣ€傜敓浜у唴鏍镐笉搴斿惎鐢ㄥ畠銆?|
| CRYPTO_BLAKE2B | tristate | BLAKE2b 鍔犲瘑鍝堝笇鍑芥暟锛圧FC 7693锛夈€侭LAKE2b 閽堝 64 浣嶅钩鍙颁紭鍖栵紝鍙敓鎴?1 鑷?64 瀛楄妭浠绘剰闀垮害鐨勬憳瑕併€備篃瀹炵幇浜嗗甫瀵嗛挜鐨勫搱甯屻€傝妯″潡鈥︹€?|
| CRYPTO_BLOWFISH | tristate | Blowfish 瀵嗙爜绠楁硶锛岀敱 Bruce Schneier 璁捐銆傝繖鏄竴绉嶅彲鍙樺瘑閽ラ暱搴︾殑瀵嗙爜锛屽彲浣跨敤 32 浣嶈嚦 448 浣嶇殑瀵嗛挜銆傚畠蹇€熴€佺畝鍗曪紝涓撲负鈥滃ぇ鍨嬧€濃€︹€﹁璁?|
| CRYPTO_BLOWFISH_COMMON | tristate | 鐢遍€氱敤 C 瀹炵幇涓庢眹缂栧疄鐜板叡浜殑 Blowfish 瀵嗙爜绠楁硶鍏叡閮ㄥ垎銆?|
| CRYPTO_CAMELLIA | tristate | Camellia 瀵嗙爜绠楁硶锛圛SO/IEC 18033-3锛夈€侰amellia 鏄敱 NTT 涓庝笁鑿辩數鏈鸿仈鍚堝紑鍙戠殑瀵圭О瀵嗛挜鍒嗙粍瀵嗙爜锛岃瀹氫簡涓夌瀵嗛挜闀垮害锛?28銆?92銆?56 浣嶁€︹€?|
| CRYPTO_CAST5 | tristate | CAST5锛圕AST-128锛夊瘑鐮佺畻娉曪紙RFC2144, ISO/IEC 18033-3锛?|
| CRYPTO_CAST6 | tristate | CAST6锛圕AST-256锛夊姞瀵嗙畻娉曪紙RFC2612锛?|
| CRYPTO_CAST_COMMON | tristate | 鐢遍€氱敤 C 瀹炵幇涓庢眹缂栧疄鐜板叡浜殑 CAST 瀵嗙爜绠楁硶鍏叡閮ㄥ垎銆?|
| CRYPTO_CBC | tristate | CBC锛堝瘑鐮佸垎缁勯摼鎺ワ級妯″紡锛圢IST SP800-38A锛夈€侷PSec ESP锛圶FRM_ESP锛夐渶瑕佹鍒嗙粍瀵嗙爜妯″紡銆?|
| CRYPTO_CCM | tristate | CCM锛堣鏁板櫒涓庡瘑鐮佸垎缁勯摼鎺?娑堟伅璁よ瘉鐮侊級璁よ瘉鍔犲瘑妯″紡锛圢IST SP800-38C锛?|
| CRYPTO_CHACHA20 | tristate | ChaCha20銆乆ChaCha20 涓?XChaCha12 娴佸瘑鐮佺畻娉曘€侰haCha20 鏄敱 Daniel J. Bernstein 璁捐鐨?256 浣嶉珮閫熸祦瀵嗙爜锛屽苟鍦?RFC7539 涓繘涓€姝ヨ鑼冿紝鐢ㄤ簬 IETF 鍗忚鈥︹€?|
| CRYPTO_CHACHA20POLY1305 | tristate | ChaCha20 娴佸瘑鐮佷笌 Poly1305 璁よ瘉鍣ㄧ殑缁勫悎妯″紡锛圧FC8439锛?|
| CRYPTO_CMAC | tristate | CMAC锛堝熀浜庡瘑鐮佺殑娑堟伅璁よ瘉鐮侊級璁よ瘉妯″紡锛圢IST SP800-38B 涓?IETF RFC4493锛?|
| CRYPTO_CRC32 | tristate | CRC32 CRC 绠楁硶锛圛EEE 802.3锛?|
| CRYPTO_CRC32C | tristate | 閲囩敤 iSCSI 澶氶」寮忕殑 CRC32c CRC 绠楁硶锛圧FC 3385 涓?RFC 3720锛夈€備竴绉?32 浣?CRC锛堝惊鐜啑浣欐牎楠岋級锛屽叾澶氶」寮忕敱 G. Castagnoli銆丼. Braeuer 涓?M. Herrman 鍦ㄣ€奜ptimization鈥︹€︺€嬩腑瀹氫箟 |
| CRYPTO_CRYPTD | tristate | 杩欐槸涓€涓€氱敤鐨勮蒋浠跺紓姝ュ姞瀵嗗畧鎶よ繘绋嬶紝鍙皢浠绘剰鍚屾杞欢鍔犲瘑绠楁硶杞崲涓哄湪鍐呮牳绾跨▼涓墽琛岀殑寮傛绠楁硶銆?|
| CRYPTO_CTR | tristate | CTR锛堣鏁板櫒锛夋ā寮忥紙NIST SP800-38A锛?|
| CRYPTO_CTS | tristate | CTS锛堝瘑鏂囩獌鍙栵級鐨?CBC-CS3 鍙樹綋锛圢IST SP800-38A 澧炶ˉ锛?010 骞?10 鏈堬級锛夈€侫ES 鍔犲瘑鐨?Kerberos gss 鏈哄埗鏀寔闇€瑕佹妯″紡銆?|
| CRYPTO_DEFLATE | tristate | Deflate 鍘嬬缉绠楁硶锛圧FC1951锛夈€傜敱 IPSec 閰嶅悎 IPCOMP 鍗忚浣跨敤锛圧FC3173, RFC2394锛?|
| CRYPTO_DES | tristate | DES锛堟暟鎹姞瀵嗘爣鍑嗭級锛團IPS 46-2, ISO/IEC 18033-3锛変笌涓夐噸 DES EDE锛堝姞瀵?瑙ｅ瘑/鍔犲瘑锛夛紙FIPS 46-3, ISO/IEC 18033-3锛夊瘑鐮佺畻娉?|
| CRYPTO_DH | tristate | DH锛圖iffie-Hellman锛夊瘑閽ヤ氦鎹㈢畻娉?|
| CRYPTO_DH_RFC7919_GROUPS | bool | RFC7919 涓畾涔夌殑 FFDHE锛堝熀浜庢湁闄愬煙鐨勪复鏃?Diffie-Hellman锛夌粍銆傚湪 DH 瀵嗛挜浜ゆ崲涓敮鎸佽繖浜涙湁闄愬煙缁勶細- ffdhe2048銆乫fdhe3072銆乫fdhe4096銆乫fdhe6144銆乫fdhe8192銆傝嫢涓嶇‘瀹氣€︹€?|
| CRYPTO_DRBG | tristate | 鏉ヨ嚜 Jitterentropy 搴撶殑 CPU Jitter RNG锛堥殢鏈烘暟鐢熸垚鍣級銆備竴绉嶉潪鐗╃悊銆侀潪纭畾鎬х殑锛堚€滅湡鈥濓級RNG锛堜緥濡傜鍚?NIST SP800-90B 鐨勭喌婧愶級锛屾棬鍦ㄦ彁渚涒€︹€?|
| CRYPTO_DRBG_CTR | bool | NIST SP800-90A 瀹氫箟鐨?CTR_DRBG 鍙樹綋銆傚畠浣跨敤 AES 瀵嗙爜绠楁硶閰嶅悎璁℃暟鍣ㄥ垎缁勬ā寮忋€?|
| CRYPTO_DRBG_HMAC | bool | NIST SP800-90A 瀹氫箟鐨?Hash_DRBG 鍙樹綋銆傚畠浣跨敤 SHA-1銆丼HA-256銆丼HA-384 鎴?SHA-512 鍝堝笇绠楁硶銆?|
| CRYPTO_DRBG_MENU | tristate | DRBG锛堢‘瀹氭€ч殢鏈烘瘮鐗圭敓鎴愬櫒锛夛紙NIST SP800-90A锛夈€傚湪闅忓悗鐨勫瓙鑿滃崟涓紝蹇呴』閫夋嫨涓€绉嶆垨澶氱 DRBG 绫诲瀷锛堣嫢 CRYPTO_DRBG_MENU锛夈€?|
| CRYPTO_ECB | tristate | ECB锛堢數瀛愬瘑鐮佹湰锛夋ā寮忥紙NIST SP800-38A锛?|
| CRYPTO_ECC | tristate | 浣跨敤 P-192銆丳-256 涓?P-384 鏇茬嚎鐨?ECDH锛堟き鍦嗘洸绾?Diffie-Hellman锛夊瘑閽ヤ氦鎹㈢畻娉曪紙FIPS 186锛?|
| CRYPTO_ECDSA | tristate | 浣跨敤 P-192銆丳-256銆丳-384 涓?P-521 鏇茬嚎鐨?ECDSA锛堟き鍦嗘洸绾挎暟瀛楃鍚嶇畻娉曪級锛團IPS 186, ISO/IEC 14888-3锛夈€傜洰鍓嶄粎瀹炵幇绛惧悕楠岃瘉銆?|
| CRYPTO_ECHAINIV | tristate | 鍔犲瘑閾惧紡 IV 鐢熸垚鍣ㄣ€傝 IV 鐢熸垚鍣ㄥ熀浜庡簭鍒楀彿涓庣洂寮傛垨鍚庡啀鍔犲瘑鏉ョ敓鎴?IV銆傝繖鏄?CBC 鐨勯粯璁ょ畻娉曘€?|
| CRYPTO_ECRDSA | tristate | 妞渾鏇茬嚎淇勭綏鏂暟瀛楃鍚嶇畻娉曪紙GOST R 34.10-2012, RFC 7091, ISO/IEC 14888-3锛夈€備縿缃楁柉瀵嗙爜鏍囧噯绠楁硶涔嬩竴锛堢О涓?GOST 绠楁硶锛夈€傜洰鍓嶄粎瀹炵幇绛惧悕楠岃瘉鈥︹€?|
| CRYPTO_ESSIV | tristate | 鍔犲瘑鐩?鎵囧尯 IV 鐢熸垚鍣ㄣ€傝 IV 鐢熸垚鍣ㄥ湪鏌愪簺鎯呭喌涓嬬敱 fscrypt 鍜?鎴?dm-crypt 浣跨敤銆傚畠浣跨敤鍧楀姞瀵嗗瘑閽ョ殑鍝堝笇浣滀负鍧楀姞瀵嗛亶鐨勫绉板瘑閽モ€︹€?|
| CRYPTO_FCRYPT | tristate | RxRPC 浣跨敤鐨?FCrypt 绠楁硶銆傚弬瑙?https://ota.polyonymo.us/fcrypt-paper.txt |
| CRYPTO_FIPS | bool | 姝ら€夐」鍚敤 fips 寮曞鍙傛暟锛岃嫢甯屾湜绯荤粺鍦?FIPS 200 璁よ瘉涓嬭繍琛屽垯闇€瑕佸畠銆傞櫎闈炰綘鐭ラ亾瀹冪殑鐢ㄩ€旓紝鍚﹀垯搴旈€夊惁銆?|
| CRYPTO_FIPS_CUSTOM_VERSION | bool | 姝ら€夐」鎻愪緵瑕嗙洊 FIPS 妯″潡鐗堟湰鐨勮兘鍔涖€傞粯璁や娇鐢?KERNELRELEASE 鍊笺€?|
| CRYPTO_FIPS_NAME | string | 姝ら€夐」璁剧疆鐢?Crypto API 閫氳繃 /proc/sys/crypto/fips_name 鏂囦欢鎶ュ憡鐨?FIPS 妯″潡鍚嶇О銆?|
| CRYPTO_GCM | tristate | GCM锛圙alois/璁℃暟鍣ㄦā寮忥級璁よ瘉鍔犲瘑妯″紡涓?GMAC锛圙CM 娑堟伅璁よ瘉鐮侊級锛圢IST SP800-38D锛夈€侷PSec ESP锛圶FRM_ESP锛夐渶瑕佸畠銆?|
| CRYPTO_GENIV | tristate | 搴忓垪鍙?IV 鐢熸垚鍣ㄣ€傝 IV 鐢熸垚鍣ㄩ€氳繃灏嗗簭鍒楀彿涓庣洂寮傛垨鏉ョ敓鎴?IV銆傝绠楁硶涓昏瀵?CTR 鏈夌敤銆侷Psec ESP锛圶FRM_ESP锛夐渶瑕佸畠銆?|
| CRYPTO_HCTR2 | tristate | HCTR2 淇濇寔闀垮害鐨勫姞瀵嗘ā寮忋€備竴绉嶇敤浜庡瓨鍌ㄥ姞瀵嗙殑妯″紡锛屽湪甯︽湁鍔犻€?AES 涓庢棤杩涗綅涔樻硶鐨勬寚浠ょ殑澶勭悊鍣紙濡傚甫鏈?AES-鈥︹€︾殑 x86 澶勭悊鍣級涓婃晥鐜囧緢楂?|
| CRYPTO_HMAC | tristate | HMAC锛堝甫瀵嗛挜鐨勫搱甯屾秷鎭璇佺爜锛夛紙FIPS 198 涓?RFC2104锛夈€侷Psec AH锛圶FRM_AH锛変笌 IPsec ESP锛圶FRM_ESP锛夐渶瑕佸畠銆?|
| CRYPTO_JITTERENTROPY_MEMORY_BLOCKS | int | 鍚敤鍝堝笇绠楁硶鐨勭敤鎴风┖闂存帴鍙ｃ€傝鍙傞槄 Documentation/crypto/userspace-if.rst 涓?https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_JITTERENTROPY_MEMSIZE_2 | bool | Jitter RNG 鍏佽鎸囧畾杩囬噰鏍风巼锛圤SR锛夈€侸itter RNG 鐨勮繍琛岄渶瑕佸浐瀹氭暟閲忕殑瀹氭椂娴嬮噺鎵嶈兘浜х敓涓€涓殢鏈烘暟杈撳嚭鍧椼€侽SR鈥︹€?|
| CRYPTO_JITTERENTROPY_TESTINTERFACE | bool | 璇ユ祴璇曟帴鍙ｅ厑璁哥壒鏉冭繘绋嬫崟鑾?Jitter RNG 鏀堕泦鐨勫師濮嬨€佹湭璋冭妭鐨勯珮鍒嗚鲸鐜囨椂闂存埑鍣０锛屼互渚涚粺璁″垎鏋愩€傜敱浜庤繖浜涙暟鎹鐢ㄤ綔鈥︹€?|
| CRYPTO_KHAZAD | tristate | Khazad 瀵嗙爜绠楁硶銆侹hazad 鏄灞?NESSIE 绔炶禌鐨勫喅璧涚畻娉曘€傚畠鏄竴绉嶉拡瀵?64 浣嶅鐞嗗櫒浼樺寲銆佸湪 32 浣嶅鐞嗗櫒涓婁篃鏈夎壇濂借〃鐜扮殑绠楁硶銆侹hazad 浣跨敤 128 浣嶁€︹€?|
| CRYPTO_KRB5ENC | tristate | 閽堝 Kerberos 5 RFC3961 绠€鍖栭厤缃枃浠剁殑缁勫悎鍝堝笇涓庡瘑鐮佹敮鎸併€俿unrpc/NFS 涓?rxrpc/AFS 鎵€浣跨敤鐨?Kerberos 5 椋庢牸鍔犲瘑闇€瑕佸畠銆?|
| CRYPTO_LRW | tristate | LRW锛圠iskov Rivest Wagner锛夋ā寮忋€備竴绉嶅彲璋冩暣銆佷笉鍙銆佷笉鍙Щ鍔ㄧ殑绐勫垎缁勫瘑鐮佹ā寮忥紝鐢ㄤ簬 dm-crypt銆傞厤鍚堝瘑鐮佽鏍煎瓧绗︿覆 aes-lrw-benbi 浣跨敤锛屽瘑閽ュ繀椤讳负 256銆?20 鎴?384鈥︹€?|
| CRYPTO_LZ4 | tristate | LZ4 鍘嬬缉绠楁硶銆傛洿澶氫俊鎭鍙傞槄 https://github.com/lz4/lz4銆?|
| CRYPTO_LZ4HC | tristate | LZ4 楂樺帇缂╂ā寮忕畻娉曘€傛洿澶氫俊鎭鍙傞槄 https://github.com/lz4/lz4銆?|
| CRYPTO_LZO | tristate | LZO 鍘嬬缉绠楁硶銆傛洿澶氫俊鎭鍙傞槄 https://www.oberhumer.com/opensource/lzo/銆?|
| CRYPTO_MANAGER2 | def_tristate | 閽堝 cbc(aes) 绛夊姞瀵嗗疄渚嬬殑鐢ㄦ埛绌洪棿閰嶇疆銆?|
| CRYPTO_MD4 | tristate | MD4 娑堟伅鎽樿绠楁硶锛圧FC1320锛?|
| CRYPTO_MD5 | tristate | MD5 娑堟伅鎽樿绠楁硶锛圧FC1321锛夛紝鍖呭惈 HMAC 鏀寔銆?|
| CRYPTO_MLDSA | tristate | ML-DSA锛堝熀浜庢ā鍧楁牸鐨勬暟瀛楃鍚嶇畻娉曪級锛團IPS-204锛夈€傜洰鍓嶄粎瀹炵幇绛惧悕楠岃瘉銆?|
| CRYPTO_NULL | tristate | 杩欎簺鏄?IPsec 浣跨敤鐨勨€淣ull鈥濈畻娉曪紝瀹冧滑涓嶅仛浠讳綍浜嬨€?|
| CRYPTO_PCBC | tristate | PCBC锛堜紶鎾紡瀵嗙爜鍒嗙粍閾炬帴锛夋ā寮忋€俁xRPC 闇€瑕佹鍒嗙粍瀵嗙爜妯″紡銆?|
| CRYPTO_PCRYPT | tristate | 杩欏皢浠绘剰鍔犲瘑绠楁硶杞崲涓哄湪鍐呮牳绾跨▼涓墽琛岀殑骞惰绠楁硶銆?|
| CRYPTO_RMD160 | tristate | RIPEMD-160 鍝堝笇鍑芥暟锛圛SO/IEC 10118-3锛夈€俁IPEMD-160 鏄竴绉?160 浣嶅姞瀵嗗搱甯屽嚱鏁帮紝鏃ㄥ湪浣滀负 128 浣嶅搱甯屽嚱鏁?MD4銆丮D5 鍙婂叾鍓嶈韩鐨勫畨鍏ㄦ浛浠ｅ搧鈥︹€?|
| CRYPTO_SEED | tristate | SEED 瀵嗙爜绠楁硶锛圧FC4269, ISO/IEC 18033-3锛夈€係EED 鏄竴绉?128 浣嶅绉板瘑閽ュ垎缁勫瘑鐮侊紝鐢?KISA锛堥煩鍥戒簰鑱旂綉涓庡畨鍏ㄥ眬锛変綔涓哄浗瀹舵爣鍑嗗姞瀵嗙畻娉曞紑鍙戔€︹€?|
| CRYPTO_SELFTESTS | bool | 鍚敤鍔犲瘑鑷娴嬭瘯銆傚姞瀵嗚嚜妫€娴嬭瘯鍦ㄥ紩瀵兼椂杩愯锛屾垨鍦ㄧ畻娉曟敞鍐屾椂锛堣嫢绠楁硶绋嶅悗鍔ㄦ€佸姞杞斤級杩愯銆備富瑕佹湁涓ょ浣跨敤鍦烘櫙鈥︹€?|
| CRYPTO_SELFTESTS_FULL | bool | 涓烘瘡涓畻娉曞惎鐢ㄥ畬鏁寸殑鍔犲瘑鑷娴嬭瘯闆嗐€傚畬鏁存祴璇曢泦搴斿湪寮€鍙戜笌鍙戝竷鍓嶆祴璇曟椂鍚敤锛屼絾涓嶅簲鍦ㄧ敓浜у唴鏍镐腑鍚敤銆傛墍鏈夊姞瀵嗕唬鐮佲€︹€?|
| CRYPTO_SERPENT | tristate | Serpent 瀵嗙爜绠楁硶锛岀敱 Anderson銆丅iham 涓?Knudsen 璁捐銆傚瘑閽ラ暱搴﹀厑璁镐负 0 鑷?256 浣嶏紝浠?8 浣嶄负姝ラ暱銆傛洿澶氫俊鎭鍙傞槄 https://www.cl.cam.ac.uk/~rja14/serpent.html鈥︹€?|
| CRYPTO_SHA1 | tristate | SHA-1 瀹夊叏鍝堝笇绠楁硶锛團IPS 180, ISO/IEC 10118-3锛夛紝鍖呭惈 HMAC 鏀寔銆?|
| CRYPTO_SHA256 | tristate | SHA-224 涓?SHA-256 瀹夊叏鍝堝笇绠楁硶锛團IPS 180, ISO/IEC 10118-3锛夛紝鍖呭惈 HMAC 鏀寔銆侷Psec AH锛圶FRM_AH锛変笌 IPsec ESP锛圶FRM_ESP锛夐渶瑕佸畠銆?|
| CRYPTO_SHA3 | tristate | SHA-3 瀹夊叏鍝堝笇绠楁硶锛團IPS 202, ISO/IEC 10118-3锛?|
| CRYPTO_SHA512 | tristate | SHA-384 涓?SHA-512 瀹夊叏鍝堝笇绠楁硶锛團IPS 180, ISO/IEC 10118-3锛夛紝鍖呭惈 HMAC 鏀寔銆?|
| CRYPTO_SIMD | tristate | RSA锛圧ivest-Shamir-Adleman锛夊叕閽ョ畻娉曪紙RFC8017锛?|
| CRYPTO_SM3 | tristate | SM3锛堝晢瀵?3锛夊畨鍏ㄥ搱甯屽嚱鏁帮紙OSCCA GM/T 0004-2012, ISO/IEC 10118-3锛夈€傝繖鏄腑鍥藉晢鐢ㄥ瘑鐮佷綋绯荤殑涓€閮ㄥ垎銆傚弬鑰冿細http://www.oscca.gov.cn/UpFile/20101222141857786鈥︹€?|
| CRYPTO_SM4 | tristate | SM4 瀵嗙爜绠楁硶锛圤SCCA GB/T 32907-2016, ISO/IEC 18033-3:2010/Amd 1:2021锛夈€係M4锛圙BT.32907-2016锛夋槸鐢变腑鍥藉浗瀹跺晢鐢ㄥ瘑鐮佺鐞嗗姙鍏鍙戝竷鐨勫瘑鐮佹爣鍑嗏€︹€?|
| CRYPTO_STREEBOG | tristate | Streebog 鍝堝笇鍑芥暟锛圙OST R 34.11-2012, RFC 6986, ISO/IEC 10118-3锛夈€傝繖鏄縿缃楁柉瀵嗙爜鏍囧噯绠楁硶涔嬩竴锛堢О涓?GOST 绠楁硶锛夈€傛璁剧疆鍚敤涓ょ鍝堝笇绠楁硶鈥︹€?|
| CRYPTO_TEA | tristate | TEA锛堝井鍨嬪姞瀵嗙畻娉曪級瀵嗙爜绠楁硶銆俆iny Encryption Algorithm 鏄竴绉嶄娇鐢ㄥ杞互淇濊瘉瀹夊叏鐨勭畝鍗曞瘑鐮侊紝閫熷害鏋佸揩涓斿崰鐢ㄥ唴瀛樺緢灏戙€傛墿灞曞井鍨嬪姞瀵嗙畻娉曗€︹€?|
| CRYPTO_TWOFISH | tristate | Twofish 瀵嗙爜绠楁硶銆俆wofish 鐢?CounterPane Systems 鐨勭爺绌朵汉鍛樹綔涓?AES锛堥珮绾у姞瀵嗘爣鍑嗭級鍊欓€夊瘑鐮佹彁浜ゃ€傚畠鏄竴绉?16 杞垎缁勫瘑鐮侊紝鏀寔鈥︹€︾殑瀵嗛挜闀垮害 |
| CRYPTO_TWOFISH_COMMON | tristate | 鐢遍€氱敤 C 瀹炵幇涓庢眹缂栧疄鐜板叡浜殑 Twofish 瀵嗙爜绠楁硶鍏叡閮ㄥ垎銆?|
| CRYPTO_USER_API_AEAD | tristate | 鍚敤 AEAD 瀵嗙爜绠楁硶鐨勭敤鎴风┖闂存帴鍙ｃ€傝鍙傞槄 Documentation/crypto/userspace-if.rst 涓?https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_ENABLE_OBSOLETE | bool | 鍏佽閫夋嫨閭ｄ簺宸蹭粠鍐呮牳鍐呴儴浣跨敤涓窐姹般€佷粎瀵逛粛渚濊禆瀹冧滑鐨勭敤鎴风┖闂村鎴风鏈夌敤鐨勮繃鏃跺姞瀵嗙畻娉曘€?|
| CRYPTO_USER_API_RNG | tristate | 鍚敤 RNG锛堥殢鏈烘暟鐢熸垚鍣級绠楁硶鐨勭敤鎴风┖闂存帴鍙ｃ€傝鍙傞槄 Documentation/crypto/userspace-if.rst 涓?https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_RNG_CAVP | bool | 鍦ㄧ敤鎴风┖闂存帴鍙ｄ腑鍚敤棰濆鐨?API锛岀敤浜?NIST CAVP锛堝姞瀵嗙畻娉曢獙璇佺▼搴忥級娴嬭瘯锛? 閲嶇疆 DRBG 鐔?- 鎻愪緵闄勫姞鏁版嵁銆傛閫夐」鍙簲鈥︹€?|
| CRYPTO_USER_API_SKCIPHER | tristate | 鍚敤瀵圭О瀵嗛挜瀵嗙爜绠楁硶鐨勭敤鎴风┖闂存帴鍙ｃ€傝鍙傞槄 Documentation/crypto/userspace-if.rst 涓?https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_WP512 | tristate | Whirlpool 鍝堝笇鍑芥暟锛圛SO/IEC 10118-3锛夛紝鏀寔 512銆?84 涓?256 浣嶅搱甯屻€俉hirlpool-512 鏄?NESSIE 瀵嗙爜鍘熻鐨勪竴閮ㄥ垎銆傚弬瑙?https://web.archive.org/web/20171129084214/http://www.larc.u鈥︹€?|
| CRYPTO_XCBC | tristate | XCBC-MAC锛堟墿灞曞瘑鐮佸垎缁勯摼鎺ユ秷鎭璇佺爜锛夛紙RFC3566锛?|
| CRYPTO_XCTR | tristate | 鐢ㄤ簬 HCTR2 鐨?XCTR锛圶OR 璁℃暟鍣級妯″紡銆傝鍒嗙粍瀵嗙爜妯″紡鏄?CTR 妯″紡鐨勪竴绉嶅彉浣擄紝浣跨敤 XOR 涓庡皬绔姞娉曡€岄潪澶х杩愮畻銆俋CTR 妯″紡鐢ㄤ簬瀹炵幇 HCTR2銆?|
| CRYPTO_XTS | tristate | XTS锛堝甫瀵嗘枃绐冨彇鐨?XOR 鍔犲瘑 XOR锛夋ā寮忥紙NIST SP800-38E 涓?IEEE 1619锛夈€傞厤鍚?aes-xts-plain 浣跨敤锛屽瘑閽ラ暱搴︿负 256銆?84 鎴?512 浣嶃€傛瀹炵幇褰撳墠鏃犳硶澶勭悊鈥︹€︾殑鎵囧尯澶у皬 |
| CRYPTO_XXHASH | tristate | xxHash 闈炲姞瀵嗗搱甯岀畻娉曘€傞€熷害鏋佸揩锛屾帴杩?RAM 鏋侀檺銆?|
| CRYPTO_ZSTD | tristate | zstd 鍘嬬缉绠楁硶銆傛洿澶氫俊鎭鍙傞槄 https://github.com/facebook/zstd銆?|
| CSD_LOCK_WAIT_DEBUG | bool | 褰?CPU 瀵?smp_call_function*() IPI 灏佽鍝嶅簲缂撴參鏃讹紝姝ら€夐」鍚敤璋冭瘯鎵撳嵃銆傝繖浜涜皟璇曟墦鍗板寘鍚綋鍓嶆鍦ㄦ墽琛岀殑 IPI 澶勭悊鍑芥暟锛堣嫢鏈夛級鍙婄浉鍏崇殑鈥︹€?|
| CSD_LOCK_WAIT_DEBUG_DEFAULT | bool | 姝ら€夐」浣?csdlock_debug= 鍐呮牳寮曞鍙傛暟榛樿涓?1锛堝熀鏈皟璇曪級鑰岄潪 0锛堟棤璋冭瘯锛夈€?|
| DCACHE_WORD_ACCESS | bool | 鍚敤姝ら」鍙湪鏂囦欢绯荤粺娉ㄥ唽鏃跺鍏跺弬鏁版弿杩拌繘琛屾牎楠屻€?|
| DEBUG_ATOMIC | bool | 鑻ュ湪姝ら€?Y锛屽唴鏍稿皢涓哄師瀛愯闂坊鍔犺繍琛屾椂瀵归綈妫€鏌ャ€傚浜庝笉瀵规湭瀵归綈璁块棶浜х敓闄烽槺鐨勪綋绯荤粨鏋勫緢鏈夌敤銆傛閫夐」鍙兘甯︽潵鏄捐憲鐨勨€︹€?|
| DEBUG_ATOMIC_LARGEST_ALIGN | bool | 鑻ュ湪姝ら€?Y锛屽垯瀵瑰師瀛愯闂嚜鐒跺榻愮殑妫€鏌ュ皢琚檺鍒朵负缂栬瘧鍣ㄥ鏍囬噺绫诲瀷鐨勬渶澶у榻愩€?|
| DEBUG_ATOMIC_SLEEP | bool | 鑻ュ湪姝ら€?Y锛屽悇绫诲彲鑳戒紤鐪犵殑渚嬬▼鑻ュ湪鍐呮牳鍘熷瓙娈靛唴琚皟鐢紙鎸佹湁鑷棆閿佹椂銆佸浜?rcu 璇荤涓寸晫鍖烘椂銆佸浜庢姠鍗犫€︹€︽椂锛夊皢浜х敓澶ч噺鍛婅銆?|
| DEBUG_BUGVERBOSE | bool | 鍦ㄦ閫?Y 鍙娇 BUG() panic 杈撳嚭 BUG 璋冪敤鐨勬枃浠跺悕涓庤鍙凤紝浠ュ強 EIP 涓?oops 璺熻釜銆傝繖鏈夊姪浜庤皟璇曪紝浣嗕細鍗犵敤绾?70-100K 鍐呭瓨銆?|
| DEBUG_BUGVERBOSE_DETAILED | bool | 鍦ㄦ閫?Y 鍙娇 WARN_ON_ONCE() 闄ゆ枃浠跺悕涓庤鍙峰锛岃繕杈撳嚭璀﹀憡鐨勬潯浠跺瓧绗︿覆銆傝繖鏈夊姪浜庤皟璇曪紝浣嗗崰鐢ㄧ害 100K 鍐呭瓨銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_CGROUP_REF | bool | 寮哄埗 cgroup css 寮曠敤璁℃暟鍑芥暟涓嶈鍐呰仈锛屼互渚垮彲閫氳繃 kprobe 杩涜璋冭瘯銆?|
| DEBUG_CLOSURES | bool | 灏嗘墍鏈夋椿璺冪殑 closure 淇濆瓨鍦ㄩ摼琛ㄤ腑锛屽苟鎻愪緵 debugfs 鎺ュ彛鍒楀嚭瀹冧滑锛屼粠鑰屽彲浠ユ煡鐪嬪崱浣忕殑寮傛鎿嶄綔銆?|
| DEBUG_FORCE_FUNCTION_ALIGN_64B | bool | 瀛樺湪杩欐牱鐨勬儏鍐碉細鏉ヨ嚜鏌愪釜棰嗗煙鐨勬彁浜ゆ敼鍙樹簡鍏朵粬棰嗗煙鐨勫嚱鏁板湴鍧€瀵归綈锛屽鑷寸濂囩殑鎬ц兘娉㈠姩锛堝洖褰掓垨鎻愬崌锛夈€傚惎鐢ㄦ閫夐」鏈夊姪浜庘€︹€?|
| DEBUG_FORCE_WEAK_PER_CPU | bool | s390 涓?alpha 瑕佹眰妯″潡涓殑 percpu 鍙橀噺瀹氫箟涓哄急绗﹀彿锛屼互瑙勯伩瀵诲潃鑼冨洿闂锛岃繖缁?percpu 鍙橀噺瀹氫箟甯︽潵浠ヤ笅涓ゆ潯闄愬埗锛?. percpu 绗﹀彿鈥︹€?|
| DEBUG_FS | bool | debugfs 鏄唴鏍稿紑鍙戣€呯敤鏉ユ斁缃皟璇曟枃浠剁殑铏氭嫙鏂囦欢绯荤粺銆傚惎鐢ㄦ閫夐」浠ヤ究鑳藉璇诲啓杩欎簺鏂囦欢銆傛湁鍏?debugfs 鐨勮缁嗘枃妗ｂ€︹€?|
| DEBUG_FS_ALLOW_ALL | bool | 涓嶆柦鍔犱换浣曢檺鍒躲€侫PI 涓庢枃浠剁郴缁熸敞鍐屽潎寮€鍚€傝繖鏄甯哥殑榛樿鎿嶄綔銆?|
| DEBUG_FS_ALLOW_NONE | bool | 绂佺敤璁块棶銆傚鎴风灏濊瘯鍦?debugfs 鏍戜腑鍒涘缓鑺傜偣鏃朵細鏀跺埌 -PERM锛屼笖 debugfs 涓嶄細琚敞鍐屼负鏂囦欢绯荤粺銆傚鎴风闅忓悗鍙€€閬挎垨鍦ㄦ病鏈?debugfs 璁块棶鐨勬儏鍐典笅缁х画銆?|
| DEBUG_HIGHMEM | bool | 姝ら€夐」涓洪珮鍐呭瓨绯荤粺鍚敤棰濆鐨勯敊璇鏌ャ€傜敓浜х郴缁熷簲绂佺敤銆?|
| DEBUG_INFO | bool | 鍦ㄤ笅闈㈢殑鈥滆皟璇曚俊鎭€濋€夋嫨涓凡閫変腑鈥淣one鈥濅互澶栫殑鍐呮牳璋冭瘯淇℃伅閫夐」锛岃〃绀哄皢涓烘瀯寤虹洰鏍囩敓鎴愯皟璇曚俊鎭€? Clang 鐢熸垚 .ule鈥︹€?|
| DEBUG_INFO_BTF | bool | 浠?DWARF 璋冭瘯淇℃伅鐢熸垚鍘婚噸鍚庣殑 BTF 绫诲瀷淇℃伅銆傚紑鍚畠闇€瑕?pahole v1.22 鎴栨洿楂樼増鏈紝瀹冧細灏?DWARF 绫诲瀷淇℃伅杞崲涓虹瓑鏁堢殑鍘婚噸 BTF 绫诲瀷淇℃伅銆?|
| DEBUG_INFO_BTF_MODULES | bool | 涓哄唴鏍告ā鍧楃敓鎴愮揣鍑戠殑鎷嗗垎 BTF 绫诲瀷淇℃伅銆?|
| DEBUG_INFO_COMPRESSED_NONE | bool | 涓嶅帇缂╄皟璇曚俊鎭銆?|
| DEBUG_INFO_COMPRESSED_ZLIB | bool | 浣跨敤 zlib 鍘嬬缉璋冭瘯淇℃伅銆傞€氳繃 debian/rules 浣跨敤 dpkg-deb 鐨勭敤鎴峰彲鑳戒細鍙戠幇鍏惰皟璇?.deb 鍖呬綋绉洜璋冭瘯淇℃伅琚帇缂╄€屽澶р€︹€?|
| DEBUG_INFO_COMPRESSED_ZSTD | bool | 浣跨敤 zstd 鍘嬬缉璋冭瘯淇℃伅銆傚湪鐩歌繎鐨勬椂闂村紑閿€涓嬶紝瀹冩瘮 zlib 鎻愪緵鏇村ソ鐨勫帇缂╃巼锛屼絾闇€瑕佽緝鏂扮殑宸ュ叿閾炬敮鎸併€傞渶瑕?GCC 13.0+ 鎴?Clang 16.0+鈥︹€?|
| DEBUG_INFO_DWARF4 | bool | 鐢熸垚 DWARF v4 璋冭瘯淇℃伅銆傝繖闇€瑕?gcc 4.5+銆佽嫢浣跨敤涓嶅甫 clang 闆嗘垚姹囩紪鍣ㄧ殑 clang 鍒欓渶瑕?binutils 2.35.2銆佷互鍙?gdb 7.0+銆傝嫢浣犳湁灏氭湭鍑嗗濂解€︹€︾殑 DWARF 璋冭瘯淇℃伅娑堣垂鑰?|
| DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT | bool | 宸ュ叿閾剧敓鎴愮殑 DWARF 璋冭瘯淇℃伅鐨勯殣寮忛粯璁ょ増鏈細闅忔椂闂村彉鍖栥€傝繖鍙兘鐮村潖灏氭湭鍗囩骇浠ユ敮鎸佹柊鐗堟湰鐨勮皟璇曚俊鎭秷璐硅€咃紝骞堕樆姝⑩€︹€?|
| DEBUG_INFO_NONE | bool | 鏋勫缓鍐呮牳鏃朵笉鍖呭惈璋冭瘯淇℃伅锛屼粠鑰岀敓鎴愭洿蹇笖鏇村皬鐨勬瀯寤恒€?|
| DEBUG_INFO_SPLIT | bool | 灏嗚皟璇曚俊鎭敓鎴愬埌鐙珛鐨?.dwo 鏂囦欢涓€傝繖鏄捐憲鍑忓皬浜嗗甫 DEBUG_INFO 鏋勫缓鐨勬瀯寤虹洰褰曚綋绉紝鍥犱负瀹冨彧鍦ㄧ鐩樹笂鐨?.dwo 鏂囦欢涓瓨鍌ㄤ竴娆′俊鎭紝鑰岄潪鈥︹€?|
| DEBUG_IRQFLAGS | bool | 鍚敤瀵瑰彲鑳戒笉瀹夊叏鐨勪腑鏂惎鐢?绂佺敤鎿嶄綔鐨勬鏌ワ紝渚嬪鍦ㄤ腑鏂凡鍚敤鏃惰皟鐢?raw_local_irq_restore()銆?|
| DEBUG_KERNEL | bool | 鑻ヤ綘姝ｅ湪寮€鍙戦┍鍔ㄦ垨灏濊瘯璋冭瘯骞跺畾浣嶅唴鏍搁棶棰橈紝鍦ㄦ閫?Y銆?|
| DEBUG_KMAP_LOCAL | bool | 姝ら€夐」涓?kmap_local 鍩虹璁炬柦鍚敤棰濆鐨勯敊璇鏌ャ€傜敓浜х幆澧冨簲绂佺敤銆?|
| DEBUG_KOBJECT | bool | 鑻ュ湪姝ら€?Y锛屼竴浜涢澶栫殑 kobject 璋冭瘯娑堟伅灏嗚鍙戦€佸埌 syslog銆?|
| DEBUG_KOBJECT_RELEASE | bool | kobject 鏄紩鐢ㄨ鏁扮殑瀵硅薄銆傝繖鎰忓懗鐫€瀹冧滑鐨勬渶鍚庝竴娆″紩鐢ㄨ鏁伴噴鏀炬槸涓嶅彲棰勬祴鐨勶紝涓?kobject 鍙兘鍦ㄩ┍鍔ㄥ喅瀹氫涪寮冨叾鍒濆鈥︹€︿箣鍚庣户缁瓨娲?|
| DEBUG_LOCKDEP | bool | 鑻ュ湪姝ら€?Y锛岄攣渚濊禆寮曟搸灏嗘墽琛岄澶栫殑杩愯鏃舵鏌ヤ互鑷垜璋冭瘯锛屼唬浠锋槸鏇村杩愯鏃跺紑閿€銆?|
| DEBUG_LOCKING_API_SELFTESTS | bool | 鑻ュ笇鏈涘唴鏍稿湪寮曞鏃惰繍琛屼竴娈电畝鐭殑鑷娴嬭瘯锛屽湪姝ら€?Y銆傝鑷娴嬭瘯浼氭鏌ュ父瑙佺被鍨嬬殑鍔犻攣缂洪櫡鏄惁鑳借璋冭瘯鏈哄埗妫€娴嬪埌锛堣嫢浣犵鐢ㄩ攣鈥︹€?|
| DEBUG_LOCK_ALLOC | bool | 璇ョ壒鎬у皢妫€鏌ヤ换浣曡鎸佹湁鐨勯攣锛堣嚜鏃嬮攣銆乺wlock銆佷簰鏂ヤ綋鎴?rwsem锛夋槸鍚﹁鍐呮牳閫氳繃浠讳竴鍐呭瓨閲婃斁渚嬬▼锛坘free()銆乲mem_cache_free()銆乫ree_pages()鈥︹€︼級閿欒鍦伴噴鏀?|
| DEBUG_MAPLE_TREE | bool | 鍚敤 maple tree 璋冭瘯淇℃伅涓庨澶栭獙璇併€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_MEMORY_INIT | bool | 鍚敤姝ら」浠ュ湪鍐呭瓨鍒濆鍖栨湡闂磋繘琛岄澶栨鏌ャ€傚仴鍏ㄦ€ф鏌ヤ細鏍￠獙 VM 鐨勫悇涓柟闈紝渚嬪鍐呭瓨妯″瀷浠ュ強浣撶郴缁撴瀯鎻愪緵鐨勫叾浠栦俊鎭€傝缁嗕俊鈥︹€?|
| DEBUG_MISC | bool | 鑻ヤ綘闇€瑕佸惎鐢ㄦ湰搴斿綊灞炴煇涓洿鍏蜂綋鐨勮皟璇曢€夐」銆佷絾瀹為檯涓婂苟鏈綊绫荤殑鏉傞」璋冭瘯浠ｇ爜锛屽湪姝ら€?Y銆?|
| DEBUG_MUTEXES | bool | 璇ョ壒鎬у厑璁告娴嬪苟鎶ュ憡瀵逛簰鏂ヤ綋璇箟鐨勮繚鍙嶃€?|
| DEBUG_NOMMU_REGIONS | bool | 姝ら€夐」浣垮尶鍚嶄笌绉佹湁鏄犲皠鍖哄煙鐨勫叏灞€鏍戣瀹氭湡妫€鏌ユ槸鍚﹀瓨鍦ㄦ棤鏁堟嫇鎵戙€?|
| DEBUG_NOTIFIERS | bool | 鍚敤姝ら」浠ュ紑鍚閫氱煡閾撅紙notifier call chain锛夌殑鍋ュ叏鎬ф鏌ャ€傝繖瀵瑰唴鏍稿紑鍙戣€呯‘淇濇ā鍧楁纭湴浠庨€氱煡閾炬敞閿€鏈€涓烘湁鐢ㄣ€傝繖鏄€︹€?|
| DEBUG_OBJECTS | bool | 鑻ュ湪姝ら€?Y锛屽唴鏍镐腑灏嗘彃鍏ラ澶栦唬鐮佷互璺熻釜鍚勭被瀵硅薄鐨勭敓鍛藉懆鏈燂紝骞堕獙璇侀拡瀵硅繖浜涘璞＄殑鎿嶄綔銆?|
| DEBUG_OBJECTS_ENABLE_DEFAULT | int | 璋冭瘯瀵硅薄鐨勫紩瀵煎弬鏁伴粯璁ゅ€?|
| DEBUG_OBJECTS_FREE | bool | 杩欏惎鐢ㄦ鏌ワ細k/v 閲婃斁鎿嶄綔鏄惁閲婃斁浜嗗寘鍚皻鏈纭幓婵€娲诲璞＄殑鍖哄煙銆傝繖鍙兘浣?kmalloc/kfree 瀵嗛泦鍨嬪伐浣滆礋杞芥槑鏄惧彉鎱€?|
| DEBUG_OBJECTS_PERCPU_COUNTER | bool | 鑻ュ湪姝ら€?Y锛屽皢鍦?percpu 璁℃暟鍣ㄤ緥绋嬩腑鎻掑叆棰濆浠ｇ爜锛屼互璺熻釜 percpu 璁℃暟鍣ㄥ璞＄殑鐢熷懡鍛ㄦ湡骞堕獙璇?percpu 璁℃暟鍣ㄦ搷浣溿€?|
| DEBUG_OBJECTS_RCU_HEAD | bool | 鍚敤姝ら」浠ュ紑鍚 RCU 閾捐〃澶达紙call_rcu() 鐢ㄦ硶锛夌殑璋冭瘯銆?|
| DEBUG_OBJECTS_SELFTEST | bool | 杩欏惎鐢ㄥ璞¤皟璇曚唬鐮佺殑鑷娴嬭瘯銆?|
| DEBUG_OBJECTS_TIMERS | bool | 鑻ュ湪姝ら€?Y锛屽皢鍦ㄥ畾鏃跺櫒渚嬬▼涓彃鍏ラ澶栦唬鐮侊紝浠ヨ窡韪畾鏃跺櫒瀵硅薄鐨勭敓鍛藉懆鏈熷苟楠岃瘉瀹氭椂鍣ㄦ搷浣溿€?|
| DEBUG_OBJECTS_WORK | bool | 鑻ュ湪姝ら€?Y锛屽皢鍦ㄥ伐浣滈槦鍒椾緥绋嬩腑鎻掑叆棰濆浠ｇ爜锛屼互璺熻釜宸ヤ綔瀵硅薄鐨勭敓鍛藉懆鏈熷苟楠岃瘉宸ヤ綔鎿嶄綔銆?|
| DEBUG_PERF_USE_VMALLOC | bool | 浣跨敤 vmalloc 鍐呭瓨浣滀负 perf mmap() 缂撳啿鍖虹殑鍚庡銆備富瑕佺敤浜庡湪涓嶈姹傚畠鐨勫钩鍙颁笂璋冭瘯 vmalloc 浠ｇ爜銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_PER_CPU_MAPS | bool | 閫?Y 浠ラ獙璇佽璁块棶鐨?per_cpu 鏄犲皠宸插缓绔嬨€傝繖浼氱粰鍐呮牳鍐呭瓨澧炲姞鐩稿綋澶氱殑浠ｇ爜骞堕檷浣庢€ц兘銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_PLIST | bool | 鍚敤姝ら」浠ュ紑鍚鎸変紭鍏堢骇鎺掑簭鐨勯摼琛紙plist锛夐亶鍘嗕緥绋嬬殑鎵╁睍妫€鏌ャ€傚畠浼氬湪姣忔鎿嶄綔鏃跺娆℃鏌ユ暣涓摼琛ㄣ€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_PREEMPT | bool | 鑻ュ湪姝ら€?Y锛屽唴鏍稿皢浣跨敤甯哥敤 smp_processor_id() 鍑芥暟鐨勮皟璇曞彉浣擄紝骞跺湪鍐呮牳浠ｇ爜浠ヤ笉瀹夊叏浜庢姠鍗犵殑鏂瑰紡浣跨敤瀹冩椂鎵撳嵃璀﹀憡銆傛澶栵紝鍐呮牳鈥︹€?|
| DEBUG_RSEQ | bool | 涓?rseq 绯荤粺璋冪敤鍚敤棰濆鐨勮皟璇曟鏌ャ€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_RT_MUTEXES | bool | 杩欏厑璁歌嚜鍔ㄦ娴嬪苟鎶ュ憡瀵?rt 浜掓枼浣撹涔夌殑杩濆弽锛屼互鍙?rt 浜掓枼浣撶浉鍏崇殑姝婚攣锛坙ockup锛夈€?|
| DEBUG_RWSEMS | bool | 璇ヨ皟璇曠壒鎬у厑璁告娴嬪苟鎶ュ憡涓嶅尮閰嶇殑璇诲啓淇″彿閲忓姞閿佷笌瑙ｉ攣銆?|
| DEBUG_SECTION_MISMATCH | bool | 娈典笉鍖归厤鍒嗘瀽妫€鏌ユ槸鍚﹀瓨鍦ㄤ粠涓€涓鍒板彟涓€涓鐨勯潪娉曞紩鐢ㄣ€傚湪閾炬帴鏃舵垨杩愯鏃讹紝鏌愪簺娈典細琚涪寮冿紱浠讳綍瀵硅繖浜涙涓厛鍓嶄唬鐮?鏁版嵁鐨勪娇鐢ㄢ€︹€?|
| DEBUG_SG | bool | 鍚敤姝ら」浠ュ紑鍚鏁ｅ垪鑱氶泦锛坰catter-gather锛夎〃鐨勬鏌ャ€傝繖鏈夊姪浜庡彂鐜版湭姝ｇ‘鍒濆鍖栧叾 sg 琛ㄧ殑椹卞姩闂銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_SHIRQ | bool | 鍚敤姝ら」浠ュ湪鍏变韩涓柇澶勭悊绋嬪簭娉ㄩ攢鍓嶇敓鎴愪竴涓吉涓柇锛堟敞鍐屾椂鐢熸垚鐩墠琚鐢級銆傞┍鍔ㄩ渶瑕佹纭鐞嗗畠銆傝嫢鈥︹€?|
| DEBUG_SPINLOCK | bool | 鍦ㄦ閫?Y 骞舵瀯寤?SMP锛屽彲鎹曡幏缂哄け鐨勮嚜鏃嬮攣鍒濆鍖栦互鍙婃煇浜涘叾浠栧父瑙佺殑鑷棆閿侀敊璇€傛渶濂戒笌 NMI 鐪嬮棬鐙楅厤鍚堜娇鐢紝浠ヤ究鑷棆閿佲€︹€?|
| DEBUG_STACK_USAGE | bool | 鍚敤鍦?sysrq-T 涓?sysrq-P 璋冭瘯杈撳嚭涓樉绀烘瘡涓换鍔℃浘缁忓彲鐢ㄧ殑鏈€灏忕┖闂叉爤绌洪棿銆傚綋杩涚▼閫€鍑烘椂锛岃嫢璇ヨ繘绋嬧€︹€﹁繕浼氬悜 dmesg 鍙戦€佷竴鏉℃秷鎭?|
| DEBUG_VFS | bool | 鍚敤姝ら」浠ュ紑鍚?VFS 灞備腑鍙兘褰卞搷鎬ц兘鐨勬墿灞曟鏌ャ€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_VM_IRQSOFF | def_bool | 鍚敤姝ら」浠ュ紑鍚櫄鎷熷唴瀛樼郴缁熶腑鍙兘褰卞搷鎬ц兘鐨勬墿灞曟鏌ャ€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_VM_MAPLE_TREE | bool | 鍚敤 VM maple tree 璋冭瘯淇℃伅涓庨澶栭獙璇併€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_VM_PGFLAGS | bool | 鍚敤瀵归〉鏍囧織鎿嶄綔鐨勯澶栭獙璇併€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_VM_PGTABLE | bool | 姝ら€夐」鎻愪緵涓€绉嶈皟璇曟柟娉曪紝鍙敤浜庡湪鍚勭骞冲彴涓婃祴璇曚綋绯荤粨鏋勭殑椤佃〃杈呭姪鍑芥暟锛岄獙璇佸叾鏄惁绗﹀悎棰勬湡鐨勯€氱敤 MM 璇箟銆傝繖灏嗏€︹€?|
| DEBUG_VM_RB | bool | 鍚敤 VM 绾㈤粦鏍戣皟璇曚俊鎭笌棰濆楠岃瘉銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_VM_SHOOT_LAZIES | bool | 鍚敤棰濆鐨?IPI锛岀‘淇?lazy tlb mm 寮曠敤鍦?mm 琚噴鏀惧墠琚Щ闄ゃ€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| DEBUG_WQ_FORCE_RR_CPU | bool | 宸ヤ綔闃熷垪鏇剧粡闅愬紡淇濊瘉锛氭湭鏄惧紡鎸囧畾 CPU 鐨勬帓闃熷伐浣滈」浼氳鏀惧埌鏈湴 CPU 涓娿€傝繖涓€淇濊瘉宸蹭笉鍐嶆垚绔嬶紝铏界劧鏈湴 CPU 浠嶆槸棣栭€夛紝浣嗗伐浣溾€︹€?|
| DEBUG_WW_MUTEX_SLOWPATH | bool | 璇ョ壒鎬ч€氳繃娉ㄥ叆棰濆鐨?-EDEADLK 鍥為€€锛坵ound/backoff锛夌敤渚嬶紝涓?w/w 浜掓枼浣撲娇鐢ㄨ€呭惎鐢ㄦ參閫熻矾寰勬祴璇曘€傞厤鍚堬紙CONFIG_PROVE_LOCKING锛夊惎鐢ㄧ殑瀹屾暣浜掓枼浣撴鏌ワ紝杩欏皢娴嬭瘯鈥︹€?|
| DEFAULT_HOSTNAME | string | 姝ら€夐」鍐冲畾鍦ㄧ敤鎴风┖闂磋皟鐢?sethostname(2) 涔嬪墠鐨勯粯璁ょ郴缁熶富鏈哄悕銆傚唴鏍镐紶缁熶笂鍦ㄦ浣跨敤鈥?none)鈥濓紝浣嗕綘鍙兘甯屾湜鍦ㄦ浣跨敤涓€涓笉鍚岄粯璁ゅ€间互鐢熸垚鏈€灏忊€︹€?|
| DEFAULT_HUNG_TASK_TIMEOUT | int | 姝ら€夐」鎺у埗鐢ㄤ簬鍒ゆ柇浠诲姟浣曟椂鍙樺緱鏃犲搷搴斿苟搴旇瑙嗕负鎸傝捣鐨勯粯璁よ秴鏃讹紙绉掞級銆傚畠鍙湪杩愯鏃堕€氳繃 kernel.hung_task_t鈥︹€﹁皟鏁?|
| DEFAULT_INIT | string | 鑻ュ唴鏍稿懡浠よ鏈紶鍏?init= 閫夐」锛屾閫夐」鍐冲畾绯荤粺鐨勯粯璁?init銆傝嫢鎵€璇锋眰鐨勮矾寰勪笉瀛樺湪锛屾垜浠粛浼氱户缁皾璇曡繘涓€姝ョ殑鈥︹€?|
| DEFAULT_MMAP_MIN_ADDR | int | 杩欐槸搴斿綋琚繚鎶や互鍏嶅彈鐢ㄦ埛绌洪棿鍒嗛厤鐨勪綆铏氭嫙鍐呭瓨閮ㄥ垎銆傞樆姝㈢敤鎴峰啓鍏ヤ綆鍦板潃椤垫湁鍔╀簬闄嶄綆鍐呮牳绌烘寚閽堢己闄风殑褰卞搷銆傛湁鍏斥€︹€?|
| DEFAULT_SECURITY_SELINUX | bool | 浠ラ€楀彿鍒嗛殧鐨?LSM 鍒楄〃锛屾寜鍒濆鍖栭『搴忋€備换浣曟湭鍒楀叆姝ゅ垪琛ㄧ殑 LSM锛堥櫎閭ｄ簺鍏锋湁 LSM_ORDER_FIRST 涓?LSM_ORDER_LAST 椤哄簭銆佸湪鈥︹€﹂€変腑鏃舵€绘槸鍚敤鐨勯櫎澶栵級鈥︹€?|
| DEFERRED_STRUCT_PAGE_INIT | bool | 閫氬父鎵€鏈?struct page 閮藉湪鏃╂湡寮曞鏈熼棿浠ュ崟绾跨▼鏂瑰紡鍒濆鍖栥€傚湪闈炲父澶х殑鏈哄櫒涓婏紝杩欏彲鑳借€楄垂鐩稿綋澶氭椂闂淬€傝嫢璁剧疆姝ら€夐」锛屽ぇ鍨嬫満鍣ㄥ皢鈥︹€?|
| DETECT_HUNG_TASK | bool | 鍦ㄦ閫?Y 浠ヨ鍐呮牳妫€娴嬧€渉ung tasks锛堟寕璧蜂换鍔★級鈥濓紝鍗冲鑷翠换鍔℃棤闄愭湡鍗″湪涓嶅彲涓柇鈥淒鈥濈姸鎬佺殑缂洪櫡銆傚綋妫€娴嬪埌鎸傝捣浠诲姟鏃讹紝鍐呮牳灏嗘墦鍗扳€︹€?|
| DETECT_HUNG_TASK_BLOCKER | bool | 鍦ㄦ閫?Y 浠ユ樉绀鸿幏鍙栦簡鈥渉ung tasks鈥濇鍦ㄧ瓑寰呯殑浜掓枼閿佺殑闃诲浠诲姟鐨勬爤璺熻釜銆傝繖浼氬鍔犲皯閲忓紑閿€锛屼絾鑳芥樉绀哄彲鐤戜换鍔″強鍏惰皟鐢ㄨ窡韪紙鑻ュ叾鏉ヨ嚜鈥︹€︼級 |
| DIMLIB | tristate | 鍔ㄦ€佷腑鏂皟鑺傚簱銆傚疄鐜颁竴绉嶆牴鎹繍琛屾椂鎬ц兘鍔ㄦ€佹敼鍙?CQ 璋冭妭鍊肩殑绠楁硶銆? # libfdt 鏂囦欢锛屼粎鍦ㄩ渶瑕佹椂閫変腑銆? |
| DST_CACHE | bool | NET_SOCK_MSG 涓烘櫘閫氬鎺ュ瓧锛堝 TCP锛夋垨 ULP锛堜笂灞傛ā鍧楋紝濡?TLS锛夋彁渚涗竴涓€熷姪 BPF 绋嬪簭澶勭悊 L7 搴旂敤鏁版嵁鐨勬鏋躲€?|
| DYNAMIC_DEBUG | bool | 灏嗚皟璇曠骇鍒秷鎭紪璇戣繘鍐呮牳锛屽惁鍒欒繖浜涙秷鎭湪杩愯鏃朵笉鍙敤銆傞殢鍚庡彲鍩轰簬涓嶅悓浣滅敤鍩熺骇鍒紙鎸夋簮鏂囦欢銆佸嚱鏁扳€︹€︼級鍚敤/绂佺敤杩欎簺娑堟伅銆?|
| DYNAMIC_DEBUG_CORE | bool | 鍚敤 dynamic debug 鐨勬牳蹇冨姛鑳芥敮鎸併€傚綋浣犲笇鏈涘皢 dynamic debug 涓庝綘涓烘瘡涓ā鍧楀畾涔夌殑 DYNAMIC_DEBUG_MODULE 鍏宠仈鍒板唴鏍告ā鍧楁椂寰堟湁鐢紝灏ゅ叾閫傜敤浜庘€︹€︾殑鎯呭喌 |
| ELFCORE | bool | 姝ら€夐」鍚敤 kernel/elfcore.o銆?|
| ELF_CORE | bool | 鍚敤瀵圭敓鎴愭牳蹇冭浆鍌ㄧ殑鏀寔銆傜鐢ㄥ彲鑺傜渷绾?4k銆?|
| ETHTOOL_NETLINK | bool | 鍩轰簬閫氱敤 netlink 鐨?ethtool 鏇夸唬鎬х敤鎴风┖闂存帴鍙ｃ€傚畠鎻愪緵鏇村ソ鐨勫彲鎵╁睍鎬т互鍙婁竴浜涙柊鐗规€э紝濡傞€氱煡娑堟伅銆?|
| EVENTFD | bool | 鍚敤 eventfd() 绯荤粺璋冪敤锛屽畠鍏佽鎺ユ敹鍐呮牳閫氱煡锛堝 KAIO锛夋垨鐢ㄦ埛绌洪棿閫氱煡銆傝嫢涓嶇‘瀹氾紝閫?Y銆?|
| EXEC_KUNIT_TEST | bool | 鏋勫缓 exec 鐨?KUnit 娴嬭瘯锛屾祴璇?exec 鍐呴儴鍚勬柟闈㈣竟鐣屾潯浠躲€?|
| EXT_GROUP_SCHED | bool | 璇ョ壒鎬ц璋冨害鍣ㄥ熀浜庡綋鍓嶅湪璇?CPU 涓婂彲璋冨害鐨?RUNNABLE 浠诲姟璺熻釜姣忎釜 CPU 鐨勯挸浣嶅埄鐢ㄧ巼銆傚惎鐢ㄦ閫夐」鍚庯紝鐢ㄦ埛鍙寚瀹氭渶灏忎笌鈥︹€?|
| FAILOVER | tristate | failover 妯″潡涓哄崐铏氭嫙鍖栭┍鍔ㄦ彁渚涗竴涓€氱敤鎺ュ彛锛岀敤浜庡悜 failover 瀹炰緥娉ㄥ唽涓€涓?netdev 涓庝竴缁勬搷浣溿€傝繖浜涙搷浣滀綔涓轰簨浠跺鐞嗙▼搴忚璋冪敤鏉ュ鐞嗏€︹€?|
| FAILSLAB | bool | 涓?kmalloc 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FAIL_FUNCTION | bool | 鎻愪緵鍩轰簬鍑芥暟鐨勬晠闅滄敞鍏ヨ兘鍔涖€傝繖灏嗗厑璁镐綘鐢ㄧ粰瀹氳繑鍥炲€肩殑杩斿洖鏉ヨ鐩栫壒瀹氬嚱鏁般€傜粨鏋滐紝鍑芥暟璋冪敤鑰呭皢鐪嬪埌涓€涓敊璇€尖€︹€?|
| FAIL_FUTEX | bool | 涓?futex 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FAIL_IO_TIMEOUT | bool | 鍦ㄧ IO 澶勭悊涓婃彁渚涙晠闅滄敞鍏ヨ兘鍔涖€傝繖灏嗕娇鍧楀眰鈥滈仐蹇樷€濅竴涓寜閰嶇疆璁惧畾鐨勪腑鏂紝浠庤€屾紨缁冮敊璇鐞嗐€備粎閫傜敤浜庝娇鐢?g鈥︹€︾殑椹卞姩 |
| FAIL_MAKE_REQUEST | bool | 涓虹鐩?IO 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FAIL_MMC_REQUEST | bool | 涓?MMC IO 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆傝繖灏嗕娇 mmc 鏍稿績杩斿洖鏁版嵁閿欒銆傝繖鏈夊姪浜庢祴璇?mmc 鍧楄澶囦腑鐨勯敊璇鐞嗭紝浠ュ強娴嬭瘯 mmc 涓绘満椹卞姩鈥︹€?|
| FAIL_PAGE_ALLOC | bool | 涓?alloc_pages() 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FAIL_SKB_REALLOC | bool | 鎻愪緵寮哄埗閲嶆柊鍒嗛厤 skb 鐨勬晠闅滄敞鍏ヨ兘鍔涳紝浠ユ崟鑾峰彲鑳界殑 skb 鏃犳晥鎸囬拡銆傛洿澶氫俊鎭鍙傞槄 Documentation/fault-injection/fault-injection.rst |
| FAIL_SUNRPC | bool | 涓?SunRPC 鍙婂叾娑堣垂鑰呮彁渚涙晠闅滄敞鍏ヨ兘鍔涖€?|
| FAULT_INJECTION | bool | 鎻愪緵鏁呴殰娉ㄥ叆妗嗘灦銆傛洿澶氱粏鑺傝鍙傞槄 Documentation/fault-injection/銆?|
| FAULT_INJECTION_CONFIGFS | bool | 姝ら€夐」鍏佽鍩轰簬 configfs 鐨勯┍鍔ㄩ€氳繃 configfs 鍔ㄦ€侀厤缃晠闅滄敞鍏ャ€傛瘡涓┍鍔ㄧ壒瀹氱殑鏁呴殰娉ㄥ叆鍙傛暟鍙綔涓?configfs 灞炴€ф樉绀哄湪涓€涓€︹€?|
| FAULT_INJECTION_DEBUG_FS | bool | 閫氳繃 debugfs 鍚敤鏁呴殰娉ㄥ叆鑳藉姏鐨勯厤缃€?|
| FAULT_INJECTION_STACKTRACE_FILTER | bool | 涓烘晠闅滄敞鍏ヨ兘鍔涙彁渚涙爤璺熻釜杩囨护鍣?|
| FAULT_INJECTION_USERCOPY | bool | 鎻愪緵鍦?usercopy 鍑芥暟锛坈opy_from_user()銆乬et_user()鈥︹€︼級涓敞鍏ュけ璐ョ殑鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FFS_KUNIT_TEST | tristate | 鏋勫缓閽堝 ffs 绯诲垪浣嶆搷浣滃嚱鏁帮紙鍖呮嫭 ffs()銆乢_ffs()銆乫ls()銆乢_fls()銆乫ls64() 涓?__ffs64()锛夌殑 KUnit 娴嬭瘯銆傝繖浜涙祴璇曢獙璇佹暟瀛︽纭€с€佽竟鐣屾儏鍐靛鐞嗏€︹€?|
| FHANDLE | bool | 鑻ュ湪姝ら€?Y锛岀敤鎴锋€佺▼搴忎究鑳藉皢鏂囦欢鍚嶆槧灏勪负鍙ユ焺锛岄殢鍚庡皢璇ュ彞鏌勭敤浜庝笉鍚岀殑鏂囦欢绯荤粺鎿嶄綔銆傝繖鍦ㄥ疄鐜扮敤鎴风┖闂存枃浠舵湇鍔♀€︹€︽椂寰堟湁鐢?|
| FIB_RULES | bool | 璇ョ壒鎬ф彁渚涗竴涓敮鎸?mpls 绛夎交閲忕骇闅ч亾鐨勫熀纭€璁炬柦銆傝交閲忕骇闅ч亾绔偣涓嶅叧鑱斾换浣?netdevice銆傞毀閬撳皝瑁呭弬鏁板瓨鍌ㄤ簬鈥︹€?|
| FILE_LOCKING | bool | 姝ら€夐」鍚敤鏍囧噯鏂囦欢閿佹敮鎸侊紝杩欐槸 NFS 绛夋枃浠剁郴缁熶互鍙?flock() 绯荤粺璋冪敤鎵€蹇呴渶鐨勩€傜鐢ㄦ閫夐」鍙妭鐪佺害 11k銆?|
| FIND_BIT_BENCHMARK | tristate | 鏋勫缓鈥渢est_find_bit鈥濇ā鍧楋紝鐢ㄤ簬娴嬮噺 find_*_bit() 鍑芥暟鐨勬€ц兘銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| FIND_BIT_BENCHMARK_RUST | tristate | 鏋勫缓鈥渇ind_bit_benchmark_rust鈥濇ā鍧椼€傚畠鏄竴涓井鍩哄噯娴嬭瘯锛屾祴閲忎笌 C 涓?find_*_bit() 鎿嶄綔瀵瑰簲鐨?Rust 鍑芥暟鐨勬€ц兘銆傚畠閬靛惊 FIND_BI鈥︹€?|
| FIND_NORMAL_PAGE | def_bool | 璇ヤ綋绯荤粨鏋勪娇鐢?lazy MMU 妯″紡銆傝繖鍏佽瀵?MMU 鐩稿叧浣撶郴缁撴瀯鐘舵€佺殑鏇存敼琚帹杩熷埌閫€鍑鸿妯″紡鏃惰繘琛屻€傝瑙?<linux/pgtable.h>銆?|
| FLATMEM_MANUAL | bool | 姝ら€夐」鏈€閫傚悎鍏锋湁骞冲潶鍦板潃绌洪棿鐨勯潪 NUMA 绯荤粺銆侳LATMEM 鍦ㄦ€ц兘涓庤祫婧愭秷鑰楁柟闈㈡槸鏈€鏈夋晥鐨勭郴缁燂紝瀵逛簬灏忊€︹€︿篃鏄渶浣抽€夐」 |
| FORCE_NR_CPUS | def_bool | 姝ら€夐」鎻愪緵鐢ㄤ簬绠€鍗曟枃鏈ā寮忓尮閰嶇殑 glob_match 鍑芥暟銆傚畠璧锋簮浜?ATA 浠ｇ爜浠ュ皢鐗瑰畾椹卞姩鍣ㄥ瀷鍙峰垪鍏ラ粦鍚嶅崟锛屼絾鍏朵粬璁惧椹卞姩绋嬪簭鍙兘涔熼渶瑕佺被浼尖€︹€?|
| FORTIFY_KUNIT_TEST | tristate | 鏋勫缓鐢ㄤ簬妫€鏌?FORTIFY_SOURCE 鍐呴儴鏈哄埗鐨勫崟鍏冩祴璇曪紝FORTIFY_SOURCE 琚?str*() 涓?mem*() 绯诲垪鍑芥暟浣跨敤銆傝娴嬭瘯 FORTIFY_SOURCE 鐨勮繍琛屾椂闄烽槺锛岃鍙傞槄 LKDTM 鐨勨€淔ORTIFY_*鈥濇祴璇曘€?|
| FPROBE_SANITY_TEST | bool | 姝ら€夐」灏嗗湪绯荤粺寮曞鏃跺惎鐢ㄥ fprobe 鐨勬祴璇曘€備細鎵ц涓€绯诲垪娴嬭瘯浠ラ獙璇?fprobe 宸ヤ綔姝ｅ父銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| FRAME_WARN | int | 鍛婄煡缂栬瘧鍣ㄥ湪鏋勫缓鏃跺姝ゅぇ灏忎互涓婄殑鏍堝抚鍙戝嚭璀﹀憡銆傝缃繃浣庝細瀵艰嚧澶ч噺璀﹀憡銆傝涓?0 鍒欑鐢ㄨ璀﹀憡銆?|
| FREEZER | def_bool |  |
| FS_DAX_PMD | bool | 姝ら€夐」鍚敤鏂囦欢绯荤粺鐨勫鍑烘搷浣滐紝浠ユ敮鎸佸閮ㄥ潡 IO銆?|
| FS_IOMAP | bool | 鐩存帴璁块棶锛圖AX锛夊彲鐢ㄤ簬鍐呭瓨鍚庡鐨勫潡璁惧銆傝嫢鍧楄澶囨敮鎸?DAX 涓旀枃浠剁郴缁熸敮鎸?DAX锛屼綘渚垮彲閬垮厤浣跨敤椤电紦瀛樻潵缂撳啿 I/O銆傚紑鍚€︹€?|
| FUNCTION_ERROR_INJECTION | bool | 灏嗘晠闅滄敞鍏ュ埌鍐呮牳涓敤 ALLOW_ERROR_INJECTION() 鏍囨敞鐨勫悇绉嶅嚱鏁颁腑銆侭PF 涔熷彲鑳戒慨鏀硅繖浜涘嚱鏁扮殑杩斿洖鍊笺€傝繖鏈夊姪浜庢祴璇曢敊璇矾寰勨€︹€?|
| FUTEX | bool | 绂佺敤姝ら€夐」灏嗗鑷村唴鏍稿湪鏋勫缓鏃朵笉鍖呭惈瀵光€渇ast userspace mutexes锛堝揩閫熺敤鎴风┖闂翠簰鏂ヤ綋锛夆€濈殑鏀寔銆傜敓鎴愮殑鍐呮牳鍙兘鏃犳硶姝ｇ‘杩愯鍩轰簬 glibc 鐨勫簲鐢ㄣ€?|
| FUTEX_PI | bool | 绂佺敤姝ら€夐」灏嗗鑷村唴鏍稿湪鏋勫缓鏃朵笉鍖呭惈瀵?epoll 绯诲垪绯荤粺璋冪敤鐨勬敮鎸併€?|
| GCD_KUNIT_TEST | tristate | 姝ら€夐」鍚敤閽堝 gcd() 鍑芥暟锛堣绠椾袱涓暟鐨勬渶澶у叕绾︽暟锛夌殑 KUnit 娴嬭瘯濂椾欢銆傝娴嬭瘯濂椾欢鍦ㄥ悇绉嶅満鏅笅楠岃瘉 gcd() 鐨勬纭€р€︹€?|
| GCOV_PROFILE_URING | bool | 鍦?io_uring 瀛愮郴缁熶笂鍚敤 GCOV 鎬ц兘鍒嗘瀽锛屼互渚胯繘琛屼唬鐮佽鐩栫巼娴嬭瘯銆傝嫢涓嶇‘瀹氾紝閫?N銆傛敞鎰忚繖灏嗗 io_uring 瀛愮郴缁熸€ц兘浜х敓璐熼潰褰卞搷鈥︹€?|
| GDB_SCRIPTS | bool | 杩欎細鍦ㄦ瀯寤虹洰褰曚腑鍒涘缓鍒?GDB 杈呭姪鑴氭湰鎵€闇€鐨勯摼鎺ャ€傝嫢浣犲皢 vmlinux 鍔犺浇鍒?gdb 涓紝杩欎簺杈呭姪鑴氭湰涔熶細琚?gdb 鑷姩瀵煎叆锛屽苟鎻愪緵棰濆鐨勫嚱鏁扳€︹€?|
| GENERIC_EARLY_IOREMAP | bool | 杩欐槸 32 浣嶇敤鎴疯繘绋嬫爤鍚戜笂澧為暱鏃讹紙鐩墠浠呭湪 parisc 浣撶郴缁撴瀯涓婏級鍏?VM 甯冨眬涓爤鐨勬渶澶уぇ灏忥紙浠ュ厗瀛楄妭璁★級锛屽綋 RLIMIT_STACK 纭檺鍒朵负鏃犻檺鍒舵椂銆傝嫢鈥︹€?|
| GENERIC_IOREMAP | bool |  |
| GLOB_KUNIT_TEST | tristate | 鍚敤姝ら€夐」浠ュ湪杩愯鏃舵祴璇?glob 鍑芥暟銆傝娴嬭瘯濂椾欢鍦ㄥ悇绉嶅満鏅紙鍖呮嫭杈圭晫鎯呭喌锛変笅楠岃瘉 glob_match() 鐨勬纭€с€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| GRACE_PERIOD | tristate | 鏌愪簺 NFS 鏈嶅姟鍣ㄦ敮鎸佷竴涓緟鍔╂€х殑 NFS LOCALIO 鍗忚锛屽畠骞堕潪 NFS 鍗忚鐨勫畼鏂圭粍鎴愰儴鍒嗐€傛閫夐」鍦ㄥ唴鏍哥殑 NFS 鏈嶅姟鍣ㄤ笌瀹㈡埛绔腑鍚敤瀵?LOCALIO 鍗忚鐨勬敮鎸佲€︹€?|
| GROUP_SCHED_WEIGHT | def_bool | 姝ら€夐」鍏佽鐢ㄦ埛鍦ㄥ叕骞崇粍璋冨害鍣ㄤ腑涓鸿繍琛岀殑浠诲姟瀹氫箟 CPU 甯﹀閫熺巼锛堥檺鍒讹級銆傛湭璁剧疆闄愬埗鐨勭粍琚涓烘棤绾︽潫锛屽皢浠ユ棤鈥︹€︾殑鏂瑰紡杩愯 |
| GUEST_PERF_EVENTS | bool | 璇﹁ tools/perf/design.txt |
| GUP_GET_PXX_LOW_HIGH | bool | 鎻愪緵涓€涓祴璇曟ā鍧楋紝鍒嗛厤骞堕噴鏀捐澶氫笉鍚屽ぇ灏忕殑鍧楀苟鎶ュ憡鑰楁椂銆傛棬鍦ㄦ彁渚涗竴绉嶄竴鑷寸殑鏂瑰紡鏉ュ害閲忓 dma_pool_all鈥︹€︾殑鏇存敼鎵€浜х敓鐨勫奖鍝?|
| GUP_TEST | bool | 鎻愪緵 /sys/kernel/debug/gup_test锛岃繘鑰屾彁渚涗竴绉嶅彂璧?ioctl 璋冪敤鐨勬柟娉曪紝杩欎簺璋冪敤鍙惎鍔ㄩ拡瀵?get_user_pages*() 涓?pin_user_pages*() 绯诲垪 API 璋冪敤鐨勫熀浜庡唴鏍哥殑鍗曞厓娴嬭瘯銆傝鈥︹€?|
| HARDLOCKUP_DETECTOR_COUNTS_HRTIMER | bool | 鍦ㄦ閫?Y 浠ヨ鍐呮牳鍦ㄢ€渉ard lockups锛堢‖閿佹锛夆€濇椂 panic锛岀‖閿佹鏄寚瀵艰嚧鍐呮牳鍦ㄤ腑鏂绂佺敤鐨勬儏鍐典笅浜庡唴鏍告ā寮忓惊鐜秴杩?10 绉掞紙鍙€氳繃 wat鈥︹€﹂厤缃級鐨勭己闄?|
| HARDLOCKUP_DETECTOR_PERF | bool | 灏嗕娇鐢ㄧ壒瀹氫簬浣撶郴缁撴瀯鐨勭‖閿佹妫€娴嬪櫒瀹炵幇銆? # 鈥減erf鈥濅笌鈥渂uddy鈥濅袱绉嶇‖閿佹妫€娴嬪櫒閮戒細璁℃暟 hrtimer 涓柇銆傛閰嶇疆鍚敤绠＄悊杩欎簺鈥︹€︾殑鍑芥暟 |
| HARDLOCKUP_DETECTOR_PREFER_BUDDY | bool | 鍦ㄦ閫?Y 浠ヤ紭鍏堜娇鐢?buddy 纭攣姝绘娴嬪櫒鑰岄潪 perf 鐗堛€備娇鐢?buddy 妫€娴嬪櫒鏃讹紝姣忎釜 CPU 鍒╃敤鑷韩鐨?softlockup hrtimer 鏉ユ鏌ヤ笅涓€涓?CPU 鏄惁閫氳繃 ve鈥︹€﹀湪澶勭悊 hrtimer 涓柇 |
| HASHTABLE_KUNIT_TEST | tristate | 鏋勫缓 hashtable 鐨?KUnit 娴嬭瘯濂椾欢銆傚畠娴嬭瘯 include/linux/hashtable.h 涓畾涔夌殑 API 鐨勫熀鏈姛鑳姐€傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄鈥︹€?|
| HASH_KUNIT_TEST | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃舵祴璇曞唴鏍哥殑瀛楃涓诧紙<linux/stringhash.h>锛変笌鏁存暟锛?linux/hash.h>锛夊搱甯屽嚱鏁般€侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞朵互 TA鈥︹€︽牸寮忓皢缁撴灉杈撳嚭鍒拌皟璇曟棩蹇?|
| HAS_SECURITY_AUDIT | def_bool | 杩欏皢鏋勫缓 securityfs 鏂囦欢绯荤粺銆傚畠鐩墠琚悇绉嶅畨鍏ㄦā鍧楋紙AppArmor銆両MA銆丼afeSetID銆乀OMOYO銆乀PM锛変娇鐢ㄣ€傝嫢浣犱笉纭畾濡備綍鍥炵瓟锛岄€?N銆?|
| HAVE_ARCH_AUDITSYSCALL | bool | 杩欐槸鍩轰簬 tick 鐨勫熀鏈?cputime 璁拌处锛岀淮鎶ゅ叧浜庣敤鎴枫€佺郴缁熶笌绌洪棽鏃堕棿锛堜互姣?jiffies 绮掑害锛夌殑缁熻鏁版嵁銆傝嫢涓嶇‘瀹氾紝閫?Y銆?|
| HAVE_ARCH_TLB_REMOVE_TABLE | def_bool | 灏濊瘯鍦?munmap 涓?exit_mmap 涔嬪鐨勮矾寰勪腑鍥炴敹绌虹殑鐢ㄦ埛椤佃〃椤点€傛敞鎰忥細鐩墠浠呭洖鏀剁┖鐨勭敤鎴?PTE 椤佃〃椤点€?|
| HAVE_ARCH_USERFAULTFD_MINOR | bool | 浣撶郴缁撴瀯鍏锋湁 userfaultfd 娆¤缂洪〉鏀寔 |
| HAVE_ARCH_USERFAULTFD_WP | bool | 浣撶郴缁撴瀯鍏锋湁 userfaultfd 鍐欎繚鎶ゆ敮鎸?|
| HAVE_DEBUG_BUGVERBOSE | bool | 鍚敤姝ら」浠ュ紑鍚閾捐〃閬嶅巻渚嬬▼鐨勬墿灞曟鏌ャ€傛閫夐」浠ユ€ц兘鎹㈠彇鏇撮珮璐ㄩ噺鐨勯敊璇姤鍛婏紝鏇撮€傚悎鍐呮牳璋冭瘯銆傝嫢浣犲叧蹇冣€︹€?|
| HAVE_DEBUG_STACKOVERFLOW | bool | 鑻ヤ綘甯屾湜妫€鏌ュ唴鏍搞€両RQ 涓庡紓甯告爤锛堣嫢浣犵殑浣撶郴缁撴瀯浣跨敤瀹冧滑锛夌殑婧㈠嚭锛屽湪姝ら€?Y銆傝嫢绌洪棽鏍堢┖闂撮檷鍒版煇涓€涓嬮檺浠ヤ笅锛屾閫夐」灏嗘樉绀鸿缁嗘秷鎭€︹€?|
| HAVE_HARDLOCKUP_DETECTOR_BUDDY | bool | 鍦ㄦ閫?Y 浠ヨ鍐呮牳鍏呭綋鐪嬮棬鐙楁娴嬬‖閿佹銆傜‖閿佹鏄寚瀵艰嚧 CPU 鍦ㄥ唴鏍告ā寮忓惊鐜秴杩?10 绉掋€佷笖涓嶈鍏朵粬涓柇鈥︹€︾殑缂洪櫡 |
| HAVE_KERNEL_GZIP | bool | Linux 鍐呮牳鏄竴绉嶈嚜瑙ｅ帇鍙墽琛屾枃浠躲€傛湁澶氱鍘嬬缉绠楁硶鍙敤锛屽畠浠湪鏁堢巼銆佸帇缂╀笌瑙ｅ帇閫熷害涓婂悇涓嶇浉鍚屻€傚帇缂╅€熷害浠呭湪鈥︹€︽椂鎵嶇浉鍏?|
| HAVE_LD_DEAD_CODE_DATA_ELIMINATION | bool | 杩欒姹備綋绯荤粨鏋勬爣娉ㄦ垨浠ュ叾浠栨柟寮忎繚鎶ゅ叾澶栭儴鍏ュ彛鐐逛笉琚涪寮冦€傞摼鎺ヨ剼鏈繕蹇呴』灏?.text.*銆?data.* 涓?.bss.* 姝ｇ‘鍚堝苟鍒拌緭鍑烘鈥︹€?|
| HAVE_PCSPKR_PLATFORM | bool | 姝ら€夐」鍏佽绂佺敤鎴栬皟鏁存煇浜涘熀纭€鍐呮牳閫夐」涓庤缃€傝繖閫傜敤浜庤兘澶熷蹇嶁€滈潪鏍囧噯鈥濆唴鏍哥殑涓撻棬鐜銆備粎褰撲綘纭疄鈥︹€︽椂鎵嶄娇鐢?|
| HAVE_PERF_EVENTS | bool | 璇﹁ tools/perf/design.txt銆?|
| HAVE_SCHED_AVG_IRQ | def_bool | 閫夋嫨姝ら€夐」浠ュ湪璋冨害鍣ㄤ腑鍚敤 HW 鍘嬪姏璁拌处銆侶W 鍘嬪姏鏄紶閫掔粰璋冨害鍣ㄧ殑涓€涓€硷紝鍙嶆槧浜嗙敱 HW 鑺傛祦鈥︹€﹀鑷寸殑 CPU 璁＄畻鑳藉姏闄嶄綆 |
| HAVE_UNSTABLE_SCHED_CLOCK | bool | 璇ョ壒鎬ц璋冨害鍣ㄥ熀浜庡綋鍓嶅湪璇?CPU 涓婂彲璋冨害鐨?RUNNABLE 浠诲姟璺熻釜姣忎釜 CPU 鐨勯挸浣嶅埄鐢ㄧ巼銆傞€氳繃姝ら€夐」锛岀敤鎴峰彲鎸囧畾姣忎釜 CPU 鐨勬渶灏忎笌鏈€澶у埄鐢ㄧ巼鈥︹€?|
| HEADERS_INSTALL | bool | 姝ら€夐」灏嗘妸 uapi 澶存枃浠讹紙瀵煎嚭鍒扮敤鎴风┖闂寸殑澶存枃浠讹級瀹夎鍒?usr/include 鐩綍锛屼緵鍐呮牳鏋勫缓鏈熼棿浣跨敤銆傛瀯寤哄唴鏍告湰韬苟涓嶉渶瑕佸畠锛屼絾鈥︹€﹂渶瑕?|
| HMM_MIRROR | bool | 鍏佽鍒涘缓 struct page 鏉ヨ〃绀轰笉鍙鍧€鐨勮澶囧唴瀛橈紝鍗冲彧鑳戒粠璁惧锛堟垨璁惧缁勶級璁块棶鐨勫唴瀛樸€備綘鍙兘杩樺笇鏈涢€変腑 HMM_MIRROR銆?|
| HUGETLB_PAGE | def_bool | 鍦ㄦ閫?Y 浠ユ煡鐪嬪悇绉嶆潅椤规枃浠剁郴缁熺殑閫夐」锛屼緥濡傛潵鑷叾浠栨搷浣滅郴缁熺殑鏂囦欢绯荤粺銆傛閫夐」鏈韩涓嶆坊鍔犱换浣曞唴鏍镐唬鐮併€傝嫢浣犻€?N锛屾墍鏈夆€︹€?|
| HUGETLB_PAGE_OPTIMIZE_VMEMMAP_DEFAULT_ON | bool | HugeTLB Vmemmap 浼樺寲锛圚VO锛夐粯璁ゅ叧闂€傚湪姝ら€?Y 浠ラ粯璁ゅ惎鐢?HVO銆傚畠鍙€氳繃 hugetlb_free_vmemmap=off锛堝紩瀵煎懡浠よ锛夋垨 hugetlb_optimize_vmemmap锛坰ysctl锛夌鐢ㄣ€?|
| HWPOISON_INJECT | tristate | NOMMU 鐨?mmap() 缁忓父闇€瑕佸垎閰嶅ぇ鍧楄繛缁唴瀛樻潵瀛樺偍鏄犲皠锛屼絾瀹冨彧鑳藉悜绯荤粺鍒嗛厤鍣ㄨ姹?2^N*PAGE_SIZE 澶у皬鐨勫潡鈥斺€旇繖鈥︹€?|
| HW_BREAKPOINT_KUNIT_TEST | bool | hw_breakpoint 绾︽潫璁拌处鐨勬祴璇曘€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| HYPERV_TESTING | bool | 閫夋嫨姝ら€夐」浠ュ惎鐢?Hyper-V vmbus 娴嬭瘯銆?|
| IDLE_PAGE_TRACKING | bool | 璇ョ壒鎬у厑璁镐及绠楀湪缁欏畾鏃堕棿娈靛唴鏈璁块棶鐨勭敤鎴烽〉鏁伴噺銆傝淇℃伅鍙敤浜庤皟浼樺唴瀛?cgroup 闄愬埗鍜?鎴栫敤浜庝綔涓氭斁缃€︹€?|
| IKCONFIG | tristate | 姝ら€夐」灏嗗畬鏁寸殑 Linux 鍐呮牳鈥?config鈥濇枃浠跺唴瀹逛繚瀛樺埌鍐呮牳涓€傚畠璁板綍浜嗚繍琛屼腑鐨勫唴鏍告垨纾佺洏涓婄殑鍐呮牳鈥︹€︿娇鐢ㄤ簡鍝簺鍐呮牳閫夐」 |
| IKCONFIG_PROC | bool | 姝ら€夐」閫氳繃 /proc/config.gz 鍚敤瀵瑰唴鏍搁厤缃枃浠剁殑璁块棶銆?|
| IKHEADERS | tristate | 姝ら€夐」鍚敤瀵规瀯寤鸿繃绋嬩腑鐢熸垚鐨勩€佸唴鏍稿唴澶存枃浠剁殑璁块棶銆傝繖浜涘ご鏂囦欢鍙敤浜庢瀯寤?eBPF 璺熻釜绋嬪簭鎴栫被浼肩▼搴忋€傝嫢浣犲皢澶存枃浠舵瀯寤轰负鈥︹€?|
| INDIRECT_IOMEM | bool | 姝ら€夐」鐢卞叾浠栭€夐」/浣撶郴缁撴瀯閫変腑锛屼互鎻愪緵妯℃嫙鐨?iomem 璁块棶鍣ㄣ€?|
| INDIRECT_IOMEM_FALLBACK | bool | 鑻ラ€変腑 INDIRECT_IOMEM锛屾閫夐」鍦?IO 鍐呭瓨鍦板潃涓嶆槸宸叉敞鍐岀殑妯℃嫙鍖哄煙鏃讹紝鍚敤鍥為€€鍒版櫘閫?mmio 璁块棶銆?|
| INET | bool | 杩欎簺鏄簰鑱旂綉涓庡ぇ澶氭暟鏈湴浠ュお缃戜笂浣跨敤鐨勫崗璁€傚己鐑堝缓璁湪姝ら€?Y锛堣繖灏嗕娇鍐呮牳澧炲ぇ绾?400 KB锛夛紝鍥犱负鏌愪簺绋嬪簭锛堝 X 绐楀彛鈥︹€?|
| INITRAMFS_PRESERVE_MTIME | bool | initramfs cpio 褰掓。涓殑姣忎釜鏉＄洰閮藉甫鏈?mtime 鍊笺€傚惎鐢ㄥ悗锛岃В鍑虹殑 cpio 椤归噰鐢ㄨ mtime锛岀洰褰?mtime 璁剧疆寤惰繜鍒板叾浠讳綍瀛愭潯鐩垱寤轰箣鍚庘€︹€?|
| INITRAMFS_TEST | bool | 鏋勫缓 initramfs 鐨?KUnit 娴嬭瘯銆傝鍙傞槄 Documentation/dev-tools/kunit |
| INTEL_TXT | bool | 姝ら€夐」鍚敤閰嶅悎 Trusted Boot锛坱boot锛夋ā鍧楀紩瀵煎唴鏍哥殑鏀寔銆傚畠灏嗗埄鐢?Intel(R) 鍙俊鎵ц鎶€鏈鍐呮牳鎵ц鍙害閲忕殑鍚姩銆傝嫢鈥︹€?|
| INTERVAL_TREE_SPAN_ITER | bool | 鏀寔鍦?XArray 涓崰鎹涓繛缁储寮曠殑鏉＄洰銆?|
| INTERVAL_TREE_TEST | tristate | 涓€涓祴閲忓尯闂存爲搴撴€ц兘鐨勫熀鍑嗘祴璇?|
| INT_LOG_KUNIT_TEST | tristate | 姝ら€夐」鍚敤閽堝 int_log 搴撶殑 KUnit 娴嬭瘯濂椾欢锛岃搴撴彁渚涗袱涓嚱鏁颁互鍒嗗埆璁＄畻浠?2 鍜?10 涓哄簳鐨勬暣鏁板鏁帮紝鍒嗗埆绉颁负 intlog2 涓?intlog10銆傝鈥︹€?|
| INT_POW_KUNIT_TEST | tristate | 姝ら€夐」鍚敤閽堝 int_pow 鍑芥暟锛堟墽琛屾暣鏁板箓杩愮畻锛夌殑 KUnit 娴嬭瘯濂椾欢銆傝娴嬭瘯濂椾欢鏃ㄥ湪楠岃瘉 int_pow 鐨勫疄鐜拌兘姝ｇ‘璁＄畻鈥︹€?|
| INT_SQRT_KUNIT_TEST | tristate | 姝ら€夐」鍚敤閽堝 int_sqrt() 鍑芥暟锛堟墽琛屽钩鏂规牴璁＄畻锛夌殑 KUnit 娴嬭瘯濂椾欢銆傝娴嬭瘯濂椾欢妫€鏌ュ悇绉嶅満鏅紙鍖呮嫭杈圭晫鎯呭喌锛変互纭繚姝ｇ‘鎬с€傝嫢鈥︹€?|
| IO_STRICT_DEVMEM | bool | 鑻ョ鐢ㄦ閫夐」锛屼綘渚垮厑璁哥敤鎴风┖闂达紙root锛夎闂墍鏈?io-memory锛屾棤璁烘槸鍚︽湁椹卞姩姝ｅ湪浣跨敤璇ヨ寖鍥淬€傛剰澶栬闂樉鐒舵槸鐏鹃毦鎬х殑锛屼絾鈥︹€?|
| IO_URING | bool | 姝ら€夐」鍚敤瀵?io_uring 鎺ュ彛鐨勬敮鎸侊紝浣垮簲鐢ㄨ兘澶熼€氳繃鍐呮牳涓庡簲鐢ㄤ箣闂村叡浜殑鎻愪氦鐜笌瀹屾垚鐜潵鎻愪氦骞跺畬鎴?IO銆?|
| IO_URING_MOCK_FILE | tristate | 涓?io_uring 瀛愮郴缁熸祴璇曞惎鐢ㄦā鎷熸枃浠躲€侫BI 浠嶅彲鑳藉彉鍖栵紝鍥犳瀹冧粛鏄疄楠屾€х殑锛屽彧搴斾负鐗瑰畾鐨勬祴璇曠洰鐨勫惎鐢ㄣ€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| IO_URING_ZCRX | def_bool |  |
| IRQ_TIME_ACCOUNTING | bool | 閫夋嫨姝ら€夐」浠ュ惎鐢ㄧ粏绮掑害浠诲姟 irq 鏃堕棿璁拌处銆傝繖閫氳繃璇诲彇 softirq 涓?hardirq 鐘舵€佹瘡娆¤浆鎹㈡椂鐨勬椂闂存埑鏉ュ疄鐜帮紝鍥犳鍙兘甯︽潵灏戦噺鎬ц兘鈥︹€?|
| IS_SIGNED_TYPE_KUNIT_TEST | tristate | 鏋勫缓閽堝 is_signed_type() 瀹忕殑鍗曞厓娴嬭瘯銆傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/ 涓殑 KUnit 鏂囨。銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| KALLSYMS | bool | 鍦ㄦ閫?Y 浠ヨ鍐呮牳鎵撳嵃绗﹀彿鍖栫殑宕╂簝淇℃伅涓庣鍙峰寲鏍堝洖婧€傝繖浼氫竴瀹氱▼搴︿笂澧炲ぇ鍐呮牳浣撶Н锛屽洜涓烘墍鏈夌鍙烽兘蹇呴』鍔犺浇杩涘唴鏍搁暅鍍忋€?|
| KALLSYMS_ALL | bool | 閫氬父 kallsyms 鍙寘鍚嚱鏁扮殑绗﹀彿浠ユ彁渚涙洿濂界殑 OOPS 娑堟伅涓庡洖婧紙鍗虫潵鑷?text 涓?inittext 娈电殑绗﹀彿锛夈€傝繖瀵瑰ぇ澶氭暟鎯呭喌宸茶冻澶熴€備粎鍦ㄢ€︹€?|
| KALLSYMS_SELFTEST | bool | 娴嬭瘯鏌愪簺鎺ュ彛锛堝 kallsyms_lookup_name锛夌殑鍩烘湰鍔熻兘涓庢€ц兘銆傚畠杩樹細璁＄畻褰撳墠绗﹀彿闆嗕笅 kallsyms 鍘嬬缉绠楁硶鐨勫帇缂╃巼鈥︹€?|
| KCMP | bool | 鍚敤鍐呮牳璧勬簮姣旇緝绯荤粺璋冪敤銆傚畠涓虹敤鎴风┖闂存彁渚涙瘮杈冧袱涓繘绋嬫槸鍚﹀叡浜叕鍏辫祫婧愶紙濡傛枃浠舵弿杩扮涔冭嚦铏氭嫙鈥︹€︼級鐨勮兘鍔?|
| KCOV | bool | KCOV 浠ヤ竴绉嶉€傚悎瑕嗙洊鐜囧紩瀵兼ā绯婃祴璇曪紙闅忔満鍖栨祴璇曪級鐨勫舰寮忔毚闇插唴鏍镐唬鐮佽鐩栫巼淇℃伅銆傛洿澶氱粏鑺傝鍙傞槄 Documentation/dev-tools/kcov.rst銆?|
| KCOV_ENABLE_COMPARISONS | bool | KCOV 杩樹細鏆撮湶鎻掓々浠ｇ爜涓瘡娆℃瘮杈冪殑鎿嶄綔鏁帮紝浠ュ強鎿嶄綔鏁板ぇ灏忎笌姣旇緝鎸囦护鐨?PC銆傝繖浜涙搷浣滄暟鍙妯＄硦娴嬭瘯寮曟搸鐢ㄦ潵鏀硅繘鈥︹€?|
| KCOV_INSTRUMENT_ALL | bool | 鑻ヤ綘鍦ㄨ繘琛岄€氱敤鐨勭郴缁熻皟鐢ㄦā绯婃祴璇曪紙濡?syzkaller锛夛紝浣犱細甯屾湜瀵规暣涓唴鏍告彃妗╋紝骞跺簲鍦ㄦ閫?y銆傝嫢浣犺繘琛屾洿鏈夐拡瀵规€х殑妯＄硦娴嬭瘯锛堝鈥︹€?|
| KCOV_IRQ_AREA_SIZE | hex | KCOV 浣跨敤棰勫垎閰嶇殑姣?CPU 鍖哄煙鏉ヤ粠杞腑鏂敹闆嗚鐩栫巼銆傝繖鎸囧畾浜嗚繖浜涘尯鍩熺殑澶у皬锛堜互 unsigned long 瀛楁暟璁★級銆?|
| KCOV_SELFTEST | bool | 鍦ㄥ紩瀵兼椂杩愯绠€鐭殑 KCOV 瑕嗙洊鐜囨敹闆嗚嚜妫€娴嬭瘯銆傛祴璇曞け璐ユ椂浼氬鑷村唴鏍?panic銆傚缓璁惎鐢紝浠ョ‘淇濆叧閿姛鑳芥寜棰勬湡宸ヤ綔銆?|
| KERNEL_BZIP2 | bool | 鍏跺帇缂╃巼涓庨€熷害灞呬腑銆傝В鍘嬮€熷害鍦ㄥ悇椤归€夋嫨涓渶鎱€備笌 gzip 鐩告瘮锛屼娇鐢?bzip2 鍐呮牳浣撶Н绾﹀噺灏?10%銆俠zip2 浼氬崰鐢ㄥぇ閲忊€︹€?|
| KERNEL_GZIP | bool | 涔呯粡鑰冮獙鐨?gzip 鍘嬬缉銆傚畠鍦ㄥ帇缂╃巼涓庤В鍘嬮€熷害涔嬮棿鎻愪緵浜嗚壇濂界殑骞宠　銆?|
| KERNEL_LZ4 | bool | LZ4 鏄竴绉嶅浐瀹氥€侀潰鍚戝瓧鑺傜紪鐮佺殑 LZ77 绫诲帇缂╁櫒銆侺Z4 瑙?鍘嬬缉宸ュ叿鐨勫垵姝ョ増鏈彲鍦?<https://code.google.com/p/lz4/> 鑾峰彇銆傚叾鍘嬬缉鐜囪緝鈥︹€?|
| KERNEL_LZMA | bool | 璇ュ帇缂╃畻娉曠殑鍘嬬缉鐜囨渶浣炽€傝В鍘嬮€熷害浠嬩簬 gzip 涓?bzip2 涔嬮棿銆傚帇缂╂渶鎱€備笌 gzip 鐩告瘮锛屼娇鐢?LZMA 鍐呮牳浣撶Н绾﹀噺灏?33%銆?|
| KERNEL_LZO | bool | 鍏跺帇缂╃巼鍦ㄥ悇椤归€夋嫨涓渶宸€傚唴鏍镐綋绉瘮 gzip 澶х害 10%锛涗絾鍏堕€熷害锛堝帇缂╀笌瑙ｅ帇锛夋渶蹇€?|
| KERNEL_UNCOMPRESSED | bool | 鐢熸垚鏈帇缂╃殑鍐呮牳闀滃儚銆傝繖閫氬父涓嶆槸浣犳兂瑕佺殑銆傚畠鍙敤浜庡湪鎱㈤€熶豢鐪熺幆澧冧腑璋冭瘯鍐呮牳锛屽洜涓哄湪杩欎簺鐜涓В鍘嬩笌绉诲姩鍐呮牳鈥︹€?|
| KERNEL_XZ | bool | XZ 浣跨敤 LZMA2 绠楁硶涓庣壒瀹氫簬鎸囦护闆嗙殑 BCJ 杩囨护鍣紝鍙敼鍠勫彲鎵ц浠ｇ爜鐨勫帇缂╃巼銆備笌 gzip 鐩告瘮锛屼娇鐢?XZ 鍐呮牳浣撶Н绾﹀噺灏?30%鈥︹€?|
| KERNEL_ZSTD | bool | ZSTD 鏄竴绉嶉潰鍚戜腑绛夊帇缂╃巼銆佽В鍘嬮€熷害蹇€熺殑鍘嬬缉绠楁硶銆傚畠鐨勫帇缂╂晥鏋滀紭浜?GZIP锛岃В鍘嬮€熷害涓?LZO 鐩歌繎锛屼絾鎱簬 LZ4鈥︹€?|
| KFIFO_KUNIT_TEST | tristate | 鏋勫缓閫氱敤 FIFO 瀹炵幇鐨?KUnit 娴嬭瘯濂椾欢銆傚畠娴嬭瘯 kfifo 绫诲瀷鍙婄浉鍏冲畯鐨?API 涓庡熀鏈姛鑳姐€傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄鈥︹€?|
| KPROBES_SANITY_TEST | tristate | 姝ら€夐」鎻愪緵鍦ㄥ紩瀵兼椂娴嬭瘯鍩烘湰 kprobes 鍔熻兘鐨勮兘鍔涖€備細鎻掑叆 kprobe 涓?kretprobe 鏍蜂緥骞堕獙璇佸叾鍔熻兘銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| LATENCYTOP | bool | 鑻ヤ綘甯屾湜浣跨敤 LatencyTOP 宸ュ叿鏉ユ煡鏄庡摢浜涚敤鎴风┖闂村洜鍝簺鍐呮牳鎿嶄綔鑰岄樆濉烇紝鍚敤姝ら€夐」銆?|
| LAZY_MMU_MODE_KUNIT_TEST | tristate | 鍚敤姝ら€夐」浠ユ鏌?lazy MMU 妯″紡鎺ュ彛鏄惁鎸夐鏈熷伐浣溿€備粎鍖呭惈瀵归€氱敤鎺ュ彛鐨勬祴璇曪紙涓嶅惈浣撶郴缁撴瀯鐩稿叧琛屼负锛夈€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| LD_DEAD_CODE_DATA_ELIMINATION | bool | 鑻ヤ綘甯屾湜閫氳繃閾炬帴鍣ㄨ繘琛屾浠ｇ爜涓庢暟鎹秷闄わ紙浠?-ffunction-sections -fdata-sections 缂栬瘧锛屽苟浠?--gc-sections 閾炬帴锛夛紝鍚敤姝ら」銆傝繖鍙互鍑忓皯纾佺洏涓庡唴瀛樹腑鈥︹€?|
| LD_ORPHAN_WARN | def_bool | 鍚敤瀵?/proc/sys/debug/exception-trace 鐨勬敮鎸併€?|
| LIBFDT | bool | 鍚敤蹇€熸煡鎵惧璞℃爣璇嗙娉ㄥ唽琛ㄣ€?|
| LINEAR_RANGES | tristate | 姝ら€夐」鎻愪緵 packing() 杈呭姪鍑芥暟锛屽畠鍏佽鍦?CPU 鍙敤琛ㄧず涓庡彲鑳藉叿鏈夎繖浜涗换鎰忕粍鍚堢壒鎬х殑鍐呭瓨琛ㄧず涔嬮棿杞崲浣嶅煙鈥︹€?|
| LINEAR_RANGES_TEST | tristate | 鏋勫缓 linear_ranges 鍗曞厓娴嬭瘯锛屽湪寮曞鏃惰繍琛屻€傛祴璇?linear_ranges 閫昏緫鐨勬纭€с€傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 KUnit 鏂囨。鈥︹€?|
| LIST_KUNIT_TEST | tristate | 鏋勫缓閾捐〃 KUnit 娴嬭瘯濂椾欢銆傚畠娴嬭瘯 list_head 绫诲瀷鍙婄浉鍏冲畯鐨?API 涓庡熀鏈姛鑳姐€侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞跺皢缁撴灉杈撳嚭鍒拌皟璇曟棩蹇椻€︹€?|
| LIST_PRIVATE_KUNIT_TEST | tristate | 鏋勫缓閽堝 include/linux/list_private.h 涓畾涔夌殑绉佹湁閾捐〃鍘熻鐨?KUnit 娴嬭瘯銆傝繖浜涘師璇厑璁告搷浣滆鏍囪涓虹鏈夊苟閲嶆柊鈥︹€︾殑 list_head 鎴愬憳 |
| LIVEUPDATE_TEST | bool | 涓?Live Update Orchestrator 鍚敤涓€涓唴缃唴鏍告祴璇曟ā鍧椼€傝妯″潡閫氳繃娉ㄥ唽涓€缁勬ā鎷?FLB 瀵硅薄锛堜笌浠讳綍鐪熷疄鏂囦欢澶勭悊绋嬪簭鈥︹€︼級鏉ラ獙璇?File-Lifecycle-Bound 瀛愮郴缁?|
| LKDTM | tristate | 璇ユā鍧楅€氳繃鍦ㄩ瀹氫箟宕╂簝鐐瑰紩鍙戠郴缁熸晠闅滐紝鏉ユ祴璇曚笉鍚岀殑杞偍鏈哄埗銆傝嫢浣犱笉闇€瑕佸畠锛氶€?N锛涘湪姝ら€?M 灏嗘湰浠ｇ爜缂栬瘧涓烘ā鍧椼€傝鈥︹€?|
| LOCALVERSION | string | 鍦ㄤ綘鐨勫唴鏍哥増鏈湯灏捐拷鍔犱竴涓澶栧瓧绗︿覆銆備緥濡傦紝瀹冧細鍦ㄤ綘杈撳叆 uname 鏃舵樉绀恒€備綘鍦ㄦ璁剧疆鐨勫瓧绗︿覆灏嗚杩藉姞鍒颁换浣曟枃浠跺悕涓衡€︹€︾殑鏂囦欢鍐呭涔嬪悗 |
| LOCALVERSION_AUTO | bool | 杩欏皢灏濊瘯閫氳繃鏌ユ壘灞炰簬褰撳墠鏍戦《淇鐗堢殑 git 鏍囩锛岃嚜鍔ㄥ垽鏂綋鍓嶆爲鏄惁涓哄彂甯冩爲銆傛牸寮忎负 -gxxxxxxxx 鐨勫瓧绗︿覆灏嗚娣诲姞鈥︹€?|
| LOCKDEP | bool | 鑻ラ亣鍒扳€淏UG: MAX_LOCKDEP_ENTRIES too low!鈥濇秷鎭紝灏濊瘯澧炲ぇ姝ゅ€笺€?|
| LOCKDEP_CHAINS_BITS | int | 鑻ラ亣鍒扳€淏UG: MAX_LOCKDEP_CHAINS too low!鈥濇秷鎭紝灏濊瘯澧炲ぇ姝ゅ€笺€?|
| LOCKDEP_CIRCULAR_QUEUE_BITS | int | 鑻ュ洜 __cq_enqueue() 澶辫触鑰岄亣鍒扳€渓ockdep bfs error:-1鈥濊鍛婏紝灏濊瘯澧炲ぇ姝ゅ€笺€?|
| LOCKDEP_STACK_TRACE_BITS | int | 鑻ラ亣鍒扳€淏UG: MAX_STACK_TRACE_ENTRIES too low!鈥濇秷鎭紝灏濊瘯澧炲ぇ姝ゅ€笺€侹ASAN 浼氭樉钁楀鍔犳爤璺熻釜娑堣€楋紝鍥犱负鍏?slab 璺熻釜涓?lockdep 鐨勪緷璧栤€︹€?|
| LOCKDEP_STACK_TRACE_HASH_BITS | int | 鑻ヤ綘闇€瑕佽緝澶х殑 STACK_TRACE_HASH_SIZE锛屽皾璇曞澶ф鍊笺€?|
| LOCKUP_DETECTOR | bool | 鍦ㄦ閫?Y 浠ヨ鍐呮牳鍏呭綋鐪嬮棬鐙楁娴嬭蒋閿佹銆傝蒋閿佹鏄寚瀵艰嚧鍐呮牳鍦ㄥ唴鏍告ā寮忓惊鐜秴杩?20 绉掋€佷笖涓嶇粰鍏朵粬浠诲姟鈥︹€︾殑缂洪櫡 |
| LOCK_DEBUGGING_SUPPORT | bool | 璇ョ壒鎬ц鍐呮牳鑳藉璇佹槑鍐呮牳杩愯鏃跺彂鐢熺殑鎵€鏈夊姞閿佸湪 mathematically 涓婃槸姝ｇ‘鐨勶細鍗充换浣曟儏鍐典笅閮戒笉鍙兘鍑虹幇浠绘剰锛堝皻鏈Е鍙戠殑锛夌粍鍚堚€︹€?|
| LOCK_MM_AND_FIND_VMA | bool | 鍚敤 NUMA 妯℃嫙銆備娇鐢ㄢ€渘uma=fake=N鈥濆紩瀵兼椂锛圢 涓鸿妭鐐规暟锛夛紝骞冲潶鏈哄櫒灏嗚鎷嗗垎涓鸿櫄鎷熻妭鐐广€傝繖浠呭璋冭瘯鏈夌敤銆?|
| LOCK_STAT | bool | 璇ョ壒鎬у惎鐢ㄥ閿佺珵浜夌偣鐨勮窡韪€傛洿澶氱粏鑺傝鍙傞槄 Documentation/locking/lockstat.rst銆傚畠杩樺惎鐢ㄢ€減erf lock鈥濓紙perf 鐨勫瓙鍛戒护锛夋墍闇€鐨勯攣浜嬩欢銆傝嫢浣犲笇鏈涒€︹€?|
| LOCK_TORTURE_TEST | tristate | 姝ら€夐」鎻愪緵涓€涓唴鏍告ā鍧楋紝瀵瑰唴鏍搁攣鍘熻杩愯 torture 娴嬭瘯銆傝嫢闇€瑕侊紝璇ユā鍧楀彲鍦ㄨ娴嬬殑杩愯涓唴鏍镐笂浜嬪悗鏋勫缓銆傚湪姝ら€?Y鈥︹€?|
| LOG_BUF_SHIFT | int | 浠?2 鐨勫箓閫夋嫨鏈€灏忓唴鏍告棩蹇楃紦鍐插尯澶у皬銆傛渶缁堝ぇ灏忓彈 LOG_CPU_MAX_BUF_SHIFT 閰嶇疆鍙傛暟褰卞搷锛堣涓嬶級銆備换浣曟洿澶х殑澶у皬涔熷彲鑳借鈥渓og_buf_len鈥濆紩瀵尖€︹€?|
| LOG_CPU_MAX_BUF_SHIFT | int | 姝ら€夐」鍏佽鏍规嵁 CPU 鏁伴噺澧炲ぇ榛樿鐜舰缂撳啿鍖哄ぇ灏忋€傝鍊煎畾涔変簡姣忎釜 CPU 浠?2 鐨勫箓璁＄殑璐＄尞銆傚凡鐢ㄧ┖闂撮€氬父鍙湁鍑犺鈥︹€?|
| LONGEST_SYM_KUNIT_TEST | tristate | 娴嬭瘯鍙兘鐨勬渶闀跨鍙枫€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| LRU_GEN | bool | 涓€绉嶇敤浜庡唴瀛樿秴閰嶇殑楂樻€ц兘 LRU 瀹炵幇銆傝瑙?Documentation/admin-guide/mm/multigen_lru.rst銆?|
| LRU_GEN_ENABLED | bool | 姝ら€夐」榛樿鍚敤澶氫唬 LRU銆?|
| LRU_GEN_STATS | bool | 闄ら潪浣犳墦绠椾负浜嗚皟璇曡€屾煡鐪嬭椹遍€愪唬鐨勫巻鍙茬粺璁★紝鍚﹀垯涓嶈鍚敤姝ら€夐」銆傛閫夐」鏈?per-memcg 涓?per-node 鐨勫唴瀛樺紑閿€銆?|
| LRU_GEN_WALKS_MMU | def_bool | 鍏佽鍦ㄧ己椤靛鐞嗘湡闂磋繘琛?per-vma 鍔犻攣銆傝鐗规€у湪澶勭悊缂洪〉鏃跺厑璁稿垎鍒攣瀹氭瘡涓櫄鎷熷唴瀛樺尯鍩燂紝鑰岄潪鑾峰彇 mmap_lock銆?|
| LSM_MMAP_MIN_ADDR | int | 杩欐槸搴斿綋琚繚鎶や互鍏嶅彈鐢ㄦ埛绌洪棿鍒嗛厤鐨勪綆铏氭嫙鍐呭瓨閮ㄥ垎銆傞樆姝㈢敤鎴峰啓鍏ヤ綆鍦板潃椤垫湁鍔╀簬闄嶄綆鍐呮牳绌烘寚閽堢己闄风殑褰卞搷銆傛湁鍏斥€︹€?|
| LWTUNNEL_BPF | bool | 鍏佽鍦ㄨ矾鐢辨煡鎵句箣鍚庯紝灏?BPF 绋嬪簭浣滀负鍏ュ悜涓庡嚭鍚戞暟鎹寘鐨勪笅涓€璺冲姩浣滆繍琛屻€?|
| LZO_COMPRESS | tristate | 椹卞姩鍙€夋嫨姝ら€夐」锛屼负鍙傛暟 'm'锛堜冀缃楃摝鍩熼樁锛変笌 't'锛堢籂閿欒兘鍔涳級寮哄埗鐗瑰畾甯告暟鍊笺€傝繖浜涚壒瀹氬€煎繀椤婚€氳繃澹版槑榛樿鈥︹€︽潵璁剧疆 |
| MAGIC_SYSRQ | bool | 鑻ュ湪姝ら€?Y锛屽嵆浣跨郴缁熷湪渚嬪鍐呮牳璋冭瘯鏈熼棿宕╂簝锛屼綘浠嶅绯荤粺鏈変竴瀹氭帶鍒舵潈锛堜緥濡傦紝浣犺兘澶熷皢缂撳啿鍖虹紦瀛樺埛鍐欏埌纾佺洏銆侀噸鍚郴缁熲€︹€︼級 |
| MAGIC_SYSRQ_DEFAULT_ENABLE | hex | 鎸囧畾榛樿鍚敤鍝簺 SysRq 閿姛鑳姐€傚彲璁句负 1 鎴?0 浠ュ叏閮ㄥ惎鐢ㄦ垨绂佺敤锛屾垨璁句负 Documentation/admin-guide/sysrq.rst 涓弿杩扮殑浣嶆帺鐮併€?|
| MAGIC_SYSRQ_SERIAL | bool | 璁稿宓屽叆寮忔澘鍗″叿鏈夋湭杩炴帴鐨?TTL 鐢靛钩涓插彛锛屽彲鑳戒骇鐢熷鑷磋鎶?sysrq 妫€娴嬬殑鍨冨溇鏁版嵁銆傛閫夐」鍏佽浣犲喅瀹氭槸鍚︹€︹€﹀惎鐢?|
| MAGIC_SYSRQ_SERIAL_SEQUENCE | string | 鎸囧畾鍙窡鍦?BREAK 涔嬪悗浠ュ湪涓插彛鎺у埗鍙颁笂鍚敤 SysRq 鐨勫瓧绗﹀簭鍒椼€傝嫢涓嶇‘瀹氾紝鐣欑┖瀛楃涓诧紝璇ラ€夐」灏嗕笉琚惎鐢ㄣ€?|
| MAX_SKB_FRAGS | int | 姣忎釜 skb_shared_info 鎷ユ湁鏇村鍒嗙墖鏈夊姪浜?GRO 鏁堢巼銆傝繖鏈夊姪浜?BIG TCP 宸ヤ綔璐熻浇锛屼絾鍙兘鏆撮湶鏌愪簺閬楃暀椹卞姩涓殑缂洪櫡銆傝繖涔熶細澧炲姞灏忔暟鎹寘鐨勫唴瀛樺紑閿€鈥︹€?|
| MEMBARRIER | bool | 鍚敤 membarrier() 绯荤粺璋冪敤锛屽畠鍏佽鍦ㄦ墍鏈夎繍琛屼腑鐨勭嚎绋嬮棿鍙戝竷鍐呭瓨灞忛殰锛屽彲鐢ㄤ簬閫氳繃鍙樻崲鈥︹€﹂潪瀵圭О鍦板垎鎽婄敤鎴风┖闂村唴瀛樺睆闅滅殑浠ｄ环 |
| MEMCG | bool | 鎻愪緵瀵?cgroup 涓换鍔″唴瀛樺崰鐢ㄧ殑鎺у埗銆?|
| MEMCG_NMI_UNSAFE | bool | 宸茶 cgroup v2 瀹炵幇搴熷純鐨勪紶缁?cgroup v1 鍐呭瓨鎺у埗鍣ㄣ€倂1 淇濈暀缁欏皻鏈縼绉诲埌鏂?cgroup v2 鎺ュ彛鐨勯仐鐣欏簲鐢ㄣ€傝嫢浣犫€︹€?|
| MEMCPY_KUNIT_TEST | tristate | 鏋勫缓閽堝 memcpy()銆乵emmove() 涓?memset() 鍑芥暟鐨勫崟鍏冩祴璇曘€傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/ 涓殑 KUnit 鏂囨。鈥︹€?|
| MEMORY_HOTREMOVE | bool | 鍏佽杩佺Щ鍦ㄥ唴瀛樻皵鐞冧腑鑶ㄨ儉鐨勯〉锛屼娇瀹冧滑鑳戒粠浠呭彲鐢ㄤ簬鍙Щ鍔ㄥ垎閰嶏紙濡?ZONE_MOVABLE銆丆MA锛夌殑鍐呭瓨鍖哄煙鍒嗛厤锛屽苟涓斿彲浠モ€︹€?|
| MEMORY_NOTIFIER_ERROR_INJECT | tristate | 姝ら€夐」鎻愪緵鍚戝唴瀛樼儹鎻掓嫈閫氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠閫氳繃 /sys/kernel/debug/notifier-error-inject/me鈥︹€︿笅鐨?debugfs 鎺ュ彛鎺у埗 |
| MEMTEST | bool | 姝ら€夐」娣诲姞鍐呮牳鍙傛暟 'memtest'锛屽厑璁歌缃苟鎵ц memtest銆俶emtest=0 琛ㄧず绂佺敤锛涢粯璁?memtest=1 琛ㄧず鎵ц 1 绉嶆祴璇曟ā寮忥紱鈥︹€?memtest=17 琛ㄧず鎵ц 17 绉嶆祴璇曟ā寮忊€︹€?|
| MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT | bool | 涓哄唴瀛樺垎閰嶆€ц兘鍒嗘瀽娣诲姞甯︽湁鏈夌敤閿欒淇℃伅鐨勮鍛娿€?|
| MEM_SOFT_DIRTY | bool | 姝ら€夐」閫氳繃鍦?PTE 涓婂紩鍏ヨ蒋鑴忎綅鏉ュ惎鐢ㄥ唴瀛樺彉鏇磋窡韪€傚綋鏈変汉鍐欏叆鏌愰〉鏃惰缃浣嶏紝濡傚悓鏅€氳剰浣嶏紝浣嗗畠涓庢櫘閫氳剰浣嶄笉鍚岋紝鍙娓呴櫎鈥︹€?|
| MESSAGE_LOGLEVEL_DEFAULT | int | 鏈寚瀹氫紭鍏堢骇鐨?printk 璇彞鐨勯粯璁ゆ棩蹇楃骇鍒€傝嚜鑷冲皯 2.6.10 璧峰畠琚‖缂栫爜涓?KERN_WARNING锛屼絾瀵嗗垏瀹¤鏃ュ織鐨勪汉鍙兘甯屾湜灏嗗叾璁句负鈥︹€?|
| MHP_DEFAULT_ONLINE_TYPE_OFFLINE | bool | 鐑彃鎷斿唴瀛橀粯璁や笉浼氫笂绾裤€傚浜庣敱椹卞姩涓庣敤鎴风瓥鐣ュ鐞嗙儹鎻掓嫈鍐呭瓨涓婄嚎鐨勭郴缁燂紝閫夋嫨姝ら」銆?|
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_AUTO | bool | 鑻ヤ綘甯屾湜鍐呮牳鑷姩灏嗙儹鎻掓嫈鍐呭瓨涓婄嚎鍒板畠璁や负鍚堢悊鐨?zone锛岄€夋嫨姝ら」銆傝鍐呭瓨鍙兘琚敤浜庡唴鏍告暟鎹€?|
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_KERNEL | bool | 鑻ヤ綘甯屾湜鍐呮牳鑷姩灏嗙儹鎻掓嫈鍐呭瓨涓婄嚎鍒板彲鐢ㄤ簬鍐呮牳鏁版嵁鐨?zone锛岄€夋嫨姝ら」銆傝繖閫氬父鎸?ZONE_NORMAL銆?|
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_MOVABLE | bool | 鑻ヤ綘甯屾湜鍐呮牳鑷姩灏嗙儹鎻掓嫈鍐呭瓨涓婄嚎鍒?ZONE_MOVABLE锛岄€夋嫨姝ら」銆傝鍐呭瓨閫氬父涓嶈鐢ㄤ簬鍐呮牳鏁版嵁銆備粎褰撶鐞嗗憳鐭ラ亾鈥︹€︽椂鎵嶅簲浣跨敤銆?|
| MIGRATION | bool | 褰撳钩鍙颁笂瀛樺湪澶氱 HugeTLB 椤靛ぇ灏忔椂锛屽厑璁?pageblock_order 鍊间负鍔ㄦ€佽€岄潪浠呬负鏍囧噯 HUGETLB_PAGE_ORDER銆傛敞鎰?pageblock_order 鏃犳硶鈥︹€?|
| MIN_HEAP_KUNIT_TEST | tristate | 姝ら€夐」鍚敤閽堝 min heap 搴擄紙鎻愪緵鍒涘缓涓庣鐞嗘渶灏忓爢鐨勫嚱鏁帮級鐨?KUnit 娴嬭瘯濂椾欢銆傝娴嬭瘯濂椾欢妫€鏌ユ渶灏忓爢搴撶殑鍔熻兘銆傝嫢鈥︹€?|
| MMAP_ALLOW_UNINITIALIZED | bool | 閫氬父锛屾牴鎹?Linux 瑙勮寖锛屼粠 mmap() 鑾峰緱鐨勫尶鍚嶅唴瀛樺湪浼犻€掔粰鐢ㄦ埛绌洪棿涔嬪墠鍏跺唴瀹逛細琚竻闆躲€傚惎鐢ㄦ閰嶇疆閫夐」鍏佽浣犺姹傗€︹€?|
| MM_ID | def_bool | 閫忔槑澶ч〉鍏佽鍐呮牳鍦ㄥ彲鑳芥椂閫忔槑鍦板搴旂敤浣跨敤澶ч〉涓?huge tlb銆傝鐗规€у彲閫氳繃鈥︹€︽彁鍗囨煇浜涘簲鐢ㄧ殑璁＄畻鎬ц兘 |
| MODULE_ALLOW_BTF_MISMATCH | bool | 瀵逛簬鎷嗗垎 BTF 涓?vmlinux 涓嶅尮閰嶇殑妯″潡锛屼笉浣跨敤 BTF 鍔犺浇鑰屼笉鎷掔粷鍔犺浇銆傚惎鐢ㄦā鍧?BTF 鏃剁殑榛樿琛屼负鏄嫆缁濇绫讳笉鍖归厤鐨勬ā鍧楋紱姝ら€夐」鈥︹€?|
| MPILIB | tristate | 鏉ヨ嚜 GnuPG 鐨勫绮惧害鏁板搴撱€傚畠鐢ㄤ簬瀹炵幇 RSA 鏁板瓧绛惧悕楠岃瘉锛孖MA/EVM 鏁板瓧绛惧悕鎵╁睍浼氱敤鍒板畠銆?|
| MSEAL_SYSTEM_MAPPINGS | bool | 瀵圭郴缁熸槧灏勫簲鐢?mseal銆傜郴缁熸槧灏勫寘鎷?vdso銆乿var銆乿var_vclock銆乿ectors锛坅rm 鍏煎妯″紡锛夈€乻igpage锛坅rm 鍏煎妯″紡锛夈€乽probes銆傚唴瀛樺瘑灏佺壒鎬ч渶瑕?64 浣嶅唴鏍糕€︹€?|
| MULTIUSER | bool | 姝ら€夐」鍚敤瀵规櫘閫氱敤鎴枫€佺粍涓庤兘鍔涚殑鏀寔銆傝嫢鍦ㄦ閫?N锛屾墍鏈夎繘绋嬪皢浠?UID 0銆丟ID 0 鍙婃墍鏈夊彲鑳界殑鑳藉姏杩愯銆傚湪姝ら€?N 杩樹細缂栬瘧鎺夆€︹€?|
| NET | bool | 闄ら潪浣犵‘瀹炵煡閬撹嚜宸卞湪鍋氫粈涔堬紝鍚﹀垯搴斿湪姝ら€?Y銆傚師鍥犳槸鏌愪簺绋嬪簭鍗充娇鍦ㄦ湭杩炴帴鈥︹€︾殑鐙珛鏈哄櫒涓婅繍琛岋紝涔熼渶瑕佸唴鏍哥綉缁滄敮鎸?|
| NETDEV_ADDR_LIST_TEST | tristate | 瑕嗙洊鏍稿績缃戠粶鍩虹璁炬柦锛堝 sk_buff锛夌殑 KUnit 娴嬭瘯銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| NETDEV_NOTIFIER_ERROR_INJECT | tristate | 姝ら€夐」鎻愪緵鍚?netdevice 閫氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠閫氳繃 /sys/kernel/debug/notifier-error-inject/netdev 涓嬬殑 debugfs 鎺ュ彛鎺у埗銆傝嫢鈥︹€?|
| NETFILTER | bool | Netfilter 鏄竴涓敤浜庤繃婊や笌绡℃敼娴佺粡 Linux 涓绘満鐨勭綉缁滄暟鎹寘鐨勬鏋躲€傛暟鎹寘杩囨护鏈€甯歌鐨勭敤閫旀槸灏嗕綘鐨?Linux 涓绘満浣滀负闃茬伀澧欎繚鎶ゆ湰鍦扳€︹€?|
| NETFILTER_ADVANCED | bool | 鑻ュ湪姝ら€?Y锛屼綘鍙互鍦ㄦ墍鏈?netfilter 妯″潡涔嬮棿閫夋嫨銆傝嫢閫?N锛岃緝涓嶅父瑙佺殑妯″潡灏嗕笉鏄剧ず锛岃€屽ぇ澶氭暟浜洪渶瑕佺殑鍩烘湰妯″潡灏嗛粯璁や负 'M'銆傝嫢涓嶇‘瀹氾紝閫?Y銆?|
| NETWORK_FILESYSTEMS | bool | 鍦ㄦ閫?Y 浠ユ煡鐪嬬綉缁滄枃浠剁郴缁熶笌鏂囦欢绯荤粺鐩稿叧缃戠粶浠ｇ爜锛堝 NFS 瀹堟姢杩涚▼涓?RPCSEC 瀹夊叏妯″潡锛夌殑閫夐」銆傛閫夐」鏈韩涓嶆坊鍔犱换浣曞唴鏍镐唬鐮併€傝嫢鈥︹€?|
| NETWORK_SECMARK | bool | 杩欏惎鐢ㄥ缃戠粶鏁版嵁鍖呯殑瀹夊叏鏍囪锛岀被浼间簬 nfmark锛屼絾涓撶敤浜庡畨鍏ㄧ洰鐨勩€傝嫢浣犱笉纭畾濡備綍鍥炵瓟锛岄€?N銆?|
| NET_DEVLINK | bool | 鍚敤椤垫睜缁熻锛屼互璺熻釜椤垫睜涓殑椤靛垎閰嶄笌鍥炴敹銆傛閫夐」鍦ㄥ垎閰嶄笌鍥炴敹璺緞涓婂甫鏉ラ澶栫殑 CPU 寮€閿€锛屼互鍙婂瓨鍌ㄧ粺璁♀€︹€︾殑棰濆鍐呭瓨寮€閿€ |
| NET_DROP_MONITOR | tristate | 璇ョ壒鎬у湪缃戠粶鏍堜腑涓㈠純鏁版嵁鍖呮椂锛屽悜鐢ㄦ埛绌洪棿鎻愪緵鍛婅鏈嶅姟銆傚憡璀﹂€氳繃 netlink 濂楁帴瀛楀箍鎾粰浠讳綍鐩戝惉鐨勭敤鎴风┖闂磋繘绋嬨€傝鈥︹€?|
| NET_FLOW_LIMIT | bool | 褰撴帴鏀跺鐞?CPU 鐨?backlog 杈惧埌 netdev_max_backlog 鏃讹紝缃戠粶鏍堝繀椤讳涪寮冩暟鎹寘銆傝嫢浼楀娲昏穬娴佷腑浠呮湁灏戞暟浜х敓浜嗙粷澶у鏁拌礋杞斤紝涓㈠純瀹冧滑鐨勬祦閲忊€︹€?|
| NET_INGRESS | bool | 杩欐瀯寤洪拡瀵?handshake upcall 鏈哄埗鐨?KUnit 娴嬭瘯銆侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞朵互 TAP 鏍煎紡锛坔ttps://testanything.org/锛夊皢缁撴灉杈撳嚭鍒拌皟璇曟棩蹇椼€備粎瀵瑰唴鏍糕€︹€︽湁鐢?|
| NET_NS | bool | 鍏佽鐢ㄦ埛绌洪棿鍒涘缓鐪嬩技澶氫釜缃戠粶鏍堝疄渚嬬殑瀵硅薄銆?|
| NET_PKTGEN | tristate | 璇ユā鍧楀皢浠ュ彲閰嶇疆閫熺巼浠庣粰瀹氭帴鍙ｆ敞鍏ラ閰嶇疆鐨勬暟鎹寘銆傚畠鐢ㄤ簬缃戠粶鎺ュ彛鍘嬪姏娴嬭瘯涓庢€ц兘鍒嗘瀽銆傚鏋滀綘涓嶇悊瑙ｂ€︹€?|
| NET_PTP_CLASSIFY | def_bool | 杩欏厑璁稿叿鏈夌‖浠舵椂闂存埑鑳藉姏鐨?PHY锛堟垨鍏朵粬 MII 鎬荤嚎鍡呮帰璁惧锛夊缃戠粶鏁版嵁鍖呰繘琛屾椂闂存埑鏍囪銆傛閫夐」鍦ㄥ彂閫佷笌鎺ユ敹璺緞涓婂鍔犱竴浜涘紑閿€銆傝嫢鈥︹€?|
| NET_RX_BUSY_POLL | bool | 鍚敤姝ら」鍏佽灏?TCP 娴佽В鏋愬櫒涓?BPF_MAP_TYPE_SOCKMAP 涓€璧蜂娇鐢ㄣ€?|
| NFS_V4_2_SSC_HELPER | bool |  |
| NLATTR | bool | 鐢ㄤ簬閫氳繃杞杩涜涓柇缂撹В鐨勮緟鍔╁簱銆?|
| NOINSTR_VALIDATION | bool | 閫夋嫨姝ら€夐」灏嗗湪閾炬帴 vmlinux 鏃跺悜 ld 浼犻€?"-Map=vmlinux.map"銆傝鏂囦欢鍙敤浜庨獙璇佷笌璋冭瘯绁炲鐨勬鎿嶄綔锛屼互鍙婃煡鐪嬪摢浜涗唬鐮佹琚秷闄も€︹€?|
| NOTIFIER_ERROR_INJECTION | tristate | 姝ら€夐」鎻愪緵鍚戞寚瀹氶€氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠鏈夊姪浜庢祴璇曢€氱煡閾惧け璐ョ殑閿欒澶勭悊銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| NO_PAGE_MAPCOUNT | bool | 涓嶄负灞炰簬杈冨ぇ鍒嗛厤锛堝閫忔槑澶ч〉锛夌殑椤电淮鎶ゆ瘡椤?mapcount銆傚惎鐢ㄦ閰嶇疆閫夐」鍚庯紝涓€浜涗緷璧栨淇℃伅鐨勬帴鍙ｅ皢鈥︹€?|
| NUMA_BALANCING_DEFAULT_ENABLED | bool | 鑻ヨ缃紝鍦?NUMA 鏈哄櫒涓婅繍琛屾椂灏嗗惎鐢ㄨ嚜鍔?NUMA 骞宠　銆?|
| NUMA_MIGRATION | bool | 鏀寔灏嗛〉杩佺Щ鍒板叾浠?NUMA 鑺傜偣锛岀敤鎴风┖闂村彲閫氳繃 migrate_pages()銆乵ove_pages() 涓?mbind() 绛夋帴鍙ｄ娇鐢ㄣ€傞€夋嫨姝ら€夐」杩樺惎鐢ㄥ椤碘€︹€︾殑鏀寔 |
| OBJTOOL | bool | 鍦ㄩ亣鍒?objtool 璀﹀憡鏃朵娇鏋勫缓澶辫触銆俹bjtool 璀﹀憡鍙兘鎸囩ず鍐呮牳涓嶇ǔ瀹氾紝鍖呮嫭寮曞澶辫触銆傚己鐑堝缓璁閫夐」銆傝嫢涓嶇‘瀹氾紝閫?Y銆?|
| OF_RECONFIG_NOTIFIER_ERROR_INJECT | tristate | 姝ら€夐」鎻愪緵鍚?OF 閲嶉厤缃€氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠閫氳繃 /sys/kernel/debug/notifier-error-inject/OF-re鈥︹€︿笅鐨?debugfs 鎺ュ彛鎺у埗 |
| OVERFLOW_KUNIT_TEST | tristate | 鏋勫缓閽堝 check_*_overflow()銆乻ize_*()銆佸垎閰嶅強鐩稿叧鍑芥暟鐨勫崟鍏冩祴璇曘€傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 KUnit 鏂囨。鈥︹€?|
| PACKING_KUNIT_TEST | tristate | 鏋勫缓閽堝 packing 搴撶殑 KUnit 娴嬭瘯銆傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/ 涓殑 KUnit 鏂囨。銆傝嫢鏈夌枒闂紝閫夆€︹€?|
| PAGE_COUNTER | bool | 姝ら€夐」榛樿鍚敤 "favordynmods" 鎸傝浇閫夐」锛屽畠浠ラ檷浣庣儹璺緞鈥︹€︿负浠ｄ环锛屽噺灏戜簡浠诲姟杩佺Щ涓庢帶鍒跺櫒寮€鍏崇瓑鍔ㄦ€?cgroup 淇敼鐨勫欢杩?|
| PAGE_IDLE_FLAG | bool | 杩欏悜 'struct page' 娣诲姞 PG_idle 涓?PG_young 鏍囧織銆侾TE Accessed 浣嶇殑鍐欏叆鑰呭彲浠ヨ缃爣蹇椾腑浣嶇殑鐘舵€侊紝浣?PTE Accessed 浣嶇殑璇诲彇鑰呭彲浠ラ伩鍏嶅共鎵般€?|
| PAGE_MAPCOUNT | def_bool | 杩欏惎鐢ㄨ繛缁唴瀛樺垎閰嶅櫒锛圕ontiguous Memory Allocator锛夛紝鍏佽鍏朵粬瀛愮郴缁熷垎閰嶅ぇ鐨勭墿鐞嗚繛缁唴瀛樺潡銆侰MA 淇濈暀涓€鍧楀唴瀛樺尯鍩燂紝骞跺彧鍏佽鍙Щ鍔ㄩ〉鈥︹€?|
| PAHOLE_HAS_BTF_TAG | def_bool | 鍐冲畾 pahole 鏄惁鍙戝嚭 btf_tag 灞炴€э紙btf_type_tag 涓?btf_decl_tag锛夈€傜洰鍓嶅彧鏈?clang 缂栬瘧鍣ㄥ疄鐜颁簡杩欎簺灞炴€э紝鍥犳浣胯閰嶇疆渚濊禆浜?CC_IS_CLANG銆?|
| PAHOLE_HAS_LANG_EXCLUDE | def_bool | 鏀寔 --lang_exclude 鏍囧織锛屼娇 pahole 鎺掗櫎鏉ヨ嚜鎵€鎻愪緵璇█鐨勭紪璇戝崟鍏冦€傚湪 Kbuild 涓敤浜庣渷鐣?pahole 1.24 鐗堟湰涓嶆敮鎸佺殑 Rust CU鈥︹€?|
| PANIC_ON_OOPS | bool | 鍦ㄦ閫?Y 浠ヨ鍐呮牳鍦?oops 鏃?panic銆傝繖涓庡湪鍐呮牳鍛戒护琛岃缃?oops=panic 鏁堟灉鐩稿悓銆傝鐗规€ф湁鍔╀簬纭繚鍐呮牳涓嶆墽琛屼换浣曗€︹€?|
| PANIC_TIMEOUT | int | 璁剧疆鍐呮牳 panic 鍚庡埌鍙戠敓閲嶅惎鐨勮秴鏃跺€硷紙绉掞級銆傝嫢 n = 0锛屽垯姘歌繙绛夊緟銆俷 > 0 鐨勮秴鏃跺€煎皢绛夊緟 n 绉掑悗閲嶅惎锛岃€?n鈥︹€?|
| PC104 | bool | 鏆撮湶鍙緵閫夋嫨涓庨厤缃殑 PC/104 瑙勬牸璁惧椹卞姩绋嬪簭涓庨€夐」銆傝嫢浣犵殑鐩爣鏈哄櫒鍏锋湁 PC/104 鎬荤嚎锛屽惎鐢ㄦ閫夐」銆?|
| PCPU_DEV_REFCNT | bool | 鑻ヨ缃閫夐」锛岀綉缁滆澶囧紩鐢ㄨ鏁板皢浣跨敤 per cpu 鍙橀噺銆傚彲寮哄埗涓?N 浠ユ娴嬩笅婧紙浼撮殢鎬ц兘涓嬮檷锛夈€?|
| PCSPKR_PLATFORM | bool | 姝ら€夐」鍏佽绂佺敤鍐呯疆 PC 鎵０鍣ㄦ敮鎸侊紝鑺傜渷涓€浜涘唴瀛樸€?|
| PERCPU_STATS | bool | 璇ョ壒鎬ч€氳繃 debugfs 鏀堕泦骞舵毚闇茬粺璁′俊鎭€傝繖浜涗俊鎭寘鎷叏灞€涓庢瘡鍧楃粺璁★紝鍙敤浜庡府鍔╃悊瑙?percpu 鍐呭瓨浣跨敤銆?|
| PERCPU_TEST | tristate | 鍚敤姝ら€夐」浠ユ瀯寤洪獙璇?per-cpu 鎿嶄綔鐨勬祴璇曟ā鍧椼€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| PERF_EVENTS | bool | 鍚敤鍐呮牳瀵瑰悇绉嶇敱杞欢涓庣‖浠舵彁渚涚殑鎬ц兘浜嬩欢鐨勬敮鎸併€傝蒋浠朵簨浠朵互鍐呯疆鏂瑰紡鎴栭€氳繃閫氱敤璺熻釜鐐规敮鎸併€傚ぇ澶氭暟鐜颁唬 CPU 鏀寔鈥︹€?|
| PHYS_ADDR_T_64BIT | def_bool | 鍚敤鍐呮牳鍚岄〉鍚堝苟锛圞ernel Samepage Merging锛夛細KSM 鍛ㄦ湡鎬ф壂鎻忓簲鐢ㄥ湴鍧€绌洪棿涓簲鐢ㄥ缓璁彲鑳藉彲鍚堝苟鐨勫尯鍩熴€傚綋瀹冩壘鍒板唴瀹圭浉鍚岀殑椤垫椂锛屼細灏嗗叾鏇挎崲鈥︹€?|
| PID_NS | bool | 鏀寔杩涚▼ ID 鍛藉悕绌洪棿銆傚彧瑕佽繘绋嬪浜庝笉鍚岀殑 pid 鍛藉悕绌洪棿涓紝灏卞厑璁稿瓨鍦ㄥ涓叿鏈夌浉鍚?pid 鐨勮繘绋嬨€傝繖鏄鍣ㄧ殑鏋勫缓妯″潡銆?|
| PM_NOTIFIER_ERROR_INJECT | tristate | 姝ら€夐」鎻愪緵鍚?PM 閫氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠閫氳繃 /sys/kernel/debug/notifier-error-inject/pm 涓嬬殑 debugfs 鎺ュ彛鎺у埗銆傝嫢閫氱煡閾锯€︹€?|
| POSIX_MQUEUE_SYSCTL | bool | 杩欐槸涓€涓€氱敤閫氱煡闃熷垪锛屼緵鍐呮牳閫氳繃灏嗕簨浠舵嫾鎺ヨ繘绠￠亾鏉ヤ紶閫掔粰鐢ㄦ埛绌洪棿銆傚畠鍙笌鐢ㄤ簬瀵嗛挜/瀵嗛挜鐜彉鏇撮€氱煡鐨勭洃瑙嗏€︹€﹂厤鍚堜娇鐢?|
| POSIX_TIMERS | bool | 杩欎负鍐呮牳鍖呭惈瀵?POSIX 瀹氭椂鍣ㄧ殑鍘熺敓鏀寔銆傛煇浜涘祵鍏ュ紡绯荤粺鐢ㄤ笉鍒板畠浠紝鍥犳鍙互灏嗗叾閰嶇疆鎺変互鍑忓皬鍐呮牳闀滃儚浣撶Н銆傚綋姝ら€夐」鈥︹€?|
| PREEMPT_NOTIFIERS | bool | 鏋勫缓涓€涓畝鍗曠殑 ASN.1 璇硶缂栬瘧鍣紝鐢熸垚鍙敱 ASN.1 娴佽В鐮佸櫒瑙ｉ噴鐨勫瓧鑺傜爜杈撳嚭锛屽苟鐢ㄤ簬鍛婄煡瀹冨湪娴佷腑棰勬湡鍑虹幇鍝簺鏍囩浠ュ強鈥︹€?|
| PRIME_NUMBERS_KUNIT_TEST | tristate | 姝ら€夐」鍚敤閽堝 {is,next}_prime_number 鍑芥暟鐨?KUnit 娴嬭瘯濂椾欢銆傚惎鐢ㄦ閫夐」灏嗗寘鍚皢杩欎簺绱犳暟鐢熸垚鍑芥暟涓庢毚鍔涘疄鐜扳€︹€﹁繘琛屽姣旂殑娴嬭瘯 |
| PRINTK | bool | 姝ら€夐」鍚敤甯歌 printk 鏀寔銆傜Щ闄ゅ畠浼氫粠鍐呮牳闀滃儚涓秷闄ゅぇ閮ㄥ垎娑堟伅瀛楃涓诧紝浣垮唴鏍告垨澶氭垨灏戜繚鎸侀潤榛樸€傜敱浜庤繖浼氫娇璇婃柇鈥︹€﹂潪甯稿洶闅?|
| PRINTK_CALLER | bool | 閫夋嫨姝ら€夐」浼氫娇 printk() 涓烘瘡鏉℃秷鎭坊鍔犺皟鐢ㄨ€呪€渢hread id鈥濓紙鑻ュ湪浠诲姟涓婁笅鏂囦腑锛夋垨璋冪敤鑰呪€減rocessor id鈥濓紙鑻ヤ笉鍦ㄤ换鍔′笂涓嬫枃涓級銆傛閫夐」闈㈠悜鈥︹€︾殑鐜 |
| PRINTK_EXECUTION_CTX | bool | 姝ら€夐」鎵╁睍 struct printk_info锛屼互鍦?printk 涓寘鍚澶栫殑鎵ц涓婁笅鏂囷紝渚嬪娑堟伅鏉ユ簮鐨勪换鍔″悕涓?CPU 缂栧彿銆傝繖鏈夊姪浜庡叧鑱?printk 娑堟伅鈥︹€?|
| PRINTK_INDEX | bool | 娣诲姞瀵圭紪璇戞椂宸茬煡鐨勬墍鏈?printk 鏍煎紡鍦?<debugfs>/printk/index/<module> 澶勫缓绔嬬储寮曠殑鏀寔銆傝繖鍙敤浜庣淮鎶ょ洃鎺?/dev/kmsg 鐨勫畧鎶よ繘绋嬶紝鍥犱负瀹冨厑璁稿璁♀€︹€?|
| PRINTK_RINGBUFFER_KUNIT_TEST | tristate | 鏋勫缓 printk 鐜舰缂撳啿鍖?KUnit 娴嬭瘯濂椾欢銆傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 KUnit 鏂囨。銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| PRINTK_TIME | bool | 閫夋嫨姝ら€夐」浼氫娇 printk() 娑堟伅鐨勬椂闂存埑琚坊鍔犲埌 syslog() 绯荤粺璋冪敤鐨勮緭鍑轰互鍙婃帶鍒跺彴杈撳嚭涓€傛椂闂存埑濮嬬粓鍦ㄥ唴閮ㄨ褰曪紝骞跺鍑衡€︹€?|
| PROC_MEM_ALWAYS_FORCE | bool | 鑻ヤ綘鍏锋湁 ptrace 璁块棶鏉冮檺锛岃繖鍏佽 /proc/pid/mem 璁块棶瑕嗙洊鍐呭瓨鏄犲皠鏉冮檺銆?|
| PROC_MEM_FORCE_PTRACE | bool | 杩欏厑璁?/proc/pid/mem 璁块棶涓?gdb 绛夋椿璺冪殑 ptracer 瑕嗙洊鍐呭瓨鏄犲皠鏉冮檺銆?|
| PROC_MEM_NO_FORCE | bool | 姘歌繙涓嶈鐩栧唴瀛樻槧灏勬潈闄?|
| PROC_PID_CPUSET | bool | 鎻愪緵涓€涓?cgroup 鎺у埗鍣紝涓?cgroup 涓殑杩涚▼鍙互 mknod 鎴栨墦寮€鐨勮澶囧疄鐜扮櫧鍚嶅崟銆?|
| PROFILING | bool | 鍦ㄦ閫?Y 浠ュ惎鐢ㄦ€ц兘鍒嗘瀽鍣ㄦ墍浣跨敤鐨勬墿灞曟€ц兘鍒嗘瀽鏀寔鏈哄埗銆?|
| PROVE_RAW_LOCK_NESTING | bool | 鍚敤 raw_spinlock 涓?spinlock 宓屽妫€鏌ワ紝浠ョ‘淇濅笉杩濆弽涓?PREEMPT_RT 鍚敤鍐呮牳鐨勯攣宓屽瑙勫垯銆?|
| PROVIDE_OHCI1394_DMA_INIT | bool | 鑻ヤ綘鎯宠皟璇曞湪寮曞鏃╂湡鎸傝捣鎴栧穿婧冨唴鏍哥殑闂锛屼笖宕╂簝鐨勬満鍣ㄥ叿鏈?FireWire 绔彛锛屼綘鍙互浣跨敤姝ょ壒鎬ц繙绋嬭闂穿婧冩満鍣ㄧ殑鍐呭瓨鈥︹€?|
| PSI | bool | 鏀堕泦鎸囩ず绯荤粺涓?CPU銆佸唴瀛樹笌 IO 瀹归噺瓒呴厤绋嬪害鐨勬寚鏍囥€傝嫢鍦ㄦ閫?Y锛屽唴鏍稿皢鍒涘缓 /proc/pressure/锛屽叾涓寘鍚帇鍔涚粺璁℃枃浠?cpu銆佲€︹€?|
| PSI_DEFAULT_DISABLED | bool | 鑻ヨ缃紝鍘嬪姏鍋滈】淇℃伅璺熻釜榛樿绂佺敤锛屼絾鍙€氳繃鍦ㄥ紩瀵兼椂浜庡唴鏍稿懡浠よ浼犻€?psi=1 鏉ュ惎鐢ㄣ€傝鐗规€у悜浠诲姟鍞ら啋鈥︹€︽坊鍔犱竴浜涗唬鐮?|
| PTE_MARKER_UFFD_WP | bool | 鍏佽涓?userfaultfd 鍐欎繚鎶ょ洰鐨勫垱寤烘爣璁?PTE銆傝鍦?shmem 涓?hugetlbfs 绛夋枃浠跺悗澶囧唴瀛樼被鍨嬩笂鍚敤 userfaultfd 鍐欎繚鎶わ紝闇€瑕佸畠銆?|
| RANDOM_KMALLOC_CACHES | bool | 涓€椤瑰姞鍥虹壒鎬э紝涓烘櫘閫?kmalloc 鍒嗛厤鍒涘缓 slab 缂撳瓨鐨勫涓壇鏈紝骞惰 kmalloc 鍩轰簬浠ｇ爜鍦板潃闅忔満閫夊彇涓€涓紝浣挎敾鍑昏€呮洿闅锯€︹€?|
| RANDSTRUCT_KUNIT_TEST | tristate | 鏋勫缓鐢ㄤ簬妫€鏌?CONFIG_RANDSTRUCT=y锛堥殢鏈哄寲缁撴瀯浣撳竷灞€锛夌殑鍗曞厓娴嬭瘯銆?|
| RATELIMIT_KUNIT_TEST | tristate | 鏋勫缓鈥渢est_ratelimit鈥濇ā鍧楋紝鐢ㄤ簬閫熺巼闄愬埗鐨勬纭€ч獙璇佷笌骞跺彂娴嬭瘯銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| RATIONAL_KUNIT_TEST | tristate | 鏋勫缓鏈夌悊鏁版暟瀛﹀崟鍏冩祴璇曘€傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/ 涓殑 KUnit 鏂囨。銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| RBTREE_TEST | tristate | 涓€涓祴閲?rbtree 搴撴€ц兘鐨勫熀鍑嗘祴璇曘€備篃鍖呮嫭 rbtree 涓嶅彉寮忔鏌ャ€?|
| READABLE_ASM | bool | 绂佺敤涓€浜涘€惧悜浜庣敓鎴愪汉绫讳笉鍙姹囩紪杈撳嚭鐨勭紪璇戝櫒浼樺寲銆傝繖鍙兘浣垮唴鏍哥◢鎱紝浣嗘湁鍔╀簬璁╅渶瑕佸ぇ閲忊€︹€︾殑鍐呮牳寮€鍙戣€?|
| READ_ONLY_THP_FOR_FS | bool | 鍏佽 khugepaged 灏嗗彧璇荤殑鏂囦欢鍚庡椤垫斁鍏?THP銆傝繖琚爣璁颁负瀹為獙鎬э紝鍥犱负瀹冩槸涓€椤规柊鐗规€с€傛枃浠?THP 鐨勫啓鍏ユ敮鎸佸皢鍦ㄦ帴涓嬫潵鐨勫嚑涓彂甯冨懆鏈熶腑寮€鍙戙€?|
| REED_SOLOMON_TEST | tristate | 姝ら€夐」鍦ㄥ紩瀵兼椂鎴栨ā鍧楀姞杞芥椂鍚敤 rslib 鐨勮嚜妫€娴嬭瘯鍑芥暟銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| RELAY | bool | 姝ら€夐」鍦ㄦ煇浜涙枃浠剁郴缁燂紙濡?debugfs锛変腑鍚敤 relay 鎺ュ彛鏀寔銆傚畠鏃ㄥ湪涓哄伐鍏蜂笌璁炬柦鎻愪緵涓€绉嶉珮鏁堟満鍒讹紝浠ヤ紶杈撳ぇ閲忊€︹€?|
| RESOURCE_KUNIT_TEST | tristate | 鏋勫缓璧勬簮 API 鍗曞厓娴嬭瘯銆傛祴璇?resource.c 涓?ioport.h 鎻愪緵鐨?API 閫昏緫銆傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 KUnit 鏂囨。鈥︹€?|
| RFS_ACCEL | bool | 鍏佽鍏锋湁娴佽繃婊よ〃鐨?multiqueue 纭欢鐨勯┍鍔ㄥ姞閫?RFS銆?|
| RPS | bool | 杞欢鎺ユ敹绔暟鎹寘瀵煎悜锛圧PS锛夊皢鎺ユ敹鏁版嵁鍖呭鐞嗙殑璐熻浇鍒嗗竷鍒板涓?CPU 涓娿€?|
| RSEQ | bool | 鍚敤鍙噸鍚簭鍒楋紙restartable sequences锛夌郴缁熻皟鐢ㄣ€傚畠鎻愪緵涓€涓敤鎴风┖闂寸紦瀛樹互瀛樻斁褰撳墠 CPU 缂栧彿鍊硷紝鍔犻€熶粠鐢ㄦ埛绌洪棿鑾峰彇褰撳墠 CPU 缂栧彿锛屽苟鎻愪緵涓€涓?ABI 浠モ€︹€?|
| RSEQ_DEBUG_DEFAULT_ENABLE | bool | 杩欏惎鐢ㄥ彲閲嶅惎搴忓垪璋冭瘯妯″紡鐨勯潤鎬佸垎鏀€傚畠涔熷彲閫氳繃鍐呮牳鍛戒护琛屽弬鏁?"rseq_debug=0/1" 浠ュ強閫氳繃 debugfs 鎺у埗銆傝嫢鈥︹€?|
| RSEQ_SLICE_EXTENSION | bool | 鍏佽鐢ㄦ埛绌洪棿閫氳繃 RSEQ 鍏变韩鏁版嵁 ABI锛屽湪浠庝腑鏂繑鍥炵敤鎴风┖闂存椂璇锋眰鏈夐檺鐨勬椂闂寸墖寤堕暱銆傝嫢鑾峰噯锛屽嵆鍙畬鎴愪竴涓复鐣屽尯锛屼粠鑰屸€︹€?|
| RSEQ_STATS | bool | 鍚敤杞婚噺绾ц鏁板櫒锛岄€氳繃 debugfs 鏆撮湶鍏充簬 RSEQ 鎿嶄綔棰戠巼鐨勪俊鎭€備富瑕佺敤浜庡唴鏍歌皟璇曟垨鎬ц兘鍒嗘瀽銆傝櫧鐒惰交閲忥紝瀹冧粛鈥︹€?|
| RT_GROUP_SCHED | bool | 璇ョ壒鎬ц浣犲彲浠ユ樉寮忓湴灏嗙湡瀹?CPU 甯﹀鍒嗛厤缁欎换鍔＄粍銆傝嫢鍚敤锛屽湪涓洪潪 root 鐢ㄦ埛鍒嗛厤瀹炴椂甯﹀鈥︹€︿箣鍓嶏紝灏嗘棤娉曚负闈?root 鐢ㄦ埛璋冨害瀹炴椂浠诲姟 |
| RT_GROUP_SCHED_DEFAULT_DISABLED | bool | 褰撹缃椂锛孯T 缁勮皟搴﹂粯璁ょ鐢ㄣ€傝閫夐」閲囩敤鍙嶅悜褰㈠紡锛屽洜姝ゅ崟绾殑 RT_GROUP_SCHED 鍗冲惎鐢ㄧ粍璋冨害銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| RUNTIME_TESTING_MENU | bool | 鍚敤姝ら」浠ュ寘鍚?Dhrystone 2.1 鍩哄噯娴嬭瘯銆傝娴嬭瘯璁＄畻姣忕鐨?Dhrystones 鏁伴噺锛屼互鍙婂皢 Dhrystone 鍒嗘暟闄や互鈥︹€﹀悗寰楀埌鐨?DMIPS锛圖hrystone MIPS锛夋暟閲?|
| RUST | bool | 鍚敤鍐呮牳涓殑 Rust 鏀寔銆傝繖鍏佽閫夋嫨鍏朵粬 Rust 鐩稿叧閫夐」锛屽鐢?Rust 缂栧啓鐨勯┍鍔ㄣ€傝鑳藉鍔犺浇鐢?Rust 缂栧啓澶栭儴鍐呮牳妯″潡涔熼渶瑕佸畠鈥︹€?|
| RUSTC_LLVM_VERSION | int | 杩欐寚绀?Rust 涓?Clang 鏄惁浣跨敤鐩稿悓涓荤増鏈殑 LLVM銆傛秹鍙婂鐞?LLVM IR 鎴?bitcode锛堝璺ㄨ瑷€ LTO锛夌殑鎿嶄綔闇€瑕佺浉鍚屼富鐗堟湰鐨?LLVM 鎵嶈兘姝ｅ父宸ヤ綔鈥︹€?|
| RUSTC_VERSION_TEXT | string | 鍙傝 `CC_VERSION_TEXT`銆?|
| RUST_BUILD_ASSERT_ALLOW | bool | 鎺у埗鍦ㄦ瀯寤烘湡闂村浣曞鐞?`build_error!` 涓?`build_assert!`銆傚鏋滀簩杩涘埗涓瓨鍦ㄥ瀹冧滑鐨勮皟鐢紝鍙兘鎸囩ず琚繚鍙嶇殑涓嶅彉閲忔垨浼樺寲鍣ㄦ湭鑳介獙璇佽涓嶅彉閲忊€︹€?|
| RUST_DEBUG_ASSERTIONS | bool | 鍚敤 rustc 鐨?`-Cdebug-assertions` codegen 閫夐」銆傝鏍囧織鍙浣犲紑鍚垨鍏抽棴 `cfg(debug_assertions)` 鏉′欢缂栬瘧銆傝繖鍙敤浜庡湪寮€鍙戔€︹€︿腑鍚敤棰濆鐨勮皟璇曚唬鐮?|
| RUST_INLINE_HELPERS | bool | 浣跨敤閾炬帴鏃朵紭鍖栵紙LTO锛夊皢 C 杈呭姪鍑芥暟鍐呰仈鍒?Rust 浠ｇ爜涓€傝嫢鍚敤姝ら€夐」锛宺ust/helpers/ 涓０鏄庣殑 C 杈呭姪鍑芥暟灏嗚鍐呰仈鍒?Rust 浠ｇ爜涓紝杩欐湁鍔╀簬鎬ц兘鈥︹€?|
| RUST_IS_AVAILABLE | def_bool | 杩欐樉绀烘槸鍚︽湁鍚堥€傜殑 Rust 宸ュ叿閾惧彲鐢紙宸叉壘鍒帮級銆傛湁鍏冲浣曟弧瓒?Rust 鏀寔鐨勬瀯寤鸿姹傜殑璇存槑锛岃鍙傞槄 Documentation/rust/quick-start.rst銆傜壒鍒€︹€?|
| RUST_KERNEL_DOCTESTS | bool | 杩欏皢 `kernel` crate 鐨勬枃妗ｆ祴璇曟瀯寤轰负 KUnit 娴嬭瘯銆傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools鈥︹€︿腑鐨?KUnit 鏂囨。 |
| RUST_OVERFLOW_CHECKS | bool | 鍚敤 rustc 鐨?`-Coverflow-checks` codegen 閫夐」銆傝鏍囧織鍏佽浣犳帶鍒惰繍琛屾椂鏁存暟婧㈠嚭鐨勮涓恒€傚綋鍚敤婧㈠嚭妫€鏌ユ椂锛屾孩鍑哄皢寮曞彂 Rust panic鈥︹€?|
| SCANF_KUNIT_TEST | tristate | 鍚敤姝ら€夐」浠ュ湪杩愯鏃舵祴璇?scanf 鍑芥暟銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| SCF_TORTURE_TEST | tristate | 姝ら€夐」鎻愪緵涓€涓唴鏍告ā鍧楋紝瀵?smp_call_function() 绯诲垪鍘熻杩愯 torture 娴嬭瘯銆傝嫢闇€瑕侊紝璇ユā鍧楀彲鍦ㄨ娴嬬殑杩愯涓唴鏍镐笂浜嬪悗鏋勫缓銆傝嫢鈥︹€?|
| SCHED_AUTOGROUP | bool | 姝ら€夐」閫氳繃鑷姩鍒涘缓骞跺～鍏呬换鍔＄粍锛屼负甯歌妗岄潰宸ヤ綔璐熻浇浼樺寲璋冨害鍣ㄣ€傝繖绉嶅伐浣滆礋杞界殑鍒嗙闅旂浜嗘縺杩涚殑 CPU 娑堣€楄€咃紙濡傛瀯寤轰綔涓氣€︹€︼級 |
| SCHED_INFO | bool | 鑻ュ湪姝ら€?Y锛屽皢鍦ㄨ皟搴﹀櫒鍙婄浉鍏充緥绋嬩腑鎻掑叆棰濆浠ｇ爜锛屼互鏀堕泦璋冨害鍣ㄨ涓虹殑缁熻淇℃伅骞堕€氳繃 /proc/schedstat 鎻愪緵銆傝繖浜涚粺璁″彲鑳解€︹€?|
| SCHED_PROXY_EXEC | bool | 姝ら€夐」鍚敤浠ｇ悊鎵ц锛坧roxy execution锛夛紝涓€绉嶈鎸佹湁浜掓枼浣撶殑浠诲姟缁ф壙鏇撮珮浼樺厛绾х瓑寰呰€呰皟搴︿笂涓嬫枃鐨勬満鍒躲€?|
| SCHED_STACK_END_CHECK | bool | 姝ら€夐」妫€鏌ュ schedule() 璋冪敤鏃剁殑鏍堟孩鍑恒€傚鏋滃彂鐜版爤鏈熬浣嶇疆琚鐩栵紝鍒欐€绘槸 panic锛屽洜涓鸿鐮村潖鍖哄煙鐨勫唴瀹瑰凡涓嶅啀鍙俊銆傝鈥︹€?|
| SECTION_MISMATCH_WARN_ONLY | bool | 鑻ュ湪姝ら€?N锛屾瀯寤鸿繃绋嬪皢鍦ㄥ嚭鐜颁换浣曟涓嶅尮閰嶆椂澶辫触锛岃€岄潪浠呬粎鎶涘嚭璀﹀憡銆傝嫢涓嶇‘瀹氾紝閫?Y銆?|
| SECURITY | bool | 杩欏厑璁镐綘閫夋嫨灏嗕笉鍚岀殑瀹夊叏妯″潡閰嶇疆杩涘唴鏍搞€傝嫢鏈€夋嫨姝ら€夐」锛屽皢浣跨敤榛樿 Linux 瀹夊叏妯″瀷銆傝嫢浣犱笉纭畾濡備綍鍥炵瓟鈥︹€?|
| SECURITY_COMMONCAP_KUNIT_TEST | bool | 鏋勫缓 commoncap KUnit 娴嬭瘯銆侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞朵互 TAP 鏍煎紡锛坔ttps://testanything.org/锛夊皢缁撴灉杈撳嚭鍒拌皟璇曟棩蹇椼€備粎瀵硅繍琛?KUnit 娴嬭瘯鐨勫唴鏍稿紑鍙戣€呮湁鐢ㄢ€︹€?|
| SECURITY_DMESG_RESTRICT | bool | 杩欏己鍒堕檺鍒堕潪鐗规潈鐢ㄦ埛閫氳繃 dmesg(8) 璇诲彇鍐呮牳 syslog銆傝嫢鏈€夋嫨姝ら€夐」锛岄櫎闈?dmesg_restrict sysctl 琚樉寮忊€︹€﹀惁鍒欎笉寮哄埗浠讳綍闄愬埗 |
| SECURITY_INFINIBAND | bool | 杩欏惎鐢?Infiniband 瀹夊叏閽╁瓙銆傝嫢鍚敤锛屽畨鍏ㄦā鍧楀彲浣跨敤杩欎簺閽╁瓙瀹炵幇 Infiniband 璁块棶鎺у埗銆傝嫢浣犱笉纭畾濡備綍鍥炵瓟锛岄€?N銆?|
| SECURITY_NETWORK | bool | 杩欏惎鐢ㄥ鎺ュ瓧涓庣綉缁滃畨鍏ㄧ殑閽╁瓙銆傝嫢鍚敤锛屽畨鍏ㄦā鍧楀彲浣跨敤杩欎簺閽╁瓙瀹炵幇濂楁帴瀛椾笌缃戠粶璁块棶鎺у埗銆傝嫢浣犱笉纭畾濡備綍鍥炵瓟鈥︹€?|
| SECURITY_NETWORK_XFRM | bool | 杩欏惎鐢?XFRM锛圛PSec锛夌綉缁滃畨鍏ㄩ挬瀛愩€傝嫢鍚敤锛屽畨鍏ㄦā鍧楀彲浣跨敤杩欎簺閽╁瓙瀹炵幇鍩轰簬浠?IPSec 绛栫暐娲剧敓鐨勬爣绛剧殑姣忓寘璁块棶鎺у埗銆傞潪 IP鈥︹€?|
| SECURITY_PATH | bool | 杩欏惎鐢ㄥ熀浜庤矾寰勫悕鐨勮闂帶鍒剁殑瀹夊叏閽╁瓙銆傝嫢鍚敤锛屽畨鍏ㄦā鍧楀彲浣跨敤杩欎簺閽╁瓙瀹炵幇鍩轰簬璺緞鍚嶇殑璁块棶鎺у埗銆傝嫢浣犱笉纭畾濡備綍鍥炵瓟鈥︹€?|
| SELECT_MEMORY_MODEL | def_bool | 姝ら€夐」鍏佽浣犳洿鏀?Linux 鍦ㄥ唴閮ㄧ鐞嗗唴瀛樼殑鏌愪簺鏂瑰紡銆傚ぇ澶氭暟鐢ㄦ埛鍙細鐢变綋绯荤粨鏋勯厤缃€変腑鍏朵腑涓€涓€夐」銆傝繖鏄甯哥殑銆?|
| SEQ_BUF_KUNIT_TEST | tristate | 鏋勫缓閽堝 seq_buf 搴撶殑鍗曞厓娴嬭瘯銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| SGETMASK_SYSCALL | bool | sys_sgetmask 涓?sys_ssetmask 鏄凡搴熷純鐨勭郴缁熻皟鐢紝libc 涓嶅啀鏀寔锛屼絾鍦ㄦ煇浜涗綋绯荤粨鏋勪笂榛樿浠嶅惎鐢ㄣ€傝嫢涓嶇‘瀹氾紝淇濈暀姝ゅ鐨勯粯璁ら€夐」銆?|
| SG_POOL | def_bool | 鎻愪緵涓€涓垎閰嶉摼寮忔暎鍒楄〃鐨勮緟鍔╁嚱鏁般€傚簲鐢卞笇鏈涘垎閰嶉摼寮忔暎鍒楄〃鐨勯┍鍔ㄦ垨 API 閫変腑銆? # sg 閾惧紡閫夐」 # |
| SHMEM | bool | shmem 鏄竴涓敤浜庣鐞嗗叡浜唴瀛樼殑鍐呴儴鏂囦欢绯荤粺銆傚畠鐢?swap 鍚庡骞剁鐞嗚祫婧愰檺鍒躲€傝嫢鍚敤 TMPFS锛屽畠涔熻浣滀负 tmpfs 瀵煎嚭鍒扮敤鎴风┖闂淬€傜鐢ㄦ閫夐」鈥︹€?|
| SHRINKER_DEBUG | bool | 閫?Y 浠ュ惎鐢?shrinker 鐨?debugfs 鎺ュ彛锛屽畠鎻愪緵瀵瑰唴鏍稿唴瀛?shrinker 瀛愮郴缁熺殑鍙鎬с€傜鐢ㄥ畠浠ラ伩鍏嶉澶栫殑鍐呭瓨鍗犵敤銆?|
| SHUFFLE_PAGE_ALLOCATOR | bool | 椤靛垎閰嶅櫒鐨勯殢鏈哄寲鏀瑰杽浜嗙洿鎺ユ槧灏勭殑鍐呭瓨渚х紦瀛樼殑骞冲潎鍒╃敤鐜囥€傚弬瑙?ACPI 6.2a 瑙勮寖涓?5.2.27 鑺傚紓鏋勫唴瀛樺睘鎬ц〃锛圚MAT锛夆€︹€?|
| SIGNALFD | bool | 鍚敤 signalfd() 绯荤粺璋冪敤锛屽畠鍏佽鍦ㄦ枃浠舵弿杩扮涓婃帴鏀朵俊鍙枫€傝嫢涓嶇‘瀹氾紝閫?Y銆?|
| SIGNATURE | tristate | 鏁板瓧绛惧悕楠岃瘉銆傜洰鍓嶄粎鏀寔 RSA銆傚疄鐜颁娇鐢?GnuPG MPI 搴撱€?|
| SIPHASH_KUNIT_TEST | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃讹紙鎴栨ā鍧楀姞杞芥椂锛夋祴璇曞唴鏍哥殑 siphash锛?linux/siphash.h>锛夊搱甯屽嚱鏁般€傚畠鏃ㄥ湪甯姪缂栧啓鐗瑰畾浜庝綋绯荤粨鏋勭殑浼樺寲鐗堟湰鐨勪汉銆傝嫢鈥︹€?|
| SLAB_BUCKETS | bool | 鍐呮牳鍫嗘敾鍑诲父甯镐緷璧栦簬鑳藉鍒涘缓鍏锋湁鐢ㄦ埛鍙帶鍐呭銆佷笖浼氳鍒嗛厤鍒颁笌鐩爣瀵硅薄鐩稿悓 kmalloc bucket 鐨勭壒瀹氬ぇ灏忓垎閰嶃€備负鈥︹€?|
| SLAB_FREELIST_HARDENED | bool | 璁稿鍐呮牳鍫嗘敾鍑昏瘯鍥鹃拡瀵?slab 缂撳瓨鍏冩暟鎹笌鍏朵粬鍩虹璁炬柦銆傛閫夐」鍋氬嚭灏戦噺鎬ц兘鐗虹壊锛屼互鍔犲浐鍐呮牳 slab 鍒嗛厤鍣ㄦ姷寰″父瑙佺殑 freelist 鍒╃敤鈥︹€?|
| SLAB_FREELIST_RANDOM | bool | 闅忔満鍖栧垱寤烘柊椤垫椂浣跨敤鐨?freelist 椤哄簭銆傝瀹夊叏鐗规€ч檷浣庝簡鍐呮牳 slab 鍒嗛厤鍣ㄥ鍫嗘孩鍑虹殑鍙娴嬫€с€?|
| SLAB_MERGE_DEFAULT | bool | 涓哄噺灏戝唴鏍稿唴瀛樼鐗囷紝褰?slab 缂撳瓨鍏锋湁鐩稿悓澶у皬涓庡叾浠栫壒寰佹椂鍙鍚堝苟銆傝繖甯︽潵鍐呮牳鍫嗘孩鍑鸿兘澶熻鐩栧璞♀€︹€︾殑椋庨櫓 |
| SLAB_OBJ_EXT | bool | 姝ら€夐」娣诲姞灏嗚繘绋嬮泦鍚堝垎缁勫湪涓€璧风殑鏀寔锛屼緵 Cpusets銆丆FS銆佸唴瀛樻帶鍒舵垨璁惧闅旂绛夎繘绋嬫帶鍒跺瓙绯荤粺浣跨敤銆傚弬瑙?Documentation/scheduler/sc鈥︹€?|
| SLUB | def_bool | 浠ヤ竴绉嶅疄鐜版渶灏忓唴瀛樺崰鐢ㄧ殑鏂瑰紡閰嶇疆 slab 鍒嗛厤鍣紝鐗虹壊鍙墿灞曟€с€佽皟璇曚笌鍏朵粬鐗规€с€傝繖浠呴潰鍚戞浘浣跨敤 SL鈥︹€︾殑鏈€灏忕郴缁?|
| SLUB_KUNIT_TEST | tristate | 鏋勫缓 SLUB 鍒嗛厤鍣ㄥ崟鍏冩祴璇曘€傛祴璇?SLUB 缂撳瓨璋冭瘯鍔熻兘銆傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-鈥︹€︿腑鐨?KUnit 鏂囨。 |
| SLUB_STATS | bool | 杩欎簺缁熻鏈夊姪浜庤皟璇?slab 鍒嗛厤琛屼负锛屼互鎵惧埌浼樺寲鍒嗛厤鍣ㄧ殑鏂规硶銆傜粷涓嶅簲鍦ㄧ敓浜х幆澧冧腑鍚敤锛屽洜涓轰繚瀛樼粺璁′細鎷栨參鏁翠釜鈥︹€?|
| SOCK_CGROUP_DATA | bool | 鎻愪緵璁╀换鍔′娇鐢ㄧ浉鍚?id 涓庝笉鍚屽璞″崗浣滅殑鏂瑰紡銆備緥濡傦紝鐩稿悓鐨?IPC id 鍦ㄤ笉鍚屸€︹€︿腑鍙兘鎸囧悜涓嶅悓瀵硅薄锛屾垨鐩稿悓鐨勭敤鎴?id 鎴?pid 鍙兘鎸囧悜涓嶅悓浠诲姟 |
| SOCK_RX_QUEUE_MAPPING | bool | 鐢ㄤ簬灏嗚繘绋嬫寜姣忎釜鎺ュ彛鍒嗛厤鍒扮綉缁滀紭鍏堢骇鐨?cgroup 瀛愮郴缁熴€?|
| SOFTLOCKUP_DETECTOR_INTR_STORM | bool | 鍦ㄦ閫?Y 浠ヨ鍐呮牳鍦ㄢ€渟oft lockups锛堣蒋閿佹锛夆€濇湡闂存娴嬩腑鏂鏆淬€傗€渟oft lockups鈥濆彲鐢卞绉嶅師鍥犲紩璧枫€傝嫢鍏朵腑涔嬩竴鐢变腑鏂鏆村鑷达紝鍒欓鏆翠腑鐨勪腑鏂€︹€?|
| SPARSEMEM | def_bool | SPARSEMEM_VMEMMAP 浣跨敤铏氭嫙鏄犲皠鐨?memmap 鏉ヤ紭鍖?pfn_to_page 涓?page_to_pfn 鎿嶄綔銆傚綋鍐呮牳璧勬簮鍏呰冻鏃讹紝杩欐槸鏈€楂樻晥鐨勯€夐」銆?|
| SPARSEMEM_MANUAL | bool | 瀵规煇浜涚郴缁燂紙鍖呮嫭鍐呭瓨鐑彃鎷旂郴缁燂級鑰岃█锛岃繖灏嗘槸鍞竴閫夐」銆傝繖鏄甯哥殑銆傛閫夐」涓虹墿鐞嗗湴鍧€绌洪棿涓瓨鍦ㄧ┖娲炩€︹€︾殑绯荤粺鎻愪緵楂樻晥鏀寔 |
| SPARSEMEM_VMEMMAP_PREINIT | bool | 鐑彃鎷斿唴瀛樼殑榛樿鍐呭瓨绫诲瀷銆傛閫夐」璁剧疆鍐呭瓨鐑彃鎷斾笂绾跨瓥鐣ワ紙/sys/devices/system/memory/auto_online_blocks锛夌殑榛樿绛栫暐锛岃绛栫暐鍐冲畾鈥︹€?|
| STACKDEPOT_ALWAYS_INIT | bool | 鍦ㄦ棭鏈熷紩瀵兼湡闂村缁堝垵濮嬪寲 stack depot銆?|
| STACKDEPOT_MAX_FRAMES | int | 杩愯杞婚噺绾ф帓闃熺殑寮曞鏃舵祴璇曘€?|
| STACKINIT_KUNIT_TEST | tristate | 娴嬭瘯鍐呮牳鏄惁瀵规爤鍙橀噺涓庡～鍏呰繘琛岄浂鍒濆鍖栥€傝鐩栬寖鍥寸敱缂栬瘧鍣ㄦ爣蹇?CONFIG_INIT_STACK_ALL_PATTERN 鎴?CONFIG_INIT_STACK_ALL_ZERO 鎺у埗銆?|
| STACKTRACE | bool | 姝ら€夐」浣垮唴鏍镐负姣忎釜杩涚▼鍒涘缓 /proc/pid/stack锛屾樉绀哄叾褰撳墠鏍堣窡韪€傚畠涔熻闇€瑕佺敓鎴愭爤璺熻釜鐨勫悇绉嶅唴鏍歌皟璇曠壒鎬т娇鐢ㄣ€?|
| STACKTRACE_BUILD_ID | bool | 閫夋嫨姝ら€夐」浼氫负浠?printk 鏍煎紡 '%p[SR]b' 鎵撳嵃鐨勬爤璺熻釜涓殑绗﹀彿娣诲姞 build ID 淇℃伅銆傛閫夐」闈㈠悜 debuginfo 涓嶆槗鑾峰彇鈥︹€︾殑鍙戣鐗?|
| STACK_VALIDATION | bool | 鍦ㄧ紪璇戞椂楠岃瘉甯ф寚閽堣鍒欍€傝繖鏈夊姪浜庣‘淇濊繍琛屾椂鏍堣窡韪洿鍙潬銆傛洿澶氫俊鎭鍙傞槄 tools/objtool/Documentation/objtool.txt銆?|
| STATIC_USERMODEHELPER | bool | 榛樿鎯呭喌涓嬶紝鍐呮牳鍙€氳繃鈥渦sermode helper锛堢敤鎴锋€佽緟鍔╋級鈥濆唴鏍告帴鍙ｈ皟鐢ㄨ澶氫笉鍚岀殑鐢ㄦ埛绌洪棿浜岃繘鍒剁▼搴忋€傚叾涓竴浜涗簩杩涘埗瑕佷箞鍦ㄢ€︹€︿腑闈欐€佸畾涔?|
| STATIC_USERMODEHELPER_PATH | string | 褰撲换浣?usermode helper 绋嬪簭甯屾湜杩愯鏃讹紝鍐呮牳璋冪敤鐨勪簩杩涘埗銆傝浼犻€掔殑鈥滅湡瀹炩€濆簲鐢ㄥ悕灏嗗湪鍛戒护琛屼紶缁欒绋嬪簭鐨勭涓€涓弬鏁颁腑銆傝嫢浣犫€︹€?|
| STRING_KUNIT_TEST | tristate | 鍚敤瀵瑰瓧绗︿覆鍑芥暟鐨勬€ц兘娴嬮噺銆傚畠鍦?KUnit 娴嬭瘯杩愯鏈熼棿娴嬮噺瀛楃涓插嚱鏁扮殑鎵ц鏁堢巼銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| STRIP_ASM_SYMS | bool | 鍦ㄩ摼鎺ユ湡闂村墺绂绘眹缂栧櫒鐢熸垚鐨勫唴閮ㄧ鍙凤紙褰㈠ '.Lxxx' 鐨勭鍙凤級锛屼互鍏嶅畠浠薄鏌?get_wchan() 绛夌殑杈撳嚭銆?|
| SYMBOLIC_ERRNAME | bool | 鑻ュ湪姝ら€?Y锛屽唴鏍哥殑 printf 瀹炵幇灏嗚兘澶熸墦鍗扮鍙峰寲鐨勯敊璇悕锛堝 ENOSPC锛夎€岄潪鏁板瓧 28銆傝繖浼氫娇鍐呮牳闀滃儚鐣ュぇ锛堢害 3KB锛夛紝浣嗏€︹€?|
| SYSCTL_ARCH_UNALIGN_ALLOW | bool | 鍚敤瀵?/proc/sys/kernel/unaligned-trap 鐨勬敮鎸併€傚厑璁镐綋绯荤粨鏋勫畾涔?浣跨敤 @unaligned_enabled 鍦ㄨ繍琛屾椂鍒囨崲鏈榻愯闂ā鎷熴€傚弬鑰?arch/parisc/kernel/unaligned.c |
| SYSCTL_ARCH_UNALIGN_NO_WARN | bool | 鍚敤瀵?/proc/sys/kernel/ignore-unaligned-usertrap 鐨勬敮鎸併€傚厑璁镐綋绯荤粨鏋勫畾涔?浣跨敤 @no_unaligned_warning 浠ュ氨鍙兘鍙戠敓鐨勬湭瀵归綈璁块棶妯℃嫙鍙戝嚭璀﹀憡銆?|
| SYSCTL_KUNIT_TEST | tristate | 鏋勫缓 proc sysctl 鍗曞厓娴嬭瘯锛屽湪寮曞鏃惰繍琛屻€傛祴璇?sysctl 鐨?API 濂戠害涓庡疄鐜版纭€с€傛湁鍏?KUnit 涓庡崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄鈥︹€?|
| SYSFS_SYSCALL | bool | sys_sysfs 鏄竴涓凡搴熷純鐨勭郴缁熻皟鐢紝libc 涓嶅啀鏀寔銆傛敞鎰忕鐢ㄦ閫夐」鏇村畨鍏紝浣嗗彲鑳界牬鍧忎笌鏌愪簺绯荤粺鐨勫吋瀹规€с€傝嫢涓嶇‘瀹氾紝鍦ㄦ閫?N銆?|
| SYSTEM_DATA_VERIFICATION | def_bool | 浣跨敤绯荤粺鍙俊瀵嗛挜鐜殑鍐呭鎻愪緵 PKCS#7 娑堟伅楠岃瘉浠ヤ緵缁欏叕閽ャ€傝繖闅忓悗鍙敤浜庢ā鍧楅獙璇併€乲exec 闀滃儚楠岃瘉涓庡浐浠垛€︹€?|
| SYSVIPC | bool | 杩涚▼闂撮€氫俊锛圛nter Process Communication锛夋槸涓€濂楀簱鍑芥暟涓庣郴缁熻皟鐢紝璁╄繘绋嬶紙杩愯涓殑绋嬪簭锛夊悓姝ュ苟浜ゆ崲淇℃伅銆傚畠閫氬父琚涓烘槸涓€浠跺ソ浜嬧€︹€?|
| SYSVIPC_SYSCTL | bool | POSIX 娑堟伅闃熷垪鏄?IPC 鐨勪竴閮ㄥ垎銆傚湪 POSIX 娑堟伅闃熷垪涓紝姣忔潯娑堟伅閮芥湁涓€涓紭鍏堢骇锛屽喅瀹氳繘绋嬫帴鏀跺畠鐨勯『搴忋€傝嫢浣犳兂缂栬瘧骞惰繍琛屸€︹€?|
| TASKSTATS | bool | 閫氳繃閫氱敤 netlink 鎺ュ彛瀵煎嚭浠诲姟/杩涚▼鐨勯€夊畾缁熻淇℃伅銆備笌 BSD 杩涚▼璁拌处涓嶅悓锛岃繖浜涚粺璁″湪浠诲姟/杩涚▼鐨勭敓鍛藉懆鏈熷唴鍙敤锛屼綔涓哄搷搴斺€︹€?|
| TASK_DELAY_ACCT | bool | 鏀堕泦浠诲姟绛夊緟绯荤粺璧勬簮锛堝 cpu銆佸悓姝ュ潡 I/O 瀹屾垚涓庨〉鎹㈠叆锛夋墍鑺辨椂闂寸殑淇℃伅銆傛绫荤粺璁℃湁鍔╀簬璁剧疆浠诲姟鐨勪紭鍏堢骇鈥︹€?|
| TASK_IO_ACCOUNTING | bool | 鏀堕泦姝や换鍔″紩璧风殑瀛樺偍 I/O 瀛楄妭鏁颁俊鎭€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TASK_XACCT | bool | 鏀堕泦鎵╁睍鐨勪换鍔¤璐︽暟鎹紝骞堕€氳繃 taskstats 鎺ュ彛灏嗘暟鎹彂閫佸埌鐢ㄦ埛绌洪棿澶勭悊銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_BITOPS | tristate | 鏋勫缓鈥渢est_bitops鈥濇ā鍧楋紝瀹冧笌 TEST_LKM 妯″潡闈炲父鐩镐技锛屽彧鏄畠瀵?set/clear_bit 瀹忎笌 get_count_order/long 鍋氬熀鏈紨缁冿紝浠ョ‘淇濇病鏈夌紪璇戔€︹€?|
| TEST_BPF | tristate | 鏋勫缓鈥渢est_bpf鈥濇ā鍧楋紝鏍规嵁褰撳墠璁剧疆瀵?BPF 瑙ｉ噴鍣ㄦ垨 BPF JIT 缂栬瘧鍣ㄨ繍琛屽悇绉嶆祴璇曞悜閲忋€傝繖瀵?BPF JIT 缂栬瘧鍣ㄢ€︹€︾壒鍒湁鐢?|
| TEST_CLOCKSOURCE_WATCHDOG | tristate | 鍚敤姝ら€夐」浠ュ垱寤轰竴涓唴鏍告ā鍧楋紝瑙﹀彂 clocksource 鐪嬮棬鐙楃殑娴嬭瘯銆傝妯″潡鍙€氳繃 modprobe 鎴?insmod 鍔犺浇锛屽姞杞芥椂鍗宠繍琛岋紝鎴栤€︹€?|
| TEST_DEBUG_VIRTUAL | tristate | 娴嬭瘯鍐呮牳妫€娴嬪鍐呮牳铏氭嫙鍦板潃鏄犲皠闈炵嚎鎬ч儴鍒嗛敊璇皟鐢?virt_to_phys() 鐨勮兘鍔涖€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_DIV64 | tristate | 鍚敤姝ら」浠ュ紑鍚?'do_div()' 鍑芥暟娴嬭瘯銆傝娴嬭瘯浠呭湪绯荤粺寮曞鏈熼棿鎵ц涓€娆★紙鍥犳鍙奖鍝嶅紩瀵兼椂闂达級锛屾垨鍦ㄦā鍧楀姞杞芥椂鎵ц銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_DYNAMIC_DEBUG | tristate | 璇ユā鍧楁敞鍐屼竴涓窡韪洖璋冿紝缁熻 'do_debugging' 鍑芥暟涓凡鍚敤鐨?pr_debug锛岀劧鍚庢敼鍙樺叾鍚敤鐘舵€併€佽皟鐢ㄨ鍑芥暟骞舵瘮杈冭鏁般€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_FIRMWARE | tristate | 鏋勫缓鈥渢est_firmware鈥濇ā鍧楋紝瀹冨垱寤轰竴涓敤浜庢祴璇曞浐浠跺姞杞界殑鐢ㄦ埛绌洪棿鎺ュ彛銆傝繖鍙敤浜庡湪涓嶉渶鐪熷疄鍥轰欢鈥︹€︾殑鎯呭喌涓嬫帶鍒跺浐浠跺姞杞界殑瑙﹀彂 |
| TEST_FPU | tristate | 鍚敤姝ら€夐」浠ユ坊鍔?/sys/kernel/debug/selftest_helpers/test_fpu锛屽畠灏嗚Е鍙戜竴绯诲垪娴偣杩愮畻銆傝繖鐢ㄤ簬娴偣鎺у埗瀵勫瓨鍣ㄨ缃殑鑷垜娴嬭瘯鈥︹€?|
| TEST_FREE_PAGES | tristate | 娴嬭瘯鏄惁鍥犻噴鏀句竴鍧楅〉涓庢帹娴嬫€ч〉寮曠敤涔嬮棿鐨勭珵浜夎€屼笉鍙戠敓鍐呭瓨娉勬紡銆傝嫢浣犵殑鍐呮牳宸蹭慨澶嶈缂洪櫡锛屽姞杞芥妯″潡鏄畨鍏ㄧ殑銆傝嫢缂洪櫡鈥︹€?|
| TEST_HEXDUMP | tristate | 鍚敤姝ら€夐」浠ュ湪杩愯鏃舵祴璇?printf 鍑芥暟銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_HMM | tristate | 杩欐槸涓€涓粎鐢ㄤ簬娴嬭瘯 HMM 鐨勪吉璁惧椹卞姩銆傝嫢浣犳兂鏋勫缓 HMM 娴嬭瘯妯″潡锛屽湪姝ら€?M銆傝繖鏍峰仛灏嗗厑璁镐綘杩愯 tools/testing/selftest/vm/hmm-tests銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_IDA | tristate | 閽堝 miscdevice API 鐨?Kunit 娴嬭瘯锛岀壒鍒槸鍏跺叧浜庨潤鎬佷笌鍔ㄦ€佹璁惧鍙风殑琛屼负銆侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞朵互 TAP 鏍煎紡锛坔ttps://test鈥︹€︼級灏嗙粨鏋滆緭鍑哄埌璋冭瘯鏃ュ織 |
| TEST_IOV_ITER | tristate | 鍚敤姝ら」浠ュ紑鍚 I/O 杩唬鍣紙iov_iter锛夋搷浣滅殑娴嬭瘯銆傝娴嬭瘯浠呭湪绯荤粺寮曞鏈熼棿鎵ц涓€娆★紙鍥犳鍙奖鍝嶅紩瀵兼椂闂达級锛屾垨鍦ㄦā鍧楀姞杞芥椂鎵ц銆傝嫢涓嶇‘瀹氾紝閫夆€︹€?|
| TEST_KALLSYMS_A | tristate | 閫夋嫨鈥淔ast鈥濅互澶栫殑鍐呭灏嗗惎鐢ㄤ細鎷栨參鏋勫缓骞跺彲鑳戒娇鏋勫缓宕╂簝鐨勬祴璇曘€?|
| TEST_KALLSYMS_FAST | bool | 浣犲疄闄呬笂涓嶄細娴嬭瘯 kallsysms锛屽洜姝よ繖鍙槸鍦ㄤ娇鐢?allmodconfig 鏃跺府鍔╁揩閫熸瀯寤恒€?|
| TEST_KALLSYMS_LARGE | bool | 杩欏皢鍚敤鏇村ぇ鏁伴噺鐨勭鍙枫€傝繖浼氭樉钁楁嫋鎱綘鐨勬瀯寤恒€?|
| TEST_KALLSYMS_MAX | bool | 杩欏皢鍚敤瀵煎嚭锛岀洿鍒版垜浠煡閬撳皢寮€濮嬩娇鏋勫缓宕╂簝鐨勭▼搴︺€?|
| TEST_KALLSYMS_NUMSYMS | int | 鍦?TEST_KALLSYMS_A 涓婂垱寤虹殑绗﹀彿鏁伴噺锛屽叾涓彧鏈?TEST_KALLSYMS_B 妯″潡浼氫娇鐢ㄥ叾涓€銆傝繖涔熺敤浜庣‘瀹?TEST_KALLSYMS_C 灏嗘嫢鏈夌殑绗﹀彿鏁伴噺锛岀敱 TEST_KALLS鈥︹€︽斁澶?|
| TEST_KALLSYMS_SCALE_FACTOR | int | TEST_KALLSYSMS_C 姣?TEST_KALLSYMS_A 澶氬嚭鐨勬湭浣跨敤绗﹀彿鏁伴噺銆傝嫢涓?8锛屽垯妯″潡 C 鎷ユ湁鐨勭鍙锋槸妯″潡 A 鐨?8 鍊嶃€傜劧鍚?TEST_KALLSYMS_D 鎷ユ湁鐨勭鍙锋暟閲忔槸鈥︹€︾殑涓ゅ€?|
| TEST_KEXEC_HANDOVER | bool | 姝ら€夐」鍚敤 Kexec HandOver锛圞HO锛夋祴璇曘€傛祴璇曠敱涓ら儴鍒嗙粍鎴愶細鍦?kexec 涔嬪墠淇濆瓨鍐呮牳鏁版嵁锛屽苟鍦?kexec 涔嬪悗鎭㈠鏁版嵁骞堕獙璇佸叾琚纭Щ浜も€︹€?|
| TEST_KMOD | tristate | 娴嬭瘯鍐呮牳鐨勬ā鍧楀姞杞芥満鍒?kmod銆俴mod 瀹炵幇浣跨敤 Linux 鍐呮牳 usermode helper 鍔犺浇妯″潡鐨勬敮鎸併€傝娴嬭瘯鎻愪緵涓€绯诲垪閽堝 kmod 鐨勬祴璇曘€傚敖绠℃妧鏈€︹€?|
| TEST_KSTRTOX | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃舵祴璇?bitmap 鍑芥暟銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_LIST_SORT | tristate | 鍚敤姝ら」浠ュ紑鍚?'list_sort()' 鍑芥暟娴嬭瘯銆傝娴嬭瘯浠呭湪绯荤粺寮曞鏈熼棿鎵ц涓€娆★紙鍥犳鍙奖鍝嶅紩瀵兼椂闂达級锛屾垨鍦ㄦā鍧楀姞杞芥椂鎵ц銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_LKM | tristate | 鏋勫缓鈥渢est_module鈥濇ā鍧楋紝鍔犺浇鏃堕€氳繃 printk 杈撳嚭鈥淗ello, world鈥濄€傚畠璁捐鐢ㄤ簬妯″潡鍔犺浇瀛愮郴缁熺殑鍩烘湰璇勪及锛堜緥濡傞獙璇佹ā鍧椻€︹€︼級 |
| TEST_LOCKUP | tristate | 鏋勫缓鈥渢est_lockup鈥濇ā鍧楋紝甯姪纭繚鐪嬮棬鐙椾笌閿佹妫€娴嬪櫒姝ｅ父宸ヤ綔銆傛牴鎹ā鍧楀弬鏁帮紝瀹冨彲浠ユā鎷熻蒋閿佹鎴栫‖閿佹銆佲€渉ung tas鈥︹€︹€?|
| TEST_MEMCAT_P | tristate | 娴嬭瘯 memcat_p() 杈呭姪鍑芥暟鏄惁姝ｇ‘鍚堝苟涓や釜鎸囬拡鏁扮粍銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_MEMINIT | tristate | 娴嬭瘯鍐呮牳鏄惁瀵瑰爢涓庨〉鍒嗛厤杩涜闆跺垵濮嬪寲銆傝繖鏈夊姪浜庢祴璇?init_on_alloc 涓?init_on_free 鐗规€с€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_MULDIV64 | tristate | 鍚敤姝ら」浠ュ紑鍚?'mul_u64_u64_div_u64()' 鍑芥暟娴嬭瘯銆傝娴嬭瘯浠呭湪绯荤粺寮曞鏈熼棿鎵ц涓€娆★紙鍥犳鍙奖鍝嶅紩瀵兼椂闂达級锛屾垨鍦ㄦā鍧楀姞杞芥椂鎵ц銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_OBJAGG | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃讹紙鎴栨ā鍧楀姞杞芥椂锛夋祴璇曞璞¤仛鍚堢鐞嗗櫒銆?|
| TEST_OBJPOOL | tristate | 鏋勫缓鈥渢est_objpool鈥濇ā鍧楋紝鐢ㄤ簬瀵硅薄鍒嗛厤涓庡洖鏀剁殑姝ｇ‘鎬т笌骞跺彂娴嬭瘯銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_PARMAN | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃讹紙鎴栨ā鍧楀姞杞芥椂锛夋祴璇曚紭鍏堢骇鏁扮粍绠＄悊鍣ㄣ€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_REF_TRACKER | tristate | 姝ら€夐」鎻愪緵涓€涓娇鐢ㄥ紩鐢ㄨ窡韪櫒鍩虹璁炬柦鎵ц娴嬭瘯鐨勫唴鏍告ā鍧椼€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_RHASHTABLE | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃舵祴璇?rhashtable 鍑芥暟銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_RUNTIME | bool | 杩欏厑璁告垜浠€氳繃鐢ㄤ簬鍦?kallsyms 涓婃斁缃鍙凤紙濡傚鍑虹鍙凤級鐨?kallsyms 鏉ュ find_symbol() 杩涜鍘嬪姏娴嬭瘯銆傛垜浠€︹€?|
| TEST_SORT | tristate | 姝ら€夐」鍦ㄥ紩瀵兼椂鎴栨ā鍧楀姞杞芥椂鍚敤 'sort()' 鐨勮嚜妫€娴嬭瘯鍑芥暟銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_STATIC_KEYS | tristate | 娴嬭瘯闈欐€侀敭鎺ュ彛銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_SYSCTL | tristate | 鏋勫缓鈥渢est_sysctl鈥濇ā鍧椼€傝椹卞姩鍙畨鍏ㄦ祴璇曢┍鍔ㄥ彲鐢ㄧ殑 proc sysctl 鎺ュ彛锛岃€屼笉褰卞搷鍙兘鏀瑰彉绯荤粺鍔熻兘鐨勭敓鎴愭棆閽€傝嫢鈥︹€?|
| TEST_UDELAY | tristate | 鏋勫缓鈥渦delay_test鈥濇ā鍧楋紝甯姪纭繚 udelay() 宸ヤ綔姝ｅ父銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_VMALLOC | tristate | 鏋勫缓鈥渢est_vmalloc鈥濇ā鍧楋紝鐢ㄤ簬鍘嬪姏涓庢€ц兘鍒嗘瀽銆傚洜姝わ紝瀵?vmalloc 瀛愮郴缁熺殑浠讳綍鏂版敼鍔ㄩ兘鍙粠鎬ц兘涓庣ǔ瀹氭€ц搴﹁瘎浼扳€︹€?|
| TEST_WORKQUEUE | tristate | 鏋勫缓鈥渢est_workqueue鈥濇ā鍧楋紝鐢ㄤ簬鍦ㄤ簤鐢ㄤ笅瀵瑰伐浣滈槦鍒楀悶鍚愰噺杩涜鍩哄噯娴嬭瘯銆傛湁鍔╀簬璇勪及浜插拰鎬ц寖鍥村彉鍖栵紙濡?cache_shard 涓?cache锛夈€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEST_XARRAY | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃舵垨妯″潡鍔犺浇鏃舵祴璇?maple tree 浠ｇ爜鍑芥暟銆傚惎鐢ㄢ€淒ebug Maple Trees鈥濆皢鍦ㄥけ璐ユ椂杈撳嚭鏇磋缁嗙殑鏃ュ織銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| TEXTSEARCH | bool | 绠€鍗曘€佸彲宓屽叆鐨勫尯闂存爲銆傚彲鍦?log(n) 鏃堕棿鍐呮壘鍒伴噸鍙犺寖鍥寸殑璧风偣锛岀劧鍚庨亶鍘嗘墍鏈夐噸鍙犺妭鐐广€傝绠楁硶瀹炵幇涓哄寮?rbtree銆傚弬瑙侊細D鈥︹€?|
| THP_SWAP | def_bool | 浠ユ暣浣撴柟寮忎氦鎹㈤€忔槑澶ч〉锛屾棤闇€鎷嗗垎銆俋XX锛氱洰鍓嶏紝鏀拺閫忔槑澶ч〉鐨勪氦鎹㈢皣灏嗗湪鎹㈠嚭鍚庤鎷嗗垎銆備緵鍏锋湁鍚堢悊 THP鈥︹€︾殑浣撶郴缁撴瀯閫夋嫨 |
| TIMERFD | bool | 鍚敤 timerfd() 绯荤粺璋冪敤锛屽畠鍏佽鍦ㄦ枃浠舵弿杩扮涓婃帴鏀跺畾鏃朵簨浠躲€傝嫢涓嶇‘瀹氾紝閫?Y銆?|
| TIME_NS | bool | 鍦ㄨ鍛藉悕绌洪棿涓紝boottime 涓庡崟璋冩椂閽熷彲琚缃€傛椂闂村皢浠ョ浉鍚岃妭濂忕户缁蛋銆?|
| TIME_NS_VDSO | def_bool | 鍦ㄨ鍛藉悕绌洪棿涓紝浠诲姟浣跨敤瀵瑰簲浜庝笉鍚屽懡鍚嶇┖闂翠腑涓嶅悓 IPC 瀵硅薄鐨?IPC id銆?|
| TMPFS | bool | Tmpfs 鏄竴绉嶅皢鎵€鏈夋枃浠朵繚瀛樺湪铏氭嫙鍐呭瓨涓殑鏂囦欢绯荤粺銆俆mpfs 涓殑涓€鍒囬兘鏄复鏃剁殑锛屾剰鍛崇潃涓嶄細鍦ㄤ綘鐨勭‖鐩樹笂鍒涘缓鏂囦欢銆傛枃浠跺瓨鍦ㄤ簬鍐呭瓨涓庝氦鎹㈢┖闂粹€︹€?|
| TMPFS_INODE64 | bool | tmpfs 鍘嗗彶涓婂彧浣跨敤涓?unsigned int 绛夊鐨勬枃浠跺彿銆傚湪鏌愪簺鎯呭喌涓嬭繖浼氬鑷村洖缁曪紝鍙兘浣垮崟涓澶団€︹€︿笂鍑虹幇澶氫釜鍏锋湁鐩稿悓鏂囦欢鍙风殑鏂囦欢 |
| TMPFS_POSIX_ACL | bool | POSIX 璁块棶鎺у埗鍒楄〃锛圓CL锛変负鏍囧噯鐨勬墍鏈夎€?缁?鍏朵粬鏂规涔嬪鐨勭敤鎴蜂笌缁勬彁渚涢澶栫殑璁块棶鏉冮檺锛屾閫夐」閫夋嫨瀵?tmpfs 鐨?ACL 鏀寔鈥︹€?|
| TMPFS_QUOTA | bool | 閰嶉鏀寔鍏佽涓?tmpfs 浣跨敤璁剧疆姣忕敤鎴蜂笌姣忕粍鐨勯檺鍒躲€傞€?Y 浠ュ惎鐢ㄩ厤棰濇敮鎸併€傚惎鐢ㄥ悗锛屼綘鍙€氳繃 quota銆乽srquota 涓?grpquot鈥︹€︽帶鍒剁敤鎴蜂笌缁勯厤棰濈殑寮哄埗 |
| TMPFS_XATTR | bool | 鎵╁睍灞炴€ф槸鐢卞唴鏍告垨鐢ㄦ埛涓?inode 鍏宠仈鐨?鍚嶇О:鍊?瀵癸紙璇﹁ attr(5) 鎵嬪唽椤碉級銆傝繖鍚敤瀵?trusted.*銆乻ecurity.* 涓?user.* 鍚嶇О鈥︹€︾殑鏀寔 |
| TRACE_IRQFLAGS | bool | 鍚敤鐢ㄤ簬璺熻釜鎴栭攣璋冭瘯鐨勪腑鏂惎鐢?绂佺敤閽╁瓙銆?|
| TRACE_IRQFLAGS_NMI | def_bool | 褰?CPU 鏈兘鍝嶅簲缁欏畾鐨?backtrace NMI 鏃跺惎鐢ㄨ皟璇曟墦鍗般€傝繖浜涙墦鍗版彁渚涗竴浜?CPU 鍙兘鍚堢悊鍦版湭鑳藉搷搴旂殑鍘熷洜锛屼緥濡傚畠澶勪簬绂荤嚎鐘舵€佹垨鈥︹€?|
| TRACE_MMIO_ACCESS | bool | 涓?MMIO 璇?鍐欐搷浣滃垱寤鸿窡韪偣銆傝繖浜涜窡韪簨浠跺彲鐢ㄤ簬璁板綍鎵€鏈?MMIO 璇?鍐欐搷浣溿€?|
| TRANSPARENT_HUGEPAGE_ALWAYS | bool | 鎬绘槸鍚敤閫忔槑澶ч〉锛屽彲鑳戒細澧炲姞搴旂敤鐨勫唴瀛樺崰鐢ㄨ€屾病鏈変繚璇佺殑鏀剁泭锛屼絾瀹冧細瀵规墍鏈夊簲鐢ㄨ嚜鍔ㄧ敓鏁堛€?|
| TRANSPARENT_HUGEPAGE_MADVISE | bool | 鍚敤閫忔槑澶ч〉 madvise锛屽彧浼氫负浣跨敤 madvise(MADV_HUGEPAGE) 鐨勫簲鐢ㄥ甫鏉ユ€ц兘鎻愬崌锛屼絾涓嶄細鍐掑鍔犲簲鐢ㄥ唴瀛樺崰鐢ㄧ殑椋庨櫓鈥︹€?|
| TRANSPARENT_HUGEPAGE_NEVER | bool | 榛樿绂佺敤閫忔槑澶ч〉銆備粛鍙湪杩愯鏃堕€氳繃 sysfs 鍚敤銆?|
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ADVISE | bool | 浠呭綋搴旂敤鎻愪緵 madvise(MADV_HUGEPAGE) 鎻愮ず鏃讹紝鎵嶄负 shmem 鎸傝浇鍚敤澶ч〉鍒嗛厤銆傝繖纭繚澶ч〉浠呭湪鍝嶅簲鏉ヨ嚜鈥︹€︾殑鏄惧紡璇锋眰鏃朵娇鐢?|
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ALWAYS | bool | 鎬绘槸灏濊瘯涓?shmem 鎸傝浇鍒嗛厤澶ч〉锛屽彲鑳戒細澧炲姞搴旂敤鍐呭瓨鍗犵敤鑰屾病鏈変繚璇佺殑鏀剁泭锛屼絾瀹冧細瀵规墍鏈夊簲鐢ㄨ嚜鍔ㄧ敓鏁堛€?|
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_NEVER | bool | 榛樿绂佺敤 shmem 鎸傝浇鐨勫ぇ椤靛垎閰嶃€備粛鍙€氳繃鍐呮牳鍛戒护琛?'transparent_hugepage_shmem=' 閫夐」鎴栬繍琛屾椂 sysfs 鏃嬮挳鍚敤銆傛敞鎰?madvise(MAD鈥︹€?|
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_WITHIN_SIZE | bool | 鑻ュ垎閰嶅皢瀹屽叏浣嶄簬 i_size 涔嬪唴锛屽垯涓?shmem 鎸傝浇鍚敤澶ч〉鍒嗛厤銆傛閰嶇疆杩樿€冭檻鈥︹€﹀彲鑳芥彁渚涚殑浠讳綍 madvise(MADV_HUGEPAGE) 鎻愮ず |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ADVISE | bool | 浠呭綋搴旂敤鎻愪緵 madvise(MADV_HUGEPAGE) 鎻愮ず鏃讹紝鎵嶄负 tmpfs 鎸傝浇鍚敤澶ч〉鍒嗛厤銆傝繖纭繚澶ч〉浠呭湪鍝嶅簲鏉ヨ嚜鈥︹€︾殑鏄惧紡璇锋眰鏃朵娇鐢?|
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ALWAYS | bool | 鎬绘槸灏濊瘯涓?tmpfs 鎸傝浇鍒嗛厤澶ч〉锛屽彲鑳戒細澧炲姞搴旂敤鍐呭瓨鍗犵敤鑰屾病鏈変繚璇佺殑鏀剁泭锛屼絾瀹冧細瀵规墍鏈夊簲鐢ㄨ嚜鍔ㄧ敓鏁堛€?|
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_NEVER | bool | 榛樿绂佺敤 tmpfs 鎸傝浇鐨勫ぇ椤靛垎閰嶃€備粛鍙€氳繃鍐呮牳鍛戒护琛?'transparent_hugepage_tmpfs=' 閫夐」鍚敤銆傛敞鎰?madvise(MADV_COLLAPSE) 浠嶅彲鑳藉鑷粹€︹€?|
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_WITHIN_SIZE | bool | 鑻ュ垎閰嶅皢瀹屽叏浣嶄簬 i_size 涔嬪唴锛屽垯涓?tmpfs 鎸傝浇鍚敤澶ч〉鍒嗛厤銆傛閰嶇疆杩樿€冭檻鈥︹€﹀彲鑳芥彁渚涚殑浠讳綍 madvise(MADV_HUGEPAGE) 鎻愮ず |
| UAPI_HEADER_TEST | bool | 缂栬瘧娴嬭瘯瀵煎嚭鍒扮敤鎴风┖闂寸殑澶存枃浠讹紝浠ョ‘淇濆畠浠槸鑷寘鍚殑锛堝嵆鍙綔涓虹嫭绔嬪崟鍏冪紪璇戯級銆傝嫢浣犳槸寮€鍙戣€呮垨娴嬭瘯鑰呭苟甯屾湜纭繚瀵煎嚭鐨勫ご鏂囦欢鏄嚜鍖呭惈鈥︹€?|
| UCLAMP_BUCKETS_COUNT | int | 瀹氫箟瑕佷娇鐢ㄧ殑閽充綅妗舵暟閲忋€傛瘡涓《鐨勮寖鍥村皢鏄?SCHED_CAPACITY_SCALE/UCLAMP_BUCKETS_COUNT銆傞挸浣嶆《鏁伴噺瓒婂锛岀矑搴﹁秺缁嗭紝涓斺€︹€?|
| UCS2_STRING | tristate | 鎻愪緵涓€涓皢鏁ｅ垪琛ㄦ媶鍒嗕负澶氫釜鍧楋紙姣忓潡鏄竴涓暎鍒楄〃锛夌殑杈呭姪鍑芥暟銆傚簲鐢卞笇鏈涘湪澶氫釜 DMA 閫氶亾闂存媶鍒嗘暎鍒楄〃鐨勯┍鍔ㄦ垨 API 閫変腑銆?|
| UID16 | bool | 杩欏惎鐢ㄤ紶缁熺殑 16 浣?UID 绯荤粺璋冪敤灏佽銆?|
| USERCOPY_KUNIT_TEST | tristate | 鏋勫缓鈥渦sercopy_kunit鈥濇ā鍧楋紝瀵?copy_to/from_user 鍩虹璁炬柦杩愯鍋ュ叏鎬ф鏌ワ紝纭繚鍩烘湰鐨勭敤鎴?鍐呮牳杈圭晫娴嬭瘯姝ｅ父宸ヤ綔銆?|
| USERFAULTFD | bool | 鍚敤 userfaultfd() 绯荤粺璋冪敤锛屽畠鍏佽鍦ㄧ敤鎴风┖闂存嫤鎴苟澶勭悊缂洪〉銆傝嫢 USERFAULTFD |
| USER_NS | bool | 杩欏厑璁稿鍣紙鍗?vservers锛変娇鐢ㄧ敤鎴峰懡鍚嶇┖闂翠负涓嶅悓鏈嶅姟鍣ㄦ彁渚涗笉鍚岀殑鐢ㄦ埛淇℃伅銆傚綋鍐呮牳涓惎鐢ㄤ簡鐢ㄦ埛鍛藉悕绌洪棿鏃讹紝寤鸿灏?MEMCG 鎴栤€︹€?|
| UTIL_MACROS_KUNIT | tristate | 鍚敤姝ら€夐」浠ュ湪寮曞鏃舵祴璇?util_macros.h 鍑芥暟銆侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞朵互 TAP 鏍煎紡锛坔ttp://testanything.org/锛夊皢缁撴灉杈撳嚭鍒拌皟璇曟棩蹇椼€備粎瀵瑰唴鏍糕€︹€︽湁鐢?|
| UTS_NS | bool | 鍦ㄨ鍛藉悕绌洪棿涓紝浠诲姟鐪嬪埌 uname() 绯荤粺璋冪敤鎻愪緵鐨勪笉鍚屼俊鎭?|
| UUID_KUNIT_TEST | tristate | 姝ら€夐」鍚敤閽堝 uuid 搴擄紙鎻愪緵鐢熸垚涓庤В鏋?UUID 鍜?GUID 鐨勫嚱鏁帮級鐨?KUnit 娴嬭瘯濂椾欢銆傝娴嬭瘯濂椾欢妫€鏌?UUID 涓?GUID 瀛楃涓茬殑瑙ｆ瀽銆傝嫢涓嶇‘瀹氣€︹€?|
| VIRT_CPU_ACCOUNTING_GEN | bool | 閫夋嫨姝ら€夐」浠ュ湪瀹屽叏 dynticks 绯荤粺涓婂惎鐢ㄤ换鍔′笌 CPU 鏃堕棿璁拌处銆傝璁拌处閫氳繃 context tracking 瀛愮郴缁熺洃瑙嗘瘡涓唴鏍?鐢ㄦ埛杈圭晫鏉ュ疄鐜般€傝鈥︹€?|
| VIRT_CPU_ACCOUNTING_NATIVE | bool | 閫夋嫨姝ら€夐」浠ュ惎鐢ㄦ洿绮剧‘鐨勪换鍔′笌 CPU 鏃堕棿璁拌处銆傝繖閫氳繃鍦ㄦ瘡娆″唴鏍歌繘鍏ヤ笌閫€鍑轰互鍙婂唴鏍稿唴绯荤粺鎬佲€︹€︿箣闂磋浆鎹㈡椂璇诲彇 CPU 璁℃暟鍣ㄦ潵瀹炵幇 |
| VMAP_PFN | bool | 鏄剧ず浜嬩欢璁℃暟闇€瑕?VM 浜嬩欢璁℃暟鍣ㄣ€傛閫夐」鍏佽鍦?EXPERT 绯荤粺涓婄鐢?VM 浜嬩欢璁℃暟鍣ㄣ€?proc/vmstat 浠呭湪 VM 浜嬩欢璁℃暟鍣ㄢ€︹€﹀瓨鍦ㄦ椂鎵嶆樉绀洪〉璁℃暟 |
| WANT_COMPAT_NETLINK_MESSAGES | bool | 姝ら€夐」鍙敱闇€瑕?compat netlink 娑堟伅鐨勫叾浠栭€夐」閫変腑銆?|
| WARN_ABI_ERRORS | bool | Documentation/ABI 涓嬬殑鏂囦欢搴旈伒寰?Documentation/ABI/README 涓殑鎻忚堪銆傜劧鑰岋紝鐢变簬瀹冧滑鏄墜鍔ㄧ紪鍐欑殑锛屾煇浜涙枃浠跺彲鑳藉瓨鍦ㄩ敊璇€︹€?|
| WARN_CONTEXT_ANALYSIS | bool | 涓婁笅鏂囧垎鏋愭槸涓€绉嶈瑷€鎵╁睍锛屽畠閫氳繃鍦ㄨ幏鍙栦笌閲婃斁鐢ㄦ埛鍙畾涔夌殑鈥渃ontext locks鈥濇椂闈欐€佹鏌ユ墍闇€涓婁笅鏂囨槸鍚﹀浜庢椿鍔紙鎴栭潪娲诲姩锛夌姸鎬併€侰lang 鐨勫悕绉扳€︹€?|
| WARN_CONTEXT_ANALYSIS_ALL | bool | 鍚敤鍏ㄦ爲鑼冨洿鐨勪笂涓嬫枃鍒嗘瀽銆傝繖寰堝彲鑳戒骇鐢熷ぇ閲忚鎶モ€斺€旈闄╄嚜璐熴€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| WARN_MISSING_DOCUMENTS | bool | 鏂囨。琚噸鍛藉悕骞朵笉缃曡銆傛閫夐」璁╁唴鏍告鏌ョ己澶辩殑渚濊禆椤癸紝骞跺湪缂哄け鏃跺彂鍑鸿鍛娿€備粎褰撳唴鏍镐粠 git 鏍戔€︹€︽瀯寤烘椂鎵嶆湁鏁?|
| WERROR | bool | 鍐呮牳鏋勫缓涓嶅簲浜х敓浠讳綍缂栬瘧鍣ㄨ鍛婏紝姝ら€夐」榛樿鍚敤 '-Werror'锛堢敤浜?C锛変笌 '-Dwarnings'锛堢敤浜?Rust锛夋爣蹇椾互寮哄埗璇ヨ鍒欍€傛潵鑷叾浠栧伐鍏风殑鏌愪簺璀﹀憡鈥︹€?|
| WQ_CPU_INTENSIVE_REPORT | bool | 鍦ㄦ閫?Y 浠ュ惎鐢ㄥ鍗犵敤 CPU 瓒呰繃 workqueue.cpu_intensive_thresh_us 鐨勫苟鍙戠鐞?per-cpu 宸ヤ綔椤圭殑鎶ュ憡銆傚伐浣滈槦鍒椾細鑷姩妫€娴嬪苟灏嗗畠浠帓闄ゅ嚭骞跺彂鈥︹€?|
| WQ_WATCHDOG | bool | 鍦ㄦ閫?Y 浠ュ惎鐢ㄥ宸ヤ綔闃熷垪鐨勫仠椤挎娴嬨€傝嫢涓€涓伐浣滄睜鍦ㄨ秴杩囩粰瀹氭椂闂达紙榛樿 30 绉掞級鍐呮湭鍦ㄥ緟澶勭悊宸ヤ綔椤逛笂鍙栧緱杩涘睍锛屼細鎵撳嵃涓€鏉¤鍛婃秷鎭€︹€?|
| WW_MUTEX_SELFTEST | tristate | 姝ら€夐」鎻愪緵涓€涓唴鏍告ā鍧楋紝瀵?struct ww_mutex 閿?API 杩愯娴嬭瘯銆傚缓璁厤鍚堟娴嬭瘯宸ュ叿鍚敤 DEBUG_WW_MUTEX_SLOWPATH銆傝嫢鈥︹€﹂€?M |
| XXHASH | tristate | 姝ら€夐」鍚敤 32 浣?PRNG 搴撳嚱鏁帮紝鍦ㄥ垵濮嬪寲鏃舵墽琛岃嚜妫€娴嬭瘯銆? # 鍘嬬缉鏀寔鍦ㄩ渶瑕佹椂琚€変腑 # |
| ZSMALLOC_CHAIN_SIZE | int | 姝ら€夐」璁剧疆 zmalloc 椤碉紙zspage锛夊彲鍖呭惈鐨勭墿鐞嗛〉鏁伴噺涓婇檺銆傛渶浼樼殑 zspage 閾惧ぇ灏忓湪鍒濆鍖栨湡闂翠负姣忎釜澶у皬绫昏绠椻€︹€?|
| ZSWAP | bool | 涓€涓敤浜庝氦鎹㈤〉鐨勮交閲忕骇鍘嬬缉缂撳瓨銆傚畠鎺ユ敹姝ｅ湪琚崲鍑虹殑椤碉紝骞跺皾璇曞皢鍏跺帇缂╁埌鍔ㄦ€佸垎閰嶇殑鍩轰簬 RAM 鐨勫唴瀛樻睜涓€傝繖鍙€︹€?|
| ZSWAP_COMPRESSOR_DEFAULT | string | 姝ら€夐」鍚敤 zsmalloc 涓殑浠ｇ爜锛屼互鏀堕泦 zsmalloc 鍐呴儴鍙戠敓鎯呭喌鐨勫悇绫荤粺璁★紝骞堕€氳繃 debugfs 灏嗗叾瀵煎嚭鍒扮敤鎴风┖闂淬€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| ZSWAP_COMPRESSOR_DEFAULT_842 | bool | 浣跨敤 842 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_DEFLATE | bool | 浣跨敤 Deflate 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_LZ4 | bool | 浣跨敤 LZ4 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_LZ4HC | bool | 浣跨敤 LZ4HC 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_LZO | bool | 浣跨敤 LZO 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_ZSTD | bool | 浣跨敤 zstd 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_DEFAULT_ON | bool | 鑻ラ€変腑锛屼氦鎹㈤〉鐨勫帇缂╃紦瀛樺皢鍦ㄥ紩瀵兼椂鍚敤锛屽惁鍒欑鐢ㄣ€傛澶勬墍鍋氱殑閫夋嫨鍙€氳繃鍐呮牳鍛戒护琛?'zswap.enabled='鈥︹€﹁鐩?|
| ZSWAP_SHRINKER_DEFAULT_ON | bool | 鑻ラ€変腑锛寊swap shrinker 灏嗚鍚敤锛屽瓨鍌ㄥ湪 zswap 姹犱腑鐨勯〉灏嗗湪鍐呭瓨鍘嬪姏涓嬪彲鐢ㄤ簬鍥炴敹锛堝嵆鍐欏洖鍚庡浜ゆ崲璁惧锛夈€傝繖鎰忓懗鈥︹€?|
| if | bool | 鑻ュ湪姝ら€?Y锛実cc 灏嗚鎸囩ず涓虹粨鏋勪綋绫诲瀷鐢熸垚鏇村皯鐨勮皟璇曚俊鎭€傝繖鎰忓懗鐫€闇€瑕佸畬鏁磋皟璇曚俊鎭紙濡?kgdb 鎴?systemtap锛夌殑宸ュ叿灏嗕笉婊℃剰銆備絾鈥︹€?|
| select | bool | 鐢熸垚 DWARF v5 璋冭瘯淇℃伅銆傞渶瑕?binutils 2.35.2銆乬cc 5.0+锛坓cc 5.0+ 鎺ュ彈 -gdwarf-5 鏍囧織锛屼絾鍦?7.0 涔嬪墠瀵规煇浜涜崏妗堢壒鎬т粎閮ㄥ垎鏀寔锛変互鍙?gdb 8.0+銆傚 s鈥︹€︾殑鏇存敼 |

---

# Makefile 鐩爣

## 鏋勫缓鐩爣

| 鐩爣 | 鎻忚堪 | 鏉ユ簮 |
|--------|-------------|--------|
| all | 濡傛灉鏋勫缓澶栭儴妯″潡锛屾垜浠苟涓嶅叧蹇?all: 瑙勫垯锛岃€屾槸璁?__all 渚濊禆浜?modules | Makefile |
| dtbs_install |  | Makefile |
| headers |  | Makefile |
| headers_install |  | Makefile |
| modules | 鏋勫缓鎵€鏈夊彲鍔犺浇鐨勫唴鏍告ā鍧?| Makefile |
| modules_install |  | Makefile |
| vmlinux |  | Makefile |

## 閰嶇疆鐩爣

| 鐩爣 | 鎻忚堪 | 鏉ユ簮 |
|--------|-------------|--------|
| config |  | Makefile |

## 娓呯悊鐩爣

| 鐩爣 | 鎻忚堪 | 鏉ユ簮 |
|--------|-------------|--------|
| clean | clean - 鍒犻櫎澶ч儴鍒嗘枃浠讹紝浣嗕繚鐣欒冻澶熷唴瀹逛互鏋勫缓澶栭儴妯″潡 | Makefile |
| distclean | distclean  | Makefile |
| mrproper | mrproper - 鍒犻櫎鎵€鏈夌敓鎴愮殑鏂囦欢锛屽寘鎷?.config | Makefile |

## 鏂囨。鐩爣

| 鐩爣 | 鎻忚堪 | 鏉ユ簮 |
|--------|-------------|--------|
| cleandocs | 鍒犻櫎鎵€鏈夌敓鎴愮殑鏂囨。鏂囦欢 | Makefile |
| htmldocs-redirects |  | Makefile |
| markdowndocs | 閫氳繃 Pandoc 鍚庡鐞嗘瀯寤?Markdown 鏂囨。 | Makefile |
| refcheckdocs | 妫€鏌ユ枃妗ｄ腑鎹熷潖鐨勬枃浠跺紩鐢?| Makefile |

## 鍏朵粬鐩爣

| 鐩爣 | 鎻忚堪 | 鏉ユ簮 |
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
| dtbs_prepare | 瀹夎 DTB 鏃跺疄闄呯‘瀹為渶瑕?include/config/kernel.release锛屽洜涓?INSTALL_DTBS_PATH 鍖呭惈 $(KERNELRELEASE)銆備絾鎴戜滑涓嶅笇鏈涜 dtbs_install 渚濊禆浜庡畠锛屽洜涓?dtbs_install 鍙兘浠?root 韬唤杩愯銆?| Makefile |
| headerdep |  | Makefile |
| help | 鏄剧ず鍙敤鐨?make 鐩爣 | Makefile |
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
| rust-analyzer | 涓?rust-analyzer锛圠anguage Server Protocol 鐨勪竴绉嶅疄鐜帮級鐢熸垚 rust-project.json锛堟弿杩伴潪 Cargo Rust 椤圭洰缁撴瀯鐨勬枃浠讹級銆?| Makefile |
| rustavailable | "Rust 鏄惁鍙敤锛? 鐩爣 | Makefile |
| rustdoc | 鏂囨。鐩爣  浣跨敤鍗曟暟褰㈠紡浠ラ伩鍏嶈Е鐘?`no-dot-config-targets`銆?| Makefile |
| rustfmt | 鏍煎紡鍖栫洰鏍? 鐢熸垚鐨勬枃浠朵互鍙?vendored crates 灏嗚璺宠繃銆?| Makefile |
| rustfmtcheck |  | Makefile |
| rusttest | 娴嬭瘯鐩爣 | Makefile |
| scripts | scripts/ 涓瀯寤虹殑棰濆杈呭姪宸ュ叿  浠旂粏鍒楀嚭渚濊禆鍏崇郴锛屼互鍏嶅苟琛屾瀯寤烘椂閲嶅鏋勫缓 scripts銆?| Makefile |
| scripts_basic | 鍦?scripts/basic/ 涓瀯寤虹殑鍩虹杈呭姪宸ュ叿 | Makefile |
| scripts_dtc |  | Makefile |
| scripts_gdb |  | Makefile |
| scripts_gen_packed_field_checks |  | Makefile |
| scripts_unifdef |  | Makefile |
| uapi-asm-generic |  | Makefile |
| usr_gen_init_cpio |  | Makefile |
| versioncheck |  | Makefile |

---

# 瀛愮郴缁熻鏄?

## arch/

鐗瑰畾浜庝綋绯荤粨鏋勭殑浠ｇ爜锛坅rm64銆亁86銆乺iscv銆乵68k銆乸owerpc 绛夛級浠ュ強寮曞鍩虹璁炬柦銆?

## crypto/

鍔犲瘑 API 涓庣畻娉曞疄鐜般€?

## drivers/

璁惧椹卞姩锛堢綉缁溿€佸潡璁惧銆佸瓧绗﹁澶囥€佸０鍗°€丟PU銆乁SB銆丳CI銆乮nfiniband 绛夛級浠ュ強椹卞姩鏍稿績銆?

## fs/

鏂囦欢绯荤粺锛坋xt4銆乥trfs銆亁fs銆乫use銆乷verlayfs銆乶fs銆乯ffs2銆乧ramfs 绛夛級銆?

## include/

鍐呮牳鍏叡澶存枃浠讹紙linux/銆乤sm-generic/銆乽api/锛夈€?

## io_uring/

io_uring 寮傛 I/O 瀛愮郴缁熴€?

## ipc/

杩涚▼闂撮€氫俊锛坢sg銆乻em銆乻hm锛夈€?

## kernel/

鏍稿績鍐呮牳瀛愮郴缁燂紙璋冨害鍣ㄣ€乸rintk銆乮rq銆佹椂闂淬€佸姞閿併€丷CU銆丅PF 绛夛級銆?

## lib/

鍐呮牳閫氱敤搴擄紙浣嶅浘銆乺btree銆乺adix-tree銆乧rc銆乲unit 绛夛級銆?

## mm/

鍐呭瓨绠＄悊锛堥〉鍒嗛厤鍣ㄣ€乻lab銆乿malloc銆乭ugetlb銆乻wap銆乵map 绛夛級銆?

## net/

缃戠粶鍗忚鏍堬紙ipv4銆乮pv6銆乶etfilter銆丅PF銆佹牳蹇冦€佷互澶綉銆佹棤绾跨瓑锛夈€?

## rust/

Rust 鍐呮牳鏀寔锛坆indings銆佹牳蹇冦€乭elpers銆乿endored crates锛夈€?

## samples/

绀轰緥涓庢暀绋嬩唬鐮侊紙BPF銆乿fio-mdev銆乸ktgen锛夈€?

## scripts/

鏋勫缓鑴氭湰銆乧heckpatch銆乧occinelle 琛ヤ竵銆乲config銆乵odpost 绛夈€?

## security/

瀹夊叏妯″潡锛坰elinux銆乤pparmor銆乴andlock銆乻mack 绛夛級銆?

## sound/

ALSA 澹伴煶瀛愮郴缁熶笌闊抽椹卞姩銆?

## tools/

鐢ㄦ埛绌洪棿宸ュ叿锛坧erf銆乥pftool銆乻elftests銆乲unit銆乧pupower 绛夛級銆?

## virt/

铏氭嫙鍖栵紙KVM銆乁ML銆乆en 绛夛級銆?
