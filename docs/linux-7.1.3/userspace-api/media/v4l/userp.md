


######## Streaming I/O (User Pointers)


当 VIDIOC_QUERYCAP ioctl 返回的 struct `v4l2_capability` 的 `capabilities` 字段中的 `V4L2_CAP_STREAMING` 标志被设置时，输入和输出设备支持这种 I/O 方法。而特定用户指针方法（不仅仅是内存映射）是否受支持，必须通过调用 VIDIOC_REQBUFS ioctl 并将内存类型设置为 `V4L2_MEMORY_USERPTR` 来确定。

这种 I/O 方法结合了 read/write 和内存映射方法的优点。缓冲区（plane）由应用程序自身分配，并且可以驻留在例如虚拟内存或共享内存中。只交换指向数据的指针，这些指针和元信息在 struct `v4l2_buffer`（或多 plane API 情况下的 struct `v4l2_plane`）中传递。必须通过调用 VIDIOC_REQBUFS 并传入所需的缓冲区类型，将驱动切换到用户指针 I/O 模式。
事先不分配任何缓冲区（plane），因此它们不被索引，也不能像映射缓冲区那样通过 VIDIOC_QUERYBUF <VIDIOC_QUERYBUF> ioctl 查询。

## Example: Initiating streaming I/O with user pointers



    struct v4l2_requestbuffers reqbuf;

    memset (&reqbuf, 0, sizeof (reqbuf));
    reqbuf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    reqbuf.memory = V4L2_MEMORY_USERPTR;

    if (ioctl (fd, VIDIOC_REQBUFS, &reqbuf) == -1) {
	if (errno == EINVAL)
	    printf ("Video capturing or user pointer streaming is not supported\\n");
	else
	    perror ("VIDIOC_REQBUFS");

	exit (EXIT_FAILURE);
    }

缓冲区（plane）的地址和大小在运行时通过 VIDIOC_QBUF <VIDIOC_QBUF> ioctl 传递。尽管缓冲区通常被循环使用，但应用程序可以在每次 VIDIOC_QBUF <VIDIOC_QBUF> 调用时传入不同的地址和大小。如果硬件有需要，驱动会在物理内存中交换内存页，以创建一块连续的内存区域。这对应用程序是透明的，发生在内核的虚拟内存子系统中。当缓冲区页被换出到磁盘后，它们会被取回，并最终被锁定在物理内存中以供 DMA 使用。[#f1]_

填充或显示完毕的缓冲区通过 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 出队。驱动可以在 DMA 完成与此 ioctl 之间的任何时候解锁内存页。当调用 VIDIOC_STREAMOFF <VIDIOC_STREAMON>、VIDIOC_REQBUFS，或设备被关闭时，内存也会被解锁。应用程序必须注意，不要在缓冲区出队之前就将其释放。首先，缓冲区会被锁定更长时间，浪费物理内存。其次，当内存返回到应用程序的空闲列表并被随后用于其他用途时，驱动不会收到通知，可能会完成所请求的 DMA 并覆盖有价值的数据。

对于采集（capturing）应用，通常的做法是入队若干空缓冲区，开始采集并进入读取循环。在这里，应用程序等待直到有已填充的缓冲区可以出队，并在数据不再需要时重新入队该缓冲区。输出（output）应用则填充并入队缓冲区，当堆叠了足够多的缓冲区后开始输出。在写入循环中，当应用程序用尽空闲缓冲区时，它必须等待直到有空缓冲区可以出队并被重用。存在两种方法来挂起应用程序的执行，直到一个或多个缓冲区可以出队。默认情况下，当外发队列中没有缓冲区时 :ref:`VIDIOC_DQBUF <VIDIOC_QBUF>` 会阻塞。当 `open()` 函数被传入了 `O_NONBLOCK` 标志时，当没有可用缓冲区时，VIDIOC_DQBUF <VIDIOC_QBUF> 会立即返回 `EAGAIN` 错误码。:ref:`select() <func-select>` 或 `poll()` 函数始终可用。

要启动和停止采集或输出应用，调用 VIDIOC_STREAMON <VIDIOC_STREAMON> 和 VIDIOC_STREAMOFF <VIDIOC_STREAMON> ioctl。

   VIDIOC_STREAMOFF <VIDIOC_STREAMON> 会作为副作用从两个队列中移除所有缓冲区并解锁所有缓冲区。由于在多任务系统上不存在"现在"执行某事的语义，如果应用程序需要与其他事件同步，它应当检查所采集或输出缓冲区的 struct `v4l2_buffer` `timestamp`。

实现用户指针 I/O 的驱动必须支持 VIDIOC_REQBUFS <VIDIOC_REQBUFS>、VIDIOC_QBUF <VIDIOC_QBUF>、VIDIOC_DQBUF <VIDIOC_QBUF>、VIDIOC_STREAMON <VIDIOC_STREAMON> 和 VIDIOC_STREAMOFF <VIDIOC_STREAMON> ioctl，以及 `select()` 和 `poll()` 函数。[#f2]_

   我们期望频繁使用的缓冲区通常不会被换出。无论如何，交换、锁定或生成分散-聚集（scatter-gather）列表的过程可能很耗时。这种延迟可以通过输入缓冲区队列的深度来掩盖，或许还可以通过维护缓存（假设某个缓冲区很快会再次入队）来掩盖。另一方面，为了优化内存使用，驱动可以限制预先锁定的缓冲区数量，并优先回收最近使用的缓冲区。当然，输入队列中空闲缓冲区的页不需要保存到磁盘。输出缓冲区必须在输入和输出队列中都被保存，因为应用程序可能与其他进程共享它们。

   在驱动层面，`select()` 和 `poll()` 是相同的，而且 `select()` 太重要了，不能是可选项。其余的应当显而易见。
