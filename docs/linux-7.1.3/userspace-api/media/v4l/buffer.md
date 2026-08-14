

######## 缓冲区


缓冲区包含由应用程序与驱动通过某一种流式 I/O（Streaming I/O）方法交换的数据。在多平面（multi-planar）API 中，数据保存在平面（planes）中，而缓冲区结构体则充当这些平面的容器。只交换指向缓冲区（平面）的指针，数据本身不会被复制。这些指针连同时间戳或场奇偶性等元信息一起，被存储在结构体 `v4l2_buffer` 中，该结构体是 VIDIOC_QUERYBUF、VIDIOC_QBUF <VIDIOC_QBUF> 以及 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 的参数。在多平面 API 中，`v4l2_buffer` 结构体里一些特定于平面的成员（如每个平面的指针和大小）改为存储在结构体 `v4l2_plane` 中。在这种情况下，`v4l2_buffer` 结构体包含一个平面结构体数组。

出队的视频缓冲区带有时间戳。由驱动决定在帧的哪一部分、使用哪个时钟来采集时间戳。请参阅 buffer-flags 中掩码 `V4L2_BUF_FLAG_TIMESTAMP_MASK` 与 `V4L2_BUF_FLAG_TSTAMP_SRC_MASK` 里的标志位。在整个视频流期间，这些标志位对所有缓冲区始终有效且保持不变。不过，作为 VIDIOC_S_INPUT <VIDIOC_G_INPUT> 或 VIDIOC_S_OUTPUT <VIDIOC_G_OUTPUT> 的副作用，这些标志位可能会发生变化。规则的一个例外是 `V4L2_BUF_FLAG_TIMESTAMP_COPY` 时间戳类型（例如用于 mem-to-mem 设备）：时间戳源标志位会从 OUTPUT 视频缓冲区复制到 CAPTURE 视频缓冲区。

## 格式、控件与缓冲区之间的交互


V4L2 暴露了一些会影响缓冲区大小或数据在缓冲区中布局方式的参数。这些参数既通过格式也通过控件来暴露。此类控件的一个例子是 `V4L2_CID_ROTATE` 控件，它会修改像素在缓冲区中存储的方向，并在所选格式在行尾包含填充时同时修改缓冲区大小。

解释缓冲区内容所需的一组信息（例如像素格式、行步长、平铺方向或旋转）在本节其余部分统称为缓冲区布局（buffer layout）。

可以修改缓冲区布局的控件应当设置 `V4L2_CTRL_FLAG_MODIFY_LAYOUT` 标志。

修改会影响缓冲区大小或布局的格式或控件要求先停止流。任何在流处于活动状态时尝试做此类修改的行为，都应使设置格式或控件的 ioctl 返回 `EBUSY` 错误码。在这种情况下，当流处于活动状态时驱动针对此类控件调用 `VIDIOC_QUERYCTRL` 或 `VIDIOC_QUERY_EXT_CTRL` 还应当设置 `V4L2_CTRL_FLAG_GRABBED` 标志。


   `VIDIOC_S_SELECTION` ioctl 可能（取决于硬件，例如设备不包含缩放器时）在修改选择矩形的同时修改格式。类似地，`VIDIOC_S_INPUT`、`VIDIOC_S_OUTPUT`、`VIDIOC_S_STD` 和 `VIDIOC_S_DV_TIMINGS` ioctl 也可以修改格式和选择矩形。当这些 ioctl 导致缓冲区大小或布局发生变化时，驱动应当按照本节所描述的各种情况中处理 `VIDIOC_S_FMT` ioctl 的方式来应对该状况。

只影响缓冲区布局的控件可以在流停止后的任意时刻修改。由于它们不影响缓冲区大小，因此不需要任何特殊的处理来将这些控件与缓冲区分配同步，并且一旦流停止，`V4L2_CTRL_FLAG_GRABBED` 标志即被清除。

