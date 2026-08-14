

######## ioctl VIDIOC_QBUF, VIDIOC_DQBUF


## 名称


VIDIOC_QBUF - VIDIOC_DQBUF - 与驱动交换一个缓冲区

## 概要


`int ioctl(int fd, VIDIOC_QBUF, struct v4l2_buffer *argp)`


`int ioctl(int fd, VIDIOC_DQBUF, struct v4l2_buffer *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `v4l2_buffer` 的指针。

## 说明


应用程序调用 `VIDIOC_QBUF` ioctl 将空的（capture 捕获）或已填充的（output 输出）缓冲区放入驱动的入队队列。其语义取决于所选择的 I/O 方法。

要入队一个缓冲区，应用程序将 struct `v4l2_buffer` 的 `type` 字段设为之前与 struct `v4l2_format` 的 `type` 以及 struct `v4l2_requestbuffers` 的 `type` 所用过的相同缓冲区类型。应用程序还必须设置 `index` 字段。有效的索引号范围从 0 到用 VIDIOC_REQBUFS（struct `v4l2_requestbuffers` 的 `count`）分配的缓冲区数量减一。VIDIOC_QUERYBUF ioctl 返回的 struct `v4l2_buffer` 的内容同样可用。当缓冲区用于输出（`type` 为 `V4L2_BUF_TYPE_VIDEO_OUTPUT`、`V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE` 或 `V4L2_BUF_TYPE_VBI_OUTPUT`）时，应用程序还必须初始化 `bytesused`、`field` 和 `timestamp` 字段，详见 buffer。应用程序还必须将 `flags` 设为 0。`reserved2` 和 `reserved` 字段必须设为 0。当使用多平面 API <planar-apis> 时，`m.planes` 字段必须包含一个指向已填充的 struct `v4l2_plane` 数组的用户空间指针，且 `length` 字段必须设为该数组的元素个数。

要入队一个内存映射 <mmap> 缓冲区，应用程序将 `memory` 字段设为 `V4L2_MEMORY_MMAP`。当用指向该结构的指针调用 `VIDIOC_QBUF` 时，驱动会设置 `V4L2_BUF_FLAG_MAPPED` 和 `V4L2_BUF_FLAG_QUEUED` 标志，并清除 `flags` 字段中的 `V4L2_BUF_FLAG_DONE` 标志，否则返回 `EINVAL` 错误码。

要入队一个用户指针 <userp> 缓冲区，应用程序将 `memory` 字段设为 `V4L2_MEMORY_USERPTR`，将 `m.userptr` 字段设为缓冲区的地址，并将 `length` 设为其大小。当使用多平面 API 时，必须改用所传入的 struct `v4l2_plane` 数组的 `m.userptr` 和 `length` 成员。当用指向该结构的指针调用 `VIDIOC_QBUF` 时，驱动会设置 `V4L2_BUF_FLAG_QUEUED` 标志并清除 `flags` 字段中的 `V4L2_BUF_FLAG_MAPPED` 和 `V4L2_BUF_FLAG_DONE` 标志，否则返回错误码。该 ioctl 会将缓冲区的物理内存页锁定，它们不能被换出到磁盘。缓冲区会一直保持锁定，直到被出队、调用 VIDIOC_STREAMOFF <VIDIOC_STREAMON> 或 VIDIOC_REQBUFS ioctl，或者设备被关闭。

要入队一个 DMABUF <dmabuf> 缓冲区，应用程序将 `memory` 字段设为 `V4L2_MEMORY_DMABUF`，并将 `m.fd` 字段设为一个与 DMABUF 缓冲区相关联的文件描述符。当使用多平面 API 时，必须改用所传入的 struct `v4l2_plane` 数组的 `m.fd` 字段。当用指向该结构的指针调用 `VIDIOC_QBUF` 时，驱动会设置 `V4L2_BUF_FLAG_QUEUED` 标志并清除 `flags` 字段中的 `V4L2_BUF_FLAG_MAPPED` 和 `V4L2_BUF_FLAG_DONE` 标志，否则返回错误码。该 ioctl 会锁定缓冲区。锁定缓冲区意味着将其交给驱动进行硬件访问（通常是 DMA）。如果应用程序访问（读/写）一个已锁定的缓冲区，结果是未定义的。缓冲区会一直保持锁定，直到被出队、调用 VIDIOC_STREAMOFF <VIDIOC_STREAMON> 或 VIDIOC_REQBUFS ioctl，或者设备被关闭。

`request_fd` 字段可以与 `VIDIOC_QBUF` ioctl 一起使用，以指定一个请求 <media-request-api> 的文件描述符（如果使用了请求）。设置它表示在该请求本身被入队之前，缓冲区不会被传递给驱动。此外，驱动会应用与该请求关联的、针对此缓冲区的任何设置。除非设置了 `V4L2_BUF_FLAG_REQUEST_FD` 标志，否则该字段会被忽略。如果设备不支持请求，则返回 `EBADR`。如果支持请求但给出了无效的请求文件描述符，则返回 `EINVAL`。

   不允许将请求入队与直接入队缓冲区混用。如果第一个缓冲区是直接入队的，然后应用程序又尝试入队一个请求，或者反之，则返回 `EBUSY`。在关闭文件描述符、调用 VIDIOC_STREAMOFF <VIDIOC_STREAMON> 或调用 VIDIOC_REQBUFS 之后，此项检查会被重置。

   对于内存到内存设备 <mem2mem>，你只能为输出缓冲区指定 `request_fd`，不能为捕获缓冲区指定。若尝试为捕获缓冲区指定，会导致 `EBADR` 错误。

应用程序调用 `VIDIOC_DQBUF` ioctl 从驱动的出队队列中取出一个已填充（capture 捕获）或已显示（output 输出）的缓冲区。它们只需按上述方式设置 struct `v4l2_buffer` 的 `type`、`memory` 和 `reserved` 字段，当用指向该结构的指针调用 `VIDIOC_DQBUF` 时，驱动会填充所有剩余字段，否则返回错误码。驱动也可能在 `flags` 字段中设置 `V4L2_BUF_FLAG_ERROR`。它表示非致命（可恢复）的流错误。在这种情况下，应用程序可以照常继续，但应当注意出队缓冲区中的数据可能已被破坏。使用多平面 API 时，也必须传入 planes 数组。

如果应用程序将 `memory` 字段设为 `V4L2_MEMORY_DMABUF` 以出队一个 DMABUF <dmabuf> 缓冲区，驱动会将 `m.fd` 字段填充为一个在数值上与缓冲区入队时提供给 `VIDIOC_QBUF` 的文件描述符相同的文件描述符。出队时不会创建新的文件描述符，该值仅供应用程序方便使用。使用多平面 API 时，改为填充所传入的 struct `v4l2_plane` 数组的 `m.fd` 字段。

默认情况下，当出队队列中没有缓冲区时 `VIDIOC_DQBUF` 会阻塞。当 `open()` 函数被给予 `O_NONBLOCK` 标志时，若没有可用缓冲区，`VIDIOC_DQBUF` 会立即返回 `EAGAIN` 错误码。

struct `v4l2_buffer` 结构的定义见 buffer。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误码 <gen-errors> 一章中描述。

EAGAIN
    已使用 `O_NONBLOCK` 选择了非阻塞 I/O，而出队队列中没有缓冲区。

EINVAL
    不支持缓冲区 `type`，或 `index` 越界，或尚未分配任何缓冲区，或 `userptr` 或 `length` 无效，或设置了 `V4L2_BUF_FLAG_REQUEST_FD` 标志但给定的 `request_fd` 无效，或 `m.fd` 是无效的 DMABUF 文件描述符。

EIO
    `VIDIOC_DQBUF` 因内部错误而失败。也可能表示信号丢失等临时性问题。

```
       The driver might dequeue an (empty) buffer despite returning
       an error, or even stop capturing. Reusing such buffer may be unsafe
       though and its details (e.g. ``index``) may not be returned either.
       It is recommended that drivers indicate recoverable errors by setting
       the ``V4L2_BUF_FLAG_ERROR`` and returning 0 instead. In that case the
       application should be able to safely reuse the buffer and continue
       streaming.

```
EPIPE
    `VIDIOC_DQBUF` 在空捕获队列上针对 mem2mem 编解码器返回此错误，条件是带有 `V4L2_BUF_FLAG_LAST` 的缓冲区已被出队且预计不会有新缓冲区可用。

EBADR
    设置了 `V4L2_BUF_FLAG_REQUEST_FD` 标志但设备不支持该给定缓冲区类型的请求，或者未设置 `V4L2_BUF_FLAG_REQUEST_FD` 标志但设备要求该缓冲区是某个请求的一部分。

EBUSY
    第一个缓冲区是通过请求入队的，但应用程序现在尝试直接入队它，或者反之（不允许混用这两种 API）。
