######## 内存到内存有状态视频解码器接口


有状态（stateful）视频解码器接收完整的数据流数据块（例如 Annex-B
H.264/HEVC 流、原VP8/9 流），并将其解码为按显示顺序排列的原始视频帧。解码器
在处理这些缓冲区时，不应需要来自客户端的任何额外信息

强烈不建议在驱动中对该数据流进行软件解析、处理等操作以支持本接口。如果确实需
此类操作，强烈建议使用无状态（Stateless）视频解码器接口（开发中）

## 本文档使用的约定与记

1. 若本文档未另有说明，则通用V4L2 API 规则适用

2. 词语 "must"may"should" 等的含义`RFC
   2119 <https://tools.ietf.org/html/rfc2119>`_ 为准

3. 所有未标注 "optional" 的步骤都是必需的

4. 除非另有说明，`VIDIOC_G_EXT_CTRLS` `VIDIOC_S_EXT_CTRLS` 可与
   `VIDIOC_G_CTRL` `VIDIOC_S_CTRL` 互换使用

5. 单平面（single-planar）API（见 planar-apis）及适用的结构体，与多平
   （multi-planar）API 可在满足解码器能力并遵循通用 V4L2 指南的前提下互换使用
   除非另有说明

6. i = [a..b]：从 a b（含端点）的整数序列，即 i = [0..2] 表示 i = 0, 1, 2

7. 给定一`OUTPUT` 缓冲A，则 A' 表示 `CAPTURE` 队列上包含由处理缓冲A
   所得数据的缓冲区

## 术语

CAPTURE
   目标缓冲区队列；对于解码器，是包含已解码帧的缓冲区队列；对于编码器，是包
   已编码数据流的缓冲区队列；对`V4L2_BUF_TYPE_VIDEO_CAPTURE` 
   `V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`；数据由硬件捕获`CAPTURE` 缓冲区中

client
   与实现了本接口的解码器或编码器通信的应用程序

coded format
   已编压缩的视频数据流格式（例H.264、VP8 等）；另见：raw format

coded height
   给定编码分辨率下的高度

coded resolution
   以像素计的流分辨率，对齐到编解码器和硬件的要求；通常为可见分辨率向上取整
   完整的宏块；另见：visible resolution

coded width
   给定编码分辨率下的宽度

coding tree unit
   HEVC 编解码器的处理单元（对应H.264、VP8、VP9 中的宏块单元），可使用最
   64×64 像素的块结构。擅长将图像细分为可变大小的结构

decode order
   帧被解码的顺序；如果编码格式包含帧重排序特性，则可能与显示顺序不同；对于解码器
   `OUTPUT` 缓冲区必须由客户端按解码顺序入队；对于编码器 `CAPTURE` 缓冲区必
   由编码器按解码顺序返回

destination
   解码过程产生的数据；`CAPTURE`

display order
   帧必须被显示的顺序；对于编码器，`OUTPUT` 缓冲区必须由客户端按显示顺序入队
   对于解码器，`CAPTURE` 缓冲区必须由解码器按显示顺序返回

DPB
   Decoded Picture Buffer（已解码图像缓冲区）；H.264/HEVC 中的一个术语，指用于存
   已解码原始帧、供后续解码步骤参考的缓冲区

EOS
   end of stream（流结束）

IDR
   Instantaneous Decoder Refresh（即时解码刷新）；H.264/HEVC 编码流中的一种关键帧
   类型，它会清除较早参考帧（DPB）的列表

keyframe
   不引用较早已解码帧的编码帧，即可以独立完整地解码

macroblock
   基于线性块变换的图像和视频压缩格式中的处理单元（例H.264、VP8、VP9）；与具
   编解码器相关，但大多数流行编解码器的尺寸16x16 采样（像素）。HEVC 编解码器
   使用一种更灵活的处理单元，称为 coding tree unit（CTU）

OUTPUT
   源缓冲区队列；对于解码器，是包含已编码数据流的缓冲区队列；对于编码器，是包含
   原始帧的缓冲区队列；对应 `V4L2_BUF_TYPE_VIDEO_OUTPUT` 
   `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`；硬件从 `OUTPUT` 缓冲区获取数据

PPS
   Picture Parameter Set（图像参数集）；H.264/HEVC 数据流中的一种元数据实体

raw format
   包含原始像素数据的未压缩格式（例YUV、RGB 格式）

resume point
   数据流中可以开继续解码、且不存在任何先前状数据的点，例如：关键
   （VP8/VP9）或 SPS/PPS/IDR 序列（H.264/HEVC）；开始解码一条新流，或在 seek
   之后恢复解码，都需要一个恢复点（resume point）

source
   馈送给解码器或编码器的数据；见 `OUTPUT`

source height
   给定源分辨率下的像素高度；仅与编码器相关

source resolution
   馈送给编码器的源帧的像素分辨率，并受限于进一步裁剪到可见分辨率的边界；仅
   编码器相关

source width
   给定源分辨率下的像素宽度；仅与编码器相关

SPS
   Sequence Parameter Set（序列参数集）；H.264/HEVC 数据流中的一种元数据实体

stream metadata
   包含在已编码数据流中的附加（非视觉）信息；例如：编码分辨率、可见分辨率
   编解码器档次（profile）

