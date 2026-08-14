## Intel 810/815 帧缓冲驱动


Tony Daplas <adaplas@pol.net>

http://i810fb.sourceforge.net

March 17, 2002

First Released: July 2001
Last Update:    September 12, 2005

## A. 简介


	这是一个用于各种兼容 Intel 810/815 的图形设备的帧缓冲驱动。这些设备包括：

 - Intel 810
 - Intel 810E
 - Intel 810-DC100
 - Intel 815 仅板载图形，100Mhz FSB
 - Intel 815 仅板载图形
 - Intel 815 板载图形与 AGP

## B. 特性


 - 可选择使用离散视频时序（Discrete Video Timings）、VESA 通用时序公式
	  （Generalized Timing Formula）或帧缓冲专用数据库来设置视频模式

 - 若启用 VESA 通用时序公式，支持可变范围的水平/垂直分辨率与垂直刷新率。

 - 支持 8、16、24 和 32 位每像素的色深

 - 支持伪彩色（pseudocolor）、直接彩色（directcolor）或真彩色（truecolor）视觉

 - 在 8、16 和 24 bpp 下提供完整且优化的硬件加速

 - 稳健的视频状态保存与恢复

 - 支持 MTRR

 - 利用用户输入的显示器规格自动计算所需的视频模式参数。

 - 可与使用原生 i810 驱动的 XFree86 同时运行

 - 支持硬件光标

 - 支持通过 DDC/I2C 或 BIOS 进行 EDID 探测

## C. 可用选项列表


   a. "video=i810fb"
	启用 i810 驱动

	建议：必选

   b. "xres:<value>"
	以像素为单位选择水平分辨率。（若指定了 ‘mode_option’，此参数将被忽略。
	见下文 ‘o’。）

	建议：用户偏好
	（默认 = 640）

   c. "yres:<value>"
	以扫描线为单位选择垂直分辨率。若启用了离散视频时序，此参数将被忽略，
	并按 3*xres/4 计算。（若指定了 ‘mode_option’，此参数将被忽略。见下文 ‘o’）

	建议：用户偏好
	（默认 = 480）

   d. "vyres:<value>"
	以扫描线为单位选择虚拟垂直分辨率。若指定为 (0) 或无，将依据最大可用内存计算。

	建议：不设置
	（默认 = 480）

   e. "vram:<value>"
	选择为显存分配的系统 RAM 大小（单位 MB）

	建议：1 - 4 MB。
	（默认 = 4）

   f. "bpp:<value>"
	选择期望的像素深度

	建议：8
	（默认 = 8）

   g. "hsync1/hsync2:<value>"
	以 kHz 为单位选择显示器水平同步频率的最小值与最大值。若使用固定频率显示器，
	hsync1 必须等于 hsync2。若 EDID 探测成功，这些值将被忽略，并取自 EDID 块。

	建议：查阅显示器手册以获取正确值
	（默认 = 29/30）

   h. "vsync1/vsync2:<value>"
	以 Hz 为单位选择显示器垂直同步频率的最小值与最大值。你也可以利用此选项锁定
	显示器的刷新率。若 EDID 探测成功，这些值将被忽略，并取自 EDID 块。

	建议：查阅显示器手册以获取正确值
	（默认 = 60/60）

	重要：若需要对时序进行钳制，请为计算误差（上溢/下溢）留出余量。例如：
	若使用 vsync1/vsync2 = 60/60，请确保 hsync1/hsync2 至少相差 1 个单位，反之亦然。

   i. "voffset:<value>"
	选择以 MB 为单位的逻辑内存偏移处分配帧缓冲内存。其目的是避开标准图形应用
	程序（XFree86）使用的内存块。默认偏移（64 MB 孔径为 16 MB，32 MB 孔径为 8 MB）
	可避免 XFree86 占用，并允许最多 7 MB/15 MB 的帧缓冲内存。根据你的使用情况，
	将该值调高或调低（0 表示最大用量，31/63 MB 表示最小用量）。注意，任意设置
	可能与 XFree86 冲突。

	建议：不设置
	（默认 = 8 或 16 MB）

   j. "accel"
	启用文本加速。可随时使用 ‘fbset -accel true/false’ 启用/重新启用。

	建议：启用
	（默认 = 未设置）

   k. "mtrr"
	启用 MTRR。这允许以突发方式向帧缓冲内存传输数据，可显著提升性能。由于
	“共享内存”，对 i810/i815 帮助不大。

	建议：不设置
	（默认 = 未设置）

   l. "extvga"
	若指定，次级/外部 VGA 输出将始终启用。当未连接显示器时 BIOS 关闭 VGA 端口时
	很有用。这样可以在不重启的情况下连接外部 VGA 显示器。

	建议：不设置
	（默认 = 未设置）

   m. "sync"
	强制硬件引擎执行 “sync” 或等待硬件完成后再开始下一条指令。这会提高稳定性，
	但速度更慢。

	建议：不设置
	（默认 = 未设置）

   n. "dcolor"
	对于大于 8 bpp 的像素深度，使用直接彩色（directcolor）视觉而非真彩色
	（truecolor）。便于颜色调校，例如伽马控制。

	建议：不设置
	（默认 = 未设置）

   o. <xres>x<yres>[-<bpp>][@<refresh>]
	驱动现在接受启动模式选项的指定。若指定了此项，‘xres’ 和 ‘yres’ 选项将被
	忽略。用法参见 Documentation/fb/modedb.rst。

