## STi ARM Linux 概述


### 简介


  ARM Linux 的 'STi' 平台支持意法半导体（ST Microelectronics）基于 CortexA9 的 Multimedia and Application Processors 系列片上系统（System-on-Chip）。目前支持 STiH407、STiH410 和 STiH418。

### 配置


  STi 平台的配置通过 multi_v7_defconfig 提供支持。

### 布局


  多个机器系列（STiH407、STiH410 和 STiH418）的所有文件都位于 arch/arm/mach-sti 内的平台代码中。

  mach 文件夹中有一个通用的 board-dt.c，支持扁平设备树（Flattened Device Tree），这意味着它可以与任何兼容的设备树板卡配合工作。

### 文档作者


  Srinivas Kandagatla <srinivas.kandagatla@st.com>, (c) 2013 ST Microelectronics
