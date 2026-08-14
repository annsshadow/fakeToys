## S3 恢复时的视频问题


2003-2006，Pavel Machek

在 S3 恢复过程中，硬件需要被重新初始化。对于大多数设备而言这很简单，
内核驱动也知道如何完成。不幸的是存在一个例外：显卡。显卡通常由 BIOS
初始化，而内核并没有足够信息来启动显卡（内核通常甚至不包含显卡驱动
——vesafb 和 vgacon 被广泛使用）。

对 swsusp 来说这不是问题，因为在 swsusp 恢复期间 BIOS 会正常运行，显卡
也就被正常初始化了。对 S1 待机而言也不应当是问题，因为硬件状态应当会在
S1 期间得以保持。

我们既可以在恢复早期运行视频 BIOS，也可以稍后用 vbetool 来解释它，又或者
在某些特定系统上可能什么都不需要做，因为视频状态被保留了。遗憾的是，不同
的方法在不同系统上有效，并且没有哪种已知方法能适用于全部系统。

一个名为 s2ram 的用户态程序已被开发出来；它包含一份很长的系统白名单，
并能针对给定系统自动选择可用的恢复方法。它可以从
www.sf.net/projects/suspend 的 CVS 处下载。如果你遇到一台不在白名单中的
系统，请试着找出可用的解决方案，并提交白名单条目，以免他人重复劳动。

目前，VBE_SAVE 方法（下文第 6 种）在大多数系统上有效。遗憾的是，vbetool
只能在用户态恢复之后运行，因此这使得对恢复早期问题的调试变得困难甚至
不可能。不依赖用户态的方法更可取。

#### 细节


在 S3 恢复后视频可用的系统有下面几种类型：

(1) 视频状态在 S3 期间被保留的系统。

(2) 可以在 S3 恢复期间调用视频 BIOS 的系统。遗憾的是，在该时刻调用视频
    BIOS 并不正确，但它碰巧能在某些机器上工作。请使用
    acpi_sleep=s3_bios。

(3) 将显卡初始化为 VGA 文本模式、且 BIOS 足以设置视频模式的系统。请在这些
    系统上使用 acpi_sleep=s3_mode。

(4) 在某些系统上 s3_bios 会把视频踢回文本模式，此时需要
    acpi_sleep=s3_bios,s3_mode。

(5) radeon 系统，其中 X 可以软启动你的显卡。你需要足够新的 X 以及一个纯
    文本控制台（不要使用 vesafb 或 radeonfb）。更多信息参见
    http://www.doesi.gmxhome.de/linux/tm800s3/s3.html 。
    或者，你也可以改用 vbetool（第 6 种）。

(6) 其他 radeon 系统，其中 vbetool 足以让系统恢复生机。它需要可用的文本
    控制台。执行 vbetool vbestate save > /tmp/delme; echo 3 > /proc/acpi/sleep;
    vbetool post; vbetool vbestate restore < /tmp/delme; setfont <whatever>，
    你的视频就应该能正常工作了。

(7) 在某些系统上，可以让内核大部分启动，然后 POST BIOS 即可工作。Ole Rohne
    有一个补丁可以做到这一点，位于
    http://dev.gentoo.org/~marineam/patch-radeonfb-2.6.11-rc2-mm2 。

(8) 在某些系统上，你可以使用 video_post 工具，或执行
    echo 3 > /sys/power/state && /usr/sbin/video_post —— 这会将显示初始化为
    控制台模式。如果你在 X 下，可以切换到虚拟终端再切回 X（使用
    CTRL+ALT+F1 - CTRL+ALT+F7）来让显示重新进入图形模式。

现在，如果你传入 acpi_sleep=something，而它在你的 BIOS 上不工作，你会在
恢复期间遇到硬崩溃。请小心。此外，最安全的做法是用老式的纯 VGA 控制台来
进行实验。vesafb 和 radeonfb（等）驱动有在恢复期间让机器崩溃的倾向。

你可能会遇到上述方法都不适用的系统。此时你要么再发明一个能用的丑陋 hack，
要么为你的显卡编写合适的驱动（祝你能拿到文档 :-)。也许从 X（真正了解你
硬件的 X，而非 XF68_FBcon）挂起成功的机会更大。

已知可用的笔记本列表：


=============================== ===============================================
Model                           hack (or "how to do it")
=============================== ===============================================
Acer Aspire 1406LC		ole's late BIOS init (7), turn off DRI
Acer TM 230			s3_bios (2)
Acer TM 242FX			vbetool (6)
Acer TM C110			video_post (8)
Acer TM C300                    vga=normal (only suspend on console, not in X),
				vbetool (6) or video_post (8)
Acer TM 4052LCi		        s3_bios (2)
Acer TM 636Lci			s3_bios,s3_mode (4)
Acer TM 650 (Radeon M7)		vga=normal plus boot-radeon (5) gets text
				console back