visible height
   给定可见分辨率下的高度；即显示高度

visible resolution
   可见图像的流分辨率（像素），用于显示目的；必须小于或等于编码分辨率；
   显示分辨率

visible width
   给定可见分辨率下的宽度；即显示宽度

## 状态机

   :alt: DOT digraph of decoder state machine
   :caption: Decoder State Machine

   digraph decoder_state_machine {
       node [shape = doublecircle, label="Decoding"] Decoding;

       node [shape = circle, label="Initialization"] Initialization;
       node [shape = circle, label="Capture\nsetup"] CaptureSetup;
       node [shape = circle, label="Dynamic\nResolution\nChange"] ResChange;
       node [shape = circle, label="Stopped"] Stopped;
       node [shape = circle, label="Drain"] Drain;
       node [shape = circle, label="Seek"] Seek;
       node [shape = circle, label="End of Stream"] EoS;

       node [shape = point]; qi
       qi -> Initialization [ label = "open()" ];

       Initialization -> CaptureSetup [ label = "CAPTURE\nformat\nestablished" ];

       CaptureSetup -> Stopped [ label = "CAPTURE\nbuffers\nready" ];

       Decoding -> ResChange [ label = "Stream\nresolution\nchange" ];
       Decoding -> Drain [ label = "V4L2_DEC_CMD_STOP" ];
       Decoding -> EoS [ label = "EoS mark\nin the stream" ];
       Decoding -> Seek [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];
       Decoding -> Stopped [ label = "VIDIOC_STREAMOFF(CAPTURE)" ];
       Decoding -> Decoding;

       ResChange -> CaptureSetup [ label = "CAPTURE\nformat\nestablished" ];
       ResChange -> Seek [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];

       EoS -> Drain [ label = "Implicit\ndrain" ];

       Drain -> Stopped [ label = "All CAPTURE\nbuffers dequeued\nor\nVIDIOC_STREAMOFF(CAPTURE)" ];
       Drain -> Seek [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];

       Seek -> Decoding [ label = "VIDIOC_STREAMON(OUTPUT)" ];
       Seek -> Initialization [ label = "VIDIOC_REQBUFS(OUTPUT, 0)" ];

       Stopped -> Decoding [ label = "V4L2_DEC_CMD_START\nor\nVIDIOC_STREAMON(CAPTURE)" ];
       Stopped -> Seek [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];
   }

## 查询能力

1. 要枚举解码器支持的编码格式集合，客户端可以在 `OUTPUT` 上调
   `VIDIOC_ENUM_FMT`銆。

   - 无论 `CAPTURE` 上设置的是什么格式，都会返回受支持格式的全部集合
   - 检`v4l2_fmtdesc` flags 字段，以了解解码器相对于每种编码格式
     能力。尤其是解码器是否具有完备的数据流解析器，以及是否支持动态分辨率
     变化

2. 要枚举受支持的原始（raw）格式集合，客户端可以在 `CAPTURE` 上调
   `VIDIOC_ENUM_FMT`銆。

   - 只会返回当前`OUTPUT` 上处于活动状态的格式所支持的格式

   - 为了枚举某给定编码格式所支持的原始格式，客户端必须先`OUTPUT` 
     设置该编码格式，然后再在 `CAPTURE` 上枚举格式

3. 客户端可以使`VIDIOC_ENUM_FRAMESIZES` 来检测给定格式支持的
   分辨率，方法是把期望的像素格式传`v4l2_frmsizeenum` `pixel_format`
   字段

   - `VIDIOC_ENUM_FRAMESIZES` 针对编码像素格式返回的值，将包含解码器
     针对给定编码像素格式支持的所有可能编码分辨率

   - `VIDIOC_ENUM_FRAMESIZES` 针对原始像素格式返回的值，将包含解码器
     针对给定原始像素格式以及当前`OUTPUT` 上设置的编码格式支持的所
     可能帧缓冲区分辨率

4. 对于当前`OUTPUT` 上设置的编码格式，如果适用，其支持的档次（profile
   和级别（level）可以通过各自的控件，经由 `VIDIOC_QUERYCTRL` 查询

## 初始

1. 通过 `VIDIOC_S_FMT` `OUTPUT` 上设置编码格式

   - **必填字段*

     `type`
         `OUTPUT` 适用`V4L2_BUF_TYPE_*` 枚举值

     `pixelformat`
         一种编码像素格式

     `width`、`height`
         数据流的编码分辨率；仅当无法从数据流中针对给定编码格式解析出该值时
         才需要设置；否则解码器会将该分辨率用作占位分辨率，一旦能从数据流中解析出
         实际编码分辨率，该值就可能会改变

     `sizeimage`
         `OUTPUT` 缓冲区的期望大小；解码器可对其进行调整以匹配硬件要求

     other fields
         遵循标准语义

   - **返回字段*

     `sizeimage`
         调整后的 `OUTPUT` 缓冲区大小

   - `CAPTURE` 格式会根`VIDIOC_S_FMT` 返回的宽度和高度，立即更新为
     合适的帧缓冲区分辨率。但是，对于包含流分辨率信息的编码格式，在解码器完成
     从数据流中解析该信息后，无论其是否与客户端设置的值匹配，它都会用新值更
     `CAPTURE` 格式并发出源变化（source change）事件