影响缓冲区大小的格式和控件会与缓冲区分配相互作用。最简单的处理方式是驱动始终要求重新分配缓冲区，以便更改这些格式或控件。在这种情况下，要进行此类更改，用户空间应用程序应先在流运行时用 `VIDIOC_STREAMOFF` ioctl 停止视频流，并在缓冲区已分配时用 `VIDIOC_REQBUFS` ioctl 释放所有缓冲区。释放所有缓冲区后，控件的 `V4L2_CTRL_FLAG_GRABBED` 标志被清除。然后可以修改格式或控件，随后应重新分配缓冲区并重新启动流。一个典型的 ioctl 序列为

 #. VIDIOC_STREAMOFF
 #. VIDIOC_REQBUFS(0)
 #. VIDIOC_S_EXT_CTRLS
 #. VIDIOC_S_FMT
 #. VIDIOC_REQBUFS(n)
 #. VIDIOC_QBUF
 #. VIDIOC_STREAMON

第二次 `VIDIOC_REQBUFS` 调用会将新的格式和控件值考虑在内，以计算要分配的缓冲区大小。如有需要，应用程序也可以通过调用 `VIDIOC_G_FMT` ioctl 来获取该大小。


   该 API 并未强制规定上述控件（3.）与格式（4.）更改的顺序。格式和控件可以按不同顺序设置，甚至可以交错设置，具体取决于设备和用例。例如某些控件对于不同的像素格式可能表现不同，在这种情况下可能需要先设置格式。

当需要重新分配时，任何在缓冲区已分配的情况下尝试修改影响缓冲区大小的格式或控件的行为，都应使设置格式或控件的 ioctl 返回 `EBUSY` 错误。任何尝试将对于当前格式或控件而言太小的缓冲区入队的行为，都应使 `VIDIOC_QBUF` ioctl 返回 `EINVAL` 错误。

缓冲区重新分配是一项开销较大的操作。为避免该开销，驱动可以（并且被鼓励）允许在缓冲区已分配的情况下更改影响缓冲区大小的格式或控件。在这种情况下，修改格式和控件的典型 ioctl 序列为

 #. VIDIOC_STREAMOFF
 #. VIDIOC_S_EXT_CTRLS
 #. VIDIOC_S_FMT
 #. VIDIOC_QBUF
 #. VIDIOC_STREAMON

为使该序列正确运行，已入队的缓冲区必须足够大以容纳新格式或控件。如果当前已入队的缓冲区对于新格式而言太小，驱动应当在响应格式更改（`VIDIOC_S_FMT`）或控件更改（`VIDIOC_S_CTRL` 或 `VIDIOC_S_EXT_CTRLS`）时返回 `ENOSPC` 错误。作为简化，驱动如果当前有任何缓冲区已入队，也可以不检查已入队缓冲区的大小而直接从这些 ioctl 返回 `EBUSY` 错误。

此外，如果正在入队的缓冲区对于当前格式或控件而言太小，驱动应从 `VIDIOC_QBUF` ioctl 返回 `EINVAL` 错误。这些要求共同确保已入队的缓冲区始终足够大以容纳所配置的格式和控件。

用户空间应用程序可以通过将所需控件值先设置好，然后尝试所需格式，来查询给定格式和控件所需的缓冲区大小。`VIDIOC_TRY_FMT` ioctl 将返回所需的缓冲区大小。

 #. VIDIOC_S_EXT_CTRLS(x)
 #. VIDIOC_TRY_FMT()
 #. VIDIOC_S_EXT_CTRLS(y)
 #. VIDIOC_TRY_FMT()

随后可以使用 `VIDIOC_CREATE_BUFS` ioctl 基于查询到的尺寸来分配缓冲区（例如分配一组对所有所需格式和控件都足够大的缓冲区，或者针对每个用例分配一组尺寸合适的单独缓冲区）。


