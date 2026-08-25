######## 内存到内存有状态视频编码器接口


有状态视频编码器按显示顺序接收原始视频帧，并将其编码为字节流。它生成完整的字节流片段，包括所有元数据、头部等。生成的字节流无需客户端做任何进一步的后处理

在驱动中执行软件流处理、头部生成等操作以支持本接口，是强烈不建议的。若确实需要此类操作，强烈建议使用无状态视频编码器接口（开发中）

## 本文档使用的约定与记


1. 除非本文档另有说明，V4L2 API 的一般规则均适用

2. 词语 "must"may"should" 等的含义遵循 `RFC
   2119 <https://tools.ietf.org/html/rfc2119>`_.

3. 所有未标注为“optional”的步骤都是必需的

4. 除非另有说明，`VIDIOC_G_EXT_CTRLS` `VIDIOC_S_EXT_CTRLS` 可分别与 `VIDIOC_G_CTRL` `VIDIOC_S_CTRL` 互换使用

5. 除非另有说明，根据编码器能力并遵V4L2 通用准则，单平面 API（见 planar-apis）及适用的结构体可与多平API 互换使用

6. i = [a..b]：从 a b（含端点）的整数序列，即 i = [0..2] 表示 i = 0, 1, 2

7. 给定一`OUTPUT` 缓冲A，则 A' 表示 `CAPTURE` 队列上的一个缓冲区，其中包含由处理缓冲A 所产生的数据

## 术语


参见 decoder-glossary

## 状态机


   :alt: DOT digraph of encoder state machine
   :caption: Encoder State Machine

   digraph encoder_state_machine {
       node [shape = doublecircle, label="Encoding"] Encoding;

       node [shape = circle, label="Initialization"] Initialization;
       node [shape = circle, label="Stopped"] Stopped;
       node [shape = circle, label="Drain"] Drain;
       node [shape = circle, label="Reset"] Reset;

       node [shape = point]; qi
       qi -> Initialization [ label = "open()" ];

       Initialization -> Encoding [ label = "Both queues streaming" ];

       Encoding -> Drain [ label = "V4L2_ENC_CMD_STOP" ];
       Encoding -> Reset [ label = "VIDIOC_STREAMOFF(CAPTURE)" ];
       Encoding -> Stopped [ label = "VIDIOC_STREAMOFF(OUTPUT)" ];
       Encoding -> Encoding;

       Drain -> Stopped [ label = "All CAPTURE\nbuffers dequeued\nor\nVIDIOC_STREAMOFF(OUTPUT)" ];
       Drain -> Reset [ label = "VIDIOC_STREAMOFF(CAPTURE)" ];

       Reset -> Encoding [ label = "VIDIOC_STREAMON(CAPTURE)" ];
       Reset -> Initialization [ label = "VIDIOC_REQBUFS(OUTPUT, 0)" ];

       Stopped -> Encoding [ label = "V4L2_ENC_CMD_START\nor\nVIDIOC_STREAMON(OUTPUT)" ];
       Stopped -> Reset [ label = "VIDIOC_STREAMOFF(CAPTURE)" ];
   }

## 查询能力


1. 为枚举编码器支持的一组编码格式，客户端可`CAPTURE` 上调`VIDIOC_ENUM_FMT`

   - 将返回完整的受支持格式集合，无论 `OUTPUT` 上设置的格式为何

2. 为枚举受支持的一组原始格式，客户端可`OUTPUT` 上调`VIDIOC_ENUM_FMT`

   - 仅返回当前在 `CAPTURE` 上处于活动状态的格式所支持的那些格式

   - 要枚举某个给定编码格式所支持的原始格式，客户端必须先`CAPTURE` 上设置该编码格式，再`OUTPUT` 上枚举格式

3. 客户端可使用 `VIDIOC_ENUM_FRAMESIZES` 来检测某给定格式所支持的分辨率，将期望的像素格式通过 `v4l2_frmsizeenum` `pixel_format` 传入

   - `VIDIOC_ENUM_FRAMESIZES` 针对编码像素格式返回的值，将包含编码器对该给定编码像素格式支持的所有可能的编码分辨率

   - `VIDIOC_ENUM_FRAMESIZES` 针对原始像素格式返回的值，将包含编码器对该给定原始像素格式、以及当前在 `CAPTURE` 上设置的编码格式所支持的所有可能的帧缓冲区分辨率

