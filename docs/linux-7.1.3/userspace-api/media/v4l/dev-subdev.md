

######## 子设备接口（Sub-device Interface）


V4L2 设备的复杂性在于，其硬件通常由
多个需要以受控方式相互协作的集成电路构成，因而产生了复杂的 V4L2 驱动。
这些驱动通常
在软件中反映硬件模型，并将不同的硬件
组件建模为称为子设备（sub-devices）的软件模块。

V4L2 子设备通常为仅内核对象。如果 V4L2 驱动
实现了 media device API，它们将自动继承自
media 实体（media entities）。应用程序将能够枚举这些子设备，
并通过 media 实体（entities）、pads 和
links 的枚举 API 发现硬件拓扑结构。

除了使子设备可被发现之外，驱动还可以选择
让应用程序直接配置它们。当
子设备驱动和 V4L2 设备驱动都支持此功能时，子设备
将具有一个字符设备节点，可在其上调用 ioctl 以

- 查询、读取和写入子设备的控制项（controls）

- 订阅和取消订阅事件并检索事件

- 在独立的 pads 上协商图像格式

- 检查并修改同一实体内 pads 之间的内部数据路由

子设备字符设备节点通常命名为
`/dev/v4l-subdev*`，使用主设备号 81。

驱动可以选择将子设备字符设备限制为只暴露
不修改设备状态的操作。在这种情况下，子设备
在本文档其余部分被称为 `read-only`，并且
相关限制记录在各自的 ioctl 中。


## 控制项（Controls）


大多数 V4L2 控制项由子设备硬件实现。驱动
通常会合并所有控制项并通过视频设备节点暴露出来。
应用程序可以通过单一接口控制所有子设备。

复杂设备有时会在不同的硬件
中实现了相同的控制项。这种情况在嵌入式平台中很常见，
传感器和图像处理硬件都实现了相同的功能，
例如对比度调节、白平衡或坏点校正。
由于 V4L2 控制项 API 不支持在单个设备中存在多个相同的控制项，
因此这些相同控制项中除一个之外的其余都被隐藏。

应用程序可以通过子设备
节点以及 control 中描述的 V4L2 控制项 API 来访问这些被隐藏的控制项。
这些 ioctl 的行为与在 V4L2 设备节点上发出时完全相同，
唯一的区别是它们只处理在
子设备中实现的控制项。

取决于驱动，这些控制项也可能通过
一个（或多个）V4L2 设备节点暴露出来。


## 事件（Events）


V4L2 子设备可以按照 event 中的描述向应用程序通知事件。
该 API 的行为与在 V4L2 设备节点上使用时完全相同，
唯一的区别是它只处理由
子设备生成的事件。取决于驱动，这些事件也可能
被报告到一个（或多个）V4L2 设备节点。



## Pad 级别格式（Pad-level Formats）



    Pad 级别的格式仅适用于那些
    需要向用户空间暴露底层格式配置的非常复杂的设备。通用的
    V4L2 应用程序**不**需要使用本节描述的 API。
    section.


    就本节而言，术语 **format** 指的是
    media 总线数据格式、帧宽度和帧高度的组合。

图像格式通常是在视频采集和输出
设备上通过使用 format 和
selection <VIDIOC_SUBDEV_G_SELECTION> ioctl 来协商的。驱动负责
根据管线输入和/或输出处所请求的格式，配置视频管线中的
每一个模块。

对于复杂设备（例如嵌入式系统中常见的设备），管线输出处相同的
图像尺寸可以通过不同的
硬件配置实现。pipeline-scaling 展示了这样一个例子，其中图像缩放
可以同时在视频传感器和主机图像处理
硬件上执行。



    :alt:   pipeline.dot
    :align: center

    管线上的图像格式协商

    高质量与高速的管线配置



传感器缩放器通常质量不如主机缩放器，但
为了在传感器上进行缩放以实现更高的帧率。
根据使用场景（质量 vs. 速度），管线必须
进行不同的配置。应用程序需要显式地
配置管线中每一处的格式。