```

      Changing the ``OUTPUT`` format may change the currently set ``CAPTURE``
      format. How the new ``CAPTURE`` format is determined is up to the decoder
      and the client must ensure it matches its needs afterwards.

```

2. 通过 `VIDIOC_REQBUFS` `OUTPUT` 上分配源（bytestream）缓冲区

    - **必填字段*

      `count`
          请求的缓冲区分配数量；必须大于零

      `type`
          `OUTPUT` 适用`V4L2_BUF_TYPE_*` 枚举值

      `memory`
          遵循标准语义

    - **返回字段*

      `count`
          实际分配的缓冲区数量

```

       The actual number of allocated buffers may differ from the ``count``
       given. The client must check the updated value of ``count`` after the
       call returns.

    Alternatively, :c:func:`VIDIOC_CREATE_BUFS` on the ``OUTPUT`` queue can be
    used to have more control over buffer allocation.

    * **Required fields:**

      ``count``
          requested number of buffers to allocate; greater than zero.

      ``type``
          a ``V4L2_BUF_TYPE_*`` enum appropriate for ``OUTPUT``.

      ``memory``
          follows standard semantics.

      ``format``
          follows standard semantics.

    * **Returned fields:**

      ``count``
          adjusted to the number of allocated buffers.

    .. warning::

       The actual number of allocated buffers may differ from the ``count``
       given. The client must check the updated value of ``count`` after the
       call returns.

```

3. 通过 `VIDIOC_STREAMON` `OUTPUT` 队列上启动数据流（streaming）

4. **此步骤仅适用于在流中包含分辨率信息的编码格式* 继续通过 `VIDIOC_QBUF`
   `VIDIOC_DQBUF` `OUTPUT` 队列上入出队数据流缓冲区。缓冲区将被
   按顺序处理并返还给客户端，直到找到配`CAPTURE` 队列所需的元数据为止。这
   解码器发`changes` 设为 `V4L2_EVENT_SRC_CH_RESOLUTION` 
   `V4L2_EVENT_SOURCE_CHANGE` 事件来指示

    - 如果第一个缓冲区包含的数据不足以触发该事件，这不算错误。只要还需要更
      数据，就会继续处理缓冲区

    - 如果触发该事件的缓冲区中的数据是解码第一帧所必需的，那么在初始化序列
      完成且该帧被解码之前，该缓冲区不会被返还给客户端

    - 如果客户端没有自行设置数据流的编码分辨率，那么在 `CAPTURE` 队列上调
      `VIDIOC_G_FMT`、`VIDIOC_S_FMT`、`VIDIOC_TRY_FMT` 
      `VIDIOC_REQBUFS`，在发出 `changes` 设为
      `V4L2_EVENT_SRC_CH_RESOLUTION` `V4L2_EVENT_SOURCE_CHANGE` 事件
      之前，都不会返回数据流的真实值

```

       Any client query issued after the decoder queues the event will return
       values applying to the just parsed stream, including queue formats,
       selection rectangles and controls.

    .. note::

       A client capable of acquiring stream parameters from the bytestream on
       its own may attempt to set the width and height of the ``OUTPUT`` format
       to non-zero values matching the coded size of the stream, skip this step
       and continue with the `Capture Setup` sequence. However, it must not
       rely on any driver queries regarding stream parameters, such as
       selection rectangles and controls, since the decoder has not parsed them
       from the stream yet. If the values configured by the client do not match
       those parsed by the decoder, a `Dynamic Resolution Change` will be
       triggered to reconfigure them.

    .. note::

       No decoded frames are produced during this phase.

```

5. 继续 `Capture Setup` 序列

## 捕获设置（Capture Setup

1. `CAPTURE` 队列上调`VIDIOC_G_FMT`，以获取从数据流中解解码出的
    目标缓冲区的格式

    - **必填字段*

      `type`
          `CAPTURE` 适用`V4L2_BUF_TYPE_*` 枚举值

    - **返回字段*

      `width`、`height`
          已解码帧的帧缓冲区分辨率

      `pixelformat`
          已解码帧的像素格式

      `num_planes`（仅适用_MPLANE `type`
          pixelformat 的平面数量

      `sizeimage`、`bytesperline`
          遵循标准语义；与帧缓冲区格式匹配

```

       The value of ``pixelformat`` may be any pixel format supported by the
       decoder for the current stream. The decoder should choose a
       preferred/optimal format for the default configuration. For example, a
       YUV format may be preferred over an RGB format if an additional
       conversion step would be required for the latter.

```

