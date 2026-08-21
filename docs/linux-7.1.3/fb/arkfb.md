## arkfb - 用于 ARK Logic 芯片fbdev 驱动


## 支持的硬

	ARK 2000PV 芯片
	ICS 5342 ramdac

 - 仅支持由 BIOS 初始化的 VGA 设备
 - 在大端（big endian）上可能不工

## 支持的特

 - 4 bpp 伪彩色模式（18 位调色板，两种变体）
 - 8 bpp 伪彩色模式（18 位调色板 - 16 bpp 真彩色模式（RGB 555 RGB 565 - 24 bpp 真彩色模式（RGB 888 - 32 bpp 真彩色模式（RGB 888 - 文本模式（由 bpp = 0 激活）
 - 双倍扫描（doublescan）模式变体（文本模式下不可用 - 双向平移（panning - 挂起/恢复支持

文本模式即使在更高分辨率下也受支持，但对较低的像素时钟（pixclock）有限制（我得到的最大约70 MHz，这取决于具体硬件）。该限制不是由驱动强制的。文本模式仅支持 8 位宽的字体（硬件限制）和 16 位高的字体（驱动限制）。不幸的是，文本模式下的字符属性（如颜色）由于未知原因已损坏，因此其可用性有限
有两4 bpp 模式。第一种模式（nonstd == 0 则选择）是打包像素（packed pixels）模式，高半字节（nibble）在前。第二种模式（若 nonstd == 1 则选择）是交错平面（interleaved planes）模式（1 字节交错），MSB 在前。两种模式都仅支8 位宽的字体（驱动限制）
挂起/恢复在以下系统上有效：在恢复期间初始化显卡，且设备处于活动状态（例如fbcon 使用）

## 缺失的特

（即 TODO 列表
 - 次级（不BIOS 初始化的）设备支 - 大端（big endian）支 - DPMS 支持
 - MMIO 支持
 - 隔行扫描（interlaced）模式变 - 4 bpp 模式下支fontwidth != 8
 - 在文本模式下支持 fontheight != 16
 - 硬件光标
 - vsync 同步
 - 特性连接器（feature connector）支 - 加速支持（8514 2D

## 已知 bug


 - 文本模式下的字符属性（和光标）已损
--
Ondrej Zajicek <santiago@crfreenet.org>
