## Multiplane Overlay (MPO)


   'Documentation/gpu/amdgpu/display/dcn-overview.rst'.


多平面叠加（Multiplane Overlay，MPO）允许通过显示控制器中的固定功能硬件来合成多个
帧缓冲，而不是使用图形或计算着色器来进行合成。如果这意味着图形/计算流水线可以进入
低功耗状态，这可以带来一定的功耗节省。总之，MPO 可带来以下好处：

- 降低 GPU 和 CPU 工作量——不需要合成着色器，不需要额外的缓冲区拷贝，GPU 可保持空闲。
- 平面独立的翻页（page flip）——无需受限于全局合成器的翻页呈现速率，延迟更低，时序
  独立。

   关于显示场景下的更多节能内容，请查看链接：
   `Power <https://gitlab.freedesktop.org/pq/color-and-hdr/-/blob/main/doc/power.rst>`__。

多平面叠加仅在采用 DRM atomic 模型时可用。该 atomic 模型仅使用一个用户空间 IOCTL 来
配置显示硬件（模式设置、翻页等）——drmModeAtomicCommit。为了查询硬件资源和限制，用户
空间还会调用 drmModeGetResources，它返回平面（plane）、CRTC 和连接器的数量。驱动可以
注册并使用三种类型的 DRM 平面：

- `DRM_PLANE_TYPE_PRIMARY`：主平面（Primary plane）代表一个 CRTC 的“主”平面，主平面是
  由 CRTC 模式设置和翻页操作所操作的平面。
- `DRM_PLANE_TYPE_CURSOR`：光标平面（Cursor plane）代表一个 CRTC 的“光标”平面。光标平面
  是由光标 IOCTL 所操作的平面。
- `DRM_PLANE_TYPE_OVERLAY`：叠加平面（Overlay plane）代表所有非主、非光标的平面。某些驱动
  在内部将这些类型的平面称为“sprites”。

为了说明其工作原理，我们来看一个向用户空间暴露如下平面的设备：

- 4 个主平面（每个 CRTC 一个）。
- 4 个光标平面（每个 CRTC 一个）。
- 1 个叠加平面（在 CRTC 之间共享）。

   平面。

对于这个硬件示例，我们有 4 个 pipe（如果你不知道 AMD pipe 的含义，请看
'Documentation/gpu/amdgpu/display/dcn-overview.rst' 的“AMD Hardware Pipeline”一节）。
通常大多数 AMD 设备以 pipe-split 配置运行以获得最优的单显示器输出（例如每个平面 2 个
pipe）。

来自用户空间的一个典型 MPO 配置——在单个显示器上使用 1 个主平面 + 1 个叠加平面——会
看到 4 个 pipe 被使用，每个平面 2 个。

每个平面（主平面和叠加平面）至少使用 1 个 pipe，因此对于我们用作示例的这块假设硬件，
所有 CRTC 上的绝对平面数限制为 4。使用超过 4 个平面的显示配置，atomic commit 将被拒绝。
再次强调，每个 DCN 都有不同的限制；这里我们只试图给出概念性的说明。

## Plane Restrictions


AMDGPU 在驱动中对 DRM 平面的使用施加了限制。

不满足以下限制的 commit 将被拒绝：

- 叠加平面必须是 ARGB8888 或 XRGB8888 格式
- 平面不能放置在 CRTC 目标矩形之外
- 平面缩小比例不能超过原始尺寸的 1/4
- 平面放大比例不能超过原始尺寸的 16 倍

并非每个属性在每个平面上都可用：

- 只有主平面支持色彩空间和非 RGB 格式
- 只有叠加平面支持 alpha 混合

## Cursor Restrictions


在描述光标和 MPO 的一些限制之前，请看下面的图片：


左侧的图片展示了 DRM 期望光标和平面如何被混合。然而，AMD 硬件对光标的处理不同，如右侧
所示；基本上，我们的光标不能绘制在其关联平面之外，因为它被视为平面的一部分。由此带来的
另一个后果是光标从平面继承了颜色和缩放。

由于上述行为，在使用 MPO 时不要使用 legacy API 来设置光标平面；否则你可能会遇到意外的
行为。

简而言之，AMD 硬件没有专用的光标平面。光标被附加到另一个平面上，因此继承了来自其父平面
的任何缩放或颜色处理。

## Use Cases


### Picture-in-Picture (PIP) playback - Underlay strategy


视频播放应当使用“主平面作为底层（underlay）”的 MPO 策略。这是一个 2 平面配置：

- 1 个 YUV DRM 主平面（例如 NV12 视频）
- 1 个 RGBA DRM 叠加平面（例如 ARGB8888 桌面）。合成器应按如下方式为平面准备帧缓冲：
  - 叠加平面包含通用的桌面 UI、视频播放器控件以及视频字幕
  - 主平面包含一个或多个视频

   但这目前我们的驱动还不支持（也许将来如果有用户空间的需求，我们可以改变这一点）。

下面是一个单视频示例：


   我们的驱动不支持。

