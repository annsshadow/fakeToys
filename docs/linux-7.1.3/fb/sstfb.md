## sstfb


## 简介


这是一个用于基于 3dfx 的 Voodoo Graphics（又称 voodoo 1，又称 sst1）和 Voodoo²
（又称 Voodoo 2，又称 CVG）视频卡的帧缓冲设备驱动。它是高度实验性的代码，但保证
在我的电脑上、配合我的 "Maxi Gamer 3D" 和 "Maxi Gamer 3d²" 板卡以及"我坐在椅子
和键盘之间"时能够工作。一些人测试了其他组合，看起来也能工作。
主页位于 <http://sstfb.sourceforge.net>，如果你想要最新版本，请查看 CVS，因为该
驱动仍在进行中，我对发布尚未完全可用的 tar 包感到不安……别担心，它仍然非常可用
（我吃自己做的狗粮）。

请阅读 Bugs 一节，并将任何成功或失败报告给我（Ghozlane Toumi <gtoumi@laposte.net>）。
顺便说一句，如果你只有一台显示器，又不想折腾 VGA 直通电缆，我只能建议你去借一台
屏幕……


## 安装


该驱动（应该）能在 ix86 上工作，配合"较新的" 2.2.x 内核（已用 x = 19 测试）和
"较新的" 2.4.x 内核，可作为模块或编译进内核。自臭名昭著的 2.4.10 起它已被纳入
主线内核。你可以应用 `sstfb/kernel/*-2.{2|4}.x.patch` 中的补丁，并将 sstfb.c 复制到
linux/drivers/video/，或者将单个补丁 `sstfb/patch-2.{2|4}.x-sstfb-yymmdd` 应用到
你的 linux 源代码树。

然后像往常一样配置内核：在 "console" 一节中选择 "m" 或 "y" 给 3Dfx Voodoo
Graphics。编译、安装、开心地使用……并请给我一份报告 :)


## 模块使用


       #. 在发出任何命令之前，你应该完整阅读本节。

       #. 如果你只有一台显示器可用，一旦你 insmod 该模块，3dfx 就会接管输出，因此
	  你必须将显示器插回"普通"视频板才能发出命令，或者你可以盲目使用 tools
	  目录中的 sst_dbg_vgapass（见 Tools）。最新的解决办法是在 insmod 驱动时
	  传入参数 vgapass=1。（见 Kernel/Modules Options）

### 模块插入


       #. insmod sstfb.o

	  你应该会看到板卡输出一些奇怪的画面：一个大蓝方块、一个绿色和一个红色
	  小方块，以及一个白色竖条。为什么？函数名已经说明了一切："sstfb_test()"……
	  （如果你没有第二台显示器，你将不得不把显示器直接插到 2D 显卡上才能看到
	  你输入的内容）

       #. con2fb /dev/fbx /dev/ttyx

	  将一个 tty 绑定到新的帧缓冲。如果你已经有一个帧缓冲驱动，voodoo fb 很可能
	  是 /dev/fb1。如果没有，该设备将是 /dev/fb0。你可以通过执行 cat /proc/fb
	  来检查。你可以在 tools/ 目录中找到 con2fb 的副本。如果你没有其他的 fb 设备，
	  此步骤是多余的，因为控制台子系统会自动将 tty 绑定到 fb。
       #. 切换到你刚刚映射的虚拟控制台。"tadaaa"……

### 模块移除


       #. con2fb /dev/fbx /dev/ttyx

	  将 tty 绑定回旧的帧缓冲，以便可以移除模块。
	  （它与 vgacon 如何配合工作？简短回答：它不工作）

       #. rmmod sstfb


### 内核/模块选项


你可以向 sstfb 模块传递一些选项，当驱动编译进内核时也可以通过内核命令行传递：
模块方式：insmod sstfb.o option1=value1 option2=value2 ...
内核方式：video=sstfb:option1,option2:value2,option3 ...

sstfb 支持以下选项：

=============== =============== ===============================================
Module		Kernel		Description
=============== =============== ===============================================
vgapass=0	vganopass	启用或禁用 VGA 直通电缆。
vgapass=1	vgapass		启用时，显示器将从 VGA 板而非 voodoo 获取信号。

				Default: nopass

mem=x		mem:x		强制设置帧缓冲内存大小（单位 MiB）
				允许的值：0、1、2、4。

				Default: 0（= 自动检测）

inverse=1	inverse		本应启用反色控制台。
				尚不能工作……

clipping=1	clipping	启用或禁用裁剪。
clipping=0	noclipping	启用裁剪后，所有屏外读写都将被丢弃。

				Default: 启用裁剪。

gfxclk=x	gfxclk:x	强制设置图形时钟频率（单位 MHz）。
				小心使用此选项，它可能很危险。

				Default: auto

     - Voodoo 1 为 50Mhz，
     - Voodoo 2 为 75MHz。

slowpci=1	fastpci		启用或禁用快速 PCI 读/写。
slowpci=1	slowpci		Default : fastpci

dev=x		dev:x		将驱动附加到设备编号 x。
				0 是第一块兼容板卡（按 lspci 顺序）
=============== =============== ===============================================

## 工具


这些工具大多用于调试目的，但你可能会觉得其中一些很有意思：

```

	con2fb /dev/fb1 /dev/tty5

```
- `sst_dbg_vgapass`，更改 VGA 直通。你需要重新编译
```

	sst_dbg_vgapass /dev/fb1 1 (enables vga cable)
	sst_dbg_vgapass /dev/fb1 0 (disables vga cable)

```
- `glide_reset`，使用 glide 重置 voodoo。在 rmmod 掉 sstfb 后，如果模块拒绝重新
  插入，可使用它。

## 缺陷


- 在 sstfb 模块加载时不要使用 glide，你极有可能会让电脑挂起。
- 如果你看到一些伪影（像素没有清干净之类），尝试关闭裁剪（clipping=0），和/或使用
  slowpci。
- 驱动无法检测 4Mb 帧缓冲的 voodoo，似乎最后 2MB 会回绕。正在研究。
- 该驱动仅支持 16 bpp，24/32 不能工作。
- 该驱动并非 your_favorite_toy 安全，这包括 SMP……

	[实际上从代码看似乎应该是安全的 - Alan]

- 当使用 XFree86 FBdev（基于 fbdev 的 X）时，你可能会在窗口边框看到奇怪的颜色
  图案（像素丢失了最低字节 -> 基本上是蓝色分量和部分绿色分量）。我无法用 XFree86-3.3
  复现此问题，但其中一名测试者在 XFree86-4 上遇到了此问题。显然较新的 Xfree86-4.x
  解决了此问题。
- 我没有真正测试过更改调色板，所以你在玩这个时可能会发现一些奇怪的现象。
- 有时驱动无法识别 DAC，导致初始化失败。对于 voodoo 2 板卡尤其如此，但应该在
  较新版本中解决。请联系我。
- 24/32 在短时间内不太可能工作，因为硬件在 24/32 bpp 下会做……不寻常的事情。

## 待办


- 去掉上一段。
- 买更多咖啡。
- 测试/移植到其他架构。
- 尝试利用前后缓冲区的调整来添加平移。
- 尝试在 voodoo2 上实现加速，尽管它是作为纯 3D 板卡出售的，但它实际上能做很多 2D
  工作……

Ghozlane Toumi <gtoumi@laposte.net>


Date: 2002/05/09 20:11:45

http://sstfb.sourceforge.net/README
