## 内核模式设置 (KMS)


驱动必须通过在 DRM 设备上调用 drmm_mode_config_init() 来初始化模式设置核心。该函数初始化 `struct drm_device <drm_device>` 的 mode_config 字段，且永远不会失败。完成后，必须通过初始化以下字段来建立模式配置。

- int min_width, min_height; int max_width, max_height;
   帧缓冲（frame buffer）的最小和最大宽度与高度，以像素为单位。

- struct drm_mode_config_funcs \*funcs;
   模式设置函数。

## 概述


   :alt: KMS 显示流水线
   :caption: KMS 显示流水线概览

   digraph "KMS" {
      node [shape=box]

      subgraph cluster_static {
          style=dashed
          label="Static Objects"

          node [bgcolor=grey style=filled]
          "drm_plane A" -> "drm_crtc"
          "drm_plane B" -> "drm_crtc"
          "drm_crtc" -> "drm_encoder A"
          "drm_crtc" -> "drm_encoder B"
      }

      subgraph cluster_user_created {
          style=dashed
          label="Userspace-Created"

          node [shape=oval]
          "drm_framebuffer 1" -> "drm_plane A"
          "drm_framebuffer 2" -> "drm_plane B"
      }

      subgraph cluster_connector {
          style=dashed
          label="Hotpluggable"

          "drm_encoder A" -> "drm_connector A"
          "drm_encoder B" -> "drm_connector B"
      }
   }

KMS 向用户空间呈现的基本对象结构相当简单。帧缓冲（由 `struct drm_framebuffer <drm_framebuffer>` 表示，参见 `Frame Buffer Abstraction`_）输入到 plane 中。Plane 由 `struct drm_plane <drm_plane>` 表示，更多细节参见 `Plane Abstraction`_。一个或多个（甚至零个）plane 将其像素数据送入一个 CRTC（由 `struct drm_crtc <drm_crtc>` 表示，参见 `CRTC Abstraction`_）进行混合（blending）。精确的混合步骤在 `Plane Composition Properties`_ 及相关章节中有更详细的说明。

在输出路由方面，第一步是 encoder（由 `struct drm_encoder <drm_encoder>` 表示，参见 `Encoder Abstraction`_）。这些实际上只是用于实现 KMS 驱动的辅助库的内部产物。除此之外，它们让用户空间更难以弄清楚 CRTC 与 connector 之间哪些连接是可能的、支持何种克隆（cloning），它们在用户空间 API 中毫无用处。遗憾的是 encoder 已经暴露给了用户空间，因此目前无法移除它们。此外，暴露的限制经常会被驱动错误地设置，并且在很多情况下不足以表达真正的限制。一个 CRTC 可以连接到多个 encoder，而对于一个处于活动状态的 CRTC 而言，必须至少有一个 encoder。

显示链中最终的、也是真正的端点是 connector（由 `struct drm_connector <drm_connector>` 表示，参见 `Connector Abstraction`_）。Connector 可以有不同的可用 encoder，但内核驱动会为每个 connector 选择使用哪个 encoder。其用例是 DVI，它可以在模拟和数字 encoder 之间切换。Encoder 也可以驱动多个不同的 connector。每个活动 encoder 恰好对应一个活动 connector。

在内部，输出流水线要稍微复杂一些，并且更贴近当今的硬件：

   :alt: KMS 输出流水线
   :caption: KMS 输出流水线

   digraph "Output Pipeline" {
      node [shape=box]

      subgraph {
          "drm_crtc" [bgcolor=grey style=filled]
      }

      subgraph cluster_internal {
          style=dashed
          label="Internal Pipeline"
          {
              node [bgcolor=grey style=filled]
              "drm_encoder A";
              "drm_encoder B";
              "drm_encoder C";
          }

          {
              node [bgcolor=grey style=filled]
              "drm_encoder B" -> "drm_bridge B"
              "drm_encoder C" -> "drm_bridge C1"
              "drm_bridge C1" -> "drm_bridge C2";
          }
      }

      "drm_crtc" -> "drm_encoder A"
      "drm_crtc" -> "drm_encoder B"
      "drm_crtc" -> "drm_encoder C"


      subgraph cluster_output {
          style=dashed
          label="Outputs"

          "drm_encoder A" -> "drm_connector A";
          "drm_bridge B" -> "drm_connector B";
          "drm_bridge C2" -> "drm_connector C";

          "drm_panel"
      }
   }

在内部还有两个额外的辅助对象发挥作用。首先，为了能够在 encoder 之间共享代码（有时在同一 SoC 上，有时在片外），可以将一个或多个 drm_bridge（由 :c:type:`struct drm_bridge <drm_bridge>` 表示）链接到某个 encoder。该链接是静态的，无法更改，这意味着交叉开关（cross-bar，如果有的话）必须映射到 CRTC 与任何 encoder 之间。通常在带有 bridge 的驱动中，encoder 层面已经没有代码剩下。Atomic 驱动可以省去所有 encoder 回调，从而实质上只留下一个哑路由（dummy routing）对象，由于 encoder 已暴露给用户空间，该对象需要保留以实现向后兼容。

