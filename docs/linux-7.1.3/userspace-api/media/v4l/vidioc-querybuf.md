


######## ioctl VIDIOC_QUERYBUF


## 名称


VIDIOC_QUERYBUF - 查询缓冲区的状
## 概要



`int ioctl(int fd, VIDIOC_QUERYBUF, struct v4l2_buffer *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_buffer` 的指针
## 描述


ioctl 属于流式 <mmap> I/O 方法的一部分。它可以在使VIDIOC_REQBUFS ioctl 分配缓冲区之后的任意时刻，用于查询缓冲区的状态
应用程序struct `v4l2_buffer` `type` 字段设置为先前与 struct `v4l2_format` `type` 以及 struct `v4l2_requestbuffers` `type` 所用过的相同缓冲区类型，并设置 `index` 字段。有效的索引编号范围从零到用 VIDIOC_REQBUFS（struct `v4l2_requestbuffers` `count`）分配的缓冲区数量减一。`reserved` `reserved2` 字段必须设置0。使用多平面 API <planar-apis> 时，`m.planes` 字段必须包含一个指struct `v4l2_plane` 数组的用户空间指针，`length` 字段必须设置为该数组的元素个数。在用一个指向该结构的指针调VIDIOC_QUERYBUF 后，驱动返回错误码或填充结构的其余部分
`flags` 字段中，`V4L2_BUF_FLAG_MAPPED`、`V4L2_BUF_FLAG_PREPARED`、`V4L2_BUF_FLAG_QUEUED` `V4L2_BUF_FLAG_DONE` 标志将是有效的。`memory` 字段将被设置为当前的 I/O 方法。对于单平面 API，`m.offset` 包含缓冲区相对设备内存起始位置的偏移量，`length` 字段为其大小。对于多平面 API，将改用 `m.planes` 数组元素中的 `m.mem_offset` `length` 字段，且 struct `v4l2_buffer` `length` 字段被设置为已填充的数组元素个数。驱动可能会也可能不会设置其余字段与标志，在此上下文中它们没有意义
struct `v4l2_buffer` 结构buffer 中定义
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EINVAL
    缓冲`type` 不受支持，或 `index` 超出范围