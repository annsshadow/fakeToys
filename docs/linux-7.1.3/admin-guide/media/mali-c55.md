
## ARM Mali-C55 Image Signal Processor driver（ARM Mali-C55 图像信号处理器驱动）


## Introduction（简介）


本文件记录了 ARM Mali-C55 图像信号处理器（ISP）的驱动。该驱动位于
drivers/media/platform/arm/mali-c55銆。
Mali-C55 ISP 接收来自传感器的数据，数据可以是原始 Bayer 格式RGB/YUV 格式通过并行接口或内存总线进入，经处理后由内部 DMA 引擎输出。存在两条可能的
输出流水线（不过具体实现可能只装配了一条），分别称为“全分辨率（Full resolution）和“缩放（Downscale）”，但此命名是历史沿用，两条流水线都具备裁剪/缩放能力全分辨率流水线还能输RAW 数据，绕ISP 的大部分处理；缩放流水线则不能输RAW 数据。集成的测试图案发生器可在没有连接相机传感器时驱ISP 并产生图像数据驱动模块名为 mali_c55，通过 CONFIG_VIDEO_MALI_C55 配置选项启用
该驱动实现了 V4L2、Media Controller V4L2 Subdevice 接口，并期望连接ISP
的相机传感器具备 V4L2 子设备接口
## Mali-C55 ISP hardware（Mali-C55 ISP 硬件

下面给出 Mali-C55 ISP 的高层功能视图。ISP 的输入可以来自实时源，或通过 DMA
引擎来自内存输入```

  +---------+    +----------+                                     +--------+
  | Sensor  |--->| CSI-2 Rx |                "Full Resolution"    |  DMA   |
  +---------+    +----------+   |\                 Output    +--->| Writer |
                       |        | \                          |    +--------+
                       |        |  \    +----------+  +------+---> Streaming I/O
  +------------+       +------->|   |   |          |  |
  |            |                |   |-->| Mali-C55 |--+
  | DMA Reader |--------------->|   |   |    ISP   |  |
  |            |                |  /    |          |  |      +---> Streaming I/O
  +------------+                | /     +----------+  |      |
                                |/                    +------+
                                                             |    +--------+
                                                             +--->|  DMA   |
                                               "Downscaled"       | Writer |
                                                  Output          +--------+

```
## Media Controller Topology（媒体控制器拓扑

下面给出一ISP 拓扑的示例（实现于带 IMX415 相机传感器与通用 CSI-2 接收器的
系统）：

    :alt:   mali-c55-graph.dot
    :align: center

该驱动拥4 V4L2 子设备：

- `mali_c55 isp`：负责配置输入裁剪与色彩空间转换
- `mali_c55 tpg`：测试图案发生器，模拟相机传感器
- `mali_c55 resizer fr`：全分辨率流水线resizer
- `mali_c55 resizer ds`：缩放流水线resizer

该驱动拥3 V4L2 视频设备
- `mali-c55 fr`：全分辨率流水线的采集设- `mali-c55 ds`：缩放流水线的采集设- `mali-c55 3a stats`A 统计信息采集设备

帧序列在两个采集设备之间是同步的，也就是说，如果某条流水线比另一条启动得晚，
其缓冲区中返回的序号将与另一条流水线相匹配，而不是从零开始
### Idiosyncrasies（特性差异）


**mali-c55 isp**
`mali-c55 isp` 子设备有一个单一sink 衬垫（pad），所有数据源都应连接到它通过启用相应的媒体链路并禁用其他所有链路来选择活跃的数据源。ISP 有两source
衬垫，反映了其内部路由数据的不同路径。ISP 内部的抽头点（tap point）允许用户分数据，以避开部分或全部硬件处理步骤。下图仅用于说明旁路机制如何工作，并非对
那些处理步骤的真实反映；有关高层功能框图，请参阅 ARM Mali-C55 开发者页面：
```

  +--------------------------------------------------------------+
  |                Possible Internal ISP Data Routes             |
  |          +------------+  +----------+  +------------+        |
  +---+      |            |  |          |  |  Colour    |    +---+
  | 0 |--+-->| Processing |->| Demosaic |->|   Space    |--->| 1 |
  +---+  |   |            |  |          |  | Conversion |    +---+
  |      |   +------------+  +----------+  +------------+        |
  |      |                                                   +---+
  |      +---------------------------------------------------| 2 |
  |                                                          +---+
  |                                                              |
  +--------------------------------------------------------------+


```
    :header-rows: 1

    - - Pad
      - Direction
      - Purpose

    - - 0
      - sink
      - 数据输入，连接到 TPG 与相机传感器

    - - 1
      - source
      - RGB/YUV 数据，连接到 FR DS V4L2 子设
    - - 2
      - source
      - RAW bayer 数据，连接到 FR V4L2 子设
