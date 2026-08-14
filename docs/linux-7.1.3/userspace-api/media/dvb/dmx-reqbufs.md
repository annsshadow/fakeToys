


######## ioctl DMX_REQBUFS


## 名称


DMX_REQBUFS - 发起内存映射和/或 DMA 缓冲区 I/O


## 概要



`int ioctl(int fd, DMX_REQBUFS, struct dmx_requestbuffers *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向 struct `dmx_requestbuffers` 的指针。

## 描述


该 ioctl 用于发起基于内存映射或 DMABUF 的解复用（demux）I/O。

内存映射缓冲区位于设备内存中，必须先通过该 ioctl 分配，然后才能映射到应用程序的地址空间。用户缓冲区由应用程序自身分配，该 ioctl 仅用于将驱动切换到用户指针 I/O 模式并设置一些内部结构。类似地，DMABUF 缓冲区由应用程序通过设备驱动分配，该 ioctl 只将驱动配置为 DMABUF I/O 模式，而不执行任何直接分配。

要分配设备缓冲区，应用程序初始化 struct `dmx_requestbuffers` 结构的所有字段。它们将 `count` 字段设置为期望的缓冲区数量，并将 `size` 设置为每个缓冲区的大小。

当使用一个指向该结构的指针调用该 ioctl 时，驱动会尝试分配所请求数量的缓冲区，并将实际分配的缓冲区数量存入 `count` 字段。当驱动耗尽空闲内存时，`count` 可以小于请求的数量，甚至为零。当驱动需要更多缓冲区才能正常工作时，也可能返回更大的数量。实际分配的缓冲区大小在 `size` 中返回，且可能小于所请求的大小。

当不支持该 I/O 方法时，ioctl 返回 `EOPNOTSUPP` 错误码。

应用程序可以再次调用 DMX_REQBUFS 来改变缓冲区的数量，但当仍有缓冲区被映射时无法成功。将 `count` 设为零会释放所有缓冲区，前提是中止或完成任何正在进行的 DMA。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述。

EOPNOTSUPP
    请求的 I/O 方法不受支持。
