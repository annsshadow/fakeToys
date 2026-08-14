## Cirrus Logic 芯片组帧缓冲驱动

Copyright 1999 Jeff Garzik <jgarzik@pobox.com>


支持的芯片系列：
 - SD64
 - Piccolo
 - Picasso
 - Spectrum
 - Alpine (GD-543x/4x)
 - Picasso4 (GD-5446)
 - GD-5480
 - Laguna (GD-546x)

支持的总线：
 - PCI
 - Zorro

支持的架构：
 - i386
 - Alpha
 - PPC (Motorola Powerstack)
 - m68k (Amiga)


### 默认视频模式

目前支持两个内核命令行参数：

- mode:640x480
- mode:800x600
- mode:1024x768

对启动视频模式（modedb）的完整支持将很快集成。

### 版本 1.9.9.1

- 修复 512kB 情况下的内存检测
- 800x600 模式
- 修正时序
- 针对 AXP 的提示：更改分辨率时使用 -accel false -vyres -1


### 版本 1.9.4.4

- 初步的 Laguna 支持
- 重构颜色寄存器例程。
- 与之相关，控制台颜色现在从名为 'palette' 的 LUT 获取，而非来自 VGA 寄存器。此代码仿照 atyfb 与 matroxfb 中的实现。
- 代码清理，添加注释。
- 重构 SR07 处理。
- 缺陷修复。


### 版本 1.9.4.3

- 正确设置默认的启动视频模式。
- 不要覆盖 ram 大小设置。若确实要覆盖 RAM 设置，定义
  CLGEN_USE_HARDCODED_RAM_SETTINGS。
- 与新 2.3.x IORESOURCE_IO[PORT] 符号变更相关的编译修复。
- 使用新的 2.3.x 资源分配。
- 部分代码清理。


### 版本 1.9.4.2

- 类型转换修复。
- 断言不再故意导致 oops。
- 缺陷修复。


### 版本 1.9.4.1

- 添加兼容性支持。现在需要 2.1.x、2.2.x 或 2.3.x 内核。


### 版本 1.9.4

- 若干增强、更小的内存占用、少量缺陷修复。
- 需要内核 2.3.14-pre1 或更高版本。


### 版本 1.9.3

- 随内核 2.3.14-pre1 或更高版本一同发布。