## D. 内核启动


用逗号（,）分隔各个选项/选项对，选项与值之间用冒号分隔：
```

	video=i810fb:option1,option2:value2

```
### 使用示例


```

  append="video=i810fb:vram:2,xres:1024,yres:768,bpp:8,hsync1:30,hsync2:55, \
	  vsync1:50,vsync2:85,accel,mtrr"

```
这会将帧缓冲初始化为 1024x768、8bpp。帧缓冲将使用 2 MB 系统 RAM。将启用 MTRR
支持。刷新率将基于 hsync1/hsync2 和 vsync1/vsync2 的值计算。

重要：
  你必须包含 hsync1、hsync2、vsync1 和 vsync2 才能启用优于 640x480@60Hz 的视频
  模式。然而，若你的芯片组/显示组合支持 I2C 并带有 EDID 块，可以安全地省略
  hsync1、hsync2、vsync1 和 vsync2 参数。这些参数将取自 EDID 块。

## E. 模块选项


模块参数本质上与内核参数类似。主要区别在于，对于那些不需要值的选项，你需要提供
一个布尔值（1 表示 TRUE，0 表示 FALSE）。

例如，要启用 MTRR，可包含 “mtrr=1”。

### 使用示例


```

	modprobe i810fb vram=2 xres=1024 bpp=8 hsync1=30 hsync2=55 vsync1=50 \
		 vsync2=85 accel=1 mtrr=1

```
```

	options i810fb vram=2 xres=1024 bpp=16 hsync1=30 hsync2=55 vsync1=50 \
	vsync2=85 accel=1 mtrr=1

```
```

	modprobe i810fb


```
## F. 配置


	a. 按你惯常的方式配置内核

	   make menuconfig/xconfig/config

	b. 在 “代码成熟度选项（Code maturity level options）” 下，启用 “提示开发中和/
	   或不完整的代码/驱动（Prompt for development and/or incomplete code/drivers）”。

	c. 为 Intel 810/815 板载图形启用 agpgart 支持。这是必需的。该选项位于
	   “字符设备（Character Devices）” 下。

	d. 在 “图形支持（Graphics Support）” 下，静态选择或作为模块选择 “Intel 810/815”。
	   若需最大化显示器能力，选择 “使用 VESA 通用时序公式（use VESA Generalized
	   Timing Formula）”。为稳妥起见，也可以不选择此项。

	e. 若需要 DDC/I2C 探测（即插即用显示器）支持，将 ‘Enable DDC Support’ 设为 ‘y’。
	   要使该选项出现，需将 ‘use VESA Generalized Timing Formula’ 设为 ‘y’。

	f. 若需要帧缓冲控制台，在 “控制台驱动（Console Drivers）” 下启用它。

	g. 编译你的内核。

	h. 按 D 和 E 节的说明加载驱动。

	i. 试用 DirectFB（http://www.directfb.org）+ i810 gfxdriver 补丁来观察该芯片组的
	   实际表现（或不表现 :-)。

## G. 致谢：


 1. Geert Uytterhoeven —— 他出色的 howto 与虚拟帧缓冲驱动代码使之成为可能。

 2. Jeff Hartmann 提供的 agpgart 代码。

 3. X 开发者。仅通过阅读 XFree86 源代码就获得了很多见解。

 4. Intel(c)。感谢这款注重性价比的芯片组驱动以及所提供的文档。

 5. Matt Sottek。他的投入与想法帮助实现了一些优化。

## H. 主页：


	更完整且可能已更新的信息请见 http://i810fb.sourceforge.net。

Tony