4. 客户端可使用 `VIDIOC_ENUM_FRAMEINTERVALS` 来检测某给定格式与分辨率所支持的帧间隔，将期望的像素格式通过 `v4l2_frmivalenum` `pixel_format` 传入，分辨率通过 `v4l2_frmivalenum` `width` `v4l2_frmivalenum` `height` 传入

   - `VIDIOC_ENUM_FRAMEINTERVALS` 针对编码像素格式与编码分辨率返回的值，将包含编码器对该给定编码像素格式与分辨率所支持的所有可能的帧间隔

   - `VIDIOC_ENUM_FRAMEINTERVALS` 针对原始像素格式与分辨率返回的值，将包含编码器对该给定原始像素格式与分辨率、以及当前在 `CAPTURE` 上设置的编码格式、编码分辨率和编码帧间隔所支持的所有可能的帧间隔

   - `VIDIOC_ENUM_FRAMEINTERVALS` 的支持是可选的。若未实现，则除编解码器本身的限制外，没有其他特殊限制

5. 对于当前`CAPTURE` 上设置的编码格式，若适用，其所支持的档次（profile）与级别（level）可通过各自对应的控件经 `VIDIOC_QUERYCTRL` 查询

6. 任何其他编码器能力均可通过查询其各自对应的控件来发现

## 初始


1. 通过 `VIDIOC_S_FMT` `CAPTURE` 队列上设置编码格式

   - **必需字段*

     `type`
         一个适用`CAPTURE` `V4L2_BUF_TYPE_*` 枚举

     `pixelformat`
         要生成的编码格式

     `sizeimage`
         `CAPTURE` 缓冲区的期望大小；编码器可能会对其进行调整以匹配硬件要求

     `width`, `height`
         忽略（只读）

     other fields
         遵循标准语义

   - **返回字段*

     `sizeimage`
         经调整的 `CAPTURE` 缓冲区大小

     `width`, `height`
         由编码器根据当前状态（例如 `OUTPUT` 格式、选择矩形等）选定的编码尺寸（只读）

```

      Changing the ``CAPTURE`` format may change the currently set ``OUTPUT``
      format. How the new ``OUTPUT`` format is determined is up to the encoder
      and the client must ensure it matches its needs afterwards.

```
2. **可选* 通过 `VIDIOC_ENUM_FMT` 枚举所选编码格式支持的 `OUTPUT` 格式（源的原始格式）

   - **必需字段*

     `type`
         一个适用`OUTPUT` `V4L2_BUF_TYPE_*` 枚举

     other fields
         遵循标准语义

   - **返回字段*

     `pixelformat`
         当前`CAPTURE` 队列上所选编码格式所支持的原始格式

     other fields
         遵循标准语义

3. 通过 `VIDIOC_S_FMT` `OUTPUT` 队列上设置原始源格式

   - **必需字段*

     `type`
         一个适用`OUTPUT` `V4L2_BUF_TYPE_*` 枚举

     `pixelformat`
         源的原始格式

     `width`, `height`
         源分辨率

     other fields
         遵循标准语义

   - **返回字段*

     `width`, `height`
         可能会被调整，以匹配当前所选格式（`VIDIOC_ENUM_FRAMESIZES` 所报告）要求的编码器最小值、最大值和对齐要求

     other fields
         遵循标准语义

   - 设置 `OUTPUT` 格式会根据新分辨率将选择矩形重置为默认值，如下一步所述

4. 通过 `VIDIOC_S_PARM` `OUTPUT` 队列上设置原始帧间隔。这同时会将 `CAPTURE` 队列上的编码帧间隔设为相同的值

   - **必需字段*

     `type`
	 一个适用`OUTPUT` `V4L2_BUF_TYPE_*` 枚举

     `parm.output`
	 `parm.output.timeperframe` 外所有字段设0

     `parm.output.timeperframe`
	 期望的帧间隔；编码器可能会对其进行调整以匹配硬件要求

   - **返回字段*

     `parm.output.timeperframe`
	 经调整的帧间隔

