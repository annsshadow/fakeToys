## sisfb - SiS 帧缓冲设备驱动


sisfb 是用于 SiS（Silicon Integrated Systems）图形芯片的帧缓冲设备驱动。支持的有：

- SiS 300 系列：SiS 300/305、540、630(S)、730(S)
- SiS 315 系列：SiS 315/H/PRO、55x、(M)65x、740、(M)661(F/M)X、(M)741(GX)
- SiS 330 系列：SiS 330（“Xabre”）、(M)760


## 为什么需要一个帧缓冲驱动？


sisfb 例如在你想要一个高分辨率文本控制台时很有用。除此之外，sisfb 是运行 DirectFB
所必需的（DirectFB 自带一个针对 315 系列的专用驱动）。

在 300 系列上，对于早于 2.6.3 的内核，sisfb 在配合 DRM/DRI 使用中还扮演重要角色：
Sisfb 管理 DRM/DRI 用于 3D 纹理和其它数据的堆内存。使用 DRI/DRM 需要这一内存管理。

>= 2.6.3 左右的内核不再需要 sisfb 来进行 DRI/DRM 内存管理。SiS DRM 驱动已被更新，
并拥有自己的内存管理器（当 sisfb 未被编译时它会被使用）。因此除非你想要一个图形
控制台，否则在 >=2.6.3 的内核上你不需要 sisfb。

旁注：由于这似乎是一个常见的错误：sisfb 和 vesafb 不能同时处于活动状态！在你的
内核配置中只能选择其中一个。


## 参数如何传递给 sisfb？


这取决于：如果静态编译进内核，使用 lilo 的 append 语句将参数添加到内核命令行。
请参阅 lilo（或 GRUB）的文档以获取更多信息。如果 sisfb 是一个内核模块，参数通过
modprobe（或 insmod）命令给出。

sisfb 作为静态内核一部分的示例：将以下行添加到你的
```

     append="video=sisfb:mode:1024x768x16,mem:12288,rate:75"

```
```

     modprobe sisfb mode=1024x768x16 rate=75 mem=12288

```
一个常见错误是人们在将驱动编译进内核时使用了错误的参数格式。请注意：如果编译进内核，
参数格式为 video=sisfb:mode:none 或 video=sisfb:mode:1024x768x16（或任何你想使用的
模式，也可选择使用上面描述的任何其它格式，或者用 vesa 关键字代替 mode）。如果编译为
模块，参数格式为 mode=none 或 mode=1024x768x16（或任何你想使用的模式）。将 “=” 用于
“:”（反之亦然）差别巨大！此外：如果你给内核内的 sisfb 提供多于一个参数，则
```

   video=sisfb:mode:1024x768x16,rate:75,mem:12288


```
## 如何使用它？


前言：本文件仅涵盖了该驱动极少的能力和特性。请访问作者的和维护者的网站
http://www.winischhofer.net/linuxsisvga.shtml 获取更多信息。此外，“modinfo sisfb”
给出了所有受支持选项的概览，包括一些解释。

期望的显示模式可以使用关键字 “mode” 并配以以下格式之一的参数来指定：

  - XxYxDepth 或
  - XxY-Depth 或
  - XxY-Depth@Rate 或
  - XxY
  - 或简单地使用十六进制或十进制的 VESA 模式号。

例如：1024x768x16、1024x768-16@75、1280x1024-16。如果未指定深度，默认为 8。如果未
给定刷新率，默认为 60Hz。深度 32 表示 24 位色深（但 32 位帧缓冲深度，这与用户无关）。

此外，sisfb 理解关键字 “vesa” 后跟一个十进制或十六进制的 VESA 模式号。例如：vesa=791
或 vesa=0x117。请使用 “mode” 或 “vesa” 之一，但不要同时使用两者。

仅 Linux 2.4：如果未给定模式，当编译为模块时 sisfb 默认为“无模式”（mode=none）；如果
sisfb 静态编译进内核，则默认为 800x600x8，除非 CRT2 类型是 LCD，此时使用 LCD 的原生
分辨率。如果要切换到不同模式，使用 fbset shell 命令。

仅 Linux 2.6：如果未给定模式，sisfb 默认为 800x600x8，除非 CRT2 类型是 LCD，此时默认
为 LCD 的原生分辨率。如果要切换到另一种模式，使用 stty shell 命令。

你应该同时编译进 vgacon（以便在你从系统中移除 SiS 卡时启动）和 sisfb（用于图形模式）。
在 Linux 2.6 下，图形控制台还需要“Framebuffer console support”（fbcon）。

你**不**应该编译进 vesafb。并且请不要在 lilo 或 grub 的配置文件中使用 “vga=” 关键字；
模式选择通过参数形式的 “mode” 或 “vesa” 关键字完成。见上文及下文。


## X11


如果使用 XFree86 或 X.org，建议你不要使用 “fbdev” 驱动，而使用专用的 “sis” X 驱动。
“sis” X 驱动和 sisfb 由同一个人（Thomas Winischhofer）开发，并且相互配合良好。


## SVGALib


SVGALib 如果直接访问硬件，永远不能正确恢复屏幕，特别是在笔记本上，或者当输出设备是
LCD 或 TV 时。因此，在 SVGALib 配置中使用芯片集 “FBDEV”。这将使 SVGALib 使用帧缓冲
设备进行模式切换和恢复。


## 配置


（部分）可接受的选项：

=========  ==================================================================
off        禁用 sisfb。此选项仅在 sisfb 是内核内（非模块）时才被理解。
mem:X      用于控制台的内存大小，其余将用于 DRI/DRM。X 以千字节为单位。在 300 系列
	   上，默认是 4096、8192 或 16384（均为千字节），取决于卡有多少显存。在
	   315/330 系列上，默认是最大可用内存（因为这些芯片组不支持 DRI/DRM）。
noaccel    不使用 2D 加速引擎。（默认：使用加速）
noypan     禁用 y 平移（y-panning），通过重绘整个屏幕来滚动。这比 y 平移慢得多。
	   （默认：使用 y 平移）
vesa:X     选择启动视频模式。X 是从 0 到 0x1FF 的数字，表示 VESA 模式号（可以十进制
	   或十六进制形式给出，后者以 “0x” 为前缀）。
mode:X     选择启动视频模式。请参阅上文了解 “X” 的格式。
=========  ==================================================================

布尔型选项如 “noaccel” 或 “noypan” 在 sisfb 为内核内时不应带参数给出（例如
“video=sisfb:noypan”）。如果 sisfb 是模块，这些应设为 1（例如 “modprobe sisfb
noypan=1”）。


Thomas Winischhofer <thomas@winischhofer.net>

2004 年 5 月 27 日
