
## 视频 模式 Selection 支持 2.13


:Copyright: |copy| 1995--1999 Martin Mares, <mj@ucw.cz>

#### Intro


此 small document describes the "视频 模式 Selection" 特性 其
allows the 使用 的 各种 特殊 视频 modes 受支持 由 the 视频 BIOS. Due
到 usage 的 the BIOS, the selection 是 limited 到 boot time (之前 the
内核 decompression starts) 和 works 仅 在 80X86 machines 该 是
booted through BIOS 固件 (相对于 through UEFI, kexec, 等.).


   Short intro 用于 the impatient: Just 使用 vga=ask 用于 the 第一 time,
   enter `scan` 在 the 视频 模式 prompt, pick the 模式 您 希望 到 使用,
   remember 其 模式 ID (the four-digit hexadecimal 数字) 和 然后
   set the vga 参数 到 此 数字 (converted 到 decimal 第一).

The 视频 模式 到 为 使用 是 selected 由 一个 内核 参数 其 可 为
specified 在 the 内核 Makefile (the SVGA_模式=... line) 或 由 the "vga=..."
选项 的 LILO (或 一些 其他 boot loader 您 使用) 或 由 the "xrandr" utility
(present 在 标准 Linux utility packages). 您可以 使用 the 以下 值
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


The ASK_VGA 模式 causes the 内核 到 offer 一个 视频 模式 menu upon
bootup. 它 displays 一个 "Press <RETURN> 到 参见 视频 modes 可用, <SPACE>
到 continue 或 wait 30 secs" message. 若 您 press <RETURN>, 您 enter the
menu, 若 您 press <SPACE> 或 wait 30 seconds, the 内核 将 boot up 在
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
<name-of-detected-video-adapter> tells 什么 视频 adapter did Linux detect
-- 它's 任一个 一个 generic adapter name (MDA, CGA, HGC, EGA, VGA, VESA VGA [一个 VGA
与 VESA-compliant BIOS]) 或 一个 chipset name (e.g., Trident). Direct detection
的 chipsets 是 turned off 默认情况下 作为 它's inherently unreliable 由于
absolutely insane PC design.

"0  0F00  80x25" means 该 the 第一 menu item (the menu items 是 numbered
来自 "0" 到 "9" 和 来自 "一个" 到 "z") 是 一个 80x25 模式 与 ID=0x0f00 (参见 the
接下来 section 用于 一个 description 的 模式 IDs).

<flashing-cursor-here> encourages 您 到 enter the item 数字 或 模式 ID
您 wish 到 set 和 press <RETURN>. 若 the computer complains something 关于
"未知 模式 ID", 它是 trying 到 tell 您 该 它 isn't 可能 到 set 此类
一个 模式. 它's 也 可能 到 press 仅 <RETURN> 其 leaves the 电流 模式.

The 模式 列出 通常 包含 一个 少量 基本 modes 和 一些 VESA modes.  在
case 您的 chipset 具有 已经 detected, 一些 chipset-specific modes 是 shown 作为
well (一些 的 这些 可能 为 missing 或 unusable 在 您的 machine 作为 不同
BIOSes 是 通常 shipped 与 the 相同 卡 和 the 模式 numbers depend purely
在 the VGA BIOS).

The modes displayed 在 the menu 是 partially sorted: The 列出 starts 与
the 标准 modes (80x25 和 80x50) followed 由 "特殊" modes (80x28 和
80x43), 本地 modes (若 the 本地 modes 特性 是 已启用), VESA modes 和
finally SVGA modes 用于 the auto-detected adapter.

若 您 是 不 happy 与 the 模式 列出 offered (e.g., 若 您 think 您的 卡
是 able 到 执行 更多), 您可以 enter "scan" 而非 item 数字 / 模式 ID.  The
program 将 try 到 ask the BIOS 用于 全部 可能 视频 模式 numbers 和 test
什么 happens 然后. The screen 将 为 probably flashing wildly 用于 一些 time 和
strange noises 将 为 heard 来自 inside the 监视器 和 因此 在 和 然后, really
全部 consistent 视频 modes 受支持 由 您的 BIOS 将 appear (增强版 maybe 一些
`ghost modes`). 若 您 是 afraid 此 可以 damage 您的 监视器, don't 使用
此 函数.

之后 scanning, the 模式 ordering 是 一个 位 不同: the auto-detected SVGA
modes 是 不 listed 在 全部 和 the modes revealed 由 `scan` 是 shown 之前
全部 VESA modes.

#### 模式 IDs


因为 的 the complexity 的 全部 the 视频 stuff, the 视频 模式 IDs
使用 此处 是 也 一个 位 复杂. 一个 视频 模式 ID 是 一个 16-位 数字 通常
expressed 在 一个 hexadecimal notation (starting 与 "0x"). 您可以 set 一个 模式
由 entering 其 模式 directly 若 您 know 它 even 若 它 isn't shown 在 the menu.

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
若 您 add 0x8000 到 the 模式 ID, the program 将 try 到 recalculate
vertical 显示器 timing 根据 模式 参数, 其 可 为 使用 到
eliminate 一些 annoying bugs 的 某些 VGA BIOSes (通常 那些 使用 用于
卡 与 S3 chipsets 和 旧 Cirrus Logic BIOSes) -- mainly extra lines 在 the
end 的 the 显示器.

#### 选项


