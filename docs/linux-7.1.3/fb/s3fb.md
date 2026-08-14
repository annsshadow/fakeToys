## s3fb - 面向 S3 Trio/Virge 芯片的 fbdev 驱动


## 支持的硬件


	S3 Trio32
	S3 Trio64（及变体 V+、UV+、V2/DX、V2/GX）
	S3 Virge （及变体 VX、DX、GX 和 GX2+）
	S3 Plato/PX		（完全未经测试）
	S3 Aurora64V+		（完全未经测试）

 - 仅支持 PCI 总线
 - 仅支持由 BIOS 初始化的 VGA 设备
 - 在大端（big endian）上可能不工作

我在 Trio64（普通、V+ 和 V2/DX）和 Virge（普通、VX、DX）上测试了 s3fb，均在 i386 上。


## 支持的特性


 - 4 bpp 伪彩色模式（带 18 位调色板，两种变体）
 - 8 bpp 伪彩色模式（带 18 位调色板）
 - 16 bpp 真彩色模式（RGB 555 与 RGB 565）
 - 24 bpp 真彩色模式（RGB 888）（仅在 Virge VX 上）
 - 32 bpp 真彩色模式（RGB 888）（不在 Virge VX 上）
 - 文本模式（由 bpp = 0 激活）
 - 隔行（interlaced）模式变体（文本模式下不可用）
 - 双扫描（doublescan）模式变体（文本模式下不可用）
 - 两个方向的平移（panning）
 - 挂起/恢复支持
 - DPMS 支持

文本模式即使在更高分辨率下也受支持，但对较低像素时钟（pixclock）有限制（最大值通常在
50-60 MHz 之间，取决于具体硬件，我在普通 S3 Trio32 卡上得到最好的结果——约 75 MHz）。
该限制并非由驱动强制。文本模式仅支持 8 位宽字体（硬件限制）和 16 位高字体（驱动限制）。
文本模式在 S3 Trio64 V2/DX 上存在问题。

有两种 4 bpp 模式。第一种模式（当 nonstd == 0 时选择）为打包像素模式，高半字节优先。
第二种模式（当 nonstd == 1 时选择）为交错平面模式（1 字节交错），MSB 优先。两种模式
都仅支持 8 位宽字体（驱动限制）。

挂起/恢复在那些在恢复时初始化显卡、且设备处于激活状态（例如正被 fbcon 使用）的系统上
工作。


## 缺失的特性


（即 TODO 列表）

 - 辅助（未经 BIOS 初始化）设备支持
 - 大端（big endian）支持
 - Zorro 总线支持
 - MMIO 支持
 - 更多卡上的 24 bpp 模式支持
 - 4 bpp 模式下 fontwidth 不等于 8 的支持
 - 文本模式下 fontheight 不等于 16 的支持
 - 复合与外接同步（有人能测试这个吗？）
 - 硬件光标
 - 视频叠加支持
 - vsync 同步
 - 功能连接器（feature connector）支持
 - 加速支持（类 8514 的 2D、Virge 3D、总线主控传输）
 - 某些 magic 寄存器更好的取值（性能问题）


## 已知缺陷


 - 文本模式下光标禁用不工作
 - 文本模式在 S3 Trio64 V2/DX 上存在问题


--
Ondrej Zajicek <santiago@crfreenet.org>