实现了 media API <media-controller-intro> 的
驱动可以向应用程序暴露 pad 级别的图像格式配置。当
它们这样做时，应用程序可以使用
VIDIOC_SUBDEV_G_FMT <VIDIOC_SUBDEV_G_FMT> 和
VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 来
按每个 pad 协商格式。

应用程序负责在整个
管线上配置一致的参数，并确保相连的 pads 具有兼容的
格式。管线会在
VIDIOC_STREAMON <VIDIOC_STREAMON> 时检查格式是否不匹配，如果配置
无效，则返回 `EPIPE` 错误码。

可以通过调用
VIDIOC_SUBDEV_G_FMT ioctl 来测试 pad 级别图像格式配置的支持情况，即在 pad
0 上调用。如果驱动返回 `EINVAL` 错误码，则表示该子设备
不支持 pad 级别的格式配置。


### 格式协商（Format Negotiation）


pad 上可接受的格式（通常确实）取决于若干
外部参数，例如其他 pads 上的格式、活动 links，或者
甚至是控制项。要为视频管线中所有 pads 找到
应用程序和驱动都可接受的格式组合，不能
仅依赖格式枚举。因此需要一种格式协商机制。

格式协商机制的核心是 get/set 格式
操作。当 `which` 参数被设置为
V4L2_SUBDEV_FORMAT_TRY <VIDIOC_SUBDEV_G_FMT> 时，
VIDIOC_SUBDEV_G_FMT <VIDIOC_SUBDEV_G_FMT> 和
VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 操作的是
一组与硬件
配置无关的格式参数。修改这些 'try' 格式不会改变设备状态
未被改变（这同时适用于驱动中存储的软件状态
设备本身存储的硬件状态）。

虽然 try 格式不被作为设备状态的一部分保存，但它们存储在
子设备的文件句柄中。一次
VIDIOC_SUBDEV_G_FMT <VIDIOC_SUBDEV_G_FMT> 调用将返回
**在同一子设备文件句柄上**设置的最后一个 try 格式。因此，多个
同时查询同一子设备的应用程序之间
不会相互影响。

为了确定某个特定格式是否被设备支持，
应用程序使用
VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl。驱动会
根据设备要求验证并在需要时修改所请求的 `format`，
然后返回可能已被修改的值。应用程序可以
随后选择尝试不同的格式，或接受返回的值并
继续。

驱动在协商迭代期间返回的格式
保证受设备支持。具体而言，驱动保证：如果将以原样
传递给 VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> 调用，返回的
格式不会被进一步修改
（只要外部参数（例如其他 pads 或 links 上的格式）的
配置未被改变）。

驱动会在子设备内部自动传播格式。当在某个 pad 上设置了 try
或活动格式时，同一子设备上
其他 pads 的相应格式可能会被驱动修改。驱动可自由
根据设备要求修改格式。但是，在可能的情况下它们应当遵守
以下规则：

- 格式应该从 sink pads 传播到 source pads。修改
   source pad 上的格式不应修改任何 sink
   pad 上的格式。

- 使用可变缩放因子缩放帧的子设备，在 sink pads 格式被
   修改时应将缩放因子重置为默认值。如果支持 1:1 缩放比，则意味着
   source pads 的格式应被重置为 sink pads 的格式。


格式不会跨 links 传播，因为那将涉及
从一个子设备文件句柄传播到另一个。
因此，应用程序必须注意使用兼容的格式显式地
配置每条 link 的两端。link 两端
相同的格式保证是兼容的。驱动可以自由接受
符合设备要求的不同格式作为兼容格式。

