
## Rockchip 摄像头接口（CIF）


## 简介


Rockchip 摄像头接口（CIF）在许多 Rockchip SoC 的不同变体中出现。
这些不同的变体是一些通用构建块的组合，例如

- 不同类型的 INTERFACE（接口）块，即

  - 数字视频端口（DVP，一种并行数据接口）
  - 用于 MIPI CSI-2 接收器的接口块

- CROP（裁剪）单元

- MIPI CSI-2 接收器（并非所有变体都有）：在 Rockchip 文档中该单元被称为 MIPI CSI HOST。
  从技术上讲，它是一个独立的硬件块，但它与 CIF 紧密耦合，因此包含在此处。

- MUX（复用）单元（并非所有变体都有），将视频数据传递给图像信号处理器（ISP）

- SCALE（缩放）单元（并非所有变体都有）

- 使用称为 ping-pong 模式的双缓冲机制，将视频数据传入系统内存的 DMA 引擎

- 每个 INTERFACE 块支持四个流（并非所有变体都有），例如用于 MIPI CSI-2 虚拟通道（VC）

本文档描述了 CIF 的不同变体、它们的硬件布局，以及它们在以媒体控制器为中心的 rkcif 设备驱动中的表示，该驱动位于 drivers/media/platform/rockchip/rkcif。

## 变体


### Rockchip PX30 视频输入处理器（VIP）


PX30 视频输入处理器（VIP）具有一个接受并行视频数据或 BT.656 的数字视频端口。
由于这些协议不支持多流，VIP 有一个 DMA 引擎将输入视频数据传入系统内存。

rkcif 驱动通过暴露一个 V4L2 子设备（DVP INTERFACE/CROP 块）和一个 V4L2 设备（DVP DMA 引擎）来表示此硬件变体。

### Rockchip RK3568 视频捕获（VICAP）


RK3568 视频捕获（VICAP）单元具有一个数字视频端口和一个可以独立接收视频数据的 MIPI CSI-2 接收器。
DVP 接受并行视频数据、BT.656 和 BT.1120。
由于 BT.1120 协议可能具有多个流，RK3568 VICAP DVP 具有四个可以捕获不同流的 DMA 引擎。
类似地，RK3568 VICAP MIPI CSI-2 接收器具有四个 DMA 引擎来处理不同的虚拟通道（VC）。

rkcif 驱动通过暴露以下 V4L2 子设备来表示此硬件变体：

- rkcif-dvp0：DVP 的 INTERFACE/CROP 块

以及以下视频设备：

- rkcif-dvp0-id0：DVP 上多流的支持尚未实现，因为很难找到测试硬件。因此，此视频设备代表 RK3568 DVP 的第一个 DMA 引擎。

    :alt:   RK3568 视频捕获（VICAP）单元的拓扑图
    :align: center