2. **可选* 通过 `VIDIOC_G_SELECTION` 获取可见分辨率

    - **必填字段*

      `type`
          `CAPTURE` 适用`V4L2_BUF_TYPE_*` 枚举值

      `target`
          设为 `V4L2_SEL_TGT_COMPOSE`

    - **返回字段*

      `r.left`、`r.top`、`r.width`、`r.height`
          可见矩形；它必须落在 `CAPTURE` `VIDIOC_G_FMT` 返回的帧缓冲
          分辨率之内

    - `CAPTURE` 上支持以下选择目标

      `V4L2_SEL_TGT_CROP_BOUNDS`
          对应于数据流的编码分辨率

      `V4L2_SEL_TGT_CROP_DEFAULT`
          覆盖 `CAPTURE` 缓冲区中包含有意义图像数据（可见区域）部分的矩形
          其宽度和高度等于数据流的可见分辨率

      `V4L2_SEL_TGT_CROP`
          编码分辨率内将要输出`CAPTURE` 的矩形；默认等于
          `V4L2_SEL_TGT_CROP_DEFAULT`；在不具备额compose/缩放能力的硬件上
          为只读

      `V4L2_SEL_TGT_COMPOSE_BOUNDS`
          `CAPTURE` 缓冲区中裁剪后帧可被合成到的最大矩形；若硬件不支持
          compose/缩放，则等于 `V4L2_SEL_TGT_CROP`

      `V4L2_SEL_TGT_COMPOSE_DEFAULT`
          等于 `V4L2_SEL_TGT_CROP`

      `V4L2_SEL_TGT_COMPOSE`
          `CAPTURE` 缓冲区中写入裁剪后帧的矩形；默认等于
          `V4L2_SEL_TGT_COMPOSE_DEFAULT`；在不具备额compose/缩放能力的硬件上
          为只读

      `V4L2_SEL_TGT_COMPOSE_PADDED`
          `CAPTURE` 缓冲区中被硬件覆盖的矩形；若硬件不写入填充像素，则等
          `V4L2_SEL_TGT_COMPOSE`銆。

```

       The values are guaranteed to be meaningful only after the decoder
       successfully parses the stream metadata. The client must not rely on the
       query before that happens.

```

3. **可选* 通过 `CAPTURE` 队列上的 `VIDIOC_ENUM_FMT` 枚举
    `CAPTURE` 格式。一旦流信息被解析并已知，客户端可以使用ioctl 来发
    给定流支持哪些原始格式，并通过 `VIDIOC_S_FMT` 选择其中之一

```

       The decoder will return only formats supported for the currently
       established coded format, as per the ``OUTPUT`` format and/or stream
       metadata parsed in this initialization sequence, even if more formats
       may be supported by the decoder in general. In other words, the set
       returned will be a subset of the initial query mentioned in the
       `Querying Capabilities` section.

       For example, a decoder may support YUV and RGB formats for resolutions
       1920x1088 and lower, but only YUV for higher resolutions (due to
       hardware limitations). After parsing a resolution of 1920x1088 or lower,
       :c:func:`VIDIOC_ENUM_FMT` may return a set of YUV and RGB pixel formats,
       but after parsing resolution higher than 1920x1088, the decoder will not
       return RGB, unsupported for this resolution.

       However, subsequent resolution change event triggered after
       discovering a resolution change within the same stream may switch
       the stream into a lower resolution and :c:func:`VIDIOC_ENUM_FMT`
       would return RGB formats again in that case.

```

4. **可选* 通过 `CAPTURE` 队列上的 `VIDIOC_S_FMT` 设置
    `CAPTURE` 格式。客户端可以选择不同于解码器`VIDIOC_G_FMT` 
    选择/建议的格式

    - **必填字段*

      `type`
          `CAPTURE` 适用`V4L2_BUF_TYPE_*` 枚举值

      `pixelformat`
          一种原始像素格式

      `width`、`height`
          已解码流的帧缓冲区分辨率；通常`VIDIOC_G_FMT` 返回的值相同，
          但如果硬件支持合成（composition）和/或缩放，则可能不同

   - 如前一节所述，设置 `CAPTURE` 格式会根据新分辨率将 compose 选择矩形
     重置为它们的默认值

5. **可选* 如果期望且解码器具备 compose 或缩放能力，通过 `CAPTURE`
   队列上的 `VIDIOC_S_SELECTION` 设置 compose 矩形

   - **必填字段*

     `type`
         `CAPTURE` 适用`V4L2_BUF_TYPE_*` 枚举值

     `target`
         设为 `V4L2_SEL_TGT_COMPOSE`

     `r.left`、`r.top`、`r.width`、`r.height`
         `CAPTURE` 缓冲区中写入裁剪后帧的矩形；默认等于
         `V4L2_SEL_TGT_COMPOSE_DEFAULT`；在不具备额compose/缩放能力
         硬件上为只读

   - **返回字段*

     `r.left`、`r.top`、`r.width`、`r.height`
         可见矩形；它必须落在 `CAPTURE` `VIDIOC_G_FMT` 返回的帧缓冲
         分辨率之内

```

      The decoder may adjust the compose rectangle to the nearest
      supported one to meet codec and hardware requirements. The client needs
      to check the adjusted rectangle returned by :c:func:`VIDIOC_S_SELECTION`.

```

6. 如果满足以下所有条件，客户端可以立即恢复解码：

    - 新格式（在前面步骤中确定）的 `sizeimage` 小于或等于当前已分配缓冲区的
      大小

    - 当前已分配的缓冲区数量大于或等于前面步骤中获取的最小缓冲区数量。为满足
      此要求，客户端可以使`VIDIOC_CREATE_BUFS` 来新增缓冲区

    在这种情况下，剩余步骤不适用，客户端可以通过下列操作之一恢复解码

    - 如果 `CAPTURE` 队列正在流式传输，则使用 `V4L2_DEC_CMD_START` 命令
      调用 `VIDIOC_DECODER_CMD`

    - 如果 `CAPTURE` 队列未流式传输，则在 `CAPTURE` 队列上调
      `VIDIOC_STREAMON`銆。

    但是，如果客户端出于降低内存占用或其他任何原因打算更改缓冲区集合，则可以
    通过执行以下步骤实现