视频缓冲区应直接用于主平面。视频可以使用以下属性进行缩放和定位以适配桌面：CRTC_X、
CRTC_Y、CRTC_W 和 CRTC_H。主平面还应根据源内容设置颜色编码和颜色范围属性：

- `COLOR_RANGE`、`COLOR_ENCODING`

叠加平面应为 CRTC 的固有尺寸。合成器必须为视频在桌面上应放置的位置绘制一个透明
挖空区（即把 alpha 设为 0）。主平面视频将通过底层透出。在进行标准双缓冲播放时，叠加
平面的缓冲区可以保持静态，而主平面的帧缓冲被使用。

合成器应创建一个与 CRTC 固有尺寸匹配的 YUV 缓冲区。每个视频缓冲区应被合成到这个 YUV
缓冲区上，以进行直接的 YUV 扫描输出（scanout）。主平面应根据源内容设置颜色编码和颜色
范围属性：`COLOR_RANGE`、`COLOR_ENCODING`。但是要注意，每个视频的源色彩空间和编码必须
匹配，因为它会影响整个平面。

叠加平面应为 CRTC 的固有尺寸。合成器必须为每个视频在桌面上应放置的位置绘制一个透明
挖空区（即把 alpha 设为 0）。主平面视频将通过底层透出。在进行视频播放时，叠加平面的
缓冲区可以保持静态，而视频播放的合成操作将在视频缓冲区上完成。

这个内核接口使用 IGT GPU Tools 进行验证。可以运行以下测试来验证定位、混合、缩放，以及
与 DPMS 和 S3 等操作在多种序列和交互下的表现：

- `kms_plane@plane-panning-bottom-right-pipe-*-planes`
- `kms_plane@plane-panning-bottom-right-suspend-pipe-*-`
- `kms_plane@plane-panning-top-left-pipe-*-`
- `kms_plane@plane-position-covered-pipe-*-`
- `kms_plane@plane-position-hole-dpms-pipe-*-`
- `kms_plane@plane-position-hole-pipe-*-`
- `kms_plane_multiple@atomic-pipe-*-tiling-`
- `kms_plane_scaling@pipe-*-plane-scaling`
- `kms_plane_alpha_blend@pipe-*-alpha-basic`
- `kms_plane_alpha_blend@pipe-*-alpha-transparant-fb`
- `kms_plane_alpha_blend@pipe-*-alpha-opaque-fb`
- `kms_plane_alpha_blend@pipe-*-constant-alpha-min`
- `kms_plane_alpha_blend@pipe-*-constant-alpha-mid`
- `kms_plane_alpha_blend@pipe-*-constant-alpha-max`

### Multiple Display MPO


AMDGPU 在使用多显示器时支持显示 MPO；然而，该特性的行为很大程度上依赖于合成器的实现。
请记住用户空间可以定义不同的策略。例如，某些操作系统可以使用 MPO 来保护处理视频播放的
平面；注意我们对单显示器的限制并不多。不过，这种处理在多显示器场景下会有更多限制。下面
的示例展示了在两个显示器中间播放视频的情况，如何处理的策略由合成器定义：


我们来讨论一下处理多显示器 MPO 时面临的一些硬件限制。

#### Limitations


为简便起见，在讨论硬件限制时，本文档假设一个例子：我们有两个显示器，并且一个视频会在
不同显示器之间移动。

- **硬件限制**

从 DCN 概述页可知，每个显示器至少需要一个 pipe，每个 MPO 平面还需要另一个 pipe。因此，
当视频位于两个显示器的中间时，我们需要使用 2 个 pipe。见下面避免 pipe split 的示例：

- 1 个显示器（1 个 pipe）+ MPO（1 个 pipe），我们将使用 2 个 pipe
- 2 个显示器（2 个 pipe）+ MPO（1-2 个 pipe）；我们将使用 4 个 pipe。位于两个显示器
  中间的 MPO 需要 2 个 pipe。
- 3 个显示器（3 个 pipe）+ MPO（1-2 个 pipe），我们需要 5 个 pipe。

如果我们对多显示器使用 MPO，用户空间必须决定：是以限制所支持外部显示器数量为代价启用多个
MPO，还是为了支持多显示器而禁用它；这是一个策略决策。例如：

- 当 ASIC 有 3 个 pipe 时，AMD 硬件无法支持带 MPO 的 2 个显示器
- 当 ASIC 有 4 个 pipe 时，AMD 硬件无法支持带 MPO 的 3 个显示器

我们来简单探讨一下在一个只支持 3 个 pipe 的 ASIC 上，用户空间如何处理这两种显示器配置。
我们可以有：


- pipe 总数为 3
- 用户点亮 2 个显示器（使用了 3 个 pipe 中的 2 个）
- 用户启动视频（1 个 pipe 用于 MPO）
- 现在，如果用户将视频移动到 2 个显示器的中间，视频的一部分将不再是 MPO，因为我们已经
  用了 3/3 个 pipe。

- **缩放限制**

MPO 无法处理小于 0.25 和大于 16 倍的缩放。例如：

如果 4k 视频（3840x2160）以窗口模式播放，窗口的物理尺寸不能小于（960x540）。


- **尺寸限制**

MPO 的最小尺寸为 12px。