Build 选项 用于 arch/x86/boot/* 是 selected 由 the 内核 kconfig
utility 和 the 内核 .配置 文件.

视频_GFX_HACK - 包含 特殊 hack 用于 设置 的 graphics modes
到 为 使用 稍后 由 特殊 驱动.
Allows 到 set _任何_ BIOS 模式 including graphic ones 和 forcing 特定
text screen resolution 而非 peeking 它 来自 BIOS variables. Don't 使用
除非 您 think 您 know 什么 您're doing. 到 activate 此 setup, 使用
模式 数字 0x0f08 (参见 the 模式 IDs section 上文).

#### 仍然 doesn't work?


当 the 模式 detection doesn't work (e.g., the 模式 列出 是 incorrect 或
the machine hangs 而非 displaying the menu), try 到 switch off 一些 的
the 配置 选项 listed 在…下 "选项". 若 它 fails, 您可以 仍然 使用
您的 内核 与 the 视频 模式 set directly 通过 the 内核 参数.

在 任一个 case, 请 send me 一个 bug report containing 什么 _exactly_
happens 和 如何 执行 the 配置 switches affect the behaviour 的 the bug.

若 您 启动 Linux 来自 M$-DOS, 您 可能 也 使用 一些 DOS tools 用于
视频 模式 设置. 在 此 case, 您 必须 specify the 0x0f04 模式 ("leave
电流 设置") 到 Linux, 因为 若 您 don't 和 您 使用 任何 non-standard
模式, Linux 将 switch 到 80x25 automatically.

若 您 set 一些 extended 模式 和 那里's one 或 更多 extra lines 在 the
bottom 的 the 显示器 containing 已经 scrolled-out text, 您的 VGA BIOS
包含 the 大多数 通用 视频 BIOS bug called "incorrect vertical 显示器
end 设置". Adding 0x8000 到 the 模式 ID 可能 fix the problem. Unfortunately,
此 必须 为 已完成 manually -- 无 autodetection mechanisms 是 可用.

#### History


=============== ================================================================
1.0 (??-Nov-95)	第一 版本 supporting 全部 adapters 受支持 由 the 旧
		setup.S + Cirrus Logic 54XX. Present 在 一些 1.3.4? kernels
		和 然后 removed 由于 instability 在 一些 machines.
2.0 (28-Jan-96)	Rewritten 来自 scratch. Cirrus Logic 64XX 支持 added, almost
		everything 是 configurable, the VESA 支持 应当 为 much 更多
		stable, explicit 模式 numbering allowed, "scan" implemented 等.
2.1 (30-Jan-96) VESA modes moved 到 0x200-0x3ff. 模式 selection 由 resolution
		受支持. 少量 bugs fixed. VESA modes 是 listed prior 到
		modes supplied 由 SVGA autodetection 作为 它们是 更多 reliable.
		CLGD autodetect works better. Doesn't depend 在 80x25 正在
		active 当 started. Scanning fixed. 80x43 (任何 VGA) added.
		Code cleaned up.
2.2 (01-Feb-96)	EGA 80x43 fixed. VESA extended 到 0x200-0x4ff (non-standard 02XX
		VESA modes work 现在). 显示器 end bug workaround 受支持.
		特殊 modes renumbered 到 允许 adding 的 the "recalculate"
		标志, 0xffff 和 0xfffe became aliases 而非 real IDs.
		Screen contents retained 期间 模式 changes.
2.3 (15-Mar-96)	Changed 到 work 与 1.3.74 内核.
2.4 (18-Mar-96)	Added patches 由 Hans Lermen fixing 一个 内存 overwrite problem
		与 一些 boot loaders. 内存 管理 rewritten 到 reflect
		这些 changes. Unfortunately, screen contents retaining works
		仅 与 一些 loaders 现在.
		Added 一个 Tseng 132x60 模式.
2.5 (19-Mar-96)	Fixed 一个 VESA 模式 scanning bug introduced 在 2.4.
2.6 (25-Mar-96)	一些 VESA BIOS 错误 不 reported -- 它 fixes 错误 reports 在
		若干 卡 与 broken VESA code (e.g., ATI VGA).
2.7 (09-Apr-96)	- Accepted 全部 VESA modes 在 range 0x100 到 0x7ff, 因为 一些
		  卡 使用 very strange 模式 numbers.
  - Added Realtek VGA modes (thanks 到 Gonzalo Tornaria).
  - 硬件 testing order slightly changed, tests 基于 ROM
		  contents 已完成 作为 第一.
  - Added 支持 用于 特殊 视频7 模式 switching 函数
		  (thanks 到 Tom Vander Aa).
  - Added 480-scanline modes (especially useful 用于 notebooks,
		  original 版本 written 由 hhanemaa@cs.ruu.nl, patched 由
		  Jeff Chua, rewritten 由 me).
  - Screen store/restore fixed.
2.8 (14-Apr-96) - 前一个 释放 曾是 不 compilable 无 配置_视频_SVGA.
  - Better recognition 的 text modes 期间 模式 scan.
2.9 (12-May-96)	- Ignored VESA modes 0x80 - 0xff (更多 VESA BIOS bugs!)
2.10(11-Nov-96) - The whole thing made 可选.
  - Added the 配置_视频_400_HACK switch.
  - Added the 配置_视频_GFX_HACK switch.
  - Code cleanup.
2.11(03-May-97) - 尚未 another cleanup, 现在 including 也 the documentation.
  - Direct testing 的 SVGA adapters turned off 默认情况下, `scan`
		  offered explicitly 在 the prompt line.
  - Removed the doc section describing adding 的 新 probing
		  函数 作为 I try 到 get rid 的 _全部_ 硬件 probing 此处.
2.12(25-May-98) Added 支持 用于 VESA 帧 缓冲区 graphics.
2.13(14-May-99) 次要 documentation fixes.
=============== ================================================================