第二个对象用于面板（panel），由 :c:type:`struct drm_panel <drm_panel>` 表示，参见 drm_panel_helper。面板没有固定的绑定点，但通常链接到内嵌了 `struct drm_connector <drm_connector>` 的驱动私有结构。

注意，目前 bridge 的链式连接以及与 connector 和 panel 的交互仍处于变动之中，尚未真正完全理清。

## KMS 核心结构体与函数


   :internal:

   :export:


## Modeset Base Object Abstraction


   :alt: 模式对象与属性
   :caption: 模式对象与属性

   digraph {
      node [shape=box]

      "drm_property A" -> "drm_mode_object A"
      "drm_property A" -> "drm_mode_object B"
      "drm_property B" -> "drm_mode_object A"
   }

所有 KMS 对象的基结构是 :c:type:`struct drm_mode_object <drm_mode_object>`。它提供的基础服务之一是跟踪属性（property），这对于 atomic IOCTL 尤为重要（参见 `Atomic Mode Setting`_）。这里有点出人意料的是，属性并非直接在每个对象上实例化，而是本身是独立的模式对象，由 `struct drm_property <drm_property>` 表示，它只规定了属性的类型和取值范围。任何给定的属性都可以通过 drm_object_attach_property() 多次附加到不同对象上。

   :internal:

   :export:

## Atomic Mode Setting



   :alt: 模式对象与属性
   :caption: 模式对象与属性

   digraph {
      node [shape=box]

      subgraph cluster_state {
          style=dashed
          label="Free-standing state"

          "drm_atomic_state" -> "duplicated drm_plane_state A"
          "drm_atomic_state" -> "duplicated drm_plane_state B"
          "drm_atomic_state" -> "duplicated drm_crtc_state"
          "drm_atomic_state" -> "duplicated drm_connector_state"
          "drm_atomic_state" -> "duplicated driver private state"
      }

      subgraph cluster_current {
          style=dashed
          label="Current state"

          "drm_device" -> "drm_plane A"
          "drm_device" -> "drm_plane B"
          "drm_device" -> "drm_crtc"
          "drm_device" -> "drm_connector"
          "drm_device" -> "driver private object"

          "drm_plane A" -> "drm_plane_state A"
          "drm_plane B" -> "drm_plane_state B"
          "drm_crtc" -> "drm_crtc_state"
          "drm_connector" -> "drm_connector_state"
          "driver private object" -> "driver private state"
      }

      "drm_atomic_state" -> "drm_device" [label="atomic_commit"]
      "duplicated drm_plane_state A" -> "drm_device"[style=invis]
   }

Atomic 提供事务性的模式设置（包括 plane）更新，但与通常的 try-commit 加 rollback 的事务方式略有不同：

- 首先，当提交（commit）会失败时，不允许进行任何硬件更改。这使我们能够实现 DRM_MODE_ATOMIC_TEST_ONLY 模式，让用户空间能够试探某些配置是否可行。

- 这仍然允许只设置和回滚软件状态，简化了对现有驱动的转换。但在这种情况下，审计驱动的 atomic_check 代码正确性变得非常困难：到处回滚数据结构中的改动很难做对。

- 最后，为了向后兼容并支持所有用例，atomic 更新需要是增量的，并且要能够并行执行。硬件并非总能做到这一点，但在可能的情况下，不同 CRTC 上的 plane 更新不应相互干扰，也不应因为不同 CRTC 上的输出路由变化而停滞。

综合起来，atomic 设计有两点后果：

- 整体状态被拆分为基于每个对象的 state 结构：plane 对应 `struct drm_plane_state <drm_plane_state>`，CRTC 对应 :c:type:`struct drm_crtc_state <drm_crtc_state>`，connector 对应 :c:type:`struct drm_connector_state <drm_connector_state>`。这些是唯一具有用户空间可见且可设置状态的对象。对于内部状态，驱动可以通过内嵌（embedding）来子类化这些结构，或者为它们全局共享的硬件功能添加全新的状态结构，参见 :c:type:`struct drm_private_state<drm_private_state>`。

- 一个 atomic 更新被组装并验证为 `drm_atomic_state <drm_atomic_state>` 容器内一组完全独立的（free-standing）结构。驱动私有状态结构也在同一结构中跟踪；参见下一章。只有当某个状态被提交时，才会将其应用到驱动和模式设置对象。这样，回滚一次更新就归结为释放内存并解除对帧缓冲等对象的引用。