ISP 的输入与输出分辨率均限制640x480 8192x8192 之间，这一点体现在 ISP resizer 子设备的 .set_fmt() 操作中
**mali-c55 resizer fr**
`mali-c55 resizer fr` 子设备有两个 *sink* 衬垫，以反映硬件中不同的插入点（RAW 去马赛克后的数据）：

    :header-rows: 1

    - - Pad
      - Direction
      - Purpose

    - - 0
      - sink
      - 数据输入，连接到 ISP 的去马赛克数据流

    - - 1
      - source
      - 数据输出，连接到采集视频设备

    - - 2
      - sink
      - 数据输入，连接到 ISP raw 数据
实际使用的的数据源通过路由 API 选择；可用两条各含单流的路由
    :header-rows: 1

    - - Sink Pad
      - Source Pad
      - Purpose

    - - 0
      - 1
      - 去马赛克数据路由

    - - 2
      - 1
      - 原始数据路由


如果去马赛克路由处于活跃状态，FR 流水线只能以 RGB/YUV 格式输出。如raw 路由
处于活跃状态，则输出反映输入（可以Bayer RGB/YUV 数据）
## Using the driver to capture video（使用驱动采集视频）


利用媒体控制API，我们可以将输入源与 ISP 配置为以多种格式采集图像。在以下
示例中，媒体图的配置通过 v4l-utils [^1^]_ 软件包的 media-ctl 工具完成，图像的
采集则通过 yavta [^2^]_ 完成
### Configuring the input source（配置输入源

第一步是通过启用正确的媒体链路来设定我们期望的输入源。使用上面的示例拓扑我们可以如下选择 TPG
    media-ctl -l "'lte-csi2-rx':1->'mali-c55 isp':0[^0^]"
    media-ctl -l "'mali-c55 tpg':0->'mali-c55 isp':0[^1^]"

### Configuring which video devices will stream data（配置将流式传输数据的视频设备）


驱动会等待所有视频设备都调用VIDIOC_STREAMON ioctl 之后，才告知传感器开流式传输。为此，我们需要启用到想要使用的视频设备的链路。在下面的示例中，我启用了到两个图像采集视频设备的链路：

    media-ctl -l "'mali-c55 resizer fr':1->'mali-c55 fr':0[^1^]"
    media-ctl -l "'mali-c55 resizer ds':1->'mali-c55 ds':0[^1^]"

### Capturing bayer data from the source and processing to RGB/YUV（采集源端的 bayer 数据并处理为 RGB/YUV

要从源端采集 1920x1080 bayer 数据，并将其推过 ISP 的完整处理流水线，我们需要在
源、ISP resizer 子设备上适当地配置数据格式，并将 FR resizer 的路由设为选择
已处理的数据。resizer source 衬垫上的媒体总线格式将是 RGB121212_1X36 YUV10_1X30，取决于你想要采RGB 还是 YUV。ISP 的去马赛克块原生输出 RGB 数据source 衬垫格式设为 YUV10_1X30 会启用色彩空间转换块
在本示例中，我们RGB565 输出为目标，因此选择 RGB121212_1X36 作为 resizer source
衬垫的格式：

    # Set formats on the TPG and ISP
    media-ctl -V "'mali-c55 tpg':0[fmt:SRGGB20_1X20/1920x1080]"
    media-ctl -V "'mali-c55 isp':0[fmt:SRGGB20_1X20/1920x1080]"
    media-ctl -V "'mali-c55 isp':1[fmt:SRGGB20_1X20/1920x1080]"

    # Set routing on the FR resizer
    media-ctl -R "'mali-c55 resizer fr'[0/0->1/0[^1^],2/0->1/0[^0^]]"

    # Set format on the resizer, must be done AFTER the routing.
    media-ctl -V "'mali-c55 resizer fr':1[fmt:RGB121212_1X36/1920x1080]"

