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

# Kconfig Summary

## Other

| Config | Type | Description |
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
| FORCE_NR_CPUS | def_bool | This option provides a glob_match function for performing simple text pattern matching.  It originated in the ATA code to blacklist particular drive models, but other 璁惧椹卞姩绋嬪簭 may need simila... |
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
| PC104 | bool | Expose PC/104 form factor 璁惧椹卞姩绋嬪簭 and options available for selection and configuration. Enable this option if your target machine has a PC/104 bus. |
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
| TEST_HMM | tristate | This is a pseudo 璁惧椹卞姩 solely for testing HMM. Say M here if you want to build the HMM test module. Doing so will allow you to run tools/testing/selftest/vm/hmm-tests. If unsure, say N. |
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
