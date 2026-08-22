


######## ioctl VIDIOC_REQBUFS


## Name


VIDIOC_REQBUFS - 发起内存映射、用户指I/O DMA 缓冲I/O

## Synopsis



`int ioctl(int fd, VIDIOC_REQBUFS, struct v4l2_requestbuffers *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_requestbuffers` 的指针
## Description


ioctl 用于发起基于内存映射 <mmap>、用户指<userp> DMABUF <dmabuf> I/O
内存映射缓冲区位于设备内存中，必须先通过ioctl 分配，然后才能映射到应用程序的地址空间。用户缓冲区由应用程序自身分配，ioctl 仅用于将驱动切换到用户指I/O 模式并设置一些内部结构。类似地，DMABUF 缓冲区由应用程序通过设备驱动分配，此 ioctl 仅将驱动配置DMABUF I/O 模式，而不执行任何直接的分配
要分配设备缓冲区，应用程序初始化 struct `v4l2_requestbuffers` 结构的所有字段。它们将 `type` 字段设为相应的流或缓冲区类型，将 `count` 字段设为所需的缓冲区数量，`memory` 必须设为请求I/O 方法，并`reserved` 数组必须清零。当以指向该结构的指针调用此 ioctl 时，驱动会尝试分配所请求数量的缓冲区，并将实际分配的缓冲区数量存`count` 字段。当驱动耗尽空闲内存时，该值可能小于请求的数量，甚至为 0。当驱动需要更多缓冲区才能正常工作时，也可能返回更大的数量。例如视频输出至少需要两个缓冲区，一个用于显示，一个由应用程序填充
I/O 方法不受支持时，ioctl 返回 `EINVAL` 错误码
应用程序可以再次调用 VIDIOC_REQBUFS 来改变缓冲区数量。注意，如果仍有任何缓冲区被映射或通过 DMABUF 导出，那么只有在设置`V4L2_BUF_CAP_SUPPORTS_ORPHANED_BUFS` 能力VIDIOC_REQBUFS 才能成功。否VIDIOC_REQBUFS 将返`EBUSY` 错误码。如果设置了 `V4L2_BUF_CAP_SUPPORTS_ORPHANED_BUFS`，则这些缓冲区会被“孤儿化（orphaned）”，并在它们被取消映射或导出DMABUF fds 被关闭时被释放。`count` 值为 0 会在中止或完成任何进行中DMA 之后释放或孤儿化所有缓冲区，这是一个隐式的 VIDIOC_STREAMOFF <VIDIOC_STREAMON>


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `count`
      - 请求或授予的缓冲区数量    - - __u32
      - `type`
      - 流或缓冲区的类型，与 struct `v4l2_format` `type` 字段相同。有效值参`v4l2_buf_type`    - - __u32
      - `memory`
      - 应用程序将此字段设为 `V4L2_MEMORY_MMAP`、`V4L2_MEMORY_DMABUF` `V4L2_MEMORY_USERPTR`。参`v4l2_memory`    - - __u32
      - `capabilities`
      - 由驱动设置。如果为 0，说明驱动不支持能力查询。在这种情况下，你所知道的只是驱动保证支`V4L2_MEMORY_MMAP`，并*可能**支持其他 `v4l2_memory` 类型。它不会支持任何其他能力
	如果你想以最小的副作用查询能力，可以使用 `count` 设为 0、`memory` 设为 `V4L2_MEMORY_MMAP`、`type` 设为缓冲区类型来调用。这会释放任何之前分配的缓冲区，因此通常是在应用程序启动时进行的操作    - - __u8
      - `flags`
      - 指定额外的缓冲区管理属性。参memory-flags    - - __u8
      - `reserved`\ [^3^]
      - 保留供将来扩展使用

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_BUF_CAP_SUPPORTS_MMAP`
      - 0x00000001
      - 此缓冲区类型支持 `V4L2_MEMORY_MMAP` 流模式    - - `V4L2_BUF_CAP_SUPPORTS_USERPTR`
      - 0x00000002
      - 此缓冲区类型支持 `V4L2_MEMORY_USERPTR` 流模式    - - `V4L2_BUF_CAP_SUPPORTS_DMABUF`
      - 0x00000004
      - 此缓冲区类型支持 `V4L2_MEMORY_DMABUF` 流模式    - - `V4L2_BUF_CAP_SUPPORTS_REQUESTS`
      - 0x00000008
      - 此缓冲区类型支持请求 <media-request-api>    - - `V4L2_BUF_CAP_SUPPORTS_ORPHANED_BUFS`
      - 0x00000010
      - 内核允许在缓冲区仍被映射或通过 DMABUF 导出时调VIDIOC_REQBUFS。这些“孤儿化”的缓冲区会在它们被取消映射或导出的 DMABUF fds 被关闭时被释放    - - `V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF`
      - 0x00000020
      - 仅对无状态解码器有效。如果设置，则用户空间可以设`V4L2_BUF_FLAG_M2M_HOLD_CAPTURE_BUF` 标志，以延迟返回捕获缓冲区，直到 OUTPUT 时间戳发生变化    - - `V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS`
      - 0x00000040
      - 此能力由驱动设置，表示队列支持缓存和内存管理提示。然而，它仅在队列用于内存映<mmap> I/O 时才有效。参V4L2_BUF_FLAG_NO_CACHE_INVALIDATE <V4L2-BUF-FLAG-NO-CACHE-INVALIDATE>、V4L2_BUF_FLAG_NO_CACHE_CLEAN <V4L2-BUF-FLAG-NO-CACHE-CLEAN> V4L2_MEMORY_FLAG_NON_COHERENT <V4L2-MEMORY-FLAG-NON-COHERENT>    - - `V4L2_BUF_CAP_SUPPORTS_MAX_NUM_BUFFERS`
      - 0x00000080
      - 如果设置，则 `struct v4l2_create_buffers` 中的 `max_num_buffers` 字段有效。如果未设置，则最大值为 `VIDEO_MAX_FRAME` 个缓冲区    - - `V4L2_BUF_CAP_SUPPORTS_REMOVE_BUFS`
      - 0x00000100
      - 如果设置，则支持 `VIDIOC_REMOVE_BUFS`

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_MEMORY_FLAG_NON_COHERENT`
      - 0x00000001
      - 缓冲区被分配在一致（coherent，它将在 CPU 和总线之间自动保持一致）或非一致（non-coherent）内存中。后者可以提供性能提升，例如，如果缓冲区仅由相应设备访问且 CPU 不对该缓冲区进行读写，则可以避免 CPU 缓存同步/刷新操作。然而，这需要驱动格外小心——它必须在需要一致性时通过发出缓存刷新/同步来保证内存一致性。如果设置了此标志，V4L2 将尝试在非一致内存中分配缓冲区。该标志仅在缓冲区用于内存映<mmap> I/O 且队列报告了 :ref:`V4L2_BUF_CAP_SUPPORTS_MMAP_CACHE_HINTS <V4L2-BUF-CAP-SUPPORTS-MMAP-CACHE-HINTS>` 能力时才生效

   \normalsize

## Return Value


成功时返0，出错时返回 -1 并适当地设`errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EINVAL
    缓冲区类型（`type` 字段）或请求I/O 方法（`memory`）不受支持