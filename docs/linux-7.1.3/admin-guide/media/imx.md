
## i.MX 视频捕获驱动

### 简介

Freescale i.MX5/6 包含一个图像处理单元（Image Processing Unit，IPU），
它负责图像帧在捕获设备和显示设备之间的流向。

对于图像捕获，IPU 包含以下内部子单元：

- 图像 DMA 控制器（Image DMA Controller，IDMAC）
- 摄像头串行接口（Camera Serial Interface，CSI）
- 图像转换器（Image Converter，IC）
- 传感器多 FIFO 控制器（Sensor Multi-FIFO Controller，SMFC）
- 图像旋转器（Image Rotator，IRT）
- 视频去隔行或合成模块（Video De-Interlacing or Combining Block，VDIC）

IDMAC 是用于在内存与图像帧之间进行传输的 DMA 控制器。针对视频捕获和显示
路径分别存在各种专用 DMA 通道。在传输过程中，IDMAC 还能够进行垂直图像翻转、
8x8 块传输（参见 IRT 描述）、同一色彩空间内的像素分量重排序（例如 UYVY 到
YUYV），以及打包（packed）<--> 平面（planar）转换。IDMAC 还可以在传输时通过
交错偶数行和奇数行的方式执行简单的去隔行（不带有需要 VDIC 支持的运动补偿）。

CSI 是后端捕获单元，通过并行（Parallel）、BT.656/1120 和 MIPI CSI-2 总线
直接与摄像头传感器接口。

IC 负责色彩空间转换、缩放（缩小和放大）、水平翻转以及 90/270 度旋转操作。

IC 内部有三个可并发执行转换的独立“任务”：预处理器编码（pre-process
encoding）、预处理器取景器（pre-process viewfinder）和后处理（post-processing）。
在每个任务内，转换被分为三个部分：缩小部分、主处理部分（放大、翻转、色彩空间
转换以及图形平面合成）和旋转部分。

IPU 以时间片方式共享 IC 任务操作。时间片粒度在缩小部分为一次突发 8 个像素，
在主处理部分为一行图像，在旋转部分为一帧图像。

SMFC 由四个独立的 FIFO 组成，每个 FIFO 都可以通过四个 IDMAC 通道并发地将
捕获的帧从传感器直接传送到内存。

IRT 执行 90 度和 270 度图像旋转操作。该旋转操作每次在 8x8 像素块上进行。该
操作由 IDMAC 配合完成，IDMAC 负责 8x8 块传输以及块重排序，并与垂直翻转协同
工作。

VDIC 负责将隔行视频转换为逐行视频，支持不同的运动补偿模式（低、中、高运动）。
VDIC 去隔行后的输出帧可以发送到 IC 预处理器取景器任务做进一步转换。VDIC 还
包含一个合成器（Combiner），可使用 Alpha 混合和色彩键控将两幅图像平面合成
在一起。

除了 IPU 内部子单元外，i.MX 上还有两个位于 IPU 外部的单元也参与视频捕获：

- 用于带 MIPI CSI-2 总线接口的摄像头传感器的 MIPI CSI-2 接收器。这是一个
  Synopsys DesignWare 核心。
- 两个视频多路复用器，用于在多个传感器输入之间选择并发送到某个 CSI。

