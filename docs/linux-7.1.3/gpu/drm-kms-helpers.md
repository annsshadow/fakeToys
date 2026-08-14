## 模式设置辅助函数


DRM 子系统致力于在核心代码与辅助库之间实现强分离。核心代码负责通用初始化与拆卸，以及将用户空间请求解码为内核内部对象。其余一切由一组庞大的辅助库处理，这些库可以自由组合，以便为每个驱动挑选合适的内容，并在需要特殊行为时避免共享代码。

核心代码与辅助代码之间的这种区分在模式设置（modesetting）代码中尤为明显，因为所有驱动共享同一套用户空间 ABI。这与渲染侧形成对比，在渲染侧，几乎所有内容（极少数例外）都可以被视为可选的辅助代码。

这些辅助代码可以分为几个领域：

- 实现模式设置的辅助代码。这里重要的是 atomic（原子）辅助函数。旧驱动通常仍使用传统的 CRTC 辅助函数。它们共享同一组通用的辅助 vtable。对于真正简单的驱动（任何本应非常适合已被弃用的 fbdev 子系统的驱动），还有简单显示管道（simple display pipe）辅助函数。

- 一大堆用于处理输出（output）的辅助函数。首先是用于处理编码器（encoder）和收发器（transcoder）IP 模块的通用桥接（bridge）辅助函数。其次是用于处理与面板相关信息和逻辑的面板（panel）辅助函数。此外还有一大组用于各种接收端标准的辅助函数（DisplayPort、HDMI、MIPI DSI）。最后还有用于处理输出探测和处理 EDID 的通用辅助函数。

- 最后一组辅助代码关注显示管线的前端：平面（Plane）、用于可见性检查和裁剪（scissoring）的矩形处理、翻转队列（flip queue）以及各类零散内容。

## 通用 Vtable 的模式设置辅助函数参考


   :doc: overview

   :internal:


## 原子模式设置辅助函数参考


### 概述


   :doc: overview

### 实现异步原子提交（Asynchronous Atomic Commit）


   :doc: implementing nonblocking commit

### 辅助函数参考


   :internal:

   :export:

### 原子状态重置与初始化


   :doc: atomic state reset and initialization

### 原子状态辅助函数参考


   :export:

### GEM 原子辅助函数参考


   :doc: overview

   :internal:

   :export:

### VBLANK 辅助函数参考


   :doc: overview

   :internal:

   :export:

## fbdev 辅助函数参考


   :doc: fbdev helpers

   :internal:

   :export:

## 格式辅助函数参考


   :export:

## 帧缓冲 DMA 辅助函数参考


   :doc: framebuffer dma helper functions

   :export:

## 帧缓冲 GEM 辅助函数参考


   :doc: overview

   :export:


## 桥接（Bridges）


### 概述


   :doc: overview

### 显示驱动集成


   :doc: display driver integration

### 处理 MIPI-DSI 桥接的特殊注意事项


   :doc: special care dsi

### 桥接操作


   :doc: bridge operations

### 桥接连接器辅助函数


   :doc: overview


### 桥接辅助函数参考


   :internal:

   :export:

### MIPI-DSI 桥接操作


   :doc: dsi bridge operations


### 桥接连接器辅助函数参考


   :export:

### Panel-Bridge 辅助函数参考


   :export:


## 面板辅助函数参考


   :doc: drm panel

   :internal:

   :export:

   :export:

   :export:

## 面板自刷新（Self Refresh）辅助函数参考


   :doc: overview

   :export:

## HDMI 原子状态辅助函数


### 概述


   :doc: hdmi helpers

### 函数参考


   :export:

## HDCP 辅助函数参考


   :export:

## Display Port 辅助函数参考


   :doc: dp helpers

   :internal:

   :internal:

   :export:

## Display Port CEC 辅助函数参考


   :doc: dp cec helpers

   :export:

## Display Port 双模适配器（Dual Mode Adaptor）辅助函数参考


   :doc: dp dual mode helpers

   :internal:

   :export:

## Display Port MST 辅助函数


### 概述


   :doc: dp mst helper

   :doc: Branch device and port refcounting

### 函数参考


   :internal:

   :export:

### 拓扑生命周期内部机制


这些函数并未导出给驱动，但在此加以文档说明以帮助理解 MST 拓扑辅助函数

   :functions: drm_dp_mst_topology_try_get_mstb drm_dp_mst_topology_get_mstb
               drm_dp_mst_topology_put_mstb
               drm_dp_mst_topology_try_get_port drm_dp_mst_topology_get_port
               drm_dp_mst_topology_put_port
               drm_dp_mst_get_mstb_malloc drm_dp_mst_put_mstb_malloc

## MIPI DBI 辅助函数参考


   :doc: overview

   :internal:

   :export:

## MIPI DSI 辅助函数参考


   :doc: dsi helpers

   :internal:

   :export:

## 显示流压缩（Display Stream Compression）辅助函数参考


   :doc: dsc helpers

   :internal:

   :export:

## 输出探测辅助函数参考


   :doc: output probing helper overview

   :export:

## EDID 辅助函数参考


   :internal:

   :export:

   :internal:

   :export:

## SCDC 辅助函数参考


   :doc: scdc helpers

   :internal:

   :export:

## HDMI Infoframes 辅助函数参考


严格来说，这不是一个 DRM 辅助库，而是任何与 HDMI 输出交互的驱动（如 v4l 或 alsa 驱动）都可通用。但它很好地契合模式设置辅助库这一整体主题，因此也包含在此处。

   :internal:

   :export:

## 矩形工具参考


   :doc: rect utils

   :internal:

   :export:

## Flip-work 辅助函数参考


   :doc: flip utils

   :internal:

   :export:

## 辅助模式设置（Auxiliary Modeset）辅助函数


   :doc: aux kms helpers

   :export:

## OF/DT 辅助函数


   :doc: overview

   :export:

## 传统平面（Legacy Plane）辅助函数参考


   :doc: overview

   :export:

## 传统 CRTC/模式设置辅助函数参考


   :doc: overview

   :export:

## 隐私屏幕（Privacy-screen）类


   :doc: overview

   :internal:

   :internal:

   :export:
