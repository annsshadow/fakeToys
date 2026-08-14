## CerfBoard/Cube


*** CerfBoard/Cube 的 StrongARM 版本已停产 ***

Intrinsyc CerfBoard 是一款基于 StrongARM 1110 的板载计算机，尺寸约为 2 英寸
见方。它包含一个以太网控制器、一个兼容 RS232 的串口、一个 USB 功能端口，以及
背面一个 CompactFlash+ 插槽。图片可在 Intrinsyc 网站 http://www.intrinsyc.com
找到。

本文档描述 Linux 内核对 Intrinsyc CerfBoard 的支持。

## 此版本支持


   - CompactFlash+ 插槽（在 General Setup 中选择 PCMCIA 以及任何可能需要的选项）
   - 板载 Crystal CS8900 以太网控制器（Network Devices 中的 Cerf CS8900A 支持）
   - 带串口控制台的串口（硬编码为 38400 8N1）

为了将此内核装入你的 Cerf，你需要一台同时运行 BOOTP 和 TFTP 的服务器。关于
如何使用引导加载程序的详细说明应随你的评估套件提供。这一系列命令
```

   make ARCH=arm CROSS_COMPILE=arm-linux- cerfcube_defconfig
   make ARCH=arm CROSS_COMPILE=arm-linux- zImage
   make ARCH=arm CROSS_COMPILE=arm-linux- modules
   cp arch/arm/boot/zImage <TFTP directory>

```
support@intrinsyc.com
