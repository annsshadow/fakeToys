锘?
## 瑙嗛 妯″紡 Selection 鏀寔 2.13


:Copyright: |copy| 1995--1999 Martin Mares, <mj@ucw.cz>

#### Intro


姝?small document describes the "瑙嗛 妯″紡 Selection" 鐗规€?鍏?
allows the 浣跨敤 鐨?鍚勭 鐗规畩 瑙嗛 modes 鍙楁敮鎸?鐢?the 瑙嗛 BIOS. Due
鍒?usage 鐨?the BIOS, the selection 鏄?limited 鍒?boot time (涔嬪墠 the
鍐呮牳 decompression starts) 鍜?works 浠?鍦?80X86 machines 璇?鏄?
booted through BIOS 鍥轰欢 (鐩稿浜?through UEFI, kexec, 绛?).


   Short intro 鐢ㄤ簬 the impatient: Just 浣跨敤 vga=ask 鐢ㄤ簬 the 绗竴 time,
   enter `scan` 鍦?the 瑙嗛 妯″紡 prompt, pick the 妯″紡 鎮?甯屾湜 鍒?浣跨敤,
   remember 鍏?妯″紡 ID (the four-digit hexadecimal 鏁板瓧) 鍜?鐒跺悗
   set the vga 鍙傛暟 鍒?姝?鏁板瓧 (converted 鍒?decimal 绗竴).

The 瑙嗛 妯″紡 鍒?涓?浣跨敤 鏄?selected 鐢?涓€涓?鍐呮牳 鍙傛暟 鍏?鍙?涓?
specified 鍦?the 鍐呮牳 Makefile (the SVGA_妯″紡=... line) 鎴?鐢?the "vga=..."
閫夐」 鐨?LILO (鎴?涓€浜?鍏朵粬 boot loader 鎮?浣跨敤) 鎴?鐢?the "xrandr" utility
(present 鍦?鏍囧噯 Linux utility packages). 鎮ㄥ彲浠?浣跨敤 the 浠ヤ笅 鍊?
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


The ASK_VGA 妯″紡 causes the 鍐呮牳 鍒?offer 涓€涓?瑙嗛 妯″紡 menu upon
bootup. 瀹?displays 涓€涓?"Press <RETURN> 鍒?鍙傝 瑙嗛 modes 鍙敤, <SPACE>
鍒?continue 鎴?wait 30 secs" message. 鑻?鎮?press <RETURN>, 鎮?enter the
menu, 鑻?鎮?press <SPACE> 鎴?wait 30 seconds, the 鍐呮牳 灏?boot up 鍦?
the 鏍囧噯 80x25 妯″紡.

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
<name-of-detected-video-adapter> tells 浠€涔?瑙嗛 adapter did Linux detect
-- 瀹?s 浠讳竴涓?涓€涓?generic adapter name (MDA, CGA, HGC, EGA, VGA, VESA VGA [涓€涓?VGA
涓?VESA-compliant BIOS]) 鎴?涓€涓?chipset name (e.g., Trident). Direct detection
鐨?chipsets 鏄?turned off 榛樿鎯呭喌涓?浣滀负 瀹?s inherently unreliable 鐢变簬
absolutely insane PC design.

"0  0F00  80x25" means 璇?the 绗竴 menu item (the menu items 鏄?numbered
鏉ヨ嚜 "0" 鍒?"9" 鍜?鏉ヨ嚜 "涓€涓? 鍒?"z") 鏄?涓€涓?80x25 妯″紡 涓?ID=0x0f00 (鍙傝 the
鎺ヤ笅鏉?section 鐢ㄤ簬 涓€涓?description 鐨?妯″紡 IDs).

<flashing-cursor-here> encourages 鎮?鍒?enter the item 鏁板瓧 鎴?妯″紡 ID
鎮?wish 鍒?set 鍜?press <RETURN>. 鑻?the computer complains something 鍏充簬
"鏈煡 妯″紡 ID", 瀹冩槸 trying 鍒?tell 鎮?璇?瀹?isn't 鍙兘 鍒?set 姝ょ被
涓€涓?妯″紡. 瀹?s 涔?鍙兘 鍒?press 浠?<RETURN> 鍏?leaves the 鐢垫祦 妯″紡.