缩放输出也可同时用于流式传输数据。在本例中，由于缩放输出只能采集已处理的数据因此无需设置路由
    # Set format on the resizer
    media-ctl -V "'mali-c55 resizer ds':1[fmt:RGB121212_1X36/1920x1080]"

随后即可FR DS 两个输出的视频设备采集图像（若需要也可同时进行）
    yavta -f RGB565 -s 1920x1080 -c10 /dev/video0
    yavta -f RGB565 -s 1920x1080 -c10 /dev/video1

#### Cropping the image（裁剪图像）


全分辨率与缩放两条流水线都能裁剪到最小分辨率 640x480。要裁剪图像，只需配置
resizer sink 衬垫crop compose 矩形，并在视频设备上设置格式
    media-ctl -V "'mali-c55 resizer fr':0[fmt:RGB121212_1X36/1920x1080 crop:(480,270)/640x480 compose:(0,0)/640x480]"
    media-ctl -V "'mali-c55 resizer fr':1[fmt:RGB121212_1X36/640x480]"
    yavta -f RGB565 -s 640x480 -c10 /dev/video0

#### Downscaling the image（缩小图像）


全分辨率与缩放两条流水线都能将图像缩小至8 倍，前提是遵守最640x480 输出分辨率。为获得最佳图像效果，各方向的缩放比应相同。要配置缩放，我们使resizer sink 衬垫上的 compose 矩形
    media-ctl -V "'mali-c55 resizer fr':0[fmt:RGB121212_1X36/1920x1080 crop:(0,0)/1920x1080 compose:(0,0)/640x480]"
    media-ctl -V "'mali-c55 resizer fr':1[fmt:RGB121212_1X36/640x480]"
    yavta -f RGB565 -s 640x480 -c10 /dev/video0

#### Capturing images in YUV formats（以 YUV 格式采集图像

如果我们需要输YUV 数据而非 RGB，则需要启用色彩空间转换块，方法是resizer
source 衬垫上设MEDIA_BUS_FMT_YUV10_1X30。随后我们可以配置一个采集格式，
例如 NV12（此处为其多平面变体）：

    media-ctl -V "'mali-c55 resizer fr':1[fmt:YUV10_1X30/1920x1080]"
    yavta -f NV12M -s 1920x1080 -c10 /dev/video0

### Capturing RGB data from the source and processing it with the resizers（采集源端的 RGB 数据并用 resizer 处理

Mali-C55 ISP 可与能够输出 RGB 数据的传感器协同工作。在这种情况下，虽然不会使用
任何图像质量块，但仍可以按常规方式裁缩放数据。因此，输入 ISP RGB 数据
仍然经过 ISP 子设备的衬垫 1 进入 resizer
为实现这一点，ISP sink 衬垫的格式被设为 MEDIA_BUS_FMT_RGB202020_1X60——这反映数据要与 ISP 协同工作所必须具备的格式。将相机传感器的输出转换为该格式是外部硬的职责
在本示例中，我们让测试图案发生器为我们提RGB 数据而非 bayer 数据
    media-ctl -V "'mali-c55 tpg':0[fmt:RGB202020_1X60/1920x1080]"
    media-ctl -V "'mali-c55 isp':0[fmt:RGB202020_1X60/1920x1080]"

裁剪或缩放数据的方式与前面所述完全相同
### Capturing raw data from the source and outputting it unmodified（采集源端的 raw 数据并原样输出）