## struct v4l2_buffer



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 2 10

    - - __u32
      - `index`
      - 缓冲区的编号，由应用程序设置，但在调用
	VIDIOC_DQBUF <VIDIOC_QBUF> 时由驱动设置。该字段的取值范围从零到通过
	VIDIOC_REQBUFS ioctl 分配的缓冲区数量（结构体 `v4l2_requestbuffers`
	的 `count` 字段），再加上通过
	VIDIOC_CREATE_BUFS 分配的缓冲区数量减一。
    - - __u32
      - `type`
      - 缓冲区的类型，与结构体
	`v4l2_format` 的 `type` 字段或结构体
	`v4l2_requestbuffers` 的 `type` 字段相同，由应用程序设置。参见 `v4l2_buf_type`
    - - __u32
      - `bytesused`
      - 缓冲区中数据所占用的字节数。它取决于协商得到的数据格式，对于 JPEG
	这类压缩的可变大小数据，每个缓冲区的值可能不同。当 `type` 指向采集（capture）流时驱动必须设置此字段，当它指向输出（output）流时由应用程序设置。对于多平面格式，此字段被忽略，改用
	`planes` 指针。
    - - __u32
      - `flags`
      - 由应用程序或驱动设置的标志位，参见 buffer-flags。
    - - __u32
      - `field`
      - 指示缓冲区中图像的场顺序，参见
	`v4l2_field`。当缓冲区包含 VBI 数据时此字段不使用。当 `type`
	指向采集流时驱动必须设置它，当它指向输出流时由应用程序设置。
    - - struct timeval
      - `timestamp`
      - 对于采集流，这是捕获第一个数据字节的时间，由
	`clock_gettime()` 函数针对相应时钟 id 返回；参见 buffer-flags 中的
	`V4L2_BUF_FLAG_TIMESTAMP_*`。对于输出流，驱动将最后一个数据字节实际发送出去的时间存入
	`timestamp` 字段。这使应用程序能够监测视频时钟与系统时钟之间的漂移。对于使用
	`V4L2_BUF_FLAG_TIMESTAMP_COPY` 的输出流，应用程序必须填入时间戳，驱动会将其复制到采集流。
    - - struct `v4l2_timecode`
      - `timecode`
      - 当 `flags` 中设置了 `V4L2_BUF_FLAG_TIMECODE` 标志时，该结构体包含一个帧时间码。在
	`V4L2_FIELD_ALTERNATE <v4l2_field>` 模式下，顶场和底场包含相同的时间码。时间码旨在辅助视频编辑，通常记录在录像带上，但也可嵌入到 MPEG 等压缩格式中。此字段独立于
	`timestamp` 和 `sequence` 字段。
    - - __u32
      - `sequence`
      - 由驱动设置，对帧（而非场！）顺序计数。该字段对输入设备和输出设备都会设置。
    - - `2`

	在 `V4L2_FIELD_ALTERNATE <v4l2_field>` 模式下，顶场和底场具有相同的序列号。计数从零开始，并包含丢弃或重复的帧。丢弃的帧是输入设备已接收到但因缺少空闲缓冲区空间而无法存储的帧。重复的帧是输出设备因应用程序未能及时传送新数据而再次显示的帧。

