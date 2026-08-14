
## IIO 的高速 DMABUF 接口


## 1. 概述


工业 I/O（IIO）子系统支持通过基于文件的接口访问缓冲区，即通过 IIO 设备的 dev 节点进行 read() 与 write() 访问调用。

它还额外支持基于 DMABUF 的接口，用户空间可以将 DMABUF 对象（外部创建）附加到 IIO 缓冲区，并随后使用它们进行数据传输。

用户空间应用程序随后可以使用此接口在多个接口之间共享 DMABUF 对象，从而以零拷贝（zero-copy）方式传输数据，例如在 IIO 与 USB 协议栈之间。

用户空间应用程序还可以内存映射（memory-map）DMABUF 对象，并直接访问采样数据。与 read() 接口相比，这样做的优势在于避免了数据在内核与用户空间之间的一次额外拷贝。这对于每秒产生数兆字节甚至数吉字节数据的高速设备尤其有用。不过，它会增加用户空间与内核空间的同步开销，因为必须使用 DMA_BUF_SYNC_START 与 DMA_BUF_SYNC_END IOCTL 来保证数据完整性。

## 2. 用户 API


作为此接口的一部分，新增了三个 IOCTL。这三个 IOCTL 必须在 IIO 缓冲区的文件描述符上执行，该描述符可通过 IIO_BUFFER_GET_FD_IOCTL() ioctl 获取。

  `IIO_BUFFER_DMABUF_ATTACH_IOCTL(int fd)`
    将由其文件描述符标识的 DMABUF 对象附加到 IIO 缓冲区。成功时返回零，出错时返回负的 errno 值。

  `IIO_BUFFER_DMABUF_DETACH_IOCTL(int fd)`
    将给定 DMABUF 对象（由其文件描述符标识）从 IIO 缓冲区分离。成功时返回零，出错时返回负的 errno 值。

    注意，关闭 IIO 缓冲区的文件描述符会自动分离所有先前附加的 DMABUF 对象。

  `IIO_BUFFER_DMABUF_ENQUEUE_IOCTL(struct iio_dmabuf *iio_dmabuf)`
    将先前附加的 DMABUF 对象入队到缓冲区队列。入队的 DMABUF 在缓冲区启用期间会被读取（若为输出缓冲区）或写入（若为输入缓冲区）。