Atomic state 结构的加锁在内部使用 :c:type:`struct drm_modeset_lock <drm_modeset_lock>`。一般原则是加锁不应暴露给驱动，相反，任何复制或窥视某个状态的函数（例如 drm_atomic_get_crtc_state()）都应自动获取正确的锁。加锁只保护软件数据结构，将状态变更提交到硬件的顺序则使用 `struct drm_crtc_commit <drm_crtc_commit>` 来排序。

本章以及 drm_atomic_helper 中还有更多关于具体主题的详细介绍，请继续阅读。

### 处理驱动私有状态


   :doc: handling driver private state

### 原子模式设置函数参考


   :internal:

   :export:

### 原子模式设置 IOCTL 与 UAPI 函数


   :doc: overview

   :export:

## CRTC Abstraction


   :doc: overview

### CRTC 函数参考


   :internal:

   :export:

### 色彩管理函数参考


   :export:

   :internal:

## Frame Buffer Abstraction


   :doc: overview

### 帧缓冲函数参考


   :internal:

   :export:

## DRM Format Handling


   :doc: overview

### 格式函数参考


   :internal:

   :export:


## Dumb Buffer Objects


   :doc: overview

## Plane Abstraction


   :doc: overview

### Plane 函数参考


   :internal:

   :export:

### Plane 合成函数参考


   :export:

### Plane 损坏跟踪函数参考


   :export:

   :internal:

### Plane 紧急显示特性


   :doc: overview

### Plane 紧急显示特性函数参考


   :internal:

   :export:

## Colorop Abstraction


   :doc: overview

### Colorop 函数参考


   :internal:

   :export:

## 显示模式函数参考


   :internal:

   :export:

## Connector Abstraction


   :doc: overview

### Connector 函数参考


   :internal:

   :export:

### Writeback Connectors


  :doc: overview

  :internal:

  :export:

## Encoder Abstraction


   :doc: overview

### Encoder 函数参考


   :internal:

   :export:

## KMS Locking


   :doc: kms locking

   :internal:

   :export:

## KMS Properties


本文档的这一节主要面向用户空间开发者。有关驱动 API，请参见其他章节。

### Requirements


KMS 驱动可能需要添加额外的属性以支持新功能。除了上面提到的一点之外，驱动中引入的每个新属性还需要满足以下几个要求：

- 它必须是标准化的，并应记录：

  - 完整、准确的名称字符串；
  - 如果该属性是枚举，所有合法的取值名称字符串；
  - 接受哪些值，以及这些值意味着什么；
  - 该属性的作用以及如何使用它；
  - 该属性可能如何与其他已有属性交互。

- 它必须在核心代码中提供一个通用辅助函数，用于将该属性注册到它所附加的对象上。

- 它的内容必须由核心代码解码，并提供到对象关联的状态结构中。这包括驱动可能想要预计算的任何内容，例如 plane 的 struct drm_clip_rect。

- 它的初始状态必须与该属性引入之前的行为一致。这可能是一个与硬件实际行为相匹配的固定值，也可能是从固件在启动期间留给系统的状态继承而来。

- 在合理的情况下，必须提交一个 IGT 测试。

由于历史原因，存在非标准的、驱动特定的属性。如果某个 KMS 驱动想要添加对其中一个属性的支持，则应在可能的情况下适用新属性的各项要求。此外，文档化的行为必须与该已有属性的事实语义相匹配，以确保兼容性。首个添加该属性的驱动的开发者应当协助完成这些任务，并尽可能 ACK 文档化的行为。

### 属性类型与 Blob 属性支持


   :doc: overview

   :internal:

   :export:


### Standard Connector Properties


   :doc: standard connector properties

### HDMI 专用 Connector 属性


   :doc: HDMI connector properties

### 模拟电视专用 Connector 属性


   :doc: Analog TV Connector Properties

### Standard CRTC Properties


   :doc: standard CRTC properties

### Standard Plane Properties


   :doc: standard plane properties


### Plane Composition Properties


   :doc: overview


### Damage Tracking Properties


   :doc: damage tracking

### Color Management Properties


   :doc: overview

### Tile Group Property


   :doc: Tile group

### Explicit Fencing Properties


   :doc: explicit fencing properties


### Variable Refresh Properties


   :doc: Variable refresh properties

### Cursor Hotspot Properties


   :doc: hotspot properties

### Existing KMS Properties


下表描述了各个模块/驱动暴露的 drm 属性。由于该表非常笨重，请勿在此处添加任何新属性。而应在上面的某个小节中记录它们。

   :header-rows: 1
   :file: kms-properties.csv

## Vertical Blanking


   :doc: vblank handling

### 垂直消隐与中断处理函数参考


   :internal:

   :export:

## Vertical Blank Work


   :doc: vblank works

### 垂直消隐工作函数参考


   :internal:

   :export:
