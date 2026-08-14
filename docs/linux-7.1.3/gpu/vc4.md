##  drm/vc4 Broadcom VC4 图形驱动


   :doc: Broadcom VC4 Graphics Driver

## 显示硬件处理


本节涵盖与显示硬件相关的一切，包括模式设置基础设施、平面（plane）、
精灵（sprite）与光标处理和显示、输出探测以及相关主题。

### 像素阀（DRM CRTC）


   :doc: VC4 CRTC module

### HVS


   :doc: VC4 HVS module.

### HVS 平面


   :doc: VC4 plane module

### HDMI 编码器


   :doc: VC4 Falcon HDMI module

### DSI 编码器


   :doc: VC4 DSI0/DSI1 module

### DPI 编码器


   :doc: VC4 DPI module

### VEC（复合电视输出）编码器


   :doc: VC4 SDTV module

## KUnit 测试


VC4 驱动使用 KUnit 执行驱动特定的单元与集成测试。

这些测试使用模拟驱动，可在 arm 或 arm64 架构上使用以下命令运行：


	$ ./tools/testing/kunit/kunit.py run \
		--kunitconfig=drivers/gpu/drm/vc4/tests/.kunitconfig \
		--cross_compile aarch64-linux-gnu- --arch arm64

当前已被测试覆盖的驱动部分包括：
 - HVS 到 PixelValve 的动态 FIFO 分配，适用于 BCM2835-7 和 BCM2711。

## 内存管理与 3D 命令提交


本节涵盖 vc4 驱动中的 GEM 实现。

### GPU 缓冲区对象（BO）管理


   :doc: VC4 GEM BO management support

### V3D binner 命令列表（BCL）校验


   :doc: Command list validator for VC4.

### V3D 渲染命令列表（RCL）生成


   :doc: Render command list generation

### VC4 的着色器校验器


   :doc: Shader validator for VC4.

### V3D 中断


   :doc: Interrupt management for the V3D engine
