锘。
## 视频 模式 Selection 支持 2.13


:Copyright: |copy| 1995--1999 Martin Mares, <mj@ucw.cz>

#### Intro


small document describes the "视频 模式 Selection" 特
allows the 使用 各种 特殊 视频 modes 受支the 视频 BIOS. Due
usage the BIOS, the selection limited boot time (之前 the
内核 decompression starts) works 80X86 machines 
booted through BIOS 固件 (相对through UEFI, kexec, ).


   Short intro 用于 the impatient: Just 使用 vga=ask 用于 the 第一 time,
   enter `scan` the 视频 模式 prompt, pick the 模式 希望 使用,
   remember 鍏，妯″紡 ID (the four-digit hexadecimal 鏁板瓧) 鍜，鐒跺悗
   set the vga 参数 数字 (converted decimal 第一).

The 视频 模式 使用 selected 一内核 参数 
specified the 内核 Makefile (the SVGA_模式=... line) the "vga=..."
选项 LILO (一其他 boot loader 使用) the "xrandr" utility
(present 标准 Linux utility packages). 您可使用 the 以下 
```

   NORMAL_VGA - Standard 80x25 mode available on all display adapters.

   EXTENDED_VGA	- Standard 8-pixel font mode: 80x43 on EGA, 80x50 on VGA.

   ASK_VGA - Display a video mode menu upon startup (see below).

   0..35 - Menu item number (when you have used the menu to view the list of
      modes available on your adapter, you can specify the menu item you want
      to use). 0..9 correspond to "0".."9", 10..35 to "a".."z". Warning: the
      mode list displayed may vary as the kernel version changes, because the
      modes are listed in a "first detected -- first displayed" manner. It's
      better to use absolute mode numbers instead.

   0x.... - Hexadecimal video mode ID (also displayed on the menu, see below
      for exact meaning of the ID). Warning: LILO doesn't support
      hexadecimal numbers -- you have to convert it to decimal manually.

```
#### Menu


The ASK_VGA 模式 causes the 内核 offer 一视频 模式 menu upon
bootup. displays 一"Press <RETURN> 参见 视频 modes 可用, <SPACE>
鍒?continue 鎴?wait 30 secs" message. 鑻，鎮?press <RETURN>, 鎮?enter the
menu, press <SPACE> wait 30 seconds, the 内核 boot up 
the 标准 80x25 模式.

```

	Video adapter: <name-of-detected-video-adapter>
	Mode:    COLSxROWS:
	0  0F00  80x25
	1  0F01  80x50
	2  0F02  80x43
	3  0F03  80x26
	....
	Enter mode number or ``scan``: <flashing-cursor-here>

```
<name-of-detected-video-adapter> tells 什视频 adapter did Linux detect
-- s 任一一generic adapter name (MDA, CGA, HGC, EGA, VGA, VESA VGA [一VGA
VESA-compliant BIOS]) 一chipset name (e.g., Trident). Direct detection
chipsets turned off 默认情况作为 s inherently unreliable 由于
absolutely insane PC design.

"0  0F00  80x25" means the 第一 menu item (the menu items numbered
来自 "0" "9" 来自 "一 "z") 一80x25 模式 ID=0x0f00 (参见 the
接下section 用于 一description 模式 IDs).

<flashing-cursor-here> encourages enter the item 数字 模式 ID
wish set press <RETURN>. the computer complains something 关于
"未知 模式 ID", 它是 trying tell isn't 可能 set 此类
一模式. s 可能 press <RETURN> leaves the 电流 模式.

The 模式 列出 通常 包含 一少量 基本 modes 一VESA modes.  
case 您的 chipset 具有 已经 detected, 一chipset-specific modes shown 作为
well (一这些 可能 missing unusable 您的 machine 作为 不同
BIOSes 通常 shipped the 相同 the 模式 numbers depend purely
鍦?the VGA BIOS).

The modes displayed the menu partially sorted: The 列出 starts 
the 标准 modes (80x25 80x50) followed "特殊" modes (80x28 
80x43), 本地 modes (the 本地 modes 特已启, VESA modes 
finally SVGA modes 用于 the auto-detected adapter.

happy the 模式 列出 offered (e.g., think 您的 
able 执行 更多), 您可enter "scan" 而非 item 数字 / 模式 ID.  The
program try ask the BIOS 用于 全部 可能 视频 模式 numbers test
什happens 然后. The screen probably flashing wildly 用于 一time 
strange noises heard 来自 inside the 监视因此 然后, really
全部 consistent 视频 modes 受支您的 BIOS appear (增强maybe 一
`ghost modes`). afraid 可以 damage 您的 监视 don't 使用
函数.

之后 scanning, the 模式 ordering 一不同: the auto-detected SVGA
modes listed 全部 the modes revealed `scan` shown 之前
全部 VESA modes.

#### 模式 IDs


因为 the complexity 全部 the 视频 stuff, the 视频 模式 IDs
使用 此处 一复杂. 一视频 模式 ID 一16-数字 通常
expressed 一hexadecimal notation (starting "0x"). 您可set 一模式
entering 模式 directly know even isn't shown the menu.

```

   0x0000 to 0x00ff - menu item references. 0x0000 is the first item. Don't use
	outside the menu as this can change from boot to boot (especially if you
	have used the ``scan`` feature).

   0x0100 to 0x017f - standard BIOS modes. The ID is a BIOS video mode number
	(as presented to INT 10, function 00) increased by 0x0100.

   0x0200 to 0x08ff - VESA BIOS modes. The ID is a VESA mode ID increased by
	0x0100. All VESA modes should be autodetected and shown on the menu.

   0x0900 to 0x09ff - Video7 special modes. Set by calling INT 0x10, AX=0x6f05.
	(Usually 940=80x43, 941=132x25, 942=132x44, 943=80x60, 944=100x60,
	945=132x28 for the standard Video7 BIOS)

   0x0f00 to 0x0fff - special modes (they are set by various tricks -- usually
	by modifying one of the standard modes). Currently available:
	0x0f00	standard 80x25, don't reset mode if already set (=FFFF)
	0x0f01	standard with 8-point font: 80x43 on EGA, 80x50 on VGA
	0x0f02	VGA 80x43 (VGA switched to 350 scanlines with a 8-point font)
	0x0f03	VGA 80x28 (standard VGA scans, but 14-point font)
	0x0f04	leave current video mode
	0x0f05	VGA 80x30 (480 scans, 16-point font)
	0x0f06	VGA 80x34 (480 scans, 14-point font)
	0x0f07	VGA 80x60 (480 scans, 8-point font)
	0x0f08	Graphics hack (see the VIDEO_GFX_HACK paragraph below)

   0x1000 to 0x7fff - modes specified by resolution. The code has a "0xRRCC"
	form where RR is a number of rows and CC is a number of columns.
	E.g., 0x1950 corresponds to a 80x25 mode, 0x2b84 to 132x43 etc.
	This is the only fully portable way to refer to a non-standard mode,
	but it relies on the mode being found and displayed on the menu
	(remember that mode scanning is not done automatically).

   0xff00 to 0xffff - aliases for backward compatibility:
	0xffff	equivalent to 0x0f00 (standard 80x25)
	0xfffe	equivalent to 0x0f01 (EGA 80x43 or VGA 80x50)

```
add 0x8000 the 模式 ID, the program try recalculate
vertical 显示timing 根据 模式 参数, 使用 
eliminate 一annoying bugs 某些 VGA BIOSes (通常 那些 使用 用于
鍗，涓?S3 chipsets 鍜，鏃?Cirrus Logic BIOSes) -- mainly extra lines 鍦?the
end the 显示

#### 选项


Build 选项 用于 arch/x86/boot/* selected the 内核 kconfig
utility the 内核 .配置 文件.

视频_GFX_HACK - 包含 特殊 hack 用于 设置 graphics modes
使用 稍后 特殊 驱动.
Allows set _任何_ BIOS 模式 including graphic ones forcing 特定
text screen resolution 而非 peeking 来自 BIOS variables. Don't 使用
除非 think know 什re doing. activate setup, 使用
模式 数字 0x0f08 (参见 the 模式 IDs section 上文).

#### 仍然 doesn't work?


the 模式 detection doesn't work (e.g., the 模式 列出 incorrect 
the machine hangs 而非 displaying the menu), try switch off 一
the 配置 选项 listed 在…下 "选项". fails, 您可仍然 使用
您的 内核 the 视频 模式 set directly 通过 the 内核 参数.

任一case, send me 一bug report containing 什_exactly_
happens 如何 执行 the 配置 switches affect the behaviour the bug.

启动 Linux 来自 M$-DOS, 可能 使用 一DOS tools 用于
视频 模式 设置. case, 必须 specify the 0x0f04 模式 ("leave
电流 设置") Linux, 因为 don't 使用 任何 non-standard
模式, Linux switch 80x25 automatically.

set 一extended 模式 那里's one 更多 extra lines the
bottom the 显示containing 已经 scrolled-out text, 您的 VGA BIOS
包含 the 大多通用 视频 BIOS bug called "incorrect vertical 显示
end 设置". Adding 0x8000 the 模式 ID 可能 fix the problem. Unfortunately,
必须 已完manually -- autodetection mechanisms 可用.

#### History


=============== ================================================================
1.0 -Nov-95)	第一 版本 supporting 全部 adapters 受支the 
		setup.S + Cirrus Logic 54XX. Present 一1.3.4 kernels
		然后 removed 由于 instability 一machines.
2.0 (28-Jan-96)	Rewritten 来自 scratch. Cirrus Logic 64XX 支持 added, almost
		everything configurable, the VESA 支持 应当 much 更多
		stable, explicit 模式 numbering allowed, "scan" implemented 
2.1 (30-Jan-96) VESA modes moved 0x200-0x3ff. 模式 selection resolution
		受支 少量 bugs fixed. VESA modes listed prior 
		modes supplied SVGA autodetection 作为 它们更多 reliable.
		CLGD autodetect works better. Doesn't depend 80x25 正在
		active started. Scanning fixed. 80x43 (任何 VGA) added.
		Code cleaned up.
2.2 (01-Feb-96)	EGA 80x43 fixed. VESA extended 鍒?0x200-0x4ff (non-standard 02XX
		VESA modes work 现在). 显示end bug workaround 受支
		特殊 modes renumbered 允许 adding the "recalculate"
		鏍囧織, 0xffff 鍜?0xfffe became aliases 鑰岄潪 real IDs.
		Screen contents retained 期间 模式 changes.
2.3 (15-Mar-96)	Changed work 1.3.74 内核.
2.4 (18-Mar-96)	Added patches Hans Lermen fixing 一内存 overwrite problem
		一boot loaders. 内存 管理 rewritten reflect
		这些 changes. Unfortunately, screen contents retaining works
		一loaders 现在.
		Added 一Tseng 132x60 模式.
2.5 (19-Mar-96)	Fixed 一VESA 模式 scanning bug introduced 2.4.
2.6 (25-Mar-96)	一VESA BIOS 错误 reported -- fixes 错误 reports 
		若干 broken VESA code (e.g., ATI VGA).
2.7 (09-Apr-96)	- Accepted 全部 VESA modes range 0x100 0x7ff, 因为 一
		  使用 very strange 模式 numbers.
  - Added Realtek VGA modes (thanks 鍒?Gonzalo Tornaria).
  - 硬件 testing order slightly changed, tests 基于 ROM
		  contents 已完作为 第一.
  - Added 支持 用于 特殊 视频7 模式 switching 函数
		  (thanks 鍒?Tom Vander Aa).
  - Added 480-scanline modes (especially useful 用于 notebooks,
		  original 版本 written hhanemaa@cs.ruu.nl, patched 
		  Jeff Chua, rewritten 鐢?me).
  - Screen store/restore fixed.
2.8 (14-Apr-96) - 前一释放 曾是 compilable 配置_视频_SVGA.
  - Better recognition text modes 期间 模式 scan.
2.9 (12-May-96)	- Ignored VESA modes 0x80 - 0xff (更多 VESA BIOS bugs!)
2.10(11-Nov-96) - The whole thing made 可
  - Added the 配置_视频_400_HACK switch.
  - Added the 配置_视频_GFX_HACK switch.
  - Code cleanup.
2.11(03-May-97) - 灏氭湭 another cleanup, 鐜板湪 including 涔?the documentation.
  - Direct testing SVGA adapters turned off 默认情况 `scan`
		  offered explicitly 鍦?the prompt line.
  - Removed the doc section describing adding 鐨，鏂?probing
		  函数 作为 I try get rid _全部_ 硬件 probing 此处.
2.12(25-May-98) Added 支持 用于 VESA 缓冲graphics.
2.13(14-May-99) 次要 documentation fixes.
=============== ================================================================