sample-pipeline-config 展示了一个示例配置序列，
对应于 pipeline-scaling 中描述的管线（表格列
列出了实体名称和 pad 编号）。



    \begingroup
    \scriptsize
    \setlength{\tabcolsep}{2pt}



    :header-rows:  1
    :stub-columns: 0
    :widths: 5 5 5 5 5 5 5

    - -
      - Sensor/0

        format
      - Frontend/0

        format
      - Frontend/1

        format
      - Scaler/0

        format
      - Scaler/0

        compose selection rectangle
      - Scaler/1

        format
    - - 初始状态
      - 2048x1536

        SGRBG8_1X8
      - (default)
      - (default)
      - (default)
      - (default)
      - (default)
    - - 配置前端 sink 格式
      - 2048x1536

        SGRBG8_1X8
      - **2048x1536**

        **SGRBG8_1X8**
      - **2046x1534**

        **SGRBG8_1X8**
      - (default)
      - (default)
      - (default)
    - - 配置缩放器 sink 格式
      - 2048x1536

        SGRBG8_1X8
      - 2048x1536

        SGRBG8_1X8
      - 2046x1534

        SGRBG8_1X8
      - **2046x1534**

        **SGRBG8_1X8**
      - **0,0/2046x1534**
      - **2046x1534**

        **SGRBG8_1X8**
    - - 配置缩放器 sink 合成选择
      - 2048x1536

        SGRBG8_1X8
      - 2048x1536

        SGRBG8_1X8
      - 2046x1534

        SGRBG8_1X8
      - 2046x1534

        SGRBG8_1X8
      - **0,0/1280x960**
      - **1280x960**

        **SGRBG8_1X8**


    \endgroup

1. 初始状态。传感器源 pad 格式被设置为其原生的 3MP
   尺寸，媒体总线码为 V4L2_MBUS_FMT_SGRBG8_1X8。主机前端以及缩放器的
   sink 和 source pads 上的格式为默认值，
   缩放器 sink pad 上的合成矩形也是如此。

2. 应用程序将前端 sink pad 格式的尺寸配置为
   2048x1536，并将其媒体总线码配置为 V4L2_MBUS_FMT_SGRBG_1X8。驱动将
   该格式传播到前端 source pad。

3. 应用程序将缩放器 sink pad 格式的尺寸配置为
   2046x1534，并将媒体总线码配置为 V4L2_MBUS_FMT_SGRBG_1X8，以
   匹配前端 source 的尺寸和媒体总线码。sink pad 上的
   媒体总线码被设置为 V4L2_MBUS_FMT_SGRBG_1X8。驱动将
   尺寸传播到缩放器 sink pad 上的合成选择矩形，
   并将格式传播到缩放器 source pad。

4. 应用程序将缩放器 sink pad 的合成选择
   矩形尺寸配置为 1280x960。驱动将
   该尺寸传播到缩放器 source pad 格式。

当对 try 结果满意后，应用程序可以设置活动
格式，方法是将 `which` 参数设置为
`V4L2_SUBDEV_FORMAT_ACTIVE`。活动格式被驱动修改的方式与 try
格式完全相同。为了避免在格式
协商期间修改硬件状态，应用程序应先协商 try 格式，然后
使用协商最后一轮迭代返回的 try 格式来修改
活动设置。这保证活动格式将
被驱动按原样应用而不会被修改。



### 选择（Selections）：裁剪、缩放与合成


许多子设备支持在其输入或输出 pads 上裁剪帧
（甚至可能同时支持两者）。裁剪用于选择图像中
感兴趣的区域，通常是在图像传感器或视频解码器上。
它也可以作为数字变焦实现的一部分，用于选择
将被放大的图像区域。

裁剪设置由一个裁剪矩形定义，并在
struct `v4l2_rect` 中由左上角的
坐标和矩形尺寸表示。坐标和尺寸
都以像素为单位表示。

与 pad 格式一样，驱动为
selection targets v4l2-selections-common 存储 try 和 active 矩形。

在 sink pads 上，裁剪是相对于当前 pad 格式应用的。
pad 格式表示子设备从管线中前一个模块
接收到的图像尺寸，而裁剪矩形
表示将在子设备内部被进一步传输以进行
处理的子图像。

缩放操作通过将图像缩放到新的
尺寸来改变图像大小。缩放比并未显式指定，而是从
原始和缩放后的图像尺寸中隐含得出。两种尺寸都由
struct `v4l2_rect` 表示。

