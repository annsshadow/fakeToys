
##  drm/komeda Arm 显示驱动


drm/komeda 驱动支持 Arm 显示处理D71 及之后的产品，本文档简要概述驱设计：它如何工作，以及为何如此设计
## D71 类显IP 概述


D71 开始，Arm 显示 IP 开始采用灵活、模块化的架构。一条显示流水线由多独立且功能化的流水线阶段（称为组件）组成，每个组件都有一些特定能力，可对
流经流水线的像素数据做特定处理
典型D71 组件
### Layer（图层）

Layer 是第一个流水线阶段，为下一阶段准备像素数据。它从内存中获取像素如果AFBC 则解码，旋转源图像，YUV 像素解包或转换为设备内部 RGB 像素然后在需要时对像素的 color_space（色彩空间）进行调整
### Scaler（缩放器
顾名思义，scaler 负责缩放，D71 还支持通过 scaler 进行图像增强scaler 的使用非常灵活，可以连接layer 输出以进行图层缩放，或连接到
compositor（合成器）并缩放整个显示帧，然后将输出数据送入 wb_layer，由后写入内存
### Compositor（compiz，合成器
Compositor 将多个图层或像素数据流混合为单一显示帧。其输出帧可以送入后图处理器（post image processor）以在显示器上显示，或同时送入 wb_layer 并写内存。用户也可以compiz wb_layer 之间插入一scaler，先对显示帧进行
缩小，再写入内存
### Writeback Layer（wb_layer，回写图层）

Writeback layer 做与 Layer 相反的事情，它连接到 compiz，并将合成结果写内存
### Post image processor（improc，后图像处理器）

Post image processor 调整帧数据，gamma 和色彩空间，以符合显示器的要求
### Timing controller（timing_ctrlr，时序控制器
显示流水线的最后一个阶段，Timing controller 不处理像素，只用于控制显示时序
### Merger（合并器
D71 scaler Layer 相比，大多只有一半的水平和输入输出能力，例如如果
Layer 支持 4K 输入尺寸，scaler 在同一时间只能支持 2K 输入/输出。为了实完整帧缩放，D71 引入Layer Split，它将整幅图像切分为两半，分别送入两个
Layer A B，并独立进行缩放。缩放后需要将结果送入 merger 将两个部分图合并在一起，然后将合并结果输出到 compiz
### Splitter（分割器
Layer Split 类似，但 Splitter 用于回写，它compiz 的结果切分为两部分，
然后分别送入两个 scaler
## D71 流水线可能的用法


受益于模块化架构，D71 流水线可以轻松调整以适配不同用途。D71 有两条流水线支持两种工作模式
- Dual display mode（双显示模式    两条流水线独立、分别地工作，驱动两个显示输出
- Single display mode（单显示模式    两条流水线协同工作，仅驱动一个显示输出
    在此模式下，pipeline_B 不独立工作，而是将其合成结果输出pipeline_A    其像素时序也派生pipeline_A.timing_ctrlr。pipeline_B 就如    pipeline_A（master，主）的一“slave”（从）
### 单流水线数据

   :alt: 单流水线 digraph
   :caption: 单流水线数据
   digraph single_ppl {
      rankdir=LR;

      subgraph {
         "Memory";
         "Monitor";
      }

      subgraph cluster_pipeline {
          style=dashed
          node [shape=box]
          {
              node [bgcolor=grey style=dashed]
              "Scaler-0";
              "Scaler-1";
              "Scaler-0/1"
          }

         node [bgcolor=grey style=filled]
         "Layer-0" -> "Scaler-0"
         "Layer-1" -> "Scaler-0"
         "Layer-2" -> "Scaler-1"
         "Layer-3" -> "Scaler-1"

         "Layer-0" -> "Compiz"
         "Layer-1" -> "Compiz"
         "Layer-2" -> "Compiz"
         "Layer-3" -> "Compiz"
         "Scaler-0" -> "Compiz"
         "Scaler-1" -> "Compiz"

         "Compiz" -> "Scaler-0/1" -> "Wb_layer"
         "Compiz" -> "Improc" -> "Timing Controller"
      }

      "Wb_layer" -> "Memory"
      "Timing Controller" -> "Monitor"
   }