```

	   This may count the frames received e.g. over USB, without
	   taking into account the frames dropped by the remote hardware due
	   to limited compression throughput or bus bandwidth. These devices
	   identify by not enumerating any video standards, see
	   :ref:`standard`.

    * - __u32
      - ``memory``
      - This field must be set by applications and/or drivers in
	accordance with the selected I/O method. See :c:type:`v4l2_memory`
    * - union {
      - ``m``
    * - __u32
      - ``offset``
      - For the single-planar API and when ``memory`` is
	``V4L2_MEMORY_MMAP`` this is the offset of the buffer from the
	start of the device memory. The value is returned by the driver
	and apart of serving as parameter to the
	:c:func:`mmap()` function not useful for applications.
	See :ref:`mmap` for details
    * - unsigned long
      - ``userptr``
      - For the single-planar API and when ``memory`` is
	``V4L2_MEMORY_USERPTR`` this is a pointer to the buffer (casted to
	unsigned long type) in virtual memory, set by the application. See
	:ref:`userp` for details.
    * - struct v4l2_plane
      - ``*planes``
      - When using the multi-planar API, contains a userspace pointer to
	an array of struct :c:type:`v4l2_plane`. The size of
	the array should be put in the ``length`` field of this
	struct :c:type:`v4l2_buffer` structure.
    * - int
      - ``fd``
      - For the single-plane API and when ``memory`` is
	``V4L2_MEMORY_DMABUF`` this is the file descriptor associated with
	a DMABUF buffer.
    * - }
      -
    * - __u32
      - ``length``
      - Size of the buffer (not the payload) in bytes for the
	single-planar API. This is set by the driver based on the calls to
	:ref:`VIDIOC_REQBUFS` and/or
	:ref:`VIDIOC_CREATE_BUFS`. For the
	multi-planar API the application sets this to the number of
	elements in the ``planes`` array. The driver will fill in the
	actual number of valid elements in that array.
    * - __u32
      - ``reserved2``
      - A place holder for future extensions. Drivers and applications
	must set this to 0.
    * - __u32
      - ``request_fd``
      - The file descriptor of the request to queue the buffer to. If the flag
        ``V4L2_BUF_FLAG_REQUEST_FD`` is set, then the buffer will be
	queued to this request. If the flag is not set, then this field will
	be ignored.

	The ``V4L2_BUF_FLAG_REQUEST_FD`` flag and this field are only used by
	:ref:`ioctl VIDIOC_QBUF <VIDIOC_QBUF>` and ignored by other ioctls that
	take a :c:type:`v4l2_buffer` as argument.

	Applications should not set ``V4L2_BUF_FLAG_REQUEST_FD`` for any ioctls
	other than :ref:`VIDIOC_QBUF <VIDIOC_QBUF>`.

	If the device does not support requests, then ``EBADR`` will be returned.
	If requests are supported but an invalid request file descriptor is
	given, then ``EINVAL`` will be returned.


```

## struct v4l2_plane



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `bytesused`
      - 该平面中数据（即其有效载荷）所占用的字节数。当 `type`
	指向采集流时驱动必须设置此字段，当它指向输出流时由应用程序设置。

```

	   Note that the actual image data starts at ``data_offset``
	   which may not be 0.
    * - __u32
      - ``length``
      - Size in bytes of the plane (not its payload). This is set by the
	driver based on the calls to
	:ref:`VIDIOC_REQBUFS` and/or
	:ref:`VIDIOC_CREATE_BUFS`.
    * - union {
      - ``m``
    * - __u32
      - ``mem_offset``
      - When the memory type in the containing struct
	:c:type:`v4l2_buffer` is ``V4L2_MEMORY_MMAP``, this
	is the value that should be passed to :c:func:`mmap()`,
	similar to the ``offset`` field in struct
	:c:type:`v4l2_buffer`.
    * - unsigned long
      - ``userptr``
      - When the memory type in the containing struct
	:c:type:`v4l2_buffer` is ``V4L2_MEMORY_USERPTR``,
	this is a userspace pointer to the memory allocated for this plane
	by an application.
    * - int
      - ``fd``
      - When the memory type in the containing struct
	:c:type:`v4l2_buffer` is ``V4L2_MEMORY_DMABUF``,
	this is a file descriptor associated with a DMABUF buffer, similar
	to the ``fd`` field in struct :c:type:`v4l2_buffer`.
    * - }
      -
    * - __u32
      - ``data_offset``
      - Offset in bytes to video data in the plane. Drivers must set this
	field when ``type`` refers to a capture stream, applications when
	it refers to an output stream.

	.. note::

	   That data_offset is included  in ``bytesused``. So the
	   size of the image in the plane is ``bytesused``-``data_offset``
	   at offset ``data_offset`` from the start of the plane.
    * - __u32
      - ``reserved[11]``
      - Reserved for future use. Should be zeroed by drivers and
	applications.


```

