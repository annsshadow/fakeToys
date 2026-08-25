AMDgpu 显示管理


    :depth: 3

   :doc: overview

   :internal:

生命周期


   :doc: DM Lifecycle

   :functions: dm_hw_init dm_hw_fini

中断


   :doc: overview

   :internal:

   :functions: register_hpd_handlers dm_crtc_high_irq dm_pflip_high_irq

原子化实


   :doc: atomic

   :functions: amdgpu_dm_atomic_check amdgpu_dm_atomic_commit_tail

颜色管理属


   :doc: overview

   :internal:


DCN 各代之间DC 颜色能力


DRM/KMS 框架定义了三CRTC 颜色校正属性：degamma（去伽马）、颜色变换矩阵（CTM）和 gamma（伽马），以及用degamma gamma LUT 大小的两个属性。AMD DC 在混合前编程部分颜色校正特性，DRM/KMS 没有逐平面的颜色校正属性

一般而言，DRM CRTC 颜色属性按如下方式编程DC：混合后CRTC gamma，以及混合前CRTC degamma。尽CTM 在混合后编程，但它被映射DPP 硬件块（混合前）。硬件中可用的其他颜色能力目前尚未由 DRM 接口暴露，因而被旁路

   :doc: color-management-caps

   :internal:

颜色流水线在 DCN 硬件各代之间发生了重大变化。混合前和混合后可执行的操作取决于硬件能力，如下所DCN 2.0 DCN 3.0 系列的架构示意

**DCN 2.0 系列颜色能力及映*


**DCN 3.0 系列颜色能力及映*


混合模式属


像素混合模式`drm_plane` 的一DRM 平面合成属性，用于描述前景平面（fg）的像素如何与背景平面（bg）合成。此处介DRM 混合模式的主要概念，以帮助理解该属性如何映射到 AMD DC 接口。有关此 DRM 属性及 alpha 混合方程的更多内容，请参:ref:`DRM Plane Composition Properties <plane_composition_properties>`

基本上，混合模式为平面合成设alpha 混合方程，该方程适用alpha 通道影响像素颜色值状态（从而影响最终像素颜色）的模式。例如，考虑 alpha 混合方程的以下元素：

- **fg.rgb**：前景像素的各个 RGB 分量值
- **fg.alpha**：前景像素的 alpha 分量值
- **bg.rgb**：背景的各个 RGB 分量值
- **plane_alpha**：由 **plane "alpha" property** 设定的平alpha 值，详见 DRM 平面合成属<plane_composition_properties>

```

   out.rgb = alpha * fg.rgb + (1 - alpha) * bg.rgb

```
平面中每个像素的 alpha 通道值被忽略，仅平面 alpha 影响最终的像素颜色值

DRM 定义了三种混合模式来规定平面合成中的混合公式

**None**：忽略像alpha 的混合公式

**Pre-multiplied**：假定平面中的像素颜色值在进行存储前已经过自身 alpha 通道预乘的混合公式

**Coverage**：假定像素颜色值未alpha 通道值预乘的混合公式

预乘是默认的像素混合模式，这意味着当未创建或定义混合模式属性时，DRM 认为平面的像素具有预乘的颜色值。在 IGT GPU 工具中，kms_plane_alpha_blend 测试提供了一组子测试，用于验证平alpha 和混合模式属性

然后，DRM 混合模式及其元素AMDGPU 显示管理器（DM）映射，以编程多管道/平面组合（MPC）的混合配置，如下所示：

   :identifiers: mpcc_blnd_cfg

因此，MPC 树上单个 MPCC 实例的混合配置由 `mpcc_blnd_cfg` 定义，其
`pre_multiplied_alpha` 是用于设`MPCC_ALPHA_MULTIPLIED_MODE` alpha 预乘模式标志。它控制 alpha 是否被乘（true/false），仅在 DRM 预乘混合模式下为 true
`mpcc_alpha_blend_mode` 定义了关于像alpha 和平alpha 值的 alpha 混合模式。它
`MPCC_ALPHA_BLND_MODE` 设定三种模式之一，如下所述

   :identifiers: mpcc_alpha_blend_mode

然后 DM `enum mpcc_alpha_blend_mode` 的元素映射到 DRM 混合公式中的元素，如下所示：

- **MPC 像素 alpha** 对应 **DRM fg.alpha**，即来自平面像素alpha 分量值
- **MPC 全局 alpha** 在应忽略像素 alpha 时对**DRM plane_alpha**，因此像素值未预乘
- **MPC 全局增益** *DRM fg.alpha** **DRM plane_alpha* 都参与混合方程时，假定为 **MPC 全局 alpha** 值

简而言之，通过选择 `MPCC_ALPHA_BLEND_MODE_GLOBAL_ALPHA` 会忽**fg.alpha**。另一方面，通过选择 `MPCC_ALPHA_BLEND_MODE_PER_PIXEL_ALPHA_COMBINED_GLOBAL_GAIN` 可使（plane_alpha * fg.alpha）分量可用。`MPCC_ALPHA_MULTIPLIED_MODE` 定义了像素颜色值是否被 alpha 预乘

混合配置流程


alpha 混合方程通过以下路径DRM 配置DC 接口

1. 更新 `drm_plane_state <drm_plane_state>` 时，DM 调用
   `amdgpu_dm_plane_fill_blending_from_plane_state()`，将
   `drm_plane_state <drm_plane_state>` 属性映射到
   `dc_plane_info <dc_plane_info>` 结构体，交由
   操作系统无关组件（DC）处理

2. DC 接口上，`struct mpcc_blnd_cfg <mpcc_blnd_cfg>` 编程
   MPCC 混合配置，并考虑来自 DPP :c:type:`dc_plane_info
   <dc_plane_info>` 输入