7. **如果** `CAPTURE` **队列正在流式传输* 继续`CAPTURE` 队列
    入队和出队缓冲区，直到出一个带`V4L2_BUF_FLAG_LAST` 标志的缓冲区

8. **如果** `CAPTURE` **队列正在流式传输* `CAPTURE` 队列上调
    `VIDIOC_STREAMOFF` 以停止流式传输

```

       The ``OUTPUT`` queue must remain streaming. Calling
       :c:func:`VIDIOC_STREAMOFF` on it would abort the sequence and trigger a
       seek.

```

9. **如果** `CAPTURE` **队列已分配缓冲区* 使用 `VIDIOC_REQBUFS`
    释放 `CAPTURE` 缓冲区

    - **必填字段*

      `count`
          设为 0

      `type`
          `CAPTURE` 适用`V4L2_BUF_TYPE_*` 枚举值

      `memory`
          遵循标准语义

10. 通过 `CAPTURE` 队列上的 `VIDIOC_REQBUFS` 分配 `CAPTURE` 缓冲区

    - **必填字段*

      `count`
          请求的缓冲区分配数量；必须大于零

      `type`
          `CAPTURE` 适用`V4L2_BUF_TYPE_*` 枚举值

      `memory`
          遵循标准语义

    - **返回字段*

      `count`
          实际分配的缓冲区数量

```

       The actual number of allocated buffers may differ from the ``count``
       given. The client must check the updated value of ``count`` after the
       call returns.

    .. note::

       To allocate more than the minimum number of buffers (for pipeline
       depth), the client may query the ``V4L2_CID_MIN_BUFFERS_FOR_CAPTURE``
       control to get the minimum number of buffers required, and pass the
       obtained value plus the number of additional buffers needed in the
       ``count`` field to :c:func:`VIDIOC_REQBUFS`.

    Alternatively, :c:func:`VIDIOC_CREATE_BUFS` on the ``CAPTURE`` queue can be
    used to have more control over buffer allocation. For example, by
    allocating buffers larger than the current ``CAPTURE`` format, future
    resolution changes can be accommodated.

    * **Required fields:**

      ``count``
          requested number of buffers to allocate; greater than zero.

      ``type``
          a ``V4L2_BUF_TYPE_*`` enum appropriate for ``CAPTURE``.

      ``memory``
          follows standard semantics.

      ``format``
          a format representing the maximum framebuffer resolution to be
          accommodated by newly allocated buffers.

    * **Returned fields:**

      ``count``
          adjusted to the number of allocated buffers.

    .. warning::

        The actual number of allocated buffers may differ from the ``count``
        given. The client must check the updated value of ``count`` after the
        call returns.

    .. note::

       To allocate buffers for a format different than parsed from the stream
       metadata, the client must proceed as follows, before the metadata
       parsing is initiated:

       * set width and height of the ``OUTPUT`` format to desired coded resolution to
         let the decoder configure the ``CAPTURE`` format appropriately,

       * query the ``CAPTURE`` format using :c:func:`VIDIOC_G_FMT` and save it
         until this step.

       The format obtained in the query may be then used with
       :c:func:`VIDIOC_CREATE_BUFS` in this step to allocate the buffers.

```

11. `CAPTURE` 队列上调`VIDIOC_STREAMON` 以开始解码帧

## 解码

`Capture Setup` 序列成功完成后即进入此状态。在此状态下，客户端通过
`VIDIOC_QBUF` `VIDIOC_DQBUF` 按照标准语义向两个队列入队和出队缓冲区

`OUTPUT` 缓冲区的内容取决于当前活动的编码像素格式，并可能受编解码
特定的扩展控件影响，如每种格式的文档所述

两个队列独立运行，遵V4L2 缓冲区队列和内存到内存（memory-to-memory）设备的
标准行为。此外，由于所选编码格式的特性（例如帧重排序），`CAPTURE` 队列
出队的已解码帧的顺序，可能与`OUTPUT` 队列入队编码帧的顺序不同

客户端不得假`CAPTURE` `OUTPUT` 缓冲区之间，以及缓冲区可被出队的
任何特定时序之间存在任何直接关系。具体而言

- 入队`OUTPUT` 的缓冲区可能不在 `CAPTURE` 上产生任何缓冲区（例如，如果
  它不包含已编码数据，或者其中仅存在元数据语法结构）

- 入队`OUTPUT` 的缓冲区可能`CAPTURE` 上产生多于一个缓冲区（如果已编码
  数据包含多个帧，或者返回一个已解码帧使得解码器能够返回一个在解码顺序上位于其
  之前、但在显示顺序上位于其之后的帧）

- 入队`OUTPUT` 的缓冲区可能在解码过程的更晚阶段、和/或在处理了更
  `OUTPUT` 缓冲区之后，才在 `CAPTURE` 上产生缓冲区，或者乱序返回（例如
  如果使用了显示重排序），

