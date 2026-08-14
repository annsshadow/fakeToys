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

| Config | 绫诲瀷 | 鎻忚堪 |
|--------|------|-------------|
| 842_COMPRESS | tristate | 鍚敤鍐呮牳涓?s390x 瀵?zlib 鐨勭‖浠舵敮鎸併€?|
| ADVISE_SYSCALLS | bool | 璇ラ€夐」鍚敤 madvise 涓?fadvise 绯荤粺璋冪敤锛屽簲鐢ㄧ▼搴忓€熸鍚戝唴鏍稿缓璁叾鏈潵鐨勫唴瀛樻垨鏂囦欢浣跨敤鏂瑰紡锛屼粠鑰屾彁鍗囨€ц兘銆傝嫢鏋勫缓鈥︹€?|
| AIO | bool | 璇ラ€夐」鍚敤 POSIX 寮傛 I/O锛岄儴鍒嗛珮鎬ц兘澶氱嚎绋嬪簲鐢ㄥ彲鑳戒細鐢ㄥ埌銆傜鐢ㄦ閫夐」鍙妭鐪佺害 7k銆?|
| ANON_VMA_NAME | bool | 鍏佽涓哄尶鍚嶈櫄鎷熷唴瀛樺尯鍩熷懡鍚嶃€傝鍔熻兘鍙负铏氭嫙鍐呭瓨鍖哄煙鎸囧畾鍚嶇О锛屾墍鎸囧畾鍚嶇О闅忓悗鍙粠 /proc/pid/maps 涓?/proc/pid/smaps 涓鍙栵紝鏈夊姪浜庤瘑鈥︹€?|
| ARCH_FORCE_MAX_ORDER | int | 椤靛潡闃讹紙page block order锛夋寚鐗╃悊杩炵画銆佸彲鍏宠仈杩佺Щ绫诲瀷鐨勯〉闈㈡暟閲忕殑 2 鐨勫箓銆傞〉鍧楅樁鐨勬渶澶у昂瀵歌嚦灏戜负鈥︹€?|
| ARCH_HAS_BINFMT_FLAT | bool | 鏀寔 uClinux FLAT 鏍煎紡浜岃繘鍒舵枃浠躲€?|
| ARCH_HAS_CC_CAN_LINK | bool | 閫夋嫨姝ら」鍙皢 thread_info 浠庢爤涓婄Щ鍏?task_struct銆備负浣挎鍔熻兘鐢熸晥锛屼綋绯荤粨鏋勯渶绉婚櫎闄?flags 澶栫殑鎵€鏈?thread_info 瀛楁骞朵慨澶嶇浉鍏宠繍琛屾椂缂洪櫡銆傚叾涓竴涓粏寰敼鍔ㄢ€︹€?|
| ARCH_HAS_CPU_CACHE_ALIASING | bool | 涓烘敮鎸?HARDENED_USERCOPY 杩涜鏍堝彉閲忕敓鍛藉懆鏈熸鏌ワ紝闇€瑕佷竴绉嶄笌浣撶郴缁撴瀯鏃犲叧鐨勬柟寮忔潵鑾峰彇鏍堟寚閽堛€備竴鏃︽煇浣撶郴缁撴瀯瀹氫箟浜?unsigned long 鍏ㄥ眬鍙橀噺 r鈥︹€?|
| ARCH_HAS_DEBUG_VIRTUAL | bool | 鍦ㄨ櫄鎷熷湴鍧€鍒伴〉鐨勮浆鎹唬鐮佷腑鍚敤涓€浜涗唬浠疯緝楂樼殑鍋ュ叏鎬ф鏌ャ€傚彲鎹曡幏 virt_to_page() 绛夊嚱鏁扮殑璇敤銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| ARCH_HAS_DEBUG_VM_PGTABLE | bool | 褰撴煇浣撶郴缁撴瀯鑳芥垚鍔熸瀯寤哄苟杩愯 DEBUG_VM_PGTABLE 鏃讹紝搴旈€夋嫨姝ら」銆?|
| ARCH_HAS_DEVMEM_IS_ALLOWED | bool | 鑻ョ鐢ㄦ閫夐」锛屽垯鍏佽鐢ㄦ埛绌洪棿锛坮oot锛夎闂叏閮ㄥ唴瀛橈紝鍖呮嫭鍐呮牳涓庣敤鎴风┖闂村唴瀛樸€傛剰澶栬闂樉鐒跺悗鏋滀弗閲嶏紝浣嗙壒瀹氳闂彲鑳解€︹€?|
| ARCH_HAS_ELF_CORE_EFLAGS | bool | 鑻ヤ綋绯荤粨鏋勫埄鐢?ELF 澶翠腑鐨?e_flags 瀛楁鏉ュ瓨鏀惧簲鍦ㄦ牳蹇冭浆鍌ㄤ腑淇濈暀鐨?ABI 鎴栧叾浠栦綋绯荤粨鏋勭浉鍏充俊鎭紝璇烽€夋嫨姝ら」銆?|
| ARCH_HAS_KCOV | bool | 褰撴煇浣撶郴缁撴瀯鑳芥垚鍔熷湪 CONFIG_KCOV 涓嬫瀯寤哄苟杩愯鏃讹紝搴旈€夋嫨姝ら」銆傝繖閫氬父闇€瀵规煇浜涙棭鏈熷紩瀵间唬鐮佺鐢ㄦ彃妗┿€?|
| ARCH_HAS_MEMBARRIER_CALLBACKS | bool | 鍩轰簬浣撶郴缁撴瀯鎺у埗 MSEAL_SYSTEM_MAPPINGS 鐨勮闂€傚唴瀛樺瘑灏佺壒鎬ч渶瑕?64 浣嶅唴鏍搞€傛棤闇€ CPU 鎻愪緵鐗瑰畾纭欢鐗规€с€傝鍚敤姝ょ壒鎬р€︹€?|
| ARCH_HAS_NON_OVERLAPPING_ADDRESS_SPACE | bool |  |
| ARCH_HAS_PTE_SPECIAL | bool | 鍚敤 memfd_secret() 绯荤粺璋冪敤锛屽彲鍒涘缓浠呭湪鎵€灞炶繘绋嬩笂涓嬫枃涓彲瑙併€佷笖涓嶆槧灏勫埌鍏朵粬杩涚▼鍙婂叾浠栧唴鏍搁〉琛ㄧ殑鍐呭瓨鍖哄煙銆?|
| ARCH_HAS_STRNCPY_FROM_USER | bool | 鍦ㄦ煇浜涗笉瀛樺湪鐙珛 I/O 绌洪棿鐨勫钩鍙颁笂锛岄儴鍒?I/O 涓绘満鏃犳硶浠?MMIO 妯″紡璁块棶銆傚€熷姪閫昏緫 PIO 鏈哄埗锛屼富鏈烘湰鍦?I/O 璧勬簮鍙鏄犲皠鍒扮郴缁熲€︹€?|
| ARCH_HAS_USER_SHADOW_STACK | bool | 璇ヤ綋绯荤粨鏋勬彁渚涘鐢ㄦ埛绌洪棿褰卞瓙璋冪敤鏍堬紙shadow call stack锛夌殑纭欢鏀寔锛堜緥濡?x86 CET銆乤rm64 GCS 鎴?RISC-V Zicfiss锛夈€?|
| ARCH_HAS_ZONE_DMA_SET | bool | 璁惧鍐呭瓨鐑彃鎷旀敮鎸佸厑璁稿湪 memmap 涓缓绔?pmem 鎴栧叾浠栫敱璁惧椹卞姩鍙戠幇鐨勫唴瀛樺尯鍩熴€傝繖浣垮緱鍙鍘熸湰鈥滆澶囩墿鐞嗏€濆湴鍧€杩涜 pfn_to_page() 鏌ユ壘鈥︹€?|
| ARCH_NO_SG_CHAIN | def_bool | 鏍堜粨搴擄紙stack depot锛夛細閬垮厤閲嶅鐨勬爤璺熻釜瀛樺偍 |
| ARCH_NO_SWAP | bool | 姝ら€夐」璁╀綘閫夋嫨鍐呮牳鏄惁鏀寔鎵€璋撶殑浜ゆ崲璁惧锛坰wap device锛夋垨浜ゆ崲鏂囦欢锛坰wap file锛夛紝鐢ㄤ簬鎻愪緵姣斿疄闄呯墿鐞?RAM 鏇村鐨勮櫄鎷熷唴瀛樷€︹€?|
| ARCH_SUPPORTS_HUGETLBFS | def_bool | hugetlbfs 鏄熀浜?ramfs 鐨?HugeTLB 椤垫枃浠剁郴缁熷悗绔€傛敮鎸佺殑浣撶郴缁撴瀯璇峰湪姝ら€?Y锛屽苟闃呰 <file:Documentation/admin-guide/mm/hugetlbpage.rst> 浜嗚В缁嗚妭銆傝嫢涓嶇‘瀹氣€︹€?|
| ARCH_SUPPORTS_KMAP_LOCAL_FORCE_MAP | bool | 姝ら€夐」鍦ㄩ潪楂樼鍐呭瓨椤靛強闈為珮绔唴瀛樼郴缁熶笂锛屽己鍒堕€氳繃 kmap_local 鏈哄埗寤虹珛涓存椂鏄犲皠銆傜敓浜х郴缁熻绂佺敤锛?|
| ARCH_SUPPORTS_MEMORY_FAILURE | bool | 鍦ㄥ叿澶?MCA 鎭㈠鑳藉姏鐨勭郴缁熶笂鍚敤浠庨儴鍒嗗唴瀛樻晠闅滀腑鎭㈠鐨勪唬鐮併€傚嵆浣块儴鍒嗗唴瀛樺瓨鍦ㄦ湭绾犳閿欒锛岀郴缁熶粛鍙户缁繍琛屻€傝繖闇€瑕佺壒娈婄殑纭€︹€?|
| ARCH_SUPPORTS_NUMA_BALANCING | bool | 姝ら€夐」娣诲姞瀵硅嚜鍔ㄦ劅鐭?NUMA 鐨勫唴瀛?浠诲姟鏀剧疆鐨勬敮鎸併€傝鏈哄埗杈冧负鍘熷锛屽熀浜庡湪鍐呭瓨寮曠敤鍒颁换鍔℃墍杩愯鐨勮妭鐐规椂杩涜杩佺Щ鈥︹€?|
| ARCH_USE_MEMTEST | bool | 褰撴煇浣撶郴缁撴瀯鍦ㄥ紩瀵艰繃绋嬩腑浣跨敤 early_memtest() 鏃讹紝搴旈€夋嫨姝ら」銆?|
| ARCH_WANT_FRAME_POINTERS | bool | 鑻ラ€?Y锛岀敓鎴愮殑鍐呮牳闀滃儚浼氱◢澶т笖绋嶆參锛屼絾鍦ㄥ唴鏍稿嚭閿欐椂鍙彁渚涢潪甯告湁鐢ㄧ殑璋冭瘯淇℃伅锛堢簿纭殑 oops銆佹爤璺熻釜銆佽鍛婏級銆?|
| ARCH_WANT_GENERAL_HUGETLB | bool | 鍚敤姝ら€夐」鍙檷浣庡ぇ闆?folio锛坔uge zero folio锛夌殑杩愯鏃跺紩鐢ㄨ鏁板紑閿€锛屽苟鎵╁睍鍐呮牳涓彲浣跨敤澶ч浂 folio 鐨勪綅缃€備緥濡傚潡 I/O 鍙粠涓彈鐩娾€︹€?|
| ASSOCIATIVE_ARRAY | bool | 閫氱敤鍏宠仈鏁扮粍銆傚彲鍦ㄨ淇敼鐨勫悓鏃惰繘琛屾煡鎵句笌閬嶅巻銆傚叾鏌ユ壘涓庝慨鏀逛篃鐩稿綋杩呴€熴€傜畻娉曚负闈為€掑綊寮忥紝鏍戠粨鏋勮緝鈥︹€?|
| ASYNC_RAID6_TEST | tristate | 杩欐槸涓€娆℃€ц嚜妫€娴嬭瘯锛屼細閬嶅巻 N 鐩橀樀鍒楁墍鏈夊彲鑳界殑鍙岀洏鏁呴殰鍦烘櫙杩涜鎭㈠銆傛仮澶嶄娇鐢ㄥ紓姝?raid6 鎭㈠渚嬬▼鈥︹€?|
| AS_HAS_NON_CONST_ULEB128 | def_bool | 閫夋嫨鈥淣one鈥濅互澶栫殑鍊间細瀵艰嚧鍐呮牳闀滃儚鍖呭惈璋冭瘯淇℃伅锛屼粠鑰屽澶ч暅鍍忎綋绉€傚畠浼氬悜鍐呮牳涓庢ā鍧楁坊鍔犺皟璇曠鍙凤紙gcc -g锛夛紝骞垛€︹€?|
| ATOMIC64_SELFTEST | tristate | 鍚敤姝ら€夐」鍙湪寮曞鏃舵垨妯″潡鍔犺浇鏃舵祴璇?atomic64_t 鍑芥暟銆傝嫢涓嶇‘瀹氾紝閫?N銆?|
| AUDIT | bool | 鍚敤瀹¤鍩虹璁炬柦锛屽彲涓庡叾浠栧唴鏍稿瓙绯荤粺锛堝 SELinux锛屽叾璁板綍 avc 娑堟伅杈撳嚭闇€瑕佸畠锛夐厤鍚堜娇鐢ㄣ€傜郴缁熻皟鐢ㄥ璁″寘鍚簬浣撶郴缁撴瀯鈥︹€?|
| BACKTRACE_SELF_TEST | tristate | 璇ラ€夐」鎻愪緵涓€涓唴鏍告ā鍧楋紝鐢ㄤ簬娴嬭瘯鍐呮牳鏍堝洖婧唬鐮併€傛閫夐」瀵瑰彂琛岀増鎴栭€氱敤鍐呮牳鏃犵敤锛屼粎瀵瑰唴鏍稿紑鍙戣€呪€︹€?|
| BASE64_KUNIT | tristate | 鏋勫缓 base64 鍗曞厓娴嬭瘯銆傛祴璇曡鐩栧唴鏍镐腑 Base64 鍑芥暟鐨勭紪鐮佷笌瑙ｇ爜閫昏緫銆傞櫎姝ｇ‘鎬ф鏌ュ锛岃繕瀵逛袱绉嶇紪鐮侀兘杩涜浜嗙畝鍗曠殑鎬ц兘鍩哄噯娴嬭瘯鈥︹€?|
| BASE_SMALL | bool | 鍚敤姝ら€夐」鍙缉鍑忓悇绫绘牳蹇冨唴鏍告暟鎹粨鏋勭殑澶у皬銆傝繖鍦ㄥ皬鍨嬫満鍣ㄤ笂鑺傜渷鍐呭瓨锛屼絾鍙兘闄嶄綆鎬ц兘銆?|
| BCH_CONST_M | int | Galois 鍩熼樁鏁扳€渕鈥濈殑甯告暟鍊笺€傝嫢鈥渒鈥濅负瑕佷繚鎶ょ殑浣嶆暟锛屽垯鈥渕鈥濆簲婊¤冻 (k + m*t) <= 2**m - 1銆傞┍鍔ㄥ簲涓鸿绗﹀彿澹版槑榛樿鍊尖€︹€?|
| BCH_CONST_T | int | 绾犻敊鑳藉姏锛堜互姣旂壒涓哄崟浣嶇殑鈥渢鈥濓級鐨勫父鏁板€笺€傝嫢椹卞姩閫夋嫨浜?BCH_CONST_PARAMS 閫夐」锛屽垯搴斾负璇ョ鍙峰０鏄庨粯璁ゅ€笺€? # 濡傞渶鈥︹€?|
| BINARY_PRINTF | def_bool | 鍦ㄥ垵濮嬪寲鏃跺鍏ㄩ儴鍙敤鐨?RAID6 PQ 鍑芥暟杩涜鍩哄噯娴嬭瘯锛屽苟閫夋嫨鏈€蹇殑涓€涓€?|
| BINDGEN_VERSION_TEXT | string | 鍥炴函鍒版瘡涓綋绯荤粨鏋勫悇鑷畾涔?cpu_online_mask 涓?cpu_possible_mask 鐨勬椂浠ｏ紝鍏朵腑涓€浜涘皢鍏跺垵濮嬪寲涓哄叏 1锛屽彟涓€浜涗负鍏?0銆傚綋瀹冧滑琚泦涓寲鈥︹€?|
| BINFMT_ELF | bool | ELF锛圗xecutable and Linkable Format锛屽彲鎵ц涓庡彲閾炬帴鏍煎紡锛夋槸涓€绉嶈法涓嶅悓浣撶郴缁撴瀯涓庢搷浣滅郴缁熶娇鐢ㄧ殑搴撲笌鍙墽琛屾枃浠舵牸寮忋€傚湪姝ら€?Y 灏嗚浣犵殑鍐呮牳鑳藉杩愯 ELF 浜岃繘鍒舵枃浠垛€︹€?|
| BINFMT_ELF_KUNIT_TEST | bool | 鏋勫缓 ELF 鍔犺浇鍣?KUnit 娴嬭瘯锛屽皾璇曞皢浠ュ線鐨勯敊璇慨澶嶆敹闆嗕负鍥炲綊娴嬭瘯闆嗐€傝繖閫氬父浠呯敤浜庤皟璇曘€傛敞鎰忓湪 CONFIG_COMPAT=y 鏃讹紝compat_b鈥︹€?|
| BINFMT_FLAT_ARGVP_ENVP_ON_STACK | bool | 鏀寔鍗佸勾鍓嶇殑 uClinux FLAT 鏍煎紡浜岃繘鍒舵枃浠躲€傞櫎闈炰綘纭畾鎷ユ湁姝ょ被鏂囦欢锛屽惁鍒欏湪姝ら€?N銆?|
| BINFMT_MISC | tristate | 鑻ュ湪姝ら€?Y锛屼究鍙悜鍐呮牳鎻掑叆鐢卞寘瑁呭櫒椹卞姩鐨勪簩杩涘埗鏍煎紡銆傚綋浣犱娇鐢ㄩ渶瑕佽В閲婂櫒鎵嶈兘杩愯鐨勭▼搴忥紙濡?Java銆丳ython鈥︹€︼級鏃朵細鐗瑰埆鏈夌敤銆?|
| BINFMT_SCRIPT | tristate | 鑻ュ笇鏈涙墽琛屼互 #! 寮€澶村苟鍚庤窡瑙ｉ噴鍣ㄨ矾寰勭殑鑴氭湰锛岃鍦ㄦ閫?Y銆備綘鍙互灏嗗叾鏋勫缓涓烘ā鍧楋紱浣嗗湪璇ユā鍧楀姞杞戒箣鍓嶏紝浣犳棤鈥︹€?|
| BINFMT_ZFLAT | bool | 鏀寔 FLAT 鏍煎紡鍘嬬缉浜岃繘鍒舵枃浠?|
| BITFIELD_KUNIT | tristate | 鍚敤姝ら€夐」鍙湪寮曞鏃舵祴璇曚綅鍩熷嚱鏁般€侹Unit 娴嬭瘯鍦ㄥ紩瀵兼湡闂磋繍琛岋紝骞朵互 TAP 鏍煎紡锛坔ttp://testanything.org/锛夊皢缁撴灉杈撳嚭鍒拌皟璇曟棩蹇椼€備粎渚涘唴鏍稿紑鍙戣€呪€︹€?|
| BITOPS_KUNIT | tristate | 璇ラ€夐」鍚敤 bitops 搴撶殑 KUnit 娴嬭瘯锛屾彁渚涗綅鎿嶄綔鍑芥暟銆傛敞鎰忓畠婧愯嚜鍘熷鐨?test_bitops 妯″潡銆傜敤浜庡井鍩哄噯娴嬭瘯涓庣紪璇戔€︹€?|
| BITREVERSE | tristate | 璇ラ€夐」鍦ㄦ煇浜涙敮鎸佹绫绘搷浣滅殑浣撶郴缁撴瀯涓婂惎鐢ㄧ‖浠朵綅鍙嶈浆鎸囦护銆?|
| BITS_TEST | tristate | 鏋勫缓 bits 鍗曞厓娴嬭瘯銆傛祴璇?bits.h 涓畾涔夌殑瀹忕殑閫昏緫銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools 涓殑 KUnit 鏂囨。鈥︹€?|
| BLACKHOLE_DEV_KUNIT_TEST | tristate | 鏋勫缓鈥渂lackhole_dev_kunit鈥濇ā鍧楋紝鐢ㄤ簬楠岃瘉閫氳繃璇ラ粦娲炵綉缁滆澶囩殑鐨勬暟鎹矾寰勩€傝嫢涓嶇‘瀹氾紝閫?N銆?|
| BLK_CGROUP | bool | 閫氱敤鍧?I/O 鎺у埗鍣?cgroup 鎺ュ彛銆傝繖鏄悇绫?I/O 鎺у埗绛栫暐搴斾娇鐢ㄧ殑閫氱敤 cgroup 鎺ュ彛銆傚綋鍓?CFQ I/O 璋冨害鍣ㄧ敤瀹冩潵璇嗗埆浠诲姟缁勨€︹€?|
| BLK_DEV_INITRD | bool | 鍒濆 RAM 鏂囦欢绯荤粺鏄敱寮曞鍔犺浇绋嬪簭锛坙oadlin 鎴?lilo锛夊姞杞界殑 ramfs锛屽苟鍦ㄦ甯稿紩瀵兼祦绋嬩箣鍓嶆寕杞戒负鏍规枃浠剁郴缁熴€傚畠閫氬父鐢ㄤ簬鍔犺浇鎵€闇€妯″潡鈥︹€?|
| BOOTPARAM_HUNG_TASK_PANIC | int | 褰撹涓洪潪闆跺€兼椂锛岃嫢鍦ㄥ崟娆℃壂鎻忎腑鍙戠幇鐨勬寕璧蜂换鍔℃暟閲忚揪鍒拌鍊硷紝灏嗚Е鍙戝唴鏍?panic銆傝 panic 鍙笌 panic_timeout 閰嶅悎浣跨敤锛屼互鈥︹€?|
| BOOTPARAM_SOFTLOCKUP_PANIC | int | 璁句负闈為浂鍊?N锛屼娇鍐呮牳鍦ㄥ嚭鐜扳€滆蒋閿佹锛坰oft lockup锛夆€濇椂 panic锛涜蒋閿佹鏄寚瀵艰嚧鍐呮牳鍦ㄥ唴鏍告ā寮忎笅寰幆瓒呰繃 (N * 20 绉?锛堝彲浣跨敤 watchdo鈥︹€﹂厤缃級鐨勭己闄枫€?|
| BOOTPARAM_WQ_STALL_PANIC | int | 璁剧疆瑙﹀彂鍐呮牳 panic 鐨勫伐浣滈槦鍒楀仠婊炴鏁般€傚綋宸ヤ綔绾跨▼姹犲湪瓒呰繃 30 绉掞紙鍙娇鐢ㄢ€︹€﹂厤缃級鍐呭鏌愪釜寰呭鐞嗗伐浣滈」娌℃湁杩涘睍鏃讹紝鍗冲彂鐢熷伐浣滈槦鍒楀仠婊炪€?|
| BOOT_CONFIG | bool | 棰濆鐨勫紩瀵奸厤缃厑璁哥郴缁熺鐞嗗憳鍦ㄥ唴鏍稿紩瀵兼椂锛屽皢涓€浠介厤缃枃浠朵綔涓哄唴鏍稿懡浠よ鍙傛暟鐨勮ˉ鍏呮墿灞曚紶鍏ャ€傝寮曞閰嶇疆鏂囦欢蹇呴』浠ユ牎楠屽拰褰㈠紡闄勫姞鍦?initramfs 鏈熬锛屽洜鈥︹€?|
| BOOT_CONFIG_EMBED | bool | 灏?BOOT_CONFIG_EMBED_FILE 鎸囧畾鐨?bootconfig 鏂囦欢宓屽叆鍐呮牳銆傞€氬父 bootconfig 鏂囦欢闅?initrd 闀滃儚鍔犺浇銆備絾鑻ョ郴缁熶笉鏀寔 initrd锛屾閫夐」浼氭湁鎵€甯姪鈥︹€?|
| BOOT_CONFIG_EMBED_FILE | string | 鎸囧畾灏嗚宓屽叆鍐呮牳鐨?bootconfig 鏂囦欢銆傚綋 initrd 涓病鏈夛紝鎴?initrd 涓病鏈夊叾浠?bootconfig 鏃讹紝灏嗕娇鐢ㄦ bootconfig銆?|
| BOOT_CONFIG_FORCE | bool | 璁剧疆姝?Kconfig 閫夐」鍚庯紝鍗充娇鐪佺暐鈥渂ootconfig鈥濆唴鏍稿紩瀵煎弬鏁帮紝涔熶細鎵ц BOOT_CONFIG 澶勭悊銆備簨瀹炰笂锛岃缃閫夐」鍚庯紝鏃犳硶浣垮唴鏍糕€︹€?|
| BOOT_PRINTK_DELAY | bool | 璇ョ紪璇戦€夐」鍏佽浣犲湪姣忔潯鍐呮牳鍚姩娑堟伅鍚庢彃鍏ヤ竴涓煭鏆傚欢杩熸潵闃呰瀹冧滑銆傚欢杩熷湪鍐呮牳鍛戒护琛屼笂浠ユ绉掍负鍗曚綅鎸囧畾锛屼娇鐢?"boot_delay=N"銆傚叾... |
| BRIDGE_NETFILTER | tristate | 鍚敤璇ラ€夐」灏嗚 arptables 涓?iptables 鍒嗗埆鐪嬪埌妗ユ帴鐨?ARP 涓?IP 娴侀噺銆傚鏋滀綘鎯宠涓€涓ˉ鎺ラ槻鐏锛屽ぇ姒備細甯屾湜鍚敤璇ラ€夐」銆傚惎鐢ㄦ垨绂佺敤璇ラ€夐」浼?.. |
| BROKEN | bool | 璇ラ€夐」鍏佽浣犻€夋嫨鏄惁灏濊瘯缂栬瘧锛堝苟淇锛夊皻鏈洿鏂板埌鏂板熀纭€璁炬柦鐨勬棫椹卞姩銆?|
| BROKEN_ON_SMP | bool | 浠庡唴鏍稿懡浠よ浼犻€掔粰 init 鐨勫弬鏁版暟閲忎笌鐜鍙橀噺鏁伴噺鍚勮嚜鐨勬渶澶у€笺€?|
| BSD_PROCESS_ACCT | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛岀敤鎴风骇绋嬪簭灏辫兘锛堥€氳繃鐗规畩鐨勭郴缁熻皟鐢級鎸囩ず鍐呮牳灏嗚繘绋嬭璐︿俊鎭啓鍏ヤ竴涓枃浠讹細姣忓綋杩涚▼閫€鍑烘椂锛屽叧浜庤杩涚▼... |
| BSD_PROCESS_ACCT_V3 | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛岃繘绋嬭璐︿俊鎭皢浠ヤ竴绉嶆柊鐨勬枃浠舵牸寮忓啓鍏ワ紝璇ユ牸寮忚繕浼氳褰曟瘡涓繘绋嬪強鍏剁埗杩涚▼鐨勮繘绋?ID銆傛敞鎰忚鏂囦欢鏍煎紡涓?.. |
| BUG | bool | 绂佺敤璇ラ€夐」浼氱Щ闄ゅ BUG 鍜?WARN 鐨勬敮鎸侊紝鍑忓皬鍐呮牳闀滃儚浣撶Н锛屽苟鍙兘鎮勭劧蹇界暐澶ч噺鑷村懡鐘跺喌銆備綘鍙簲鍦ㄧ‘鏈夊繀瑕佺殑鎯呭喌涓嬭€冭檻绂佺敤... |
| BUILD_SALT | string | 鏋勫缓 ID 鐢ㄤ簬閾炬帴浜岃繘鍒舵枃浠跺強鍏惰皟璇曚俊鎭€傝缃閫夐」灏嗗湪鏋勫缓 ID 鐨勮绠椾腑浣跨敤璇ュ€笺€傝繖瀵逛簬甯屾湜纭繚... |
| BUILTIN_MODULE_RANGES | bool | 褰撴ā鍧楄鏋勫缓杩涘唴鏍告椂锛屽叾鍦?/proc/kallsyms 涓殑绗﹀彿涓嶄細鍏宠仈妯″潡鍚嶃€傝窡韪櫒鍙兘甯屾湜涓嶈... |
| CACHESTAT_SYSCALL | bool | 鍚敤 cachestat 绯荤粺璋冪敤锛屽畠鏌ヨ鏂囦欢鐨勯〉缂撳瓨缁熻淇℃伅锛堝凡缂撳瓨椤垫暟銆佽剰椤垫暟銆佹爣璁颁负鍥炲啓鐨勯〉銆佷互鍙婏紙鏈€杩戯級琚€愬嚭鐨勯〉锛夈€傚鏋滀笉纭畾锛屽湪姝ら€夋嫨 Y銆?|
| CC_IS_GCC | def_bool | 瀹冧笉渚濊禆浜?`RUST`锛屽洜涓哄悗鑰呭彲鑳介渶瑕佸湪 `depends on` 涓娇鐢ㄨ鐗堟湰銆?|
| CC_OPTIMIZE_FOR_PERFORMANCE | bool | 杩欐槸鍐呮牳鐨勯粯璁や紭鍖栫骇鍒紝浣跨敤 "-O2" 缂栬瘧鍣ㄦ爣蹇楄繘琛屾瀯寤猴紝浠ヨ幏寰楁渶浣虫€ц兘鍜屾渶鏈夊府鍔╃殑缂栬瘧鏈熻鍛娿€?|
| CC_OPTIMIZE_FOR_SIZE | bool | 閫夋嫨璇ラ€夐」浼氬悜缂栬瘧鍣ㄤ紶閫?"-Os"锛屼粠鑰岀敓鎴愭洿灏忕殑鍐呮牳銆?|
| CC_VERSION_TEXT | string | 鍏剁敤閫斿苟涓嶆槑纭細 - 褰撶紪璇戝櫒鏇存柊鏃堕噸鏂拌繍琛?Kconfig 'default' 灞炴€у紩鐢ㄧ幆澧冨彉閲?CC_VERSION_TEXT锛屽洜姝ゅ畠浼氳璁板綍鍦?include/config/auto.conf... |
| CGROUP_BPF | bool | 鍏佽浣跨敤 bpf(2) 绯荤粺璋冪敤鍛戒护 BPF_PROG_ATTACH 灏?eBPF 绋嬪簭闄勫姞鍒?cgroup銆傝繖浜涚▼搴忓湪浣曠涓婁笅鏂囦腑琚闂彇鍐充簬闄勫姞鐨勭被鍨嬨€備緥濡傦紝绋嬪簭... |
| CGROUP_CPUACCT | bool | 鎻愪緵涓€涓畝鍗曠殑鎺у埗鍣紝鐢ㄤ簬鐩戞帶 cgroup 涓换鍔℃秷鑰楃殑鎬?CPU銆?|
| CGROUP_DEBUG | bool | 璇ラ€夐」鍚敤涓€涓畝鍗曠殑鎺у埗鍣紝瀵煎嚭鍏充簬 cgroups 妗嗘灦鐨勮皟璇曚俊鎭€傝鎺у埗鍣ㄤ粎鐢ㄤ簬 cgroup 璋冭瘯銆傚叾鎺ュ彛涓嶇ǔ瀹氥€傞€夋嫨 N銆?|
| CGROUP_DMEM | bool | DMEM 鎺у埗鍣ㄥ厑璁稿吋瀹硅澶囧熀浜?cgroup 灞傜骇闄愬埗璁惧鍐呭瓨浣跨敤銆備緥濡傦紝瀹冨厑璁镐綘闄愬埗 DRM 瀛愮郴缁熶腑搴旂敤绋嬪簭鐨?VRAM 浣跨敤銆?|
| CGROUP_FREEZER | bool | 鎻愪緵涓€绉嶅喕缁撳拰瑙ｅ喕 cgroup 涓墍鏈変换鍔＄殑鏂规硶銆傝閫夐」褰卞搷鍘熷鐨?cgroup 鎺ュ彛銆俢group2 鍐呭瓨鎺у埗鍣ㄩ粯璁ゅ寘鍚噸瑕佺殑鍐呮牳鍐呭唴瀛樻秷鑰楄€?.. |
| CGROUP_HUGETLB | bool | 鎻愪緵涓€涓敤浜?HugeTLB 椤电殑 cgroup 鎺у埗鍣ㄣ€傚惎鐢ㄥ悗锛屼綘鍙互瀵?HugeTLB 浣跨敤璁剧疆姣忎釜 cgroup 鐨勯檺鍒躲€傝闄愬埗鍦ㄧ己椤垫椂寮哄埗瀹炴柦銆傜敱浜?HugeTLB 涓嶆敮鎸侀〉閲?.. |
| CGROUP_MISC | bool | 涓轰富鏈轰笂鐨勬潅椤硅祫婧愭彁渚涙帶鍒跺櫒銆傛潅椤规爣閲忚祫婧愭槸涓绘満绯荤粺涓婃棤娉曞儚鍏朵粬 cgroup 閭ｆ牱琚娊璞＄殑璧勬簮銆傝鎺у埗鍣?.. |
| CGROUP_NET_CLASSID | bool | 鐢ㄤ綔閫氱敤濂楁帴瀛?classid 鏍囪鐨?cgroup 瀛愮郴缁燂紝鐢ㄤ簬 cls_cgroup 鍜?netfilter 鍖归厤銆?|
| CGROUP_PERF | bool | 璇ラ€夐」鎵╁睍 perf 鐨勬瘡 CPU 妯″紡锛屽皢鐩戞帶闄愬埗涓哄睘浜庢寚瀹?cgroup 骞跺湪鎸囧畾 CPU 涓婅繍琛岀殑绾跨▼銆傛垨鑰呭彲鐢ㄤ簬鍦ㄩ噰鏍蜂腑鎼哄甫 cgroup ID锛屼粠鑰?.. |
| CGROUP_PIDS | bool | 鍦?cgroup 鑼冨洿鍐呭己鍒跺疄鏂借繘绋嬫暟閲忛檺鍒躲€備换浣曡秴鍑?cgroup 鍏佽鏁伴噺鑰?fork 鏇村杩涚▼鐨勫皾璇曢兘浼氬け璐ャ€侾ID 浠庢牴鏈笂鏄竴绉嶅叏灞€璧勬簮锛屽洜涓?.. |
| CGROUP_RDMA | bool | 寮哄埗瀹炴柦鐢?IB 鏍堝畾涔夌殑 RDMA 璧勬簮銆備娇鐢ㄨ€呭緢瀹规槗鑰楀敖 RDMA 璧勬簮锛屼粠鑰屽鑷村叾浠栦娇鐢ㄨ€呮棤娉曡幏寰楄祫婧愩€俁DMA 鎺у埗... |
| CGROUP_WRITEBACK | bool | 璇ョ壒鎬ц CPU 璋冨害鍣ㄨ瘑鍒换鍔＄粍锛屽苟鎺у埗鍚戣繖浜涗换鍔＄粍鐨?CPU 甯﹀鍒嗛厤銆傚畠浣跨敤 cgroup 灏嗕换鍔″垎缁勩€備緷璧栦簬 CGROUP_SCHED |
| CHECKPOINT_RESTORE | bool | 涓烘鏌ョ偣/鎭㈠鍚敤棰濆鐨勫唴鏍哥壒鎬с€傜壒鍒槸瀹冩坊鍔犱簡杈呭姪鐨?prctl 浠ｇ爜鏉ヨ缃繘绋嬫枃鏈€佹暟鎹拰鍫嗘澶у皬锛屼互鍙婂皯閲忛澶栫殑 /proc 鏂囦欢... |
| CHECKSUM_KUNIT | tristate | Enable this option to test the checksum functions at boot. KUnit tests run during boot and output the results to the debug log in TAP format (http://testanything.org/). Only useful for kernel devs ... |
| CLOSURES | bool | 瀵?cpumask_var_t 浣跨敤鍔ㄦ€佸垎閰嶏紝鑰屼笉鏄皢鍏舵斁鍦ㄦ爤涓娿€傝繖鏍峰紑閿€鐣ュぇ锛屼絾鍙伩鍏嶆爤婧㈠嚭銆?|
| CMA_AREAS | int | CMA 鍏佽涓虹壒瀹氱敤閫斿垱寤?CMA 鍖哄煙锛屼富瑕佺敤浣滆澶囩鏈夊尯鍩熴€傝鍙傛暟璁剧疆绯荤粺涓?CMA 鍖哄煙鐨勬渶澶ф暟閲忋€傚鏋滀笉纭畾锛屼繚鐣欓粯璁ゅ€?"8"... |
| CMA_DEBUGFS | bool | 寮€鍚?CMA 鐨?DebugFS 鎺ュ彛銆?|
| CMA_SYSFS | bool | 璇ラ€夐」鏆撮湶涓€浜?sysfs 灞炴€э紝浠ヤ究浠?CMA 鑾峰彇淇℃伅銆?|
| CMDLINE_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 cmdline API 鍗曞厓娴嬭瘯锛屾祴璇?cmdline.c 鎵€鎻愪緵鐨?API 閫昏緫銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation 涓殑 KUnit 鏂囨。... |
| CMDLINE_LOG_WRAP_IDEAL_LEN | int | 鍦ㄥ惎鍔ㄦ椂锛屽唴鏍稿懡浠よ浼氳璁板綍鍒版帶鍒跺彴銆傛棩蹇楁秷鎭互鍓嶇紑 "Kernel command line: " 寮€澶淬€傝鏃ュ織娑堟伅浼氬皾璇曟崲琛岋紙鎷嗗垎涓哄琛?.. |
| CODE_TAGGING | bool | 璺熻釜鍒嗛厤婧愪唬鐮佸苟璁板綍鍦ㄨ浠ｇ爜浣嶇疆鍙戣捣鐨勫垎閰嶆€诲ぇ灏忋€傝鏈哄埗鍙敤浜庝互杈冧綆鐨勬€ц兘鍜屽唴瀛樺紑閿€璺熻釜鍐呭瓨娉勬紡銆?|
| COMPACTION | bool | 鍐呭瓨瑙勬暣锛坈ompaction锛夋槸鍞竴鑳藉彲闈犲湴褰㈡垚楂橀樁锛堟洿澶х殑鐗╃悊杩炵画锛夊唴瀛樺潡鐨勫唴瀛樼鐞嗙粍浠躲€傞〉鍒嗛厤鍣ㄤ弗閲嶄緷璧栧唴瀛樿鏁达紝鑰岃鐗规€х殑缂哄け... |
| COMPACT_UNEVICTABLE_DEFAULT | int | 绌洪棽椤垫姤鍛婂厑璁镐粠浼欎即鍒嗛厤鍣ㄥ閲忚幏鍙栫┖闂查〉锛屼互渚垮皢杩欎簺椤垫姤鍛婄粰鍙︿竴涓疄浣擄紙渚嬪铏氭嫙鏈虹洃鎺у櫒锛夛紝浠庤€屼娇鍐呭瓨... |
| COMPAT_BINFMT_ELF | def_bool | ELF FDPIC 浜岃繘鍒跺熀浜?ELF锛屼絾鍏佽浜岃繘鍒跺悇涓姞杞芥褰兼鐙珛鍦颁綅浜庡唴瀛樹腑銆傝繖浣垮緱璇ユ牸寮忛潪甯搁€傜敤浜?.. |
| COMPAT_BRK | bool | 闅忔満鍖栧爢甯冨眬浣垮爢婕忔礊鍒╃敤鏇村洶闅撅紝浣嗕篃浼氱牬鍧忓彜鑰佷簩杩涘埗锛堝寘鎷换浣曞熀浜?libc5 鐨勭▼搴忥級銆傝閫夐」灏嗗惎鍔ㄩ粯璁ゅ€兼敼涓虹鐢ㄥ爢闅忔満鍖栵紝浠?.. |
| COMPAT_NETLINK_MESSAGES | def_bool | 璇ラ€夐」浣垮緱鑳藉鏍规嵁浠诲姟鏄惁涓哄吋瀹癸紙compat锛変换鍔★紝鍚戜换鍔″彂閫佷笉鍚岀殑 netlink 娑堟伅銆備负姝わ紝浣犻渶瑕佸皢 skb_shinfo(skb)->frag_list 璁剧疆涓?.. |
| COMPILE_TEST | bool | 鏌愪簺椹卞姩鍙互鍦ㄤ笌鍏堕鏈熻繍琛屽钩鍙颁笉鍚岀殑骞冲彴涓婄紪璇戙€傚敖绠″畠浠棤娉曞湪閭ｉ噷鍔犺浇锛堟垨鑰呭嵆浣垮姞杞戒篃浼氬洜缂哄皯纭欢鏀寔鑰屾棤娉曚娇鐢級锛?.. |
| CONSOLE_LOGLEVEL_DEFAULT | int | 鐢ㄤ簬鍐冲畾鍦ㄦ帶鍒跺彴涓婃墦鍗板唴瀹圭殑榛樿鏃ュ織绾у埆銆傚湪姝よ缃粯璁ゅ€肩瓑浠蜂簬鍦ㄥ唴鏍稿惎鍔ㄥ弬鏁颁腑浼犲叆 loglevel=<x>銆俵oglevel=<x> 浠嶄細瑕嗙洊... |
| CONSOLE_LOGLEVEL_QUIET | int | 褰撳唴鏍稿懡浠よ浼犲叆 "quiet" 鏃朵娇鐢ㄧ殑鏃ュ織绾у埆銆傚綋鍐呮牳鍛戒护琛屼紶鍏?"quiet" 鏃讹紝灏嗕娇鐢ㄨ鏃ュ織绾у埆銆傛崲瑷€涔嬶紝浼犲叆 "quiet" 绛変环浜?.. |
| CONTEXT_ANALYSIS_TEST | bool | 璇ラ€夐」鏋勫缓閽堝鍩轰簬缂栬瘧鍣ㄧ殑涓婁笅鏂囧垎鏋愮殑娴嬭瘯銆傝娴嬭瘯涓嶄細鍚戝唴鏍告坊鍔犲彲鎵ц浠ｇ爜锛岃€屾槸鐢ㄤ簬娴嬭瘯鍒嗘瀽鎵€鏀寔鐨勯€氱敤妯″紡涓嶄細瀵艰嚧... |
| CONTIG_ALLOC | def_bool | 鍦ㄩ〉鍒嗛厤鍣ㄤ腑锛孭CP锛堟瘡 CPU 椤甸泦锛変互鎵规柟寮忚ˉ鍏呭拰娓呯┖銆傛壒鏁伴噺浼氳嚜鍔ㄧ缉鏀句互鎻愰珮椤靛垎閰?閲婃斁鍚炲悙銆備絾杩囧ぇ鐨勭缉鏀惧洜瀛愬彲鑳芥崯... |
| COREDUMP | bool | 璇ラ€夐」鍚敤瀵圭敓鎴愭牳蹇冭浆鍌ㄧ殑鏀寔銆備綘鍑犱箮鑲畾搴旇鍦ㄦ閫夋嫨 Y銆傚浜庝粠涓嶉渶瑕佽皟璇曟垨鍙繍琛屾棤鐟曚唬鐮佺殑绯荤粺鍒欓潪蹇呴渶銆?|
| CORE_DUMP_DEFAULT_ELF_HEADERS | bool | ELF 鏍稿績杞偍鏂囦欢鎻忚堪宕╂簝杩涚▼鐨勬瘡涓唴瀛樻槧灏勶紝骞跺彲鍖呭惈鎴栫渷鐣ユ瘡涓槧灏勭殑鍐呭瓨鍐呭銆傛湭淇敼鐨勬枃鏈槧灏勫唴瀹归粯璁よ鐪佺暐銆?.. |
| CPUMASK_KUNIT_TEST | tristate | 鍚敤 cpumask 娴嬭瘯锛屽湪鍚姩鎴栨ā鍧楀姞杞芥椂杩愯銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit 涓殑 KUnit 鏂囨。... |
| CPUSETS | bool | 璇ラ€夐」鍏佽浣犲垱寤哄拰绠＄悊 CPUSET锛屼粠鑰屽皢绯荤粺鍔ㄦ€佸垝鍒嗕负 CPU 鍜屽唴瀛樿妭鐐圭殑闆嗗悎锛屽苟鎸囨淳浠诲姟浠呭湪杩欎簺闆嗗悎鍐呰繍琛屻€傝繖涓昏... |
| CPUSETS_V1 | bool | 宸茶 cgroup v2 瀹炵幇寮冪敤鐨勪紶缁?cgroup v1 cpusets 鎺у埗鍣ㄣ€倂1 淇濈暀鐢ㄤ簬灏氭湭杩佺Щ鍒版柊 cgroup v2 鎺ュ彛鐨勪紶缁熷簲鐢ㄣ€備紶缁?.. |
| CPU_HOTPLUG_STATE_CONTROL | bool | 鍏佽鍚?CPU 鐨?sysfs 鐩爣鏂囦欢鍐欏叆 "offline" 涓?"online" 涔嬮棿鐨勯樁姊楠わ紝浠庤€屽彲浠ョ粏绮掑害鍦伴€愭鍒囨崲鐘舵€併€傜洰鍓嶈繖鏄竴涓皟璇曢€夐」锛屽洜涓虹儹鎻掓嫈鏈哄埗鏃犳硶鍋滄鍜岄噸... |
| CPU_ISOLATION | bool | 纭繚杩愯鍏抽敭浠诲姟鐨?CPU 涓嶅彈浠讳綍"鍣０"婧愬共鎵帮紝渚嬪鏈粦瀹氱殑宸ヤ綔闃熷垪銆佸畾鏃跺櫒銆佸唴鏍哥嚎绋嬬瓑銆傛湭缁戝畾鐨勪换鍔′細琚嵏杞藉埌绠″锛坔ousekeeping锛塁PU 涓娿€傝繖鐢?.. |
| CROSS_MEMORY_ATTACH | bool | 鍚敤璇ラ€夐」浼氭坊鍔?process_vm_readv 涓?process_vm_writev 绯荤粺璋冪敤锛屽厑璁稿叿鏈夌浉搴旀潈闄愮殑杩涚▼鐩存帴璇诲彇鎴栧啓鍏ュ彟涓€涓繘绋嬬殑鍦板潃绌洪棿銆?.. |
| CRYPTO | tristate | 璇ラ€夐」鎻愪緵鏍稿績鍔犲瘑 API銆備緷璧栦簬 CRYPTO |
| CRYPTO_842 | tristate | 842 compression algorithm by IBM See https://github.com/plauth/lib842 for further information. |
| CRYPTO_ADIANTUM | tristate | Adiantum 鍙皟鏁淬€佷繚闀跨殑鍔犲瘑妯″紡銆傝璁＄敤浜庡揩閫熶笖瀹夊叏鐨勭鐩樺姞瀵嗭紝灏ゅ叾閫傜敤浜庢病鏈変笓鐢ㄥ姞瀵嗘寚浠ょ殑 CPU銆傚畠浣跨敤 XCha... |
| CRYPTO_AEGIS128 | tristate | AEGIS-128 AEAD 绠楁硶 |
| CRYPTO_AEGIS128_SIMD | bool | AEGIS-128 AEAD 绠楁硶銆傛灦鏋勶細arm 鎴?arm64锛屼娇鐢細 - NEON锛堥珮绾?SIMD锛夋墿灞?|
| CRYPTO_AES | tristate | AES 鍔犲瘑绠楁硶锛圧ijndael锛夛紙FIPS-197銆両SO/IEC 18033-3锛夈€俁ijndael 鍦ㄥ箍娉涚殑纭欢涓庤蒋浠惰绠楃幆澧冧腑閮藉缁堣〃鐜颁紭寮?.. |
| CRYPTO_ALGAPI | tristate | 璇ラ€夐」鎻愪緵鍔犲瘑绠楁硶鐨?API銆?|
| CRYPTO_ALGAPI2 | tristate | 杩欐彁渚涗簡瀹炰緥鍖栬濡?cbc(aes) 绛夋ā鏉跨殑鏀寔锛屼互鍙婂姞瀵嗚嚜娴嬬殑鏀寔銆?|
| CRYPTO_ANUBIS | tristate | Anubis cipher algorithm Anubis is a variable key length cipher which can use keys from 128 bits to 320 bits in length.  It was evaluated as a entrant in the NESSIE competition. See https://web.arch... |
| CRYPTO_ARC4 | tristate | ARC4 鍔犲瘑绠楁硶銆侫RC4 鏄竴绉嶆祦瀵嗙爜锛屼娇鐢ㄩ暱搴︿粠 8 浣嶅埌 2048 浣嶇殑瀵嗛挜銆傝绠楁硶鏄熀浜庨┍鍔ㄧ殑 WEP 鎵€蹇呴渶鐨勶紝浣嗕笉搴旇鐢ㄤ簬鍏朵粬鐩殑锛屽洜涓?.. |
| CRYPTO_ARIA | tristate | ARIA 鍔犲瘑绠楁硶锛圧FC5794锛夈€侫RIA 鏄煩鍥芥爣鍑嗗姞瀵嗙畻娉曘€侫RIA 瑙勫畾浜嗕笁绉嶅瘑閽ラ暱搴︿笌杞暟锛?28 浣?12 杞紝192 浣?14 杞紝256 浣?16... |
| CRYPTO_AUTHENC | tristate | Authenc锛氱敤浜?IPsec 鐨勭粍鍚堟ā寮忓皝瑁呫€傝繖鏄?IPSec ESP锛圶FRM_ESP锛夋墍蹇呴渶鐨勩€?|
| CRYPTO_BENCHMARK | tristate | 绮楃暐鐨勫姞瀵嗗熀鍑嗘祴璇曟ā鍧椼€備富瑕佷緵鍦ㄥ唴鏍镐腑寮€鍙戝姞瀵嗙畻娉曠殑浜哄憳浣跨敤銆備笉搴斿湪鐢熶骇鍐呮牳涓惎鐢ㄣ€?|
| CRYPTO_BLAKE2B | tristate | BLAKE2b 鍔犲瘑鍝堝笇鍑芥暟锛圧FC 7693锛夈€侭LAKE2b 閽堝 64 浣嶅钩鍙颁紭鍖栵紝鍙敓鎴?1 鍒?64 瀛楄妭涔嬮棿浠绘剰澶у皬鐨勬憳瑕併€傚瘑閽ュ寲鍝堝笇涔熷凡瀹炵幇銆傝妯″潡... |
| CRYPTO_BLOWFISH | tristate | Blowfish 鍔犲瘑绠楁硶锛岀敱 Bruce Schneier 璁捐銆傝繖鏄竴绉嶅彲鍙樺瘑閽ラ暱搴︾殑瀵嗙爜锛屽彲浣跨敤 32 浣嶅埌 448 浣嶇殑瀵嗛挜銆傚畠蹇€熴€佺畝鍗曪紝涓撲负鍦?澶у瀷... |
| CRYPTO_BLOWFISH_COMMON | tristate | Blowfish 鍔犲瘑绠楁硶鐢遍€氱敤 C 瀹炵幇涓庢眹缂栧疄鐜板叡浜殑閫氱敤閮ㄥ垎銆?|
| CRYPTO_CAMELLIA | tristate | Camellia 鍔犲瘑绠楁硶锛圛SO/IEC 18033-3锛夈€侰amellia 鏄敱 NTT 涓庝笁鑿辩數鏈鸿仈鍚堝紑鍙戠殑瀵圭О瀵嗛挜鍒嗙粍瀵嗙爜銆侰amellia 瑙勫畾浜嗕笁绉嶅瘑閽ラ暱搴︼細128銆?92... |
| CRYPTO_CAST5 | tristate | CAST5锛圕AST-128锛夊姞瀵嗙畻娉曪紙RFC2144銆両SO/IEC 18033-3锛?|
| CRYPTO_CAST6 | tristate | CAST6锛圕AST-256锛夊姞瀵嗙畻娉曪紙RFC2612锛?|
| CRYPTO_CAST_COMMON | tristate | CAST 鍔犲瘑绠楁硶鐢遍€氱敤 C 瀹炵幇涓庢眹缂栧疄鐜板叡浜殑閫氱敤閮ㄥ垎銆?|
| CRYPTO_CBC | tristate | CBC锛堝瘑鐮佸潡閾炬帴锛夋ā寮忥紙NIST SP800-38A锛夈€傝鍒嗙粍瀵嗙爜妯″紡鏄?IPSec ESP锛圶FRM_ESP锛夋墍蹇呴渶鐨勩€?|
| CRYPTO_CCM | tristate | CCM锛堣鏁板櫒涓庡瘑鐮佸潡閾炬帴-娑堟伅璁よ瘉鐮侊級璁よ瘉鍔犲瘑妯″紡锛圢IST SP800-38C锛?|
| CRYPTO_CHACHA20 | tristate | ChaCha20銆乆ChaCha20 涓?XChaCha12 娴佸瘑鐮佺畻娉曘€侰haCha20 鏄敱 Daniel J. Bernstein 璁捐鐨?256 浣嶉珮閫熸祦瀵嗙爜锛屽苟鍦?RFC7539 涓繘涓€姝ヨ瀹氱敤浜?IETF 鍗忚... |
| CRYPTO_CHACHA20POLY1305 | tristate | ChaCha20 娴佸瘑鐮佷笌 Poly1305 璁よ瘉鍣ㄧ粍鍚堟ā寮忥紙RFC8439锛?|
| CRYPTO_CMAC | tristate | CMAC锛堝熀浜庡瘑鐮佺殑娑堟伅璁よ瘉鐮侊級璁よ瘉妯″紡锛圢IST SP800-38B 涓?IETF RFC4493锛?|
| CRYPTO_CRC32 | tristate | CRC32 CRC 绠楁硶锛圛EEE 802.3锛?|
| CRYPTO_CRC32C | tristate | CRC32c CRC 绠楁硶锛屼娇鐢?iSCSI 澶氶」寮忥紙RFC 3385 涓?RFC 3720锛夈€傝繖鏄竴绉?32 浣?CRC锛堝惊鐜啑浣欐牎楠岋級锛屽叾澶氶」寮忕敱 G. Castagnoli銆丼. Braeuer 涓?M. Herrman 鍦?Optimization... |
| CRYPTO_CRYPTD | tristate | 杩欐槸涓€涓€氱敤鐨勮蒋浠跺紓姝ュ姞瀵嗗畧鎶よ繘绋嬶紝鍙皢浠绘剰鍚屾杞欢鍔犲瘑绠楁硶杞崲涓哄湪鍐呮牳绾跨▼涓墽琛岀殑寮傛绠楁硶銆?|
| CRYPTO_CTR | tristate | CTR锛堣鏁板櫒锛夋ā寮忥紙NIST SP800-38A锛?|
| CRYPTO_CTS | tristate | CTS锛堝瘑鏂囩獌鍙栵級鐨?CBC-CS3 鍙樹綋锛圢IST 瀵?SP800-38A 鐨勯檮褰曪紙2010 骞?10 鏈堬級锛夈€傝妯″紡鏄?Kerberos gss 鏈哄埗鏀寔 AES 鍔犲瘑鎵€蹇呴渶鐨勩€?|
| CRYPTO_DEFLATE | tristate | Deflate 鍘嬬缉绠楁硶锛圧FC1951锛夈€傜敱 IPSec 閰嶅悎 IPCOMP 鍗忚锛圧FC3173銆丷FC2394锛変娇鐢ㄣ€?|
| CRYPTO_DES | tristate | DES锛堟暟鎹姞瀵嗘爣鍑嗭級锛團IPS 46-2銆両SO/IEC 18033-3锛変笌涓夐噸 DES EDE锛堝姞瀵?瑙ｅ瘑/鍔犲瘑锛夛紙FIPS 46-3銆両SO/IEC 18033-3锛夊姞瀵嗙畻娉曘€?|
| CRYPTO_DH | tristate | DH锛圖iffie-Hellman锛夊瘑閽ヤ氦鎹㈢畻娉曘€?|
| CRYPTO_DH_RFC7919_GROUPS | bool | RFC7919 涓畾涔夌殑 FFDHE锛堝熀浜庢湁闄愬煙鐨?Diffie-Hellman 涓存椂锛夌粍銆傚湪 DH 瀵嗛挜浜ゆ崲涓敮鎸佽繖浜涙湁闄愬煙缁勶細 - ffdhe2048銆乫fdhe3072銆乫fdhe4096銆乫fdhe6144銆乫fdhe8192銆傚鏋滀笉纭畾... |
| CRYPTO_DRBG | tristate | 鏉ヨ嚜 Jitterentropy 搴撶殑 CPU 鎶栧姩 RNG锛堥殢鏈烘暟鐢熸垚鍣級銆備竴绉嶉潪鐗╃悊銆侀潪纭畾鎬х殑锛?鐪?锛塕NG锛堜緥濡傜鍚?NIST SP800-90B 鐨勭喌婧愶級锛屾棬鍦ㄦ彁渚?.. |
| CRYPTO_DRBG_CTR | bool | NIST SP800-90A 涓畾涔夌殑 CTR_DRBG 鍙樹綋銆傚畠浣跨敤 AES 鍔犲瘑绠楁硶涓庤鏁板櫒鍒嗙粍妯″紡銆?|
| CRYPTO_DRBG_HMAC | bool | NIST SP800-90A 涓畾涔夌殑 Hash_DRBG 鍙樹綋銆傚畠浣跨敤 SHA-1銆丼HA-256銆丼HA-384 鎴?SHA-512 鍝堝笇绠楁硶銆?|
| CRYPTO_DRBG_MENU | tristate | DRBG锛堢‘瀹氭€ч殢鏈烘瘮鐗圭敓鎴愬櫒锛夛紙NIST SP800-90A锛夈€傚湪涓嬮潰鐨勫瓙鑿滃崟涓紝蹇呴』閫夋嫨涓€绉嶆垨澶氱 DRBG 绫诲瀷銆備緷璧栦簬 CRYPTO_DRBG_MENU |
| CRYPTO_ECB | tristate | ECB锛堢數瀛愬瘑鐮佹湰锛夋ā寮忥紙NIST SP800-38A锛夈€?|
| CRYPTO_ECC | tristate | 浣跨敤 P-192銆丳-256 涓?P-384 鏇茬嚎锛團IPS 186锛夌殑 ECDH锛堟き鍦嗘洸绾?Diffie-Hellman锛夊瘑閽ヤ氦鎹㈢畻娉曘€?|
| CRYPTO_ECDSA | tristate | ECDSA锛堟き鍦嗘洸绾挎暟瀛楃鍚嶇畻娉曪級锛團IPS 186銆両SO/IEC 14888-3锛夛紝浣跨敤 P-192銆丳-256銆丳-384 涓?P-521 鏇茬嚎銆備粎瀹炵幇绛惧悕楠岃瘉銆?|
| CRYPTO_ECHAINIV | tristate | 鍔犲瘑閾?IV 鐢熸垚鍣ㄣ€傝 IV 鐢熸垚鍣ㄥ熀浜庡搴忓垪鍙蜂笌鐩愬紓鎴栧悗鍐嶅姞瀵嗘潵鐢熸垚 IV銆傝繖鏄?CBC 鐨勯粯璁ょ畻娉曘€?|
| CRYPTO_ECRDSA | tristate | 妞渾鏇茬嚎淇勭綏鏂暟瀛楃鍚嶇畻娉曪紙GOST R 34.10-2012銆丷FC 7091銆両SO/IEC 14888-3锛夈€備縿缃楁柉鍔犲瘑鏍囧噯绠楁硶涔嬩竴锛堢О涓?GOST 绠楁硶锛夈€備粎瀹炵幇绛惧悕楠岃瘉... |
| CRYPTO_ESSIV | tristate | 鍔犲瘑鐩?鎵囧尯 IV 鐢熸垚鍣ㄣ€傝 IV 鐢熸垚鍣ㄥ湪鏌愪簺鍦烘櫙涓嬭 fscrypt 鍜?鎴?dm-crypt 浣跨敤銆傚畠浣跨敤鍧楀姞瀵嗗瘑閽ョ殑鍝堝笇浣滀负鍧楀姞瀵嗛亶鐨勫绉板瘑閽?.. |
| CRYPTO_FCRYPT | tristate | FCrypt algorithm used by RxRPC See https://ota.polyonymo.us/fcrypt-paper.txt |
| CRYPTO_FIPS | bool | 璇ラ€夐」鍚敤 fips 鍚姩閫夐」锛屽鏋滀綘甯屾湜绯荤粺鍦?FIPS 200 璁よ瘉涓嬭繍琛屽垯闇€瑕佸畠銆傞櫎闈炰綘鐭ラ亾瀹冪殑鍚箟锛屽惁鍒欏簲閫夋嫨鍚︺€?|
| CRYPTO_FIPS_CUSTOM_VERSION | bool | 璇ラ€夐」鎻愪緵瑕嗙洊 FIPS 妯″潡鐗堟湰鐨勮兘鍔涖€傞粯璁や娇鐢?KERNELRELEASE 鍊笺€?|
| CRYPTO_FIPS_NAME | string | 璇ラ€夐」璁剧疆鐢?Crypto API 閫氳繃 /proc/sys/crypto/fips_name 鏂囦欢鎶ュ憡鐨?FIPS 妯″潡鍚嶇О銆?|
| CRYPTO_GCM | tristate | GCM锛圙alois/璁℃暟鍣ㄦā寮忥級璁よ瘉鍔犲瘑妯″紡涓?GMAC锛圙CM 娑堟伅璁よ瘉鐮侊級锛圢IST SP800-38D锛夈€傝繖鏄?IPSec ESP锛圶FRM_ESP锛夋墍蹇呴渶鐨勩€?|
| CRYPTO_GENIV | tristate | 搴忓垪鍙?IV 鐢熸垚鍣ㄣ€傝 IV 鐢熸垚鍣ㄩ€氳繃鎶婂簭鍒楀彿涓庣洂寮傛垨鏉ョ敓鎴?IV銆傝绠楁硶涓昏鐢ㄤ簬 CTR銆傝繖鏄?IPsec ESP锛圶FRM_ESP锛夋墍蹇呴渶鐨勩€?|
| CRYPTO_HCTR2 | tristate | HCTR2 淇濋暱鍔犲瘑妯″紡銆備竴绉嶇敤浜庡瓨鍌ㄥ姞瀵嗙殑妯″紡锛屽湪甯︽湁鍔犻€?AES 涓庢棤杩涗綅涔樻硶鎸囦护鐨勫鐞嗗櫒锛堜緥濡傚甫鏈?AES-...鐨?x86 澶勭悊鍣級涓婇珮鏁堛€?|
| CRYPTO_HMAC | tristate | HMAC锛堝甫瀵嗛挜鐨勫搱甯屾秷鎭璇佺爜锛夛紙FIPS 198 涓?RFC2104锛夈€傝繖鏄?IPsec AH锛圶FRM_AH锛変笌 IPsec ESP锛圶FRM_ESP锛夋墍蹇呴渶鐨勩€?|
| CRYPTO_JITTERENTROPY_MEMORY_BLOCKS | int | Enable the userspace interface for hash algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_JITTERENTROPY_MEMSIZE_2 | bool | Jitter RNG 鍏佽鎸囧畾杩囬噰鏍风巼锛圤SR锛夈€侸itter RNG 鐨勮繍琛岄渶瑕佸浐瀹氭暟閲忕殑瀹氭椂娴嬮噺鏉ヤ骇鐢熶竴涓緭鍑洪殢鏈哄尯鍧椼€侽SR... |
| CRYPTO_JITTERENTROPY_TESTINTERFACE | bool | 娴嬭瘯鎺ュ彛鍏佽鐗规潈杩涚▼鎹曡幏 Jitter RNG 鏀堕泦鐨勩€佺敤浜庣粺璁″垎鏋愮殑鍘熷鏈皟鐞嗛珮鍒嗚鲸鐜囨椂闂存埑鍣０銆傜敱浜庤鏁版嵁琚敤浜?.. |
| CRYPTO_KHAZAD | tristate | Khazad 鍔犲瘑绠楁硶銆侹hazad 鏄垵濮?NESSIE 绔炶禌鐨勫喅璧涚畻娉曘€傚畠鏄竴绉嶄负 64 浣嶅鐞嗗櫒浼樺寲銆佸湪 32 浣嶅鐞嗗櫒涓婁篃鏈夎壇濂借〃鐜扮殑绠楁硶銆侹hazad 浣跨敤 128... |
| CRYPTO_KRB5ENC | tristate | 鐢ㄤ簬 Kerberos 5 RFC3961 绠€鍖栭厤缃枃浠剁殑鍝堝笇涓庡姞瀵嗙粍鍚堟敮鎸併€傝繖鏄?sunrpc/NFS 涓?rxrpc/AFS 鎵€浣跨敤鐨?Kerberos 5 椋庢牸鍔犲瘑鎵€蹇呴渶鐨勩€?|
| CRYPTO_LRW | tristate | LRW锛圠iskov Rivest Wagner锛夋ā寮忋€備竴绉嶇敤浜?dm-crypt 鐨勫彲璋冩暣銆佷笉鍙銆佷笉鍙Щ鍔ㄧ殑绐勫潡瀵嗙爜妯″紡銆備笌瀵嗙爜瑙勬牸瀛楃涓?aes-lrw-benbi 涓€璧蜂娇鐢紝瀵嗛挜蹇呴』涓?256銆?20 鎴?38... |
| CRYPTO_LZ4 | tristate | LZ4 compression algorithm See https://github.com/lz4/lz4 for further information. |
| CRYPTO_LZ4HC | tristate | LZ4 high compression mode algorithm See https://github.com/lz4/lz4 for further information. |
| CRYPTO_LZO | tristate | LZO compression algorithm See https://www.oberhumer.com/opensource/lzo/ for further information. |
| CRYPTO_MANAGER2 | def_tristate | 閽堝 cbc(aes) 绛夊姞瀵嗗疄渚嬬殑鐢ㄦ埛绌洪棿閰嶇疆銆?|
| CRYPTO_MD4 | tristate | MD4 娑堟伅鎽樿绠楁硶锛圧FC1320锛夈€?|
| CRYPTO_MD5 | tristate | MD5 娑堟伅鎽樿绠楁硶锛圧FC1321锛夛紝鍖呭惈 HMAC 鏀寔銆?|
| CRYPTO_MLDSA | tristate | ML-DSA锛堝熀浜庢ā鍧楁牸鐨勬暟瀛楃鍚嶇畻娉曪級锛團IPS-204锛夈€備粎瀹炵幇绛惧悕楠岃瘉銆?|
| CRYPTO_NULL | tristate | 杩欎簺鏄?IPsec 浣跨敤鐨?绌?绠楁硶锛屽畠浠笉鍋氫换浣曚簨鎯呫€?|
| CRYPTO_PCBC | tristate | PCBC锛堜紶鎾瘑鐮佸潡閾炬帴锛夋ā寮忋€傝鍒嗙粍瀵嗙爜妯″紡鏄?RxRPC 鎵€蹇呴渶鐨勩€?|
| CRYPTO_PCRYPT | tristate | 璇ラ€夐」灏嗕换鎰忓姞瀵嗙畻娉曡浆鎹负鍦ㄥ唴鏍哥嚎绋嬩腑鎵ц鐨勫苟琛岀畻娉曘€?|
| CRYPTO_RMD160 | tristate | RIPEMD-160 鍝堝笇鍑芥暟锛圛SO/IEC 10118-3锛夈€俁IPEMD-160 鏄竴绉?160 浣嶅姞瀵嗗搱甯屽嚱鏁般€傚畠鏃ㄥ湪浣滀负 128 浣嶅搱甯屽嚱鏁?MD4銆丮D5 鍙婂叾鍓嶈韩鐨勫畨鍏ㄦ浛浠?.. |
| CRYPTO_SEED | tristate | SEED 鍔犲瘑绠楁硶锛圧FC4269銆両SO/IEC 18033-3锛夈€係EED 鏄竴绉?128 浣嶅绉板瘑閽ュ垎缁勫瘑鐮侊紝鐢?KISA锛堥煩鍥戒俊鎭畨鍏ㄩ櫌锛変綔涓哄浗瀹跺姞瀵嗘爣鍑嗙畻娉曞紑鍙?.. |
| CRYPTO_SELFTESTS | bool | 鍚敤鍔犲瘑鑷祴銆傚姞瀵嗚嚜娴嬪湪鍚姩鏃惰繍琛岋紝鎴栧湪绠楁硶绋嶅悗鍔ㄦ€佸姞杞芥椂浜庣畻娉曟敞鍐屾椂杩愯銆傝繖鏈変袱涓富瑕佺敤渚?.. |
| CRYPTO_SELFTESTS_FULL | bool | 鍚敤姣忕绠楁硶鐨勫畬鏁村姞瀵嗚嚜娴嬮泦銆傚畬鏁存祴璇曢泦搴斿惎鐢ㄤ簬寮€鍙戜笌鍙戝竷鍓嶆祴璇曪紝鑰屼笉搴斿湪鐢熶骇鍐呮牳涓惎鐢ㄣ€傛墍鏈夊姞瀵嗕唬鐮?.. |
| CRYPTO_SERPENT | tristate | Serpent cipher algorithm, by Anderson, Biham & Knudsen Keys are allowed to be from 0 to 256 bits in length, in steps of 8 bits. See https://www.cl.cam.ac.uk/~rja14/serpent.html for further informat... |
| CRYPTO_SHA1 | tristate | SHA-1 瀹夊叏鍝堝笇绠楁硶锛團IPS 180銆両SO/IEC 10118-3锛夛紝鍖呭惈 HMAC 鏀寔銆?|
| CRYPTO_SHA256 | tristate | SHA-224 涓?SHA-256 瀹夊叏鍝堝笇绠楁硶锛團IPS 180銆両SO/IEC 10118-3锛夛紝鍖呭惈 HMAC 鏀寔銆傝繖鏄?IPsec AH锛圶FRM_AH锛変笌 IPsec ESP锛圶FRM_ESP锛夋墍蹇呴渶鐨勩€?|
| CRYPTO_SHA3 | tristate | SHA-3 瀹夊叏鍝堝笇绠楁硶锛團IPS 202銆両SO/IEC 10118-3锛夈€?|
| CRYPTO_SHA512 | tristate | SHA-384 涓?SHA-512 瀹夊叏鍝堝笇绠楁硶锛團IPS 180銆両SO/IEC 10118-3锛夛紝鍖呭惈 HMAC 鏀寔銆?|
| CRYPTO_SIMD | tristate | RSA锛圧ivest-Shamir-Adleman锛夊叕閽ョ畻娉曪紙RFC8017锛夈€?|
| CRYPTO_SM3 | tristate | SM3 (ShangMi 3) secure hash function (OSCCA GM/T 0004-2012, ISO/IEC 10118-3) This is part of the Chinese Commercial Cryptography suite. References: http://www.oscca.gov.cn/UpFile/20101222141857786.... |
| CRYPTO_SM4 | tristate | SM4 鍔犲瘑绠楁硶锛圤SCCA GB/T 32907-2016銆両SO/IEC 18033-3:2010/Amd 1:2021锛夈€係M4锛圙BT.32907-2016锛夋槸鐢变腑鍥藉浗瀹跺晢鐢ㄥ瘑鐮佺鐞嗗姙鍏鍙戝竷鐨勫瘑鐮佹爣鍑?.. |
| CRYPTO_STREEBOG | tristate | Streebog 鍝堝笇鍑芥暟锛圙OST R 34.11-2012銆丷FC 6986銆両SO/IEC 10118-3锛夈€傝繖鏄縿缃楁柉鍔犲瘑鏍囧噯绠楁硶涔嬩竴锛堢О涓?GOST 绠楁硶锛夈€傝璁剧疆鍚敤涓ょ鍝堝笇绠楁硶... |
| CRYPTO_TEA | tristate | TEA锛堝井鍨嬪姞瀵嗙畻娉曪級鍔犲瘑绠楁硶銆傚井鍨嬪姞瀵嗙畻娉曟槸涓€绉嶄娇鐢ㄥ杞互淇濊瘉瀹夊叏鎬х殑绠€鍗曞瘑鐮併€傚畠闈炲父蹇笖鍗犵敤鍐呭瓨灏戙€傛墿灞曞井鍨嬪姞瀵?.. |
| CRYPTO_TWOFISH | tristate | Twofish 鍔犲瘑绠楁硶銆俆wofish 鐢?CounterPane Systems 鐨勭爺绌朵汉鍛樹綔涓?AES锛堥珮绾у姞瀵嗘爣鍑嗭級鍊欓€夊瘑鐮佹彁浜ゃ€傚畠鏄竴绉?16 杞垎缁勫瘑鐮侊紝鏀寔...鐨勫瘑閽ラ暱搴︺€?|
| CRYPTO_TWOFISH_COMMON | tristate | Twofish 鍔犲瘑绠楁硶鐢遍€氱敤 C 瀹炵幇涓庢眹缂栧疄鐜板叡浜殑閫氱敤閮ㄥ垎銆?|
| CRYPTO_USER_API_AEAD | tristate | Enable the userspace interface for AEAD cipher algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_ENABLE_OBSOLETE | bool | 鍏佽閫夋嫨宸茶鍐呮牳鍐呴儴浣跨敤閫愭娣樻卑銆佷粎瀵逛粛渚濊禆瀹冧滑鐨勭敤鎴风┖闂村鎴风鏈夌敤鐨勮繃鏃跺姞瀵嗙畻娉曘€?|
| CRYPTO_USER_API_RNG | tristate | Enable the userspace interface for RNG (random number generator) algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_USER_API_RNG_CAVP | bool | 鍦ㄧ敤鎴风┖闂存帴鍙ｄ腑涓?NIST CAVP锛堝姞瀵嗙畻娉曢獙璇佺▼搴忥級娴嬭瘯鍚敤棰濆 API锛?- 閲嶇疆 DRBG 鐔?- 鎻愪緵闄勫姞鏁版嵁銆傝繖鍙簲... |
| CRYPTO_USER_API_SKCIPHER | tristate | Enable the userspace interface for symmetric key cipher algorithms. See Documentation/crypto/userspace-if.rst and https://www.chronox.de/libkcapi/html/index.html |
| CRYPTO_WP512 | tristate | Whirlpool hash function (ISO/IEC 10118-3) 512, 384 and 256-bit hashes. Whirlpool-512 is part of the NESSIE cryptographic primitives. See https://web.archive.org/web/20171129084214/http://www.larc.u... |
| CRYPTO_XCBC | tristate | XCBC-MAC锛堟墿灞曞瘑鐮佸潡閾炬帴娑堟伅璁よ瘉鐮侊級锛圧FC3566锛夈€?|
| CRYPTO_XCTR | tristate | 鐢ㄤ簬 HCTR2 鐨?XCTR锛堝紓鎴栬鏁板櫒锛夋ā寮忋€傝鍒嗙粍瀵嗙爜妯″紡鏄?CTR 妯″紡鐨勫彉浣擄紝浣跨敤寮傛垨涓庡皬绔姞娉曡€岄潪澶х绠楁湳銆俋CTR 妯″紡鐢ㄤ簬瀹炵幇 HCTR2銆?|
| CRYPTO_XTS | tristate | XTS锛堝紓鎴栧姞瀵嗗紓鎴栧苟绐冨彇瀵嗘枃锛夋ā寮忥紙NIST SP800-38E 涓?IEEE 1619锛夈€備笌 aes-xts-plain 涓€璧蜂娇鐢紝瀵嗛挜闀垮害 256銆?84 鎴?512 浣嶃€傝瀹炵幇鐩墠鏃犳硶澶勭悊...鐨勬墖鍖哄ぇ灏忋€?|
| CRYPTO_XXHASH | tristate | xxHash 闈炲姞瀵嗗搱甯岀畻娉曘€傛瀬鍏跺揩閫燂紝閫熷害鎺ヨ繎 RAM 鏋侀檺銆?|
| CRYPTO_ZSTD | tristate | zstd compression algorithm See https://github.com/facebook/zstd for further information. |
| CSD_LOCK_WAIT_DEBUG | bool | 璇ラ€夐」鍦?CPU 瀵?smp_call_function*() IPI 鍖呰鍑芥暟鍝嶅簲缂撴參鏃跺惎鐢ㄨ皟璇曟墦鍗般€傝繖浜涜皟璇曟墦鍗板寘鎷綋鍓嶆鍦ㄦ墽琛岀殑 IPI 澶勭悊鍑芥暟锛堝鏋滄湁锛変互鍙婄浉鍏崇殑... |
| CSD_LOCK_WAIT_DEBUG_DEFAULT | bool | 璇ラ€夐」浣?csdlock_debug= 鍐呮牳鍚姩鍙傛暟榛樿涓?1锛堝熀鏈皟璇曪級鑰岄潪 0锛堟棤璋冭瘯锛夈€?|
| DCACHE_WORD_ACCESS | bool | 鍚敤璇ラ€夐」鍙湪鏂囦欢绯荤粺娉ㄥ唽鏃跺鍏跺弬鏁版弿杩拌繘琛屾牎楠屻€?|
| DEBUG_ATOMIC | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽唴鏍稿皢涓哄師瀛愯闂坊鍔犺繍琛屾椂瀵归綈妫€鏌ャ€傚娌℃湁鏈榻愯闂櫡闃辩殑浣撶郴缁撴瀯寰堟湁鐢ㄣ€傝閫夐」鍙兘鏈夋樉钁楃殑... |
| DEBUG_ATOMIC_LARGEST_ALIGN | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽鍘熷瓙璁块棶鑷劧瀵归綈鐨勬鏌ュ皢琚檺鍒朵负缂栬瘧鍣ㄥ鏍囬噺绫诲瀷鐨勬渶澶у榻愩€?|
| DEBUG_ATOMIC_SLEEP | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽悇绉嶅彲鑳戒紤鐪犵殑渚嬬▼濡傛灉鍦ㄥ師瀛愬尯闂村唴琚皟鐢ㄤ細鍙樺緱闈炲父鍢堟潅锛氬綋鎸佹湁鑷棆閿佹椂銆佸湪 rcu 璇讳晶涓寸晫鍖哄唴銆佸湪鎶㈠崰绂佺敤鏃?.. |
| DEBUG_BUGVERBOSE | bool | 鍦ㄦ閫夋嫨 Y 鍙娇 BUG() 鎭愭厡鍚屾椂杈撳嚭 BUG 璋冪敤鐨勬枃浠跺悕涓庤鍙凤紝浠ュ強 EIP 涓?oops 璺熻釜銆傝繖鏈夊姪浜庤皟璇曪紝浣嗚€楄垂绾?70-100K 鍐呭瓨銆?|
| DEBUG_BUGVERBOSE_DETAILED | bool | 鍦ㄦ閫夋嫨 Y 鍙娇 WARN_ON_ONCE() 闄ゆ枃浠跺悕涓庤鍙峰锛岃繕杈撳嚭璀﹀憡鐨勬潯浠跺瓧绗︿覆銆傝繖鏈夊姪浜庤皟璇曪紝浣嗚€楄垂绾?100K 鍐呭瓨銆傚鏋滀笉纭畾閫夋嫨 N銆?|
| DEBUG_CGROUP_REF | bool | 寮哄埗 cgroup css 寮曠敤璁℃暟鍑芥暟涓嶈鍐呰仈锛屼互渚垮畠浠彲浠ヨ kprobe 鐢ㄤ簬璋冭瘯銆?|
| DEBUG_CLOSURES | bool | 灏嗘墍鏈夋椿鍔ㄧ殑 closure 淇濈暀鍦ㄤ竴涓摼琛ㄤ腑锛屽苟鎻愪緵 debugfs 鎺ュ彛鏉ュ垪鍑哄畠浠紝浠庤€屽彲浠ユ煡鐪嬪崱浣忕殑寮傛鎿嶄綔銆?|
| DEBUG_FORCE_FUNCTION_ALIGN_64B | bool | 瀛樺湪杩欐牱鐨勬儏鍐碉細涓€涓煙鐨勬彁浜や細鏀瑰彉鍏朵粬鍩熺殑鍑芥暟鍦板潃瀵归綈锛屽苟瀵艰嚧绁炲鐨勬€ц兘绐佸彉锛堝洖褰掓垨鎻愬崌锛夈€傚惎鐢ㄨ閫夐」灏嗘湁鍔╀簬... |
| DEBUG_FORCE_WEAK_PER_CPU | bool | s390 涓?alpha 瑕佹眰妯″潡涓殑 percpu 鍙橀噺琚畾涔変负寮辩鍙凤紝浠ヨ閬垮鍧€鑼冨洿闂锛岃繖瀵?percpu 鍙橀噺瀹氫箟鏂藉姞浜嗕互涓嬩袱鏉￠檺鍒躲€?. percpu 绗﹀彿... |
| DEBUG_FS | bool | debugfs 鏄唴鏍稿紑鍙戣€呯敤鏉ユ斁缃皟璇曟枃浠剁殑铏氭嫙鏂囦欢绯荤粺銆傚惎鐢ㄨ閫夐」浠ヤ究鑳藉璇诲啓杩欎簺鏂囦欢銆傛湁鍏?debugfs 鐨勮缁嗘枃妗?.. |
| DEBUG_FS_ALLOW_ALL | bool | 鏃犻檺鍒躲€侫PI 涓庢枃浠剁郴缁熸敞鍐屽潎寮€鍚€傝繖鏄甯哥殑榛樿鎿嶄綔銆?|
| DEBUG_FS_ALLOW_NONE | bool | 璁块棶鍏抽棴銆傚鎴风鍦ㄥ皾璇曞湪 debugfs 鏍戜腑鍒涘缓鑺傜偣鏃跺緱鍒?-PERM锛屼笖 debugfs 鏈敞鍐屼负鏂囦欢绯荤粺銆傚鎴风闅忓悗鍙€€閬挎垨鍦ㄦ病鏈?debugfs 璁块棶鐨勬儏鍐典笅缁х画銆?|
| DEBUG_HIGHMEM | bool | 璇ラ€夐」鍚敤閽堝楂樺唴瀛樼郴缁熺殑棰濆閿欒妫€鏌ャ€傚湪鐢熶骇绯荤粺涓婄鐢ㄣ€?|
| DEBUG_INFO | bool | 鍦ㄤ笅闈㈢殑"璋冭瘯淇℃伅"閫夐」涓€夋嫨浜?鏃?浠ュ鐨勫唴鏍歌皟璇曚俊鎭€夐」锛岃〃鏄庡皢涓烘瀯寤虹洰鏍囩敓鎴愯皟璇曚俊鎭€? Clang 鐢熸垚 .ule... |
| DEBUG_INFO_BTF | bool | 浠?DWARF 璋冭瘯淇℃伅鐢熸垚鍘婚噸鍚庣殑 BTF 绫诲瀷淇℃伅銆傚紑鍚畠闇€瑕?pahole v1.22 鎴栨洿楂樼増鏈紝瀹冨皢鎶?DWARF 绫诲瀷淇℃伅杞崲涓虹瓑浠风殑鍘婚噸 BTF 绫诲瀷淇℃伅銆?|
| DEBUG_INFO_BTF_MODULES | bool | 涓哄唴鏍告ā鍧楃敓鎴愮揣鍑戠殑鎷嗗垎 BTF 绫诲瀷淇℃伅銆?|
| DEBUG_INFO_COMPRESSED_NONE | bool | 涓嶅帇缂╄皟璇曚俊鎭妭銆?|
| DEBUG_INFO_COMPRESSED_ZLIB | bool | 浣跨敤 zlib 鍘嬬缉璋冭瘯淇℃伅銆傞€氳繃 debian/rules 浣跨敤 dpkg-deb 鐨勭敤鎴峰彲鑳戒細鍙戠幇锛岀敱浜庤皟璇曚俊鎭鍘嬬缉锛屽叾璋冭瘯 .deb 鍖呯殑浣撶Н浼氬澶?.. |
| DEBUG_INFO_COMPRESSED_ZSTD | bool | 浣跨敤 zstd 鍘嬬缉璋冭瘯淇℃伅銆傝繖鍙兘鎻愪緵姣?zlib 鏇村ソ鐨勫帇缂╃巼锛岃€楁椂澶ц嚧鐩稿綋锛屼絾闇€瑕佽緝鏂扮殑宸ュ叿閾炬敮鎸併€傞渶瑕?GCC 13.0+ 鎴?Clang 16.0+锛屼絾... |
| DEBUG_INFO_DWARF4 | bool | 鐢熸垚 DWARF v4 璋冭瘯淇℃伅銆傝繖闇€瑕?gcc 4.5+銆佽嫢浣跨敤涓嶅甫 clang 闆嗘垚姹囩紪鍣ㄧ殑 clang 鍒欓渶瑕?binutils 2.35.2锛屼互鍙?gdb 7.0+銆傚鏋滀綘鏈夊皻鏈噯澶囧ソ...鐨?DWARF 璋冭瘯淇℃伅浣跨敤鑰呫€?|
| DEBUG_INFO_DWARF_TOOLCHAIN_DEFAULT | bool | 宸ュ叿閾句骇鐢熺殑 DWARF 璋冭瘯淇℃伅鐨勯殣寮忛粯璁ょ増鏈細闅忔椂闂村彉鍖栥€傝繖鍙兘鐮村潖灏氭湭鍗囩骇浠ユ敮鎸佹洿鏂扮増鏈殑璋冭瘯淇℃伅浣跨敤鑰咃紝骞堕樆姝?.. |
| DEBUG_INFO_NONE | bool | 涓嶄互鍐呮牳璋冭瘯淇℃伅鏋勫缓锛岃繖灏嗕骇鐢熸洿蹇€佹洿灏忕殑鏋勫缓銆?|
| DEBUG_INFO_SPLIT | bool | 灏嗚皟璇曚俊鎭敓鎴愬埌鐙珛鐨?.dwo 鏂囦欢涓€傝繖鏄捐憲鍑忓皬浜嗗甫 DEBUG_INFO 鏋勫缓鐨勬瀯寤虹洰褰曚綋绉紝鍥犱负瀹冨彧鍦ㄧ鐩樹笂鐨?.dwo 鏂囦欢涓瓨鍌ㄤ竴娆′俊鎭紝鑰岄潪... |
| DEBUG_IRQFLAGS | bool | 鍚敤瀵瑰彲鑳戒笉瀹夊叏鐨勫紑/鍏充腑鏂殑妫€鏌ワ紝渚嬪鍦ㄤ腑鏂凡鍚敤鏃惰皟鐢?raw_local_irq_restore()銆?|
| DEBUG_KERNEL | bool | 濡傛灉浣犲湪寮€鍙戦┍鍔ㄦ垨璇曞浘璋冭瘯骞跺畾浣嶅唴鏍搁棶棰橈紝鍦ㄦ閫夋嫨 Y銆?|
| DEBUG_KMAP_LOCAL | bool | 璇ラ€夐」涓?kmap_local 鍩虹璁炬柦鍚敤棰濆鐨勯敊璇鏌ャ€傜敓浜х幆澧冭绂佺敤銆?|
| DEBUG_KOBJECT | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屼竴浜涢澶栫殑 kobject 璋冭瘯娑堟伅灏嗚鍙戦€佸埌 syslog銆?|
| DEBUG_KOBJECT_RELEASE | bool | kobject 鏄紩鐢ㄨ鏁扮殑瀵硅薄銆傝繖鎰忓懗鐫€瀹冧滑鐨勬渶鍚庝竴娆″紩鐢ㄨ鏁伴噴鏀炬槸涓嶅彲棰勬祴鐨勶紝骞朵笖 kobject 鍙兘鍦ㄩ┍鍔ㄥ喅瀹氫涪寮冨叾鍒濆... |
| DEBUG_LOCKDEP | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛岄攣渚濊禆寮曟搸灏嗗仛棰濆鐨勮繍琛屾椂妫€鏌ヤ互鑷垜璋冭瘯锛屼唬浠锋槸鏇村鐨勮繍琛屾椂寮€閿€銆?|
| DEBUG_LOCKING_API_SELFTESTS | bool | 濡傛灉浣犲笇鏈涘唴鏍稿湪鍚姩鏈熼棿杩愯涓€涓畝鐭殑鑷祴锛屽湪姝ら€夋嫨 Y銆傝鑷祴妫€鏌ュ父瑙佺殑鍚勭被閿?Bug 鏄惁琚皟璇曟満鍒舵娴嬪埌锛堝鏋滀綘绂佺敤閿?..锛夈€?|
| DEBUG_LOCK_ALLOC | bool | 璇ョ壒鎬у皢閫氳繃浠讳綍鍐呭瓨閲婃斁渚嬬▼锛坘free()銆乲mem_cache_free()銆乫ree_pages()...锛夋鏌ユ槸鍚︽湁浠讳綍琚寔鏈夌殑閿侊紙鑷棆閿併€乺wlock銆乵utex 鎴?rwsem锛夎鍐呮牳閿欒鍦伴噴鏀俱€?|
| DEBUG_MAPLE_TREE | bool | 鍚敤 maple tree 璋冭瘯淇℃伅涓庨澶栨牎楠屻€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_MEMORY_INIT | bool | 鍚敤璇ラ€夐」浠ュ湪鍐呭瓨鍒濆鍖栨湡闂磋繘琛岄澶栨鏌ャ€傚仴鍏ㄦ€ф鏌ラ獙璇?VM 鐨勫悇涓柟闈紝渚嬪鍐呭瓨妯″瀷浠ュ強浣撶郴缁撴瀯鎻愪緵鐨勫叾浠栦俊鎭€傝缁嗕俊鎭?.. |
| DEBUG_MISC | bool | 濡傛灉浣犻渶瑕佸惎鐢ㄦ湰搴斿綊鍏ユ洿鍏蜂綋鐨勮皟璇曢€夐」浣嗗苟闈炲姝ょ殑鏉傞」璋冭瘯浠ｇ爜锛屽湪姝ら€夋嫨 Y銆?|
| DEBUG_MUTEXES | bool | 璇ョ壒鎬у厑璁告娴嬪苟鎶ュ憡 mutex 璇箟杩濊銆?|
| DEBUG_NOMMU_REGIONS | bool | 璇ョ壒鎬т娇鍖垮悕涓庣鏈夋槧灏勫尯鍩熺殑鍏ㄥ眬鏍戣瀹氭湡妫€鏌ユ槸鍚﹀瓨鍦ㄦ棤鏁堟嫇鎵戙€?|
| DEBUG_NOTIFIERS | bool | 鍚敤璇ラ€夐」浠ュ紑鍚閫氱煡閾撅紙notifier call chain锛夌殑鍋ュ叏鎬ф鏌ャ€傝繖瀵瑰唴鏍稿紑鍙戣€呯‘淇濇ā鍧楁纭湴浠庨€氱煡閾炬敞閿€鏈€涓烘湁鐢ㄣ€傝繖鏄竴涓?.. |
| DEBUG_OBJECTS | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽唴鏍镐腑灏嗘彃鍏ラ澶栦唬鐮佹潵璺熻釜鍚勭被瀵硅薄鐨勭敓鍛藉懆鏈燂紝骞舵牎楠屽杩欎簺瀵硅薄鐨勬搷浣溿€?|
| DEBUG_OBJECTS_ENABLE_DEFAULT | int | 璋冭瘯瀵硅薄鐨勫惎鍔ㄥ弬鏁伴粯璁ゅ€笺€?|
| DEBUG_OBJECTS_FREE | bool | 璇ラ€夐」鍚敤妫€鏌ワ細k/v 閲婃斁鎿嶄綔鏄惁閲婃斁浜嗕竴涓寘鍚皻鏈姝ｇ‘鍋滅敤鐨勫璞＄殑鍖哄煙銆傝繖鍙兘浣?kmalloc/kfree 瀵嗛泦鐨勫伐浣滆礋杞芥槑鏄惧彉鎱€?|
| DEBUG_OBJECTS_PERCPU_COUNTER | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛宲ercpu 璁℃暟鍣ㄤ緥绋嬩腑灏嗘彃鍏ラ澶栦唬鐮侊紝浠ヨ窡韪?percpu 璁℃暟鍣ㄥ璞＄殑鐢熷懡鍛ㄦ湡骞舵牎楠?percpu 璁℃暟鍣ㄦ搷浣溿€?|
| DEBUG_OBJECTS_RCU_HEAD | bool | 鍚敤璇ラ€夐」浠ュ紑鍚 RCU 閾捐〃澶达紙call_rcu() 鐢ㄦ硶锛夌殑璋冭瘯銆?|
| DEBUG_OBJECTS_SELFTEST | bool | 璇ラ€夐」鍚敤瀵硅薄璋冭瘯浠ｇ爜鐨勮嚜娴嬨€?|
| DEBUG_OBJECTS_TIMERS | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽畾鏃跺櫒渚嬬▼涓皢鎻掑叆棰濆浠ｇ爜锛屼互璺熻釜瀹氭椂鍣ㄥ璞＄殑鐢熷懡鍛ㄦ湡骞舵牎楠屽畾鏃跺櫒鎿嶄綔銆?|
| DEBUG_OBJECTS_WORK | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽伐浣滈槦鍒椾緥绋嬩腑灏嗘彃鍏ラ澶栦唬鐮侊紝浠ヨ窡韪伐浣滃璞＄殑鐢熷懡鍛ㄦ湡骞舵牎楠屽伐浣滄搷浣溿€?|
| DEBUG_PERF_USE_VMALLOC | bool | 浣跨敤 vmalloc 鍐呭瓨浣滀负 perf mmap() 缂撳啿鍖虹殑鍚庡銆備富瑕佺敤浜庡湪涓嶉渶瑕佸畠鐨勫钩鍙颁笂璋冭瘯 vmalloc 浠ｇ爜銆傚鏋滀笉纭畾閫夋嫨 N銆?|
| DEBUG_PER_CPU_MAPS | bool | 閫夋嫨 Y 浠ラ獙璇佹鍦ㄨ闂殑 per_cpu 鏄犲皠宸茶寤虹珛銆傝繖浼氬悜鍐呮牳鍐呭瓨娣诲姞鐩稿綋澶氱殑浠ｇ爜骞堕檷浣庢€ц兘銆傚鏋滀笉纭畾閫夋嫨 N銆?|
| DEBUG_PLIST | bool | 鍚敤璇ラ€夐」浠ュ紑鍚湪鎸変紭鍏堢骇鎺掑簭鐨勯摼琛紙plist锛夐亶鍘嗕緥绋嬩腑鐨勬墿灞曟鏌ャ€傚畠浼氬湪姣忔鎿嶄綔鏃跺娆℃鏌ユ暣涓摼琛ㄣ€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_PREEMPT | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽唴鏍稿皢浣跨敤甯哥敤 smp_processor_id() 鍑芥暟鐨勮皟璇曞彉浣擄紝骞跺湪鍐呮牳浠ｇ爜浠ヤ笉瀹夊叏鎶㈠崰鐨勬柟寮忎娇鐢ㄥ畠鏃舵墦鍗拌鍛娿€傛澶栵紝鍐呮牳... |
| DEBUG_RSEQ | bool | 涓?rseq 绯荤粺璋冪敤鍚敤棰濆鐨勮皟璇曟鏌ャ€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_RT_MUTEXES | bool | 璇ラ€夐」鍏佽鑷姩妫€娴嬪苟鎶ュ憡 rt mutex 璇箟杩濊浠ュ強 rt mutex 鐩稿叧鐨勬閿侊紙lockup锛夈€?|
| DEBUG_RWSEMS | bool | 璇ヨ皟璇曠壒鎬у厑璁告娴嬪苟鎶ュ憡涓嶅尮閰嶇殑璇诲啓淇″彿閲忓姞閿佷笌瑙ｉ攣銆?|
| DEBUG_SECTION_MISMATCH | bool | 鑺傚尯涓嶅尮閰嶅垎鏋愭鏌ユ槸鍚﹀瓨鍦ㄤ粠涓€涓妭鍖哄埌鍙︿竴涓妭鍖虹殑闈炴硶寮曠敤銆傚湪閾炬帴鏃舵垨杩愯鏃讹紝鏌愪簺鑺傚尯浼氳涓㈠純锛涗换浣曞杩欎簺鑺傚尯涓師鏈変唬鐮?鏁版嵁鐨勪娇鐢?.. |
| DEBUG_SG | bool | 鍚敤璇ラ€夐」浠ュ紑鍚鍒嗘暎-鑱氶泦锛坰catter-gather锛夎〃鐨勬鏌ャ€傝繖鏈夊姪浜庡彂鐜版湭姝ｇ‘鍒濆鍖栧叾 sg 琛ㄧ殑椹卞姩闂銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_SHIRQ | bool | 鍚敤璇ラ€夐」浠ュ湪鍏变韩涓柇澶勭悊绋嬪簭娉ㄩ攢鍓嶇敓鎴愪竴涓吉涓柇锛堟敞鍐屾椂鐢熸垚褰撳墠琚鐢級銆傞┍鍔ㄩ渶瑕佹纭鐞嗗畠銆傚鏋滀笉纭畾... |
| DEBUG_SPINLOCK | bool | 鍦ㄦ閫夋嫨 Y 骞舵瀯寤?SMP 浠ユ崟鑾风己澶辩殑鑷棆閿佸垵濮嬪寲浠ュ強甯哥姱鐨勫叾浠栬嚜鏃嬮攣閿欒銆傛渶濂戒笌 NMI 鐪嬮棬鐙楅厤鍚堜娇鐢紝浠ヤ究鑷棆閿?.. |
| DEBUG_STACK_USAGE | bool | 鍚敤鍦?sysrq-T 涓?sysrq-P 璋冭瘯杈撳嚭涓樉绀烘瘡涓换鍔℃浘缁忓彲鐢ㄧ殑鏈€灏忕┖闂叉爤閲忋€傚綋杩涚▼閫€鍑烘椂锛屽鏋滆杩涚▼...涔熶細鍚?dmesg 鍙戦€佷竴鏉℃秷鎭€?|
| DEBUG_VFS | bool | 鍚敤璇ラ€夐」浠ュ紑鍚?VFS 灞備腑鍙兘褰卞搷鎬ц兘鐨勬墿灞曟鏌ャ€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_VM_IRQSOFF | def_bool | 鍚敤璇ラ€夐」浠ュ紑鍚櫄鎷熷唴瀛樼郴缁熶腑鍙兘褰卞搷鎬ц兘鐨勬墿灞曟鏌ャ€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_VM_MAPLE_TREE | bool | 鍚敤 VM maple tree 璋冭瘯淇℃伅涓庨澶栨牎楠屻€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_VM_PGFLAGS | bool | 鍦ㄩ〉鏍囧織锛坧age flags锛夋搷浣滀笂鍚敤棰濆鏍￠獙銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_VM_PGTABLE | bool | 璇ラ€夐」鎻愪緵涓€绉嶈皟璇曟柟娉曪紝鍙敤浜庡湪鍚勭骞冲彴涓婃祴璇曚綋绯荤粨鏋勭殑椤佃〃杈呭姪鍑芥暟鏄惁绗﹀悎棰勬湡鐨勯€氱敤 MM 璇箟銆傝繖灏?.. |
| DEBUG_VM_RB | bool | 鍚敤 VM 绾㈤粦鏍戣皟璇曚俊鎭笌棰濆鏍￠獙銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_VM_SHOOT_LAZIES | bool | 鍚敤棰濆鐨?IPI锛屼互纭繚鍦ㄤ娇鐢ㄥ畬 mm 涔嬪墠绉婚櫎鎯版€?tlb mm 寮曠敤銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| DEBUG_WQ_FORCE_RR_CPU | bool | 宸ヤ綔闃熷垪杩囧幓闅愬紡淇濊瘉锛氭湭鎸囧畾鍏蜂綋 CPU 鍏ラ槦鐨勫伐浣滈」浼氳鏀惧埌鏈湴 CPU 涓娿€傝淇濊瘉宸蹭笉鍐嶆垚绔嬶紝铏界劧鏈湴 CPU 浠嶈浼樺厛... |
| DEBUG_WW_MUTEX_SLOWPATH | bool | 璇ョ壒鎬ч€氳繃娉ㄥ叆棰濆鐨?-EDEADLK 浼ゅ/閫€閬挎儏褰紝涓?w/w mutex 浣跨敤鑰呭惎鐢ㄦ參閫熻矾寰勬祴璇曘€傞厤鍚堢敱 (CONFIG_PROVE_LOCKING) 鍚敤鐨勫畬鏁?mutex 妫€鏌ワ紝杩欏皢娴嬭瘯... |
| DEFAULT_HOSTNAME | string | 璇ラ€夐」鍐冲畾鍦ㄧ敤鎴风┖闂磋皟鐢?sethostname(2) 涔嬪墠绯荤粺鐨勯粯璁や富鏈哄悕銆傚唴鏍镐紶缁熶笂浣跨敤 "(none)"锛屼絾浣犲彲鑳藉笇鏈涗娇鐢ㄤ笉鍚岀殑榛樿鍊硷紝浠ユ瀯鎴愪竴涓渶灏?.. |
| DEFAULT_HUNG_TASK_TIMEOUT | int | 璇ラ€夐」鎺у埗鍦ㄥ垽瀹氫换鍔″彉涓烘棤鍝嶅簲骞跺簲琚涓烘寕璧锋椂鎵€鐢ㄧ殑榛樿瓒呮椂锛堢锛夈€傚畠鍙互鍦ㄨ繍琛屾椂閫氳繃 kernel.hung_task_t... 璋冩暣銆?|
| DEFAULT_INIT | string | 璇ラ€夐」鍐冲畾褰撳唴鏍稿懡浠よ鏈紶鍏?init= 閫夐」鏃剁郴缁熺殑榛樿 init銆傚鏋滆姹傜殑璺緞涓嶅瓨鍦紝鎴戜滑浠嶅皢缁х画灏濊瘯杩涗竴姝ョ殑... |
| DEFAULT_MMAP_MIN_ADDR | int | 杩欐槸搴斿綋鍏嶅彈鐢ㄦ埛绌洪棿鍒嗛厤褰卞搷鐨勪綆铏氭嫙鍐呭瓨閮ㄥ垎銆傞樆姝㈢敤鎴峰啓鍏ヤ綆鍦板潃椤垫湁鍔╀簬鍑忓皯鍐呮牳绌烘寚閽?Bug 鐨勫奖鍝嶃€傚浜?.. |
| DEFAULT_SECURITY_SELINUX | bool | 涓€涓互閫楀彿鍒嗛殧鐨?LSM 鍒楄〃锛屾寜鍒濆鍖栭『搴忔帓鍒椼€備换浣曟湭鍒楀叆姝ゅ垪琛ㄧ殑 LSM锛岄櫎椤哄簭涓?LSM_ORDER_FIRST 涓?LSM_ORDER_LAST 鑰呭锛堣嫢鍦ㄥ唴鏍镐腑閫変腑鐨勮瘽鎬绘槸鍚敤锛夛紝... |
| DEFERRED_STRUCT_PAGE_INIT | bool | 閫氬父鎵€鏈?struct page 閮藉湪鏃╂湡鍚姩鏈熼棿鐢卞崟绾跨▼鍒濆鍖栥€傚湪闈炲父澶х殑鏈哄櫒涓婅繖鍙兘鑰楄垂鐩稿綋澶氱殑鏃堕棿銆傝嫢璁剧疆璇ラ€夐」锛屽ぇ鍨嬫満鍣ㄥ皢甯?.. |
| DETECT_HUNG_TASK | bool | 鍦ㄦ閫夋嫨 Y 浠ヨ鍐呮牳妫€娴?鎸傝捣浠诲姟"锛屽嵆瀵艰嚧浠诲姟鏃犻檺鏈熷崱鍦ㄤ笉鍙腑鏂殑 "D" 鐘舵€佺殑 Bug銆傚綋妫€娴嬪埌鎸傝捣浠诲姟鏃讹紝鍐呮牳灏嗘墦鍗?.. |
| DETECT_HUNG_TASK_BLOCKER | bool | 鍦ㄦ閫夋嫨 Y 浠ユ樉绀鸿幏鍙栦簡"鎸傝捣浠诲姟"鎵€绛夊緟鐨?mutex 閿佺殑闃诲浠诲姟鏍堣窡韪€傝繖浼氬鍔犲皯閲忓紑閿€锛屼絾鑻ュ叾鏉ヨ嚜...浼氭樉绀哄彲鐤戜换鍔′笌璋冪敤鏍堛€?|
| DIMLIB | tristate | 鍔ㄦ€佷腑鏂皟鑺傚簱銆傚疄鐜颁竴绉嶆牴鎹繍琛屾椂鎬ц兘鍔ㄦ€佹敼鍙?CQ 璋冭妭鍊肩殑绠楁硶銆? # libfdt 鏂囦欢锛屼粎鍦ㄩ渶瑕佹椂閫変腑銆? |
| DST_CACHE | bool | NET_SOCK_MSG 涓烘櫘閫氬鎺ュ瓧锛堜緥濡?TCP锛夋垨 ULP锛堜笂灞傛ā鍧楋紝渚嬪 TLS锛夋彁渚涗竴涓€熷姪 BPF 绋嬪簭澶勭悊 L7 搴旂敤鏁版嵁鐨勬鏋躲€?|
| DYNAMIC_DEBUG | bool | 灏嗚皟璇曠骇娑堟伅缂栬瘧杩涘唴鏍革紝鍚﹀垯鍦ㄨ繍琛屾椂灏嗕笉鍙敤銆傞殢鍚庤繖浜涙秷鎭彲鏍规嵁涓嶅悓鑼冨洿灞傜骇鍚敤/绂佺敤鈥斺€旀瘡涓簮鏂囦欢銆佸嚱鏁?.. |
| DYNAMIC_DEBUG_CORE | bool | 鍚敤鍔ㄦ€佽皟璇曠殑鏍稿績鍔熻兘鏀寔銆傚綋浣犲笇鏈涘皢鍔ㄦ€佽皟璇曢€氳繃涓烘瘡涓唴鏍告ā鍧楀畾涔夌殑 DYNAMIC_DEBUG_MODULE 缁戝畾鍒板畠浠椂寰堟湁鐢紝灏ゅ叾鏄?.. |
| ELFCORE | bool | 璇ラ€夐」鍚敤 kernel/elfcore.o銆?|
| ELF_CORE | bool | 鍚敤瀵圭敓鎴愭牳蹇冭浆鍌ㄧ殑鏀寔銆傜鐢ㄥ彲鑺傜渷绾?4k銆?|
| ETHTOOL_NETLINK | bool | 涓€涓熀浜?generic netlink 鐨勩€佺敤浜?ethtool 鐨勬浛浠ｇ敤鎴风┖闂存帴鍙ｃ€傚畠鎻愪緵鏇村ソ鐨勫彲鎵╁睍鎬т互鍙婁竴浜涙柊鐗规€э紝渚嬪閫氱煡娑堟伅銆?|
| EVENTFD | bool | 鍚敤 eventfd() 绯荤粺璋冪敤锛屽厑璁告帴鏀跺唴鏍搁€氱煡锛堝嵆 KAIO锛夋垨鐢ㄦ埛绌洪棿閫氱煡銆傚鏋滀笉纭畾锛岄€夋嫨 Y銆?|
| EXEC_KUNIT_TEST | bool | 璇ラ€夐」鏋勫缓 exec 鐨?KUnit 娴嬭瘯锛屾祴璇?exec 鍐呴儴鍚勬柟闈㈣竟鐣屾潯浠躲€?|
| EXT_GROUP_SCHED | bool | 璇ョ壒鎬т娇璋冨害鍣ㄨ兘澶熷熀浜庡綋鍓嶅湪璇?CPU 涓婅皟搴︾殑 RUNNABLE 浠诲姟璺熻釜姣忎釜 CPU 鐨勯挸鍒跺埄鐢ㄧ巼銆傚惎鐢ㄨ閫夐」鍚庯紝鐢ㄦ埛鍙互鎸囧畾涓€涓渶灏忓€间笌... |
| FAILOVER | tristate | failover 妯″潡涓哄崐铏氭嫙鍖栭┍鍔ㄦ彁渚涗竴涓€氱敤鎺ュ彛锛屼互鍚?failover 瀹炰緥娉ㄥ唽涓€涓?netdev 涓庝竴缁勬搷浣溿€傝繖浜涙搷浣滅敤浣滀簨浠跺鐞嗙▼搴忥紝鍦?..鏃惰璋冪敤銆?|
| FAILSLAB | bool | 涓?kmalloc 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FAIL_FUNCTION | bool | 鎻愪緵鍩轰簬鍑芥暟鐨勬晠闅滄敞鍏ヨ兘鍔涖€傝繖灏嗗厑璁镐綘鐢ㄧ粰瀹氳繑鍥炲€肩殑杩斿洖璇彞瑕嗙洊鐗瑰畾鍑芥暟銆傜粨鏋滐紝鍑芥暟璋冪敤鑰呭皢鐪嬪埌涓€涓敊璇€?.. |
| FAIL_FUTEX | bool | 涓?futex 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FAIL_IO_TIMEOUT | bool | 鍦ㄧ IO 澶勭悊涓婃彁渚涙晠闅滄敞鍏ヨ兘鍔涖€傝繖灏嗕娇鍧楀眰鎸夐厤缃?閬楀繕"涓€涓腑鏂紝浠庤€屾紨缁冮敊璇鐞嗐€備粎瀵逛娇鐢ㄩ€氱敤...鐨勯┍鍔ㄦ湁鏁堛€?|
| FAIL_MAKE_REQUEST | bool | 涓虹鐩?IO 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FAIL_MMC_REQUEST | bool | 涓?MMC IO 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆傝繖灏嗕娇 mmc 鏍稿績杩斿洖鏁版嵁閿欒銆傝繖瀵逛簬娴嬭瘯 mmc 鍧楄澶囦腑鐨勯敊璇鐞嗭紝浠ュ強娴嬭瘯 mmc 涓绘満椹卞姩...寰堟湁鐢ㄣ€?|
| FAIL_PAGE_ALLOC | bool | 涓?alloc_pages() 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FAIL_SKB_REALLOC | bool | 鎻愪緵鏁呴殰娉ㄥ叆鑳藉姏锛屽己鍒?skb 琚噸鏂板垎閰嶏紝浠ユ崟鑾锋寚鍚?skb 鐨勫彲鑳芥棤鏁堟寚閽堛€傛洿澶氫俊鎭鍙傞槄 Documentation/fault-injection/fault-injection.rst |
| FAIL_SUNRPC | bool | 涓?SunRPC 鍙婂叾浣跨敤鑰呮彁渚涙晠闅滄敞鍏ヨ兘鍔涖€?|
| FAULT_INJECTION | bool | 鎻愪緵鏁呴殰娉ㄥ叆妗嗘灦銆傛洿澶氱粏鑺傝鍙傞槄 Documentation/fault-injection/銆?|
| FAULT_INJECTION_CONFIGFS | bool | 璇ラ€夐」鍏佽鍩轰簬 configfs 鐨勯┍鍔ㄩ€氳繃 configfs 鍔ㄦ€侀厤缃晠闅滄敞鍏ャ€傞┍鍔ㄧ壒瀹氱殑姣忎釜鏁呴殰娉ㄥ叆鍙傛暟閮藉彲浠ヤ綔涓?configfs 灞炴€у湪...涓彲瑙併€?|
| FAULT_INJECTION_DEBUG_FS | bool | 閫氳繃 debugfs 鍚敤鏁呴殰娉ㄥ叆鑳藉姏鐨勯厤缃€?|
| FAULT_INJECTION_STACKTRACE_FILTER | bool | 涓烘晠闅滄敞鍏ヨ兘鍔涙彁渚涙爤璺熻釜杩囨护鍣ㄣ€?|
| FAULT_INJECTION_USERCOPY | bool | 涓?usercopy 鍑芥暟锛坈opy_from_user()銆乬et_user()...锛夋彁渚涙敞鍏ュけ璐ョ殑鏁呴殰娉ㄥ叆鑳藉姏銆?|
| FFS_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 ffs 绯诲垪浣嶆搷浣滃嚱鏁帮紙鍖呮嫭 ffs()銆乢_ffs()銆乫ls()銆乢_fls()銆乫ls64() 涓?__ffs64()锛夌殑 KUnit 娴嬭瘯銆傝繖浜涙祴璇曢獙璇佹暟瀛︽纭€с€佽竟鐣屾儏鍐靛鐞?.. |
| FHANDLE | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛岀敤鎴风骇绋嬪簭灏嗚兘澶熷皢鏂囦欢鍚嶆槧灏勪负鍙ユ焺锛岄殢鍚庡皢璇ュ彞鏌勭敤浜庝笉鍚岀殑鏂囦欢绯荤粺鎿嶄綔銆傝繖鍦ㄥ疄鐜扮敤鎴风┖闂存枃浠舵湇鍔?..鏃跺緢鏈夌敤銆?|
| FIB_RULES | bool | 璇ョ壒鎬ф彁渚涗竴涓敮鎸佽交閲忕骇闅ч亾锛堝 mpls锛夌殑鍩虹璁炬柦銆傝交閲忕骇闅ч亾绔偣娌℃湁鍏宠仈鐨勭綉缁滆澶囥€傞毀閬撳皝瑁呭弬鏁板瓨鍌ㄤ簬...銆?|
| FILE_LOCKING | bool | 璇ラ€夐」鍚敤鏍囧噯鏂囦欢閿佹敮鎸侊紝杩欐槸 NFS 绛夋枃浠剁郴缁熶互鍙?flock() 绯荤粺璋冪敤鎵€蹇呴渶鐨勩€傜鐢ㄨ閫夐」鍙妭鐪佺害 11k銆?|
| FIND_BIT_BENCHMARK | tristate | 璇ラ€夐」鏋勫缓 "test_find_bit" 妯″潡锛岀敤浜庢祴閲?find_*_bit() 鍑芥暟鐨勬€ц兘銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| FIND_BIT_BENCHMARK_RUST | tristate | 璇ラ€夐」鏋勫缓 "find_bit_benchmark_rust" 妯″潡銆傚畠鏄竴涓井鍩哄噯娴嬭瘯锛屾祴閲忎笌 C 涓?find_*_bit() 鎿嶄綔瀵瑰簲鐨?Rust 鍑芥暟鎬ц兘銆傚畠閬靛惊 FIND_BI... |
| FIND_NORMAL_PAGE | def_bool | 璇ヤ綋绯荤粨鏋勪娇鐢ㄦ儼鎬?MMU 妯″紡銆傝繖鍏佽灏嗕笌 MMU 鐩稿叧鐨勪綋绯荤粨鏋勭姸鎬佸彉鏇存帹杩熷埌閫€鍑鸿妯″紡鏃舵墠杩涜銆傝鎯呰鍙傞槄 <linux/pgtable.h>銆?|
| FLATMEM_MANUAL | bool | 璇ラ€夐」鏈€閫傚悎鍏锋湁鎵佸钩鍦板潃绌洪棿鐨勯潪 NUMA 绯荤粺銆侳LATMEM 鍦ㄦ€ц兘涓庤祫婧愭秷鑰楁柟闈㈡槸鏈€楂樻晥鐨勭郴缁燂紝涔熸槸灏?..鐨勬渶浣抽€夐」銆?|
| FORCE_NR_CPUS | def_bool | This option provides a glob_match function for performing simple text pattern matching.  It originated in the ATA code to blacklist particular drive models, but other 璁惧椹卞姩绋嬪簭 may need simila... |
| FORTIFY_KUNIT_TEST | tristate | 鏋勫缓鐢ㄤ簬妫€鏌?FORTIFY_SOURCE 鍐呴儴鏈哄埗鐨勫崟鍏冩祴璇曪紝FORTIFY_SOURCE 鐢?str*() 涓?mem*() 绯诲垪鍑芥暟浣跨敤銆傛湁鍏?FORTIFY_SOURCE 杩愯鏃堕櫡闃辩殑娴嬭瘯锛岃鍙傞槄 LKDTM 鐨?"FORTIFY_*" 娴嬭瘯銆?|
| FPROBE_SANITY_TEST | bool | 璇ラ€夐」灏嗗湪绯荤粺鍚姩鏃跺惎鐢ㄥ fprobe 鐨勬祴璇曘€備細杩涜涓€绯诲垪娴嬭瘯浠ラ獙璇?fprobe 鏄惁姝ｅ父宸ヤ綔銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| FRAME_WARN | int | 鍛婅瘔缂栬瘧鍣ㄥ湪鏋勫缓鏃跺澶т簬姝ゅ€肩殑鏍堝抚鍙戝嚭璀﹀憡銆傝缃繃浣庝細瀵艰嚧澶ч噺璀﹀憡銆傝缃负 0 鍒欑鐢ㄨ璀﹀憡銆?|
| FREEZER | def_bool |  |
| FS_DAX_PMD | bool | 璇ラ€夐」鍚敤鏂囦欢绯荤粺鐨勫鍑烘搷浣滐紝浠ユ敮鎸佸閮ㄥ潡 IO銆?|
| FS_IOMAP | bool | 鐩存帴璁块棶锛圖AX锛夊彲鐢ㄤ簬鍐呭瓨鍚庡鐨勫潡璁惧銆傚鏋滃潡璁惧鏀寔 DAX 涓旀枃浠剁郴缁熸敮鎸?DAX锛屼綘灏卞彲浠ラ伩鍏嶇敤椤电紦瀛樻潵缂撳啿 I/O銆傚紑鍚?.. |
| FUNCTION_ERROR_INJECTION | bool | 鍚戝唴鏍镐腑鐢?ALLOW_ERROR_INJECTION() 娉ㄨВ鐨勫悇绉嶅嚱鏁版敞鍏ユ晠闅溿€侭PF 涔熷彲鑳戒慨鏀硅繖浜涘嚱鏁扮殑杩斿洖鍊笺€傝繖鏈夊姪浜庢祴璇曢敊璇矾寰?.. |
| FUTEX | bool | 绂佺敤璇ラ€夐」灏嗗鑷存瀯寤哄嚭鐨勫唴鏍镐笉鏀寔"蹇€熺敤鎴风┖闂翠簰鏂ヤ綋"銆傛墍寰楀唴鏍稿彲鑳芥棤娉曟纭繍琛屽熀浜?glibc 鐨勫簲鐢ㄧ▼搴忋€?|
| FUTEX_PI | bool | 绂佺敤璇ラ€夐」灏嗗鑷存瀯寤哄嚭鐨勫唴鏍镐笉鏀寔 epoll 绯诲垪绯荤粺璋冪敤銆?|
| GCD_KUNIT_TEST | tristate | 璇ラ€夐」鍚敤閽堝 gcd() 鍑芥暟鐨?KUnit 娴嬭瘯濂椾欢锛実cd() 璁＄畻涓や釜鏁扮殑鏈€澶у叕绾︽暟銆傝娴嬭瘯濂椾欢鍦ㄥ悇绉嶅満鏅笅楠岃瘉 gcd() 鐨勬纭€?.. |
| GCOV_PROFILE_URING | bool | 鍦?io_uring 瀛愮郴缁熶笂鍚敤 GCOV 鎬ц兘鍒嗘瀽锛屼互鏂逛究浠ｇ爜瑕嗙洊鐜囨祴璇曘€傚鏋滀笉纭畾锛岄€夋嫨 N銆傛敞鎰忚繖浼氬 io_uring 瀛愮郴缁熺殑鎬ц兘浜х敓璐熼潰褰卞搷锛屽洜姝?.. |
| GDB_SCRIPTS | bool | 杩欎細鍦ㄦ瀯寤虹洰褰曚腑鍒涘缓鎸囧悜 GDB 杈呭姪鑴氭湰鎵€闇€鐨勯摼鎺ャ€傚鏋滀綘灏?vmlinux 鍔犺浇鍒?gdb 涓紝杈呭姪鑴氭湰涔熶細琚?gdb 鑷姩瀵煎叆锛屽苟鎻愪緵棰濆鐨勫姛鑳?.. |
| GENERIC_EARLY_IOREMAP | bool | 杩欐槸 32 浣嶇敤鎴疯繘绋嬪湪鏍堝悜涓婂闀挎椂锛堢洰鍓嶄粎鍦?parisc 鏋舵瀯涓婏級鐨?VM 甯冨眬涓爤鐨勬渶澶уぇ灏忥紙浠?MB 涓哄崟浣嶏級锛屽綋 RLIMIT_STACK 纭檺鍒朵负鏃犻檺鍒舵椂銆備竴涓?.. |
| GENERIC_IOREMAP | bool |  |
| GLOB_KUNIT_TEST | tristate | 鍚敤璇ラ€夐」浠ュ湪杩愯鏃舵祴璇?glob 鍑芥暟銆傝娴嬭瘯濂椾欢鍦ㄥ悇绉嶅満鏅紙鍖呮嫭杈圭晫鎯呭喌锛変笅楠岃瘉 glob_match() 鐨勬纭€с€傚鏋滀笉纭畾锛岄€夋嫨 N |
| GRACE_PERIOD | tristate | 涓€浜?NFS 鏈嶅姟鍣ㄦ敮鎸佷竴涓緟鍔╃殑 NFS LOCALIO 鍗忚锛屽畠涓嶆槸 NFS 鍗忚鐨勬寮忛儴鍒嗐€傝閫夐」鍦ㄥ唴鏍哥殑 NFS 鏈嶅姟鍣ㄤ笌瀹㈡埛绔腑鍚敤瀵?LOCALIO 鍗忚鐨勬敮鎸?.. |
| GROUP_SCHED_WEIGHT | def_bool | 璇ラ€夐」鍏佽鐢ㄦ埛鍦ㄥ叕骞崇粍璋冨害鍣ㄥ唴涓轰换鍔″畾涔?CPU 甯﹀閫熺巼锛堥檺鍒讹級銆傛湭璁剧疆闄愬埗鐨勭粍琚涓轰笉鍙楃害鏉燂紝灏嗕互鏃?..鐨勬柟寮忚繍琛屻€?|
| GUEST_PERF_EVENTS | bool | 璇︽儏璇峰弬闃?tools/perf/design.txt |
| GUP_GET_PXX_LOW_HIGH | bool | 鎻愪緵涓€涓祴璇曟ā鍧楋紝瀹冧細鍒嗛厤骞堕噴鏀捐澶氫笉鍚屽ぇ灏忕殑鍧楋紝骞舵姤鍛婅€楁椂銆傛棬鍦ㄦ彁渚涗竴绉嶄竴鑷寸殑鏂规硶鏉ヨ　閲忓 dma_pool_all...鐨勪慨鏀规晥鏋溿€?|
| GUP_TEST | bool | 鎻愪緵 /sys/kernel/debug/gup_test锛岃繘鑰屾彁渚涗竴绉嶅彂璧?ioctl 璋冪敤鐨勬柟寮忥紝鐢ㄤ簬鍚姩閽堝 get_user_pages*() 涓?pin_user_pages*() 绯诲垪 API 鐨勫唴鏍稿崟鍏冩祴璇曘€傝繖... |
| HARDLOCKUP_DETECTOR_COUNTS_HRTIMER | bool | 鍦ㄦ閫夋嫨 Y 浠ヨ鍐呮牳鍦?纭攣姝?鏃舵亹鎱岋紝纭攣姝绘槸鎸囧鑷村唴鏍稿湪涓柇绂佺敤鐘舵€佷笅浜庡唴鏍告ā寮忓惊鐜秴杩?10 绉掞紙鍙€氳繃 watchdog...閰嶇疆锛夌殑 Bug銆?|
| HARDLOCKUP_DETECTOR_PERF | bool | 灏嗕娇鐢ㄧ壒瀹氫簬浣撶郴缁撴瀯鐨勭‖閿佹妫€娴嬪櫒瀹炵幇銆? # "perf" 涓?"buddy" 纭攣姝绘娴嬪櫒閮藉 hrtimer 涓柇璁℃暟銆傝閰嶇疆鍚敤绠＄悊杩欎簺...鐨勫嚱鏁般€?|
| HARDLOCKUP_DETECTOR_PREFER_BUDDY | bool | 鍦ㄦ閫夋嫨 Y 浠ヤ紭鍏堜娇鐢?buddy 纭攣姝绘娴嬪櫒鑰岄潪 perf 妫€娴嬪櫒銆備娇鐢?buddy 妫€娴嬪櫒鏃讹紝姣忎釜 CPU 鍒╃敤鍏?softlockup hrtimer 閫氳繃妫€鏌ヤ笅涓€涓?CPU 鏄惁姝ｅ湪澶勭悊 hrtimer 涓柇鏉?.. |
| HASHTABLE_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 hashtable 鐨?KUnit 娴嬭瘯濂椾欢銆傚畠娴嬭瘯 include/linux/hashtable.h 涓畾涔夌殑 API 鐨勫熀鏈姛鑳姐€傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄... |
| HASH_KUNIT_TEST | tristate | 鍚敤璇ラ€夐」浠ュ湪鍚姩鏃舵祴璇曞唴鏍哥殑瀛楃涓诧紙<linux/stringhash.h>锛変笌鏁存暟锛?linux/hash.h>锛夊搱甯屽嚱鏁般€侹Unit 娴嬭瘯鍦ㄥ惎鍔ㄦ椂杩愯锛屽苟浠?TA...鏍煎紡灏嗙粨鏋滆緭鍑哄埌璋冭瘯鏃ュ織銆?|
| HAS_SECURITY_AUDIT | def_bool | 杩欏皢鏋勫缓 securityfs 鏂囦欢绯荤粺銆傚畠鐩墠琚悇绉嶅畨鍏ㄦā鍧楋紙AppArmor銆両MA銆丼afeSetID銆乀OMOYO銆乀PM锛変娇鐢ㄣ€傚鏋滀綘涓嶇‘瀹氬浣曞洖绛旓紝閫夋嫨 N銆?|
| HAVE_ARCH_AUDITSYSCALL | bool | 杩欐槸鍩烘湰鐨勫熀浜?tick 鐨?cputime 璁拌处锛屾寜姣忔 jiffy 鐨勭矑搴︾淮鎶ゅ叧浜庣敤鎴枫€佺郴缁熶笌绌洪棽鏃堕棿娑堣€楃殑缁熻銆傚鏋滀笉纭畾锛岄€夋嫨 Y銆?|
| HAVE_ARCH_TLB_REMOVE_TABLE | def_bool | 灏濊瘯鍦?munmap 涓?exit_mmap 璺緞涔嬪鐨勮矾寰勪腑鍥炴敹绌虹殑鐢ㄦ埛椤佃〃椤点€傛敞鎰忥細鐩墠鍙細鍥炴敹绌虹殑鐢ㄦ埛 PTE 椤佃〃椤点€?|
| HAVE_ARCH_USERFAULTFD_MINOR | bool | 浣撶郴缁撴瀯鍏锋湁 userfaultfd 娆¤缂洪〉鏀寔銆?|
| HAVE_ARCH_USERFAULTFD_WP | bool | 浣撶郴缁撴瀯鍏锋湁 userfaultfd 鍐欎繚鎶ゆ敮鎸併€?|
| HAVE_DEBUG_BUGVERBOSE | bool | 鍚敤璇ラ€夐」浠ュ紑鍚摼琛ㄩ亶鍘嗕緥绋嬩腑鐨勬墿灞曟鏌ャ€傝閫夐」浠ユ€ц兘鎹㈠彇鏇撮珮璐ㄩ噺鐨勯敊璇姤鍛婏紝鏇撮€傚悎鍐呮牳璋冭瘯銆傚鏋滀綘鍦ㄦ剰... |
| HAVE_DEBUG_STACKOVERFLOW | bool | 濡傛灉浣犳兂妫€鏌ュ唴鏍搞€両RQ 涓庡紓甯告爤锛堣嫢浣犵殑浣撶郴缁撴瀯浣跨敤瀹冧滑锛夌殑婧㈠嚭锛屽湪姝ら€夋嫨 Y銆傚鏋滅┖闂叉爤绌洪棿浣庝簬鏌愪釜...锛岃閫夐」灏嗘樉绀鸿缁嗘秷鎭€?|
| HAVE_HARDLOCKUP_DETECTOR_BUDDY | bool | 鍦ㄦ閫夋嫨 Y 浠ヨ鍐呮牳鍏呭綋鐪嬮棬鐙楁潵妫€娴嬬‖閿佹銆傜‖閿佹鏄寚瀵艰嚧 CPU 鍦ㄥ唴鏍告ā寮忓惊鐜秴杩?10 绉掋€佷笖涓嶈鍏朵粬涓柇...鐨?Bug銆?|
| HAVE_KERNEL_GZIP | bool | Linux 鍐呮牳鏄竴绉嶈嚜瑙ｅ帇鍙墽琛屾枃浠躲€傛湁澶氱鍘嬬缉绠楁硶鍙敤锛屽畠浠湪鏁堢巼銆佸帇缂╀笌瑙ｅ帇缂╅€熷害涓婃湁鎵€涓嶅悓銆傚帇缂╅€熷害浠呭湪...鏃舵湁鎰忎箟銆?|
| HAVE_LD_DEAD_CODE_DATA_ELIMINATION | bool | 杩欒姹備綋绯荤粨鏋勫鍏跺閮ㄥ叆鍙ｇ偣杩涜娉ㄨВ鎴栦互鍏朵粬鏂瑰紡淇濇姢锛屼娇鍏朵笉琚涪寮冦€傞摼鎺ュ櫒鑴氭湰杩樺繀椤绘纭湴灏?.text.*銆?data.* 涓?.bss.* 鍚堝苟鍒拌緭鍑鸿妭...銆?|
| HAVE_PCSPKR_PLATFORM | bool | 璇ラ€夐」鍏佽绂佺敤鎴栬皟鏁存煇浜涘熀纭€鍐呮牳閫夐」涓庤缃€傝繖鐢ㄤ簬鑳藉瀹瑰繊"闈炴爣鍑?鍐呮牳鐨勪笓鐢ㄧ幆澧冦€傚彧鏈夊綋浣犵‘瀹?..鏃舵墠浣跨敤銆?|
| HAVE_PERF_EVENTS | bool | 璇︽儏璇峰弬闃?tools/perf/design.txt銆?|
| HAVE_SCHED_AVG_IRQ | def_bool | 閫夋嫨璇ラ€夐」浠ュ湪璋冨害鍣ㄤ腑鍚敤纭欢鍘嬪姏璁拌处銆傜‖浠跺帇鍔涙槸浼犺揪缁欒皟搴﹀櫒鐨勫€硷紝鍙嶆槧鐢辩‖浠惰妭娴佸鑷寸殑 CPU 璁＄畻鑳藉姏闄嶄綆... |
| HAVE_UNSTABLE_SCHED_CLOCK | bool | 璇ョ壒鎬т娇璋冨害鍣ㄨ兘澶熷熀浜庡綋鍓嶅湪璇?CPU 涓婅皟搴︾殑 RUNNABLE 浠诲姟璺熻釜姣忎釜 CPU 鐨勯挸鍒跺埄鐢ㄧ巼銆傞€氳繃璇ラ€夐」锛岀敤鎴峰彲浠ユ寚瀹?CPU 鍒╃敤鐜囩殑鏈€灏忓€间笌鏈€澶у€?.. |
| HEADERS_INSTALL | bool | 璇ラ€夐」灏嗗畨瑁?uapi 澶存枃浠讹紙瀵煎嚭鍒扮敤鎴风┖闂寸殑澶存枃浠讹級鍒?usr/include 鐩綍锛屼緵鍐呮牳鏋勫缓鏈熼棿浣跨敤銆傛瀯寤哄唴鏍告湰韬笉闇€瑕佸畠锛屼絾...闇€瑕佸畠銆?|
| HMM_MIRROR | bool | 鍏佽鍒涘缓 struct page 鏉ヨ〃绀轰笉鍙鍧€鐨勮澶囧唴瀛橈紱鍗冲彧鑳戒粠璁惧锛堟垨璁惧缁勶級璁块棶鐨勫唴瀛樸€備綘鍙兘杩樺笇鏈涢€夋嫨 HMM_MIRROR銆?|
| HUGETLB_PAGE | def_bool | 鍦ㄦ閫夋嫨 Y 浠ユ煡鐪嬪悇绉嶆潅椤规枃浠剁郴缁熺殑閫夐」锛屼緥濡傛潵鑷叾浠栨搷浣滅郴缁熺殑鏂囦欢绯荤粺銆傝閫夐」鏈韩涓嶆坊鍔犱换浣曞唴鏍镐唬鐮併€傚鏋滀綘閫夋嫨 N锛屾墍鏈?.. |
| HUGETLB_PAGE_OPTIMIZE_VMEMMAP_DEFAULT_ON | bool | HugeTLB Vmemmap 浼樺寲锛圚VO锛夐粯璁ゅ叧闂€傚湪姝ら€夋嫨 Y 浠ラ粯璁ゅ惎鐢?HVO銆傚畠鍙€氳繃 hugetlb_free_vmemmap=off锛堝惎鍔ㄥ懡浠よ锛夋垨 hugetlb_optimize_vmemmap锛坰ysctl锛夌鐢ㄣ€?|
| HWPOISON_INJECT | tristate | NOMMU 鐨?mmap() 缁忓父闇€瑕佸垎閰嶅ぇ鍧楄繛缁唴瀛樻潵瀛樺偍鏄犲皠锛屼絾瀹冨彧鑳藉悜绯荤粺鍒嗛厤鍣ㄨ姹?2^N*PAGE_SIZE 澶у皬鐨勫潡鈥斺€旇繖... |
| HW_BREAKPOINT_KUNIT_TEST | bool | hw_breakpoint 绾︽潫璁拌处娴嬭瘯銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| HYPERV_TESTING | bool | 閫夋嫨璇ラ€夐」浠ュ惎鐢?Hyper-V vmbus 娴嬭瘯銆?|
| IDLE_PAGE_TRACKING | bool | 璇ョ壒鎬у厑璁镐及璁″湪缁欏畾鏃堕棿娈靛唴鏈璁块棶鐨勭敤鎴烽〉鏁伴噺銆傝淇℃伅鍙敤浜庤皟浼樺唴瀛?cgroup 闄愬埗鍜?鎴栦负浣滀笟鏀剧疆... |
| IKCONFIG | tristate | 璇ラ€夐」鍚敤灏嗗畬鏁寸殑 Linux 鍐呮牳 ".config" 鏂囦欢鍐呭淇濆瓨鍒板唴鏍镐腑銆傚畠璁板綍浜嗚繍琛屽唴鏍告垨纾佺洏鍐呮牳涓娇鐢ㄤ簡鍝簺鍐呮牳閫夐」... |
| IKCONFIG_PROC | bool | 璇ラ€夐」鍚敤閫氳繃 /proc/config.gz 璁块棶鍐呮牳閰嶇疆鏂囦欢銆?|
| IKHEADERS | tristate | 璇ラ€夐」鍚敤璁块棶鏋勫缓杩囩▼涓敓鎴愮殑鍐呮牳澶存枃浠躲€傝繖浜涘彲鐢ㄤ簬鏋勫缓 eBPF 璺熻釜绋嬪簭鎴栫被浼肩▼搴忋€傚鏋滀綘灏嗗ご鏂囦欢鏋勫缓涓?.. |
| INDIRECT_IOMEM | bool | 璇ラ€夐」鐢卞叾浠栭€夐」/浣撶郴缁撴瀯閫変腑锛屼互鎻愪緵妯℃嫙鐨?iomem 璁块棶鍣ㄣ€?|
| INDIRECT_IOMEM_FALLBACK | bool | 濡傛灉閫変腑 INDIRECT_IOMEM锛岃閫夐」鍦?IO 鍐呭瓨鍦板潃涓嶆槸宸叉敞鍐岀殑妯℃嫙鍖哄煙鏃讹紝鍚敤鍥為€€鍒版櫘閫?mmio 璁块棶銆?|
| INET | bool | 杩欎簺鏄敤浜庝簰鑱旂綉涓庡ぇ澶氭暟鏈湴浠ュお缃戜笂鐨勫崗璁€傚己鐑堝缓璁湪姝ら€夋嫨 Y锛堣繖浼氫娇鍐呮牳澧炲ぇ绾?400 KB锛夛紝鍥犱负鏌愪簺绋嬪簭锛堜緥濡?X 绐楀彛...锛夐渶瑕佸畠銆?|
| INITRAMFS_PRESERVE_MTIME | bool | initramfs cpio 褰掓。涓殑姣忎釜鏉＄洰閮藉甫鏈変竴涓?mtime 鍊笺€傚惎鐢ㄥ悗锛屾彁鍙栫殑 cpio 椤归噰鐢ㄨ mtime锛岀洰褰?mtime 鐨勮缃帹杩熷埌鍏朵换浣曞瓙椤瑰垱寤轰箣鍚庛€?.. |
| INITRAMFS_TEST | bool | 涓?initramfs 鏋勫缓 KUnit 娴嬭瘯銆傝鍙傞槄 Documentation/dev-tools/kunit |
| INTEL_TXT | bool | 璇ラ€夐」鍚敤鍦ㄥ唴鏍镐腑浣跨敤鍙俊鍚姩锛坱boot锛夋ā鍧楀惎鍔ㄧ殑鏀寔銆傝繖灏嗗埄鐢?Intel(R) 鍙俊鎵ц鎶€鏈鍐呮牳杩涜搴﹂噺鍚姩銆傚鏋?.. |
| INTERVAL_TREE_SPAN_ITER | bool | 鏀寔鍦?XArray 涓崰鎹涓繛缁储寮曠殑鏉＄洰銆?|
| INTERVAL_TREE_TEST | tristate | 涓€涓祴閲忓尯闂存爲搴撴€ц兘鐨勫熀鍑嗘祴璇曘€?|
| INT_LOG_KUNIT_TEST | tristate | 璇ラ€夐」鍚敤閽堝 int_log 搴撶殑 KUnit 娴嬭瘯濂椾欢锛岃搴撴彁渚涗袱涓垎鍒О涓?intlog2 涓?intlog10 鐨勫嚱鏁帮紝鐢ㄤ簬璁＄畻浠?2 涓哄簳鍜屼互 10 涓哄簳鐨勬暣鏁板鏁般€傝... |
| INT_POW_KUNIT_TEST | tristate | 璇ラ€夐」鍚敤閽堝 int_pow 鍑芥暟锛堟墽琛屾暣鏁板箓杩愮畻锛夌殑 KUnit 娴嬭瘯濂椾欢銆傝娴嬭瘯濂椾欢鏃ㄥ湪楠岃瘉 int_pow 鐨勫疄鐜拌兘姝ｇ‘璁＄畻... |
| INT_SQRT_KUNIT_TEST | tristate | 璇ラ€夐」鍚敤閽堝 int_sqrt() 鍑芥暟锛堟墽琛屽钩鏂规牴璁＄畻锛夌殑 KUnit 娴嬭瘯濂椾欢銆傝娴嬭瘯濂椾欢妫€鏌ュ悇绉嶅満鏅紙鍖呮嫭杈圭晫鎯呭喌锛変互纭繚姝ｇ‘鎬с€傚鏋?.. |
| IO_STRICT_DEVMEM | bool | 濡傛灉绂佺敤璇ラ€夐」锛屼綘灏嗗厑璁哥敤鎴风┖闂达紙root锛夎闂墍鏈?io 鍐呭瓨锛屾棤璁洪┍鍔ㄦ槸鍚︽鍦ㄤ娇鐢ㄨ鑼冨洿銆傛剰澶栬闂繖鏄剧劧鏄伨闅炬€х殑锛屼絾... |
| IO_URING | bool | 璇ラ€夐」鍚敤瀵?io_uring 鎺ュ彛鐨勬敮鎸侊紝浣垮簲鐢ㄧ▼搴忚兘澶熼€氳繃鍐呮牳涓庡簲鐢ㄧ▼搴忎箣闂村叡浜殑鎻愪氦涓庡畬鎴愮幆鏉ユ彁浜ゅ苟瀹屾垚 IO銆?|
| IO_URING_MOCK_FILE | tristate | 涓?io_uring 瀛愮郴缁熸祴璇曞惎鐢ㄦā鎷熸枃浠躲€侫BI 浠嶅彲鑳藉彉鍖栵紝鍥犳瀹冧粛鏄疄楠屾€х殑锛屽彧搴斾负鐗瑰畾娴嬭瘯鐩殑鍚敤銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| IO_URING_ZCRX | def_bool |  |
| IRQ_TIME_ACCOUNTING | bool | 閫夋嫨璇ラ€夐」浠ュ惎鐢ㄧ粏绮掑害鐨勪换鍔?IRQ 鏃堕棿璁拌处銆傝繖閫氳繃鍦ㄨ蒋涓柇涓庣‖涓柇鐘舵€佷箣闂寸殑姣忔杞崲璇诲彇鏃堕棿鎴虫潵瀹炵幇锛屽洜姝ゅ彲鑳戒細鏈夊皬鐨勬€ц兘... |
| IS_SIGNED_TYPE_KUNIT_TEST | tristate | 鏋勫缓閽堝 is_signed_type() 瀹忕殑鍗曞厓娴嬭瘯銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/ 涓殑 KUnit 鏂囨。銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| KALLSYMS | bool | 鍦ㄦ閫夋嫨 Y 浠ヨ鍐呮牳鎵撳嵃绗﹀彿鍖栫殑宕╂簝淇℃伅涓庣鍙峰寲鏍堝洖婧€傝繖浼氫娇鍐呮牳浣撶Н鐣ユ湁澧炲ぇ锛屽洜涓烘墍鏈夌鍙烽兘蹇呴』鍔犺浇鍒板唴鏍搁暅鍍忎腑銆?|
| KALLSYMS_ALL | bool | 閫氬父 kallsyms 鍙寘鍚嚱鏁扮殑绗﹀彿锛屼互渚垮緱鍒版洿濂界殑 OOPS 娑堟伅涓庡洖婧紙鍗虫潵鑷?text 涓?inittext 鑺傜殑绗﹀彿锛夈€傝繖瀵瑰ぇ澶氭暟鎯呭喌宸茶冻澶熴€傚彧鏈夊湪浣?.. |
| KALLSYMS_SELFTEST | bool | 娴嬭瘯涓€浜涙帴鍙ｏ紙渚嬪 kallsyms_lookup_name锛夌殑鍩烘湰鍔熻兘涓庢€ц兘銆傚畠杩樿绠楀綋鍓嶇鍙烽泦鐨?kallsyms 鍘嬬缉绠楁硶鐨勫帇缂╃巼銆?.. |
| KCMP | bool | 鍚敤鍐呮牳璧勬簮姣旇緝绯荤粺璋冪敤銆傚畠涓虹敤鎴风┖闂存彁渚涙瘮杈冧袱涓繘绋嬫槸鍚﹀叡浜叕鍏辫祫婧愶紙渚嬪鏂囦欢鎻忚堪绗︾敋鑷宠櫄鎷?..锛夌殑鑳藉姏銆?|
| KCOV | bool | KCOV 浠ラ€傚悎瑕嗙洊鐜囧紩瀵兼ā绯婃祴璇曪紙闅忔満鍖栨祴璇曪級鐨勫舰寮忓鍑哄唴鏍镐唬鐮佽鐩栫巼淇℃伅銆傛洿澶氱粏鑺傝鍙傞槄 Documentation/dev-tools/kcov.rst銆?|
| KCOV_ENABLE_COMPARISONS | bool | KCOV 杩樺鍑鸿鎻掓々浠ｇ爜涓瘡娆℃瘮杈冪殑鎿嶄綔鏁帮紝浠ュ強鎿嶄綔鏁板ぇ灏忎笌姣旇緝鎸囦护鐨?PC銆傝繖浜涙搷浣滄暟鍙妯＄硦娴嬭瘯寮曟搸鐢ㄦ潵鏀硅繘... |
| KCOV_INSTRUMENT_ALL | bool | 濡傛灉浣犲湪杩涜閫氱敤鐨勭郴缁熻皟鐢ㄦā绯婃祴璇曪紙渚嬪 syzkaller锛夛紝浣犱細甯屾湜鎻掓々鏁翠釜鍐呮牳锛屽苟搴斿湪姝ら€夋嫨 y銆傚鏋滀綘鍦ㄨ繘琛屾洿鏈夐拡瀵规€х殑妯＄硦娴嬭瘯锛堜緥濡?..锛夛紝 |
| KCOV_IRQ_AREA_SIZE | hex | KCOV 浣跨敤棰勫垎閰嶇殑姣?CPU 鍖哄煙鏉ヤ粠杞腑鏂敹闆嗚鐩栫巼銆傝繖鎸囧畾浜嗚繖浜涘尯鍩熺殑澶у皬锛屼互 unsigned long 瀛楁暟璁°€?|
| KCOV_SELFTEST | bool | 鍦ㄥ惎鍔ㄦ椂杩愯绠€鐭殑 KCOV 瑕嗙洊鐜囨敹闆嗚嚜娴嬨€傛祴璇曞け璐ユ椂瀵艰嚧鍐呮牳鎭愭厡銆傚缓璁惎鐢紝浠ョ‘淇濆叧閿姛鑳芥寜棰勬湡宸ヤ綔銆?|
| KERNEL_BZIP2 | bool | 瀹冪殑鍘嬬缉鐜囦笌閫熷害灞呬腑銆傝В鍘嬬缉閫熷害鍦ㄥ彲閫夋柟妗堜腑鏄渶鎱㈢殑銆備笌 gzip 鐩告瘮锛屼娇鐢?bzip2 鐨勫唴鏍镐綋绉皬绾?10%銆俠zip2 鍗犵敤澶ч噺... |
| KERNEL_GZIP | bool | 涔呯粡鑰冮獙鐨勬棫寮?gzip 鍘嬬缉銆傚畠鍦ㄥ帇缂╃巼涓庤В鍘嬬缉閫熷害涔嬮棿鎻愪緵浜嗚壇濂界殑骞宠　銆?|
| KERNEL_LZ4 | bool | LZ4 is an LZ77-type compressor with a fixed, byte-oriented encoding. A preliminary version of LZ4 de/compression tool is available at <https://code.google.com/p/lz4/>. Its compression ratio is wors... |
| KERNEL_LZMA | bool | 璇ュ帇缂╃畻娉曠殑鍘嬬缉鐜囨渶浣炽€傝В鍘嬬缉閫熷害浠嬩簬 gzip 涓?bzip2 涔嬮棿銆傚帇缂╂渶鎱€備笌 gzip 鐩告瘮锛屼娇鐢?LZMA 鐨勫唴鏍镐綋绉皬绾?33%銆?|
| KERNEL_LZO | bool | 瀹冪殑鍘嬬缉鐜囧湪鍙€夋柟妗堜腑鏈€宸€傚唴鏍镐綋绉瘮 gzip 澶х害 10%锛涗絾鍏堕€熷害锛堝帇缂╀笌瑙ｅ帇缂╋級鏈€蹇€?|
| KERNEL_UNCOMPRESSED | bool | 鐢熸垚鏈帇缂╃殑鍐呮牳闀滃儚銆傝閫夐」閫氬父涓嶆槸浣犳兂瑕佺殑銆傚畠鍦ㄧ紦鎱㈢殑浠跨湡鐜涓皟璇曞唴鏍告椂鏈夌敤锛屽湪閭ｉ噷瑙ｅ帇鍜岀Щ鍔ㄥ唴鏍搁潪甯?.. |
| KERNEL_XZ | bool | XZ 浣跨敤 LZMA2 绠楁硶浠ュ強鐗瑰畾浜庢寚浠ら泦鐨?BCJ 杩囨护鍣紝鍙敼鍠勫彲鎵ц浠ｇ爜鐨勫帇缂╃巼銆備笌 gzip 鐩告瘮锛屼娇鐢?XZ 鐨勫唴鏍镐綋绉皬绾?30%... |
| KERNEL_ZSTD | bool | ZSTD 鏄竴绉嶉潰鍚戜腑绛夊帇缂╃巼涓庡揩閫熻В鍘嬬缉閫熷害鐨勫帇缂╃畻娉曘€傚畠鐨勫帇缂╂瘮 GZIP 鏇村ソ锛岃В鍘嬮€熷害涓?LZO 澶ц嚧鐩稿綋锛屼絾姣?LZ4 鎱?.. |
| KFIFO_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓閫氱敤 FIFO 瀹炵幇鐨?KUnit 娴嬭瘯濂椾欢銆傚畠娴嬭瘯 kfifo 绫诲瀷鍙婄浉鍏冲畯鐨?API 涓庡熀鏈姛鑳姐€傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅璇峰弬闃?.. |
| KPROBES_SANITY_TEST | tristate | 璇ラ€夐」鎻愪緵鍦ㄥ惎鍔ㄦ椂娴嬭瘯鍩烘湰 kprobes 鍔熻兘鐨勮兘鍔涖€傛彃鍏?kprobe 涓?kretprobe 鏍锋湰骞堕獙璇佸叾鍔熻兘銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| LATENCYTOP | bool | 濡傛灉浣犳兂浣跨敤 LatencyTOP 宸ュ叿鎵惧嚭鍝釜鐢ㄦ埛绌洪棿杩涚▼闃诲鍦ㄥ摢涓唴鏍告搷浣滀笂锛屽惎鐢ㄨ閫夐」銆?|
| LAZY_MMU_MODE_KUNIT_TEST | tristate | 鍚敤璇ラ€夐」浠ユ鏌ユ儼鎬?MMU 妯″紡鎺ュ彛鏄惁濡傞鏈熻埇宸ヤ綔銆傚彧鍖呭惈瀵归€氱敤鎺ュ彛鐨勬祴璇曪紙涓嶅寘鎷綋绯荤粨鏋勭壒瀹氱殑琛屼负锛夈€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| LD_DEAD_CODE_DATA_ELIMINATION | bool | 鍚敤瀵?-ffunction-sections -fdata-sections 缂栬瘧骞朵娇鐢?--gc-sections 閾炬帴锛屼互閫氳繃閾炬帴鍣ㄨ繘琛屾浠ｇ爜涓庢鏁版嵁娑堥櫎銆傝繖鍙互鍑忓皯纾佺洏涓婂拰鍐呭瓨涓?.. |
| LD_ORPHAN_WARN | def_bool | 鍚敤瀵?/proc/sys/debug/exception-trace 鐨勬敮鎸併€?|
| LIBFDT | bool | 鍚敤蹇€熸煡鎵惧璞℃爣璇嗙娉ㄥ唽琛ㄣ€?|
| LINEAR_RANGES | tristate | 璇ラ€夐」鎻愪緵 packing() 杈呭姪鍑芥暟锛屽畠鍏佽鍦?CPU 鍙敤琛ㄧず涓庡彲鑳藉叿鏈変换鎰忎互涓嬬壒鎬х殑鍐呭瓨琛ㄧず涔嬮棿杞崲浣嶅煙锛?.. |
| LINEAR_RANGES_TEST | tristate | 璇ラ€夐」鏋勫缓 linear_ranges 鍗曞厓娴嬭瘯锛屽湪鍚姩鏃惰繍琛屻€傛祴璇?linear_ranges 閫昏緫鐨勬纭€с€傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 KUnit 鏂囨。... |
| LIST_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓閾捐〃 KUnit 娴嬭瘯濂椾欢銆傚畠娴嬭瘯 list_head 绫诲瀷鍙婄浉鍏冲畯鐨?API 涓庡熀鏈姛鑳姐€侹Unit 娴嬭瘯鍦ㄥ惎鍔ㄦ椂杩愯锛屽苟灏嗙粨鏋滆緭鍑哄埌璋冭瘯... |
| LIST_PRIVATE_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓閽堝 include/linux/list_private.h 涓畾涔夌殑绉佹湁閾捐〃鍘熻鐨?KUnit 娴嬭瘯銆傝繖浜涘師璇厑璁告搷浣滆鏍囪涓虹鏈変笖...鐨?list_head 鎴愬憳銆?|
| LIVEUPDATE_TEST | bool | 涓?Live Update Orchestrator 鍚敤涓€涓唴寤哄唴鏍告祴璇曟ā鍧椼€傝妯″潡閫氳繃鍚戜换鎰忕湡瀹炴枃浠跺鐞嗙▼搴忔敞鍐屼竴缁勬ā鎷?FLB 瀵硅薄鏉ラ獙璇?File-Lifecycle-Bound 瀛愮郴缁?.. |
| LKDTM | tristate | 璇ユā鍧楅€氳繃鍦ㄩ瀹氫箟鐨勫穿婧冪偣璇卞彂绯荤粺鏁呴殰鏉ユ祴璇曚笉鍚岀殑杞偍鏈哄埗銆傚鏋滀笉闇€瑕侊細閫夋嫨 N銆傚湪姝ら€夋嫨 M 灏嗘湰浠ｇ爜缂栬瘧涓烘ā鍧椼€傝... |
| LOCALVERSION | string | 鍦ㄥ唴鏍哥増鏈湯灏捐拷鍔犱竴涓澶栫殑瀛楃涓层€備緥濡傦紝杩欎細鍦ㄤ綘杈撳叆 uname 鏃舵樉绀恒€備綘鍦ㄦ璁剧疆鐨勫瓧绗︿覆灏嗚杩藉姞鍒颁换浣曚互璇ユ枃浠跺悕...鐨勬枃浠跺唴瀹逛箣鍚庛€?|
| LOCALVERSION_AUTO | bool | 杩欏皢灏濊瘯閫氳繃鏌ユ壘灞炰簬褰撳墠鏍戦《淇鐨?git 鏍囩锛岃嚜鍔ㄧ‘瀹氬綋鍓嶆爲鏄惁涓哄彂甯冩爲銆傛牸寮忎负 -gxxxxxxxx 鐨勫瓧绗︿覆灏嗚娣诲姞... |
| LOCKDEP | bool | 濡傛灉浣犻亣鍒?"BUG: MAX_LOCKDEP_ENTRIES too low!" 娑堟伅锛屽皾璇曞澶ц鍊笺€?|
| LOCKDEP_CHAINS_BITS | int | 濡傛灉浣犻亣鍒?"BUG: MAX_LOCKDEP_CHAINS too low!" 娑堟伅锛屽皾璇曞澶ц鍊笺€?|
| LOCKDEP_CIRCULAR_QUEUE_BITS | int | 濡傛灉浣犲洜 __cq_enqueue() 澶辫触鑰岄亣鍒?"lockdep bfs error:-1" 璀﹀憡锛屽皾璇曞澶ц鍊笺€?|
| LOCKDEP_STACK_TRACE_BITS | int | 濡傛灉浣犻亣鍒?"BUG: MAX_STACK_TRACE_ENTRIES too low!" 娑堟伅锛屽皾璇曞澶ц鍊笺€侹ASAN 浼氭樉钁楀鍔犳爤璺熻釜娑堣€楋紝鍥犱负鍏?slab 璺熻釜涓?lockdep 鐨勪緷璧?.. |
| LOCKDEP_STACK_TRACE_HASH_BITS | int | 濡傛灉浣犻渶瑕佽緝澶х殑 STACK_TRACE_HASH_SIZE锛屽皾璇曞澶ц鍊笺€?|
| LOCKUP_DETECTOR | bool | 鍦ㄦ閫夋嫨 Y 浠ヨ鍐呮牳鍏呭綋鐪嬮棬鐙楁娴嬭蒋閿佹銆傝蒋閿佹鏄寚瀵艰嚧鍐呮牳鍦ㄥ唴鏍告ā寮忓惊鐜秴杩?20 绉掋€佷笖涓嶇粰鍏朵粬浠诲姟...鐨?Bug銆?|
| LOCK_DEBUGGING_SUPPORT | bool | 璇ョ壒鎬т娇鍐呮牳鑳藉璇佹槑鍐呮牳杩愯鏃跺彂鐢熺殑鎵€鏈夊姞閿佸湪鏁板涓婃槸姝ｇ‘鐨勶細鍗冲湪浠讳綍鎯呭喌涓嬶紝浠绘剰锛堜笖灏氭湭瑙﹀彂鐨勶級缁勫悎...閮戒笉鍙兘瀵艰嚧姝婚攣銆?|
| LOCK_MM_AND_FIND_VMA | bool | 鍚敤 NUMA 浠跨湡銆傚綋浠?"numa=fake=N" 鍚姩鏃讹紙N 涓鸿妭鐐规暟锛夛紝鎵佸钩鏈哄櫒灏嗚鎷嗗垎涓鸿櫄鎷熻妭鐐广€傝繖浠呯敤浜庤皟璇曘€?|
| LOCK_STAT | bool | 璇ョ壒鎬у惎鐢ㄥ閿佺珵浜夌偣鐨勮窡韪€傛洿澶氱粏鑺傝鍙傞槄 Documentation/locking/lockstat.rst銆傝繖涔熷惎鐢ㄤ簡 "perf lock"锛坧erf 鐨勫瓙鍛戒护锛夋墍闇€鐨勯攣浜嬩欢銆傚鏋滀綘甯屾湜... |
| LOCK_TORTURE_TEST | tristate | 璇ラ€夐」鎻愪緵涓€涓唴鏍告ā鍧楋紝瀵瑰唴鏍搁攣鍘熻杩愯 torture 娴嬭瘯銆傚鏋滈渶瑕侊紝璇ュ唴鏍告ā鍧楀彲浠ュ湪琚祴鐨勬鍦ㄨ繍琛岀殑鍐呮牳涓婁簨鍚庢瀯寤恒€傚湪姝ら€夋嫨 Y... |
| LOG_BUF_SHIFT | int | 閫夋嫨鏈€灏忓唴鏍告棩蹇楃紦鍐插尯澶у皬锛堜互 2 鐨勫箓璁★級銆傛渶缁堝ぇ灏忓彈 LOG_CPU_MAX_BUF_SHIFT 閰嶇疆鍙傛暟褰卞搷锛岃涓嬨€備换浣曟洿澶х殑澶у皬涔熷彲鑳借 "log_buf_len" 鍚姩...寮哄埗銆?|
| LOG_CPU_MAX_BUF_SHIFT | int | 璇ラ€夐」鍏佽鏍规嵁 CPU 鏁伴噺澧炲ぇ榛樿鐜舰缂撳啿鍖哄ぇ灏忋€傝鍊煎畾涔夋瘡涓?CPU 鐨勮础鐚紙浠?2 鐨勫箓璁★級銆備娇鐢ㄧ殑绌洪棿閫氬父鍙湁鍑犺... |
| LONGEST_SYM_KUNIT_TEST | tristate | 娴嬭瘯鍙兘鐨勬渶闀跨鍙枫€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| LRU_GEN | bool | 涓€涓敤浜庡唴瀛樿繃閲忔彁浜ょ殑楂樻€ц兘 LRU 瀹炵幇銆傝鎯呰鍙傞槄 Documentation/admin-guide/mm/multigen_lru.rst銆?|
| LRU_GEN_ENABLED | bool | 璇ラ€夐」榛樿鍚敤澶氫唬 LRU銆?|
| LRU_GEN_STATS | bool | 闄ら潪浣犺鍒掍负璋冭瘯鐩殑鏌ョ湅琚€愬嚭浠ｉ檯鐨勫巻鍙茬粺璁★紝鍚﹀垯涓嶈鍚敤璇ラ€夐」銆傝閫夐」鏈夋瘡涓?memcg 涓庢瘡涓妭鐐圭殑鍐呭瓨寮€閿€銆?|
| LRU_GEN_WALKS_MMU | def_bool | 鍏佽鍦ㄧ己椤靛鐞嗘湡闂磋繘琛屾瘡 vma 鍔犻攣銆傝鐗规€у湪澶勭悊缂洪〉鏃跺垎鍒攣瀹氭瘡涓櫄鎷熷唴瀛樺尯鍩燂紝鑰屼笉鏄幏鍙?mmap_lock銆?|
| LSM_MMAP_MIN_ADDR | int | 杩欐槸搴斿綋鍏嶅彈鐢ㄦ埛绌洪棿鍒嗛厤褰卞搷鐨勪綆铏氭嫙鍐呭瓨閮ㄥ垎銆傞樆姝㈢敤鎴峰啓鍏ヤ綆鍦板潃椤垫湁鍔╀簬鍑忓皯鍐呮牳绌烘寚閽?Bug 鐨勫奖鍝嶃€傚浜?.. |
| LWTUNNEL_BPF | bool | 鍏佽鍦ㄨ矾鐢辨煡鎵句箣鍚庯紝灏?BPF 绋嬪簭浣滀负涓嬩竴璺冲姩浣滆繍琛屼簬浼犲叆涓庝紶鍑虹殑鏁版嵁鍖呫€?|
| LZO_COMPRESS | tristate | 椹卞姩鍙互閫夋嫨璇ラ€夐」浠ュ己鍒朵负鍙傛暟 'm'锛堜冀缃楃摝鍩熼樁鏁帮級涓?'t'锛堢籂閿欒兘鍔涳級鎸囧畾鐗瑰畾甯搁噺鍊笺€傞偅浜涚壒瀹氬€煎繀椤婚€氳繃澹版槑榛樿鍊?..鏉ヨ缃€?|
| MAGIC_SYSRQ | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽嵆浣跨郴缁熷穿婧冿紙渚嬪鍦ㄥ唴鏍歌皟璇曟湡闂达級锛屼綘涔熻兘瀵圭郴缁熸湁涓€瀹氭帶鍒讹紙渚嬪锛屼綘灏嗚兘澶熷皢缂撳啿鍖虹紦瀛樺埛鏂板埌纾佺洏銆侀噸鍚郴缁?..锛夈€?|
| MAGIC_SYSRQ_DEFAULT_ENABLE | hex | 鎸囧畾榛樿鍚敤鍝簺 SysRq 閿姛鑳姐€傚彲璁剧疆涓?1 鎴?0 浠ュ叏閮ㄥ惎鐢ㄦ垨绂佺敤锛屾垨璁剧疆涓?Documentation/admin-guide/sysrq.rst 涓弿杩扮殑浣嶆帺鐮併€?|
| MAGIC_SYSRQ_SERIAL | bool | 璁稿宓屽叆寮忔澘鍗℃湁鏂紑鐨?TTL 鐢靛钩涓插彛锛屽彲鑳戒骇鐢熶竴浜涘瀮鍦炬暟鎹紝瀵艰嚧铏氬亣鐨?sysrq 璇娴嬨€傝閫夐」鍏佽浣犲喅瀹氭槸鍚﹁鍚敤... |
| MAGIC_SYSRQ_SERIAL_SEQUENCE | string | 鎸囧畾鍙窡鍦?BREAK 涔嬪悗浠ュ湪涓插彛鎺у埗鍙颁笂鍚敤 SysRq 鐨勫瓧绗﹀簭鍒椼€傚鏋滀笉纭畾锛岀暀绌哄瓧绗︿覆锛岃閫夐」灏嗕笉琚惎鐢ㄣ€?|
| MAX_SKB_FRAGS | int | 姣忎釜 skb_shared_info 鎷ユ湁鏇村鍒嗙墖鏈夊姪浜庢彁楂?GRO 鏁堢巼銆傝繖鏈夊姪浜?BIG TCP 宸ヤ綔璐熻浇锛屼絾鍙兘鏆撮湶鏌愪簺閬楃暀椹卞姩涓殑 Bug銆傝繖涔熶細澧炲姞灏忓寘鐨勫唴瀛樺紑閿€锛?.. |
| MEMBARRIER | bool | 鍚敤 membarrier() 绯荤粺璋冪敤锛屽畠鍏佽璺ㄦ墍鏈夎繍琛岀嚎绋嬪彂鍑哄唴瀛樺睆闅滐紝鍙敤浜庨€氳繃灏嗙敤鎴风┖闂村唴瀛樺睆闅滅殑鎴愭湰闈炲绉板湴杞Щ鏉?..銆?|
| MEMCG | bool | 鎻愪緵瀵?cgroup 涓换鍔″唴瀛樺崰鐢ㄧ殑鎺у埗銆?|
| MEMCG_NMI_UNSAFE | bool | 宸茶 cgroup v2 瀹炵幇寮冪敤鐨勪紶缁?cgroup v1 鍐呭瓨鎺у埗鍣ㄣ€倂1 淇濈暀鐢ㄤ簬灏氭湭杩佺Щ鍒版柊 cgroup v2 鎺ュ彛鐨勪紶缁熷簲鐢ㄣ€傚鏋滀綘... |
| MEMCPY_KUNIT_TEST | tristate | 鏋勫缓閽堝 memcpy()銆乵emmove() 涓?memset() 鍑芥暟鐨勫崟鍏冩祴璇曘€傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/ 涓殑 KUnit 鏂囨。... |
| MEMORY_HOTREMOVE | bool | 鍏佽杩佺Щ鍦ㄥ唴瀛樻皵鐞冧腑鑶ㄨ儉鐨勯〉锛屼娇瀹冧滑鑳戒粠浠呭彲鐢ㄤ簬鍙Щ鍔ㄥ垎閰嶏紙渚嬪 ZONE_MOVABLE銆丆MA锛夌殑鍐呭瓨鍖哄煙鍒嗛厤锛屽苟涓?.. |
| MEMORY_NOTIFIER_ERROR_INJECT | tristate | 璇ラ€夐」鎻愪緵鍚戝唴瀛樼儹鎻掓嫈閫氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠閫氳繃 /sys/kernel/debug/notifier-error-inject/me...涓嬬殑 debugfs 鎺ュ彛鎺у埗銆?|
| MEMTEST | bool | 璇ラ€夐」娣诲姞鍐呮牳鍙傛暟 'memtest'锛屽厑璁歌缃苟鎵ц memtest銆俶emtest=0 琛ㄧず绂佺敤锛?- 榛樿 memtest=1 琛ㄧず鎵ц 1 绉嶆祴璇曟ā寮忥紱...memtest=17 琛ㄧず鎵ц 17 绉嶆祴璇曟ā寮?.. |
| MEM_ALLOC_PROFILING_ENABLED_BY_DEFAULT | bool | 涓哄唴瀛樺垎閰嶆€ц兘鍒嗘瀽娣诲姞甯︽湁甯姪鎬ч敊璇秷鎭殑璀﹀憡銆?|
| MEM_SOFT_DIRTY | bool | 璇ラ€夐」閫氳繃鍦?pte 涓婂紩鍏ヤ竴涓蒋鑴忎綅鏉ュ惎鐢ㄥ唴瀛樺彉鏇磋窡韪€傚綋鏈変汉鍐欏叆涓€涓〉鏃讹紝璇ヤ綅琚缃紝濡傚悓鏅€氱殑鑴忎綅锛屼絾涓嶅悓浜庡悗鑰咃紝瀹冨彲浠ヨ娓?.. |
| MESSAGE_LOGLEVEL_DEFAULT | int | 娌℃湁鎸囧畾浼樺厛绾х殑 printk 璇彞鐨勯粯璁ゆ棩蹇楃骇鍒€傝嚦灏戜粠 2.6.10 璧峰畠琚‖缂栫爜涓?KERN_WARNING锛屼絾绱у瘑瀹¤鍏舵棩蹇楃殑浜哄憳鍙兘甯屾湜灏嗗叾璁剧疆涓?.. |
| MHP_DEFAULT_ONLINE_TYPE_OFFLINE | bool | 鐑彃鎷斿唴瀛橀粯璁や笉浼氳涓婄嚎銆備负鍏锋湁澶勭悊鐑彃鎷斿唴瀛樹笂绾跨瓥鐣ョ殑椹卞姩涓庣敤鎴风瓥鐣ョ殑绯荤粺閫夋嫨姝ら」銆?|
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_AUTO | bool | 濡傛灉浣犲笇鏈涘唴鏍歌嚜鍔ㄥ皢鐑彃鎷斿唴瀛樹笂绾垮埌瀹冭涓哄悎鐞嗙殑鍖哄煙锛岄€夋嫨姝ら」銆傝鍐呭瓨鍙兘琚敤浜庡唴鏍告暟鎹€?|
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_KERNEL | bool | 濡傛灉浣犲笇鏈涘唴鏍歌嚜鍔ㄥ皢鐑彃鎷斿唴瀛樹笂绾垮埌鍙敤浜庡唴鏍告暟鎹殑鍖哄煙锛岄€夋嫨姝ら」銆傝繖閫氬父鎰忓懗鐫€ ZONE_NORMAL銆?|
| MHP_DEFAULT_ONLINE_TYPE_ONLINE_MOVABLE | bool | 濡傛灉浣犲笇鏈涘唴鏍歌嚜鍔ㄥ皢鐑彃鎷斿唴瀛樹笂绾垮埌 ZONE_MOVABLE锛岄€夋嫨姝ら」銆傝鍐呭瓨閫氬父涓嶄細琚敤浜庡唴鏍告暟鎹€傝繖搴斾粎鍦ㄧ鐞嗗憳鐭ラ亾...鏃朵娇鐢ㄣ€?|
| MIGRATION | bool | 褰撳钩鍙颁笂瀛樺湪澶氱 HugeTLB 椤靛ぇ灏忔椂锛屽厑璁?pageblock_order 鍊间负鍔ㄦ€佸€硷紝鑰岄潪浠呮爣鍑?HUGETLB_PAGE_ORDER銆傛敞鎰?pageblock_order 涓嶈兘... |
| MIN_HEAP_KUNIT_TEST | tristate | 璇ラ€夐」鍚敤閽堝鏈€灏忓爢锛坢in heap锛夊簱鐨?KUnit 娴嬭瘯濂椾欢锛岃搴撴彁渚涘垱寤轰笌绠＄悊鏈€灏忓爢鐨勫嚱鏁般€傝娴嬭瘯濂椾欢妫€鏌ユ渶灏忓爢搴撶殑鍔熻兘銆傚鏋滀笉纭畾... |
| MMAP_ALLOW_UNINITIALIZED | bool | 閫氬父锛屾寜鐓?Linux 瑙勮寖锛屼粠 mmap() 鑾峰緱鐨勫尶鍚嶅唴瀛樺湪浼犻€掔粰鐢ㄦ埛绌洪棿涔嬪墠鍏跺唴瀹逛細琚竻闄ゃ€傚惎鐢ㄨ閰嶇疆閫夐」鍏佽浣犺姹?.. |
| MM_ID | def_bool | 閫忔槑澶ч〉鍏佽鍐呮牳鍦ㄥ彲鑳芥椂閫忔槑鍦板搴旂敤绋嬪簭浣跨敤澶ч〉涓庡ぇ椤?TLB銆傝鐗规€у彲閫氳繃...瀵规煇浜涘簲鐢ㄧ▼搴忔彁鍗囪绠楁€ц兘銆?|
| MODULE_ALLOW_BTF_MISMATCH | bool | 瀵逛簬鎷嗗垎 BTF 涓?vmlinux 涓嶅尮閰嶇殑妯″潡锛屼笉鎷掔粷鍔犺浇鑰屾槸涓嶅甫 BTF 鍔犺浇銆傚惎鐢ㄦā鍧?BTF 鏃剁殑榛樿琛屼负鏄嫆缁濇绫讳笉鍖归厤鐨勬ā鍧楋紱璇ラ€夐」... |
| MPILIB | tristate | 鏉ヨ嚜 GnuPG 鐨勫绮惧害鏁板搴撱€傚畠鐢ㄤ簬瀹炵幇 RSA 鏁板瓧绛惧悕楠岃瘉锛岃楠岃瘉琚?IMA/EVM 鏁板瓧绛惧悕鎵╁睍浣跨敤銆?|
| MSEAL_SYSTEM_MAPPINGS | bool | 瀵圭郴缁熸槧灏勫簲鐢?mseal銆傜郴缁熸槧灏勫寘鎷?vdso銆乿var銆乿var_vclock銆乿ectors锛坅rm 鍏煎妯″紡锛夈€乻igpage锛坅rm 鍏煎妯″紡锛夈€乽probes銆傚唴瀛樺瘑灏侀渶瑕?64 浣嶅唴鏍?.. |
| MULTIUSER | bool | 璇ラ€夐」鍚敤瀵归潪 root 鐢ㄦ埛銆佺粍涓庤兘鍔涚殑鏀寔銆傚鏋滀綘鍦ㄦ閫夋嫨 N锛屾墍鏈夎繘绋嬪皢浠?UID 0銆丟ID 0 浠ュ強鎵€鏈夊彲鑳界殑 capability 杩愯銆傚湪姝ら€夋嫨 N 杩樹細缂栬瘧鎺?.. |
| NET | bool | 闄ら潪浣犵‘瀹炵煡閬撹嚜宸卞湪鍋氫粈涔堬紝鍚﹀垯搴斿湪姝ら€夋嫨 Y銆傚師鍥犳槸鏌愪簺绋嬪簭鍗充娇鍦ㄤ笉杩炴帴缃戠粶鐨勭嫭绔嬫満鍣ㄤ笂杩愯涔熼渶瑕佸唴鏍哥綉缁滄敮鎸?.. |
| NETDEV_ADDR_LIST_TEST | tristate | 瑕嗙洊鏍稿績缃戠粶鍩虹璁炬柦锛堜緥濡?sk_buff锛夌殑 KUnit 娴嬭瘯銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| NETDEV_NOTIFIER_ERROR_INJECT | tristate | 璇ラ€夐」鎻愪緵鍚?netdevice 閫氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠閫氳繃 /sys/kernel/debug/notifier-error-inject/netdev 涓嬬殑 debugfs 鎺ュ彛鎺у埗銆傚鏋?.. |
| NETFILTER | bool | Netfilter 鏄竴涓敤浜庤繃婊や笌绡℃敼缁忚繃浣犵殑 Linux 涓绘満鐨勭綉缁滄暟鎹寘鐨勬鏋躲€傚寘杩囨护鏈€甯歌鐨勭敤閫旀槸灏嗕綘鐨?Linux 涓绘満浣滀负淇濇姢鏈湴...鐨勯槻鐏銆?|
| NETFILTER_ADVANCED | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屼綘鍙互鍦ㄦ墍鏈?netfilter 妯″潡涔嬮棿閫夋嫨銆傚鏋滈€夋嫨 N锛屽垯涓嶅父瑙佺殑妯″潡灏嗕笉鏄剧ず锛岃€屽ぇ澶氭暟浜洪渶瑕佺殑鍩烘湰妯″潡灏嗛粯璁や负 'M'銆傚鏋滀笉纭畾锛岄€夋嫨 Y銆?|
| NETWORK_FILESYSTEMS | bool | 鍦ㄦ閫夋嫨 Y 浠ユ煡鐪嬬綉缁滄枃浠剁郴缁熶笌鏂囦欢绯荤粺鐩稿叧缃戠粶浠ｇ爜锛堜緥濡?NFS 瀹堟姢杩涚▼涓?RPCSEC 瀹夊叏妯″潡锛夌殑閫夐」銆傝閫夐」鏈韩涓嶆坊鍔犱换浣曞唴鏍镐唬鐮併€傚鏋?.. |
| NETWORK_SECMARK | bool | 璇ラ€夐」鍚敤缃戠粶鏁版嵁鍖呯殑瀹夊叏鏍囪锛岀被浼间簬 nfmark锛屼絾涓撶敤浜庡畨鍏ㄧ洰鐨勩€傚鏋滀綘涓嶇‘瀹氬浣曞洖绛旓紝閫夋嫨 N銆?|
| NET_DEVLINK | bool | 鍚敤椤垫睜缁熻浠ヨ窡韪〉姹犱腑鐨勯〉鍒嗛厤涓庡洖鏀躲€傝閫夐」鍦ㄥ垎閰嶄笌鍥炴敹璺緞涓婁骇鐢熼澶栫殑 CPU 寮€閿€锛屼互鍙婂瓨鍌ㄧ粺璁′俊鎭殑棰濆鍐呭瓨寮€閿€... |
| NET_DROP_MONITOR | tristate | 璇ョ壒鎬у湪缃戠粶鏍堜腑涓㈠純鏁版嵁鍖呮椂鍚戠敤鎴风┖闂存彁渚涘憡璀︽湇鍔°€傚憡璀﹂€氳繃 netlink 濂楁帴瀛楀箍鎾粰浠讳綍鐩戝惉鐨勭敤鎴风┖闂磋繘绋嬨€傝... |
| NET_FLOW_LIMIT | bool | 褰撴帴鏀跺鐞?CPU 鐨?backlog 杈惧埌 netdev_max_backlog 鏃讹紝缃戠粶鏍堝繀椤讳涪寮冩暟鎹寘銆傚鏋滆澶氭椿鍔ㄦ祦涓彧鏈夊皯鏁板嚑涓骇鐢熶簡缁濆ぇ澶氭暟璐熻浇锛屽氨鎻愬墠涓㈠純瀹冧滑鐨勬祦閲?.. |
| NET_INGRESS | bool | This builds the KUnit tests for the handshake upcall mechanism. KUnit tests run during boot and output the results to the debug log in TAP format (https://testanything.org/). Only useful for kernel... |
| NET_NS | bool | 鍏佽鐢ㄦ埛绌洪棿鍒涘缓鐪嬩技缃戠粶鏍堝涓疄渚嬬殑涓滆タ銆?|
| NET_PKTGEN | tristate | 璇ユā鍧楀皢浠ュ彲閰嶇疆鐨勯€熺巼锛屼粠缁欏畾鎺ュ彛娉ㄥ叆棰勯厤缃殑鏁版嵁鍖呫€傚畠鐢ㄤ簬缃戠粶鎺ュ彛鍘嬪姏娴嬭瘯涓庢€ц兘鍒嗘瀽銆傚鏋滀綘涓嶇悊瑙?.. |
| NET_PTP_CLASSIFY | def_bool | 璇ラ€夐」鍏佽鍏锋湁纭欢鏃堕棿鎴宠兘鍔涚殑 PHY锛堟垨鍏朵粬 MII 鎬荤嚎鍡呮帰璁惧锛夊缃戠粶鏁版嵁鍖呰繘琛屾椂闂存埑鏍囪銆傝閫夐」鍦ㄥ彂閫佷笌鎺ユ敹璺緞涓婂鍔犱竴浜涘紑閿€銆傚鏋?.. |
| NET_RX_BUSY_POLL | bool | 鍚敤璇ラ€夐」鍏佽灏?TCP 娴佽В鏋愬櫒涓?BPF_MAP_TYPE_SOCKMAP 涓€璧蜂娇鐢ㄣ€?|
| NFS_V4_2_SSC_HELPER | bool |  |
| NLATTR | bool | 鐢ㄤ簬浣跨敤杞杩涜涓柇缂撹В杞鐨勮緟鍔╁簱銆?|
| NOINSTR_VALIDATION | bool | 閫夋嫨璇ラ€夐」灏嗗湪閾炬帴 vmlinux 鏃跺悜 ld 浼犻€?"-Map=vmlinux.map"銆傝鏂囦欢瀵逛簬楠岃瘉涓庤皟璇曠濂囩殑鑺傚尯澶勭悊锛屼互鍙婃煡鐪嬪摢浜涗唬鐮佹琚秷闄?..寰堟湁鐢ㄣ€?|
| NOTIFIER_ERROR_INJECTION | tristate | 璇ラ€夐」鎻愪緵鍚戞寚瀹氱殑閫氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠鍙敤浜庢祴璇曢€氱煡閾惧け璐ユ椂鐨勯敊璇鐞嗐€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| NO_PAGE_MAPCOUNT | bool | 瀵逛簬灞炰簬杈冨ぇ鍒嗛厤锛堜緥濡傞€忔槑澶ч〉锛変竴閮ㄥ垎鐨勯〉锛屼笉缁存姢姣忛〉 mapcount銆傚惎鐢ㄨ閰嶇疆閫夐」鍚庯紝涓€浜涗緷璧栨淇℃伅鐨勬帴鍙ｅ皢... |
| NUMA_BALANCING_DEFAULT_ENABLED | bool | 鑻ヨ缃紝鍦?NUMA 鏈哄櫒涓婅繍琛屾椂灏嗗惎鐢ㄨ嚜鍔?NUMA 骞宠　銆?|
| NUMA_MIGRATION | bool | 鏀寔灏嗛〉杩佺Щ鍒板叾浠?NUMA 鑺傜偣锛岄€氳繃 migrate_pages()銆乵ove_pages() 涓?mbind() 绛夋帴鍙ｅ鐢ㄦ埛绌洪棿鍙敤銆傞€夋嫨璇ラ€夐」涔熷惎鐢ㄥ椤?..鐨勬敮鎸併€?|
| OBJTOOL | bool | 鍦?objtool 璀﹀憡鏃朵娇鏋勫缓澶辫触銆俹bjtool 璀﹀憡鍙兘鎸囩ず鍐呮牳涓嶇ǔ瀹氾紝鍖呮嫭鍚姩澶辫触銆傚己鐑堝缓璁惎鐢ㄨ閫夐」銆傚鏋滀笉纭畾锛岄€夋嫨 Y銆?|
| OF_RECONFIG_NOTIFIER_ERROR_INJECT | tristate | 璇ラ€夐」鎻愪緵鍚?OF 閲嶉厤缃€氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠閫氳繃 /sys/kernel/debug/notifier-error-inject/OF-re...涓嬬殑 debugfs 鎺ュ彛鎺у埗銆?|
| OVERFLOW_KUNIT_TEST | tristate | 鏋勫缓閽堝 check_*_overflow()銆乻ize_*()銆佸垎閰嶅強鐩稿叧鍑芥暟鐨勫崟鍏冩祴璇曘€傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation 涓殑 KUnit 鏂囨。... |
| PACKING_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 packing 搴撶殑 KUnit 娴嬭瘯銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/ 涓殑 KUnit 鏂囨。銆傚鏈夌枒闂紝閫夋嫨 N銆?|
| PAGE_COUNTER | bool | 璇ラ€夐」榛樿鍚敤 "favordynmods" 鎸傝浇閫夐」锛屽畠闄嶄綆浜嗕换鍔¤縼绉讳笌鎺у埗鍣ㄥ紑鍏崇瓑鍔ㄦ€?cgroup 淇敼鐨勫欢杩燂紝浠ｄ环鏄娇鐑?.. |
| PAGE_IDLE_FLAG | bool | 杩欏悜 'struct page' 娣诲姞 PG_idle 涓?PG_young 鏍囧織銆侾TE Accessed 浣嶇殑鍐欏叆鑰呭彲浠ヨ缃爣蹇椾腑璇ヤ綅鐨勭姸鎬侊紝浣?PTE Accessed 浣嶇殑璇诲彇鑰呴伩鍏嶅共鎵般€?|
| PAGE_MAPCOUNT | def_bool | 璇ラ€夐」鍚敤杩炵画鍐呭瓨鍒嗛厤鍣紙CMA锛夛紝瀹冨厑璁稿叾浠栧瓙绯荤粺鍒嗛厤澶х殑鐗╃悊杩炵画鍐呭瓨鍧椼€侰MA 淇濈暀涓€鍧楀唴瀛樺尯鍩燂紝骞跺彧鍏佽鍙Щ鍔ㄩ〉... |
| PAHOLE_HAS_BTF_TAG | def_bool | 鍐冲畾 pahole 鏄惁鍙戝嚭 btf_tag 灞炴€э紙btf_type_tag 涓?btf_decl_tag锛夈€傜洰鍓嶅彧鏈?clang 缂栬瘧鍣ㄥ疄鐜颁簡杩欎簺灞炴€э紝鍥犳浣胯閰嶇疆渚濊禆浜?CC_IS_CLANG銆?|
| PAHOLE_HAS_LANG_EXCLUDE | def_bool | 鏀寔 --lang_exclude 鏍囧織锛屼娇 pahole 鎺掗櫎鎵€鎻愪緵璇█鐨勭紪璇戝崟鍏冦€傚湪 Kbuild 涓敤浜庣渷鐣?pahole 1.24 鐗堟湰涓嶆敮鎸佺殑 Rust 缂栬瘧鍗曞厓锛屾澶?.. |
| PANIC_ON_OOPS | bool | 鍦ㄦ閫夋嫨 Y 浠ヨ鍐呮牳鍦?oops 鏃舵亹鎱屻€傚叾鏁堟灉绛夊悓浜庡湪鍐呮牳鍛戒护琛岃缃?oops=panic銆傝鐗规€ф湁鍔╀簬纭繚鍐呮牳涓嶅仛浠讳綍... |
| PANIC_TIMEOUT | int | 璁剧疆鍐呮牳鎭愭厡鍚庣洿鍒板彂鐢熼噸鍚殑瓒呮椂鍊硷紙绉掞級銆傝嫢 n = 0锛屽垯姘歌繙绛夊緟銆傝秴鏃跺€?n > 0 灏嗗湪閲嶅惎鍓嶇瓑寰?n 绉掞紝鑰岃秴鏃跺€?n... |
| PC104 | bool | Expose PC/104 form factor 璁惧椹卞姩绋嬪簭 and options available for selection and configuration. Enable this option if your target machine has a PC/104 bus. |
| PCPU_DEV_REFCNT | bool | 鑻ヨ缃閫夐」锛岀綉缁滆澶囧紩鐢ㄨ鏁颁娇鐢ㄦ瘡 CPU 鍙橀噺銆傚彲寮哄埗璁句负 N 浠ユ娴嬩笅婧紙浠ｄ环鏄€ц兘涓嬮檷锛夈€?|
| PCSPKR_PLATFORM | bool | 璇ラ€夐」鍏佽绂佺敤鍐呴儴 PC 鎵０鍣ㄦ敮鎸侊紝鑺傜渷涓€浜涘唴瀛樸€?|
| PERCPU_STATS | bool | 璇ョ壒鎬ч€氳繃 debugfs 鏀堕泦骞舵毚闇茬粺璁′俊鎭€備俊鎭寘鎷叏灞€涓庢瘡鍧楃粺璁★紝鍙敤浜庡府鍔╃悊瑙?percpu 鍐呭瓨浣跨敤銆?|
| PERCPU_TEST | tristate | 鍚敤璇ラ€夐」浠ユ瀯寤洪獙璇?per-cpu 鎿嶄綔鐨勬祴璇曟ā鍧椼€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| PERF_EVENTS | bool | 鍚敤鍐呮牳瀵硅蒋浠朵笌纭欢鎻愪緵鐨勫悇绉嶆€ц兘浜嬩欢鐨勫唴鏍告敮鎸併€傝蒋浠朵簨浠惰涔堝唴寤烘敮鎸侊紝瑕佷箞閫氳繃閫氱敤璺熻釜鐐规敮鎸併€傚ぇ澶氭暟鐜颁唬 CPU 鏀寔... |
| PHYS_ADDR_T_64BIT | def_bool | 鍚敤鍐呮牳鍚岄〉鍚堝苟锛圞SM锛夛細KSM 瀹氭湡鎵弿搴旂敤绋嬪簭鍦板潃绌洪棿涓偅浜涘簲鐢ㄥ缓璁彲鑳藉彲鍚堝苟鐨勫尯鍩熴€傚綋瀹冩壘鍒板唴瀹圭浉鍚岀殑椤垫椂锛屼細灏嗗叾鏇挎崲... |
| PID_NS | bool | 鏀寔杩涚▼ ID 鍛藉悕绌洪棿銆傚彧瑕佸浜庝笉鍚岀殑 pid 鍛藉悕绌洪棿涓紝杩欏氨鍏佽瀛樺湪澶氫釜鍏锋湁鐩稿悓 pid 鐨勮繘绋嬨€傝繖鏄鍣ㄧ殑鍩烘湰鏋勪欢銆?|
| PM_NOTIFIER_ERROR_INJECT | tristate | 璇ラ€夐」鎻愪緵鍚?PM 閫氱煡閾惧洖璋冩敞鍏ヤ汉涓洪敊璇殑鑳藉姏銆傚畠閫氳繃 /sys/kernel/debug/notifier-error-inject/pm 涓嬬殑 debugfs 鎺ュ彛鎺у埗銆傚鏋滈€氱煡... |
| POSIX_MQUEUE_SYSCTL | bool | 杩欐槸鍐呮牳閫氳繃灏嗗叾鎷兼帴杩涚閬撴潵鍚戠敤鎴风┖闂翠紶閫掍簨浠剁殑閫氱敤閫氱煡闃熷垪銆傚畠鍙互涓庣敤浜庡瘑閽?瀵嗛挜鐜彉鏇撮€氱煡涓庤澶?..鐨勭洃瑙嗗櫒閰嶅悎浣跨敤銆?|
| POSIX_TIMERS | bool | 杩欏寘鍚 POSIX 瀹氭椂鍣ㄧ殑鍐呮牳鍘熺敓鏀寔銆備竴浜涘祵鍏ュ紡绯荤粺鐢ㄤ笉鍒板畠浠紝鍥犳鍙互閰嶇疆鎺変互鍑忓皬鍐呮牳闀滃儚浣撶Н銆傚綋璇ラ€夐」... |
| PREEMPT_NOTIFIERS | bool | 鏋勫缓涓€涓畝鍗曠殑 ASN.1 璇硶缂栬瘧鍣紝瀹冧骇鐢熷彲琚?ASN.1 娴佽В鐮佸櫒瑙ｉ噴鐨勫瓧鑺傜爜杈撳嚭锛屽苟鐢ㄤ簬鍛婄煡瑙ｇ爜鍣ㄥ湪娴佷腑棰勬湡鍝簺鏍囩浠ュ強... |
| PRIME_NUMBERS_KUNIT_TEST | tristate | 璇ラ€夐」鍚敤閽堝 {is,next}_prime_number 鍑芥暟鐨?KUnit 娴嬭瘯濂椾欢銆傚惎鐢ㄨ閫夐」灏嗗寘鍚皢绱犳暟鐢熸垚鍣ㄥ嚱鏁颁笌鏆村姏瀹炵幇杩涜姣旇緝鐨勬祴璇?.. |
| PRINTK | bool | 璇ラ€夐」鍚敤姝ｅ父鐨?printk 鏀寔銆傜Щ闄ゅ畠浼氭秷闄ゅ唴鏍搁暅鍍忎腑澶ч儴鍒嗘秷鎭瓧绗︿覆锛屼娇鍐呮牳鎴栧鎴栧皯娌夐粯銆傜敱浜庤繖浣垮緱璇婃柇...闈炲父鍥伴毦銆?|
| PRINTK_CALLER | bool | 閫夋嫨璇ラ€夐」浼氫娇 printk() 鍚戞瘡鏉℃秷鎭坊鍔犺皟鐢ㄨ€?绾跨▼ ID"锛堣嫢澶勪簬浠诲姟涓婁笅鏂囷級鎴栬皟鐢ㄨ€?澶勭悊鍣?ID"锛堣嫢涓嶅湪浠诲姟涓婁笅鏂囷級銆傝閫夐」鐢ㄤ簬...鐜銆?|
| PRINTK_EXECUTION_CTX | bool | 璇ラ€夐」鎵╁睍 struct printk_info 浠ュ湪 printk 涓寘鍚澶栫殑鎵ц涓婁笅鏂囷紝渚嬪娑堟伅鏉ユ簮鐨勮繘绋嬪悕涓?CPU 缂栧彿銆傝繖瀵逛簬鍏宠仈 printk 娑堟伅...寰堟湁鐢ㄣ€?|
| PRINTK_INDEX | bool | 娣诲姞瀵圭紪璇戞椂宸茬煡鐨勫叏閮?printk 鏍煎紡鍦?<debugfs>/printk/index/<module> 澶勫缓绔嬬储寮曠殑鏀寔銆傝繖鍙綔涓虹淮鎶ょ洃瑙?/dev/kmsg 鐨勫畧鎶よ繘绋嬬殑涓€閮ㄥ垎锛屽洜涓哄畠鍏佽瀹¤... |
| PRINTK_RINGBUFFER_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 printk 鐜舰缂撳啿鍖?KUnit 娴嬭瘯濂椾欢銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 KUnit 鏂囨。銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| PRINTK_TIME | bool | 閫夋嫨璇ラ€夐」浼氫娇 printk() 娑堟伅鐨勬椂闂存埑琚坊鍔犲埌 syslog() 绯荤粺璋冪敤鐨勮緭鍑轰笌鎺у埗鍙颁笂銆傛椂闂存埑鎬绘槸鍦ㄥ唴閮ㄨ褰曪紝骞跺鍑?.. |
| PROC_MEM_ALWAYS_FORCE | bool | 璇ラ€夐」鍏佽 /proc/pid/mem 璁块棶鍦ㄦ嫢鏈?ptrace 璁块棶鏉冮檺鏃惰鐩栧唴瀛樻槧灏勬潈闄愩€?|
| PROC_MEM_FORCE_PTRACE | bool | 璇ラ€夐」鍏佽 /proc/pid/mem 璁块棶涓哄儚 gdb 杩欐牱鐨勬椿鍔?ptracer 瑕嗙洊鍐呭瓨鏄犲皠鏉冮檺銆?|
| PROC_MEM_NO_FORCE | bool | 姘歌繙涓嶈瑕嗙洊鍐呭瓨鏄犲皠鏉冮檺銆?|
| PROC_PID_CPUSET | bool | 鎻愪緵涓€涓?cgroup 鎺у埗鍣紝瀹炵幇 cgroup 涓繘绋嬪彲浠?mknod 鎴栨墦寮€鐨勮澶囩殑鐧藉悕鍗曘€?|
| PROFILING | bool | 鍦ㄦ閫夋嫨 Y 浠ュ惎鐢ㄥ垎鏋愬櫒浣跨敤鐨勬墿灞曟€ц兘鍒嗘瀽鏀寔鏈哄埗銆?|
| PROVE_RAW_LOCK_NESTING | bool | 鍚敤 raw_spinlock 涓?spinlock 宓屽妫€鏌ワ紝浠ョ‘淇濅笉杩濆弽涓?PREEMPT_RT 鍚敤鐨勫唴鏍哥殑閿佸祵濂楄鍒欍€?|
| PROVIDE_OHCI1394_DMA_INIT | bool | 濡傛灉浣犳兂璋冭瘯鍦ㄥ惎鍔ㄦ棭鏈熸寕璧锋垨宕╂簝鍐呮牳鐨勯棶棰橈紝涓斿穿婧冪殑鏈哄櫒鏈?FireWire 绔彛锛屼綘鍙互浣跨敤姝ょ壒鎬ц繙绋嬭闂穿婧冩満鍣ㄧ殑鍐呭瓨... |
| PSI | bool | 鏀堕泦鎸囩ず绯荤粺涓?CPU銆佸唴瀛樹笌 IO 瀹归噺杩囬噺鎻愪氦绋嬪害鐨勬寚鏍囥€傚鏋滀綘鍦ㄦ閫夋嫨 Y锛屽唴鏍稿皢鍒涘缓 /proc/pressure/锛屽叾涓寘鍚帇鍔涚粺璁℃枃浠?cpu... |
| PSI_DEFAULT_DISABLED | bool | 鑻ヨ缃紝鍘嬪姏鍋滈】淇℃伅璺熻釜榛樿绂佺敤锛屼絾鍙湪鍚姩鏃堕€氳繃鍐呮牳鍛戒护琛屼紶鍏?psi=1 鍚敤銆傝鐗规€у悜浠诲姟鍞ら啋...娣诲姞涓€浜涗唬鐮併€?|
| PTE_MARKER_UFFD_WP | bool | 鍏佽涓?userfaultfd 鍐欎繚鎶ょ洰鐨勫垱寤烘爣璁?PTE銆傚湪鍩轰簬鏂囦欢鐨?shmem 涓?hugetlbfs 绛夊唴瀛樼被鍨嬩笂鍚敤 userfaultfd 鍐欎繚鎶ゆ椂闇€瑕佸畠銆?|
| RANDOM_KMALLOC_CACHES | bool | 涓€绉嶅姞鍥虹壒鎬э紝涓烘甯哥殑 kmalloc 鍒嗛厤鍒涘缓 slab 缂撳瓨鐨勫涓壇鏈紝骞朵娇 kmalloc 鍩轰簬浠ｇ爜鍦板潃闅忔満閫夊彇鍏朵腑涔嬩竴锛屼娇鏀诲嚮鑰呮洿闅?.. |
| RANDSTRUCT_KUNIT_TEST | tristate | 鏋勫缓鐢ㄤ簬妫€鏌?CONFIG_RANDSTRUCT=y锛堥殢鏈哄寲缁撴瀯浣撳竷灞€锛夌殑鍗曞厓娴嬭瘯銆?|
| RATELIMIT_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 "test_ratelimit" 妯″潡锛屽簲鐢ㄤ簬閫熺巼闄愬埗鐨勫苟鍙戞祴璇曚笌姝ｇ‘鎬ч獙璇併€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| RATIONAL_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓鏈夌悊鏁板鍗曞厓娴嬭瘯銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/kunit/ 涓殑 KUnit 鏂囨。銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| RBTREE_TEST | tristate | 涓€涓祴閲?rbtree 搴撴€ц兘鐨勫熀鍑嗘祴璇曘€傝繕鍖呮嫭 rbtree 涓嶅彉寮忔鏌ャ€?|
| READABLE_ASM | bool | 绂佺敤鏌愪簺鍊惧悜浜庣敓鎴愪汉绫婚毦浠ラ槄璇荤殑姹囩紪杈撳嚭鐨勭紪璇戝櫒浼樺寲銆傝繖鍙兘浣垮唴鏍哥◢鎱紝浣嗘湁鍔╀簬闇€瑕佺粡甯哥洴鐫€...鐨勫唴鏍稿紑鍙戣€呫€?|
| READ_ONLY_THP_FOR_FS | bool | 鍏佽 khugepaged 灏嗗彧璇荤殑鍩轰簬鏂囦欢鐨勯〉鏀惧叆 THP銆傚畠琚爣璁颁负瀹為獙鎬э紝鍥犱负杩欐槸涓€涓柊鐗规€с€傛枃浠?THP 鐨勫啓鏀寔灏嗗湪鎺ヤ笅鏉ョ殑鍑犱釜鍙戝竷鍛ㄦ湡涓紑鍙戙€?|
| REED_SOLOMON_TEST | tristate | 璇ラ€夐」鍦ㄥ惎鍔ㄦ椂鎴栨ā鍧楀姞杞芥椂鍚敤 rslib 鐨勮嚜娴嬪嚱鏁般€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| RELAY | bool | 璇ラ€夐」鍦ㄦ煇浜涙枃浠剁郴缁燂紙渚嬪 debugfs锛変腑鍚敤瀵?relay 鎺ュ彛鐨勬敮鎸併€傚畠鏃ㄥ湪涓哄伐鍏蜂笌璁炬柦鎻愪緵楂樻晥鐨勬満鍒舵潵涓浆澶ч噺... |
| RESOURCE_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓璧勬簮 API 鍗曞厓娴嬭瘯銆傛祴璇?resource.c 涓?ioport.h 鎻愪緵鐨?API 閫昏緫銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 KUnit 鏂囨。... |
| RFS_ACCEL | bool | 鍏佽鍏锋湁娴佽繃婊よ〃鐨勫涓槦鍒楃‖浠剁殑椹卞姩鍔犻€?RFS銆?|
| RPS | bool | 杞欢鎺ユ敹渚ф暟鎹寘瀵煎悜锛圧PS锛夊皢鎺ユ敹鏁版嵁鍖呭鐞嗙殑璐熻浇鍒嗗竷鍒板涓?CPU 涓娿€?|
| RSEQ | bool | 鍚敤鍙噸鍚簭鍒楃郴缁熻皟鐢ㄣ€傚畠鎻愪緵涓€涓敤鎴风┖闂村綋鍓?CPU 缂栧彿鍊肩殑缂撳瓨锛屽姞閫熶粠鐢ㄦ埛绌洪棿鑾峰彇褰撳墠 CPU 缂栧彿锛屼互鍙婁竴涓?..鐨?ABI銆?|
| RSEQ_DEBUG_DEFAULT_ENABLE | bool | 璇ラ€夐」涓哄彲閲嶅惎搴忓垪鐨勮皟璇曟ā寮忓惎鐢ㄩ潤鎬佸垎鏀€傚畠涔熷彲浠ラ€氳繃鍐呮牳鍛戒护琛屽弬鏁?"rseq_debug=0/1" 涓?debugfs 鎺у埗銆傚鏋?.. |
| RSEQ_SLICE_EXTENSION | bool | 鍏佽鐢ㄦ埛绌洪棿鍦ㄩ€氳繃 RSEQ 鍏变韩鏁版嵁 ABI 浠庝腑鏂繑鍥炲埌鐢ㄦ埛绌洪棿鏃惰姹傛湁闄愮殑鏃堕殭鎵╁睍銆傝嫢琚巿浜堬紝杩欏厑璁稿畬鎴愪竴涓复鐣屽尯锛屼粠鑰?.. |
| RSEQ_STATS | bool | 鍚敤杞婚噺绾ц鏁板櫒锛岄€氳繃 debugfs 鏆撮湶鍏充簬 RSEQ 鎿嶄綔棰戠巼鐨勪俊鎭€備富瑕佺敤浜庡唴鏍歌皟璇曟垨鎬ц兘鍒嗘瀽銆傝櫧鐒舵槸杞婚噺绾х殑锛屼絾瀹冧粛... |
| RT_GROUP_SCHED | bool | 璇ョ壒鎬ц浣犳樉寮忓湴涓轰换鍔＄粍鍒嗛厤鐪熷疄鐨?CPU 甯﹀銆傝嫢鍚敤锛屽湪浣犱负鏅€氱敤鎴峰垎閰嶅疄鏃跺甫瀹戒箣鍓嶏紝灏嗘棤娉曡皟搴﹀疄鏃朵换鍔?.. |
| RT_GROUP_SCHED_DEFAULT_DISABLED | bool | 璁剧疆鏃讹紝RT 缁勮皟搴﹂粯璁ょ鐢ㄣ€傝閫夐」浠ュ弽鐩稿舰寮忓瓨鍦紝浠ヤ究鍗曠函鐨?RT_GROUP_SCHED 鍗冲惎鐢ㄧ粍璋冨害銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| RUNTIME_TESTING_MENU | bool | 鍚敤璇ラ€夐」浠ュ寘鍚?Dhrystone 2.1 鍩哄噯娴嬭瘯銆傝娴嬭瘯璁＄畻姣忕 Dhrystone 鏁帮紝浠ュ強褰?Dhrystone 鍒嗘暟闄や互...鏃惰幏寰楃殑 DMIPS锛圖hrystone MIPS锛夋暟銆?|
| RUST | bool | 鍦ㄥ唴鏍镐腑鍚敤 Rust 鏀寔銆傝繖鍏佽閫夋嫨鍏朵粬 Rust 鐩稿叧閫夐」锛屼緥濡傜敤 Rust 缂栧啓鐨勯┍鍔ㄣ€傝鑳藉鍔犺浇鐢?Rust 缂栧啓鐨勫閮ㄥ唴鏍告ā鍧椾篃闇€瑕佸畠... |
| RUSTC_LLVM_VERSION | int | 杩欒〃鏄?Rust 涓?Clang 鏄惁浣跨敤鐩稿悓涓荤増鏈殑 LLVM銆傛秹鍙婂鐞?LLVM IR 鎴栦綅鐮侊紙渚嬪璺ㄨ瑷€ LTO锛夌殑鎿嶄綔闇€瑕佺浉鍚岀殑 LLVM 涓荤増鏈墠鑳芥甯稿伐浣?.. |
| RUSTC_VERSION_TEXT | string | 璇峰弬闃?`CC_VERSION_TEXT`銆?|
| RUST_BUILD_ASSERT_ALLOW | bool | 鎺у埗鏋勫缓鏈熼棿濡備綍澶勭悊 `build_error!` 涓?`build_assert!`銆傚鏋滀簩杩涘埗涓瓨鍦ㄥ瀹冧滑鐨勮皟鐢紝鍙兘琛ㄦ槑涓€涓杩濆弽鐨勪笉鍙橀噺锛屾垨浼樺寲鍣ㄦ湭鑳介獙璇佽... |
| RUST_DEBUG_ASSERTIONS | bool | 鍚敤 rustc 鐨?`-Cdebug-assertions` 浠ｇ爜鐢熸垚閫夐」銆傝鏍囧織璁╀綘寮€鍚垨鍏抽棴 `cfg(debug_assertions)` 鏉′欢缂栬瘧銆傝繖鍙敤浜庡湪寮€鍙戞椂鍚敤棰濆鐨勮皟璇曚唬鐮?.. |
| RUST_INLINE_HELPERS | bool | 浣跨敤閾炬帴鏃朵紭鍖栵紙LTO锛夊皢 C 杈呭姪鍑芥暟鍐呰仈鍒?Rust 浠ｇ爜涓€傝嫢鍚敤璇ラ€夐」锛宺ust/helpers/ 涓０鏄庣殑 C 杈呭姪鍑芥暟浼氳鍐呰仈鍒?Rust 浠ｇ爜涓紝杩欐湁鍔╀簬...鐨勬€ц兘銆?|
| RUST_IS_AVAILABLE | def_bool | 杩欒〃鏄庢槸鍚︽湁鍚堥€傜殑 Rust 宸ュ叿閾惧彲鐢紙宸叉壘鍒帮級銆傛弧瓒?Rust 鏀寔鐨勬瀯寤鸿姹傜殑鏂规硶璇峰弬闃?Documentation/rust/quick-start.rst銆傜壒鍒?.. |
| RUST_KERNEL_DOCTESTS | bool | 璇ラ€夐」灏?`kernel` crate 鐨勬枃妗ｆ祴璇曟瀯寤轰负 KUnit 娴嬭瘯銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-tools/...涓殑 KUnit 鏂囨。銆?|
| RUST_OVERFLOW_CHECKS | bool | 鍚敤 rustc 鐨?`-Coverflow-checks` 浠ｇ爜鐢熸垚閫夐」銆傝鏍囧織鍏佽浣犳帶鍒惰繍琛屾椂鏁存暟婧㈠嚭鐨勮涓恒€傚綋鍚敤婧㈠嚭妫€鏌ユ椂锛屾孩鍑哄皢鍙戠敓 Rust 鎭愭厡銆?.. |
| SCANF_KUNIT_TEST | tristate | 鍚敤璇ラ€夐」浠ュ湪杩愯鏃舵祴璇?scanf 鍑芥暟銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| SCF_TORTURE_TEST | tristate | 璇ラ€夐」鎻愪緵涓€涓唴鏍告ā鍧楋紝瀵?smp_call_function() 绯诲垪鍘熻杩愯 torture 娴嬭瘯銆傚鏋滈渶瑕侊紝璇ュ唴鏍告ā鍧楀彲浠ュ湪琚祴鐨勬鍦ㄨ繍琛岀殑鍐呮牳涓婁簨鍚庢瀯寤?.. |
| SCHED_AUTOGROUP | bool | 璇ラ€夐」閫氳繃鑷姩鍒涘缓骞跺～鍏呬换鍔＄粍鏉ヤ负甯歌妗岄潰宸ヤ綔璐熻浇浼樺寲璋冨害鍣ㄣ€傝繖绉嶅伐浣滆礋杞界殑鍒嗙闅旂浜嗘縺杩涚殑 CPU 娑堣€楄€咃紙渚嬪鏋勫缓浠诲姟...锛夈€?|
| SCHED_INFO | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛岃皟搴﹀櫒鍙婄浉鍏充緥绋嬩腑灏嗘彃鍏ラ澶栦唬鐮侊紝浠ユ敹闆嗚皟搴﹀櫒琛屼负缁熻骞舵彁渚涘湪 /proc/schedstat 涓€傝繖浜涚粺璁″彲琚?.. |
| SCHED_PROXY_EXEC | bool | 璇ラ€夐」鍚敤浠ｇ悊鎵ц锛坧roxy execution锛夛紝涓€绉嶈鎸佹湁 mutex 鐨勪换鍔＄户鎵挎洿楂樹紭鍏堢骇绛夊緟鑰呰皟搴︿笂涓嬫枃鐨勬満鍒躲€?|
| SCHED_STACK_END_CHECK | bool | 璇ラ€夐」妫€鏌ュ schedule() 璋冪敤鏃剁殑鏍堟孩鍑恒€傚鏋滃彂鐜版爤鏈熬浣嶇疆琚鍐欙紝鍒欐€绘槸鎭愭厡锛屽洜涓鸿鎹熷潖鍖哄煙鐨勫唴瀹逛笉鍐嶅彲淇°€傝... |
| SECTION_MISMATCH_WARN_ONLY | bool | 濡傛灉浣犲湪姝ら€夋嫨 N锛屾瀯寤鸿繃绋嬪皢鍦ㄥ瓨鍦ㄤ换浣曡妭鍖轰笉鍖归厤鏃跺け璐ワ紝鑰屼笉浠呬粎鏄姏鍑鸿鍛娿€傚鏋滀笉纭畾锛岄€夋嫨 Y銆?|
| SECURITY | bool | 璇ラ€夐」鍏佽浣犻€夋嫨涓嶅悓鐨勫畨鍏ㄦā鍧楅厤缃繘鍐呮牳銆傚鏋滄湭閫夋嫨璇ラ€夐」锛屽皢浣跨敤榛樿 Linux 瀹夊叏妯″瀷銆傚鏋滀綘涓嶇‘瀹氬浣曞洖绛?.. |
| SECURITY_COMMONCAP_KUNIT_TEST | bool | This builds the commoncap KUnit tests. KUnit tests run during boot and output the results to the debug log in TAP format (https://testanything.org/). Only useful for kernel devs running KUnit test ... |
| SECURITY_DMESG_RESTRICT | bool | 璇ラ€夐」寮哄埗瀵规湭鐗规潈鐢ㄦ埛閫氳繃 dmesg(8) 璇诲彇鍐呮牳 syslog 鐨勯檺鍒躲€傚鏋滄湭閫夋嫨璇ラ€夐」锛岄櫎闈?dmesg_restrict sysctl 琚樉寮?..锛屽惁鍒欎笉浼氬疄鏂介檺鍒躲€?|
| SECURITY_INFINIBAND | bool | 璇ラ€夐」鍚敤 Infiniband 瀹夊叏閽╁瓙銆傝嫢鍚敤锛屽畨鍏ㄦā鍧楀彲浠ヤ娇鐢ㄨ繖浜涢挬瀛愬疄鐜?Infiniband 璁块棶鎺у埗銆傚鏋滀綘涓嶇‘瀹氬浣曞洖绛旓紝閫夋嫨 N銆?|
| SECURITY_NETWORK | bool | 璇ラ€夐」鍚敤濂楁帴瀛椾笌缃戠粶鐨勫畨鍏ㄩ挬瀛愩€傝嫢鍚敤锛屽畨鍏ㄦā鍧楀彲浠ヤ娇鐢ㄨ繖浜涢挬瀛愬疄鐜板鎺ュ瓧涓庣綉缁滅殑璁块棶鎺у埗銆傚鏋滀綘涓嶇‘瀹氬浣曞洖绛?.. |
| SECURITY_NETWORK_XFRM | bool | 璇ラ€夐」鍚敤 XFRM锛圛PSec锛夌綉缁滅殑瀹夊叏閽╁瓙銆傝嫢鍚敤锛屽畨鍏ㄦā鍧楀彲浠ヤ娇鐢ㄨ繖浜涢挬瀛愬熀浜庝粠 IPSec 绛栫暐娲剧敓鐨勬爣绛惧疄鐜伴€愬寘璁块棶鎺у埗銆傞潪 IP... |
| SECURITY_PATH | bool | 璇ラ€夐」鍚敤鍩轰簬璺緞鍚嶈闂帶鍒剁殑瀹夊叏閽╁瓙銆傝嫢鍚敤锛屽畨鍏ㄦā鍧楀彲浠ヤ娇鐢ㄨ繖浜涢挬瀛愬疄鐜板熀浜庤矾寰勫悕鐨勮闂帶鍒躲€傚鏋滀綘涓嶇‘瀹氬浣曞洖绛?.. |
| SELECT_MEMORY_MODEL | def_bool | 璇ラ€夐」鍏佽浣犳洿鏀?Linux 鍐呴儴绠＄悊鍏跺唴瀛樼殑涓€浜涙柟寮忋€傚ぇ澶氭暟鐢ㄦ埛鍙細鐢变綋绯荤粨鏋勯厤缃€変腑鍏朵腑涓€涓€夐」銆傝繖鏄甯哥殑銆?|
| SEQ_BUF_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 seq_buf 搴撶殑鍗曞厓娴嬭瘯銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| SGETMASK_SYSCALL | bool | sys_sgetmask 涓?sys_ssetmask 鏄凡搴熷純鐨勭郴缁熻皟鐢紝libc 涓嶅啀鏀寔锛屼絾鍦ㄦ煇浜涗綋绯荤粨鏋勪笂榛樿浠嶅惎鐢ㄣ€傚鏋滀笉纭畾锛屼繚鐣欒繖閲岀殑榛樿閫夐」銆?|
| SG_POOL | def_bool | 鎻愪緵涓€涓垎閰嶉摼寮忓垎鏁ｅ垪琛紙scatterlist锛夌殑杈呭姪鍑芥暟銆傚簲鐢卞笇鏈涘垎閰嶉摼寮?scatterlist 鐨勯┍鍔ㄦ垨 API 閫変腑銆? # sg 閾惧紡閫夐」 # |
| SHMEM | bool | shmem 鏄竴涓敤浜庣鐞嗗叡浜唴瀛樼殑鍐呴儴鏂囦欢绯荤粺銆傚畠鐢变氦鎹㈠尯鍚庡骞剁鐞嗚祫婧愰檺鍒躲€傝嫢鍚敤 TMPFS锛屽畠涔熶細浣滀负 tmpfs 瀵煎嚭鍒扮敤鎴风┖闂淬€傜鐢ㄨ閫夐」... |
| SHRINKER_DEBUG | bool | 鍦ㄦ閫夋嫨 Y 浠ュ惎鐢?shrinker 鐨?debugfs 鎺ュ彛锛屽畠鎻愪緵瀵瑰唴鏍稿唴瀛?shrinker 瀛愮郴缁熺殑鍙鎬с€傜鐢ㄥ畠浠ラ伩鍏嶉澶栫殑鍐呭瓨鍗犵敤銆?|
| SHUFFLE_PAGE_ALLOCATOR | bool | 椤靛垎閰嶅櫒鐨勯殢鏈哄寲鎻愰珮浜嗙洿鎺ユ槧灏勭殑鍐呭瓨渚х紦瀛樼殑骞冲潎鍒╃敤鐜囥€傝鍙傞槄 ACPI 6.2a 瑙勮寖涓 5.2.27 鑺傚紓鏋勫唴瀛樺睘鎬ц〃锛圚MAT锛?.. |
| SIGNALFD | bool | 鍚敤 signalfd() 绯荤粺璋冪敤锛屽厑璁稿湪鏂囦欢鎻忚堪绗︿笂鎺ユ敹淇″彿銆傚鏋滀笉纭畾锛岄€夋嫨 Y銆?|
| SIGNATURE | tristate | 鏁板瓧绛惧悕楠岃瘉銆傜洰鍓嶄粎鏀寔 RSA銆傚疄鐜颁娇鐢?GnuPG MPI 搴撱€?|
| SIPHASH_KUNIT_TEST | tristate | 鍚敤璇ラ€夐」浠ュ湪鍚姩鏃讹紙鎴栨ā鍧楀姞杞芥椂锛夋祴璇曞唴鏍哥殑 siphash锛?linux/siphash.h>锛夊搱甯屽嚱鏁般€傛棬鍦ㄥ府鍔╃紪鍐欎綋绯荤粨鏋勭壒瀹氱殑浼樺寲鐗堟湰銆傚鏋滀笉纭畾... |
| SLAB_BUCKETS | bool | 鍐呮牳鍫嗘敾鍑诲父甯镐緷璧栦簬鑳藉鍒涘缓鐢辩敤鎴锋帶鍒跺唴瀹广€佺壒瀹氬ぇ灏忕殑鍒嗛厤锛屼娇鍏朵笌鐩爣瀵硅薄鍒嗛厤鍒板悓涓€涓?kmalloc 妗朵腑銆備负... |
| SLAB_FREELIST_HARDENED | bool | 璁稿鍐呮牳鍫嗘敾鍑昏瘯鍥鹃拡瀵?slab 缂撳瓨鍏冩暟鎹笌鍏朵粬鍩虹璁炬柦銆傝閫夐」鍋氬嚭寰皬鐨勬€ц兘鐗虹壊锛屼互鍔犲浐鍐呮牳 slab 鍒嗛厤鍣ㄦ姷寰″父瑙佺殑绌洪棽閾捐〃鍒╃敤... |
| SLAB_FREELIST_RANDOM | bool | 闅忔満鍖栧垱寤烘柊椤垫椂浣跨敤鐨勭┖闂查摼琛ㄩ『搴忋€傝瀹夊叏鐗规€ч檷浣庡唴鏍?slab 鍒嗛厤鍣ㄥ鍫嗘孩鍑虹殑鍙娴嬫€с€?|
| SLAB_MERGE_DEFAULT | bool | 涓哄噺灏戝唴鏍稿唴瀛樼鐗囷紝褰?slab 缂撳瓨鍏变韩鐩稿悓澶у皬涓庡叾浠栫壒鎬ф椂鍙鍚堝苟銆傝繖甯︽潵鍐呮牳鍫嗘孩鍑鸿兘澶熻鍐欏璞?..鐨勯闄┿€?|
| SLAB_OBJ_EXT | bool | 璇ラ€夐」澧炲姞瀵逛竴缁勮繘绋嬭繘琛屽垎缁勭鐞嗙殑鏀寔锛岀敤浜庨厤鍚?Cpusets銆丆FS銆佸唴瀛樻帶鍒舵垨璁惧闅旂绛夎繘绋嬫帶鍒跺瓙绯荤粺銆傝鍙傞槄 - Documentation/scheduler/sc... |
| SLUB | def_bool | 浠ユ渶灏忓寲鍐呭瓨鍗犵敤鏂瑰紡閰嶇疆 slab 鍒嗛厤鍣紝鐗虹壊鍙墿灞曟€с€佽皟璇曚笌鍏朵粬鐗规€с€傝繖浠呯敤浜庢浘浣跨敤 SL...鐨勬渶灏忕郴缁熴€?|
| SLUB_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 SLUB 鍒嗛厤鍣ㄥ崟鍏冩祴璇曘€傛祴璇?SLUB 缂撳瓨璋冭瘯鍔熻兘銆傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄 Documentation/dev-...涓殑 KUnit 鏂囨。銆?|
| SLUB_STATS | bool | 杩欎簺缁熻淇℃伅鏈夊姪浜庤皟璇?slab 鍒嗛厤琛屼负锛屼互瀵绘壘浼樺寲鍒嗛厤鍣ㄧ殑鏂规硶銆傜敱浜庝繚鎸佺粺璁′細鎷栨參鏁翠綋...锛岀粷涓嶅簲鐢ㄤ簬鐢熶骇鐜銆?|
| SOCK_CGROUP_DATA | bool | 鎻愪緵涓€绉嶈浠诲姟浣跨敤鐩稿悓 id 鎿嶄綔涓嶅悓瀵硅薄鐨勬柟寮忋€備緥濡傦紝褰撳湪...涓娇鐢ㄦ椂锛岀浉鍚岀殑 IPC id 鍙兘寮曠敤涓嶅悓瀵硅薄锛屾垨鐩稿悓鐨勭敤鎴?id 鎴?pid 鍙兘寮曠敤涓嶅悓浠诲姟銆?|
| SOCK_RX_QUEUE_MAPPING | bool | 鐢ㄤ簬鍦ㄦ瘡鎺ュ彛鍩虹涓婁负杩涚▼鍒嗛厤缃戠粶浼樺厛绾х殑 cgroup 瀛愮郴缁熴€?|
| SOFTLOCKUP_DETECTOR_INTR_STORM | bool | 鍦ㄦ閫夋嫨 Y 浠ヨ鍐呮牳妫€娴?杞攣姝?鏈熼棿鐨処nterrupt Storm锛堜腑鏂鏆达級銆?杞攣姝?鍙敱澶氱鍘熷洜寮曡捣銆傝嫢鐢变腑鏂鏆村紩璧凤紝鍒欓鏆寸殑涓柇... |
| SPARSEMEM | def_bool | SPARSEMEM_VMEMMAP 浣跨敤铏氭嫙鏄犲皠鐨?memmap 鏉ヤ紭鍖?pfn_to_page 涓?page_to_pfn 鎿嶄綔銆傚湪鎷ユ湁鍏呰冻鍐呮牳璧勬簮鏃惰繖鏄渶楂樻晥鐨勯€夐」銆?|
| SPARSEMEM_MANUAL | bool | 杩欏皢鏄煇浜涚郴缁燂紙鍖呮嫭鍐呭瓨鐑彃鎷旂郴缁燂級鐨勫敮涓€閫夐」銆傝繖鏄甯哥殑銆傝閫夐」涓虹墿鐞嗗湴鍧€绌洪棿瀛樺湪绌烘礊鐨勭郴缁熸彁渚涢珮鏁堟敮鎸侊紝骞?.. |
| SPARSEMEM_VMEMMAP_PREINIT | bool | 鐑彃鎷斿唴瀛樼殑榛樿鍐呭瓨绫诲瀷銆傝閫夐」璁剧疆鍐呭瓨鐑彃鎷斾笂绾跨瓥鐣ワ紙/sys/devices/system/memory/auto_online_blocks锛夌殑榛樿绛栫暐锛屽喅瀹氬彂鐢?..鏃剁殑鎯呭喌銆?|
| STACKDEPOT_ALWAYS_INIT | bool | 鍦ㄥ惎鍔ㄦ棭鏈熷缁堝垵濮嬪寲鏍堜粨搴擄紙stack depot锛夈€?|
| STACKDEPOT_MAX_FRAMES | int | 杩愯杞婚噺绾ф帓闃熺殑鍚姩鏈熸祴璇曘€?|
| STACKINIT_KUNIT_TEST | tristate | 娴嬭瘯鍐呮牳鏄惁瀵规爤鍙橀噺涓庡～鍏呰繘琛岄浂鍒濆鍖栥€傝鐩栫巼鐢辩紪璇戝櫒鏍囧織 CONFIG_INIT_STACK_ALL_PATTERN 鎴?CONFIG_INIT_STACK_ALL_ZERO 鎺у埗銆?|
| STACKTRACE | bool | 璇ラ€夐」浣垮唴鏍镐负姣忎釜杩涚▼鍒涘缓 /proc/pid/stack锛屾樉绀哄叾褰撳墠鏍堣窡韪€傚畠涔熻鍚勭闇€瑕佹爤璺熻釜鐢熸垚鐨勫唴鏍歌皟璇曠壒鎬т娇鐢ㄣ€?|
| STACKTRACE_BUILD_ID | bool | 閫夋嫨璇ラ€夐」浼氫负浠?printk 鏍煎紡 '%p[SR]b' 鎵撳嵃鐨勬爤璺熻釜涓殑绗﹀彿娣诲姞鏋勫缓 ID 淇℃伅銆傝閫夐」鐢ㄤ簬涓嶆槗鑾峰彇 debuginfo 鐨勫彂琛岀増锛屼絾... |
| STACK_VALIDATION | bool | 鍦ㄧ紪璇戞椂鏍￠獙甯ф寚閽堣鍒欍€傝繖鏈夊姪浜庣‘淇濊繍琛屾椂鏍堣窡韪洿鍙潬銆傛洿澶氫俊鎭鍙傞槄 tools/objtool/Documentation/objtool.txt銆?|
| STATIC_USERMODEHELPER | bool | 榛樿鎯呭喌涓嬶紝鍐呮牳鍙互閫氳繃"鐢ㄦ埛鎬佽緟鍔╃▼搴?鍐呮牳鎺ュ彛璋冪敤璁稿涓嶅悓鐨勭敤鎴风┖闂翠簩杩涘埗绋嬪簭銆傚叾涓竴浜涗簩杩涘埗鍦ㄤ唬鐮佹垨...涓闈欐€佸畾涔夈€?|
| STATIC_USERMODEHELPER_PATH | string | 褰撲换浣曠敤鎴锋€佽緟鍔╃▼搴忓笇鏈涜繍琛屾椂锛屽唴鏍歌皟鐢ㄧ殑浜岃繘鍒躲€?鐪熸"搴旂敤绋嬪簭鐨勫悕绉板皢浣滀负绗竴涓弬鏁板湪鍛戒护琛屼笂浼犵粰璇ョ▼搴忋€傚鏋滀綘甯屾湜... |
| STRING_KUNIT_TEST | tristate | 鍚敤瀛楃涓插嚱鏁扮殑鎬ц兘娴嬮噺銆傚畠鍦?KUnit 娴嬭瘯杩愯鏈熼棿娴嬮噺瀛楃涓插嚱鏁扮殑鎵ц鏁堢巼銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| STRIP_ASM_SYMS | bool | 鍦ㄩ摼鎺ユ湡闂村墺绂绘眹缂栧櫒鍐呴儴鐢熸垚鐨勭鍙凤紙褰㈠ '.Lxxx' 鐨勭鍙凤級锛屼娇瀹冧滑涓嶄細姹℃煋 get_wchan() 绛夎緭鍑虹殑缁撴灉銆?|
| SYMBOLIC_ERRNAME | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛屽唴鏍哥殑 printf 瀹炵幇灏嗚兘澶熸墦鍗扮鍙峰寲鐨勯敊璇悕锛堜緥濡?ENOSPC锛夎€岄潪鏁板瓧 28銆傚畠浣垮唴鏍搁暅鍍忕暐澶э紙绾?3KB锛夛紝浣?.. |
| SYSCTL_ARCH_UNALIGN_ALLOW | bool | 鍚敤瀵?/proc/sys/kernel/unaligned-trap 鐨勬敮鎸併€傚厑璁镐綋绯荤粨鏋勫畾涔?浣跨敤 @unaligned_enabled 鏉ュ湪杩愯鏃跺垏鎹㈡湭瀵归綈璁块棶妯℃嫙銆傚弬鑰?arch/parisc/kernel/unaligned.c |
| SYSCTL_ARCH_UNALIGN_NO_WARN | bool | 鍚敤瀵?/proc/sys/kernel/ignore-unaligned-usertrap 鐨勬敮鎸併€傚厑璁镐綋绯荤粨鏋勫畾涔?浣跨敤 @no_unaligned_warning 浠ュ氨鍙兘璀﹀憡搴曞眰姝ｅ湪杩涜鐨勬湭瀵归綈璁块棶妯℃嫙銆?|
| SYSCTL_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 proc sysctl 鍗曞厓娴嬭瘯锛屽湪鍚姩鏃惰繍琛屻€傛祴璇?sysctl 鐨?API 濂戠害涓庡疄鐜版纭€с€傛湁鍏?KUnit 鍙婂崟鍏冩祴璇曠殑鏇村淇℃伅锛岃鍙傞槄... |
| SYSFS_SYSCALL | bool | sys_sysfs 鏄竴涓凡搴熷純鐨勭郴缁熻皟鐢紝libc 涓嶅啀鏀寔銆傛敞鎰忕鐢ㄨ閫夐」鏇村畨鍏紝浣嗗彲鑳界牬鍧忔煇浜涚郴缁熺殑鍏煎鎬с€傚鏋滀笉纭畾锛屽湪姝ら€夋嫨 N銆?|
| SYSTEM_DATA_VERIFICATION | def_bool | 浣跨敤绯荤粺鍙俊瀵嗛挜鐜殑鍐呭鎻愪緵鍏挜锛岃繘琛?PKCS#7 娑堟伅楠岃瘉銆傝繖闅忓悗鍙敤浜庢ā鍧楅獙璇併€乲exec 闀滃儚楠岃瘉涓庡浐浠?..銆?|
| SYSVIPC | bool | 杩涚▼闂撮€氫俊锛圛PC锛夋槸涓€濂楀簱鍑芥暟涓庣郴缁熻皟鐢紝璁╄繘绋嬶紙杩愯涓殑绋嬪簭锛夊悓姝ュ苟浜ゆ崲淇℃伅銆傝繖閫氬父琚涓烘槸涓€浠跺ソ浜?.. |
| SYSVIPC_SYSCTL | bool | POSIX 娑堟伅闃熷垪鏄?IPC 鐨勪竴閮ㄥ垎銆傚湪 POSIX 娑堟伅闃熷垪涓紝姣忎釜娑堟伅閮芥湁涓€涓紭鍏堢骇锛屽喅瀹氳繘绋嬫帴鏀跺畠鐨勯『搴忋€傚鏋滀綘鎯崇紪璇戝苟杩愯... |
| TASKSTATS | bool | 閫氳繃閫氱敤 netlink 鎺ュ彛瀵煎嚭閽堝浠诲姟/杩涚▼鐨勯€夊畾缁熻淇℃伅銆備笌 BSD 杩涚▼璁拌处涓嶅悓锛岃繖浜涚粺璁″湪浠诲姟/杩涚▼鐨勭敓鍛藉懆鏈熷唴浣滀负鍝嶅簲...鍙敤銆?|
| TASK_DELAY_ACCT | bool | 鏀堕泦浠诲姟绛夊緟绯荤粺璧勬簮锛堜緥濡?CPU銆佸悓姝ュ潡 I/O 瀹屾垚銆佷互鍙婇〉鎹㈠叆锛夋墍鑺辫垂鏃堕棿鐨勪俊鎭€傛绫荤粺璁℃湁鍔╀簬璁剧疆浠诲姟鐨勪紭鍏堢骇... |
| TASK_IO_ACCOUNTING | bool | 鏀堕泦璇ヤ换鍔″紩璧风殑瀛樺偍 I/O 瀛楄妭鏁扮殑淇℃伅銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TASK_XACCT | bool | 鏀堕泦鎵╁睍鐨勪换鍔¤璐︽暟鎹苟閫氳繃 taskstats 鎺ュ彛灏嗘暟鎹彂閫佸埌鐢ㄦ埛绌洪棿杩涜澶勭悊銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_BITOPS | tristate | 璇ラ€夐」鏋勫缓 "test_bitops" 妯″潡锛屼笌 TEST_LKM 妯″潡绫讳技锛屽彧鏄畠瀵?set/clear_bit 瀹忎笌 get_count_order/long 鍋氬熀鏈紨缁冿紝浠ョ‘淇濇病鏈夌紪璇?.. |
| TEST_BPF | tristate | 璇ラ€夐」鏋勫缓 "test_bpf" 妯″潡锛屾牴鎹綋鍓嶈缃 BPF 瑙ｉ噴鍣ㄦ垨 BPF JIT 缂栬瘧鍣ㄨ繍琛屽悇绉嶆祴璇曞悜閲忋€傝繖瀵?BPF JIT 缂栬瘧鍣?..灏ゅ叾鏈夌敤銆?|
| TEST_CLOCKSOURCE_WATCHDOG | tristate | 鍚敤璇ラ€夐」浠ュ垱寤轰竴涓唴鏍告ā鍧楋紝瑙﹀彂瀵规椂閽熸簮鐪嬮棬鐙楃殑娴嬭瘯銆傝妯″潡鍙€氳繃 modprobe 鎴?insmod 鍔犺浇锛屽姞杞芥椂鍗宠繍琛岋紝鎴?.. |
| TEST_DEBUG_VIRTUAL | tristate | 娴嬭瘯鍐呮牳妫€娴嬮拡瀵瑰唴鏍歌櫄鎷熷湴鍧€鏄犲皠闈炵嚎鎬ч儴鍒嗛敊璇皟鐢?virt_to_phys() 鐨勮兘鍔涖€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_DIV64 | tristate | 鍚敤璇ラ€夐」浠ュ紑鍚?'do_div()' 鍑芥暟娴嬭瘯銆傝娴嬭瘯浠呭湪绯荤粺鍚姩鏈熼棿鎵ц涓€娆★紙鍥犳鍙奖鍝嶅惎鍔ㄦ椂闂达級锛屾垨鍦ㄦā鍧楀姞杞芥椂鎵ц銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_DYNAMIC_DEBUG | tristate | 璇ユā鍧楁敞鍐屼竴涓窡韪櫒鍥炶皟锛岀粺璁?'do_debugging' 鍑芥暟涓凡鍚敤鐨?pr_debug 鏁伴噺锛岀劧鍚庢敼鍙樺叾鍚敤鐘舵€侊紝璋冪敤璇ュ嚱鏁板苟姣旇緝璁℃暟銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_FIRMWARE | tristate | 璇ラ€夐」鏋勫缓 "test_firmware" 妯″潡锛屽垱寤虹敤浜庢祴璇曞浐浠跺姞杞界殑鐢ㄦ埛绌洪棿鎺ュ彛銆傝繖鍙敤浜庢帶鍒跺浐浠跺姞杞界殑瑙﹀彂锛岃€屾棤闇€瀹為檯鐨勫浐浠?.. |
| TEST_FPU | tristate | 鍚敤璇ラ€夐」浠ユ坊鍔?/sys/kernel/debug/selftest_helpers/test_fpu锛屽畠灏嗚Е鍙戜竴绯诲垪娴偣杩愮畻銆傝繖鐢ㄤ簬鑷祴娴偣鎺у埗瀵勫瓨鍣ㄨ缃?.. |
| TEST_FREE_PAGES | tristate | 娴嬭瘯鏄惁涓嶄細鍥犻噴鏀句竴鍧楅〉涓庢姇鏈烘€ч〉寮曠敤涔嬮棿鐨勭珵浜夎€屽彂鐢熷唴瀛樻硠婕忋€傚鏋滀綘鐨勫唴鏍稿凡淇璇?Bug锛屽姞杞芥妯″潡鏄畨鍏ㄧ殑銆傚鏋?Bug 鏈?.. |
| TEST_HEXDUMP | tristate | 鍚敤璇ラ€夐」浠ュ湪杩愯鏃舵祴璇?printf 鍑芥暟銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_HMM | tristate | This is a pseudo 璁惧椹卞姩 solely for testing HMM. Say M here if you want to build the HMM test module. Doing so will allow you to run tools/testing/selftest/vm/hmm-tests. If unsure, say N. |
| TEST_IDA | tristate | Kunit test for miscdevice API, specially its behavior in respect to static and dynamic minor numbers. KUnit tests run during boot and output the results to the debug log in TAP format (https://test... |
| TEST_IOV_ITER | tristate | 鍚敤璇ラ€夐」浠ュ紑鍚 I/O 杩唬鍣紙iov_iter锛夋搷浣滅殑娴嬭瘯銆傝娴嬭瘯浠呭湪绯荤粺鍚姩鏈熼棿鎵ц涓€娆★紙鍥犳鍙奖鍝嶅惎鍔ㄦ椂闂达級锛屾垨鍦ㄦā鍧楀姞杞芥椂鎵ц銆傚鏋滀笉纭畾锛岄€夋嫨... |
| TEST_KALLSYMS_A | tristate | 閫夋嫨"蹇€?浠ュ鐨勯€夐」灏嗗惎鐢ㄤ細鎷栨參鏋勫缓骞跺彲鑳戒娇鏋勫缓宕╂簝鐨勬祴璇曘€?|
| TEST_KALLSYMS_FAST | bool | 浣犲苟涓嶄細鐪熸娴嬭瘯 kallsyms锛屾墍浠ヨ繖鍙槸鍦ㄤ娇鐢?allmodconfig 鏃跺府鍔╁揩閫熸瀯寤恒€?|
| TEST_KALLSYMS_LARGE | bool | 杩欏皢鍚敤鏇村鏁伴噺鐨勭鍙枫€傝繖灏嗘樉钁楁嫋鎱綘鐨勬瀯寤恒€?|
| TEST_KALLSYMS_MAX | bool | 杩欏皢鍚敤瀵煎嚭鍒版垜浠煡閬撲細寮€濮嬩娇鏋勫缓宕╂簝鐨勭▼搴︺€?|
| TEST_KALLSYMS_NUMSYMS | int | 鍦?TEST_KALLSYMS_A 涓婂垱寤虹殑绗﹀彿鏁伴噺锛屽叾涓彧鏈?TEST_KALLSYMS_B 妯″潡浼氫娇鐢ㄤ竴涓€傝繖涔熺敤浜?TEST_KALLSYMS_C 灏嗘嫢鏈夌殑绗﹀彿鏁伴噺锛屾寜 TEST_KALLS...缂╂斁銆?|
| TEST_KALLSYMS_SCALE_FACTOR | int | TEST_KALLSYSMS_C 姣?TEST_KALLSYMS_A 澶氬嚭鐨勬湭浣跨敤绗﹀彿鏁伴噺銆傝嫢涓?8锛屽垯妯″潡 C 灏嗘瘮妯″潡 A 澶?8 * syms 涓鍙枫€傜劧鍚?TEST_KALLSYMS_D 灏嗘嫢鏈夋瘮...澶氫竴鍊嶇殑绗﹀彿銆?|
| TEST_KEXEC_HANDOVER | bool | 璇ラ€夐」鍚敤瀵?Kexec HandOver锛圞HO锛夌殑娴嬭瘯銆傛祴璇曞寘鍚袱閮ㄥ垎锛氬湪 kexec 鍓嶄繚瀛樺唴鏍告暟鎹紝骞跺湪 kexec 鍚庢仮澶嶆暟鎹苟楠岃瘉鍏惰姝ｇ‘绉讳氦...銆?|
| TEST_KMOD | tristate | 娴嬭瘯鍐呮牳鐨勬ā鍧楀姞杞芥満鍒讹細kmod銆俴mod 瀹炵幇浣跨敤 Linux 鍐呮牳鐨勭敤鎴锋€佽緟鍔╃▼搴忓姞杞芥ā鍧楃殑鏀寔銆傝娴嬭瘯鎻愪緵涓€绯诲垪閽堝 kmod 鐨勬祴璇曘€傚敖绠℃妧鏈笂... |
| TEST_KSTRTOX | tristate | 鍚敤璇ラ€夐」浠ュ湪鍚姩鏃舵祴璇曚綅鍥惧嚱鏁般€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_LIST_SORT | tristate | 鍚敤璇ラ€夐」浠ュ紑鍚?'list_sort()' 鍑芥暟娴嬭瘯銆傝娴嬭瘯浠呭湪绯荤粺鍚姩鏈熼棿鎵ц涓€娆★紙鍥犳鍙奖鍝嶅惎鍔ㄦ椂闂达級锛屾垨鍦ㄦā鍧楀姞杞芥椂鎵ц銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_LKM | tristate | 璇ラ€夐」鏋勫缓 "test_module" 妯″潡锛屽姞杞芥椂閫氳繃 printk 杈撳嚭 "Hello, world"銆傚畠鏃ㄥ湪鐢ㄤ簬妯″潡鍔犺浇瀛愮郴缁燂紙渚嬪楠岃瘉妯″潡...锛夌殑鍩烘湰璇勪及銆?|
| TEST_LOCKUP | tristate | 璇ラ€夐」鏋勫缓 "test_lockup" 妯″潡锛屾湁鍔╀簬纭繚鐪嬮棬鐙椾笌閿佹妫€娴嬪櫒姝ｅ父宸ヤ綔銆傛牴鎹ā鍧楀弬鏁帮紝瀹冨彲浠ユā鎷熻蒋閿佹鎴栫‖閿佹銆?鎸傝捣浠诲姟...銆?|
| TEST_MEMCAT_P | tristate | 娴嬭瘯 memcat_p() 杈呭姪鍑芥暟鏄惁姝ｇ‘鍚堝苟涓や釜鎸囬拡鏁扮粍銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_MEMINIT | tristate | 娴嬭瘯鍐呮牳鏄惁瀵瑰爢涓庨〉鍒嗛厤杩涜闆跺垵濮嬪寲銆傝繖鍙敤浜庢祴璇?init_on_alloc 涓?init_on_free 鐗规€с€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_MULDIV64 | tristate | 鍚敤璇ラ€夐」浠ュ紑鍚?'mul_u64_u64_div_u64()' 鍑芥暟娴嬭瘯銆傝娴嬭瘯浠呭湪绯荤粺鍚姩鏈熼棿鎵ц涓€娆★紙鍥犳鍙奖鍝嶅惎鍔ㄦ椂闂达級锛屾垨鍦ㄦā鍧楀姞杞芥椂鎵ц銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_OBJAGG | tristate | 鍚敤璇ラ€夐」浠ュ湪鍚姩鏃讹紙鎴栨ā鍧楀姞杞芥椂锛夋祴璇曞璞¤仛鍚堢鐞嗗櫒銆?|
| TEST_OBJPOOL | tristate | 璇ラ€夐」鏋勫缓 "test_objpool" 妯″潡锛屽簲鐢ㄤ簬瀵硅薄鍒嗛厤涓庡洖鏀剁殑姝ｇ‘鎬т笌骞跺彂娴嬭瘯銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_PARMAN | tristate | 鍚敤璇ラ€夐」浠ュ湪鍚姩鏃讹紙鎴栨ā鍧楀姞杞芥椂锛夋祴璇曚紭鍏堢骇鏁扮粍绠＄悊鍣ㄣ€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_REF_TRACKER | tristate | 璇ラ€夐」鎻愪緵涓€涓娇鐢ㄥ紩鐢ㄨ窡韪櫒鍩虹璁炬柦鎵ц娴嬭瘯鐨勫唴鏍告ā鍧椼€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_RHASHTABLE | tristate | 鍚敤璇ラ€夐」浠ュ湪鍚姩鏃舵祴璇?rhashtable 鍑芥暟銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_RUNTIME | bool | 杩欏厑璁告垜浠€氳繃鐢ㄤ簬灏嗙鍙锋斁缃湪鍐呮牳 ELF kallsyms 涓庢ā鍧?kallsyms 涓婄殑 kallsyms 鏉ュ find_symbol() 杩涜鍘嬪姏娴嬭瘯锛屾垜浠湪鍏朵腑鏀剧疆瀵煎嚭鐨勭鍙风瓑鍐呮牳绗﹀彿銆傛垜浠凡... |
| TEST_SORT | tristate | 璇ラ€夐」鍦ㄥ惎鍔ㄦ椂锛堟垨妯″潡鍔犺浇鏃讹級鍚敤 'sort()' 鐨勮嚜娴嬪嚱鏁般€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_STATIC_KEYS | tristate | 娴嬭瘯闈欐€侀敭锛坰tatic key锛夋帴鍙ｃ€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_SYSCTL | tristate | 璇ラ€夐」鏋勫缓 "test_sysctl" 妯″潡銆傝椹卞姩鑳藉鍦ㄤ笉褰卞搷鍙兘鏀瑰彉绯荤粺鍔熻兘鐨勭敓浜у紑鍏崇殑鎯呭喌涓嬶紝瀹夊叏鍦版祴璇曢┍鍔ㄥ彲鐢ㄧ殑 proc sysctl 鎺ュ彛銆傚鏋?.. |
| TEST_UDELAY | tristate | 璇ラ€夐」鏋勫缓 "udelay_test" 妯″潡锛屾湁鍔╀簬纭繚 udelay() 姝ｅ父宸ヤ綔銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_VMALLOC | tristate | 璇ラ€夐」鏋勫缓 "test_vmalloc" 妯″潡锛屽簲鐢ㄤ簬鍘嬪姏涓庢€ц兘鍒嗘瀽銆傚洜姝わ紝鍙互浠庢€ц兘涓庣ǔ瀹氭€ц搴﹁瘎浼?vmalloc 瀛愮郴缁熺殑浠讳綍鏂板彉鏇?.. |
| TEST_WORKQUEUE | tristate | 璇ラ€夐」鏋勫缓 "test_workqueue" 妯″潡锛岀敤浜庡湪鏈夌珵浜夋椂鍩哄噯娴嬭瘯宸ヤ綔闃熷垪鍚炲悙銆傛湁鍔╀簬璇勪及浜插拰鎬ц寖鍥村彉鏇达紙渚嬪 cache_shard 涓?cache锛夈€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEST_XARRAY | tristate | 鍚敤璇ラ€夐」浠ュ湪鍚姩鏃舵垨妯″潡鍔犺浇鏃舵祴璇?maple tree 浠ｇ爜鍑芥暟銆傚惎鐢?"Debug Maple Trees" 灏嗗湪澶辫触鏃惰緭鍑烘洿璇︾粏鐨勯敊璇俊鎭€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| TEXTSEARCH | bool | 绠€鍗曘€佸彲宓屽叆鐨勫尯闂存爲銆傚彲浠ュ湪 log(n) 鏃堕棿鍐呮壘鍒伴噸鍙犺寖鍥寸殑璧风偣锛岀劧鍚庨亶鍘嗘墍鏈夐噸鍙犺妭鐐广€傝绠楁硶瀹炵幇涓轰竴涓寮虹殑 rbtree銆傝鍙傞槄锛欴... |
| THP_SWAP | def_bool | 鏁翠綋浜ゆ崲閫忔槑澶ч〉锛屼笉鎷嗗垎銆俋XX锛氱洰鍓嶏紝鍚庡閫忔槑澶ч〉鐨勪氦鎹㈢皣灏嗗湪鎹㈠嚭鍚庢媶鍒嗐€備緵鍏锋湁鍚堢悊 THP...鏀寔鐨勪綋绯荤粨鏋勯€夋嫨銆?|
| TIMERFD | bool | 鍚敤 timerfd() 绯荤粺璋冪敤锛屽厑璁稿湪鏂囦欢鎻忚堪绗︿笂鎺ユ敹瀹氭椂鍣ㄤ簨浠躲€傚鏋滀笉纭畾锛岄€夋嫨 Y銆?|
| TIME_NS | bool | 鍦ㄨ鍛藉悕绌洪棿涓紝鍚姩鏃堕棿涓庡崟璋冩椂閽熷彲浠ヨ璁剧疆銆傛椂闂村皢浠ョ浉鍚岀殑鑺傚缁х画鎺ㄨ繘銆?|
| TIME_NS_VDSO | def_bool | 鍦ㄨ鍛藉悕绌洪棿涓紝浠诲姟浣跨敤涓庝笉鍚屽懡鍚嶇┖闂翠腑涓嶅悓 IPC 瀵硅薄鐩稿搴旂殑 IPC id銆?|
| TMPFS | bool | Tmpfs 鏄竴涓皢鎵€鏈夋枃浠朵繚瀛樺湪铏氭嫙鍐呭瓨涓殑鏂囦欢绯荤粺銆倀mpfs 涓殑涓€鍒囬兘鏄复鏃剁殑锛屽嵆涓嶄細鍦ㄤ綘鐨勭‖鐩樹笂鍒涘缓鏂囦欢銆傛枃浠跺瓨鍦ㄤ簬鍐呭瓨涓庝氦鎹㈢┖闂?..銆?|
| TMPFS_INODE64 | bool | tmpfs 鍘嗗彶涓婂彧浣跨敤涓?unsigned int 涓€鏍峰鐨?inode 鍙枫€傚湪鏌愪簺鎯呭喌涓嬭繖鍙兘瀵艰嚧鍥炵粫锛屾綔鍦ㄥ湴瀵艰嚧鍗曚釜璁惧涓婂嚭鐜板涓叿鏈夌浉鍚?inode 鍙风殑鏂囦欢... |
| TMPFS_POSIX_ACL | bool | POSIX 璁块棶鎺у埗鍒楄〃锛圓CL锛夊湪鏍囧噯鎵€鏈夎€?缁?鍏朵粬鏂规涔嬪锛屼负鐢ㄦ埛涓庣粍鎻愪緵棰濆鐨勮闂潈闄愶紝璇ラ€夐」涓撻棬閫夋嫨瀵?tmpfs 鐨?ACL 鏀寔... |
| TMPFS_QUOTA | bool | 閰嶉鏀寔鍏佽璁剧疆姣忎釜鐢ㄦ埛涓庣粍鐨?tmpfs 浣跨敤闄愬埗銆傞€夋嫨 Y 浠ュ惎鐢ㄩ厤棰濇敮鎸併€備竴鏃﹀惎鐢紝浣犲彲浠ラ€氳繃 quota銆乽srquota 涓?grpquot...鎺у埗鐢ㄦ埛涓庣粍鐨勯厤棰濆疄鏂姐€?|
| TMPFS_XATTR | bool | 鎵╁睍灞炴€ф槸鐢卞唴鏍告垨鐢ㄦ埛涓?inode 鍏宠仈鐨?鍚嶇О:鍊?瀵癸紙璇︽儏璇峰弬闃?attr(5) 鎵嬪唽椤碉級銆傝繖鍚敤瀵?trusted.*銆乻ecurity.* 涓?user.* 鍚嶇О...鐨勬敮鎸併€?|
| TRACE_IRQFLAGS | bool | 鍚敤閽╁瓙浠ヤ腑鏂窡韪垨閿佽皟璇曠殑鍚敤涓庣鐢ㄣ€?|
| TRACE_IRQFLAGS_NMI | def_bool | 褰?CPU 鏈兘鍝嶅簲缁欏畾鐨?backtrace NMI 鏃跺惎鐢ㄨ皟璇曟墦鍗般€傝繖浜涙墦鍗版彁渚涗竴浜?CPU 鍙兘鍚堢悊鏈兘鍝嶅簲鐨勫師鍥狅紝渚嬪瀹冨浜庣绾跨姸鎬佹垨... |
| TRACE_MMIO_ACCESS | bool | 涓?MMIO 璇?鍐欐搷浣滃垱寤鸿窡韪偣銆傝繖浜涜窡韪簨浠跺彲鐢ㄤ簬璁板綍鎵€鏈?MMIO 璇?鍐欐搷浣溿€?|
| TRANSPARENT_HUGEPAGE_ALWAYS | bool | 濮嬬粓鍚敤閫忔槑澶ч〉浼氬鍔犲簲鐢ㄧ▼搴忕殑鍐呭瓨鍗犵敤锛屽嵈娌℃湁淇濊瘉鐨勬敹鐩婏紝浣嗗畠浼氬鎵€鏈夊簲鐢ㄧ▼搴忚嚜鍔ㄧ敓鏁堛€?|
| TRANSPARENT_HUGEPAGE_MADVISE | bool | 鍚敤閫忔槑澶ч〉鐨?madvise 妯″紡锛屽彧浼氫负浣跨敤 madvise(MADV_HUGEPAGE) 鐨勫簲鐢ㄧ▼搴忓甫鏉ユ€ц兘鎻愬崌鏀剁泭锛屼絾涓嶄細澧炲姞搴旂敤绋嬪簭鍐呭瓨鍗犵敤鐨勯闄?.. |
| TRANSPARENT_HUGEPAGE_NEVER | bool | 榛樿绂佺敤閫忔槑澶ч〉銆備粛鍙湪杩愯鏃堕€氳繃 sysfs 鍚敤銆?|
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ADVISE | bool | 浠呭綋搴旂敤绋嬪簭鎻愪緵 madvise(MADV_HUGEPAGE) 鎻愮ず鏃讹紝鎵嶄负 shmem 鎸傝浇鐐圭嫭鍗犲垎閰嶅ぇ椤点€傝繖纭繚澶ч〉浠呯敤浜庡搷搴旀潵鑷?..鐨勬樉寮忚姹傘€?|
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_ALWAYS | bool | 濮嬬粓灏濊瘯涓?shmem 鎸傝浇鐐瑰垎閰嶅ぇ椤碉紝浼氬鍔犲簲鐢ㄧ▼搴忕殑鍐呭瓨鍗犵敤鑰屾病鏈変繚璇佺殑鏀剁泭锛屼絾瀹冧細瀵规墍鏈夊簲鐢ㄧ▼搴忚嚜鍔ㄧ敓鏁堛€?|
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_NEVER | bool | 榛樿绂佺敤 shmem 鎸傝浇鐐圭殑澶ч〉鍒嗛厤銆備粛鍙€氳繃鍐呮牳鍛戒护琛?'transparent_hugepage_shmem=' 閫夐」鎴栬繍琛屾椂鐨?sysfs 寮€鍏冲惎鐢ㄣ€傛敞鎰?madvise(MADV_COLLAPSE)... |
| TRANSPARENT_HUGEPAGE_SHMEM_HUGE_WITHIN_SIZE | bool | 濡傛灉鍒嗛厤瀹屽叏鍦?i_size 涔嬪唴锛屽垯涓?shmem 鎸傝浇鐐瑰惎鐢ㄥぇ椤靛垎閰嶃€傝閰嶇疆涔熻€冭檻搴旂敤绋嬪簭鍙兘鎻愪緵鐨勪换浣?madvise(MADV_HUGEPAGE) 鎻愮ず...銆?|
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ADVISE | bool | 浠呭綋搴旂敤绋嬪簭鎻愪緵 madvise(MADV_HUGEPAGE) 鎻愮ず鏃讹紝鎵嶄负 tmpfs 鎸傝浇鐐圭嫭鍗犲垎閰嶅ぇ椤点€傝繖纭繚澶ч〉浠呯敤浜庡搷搴旀潵鑷?..鐨勬樉寮忚姹傘€?|
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_ALWAYS | bool | 濮嬬粓灏濊瘯涓?tmpfs 鎸傝浇鐐瑰垎閰嶅ぇ椤碉紝浼氬鍔犲簲鐢ㄧ▼搴忕殑鍐呭瓨鍗犵敤鑰屾病鏈変繚璇佺殑鏀剁泭锛屼絾瀹冧細瀵规墍鏈夊簲鐢ㄧ▼搴忚嚜鍔ㄧ敓鏁堛€?|
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_NEVER | bool | 榛樿绂佺敤 tmpfs 鎸傝浇鐐圭殑澶ч〉鍒嗛厤銆備粛鍙€氳繃鍐呮牳鍛戒护琛?'transparent_hugepage_tmpfs=' 閫夐」鍚敤銆傛敞鎰?madvise(MADV_COLLAPSE) 浠嶅彲瀵艰嚧... |
| TRANSPARENT_HUGEPAGE_TMPFS_HUGE_WITHIN_SIZE | bool | 濡傛灉鍒嗛厤瀹屽叏鍦?i_size 涔嬪唴锛屽垯涓?tmpfs 鎸傝浇鐐瑰惎鐢ㄥぇ椤靛垎閰嶃€傝閰嶇疆涔熻€冭檻搴旂敤绋嬪簭鍙兘鎻愪緵鐨勪换浣?madvise(MADV_HUGEPAGE) 鎻愮ず...銆?|
| UAPI_HEADER_TEST | bool | 缂栬瘧瀵煎嚭鍒扮敤鎴风┖闂寸殑娴嬭瘯澶存枃浠讹紝浠ョ‘淇濆畠浠槸鑷寘鍚殑锛堝嵆鍙綔涓虹嫭绔嬪崟鍏冪紪璇戯級銆傚鏋滀綘鏄紑鍙戣€呮垨娴嬭瘯鑰咃紝骞跺笇鏈涚‘淇濆鍑虹殑澶存枃浠舵槸鑷寘鍚殑... |
| UCLAMP_BUCKETS_COUNT | int | 瀹氫箟瑕佷娇鐢ㄧ殑閽冲埗妗讹紙clamp bucket锛夋暟閲忋€傛瘡涓《鐨勮寖鍥翠负 SCHED_CAPACITY_SCALE/UCLAMP_BUCKETS_COUNT銆傞挸鍒舵《鏁伴噺瓒婂锛屽叾绮掑害瓒婄粏锛岃秺楂?.. |
| UCS2_STRING | tristate | 鎻愪緵涓€涓皢鍒嗘暎鍒楄〃鎷嗗垎涓哄涓潡锛堟瘡鍧椾负涓€涓垎鏁ｅ垪琛級鐨勮緟鍔╁嚱鏁般€傚簲鐢卞笇鏈涘皢鍒嗘暎鍒楄〃鎷嗗垎鍒板涓?DMA 閫氶亾鐨勯┍鍔ㄦ垨 API 閫変腑銆?|
| UID16 | bool | 璇ラ€夐」鍚敤浼犵粺鐨?16 浣?UID 绯荤粺璋冪敤鍖呰鍣ㄣ€?|
| USERCOPY_KUNIT_TEST | tristate | 璇ラ€夐」鏋勫缓 "usercopy_kunit" 妯″潡锛屽 copy_to/from_user 鍩虹璁炬柦杩愯鍋ュ叏鎬ф鏌ワ紝纭繚鍩烘湰鐨勭敤鎴?鍐呮牳杈圭晫娴嬭瘯姝ｅ父宸ヤ綔銆?|
| USERFAULTFD | bool | 鍚敤 userfaultfd() 绯荤粺璋冪敤锛屽厑璁稿湪鐢ㄦ埛绌洪棿鎷︽埅骞跺鐞嗛〉閿欒銆備緷璧栦簬 USERFAULTFD |
| USER_NS | bool | 杩欏厑璁稿鍣紙鍗?vservers锛変娇鐢ㄧ敤鎴峰懡鍚嶇┖闂翠负涓嶅悓鏈嶅姟鍣ㄦ彁渚涗笉鍚岀殑鐢ㄦ埛淇℃伅銆傚綋鍐呮牳涓惎鐢ㄤ簡鐢ㄦ埛鍛藉悕绌洪棿鏃讹紝寤鸿鍚屾椂鍚敤 MEMCG 鎴?.. |
| UTIL_MACROS_KUNIT | tristate | Enable this option to test the util_macros.h function at boot. KUnit tests run during boot and output the results to the debug log in TAP format (http://testanything.org/). Only useful for kernel d... |
| UTS_NS | bool | 鍦ㄨ鍛藉悕绌洪棿涓紝浠诲姟鐪嬪埌 uname() 绯荤粺璋冪敤鎻愪緵鐨勪笉鍚屼俊鎭€?|
| UUID_KUNIT_TEST | tristate | 璇ラ€夐」鍚敤閽堝 uuid 搴撶殑 KUnit 娴嬭瘯濂椾欢锛岃搴撴彁渚涚敓鎴愪笌瑙ｆ瀽 UUID 鍜?GUID 鐨勫嚱鏁般€傝娴嬭瘯濂椾欢妫€鏌?UUID 涓?GUID 瀛楃涓茬殑瑙ｆ瀽銆傚鏋滀笉纭畾锛岄€夋嫨... |
| VIRT_CPU_ACCOUNTING_GEN | bool | 閫夋嫨璇ラ€夐」浠ュ湪鍏ㄥ姩鎬?tick锛坉ynticks锛夌郴缁熶笂鍚敤浠诲姟涓?CPU 鏃堕棿璁拌处銆傝璁拌处閫氳繃鍒╃敤涓婁笅鏂囪窡韪瓙绯荤粺鐩戣姣忎釜鍐呮牳-鐢ㄦ埛杈圭晫鏉ュ疄鐜般€傝... |
| VIRT_CPU_ACCOUNTING_NATIVE | bool | 閫夋嫨璇ラ€夐」浠ュ惎鐢ㄦ洿绮剧‘鐨勪换鍔′笌 CPU 鏃堕棿璁拌处銆傝繖閫氳繃鍦ㄦ瘡娆″唴鏍歌繘鍏ヤ笌閫€鍑轰互鍙婂唴鏍稿唴绯荤粺...涔嬮棿鐨勮浆鎹㈡椂璇诲彇 CPU 璁℃暟鍣ㄦ潵瀹炵幇銆?|
| VMAP_PFN | bool | VM 浜嬩欢璁℃暟鍣ㄦ槸鏄剧ず浜嬩欢璁℃暟鎵€蹇呴渶鐨勩€傝閫夐」鍏佽鍦?EXPERT 绯荤粺涓婄鐢?VM 浜嬩欢璁℃暟鍣ㄣ€傚鏋滅鐢ㄤ簡 VM 浜嬩欢璁℃暟鍣紝/proc/vmstat 灏嗗彧鏄剧ず椤佃鏁?.. |
| WANT_COMPAT_NETLINK_MESSAGES | bool | 璇ラ€夐」鍙敱闇€瑕佸吋瀹?netlink 娑堟伅鐨勫叾浠栭€夐」閫変腑銆?|
| WARN_ABI_ERRORS | bool | Documentation/ABI 涓嬬殑鏂囦欢搴旈伒寰?Documentation/ABI/README 涓殑鎻忚堪銆傜劧鑰岋紝鐢变簬瀹冧滑鏄墜宸ョ紪鍐欑殑锛屾煇浜涙枃浠跺彲鑳藉瓨鍦ㄤ竴浜涢敊璇?.. |
| WARN_CONTEXT_ANALYSIS | bool | 涓婁笅鏂囧垎鏋愶紙Context Analysis锛夋槸涓€绉嶈瑷€鎵╁睍锛岄€氳繃鑾峰彇涓庨噴鏀剧敤鎴峰彲瀹氫箟鐨?涓婁笅鏂囬攣"锛岄潤鎬佹鏌ユ墍闇€鐨勪笂涓嬫枃鏄浜庢椿鍔紙鎴栭潪娲诲姩锛夌姸鎬併€侰lang 灏嗗叾绉颁负... |
| WARN_CONTEXT_ANALYSIS_ALL | bool | 鍚敤鍏ㄦ爲鑼冨洿鐨勪笂涓嬫枃鍒嗘瀽銆傝繖鍙兘浼氫骇鐢熷ぇ閲忚鎶モ€斺€旈闄╄嚜璐熷惎鐢ㄣ€傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| WARN_MISSING_DOCUMENTS | bool | 鏂囨。琚噸鍛藉悕骞朵笉缃曡銆傝閫夐」浣垮唴鏍告鏌ョ己澶辩殑渚濊禆锛屽苟鍦ㄧ己澶辨椂鍙戝嚭璀﹀憡銆備粎鍦ㄤ粠 git 鏍戞瀯寤哄唴鏍告椂鏈夋晥... |
| WERROR | bool | 鍐呮牳鏋勫缓涓嶅簲浜х敓浠讳綍缂栬瘧鍣ㄨ鍛婏紝璇ラ€夐」鍚敤 '-Werror'锛堥拡瀵?C锛変笌 '-Dwarnings'锛堥拡瀵?Rust锛夋爣蹇椾互榛樿寮哄埗鎵ц璇ヨ鍒欍€傛潵鑷叾浠栧伐鍏风殑鏌愪簺璀﹀憡... |
| WQ_CPU_INTENSIVE_REPORT | bool | 鍦ㄦ閫夋嫨 Y 浠ュ惎鐢ㄥ鍗犵敤 CPU 瓒呰繃 workqueue.cpu_intensive_thresh_us 鐨勫苟鍙戠鐞嗙殑姣?CPU 宸ヤ綔椤圭殑鎶ュ憡銆傚伐浣滈槦鍒椾細鑷姩妫€娴嬪苟灏嗗叾鎺掗櫎鍑哄苟鍙?.. |
| WQ_WATCHDOG | bool | 鍦ㄦ閫夋嫨 Y 浠ュ惎鐢ㄥ伐浣滈槦鍒椾笂鐨勫仠椤匡紙stall锛夋娴嬨€傚鏋滀竴涓伐浣滄睜鍦ㄨ秴杩囩粰瀹氭椂闂达紙榛樿 30 绉掞級鍐呭鎸傝捣鐨勫伐浣滈」娌℃湁鍙栧緱杩涘睍锛屽垯浼氭墦鍗拌鍛婃秷鎭?.. |
| WW_MUTEX_SELFTEST | tristate | 璇ラ€夐」鎻愪緵涓€涓湪鍐呮牳 struct ww_mutex 閿?API 涓婅繍琛屾祴璇曠殑鍐呮牳妯″潡銆傚缓璁厤鍚?DEBUG_WW_MUTEX_SLOWPATH 鍚敤姝ゆ祴璇曞伐鍏枫€傚鏋?..閫夋嫨 M銆?|
| XXHASH | tristate | 璇ラ€夐」鍚敤 32 浣?PRNG 搴撳嚱鏁扮殑鍒濆鍖栬嚜娴嬨€? # 鍘嬬缉鏀寔鍦ㄩ渶瑕佹椂琚?select # |
| ZSMALLOC_CHAIN_SIZE | int | 璇ラ€夐」璁剧疆 zmalloc 椤碉紙zspage锛夊彲鐢卞叾缁勬垚鐨勭墿鐞嗛〉鏁伴噺涓婇檺銆傛渶浼樼殑 zspage 閾惧ぇ灏忓湪鍒濆鍖栨湡闂翠负姣忎釜澶у皬绫昏绠?.. |
| ZSWAP | bool | 涓€涓敤浜庝氦鎹㈤〉鐨勮交閲忕骇鍘嬬缉缂撳瓨銆傚畠鑾峰彇姝ｅ湪琚崲鍑虹殑椤碉紝骞跺皾璇曞皢瀹冧滑鍘嬬缉鍒板姩鎬佸垎閰嶇殑鍩轰簬 RAM 鐨勫唴瀛樻睜涓€傝繖鍙?.. |
| ZSWAP_COMPRESSOR_DEFAULT | string | 璇ラ€夐」鍦?zsmalloc 涓惎鐢ㄤ唬鐮佷互鏀堕泦鍏充簬 zsmalloc 鍐呴儴鍙戠敓鎯呭喌鐨勫悇绫荤粺璁★紝骞堕€氳繃 debugfs 灏嗕俊鎭鍑哄埌鐢ㄦ埛绌洪棿銆傚鏋滀笉纭畾锛岄€夋嫨 N銆?|
| ZSWAP_COMPRESSOR_DEFAULT_842 | bool | 浣跨敤 842 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_DEFLATE | bool | 浣跨敤 Deflate 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_LZ4 | bool | 浣跨敤 LZ4 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_LZ4HC | bool | 浣跨敤 LZ4HC 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_LZO | bool | 浣跨敤 LZO 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_COMPRESSOR_DEFAULT_ZSTD | bool | 浣跨敤 zstd 绠楁硶浣滀负榛樿鍘嬬缉绠楁硶銆?|
| ZSWAP_DEFAULT_ON | bool | 鑻ラ€変腑锛屼氦鎹㈤〉鐨勫帇缂╃紦瀛樺皢鍦ㄥ惎鍔ㄦ椂鍚敤锛屽惁鍒欑鐢ㄣ€傛澶勭殑閫夋嫨鍙€氳繃鍐呮牳鍛戒护琛?'zswap.enabled='...瑕嗙洊銆?|
| ZSWAP_SHRINKER_DEFAULT_ON | bool | 鑻ラ€変腑锛屽皢鍚敤 zswap shrinker锛屽瓨鍌ㄥ湪 zswap 姹犱腑鐨勯〉灏嗗湪鍐呭瓨鍘嬪姏涓嬪彲鐢ㄤ簬鍥炴敹锛堝嵆鍐欏洖鍚庡浜ゆ崲璁惧锛夈€傝繖鎰忓懗鐫€... |
| if | bool | 濡傛灉浣犲湪姝ら€夋嫨 Y锛実cc 浼氳鎸囩ず涓虹粨鏋勪綋绫诲瀷鐢熸垚杈冨皯鐨勮皟璇曚俊鎭€傝繖鎰忓懗鐫€闇€瑕佸畬鏁磋皟璇曚俊鎭殑宸ュ叿锛堝 kgdb 鎴?systemtap锛変細涓嶆弧鎰忋€備絾... |
| select | bool | 鐢熸垚 DWARF v5 璋冭瘯淇℃伅銆傞渶瑕?binutils 2.35.2銆乬cc 5.0+锛坓cc 5.0+ 鎺ュ彈 -gdwarf-5 鏍囧織锛屼絾瀵规煇浜涜崏妗堢壒鎬х洿鍒?7.0 鎵嶄粎鏈夐儴鍒嗘敮鎸侊級浠ュ強 gdb 8.0+銆傚...鐨勬洿鏀广€?|

---

# Makefile Targets

## Build targets

| Target | 鎻忚堪 | 鏉ユ簮 |
|--------|-------------|--------|
| all | 濡傛灉鏋勫缓澶栭儴妯″潡锛屾垜浠笉鍏冲績 all: 瑙勫垯锛岃€屾槸璁?__all 渚濊禆浜?modules | Makefile |
| dtbs_install |  | Makefile |
| headers |  | Makefile |
| headers_install |  | Makefile |
| modules | 鏋勫缓鎵€鏈夊彲鍔犺浇鐨勫唴鏍告ā鍧?| Makefile |
| modules_install |  | Makefile |
| vmlinux |  | Makefile |

## Configuration targets

| Target | 鎻忚堪 | 鏉ユ簮 |
|--------|-------------|--------|
| config |  | Makefile |

## Clean targets

| Target | 鎻忚堪 | 鏉ユ簮 |
|--------|-------------|--------|
| clean | clean - 鍒犻櫎澶ч儴鍒嗗唴瀹癸紝浣嗕繚鐣欒冻澶熷唴瀹逛互鏋勫缓澶栭儴妯″潡 | Makefile |
| distclean | distclean | Makefile |
| mrproper | mrproper - 鍒犻櫎鎵€鏈夌敓鎴愮殑鏂囦欢锛屽寘鎷?.config | Makefile |

## Documentation targets

| Target | 鎻忚堪 | 鏉ユ簮 |
|--------|-------------|--------|
| cleandocs | 鍒犻櫎鎵€鏈夌敓鎴愮殑鏂囨。鏂囦欢 | Makefile |
| htmldocs-redirects |  | Makefile |
| markdowndocs | 閫氳繃 Pandoc 鍚庡鐞嗘瀯寤?Markdown 鏂囨。 | Makefile |
| refcheckdocs | 妫€鏌ユ枃妗ｄ腑鎹熷潖鐨勬枃浠跺紩鐢?| Makefile |

## Other targets

| Target | 鎻忚堪 | 鏉ユ簮 |
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
| rust-analyzer | 涓?rust-analyzer锛堣瑷€鏈嶅姟鍣ㄥ崗璁殑涓€绉嶅疄鐜帮級鐢熸垚 rust-project.json锛堟弿杩伴潪 Cargo Rust 椤圭洰缁撴瀯鐨勬枃浠讹級銆?| Makefile |
| rustavailable | "Rust 鏄惁鍙敤锛? 鐩爣 | Makefile |
| rustdoc | 鏂囨。鐩爣  浣跨敤鍗曟暟褰㈠紡浠ラ伩鍏嶈繚鍙?`no-dot-config-targets`銆?| Makefile |
| rustfmt | 鏍煎紡鍖栫洰鏍? 鐢熸垚鐨勬枃浠朵互鍙?vendored crate 浼氳璺宠繃銆?| Makefile |
| rustfmtcheck |  | Makefile |
| rusttest | 娴嬭瘯鐩爣 | Makefile |
| scripts | 鏋勫缓浜?scripts/ 涓殑棰濆杈呭姪绋嬪簭銆備粩缁嗗垪鍑轰緷璧栵紝浠ュ厤鎴戜滑灏濊瘯骞惰鏋勫缓 scripts 涓ゆ | Makefile |
| scripts_basic | 鏋勫缓浜?scripts/basic/ 涓殑鍩烘湰杈呭姪绋嬪簭 | Makefile |
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
