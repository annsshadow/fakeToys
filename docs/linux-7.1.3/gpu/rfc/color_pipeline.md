
## Linux 颜色管线 API


## 我们要解决什么问题？


我们希望在显示控制器硬件中支持混合前与混合后的复杂颜色变换，以便支持由硬件实现的 HDR 用例，并为颜色管理应用（如视频或图像编辑器）提供支持。

在支持 Colorspace 与 HDR Metadata drm_connector 属性的硬件上，是有可能支持 HDR 输出的，但那需要合成器（compositor）或应用将内容渲染并合成为一个最终用于显示的缓冲区。这样做代价高昂。

大多数现代显示硬件提供各种 1D LUT、3D LUT、矩阵以及其他操作来支持颜色变换。这些操作通常实现在固定功能（fixed-function）硬件中，因此比通过着色器或 CPU 执行类似操作要节能得多。

我们希望利用这种硬件功能，以零或最小的 CPU 或着色器负载支持复杂的颜色变换。在固定功能硬件块与着色器/CPU 之间的切换必须是无缝的，当任何时刻需要回退到着色器/CPU 时，都不应有可见的差异。


## 其他操作系统是如何解决这个问题的？


最广泛支持的用例涉及 HDR 内容，无论是视频还是游戏。

大多数操作系统会向驱动指定源内容格式（色域、编码传递函数，以及其他元数据，如最大与平均亮度等级）。驱动随后会相应地编程其固定功能硬件，以从源内容缓冲区的色彩空间映射到显示器的色彩空间。

当固定功能硬件不可用时，合成器会组装一个着色器，请求 GPU 执行从源内容格式到显示器格式的变换。

合成器的映射函数与驱动器的映射函数通常是两个完全独立的概念。在硬件厂商无法了解闭源合成器代码的那些操作系统上，厂商会调整其颜色管理代码，使其在视觉上匹配合成器的效果。在其他操作系统上，当两个映射函数对实现者都开放时，他们会确保两个映射相匹配。

这导致映射算法被锁定，意味着没有人能够单独试验或引入新的映射算法，并无论采用哪种实现路径都获得一致的结果。

## 为什么 Linux 不同？


与其他操作系统上“一个驱动对应一个合成器、或一个驱动对应多个合成器”不同，在 Linux 上我们有的是多对多的关系。许多合成器；许多驱动。此外，每个合成器厂商或社区对于颜色管理应当如何做都有自己的看法。这正是 Linux 之美所在。

这意味着硬件厂商现在不能再把一个驱动调整到与某一个合成器匹配，因为调整到某一个可能让它看起来与另一个合成器的颜色映射相当不同。

我们需要一个更好的解决方案。


## 描述式 API


一个描述源与目标色彩空间的 API 是描述式（descriptive）API。它描述输入和输出的色彩空间，但不描述它们应当如何被精确映射。这样的映射包含许多细微的设计决策，会极大地影响最终结果的外观。

要用足够的细节来描述这样的映射、以确保每个实现得到相同的结果，是不现实的。事实上，这些映射是一个非常活跃的研究领域。


## 规定式 API


规定式（prescriptive）API 描述的不是源与目标色彩空间。相反，它规定了一个如何处理像素值以得到期望结果的配方。

这个配方通常是一个有序的简单操作列表，具有清晰的数学定义，例如 1D LUT、3D LUT、矩阵，或其他能够以精确方式描述的操作。


## 颜色管线 API


硬件颜色管理管线在硬件块的可用性、顺序与能力上，可能在硬件厂商之间显著不同。这使得对颜色管理块及其顺序的共同定义几乎不可能。因此，我们定义的是一个允许用户空间以通用的方式、与特定驱动和硬件无关地发现硬件能力的 API。


## drm_colorop 对象


为了支持颜色管线的定义，我们定义了 DRM 核心对象类型 drm_colorop。各个 drm_colorop 对象将通过 drm_colorop 的 NEXT 属性链接起来，构成一个颜色管线。每个 drm_colorop 对象是唯一的，即，即使多个颜色管线拥有相同的操作，它们也不会共享同一个 drm_colorop 对象来描述该操作。

注意，驱动并不被期望将 drm_colorop 对象静态映射到特定的硬件块。drm_colorop 对象的映射完全是驱动内部的细节，可以如驱动所需那样动态或静态。详见下文“驱动实现者指南”一节。

每个 drm_colorop 有三个核心属性：

TYPE：一个枚举属性，定义变换的类型，例如
- 枚举曲线
- 自定义（均匀）1D LUT
- 3x3 矩阵
- 3x4 矩阵
- 3D LUT
- 等等

根据变换类型的不同，其他属性会描述更多细节。

BYPASS：一个布尔属性，可用于轻松地将一个块置于旁路（bypass）模式。BYPASS 属性对 colorop 不是必须的，只要通过将一个 plane 上的 COLOR_PIPELINE 设置为 '0' 可以旁路整个管线即可。