ISP 还能以完全未修改的方式，仅从源端采集 raw 数据并在全分辨率流水线上输出。在这种
情况下，缩放流水线仍可正常处理数据，并且可以同时被使用
要配raw 旁路，需要先配置 FR resizer 子设备的路由表，然后在适当位置设置格式
    media-ctl -R "'mali-c55 resizer fr'[0/0->1/0[^0^],2/0->1/0[^1^]]"
    media-ctl -V "'mali-c55 isp':0[fmt:RGB202020_1X60/1920x1080]"
    media-ctl -V "'mali-c55 resizer fr':2[fmt:RGB202020_1X60/1920x1080]"
    media-ctl -V "'mali-c55 resizer fr':1[fmt:RGB202020_1X60/1920x1080]"

    # Set format on the video device and stream
    yavta -f RGB565 -s 1920x1080 -c10 /dev/video0


## Capturing ISP Statistics（采ISP 统计信息

ISP 能够产生统计信息，供运行在用户空间的图像处理算法使用。这些统计信息可以在
ISP 流式传输期间，通过`mali-c55 3a stats` V4L2 设备排队缓冲区来采集。仅支持
V4L2_META_FMT_MALI_C55_STATS <v4l2-meta-fmt-mali-c55-stats> 格式，因此无需设置
格式
    # We assume the media graph has been configured to support RGB565 capture
    # from the mali-c55 fr V4L2 Device, which is at /dev/video0. The statistics
    # V4L2 device is at /dev/video3

    yavta -f RGB565 -s 1920x1080 -c32 /dev/video0 && \
    yavta -c10 -F /dev/video3

缓冲区的布局`mali_c55_stats_buffer` 描述，但大体上统计信息是为支持三种图处理算法而生成的：AEXP（自动曝光）、AWB（自动白平衡）与 AF（自动对焦）。这统计信息可以取自 Mali-C55 ISP 流水线中的不同位置，即所谓的“抽头点（tap points）”下面这个高层框图旨在说明这些统计信息在何处生成：
```

                  +--> AEXP-2            +----> AEXP-1          +--> AF-0
                  |                      +----> AF-1            |
                  |                      |                      |
      +---------+ |   +--------------+   |   +--------------+   |
      |  Input  +-+-->+ Digital Gain +---+-->+ Black Level  +---+---+
      +---------+     +--------------+       +--------------+       |
  +-----------------------------------------------------------------+
  |
  |   +--------------+ +---------+       +----------------+
  +-->| Sinter Noise +-+  White  +--+--->|  Lens Shading  +--+---------------+
      |   Reduction  | | Balance |  |    |                |  |               |
      +--------------+ +---------+  |    +----------------+  |               |
                                    +---> AEXP-0 (A)         +--> AEXP-0 (B) |
  +--------------------------------------------------------------------------+
  |
  |   +----------------+      +--------------+  +----------------+
  +-->|  Tone mapping  +-+--->| Demosaicing  +->+ Purple Fringe  +-+-----------+
      |                | |    +--------------+  |   Correction   | |           |
      +----------------+ +-> AEXP-IRIDIX        +----------------+ +---> AWB-0 |
  +----------------------------------------------------------------------------+
  |                    +-------------+        +-------------+
  +------------------->|   Colour    +---+--->|    Output   |
                       | Correction  |   |    |  Pipelines  |
                       +-------------+   |    +-------------+
                                         +-->  AWB-1

```
默认情况下，所有统计信息都取自每种算法的第 0 个抽头点；即 AEXP 统计信息来自
AEXP-0 (A)，AWB 统计信息来自 AWB-0，AF 统计信息来自 AF-0。通过编程 ISP 的参数，
可对 AEXP AWB 统计信息的抽头点进行配置

## Programming ISP Parameters（编ISP 参数

ISP 可以从用户空间以各种参数进行编程，以便在视频流开始前及进行中应用到硬件这使用户空间能够动态改变诸如黑电平、白平衡与镜头阴影增益等数值
缓冲区格式及其填充方式由 V4L2_META_FMT_MALI_C55_PARAMS <v4l2-meta-fmt-mali-c55-params>
格式描述，应将其设为 `mali-c55 3a params` 视频节点的数据格式
## References（参考资料）