The 妯″紡 鍒楀嚭 閫氬父 鍖呭惈 涓€涓?灏戦噺 鍩烘湰 modes 鍜?涓€浜?VESA modes.  鍦?
case 鎮ㄧ殑 chipset 鍏锋湁 宸茬粡 detected, 涓€浜?chipset-specific modes 鏄?shown 浣滀负
well (涓€浜?鐨?杩欎簺 鍙兘 涓?missing 鎴?unusable 鍦?鎮ㄧ殑 machine 浣滀负 涓嶅悓
BIOSes 鏄?閫氬父 shipped 涓?the 鐩稿悓 鍗?鍜?the 妯″紡 numbers depend purely
鍦?the VGA BIOS).

The modes displayed 鍦?the menu 鏄?partially sorted: The 鍒楀嚭 starts 涓?
the 鏍囧噯 modes (80x25 鍜?80x50) followed 鐢?"鐗规畩" modes (80x28 鍜?
80x43), 鏈湴 modes (鑻?the 鏈湴 modes 鐗规€?鏄?宸插惎鐢?, VESA modes 鍜?
finally SVGA modes 鐢ㄤ簬 the auto-detected adapter.

鑻?鎮?鏄?涓?happy 涓?the 妯″紡 鍒楀嚭 offered (e.g., 鑻?鎮?think 鎮ㄧ殑 鍗?
鏄?able 鍒?鎵ц 鏇村), 鎮ㄥ彲浠?enter "scan" 鑰岄潪 item 鏁板瓧 / 妯″紡 ID.  The
program 灏?try 鍒?ask the BIOS 鐢ㄤ簬 鍏ㄩ儴 鍙兘 瑙嗛 妯″紡 numbers 鍜?test
浠€涔?happens 鐒跺悗. The screen 灏?涓?probably flashing wildly 鐢ㄤ簬 涓€浜?time 鍜?
strange noises 灏?涓?heard 鏉ヨ嚜 inside the 鐩戣鍣?鍜?鍥犳 鍦?鍜?鐒跺悗, really
鍏ㄩ儴 consistent 瑙嗛 modes 鍙楁敮鎸?鐢?鎮ㄧ殑 BIOS 灏?appear (澧炲己鐗?maybe 涓€浜?
`ghost modes`). 鑻?鎮?鏄?afraid 姝?鍙互 damage 鎮ㄧ殑 鐩戣鍣? don't 浣跨敤
姝?鍑芥暟.

涔嬪悗 scanning, the 妯″紡 ordering 鏄?涓€涓?浣?涓嶅悓: the auto-detected SVGA
modes 鏄?涓?listed 鍦?鍏ㄩ儴 鍜?the modes revealed 鐢?`scan` 鏄?shown 涔嬪墠
鍏ㄩ儴 VESA modes.

#### 妯″紡 IDs


鍥犱负 鐨?the complexity 鐨?鍏ㄩ儴 the 瑙嗛 stuff, the 瑙嗛 妯″紡 IDs
浣跨敤 姝ゅ 鏄?涔?涓€涓?浣?澶嶆潅. 涓€涓?瑙嗛 妯″紡 ID 鏄?涓€涓?16-浣?鏁板瓧 閫氬父
expressed 鍦?涓€涓?hexadecimal notation (starting 涓?"0x"). 鎮ㄥ彲浠?set 涓€涓?妯″紡
鐢?entering 鍏?妯″紡 directly 鑻?鎮?know 瀹?even 鑻?瀹?isn't shown 鍦?the menu.

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
鑻?鎮?add 0x8000 鍒?the 妯″紡 ID, the program 灏?try 鍒?recalculate
vertical 鏄剧ず鍣?timing 鏍规嵁 妯″紡 鍙傛暟, 鍏?鍙?涓?浣跨敤 鍒?
eliminate 涓€浜?annoying bugs 鐨?鏌愪簺 VGA BIOSes (閫氬父 閭ｄ簺 浣跨敤 鐢ㄤ簬
鍗?涓?S3 chipsets 鍜?鏃?Cirrus Logic BIOSes) -- mainly extra lines 鍦?the
end 鐨?the 鏄剧ず鍣?

#### 閫夐」


Build 閫夐」 鐢ㄤ簬 arch/x86/boot/* 鏄?selected 鐢?the 鍐呮牳 kconfig
utility 鍜?the 鍐呮牳 .閰嶇疆 鏂囦欢.

瑙嗛_GFX_HACK - 鍖呭惈 鐗规畩 hack 鐢ㄤ簬 璁剧疆 鐨?graphics modes
鍒?涓?浣跨敤 绋嶅悗 鐢?鐗规畩 椹卞姩.
Allows 鍒?set _浠讳綍_ BIOS 妯″紡 including graphic ones 鍜?forcing 鐗瑰畾
text screen resolution 鑰岄潪 peeking 瀹?鏉ヨ嚜 BIOS variables. Don't 浣跨敤
闄ら潪 鎮?think 鎮?know 浠€涔?鎮?re doing. 鍒?activate 姝?setup, 浣跨敤
妯″紡 鏁板瓧 0x0f08 (鍙傝 the 妯″紡 IDs section 涓婃枃).

#### 浠嶇劧 doesn't work?


褰?the 妯″紡 detection doesn't work (e.g., the 妯″紡 鍒楀嚭 鏄?incorrect 鎴?
the machine hangs 鑰岄潪 displaying the menu), try 鍒?switch off 涓€浜?鐨?
the 閰嶇疆 閫夐」 listed 鍦ㄢ€︿笅 "閫夐」". 鑻?瀹?fails, 鎮ㄥ彲浠?浠嶇劧 浣跨敤
鎮ㄧ殑 鍐呮牳 涓?the 瑙嗛 妯″紡 set directly 閫氳繃 the 鍐呮牳 鍙傛暟.

鍦?浠讳竴涓?case, 璇?send me 涓€涓?bug report containing 浠€涔?_exactly_
happens 鍜?濡備綍 鎵ц the 閰嶇疆 switches affect the behaviour 鐨?the bug.

鑻?鎮?鍚姩 Linux 鏉ヨ嚜 M$-DOS, 鎮?鍙兘 涔?浣跨敤 涓€浜?DOS tools 鐢ㄤ簬
瑙嗛 妯″紡 璁剧疆. 鍦?姝?case, 鎮?蹇呴』 specify the 0x0f04 妯″紡 ("leave
鐢垫祦 璁剧疆") 鍒?Linux, 鍥犱负 鑻?鎮?don't 鍜?鎮?浣跨敤 浠讳綍 non-standard
妯″紡, Linux 灏?switch 鍒?80x25 automatically.

鑻?鎮?set 涓€浜?extended 妯″紡 鍜?閭ｉ噷's one 鎴?鏇村 extra lines 鍦?the
bottom 鐨?the 鏄剧ず鍣?containing 宸茬粡 scrolled-out text, 鎮ㄧ殑 VGA BIOS
鍖呭惈 the 澶у鏁?閫氱敤 瑙嗛 BIOS bug called "incorrect vertical 鏄剧ず鍣?
end 璁剧疆". Adding 0x8000 鍒?the 妯″紡 ID 鍙兘 fix the problem. Unfortunately,
姝?蹇呴』 涓?宸插畬鎴?manually -- 鏃?autodetection mechanisms 鏄?鍙敤.

#### History


=============== ================================================================
1.0 锛?-Nov-95)	绗竴 鐗堟湰 supporting 鍏ㄩ儴 adapters 鍙楁敮鎸?鐢?the 鏃?
		setup.S + Cirrus Logic 54XX. Present 鍦?涓€浜?1.3.4? kernels
		鍜?鐒跺悗 removed 鐢变簬 instability 鍦?涓€浜?machines.
2.0 (28-Jan-96)	Rewritten 鏉ヨ嚜 scratch. Cirrus Logic 64XX 鏀寔 added, almost
		everything 鏄?configurable, the VESA 鏀寔 搴斿綋 涓?much 鏇村
		stable, explicit 妯″紡 numbering allowed, "scan" implemented 绛?
2.1 (30-Jan-96) VESA modes moved 鍒?0x200-0x3ff. 妯″紡 selection 鐢?resolution
		鍙楁敮鎸? 灏戦噺 bugs fixed. VESA modes 鏄?listed prior 鍒?
		modes supplied 鐢?SVGA autodetection 浣滀负 瀹冧滑鏄?鏇村 reliable.
		CLGD autodetect works better. Doesn't depend 鍦?80x25 姝ｅ湪
		active 褰?started. Scanning fixed. 80x43 (浠讳綍 VGA) added.
		Code cleaned up.
2.2 (01-Feb-96)	EGA 80x43 fixed. VESA extended 鍒?0x200-0x4ff (non-standard 02XX
		VESA modes work 鐜板湪). 鏄剧ず鍣?end bug workaround 鍙楁敮鎸?
		鐗规畩 modes renumbered 鍒?鍏佽 adding 鐨?the "recalculate"
		鏍囧織, 0xffff 鍜?0xfffe became aliases 鑰岄潪 real IDs.
		Screen contents retained 鏈熼棿 妯″紡 changes.
2.3 (15-Mar-96)	Changed 鍒?work 涓?1.3.74 鍐呮牳.
2.4 (18-Mar-96)	Added patches 鐢?Hans Lermen fixing 涓€涓?鍐呭瓨 overwrite problem
		涓?涓€浜?boot loaders. 鍐呭瓨 绠＄悊 rewritten 鍒?reflect
		杩欎簺 changes. Unfortunately, screen contents retaining works
		浠?涓?涓€浜?loaders 鐜板湪.
		Added 涓€涓?Tseng 132x60 妯″紡.
2.5 (19-Mar-96)	Fixed 涓€涓?VESA 妯″紡 scanning bug introduced 鍦?2.4.
2.6 (25-Mar-96)	涓€浜?VESA BIOS 閿欒 涓?reported -- 瀹?fixes 閿欒 reports 鍦?
		鑻ュ共 鍗?涓?broken VESA code (e.g., ATI VGA).
2.7 (09-Apr-96)	- Accepted 鍏ㄩ儴 VESA modes 鍦?range 0x100 鍒?0x7ff, 鍥犱负 涓€浜?
		  鍗?浣跨敤 very strange 妯″紡 numbers.
  - Added Realtek VGA modes (thanks 鍒?Gonzalo Tornaria).
  - 纭欢 testing order slightly changed, tests 鍩轰簬 ROM
		  contents 宸插畬鎴?浣滀负 绗竴.
  - Added 鏀寔 鐢ㄤ簬 鐗规畩 瑙嗛7 妯″紡 switching 鍑芥暟
		  (thanks 鍒?Tom Vander Aa).
  - Added 480-scanline modes (especially useful 鐢ㄤ簬 notebooks,
		  original 鐗堟湰 written 鐢?hhanemaa@cs.ruu.nl, patched 鐢?
		  Jeff Chua, rewritten 鐢?me).
  - Screen store/restore fixed.
2.8 (14-Apr-96) - 鍓嶄竴涓?閲婃斁 鏇炬槸 涓?compilable 鏃?閰嶇疆_瑙嗛_SVGA.
  - Better recognition 鐨?text modes 鏈熼棿 妯″紡 scan.
2.9 (12-May-96)	- Ignored VESA modes 0x80 - 0xff (鏇村 VESA BIOS bugs!)
2.10(11-Nov-96) - The whole thing made 鍙€?
  - Added the 閰嶇疆_瑙嗛_400_HACK switch.
  - Added the 閰嶇疆_瑙嗛_GFX_HACK switch.
  - Code cleanup.
2.11(03-May-97) - 灏氭湭 another cleanup, 鐜板湪 including 涔?the documentation.
  - Direct testing 鐨?SVGA adapters turned off 榛樿鎯呭喌涓? `scan`
		  offered explicitly 鍦?the prompt line.
  - Removed the doc section describing adding 鐨?鏂?probing
		  鍑芥暟 浣滀负 I try 鍒?get rid 鐨?_鍏ㄩ儴_ 纭欢 probing 姝ゅ.
2.12(25-May-98) Added 鏀寔 鐢ㄤ簬 VESA 甯?缂撳啿鍖?graphics.
2.13(14-May-99) 娆¤ documentation fixes.
=============== ================================================================