缩放支持是可选的。当子设备支持时，子设备 sink pad 上的裁剪
矩形会被缩放到使用
以下方式配置的
VIDIOC_SUBDEV_S_SELECTION <VIDIOC_SUBDEV_G_SELECTION> IOCTL
在同一个 pad 上以 `V4L2_SEL_TGT_COMPOSE` 选择目标指定的尺寸。如果
子设备支持缩放但不支持合成，则 top 和 left 值
不会被使用，且必须始终设置为零。

在 source pads 上，裁剪与 sink pads 类似，区别在于
执行裁剪所基于的源尺寸是
sink pad 上的 COMPOSE 矩形。在 sink 和 source pads 上，
裁剪矩形必须完全包含在源图像尺寸
之内，裁剪操作才有效。

驱动应始终使用用户在所有选择目标上请求的
最接近的可能矩形，除非另有明确说明。
`V4L2_SEL_FLAG_GE` 和 `V4L2_SEL_FLAG_LE` 标志可用于将
图像尺寸向上或向下取整。v4l2-selection-flags


### 选择目标的类型



##### 实际目标（Actual targets）


实际目标（不带后缀）反映任意时刻实际的硬件
配置。每个实际目标都对应一个
BOUNDS 目标。


##### BOUNDS 目标


BOUNDS 目标是包含所有有效实际
矩形的最小矩形。然而，可能无法将实际矩形设置为
与 BOUNDS 矩形一样大。这可能是因为，例如，传感器的
像素阵列不是矩形而是十字形或圆形。最大
尺寸也可能小于 BOUNDS 矩形。



### 配置顺序与格式传播


在子设备内部，图像处理的步骤顺序始终是从
sink pad 朝向 source pad。这也反映在用户必须执行配置的
顺序中：所做的更改
将被传播到任何后续阶段。如果不希望这种行为，
用户必须设置 `V4L2_SEL_FLAG_KEEP_CONFIG` 标志。该
标志意味着在任何情况下都不允许
传播更改。根据底层硬件的特性，这还可能导致被访问的矩形被驱动
调整。

某一步骤的坐标始终指向上一步骤的
实际尺寸。此规则的唯一例外是 sink 的 compose
矩形，它指的是 sink compose bounds 矩形 --- 如果
硬件支持的话。

1. Sink pad 格式。用户配置 sink pad 格式。该格式
   定义了实体通过 pad 接收以进行
   进一步处理的图像参数。

2. Sink pad 实际裁剪选择。Sink pad 裁剪定义了针对
   sink pad 格式执行的裁剪。

3. Sink pad 实际合成选择。Sink pad 合成
   矩形的尺寸定义了相对于 sink pad
   裁剪矩形尺寸的缩放比。合成矩形的位置指定了
   实际 sink 合成矩形在 sink compose
   bounds 矩形中的位置。

4. Source pad 实际裁剪选择。Source pad 上的裁剪定义了针对
   sink compose bounds 矩形中图像的裁剪。

5. Source pad 格式。Source pad 格式定义了子设备的输出像素
   格式，以及其他参数，但
   图像宽度和高度除外。宽度和高度由
   source pad 实际裁剪选择的尺寸定义。

访问子设备不支持的上述任何矩形都将
返回 `EINVAL`。任何引用先前不支持的
矩形坐标的矩形，将改为引用先前支持的
矩形。例如，如果不支持 sink 裁剪，则 compose
选择将改为引用 sink pad 格式的尺寸。



    :alt:   subdev-image-processing-crop.svg
    :align: center

    子设备中的图像处理：简单裁剪示例

在上面的示例中，子设备支持在其 sink pad 上进行裁剪。要
配置它，用户在子设备的 sink
pad 上设置媒体总线格式。现在可以在 sink pad 上设置实际的裁剪矩形 --- 该
矩形的位置和尺寸反映了要从 sink 格式中
裁剪出的矩形的位置和尺寸。Sink 裁剪
矩形的尺寸也将是子设备 source
pad 的格式尺寸。



    :alt:   subdev-image-processing-scaling-multi-source.svg
    :align: center

    子设备中的图像处理：多源缩放

