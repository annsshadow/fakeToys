## gxfb - AMD Geode GX2 甯х紦鍐查┍鍔。

这是面向基于 AMD Geode GX2 的处理器的图形帧缓冲驱动
优点
 - 无需BIOS 中使AMD VSA 代码（或其他 VESA 仿真层） - 它提供一个不错的大控制台024x768 128 + 48 行）   无需使用微小、难以辨认的字体 - 你可以在 /dev/fb0 之上运行 XF68_FBDev
 - 最重要的：启动 logo :-)

缺点
 - 图形模式比文本模式慢…

## 如何使用

切换模式通过 gxfb.mode_option=<分辨... 启动参数或使`fbset` 程序完成
详见 Documentation/fb/modedb.rst 了解有关 modedb 分辨率的更多信息

## X11


XF68_FBDev 通常工作良好，但它是非加速的

## 配置


你可以通过 gxfb.<option> gxfb 传递内核命令行选项例如 gxfb.mode_option=800x600@75接受的选项
================ ==================================================
mode_option	 指定视频模式。形式为
		 <x>x<y>[-<bpp>][@<refresh>]
vram		 视频内存大小（通常自动检测）
vt_switch	 在挂恢复期间启用 vt 切换。该 vt
		 切换较慢，但无害================ ==================================================

Andres Salomon <dilinger@debian.org>
