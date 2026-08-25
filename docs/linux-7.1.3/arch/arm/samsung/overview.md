## Samsung ARM Linux 概述


### 简

  Samsung 系列ARM SoC 涵盖了许多相似的设备，从最初的 ARM9 一直到最新的 ARM 核心。本文档给出了当前内核支持的概述、如何使用它，以及在哪里可以找到支持它的代码
  当前受支持的 SoC 有：

  - S3C64XX：S3C6400 S3C6410
  - S5PC110 / S5PV210


### 配置


  提供了若干种配置，因为目前没有办法将所SoC 统一到一个内核中
  s5pc110_defconfig
 - S5PC110 特定的默认配  s5pv210_defconfig
 - S5PV210 特定的默认配

### 布局


  目录布局目前正在进行重组，它由若干平台目录以及所构建 CPU 的机器特定目录组成
  plat-samsung 为所有实现提供基础，并且是在构建特定信息处理过程中最后处理的 include 目录。它包含使系统运行所需的基础时钟、GPIO 和设备定义
  plat-s5p 用于 s5p 特定的构建，包含S5P 特定系统的通用支持。由于硬件差异，并非所S5P 都使用此目录中的所有特性

### 布局变化


  旧的 plat-s3c plat-s5pc1xx 目录已被移除，相关支持已根据需要移动到 plat-samsung plat-s5p。这些移动是为了简化由于存在如此多不同平台目录而带来的 include 和依赖问题

### 移植贡献

  Ben Dooks (BJD)
  Vincent Sanders
  Herbert Potzl
  Arnaud Patard (RTP)
  Roc Wu
  Klaus Fetscher
  Dimitry Andric
  Shannon Holland
  Guillaume Gourat (NexVision)
  Christer Weinigel (wingel) (Acer N30)
  Lucas Correia Villa Real (S3C2400 移植)


### 文档作

Copyright 2009-2010 Ben Dooks <ben-linux@fluff.org>