```

      Changing the ``OUTPUT`` frame interval *also* sets the framerate that
      the encoder uses to encode the video. So setting the frame interval
      to 1/24 (or 24 frames per second) will produce a coded video stream
      that can be played back at that speed. The frame interval for the
      ``OUTPUT`` queue is just a hint, the application may provide raw
      frames at a different rate. It can be used by the driver to help
      schedule multiple encoders running in parallel.

      In the next step the ``CAPTURE`` frame interval can optionally be
      changed to a different value. This is useful for off-line encoding
      were the coded frame interval can be different from the rate at
      which raw frames are supplied.

   .. important::

      ``timeperframe`` deals with *frames*, not fields. So for interlaced
      formats this is the time per two fields, since a frame consists of
      a top and a bottom field.

   .. note::

      It is due to historical reasons that changing the ``OUTPUT`` frame
      interval also changes the coded frame interval on the ``CAPTURE``
      queue. Ideally these would be independent settings, but that would
      break the existing API.

```
5. **可* 通过 `VIDIOC_S_PARM` `CAPTURE` 队列上设置编码帧间隔。仅当编码帧间隔与原始帧间隔不同时才需要此步骤，离线编码通常就属于这种情况。该特性的支持V4L2_FMT_FLAG_ENC_CAP_FRAME_INTERVAL <fmtdesc-flags> 格式标志来标示

   - **必需字段*

     `type`
	 一个适用`CAPTURE` `V4L2_BUF_TYPE_*` 枚举

     `parm.capture`
	 `parm.capture.timeperframe` 外所有字段设0

     `parm.capture.timeperframe`
	 期望的编码帧间隔；编码器可能会对其进行调整以匹配硬件要求

   - **返回字段*

     `parm.capture.timeperframe`
	 经调整的帧间隔

```

      Changing the ``CAPTURE`` frame interval sets the framerate for the
      coded video. It does *not* set the rate at which buffers arrive on the
      ``CAPTURE`` queue, that depends on how fast the encoder is and how
      fast raw frames are queued on the ``OUTPUT`` queue.

   .. important::

      ``timeperframe`` deals with *frames*, not fields. So for interlaced
      formats this is the time per two fields, since a frame consists of
      a top and a bottom field.

   .. note::

      Not all drivers support this functionality, in that case just set
      the desired coded frame interval for the ``OUTPUT`` queue.

      However, drivers that can schedule multiple encoders based on the
      ``OUTPUT`` frame interval must support this optional feature.

```
6. **可选* 若希望流元数据的可见分辨率不同于完整OUTPUT 分辨率，可通过 `VIDIOC_S_SELECTION` `OUTPUT` 队列上设置可见分辨率

   - **必需字段*

     `type`
         一个适用`OUTPUT` `V4L2_BUF_TYPE_*` 枚举

     `target`
         设为 `V4L2_SEL_TGT_CROP`

     `r.left`, `r.top`, `r.width`, `r.height`
         可见矩形；它必须落在 `V4L2_SEL_TGT_CROP_BOUNDS` 矩形之内，并可能被调整以符合编解码器和硬件约束

   - **返回字段*

     `r.left`, `r.top`, `r.width`, `r.height`
         经编码器调整的可见矩形

   - `OUTPUT` 上支持以下选择目标

     `V4L2_SEL_TGT_CROP_BOUNDS`
         等于完整的源帧，与活动的 `OUTPUT` 格式一致

     `V4L2_SEL_TGT_CROP_DEFAULT`
         等于 `V4L2_SEL_TGT_CROP_BOUNDS`

     `V4L2_SEL_TGT_CROP`
         源缓冲区中将被编码进 `CAPTURE` 流的矩形；默认为 `V4L2_SEL_TGT_CROP_DEFAULT`

```

            A common use case for this selection target is encoding a source
            video with a resolution that is not a multiple of a macroblock,
            e.g.  the common 1920x1080 resolution may require the source
            buffers to be aligned to 1920x1088 for codecs with 16x16 macroblock
            size. To avoid encoding the padding, the client needs to explicitly
            configure this selection target to 1920x1080.

   .. warning::

      The encoder may adjust the crop/compose rectangles to the nearest
      supported ones to meet codec and hardware requirements. The client needs
      to check the adjusted rectangle returned by :c:func:`VIDIOC_S_SELECTION`.

```
7. 通过 `VIDIOC_REQBUFS` `OUTPUT` `CAPTURE` 分配缓冲区。可以以任意顺序执行

   - **必需字段*

     `count`
         请求分配的缓冲区数量；必须大于零

     `type`
         一个适用`OUTPUT` `CAPTURE` `V4L2_BUF_TYPE_*` 枚举

     other fields
         遵循标准语义

   - **返回字段*

     `count`
         实际分配的缓冲区数量