NEXT：颜色管线中下一个 drm_colorop 的 ID，如果该 drm_colorop 是链中的最后一个，则为 0。

```
    /* 1D 枚举曲线 */
    Color operation 42
    ├─ "TYPE": immutable enum {1D enumerated curve, 1D LUT, 3x3 matrix, 3x4 matrix, 3D LUT, etc.} = 1D enumerated curve
    ├─ "BYPASS": bool {true, false}
    ├─ "CURVE_1D_TYPE": enum {sRGB EOTF, sRGB inverse EOTF, PQ EOTF, PQ inverse EOTF, …}
    └─ "NEXT": immutable color operation ID = 43

    /* 自定义 4k 条目 1D LUT */
    Color operation 52
    ├─ "TYPE": immutable enum {1D enumerated curve, 1D LUT, 3x3 matrix, 3x4 matrix, 3D LUT, etc.} = 1D LUT
    ├─ "BYPASS": bool {true, false}
    ├─ "SIZE": immutable range = 4096
    ├─ "DATA": blob
    └─ "NEXT": immutable color operation ID = 0

    /* 17^3 3D LUT */
    Color operation 72
    ├─ "TYPE": immutable enum {1D enumerated curve, 1D LUT, 3x3 matrix, 3x4 matrix, 3D LUT, etc.} = 3D LUT
    ├─ "BYPASS": bool {true, false}
    ├─ "SIZE": immutable range = 17
    ├─ "DATA": blob
    └─ "NEXT": immutable color operation ID = 73
```
### drm_colorop 可扩展性


与现有的 DRM 核心对象（如 &drm_plane）不同，drm_colorop 不可扩展。这简化了实现，并将管理 &drm_colorop 对象的所有功能保留在 DRM 核心中。

如果有需要，未来可以引入一个简单的 &drm_colorop_funcs 函数表，例如用来支持 &drm_colorop 上的 IN_FORMATS 属性。

如果驱动需要创建驱动特定的 colorop 对象，他们将需要添加 &drm_colorop func 表支持，并支持通常的函数，如 destroy、atomic_duplicate_state 与 atomic_destroy_state。


## COLOR_PIPELINE 平面属性


颜色管线由驱动创建，并通过每个平面（plane）上的一个新 COLOR_PIPELINE 枚举属性来通告。该属性的值始终包含对象 id 0，它是默认值，表示禁用所有颜色处理。额外的值将是管线中第一个 drm_colorop 的对象 ID。一个驱动可以创建并通告零个、一个或更多可能的颜色管线。一个 DRM 客户端将通过把 COLOR PIPELINE 设置为相应的值来选择一条颜色管线。

注意：许多 DRM 客户端会通过字符串值来设置枚举属性，常常是硬编码的。由于这个枚举是基于 colorop 对象 ID 生成的，因此执行下文描述的颜色管线发现（Color Pipeline Discovery），而不是硬编码颜色管线的分配，是很重要的。驱动可能会动态生成枚举字符串。硬编码的字符串可能只对特定硬件上的特定驱动有效。只要驱动实现了所需的颜色操作，颜色管线发现就能普遍工作。

COLOR_PIPELINE 属性仅在设置了 DRM_CLIENT_CAP_PLANE_COLOR_PIPELINE 时才暴露。当设置了此能力时，驱动应忽略任何已有的混合前颜色操作，例如 COLOR_RANGE 与 COLOR_ENCODING。如果驱动希望在颜色管线客户端能力被设置时支持 COLOR_RANGE 或 COLOR_ENCODING 功能，他们应当通过在管线中暴露 colorop 来允许相应的颜色变换。

仅当设置了此客户端能力的用户空间才允许设置 COLOR_PIPELINE 平面属性或 drm_colorop 属性。

```
    Plane 10
    ├─ "TYPE": immutable enum {Overlay, Primary, Cursor} = Primary
    ├─ …
    └─ "COLOR_PIPELINE": enum {0, 42, 52} = 0
```
## 颜色管线发现


一个希望在某 drm_plane 上进行颜色管理的 DRM 客户端将：

1. 获取该平面的 COLOR_PIPELINE 属性
2. 遍历所有 COLOR_PIPELINE 枚举值
3. 对每个枚举值沿颜色管线遍历（通过 NEXT 指针），查看可用的颜色操作是否适合期望的颜色管理操作

如果用户在发现过程中遇到未知或不合适的颜色操作，它无需直接拒绝整条颜色管线，只要该未知或不合适的 colorop 有一个 “BYPASS” 属性。驱动将确保被旁路的块不会产生任何效果。

