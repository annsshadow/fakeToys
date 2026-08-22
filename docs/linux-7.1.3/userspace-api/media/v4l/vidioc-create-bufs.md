


######## ioctl VIDIOC_CREATE_BUFS


## 名称


VIDIOC_CREATE_BUFS - 为内存映射、用户指针或 DMA 缓冲I/O 创建缓冲
## 概要



`int ioctl(int fd, VIDIOC_CREATE_BUFS, struct v4l2_create_buffers *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `v4l2_create_buffers` 的指针
## 描述


ioctl 用于为内存映<mmap>、用户指<userp> DMA 缓冲<dmabuf>
I/O 创建缓冲区。当需要对缓冲区进行更严格的控制时，它可以作为 VIDIOC_REQBUFS
ioctl 的替代或补充来使用。该 ioctl 可以多次调用，以创建不同大小的缓冲区
为了分配设备缓冲区，应用程序必须初始struct `v4l2_create_buffers` 结构相关字段。`count` 字段必须设为请求的缓冲区数量，`memory` 字段指定请求I/O
方法，`reserved` 数组必须清零
`format` 字段指定缓冲区必须能够处理的图像格式。应用程序必须填struct
`v4l2_format`。通常这会通过 VIDIOC_TRY_FMT <VIDIOC_G_FMT> VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl 来完成，以确保请求的格式受驱动支持根据格式`type` 字段，分配缓冲区时将使用请求的缓冲区大小（对于单平面）或
平面大小（对于多平面格式）。如果大小不受硬件支持（通常是因为太小），驱可能返回错误
ioctl 创建的缓冲区的最小大小为 `format.pix.sizeimage` 字段（或其它格式类型
的对应字段）所定义的大小。通常，如`format.pix.sizeimage` 字段小于给定格式
所需的最小值，则会返回错误，因为驱动通常不允许这样做。如果它更大，则该值将
原样使用。换句话说，驱动可能拒绝请求的大小，但如果被接受，驱动将不加修改使用它
当以指向该结构的指针调用ioctl 时，驱动将尝试分配多达请求数量的缓冲区，分别把实际分配的数量和起始索引存`count` `index` 字段。返回时 `count` 可能
小于请求的数量


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `index`
      - 起始缓冲区索引，由驱动返回    - - __u32
      - `count`
      - 请求或授予的缓冲区数量。如count == 0，则 VIDIOC_CREATE_BUFS 会将
	`index` 设为当前已创建缓冲区的数量，并检`memory` `format.type`
	的有效性。如果它们无效则返回 -1 并将 errno 设为 `EINVAL` 错误码，否则
	VIDIOC_CREATE_BUFS 返回 0。在这种特定情况下它绝不会将 errno 设为
	`EBUSY` 错误码    - - __u32
      - `memory`
      - 应用程序将该字段设为 `V4L2_MEMORY_MMAP`、`V4L2_MEMORY_DMABUF` 	`V4L2_MEMORY_USERPTR`。参`v4l2_memory`
    - - struct `v4l2_format`
      - `format`
      - 由应用程序填写，由驱动保留    - - __u32
      - `capabilities`
      - 由驱动设置。如果为 0，表示驱动不支持 capabilities。在这种情况下，你所知道	只是驱动保证支持 `V4L2_MEMORY_MMAP`，并*可能**支持其它 `v4l2_memory`
	类型。它不支持任何其capabilities。有capabilities 列表，请参见
	此处 <v4l2-buf-capabilities>
	如果你只想查capabilities 而不做任何其它改动，则将 `count` 设为 0	`memory` 设为 `V4L2_MEMORY_MMAP`，并`format.type` 设为缓冲区类型
    - - __u32
      - `flags`
      - 指定额外的缓冲区管理属性。参memory-flags    - - __u32
      - `max_num_buffers`
      - 如果设置V4L2_BUF_CAP_SUPPORTS_MAX_NUM_BUFFERS capability 标志，则	字段指示此队列可能的最大缓冲区数量    - - __u32
      - `reserved`\ [^5^]
      - 为将来扩展保留的占位符。驱动和应用程序必须将该数组置为零
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述
ENOMEM
    没有内存为内存映<mmap> I/O 分配缓冲区
EINVAL
    缓冲区类型（`format.type` 字段）、请求的 I/O 方法（`memory`）或格式
    （`format` 字段）无效