## enum v4l2_buf_type



    :header-rows:  0
    :stub-columns: 0
    :widths:       4 1 9

    - - `V4L2_BUF_TYPE_VIDEO_CAPTURE`
      - 1
      - 单平面视频采集流的缓冲区，参见
	capture。
    - - `V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE`
      - 9
      - 多平面视频采集流的缓冲区，参见
	capture。
    - - `V4L2_BUF_TYPE_VIDEO_OUTPUT`
      - 2
      - 单平面视频输出流的缓冲区，参见
	output。
    - - `V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE`
      - 10
      - 多平面视频输出流的缓冲区，参见 output。
    - - `V4L2_BUF_TYPE_VIDEO_OVERLAY`
      - 3
      - 视频叠加（overlay）的缓冲区，参见 overlay。
    - - `V4L2_BUF_TYPE_VBI_CAPTURE`
      - 4
      - 原始 VBI 采集流的缓冲区，参见 raw-vbi。
    - - `V4L2_BUF_TYPE_VBI_OUTPUT`
      - 5
      - 原始 VBI 输出流的缓冲区，参见 raw-vbi。
    - - `V4L2_BUF_TYPE_SLICED_VBI_CAPTURE`
      - 6
      - 切片 VBI 采集流的缓冲区，参见 sliced。
    - - `V4L2_BUF_TYPE_SLICED_VBI_OUTPUT`
      - 7
      - 切片 VBI 输出流的缓冲区，参见 sliced。
    - - `V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY`
      - 8
      - 视频输出叠加（OSD）的缓冲区，参见 osd。
    - - `V4L2_BUF_TYPE_SDR_CAPTURE`
      - 11
      - 软件定义无线电（SDR）采集流的缓冲区，参见
	sdr。
    - - `V4L2_BUF_TYPE_SDR_OUTPUT`
      - 12
      - 软件定义无线电（SDR）输出流的缓冲区，参见 sdr。
    - - `V4L2_BUF_TYPE_META_CAPTURE`
      - 13
      - 元数据采集的缓冲区，参见 metadata。
    - - `V4L2_BUF_TYPE_META_OUTPUT`
      - 14
      - 元数据输出的缓冲区，参见 metadata。