一个用于定义 AMD 混合前颜色管线的链式属性示例
```
    Plane 10
    ├─ "TYPE" (immutable) = Primary
    └─ "COLOR_PIPELINE": enum {0, 44} = 0

    Color operation 44
    ├─ "TYPE" (immutable) = 1D enumerated curve
    ├─ "BYPASS": bool
    ├─ "CURVE_1D_TYPE": enum {sRGB EOTF, PQ EOTF} = sRGB EOTF
    └─ "NEXT" (immutable) = 45

    Color operation 45
    ├─ "TYPE" (immutable) = 3x4 Matrix
    ├─ "BYPASS": bool
    ├─ "DATA": blob
    └─ "NEXT" (immutable) = 46

    Color operation 46
    ├─ "TYPE" (immutable) = 1D enumerated curve
    ├─ "BYPASS": bool
    ├─ "CURVE_1D_TYPE": enum {sRGB Inverse EOTF, PQ Inverse EOTF} = sRGB EOTF
    └─ "NEXT" (immutable) = 47

    Color operation 47
    ├─ "TYPE" (immutable) = 1D LUT
    ├─ "SIZE": immutable range = 4096
    ├─ "DATA": blob
    └─ "NEXT" (immutable) = 48

    Color operation 48
    ├─ "TYPE" (immutable) = 3D LUT
    ├─ "DATA": blob
    └─ "NEXT" (immutable) = 49

    Color operation 49
    ├─ "TYPE" (immutable) = 1D enumerated curve
    ├─ "BYPASS": bool
    ├─ "CURVE_1D_TYPE": enum {sRGB EOTF, PQ EOTF} = sRGB EOTF
    └─ "NEXT" (immutable) = 0
```
## 颜色管线编程


一旦一个 DRM 客户端找到了合适的管线，它将：

1. 将 COLOR_PIPELINE 枚举值设置为指向期望管线的第一个 drm_colorop 对象的那个值
2. 将管线中所有 drm_colorop 对象的属性设置为期望值，对未使用的 drm_colorop 块将 BYPASS 设为 true，对启用的 drm_colorop 块设为 false
3. 与它希望改变的所有其他 KMS 状态一起执行（TEST_ONLY 或否）原子提交（atomic commit）

为了将管线配置为 HDR10 PQ 平面并在线性空间混合，一个合成器可能会执行如下原子提交：
```
    Plane 10
    └─ "COLOR_PIPELINE" = 42

    Color operation 42
    └─ "BYPASS" = true

    Color operation 44
    └─ "BYPASS" = true

    Color operation 45
    └─ "BYPASS" = true

    Color operation 46
    └─ "BYPASS" = true

    Color operation 47
    ├─ "DATA" = Gamut mapping + tone mapping + night mode
    └─ "BYPASS" = false

    Color operation 48
    ├─ "CURVE_1D_TYPE" = PQ EOTF
    └─ "BYPASS" = false
```
## 驱动实现者指南


这一切对驱动实现意味着什么？如上所述，colorop 可以直接映射到硬件，但不需要这样做。这里有一些关于如何思考创建你的颜色管线的建议：

- 尝试暴露使用已定义 colorop 的管线，即便你的硬件管线划分方式不同。这让现有的用户空间能够立即利用硬件。

- 此外，尝试将你实际的硬件块作为 colorop 暴露出来。在你认为如果用户空间学会编程它们就能带来显著好处的地方，定义新的 colorop 类型。

- 避免为范围非常窄的复合操作定义新的 colorop。如果你有一个无法进一步拆分的特殊操作的硬件块，你可以将其作为一个新的 colorop 类型暴露。但是，尝试不要为“用例”定义 colorop，尤其是当它们要求你组合多个硬件块时。

- 将新的 colorop 设计为规定式的而非描述式的；依据数学公式，而非假定的输入与输出。

一个已定义的 colorop 类型必须是确定性的。colorop 的确切行为必须被完整记录，无论是通过数学公式还是其他某种描述。它的操作只能依赖于它的属性和输入，而不依赖其他任何东西（允许的误差容限除外）。


## 驱动前向/后向兼容性


由于这是 uAPI，驱动不能使已经为给定硬件代引入的颜色管线发生退化（regress）。新的硬件代可以自由地抛弃为前代通告的颜色管线。不过，延续对现有颜色管线的支持可能是有益的，因为它们很可能已经在 DRM 客户端中拥有支持。

向一条管线引入新的 colorop 是可以的，只要它们可以被旁路，或纯粹是信息性的。实现了该管线支持的 DRM 客户端总是可以跳过未知属性，只要它们能够确信这样做不会导致非预期的结果。

如果一个新的 colorop 不属于上述类别之一（可旁路或信息性），那么修改后的管线对用户空间将是不可用的。在这种情况下应当定义一条新的管线。


## 参考资料


1. https://lore.kernel.org/dri-devel/QMers3awXvNCQlyhWdTtsPwkp5ie9bze_hD5nAccFW7a_RXlWjYB7MoUW_8CKLT2bSQwIXVi5H6VULYIxCdgvryZoAoJnC5lZgyK1QWn488=@emersion.fr/