```

      The actual number of allocated buffers may differ from the ``count``
      given. The client must check the updated value of ``count`` after the
      call returns.

   .. note::

      To allocate more than the minimum number of OUTPUT buffers (for pipeline
      depth), the client may query the ``V4L2_CID_MIN_BUFFERS_FOR_OUTPUT``
      control to get the minimum number of buffers required, and pass the
      obtained value plus the number of additional buffers needed in the
      ``count`` field to :c:func:`VIDIOC_REQBUFS`.

   Alternatively, :c:func:`VIDIOC_CREATE_BUFS` can be used to have more
   control over buffer allocation.

   * **Required fields:**

     ``count``
         requested number of buffers to allocate; greater than zero.

     ``type``
         a ``V4L2_BUF_TYPE_*`` enum appropriate for ``OUTPUT``.

     other fields
         follow standard semantics.

   * **Returned fields:**

     ``count``
         adjusted to the number of allocated buffers.

```
8. 通过 `VIDIOC_STREAMON` `OUTPUT` `CAPTURE` 两个队列上开始数据流。可以以任意顺序执行。当两个队列都开始数据流时，实际的编码过程才开始


   若客户端在编码过程中停止 `CAPTURE` 队列，随后又重新启动它，编码器将开始生成一条与停止前所生成流相互独立的流。具体的约束取决于编码格式，但可能包括以下后果：

   - 重启后生成的编码帧不得引用停止前生成的任何帧，例H.264/HEVC 中不允许长期参考，

   - 任何必须包含在独立流中的头部都必须重新生成，例如 H.264/HEVC SPS PPS

## 编码


`Initialization` 序列成功完成后进入此状态。在此状态下，客户端通过 `VIDIOC_QBUF` `VIDIOC_DQBUF` 向两个队列入队和出队缓冲区，遵循标准语义

编码`CAPTURE` 缓冲区的内容取决于活动的编码像素格式，并可能受各格式文档中所述的编解码器特定扩展控件影响

两个队列独立运行，遵V4L2 缓冲区队列与内存到内存设备的标准行为。此外，由于所选编码格式的特性（例如帧重排序），`CAPTURE` 队列出队的编码帧顺序，可能与`OUTPUT` 队列入队原始帧的顺序不同

客户端不得假`CAPTURE` `OUTPUT` 缓冲区之间存在任何直接关系，也不得假定缓冲区变为可出队的具体时机。具体而言

- 入队`OUTPUT` 的缓冲区可能`CAPTURE` 上产生多于一个缓冲区（例如，若返回一个编码帧使编码器得以返回一个在显示顺序中位于其之前、但在解码顺序中位于其之后的帧；当然也可能有其他原因），

- 入队`OUTPUT` 的缓冲区可能在编码过程的更晚阶段、和/或在处理了更`OUTPUT` 缓冲区之后，才在 `CAPTURE` 上产生一个缓冲区，或者乱序返回（例如在使用显示重排序时）

- 即使没有额外缓冲区入队到 `OUTPUT`，`CAPTURE` 队列上也可能会有缓冲区变为可用（例如drain `EOS` 期间），这是因为过去入队`OUTPUT` 缓冲区，其编码结果由于编码过程的特性而要到更晚的时刻才可用，

- 入队`OUTPUT` 的缓冲区在编码进相应`CAPTURE` 缓冲区后，可能不会立即变为可出队，例如当编码器需要将该帧用作编码后续帧的参考时


   为使编码后的 `CAPTURE` 缓冲区能与其来源 `OUTPUT` 缓冲区相匹配，客户端可在入队一`OUTPUT` 缓冲区时，设`v4l2_buffer` 结构体的 `timestamp` 字段。由编码`OUTPUT` 缓冲区所产生`CAPTURE` 缓冲区，在出队时`timestamp` 字段会被设为相同的值

   除一`OUTPUT` 缓冲区产生一`CAPTURE` 缓冲区的简单情况外，还定义了以下情况：

   - 一`OUTPUT` 缓冲区生成多`CAPTURE` 缓冲区：相同`OUTPUT` 时间戳会被复制到多个 `CAPTURE` 缓冲区，

   - 编码顺序与呈现顺序不同（`CAPTURE` 缓冲区相对于 `OUTPUT` 缓冲区是乱序的）：`CAPTURE` 时间戳不会保`OUTPUT` 时间戳的顺序


   为让客户端区分帧类型（关键帧、中间帧；确切的类型列表取决于编码格式），`CAPTURE` 缓冲区在出队时，`v4l2_buffer` 结构体中会设置相应的标志位。确切的标志列表及其含义，请参阅 `v4l2_buffer` 以及各编码像素格式的文档

若发生编码错误，将依据编码器的能力，以相应的详细程度报告给客户端。具体而言

- 包含失败编码操作结果`CAPTURE` 缓冲区（若有）将以设`V4L2_BUF_FLAG_ERROR` 标志的状态返回，

- 若编码器能够精确报告触发错误`OUTPUT` 缓冲区，则此类缓冲区将以设置 `V4L2_BUF_FLAG_ERROR` 标志的状态返回


   `CAPTURE` 缓冲区过小，则它仅以设置 `V4L2_BUF_FLAG_ERROR` 标志的状态返回。还需要做更多工作来检测“缓冲区过小”这一错误原因，并提供释放过小缓冲区的支持

如果发生不允许编码继续的致命失败，对该编码器文件句柄的任何进一步操作都将返-EIO 错误码。客户端可以关闭该文件句柄并打开一个新的，或者通过停止两个队列的数据流、释放所有缓冲区并重新执行初始化序列来重新初始化该实例

## 编码参数更改


客户端可随时使用 `VIDIOC_S_CTRL` 来更改编码器参数。参数的可用性因编码器而异，客户端必须查询编码器以确定可用控件集合

能否在编码过程中更改每个参数因编码器而异，遵V4L2 控件接口的标准语义。客户端可以尝试在编码过程中设置控件，若操作-EBUSY 错误码失败，则需要停`CAPTURE` 队列才允许更改配置。为此，它可以遵`Drain` 序列，以避免丢失已入已编码的帧

参数更新的时机因编码器而异，遵V4L2 控件接口的标准语义。若客户端需要在特定帧精确应用参数，应考虑使用 Request API（media-request-api），前提是编码器支持

## 排空（Drain


为确保所有已入队`OUTPUT` 缓冲区都已被处理，且相关`CAPTURE` 缓冲区已交付给客户端，客户端必须遵循下述 drain 序列。drain 序列结束后，客户端已收到在该序列启动前入队的所`OUTPUT` 缓冲区的全部编码帧

1. 通过发出 `VIDIOC_ENCODER_CMD` 开drain 序列

   - **必需字段*

     `cmd`
         设为 `V4L2_ENC_CMD_STOP`

     `flags`
         设为 0

     `pts`
         设为 0

```

      The sequence can be only initiated if both ``OUTPUT`` and ``CAPTURE``
      queues are streaming. For compatibility reasons, the call to
      :c:func:`VIDIOC_ENCODER_CMD` will not fail even if any of the queues is
      not streaming, but at the same time it will not initiate the `Drain`
      sequence and so the steps described below would not be applicable.

```
2. 在发`VIDIOC_ENCODER_CMD` 之前客户端入队的任何 `OUTPUT` 缓冲区，都将照常处理和编码。客户端必须继续独立处理两个队列，类似于正常的编码操作。这包括

   - 入队和出`CAPTURE` 缓冲区，直到出队一个带`V4L2_BUF_FLAG_LAST` 标志的缓冲区