在此示例中，子设备能够先裁剪，再缩放，
最后从生成的缩放图像中分别为两个 source pads
单独裁剪。缩放图像在裁剪图像中的位置
在 sink compose 目标中被忽略。两个 source 裁剪
矩形的位置都引用 sink 缩放矩形，分别
从中裁剪出由 source 裁剪矩形指定位置的区域。



    :alt:    subdev-image-processing-full.svg
    :align:  center

    子设备中的图像处理：多 sink 与多 source 的缩放与合成

该子设备驱动支持两个 sink pads 和两个 source pads。来自
两个 sink pads 的图像被分别裁剪，然后缩放，
并在合成 bounds 矩形上进一步合成。由此，两条
独立的流被裁剪，并从子设备的
source pads。


- [subdev-formats](subdev-formats)


## 流、多路复用 media pads 与内部路由


简单的 V4L2 子设备不支持多个不相关的视频流，
只有单个流可以通过一个 media link 和一个 media pad。
因此每个 pad 包含该
单个流的格式和选择配置。子设备可以进行流处理并将一个流拆分为
两个，或将两个流合成为一个，但子设备的
输入和输出仍然是每个 pad 一个流。

某些硬件（例如 MIPI CSI-2）支持多路复用流，即多个
数据流在同一总线上传输，这由一个连接发送端 source pad 与接收端
sink pad 的 media link 来表示。例如，
摄像头传感器可以产生两个不同的流：像素流和
元数据流，它们在多路复用的数据总线上传输，由连接
单一传感器 source pad 与接收端
sink pad 的 media link 表示。支持流的接收端会对其 sink pad 上
接收到的流进行解复用，并允许将它们分别
路由到其某个 source pad。

支持多路复用流的子设备驱动与非
多路复用子设备驱动兼容。但是，如果一条 link 的 sink 端的驱动
不支持流，则只能捕获 source 端的流 0。
可能还存在特定于 sink 设备的其他限制。

### 理解流（streams）


流是内容（例如像素数据或元数据）的流，它流经
media 管线，从源（例如传感器）流向最终的 sink（例如
SoC 中的接收端和解复用器）。每条 media link 承载所有已启用的
流，从 link 的一端到另一端，子设备具有路由
表，描述来自 sink pads 的传入流如何被路由到
source pads。

流 ID 是流的 media pad 本地标识符。同一个
流的流 ID 必须在 link 的两端相等。换句话说，
特定的流 ID 必须存在于 media
link 的两侧，但子设备的
另一侧可以使用另一个流 ID 来表示同一个流。

media 管线中特定位置的流由
子设备和一个 (pad, stream) 对来标识。对于不支持
多路复用流的子设备，'stream' 字段始终为 0。

### 路由、流、格式与选择之间的交互


向 V4L2 子设备接口添加流后，子设备的
格式和选择从 pads 转移到 (pad, stream) 对。除了
通常的 pad 之外，设置格式和
选择时还需要提供流 ID。沿流配置格式和选择的顺序
与没有流时相同（见 format-propagation）。

与子设备范围内将所有 sink pads 的流合并到
所有 source pads 不同，每条路由的数据流彼此
独立。在驱动支持的范围内，允许任意数量的从 sink pads 上的流到
source pads 上的流的路由。但是，对于
source pad 上的每个流，只允许一条路由。

pad 内某个流的任何配置（例如格式或选择）
都独立于其他流上的类似配置。这一点
将来可能会改变。

### 设备类型与路由设置


不同种类的子设备对于路由激活具有不同的行为，
这取决于硬件。但在所有情况下，只有设置了
`V4L2_SUBDEV_ROUTE_FL_ACTIVE` 标志的路由才是活动的。

生成流的设备可能允许启用和禁用某些
路由，或者具有固定的路由配置。如果路由可以被禁用，则不在
`VIDIOC_SUBDEV_S_ROUTING` 中声明路由（或声明时不设置 `V4L2_SUBDEV_ROUTE_FL_ACTIVE`
标志）将禁用这些路由。
`VIDIOC_SUBDEV_S_ROUTING` 仍会将这些路由返回给用户，
位于 routes 数组中，且 `V4L2_SUBDEV_ROUTE_FL_ACTIVE` 标志未设置。