### 启用 Slave 的双流水

   :alt: Slave 流水digraph
   :caption: 启用 Slave 流水线的数据
   digraph slave_ppl {
      rankdir=LR;

      subgraph {
         "Memory";
         "Monitor";
      }
      node [shape=box]
      subgraph cluster_pipeline_slave {
          style=dashed
          label="Slave Pipeline_B"
          node [shape=box]
          {
              node [bgcolor=grey style=dashed]
              "Slave.Scaler-0";
              "Slave.Scaler-1";
          }

         node [bgcolor=grey style=filled]
         "Slave.Layer-0" -> "Slave.Scaler-0"
         "Slave.Layer-1" -> "Slave.Scaler-0"
         "Slave.Layer-2" -> "Slave.Scaler-1"
         "Slave.Layer-3" -> "Slave.Scaler-1"

         "Slave.Layer-0" -> "Slave.Compiz"
         "Slave.Layer-1" -> "Slave.Compiz"
         "Slave.Layer-2" -> "Slave.Compiz"
         "Slave.Layer-3" -> "Slave.Compiz"
         "Slave.Scaler-0" -> "Slave.Compiz"
         "Slave.Scaler-1" -> "Slave.Compiz"
      }

      subgraph cluster_pipeline_master {
          style=dashed
          label="Master Pipeline_A"
          node [shape=box]
          {
              node [bgcolor=grey style=dashed]
              "Scaler-0";
              "Scaler-1";
              "Scaler-0/1"
          }

         node [bgcolor=grey style=filled]
         "Layer-0" -> "Scaler-0"
         "Layer-1" -> "Scaler-0"
         "Layer-2" -> "Scaler-1"
         "Layer-3" -> "Scaler-1"

         "Slave.Compiz" -> "Compiz"
         "Layer-0" -> "Compiz"
         "Layer-1" -> "Compiz"
         "Layer-2" -> "Compiz"
         "Layer-3" -> "Compiz"
         "Scaler-0" -> "Compiz"
         "Scaler-1" -> "Compiz"

         "Compiz" -> "Scaler-0/1" -> "Wb_layer"
         "Compiz" -> "Improc" -> "Timing Controller"
      }

      "Wb_layer" -> "Memory"
      "Timing Controller" -> "Monitor"
   }

### 用于输入和输出的子流水线


一条完整的显示流水线可以根据输输出用途轻松分为三个子流水线
#### Layer(input) 流水

   :alt: Layer 数据 digraph
   :caption: Layer（输入）数据
   digraph layer_data_flow {
      rankdir=LR;
      node [shape=box]

      {
         node [bgcolor=grey style=dashed]
           "Scaler-n";
      }

      "Layer-n" -> "Scaler-n" -> "Compiz"
   }

   :alt: Layer Split digraph
   :caption: Layer Split 流水
   digraph layer_data_flow {
      rankdir=LR;
      node [shape=box]

      "Layer-0/1" -> "Scaler-0" -> "Merger"
      "Layer-2/3" -> "Scaler-1" -> "Merger"
      "Merger" -> "Compiz"
   }

#### Writeback(output) 流水
   :alt: 回写 digraph
   :caption: Writeback（输出）数据
   digraph writeback_data_flow {
      rankdir=LR;
      node [shape=box]

      {
         node [bgcolor=grey style=dashed]
           "Scaler-n";
      }

      "Compiz" -> "Scaler-n" -> "Wb_layer"
   }

   :alt: 拆分回写 digraph
   :caption: Writeback（输出）拆分数据
   digraph writeback_data_flow {
      rankdir=LR;
      node [shape=box]

      "Compiz" -> "Splitter"
      "Splitter" -> "Scaler-0" -> "Merger"
      "Splitter" -> "Scaler-1" -> "Merger"
      "Merger" -> "Wb_layer"
   }

#### 显示输出流水
   :alt: 显示 digraph
   :caption: 显示输出数据
   digraph single_ppl {
      rankdir=LR;
      node [shape=box]

      "Compiz" -> "Improc" -> "Timing Controller"
   }

在下面的小节中，我们将看到这三个子流水线分别KMS-plane/wb_conn/crtc 处理
## Komeda 资源抽象


### struct komeda_pipeline/component


为了充分利用并易于访配置硬件，驱动侧也使用类似的架构：Pipeline/Component
来描述硬件特性和能力，一个特定的组件包含两部分：

- 数据流控制- 特定组件的能力与特性
因此驱动定义了一个通用头部结构komeda_component 来描述数据流控制，所特定组件都是此基础结构的子类
   :internal:

## 资源发现与初始化


Pipeline component 用于描述如何处理像素数据。我们仍然需要一@struct
komeda_dev 来描述设备的整体视图，以及设备的控制能力
我们&komeda_devkomeda_pipelinekomeda_component。现在用流水线填设备。由komeda 不仅用于 D71，也面向之后的产品，我们当然最好在不同产品尽可能多地共享。为此，komeda 设备分为两层：CORE CHIP
- CORE：用于通用特性与能力的处理- CHIP：用于寄存器编程和硬件特定特性（限制）的处理
CORE 可以通过三个 chip 函数结构访问 CHIP
- struct komeda_dev_funcs
- struct komeda_pipeline_funcs
- struct komeda_component_funcs

   :internal:

## 格式处理


   :internal:
   :internal:

## komeda_dev 挂接DRM-KMS


Komeda 通过 pipeline/component 抽象资源，但 DRM-KMS 使用 crtc/plane/connector一KMS 对象不能仅代表单个组件，因为单个 KMS 对象的要求不能简单地由单组件满足，通常那需要多个组件来满足要求。例如设mode、gamma、ctm 都是针对
KMS CRTC 对象，但 komeda 需compiz、improc timing_ctrlr 协同工作满足这些要求。而一KMS-Plane 可能需要多komeda 资源：layer/scaler/compiz
因此，一KMS 对象代表 komeda 资源的一个子流水线
- Plane：`Layer(input) pipeline`_
- Wb_connector：`Writeback(output) pipeline`_
- Crtc：`Display output pipeline`_

因此，对komeda，我们将 KMS crtc/plane/connector 视为 pipeline component
的使用者，并且在任意时刻一pipeline/component 只能被一个使用者使用。pipeline/component 将被视为 DRM-KMS 的私有对象；其状态也drm_atomic_state
管理
### 如何plane 映射Layer(input) 流水

Komeda 有多Layer 输入流水线，参见- `Single pipeline data flow`_
- `Dual pipeline with Slave enabled`_

最简单的方法是把一plane 绑定到一个固定的 Layer 流水线，但考虑komeda 能力
- Layer Split，参`Layer(input) pipeline`_

    Layer_Split 是一个相当复杂的特性，它将一幅大图像切分为两部分，由两层    两个 scaler 分别处理。但它会在切分后在图像中间引入边缘问题或效果。为
    避免此类问题，需要对切分进行复杂计算，并layer scaler 做一些特    配置。我们最好将此类的硬件相关复杂性对用户态隐藏
- Slave 流水线，参见 `Dual pipeline with Slave enabled`_

    由于 compiz 组件不输alpha 值，slave 流水线只能用于底层（bottom）图层的
    合成。komeda 驱动希望向用户隐藏此限制。做法是根据 plane_state->zpos 选择
    一个合适的 Layer
因此对于 komeda，KMS-plane 不代表一个固定的 komeda layer 流水线，而是代表
多个具有相同能力Layer。Komeda 会选择一个或多个 Layer 来满足一KMS-plane
的要求
### component/pipeline 设为 drm_private_obj


`drm_private_obj` 添加`komeda_component`、`komeda_pipeline`


    struct komeda_component {
        struct drm_private_obj obj;
        ...
    }

    struct komeda_pipeline {
        struct drm_private_obj obj;
        ...
    }

### 通过 drm_atomic_state 跟踪 component_state/pipeline_state


`drm_private_state` user 添加`komeda_component_state``komeda_pipeline_state`


    struct komeda_component_state {
        struct drm_private_state obj;
        void *binding_user;
        ...
    }

    struct komeda_pipeline_state {
        struct drm_private_state obj;
        struct drm_crtc *crtc;
        ...
    }

### komeda 组件校验


Komeda 有多种类型的组件，但校验过程类似，通常包括以下步骤

    int komeda_xxxx_validate(struct komeda_component_xxx xxx_comp,
                struct komeda_component_output *input_dflow,
                struct drm_plane/crtc/connector *user,
                struct drm_plane/crtc/connector_state, *user_state)
    {
         setup 1: 检查是否需要该组件，例scaler 取决user_state 是可选的                  如果不需要，直接返回，调用者会将数据流送入下一阶段         Setup 2: 用组件特性和能力user_state 进行比对，看是否能满足要求；
                  如果不满足，返回失败         Setup 3: drm_atomic_state 获取 component_state，并尝试user 设置
                  到组件；如果组件已经被分配给另一user，返回失败         Setup 3: 配置 component_state，例如设置其输入组件                  user_state 转换为组件特定的状态         Setup 4: 调整 input_dflow 并为下一阶段做准备    }

### komeda_kms 抽象


   :internal:

### komde_kms 函数


   :internal:
   :internal:

## komeda 构建Linux 模块驱动


现在我们有两个层级设备：

- komeda_dev：描述真实的显示硬件- komeda_kms_dev：将 komeda_dev 挂接或连接到 DRM-KMS
所komeda 操作都由 komeda_dev komeda_kms_dev 提供或执行，模块驱动只是
一个简单的封装，用于将 Linux 命令（probe/remove/pm）传komeda_dev komeda_kms_dev