```

        The last buffer may be empty (with :c:type:`v4l2_buffer`
        ``bytesused`` = 0) and in that case it must be ignored by the client,
        as it does not contain an encoded frame.

     .. note::

        Any attempt to dequeue more ``CAPTURE`` buffers beyond the buffer
        marked with ``V4L2_BUF_FLAG_LAST`` will result in a -EPIPE error from
        :c:func:`VIDIOC_DQBUF`.

   * dequeuing processed ``OUTPUT`` buffers, until all the buffers queued
     before the ``V4L2_ENC_CMD_STOP`` command are dequeued,

   * dequeuing the ``V4L2_EVENT_EOS`` event, if the client subscribes to it.

   .. note::

      For backwards compatibility, the encoder will signal a ``V4L2_EVENT_EOS``
      event when the last frame has been encoded and all frames are ready to be
      dequeued. It is deprecated behavior and the client must not rely on it.
      The ``V4L2_BUF_FLAG_LAST`` buffer flag should be used instead.

```
3. 一旦在 `V4L2_ENC_CMD_STOP` 调用之前入队的所`OUTPUT` 缓冲区都已出队，且最后一`CAPTURE` 缓冲区也已出队，编码器即停止；此后它将接受、但不再处理任何新入队的 `OUTPUT` 缓冲区，直到客户端发出以下任一操作

   - `V4L2_ENC_CMD_START` —编码器不会被重置，将带着 drain 之前的所有状态恢复正常操作，

   - `CAPTURE` 队列上的一`VIDIOC_STREAMOFF` `VIDIOC_STREAMON` —编码器将被重置（`Reset` 序列），然后恢复编码

   - `OUTPUT` 队列上的一`VIDIOC_STREAMOFF` `VIDIOC_STREAMON` —编码器将恢复正常操作，但`V4L2_ENC_CMD_STOP` `VIDIOC_STREAMOFF` 之间入队`OUTPUT` 队列的任何源帧都将被丢弃


   一旦启动了 drain 序列，客户端就需要按上述步骤将其推进至完成，除非它通过在任`OUTPUT` `CAPTURE` 队列上发`VIDIOC_STREAMOFF` 来中止该过程。在 drain 序列进行期间，客户端不得再次发出 `V4L2_ENC_CMD_START` `V4L2_ENC_CMD_STOP`，否则尝试时将失败并返回 -EBUSY 错误码

   作为参考，下面描述了各种边界情况的处理

   - 若在发出 `V4L2_ENC_CMD_STOP` 命令`OUTPUT` 队列中没有缓冲区，则 drain 序列立即完成，编码器返回一个带`V4L2_BUF_FLAG_LAST` 标志的空 `CAPTURE` 缓冲区

   - 若在 drain 序列完成`CAPTURE` 队列中没有缓冲区，则下次客户端入队一`CAPTURE` 缓冲区时，它会立即作为一个带`V4L2_BUF_FLAG_LAST` 标志的空缓冲区返回

   - 若在 drain 序列进行期间`CAPTURE` 队列上调`VIDIOC_STREAMOFF`，则 drain 序列被取消，所`CAPTURE` 缓冲区被隐式返回给客户端

   - 若在 drain 序列进行期间`OUTPUT` 队列上调`VIDIOC_STREAMOFF`，则 drain 序列立即完成，下一`CAPTURE` 缓冲区将作为带有 `V4L2_BUF_FLAG_LAST` 标志的空缓冲区返回

   尽管不是强制要求，但可以使用 `VIDIOC_TRY_ENCODER_CMD` 查询编码器命令的可用性

## 重置


客户端可能希望请求编码器重新初始化编码，使得后续的流数据独立于之前生成的流数据。根据编码格式的不同，这可能意味着

- 重启后生成的编码帧不得引用停止前生成的任何帧，例H.264/HEVC 中不允许长期参考，

- 任何必须包含在独立流中的头部都必须重新生成，例如 H.264/HEVC SPS PPS

这可以通过执行重置序列来实现

1. 执行 `Drain` 序列，以确保所有在途编码都已完成且相应缓冲区都已出队

2. 通过 `VIDIOC_STREAMOFF` 停止 `CAPTURE` 队列上的数据流。这将把所有当前已入队`CAPTURE` 缓冲区返回给客户端，且不含有效帧数据

3. 通过 `VIDIOC_STREAMON` `CAPTURE` 队列上启动数据流，并继续进行常规编码序列。从此刻起，生成`CAPTURE` 缓冲区中的编码帧将包含一条独立流，无需重置序列之前编码的帧即可解码；该独立流始于在发出 `Drain` 序列`V4L2_ENC_CMD_STOP` 之后入队的第一`OUTPUT` 缓冲区

该序列也可用于为那些无法在运行中更改参数的编码器更改编码参数

## 鎻愪氦鐐。


设置格式和分配缓冲区会触发编码器行为的改变

1. `CAPTURE` 队列上设置格式，可能会改`OUTPUT` 队列上支通告的格式集合。特别地，这也意味着 `OUTPUT` 格式可能会被重置，客户端不得依赖之前设置的格式被保留

2. `OUTPUT` 队列上枚举格式，总是只返回当`CAPTURE` 格式所支持的格式

3. `OUTPUT` 队列上设置格式，不会改变 `CAPTURE` 队列上可用格式列表。若尝试设置当前所`CAPTURE` 格式不支持的 `OUTPUT` 格式，编码器会将所请求`OUTPUT` 格式调整为受支持的某个格式

4. `CAPTURE` 队列上枚举格式，总是返回受支持编码格式的完整集合，与当前 `OUTPUT` 格式无关

5. 当缓冲区已在 `OUTPUT` `CAPTURE` 任一队列上分配时，客户端不得更改 `CAPTURE` 队列上的格式。对于任何此类格式更改尝试，驱动将返-EBUSY 错误码

总结而言，设置格式与分配缓冲区必须始终从 `CAPTURE` 队列开始，`CAPTURE` 队列是主控方，它决定`OUTPUT` 队列所支持的格式集合