- 即使没有`OUTPUT` 额外入队缓冲区，`CAPTURE` 队列上也可能会出现可
  缓冲区（例如drain `EOS` 期间），这是因为过去入队`OUTPUT` 
  某些缓冲区，其解码结果由于解码过程的特性要等到更晚的时刻才可用

   为了能够将已解码`CAPTURE` 缓冲区与产生它们`OUTPUT` 缓冲区对应起来，
   客户端可以在入队 `OUTPUT` 缓冲区时设置 `v4l2_buffer` 结构体的 `timestamp`
   字段。由解码`OUTPUT` 缓冲区所产生`CAPTURE` 缓冲区，在出队时
   `timestamp` 字段将被设为相同的值

   除了一`OUTPUT` 缓冲区产生一`CAPTURE` 缓冲区这种简单情况外，还定义
   以下情况

   - 一`OUTPUT` 缓冲区产生多`CAPTURE` 缓冲区：同一`OUTPUT` 时间
     将被复制到多`CAPTURE` 缓冲区

   - 多个 `OUTPUT` 缓冲区产生一`CAPTURE` 缓冲区：将复制最先入队的
     `OUTPUT` 缓冲区的时间戳

   - 解码顺序与显示顺序不同（`CAPTURE` 缓冲区相对于 `OUTPUT` 缓冲区是
     乱序的）：`CAPTURE` 时间戳将不会保留 `OUTPUT` 时间戳的顺序


   被流用作参考帧`CAPTURE` 缓冲区，其底层内存在出队后仍可能被硬件读取
   因此，客户端应避免在 `CAPTURE` 队列流式传输期间写入这块内存。否则可能导
   已解码帧损坏

   类似地，当使用的内存类型不是 `V4L2_MEMORY_MMAP` 时，客户端应确保
   `CAPTURE` 队列流式传输期间，每`CAPTURE` 缓冲区始终使用相同的底层内存
   入队。原因是 V4L2 缓冲区索引可被驱动用来识别帧。因此，如果参考帧的底层内
   以不同的缓冲ID 提交，驱动可能会误识别它，并在其仍被使用时将新帧解码到其中，
   从而导致后续帧损坏

在解码过程中，解码器可能会启动下列特殊序列之一。这些序列会导致解码器返回所
在序列开始之前处理的 `OUTPUT` 缓冲区所产生`CAPTURE` 缓冲区。最后一
缓冲区将带有 `V4L2_BUF_FLAG_LAST` 标志。为了确定需要遵循哪个序列，客户端必
检查是否存在待处理事件，并且：

- 如果待处理的`changes` 设为 `V4L2_EVENT_SRC_CH_RESOLUTION` 
  `V4L2_EVENT_SOURCE_CHANGE` 事件，则需要遵`Dynamic Resolution
  Change` 序列

- 如果待处理的`V4L2_EVENT_EOS` 事件，则需要遵`End of Stream` 序列

某些序列可以相互交错，需要按发生时的情形处理。每个序列的确切操作在相应章节中
有文档说明

如果发生解码错误，将根据解码器的能力，以不同的详细程度报告给客户端。具体而言

- 包含失败解码操作结果CAPTURE 缓冲区将被返回，并带V4L2_BUF_FLAG_ERROR 标志

- 如果解码器能够精确报告触发错误的 OUTPUT 缓冲区，则该缓冲区将被返回，并带
  V4L2_BUF_FLAG_ERROR 标志

如果发生不允许继续解码的致命失败，则对该解码器文件句柄的任何进一步操作都会返
-EIO 错误码。客户端可以关闭该文件句柄并打开一个新的，或者通过在两个队列上停止
流式传输、释放所有缓冲区并再次执`Initialization` 序列来重新初始化实例

## 定位（Seek

Seek `OUTPUT` 队列控制，因为它是已编码数据的来源。seek 不需要对 `CAPTURE`
队列执行任何特定操作，但它可能会受到解码器正常操作的影响

1. 通过 `VIDIOC_STREAMOFF` 停止 `OUTPUT` 队列以开seek 序列

   - **必填字段*

     `type`
         `OUTPUT` 适用`V4L2_BUF_TYPE_*` 枚举值

   - 解码器将丢弃所有待处理`OUTPUT` 缓冲区，它们必须被视为已返还给客户端
     （遵循标准语义）

2. 通过 `VIDIOC_STREAMON` 重启 `OUTPUT` 队列

   - **必填字段*

     `type`
         `OUTPUT` 适用`V4L2_BUF_TYPE_*` 枚举值

   - 调用返回后，解码器将开始接受新的源数据流缓冲区

3. 开始将包含 seek 之后编码数据的缓冲区入队`OUTPUT` 队列，直到找到合适的
   恢复点（resume point）

```

      There is no requirement to begin queuing coded data starting exactly
      from a resume point (e.g. SPS or a keyframe). Any queued ``OUTPUT``
      buffers will be processed and returned to the client until a suitable
      resume point is found.  While looking for a resume point, the decoder
      should not produce any decoded frames into ``CAPTURE`` buffers.

      Some hardware is known to mishandle seeks to a non-resume point. Such an
      operation may result in an unspecified number of corrupted decoded frames
      being made available on the ``CAPTURE`` queue. Drivers must ensure that
      no fatal decoding errors or crashes occur, and implement any necessary
      handling and workarounds for hardware issues related to seek operations.

   .. warning::

      In case of the H.264/HEVC codec, the client must take care not to seek
      over a change of SPS/PPS. Even though the target frame could be a
      keyframe, the stale SPS/PPS inside decoder state would lead to undefined
      results when decoding. Although the decoder must handle that case without
      a crash or a fatal decode error, the client must not expect a sensible
      decode output.

      If the hardware can detect such corrupted decoded frames, then
      corresponding buffers will be returned to the client with the
      V4L2_BUF_FLAG_ERROR set. See the `Decoding` section for further
      description of decode error reporting.

```

