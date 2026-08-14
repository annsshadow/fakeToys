


######## V4L2 mmap()

## 名称

v4l2-mmap - 将设备内存映射到应用程序地址空间

## 概要


    #include <unistd.h>
    #include <sys/mman.h>


## 参数

`start`
    将缓冲区映射到应用程序地址空间中的这个地址。当指定了 `MAP_FIXED` 标志时，
    `start` 必须是页大小的整数倍，并且当指定的地址无法使用时 mmap 会失败。不
    鼓励使用此选项；应用程序在这里应直接指定一个 `NULL` 指针。

`length`
    要映射的内存区域长度。对于单平面（single-planar）API，此值必须与驱动在
    struct `v4l2_buffer` 的 `length` 字段中返回的值相同；对于多平面
    （multi-planar）API，此值必须与驱动在 struct `v4l2_plane` 的 `length`
    字段中返回的值相同。

`prot`
    `prot` 参数描述了期望的内存保护。无论设备类型和数据交换方向如何，它都应
    设为 `PROT_READ` | `PROT_WRITE`，以允许对图像缓冲区进行读写访问。驱动应
    至少支持这种标志组合。

```

      #. Linux 的 ``videobuf`` 内核模块被一些驱动使用，它只支持
	 ``PROT_READ`` | ``PROT_WRITE``。当驱动不支持期望的保护时，
	 :c:func:`mmap()` 函数会失败。

      #. 设备内存访问（例如带有视频采集硬件的显卡上的内存）与访问主内存相比
	 可能会产生性能损失，或者读操作可能明显慢于写操作，反之亦然。在这种
	 情况下，其它 I/O 方法可能更高效。

```
`flags`
    `flags` 参数指定了映射对象的类型、映射选项，以及对映射页面副本的修改是
    进程私有的还是与其它引用共享。

    `MAP_FIXED` 要求驱动不选择除指定地址以外的其它地址。如果指定的地址无法使用，
    `mmap()` 会失败。如果指定了 `MAP_FIXED`，`start` 必须是页大小的整数倍。不
    鼓励使用此选项。

    必须设置 `MAP_SHARED` 或 `MAP_PRIVATE` 之一。`MAP_SHARED` 允许应用程序将映射的
    内存与其它（例如子）进程共享。

```

       Linux 的 ``videobuf`` 模块被一些驱动使用，只支持 ``MAP_SHARED``。
       ``MAP_PRIVATE`` 请求写时复制（copy-on-write）语义。V4L2 应用程序不应设置
       ``MAP_PRIVATE``、``MAP_DENYWRITE``、``MAP_EXECUTABLE`` 或 ``MAP_ANON``
       标志。

```
`fd`
    由 `open()` 返回的文件描述符。

`offset`
    缓冲区在设备内存中的偏移量。对于单平面 API，此值必须与驱动在 struct
    `v4l2_buffer` 的 `m` 联合体 `offset` 字段中返回的值相同；对于多平面 API，
    此值必须与驱动在 struct `v4l2_plane` 的 `m` 联合体 `mem_offset` 字段中返回的
    值相同。

## 描述

`mmap()` 函数请求将 `fd` 指定设备的内存中从 `offset` 开始的 `length` 字节映射到
应用程序地址空间，最好映射到地址 `start`。后一个地址只是一个提示，通常指定为 0。

合适的 length 和 offset 参数通过 VIDIOC_QUERYBUF ioctl 查询。缓冲区必须先通过
VIDIOC_REQBUFS ioctl 分配，然后才能被查询。

要解除映射缓冲区，使用 `munmap()` 函数。

## 返回值

成功时 `mmap()` 返回指向已映射缓冲区的指针。出错时返回 `MAP_FAILED`（-1），并
相应地设置 `errno` 变量。可能的错误码如下：

EBADF
    `fd` 不是有效的文件描述符。

EACCES
    `fd` 未以读写方式打开。

EINVAL
    `start`、`length` 或 `offset` 不合适（例如它们太大，或未按 `PAGESIZE` 边界
    对齐）。

    `flags` 或 `prot` 的值不被支持。

    没有通过 VIDIOC_REQBUFS ioctl 分配任何缓冲区。

ENOMEM
    没有足够的物理或虚拟内存来完成该请求。