更多信息，请参考最新版本的 i.MX5/6 参考手册 [#f1]_ 和 [#f2]_。

### 特性

本驱动的部分特性包括：

- 可通过 media controller API 配置许多不同的流水线（pipeline），它们对应 i.MX
  中支持的硬件视频捕获流水线。

- 支持并行、BT.565 以及 MIPI CSI-2 接口。

- 通过配置流水线到多个视频捕获接口（使用独立的实体），支持并发的独立数据流。

- 通过 IC 任务子设备（subdev）实现缩放、色彩空间转换、水平和垂直翻转以及
  图像旋转。

- 支持多种像素格式（RGB、打包和平面 YUV、部分平面 YUV）。

- VDIC 子设备支持运动补偿去隔行，具有三种运动补偿模式：低、中、高运动。定义了
  允许从 CSI 直接向 VDIC 子设备发送帧的流水线。未来还支持通过输出/内存到内存
  （mem2mem）设备从内存缓冲区向 VDIC 发送帧。

- 包含一个帧间隔监视器（Frame Interval Monitor，FIM），可以纠正 ADV718x
  视频解码器的垂直同步问题。

### 拓扑结构

下面展示了 i.MX6Q SabreSD 和 i.MX6Q SabreAuto 的 media 拓扑结构。请参考下一
小节实体描述中的这些图。

i.MX5/6 的拓扑结构在 IPUv3 CSI 视频多路复用器上游可能有所不同，但从那里往下的
内部 IPUv3 拓扑对所有 i.MX5/6 平台都是通用的。例如，带 MIPI CSI-2 OV5640 传感器的
SabreSD 需要 i.MX6 MIPI CSI-2 接收器。而 SabreAuto 在并行 bt.656 总线上只有
ADV7180 解码器，因此不需要 MIPI CSI-2 接收器，所以在它的图中没有该部分。

    :alt:   Diagram of the i.MX6Q SabreSD media pipeline topology
    :align: center

    Media pipeline graph on i.MX6Q SabreSD

    :alt:   Diagram of the i.MX6Q SabreAuto media pipeline topology
    :align: center

    Media pipeline graph on i.MX6Q SabreAuto

### 实体

### imx6-mipi-csi2

这是 MIPI CSI-2 接收器实体。它有一个 sink 管脚（pad）用于接收 MIPI CSI-2 流
（通常来自 MIPI CSI-2 摄像头传感器）。它有四个 source 管脚，对应四个 MIPI CSI-2
解复用（demuxed）的虚拟通道输出。可以启用多个 source 管脚以从多个虚拟通道
独立地进行流传输。

该实体实际上由两个子块组成。一个是 MIPI CSI-2 核心，这是一个 Synopsys
Designware MIPI CSI-2 核心。另一个子块是“CSI-2 到 IPU 的垫片（gasket）”。该
垫片充当四个虚拟通道流的解复用器，提供四条独立的并行总线，每条包含各自的
虚拟通道，并如上所述路由到 CSI 或视频多路复用器。

在 i.MX6 solo/dual-lite 上，全部四个虚拟通道总线都被路由到两个视频多路复用器。
CSI0 和 CSI1 都可以通过视频多路复用器选择接收任意虚拟通道。

在 i.MX6 Quad 上，虚拟通道 0 路由到 IPU1-CSI0（经过视频多路复用器选择），虚拟
通道 1 和 2 分别硬连线到 IPU1-CSI1 和 IPU2-CSI0，虚拟通道 3 路由到 IPU2-CSI1
（同样经过视频多路复用器选择）。

### ipuX_csiY_mux

这些是视频多路复用器。它们有两个或更多 sink 管脚，用于从带并行接口的摄像头
传感器选择，或从 imx6-mipi-csi2 实体的 MIPI CSI-2 虚拟通道选择。它们有一个
单独的 source 管脚，路由到某个 CSI（ipuX_csiY 实体）。

在 i.MX6 solo/dual-lite 上，有两个视频多路复用器实体。一个位于 IPU1-CSI0 之前，
用于在并行传感器和四个 MIPI CSI-2 虚拟通道中任选其一（共五个 sink 管脚）。另一个
多路复用器位于 IPU1-CSI1 之前，同样有五个 sink 管脚，用于在并行传感器和四个
MIPI CSI-2 虚拟通道中任选其一。

在 i.MX6 Quad 上，有两个视频多路复用器实体。一个位于 IPU1-CSI0 之前，用于在
并行传感器和 MIPI CSI-2 虚拟通道 0 之间选择（两个 sink 管脚）。另一个多路复用器
位于 IPU2-CSI1 之前，用于在并行传感器和 MIPI CSI-2 虚拟通道 3 之间选择（两个
sink 管脚）。

### ipuX_csiY

这些是 CSI 实体。它们有一个单独的 sink 管脚，如上所述从视频多路复用器或 MIPI
CSI-2 虚拟通道接收。

该实体有两个 source 管脚。第一个 source 管脚可以使用硬件链路直接链接到
ipuX_vdic 实体或 ipuX_ic_prp 实体，这种链接不需要 IDMAC 内存缓冲区传输。

当直接 source 管脚路由到 ipuX_ic_prp 实体时，来自 CSI 的帧可以由一个或两个 IC
预处理任务处理。

当直接 source 管脚路由到 ipuX_vdic 实体时，VDIC 将使用“高运动”模式执行运动
补偿去隔行（参见 ipuX_vdic 实体描述）。

第二个 source 管脚通过 SMFC 和某个 IDMAC 通道将视频帧直接发送到内存缓冲区，
绕过 IC 预处理。该 source 管脚路由到一个捕获设备节点，节点名称格式为
“ipuX_csiY capture”。

注意，由于 IDMAC source 管脚使用了 IDMAC 通道，因此同一色彩空间内的像素重排序
可以由 IDMAC 通道完成。例如，如果 CSI sink 管脚以 UYVY 顺序接收，则链接到 IDMAC
source 管脚的捕获设备可以以 YUYV 顺序捕获。此外，如果 CSI sink 管脚接收的是
打包（packed）YUV 格式，则捕获设备可以捕获平面（planar）YUV 格式，例如 YUV420。

IDMAC source 管脚处的 IDMAC 通道还支持无运动补偿的简单交织（interweave），当
source 管脚的场（field）类型为顺序顶-底（sequential top-bottom）或底-顶
（bottom-top），且请求的捕获接口场类型设置为隔行（interlaced，t-b、b-t 或未
限定隔行）时激活。捕获接口将强制采用与 source 管脚相同的场顺序（如果 source
管脚为 seq-bt，则为 interlaced-bt；如果 source 管脚为 seq-tb，则为 interlaced-tb）。

关于 ipuX_csiY 产生的事件，请参见 ref:`imx_api_ipuX_csiY`。

### ipuX_csiY 中的裁剪

CSI 支持对输入的原始传感器帧进行裁剪。这在 ipuX_csiY 实体的 sink 管脚处通过
crop selection 子设备 API 实现。

CSI 还支持在宽度和高度上独立的固定二分（divide-by-two）缩小。这在 ipuX_csiY
实体的 sink 管脚处通过 compose selection 子设备 API 实现。

ipuX_csiY source 管脚处的输出矩形与 sink 管脚处的 compose 矩形相同。因此 source
管脚矩形无法进行协商，必须使用 sink 管脚处的 compose selection API 来设置（如果
需要 /2 缩小；否则 source 管脚矩形等于输入矩形）。

作为 crop 和 /2 缩小的示例，这会将一个 1280x960 的输入帧裁剪为 640x480，然后
在两个维度上 /2 缩小到 320x240（假设 ipu1_csi0 链接到 ipu1_csi0_mux）：

   media-ctl -V "'ipu1_csi0_mux':2[fmt:UYVY2X8/1280x960]"
   media-ctl -V "'ipu1_csi0':0[crop:(0,0)/640x480]"
   media-ctl -V "'ipu1_csi0':0[compose:(0,0)/320x240]"

### ipuX_csiY 中的跳帧

CSI 支持通过跳帧进行帧率抽取（frame rate decimation）。帧率抽取通过在 sink 和
source 管脚设置帧间隔来指定。然后 ipuX_csiY 实体将最佳跳帧设置应用到 CSI，以在
source 管脚达到期望的帧率。

以下示例将 IDMAC 输出 source 管脚上假设的 60 Hz 输入帧率减半：

   media-ctl -V "'ipu1_csi0':0[fmt:UYVY2X8/640x480@1/60]"
   media-ctl -V "'ipu1_csi0':2[fmt:UYVY2X8/640x480@1/30]"

### ipuX_csiY 中的帧间隔监视器

请参见 ref:`imx_api_FIM`。

### ipuX_vdic

VDIC 执行运动补偿去隔行，具有三种运动补偿模式：低、中、高运动。模式通过菜单
控件 V4L2_CID_DEINTERLACING_MODE 指定。VDIC 有两个 sink 管脚和一个单独的
source 管脚。

直接 sink 管脚从 ipuX_csiY 直接管脚接收。使用该链接时，VDIC 只能以高运动模式
运行。

当 IDMAC sink 管脚被激活时，它从输出或 mem2mem 设备节点接收。使用该流水线时，
VDIC 也可以以低和中模式运行，因为这些模式需要从内存缓冲区接收帧。注意，输出
或 mem2mem 设备尚未实现，因此该 sink 管脚当前没有任何链接。

source 管脚路由到 IC 预处理实体 ipuX_ic_prp。

### ipuX_ic_prp

这是 IC 预处理实体。它充当路由器，将其 sink 管脚的数据路由到其一个或两个 source
管脚。

该实体有一个单独的 sink 管脚。sink 管脚可以从 ipuX_csiY 直接管脚或 ipuX_vdic
接收。

该实体有两个 source 管脚。一个 source 管脚路由到预处理器编码任务实体
（ipuX_ic_prpenc），另一个路由到预处理器取景器任务实体（ipuX_ic_prpvf）。如果
sink 管脚从 ipuX_csiY 接收，则两个 source 管脚可以同时激活。如果 sink 管脚从
ipuX_vdic 接收，则只能激活到预处理器取景器任务实体的 source 管脚（来自 VDIC 的
帧只能由预处理器取景器任务处理）。

### ipuX_ic_prpenc

这是 IC 预处理编码实体。它有一个来自 ipuX_ic_prp 的单独 sink 管脚，以及一个
单独的 source 管脚。source 管脚路由到一个捕获设备节点，节点名称格式为
“ipuX_ic_prpenc capture”。

该实体执行 IC 预处理编码任务操作：色彩空间转换、缩放（缩小和放大）、水平和垂直
翻转以及 90/270 度旋转。翻转和旋转通过标准 V4L2 控件提供。

与 ipuX_csiY IDMAC source 类似，该实体也支持无运动补偿的简单去隔行，以及像素
重排序。

### ipuX_ic_prpvf

这是 IC 预处理取景器实体。它有一个来自 ipuX_ic_prp 的单独 sink 管脚，以及一个
单独的 source 管脚。source 管脚路由到一个捕获设备节点，节点名称格式为
“ipuX_ic_prpvf capture”。

该实体的操作与 ipuX_ic_prpenc 相同，具有相同的缩放和 CSC 操作以及翻转/旋转
控件。如果 ipuX_ic_prp 从 ipuX_vdic 接收，它将接收并处理来自 ipuX_vdic 的去隔行
帧。

与 ipuX_csiY IDMAC source 类似，该实体支持无运动补偿的简单交织（interweaving）。
但是请注意，如果 ipuX_vdic 包含在流水线中（ipuX_ic_prp 从 ipuX_vdic 接收），则
无法在 ipuX_ic_prpvf 中使用交织，因为 ipuX_vdic 已经执行了去隔行（带运动补偿），
因此 ipuX_vdic 输出的场类型只能是 none（逐行）。

### 捕获流水线

下面描述流水线支持的各种用例。

所示链接不包含后端传感器、视频多路复用器或 mipi csi-2 接收器链接。这取决于
传感器接口类型（并行或 mipi csi-2）。因此这些流水线从以下内容开始：

sensor -> ipuX_csiY_mux -> ...

用于并行传感器，或：

sensor -> imx6-mipi-csi2 -> (ipuX_csiY_mux) -> ...

用于 mipi csi-2 传感器。视 mipi csi-2 虚拟通道而定，imx6-mipi-csi2 接收器可能需要
先路由到视频多路复用器（ipuX_csiY_mux）再发送到 CSI，因此 ipuX_csiY_mux 用
括号表示。

### 未处理视频捕获：

通过 ipuX_csiY IDMAC source 管脚，将帧从传感器直接发送到摄像头设备接口节点，
不做任何转换：

-> ipuX_csiY:2 -> ipuX_csiY capture

### IC 直接转换：

该流水线使用预处理编码实体将帧直接从 CSI 路由到 IC，以执行最高 1024x1024
分辨率的缩放、CSC、翻转以及图像旋转：

-> ipuX_csiY:1 -> 0:ipuX_ic_prp:1 -> 0:ipuX_ic_prpenc:1 -> ipuX_ic_prpenc capture

### 运动补偿去隔行：

该流水线将帧从 CSI 直接管脚路由到 VDIC 实体，以支持运动补偿去隔行（仅高运动
模式）、最高 1024x1024 的缩放、CSC、翻转以及旋转：

-> ipuX_csiY:1 -> 0:ipuX_vdic:2 -> 0:ipuX_ic_prp:2 -> 0:ipuX_ic_prpvf:1 -> ipuX_ic_prpvf capture

### 使用说明

为了辅助配置并与只从视频设备节点访问控件（control）的 V4L2 应用向后兼容，捕获
设备接口会继承当前流水线中活动实体的控件，因此控件既可以直接从子设备访问，也
可以从活动捕获设备接口访问。例如，FIM 控件既可从 ipuX_csiY 子设备获得，也可从
活动捕获设备获得。

以下是针对 Sabre* 参考板的具体使用说明：

### 带 OV5642 和 OV5640 的 i.MX6Q SabreLite

该平台需要带并行摄像头接口的 OmniVision OV5642 模块，以及带 MIPI CSI-2 接口的
OV5640 模块。两个模块均可从 Boundary Devices 获得：

- https://boundarydevices.com/product/nit6x_5mp
- https://boundarydevices.com/product/nit6x_5mp_mipi

注意，如果只有一个摄像头模块可用，则可以在设备树中禁用另一个传感器节点。

OV5642 模块连接到 i.MX 内部视频多路复用器到 IPU1 CSI0 的并行总线输入。它的 i2c
总线连接到 i2c 总线 2。

MIPI CSI-2 OV5640 模块连接到 i.MX 内部 MIPI CSI-2 接收器，来自接收器的四个虚拟
通道输出路由如下：vc0 到 IPU1 CSI0 多路复用器，vc1 直接到 IPU1 CSI1，vc2 直接
到 IPU2 CSI0，vc3 到 IPU2 CSI1 多路复用器。OV5640 也连接到 SabreLite 上的 i2c
总线 2，因此 OV5642 和 OV5640 不能共享相同的 i2c 从地址。

以下基本示例为两个传感器配置未处理视频捕获流水线。OV5642 路由到 ipu1_csi0，
而通过 MIPI CSI-2 虚拟通道 1（即 imx6-mipi-csi2 管脚 2）传输的 OV5640 路由到
ipu1_csi1。两个传感器都配置为输出 640x480，OV5642 输出 YUYV2X8，OV5640 输出
UYVY2X8：

   # Setup links for OV5642
   media-ctl -l "'ov5642 1-0042':0 -> 'ipu1_csi0_mux':1[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':2 -> 'ipu1_csi0 capture':0[^1^]"
   # Setup links for OV5640
   media-ctl -l "'ov5640 1-0040':0 -> 'imx6-mipi-csi2':0[^1^]"
   media-ctl -l "'imx6-mipi-csi2':2 -> 'ipu1_csi1':0[^1^]"
   media-ctl -l "'ipu1_csi1':2 -> 'ipu1_csi1 capture':0[^1^]"
   # Configure pads for OV5642 pipeline
   media-ctl -V "'ov5642 1-0042':0 [fmt:YUYV2X8/640x480 field:none]"
   media-ctl -V "'ipu1_csi0_mux':2 [fmt:YUYV2X8/640x480 field:none]"
   media-ctl -V "'ipu1_csi0':2 [fmt:AYUV32/640x480 field:none]"
   # Configure pads for OV5640 pipeline
   media-ctl -V "'ov5640 1-0040':0 [fmt:UYVY2X8/640x480 field:none]"
   media-ctl -V "'imx6-mipi-csi2':2 [fmt:UYVY2X8/640x480 field:none]"
   media-ctl -V "'ipu1_csi1':2 [fmt:AYUV32/640x480 field:none]"

然后可以在捕获设备节点“ipu1_csi0 capture”和“ipu1_csi1 capture”上独立开始
流传输。v4l2-ctl 工具可用于在捕获设备节点上选择任何受支持的 YUV 像素格式，
包括平面格式。

### 带 ADV7180 解码器的 i.MX6Q SabreAuto

在 i.MX6Q SabreAuto 上，板载 ADV7180 SD 解码器连接到内部视频多路复用器到 IPU1
CSI0 的并行总线输入。

以下示例配置一条流水线，以从 ADV7180 视频解码器捕获，假设 NTSC 720x480 输入
信号，使用简单交织（未转换且无需运动补偿）。adv7180 必须输出顺序或交替场（NTSC
的场类型为“seq-bt”，或“alternate”）：

   # Setup links
   media-ctl -l "'adv7180 3-0021':0 -> 'ipu1_csi0_mux':1[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':2 -> 'ipu1_csi0 capture':0[^1^]"
   # Configure pads
   media-ctl -V "'adv7180 3-0021':0 [fmt:UYVY2X8/720x480 field:seq-bt]"
   media-ctl -V "'ipu1_csi0_mux':2 [fmt:UYVY2X8/720x480]"
   media-ctl -V "'ipu1_csi0':2 [fmt:AYUV32/720x480]"
   # Configure "ipu1_csi0 capture" interface (assumed at /dev/video4)
   v4l2-ctl -d4 --set-fmt-video=field=interlaced_bt

然后可以在 /dev/video4 上开始流传输。v4l2-ctl 工具也可用于在 /dev/video4 上选择
任何受支持的 YUV 像素格式。

此示例配置一条流水线，以从 ADV7180 视频解码器捕获，假设 PAL 720x576 输入信号，
使用运动补偿去隔行。adv7180 必须输出顺序或交替场（PAL 的场类型为“seq-tb”，
或“alternate”）：

   # Setup links
   media-ctl -l "'adv7180 3-0021':0 -> 'ipu1_csi0_mux':1[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':1 -> 'ipu1_vdic':0[^1^]"
   media-ctl -l "'ipu1_vdic':2 -> 'ipu1_ic_prp':0[^1^]"
   media-ctl -l "'ipu1_ic_prp':2 -> 'ipu1_ic_prpvf':0[^1^]"
   media-ctl -l "'ipu1_ic_prpvf':1 -> 'ipu1_ic_prpvf capture':0[^1^]"
   # Configure pads
   media-ctl -V "'adv7180 3-0021':0 [fmt:UYVY2X8/720x576 field:seq-tb]"
   media-ctl -V "'ipu1_csi0_mux':2 [fmt:UYVY2X8/720x576]"
   media-ctl -V "'ipu1_csi0':1 [fmt:AYUV32/720x576]"
   media-ctl -V "'ipu1_vdic':2 [fmt:AYUV32/720x576 field:none]"
   media-ctl -V "'ipu1_ic_prp':2 [fmt:AYUV32/720x576 field:none]"
   media-ctl -V "'ipu1_ic_prpvf':1 [fmt:AYUV32/720x576 field:none]"
   # Configure "ipu1_ic_prpvf capture" interface (assumed at /dev/video2)
   v4l2-ctl -d2 --set-fmt-video=field=none

然后可以在 /dev/video2 上开始流传输。v4l2-ctl 工具也可用于在 /dev/video2 上选择
任何受支持的 YUV 像素格式。

该平台接受 ADV7180 上 Ain1（连接器 J42）的复合视频（Composite Video）模拟输入。

### 带 ADV7180 解码器的 i.MX6DL SabreAuto

在 i.MX6DL SabreAuto 上，板载 ADV7180 SD 解码器连接到内部视频多路复用器到 IPU1
CSI0 的并行总线输入。

以下示例配置一条流水线，以从 ADV7180 视频解码器捕获，假设 NTSC 720x480 输入
信号，使用简单交织（未转换且无需运动补偿）。adv7180 必须输出顺序或交替场（NTSC
的场类型为“seq-bt”，或“alternate”）：

   # Setup links
   media-ctl -l "'adv7180 4-0021':0 -> 'ipu1_csi0_mux':4[^1^]"
   media-ctl -l "'ipu1_csi0_mux':5 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':2 -> 'ipu1_csi0 capture':0[^1^]"
   # Configure pads
   media-ctl -V "'adv7180 4-0021':0 [fmt:UYVY2X8/720x480 field:seq-bt]"
   media-ctl -V "'ipu1_csi0_mux':5 [fmt:UYVY2X8/720x480]"
   media-ctl -V "'ipu1_csi0':2 [fmt:AYUV32/720x480]"
   # Configure "ipu1_csi0 capture" interface (assumed at /dev/video0)
   v4l2-ctl -d0 --set-fmt-video=field=interlaced_bt

然后可以在 /dev/video0 上开始流传输。v4l2-ctl 工具也可用于在 /dev/video0 上选择
任何受支持的 YUV 像素格式。

此示例配置一条流水线，以从 ADV7180 视频解码器捕获，假设 PAL 720x576 输入信号，
使用运动补偿去隔行。adv7180 必须输出顺序或交替场（PAL 的场类型为“seq-tb”，
或“alternate”）：

   # Setup links
   media-ctl -l "'adv7180 4-0021':0 -> 'ipu1_csi0_mux':4[^1^]"
   media-ctl -l "'ipu1_csi0_mux':5 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':1 -> 'ipu1_vdic':0[^1^]"
   media-ctl -l "'ipu1_vdic':2 -> 'ipu1_ic_prp':0[^1^]"
   media-ctl -l "'ipu1_ic_prp':2 -> 'ipu1_ic_prpvf':0[^1^]"
   media-ctl -l "'ipu1_ic_prpvf':1 -> 'ipu1_ic_prpvf capture':0[^1^]"
   # Configure pads
   media-ctl -V "'adv7180 4-0021':0 [fmt:UYVY2X8/720x576 field:seq-tb]"
   media-ctl -V "'ipu1_csi0_mux':5 [fmt:UYVY2X8/720x576]"
   media-ctl -V "'ipu1_csi0':1 [fmt:AYUV32/720x576]"
   media-ctl -V "'ipu1_vdic':2 [fmt:AYUV32/720x576 field:none]"
   media-ctl -V "'ipu1_ic_prp':2 [fmt:AYUV32/720x576 field:none]"
   media-ctl -V "'ipu1_ic_prpvf':1 [fmt:AYUV32/720x576 field:none]"
   # Configure "ipu1_ic_prpvf capture" interface (assumed at /dev/video2)
   v4l2-ctl -d2 --set-fmt-video=field=none

然后可以在 /dev/video2 上开始流传输。v4l2-ctl 工具也可用于在 /dev/video2 上选择
任何受支持的 YUV 像素格式。

该平台接受 ADV7180 上 Ain1（连接器 J42）的复合视频（Composite Video）模拟输入。

### 带 MIPI CSI-2 OV5640 的 i.MX6Q SabreSD

与 i.MX6Q SabreLite 类似，i.MX6Q SabreSD 在 IPU1 CSI0 上支持并行接口的 OV5642
模块，以及 MIPI CSI-2 OV5640 模块。OV5642 连接到 i2c 总线 1，OV5640 连接到 i2c
总线 2。

SabreSD 的设备树包含了并行 OV5642 和 MIPI CSI-2 OV5640 的 OF 图（OF graphs），
但截至本文撰写时，仅 MIPI CSI-2 OV5640 经过测试，因此 OV5642 节点当前被禁用。
OV5640 模块连接到 MIPI 连接器 J5。连接到 SabreSD 板的 OV5640 模块的 NXP 部件号
为 H120729。

以下示例配置未处理视频捕获流水线，以从通过 MIPI CSI-2 虚拟通道 0 传输的 OV5640
捕获：

   # Setup links
   media-ctl -l "'ov5640 1-003c':0 -> 'imx6-mipi-csi2':0[^1^]"
   media-ctl -l "'imx6-mipi-csi2':1 -> 'ipu1_csi0_mux':0[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':2 -> 'ipu1_csi0 capture':0[^1^]"
   # Configure pads
   media-ctl -V "'ov5640 1-003c':0 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'imx6-mipi-csi2':1 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'ipu1_csi0_mux':0 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'ipu1_csi0':0 [fmt:AYUV32/640x480]"

然后可以在“ipu1_csi0 capture”节点上开始流传输。v4l2-ctl 工具可用于在捕获设备
节点上选择任何受支持的像素格式。

要确定与“ipu1_csi0 capture”对应的 /dev/video 节点：

   media-ctl -e "ipu1_csi0 capture"
   /dev/video0

/dev/video0 是这种情况下的流传输元素。

通过 v4l2-ctl 启动流传输：

   v4l2-ctl --stream-mmap -d /dev/video0

通过 Gstreamer 启动流传输并将内容发送到显示器：

   gst-launch-1.0 v4l2src device=/dev/video0 ! kmssink

以下示例配置一条直接转换流水线，以从通过 MIPI CSI-2 虚拟通道 0 传输的 OV5640
捕获。它还展示了在 IC 输出处的色彩空间转换和缩放。

   # Setup links
   media-ctl -l "'ov5640 1-003c':0 -> 'imx6-mipi-csi2':0[^1^]"
   media-ctl -l "'imx6-mipi-csi2':1 -> 'ipu1_csi0_mux':0[^1^]"
   media-ctl -l "'ipu1_csi0_mux':2 -> 'ipu1_csi0':0[^1^]"
   media-ctl -l "'ipu1_csi0':1 -> 'ipu1_ic_prp':0[^1^]"
   media-ctl -l "'ipu1_ic_prp':1 -> 'ipu1_ic_prpenc':0[^1^]"
   media-ctl -l "'ipu1_ic_prpenc':1 -> 'ipu1_ic_prpenc capture':0[^1^]"
   # Configure pads
   media-ctl -V "'ov5640 1-003c':0 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'imx6-mipi-csi2':1 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'ipu1_csi0_mux':2 [fmt:UYVY2X8/640x480]"
   media-ctl -V "'ipu1_csi0':1 [fmt:AYUV32/640x480]"
   media-ctl -V "'ipu1_ic_prp':1 [fmt:AYUV32/640x480]"
   media-ctl -V "'ipu1_ic_prpenc':1 [fmt:ARGB8888_1X32/800x600]"
   # Set a format at the capture interface
   v4l2-ctl -d /dev/video1 --set-fmt-video=pixelformat=RGB3

然后可以在“ipu1_ic_prpenc capture”节点上开始流传输。

要确定与“ipu1_ic_prpenc capture”对应的 /dev/video 节点：

   media-ctl -e "ipu1_ic_prpenc capture"
   /dev/video1

/dev/video1 是这种情况下的流传输元素。

通过 v4l2-ctl 启动流传输：

   v4l2-ctl --stream-mmap -d /dev/video1

通过 Gstreamer 启动流传输并将内容发送到显示器：

   gst-launch-1.0 v4l2src device=/dev/video1 ! kmssink

### 已知问题

1. 当在接近 IC 缩放器 1024x1024 限制的分辨率下使用 90 或 270 度旋转控件，
   并且与平面像素格式（YUV420、YUV422p）结合使用时，帧捕获经常会失败，且
   IDMAC 通道没有帧结束中断。变通方法是，在需要 90 或 270 度旋转时，使用较低的
   分辨率和/或打包格式（YUYV、RGB3 等）。

### 文件列表

drivers/staging/media/imx/
include/media/imx.h
include/linux/imx-media.h

### 参考资料

### 作者

- Steve Longerbeam <steve_longerbeam@mentor.com>
- Philipp Zabel <kernel@pengutronix.de>
- Russell King <linux@armlinux.org.uk>

Copyright (C) 2012-2017 Mentor Graphics Inc.
