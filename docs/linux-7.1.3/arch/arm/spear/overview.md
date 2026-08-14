## SPEAr ARM Linux 概述


### 简介


  SPEAr（Structured Processor Enhanced Architecture，结构化处理器增强架构）。
  weblink : http://www.st.com/spear

  ST Microelectronics 的 SPEAr 系列 ARM9/CortexA9 片上系统（System-on-Chip）CPU 由 ARM Linux 的 'spear' 平台支持。当前支持 SPEAr1310、SPEAr1340、SPEAr300、SPEAr310、SPEAr320 和 SPEAr600 这些 SoC。

  SPEAr 中的层级结构如下：

  SPEAr（平台）

 - SPEAr3XX（3XX SOC 系列，基于 ARM9）
  - SPEAr300（SOC）
   - SPEAr300 评估板
  - SPEAr310（SOC）
   - SPEAr310 评估板
  - SPEAr320（SOC）
   - SPEAr320 评估板
 - SPEAr6XX（6XX SOC 系列，基于 ARM9）
  - SPEAr600（SOC）
   - SPEAr600 评估板
 - SPEAr13XX（13XX SOC 系列，基于 ARM CORTEXA9）
  - SPEAr1310（SOC）
   - SPEAr1310 评估板
  - SPEAr1340（SOC）
   - SPEAr1340 评估板

### 配置


  为每台机器提供了一个通用配置，可以用作
```

	make spear13xx_defconfig
	make spear3xx_defconfig
	make spear6xx_defconfig

```
### 布局


  多个机器系列（SPEAr3xx、SPEAr6xx 和 SPEAr13xx）的公共文件位于平台代码中，包含在 arch/arm/plat-spear 中，头文件在 plat/ 中。

  每个机器系列都有一个以 arch/arm/mach-spear 加系列名命名的目录。例如 mach-spear3xx、mach-spear6xx 和 mach-spear13xx。

  spear3xx 系列机器的公共文件是 mach-spear3xx/spear3xx.c，spear6xx 的是 mach-spear6xx/spear6xx.c，spear13xx 系列的是 mach-spear13xx/spear13xx.c。mach-spear* 还包含 soc/机器特定的文件，如 spear1310.c、spear1340.c、spear300.c、spear310.c、spear320.c 和 spear600.c。mach-spear* 不包含板级特定的文件，因为它们完全支持 Flattened Device Tree。


### 文档作者


  Viresh Kumar <vireshk@kernel.org>, (c) 2010-2012 ST Microelectronics
