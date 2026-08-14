## Sony 可编程 I/O 控制设备驱动自述


 - Copyright (C) 2001-2004 Stelian Pop <stelian@popies.net>
 - Copyright (C) 2001-2002 Alcôve <www.alcove.com>
 - Copyright (C) 2001 Michael Ashley <m.ashley@unsw.edu.au>
 - Copyright (C) 2001 Junichi Morita <jun1m@mars.dti.ne.jp>
 - Copyright (C) 2000 Takaya Kinjo <t-kinjo@tc4.so-net.ne.jp>
 - Copyright (C) 2000 Andrew Tridgell <tridge@samba.org>

该驱动提供对 Sony 可编程 I/O 控制设备的访问，该设备可在许多 Sony Vaio 笔记本中找到。
一些较新的 Sony 笔记本（似乎仅限于新的 FX 系列笔记本，至少 FX501 与 FX702）缺少
sonypi 设备，因此完全不受该驱动支持。

它将提供（通过一个用户态工具）对那些笔记本所产生的某些事件的访问，例如：

 - 滚轮（jogdial）事件（Vaio 侧面上的小轮）
 - 拍照按钮事件（仅限 Vaio Picturebook 系列）
 - Fn 键
 - 蓝牙按钮（仅限 C1VR 型号）
 - 可编程键、返回、帮助、缩放、拇指短语按钮等。
	  （当可用时）

这些事件（见 linux/sonypi.h）可以使用字符设备节点 /dev/sonypi（主设备号 10，次设备号
自动分配或作为选项指定）来轮询。一个简单的将滚轮移动转换为鼠标滚轮事件的守护进程
可在以下地址下载：<http://popies.net/sonypi/>

拦截这些事件的另一种方式是直接通过 input 层获取它们。

该驱动还支持一些 ioctl 命令，用于设置 LCD 屏幕亮度以及查询电池充电信息（未来可能会
添加更多命令）。

该驱动也可用于设置 Picturebook 系列（亮度、对比度等）上的摄像头控制，并被用于 Motion
Eye 摄像头的 video4linux 驱动。

请注意，该驱动是通过对 Windows 驱动与 ACPI BIOS 进行逆向工程而创建的，因为 Sony 不同意
发布其笔记本的任何编程规范。如果有人说服他们这样做，请给我留个言。

### 驱动选项：


可以使用标准模块参数语法（向模块传递选项时为 <param>=<value>，或在 sonypi 静态链接
进内核时于内核引导行上使用 sonypi.<param>=<value>）向 sonypi 驱动传递若干选项。这些
选项是：

	=============== =======================================================
	minor:		字符设备 /dev/sonypi 的次设备号，
			默认是 -1（自动分配，见 /proc/misc 或内核日志）

	camera:		如果你有一台 PictureBook 系列 Vaio（带集成的 MotionEye
			摄像头），将此参数设为 1 以让驱动访问摄像头

	fnkeyinit:	在一些 Vaio（C1VE、C1VR 等）上，除非将此参数设为 1，
			Fn 键事件不会被启用。除非确实必要，否则不要使用此选项，
			某些 Vaio 型号对此选项处理得不好。此选项仅在内核未编译
			ACPI 支持时可用（因为它与 ACPI 冲突，而且如果 ACPI 已
			启用，本来也不应需要它）。

	verbose:	设为 1 以打印从 sonypi 设备收到的未知事件。
			设为 2 以打印从 sonypi 设备收到的所有事件。

	compat:		使用一些兼容性代码来启用 sonypi 事件。如果驱动过去
			（1.5 版本之前）对你可用而现在不再工作，请添加此选项
			并向作者报告。

	mask:		事件掩码，告诉驱动哪些事件将被报告给用户。某些 Vaio 型号
			需要此参数，因为硬件复用了其它 Vaio 型号使用的值（例如
			FX 系列没有滚轮，但将滚轮事件复用于可编程键事件）。默认
			事件掩码设为 0xffffffff，意味着将尝试所有可能的事件。你
			可以使用以下位来构造自己的事件掩码（来自
```
				SONYPI_JOGGER_MASK		0x0001
				SONYPI_CAPTURE_MASK		0x0002
				SONYPI_FNKEY_MASK		0x0004
				SONYPI_BLUETOOTH_MASK		0x0008
				SONYPI_PKEY_MASK		0x0010
				SONYPI_BACK_MASK		0x0020
				SONYPI_HELP_MASK		0x0040
				SONYPI_LID_MASK			0x0080
				SONYPI_ZOOM_MASK		0x0100
				SONYPI_THUMBPHRASE_MASK		0x0200
				SONYPI_MEYE_MASK		0x0400
				SONYPI_MEMORYSTICK_MASK		0x0800
				SONYPI_BATTERY_MASK		0x1000
				SONYPI_WIRELESS_MASK		0x2000

	useinput:	如果设置（此为默认值），将创建两个 input 设备，一个将
			滚轮事件解释为鼠标事件，另一个则表现得像一个键盘，
			报告特殊按键的按下。
	=============== =======================================================

```
### 模块使用：


为了在使用时自动加载 sonypi 模块，你可以放入以下
```
	alias char-major-10-250 sonypi
	options sonypi minor=250

```
```
	# mknod /dev/sonypi c 10 250

```
### 缺陷：


 - 若干用户报告该驱动禁用了 BIOS 管理的 Fn 键，这些键会将笔记本置于睡眠状态，或
	  切换外接显示器开/关。目前尚无变通办法，因为该驱动通过启用 ACPI 管理（而 ACPI
	  核心部分尚不完整）禁用了这些键的所有 APM 管理。如果你有一台这些 Fn 键可用且
	  想继续使用的笔记本，请不要使用此驱动。

 - 一些用户报告在使用带 fnkeyinit 参数的驱动时笔记本速度较低（dhrystone 测试）。
	  我无法在我的笔记本上复现，也并非所有用户都有此问题。发生这种情况是因为 fnkeyinit
	  参数启用了 ACPI 模式（但没有额外的 ACPI 控制，例如处理器速度处理等）。如果它
	  在你的笔记本上可用，请使用 ACPI 而非 APM。

 - sonypi 缺乏在某些型号上区分特定按键事件的能力。

 - 一些带 nvidia 显卡（geforce go 6200 tc）的型号使用不同的方式来调节屏幕背光。
	  有一个用户态工具可用于在这些型号上调节亮度，可从以下地址下载：
	  https://www.acc.umu.se/~erikw/program/smartdimmer-0.1.tar.bz2

 - 由于所有开发都是通过逆向工程完成的，因此**绝对不保证**该驱动不会令你的笔记本
	  崩溃。永久性地。
