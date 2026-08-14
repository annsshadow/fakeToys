## aty128fb - ATI Rage128 帧缓冲驱动


这是一款面向基于 ATI Rage128 设备的图形帧缓冲驱动，适用于 Intel 与 PPC 机器。

优点：

 - 无需使用细小、难以辨认的字体，即可提供一个漂亮的大号控制台（1024x768 下 128 列 + 48 行）。
 - 你可以在 /dev/fb0 之上运行 XF68_FBDev。
 - 最重要的是：启动 logo :-)

缺点：

 - 图形模式比文本模式慢……但如果你使用的分辨率与文本模式相同，应该不会察觉到。
 - 仍处于实验阶段。


## 如何使用？


切换模式可通过 `video=aty128fb:<resolution>...` modedb 引导参数或使用 `fbset` 程序完成。

有关 modedb 分辨率的更多信息，请参阅 Documentation/fb/modedb.rst。

你应当同时编译进 vgacon（以便在你从机器中移除 Rage128 时启动）与 aty128fb（用于图形模式）。除非你的主显示位于非 Rage128 的 VBE2.0 设备上（详见 Documentation/fb/vesafb.rst），否则不应编译进 vesafb。


## X11


XF68_FBDev 通常工作良好，但没有加速。截至本文档撰写时，8 与 32bpp 工作正常。从 X 切换到控制台再切回 X 时曾出现过调色板问题，你需要重启 X 来修复。


## 配置


你可以通过 `video=aty128fb:option1,option2:value2,option3`（多个选项用逗号分隔，值与选项用 `:` 分隔）向 vesafb 传递内核命令行选项。可接受的选项：

========= =======================================================
noaccel   不使用加速引擎。此为默认。
accel     使用加速引擎。尚未完成。
vmode:x   选择 PowerMacintosh 视频模式 <x>。已废弃。
cmode:x   选择 PowerMacintosh 颜色模式 <x>。已废弃。
<XxX@X>   选择启动视频模式。详细说明见 modedb.txt。默认为 640x480x8bpp。
========= =======================================================


## 限制


存在已知与未知的 bug、特性与反特性。当前已知的 bug 如下：

 - 该驱动仍处于实验阶段且尚未完成。bug/勘误表过多，无法在此一一列出。

Brad Douglas <brad@neruo.com>