Acer TM 660			??? [#f1]_
Acer TM 800			vga=normal, X patches, see webpage (5)
				or vbetool (6)
Acer TM 803			vga=normal, X patches, see webpage (5)
				or vbetool (6)
Acer TM 803LCi			vga=normal, vbetool (6)
Arima W730a			vbetool needed (6)
Asus L2400D                     s3_mode (3) [#f2]_ (S1 also works OK)
Asus L3350M (SiS 740)           (6)
Asus L3800C (Radeon M7)		s3_bios (2) (S1 also works OK)
Asus M6887Ne			vga=normal, s3_bios (2), use radeon driver
				instead of fglrx in x.org
Athlon64 desktop prototype	s3_bios (2)
Compal CL-50			??? [#f1]_
Compaq Armada E500 - P3-700     none (1) (S1 also works OK)
Compaq Evo N620c		vga=normal, s3_bios (2)
Dell 600m, ATI R250 Lf		none (1), but needs xorg-x11-6.8.1.902-1
Dell D600, ATI RV250            vga=normal and X, or try vbestate (6)
Dell D610			vga=normal and X (possibly vbestate (6) too,
				but not tested)
Dell Inspiron 4000		??? [#f1]_
Dell Inspiron 500m		??? [#f1]_
Dell Inspiron 510m		???
Dell Inspiron 5150		vbetool needed (6)
Dell Inspiron 600m		??? [#f1]_
Dell Inspiron 8200		??? [#f1]_
Dell Inspiron 8500		??? [#f1]_
Dell Inspiron 8600		??? [#f1]_
eMachines athlon64 machines	vbetool needed (6) (someone please get
				me model #s)
HP NC6000			s3_bios, may not use radeonfb (2);
				or vbetool (6)
HP NX7000			??? [#f1]_
HP Pavilion ZD7000		vbetool post needed, need open-source nv
				driver for X
HP Omnibook XE3	athlon version	none (1)
HP Omnibook XE3GC		none (1), video is S3 Savage/IX-MV
HP Omnibook XE3L-GF		vbetool (6)
HP Omnibook 5150		none (1), (S1 also works OK)
IBM TP T20, model 2647-44G	none (1), video is S3 Inc. 86C270-294
				Savage/IX-MV, vesafb gets "interesting"
				but X work.
IBM TP A31 / Type 2652-M5G      s3_mode (3) [works ok with
				BIOS 1.04 2002-08-23, but not at all with
				BIOS 1.11 2004-11-05 :-(]
IBM TP R32 / Type 2658-MMG      none (1)
IBM TP R40 2722B3G		??? [#f1]_
IBM TP R50p / Type 1832-22U     s3_bios (2)
IBM TP R51			none (1)
IBM TP T30	236681A		??? [#f1]_
IBM TP T40 / Type 2373-MU4      none (1)
IBM TP T40p			none (1)
IBM TP R40p			s3_bios (2)
IBM TP T41p			s3_bios (2), switch to X after resume
IBM TP T42			s3_bios (2)
IBM ThinkPad T42p (2373-GTG)	s3_bios (2)
IBM TP X20			??? [#f1]_
IBM TP X30			s3_bios, s3_mode (4)
IBM TP X31 / Type 2672-XXH      none (1), use radeontool
				(http://fdd.com/software/radeon/) to
				turn off backlight.
IBM TP X32			none (1), but backlight is on and video is
				trashed after long suspend. s3_bios,
				s3_mode (4) works too. Perhaps that gets
				better results?
IBM Thinkpad X40 Type 2371-7JG  s3_bios,s3_mode (4)
IBM TP 600e			none(1), but a switch to console and
				back to X is needed
Medion MD4220			??? [#f1]_
Samsung P35			vbetool needed (6)
Sharp PC-AR10 (ATI rage)	none (1), backlight does not switch off
Sony Vaio PCG-C1VRX/K		s3_bios (2)
Sony Vaio PCG-F403		??? [#f1]_
Sony Vaio PCG-GRT995MP		none (1), works with 'nv' X driver
Sony Vaio PCG-GR7/K		none (1), but needs radeonfb, use
				radeontool (http://fdd.com/software/radeon/)
				to turn off backlight.
Sony Vaio PCG-N505SN		??? [#f1]_
Sony Vaio vgn-s260		X or boot-radeon can init it (5)
Sony Vaio vgn-S580BH		vga=normal, but suspend from X. Console will
				be blank unless you return to X.
Sony Vaio vgn-FS115B		s3_bios (2),s3_mode (4)
Toshiba Libretto L5		none (1)
Toshiba Libretto 100CT/110CT    vbetool (6)
Toshiba Portege 3020CT		s3_mode (3)
Toshiba Satellite 4030CDT	s3_mode (3) (S1 also works OK)
Toshiba Satellite 4080XCDT      s3_mode (3) (S1 also works OK)
Toshiba Satellite 4090XCDT      ??? [#f1]_
Toshiba Satellite P10-554       s3_bios,s3_mode (4) [#f3]_
Toshiba M30                     (2) xor X with nvidia driver using internal AGP
Uniwill 244IIO			??? [#f1]_
=============================== ===============================================

#### 已知可用的台式机系统


=================== ============================= ========================
Mainboard	    Graphics card                 hack (or "how to do it")
=================== ============================= ========================
Asus A7V8X	    nVidia RIVA TNT2 model 64	  s3_bios,s3_mode (4)
=================== ============================= ========================


         应使用哪些选项。如果你知道，请告诉我。