传输流的设备几乎总是在路由方面具有更高的
可配置性。通常，子设备 sink 和 source
pads 之间的任何路由都是可能的，并且多个路由（通常上限为某个有限数量）可以
同时处于活动状态。对于此类设备，驱动不会创建任何路由，
当用户在子设备上调用 `VIDIOC_SUBDEV_S_ROUTING` 时，
用户创建的路由会被完全替换。这些新创建的路由具有设备默认的
格式和选择矩形配置。

### 配置流


流的配置是针对每个子设备单独进行的，
子设备之间流的有效性在管线
启动时进行验证。

配置流分为

1. 设置 links。使用
   Media Controller API <media_controller> 连接子设备之间的 pads。

2. 流。通过为子设备设置路由表来声明流并配置其路由，使用
   :ref:`VIDIOC_SUBDEV_S_ROUTING
   <VIDIOC_SUBDEV_G_ROUTING>` ioctl。注意，设置路由表会
   将子设备中的格式和选择重置为默认值。

3. 配置格式和选择。每个流的格式和选择
   按照纯子设备的文档（见 format-propagation）分别配置。
   流 ID 被设置为与通过
   VIDIOC_SUBDEV_S_ROUTING <VIDIOC_SUBDEV_G_ROUTING> ioctl 配置的路由的
   sink 或 source pads 相关联的同一个流 ID。

### 多路复用流配置示例


一个多路复用流配置的简单示例如下：

- 两个相同的传感器（Sensor A 和 Sensor B）。每个传感器有一个 source
  pad（pad 0），承载一个像素数据流。

- 多路复用器桥（Bridge）。该桥有两个 sink pads，连接到
  传感器（pads 0、1），以及一个 source pad（pad 2），输出两个流。

- SoC 中的接收端（Receiver）。接收端有一个 sink pad（pad 0），
  连接到桥，以及两个 source pads（pads 1-2），通向 DMA
  引擎。接收端将传入的流解复用到 source pads。

- SoC 中的 DMA 引擎（DMA Engine），每个流一个。每个 DMA 引擎
  连接到接收端中的一个 source pad。

传感器、桥和接收端被建模为 V4L2 子设备，
通过 /dev/v4l-subdevX 设备节点暴露给用户空间。DMA 引擎被
建模为 V4L2 设备，通过 /dev/videoX 节点暴露给用户空间。

要配置此管线，用户空间必须采取以下步骤：

1. 在实体之间建立 media links：将传感器连接到桥，
   桥连接到接收端，接收端连接到 DMA 引擎。此步骤与
   普通的未多路复用 media controller 设置没有区别。

2. 配置路由

    :header-rows:  1

    - - Sink Pad/流
      - Source Pad/流
      - 路由标志
      - 注释
    - - 0/0
      - 2/0
      - V4L2_SUBDEV_ROUTE_FL_ACTIVE
      - 来自 Sensor A 的像素数据流
    - - 1/0
      - 2/1
      - V4L2_SUBDEV_ROUTE_FL_ACTIVE
      - 来自 Sensor B 的像素数据流

    :header-rows:  1

    - - Sink Pad/流
      - Source Pad/流
      - 路由标志
      - 注释
    - - 0/0
      - 1/0
      - V4L2_SUBDEV_ROUTE_FL_ACTIVE
      - 来自 Sensor A 的像素数据流
    - - 0/1
      - 2/0
      - V4L2_SUBDEV_ROUTE_FL_ACTIVE
      - 来自 Sensor B 的像素数据流

3. 配置格式和选择

   配置路由之后，下一步是配置流的格式和
   选择。这与在没有流的情况下执行此步骤类似，
   只有一个例外：`stream` 字段需要被赋值为
   流 ID 的值。

   实现这一点的常见方法是从传感器开始，沿流
   向接收端传播配置，使用
   VIDIOC_SUBDEV_S_FMT <VIDIOC_SUBDEV_G_FMT> ioctl 来配置每个
   子设备中的每个流端点。
