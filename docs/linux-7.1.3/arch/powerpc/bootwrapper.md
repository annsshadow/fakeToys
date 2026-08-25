## PowerPC 引导包装

Copyright (C) Secret Lab Technologies Ltd.

PowerPC 镜像目标会使用引导包装器（boot wrapper）压缩并封装内核镜像
（vmlinux），使其能被系统固件使用。由于不存在标准PowerPC 固件接口引导包装器被设计成可适配每一种需要构建的镜像类型
引导包装器可arch/powerpc/boot/ 目录中找到。该目录下的 Makefile 包含所有可用镜像类型的目标。不同的镜像类型用于支持 PowerPC 平台上各种固件接口OpenFirmware Apple、IBM 及其他厂商的通用 PowerPC 系统上最常用的固件类型U-Boot 通常出现在嵌入式 PowerPC 硬件上，但也有少数其他固件实现同样流行每种固件接口都需要不同的镜像格式
引导包装器由 arch/powerpc/boot/Makefile 中的 makefile 构建，它使用 wrapper 脚本
（arch/powerpc/boot/wrapper）来生成目标镜像。构建系统的细节将在下一节讨论目前存在以下镜像格式目标
   ==================== ========================================================
   cuImage.%:		用于旧版 U-Boot（不支持设备树的版本）的向后兼容
			uImage。该镜像在内部嵌入了一个设备树 blob。引			包装器、内核与设备树全部嵌入在 U-Boot uImage 文件
			格式中，其中的引导包装器代码会从旧的 bd_info 结构
			中提取数据，并在跳入内核之前将其加载进设备树
			由于U-Boot 接口中使用的 bd_info 结构里存在一系列
			#ifdef，cuImage 是平台相关的。每个特定的 U-Boot 平台
			都有不同的平台初始化文件，它用来自平台特定的 bd_info
			文件的数据填充嵌入的设备树。平台特定的 cuImage 平台
			初始化代码可`arch/powerpc/boot/cuboot.*.c` 中找到			为特定板卡选择正确cuImage 初始化代码可wrapper
			结构中找到
   dtbImage.%:		类似zImage，但设备blob 被嵌入镜像内部，而非			固件提供。根据平台的不同，输出镜像文件可以是 elf 文件
			或扁平二进制文件
			dtbImage 用于那些没有接口直接传递设备树的系统。dtbImage
			simpleImage 类似，区别在dtbImage 拥有用于从板			固件提取数据的平台特定代码，simpleImage 完全不与
			固件交互
			PlayStation 3 的支持使dtbImage。使PlanetCore 固件
			Embedded Planet 板卡也是如此。板卡特定的初始化代			通常位于名为 arch/powerpc/boot/<platform>.c 的文件中；但
			这可以被 wrapper 脚本覆盖
   simpleImage.%:	与固件无关的压缩镜像，不依赖任何特定的固件接口，			嵌入设备blob。该镜像是一个扁平二进制文件，可被加			RAM 中的任意位置并跳转执行。使用该镜像类型时，固件
			无法向内核传递任何配置数据，它完全依赖嵌入的设备树来
			获取所有信息
   treeImage.%;		用于某些 ppc4xx 硬件OpenBIOS 固件的镜像格式。该
			镜像在内部嵌入一个设备树 blob
   uImage:		U-Boot 使用的原生镜像格式。uImage 目标不添加任何引			代码。它只是把压缩后vmlinux 封装uImage 数据结构			该镜像需要一个能够向内核传递设备树U-Boot 版本			如果使用较旧版本U-Boot，则需要改cuImage
   zImage.%:		不嵌入设备树的镜像格式。用OpenFirmware 及其他能			提供设备树的固件接口。该镜像期望固件在引导时提供设备树			通常，如果你拥有通用 PowerPC 硬件，你会需要这种镜像格式   ==================== ========================================================

嵌入设备blob 的镜像类型（simpleImage、dtbImage、treeImage cuImage）都arch/powerpc/boot/dts/ 目录下的某个文件生成设备blob。Makefile 会根据目的名称选择正确的设备树源。因此，如果内核'make treeImage.walnut' 构建，那构建系统将使arch/powerpc/boot/dts/walnut.dts 来构treeImage.walnut
还存在两个名'zImage' 'zImage.initrd' 的特殊目标。这些目标会构建由内配置所选的所有默认镜像。默认镜像由引导包装Makefile
（arch/powerpc/boot/Makefile）通过将目标添加到 $image-y 变量中来选定。请查看
Makefile 了解有哪些可用的默认镜像目标
### 它是如何构建

arch/powerpc 被设计成支持多平台内核，这意味着单个 vmlinux 镜像可以被引导到
许多不同的目标板卡上。这也意味着引导包装器必须能够针对多种镜像类型在单次
构建中完成封装。设计决策是：不在引导包装器源代码中使用任何条件编译代码
ifdef 等）。所有的引导包装器片段都可以在任何时候、无论内核配置如何地被构建在每次内核构建时构建全部包装器片段，也确保了包装器中那些冷僻的部分至少能大量不同环境中通过编译测试
包装器在链接时通过仅链接适用于该镜像类型的包装器片段，来适配不同的镜像类型'wrapper 脚本'（位arch/powerpc/boot/wrapper）由 Makefile 调用，负责为该镜像类选择正确的包装器片段。其参数在脚本的注释块中有详尽说明，因此此处不再重复。不值得一提的是，该脚本使-p（platform）参数作为决定编译哪些包装器片段的主要方法请查找脚本中间那个大'case "$platform" in' 块。这也是可以通过改变链接顺序选择平台特定修复（fixup）的地方
特别地，在处cuImage 时应小心。cuImage 包装器片段与具体板卡高度相关，应小心
确保你试图构建的目标受这些包装器片段支持