## 缓冲区标志



    \footnotesize



    :header-rows:  0
    :stub-columns: 0
    :widths:       65 18 70

    - .. _`V4L2-BUF-FLAG-MAPPED`:

      - `V4L2_BUF_FLAG_MAPPED`
      - 0x00000001
      - 缓冲区位于设备内存中，并已映射到应用程序的地址空间，详见 mmap。驱动在调用
	VIDIOC_QUERYBUF、
	VIDIOC_QBUF 或
	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 时设置或清除此标志。由驱动设置。
    - .. _`V4L2-BUF-FLAG-QUEUED`:

      - `V4L2_BUF_FLAG_QUEUED`
      - 0x00000002
      - 驱动内部维护两个缓冲区队列：入队队列和出队队列。当设置此标志时，缓冲区当前位于入队队列中。在缓冲区被填充（采集设备）或显示（输出设备）后，它会自动移动到出队队列。驱动在调用
	`VIDIOC_QUERYBUF` ioctl 时设置或清除此标志。成功调用
	`VIDIOC_QBUF`\ ioctl 后它总是被设置，调用 `VIDIOC_DQBUF` 后总是被清除。
    - .. _`V4L2-BUF-FLAG-DONE`:

      - `V4L2_BUF_FLAG_DONE`
      - 0x00000004
      - 当设置此标志时，缓冲区当前位于出队队列中，已准备好从驱动中出队。驱动在调用
	`VIDIOC_QUERYBUF` ioctl 时设置或清除此标志。调用 `VIDIOC_QBUF` 或
	`VIDIOC_DQBUF` 后它总是被清除。当然，缓冲区不可能同时位于两个队列中，`V4L2_BUF_FLAG_QUEUED` 和
	`V4L2_BUF_FLAG_DONE` 标志是互斥的。不过它们也可以都被清除，此时缓冲区处于“已出队（dequeued）”状态，即在应用程序的管辖范围内。
    - .. _`V4L2-BUF-FLAG-ERROR`:

      - `V4L2_BUF_FLAG_ERROR`
      - 0x00000040
      - 当设置此标志时，缓冲区已成功出队，尽管数据可能已损坏。这是可恢复的，流传输可以照常继续，缓冲区也可以照常重用。驱动在调用
	`VIDIOC_DQBUF` ioctl 时设置此标志。
    - .. _`V4L2-BUF-FLAG-IN-REQUEST`:

      - `V4L2_BUF_FLAG_IN_REQUEST`
      - 0x00000080
      - 该缓冲区是一个尚未入队的请求的一部分。
    - .. _`V4L2-BUF-FLAG-KEYFRAME`:

      - `V4L2_BUF_FLAG_KEYFRAME`
      - 0x00000008
      - 驱动在调用 `VIDIOC_DQBUF` ioctl 时设置或清除此标志。当缓冲区包含可作为关键帧（或场）独立解压缩的压缩图像时，视频采集设备可能会设置它，也称为 I 帧（I-frame）。当
	`type` 指向输出流时，应用程序可以设置此位。
    - .. _`V4L2-BUF-FLAG-PFRAME`:

      - `V4L2_BUF_FLAG_PFRAME`
      - 0x00000010
      - 与 `V4L2_BUF_FLAG_KEYFRAME` 类似，此标志标记仅包含与前一关键帧差异的预测帧或场。当
	`type` 指向输出流时，应用程序可以设置此位。
    - .. _`V4L2-BUF-FLAG-BFRAME`:

      - `V4L2_BUF_FLAG_BFRAME`
      - 0x00000020
      - 与 `V4L2_BUF_FLAG_KEYFRAME` 类似，此标志标记双向预测帧或场，其内容仅由当前帧与前一关键帧和后一关键帧之间的差异来指定。当
	`type` 指向输出流时，应用程序可以设置此位。
    - .. _`V4L2-BUF-FLAG-TIMECODE`:

      - `V4L2_BUF_FLAG_TIMECODE`
      - 0x00000100
      - `timecode` 字段有效。驱动在调用 `VIDIOC_DQBUF`
	ioctl 时设置或清除此标志。当 `type` 指向输出流时，应用程序可以设置此位以及相应的
	`timecode` 结构体。
    - .. _`V4L2-BUF-FLAG-PREPARED`:

      - `V4L2_BUF_FLAG_PREPARED`
      - 0x00000400
      - 缓冲区已为 I/O 做好准备，可由应用程序入队。驱动在调用
	VIDIOC_QUERYBUF <VIDIOC_QUERYBUF>、
	VIDIOC_PREPARE_BUF <VIDIOC_QBUF>、
	VIDIOC_QBUF <VIDIOC_QBUF> 或
	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 时设置或清除此标志。
    - .. _`V4L2-BUF-FLAG-NO-CACHE-INVALIDATE`:

      - `V4L2_BUF_FLAG_NO_CACHE_INVALIDATE`
      - 0x00000800
      - 不必使该缓冲区的缓存失效。通常，如果缓冲区中捕获的数据不会被 CPU 触碰，而是很可能被传递给支持 DMA 的硬件单元做进一步处理或输出，应用程序应使用此标志。除非队列用于内存映射（memory mapping <mmap>）流式 I/O 并且报告 :ref:`V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS
	<V4L2-BUF-CAP-SUPPORTS-MMAP-CACHE-HINTS>` 能力，否则此标志被忽略。
    - .. _`V4L2-BUF-FLAG-NO-CACHE-CLEAN`:

      - `V4L2_BUF_FLAG_NO_CACHE_CLEAN`
      - 0x00001000
      - 不必清理该缓冲区的缓存。通常，如果该缓冲区中的数据不是由 CPU 而是由某个支持 DMA 的单元创建的（这种情况下并未使用缓存），应用程序应对输出缓冲区使用此标志。除非队列用于内存映射（memory mapping <mmap>）流式 I/O 并且报告 :ref:`V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS
	<V4L2-BUF-CAP-SUPPORTS-MMAP-CACHE-HINTS>` 能力，否则此标志被忽略。
    - .. _`V4L2-BUF-FLAG-M2M-HOLD-CAPTURE-BUF`:

      - `V4L2_BUF_FLAG_M2M_HOLD_CAPTURE_BUF`
      - 0x00000200
      - 仅当结构体 `v4l2_requestbuffers` 的 `V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF` 标志被设置时才有效。它通常与无状态解码器一起使用，其中多个输出缓冲区各自解码为解码后帧的一个切片。应用程序在入队输出缓冲区时可以设置此标志，以防止驱动在输出缓冲区解码完成后将采集缓冲区出队（即“保持”采集缓冲区）。如果该输出缓冲区的时间戳与前一个输出缓冲区的时间戳不同，则表明一个新帧开始，之前保持的采集缓冲区被出队。
    - .. _`V4L2-BUF-FLAG-LAST`:

      - `V4L2_BUF_FLAG_LAST`
      - 0x00100000
      - 硬件产生的最后一个缓冲区。当调用 VIDIOC_QUERYBUF 或
	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 时，mem2mem 编解码器驱动会在采集队列的最后一个缓冲区上设置此标志。受硬件限制，最后一个缓冲区可能为空。此时驱动会将
	`bytesused` 字段设为 0，与格式无关。之后任何对
	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 的调用都不再阻塞，而是返回 `EPIPE` 错误码。
    - .. _`V4L2-BUF-FLAG-REQUEST-FD`:

      - `V4L2_BUF_FLAG_REQUEST_FD`
      - 0x00800000
      - `request_fd` 字段包含一个有效的文件描述符。
    - .. _`V4L2-BUF-FLAG-TIMESTAMP-MASK`:

      - `V4L2_BUF_FLAG_TIMESTAMP_MASK`
      - 0x0000e000
      - 下方时间戳类型的掩码。要测试时间戳类型，可通过对缓冲区标志位和时间戳掩码执行逻辑与操作，将不属于时间戳类型的位屏蔽掉。
    - .. _`V4L2-BUF-FLAG-TIMESTAMP-UNKNOWN`:

      - `V4L2_BUF_FLAG_TIMESTAMP_UNKNOWN`
      - 0x00000000
      - 未知的时间戳类型。Linux 3.9 之前的驱动使用此类型，它可能是单调时钟（见下文）或实时时钟（墙上时钟）。嵌入式系统中倾向于使用单调时钟，而大多数驱动使用实时时钟。这两种时间戳都可通过
	`clock_gettime` 分别使用时钟 ID `CLOCK_MONOTONIC`
	和 `CLOCK_REALTIME` 在用户空间获得。
    - .. _`V4L2-BUF-FLAG-TIMESTAMP-MONOTONIC`:

      - `V4L2_BUF_FLAG_TIMESTAMP_MONOTONIC`
      - 0x00002000
      - 缓冲区时间戳取自 `CLOCK_MONOTONIC` 时钟。要在 V4L2 之外访问同一时钟，请使用
	`clock_gettime`。
    - .. _`V4L2-BUF-FLAG-TIMESTAMP-COPY`:

      - `V4L2_BUF_FLAG_TIMESTAMP_COPY`
      - 0x00004000
      - CAPTURE 缓冲区的时间戳取自对应的 OUTPUT 缓冲区。此标志仅适用于 mem2mem 设备。
    - .. _`V4L2-BUF-FLAG-TSTAMP-SRC-MASK`:

      - `V4L2_BUF_FLAG_TSTAMP_SRC_MASK`
      - 0x00070000
      - 下方时间戳源的掩码。时间戳源定义相对于帧而言采集时间戳的时间点。对 `flags` 字段和
	`V4L2_BUF_FLAG_TSTAMP_SRC_MASK` 执行逻辑与操作可得到时间戳源的值。当
	`type` 指向输出流且设置了 `V4L2_BUF_FLAG_TIMESTAMP_COPY` 时，应用程序必须设置时间戳源。
    - .. _`V4L2-BUF-FLAG-TSTAMP-SRC-EOF`:

      - `V4L2_BUF_FLAG_TSTAMP_SRC_EOF`
      - 0x00000000
      - 帧结束（End Of Frame）。时间戳在帧的最后一个像素被接收或帧的最后一个像素被发送时采集。实际上，软件生成的时间戳通常会在最后一个像素被接收或发送后的短暂停顿后从时钟读取，具体取决于系统及其中的其他活动。
    - .. _`V4L2-BUF-FLAG-TSTAMP-SRC-SOE`:

      - `V4L2_BUF_FLAG_TSTAMP_SRC_SOE`
      - 0x00010000
      - 曝光开始（Start Of Exposure）。时间戳在帧的曝光开始时采集。这仅对
	`V4L2_BUF_TYPE_VIDEO_CAPTURE` 缓冲区类型有效。



    \normalsize