4. 找到恢复点后，解码器将开始返回包含已解码帧的 `CAPTURE` 缓冲区


   seek 可能导致 `Dynamic Resolution Change` 序列被启动，原因seek 目标
   解码参数seek 之前已解码的流部分不同。必须按解码器的正常操作来处理该序列


   未规`CAPTURE` 队列何时开始产生包seek 之后入队`OUTPUT` 缓冲
   解码数据的缓冲区，因为它`OUTPUT` 队列独立运行

   解码器可能会返回若干剩余`CAPTURE` 缓冲区，其中包含由在 seek 序列执行
   入队`OUTPUT` 缓冲区产生的已解码帧

   `VIDIOC_STREAMOFF` 操作会丢弃所有剩余的已入`OUTPUT` 缓冲区，这意味着
   并非所有在 seek 序列之前入队`OUTPUT` 缓冲区都一定有对应`CAPTURE`
   缓冲区产生。例如，给定 `OUTPUT` 队列上的操作序列

     QBUF(A), QBUF(B), STREAMOFF(), STREAMON(), QBUF(G), QBUF(H),

   `CAPTURE` 队列上出现以下任何结果都是允许的

     {A', B', G', H'}, {A', G', H'}, {G', H'}.

   要确seek 之后包含第一个已解码帧的 CAPTURE 缓冲区，客户端可以观察时间戳
   匹配 CAPTURE OUTPUT 缓冲区，或者使V4L2_DEC_CMD_STOP 
   V4L2_DEC_CMD_START 来排空（drain）解码器


   为了实现即时 seek，客户端也可以在 `CAPTURE` 队列上重启流式传输，以丢弃已
   解码但尚未出队的缓冲区

## 动态分辨率变化（Dynamic Resolution Change

在流中包含分辨率元数据的数据流，可能在解码过程中需要切换到不同的分辨率


   并非所有解码器都能检测分辨率变化。那些能够检测的解码器会在调
   `VIDIOC_ENUM_FMT` 时，为编码格式设`V4L2_FMT_FLAG_DYN_RESOLUTION`
   标志

当解码器检测到某一编码帧的以下一个或多个参数，与之前已确立（并反映在相应查询中）
的参数不同时，序列即开始：

- 编码分辨率（`OUTPUT` 的宽度和高度），

- 可见分辨率（选择矩形），

- 解码所需的最小缓冲区数量

- 数据流的位深（bit-depth）已改变

- 数据流的色彩空间（colorspace）已改变，但不需要重新分配缓冲区

一旦发生上述情况，解码器必须按如下方式继续

1. 在流中遇到分辨率变化后，解码器发`changes` 设为
    `V4L2_EVENT_SRC_CH_RESOLUTION` `V4L2_EVENT_SOURCE_CHANGE` 事件

```

       Any client query issued after the decoder queues the event will return
       values applying to the stream after the resolution change, including
       queue formats, selection rectangles and controls.

```

2. 然后，解码器将处理并解码分辨率变化点之前的所有剩余缓冲区

    - 变化之前的最后一个缓冲区必须带有 `V4L2_BUF_FLAG_LAST` 标志，类似于
      上面`Drain` 序列

```

       The last buffer may be empty (with :c:type:`v4l2_buffer` ``bytesused``
       = 0) and in that case it must be ignored by the client, as it does not
       contain a decoded frame.

    .. note::

       Any attempt to dequeue more ``CAPTURE`` buffers beyond the buffer marked
       with ``V4L2_BUF_FLAG_LAST`` will result in a -EPIPE error from
       :c:func:`VIDIOC_DQBUF`.

```

客户端必须按照下述方式继续该序列，以继续解码过程

1. 出队源变化事件

```

       A source change triggers an implicit decoder drain, similar to the
       explicit `Drain` sequence. The decoder is stopped after it completes.
       The decoding process must be resumed with either a pair of calls to
       :c:func:`VIDIOC_STREAMOFF` and :c:func:`VIDIOC_STREAMON` on the
       ``CAPTURE`` queue, or a call to :c:func:`VIDIOC_DECODER_CMD` with the
       ``V4L2_DEC_CMD_START`` command.

```

2. 继续 `Capture Setup` 序列


   在分辨率变化序列期间，`OUTPUT` 队列必须保持流式传输。在 `OUTPUT` 队列
   调用 `VIDIOC_STREAMOFF` 会中止该序列并启动一seek

   原则上，`OUTPUT` 队列`CAPTURE` 队列独立运行，在整个分辨率变化序列期
   也是如此

   为了获得最佳性能和简便性，客户端即使在处理此序列时，也应继续向 `OUTPUT`
   队列入队/出队缓冲区

## 排空（Drain

为了确保所有已入队`OUTPUT` 缓冲区都已被处理，且相关`CAPTURE` 缓冲
都已交给客户端，客户端必须遵循下drain 序列。drain 序列结束后，客户端已收到
在序列启动之前入队的所`OUTPUT` 缓冲区对应的所有已解码帧

1. 通过发出 `VIDIOC_DECODER_CMD` 开drain

   - **必填字段*

     `cmd`
         设为 `V4L2_DEC_CMD_STOP`

     `flags`
         设为 0

     `pts`
         设为 0

```

      The sequence can be only initiated if both ``OUTPUT`` and ``CAPTURE``
      queues are streaming. For compatibility reasons, the call to
      :c:func:`VIDIOC_DECODER_CMD` will not fail even if any of the queues is
      not streaming, but at the same time it will not initiate the `Drain`
      sequence and so the steps described below would not be applicable.

```

2. 客户端在发出 `VIDIOC_DECODER_CMD` 之前入队的任`OUTPUT` 缓冲区，
   像正常情况一样被处理和解码。客户端必须继续独立处理两个队列，类似于正常
   解码操作。这包括

   - 在处理这些缓冲区所触发的所有操作（例如 `Dynamic Resolution Change`
     序列）之后，再继drain 序列

   - 入队和出`CAPTURE` 缓冲区，直到出一个带`V4L2_BUF_FLAG_LAST`
     标志的缓冲区

```

        The last buffer may be empty (with :c:type:`v4l2_buffer`
        ``bytesused`` = 0) and in that case it must be ignored by the client,
        as it does not contain a decoded frame.

     .. note::

        Any attempt to dequeue more ``CAPTURE`` buffers beyond the buffer
        marked with ``V4L2_BUF_FLAG_LAST`` will result in a -EPIPE error from
        :c:func:`VIDIOC_DQBUF`.

   * dequeuing processed ``OUTPUT`` buffers, until all the buffers queued
     before the ``V4L2_DEC_CMD_STOP`` command are dequeued,

   * dequeuing the ``V4L2_EVENT_EOS`` event, if the client subscribed to it.

   .. note::

      For backwards compatibility, the decoder will signal a ``V4L2_EVENT_EOS``
      event when the last frame has been decoded and all frames are ready to be
      dequeued. It is a deprecated behavior and the client must not rely on it.
      The ``V4L2_BUF_FLAG_LAST`` buffer flag should be used instead.

```

3. 一旦在 `V4L2_DEC_CMD_STOP` 调用之前入队的所`OUTPUT` 缓冲区都已出队，
   且最后一`CAPTURE` 缓冲区也已出队，解码器即停止，它将接受但不会处理任何
   新入队的 `OUTPUT` 缓冲区，直到客户端发出以下任一操作

   - `V4L2_DEC_CMD_START` - 解码器不会被重置，并将带着 drain 之前的所有状
     正常恢复操作

   - `CAPTURE` 队列上的一`VIDIOC_STREAMOFF` `VIDIOC_STREAMON` -
     解码器将正常恢复操作，但队列中任何仍存在`CAPTURE` 缓冲区将被返回给
     客户端，

   - `OUTPUT` 队列上的一`VIDIOC_STREAMOFF` `VIDIOC_STREAMON` - 任何
     待处理的源缓冲区将被返回给客户端，并且会触发 `Seek` 序列


   一drain 序列启动，客户端就需要按上述步骤将其驱动至完成，除非它通过
   `OUTPUT` `CAPTURE` 队列上发`VIDIOC_STREAMOFF` 中止该过程。在 drain
   序列进行期间，客户端不允许再次发`V4L2_DEC_CMD_START` 
   `V4L2_DEC_CMD_STOP`，如果尝试，它们将以 -EBUSY 错误码失败

   虽然并非强制，但解码器命令的可用性可以通过 `VIDIOC_TRY_DECODER_CMD` 查询

## 流结束（End of Stream

如果解码器在流中遇到流结束（end of stream）标记，解码器将启动 `Drain` 序列
客户端必须按上述方式处理该序列，但跳过初始的 `VIDIOC_DECODER_CMD`

## 提交点（Commit Points

设置格式和分配缓冲区会触发解码器行为的变化

1. `OUTPUT` 队列上设置格式，可能会改`CAPTURE` 队列上受支持/通告
   格式集合。特别地，这也意味着 `CAPTURE` 格式可能会被重置，客户端不得依赖
   先前设置的格式被保留

2. `CAPTURE` 队列上枚举格式，总是只返回当`OUTPUT` 格式所支持的格式

3. `CAPTURE` 队列上设置格式，不会改变 `OUTPUT` 队列上可用格式列表。尝
   设置一个对当前所`OUTPUT` 格式不支持的 `CAPTURE` 格式，会导致解码器将
   所请求`CAPTURE` 格式调整为受支持的格式

4. `OUTPUT` 队列上枚举格式，总是返回受支持编码格式的完整集合，与当前
   `CAPTURE` 格式无关

5. 只要 `OUTPUT` `CAPTURE` 队列上分配了缓冲区，客户端就不得更改 `OUTPUT`
   队列上的格式。对于任何此类格式更改尝试，驱动都会返回 -EBUSY 错误码

总而言之，设置格式和分配必须始终从 `OUTPUT` 队列开始，并且 `OUTPUT` 队列
掌管 `CAPTURE` 队列受支持格式集合的主控方