## enum v4l2_memory



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_MEMORY_MMAP`
      - 1
      - 缓冲区用于内存映射（memory mapping <mmap>）I/O。
    - - `V4L2_MEMORY_USERPTR`
      - 2
      - 缓冲区用于用户指针（user pointer <userp>）I/O。
    - - `V4L2_MEMORY_OVERLAY`
      - 3
      - [to do]
    - - `V4L2_MEMORY_DMABUF`
      - 4
      - 缓冲区用于 DMA 共享缓冲区（DMA shared buffer <dmabuf>）I/O。


    \normalsize

## 时间码


`v4l2_buffer_timecode` 结构体设计用于保存 smpte12m 或类似的时间码。
（结构体 `timeval` 时间戳存储在结构体
`v4l2_buffer` 的 `timestamp` 字段中。）


### struct v4l2_timecode



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 时间码所基于的帧率，参见 timecode-type。
    - - __u32
      - `flags`
      - 时间码标志，参见 timecode-flags。
    - - __u8
      - `frames`
      - 帧计数，0 ... 23/24/29/49/59，取决于时间码的类型。
    - - __u8
      - `seconds`
      - 秒计数，0 ... 59。这是二进制数，不是 BCD 码。
    - - __u8
      - `minutes`
      - 分计数，0 ... 59。这是二进制数，不是 BCD 码。
    - - __u8
      - `hours`
      - 小时计数，0 ... 29。这是二进制数，不是 BCD 码。
    - - __u8
      - `userbits`\ [^4^]
      - 时间码中的“用户组（user group）”位。



### 时间码类型



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TC_TYPE_24FPS`
      - 1
      - 每秒 24 帧，即 film（电影）。
    - - `V4L2_TC_TYPE_25FPS`
      - 2
      - 每秒 25 帧，即 PAL 或 SECAM 视频。
    - - `V4L2_TC_TYPE_30FPS`
      - 3
      - 每秒 30 帧，即 NTSC 视频。
    - - `V4L2_TC_TYPE_50FPS`
      - 4
      -
    - - `V4L2_TC_TYPE_60FPS`
      - 5
      -



### 时间码标志



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_TC_FLAG_DROPFRAME`
      - 0x0001
      - 指示用于 29.97 fps 素材计帧的“丢帧（drop frame）”语义。设置后，除第 0、10、20、30、40、50 分钟外，每分钟开始时帧号 0 和 1 被从计数中省略。
    - - `V4L2_TC_FLAG_COLORFRAME`
      - 0x0002
      - “彩色帧（color frame）”标志。
    - - `V4L2_TC_USERBITS_field`
      - 0x000C
      - “二进制组标志（binary group flags）”的字段掩码。
    - - `V4L2_TC_USERBITS_USERDEFINED`
      - 0x0000
      - 未指定格式。
    - - `V4L2_TC_USERBITS_8BITCHARS`
      - 0x0008
      - 8 位 ISO 字符。
